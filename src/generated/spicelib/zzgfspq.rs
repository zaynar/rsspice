//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ATOL: f64 = 0.000000000001;

//$Procedure ZZGFSPQ ( GF, separation quantity )
pub fn ZZGFSPQ(
    ET: f64,
    TARG1: i32,
    TARG2: i32,
    R1: f64,
    R2: f64,
    OBS: i32,
    ABCORR: &[u8],
    REF: &[u8],
    VALUE: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut ANG1: f64 = 0.0;
    let mut ANG2: f64 = 0.0;
    let mut PV1 = StackArray::<f64, 3>::new(1..=3);
    let mut PV2 = StackArray::<f64, 3>::new(1..=3);
    let mut RANGE1: f64 = 0.0;
    let mut RANGE2: f64 = 0.0;
    let mut THETA: f64 = 0.0;
    let mut LT: f64 = 0.0;

    //
    // SPICELIB functions.
    //

    //
    // Local Variables.
    //

    //
    // ATOL is a tolerance value for computing arc sine.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFSPQ", ctx)?;

    //
    // First check for bad inputs.
    //
    if ((R1 < 0.0) || (R2 < 0.0)) {
        SETMSG(b"A negative radius for a body was encountered. The radius for body # was given as #, the radius of body # was given as #. ", ctx);

        ERRINT(b"#", TARG1, ctx);
        ERRDP(b"#", R1, ctx);
        ERRINT(b"#", TARG2, ctx);
        ERRDP(b"#", R2, ctx);
        SIGERR(b"SPICE(BADRADIUS)", ctx)?;
        CHKOUT(b"ZZGFSPQ", ctx)?;
        return Ok(());
    }

    //
    // Get the state of the TARG1, TARG2 objects relative to OBS.
    //
    SPKEZP(
        TARG1,
        ET,
        REF,
        ABCORR,
        OBS,
        PV1.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFSPQ", ctx)?;
        return Ok(());
    }

    SPKEZP(
        TARG2,
        ET,
        REF,
        ABCORR,
        OBS,
        PV2.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFSPQ", ctx)?;
        return Ok(());
    }

    //
    // Compute the range to the objects of interest.
    //
    RANGE1 = VNORM(PV1.as_slice());
    RANGE2 = VNORM(PV2.as_slice());

    //
    // Compute the apparent angular radii as seen from OBS.
    //
    if (RANGE1 > R1) {
        ANG1 = DASINE((R1 / RANGE1), ATOL, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZGFSPQ", ctx)?;
            return Ok(());
        }
    } else {
        ANG1 = HALFPI(ctx);
    }

    if (RANGE2 > R2) {
        ANG2 = DASINE((R2 / RANGE2), ATOL, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZGFSPQ", ctx)?;
            return Ok(());
        }
    } else {
        ANG2 = HALFPI(ctx);
    }

    //
    // Finally compute the apparent separation.
    //
    THETA = VSEP(PV1.as_slice(), PV2.as_slice(), ctx);
    *VALUE = ((THETA - ANG1) - ANG2);

    CHKOUT(b"ZZGFSPQ", ctx)?;
    Ok(())
}
