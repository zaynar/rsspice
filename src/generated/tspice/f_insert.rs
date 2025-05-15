//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LBCELL: i32 = -5;
const MSGLEN: i32 = 240;
const ITMWID: i32 = 30;
const LNSIZE: i32 = 80;
const CSIZE: i32 = 300;
const DSIZE: i32 = 200;
const ISIZE: i32 = 100;

//$Procedure F_INSERT ( SPICE set insertion tests )
pub fn F_INSERT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut CSET = ActualCharArray::new(ITMWID, LBCELL..=CSIZE);
    let mut CVALS = ActualCharArray::new(ITMWID, 1..=(CSIZE + 1));
    let mut LNVALS = ActualCharArray::new(LNSIZE, 1..=CSIZE);
    let mut TITLE = [b' '; MSGLEN as usize];
    let mut XITEMC = ActualCharArray::new(ITMWID, 1..=CSIZE);
    let mut DSET = StackArray::<f64, 206>::new(LBCELL..=DSIZE);
    let mut XITEMD = StackArray::<f64, 200>::new(1..=DSIZE);
    let mut CARD: i32 = 0;
    let mut ISET = StackArray::<i32, 106>::new(LBCELL..=ISIZE);
    let mut N: i32 = 0;
    let mut XITEMI = StackArray::<i32, 100>::new(1..=ISIZE);

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
    testutil::TOPEN(b"F_INSERT", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"INSRTI test:  insert distinct elements in order.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::SSIZEI(ISIZE, ISET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CARD = ISIZE;

    for I in 1..=CARD {
        spicelib::INSRTI(I, ISET.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        XITEMI[I] = I;
    }

    N = spicelib::CARDI(ISET.as_slice(), ctx)?;

    testutil::CHCKSI(b"N", N, b"=", CARD, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"ITEMS",
        ISET.subarray(1),
        b"=",
        XITEMI.as_slice(),
        N,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"INSRTI test:  insert non-distinct elements in order.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::SSIZEI(ISIZE, ISET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CARD = ISIZE;

    for I in 1..=CARD {
        spicelib::INSRTI(I, ISET.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::INSRTI(I, ISET.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        XITEMI[I] = I;
    }

    N = spicelib::CARDI(ISET.as_slice(), ctx)?;

    testutil::CHCKSI(b"N", N, b"=", CARD, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"ITEMS",
        ISET.subarray(1),
        b"=",
        XITEMI.as_slice(),
        N,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"INSRTI test:  insert distinct elements in reverse order.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::SSIZEI(ISIZE, ISET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CARD = ISIZE;

    for I in intrinsics::range(CARD, 1, -1) {
        spicelib::INSRTI(I, ISET.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        XITEMI[I] = I;
    }

    N = spicelib::CARDI(ISET.as_slice(), ctx)?;

    testutil::CHCKSI(b"N", N, b"=", CARD, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"ITEMS",
        ISET.subarray(1),
        b"=",
        XITEMI.as_slice(),
        N,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"INSRTI test:  insert non-distinct elements in reverse order.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::SSIZEI(ISIZE, ISET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CARD = ISIZE;

    for I in intrinsics::range(CARD, 1, -1) {
        spicelib::INSRTI(I, ISET.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::INSRTI(I, ISET.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        XITEMI[I] = I;
    }

    N = spicelib::CARDI(ISET.as_slice(), ctx)?;

    testutil::CHCKSI(b"N", N, b"=", CARD, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"ITEMS",
        ISET.subarray(1),
        b"=",
        XITEMI.as_slice(),
        N,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"INSRTI test:  insert non-distinct elements in non-sequential order.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::SSIZEI(ISIZE, ISET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CARD = ISIZE;

    for I in 1..=CARD {
        spicelib::INSRTI(((CARD + 1) - I), ISET.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::INSRTI(I, ISET.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        XITEMI[I] = I;
    }

    N = spicelib::CARDI(ISET.as_slice(), ctx)?;

    testutil::CHCKSI(b"N", N, b"=", CARD, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"ITEMS",
        ISET.subarray(1),
        b"=",
        XITEMI.as_slice(),
        N,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"INSRTI error case:  set overflow.", ctx)?;

    spicelib::SSIZEI(ISIZE, ISET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CARD = ISIZE;

    for I in 1..=CARD {
        spicelib::INSRTI(I, ISET.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::INSRTI((CARD + 1), ISET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(SETEXCESS)", OK, ctx)?;

    //*************************************************************
    //
    //
    //     INSRTD tests follow.
    //
    //
    //*************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"INSRTD test:  insert distinct elements in order.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::SSIZED(DSIZE, DSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CARD = DSIZE;

    for I in 1..=CARD {
        spicelib::INSRTD((I as f64), DSET.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        XITEMD[I] = I as f64;
    }

    N = spicelib::CARDD(DSET.as_slice(), ctx)?;

    testutil::CHCKSI(b"N", N, b"=", CARD, 0, OK, ctx)?;

    testutil::CHCKAD(
        b"ITEMS",
        DSET.subarray(1),
        b"=",
        XITEMD.as_slice(),
        N,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"INSRTD test:  insert non-distinct elements in order.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::SSIZED(DSIZE, DSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CARD = DSIZE;

    for I in 1..=CARD {
        spicelib::INSRTD((I as f64), DSET.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::INSRTD((I as f64), DSET.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        XITEMD[I] = I as f64;
    }

    N = spicelib::CARDD(DSET.as_slice(), ctx)?;

    testutil::CHCKSI(b"N", N, b"=", CARD, 0, OK, ctx)?;

    testutil::CHCKAD(
        b"ITEMS",
        DSET.subarray(1),
        b"=",
        XITEMD.as_slice(),
        N,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"INSRTD test:  insert distinct elements in reverse order.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::SSIZED(DSIZE, DSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CARD = DSIZE;

    for I in intrinsics::range(CARD, 1, -1) {
        spicelib::INSRTD((I as f64), DSET.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        XITEMD[I] = I as f64;
    }

    N = spicelib::CARDD(DSET.as_slice(), ctx)?;

    testutil::CHCKSI(b"N", N, b"=", CARD, 0, OK, ctx)?;

    testutil::CHCKAD(
        b"ITEMS",
        DSET.subarray(1),
        b"=",
        XITEMD.as_slice(),
        N,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"INSRTD test:  insert non-distinct elements in reverse order.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::SSIZED(DSIZE, DSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CARD = DSIZE;

    for I in intrinsics::range(CARD, 1, -1) {
        spicelib::INSRTD((I as f64), DSET.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::INSRTD((I as f64), DSET.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        XITEMD[I] = I as f64;
    }

    N = spicelib::CARDD(DSET.as_slice(), ctx)?;

    testutil::CHCKSI(b"N", N, b"=", CARD, 0, OK, ctx)?;

    testutil::CHCKAD(
        b"ITEMS",
        DSET.subarray(1),
        b"=",
        XITEMD.as_slice(),
        N,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"INSRTD test:  insert non-distinct elements in non-sequential order.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::SSIZED(DSIZE, DSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CARD = DSIZE;

    for I in 1..=CARD {
        spicelib::INSRTD((((CARD + 1) - I) as f64), DSET.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::INSRTD((I as f64), DSET.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        XITEMD[I] = I as f64;
    }

    N = spicelib::CARDD(DSET.as_slice(), ctx)?;

    testutil::CHCKSI(b"N", N, b"=", CARD, 0, OK, ctx)?;

    testutil::CHCKAD(
        b"ITEMS",
        DSET.subarray(1),
        b"=",
        XITEMD.as_slice(),
        N,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"INSRTD error case:  set overflow.", ctx)?;

    spicelib::SSIZED(DSIZE, DSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CARD = DSIZE;

    for I in 1..=CARD {
        spicelib::INSRTD((I as f64), DSET.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::INSRTD(((CARD + 1) as f64), DSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(SETEXCESS)", OK, ctx)?;

    //*************************************************************
    //
    //
    //     INSRTC tests follow.
    //
    //
    //*************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"INSRTC test:  insert distinct elements in order.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Create an array of character set values to be used in place
    // of the numeric values used in the preceding tests.
    //
    for I in 1..=CSIZE {
        //
        // See the description of T_ITHSYM at the bottom of this file.
        //
        T_ITHSYM(I, (CSIZE + 1), &mut CVALS[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        //
        // For each element of CVALS, create a longer string that agrees
        // with the first ITMWID characters of the element.
        //
        fstr::assign(LNVALS.get_mut(I), CVALS.get(I));

        spicelib::SUFFIX(b"YYY", 0, &mut LNVALS[I]);
    }

    //
    // Create one extra element of CVALS, which will be used later.
    //
    T_ITHSYM((CSIZE + 1), (CSIZE + 1), &mut CVALS[(CSIZE + 1)], ctx)?;

    spicelib::SSIZEC(CSIZE, CSET.as_arg_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CARD = CSIZE;

    for I in 1..=CARD {
        spicelib::INSRTC(&CVALS[I], CSET.as_arg_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(XITEMC.get_mut(I), CVALS.get(I));
    }

    N = spicelib::CARDC(CSET.as_arg(), ctx)?;

    testutil::CHCKSI(b"N", N, b"=", CARD, 0, OK, ctx)?;

    testutil::CHCKAC(
        b"ITEMS",
        CSET.subarray(1),
        b"=",
        XITEMC.as_arg(),
        N,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"INSRTC test:  insert non-distinct elements in order.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::SSIZEC(CSIZE, CSET.as_arg_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CARD = CSIZE;

    for I in 1..=CARD {
        spicelib::INSRTC(&CVALS[I], CSET.as_arg_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::INSRTC(&CVALS[I], CSET.as_arg_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(XITEMC.get_mut(I), CVALS.get(I));
    }

    N = spicelib::CARDC(CSET.as_arg(), ctx)?;

    testutil::CHCKSI(b"N", N, b"=", CARD, 0, OK, ctx)?;

    testutil::CHCKAC(
        b"ITEMS",
        CSET.subarray(1),
        b"=",
        XITEMC.as_arg(),
        N,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"INSRTC test:  insert non-distinct elements in order. This time use duplicate elements that disagree with the set elements only at positions past the string length of the set.");

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::SSIZEC(CSIZE, CSET.as_arg_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CARD = CSIZE;

    for I in 1..=CARD {
        spicelib::INSRTC(&CVALS[I], CSET.as_arg_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::INSRTC(&LNVALS[I], CSET.as_arg_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(XITEMC.get_mut(I), CVALS.get(I));
    }

    N = spicelib::CARDC(CSET.as_arg(), ctx)?;

    testutil::CHCKSI(b"N", N, b"=", CARD, 0, OK, ctx)?;

    testutil::CHCKAC(
        b"ITEMS",
        CSET.subarray(1),
        b"=",
        XITEMC.as_arg(),
        N,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"INSRTC test:  insert distinct elements in reverse order.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::SSIZEC(CSIZE, CSET.as_arg_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CARD = CSIZE;

    for I in intrinsics::range(CARD, 1, -1) {
        spicelib::INSRTC(&CVALS[I], CSET.as_arg_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(XITEMC.get_mut(I), CVALS.get(I));
    }

    N = spicelib::CARDC(CSET.as_arg(), ctx)?;

    testutil::CHCKSI(b"N", N, b"=", CARD, 0, OK, ctx)?;

    testutil::CHCKAC(
        b"ITEMS",
        CSET.subarray(1),
        b"=",
        XITEMC.as_arg(),
        N,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"INSRTC test:  insert non-distinct elements in reverse order. This time use duplicate elements that disagree with the set elements only at positions past the string length of the set.");

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::SSIZEC(CSIZE, CSET.as_arg_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CARD = CSIZE;

    for I in intrinsics::range(CARD, 1, -1) {
        spicelib::INSRTC(&CVALS[I], CSET.as_arg_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::INSRTC(&LNVALS[I], CSET.as_arg_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(XITEMC.get_mut(I), CVALS.get(I));
    }

    N = spicelib::CARDC(CSET.as_arg(), ctx)?;

    testutil::CHCKSI(b"N", N, b"=", CARD, 0, OK, ctx)?;

    testutil::CHCKAC(
        b"ITEMS",
        CSET.subarray(1),
        b"=",
        XITEMC.as_arg(),
        N,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"INSRTC test:  insert non-distinct elements in non-sequential order.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::SSIZEC(CSIZE, CSET.as_arg_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CARD = CSIZE;

    for I in 1..=CARD {
        spicelib::INSRTC(&CVALS[((CARD + 1) - I)], CSET.as_arg_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::INSRTC(&CVALS[I], CSET.as_arg_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(XITEMC.get_mut(I), CVALS.get(I));
    }

    N = spicelib::CARDC(CSET.as_arg(), ctx)?;

    testutil::CHCKSI(b"N", N, b"=", CARD, 0, OK, ctx)?;

    testutil::CHCKAC(
        b"ITEMS",
        CSET.subarray(1),
        b"=",
        XITEMC.as_arg(),
        N,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"INSRTC test:  insert non-distinct elements in non-sequential order. This time use duplicate elements that disagree with the set elements only at positions past the string length of the set.");

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::SSIZEC(CSIZE, CSET.as_arg_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CARD = CSIZE;

    for I in 1..=CARD {
        spicelib::INSRTC(&CVALS[((CARD + 1) - I)], CSET.as_arg_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::INSRTC(&LNVALS[I], CSET.as_arg_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(XITEMC.get_mut(I), CVALS.get(I));
    }

    N = spicelib::CARDC(CSET.as_arg(), ctx)?;

    testutil::CHCKSI(b"N", N, b"=", CARD, 0, OK, ctx)?;

    testutil::CHCKAC(
        b"ITEMS",
        CSET.subarray(1),
        b"=",
        XITEMC.as_arg(),
        N,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"INSRTC error case:  set overflow.", ctx)?;

    spicelib::SSIZEC(CSIZE, CSET.as_arg_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CARD = CSIZE;

    for I in 1..=CARD {
        spicelib::INSRTC(&CVALS[I], CSET.as_arg_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::INSRTC(&CVALS[(CARD + 1)], CSET.as_arg_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(SETEXCESS)", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
