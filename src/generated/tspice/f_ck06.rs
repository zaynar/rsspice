//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXDEG: i32 = 23;
const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const C06TP0: i32 = 0;
const C06TP1: i32 = (C06TP0 + 1);
const C06TP2: i32 = (C06TP1 + 1);
const C06TP3: i32 = (C06TP2 + 1);
const C06NST: i32 = 4;
const C06PS0: i32 = 8;
const C06PS1: i32 = 4;
const C06PS2: i32 = 14;
const C06PS3: i32 = 7;
const C06MXZ: i32 = C06PS2;
const C06MNZ: i32 = C06PS1;
const MAXRSZ: i32 = (4 + ((MAXDEG + 1) * (C06PS3 + 1)));
const CK06E: &[u8] = b"test06err.bc";
const CK06T0: &[u8] = b"ck06t0.bc";
const CK06T1: &[u8] = b"ck06t1.bc";
const CK06MU: &[u8] = b"ck06mult.bc";
const CK06B0: &[u8] = b"ck06big0.bc";
const CK06B1: &[u8] = b"ck06big1.bc";
const CK06B2: &[u8] = b"ck06big2.bc";
const VTIGHT: f64 = 0.00000000000001;
const TIGHT: f64 = 0.0000000001;
const MTIGHT: f64 = 0.00000001;
const MED: f64 = 0.000001;
const FILSIZ: i32 = 255;
const MAXIVL: i32 = 21000;
const BIGN: i32 = (MAXIVL * 40);
const DSCSIZ: i32 = 5;
const LNSIZE: i32 = 80;
const MSGLEN: i32 = 800;
const ND: i32 = 2;
const NI: i32 = 6;
const SIDLEN: i32 = 60;
const FRNMLN: i32 = 32;
const RECSIZ: i32 = (MAXRSZ + 4);
const MAXSEG: i32 = 3;

struct SaveVars {
    CK: Vec<u8>,
    CKFRAM: Vec<u8>,
    NAME: Vec<u8>,
    SEGID: Vec<u8>,
    TITLE: Vec<u8>,
    XREF: Vec<u8>,
    XREFS: ActualCharArray,
    AV: StackArray<f64, 3>,
    AVBUFF: ActualArray2D<f64>,
    AVQ: StackArray<f64, 4>,
    AVVS: ActualArray2D<f64>,
    CKTOL: f64,
    CLKOUT: f64,
    CMAT: StackArray2D<f64, 9>,
    DAVV: StackArray<f64, 3>,
    DC: StackArray<f64, 2>,
    DELTA: f64,
    DESCR: StackArray<f64, 5>,
    EPOCHS: ActualArray<f64>,
    ET: f64,
    FIRST: f64,
    IVLBDS: ActualArray<f64>,
    LAST: f64,
    LSTEPC: f64,
    PACKTS: ActualArray<f64>,
    QNEG: StackArray<f64, 4>,
    QUATS: ActualArray2D<f64>,
    RATE: f64,
    RATES: ActualArray<f64>,
    RECORD: StackArray<f64, 200>,
    SCLKDP: ActualArray<f64>,
    T: f64,
    T0: f64,
    T1: f64,
    TOL: f64,
    XAV: StackArray<f64, 3>,
    XAVVS: ActualArray2D<f64>,
    XCMAT: StackArray2D<f64, 9>,
    XCMATS: ActualArray2D<f64>,
    XDQ: StackArray<f64, 4>,
    XEPOCH: ActualArray<f64>,
    XFORM: StackArray2D<f64, 36>,
    XIVBDS: ActualArray<f64>,
    XQ: StackArray<f64, 4>,
    XQUATS: ActualArray2D<f64>,
    XREC: StackArray<f64, 200>,
    XT: f64,
    BEGREC: i32,
    DEGRES: ActualArray<i32>,
    DATIDX: ActualArray<i32>,
    HANDLE: i32,
    I: i32,
    IC: StackArray<i32, 6>,
    J: i32,
    JMAX: i32,
    K: i32,
    M: i32,
    MNSGNO: i32,
    N: i32,
    NINTVL: i32,
    NIVALS: StackArray<i32, 3>,
    NMINI: i32,
    NPKTS: ActualArray<i32>,
    NREC: i32,
    NSEG: i32,
    PAD: i32,
    PSIZES: ActualArray<i32>,
    PKTBEG: ActualArray<i32>,
    PKTPTR: i32,
    PKTSIZ: i32,
    PKTSZS: StackArray<i32, 4>,
    PKTTOT: i32,
    RECTOT: i32,
    SEGNO: i32,
    SEGPTR: i32,
    SIZE: i32,
    SKIP: i32,
    SUBTPS: ActualArray<i32>,
    SUBTYP: i32,
    XINST: i32,
    XINSTS: StackArray<i32, 3>,
    WINSIZ: i32,
    XWNSIZ: i32,
    FOUND: bool,
    SELLST: bool,
    XAVFLG: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CK = vec![b' '; FILSIZ as usize];
        let mut CKFRAM = vec![b' '; FRNMLN as usize];
        let mut NAME = vec![b' '; LNSIZE as usize];
        let mut SEGID = vec![b' '; SIDLEN as usize];
        let mut TITLE = vec![b' '; MSGLEN as usize];
        let mut XREF = vec![b' '; FRNMLN as usize];
        let mut XREFS = ActualCharArray::new(FRNMLN, 1..=MAXSEG);
        let mut AV = StackArray::<f64, 3>::new(1..=3);
        let mut AVBUFF = ActualArray2D::<f64>::new(1..=3, 1..=BIGN);
        let mut AVQ = StackArray::<f64, 4>::new(0..=3);
        let mut AVVS = ActualArray2D::<f64>::new(1..=3, 1..=BIGN);
        let mut CKTOL: f64 = 0.0;
        let mut CLKOUT: f64 = 0.0;
        let mut CMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut DAVV = StackArray::<f64, 3>::new(1..=3);
        let mut DC = StackArray::<f64, 2>::new(1..=ND);
        let mut DELTA: f64 = 0.0;
        let mut DESCR = StackArray::<f64, 5>::new(1..=DSCSIZ);
        let mut EPOCHS = ActualArray::<f64>::new(1..=BIGN);
        let mut ET: f64 = 0.0;
        let mut FIRST: f64 = 0.0;
        let mut IVLBDS = ActualArray::<f64>::new(1..=(MAXIVL + 1));
        let mut LAST: f64 = 0.0;
        let mut LSTEPC: f64 = 0.0;
        let mut PACKTS = ActualArray::<f64>::new(1..=(BIGN * C06PS0));
        let mut QNEG = StackArray::<f64, 4>::new(0..=3);
        let mut QUATS = ActualArray2D::<f64>::new(0..=3, 1..=BIGN);
        let mut RATE: f64 = 0.0;
        let mut RATES = ActualArray::<f64>::new(1..=MAXIVL);
        let mut RECORD = StackArray::<f64, 200>::new(1..=RECSIZ);
        let mut SCLKDP = ActualArray::<f64>::new(1..=BIGN);
        let mut T: f64 = 0.0;
        let mut T0: f64 = 0.0;
        let mut T1: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut XAV = StackArray::<f64, 3>::new(1..=3);
        let mut XAVVS = ActualArray2D::<f64>::new(1..=3, 1..=BIGN);
        let mut XCMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut XCMATS = ActualArray2D::<f64>::new(1..=9, 1..=BIGN);
        let mut XDQ = StackArray::<f64, 4>::new(0..=3);
        let mut XEPOCH = ActualArray::<f64>::new(1..=BIGN);
        let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
        let mut XIVBDS = ActualArray::<f64>::new(1..=(MAXIVL + 1));
        let mut XQ = StackArray::<f64, 4>::new(0..=3);
        let mut XQUATS = ActualArray2D::<f64>::new(0..=3, 1..=BIGN);
        let mut XREC = StackArray::<f64, 200>::new(1..=RECSIZ);
        let mut XT: f64 = 0.0;
        let mut BEGREC: i32 = 0;
        let mut DEGRES = ActualArray::<i32>::new(1..=MAXIVL);
        let mut DATIDX = ActualArray::<i32>::new(1..=MAXIVL);
        let mut HANDLE: i32 = 0;
        let mut I: i32 = 0;
        let mut IC = StackArray::<i32, 6>::new(1..=NI);
        let mut J: i32 = 0;
        let mut JMAX: i32 = 0;
        let mut K: i32 = 0;
        let mut M: i32 = 0;
        let mut MNSGNO: i32 = 0;
        let mut N: i32 = 0;
        let mut NINTVL: i32 = 0;
        let mut NIVALS = StackArray::<i32, 3>::new(1..=MAXSEG);
        let mut NMINI: i32 = 0;
        let mut NPKTS = ActualArray::<i32>::new(1..=MAXIVL);
        let mut NREC: i32 = 0;
        let mut NSEG: i32 = 0;
        let mut PAD: i32 = 0;
        let mut PSIZES = ActualArray::<i32>::new(1..=MAXIVL);
        let mut PKTBEG = ActualArray::<i32>::new(1..=MAXIVL);
        let mut PKTPTR: i32 = 0;
        let mut PKTSIZ: i32 = 0;
        let mut PKTSZS = StackArray::<i32, 4>::new(0..=3);
        let mut PKTTOT: i32 = 0;
        let mut RECTOT: i32 = 0;
        let mut SEGNO: i32 = 0;
        let mut SEGPTR: i32 = 0;
        let mut SIZE: i32 = 0;
        let mut SKIP: i32 = 0;
        let mut SUBTPS = ActualArray::<i32>::new(1..=MAXIVL);
        let mut SUBTYP: i32 = 0;
        let mut XINST: i32 = 0;
        let mut XINSTS = StackArray::<i32, 3>::new(1..=MAXSEG);
        let mut WINSIZ: i32 = 0;
        let mut XWNSIZ: i32 = 0;
        let mut FOUND: bool = false;
        let mut SELLST: bool = false;
        let mut XAVFLG: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(C06PS0),
                Val::I(C06PS1),
                Val::I(C06PS2),
                Val::I(C06PS3),
            ]
            .into_iter();
            PKTSZS
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            CK,
            CKFRAM,
            NAME,
            SEGID,
            TITLE,
            XREF,
            XREFS,
            AV,
            AVBUFF,
            AVQ,
            AVVS,
            CKTOL,
            CLKOUT,
            CMAT,
            DAVV,
            DC,
            DELTA,
            DESCR,
            EPOCHS,
            ET,
            FIRST,
            IVLBDS,
            LAST,
            LSTEPC,
            PACKTS,
            QNEG,
            QUATS,
            RATE,
            RATES,
            RECORD,
            SCLKDP,
            T,
            T0,
            T1,
            TOL,
            XAV,
            XAVVS,
            XCMAT,
            XCMATS,
            XDQ,
            XEPOCH,
            XFORM,
            XIVBDS,
            XQ,
            XQUATS,
            XREC,
            XT,
            BEGREC,
            DEGRES,
            DATIDX,
            HANDLE,
            I,
            IC,
            J,
            JMAX,
            K,
            M,
            MNSGNO,
            N,
            NINTVL,
            NIVALS,
            NMINI,
            NPKTS,
            NREC,
            NSEG,
            PAD,
            PSIZES,
            PKTBEG,
            PKTPTR,
            PKTSIZ,
            PKTSZS,
            PKTTOT,
            RECTOT,
            SEGNO,
            SEGPTR,
            SIZE,
            SKIP,
            SUBTPS,
            SUBTYP,
            XINST,
            XINSTS,
            WINSIZ,
            XWNSIZ,
            FOUND,
            SELLST,
            XAVFLG,
        }
    }
}

//$Procedure F_CK06 ( CK data type 06 tests )
pub fn F_CK06(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    // Open the test family.
    //
    testutil::TOPEN(b"F_CK06", ctx)?;

    //*****************************************************************
    //*
    //*    CKW06 error cases:
    //*
    //*****************************************************************

    //
    // Test CKW06:  start out with error handling.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"CKW06 setup", ctx)?;

    //
    // Initialize the inputs to CKW06.
    //
    save.XINST = 3;
    save.XAVFLG = true;
    fstr::assign(&mut save.XREF, b"J2000");
    //
    // Create a segment identifier.
    //
    fstr::assign(&mut save.SEGID, b"CK type 06 test segment");

    //
    // Open a new CK file.
    //
    if spicelib::EXISTS(CK06E, ctx)? {
        spicelib::DELFIL(CK06E, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::CKOPN(CK06E, b"Type 06 CK error file", 4, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NINTVL = 1;
    save.NPKTS[1] = 2;
    save.RATES[1] = 1.0;
    save.SUBTPS[1] = C06TP1;
    save.DEGRES[1] = 1;

    save.RECTOT = 0;
    save.PKTTOT = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NINTVL;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Update total counts of records and packet elements.
            // The latter depends on the packet size of the current
            // mini-segment.
            //
            if (save.SUBTPS[save.I] == C06TP0) {
                save.PKTSIZ = C06PS0;
            } else if (save.SUBTPS[save.I] == C06TP1) {
                save.PKTSIZ = C06PS1;
            } else {
                spicelib::SETMSG(
                    b"Unexpected CK type 06 subtype # found in mini-segment #.",
                    ctx,
                );
                spicelib::ERRINT(b"#", save.SUBTPS[save.I], ctx);
                spicelib::ERRINT(b"#", save.I, ctx);
                spicelib::SIGERR(b"SPICE(INVALIDSUBTYPE)", ctx)?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;
            }

            save.PKTTOT = (save.PKTTOT + (save.NPKTS[save.I] * save.PKTSIZ));

            save.RECTOT = (save.RECTOT + save.NPKTS[save.I]);

            save.I += m3__;
        }
    }

    spicelib::FILLD(0.5, save.PKTTOT, save.PACKTS.as_slice_mut());

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.RECTOT;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.SCLKDP[save.I] = (save.I as f64);
            save.I += m3__;
        }
    }

    save.FIRST = save.SCLKDP[1];
    save.LAST = save.SCLKDP[2];

    save.IVLBDS[1] = save.FIRST;
    save.IVLBDS[2] = save.LAST;

    save.SELLST = true;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"CKW06 error case: descriptor times swapped.", ctx)?;

    spicelib::CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.LAST,
        save.FIRST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADDESCRTIMES)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"CKW06 error case: bad frame name.", ctx)?;

    spicelib::CKW06(
        save.HANDLE,
        save.XINST,
        b"SPUD",
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDREFFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"CKW06 error case: SEGID too long.", ctx)?;

    spicelib::CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        b"X                                               X",
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SEGIDTOOLONG)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"CKW06 error case: unprintable SEGID characters.", ctx)?;

    spicelib::CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &intrinsics::CHAR(7),
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NONPRINTABLECHARS)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"CKW06 error case: non-positive mini-segment count", ctx)?;

    spicelib::CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        0,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDCOUNT)", OK, ctx)?;

    save.NINTVL = 1;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"CKW06 error case: non-increasing mini-segment bounds", ctx)?;

    save.IVLBDS[2] = save.IVLBDS[1];
    save.FIRST = 1.5;
    save.LAST = 1.6;

    spicelib::CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
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
    testutil::TCASE(
        b"CKW06 error case: descriptor  start time is too early.",
        ctx,
    )?;

    spicelib::CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        (save.FIRST - 1.0),
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(COVERAGEGAP)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"CKW06 error case: descriptor end time is too late.", ctx)?;

    spicelib::CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        (save.LAST + 1.0),
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(COVERAGEGAP)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"CKW06 error case: too few packets", ctx)?;

    save.NPKTS[1] = 1;

    spicelib::CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(TOOFEWPACKETS)", OK, ctx)?;

    save.NPKTS[1] = 2;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"CKW06 error case: invalid SCLK rate.", ctx)?;

    save.RATES[1] = 0.0;

    spicelib::CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDSCLKRATE)", OK, ctx)?;

    save.RATES[1] = -1.0;

    spicelib::CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDSCLKRATE)", OK, ctx)?;

    save.RATES[1] = 1.0;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"CKW06 error case: invalid subtype.", ctx)?;

    save.SUBTPS[1] = -1;

    spicelib::CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDSUBTYPE)", OK, ctx)?;

    save.SUBTPS[1] = C06TP1;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"CKW06 error case: polynomial degree too high.", ctx)?;

    save.DEGRES[1] = 40;

    spicelib::CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDEGREE)", OK, ctx)?;

    save.DEGRES[1] = 1;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"CKW06 error case: polynomial degree too low.", ctx)?;

    save.DEGRES[1] = 0;

    spicelib::CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDEGREE)", OK, ctx)?;

    save.DEGRES[1] = 1;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"CKW06 error case:  odd window size.", ctx)?;

    save.DEGRES[1] = 2;

    spicelib::CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADWINDOWSIZE)", OK, ctx)?;

    save.DEGRES[1] = 1;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"CKW06 error case: epochs for first mini-segment are out of order.",
        ctx,
    )?;

    save.NINTVL = 1;
    save.NPKTS[1] = 4;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NPKTS[1];
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.SCLKDP[save.I] = (save.I as f64);
            save.I += m3__;
        }
    }

    save.FIRST = save.SCLKDP[1];
    save.LAST = save.SCLKDP[save.NPKTS[1]];

    save.IVLBDS[1] = save.FIRST;
    save.IVLBDS[2] = save.LAST;

    spicelib::SWAPD_ARRAY(
        save.SCLKDP.subscript(2),
        save.SCLKDP.subscript(3),
        save.SCLKDP.as_slice_mut(),
    );

    spicelib::CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(TIMESOUTOFORDER)", OK, ctx)?;

    spicelib::SWAPD_ARRAY(
        save.SCLKDP.subscript(2),
        save.SCLKDP.subscript(3),
        save.SCLKDP.as_slice_mut(),
    );

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"CKW06 error case: first mini-segment epoch follows interval start.",
        ctx,
    )?;

    save.SCLKDP[1] = (save.IVLBDS[1] + 0.1);

    spicelib::CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BOUNDSDISAGREE)", OK, ctx)?;

    save.SCLKDP[1] = 1.0;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"CKW06 error case: last mini-segment epoch precedes interval start.",
        ctx,
    )?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NPKTS[1];
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.SCLKDP[save.I] = (((save.I as f64) - save.NPKTS[1] as f64) - 1.0);
            save.I += m3__;
        }
    }

    spicelib::CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BOUNDSDISAGREE)", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NPKTS[1];
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.SCLKDP[save.I] = (save.I as f64);
            save.I += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"CKW06 *non* error case: epochs for first mini-segment don\'t cover first mini-segment interval. A gap exists at the end of the first mini-segment.", ctx)?;

    save.NINTVL = 1;
    save.NPKTS[1] = 4;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NPKTS[1];
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.SCLKDP[save.I] = (save.I as f64);
            save.I += m3__;
        }
    }

    save.FIRST = save.SCLKDP[1];
    save.LAST = save.SCLKDP[save.NPKTS[1]];

    save.IVLBDS[1] = save.FIRST;
    save.IVLBDS[2] = save.LAST;

    save.SCLKDP[4] = (save.SCLKDP[4] - 0.5);

    save.PKTTOT = (C06PS1 * save.NPKTS[1]);

    spicelib::FILLD(0.5, save.PKTTOT, save.PACKTS.as_slice_mut());

    spicelib::CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NPKTS[1];
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.SCLKDP[save.I] = (save.I as f64);
            save.I += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"CKW06 error case: zero quaternion.", ctx)?;

    spicelib::FILLD(0.0, 4, save.PACKTS.as_slice_mut());

    spicelib::CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.IVLBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(ZEROQUATERNION)", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NPKTS[1];
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.SCLKDP[save.I] = (save.I as f64);
            save.I += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"CKW06 error case: quaternion sign error for subtype 0.",
        ctx,
    )?;

    save.SEGNO = 1;
    save.MNSGNO = 1;
    save.BEGREC = 1;
    save.N = 201;
    fstr::assign(&mut save.XREF, b"MARSIAU");

    save.XINST = -1000;
    save.XAVFLG = true;

    save.NINTVL = 1;
    save.NPKTS[1] = save.N;

    save.RATES[1] = 0.001;
    save.RATE = save.RATES[1];

    save.SUBTPS[1] = C06TP0;
    save.SUBTYP = save.SUBTPS[1];

    save.PKTSIZ = C06PS0;

    save.DEGRES[1] = 3;

    T_GENCSM(
        save.SEGNO,
        save.MNSGNO,
        save.BEGREC,
        save.NPKTS[save.MNSGNO],
        &save.XREF,
        save.RATE,
        save.SUBTYP,
        save.SCLKDP.as_slice_mut(),
        save.QUATS.as_slice_mut(),
        save.AVBUFF.as_slice_mut(),
        OK,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XIVBDS[1] = save.SCLKDP[1];
    save.XIVBDS[2] = save.SCLKDP[save.N];

    save.FIRST = save.XIVBDS[1];
    save.LAST = save.XIVBDS[2];

    save.PKTPTR = 1;

    spicelib::CLEARD(3, save.DAVV.as_slice_mut());

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.PKTPTR = (1 + ((save.I - 1) * save.PKTSIZ));

            T_XSUBTP(
                save.QUATS.subarray([0, save.I]),
                save.AVBUFF.subarray([1, save.I]),
                save.DAVV.as_slice(),
                save.SUBTYP,
                save.PKTSIZ,
                save.PACKTS.subarray_mut(save.PKTPTR),
                ctx,
            )?;

            //
            // Create error in quaternion sequence at packet index 5:
            //
            if (save.I == 5) {
                spicelib::VMINUG(
                    save.PACKTS.subarray(save.PKTPTR),
                    4,
                    save.QNEG.as_slice_mut(),
                );
                spicelib::MOVED(
                    save.QNEG.as_slice(),
                    4,
                    save.PACKTS.subarray_mut(save.PKTPTR),
                );
            }

            save.I += m3__;
        }
    }

    fstr::assign(&mut save.SEGID, b"Subtype 0 test segment");
    save.SELLST = true;

    spicelib::CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.XIVBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADQUATSIGN)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"CKW06 error case: quaternion sign error for subtype 2.",
        ctx,
    )?;

    //
    // Use the inputs from the previous case, where possible.
    //
    save.SUBTPS[1] = C06TP2;
    save.PKTSIZ = C06PS2;

    save.PKTPTR = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.PKTPTR = (1 + ((save.I - 1) * save.PKTSIZ));

            T_XSUBTP(
                save.QUATS.subarray([0, save.I]),
                save.AVBUFF.subarray([1, save.I]),
                save.DAVV.as_slice(),
                save.SUBTYP,
                save.PKTSIZ,
                save.PACKTS.subarray_mut(save.PKTPTR),
                ctx,
            )?;

            //
            // Create error in quaternion sequence at packet index 6:
            //
            if (save.I == 6) {
                spicelib::VMINUG(
                    save.PACKTS.subarray(save.PKTPTR),
                    4,
                    save.QNEG.as_slice_mut(),
                );
                spicelib::MOVED(
                    save.QNEG.as_slice(),
                    4,
                    save.PACKTS.subarray_mut(save.PKTPTR),
                );
            }

            save.I += m3__;
        }
    }

    spicelib::CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.XIVBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADQUATSIGN)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"CKW06 non-error case: quaternion negation for subtype 1. Make sure no error is signaled.",
        ctx,
    )?;

    //
    // Use the inputs from the previous case, where possible.
    //
    save.SUBTPS[1] = C06TP1;
    save.SUBTYP = save.SUBTPS[1];
    save.PKTSIZ = C06PS1;

    save.PKTPTR = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.PKTPTR = (1 + ((save.I - 1) * save.PKTSIZ));

            T_XSUBTP(
                save.QUATS.subarray([0, save.I]),
                save.AVBUFF.subarray([1, save.I]),
                save.DAVV.as_slice(),
                save.SUBTYP,
                save.PKTSIZ,
                save.PACKTS.subarray_mut(save.PKTPTR),
                ctx,
            )?;

            //
            // Create error in quaternion sequence at packet index 7:
            //
            if (save.I == 7) {
                spicelib::VMINUG(
                    save.PACKTS.subarray(save.PKTPTR),
                    4,
                    save.QNEG.as_slice_mut(),
                );
                spicelib::MOVED(
                    save.QNEG.as_slice(),
                    4,
                    save.PACKTS.subarray_mut(save.PKTPTR),
                );
            }

            save.I += m3__;
        }
    }

    spicelib::CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.XIVBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"CKW06 non-error case: quaternion negation for subtype 3. Make sure no error is signaled.",
        ctx,
    )?;

    //
    // Use the inputs from the previous case, where possible.
    //
    save.SUBTPS[1] = C06TP3;
    save.SUBTYP = save.SUBTPS[1];
    save.PKTSIZ = C06PS3;

    save.PKTPTR = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.PKTPTR = (1 + ((save.I - 1) * save.PKTSIZ));

            T_XSUBTP(
                save.QUATS.subarray([0, save.I]),
                save.AVBUFF.subarray([1, save.I]),
                save.DAVV.as_slice(),
                save.SUBTYP,
                save.PKTSIZ,
                save.PACKTS.subarray_mut(save.PKTPTR),
                ctx,
            )?;

            //
            // Create error in quaternion sequence at packet index 8:
            //
            if (save.I == 8) {
                spicelib::VMINUG(
                    save.PACKTS.subarray(save.PKTPTR),
                    4,
                    save.QNEG.as_slice_mut(),
                );
                spicelib::MOVED(
                    save.QNEG.as_slice(),
                    4,
                    save.PACKTS.subarray_mut(save.PKTPTR),
                );
            }

            save.I += m3__;
        }
    }

    spicelib::CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.XIVBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*****************************************************************
    //*
    //*    End of CKW06 error cases:
    //*
    //*****************************************************************

    //
    // Close the CK file at the DAF level; CKCLS won't close
    // a file without segments.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*****************************************************************
    //*
    //*     CKW06 Non-error cases:
    //*
    //*****************************************************************

    //*****************************************************************
    //*
    //*    Trivial case: write an CK containing a small subtype 1
    //*    segment.
    //*
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test CKW06: create trivial subtype 1 segment.", ctx)?;

    save.SEGNO = 1;
    save.MNSGNO = 1;
    save.BEGREC = 1;
    save.N = 201;
    fstr::assign(&mut save.XREF, b"MARSIAU");

    T_GENC06(
        save.SEGNO,
        save.MNSGNO,
        save.BEGREC,
        save.N,
        &save.XREF,
        1.0,
        save.SCLKDP.as_slice_mut(),
        save.PACKTS.as_slice_mut(),
        save.AVVS.as_slice_mut(),
        OK,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XINST = -1000;
    save.XAVFLG = true;

    save.NINTVL = 1;
    save.NPKTS[1] = save.N;
    save.RATES[1] = 0.001;
    save.SUBTPS[1] = C06TP1;
    save.DEGRES[1] = 3;

    save.XIVBDS[1] = save.SCLKDP[1];
    save.XIVBDS[2] = save.SCLKDP[save.N];

    save.FIRST = save.XIVBDS[1];
    save.LAST = save.XIVBDS[2];

    fstr::assign(&mut save.SEGID, b"Subtype 1 test segment");

    save.SELLST = true;

    //
    // Open new C-kernel.
    //
    if spicelib::EXISTS(CK06T1, ctx)? {
        spicelib::DELFIL(CK06T1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::CKOPN(CK06T1, CK06T1, 0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Write segment.
    //
    spicelib::CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.XIVBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    //
    // Close kernel.
    //
    spicelib::CKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Read data records from trivial subtype 1 segment", ctx)?;

    spicelib::DAFOPR(CK06T1, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get mini-segment count.
    //
    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CKNM06(save.HANDLE, save.DESCR.as_slice(), &mut save.NMINI, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CKMP06(
        save.HANDLE,
        save.DESCR.as_slice(),
        1,
        &mut save.RATE,
        &mut save.SUBTYP,
        &mut save.WINSIZ,
        &mut save.NREC,
        save.IVLBDS.as_slice_mut(),
        &mut save.LSTEPC,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.PKTSIZ = C06PS1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NREC;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::CKGR06(
                save.HANDLE,
                save.DESCR.as_slice(),
                1,
                save.I,
                save.RECORD.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // Check the record. The time tag is the first element.
            // We expect exact matches for all elements.
            //
            fstr::assign(&mut save.NAME, b"Time tag *");
            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.I, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSD(
                &save.NAME,
                save.RECORD[1],
                b"=",
                save.SCLKDP[save.I],
                0.0,
                OK,
                ctx,
            )?;

            fstr::assign(&mut save.NAME, b"Subtype code *");
            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.I, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSD(
                &save.NAME,
                save.RECORD[2],
                b"=",
                (save.SUBTPS[1] as f64),
                0.0,
                OK,
                ctx,
            )?;

            fstr::assign(&mut save.NAME, b"Clock rate *");
            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.I, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSD(
                &save.NAME,
                save.RECORD[3],
                b"=",
                save.RATES[1],
                0.0,
                OK,
                ctx,
            )?;

            //
            // Check the record data.
            //
            fstr::assign(&mut save.NAME, b"Quaternion *");
            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.I, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.K = ((save.PKTSIZ * (save.I - 1)) + 1);

            testutil::CHCKAD(
                &save.NAME,
                save.RECORD.subarray(4),
                b"=",
                save.PACKTS.subarray(save.K),
                save.PKTSIZ,
                0.0,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // Close kernel.
    //
    spicelib::CKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Read data records from trivial subtype 1 segment. Open CK for write access but don\'t modify the file.", ctx)?;

    spicelib::DAFOPW(CK06T1, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get mini-segment count.
    //
    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CKNM06(save.HANDLE, save.DESCR.as_slice(), &mut save.NMINI, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CKMP06(
        save.HANDLE,
        save.DESCR.as_slice(),
        1,
        &mut save.RATE,
        &mut save.SUBTYP,
        &mut save.WINSIZ,
        &mut save.NREC,
        save.IVLBDS.as_slice_mut(),
        &mut save.LSTEPC,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.PKTSIZ = C06PS1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NREC;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::CKGR06(
                save.HANDLE,
                save.DESCR.as_slice(),
                1,
                save.I,
                save.RECORD.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // Check the record. The time tag is the first element.
            // We expect exact matches for all elements.
            //
            fstr::assign(&mut save.NAME, b"Time tag *");
            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.I, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSD(
                &save.NAME,
                save.RECORD[1],
                b"=",
                save.SCLKDP[save.I],
                0.0,
                OK,
                ctx,
            )?;

            fstr::assign(&mut save.NAME, b"Subtype code *");
            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.I, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSD(
                &save.NAME,
                save.RECORD[2],
                b"=",
                (save.SUBTPS[1] as f64),
                0.0,
                OK,
                ctx,
            )?;

            fstr::assign(&mut save.NAME, b"Clock rate *");
            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.I, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSD(
                &save.NAME,
                save.RECORD[3],
                b"=",
                save.RATES[1],
                0.0,
                OK,
                ctx,
            )?;

            //
            // Check the record data.
            //
            fstr::assign(&mut save.NAME, b"Quaternion *");
            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.I, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.K = ((save.PKTSIZ * (save.I - 1)) + 1);

            testutil::CHCKAD(
                &save.NAME,
                save.RECORD.subarray(4),
                b"=",
                save.PACKTS.subarray(save.K),
                save.PKTSIZ,
                0.0,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // Close kernel.
    //
    spicelib::CKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Read from trivial subtype 1 segment using CKGP", ctx)?;

    spicelib::FURNSH(CK06T1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CKTOL = 0.0;
    save.TOL = VTIGHT;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NREC;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::CKGP(
                save.XINST,
                save.SCLKDP[save.I],
                save.CKTOL,
                &save.XREF,
                save.CMAT.as_slice_mut(),
                &mut save.CLKOUT,
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // Make sure data were found.
            //
            fstr::assign(&mut save.NAME, b"FOUND *");
            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.I, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.NAME, save.FOUND, true, OK, ctx)?;

            //
            // Check the output time.
            //
            fstr::assign(&mut save.NAME, b"CLKOUT *");
            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.I, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSD(
                &save.NAME,
                save.CLKOUT,
                b"=",
                save.SCLKDP[save.I],
                0.0,
                OK,
                ctx,
            )?;

            //
            // Check the C-matrix. Some round-off will occur here.
            //
            fstr::assign(&mut save.NAME, b"CMAT *");
            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.I, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.K = ((save.PKTSIZ * (save.I - 1)) + 1);

            spicelib::Q2M(save.PACKTS.subarray(save.K), save.XCMAT.as_slice_mut());
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.NAME,
                save.CMAT.as_slice(),
                b"~~/",
                save.XCMAT.as_slice(),
                9,
                VTIGHT,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    spicelib::UNLOAD(CK06T1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*****************************************************************
    //*
    //*     Tests involving segment containing mini-segments having
    //*     varied attributes
    //*
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Test CKW06: create segment containing mini-segments of all subtypes.",
        ctx,
    )?;

    //
    // Each mini-segment will have a gap at the end.
    //
    save.XINST = -1000;
    save.XAVFLG = true;
    fstr::assign(&mut save.XREF, b"GALACTIC");
    fstr::assign(&mut save.CKFRAM, b"IAU_EARTH");
    save.NINTVL = 4;
    fstr::assign(&mut save.SEGID, b"Subtype 1 test segment");
    save.SELLST = true;
    save.SEGNO = 1;
    save.N = 0;

    //
    // Below, we'll use different clock rates for each mini-segment. But
    // where we can,we'll adjust the angular velocity and quaternion
    // derivatives for the Hermite types so that the angular velocities
    // match when converted to radians/s.
    //
    // Where applicable, angular velocity will be scaled by the
    // corresponding rate. This will allow recovery of the original
    // angular velocity after conversion to radians/s.
    //

    //
    // Pointer into packet array.
    //
    save.PKTPTR = 1;
    //
    // Mini-segment 1:
    //
    save.MNSGNO = 1;
    save.BEGREC = 1;
    save.NPKTS[1] = 11;
    save.SUBTPS[1] = C06TP0;
    save.PKTSIZ = C06PS0;
    save.PSIZES[1] = save.PKTSIZ;
    save.DEGRES[1] = 3;
    save.RATES[1] = 0.001;

    T_GENC06(
        save.SEGNO,
        save.MNSGNO,
        save.BEGREC,
        save.NPKTS[1],
        &save.XREF,
        save.RATES[1],
        save.SCLKDP.as_slice_mut(),
        save.QUATS.as_slice_mut(),
        save.AVVS.as_slice_mut(),
        OK,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XIVBDS[1] = save.SCLKDP[1];

    save.FIRST = save.XIVBDS[1];

    //
    // Create quaternion derivatives from quaternions and avvs.
    // Pack quaternions and their derivatives into the PACKTS array.
    //
    spicelib::CLEARD(3, save.DAVV.as_slice_mut());

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NPKTS[1];
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // We're not using the derivative of angular velocity for
            // this subtype.
            //
            T_XSUBTP(
                save.QUATS.subarray([0, save.I]),
                save.AVVS.subarray([1, save.I]),
                save.DAVV.as_slice(),
                save.SUBTPS[1],
                save.PKTSIZ,
                save.PACKTS.subarray_mut(save.PKTPTR),
                ctx,
            )?;

            save.PKTPTR = (save.PKTPTR + save.PKTSIZ);

            save.I += m3__;
        }
    }

    //
    // Mini-segment 2:
    //
    save.MNSGNO = 2;
    save.BEGREC = (save.BEGREC + save.NPKTS[1]);
    save.NPKTS[2] = 12;
    save.SUBTPS[2] = C06TP1;
    save.PKTSIZ = C06PS1;
    save.PSIZES[2] = save.PKTSIZ;
    save.DEGRES[2] = 7;
    save.RATES[2] = 0.002;

    T_GENC06(
        save.SEGNO,
        save.MNSGNO,
        save.BEGREC,
        save.NPKTS[2],
        &save.XREF,
        save.RATES[2],
        save.EPOCHS.as_slice_mut(),
        save.QUATS.as_slice_mut(),
        save.AVVS.as_slice_mut(),
        OK,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // No AVV scaling is needed because the vector is
    // already considered to have units of radians/s.
    //
    spicelib::MOVED(
        save.EPOCHS.as_slice(),
        save.NPKTS[2],
        save.SCLKDP.subarray_mut(save.PKTPTR),
    );

    save.XIVBDS[2] = save.SCLKDP[save.BEGREC];

    //
    // This mini-segment has subtype 1; we can just
    // transfer the quaternions into the packet array.
    //
    spicelib::MOVED(
        save.QUATS.as_slice(),
        (4 * save.NPKTS[2]),
        save.PACKTS.subarray_mut(save.PKTPTR),
    );

    save.PKTPTR = (save.PKTPTR + (save.NPKTS[2] * save.PKTSIZ));

    //
    // Mini-segment 3:
    //
    save.MNSGNO = 3;
    save.BEGREC = (save.BEGREC + save.NPKTS[2]);
    save.NPKTS[3] = 40;
    save.SUBTPS[3] = C06TP2;
    save.PKTSIZ = C06PS2;
    save.PSIZES[3] = save.PKTSIZ;
    save.DEGRES[3] = 23;
    save.RATES[3] = 0.003;

    T_GENC06(
        save.SEGNO,
        save.MNSGNO,
        save.BEGREC,
        save.NPKTS[3],
        &save.XREF,
        save.RATES[3],
        save.EPOCHS.as_slice_mut(),
        save.QUATS.as_slice_mut(),
        save.AVVS.as_slice_mut(),
        OK,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::MOVED(
        save.EPOCHS.as_slice(),
        save.NPKTS[3],
        save.SCLKDP.subarray_mut(save.BEGREC),
    );

    save.XIVBDS[3] = save.SCLKDP[save.BEGREC];

    //
    // Create quaternion derivatives from quaternions and avvs.
    // Pack quaternions and their derivatives, as well as the
    // avvs and their derivatives, into the PACKTS array.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NPKTS[3];
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // We need to generate a derivative of the AVV. We'll
            // compute a 2-sided derivative if one's available;
            // otherwise we'll use a 1-sided derivative.
            //
            if (save.I == 1) {
                save.DELTA = ((save.SCLKDP[2] - save.SCLKDP[1]) * save.RATES[3]);

                spicelib::VLCOM(
                    (1.0 / save.DELTA),
                    save.AVVS.subarray([1, (save.I + 1)]),
                    -(1.0 / save.DELTA),
                    save.AVVS.subarray([1, save.I]),
                    save.DAVV.as_slice_mut(),
                );
            } else if (save.I < save.NPKTS[3]) {
                save.DELTA = (((save.SCLKDP[(save.I + 1)] - save.SCLKDP[save.I]) * save.RATES[3])
                    / 2 as f64);

                spicelib::VLCOM(
                    (1.0 / save.DELTA),
                    save.AVVS.subarray([1, (save.I + 1)]),
                    -(1.0 / save.DELTA),
                    save.AVVS.subarray([1, (save.I - 1)]),
                    save.DAVV.as_slice_mut(),
                );
            } else {
                save.DELTA = ((save.SCLKDP[save.I] - save.SCLKDP[(save.I - 1)]) * save.RATES[3]);

                spicelib::VLCOM(
                    (1.0 / save.DELTA),
                    save.AVVS.subarray([1, save.I]),
                    -(1.0 / save.DELTA),
                    save.AVVS.subarray([1, (save.I - 1)]),
                    save.DAVV.as_slice_mut(),
                );
            }

            T_XSUBTP(
                save.QUATS.subarray([0, save.I]),
                save.AVVS.subarray([1, save.I]),
                save.DAVV.as_slice(),
                save.SUBTPS[3],
                save.PKTSIZ,
                save.PACKTS.subarray_mut(save.PKTPTR),
                ctx,
            )?;

            save.PKTPTR = (save.PKTPTR + save.PKTSIZ);

            save.I += m3__;
        }
    }

    //
    // Mini-segment 4:
    //
    save.MNSGNO = 4;
    save.BEGREC = (save.BEGREC + save.NPKTS[3]);
    save.NPKTS[4] = 50;
    save.SUBTPS[4] = C06TP3;
    save.PKTSIZ = C06PS3;
    save.PSIZES[4] = save.PKTSIZ;
    save.DEGRES[4] = 23;
    save.RATES[4] = 0.004;

    T_GENC06(
        save.SEGNO,
        save.MNSGNO,
        save.BEGREC,
        save.NPKTS[4],
        &save.XREF,
        save.RATES[4],
        save.EPOCHS.as_slice_mut(),
        save.QUATS.as_slice_mut(),
        save.AVVS.as_slice_mut(),
        OK,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XIVBDS[4] = save.EPOCHS[1];
    save.XIVBDS[5] = (save.EPOCHS[save.NPKTS[4]] + 1.0);
    save.LAST = save.XIVBDS[5];

    spicelib::MOVED(
        save.EPOCHS.as_slice(),
        save.NPKTS[4],
        save.SCLKDP.subarray_mut(save.BEGREC),
    );

    //
    // Copy both quaternions and avvs into the packet array.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NPKTS[4];
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::MOVED(
                save.QUATS.subarray([0, save.I]),
                4,
                save.PACKTS.subarray_mut(save.PKTPTR),
            );
            save.PKTPTR = (save.PKTPTR + 4);

            spicelib::MOVED(
                save.AVVS.subarray([1, save.I]),
                3,
                save.PACKTS.subarray_mut(save.PKTPTR),
            );
            save.PKTPTR = (save.PKTPTR + 3);

            save.I += m3__;
        }
    }

    //
    // Open new C-kernel.
    //
    if spicelib::EXISTS(CK06MU, ctx)? {
        spicelib::DELFIL(CK06MU, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::CKOPN(CK06MU, CK06MU, 0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Write segment.
    //
    spicelib::CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.XIVBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    //
    // Close kernel.
    //
    spicelib::CKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*****************************************************************
    //*
    //*     CKNM06, CKMP06, CKGR06 Non-error cases:
    //*
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Examine the segment in the file CK06MU.", ctx)?;

    spicelib::DAFOPR(CK06MU, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the mini-segment count:
    //
    spicelib::CKNM06(save.HANDLE, save.DESCR.as_slice(), &mut save.NMINI, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NMINI", save.NMINI, b"=", 4, 0, OK, ctx)?;

    //
    // Check the mini-segment parameters:
    //
    save.BEGREC = 1;
    save.PKTPTR = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NMINI;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::CKMP06(
                save.HANDLE,
                save.DESCR.as_slice(),
                save.I,
                &mut save.RATE,
                &mut save.SUBTYP,
                &mut save.WINSIZ,
                &mut save.NREC,
                save.IVLBDS.as_slice_mut(),
                &mut save.LSTEPC,
                ctx,
            )?;

            fstr::assign(&mut save.NAME, b"Clock rate *");
            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.I, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSD(
                &save.NAME,
                save.RATE,
                b"=",
                save.RATES[save.I],
                0.0,
                OK,
                ctx,
            )?;

            fstr::assign(&mut save.NAME, b"Subtype code *");
            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.I, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSI(
                &save.NAME,
                save.SUBTYP,
                b"=",
                save.SUBTPS[save.I],
                0,
                OK,
                ctx,
            )?;

            fstr::assign(&mut save.NAME, b"Packet count *");
            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.I, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSI(&save.NAME, save.NREC, b"=", save.NPKTS[save.I], 0, OK, ctx)?;

            fstr::assign(&mut save.NAME, b"Window size *");
            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.I, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            if ((save.SUBTYP == 0) || (save.SUBTYP == 2)) {
                save.XWNSIZ = ((save.DEGRES[save.I] + 1) / 2);
            } else {
                save.XWNSIZ = (save.DEGRES[save.I] + 1);
            }

            testutil::CHCKSI(&save.NAME, save.WINSIZ, b"=", save.XWNSIZ, 0, OK, ctx)?;

            fstr::assign(&mut save.NAME, b"Interval start *");
            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.I, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.XT = save.XIVBDS[save.I];

            testutil::CHCKSD(&save.NAME, save.IVLBDS[1], b"=", save.XT, 0.0, OK, ctx)?;

            fstr::assign(&mut save.NAME, b"Interval stop *");
            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.I, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.XT = save.XIVBDS[(save.I + 1)];

            testutil::CHCKSD(&save.NAME, save.IVLBDS[2], b"=", save.XT, 0.0, OK, ctx)?;

            fstr::assign(&mut save.NAME, b"Last epoch *");
            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.I, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.XT = save.SCLKDP[((save.BEGREC + save.NPKTS[save.I]) - 1)];

            testutil::CHCKSD(&save.NAME, save.LSTEPC, b"=", save.XT, 0.0, OK, ctx)?;

            //
            // Check the packets.
            //
            save.PKTSIZ = save.PSIZES[save.I];

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NPKTS[save.I];
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    spicelib::CKGR06(
                        save.HANDLE,
                        save.DESCR.as_slice(),
                        save.I,
                        save.J,
                        save.RECORD.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.K = (save.PKTPTR + ((save.J - 1) * save.PKTSIZ));

                    //
                    // Check the record's time tag.
                    //
                    fstr::assign(&mut save.NAME, b"Mini-segment *; record time tag *");
                    spicelib::REPMI(&save.NAME.to_vec(), b"*", save.I, &mut save.NAME, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    spicelib::REPMI(&save.NAME.to_vec(), b"*", save.J, &mut save.NAME, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.XT = save.SCLKDP[((save.BEGREC + save.J) - 1)];

                    testutil::CHCKSD(&save.NAME, save.RECORD[1], b"=", save.XT, 0.0, OK, ctx)?;
                    //
                    // Check the subtype and rate.
                    //
                    fstr::assign(&mut save.NAME, b"Mini-segment *; record subtype *");
                    spicelib::REPMI(&save.NAME.to_vec(), b"*", save.I, &mut save.NAME, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    spicelib::REPMI(&save.NAME.to_vec(), b"*", save.J, &mut save.NAME, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::CHCKSI(
                        &save.NAME,
                        intrinsics::IDNINT(save.RECORD[2]),
                        b"=",
                        save.SUBTYP,
                        0,
                        OK,
                        ctx,
                    )?;

                    fstr::assign(&mut save.NAME, b"Mini-segment *; record rate *");
                    spicelib::REPMI(&save.NAME.to_vec(), b"*", save.I, &mut save.NAME, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    spicelib::REPMI(&save.NAME.to_vec(), b"*", save.J, &mut save.NAME, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::CHCKSD(&save.NAME, save.RECORD[3], b"=", save.RATE, 0.0, OK, ctx)?;

                    //
                    // Check the data portion of the record.
                    //
                    fstr::assign(&mut save.NAME, b"Mini-segment *; packet *");
                    spicelib::REPMI(&save.NAME.to_vec(), b"*", save.I, &mut save.NAME, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    spicelib::REPMI(&save.NAME.to_vec(), b"*", save.J, &mut save.NAME, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::CHCKAD(
                        &save.NAME,
                        save.RECORD.subarray(4),
                        b"=",
                        save.PACKTS.subarray(save.K),
                        save.PKTSIZ,
                        0.0,
                        OK,
                        ctx,
                    )?;

                    save.J += m3__;
                }
            }

            //
            // Update pointers.
            //
            save.BEGREC = (save.BEGREC + save.NPKTS[save.I]);
            save.PKTPTR = (save.PKTPTR + (save.NPKTS[save.I] * save.PKTSIZ));

            save.I += m3__;
        }
    }

    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*****************************************************************
    //*
    //*     CKE06 Non-error cases:
    //*
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Look up and check data from CK06MU. Look ups occur at time tags. Check first mini-segment.", ctx)?;

    spicelib::FURNSH(CK06MU, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SEGNO = 1;

    save.MNSGNO = 1;
    save.BEGREC = 1;
    save.PKTSIZ = C06PS0;

    //
    // Generate test data again.
    //
    T_GENC06(
        save.SEGNO,
        save.MNSGNO,
        save.BEGREC,
        save.NPKTS[1],
        &save.XREF,
        save.RATES[1],
        save.SCLKDP.as_slice_mut(),
        save.QUATS.as_slice_mut(),
        save.AVVS.as_slice_mut(),
        OK,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check data from the first mini-segment.
    //
    save.PKTPTR = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NPKTS[1];
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.J = ((save.BEGREC - 1) + save.I);

            spicelib::CKGPAV(
                save.XINST,
                save.SCLKDP[save.J],
                save.CKTOL,
                &save.XREF,
                save.CMAT.as_slice_mut(),
                save.AV.as_slice_mut(),
                &mut save.CLKOUT,
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

            //
            // Generate expected values.
            //
            save.K = (save.PKTPTR + ((save.I - 1) * save.PKTSIZ));

            spicelib::MOVED(save.PACKTS.subarray(save.K), 4, save.XQ.as_slice_mut());

            spicelib::Q2M(save.XQ.as_slice(), save.XCMAT.as_slice_mut());
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::VEQU(save.AVVS.subarray([1, save.J]), save.XAV.as_slice_mut());

            fstr::assign(&mut save.NAME, b"Mini-segment *;  * CMAT");
            spicelib::REPMI(&save.NAME.to_vec(), b"*", 1, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.I, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.NAME,
                save.CMAT.as_slice(),
                b"~~/",
                save.XCMAT.as_slice(),
                9,
                VTIGHT,
                OK,
                ctx,
            )?;

            fstr::assign(&mut save.NAME, b"Mini-segment *;  * AVV");
            spicelib::REPMI(&save.NAME.to_vec(), b"*", 1, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.I, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // For this case, we cannot expect high accuracy.
            // We're comparing angular velocity computed from
            // interpolated quaternion derivatives to the original
            // value.
            //
            testutil::CHCKAD(
                &save.NAME,
                save.AV.as_slice(),
                b"~~/",
                save.XAV.as_slice(),
                3,
                MED,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Look up and check data from CK06MU. Look ups occur at time tags. Check second mini-segment.", ctx)?;

    //
    // Check data from the second mini-segment.
    //
    save.MNSGNO = 2;
    save.BEGREC = (save.BEGREC + save.NPKTS[1]);
    save.PKTSIZ = C06PS1;

    //
    // Generate test data again.
    //
    T_GENC06(
        save.SEGNO,
        save.MNSGNO,
        save.BEGREC,
        save.NPKTS[2],
        &save.XREF,
        save.RATES[2],
        save.EPOCHS.as_slice_mut(),
        save.QUATS.as_slice_mut(),
        save.AVVS.as_slice_mut(),
        OK,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.PKTPTR = ((save.NPKTS[1] * save.PSIZES[1]) + 1);

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NPKTS[2];
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::CKGPAV(
                save.XINST,
                save.EPOCHS[save.I],
                save.CKTOL,
                &save.XREF,
                save.CMAT.as_slice_mut(),
                save.AV.as_slice_mut(),
                &mut save.CLKOUT,
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

            //
            // Generate expected values.
            //
            save.K = (save.PKTPTR + ((save.I - 1) * save.PKTSIZ));

            spicelib::MOVED(save.PACKTS.subarray(save.K), 4, save.XQ.as_slice_mut());

            spicelib::Q2M(save.XQ.as_slice(), save.XCMAT.as_slice_mut());
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::VEQU(save.AVVS.subarray([1, save.I]), save.XAV.as_slice_mut());
            //
            // Scale the expected AV to be compatible with
            // AV derived from the quaternions.
            //
            //  CALL VSCLIP ( 1/RATES(2), XAV )

            fstr::assign(&mut save.NAME, b"Mini-segment *;  * CMAT");
            spicelib::REPMI(&save.NAME.to_vec(), b"*", 2, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.I, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.NAME,
                save.CMAT.as_slice(),
                b"~~/",
                save.XCMAT.as_slice(),
                9,
                VTIGHT,
                OK,
                ctx,
            )?;

            fstr::assign(&mut save.NAME, b"Mini-segment *;  * AVV");
            spicelib::REPMI(&save.NAME.to_vec(), b"*", 2, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.I, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // For this case, we cannot expect high accuracy.
            // We're comparing angular velocity computed from
            // interpolated quaternion derivatives to the original
            // value.
            //
            testutil::CHCKAD(
                &save.NAME,
                save.AV.as_slice(),
                b"~~/",
                save.XAV.as_slice(),
                3,
                MED,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Look up and check data from CK06MU. Look ups occur at time tags. Check third mini-segment.", ctx)?;

    //
    // Check data from the third mini-segment.
    //

    save.MNSGNO = 3;
    save.BEGREC = (save.BEGREC + save.NPKTS[2]);
    save.PKTSIZ = C06PS2;

    //
    // Generate test data again.
    //
    T_GENC06(
        save.SEGNO,
        save.MNSGNO,
        save.BEGREC,
        save.NPKTS[3],
        &save.XREF,
        save.RATES[3],
        save.EPOCHS.as_slice_mut(),
        save.QUATS.as_slice_mut(),
        save.AVVS.as_slice_mut(),
        OK,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.PKTPTR = (save.PKTPTR + (save.NPKTS[2] * save.PSIZES[2]));

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NPKTS[3];
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::CKGPAV(
                save.XINST,
                save.EPOCHS[save.I],
                save.CKTOL,
                &save.XREF,
                save.CMAT.as_slice_mut(),
                save.AV.as_slice_mut(),
                &mut save.CLKOUT,
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

            //
            // Generate expected values.
            //
            save.K = (save.PKTPTR + ((save.I - 1) * save.PKTSIZ));

            spicelib::MOVED(save.PACKTS.subarray(save.K), 4, save.XQ.as_slice_mut());

            spicelib::Q2M(save.XQ.as_slice(), save.XCMAT.as_slice_mut());
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::VEQU(save.AVVS.subarray([1, save.I]), save.XAV.as_slice_mut());
            //
            // Scale the expected AV to be compatible with
            // AV derived from the quaternions.
            //
            //  CALL VSCLIP ( 1/RATES(3), XAV )

            fstr::assign(&mut save.NAME, b"Mini-segment *;  * CMAT");
            spicelib::REPMI(&save.NAME.to_vec(), b"*", 3, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.I, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.NAME,
                save.CMAT.as_slice(),
                b"~~/",
                save.XCMAT.as_slice(),
                9,
                VTIGHT,
                OK,
                ctx,
            )?;

            fstr::assign(&mut save.NAME, b"Mini-segment *;  * AVV");
            spicelib::REPMI(&save.NAME.to_vec(), b"*", 3, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.I, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.NAME,
                save.AV.as_slice(),
                b"~~/",
                save.XAV.as_slice(),
                3,
                VTIGHT,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Look up and check data from CK06MU. Look ups occur at time tags. Check fourth mini-segment.", ctx)?;

    //
    // Check data from the fourth mini-segment.
    //

    save.MNSGNO = 4;
    save.BEGREC = (save.BEGREC + save.NPKTS[3]);
    save.PKTSIZ = C06PS3;

    T_GENC06(
        save.SEGNO,
        save.MNSGNO,
        save.BEGREC,
        save.NPKTS[4],
        &save.XREF,
        save.RATES[4],
        save.EPOCHS.as_slice_mut(),
        save.QUATS.as_slice_mut(),
        save.AVVS.as_slice_mut(),
        OK,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.PKTPTR = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.PKTPTR = (save.PKTPTR + (save.NPKTS[save.I] * save.PSIZES[save.I]));
            save.I += m3__;
        }
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NPKTS[4];
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::CKGPAV(
                save.XINST,
                save.EPOCHS[save.I],
                save.CKTOL,
                &save.XREF,
                save.CMAT.as_slice_mut(),
                save.AV.as_slice_mut(),
                &mut save.CLKOUT,
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

            //
            // Generate expected values.
            //
            save.K = (save.PKTPTR + ((save.I - 1) * save.PKTSIZ));

            spicelib::MOVED(save.PACKTS.subarray(save.K), 4, save.XQ.as_slice_mut());

            spicelib::Q2M(save.XQ.as_slice(), save.XCMAT.as_slice_mut());
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::VEQU(save.AVVS.subarray([1, save.I]), save.XAV.as_slice_mut());

            fstr::assign(&mut save.NAME, b"Mini-segment *;  * CMAT");
            spicelib::REPMI(&save.NAME.to_vec(), b"*", 4, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.I, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Check C-matrix.
            //
            testutil::CHCKAD(
                &save.NAME,
                save.CMAT.as_slice(),
                b"~~/",
                save.XCMAT.as_slice(),
                9,
                VTIGHT,
                OK,
                ctx,
            )?;

            fstr::assign(&mut save.NAME, b"Mini-segment *;  * AVV");
            spicelib::REPMI(&save.NAME.to_vec(), b"*", 4, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.I, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // Check AV.
            //
            testutil::CHCKAD(
                &save.NAME,
                save.AV.as_slice(),
                b"~~/",
                save.XAV.as_slice(),
                3,
                VTIGHT,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    spicelib::UNLOAD(CK06MU, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*****************************************************************
    //*
    //*     CKR06 Error cases:
    //*
    //*****************************************************************

    //
    //     Bad mini-segment subtype test:
    //
    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"Create a trivial segment with an invalid subtype. Try to read from this segment.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    save.SEGNO = 1;
    save.MNSGNO = 1;
    save.BEGREC = 1;
    save.N = 201;
    fstr::assign(&mut save.XREF, b"MARSIAU");

    T_GENC06(
        save.SEGNO,
        save.MNSGNO,
        save.BEGREC,
        save.N,
        &save.XREF,
        1.0,
        save.SCLKDP.as_slice_mut(),
        save.PACKTS.as_slice_mut(),
        save.AVVS.as_slice_mut(),
        OK,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XINST = -1000;

    //
    // Set the AV flag to FALSE to set up next test.
    //
    save.XAVFLG = false;

    save.NINTVL = 1;
    save.NPKTS[1] = save.N;
    save.RATES[1] = 0.001;
    save.SUBTPS[1] = -1;
    save.DEGRES[1] = 3;

    save.XIVBDS[1] = save.SCLKDP[1];
    save.XIVBDS[2] = save.SCLKDP[save.N];

    save.FIRST = save.XIVBDS[1];
    save.LAST = save.XIVBDS[2];

    fstr::assign(&mut save.SEGID, b"Subtype 1 test segment");

    save.SELLST = true;

    //
    // Open new C-kernel.
    //
    if spicelib::EXISTS(CK06E, ctx)? {
        spicelib::DELFIL(CK06E, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::CKOPN(CK06E, b"Type 06 CK error file", 4, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Write segment.
    //
    T_CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.XIVBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    //
    // Close kernel.
    //
    spicelib::CKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load kernel for read access.
    //
    spicelib::FURNSH(CK06E, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Try a lookup.
    //
    save.CKTOL = 0.0;
    spicelib::CKGP(
        save.XINST,
        save.SCLKDP[1],
        save.CKTOL,
        &save.XREF,
        save.CMAT.as_slice_mut(),
        &mut save.CLKOUT,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDSUBTYPE)", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    //
    // Unload kernel.
    //
    spicelib::UNLOAD(CK06E, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Try to read record with AV.", ctx)?;

    //
    // Use the CK from the previous test case.
    //
    spicelib::DAFOPR(CK06E, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CKR06(
        save.HANDLE,
        save.DESCR.as_slice(),
        save.XIVBDS[1],
        0.0,
        true,
        save.RECORD.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOAVDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad segment data type", ctx)?;

    spicelib::DAFUS(
        save.DESCR.as_slice(),
        ND,
        NI,
        save.DC.as_slice_mut(),
        save.IC.as_slice_mut(),
    );
    save.IC[3] = -3;

    spicelib::DAFPS(
        ND,
        NI,
        save.DC.as_slice(),
        save.IC.as_slice(),
        save.DESCR.as_slice_mut(),
    );

    spicelib::CKR06(
        save.HANDLE,
        save.DESCR.as_slice(),
        save.XIVBDS[1],
        0.0,
        false,
        save.RECORD.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(WRONGCKTYPE)", OK, ctx)?;

    //
    // Close kernel.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Create a trivial segment with subtype 0 and a quaternion sign error. Try to read from this segment.");

    testutil::TCASE(&save.TITLE, ctx)?;

    save.SEGNO = 1;
    save.MNSGNO = 1;
    save.BEGREC = 1;
    save.N = 201;
    fstr::assign(&mut save.XREF, b"MARSIAU");
    save.SUBTYP = C06TP0;
    save.PKTSIZ = C06PS0;

    T_GENC06(
        save.SEGNO,
        save.MNSGNO,
        save.BEGREC,
        save.N,
        &save.XREF,
        1.0,
        save.SCLKDP.as_slice_mut(),
        save.QUATS.as_slice_mut(),
        save.AVVS.as_slice_mut(),
        OK,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.PKTPTR = (1 + ((save.I - 1) * save.PKTSIZ));

            T_XSUBTP(
                save.QUATS.subarray([0, save.I]),
                save.AVVS.subarray([1, save.I]),
                save.DAVV.as_slice(),
                save.SUBTYP,
                save.PKTSIZ,
                save.PACKTS.subarray_mut(save.PKTPTR),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Create error in quaternion sequence at packet index 6:
            //
            if (save.I == 6) {
                spicelib::VMINUG(
                    save.PACKTS.subarray(save.PKTPTR),
                    4,
                    save.QNEG.as_slice_mut(),
                );
                spicelib::MOVED(
                    save.QNEG.as_slice(),
                    4,
                    save.PACKTS.subarray_mut(save.PKTPTR),
                );
            }

            save.I += m3__;
        }
    }

    save.XINST = -1000;
    save.XAVFLG = true;

    save.NINTVL = 1;
    save.NPKTS[1] = save.N;
    save.RATES[1] = 0.001;
    save.SUBTPS[1] = save.SUBTYP;
    save.DEGRES[1] = 15;

    save.XIVBDS[1] = save.SCLKDP[1];
    save.XIVBDS[2] = save.SCLKDP[save.N];

    save.FIRST = save.XIVBDS[1];
    save.LAST = save.XIVBDS[2];

    fstr::assign(&mut save.SEGID, b"Subtype 0 test segment");

    save.SELLST = true;

    //
    // Open new C-kernel.
    //
    if spicelib::EXISTS(CK06E, ctx)? {
        spicelib::DELFIL(CK06E, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::CKOPN(CK06E, b"Type 06 CK error file", 4, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Write segment.
    //
    T_CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.XIVBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    //
    // Close kernel.
    //
    spicelib::CKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load kernel for read access.
    //
    spicelib::FURNSH(CK06E, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Try a lookup.
    //
    save.CKTOL = 0.0;
    spicelib::CKGP(
        save.XINST,
        save.SCLKDP[6],
        save.CKTOL,
        &save.XREF,
        save.CMAT.as_slice_mut(),
        &mut save.CLKOUT,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADQUATSIGN)", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    //
    // Unload kernel.
    //
    spicelib::UNLOAD(CK06E, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Create a trivial segment with subtype 2 and a quaternion sign error. Try to read from this segment.");

    testutil::TCASE(&save.TITLE, ctx)?;

    save.SEGNO = 1;
    save.MNSGNO = 1;
    save.BEGREC = 1;
    save.N = 201;
    fstr::assign(&mut save.XREF, b"MARSIAU");
    save.SUBTYP = C06TP2;
    save.PKTSIZ = C06PS2;

    T_GENC06(
        save.SEGNO,
        save.MNSGNO,
        save.BEGREC,
        save.N,
        &save.XREF,
        1.0,
        save.SCLKDP.as_slice_mut(),
        save.QUATS.as_slice_mut(),
        save.AVVS.as_slice_mut(),
        OK,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.PKTPTR = (1 + ((save.I - 1) * save.PKTSIZ));

            T_XSUBTP(
                save.QUATS.subarray([0, save.I]),
                save.AVVS.subarray([1, save.I]),
                save.DAVV.as_slice(),
                save.SUBTYP,
                save.PKTSIZ,
                save.PACKTS.subarray_mut(save.PKTPTR),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Create error in quaternion sequence at packet index 7:
            //
            if (save.I == 7) {
                spicelib::VMINUG(
                    save.PACKTS.subarray(save.PKTPTR),
                    4,
                    save.QNEG.as_slice_mut(),
                );
                spicelib::MOVED(
                    save.QNEG.as_slice(),
                    4,
                    save.PACKTS.subarray_mut(save.PKTPTR),
                );
            }

            save.I += m3__;
        }
    }

    save.XINST = -1000;
    save.XAVFLG = true;

    save.NINTVL = 1;
    save.NPKTS[1] = save.N;
    save.RATES[1] = 0.001;
    save.SUBTPS[1] = save.SUBTYP;
    save.DEGRES[1] = 15;

    save.XIVBDS[1] = save.SCLKDP[1];
    save.XIVBDS[2] = save.SCLKDP[save.N];

    save.FIRST = save.XIVBDS[1];
    save.LAST = save.XIVBDS[2];

    fstr::assign(&mut save.SEGID, b"Subtype 2 test segment");

    save.SELLST = true;

    //
    // Open new C-kernel.
    //
    if spicelib::EXISTS(CK06E, ctx)? {
        spicelib::DELFIL(CK06E, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::CKOPN(CK06E, b"Type 06 CK error file", 4, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Write segment.
    //
    T_CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.XIVBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    //
    // Close kernel.
    //
    spicelib::CKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load kernel for read access.
    //
    spicelib::FURNSH(CK06E, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Try a lookup.
    //
    save.CKTOL = 0.0;
    spicelib::CKGP(
        save.XINST,
        save.SCLKDP[6],
        save.CKTOL,
        &save.XREF,
        save.CMAT.as_slice_mut(),
        &mut save.CLKOUT,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADQUATSIGN)", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    //
    // Unload kernel.
    //
    spicelib::UNLOAD(CK06E, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Create a trivial segment with subtype 2 and a window size equivalent to 2 mod 4. Try to read from this segment.");

    testutil::TCASE(&save.TITLE, ctx)?;

    save.SEGNO = 1;
    save.MNSGNO = 1;
    save.BEGREC = 1;
    save.N = 201;
    fstr::assign(&mut save.XREF, b"MARSIAU");
    save.SUBTYP = C06TP2;
    save.PKTSIZ = C06PS2;

    T_GENC06(
        save.SEGNO,
        save.MNSGNO,
        save.BEGREC,
        save.N,
        &save.XREF,
        1.0,
        save.SCLKDP.as_slice_mut(),
        save.QUATS.as_slice_mut(),
        save.AVVS.as_slice_mut(),
        OK,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.PKTPTR = (1 + ((save.I - 1) * save.PKTSIZ));

            T_XSUBTP(
                save.QUATS.subarray([0, save.I]),
                save.AVVS.subarray([1, save.I]),
                save.DAVV.as_slice(),
                save.SUBTYP,
                save.PKTSIZ,
                save.PACKTS.subarray_mut(save.PKTPTR),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    save.XINST = -1000;
    save.XAVFLG = true;

    save.NINTVL = 1;
    save.NPKTS[1] = save.N;
    save.RATES[1] = 0.001;
    save.SUBTPS[1] = save.SUBTYP;
    save.DEGRES[1] = 14;

    save.XIVBDS[1] = save.SCLKDP[1];
    save.XIVBDS[2] = save.SCLKDP[save.N];

    save.FIRST = save.XIVBDS[1];
    save.LAST = save.XIVBDS[2];

    fstr::assign(&mut save.SEGID, b"Subtype 2 test segment");

    save.SELLST = true;

    //
    // Open new C-kernel.
    //
    if spicelib::EXISTS(CK06E, ctx)? {
        spicelib::DELFIL(CK06E, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::CKOPN(CK06E, b"Type 06 CK error file", 4, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Write segment.
    //
    T_CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.XIVBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    //
    // Close kernel.
    //
    spicelib::CKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load kernel for read access.
    //
    spicelib::FURNSH(CK06E, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Try a lookup.
    //
    save.CKTOL = 0.0;
    spicelib::CKGP(
        save.XINST,
        save.SCLKDP[6],
        save.CKTOL,
        &save.XREF,
        save.CMAT.as_slice_mut(),
        &mut save.CLKOUT,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDVALUE)", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    //
    // Unload kernel.
    //
    spicelib::UNLOAD(CK06E, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Create a trivial segment with subtype 1 and an even window size. Try to read from this segment.");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::DAFOPW(CK06E, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SUBTYP = C06TP1;
    save.PKTSIZ = C06PS1;

    T_GENC06(
        save.SEGNO,
        save.MNSGNO,
        save.BEGREC,
        save.N,
        &save.XREF,
        1.0,
        save.SCLKDP.as_slice_mut(),
        save.QUATS.as_slice_mut(),
        save.AVVS.as_slice_mut(),
        OK,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.PKTPTR = (1 + ((save.I - 1) * save.PKTSIZ));

            T_XSUBTP(
                save.QUATS.subarray([0, save.I]),
                save.AVVS.subarray([1, save.I]),
                save.DAVV.as_slice(),
                save.SUBTYP,
                save.PKTSIZ,
                save.PACKTS.subarray_mut(save.PKTPTR),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    save.XINST = -2000;
    save.XAVFLG = true;

    save.NINTVL = 1;
    save.NPKTS[1] = save.N;
    save.RATES[1] = 0.001;
    save.SUBTPS[1] = save.SUBTYP;
    save.DEGRES[1] = 14;

    save.XIVBDS[1] = save.SCLKDP[1];
    save.XIVBDS[2] = save.SCLKDP[save.N];

    save.FIRST = save.XIVBDS[1];
    save.LAST = save.XIVBDS[2];

    fstr::assign(&mut save.SEGID, b"Subtype 1 test segment");

    save.SELLST = true;

    //
    // Write segment.
    //
    T_CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.XIVBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    //
    // Close kernel.
    //
    spicelib::CKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load kernel for read access.
    //
    spicelib::FURNSH(CK06E, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Try a lookup.
    //
    save.CKTOL = 0.0;
    spicelib::CKGP(
        -2000,
        save.SCLKDP[6],
        save.CKTOL,
        &save.XREF,
        save.CMAT.as_slice_mut(),
        &mut save.CLKOUT,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDVALUE)", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    //
    // Unload kernel.
    //
    spicelib::UNLOAD(CK06E, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Create a trivial segment with subtype 1 and window size too small. Try to read from this segment.");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::DAFOPW(CK06E, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SUBTYP = C06TP1;
    save.PKTSIZ = C06PS1;

    T_GENC06(
        save.SEGNO,
        save.MNSGNO,
        save.BEGREC,
        save.N,
        &save.XREF,
        1.0,
        save.SCLKDP.as_slice_mut(),
        save.QUATS.as_slice_mut(),
        save.AVVS.as_slice_mut(),
        OK,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.PKTPTR = (1 + ((save.I - 1) * save.PKTSIZ));

            T_XSUBTP(
                save.QUATS.subarray([0, save.I]),
                save.AVVS.subarray([1, save.I]),
                save.DAVV.as_slice(),
                save.SUBTYP,
                save.PKTSIZ,
                save.PACKTS.subarray_mut(save.PKTPTR),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    save.XINST = -2000;
    save.XAVFLG = true;

    save.NINTVL = 1;
    save.NPKTS[1] = save.N;
    save.RATES[1] = 0.001;
    save.SUBTPS[1] = save.SUBTYP;
    save.DEGRES[1] = 0;

    save.XIVBDS[1] = save.SCLKDP[1];
    save.XIVBDS[2] = save.SCLKDP[save.N];

    save.FIRST = save.XIVBDS[1];
    save.LAST = save.XIVBDS[2];

    fstr::assign(&mut save.SEGID, b"Subtype 1 test segment");

    save.SELLST = true;

    //
    // Write segment.
    //
    T_CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.XIVBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    //
    // Close kernel.
    //
    spicelib::CKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load kernel for read access.
    //
    spicelib::FURNSH(CK06E, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Try a lookup.
    //
    save.CKTOL = 0.0;
    spicelib::CKGP(
        -2000,
        save.SCLKDP[6],
        save.CKTOL,
        &save.XREF,
        save.CMAT.as_slice_mut(),
        &mut save.CLKOUT,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDVALUE)", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    //
    // Unload kernel.
    //
    spicelib::UNLOAD(CK06E, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Create a trivial segment with subtype 1 and window size too large. Try to read from this segment.");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::DAFOPW(CK06E, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SUBTYP = C06TP1;
    save.PKTSIZ = C06PS1;

    T_GENC06(
        save.SEGNO,
        save.MNSGNO,
        save.BEGREC,
        save.N,
        &save.XREF,
        1.0,
        save.SCLKDP.as_slice_mut(),
        save.QUATS.as_slice_mut(),
        save.AVVS.as_slice_mut(),
        OK,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.PKTPTR = (1 + ((save.I - 1) * save.PKTSIZ));

            T_XSUBTP(
                save.QUATS.subarray([0, save.I]),
                save.AVVS.subarray([1, save.I]),
                save.DAVV.as_slice(),
                save.SUBTYP,
                save.PKTSIZ,
                save.PACKTS.subarray_mut(save.PKTPTR),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    save.XINST = -2000;
    save.XAVFLG = true;

    save.NINTVL = 1;
    save.NPKTS[1] = save.N;
    save.RATES[1] = 0.001;
    save.SUBTPS[1] = save.SUBTYP;
    save.DEGRES[1] = 29;

    save.XIVBDS[1] = save.SCLKDP[1];
    save.XIVBDS[2] = save.SCLKDP[save.N];

    save.FIRST = save.XIVBDS[1];
    save.LAST = save.XIVBDS[2];

    fstr::assign(&mut save.SEGID, b"Subtype 1 test segment");

    save.SELLST = true;

    //
    // Write segment.
    //
    T_CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.XIVBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;

    //
    // Close kernel.
    //
    spicelib::CKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load kernel for read access.
    //
    spicelib::FURNSH(CK06E, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Try a lookup.
    //
    save.CKTOL = 0.0;
    spicelib::CKGP(
        -2000,
        save.SCLKDP[6],
        save.CKTOL,
        &save.XREF,
        save.CMAT.as_slice_mut(),
        &mut save.CLKOUT,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDVALUE)", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    //
    // Unload kernel.
    //
    spicelib::UNLOAD(CK06E, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // This is a "non-error" case: we want to make sure that
    // quaternion inversion is handled for subtype 1. The
    // result of interpolation should be the same as if all
    // the quaternions had compatible signs.
    //

    fstr::assign(&mut save.TITLE, b"Create a trivial segment with subtype 1 and a quaternion sign error. Try to read from this segment.");

    testutil::TCASE(&save.TITLE, ctx)?;

    save.SEGNO = 1;
    save.MNSGNO = 1;
    save.BEGREC = 1;
    save.N = 201;
    fstr::assign(&mut save.XREF, b"MARSIAU");
    save.SUBTYP = C06TP1;
    save.PKTSIZ = C06PS1;

    T_GENC06(
        save.SEGNO,
        save.MNSGNO,
        save.BEGREC,
        save.N,
        &save.XREF,
        1.0,
        save.SCLKDP.as_slice_mut(),
        save.QUATS.as_slice_mut(),
        save.AVVS.as_slice_mut(),
        OK,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.PKTPTR = (1 + ((save.I - 1) * save.PKTSIZ));

            T_XSUBTP(
                save.QUATS.subarray([0, save.I]),
                save.AVVS.subarray([1, save.I]),
                save.DAVV.as_slice(),
                save.SUBTYP,
                save.PKTSIZ,
                save.PACKTS.subarray_mut(save.PKTPTR),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Create error in quaternion sequence at every even-
            // indexed quaternion.
            //
            if spicelib::EVEN(save.I) {
                spicelib::VMINUG(
                    save.PACKTS.subarray(save.PKTPTR),
                    4,
                    save.QNEG.as_slice_mut(),
                );
                spicelib::MOVED(
                    save.QNEG.as_slice(),
                    4,
                    save.PACKTS.subarray_mut(save.PKTPTR),
                );
            }

            save.I += m3__;
        }
    }

    save.XINST = -1000;
    save.XAVFLG = true;

    save.NINTVL = 1;
    save.NPKTS[1] = save.N;
    save.RATES[1] = 0.001;
    save.SUBTPS[1] = save.SUBTYP;
    save.DEGRES[1] = 15;

    save.XIVBDS[1] = save.SCLKDP[1];
    save.XIVBDS[2] = save.SCLKDP[save.N];

    save.FIRST = save.XIVBDS[1];
    save.LAST = save.XIVBDS[2];

    fstr::assign(&mut save.SEGID, b"Partially inverted subtype 1 segment");

    save.SELLST = true;

    //
    // Open new C-kernel.
    //
    if spicelib::EXISTS(CK06E, ctx)? {
        spicelib::DELFIL(CK06E, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::CKOPN(CK06E, b"Type 06 CK error file", 4, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Write segment. We can use the SPICELIB type 6 writer
    // since it's supposed to allow the quaternion mess we've
    // created when the subtype is either 1 or 3.
    //
    spicelib::CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.XIVBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a second segment for instrument -2000. This one
    // has a normal quaternion sequence.
    //
    T_GENC06(
        save.SEGNO,
        save.MNSGNO,
        save.BEGREC,
        save.N,
        &save.XREF,
        1.0,
        save.SCLKDP.as_slice_mut(),
        save.QUATS.as_slice_mut(),
        save.AVVS.as_slice_mut(),
        OK,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.PKTPTR = (1 + ((save.I - 1) * save.PKTSIZ));

            T_XSUBTP(
                save.QUATS.subarray([0, save.I]),
                save.AVVS.subarray([1, save.I]),
                save.DAVV.as_slice(),
                save.SUBTYP,
                save.PKTSIZ,
                save.PACKTS.subarray_mut(save.PKTPTR),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    save.XINST = -2000;
    save.XAVFLG = true;

    save.NINTVL = 1;
    save.NPKTS[1] = save.N;
    save.RATES[1] = 0.001;
    save.SUBTPS[1] = save.SUBTYP;
    save.DEGRES[1] = 15;

    save.XIVBDS[1] = save.SCLKDP[1];
    save.XIVBDS[2] = save.SCLKDP[save.N];

    save.FIRST = save.XIVBDS[1];
    save.LAST = save.XIVBDS[2];

    fstr::assign(&mut save.SEGID, b"Normal subtype 1 test segment");

    save.SELLST = true;

    spicelib::CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.XIVBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close kernel.
    //
    spicelib::CKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load kernel for read access.
    //
    spicelib::FURNSH(CK06E, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Try lookups for each instrument.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = (save.N - 1);
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.T = ((save.SCLKDP[save.I] + save.SCLKDP[(save.I + 1)]) / 2 as f64);

            save.CKTOL = 0.0;
            spicelib::CKGPAV(
                -2000,
                save.T,
                save.CKTOL,
                &save.XREF,
                save.XCMAT.as_slice_mut(),
                save.XAV.as_slice_mut(),
                &mut save.CLKOUT,
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSL(b"FOUND 1", save.FOUND, true, OK, ctx)?;

            spicelib::CKGPAV(
                -1000,
                save.T,
                save.CKTOL,
                &save.XREF,
                save.CMAT.as_slice_mut(),
                save.AV.as_slice_mut(),
                &mut save.CLKOUT,
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSL(b"FOUND 2", save.FOUND, true, OK, ctx)?;

            testutil::CHCKAD(
                b"CMAT",
                save.CMAT.as_slice(),
                b"~~/",
                save.XCMAT.as_slice(),
                9,
                VTIGHT,
                OK,
                ctx,
            )?;
            testutil::CHCKAD(
                b"CMAT",
                save.AV.as_slice(),
                b"~~/",
                save.XAV.as_slice(),
                3,
                VTIGHT,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }
    //
    // Unload kernel.
    //
    spicelib::UNLOAD(CK06E, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // This is a "non-error" case: same deal as before, this time
    // for subtype 3.
    //

    fstr::assign(&mut save.TITLE, b"Create a trivial segment with subtype 3 and a quaternion sign error. Try to read from this segment.");

    testutil::TCASE(&save.TITLE, ctx)?;

    save.SEGNO = 1;
    save.MNSGNO = 1;
    save.BEGREC = 1;
    save.N = 201;
    fstr::assign(&mut save.XREF, b"MARSIAU");
    save.SUBTYP = C06TP3;
    save.PKTSIZ = C06PS3;

    T_GENC06(
        save.SEGNO,
        save.MNSGNO,
        save.BEGREC,
        save.N,
        &save.XREF,
        1.0,
        save.SCLKDP.as_slice_mut(),
        save.QUATS.as_slice_mut(),
        save.AVVS.as_slice_mut(),
        OK,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.PKTPTR = (1 + ((save.I - 1) * save.PKTSIZ));

            T_XSUBTP(
                save.QUATS.subarray([0, save.I]),
                save.AVVS.subarray([1, save.I]),
                save.DAVV.as_slice(),
                save.SUBTYP,
                save.PKTSIZ,
                save.PACKTS.subarray_mut(save.PKTPTR),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Create error in quaternion sequence at every even-
            // indexed quaternion.
            //
            if spicelib::EVEN(save.I) {
                spicelib::VMINUG(
                    save.PACKTS.subarray(save.PKTPTR),
                    4,
                    save.QNEG.as_slice_mut(),
                );
                spicelib::MOVED(
                    save.QNEG.as_slice(),
                    4,
                    save.PACKTS.subarray_mut(save.PKTPTR),
                );
            }

            save.I += m3__;
        }
    }

    save.XINST = -1000;
    save.XAVFLG = true;

    save.NINTVL = 1;
    save.NPKTS[1] = save.N;
    save.RATES[1] = 0.001;
    save.SUBTPS[1] = save.SUBTYP;
    save.DEGRES[1] = 15;

    save.XIVBDS[1] = save.SCLKDP[1];
    save.XIVBDS[2] = save.SCLKDP[save.N];

    save.FIRST = save.XIVBDS[1];
    save.LAST = save.XIVBDS[2];

    fstr::assign(&mut save.SEGID, b"Partially inverted subtype 3 segment");

    save.SELLST = true;

    //
    // Open new C-kernel.
    //
    if spicelib::EXISTS(CK06E, ctx)? {
        spicelib::DELFIL(CK06E, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::CKOPN(CK06E, b"Type 06 CK error file", 4, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Write segment. We can use the SPICELIB type 6 writer
    // since it's supposed to allow the quaternion mess we've
    // created when the subtype is either 1 or 3.
    //
    spicelib::CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.XIVBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a second segment for instrument -2000. This one
    // has a normal quaternion sequence.
    //
    T_GENC06(
        save.SEGNO,
        save.MNSGNO,
        save.BEGREC,
        save.N,
        &save.XREF,
        1.0,
        save.SCLKDP.as_slice_mut(),
        save.QUATS.as_slice_mut(),
        save.AVVS.as_slice_mut(),
        OK,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.PKTPTR = (1 + ((save.I - 1) * save.PKTSIZ));

            T_XSUBTP(
                save.QUATS.subarray([0, save.I]),
                save.AVVS.subarray([1, save.I]),
                save.DAVV.as_slice(),
                save.SUBTYP,
                save.PKTSIZ,
                save.PACKTS.subarray_mut(save.PKTPTR),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    save.XINST = -2000;
    save.XAVFLG = true;

    save.NINTVL = 1;
    save.NPKTS[1] = save.N;
    save.RATES[1] = 0.001;
    save.SUBTPS[1] = save.SUBTYP;
    save.DEGRES[1] = 15;

    save.XIVBDS[1] = save.SCLKDP[1];
    save.XIVBDS[2] = save.SCLKDP[save.N];

    save.FIRST = save.XIVBDS[1];
    save.LAST = save.XIVBDS[2];

    fstr::assign(&mut save.SEGID, b"Normal subtype 3 test segment");

    save.SELLST = true;

    spicelib::CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.XIVBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close kernel.
    //
    spicelib::CKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load kernel for read access.
    //
    spicelib::FURNSH(CK06E, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Try lookups for each instrument.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = (save.N - 1);
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.T = ((save.SCLKDP[save.I] + save.SCLKDP[(save.I + 1)]) / 2 as f64);

            save.CKTOL = 0.0;
            spicelib::CKGPAV(
                -2000,
                save.T,
                save.CKTOL,
                &save.XREF,
                save.XCMAT.as_slice_mut(),
                save.XAV.as_slice_mut(),
                &mut save.CLKOUT,
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSL(b"FOUND 1", save.FOUND, true, OK, ctx)?;

            spicelib::CKGPAV(
                -1000,
                save.T,
                save.CKTOL,
                &save.XREF,
                save.CMAT.as_slice_mut(),
                save.AV.as_slice_mut(),
                &mut save.CLKOUT,
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSL(b"FOUND 2", save.FOUND, true, OK, ctx)?;

            testutil::CHCKAD(
                b"CMAT",
                save.CMAT.as_slice(),
                b"~~/",
                save.XCMAT.as_slice(),
                9,
                VTIGHT,
                OK,
                ctx,
            )?;
            testutil::CHCKAD(
                b"CMAT",
                save.AV.as_slice(),
                b"~~/",
                save.XAV.as_slice(),
                3,
                VTIGHT,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"CKR06 error case: negative tolerance.", ctx)?;

    //
    // We can use the CK from the previous test case.
    //

    //
    // Unload kernel; open for DAF read access.
    //
    spicelib::UNLOAD(CK06E, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFOPR(CK06E, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.T = save.SCLKDP[1];

    save.CKTOL = -1.0;

    spicelib::CKR06(
        save.HANDLE,
        save.DESCR.as_slice(),
        save.T,
        save.CKTOL,
        true,
        save.RECORD.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NEGATIVETOL)", OK, ctx)?;

    //
    // Unload kernel.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*****************************************************************
    //*
    //*     CKR06 Non-error cases:
    //*
    //*****************************************************************

    //
    // Use CK from previous tests to examine gap and non-zero
    // tolerance logic in CKR06.
    //
    // These tests use CK06MU. To start out, restore the arrays
    // of parameters used to create this file.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Tolerance is positive; lookup time precedes segment start by less than the tolerance. Populate data arrays for further tests.", ctx)?;

    spicelib::DAFOPR(CK06MU, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CKNM06(save.HANDLE, save.DESCR.as_slice(), &mut save.NINTVL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if *OK {
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NINTVL;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                spicelib::CKMP06(
                    save.HANDLE,
                    save.DESCR.as_slice(),
                    save.I,
                    &mut save.RATES[save.I],
                    &mut save.SUBTPS[save.I],
                    &mut save.WINSIZ,
                    &mut save.NPKTS[save.I],
                    save.IVLBDS.as_slice_mut(),
                    &mut save.LSTEPC,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.I += m3__;
            }
        }
    }

    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(CK06MU, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Tolerance is positive; lookup time precedes segment start by less than the tolerance.",
        ctx,
    )?;

    save.XINST = -1000;
    fstr::assign(&mut save.XREF, b"GALACTIC");
    save.SEGNO = 1;

    //
    // Re-generate data for the first mini-segment.
    //
    save.MNSGNO = 1;
    save.BEGREC = 1;
    save.PKTSIZ = save.NPKTS[1];

    save.N = save.NPKTS[1];

    T_GENC06(
        save.SEGNO,
        save.MNSGNO,
        save.BEGREC,
        save.N,
        &save.XREF,
        save.RATES[1],
        save.EPOCHS.as_slice_mut(),
        save.QUATS.as_slice_mut(),
        save.AVVS.as_slice_mut(),
        OK,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CKTOL = 0.5;
    save.T = ((save.EPOCHS[1] - save.CKTOL) + 0.000001);

    save.T = save.EPOCHS[1];

    save.FIRST = save.EPOCHS[1];
    save.LAST = (save.EPOCHS[save.N] + 1.0);

    spicelib::CKGPAV(
        save.XINST,
        save.T,
        save.CKTOL,
        &save.XREF,
        save.CMAT.as_slice_mut(),
        save.AV.as_slice_mut(),
        &mut save.CLKOUT,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // The data should be those for the first epoch.
    //
    testutil::CHCKSD(b"CLKOUT", save.CLKOUT, b"=", save.EPOCHS[1], 0.0, OK, ctx)?;

    spicelib::VEQU(save.AVVS.subarray([1, 1]), save.XAV.as_slice_mut());

    testutil::CHCKAD(
        b"AV",
        save.AV.as_slice(),
        b"~~/",
        save.XAV.as_slice(),
        3,
        MTIGHT,
        OK,
        ctx,
    )?;

    spicelib::MOVED(save.QUATS.subarray([0, 1]), 4, save.XQ.as_slice_mut());

    spicelib::Q2M(save.XQ.as_slice(), save.XCMAT.as_slice_mut());

    testutil::CHCKAD(
        b"CMAT",
        save.CMAT.as_slice(),
        b"~~/",
        save.XCMAT.as_slice(),
        9,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Convert expected AV to quaternion derivative and back.
    // The purpose of this test is to quantify the error made
    // by these conversions; this error should dominate the
    // error in the AV test above.
    //
    spicelib::CLEARD(4, save.AVQ.as_slice_mut());
    spicelib::VSCL(-0.5, save.AV.as_slice(), save.AVQ.subarray_mut(1));
    spicelib::QXQ(
        save.XQ.as_slice(),
        save.AVQ.as_slice(),
        save.XDQ.as_slice_mut(),
    );
    spicelib::QDQ2AV(
        save.XQ.as_slice(),
        save.XDQ.as_slice(),
        save.AV.as_slice_mut(),
    );

    testutil::CHCKAD(
        b"AV 2",
        save.AV.as_slice(),
        b"~~/",
        save.XAV.as_slice(),
        3,
        MTIGHT,
        OK,
        ctx,
    )?;

    //
    // Make sure a second look-up produces the same results.
    //

    spicelib::CKGPAV(
        save.XINST,
        save.T,
        save.CKTOL,
        &save.XREF,
        save.CMAT.as_slice_mut(),
        save.AV.as_slice_mut(),
        &mut save.CLKOUT,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // The data should be those for the first epoch.
    //
    testutil::CHCKSD(b"CLKOUT", save.CLKOUT, b"=", save.EPOCHS[1], 0.0, OK, ctx)?;

    spicelib::VEQU(save.AVVS.subarray([1, 1]), save.XAV.as_slice_mut());

    testutil::CHCKAD(
        b"AV",
        save.AV.as_slice(),
        b"~~/",
        save.XAV.as_slice(),
        3,
        MTIGHT,
        OK,
        ctx,
    )?;

    spicelib::MOVED(save.QUATS.subarray([0, 1]), 4, save.XQ.as_slice_mut());

    spicelib::Q2M(save.XQ.as_slice(), save.XCMAT.as_slice_mut());

    testutil::CHCKAD(
        b"CMAT",
        save.CMAT.as_slice(),
        b"~~/",
        save.XCMAT.as_slice(),
        9,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Tolerance is positive; lookup time precedes segment start by more than the tolerance.",
        ctx,
    )?;

    spicelib::UNLOAD(CK06MU, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CKTOL = 0.5;
    save.T = ((save.EPOCHS[1] - save.CKTOL) - 0.000001);

    spicelib::DAFOPR(CK06MU, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CKR06(
        save.HANDLE,
        save.DESCR.as_slice(),
        save.T,
        save.CKTOL,
        false,
        save.RECORD.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Tolerance is positive; lookup time exceeds segment stop by more than the tolerance.",
        ctx,
    )?;

    save.CKTOL = 0.5;

    spicelib::DAFUS(
        save.DESCR.as_slice(),
        ND,
        NI,
        save.DC.as_slice_mut(),
        save.IC.as_slice_mut(),
    );

    save.T = ((save.DC[2] + save.CKTOL) + 0.000001);

    spicelib::CKR06(
        save.HANDLE,
        save.DESCR.as_slice(),
        save.T,
        save.CKTOL,
        false,
        save.RECORD.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Tolerance is positive; lookup time exceeds segment stop by less than the tolerance, but is too far from the last epoch of the last mini-segment for data to be found.", ctx)?;

    spicelib::FURNSH(CK06MU, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Re-generate data for the last mini-segment.
    //
    save.MNSGNO = 4;
    save.BEGREC = (((save.NPKTS[1] + save.NPKTS[2]) + save.NPKTS[3]) + 1);
    save.PKTSIZ = C06PS3;

    save.N = save.NPKTS[4];

    T_GENC06(
        save.SEGNO,
        save.MNSGNO,
        save.BEGREC,
        save.N,
        &save.XREF,
        save.RATES[4],
        save.EPOCHS.as_slice_mut(),
        save.QUATS.as_slice_mut(),
        save.AVVS.as_slice_mut(),
        OK,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.FIRST = save.EPOCHS[1];
    save.LAST = (save.EPOCHS[save.N] + 1.0);

    save.CKTOL = 0.5;
    save.T = (save.LAST + (save.CKTOL / 2 as f64));

    spicelib::CKGPAV(
        save.XINST,
        save.T,
        save.CKTOL,
        &save.XREF,
        save.CMAT.as_slice_mut(),
        save.AV.as_slice_mut(),
        &mut save.CLKOUT,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Tolerance is positive; lookup time exceeds segment stop by less than the tolerance, and is close enough to the last epoch of the last mini-segment for data to be found.", ctx)?;

    //
    // Re-generate data for the last mini-segment.
    //
    save.MNSGNO = 4;
    save.BEGREC = (((save.NPKTS[1] + save.NPKTS[2]) + save.NPKTS[3]) + 1);
    save.PKTSIZ = C06PS3;

    save.N = save.NPKTS[4];

    T_GENC06(
        save.SEGNO,
        save.MNSGNO,
        save.BEGREC,
        save.N,
        &save.XREF,
        save.RATES[4],
        save.EPOCHS.as_slice_mut(),
        save.QUATS.as_slice_mut(),
        save.AVVS.as_slice_mut(),
        OK,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.FIRST = save.EPOCHS[1];
    save.LAST = (save.EPOCHS[save.N] + 1.0);

    save.CKTOL = 2.0;
    save.T = ((save.LAST + (save.CKTOL / 2 as f64)) - 0.000001);

    spicelib::CKGPAV(
        save.XINST,
        save.T,
        save.CKTOL,
        &save.XREF,
        save.CMAT.as_slice_mut(),
        save.AV.as_slice_mut(),
        &mut save.CLKOUT,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // The data should be those for the last epoch.
    //
    testutil::CHCKSD(
        b"CLKOUT",
        save.CLKOUT,
        b"=",
        save.EPOCHS[save.N],
        0.0,
        OK,
        ctx,
    )?;

    spicelib::VEQU(save.AVVS.subarray([1, save.N]), save.XAV.as_slice_mut());

    testutil::CHCKAD(
        b"AV",
        save.AV.as_slice(),
        b"~~/",
        save.XAV.as_slice(),
        3,
        TIGHT,
        OK,
        ctx,
    )?;

    spicelib::MOVED(save.QUATS.subarray([0, save.N]), 4, save.XQ.as_slice_mut());

    spicelib::Q2M(save.XQ.as_slice(), save.XCMAT.as_slice_mut());

    testutil::CHCKAD(
        b"CMAT",
        save.CMAT.as_slice(),
        b"~~/",
        save.XCMAT.as_slice(),
        9,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // Make sure a second look-up produces the same results.
    //
    spicelib::CKGPAV(
        save.XINST,
        save.T,
        save.CKTOL,
        &save.XREF,
        save.CMAT.as_slice_mut(),
        save.AV.as_slice_mut(),
        &mut save.CLKOUT,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // The data should be those for the last epoch.
    //
    testutil::CHCKSD(
        b"CLKOUT",
        save.CLKOUT,
        b"=",
        save.EPOCHS[save.N],
        0.0,
        OK,
        ctx,
    )?;

    spicelib::VEQU(save.AVVS.subarray([1, save.N]), save.XAV.as_slice_mut());

    testutil::CHCKAD(
        b"AV",
        save.AV.as_slice(),
        b"~~/",
        save.XAV.as_slice(),
        3,
        TIGHT,
        OK,
        ctx,
    )?;

    spicelib::MOVED(save.QUATS.subarray([0, save.N]), 4, save.XQ.as_slice_mut());

    spicelib::Q2M(save.XQ.as_slice(), save.XCMAT.as_slice_mut());

    testutil::CHCKAD(
        b"CMAT",
        save.CMAT.as_slice(),
        b"~~/",
        save.XCMAT.as_slice(),
        9,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Lookup time is gap midpoint; tolerance is zero", ctx)?;

    save.XINST = -1000;
    save.XAVFLG = true;
    fstr::assign(&mut save.XREF, b"GALACTIC");
    save.NINTVL = 4;
    save.SELLST = true;
    save.SEGNO = 1;

    //
    // Re-generate data for the third mini-segment.
    //
    save.MNSGNO = 3;
    save.BEGREC = ((save.NPKTS[1] + save.NPKTS[2]) + 1);
    save.PKTSIZ = C06PS2;

    save.N = save.NPKTS[3];

    T_GENC06(
        save.SEGNO,
        save.MNSGNO,
        save.BEGREC,
        save.N,
        &save.XREF,
        save.RATES[3],
        save.EPOCHS.as_slice_mut(),
        save.QUATS.as_slice_mut(),
        save.AVVS.as_slice_mut(),
        OK,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Let T be the midpoint of the gap at the end of the
    // third mini-segment.
    //
    save.T = (save.EPOCHS[save.N] + 0.5);
    save.CKTOL = 0.0;

    spicelib::CKGPAV(
        save.XINST,
        save.T,
        save.CKTOL,
        &save.XREF,
        save.CMAT.as_slice_mut(),
        save.AV.as_slice_mut(),
        &mut save.CLKOUT,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Lookup time is gap midpoint; tolerance is positive but too small to find pointing.",
        ctx,
    )?;

    save.T = (save.EPOCHS[save.N] + 0.5);
    save.CKTOL = 0.001;

    spicelib::CKGPAV(
        save.XINST,
        save.T,
        save.CKTOL,
        &save.XREF,
        save.CMAT.as_slice_mut(),
        save.AV.as_slice_mut(),
        &mut save.CLKOUT,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Lookup time is in gap and is before gap midpoint; tolerance is 1/2 gap length. Mini-segment is not the last.", ctx)?;

    save.CKTOL = 0.5;

    save.T = ((save.EPOCHS[save.N] + 0.5) - 0.000001);

    spicelib::CKGPAV(
        save.XINST,
        save.T,
        save.CKTOL,
        &save.XREF,
        save.CMAT.as_slice_mut(),
        save.AV.as_slice_mut(),
        &mut save.CLKOUT,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // The data should be those for the final epoch.
    //
    testutil::CHCKSD(
        b"CLKOUT",
        save.CLKOUT,
        b"=",
        save.EPOCHS[save.N],
        0.0,
        OK,
        ctx,
    )?;

    spicelib::VEQU(save.AVVS.subarray([1, save.N]), save.XAV.as_slice_mut());

    testutil::CHCKAD(
        b"AV",
        save.AV.as_slice(),
        b"~~/",
        save.XAV.as_slice(),
        3,
        TIGHT,
        OK,
        ctx,
    )?;

    spicelib::MOVED(save.QUATS.subarray([0, save.N]), 4, save.XQ.as_slice_mut());

    spicelib::Q2M(save.XQ.as_slice(), save.XCMAT.as_slice_mut());

    testutil::CHCKAD(
        b"CMAT",
        save.CMAT.as_slice(),
        b"~~/",
        save.XCMAT.as_slice(),
        9,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // Make sure a second look-up produces the same results.
    //
    spicelib::CKGPAV(
        save.XINST,
        save.T,
        save.CKTOL,
        &save.XREF,
        save.CMAT.as_slice_mut(),
        save.AV.as_slice_mut(),
        &mut save.CLKOUT,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // The data should be those for the final epoch.
    //
    testutil::CHCKSD(
        b"CLKOUT",
        save.CLKOUT,
        b"=",
        save.EPOCHS[save.N],
        0.0,
        OK,
        ctx,
    )?;

    spicelib::VEQU(save.AVVS.subarray([1, save.N]), save.XAV.as_slice_mut());

    testutil::CHCKAD(
        b"AV",
        save.AV.as_slice(),
        b"~~/",
        save.XAV.as_slice(),
        3,
        TIGHT,
        OK,
        ctx,
    )?;

    spicelib::MOVED(save.QUATS.subarray([0, save.N]), 4, save.XQ.as_slice_mut());

    spicelib::Q2M(save.XQ.as_slice(), save.XCMAT.as_slice_mut());

    testutil::CHCKAD(
        b"CMAT",
        save.CMAT.as_slice(),
        b"~~/",
        save.XCMAT.as_slice(),
        9,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Lookup time is in gap and is after gap midpoint; tolerance is 1/2 gap length.",
        ctx,
    )?;

    save.CKTOL = 0.5;

    save.T = ((save.EPOCHS[save.N] + 0.5) + 0.000001);

    spicelib::CKGPAV(
        save.XINST,
        save.T,
        save.CKTOL,
        &save.XREF,
        save.CMAT.as_slice_mut(),
        save.AV.as_slice_mut(),
        &mut save.CLKOUT,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // The data should be those for the first epoch of the
    // fourth mini-segment. Re-generate data for that mini-segment.
    //
    save.MNSGNO = 4;
    save.BEGREC = (((save.NPKTS[1] + save.NPKTS[2]) + save.NPKTS[3]) + 1);
    save.PKTSIZ = C06PS3;

    save.N = save.NPKTS[4];

    T_GENC06(
        save.SEGNO,
        save.MNSGNO,
        save.BEGREC,
        save.N,
        &save.XREF,
        save.RATES[4],
        save.EPOCHS.as_slice_mut(),
        save.QUATS.as_slice_mut(),
        save.AVVS.as_slice_mut(),
        OK,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"CLKOUT", save.CLKOUT, b"=", save.EPOCHS[1], 0.0, OK, ctx)?;

    spicelib::VEQU(save.AVVS.subarray([1, 1]), save.XAV.as_slice_mut());

    testutil::CHCKAD(
        b"AV",
        save.AV.as_slice(),
        b"~~/",
        save.XAV.as_slice(),
        3,
        TIGHT,
        OK,
        ctx,
    )?;

    spicelib::MOVED(save.QUATS.subarray([0, 1]), 4, save.XQ.as_slice_mut());

    spicelib::Q2M(save.XQ.as_slice(), save.XCMAT.as_slice_mut());

    testutil::CHCKAD(
        b"CMAT",
        save.CMAT.as_slice(),
        b"~~/",
        save.XCMAT.as_slice(),
        9,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // Make sure a second look-up produces the same results.
    //
    spicelib::CKGPAV(
        save.XINST,
        save.T,
        save.CKTOL,
        &save.XREF,
        save.CMAT.as_slice_mut(),
        save.AV.as_slice_mut(),
        &mut save.CLKOUT,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // The data should be those for the first epoch of the
    // fourth mini-segment. Re-generate data for that mini-segment.
    //
    save.MNSGNO = 4;
    save.BEGREC = (((save.NPKTS[1] + save.NPKTS[2]) + save.NPKTS[3]) + 1);
    save.PKTSIZ = C06PS3;

    save.N = save.NPKTS[4];

    testutil::CHCKSD(b"CLKOUT", save.CLKOUT, b"=", save.EPOCHS[1], 0.0, OK, ctx)?;

    spicelib::VEQU(save.AVVS.subarray([1, 1]), save.XAV.as_slice_mut());

    testutil::CHCKAD(
        b"AV",
        save.AV.as_slice(),
        b"~~/",
        save.XAV.as_slice(),
        3,
        TIGHT,
        OK,
        ctx,
    )?;

    spicelib::MOVED(save.QUATS.subarray([0, 1]), 4, save.XQ.as_slice_mut());

    spicelib::Q2M(save.XQ.as_slice(), save.XCMAT.as_slice_mut());

    testutil::CHCKAD(
        b"CMAT",
        save.CMAT.as_slice(),
        b"~~/",
        save.XCMAT.as_slice(),
        9,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Lookup time is in gap and is before gap midpoint in last mini-segment; tolerance is 1/2 gap length.", ctx)?;

    //
    // The data should be those for the last epoch of the
    // fourth mini-segment. Re-generate data for that mini-segment.
    //
    save.MNSGNO = 4;
    save.BEGREC = (((save.NPKTS[1] + save.NPKTS[2]) + save.NPKTS[3]) + 1);
    save.PKTSIZ = C06PS3;

    save.N = save.NPKTS[4];

    T_GENC06(
        save.SEGNO,
        save.MNSGNO,
        save.BEGREC,
        save.N,
        &save.XREF,
        save.RATES[4],
        save.EPOCHS.as_slice_mut(),
        save.QUATS.as_slice_mut(),
        save.AVVS.as_slice_mut(),
        OK,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.T = ((save.EPOCHS[save.N] + 0.5) - 0.000001);

    spicelib::CKGPAV(
        save.XINST,
        save.T,
        save.CKTOL,
        &save.XREF,
        save.CMAT.as_slice_mut(),
        save.AV.as_slice_mut(),
        &mut save.CLKOUT,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    testutil::CHCKSD(
        b"CLKOUT",
        save.CLKOUT,
        b"=",
        save.EPOCHS[save.N],
        0.0,
        OK,
        ctx,
    )?;

    spicelib::VEQU(save.AVVS.subarray([1, save.N]), save.XAV.as_slice_mut());

    testutil::CHCKAD(
        b"AV",
        save.AV.as_slice(),
        b"~~/",
        save.XAV.as_slice(),
        3,
        TIGHT,
        OK,
        ctx,
    )?;

    spicelib::MOVED(save.QUATS.subarray([0, save.N]), 4, save.XQ.as_slice_mut());

    spicelib::Q2M(save.XQ.as_slice(), save.XCMAT.as_slice_mut());

    testutil::CHCKAD(
        b"CMAT",
        save.CMAT.as_slice(),
        b"~~/",
        save.XCMAT.as_slice(),
        9,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Lookup time is in gap and is after gap midpoint in last mini-segment; tolerance is 1/2 gap length.", ctx)?;

    //
    // The data should be those for the last epoch of the
    // fourth mini-segment. Re-generate data for that mini-segment.
    //
    save.MNSGNO = 4;
    save.BEGREC = (((save.NPKTS[1] + save.NPKTS[2]) + save.NPKTS[3]) + 1);
    save.PKTSIZ = C06PS3;

    save.N = save.NPKTS[4];

    T_GENC06(
        save.SEGNO,
        save.MNSGNO,
        save.BEGREC,
        save.N,
        &save.XREF,
        save.RATES[4],
        save.EPOCHS.as_slice_mut(),
        save.QUATS.as_slice_mut(),
        save.AVVS.as_slice_mut(),
        OK,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.T = ((save.EPOCHS[save.N] + 0.5) + 0.000001);

    spicelib::CKGPAV(
        save.XINST,
        save.T,
        save.CKTOL,
        &save.XREF,
        save.CMAT.as_slice_mut(),
        save.AV.as_slice_mut(),
        &mut save.CLKOUT,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    spicelib::UNLOAD(CK06MU, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*****************************************************************
    //*****************************************************************
    //*
    //*     CKR06: create CKs for search tests
    //*
    //*****************************************************************
    //*****************************************************************

    //$$$

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create CK for packet selection logic tests.", ctx)?;

    //
    // Create CKs for testing.
    //
    // The first kernel contains three mini-segments, all for the
    // same instrument. The packet counts in these mini-segments
    // are such that the first one contains no packet directories,
    // the second contains fewer than one directory buffer full,
    // and the third contains too many to be buffered.
    //
    // The selection flag indicates "select last."
    //
    fstr::assign(&mut save.CK, CK06B0);

    save.XINST = -1000;
    fstr::assign(&mut save.XREF, b"GALACTIC");
    fstr::assign(&mut save.CKFRAM, b"IAU_EARTH");
    save.SEGNO = 1;
    save.SELLST = true;

    save.NINTVL = 3;

    save.NPKTS[1] = 98;
    save.NPKTS[2] = 398;
    save.NPKTS[3] = 10381;

    save.SUBTPS[1] = C06TP0;
    save.SUBTPS[2] = C06TP1;
    save.SUBTPS[3] = C06TP3;

    save.PSIZES[1] = C06PS0;
    save.PSIZES[2] = C06PS1;
    save.PSIZES[3] = C06PS3;

    save.DEGRES[1] = 7;
    save.DEGRES[2] = 3;
    save.DEGRES[3] = 23;

    save.RATES[1] = 10.0;
    save.RATES[2] = 2.0;
    save.RATES[3] = 3.0;

    save.PAD = 3;

    //
    // Create time tags and packets.
    //
    save.BEGREC = 1;
    save.N = 1;
    save.PKTPTR = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NINTVL;
        let m3__: i32 = 1;
        save.MNSGNO = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Record the index of the first non-pad time tag of
            // the current mini-segment.
            //
            save.DATIDX[save.MNSGNO] = (save.N + save.PAD);

            //
            // Record the corresponding start index in the packet array.
            //
            save.PKTBEG[save.MNSGNO] = save.PKTPTR;

            //
            // Generate time tags and packet data for the current
            // mini-segment.
            //
            save.RATE = save.RATES[save.MNSGNO];
            save.SUBTYP = save.SUBTPS[save.MNSGNO];

            T_GENCSM(
                save.SEGNO,
                save.MNSGNO,
                save.BEGREC,
                save.NPKTS[save.MNSGNO],
                &save.XREF,
                save.RATE,
                save.SUBTYP,
                save.EPOCHS.as_slice_mut(),
                save.QUATS.as_slice_mut(),
                save.AVVS.as_slice_mut(),
                OK,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // The start time of the current interval is the epoch at
            // PAD positions later than the start epoch.
            //
            save.XIVBDS[save.MNSGNO] = save.EPOCHS[(1 + save.PAD)];

            //
            // Setting up the data arrays for CKW06 is simple: we simply
            // append the data for each mini-segment.
            //
            spicelib::MOVED(
                save.EPOCHS.as_slice(),
                save.NPKTS[save.MNSGNO],
                save.SCLKDP.subarray_mut(save.N),
            );

            spicelib::MOVED(
                save.AVVS.as_slice(),
                (3 * save.NPKTS[save.MNSGNO]),
                save.XAVVS.subarray_mut([1, save.N]),
            );
            spicelib::MOVED(
                save.QUATS.as_slice(),
                (4 * save.NPKTS[save.MNSGNO]),
                save.XQUATS.subarray_mut([0, save.N]),
            );

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NPKTS[save.MNSGNO];
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    spicelib::Q2M(
                        save.QUATS.subarray([0, save.J]),
                        save.XCMATS.subarray_mut([1, ((save.N - 1) + save.J)]),
                    );
                    save.J += m3__;
                }
            }

            //
            // We're not using angular acceleration.
            //
            spicelib::CLEARD(3, save.DAVV.as_slice_mut());

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NPKTS[save.MNSGNO];
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    T_XSUBTP(
                        save.QUATS.subarray([0, save.J]),
                        save.AVVS.subarray([1, save.J]),
                        save.DAVV.as_slice(),
                        save.SUBTPS[save.MNSGNO],
                        save.PSIZES[save.MNSGNO],
                        save.RECORD.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::MOVED(
                        save.RECORD.as_slice(),
                        save.PSIZES[save.MNSGNO],
                        save.PACKTS.subarray_mut(save.PKTPTR),
                    );

                    save.PKTPTR = (save.PKTPTR + save.PSIZES[save.MNSGNO]);

                    save.J += m3__;
                }
            }

            //
            // Adjust the starting record number of the next mini-segment
            // to account for padding.
            //
            save.BEGREC = (((save.BEGREC - 1) + save.NPKTS[save.MNSGNO]) - (2 * save.PAD));

            //
            // Update the count of the records in the segment.
            //
            save.N = (save.N + save.NPKTS[save.MNSGNO]);

            save.MNSGNO += m3__;
        }
    }

    //
    // The stop time of the last interval is the epoch at
    // PAD positions before the final epoch.
    //
    save.XIVBDS[(save.NINTVL + 1)] = save.SCLKDP[(save.N - save.PAD)];

    //
    // Let the segment bounds coincide with the start and stop
    // times of the first and last mini-segments.
    //
    save.FIRST = save.XIVBDS[1];
    save.LAST = save.XIVBDS[(save.NINTVL + 1)];

    //
    // Create the CK.
    //
    if spicelib::EXISTS(&save.CK, ctx)? {
        spicelib::DELFIL(&save.CK, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::CKOPN(&save.CK, &save.CK, 0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CKW06(
        save.HANDLE,
        save.XINST,
        &save.XREF,
        save.XAVFLG,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.NINTVL,
        save.NPKTS.as_slice(),
        save.SUBTPS.as_slice(),
        save.DEGRES.as_slice(),
        save.PACKTS.as_slice(),
        save.RATES.as_slice(),
        save.SCLKDP.as_slice(),
        save.XIVBDS.as_slice(),
        save.SELLST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*****************************************************************
    //*
    //*     CKR06 reduction of order tests
    //*
    //*****************************************************************

    //
    // These tests examine the behavior of the CKR06 interpolation
    // window construction algorithm for request times near mini-segment
    // interpolation interval boundaries.
    //
    // We'll work with the third mini-segment of CK06B0. We start by
    // looking up the relevant mini-segment attributes:
    //
    //    - Interpolation interval bounds
    //    - Window size
    //    - Packet count
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Set up CKR06 reduction of order tests.", ctx)?;

    fstr::assign(&mut save.CK, CK06B0);

    spicelib::DAFOPR(&save.CK, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MNSGNO = 3;
    spicelib::CKMP06(
        save.HANDLE,
        save.DESCR.as_slice(),
        save.MNSGNO,
        &mut save.RATE,
        &mut save.SUBTYP,
        &mut save.WINSIZ,
        &mut save.NREC,
        save.IVLBDS.as_slice_mut(),
        &mut save.LSTEPC,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.PKTSIZ = save.PKTSZS[save.SUBTYP];

    //
    // Determine the left hand pad size.
    //
    save.PAD = 0;
    save.I = 1;

    spicelib::CKGR06(
        save.HANDLE,
        save.DESCR.as_slice(),
        save.MNSGNO,
        save.I,
        save.RECORD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.T = save.RECORD[1];

    while (save.T < save.IVLBDS[1]) {
        save.PAD = (save.PAD + 1);
        save.I = (save.I + 1);

        spicelib::CKGR06(
            save.HANDLE,
            save.DESCR.as_slice(),
            save.MNSGNO,
            save.I,
            save.RECORD.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        save.T = save.RECORD[1];
    }

    //
    // See whether CKR06 returns the expected window for
    // request times near the left boundary of the interval.
    //
    {
        let m1__: i32 = (1 + (save.WINSIZ / 2));
        let m2__: i32 = (save.PAD + 2);
        let m3__: i32 = -1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // --- Case: ------------------------------------------------------
            //
            fstr::assign(&mut save.TITLE, b"CK06B0: Reduction of order on left side. Request time is between tags at indices # and #.");

            spicelib::REPMI(
                &save.TITLE.to_vec(),
                b"#",
                (save.I - 1),
                &mut save.TITLE,
                ctx,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::TCASE(&save.TITLE, ctx)?;

            //
            // Construct the expected record for a request time
            // between time tags at indices I-1 and I.
            //
            //
            // N is the expected packet count.
            //
            save.N = ((save.I + (save.WINSIZ / 2)) - 1);

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.N;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    spicelib::CKGR06(
                        save.HANDLE,
                        save.DESCR.as_slice(),
                        save.MNSGNO,
                        save.J,
                        save.RECORD.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.K = ((4 + ((save.J - 1) * save.PKTSIZ)) + 1);
                    //
                    // Insert packet data into the expected record.
                    //
                    spicelib::MOVED(
                        save.RECORD.subarray(4),
                        save.PKTSIZ,
                        save.XREC.subarray_mut(save.K),
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    //
                    // Insert the time tag into the expected record.
                    //
                    save.K = ((4 + (save.N * save.PKTSIZ)) + save.J);

                    save.XREC[save.K] = save.RECORD[1];

                    save.J += m3__;
                }
            }

            //
            // Get the time tags that bracket the request time we
            // want to use.
            //
            save.K = ((4 + (save.N * save.PKTSIZ)) + (save.I - 1));

            save.T0 = save.XREC[save.K];
            save.T1 = save.XREC[(save.K + 1)];

            //
            // Generate the request time.
            //
            save.T = ((save.T0 + save.T1) / 2 as f64);

            //
            // Insert the initial parameters into the expected record.
            //
            save.XREC[1] = save.T;
            save.XREC[2] = save.SUBTYP as f64;
            save.XREC[3] = save.N as f64;
            save.XREC[4] = save.RATE;

            //
            // Look up the record for request time SCLKDP.
            //
            spicelib::CKR06(
                save.HANDLE,
                save.DESCR.as_slice(),
                save.T,
                0.0,
                true,
                save.RECORD.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

            fstr::assign(&mut save.NAME, b"Record # packet count");
            spicelib::REPMI(&save.NAME.to_vec(), b"#", save.I, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSI(
                &save.NAME,
                intrinsics::IDNINT(save.RECORD[3]),
                b"=",
                save.N,
                0,
                OK,
                ctx,
            )?;

            if *OK {
                fstr::assign(&mut save.NAME, b"Record #");
                spicelib::REPMI(&save.NAME.to_vec(), b"#", save.I, &mut save.NAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.SIZE = (4 + (save.N * (save.PKTSIZ + 1)));

                testutil::CHCKAD(
                    &save.NAME,
                    save.RECORD.as_slice(),
                    b"=",
                    save.XREC.as_slice(),
                    save.SIZE,
                    0.0,
                    OK,
                    ctx,
                )?;
            }

            save.I += m3__;
        }
    }

    //
    //@@@
    //

    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Determine the right hand pad size.", ctx)?;

    save.PAD = 0;
    save.I = save.NREC;

    spicelib::CKGR06(
        save.HANDLE,
        save.DESCR.as_slice(),
        save.MNSGNO,
        save.I,
        save.RECORD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.T = save.RECORD[1];

    while (save.T > save.IVLBDS[2]) {
        save.PAD = (save.PAD + 1);
        save.I = (save.I - 1);

        spicelib::CKGR06(
            save.HANDLE,
            save.DESCR.as_slice(),
            save.MNSGNO,
            save.I,
            save.RECORD.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        save.T = save.RECORD[1];
    }

    //
    // See whether CKR06 returns the expected window for
    // request times near the right boundary of the interval.
    //
    {
        let m1__: i32 = ((save.NREC - (save.WINSIZ / 2)) + 1);
        let m2__: i32 = (save.NREC - save.PAD);
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // --- Case: ------------------------------------------------------
            //
            //
            //        Let M represent the loop iteration count. This is the
            //        offset of I from the value preceding the loop start
            //        value.
            //
            save.M = (save.I - (save.NREC - (save.WINSIZ / 2)));

            fstr::assign(&mut save.TITLE, b"CK06B0: Reduction of order on left side. Request time is between tags at indices # and #. Iteration = #.");

            spicelib::REPMI(
                &save.TITLE.to_vec(),
                b"#",
                (save.I - 1),
                &mut save.TITLE,
                ctx,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.M, &mut save.TITLE, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::TCASE(&save.TITLE, ctx)?;

            //
            // Construct the expected record for a request time
            // between time tags at indices I-1 and I.
            //
            //
            // N is the expected packet count.
            //
            save.N = ((save.WINSIZ + 1) - save.M);

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.N;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.K = ((save.NREC - save.N) + save.J);

                    spicelib::CKGR06(
                        save.HANDLE,
                        save.DESCR.as_slice(),
                        save.MNSGNO,
                        save.K,
                        save.RECORD.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.K = ((4 + ((save.J - 1) * save.PKTSIZ)) + 1);
                    //
                    // Insert packet data into the expected record.
                    //
                    spicelib::MOVED(
                        save.RECORD.subarray(4),
                        save.PKTSIZ,
                        save.XREC.subarray_mut(save.K),
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    //
                    // Insert the time tag into the expected record.
                    //
                    save.K = ((4 + (save.N * save.PKTSIZ)) + save.J);

                    save.XREC[save.K] = save.RECORD[1];

                    save.J += m3__;
                }
            }

            //
            // Get the time tags that bracket the request time we
            // want to use. Note that for this case, the offset
            // is independent of J.
            //
            save.K = ((4 + (save.N * save.PKTSIZ)) + (save.WINSIZ / 2));

            save.T0 = save.XREC[save.K];
            save.T1 = save.XREC[(save.K + 1)];

            //
            // Generate the request time.
            //
            save.T = ((save.T0 + save.T1) / 2 as f64);

            //
            // Insert the initial parameters into the expected record.
            //
            save.XREC[1] = save.T;
            save.XREC[2] = save.SUBTYP as f64;
            save.XREC[3] = save.N as f64;
            save.XREC[4] = save.RATE;

            //
            // Look up the record for request time SCLKDP.
            //
            spicelib::CKR06(
                save.HANDLE,
                save.DESCR.as_slice(),
                save.T,
                0.0,
                true,
                save.RECORD.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

            fstr::assign(&mut save.NAME, b"Record # packet count");
            spicelib::REPMI(&save.NAME.to_vec(), b"#", save.I, &mut save.NAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSI(
                &save.NAME,
                intrinsics::IDNINT(save.RECORD[3]),
                b"=",
                save.N,
                0,
                OK,
                ctx,
            )?;

            if *OK {
                fstr::assign(&mut save.NAME, b"Record #");
                spicelib::REPMI(&save.NAME.to_vec(), b"#", save.I, &mut save.NAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.SIZE = (4 + (save.N * (save.PKTSIZ + 1)));

                testutil::CHCKAD(
                    &save.NAME,
                    save.RECORD.as_slice(),
                    b"=",
                    save.XREC.as_slice(),
                    save.SIZE,
                    0.0,
                    OK,
                    ctx,
                )?;
            }

            save.I += m3__;
        }
    }

    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*****************************************************************
    //*
    //*     CKR06 packet selection logic tests
    //*
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Packet selection logic test: look up data at time tags.",
        ctx,
    )?;

    fstr::assign(&mut save.CK, CK06B0);
    spicelib::FURNSH(&save.CK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CKTOL = 0.0;

    save.BEGREC = 1;
    save.N = 1;
    save.PKTPTR = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NINTVL;
        let m3__: i32 = 1;
        save.MNSGNO = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Create the expected CMAT and AV values.
            //
            save.RATE = save.RATES[save.MNSGNO];
            save.SUBTYP = save.SUBTPS[save.MNSGNO];

            // CALL MOVED ( XAVVS (1,N), 3*NPKTS(MNSGNO), AVVS )

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.J = save.DATIDX[save.MNSGNO];

            // N      = NPKTS (MNSGNO) - 2*PAD

            save.T = save.SCLKDP[save.J];

            while (save.T < save.XIVBDS[(save.MNSGNO + 1)]) {
                spicelib::VEQU(save.XAVVS.subarray([1, save.J]), save.XAV.as_slice_mut());
                spicelib::MOVED(
                    save.XCMATS.subarray([1, save.J]),
                    9,
                    save.XCMAT.as_slice_mut(),
                );

                //
                // Look up pointing data.
                //
                spicelib::CKGPAV(
                    save.XINST,
                    save.T,
                    save.CKTOL,
                    &save.XREF,
                    save.CMAT.as_slice_mut(),
                    save.AV.as_slice_mut(),
                    &mut save.CLKOUT,
                    &mut save.FOUND,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                if !*OK {
                    spicelib::SIGERR(b"NO POINTING", ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    ctx.stop()?;
                }

                testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

                //
                // Check output time.
                //
                fstr::assign(&mut save.NAME, b"Mini-segment *; CLKOUT no. *");
                spicelib::REPMI(&save.NAME.to_vec(), b"*", save.MNSGNO, &mut save.NAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                spicelib::REPMI(&save.NAME.to_vec(), b"*", save.J, &mut save.NAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSD(&save.NAME, save.CLKOUT, b"~", save.T, 0.0, OK, ctx)?;

                //
                // Check C-matrix.
                //

                fstr::assign(&mut save.NAME, b"Mini-segment *; CMAT no. *");
                spicelib::REPMI(&save.NAME.to_vec(), b"*", save.MNSGNO, &mut save.NAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                spicelib::REPMI(&save.NAME.to_vec(), b"*", save.J, &mut save.NAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    &save.NAME,
                    save.CMAT.as_slice(),
                    b"~~/",
                    save.XCMAT.as_slice(),
                    9,
                    VTIGHT,
                    OK,
                    ctx,
                )?;

                //
                // Check AV.
                //
                fstr::assign(&mut save.NAME, b"Mini-segment *; AV no. *");
                spicelib::REPMI(&save.NAME.to_vec(), b"*", save.MNSGNO, &mut save.NAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                spicelib::REPMI(&save.NAME.to_vec(), b"*", save.J, &mut save.NAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Check angular velocity.
                //
                // For derived AV, use a looser tolerance.
                //
                if (save.SUBTPS[save.MNSGNO] == C06TP1) {
                    save.TOL = MTIGHT;

                // TOL = MED
                } else if (save.SUBTPS[save.MNSGNO] == C06TP0) {
                    save.TOL = TIGHT;
                } else if (save.SUBTPS[save.MNSGNO] == C06TP3) {
                    save.TOL = VTIGHT;
                } else {
                    save.TOL = TIGHT;
                }

                testutil::CHCKAD(
                    &save.NAME,
                    save.AV.as_slice(),
                    b"~~/",
                    save.XAV.as_slice(),
                    3,
                    save.TOL,
                    OK,
                    ctx,
                )?;

                if !*OK {
                    ctx.stop()?;
                }

                save.J = (save.J + 1);
                save.PKTPTR = (save.PKTPTR + save.PSIZES[save.MNSGNO]);

                save.T = save.SCLKDP[save.J];
            }
            //
            // Adjust the starting record number of the next mini-segment
            // to account for padding.
            //
            save.BEGREC = (((save.BEGREC - 1) + save.NPKTS[save.MNSGNO]) - (2 * save.PAD));

            save.N = (save.N + save.NPKTS[save.MNSGNO]);

            save.MNSGNO += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Packet selection logic test: look up data at midpoints between time tags.",
        ctx,
    )?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NINTVL;
        let m3__: i32 = 1;
        save.MNSGNO = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Create the expected CMAT and AV values.
            //
            save.RATE = save.RATES[save.MNSGNO];
            save.SUBTYP = save.SUBTPS[save.MNSGNO];

            save.J = save.DATIDX[save.MNSGNO];
            save.PKTPTR = (save.PKTBEG[save.MNSGNO] + (save.PAD * save.PSIZES[save.MNSGNO]));
            save.N = (save.NPKTS[save.MNSGNO] - (2 * save.PAD));

            if (save.J < save.NPKTS[save.MNSGNO]) {
                save.T = ((save.SCLKDP[save.J] + save.SCLKDP[(save.J + 1)]) / 2 as f64);
            } else {
                save.T = (save.LAST + 1.0);
            }

            while (save.T < save.XIVBDS[(save.MNSGNO + 1)]) {
                //
                // Create expected CMAT and AV. This will only be
                // an approximation.
                //
                save.ET = (save.T * save.RATE);

                spicelib::SXFORM(
                    &save.XREF,
                    &save.CKFRAM,
                    save.ET,
                    save.XFORM.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::XF2RAV(
                    save.XFORM.as_slice(),
                    save.XCMAT.as_slice_mut(),
                    save.XAV.as_slice_mut(),
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::M2Q(save.XCMAT.as_slice(), save.XQ.as_slice_mut(), ctx)?;

                if (save.XQ[0] < 0.0) {
                    {
                        let m1__: i32 = 0;
                        let m2__: i32 = 3;
                        let m3__: i32 = 1;
                        save.I = m1__;
                        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                            save.XQ[save.I] = -save.XQ[save.I];
                            save.I += m3__;
                        }
                    }
                }

                //
                // Look up pointing data.
                //
                spicelib::CKGPAV(
                    save.XINST,
                    save.T,
                    save.CKTOL,
                    &save.XREF,
                    save.CMAT.as_slice_mut(),
                    save.AV.as_slice_mut(),
                    &mut save.CLKOUT,
                    &mut save.FOUND,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                if !*OK {
                    spicelib::SIGERR(b"NO POINTING", ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    ctx.stop()?;
                }

                testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

                //
                // Check output time.
                //
                fstr::assign(&mut save.NAME, b"Mini-segment *; CLKOUT no. *");
                spicelib::REPMI(&save.NAME.to_vec(), b"*", save.MNSGNO, &mut save.NAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                spicelib::REPMI(&save.NAME.to_vec(), b"*", save.J, &mut save.NAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSD(&save.NAME, save.CLKOUT, b"~", save.T, 0.0, OK, ctx)?;

                //
                // Check C-matrix.
                //
                spicelib::Q2M(save.XQ.as_slice(), save.XCMAT.as_slice_mut());
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                fstr::assign(&mut save.NAME, b"Mini-segment *; CMAT no. *");
                spicelib::REPMI(&save.NAME.to_vec(), b"*", save.MNSGNO, &mut save.NAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                spicelib::REPMI(&save.NAME.to_vec(), b"*", save.J, &mut save.NAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Use a tight tolerance for CMAT midpoint comparisons.
                //
                testutil::CHCKAD(
                    &save.NAME,
                    save.CMAT.as_slice(),
                    b"~~/",
                    save.XCMAT.as_slice(),
                    9,
                    VTIGHT,
                    OK,
                    ctx,
                )?;
                //
                // Check angular velocity.
                //
                fstr::assign(&mut save.NAME, b"Mini-segment *; AV no. *");
                spicelib::REPMI(&save.NAME.to_vec(), b"*", save.MNSGNO, &mut save.NAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                spicelib::REPMI(&save.NAME.to_vec(), b"*", save.J, &mut save.NAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Set the tolerance for the AV check.
                //
                if (save.SUBTPS[save.MNSGNO] == C06TP1) {
                    //
                    // For derived AV, use a looser tolerance.
                    //
                    save.TOL = MED;
                } else if (save.SUBTPS[save.MNSGNO] == C06TP0) {
                    save.TOL = TIGHT;
                } else if (save.SUBTPS[save.MNSGNO] == C06TP3) {
                    save.TOL = VTIGHT;
                } else {
                    save.TOL = TIGHT;
                }

                testutil::CHCKAD(
                    &save.NAME,
                    save.AV.as_slice(),
                    b"~~/",
                    save.XAV.as_slice(),
                    3,
                    save.TOL,
                    OK,
                    ctx,
                )?;

                save.J = (save.J + 1);
                save.PKTPTR = (save.PKTPTR + save.PSIZES[save.MNSGNO]);

                if (save.J < save.NPKTS[save.MNSGNO]) {
                    save.T = ((save.SCLKDP[save.J] + save.SCLKDP[(save.J + 1)]) / 2 as f64);
                } else {
                    save.T = (save.LAST + 1.0);
                }
            }

            save.MNSGNO += m3__;
        }
    }

    spicelib::UNLOAD(&save.CK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*****************************************************************
    //*
    //*     CKR06 interval selection logic tests
    //*
    //*****************************************************************

    //
    // $$$
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create CKs for interval selection logic tests.", ctx)?;

    //
    // Create CKs for testing.
    //
    // The first kernel contains three segments for the three different
    // instruments. The interval counts in these segments
    // are such that the first one contains no interval directories,
    // the second contains fewer than one directory buffer full,
    // and the third contains too many to be buffered.
    //
    // The selection flag for the first CK is set to indicate
    // "select first."
    //
    fstr::assign(&mut save.CK, CK06B1);

    //
    // Create the CK.
    //
    if spicelib::EXISTS(&save.CK, ctx)? {
        spicelib::DELFIL(&save.CK, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::CKOPN(&save.CK, &save.CK, 0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NSEG = 3;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.XINSTS[save.I] = -(1000 * save.I);
            save.I += m3__;
        }
    }

    fstr::assign(save.XREFS.get_mut(1), b"GALACTIC");
    fstr::assign(save.XREFS.get_mut(2), b"MARSIAU");
    fstr::assign(save.XREFS.get_mut(3), b"GALACTIC");
    fstr::assign(&mut save.CKFRAM, b"IAU_EARTH");

    save.SELLST = false;

    //
    // Set the interval counts for the three segments.
    //
    save.NIVALS[1] = 98;
    save.NIVALS[2] = 398;
    save.NIVALS[3] = 10381;

    //
    // Create data for the segments and write them to the file.
    //
    save.N = 0;
    save.SEGPTR = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG;
        let m3__: i32 = 1;
        save.SEGNO = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.SEGPTR = (save.SEGPTR + save.N);
            //
            // Set the mini-segment attributes for the current segment.
            //
            save.XINST = save.XINSTS[save.SEGNO];
            save.NINTVL = save.NIVALS[save.SEGNO];
            fstr::assign(&mut save.XREF, save.XREFS.get(save.SEGNO));

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NINTVL;
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    if (intrinsics::MOD(save.I, 3) == 0) {
                        save.SUBTPS[save.I] = C06TP3;
                        save.PSIZES[save.I] = C06PS3;
                        save.DEGRES[save.I] = 3;
                        save.RATES[save.I] = 1.1;
                        save.NPKTS[save.I] = 5;
                    } else if (intrinsics::MOD(save.I, 3) == 1) {
                        save.SUBTPS[save.I] = C06TP1;
                        save.PSIZES[save.I] = C06PS1;
                        save.DEGRES[save.I] = 7;
                        save.RATES[save.I] = 0.8;
                        save.NPKTS[save.I] = 9;
                    } else {
                        save.SUBTPS[save.I] = C06TP0;
                        save.PSIZES[save.I] = C06PS0;
                        save.DEGRES[save.I] = 7;
                        save.RATES[save.I] = 4.0;
                        save.NPKTS[save.I] = 11;
                    }

                    save.I += m3__;
                }
            }

            save.PAD = 1;

            //
            // Create time tags and packets for the current segment.
            //
            save.BEGREC = 1;
            save.N = 1;
            save.PKTPTR = 1;

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NINTVL;
                let m3__: i32 = 1;
                save.MNSGNO = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    //
                    // Record the index of the first non-pad time tag of
                    // the current mini-segment.
                    //
                    save.DATIDX[save.MNSGNO] = (save.N + save.PAD);

                    //
                    // Record the corresponding start index in the packet array.
                    //
                    save.PKTBEG[save.MNSGNO] = save.PKTPTR;

                    //
                    // Generate time tags and packet data for the current
                    // mini-segment.
                    //
                    save.RATE = save.RATES[save.MNSGNO];
                    save.SUBTYP = save.SUBTPS[save.MNSGNO];

                    T_GENCSM(
                        save.SEGNO,
                        save.MNSGNO,
                        save.BEGREC,
                        save.NPKTS[save.MNSGNO],
                        &save.XREF,
                        save.RATE,
                        save.SUBTYP,
                        save.EPOCHS.as_slice_mut(),
                        save.QUATS.as_slice_mut(),
                        save.AVVS.as_slice_mut(),
                        OK,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // The start time of the current interval is the epoch at
                    // PAD positions later than the start epoch.
                    //
                    save.XIVBDS[save.MNSGNO] = save.EPOCHS[(1 + save.PAD)];

                    //
                    // Setting up the data arrays for CKW06 is simple: we simply
                    // append the data for each mini-segment.
                    //
                    spicelib::MOVED(
                        save.EPOCHS.as_slice(),
                        save.NPKTS[save.MNSGNO],
                        save.SCLKDP.subarray_mut(save.N),
                    );

                    spicelib::MOVED(
                        save.EPOCHS.as_slice(),
                        save.NPKTS[save.MNSGNO],
                        save.XEPOCH.subarray_mut((save.SEGPTR + save.N)),
                    );
                    spicelib::MOVED(
                        save.AVVS.as_slice(),
                        (3 * save.NPKTS[save.MNSGNO]),
                        save.XAVVS.subarray_mut([1, (save.SEGPTR + save.N)]),
                    );
                    spicelib::MOVED(
                        save.QUATS.as_slice(),
                        (4 * save.NPKTS[save.MNSGNO]),
                        save.XQUATS.subarray_mut([0, (save.SEGPTR + save.N)]),
                    );

                    {
                        let m1__: i32 = 1;
                        let m2__: i32 = save.NPKTS[save.MNSGNO];
                        let m3__: i32 = 1;
                        save.J = m1__;
                        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                            spicelib::Q2M(
                                save.QUATS.subarray([0, save.J]),
                                save.XCMATS
                                    .subarray_mut([1, (((save.SEGPTR + save.N) + save.J) - 1)]),
                            );
                            save.J += m3__;
                        }
                    }

                    //
                    // We're not using angular acceleration.
                    //
                    spicelib::CLEARD(3, save.DAVV.as_slice_mut());

                    {
                        let m1__: i32 = 1;
                        let m2__: i32 = save.NPKTS[save.MNSGNO];
                        let m3__: i32 = 1;
                        save.J = m1__;
                        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                            T_XSUBTP(
                                save.QUATS.subarray([0, save.J]),
                                save.AVVS.subarray([1, save.J]),
                                save.DAVV.as_slice(),
                                save.SUBTPS[save.MNSGNO],
                                save.PSIZES[save.MNSGNO],
                                save.RECORD.as_slice_mut(),
                                ctx,
                            )?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            spicelib::MOVED(
                                save.RECORD.as_slice(),
                                save.PSIZES[save.MNSGNO],
                                save.PACKTS.subarray_mut(save.PKTPTR),
                            );

                            save.PKTPTR = (save.PKTPTR + save.PSIZES[save.MNSGNO]);

                            save.J += m3__;
                        }
                    }

                    //
                    // Adjust the starting record number of the next mini-segment
                    // to account for padding.
                    //
                    save.BEGREC = (((save.BEGREC - 1) + save.NPKTS[save.MNSGNO]) - (2 * save.PAD));

                    //
                    // Update the count of the records in the segment.
                    //
                    save.N = (save.N + save.NPKTS[save.MNSGNO]);

                    save.MNSGNO += m3__;
                }
            }

            //
            // The stop time of the last interval is the epoch at
            // PAD positions before the final epoch.
            //
            save.XIVBDS[(save.NINTVL + 1)] = save.SCLKDP[(save.N - save.PAD)];

            //
            // Let the segment bounds coincide with the start and stop
            // times of the first and last mini-segments.
            //
            save.FIRST = save.XIVBDS[1];
            save.LAST = save.XIVBDS[(save.NINTVL + 1)];

            spicelib::CKW06(
                save.HANDLE,
                save.XINST,
                &save.XREF,
                save.XAVFLG,
                save.FIRST,
                save.LAST,
                &save.SEGID,
                save.NINTVL,
                save.NPKTS.as_slice(),
                save.SUBTPS.as_slice(),
                save.DEGRES.as_slice(),
                save.PACKTS.as_slice(),
                save.RATES.as_slice(),
                save.SCLKDP.as_slice(),
                save.XIVBDS.as_slice(),
                save.SELLST,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.SEGNO += m3__;
        }
    }

    //
    // Close the kernel.
    //
    spicelib::CKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    // The selection flag for the second CK is set to indicate
    // "select last." Other than that, the CK is identical to
    // the first.
    //
    fstr::assign(&mut save.CK, CK06B2);

    //
    // Create the CK.
    //
    if spicelib::EXISTS(&save.CK, ctx)? {
        spicelib::DELFIL(&save.CK, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::CKOPN(&save.CK, &save.CK, 0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    // CALL TOSTDO ( 'Opened CK '//CK )

    save.NSEG = 3;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.XINSTS[save.I] = -(1000 * save.I);
            save.I += m3__;
        }
    }

    fstr::assign(save.XREFS.get_mut(1), b"GALACTIC");
    fstr::assign(save.XREFS.get_mut(2), b"MARSIAU");
    fstr::assign(save.XREFS.get_mut(3), b"GALACTIC");
    fstr::assign(&mut save.CKFRAM, b"IAU_EARTH");

    save.SELLST = true;

    //
    // Set the interval counts for the three segments.
    //
    save.NIVALS[1] = 98;
    save.NIVALS[2] = 398;
    save.NIVALS[3] = 10381;

    //
    // Create data for the segments and write them to the file.
    //
    save.N = 0;
    save.SEGPTR = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG;
        let m3__: i32 = 1;
        save.SEGNO = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.SEGPTR = (save.SEGPTR + save.N);

            //
            // Set the mini-segment attributes for the current segment.
            //
            save.XINST = save.XINSTS[save.SEGNO];
            save.NINTVL = save.NIVALS[save.SEGNO];
            fstr::assign(&mut save.XREF, save.XREFS.get(save.SEGNO));

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NINTVL;
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    if (intrinsics::MOD(save.I, 3) == 0) {
                        save.SUBTPS[save.I] = C06TP3;
                        save.PSIZES[save.I] = C06PS3;
                        save.DEGRES[save.I] = 3;
                        save.RATES[save.I] = 1.1;
                        save.NPKTS[save.I] = 5;
                    } else if (intrinsics::MOD(save.I, 3) == 1) {
                        save.SUBTPS[save.I] = C06TP1;
                        save.PSIZES[save.I] = C06PS1;
                        save.DEGRES[save.I] = 7;
                        save.RATES[save.I] = 0.8;
                        save.NPKTS[save.I] = 9;
                    } else {
                        save.SUBTPS[save.I] = C06TP0;
                        save.PSIZES[save.I] = C06PS0;
                        save.DEGRES[save.I] = 7;
                        save.RATES[save.I] = 4.0;
                        save.NPKTS[save.I] = 11;
                    }

                    save.I += m3__;
                }
            }

            save.PAD = 1;

            //
            // Create time tags and packets for the current segment.
            //
            save.BEGREC = 1;
            save.N = 1;
            save.PKTPTR = 1;

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NINTVL;
                let m3__: i32 = 1;
                save.MNSGNO = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    //
                    // Record the index of the first non-pad time tag of
                    // the current mini-segment.
                    //
                    save.DATIDX[save.MNSGNO] = (save.N + save.PAD);

                    //
                    // Record the corresponding start index in the packet array.
                    //
                    save.PKTBEG[save.MNSGNO] = save.PKTPTR;

                    //
                    // Generate time tags and packet data for the current
                    // mini-segment.
                    //
                    save.RATE = save.RATES[save.MNSGNO];
                    save.SUBTYP = save.SUBTPS[save.MNSGNO];

                    spicelib::MOVED(
                        save.XQUATS.subarray([0, (save.SEGPTR + save.N)]),
                        (4 * save.NPKTS[save.MNSGNO]),
                        save.QUATS.as_slice_mut(),
                    );
                    spicelib::MOVED(
                        save.XAVVS.subarray([1, (save.SEGPTR + save.N)]),
                        (3 * save.NPKTS[save.MNSGNO]),
                        save.AVVS.as_slice_mut(),
                    );
                    spicelib::MOVED(
                        save.XEPOCH.subarray((save.SEGPTR + save.N)),
                        save.NPKTS[save.MNSGNO],
                        save.EPOCHS.as_slice_mut(),
                    );

                    //
                    // The start time of the current interval is the epoch at
                    // PAD positions later than the start epoch.
                    //
                    save.XIVBDS[save.MNSGNO] = save.EPOCHS[(1 + save.PAD)];

                    //
                    // Setting up the data arrays for CKW06 is simple: we simply
                    // append the data for each mini-segment.
                    //
                    spicelib::MOVED(
                        save.EPOCHS.as_slice(),
                        save.NPKTS[save.MNSGNO],
                        save.SCLKDP.subarray_mut(save.N),
                    );

                    //
                    // We're not using angular acceleration.
                    //
                    spicelib::CLEARD(3, save.DAVV.as_slice_mut());

                    {
                        let m1__: i32 = 1;
                        let m2__: i32 = save.NPKTS[save.MNSGNO];
                        let m3__: i32 = 1;
                        save.J = m1__;
                        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                            T_XSUBTP(
                                save.QUATS.subarray([0, save.J]),
                                save.AVVS.subarray([1, save.J]),
                                save.DAVV.as_slice(),
                                save.SUBTPS[save.MNSGNO],
                                save.PSIZES[save.MNSGNO],
                                save.RECORD.as_slice_mut(),
                                ctx,
                            )?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            spicelib::MOVED(
                                save.RECORD.as_slice(),
                                save.PSIZES[save.MNSGNO],
                                save.PACKTS.subarray_mut(save.PKTPTR),
                            );

                            save.PKTPTR = (save.PKTPTR + save.PSIZES[save.MNSGNO]);

                            save.J += m3__;
                        }
                    }

                    //
                    // Adjust the starting record number of the next mini-segment
                    // to account for padding.
                    //
                    save.BEGREC = (((save.BEGREC - 1) + save.NPKTS[save.MNSGNO]) - (2 * save.PAD));

                    //
                    // Update the count of the records in the segment.
                    //
                    save.N = (save.N + save.NPKTS[save.MNSGNO]);

                    save.MNSGNO += m3__;
                }
            }

            //
            // The stop time of the last interval is the epoch at
            // PAD positions before the final epoch.
            //
            save.XIVBDS[(save.NINTVL + 1)] = save.SCLKDP[(save.N - save.PAD)];

            //
            // Let the segment bounds coincide with the start and stop
            // times of the first and last mini-segments.
            //
            save.FIRST = save.XIVBDS[1];
            save.LAST = save.XIVBDS[(save.NINTVL + 1)];

            spicelib::CKW06(
                save.HANDLE,
                save.XINST,
                &save.XREF,
                save.XAVFLG,
                save.FIRST,
                save.LAST,
                &save.SEGID,
                save.NINTVL,
                save.NPKTS.as_slice(),
                save.SUBTPS.as_slice(),
                save.DEGRES.as_slice(),
                save.PACKTS.as_slice(),
                save.RATES.as_slice(),
                save.SCLKDP.as_slice(),
                save.XIVBDS.as_slice(),
                save.SELLST,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.SEGNO += m3__;
        }
    }

    //
    // Close the kernel.
    //
    spicelib::CKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    //  ###

    //
    // Perform tests on CK06B1.
    //
    fstr::assign(&mut save.CK, CK06B1);
    save.SELLST = false;

    fstr::assign(&mut save.TITLE, b"Interval/Packet selection logic test:  look up data at time tags within all three segments of CK #.");

    spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CK, &mut save.TITLE);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // Load the test CK.
    //
    spicelib::FURNSH(&save.CK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = 0;
    save.SEGPTR = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG;
        let m3__: i32 = 1;
        save.SEGNO = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.SEGPTR = (save.SEGPTR + save.N);

            //
            // --- Case: ------------------------------------------------------
            //

            fstr::assign(&mut save.TITLE, b"Interval/Packet selection logic test:  look up data at time tags within segment # of CK #.");

            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.SEGNO, &mut save.TITLE, ctx);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CK, &mut save.TITLE);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::TCASE(&save.TITLE, ctx)?;

            save.XINST = save.XINSTS[save.SEGNO];
            save.NINTVL = save.NIVALS[save.SEGNO];
            fstr::assign(&mut save.XREF, save.XREFS.get(save.SEGNO));

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NINTVL;
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    if (intrinsics::MOD(save.I, 3) == 0) {
                        save.SUBTPS[save.I] = C06TP3;
                        save.PSIZES[save.I] = C06PS3;
                        save.DEGRES[save.I] = 3;
                        save.RATES[save.I] = 1.1;
                        save.NPKTS[save.I] = 5;
                    } else if (intrinsics::MOD(save.I, 3) == 1) {
                        save.SUBTPS[save.I] = C06TP1;
                        save.PSIZES[save.I] = C06PS1;
                        save.DEGRES[save.I] = 7;
                        save.RATES[save.I] = 0.8;
                        save.NPKTS[save.I] = 9;
                    } else {
                        save.SUBTPS[save.I] = C06TP0;
                        save.PSIZES[save.I] = C06PS0;
                        save.DEGRES[save.I] = 7;
                        save.RATES[save.I] = 4.0;
                        save.NPKTS[save.I] = 11;
                    }

                    save.I += m3__;
                }
            }

            //
            // Create time tags and packets for the current segment.
            //
            save.BEGREC = 1;
            save.N = 1;
            save.PKTPTR = 1;

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NINTVL;
                let m3__: i32 = 1;
                save.MNSGNO = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    //
                    // Record the index of the first non-pad time tag of
                    // the current mini-segment.
                    //
                    save.DATIDX[save.MNSGNO] = (save.N + save.PAD);
                    //
                    // Generate time tags and packet data for the current
                    // mini-segment.
                    //
                    spicelib::MOVED(
                        save.XEPOCH.subarray((save.SEGPTR + save.N)),
                        save.NPKTS[save.MNSGNO],
                        save.SCLKDP.subarray_mut(save.N),
                    );

                    // Update the count of the records in the segment.
                    //
                    save.N = (save.N + save.NPKTS[save.MNSGNO]);

                    save.MNSGNO += m3__;
                }
            }

            //
            // Check data for each interval.
            //
            if (save.SEGNO == 3) {
                //
                // SKIP can be reset to 40 to increase execution speed.
                //
                save.SKIP = 1;
            } else {
                save.SKIP = 1;
            }

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NINTVL;
                let m3__: i32 = save.SKIP;
                save.MNSGNO = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.J = save.DATIDX[save.MNSGNO];

                    if (!save.SELLST && (save.MNSGNO > 1)) {
                        //
                        // Skip the first time tag of each mini-segment but the
                        // first, since the correct data will come from the
                        // previous interval.
                        //
                        save.J = (save.J + 1);
                    }

                    save.JMAX = (((save.DATIDX[save.MNSGNO] - 1) + save.NPKTS[save.MNSGNO])
                        - (2 * save.PAD));

                    while (save.J <= save.JMAX) {
                        save.T = save.SCLKDP[save.J];

                        spicelib::MOVED(
                            save.XAVVS.subarray([1, (save.SEGPTR + save.J)]),
                            3,
                            save.XAV.as_slice_mut(),
                        );

                        spicelib::MOVED(
                            save.XCMATS.subarray([1, (save.SEGPTR + save.J)]),
                            9,
                            save.XCMAT.as_slice_mut(),
                        );

                        //
                        // Look up pointing data.
                        //
                        save.CKTOL = 0.0;

                        spicelib::CKGPAV(
                            save.XINST,
                            save.T,
                            save.CKTOL,
                            &save.XREF,
                            save.CMAT.as_slice_mut(),
                            save.AV.as_slice_mut(),
                            &mut save.CLKOUT,
                            &mut save.FOUND,
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        if !*OK {
                            spicelib::SIGERR(b"NO POINTING", ctx)?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                            ctx.stop()?;
                        }

                        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

                        //
                        // Check output time.
                        //
                        testutil::CHCKSD(b"CLKOUT", save.CLKOUT, b"~", save.T, 0.0, OK, ctx)?;

                        if !*OK {
                            //
                            // Re-check with a more specific item name.
                            //
                            fstr::assign(&mut save.NAME, b"Mini-segment *; CLKOUT no. *");
                            spicelib::REPMI(
                                &save.NAME.to_vec(),
                                b"*",
                                save.MNSGNO,
                                &mut save.NAME,
                                ctx,
                            );
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.J, &mut save.NAME, ctx);
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            testutil::CHCKSD(&save.NAME, save.CLKOUT, b"~", save.T, 0.0, OK, ctx)?;
                        }

                        //
                        // Check C-matrix.
                        //
                        testutil::CHCKAD(
                            b"CMAT",
                            save.CMAT.as_slice(),
                            b"~~/",
                            save.XCMAT.as_slice(),
                            9,
                            VTIGHT,
                            OK,
                            ctx,
                        )?;

                        if !*OK {
                            fstr::assign(&mut save.NAME, b"Mini-segment *; CMAT no. *");
                            spicelib::REPMI(
                                &save.NAME.to_vec(),
                                b"*",
                                save.MNSGNO,
                                &mut save.NAME,
                                ctx,
                            );
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.J, &mut save.NAME, ctx);
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            testutil::CHCKAD(
                                &save.NAME,
                                save.CMAT.as_slice(),
                                b"~~/",
                                save.XCMAT.as_slice(),
                                9,
                                VTIGHT,
                                OK,
                                ctx,
                            )?;
                        }

                        //
                        // Check angular velocity.
                        //
                        // For derived AV, use a looser tolerance.
                        //
                        if (save.SUBTPS[save.MNSGNO] == C06TP1) {
                            save.TOL = MED;
                        } else if (save.SUBTPS[save.MNSGNO] == C06TP0) {
                            save.TOL = TIGHT;
                        } else if (save.SUBTPS[save.MNSGNO] == C06TP3) {
                            save.TOL = VTIGHT;
                        } else {
                            save.TOL = TIGHT;
                        }

                        testutil::CHCKAD(
                            b"AV",
                            save.AV.as_slice(),
                            b"~~/",
                            save.XAV.as_slice(),
                            3,
                            save.TOL,
                            OK,
                            ctx,
                        )?;

                        if !*OK {
                            fstr::assign(&mut save.NAME, b"Mini-segment *; AV no. *");
                            spicelib::REPMI(
                                &save.NAME.to_vec(),
                                b"*",
                                save.MNSGNO,
                                &mut save.NAME,
                                ctx,
                            );
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.J, &mut save.NAME, ctx);
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            testutil::CHCKAD(
                                b"AV",
                                save.AV.as_slice(),
                                b"~~/",
                                save.XAV.as_slice(),
                                3,
                                save.TOL,
                                OK,
                                ctx,
                            )?;
                        }

                        save.J = (save.J + 1);
                    }
                    //
                    // End of tests for current interval.
                    //
                    save.MNSGNO += m3__;
                }
            }
            //
            // End of tests for current segment.
            //
            save.SEGNO += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     Perform tests using data lookups at midpoints
    //     between consecutive time tags. This set of tests
    //     is for segments using the "select first" option.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG;
        let m3__: i32 = 1;
        save.SEGNO = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // --- Case: ------------------------------------------------------
            //

            fstr::assign(&mut save.TITLE, b"Interval/Packet selection logic test:  look up data at midpoints between time tags within segment # of CK #.");

            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.SEGNO, &mut save.TITLE, ctx);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CK, &mut save.TITLE);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::TCASE(&save.TITLE, ctx)?;

            save.SELLST = false;

            save.XINST = save.XINSTS[save.SEGNO];
            save.NINTVL = save.NIVALS[save.SEGNO];
            fstr::assign(&mut save.XREF, save.XREFS.get(save.SEGNO));

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NINTVL;
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    if (intrinsics::MOD(save.I, 3) == 0) {
                        save.SUBTPS[save.I] = C06TP3;
                        save.PSIZES[save.I] = C06PS3;
                        save.DEGRES[save.I] = 3;
                        save.RATES[save.I] = 1.1;
                        save.NPKTS[save.I] = 5;
                    } else if (intrinsics::MOD(save.I, 3) == 1) {
                        save.SUBTPS[save.I] = C06TP1;
                        save.PSIZES[save.I] = C06PS1;
                        save.DEGRES[save.I] = 7;
                        save.RATES[save.I] = 0.8;
                        save.NPKTS[save.I] = 9;
                    } else {
                        save.SUBTPS[save.I] = C06TP0;
                        save.PSIZES[save.I] = C06PS0;
                        save.DEGRES[save.I] = 7;
                        save.RATES[save.I] = 4.0;
                        save.NPKTS[save.I] = 11;
                    }

                    save.I += m3__;
                }
            }

            //
            // Create time tags for the current segment.
            //
            save.BEGREC = 1;
            save.N = 1;
            save.PKTPTR = 1;

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NINTVL;
                let m3__: i32 = 1;
                save.MNSGNO = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    //
                    // Record the index of the first non-pad time tag of
                    // the current mini-segment.
                    //
                    save.DATIDX[save.MNSGNO] = (save.N + save.PAD);

                    save.RATE = save.RATES[save.MNSGNO];
                    save.SUBTYP = save.SUBTPS[save.MNSGNO];

                    T_GENTAG(
                        save.BEGREC,
                        save.NPKTS[save.MNSGNO],
                        save.EPOCHS.as_slice_mut(),
                        ctx,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // The start time of the current interval is the epoch at
                    // PAD positions later than the start epoch.
                    //
                    save.XIVBDS[save.MNSGNO] = save.EPOCHS[(1 + save.PAD)];

                    spicelib::MOVED(
                        save.EPOCHS.as_slice(),
                        save.NPKTS[save.MNSGNO],
                        save.SCLKDP.subarray_mut(save.N),
                    );

                    //
                    // Adjust the starting record number of the next mini-segment
                    // to account for padding.
                    //
                    save.BEGREC = (((save.BEGREC - 1) + save.NPKTS[save.MNSGNO]) - (2 * save.PAD));

                    //
                    // Update the count of the records in the segment.
                    //
                    save.N = (save.N + save.NPKTS[save.MNSGNO]);

                    save.MNSGNO += m3__;
                }
            }

            //
            // Check data for each interval.
            //
            if (save.SEGNO == 3) {
                save.SKIP = 1;
            } else {
                save.SKIP = 1;
            }

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NINTVL;
                let m3__: i32 = save.SKIP;
                save.MNSGNO = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.RATE = save.RATES[save.MNSGNO];
                    save.J = save.DATIDX[save.MNSGNO];

                    save.JMAX = (((save.DATIDX[save.MNSGNO] - 1) + save.NPKTS[save.MNSGNO])
                        - (2 * save.PAD));

                    while (save.J < save.JMAX) {
                        save.T = ((save.SCLKDP[save.J] + save.SCLKDP[(save.J + 1)]) / 2 as f64);

                        // The following formula for ET looks strange, but it's
                        // correct.
                        //
                        save.ET = (save.T * save.RATE);
                        //
                        // Generate expected results for ET.
                        //
                        spicelib::SXFORM(
                            &save.XREF,
                            &save.CKFRAM,
                            save.ET,
                            save.XFORM.as_slice_mut(),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        spicelib::XF2RAV(
                            save.XFORM.as_slice(),
                            save.XCMAT.as_slice_mut(),
                            save.XAV.as_slice_mut(),
                        );
                        //
                        // Look up pointing data.
                        //
                        save.CKTOL = 0.0;
                        spicelib::CKGPAV(
                            save.XINST,
                            save.T,
                            save.CKTOL,
                            &save.XREF,
                            save.CMAT.as_slice_mut(),
                            save.AV.as_slice_mut(),
                            &mut save.CLKOUT,
                            &mut save.FOUND,
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        if !*OK {
                            spicelib::SIGERR(b"NO POINTING", ctx)?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                            ctx.stop()?;
                        }

                        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

                        //
                        // Check output time.
                        //
                        testutil::CHCKSD(b"CLKOUT", save.CLKOUT, b"~", save.T, 0.0, OK, ctx)?;

                        if !*OK {
                            //
                            // Re-check with a more specific item name.
                            //
                            fstr::assign(&mut save.NAME, b"Mini-segment *; CLKOUT no. *");
                            spicelib::REPMI(
                                &save.NAME.to_vec(),
                                b"*",
                                save.MNSGNO,
                                &mut save.NAME,
                                ctx,
                            );
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.J, &mut save.NAME, ctx);
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            testutil::CHCKSD(&save.NAME, save.CLKOUT, b"~", save.T, 0.0, OK, ctx)?;
                        }

                        //
                        // Check C-matrix.
                        //
                        testutil::CHCKAD(
                            b"CMAT",
                            save.CMAT.as_slice(),
                            b"~~/",
                            save.XCMAT.as_slice(),
                            9,
                            VTIGHT,
                            OK,
                            ctx,
                        )?;

                        if !*OK {
                            fstr::assign(&mut save.NAME, b"Mini-segment *; CMAT no. *");
                            spicelib::REPMI(
                                &save.NAME.to_vec(),
                                b"*",
                                save.MNSGNO,
                                &mut save.NAME,
                                ctx,
                            );
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.J, &mut save.NAME, ctx);
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            testutil::CHCKAD(
                                &save.NAME,
                                save.CMAT.as_slice(),
                                b"~~/",
                                save.XCMAT.as_slice(),
                                9,
                                VTIGHT,
                                OK,
                                ctx,
                            )?;
                        }

                        //
                        // Check angular velocity.
                        //
                        // For derived AV, use a looser tolerance.
                        //
                        if (save.SUBTPS[save.MNSGNO] == C06TP1) {
                            save.TOL = MED;
                        } else if (save.SUBTPS[save.MNSGNO] == C06TP0) {
                            save.TOL = TIGHT;
                        } else if (save.SUBTPS[save.MNSGNO] == C06TP3) {
                            save.TOL = VTIGHT;
                        } else {
                            save.TOL = TIGHT;
                        }

                        testutil::CHCKAD(
                            b"AV",
                            save.AV.as_slice(),
                            b"~~/",
                            save.XAV.as_slice(),
                            3,
                            save.TOL,
                            OK,
                            ctx,
                        )?;

                        if !*OK {
                            fstr::assign(&mut save.NAME, b"Mini-segment *; AV no. *");
                            spicelib::REPMI(
                                &save.NAME.to_vec(),
                                b"*",
                                save.MNSGNO,
                                &mut save.NAME,
                                ctx,
                            );
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.J, &mut save.NAME, ctx);
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            testutil::CHCKAD(
                                b"AV",
                                save.AV.as_slice(),
                                b"~~/",
                                save.XAV.as_slice(),
                                3,
                                save.TOL,
                                OK,
                                ctx,
                            )?;
                        }

                        save.J = (save.J + 1);
                        save.PKTPTR = (save.PKTPTR + save.PSIZES[save.MNSGNO]);
                    }
                    //
                    // End of tests for current interval.
                    //
                    save.MNSGNO += m3__;
                }
            }
            //
            // End of tests for current segment.
            //
            save.SEGNO += m3__;
        }
    }

    spicelib::UNLOAD(&save.CK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Perform tests on CK06B2.
    //
    fstr::assign(&mut save.CK, CK06B2);
    save.SELLST = true;

    fstr::assign(&mut save.TITLE, b"Interval/Packet selection logic test:  look up data at time tags within all three segments of CK #.");

    spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CK, &mut save.TITLE);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // Load the test CK.
    //
    spicelib::FURNSH(&save.CK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = 0;
    save.SEGPTR = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG;
        let m3__: i32 = 1;
        save.SEGNO = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.SEGPTR = (save.SEGPTR + save.N);
            //
            // --- Case: ------------------------------------------------------
            //

            fstr::assign(&mut save.TITLE, b"Interval/Packet selection logic test:  look up data at time tags within segment # of CK #.");

            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.SEGNO, &mut save.TITLE, ctx);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CK, &mut save.TITLE);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::TCASE(&save.TITLE, ctx)?;

            save.XINST = save.XINSTS[save.SEGNO];
            save.NINTVL = save.NIVALS[save.SEGNO];
            fstr::assign(&mut save.XREF, save.XREFS.get(save.SEGNO));

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NINTVL;
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    if (intrinsics::MOD(save.I, 3) == 0) {
                        save.SUBTPS[save.I] = C06TP3;
                        save.PSIZES[save.I] = C06PS3;
                        save.DEGRES[save.I] = 3;
                        save.RATES[save.I] = 1.1;
                        save.NPKTS[save.I] = 5;
                    } else if (intrinsics::MOD(save.I, 3) == 1) {
                        save.SUBTPS[save.I] = C06TP1;
                        save.PSIZES[save.I] = C06PS1;
                        save.DEGRES[save.I] = 7;
                        save.RATES[save.I] = 0.8;
                        save.NPKTS[save.I] = 9;
                    } else {
                        save.SUBTPS[save.I] = C06TP0;
                        save.PSIZES[save.I] = C06PS0;
                        save.DEGRES[save.I] = 7;
                        save.RATES[save.I] = 4.0;
                        save.NPKTS[save.I] = 11;
                    }

                    save.I += m3__;
                }
            }

            //
            // Create time tags and packets for the current segment.
            //
            save.BEGREC = 1;
            save.N = 1;
            save.PKTPTR = 1;

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NINTVL;
                let m3__: i32 = 1;
                save.MNSGNO = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    //
                    // Record the index of the first non-pad time tag of
                    // the current mini-segment.
                    //
                    save.DATIDX[save.MNSGNO] = (save.N + save.PAD);

                    spicelib::MOVED(
                        save.XEPOCH.subarray((save.SEGPTR + save.N)),
                        save.NPKTS[save.MNSGNO],
                        save.SCLKDP.subarray_mut(save.N),
                    );
                    //
                    // Update the count of the records in the segment.
                    //
                    save.N = (save.N + save.NPKTS[save.MNSGNO]);

                    save.MNSGNO += m3__;
                }
            }

            //
            // The stop time of the last interval is the epoch at
            // PAD positions before the final epoch.
            //
            save.XIVBDS[(save.NINTVL + 1)] = save.SCLKDP[(save.N - save.PAD)];

            //
            // Let the segment bounds coincide with the start and stop
            // times of the first and last mini-segments.
            //
            save.FIRST = save.XIVBDS[1];
            save.LAST = save.XIVBDS[(save.NINTVL + 1)];

            //
            // Check data for each interval.
            //
            if (save.SEGNO == 3) {
                save.SKIP = 1;
            } else {
                save.SKIP = 1;
            }

            save.N = 1;

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NINTVL;
                let m3__: i32 = save.SKIP;
                save.MNSGNO = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.J = save.DATIDX[save.MNSGNO];

                    save.JMAX = (((save.DATIDX[save.MNSGNO] - 1) + save.NPKTS[save.MNSGNO])
                        - (2 * save.PAD));

                    if (save.SELLST && (save.MNSGNO < save.NINTVL)) {
                        //
                        // Skip the last time tag of each mini-segment but the
                        // last, since the correct data will come from the
                        // next interval.
                        //
                        save.JMAX = (save.JMAX - 1);
                    }

                    //
                    // In the test below, ".LT." is used for the "select last"
                    // option.
                    //
                    while (save.J < save.JMAX) {
                        save.T = save.SCLKDP[save.J];

                        spicelib::MOVED(
                            save.XQUATS.subarray([0, (save.SEGPTR + save.J)]),
                            4,
                            save.XQ.as_slice_mut(),
                        );
                        spicelib::MOVED(
                            save.XAVVS.subarray([1, (save.SEGPTR + save.J)]),
                            3,
                            save.XAV.as_slice_mut(),
                        );

                        spicelib::Q2M(save.XQ.as_slice(), save.XCMAT.as_slice_mut());
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        //
                        // Look up pointing data.
                        //
                        spicelib::CKGPAV(
                            save.XINST,
                            save.T,
                            save.CKTOL,
                            &save.XREF,
                            save.CMAT.as_slice_mut(),
                            save.AV.as_slice_mut(),
                            &mut save.CLKOUT,
                            &mut save.FOUND,
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        if !*OK {
                            spicelib::SIGERR(b"NO POINTING", ctx)?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                            ctx.stop()?;
                        }

                        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

                        //
                        // Check output time.
                        //
                        testutil::CHCKSD(b"CLKOUT", save.CLKOUT, b"~", save.T, 0.0, OK, ctx)?;

                        if !*OK {
                            //
                            // Re-check with a more specific item name.
                            //
                            fstr::assign(&mut save.NAME, b"Mini-segment *; CLKOUT no. *");
                            spicelib::REPMI(
                                &save.NAME.to_vec(),
                                b"*",
                                save.MNSGNO,
                                &mut save.NAME,
                                ctx,
                            );
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.J, &mut save.NAME, ctx);
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            testutil::CHCKSD(&save.NAME, save.CLKOUT, b"~", save.T, 0.0, OK, ctx)?;
                        }

                        //
                        // Check C-matrix.
                        //
                        testutil::CHCKAD(
                            b"CMAT",
                            save.CMAT.as_slice(),
                            b"~~/",
                            save.XCMAT.as_slice(),
                            9,
                            VTIGHT,
                            OK,
                            ctx,
                        )?;

                        if !*OK {
                            fstr::assign(&mut save.NAME, b"Mini-segment *; CMAT no. *");
                            spicelib::REPMI(
                                &save.NAME.to_vec(),
                                b"*",
                                save.MNSGNO,
                                &mut save.NAME,
                                ctx,
                            );
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.J, &mut save.NAME, ctx);
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            testutil::CHCKAD(
                                &save.NAME,
                                save.CMAT.as_slice(),
                                b"~~/",
                                save.XCMAT.as_slice(),
                                9,
                                VTIGHT,
                                OK,
                                ctx,
                            )?;
                        }

                        //
                        // Check angular velocity.
                        //
                        // For derived AV, use a looser tolerance.
                        //
                        if (save.SUBTPS[save.MNSGNO] == C06TP1) {
                            save.TOL = MED;
                        } else if (save.SUBTPS[save.MNSGNO] == C06TP0) {
                            save.TOL = TIGHT;
                        } else if (save.SUBTPS[save.MNSGNO] == C06TP3) {
                            save.TOL = VTIGHT;
                        } else {
                            save.TOL = TIGHT;
                        }

                        testutil::CHCKAD(
                            b"AV",
                            save.AV.as_slice(),
                            b"~~/",
                            save.XAV.as_slice(),
                            3,
                            save.TOL,
                            OK,
                            ctx,
                        )?;

                        if !*OK {
                            fstr::assign(&mut save.NAME, b"Mini-segment *; AV no. *");
                            spicelib::REPMI(
                                &save.NAME.to_vec(),
                                b"*",
                                save.MNSGNO,
                                &mut save.NAME,
                                ctx,
                            );
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.J, &mut save.NAME, ctx);
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            testutil::CHCKAD(
                                b"AV",
                                save.AV.as_slice(),
                                b"~~/",
                                save.XAV.as_slice(),
                                3,
                                save.TOL,
                                OK,
                                ctx,
                            )?;
                        }

                        save.J = (save.J + 1);
                    }

                    save.N = (save.N + save.NPKTS[save.MNSGNO]);
                    //
                    // End of tests for current interval.
                    //
                    save.MNSGNO += m3__;
                }
            }
            //
            // End of tests for current segment.
            //
            save.SEGNO += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     Perform tests using data lookups at midpoints
    //     between consecutive time tags. This set of tests
    //     is for segments using the "select last" option.
    //
    //     CK is CK06B2.
    //

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG;
        let m3__: i32 = 1;
        save.SEGNO = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // --- Case: ------------------------------------------------------
            //

            fstr::assign(&mut save.TITLE, b"Interval/Packet selection logic test:  look up data at midpoints between time tags within segment # of CK #.");

            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.SEGNO, &mut save.TITLE, ctx);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CK, &mut save.TITLE);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::TCASE(&save.TITLE, ctx)?;

            save.SELLST = true;

            save.XINST = save.XINSTS[save.SEGNO];
            save.NINTVL = save.NIVALS[save.SEGNO];
            fstr::assign(&mut save.XREF, save.XREFS.get(save.SEGNO));
            fstr::assign(&mut save.CKFRAM, b"IAU_EARTH");

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NINTVL;
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    if (intrinsics::MOD(save.I, 3) == 0) {
                        save.SUBTPS[save.I] = C06TP3;
                        save.PSIZES[save.I] = C06PS3;
                        save.DEGRES[save.I] = 3;
                        save.RATES[save.I] = 1.1;
                        save.NPKTS[save.I] = 5;
                    } else if (intrinsics::MOD(save.I, 3) == 1) {
                        save.SUBTPS[save.I] = C06TP1;
                        save.PSIZES[save.I] = C06PS1;
                        save.DEGRES[save.I] = 7;
                        save.RATES[save.I] = 0.8;
                        save.NPKTS[save.I] = 9;
                    } else {
                        save.SUBTPS[save.I] = C06TP0;
                        save.PSIZES[save.I] = C06PS0;
                        save.DEGRES[save.I] = 7;
                        save.RATES[save.I] = 4.0;
                        save.NPKTS[save.I] = 11;
                    }

                    save.I += m3__;
                }
            }

            //
            // Create time tags for the current segment.
            //
            save.BEGREC = 1;
            save.N = 1;
            save.PKTPTR = 1;

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NINTVL;
                let m3__: i32 = 1;
                save.MNSGNO = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    //
                    // Record the index of the first non-pad time tag of
                    // the current mini-segment.
                    //
                    save.DATIDX[save.MNSGNO] = (save.N + save.PAD);

                    save.RATE = save.RATES[save.MNSGNO];
                    save.SUBTYP = save.SUBTPS[save.MNSGNO];

                    T_GENTAG(
                        save.BEGREC,
                        save.NPKTS[save.MNSGNO],
                        save.EPOCHS.as_slice_mut(),
                        ctx,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // The start time of the current interval is the epoch at
                    // PAD positions later than the start epoch.
                    //
                    save.XIVBDS[save.MNSGNO] = save.EPOCHS[(1 + save.PAD)];

                    spicelib::MOVED(
                        save.EPOCHS.as_slice(),
                        save.NPKTS[save.MNSGNO],
                        save.SCLKDP.subarray_mut(save.N),
                    );

                    //
                    // Adjust the starting record number of the next mini-segment
                    // to account for padding.
                    //
                    save.BEGREC = (((save.BEGREC - 1) + save.NPKTS[save.MNSGNO]) - (2 * save.PAD));

                    //
                    // Update the count of the records in the segment.
                    //
                    save.N = (save.N + save.NPKTS[save.MNSGNO]);

                    save.MNSGNO += m3__;
                }
            }

            //
            // The stop time of the last interval is the epoch at
            // PAD positions before the final epoch.
            //
            save.XIVBDS[(save.NINTVL + 1)] = save.SCLKDP[(save.N - save.PAD)];

            //
            // Let the segment bounds coincide with the start and stop
            // times of the first and last mini-segments.
            //
            save.FIRST = save.XIVBDS[1];
            save.LAST = save.XIVBDS[(save.NINTVL + 1)];

            //
            // Check data for each interval.
            //
            if (save.SEGNO == 3) {
                save.SKIP = 1;
            } else {
                save.SKIP = 1;
            }

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NINTVL;
                let m3__: i32 = save.SKIP;
                save.MNSGNO = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.RATE = save.RATES[save.MNSGNO];
                    save.J = save.DATIDX[save.MNSGNO];

                    save.JMAX = (((save.DATIDX[save.MNSGNO] - 1) + save.NPKTS[save.MNSGNO])
                        - (2 * save.PAD));

                    while (save.J < save.JMAX) {
                        save.T = ((save.SCLKDP[save.J] + save.SCLKDP[(save.J + 1)]) / 2 as f64);

                        // The following formula for ET looks strange, but it's
                        // correct.
                        //
                        save.ET = (save.T * save.RATE);
                        //
                        // Generate expected results for ET.
                        //
                        spicelib::SXFORM(
                            &save.XREF,
                            &save.CKFRAM,
                            save.ET,
                            save.XFORM.as_slice_mut(),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        spicelib::XF2RAV(
                            save.XFORM.as_slice(),
                            save.XCMAT.as_slice_mut(),
                            save.XAV.as_slice_mut(),
                        );
                        //
                        // Look up pointing data.
                        //
                        spicelib::CKGPAV(
                            save.XINST,
                            save.T,
                            save.CKTOL,
                            &save.XREF,
                            save.CMAT.as_slice_mut(),
                            save.AV.as_slice_mut(),
                            &mut save.CLKOUT,
                            &mut save.FOUND,
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        if !*OK {
                            spicelib::SIGERR(b"NO POINTING", ctx)?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                            ctx.stop()?;
                        }

                        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

                        //
                        // Check output time.
                        //
                        testutil::CHCKSD(b"CLKOUT", save.CLKOUT, b"~", save.T, 0.0, OK, ctx)?;

                        if !*OK {
                            //
                            // Re-check with a more specific item name.
                            //
                            fstr::assign(&mut save.NAME, b"Mini-segment *; CLKOUT no. *");
                            spicelib::REPMI(
                                &save.NAME.to_vec(),
                                b"*",
                                save.MNSGNO,
                                &mut save.NAME,
                                ctx,
                            );
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.J, &mut save.NAME, ctx);
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            testutil::CHCKSD(&save.NAME, save.CLKOUT, b"~", save.T, 0.0, OK, ctx)?;
                        }

                        //
                        // Check C-matrix.
                        //
                        testutil::CHCKAD(
                            b"CMAT",
                            save.CMAT.as_slice(),
                            b"~~/",
                            save.XCMAT.as_slice(),
                            9,
                            VTIGHT,
                            OK,
                            ctx,
                        )?;

                        if !*OK {
                            fstr::assign(&mut save.NAME, b"Mini-segment *; CMAT no. *");
                            spicelib::REPMI(
                                &save.NAME.to_vec(),
                                b"*",
                                save.MNSGNO,
                                &mut save.NAME,
                                ctx,
                            );
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.J, &mut save.NAME, ctx);
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            testutil::CHCKAD(
                                &save.NAME,
                                save.CMAT.as_slice(),
                                b"~~/",
                                save.XCMAT.as_slice(),
                                9,
                                VTIGHT,
                                OK,
                                ctx,
                            )?;
                        }

                        //
                        // Check angular velocity.
                        //
                        // For derived AV, use a looser tolerance.
                        //
                        if (save.SUBTPS[save.MNSGNO] == C06TP1) {
                            save.TOL = MED;
                        } else if (save.SUBTPS[save.MNSGNO] == C06TP0) {
                            save.TOL = TIGHT;
                        } else if (save.SUBTPS[save.MNSGNO] == C06TP3) {
                            save.TOL = VTIGHT;
                        } else {
                            save.TOL = TIGHT;
                        }

                        testutil::CHCKAD(
                            b"AV",
                            save.AV.as_slice(),
                            b"~~/",
                            save.XAV.as_slice(),
                            3,
                            save.TOL,
                            OK,
                            ctx,
                        )?;

                        if !*OK {
                            fstr::assign(&mut save.NAME, b"Mini-segment *; AV no. *");
                            spicelib::REPMI(
                                &save.NAME.to_vec(),
                                b"*",
                                save.MNSGNO,
                                &mut save.NAME,
                                ctx,
                            );
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                            spicelib::REPMI(&save.NAME.to_vec(), b"*", save.J, &mut save.NAME, ctx);
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            testutil::CHCKAD(
                                b"AV",
                                save.AV.as_slice(),
                                b"~~/",
                                save.XAV.as_slice(),
                                3,
                                save.TOL,
                                OK,
                                ctx,
                            )?;
                        }

                        save.J = (save.J + 1);
                        save.PKTPTR = (save.PKTPTR + save.PSIZES[save.MNSGNO]);
                    }
                    //
                    // End of tests for current interval.
                    //
                    save.MNSGNO += m3__;
                }
            }
            //
            // End of tests for current segment.
            //
            save.SEGNO += m3__;
        }
    }

    spicelib::UNLOAD(&save.CK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up:  delete CK files.", ctx)?;

    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if spicelib::EXISTS(CK06E, ctx)? {
        spicelib::DELFIL(CK06E, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    if spicelib::EXISTS(CK06MU, ctx)? {
        spicelib::DELFIL(CK06MU, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    if spicelib::EXISTS(CK06T0, ctx)? {
        spicelib::DELFIL(CK06T0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    if spicelib::EXISTS(CK06T1, ctx)? {
        spicelib::DELFIL(CK06T1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    if spicelib::EXISTS(CK06B0, ctx)? {
        spicelib::DELFIL(CK06B0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    if spicelib::EXISTS(CK06B1, ctx)? {
        spicelib::DELFIL(CK06B1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    if spicelib::EXISTS(CK06B2, ctx)? {
        spicelib::DELFIL(CK06B2, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
