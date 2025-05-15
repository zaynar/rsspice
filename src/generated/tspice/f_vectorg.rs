//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const N: i32 = 99;
const TIGHT: f64 = 0.000000000001;
const TINY: f64 = 0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001;
const SEED0: i32 = 19810518;

//$Procedure F_VECTORG (Family of tests on general vector operations)
pub fn F_VECTORG(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut VZERO: bool = false;
    let mut V_ZERO = StackArray::<f64, 99>::new(1..=N);
    let mut V_INDX = StackArray::<f64, 99>::new(1..=N);
    let mut V_RAN = StackArray::<f64, 99>::new(1..=N);
    let mut V_OUT = StackArray::<f64, 99>::new(1..=N);
    let mut V_EXPT = StackArray::<f64, 99>::new(1..=N);
    let mut V_TMP = StackArray::<f64, 99>::new(1..=N);
    let mut VECMAG: f64 = 0.0;
    let mut EVAL: f64 = 0.0;
    let mut EXPT: f64 = 0.0;
    let mut SCALE: f64 = 0.0;
    let mut DUMMY: f64 = 0.0;
    let mut SEED: i32 = 0;

    //
    // Test Utility Functions
    //

    //
    // SPICELIB Functions
    //

    //
    // Local parameters
    //

    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_VECTORG", ctx)?;

    //
    // Form vectors to be used for more than one test case.
    //

    //
    // A zero vector
    //
    for I in 1..=N {
        V_ZERO[I] = 0.0;
    }

    //
    // An index vector, whose components are their indices.
    //
    for I in 1..=N {
        V_INDX[I] = (I as f64);
    }

    //
    // A random vector, whose components are
    // uniformly distributed on the interval [-1.0D0,+1.0D0].
    //
    SEED = SEED0;

    for I in 1..=N {
        V_RAN[I] = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;
    }

    //
    // Cases to test VZEROG
    //
    testutil::TCASE(b"Test VZEROG with a zero vector", ctx)?;

    VZERO = spicelib::VZEROG(V_ZERO.as_slice(), N);

    testutil::CHCKSL(b"VZERO", VZERO, true, OK, ctx)?;

    testutil::TCASE(b"Test VZEROG with an index vector", ctx)?;

    VZERO = spicelib::VZEROG(V_INDX.as_slice(), N);

    testutil::CHCKSL(b"VZERO", VZERO, false, OK, ctx)?;

    testutil::TCASE(b"Test VZEROG with a zero dimension", ctx)?;

    VZERO = spicelib::VZEROG(V_ZERO.as_slice(), 0);

    testutil::CHCKSL(b"VZERO", VZERO, false, OK, ctx)?;

    testutil::TCASE(b"Test VZEROG with a negative dimension", ctx)?;

    VZERO = spicelib::VZEROG(V_ZERO.as_slice(), -N);

    testutil::CHCKSL(b"VZERO", VZERO, false, OK, ctx)?;

    //
    // Cases to test VEQUG
    //
    testutil::TCASE(b"Test VEQUG with a zero vector", ctx)?;

    spicelib::VEQUG(V_ZERO.as_slice(), N, V_OUT.as_slice_mut());

    testutil::CHCKAD(
        b"V_OUT",
        V_OUT.as_slice(),
        b"~",
        V_ZERO.as_slice(),
        N,
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Test VEQUG with an index vector", ctx)?;

    spicelib::VEQUG(V_INDX.as_slice(), N, V_OUT.as_slice_mut());

    testutil::CHCKAD(
        b"V_OUT",
        V_OUT.as_slice(),
        b"~",
        V_INDX.as_slice(),
        N,
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Test VEQUG with a random vector", ctx)?;

    spicelib::VEQUG(V_RAN.as_slice(), N, V_OUT.as_slice_mut());

    testutil::CHCKAD(
        b"V_OUT",
        V_OUT.as_slice(),
        b"~",
        V_RAN.as_slice(),
        N,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Cases to test VADDG
    //
    testutil::TCASE(
        b"Test VADDG by adding a zero vector and a random vector",
        ctx,
    )?;

    spicelib::VADDG(V_ZERO.as_slice(), V_RAN.as_slice(), N, V_OUT.as_slice_mut());

    testutil::CHCKAD(
        b"V_OUT",
        V_OUT.as_slice(),
        b"~",
        V_RAN.as_slice(),
        N,
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(
        b"Test VADDG by adding an index vector and a random vector",
        ctx,
    )?;

    spicelib::VADDG(V_INDX.as_slice(), V_RAN.as_slice(), N, V_OUT.as_slice_mut());

    for I in 1..=N {
        V_EXPT[I] = (V_INDX[I] + V_RAN[I]);
    }

    testutil::CHCKAD(
        b"V_OUT",
        V_OUT.as_slice(),
        b"~",
        V_EXPT.as_slice(),
        N,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Cases to test VSUBG
    //
    testutil::TCASE(
        b"Test VSUBG by subtracting a zero vector from a random vector",
        ctx,
    )?;

    spicelib::VSUBG(V_RAN.as_slice(), V_ZERO.as_slice(), N, V_OUT.as_slice_mut());

    testutil::CHCKAD(
        b"V_OUT",
        V_OUT.as_slice(),
        b"~",
        V_RAN.as_slice(),
        N,
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(
        b"Test VADDG by subtracting an index vector from a random vector",
        ctx,
    )?;

    spicelib::VSUBG(V_RAN.as_slice(), V_INDX.as_slice(), N, V_OUT.as_slice_mut());

    for I in 1..=N {
        V_EXPT[I] = (V_RAN[I] - V_INDX[I]);
    }

    testutil::CHCKAD(
        b"V_OUT",
        V_OUT.as_slice(),
        b"~",
        V_EXPT.as_slice(),
        N,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Cases to test VDOTG
    //
    testutil::TCASE(b"Test VDOTG with a zero vector and a random vector", ctx)?;

    EVAL = spicelib::VDOTG(V_ZERO.as_slice(), V_RAN.as_slice(), N);

    EXPT = 0.0;

    testutil::CHCKSD(b"EVAL", EVAL, b"~", EXPT, 0.0, OK, ctx)?;

    testutil::TCASE(b"Test VDOTG with an index vector and a random vector", ctx)?;

    EVAL = spicelib::VDOTG(V_INDX.as_slice(), V_RAN.as_slice(), N);

    EXPT = 0.0;

    for I in 1..=N {
        EXPT = (EXPT + (V_INDX[I] * V_RAN[I]));
    }

    testutil::CHCKSD(b"EVAL", EVAL, b"~", EXPT, TINY, OK, ctx)?;

    //
    // Cases to test VNORMG
    //
    testutil::TCASE(b"Test VNORMG with a zero vector", ctx)?;

    EVAL = spicelib::VNORMG(V_ZERO.as_slice(), N);

    EXPT = 0.0;

    testutil::CHCKSD(b"EVAL", EVAL, b"~", EXPT, 0.0, OK, ctx)?;

    testutil::TCASE(b"Test VNORMG with an index vector", ctx)?;

    EVAL = spicelib::VNORMG(V_INDX.as_slice(), N);

    DUMMY = 0.0;

    for I in 1..=N {
        DUMMY = (DUMMY + (V_INDX[I] * V_INDX[I]));
    }

    EXPT = f64::sqrt(DUMMY);

    testutil::CHCKSD(b"EVAL", EVAL, b"~/", EXPT, TIGHT, OK, ctx)?;

    //
    // Cases to test UNORMG
    //
    testutil::TCASE(b"Test UNORMG with a zero vector", ctx)?;

    spicelib::UNORMG(V_ZERO.as_slice(), N, V_OUT.as_slice_mut(), &mut VECMAG);

    testutil::CHCKAD(
        b"V_OUT",
        V_OUT.as_slice(),
        b"~",
        V_ZERO.as_slice(),
        N,
        0.0,
        OK,
        ctx,
    )?;

    testutil::CHCKSD(b"VECMAG", VECMAG, b"~", 0.0, 0.0, OK, ctx)?;

    testutil::TCASE(b"Test UNORMG with a random vector", ctx)?;

    spicelib::UNORMG(V_RAN.as_slice(), N, V_OUT.as_slice_mut(), &mut VECMAG);

    EXPT = spicelib::VNORMG(V_RAN.as_slice(), N);

    testutil::CHCKSD(b"VECMAG", VECMAG, b"~", EXPT, 0.0, OK, ctx)?;

    EXPT = 1.0;

    DUMMY = spicelib::VNORMG(V_OUT.as_slice(), N);

    testutil::CHCKSD(b"DUMMY", DUMMY, b"~", EXPT, TIGHT, OK, ctx)?;

    spicelib::VSCLG(VECMAG, V_OUT.as_slice(), N, V_TMP.as_slice_mut());

    testutil::CHCKAD(
        b"V_TMP",
        V_TMP.as_slice(),
        b"~",
        V_RAN.as_slice(),
        N,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // Cases to test VHATG
    //
    testutil::TCASE(b"Test VHATG with a zero vector", ctx)?;

    spicelib::VHATG(V_ZERO.as_slice(), N, V_OUT.as_slice_mut());

    //
    // VHATG returns a zero vector for a zero vector input.
    //
    testutil::CHCKAD(
        b"V_OUT",
        V_OUT.as_slice(),
        b"~",
        V_ZERO.as_slice(),
        N,
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Test VHATG with a random vector", ctx)?;

    spicelib::VHATG(V_RAN.as_slice(), N, V_OUT.as_slice_mut());

    EVAL = spicelib::VNORMG(V_OUT.as_slice(), N);

    testutil::CHCKSD(b"EVAL", EVAL, b"~", 1.0, TIGHT, OK, ctx)?;

    //
    // Cases to test VSCLG
    //
    testutil::TCASE(b"Test VSCLG by scaling an index vector", ctx)?;

    SCALE = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;

    spicelib::VSCLG(SCALE, V_INDX.as_slice(), N, V_OUT.as_slice_mut());

    for I in 1..=N {
        V_EXPT[I] = (SCALE * V_INDX[I]);
    }

    testutil::CHCKAD(
        b"V_OUT",
        V_OUT.as_slice(),
        b"~",
        V_EXPT.as_slice(),
        N,
        TIGHT,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Test VSCLG by scaling back to the index vector", ctx)?;

    SCALE = (1.0 / SCALE);

    spicelib::VSCLG(SCALE, V_OUT.as_slice(), N, V_TMP.as_slice_mut());

    testutil::CHCKAD(
        b"V_TMP",
        V_TMP.as_slice(),
        b"~",
        V_INDX.as_slice(),
        N,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // Cases to test VDISTG
    //
    testutil::TCASE(b"Test VDISTG using the same vector", ctx)?;

    EVAL = spicelib::VDISTG(V_INDX.as_slice(), V_INDX.as_slice(), N);

    //
    // Distance between same vectors should be zero.
    //
    testutil::CHCKSD(b"EVAL", EVAL, b"~", 0.0, 0.0, OK, ctx)?;

    testutil::TCASE(
        b"Test VDISTG using an index vector and a random vector",
        ctx,
    )?;

    EVAL = spicelib::VDISTG(V_INDX.as_slice(), V_RAN.as_slice(), N);

    spicelib::VSUBG(V_INDX.as_slice(), V_RAN.as_slice(), N, V_TMP.as_slice_mut());

    EXPT = spicelib::VNORMG(V_TMP.as_slice(), N);

    testutil::CHCKSD(b"EVAL", EVAL, b"~", EXPT, TIGHT, OK, ctx)?;

    //
    // Cases to test VRELG
    //
    testutil::TCASE(b"Test VRELG using two zero vectors", ctx)?;

    EVAL = spicelib::VRELG(V_ZERO.as_slice(), V_ZERO.as_slice(), N);

    testutil::CHCKSD(b"EVAL", EVAL, b"~", 0.0, 0.0, OK, ctx)?;

    testutil::TCASE(b"Test VRELG using an index vector and a random vector", ctx)?;

    EVAL = spicelib::VRELG(V_INDX.as_slice(), V_RAN.as_slice(), N);

    EXPT = (spicelib::VDISTG(V_INDX.as_slice(), V_RAN.as_slice(), N)
        / intrinsics::DMAX1(&[
            spicelib::VNORMG(V_INDX.as_slice(), N),
            spicelib::VNORMG(V_RAN.as_slice(), N),
        ]));

    testutil::CHCKSD(b"EVAL", EVAL, b"~", EXPT, 0.0, OK, ctx)?;

    //
    // Cases to test VSEPG
    //
    testutil::TCASE(
        b"Test VSEPG for the angle between a zero vector and a random vector",
        ctx,
    )?;

    EVAL = spicelib::VSEPG(V_ZERO.as_slice(), V_RAN.as_slice(), N, ctx);

    testutil::CHCKSD(b"EVAL", EVAL, b"~", 0.0, 0.0, OK, ctx)?;

    testutil::TCASE(
        b"Test VSEPG for the angle between a random vector and a zero vector",
        ctx,
    )?;

    EVAL = spicelib::VSEPG(V_RAN.as_slice(), V_ZERO.as_slice(), N, ctx);

    testutil::CHCKSD(b"EVAL", EVAL, b"~", 0.0, 0.0, OK, ctx)?;

    testutil::TCASE(b"Test VSEPG for the angle between two same vectors", ctx)?;

    EVAL = spicelib::VSEPG(V_INDX.as_slice(), V_INDX.as_slice(), N, ctx);

    testutil::CHCKSD(b"EVAL", EVAL, b"~", 0.0, 0.0, OK, ctx)?;

    testutil::TCASE(
        b"Test VSEPG for the angle between a random vector and an index vector",
        ctx,
    )?;

    EVAL = spicelib::VSEPG(V_INDX.as_slice(), V_RAN.as_slice(), N, ctx);

    EXPT = f64::acos(
        (spicelib::VDOTG(V_INDX.as_slice(), V_RAN.as_slice(), N)
            / (spicelib::VNORMG(V_INDX.as_slice(), N) * spicelib::VNORMG(V_RAN.as_slice(), N))),
    );

    testutil::CHCKSD(b"EVAL", EVAL, b"~", EXPT, TIGHT, OK, ctx)?;

    //
    // Cases to test VLCOMG
    //
    testutil::TCASE(
        b"Test VLCOMG using an index vector and a random vector",
        ctx,
    )?;

    SCALE = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;
    DUMMY = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;

    spicelib::VLCOMG(
        N,
        SCALE,
        V_INDX.as_slice(),
        DUMMY,
        V_RAN.as_slice(),
        V_OUT.as_slice_mut(),
    );

    for I in 1..=N {
        V_EXPT[I] = ((SCALE * V_INDX[I]) + (DUMMY * V_RAN[I]));
    }

    testutil::CHCKAD(
        b"V_OUT",
        V_OUT.as_slice(),
        b"~",
        V_EXPT.as_slice(),
        N,
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Test VLCOMG using an zero vector and a random vector", ctx)?;

    SCALE = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;

    spicelib::VLCOMG(
        N,
        SCALE,
        V_ZERO.as_slice(),
        1.0,
        V_RAN.as_slice(),
        V_OUT.as_slice_mut(),
    );

    testutil::CHCKAD(
        b"V_OUT",
        V_OUT.as_slice(),
        b"~",
        V_RAN.as_slice(),
        N,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Cases to test VPROJG
    //
    testutil::TCASE(
        b"Test VPROJG by projecting a random vector to a zero vector",
        ctx,
    )?;

    spicelib::VPROJG(V_RAN.as_slice(), V_ZERO.as_slice(), N, V_OUT.as_slice_mut());

    testutil::CHCKAD(
        b"V_OUT",
        V_OUT.as_slice(),
        b"~",
        V_ZERO.as_slice(),
        N,
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(
        b"Test VPROJG by projecting a random vector to an index vector",
        ctx,
    )?;

    spicelib::VPROJG(V_RAN.as_slice(), V_INDX.as_slice(), N, V_OUT.as_slice_mut());

    //
    // Compute V_TMP to be the unit vector along V_INDX.
    //
    spicelib::VHATG(V_INDX.as_slice(), N, V_TMP.as_slice_mut());

    //
    // Check ( V_RAN DOT V_TMP ) .EQ. ( V_OUT DOT V_TMP ).
    //
    DUMMY = spicelib::VDOTG(V_RAN.as_slice(), V_TMP.as_slice(), N);

    EVAL = spicelib::VDOTG(V_OUT.as_slice(), V_TMP.as_slice(), N);

    testutil::CHCKSD(b"EVAL", EVAL, b"~", DUMMY, TIGHT, OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
