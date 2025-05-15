//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;

struct SaveVars {
    SVFNAM: Vec<u8>,
    SVREF: f64,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SVFNAM = vec![b' '; LNSIZE as usize];
        let mut SVREF: f64 = 0.0;

        fstr::assign(&mut SVFNAM, b" ");
        SVREF = 0.0;

        Self { SVFNAM, SVREF }
    }
}

//*********************************************************************
//*
//*    External routines
//*
//*********************************************************************

//
// Utility package umbrella:
//
pub fn T_ZZGFREL_UTIL(
    F: &[u8],
    ET: f64,
    REF: f64,
    VALUE: f64,
    DECRES: bool,
    LSSTHN: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

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

    spicelib::CHKIN(b"T_ZZGFREL_UTIL", ctx)?;
    spicelib::SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    spicelib::CHKOUT(b"T_ZZGFREL_UTIL", ctx)?;
    Ok(())
}

//
// Get quantity at ET.
//
pub fn T_ZZGFREL_GET(ET: f64, VALUE: &mut f64, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    spicelib::CHKIN(b"T_ZZGFREL_GET", ctx)?;

    if fstr::eq(&save.SVFNAM, b"X") {
        *VALUE = ET;
    } else if fstr::eq(&save.SVFNAM, b"SIN(X)") {
        *VALUE = f64::sin(ET);
    } else if fstr::eq(&save.SVFNAM, b"(X**2)*SIN(X)") {
        *VALUE = (f64::powi(ET, 2) * f64::sin(ET));
    } else if fstr::eq(&save.SVFNAM, b"D((X**2)*SIN(X))") {
        //
        // This is the derivative of (X**2)*SIN(X).
        //
        *VALUE = ((((2 as f64) * ET) * f64::sin(ET)) + (f64::powi(ET, 2) * f64::cos(ET)));
    } else if fstr::eq(&save.SVFNAM, b"D2((X**2)*SIN(X))") {
        //
        // This is the 2nd derivative of (X**2)*SIN(X).
        //
        //
        *VALUE = (((((2 as f64) * f64::sin(ET)) + (((2 as f64) * ET) * f64::cos(ET)))
            + (((2 as f64) * ET) * f64::cos(ET)))
            + (f64::powi(ET, 2) * -f64::sin(ET)));
    } else {
        spicelib::SETMSG(b"Bad function; function name is #.", ctx);
        spicelib::ERRCH(b"#", &save.SVFNAM, ctx);
        spicelib::SIGERR(b"SPICE(TSPICEBUG)", ctx)?;
    }

    spicelib::CHKOUT(b"T_ZZGFREL_GET", ctx)?;
    Ok(())
}

//
// Update reference value.
//
pub fn T_ZZGFREL_UP(REF: f64, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    save.SVREF = REF;
}

//
// Indicate whether function is less than reference value
// at specified epoch.
//
pub fn T_ZZGFREL_LT(ET: f64, LSSTHN: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut FVAL: f64 = 0.0;

    *LSSTHN = false;

    if fstr::eq(&save.SVFNAM, b"X") {
        FVAL = ET;
    } else if fstr::eq(&save.SVFNAM, b"SIN(X)") {
        FVAL = f64::sin(ET);
    } else if fstr::eq(&save.SVFNAM, b"(X**2)*SIN(X)") {
        FVAL = (f64::powi(ET, 2) * f64::sin(ET));
    } else if fstr::eq(&save.SVFNAM, b"D((X**2)*SIN(X))") {
        //
        // This is the derivative of (X**2)*SIN(X)
        //
        FVAL = ((((2 as f64) * ET) * f64::sin(ET)) + (f64::powi(ET, 2) * f64::cos(ET)));
    } else if fstr::eq(&save.SVFNAM, b"D2((X**2)*SIN(X))") {
        //
        // This is the 2nd derivative of (X**2)*SIN(X)
        //
        FVAL = (((((2 as f64) * f64::sin(ET)) + (((2 as f64) * ET) * f64::cos(ET)))
            + (((2 as f64) * ET) * f64::cos(ET)))
            + (f64::powi(ET, 2) * -f64::sin(ET)));
    } else {
        spicelib::SETMSG(b"Bad function; function name is #.", ctx);
        spicelib::ERRCH(b"#", &save.SVFNAM, ctx);
        spicelib::SIGERR(b"SPICE(TSPICEBUG)", ctx)?;
    }

    *LSSTHN = (FVAL < save.SVREF);

    Ok(())
}

//
// Indicate whether function is decreasing
// at specified epoch.
//
pub fn T_ZZGFREL_DEC(ET: f64, DECRES: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut DFVAL: f64 = 0.0;

    *DECRES = false;

    if fstr::eq(&save.SVFNAM, b"X") {
        DFVAL = 1.0;
    } else if fstr::eq(&save.SVFNAM, b"SIN(X)") {
        DFVAL = f64::cos(ET);
    } else if fstr::eq(&save.SVFNAM, b"(X**2)*SIN(X)") {
        DFVAL = ((((2 as f64) * ET) * f64::sin(ET)) + (f64::powi(ET, 2) * f64::cos(ET)));

    // WRITE (*,*) 'ET, DFVAL = ', ET, DFVAL
    } else if fstr::eq(&save.SVFNAM, b"D((X**2)*SIN(X))") {
        //
        // The function is the derivative of (X**2)*SIN(X).
        // DFVAL is the 2nd derivative of this function,
        // in other words the derivative of
        //
        //    2*X*SIN(X) +  (X**2)*COS(X)
        //
        DFVAL = (((((2 as f64) * f64::sin(ET)) + (((2 as f64) * ET) * f64::cos(ET)))
            + (((2 as f64) * ET) * f64::cos(ET)))
            + (f64::powi(ET, 2) * -f64::sin(ET)));
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
pub fn T_ZZGFREL_SET(F: &[u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    spicelib::LJUST(F, &mut save.SVFNAM);
    spicelib::UCASE(&save.SVFNAM.to_vec(), &mut save.SVFNAM, ctx);
}
