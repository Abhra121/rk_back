//! 自动白平衡
//!
//! AWB 模块的功能是通过改变拍摄设备的色彩通道的增益，The function of the module is to change the gain of the color channel of the shooting device
//！对色温环境所造成的颜色偏差和拍摄设备本身所固有的色彩通道增益的偏差进行统一补偿，Uniformly compensate for the color deviation caused by the color temperature environment and the deviation of the color channel gain inherent in the shooting equipment itself
//！从而让获得的图像能正确反映物体的真实色彩。This allows the image obtained to correctly reflect the true color of the object.
use super::context::Context;
use super::error::XCamError;
use super::ffi;
use super::types::{OpMode, MWBalanceMode, WbGain, AutoWbGain,AutoWbAttrib, WbScene, GetMultiWindowAttrib, XCamResult};

/// 一个描述自动白平衡的契定。A convention describing automatic white balance.
pub trait AutoWhiteBalance {
    /// 获取白平衡工作模式。Get the white balance working mode.
    fn get_wb_mode(&self) -> XCamResult<OpMode>;

    /// 设置白平衡工作模式。Set the white balance working mode.
    fn set_wb_mode(&self, mode: OpMode) -> XCamResult<()>;

    /// 锁定当前白平衡参数。Lock the current white balance parameters
    fn lock_awb(&self) -> XCamResult<()>;

    /// 解锁已被锁定的白平衡参数。Unlock locked white balance parameters
    fn unlock_awb(&self) -> XCamResult<()>;

    /// 获取白平衡场景。Get the white balance scene
    fn get_mwb_scene(&self) -> XCamResult<WbScene>;

    /// 设置白平衡场景。Set the white balance scene.
    fn set_mwb_scene<T: Into<WbScene>>(&self, scene: T) -> XCamResult<()>;

    /// 获取白平衡增益系数。Get the white balance gain coefficient
    fn get_mwb_gain(&self) -> XCamResult<WbGain>;

    /// 设置白平衡增益系数。Set the white balance gain coefficient.
    fn set_mwb_gain<T: Into<WbGain>>(&self, gain: T) -> XCamResult<()>;

    /// 获取白平衡色温参数。Get the white balance color temperature parameters
    fn get_mwb_ct(&self) -> XCamResult<u32>;

    /// 设置白平衡色温参数。Set the white balance color temperature parameters.
    fn set_mwb_ct(&self, ct: u32) -> XCamResult<()>;

    fn get_multiwindow_attrib(&self) -> XCamResult<GetMultiWindowAttrib>;

   // fn set_multiwindow_attrib<T: Into<GetMultiWindowMAttrib>>(&self, multiwindow_attr: T) -> XCamResult<()>;

    fn get_autowb_gain(&self) -> XCamResult<AutoWbGain>;

    //get_autowb_attrib
    fn get_autowb_attrib(&self) -> XCamResult<AutoWbAttrib>;
}

impl AutoWhiteBalance for Context {
    fn get_wb_mode(&self) -> XCamResult<OpMode> {
    //fn get_wb_mode(&self) -> XCamResult<MWBalanceMode> {
        let mut mode = ffi::opMode_t::OP_INVAL;
//rk_aiq_wb_mwb_mode_t
        //let mut mode = ffi::rk_aiq_wb_mwb_mode_t::RK_AIQ_MWB_MODE_INVAILD;
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi2_getWBMode(
                self.internal.as_ptr(),
                &mut mode,
            ))
            .ok()
            .map(|_| mode.into())
        }
    }

    #[cfg(any(feature = "v1_0", feature = "v2_0", feature = "v3_0"))]
    fn set_wb_mode(&self, mode: OpMode) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi2_setWBMode(
                self.internal.as_ptr(),
                mode.into(),
            ))
            .ok()
        }
    }

    #[cfg(any(feature = "v4_0", feature = "v5_0"))]
    fn set_wb_mode(&self, mode: OpMode) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi2_setWBMode(
                self.internal.as_ptr(),
                mode.into(),
            ))
            .ok()
        }
    }

    fn lock_awb(&self) -> XCamResult<()> {
        unsafe { XCamError::from(ffi::rk_aiq_uapi2_lockAWB(self.internal.as_ptr())).ok() }
    }

    fn unlock_awb(&self) -> XCamResult<()> {
        unsafe { XCamError::from(ffi::rk_aiq_uapi2_unlockAWB(self.internal.as_ptr())).ok() }
    }

    fn get_mwb_scene(&self) -> XCamResult<WbScene> {
        let mut scene: WbScene = Default::default();
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi2_getMWBScene(
                self.internal.as_ptr(),
                &mut scene,
            ))
            .ok()
            .map(|_| scene)
        }
    }

    fn set_mwb_scene<T: Into<WbScene>>(&self, scene: T) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi2_setMWBScene(
                self.internal.as_ptr(),
                scene.into(),
            ))
            .ok()
        }
    }

    fn get_mwb_gain(&self) -> XCamResult<WbGain> {
        let mut gain: WbGain = Default::default();
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi2_getWBGain(
                self.internal.as_ptr(),
                &mut gain,
            ))
            .ok()
            .map(|_| gain)
        }
    }

    fn set_mwb_gain<T: Into<WbGain>>(&self, gain: T) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi2_setMWBGain(
                self.internal.as_ptr(),
                &mut gain.into(),
            ))
            .ok()
        }
    }

    fn get_mwb_ct(&self) -> XCamResult<u32> {
        let mut ct: u32 = 0;
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi2_getWBCT(self.internal.as_ptr(), &mut ct))
                .ok()
                .map(|_| ct)
        }
    }

    fn set_mwb_ct(&self, ct: u32) -> XCamResult<()> {
        unsafe { XCamError::from(ffi::rk_aiq_uapi2_setMWBCT(self.internal.as_ptr(), ct)).ok() }
    }
    
    fn get_multiwindow_attrib(&self) -> XCamResult<GetMultiWindowAttrib> {
        let mut multiwindow_attr: GetMultiWindowAttrib = Default::default();
        unsafe {
            XCamError::from(ffi:: rk_aiq_user_api2_awb_GetMultiWindowAttrib(
                self.internal.as_ptr(),
                &mut multiwindow_attr,
            ))
            .ok()
            .map(|_| multiwindow_attr)
        }
    }
/*
    fn set_multiwindow_attrib<T: Into<GetMultiWindowAttrib>>(&self, multiwindow_attr: T) -> XCamResult<()> {
        
        unsafe {
            XCamError::from(ffi:: rk_aiq_user_api2_awb_SetMultiWindowAttrib(
                self.internal.as_ptr(),
                multiwindow_attr.into(),
            ))
            .ok()
        }
    }
*/
    fn get_autowb_gain(&self) -> XCamResult<AutoWbGain> {
        let mut autogain_attr: AutoWbGain = Default::default();
        unsafe {//rk_aiq_user_api2_awb_QueryWBInfo
            XCamError::from(ffi:: rk_aiq_uapi2_getAwbGainOffsetAttrib(
                self.internal.as_ptr(),
                &mut autogain_attr,
            ))
            .ok()
            .map(|_| autogain_attr)
        }
    }

    fn get_autowb_attrib(&self) -> XCamResult<AutoWbAttrib> {
        let mut autowb_attr: AutoWbAttrib = Default::default();
        unsafe {
            XCamError::from(ffi:: rk_aiq_user_api2_awb_QueryWBInfo(
                self.internal.as_ptr(),
                &mut autowb_attr,
            ))
            .ok()
            .map(|_| autowb_attr)
        }
    }
    
}


pub enum WbOpMode {
    #[cfg(feature = "v3_0")]
    Invalid,
    Manual,
    Auto,
    Max,
}

impl From<ffi::opMode_t> for WbOpMode {
    fn from(val: ffi::opMode_t) -> Self {
        use ffi::opMode_t::*;
        match val {
            OP_AUTO => WbOpMode::Auto,
            #[cfg(feature = "v1_0")]
            OP_SEMI_AUTO => WbOpMode::Invalid,
            #[cfg(not(feature = "v1_0"))]
            OP_SEMI_AUTO => WbOpMode::Max,
            OP_MANUAL => WbOpMode::Manual,
            #[cfg(feature = "v1_0")]
            _ => WbOpMode::Invalid,
            #[cfg(not(feature = "v1_0"))]
            _ => WbOpMode::Max,
        }
    }
}

impl From<ffi::rk_aiq_wb_op_mode_t> for WbOpMode {
    fn from(val: ffi::rk_aiq_wb_op_mode_t) -> Self {
        use ffi::rk_aiq_wb_op_mode_t::*;
        match val {
            #[cfg(feature = "v3_0")]
            RK_AIQ_WB_MODE_INVALID => WbOpMode::Invalid,
            RK_AIQ_WB_MODE_MANUAL => WbOpMode::Manual,
            RK_AIQ_WB_MODE_AUTO => WbOpMode::Auto,
            RK_AIQ_WB_MODE_MAX => WbOpMode::Max,
        }
    }
}

impl From<WbOpMode> for ffi::opMode_t {
    fn from(val: WbOpMode) -> Self {
        use ffi::opMode_t::*;
        match val {
            #[cfg(feature = "v3_0")]
            WbOpMode::Invalid => OP_INVAL,
            WbOpMode::Manual => OP_MANUAL,
            WbOpMode::Auto => OP_AUTO,
            WbOpMode::Max => OP_INVAL,
        }
    }
}

impl From<WbOpMode> for ffi::rk_aiq_wb_op_mode_t {
    fn from(val: WbOpMode) -> Self {
        use ffi::rk_aiq_wb_op_mode_t::*;
        match val {
            #[cfg(feature = "v3_0")]
            WbOpMode::Invalid => RK_AIQ_WB_MODE_INVALID,
            WbOpMode::Manual => RK_AIQ_WB_MODE_MANUAL,
            WbOpMode::Auto => RK_AIQ_WB_MODE_AUTO,
            WbOpMode::Max => RK_AIQ_WB_MODE_MAX,
        }
    }
}
