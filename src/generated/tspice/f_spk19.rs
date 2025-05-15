//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXDEG: i32 = 27;
const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const S19TP0: i32 = 0;
const S19TP1: i32 = (S19TP0 + 1);
const S19TP2: i32 = (S19TP1 + 1);
const S19PS0: i32 = 12;
const S19PS1: i32 = 6;
const S19PS2: i32 = 6;
const S19NST: i32 = 3;
const S19MXZ: i32 = S19PS0;
const S19MNZ: i32 = S19PS1;
const MAXRSZ: i32 = (2 + ((MAXDEG + 1) * (S19PS1 + 1)));
const SPK19E: &[u8] = b"test19err.bsp";
const SP19T0: &[u8] = b"sp19t0.bsp";
const SP19T1: &[u8] = b"sp19t1.bsp";
const SP19T2: &[u8] = b"sp19t2.bsp";
const TIGHT: f64 = 0.00000000000001;
const MAXIVL: i32 = 21000;
const BIGN: i32 = (MAXIVL * 100);
const LNSIZE: i32 = 80;
const MSGLEN: i32 = 800;
const NDSCRT: i32 = 9;
const SIDLEN: i32 = 60;
const FRNMLN: i32 = 32;

struct SaveVars {
    NAME: Vec<u8>,
    SEGID: Vec<u8>,
    TITLE: Vec<u8>,
    XREF: Vec<u8>,
    DSCEPC: StackArray<f64, 9>,
    DSCSTS: StackArray2D<f64, 54>,
    EPOCHS: ActualArray<f64>,
    ENDEPC: f64,
    ESCALE: f64,
    ET: f64,
    FIRST: f64,
    IVLBDS: ActualArray<f64>,
    LAST: f64,
    LT: f64,
    MIDET: f64,
    MIDSTA: StackArray<f64, 6>,
    PACKTS: ActualArray<f64>,
    STATE: StackArray<f64, 6>,
    XSTATE: StackArray<f64, 6>,
    XVALS: StackArray<f64, 28>,
    YVALS: StackArray<f64, 28>,
    WORK: StackArray<f64, 28>,
    B: i32,
    DEGRES: ActualArray<i32>,
    E: i32,
    EPCTO: i32,
    HANDLE: i32,
    LB: i32,
    NINTVL: i32,
    NPKTS: ActualArray<i32>,
    PKTSIZ: i32,
    PKTTO: i32,
    PKTTOT: i32,
    RECTOT: i32,
    SUBTPS: ActualArray<i32>,
    UB: i32,
    XBODY: i32,
    XCENTR: i32,
    WINSIZ: i32,
    SELLST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut NAME = vec![b' '; LNSIZE as usize];
        let mut SEGID = vec![b' '; SIDLEN as usize];
        let mut TITLE = vec![b' '; MSGLEN as usize];
        let mut XREF = vec![b' '; FRNMLN as usize];
        let mut DSCEPC = StackArray::<f64, 9>::new(1..=NDSCRT);
        let mut DSCSTS = StackArray2D::<f64, 54>::new(1..=6, 1..=NDSCRT);
        let mut EPOCHS = ActualArray::<f64>::new(1..=BIGN);
        let mut ENDEPC: f64 = 0.0;
        let mut ESCALE: f64 = 0.0;
        let mut ET: f64 = 0.0;
        let mut FIRST: f64 = 0.0;
        let mut IVLBDS = ActualArray::<f64>::new(1..=(MAXIVL + 1));
        let mut LAST: f64 = 0.0;
        let mut LT: f64 = 0.0;
        let mut MIDET: f64 = 0.0;
        let mut MIDSTA = StackArray::<f64, 6>::new(1..=6);
        let mut PACKTS = ActualArray::<f64>::new(1..=(BIGN * S19PS0));
        let mut STATE = StackArray::<f64, 6>::new(1..=6);
        let mut XSTATE = StackArray::<f64, 6>::new(1..=6);
        let mut XVALS = StackArray::<f64, 28>::new(1..=(MAXDEG + 1));
        let mut YVALS = StackArray::<f64, 28>::new(1..=(MAXDEG + 1));
        let mut WORK = StackArray::<f64, 28>::new(1..=(MAXDEG + 1));
        let mut B: i32 = 0;
        let mut DEGRES = ActualArray::<i32>::new(1..=MAXIVL);
        let mut E: i32 = 0;
        let mut EPCTO: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut LB: i32 = 0;
        let mut NINTVL: i32 = 0;
        let mut NPKTS = ActualArray::<i32>::new(1..=MAXIVL);
        let mut PKTSIZ: i32 = 0;
        let mut PKTTO: i32 = 0;
        let mut PKTTOT: i32 = 0;
        let mut RECTOT: i32 = 0;
        let mut SUBTPS = ActualArray::<i32>::new(1..=MAXIVL);
        let mut UB: i32 = 0;
        let mut XBODY: i32 = 0;
        let mut XCENTR: i32 = 0;
        let mut WINSIZ: i32 = 0;
        let mut SELLST: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(100.0),
                Val::D(200.0),
                Val::D(300.0),
                Val::D(400.0),
                Val::D(500.0),
                Val::D(600.0),
                Val::D(700.0),
                Val::D(800.0),
                Val::D(900.0),
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
                Val::D(101.0),
                Val::D(201.0),
                Val::D(301.0),
                Val::D(401.0),
                Val::D(501.0),
                Val::D(601.0),
                Val::D(102.0),
                Val::D(202.0),
                Val::D(302.0),
                Val::D(402.0),
                Val::D(502.0),
                Val::D(602.0),
                Val::D(103.0),
                Val::D(203.0),
                Val::D(303.0),
                Val::D(403.0),
                Val::D(503.0),
                Val::D(603.0),
                Val::D(104.0),
                Val::D(204.0),
                Val::D(304.0),
                Val::D(404.0),
                Val::D(504.0),
                Val::D(604.0),
                Val::D(105.0),
                Val::D(205.0),
                Val::D(305.0),
                Val::D(405.0),
                Val::D(505.0),
                Val::D(605.0),
                Val::D(106.0),
                Val::D(206.0),
                Val::D(306.0),
                Val::D(406.0),
                Val::D(506.0),
                Val::D(606.0),
                Val::D(107.0),
                Val::D(207.0),
                Val::D(307.0),
                Val::D(407.0),
                Val::D(507.0),
                Val::D(607.0),
                Val::D(108.0),
                Val::D(208.0),
                Val::D(308.0),
                Val::D(408.0),
                Val::D(508.0),
                Val::D(608.0),
                Val::D(109.0),
                Val::D(209.0),
                Val::D(309.0),
                Val::D(409.0),
                Val::D(509.0),
                Val::D(609.0),
            ]
            .into_iter();
            DSCSTS
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            NAME,
            SEGID,
            TITLE,
            XREF,
            DSCEPC,
            DSCSTS,
            EPOCHS,
            ENDEPC,
            ESCALE,
            ET,
            FIRST,
            IVLBDS,
            LAST,
            LT,
            MIDET,
            MIDSTA,
            PACKTS,
            STATE,
            XSTATE,
            XVALS,
            YVALS,
            WORK,
            B,
            DEGRES,
            E,
            EPCTO,
            HANDLE,
            LB,
            NINTVL,
            NPKTS,
            PKTSIZ,
            PKTTO,
            PKTTOT,
            RECTOT,
            SUBTPS,
            UB,
            XBODY,
            XCENTR,
            WINSIZ,
            SELLST,
        }
    }
}

//$Procedure F_SPK19 ( SPK data type 19 tests )
pub fn F_SPK19(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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

    //
    // Initial values
    //

    //
    // Epochs and states:
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_SPK19", ctx)?;

    //*****************************************************************
    //*
    //*    SPKW19 error cases:
    //*
    //*****************************************************************

    //
    //     Test SPKW19:  start out with error handling.
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKW19 error case: bad frame name.", ctx)?;

    //
    // Initialize the inputs to SPKW19.
    //
    save.XBODY = 3;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");
    //
    // Create a segment identifier.
    //
    fstr::assign(&mut save.SEGID, b"SPK type 19 test segment");

    //
    // Open a new SPK file.
    //
    if spicelib::EXISTS(SPK19E, ctx)? {
        spicelib::DELFIL(SPK19E, ctx)?;
    }

    spicelib::SPKOPN(
        SPK19E,
        b"Type 19 SPK internal file name",
        4,
        &mut save.HANDLE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NINTVL = 1;
    save.NPKTS[1] = 2;
    save.SUBTPS[1] = S19TP1;
    save.DEGRES[1] = 1;

    save.RECTOT = 0;
    save.PKTTOT = 0;

    for I in 1..=save.NINTVL {
        //
        // Update total counts of records and packet elements.
        // The latter depends on the packet size of the current
        // mini-segment.
        //
        if (save.SUBTPS[I] == S19TP0) {
            save.PKTSIZ = S19PS0;
        } else if (save.SUBTPS[I] == S19TP1) {
            save.PKTSIZ = S19PS1;
        } else {
            spicelib::SETMSG(
                b"Unexpected SPK type 19 subtype # found in mini-segment #.",
                ctx,
            );
            spicelib::ERRINT(b"#", save.SUBTPS[I], ctx);
            spicelib::ERRINT(b"#", I, ctx);
            spicelib::SIGERR(b"SPICE(INVALIDSUBTYPE)", ctx)?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        save.PKTTOT = (save.PKTTOT + (save.NPKTS[I] * save.PKTSIZ));

        save.RECTOT = (save.RECTOT + save.NPKTS[I]);
    }

    spicelib::CLEARD(save.PKTTOT, save.PACKTS.as_slice_mut());

    for I in 1..=save.RECTOT {
        save.EPOCHS[I] = (I as f64);
    }

    save.FIRST = save.EPOCHS[1];
    save.LAST = save.EPOCHS[2];

    save.IVLBDS[1] = save.FIRST;
    save.IVLBDS[2] = save.LAST;

    save.SELLST = true;

    spicelib::SPKW19(
        save.HANDLE,
        save.XBODY,
        save.XCENTR,
        b"SPUD",
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.EPOCHS.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDREFFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKW19 error case: SEGID too long.", ctx)?;

    spicelib::SPKW19(
        save.HANDLE,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.FIRST,
        save.LAST,
        b"X                                               X",
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.EPOCHS.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SEGIDTOOLONG)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKW19 error case: unprintable SEGID characters.", ctx)?;

    spicelib::SPKW19(
        save.HANDLE,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.FIRST,
        save.LAST,
        &intrinsics::CHAR(7),
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.EPOCHS.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NONPRINTABLECHARS)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"SPKW19 error case: non-positive interval count", ctx)?;

    spicelib::SPKW19(
        save.HANDLE,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        0,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.EPOCHS.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDCOUNT)", OK, ctx)?;

    save.NINTVL = 1;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKW19 error case: non-increasing interval bounds", ctx)?;

    save.IVLBDS[2] = save.IVLBDS[1];
    save.FIRST = 1.5;
    save.LAST = 1.6;

    spicelib::SPKW19(
        save.HANDLE,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.EPOCHS.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BOUNDSOUTOFORDER)", OK, ctx)?;

    save.IVLBDS[1] = 1.0;
    save.IVLBDS[2] = 2.0;

    save.FIRST = save.IVLBDS[1];
    save.LAST = save.IVLBDS[2];

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKW19 error case: polynomial degree too high.", ctx)?;

    save.DEGRES[1] = 40;

    spicelib::SPKW19(
        save.HANDLE,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.EPOCHS.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDEGREE)", OK, ctx)?;

    save.DEGRES[1] = 1;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKW19 error case: polynomial degree too low.", ctx)?;

    save.DEGRES[1] = 0;

    spicelib::SPKW19(
        save.HANDLE,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.EPOCHS.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDEGREE)", OK, ctx)?;

    save.DEGRES[1] = 1;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKW19 error case:  odd window size.", ctx)?;

    save.DEGRES[1] = 2;

    spicelib::SPKW19(
        save.HANDLE,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.EPOCHS.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADWINDOWSIZE)", OK, ctx)?;

    save.SUBTPS[1] = S19TP0;

    spicelib::SPKW19(
        save.HANDLE,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.EPOCHS.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADWINDOWSIZE)", OK, ctx)?;

    save.SUBTPS[1] = S19TP1;
    save.DEGRES[1] = 1;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKW19 error case: too few states", ctx)?;

    save.NPKTS[1] = 1;

    spicelib::SPKW19(
        save.HANDLE,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.EPOCHS.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(TOOFEWPACKETS)", OK, ctx)?;

    save.NPKTS[1] = 2;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKW19 error case: descriptor times swapped.", ctx)?;

    spicelib::SPKW19(
        save.HANDLE,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.LAST,
        save.FIRST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.EPOCHS.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADDESCRTIMES)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SPKW19 error case: descriptor start time is too early.",
        ctx,
    )?;

    spicelib::SPKW19(
        save.HANDLE,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        (save.FIRST - 1.0),
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.EPOCHS.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(COVERAGEGAP)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKW19 error case: descriptor end time is too late.", ctx)?;

    spicelib::SPKW19(
        save.HANDLE,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.FIRST,
        (save.LAST + 1.0),
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.EPOCHS.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(COVERAGEGAP)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SPKW19 error case: epochs for first mini-segment are out of order.",
        ctx,
    )?;

    save.NINTVL = 1;
    save.NPKTS[1] = 4;

    for I in 1..=save.NPKTS[1] {
        save.EPOCHS[I] = (I as f64);
    }

    save.FIRST = save.EPOCHS[1];
    save.LAST = save.EPOCHS[save.NPKTS[1]];

    save.IVLBDS[1] = save.FIRST;
    save.IVLBDS[2] = save.LAST;

    spicelib::SWAPD_ARRAY(
        save.EPOCHS.subscript(2),
        save.EPOCHS.subscript(3),
        save.EPOCHS.as_slice_mut(),
    );

    spicelib::SPKW19(
        save.HANDLE,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.EPOCHS.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(TIMESOUTOFORDER)", OK, ctx)?;

    spicelib::SWAPD_ARRAY(
        save.EPOCHS.subscript(2),
        save.EPOCHS.subscript(3),
        save.EPOCHS.as_slice_mut(),
    );

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKW19 error case: epochs for first mini-segment don\'t cover first interpolation interval.", ctx)?;

    save.NINTVL = 1;
    save.NPKTS[1] = 4;

    for I in 1..=save.NPKTS[1] {
        save.EPOCHS[I] = (I as f64);
    }

    save.FIRST = save.EPOCHS[1];
    save.LAST = save.EPOCHS[save.NPKTS[1]];

    save.IVLBDS[1] = save.FIRST;
    save.IVLBDS[2] = save.LAST;

    save.EPOCHS[1] = (save.EPOCHS[1] + 0.5);

    spicelib::SPKW19(
        save.HANDLE,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.EPOCHS.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BOUNDSDISAGREE)", OK, ctx)?;

    save.EPOCHS[1] = 1.0;
    save.EPOCHS[4] = (save.EPOCHS[4] - 0.5);

    spicelib::SPKW19(
        save.HANDLE,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.EPOCHS.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BOUNDSDISAGREE)", OK, ctx)?;

    for I in 1..=save.NPKTS[1] {
        save.EPOCHS[I] = (I as f64);
    }

    //
    // End of SPKW19 error cases.
    //

    //
    // Close the SPK file at the DAF level; SPKCLS won't close
    // a file without segments.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // SPKW19 Non-error cases:
    //

    //*****************************************************************
    //*
    //*    Trivial case: write an SPK containing a small subtype 1
    //*    segment.
    //*
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Write a 1-interval subtype 1 segment.", ctx)?;

    //
    // This file lacks both interval directories and mini-segment
    // epoch directories.
    //
    //
    // Initialize the inputs to SPKW19.
    //
    save.XBODY = 3;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");
    //
    // Create a segment identifier.
    //
    fstr::assign(&mut save.SEGID, b"SPK type 19 test segment");

    spicelib::SPKOPN(SP19T1, SP19T1, 0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NINTVL = 1;
    save.NPKTS[1] = NDSCRT;
    save.SUBTPS[1] = S19TP1;
    save.DEGRES[1] = 3;

    save.FIRST = save.DSCEPC[3];
    save.LAST = save.DSCEPC[(save.NPKTS[1] - 2)];

    save.IVLBDS[1] = save.DSCEPC[2];
    save.IVLBDS[2] = save.DSCEPC[(save.NPKTS[1] - 1)];

    save.SELLST = true;

    spicelib::SPKW19(
        save.HANDLE,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.DSCSTS.as_slice(),
        save.DSCEPC.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Read states from the file just created. Epochs match input epochs.",
        ctx,
    )?;

    spicelib::FURNSH(SP19T1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XBODY = 3;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");

    for I in 3..=(save.NPKTS[1] - 2) {
        save.ET = save.DSCEPC[I];

        spicelib::SPKEZ(
            save.XBODY,
            save.ET,
            &save.XREF,
            b"NONE",
            save.XCENTR,
            save.STATE.as_slice_mut(),
            &mut save.LT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if *OK {
            fstr::assign(&mut save.NAME, b"State #.");
            spicelib::REPMI(&save.NAME.to_vec(), b"#", I, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.NAME,
                save.STATE.as_slice(),
                b"~~",
                save.DSCSTS.subarray([1, I]),
                6,
                TIGHT,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Read states from the file just created. Epochs are at midpoints between adjacent input epochs.", ctx)?;

    save.XBODY = 3;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");

    for I in 3..=(save.NPKTS[1] - 3) {
        save.ET = ((save.DSCEPC[I] + save.DSCEPC[(I + 1)]) / 2 as f64);

        spicelib::SPKEZ(
            save.XBODY,
            save.ET,
            &save.XREF,
            b"NONE",
            save.XCENTR,
            save.STATE.as_slice_mut(),
            &mut save.LT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if *OK {
            fstr::assign(&mut save.NAME, b"State #.");
            spicelib::REPMI(&save.NAME.to_vec(), b"#", I, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            for J in 1..=6 {
                save.XSTATE[J] = ((save.DSCSTS[[J, I]] + save.DSCSTS[[J, (I + 1)]]) / 2 as f64);
            }

            testutil::CHCKAD(
                &save.NAME,
                save.STATE.as_slice(),
                b"~~",
                save.XSTATE.as_slice(),
                6,
                TIGHT,
                OK,
                ctx,
            )?;
        }
    }

    spicelib::UNLOAD(SP19T1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Remove the trivial SPK we created; we'll create a new
    // one with more complexity.
    //
    spicelib::DELFIL(SP19T1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*****************************************************************
    //*
    //*    Test subtype 2: write an SPK containing a small subtype 2
    //*    mini-segment.
    //*
    //*****************************************************************

    //
    // Below, it will be convenient to use cubic interpolation and a
    // window size of 2.

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create an SPK with subtype 2 mini-segments.", ctx)?;

    spicelib::SPKOPN(SP19T2, SP19T2, 0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set the interval count.
    //
    save.NINTVL = 10;

    //
    // Set the packet counts, subtypes, and degrees.
    //
    // Since we're concentrating on interval selection
    // logic, we'll use small packet counts.
    //
    for I in 1..=save.NINTVL {
        save.SUBTPS[I] = S19TP2;
        save.DEGRES[I] = 3;

        if spicelib::ODD(I) {
            save.NPKTS[I] = 3;
        } else {
            save.NPKTS[I] = 4;
        }
    }

    //
    // Generate epochs and packets.
    //
    save.EPCTO = 0;
    //
    // The end epoch of each interval must be the start epoch of
    // the next.
    //
    save.ENDEPC = 0.0;

    for I in 1..=save.NINTVL {
        //
        // Set the interval start time.
        //
        save.IVLBDS[I] = save.ENDEPC;

        //
        // Set the separation of epochs.
        //
        if spicelib::ODD(I) {
            save.ESCALE = 10.0;
        } else {
            save.ESCALE = 20.0;
        }

        for J in 1..=save.NPKTS[I] {
            save.EPCTO = (save.EPCTO + 1);

            save.EPOCHS[save.EPCTO] = (save.ENDEPC + (save.ESCALE * (J - 1) as f64));
        }

        //
        // Save the last epoch of this interval.
        //
        save.ENDEPC = save.EPOCHS[save.EPCTO];
    }

    //
    // Save the stop time of the last interval.
    //
    save.IVLBDS[(save.NINTVL + 1)] = save.ENDEPC;

    //
    // Set the descriptor bounds. We don't need to do anything
    // fancy here.
    //
    save.FIRST = save.IVLBDS[1];
    save.LAST = save.IVLBDS[(save.NINTVL + 1)];

    //
    // Now assign the packets. Since we're using two-point
    // interpolation, selecting the wrong packets to interpolate
    // should yield an obviously wrong answer.
    //
    // We also want to have large packet discontinuities at
    // interval boundaries, so the consequence of selecting
    // the wrong interval is obvious.
    //

    save.PKTTO = 0;

    for I in 1..=save.NINTVL {
        //
        // Assign the packets for the Ith interval.
        //
        for J in 1..=save.NPKTS[I] {
            //
            // Position components:
            //
            for K in 1..=3 {
                save.PKTTO = (save.PKTTO + 1);

                save.PACKTS[save.PKTTO] =
                    (((1000000000.0 * I as f64) + (100.0 * J as f64)) + K as f64);
            }

            //
            // Velocities associated with positions:
            //
            for K in 1..=3 {
                save.PKTTO = (save.PKTTO + 1);

                save.PACKTS[save.PKTTO] = 0.0;
            }
        }
    }

    //
    // For this segment, we'll select the last applicable interval
    // when the request time lies on an interval boundary.
    //
    save.SELLST = true;

    spicelib::SPKW19(
        save.HANDLE,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.EPOCHS.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Read states from type 2 file. SELLST = .TRUE.", ctx)?;

    spicelib::FURNSH(SP19T2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if *OK {
        //
        // Read states corresponding to epochs.
        //
        for I in 1..=save.NINTVL {
            if spicelib::ODD(I) {
                save.ESCALE = 10.0;
            } else {
                save.ESCALE = 20.0;
            }

            //
            // Note that the last epoch of each interval,
            // except the last, is a special case. The
            // state returned for that epoch is computed
            // from data in the following interval, if any.
            //
            if (I < save.NINTVL) {
                save.UB = (save.NPKTS[I] - 1);
            } else {
                save.UB = save.NPKTS[I];
            }

            for J in 1..=save.UB {
                //
                // --- Case: ------------------------------------------------------
                //
                save.ET = (save.IVLBDS[I] + (save.ESCALE * (J - 1) as f64));

                fstr::assign(&mut save.TITLE, b"State at interval #; packet index #");

                spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                testutil::TCASE(&save.TITLE, ctx)?;

                spicelib::SPKEZ(
                    save.XBODY,
                    save.ET,
                    &save.XREF,
                    b"NONE",
                    save.XCENTR,
                    save.STATE.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                for K in 1..=3 {
                    save.XSTATE[K] = (((1000000000.0 * I as f64) + (100.0 * J as f64)) + K as f64);

                    save.XSTATE[(K + 3)] = 0.0;
                    //
                    // Also compute the state at the midpoint
                    // between the Jth epoch and the next. Because
                    // the cubics are fit to endpoints with
                    // zero derivatives, the function value
                    // at the midpoint is just the average of
                    // the values at the endpoints.
                    //
                    // The midpoint velocity should be 3/2 * the difference
                    // between the Jth and (J+1)st positions, divided by
                    // the epoch difference. This can be derived from the
                    // simple case
                    //
                    //              3       2
                    //    p(x) = a x  +  a x  +  a x  +  a
                    //            3       2       1       0
                    //
                    //
                    //    p(0)      = alpha
                    //    p(1)      = beta
                    //    dp/dx (0) = 0
                    //    dp/dx (1) = 0
                    //
                    //
                    //
                    save.MIDSTA[K] = (save.XSTATE[K] + (100.0 * 0.5));
                    save.MIDSTA[(K + 3)] = ((1.5 / save.ESCALE) * 100.0);
                }

                //
                // Check position at the Jth epoch.
                //
                testutil::CHCKAD(
                    b"Position",
                    save.STATE.as_slice(),
                    b"~~/",
                    save.XSTATE.as_slice(),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;
                //
                // Check velocity at the Jth epoch.
                //
                testutil::CHCKAD(
                    b"Velocity",
                    save.STATE.subarray(4),
                    b"~~/",
                    save.XSTATE.subarray(4),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                //
                // --- Case: ------------------------------------------------------
                //

                //
                // Perform this test only if the packet index
                // is not the last of the interval.
                //
                if (J < save.NPKTS[I]) {
                    fstr::assign(&mut save.TITLE, b"State in interval #; epoch midway between that of packet index # and packet index #.");

                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", (J + 1), &mut save.TITLE, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::TCASE(&save.TITLE, ctx)?;

                    save.MIDET = (save.ET + (save.ESCALE * 0.5));

                    spicelib::SPKEZ(
                        save.XBODY,
                        save.MIDET,
                        &save.XREF,
                        b"NONE",
                        save.XCENTR,
                        save.STATE.as_slice_mut(),
                        &mut save.LT,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Check position midway between the Jth and J+1st
                    // epochs.
                    //
                    testutil::CHCKAD(
                        b"Mid pos",
                        save.STATE.as_slice(),
                        b"~~/",
                        save.MIDSTA.as_slice(),
                        3,
                        TIGHT,
                        OK,
                        ctx,
                    )?;
                    //
                    // Check velocity midway between the Jth and J+1st
                    // epochs.
                    //
                    testutil::CHCKAD(
                        b"Mid vel",
                        save.STATE.subarray(4),
                        b"~~/",
                        save.MIDSTA.subarray(4),
                        3,
                        TIGHT,
                        OK,
                        ctx,
                    )?;
                }
            }
        }
    }

    spicelib::UNLOAD(SP19T2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Delete the file we created.
    //
    spicelib::DELFIL(SP19T2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*****************************************************************
    //*
    //*    Test subtype 2: write an SPK containing a small subtype 2
    //*    mini-segment. This time use different velocities for each
    //*    component.
    //*
    //*****************************************************************

    //
    //     Below, we'll use linear functions for each position component.
    //     The rates for each component will be distinct.
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create an SPK with subtype 2 mini-segments. Use linear position functions with different rates for each component.", ctx)?;

    spicelib::SPKOPN(SP19T2, SP19T2, 0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set the interval count.
    //
    save.NINTVL = 10;

    //
    // Set the packet counts, subtypes, and degrees.
    //
    // Since we're concentrating on interval selection
    // logic, we'll use small packet counts.
    //
    for I in 1..=save.NINTVL {
        save.SUBTPS[I] = S19TP2;
        save.DEGRES[I] = 3;

        if spicelib::ODD(I) {
            save.NPKTS[I] = 3;
        } else {
            save.NPKTS[I] = 4;
        }
    }

    //
    // Generate epochs and packets.
    //
    save.EPCTO = 0;
    //
    // The end epoch of each interval must be the start epoch of
    // the next.
    //
    save.ENDEPC = 0.0;

    for I in 1..=save.NINTVL {
        //
        // Set the interval start time.
        //
        save.IVLBDS[I] = save.ENDEPC;

        //
        // Set the separation of epochs.
        //
        if spicelib::ODD(I) {
            save.ESCALE = 10.0;
        } else {
            save.ESCALE = 20.0;
        }

        for J in 1..=save.NPKTS[I] {
            save.EPCTO = (save.EPCTO + 1);

            save.EPOCHS[save.EPCTO] = (save.ENDEPC + (save.ESCALE * (J - 1) as f64));
        }

        //
        // Save the last epoch of this interval.
        //
        save.ENDEPC = save.EPOCHS[save.EPCTO];
    }

    //
    // Save the stop time of the last interval.
    //
    save.IVLBDS[(save.NINTVL + 1)] = save.ENDEPC;

    //
    // Set the descriptor bounds. We don't need to do anything
    // fancy here.
    //
    save.FIRST = save.IVLBDS[1];
    save.LAST = save.IVLBDS[(save.NINTVL + 1)];

    //
    // Now assign the packets. Since we're using two-point
    // interpolation, selecting the wrong packets to interpolate
    // should yield an obviously wrong answer.
    //
    // We also want to have large packet discontinuities at
    // interval boundaries, so the consequence of selecting
    // the wrong interval is obvious.
    //

    save.PKTTO = 0;

    for I in 1..=save.NINTVL {
        if spicelib::ODD(I) {
            save.ESCALE = 10.0;
        } else {
            save.ESCALE = 20.0;
        }

        //
        // Assign the packets for the Ith interval.
        //
        for J in 1..=save.NPKTS[I] {
            //
            // Position components:
            //
            for K in 1..=3 {
                save.PKTTO = (save.PKTTO + 1);

                save.PACKTS[save.PKTTO] =
                    (((1000000000.0 * I as f64) + ((100.0 * J as f64) * K as f64)) + K as f64);
            }

            //
            // Velocities associated with positions:
            //
            for K in 1..=3 {
                save.PKTTO = (save.PKTTO + 1);

                save.PACKTS[save.PKTTO] = ((100.0 * K as f64) / save.ESCALE);
            }
        }
    }

    //
    // For this segment, we'll select the last applicable interval
    // when the request time lies on an interval boundary.
    //
    save.SELLST = true;

    spicelib::SPKW19(
        save.HANDLE,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.EPOCHS.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Read states from constant rate type 2 file. SELLST = .TRUE.",
        ctx,
    )?;

    spicelib::FURNSH(SP19T2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if *OK {
        //
        // Read states corresponding to epochs.
        //
        for I in 1..=save.NINTVL {
            if spicelib::ODD(I) {
                save.ESCALE = 10.0;
            } else {
                save.ESCALE = 20.0;
            }

            //
            // Note that the last epoch of each interval,
            // except the last, is a special case. The
            // state returned for that epoch is computed
            // from data in the following interval, if any.
            //
            if (I < save.NINTVL) {
                save.UB = (save.NPKTS[I] - 1);
            } else {
                save.UB = save.NPKTS[I];
            }

            for J in 1..=save.UB {
                //
                // --- Case: ------------------------------------------------------
                //
                save.ET = (save.IVLBDS[I] + (save.ESCALE * (J - 1) as f64));

                fstr::assign(
                    &mut save.TITLE,
                    b"State at interval #; packet index #; subtype 2; linear case",
                );

                spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                testutil::TCASE(&save.TITLE, ctx)?;

                spicelib::SPKEZ(
                    save.XBODY,
                    save.ET,
                    &save.XREF,
                    b"NONE",
                    save.XCENTR,
                    save.STATE.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                for K in 1..=3 {
                    save.XSTATE[K] =
                        (((1000000000.0 * I as f64) + ((100.0 * J as f64) * K as f64)) + K as f64);

                    save.XSTATE[(K + 3)] = ((100.0 * K as f64) / save.ESCALE);

                    //
                    // Also compute the state at the midpoint
                    // between the Jth epoch and the next.
                    //
                    save.MIDSTA[K] = (save.XSTATE[K] + ((100.0 * K as f64) * 0.5));
                    save.MIDSTA[(K + 3)] = ((100.0 * K as f64) / save.ESCALE);
                }

                //
                // Check position at the Jth epoch.
                //
                testutil::CHCKAD(
                    b"Position",
                    save.STATE.as_slice(),
                    b"~~/",
                    save.XSTATE.as_slice(),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;
                //
                // Check velocity at the Jth epoch.
                //
                testutil::CHCKAD(
                    b"Velocity",
                    save.STATE.subarray(4),
                    b"~~/",
                    save.XSTATE.subarray(4),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                //
                // --- Case: ------------------------------------------------------
                //

                //
                // Perform this test only if the packet index
                // is not the last of the interval.
                //
                if (J < save.NPKTS[I]) {
                    fstr::assign(&mut save.TITLE, b"State in interval #; epoch midway between that of packet index # and packet index #. Subtype 2; linear case.");

                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", (J + 1), &mut save.TITLE, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::TCASE(&save.TITLE, ctx)?;

                    save.MIDET = (save.ET + (save.ESCALE * 0.5));

                    spicelib::SPKEZ(
                        save.XBODY,
                        save.MIDET,
                        &save.XREF,
                        b"NONE",
                        save.XCENTR,
                        save.STATE.as_slice_mut(),
                        &mut save.LT,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Check position midway between the Jth and J+1st
                    // epochs.
                    //
                    testutil::CHCKAD(
                        b"Mid pos",
                        save.STATE.as_slice(),
                        b"~~/",
                        save.MIDSTA.as_slice(),
                        3,
                        TIGHT,
                        OK,
                        ctx,
                    )?;
                    //
                    // Check velocity midway between the Jth and J+1st
                    // epochs.
                    //
                    testutil::CHCKAD(
                        b"Mid vel",
                        save.STATE.subarray(4),
                        b"~~/",
                        save.MIDSTA.subarray(4),
                        3,
                        TIGHT,
                        OK,
                        ctx,
                    )?;
                }
            }
        }
    }

    spicelib::UNLOAD(SP19T2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Delete the file we created.
    //
    spicelib::DELFIL(SP19T2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*****************************************************************
    //*
    //*    SPKR19 window selection logic cases:
    //*
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Create a subtype 1 segment suitable for testing interpolation window packet selection.",
        ctx,
    )?;
    //
    // The position and velocity data in this segment are defined by
    // exponential functions of time.
    //
    spicelib::SPKOPN(SP19T1, SP19T1, 0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Initialize the inputs to SPKW19.
    //
    save.XBODY = 3;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");
    //
    // Create a segment identifier.
    //
    fstr::assign(&mut save.SEGID, b"SPK type 19 test segment");

    save.NINTVL = 1;
    save.NPKTS[1] = (2 * (MAXDEG + 1));
    save.SUBTPS[1] = S19TP1;
    save.DEGRES[1] = MAXDEG;

    //
    // Assign the packet epochs.
    //
    for I in 1..=save.NPKTS[1] {
        save.EPOCHS[I] = (I as f64);
    }

    save.FIRST = save.EPOCHS[1];
    save.LAST = save.EPOCHS[save.NPKTS[1]];

    save.IVLBDS[1] = save.FIRST;
    save.IVLBDS[2] = save.LAST;

    //
    // Assign the packet values.
    //
    save.PKTTO = 0;

    for I in 1..=save.NPKTS[1] {
        for J in 1..=6 {
            save.PKTTO = (save.PKTTO + 1);

            save.PACKTS[save.PKTTO] = (f64::powi(10.0, J) * f64::exp(((I as f64) / 100.0)));
        }
    }

    save.SELLST = true;

    spicelib::SPKW19(
        save.HANDLE,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.EPOCHS.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Look up states at midpoints between packet epochs of segment we just created.",
        ctx,
    )?;

    //
    // Touch array WORK to make f2c happy. WORK is simply a workspace
    // array.
    //
    spicelib::CLEARD((MAXDEG + 1), save.WORK.as_slice_mut());

    spicelib::FURNSH(SP19T1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.WINSIZ = (MAXDEG + 1);

    for I in 1..=(save.NPKTS[1] - 1) {
        save.ET = ((save.EPOCHS[I] + save.EPOCHS[(I + 1)]) / 2 as f64);

        spicelib::SPKEZ(
            save.XBODY,
            save.ET,
            &save.XREF,
            b"NONE",
            save.XCENTR,
            save.STATE.as_slice_mut(),
            &mut save.LT,
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Generate expected state components at ET.
        //
        //
        // Let B and E be the begin and end indices of the
        // packets comprising the interpolation window.
        //
        save.B = intrinsics::MAX0(&[1, (I - ((save.WINSIZ / 2) - 1))]);
        save.E = intrinsics::MIN0(&[save.NPKTS[1], ((I + ((save.WINSIZ / 2) - 1)) + 1)]);

        for J in 1..=6 {
            for K in save.B..=save.E {
                save.XVALS[((K - save.B) + 1)] = save.EPOCHS[K];

                save.YVALS[((K - save.B) + 1)] =
                    (f64::powi(10.0, J) * f64::exp(((K as f64) / 100.0)));
            }

            save.XSTATE[J] = spicelib::LGRINT(
                ((save.E - save.B) + 1),
                save.XVALS.as_slice(),
                save.YVALS.as_slice(),
                save.WORK.as_slice_mut(),
                save.ET,
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        //
        // Test the state at ET.
        //
        for J in 1..=6 {
            //
            // --- Case: ------------------------------------------------------
            //
            fstr::assign(&mut save.TITLE, b"State # component #");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::TCASE(&save.TITLE, ctx)?;

            testutil::CHCKSD(
                b"State",
                save.STATE[J],
                b"~/",
                save.XSTATE[J],
                TIGHT,
                OK,
                ctx,
            )?;
        }
    }

    //
    // Delete the file we created so we can start over.
    //
    spicelib::UNLOAD(SP19T1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SP19T1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*****************************************************************
    //*
    //*    SPKR19 interval re-use logic cases:
    //*
    //*****************************************************************

    //*****************************************************************
    //*
    //*    SPKR19 subtype logic cases:
    //*
    //*****************************************************************

    //*****************************************************************
    //*
    //*    SPKR19 interval selection logic cases:
    //*
    //*****************************************************************

    //
    // Below, we'll use type 0 segments because it will
    // be convenient to use cubic interpolation and
    // a window size of 2.
    //
    // The velocity/acceleration data will not be compatible with the
    // position/velocity data.
    //

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // We need a segment containing enough interval directories
    // so that multiple buffers full of directory entries must
    // be read. This implies we need more than DIRSIZ**2 intervals
    // in the segment.
    //

    testutil::TCASE(b"Create an SPK with interval directories.", ctx)?;

    spicelib::SPKOPN(SP19T0, SP19T0, 0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set the interval count.
    //
    save.NINTVL = MAXIVL;

    //
    // Set the packet counts, subtypes, and degrees.
    //
    // Since we're concentrating on interval selection
    // logic, we'll use small packet counts.
    //
    for I in 1..=save.NINTVL {
        save.SUBTPS[I] = S19TP0;
        save.DEGRES[I] = 3;

        if spicelib::ODD(I) {
            save.NPKTS[I] = 3;
        } else {
            save.NPKTS[I] = 4;
        }
    }

    //
    // Generate epochs and packets.
    //
    save.EPCTO = 0;
    //
    // The end epoch of each interval must be the start epoch of
    // the next.
    //
    save.ENDEPC = 0.0;

    for I in 1..=save.NINTVL {
        //
        // Set the interval start time.
        //
        save.IVLBDS[I] = save.ENDEPC;

        //
        // Set the separation of epochs.
        //
        if spicelib::ODD(I) {
            save.ESCALE = 10.0;
        } else {
            save.ESCALE = 20.0;
        }

        for J in 1..=save.NPKTS[I] {
            save.EPCTO = (save.EPCTO + 1);

            save.EPOCHS[save.EPCTO] = (save.ENDEPC + (save.ESCALE * (J - 1) as f64));
        }

        //
        // Save the last epoch of this interval.
        //
        save.ENDEPC = save.EPOCHS[save.EPCTO];
    }

    //
    // Save the stop time of the last interval.
    //
    save.IVLBDS[(save.NINTVL + 1)] = save.ENDEPC;

    //
    // Set the descriptor bounds. We don't need to do anything
    // fancy here.
    //
    save.FIRST = save.IVLBDS[1];
    save.LAST = save.IVLBDS[(save.NINTVL + 1)];

    //
    // Now assign the packets. Since we're using two-point
    // interpolation, selecting the wrong packets to interpolate
    // should yield an obviously wrong answer.
    //
    // We also want to have large packet discontinuities at
    // interval boundaries, so the consequence of selecting
    // the wrong interval is obvious.
    //

    save.PKTTO = 0;

    for I in 1..=save.NINTVL {
        //
        // Assign the packets for the Ith interval.
        //
        for J in 1..=save.NPKTS[I] {
            //
            // Position components:
            //
            for K in 1..=3 {
                save.PKTTO = (save.PKTTO + 1);

                save.PACKTS[save.PKTTO] =
                    (((1000000000.0 * I as f64) + (100.0 * J as f64)) + K as f64);
            }

            //
            // Velocities associated with positions:
            //
            for K in 1..=3 {
                save.PKTTO = (save.PKTTO + 1);

                save.PACKTS[save.PKTTO] = 0.0;
            }

            //
            // Velocity/acceleration packets:
            //
            for K in 1..=6 {
                save.PKTTO = (save.PKTTO + 1);

                save.PACKTS[save.PKTTO] = (save.PACKTS[(save.PKTTO - 6)] * 10.0);
            }
        }
    }

    //
    // For this segment, we'll select the last applicable interval
    // when the request time lies on an interval boundary.
    //
    save.SELLST = true;

    spicelib::SPKW19(
        save.HANDLE,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.EPOCHS.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Read states from large type 0 file. SELLST = .TRUE.", ctx)?;

    spicelib::FURNSH(SP19T0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if *OK {
        //
        // Read states corresponding to epochs.
        //
        for I in 1..=save.NINTVL {
            if spicelib::ODD(I) {
                save.ESCALE = 10.0;
            } else {
                save.ESCALE = 20.0;
            }

            //
            // Note that the last epoch of each interval,
            // except the last, is a special case. The
            // state returned for that epoch is computed
            // from data in the following interval, if any.
            //
            if (I < save.NINTVL) {
                save.UB = (save.NPKTS[I] - 1);
            } else {
                save.UB = save.NPKTS[I];
            }

            for J in 1..=save.UB {
                //
                // --- Case: ------------------------------------------------------
                //
                save.ET = (save.IVLBDS[I] + (save.ESCALE * (J - 1) as f64));

                fstr::assign(&mut save.TITLE, b"State at interval #; packet index #");

                spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                testutil::TCASE(&save.TITLE, ctx)?;

                spicelib::SPKEZ(
                    save.XBODY,
                    save.ET,
                    &save.XREF,
                    b"NONE",
                    save.XCENTR,
                    save.STATE.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                for K in 1..=3 {
                    save.XSTATE[K] = (((1000000000.0 * I as f64) + (100.0 * J as f64)) + K as f64);

                    save.XSTATE[(K + 3)] = (10.0 * save.XSTATE[K]);

                    //
                    // Also compute the state at the midpoint
                    // between the Jth epoch and the next. Because
                    // the cubics are fit to endpoints with
                    // zero derivatives, the function value
                    // at the midpoint is just the average of
                    // the values at the endpoints.
                    //
                    save.MIDSTA[K] = (save.XSTATE[K] + (100.0 * 0.5));
                    save.MIDSTA[(K + 3)] = (10.0 * save.MIDSTA[K]);
                }

                //
                // Check position at the Jth epoch.
                //
                testutil::CHCKAD(
                    b"Position",
                    save.STATE.as_slice(),
                    b"~~/",
                    save.XSTATE.as_slice(),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;
                //
                // Check velocity at the Jth epoch.
                //
                testutil::CHCKAD(
                    b"Velocity",
                    save.STATE.subarray(4),
                    b"~~/",
                    save.XSTATE.subarray(4),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                //
                // --- Case: ------------------------------------------------------
                //

                //
                // Perform this test only if the packet index
                // is not the last of the interval.
                //
                if (J < save.NPKTS[I]) {
                    fstr::assign(&mut save.TITLE, b"State in interval #; epoch midway between that of packet index # and packet index #.");

                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", (J + 1), &mut save.TITLE, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::TCASE(&save.TITLE, ctx)?;

                    save.MIDET = (save.ET + (save.ESCALE * 0.5));

                    spicelib::SPKEZ(
                        save.XBODY,
                        save.MIDET,
                        &save.XREF,
                        b"NONE",
                        save.XCENTR,
                        save.STATE.as_slice_mut(),
                        &mut save.LT,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Check position midway between the Jth and J+1st
                    // epochs.
                    //
                    testutil::CHCKAD(
                        b"Mid pos",
                        save.STATE.as_slice(),
                        b"~~/",
                        save.MIDSTA.as_slice(),
                        3,
                        TIGHT,
                        OK,
                        ctx,
                    )?;
                    //
                    // Check velocity midway between the Jth and J+1st
                    // epochs.
                    //
                    testutil::CHCKAD(
                        b"Mid vel",
                        save.STATE.subarray(4),
                        b"~~/",
                        save.MIDSTA.subarray(4),
                        3,
                        TIGHT,
                        OK,
                        ctx,
                    )?;
                }
            }
        }
    }

    spicelib::UNLOAD(SP19T0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Delete the file we created so we can start over.
    //
    spicelib::DELFIL(SP19T0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    //
    // Repeat the previous cases using a segment for which the
    // first applicable interval is picked.
    //
    spicelib::SPKOPN(SP19T0, SP19T0, 0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SELLST = false;

    spicelib::SPKW19(
        save.HANDLE,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.EPOCHS.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Read states from large type 0 file. SELLST = .FALSE.", ctx)?;

    spicelib::FURNSH(SP19T0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if *OK {
        //
        // Read states corresponding to epochs.
        //
        for I in 1..=save.NINTVL {
            if spicelib::ODD(I) {
                save.ESCALE = 10.0;
            } else {
                save.ESCALE = 20.0;
            }

            //
            // Note that the first epoch of each interval,
            // except the first, is a special case. The
            // state returned for that epoch is computed
            // from data in the preceding interval, if any.
            //
            if (I > 1) {
                save.LB = 2;
            } else {
                save.LB = 1;
            }

            for J in save.LB..=save.NPKTS[I] {
                //
                // --- Case: ------------------------------------------------------
                //
                save.ET = (save.IVLBDS[I] + (save.ESCALE * (J - 1) as f64));

                fstr::assign(&mut save.TITLE, b"State at interval #; packet index #");

                spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                testutil::TCASE(&save.TITLE, ctx)?;

                spicelib::SPKEZ(
                    save.XBODY,
                    save.ET,
                    &save.XREF,
                    b"NONE",
                    save.XCENTR,
                    save.STATE.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                for K in 1..=3 {
                    save.XSTATE[K] = (((1000000000.0 * I as f64) + (100.0 * J as f64)) + K as f64);

                    save.XSTATE[(K + 3)] = (10.0 * save.XSTATE[K]);

                    //
                    // Also compute the state at the midpoint
                    // between the Jth epoch and the J-1st. Because
                    // the cubics are fit to endpoints with
                    // zero derivatives, the function value
                    // at the midpoint is just the average of
                    // the values at the endpoints.
                    //
                    save.MIDSTA[K] = (save.XSTATE[K] - (100.0 * 0.5));
                    save.MIDSTA[(K + 3)] = (10.0 * save.MIDSTA[K]);
                }

                //
                // Check position at the Jth epoch.
                //
                testutil::CHCKAD(
                    b"Position",
                    save.STATE.as_slice(),
                    b"~~/",
                    save.XSTATE.as_slice(),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;
                //
                // Check velocity at the Jth epoch.
                //
                testutil::CHCKAD(
                    b"Velocity",
                    save.STATE.subarray(4),
                    b"~~/",
                    save.XSTATE.subarray(4),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                //
                // --- Case: ------------------------------------------------------
                //

                //
                // Perform the following test only if a preceding
                // index exists.
                //
                if (J > 1) {
                    fstr::assign(&mut save.TITLE, b"State in interval #; epoch midway between that of packet index # and packet index #.");

                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", (J - 1), &mut save.TITLE, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::TCASE(&save.TITLE, ctx)?;

                    save.MIDET = (save.ET - (save.ESCALE * 0.5));

                    spicelib::SPKEZ(
                        save.XBODY,
                        save.MIDET,
                        &save.XREF,
                        b"NONE",
                        save.XCENTR,
                        save.STATE.as_slice_mut(),
                        &mut save.LT,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Check position midway between the Jth and J-1st
                    // epochs.
                    //
                    testutil::CHCKAD(
                        b"Mid pos",
                        save.STATE.as_slice(),
                        b"~~/",
                        save.MIDSTA.as_slice(),
                        3,
                        TIGHT,
                        OK,
                        ctx,
                    )?;
                    //
                    // Check velocity midway between the Jth and J-1st
                    // epochs.
                    //
                    testutil::CHCKAD(
                        b"Mid vel",
                        save.STATE.subarray(4),
                        b"~~/",
                        save.MIDSTA.subarray(4),
                        3,
                        TIGHT,
                        OK,
                        ctx,
                    )?;
                }
            }
        }
    }

    spicelib::UNLOAD(SP19T0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Delete the SPK file.
    //
    spicelib::DELFIL(SP19T0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*****************************************************************
    //*
    //*    SPKR19 packet selection logic cases:
    //*
    //*****************************************************************

    //
    // Below, we'll use type 0 segments because it will
    // be convenient to use cubic interpolation and
    // a window size of 2.
    //
    // The velocity/acceleration data will not be compatible with the
    // position/velocity data.
    //

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // We must now test the packet selection logic. The first
    // aspect to test is use of packet directories. We must have
    // at least one interval that contains enough directories so
    // that they can't all be buffered. It's all right to have
    // a small number of intervals for this test.
    //

    testutil::TCASE(b"Create an SPK with interval directories.", ctx)?;

    spicelib::SPKOPN(SP19T0, SP19T0, 0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set the interval count.
    //
    save.NINTVL = 2;

    //
    // Set the packet counts, subtypes, and degrees.
    //
    for I in 1..=save.NINTVL {
        save.SUBTPS[I] = S19TP0;
        save.DEGRES[I] = 3;

        if spicelib::ODD(I) {
            save.NPKTS[I] = (MAXIVL - 100);
        } else {
            save.NPKTS[I] = MAXIVL;
        }
    }

    //
    // Generate epochs and packets.
    //
    save.EPCTO = 0;
    //
    // The end epoch of each interval must be the start epoch of
    // the next.
    //
    save.ENDEPC = 0.0;

    for I in 1..=save.NINTVL {
        //
        // Set the interval start time.
        //
        save.IVLBDS[I] = save.ENDEPC;

        //
        // Set the separation of epochs.
        //
        if spicelib::ODD(I) {
            save.ESCALE = 1.0;
        } else {
            save.ESCALE = 2.0;
        }

        for J in 1..=save.NPKTS[I] {
            save.EPCTO = (save.EPCTO + 1);

            save.EPOCHS[save.EPCTO] = (save.ENDEPC + (save.ESCALE * (J - 1) as f64));
        }

        //
        // Save the last epoch of this interval.
        //
        save.ENDEPC = save.EPOCHS[save.EPCTO];
    }

    //
    // Save the stop time of the last interval.
    //
    save.IVLBDS[(save.NINTVL + 1)] = save.ENDEPC;

    //
    // Set the descriptor bounds. We don't need to do anything
    // fancy here.
    //
    save.FIRST = save.IVLBDS[1];
    save.LAST = save.IVLBDS[(save.NINTVL + 1)];

    //
    // Now assign the packets. Since we're using two-point
    // interpolation, selecting the wrong packets to interpolate
    // should yield an obviously wrong answer.
    //
    // We also want to have large packet discontinuities at
    // interval boundaries, so the consequence of selecting
    // the wrong interval is obvious.
    //

    save.PKTTO = 0;

    for I in 1..=save.NINTVL {
        //
        // Assign the packets for the Ith interval.
        //
        for J in 1..=save.NPKTS[I] {
            //
            // Position components:
            //
            for K in 1..=3 {
                save.PKTTO = (save.PKTTO + 1);

                save.PACKTS[save.PKTTO] =
                    (((1000000000.0 * I as f64) + (100.0 * J as f64)) + K as f64);
            }

            //
            // Velocities associated with positions:
            //
            for K in 1..=3 {
                save.PKTTO = (save.PKTTO + 1);

                save.PACKTS[save.PKTTO] = 0.0;
            }

            //
            // Velocity/acceleration packets:
            //
            for K in 1..=6 {
                save.PKTTO = (save.PKTTO + 1);

                save.PACKTS[save.PKTTO] = (save.PACKTS[(save.PKTTO - 6)] * 10.0);
            }
        }
    }

    //
    // For this segment, we'll select the last applicable interval
    // when the request time lies on an interval boundary.
    //
    save.SELLST = true;

    spicelib::SPKW19(
        save.HANDLE,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.EPOCHS.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Read states from large type 0 file. Packet selection test. SELLST = .TRUE.",
        ctx,
    )?;

    spicelib::FURNSH(SP19T0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if *OK {
        //
        // Read states corresponding to epochs.
        //
        for I in 1..=save.NINTVL {
            if spicelib::ODD(I) {
                save.ESCALE = 1.0;
            } else {
                save.ESCALE = 2.0;
            }

            //
            // Note that the last epoch of each interval,
            // except the last, is a special case. The
            // state returned for that epoch is computed
            // from data in the following interval, if any.
            //
            if (I < save.NINTVL) {
                save.UB = (save.NPKTS[I] - 1);
            } else {
                save.UB = save.NPKTS[I];
            }

            for J in 1..=save.UB {
                //
                // --- Case: ------------------------------------------------------
                //
                save.ET = (save.IVLBDS[I] + (save.ESCALE * (J - 1) as f64));

                fstr::assign(&mut save.TITLE, b"State at interval #; packet index #");

                spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                testutil::TCASE(&save.TITLE, ctx)?;

                spicelib::SPKEZ(
                    save.XBODY,
                    save.ET,
                    &save.XREF,
                    b"NONE",
                    save.XCENTR,
                    save.STATE.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                for K in 1..=3 {
                    save.XSTATE[K] = (((1000000000.0 * I as f64) + (100.0 * J as f64)) + K as f64);

                    save.XSTATE[(K + 3)] = (10.0 * save.XSTATE[K]);

                    //
                    // Also compute the state at the midpoint
                    // between the Jth epoch and the next. Because
                    // the cubics are fit to endpoints with
                    // zero derivatives, the function value
                    // at the midpoint is just the average of
                    // the values at the endpoints.
                    //
                    save.MIDSTA[K] = (save.XSTATE[K] + (100.0 * 0.5));
                    save.MIDSTA[(K + 3)] = (10.0 * save.MIDSTA[K]);
                }

                //
                // Check position at the Jth epoch.
                //
                testutil::CHCKAD(
                    b"Position",
                    save.STATE.as_slice(),
                    b"~~/",
                    save.XSTATE.as_slice(),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;
                //
                // Check velocity at the Jth epoch.
                //
                testutil::CHCKAD(
                    b"Velocity",
                    save.STATE.subarray(4),
                    b"~~/",
                    save.XSTATE.subarray(4),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                //
                // --- Case: ------------------------------------------------------
                //

                //
                // Perform this test only if the packet index
                // is not the last of the interval.
                //
                if (J < save.NPKTS[I]) {
                    fstr::assign(&mut save.TITLE, b"State in interval #; epoch midway between that of packet index # and packet index #.");

                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", (J + 1), &mut save.TITLE, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::TCASE(&save.TITLE, ctx)?;

                    save.MIDET = (save.ET + (save.ESCALE * 0.5));

                    spicelib::SPKEZ(
                        save.XBODY,
                        save.MIDET,
                        &save.XREF,
                        b"NONE",
                        save.XCENTR,
                        save.STATE.as_slice_mut(),
                        &mut save.LT,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Check position midway between the Jth and J+1st
                    // epochs.
                    //
                    testutil::CHCKAD(
                        b"Mid pos",
                        save.STATE.as_slice(),
                        b"~~/",
                        save.MIDSTA.as_slice(),
                        3,
                        TIGHT,
                        OK,
                        ctx,
                    )?;
                    //
                    // Check velocity midway between the Jth and J+1st
                    // epochs.
                    //
                    testutil::CHCKAD(
                        b"Mid vel",
                        save.STATE.subarray(4),
                        b"~~/",
                        save.MIDSTA.subarray(4),
                        3,
                        TIGHT,
                        OK,
                        ctx,
                    )?;
                }
            }
        }
    }

    spicelib::UNLOAD(SP19T0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Delete the file we created so we can start over.
    //
    spicelib::DELFIL(SP19T0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up:  delete SPK files.", ctx)?;

    if spicelib::EXISTS(SPK19E, ctx)? {
        spicelib::DELFIL(SPK19E, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    if spicelib::EXISTS(SP19T0, ctx)? {
        spicelib::DELFIL(SP19T0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    if spicelib::EXISTS(SP19T1, ctx)? {
        spicelib::DELFIL(SP19T1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
