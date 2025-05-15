//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    CRIT: f64,
    NRPT: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CRIT: f64 = 0.0;
        let mut NRPT: i32 = 0;

        NRPT = 0;

        Self { CRIT, NRPT }
    }
}

//$Procedure T_SOLVX ( Test utilities )
pub fn T_SOLVX(
    UDFUNCX: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    ET: f64,
    IN_CON: bool,
    VAL: f64,
    IVBEG: f64,
    IVEND: f64,
    NVAL: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    spicelib::CHKIN(b"T_SOLVX", ctx)?;
    spicelib::SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    spicelib::CHKOUT(b"T_SOLVX", ctx)?;

    Ok(())
}

//
// A UDCOND test function. The function returns FALSE
// for ET <= CRIT and ET > 2*CRIT.
//
//
pub fn T_UDQLTX(
    UDFUNCX: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    ET: &mut f64,
    IN_CON: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut X: f64 = 0.0;

    *IN_CON = false;

    UDFUNCX(ET, &mut X, ctx)?;

    //
    // The function returns true after the CRIT value.
    //
    if (X > save.CRIT) {
        *IN_CON = true;
    }

    //
    // The function returns false after the 2*CRIT value.
    //
    if (X > (2.0 * save.CRIT)) {
        *IN_CON = false;
    }

    Ok(())
}

//
// Set the critical value used in the tests.
//
pub fn ZZCRITSX(VAL: f64, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    save.CRIT = VAL;
}

//
// Return the critical value used in the tests.
//
pub fn ZZCRITGX(VAL: &mut f64, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    *VAL = save.CRIT;
}

//
// A UDREPU test function.
//
pub fn T_REPUX(IVBEG: f64, IVEND: f64, ET: f64, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Count the number of progress report calls.
    //
    save.NRPT = (save.NRPT + 1);

    Ok(())
}

//
// Return the number of progress report calls then reset the
// count variable to zero.
//
pub fn ZZNRPTGX(NVAL: &mut i32, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    *NVAL = save.NRPT;
    save.NRPT = 0;
}

pub fn ZZRPTIX(ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    save.NRPT = 0;
}
