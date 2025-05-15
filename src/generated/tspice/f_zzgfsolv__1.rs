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

//
// ZZGFSOLV test family specific utility code.
//
pub fn T_ZZGFSOLV(
    ET: f64,
    IN_CON: bool,
    VAL: f64,
    IVBEG: f64,
    IVEND: f64,
    NVAL: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    spicelib::CHKIN(b"T_ZZGFSOLV", ctx)?;
    spicelib::SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    spicelib::CHKOUT(b"T_ZZGFSOLV", ctx)?;

    Ok(())
}

//
// A UDCOND test function. The function returns FALSE
// for ET <= CRIT and ET > 2*CRIT.
//
//
pub fn T_UDQLT(ET: f64, IN_CON: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    *IN_CON = false;

    //
    // The function returns true after the CRIT value.
    //
    if (ET > save.CRIT) {
        *IN_CON = true;
    }

    //
    // The function returns false after the 2*CRIT value.
    //
    if (ET > (2.0 * save.CRIT)) {
        *IN_CON = false;
    }

    Ok(())
}

//
// Set the critical value used in the tests.
//
pub fn ZZCRITS(VAL: f64, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    save.CRIT = VAL;
}

//
// Return the critical value used in the tests.TOL
//
pub fn ZZCRITG(VAL: &mut f64, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    *VAL = save.CRIT;
}

//
// A UDREPU test function.
//
pub fn T_REPU(IVBEG: f64, IVEND: f64, ET: f64, ctx: &mut Context) -> f2rust_std::Result<()> {
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
pub fn ZZNRPTG(NVAL: &mut i32, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    *NVAL = save.NRPT;
    save.NRPT = 0;
}

//
// Initialize the progress reporter count to zero.
//
pub fn ZZRPTI(ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    save.NRPT = 0;
}
