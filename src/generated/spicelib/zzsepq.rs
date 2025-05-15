//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ATOL: f64 = 0.000000000001;

//$Procedure ZZSEPQ ( Separation quantity from observer )
pub fn ZZSEPQ(
    ET: f64,
    BOD1: i32,
    BOD2: i32,
    R1: f64,
    R2: f64,
    OBS: i32,
    ABCORR: &[u8],
    REF: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<f64> {
    let mut ZZSEPQ: f64 = 0.0;
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
    // Set an initial value to return in case of error.
    //
    ZZSEPQ = 0.0;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(ZZSEPQ);
    }

    CHKIN(b"ZZSEPQ", ctx)?;

    //
    // First check for bad inputs.
    //
    if ((R1 < 0.0) || (R2 < 0.0)) {
        SETMSG(b"A negative radius for a body was encountered. The radius for body #1 was given as #2, the radius of body #3 was given as #4.", ctx);

        ERRINT(b"#1", BOD1, ctx);
        ERRDP(b"#2", R1, ctx);
        ERRINT(b"#3", BOD2, ctx);
        ERRDP(b"#4", R2, ctx);
        SIGERR(b"SPICE(BADRADIUS)", ctx)?;
        CHKOUT(b"ZZSEPQ", ctx)?;
        return Ok(ZZSEPQ);
    }

    //
    // Get the state of the BOD1, BOD2 objects relative to OBS.
    //
    SPKEZP(BOD1, ET, REF, ABCORR, OBS, PV1.as_slice_mut(), &mut LT, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZSEPQ", ctx)?;
        return Ok(ZZSEPQ);
    }

    SPKEZP(BOD2, ET, REF, ABCORR, OBS, PV2.as_slice_mut(), &mut LT, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZSEPQ", ctx)?;
        return Ok(ZZSEPQ);
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
            CHKOUT(b"ZZSEPQ", ctx)?;
            return Ok(ZZSEPQ);
        }
    } else {
        SETMSG(b"Observer object #1 located within surface of target 1 object #2. Range to target 1 #3, radius of target 1 #4", ctx);

        ERRINT(b"#1", OBS, ctx);
        ERRINT(b"#2", BOD1, ctx);
        ERRDP(b"#3", RANGE1, ctx);
        ERRDP(b"#4", R1, ctx);
        SIGERR(b"SPICE(INSIDEBODY)", ctx)?;
        CHKOUT(b"ZZSEPQ", ctx)?;
        return Ok(ZZSEPQ);
    }

    if (RANGE2 > R2) {
        ANG2 = DASINE((R2 / RANGE2), ATOL, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZSEPQ", ctx)?;
            return Ok(ZZSEPQ);
        }
    } else {
        SETMSG(b"Observer object #1 located within surface of target 2 object #2. Range to target 2 #3, radius of target 2 #4", ctx);

        ERRINT(b"#1", OBS, ctx);
        ERRINT(b"#2", BOD2, ctx);
        ERRDP(b"#3", RANGE2, ctx);
        ERRDP(b"#4", R2, ctx);
        SIGERR(b"SPICE(INSIDEBODY)", ctx)?;
        CHKOUT(b"ZZSEPQ", ctx)?;
        return Ok(ZZSEPQ);
    }

    //
    // Finally compute the apparent separation.
    //
    THETA = VSEP(PV1.as_slice(), PV2.as_slice(), ctx);
    ZZSEPQ = ((THETA - ANG1) - ANG2);

    CHKOUT(b"ZZSEPQ", ctx)?;
    Ok(ZZSEPQ)
}
