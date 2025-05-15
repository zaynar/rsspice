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
    SVCORR: Vec<u8>,
    SVOBS: i32,
    SVTARG: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SVCORR = vec![b' '; CORLEN as usize];
        let mut SVOBS: i32 = 0;
        let mut SVTARG: i32 = 0;

        Self {
            SVCORR,
            SVOBS,
            SVTARG,
        }
    }
}

//$Procedure ZZGFDIU ( Private --- GF, distance utilities )
pub fn ZZGFDIU(
    TARGET: &[u8],
    ABCORR: &[u8],
    OBSRVR: &[u8],
    UDFUNC: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    ET: f64,
    DECRES: bool,
    DIST: f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local Variables
    //

    //
    // Saved Variables
    //

    //
    // This routine should never be called directly.
    //
    CHKIN(b"ZZGFDIU", ctx)?;
    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    CHKOUT(b"ZZGFDIU", ctx)?;
    Ok(())
}

//$Procedure  ZZGFDIIN ( Private --- GF, distance initialization )
pub fn ZZGFDIIN(
    TARGET: &[u8],
    ABCORR: &[u8],
    OBSRVR: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ATTBLK = StackArray::<bool, 15>::new(1..=NABCOR);
    let mut FOUND: bool = false;

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFDIIN", ctx)?;

    //
    // Find NAIF IDs for TARGET and OBSRVR.
    //
    BODS2C(TARGET, &mut save.SVTARG, &mut FOUND, ctx)?;

    if !FOUND {
        SETMSG(b"The target object, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE Toolkit. ", ctx);
        ERRCH(b"#", TARGET, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(b"ZZGFDIIN", ctx)?;
        return Ok(());
    }

    BODS2C(OBSRVR, &mut save.SVOBS, &mut FOUND, ctx)?;

    if !FOUND {
        SETMSG(b"The observer, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE toolkit. ", ctx);
        ERRCH(b"#", OBSRVR, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(b"ZZGFDIIN", ctx)?;
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
        CHKOUT(b"ZZGFDIIN", ctx)?;
        return Ok(());
    }

    //
    // Squeeze all blanks out of the aberration correction
    // string; ensure the string is in upper case.
    //
    CMPRSS(b" ", 0, ABCORR, &mut save.SVCORR);
    UCASE(&save.SVCORR.to_vec(), &mut save.SVCORR, ctx);

    //
    // Check the aberration correction. If SPKEZR can't handle it,
    // neither can we.
    //
    ZZVALCOR(&save.SVCORR, ATTBLK.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFDIIN", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZGFDIIN", ctx)?;
    Ok(())
}

//$Procedure ZZGFDIDC ( Private --- GF, is distance decreasing? )
pub fn ZZGFDIDC(
    UDFUNC: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    ET: &mut f64,
    DECRES: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut LT: f64 = 0.0;
    let mut STATE = StackArray::<f64, 6>::new(1..=6);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFDIDC", ctx)?;

    SPKEZ(
        save.SVTARG,
        *ET,
        b"J2000",
        &save.SVCORR,
        save.SVOBS,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFDIDC", ctx)?;
        return Ok(());
    }

    //
    // The observer-target distance is decreasing if and only
    // if the dot product of the velocity and position is
    // negative.
    //
    *DECRES = (VDOT(STATE.as_slice(), STATE.subarray(4)) < 0.0);

    CHKOUT(b"ZZGFDIDC", ctx)?;
    Ok(())
}

//$Procedure ZZGFDIGQ ( Private --- GF, get observer-target distance )
pub fn ZZGFDIGQ(ET: &mut f64, DIST: &mut f64, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFDIGQ", ctx)?;

    ZZGFDIQ(save.SVTARG, *ET, &save.SVCORR, save.SVOBS, DIST, ctx)?;

    CHKOUT(b"ZZGFDIGQ", ctx)?;
    Ok(())
}
