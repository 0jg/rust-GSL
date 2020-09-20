//
// A rust binding for the GSL library by Guillaume Gomez (guillaume1.gomez@gmail.com)
//

/*!
Lambert’s W functions, W(x), are defined to be solutions of the equation W(x) \exp(W(x)) = x. This function has multiple branches for x < 0; however, it has only two real-valued branches.
We define W_0(x) to be the principal branch, where W > -1 for x < 0, and W_{-1}(x) to be the other real branch, where W < -1 for x < 0.
!*/

use enums;
use std::mem::zeroed;

/// This computes the principal branch of the Lambert W function, W_0(x).
pub fn lambert_W0(x: f64) -> f64 {
    unsafe { sys::gsl_sf_lambert_W0(x) }
}

/// This computes the principal branch of the Lambert W function, W_0(x).
pub fn lambert_W0_e(x: f64) -> Result<::types::Result, enums::Value> {
    let mut result = unsafe { zeroed::<sys::gsl_sf_result>() };
    let ret = unsafe { sys::gsl_sf_lambert_W0_e(x, &mut result) };

    let ret = enums::Value::from(ret);
    if ret.is_success() {
        Ok(result.into())
    } else {
        Err(ret)
    }
}

/// This computes the secondary real-valued branch of the Lambert W function, W_{-1}(x).
pub fn lambert_Wm1(x: f64) -> f64 {
    unsafe { sys::gsl_sf_lambert_Wm1(x) }
}

/// This computes the secondary real-valued branch of the Lambert W function, W_{-1}(x).
pub fn lambert_Wm1_e(x: f64) -> Result<::types::Result, enums::Value> {
    let mut result = unsafe { zeroed::<sys::gsl_sf_result>() };
    let ret = unsafe { sys::gsl_sf_lambert_Wm1_e(x, &mut result) };

    let ret = enums::Value::from(ret);
    if ret.is_success() {
        Ok(result.into())
    } else {
        Err(ret)
    }
}
