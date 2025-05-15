//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MSGLEN: i32 = 240;
const LARGE: i32 = 1000;
const NUMCAS: i32 = 1000;
const SMALL: i32 = 10;
const LNSIZE: i32 = 80;

//$Procedure F_SWAPAC ( Test the SPICELIB routine SWAPAC )
pub fn F_SWAPAC(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut TITLE = [b' '; MSGLEN as usize];
    let mut ARRAY0 = ActualCharArray::new(LNSIZE, 1..=LARGE);
    let mut ARRAY1 = ActualCharArray::new(LNSIZE, 1..=LARGE);
    let mut WORK = ActualCharArray::new(LNSIZE, 1..=LARGE);
    let mut XARRAY = ActualCharArray::new(LNSIZE, 1..=LARGE);
    let mut SIZE: i32 = 0;
    let mut START: i32 = 0;

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
    // Saved values
    //

    //
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_SWAPAC", ctx)?;

    //
    // We'll do an exhaustive set of tests on a small array.
    //
    spicelib::CLEARC(LARGE, ARRAY0.as_arg_mut());

    SIZE = SMALL;

    for I in 1..=SIZE {
        spicelib::INTSTR(I, &mut ARRAY0[I], ctx);
    }

    //
    // LOCN is the start of the "upper" slice.
    //
    for LOCN in 1..=SIZE {
        //
        // N is the size of the upper slice.
        //
        for N in 0..=((SIZE - LOCN) + 1) {
            //
            // LOCM is the start of the "lower" slice.
            //
            START = (LOCN + intrinsics::MAX0(&[1, N]));

            for LOCM in START..=SIZE {
                //
                // M is the size of the lower slice.
                //
                for M in 0..=((SIZE - LOCM) + 1) {
                    //
                    // --- Case: ------------------------------------------------------
                    //
                    fstr::assign(&mut TITLE, b"Case: N = #; LOCN = #; M = #; LOCM = #.");

                    spicelib::REPMI(&TITLE.clone(), b"#", N, &mut TITLE, ctx);
                    spicelib::REPMI(&TITLE.clone(), b"#", LOCN, &mut TITLE, ctx);
                    spicelib::REPMI(&TITLE.clone(), b"#", M, &mut TITLE, ctx);
                    spicelib::REPMI(&TITLE.clone(), b"#", LOCM, &mut TITLE, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::TCASE(&TITLE, ctx)?;

                    //
                    // Make two copies of the initial array.
                    //
                    spicelib::MOVEC(ARRAY0.as_arg(), SIZE, ARRAY1.as_arg_mut());
                    spicelib::MOVEC(ARRAY0.as_arg(), SIZE, XARRAY.as_arg_mut());

                    //
                    // Swap the array slices indicated by LOCN, N, LOCM,
                    // and M.
                    //
                    spicelib::SWAPAC(N, LOCN, M, LOCM, ARRAY1.as_arg_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Produce our expected array.
                    //
                    T_SWAPAC(
                        SIZE,
                        N,
                        LOCN,
                        M,
                        LOCM,
                        XARRAY.as_arg_mut(),
                        WORK.as_arg_mut(),
                    );

                    //
                    // Test our results.
                    //
                    for I in 1..=SIZE {
                        fstr::assign(&mut TITLE, b"swapped array element # (0)");
                        spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);

                        testutil::CHCKSC(&TITLE, &ARRAY1[I], b"=", &XARRAY[I], OK, ctx)?;
                    }

                    //
                    // Now we'll repeat the test with the order of
                    // the arguments swapped.
                    //
                    spicelib::MOVEC(ARRAY0.as_arg(), SIZE, ARRAY1.as_arg_mut());
                    spicelib::MOVEC(ARRAY0.as_arg(), SIZE, XARRAY.as_arg_mut());

                    //
                    // Swap the array slices indicated by LOCN, N, LOCM,
                    // and M.
                    //
                    spicelib::SWAPAC(M, LOCM, N, LOCN, ARRAY1.as_arg_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Produce our expected array.
                    //
                    T_SWAPAC(
                        SIZE,
                        M,
                        LOCM,
                        N,
                        LOCN,
                        XARRAY.as_arg_mut(),
                        WORK.as_arg_mut(),
                    );

                    //
                    // Test our results.
                    //
                    for I in 1..=SIZE {
                        fstr::assign(&mut TITLE, b"swapped array element # (1)");
                        spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);

                        testutil::CHCKSC(&TITLE, &ARRAY1[I], b"=", &XARRAY[I], OK, ctx)?;
                    }
                }
            }
        }
    }

    //
    //     Now for some error handling tests.
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SWAPAC: overlapping array slices.", ctx)?;

    spicelib::SWAPAC(2, 1, 2, 1, ARRAY1.as_arg_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(NOTDISTINCT)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SWAPAC: non-positive value of LOCN, LOCM", ctx)?;

    spicelib::SWAPAC(2, 0, 2, 1, ARRAY1.as_arg_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDINDEX)", OK, ctx)?;

    spicelib::SWAPAC(2, -1, 2, 1, ARRAY1.as_arg_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDINDEX)", OK, ctx)?;

    spicelib::SWAPAC(2, 1, 2, -1, ARRAY1.as_arg_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDINDEX)", OK, ctx)?;

    spicelib::SWAPAC(2, 1, 2, 0, ARRAY1.as_arg_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDINDEX)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SWAPAC: Negative value of N, M", ctx)?;

    spicelib::SWAPAC(-2, 0, 2, 1, ARRAY1.as_arg_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDARGUMENT)", OK, ctx)?;

    spicelib::SWAPAC(2, 0, -2, 1, ARRAY1.as_arg_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDARGUMENT)", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
