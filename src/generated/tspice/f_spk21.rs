//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXTRM: i32 = 25;
const MAXREC: i32 = 198;
const SPK0: &[u8] = b"test.bsp";
const SPK1: &[u8] = b"spk_test_21_v1.bsp";
const SPK2: &[u8] = b"spk_test_21_v2.bsp";
const SPK21S: &[u8] = b"test21sub.bsp";
const DEFSID: &[u8] = b"spkw21 test segment";
const TIGHT: f64 = 0.000000000005;
const VTIGHT: f64 = 0.0000000000001;
const DLMAX: i32 = ((4 * MAXTRM) + 11);
const MXNREC: i32 = 10099;
const MAXSMP: i32 = 100001;
const FRNMLN: i32 = 32;
const LNSIZE: i32 = 80;
const ND: i32 = 2;
const NI: i32 = 6;
const DSCSIZ: i32 = 5;

struct SaveVars {
    SEGID: Vec<u8>,
    FRAME: Vec<u8>,
    LABEL: Vec<u8>,
    DC: StackArray<f64, 2>,
    DELTA: f64,
    DESCR: StackArray<f64, 5>,
    DLBUFF: ActualArray2D<f64>,
    DPBUFF: ActualArray<f64>,
    TBUFF: ActualArray<f64>,
    FIRST: f64,
    LAST: f64,
    LT: f64,
    ET: f64,
    STABUF: ActualArray2D<f64>,
    STATE: StackArray<f64, 6>,
    STEP: f64,
    T13REC: StackArray<f64, 198>,
    T21REC: ActualArray<f64>,
    XSTATE: StackArray<f64, 6>,
    XSTBUF: ActualArray2D<f64>,
    BODY: i32,
    CENTER: i32,
    DLSIZE: i32,
    FRCODE: i32,
    HAN0: i32,
    HAN2: i32,
    HANDLE: i32,
    HANS: i32,
    I: i32,
    IC: StackArray<i32, 6>,
    J: i32,
    K: i32,
    MAXDIM: i32,
    MIOIDX: i32,
    N: i32,
    NSAMP: i32,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SEGID = vec![b' '; LNSIZE as usize];
        let mut FRAME = vec![b' '; FRNMLN as usize];
        let mut LABEL = vec![b' '; LNSIZE as usize];
        let mut DC = StackArray::<f64, 2>::new(1..=ND);
        let mut DELTA: f64 = 0.0;
        let mut DESCR = StackArray::<f64, 5>::new(1..=DSCSIZ);
        let mut DLBUFF = ActualArray2D::<f64>::new(1..=DLMAX, 1..=MXNREC);
        let mut DPBUFF = ActualArray::<f64>::new(1..=((DLMAX * MXNREC) + 1));
        let mut TBUFF = ActualArray::<f64>::new(1..=MXNREC);
        let mut FIRST: f64 = 0.0;
        let mut LAST: f64 = 0.0;
        let mut LT: f64 = 0.0;
        let mut ET: f64 = 0.0;
        let mut STABUF = ActualArray2D::<f64>::new(1..=6, 1..=MAXSMP);
        let mut STATE = StackArray::<f64, 6>::new(1..=6);
        let mut STEP: f64 = 0.0;
        let mut T13REC = StackArray::<f64, 198>::new(1..=MAXREC);
        let mut T21REC = ActualArray::<f64>::new(1..=300);
        let mut XSTATE = StackArray::<f64, 6>::new(1..=6);
        let mut XSTBUF = ActualArray2D::<f64>::new(1..=6, 1..=MAXSMP);
        let mut BODY: i32 = 0;
        let mut CENTER: i32 = 0;
        let mut DLSIZE: i32 = 0;
        let mut FRCODE: i32 = 0;
        let mut HAN0: i32 = 0;
        let mut HAN2: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut HANS: i32 = 0;
        let mut I: i32 = 0;
        let mut IC = StackArray::<i32, 6>::new(1..=NI);
        let mut J: i32 = 0;
        let mut K: i32 = 0;
        let mut MAXDIM: i32 = 0;
        let mut MIOIDX: i32 = 0;
        let mut N: i32 = 0;
        let mut NSAMP: i32 = 0;
        let mut FOUND: bool = false;

        Self {
            SEGID,
            FRAME,
            LABEL,
            DC,
            DELTA,
            DESCR,
            DLBUFF,
            DPBUFF,
            TBUFF,
            FIRST,
            LAST,
            LT,
            ET,
            STABUF,
            STATE,
            STEP,
            T13REC,
            T21REC,
            XSTATE,
            XSTBUF,
            BODY,
            CENTER,
            DLSIZE,
            FRCODE,
            HAN0,
            HAN2,
            HANDLE,
            HANS,
            I,
            IC,
            J,
            K,
            MAXDIM,
            MIOIDX,
            N,
            NSAMP,
            FOUND,
        }
    }
}

//$Procedure F_SPK21 ( SPK data type 21 tests )
pub fn F_SPK21(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Test Utility Functions
    //

    //
    // SPICELIB Functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //
    //
    // Note: SEGID is declared longer than SIDLEN because of the need to
    // hold a long string for testing error handling.
    //

    //
    // Saved variables
    //
    //
    // Save all local variables to avoid stack problems on some
    // platforms.
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_SPK21", ctx)?;

    //
    //     Open a new SPK file for writing.
    //
    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Setup:  open a new SPK file for writing.", ctx)?;

    if spicelib::EXISTS(SPK1, ctx)? {
        spicelib::DELFIL(SPK1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::SPKOPN(SPK1, b" ", 0, &mut save.HANDLE, ctx)?;

    //
    // Initialize the time and data buffers with values that are
    // recognizable but otherwise bogus.
    //
    save.MAXDIM = MAXTRM;
    save.MIOIDX = ((4 * save.MAXDIM) + 8);

    {
        let m1__: i32 = 1;
        let m2__: i32 = MXNREC;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.TBUFF[save.I] = ((save.I as f64) * 1000.0);

            {
                let m1__: i32 = 1;
                let m2__: i32 = DLMAX;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.DLBUFF[[save.J, save.I]] =
                        ((save.TBUFF[save.I] + save.J as f64) - 1 as f64);

                    save.J += m3__;
                }
            }

            save.DLBUFF[[save.MIOIDX, save.I]] = save.MAXDIM as f64;

            save.I += m3__;
        }
    }

    //
    // Pick body, center, and frame.
    //
    save.BODY = 3;
    save.CENTER = 10;
    fstr::assign(&mut save.FRAME, b"J2000");

    //
    // Initial difference line count.
    //
    save.N = 100;

    //
    // Pick nominal time bounds.
    //
    save.FIRST = 0.0;
    save.LAST = save.TBUFF[save.N];

    //
    // Initialize segment identifier.
    //
    fstr::assign(&mut save.SEGID, DEFSID);

    //
    //*****************************************************************
    //*
    //*    SPKW21 error cases:
    //*
    //*****************************************************************
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid frame.", ctx)?;

    spicelib::SPKW21(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        b"XXX",
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.N,
        DLMAX,
        save.DLBUFF.as_slice(),
        save.TBUFF.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDREFFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SEGID too long.", ctx)?;

    fstr::assign(
        &mut save.SEGID,
        b"1234567890123456789012345678912345678901234567890",
    );

    spicelib::SPKW21(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.N,
        DLMAX,
        save.DLBUFF.as_slice(),
        save.TBUFF.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SEGIDTOOLONG)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SEGID contains non-printable characters.", ctx)?;

    fstr::assign(&mut save.SEGID, DEFSID);

    fstr::assign(
        fstr::substr_mut(&mut save.SEGID, 5..=5),
        &intrinsics::CHAR(7),
    );

    spicelib::SPKW21(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.N,
        DLMAX,
        save.DLBUFF.as_slice(),
        save.TBUFF.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NONPRINTABLECHARS)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid difference line count", ctx)?;

    fstr::assign(&mut save.SEGID, DEFSID);
    save.N = 0;

    spicelib::SPKW21(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.N,
        DLMAX,
        save.DLBUFF.as_slice(),
        save.TBUFF.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDCOUNT)", OK, ctx)?;

    save.N = -1;

    spicelib::SPKW21(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.N,
        DLMAX,
        save.DLBUFF.as_slice(),
        save.TBUFF.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDCOUNT)", OK, ctx)?;

    save.N = 100;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Descriptor times out of order", ctx)?;

    save.FIRST = (save.LAST + 1.0);

    spicelib::SPKW21(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.N,
        DLMAX,
        save.DLBUFF.as_slice(),
        save.TBUFF.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADDESCRTIMES)", OK, ctx)?;

    save.FIRST = 0.0;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"epochs out of order", ctx)?;

    save.ET = save.TBUFF[3];
    save.TBUFF[3] = save.TBUFF[2];

    spicelib::SPKW21(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.N,
        DLMAX,
        save.DLBUFF.as_slice(),
        save.TBUFF.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(TIMESOUTOFORDER)", OK, ctx)?;

    save.TBUFF[3] = save.ET;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Gap following last epoch", ctx)?;

    save.ET = save.TBUFF[save.N];
    save.TBUFF[save.N] = (save.TBUFF[save.N] - 0.000001);

    spicelib::SPKW21(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.N,
        DLMAX,
        save.DLBUFF.as_slice(),
        save.TBUFF.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(COVERAGEGAP)", OK, ctx)?;

    save.TBUFF[save.N] = save.ET;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"DLMAX is out of range.", ctx)?;

    spicelib::SPKW21(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.N,
        200,
        save.DLBUFF.as_slice(),
        save.TBUFF.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(DIFFLINETOOLARGE)", OK, ctx)?;

    spicelib::SPKW21(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.N,
        0,
        save.DLBUFF.as_slice(),
        save.TBUFF.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(DIFFLINETOOSMALL)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Zero step in step size array", ctx)?;

    //
    // Initialize the time and data buffers with values that are
    // recognizable but otherwise bogus.
    //
    save.MAXDIM = MAXTRM;

    {
        let m1__: i32 = 1;
        let m2__: i32 = MXNREC;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.TBUFF[save.I] = ((save.I as f64) * 1000.0);

            {
                let m1__: i32 = 1;
                let m2__: i32 = DLMAX;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.DLBUFF[[save.J, save.I]] =
                        ((save.TBUFF[save.I] + save.J as f64) - 1 as f64);

                    save.J += m3__;
                }
            }

            save.DLBUFF[[save.MIOIDX, save.I]] = save.MAXDIM as f64;

            save.I += m3__;
        }
    }

    save.DLBUFF[[2, 3]] = 0.0;

    spicelib::SPKW21(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.N,
        DLMAX,
        save.DLBUFF.as_slice(),
        save.TBUFF.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(ZEROSTEP)", OK, ctx)?;

    //
    // Restore original step value.
    //
    save.DLBUFF[[2, 3]] = ((save.TBUFF[3] + 2 as f64) - 1 as f64);

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Valid zero step in step size array", ctx)?;

    //
    // This is a non-error exceptional case.
    //
    save.N = 1;
    save.MAXDIM = 15;
    save.J = ((4 * save.MAXDIM) + 11);
    save.DLBUFF[[1, 1]] = save.J as f64;

    save.MIOIDX = ((4 * save.MAXDIM) + 8);

    save.DLBUFF[[save.MIOIDX, 1]] = 4.0;
    save.DLBUFF[[(save.MIOIDX + 1), 1]] = 3.0;
    save.DLBUFF[[(save.MIOIDX + 2), 1]] = 2.0;
    save.DLBUFF[[(save.MIOIDX + 3), 1]] = 1.0;

    //
    // I is the index of the last non-zero step in the step size
    // table. I corresponds to MQ2 (== KQMAX1-2) in SPKE21.
    //
    save.I = intrinsics::IDNINT((save.DLBUFF[[save.MIOIDX, 1]] - 2 as f64));

    spicelib::CLEARD(
        (save.MAXDIM - save.I),
        save.DLBUFF.subarray_mut([(2 + save.I), 1]),
    );

    // DO I = 1, J
    //    WRITE (*,*) I, ' ', DLBUFF(I,1)
    // END DO

    spicelib::SPKW21(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.TBUFF[1],
        &save.SEGID,
        save.N,
        save.J,
        save.DLBUFF.as_slice(),
        save.TBUFF.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case clean-up.", ctx)?;
    //
    // Delete SPK1.
    //
    spicelib::DELFIL(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*****************************************************************
    //*
    //*    SPKR21 normal cases:
    //*
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Make sure segment containing one difference line is readable.",
        ctx,
    )?;

    //
    // Re-create SPK1.
    //
    spicelib::SPKOPN(SPK1, SPK1, 0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Initialize the time and data buffers with values that are
    // recognizable but otherwise bogus.
    //
    save.MAXDIM = MAXTRM;
    save.MIOIDX = ((4 * save.MAXDIM) + 8);

    {
        let m1__: i32 = 1;
        let m2__: i32 = MXNREC;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.TBUFF[save.I] = ((save.I as f64) * 1000.0);

            {
                let m1__: i32 = 1;
                let m2__: i32 = DLMAX;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.DLBUFF[[save.J, save.I]] =
                        ((save.TBUFF[save.I] + save.J as f64) - 1 as f64);

                    save.J += m3__;
                }
            }

            save.DLBUFF[[save.MIOIDX, save.I]] = save.MAXDIM as f64;

            save.I += m3__;
        }
    }

    save.LAST = save.TBUFF[1];

    spicelib::SPKW21(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        1,
        DLMAX,
        save.DLBUFF.as_slice(),
        save.TBUFF.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFOPR(SPK1, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"Segment found", save.FOUND, true, OK, ctx)?;

    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFUS(
        save.DESCR.as_slice(),
        ND,
        NI,
        save.DC.as_slice_mut(),
        save.IC.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the descriptor.
    //
    spicelib::NAMFRM(&save.FRAME, &mut save.FRCODE, ctx)?;

    testutil::CHCKSI(b"Body", save.IC[1], b"=", save.BODY, 0, OK, ctx)?;
    testutil::CHCKSI(b"Center", save.IC[2], b"=", save.CENTER, 0, OK, ctx)?;
    testutil::CHCKSI(b"Frame", save.IC[3], b"=", save.FRCODE, 0, OK, ctx)?;
    testutil::CHCKSI(b"Data type", save.IC[4], b"=", 21, 0, OK, ctx)?;

    testutil::CHCKSD(b"Start time", save.DC[1], b"=", save.FIRST, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"Stop time", save.DC[2], b"=", save.LAST, 0.0, OK, ctx)?;

    //
    // Look up the data and compare it to what we put in.  We
    // expect an exact match.
    //
    spicelib::SPKR21(
        save.HANDLE,
        save.DESCR.as_slice(),
        save.TBUFF[1],
        save.T21REC.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKAD(
        b"Diff line",
        save.T21REC.subarray(2),
        b"=",
        save.DLBUFF.subarray([1, 1]),
        DLMAX,
        0.0,
        OK,
        ctx,
    )?;

    fstr::assign(&mut save.LABEL, b"MAXDIM");

    testutil::CHCKSD(
        &save.LABEL,
        save.T21REC[1],
        b"=",
        (save.MAXDIM as f64),
        0.0,
        OK,
        ctx,
    )?;

    spicelib::SPKCLS(save.HANDLE, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     Repeat the test with MXNREC records.  The new segment will mask
    //     the previous one.
    //
    testutil::TCASE(
        b"Create and read segment with multiple difference lines.",
        ctx,
    )?;
    spicelib::SPKOPA(SPK1, &mut save.HANDLE, ctx)?;

    save.N = MXNREC;
    save.LAST = save.TBUFF[save.N];

    spicelib::SPKW21(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.N,
        DLMAX,
        save.DLBUFF.as_slice(),
        save.TBUFF.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Open the file for read access; find the 2nd descriptor.
    //
    spicelib::DAFOPR(SPK1, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"Segment found", save.FOUND, true, OK, ctx)?;

    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFUS(
        save.DESCR.as_slice(),
        ND,
        NI,
        save.DC.as_slice_mut(),
        save.IC.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the descriptor.
    //
    spicelib::NAMFRM(&save.FRAME, &mut save.FRCODE, ctx)?;

    testutil::CHCKSI(b"Body", save.IC[1], b"=", save.BODY, 0, OK, ctx)?;
    testutil::CHCKSI(b"Center", save.IC[2], b"=", save.CENTER, 0, OK, ctx)?;
    testutil::CHCKSI(b"Frame", save.IC[3], b"=", save.FRCODE, 0, OK, ctx)?;
    testutil::CHCKSI(b"Data type", save.IC[4], b"=", 21, 0, OK, ctx)?;

    testutil::CHCKSD(b"Start time", save.DC[1], b"=", save.FIRST, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"Stop time", save.DC[2], b"=", save.LAST, 0.0, OK, ctx)?;

    //
    // Look up the data and compare it to what we put in.  We
    // expect an exact match.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.LABEL, b"Difference line number *, time *");

            spicelib::REPMI(&save.LABEL.to_vec(), b"*", save.I, &mut save.LABEL, ctx);
            spicelib::REPMD(
                &save.LABEL.to_vec(),
                b"*",
                save.TBUFF[save.I],
                14,
                &mut save.LABEL,
                ctx,
            );

            spicelib::SPKR21(
                save.HANDLE,
                save.DESCR.as_slice(),
                save.TBUFF[save.I],
                save.T21REC.as_slice_mut(),
                ctx,
            )?;

            testutil::CHCKAD(
                &save.LABEL,
                save.T21REC.subarray(2),
                b"=",
                save.DLBUFF.subarray([1, save.I]),
                DLMAX,
                0.0,
                OK,
                ctx,
            )?;

            fstr::assign(
                &mut save.LABEL,
                b"MAXDIM for difference line number *, time *",
            );

            spicelib::REPMI(&save.LABEL.to_vec(), b"*", save.I, &mut save.LABEL, ctx);
            spicelib::REPMD(
                &save.LABEL.to_vec(),
                b"*",
                save.TBUFF[save.I],
                14,
                &mut save.LABEL,
                ctx,
            );

            testutil::CHCKSD(
                &save.LABEL,
                save.T21REC[1],
                b"=",
                (save.MAXDIM as f64),
                0.0,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    spicelib::SPKCLS(save.HANDLE, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     Repeat the test with a new segment having a smaller value
    //     of MAXDIM.  The new segment will mask the previous one.
    //
    testutil::TCASE(b"Create and read segment with MAXDIM set to 15.", ctx)?;
    spicelib::SPKOPA(SPK1, &mut save.HANDLE, ctx)?;

    save.N = MXNREC;
    save.LAST = save.TBUFF[save.N];

    save.MAXDIM = 15;
    save.DLSIZE = ((4 * save.MAXDIM) + 11);
    save.MIOIDX = ((4 * save.MAXDIM) + 8);

    {
        let m1__: i32 = 1;
        let m2__: i32 = MXNREC;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.TBUFF[save.I] = ((save.I as f64) * 1000.0);

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.DLSIZE;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    //
                    // K is the offset of the (J,I) entry of DPBUFF.
                    //
                    save.K = (((save.I - 1) * save.DLSIZE) + save.J);
                    save.DPBUFF[save.K] = ((save.TBUFF[save.I] + save.J as f64) - 1 as f64);

                    save.J += m3__;
                }
            }

            save.K = (((save.I - 1) * save.DLSIZE) + save.MIOIDX);

            save.DPBUFF[save.K] = save.MAXDIM as f64;

            save.I += m3__;
        }
    }

    //
    // We need to use a different data buffer here; we'll call it
    // DPBUFF.
    //
    save.BODY = 5;

    spicelib::SPKW21(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.N,
        save.DLSIZE,
        save.DPBUFF.as_slice(),
        save.TBUFF.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Open the file for read access; find the 3rd descriptor.
    //
    spicelib::DAFOPR(SPK1, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::DAFFNA(&mut save.FOUND, ctx)?;
            testutil::CHCKSL(b"Segment found", save.FOUND, true, OK, ctx)?;
            save.I += m3__;
        }
    }

    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFUS(
        save.DESCR.as_slice(),
        ND,
        NI,
        save.DC.as_slice_mut(),
        save.IC.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the descriptor.
    //
    spicelib::NAMFRM(&save.FRAME, &mut save.FRCODE, ctx)?;

    testutil::CHCKSI(b"Body", save.IC[1], b"=", save.BODY, 0, OK, ctx)?;
    testutil::CHCKSI(b"Center", save.IC[2], b"=", save.CENTER, 0, OK, ctx)?;
    testutil::CHCKSI(b"Frame", save.IC[3], b"=", save.FRCODE, 0, OK, ctx)?;
    testutil::CHCKSI(b"Data type", save.IC[4], b"=", 21, 0, OK, ctx)?;

    testutil::CHCKSD(b"Start time", save.DC[1], b"=", save.FIRST, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"Stop time", save.DC[2], b"=", save.LAST, 0.0, OK, ctx)?;

    //
    // Look up the data and compare it to what we put in.  We
    // expect an exact match.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.LABEL, b"Difference line number *, time *");

            spicelib::REPMI(&save.LABEL.to_vec(), b"*", save.I, &mut save.LABEL, ctx);
            spicelib::REPMD(
                &save.LABEL.to_vec(),
                b"*",
                save.TBUFF[save.I],
                14,
                &mut save.LABEL,
                ctx,
            );

            spicelib::SPKR21(
                save.HANDLE,
                save.DESCR.as_slice(),
                save.TBUFF[save.I],
                save.T21REC.as_slice_mut(),
                ctx,
            )?;

            save.K = (((save.I - 1) * save.DLSIZE) + 1);

            testutil::CHCKAD(
                &save.LABEL,
                save.T21REC.subarray(2),
                b"=",
                save.DPBUFF.subarray(save.K),
                save.DLSIZE,
                0.0,
                OK,
                ctx,
            )?;

            fstr::assign(
                &mut save.LABEL,
                b"MAXDIM for difference line number *, time *",
            );

            spicelib::REPMI(&save.LABEL.to_vec(), b"*", save.I, &mut save.LABEL, ctx);
            spicelib::REPMD(
                &save.LABEL.to_vec(),
                b"*",
                save.TBUFF[save.I],
                14,
                &mut save.LABEL,
                ctx,
            );

            testutil::CHCKSD(
                &save.LABEL,
                save.T21REC[1],
                b"=",
                (save.MAXDIM as f64),
                0.0,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    spicelib::SPKCLS(save.HANDLE, ctx)?;

    //*****************************************************************
    //*
    //*    SPKR21/SPKE21/SPKPVN normal cases:
    //*
    //*****************************************************************

    //
    // We'll now create a type 21 segment containing some more or
    // less realistic data. We'll start states sampled from a
    // Jupiter barycenter segment created by TSTSPK. We'll use
    // these to create a type 13 record, which we'll then convert
    // to a type 21 record.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create realistic data for a type 21 record.", ctx)?;

    //
    // Start out with a generic SPK file.
    //
    testutil::TSTSPK(SPK0, false, &mut save.HAN0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a type 13 record from a sequence of
    // epochs and states.
    //
    save.BODY = 5;
    save.CENTER = 10;

    save.N = 10;
    save.STEP = spicelib::SPD();

    save.FIRST = 0 as f64;
    save.LAST = (save.FIRST + (((save.N - 1) as f64) * save.STEP));

    fstr::assign(&mut save.SEGID, b"Type 21 Jup Barycenter MAXDIM = 21");

    fstr::assign(&mut save.FRAME, b"J2000");

    spicelib::FURNSH(SPK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.T13REC[1] = (save.N as f64);

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.ET = (save.FIRST + (((save.I - 1) as f64) * save.STEP));

            spicelib::SPKEZ(
                save.BODY,
                save.ET,
                &save.FRAME,
                b"NONE",
                save.CENTER,
                save.STATE.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.J = (2 + ((save.I - 1) * 6));

            spicelib::MOVED(save.STATE.as_slice(), 6, save.T13REC.subarray_mut(save.J));

            save.J = ((1 + (save.N * 6)) + save.I);

            save.T13REC[save.J] = save.ET;

            save.I += m3__;
        }
    }

    spicelib::UNLOAD(SPK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create a type 21 record from a type 13 record.", ctx)?;

    testutil::T_T13XMD(
        save.T13REC.as_slice(),
        save.FIRST,
        save.LAST,
        save.T21REC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"Type 21 record created", save.FOUND, true, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create a new SPK from a type 21 record.", ctx)?;

    if spicelib::EXISTS(SPK2, ctx)? {
        spicelib::DELFIL(SPK2, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::SPKOPN(SPK2, SPK2, 0, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note that our time buffer contains a single element.
    //
    spicelib::SPKW21(
        save.HAN2,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        1,
        DLMAX,
        save.T21REC.subarray(2),
        &[save.LAST],
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKCLS(save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Here's the main event: test the states obtained from the
    // type 21 segment.
    //

    testutil::TCASE(b"Recover states from the type 21 segment.", ctx)?;

    spicelib::FURNSH(SPK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.ET = (save.FIRST + (((save.I - 1) as f64) * save.STEP));

            spicelib::SPKEZ(
                save.BODY,
                save.ET,
                &save.FRAME,
                b"NONE",
                save.CENTER,
                save.STATE.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.J = (2 + ((save.I - 1) * 6));

            spicelib::MOVED(save.T13REC.subarray(save.J), 6, save.XSTATE.as_slice_mut());

            fstr::assign(&mut save.LABEL, b"Position number *, time *");

            spicelib::REPMI(&save.LABEL.to_vec(), b"*", save.I, &mut save.LABEL, ctx);
            spicelib::REPMD(
                &save.LABEL.to_vec(),
                b"*",
                save.ET,
                14,
                &mut save.LABEL,
                ctx,
            );

            testutil::CHCKAD(
                &save.LABEL,
                save.STATE.as_slice(),
                b"~~/",
                save.XSTATE.as_slice(),
                3,
                TIGHT,
                OK,
                ctx,
            )?;

            fstr::assign(&mut save.LABEL, b"Velocity number *, time *");

            spicelib::REPMI(&save.LABEL.to_vec(), b"*", save.I, &mut save.LABEL, ctx);
            spicelib::REPMD(
                &save.LABEL.to_vec(),
                b"*",
                save.ET,
                14,
                &mut save.LABEL,
                ctx,
            );

            testutil::CHCKAD(
                &save.LABEL,
                save.STATE.subarray(4),
                b"~~/",
                save.XSTATE.subarray(4),
                3,
                TIGHT,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    spicelib::UNLOAD(SPK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*****************************************************************
    //*
    //*    SPKS21 normal cases:
    //*
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKS21 test: subset small segment.", ctx)?;

    //
    // SPK2 has realistic MDA data.
    //
    spicelib::DAFOPR(SPK2, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DAFGN(&mut save.SEGID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFUS(
        save.DESCR.as_slice(),
        ND,
        NI,
        save.DC.as_slice_mut(),
        save.IC.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.FIRST = (save.DC[1] + 6400.0);
    save.LAST = (save.DC[2] - 6400.0);

    if spicelib::EXISTS(SPK21S, ctx)? {
        spicelib::DELFIL(SPK21S, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::SPKOPN(SPK21S, b" ", 0, &mut save.HANS, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKSUB(
        save.HANDLE,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.FIRST,
        save.LAST,
        save.HANS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close subsetted SPK.
    //
    spicelib::SPKCLS(save.HANS, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close source SPK.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Re-open subsetted SPK for DAF search.
    //
    spicelib::DAFOPR(SPK21S, &mut save.HANS, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANS, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DAFGN(&mut save.SEGID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFUS(
        save.DESCR.as_slice(),
        ND,
        NI,
        save.DC.as_slice_mut(),
        save.IC.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BODY = save.IC[1];
    save.CENTER = save.IC[2];

    //
    // Close subsetted SPK and re-open for sampling.
    //
    spicelib::DAFCLS(save.HANS, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(SPK21S, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get expected states from source SPK.
    //
    save.NSAMP = 100001;

    save.DELTA = ((save.LAST - save.FIRST) / (save.NSAMP - 1) as f64);

    spicelib::FURNSH(SPK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Sample and check states from the new type 21 SPK file.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSAMP;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.ET = (save.FIRST + (((save.I - 1) as f64) * save.DELTA));

            spicelib::SPKEZ(
                save.BODY,
                save.ET,
                &save.FRAME,
                b"NONE",
                save.CENTER,
                save.XSTBUF.subarray_mut([1, save.I]),
                &mut save.LT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    spicelib::UNLOAD(SPK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Re-open subsetted SPK for sampling.
    //
    spicelib::FURNSH(SPK21S, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Sample and check states from the new type 21 SPK file.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSAMP;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.ET = (save.FIRST + (((save.I - 1) as f64) * save.DELTA));

            spicelib::SPKEZ(
                save.BODY,
                save.ET,
                &save.FRAME,
                b"NONE",
                save.CENTER,
                save.STABUF.subarray_mut([1, save.I]),
                &mut save.LT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    spicelib::UNLOAD(SPK21S, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Test position results.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSAMP;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.LABEL, b"Pos(@)");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", save.I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.STABUF.subarray([1, save.I]),
                b"~~/",
                save.XSTBUF.subarray([1, save.I]),
                3,
                VTIGHT,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // Test velocity results.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSAMP;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.LABEL, b"Vel(@)");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", save.I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.STABUF.subarray([4, save.I]),
                b"~~/",
                save.XSTBUF.subarray([4, save.I]),
                3,
                VTIGHT,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    spicelib::UNLOAD(SPK21S, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKS21 test: subset large segment.", ctx)?;

    //
    // We'll use an input segment that has unrealistic data
    // but a large number of records.
    //
    spicelib::DAFOPR(SPK1, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Subset the third segment.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::DAFFNA(&mut save.FOUND, ctx)?;
            testutil::CHCKSL(b"Segment found", save.FOUND, true, OK, ctx)?;
            save.I += m3__;
        }
    }

    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DAFGN(&mut save.SEGID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFUS(
        save.DESCR.as_slice(),
        ND,
        NI,
        save.DC.as_slice_mut(),
        save.IC.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BODY = save.IC[1];
    save.CENTER = save.IC[2];
    save.FRCODE = save.IC[3];

    //
    // This time chop off 10 days at both ends of the segment.
    //
    save.FIRST = (save.DC[1] + ((10 as f64) * spicelib::SPD()));
    save.LAST = (save.DC[2] - ((10 as f64) * spicelib::SPD()));

    if spicelib::EXISTS(SPK21S, ctx)? {
        spicelib::DELFIL(SPK21S, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::SPKOPN(SPK21S, b" ", 0, &mut save.HANS, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKSUB(
        save.HANDLE,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.FIRST,
        save.LAST,
        save.HANS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close subsetted SPK.
    //
    spicelib::SPKCLS(save.HANS, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close source SPK.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Open the subsetted SPK.
    //
    spicelib::DAFOPR(SPK21S, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFUS(
        save.DESCR.as_slice(),
        ND,
        NI,
        save.DC.as_slice_mut(),
        save.IC.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the descriptor.
    //
    spicelib::NAMFRM(&save.FRAME, &mut save.FRCODE, ctx)?;

    testutil::CHCKSI(b"Body", save.IC[1], b"=", save.BODY, 0, OK, ctx)?;
    testutil::CHCKSI(b"Center", save.IC[2], b"=", save.CENTER, 0, OK, ctx)?;
    testutil::CHCKSI(b"Frame", save.IC[3], b"=", save.FRCODE, 0, OK, ctx)?;
    testutil::CHCKSI(b"Data type", save.IC[4], b"=", 21, 0, OK, ctx)?;

    testutil::CHCKSD(b"Start time", save.DC[1], b"=", save.FIRST, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"Stop time", save.DC[2], b"=", save.LAST, 0.0, OK, ctx)?;

    //
    // Look up the data and compare it to what we put in.  We
    // expect an exact match.
    //
    save.MAXDIM = 15;
    save.DLSIZE = ((4 * save.MAXDIM) + 11);
    save.MIOIDX = ((4 * save.MAXDIM) + 8);

    //
    // Re-create the test data for the third segment of SPK1.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = MXNREC;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.TBUFF[save.I] = ((save.I as f64) * 1000.0);

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.DLSIZE;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    //
                    // K is the offset of the (J,I) entry of DPBUFF.
                    //
                    save.K = (((save.I - 1) * save.DLSIZE) + save.J);
                    save.DPBUFF[save.K] = ((save.TBUFF[save.I] + save.J as f64) - 1 as f64);

                    save.J += m3__;
                }
            }

            save.K = (((save.I - 1) * save.DLSIZE) + save.MIOIDX);

            save.DPBUFF[save.K] = save.MAXDIM as f64;

            save.I += m3__;
        }
    }

    save.J = intrinsics::IDNINT((save.FIRST / 1000 as f64));
    save.K = intrinsics::IDNINT((save.LAST / 1000 as f64));

    {
        let m1__: i32 = save.J;
        let m2__: i32 = save.K;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.LABEL, b"Difference line number *, time *");

            spicelib::REPMI(&save.LABEL.to_vec(), b"*", save.I, &mut save.LABEL, ctx);
            spicelib::REPMD(
                &save.LABEL.to_vec(),
                b"*",
                save.TBUFF[save.I],
                14,
                &mut save.LABEL,
                ctx,
            );

            spicelib::SPKR21(
                save.HANDLE,
                save.DESCR.as_slice(),
                save.TBUFF[save.I],
                save.T21REC.as_slice_mut(),
                ctx,
            )?;

            save.K = (((save.I - 1) * save.DLSIZE) + 1);

            testutil::CHCKAD(
                &save.LABEL,
                save.T21REC.subarray(2),
                b"=",
                save.DPBUFF.subarray(save.K),
                save.DLSIZE,
                0.0,
                OK,
                ctx,
            )?;

            fstr::assign(
                &mut save.LABEL,
                b"MAXDIM for difference line number *, time *",
            );

            spicelib::REPMI(&save.LABEL.to_vec(), b"*", save.I, &mut save.LABEL, ctx);
            spicelib::REPMD(
                &save.LABEL.to_vec(),
                b"*",
                save.TBUFF[save.I],
                14,
                &mut save.LABEL,
                ctx,
            );

            testutil::CHCKSD(
                &save.LABEL,
                save.T21REC[1],
                b"=",
                (save.MAXDIM as f64),
                0.0,
                OK,
                ctx,
            )?;
            save.I += m3__;
        }
    }

    spicelib::SPKCLS(save.HANDLE, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     Close and delete the SPK files.
    //
    testutil::TCASE(b"Clean up.", ctx)?;

    spicelib::DAFCLS(save.HAN0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFCLS(save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK21S, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
