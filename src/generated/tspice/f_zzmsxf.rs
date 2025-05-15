//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

fn DELTA(I: i32, J: i32) -> i32 {
    -((i32::abs((I - J)) - 1) / (i32::abs((I - J)) + 1))
}

//$Procedure      F_ZZMSXF ( Check the routine zzmsxf )
pub fn F_ZZMSXF(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut SXFORM = StackArray3D::<f64, 180>::new(1..=6, 1..=6, 1..=5);
    let mut TEMP = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut OUTPUT = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut EXPECT = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut ROT1 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut ROT2 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut DROT1 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut DROT2 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut I: i32 = 0;
    let mut J: i32 = 0;
    let mut N: i32 = 0;

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
    testutil::TOPEN(b"F_ZZMSXF", ctx)?;

    testutil::TCASE(
        b"Test out the case when N = 0. We should get the 6x6 identity matrix in this case.",
        ctx,
    )?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 6;
        let m3__: i32 = 1;
        J = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = 6;
                let m3__: i32 = 1;
                I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    EXPECT[[I, J]] = (DELTA(I, J) as f64);
                    OUTPUT[[I, J]] = -1.0;
                    I += m3__;
                }
            }
            J += m3__;
        }
    }
    N = 0;

    spicelib::ZZMSXF(SXFORM.as_slice(), N, OUTPUT.as_slice_mut());

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"OUTPUT",
        OUTPUT.as_slice(),
        b"=",
        EXPECT.as_slice(),
        36,
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(
        b"Test the case when N = 1.  For this case the output should be a copy of the input.",
        ctx,
    )?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 6;
        let m3__: i32 = 1;
        J = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = 6;
                let m3__: i32 = 1;
                I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    OUTPUT[[I, J]] = -1.0;
                    I += m3__;
                }
            }
            J += m3__;
        }
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        J = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = 3;
                let m3__: i32 = 1;
                I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    SXFORM[[I, J, 1]] = ((J * DELTA(I, J)) as f64);
                    SXFORM[[(I + 3), (J + 3), 1]] = ((J * DELTA(I, J)) as f64);
                    SXFORM[[(I + 3), J, 1]] = (DELTA(I, J) as f64);
                    SXFORM[[I, (J + 3), 1]] = 0.0;

                    EXPECT[[I, J]] = ((J * DELTA(I, J)) as f64);
                    EXPECT[[(I + 3), (J + 3)]] = ((J * DELTA(I, J)) as f64);
                    EXPECT[[(I + 3), J]] = (DELTA(I, J) as f64);
                    EXPECT[[I, (J + 3)]] = 0.0;

                    I += m3__;
                }
            }
            J += m3__;
        }
    }

    N = 1;

    spicelib::ZZMSXF(SXFORM.as_slice(), N, OUTPUT.as_slice_mut());

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"OUTPUT",
        OUTPUT.as_slice(),
        b"=",
        EXPECT.as_slice(),
        36,
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"N = 2, Actual product computed.", ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 6;
        let m3__: i32 = 1;
        J = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = 6;
                let m3__: i32 = 1;
                I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    OUTPUT[[I, J]] = -1.0;
                    I += m3__;
                }
            }
            J += m3__;
        }
    }

    ROT1[[1, 1]] = 1.0;
    ROT1[[2, 1]] = 2.0;
    ROT1[[3, 1]] = 3.0;

    ROT1[[1, 2]] = 4.0;
    ROT1[[2, 2]] = 5.0;
    ROT1[[3, 2]] = 6.0;

    ROT1[[1, 3]] = 7.0;
    ROT1[[2, 3]] = 8.0;
    ROT1[[3, 3]] = 9.0;

    ROT2[[1, 1]] = 1.0;
    ROT2[[2, 1]] = 2.0;
    ROT2[[3, 1]] = 3.0;

    ROT2[[1, 2]] = 2.0;
    ROT2[[2, 2]] = 3.0;
    ROT2[[3, 2]] = 1.0;

    ROT2[[1, 3]] = 3.0;
    ROT2[[2, 3]] = 1.0;
    ROT2[[3, 3]] = 2.0;

    DROT1[[1, 1]] = 1.0;
    DROT1[[2, 1]] = 0.0;
    DROT1[[3, 1]] = 0.0;

    DROT1[[1, 2]] = 0.0;
    DROT1[[2, 2]] = 1.0;
    DROT1[[3, 2]] = 0.0;

    DROT1[[1, 3]] = 0.0;
    DROT1[[2, 3]] = 0.0;
    DROT1[[3, 3]] = 1.0;

    DROT2[[1, 1]] = 1.0;
    DROT2[[2, 1]] = 2.0;
    DROT2[[3, 1]] = 3.0;

    DROT2[[1, 2]] = 2.0;
    DROT2[[2, 2]] = 3.0;
    DROT2[[3, 2]] = 1.0;

    DROT2[[1, 3]] = 3.0;
    DROT2[[2, 3]] = 1.0;
    DROT2[[3, 3]] = 2.0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        J = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = 3;
                let m3__: i32 = 1;
                I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    SXFORM[[I, J, 1]] = ROT1[[I, J]];
                    SXFORM[[(I + 3), (J + 3), 1]] = ROT1[[I, J]];
                    SXFORM[[I, (J + 3), 1]] = 0.0;
                    SXFORM[[(I + 3), J, 1]] = DROT1[[I, J]];

                    SXFORM[[I, J, 2]] = ROT2[[I, J]];
                    SXFORM[[(I + 3), (J + 3), 2]] = ROT2[[I, J]];
                    SXFORM[[I, (J + 3), 2]] = 0.0;
                    SXFORM[[(I + 3), J, 2]] = DROT2[[I, J]];

                    I += m3__;
                }
            }
            J += m3__;
        }
    }

    spicelib::MXMG(
        SXFORM.subarray([1, 1, 2]),
        SXFORM.subarray([1, 1, 1]),
        6,
        6,
        6,
        EXPECT.as_slice_mut(),
    );
    spicelib::ZZMSXF(SXFORM.as_slice(), N, OUTPUT.as_slice_mut());

    testutil::TCASE(b"Three matrices multiplied together", ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 6;
        let m3__: i32 = 1;
        J = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = 6;
                let m3__: i32 = 1;
                I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    OUTPUT[[I, J]] = -1.0;
                    I += m3__;
                }
            }
            J += m3__;
        }
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        J = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = 3;
                let m3__: i32 = 1;
                I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    SXFORM[[I, J, 3]] = ROT1[[I, J]];
                    SXFORM[[(I + 3), (J + 3), 3]] = ROT1[[I, J]];
                    SXFORM[[I, (J + 3), 3]] = 0.0;
                    SXFORM[[(I + 3), J, 3]] = DROT1[[I, J]];

                    I += m3__;
                }
            }
            J += m3__;
        }
    }

    N = 3;

    spicelib::MXMG(
        SXFORM.subarray([1, 1, 3]),
        SXFORM.subarray([1, 1, 2]),
        6,
        6,
        6,
        TEMP.as_slice_mut(),
    );
    spicelib::MXMG(
        TEMP.as_slice(),
        SXFORM.subarray([1, 1, 1]),
        6,
        6,
        6,
        EXPECT.as_slice_mut(),
    );
    spicelib::ZZMSXF(SXFORM.as_slice(), N, OUTPUT.as_slice_mut());

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"OUTPUT",
        OUTPUT.as_slice(),
        b"=",
        EXPECT.as_slice(),
        36,
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Four matrices multiplied together", ctx)?;

    N = 4;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 6;
        let m3__: i32 = 1;
        J = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = 6;
                let m3__: i32 = 1;
                I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    OUTPUT[[I, J]] = -1.0;
                    I += m3__;
                }
            }
            J += m3__;
        }
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        J = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = 3;
                let m3__: i32 = 1;
                I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    SXFORM[[I, J, 4]] = ROT2[[I, J]];
                    SXFORM[[(I + 3), (J + 3), 4]] = ROT2[[I, J]];
                    SXFORM[[I, (J + 3), 4]] = 0.0;
                    SXFORM[[(I + 3), J, 4]] = DROT2[[I, J]];

                    I += m3__;
                }
            }
            J += m3__;
        }
    }

    spicelib::MXMG(
        SXFORM.subarray([1, 1, 4]),
        SXFORM.subarray([1, 1, 3]),
        6,
        6,
        6,
        EXPECT.as_slice_mut(),
    );
    spicelib::MXMG(
        EXPECT.as_slice(),
        SXFORM.subarray([1, 1, 2]),
        6,
        6,
        6,
        TEMP.as_slice_mut(),
    );
    spicelib::MXMG(
        TEMP.as_slice(),
        SXFORM.subarray([1, 1, 1]),
        6,
        6,
        6,
        EXPECT.as_slice_mut(),
    );

    spicelib::ZZMSXF(SXFORM.as_slice(), N, OUTPUT.as_slice_mut());

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"OUTPUT",
        OUTPUT.as_slice(),
        b"=",
        EXPECT.as_slice(),
        36,
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Five matrices multiplied together", ctx)?;

    N = 5;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 6;
        let m3__: i32 = 1;
        J = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = 6;
                let m3__: i32 = 1;
                I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    OUTPUT[[I, J]] = -1.0;
                    I += m3__;
                }
            }
            J += m3__;
        }
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        J = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = 3;
                let m3__: i32 = 1;
                I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    SXFORM[[I, J, 5]] = ROT1[[I, J]];
                    SXFORM[[(I + 3), (J + 3), 5]] = ROT1[[I, J]];
                    SXFORM[[I, (J + 3), 5]] = 0.0;
                    SXFORM[[(I + 3), J, 5]] = DROT1[[I, J]];

                    I += m3__;
                }
            }
            J += m3__;
        }
    }

    spicelib::MXMG(
        SXFORM.subarray([1, 1, 5]),
        SXFORM.subarray([1, 1, 4]),
        6,
        6,
        6,
        TEMP.as_slice_mut(),
    );
    spicelib::MXMG(
        TEMP.as_slice(),
        SXFORM.subarray([1, 1, 3]),
        6,
        6,
        6,
        EXPECT.as_slice_mut(),
    );
    spicelib::MXMG(
        EXPECT.as_slice(),
        SXFORM.subarray([1, 1, 2]),
        6,
        6,
        6,
        TEMP.as_slice_mut(),
    );
    spicelib::MXMG(
        TEMP.as_slice(),
        SXFORM.subarray([1, 1, 1]),
        6,
        6,
        6,
        EXPECT.as_slice_mut(),
    );

    spicelib::ZZMSXF(SXFORM.as_slice(), N, OUTPUT.as_slice_mut());

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"OUTPUT",
        OUTPUT.as_slice(),
        b"=",
        EXPECT.as_slice(),
        36,
        0.0,
        OK,
        ctx,
    )?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
