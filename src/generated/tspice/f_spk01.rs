//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXREC: i32 = 198;
const MAXTRM: i32 = 25;
const SPK0: &[u8] = b"test.bsp";
const SPK1: &[u8] = b"spk_test_01_v1.bsp";
const SPK2: &[u8] = b"spk_test_01_v2.bsp";
const DEFSID: &[u8] = b"spkw01 test segment";
const TIGHT: f64 = 0.000000000005;
const MAXT01: i32 = 15;
const MDASZ1: i32 = 71;
const DLMAX: i32 = ((4 * MAXTRM) + 11);
const MXNREC: i32 = 10000;
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
    DESCR: StackArray<f64, 5>,
    DLBUFF: ActualArray2D<f64>,
    TBUFF: ActualArray<f64>,
    FIRST: f64,
    LAST: f64,
    LT: f64,
    ET: f64,
    STATE: StackArray<f64, 6>,
    STEP: f64,
    T13REC: StackArray<f64, 198>,
    T21REC: StackArray<f64, 112>,
    T01REC: StackArray<f64, 71>,
    XSTATE: StackArray<f64, 6>,
    BODY: i32,
    CENTER: i32,
    FRCODE: i32,
    HAN0: i32,
    HAN2: i32,
    HANDLE: i32,
    IC: StackArray<i32, 6>,
    J: i32,
    N: i32,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SEGID = vec![b' '; LNSIZE as usize];
        let mut FRAME = vec![b' '; FRNMLN as usize];
        let mut LABEL = vec![b' '; LNSIZE as usize];
        let mut DC = StackArray::<f64, 2>::new(1..=ND);
        let mut DESCR = StackArray::<f64, 5>::new(1..=DSCSIZ);
        let mut DLBUFF = ActualArray2D::<f64>::new(1..=MDASZ1, 1..=MXNREC);
        let mut TBUFF = ActualArray::<f64>::new(1..=MXNREC);
        let mut FIRST: f64 = 0.0;
        let mut LAST: f64 = 0.0;
        let mut LT: f64 = 0.0;
        let mut ET: f64 = 0.0;
        let mut STATE = StackArray::<f64, 6>::new(1..=6);
        let mut STEP: f64 = 0.0;
        let mut T13REC = StackArray::<f64, 198>::new(1..=MAXREC);
        let mut T21REC = StackArray::<f64, 112>::new(1..=(DLMAX + 1));
        let mut T01REC = StackArray::<f64, 71>::new(1..=MDASZ1);
        let mut XSTATE = StackArray::<f64, 6>::new(1..=6);
        let mut BODY: i32 = 0;
        let mut CENTER: i32 = 0;
        let mut FRCODE: i32 = 0;
        let mut HAN0: i32 = 0;
        let mut HAN2: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut IC = StackArray::<i32, 6>::new(1..=NI);
        let mut J: i32 = 0;
        let mut N: i32 = 0;
        let mut FOUND: bool = false;

        Self {
            SEGID,
            FRAME,
            LABEL,
            DC,
            DESCR,
            DLBUFF,
            TBUFF,
            FIRST,
            LAST,
            LT,
            ET,
            STATE,
            STEP,
            T13REC,
            T21REC,
            T01REC,
            XSTATE,
            BODY,
            CENTER,
            FRCODE,
            HAN0,
            HAN2,
            HANDLE,
            IC,
            J,
            N,
            FOUND,
        }
    }
}

//$Procedure F_SPK01 ( SPK data type 01 tests )
pub fn F_SPK01(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    testutil::TOPEN(b"F_SPK01", ctx)?;

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
    for I in 1..=MXNREC {
        save.TBUFF[I] = ((I as f64) * 1000.0);

        {
            let m1__: i32 = 1;
            let m2__: i32 = MDASZ1;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.DLBUFF[[save.J, I]] = ((save.TBUFF[I] + save.J as f64) - 1 as f64);

                save.J += m3__;
            }
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
    //*    SPKW01 error cases:
    //*
    //*****************************************************************
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid frame.", ctx)?;

    spicelib::SPKW01(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        b"XXX",
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.N,
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

    spicelib::SPKW01(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.N,
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

    spicelib::SPKW01(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.N,
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

    spicelib::SPKW01(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.N,
        save.DLBUFF.as_slice(),
        save.TBUFF.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDCOUNT)", OK, ctx)?;

    save.N = -1;

    spicelib::SPKW01(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.N,
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

    spicelib::SPKW01(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.N,
        save.DLBUFF.as_slice(),
        save.TBUFF.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADDESCRTIMES)", OK, ctx)?;

    save.FIRST = 0.0;

    save.ET = save.LAST;
    save.LAST = (save.FIRST - 1.0);

    spicelib::SPKW01(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.N,
        save.DLBUFF.as_slice(),
        save.TBUFF.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADDESCRTIMES)", OK, ctx)?;

    save.LAST = save.ET;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"epochs out of order", ctx)?;

    save.ET = save.TBUFF[3];
    save.TBUFF[3] = save.TBUFF[2];

    spicelib::SPKW01(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.N,
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

    spicelib::SPKW01(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.N,
        save.DLBUFF.as_slice(),
        save.TBUFF.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADDESCRTIMES)", OK, ctx)?;

    save.TBUFF[save.N] = save.ET;

    //*****************************************************************
    //*
    //*    SPKR01 normal cases:
    //*
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Make sure segment containing one difference line is readable.",
        ctx,
    )?;

    save.LAST = save.TBUFF[1];

    spicelib::SPKW01(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        1,
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
    testutil::CHCKSI(b"Data type", save.IC[4], b"=", 1, 0, OK, ctx)?;

    testutil::CHCKSD(b"Start time", save.DC[1], b"=", save.FIRST, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"Stop time", save.DC[2], b"=", save.LAST, 0.0, OK, ctx)?;

    //
    // Look up the data and compare it to what we put in.  We
    // expect an exact match.
    //
    spicelib::SPKR01(
        save.HANDLE,
        save.DESCR.as_slice(),
        save.TBUFF[1],
        save.T01REC.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKAD(
        b"Diff line",
        save.T01REC.subarray(1),
        b"=",
        save.DLBUFF.subarray([1, 1]),
        MDASZ1,
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

    spicelib::SPKW01(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.N,
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
    testutil::CHCKSI(b"Data type", save.IC[4], b"=", 1, 0, OK, ctx)?;

    testutil::CHCKSD(b"Start time", save.DC[1], b"=", save.FIRST, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"Stop time", save.DC[2], b"=", save.LAST, 0.0, OK, ctx)?;

    //
    // Look up the data and compare it to what we put in.  We
    // expect an exact match.
    //
    for I in 1..=save.N {
        fstr::assign(&mut save.LABEL, b"Difference line number *, time *");

        spicelib::REPMI(&save.LABEL.to_vec(), b"*", I, &mut save.LABEL, ctx);
        spicelib::REPMD(
            &save.LABEL.to_vec(),
            b"*",
            save.TBUFF[I],
            14,
            &mut save.LABEL,
            ctx,
        );

        spicelib::SPKR01(
            save.HANDLE,
            save.DESCR.as_slice(),
            save.TBUFF[I],
            save.T01REC.as_slice_mut(),
            ctx,
        )?;

        testutil::CHCKAD(
            &save.LABEL,
            save.T01REC.subarray(1),
            b"=",
            save.DLBUFF.subarray([1, I]),
            MDASZ1,
            0.0,
            OK,
            ctx,
        )?;
    }

    spicelib::SPKCLS(save.HANDLE, ctx)?;

    //*****************************************************************
    //*
    //*    SPKR01/SPKE01/SPKPVN normal cases:
    //*
    //*****************************************************************

    //
    // We'll now create a type 1 segment containing some more or
    // less realistic data. We'll start states sampled from a
    // Jupiter barycenter segment created by TSTSPK. We'll use
    // these to create a type 13 record, which we'll then convert
    // to a type 1 record.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create realistic data for a type 01 record.", ctx)?;

    //
    // Start out with a generic SPK file.
    //
    testutil::TSTSPK(SPK0, false, &mut save.HAN0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a type 13 record from a sequence of
    // epochs and states. Use a polynomial degree of 15.
    // The corresponding difference line will have
    // 14 terms in each component of the difference table.
    //
    save.BODY = 5;
    save.CENTER = 10;

    save.N = 8;
    save.STEP = spicelib::SPD();

    save.FIRST = 0 as f64;
    save.LAST = (save.FIRST + (((save.N - 1) as f64) * save.STEP));

    fstr::assign(&mut save.SEGID, b"Type 01 Jup Barycenter");

    fstr::assign(&mut save.FRAME, b"J2000");

    spicelib::FURNSH(SPK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.T13REC[1] = (save.N as f64);

    for I in 1..=save.N {
        save.ET = (save.FIRST + (((I - 1) as f64) * save.STEP));

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

        save.J = (2 + ((I - 1) * 6));

        spicelib::MOVED(save.STATE.as_slice(), 6, save.T13REC.subarray_mut(save.J));

        save.J = ((1 + (save.N * 6)) + I);

        save.T13REC[save.J] = save.ET;
    }

    spicelib::UNLOAD(SPK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create a type 01 record from a type 13 record.", ctx)?;

    // CALL T13MDA ( T13REC, FIRST, LAST, T01REC, FOUND )
    testutil::T_T13XMD(
        save.T13REC.as_slice(),
        save.FIRST,
        save.LAST,
        save.T21REC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a type 01 record from the type 21 record we just
    // created.
    //
    spicelib::CLEARD(MDASZ1, save.T01REC.as_slice_mut());

    //
    // Copy the reference epoch.
    //
    save.T01REC[1] = save.T21REC[2];

    //
    // Copy the stepsize vector.
    //
    spicelib::MOVED(save.T21REC.subarray(3), MAXT01, save.T01REC.subarray_mut(2));

    //
    // Copy the reference position and velocity.
    //
    spicelib::MOVED(
        save.T21REC.subarray((MAXTRM + 3)),
        6,
        save.T01REC.subarray_mut(17),
    );

    //
    // Copy the difference table. Since each component of the type 21
    // difference table has a difference size from the corresponding
    // type 1 component, we must copy the components individually.
    //
    spicelib::MOVED(
        save.T21REC.subarray((MAXTRM + 9)),
        MAXT01,
        save.T01REC.subarray_mut(23),
    );
    spicelib::MOVED(
        save.T21REC.subarray(((2 * MAXTRM) + 9)),
        MAXT01,
        save.T01REC.subarray_mut(38),
    );
    spicelib::MOVED(
        save.T21REC.subarray(((3 * MAXTRM) + 9)),
        MAXT01,
        save.T01REC.subarray_mut(53),
    );

    //
    // Don't copy the order parameters; set the parameters for
    // type 1
    //
    save.T01REC[68] = 15 as f64;
    save.T01REC[69] = 14 as f64;
    save.T01REC[70] = 14 as f64;
    save.T01REC[71] = 14 as f64;

    testutil::CHCKSL(b"Type 01 record created", save.FOUND, true, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create a new SPK from a type 01 record.", ctx)?;

    if spicelib::EXISTS(SPK2, ctx)? {
        spicelib::DELFIL(SPK2, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::SPKOPN(SPK2, SPK2, 0, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note that our time buffer contains a single element.
    //
    spicelib::SPKW01(
        save.HAN2,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        1,
        save.T01REC.subarray(1),
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
    // type 01 segment.
    //

    testutil::TCASE(b"Recover states from the type 01 segment.", ctx)?;

    spicelib::FURNSH(SPK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=save.N {
        save.ET = spicelib::BRCKTD(
            (save.FIRST + (((I - 1) as f64) * save.STEP)),
            save.FIRST,
            save.LAST,
        );

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

        save.J = (2 + ((I - 1) * 6));

        spicelib::MOVED(save.T13REC.subarray(save.J), 6, save.XSTATE.as_slice_mut());

        fstr::assign(&mut save.LABEL, b"Position number *, time *");

        spicelib::REPMI(&save.LABEL.to_vec(), b"*", I, &mut save.LABEL, ctx);
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

        spicelib::REPMI(&save.LABEL.to_vec(), b"*", I, &mut save.LABEL, ctx);
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
    }

    spicelib::UNLOAD(SPK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     Close and delete the SPK file.
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

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
