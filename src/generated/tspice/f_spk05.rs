//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const UTC1: &[u8] = b"1999 JUL 1";
const REF1: &[u8] = b"J2000";
const SPK1: &[u8] = b"test.bsp";
const SPK05E: &[u8] = b"test05err.bsp";
const SPK05: &[u8] = b"test05.bsp";
const SPK05B: &[u8] = b"test05big.bsp";
const SPK05S: &[u8] = b"test05sub.bsp";
const BIGSTP: f64 = 100.0;
const GMSUN: f64 = 132712439812.232;
const TIGHT: f64 = 0.00000000000001;
const MEDIUM: f64 = 0.00000000001;
const LOOSE: f64 = 0.001;
const SLOPPY: f64 = 0.1;
const BIGN: i32 = 10101;
const BIGID: i32 = 399;
const BIGCTR: i32 = 10;
const BIGDEG: i32 = 3;
const CHBDEG: i32 = 2;
const DSCSIZ: i32 = 5;
const LNSIZE: i32 = 80;
const MSGLEN: i32 = 240;
const ND: i32 = 2;
const NDSCRT: i32 = 9;
const NCHBRC: i32 = 4;
const NI: i32 = 6;
const SIDLEN: i32 = 60;
const FRNMLN: i32 = 32;

struct SaveVars {
    BADEPC: StackArray<f64, 9>,
    DSCEPC: StackArray<f64, 9>,
    DSCSTS: StackArray2D<f64, 54>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BADEPC = StackArray::<f64, 9>::new(1..=NDSCRT);
        let mut DSCEPC = StackArray::<f64, 9>::new(1..=NDSCRT);
        let mut DSCSTS = StackArray2D::<f64, 54>::new(1..=6, 1..=NDSCRT);

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
                Val::D(-800.0),
                Val::D(900.0),
            ]
            .into_iter();
            BADEPC
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
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
            BADEPC,
            DSCEPC,
            DSCSTS,
        }
    }
}

//$Procedure F_SPK05 ( SPK data type 05 tests )
pub fn F_SPK05(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut SEGID = [b' '; SIDLEN as usize];
    let mut SEGID2 = [b' '; SIDLEN as usize];
    let mut TITLE = [b' '; MSGLEN as usize];
    let mut XREF = [b' '; FRNMLN as usize];
    let mut BEPLST = ActualArray::<f64>::new(1..=(BIGN + 1));
    let mut BSTLST = ActualArray2D::<f64>::new(1..=6, 1..=(BIGN + 1));
    let mut DC = StackArray::<f64, 2>::new(1..=ND);
    let mut DESCR = StackArray::<f64, 5>::new(1..=DSCSIZ);
    let mut DT: f64 = 0.0;
    let mut LT: f64 = 0.0;
    let mut PCOMP = StackArray::<f64, 3>::new(1..=3);
    let mut PROPST = StackArray2D::<f64, 12>::new(1..=6, 1..=2);
    let mut STATE = StackArray::<f64, 6>::new(1..=6);
    let mut STEP: f64 = 0.0;
    let mut VCOMP = StackArray::<f64, 3>::new(1..=3);
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
    // This GM value is from the test utility TSTST.
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
    testutil::TOPEN(b"F_SPK05", ctx)?;

    //
    //     Test SPKW05:  start out with error handling.
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKW05 error case: bad frame name.", ctx)?;

    XBODY = 3;
    XCENTR = 10;
    fstr::assign(&mut XREF, b"J2000");

    //
    // Create a segment identifier.
    //
    fstr::assign(&mut SEGID, b"SPK type 05 test segment");

    //
    // Open a new SPK file.
    //
    if spicelib::EXISTS(SPK05E, ctx)? {
        spicelib::DELFIL(SPK05E, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::SPKOPN(
        SPK05E,
        b"Type 05 SPK internal file name",
        4,
        &mut HANDLE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    STEP = (save.DSCEPC[2] - save.DSCEPC[1]);

    spicelib::SPKW05(
        HANDLE,
        XBODY,
        XCENTR,
        b"SPUD",
        save.DSCEPC[1],
        save.DSCEPC[NDSCRT],
        &SEGID,
        GMSUN,
        NDSCRT,
        save.DSCSTS.as_slice(),
        save.DSCEPC.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDREFFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKW05 error case: SEGID too long.", ctx)?;

    spicelib::SPKW05(
        HANDLE,
        XBODY,
        XCENTR,
        &XREF,
        save.DSCEPC[1],
        save.DSCEPC[NDSCRT],
        b"X                                               X",
        GMSUN,
        NDSCRT,
        save.DSCSTS.as_slice(),
        save.DSCEPC.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SEGIDTOOLONG)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKW05 error case: unprintable SEGID characters.", ctx)?;

    spicelib::SPKW05(
        HANDLE,
        XBODY,
        XCENTR,
        &XREF,
        save.DSCEPC[1],
        save.DSCEPC[NDSCRT],
        &intrinsics::CHAR(7),
        GMSUN,
        NDSCRT,
        save.DSCSTS.as_slice(),
        save.DSCEPC.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NONPRINTABLECHARS)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKW05 error case: zero GM", ctx)?;

    spicelib::SPKW05(
        HANDLE,
        XBODY,
        XCENTR,
        &XREF,
        save.DSCEPC[1],
        save.DSCEPC[NDSCRT],
        &SEGID,
        0.0,
        NDSCRT,
        save.DSCSTS.as_slice(),
        save.DSCEPC.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NONPOSITIVEMASS)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKW05 error case: negative number of states", ctx)?;

    spicelib::SPKW05(
        HANDLE,
        XBODY,
        XCENTR,
        &XREF,
        save.DSCEPC[1],
        save.DSCEPC[NDSCRT],
        &SEGID,
        GMSUN,
        -1,
        save.DSCSTS.as_slice(),
        save.DSCEPC.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NUMSTATESNOTPOS)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKW05 error case: descriptor times swapped.", ctx)?;

    spicelib::SPKW05(
        HANDLE,
        XBODY,
        XCENTR,
        &XREF,
        save.DSCEPC[NDSCRT],
        save.DSCEPC[1],
        &SEGID,
        GMSUN,
        NDSCRT,
        save.DSCSTS.as_slice(),
        save.DSCEPC.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADDESCRTIMES)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKW05 error case: epochs non-increasing.", ctx)?;

    spicelib::SPKW05(
        HANDLE,
        XBODY,
        XCENTR,
        &XREF,
        (save.DSCEPC[1] - 1.0),
        save.DSCEPC[NDSCRT],
        &SEGID,
        GMSUN,
        NDSCRT,
        save.DSCSTS.as_slice(),
        save.BADEPC.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(UNORDEREDTIMES)", OK, ctx)?;

    //
    // Close the SPK file at the DAF level; SPKCLS won't close
    // a file without segments.
    //
    spicelib::DAFCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //     Enough with the error cases; write a segment already.
    //
    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Create a type 05 segment.", ctx)?;

    spicelib::SPKOPN(
        SPK05,
        b"Type 05 SPK internal file name",
        4,
        &mut HANDLE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKW05(
        HANDLE,
        XBODY,
        XCENTR,
        &XREF,
        save.DSCEPC[1],
        save.DSCEPC[NDSCRT],
        &SEGID,
        GMSUN,
        NDSCRT,
        save.DSCSTS.as_slice(),
        save.DSCEPC.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the SPK file.
    //
    spicelib::SPKCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load the SPK file.
    //
    spicelib::SPKLEF(SPK05, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up states for each epoch in our list.  Compare.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NDSCRT;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::SPKGEO(
                XBODY,
                save.DSCEPC[I],
                &XREF,
                XCENTR,
                STATE.as_slice_mut(),
                &mut LT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                b"type 05 state",
                STATE.as_slice(),
                b"~",
                save.DSCSTS.subarray([1, I]),
                6,
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
        b"SPKW05 test:  create a large segment with multiple directories.",
        ctx,
    )?;

    //
    // Create the state and epoch values we'll use.  Use the
    // standard test SPK file to generate reasonable states.
    //
    if spicelib::EXISTS(SPK1, ctx)? {
        spicelib::DELFIL(SPK1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    testutil::TSTSPK(SPK1, true, &mut HANDLE, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = (BIGN + 1);
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            BEPLST[I] = (BIGSTP * I as f64);

            spicelib::SPKGEO(
                BIGID,
                BEPLST[I],
                b"J2000",
                BIGCTR,
                BSTLST.subarray_mut([1, I]),
                &mut LT,
                ctx,
            )?;

            I += m3__;
        }
    }

    spicelib::SPKUEF(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Open a new type 05 SPK file.
    //
    if spicelib::EXISTS(SPK05B, ctx)? {
        spicelib::DELFIL(SPK05B, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::SPKOPN(
        SPK05B,
        b"Type 05 SPK internal file name",
        0,
        &mut HANDLE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKW05(
        HANDLE,
        BIGID,
        BIGCTR,
        &XREF,
        BEPLST[1],
        BEPLST[BIGN],
        &SEGID,
        GMSUN,
        BIGN,
        BSTLST.as_slice(),
        BEPLST.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the SPK file.
    //
    spicelib::SPKCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load the SPK file.
    //
    spicelib::SPKLEF(SPK05B, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up states for each midpoint of adjacent epochs in our list.
    // Compare.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = (BIGN - 1);
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::SPKGEO(
                BIGID,
                (BEPLST[I] + (BIGSTP / 2 as f64)),
                &XREF,
                BIGCTR,
                STATE.as_slice_mut(),
                &mut LT,
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Set up the expected state vector.  Start out by propagating
            // the state at epoch I forward to the midpoint between epoch I
            // and epoch I+1.
            //
            DT = (BIGSTP / 2 as f64);

            spicelib::PROP2B(
                GMSUN,
                BSTLST.subarray([1, I]),
                DT,
                PROPST.subarray_mut([1, 1]),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Propagate the trajectory backward from epoch I+1:
            //
            spicelib::PROP2B(
                GMSUN,
                BSTLST.subarray([1, (I + 1)]),
                -DT,
                PROPST.subarray_mut([1, 2]),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Combine the positions:
            //
            spicelib::VLCOM(
                0.5,
                PROPST.subarray([1, 1]),
                0.5,
                PROPST.subarray([1, 2]),
                XSTATE.as_slice_mut(),
            );

            testutil::CHCKAD(
                b"type 05 position",
                STATE.as_slice(),
                b"~~/",
                XSTATE.as_slice(),
                3,
                TIGHT,
                OK,
                ctx,
            )?;

            //
            // Combine the velocities.  At the midpoint t between the
            // epochs, the velocity should be
            //
            //    V (t) + V (t)     P (t) - P (t)
            //     1       2         2       1
            //    -------------  +  ------------ * Pi
            //
            //           2           2 * BIGSTP
            //

            spicelib::VLCOM(
                0.5,
                PROPST.subarray([4, 1]),
                0.5,
                PROPST.subarray([4, 2]),
                VCOMP.as_slice_mut(),
            );

            spicelib::VLCOM(
                ((0.5 * spicelib::PI(ctx)) / BIGSTP),
                PROPST.subarray([1, 2]),
                -((0.5 * spicelib::PI(ctx)) / BIGSTP),
                PROPST.subarray([1, 1]),
                PCOMP.as_slice_mut(),
            );

            spicelib::VADD(VCOMP.as_slice(), PCOMP.as_slice(), XSTATE.subarray_mut(4));

            fstr::assign(&mut TITLE, b"type 05 velocity #");
            spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);

            testutil::CHCKAD(
                &TITLE,
                STATE.subarray(4),
                b"~~/",
                XSTATE.subarray(4),
                3,
                MEDIUM,
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
    testutil::TCASE(b"Test SPKS05: write new file having a segment created by subsetting small segment from SPK05.", ctx)?;

    XBODY = 3;
    XCENTR = 10;
    fstr::assign(&mut XREF, b"J2000");

    //
    // Create a segment identifier.
    //
    fstr::assign(&mut SEGID, b"SPK type 05 test subset segment");

    //
    // Open a new SPK file.
    //
    if spicelib::EXISTS(SPK05S, ctx)? {
        spicelib::DELFIL(SPK05S, ctx)?;
    }

    spicelib::SPKOPN(SPK05S, b"Type 05 SPK internal file name", 0, &mut NEWH, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Open SPK05 and extract segment descriptor and ID of first segment.
    //
    spicelib::DAFOPR(SPK05, &mut HANDLE, ctx)?;
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
    // Create a type 05 segment in new file.  Shorten the time
    // coverage by knocking off the coverage contributed by
    // the first and last packets of the source segment.
    //
    spicelib::SPKSUB(
        HANDLE,
        DESCR.as_slice(),
        &SEGID,
        save.DSCEPC[2],
        save.DSCEPC[(NDSCRT - 1)],
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
        b"Test SPKS05: check descriptor bounds on subsetted file.",
        ctx,
    )?;

    //
    // Open SPK05S and extract segment descriptor and ID of first
    // segment.
    //
    spicelib::DAFOPR(SPK05S, &mut HANDLE, ctx)?;
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
    I = (NDSCRT - 1);

    testutil::CHCKSD(b"Segment start", DC[1], b"=", save.DSCEPC[2], 0.0, OK, ctx)?;
    testutil::CHCKSD(b"Segment end", DC[2], b"=", save.DSCEPC[I], 0.0, OK, ctx)?;

    spicelib::SPKCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test SPKS03: read states from subsetted file.", ctx)?;

    //
    // Load the SPK file.
    //
    spicelib::SPKLEF(SPK05S, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XBODY = 3;
    XCENTR = 10;
    fstr::assign(&mut XREF, b"J2000");

    //
    // Look up states for each epoch in our list.  Compare.
    //
    {
        let m1__: i32 = 2;
        let m2__: i32 = (NDSCRT - 1);
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::SPKGEO(
                XBODY,
                save.DSCEPC[I],
                &XREF,
                XCENTR,
                STATE.as_slice_mut(),
                &mut LT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                b"type 05 state",
                STATE.as_slice(),
                b"~",
                save.DSCSTS.subarray([1, I]),
                6,
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
    testutil::TCASE(b"Clean up:  delete SPK files.", ctx)?;

    spicelib::DELFIL(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK05E, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK05, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK05B, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK05S, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
