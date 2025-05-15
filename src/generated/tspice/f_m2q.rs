//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      F_M2Q ( Family of tests for the SPICE routine M2Q)
pub fn F_M2Q(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut Q1 = StackArray::<f64, 4>::new(0..=3);
    let mut Q2 = StackArray::<f64, 4>::new(0..=3);
    let mut Q3 = StackArray::<f64, 4>::new(0..=3);
    let mut Q4 = StackArray::<f64, 4>::new(0..=3);
    let mut Q = StackArray::<f64, 4>::new(0..=3);
    let mut R1 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut R2 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut R3 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut R4 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);

    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_M2Q", ctx)?;

    Q1[0] = 0.9;
    Q1[1] = 0.3;
    Q1[2] = 0.3;
    Q1[3] = 0.1;

    Q2[0] = 0.1;
    Q2[1] = 0.9;
    Q2[2] = 0.3;
    Q2[3] = 0.3;

    Q3[0] = 0.3;
    Q3[1] = 0.1;
    Q3[2] = 0.9;
    Q3[3] = 0.3;

    Q4[0] = 0.3;
    Q4[1] = 0.3;
    Q4[2] = 0.1;
    Q4[3] = 0.9;

    spicelib::Q2M(Q1.as_slice(), R1.as_slice_mut());
    spicelib::Q2M(Q2.as_slice(), R2.as_slice_mut());
    spicelib::Q2M(Q3.as_slice(), R3.as_slice_mut());
    spicelib::Q2M(Q4.as_slice(), R4.as_slice_mut());

    testutil::TCASE(b"Expecting real component to be .9", ctx)?;

    spicelib::M2Q(R1.as_slice(), Q.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"Q1",
        Q.as_slice(),
        b"~",
        Q1.as_slice(),
        4,
        0.00000000000001,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Expecting I component to be 0.9", ctx)?;

    spicelib::M2Q(R2.as_slice(), Q.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"Q2",
        Q.as_slice(),
        b"~",
        Q2.as_slice(),
        4,
        0.00000000000001,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Expecting J component to be 0.9", ctx)?;

    spicelib::M2Q(R3.as_slice(), Q.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"Q3",
        Q.as_slice(),
        b"~",
        Q3.as_slice(),
        4,
        0.00000000000001,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Expecting K component to be 0.9", ctx)?;

    spicelib::M2Q(R4.as_slice(), Q.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"Q4",
        Q.as_slice(),
        b"~",
        Q4.as_slice(),
        4,
        0.00000000000001,
        OK,
        ctx,
    )?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
