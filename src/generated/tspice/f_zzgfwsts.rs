//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CNVTOL: f64 = 0.000001;
const NWMAX: i32 = 15;
const NWDIST: i32 = 5;
const NWSEP: i32 = 5;
const NWRR: i32 = 5;
const NWUDS: i32 = 5;
const NWPA: i32 = 5;
const NWILUM: i32 = 5;
const ADDWIN: f64 = 0.5;
const FRMNLN: i32 = 32;
const FOVTLN: i32 = 40;
const FTCIRC: &[u8] = b"CIRCLE";
const FTELLI: &[u8] = b"ELLIPSE";
const FTPOLY: &[u8] = b"POLYGON";
const FTRECT: &[u8] = b"RECTANGLE";
const ANNULR: &[u8] = b"ANNULAR";
const ANY: &[u8] = b"ANY";
const PARTL: &[u8] = b"PARTIAL";
const FULL: &[u8] = b"FULL";
const DSSHAP: &[u8] = b"DSK";
const EDSHAP: &[u8] = b"ELLIPSOID";
const PTSHAP: &[u8] = b"POINT";
const RYSHAP: &[u8] = b"RAY";
const SPSHAP: &[u8] = b"SPHERE";
const NOCTYP: i32 = 4;
const OCLLN: i32 = 7;
const SHPLEN: i32 = 9;
const MAXVRT: i32 = 10000;
const CIRFOV: &[u8] = b"CIRCLE";
const ELLFOV: &[u8] = b"ELLIPSE";
const POLFOV: &[u8] = b"POLYGON";
const RECFOV: &[u8] = b"RECTANGLE";
const INCLEN: i32 = 2;
const LBCELL: i32 = -5;
const LNSIZE: i32 = 80;
const MAXWIN: i32 = 1000;
const NOPS: i32 = 4;

struct SaveVars {
    OPS: ActualCharArray,
    QNAME: Vec<u8>,
    DELTA1: f64,
    DELTA2: f64,
    FINISH: f64,
    START: f64,
    STEP1: f64,
    STEP2: f64,
    WNDW1: ActualArray<f64>,
    WNDW2: ActualArray<f64>,
    WNDW3: ActualArray<f64>,
    XWIN: ActualArray<f64>,
    CARD: i32,
    I: i32,
    N: i32,
    XCARD: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut OPS = ActualCharArray::new(INCLEN, 1..=NOPS);
        let mut QNAME = vec![b' '; LNSIZE as usize];
        let mut DELTA1: f64 = 0.0;
        let mut DELTA2: f64 = 0.0;
        let mut FINISH: f64 = 0.0;
        let mut START: f64 = 0.0;
        let mut STEP1: f64 = 0.0;
        let mut STEP2: f64 = 0.0;
        let mut WNDW1 = ActualArray::<f64>::new(LBCELL..=MAXWIN);
        let mut WNDW2 = ActualArray::<f64>::new(LBCELL..=MAXWIN);
        let mut WNDW3 = ActualArray::<f64>::new(LBCELL..=MAXWIN);
        let mut XWIN = ActualArray::<f64>::new(LBCELL..=MAXWIN);
        let mut CARD: i32 = 0;
        let mut I: i32 = 0;
        let mut N: i32 = 0;
        let mut XCARD: i32 = 0;

        {
            use f2rust_std::data::Val;

            let mut clist =
                [Val::C(b"()"), Val::C(b"[]"), Val::C(b"[)"), Val::C(b"(]")].into_iter();
            OPS.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            OPS,
            QNAME,
            DELTA1,
            DELTA2,
            FINISH,
            START,
            STEP1,
            STEP2,
            WNDW1,
            WNDW2,
            WNDW3,
            XWIN,
            CARD,
            I,
            N,
            XCARD,
        }
    }
}

//$Procedure      F_ZZGFWSTS ( Test ZZGFWSTS )
pub fn F_ZZGFWSTS(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local Variables
    //

    //
    // Saved everything
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZGFWSTS", ctx)?;

    //*********************************************************************
    //*
    //*    Error cases
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Invalid inclusion operator", ctx)?;

    spicelib::SSIZED(MAXWIN, save.WNDW1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.WNDW2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.WNDW3.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFWSTS(
        save.WNDW1.as_slice(),
        save.WNDW2.as_slice(),
        b") )",
        save.WNDW3.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(UNKNOWNINCLUSION)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Window overflow", ctx)?;

    spicelib::SSIZED(MAXWIN, save.WNDW1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.WNDW2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(2, save.WNDW3.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(1.0, 2.0, save.WNDW1.as_slice_mut(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(3.0, 4.0, save.WNDW1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(0.0, 5.0, save.WNDW2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFWSTS(
        save.WNDW1.as_slice(),
        save.WNDW2.as_slice(),
        b"[]",
        save.WNDW3.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(OUTOFROOM)", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Normal cases
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"First window precedes second; one interval each.", ctx)?;

    spicelib::SSIZED(MAXWIN, save.WNDW1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.WNDW2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.WNDW3.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(1.0, 2.0, save.WNDW1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(3.0, 4.0, save.WNDW2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NOPS;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::SCARDD(0, save.WNDW3.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::ZZGFWSTS(
                save.WNDW1.as_slice(),
                save.WNDW2.as_slice(),
                &save.OPS[save.I],
                save.WNDW3.as_slice_mut(),
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.CARD = spicelib::WNCARD(save.WNDW3.as_slice(), ctx)?;

            fstr::assign(&mut save.QNAME, b"CARD (I=#)");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

            testutil::CHCKSI(&save.QNAME, save.CARD, b"=", 0, 0, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Second window precedes first; one interval each.", ctx)?;

    spicelib::SSIZED(MAXWIN, save.WNDW1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.WNDW2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.WNDW3.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(1.0, 2.0, save.WNDW2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(3.0, 4.0, save.WNDW1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NOPS;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::SCARDD(0, save.WNDW3.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::ZZGFWSTS(
                save.WNDW1.as_slice(),
                save.WNDW2.as_slice(),
                &save.OPS[save.I],
                save.WNDW3.as_slice_mut(),
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.CARD = spicelib::WNCARD(save.WNDW3.as_slice(), ctx)?;

            fstr::assign(&mut save.QNAME, b"CARD (I=#)");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

            testutil::CHCKSI(&save.QNAME, save.CARD, b"=", 0, 0, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"First window properly contained in second; one interval each.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.WNDW1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.WNDW2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.WNDW3.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(1.0, 2.0, save.WNDW1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(-2.0, 3.0, save.WNDW2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.XWIN.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::COPYD(save.WNDW1.as_slice(), save.XWIN.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NOPS;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::ZZGFWSTS(
                save.WNDW1.as_slice(),
                save.WNDW2.as_slice(),
                &save.OPS[save.I],
                save.WNDW3.as_slice_mut(),
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.CARD = spicelib::WNCARD(save.WNDW3.as_slice(), ctx)?;

            fstr::assign(&mut save.QNAME, b"CARD (I=#)");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

            testutil::CHCKSI(&save.QNAME, save.CARD, b"=", 1, 0, OK, ctx)?;

            fstr::assign(&mut save.QNAME, b"WNDW3 (I=#)");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

            testutil::CHCKAD(
                &save.QNAME,
                save.WNDW3.as_slice(),
                b"=",
                save.XWIN.as_slice(),
                3,
                0.0,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Second window properly contained in first; one interval each.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.WNDW1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.WNDW2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.WNDW3.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(1.0, 2.0, save.WNDW2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(-2.0, 3.0, save.WNDW1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.XWIN.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::COPYD(save.WNDW2.as_slice(), save.XWIN.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NOPS;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::ZZGFWSTS(
                save.WNDW1.as_slice(),
                save.WNDW2.as_slice(),
                &save.OPS[save.I],
                save.WNDW3.as_slice_mut(),
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.CARD = spicelib::WNCARD(save.WNDW3.as_slice(), ctx)?;

            fstr::assign(&mut save.QNAME, b"CARD (I=#)");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

            //
            // We expect the sift result to be empty.
            //
            testutil::CHCKSI(&save.QNAME, save.CARD, b"=", 0, 0, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"First window\'s endpoints match those of second; one interval each.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.WNDW1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.WNDW2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.WNDW3.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(1.0, 2.0, save.WNDW1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::COPYD(save.WNDW1.as_slice(), save.WNDW2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.XWIN.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::COPYD(save.WNDW1.as_slice(), save.XWIN.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NOPS;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::ZZGFWSTS(
                save.WNDW1.as_slice(),
                save.WNDW2.as_slice(),
                &save.OPS[save.I],
                save.WNDW3.as_slice_mut(),
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.CARD = spicelib::WNCARD(save.WNDW3.as_slice(), ctx)?;

            //
            // We have inclusion only if the second window's interval
            // is treated as being closed.
            //
            if fstr::eq(save.OPS.get(save.I), b"[]") {
                save.XCARD = 1;
            } else {
                save.XCARD = 0;
            }

            fstr::assign(&mut save.QNAME, b"CARD (I=#)");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

            testutil::CHCKSI(&save.QNAME, save.CARD, b"=", save.XCARD, 0, OK, ctx)?;

            if (save.XCARD > 0) {
                fstr::assign(&mut save.QNAME, b"WNDW3 (I=#)");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                testutil::CHCKAD(
                    &save.QNAME,
                    save.WNDW3.as_slice(),
                    b"=",
                    save.XWIN.as_slice(),
                    3,
                    0.0,
                    OK,
                    ctx,
                )?;
            }

            save.I += m3__;
        }
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"First window\'s left endpoint matches that of second; one interval each.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.WNDW1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.WNDW2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.WNDW3.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(1.0, 2.0, save.WNDW1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(1.0, 3.0, save.WNDW2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.XWIN.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::COPYD(save.WNDW1.as_slice(), save.XWIN.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NOPS;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::ZZGFWSTS(
                save.WNDW1.as_slice(),
                save.WNDW2.as_slice(),
                &save.OPS[save.I],
                save.WNDW3.as_slice_mut(),
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.CARD = spicelib::WNCARD(save.WNDW3.as_slice(), ctx)?;

            //
            // We have inclusion only if the second window's interval
            // is treated as being closed on the left.
            //
            if fstr::eq(save.OPS.get(save.I), b"[]") {
                save.XCARD = 1;
            } else if fstr::eq(save.OPS.get(save.I), b"[)") {
                save.XCARD = 1;
            } else {
                save.XCARD = 0;
            }

            fstr::assign(&mut save.QNAME, b"CARD (I=#)");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

            testutil::CHCKSI(&save.QNAME, save.CARD, b"=", save.XCARD, 0, OK, ctx)?;

            if (save.XCARD > 0) {
                fstr::assign(&mut save.QNAME, b"WNDW3 (I=#)");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                testutil::CHCKAD(
                    &save.QNAME,
                    save.WNDW3.as_slice(),
                    b"=",
                    save.XWIN.as_slice(),
                    3,
                    0.0,
                    OK,
                    ctx,
                )?;
            }

            save.I += m3__;
        }
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"First window\'s right endpoint matches that of second; one interval each.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.WNDW1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.WNDW2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.WNDW3.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(2.0, 3.0, save.WNDW1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(1.0, 3.0, save.WNDW2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.XWIN.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::COPYD(save.WNDW1.as_slice(), save.XWIN.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NOPS;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::ZZGFWSTS(
                save.WNDW1.as_slice(),
                save.WNDW2.as_slice(),
                &save.OPS[save.I],
                save.WNDW3.as_slice_mut(),
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.CARD = spicelib::WNCARD(save.WNDW3.as_slice(), ctx)?;

            //
            // We have inclusion only if the second window's interval
            // is treated as being closed on the right.
            //
            if fstr::eq(save.OPS.get(save.I), b"[]") {
                save.XCARD = 1;
            } else if fstr::eq(save.OPS.get(save.I), b"(]") {
                save.XCARD = 1;
            } else {
                save.XCARD = 0;
            }

            fstr::assign(&mut save.QNAME, b"CARD (I=#)");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

            testutil::CHCKSI(&save.QNAME, save.CARD, b"=", save.XCARD, 0, OK, ctx)?;

            if (save.XCARD > 0) {
                fstr::assign(&mut save.QNAME, b"WNDW3 (I=#)");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                testutil::CHCKAD(
                    &save.QNAME,
                    save.WNDW3.as_slice(),
                    b"=",
                    save.XWIN.as_slice(),
                    3,
                    0.0,
                    OK,
                    ctx,
                )?;
            }

            save.I += m3__;
        }
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Multiple WNDW1 intervals in each WNDW2 interval.", ctx)?;

    spicelib::SSIZED(MAXWIN, save.WNDW1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.WNDW2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.WNDW3.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.STEP1 = 1.0;
    save.DELTA1 = 0.2;

    save.STEP2 = 10.0;
    save.DELTA2 = 4.0;

    //
    // Create second window.
    //
    save.N = 20;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.START = (((save.I as f64) * save.STEP2) - save.DELTA2);
            save.FINISH = (((save.I as f64) * save.STEP2) + save.DELTA2);

            spicelib::WNINSD(save.START, save.FINISH, save.WNDW2.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Create first window. There are 7 small intervals
    // in each interval of the large window.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            for J in -3..=3 {
                save.START =
                    ((((save.I as f64) * save.STEP2) + ((J as f64) * save.STEP1)) - save.DELTA1);
                save.FINISH =
                    ((((save.I as f64) * save.STEP2) + ((J as f64) * save.STEP1)) + save.DELTA1);

                spicelib::WNINSD(save.START, save.FINISH, save.WNDW1.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
            }

            save.I += m3__;
        }
    }

    spicelib::SSIZED(MAXWIN, save.XWIN.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::COPYD(save.WNDW1.as_slice(), save.XWIN.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFWSTS(
        save.WNDW1.as_slice(),
        save.WNDW2.as_slice(),
        b"[]",
        save.WNDW3.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CARD = spicelib::WNCARD(save.WNDW3.as_slice(), ctx)?;

    fstr::assign(&mut save.QNAME, b"CARD");
    spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

    testutil::CHCKSI(&save.QNAME, save.CARD, b"=", (save.N * 7), 0, OK, ctx)?;

    fstr::assign(&mut save.QNAME, b"WNDW3");
    spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

    testutil::CHCKAD(
        &save.QNAME,
        save.WNDW3.as_slice(),
        b"=",
        save.XWIN.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"WNDW2 intervals span multiple WNDW1 intervals.", ctx)?;

    //
    // Use the input windows from the previous test.
    //

    spicelib::ZZGFWSTS(
        save.WNDW2.as_slice(),
        save.WNDW1.as_slice(),
        b"[]",
        save.WNDW3.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CARD = spicelib::WNCARD(save.WNDW3.as_slice(), ctx)?;

    fstr::assign(&mut save.QNAME, b"CARD");
    spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

    testutil::CHCKSI(&save.QNAME, save.CARD, b"=", 0, 0, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Multiple WNDW1 intervals in each WNDW2 interval; some intervals not contained.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.WNDW1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.WNDW2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.WNDW3.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.STEP1 = 1.0;
    save.DELTA1 = 0.2;

    save.STEP2 = 10.0;
    save.DELTA2 = 4.0;

    //
    // Create second window.
    //
    save.N = 20;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.START = (((save.I as f64) * save.STEP2) - save.DELTA2);
            save.FINISH = (((save.I as f64) * save.STEP2) + save.DELTA2);

            spicelib::WNINSD(save.START, save.FINISH, save.WNDW2.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Create first window. There is a sequence of 9 small intervals
    // centered at the center of each interval of the large window;
    // the intervals at the ends of each of these sequences extend
    // beyond the corresponding interval of the second window.
    //
    spicelib::SSIZED(MAXWIN, save.XWIN.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            for J in -4..=4 {
                save.START =
                    ((((save.I as f64) * save.STEP2) + ((J as f64) * save.STEP1)) - save.DELTA1);
                save.FINISH =
                    ((((save.I as f64) * save.STEP2) + ((J as f64) * save.STEP1)) + save.DELTA1);

                spicelib::WNINSD(save.START, save.FINISH, save.WNDW1.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                if ((J > -4) || (J < 4)) {
                    spicelib::WNINSD(save.START, save.FINISH, save.XWIN.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }
            }

            save.I += m3__;
        }
    }

    spicelib::ZZGFWSTS(
        save.WNDW1.as_slice(),
        save.WNDW2.as_slice(),
        b"[]",
        save.WNDW3.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CARD = spicelib::WNCARD(save.WNDW3.as_slice(), ctx)?;

    fstr::assign(&mut save.QNAME, b"CARD");
    spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

    testutil::CHCKSI(&save.QNAME, save.CARD, b"=", (save.N * 7), 0, OK, ctx)?;

    fstr::assign(&mut save.QNAME, b"WNDW3");
    spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

    testutil::CHCKAD(
        &save.QNAME,
        save.WNDW3.as_slice(),
        b"=",
        save.XWIN.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Multiple WNDW1 intervals between each WNDW2 interval.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.WNDW1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.WNDW2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.WNDW3.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.STEP1 = 1.0;
    save.DELTA1 = 0.2;

    save.STEP2 = 10.0;
    save.DELTA2 = 1.0;

    //
    // Create second window.
    //
    save.N = 20;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.START = (((save.I as f64) * save.STEP2) - save.DELTA2);
            save.FINISH = (((save.I as f64) * save.STEP2) + save.DELTA2);

            spicelib::WNINSD(save.START, save.FINISH, save.WNDW2.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Create first window. There are 7 small intervals
    // between each interval of the large window.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = (save.N - 1);
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            for J in -3..=3 {
                save.START = (((((save.I as f64) + 0.5) * save.STEP2) + ((J as f64) * save.STEP1))
                    - save.DELTA1);
                save.FINISH = (((((save.I as f64) + 0.5) * save.STEP2)
                    + ((J as f64) * save.STEP1))
                    + save.DELTA1);

                spicelib::WNINSD(save.START, save.FINISH, save.WNDW1.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
            }

            save.I += m3__;
        }
    }

    spicelib::ZZGFWSTS(
        save.WNDW1.as_slice(),
        save.WNDW2.as_slice(),
        b"[]",
        save.WNDW3.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CARD = spicelib::WNCARD(save.WNDW3.as_slice(), ctx)?;

    fstr::assign(&mut save.QNAME, b"CARD");
    spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

    testutil::CHCKSI(&save.QNAME, save.CARD, b"=", 0, 0, OK, ctx)?;

    fstr::assign(&mut save.QNAME, b"WNDW3");
    spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

    testutil::CHCKAD(
        &save.QNAME,
        save.WNDW3.as_slice(),
        b"=",
        save.XWIN.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Valid inclusion operators with blanks", ctx)?;

    spicelib::WNINSD(1.0, 2.0, save.WNDW1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(-2.0, 3.0, save.WNDW1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFWSTS(
        save.WNDW2.as_slice(),
        save.WNDW1.as_slice(),
        b"  (   ) ",
        save.WNDW3.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFWSTS(
        save.WNDW2.as_slice(),
        save.WNDW1.as_slice(),
        b"  [   ] ",
        save.WNDW3.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFWSTS(
        save.WNDW2.as_slice(),
        save.WNDW1.as_slice(),
        b"  [   ) ",
        save.WNDW3.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFWSTS(
        save.WNDW2.as_slice(),
        save.WNDW1.as_slice(),
        b"  (   ] ",
        save.WNDW3.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    Ok(())
}
