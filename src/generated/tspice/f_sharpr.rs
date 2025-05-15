//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const AMP: f64 = 0.01;
const DTOL: f64 = 0.00000000000001;
const NTOL: f64 = 0.00000000000001;
const MSGLEN: i32 = 240;
const NUMCAS: i32 = 1000;

//$Procedure F_SHARPR ( Test the SPICELIB routine SHARPR )
pub fn F_SHARPR(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut TITLE = [b' '; MSGLEN as usize];
    let mut ANGLE = StackArray::<f64, 3>::new(1..=3);
    let mut AXIS = StackArray::<f64, 3>::new(1..=3);
    let mut NEARLY = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut NOISE = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut Q = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut QANGLE: f64 = 0.0;
    let mut R = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut SEED: i32 = 0;
    let mut IS: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Other functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Saved values
    //

    //
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_SHARPR", ctx)?;

    SEED = -1;

    for CASE in 1..=NUMCAS {
        //
        // --- Case: ------------------------------------------------------
        //
        fstr::assign(
            &mut TITLE,
            b"Perturb a rotation matrix; then sharpen; case #.",
        );

        spicelib::REPMI(&TITLE.clone(), b"#", CASE, &mut TITLE, ctx);
        testutil::TCASE(&TITLE, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Construct a rotation matrix from three Euler angles.
        //
        ANGLE[1] = testutil::T_RANDD(-spicelib::PI(ctx), spicelib::PI(ctx), &mut SEED, ctx)?;
        ANGLE[2] = testutil::T_RANDD(
            -(spicelib::PI(ctx) / 2.0),
            (spicelib::PI(ctx) / 2.0),
            &mut SEED,
            ctx,
        )?;
        ANGLE[3] = testutil::T_RANDD(-spicelib::PI(ctx), spicelib::PI(ctx), &mut SEED, ctx)?;

        spicelib::EUL2M(ANGLE[3], ANGLE[2], ANGLE[1], 3, 2, 3, R.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Construct a "noise" matrix with which to perturb the
        // rotation.
        //
        for I in 1..=3 {
            for J in 1..=3 {
                NOISE[[I, J]] = testutil::T_RANDD(-AMP, AMP, &mut SEED, ctx)?;
            }
        }

        //
        // NEARLY is "nearly" a rotation.
        //
        spicelib::VADDG(R.as_slice(), NOISE.as_slice(), 9, NEARLY.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Now sharpen the rotation.
        //
        spicelib::SHARPR(NEARLY.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Make sure the result is a rotation.
        //
        IS = spicelib::ISROT(NEARLY.as_slice(), NTOL, DTOL, ctx)?;

        testutil::CHCKSL(b"Is result a rotation?", IS, true, OK, ctx)?;

        //
        // Make sure the result is not too different from R.
        // Express the quotient of R and NEARLY as a rotation;
        // measure the rotation angle.
        //
        spicelib::MTXM(NEARLY.as_slice(), R.as_slice(), Q.as_slice_mut());

        spicelib::RAXISA(Q.as_slice(), AXIS.as_slice_mut(), &mut QANGLE, ctx)?;

        testutil::CHCKSD(b"QANGLE", QANGLE, b"~", 0.0, ((4 as f64) * AMP), OK, ctx)?;
    }

    //
    //     Now for some error handling tests.
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SHARPR:  pass in a singular matrix.  All we expect is that the routine doesn\'t crash.",
        ctx,
    )?;

    spicelib::CLEARD(9, R.as_slice_mut());

    spicelib::SHARPR(R.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
