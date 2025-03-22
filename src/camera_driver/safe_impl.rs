use crate::camera_driver::bindings::*;
use std::marker::PhantomData;

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
