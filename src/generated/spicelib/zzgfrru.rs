//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;

struct SaveVars {
    SVABCO: Vec<u8>,
    SVTARG: i32,
    SVREF: Vec<u8>,
    SVOBS: i32,
    SVDT: f64,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SVABCO = vec![b' '; CORLEN as usize];
        let mut SVTARG: i32 = 0;
        let mut SVREF = vec![b' '; 32 as usize];
        let mut SVOBS: i32 = 0;
        let mut SVDT: f64 = 0.0;

        Self {
            SVABCO,
            SVTARG,
            SVREF,
            SVOBS,
            SVDT,
        }
    }
}

//$Procedure ZZGFRRU ( Private --- GF, range rate utility routine )
pub fn ZZGFRRU(
    TARGET: &[u8],
    ABCORR: &[u8],
    OBSRVR: &[u8],
    DT: f64,
    UDFUNC: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    ET: f64,
    DECRES: bool,
    RVL: f64,
    XTARG: i32,
    XABCOR: &[u8],
    XOBS: i32,
    XDT: f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Local Variables
    //

    //
    // Saved Variables
    //

    CHKIN(b"ZZGFRRU", ctx)?;
    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    CHKOUT(b"ZZGFRRU", ctx)?;

    Ok(())
}

//$Procedure ZZGFRRIN ( Private --- GF, range rate initialization )
pub fn ZZGFRRIN(
    TARGET: &[u8],
    ABCORR: &[u8],
    OBSRVR: &[u8],
    DT: f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ATTBLK = StackArray::<bool, 15>::new(1..=NABCOR);
    let mut FOUND: bool = false;

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFRRIN", ctx)?;

    //
    // Find NAIF IDs for TARGET and OBSRVR.
    //
    BODS2C(TARGET, &mut save.SVTARG, &mut FOUND, ctx)?;

    if !FOUND {
        SETMSG(b"The target object, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE Toolkit. ", ctx);
        ERRCH(b"#", TARGET, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(b"ZZGFRRIN", ctx)?;
        return Ok(());
    }

    BODS2C(OBSRVR, &mut save.SVOBS, &mut FOUND, ctx)?;

    if !FOUND {
        SETMSG(b"The observer, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE toolkit. ", ctx);
        ERRCH(b"#", OBSRVR, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(b"ZZGFRRIN", ctx)?;
        return Ok(());
    }

    //
    // Make sure the observer and target are distinct.
    //
    if (save.SVTARG == save.SVOBS) {
        SETMSG(b"The observer and target must be distinct objects, but are not: OBSRVR = #; TARGET = #.", ctx);
        ERRCH(b"#", OBSRVR, ctx);
        ERRCH(b"#", TARGET, ctx);
        SIGERR(b"SPICE(BODIESNOTDISTINCT)", ctx)?;
        CHKOUT(b"ZZGFRRIN", ctx)?;
        return Ok(());
    }

    //
    // Squeeze all blanks out of the aberration correction
    // string; ensure the string is in upper case.
    //
    CMPRSS(b" ", 0, ABCORR, &mut save.SVABCO);
    UCASE(&save.SVABCO.to_vec(), &mut save.SVABCO, ctx);

    //
    // Check the aberration correction. If SPKEZR can't handle it,
    // neither can we.
    //
    ZZVALCOR(&save.SVABCO, ATTBLK.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFRRIN", ctx)?;
        return Ok(());
    }

    //
    // The "delta" argument for QDERIV, DT, must have a non-zero value.
    //
    if (DT == 0.0) {
        SETMSG(
            b"Delta value for QDERIV is zero; a non-zero value is required.",
            ctx,
        );
        SIGERR(b"SPICE(INVALIDVALUE)", ctx)?;
        CHKOUT(b"ZZGFRRIN", ctx)?;
        return Ok(());
    }

    //
    // Save the DT value.
    //
    fstr::assign(&mut save.SVREF, b"J2000");
    save.SVDT = DT;

    CHKOUT(b"ZZGFRRIN", ctx)?;
    Ok(())
}

//$Procedure ZZGFRRDC (  Private --- GF, when range rate is decreasing )
pub fn ZZGFRRDC(
    UDFUNC: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    ET: &mut f64,
    DECRES: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut LT: f64 = 0.0;
    let mut DRVEL: f64 = 0.0;
    let mut SRHAT = StackArray::<f64, 6>::new(1..=6);
    let mut STATE = StackArray::<f64, 6>::new(1..=6);
    let mut DFDT = StackArray::<f64, 6>::new(1..=6);
    let mut STATES = StackArray2D::<f64, 12>::new(1..=6, 1..=2);
    let mut N: i32 = 0;

    //
    // Standard SPICE error handling.
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFRRDC", ctx)?;

    N = 6;

    //
    // The range rate of interest is of SVTARG relative to the SVOBS.
    // The function requires the acceleration of SVTARG relative
    // to SVOBS.
    //

    SPKEZ(
        save.SVTARG,
        (*ET - save.SVDT),
        &save.SVREF,
        &save.SVABCO,
        save.SVOBS,
        STATES.subarray_mut([1, 1]),
        &mut LT,
        ctx,
    )?;
    SPKEZ(
        save.SVTARG,
        (*ET + save.SVDT),
        &save.SVREF,
        &save.SVABCO,
        save.SVOBS,
        STATES.subarray_mut([1, 2]),
        &mut LT,
        ctx,
    )?;

    //
    // Approximate the derivative of the position and velocity by
    // finding the derivative of a quadratic approximating function.
    //
    //    DFDT(1) = Vx
    //    DFDT(2) = Vy
    //    DFDT(3) = Vz
    //    DFDT(4) = Ax
    //    DFDT(5) = Ay
    //    DFDT(6) = Az
    //
    QDERIV(
        N,
        STATES.subarray([1, 1]),
        STATES.subarray([1, 2]),
        save.SVDT,
        DFDT.as_slice_mut(),
        ctx,
    )?;

    SPKEZ(
        save.SVTARG,
        *ET,
        &save.SVREF,
        &save.SVABCO,
        save.SVOBS,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFRRDC", ctx)?;
        return Ok(());
    }

    //
    // d ||r||     ^
    // ------- = < r, v >
    // dt
    //
    //  2           ^          ^
    // d ||r||   < dr, v > + < r, dv >
    // ------- =   --             --
    //   2
    // dt          dt             dt
    //
    DVHAT(STATE.as_slice(), SRHAT.as_slice_mut());

    DRVEL = (VDOT(DFDT.subarray(4), SRHAT.as_slice()) + VDOT(STATE.subarray(4), SRHAT.subarray(4)));
    *DECRES = (DRVEL < 0.0);

    CHKOUT(b"ZZGFRRDC", ctx)?;
    Ok(())
}

//$Procedure ZZGFRRGQ ( Private --- GF, get range rate between bodies )
pub fn ZZGFRRGQ(ET: &mut f64, RVL: &mut f64, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    ZZGFRRQ(*ET, save.SVTARG, save.SVOBS, &save.SVABCO, RVL, ctx)?;

    Ok(())
}

//$Procedure ZZGFRRX ( Private -- GF, retrieve ZZGFRRIN values )
pub fn ZZGFRRX(
    XTARG: &mut i32,
    XABCOR: &mut [u8],
    XOBS: &mut i32,
    XDT: &mut f64,
    ctx: &mut Context,
) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    *XTARG = save.SVTARG;
    fstr::assign(XABCOR, &save.SVABCO);
    *XOBS = save.SVOBS;
    *XDT = save.SVDT;
}
