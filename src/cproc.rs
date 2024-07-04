use super::context::Context;
use super::error::XCamError;
use super::ffi;
use super::types::{CprocAttrib, XCamResult};

/// A contract describing colour processing.

pub trait ColorProcessing {
	
/// 

    fn get_cproc_attrib(&self) -> XCamResult<CprocAttrib>;
	 
    fn set_cproc_attrib<T: Into<CprocAttrib>>(&self, cproc_attr: T) -> XCamResult<()>;
	
}

impl ColorProcessing for Context {
	
    fn get_cproc_attrib(&self) -> XCamResult<CprocAttrib> {
        let mut cproc_attr: CprocAttrib = Default::default();
        unsafe {
            XCamError::from(ffi:: rk_aiq_user_api2_acp_GetAttrib(
                self.internal.as_ptr(),
                &mut cproc_attr,
            ))
            .ok()
            .map(|_| cproc_attr)
        }
    }
	
	
    fn set_cproc_attrib<T: Into<CprocAttrib>>(&self, cproc_attr: T) -> XCamResult<()> {
        
        unsafe {
            XCamError::from(ffi:: rk_aiq_user_api2_acp_SetAttrib(
                self.internal.as_ptr(),
                cproc_attr.into(),
            ))
            .ok()
        }
    }
}
