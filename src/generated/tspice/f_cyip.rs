//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CSIZE: i32 = 10;
const DSIZE: i32 = 100;
const ISIZE: i32 = 25;
const STLEN: i32 = 80;
const LNSIZE: i32 = 80;

//$Procedure F_CYIP ( Cycle array in place tests )
pub fn F_CYIP(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut CARRAY = ActualCharArray::new(STLEN, 1..=CSIZE);
    let mut C2ARRY = ActualCharArray::new(STLEN, 1..=CSIZE);
    let mut LABEL = [b' '; LNSIZE as usize];
    let mut XCARRY = ActualCharArray::new(STLEN, 1..=CSIZE);
    let mut DARRAY = StackArray::<f64, 100>::new(1..=DSIZE);
    let mut D2ARRY = StackArray::<f64, 100>::new(1..=DSIZE);
    let mut XDARRY = StackArray::<f64, 100>::new(1..=DSIZE);
    let mut IARRAY = StackArray::<i32, 25>::new(1..=ISIZE);
    let mut I2ARRY = StackArray::<i32, 25>::new(1..=ISIZE);
    let mut Q: i32 = 0;
    let mut R: i32 = 0;
    let mut XIARRY = StackArray::<i32, 25>::new(1..=ISIZE);

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_CYIP", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test CYAIIP:  cycle forward.", ctx)?;

    for I in 1..=ISIZE {
        IARRAY[I] = I;
    }

    for I in 1..=(2 * ISIZE) {
        //
        // Use CYCLAI to find the expected result.
        //
        spicelib::CYCLAI(
            IARRAY.as_slice(),
            ISIZE,
            b"F",
            I,
            XIARRY.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Also cycle the array in place.
        //
        spicelib::MOVEI(IARRAY.as_slice(), ISIZE, I2ARRY.as_slice_mut());

        spicelib::CYAIIP(ISIZE, b"F", I, I2ARRY.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            b"I2ARRY",
            I2ARRY.as_slice(),
            b"=",
            XIARRY.as_slice(),
            ISIZE,
            OK,
            ctx,
        )?;

        //
        // Just to be careful, let's test CYCLAI itself.  Create
        // the expected result of cycling the array forward by I.
        //
        spicelib::CLEARI(ISIZE, I2ARRY.as_slice_mut());

        for J in 1..=ISIZE {
            //
            // Find the target location of the Jth element of IARRAY.
            //
            spicelib::RMAINI((J + I), ISIZE, &mut Q, &mut R, ctx)?;

            if (R == 0) {
                R = ISIZE;
            }

            I2ARRY[R] = IARRAY[J];
        }

        testutil::CHCKAI(
            b"I2ARRY",
            I2ARRY.as_slice(),
            b"=",
            XIARRY.as_slice(),
            ISIZE,
            OK,
            ctx,
        )?;

        //
        // Cycle backward by -I as well.
        //
        spicelib::CYCLAI(
            IARRAY.as_slice(),
            ISIZE,
            b"B",
            -I,
            XIARRY.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            b"I2ARRY",
            I2ARRY.as_slice(),
            b"=",
            XIARRY.as_slice(),
            ISIZE,
            OK,
            ctx,
        )?;

        //
        // Cycle in the opposite direction using a negative count.
        //
        spicelib::MOVEI(IARRAY.as_slice(), ISIZE, I2ARRY.as_slice_mut());
        spicelib::CYAIIP(ISIZE, b"B", -I, I2ARRY.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            b"I2ARRY",
            I2ARRY.as_slice(),
            b"=",
            XIARRY.as_slice(),
            ISIZE,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test CYAIIP:  cycle backward.", ctx)?;

    for I in 1..=(2 * ISIZE) {
        //
        // Use CYCLAI to find the expected result.
        //
        spicelib::CYCLAI(
            IARRAY.as_slice(),
            ISIZE,
            b"B",
            I,
            XIARRY.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Also cycle the array in place.
        //
        spicelib::MOVEI(IARRAY.as_slice(), ISIZE, I2ARRY.as_slice_mut());

        spicelib::CYAIIP(ISIZE, b"B", I, I2ARRY.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            b"I2ARRY",
            I2ARRY.as_slice(),
            b"=",
            XIARRY.as_slice(),
            ISIZE,
            OK,
            ctx,
        )?;

        //
        // Just to be careful, let's test CYCLAI itself.  Create
        // the expected result of cycling the array backward by I.
        //
        spicelib::CLEARI(ISIZE, I2ARRY.as_slice_mut());

        for J in 1..=ISIZE {
            //
            // Find the target location of the Jth element of IARRAY.
            //
            spicelib::RMAINI((J - I), ISIZE, &mut Q, &mut R, ctx)?;

            if (R == 0) {
                R = ISIZE;
            }

            I2ARRY[R] = IARRAY[J];
        }

        testutil::CHCKAI(
            b"I2ARRY",
            I2ARRY.as_slice(),
            b"=",
            XIARRY.as_slice(),
            ISIZE,
            OK,
            ctx,
        )?;

        //
        // Cycle forward by -I as well.
        //
        spicelib::CYCLAI(
            IARRAY.as_slice(),
            ISIZE,
            b"F",
            -I,
            XIARRY.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            b"I2ARRY",
            I2ARRY.as_slice(),
            b"=",
            XIARRY.as_slice(),
            ISIZE,
            OK,
            ctx,
        )?;

        //
        // Cycle in the opposite direction using a negative count.
        //
        spicelib::MOVEI(IARRAY.as_slice(), ISIZE, I2ARRY.as_slice_mut());
        spicelib::CYAIIP(ISIZE, b"F", -I, I2ARRY.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            b"I2ARRY",
            I2ARRY.as_slice(),
            b"=",
            XIARRY.as_slice(),
            ISIZE,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     When the array size is non-positive, the call should be
    //     a no-op.
    //
    //
    //     Use a negative size.
    //
    testutil::TCASE(b"Error case:  non-positive array size", ctx)?;

    spicelib::MOVEI(IARRAY.as_slice(), ISIZE, I2ARRY.as_slice_mut());

    spicelib::CYAIIP(-ISIZE, b"F", -5, I2ARRY.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAI(
        b"I2ARRY",
        I2ARRY.as_slice(),
        b"=",
        IARRAY.as_slice(),
        ISIZE,
        OK,
        ctx,
    )?;

    //
    // Use size zero.
    //
    spicelib::MOVEI(IARRAY.as_slice(), ISIZE, I2ARRY.as_slice_mut());

    spicelib::CYAIIP(0, b"F", -5, I2ARRY.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAI(
        b"I2ARRY",
        I2ARRY.as_slice(),
        b"=",
        IARRAY.as_slice(),
        ISIZE,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case:  invalid direction.", ctx)?;

    spicelib::CYAIIP(ISIZE, b"Z", -5, I2ARRY.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDDIRECTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test CYADIP:  cycle forward.", ctx)?;

    for I in 1..=DSIZE {
        DARRAY[I] = I as f64;
    }

    for I in 1..=(2 * DSIZE) {
        //
        // Use CYCLAD to find the expected result.
        //
        spicelib::CYCLAD(
            DARRAY.as_slice(),
            DSIZE,
            b"F",
            I,
            XDARRY.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Also cycle the array in place.
        //
        spicelib::MOVED(DARRAY.as_slice(), DSIZE, D2ARRY.as_slice_mut());

        spicelib::CYADIP(DSIZE, b"F", I, D2ARRY.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"D2ARRY",
            D2ARRY.as_slice(),
            b"=",
            XDARRY.as_slice(),
            DSIZE,
            0.0,
            OK,
            ctx,
        )?;

        //
        // Just to be careful, let's test CYCLAD itself.  Create
        // the expected result of cycling the array forward by I.
        //
        spicelib::CLEARD(DSIZE, D2ARRY.as_slice_mut());

        for J in 1..=DSIZE {
            //
            // Find the target location of the Jth element of DARRAY.
            //
            spicelib::RMAINI((J + I), DSIZE, &mut Q, &mut R, ctx)?;

            if (R == 0) {
                R = DSIZE;
            }

            D2ARRY[R] = DARRAY[J];
        }

        testutil::CHCKAD(
            b"D2ARRY",
            D2ARRY.as_slice(),
            b"=",
            XDARRY.as_slice(),
            DSIZE,
            0.0,
            OK,
            ctx,
        )?;

        //
        // Cycle backward by -I as well.
        //
        spicelib::CYCLAD(
            DARRAY.as_slice(),
            DSIZE,
            b"B",
            -I,
            XDARRY.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"D2ARRY",
            D2ARRY.as_slice(),
            b"=",
            XDARRY.as_slice(),
            DSIZE,
            0.0,
            OK,
            ctx,
        )?;

        //
        // Cycle in the opposite direction using a negative count.
        //
        spicelib::MOVED(DARRAY.as_slice(), DSIZE, D2ARRY.as_slice_mut());
        spicelib::CYADIP(DSIZE, b"B", -I, D2ARRY.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"D2ARRY",
            D2ARRY.as_slice(),
            b"=",
            XDARRY.as_slice(),
            DSIZE,
            0.0,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test CYADIP:  cycle backward.", ctx)?;

    for I in 1..=(2 * DSIZE) {
        //
        // Use CYCLAD to find the expected result.
        //
        spicelib::CYCLAD(
            DARRAY.as_slice(),
            DSIZE,
            b"B",
            I,
            XDARRY.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Also cycle the array in place.
        //
        spicelib::MOVED(DARRAY.as_slice(), DSIZE, D2ARRY.as_slice_mut());

        spicelib::CYADIP(DSIZE, b"B", I, D2ARRY.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"D2ARRY",
            D2ARRY.as_slice(),
            b"=",
            XDARRY.as_slice(),
            DSIZE,
            0.0,
            OK,
            ctx,
        )?;

        //
        // Just to be careful, let's test CYCLAD itself.  Create
        // the expected result of cycling the array backward by I.
        //
        spicelib::CLEARD(DSIZE, D2ARRY.as_slice_mut());

        for J in 1..=DSIZE {
            //
            // Find the target location of the Jth element of DARRAY.
            //
            spicelib::RMAINI((J - I), DSIZE, &mut Q, &mut R, ctx)?;

            if (R == 0) {
                R = DSIZE;
            }

            D2ARRY[R] = DARRAY[J];
        }

        testutil::CHCKAD(
            b"D2ARRY",
            D2ARRY.as_slice(),
            b"=",
            XDARRY.as_slice(),
            DSIZE,
            0.0,
            OK,
            ctx,
        )?;

        //
        // Cycle forward by -I as well.
        //
        spicelib::CYCLAD(
            DARRAY.as_slice(),
            DSIZE,
            b"F",
            -I,
            XDARRY.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"D2ARRY",
            D2ARRY.as_slice(),
            b"=",
            XDARRY.as_slice(),
            DSIZE,
            0.0,
            OK,
            ctx,
        )?;

        //
        // Cycle in the opposite direction using a negative count.
        //
        spicelib::MOVED(DARRAY.as_slice(), DSIZE, D2ARRY.as_slice_mut());
        spicelib::CYADIP(DSIZE, b"F", -I, D2ARRY.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"D2ARRY",
            D2ARRY.as_slice(),
            b"=",
            XDARRY.as_slice(),
            DSIZE,
            0.0,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     When the array size is non-positive, the call should be
    //     a no-op.
    //
    //
    //     Use a negative size.
    //
    testutil::TCASE(b"Error case:  non-positive array size", ctx)?;

    spicelib::MOVED(DARRAY.as_slice(), DSIZE, D2ARRY.as_slice_mut());

    spicelib::CYADIP(-DSIZE, b"F", -5, D2ARRY.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"D2ARRY",
        D2ARRY.as_slice(),
        b"=",
        DARRAY.as_slice(),
        DSIZE,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Use size zero.
    //
    spicelib::MOVED(DARRAY.as_slice(), DSIZE, D2ARRY.as_slice_mut());

    spicelib::CYADIP(0, b"F", -5, D2ARRY.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"D2ARRY",
        D2ARRY.as_slice(),
        b"=",
        DARRAY.as_slice(),
        DSIZE,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case:  invalid direction.", ctx)?;

    spicelib::CYADIP(DSIZE, b"Z", -5, D2ARRY.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDDIRECTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test CYACIP:  cycle forward.", ctx)?;

    for I in 1..=CSIZE {
        spicelib::INTSTR(I, &mut CARRAY[I], ctx);
    }

    for I in 1..=(2 * CSIZE) {
        //
        // Use CYCLAC to find the expected result.
        //
        spicelib::CYCLAC(CARRAY.as_arg(), CSIZE, b"F", I, XCARRY.as_arg_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Also cycle the array in place.
        //
        spicelib::MOVEC(CARRAY.as_arg(), CSIZE, C2ARRY.as_arg_mut());

        spicelib::CYACIP(CSIZE, b"F", I, C2ARRY.as_arg_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        for J in 1..=CSIZE {
            fstr::assign(&mut LABEL, b"C2ARRY(#)");
            spicelib::REPMI(&LABEL.clone(), b"#", J, &mut LABEL, ctx);
            testutil::CHCKSC(&LABEL, &C2ARRY[J], b"=", &XCARRY[J], OK, ctx)?;
        }

        //
        // Just to be careful, let's test CYCLAC itself.  Create
        // the expected result of cycling the array forward by I.
        //
        spicelib::CLEARC(CSIZE, C2ARRY.as_arg_mut());

        for J in 1..=CSIZE {
            //
            // Find the target location of the Jth element of CARRAY.
            //
            spicelib::RMAINI((J + I), CSIZE, &mut Q, &mut R, ctx)?;

            if (R == 0) {
                R = CSIZE;
            }

            fstr::assign(C2ARRY.get_mut(R), CARRAY.get(J));
        }

        for J in 1..=CSIZE {
            fstr::assign(&mut LABEL, b"C2ARRY(#)");
            spicelib::REPMI(&LABEL.clone(), b"#", J, &mut LABEL, ctx);
            testutil::CHCKSC(&LABEL, &C2ARRY[J], b"=", &XCARRY[J], OK, ctx)?;
        }

        //
        // Cycle backward by -I as well.
        //
        spicelib::CYCLAC(CARRAY.as_arg(), CSIZE, b"B", -I, XCARRY.as_arg_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        for J in 1..=CSIZE {
            fstr::assign(&mut LABEL, b"C2ARRY(#)");
            spicelib::REPMI(&LABEL.clone(), b"#", J, &mut LABEL, ctx);
            testutil::CHCKSC(&LABEL, &C2ARRY[J], b"=", &XCARRY[J], OK, ctx)?;
        }

        //
        // Cycle in the opposite direction using a negative count.
        //
        spicelib::MOVEC(CARRAY.as_arg(), CSIZE, C2ARRY.as_arg_mut());
        spicelib::CYACIP(CSIZE, b"B", -I, C2ARRY.as_arg_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        for J in 1..=CSIZE {
            fstr::assign(&mut LABEL, b"C2ARRY(#)");
            spicelib::REPMI(&LABEL.clone(), b"#", J, &mut LABEL, ctx);
            testutil::CHCKSC(&LABEL, &C2ARRY[J], b"=", &XCARRY[J], OK, ctx)?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test CYACIP:  cycle backward.", ctx)?;

    for I in 1..=(2 * CSIZE) {
        //
        // Use CYCLAC to find the expected result.
        //
        spicelib::CYCLAC(CARRAY.as_arg(), CSIZE, b"B", I, XCARRY.as_arg_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Also cycle the array in place.
        //
        spicelib::MOVEC(CARRAY.as_arg(), CSIZE, C2ARRY.as_arg_mut());

        spicelib::CYACIP(CSIZE, b"B", I, C2ARRY.as_arg_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        for J in 1..=CSIZE {
            fstr::assign(&mut LABEL, b"C2ARRY(#)");
            spicelib::REPMI(&LABEL.clone(), b"#", J, &mut LABEL, ctx);
            testutil::CHCKSC(b"C2ARRY", &C2ARRY[J], b"=", &XCARRY[J], OK, ctx)?;
        }

        //
        // Just to be careful, let's test CYCLAC itself.  Create
        // the expected result of cycling the array backward by I.
        //
        spicelib::CLEARC(CSIZE, C2ARRY.as_arg_mut());

        for J in 1..=CSIZE {
            //
            // Find the target location of the Jth element of CARRAY.
            //
            spicelib::RMAINI((J - I), CSIZE, &mut Q, &mut R, ctx)?;

            if (R == 0) {
                R = CSIZE;
            }

            fstr::assign(C2ARRY.get_mut(R), CARRAY.get(J));
        }

        for J in 1..=CSIZE {
            fstr::assign(&mut LABEL, b"C2ARRY(#)");
            spicelib::REPMI(&LABEL.clone(), b"#", J, &mut LABEL, ctx);
            testutil::CHCKSC(&LABEL, &C2ARRY[J], b"=", &XCARRY[J], OK, ctx)?;
        }

        //
        // Cycle forward by -I as well.
        //
        spicelib::CYCLAC(CARRAY.as_arg(), CSIZE, b"F", -I, XCARRY.as_arg_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        for J in 1..=CSIZE {
            fstr::assign(&mut LABEL, b"C2ARRY(#)");
            spicelib::REPMI(&LABEL.clone(), b"#", J, &mut LABEL, ctx);
            testutil::CHCKSC(&LABEL, &C2ARRY[J], b"=", &XCARRY[J], OK, ctx)?;
        }

        //
        // Cycle in the opposite direction using a negative count.
        //
        spicelib::MOVEC(CARRAY.as_arg(), CSIZE, C2ARRY.as_arg_mut());
        spicelib::CYACIP(CSIZE, b"F", -I, C2ARRY.as_arg_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        for J in 1..=CSIZE {
            fstr::assign(&mut LABEL, b"C2ARRY(#)");
            spicelib::REPMI(&LABEL.clone(), b"#", J, &mut LABEL, ctx);
            testutil::CHCKSC(b"C2ARRY", &C2ARRY[J], b"=", &XCARRY[J], OK, ctx)?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     When the array size is non-positive, the call should be
    //     a no-op.
    //
    //
    //     Use a negative size.
    //
    testutil::TCASE(b"Error case:  non-positive array size", ctx)?;

    spicelib::MOVEC(CARRAY.as_arg(), CSIZE, C2ARRY.as_arg_mut());

    spicelib::CYACIP(-CSIZE, b"F", -5, C2ARRY.as_arg_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=CSIZE {
        fstr::assign(&mut LABEL, b"C2ARRY(#)");
        spicelib::REPMI(&LABEL.clone(), b"#", I, &mut LABEL, ctx);

        testutil::CHCKSC(&LABEL, &C2ARRY[I], b"=", &CARRAY[I], OK, ctx)?;
    }

    //
    // Use size zero.
    //
    spicelib::MOVEC(CARRAY.as_arg(), CSIZE, C2ARRY.as_arg_mut());

    spicelib::CYACIP(0, b"F", -5, C2ARRY.as_arg_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=CSIZE {
        fstr::assign(&mut LABEL, b"C2ARRY(#)");
        spicelib::REPMI(&LABEL.clone(), b"#", I, &mut LABEL, ctx);

        testutil::CHCKSC(&LABEL, &C2ARRY[I], b"=", &CARRAY[I], OK, ctx)?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case:  invalid direction.", ctx)?;

    spicelib::CYACIP(CSIZE, b"Z", -5, C2ARRY.as_arg_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDDIRECTION)", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
