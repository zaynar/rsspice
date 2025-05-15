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
const SP19MX: &[u8] = b"sp19mixed.bsp";
const SP19T1: &[u8] = b"sp19t1.bsp";
const SP19T2: &[u8] = b"sp19t2.bsp";
const SUB19: &[u8] = b"sp19subset.bsp";
const TMPSUB: &[u8] = b"spk19tempsub.bsp";
const TIGHT: f64 = 0.00000000000001;
const MAXIVL: i32 = 21000;
const MAXEPC: i32 = (MAXIVL * 100);
const DSCSIZ: i32 = 5;
const LNSIZE: i32 = 80;
const NDSCRT: i32 = 9;
const SIDLEN: i32 = 60;
const FRNMLN: i32 = 32;

struct SaveVars {
    NAME: Vec<u8>,
    SEGID: Vec<u8>,
    XREF: Vec<u8>,
    BEGIN: f64,
    DESCR: StackArray<f64, 5>,
    DSCEPC: StackArray<f64, 9>,
    DSCSTS: StackArray2D<f64, 54>,
    EPOCHS: ActualArray<f64>,
    END: f64,
    ENDEPC: f64,
    ESCALE: f64,
    ET: f64,
    FIRST: f64,
    IVLBDS: ActualArray<f64>,
    LAST: f64,
    LT: f64,
    PACKTS: ActualArray<f64>,
    STATE: StackArray<f64, 6>,
    TMPFST: f64,
    TMPIVB: ActualArray<f64>,
    TMPLST: f64,
    XLT: f64,
    XSTATE: StackArray<f64, 6>,
    XSTBUF: StackArray2D<f64, 108>,
    CPAD: i32,
    DEGRES: ActualArray<i32>,
    EPCTO: i32,
    HANDLE: i32,
    IVBEG: i32,
    J: i32,
    LTRUNC: i32,
    NINTVL: i32,
    NPAD: i32,
    NPKTS: ActualArray<i32>,
    PKTBEG: i32,
    PKTOFF: i32,
    PKTSIZ: i32,
    PKTSZS: ActualArray<i32>,
    PKTSUM: ActualArray<i32>,
    PKTTO: i32,
    RECTOT: i32,
    RTRUNC: i32,
    SUBHAN: i32,
    SUBTPS: ActualArray<i32>,
    TMPNIV: i32,
    TMPNPK: ActualArray<i32>,
    TMPTOT: i32,
    TO: i32,
    XBODY: i32,
    XCENTR: i32,
    FOUND: bool,
    SELLST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut NAME = vec![b' '; LNSIZE as usize];
        let mut SEGID = vec![b' '; SIDLEN as usize];
        let mut XREF = vec![b' '; FRNMLN as usize];
        let mut BEGIN: f64 = 0.0;
        let mut DESCR = StackArray::<f64, 5>::new(1..=DSCSIZ);
        let mut DSCEPC = StackArray::<f64, 9>::new(1..=NDSCRT);
        let mut DSCSTS = StackArray2D::<f64, 54>::new(1..=6, 1..=NDSCRT);
        let mut EPOCHS = ActualArray::<f64>::new(1..=MAXEPC);
        let mut END: f64 = 0.0;
        let mut ENDEPC: f64 = 0.0;
        let mut ESCALE: f64 = 0.0;
        let mut ET: f64 = 0.0;
        let mut FIRST: f64 = 0.0;
        let mut IVLBDS = ActualArray::<f64>::new(1..=(MAXIVL + 1));
        let mut LAST: f64 = 0.0;
        let mut LT: f64 = 0.0;
        let mut PACKTS = ActualArray::<f64>::new(1..=(MAXEPC * S19PS0));
        let mut STATE = StackArray::<f64, 6>::new(1..=6);
        let mut TMPFST: f64 = 0.0;
        let mut TMPIVB = ActualArray::<f64>::new(1..=(MAXIVL + 1));
        let mut TMPLST: f64 = 0.0;
        let mut XLT: f64 = 0.0;
        let mut XSTATE = StackArray::<f64, 6>::new(1..=6);
        let mut XSTBUF = StackArray2D::<f64, 108>::new(1..=6, 1..=(2 * NDSCRT));
        let mut CPAD: i32 = 0;
        let mut DEGRES = ActualArray::<i32>::new(1..=MAXIVL);
        let mut EPCTO: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut IVBEG: i32 = 0;
        let mut J: i32 = 0;
        let mut LTRUNC: i32 = 0;
        let mut NINTVL: i32 = 0;
        let mut NPAD: i32 = 0;
        let mut NPKTS = ActualArray::<i32>::new(1..=MAXIVL);
        let mut PKTBEG: i32 = 0;
        let mut PKTOFF: i32 = 0;
        let mut PKTSIZ: i32 = 0;
        let mut PKTSZS = ActualArray::<i32>::new(1..=MAXIVL);
        let mut PKTSUM = ActualArray::<i32>::new(0..=MAXEPC);
        let mut PKTTO: i32 = 0;
        let mut RECTOT: i32 = 0;
        let mut RTRUNC: i32 = 0;
        let mut SUBHAN: i32 = 0;
        let mut SUBTPS = ActualArray::<i32>::new(1..=MAXIVL);
        let mut TMPNIV: i32 = 0;
        let mut TMPNPK = ActualArray::<i32>::new(1..=MAXIVL);
        let mut TMPTOT: i32 = 0;
        let mut TO: i32 = 0;
        let mut XBODY: i32 = 0;
        let mut XCENTR: i32 = 0;
        let mut FOUND: bool = false;
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
            XREF,
            BEGIN,
            DESCR,
            DSCEPC,
            DSCSTS,
            EPOCHS,
            END,
            ENDEPC,
            ESCALE,
            ET,
            FIRST,
            IVLBDS,
            LAST,
            LT,
            PACKTS,
            STATE,
            TMPFST,
            TMPIVB,
            TMPLST,
            XLT,
            XSTATE,
            XSTBUF,
            CPAD,
            DEGRES,
            EPCTO,
            HANDLE,
            IVBEG,
            J,
            LTRUNC,
            NINTVL,
            NPAD,
            NPKTS,
            PKTBEG,
            PKTOFF,
            PKTSIZ,
            PKTSZS,
            PKTSUM,
            PKTTO,
            RECTOT,
            RTRUNC,
            SUBHAN,
            SUBTPS,
            TMPNIV,
            TMPNPK,
            TMPTOT,
            TO,
            XBODY,
            XCENTR,
            FOUND,
            SELLST,
        }
    }
}

//$Procedure F_SPKS19 ( SPK data type 19 subsetter tests )
pub fn F_SPKS19(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    testutil::TOPEN(b"F_SPKS19", ctx)?;

    //*****************************************************************
    //*
    //*    SPKS19 error cases:
    //*
    //*****************************************************************

    //
    //     Test SPKS19:  start out with error handling.
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKW19 error case: bad frame name.", ctx)?;

    //
    // End of SPKS19 error cases.
    //

    //
    // Close the SPK file at the DAF level; SPKCLS won't close
    // a file without segments.
    //

    // CALL DAFCLS ( HANDLE )
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
    // This file lacks both interval directories and mini-segment
    // epoch directories.
    //
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
        b"Test SPKS19\'s ability to copy the segment we just created.",
        ctx,
    )?;

    //
    // Search the input file for the segment we'll copy from.
    //
    spicelib::DAFOPR(SP19T1, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up segment descriptor and segment ID.
    //
    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGN(&mut save.SEGID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TMPFST = save.DSCEPC[3];
    save.TMPLST = save.DSCEPC[(save.NPKTS[1] - 2)];

    save.BEGIN = save.TMPFST;
    save.END = save.TMPLST;

    //
    // Create a new file containing the specified subset of the
    // input file. Write a new segment from scratch that
    // contains the data we expect to have in the subset file.
    // Compare the segments; verify the contents match.
    //
    // In order to compare the segments, we need to supply the
    // SPK type 19 writer with the proper inputs. Specifically,
    // the input arrays corresponding to mini-segments must
    // have the correct start indices. Otherwise the writer will
    // create a segment that contains unused data at the start.
    //
    // Let IVBEG be the start index of the arrays that
    // have a one-to-one correspondence with intervals.
    //
    // Let PKTBEG be the start index of the arrays that
    // have a one-to-one correspondence with packets.
    //
    // Let RTRUNC be the amount by which the final mini-segment's
    // packet count is truncated.
    //
    // In this case, the window size associated with the
    // first (and only) mini-segment is 4, so there needs
    // to be one pad epoch to the left of the first
    // interval start time. The packet start is therefore
    // at index 2.
    //
    save.PKTBEG = 2;

    //
    // The first interval in the subset segment is a
    // truncated version of the first interval of the
    // input file. We don't actually lose any intervals
    // in this case. So the interval index doesn't change.
    //
    save.IVBEG = 1;
    save.TMPNIV = save.NINTVL;
    //
    // The packet count and time bounds of the first
    // interval do change.
    //
    // The first mini-segment ends one epoch after the
    // last epoch contained in the mini-segment's coverage
    // interval.
    //
    save.RTRUNC = 1;

    //
    // Adjust the packet count.
    //
    save.TMPNPK[1] = ((save.NPKTS[1] - (save.PKTBEG - 1)) - save.RTRUNC);

    //
    // Adjust the interval bounds.
    //
    save.TMPIVB[1] = save.DSCEPC[save.PKTBEG];
    save.TMPIVB[2] = save.DSCEPC[((save.PKTBEG + save.TMPNPK[1]) - 1)];

    T_SPKS19(
        save.HANDLE,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.BEGIN,
        save.END,
        TMPSUB,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.TMPFST,
        save.TMPLST,
        save.TMPNIV,
        save.TMPNPK.subarray(save.IVBEG),
        save.SUBTPS.subarray(save.IVBEG),
        save.DEGRES.subarray(save.IVBEG),
        save.DSCSTS.subarray([1, save.PKTBEG]),
        save.DSCEPC.subarray(save.PKTBEG),
        save.TMPIVB.subarray(save.IVBEG),
        save.SELLST,
        true,
        OK,
        ctx,
    )?;

    //
    // Close the input file from which we created the subset segment.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The test routine T_SPKS19 deletes the subset SPK file, so
    // we don't do it here.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Create a new SPK containing a copy of the segment that was just created.",
        ctx,
    )?;

    //
    // Search the input file for the segment we'll copy from.
    //
    spicelib::DAFOPR(SP19T1, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up segment descriptor and segment ID.
    //
    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGN(&mut save.SEGID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKOPN(SUB19, SUB19, 0, &mut save.SUBHAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create subset.
    //
    spicelib::SPKSUB(
        save.HANDLE,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.FIRST,
        save.LAST,
        save.SUBHAN,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the subsetted file and the input file.
    //
    spicelib::SPKCLS(save.SUBHAN, ctx)?;
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

    spicelib::FURNSH(SUB19, ctx)?;
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

            {
                let m1__: i32 = 1;
                let m2__: i32 = 6;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.XSTATE[save.J] =
                        ((save.DSCSTS[[save.J, I]] + save.DSCSTS[[save.J, (I + 1)]]) / 2 as f64);
                    save.J += m3__;
                }
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

    spicelib::UNLOAD(SUB19, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Remove the trivial SPK we created; we'll create a new
    // one with more complexity.
    //
    spicelib::DELFIL(SUB19, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Delete the input file as well.
    //
    spicelib::DELFIL(SP19T1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*****************************************************************
    //*
    //*    Write an SPK containing a small subtype 2 segment
    //*    with two intervals. This file is needed to test packet size
    //*    detection logic for subtype 2.
    //*
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Write a 2-interval subtype 2 segment.", ctx)?;

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
    // This file lacks both interval directories and mini-segment
    // epoch directories.
    //
    spicelib::SPKOPN(SP19T2, SP19T2, 0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NINTVL = 2;

    for I in 1..=save.NINTVL {
        save.NPKTS[I] = NDSCRT;
        save.SUBTPS[I] = S19TP2;

        //
        // Use degree 7 to make window size match that of subtype 1 case
        // above.
        //
        save.DEGRES[I] = 7;
    }

    save.TO = 1;
    save.PKTTO = 1;
    save.ENDEPC = 100.0;

    for I in 1..=save.NINTVL {
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NPKTS[I];
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.EPOCHS[save.TO] = (save.ENDEPC + (((save.J - 1) as f64) * 100.0));

                for K in 1..=6 {
                    save.PACKTS[save.PKTTO] = (save.EPOCHS[save.TO] + K as f64);
                    save.PKTTO = (save.PKTTO + 1);
                }

                save.TO = (save.TO + 1);

                save.J += m3__;
            }
        }

        save.ENDEPC = save.EPOCHS[(save.TO - 1)];
    }

    save.RECTOT = (save.TO - 1);

    save.FIRST = save.EPOCHS[3];

    save.IVLBDS[1] = save.EPOCHS[2];
    save.IVLBDS[2] = save.EPOCHS[save.NPKTS[1]];
    save.IVLBDS[3] = save.EPOCHS[(save.RECTOT - 1)];

    save.LAST = save.EPOCHS[(save.RECTOT - 2)];

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
        b"Test SPKS19\'s ability to copy the segment we just created; subtype 2 case.",
        ctx,
    )?;

    //
    // Search the input file for the segment we'll copy from.
    //
    spicelib::DAFOPR(SP19T2, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up segment descriptor and segment ID.
    //
    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGN(&mut save.SEGID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TMPFST = save.EPOCHS[3];
    save.TMPLST = save.EPOCHS[(save.RECTOT - 2)];

    save.BEGIN = save.TMPFST;
    save.END = save.TMPLST;

    save.PKTSIZ = S19PS2;

    //
    // Create a new file containing the specified subset of the
    // input file. Write a new segment from scratch that
    // contains the data we expect to have in the subset file.
    // Compare the segments; verify the contents match.
    //
    // In order to compare the segments, we need to supply the
    // SPK type 19 writer with the proper inputs. Specifically,
    // the input arrays corresponding to mini-segments must
    // have the correct start indices. Otherwise the writer will
    // create a segment that contains unused data at the start.
    //
    // Let IVBEG be the start index of the arrays that
    // have a one-to-one correspondence with intervals.
    //
    // Let PKTBEG be the start index of the arrays that
    // have a one-to-one correspondence with packets.
    //
    // Let RTRUNC be the amount by which the final mini-segment's
    // packet count is truncated.
    //
    // In this case, the window size associated with the
    // both mini-segments is 4, so there needs
    // to be one pad epoch to the left of the first
    // interval start time. The packet start is therefore
    // at index 2.
    //
    save.PKTBEG = 2;

    //
    // The first interval in the subset segment is a
    // truncated version of the first interval of the
    // input file. We don't actually lose any intervals
    // in this case. So the interval index doesn't change.
    //
    save.IVBEG = 1;
    save.TMPNIV = save.NINTVL;
    //
    // The packet count and time bounds of the first
    // interval do change.
    //
    // The second mini-segment ends one epoch after the
    // last epoch contained in the mini-segment's coverage
    // interval.
    //
    save.RTRUNC = 1;

    //
    // Adjust the packet counts.
    //
    save.TMPNPK[1] = (save.NPKTS[1] - (save.PKTBEG - 1));
    save.TMPNPK[2] = (save.NPKTS[2] - save.RTRUNC);

    //
    // Adjust the interval bounds.
    //
    save.TMPIVB[1] = save.EPOCHS[save.PKTBEG];
    save.TMPIVB[2] = save.EPOCHS[save.NPKTS[1]];
    save.TMPIVB[3] = save.EPOCHS[(save.RECTOT - save.RTRUNC)];

    T_SPKS19(
        save.HANDLE,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.BEGIN,
        save.END,
        TMPSUB,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.TMPFST,
        save.TMPLST,
        save.TMPNIV,
        save.TMPNPK.subarray(save.IVBEG),
        save.SUBTPS.subarray(save.IVBEG),
        save.DEGRES.subarray(save.IVBEG),
        save.PACKTS
            .subarray((((save.PKTBEG - 1) * save.PKTSIZ) + 1)),
        save.EPOCHS.subarray(save.PKTBEG),
        save.TMPIVB.subarray(save.IVBEG),
        save.SELLST,
        true,
        OK,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the input file from which we created the subset segment.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The test routine T_SPKS19 deletes the subset SPK file, so
    // we don't do it here.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Create a new SPK containing a copy of the segment that was just created. Subtype 2 case.",
        ctx,
    )?;

    //
    // Search the input file for the segment we'll copy from.
    //
    spicelib::DAFOPR(SP19T2, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up segment descriptor and segment ID.
    //
    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGN(&mut save.SEGID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKOPN(SUB19, SUB19, 0, &mut save.SUBHAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create subset.
    //
    spicelib::SPKSUB(
        save.HANDLE,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.FIRST,
        save.LAST,
        save.SUBHAN,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the subsetted file and the input file.
    //
    spicelib::SPKCLS(save.SUBHAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Read states from the file just created. Epochs match input epochs. Subtype 2 case.",
        ctx,
    )?;

    spicelib::FURNSH(SUB19, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XBODY = 3;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");

    for I in 3..=(save.RECTOT - 2) {
        save.ET = save.EPOCHS[I];

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

            save.J = (((I - 1) * save.PKTSIZ) + 1);

            testutil::CHCKAD(
                &save.NAME,
                save.STATE.as_slice(),
                b"~~",
                save.PACKTS.subarray(save.J),
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
    testutil::TCASE(b"Read states from the file just created. Epochs are at midpoints between adjacent input epochs. Subtype 2 case.", ctx)?;

    save.XBODY = 3;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");

    //
    // Buffer expected states.
    //
    spicelib::UNLOAD(SUB19, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(SP19T2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 3..=(save.RECTOT - 3) {
        save.ET = ((save.EPOCHS[I] + save.EPOCHS[(I + 1)]) / 2 as f64);

        spicelib::SPKEZ(
            save.XBODY,
            save.ET,
            &save.XREF,
            b"NONE",
            save.XCENTR,
            save.XSTBUF.subarray_mut([1, I]),
            &mut save.LT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::UNLOAD(SP19T2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(SUB19, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 3..=(save.RECTOT - 3) {
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

        if *OK {
            fstr::assign(&mut save.NAME, b"State #.");
            spicelib::REPMI(&save.NAME.to_vec(), b"#", I, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::MOVED(save.XSTBUF.subarray([1, I]), 6, save.XSTATE.as_slice_mut());

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

    spicelib::UNLOAD(SUB19, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Remove the trivial SPK we created; we'll create a new
    // one with more complexity.
    //
    spicelib::DELFIL(SUB19, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Delete the input file as well.
    //
    spicelib::DELFIL(SP19T2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*****************************************************************
    //*
    //*    Large segment subsetting cases:
    //*
    //*****************************************************************

    //
    // Below, we'll use type 0 segments because it will be convenient to
    // use cubic interpolation and a window size of 2.
    //
    // The velocity/acceleration data will not be compatible with the
    // position/velocity data.
    //
    // We need a segment containing enough interval directories
    // so that multiple buffers full of directory entries must
    // be read. This implies we need more than DIRSIZ**2 intervals
    // in the segment.
    //
    // Note that these cases do not exercise padding logic.
    //
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create an SPK with interval directories.", ctx)?;

    spicelib::SPKOPN(SP19MX, SP19MX, 0, &mut save.HANDLE, ctx)?;
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

    //
    // Set the interval count.
    //
    save.NINTVL = MAXIVL;

    //
    // PKTSUM is an array whose Ith element contains the sum of
    // counts of elements in the first through Ith packets.
    // Initialize PKTSUM here.
    //
    spicelib::CLEARI((MAXEPC + 1), save.PKTSUM.subarray_mut(0));

    //
    // Set the packet counts, subtypes, and degrees.
    //
    // Since we're concentrating on interval selection
    // logic, we'll mostly use small packet counts. However,
    // we'll create larger mini-segments at the beginning
    // and end of the segment.
    //
    for I in 1..=save.NINTVL {
        if (I <= 2) {
            save.SUBTPS[I] = S19TP0;
            save.PKTSZS[I] = S19PS0;
            save.NPKTS[I] = 10001;
            save.DEGRES[I] = 11;
        } else if (I >= (save.NINTVL - 1)) {
            //
            // Vary the subtype; use subtype 1.
            // Adjust the degree to keep the window
            // size equal to 6.
            //
            save.SUBTPS[I] = S19TP1;
            save.PKTSZS[I] = S19PS1;
            save.NPKTS[I] = 10001;
            save.DEGRES[I] = 5;
        } else if (intrinsics::MOD(I, 3) == 0) {
            //
            // Vary the subtype, based on equivalence class of the interval
            // index.
            //
            save.SUBTPS[I] = S19TP1;
            save.PKTSZS[I] = S19PS1;
            save.NPKTS[I] = 3;
            save.DEGRES[I] = 1;
        } else if (intrinsics::MOD(I, 3) == 1) {
            save.SUBTPS[I] = S19TP0;
            save.PKTSZS[I] = S19PS0;
            save.NPKTS[I] = 4;
            save.DEGRES[I] = 3;
        } else {
            save.SUBTPS[I] = S19TP2;
            save.PKTSZS[I] = S19PS2;
            save.NPKTS[I] = 4;
            save.DEGRES[I] = 3;
        }
    }

    //
    // Set the pad size for degree 11 Hermite interpolation.
    //
    // Create padding at the boundaries of the first two and
    // last two intervals.
    //
    save.NPAD = 2;

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

        //
        // Create left-side padding for the first two and last
        // two intervals.
        //
        if ((I <= 2) || (I >= (save.NINTVL - 1))) {
            {
                let m1__: i32 = (1 - save.NPAD);
                let m2__: i32 = 0;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.EPCTO = (save.EPCTO + 1);

                    save.EPOCHS[save.EPCTO] = (save.ENDEPC + (save.ESCALE * (save.J - 1) as f64));

                    save.PKTSUM[save.EPCTO] = (save.PKTSUM[(save.EPCTO - 1)] + save.PKTSZS[I]);

                    save.J += m3__;
                }
            }
        }

        //
        // This is the common epoch-generating loop.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NPKTS[I];
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.EPCTO = (save.EPCTO + 1);

                save.EPOCHS[save.EPCTO] = (save.ENDEPC + (save.ESCALE * (save.J - 1) as f64));

                save.PKTSUM[save.EPCTO] = (save.PKTSUM[(save.EPCTO - 1)] + save.PKTSZS[I]);

                save.J += m3__;
            }
        }

        //
        // Save the last epoch of this interval.
        //
        save.ENDEPC = save.EPOCHS[save.EPCTO];

        //
        // Create right-side padding for the first two intervals.
        //
        if ((I <= 2) || (I >= (save.NINTVL - 1))) {
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NPAD;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.EPCTO = (save.EPCTO + 1);

                    save.EPOCHS[save.EPCTO] = (save.ENDEPC + (save.ESCALE * save.J as f64));

                    save.PKTSUM[save.EPCTO] = (save.PKTSUM[(save.EPCTO - 1)] + save.PKTSZS[I]);

                    save.J += m3__;
                }
            }

            //
            // Update the packet count for the current interval.
            //
            save.NPKTS[I] = (save.NPKTS[I] + (2 * save.NPAD));
        }
    }

    //
    // Let RECTOT be the total number of records in the segment.
    //
    save.RECTOT = save.EPCTO;

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
    // interpolation for the middle segments,
    // selecting the wrong packets to interpolate
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
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NPKTS[I];
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                //
                // Position components:
                //
                for K in 1..=3 {
                    save.PKTTO = (save.PKTTO + 1);

                    save.PACKTS[save.PKTTO] =
                        (((1000000000.0 * I as f64) + (100.0 * save.J as f64)) + K as f64);
                }

                //
                // Velocities associated with positions:
                //
                for K in 1..=3 {
                    save.PKTTO = (save.PKTTO + 1);

                    save.PACKTS[save.PKTTO] = 0.0;
                }

                if (save.PKTSZS[I] == S19PS0) {
                    //
                    // Velocity/acceleration packets:
                    //
                    for K in 1..=6 {
                        save.PKTTO = (save.PKTTO + 1);

                        save.PACKTS[save.PKTTO] = (save.PACKTS[(save.PKTTO - 6)] * 10.0);
                    }
                }

                save.J += m3__;
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
    testutil::TCASE(b"Trivial case: subset is whole file.", ctx)?;

    //
    // Search the input file for the segment we'll copy from.
    //
    spicelib::DAFOPR(SP19MX, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up segment descriptor and segment ID.
    //
    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGN(&mut save.SEGID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XBODY = 3;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");
    //
    // Create a segment identifier.
    //
    fstr::assign(&mut save.SEGID, b"SPK type 19 test segment");

    //
    // Set the interval count.
    //
    save.TMPNIV = save.NINTVL;

    save.BEGIN = save.IVLBDS[1];
    save.END = save.IVLBDS[(save.NINTVL + 1)];

    T_SPKS19(
        save.HANDLE,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.BEGIN,
        save.END,
        TMPSUB,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.BEGIN,
        save.END,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.EPOCHS.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        true,
        OK,
        ctx,
    )?;

    //
    // Close the input file from which we created the subset segment.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test the structure of a subsetted segment created from the large SPK file. Coverage is for the singleton interval located at the first interval start time.", ctx)?;

    //
    // Search the input file for the segment we'll copy from.
    //
    spicelib::DAFOPR(SP19MX, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up segment descriptor and segment ID.
    //
    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGN(&mut save.SEGID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XBODY = 3;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");
    //
    // Create a segment identifier.
    //
    fstr::assign(&mut save.SEGID, b"SPK type 19 test segment");

    //
    // Set the interval count.
    //
    save.TMPNIV = 1;

    //
    // Set the index of the first packet we'll use.
    //
    save.PKTBEG = 1;

    //
    // PKTOFF is the offset of the first packet to be
    // used. Since PACKTS is a one-dimensional array,
    // we must skip over the total number of addresses
    // occupied by the packets that precede the first
    // of the subset.
    //
    save.PKTOFF = (save.PKTSUM[(save.PKTBEG - 1)] + 1);

    //
    // The first interval in the subset segment is a
    // truncated version of the first interval of the
    // input file. We don't actually lose any intervals
    // in this case. So the interval index doesn't change.
    //
    save.IVBEG = 1;
    //
    // Copy the packet counts.
    //
    spicelib::MOVEI(
        save.NPKTS.as_slice(),
        save.TMPNIV,
        save.TMPNPK.as_slice_mut(),
    );

    //
    // Adjust the first and last packet counts.
    //
    save.TMPNPK[1] = ((2 * save.NPAD) + 1);
    //
    // Adjust the interval bounds.
    //
    spicelib::MOVED(
        save.IVLBDS.as_slice(),
        (save.NINTVL + 1),
        save.TMPIVB.as_slice_mut(),
    );

    save.TMPIVB[1] = save.EPOCHS[(save.PKTBEG + save.NPAD)];
    save.TMPIVB[(save.TMPNIV + 1)] = save.EPOCHS[(save.PKTBEG + (2 * save.NPAD))];

    save.BEGIN = save.IVLBDS[1];
    save.END = save.BEGIN;

    //
    // Save the subset file created by the following call.
    //
    T_SPKS19(
        save.HANDLE,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.BEGIN,
        save.END,
        TMPSUB,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.BEGIN,
        save.END,
        save.TMPNIV,
        save.TMPNPK.subarray(save.IVBEG),
        save.SUBTPS.subarray(save.IVBEG),
        save.DEGRES.subarray(save.IVBEG),
        save.PACKTS.subarray(save.PKTOFF),
        save.EPOCHS.subarray(save.PKTBEG),
        save.TMPIVB.subarray(save.IVBEG),
        save.SELLST,
        false,
        OK,
        ctx,
    )?;

    //
    // Close the input file from which we created the subset segment.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Compare states looked up from the input file and the subset
    // file.
    //
    spicelib::FURNSH(SP19MX, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKEZ(
        save.XBODY,
        save.BEGIN,
        &save.XREF,
        b"NONE",
        save.XCENTR,
        save.XSTATE.as_slice_mut(),
        &mut save.XLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(SP19MX, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(TMPSUB, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKEZ(
        save.XBODY,
        save.BEGIN,
        &save.XREF,
        b"NONE",
        save.XCENTR,
        save.STATE.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(TMPSUB, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"Position: ",
        save.STATE.as_slice(),
        b"=",
        save.XSTATE.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"Velocity: ",
        save.STATE.subarray(4),
        b"=",
        save.XSTATE.subarray(4),
        3,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Clean up the subset file.
    //
    if spicelib::EXISTS(TMPSUB, ctx)? {
        spicelib::DELFIL(TMPSUB, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test the structure of a subsetted segment created from the large SPK file. Coverage is for the singleton interval located at the first interval end time.", ctx)?;

    //
    // Search the input file for the segment we'll copy from.
    //
    spicelib::DAFOPR(SP19MX, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up segment descriptor and segment ID.
    //
    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGN(&mut save.SEGID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XBODY = 3;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");
    //
    // Create a segment identifier.
    //
    fstr::assign(&mut save.SEGID, b"SPK type 19 test segment");

    //
    // In this case, since the segment time coincides with the end
    // time of the first interval, SPKS19 will append a second,
    // small interval to the first output interval.
    //
    // Set the interval count.
    //
    save.TMPNIV = 2;

    //
    // Set the index of the first packet we'll use.
    //
    save.PKTBEG = (save.NPKTS[1] - (2 * save.NPAD));

    //
    // PKTOFF is the offset of the first packet to be
    // used. Since PACKTS is a one-dimensional array,
    // we must skip over the total number of addresses
    // occupied by the packets that precede the first
    // of the subset.
    //
    save.PKTOFF = (save.PKTSUM[(save.PKTBEG - 1)] + 1);

    //
    // The first interval in the subset segment is a
    // truncated version of the first interval of the
    // input file. We don't actually lose any intervals
    // in this case. So the interval index doesn't change.
    //
    save.IVBEG = 1;
    //
    // Copy the packet counts.
    //
    spicelib::MOVEI(
        save.NPKTS.as_slice(),
        save.TMPNIV,
        save.TMPNPK.as_slice_mut(),
    );

    //
    // Adjust the first and last packet counts.
    //
    save.TMPNPK[1] = ((2 * save.NPAD) + 1);
    save.TMPNPK[2] = ((2 * save.NPAD) + 1);

    //
    // Adjust the interval bounds.
    //
    spicelib::MOVED(
        save.IVLBDS.as_slice(),
        (save.NINTVL + 1),
        save.TMPIVB.as_slice_mut(),
    );

    save.TMPIVB[1] = save.EPOCHS[save.PKTBEG];
    save.TMPIVB[2] = save.EPOCHS[(save.PKTBEG + save.NPAD)];
    save.TMPIVB[(save.TMPNIV + 1)] = save.EPOCHS[((save.PKTBEG + (4 * save.NPAD)) + 1)];

    save.BEGIN = save.IVLBDS[2];
    save.END = save.BEGIN;

    //
    // Save the subset file created by the following call.
    //
    T_SPKS19(
        save.HANDLE,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.BEGIN,
        save.END,
        TMPSUB,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.BEGIN,
        save.END,
        save.TMPNIV,
        save.TMPNPK.subarray(save.IVBEG),
        save.SUBTPS.subarray(save.IVBEG),
        save.DEGRES.subarray(save.IVBEG),
        save.PACKTS.subarray(save.PKTOFF),
        save.EPOCHS.subarray(save.PKTBEG),
        save.TMPIVB.subarray(save.IVBEG),
        save.SELLST,
        false,
        OK,
        ctx,
    )?;

    //
    // Close the input file from which we created the subset segment.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Compare states looked up from the input file and the subset
    // file.
    //
    spicelib::FURNSH(SP19MX, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKEZ(
        save.XBODY,
        save.BEGIN,
        &save.XREF,
        b"NONE",
        save.XCENTR,
        save.XSTATE.as_slice_mut(),
        &mut save.XLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(SP19MX, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(TMPSUB, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKEZ(
        save.XBODY,
        save.BEGIN,
        &save.XREF,
        b"NONE",
        save.XCENTR,
        save.STATE.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(TMPSUB, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"Position: ",
        save.STATE.as_slice(),
        b"=",
        save.XSTATE.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"Velocity: ",
        save.STATE.subarray(4),
        b"=",
        save.XSTATE.subarray(4),
        3,
        0.0,
        OK,
        ctx,
    )?;

    if spicelib::EXISTS(TMPSUB, ctx)? {
        spicelib::DELFIL(TMPSUB, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test the structure of a subsetted segment created from the large SPK file. Coverage is for the singleton interval located at the last interval end time.", ctx)?;

    //
    // Search the input file for the segment we'll copy from.
    //
    spicelib::DAFOPR(SP19MX, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up segment descriptor and segment ID.
    //
    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGN(&mut save.SEGID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XBODY = 3;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");
    //
    // Create a segment identifier.
    //
    fstr::assign(&mut save.SEGID, b"SPK type 19 test segment");

    //
    // Set the interval count.
    //
    save.TMPNIV = 1;

    //
    // Set the index of the first packet we'll use.
    //
    save.PKTBEG = (save.RECTOT - (2 * save.NPAD));

    //
    // PKTOFF is the offset of the first packet to be
    // used. Since PACKTS is a one-dimensional array,
    // we must skip over the total number of addresses
    // occupied by the packets that precede the first
    // of the subset.
    //
    save.PKTOFF = (save.PKTSUM[(save.PKTBEG - 1)] + 1);

    //
    // The first interval in the subset segment is a
    // truncated version of the first interval of the
    // input file. We don't actually lose any intervals
    // in this case. So the interval index doesn't change.
    //
    save.IVBEG = save.NINTVL;
    //
    // Copy the packet counts.
    //
    spicelib::MOVEI(
        save.NPKTS.as_slice(),
        save.TMPNIV,
        save.TMPNPK.as_slice_mut(),
    );

    //
    // Adjust the last packet count.
    //
    save.TMPNPK[save.NINTVL] = ((2 * save.NPAD) + 1);

    //
    // Adjust the interval bounds.
    //
    spicelib::MOVED(
        save.IVLBDS.as_slice(),
        (save.NINTVL + 1),
        save.TMPIVB.as_slice_mut(),
    );

    save.TMPIVB[save.IVBEG] = save.EPOCHS[save.PKTBEG];
    save.TMPIVB[(save.IVBEG + 1)] = save.IVLBDS[(save.NINTVL + 1)];

    save.BEGIN = save.IVLBDS[(save.NINTVL + 1)];
    save.END = save.BEGIN;

    //
    // Save the subset file created by the following call.
    //
    T_SPKS19(
        save.HANDLE,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.BEGIN,
        save.END,
        TMPSUB,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.BEGIN,
        save.END,
        save.TMPNIV,
        save.TMPNPK.subarray(save.IVBEG),
        save.SUBTPS.subarray(save.IVBEG),
        save.DEGRES.subarray(save.IVBEG),
        save.PACKTS.subarray(save.PKTOFF),
        save.EPOCHS.subarray(save.PKTBEG),
        save.TMPIVB.subarray(save.IVBEG),
        save.SELLST,
        false,
        OK,
        ctx,
    )?;

    //
    // Close the input file from which we created the subset segment.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Compare states looked up from the input file and the subset
    // file.
    //
    spicelib::FURNSH(SP19MX, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKEZ(
        save.XBODY,
        save.BEGIN,
        &save.XREF,
        b"NONE",
        save.XCENTR,
        save.XSTATE.as_slice_mut(),
        &mut save.XLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(SP19MX, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(TMPSUB, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKEZ(
        save.XBODY,
        save.BEGIN,
        &save.XREF,
        b"NONE",
        save.XCENTR,
        save.STATE.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(TMPSUB, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"Position: ",
        save.STATE.as_slice(),
        b"=",
        save.XSTATE.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"Velocity: ",
        save.STATE.subarray(4),
        b"=",
        save.XSTATE.subarray(4),
        3,
        0.0,
        OK,
        ctx,
    )?;

    if spicelib::EXISTS(TMPSUB, ctx)? {
        spicelib::DELFIL(TMPSUB, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test the structure of a subsetted segment created from the large SPK file. Coverage is for the singleton interval located between two interior epochs.", ctx)?;

    //
    // Search the input file for the segment we'll copy from.
    //
    spicelib::DAFOPR(SP19MX, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up segment descriptor and segment ID.
    //
    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGN(&mut save.SEGID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XBODY = 3;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");
    //
    // Create a segment identifier.
    //
    fstr::assign(&mut save.SEGID, b"SPK type 19 test segment");

    //
    // Set the interval count.
    //
    save.TMPNIV = 1;

    //
    // Set the index of the first packet we'll use.
    //
    save.PKTBEG = 500;

    //
    // PKTOFF is the offset of the first packet to be
    // used. Since PACKTS is a one-dimensional array,
    // we must skip over the total number of addresses
    // occupied by the packets that precede the first
    // of the subset.
    //
    save.PKTOFF = (save.PKTSUM[(save.PKTBEG - 1)] + 1);

    //
    // The first interval in the subset segment is a
    // truncated version of the first interval of the
    // input file. We don't actually lose any intervals
    // in this case. So the interval index doesn't change.
    //
    save.IVBEG = 1;
    //
    // Copy the packet counts.
    //
    spicelib::MOVEI(
        save.NPKTS.as_slice(),
        save.TMPNIV,
        save.TMPNPK.as_slice_mut(),
    );

    //
    // Adjust the first packet count.
    //
    save.TMPNPK[1] = ((2 * save.NPAD) + 2);

    //
    // Adjust the interval bounds.
    //
    spicelib::MOVED(
        save.IVLBDS.as_slice(),
        (save.NINTVL + 1),
        save.TMPIVB.as_slice_mut(),
    );

    save.TMPIVB[save.IVBEG] = save.EPOCHS[save.PKTBEG];
    save.TMPIVB[(save.IVBEG + 1)] = save.EPOCHS[((save.PKTBEG + (2 * save.NPAD)) + 1)];

    save.BEGIN = (save.EPOCHS[(save.PKTBEG + save.NPAD)] + 1.0);
    save.END = save.BEGIN;

    //
    // Save the subset file created by the following call.
    //
    T_SPKS19(
        save.HANDLE,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.BEGIN,
        save.END,
        TMPSUB,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.BEGIN,
        save.END,
        save.TMPNIV,
        save.TMPNPK.subarray(save.IVBEG),
        save.SUBTPS.subarray(save.IVBEG),
        save.DEGRES.subarray(save.IVBEG),
        save.PACKTS.subarray(save.PKTOFF),
        save.EPOCHS.subarray(save.PKTBEG),
        save.TMPIVB.subarray(save.IVBEG),
        save.SELLST,
        false,
        OK,
        ctx,
    )?;

    //
    // Close the input file from which we created the subset segment.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Compare states looked up from the input file and the subset
    // file.
    //
    spicelib::FURNSH(SP19MX, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKEZ(
        save.XBODY,
        save.BEGIN,
        &save.XREF,
        b"NONE",
        save.XCENTR,
        save.XSTATE.as_slice_mut(),
        &mut save.XLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(SP19MX, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(TMPSUB, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKEZ(
        save.XBODY,
        save.BEGIN,
        &save.XREF,
        b"NONE",
        save.XCENTR,
        save.STATE.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(TMPSUB, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"Position: ",
        save.STATE.as_slice(),
        b"=",
        save.XSTATE.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"Velocity: ",
        save.STATE.subarray(4),
        b"=",
        save.XSTATE.subarray(4),
        3,
        0.0,
        OK,
        ctx,
    )?;

    if spicelib::EXISTS(TMPSUB, ctx)? {
        spicelib::DELFIL(TMPSUB, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test the structure of a subsetted segment created from the large SPK file. Coverage is for the singleton interval located at the end of the third interpolation interval.", ctx)?;
    //
    // This case exercises epoch selection logic for the final
    // output mini-segment. In this case, the input mini-segments
    // from which the output mini-segments are taken contain no
    // no padding.
    //

    //
    // Search the input file for the segment we'll copy from.
    //
    spicelib::DAFOPR(SP19MX, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up segment descriptor and segment ID.
    //
    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGN(&mut save.SEGID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XBODY = 3;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");
    //
    // Create a segment identifier.
    //
    fstr::assign(&mut save.SEGID, b"SPK type 19 test segment");

    //
    // Set the interval count.
    //
    save.TMPNIV = 2;

    //
    // Set the index of the first packet we'll use.
    //
    save.CPAD = 1;

    save.PKTBEG = 0;

    for I in 1..=3 {
        save.PKTBEG = (save.NPKTS[I] + save.PKTBEG);
    }

    save.PKTBEG = (save.PKTBEG - save.CPAD);

    //
    // PKTOFF is the offset of the first packet to be
    // used. Since PACKTS is a one-dimensional array,
    // we must skip over the total number of addresses
    // occupied by the packets that precede the first
    // of the subset.
    //
    save.PKTOFF = (save.PKTSUM[(save.PKTBEG - 1)] + 1);

    //
    // The first interval in the subset segment is a
    // truncated version of the third interval of the
    // input file.
    //
    save.IVBEG = 3;
    //
    // Copy the packet counts.
    //
    spicelib::MOVEI(
        save.NPKTS.subarray(save.IVBEG),
        save.TMPNIV,
        save.TMPNPK.as_slice_mut(),
    );

    //
    // Adjust the packet counts.
    //
    save.TMPNPK[save.IVBEG] = (1 + save.CPAD);
    save.TMPNPK[(save.IVBEG + 1)] = (1 + save.CPAD);

    //
    // Adjust the interval bounds.
    //
    spicelib::MOVED(
        save.IVLBDS.as_slice(),
        (save.NINTVL + 1),
        save.TMPIVB.as_slice_mut(),
    );

    save.TMPIVB[save.IVBEG] = save.EPOCHS[save.PKTBEG];
    save.TMPIVB[(save.IVBEG + 1)] = save.IVLBDS[(save.IVBEG + 1)];
    //
    // Note that the epochs at indices
    //
    //    PKTBEG + CPAD
    //    PKTBEG + 2*CPAD
    //
    // are, respectively, the last epoch of interval 3 and the
    // first of interval 4; hence they coincide. The next
    // distinct epoch is at index
    //
    //    PKTBEG + 3*CPAD
    //
    save.TMPIVB[(save.IVBEG + 2)] = save.EPOCHS[(save.PKTBEG + (3 * save.CPAD))];

    save.BEGIN = save.IVLBDS[4];
    save.END = save.IVLBDS[4];

    //
    // Save the subset file created by the following call.
    //
    T_SPKS19(
        save.HANDLE,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.BEGIN,
        save.END,
        TMPSUB,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.BEGIN,
        save.END,
        save.TMPNIV,
        save.TMPNPK.subarray(save.IVBEG),
        save.SUBTPS.subarray(save.IVBEG),
        save.DEGRES.subarray(save.IVBEG),
        save.PACKTS.subarray(save.PKTOFF),
        save.EPOCHS.subarray(save.PKTBEG),
        save.TMPIVB.subarray(save.IVBEG),
        save.SELLST,
        false,
        OK,
        ctx,
    )?;

    //
    // Close the input file from which we created the subset segment.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Compare states looked up from the input file and the subset
    // file.
    //
    spicelib::FURNSH(SP19MX, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKEZ(
        save.XBODY,
        save.BEGIN,
        &save.XREF,
        b"NONE",
        save.XCENTR,
        save.XSTATE.as_slice_mut(),
        &mut save.XLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(SP19MX, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(TMPSUB, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKEZ(
        save.XBODY,
        save.BEGIN,
        &save.XREF,
        b"NONE",
        save.XCENTR,
        save.STATE.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(TMPSUB, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"Position: ",
        save.STATE.as_slice(),
        b"=",
        save.XSTATE.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"Velocity: ",
        save.STATE.subarray(4),
        b"=",
        save.XSTATE.subarray(4),
        3,
        0.0,
        OK,
        ctx,
    )?;

    if spicelib::EXISTS(TMPSUB, ctx)? {
        spicelib::DELFIL(TMPSUB, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test the structure of a subsetted segment created from the large SPK file. Use the entire first mini-segment. Inset the segment end time slightly to prevent addition of a small interval on the right.", ctx)?;

    //
    // Search the input file for the segment we'll copy from.
    //
    spicelib::DAFOPR(SP19MX, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up segment descriptor and segment ID.
    //
    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGN(&mut save.SEGID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XBODY = 3;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");
    //
    // Create a segment identifier.
    //
    fstr::assign(&mut save.SEGID, b"SPK type 19 test segment");
    //
    // Set the interval count.
    //
    save.TMPNIV = 1;

    //
    // Set the index of the first packet we'll use.
    //
    save.PKTBEG = 1;

    //
    // PKTOFF is the offset of the first packet to be
    // used. Since PACKTS is a one-dimensional array,
    // we must skip over the total number of addresses
    // occupied by the packets that precede the first
    // of the subset.
    //
    // Note that PKTSUM has a 0th element which is set to 0.
    //
    save.PKTOFF = (save.PKTSUM[(save.PKTBEG - 1)] + 1);

    //
    // The first interval in the subset segment is the
    // first interval of the input file.
    //
    save.IVBEG = 1;

    //
    // Copy the packet counts.
    //
    spicelib::MOVEI(
        save.NPKTS.as_slice(),
        save.NINTVL,
        save.TMPNPK.as_slice_mut(),
    );

    //
    // Copy the interval bounds.
    //
    spicelib::MOVED(
        save.IVLBDS.as_slice(),
        (save.NINTVL + 1),
        save.TMPIVB.as_slice_mut(),
    );

    save.TMPTOT = save.TMPNPK[1];

    save.BEGIN = save.EPOCHS[(save.PKTBEG + save.NPAD)];
    save.END = (save.EPOCHS[(((save.PKTBEG + save.TMPTOT) - 1) - save.NPAD)] - 1.0);

    T_SPKS19(
        save.HANDLE,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.BEGIN,
        save.END,
        TMPSUB,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.BEGIN,
        save.END,
        save.TMPNIV,
        save.TMPNPK.subarray(save.IVBEG),
        save.SUBTPS.subarray(save.IVBEG),
        save.DEGRES.subarray(save.IVBEG),
        save.PACKTS.subarray(save.PKTOFF),
        save.EPOCHS.subarray(save.PKTBEG),
        save.TMPIVB.subarray(save.IVBEG),
        save.SELLST,
        true,
        OK,
        ctx,
    )?;

    //
    // Close the input file from which we created the subset segment.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test the structure of a subsetted segment created from the large SPK file. Use the entire first mini-segment.", ctx)?;

    //
    // Search the input file for the segment we'll copy from.
    //
    spicelib::DAFOPR(SP19MX, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up segment descriptor and segment ID.
    //
    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGN(&mut save.SEGID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XBODY = 3;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");
    //
    // Create a segment identifier.
    //
    fstr::assign(&mut save.SEGID, b"SPK type 19 test segment");
    //
    // Set the interval count. The output segment will contain
    // a second, small interval on the right to support
    // correct interpolation when the request time is the
    // end time of the first interval and the interval
    // selection method is "select last interval."
    //
    save.TMPNIV = 2;

    //
    // Set the index of the first packet we'll use.
    //
    save.PKTBEG = 1;

    //
    // PKTOFF is the offset of the first packet to be
    // used. Since PACKTS is a one-dimensional array,
    // we must skip over the total number of addresses
    // occupied by the packets that precede the first
    // of the subset.
    //
    save.PKTOFF = (save.PKTSUM[(save.PKTBEG - 1)] + 1);

    //
    // The first interval in the subset segment is the
    // first interval of the input file.
    //
    save.IVBEG = 1;

    //
    // Copy and adjust the packet counts.
    //
    spicelib::MOVEI(
        save.NPKTS.as_slice(),
        save.NINTVL,
        save.TMPNPK.as_slice_mut(),
    );

    save.TMPNPK[2] = ((2 * save.NPAD) + 1);

    //
    // Copy and adjust the interval bounds.
    //
    spicelib::MOVED(
        save.IVLBDS.as_slice(),
        (save.NINTVL + 1),
        save.TMPIVB.as_slice_mut(),
    );

    save.TMPIVB[3] = save.EPOCHS[((save.NPKTS[1] + 1) + (2 * save.NPAD))];

    save.TMPTOT = (save.TMPNPK[1] + save.TMPNPK[2]);

    save.BEGIN = save.IVLBDS[1];
    save.END = save.IVLBDS[2];

    T_SPKS19(
        save.HANDLE,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.BEGIN,
        save.END,
        TMPSUB,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.BEGIN,
        save.END,
        save.TMPNIV,
        save.TMPNPK.subarray(save.IVBEG),
        save.SUBTPS.subarray(save.IVBEG),
        save.DEGRES.subarray(save.IVBEG),
        save.PACKTS.subarray(save.PKTOFF),
        save.EPOCHS.subarray(save.PKTBEG),
        save.TMPIVB.subarray(save.IVBEG),
        save.SELLST,
        true,
        OK,
        ctx,
    )?;

    //
    // Close the input file from which we created the subset segment.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test the structure of a subsetted segment created from the large SPK file. Create a file with one interpolation interval. Skip over the first 500 packets of the first interval, and truncate the last 500 packets of the interval. Start and stop times are inset and lie between members of the epoch list.", ctx)?;

    //
    // Search the input file for the segment we'll copy from.
    //
    spicelib::DAFOPR(SP19MX, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up segment descriptor and segment ID.
    //
    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGN(&mut save.SEGID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XBODY = 3;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");
    //
    // Create a segment identifier.
    //
    fstr::assign(&mut save.SEGID, b"SPK type 19 test segment");

    //
    // Set the interval count.
    //
    save.TMPNIV = 1;

    //
    // Set the index of the first packet we'll use.
    //
    save.PKTBEG = 501;

    //
    // PKTOFF is the offset of the first packet to be
    // used. Since PACKTS is a one-dimensional array,
    // we must skip over the total number of addresses
    // occupied by the packets that precede the first
    // of the subset.
    //
    save.PKTOFF = (save.PKTSUM[(save.PKTBEG - 1)] + 1);

    //
    // The first interval in the subset segment is a
    // truncated version of the first interval of the
    // input file. We don't actually lose any intervals
    // in this case. So the interval index doesn't change.
    //
    save.IVBEG = 1;
    //
    // The packet count and time bounds of the first
    // interval do change.
    //
    // The first mini-segment ends one epoch after the
    // last epoch contained in the mini-segment's coverage
    // interval.
    //
    save.RTRUNC = 500;

    //
    // Copy the packet counts.
    //
    spicelib::MOVEI(
        save.NPKTS.as_slice(),
        save.TMPNIV,
        save.TMPNPK.as_slice_mut(),
    );

    //
    // Adjust the first and last packet counts.
    //
    save.TMPNPK[1] = ((save.NPKTS[1] - (save.PKTBEG - 1)) - save.RTRUNC);

    //
    // Adjust the interval bounds.
    //
    spicelib::MOVED(
        save.IVLBDS.as_slice(),
        (save.NINTVL + 1),
        save.TMPIVB.as_slice_mut(),
    );

    //
    // Since we have only one interval, the total epoch
    // count is obtained by subtracting the sizes of
    // the initial and final truncated portions from the
    // initial size of the first mini-segment.
    //
    save.TMPTOT = ((save.NPKTS[1] - (save.PKTBEG - 1)) - save.RTRUNC);

    save.TMPIVB[1] = save.EPOCHS[save.PKTBEG];
    save.TMPIVB[(save.TMPNIV + 1)] = save.EPOCHS[((save.PKTBEG + save.TMPTOT) - 1)];

    //
    // Inset BEGIN and END by the pad size.
    //
    save.BEGIN = (save.EPOCHS[(save.PKTBEG + save.NPAD)] + 1.0);
    save.END = (save.EPOCHS[(((save.PKTBEG + save.TMPTOT) - 1) - save.NPAD)] - 1.0);

    T_SPKS19(
        save.HANDLE,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.BEGIN,
        save.END,
        TMPSUB,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.BEGIN,
        save.END,
        save.TMPNIV,
        save.TMPNPK.subarray(save.IVBEG),
        save.SUBTPS.subarray(save.IVBEG),
        save.DEGRES.subarray(save.IVBEG),
        save.PACKTS.subarray(save.PKTOFF),
        save.EPOCHS.subarray(save.PKTBEG),
        save.TMPIVB.subarray(save.IVBEG),
        save.SELLST,
        true,
        OK,
        ctx,
    )?;

    //
    // Close the input file from which we created the subset segment.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test the structure of a subsetted segment created from the large SPK file. Create a file with one interpolation interval. Skip over the first 500 packets of the first interval, and truncate the last 500 packets of the interval. Start and stop times coincide with members of the epoch list.", ctx)?;

    //
    // Search the input file for the segment we'll copy from.
    //
    spicelib::DAFOPR(SP19MX, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up segment descriptor and segment ID.
    //
    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGN(&mut save.SEGID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XBODY = 3;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");
    //
    // Create a segment identifier.
    //
    fstr::assign(&mut save.SEGID, b"SPK type 19 test segment");

    //
    // Set the interval count.
    //
    save.TMPNIV = 1;

    //
    // Set the index of the first packet we'll use.
    //
    save.PKTBEG = 501;

    //
    // PKTOFF is the offset of the first packet to be
    // used. Since PACKTS is a one-dimensional array,
    // we must skip over the total number of addresses
    // occupied by the packets that precede the first
    // of the subset.
    //
    save.PKTOFF = (save.PKTSUM[(save.PKTBEG - 1)] + 1);

    //
    // The first interval in the subset segment is a
    // truncated version of the first interval of the
    // input file. We don't actually lose any intervals
    // in this case. So the interval index doesn't change.
    //
    save.IVBEG = 1;
    //
    // The packet count and time bounds of the first
    // interval do change.
    //
    // The first mini-segment ends one epoch after the
    // last epoch contained in the mini-segment's coverage
    // interval.
    //
    save.RTRUNC = 500;

    //
    // Copy the packet counts.
    //
    spicelib::MOVEI(
        save.NPKTS.as_slice(),
        save.TMPNIV,
        save.TMPNPK.as_slice_mut(),
    );

    //
    // Adjust the first and last packet counts.
    //
    save.TMPNPK[1] = ((save.NPKTS[1] - (save.PKTBEG - 1)) - save.RTRUNC);

    //
    // Adjust the interval bounds.
    //
    spicelib::MOVED(
        save.IVLBDS.as_slice(),
        (save.NINTVL + 1),
        save.TMPIVB.as_slice_mut(),
    );

    //
    // Since we have only one interval, the total epoch
    // count is obtained by subtracting the sizes of
    // the initial and final truncated portions from the
    // initial size of the first mini-segment.
    //
    save.TMPTOT = ((save.NPKTS[1] - (save.PKTBEG - 1)) - save.RTRUNC);

    save.TMPIVB[1] = save.EPOCHS[save.PKTBEG];
    save.TMPIVB[(save.TMPNIV + 1)] = save.EPOCHS[((save.PKTBEG + save.TMPTOT) - 1)];

    save.BEGIN = save.EPOCHS[(save.PKTBEG + save.NPAD)];
    save.END = save.EPOCHS[(((save.PKTBEG + save.TMPTOT) - 1) - save.NPAD)];

    T_SPKS19(
        save.HANDLE,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.BEGIN,
        save.END,
        TMPSUB,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.BEGIN,
        save.END,
        save.TMPNIV,
        save.TMPNPK.subarray(save.IVBEG),
        save.SUBTPS.subarray(save.IVBEG),
        save.DEGRES.subarray(save.IVBEG),
        save.PACKTS.subarray(save.PKTOFF),
        save.EPOCHS.subarray(save.PKTBEG),
        save.TMPIVB.subarray(save.IVBEG),
        save.SELLST,
        true,
        OK,
        ctx,
    )?;

    //
    // Close the input file from which we created the subset segment.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test the structure of a subsetted segment created from the large SPK file. Create a file with two interpolation intervals. Skip over the first 500 packets of the first interval, and truncate the last 500 packets of the second interval. The output segment has no \"middle group.\" Start and stop times are inset; they lie between members of the epoch list.", ctx)?;

    //
    // Search the input file for the segment we'll copy from.
    //
    spicelib::DAFOPR(SP19MX, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up segment descriptor and segment ID.
    //
    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGN(&mut save.SEGID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XBODY = 3;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");
    //
    // Create a segment identifier.
    //
    fstr::assign(&mut save.SEGID, b"SPK type 19 test segment");

    //
    // Set the interval count.
    //
    save.TMPNIV = 2;

    //
    // Set the index of the first packet we'll use.
    //
    save.PKTBEG = 501;

    //
    // PKTOFF is the offset of the first packet to be
    // used. Since PACKTS is a one-dimensional array,
    // we must skip over the total number of addresses
    // occupied by the packets that precede the first
    // of the subset.
    //
    save.PKTOFF = (save.PKTSUM[(save.PKTBEG - 1)] + 1);

    //
    // The first interval in the subset segment is a
    // truncated version of the first interval of the
    // input file. We don't actually lose any intervals
    // in this case. So the interval index doesn't change.
    //
    save.IVBEG = 1;
    //
    // The packet count and time bounds of the first
    // interval do change.
    //
    // The second mini-segment ends one epoch after the
    // last epoch contained in the mini-segment's coverage
    // interval.
    //
    save.RTRUNC = 500;

    //
    // Copy the packet counts.
    //
    spicelib::MOVEI(
        save.NPKTS.as_slice(),
        save.TMPNIV,
        save.TMPNPK.as_slice_mut(),
    );

    //
    // Adjust the first and last packet counts.
    //
    save.TMPNPK[1] = (save.NPKTS[1] - (save.PKTBEG - 1));
    save.TMPNPK[2] = (save.NPKTS[2] - save.RTRUNC);

    //
    // Adjust the interval bounds.
    //
    spicelib::MOVED(
        save.IVLBDS.as_slice(),
        (save.NINTVL + 1),
        save.TMPIVB.as_slice_mut(),
    );

    //
    // Since we have two intervals, the total epoch
    // count is obtained by subtracting the sizes of
    // the initial and final truncated portions from the
    // sum of the sizes of the first two mini-segments.
    //
    save.TMPTOT = (((save.NPKTS[1] + save.NPKTS[2]) - (save.PKTBEG - 1)) - save.RTRUNC);

    save.TMPIVB[1] = save.EPOCHS[save.PKTBEG];
    save.TMPIVB[(save.TMPNIV + 1)] = save.EPOCHS[((save.PKTBEG + save.TMPTOT) - 1)];

    //
    // Inset BEGIN and END by the pad size.
    //
    save.BEGIN = (save.EPOCHS[(save.PKTBEG + save.NPAD)] + 1.0);
    save.END = (save.EPOCHS[(((save.PKTBEG + save.TMPTOT) - 1) - save.NPAD)] - 1.0);

    T_SPKS19(
        save.HANDLE,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.BEGIN,
        save.END,
        TMPSUB,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.BEGIN,
        save.END,
        save.TMPNIV,
        save.TMPNPK.subarray(save.IVBEG),
        save.SUBTPS.subarray(save.IVBEG),
        save.DEGRES.subarray(save.IVBEG),
        save.PACKTS.subarray(save.PKTOFF),
        save.EPOCHS.subarray(save.PKTBEG),
        save.TMPIVB.subarray(save.IVBEG),
        save.SELLST,
        true,
        OK,
        ctx,
    )?;

    //
    // Close the input file from which we created the subset segment.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test the structure of a subsetted segment created from the large SPK file. Create a file with two interpolation interval. Skip over the first 500 packets of the first interval, and truncate the last 500 packets of the second interval. The output segment has no \"middle group.\" Start and stop times coincide with members of the epoch list.", ctx)?;

    //
    // Search the input file for the segment we'll copy from.
    //
    spicelib::DAFOPR(SP19MX, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up segment descriptor and segment ID.
    //
    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGN(&mut save.SEGID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XBODY = 3;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");
    //
    // Create a segment identifier.
    //
    fstr::assign(&mut save.SEGID, b"SPK type 19 test segment");

    //
    // Set the interval count.
    //
    save.TMPNIV = 2;

    //
    // Set the index of the first packet we'll use.
    //
    save.PKTBEG = 501;

    //
    // PKTOFF is the offset of the first packet to be
    // used. Since PACKTS is a one-dimensional array,
    // we must skip over the total number of addresses
    // occupied by the packets that precede the first
    // of the subset.
    //
    save.PKTOFF = (save.PKTSUM[(save.PKTBEG - 1)] + 1);

    //
    // The first interval in the subset segment is a
    // truncated version of the first interval of the
    // input file. We don't actually lose any intervals
    // in this case. So the interval index doesn't change.
    //
    save.IVBEG = 1;
    //
    // The packet count and time bounds of the first
    // interval do change.
    //
    // The second mini-segment ends one epoch after the
    // last epoch contained in the mini-segment's coverage
    // interval.
    //
    save.RTRUNC = 500;

    //
    // Copy the packet counts.
    //
    spicelib::MOVEI(
        save.NPKTS.as_slice(),
        save.TMPNIV,
        save.TMPNPK.as_slice_mut(),
    );

    //
    // Adjust the first and last packet counts.
    //
    save.TMPNPK[1] = (save.NPKTS[1] - (save.PKTBEG - 1));
    save.TMPNPK[2] = (save.NPKTS[2] - save.RTRUNC);

    //
    // Adjust the interval bounds.
    //
    spicelib::MOVED(
        save.IVLBDS.as_slice(),
        (save.NINTVL + 1),
        save.TMPIVB.as_slice_mut(),
    );

    //
    // Since we have only one interval, the total epoch
    // count is obtained by subtracting the sizes of
    // the initial and final truncated portions from the
    // initial size of the first mini-segment.
    //
    save.TMPTOT = (((save.NPKTS[1] + save.NPKTS[2]) - (save.PKTBEG - 1)) - save.RTRUNC);

    save.TMPIVB[1] = save.EPOCHS[save.PKTBEG];
    save.TMPIVB[(save.TMPNIV + 1)] = save.EPOCHS[((save.PKTBEG + save.TMPTOT) - 1)];

    save.BEGIN = save.TMPIVB[1];
    save.END = save.TMPIVB[(save.TMPNIV + 1)];

    //
    // Inset BEGIN and END by the pad size.
    //
    save.BEGIN = save.EPOCHS[(save.PKTBEG + save.NPAD)];
    save.END = save.EPOCHS[(((save.PKTBEG + save.TMPTOT) - 1) - save.NPAD)];

    save.TMPFST = save.BEGIN;
    save.TMPLST = save.END;

    T_SPKS19(
        save.HANDLE,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.BEGIN,
        save.END,
        TMPSUB,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.TMPFST,
        save.TMPLST,
        save.TMPNIV,
        save.TMPNPK.subarray(save.IVBEG),
        save.SUBTPS.subarray(save.IVBEG),
        save.DEGRES.subarray(save.IVBEG),
        save.PACKTS.subarray(save.PKTOFF),
        save.EPOCHS.subarray(save.PKTBEG),
        save.TMPIVB.subarray(save.IVBEG),
        save.SELLST,
        true,
        OK,
        ctx,
    )?;

    //
    // Close the input file from which we created the subset segment.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test the structure of a subsetted segment created from the large SPK file. Skip over the first 500 packets of the first interval, and truncate the last 500 packets of the last. Start and stop times are inset and lie between members of the epoch list.", ctx)?;

    //
    // Search the input file for the segment we'll copy from.
    //
    spicelib::DAFOPR(SP19MX, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up segment descriptor and segment ID.
    //
    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGN(&mut save.SEGID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XBODY = 3;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");
    //
    // Create a segment identifier.
    //
    fstr::assign(&mut save.SEGID, b"SPK type 19 test segment");
    //
    // Set the interval count.
    //
    save.TMPNIV = save.NINTVL;

    //
    // Set the index of the first packet we'll use.
    //
    save.PKTBEG = 501;

    //
    // PKTOFF is the offset of the first packet to be
    // used. Since PACKTS is a one-dimensional array,
    // we must skip over the total number of addresses
    // occupied by the packets that precede the first
    // of the subset.
    //
    save.PKTOFF = (save.PKTSUM[(save.PKTBEG - 1)] + 1);

    //
    // The first interval in the subset segment is a
    // truncated version of the first interval of the
    // input file. We don't actually lose any intervals
    // in this case. So the interval index doesn't change.
    //
    save.IVBEG = 1;
    //
    // The packet count and time bounds of the first
    // interval do change.
    //
    // The first mini-segment ends one epoch after the
    // last epoch contained in the mini-segment's coverage
    // interval.
    //
    save.RTRUNC = 500;

    //
    // Copy the packet counts.
    //
    spicelib::MOVEI(
        save.NPKTS.as_slice(),
        save.TMPNIV,
        save.TMPNPK.as_slice_mut(),
    );

    //
    // Adjust the first and last packet counts.
    //
    save.TMPNPK[1] = (save.NPKTS[1] - (save.PKTBEG - 1));
    save.TMPNPK[save.TMPNIV] = (save.NPKTS[save.TMPNIV] - save.RTRUNC);

    //
    // Adjust the interval bounds.
    //
    spicelib::MOVED(
        save.IVLBDS.as_slice(),
        (save.NINTVL + 1),
        save.TMPIVB.as_slice_mut(),
    );

    save.TMPTOT = ((save.RECTOT - (save.PKTBEG - 1)) - save.RTRUNC);

    save.TMPIVB[1] = save.EPOCHS[save.PKTBEG];
    save.TMPIVB[(save.TMPNIV + 1)] = save.EPOCHS[((save.PKTBEG + save.TMPTOT) - 1)];

    save.BEGIN = (save.EPOCHS[(save.PKTBEG + save.NPAD)] + 1.0);
    save.END = (save.EPOCHS[(((save.PKTBEG + save.TMPTOT) - 1) - save.NPAD)] - 1.0);

    T_SPKS19(
        save.HANDLE,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.BEGIN,
        save.END,
        TMPSUB,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.BEGIN,
        save.END,
        save.TMPNIV,
        save.TMPNPK.subarray(save.IVBEG),
        save.SUBTPS.subarray(save.IVBEG),
        save.DEGRES.subarray(save.IVBEG),
        save.PACKTS.subarray(save.PKTOFF),
        save.EPOCHS.subarray(save.PKTBEG),
        save.TMPIVB.subarray(save.IVBEG),
        save.SELLST,
        true,
        OK,
        ctx,
    )?;

    //
    // Close the input file from which we created the subset segment.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test the structure of a subsetted segment created from the large SPK file. Skip over the first 500 packets of the first interval, and truncate the last 500 packets of the last. Start and stop times coincide with members of the epoch list.", ctx)?;

    //
    // Search the input file for the segment we'll copy from.
    //
    spicelib::DAFOPR(SP19MX, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up segment descriptor and segment ID.
    //
    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGN(&mut save.SEGID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XBODY = 3;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");
    //
    // Create a segment identifier.
    //
    fstr::assign(&mut save.SEGID, b"SPK type 19 test segment");

    //
    // Set the interval count.
    //
    save.TMPNIV = save.NINTVL;

    //
    // Set the index of the first packet we'll use.
    //
    save.PKTBEG = 501;

    //
    // PKTOFF is the offset of the first packet to be
    // used. Since PACKTS is a one-dimensional array,
    // we must skip over the total number of addresses
    // occupied by the packets that precede the first
    // of the subset.
    //
    save.PKTOFF = (save.PKTSUM[(save.PKTBEG - 1)] + 1);

    //
    // The first interval in the subset segment is a
    // truncated version of the first interval of the
    // input file. We don't actually lose any intervals
    // in this case. So the interval index doesn't change.
    //
    save.IVBEG = 1;
    //
    // The packet count and time bounds of the first
    // interval do change.
    //
    // The first mini-segment ends one epoch after the
    // last epoch contained in the mini-segment's coverage
    // interval.
    //
    save.RTRUNC = 500;

    //
    // Copy the packet counts.
    //
    spicelib::MOVEI(
        save.NPKTS.as_slice(),
        save.TMPNIV,
        save.TMPNPK.as_slice_mut(),
    );

    //
    // Adjust the first and last packet counts.
    //
    save.TMPNPK[1] = (save.NPKTS[1] - (save.PKTBEG - 1));
    save.TMPNPK[save.TMPNIV] = (save.NPKTS[save.TMPNIV] - save.RTRUNC);

    //
    // Adjust the interval bounds.
    //
    spicelib::MOVED(
        save.IVLBDS.as_slice(),
        (save.NINTVL + 1),
        save.TMPIVB.as_slice_mut(),
    );

    save.TMPTOT = ((save.RECTOT - (save.PKTBEG - 1)) - save.RTRUNC);

    save.TMPIVB[1] = save.EPOCHS[save.PKTBEG];
    save.TMPIVB[(save.TMPNIV + 1)] = save.EPOCHS[((save.PKTBEG + save.TMPTOT) - 1)];

    save.BEGIN = save.EPOCHS[(save.PKTBEG + save.NPAD)];
    save.END = save.EPOCHS[(((save.PKTBEG + save.TMPTOT) - 1) - save.NPAD)];

    T_SPKS19(
        save.HANDLE,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.BEGIN,
        save.END,
        TMPSUB,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.BEGIN,
        save.END,
        save.TMPNIV,
        save.TMPNPK.subarray(save.IVBEG),
        save.SUBTPS.subarray(save.IVBEG),
        save.DEGRES.subarray(save.IVBEG),
        save.PACKTS.subarray(save.PKTOFF),
        save.EPOCHS.subarray(save.PKTBEG),
        save.TMPIVB.subarray(save.IVBEG),
        save.SELLST,
        true,
        OK,
        ctx,
    )?;

    //
    // Close the input file from which we created the subset segment.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test the structure of a subsetted segment created from the large SPK file. Skip over the first 500 packets of the penultimate interval, and truncate the last 500 packets of the last. Start and stop times are inset and lie between members of the epoch list.", ctx)?;

    //
    // Search the input file for the segment we'll copy from.
    //
    spicelib::DAFOPR(SP19MX, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up segment descriptor and segment ID.
    //
    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGN(&mut save.SEGID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XBODY = 3;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");
    //
    // Create a segment identifier.
    //
    fstr::assign(&mut save.SEGID, b"SPK type 19 test segment");
    //
    // Set the interval count.
    //
    save.TMPNIV = 2;

    //
    // Set the index of the first packet we'll use.
    //
    save.LTRUNC = 500;

    save.PKTBEG = ((((save.RECTOT - save.NPKTS[save.NINTVL]) - save.NPKTS[(save.NINTVL - 1)])
        + save.LTRUNC)
        + 1);

    //
    // The first interval in the subset segment is a
    // truncated version of the penultimate interval of the
    // input file.
    //
    save.IVBEG = (save.NINTVL - 1);

    //
    // PKTOFF is the offset of the first packet to be
    // used. Since PACKTS is a one-dimensional array,
    // we must skip over the total number of addresses
    // occupied by the packets that precede the first
    // of the subset.
    //
    save.PKTOFF = (save.PKTSUM[(save.PKTBEG - 1)] + 1);

    //
    //
    // The second mini-segment ends 500 epochs before
    // the end of the mini-segment's coverage interval.
    //
    save.RTRUNC = 500;

    //
    // Copy the packet counts.
    //
    spicelib::MOVEI(
        save.NPKTS.as_slice(),
        save.NINTVL,
        save.TMPNPK.as_slice_mut(),
    );

    //
    // Adjust the first and last packet counts.
    //
    save.TMPNPK[(save.NINTVL - 1)] = (save.NPKTS[(save.NINTVL - 1)] - save.LTRUNC);
    save.TMPNPK[save.NINTVL] = (save.NPKTS[save.NINTVL] - save.RTRUNC);

    //
    // Adjust the interval bounds.
    //
    spicelib::MOVED(
        save.IVLBDS.as_slice(),
        save.NINTVL,
        save.TMPIVB.as_slice_mut(),
    );

    save.TMPTOT = (save.TMPNPK[(save.NINTVL - 1)] + save.TMPNPK[save.NINTVL]);

    //
    // Note that the first output interval ends at the end
    // time of the corresponding interval of the input segment,
    // so this time is unchanged.
    //
    save.TMPIVB[(save.NINTVL - 1)] = save.EPOCHS[save.PKTBEG];
    save.TMPIVB[(save.NINTVL + 1)] = save.EPOCHS[((save.PKTBEG + save.TMPTOT) - 1)];

    save.BEGIN = (save.EPOCHS[(save.PKTBEG + save.NPAD)] + 1.0);
    save.END = (save.EPOCHS[(((save.PKTBEG + save.TMPTOT) - 1) - save.NPAD)] - 1.0);

    T_SPKS19(
        save.HANDLE,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.BEGIN,
        save.END,
        TMPSUB,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.BEGIN,
        save.END,
        save.TMPNIV,
        save.TMPNPK.subarray(save.IVBEG),
        save.SUBTPS.subarray(save.IVBEG),
        save.DEGRES.subarray(save.IVBEG),
        save.PACKTS.subarray(save.PKTOFF),
        save.EPOCHS.subarray(save.PKTBEG),
        save.TMPIVB.subarray(save.IVBEG),
        save.SELLST,
        true,
        OK,
        ctx,
    )?;

    //
    // Close the input file from which we created the subset segment.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test the structure of a subsetted segment created from the large SPK file. Skip over the first 500 packets of the penultimate interval, and truncate the last 500 packets of the last. Start and stop times coincide with members of the epoch list.", ctx)?;

    //
    // Search the input file for the segment we'll copy from.
    //
    spicelib::DAFOPR(SP19MX, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up segment descriptor and segment ID.
    //
    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGN(&mut save.SEGID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XBODY = 3;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");
    //
    // Create a segment identifier.
    //
    fstr::assign(&mut save.SEGID, b"SPK type 19 test segment");
    //
    // Set the interval count.
    //
    save.TMPNIV = 2;

    //
    // Set the index of the first packet we'll use.
    //
    save.LTRUNC = 500;

    save.PKTBEG = ((((save.RECTOT - save.NPKTS[save.NINTVL]) - save.NPKTS[(save.NINTVL - 1)])
        + save.LTRUNC)
        + 1);

    //
    // PKTOFF is the offset of the first packet to be
    // used. Since PACKTS is a one-dimensional array,
    // we must skip over the total number of addresses
    // occupied by the packets that precede the first
    // of the subset.
    //
    save.PKTOFF = (save.PKTSUM[(save.PKTBEG - 1)] + 1);

    //
    // The first interval in the subset segment is a
    // truncated version of the penultimate interval of the
    // input file.
    //
    save.IVBEG = (save.NINTVL - 1);
    //
    //
    // The second mini-segment ends 500 epochs before
    // the end of the mini-segment's coverage interval.
    //
    save.RTRUNC = 500;

    //
    // Copy the packet counts.
    //
    spicelib::MOVEI(
        save.NPKTS.as_slice(),
        save.NINTVL,
        save.TMPNPK.as_slice_mut(),
    );

    //
    // Adjust the first and last packet counts.
    //
    save.TMPNPK[(save.NINTVL - 1)] = (save.NPKTS[(save.NINTVL - 1)] - save.LTRUNC);
    save.TMPNPK[save.NINTVL] = (save.NPKTS[save.NINTVL] - save.RTRUNC);

    //
    // Adjust the interval bounds.
    //
    spicelib::MOVED(
        save.IVLBDS.as_slice(),
        save.NINTVL,
        save.TMPIVB.as_slice_mut(),
    );

    save.TMPTOT = (save.TMPNPK[(save.NINTVL - 1)] + save.TMPNPK[save.NINTVL]);

    //
    // Note that the first output interval ends at the end
    // time of the corresponding interval of the input segment,
    // so this time is unchanged.
    //
    save.TMPIVB[(save.NINTVL - 1)] = save.EPOCHS[save.PKTBEG];
    save.TMPIVB[(save.NINTVL + 1)] = save.EPOCHS[((save.PKTBEG + save.TMPTOT) - 1)];

    save.BEGIN = save.EPOCHS[(save.PKTBEG + save.NPAD)];
    save.END = save.EPOCHS[(((save.PKTBEG + save.TMPTOT) - 1) - save.NPAD)];

    T_SPKS19(
        save.HANDLE,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.BEGIN,
        save.END,
        TMPSUB,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.BEGIN,
        save.END,
        save.TMPNIV,
        save.TMPNPK.subarray(save.IVBEG),
        save.SUBTPS.subarray(save.IVBEG),
        save.DEGRES.subarray(save.IVBEG),
        save.PACKTS.subarray(save.PKTOFF),
        save.EPOCHS.subarray(save.PKTBEG),
        save.TMPIVB.subarray(save.IVBEG),
        save.SELLST,
        true,
        OK,
        ctx,
    )?;

    //
    // Close the input file from which we created the subset segment.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test the structure of a subsetted segment created from the large SPK file. Skip over the first 500 packets of the last interval, and truncate the last 500 packets of the last. Start and stop times are inset and lie between members of the epoch list.", ctx)?;

    //
    // Search the input file for the segment we'll copy from.
    //
    spicelib::DAFOPR(SP19MX, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up segment descriptor and segment ID.
    //
    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGN(&mut save.SEGID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XBODY = 3;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");
    //
    // Create a segment identifier.
    //
    fstr::assign(&mut save.SEGID, b"SPK type 19 test segment");
    //
    // Set the interval count.
    //
    save.TMPNIV = 1;

    //
    // Set the index of the first packet we'll use.
    //
    save.LTRUNC = 500;

    save.PKTBEG = (((save.RECTOT - save.NPKTS[save.NINTVL]) + save.LTRUNC) + 1);

    //
    // PKTOFF is the offset of the first packet to be
    // used. Since PACKTS is a one-dimensional array,
    // we must skip over the total number of addresses
    // occupied by the packets that precede the first
    // of the subset.
    //
    save.PKTOFF = (save.PKTSUM[(save.PKTBEG - 1)] + 1);

    //
    // The first interval in the subset segment is a
    // truncated version of the penultimate interval of the
    // input file.
    //
    save.IVBEG = save.NINTVL;
    //
    //
    // The second mini-segment ends 500 epochs before
    // the end of the mini-segment's coverage interval.
    //
    save.RTRUNC = 500;

    //
    // Copy the packet counts.
    //
    spicelib::MOVEI(
        save.NPKTS.as_slice(),
        save.NINTVL,
        save.TMPNPK.as_slice_mut(),
    );

    //
    // Adjust the first and last packet counts.
    //
    save.TMPNPK[save.NINTVL] = ((save.NPKTS[save.NINTVL] - save.RTRUNC) - save.LTRUNC);

    //
    // Adjust the interval bounds.
    //
    spicelib::MOVED(
        save.IVLBDS.as_slice(),
        save.NINTVL,
        save.TMPIVB.as_slice_mut(),
    );

    save.TMPTOT = save.TMPNPK[save.NINTVL];

    save.TMPIVB[save.NINTVL] = save.EPOCHS[save.PKTBEG];
    save.TMPIVB[(save.NINTVL + 1)] = save.EPOCHS[((save.PKTBEG + save.TMPTOT) - 1)];

    save.BEGIN = (save.EPOCHS[(save.PKTBEG + save.NPAD)] + 1.0);
    save.END = (save.EPOCHS[(((save.PKTBEG + save.TMPTOT) - 1) - save.NPAD)] - 1.0);

    T_SPKS19(
        save.HANDLE,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.BEGIN,
        save.END,
        TMPSUB,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.BEGIN,
        save.END,
        save.TMPNIV,
        save.TMPNPK.subarray(save.IVBEG),
        save.SUBTPS.subarray(save.IVBEG),
        save.DEGRES.subarray(save.IVBEG),
        save.PACKTS.subarray(save.PKTOFF),
        save.EPOCHS.subarray(save.PKTBEG),
        save.TMPIVB.subarray(save.IVBEG),
        save.SELLST,
        true,
        OK,
        ctx,
    )?;

    //
    // Close the input file from which we created the subset segment.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test the structure of a subsetted segment created from the large SPK file. Skip over the first 500 packets of the last interval, and truncate the last 500 packets of the last. Start and stop times are members of the epoch list.", ctx)?;

    //
    // Search the input file for the segment we'll copy from.
    //
    spicelib::DAFOPR(SP19MX, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up segment descriptor and segment ID.
    //
    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGN(&mut save.SEGID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XBODY = 3;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");
    //
    // Create a segment identifier.
    //
    fstr::assign(&mut save.SEGID, b"SPK type 19 test segment");
    //
    // Set the interval count.
    //
    save.TMPNIV = 1;

    //
    // Set the index of the first packet we'll use.
    //
    save.LTRUNC = 500;

    save.PKTBEG = (((save.RECTOT - save.NPKTS[save.NINTVL]) + save.LTRUNC) + 1);

    //
    // PKTOFF is the offset of the first packet to be
    // used. Since PACKTS is a one-dimensional array,
    // we must skip over the total number of addresses
    // occupied by the packets that precede the first
    // of the subset.
    //
    save.PKTOFF = (save.PKTSUM[(save.PKTBEG - 1)] + 1);

    //
    // The first interval in the subset segment is a
    // truncated version of the penultimate interval of the
    // input file.
    //
    save.IVBEG = save.NINTVL;
    //
    //
    // The second mini-segment ends 500 epochs before
    // the end of the mini-segment's coverage interval.
    //
    save.RTRUNC = 500;

    //
    // Copy the packet counts.
    //
    spicelib::MOVEI(
        save.NPKTS.as_slice(),
        save.NINTVL,
        save.TMPNPK.as_slice_mut(),
    );

    //
    // Adjust the first and last packet counts.
    //
    save.TMPNPK[save.NINTVL] = ((save.NPKTS[save.NINTVL] - save.RTRUNC) - save.LTRUNC);

    //
    // Adjust the interval bounds.
    //
    spicelib::MOVED(
        save.IVLBDS.as_slice(),
        save.NINTVL,
        save.TMPIVB.as_slice_mut(),
    );

    save.TMPTOT = save.TMPNPK[save.NINTVL];

    save.TMPIVB[save.NINTVL] = save.EPOCHS[save.PKTBEG];
    save.TMPIVB[(save.NINTVL + 1)] = save.EPOCHS[((save.PKTBEG + save.TMPTOT) - 1)];

    save.BEGIN = save.EPOCHS[(save.PKTBEG + save.NPAD)];
    save.END = save.EPOCHS[(((save.PKTBEG + save.TMPTOT) - 1) - save.NPAD)];

    T_SPKS19(
        save.HANDLE,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.BEGIN,
        save.END,
        TMPSUB,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.BEGIN,
        save.END,
        save.TMPNIV,
        save.TMPNPK.subarray(save.IVBEG),
        save.SUBTPS.subarray(save.IVBEG),
        save.DEGRES.subarray(save.IVBEG),
        save.PACKTS.subarray(save.PKTOFF),
        save.EPOCHS.subarray(save.PKTBEG),
        save.TMPIVB.subarray(save.IVBEG),
        save.SELLST,
        true,
        OK,
        ctx,
    )?;

    //
    // Close the input file from which we created the subset segment.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test the structure of a subsetted segment created from the large SPK file. Use the entire final mini-segment. Inset the segment start time slightly to prevent addition of a small interval on the left.", ctx)?;

    //
    // Search the input file for the segment we'll copy from.
    //
    spicelib::DAFOPR(SP19MX, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up segment descriptor and segment ID.
    //
    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGN(&mut save.SEGID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XBODY = 3;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");
    //
    // Create a segment identifier.
    //
    fstr::assign(&mut save.SEGID, b"SPK type 19 test segment");
    //
    // Set the interval count.
    //
    save.TMPNIV = 1;

    //
    // Set the index of the first packet we'll use.
    //
    save.PKTBEG = ((save.RECTOT - save.NPKTS[save.NINTVL]) + 1);

    //
    // PKTOFF is the offset of the first packet to be
    // used. Since PACKTS is a one-dimensional array,
    // we must skip over the total number of addresses
    // occupied by the packets that precede the first
    // of the subset.
    //
    save.PKTOFF = (save.PKTSUM[(save.PKTBEG - 1)] + 1);

    //
    // The first interval in the subset segment is a
    // truncated version of the penultimate interval of the
    // input file.
    //
    save.IVBEG = save.NINTVL;

    //
    // Copy the packet counts.
    //
    spicelib::MOVEI(
        save.NPKTS.as_slice(),
        save.NINTVL,
        save.TMPNPK.as_slice_mut(),
    );

    //
    // Copy the interval bounds.
    //
    spicelib::MOVED(
        save.IVLBDS.as_slice(),
        (save.NINTVL + 1),
        save.TMPIVB.as_slice_mut(),
    );

    save.TMPTOT = save.TMPNPK[save.NINTVL];

    save.BEGIN = (save.EPOCHS[(save.PKTBEG + save.NPAD)] + 1.0);
    save.END = save.EPOCHS[(((save.PKTBEG + save.TMPTOT) - 1) - save.NPAD)];

    T_SPKS19(
        save.HANDLE,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.BEGIN,
        save.END,
        TMPSUB,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.BEGIN,
        save.END,
        save.TMPNIV,
        save.TMPNPK.subarray(save.IVBEG),
        save.SUBTPS.subarray(save.IVBEG),
        save.DEGRES.subarray(save.IVBEG),
        save.PACKTS.subarray(save.PKTOFF),
        save.EPOCHS.subarray(save.PKTBEG),
        save.TMPIVB.subarray(save.IVBEG),
        save.SELLST,
        true,
        OK,
        ctx,
    )?;

    //
    // Close the input file from which we created the subset segment.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test the structure of a subsetted segment created from the large SPK file. Use the entire final mini-segment.", ctx)?;

    //
    // Search the input file for the segment we'll copy from.
    //
    spicelib::DAFOPR(SP19MX, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up segment descriptor and segment ID.
    //
    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGN(&mut save.SEGID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XBODY = 3;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");
    //
    // Create a segment identifier.
    //
    fstr::assign(&mut save.SEGID, b"SPK type 19 test segment");
    //
    // Set the interval count. Note that a zero-length
    // interval will be added on the left side of the
    // output segment to accommodate correct interpolation
    // when the request time is at the start of the
    // final interval and the interval selection method
    // is "select first."
    //
    save.TMPNIV = 2;

    //
    // Copy and adjust the packet counts.
    //
    spicelib::MOVEI(
        save.NPKTS.as_slice(),
        save.NINTVL,
        save.TMPNPK.as_slice_mut(),
    );

    save.TMPNPK[(save.NINTVL - 1)] = ((2 * save.NPAD) + 1);

    //
    // Set the index of the first packet we'll use.
    //
    save.PKTBEG = (((save.RECTOT - save.TMPNPK[save.NINTVL]) - save.TMPNPK[(save.NINTVL - 1)]) + 1);

    //
    // PKTOFF is the offset of the first packet to be
    // used. Since PACKTS is a one-dimensional array,
    // we must skip over the total number of addresses
    // occupied by the packets that precede the first
    // of the subset.
    //
    save.PKTOFF = (save.PKTSUM[(save.PKTBEG - 1)] + 1);

    //
    // The first interval in the subset segment is a
    // truncated version of the penultimate interval of the
    // input file.
    //
    save.IVBEG = (save.NINTVL - 1);

    //
    // Copy and adjust the interval bounds.
    //
    spicelib::MOVED(
        save.IVLBDS.as_slice(),
        (save.NINTVL + 1),
        save.TMPIVB.as_slice_mut(),
    );

    save.TMPIVB[(save.NINTVL - 1)] = save.EPOCHS[save.PKTBEG];

    save.TMPTOT = (save.TMPNPK[save.NINTVL] + save.TMPNPK[(save.NINTVL - 1)]);

    save.BEGIN = save.IVLBDS[save.NINTVL];
    save.END = save.IVLBDS[(save.NINTVL + 1)];

    T_SPKS19(
        save.HANDLE,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.BEGIN,
        save.END,
        TMPSUB,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.BEGIN,
        save.END,
        save.TMPNIV,
        save.TMPNPK.subarray(save.IVBEG),
        save.SUBTPS.subarray(save.IVBEG),
        save.DEGRES.subarray(save.IVBEG),
        save.PACKTS.subarray(save.PKTOFF),
        save.EPOCHS.subarray(save.PKTBEG),
        save.TMPIVB.subarray(save.IVBEG),
        save.SELLST,
        true,
        OK,
        ctx,
    )?;

    //
    // Close the input file from which we created the subset segment.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up:  delete SPK files.", ctx)?;

    if spicelib::EXISTS(SPK19E, ctx)? {
        spicelib::DELFIL(SPK19E, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    if spicelib::EXISTS(SP19MX, ctx)? {
        spicelib::DELFIL(SP19MX, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    if spicelib::EXISTS(SP19T1, ctx)? {
        spicelib::DELFIL(SP19T1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    if spicelib::EXISTS(SUB19, ctx)? {
        spicelib::DELFIL(SUB19, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
