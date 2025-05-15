//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LBCELL: i32 = -5;
const LNSIZE: i32 = 320;
const DBSIZE: i32 = 100;

struct SaveVars {
    TITLE: Vec<u8>,
    DARRAY: StackArray<f64, 100>,
    DARRY2: StackArray<f64, 100>,
    DARRY3: StackArray<f64, 100>,
    DSET: StackArray<f64, 106>,
    DVAL: f64,
    I: i32,
    IORDER: StackArray<i32, 100>,
    INVORD: StackArray<i32, 100>,
    J: i32,
    NDIM: i32,
    TO: i32,
    RNGMAX: i32,
    XORDER: StackArray<i32, 100>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut DARRAY = StackArray::<f64, 100>::new(1..=DBSIZE);
        let mut DARRY2 = StackArray::<f64, 100>::new(1..=DBSIZE);
        let mut DARRY3 = StackArray::<f64, 100>::new(1..=DBSIZE);
        let mut DSET = StackArray::<f64, 106>::new(LBCELL..=DBSIZE);
        let mut DVAL: f64 = 0.0;
        let mut I: i32 = 0;
        let mut IORDER = StackArray::<i32, 100>::new(1..=DBSIZE);
        let mut INVORD = StackArray::<i32, 100>::new(1..=DBSIZE);
        let mut J: i32 = 0;
        let mut NDIM: i32 = 0;
        let mut TO: i32 = 0;
        let mut RNGMAX: i32 = 0;
        let mut XORDER = StackArray::<i32, 100>::new(1..=DBSIZE);

        Self {
            TITLE,
            DARRAY,
            DARRY2,
            DARRY3,
            DSET,
            DVAL,
            I,
            IORDER,
            INVORD,
            J,
            NDIM,
            TO,
            RNGMAX,
            XORDER,
        }
    }
}

//$Procedure F_IOVCMP ( IOVCMP tests )
pub fn F_IOVCMP(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

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
    // Save variables in order to avoid stack room problems.
    //

    //
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_IOVCMP", ctx)?;

    //**********************************************************************
    //
    //     IOVCMP normal cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Trivial case: ordered input set.");

    testutil::TCASE(&save.TITLE, ctx)?;

    save.NDIM = 100;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NDIM;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.DARRAY[save.I] = (save.I as f64);
            save.I += m3__;
        }
    }

    support::IOVCMP(
        save.DARRAY.as_slice(),
        save.NDIM,
        save.IORDER.as_slice_mut(),
        save.INVORD.as_slice_mut(),
        &mut save.RNGMAX,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The order vector and its inverse should both be
    // the identity permutation.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NDIM;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.XORDER[save.I] = save.I;
            save.I += m3__;
        }
    }

    testutil::CHCKAI(
        b"IORDER",
        save.IORDER.as_slice(),
        b"=",
        save.XORDER.as_slice(),
        save.NDIM,
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"INVORD",
        save.INVORD.as_slice(),
        b"=",
        save.XORDER.as_slice(),
        save.NDIM,
        OK,
        ctx,
    )?;

    //
    // The maximum element of the inverse order vector is
    // supposed to be NDIM.
    //
    testutil::CHCKSI(b"RNGMAX", save.RNGMAX, b"=", save.NDIM, 0, OK, ctx)?;
    save.NDIM = 100;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Reverse-ordered input set.");

    testutil::TCASE(&save.TITLE, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NDIM;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.DARRAY[save.I] = -(save.I as f64);
            save.I += m3__;
        }
    }

    support::IOVCMP(
        save.DARRAY.as_slice(),
        save.NDIM,
        save.IORDER.as_slice_mut(),
        save.INVORD.as_slice_mut(),
        &mut save.RNGMAX,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The order vector and its inverse should both be
    // the reverse permutation.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NDIM;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.XORDER[save.I] = ((save.NDIM + 1) - save.I);
            save.I += m3__;
        }
    }

    testutil::CHCKAI(
        b"IORDER",
        save.IORDER.as_slice(),
        b"=",
        save.XORDER.as_slice(),
        save.NDIM,
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"INVORD",
        save.INVORD.as_slice(),
        b"=",
        save.XORDER.as_slice(),
        save.NDIM,
        OK,
        ctx,
    )?;

    //
    // The maximum element of the inverse order vector is
    // supposed to be NDIM.
    //
    testutil::CHCKSI(b"RNGMAX", save.RNGMAX, b"=", save.NDIM, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"Ordered input set with paired, duplicate elements.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    save.NDIM = 100;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NDIM;
        let m3__: i32 = 2;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.DARRAY[save.I] = (save.I as f64);
            save.DARRAY[(save.I + 1)] = save.DARRAY[save.I];
            save.I += m3__;
        }
    }

    support::IOVCMP(
        save.DARRAY.as_slice(),
        save.NDIM,
        save.IORDER.as_slice_mut(),
        save.INVORD.as_slice_mut(),
        &mut save.RNGMAX,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Because the shell sort isn't stable, we don't know what
    // the expected order vector is. But we do know that we
    // should be able to re-order the d.p. array using the
    // order vector and obtain an ordered set.
    //
    spicelib::MOVED(
        save.DARRAY.as_slice(),
        save.NDIM,
        save.DARRY2.as_slice_mut(),
    );
    spicelib::REORDD(
        save.IORDER.as_slice_mut(),
        save.NDIM,
        save.DARRY2.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create the expected set.
    //
    spicelib::MOVED(
        save.DARRAY.as_slice(),
        save.NDIM,
        save.DARRY3.as_slice_mut(),
    );
    spicelib::SHELLD(save.NDIM, save.DARRY3.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // This test actually checks IORDER.
    //
    testutil::CHCKAD(
        b"DARRY2/IORDER",
        save.DARRY2.as_slice(),
        b"=",
        save.DARRY3.as_slice(),
        save.NDIM,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Now check the compressed inverse order vector.
    //
    // Make a set out of the original array.
    //
    spicelib::MOVED(save.DARRAY.as_slice(), save.NDIM, save.DSET.subarray_mut(1));

    spicelib::VALIDD(save.NDIM, save.NDIM, save.DSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The compressed inverse order vector should map each index of
    // DARRAY to the index of an element in DSET containing the
    // same value as that located at the original index.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NDIM;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.DVAL = save.DSET[save.INVORD[save.I]];

            testutil::CHCKSD(b"DVAL", save.DVAL, b"=", save.DARRAY[save.I], 0.0, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // The maximum element of the inverse order vector is
    // supposed to be the cardinality of DSET.
    //
    testutil::CHCKSI(
        b"RNGMAX",
        save.RNGMAX,
        b"=",
        spicelib::CARDD(save.DSET.as_slice(), ctx)?,
        0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"Ordered input set with varying numbers of duplicate elements.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    save.NDIM = 30;

    save.I = 1;
    save.TO = 1;

    while (save.TO <= save.NDIM) {
        //
        // Let the number of duplicates range from 1 : 7.
        //
        save.J = intrinsics::MIN0(&[
            ((save.NDIM + 1) - save.I),
            (intrinsics::MOD((save.I - 1), 7) + 1),
        ]);

        //
        // Fill in array elements TO : TO+J-1.
        //
        for K in 1..=save.J {
            save.DARRAY[((save.TO + K) - 1)] = (save.I as f64);
        }

        save.TO = (save.TO + save.J);
        save.I = (save.I + 1);
    }

    support::IOVCMP(
        save.DARRAY.as_slice(),
        save.NDIM,
        save.IORDER.as_slice_mut(),
        save.INVORD.as_slice_mut(),
        &mut save.RNGMAX,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Because the shell sort isn't stable, we don't know what
    // the expected order vector is. But we do know that we
    // should be able to re-order the d.p. array using the
    // order vector and obtain an ordered set.
    //
    spicelib::MOVED(
        save.DARRAY.as_slice(),
        save.NDIM,
        save.DARRY2.as_slice_mut(),
    );
    spicelib::REORDD(
        save.IORDER.as_slice_mut(),
        save.NDIM,
        save.DARRY2.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create the expected set.
    //
    spicelib::MOVED(
        save.DARRAY.as_slice(),
        save.NDIM,
        save.DARRY3.as_slice_mut(),
    );
    spicelib::SHELLD(save.NDIM, save.DARRY3.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // This test actually checks IORDER.
    //
    testutil::CHCKAD(
        b"DARRY2/IORDER",
        save.DARRY2.as_slice(),
        b"=",
        save.DARRY3.as_slice(),
        save.NDIM,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Now check the compressed inverse order vector.
    //
    // Make a set out of the original array.
    //
    spicelib::MOVED(save.DARRAY.as_slice(), save.NDIM, save.DSET.subarray_mut(1));

    spicelib::VALIDD(save.NDIM, save.NDIM, save.DSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The compressed inverse order vector should map each index of
    // DARRAY to the index of an element in DSET containing the
    // same value as that located at the original index.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NDIM;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.DVAL = save.DSET[save.INVORD[save.I]];

            testutil::CHCKSD(b"DVAL", save.DVAL, b"=", save.DARRAY[save.I], 0.0, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // The maximum element of the inverse order vector is
    // supposed to be the cardinality of DSET.
    //
    testutil::CHCKSI(
        b"RNGMAX",
        save.RNGMAX,
        b"=",
        spicelib::CARDD(save.DSET.as_slice(), ctx)?,
        0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Ordered input set with varying numbers of duplicate elements. Negate the input set values from the previous case.");

    testutil::TCASE(&save.TITLE, ctx)?;

    save.NDIM = 30;

    save.I = 1;
    save.TO = 1;

    while (save.TO <= save.NDIM) {
        //
        // Let the number of duplicates range from 1 : 7.
        //
        save.J = intrinsics::MIN0(&[
            ((save.NDIM + 1) - save.I),
            (intrinsics::MOD((save.I - 1), 7) + 1),
        ]);

        //
        // Fill in array elements TO : TO+J-1.
        //
        for K in 1..=save.J {
            save.DARRAY[((save.TO + K) - 1)] = -(save.I as f64);
        }

        save.TO = (save.TO + save.J);
        save.I = (save.I + 1);
    }

    support::IOVCMP(
        save.DARRAY.as_slice(),
        save.NDIM,
        save.IORDER.as_slice_mut(),
        save.INVORD.as_slice_mut(),
        &mut save.RNGMAX,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Because the shell sort isn't stable, we don't know what
    // the expected order vector is. But we do know that we
    // should be able to re-order the d.p. array using the
    // order vector and obtain an ordered set.
    //
    spicelib::MOVED(
        save.DARRAY.as_slice(),
        save.NDIM,
        save.DARRY2.as_slice_mut(),
    );
    spicelib::REORDD(
        save.IORDER.as_slice_mut(),
        save.NDIM,
        save.DARRY2.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create the expected set.
    //
    spicelib::MOVED(
        save.DARRAY.as_slice(),
        save.NDIM,
        save.DARRY3.as_slice_mut(),
    );
    spicelib::SHELLD(save.NDIM, save.DARRY3.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // This test actually checks IORDER.
    //
    testutil::CHCKAD(
        b"DARRY2/IORDER",
        save.DARRY2.as_slice(),
        b"=",
        save.DARRY3.as_slice(),
        save.NDIM,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Now check the compressed inverse order vector.
    //
    // Make a set out of the original array.
    //
    spicelib::MOVED(save.DARRAY.as_slice(), save.NDIM, save.DSET.subarray_mut(1));

    spicelib::VALIDD(save.NDIM, save.NDIM, save.DSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The compressed inverse order vector should map each index of
    // DARRAY to the index of an element in DSET containing the
    // same value as that located at the original index.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NDIM;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.DVAL = save.DSET[save.INVORD[save.I]];

            testutil::CHCKSD(b"DVAL", save.DVAL, b"=", save.DARRAY[save.I], 0.0, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // The maximum element of the inverse order vector is
    // supposed to be the cardinality of DSET.
    //
    testutil::CHCKSI(
        b"RNGMAX",
        save.RNGMAX,
        b"=",
        spicelib::CARDD(save.DSET.as_slice(), ctx)?,
        0,
        OK,
        ctx,
    )?;

    //**********************************************************************
    //
    //     IOVCMP error cases
    //
    //**********************************************************************

    //
    // None so far.
    //

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
