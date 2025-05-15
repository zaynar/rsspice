//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      F_Q2M (Family of tests for Q2M )
pub fn F_Q2M(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut Q = StackArray::<f64, 4>::new(0..=3);
    let mut M = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut MEXP = StackArray2D::<f64, 9>::new(1..=3, 1..=3);

    //
    // Test Utility Functions
    //

    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_Q2M", ctx)?;

    testutil::TCASE(b"Identity Transformation", ctx)?;

    Q[0] = 1.0;

    Q[1] = 0.0;
    Q[2] = 0.0;
    Q[3] = 0.0;

    spicelib::Q2M(Q.as_slice(), M.as_slice_mut());
    spicelib::IDENT(MEXP.as_slice_mut());

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"M",
        M.as_slice(),
        b"~",
        MEXP.as_slice(),
        9,
        0.00000000000001,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Rotation by 90 degrees about Z", ctx)?;

    Q[0] = (f64::sqrt(2.0) / 2.0);
    Q[1] = 0.0;
    Q[2] = 0.0;
    Q[3] = Q[0];

    MEXP[[1, 1]] = 0.0;
    MEXP[[2, 1]] = 1.0;
    MEXP[[3, 1]] = 0.0;

    MEXP[[1, 2]] = -1.0;
    MEXP[[2, 2]] = 0.0;
    MEXP[[3, 2]] = 0.0;

    MEXP[[1, 3]] = 0.0;
    MEXP[[2, 3]] = 0.0;
    MEXP[[3, 3]] = 1.0;

    spicelib::Q2M(Q.as_slice(), M.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"M",
        M.as_slice(),
        b"~",
        MEXP.as_slice(),
        9,
        0.00000000000001,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Rotation by 90 degrees about Y", ctx)?;

    Q[0] = (f64::sqrt(2.0) / 2.0);
    Q[1] = 0.0;
    Q[2] = Q[0];
    Q[3] = 0.0;

    MEXP[[1, 1]] = 0.0;
    MEXP[[2, 1]] = 0.0;
    MEXP[[3, 1]] = -1.0;

    MEXP[[1, 2]] = 0.0;
    MEXP[[2, 2]] = 1.0;
    MEXP[[3, 2]] = 0.0;

    MEXP[[1, 3]] = 1.0;
    MEXP[[2, 3]] = 0.0;
    MEXP[[3, 3]] = 0.0;

    spicelib::Q2M(Q.as_slice(), M.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"M",
        M.as_slice(),
        b"~",
        MEXP.as_slice(),
        9,
        0.00000000000001,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Rotation by 90 degrees about X", ctx)?;

    Q[0] = (f64::sqrt(2.0) / 2.0);
    Q[1] = Q[0];
    Q[2] = 0.0;
    Q[3] = 0.0;

    MEXP[[1, 1]] = 1.0;
    MEXP[[2, 1]] = 0.0;
    MEXP[[3, 1]] = 0.0;

    MEXP[[1, 2]] = 0.0;
    MEXP[[2, 2]] = 0.0;
    MEXP[[3, 2]] = 1.0;

    MEXP[[1, 3]] = 0.0;
    MEXP[[2, 3]] = -1.0;
    MEXP[[3, 3]] = 0.0;

    spicelib::Q2M(Q.as_slice(), M.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"M",
        M.as_slice(),
        b"~",
        MEXP.as_slice(),
        9,
        0.00000000000001,
        OK,
        ctx,
    )?;

    testutil::TCASE(
        b"Rotation by 90 degrees about X with non-unit input quaternion. ",
        ctx,
    )?;

    Q[0] = 1.0;
    Q[1] = Q[0];
    Q[2] = 0.0;
    Q[3] = 0.0;

    MEXP[[1, 1]] = 1.0;
    MEXP[[2, 1]] = 0.0;
    MEXP[[3, 1]] = 0.0;

    MEXP[[1, 2]] = 0.0;
    MEXP[[2, 2]] = 0.0;
    MEXP[[3, 2]] = 1.0;

    MEXP[[1, 3]] = 0.0;
    MEXP[[2, 3]] = -1.0;
    MEXP[[3, 3]] = 0.0;

    spicelib::Q2M(Q.as_slice(), M.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"M",
        M.as_slice(),
        b"~",
        MEXP.as_slice(),
        9,
        0.00000000000001,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Rotation by 60 degrees about Z", ctx)?;
    //
    // Recall that we need to put in the COS and SIN of 30 degrees
    // into the various components of the quaternion.  But these
    // values are SQRT(3)/2 and 0.5 respectively.
    //
    Q[0] = (f64::sqrt(3.0) / 2.0);
    Q[1] = 0.0;
    Q[2] = 0.0;
    Q[3] = 0.5;

    MEXP[[1, 1]] = 0.5;
    MEXP[[2, 1]] = Q[0];
    MEXP[[3, 1]] = 0.0;

    MEXP[[1, 2]] = -Q[0];
    MEXP[[2, 2]] = 0.5;
    MEXP[[3, 2]] = 0.0;

    MEXP[[1, 3]] = 0.0;
    MEXP[[2, 3]] = 0.0;
    MEXP[[3, 3]] = 1.0;

    spicelib::Q2M(Q.as_slice(), M.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"M",
        M.as_slice(),
        b"~",
        MEXP.as_slice(),
        9,
        0.00000000000001,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Rotation by 60 degrees about Y", ctx)?;

    Q[0] = (f64::sqrt(3.0) / 2.0);
    Q[1] = 0.0;
    Q[2] = 0.5;
    Q[3] = 0.0;

    MEXP[[1, 1]] = 0.5;
    MEXP[[2, 1]] = 0.0;
    MEXP[[3, 1]] = -Q[0];

    MEXP[[1, 2]] = 0.0;
    MEXP[[2, 2]] = 1.0;
    MEXP[[3, 2]] = 0.0;

    MEXP[[1, 3]] = Q[0];
    MEXP[[2, 3]] = 0.0;
    MEXP[[3, 3]] = 0.5;

    spicelib::Q2M(Q.as_slice(), M.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"M",
        M.as_slice(),
        b"~",
        MEXP.as_slice(),
        9,
        0.00000000000001,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Rotation by 60 degrees about X", ctx)?;

    Q[0] = (f64::sqrt(3.0) / 2.0);
    Q[1] = 0.5;
    Q[2] = 0.0;
    Q[3] = 0.0;

    MEXP[[1, 1]] = 1.0;
    MEXP[[2, 1]] = 0.0;
    MEXP[[3, 1]] = 0.0;

    MEXP[[1, 2]] = 0.0;
    MEXP[[2, 2]] = 0.5;
    MEXP[[3, 2]] = Q[0];

    MEXP[[1, 3]] = 0.0;
    MEXP[[2, 3]] = -Q[0];
    MEXP[[3, 3]] = 0.5;

    spicelib::Q2M(Q.as_slice(), M.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"M",
        M.as_slice(),
        b"~",
        MEXP.as_slice(),
        9,
        0.00000000000001,
        OK,
        ctx,
    )?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
