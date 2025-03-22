#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types, unused)]

use std::os::raw::{
    c_char, c_int, c_long, c_schar, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
};

pub const _STDINT_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const _DEFAULT_SOURCE: u32 = 1;
pub const __GLIBC_USE_ISOC2X: u32 = 0;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const __USE_POSIX_IMPLICITLY: u32 = 1;
pub const _POSIX_SOURCE: u32 = 1;
pub const _POSIX_C_SOURCE: u32 = 200809;
pub const __USE_POSIX: u32 = 1;
pub const __USE_POSIX2: u32 = 1;
pub const __USE_POSIX199309: u32 = 1;
pub const __USE_POSIX199506: u32 = 1;
pub const __USE_XOPEN2K: u32 = 1;
pub const __USE_XOPEN2K8: u32 = 1;
pub const _ATFILE_SOURCE: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
pub const __SYSCALL_WORDSIZE: u32 = 64;
pub const __TIMESIZE: u32 = 64;
pub const __USE_MISC: u32 = 1;
pub const __USE_ATFILE: u32 = 1;
pub const __USE_FORTIFY_LEVEL: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_SCANF: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_60559_BFP__: u32 = 201404;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_IEC_60559_COMPLEX__: u32 = 201404;
pub const __STDC_ISO_10646__: u32 = 201706;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 35;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __glibc_c99_flexarr_available: u32 = 1;
pub const __LDOUBLE_REDIRECTS_TO_FLOAT128_ABI: u32 = 0;
pub const __HAVE_GENERIC_SELECTION: u32 = 1;
pub const __GLIBC_USE_LIB_EXT2: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT_C2X: u32 = 0;
pub const __GLIBC_USE_IEC_60559_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT_C2X: u32 = 0;
pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 0;
pub const _BITS_TYPES_H: u32 = 1;
pub const _BITS_TYPESIZES_H: u32 = 1;
pub const __OFF_T_MATCHES_OFF64_T: u32 = 1;
pub const __INO_T_MATCHES_INO64_T: u32 = 1;
pub const __RLIM_T_MATCHES_RLIM64_T: u32 = 1;
pub const __STATFS_MATCHES_STATFS64: u32 = 1;
pub const __KERNEL_OLD_TIMEVAL_MATCHES_TIMEVAL64: u32 = 1;
pub const __FD_SETSIZE: u32 = 1024;
pub const _BITS_TIME64_H: u32 = 1;
pub const _BITS_WCHAR_H: u32 = 1;
pub const _BITS_STDINT_INTN_H: u32 = 1;
pub const _BITS_STDINT_UINTN_H: u32 = 1;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i64 = -9223372036854775808;
pub const INT_FAST32_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u64 = 9223372036854775807;
pub const INT_FAST32_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: i32 = -1;
pub const UINT_FAST32_MAX: i32 = -1;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const UINTPTR_MAX: i32 = -1;
pub const PTRDIFF_MIN: i64 = -9223372036854775808;
pub const PTRDIFF_MAX: u64 = 9223372036854775807;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const SIZE_MAX: i32 = -1;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 4294967295;
pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub const IMV_OK: u32 = 0;
pub const IMV_ERROR: i32 = -101;
pub const IMV_INVALID_HANDLE: i32 = -102;
pub const IMV_INVALID_PARAM: i32 = -103; // 参数错误
pub const IMV_INVALID_FRAME_HANDLE: i32 = -104;
pub const IMV_INVALID_FRAME: i32 = -105;
pub const IMV_INVALID_RESOURCE: i32 = -106;
pub const IMV_INVALID_IP: i32 = -107;
pub const IMV_NO_MEMORY: i32 = -108;
pub const IMV_INSUFFICIENT_MEMORY: i32 = -109;
pub const IMV_ERROR_PROPERTY_TYPE: i32 = -110;
pub const IMV_INVALID_ACCESS: i32 = -111;
pub const IMV_INVALID_RANGE: i32 = -112;
pub const IMV_NOT_SUPPORT: i32 = -113;
pub const IMV_RESTORE_STREAM: i32 = -114;
pub const IMV_RECONNECT_DEVICE: i32 = -115;
pub const IMV_NOT_AVAILABLE: i32 = -116;
pub const IMV_NOT_GRABBING: i32 = -117;
pub const IMV_NOT_CONNECTED: i32 = -118;
pub const IMV_TIMEOUT: i32 = -119;
pub const IMV_IS_CONNECTED: i32 = -120;
pub const IMV_IS_GRABBING: i32 = -121;
pub const IMV_INVOCATION_ERROR: i32 = -122;
pub const IMV_SYSTEM_ERROR: i32 = -123;
pub const IMV_OPENFILE_ERROR: i32 = -124;
pub const IMV_MAX_DEVICE_ENUM_NUM: u32 = 100;
pub const IMV_MAX_STRING_LENTH: u32 = 256;
pub const IMV_MAX_ERROR_LIST_NUM: u32 = 128;
pub const IMV_GVSP_PIX_MONO: u32 = 16777216;
pub const IMV_GVSP_PIX_RGB: u32 = 33554432;
pub const IMV_GVSP_PIX_COLOR: u32 = 33554432;
pub const IMV_GVSP_PIX_CUSTOM: u32 = 2147483648;
pub const IMV_GVSP_PIX_COLOR_MASK: u32 = 4278190080;
pub const IMV_GVSP_PIX_OCCUPY1BIT: u32 = 65536;
pub const IMV_GVSP_PIX_OCCUPY2BIT: u32 = 131072;
pub const IMV_GVSP_PIX_OCCUPY4BIT: u32 = 262144;
pub const IMV_GVSP_PIX_OCCUPY8BIT: u32 = 524288;
pub const IMV_GVSP_PIX_OCCUPY12BIT: u32 = 786432;
pub const IMV_GVSP_PIX_OCCUPY16BIT: u32 = 1048576;
pub const IMV_GVSP_PIX_OCCUPY24BIT: u32 = 1572864;
pub const IMV_GVSP_PIX_OCCUPY32BIT: u32 = 2097152;
pub const IMV_GVSP_PIX_OCCUPY36BIT: u32 = 2359296;
pub const IMV_GVSP_PIX_OCCUPY48BIT: u32 = 3145728;
pub const IMV_MSG_EVENT_ID_EXPOSURE_END: u32 = 36865;
pub const IMV_MSG_EVENT_ID_FRAME_TRIGGER: u32 = 36866;
pub const IMV_MSG_EVENT_ID_FRAME_START: u32 = 36867;
pub const IMV_MSG_EVENT_ID_ACQ_START: u32 = 36868;
pub const IMV_MSG_EVENT_ID_ACQ_TRIGGER: u32 = 36869;
pub const IMV_MSG_EVENT_ID_DATA_READ_OUT: u32 = 36870;
pub type __u_char = c_uchar;
pub type __u_short = c_ushort;
pub type __u_int = c_uint;
pub type __u_long = c_ulong;
pub type __int8_t = c_schar;
pub type __uint8_t = c_uchar;
pub type __int16_t = c_short;
pub type __uint16_t = c_ushort;
pub type __int32_t = c_int;
pub type __uint32_t = c_uint;
pub type __int64_t = c_long;
pub type __uint64_t = c_ulong;
pub type __int_least8_t = __int8_t;
pub type __uint_least8_t = __uint8_t;
pub type __int_least16_t = __int16_t;
pub type __uint_least16_t = __uint16_t;
pub type __int_least32_t = __int32_t;
pub type __uint_least32_t = __uint32_t;
pub type __int_least64_t = __int64_t;
pub type __uint_least64_t = __uint64_t;
pub type __quad_t = c_long;
pub type __u_quad_t = c_ulong;
pub type __intmax_t = c_long;
pub type __uintmax_t = c_ulong;
pub type __dev_t = c_ulong;
pub type __uid_t = c_uint;
pub type __gid_t = c_uint;
pub type __ino_t = c_ulong;
pub type __ino64_t = c_ulong;
pub type __mode_t = c_uint;
pub type __nlink_t = c_ulong;
pub type __off_t = c_long;
pub type __off64_t = c_long;
pub type __pid_t = c_int;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __fsid_t {
    pub __val: [c_int; 2usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __fsid_t"][std::mem::size_of::<__fsid_t>() - 8usize];
    ["Alignment of __fsid_t"][std::mem::align_of::<__fsid_t>() - 4usize];
    ["Offset of field: __fsid_t::__val"][std::mem::offset_of!(__fsid_t, __val) - 0usize];
};

pub type __clock_t = c_long;
pub type __rlim_t = c_ulong;
pub type __rlim64_t = c_ulong;
pub type __id_t = c_uint;
pub type __time_t = c_long;
pub type __useconds_t = c_uint;
pub type __suseconds_t = c_long;
pub type __suseconds64_t = c_long;
pub type __daddr_t = c_int;
pub type __key_t = c_int;
pub type __clockid_t = c_int;
pub type __timer_t = *mut c_void;
pub type __blksize_t = c_long;
pub type __blkcnt_t = c_long;
pub type __blkcnt64_t = c_long;
pub type __fsblkcnt_t = c_ulong;
pub type __fsblkcnt64_t = c_ulong;
pub type __fsfilcnt_t = c_ulong;
pub type __fsfilcnt64_t = c_ulong;
pub type __fsword_t = c_long;
pub type __ssize_t = c_long;
pub type __syscall_slong_t = c_long;
pub type __syscall_ulong_t = c_ulong;
pub type __loff_t = __off64_t;
pub type __caddr_t = *mut c_char;
pub type __intptr_t = c_long;
pub type __socklen_t = c_uint;
pub type __sig_atomic_t = c_int;
pub type int_least8_t = __int_least8_t;
pub type int_least16_t = __int_least16_t;
pub type int_least32_t = __int_least32_t;
pub type int_least64_t = __int_least64_t;
pub type uint_least8_t = __uint_least8_t;
pub type uint_least16_t = __uint_least16_t;
pub type uint_least32_t = __uint_least32_t;
pub type uint_least64_t = __uint_least64_t;
pub type int_fast8_t = c_schar;
pub type int_fast16_t = c_long;
pub type int_fast32_t = c_long;
pub type int_fast64_t = c_long;
pub type uint_fast8_t = c_uchar;
pub type uint_fast16_t = c_ulong;
pub type uint_fast32_t = c_ulong;
pub type uint_fast64_t = c_ulong;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type bool_ = c_char;
pub type IMV_HANDLE = *mut c_void;
pub type IMV_FRAME_HANDLE = *mut c_void;

/// 枚举：属性类型
pub mod IMV_EFeatureType {
    pub type IMV_EFeatureType = std::os::raw::c_uint;

    /// 整型数
    pub const featureInt: IMV_EFeatureType = 268435456;
    /// 浮点数
    pub const featureFloat: IMV_EFeatureType = 536870912;
    /// 枚举
    pub const featureEnum: IMV_EFeatureType = 805306368;
    /// 布尔
    pub const featureBool: IMV_EFeatureType = 1073741824;
    /// 字符串
    pub const featureString: IMV_EFeatureType = 1342177280;
    /// 命令
    pub const featureCommand: IMV_EFeatureType = 1610612736;
    /// 分组节点
    pub const featureGroup: IMV_EFeatureType = 1879048192;
    /// 寄存器节点
    pub const featureReg: IMV_EFeatureType = 2147483648;
    /// 未定义
    pub const featureUndefined: IMV_EFeatureType = 2415919104;
}

/// 枚举：接口类型
pub mod IMV_EInterfaceType {
    pub type IMV_EInterfaceType = std::os::raw::c_uint;

    /// 网卡接口类型
    pub const interfaceTypeGige: IMV_EInterfaceType = 1;
    /// USB3.0接口类型
    pub const interfaceTypeUsb3: IMV_EInterfaceType = 2;
    /// CAMERALINK接口类型
    pub const interfaceTypeCL: IMV_EInterfaceType = 4;
    /// PCIe接口类型
    pub const interfaceTypePCIe: IMV_EInterfaceType = 8;
    /// 忽略接口类型（CAMERALINK接口除外)
    pub const interfaceTypeAll: IMV_EInterfaceType = 0;
    /// 无效接口类型
    pub const interfaceInvalidType: IMV_EInterfaceType = 4294967295;
}

/// 枚举：设备类型
pub mod IMV_ECameraType {
    pub type IMV_ECameraType = std::os::raw::c_uint;

    /// GIGE相机
    pub const typeGigeCamera: IMV_ECameraType = 0;
    /// USB3.0相机
    pub const typeU3vCamera: IMV_ECameraType = 1;
    /// CAMERALINK 相机
    pub const typeCLCamera: IMV_ECameraType = 2;
    /// PCIe相机
    pub const typePCIeCamera: IMV_ECameraType = 3;
    /// 未知类型
    pub const typeUndefinedCamera: IMV_ECameraType = 255;
}

pub mod IMV_ECreateHandleMode {
    pub type IMV_ECreateHandleMode = std::os::raw::c_uint;

    /// 通过已枚举设备的索引(从0开始，比如 0, 1, 2...)
    pub const modeByIndex: IMV_ECreateHandleMode = 0u32 as std::os::raw::c_uint;
    /// 通过设备键\"厂商:序列号\"
    pub const modeByCameraKey: IMV_ECreateHandleMode = 1u32 as std::os::raw::c_uint;
    /// 通过设备自定义名
    pub const modeByDeviceUserID: IMV_ECreateHandleMode = 2u32;
    /// 通过设备IP地址
    pub const modeByIPAddress: IMV_ECreateHandleMode = 3u32;
}

/// 枚举：访问权限
pub mod IMV_ECameraAccessPermission {
    pub type IMV_ECameraAccessPermission = std::os::raw::c_uint;

    /// GigE相机没有被连接
    pub const accessPermissionOpen: IMV_ECameraAccessPermission = 0;
    /// 独占访问权限
    pub const accessPermissionExclusive: IMV_ECameraAccessPermission = 1;
    /// 非独占控制权限,其他App允许读取所有寄存器
    pub const accessPermissionControl: IMV_ECameraAccessPermission = 2;
    /// 切换控制访问权限
    pub const accessPermissionControlWithSwitchover: IMV_ECameraAccessPermission = 3;
    /// 非独占访问权限,以读的模式打开设备
    pub const accessPermissionMonitor: IMV_ECameraAccessPermission = 4;
    /// 无法确定
    pub const accessPermissionUnknown: IMV_ECameraAccessPermission = 254;
    /// 未定义访问权限
    pub const accessPermissionUndefined: IMV_ECameraAccessPermission = 255;
}

/// 枚举：抓图策略
pub mod IMV_EGrabStrategy {
    pub type IMV_EGrabStrategy = std::os::raw::c_uint;

    /// 按到达顺序处理图片
    pub const grabStrartegySequential: IMV_EGrabStrategy = 0;
    /// 获取最新的图片
    pub const grabStrartegyLatestImage: IMV_EGrabStrategy = 1;
    /// 等待获取下一张图片(只针对GigE相机)
    pub const grabStrartegyUpcomingImage: IMV_EGrabStrategy = 2;
    /// 未定义
    pub const grabStrartegyUndefined: IMV_EGrabStrategy = 3;
}

pub mod IMV_EEventStatus {
    pub type IMV_EEventStatus = std::os::raw::c_uint;

    /// 正常流事件
    pub const streamEventNormal: IMV_EEventStatus = 1;
    /// 丢帧事件
    pub const streamEventLostFrame: IMV_EEventStatus = 2;
    /// 丢包事件
    pub const streamEventLostPacket: IMV_EEventStatus = 3;
    /// 图像错误事件
    pub const streamEventImageError: IMV_EEventStatus = 4;
    /// 取流错误事件
    pub const streamEventStreamChannelError: IMV_EEventStatus = 5;
    /// 太多连续重传
    pub const streamEventTooManyConsecutiveResends: IMV_EEventStatus = 6;
    /// 太多丢包
    pub const streamEventTooManyLostPacket: IMV_EEventStatus = 7;
}

/// 枚举：图像转换Bayer格式所用的算法
pub mod IMV_EBayerDemosaic {
    pub type IMV_EBayerDemosaic = std::os::raw::c_uint;

    /// 最近邻
    pub const demosaicNearestNeighbor: IMV_EBayerDemosaic = 0;
    /// 双线性
    pub const demosaicBilinear: IMV_EBayerDemosaic = 1;
    /// 边缘检测
    pub const demosaicEdgeSensing: IMV_EBayerDemosaic = 2;
    /// 不支持
    pub const demosaicNotSupport: IMV_EBayerDemosaic = 255;
}

/// 枚举：事件类型
pub mod IMV_EVType {
    pub type IMV_EVType = std::os::raw::c_uint;

    /// 设备离线通知
    pub const offLine: IMV_EVType = 0;
    /// 设备在线通知
    pub const onLine: IMV_EVType = 1;
}

/// 枚举：视频格式
pub mod IMV_EVideoType {
    pub type IMV_EVideoType = std::os::raw::c_uint;

    /// AVI格式
    pub const typeVideoFormatAVI: IMV_EVideoType = 0;
    /// 不支持
    pub const typeVideoFormatNotSupport: IMV_EVideoType = 255;
}

/// 枚举：图像翻转类型
pub mod IMV_EFlipType {
    pub type IMV_EFlipType = std::os::raw::c_uint;

    /// 垂直(Y轴)翻转
    pub const typeFlipVertical: IMV_EFlipType = 0;
    /// 水平(X轴)翻转
    pub const typeFlipHorizontal: IMV_EFlipType = 1;
}

/// 枚举：顺时针旋转角度
pub mod IMV_ERotationAngle {
    pub type IMV_ERotationAngle = std::os::raw::c_uint;

    /// 顺时针旋转90度
    pub const rotationAngle90: IMV_ERotationAngle = 0;
    /// 顺时针旋转180度
    pub const rotationAngle180: IMV_ERotationAngle = 1;
    /// 顺时针旋转270度
    pub const rotationAngle270: IMV_ERotationAngle = 2;
}

/// 枚举：图像格式
pub mod IMV_EPixelType {
    pub type IMV_EPixelType = std::os::raw::c_int;

    pub const gvspPixelTypeUndefined: IMV_EPixelType = -1;
    pub const gvspPixelMono1p: IMV_EPixelType = 16842807;
    pub const gvspPixelMono2p: IMV_EPixelType = 16908344;
    pub const gvspPixelMono4p: IMV_EPixelType = 17039417;
    pub const gvspPixelMono8: IMV_EPixelType = 17301505;
    pub const gvspPixelMono8S: IMV_EPixelType = 17301506;
    pub const gvspPixelMono10: IMV_EPixelType = 17825795;
    pub const gvspPixelMono10Packed: IMV_EPixelType = 17563652;
    pub const gvspPixelMono12: IMV_EPixelType = 17825797;
    pub const gvspPixelMono12Packed: IMV_EPixelType = 17563654;
    pub const gvspPixelMono14: IMV_EPixelType = 17825829;
    pub const gvspPixelMono16: IMV_EPixelType = 17825799;
    pub const gvspPixelBayGR8: IMV_EPixelType = 17301512;
    pub const gvspPixelBayRG8: IMV_EPixelType = 17301513;
    pub const gvspPixelBayGB8: IMV_EPixelType = 17301514;
    pub const gvspPixelBayBG8: IMV_EPixelType = 17301515;
    pub const gvspPixelBayGR10: IMV_EPixelType = 17825804;
    pub const gvspPixelBayRG10: IMV_EPixelType = 17825805;
    pub const gvspPixelBayGB10: IMV_EPixelType = 17825806;
    pub const gvspPixelBayBG10: IMV_EPixelType = 17825807;
    pub const gvspPixelBayGR12: IMV_EPixelType = 17825808;
    pub const gvspPixelBayRG12: IMV_EPixelType = 17825809;
    pub const gvspPixelBayGB12: IMV_EPixelType = 17825810;
    pub const gvspPixelBayBG12: IMV_EPixelType = 17825811;
    pub const gvspPixelBayGR10Packed: IMV_EPixelType = 17563686;
    pub const gvspPixelBayRG10Packed: IMV_EPixelType = 17563687;
    pub const gvspPixelBayGB10Packed: IMV_EPixelType = 17563688;
    pub const gvspPixelBayBG10Packed: IMV_EPixelType = 17563689;
    pub const gvspPixelBayGR12Packed: IMV_EPixelType = 17563690;
    pub const gvspPixelBayRG12Packed: IMV_EPixelType = 17563691;
    pub const gvspPixelBayGB12Packed: IMV_EPixelType = 17563692;
    pub const gvspPixelBayBG12Packed: IMV_EPixelType = 17563693;
    pub const gvspPixelBayGR16: IMV_EPixelType = 17825838;
    pub const gvspPixelBayRG16: IMV_EPixelType = 17825839;
    pub const gvspPixelBayGB16: IMV_EPixelType = 17825840;
    pub const gvspPixelBayBG16: IMV_EPixelType = 17825841;
    pub const gvspPixelRGB8: IMV_EPixelType = 35127316;
    pub const gvspPixelBGR8: IMV_EPixelType = 35127317;
    pub const gvspPixelRGBA8: IMV_EPixelType = 35651606;
    pub const gvspPixelBGRA8: IMV_EPixelType = 35651607;
    pub const gvspPixelRGB10: IMV_EPixelType = 36700184;
    pub const gvspPixelBGR10: IMV_EPixelType = 36700185;
    pub const gvspPixelRGB12: IMV_EPixelType = 36700186;
    pub const gvspPixelBGR12: IMV_EPixelType = 36700187;
    pub const gvspPixelRGB16: IMV_EPixelType = 36700211;
    pub const gvspPixelRGB10V1Packed: IMV_EPixelType = 35651612;
    pub const gvspPixelRGB10P32: IMV_EPixelType = 35651613;
    pub const gvspPixelRGB12V1Packed: IMV_EPixelType = 35913780;
    pub const gvspPixelRGB565P: IMV_EPixelType = 34603061;
    pub const gvspPixelBGR565P: IMV_EPixelType = 34603062;
    pub const gvspPixelYUV411_8_UYYVYY: IMV_EPixelType = 34340894;
    pub const gvspPixelYUV422_8_UYVY: IMV_EPixelType = 34603039;
    pub const gvspPixelYUV422_8: IMV_EPixelType = 34603058;
    pub const gvspPixelYUV8_UYV: IMV_EPixelType = 35127328;
    pub const gvspPixelYCbCr8CbYCr: IMV_EPixelType = 35127354;
    pub const gvspPixelYCbCr422_8: IMV_EPixelType = 34603067;
    pub const gvspPixelYCbCr422_8_CbYCrY: IMV_EPixelType = 34603075;
    pub const gvspPixelYCbCr411_8_CbYYCrYY: IMV_EPixelType = 34340924;
    pub const gvspPixelYCbCr601_8_CbYCr: IMV_EPixelType = 35127357;
    pub const gvspPixelYCbCr601_422_8: IMV_EPixelType = 34603070;
    pub const gvspPixelYCbCr601_422_8_CbYCrY: IMV_EPixelType = 34603076;
    pub const gvspPixelYCbCr601_411_8_CbYYCrYY: IMV_EPixelType = 34340927;
    pub const gvspPixelYCbCr709_8_CbYCr: IMV_EPixelType = 35127360;
    pub const gvspPixelYCbCr709_422_8: IMV_EPixelType = 34603073;
    pub const gvspPixelYCbCr709_422_8_CbYCrY: IMV_EPixelType = 34603077;
    pub const gvspPixelYCbCr709_411_8_CbYYCrYY: IMV_EPixelType = 34340930;
    pub const gvspPixelYUV420SP_NV12: IMV_EPixelType = 34373633;
    pub const gvspPixelRGB8Planar: IMV_EPixelType = 35127329;
    pub const gvspPixelRGB10Planar: IMV_EPixelType = 36700194;
    pub const gvspPixelRGB12Planar: IMV_EPixelType = 36700195;
    pub const gvspPixelRGB16Planar: IMV_EPixelType = 36700196;
    pub const gvspPixelBayRG10p: IMV_EPixelType = 17432664;
    pub const gvspPixelBayRG12p: IMV_EPixelType = 17563737;
    pub const gvspPixelMono1c: IMV_EPixelType = 18874623;
    pub const gvspPixelMono1e: IMV_EPixelType = 17305599;
}

/// brief 传输模式(gige)
pub mod IMV_TransmissionType {
    pub type IMV_TransmissionType = std::os::raw::c_uint;

    /// 单播模式
    pub const transTypeUnicast: IMV_TransmissionType = 0;
    /// 组播模式
    pub const transTypeMulticast: IMV_TransmissionType = 1;
}

/// brief 字符串信息
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMV_String {
    /// 字符串.长度不超过256
    pub str_: [c_char; 256usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMV_String"][std::mem::size_of::<IMV_String>() - 256usize];
    ["Alignment of IMV_String"][std::mem::align_of::<IMV_String>() - 1usize];
    ["Offset of field: IMV_String::str_"][std::mem::offset_of!(IMV_String, str_) - 0usize];
};

/// brief GigE网卡信息
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMV_GigEInterfaceInfo {
    /// 网卡描述信息
    pub description: [c_char; 256usize],
    /// 网卡Mac地址
    pub macAddress: [c_char; 256usize],
    /// 设备Ip地址
    pub ipAddress: [c_char; 256usize],
    /// 子网掩码
    pub subnetMask: [c_char; 256usize],
    /// 默认网关
    pub defaultGateWay: [c_char; 256usize],
    /// 保留
    pub chReserved: [[c_char; 256usize]; 5usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMV_GigEInterfaceInfo"][std::mem::size_of::<IMV_GigEInterfaceInfo>() - 2560usize];
    ["Alignment of IMV_GigEInterfaceInfo"][std::mem::align_of::<IMV_GigEInterfaceInfo>() - 1usize];
    ["Offset of field: IMV_GigEInterfaceInfo::description"]
        [std::mem::offset_of!(IMV_GigEInterfaceInfo, description) - 0usize];
    ["Offset of field: IMV_GigEInterfaceInfo::macAddress"]
        [std::mem::offset_of!(IMV_GigEInterfaceInfo, macAddress) - 256usize];
    ["Offset of field: IMV_GigEInterfaceInfo::ipAddress"]
        [std::mem::offset_of!(IMV_GigEInterfaceInfo, ipAddress) - 512usize];
    ["Offset of field: IMV_GigEInterfaceInfo::subnetMask"]
        [std::mem::offset_of!(IMV_GigEInterfaceInfo, subnetMask) - 768usize];
    ["Offset of field: IMV_GigEInterfaceInfo::defaultGateWay"]
        [std::mem::offset_of!(IMV_GigEInterfaceInfo, defaultGateWay) - 1024usize];
    ["Offset of field: IMV_GigEInterfaceInfo::chReserved"]
        [std::mem::offset_of!(IMV_GigEInterfaceInfo, chReserved) - 1280usize];
};

/// brief USB接口信息
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMV_UsbInterfaceInfo {
    /// USB接口描述信息
    pub description: [c_char; 256usize],
    /// USB接口Vendor ID
    pub vendorID: [c_char; 256usize],
    /// USB接口设备ID
    pub deviceID: [c_char; 256usize],
    /// USB接口Subsystem ID
    pub subsystemID: [c_char; 256usize],
    /// USB接口Revision
    pub revision: [c_char; 256usize],
    /// USB接口speed
    pub speed: [c_char; 256usize],
    /// 保留
    pub chReserved: [[c_char; 256usize]; 4usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMV_UsbInterfaceInfo"][std::mem::size_of::<IMV_UsbInterfaceInfo>() - 2560usize];
    ["Alignment of IMV_UsbInterfaceInfo"][std::mem::align_of::<IMV_UsbInterfaceInfo>() - 1usize];
    ["Offset of field: IMV_UsbInterfaceInfo::description"]
        [std::mem::offset_of!(IMV_UsbInterfaceInfo, description) - 0usize];
    ["Offset of field: IMV_UsbInterfaceInfo::vendorID"]
        [std::mem::offset_of!(IMV_UsbInterfaceInfo, vendorID) - 256usize];
    ["Offset of field: IMV_UsbInterfaceInfo::deviceID"]
        [std::mem::offset_of!(IMV_UsbInterfaceInfo, deviceID) - 512usize];
    ["Offset of field: IMV_UsbInterfaceInfo::subsystemID"]
        [std::mem::offset_of!(IMV_UsbInterfaceInfo, subsystemID) - 768usize];
    ["Offset of field: IMV_UsbInterfaceInfo::revision"]
        [std::mem::offset_of!(IMV_UsbInterfaceInfo, revision) - 1024usize];
    ["Offset of field: IMV_UsbInterfaceInfo::speed"]
        [std::mem::offset_of!(IMV_UsbInterfaceInfo, speed) - 1280usize];
    ["Offset of field: IMV_UsbInterfaceInfo::chReserved"]
        [std::mem::offset_of!(IMV_UsbInterfaceInfo, chReserved) - 1536usize];
};

/// brief GigE设备信息
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMV_GigEDeviceInfo {
    /// 设备支持的IP配置选项
    /// value:4 相机只支持LLA
    /// value:5 相机支持LLA和Persistent IP
    /// value:6 相机支持LLA和DHCP
    /// value:7 相机支持LLA、DHCP和Persistent IP
    /// value:0 获取失败
    pub nIpConfigOptions: c_uint,
    /// 设备当前的IP配置选项
    /// value:4 LLA处于活动状态
    /// value:5 LLA和Persistent IP处于活动状态
    /// value:6 LLA和DHCP处于活动状态
    /// value:7 LLA、DHCP和Persistent IP处于活动状态
    /// value:0 获取失败
    pub nIpConfigCurrent: c_uint,
    /// 保留
    pub nReserved: [c_uint; 3usize],
    /// 设备Mac地址
    pub macAddress: [c_char; 256usize],
    /// 设备Ip地址
    pub ipAddress: [c_char; 256usize],
    /// 子网掩码
    pub subnetMask: [c_char; 256usize],
    /// 默认网关
    pub defaultGateWay: [c_char; 256usize],
    /// 网络协议版本
    pub protocolVersion: [c_char; 256usize],
    /// Ip配置有效性
    /// Ip配置有效时字符串值\"Valid\
    /// Ip配置无效时字符串值\"Invalid On This Interface\"
    pub ipConfiguration: [c_char; 256usize],
    /// 保留
    pub chReserved: [[c_char; 256usize]; 6usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMV_GigEDeviceInfo"][std::mem::size_of::<IMV_GigEDeviceInfo>() - 3092usize];
    ["Alignment of IMV_GigEDeviceInfo"][std::mem::align_of::<IMV_GigEDeviceInfo>() - 4usize];
    ["Offset of field: IMV_GigEDeviceInfo::nIpConfigOptions"]
        [std::mem::offset_of!(IMV_GigEDeviceInfo, nIpConfigOptions) - 0usize];
    ["Offset of field: IMV_GigEDeviceInfo::nIpConfigCurrent"]
        [std::mem::offset_of!(IMV_GigEDeviceInfo, nIpConfigCurrent) - 4usize];
    ["Offset of field: IMV_GigEDeviceInfo::nReserved"]
        [std::mem::offset_of!(IMV_GigEDeviceInfo, nReserved) - 8usize];
    ["Offset of field: IMV_GigEDeviceInfo::macAddress"]
        [std::mem::offset_of!(IMV_GigEDeviceInfo, macAddress) - 20usize];
    ["Offset of field: IMV_GigEDeviceInfo::ipAddress"]
        [std::mem::offset_of!(IMV_GigEDeviceInfo, ipAddress) - 276usize];
    ["Offset of field: IMV_GigEDeviceInfo::subnetMask"]
        [std::mem::offset_of!(IMV_GigEDeviceInfo, subnetMask) - 532usize];
    ["Offset of field: IMV_GigEDeviceInfo::defaultGateWay"]
        [std::mem::offset_of!(IMV_GigEDeviceInfo, defaultGateWay) - 788usize];
    ["Offset of field: IMV_GigEDeviceInfo::protocolVersion"]
        [std::mem::offset_of!(IMV_GigEDeviceInfo, protocolVersion) - 1044usize];
    ["Offset of field: IMV_GigEDeviceInfo::ipConfiguration"]
        [std::mem::offset_of!(IMV_GigEDeviceInfo, ipConfiguration) - 1300usize];
    ["Offset of field: IMV_GigEDeviceInfo::chReserved"]
        [std::mem::offset_of!(IMV_GigEDeviceInfo, chReserved) - 1556usize];
};

/// brief Usb设备信息
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMV_UsbDeviceInfo {
    /// true支持，false不支持，其他值 非法。
    pub bLowSpeedSupported: bool_,
    /// true支持，false不支持，其他值 非法。
    pub bFullSpeedSupported: bool_,
    /// true支持，false不支持，其他值 非法。
    pub bHighSpeedSupported: bool_,
    /// true支持，false不支持，其他值 非法。
    pub bSuperSpeedSupported: bool_,
    /// true安装，false未安装，其他值 非法。
    pub bDriverInstalled: bool_,
    /// 保留
    pub boolReserved: [bool_; 3usize],
    /// 保留
    pub Reserved: [c_uint; 4usize],
    /// 配置有效性
    pub configurationValid: [c_char; 256usize],
    /// GenCP 版本
    pub genCPVersion: [c_char; 256usize],
    /// U3V 版本号
    pub u3vVersion: [c_char; 256usize],
    /// 设备引导号
    pub deviceGUID: [c_char; 256usize],
    /// 设备系列号
    pub familyName: [c_char; 256usize],
    /// 设备序列号
    pub u3vSerialNumber: [c_char; 256usize],
    /// 设备传输速度
    pub speed: [c_char; 256usize],
    /// 设备最大供电量
    pub maxPower: [c_char; 256usize],
    /// 保留
    pub chReserved: [[c_char; 256usize]; 4usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMV_UsbDeviceInfo"][std::mem::size_of::<IMV_UsbDeviceInfo>() - 3096usize];
    ["Alignment of IMV_UsbDeviceInfo"][std::mem::align_of::<IMV_UsbDeviceInfo>() - 4usize];
    ["Offset of field: IMV_UsbDeviceInfo::bLowSpeedSupported"]
        [std::mem::offset_of!(IMV_UsbDeviceInfo, bLowSpeedSupported) - 0usize];
    ["Offset of field: IMV_UsbDeviceInfo::bFullSpeedSupported"]
        [std::mem::offset_of!(IMV_UsbDeviceInfo, bFullSpeedSupported) - 1usize];
    ["Offset of field: IMV_UsbDeviceInfo::bHighSpeedSupported"]
        [std::mem::offset_of!(IMV_UsbDeviceInfo, bHighSpeedSupported) - 2usize];
    ["Offset of field: IMV_UsbDeviceInfo::bSuperSpeedSupported"]
        [std::mem::offset_of!(IMV_UsbDeviceInfo, bSuperSpeedSupported) - 3usize];
    ["Offset of field: IMV_UsbDeviceInfo::bDriverInstalled"]
        [std::mem::offset_of!(IMV_UsbDeviceInfo, bDriverInstalled) - 4usize];
    ["Offset of field: IMV_UsbDeviceInfo::boolReserved"]
        [std::mem::offset_of!(IMV_UsbDeviceInfo, boolReserved) - 5usize];
    ["Offset of field: IMV_UsbDeviceInfo::Reserved"]
        [std::mem::offset_of!(IMV_UsbDeviceInfo, Reserved) - 8usize];
    ["Offset of field: IMV_UsbDeviceInfo::configurationValid"]
        [std::mem::offset_of!(IMV_UsbDeviceInfo, configurationValid) - 24usize];
    ["Offset of field: IMV_UsbDeviceInfo::genCPVersion"]
        [std::mem::offset_of!(IMV_UsbDeviceInfo, genCPVersion) - 280usize];
    ["Offset of field: IMV_UsbDeviceInfo::u3vVersion"]
        [std::mem::offset_of!(IMV_UsbDeviceInfo, u3vVersion) - 536usize];
    ["Offset of field: IMV_UsbDeviceInfo::deviceGUID"]
        [std::mem::offset_of!(IMV_UsbDeviceInfo, deviceGUID) - 792usize];
    ["Offset of field: IMV_UsbDeviceInfo::familyName"]
        [std::mem::offset_of!(IMV_UsbDeviceInfo, familyName) - 1048usize];
    ["Offset of field: IMV_UsbDeviceInfo::u3vSerialNumber"]
        [std::mem::offset_of!(IMV_UsbDeviceInfo, u3vSerialNumber) - 1304usize];
    ["Offset of field: IMV_UsbDeviceInfo::speed"]
        [std::mem::offset_of!(IMV_UsbDeviceInfo, speed) - 1560usize];
    ["Offset of field: IMV_UsbDeviceInfo::maxPower"]
        [std::mem::offset_of!(IMV_UsbDeviceInfo, maxPower) - 1816usize];
    ["Offset of field: IMV_UsbDeviceInfo::chReserved"]
        [std::mem::offset_of!(IMV_UsbDeviceInfo, chReserved) - 2072usize];
};

/// brief 设备通用信息
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IMV_DeviceInfo {
    /// 设备类别
    pub nCameraType: IMV_ECameraType::IMV_ECameraType,
    /// 保留
    pub nCameraReserved: [c_int; 5usize],
    /// 厂商:序列号
    pub cameraKey: [c_char; 256usize],
    /// 用户自定义名
    pub cameraName: [c_char; 256usize],
    /// 设备序列号
    pub serialNumber: [c_char; 256usize],
    /// 厂商
    pub vendorName: [c_char; 256usize],
    /// 设备型号
    pub modelName: [c_char; 256usize],
    /// 设备制造信息
    pub manufactureInfo: [c_char; 256usize],
    /// 设备版本
    pub deviceVersion: [c_char; 256usize],
    /// 保留
    pub cameraReserved: [[c_char; 256usize]; 5usize],
    pub DeviceSpecificInfo: _DeviceSpecificInfo,
    /// 接口类别
    pub nInterfaceType: IMV_EInterfaceType::IMV_EInterfaceType,
    /// 保留
    pub nInterfaceReserved: [c_int; 5usize],
    /// 接口名
    pub interfaceName: [c_char; 256usize],
    /// 保留
    pub interfaceReserved: [[c_char; 256usize]; 5usize],
    pub InterfaceInfo: _InterfaceInfo,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _DeviceSpecificInfo {
    /// Gige设备信息
    pub gigeDeviceInfo: IMV_GigEDeviceInfo,
    /// Usb设备信息
    pub usbDeviceInfo: IMV_UsbDeviceInfo,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _DeviceSpecificInfo"][std::mem::size_of::<_DeviceSpecificInfo>() - 3096usize];
    ["Alignment of _DeviceSpecificInfo"][std::mem::align_of::<_DeviceSpecificInfo>() - 4usize];
    ["Offset of field: _DeviceSpecificInfo::gigeDeviceInfo"]
        [std::mem::offset_of!(_DeviceSpecificInfo, gigeDeviceInfo) - 0usize];
    ["Offset of field: _DeviceSpecificInfo::usbDeviceInfo"]
        [std::mem::offset_of!(_DeviceSpecificInfo, usbDeviceInfo) - 0usize];
};

#[repr(C)]
#[derive(Copy, Clone)]
pub union _InterfaceInfo {
    /// GigE网卡信息
    pub gigeInterfaceInfo: IMV_GigEInterfaceInfo,
    /// Usb接口信息
    pub usbInterfaceInfo: IMV_UsbInterfaceInfo,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _InterfaceInfo"][std::mem::size_of::<_InterfaceInfo>() - 2560usize];
    ["Alignment of _InterfaceInfo"][std::mem::align_of::<_InterfaceInfo>() - 1usize];
    ["Offset of field: _InterfaceInfo::gigeInterfaceInfo"]
        [std::mem::offset_of!(_InterfaceInfo, gigeInterfaceInfo) - 0usize];
    ["Offset of field: _InterfaceInfo::usbInterfaceInfo"]
        [std::mem::offset_of!(_InterfaceInfo, usbInterfaceInfo) - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMV_DeviceInfo"][std::mem::size_of::<IMV_DeviceInfo>() - 10312usize];
    ["Alignment of IMV_DeviceInfo"][std::mem::align_of::<IMV_DeviceInfo>() - 4usize];
    ["Offset of field: IMV_DeviceInfo::nCameraType"]
        [std::mem::offset_of!(IMV_DeviceInfo, nCameraType) - 0usize];
    ["Offset of field: IMV_DeviceInfo::nCameraReserved"]
        [std::mem::offset_of!(IMV_DeviceInfo, nCameraReserved) - 4usize];
    ["Offset of field: IMV_DeviceInfo::cameraKey"]
        [std::mem::offset_of!(IMV_DeviceInfo, cameraKey) - 24usize];
    ["Offset of field: IMV_DeviceInfo::cameraName"]
        [std::mem::offset_of!(IMV_DeviceInfo, cameraName) - 280usize];
    ["Offset of field: IMV_DeviceInfo::serialNumber"]
        [std::mem::offset_of!(IMV_DeviceInfo, serialNumber) - 536usize];
    ["Offset of field: IMV_DeviceInfo::vendorName"]
        [std::mem::offset_of!(IMV_DeviceInfo, vendorName) - 792usize];
    ["Offset of field: IMV_DeviceInfo::modelName"]
        [std::mem::offset_of!(IMV_DeviceInfo, modelName) - 1048usize];
    ["Offset of field: IMV_DeviceInfo::manufactureInfo"]
        [std::mem::offset_of!(IMV_DeviceInfo, manufactureInfo) - 1304usize];
    ["Offset of field: IMV_DeviceInfo::deviceVersion"]
        [std::mem::offset_of!(IMV_DeviceInfo, deviceVersion) - 1560usize];
    ["Offset of field: IMV_DeviceInfo::cameraReserved"]
        [std::mem::offset_of!(IMV_DeviceInfo, cameraReserved) - 1816usize];
    ["Offset of field: IMV_DeviceInfo::DeviceSpecificInfo"]
        [std::mem::offset_of!(IMV_DeviceInfo, DeviceSpecificInfo) - 3096usize];
    ["Offset of field: IMV_DeviceInfo::nInterfaceType"]
        [std::mem::offset_of!(IMV_DeviceInfo, nInterfaceType) - 6192usize];
    ["Offset of field: IMV_DeviceInfo::nInterfaceReserved"]
        [std::mem::offset_of!(IMV_DeviceInfo, nInterfaceReserved) - 6196usize];
    ["Offset of field: IMV_DeviceInfo::interfaceName"]
        [std::mem::offset_of!(IMV_DeviceInfo, interfaceName) - 6216usize];
    ["Offset of field: IMV_DeviceInfo::interfaceReserved"]
        [std::mem::offset_of!(IMV_DeviceInfo, interfaceReserved) - 6472usize];
    ["Offset of field: IMV_DeviceInfo::InterfaceInfo"]
        [std::mem::offset_of!(IMV_DeviceInfo, InterfaceInfo) - 7752usize];
};

/// brief 网络传输模式
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMV_TRANSMISSION_TYPE {
    /// 传输模式
    pub eTransmissionType: IMV_TransmissionType::IMV_TransmissionType,
    /// 目标ip地址
    pub nDesIp: c_uint,
    /// 目标端口号
    pub nDesPort: c_ushort,
    /// 预留位
    pub nReserved: [c_uint; 32usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMV_TRANSMISSION_TYPE"][std::mem::size_of::<IMV_TRANSMISSION_TYPE>() - 140usize];
    ["Alignment of IMV_TRANSMISSION_TYPE"][std::mem::align_of::<IMV_TRANSMISSION_TYPE>() - 4usize];
    ["Offset of field: IMV_TRANSMISSION_TYPE::eTransmissionType"]
        [std::mem::offset_of!(IMV_TRANSMISSION_TYPE, eTransmissionType) - 0usize];
    ["Offset of field: IMV_TRANSMISSION_TYPE::nDesIp"]
        [std::mem::offset_of!(IMV_TRANSMISSION_TYPE, nDesIp) - 4usize];
    ["Offset of field: IMV_TRANSMISSION_TYPE::nDesPort"]
        [std::mem::offset_of!(IMV_TRANSMISSION_TYPE, nDesPort) - 8usize];
    ["Offset of field: IMV_TRANSMISSION_TYPE::nReserved"]
        [std::mem::offset_of!(IMV_TRANSMISSION_TYPE, nReserved) - 12usize];
};

/// brief 加载失败的属性信息
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMV_ErrorList {
    /// 加载失败的属性个数
    pub nParamCnt: c_uint,
    /// 加载失败的属性集合，上限128
    pub paramNameList: [IMV_String; 128usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMV_ErrorList"][std::mem::size_of::<IMV_ErrorList>() - 32772usize];
    ["Alignment of IMV_ErrorList"][std::mem::align_of::<IMV_ErrorList>() - 4usize];
    ["Offset of field: IMV_ErrorList::nParamCnt"]
        [std::mem::offset_of!(IMV_ErrorList, nParamCnt) - 0usize];
    ["Offset of field: IMV_ErrorList::paramNameList"]
        [std::mem::offset_of!(IMV_ErrorList, paramNameList) - 4usize];
};

/// brief 设备信息列表
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMV_DeviceList {
    /// 设备数量
    pub nDevNum: c_uint,
    /// 设备息列表(SDK内部缓存)，最多100设备
    pub pDevInfo: *mut IMV_DeviceInfo,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMV_DeviceList"][std::mem::size_of::<IMV_DeviceList>() - 16usize];
    ["Alignment of IMV_DeviceList"][std::mem::align_of::<IMV_DeviceList>() - 8usize];
    ["Offset of field: IMV_DeviceList::nDevNum"]
        [std::mem::offset_of!(IMV_DeviceList, nDevNum) - 0usize];
    ["Offset of field: IMV_DeviceList::pDevInfo"]
        [std::mem::offset_of!(IMV_DeviceList, pDevInfo) - 8usize];
};

/// brief 连接事件信息
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMV_SConnectArg {
    /// 事件类型
    pub event: IMV_EVType::IMV_EVType,
    /// 预留字段
    pub nReserve: [c_uint; 10usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMV_SConnectArg"][std::mem::size_of::<IMV_SConnectArg>() - 44usize];
    ["Alignment of IMV_SConnectArg"][std::mem::align_of::<IMV_SConnectArg>() - 4usize];
    ["Offset of field: IMV_SConnectArg::event"]
        [std::mem::offset_of!(IMV_SConnectArg, event) - 0usize];
    ["Offset of field: IMV_SConnectArg::nReserve"]
        [std::mem::offset_of!(IMV_SConnectArg, nReserve) - 4usize];
};

/// brief 参数更新事件信息
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMV_SParamUpdateArg {
    /// 是否是定时更新,true:表示是定时更新，false:表示非定时更新
    pub isPoll: bool_,
    /// 预留字段
    pub nReserve: [c_uint; 10usize],
    /// 更新的参数个数
    pub nParamCnt: c_uint,
    /// 更新的参数名称集合(SDK内部缓存)
    pub pParamNameList: *mut IMV_String,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMV_SParamUpdateArg"][std::mem::size_of::<IMV_SParamUpdateArg>() - 56usize];
    ["Alignment of IMV_SParamUpdateArg"][std::mem::align_of::<IMV_SParamUpdateArg>() - 8usize];
    ["Offset of field: IMV_SParamUpdateArg::isPoll"]
        [std::mem::offset_of!(IMV_SParamUpdateArg, isPoll) - 0usize];
    ["Offset of field: IMV_SParamUpdateArg::nReserve"]
        [std::mem::offset_of!(IMV_SParamUpdateArg, nReserve) - 4usize];
    ["Offset of field: IMV_SParamUpdateArg::nParamCnt"]
        [std::mem::offset_of!(IMV_SParamUpdateArg, nParamCnt) - 44usize];
    ["Offset of field: IMV_SParamUpdateArg::pParamNameList"]
        [std::mem::offset_of!(IMV_SParamUpdateArg, pParamNameList) - 48usize];
};

/// brief 流事件信息
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMV_SStreamArg {
    /// 流通道号
    pub channel: c_uint,
    /// 流数据BlockID
    pub blockId: u64,
    /// 时间戳
    pub timeStamp: u64,
    /// 流事件状态码
    pub eStreamEventStatus: IMV_EEventStatus::IMV_EEventStatus,
    /// 事件状态错误码
    pub status: c_uint,
    /// 预留字段
    pub nReserve: [c_uint; 9usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMV_SStreamArg"][std::mem::size_of::<IMV_SStreamArg>() - 72usize];
    ["Alignment of IMV_SStreamArg"][std::mem::align_of::<IMV_SStreamArg>() - 8usize];
    ["Offset of field: IMV_SStreamArg::channel"]
        [std::mem::offset_of!(IMV_SStreamArg, channel) - 0usize];
    ["Offset of field: IMV_SStreamArg::blockId"]
        [std::mem::offset_of!(IMV_SStreamArg, blockId) - 8usize];
    ["Offset of field: IMV_SStreamArg::timeStamp"]
        [std::mem::offset_of!(IMV_SStreamArg, timeStamp) - 16usize];
    ["Offset of field: IMV_SStreamArg::eStreamEventStatus"]
        [std::mem::offset_of!(IMV_SStreamArg, eStreamEventStatus) - 24usize];
    ["Offset of field: IMV_SStreamArg::status"]
        [std::mem::offset_of!(IMV_SStreamArg, status) - 28usize];
    ["Offset of field: IMV_SStreamArg::nReserve"]
        [std::mem::offset_of!(IMV_SStreamArg, nReserve) - 32usize];
};

/// brief 消息通道事件信息
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMV_SMsgChannelArg {
    /// 事件Id
    pub eventId: c_ushort,
    /// 消息通道号
    pub channelId: c_ushort,
    /// 流数据BlockID
    pub blockId: u64,
    /// 时间戳
    pub timeStamp: u64,
    /// 预留字段
    pub nReserve: [c_uint; 8usize],
    /// 参数个数
    pub nParamCnt: c_uint,
    /// 事件相关的属性名列集合(SDK内部缓存)
    pub pParamNameList: *mut IMV_String,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMV_SMsgChannelArg"][std::mem::size_of::<IMV_SMsgChannelArg>() - 72usize];
    ["Alignment of IMV_SMsgChannelArg"][std::mem::align_of::<IMV_SMsgChannelArg>() - 8usize];
    ["Offset of field: IMV_SMsgChannelArg::eventId"]
        [std::mem::offset_of!(IMV_SMsgChannelArg, eventId) - 0usize];
    ["Offset of field: IMV_SMsgChannelArg::channelId"]
        [std::mem::offset_of!(IMV_SMsgChannelArg, channelId) - 2usize];
    ["Offset of field: IMV_SMsgChannelArg::blockId"]
        [std::mem::offset_of!(IMV_SMsgChannelArg, blockId) - 8usize];
    ["Offset of field: IMV_SMsgChannelArg::timeStamp"]
        [std::mem::offset_of!(IMV_SMsgChannelArg, timeStamp) - 16usize];
    ["Offset of field: IMV_SMsgChannelArg::nReserve"]
        [std::mem::offset_of!(IMV_SMsgChannelArg, nReserve) - 24usize];
    ["Offset of field: IMV_SMsgChannelArg::nParamCnt"]
        [std::mem::offset_of!(IMV_SMsgChannelArg, nParamCnt) - 56usize];
    ["Offset of field: IMV_SMsgChannelArg::pParamNameList"]
        [std::mem::offset_of!(IMV_SMsgChannelArg, pParamNameList) - 64usize];
};

/// brief 消息通道事件信息（通用）
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMV_SMsgEventArg {
    /// 事件Id
    pub eventId: c_ushort,
    /// 消息通道号
    pub channelId: c_ushort,
    /// 流数据BlockID
    pub blockId: u64,
    /// 时间戳
    pub timeStamp: u64,
    /// 事件数据，内部缓存，需要及时进行数据处理
    pub pEventData: *mut c_void,
    /// 事件数据长度
    pub nEventDataSize: c_uint,
    /// 预留字段
    pub reserve: [c_uint; 8usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMV_SMsgEventArg"][std::mem::size_of::<IMV_SMsgEventArg>() - 72usize];
    ["Alignment of IMV_SMsgEventArg"][std::mem::align_of::<IMV_SMsgEventArg>() - 8usize];
    ["Offset of field: IMV_SMsgEventArg::eventId"]
        [std::mem::offset_of!(IMV_SMsgEventArg, eventId) - 0usize];
    ["Offset of field: IMV_SMsgEventArg::channelId"]
        [std::mem::offset_of!(IMV_SMsgEventArg, channelId) - 2usize];
    ["Offset of field: IMV_SMsgEventArg::blockId"]
        [std::mem::offset_of!(IMV_SMsgEventArg, blockId) - 8usize];
    ["Offset of field: IMV_SMsgEventArg::timeStamp"]
        [std::mem::offset_of!(IMV_SMsgEventArg, timeStamp) - 16usize];
    ["Offset of field: IMV_SMsgEventArg::pEventData"]
        [std::mem::offset_of!(IMV_SMsgEventArg, pEventData) - 24usize];
    ["Offset of field: IMV_SMsgEventArg::nEventDataSize"]
        [std::mem::offset_of!(IMV_SMsgEventArg, nEventDataSize) - 32usize];
    ["Offset of field: IMV_SMsgEventArg::reserve"]
        [std::mem::offset_of!(IMV_SMsgEventArg, reserve) - 36usize];
};

/// brief Chunk数据信息
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMV_ChunkDataInfo {
    /// ChunkID
    pub chunkID: c_uint,
    /// 属性名个数
    pub nParamCnt: c_uint,
    /// Chunk数据对应的属性名集合(SDK内部缓存)
    pub pParamNameList: *mut IMV_String,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMV_ChunkDataInfo"][std::mem::size_of::<IMV_ChunkDataInfo>() - 16usize];
    ["Alignment of IMV_ChunkDataInfo"][std::mem::align_of::<IMV_ChunkDataInfo>() - 8usize];
    ["Offset of field: IMV_ChunkDataInfo::chunkID"]
        [std::mem::offset_of!(IMV_ChunkDataInfo, chunkID) - 0usize];
    ["Offset of field: IMV_ChunkDataInfo::nParamCnt"]
        [std::mem::offset_of!(IMV_ChunkDataInfo, nParamCnt) - 4usize];
    ["Offset of field: IMV_ChunkDataInfo::pParamNameList"]
        [std::mem::offset_of!(IMV_ChunkDataInfo, pParamNameList) - 8usize];
};

/// brief 帧图像信息
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMV_FrameInfo {
    /// 帧Id(仅对GigE/Usb/PCIe相机有效)
    pub blockId: u64,
    /// 数据帧状态(0是正常状态)
    pub status: c_uint,
    /// 图像宽度
    pub width: c_uint,
    /// 图像高度
    pub height: c_uint,
    /// 图像大小
    pub size: c_uint,
    /// 图像像素格式
    pub pixelFormat: IMV_EPixelType::IMV_EPixelType,
    /// 图像时间戳(仅对GigE/Usb/PCIe相机有效)
    pub timeStamp: u64,
    /// 帧数据中包含的Chunk个数(仅对GigE/Usb相机有效)
    pub chunkCount: c_uint,
    /// 图像paddingX(仅对GigE/Usb/PCIe相机有效)
    pub paddingX: c_uint,
    /// 图像paddingY(仅对GigE/Usb/PCIe相机有效)
    pub paddingY: c_uint,
    /// 图像在网络传输所用的时间(单位:微秒,非GigE相机该值为0)
    pub recvFrameTime: c_uint,
    /// 预留字段
    pub nReserved: [c_uint; 19usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMV_FrameInfo"][std::mem::size_of::<IMV_FrameInfo>() - 136usize];
    ["Alignment of IMV_FrameInfo"][std::mem::align_of::<IMV_FrameInfo>() - 8usize];
    ["Offset of field: IMV_FrameInfo::blockId"]
        [std::mem::offset_of!(IMV_FrameInfo, blockId) - 0usize];
    ["Offset of field: IMV_FrameInfo::status"]
        [std::mem::offset_of!(IMV_FrameInfo, status) - 8usize];
    ["Offset of field: IMV_FrameInfo::width"][std::mem::offset_of!(IMV_FrameInfo, width) - 12usize];
    ["Offset of field: IMV_FrameInfo::height"]
        [std::mem::offset_of!(IMV_FrameInfo, height) - 16usize];
    ["Offset of field: IMV_FrameInfo::size"][std::mem::offset_of!(IMV_FrameInfo, size) - 20usize];
    ["Offset of field: IMV_FrameInfo::pixelFormat"]
        [std::mem::offset_of!(IMV_FrameInfo, pixelFormat) - 24usize];
    ["Offset of field: IMV_FrameInfo::timeStamp"]
        [std::mem::offset_of!(IMV_FrameInfo, timeStamp) - 32usize];
    ["Offset of field: IMV_FrameInfo::chunkCount"]
        [std::mem::offset_of!(IMV_FrameInfo, chunkCount) - 40usize];
    ["Offset of field: IMV_FrameInfo::paddingX"]
        [std::mem::offset_of!(IMV_FrameInfo, paddingX) - 44usize];
    ["Offset of field: IMV_FrameInfo::paddingY"]
        [std::mem::offset_of!(IMV_FrameInfo, paddingY) - 48usize];
    ["Offset of field: IMV_FrameInfo::recvFrameTime"]
        [std::mem::offset_of!(IMV_FrameInfo, recvFrameTime) - 52usize];
    ["Offset of field: IMV_FrameInfo::nReserved"]
        [std::mem::offset_of!(IMV_FrameInfo, nReserved) - 56usize];
};

/// brief 帧图像数据信息
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMV_Frame {
    /// 帧图像句柄(SDK内部帧管理用)
    pub frameHandle: IMV_FRAME_HANDLE,
    /// 帧图像数据的内存首地址
    pub pData: *mut c_uchar,
    /// 帧信息
    pub frameInfo: IMV_FrameInfo,
    /// 预留字段
    pub nReserved: [c_uint; 10usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMV_Frame"][std::mem::size_of::<IMV_Frame>() - 192usize];
    ["Alignment of IMV_Frame"][std::mem::align_of::<IMV_Frame>() - 8usize];
    ["Offset of field: IMV_Frame::frameHandle"]
        [std::mem::offset_of!(IMV_Frame, frameHandle) - 0usize];
    ["Offset of field: IMV_Frame::pData"][std::mem::offset_of!(IMV_Frame, pData) - 8usize];
    ["Offset of field: IMV_Frame::frameInfo"][std::mem::offset_of!(IMV_Frame, frameInfo) - 16usize];
    ["Offset of field: IMV_Frame::nReserved"]
        [std::mem::offset_of!(IMV_Frame, nReserved) - 152usize];
};

/// brief PCIE设备统计流信息
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMV_PCIEStreamStatsInfo {
    /// 图像错误的帧数
    pub imageError: c_uint,
    /// 丢包的帧数
    pub lostPacketBlock: c_uint,
    /// 预留
    pub nReserved0: [c_uint; 10usize],
    /// 正常获取的帧数
    pub imageReceived: c_uint,
    /// 帧率
    pub fps: f64,
    /// 带宽(Mbps)
    pub bandwidth: f64,
    /// 预留
    pub nReserved: [c_uint; 8usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMV_PCIEStreamStatsInfo"][std::mem::size_of::<IMV_PCIEStreamStatsInfo>() - 104usize];
    ["Alignment of IMV_PCIEStreamStatsInfo"]
        [std::mem::align_of::<IMV_PCIEStreamStatsInfo>() - 8usize];
    ["Offset of field: IMV_PCIEStreamStatsInfo::imageError"]
        [std::mem::offset_of!(IMV_PCIEStreamStatsInfo, imageError) - 0usize];
    ["Offset of field: IMV_PCIEStreamStatsInfo::lostPacketBlock"]
        [std::mem::offset_of!(IMV_PCIEStreamStatsInfo, lostPacketBlock) - 4usize];
    ["Offset of field: IMV_PCIEStreamStatsInfo::nReserved0"]
        [std::mem::offset_of!(IMV_PCIEStreamStatsInfo, nReserved0) - 8usize];
    ["Offset of field: IMV_PCIEStreamStatsInfo::imageReceived"]
        [std::mem::offset_of!(IMV_PCIEStreamStatsInfo, imageReceived) - 48usize];
    ["Offset of field: IMV_PCIEStreamStatsInfo::fps"]
        [std::mem::offset_of!(IMV_PCIEStreamStatsInfo, fps) - 56usize];
    ["Offset of field: IMV_PCIEStreamStatsInfo::bandwidth"]
        [std::mem::offset_of!(IMV_PCIEStreamStatsInfo, bandwidth) - 64usize];
    ["Offset of field: IMV_PCIEStreamStatsInfo::nReserved"]
        [std::mem::offset_of!(IMV_PCIEStreamStatsInfo, nReserved) - 72usize];
};

// brief U3V设备统计流信息
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMV_U3VStreamStatsInfo {
    /// 图像错误的帧数
    pub imageError: c_uint,
    /// 丢包的帧数
    pub lostPacketBlock: c_uint,
    /// 预留
    pub nReserved0: [c_uint; 10usize],
    /// 正常获取的帧数
    pub imageReceived: c_uint,
    /// 帧率
    pub fps: f64,
    /// 带宽(Mbps)
    pub bandwidth: f64,
    /// 预留
    pub nReserved: [c_uint; 8usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMV_U3VStreamStatsInfo"][std::mem::size_of::<IMV_U3VStreamStatsInfo>() - 104usize];
    ["Alignment of IMV_U3VStreamStatsInfo"]
        [std::mem::align_of::<IMV_U3VStreamStatsInfo>() - 8usize];
    ["Offset of field: IMV_U3VStreamStatsInfo::imageError"]
        [std::mem::offset_of!(IMV_U3VStreamStatsInfo, imageError) - 0usize];
    ["Offset of field: IMV_U3VStreamStatsInfo::lostPacketBlock"]
        [std::mem::offset_of!(IMV_U3VStreamStatsInfo, lostPacketBlock) - 4usize];
    ["Offset of field: IMV_U3VStreamStatsInfo::nReserved0"]
        [std::mem::offset_of!(IMV_U3VStreamStatsInfo, nReserved0) - 8usize];
    ["Offset of field: IMV_U3VStreamStatsInfo::imageReceived"]
        [std::mem::offset_of!(IMV_U3VStreamStatsInfo, imageReceived) - 48usize];
    ["Offset of field: IMV_U3VStreamStatsInfo::fps"]
        [std::mem::offset_of!(IMV_U3VStreamStatsInfo, fps) - 56usize];
    ["Offset of field: IMV_U3VStreamStatsInfo::bandwidth"]
        [std::mem::offset_of!(IMV_U3VStreamStatsInfo, bandwidth) - 64usize];
    ["Offset of field: IMV_U3VStreamStatsInfo::nReserved"]
        [std::mem::offset_of!(IMV_U3VStreamStatsInfo, nReserved) - 72usize];
};

/// brief Gige设备统计流信息
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMV_GigEStreamStatsInfo {
    /// 预留
    pub nReserved0: [c_uint; 10usize],
    /// 图像错误的帧数
    pub imageError: c_uint,
    /// 丢包的帧数
    pub lostPacketBlock: c_uint,
    /// 预留
    pub nReserved1: [c_uint; 4usize],
    /// 预留
    pub nReserved2: [c_uint; 5usize],
    /// 正常获取的帧数
    pub imageReceived: c_uint,
    /// 帧率
    pub fps: f64,
    /// 带宽(Mbps)
    pub bandwidth: f64,
    /// 预留
    pub nReserved: [c_uint; 4usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMV_GigEStreamStatsInfo"][std::mem::size_of::<IMV_GigEStreamStatsInfo>() - 120usize];
    ["Alignment of IMV_GigEStreamStatsInfo"]
        [std::mem::align_of::<IMV_GigEStreamStatsInfo>() - 8usize];
    ["Offset of field: IMV_GigEStreamStatsInfo::nReserved0"]
        [std::mem::offset_of!(IMV_GigEStreamStatsInfo, nReserved0) - 0usize];
    ["Offset of field: IMV_GigEStreamStatsInfo::imageError"]
        [std::mem::offset_of!(IMV_GigEStreamStatsInfo, imageError) - 40usize];
    ["Offset of field: IMV_GigEStreamStatsInfo::lostPacketBlock"]
        [std::mem::offset_of!(IMV_GigEStreamStatsInfo, lostPacketBlock) - 44usize];
    ["Offset of field: IMV_GigEStreamStatsInfo::nReserved1"]
        [std::mem::offset_of!(IMV_GigEStreamStatsInfo, nReserved1) - 48usize];
    ["Offset of field: IMV_GigEStreamStatsInfo::nReserved2"]
        [std::mem::offset_of!(IMV_GigEStreamStatsInfo, nReserved2) - 64usize];
    ["Offset of field: IMV_GigEStreamStatsInfo::imageReceived"]
        [std::mem::offset_of!(IMV_GigEStreamStatsInfo, imageReceived) - 84usize];
    ["Offset of field: IMV_GigEStreamStatsInfo::fps"]
        [std::mem::offset_of!(IMV_GigEStreamStatsInfo, fps) - 88usize];
    ["Offset of field: IMV_GigEStreamStatsInfo::bandwidth"]
        [std::mem::offset_of!(IMV_GigEStreamStatsInfo, bandwidth) - 96usize];
    ["Offset of field: IMV_GigEStreamStatsInfo::nReserved"]
        [std::mem::offset_of!(IMV_GigEStreamStatsInfo, nReserved) - 104usize];
};

/// brief 统计流信息
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IMV_StreamStatisticsInfo {
    /// 设备类型
    pub nCameraType: IMV_ECameraType::IMV_ECameraType,
    pub IMV_StreamStatsInfo_union: IMV_StreamStatsInfo_Union,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union IMV_StreamStatsInfo_Union {
    /// PCIE设备统计信息
    pub pcieStatisticsInfo: IMV_PCIEStreamStatsInfo,
    /// U3V设备统计信息
    pub u3vStatisticsInfo: IMV_U3VStreamStatsInfo,
    /// Gige设备统计信息
    pub gigeStatisticsInfo: IMV_GigEStreamStatsInfo,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMV_StreamStatsInfo_Union"]
        [std::mem::size_of::<IMV_StreamStatsInfo_Union>() - 120usize];
    ["Alignment of IMV_StreamStatsInfo_Union"]
        [std::mem::align_of::<IMV_StreamStatsInfo_Union>() - 8usize];
    ["Offset of field: IMV_StreamStatsInfo_Union::pcieStatisticsInfo"]
        [std::mem::offset_of!(IMV_StreamStatsInfo_Union, pcieStatisticsInfo) - 0usize];
    ["Offset of field: IMV_StreamStatsInfo_Union::u3vStatisticsInfo"]
        [std::mem::offset_of!(IMV_StreamStatsInfo_Union, u3vStatisticsInfo) - 0usize];
    ["Offset of field: IMV_StreamStatsInfo_Union::gigeStatisticsInfo"]
        [std::mem::offset_of!(IMV_StreamStatsInfo_Union, gigeStatisticsInfo) - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMV_StreamStatisticsInfo"]
        [std::mem::size_of::<IMV_StreamStatisticsInfo>() - 128usize];
    ["Alignment of IMV_StreamStatisticsInfo"]
        [std::mem::align_of::<IMV_StreamStatisticsInfo>() - 8usize];
    ["Offset of field: IMV_StreamStatisticsInfo::nCameraType"]
        [std::mem::offset_of!(IMV_StreamStatisticsInfo, nCameraType) - 0usize];
};

/// brief 枚举属性的枚举值信息
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMV_EnumEntryInfo {
    /// 枚举值
    pub value: u64,
    /// symbol名
    pub name: [c_char; 256usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMV_EnumEntryInfo"][std::mem::size_of::<IMV_EnumEntryInfo>() - 264usize];
    ["Alignment of IMV_EnumEntryInfo"][std::mem::align_of::<IMV_EnumEntryInfo>() - 8usize];
    ["Offset of field: IMV_EnumEntryInfo::value"]
        [std::mem::offset_of!(IMV_EnumEntryInfo, value) - 0usize];
    ["Offset of field: IMV_EnumEntryInfo::name"]
        [std::mem::offset_of!(IMV_EnumEntryInfo, name) - 8usize];
};

/// brief 枚举属性的可设枚举值列表信息
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMV_EnumEntryList {
    /// 存放枚举值内存大小
    pub nEnumEntryBufferSize: c_uint,
    /// 存放可设枚举值列表(调用者分配缓存)
    pub pEnumEntryInfo: *mut IMV_EnumEntryInfo,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMV_EnumEntryList"][std::mem::size_of::<IMV_EnumEntryList>() - 16usize];
    ["Alignment of IMV_EnumEntryList"][std::mem::align_of::<IMV_EnumEntryList>() - 8usize];
    ["Offset of field: IMV_EnumEntryList::nEnumEntryBufferSize"]
        [std::mem::offset_of!(IMV_EnumEntryList, nEnumEntryBufferSize) - 0usize];
    ["Offset of field: IMV_EnumEntryList::pEnumEntryInfo"]
        [std::mem::offset_of!(IMV_EnumEntryList, pEnumEntryInfo) - 8usize];
};

/// brief 像素转换结构体
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMV_PixelConvertParam {
    /// [IN] 图像宽
    pub nWidth: c_uint,
    /// [IN] 图像高
    pub nHeight: c_uint,
    /// [IN] 像素格式
    pub ePixelFormat: IMV_EPixelType::IMV_EPixelType,
    /// [IN] 输入图像数据
    pub pSrcData: *mut c_uchar,
    /// [IN] 输入图像长度
    pub nSrcDataLen: c_uint,
    /// [IN] 图像宽填充
    pub nPaddingX: c_uint,
    /// [IN] 图像高填充
    pub nPaddingY: c_uint,
    /// [IN] 转换Bayer格式算法
    pub eBayerDemosaic: IMV_EBayerDemosaic::IMV_EBayerDemosaic,
    /// [IN] 目标像素格式
    pub eDstPixelFormat: IMV_EPixelType::IMV_EPixelType,
    /// [OUT] 输出数据缓存(调用者分配缓存)
    pub pDstBuf: *mut c_uchar,
    /// [IN] 提供的输出缓冲区大小
    pub nDstBufSize: c_uint,
    /// [OUT] 输出数据长度
    pub nDstDataLen: c_uint,
    /// 预留
    pub nReserved: [c_uint; 8usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMV_PixelConvertParam"][std::mem::size_of::<IMV_PixelConvertParam>() - 96usize];
    ["Alignment of IMV_PixelConvertParam"][std::mem::align_of::<IMV_PixelConvertParam>() - 8usize];
    ["Offset of field: IMV_PixelConvertParam::nWidth"]
        [std::mem::offset_of!(IMV_PixelConvertParam, nWidth) - 0usize];
    ["Offset of field: IMV_PixelConvertParam::nHeight"]
        [std::mem::offset_of!(IMV_PixelConvertParam, nHeight) - 4usize];
    ["Offset of field: IMV_PixelConvertParam::ePixelFormat"]
        [std::mem::offset_of!(IMV_PixelConvertParam, ePixelFormat) - 8usize];
    ["Offset of field: IMV_PixelConvertParam::pSrcData"]
        [std::mem::offset_of!(IMV_PixelConvertParam, pSrcData) - 16usize];
    ["Offset of field: IMV_PixelConvertParam::nSrcDataLen"]
        [std::mem::offset_of!(IMV_PixelConvertParam, nSrcDataLen) - 24usize];
    ["Offset of field: IMV_PixelConvertParam::nPaddingX"]
        [std::mem::offset_of!(IMV_PixelConvertParam, nPaddingX) - 28usize];
    ["Offset of field: IMV_PixelConvertParam::nPaddingY"]
        [std::mem::offset_of!(IMV_PixelConvertParam, nPaddingY) - 32usize];
    ["Offset of field: IMV_PixelConvertParam::eBayerDemosaic"]
        [std::mem::offset_of!(IMV_PixelConvertParam, eBayerDemosaic) - 36usize];
    ["Offset of field: IMV_PixelConvertParam::eDstPixelFormat"]
        [std::mem::offset_of!(IMV_PixelConvertParam, eDstPixelFormat) - 40usize];
    ["Offset of field: IMV_PixelConvertParam::pDstBuf"]
        [std::mem::offset_of!(IMV_PixelConvertParam, pDstBuf) - 48usize];
    ["Offset of field: IMV_PixelConvertParam::nDstBufSize"]
        [std::mem::offset_of!(IMV_PixelConvertParam, nDstBufSize) - 56usize];
    ["Offset of field: IMV_PixelConvertParam::nDstDataLen"]
        [std::mem::offset_of!(IMV_PixelConvertParam, nDstDataLen) - 60usize];
    ["Offset of field: IMV_PixelConvertParam::nReserved"]
        [std::mem::offset_of!(IMV_PixelConvertParam, nReserved) - 64usize];
};

/// brief 录像结构体
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMV_RecordParam {
    /// [IN] 图像宽
    pub nWidth: c_uint,
    /// [IN] 图像高
    pub nHeight: c_uint,
    /// [IN] 帧率(大于0)
    pub fFameRate: f32,
    /// [IN] 视频质量(1-100)
    pub nQuality: c_uint,
    /// [IN] 视频格式
    pub recordFormat: IMV_EVideoType::IMV_EVideoType,
    /// [IN] 保存视频路径
    pub pRecordFilePath: *const c_char,
    /// [IN] 码率kbps(128-16*1024)
    pub nBitrate: c_uint,
    /// [IN] 像素格式
    pub ePixelFormat: IMV_EPixelType::IMV_EPixelType,
    /// 预留
    pub nReserved: [c_uint; 3usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMV_RecordParam"][std::mem::size_of::<IMV_RecordParam>() - 56usize];
    ["Alignment of IMV_RecordParam"][std::mem::align_of::<IMV_RecordParam>() - 8usize];
    ["Offset of field: IMV_RecordParam::nWidth"]
        [std::mem::offset_of!(IMV_RecordParam, nWidth) - 0usize];
    ["Offset of field: IMV_RecordParam::nHeight"]
        [std::mem::offset_of!(IMV_RecordParam, nHeight) - 4usize];
    ["Offset of field: IMV_RecordParam::fFameRate"]
        [std::mem::offset_of!(IMV_RecordParam, fFameRate) - 8usize];
    ["Offset of field: IMV_RecordParam::nQuality"]
        [std::mem::offset_of!(IMV_RecordParam, nQuality) - 12usize];
    ["Offset of field: IMV_RecordParam::recordFormat"]
        [std::mem::offset_of!(IMV_RecordParam, recordFormat) - 16usize];
    ["Offset of field: IMV_RecordParam::pRecordFilePath"]
        [std::mem::offset_of!(IMV_RecordParam, pRecordFilePath) - 24usize];
    ["Offset of field: IMV_RecordParam::nBitrate"]
        [std::mem::offset_of!(IMV_RecordParam, nBitrate) - 32usize];
    ["Offset of field: IMV_RecordParam::ePixelFormat"]
        [std::mem::offset_of!(IMV_RecordParam, ePixelFormat) - 36usize];
    ["Offset of field: IMV_RecordParam::nReserved"]
        [std::mem::offset_of!(IMV_RecordParam, nReserved) - 40usize];
};

/// brief 录像用帧信息结构体
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMV_RecordFrameInfoParam {
    /// [IN] 图像数据
    pub pData: *mut c_uchar,
    /// [IN] 图像数据长度
    pub nDataLen: c_uint,
    /// [IN] 图像宽填充(可忽略)
    pub nPaddingX: c_uint,
    /// [IN] 图像高填充(可忽略)
    pub nPaddingY: c_uint,
    /// [IN] 像素格式(可忽略)
    pub ePixelFormat: IMV_EPixelType::IMV_EPixelType,
    /// 预留
    pub nReserved: [c_uint; 5usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMV_RecordFrameInfoParam"][std::mem::size_of::<IMV_RecordFrameInfoParam>() - 48usize];
    ["Alignment of IMV_RecordFrameInfoParam"]
        [std::mem::align_of::<IMV_RecordFrameInfoParam>() - 8usize];
    ["Offset of field: IMV_RecordFrameInfoParam::pData"]
        [std::mem::offset_of!(IMV_RecordFrameInfoParam, pData) - 0usize];
    ["Offset of field: IMV_RecordFrameInfoParam::nDataLen"]
        [std::mem::offset_of!(IMV_RecordFrameInfoParam, nDataLen) - 8usize];
    ["Offset of field: IMV_RecordFrameInfoParam::nPaddingX"]
        [std::mem::offset_of!(IMV_RecordFrameInfoParam, nPaddingX) - 12usize];
    ["Offset of field: IMV_RecordFrameInfoParam::nPaddingY"]
        [std::mem::offset_of!(IMV_RecordFrameInfoParam, nPaddingY) - 16usize];
    ["Offset of field: IMV_RecordFrameInfoParam::ePixelFormat"]
        [std::mem::offset_of!(IMV_RecordFrameInfoParam, ePixelFormat) - 20usize];
    ["Offset of field: IMV_RecordFrameInfoParam::nReserved"]
        [std::mem::offset_of!(IMV_RecordFrameInfoParam, nReserved) - 24usize];
};

/// brief 图像翻转结构体
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMV_FlipImageParam {
    /// [IN] 图像宽
    pub nWidth: c_uint,
    /// [IN] 图像高
    pub nHeight: c_uint,
    /// [IN] 像素格式
    pub ePixelFormat: IMV_EPixelType::IMV_EPixelType,
    /// [IN] 翻转类型
    pub eFlipType: IMV_EFlipType::IMV_EFlipType,
    /// [IN] 输入图像数据
    pub pSrcData: *mut c_uchar,
    /// [IN] 输入图像长度
    pub nSrcDataLen: c_uint,
    /// [OUT] 输出数据缓存(调用者分配缓存)
    pub pDstBuf: *mut c_uchar,
    /// [IN] 提供的输出缓冲区大小
    pub nDstBufSize: c_uint,
    /// [OUT] 输出数据长度
    pub nDstDataLen: c_uint,
    /// 预留
    pub nReserved: [c_uint; 8usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMV_FlipImageParam"][std::mem::size_of::<IMV_FlipImageParam>() - 80usize];
    ["Alignment of IMV_FlipImageParam"][std::mem::align_of::<IMV_FlipImageParam>() - 8usize];
    ["Offset of field: IMV_FlipImageParam::nWidth"]
        [std::mem::offset_of!(IMV_FlipImageParam, nWidth) - 0usize];
    ["Offset of field: IMV_FlipImageParam::nHeight"]
        [std::mem::offset_of!(IMV_FlipImageParam, nHeight) - 4usize];
    ["Offset of field: IMV_FlipImageParam::ePixelFormat"]
        [std::mem::offset_of!(IMV_FlipImageParam, ePixelFormat) - 8usize];
    ["Offset of field: IMV_FlipImageParam::eFlipType"]
        [std::mem::offset_of!(IMV_FlipImageParam, eFlipType) - 12usize];
    ["Offset of field: IMV_FlipImageParam::pSrcData"]
        [std::mem::offset_of!(IMV_FlipImageParam, pSrcData) - 16usize];
    ["Offset of field: IMV_FlipImageParam::nSrcDataLen"]
        [std::mem::offset_of!(IMV_FlipImageParam, nSrcDataLen) - 24usize];
    ["Offset of field: IMV_FlipImageParam::pDstBuf"]
        [std::mem::offset_of!(IMV_FlipImageParam, pDstBuf) - 32usize];
    ["Offset of field: IMV_FlipImageParam::nDstBufSize"]
        [std::mem::offset_of!(IMV_FlipImageParam, nDstBufSize) - 40usize];
    ["Offset of field: IMV_FlipImageParam::nDstDataLen"]
        [std::mem::offset_of!(IMV_FlipImageParam, nDstDataLen) - 44usize];
    ["Offset of field: IMV_FlipImageParam::nReserved"]
        [std::mem::offset_of!(IMV_FlipImageParam, nReserved) - 48usize];
};

/// brief 图像旋转结构体
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMV_RotateImageParam {
    /// [IN][OUT] 图像宽
    pub nWidth: c_uint,
    /// [IN][OUT] 图像高
    pub nHeight: c_uint,
    /// [IN] 像素格式
    pub ePixelFormat: IMV_EPixelType::IMV_EPixelType,
    /// [IN] 旋转角度
    pub eRotationAngle: IMV_ERotationAngle::IMV_ERotationAngle,
    /// [IN] 输入图像数据
    pub pSrcData: *mut c_uchar,
    /// [IN] 输入图像长度
    pub nSrcDataLen: c_uint,
    /// [OUT] 输出数据缓存(调用者分配缓存)
    pub pDstBuf: *mut c_uchar,
    /// [IN] 提供的输出缓冲区大小
    pub nDstBufSize: c_uint,
    /// [OUT] 输出数据长度
    pub nDstDataLen: c_uint,
    /// 预留
    pub nReserved: [c_uint; 8usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMV_RotateImageParam"][std::mem::size_of::<IMV_RotateImageParam>() - 80usize];
    ["Alignment of IMV_RotateImageParam"][std::mem::align_of::<IMV_RotateImageParam>() - 8usize];
    ["Offset of field: IMV_RotateImageParam::nWidth"]
        [std::mem::offset_of!(IMV_RotateImageParam, nWidth) - 0usize];
    ["Offset of field: IMV_RotateImageParam::nHeight"]
        [std::mem::offset_of!(IMV_RotateImageParam, nHeight) - 4usize];
    ["Offset of field: IMV_RotateImageParam::ePixelFormat"]
        [std::mem::offset_of!(IMV_RotateImageParam, ePixelFormat) - 8usize];
    ["Offset of field: IMV_RotateImageParam::eRotationAngle"]
        [std::mem::offset_of!(IMV_RotateImageParam, eRotationAngle) - 12usize];
    ["Offset of field: IMV_RotateImageParam::pSrcData"]
        [std::mem::offset_of!(IMV_RotateImageParam, pSrcData) - 16usize];
    ["Offset of field: IMV_RotateImageParam::nSrcDataLen"]
        [std::mem::offset_of!(IMV_RotateImageParam, nSrcDataLen) - 24usize];
    ["Offset of field: IMV_RotateImageParam::pDstBuf"]
        [std::mem::offset_of!(IMV_RotateImageParam, pDstBuf) - 32usize];
    ["Offset of field: IMV_RotateImageParam::nDstBufSize"]
        [std::mem::offset_of!(IMV_RotateImageParam, nDstBufSize) - 40usize];
    ["Offset of field: IMV_RotateImageParam::nDstDataLen"]
        [std::mem::offset_of!(IMV_RotateImageParam, nDstDataLen) - 44usize];
    ["Offset of field: IMV_RotateImageParam::nReserved"]
        [std::mem::offset_of!(IMV_RotateImageParam, nReserved) - 48usize];
};

/// brief 枚举：图像保存格式
pub mod IMV_ESaveFileType {
    pub type IMV_ESaveFileType = std::os::raw::c_uint;

    /// BMP图像格式
    pub const typeSaveBmp: IMV_ESaveFileType = 0;
    /// JPEG图像格式
    pub const typeSaveJpeg: IMV_ESaveFileType = 1;
    /// PNG图像格式
    pub const typeSavePng: IMV_ESaveFileType = 2;
    /// TIFF原图像格式
    pub const typeSaveTiff: IMV_ESaveFileType = 3;
    /// TIFF有效位拉伸至16Bit图像格式
    pub const typeSaveTiff_Extend: IMV_ESaveFileType = 4;
    /// TIFF(仅支持Bayer10\\12\\16)原图像格式
    pub const typeSaveTiff_RGB: IMV_ESaveFileType = 5;
    /// TIFF(仅支持Bayer10\\12\\16)有效位拉伸至16Bit图像格式
    pub const typeSaveTiff_RGB_Extend: IMV_ESaveFileType = 6;
    /// 未定义的图像格式
    pub const typeSaveUndefined: IMV_ESaveFileType = 255;
}

/// brief 保存图像结构体
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMV_SaveImageToFileParam {
    /// [IN] 图像宽
    pub nWidth: c_uint,
    /// [IN] 图像高
    pub nHeight: c_uint,
    /// [IN] 输入数据的像素格式
    pub ePixelFormat: IMV_EPixelType::IMV_EPixelType,
    /// [IN] 输入图像数据
    pub pSrcData: *mut c_uchar,
    /// [IN] 输入数据大小
    pub nSrcDataLen: c_uint,
    /// [IN] 输入保存图片格式
    pub eImageType: IMV_ESaveFileType::IMV_ESaveFileType,
    /// [IN] 输入文件路径
    pub pImagePath: *mut c_char,
    /// [IN] JPG编码质量(50-99]，PNG编码质量[0-9]
    pub nQuality: c_uint,
    /// [IN] 转换Bayer格式算法
    pub eBayerDemosaic: IMV_EBayerDemosaic::IMV_EBayerDemosaic,
    /// 预留
    pub nReserved: [c_uint; 8usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMV_SaveImageToFileParam"][std::mem::size_of::<IMV_SaveImageToFileParam>() - 80usize];
    ["Alignment of IMV_SaveImageToFileParam"]
        [std::mem::align_of::<IMV_SaveImageToFileParam>() - 8usize];
    ["Offset of field: IMV_SaveImageToFileParam::nWidth"]
        [std::mem::offset_of!(IMV_SaveImageToFileParam, nWidth) - 0usize];
    ["Offset of field: IMV_SaveImageToFileParam::nHeight"]
        [std::mem::offset_of!(IMV_SaveImageToFileParam, nHeight) - 4usize];
    ["Offset of field: IMV_SaveImageToFileParam::ePixelFormat"]
        [std::mem::offset_of!(IMV_SaveImageToFileParam, ePixelFormat) - 8usize];
    ["Offset of field: IMV_SaveImageToFileParam::pSrcData"]
        [std::mem::offset_of!(IMV_SaveImageToFileParam, pSrcData) - 16usize];
    ["Offset of field: IMV_SaveImageToFileParam::nSrcDataLen"]
        [std::mem::offset_of!(IMV_SaveImageToFileParam, nSrcDataLen) - 24usize];
    ["Offset of field: IMV_SaveImageToFileParam::eImageType"]
        [std::mem::offset_of!(IMV_SaveImageToFileParam, eImageType) - 28usize];
    ["Offset of field: IMV_SaveImageToFileParam::pImagePath"]
        [std::mem::offset_of!(IMV_SaveImageToFileParam, pImagePath) - 32usize];
    ["Offset of field: IMV_SaveImageToFileParam::nQuality"]
        [std::mem::offset_of!(IMV_SaveImageToFileParam, nQuality) - 40usize];
    ["Offset of field: IMV_SaveImageToFileParam::eBayerDemosaic"]
        [std::mem::offset_of!(IMV_SaveImageToFileParam, eBayerDemosaic) - 44usize];
    ["Offset of field: IMV_SaveImageToFileParam::nReserved"]
        [std::mem::offset_of!(IMV_SaveImageToFileParam, nReserved) - 48usize];
};

/// brief 设备连接状态事件回调函数声明
/// param pParamUpdateArg [in] 回调时主动推送的设备连接状态事件信息
/// param pUser [in] 用户自定义数据
pub type IMV_ConnectCallBack = std::option::Option<
    unsafe extern "C" fn(pConnectArg: *const IMV_SConnectArg, pUser: *mut c_void),
>;

/// brief 参数更新事件回调函数声明
/// param pParamUpdateArg [in] 回调时主动推送的参数更新事件信息
/// param pUser [in] 用户自定义数据
pub type IMV_ParamUpdateCallBack = std::option::Option<
    unsafe extern "C" fn(pParamUpdateArg: *const IMV_SParamUpdateArg, pUser: *mut c_void),
>;

/// brief 流事件回调函数声明
/// param pStreamArg [in] 回调时主动推送的流事件信息
/// param pUser [in] 用户自定义数据
pub type IMV_StreamCallBack = std::option::Option<
    unsafe extern "C" fn(pStreamArg: *const IMV_SStreamArg, pUser: *mut c_void),
>;

/// brief 消息通道事件回调函数声明（Gige专用）
/// param pMsgChannelArg [in] 回调时主动推送的消息通道事件信息
/// param pUser [in] 用户自定义数据
pub type IMV_MsgChannelCallBack = std::option::Option<
    unsafe extern "C" fn(pMsgChannelArg: *const IMV_SMsgChannelArg, pUser: *mut c_void),
>;

/// brief 消息通道事件回调函数声明（通用）
/// param pMsgChannelArg [in] 回调时主动推送的消息通道事件信息
/// param pUser [in] 用户自定义数据
pub type IMV_MsgChannelCallBackEx = std::option::Option<
    unsafe extern "C" fn(pMsgChannelArg: *const IMV_SMsgEventArg, pUser: *mut c_void),
>;

/// brief 帧数据信息回调函数声明
/// param pFrame [in] 回调时主动推送的帧信息
/// param pUser [in] 用户自定义数据
pub type IMV_FrameCallBack =
    std::option::Option<unsafe extern "C" fn(pFrame: *mut IMV_Frame, pUser: *mut c_void)>;

// function define
extern "C" {
    /// brief 获取版本信息
    /// return 成功时返回版本信息，失败时返回NULL
    pub fn IMV_GetVersion() -> *const c_char;

    /// brief 枚举设备
    /// param pDeviceList [OUT] 设备列表
    /// param interfaceType [IN] 待枚举的接口类型, 类型可任意组合,如 interfaceTypeGige | interfaceTypeUsb3
    /// return 成功，返回IMV_OK；错误，返回错误码
    /// remakes 1、当interfaceType = interfaceTypeAll  时，枚举所有接口下的在线设备
    /// 2、当interfaceType = interfaceTypeGige 时，枚举所有GigE网口下的在线设备
    /// 3、当interfaceType = interfaceTypeUsb3 时，枚举所有USB接口下的在线设备
    /// 4、当interfaceType = interfaceTypeCL   时，枚举所有CameraLink接口下的在线设备
    /// 5、该接口下的interfaceType支持任意接口类型的组合,如，若枚举所有GigE网口和USB3接口下的在线设备时,
    /// 可将interfaceType设置为 interfaceType = interfaceTypeGige | interfaceTypeUsb3,其它接口类型组合以此类推
    pub fn IMV_EnumDevices(pDeviceList: *mut IMV_DeviceList, interfaceType: c_uint) -> c_int;

    /// brief 以单播形式枚举设备, 仅限Gige设备使用
    /// param pDeviceList [OUT] 设备列表
    /// param pIpAddress [IN] 设备的IP地址
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_EnumDevicesByUnicast(
        pDeviceList: *mut IMV_DeviceList,
        pIpAddress: *const c_char,
    ) -> c_int;

    /// brief 通过指定标示符创建设备句柄，如指定索引、设备键、设备自定义名、IP地址
    /// param handle [OUT] 设备句柄
    /// param mode [IN] 创建设备方式
    /// param pIdentifier [IN] 指定标示符(设备键、设备自定义名、IP地址为char类型指针强转void类型指针，索引为unsigned int类型指针强转void类型指针)
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_CreateHandle(
        handle: *mut IMV_HANDLE,
        mode: IMV_ECreateHandleMode::IMV_ECreateHandleMode,
        pIdentifier: *mut c_void,
    ) -> c_int;

    /// brief 销毁设备句柄
    /// param handle [IN] 设备句柄
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_DestroyHandle(handle: IMV_HANDLE) -> c_int;

    /// brief 获取设备信息
    /// param handle [IN] 设备句柄
    /// param pDevInfo [OUT] 设备信息
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_GetDeviceInfo(handle: IMV_HANDLE, pDevInfo: *mut IMV_DeviceInfo) -> c_int;

    /// brief  打开设备
    /// param handle [IN] 设备句柄
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_Open(handle: IMV_HANDLE) -> c_int;

    /// brief  打开设备
    /// param handle [IN] 设备句柄
    /// param accessPermission [IN] 控制通道权限(IMV_Open默认使用accessPermissionControl权限)
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_OpenEx(
        handle: IMV_HANDLE,
        accessPermission: IMV_ECameraAccessPermission::IMV_ECameraAccessPermission,
    ) -> c_int;

    /// brief 判断设备是否已打开
    /// param handle [IN] 设备句柄
    /// return 打开状态，返回true；关闭状态或者掉线状态，返回false
    pub fn IMV_IsOpen(handle: IMV_HANDLE) -> bool_;

    /// brief  关闭设备
    /// param handle [IN] 设备句柄
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_Close(handle: IMV_HANDLE) -> c_int;

    /// brief  设置传输类型
    /// param handle [IN] 设备句柄
    /// param transmissionType [IN] 传输类型结构体
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_GIGE_SetTransmissionType(
        handle: IMV_HANDLE,
        transmissionType: IMV_TRANSMISSION_TYPE,
    ) -> c_int;

    /// brief 修改设备IP, 仅限Gige设备使用
    /// param handle [IN] 设备句柄
    /// param pIpAddress [IN] IP地址
    /// param pSubnetMask [IN] 子网掩码
    /// param pGateway [IN] 默认网关
    /// return 成功，返回IMV_OK；错误，返回错误码
    /// remakes 1、调用该函数时如果pSubnetMask和pGateway都设置了有效值，则以此有效值为准;
    /// 2、调用该函数时如果pSubnetMask和pGateway都设置了NULL，则内部实现时用它所连接网卡的子网掩码和网关代替
    /// 3、调用该函数时如果pSubnetMask和pGateway两者中其中一个为NULL，另一个非NULL，则返回错误
    pub fn IMV_GIGE_ForceIpAddress(
        handle: IMV_HANDLE,
        pIpAddress: *const c_char,
        pSubnetMask: *const c_char,
        pGateway: *const c_char,
    ) -> c_int;

    /// brief 获取设备的当前访问权限, 仅限Gige设备使用
    /// param handle [IN] 设备句柄
    /// param pAccessPermission [OUT] 设备的当前访问权限
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_GIGE_GetAccessPermission(
        handle: IMV_HANDLE,
        pAccessPermission: *mut IMV_ECameraAccessPermission::IMV_ECameraAccessPermission,
    ) -> c_int;

    /// brief 设置设备对sdk命令的响应超时时间,仅限Gige设备使用
    /// param handle [IN] 设备句柄
    /// param timeout [IN] 超时时间，单位ms
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_GIGE_SetAnswerTimeout(handle: IMV_HANDLE, timeout: c_uint) -> c_int;

    /// brief 下载设备描述XML文件，并保存到指定路径，如：D:\\\\xml.zip
    /// param handle [IN] 设备句柄
    /// param pFullFileName [IN] 文件要保存的路径
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_DownLoadGenICamXML(handle: IMV_HANDLE, pFullFileName: *const c_char) -> c_int;

    /// brief 保存设备配置到指定的位置。同名文件已存在时，覆盖。
    /// param handle [IN] 设备句柄
    /// param pFullFileName [IN] 导出的设备配置文件全名(含路径)，如：D:\\\\config.xml 或 D:\\\\config.mvcfg
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_SaveDeviceCfg(handle: IMV_HANDLE, pFullFileName: *const c_char) -> c_int;

    /// brief 从文件加载设备xml配置
    /// param handle [IN] 设备句柄
    /// param pFullFileName [IN] 设备配置(xml)文件全名(含路径)，如：D:\\\\config.xml 或 D:\\\\config.mvcfg
    /// param pErrorList [OUT] 加载失败的属性名列表。存放加载失败的属性上限为IMV_MAX_ERROR_LIST_NUM。
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_LoadDeviceCfg(
        handle: IMV_HANDLE,
        pFullFileName: *const c_char,
        pErrorList: *mut IMV_ErrorList,
    ) -> c_int;

    /// brief 从文件加载设备xml配置（支持用户自定义选择是否导入相机网络相关配置，默认不导入）
    /// param handle [IN] 设备句柄
    /// param pFullFileName [IN] 设备配置(xml)文件全名(含路径)，如：D:\\\\config.xml 或 D:\\\\config.mvcfg
    /// param pErrorList [OUT] 加载失败的属性名列表。存放加载失败的属性上限为IMV_MAX_ERROR_LIST_NUM。
    /// param isLoadNetConfig [in] 是否加载网络相关配置
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_LoadDeviceCfgEx(
        handle: IMV_HANDLE,
        pFullFileName: *const c_char,
        pErrorList: *mut IMV_ErrorList,
    ) -> c_int;

    /// brief 写用户自定义数据。相机内部保留32768字节用于用户存储自定义数据
    /// (此功能针对本品牌相机，其它品牌相机无此功能)
    /// param handle [IN] 设备句柄
    /// param pBuffer [IN] 数据缓冲的指针
    /// param pLength [IN] 期望写入的字节数 [OUT] 实际写入的字节数
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_WriteUserPrivateData(
        handle: IMV_HANDLE,
        pBuffer: *mut c_void,
        pLength: *mut c_uint,
    ) -> c_int;

    /// brief 读用户自定义数据。相机内部保留32768字节用于用户存储自定义数据
    /// (此功能针对本品牌相机，其它品牌相机无此功能)
    /// param handle [IN] 设备句柄
    /// param pBuffer [OUT] 数据缓冲的指针
    /// param pLength [IN] 期望读出的字节数 [OUT] 实际读出的字节数
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_ReadUserPrivateData(
        handle: IMV_HANDLE,
        pBuffer: *mut c_void,
        pLength: *mut c_uint,
    ) -> c_int;

    /// brief 往相机串口寄存器写数据，每次写会清除掉上次的数据
    /// (此功能只支持包含串口功能的本品牌相机)
    /// param handle [IN] 设备句柄
    /// param pBuffer [IN] 数据缓冲的指针
    /// param pLength [IN] 期望写入的字节数 [OUT] 实际写入的字节数
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_WriteUARTData(
        handle: IMV_HANDLE,
        pBuffer: *mut c_void,
        pLength: *mut c_uint,
    ) -> c_int;

    /// brief 从相机串口寄存器读取串口数据
    /// (此功能只支持包含串口功能的本品牌相机 )
    /// param handle [IN] 设备句柄
    /// param pBuffer [OUT] 数据缓冲的指针
    /// param pLength [IN] 期望读出的字节数 [OUT] 实际读出的字节数
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_ReadUARTData(
        handle: IMV_HANDLE,
        pBuffer: *mut c_void,
        pLength: *mut c_uint,
    ) -> c_int;

    /// brief 设备连接状态事件回调注册
    /// param handle [IN] 设备句柄
    /// param proc [IN] 设备连接状态事件回调函数
    /// param pUser [IN] 用户自定义数据, 可设为NULL
    /// return 成功，返回IMV_OK；错误，返回错误码
    /// remakes
    /// 只支持一个回调函数,且设备关闭后，注册会失效，打开设备后需重新注册
    pub fn IMV_SubscribeConnectArg(
        handle: IMV_HANDLE,
        proc_: IMV_ConnectCallBack,
        pUser: *mut c_void,
    ) -> c_int;

    /// brief 参数更新事件回调注册
    /// param handle [IN] 设备句柄
    /// param proc [IN] 参数更新注册的事件回调函数
    /// param pUser [IN] 用户自定义数据, 可设为NULL
    /// return 成功，返回IMV_OK；错误，返回错误码
    /// remakes
    /// 只支持一个回调函数,且设备关闭后，注册会失效，打开设备后需重新注册
    pub fn IMV_SubscribeParamUpdateArg(
        handle: IMV_HANDLE,
        proc_: IMV_ParamUpdateCallBack,
        pUser: *mut c_void,
    ) -> c_int;

    /// brief 流通道事件回调注册
    /// param handle [IN] 设备句柄
    /// param proc [IN] 流通道事件回调注册函数
    /// param pUser [IN] 用户自定义数据, 可设为NULL
    /// return 成功，返回IMV_OK；错误，返回错误码
    /// remakes
    /// 只支持一个回调函数,且设备关闭后，注册会失效，打开设备后需重新注册
    pub fn IMV_SubscribeStreamArg(
        handle: IMV_HANDLE,
        proc_: IMV_StreamCallBack,
        pUser: *mut c_void,
    ) -> c_int;

    /// brief 消息通道事件回调注册(GigE专用)
    /// param handle [IN] 设备句柄
    /// param proc [IN] 消息通道事件回调注册函数
    /// param pUser [IN] 用户自定义数据, 可设为NULL
    /// return 成功，返回IMV_OK；错误，返回错误码
    /// remakes
    /// 只支持一个回调函数,且设备关闭后，注册会失效，打开设备后需重新注册
    pub fn IMV_SubscribeMsgChannelArg(
        handle: IMV_HANDLE,
        proc_: IMV_MsgChannelCallBack,
        pUser: *mut c_void,
    ) -> c_int;

    /// brief 设置帧数据缓存个数
    /// param handle [IN] 设备句柄
    /// param nSize [IN] 缓存数量
    /// return 成功，返回IMV_OK；错误，返回错误码
    /// remakes
    /// 不能在拉流过程中设置
    pub fn IMV_SetBufferCount(handle: IMV_HANDLE, nSize: c_uint) -> c_int;

    /// brief 清除帧数据缓存
    /// param handle [IN] 设备句柄
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_ClearFrameBuffer(handle: IMV_HANDLE) -> c_int;

    /// brief 设置驱动包间隔时间(MS),仅对Gige设备有效
    /// param handle [IN] 设备句柄
    /// param nTimeout [IN] 包间隔时间，单位是毫秒
    /// return 成功，返回IMV_OK；错误，返回错误码
    /// remakes 触发模式尾包丢失重传机制
    pub fn IMV_GIGE_SetInterPacketTimeout(handle: IMV_HANDLE, nTimeout: c_uint) -> c_int;

    /// brief 设置单次重传最大包个数, 仅对GigE设备有效
    /// param handle [IN] 设备句柄
    /// param maxPacketNum [IN] 单次重传最大包个数
    /// return 成功，返回IMV_OK；错误，返回错误码
    /// remakes
    /// maxPacketNum为0时，该功能无效
    pub fn IMV_GIGE_SetSingleResendMaxPacketNum(handle: IMV_HANDLE, maxPacketNum: c_uint) -> c_int;

    /// brief 设置同一帧最大丢包的数量,仅对GigE设备有效
    /// param handle [IN] 设备句柄
    /// param maxLostPacketNum [IN] 最大丢包的数量
    /// return 成功，返回IMV_OK；错误，返回错误码
    /// remakes
    /// maxLostPacketNum为0时，该功能无效
    pub fn IMV_GIGE_SetMaxLostPacketNum(handle: IMV_HANDLE, maxLostPacketNum: c_uint) -> c_int;

    /// brief 设置U3V设备的传输数据块的数量和大小,仅对USB设备有效
    /// param handle [IN] 设备句柄
    /// param nNum [IN] 传输数据块的数量(范围:5-256)
    /// param nSize [IN] 传输数据块的大小(范围:8-512, 单位:KByte)
    /// return 成功，返回IMV_OK；错误，返回错误码
    /// remarks
    /// 1、传输数据块数量，范围5 - 256, 默认为64，高分辨率高帧率时可以适当增加该值；多台相机共同使用时，可以适当减小该值
    /// 2、传输每个数据块大小，范围8 - 512, 默认为64，单位是KByte
    pub fn IMV_USB_SetUrbTransfer(handle: IMV_HANDLE, nNum: c_uint, nSize: c_uint) -> c_int;

    /// brief 开始取流
    /// param handle [IN] 设备句柄
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_StartGrabbing(handle: IMV_HANDLE) -> c_int;

    /// brief 开始取流
    /// param handle [IN] 设备句柄
    /// param maxImagesGrabbed [IN] 允许最多的取帧数，达到指定取帧数后停止取流，如果为0，表示忽略此参数连续取流(IMV_StartGrabbing默认0)
    /// param strategy [IN] 取流策略,(IMV_StartGrabbing默认使用grabStrartegySequential策略取流)
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_StartGrabbingEx(
        handle: IMV_HANDLE,
        maxImagesGrabbed: u64,
        strategy: IMV_EGrabStrategy::IMV_EGrabStrategy,
    ) -> c_int;

    /// brief 判断设备是否正在取流
    /// param handle [IN] 设备句柄
    /// return 正在取流，返回true；不在取流，返回false
    pub fn IMV_IsGrabbing(handle: IMV_HANDLE) -> bool_;

    /// brief 停止取流
    /// param handle [IN] 设备句柄
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_StopGrabbing(handle: IMV_HANDLE) -> c_int;

    /// brief 注册帧数据回调函数(异步获取帧数据机制)
    /// param handle [IN] 设备句柄
    /// param proc [IN] 帧数据信息回调函数，建议不要在该函数中处理耗时的操作，否则会阻塞后续帧数据的实时性
    /// param pUser [IN] 用户自定义数据, 可设为NULL
    /// return 成功，返回IMV_OK；错误，返回错误码
    /// remakes
    /// 该异步获取帧数据机制和同步获取帧数据机制(IMV_GetFrame)互斥，对于同一设备，系统中两者只能选其一
    /// 只支持一个回调函数, 且设备关闭后，注册会失效，打开设备后需重新注册
    pub fn IMV_AttachGrabbing(
        handle: IMV_HANDLE,
        proc_: IMV_FrameCallBack,
        pUser: *mut c_void,
    ) -> c_int;

    /// brief 获取一帧图像(同步获取帧数据机制)
    /// param handle [IN] 设备句柄
    /// param pFrame [OUT] 帧数据信息
    /// param timeoutMS [IN] 获取一帧图像的超时时间,INFINITE时表示无限等待,直到收到一帧数据或者停止取流。单位是毫秒
    /// return 成功，返回IMV_OK；错误，返回错误码
    /// remakes
    /// 该接口不支持多线程调用。
    /// 该同步获取帧机制和异步获取帧机制(IMV_AttachGrabbing)互斥,对于同一设备，系统中两者只能选其一。
    /// 使用内部缓存获取图像，需要IMV_ReleaseFrame进行释放图像缓存。
    pub fn IMV_GetFrame(handle: IMV_HANDLE, pFrame: *mut IMV_Frame, timeoutMS: c_uint) -> c_int;

    /// brief 释放图像缓存
    /// param handle [IN] 设备句柄
    /// param pFrame [IN] 帧数据信息
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_ReleaseFrame(handle: IMV_HANDLE, pFrame: *mut IMV_Frame) -> c_int;

    /// brief 帧数据深拷贝克隆
    /// param handle [IN] 设备句柄
    /// param pFrame [IN] 克隆源帧数据信息
    /// param pCloneFrame [OUT] 新的帧数据信息
    /// return 成功，返回IMV_OK；错误，返回错误码
    /// remakes 使用IMV_ReleaseFrame进行释放图像缓存。
    pub fn IMV_CloneFrame(
        handle: IMV_HANDLE,
        pFrame: *mut IMV_Frame,
        pCloneFrame: *mut IMV_Frame,
    ) -> c_int;

    /// brief 获取Chunk数据(仅对GigE/Usb相机有效)
    /// param handle [IN] 设备句柄
    /// param pFrame [IN] 帧数据信息
    /// param index [IN] 索引ID
    /// param pChunkDataInfo [OUT] Chunk数据信息
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_GetChunkDataByIndex(
        handle: IMV_HANDLE,
        pFrame: *mut IMV_Frame,
        index: c_uint,
        pChunkDataInfo: *mut IMV_ChunkDataInfo,
    ) -> c_int;

    /// brief 获取流统计信息(IMV_StartGrabbing / IMV_StartGrabbing执行后调用)
    /// param handle [IN] 设备句柄
    /// param pStreamStatsInfo [OUT] 流统计信息数据
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_GetStatisticsInfo(
        handle: IMV_HANDLE,
        pStreamStatsInfo: *mut IMV_StreamStatisticsInfo,
    ) -> c_int;

    /// brief 重置流统计信息(IMV_StartGrabbing / IMV_StartGrabbing执行后调用)
    /// param handle [IN] 设备句柄
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_ResetStatisticsInfo(handle: IMV_HANDLE) -> c_int;

    /// brief 判断属性是否可用
    /// param handle [IN] 设备句柄
    /// param pFeatureName [IN] 属性名
    /// return 可用，返回true；不可用，返回false
    pub fn IMV_FeatureIsAvailable(handle: IMV_HANDLE, pFeatureName: *const c_char) -> bool_;

    /// brief 判断属性是否可读
    /// param handle [IN] 设备句柄
    /// param pFeatureName [IN] 属性名
    /// return 可读，返回true；不可读，返回false
    pub fn IMV_FeatureIsReadable(handle: IMV_HANDLE, pFeatureName: *const c_char) -> bool_;

    /// brief 判断属性是否可写
    /// param handle [IN] 设备句柄
    /// param pFeatureName [IN] 属性名
    /// return 可写，返回true；不可写，返回false
    pub fn IMV_FeatureIsWriteable(handle: IMV_HANDLE, pFeatureName: *const c_char) -> bool_;

    /// brief 判断属性是否可流
    /// param handle [IN] 设备句柄
    /// param pFeatureName [IN] 属性名
    /// return 可流，返回true；不可流，返回false
    pub fn IMV_FeatureIsStreamable(handle: IMV_HANDLE, pFeatureName: *const c_char) -> bool_;

    /// brief 判断属性是否有效
    /// param handle [IN] 设备句柄
    /// param pFeatureName [IN] 属性名
    /// return 有效，返回true；无效，返回false
    pub fn IMV_FeatureIsValid(handle: IMV_HANDLE, pFeatureName: *const c_char) -> bool_;

    /// brief 获取属性类型
    /// param handle [IN] 设备句柄
    /// param pFeatureName [IN] 属性名
    /// param pPropertyType [OUT] 属性类型
    /// return 获取成功，返回true；获取失败，返回false
    pub fn IMV_GetFeatureType(
        handle: IMV_HANDLE,
        pFeatureName: *const c_char,
        pPropertyType: *mut IMV_EFeatureType::IMV_EFeatureType,
    ) -> bool_;

    /// brief 获取整型属性值
    /// param handle [IN] 设备句柄
    /// param pFeatureName [IN] 属性名
    /// param pIntValue [OUT] 整型属性值
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_GetIntFeatureValue(
        handle: IMV_HANDLE,
        pFeatureName: *const c_char,
        pIntValue: *mut i64,
    ) -> c_int;

    /// brief 获取整型属性可设的最小值
    /// param handle [IN] 设备句柄
    /// param pFeatureName [IN] 属性名
    /// param pIntValue [OUT] 整型属性可设的最小值
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_GetIntFeatureMin(
        handle: IMV_HANDLE,
        pFeatureName: *const c_char,
        pIntValue: *mut i64,
    ) -> c_int;

    /// brief 获取整型属性可设的最大值
    /// param handle [IN] 设备句柄
    /// param pFeatureName [IN] 属性名
    /// param pIntValue [OUT] 整型属性可设的最大值
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_GetIntFeatureMax(
        handle: IMV_HANDLE,
        pFeatureName: *const c_char,
        pIntValue: *mut i64,
    ) -> c_int;

    /// brief 获取整型属性步长
    /// param handle [IN] 设备句柄
    /// param pFeatureName [IN] 属性名
    /// param pIntValue [OUT] 整型属性步长
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_GetIntFeatureInc(
        handle: IMV_HANDLE,
        pFeatureName: *const c_char,
        pIntValue: *mut i64,
    ) -> c_int;

    /// brief 设置整型属性值
    /// param handle [IN] 设备句柄
    /// param pFeatureName [IN] 属性名
    /// param intValue [IN] 待设置的整型属性值
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_SetIntFeatureValue(
        handle: IMV_HANDLE,
        pFeatureName: *const c_char,
        intValue: i64,
    ) -> c_int;

    /// brief 获取浮点属性值
    /// param handle [IN] 设备句柄
    /// param pFeatureName [IN] 属性名
    /// param pDoubleValue [OUT] 浮点属性值
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_GetDoubleFeatureValue(
        handle: IMV_HANDLE,
        pFeatureName: *const c_char,
        pDoubleValue: *mut f64,
    ) -> c_int;

    /// brief 获取浮点属性可设的最小值
    /// param handle [IN] 设备句柄
    /// param pFeatureName [IN] 属性名
    /// param pDoubleValue [OUT] 浮点属性可设的最小值
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_GetDoubleFeatureMin(
        handle: IMV_HANDLE,
        pFeatureName: *const c_char,
        pDoubleValue: *mut f64,
    ) -> c_int;

    /// brief 获取浮点属性可设的最大值
    /// param handle [IN] 设备句柄
    /// param pFeatureName [IN] 属性名
    /// param pDoubleValue [OUT] 浮点属性可设的最大值
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_GetDoubleFeatureMax(
        handle: IMV_HANDLE,
        pFeatureName: *const c_char,
        pDoubleValue: *mut f64,
    ) -> c_int;

    /// brief 设置浮点属性值
    /// param handle [IN] 设备句柄
    /// param pFeatureName [IN] 属性名
    /// param doubleValue [IN] 待设置的浮点属性值
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_SetDoubleFeatureValue(
        handle: IMV_HANDLE,
        pFeatureName: *const c_char,
        doubleValue: f64,
    ) -> c_int;

    /// brief 获取布尔属性值
    /// param handle [IN] 设备句柄
    /// param pFeatureName [IN] 属性名
    /// param pBoolValue [OUT] 布尔属性值
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_GetBoolFeatureValue(
        handle: IMV_HANDLE,
        pFeatureName: *const c_char,
        pBoolValue: *mut bool_,
    ) -> c_int;

    /// brief 设置布尔属性值
    /// param handle [IN] 设备句柄
    /// param pFeatureName [IN] 属性名
    /// param boolValue [IN] 待设置的布尔属性值
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_SetBoolFeatureValue(
        handle: IMV_HANDLE,
        pFeatureName: *const c_char,
        boolValue: bool_,
    ) -> c_int;

    /// brief 获取枚举属性值
    /// param handle [IN] 设备句柄
    /// param pFeatureName [IN] 属性名
    /// param pEnumValue [OUT] 枚举属性值
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_GetEnumFeatureValue(
        handle: IMV_HANDLE,
        pFeatureName: *const c_char,
        pEnumValue: *mut u64,
    ) -> c_int;

    /// brief 设置枚举属性值
    /// param handle [IN] 设备句柄
    /// param pFeatureName [IN] 属性名
    /// param enumValue [IN] 待设置的枚举属性值
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_SetEnumFeatureValue(
        handle: IMV_HANDLE,
        pFeatureName: *const c_char,
        enumValue: u64,
    ) -> c_int;

    /// brief 获取枚举属性symbol值
    /// param handle [IN] 设备句柄
    /// param pFeatureName [IN] 属性名
    /// param pEnumSymbol [OUT] 枚举属性symbol值
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_GetEnumFeatureSymbol(
        handle: IMV_HANDLE,
        pFeatureName: *const c_char,
        pEnumSymbol: *mut IMV_String,
    ) -> c_int;

    /// brief 设置枚举属性symbol值
    /// param handle [IN] 设备句柄
    /// param pFeatureName [IN] 属性名
    /// param pEnumSymbol [IN] 待设置的枚举属性symbol值
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_SetEnumFeatureSymbol(
        handle: IMV_HANDLE,
        pFeatureName: *const c_char,
        pEnumSymbol: *const c_char,
    ) -> c_int;

    /// brief 获取枚举属性的可设枚举值的个数
    /// param handle [IN] 设备句柄
    /// param pFeatureName [IN] 属性名
    /// param pEntryNum [OUT] 枚举属性的可设枚举值的个数
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_GetEnumFeatureEntryNum(
        handle: IMV_HANDLE,
        pFeatureName: *const c_char,
        pEntryNum: *mut c_uint,
    ) -> c_int;

    /// brief 获取枚举属性的可设枚举值列表
    /// param handle [IN] 设备句柄
    /// param pFeatureName [IN] 属性名
    /// param pEnumEntryList [OUT] 枚举属性的可设枚举值列表
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_GetEnumFeatureEntrys(
        handle: IMV_HANDLE,
        pFeatureName: *const c_char,
        pEnumEntryList: *mut IMV_EnumEntryList,
    ) -> c_int;

    /// brief 获取字符串属性值
    /// param handle [IN] 设备句柄
    /// param pFeatureName [IN] 属性名
    /// param pStringValue [OUT] 字符串属性值
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_GetStringFeatureValue(
        handle: IMV_HANDLE,
        pFeatureName: *const c_char,
        pStringValue: *mut IMV_String,
    ) -> c_int;

    /// brief 设置字符串属性值
    /// param handle [IN] 设备句柄
    /// param pFeatureName [IN] 属性名
    /// param pStringValue [IN] 待设置的字符串属性值
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_SetStringFeatureValue(
        handle: IMV_HANDLE,
        pFeatureName: *const c_char,
        pStringValue: *const c_char,
    ) -> c_int;

    /// brief 执行命令属性
    /// param handle [IN] 设备句柄
    /// param pFeatureName [IN] 属性名
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_ExecuteCommandFeature(handle: IMV_HANDLE, pFeatureName: *const c_char) -> c_int;

    /// brief 像素格式转换
    /// param handle [IN] 设备句柄
    /// param pstPixelConvertParam [IN][OUT] 像素格式转换参数结构体
    /// return 成功，返回IMV_OK；错误，返回错误码
    /// remarks
    /// 只支持转化成目标像素格式gvspPixelRGB8 / gvspPixelBGR8 / gvspPixelMono8 / gvspPixelBGRA8
    /// 通过该接口将原始图像数据转换成用户所需的像素格式并存放在调用者指定内存中。
    /// 像素格式为YUV411Packed的时，图像宽须能被4整除
    /// 像素格式为YUV422Packed的时，图像宽须能被2整除
    /// 像素格式为YUYVPacked的时，图像宽须能被2整除
    /// 转换后的图像:数据存储是从最上面第一行开始的，这个是相机数据的默认存储方向
    pub fn IMV_PixelConvert(
        handle: IMV_HANDLE,
        pstPixelConvertParam: *mut IMV_PixelConvertParam,
    ) -> c_int;

    /// brief 打开录像
    /// param handle [IN] 设备句柄
    /// param pstRecordParam [IN] 录像参数结构体
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_OpenRecord(handle: IMV_HANDLE, pstRecordParam: *mut IMV_RecordParam) -> c_int;

    /// brief 录制一帧图像
    /// param handle [IN] 设备句柄
    /// param pstRecordFrameInfoParam [IN] 录像用帧信息结构体
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_InputOneFrame(
        handle: IMV_HANDLE,
        pstRecordFrameInfoParam: *mut IMV_RecordFrameInfoParam,
    ) -> c_int;

    /// brief 关闭录像
    /// param handle [IN] 设备句柄
    /// return 成功，返回IMV_OK；错误，返回错误码
    pub fn IMV_CloseRecord(handle: IMV_HANDLE) -> c_int;

    /// brief 图像翻转
    /// param handle [IN] 设备句柄
    /// param pstFlipImageParam [IN][OUT] 图像翻转参数结构体
    /// return 成功，返回IMV_OK；错误，返回错误码
    /// remarks
    /// 只支持像素格式gvspPixelRGB8 / gvspPixelBGR8 / gvspPixelMono8的图像的垂直和水平翻转。
    /// 通过该接口将原始图像数据翻转后并存放在调用者指定内存中。
    pub fn IMV_FlipImage(handle: IMV_HANDLE, pstFlipImageParam: *mut IMV_FlipImageParam) -> c_int;

    /// brief 图像顺时针旋转
    /// param handle [IN] 设备句柄
    /// param pstRotateImageParam [IN][OUT] 图像旋转参数结构体
    /// return 成功，返回IMV_OK；错误，返回错误码
    /// remarks
    /// 只支持gvspPixelRGB8 / gvspPixelBGR8 / gvspPixelMono8格式数据的90/180/270度顺时针旋转。
    /// 通过该接口将原始图像数据旋转后并存放在调用者指定内存中。
    pub fn IMV_RotateImage(
        handle: IMV_HANDLE,
        pstRotateImageParam: *mut IMV_RotateImageParam,
    ) -> c_int;

    /// brief 保存图像到文件
    /// param handle [IN] 设备句柄
    /// param pstSaveFileParam [IN] 保存图片到文件参数结构体
    /// return 成功，返回IMV_OK；错误，返回错误码
    /// remarks
    /// 该接口支持保存BMP/JPEG/PNG/TIFF
    /// JPEG格式最大支持宽高为65500
    pub fn IMV_SaveImageToFile(
        handle: IMV_HANDLE,
        pstSaveFileParam: *mut IMV_SaveImageToFileParam,
    ) -> c_int;
}
