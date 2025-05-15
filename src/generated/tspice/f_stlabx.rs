//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const TOL: f64 = 0.00000000000001;

//$Procedure  F_STLABX ( Family of tests for STLABX )
pub fn F_STLABX(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ANGLE: f64 = 0.0;
    let mut AXIS = StackArray::<f64, 3>::new(1..=3);
    let mut CORPOS = StackArray::<f64, 3>::new(1..=3);
    let mut OBSVEL = StackArray::<f64, 3>::new(1..=3);
    let mut SAVEC = StackArray::<f64, 3>::new(1..=3);
    let mut TRGPOS = StackArray::<f64, 3>::new(1..=3);
    let mut EXPPOS = StackArray::<f64, 3>::new(1..=3);

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
    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_STLABX", ctx)?;

    //
    // Case 1.
    //
    testutil::TCASE(b"Check error handling for excessive speed.", ctx)?;

    spicelib::VPACK(
        300000000.0,
        200000000.0,
        -400000000.0,
        TRGPOS.as_slice_mut(),
    );
    spicelib::VPACK(
        10000000000.0,
        1000000000.0,
        10000000000.0,
        OBSVEL.as_slice_mut(),
    );

    spicelib::STLABX(
        TRGPOS.as_slice(),
        OBSVEL.as_slice(),
        CORPOS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // Case 2.
    //
    testutil::TCASE(b"Try a routine computation.", ctx)?;

    //
    // Set up a reasonable velocity vector.
    //
    spicelib::VPACK(10.0, 3.0, 9.0, OBSVEL.as_slice_mut());

    //
    // Get the transmission stellar aberration correction from
    // STLABX.
    //
    spicelib::STLABX(
        TRGPOS.as_slice(),
        OBSVEL.as_slice(),
        CORPOS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Obtain the target position vector, corrected for the usual
    // stellar aberration effect.
    //
    spicelib::STELAB(
        TRGPOS.as_slice(),
        OBSVEL.as_slice(),
        SAVEC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Find the rotation axis about which POJB was rotated to
    // obtain SAVEC.  Find the angular separation between TRGPOS
    // and SAVEC as well.
    //
    spicelib::UCRSS(TRGPOS.as_slice(), SAVEC.as_slice(), AXIS.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    ANGLE = spicelib::VSEP(TRGPOS.as_slice(), SAVEC.as_slice(), ctx);

    //
    // The corrected vector we seek is obtained by rotating TRGPOS
    // about AXIS in the opposite direction required to obtain
    // SAVEC.
    //
    spicelib::VROTV(
        TRGPOS.as_slice(),
        AXIS.as_slice(),
        -ANGLE,
        EXPPOS.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"CORPOS",
        CORPOS.as_slice(),
        b"~~/",
        EXPPOS.as_slice(),
        3,
        TOL,
        OK,
        ctx,
    )?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
