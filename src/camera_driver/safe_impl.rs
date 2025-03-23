use log::{error, info};

use crate::camera_driver::bindings::*;
use std::{
    ffi::{c_char, CString},
    marker::PhantomData,
};

// 创建一个包装器来安全地管理帧
pub struct ImvFrameTs {
    pub frame: IMV_Frame,
    _marker: PhantomData<*mut ()>, // 标记这个类型含有不能自动 Send/Sync 的数据
}

impl ImvFrameTs {
    pub fn new(frame: IMV_Frame) -> Self {
        Self {
            frame,
            _marker: PhantomData,
        }
    }

    pub fn height(&self) -> u32 {
        self.frame.frameInfo.height
    }

    pub fn width(&self) -> u32 {
        self.frame.frameInfo.width
    }

    pub fn size(&self) -> u32 {
        self.frame.frameInfo.size
    }
}

unsafe impl Send for ImvFrameTs {}

pub struct ImvHandleTs {
    handle: IMV_HANDLE,
    _marker: PhantomData<*mut ()>,
}

impl ImvHandleTs {
    pub fn new(handle: IMV_HANDLE) -> Self {
        Self {
            handle,
            _marker: PhantomData,
        }
    }

    pub fn handle(&self) -> IMV_HANDLE {
        self.handle
    }
}

unsafe impl Send for ImvHandleTs {}

pub fn display_device_info(device_info: &IMV_DeviceInfo) {
    info!("Vendor: {}", trsnsfer_i8_to_string(&device_info.vendorName));
    match device_info.nCameraType {
        0 => info!("Camera type: GigE"),
        1 => info!("Camera type: U3V"),
        2 => info!("Camera type: CameraLink"),
        3 => info!("Camera type: CameraSimulator"),
        _ => info!("Camera type: Unknown"),
    }
    info!("Model: {}", trsnsfer_i8_to_string(&device_info.modelName));
    info!(
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

#[allow(unused)]
pub fn get_cam_param(dev_handle_ts: &mut ImvHandleTs) -> Result<(), String> {
    let config_file_name =
        CString::new("/home/flamingo/Project/RustyRoboVision/cam_config/config_get.xml").unwrap();
    unsafe {
        let ret_val = IMV_SaveDeviceCfg(dev_handle_ts.handle, config_file_name.as_ptr());
        if ret_val != IMV_OK as i32 {
            error!("Failed to get camera param, Error Code[{}]", ret_val);
        }
    }
    Ok(())
}

pub fn set_cam_param(dev_handle_ts: &mut ImvHandleTs) -> Result<(), String> {
    let mut error_param_list = IMV_ErrorList {
        nParamCnt: 0u32,
        paramNameList: [IMV_String {
            str_: [0 as c_char; 256],
        }; 128],
    };
    let config_file_name =
        CString::new("/home/flamingo/Project/RustyRoboVision/cam_config/config.xml").unwrap();
    unsafe {
        let ret_val = IMV_LoadDeviceCfg(
            dev_handle_ts.handle,
            config_file_name.as_ptr(),
            &mut error_param_list,
        );
        if ret_val != (IMV_OK as i32) {
            error!("Failed to load param from config xml");
        }
    }
    Ok(())
}

#[allow(unused)]
pub fn get_statisitic_info(dev_handle_ts: &ImvHandleTs) -> Result<(), String> {
    let mut imv_stream_statistic_info =
        std::mem::MaybeUninit::<IMV_StreamStatsInfo_Union>::uninit();
    unsafe {
        imv_stream_statistic_info
            .as_mut_ptr()
            .write(IMV_StreamStatsInfo_Union {
                u3vStatisticsInfo: IMV_U3VStreamStatsInfo {
                    imageError: 0,
                    lostPacketBlock: 0,
                    nReserved0: [0; 10usize],
                    imageReceived: 0,
                    fps: 0.0f64,
                    bandwidth: 0.0f64,
                    nReserved: [0; 8usize],
                },
            });
        let mut stream_stats_info = IMV_StreamStatisticsInfo {
            nCameraType: IMV_ECameraType::typeU3vCamera,
            IMV_StreamStatsInfo_union: *imv_stream_statistic_info.as_ptr(),
        };
        IMV_GetStatisticsInfo(dev_handle_ts.handle, &mut stream_stats_info);
        let fps = stream_stats_info
            .IMV_StreamStatsInfo_union
            .u3vStatisticsInfo
            .fps;
        error!("{:?}", fps);
    }
    Ok(())
}
