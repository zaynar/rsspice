//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CK: &[u8] = b"pckcov.bc";
const EK: &[u8] = b"pckcov.bes";
const PCK: &[u8] = b"pckcov.bpc";
const SCLK: &[u8] = b"pckcov.tsc";
const SPK: &[u8] = b"pckcov.bsp";
const XFRPCK: &[u8] = b"pckcov.xsp";
const DELTA: f64 = 0.000001;
const LBCELL: i32 = -5;
const FILSIZ: i32 = 255;
const LNSIZE: i32 = 80;
const NFRM: i32 = 3;
const MAXCOV: i32 = 1000;

struct SaveVars {
    FRAME: StackArray<i32, 3>,
    NSEG: StackArray<i32, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FRAME = StackArray::<i32, 3>::new(1..=NFRM);
        let mut NSEG = StackArray::<i32, 3>::new(1..=NFRM);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(4), Val::I(5), Val::I(6)].into_iter();
            FRAME
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(10), Val::I(20), Val::I(30)].into_iter();
            NSEG.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { FRAME, NSEG }
    }
}

//$Procedure      F_PCKCOV ( PCKCOV tests )
pub fn F_PCKCOV(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TITLE = [b' '; LNSIZE as usize];
    let mut COVER = ActualArray::<f64>::new(LBCELL..=MAXCOV);
    let mut FIRST: f64 = 0.0;
    let mut INTLEN: f64 = 0.0;
    let mut LAST: f64 = 0.0;
    let mut COEFFS = StackArray2D::<f64, 6>::new(1..=2, 1..=3);
    let mut XCOVER = ActualArray2D::<f64>::new(LBCELL..=MAXCOV, 1..=NFRM);
    let mut HANDLE: i32 = 0;
    let mut IDS = StackArray::<i32, 10>::new(LBCELL..=(NFRM + 1));
    let mut XIDS = StackArray::<i32, 10>::new(LBCELL..=(NFRM + 1));
    let mut XUNIT: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_PCKCOV", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Setup:  create PCK file.", ctx)?;

    //
    // Create an PCK file with data for three frames.
    //
    spicelib::PCKOPN(PCK, PCK, 0, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Initialize the expected coverage windows.
    //
    for I in 1..=NFRM {
        spicelib::SSIZED(MAXCOV, XCOVER.subarray_mut([LBCELL, I]), ctx)?;
    }

    spicelib::CLEARD(6, COEFFS.as_slice_mut());

    for I in 1..=NFRM {
        for J in 1..=save.NSEG[I] {
            //
            // Create segments for frame I.
            //
            if (I == 1) {
                //
                // Create NSEG(1) segments, each one separated
                // by a 1 second gap.
                //
                FIRST = (((J - 1) as f64) * 11.0);
                LAST = (FIRST + 10.0);
            } else if (I == 2) {
                //
                // Create NSEG(2) segments, each one separated
                // by a 1 second gap.  This time, create the
                // segments in decreasing time order.
                //
                FIRST = (((save.NSEG[2] - J) as f64) * 101.0);
                LAST = (FIRST + 100.0);
            } else {
                //
                // I equals 3.
                //
                // Create NSEG(3) segments with no gaps.
                //
                FIRST = (((J - 1) as f64) * 1000.0);
                LAST = (FIRST + 1000.0);
            }

            //
            // Add to the expected coverage window for this frame.
            //
            spicelib::WNINSD(FIRST, LAST, XCOVER.subarray_mut([LBCELL, I]), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // There's no need for the data to be complicated. Create
            // just one record per segment.
            //
            COEFFS[[1, 1]] = 11.0;
            COEFFS[[2, 1]] = 21.0;

            COEFFS[[1, 2]] = 12.0;
            COEFFS[[2, 2]] = 22.0;

            COEFFS[[1, 3]] = 13.0;
            COEFFS[[2, 3]] = 23.0;

            INTLEN = (LAST - FIRST);

            spicelib::PCKW02(
                HANDLE,
                save.FRAME[I],
                b"J2000",
                FIRST,
                LAST,
                b"TEST",
                INTLEN,
                1,
                1,
                COEFFS.as_slice(),
                FIRST,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }

    spicelib::PCKCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Loop through the canned cases.
    //
    for I in 1..=NFRM {
        //
        // --- Case: ------------------------------------------------------
        //
        fstr::assign(&mut TITLE, b"Check coverage for frame #.");
        spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);

        testutil::TCASE(&TITLE, ctx)?;

        spicelib::SSIZED(MAXCOV, COVER.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::PCKCOV(PCK, save.FRAME[I], COVER.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check cardinality of coverage window.
        //
        testutil::CHCKSI(
            b"CARDD(COVER)",
            spicelib::CARDD(COVER.as_slice(), ctx)?,
            b"=",
            spicelib::CARDD(XCOVER.subarray([LBCELL, I]), ctx)?,
            0,
            OK,
            ctx,
        )?;

        //
        // Check coverage window.
        //
        testutil::CHCKAD(
            b"COVER",
            COVER.subarray(1),
            b"=",
            XCOVER.subarray([1, I]),
            spicelib::CARDD(COVER.as_slice(), ctx)?,
            0.0,
            OK,
            ctx,
        )?;
    }

    //
    // Loop through the canned cases.  This time, use a coverage
    // window that already contains data.
    //
    spicelib::SCARDD(0, COVER.as_slice_mut(), ctx)?;

    for I in 1..=NFRM {
        //
        // --- Case: ------------------------------------------------------
        //
        fstr::assign(
            &mut TITLE,
            b"Check coverage for frame #; COVER starts out non-empty.",
        );
        spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);

        testutil::TCASE(&TITLE, ctx)?;

        spicelib::SSIZED(MAXCOV, COVER.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::WNINSD(1000000.0, 10000000.0, COVER.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::PCKCOV(PCK, save.FRAME[I], COVER.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check cardinality of coverage window.
        //
        spicelib::WNINSD(1000000.0, 10000000.0, XCOVER.subarray_mut([LBCELL, I]), ctx)?;

        testutil::CHCKSI(
            b"CARDD(COVER)",
            spicelib::CARDD(COVER.as_slice(), ctx)?,
            b"=",
            spicelib::CARDD(XCOVER.subarray([LBCELL, I]), ctx)?,
            0,
            OK,
            ctx,
        )?;

        //
        // Check coverage window.
        //
        testutil::CHCKAD(
            b"COVER",
            COVER.subarray(1),
            b"=",
            XCOVER.subarray([1, I]),
            spicelib::CARDD(COVER.as_slice(), ctx)?,
            0.0,
            OK,
            ctx,
        )?;
    }

    //
    // Error cases:
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Try to find coverage for a transfer PCK.", ctx)?;

    spicelib::TXTOPN(XFRPCK, &mut XUNIT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBT(PCK, XUNIT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(XUNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    spicelib::PCKCOV(XFRPCK, save.FRAME[1], COVER.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDFORMAT)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Try to find coverage for a CK.", ctx)?;

    testutil::TSTCK3(CK, SCLK, false, false, false, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PCKCOV(CK, save.FRAME[1], COVER.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDFILETYPE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Try to find coverage for an SPK.", ctx)?;

    testutil::TSTSPK(SPK, false, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PCKCOV(SPK, save.FRAME[1], COVER.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDFILETYPE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Try to find coverage for an EK.", ctx)?;

    testutil::TSTEK(EK, 1, 20, false, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PCKCOV(EK, save.FRAME[1], COVER.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDARCHTYPE)", OK, ctx)?;

    // ******************************************************
    // ******************************************************
    // ******************************************************
    //     PCKFRM tests
    // ******************************************************
    // ******************************************************
    // ******************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Find frames in our test PCK.", ctx)?;

    spicelib::SSIZEI(NFRM, IDS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZEI(NFRM, XIDS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=NFRM {
        spicelib::INSRTI(save.FRAME[I], XIDS.as_slice_mut(), ctx)?;
    }

    spicelib::PCKFRM(PCK, IDS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check cardinality of frame set.
    //
    testutil::CHCKSI(
        b"CARDI(IDS)",
        spicelib::CARDI(IDS.as_slice(), ctx)?,
        b"=",
        spicelib::CARDI(XIDS.as_slice(), ctx)?,
        0,
        OK,
        ctx,
    )?;

    //
    // Check frame set.
    //
    testutil::CHCKAI(
        b"IDS",
        IDS.subarray(1),
        b"=",
        XIDS.subarray(1),
        spicelib::CARDI(XIDS.as_slice(), ctx)?,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Find frames in our test PCK.  Start with non-empty ID set.",
        ctx,
    )?;

    spicelib::SSIZEI((NFRM + 1), IDS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZEI((NFRM + 1), XIDS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::INSRTI(-1000000, XIDS.as_slice_mut(), ctx)?;

    for I in 1..=NFRM {
        spicelib::INSRTI(save.FRAME[I], XIDS.as_slice_mut(), ctx)?;
    }

    spicelib::INSRTI(-1000000, IDS.as_slice_mut(), ctx)?;

    spicelib::PCKFRM(PCK, IDS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check cardinality of frame set.
    //
    testutil::CHCKSI(
        b"CARDI(IDS)",
        spicelib::CARDI(IDS.as_slice(), ctx)?,
        b"=",
        spicelib::CARDI(XIDS.as_slice(), ctx)?,
        0,
        OK,
        ctx,
    )?;

    //
    // Check frame set.
    //
    testutil::CHCKAI(
        b"IDS",
        IDS.subarray(1),
        b"=",
        XIDS.subarray(1),
        spicelib::CARDI(XIDS.as_slice(), ctx)?,
        OK,
        ctx,
    )?;
    //
    // Error cases:
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Try to find frames for a transfer PCK.", ctx)?;

    //
    // Initialize the IDS set.
    //
    spicelib::SSIZEI(NFRM, IDS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PCKFRM(XFRPCK, IDS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDFORMAT)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Try to find frames for a CK.", ctx)?;

    spicelib::PCKFRM(CK, IDS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDFILETYPE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Try to find frames for an EK.", ctx)?;

    spicelib::PCKFRM(EK, IDS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDARCHTYPE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Try to find frames for an SPK.", ctx)?;

    spicelib::PCKFRM(SPK, IDS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDFILETYPE)", OK, ctx)?;

    //
    // Clean up.
    //
    spicelib::DELFIL(PCK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(XFRPCK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(CK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(EK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
