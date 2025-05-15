//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXDEG: i32 = 27;
const TOLSCL: f64 = 0.0000000000001;
const MAXREC: i32 = 198;
const SPK2: &[u8] = b"test2.bsp";
const SPK2E: &[u8] = b"test2err.bsp";
const SPK2S: &[u8] = b"test2sub.bsp";
const SPK2B: &[u8] = b"test2big.bsp";
const TIGHT: f64 = 0.00000000001;
const BIGN: i32 = 10101;
const CHBDEG: i32 = 2;
const DSCSIZ: i32 = 5;
const NCHBRC: i32 = 4;
const ND: i32 = 2;
const NEPOCH: i32 = (NCHBRC + 1);
const NI: i32 = 6;
const SIDLEN: i32 = 60;
const FRNMLN: i32 = 32;

struct SaveVars {
    CHBCF2: StackArray3D<f64, 36>,
    DSCEPC: StackArray<f64, 5>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CHBCF2 = StackArray3D::<f64, 36>::new(0..=CHBDEG, 1..=3, 1..=NCHBRC);
        let mut DSCEPC = StackArray::<f64, 5>::new(1..=NEPOCH);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(100.0),
                Val::D(200.0),
                Val::D(300.0),
                Val::D(400.0),
                Val::D(500.0),
            ]
            .into_iter();
            DSCEPC
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(1.0101),
                Val::D(1.0102),
                Val::D(1.0103),
                Val::D(1.0201),
                Val::D(1.0202),
                Val::D(1.0203),
                Val::D(1.0301),
                Val::D(1.0302),
                Val::D(1.0303),
                Val::D(2.0101),
                Val::D(2.0102),
                Val::D(2.0103),
                Val::D(2.0201),
                Val::D(2.0202),
                Val::D(2.0203),
                Val::D(2.0301),
                Val::D(2.0302),
                Val::D(2.0303),
                Val::D(3.0101),
                Val::D(3.0102),
                Val::D(3.0103),
                Val::D(3.0201),
                Val::D(3.0202),
                Val::D(3.0203),
                Val::D(3.0301),
                Val::D(3.0302),
                Val::D(3.0303),
                Val::D(4.0101),
                Val::D(4.0102),
                Val::D(4.0103),
                Val::D(4.0201),
                Val::D(4.0202),
                Val::D(4.0203),
                Val::D(4.0301),
                Val::D(4.0302),
                Val::D(4.0303),
            ]
            .into_iter();
            CHBCF2
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { CHBCF2, DSCEPC }
    }
}

fn T(N: i32, THETA: f64) -> f64 {
    f64::cos(((N as f64) * f64::acos(intrinsics::DMIN1(&[1.0, intrinsics::DMAX1(&[-1.0, THETA])]))))
}

//$Procedure F_SPK02 ( SPK type 2 tests )
pub fn F_SPK02(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut SEGID = [b' '; SIDLEN as usize];
    let mut SEGID2 = [b' '; SIDLEN as usize];
    let mut XREF = [b' '; FRNMLN as usize];
    let mut BEPLST = ActualArray::<f64>::new(1..=(BIGN + 1));
    let mut CHBCFB = ActualArray3D::<f64>::new(0..=CHBDEG, 1..=3, 1..=BIGN);
    let mut DC = StackArray::<f64, 2>::new(1..=ND);
    let mut DESCR = StackArray::<f64, 5>::new(1..=DSCSIZ);
    let mut ET: f64 = 0.0;
    let mut INTLEN: f64 = 0.0;
    let mut LT: f64 = 0.0;
    let mut MIDPT: f64 = 0.0;
    let mut RADIUS: f64 = 0.0;
    let mut RECORD = StackArray::<f64, 198>::new(1..=MAXREC);
    let mut STATE = StackArray::<f64, 6>::new(1..=6);
    let mut THETA: f64 = 0.0;
    let mut XSTATE = StackArray::<f64, 6>::new(1..=6);
    let mut HANDLE: i32 = 0;
    let mut I: i32 = 0;
    let mut IC = StackArray::<i32, 6>::new(1..=NI);
    let mut NEWH: i32 = 0;
    let mut XBODY: i32 = 0;
    let mut XCENTR: i32 = 0;
    let mut FOUND: bool = false;

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
    // Epochs associated with coefficient sets.
    //

    //
    // Chebyshev coefficients for testing SPKW02.
    //

    //
    // Statement functions
    //
    //
    // T(n,theta) represents the Chebyshev polynomial
    //
    //   T ( theta )
    //    n
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_SPK02", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Setup", ctx)?;

    //
    // Create and load a leapseconds kernel.
    //
    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Test SPKW02:  start out with error handling.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad coefficient set count.", ctx)?;

    if spicelib::EXISTS(SPK2E, ctx)? {
        spicelib::DELFIL(SPK2E, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::SPKOPN(SPK2E, b"Type 2 SPK internal file name", 4, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XBODY = 301;
    XCENTR = 3;
    fstr::assign(&mut XREF, b"J2000");
    fstr::assign(&mut SEGID, b"SPK Type 2 test segment");

    INTLEN = (save.DSCEPC[2] - save.DSCEPC[1]);

    spicelib::SPKW02(
        HANDLE,
        XBODY,
        XCENTR,
        &XREF,
        save.DSCEPC[1],
        save.DSCEPC[(NCHBRC + 1)],
        &SEGID,
        INTLEN,
        0,
        CHBDEG,
        save.CHBCF2.as_slice(),
        save.DSCEPC[1],
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NUMCOEFFSNOTPOS)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Polynomial degree too high.", ctx)?;

    //
    // POLYDG = MAXDEG + 1
    //
    spicelib::SPKW02(
        HANDLE,
        XBODY,
        XCENTR,
        &XREF,
        save.DSCEPC[1],
        save.DSCEPC[(NCHBRC + 1)],
        &SEGID,
        INTLEN,
        NCHBRC,
        28,
        save.CHBCF2.as_slice(),
        save.DSCEPC[1],
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDEGREE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Polynomial degree too low.", ctx)?;

    spicelib::SPKW02(
        HANDLE,
        XBODY,
        XCENTR,
        &XREF,
        save.DSCEPC[1],
        save.DSCEPC[(NCHBRC + 1)],
        &SEGID,
        INTLEN,
        NCHBRC,
        -1,
        save.CHBCF2.as_slice(),
        save.DSCEPC[1],
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDEGREE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid interval length.", ctx)?;

    INTLEN = (save.DSCEPC[1] - save.DSCEPC[2]);

    spicelib::SPKW02(
        HANDLE,
        XBODY,
        XCENTR,
        &XREF,
        save.DSCEPC[1],
        save.DSCEPC[(NCHBRC + 1)],
        &SEGID,
        INTLEN,
        NCHBRC,
        CHBDEG,
        save.CHBCF2.as_slice(),
        save.DSCEPC[1],
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INTLENNOTPOS)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid reference frame.", ctx)?;

    INTLEN = (save.DSCEPC[2] - save.DSCEPC[1]);

    spicelib::SPKW02(
        HANDLE,
        XBODY,
        XCENTR,
        b"SPUD",
        save.DSCEPC[1],
        save.DSCEPC[(NCHBRC + 1)],
        &SEGID,
        INTLEN,
        NCHBRC,
        CHBDEG,
        save.CHBCF2.as_slice(),
        save.DSCEPC[1],
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDREFFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Descriptor times out of order.", ctx)?;

    INTLEN = (save.DSCEPC[2] - save.DSCEPC[1]);

    spicelib::SPKW02(
        HANDLE,
        XBODY,
        XCENTR,
        &XREF,
        save.DSCEPC[(NCHBRC + 1)],
        save.DSCEPC[1],
        &SEGID,
        INTLEN,
        NCHBRC,
        CHBDEG,
        save.CHBCF2.as_slice(),
        save.DSCEPC[1],
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADDESCRTIMES)", OK, ctx)?;

    // CALL DAFCLS ( HANDLE )
    // CALL CHCKXC ( .FALSE., ' ', OK )

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Descriptor start time is too early.", ctx)?;

    INTLEN = (save.DSCEPC[2] - save.DSCEPC[1]);

    spicelib::SPKW02(
        HANDLE,
        XBODY,
        XCENTR,
        &XREF,
        (save.DSCEPC[1] - 0.001),
        save.DSCEPC[(NCHBRC + 1)],
        &SEGID,
        INTLEN,
        NCHBRC,
        CHBDEG,
        save.CHBCF2.as_slice(),
        save.DSCEPC[1],
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(COVERAGEGAP)", OK, ctx)?;

    spicelib::SPKW02(
        HANDLE,
        XBODY,
        XCENTR,
        &XREF,
        (save.DSCEPC[1] - 0.0000000000001),
        save.DSCEPC[(NCHBRC + 1)],
        &SEGID,
        INTLEN,
        NCHBRC,
        CHBDEG,
        save.CHBCF2.as_slice(),
        save.DSCEPC[1],
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Descriptor stop time is too late.", ctx)?;

    INTLEN = (save.DSCEPC[2] - save.DSCEPC[1]);

    spicelib::SPKW02(
        HANDLE,
        XBODY,
        XCENTR,
        &XREF,
        save.DSCEPC[1],
        (save.DSCEPC[(NCHBRC + 1)] + 0.001),
        &SEGID,
        INTLEN,
        NCHBRC,
        CHBDEG,
        save.CHBCF2.as_slice(),
        save.DSCEPC[1],
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(COVERAGEGAP)", OK, ctx)?;

    spicelib::SPKW02(
        HANDLE,
        XBODY,
        XCENTR,
        &XREF,
        save.DSCEPC[1],
        (save.DSCEPC[(NCHBRC + 1)] + 0.0000000000001),
        &SEGID,
        INTLEN,
        NCHBRC,
        CHBDEG,
        save.CHBCF2.as_slice(),
        save.DSCEPC[1],
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close this file.  Note that the file contains no segments,
    // so SPKCLS won't close it.
    //
    spicelib::DAFCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test SPKE02: bad coefficient count.", ctx)?;

    spicelib::CLEARD(MAXREC, RECORD.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    RECORD[1] = 0 as f64;

    spicelib::SPKE02(0.0, RECORD.as_slice(), STATE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDCOUNT)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test SPKE02: bad interval radius.", ctx)?;

    spicelib::CLEARD(MAXREC, RECORD.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    RECORD[1] = 5 as f64;
    RECORD[3] = 0.0;

    spicelib::SPKE02(0.0, RECORD.as_slice(), STATE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDRADIUS)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test SPKW02: write small segment.", ctx)?;

    XBODY = 3;
    XCENTR = 10;
    fstr::assign(&mut XREF, b"J2000");

    //
    // Create a segment identifier.
    //
    fstr::assign(&mut SEGID, b"SPK type 2 test segment");

    //
    // Open a new SPK file.
    //
    if spicelib::EXISTS(SPK2, ctx)? {
        spicelib::DELFIL(SPK2, ctx)?;
    }

    spicelib::SPKOPN(SPK2, b"Type 2 SPK internal file name", 4, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    INTLEN = (save.DSCEPC[2] - save.DSCEPC[1]);

    //
    // Create a type 2 segment.
    //
    spicelib::SPKW02(
        HANDLE,
        XBODY,
        XCENTR,
        &XREF,
        save.DSCEPC[1],
        save.DSCEPC[(NCHBRC + 1)],
        &SEGID,
        INTLEN,
        NCHBRC,
        CHBDEG,
        save.CHBCF2.as_slice(),
        save.DSCEPC[1],
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the SPK file.
    //
    spicelib::SPKCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test SPKR02, SPKE02: read small segment.", ctx)?;

    //
    // Load the SPK file.
    //
    spicelib::SPKLEF(SPK2, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up states for each epoch in our list.  Compare.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NCHBRC;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Generate look-up epoch ET.
            //
            RADIUS = (0.5 * INTLEN);
            MIDPT = (save.DSCEPC[I] + RADIUS);

            ET = (MIDPT + (0.5 * RADIUS));

            spicelib::SPKGEO(XBODY, ET, &XREF, XCENTR, STATE.as_slice_mut(), &mut LT, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Evaluate the position "manually."
            //
            THETA = ((ET - MIDPT) / RADIUS);

            for J in 1..=3 {
                XSTATE[J] = 0.0;

                for K in 0..=CHBDEG {
                    XSTATE[J] = (XSTATE[J] + (save.CHBCF2[[K, J, I]] * T(K, THETA)));
                }
            }

            testutil::CHCKAD(
                b"type 2 position",
                STATE.as_slice(),
                b"~",
                XSTATE.as_slice(),
                3,
                TIGHT,
                OK,
                ctx,
            )?;
            I += m3__;
        }
    }
    //
    // Unload the SPK file.
    //
    spicelib::SPKUEF(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test SPKS02: write new file having a segment created by subsetting small segment from SPK2.", ctx)?;

    XBODY = 3;
    XCENTR = 10;
    fstr::assign(&mut XREF, b"J2000");

    //
    // Create a segment identifier.
    //
    fstr::assign(&mut SEGID, b"SPK type 2 test subset segment");

    //
    // Open a new SPK file.
    //
    if spicelib::EXISTS(SPK2S, ctx)? {
        spicelib::DELFIL(SPK2S, ctx)?;
    }

    spicelib::SPKOPN(SPK2S, b"Type 2 SPK internal file name", 0, &mut NEWH, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Open SPK2 and extract segment descriptor and ID of first segment.
    //
    spicelib::DAFOPR(SPK2, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGS(DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGN(&mut SEGID2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a type 2 segment in new file.  Shorten the time
    // coverage by knocking off the coverage contributed by
    // the first and last packets of the source segment.
    //
    spicelib::SPKSUB(
        HANDLE,
        DESCR.as_slice(),
        &SEGID,
        save.DSCEPC[2],
        save.DSCEPC[(NEPOCH - 1)],
        NEWH,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the new SPK file.
    //
    spicelib::SPKCLS(NEWH, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the old SPK file.
    //
    spicelib::SPKCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Test SPKS02: check descriptor bounds on subsetted file.",
        ctx,
    )?;

    //
    // Open SPK2S and extract segment descriptor and ID of first
    // segment.
    //
    spicelib::DAFOPR(SPK2S, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGS(DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFUS(
        DESCR.as_slice(),
        ND,
        NI,
        DC.as_slice_mut(),
        IC.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the time bounds.
    //
    I = (NEPOCH - 1);

    testutil::CHCKSD(b"Segment start", DC[1], b"=", save.DSCEPC[2], 0.0, OK, ctx)?;
    testutil::CHCKSD(b"Segment end", DC[2], b"=", save.DSCEPC[I], 0.0, OK, ctx)?;

    spicelib::SPKCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test SPKS02: read states from subsetted file.", ctx)?;

    //
    // Load the SPK file.
    //
    spicelib::SPKLEF(SPK2S, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XBODY = 3;
    XCENTR = 10;
    fstr::assign(&mut XREF, b"J2000");
    INTLEN = (save.DSCEPC[2] - save.DSCEPC[1]);

    //
    // Look up states for each epoch in our list.  Compare.
    //
    {
        let m1__: i32 = 2;
        let m2__: i32 = (NCHBRC - 1);
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Generate look-up epoch ET.
            //
            RADIUS = (0.5 * INTLEN);
            MIDPT = (save.DSCEPC[I] + RADIUS);

            ET = (MIDPT + (0.5 * RADIUS));

            spicelib::SPKGEO(XBODY, ET, &XREF, XCENTR, STATE.as_slice_mut(), &mut LT, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Evaluate the position "manually."
            //
            THETA = ((ET - MIDPT) / RADIUS);

            for J in 1..=3 {
                XSTATE[J] = 0.0;

                for K in 0..=CHBDEG {
                    XSTATE[J] = (XSTATE[J] + (save.CHBCF2[[K, J, I]] * T(K, THETA)));
                }
            }

            testutil::CHCKAD(
                b"type 2 position",
                STATE.as_slice(),
                b"~",
                XSTATE.as_slice(),
                3,
                TIGHT,
                OK,
                ctx,
            )?;
            I += m3__;
        }
    }
    //
    // Unload the SPK file.
    //
    spicelib::SPKUEF(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SPKR02/SPKE02 test:  create a large segment with multiple directories.",
        ctx,
    )?;

    //
    // Create the coefficient and epoch values we'll use. We're going to
    // follow a pattern similar to that used for the smaller segments
    // created so far: each coefficient will have the value
    //
    //    I + J*10**-5 + K*10**-10
    //
    // where I is the coefficient set index, J is the component (X,Y, or
    // Z) index, and K-1 is the associated degree.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = BIGN;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            for J in 1..=3 {
                for K in 0..=CHBDEG {
                    CHBCFB[[K, J, I]] =
                        (((I as f64) + ((J as f64) * 0.00001)) + ((K as f64) * 0.0000000001));
                }
            }

            I += m3__;
        }
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = (BIGN + 1);
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Initialize the Ith epoch.
            //
            BEPLST[I] = (100 * I) as f64;

            I += m3__;
        }
    }

    XBODY = 3;
    XCENTR = 10;
    fstr::assign(&mut XREF, b"J2000");

    //
    // Create a segment identifier.
    //
    fstr::assign(&mut SEGID, b"SPK type 2 big test segment");

    //
    // Open a new SPK file.
    //
    if spicelib::EXISTS(SPK2B, ctx)? {
        spicelib::DELFIL(SPK2B, ctx)?;
    }

    spicelib::SPKOPN(SPK2B, b"Type 2 SPK internal file name", 0, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    INTLEN = (save.DSCEPC[2] - save.DSCEPC[1]);

    //
    // Create a type 2 segment.
    //
    spicelib::SPKW02(
        HANDLE,
        XBODY,
        XCENTR,
        &XREF,
        BEPLST[1],
        BEPLST[(BIGN + 1)],
        &SEGID,
        INTLEN,
        BIGN,
        CHBDEG,
        CHBCFB.as_slice(),
        BEPLST[1],
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the SPK file.
    //
    spicelib::SPKCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test SPKS02: read states from large file.", ctx)?;

    //
    // Load the SPK file.
    //
    spicelib::SPKLEF(SPK2B, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XBODY = 3;
    XCENTR = 10;
    fstr::assign(&mut XREF, b"J2000");
    INTLEN = (save.DSCEPC[2] - save.DSCEPC[1]);

    //
    // Look up states for each epoch in our list.  Compare.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = BIGN;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Generate look-up epoch ET.
            //
            RADIUS = (0.5 * INTLEN);
            MIDPT = (BEPLST[I] + RADIUS);

            ET = (MIDPT + (0.5 * RADIUS));

            spicelib::SPKGEO(XBODY, ET, &XREF, XCENTR, STATE.as_slice_mut(), &mut LT, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Evaluate the position "manually."
            //
            THETA = ((ET - MIDPT) / RADIUS);

            for J in 1..=3 {
                XSTATE[J] = 0.0;

                for K in 0..=CHBDEG {
                    XSTATE[J] = (XSTATE[J] + (CHBCFB[[K, J, I]] * T(K, THETA)));
                }
            }

            testutil::CHCKAD(
                b"type 2 position",
                STATE.as_slice(),
                b"~",
                XSTATE.as_slice(),
                3,
                TIGHT,
                OK,
                ctx,
            )?;
            I += m3__;
        }
    }
    //
    // Unload the SPK file.
    //
    spicelib::SPKUEF(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Deleting SPK files at clean-up time.", ctx)?;

    //
    // Clean up the SPK files.
    //
    spicelib::DELFIL(SPK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK2E, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK2S, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK2B, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
