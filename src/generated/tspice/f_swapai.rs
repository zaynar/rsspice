//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MSGLEN: i32 = 240;
const LARGE: i32 = 1000;
const NUMCAS: i32 = 1000;
const SMALL: i32 = 10;

//$Procedure F_SWAPAI ( Test the SPICELIB routine SWAPAI )
pub fn F_SWAPAI(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut TITLE = [b' '; MSGLEN as usize];
    let mut ARRAY0 = ActualArray::<i32>::new(1..=LARGE);
    let mut ARRAY1 = ActualArray::<i32>::new(1..=LARGE);
    let mut SIZE: i32 = 0;
    let mut START: i32 = 0;
    let mut WORK = ActualArray::<i32>::new(1..=LARGE);
    let mut XARRAY = ActualArray::<i32>::new(1..=LARGE);

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
    testutil::TOPEN(b"F_SWAPAI", ctx)?;

    //
    // We'll do an exhaustive set of tests on a small array.
    //
    spicelib::CLEARI(LARGE, ARRAY0.as_slice_mut());

    SIZE = SMALL;

    for I in 1..=SIZE {
        ARRAY0[I] = I;
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
                    spicelib::MOVEI(ARRAY0.as_slice(), SIZE, ARRAY1.as_slice_mut());
                    spicelib::MOVEI(ARRAY0.as_slice(), SIZE, XARRAY.as_slice_mut());

                    //
                    // Swap the array slices indicated by LOCN, N, LOCM,
                    // and M.
                    //
                    spicelib::SWAPAI(N, LOCN, M, LOCM, ARRAY1.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Produce our expected array.
                    //
                    T_SWAPAI(
                        SIZE,
                        N,
                        LOCN,
                        M,
                        LOCM,
                        XARRAY.as_slice_mut(),
                        WORK.as_slice_mut(),
                    );

                    //
                    // Test our results.
                    //
                    testutil::CHCKAI(
                        b"swapped array (0)",
                        ARRAY1.as_slice(),
                        b"=",
                        XARRAY.as_slice(),
                        SIZE,
                        OK,
                        ctx,
                    )?;

                    //
                    // Now we'll repeat the test with the order of
                    // the arguments swapped.
                    //
                    spicelib::MOVEI(ARRAY0.as_slice(), SIZE, ARRAY1.as_slice_mut());
                    spicelib::MOVEI(ARRAY0.as_slice(), SIZE, XARRAY.as_slice_mut());

                    //
                    // Swap the array slices indicated by LOCN, N, LOCM,
                    // and M.
                    //
                    spicelib::SWAPAI(M, LOCM, N, LOCN, ARRAY1.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Produce our expected array.
                    //
                    T_SWAPAI(
                        SIZE,
                        M,
                        LOCM,
                        N,
                        LOCN,
                        XARRAY.as_slice_mut(),
                        WORK.as_slice_mut(),
                    );

                    //
                    // Test our results.
                    //
                    testutil::CHCKAI(
                        b"swapped array (1)",
                        ARRAY1.as_slice(),
                        b"=",
                        XARRAY.as_slice(),
                        SIZE,
                        OK,
                        ctx,
                    )?;
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
    testutil::TCASE(b"SWAPAI: overlapping array slices.", ctx)?;

    spicelib::SWAPAI(2, 1, 2, 1, ARRAY1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(NOTDISTINCT)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SWAPAI: non-positive value of LOCN, LOCM", ctx)?;

    spicelib::SWAPAI(2, 0, 2, 1, ARRAY1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDINDEX)", OK, ctx)?;

    spicelib::SWAPAI(2, -1, 2, 1, ARRAY1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDINDEX)", OK, ctx)?;

    spicelib::SWAPAI(2, 1, 2, -1, ARRAY1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDINDEX)", OK, ctx)?;

    spicelib::SWAPAI(2, 1, 2, 0, ARRAY1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDINDEX)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SWAPAI: Negative value of N, M", ctx)?;

    spicelib::SWAPAI(-2, 0, 2, 1, ARRAY1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDARGUMENT)", OK, ctx)?;

    spicelib::SWAPAI(2, 0, -2, 1, ARRAY1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDARGUMENT)", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
