//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

fn DELTA(I: i32, J: i32) -> i32 {
    -((i32::abs((I - J)) - 1) / (i32::abs((I - J)) + 1))
}

//$Procedure      F_ZZRXR ( Check the routine zzRXR )
pub fn F_ZZRXR(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut SXFORM = StackArray3D::<f64, 45>::new(1..=3, 1..=3, 1..=5);
    let mut TEMP = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut OUTPUT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut EXPECT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut ROT1 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut ROT2 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
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
    testutil::TOPEN(b"F_ZZRXR", ctx)?;

    testutil::TCASE(
        b"Test out the case when N = 0. We should get the 3x3 identity matrix in this case.",
        ctx,
    )?;

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
                    EXPECT[[I, J]] = (DELTA(I, J) as f64);
                    OUTPUT[[I, J]] = -1.0;
                    I += m3__;
                }
            }
            J += m3__;
        }
    }
    N = 0;

    spicelib::ZZRXR(SXFORM.as_slice(), N, OUTPUT.as_slice_mut());

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"OUTPUT",
        OUTPUT.as_slice(),
        b"=",
        EXPECT.as_slice(),
        9,
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

                    EXPECT[[I, J]] = ((J * DELTA(I, J)) as f64);

                    I += m3__;
                }
            }
            J += m3__;
        }
    }

    N = 1;

    spicelib::ZZRXR(SXFORM.as_slice(), N, OUTPUT.as_slice_mut());

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"OUTPUT",
        OUTPUT.as_slice(),
        b"=",
        EXPECT.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"N = 2, Actual product computed.", ctx)?;

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
                    SXFORM[[I, J, 2]] = ROT2[[I, J]];

                    I += m3__;
                }
            }
            J += m3__;
        }
    }

    N = 2;

    spicelib::MXM(
        SXFORM.subarray([1, 1, 2]),
        SXFORM.subarray([1, 1, 1]),
        EXPECT.as_slice_mut(),
    );
    spicelib::ZZRXR(SXFORM.as_slice(), N, OUTPUT.as_slice_mut());

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"OUTPUT",
        OUTPUT.as_slice(),
        b"=",
        EXPECT.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Three matrices multiplied together", ctx)?;

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

                    I += m3__;
                }
            }
            J += m3__;
        }
    }

    N = 3;

    spicelib::MXM(
        SXFORM.subarray([1, 1, 3]),
        SXFORM.subarray([1, 1, 2]),
        TEMP.as_slice_mut(),
    );
    spicelib::MXM(
        TEMP.as_slice(),
        SXFORM.subarray([1, 1, 1]),
        EXPECT.as_slice_mut(),
    );
    spicelib::ZZRXR(SXFORM.as_slice(), N, OUTPUT.as_slice_mut());

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"OUTPUT",
        OUTPUT.as_slice(),
        b"=",
        EXPECT.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Four matrices multiplied together", ctx)?;

    N = 4;

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

                    I += m3__;
                }
            }
            J += m3__;
        }
    }

    spicelib::MXM(
        SXFORM.subarray([1, 1, 4]),
        SXFORM.subarray([1, 1, 3]),
        EXPECT.as_slice_mut(),
    );
    spicelib::MXM(
        EXPECT.as_slice(),
        SXFORM.subarray([1, 1, 2]),
        TEMP.as_slice_mut(),
    );
    spicelib::MXM(
        TEMP.as_slice(),
        SXFORM.subarray([1, 1, 1]),
        EXPECT.as_slice_mut(),
    );

    spicelib::ZZRXR(SXFORM.as_slice(), N, OUTPUT.as_slice_mut());

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"OUTPUT",
        OUTPUT.as_slice(),
        b"=",
        EXPECT.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Five matrices multiplied together", ctx)?;

    N = 5;

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
                    I += m3__;
                }
            }
            J += m3__;
        }
    }

    spicelib::MXM(
        SXFORM.subarray([1, 1, 5]),
        SXFORM.subarray([1, 1, 4]),
        TEMP.as_slice_mut(),
    );
    spicelib::MXM(
        TEMP.as_slice(),
        SXFORM.subarray([1, 1, 3]),
        EXPECT.as_slice_mut(),
    );
    spicelib::MXM(
        EXPECT.as_slice(),
        SXFORM.subarray([1, 1, 2]),
        TEMP.as_slice_mut(),
    );
    spicelib::MXM(
        TEMP.as_slice(),
        SXFORM.subarray([1, 1, 1]),
        EXPECT.as_slice_mut(),
    );

    spicelib::ZZRXR(SXFORM.as_slice(), N, OUTPUT.as_slice_mut());

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"OUTPUT",
        OUTPUT.as_slice(),
        b"=",
        EXPECT.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
