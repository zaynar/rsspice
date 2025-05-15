//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const N: i32 = 3;
const TIGHT: f64 = 0.000000000001;
const VTIGHT: f64 = 0.000000000000001;
const SEED0: i32 = 1980518;

//$Procedure F_VECTOR3 (Family of tests on 3-vector operations)
pub fn F_VECTOR3(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut SEED: i32 = 0;
    let mut V_ZERO = StackArray::<f64, 3>::new(1..=N);
    let mut V_INDX = StackArray::<f64, 3>::new(1..=N);
    let mut V_RAN = StackArray::<f64, 3>::new(1..=N);
    let mut V_OUT = StackArray::<f64, 3>::new(1..=N);
    let mut V_EXPT = StackArray::<f64, 3>::new(1..=N);
    let mut V_TEMP = StackArray::<f64, 3>::new(1..=N);
    let mut VMAG: f64 = 0.0;
    let mut EVAL: f64 = 0.0;
    let mut EXPT: f64 = 0.0;
    let mut X: f64 = 0.0;
    let mut Y: f64 = 0.0;
    let mut Z: f64 = 0.0;
    let mut SCALE: f64 = 0.0;
    let mut DUMMY: f64 = 0.0;
    let mut SCALE1: f64 = 0.0;
    let mut SCALE2: f64 = 0.0;
    let mut SCALE3: f64 = 0.0;
    let mut THETA: f64 = 0.0;
    let mut CHECK: bool = false;

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
    // initial seed number for pseudo-random number generation
    //

    //
    // Local Variables
    //

    //
    // An index
    //

    //
    // Seed number for pseudo-random number generation
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_VECTOR3", ctx)?;

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
    // An index vector, whose components are their indices
    //
    for I in 1..=N {
        V_INDX[I] = (I as f64);
    }

    //
    // A pseudo-random vector, whose components are
    // uniformly distributed on the interval [-1.0D0, +1.0D0]
    //
    SEED = SEED0;

    for I in 1..=N {
        V_RAN[I] = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;
    }

    //
    // Cases to test VZERO
    //
    testutil::TCASE(b"VZERO: zero vector", ctx)?;

    CHECK = spicelib::VZERO(V_ZERO.as_slice());

    testutil::CHCKSL(b"CHECK", CHECK, true, OK, ctx)?;

    testutil::TCASE(b"VZERO: index vector", ctx)?;

    CHECK = spicelib::VZERO(V_INDX.as_slice());

    testutil::CHCKSL(b"CHECK", CHECK, false, OK, ctx)?;

    testutil::TCASE(b"VZERO: random vector", ctx)?;

    CHECK = spicelib::VZERO(V_RAN.as_slice());

    testutil::CHCKSL(b"CHECK", CHECK, false, OK, ctx)?;

    //
    // Cases to test VEQU
    //
    testutil::TCASE(b"VEQU: zero vector", ctx)?;

    spicelib::VEQU(V_ZERO.as_slice(), V_OUT.as_slice_mut());

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

    testutil::TCASE(b"VEQU: index vector", ctx)?;

    spicelib::VEQU(V_INDX.as_slice(), V_OUT.as_slice_mut());

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

    testutil::TCASE(b"VEQU: random vector", ctx)?;

    spicelib::VEQU(V_RAN.as_slice(), V_OUT.as_slice_mut());

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
    // Cases to test VADD
    //
    testutil::TCASE(b"VADD: zero vector and random vector", ctx)?;

    spicelib::VADD(V_ZERO.as_slice(), V_RAN.as_slice(), V_OUT.as_slice_mut());

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

    testutil::TCASE(b"VADD: index vector and random vector", ctx)?;

    spicelib::VADD(V_INDX.as_slice(), V_RAN.as_slice(), V_OUT.as_slice_mut());

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
    // Cases to test VSUB
    //
    testutil::TCASE(b"VSUB: zero vector and random vector", ctx)?;

    spicelib::VSUB(V_RAN.as_slice(), V_ZERO.as_slice(), V_OUT.as_slice_mut());

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

    testutil::TCASE(b"VSUB: index vector and random vector", ctx)?;

    spicelib::VSUB(V_RAN.as_slice(), V_INDX.as_slice(), V_OUT.as_slice_mut());

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
    // Cases to test VDOT.
    //
    testutil::TCASE(b"VDOT: zero vector and random vector", ctx)?;

    EVAL = spicelib::VDOT(V_ZERO.as_slice(), V_RAN.as_slice());

    testutil::CHCKSD(b"EVAL", EVAL, b"~", 0.0, 0.0, OK, ctx)?;

    testutil::TCASE(b"VDOT: index vector and random vector", ctx)?;

    EVAL = spicelib::VDOT(V_INDX.as_slice(), V_RAN.as_slice());

    EXPT = 0.0;

    for I in 1..=N {
        EXPT = (EXPT + (V_INDX[I] * V_RAN[I]));
    }

    testutil::CHCKSD(b"EVAL", EVAL, b"~", EXPT, 0.0, OK, ctx)?;

    //
    // Cases to test VPACK
    //
    testutil::TCASE(b"VPACK: random vector.", ctx)?;

    X = V_RAN[1];
    Y = V_RAN[2];
    Z = V_RAN[3];

    spicelib::VPACK(X, Y, Z, V_OUT.as_slice_mut());

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
    // Cases to test VUPACK
    //
    testutil::TCASE(b"VUPACK: random vector", ctx)?;

    spicelib::VUPACK(V_RAN.as_slice(), &mut X, &mut Y, &mut Z);

    V_EXPT[1] = X;
    V_EXPT[2] = Y;
    V_EXPT[3] = Z;

    testutil::CHCKAD(
        b"V_EXPT",
        V_EXPT.as_slice(),
        b"~",
        V_RAN.as_slice(),
        N,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Cases to test VSCL
    //
    testutil::TCASE(b"VSCL: scaling an index vector", ctx)?;

    SCALE = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;

    spicelib::VSCL(SCALE, V_INDX.as_slice(), V_OUT.as_slice_mut());

    for I in 1..=N {
        V_EXPT[I] = (SCALE * V_INDX[I]);
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

    testutil::TCASE(b"VSCL: scaling back to the index vector", ctx)?;

    SCALE = (1.0 / SCALE);

    spicelib::VSCL(SCALE, V_OUT.as_slice(), V_TEMP.as_slice_mut());

    testutil::CHCKAD(
        b"V_TEMP",
        V_TEMP.as_slice(),
        b"~",
        V_INDX.as_slice(),
        N,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // Cases to test VSCLIP
    //
    testutil::TCASE(b"VSCLIP: scaling an index vector", ctx)?;

    SCALE = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;

    spicelib::VEQU(V_INDX.as_slice(), V_OUT.as_slice_mut());

    spicelib::VSCLIP(SCALE, V_OUT.as_slice_mut());

    for I in 1..=N {
        V_EXPT[I] = (SCALE * V_INDX[I]);
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
    // Cases to test VNORM
    //
    testutil::TCASE(b"VNORM: zero vector", ctx)?;

    EVAL = spicelib::VNORM(V_ZERO.as_slice());

    testutil::CHCKSD(b"EVAL", EVAL, b"~", 0.0, 0.0, OK, ctx)?;

    testutil::TCASE(b"Test VNORM with an index vector", ctx)?;

    EVAL = spicelib::VNORM(V_INDX.as_slice());

    EXPT = 0.0;

    for I in 1..=N {
        EXPT = (EXPT + (V_INDX[I] * V_INDX[I]));
    }

    EXPT = f64::sqrt(EXPT);

    testutil::CHCKSD(b"EVAL", EVAL, b"~", EXPT, VTIGHT, OK, ctx)?;

    //
    // Cases to test VMINUS
    //
    testutil::TCASE(b"VMINUS: random vector", ctx)?;

    spicelib::VMINUS(V_RAN.as_slice(), V_OUT.as_slice_mut());

    spicelib::VADD(V_RAN.as_slice(), V_OUT.as_slice(), V_TEMP.as_slice_mut());

    testutil::CHCKAD(
        b"V_TEMP",
        V_TEMP.as_slice(),
        b"~",
        V_ZERO.as_slice(),
        N,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Cases to test UNORM
    //
    testutil::TCASE(b"UNORM: zero vector", ctx)?;

    spicelib::UNORM(V_ZERO.as_slice(), V_OUT.as_slice_mut(), &mut VMAG);

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

    testutil::CHCKSD(b"VMAG", VMAG, b"~", 0.0, 0.0, OK, ctx)?;

    testutil::TCASE(b"UNORM:  random vector.", ctx)?;

    spicelib::UNORM(V_RAN.as_slice(), V_OUT.as_slice_mut(), &mut VMAG);

    EXPT = spicelib::VNORM(V_RAN.as_slice());
    testutil::CHCKSD(b"VMAG", VMAG, b"~", EXPT, TIGHT, OK, ctx)?;

    EVAL = spicelib::VNORM(V_OUT.as_slice());
    testutil::CHCKSD(b"EVAL", EVAL, b"~", 1.0, TIGHT, OK, ctx)?;

    spicelib::VSCL(VMAG, V_OUT.as_slice(), V_EXPT.as_slice_mut());
    testutil::CHCKAD(
        b"V_EXPT",
        V_EXPT.as_slice(),
        b"~",
        V_RAN.as_slice(),
        N,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // Cases to test VHAT
    //
    testutil::TCASE(b"VHAT: zero vector", ctx)?;

    spicelib::VHAT(V_ZERO.as_slice(), V_OUT.as_slice_mut());

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

    testutil::TCASE(b"VHAT: random vector.", ctx)?;

    spicelib::VHAT(V_RAN.as_slice(), V_OUT.as_slice_mut());

    EVAL = spicelib::VNORM(V_OUT.as_slice());

    testutil::CHCKSD(b"EVAL", EVAL, b"~", 1.0, TIGHT, OK, ctx)?;

    //
    // Cases to test VHATIP
    //
    testutil::TCASE(b"VHATIP: zero vector", ctx)?;

    spicelib::VEQU(V_ZERO.as_slice(), V_OUT.as_slice_mut());

    spicelib::VHATIP(V_OUT.as_slice_mut());

    testutil::CHCKAD(
        b"V_OUT",
        V_OUT.as_slice(),
        b"~",
        V_ZERO.as_slice(),
        N,
        TIGHT,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"VHATIP: random vector", ctx)?;

    spicelib::VEQU(V_RAN.as_slice(), V_OUT.as_slice_mut());

    spicelib::VHATIP(V_OUT.as_slice_mut());

    EVAL = spicelib::VNORM(V_OUT.as_slice());

    testutil::CHCKSD(b"EVAL", EVAL, b"~", 1.0, TIGHT, OK, ctx)?;

    //
    // Cases to test VDIST
    //
    testutil::TCASE(b"VDIST: same vector", ctx)?;

    EVAL = spicelib::VDIST(V_RAN.as_slice(), V_RAN.as_slice());

    testutil::CHCKSD(b"EVAL", EVAL, b"~", 0.0, 0.0, OK, ctx)?;

    testutil::TCASE(b"VDIST: index vector and random vector", ctx)?;

    EVAL = spicelib::VDIST(V_INDX.as_slice(), V_RAN.as_slice());

    spicelib::VSUB(V_INDX.as_slice(), V_RAN.as_slice(), V_OUT.as_slice_mut());

    EXPT = spicelib::VNORM(V_OUT.as_slice());

    testutil::CHCKSD(b"EVAL", EVAL, b"~", EXPT, TIGHT, OK, ctx)?;

    //
    // Cases to test VREL
    //
    testutil::TCASE(b"VREL: two zero vectors", ctx)?;

    EVAL = spicelib::VREL(V_ZERO.as_slice(), V_ZERO.as_slice());

    testutil::CHCKSD(b"EVAL", EVAL, b"~", 0.0, 0.0, OK, ctx)?;

    testutil::TCASE(b"VREL: index vector and random vector", ctx)?;

    EVAL = spicelib::VREL(V_INDX.as_slice(), V_RAN.as_slice());

    EXPT = (spicelib::VDIST(V_INDX.as_slice(), V_RAN.as_slice())
        / intrinsics::DMAX1(&[
            spicelib::VNORM(V_INDX.as_slice()),
            spicelib::VNORM(V_RAN.as_slice()),
        ]));

    testutil::CHCKSD(b"EVAL", EVAL, b"~", EXPT, TIGHT, OK, ctx)?;

    //
    // Cases to test VSEP
    //
    testutil::TCASE(
        b"VSEP: angle between a zero vector and a random vector",
        ctx,
    )?;

    EVAL = spicelib::VSEP(V_ZERO.as_slice(), V_RAN.as_slice(), ctx);

    testutil::CHCKSD(b"EVAL", EVAL, b"~", 0.0, TIGHT, OK, ctx)?;

    testutil::TCASE(
        b"VSEP: angle between a random vector and a zero vector",
        ctx,
    )?;

    EVAL = spicelib::VSEP(V_RAN.as_slice(), V_ZERO.as_slice(), ctx);

    testutil::CHCKSD(b"EVAL", EVAL, b"~", 0.0, TIGHT, OK, ctx)?;

    //
    // Cases to test VLCOM
    //
    testutil::TCASE(b"VLCOM: index vector and random vector", ctx)?;

    SCALE = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;

    DUMMY = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;

    spicelib::VLCOM(
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
        TIGHT,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"VLCOM: zero vector and random vector", ctx)?;

    spicelib::VLCOM(
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
    // Cases to test VLCOM3
    //
    testutil::TCASE(b"VLCOM: index vector and random vector", ctx)?;

    SCALE1 = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;
    SCALE2 = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;
    SCALE3 = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;

    spicelib::VLCOM3(
        SCALE1,
        V_INDX.as_slice(),
        SCALE2,
        V_RAN.as_slice(),
        SCALE3,
        V_RAN.as_slice(),
        V_OUT.as_slice_mut(),
    );

    for I in 1..=N {
        V_EXPT[I] = (((SCALE1 * V_INDX[I]) + (SCALE2 * V_RAN[I])) + (SCALE3 * V_RAN[I]));
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

    //
    // Cases to test VPROJ
    //
    testutil::TCASE(b"VPROJ: projecting a random vector to a zero vector", ctx)?;

    spicelib::VPROJ(V_RAN.as_slice(), V_ZERO.as_slice(), V_OUT.as_slice_mut());

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

    testutil::TCASE(b"VPROJ: projecting a random vector to an index vector", ctx)?;

    spicelib::VPROJ(V_RAN.as_slice(), V_INDX.as_slice(), V_OUT.as_slice_mut());

    //
    // Compute V_TEMP to be the unit vector along V_INDX.
    //
    spicelib::VHAT(V_INDX.as_slice(), V_TEMP.as_slice_mut());

    //
    // Check ( V_RAN DOT V_TEMP ) .EQ. ( V_OUT DOT V_TEMP ).
    //
    DUMMY = spicelib::VDOT(V_RAN.as_slice(), V_TEMP.as_slice());

    EVAL = spicelib::VDOT(V_OUT.as_slice(), V_TEMP.as_slice());

    testutil::CHCKSD(b"EVAL", EVAL, b"~", DUMMY, TIGHT, OK, ctx)?;

    //
    // Cases to test VPERP
    //

    testutil::TCASE(b"VPERP: projecting a zero vector to a random vector", ctx)?;

    spicelib::VPERP(V_ZERO.as_slice(), V_RAN.as_slice(), V_OUT.as_slice_mut());

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

    testutil::TCASE(b"VPERP: projecting an index vector to a random vector", ctx)?;

    spicelib::VPERP(V_INDX.as_slice(), V_RAN.as_slice(), V_OUT.as_slice_mut());

    spicelib::VSUB(V_INDX.as_slice(), V_OUT.as_slice(), V_TEMP.as_slice_mut());

    spicelib::VCRSS(V_TEMP.as_slice(), V_RAN.as_slice(), V_OUT.as_slice_mut());

    testutil::CHCKAD(
        b"V_OUT",
        V_OUT.as_slice(),
        b"~",
        V_ZERO.as_slice(),
        N,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // Cases to test UCRSS
    //
    testutil::TCASE(b"UCRSS: index vector and random vector", ctx)?;

    spicelib::UCRSS(V_INDX.as_slice(), V_RAN.as_slice(), V_OUT.as_slice_mut());

    EVAL = spicelib::VDOT(V_OUT.as_slice(), V_INDX.as_slice());

    testutil::CHCKSD(b"EVAL", EVAL, b"~", 0.0, TIGHT, OK, ctx)?;

    EVAL = spicelib::VDOT(V_OUT.as_slice(), V_RAN.as_slice());

    testutil::CHCKSD(b"EVAL", EVAL, b"~", 0.0, TIGHT, OK, ctx)?;

    EVAL = spicelib::VDOT(V_OUT.as_slice(), V_OUT.as_slice());

    testutil::CHCKSD(b"EVAL", EVAL, b"~", 1.0, TIGHT, OK, ctx)?;

    testutil::TCASE(b"UCRSS: zero vector and random vector", ctx)?;

    spicelib::UCRSS(V_ZERO.as_slice(), V_RAN.as_slice(), V_OUT.as_slice_mut());

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

    //
    // Cases to test VCRSS
    //
    testutil::TCASE(b"VCRSS: index vector and random vector", ctx)?;

    spicelib::VCRSS(V_INDX.as_slice(), V_RAN.as_slice(), V_OUT.as_slice_mut());

    EVAL = spicelib::VDOT(V_OUT.as_slice(), V_INDX.as_slice());

    testutil::CHCKSD(b"EVAL", EVAL, b"~", 0.0, TIGHT, OK, ctx)?;

    EVAL = spicelib::VDOT(V_OUT.as_slice(), V_RAN.as_slice());

    testutil::CHCKSD(b"EVAL", EVAL, b"~", 0.0, TIGHT, OK, ctx)?;

    EVAL = ((f64::powi(
        (spicelib::VNORM(V_INDX.as_slice()) * spicelib::VNORM(V_RAN.as_slice())),
        2,
    ) - f64::powi(spicelib::VNORM(V_OUT.as_slice()), 2))
        - f64::powi(spicelib::VDOT(V_INDX.as_slice(), V_RAN.as_slice()), 2));

    testutil::CHCKSD(b"EVAL", EVAL, b"~", 0.0, TIGHT, OK, ctx)?;

    testutil::TCASE(b"VCRSS: zero vector and random vector", ctx)?;

    spicelib::VCRSS(V_ZERO.as_slice(), V_RAN.as_slice(), V_OUT.as_slice_mut());

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

    //
    // Cases to test VROTV
    //
    testutil::TCASE(b"VROTV: rotating a random vector wrt a zero vector", ctx)?;

    THETA = testutil::T_RANDD(0.0, spicelib::TWOPI(ctx), &mut SEED, ctx)?;

    spicelib::VROTV(
        V_RAN.as_slice(),
        V_ZERO.as_slice(),
        THETA,
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

    testutil::TCASE(b"VROTV: rotating a random vector wrt an index vector", ctx)?;

    THETA = testutil::T_RANDD(0.0, spicelib::TWOPI(ctx), &mut SEED, ctx)?;

    spicelib::VROTV(
        V_RAN.as_slice(),
        V_INDX.as_slice(),
        THETA,
        V_OUT.as_slice_mut(),
    );

    //
    // Angle between V_RAN and V_INDX should not be changed.
    //
    EVAL = (spicelib::VSEP(V_RAN.as_slice(), V_INDX.as_slice(), ctx)
        - spicelib::VSEP(V_OUT.as_slice(), V_INDX.as_slice(), ctx));

    testutil::CHCKSD(b"EVAL", EVAL, b"~", 0.0, TIGHT, OK, ctx)?;

    //
    // Cases to test ROTVEC
    //
    testutil::TCASE(
        b"ROTVEC: rotating a random vector with respect to the x-axis",
        ctx,
    )?;

    spicelib::ROTVEC(V_RAN.as_slice(), THETA, 1, V_OUT.as_slice_mut(), ctx);

    V_TEMP[1] = 1.0;
    V_TEMP[2] = 0.0;
    V_TEMP[3] = 0.0;

    spicelib::VROTV(
        V_RAN.as_slice(),
        V_TEMP.as_slice(),
        -THETA,
        V_EXPT.as_slice_mut(),
    );

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

    spicelib::ROTVEC(V_RAN.as_slice(), THETA, 2, V_OUT.as_slice_mut(), ctx);

    V_TEMP[1] = 0.0;
    V_TEMP[2] = 1.0;
    V_TEMP[3] = 0.0;

    spicelib::VROTV(
        V_RAN.as_slice(),
        V_TEMP.as_slice(),
        -THETA,
        V_EXPT.as_slice_mut(),
    );

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

    spicelib::ROTVEC(V_RAN.as_slice(), THETA, 3, V_OUT.as_slice_mut(), ctx);

    V_TEMP[1] = 0.0;
    V_TEMP[2] = 0.0;
    V_TEMP[3] = 1.0;

    spicelib::VROTV(
        V_RAN.as_slice(),
        V_TEMP.as_slice(),
        -THETA,
        V_EXPT.as_slice_mut(),
    );
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

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
