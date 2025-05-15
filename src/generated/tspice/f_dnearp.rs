//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure F_DNEARP ( Family of DNEARP tests )
pub fn F_DNEARP(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut A: f64 = 0.0;
    let mut ALT: f64 = 0.0;
    let mut B: f64 = 0.0;
    let mut C: f64 = 0.0;
    let mut DALT = StackArray::<f64, 2>::new(1..=2);
    let mut DNEAR = StackArray::<f64, 6>::new(1..=6);
    let mut EALT = StackArray::<f64, 2>::new(1..=2);
    let mut EDNEAR = StackArray::<f64, 6>::new(1..=6);
    let mut NPOINT1 = StackArray::<f64, 3>::new(1..=3);
    let mut NPOINT2 = StackArray::<f64, 3>::new(1..=3);
    let mut STATE = StackArray::<f64, 6>::new(1..=6);
    let mut TMPSTA = StackArray::<f64, 6>::new(1..=6);
    let mut FOUND: bool = false;

    //
    // Spicelib Functions
    //

    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_DNEARP", ctx)?;

    //
    // *****************************************************************
    //
    // Normal cases: DNEARP
    //
    // *****************************************************************

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Check the results of DNEARP when the body in question is the unit sphere. ",
        ctx,
    )?;

    A = 1.0;
    B = 1.0;
    C = 1.0;

    STATE[1] = 10.0;
    STATE[2] = 11.0;
    STATE[3] = 12.0;
    STATE[4] = 2.0;
    STATE[5] = 3.0;
    STATE[6] = -1.0;
    //
    // When the body is the unit sphere, the near point is just
    // the unit vector that is parallel to the position component
    // of the state vector.  The velocity of the near point
    // is the derivative of this unit vector  DVHAT handles the
    // whole problem.
    //
    spicelib::DVHAT(STATE.as_slice(), EDNEAR.as_slice_mut());

    //
    // The latitude will be the distance from the origin minus 1.
    // The rate of change of
    //
    EALT[1] = (spicelib::VNORM(STATE.as_slice()) - 1.0);
    EALT[2] = spicelib::VDOT(EDNEAR.as_slice(), STATE.subarray(4));

    spicelib::DNEARP(
        STATE.as_slice(),
        A,
        B,
        C,
        DNEAR.as_slice_mut(),
        DALT.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKAD(
        b"DNEAR",
        DNEAR.as_slice(),
        b"~/",
        EDNEAR.as_slice(),
        6,
        0.0000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKAD(
        b"DALT",
        DALT.as_slice(),
        b"~/",
        EALT.as_slice(),
        2,
        0.0000000000001,
        OK,
        ctx,
    )?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Check state of near point over a vertex of an ellipsoid. ",
        ctx,
    )?;

    A = 4.0;
    B = 4.0;
    C = 9.0;

    STATE[1] = 0.0;
    STATE[2] = 0.0;
    STATE[3] = 12.0;
    STATE[4] = 1.0;
    STATE[5] = 2.0;
    STATE[6] = 5.0;

    EDNEAR[1] = 0.0;
    EDNEAR[2] = 0.0;
    EDNEAR[3] = 9.0;
    EDNEAR[4] = 0.75;
    EDNEAR[5] = 1.5;
    EDNEAR[6] = 0.0;

    EALT[1] = 3.0;
    EALT[2] = 5.0;

    spicelib::NEARPT(
        STATE.as_slice(),
        A,
        B,
        C,
        NPOINT1.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    spicelib::VLCOM(
        1.0,
        STATE.as_slice(),
        0.00001,
        STATE.subarray(4),
        TMPSTA.as_slice_mut(),
    );
    spicelib::MOVED(TMPSTA.as_slice(), 6, STATE.as_slice_mut());
    spicelib::NEARPT(
        STATE.as_slice(),
        A,
        B,
        C,
        NPOINT2.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;

    spicelib::VLCOM(
        100000.0,
        NPOINT2.as_slice(),
        -100000.0,
        NPOINT1.as_slice(),
        EDNEAR.subarray_mut(4),
    );

    STATE[1] = 0.0;
    STATE[2] = 0.0;
    STATE[3] = 12.0;
    STATE[4] = 1.0;
    STATE[5] = 2.0;
    STATE[6] = 5.0;

    spicelib::DNEARP(
        STATE.as_slice(),
        A,
        B,
        C,
        DNEAR.as_slice_mut(),
        DALT.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKAD(
        b"DNEAR",
        DNEAR.as_slice(),
        b"~",
        EDNEAR.as_slice(),
        6,
        0.00001,
        OK,
        ctx,
    )?;
    testutil::CHCKAD(
        b"DALT",
        DALT.as_slice(),
        b"~/",
        EALT.as_slice(),
        2,
        0.0000000000001,
        OK,
        ctx,
    )?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
