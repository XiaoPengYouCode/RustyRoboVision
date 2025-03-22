mod bindings;
mod safe_impl;

use std::{
    alloc::{alloc, dealloc, Layout},
    collections::VecDeque,
    ffi::{c_char, CString},
    fs::File,
    os::raw::{c_uint, c_void},
    str::FromStr,
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use once_cell::sync::Lazy;

use bindings::*;
use safe_impl::{ImvFrameTs, ImvHandleTs};

// 使用 once_cell 替代 lazy_static，功能类似但更现代
static FRAME_QUEUE: Lazy<Mutex<FrameQueue>> = Lazy::new(|| {
    Mutex::new(FrameQueue::new(3)) // 设置最大队列长度为3帧
});

// 帧队列结构体，封装 VecDeque 提高可读性和安全性
struct FrameQueue {
    frames: VecDeque<ImvFrameTs>,
    max_size: usize,
}

impl FrameQueue {
    fn new(max_size: usize) -> Self {
        Self {
            frames: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    // 添加帧到队列
    fn push(&mut self, frame: IMV_Frame) -> bool {
        // 如果队列已满，返回失败
        if self.frames.len() >= self.max_size {
            return false;
        }

        self.frames.push_back(ImvFrameTs::new(frame));
        true
    }

    // 从队列中取出一帧
    fn pop(&mut self) -> Option<ImvFrameTs> {
        self.frames.pop_front()
    }

    // 获取当前队列长度
    fn len(&self) -> usize {
        self.frames.len()
    }

    // 清空队列
    fn clear(&mut self) {
        self.frames.clear();
    }
}

// 数据帧回调函数
#[no_mangle]
unsafe extern "C" fn get_frame(p_frame: *mut IMV_Frame, _p_user: *mut c_void) {
    // println!("收到帧");
    if p_frame.is_null() {
        println!("错误：收到空帧指针");
        return;
    }

    // 将帧添加到全局队列
    let frame = *p_frame;
    match FRAME_QUEUE.lock() {
        Ok(mut queue) => {
            if queue.push(frame) {
                println!("帧已添加到队列，当前队列长度：{}", queue.len());
            } else {
                println!("队列已满，丢弃帧");
            }
        }
        Err(e) => {
            println!("获取队列锁失败：{:?}", e);
        }
    }
}

// 处理帧的线程函数
fn process_frame(handle_contain: ImvHandleTs, running: Arc<Mutex<bool>>) {
    while let Ok(is_running) = running.lock() {
        if !*is_running {
            break;
        }

        // 尝试从队列获取帧
        let frame_opt = match FRAME_QUEUE.lock() {
            Ok(mut queue) => queue.pop(),
            Err(_) => None,
        };

        // 如果有帧，处理它
        if let Some(mut frame) = frame_opt {
            unsafe {
                let n_dst_buf_size = frame.width() * frame.height() * 3;
                // let layout = Layout::new::<dst_buffer>();
                let layout = Layout::array::<c_char>(n_dst_buf_size as usize)
                    .expect("Failed to calculate array layout.");
                let ptr = alloc(layout);
                // 检查分配是否成功
                if ptr.is_null() {
                    panic!("内存分配失败");
                }
                let mut convert_param: IMV_PixelConvertParam = IMV_PixelConvertParam {
                    nWidth: frame.width(),
                    nHeight: frame.height(),
                    ePixelFormat: IMV_EPixelType::gvspPixelBayRG8,
                    pSrcData: frame.frame.pData,
                    nSrcDataLen: frame.width() * frame.height(),
                    nPaddingX: 0,
                    nPaddingY: 0,
                    eBayerDemosaic: IMV_EBayerDemosaic::demosaicBilinear,
                    eDstPixelFormat: IMV_EPixelType::gvspPixelRGB8,
                    pDstBuf: ptr,
                    nDstBufSize: frame.width() * frame.height() * 3,
                    nDstDataLen: 8,
                    nReserved: [0; 8],
                };
                let return_val = IMV_PixelConvert(handle_contain.handle(), &mut convert_param);
                if return_val != IMV_OK as i32 {
                    println!("Failed to convert pixel type");
                }

                frame.frame.frameInfo.pixelFormat = IMV_EPixelType::gvspPixelRGB8;
                frame.frame.pData = ptr;
                // Todo: Release pData

                let path = format!("./Dahua/{}.jpg", frame.frame.frameInfo.blockId);
                let cpath = CString::from_str(&path).expect("Failed to convert path to CString");
                File::create(path).expect("Failed to create file");
                let return_val = IMV_SaveImageToFile(
                    handle_contain.handle(),
                    &mut IMV_SaveImageToFileParam {
                        nWidth: frame.width(),
                        nHeight: frame.height(),
                        ePixelFormat: IMV_EPixelType::gvspPixelRGB8,
                        pSrcData: frame.frame.pData,
                        nSrcDataLen: frame.size(),
                        eImageType: IMV_ESaveFileType::typeSaveJpeg,
                        pImagePath: cpath.as_ptr() as *mut i8,
                        nQuality: 55,
                        eBayerDemosaic: IMV_EBayerDemosaic::demosaicBilinear,
                        nReserved: [0; 8],
                    },
                );
                if return_val != IMV_OK as i32 {
                    println!("Failed to save image, Error[{}]", return_val);
                }

                if !frame.frame.frameHandle.is_null() {
                    IMV_ReleaseFrame(frame.frame.frameHandle, &mut frame.frame);
                }

                dealloc(ptr, layout);
            }
        }
    }
    println!("处理帧线程退出");
}

pub fn pic_record() {
    let device_info_list = unsafe {
        let mut device_info_list: IMV_DeviceList = std::mem::zeroed();
        let _ret = IMV_EnumDevices(&mut device_info_list, IMV_EInterfaceType::interfaceTypeAll);
        device_info_list
    };

    let device_0 = unsafe {
        let device_ptr = device_info_list.pDevInfo;
        if device_ptr.is_null() {
            eprintln!("错误：设备指针为空");
            return;
        }

        match device_ptr.as_ref() {
            Some(dev) => dev,
            None => {
                eprintln!("错误：无法解引用设备指针");
                return;
            }
        }
    };

    display_device_info(device_0);

    unsafe {
        loop {
            let mut camera_index: c_uint = 0;
            let camera_index_ptr = &mut camera_index as *mut std::os::raw::c_uint as *mut c_void;
            let mut handle: IMV_HANDLE = std::ptr::null_mut();
            let return_value = IMV_CreateHandle(
                &mut handle as *mut IMV_HANDLE,
                IMV_ECreateHandleMode::modeByIndex,
                camera_index_ptr,
            );
            if return_value != IMV_OK as i32 {
                panic!("错误 {return_value}");
            }
            let handle_container = ImvHandleTs::new(handle);
            IMV_Open(handle);

            // 清空帧队列，确保干净状态开始
            if let Ok(mut queue) = FRAME_QUEUE.lock() {
                queue.clear();
            }

            // 创建线程控制标志
            let running = Arc::new(Mutex::new(true));
            let running_clone = Arc::clone(&running);

            // 创建处理帧的线程
            let process_thread = std::thread::spawn(move || {
                process_frame(handle_container, running_clone);
            });

            // 注册数据帧回调函数
            let null_ptr: *const i32 = std::ptr::null();
            let return_value = IMV_AttachGrabbing(handle, Some(get_frame), null_ptr as *mut c_void);
            if return_value != IMV_OK as i32 {
                println!("附加采集回调失败！错误码[{}]", return_value);
                break;
            }

            // 开始拉流
            let return_value = IMV_StartGrabbing(handle);
            if IMV_OK as i32 != return_value {
                println!("开始采集失败！错误码[{}]", return_value);
                break;
            }

            // 采集
            sleep(Duration::from_secs(1));

            // 停止拉流
            let return_value = IMV_StopGrabbing(handle);
            if IMV_OK as i32 != return_value {
                println!("停止采集失败！错误码[{}]", return_value);
                break;
            }

            // 停止处理帧线程
            if let Ok(mut is_running) = running.lock() {
                *is_running = false;
            }

            // 等待处理线程结束
            if let Err(e) = process_thread.join() {
                println!("等待处理线程结束时出错：{:?}", e);
            }

            // 显示队列中剩余的帧数量
            if let Ok(queue) = FRAME_QUEUE.lock() {
                println!("队列中剩余帧数：{}", queue.len());
            }

            // 关闭相机
            let return_value = IMV_Close(handle);
            if return_value != IMV_OK as i32 {
                panic!("关闭相机失败");
            }
            if !handle.is_null() {
                IMV_DestroyHandle(handle);
                println!("销毁相机句柄");
            }

            println!("退出\n");
            break;
        }
    }
}

fn display_device_info(device_info: &IMV_DeviceInfo) {
    println!("Vendor: {}", trsnsfer_i8_to_string(&device_info.vendorName));
    match device_info.nCameraType {
        0 => println!("Camera type: GigE"),
        1 => println!("Camera type: U3V"),
        2 => println!("Camera type: CameraLink"),
        3 => println!("Camera type: CameraSimulator"),
        _ => println!("Camera type: Unknown"),
    }
    println!("Model: {}", trsnsfer_i8_to_string(&device_info.modelName));
    println!(
        "Serial number: {}",
        trsnsfer_i8_to_string(&device_info.serialNumber)
    );
}

fn trsnsfer_i8_to_string(i8_array: &[i8]) -> String {
    let mut result = String::new();
    for i in i8_array.iter().take(256) {
        if *i != 0 {
            result.push(*i as u8 as char);
        }
    }
    result
}
