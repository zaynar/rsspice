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
const SHAPLN: i32 = 6;
const SPSHPN: i32 = 2;
const POINT: i32 = 1;
const SPHERE: i32 = 2;

struct SaveVars {
    SVSHAP: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SVSHAP = ActualCharArray::new(32, 1..=SPSHPN);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"POINT"), Val::C(b"SPHERE")].into_iter();
            SVSHAP
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { SVSHAP }
    }
}

//$Procedure ZZSPIN ( Separation quantity initializer )
pub fn ZZSPIN(
    TARG1: &[u8],
    SHAPE1: &[u8],
    FRAME1: &[u8],
    TARG2: &[u8],
    SHAPE2: &[u8],
    FRAME2: &[u8],
    OBSRVR: &[u8],
    ABCORR: &[u8],
    BODS: &mut [i32],
    FRAMES: &mut [i32],
    MXRAD: &mut [f64],
    OBS: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut BODS = DummyArrayMut::new(BODS, 1..=2);
    let mut FRAMES = DummyArrayMut::new(FRAMES, 1..=2);
    let mut MXRAD = DummyArrayMut::new(MXRAD, 1..=2);
    let mut FOUND: bool = false;
    let mut SHAP1 = [b' '; SHAPLN as usize];
    let mut SHAP2 = [b' '; SHAPLN as usize];
    let mut AXES1 = StackArray::<f64, 3>::new(1..=3);
    let mut AXES2 = StackArray::<f64, 3>::new(1..=3);
    let mut CLASS: i32 = 0;
    let mut CLSSID: i32 = 0;
    let mut CTR1: i32 = 0;
    let mut CTR2: i32 = 0;
    let mut SHP1: i32 = 0;
    let mut SHP2: i32 = 0;
    let mut ATTBLK = StackArray::<bool, 15>::new(1..=NABCOR);

    //
    // SPICELIB functions.
    //

    //
    // Local parameters
    //

    //
    // Local Variables
    //

    //
    // Below we initialize the list of shape names.
    //

    //
    // Define integer ID parameters for the shape names in
    // SVSHAP.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZSPIN", ctx)?;

    //
    // FRAMES argument not currently used.
    //
    FRAMES[1] = -1;
    FRAMES[2] = -1;

    BODS2C(TARG1, &mut BODS[1], &mut FOUND, ctx)?;

    if !FOUND {
        SETMSG(b"The object name for target 1, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE Toolkit.", ctx);
        ERRCH(b"#", TARG1, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(b"ZZSPIN", ctx)?;
        return Ok(());
    }

    BODS2C(TARG2, &mut BODS[2], &mut FOUND, ctx)?;

    if !FOUND {
        SETMSG(b"The object name for target 2, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE Toolkit.", ctx);
        ERRCH(b"#", TARG2, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(b"ZZSPIN", ctx)?;
        return Ok(());
    }

    BODS2C(OBSRVR, OBS, &mut FOUND, ctx)?;

    if !FOUND {
        SETMSG(b"The object name for the observer, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE Toolkit.", ctx);
        ERRCH(b"#", OBSRVR, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(b"ZZSPIN", ctx)?;
        return Ok(());
    }

    //
    // Confirm the three bodies have unique IDs.
    //
    if (((*OBS == BODS[1]) || (*OBS == BODS[2])) || (BODS[1] == BODS[2])) {
        SETMSG(b"All three objects associated with an ANGULAR SEPARATION calculation must be distinct. The objects whose angular separation is of interest were # and #. The observer was #.", ctx);
        ERRINT(b"#", BODS[1], ctx);
        ERRINT(b"#", BODS[2], ctx);
        ERRINT(b"#", *OBS, ctx);
        SIGERR(b"SPICE(BODIESNOTDISTINCT)", ctx)?;
        CHKOUT(b"ZZSPIN", ctx)?;
        return Ok(());
    }

    //
    // Check the aberration correction. If SPKEZR can't handle it,
    // neither can we.
    //
    ZZVALCOR(ABCORR, ATTBLK.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZSPIN", ctx)?;
        return Ok(());
    }

    //
    // Check first shape.
    //
    LJUCRS(1, SHAPE1, &mut SHAP1, ctx);

    //
    // If we pass the error check, then SHAPE1 exists in SVSHAP.
    //
    SHP1 = ISRCHC(&SHAP1, SPSHPN, save.SVSHAP.as_arg());

    if (SHP1 == 0) {
        SETMSG(
            b"The body shape, # is not recognized.  Supported quantities are: POINT, SPHERE.",
            ctx,
        );
        ERRCH(b"#", &SHAP1, ctx);
        SIGERR(b"SPICE(NOTRECOGNIZED)", ctx)?;
        CHKOUT(b"ZZSPIN", ctx)?;
        return Ok(());
    } else if (SHP1 == POINT) {
        MXRAD[1] = 0.0;
    } else if (SHP1 == SPHERE) {
        ZZGFTREB(BODS[1], AXES1.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZSPIN", ctx)?;
            return Ok(());
        }

        MXRAD[1] = intrinsics::DMAX1(&[AXES1[1], AXES1[2], AXES1[3]]);
    } else {
        //
        // This code executes only if someone adds a new shape
        // name to SVSHAP then fails to update the SHP1 condition
        // block to respond to the name. Fortran needs SWITCH...CASE.
        //
        SETMSG(
            b"Encountered uncoded shape ID for #. This indicates a bug. Please contact NAIF.",
            ctx,
        );
        ERRCH(b"#", &SHAP1, ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZSPIN", ctx)?;
        return Ok(());
    }

    //
    // Check second shape.
    //
    LJUCRS(1, SHAPE2, &mut SHAP2, ctx);

    //
    // If we pass the error check, then SHAPE2 exists in SVSHAP.
    //
    SHP2 = ISRCHC(&SHAP2, SPSHPN, save.SVSHAP.as_arg());

    if (SHP2 == 0) {
        SETMSG(
            b"The body shape, # is not recognized.  Supported quantities are: POINT, SPHERE.",
            ctx,
        );
        ERRCH(b"#", &SHAP2, ctx);
        SIGERR(b"SPICE(NOTRECOGNIZED)", ctx)?;
        CHKOUT(b"ZZSPIN", ctx)?;
        return Ok(());
    } else if (SHP2 == POINT) {
        MXRAD[2] = 0.0;
    } else if (SHP2 == SPHERE) {
        ZZGFTREB(BODS[2], AXES2.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZSPIN", ctx)?;
            return Ok(());
        }

        MXRAD[2] = intrinsics::DMAX1(&[AXES2[1], AXES2[2], AXES2[3]]);
    } else {
        //
        // This code executes only if someone adds a new shape
        // name to SVSHAP then fails to update the SHP2 condition
        // block to respond to the name. Fortran needs SWITCH...CASE.
        //
        SETMSG(
            b"Encountered uncoded shape ID for #. This indicates a bug. Please contact NAIF.",
            ctx,
        );
        ERRCH(b"#", &SHAP2, ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZSPIN", ctx)?;
        return Ok(());
    }

    //
    // Confirm the center of the input reference frames correspond
    // to the target bodies for non-point, non-spherical bodies.
    //
    //    FRAME1 centered on TARG1
    //    FRAME2 centered on TARG2
    //
    // This check does not apply to POINT or SPHERE shapes.
    //

    if ((SHP1 != POINT) && (SHP1 != SPHERE)) {
        NAMFRM(FRAME1, &mut FRAMES[1], ctx)?;

        FRINFO(
            FRAMES[1],
            &mut CTR1,
            &mut CLASS,
            &mut CLSSID,
            &mut FOUND,
            ctx,
        )?;

        if !FOUND {
            SETMSG(b"Frame system did not recognize frame #.", ctx);
            ERRCH(b"#", FRAME1, ctx);
            SIGERR(b"SPICE(NOFRAME)", ctx)?;
            CHKOUT(b"ZZSPIN", ctx)?;
            return Ok(());
        }

        if (BODS[1] != CTR1) {
            SETMSG(b"The reference frame #1 associated with target body #2 is not centered on #2. The frame must be centered on the target body.", ctx);

            ERRCH(b"#1", FRAME1, ctx);
            ERRCH(b"#2", TARG1, ctx);
            SIGERR(b"SPICE(INVALIDFRAME)", ctx)?;
            CHKOUT(b"ZZSPIN", ctx)?;
            return Ok(());
        }
    }

    if ((SHP2 != POINT) && (SHP2 != SPHERE)) {
        NAMFRM(FRAME2, &mut FRAMES[2], ctx)?;

        FRINFO(
            FRAMES[2],
            &mut CTR2,
            &mut CLASS,
            &mut CLSSID,
            &mut FOUND,
            ctx,
        )?;

        if !FOUND {
            SETMSG(b"Frame system did not recognize frame #.", ctx);
            ERRCH(b"#", FRAME2, ctx);
            SIGERR(b"SPICE(NOFRAME)", ctx)?;
            CHKOUT(b"ZZSPIN", ctx)?;
            return Ok(());
        }

        if (BODS[2] != CTR2) {
            SETMSG(b"The reference frame #1 associated with target body #2 is not centered on #2. The frame must be centered on the target body.", ctx);

            ERRCH(b"#1", FRAME2, ctx);
            ERRCH(b"#2", TARG2, ctx);
            SIGERR(b"SPICE(INVALIDFRAME)", ctx)?;
            CHKOUT(b"ZZSPIN", ctx)?;
            return Ok(());
        }
    }

    //
    // Done. Check out. Return.
    //

    CHKOUT(b"ZZSPIN", ctx)?;
    Ok(())
}
