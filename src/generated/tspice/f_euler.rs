//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const DTOL: f64 = 0.00000000000001;
const MTOL: f64 = 0.00000000000001;
const NTOL: f64 = 0.00000000000001;
const ANGTOL: f64 = 0.000000000002;
const MSGLEN: i32 = 240;
const NUMCAS: i32 = 300;

struct SaveVars {
    NEXT: StackArray<i32, 4>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut NEXT = StackArray::<i32, 4>::new(1..=4);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(2), Val::I(3), Val::I(1), Val::I(2)].into_iter();
            NEXT.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { NEXT }
    }
}

//$Procedure F_EULER ( Test the SPICELIB Euler angle routines )
pub fn F_EULER(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TITLE = [b' '; MSGLEN as usize];
    let mut ANGLE = StackArray::<f64, 3>::new(1..=3);
    let mut LB: f64 = 0.0;
    let mut Q = StackArray::<f64, 4>::new(0..=3);
    let mut Q1 = StackArray::<f64, 4>::new(0..=3);
    let mut Q2 = StackArray::<f64, 4>::new(0..=3);
    let mut Q3 = StackArray::<f64, 4>::new(0..=3);
    let mut QTEMP = StackArray::<f64, 4>::new(0..=3);
    let mut R = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut UB: f64 = 0.0;
    let mut XANGLE = StackArray::<f64, 3>::new(1..=3);
    let mut XR = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut SEED: i32 = 0;
    let mut ABC: bool = false;
    let mut IS: bool = false;
    let mut RHAND: bool = false;

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
    // NUMCAS is the number of random Euler angle sequence test
    // cases per axis sequence.  Since there are 27 axis sequences
    // for the EUL2M tests and 12 sequences for the M2EUL tests,
    // and since we use a random selection of angles for each, we
    // actually have 8100 random cases for EUL2M and 3600 for M2EUL.
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
    // The principal test approach is as follows: we'll construct
    // rotation matrices from sets of Euler angles using EUL2M.  We'll
    // use an alternate, computationally independent approach to
    // construct the expected matrices and compare the output of EUL2M
    // against the expected results.
    //
    // Having in hand rotations with known Euler angle factorizations,
    // we'll use M2EUL to recover the original angles.
    //
    // The EUL2M tests will be performed using every possible Euler axis
    // sequence.
    //
    // The M2EUL tests will be restricted to Euler axis sequences of the
    // forms
    //
    //    a-b-a
    //    a-b-c
    //
    // since M2EUL doesn't allow axis sequences where the middle axis is
    // equal to one of the others.
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_EULER", ctx)?;

    SEED = -1;

    //
    // We'll perform tests for each possible axis sequence.
    //
    for AXIS1 in 1..=3 {
        for AXIS2 in 1..=3 {
            for AXIS3 in 1..=3 {
                for CASE in 1..=NUMCAS {
                    //
                    // --- Case: ------------------------------------------------------
                    //
                    //
                    //                 Test EUL2M.  Select random Euler angles; construct
                    //                 a rotation matrix.
                    //
                    XANGLE[1] =
                        testutil::T_RANDD(-spicelib::PI(ctx), spicelib::PI(ctx), &mut SEED, ctx)?;

                    //
                    // The range of the middle angle depends on
                    // the type of axis sequence.  This is a matter
                    // of convention, not mathematics.
                    //
                    if (AXIS1 == AXIS3) {
                        LB = 0.0;
                        UB = spicelib::PI(ctx);
                    } else {
                        LB = -spicelib::HALFPI(ctx);
                        UB = spicelib::HALFPI(ctx);
                    }

                    XANGLE[2] = testutil::T_RANDD(LB, UB, &mut SEED, ctx)?;

                    XANGLE[3] =
                        testutil::T_RANDD(-spicelib::PI(ctx), spicelib::PI(ctx), &mut SEED, ctx)?;

                    fstr::assign(&mut TITLE, b"Test EUL2M: Create a rotation matrix from a random sequence of Euler angles; case = #.  Axis sequence = # # #. Angle sequence = # # #.");

                    spicelib::REPMI(&TITLE.clone(), b"#", CASE, &mut TITLE, ctx);
                    spicelib::REPMI(&TITLE.clone(), b"#", AXIS3, &mut TITLE, ctx);
                    spicelib::REPMI(&TITLE.clone(), b"#", AXIS2, &mut TITLE, ctx);
                    spicelib::REPMI(&TITLE.clone(), b"#", AXIS1, &mut TITLE, ctx);
                    spicelib::REPMD(&TITLE.clone(), b"#", XANGLE[3], 14, &mut TITLE, ctx);
                    spicelib::REPMD(&TITLE.clone(), b"#", XANGLE[2], 14, &mut TITLE, ctx);
                    spicelib::REPMD(&TITLE.clone(), b"#", XANGLE[1], 14, &mut TITLE, ctx);

                    testutil::TCASE(&TITLE, ctx)?;

                    //
                    // Build the rotation matrix R.
                    //
                    spicelib::EUL2M(
                        XANGLE[3],
                        XANGLE[2],
                        XANGLE[1],
                        AXIS3,
                        AXIS2,
                        AXIS1,
                        R.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Verify that EUL2M created a rotation matrix.
                    //
                    IS = spicelib::ISROT(R.as_slice(), DTOL, NTOL, ctx)?;

                    testutil::CHCKSL(b"Is R a rotation?", IS, true, OK, ctx)?;

                    //
                    // Validate the rotation by building the same
                    // matrix using quaternions.  Construct a
                    // quaternion corresponding to each factor rotation.
                    //
                    spicelib::CLEARD(4, Q1.as_slice_mut());

                    Q1[0] = f64::cos(-(XANGLE[1] / 2.0));
                    Q1[AXIS1] = f64::sin(-(XANGLE[1] / 2.0));

                    spicelib::CLEARD(4, Q2.as_slice_mut());

                    Q2[0] = f64::cos(-(XANGLE[2] / 2.0));
                    Q2[AXIS2] = f64::sin(-(XANGLE[2] / 2.0));

                    spicelib::CLEARD(4, Q3.as_slice_mut());

                    Q3[0] = f64::cos(-(XANGLE[3] / 2.0));
                    Q3[AXIS3] = f64::sin(-(XANGLE[3] / 2.0));

                    //
                    // Compute the product quaternion
                    //
                    //    Q = Q3*Q2*Q1
                    //
                    spicelib::QXQ(Q3.as_slice(), Q2.as_slice(), QTEMP.as_slice_mut());
                    spicelib::QXQ(QTEMP.as_slice(), Q1.as_slice(), Q.as_slice_mut());

                    //
                    // Convert Q to a rotation matrix for comparison.
                    //
                    spicelib::Q2M(Q.as_slice(), XR.as_slice_mut());

                    //
                    // How close is R to XR?
                    //
                    testutil::CHCKAD(b"R", R.as_slice(), b"~", XR.as_slice(), 3, MTOL, OK, ctx)?;

                    //
                    // --- Case: ------------------------------------------------------
                    //
                    //
                    //                 Test EUL2M.  Decompose the rotation matrix
                    //                 created using EUL2M.
                    //
                    //                 Skip the cases where AXIS2 == AXIS1 or AXIS3.
                    //
                    if ((AXIS2 != AXIS1) && (AXIS2 != AXIS3)) {
                        fstr::assign(&mut TITLE, b"Test M2EUL: Factor a rotation matrix from a random sequence of Euler angles; case = #.  Axis sequence = # # #. Angle sequence = # # #.");

                        spicelib::REPMI(&TITLE.clone(), b"#", CASE, &mut TITLE, ctx);
                        spicelib::REPMI(&TITLE.clone(), b"#", AXIS3, &mut TITLE, ctx);
                        spicelib::REPMI(&TITLE.clone(), b"#", AXIS2, &mut TITLE, ctx);
                        spicelib::REPMI(&TITLE.clone(), b"#", AXIS1, &mut TITLE, ctx);
                        spicelib::REPMD(&TITLE.clone(), b"#", XANGLE[3], 14, &mut TITLE, ctx);
                        spicelib::REPMD(&TITLE.clone(), b"#", XANGLE[2], 14, &mut TITLE, ctx);
                        spicelib::REPMD(&TITLE.clone(), b"#", XANGLE[1], 14, &mut TITLE, ctx);

                        testutil::TCASE(&TITLE, ctx)?;

                        //
                        // Decompose the matrix.
                        //
                        let [arg4, arg5, arg6] = ANGLE.get_disjoint_mut([3, 2, 1]).expect(
                            "mutable array elements passed to function must have disjoint indexes",
                        );
                        spicelib::M2EUL(R.as_slice(), AXIS3, AXIS2, AXIS1, arg4, arg5, arg6, ctx)?;

                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        //
                        // Test the recovered Euler angles.
                        //
                        testutil::CHCKAD(
                            b"Euler angles",
                            ANGLE.as_slice(),
                            b"~",
                            XANGLE.as_slice(),
                            3,
                            ANGTOL,
                            OK,
                            ctx,
                        )?;
                    }
                }
            }
        }
    }

    //
    // There is also a set of special cases for M2EUL:  those for
    // which ANGLE3 and ANGLE1 are not uniquely determined.  For
    // these cases, we expect that ANGLE3 is set to zero and ANGLE1
    // "absorbs" any contribution from ANGLE3.
    //
    // We don't need to test the numeric performance of M2EUL here
    // (we just did that), so we won't run a large number of cases.
    // We do need to make sure we try every valid axis sequence.
    //
    for AXIS1 in 1..=3 {
        for AXIS2 in 1..=3 {
            for AXIS3 in 1..=3 {
                if ((AXIS2 != AXIS1) && (AXIS2 != AXIS3)) {
                    for CASE in 1..=2 {
                        //
                        // --- Case: ------------------------------------------------------
                        //
                        //
                        //                    If we have an a-b-a rotation, we need to
                        //                    test the cases where the middle angle is zero
                        //                    or pi.  For the a-b-c rotations, we need to
                        //                    test the cases where the middle angle is
                        //                    +/- pi/2.
                        //
                        //                    We'll use the same first and third angles for
                        //                    each case.
                        //
                        XANGLE[3] = (spicelib::PI(ctx) / 6.0);
                        XANGLE[1] = (spicelib::PI(ctx) / 3.0);

                        if (AXIS3 == AXIS1) {
                            //
                            // This is the a-b-a case.
                            //
                            ABC = false;

                            if (CASE == 1) {
                                XANGLE[2] = 0.0;
                            } else {
                                XANGLE[2] = spicelib::PI(ctx);
                            }
                        } else {
                            //
                            // This is the a-b-c case.
                            //
                            ABC = true;

                            if (CASE == 1) {
                                XANGLE[2] = spicelib::HALFPI(ctx);
                            } else {
                                XANGLE[2] = -spicelib::HALFPI(ctx);
                            }
                        }

                        fstr::assign(&mut TITLE, b"Test M2EUL: Factor a rotation matrix from a sequence of Euler angles; special case = #.  Axis sequence = # # #. Angle sequence = # # #.");

                        spicelib::REPMI(&TITLE.clone(), b"#", CASE, &mut TITLE, ctx);
                        spicelib::REPMI(&TITLE.clone(), b"#", AXIS3, &mut TITLE, ctx);
                        spicelib::REPMI(&TITLE.clone(), b"#", AXIS2, &mut TITLE, ctx);
                        spicelib::REPMI(&TITLE.clone(), b"#", AXIS1, &mut TITLE, ctx);
                        spicelib::REPMD(&TITLE.clone(), b"#", XANGLE[3], 14, &mut TITLE, ctx);
                        spicelib::REPMD(&TITLE.clone(), b"#", XANGLE[2], 14, &mut TITLE, ctx);
                        spicelib::REPMD(&TITLE.clone(), b"#", XANGLE[1], 14, &mut TITLE, ctx);

                        testutil::TCASE(&TITLE, ctx)?;

                        //
                        // Create the rotation matrix.
                        //
                        spicelib::EUL2M(
                            XANGLE[3],
                            XANGLE[2],
                            XANGLE[1],
                            AXIS3,
                            AXIS2,
                            AXIS1,
                            XR.as_slice_mut(),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        //
                        // Now decompose the matrix.
                        //
                        let [arg4, arg5, arg6] = ANGLE.get_disjoint_mut([3, 2, 1]).expect(
                            "mutable array elements passed to function must have disjoint indexes",
                        );
                        spicelib::M2EUL(XR.as_slice(), AXIS3, AXIS2, AXIS1, arg4, arg5, arg6, ctx)?;

                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        //
                        // Due to round-off error, we might not have
                        // succeeded in constructing a matrix that
                        // decomposes as we expect.  If the third angle
                        // is zero, perform the test for the special case.
                        // Otherwise, just confirm that the Euler angles
                        // we found generate the original matrix.
                        //
                        if (ANGLE[3] == 0.0) {
                            //
                            // Test the recovered Euler angles.  We first
                            // adjust the expected first and third angles.
                            //
                            if ABC {
                                //
                                // The way the first and third angles
                                // combine is a bit complicated:  it
                                // depends on whether the axis sequence
                                // is right-handed.
                                //
                                RHAND =
                                    ((AXIS2 == save.NEXT[AXIS1]) && (AXIS3 == save.NEXT[AXIS2]));

                                if RHAND {
                                    if (CASE == 1) {
                                        //
                                        // The first and third angles combine
                                        // additively.
                                        //
                                        XANGLE[1] = (XANGLE[1] + XANGLE[3]);
                                    } else {
                                        //
                                        // The third angle acts as a negative
                                        // rotation about the first axis.
                                        //
                                        XANGLE[1] = (XANGLE[1] - XANGLE[3]);
                                    }
                                } else {
                                    //
                                    // For left-handed axis sequences, the
                                    // combination pattern is reversed.
                                    //
                                    if (CASE == 2) {
                                        //
                                        // The first and third angles combine
                                        // additively.
                                        //
                                        XANGLE[1] = (XANGLE[1] + XANGLE[3]);
                                    } else {
                                        //
                                        // The third angle acts as a negative
                                        // rotation about the first axis.
                                        //
                                        XANGLE[1] = (XANGLE[1] - XANGLE[3]);
                                    }
                                }
                            } else {
                                if (CASE == 1) {
                                    //
                                    // The first and third angles combine
                                    // additively.
                                    //
                                    XANGLE[1] = (XANGLE[1] + XANGLE[3]);
                                } else {
                                    //
                                    // The third angle acts as a negative
                                    // rotation about the first axis.
                                    //
                                    XANGLE[1] = (XANGLE[1] - XANGLE[3]);
                                }
                            }

                            XANGLE[3] = 0.0;

                            testutil::CHCKAD(
                                b"Euler angles",
                                ANGLE.as_slice(),
                                b"~",
                                XANGLE.as_slice(),
                                3,
                                ANGTOL,
                                OK,
                                ctx,
                            )?;
                        }

                        //
                        // Always test the angles produced by M2EUL to see
                        // whether we can recover XR from them.  This is
                        // the most basic test of validity of the angles:
                        // even when the geometry is degenerate, we should be
                        // able to find angles that correspond to the same
                        // composite rotation produced by the original angles.
                        //
                        spicelib::EUL2M(
                            ANGLE[3],
                            ANGLE[2],
                            ANGLE[1],
                            AXIS3,
                            AXIS2,
                            AXIS1,
                            R.as_slice_mut(),
                            ctx,
                        )?;

                        testutil::CHCKAD(
                            b"Recovered matrix",
                            R.as_slice(),
                            b"~",
                            XR.as_slice(),
                            9,
                            MTOL,
                            OK,
                            ctx,
                        )?;
                    }
                }
            }
        }
    }

    //
    //     Now for some error handling tests.
    //
    //
    //
    //     Error cases for EUL2M:
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"EUL2M: axis numbers out of range.", ctx)?;

    spicelib::EUL2M(ANGLE[3], ANGLE[2], ANGLE[1], 0, 1, 3, R.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADAXISNUMBERS)", OK, ctx)?;

    spicelib::EUL2M(ANGLE[3], ANGLE[2], ANGLE[1], 4, 1, 3, R.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADAXISNUMBERS)", OK, ctx)?;

    spicelib::EUL2M(ANGLE[3], ANGLE[2], ANGLE[1], 3, 0, 3, R.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADAXISNUMBERS)", OK, ctx)?;

    spicelib::EUL2M(ANGLE[3], ANGLE[2], ANGLE[1], 3, 4, 3, R.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADAXISNUMBERS)", OK, ctx)?;

    spicelib::EUL2M(ANGLE[3], ANGLE[2], ANGLE[1], 3, 1, 0, R.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADAXISNUMBERS)", OK, ctx)?;

    spicelib::EUL2M(ANGLE[3], ANGLE[2], ANGLE[1], 3, 1, 4, R.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADAXISNUMBERS)", OK, ctx)?;

    //
    //     Error cases for M2EUL:
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"M2EUL: axis numbers out of range.", ctx)?;

    let [arg4, arg5, arg6] = ANGLE
        .get_disjoint_mut([3, 2, 1])
        .expect("mutable array elements passed to function must have disjoint indexes");
    spicelib::M2EUL(R.as_slice(), 0, 1, 3, arg4, arg5, arg6, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADAXISNUMBERS)", OK, ctx)?;

    let [arg4, arg5, arg6] = ANGLE
        .get_disjoint_mut([3, 2, 1])
        .expect("mutable array elements passed to function must have disjoint indexes");
    spicelib::M2EUL(R.as_slice(), 4, 1, 3, arg4, arg5, arg6, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADAXISNUMBERS)", OK, ctx)?;

    let [arg4, arg5, arg6] = ANGLE
        .get_disjoint_mut([3, 2, 1])
        .expect("mutable array elements passed to function must have disjoint indexes");
    spicelib::M2EUL(R.as_slice(), 3, 0, 3, arg4, arg5, arg6, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADAXISNUMBERS)", OK, ctx)?;

    let [arg4, arg5, arg6] = ANGLE
        .get_disjoint_mut([3, 2, 1])
        .expect("mutable array elements passed to function must have disjoint indexes");
    spicelib::M2EUL(R.as_slice(), 3, 4, 3, arg4, arg5, arg6, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADAXISNUMBERS)", OK, ctx)?;

    let [arg4, arg5, arg6] = ANGLE
        .get_disjoint_mut([3, 2, 1])
        .expect("mutable array elements passed to function must have disjoint indexes");
    spicelib::M2EUL(R.as_slice(), 3, 1, 0, arg4, arg5, arg6, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADAXISNUMBERS)", OK, ctx)?;

    let [arg4, arg5, arg6] = ANGLE
        .get_disjoint_mut([3, 2, 1])
        .expect("mutable array elements passed to function must have disjoint indexes");
    spicelib::M2EUL(R.as_slice(), 3, 1, 4, arg4, arg5, arg6, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADAXISNUMBERS)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"M2EUL: middle axis matches first or third axis.", ctx)?;

    let [arg4, arg5, arg6] = ANGLE
        .get_disjoint_mut([3, 2, 1])
        .expect("mutable array elements passed to function must have disjoint indexes");
    spicelib::M2EUL(R.as_slice(), 3, 3, 1, arg4, arg5, arg6, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADAXISNUMBERS)", OK, ctx)?;

    let [arg4, arg5, arg6] = ANGLE
        .get_disjoint_mut([3, 2, 1])
        .expect("mutable array elements passed to function must have disjoint indexes");
    spicelib::M2EUL(R.as_slice(), 1, 3, 3, arg4, arg5, arg6, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADAXISNUMBERS)", OK, ctx)?;

    let [arg4, arg5, arg6] = ANGLE
        .get_disjoint_mut([3, 2, 1])
        .expect("mutable array elements passed to function must have disjoint indexes");
    spicelib::M2EUL(R.as_slice(), 2, 2, 1, arg4, arg5, arg6, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADAXISNUMBERS)", OK, ctx)?;

    let [arg4, arg5, arg6] = ANGLE
        .get_disjoint_mut([3, 2, 1])
        .expect("mutable array elements passed to function must have disjoint indexes");
    spicelib::M2EUL(R.as_slice(), 1, 2, 2, arg4, arg5, arg6, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADAXISNUMBERS)", OK, ctx)?;

    let [arg4, arg5, arg6] = ANGLE
        .get_disjoint_mut([3, 2, 1])
        .expect("mutable array elements passed to function must have disjoint indexes");
    spicelib::M2EUL(R.as_slice(), 1, 1, 3, arg4, arg5, arg6, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADAXISNUMBERS)", OK, ctx)?;

    let [arg4, arg5, arg6] = ANGLE
        .get_disjoint_mut([3, 2, 1])
        .expect("mutable array elements passed to function must have disjoint indexes");
    spicelib::M2EUL(R.as_slice(), 3, 1, 1, arg4, arg5, arg6, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADAXISNUMBERS)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"M2EUL:  input matrix is not a rotation.", ctx)?;

    spicelib::FILLD(1.0, 9, R.as_slice_mut());

    let [arg4, arg5, arg6] = ANGLE
        .get_disjoint_mut([3, 2, 1])
        .expect("mutable array elements passed to function must have disjoint indexes");
    spicelib::M2EUL(R.as_slice(), 3, 1, 3, arg4, arg5, arg6, ctx)?;
    testutil::CHCKXC(true, b"SPICE(NOTAROTATION)", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
