//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

fn D(I: i32, J: i32) -> i32 {
    i32::abs(((i32::abs((I - J)) - 1) / (i32::abs((I - J)) + 1)))
}

//$Procedure      F_XFNEUL ( State transformations and Euler angles)
pub fn F_XFNEUL(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut AV = StackArray::<f64, 3>::new(1..=3);
    let mut AXIS = StackArray3D::<f64, 27>::new(1..=3, 1..=3, 1..=3);
    let mut CEULER = StackArray::<f64, 6>::new(1..=6);
    let mut DIAG = StackArray3D::<f64, 27>::new(1..=3, 1..=3, 1..=3);
    let mut EULER = StackArray::<f64, 6>::new(1..=6);
    let mut OMEGA = StackArray3D::<f64, 27>::new(1..=3, 1..=3, 1..=3);
    let mut ROT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut XEULER = StackArray::<f64, 6>::new(1..=6);
    let mut XF = StackArray3D::<f64, 108>::new(1..=6, 1..=6, 1..=3);
    let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XPECT = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XTEMP = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut A = StackArray::<i32, 3>::new(1..=3);
    let mut I: i32 = 0;
    let mut J: i32 = 0;
    let mut UNIQUE: bool = false;

    //
    // Test Utility Functions
    //

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_XFNEUL", ctx)?;
    // Validate the computation of state transformation from
    // Euler angles and derivatives.
    //

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = 3;
                let m3__: i32 = 1;
                J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    for K in 1..=3 {
                        OMEGA[[I, J, K]] = 0.0;
                        AXIS[[I, J, K]] = 0.0;
                        DIAG[[I, J, K]] = 0.0;
                    }
                    J += m3__;
                }
            }
            I += m3__;
        }
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = 3;
                let m3__: i32 = 1;
                J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    DIAG[[J, J, I]] = (1.0 - (D(I, J) as f64));
                    AXIS[[J, J, I]] = (D(I, J) as f64);
                    J += m3__;
                }
            }
            I += m3__;
        }
    }

    OMEGA[[1, 2, 3]] = 1.0;
    OMEGA[[2, 3, 1]] = 1.0;
    OMEGA[[3, 1, 2]] = 1.0;

    OMEGA[[3, 2, 1]] = -1.0;
    OMEGA[[2, 1, 3]] = -1.0;
    OMEGA[[1, 3, 2]] = -1.0;

    testutil::TCASE(b"Validate the computation of state transformation from Euler angles and derivatives. Every possible combination of axes is tested. ", ctx)?;

    EULER[1] = 0.33;
    EULER[2] = -0.2;
    EULER[3] = 0.5;
    EULER[4] = -0.3;
    EULER[5] = 0.1;
    EULER[6] = 0.7;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = 3;
                let m3__: i32 = 1;
                J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    for K in 1..=3 {
                        //
                        // We construct the state transformation matrix
                        // from scratch and compare that to the results
                        // returned by EUL2XF.
                        //
                        A[1] = I;
                        A[2] = J;
                        A[3] = K;

                        //
                        // Set the expected state transformation to the identity
                        // to begin with.
                        //
                        spicelib::CLEARD(36, XPECT.as_slice_mut());

                        for M in 1..=6 {
                            XPECT[[M, M]] = 1.0;
                        }

                        for M in 1..=3 {
                            //
                            // Construct the state transformation for the Mth
                            // rotation in the sequence.  We start out with
                            // the rotation matrix and angular velocity vector.
                            //
                            spicelib::ROTATE(EULER[M], A[M], ROT.as_slice_mut(), ctx);

                            //
                            // Set the angular velocity vector:  the component
                            // corresponding to axis M is the Mth rate; the other
                            // components are zero.
                            //
                            spicelib::CLEARD(3, AV.as_slice_mut());

                            AV[A[M]] = EULER[(M + 3)];

                            spicelib::RAV2XF(
                                ROT.as_slice(),
                                AV.as_slice(),
                                XF.subarray_mut([1, 1, M]),
                            );

                            spicelib::MXMG(
                                XPECT.as_slice(),
                                XF.subarray([1, 1, M]),
                                6,
                                6,
                                6,
                                XTEMP.as_slice_mut(),
                            );
                            spicelib::MOVED(XTEMP.as_slice(), 36, XPECT.as_slice_mut());
                        }

                        spicelib::EUL2XF(EULER.as_slice(), I, J, K, XFORM.as_slice_mut(), ctx)?;

                        testutil::TSTMSG(b"#", b"Rotation is a #-#-# ", ctx);
                        testutil::TSTMSI(I, ctx);
                        testutil::TSTMSI(J, ctx);
                        testutil::TSTMSI(K, ctx);

                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                        testutil::CHCKAD(
                            b"XFORM",
                            XFORM.as_slice(),
                            b"~",
                            XPECT.as_slice(),
                            36,
                            0.00000000000002,
                            OK,
                            ctx,
                        )?;
                    }

                    J += m3__;
                }
            }

            I += m3__;
        }
    }

    testutil::TCASE(b"Validate the computation of euler angles and derivatives from the state transformation matrix. Every combination of rotation axes is exercised. ", ctx)?;

    EULER[1] = 0.33;
    EULER[2] = 0.2;
    EULER[3] = 0.5;
    EULER[4] = 0.3;
    EULER[5] = 0.1;
    EULER[6] = 0.7;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = 3;
                let m3__: i32 = 1;
                J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    if (I != J) {
                        for K in 1..=3 {
                            if (K != J) {
                                spicelib::EUL2XF(
                                    EULER.as_slice(),
                                    I,
                                    J,
                                    K,
                                    XFORM.as_slice_mut(),
                                    ctx,
                                )?;
                                spicelib::XF2EUL(
                                    XFORM.as_slice(),
                                    I,
                                    J,
                                    K,
                                    CEULER.as_slice_mut(),
                                    &mut UNIQUE,
                                    ctx,
                                )?;

                                testutil::TSTMSG(b"#", b"Rotation is a #-#-# ", ctx);
                                testutil::TSTMSI(I, ctx);
                                testutil::TSTMSI(J, ctx);
                                testutil::TSTMSI(K, ctx);

                                testutil::CHCKXC(false, b" ", OK, ctx)?;
                                testutil::CHCKSL(b"UNIQUE", UNIQUE, true, OK, ctx)?;
                                testutil::CHCKAD(
                                    b"XFORM",
                                    CEULER.as_slice(),
                                    b"~",
                                    EULER.as_slice(),
                                    6,
                                    0.00000000000002,
                                    OK,
                                    ctx,
                                )?;
                            }
                        }
                    }
                    J += m3__;
                }
            }

            I += m3__;
        }
    }

    testutil::TCASE(
        b"Exercise the degenerate cases where the second angle is nearly zero. ",
        ctx,
    )?;

    EULER[1] = 0.0;
    EULER[2] = 0.0000000001;
    EULER[3] = 0.5;
    EULER[4] = 0.0;
    EULER[5] = 0.1;
    EULER[6] = 0.7;

    XEULER[1] = 0.0;
    XEULER[2] = 0.0;
    XEULER[3] = 0.5;
    XEULER[4] = 0.0;
    XEULER[5] = 0.1;
    XEULER[6] = 0.7;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = 3;
                let m3__: i32 = 1;
                J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    if (I != J) {
                        for K in 1..=3 {
                            if (K == I) {
                                spicelib::EUL2XF(
                                    EULER.as_slice(),
                                    I,
                                    J,
                                    K,
                                    XFORM.as_slice_mut(),
                                    ctx,
                                )?;
                                spicelib::XF2EUL(
                                    XFORM.as_slice(),
                                    I,
                                    J,
                                    K,
                                    CEULER.as_slice_mut(),
                                    &mut UNIQUE,
                                    ctx,
                                )?;

                                testutil::TSTMSG(b"#", b"Rotation is a #-#-# ", ctx);
                                testutil::TSTMSI(I, ctx);
                                testutil::TSTMSI(J, ctx);
                                testutil::TSTMSI(K, ctx);

                                testutil::CHCKXC(false, b" ", OK, ctx)?;
                                testutil::CHCKSL(b"UNIQUE", UNIQUE, false, OK, ctx)?;
                                testutil::CHCKAD(
                                    b"XFORM",
                                    CEULER.as_slice(),
                                    b"~",
                                    XEULER.as_slice(),
                                    6,
                                    0.00000000000002,
                                    OK,
                                    ctx,
                                )?;
                            }
                        }
                    }
                    J += m3__;
                }
            }

            I += m3__;
        }
    }

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
