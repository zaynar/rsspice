//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      T_RANDD ( Random double precision number )
pub fn T_RANDD(
    LOWER: f64,
    UPPER: f64,
    SEED: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<f64> {
    let mut T_RANDD: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Other functions
    //

    //
    // Give the function an initial value.
    //
    T_RANDD = LOWER;

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(T_RANDD);
    } else {
        spicelib::CHKIN(b"T_RANDD", ctx)?;
    }

    //
    // Check bounds.
    //
    if (LOWER > UPPER) {
        spicelib::SETMSG(b"Lower, upper bounds are:  #  #. ", ctx);
        spicelib::ERRDP(b"#", LOWER, ctx);
        spicelib::ERRDP(b"#", UPPER, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDBOUNDS)", ctx)?;
        spicelib::CHKOUT(b"T_RANDD", ctx)?;
        return Ok(T_RANDD);
    }

    //
    // Get a random number in the range [LOWER, UPPER].  The
    // T_URAND function returns numbers in the range [0, 1].
    //
    T_RANDD = (LOWER + ((UPPER - LOWER) * T_URAND(SEED, ctx)));

    spicelib::CHKOUT(b"T_RANDD", ctx)?;
    Ok(T_RANDD)
}
