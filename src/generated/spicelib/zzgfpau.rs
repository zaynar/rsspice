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
const REF: &[u8] = b"J2000";

struct SaveVars {
    SVABCO: Vec<u8>,
    SVILLM: i32,
    SVOBS: i32,
    SVTARG: i32,
    SVABLK: StackArray<bool, 15>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SVABCO = vec![b' '; CORLEN as usize];
        let mut SVILLM: i32 = 0;
        let mut SVOBS: i32 = 0;
        let mut SVTARG: i32 = 0;
        let mut SVABLK = StackArray::<bool, 15>::new(1..=NABCOR);

        Self {
            SVABCO,
            SVILLM,
            SVOBS,
            SVTARG,
            SVABLK,
        }
    }
}

//$Procedure ZZGFPAU ( Private --- GF, phase angle utility routine )
pub fn ZZGFPAU(
    TARGET: &[u8],
    ILLMN: &[u8],
    ABCORR: &[u8],
    OBSRVR: &[u8],
    UDFUNC: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    ET: f64,
    DECRES: bool,
    RVL: f64,
    XTARG: i32,
    XILLMN: i32,
    XABCOR: &[u8],
    XOBS: i32,
    XABLK: &[bool],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // The phase angle calculation is invariant with respect to
    // reference frame. Use J2000 for convenience.
    //

    //
    // Local Variables
    //

    //
    // Saved Variables
    //

    CHKIN(b"ZZGFPAU", ctx)?;
    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    CHKOUT(b"ZZGFPAU", ctx)?;

    Ok(())
}

//$Procedure  ZZGFPAIN ( Private --- GF, phase angle initialization )
pub fn ZZGFPAIN(
    TARGET: &[u8],
    ILLMN: &[u8],
    ABCORR: &[u8],
    OBSRVR: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut FOUND: bool = false;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFPAIN", ctx)?;

    //
    // Find NAIF IDs for TARGET, ILLMN, and OBSRVR.
    //
    BODS2C(TARGET, &mut save.SVTARG, &mut FOUND, ctx)?;

    if !FOUND {
        SETMSG(b"The target object, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE Toolkit. ", ctx);
        ERRCH(b"#", TARGET, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(b"ZZGFPAIN", ctx)?;
        return Ok(());
    }

    BODS2C(ILLMN, &mut save.SVILLM, &mut FOUND, ctx)?;

    if !FOUND {
        SETMSG(b"The illuminator object, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE toolkit. ", ctx);
        ERRCH(b"#", OBSRVR, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(b"ZZGFPAIN", ctx)?;
        return Ok(());
    }

    BODS2C(OBSRVR, &mut save.SVOBS, &mut FOUND, ctx)?;

    if !FOUND {
        SETMSG(b"The observer object, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE toolkit. ", ctx);
        ERRCH(b"#", OBSRVR, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(b"ZZGFPAIN", ctx)?;
        return Ok(());
    }

    //
    // Make sure the observer, illuminator, and target are distinct.
    //
    if (((save.SVTARG == save.SVOBS) || (save.SVTARG == save.SVILLM))
        || (save.SVOBS == save.SVILLM))
    {
        SETMSG(b"The observer, illuminator, and target must be distinct objects, but are not: OBSRVR = #, TARGET = #, are not: ILLMN= #.", ctx);
        ERRCH(b"#", OBSRVR, ctx);
        ERRCH(b"#", TARGET, ctx);
        ERRCH(b"#", ILLMN, ctx);
        SIGERR(b"SPICE(BODIESNOTDISTINCT)", ctx)?;
        CHKOUT(b"ZZGFPAIN", ctx)?;
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
    ZZVALCOR(&save.SVABCO, save.SVABLK.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFPAIN", ctx)?;
        return Ok(());
    }

    //
    // Restrict correction to reception cases.
    //

    if save.SVABLK[XMTIDX] {
        SETMSG(b"Invalid aberration correction \'#\'. Phase angle geometry calculations currently restricted to reception cases.", ctx);
        ERRCH(b"#", ABCORR, ctx);
        SIGERR(b"SPICE(INVALIDOPTION)", ctx)?;
        CHKOUT(b"ZZGFPAIN", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZGFPAIN", ctx)?;
    Ok(())
}

//$Procedure ZZGFPADC ( Private --- GF, when phase angle is decreasing )
pub fn ZZGFPADC(
    UDFUNC: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    ET: &mut f64,
    DECRES: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut DSEP: f64 = 0.0;
    let mut LT: f64 = 0.0;
    let mut DLT: f64 = 0.0;
    let mut S1 = StackArray::<f64, 6>::new(1..=6);
    let mut S2 = StackArray::<f64, 6>::new(1..=6);
    let mut UVEC = StackArray::<f64, 3>::new(1..=3);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFPADC", ctx)?;

    //
    // Get the state of the TARG object relative to OBS at ET.
    //
    SPKEZ(
        save.SVTARG,
        *ET,
        REF,
        &save.SVABCO,
        save.SVOBS,
        S1.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFPADC", ctx)?;
        return Ok(());
    }

    //
    // Get the state of the ILLMN object relative to TARG at ET
    // for no aberration correction, or ET - LT otherwise.
    //
    if save.SVABLK[GEOIDX] {
        //
        // No correction, geometric.
        //
        SPKEZ(
            save.SVILLM,
            *ET,
            REF,
            &save.SVABCO,
            save.SVTARG,
            S2.as_slice_mut(),
            &mut LT,
            ctx,
        )?;
    } else {
        SPKEZ(
            save.SVILLM,
            (*ET - LT),
            REF,
            &save.SVABCO,
            save.SVTARG,
            S2.as_slice_mut(),
            &mut LT,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZGFPADC", ctx)?;
            return Ok(());
        }

        //
        // Correct velocity for time derivative of the observer target
        // light-time. We need to do this since the SPK evaluation occurs
        // at ET - LT.
        //
        // d( ET - LT ) = (1 - d LT )
        // ------------        ----
        // dt                  dt
        //
        // LT = ||R||
        //      -----
        //        C
        //                              ^
        // d LT    = 1   d ||R||  = 1 < R, V >
        // ----      -   -------    -
        // dt        C   dt         C
        //
        //
        VHAT(S1.as_slice(), UVEC.as_slice_mut());

        DLT = (VDOT(UVEC.as_slice(), S1.subarray(4)) / CLIGHT());

        //
        // Apply the correction to the velocity vector components.
        //
        VSCLIP((1.0 - DLT), S2.subarray_mut(4));
    }

    if FAILED(ctx) {
        CHKOUT(b"ZZGFPADC", ctx)?;
        return Ok(());
    }

    //
    //                   ILLMN      OBS
    //   ILLMN as seen      *       /
    //   from TARG at       |      /
    //   ET - LT.           |     /
    //                     >|..../< phase angle
    //                      |   /
    //                    . |  /
    //                  .   | /
    //                 .     *     TARG as seen from OBS
    //           SEP   .   TARG    at ET
    //                  .  /
    //                    /
    //                   *
    //
    // Calculate the derivative of the angle separating the vectors
    // relative to TARG.
    //
    //    PI = SEP + PHASE
    //
    //    dPHASE     dSEP
    //    ------ = - ----
    //     dt         dt
    //
    DSEP = DVSEP(S1.as_slice(), S2.as_slice(), ctx)?;

    *DECRES = (-DSEP < 0.0);

    CHKOUT(b"ZZGFPADC", ctx)?;
    Ok(())
}

//$Procedure ZZGFPAGQ ( Private --- GF, phase angle between two bodies )
pub fn ZZGFPAGQ(ET: &mut f64, RVL: &mut f64, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    ZZGFPAQ(
        *ET,
        save.SVTARG,
        save.SVILLM,
        save.SVOBS,
        &save.SVABCO,
        RVL,
        ctx,
    )?;

    Ok(())
}

//$Procedure ZZGFPAX ( Private -- GF, retrieve ZZGFPAIN values )
pub fn ZZGFPAX(
    XTARG: &mut i32,
    XILLMN: &mut i32,
    XABCOR: &mut [u8],
    XOBS: &mut i32,
    XABLK: &mut [bool],
    ctx: &mut Context,
) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut XABLK = DummyArrayMut::new(XABLK, 1..=NABCOR);

    *XTARG = save.SVTARG;
    *XILLMN = save.SVILLM;
    fstr::assign(XABCOR, &save.SVABCO);
    *XOBS = save.SVOBS;

    for I in 1..=ABATSZ {
        XABLK[I] = save.SVABLK[I];
    }
}
