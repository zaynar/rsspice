//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ZZGET: i32 = -1;
const ZZPUT: i32 = -2;
const ZZRESET: i32 = -3;
const ZZNOP: i32 = 3;
const GEN: i32 = 1;
const GF_REF: i32 = 2;
const GF_TOL: i32 = 3;
const GF_DT: i32 = 4;
const NID: i32 = 4;
const LNSIZE: i32 = 80;

struct SaveVars {
    SVFNAM: Vec<u8>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SVFNAM = vec![b' '; LNSIZE as usize];

        fstr::assign(&mut SVFNAM, b" ");

        Self { SVFNAM }
    }
}

//$Procedure  T_RELX (Test utilities)
pub fn T_RELX(
    UDFUNC: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    F: &[u8],
    ET: f64,
    VALUE: f64,
    DECRES: bool,
    LSSTHN: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    spicelib::CHKIN(b"T_RELX", ctx)?;
    spicelib::SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    spicelib::CHKOUT(b"T_RELX", ctx)?;
    Ok(())
}

//
// UDFUNC: Get quantity at ET.
//
pub fn T_GET(ET: &mut f64, VALUE: &mut f64, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    spicelib::CHKIN(b"T_GET", ctx)?;

    if fstr::eq(&save.SVFNAM, b"X") {
        *VALUE = *ET;
    } else if fstr::eq(&save.SVFNAM, b"SIN(X)") {
        *VALUE = f64::sin(*ET);
    } else if fstr::eq(&save.SVFNAM, b"(X**2)*SIN(X)") {
        *VALUE = (f64::powi(*ET, 2) * f64::sin(*ET));
    } else if fstr::eq(&save.SVFNAM, b"D((X**2)*SIN(X))") {
        //
        // This is the derivative of (X**2)*SIN(X).
        //
        *VALUE = ((((2 as f64) * *ET) * f64::sin(*ET)) + (f64::powi(*ET, 2) * f64::cos(*ET)));
    } else if fstr::eq(&save.SVFNAM, b"D2((X**2)*SIN(X))") {
        //
        // This is the 2nd derivative of (X**2)*SIN(X).
        //
        //
        *VALUE = (((((2 as f64) * f64::sin(*ET)) + (((2 as f64) * *ET) * f64::cos(*ET)))
            + (((2 as f64) * *ET) * f64::cos(*ET)))
            + (f64::powi(*ET, 2) * -f64::sin(*ET)));
    } else {
        spicelib::SETMSG(b"Bad function; function name is #.", ctx);
        spicelib::ERRCH(b"#", &save.SVFNAM, ctx);
        spicelib::SIGERR(b"SPICE(TSPICEBUG)", ctx)?;
    }

    spicelib::CHKOUT(b"T_GET", ctx)?;
    Ok(())
}

//
// UDCOND: Indicate whether function is less than reference value
// at specified epoch.
//
pub fn T_LT(
    UDFUNC: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    ET: &mut f64,
    LSSTHN: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut FVAL: f64 = 0.0;
    let mut SVREF: f64 = 0.0;
    let mut OK: bool = false;

    *LSSTHN = false;

    if fstr::eq(&save.SVFNAM, b"X") {
        FVAL = *ET;
    } else if fstr::eq(&save.SVFNAM, b"SIN(X)") {
        FVAL = f64::sin(*ET);
    } else if fstr::eq(&save.SVFNAM, b"(X**2)*SIN(X)") {
        FVAL = (f64::powi(*ET, 2) * f64::sin(*ET));
    } else if fstr::eq(&save.SVFNAM, b"D((X**2)*SIN(X))") {
        //
        // This is the derivative of (X**2)*SIN(X)
        //
        FVAL = ((((2 as f64) * *ET) * f64::sin(*ET)) + (f64::powi(*ET, 2) * f64::cos(*ET)));
    } else if fstr::eq(&save.SVFNAM, b"D2((X**2)*SIN(X))") {
        //
        // This is the 2nd derivative of (X**2)*SIN(X)
        //
        FVAL = (((((2 as f64) * f64::sin(*ET)) + (((2 as f64) * *ET) * f64::cos(*ET)))
            + (((2 as f64) * *ET) * f64::cos(*ET)))
            + (f64::powi(*ET, 2) * -f64::sin(*ET)));
    } else {
        spicelib::SETMSG(b"Bad function; function name is #.", ctx);
        spicelib::ERRCH(b"#", &save.SVFNAM, ctx);
        spicelib::SIGERR(b"SPICE(TSPICEBUG)", ctx)?;
    }

    //
    // Retrieve the reference value.
    //
    spicelib::ZZHOLDD(ZZGET, GF_REF, &mut OK, &mut SVREF, ctx)?;

    if !OK {
        spicelib::SETMSG(b"ZZHOLDD GET failed. This indicates a logic error in the GF code due either to a failure to store the GF reference value or a post store reset of ZZHOLDD.", ctx);
        spicelib::SIGERR(b"SPICE(ZZHOLDDGETFAILED)", ctx)?;
        return Ok(());
    }

    *LSSTHN = (FVAL < SVREF);

    Ok(())
}

//
// UDQDEC: Indicate whether function is decreasing
// at specified epoch.
//
pub fn T_DEC(
    UDFUNC: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    ET: &mut f64,
    DECRES: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut DFVAL: f64 = 0.0;

    *DECRES = false;

    if fstr::eq(&save.SVFNAM, b"X") {
        DFVAL = 1.0;
    } else if fstr::eq(&save.SVFNAM, b"SIN(X)") {
        DFVAL = f64::cos(*ET);
    } else if fstr::eq(&save.SVFNAM, b"(X**2)*SIN(X)") {
        DFVAL = ((((2 as f64) * *ET) * f64::sin(*ET)) + (f64::powi(*ET, 2) * f64::cos(*ET)));
    } else if fstr::eq(&save.SVFNAM, b"D((X**2)*SIN(X))") {
        //
        // The function is the derivative of (X**2)*SIN(X).
        // DFVAL is the 2nd derivative of this function,
        // in other words the derivative of
        //
        //    2*X*SIN(X) +  (X**2)*COS(X)
        //
        DFVAL = (((((2 as f64) * f64::sin(*ET)) + (((2 as f64) * *ET) * f64::cos(*ET)))
            + (((2 as f64) * *ET) * f64::cos(*ET)))
            + (f64::powi(*ET, 2) * -f64::sin(*ET)));
    } else {
        spicelib::SETMSG(b"Bad function; function name is #.", ctx);
        spicelib::ERRCH(b"#", &save.SVFNAM, ctx);
        spicelib::SIGERR(b"SPICE(TSPICEBUG)", ctx)?;
    }

    *DECRES = (DFVAL < 0.0);

    Ok(())
}

//
// Select function.
//
pub fn T_SET(F: &[u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    spicelib::LJUST(F, &mut save.SVFNAM);
    spicelib::UCASE(&save.SVFNAM.to_vec(), &mut save.SVFNAM, ctx);
}
