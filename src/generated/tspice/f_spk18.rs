//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const S18TP0: i32 = 0;
const S18TP1: i32 = (S18TP0 + 1);
const S18PS0: i32 = 12;
const S18PS1: i32 = 6;
const SPK18E: &[u8] = b"test18err.bsp";
const SP18T0: &[u8] = b"sp18t0.bsp";
const SP18T1: &[u8] = b"sp18t1.bsp";
const SP18B0: &[u8] = b"sp18big0.bsp";
const SP18B1: &[u8] = b"sp18big1.bsp";
const SP18SM: &[u8] = b"sp18small.bsp";
const SP18S0: &[u8] = b"sp18sub0.bsp";
const SP18S1: &[u8] = b"sp18sub1.bsp";
const BIGSTP: f64 = 10.0;
const TIGHT: f64 = 0.00000000000001;
const BIGN: i32 = 10101;
const BIGID: i32 = -10000;
const BIGCTR: i32 = 5;
const BIGDEG: i32 = 3;
const DSCSIZ: i32 = 5;
const ND: i32 = 2;
const NDSCRT: i32 = 9;
const NI: i32 = 6;
const POLDEG: i32 = 3;
const SIDLEN: i32 = 60;
const FRNMLN: i32 = 32;

struct SaveVars {
    SEGID: Vec<u8>,
    SEGID2: Vec<u8>,
    XREF: Vec<u8>,
    BEPLST: ActualArray<f64>,
    BT0LST: ActualArray2D<f64>,
    BT1LST: ActualArray2D<f64>,
    DC: StackArray<f64, 2>,
    DESCR: StackArray<f64, 5>,
    DPVAL: f64,
    DSCEPC: StackArray<f64, 9>,
    DSCSTS: StackArray2D<f64, 54>,
    DSCPAK: StackArray2D<f64, 108>,
    LT: f64,
    STATE: StackArray<f64, 6>,
    XSTATE: StackArray<f64, 6>,
    DEGREE: i32,
    EADDR: i32,
    HANDLE: i32,
    I: i32,
    IC: StackArray<i32, 6>,
    N: i32,
    NEWH: i32,
    XBODY: i32,
    XCENTR: i32,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SEGID = vec![b' '; SIDLEN as usize];
        let mut SEGID2 = vec![b' '; SIDLEN as usize];
        let mut XREF = vec![b' '; FRNMLN as usize];
        let mut BEPLST = ActualArray::<f64>::new(1..=BIGN);
        let mut BT0LST = ActualArray2D::<f64>::new(1..=12, 1..=BIGN);
        let mut BT1LST = ActualArray2D::<f64>::new(1..=6, 1..=BIGN);
        let mut DC = StackArray::<f64, 2>::new(1..=ND);
        let mut DESCR = StackArray::<f64, 5>::new(1..=DSCSIZ);
        let mut DPVAL: f64 = 0.0;
        let mut DSCEPC = StackArray::<f64, 9>::new(1..=NDSCRT);
        let mut DSCSTS = StackArray2D::<f64, 54>::new(1..=6, 1..=NDSCRT);
        let mut DSCPAK = StackArray2D::<f64, 108>::new(1..=12, 1..=NDSCRT);
        let mut LT: f64 = 0.0;
        let mut STATE = StackArray::<f64, 6>::new(1..=6);
        let mut XSTATE = StackArray::<f64, 6>::new(1..=6);
        let mut DEGREE: i32 = 0;
        let mut EADDR: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut I: i32 = 0;
        let mut IC = StackArray::<i32, 6>::new(1..=NI);
        let mut N: i32 = 0;
        let mut NEWH: i32 = 0;
        let mut XBODY: i32 = 0;
        let mut XCENTR: i32 = 0;
        let mut FOUND: bool = false;

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
            SEGID,
            SEGID2,
            XREF,
            BEPLST,
            BT0LST,
            BT1LST,
            DC,
            DESCR,
            DPVAL,
            DSCEPC,
            DSCSTS,
            DSCPAK,
            LT,
            STATE,
            XSTATE,
            DEGREE,
            EADDR,
            HANDLE,
            I,
            IC,
            N,
            NEWH,
            XBODY,
            XCENTR,
            FOUND,
        }
    }
}

//$Procedure F_SPK18 ( SPK data type 18 tests )
pub fn F_SPK18(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    testutil::TOPEN(b"F_SPK18", ctx)?;

    //
    //     Test SPKW18:  start out with error handling.
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKW18 error case: bad frame name.", ctx)?;

    save.XBODY = 3;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");

    //
    // Create a segment identifier.
    //
    fstr::assign(&mut save.SEGID, b"SPK type 18 test segment");

    //
    // Open a new SPK file.
    //
    if spicelib::EXISTS(SPK18E, ctx)? {
        spicelib::DELFIL(SPK18E, ctx)?;
    }

    spicelib::SPKOPN(
        SPK18E,
        b"Type 18 SPK internal file name",
        4,
        &mut save.HANDLE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKW18(
        save.HANDLE,
        S18TP1,
        save.XBODY,
        save.XCENTR,
        b"SPUD",
        save.DSCEPC[1],
        save.DSCEPC[NDSCRT],
        &save.SEGID,
        POLDEG,
        NDSCRT,
        save.DSCSTS.as_slice(),
        save.DSCEPC.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDREFFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKW18 error case: SEGID too long.", ctx)?;

    spicelib::SPKW18(
        save.HANDLE,
        S18TP1,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.DSCEPC[1],
        save.DSCEPC[NDSCRT],
        b"X                                               X",
        POLDEG,
        NDSCRT,
        save.DSCSTS.as_slice(),
        save.DSCEPC.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SEGIDTOOLONG)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKW18 error case: unprintable SEGID characters.", ctx)?;

    spicelib::SPKW18(
        save.HANDLE,
        S18TP1,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.DSCEPC[1],
        save.DSCEPC[NDSCRT],
        &intrinsics::CHAR(7),
        POLDEG,
        NDSCRT,
        save.DSCSTS.as_slice(),
        save.DSCEPC.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NONPRINTABLECHARS)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKW18 error case: polynomial degree too high.", ctx)?;

    spicelib::SPKW18(
        save.HANDLE,
        S18TP1,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.DSCEPC[1],
        save.DSCEPC[NDSCRT],
        &save.SEGID,
        25,
        NDSCRT,
        save.DSCSTS.as_slice(),
        save.DSCEPC.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDEGREE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKW18 error case: polynomial degree too low.", ctx)?;

    spicelib::SPKW18(
        save.HANDLE,
        S18TP1,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.DSCEPC[1],
        save.DSCEPC[NDSCRT],
        &save.SEGID,
        0,
        NDSCRT,
        save.DSCSTS.as_slice(),
        save.DSCEPC.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDEGREE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKW18 error case:  odd window size.", ctx)?;

    spicelib::SPKW18(
        save.HANDLE,
        S18TP1,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.DSCEPC[1],
        save.DSCEPC[NDSCRT],
        &save.SEGID,
        4,
        NDSCRT,
        save.DSCSTS.as_slice(),
        save.DSCEPC.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDEGREE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKW18 error case: too few states", ctx)?;

    spicelib::SPKW18(
        save.HANDLE,
        S18TP1,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.DSCEPC[1],
        save.DSCEPC[1],
        &save.SEGID,
        POLDEG,
        0,
        save.DSCSTS.as_slice(),
        save.DSCEPC.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(TOOFEWSTATES)", OK, ctx)?;

    testutil::TCASE(b"SPKW18 error case: too few states", ctx)?;

    spicelib::SPKW18(
        save.HANDLE,
        S18TP1,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.DSCEPC[1],
        save.DSCEPC[1],
        &save.SEGID,
        POLDEG,
        1,
        save.DSCSTS.as_slice(),
        save.DSCEPC.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(TOOFEWSTATES)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKW18 error case: descriptor times swapped.", ctx)?;

    spicelib::SPKW18(
        save.HANDLE,
        S18TP1,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.DSCEPC[NDSCRT],
        save.DSCEPC[1],
        &save.SEGID,
        POLDEG,
        NDSCRT,
        save.DSCSTS.as_slice(),
        save.DSCEPC.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADDESCRTIMES)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SPKW18 error case: descriptor start time is too early.",
        ctx,
    )?;

    spicelib::SPKW18(
        save.HANDLE,
        S18TP1,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        (save.DSCEPC[1] - 1.0),
        save.DSCEPC[NDSCRT],
        &save.SEGID,
        POLDEG,
        NDSCRT,
        save.DSCSTS.as_slice(),
        save.DSCEPC.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADDESCRTIMES)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKW18 error case: descriptor end time is too late.", ctx)?;

    spicelib::SPKW18(
        save.HANDLE,
        S18TP1,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.DSCEPC[1],
        (save.DSCEPC[NDSCRT] + 1.0),
        &save.SEGID,
        POLDEG,
        NDSCRT,
        save.DSCSTS.as_slice(),
        save.DSCEPC.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADDESCRTIMES)", OK, ctx)?;

    //
    // Close the SPK file at the DAF level; SPKCLS won't close
    // a file without segments.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //     Enough with the error cases; write a segment already.
    //
    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Create a type 18 subtype 1 segment.", ctx)?;

    spicelib::SPKOPN(
        SP18T1,
        b"Type 18 SPK internal file name",
        4,
        &mut save.HANDLE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKW18(
        save.HANDLE,
        S18TP1,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.DSCEPC[1],
        save.DSCEPC[NDSCRT],
        &save.SEGID,
        POLDEG,
        NDSCRT,
        save.DSCSTS.as_slice(),
        save.DSCEPC.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the SPK file.
    //
    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load the SPK file.
    //
    spicelib::SPKLEF(SP18T1, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up states for each epoch in our list.  Compare.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NDSCRT;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::SPKGEO(
                save.XBODY,
                save.DSCEPC[save.I],
                &save.XREF,
                save.XCENTR,
                save.STATE.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                b"type 18 state",
                save.STATE.as_slice(),
                b"~",
                save.DSCSTS.subarray([1, save.I]),
                6,
                TIGHT,
                OK,
                ctx,
            )?;
            save.I += m3__;
        }
    }

    //
    // Unload the SPK file.
    //
    spicelib::SPKUEF(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Repeat this test using subtype 0.
    //

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Create a type 18 subtype 0 segment.", ctx)?;

    spicelib::SPKOPN(
        SP18T0,
        b"Type 18 SPK internal file name",
        4,
        &mut save.HANDLE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Fill in the packets using the discrete state set.  Make the
    // velocity/acceleration data 10x the position/velocity data.  Note
    // that these two data sets are independent; realistic
    // correspondence between them is not required.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NDSCRT;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::MOVED(
                save.DSCSTS.subarray([1, save.I]),
                6,
                save.DSCPAK.subarray_mut([1, save.I]),
            );

            spicelib::VSCLG(
                10.0,
                save.DSCSTS.subarray([1, save.I]),
                6,
                save.DSCPAK.subarray_mut([7, save.I]),
            );

            save.I += m3__;
        }
    }

    spicelib::SPKW18(
        save.HANDLE,
        S18TP0,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.DSCEPC[1],
        save.DSCEPC[NDSCRT],
        &save.SEGID,
        POLDEG,
        NDSCRT,
        save.DSCPAK.as_slice(),
        save.DSCEPC.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the SPK file.
    //
    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load the SPK file.
    //
    spicelib::SPKLEF(SP18T0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up states for each epoch in our list.  Compare.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NDSCRT;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::SPKGEO(
                save.XBODY,
                save.DSCEPC[save.I],
                &save.XREF,
                save.XCENTR,
                save.STATE.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::MOVED(
                save.DSCSTS.subarray([1, save.I]),
                3,
                save.XSTATE.subarray_mut(1),
            );
            spicelib::VSCL(
                10.0,
                save.DSCSTS.subarray([1, save.I]),
                save.XSTATE.subarray_mut(4),
            );

            testutil::CHCKAD(
                b"type 18 state",
                save.STATE.as_slice(),
                b"~",
                save.XSTATE.as_slice(),
                6,
                TIGHT,
                OK,
                ctx,
            )?;
            save.I += m3__;
        }
    }

    //
    // Unload the SPK file.
    //
    spicelib::SPKUEF(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SPKW18 test:  create a large subtype 1 segment with multiple directories.",
        ctx,
    )?;

    //
    // Create the state and epoch values we'll use.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = BIGN;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            for J in 1..=6 {
                save.BT1LST[[J, save.I]] = ((BIGSTP * save.I as f64) + J as f64);
            }

            save.BEPLST[save.I] = (10.0 * save.I as f64);

            save.I += m3__;
        }
    }

    //
    // Open a new type 18 SPK file.
    //
    if spicelib::EXISTS(SP18B1, ctx)? {
        spicelib::DELFIL(SP18B1, ctx)?;
    }

    spicelib::SPKOPN(
        SP18B1,
        b"Type 18 SPK internal file name",
        0,
        &mut save.HANDLE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKW18(
        save.HANDLE,
        S18TP1,
        BIGID,
        BIGCTR,
        &save.XREF,
        save.BEPLST[1],
        save.BEPLST[BIGN],
        &save.SEGID,
        BIGDEG,
        BIGN,
        save.BT1LST.as_slice(),
        save.BEPLST.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the SPK file.
    //
    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load the SPK file.
    //
    spicelib::SPKLEF(SP18B1, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up states for each midpoint of adjacent epochs in our list.
    // Compare.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = (BIGN - 1);
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::SPKGEO(
                BIGID,
                (save.BEPLST[save.I] + (BIGSTP / 2 as f64)),
                &save.XREF,
                BIGCTR,
                save.STATE.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Set up the expected state vector.
            //
            spicelib::MOVED(
                save.BT1LST.subarray([1, save.I]),
                6,
                save.XSTATE.as_slice_mut(),
            );

            for J in 1..=6 {
                save.XSTATE[J] = (save.XSTATE[J] + (BIGSTP / 2 as f64));
            }

            testutil::CHCKAD(
                b"type 18 position",
                save.STATE.as_slice(),
                b"~",
                save.XSTATE.as_slice(),
                3,
                TIGHT,
                OK,
                ctx,
            )?;

            testutil::CHCKAD(
                b"type 18 velocity",
                save.STATE.subarray(4),
                b"~",
                save.XSTATE.subarray(4),
                3,
                TIGHT,
                OK,
                ctx,
            )?;
            save.I += m3__;
        }
    }

    //
    // Unload the SPK file.
    //
    spicelib::SPKUEF(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SPKW18 test:  create a large subtype 0 segment with multiple directories.",
        ctx,
    )?;

    //
    // Create the state and epoch values we'll use. We're going to set
    // all velocities to zero to create a rounded stair-step sort of
    // pattern in the position components. This will ensure that the
    // correct states cannot be obtained without selecting the correct
    // window of states in the reader.
    //
    // For velocity and acceleration, we'll use the same idea, but we'll
    // scale the values to distinguish them.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = BIGN;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            for J in 1..=3 {
                save.BT0LST[[J, save.I]] = ((BIGSTP * save.I as f64) + J as f64);
                save.BT0LST[[(J + 3), save.I]] = 0.0;
                save.BT0LST[[(J + 6), save.I]] = (1000000.0 * save.BT0LST[[J, save.I]]);
                save.BT0LST[[(J + 9), save.I]] = 0.0;
            }

            save.BEPLST[save.I] = (10.0 * save.I as f64);

            save.I += m3__;
        }
    }

    //
    // Open a new type 18 SPK file.
    //
    if spicelib::EXISTS(SP18B0, ctx)? {
        spicelib::DELFIL(SP18B0, ctx)?;
    }

    spicelib::SPKOPN(
        SP18B0,
        b"Type 18 SPK internal file name",
        0,
        &mut save.HANDLE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKW18(
        save.HANDLE,
        S18TP0,
        BIGID,
        BIGCTR,
        &save.XREF,
        save.BEPLST[1],
        save.BEPLST[BIGN],
        &save.SEGID,
        BIGDEG,
        BIGN,
        save.BT0LST.as_slice(),
        save.BEPLST.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the SPK file.
    //
    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load the SPK file.
    //
    spicelib::SPKLEF(SP18B0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up states for each midpoint of adjacent epochs in our list.
    // Compare.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = (BIGN - 1);
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::SPKGEO(
                BIGID,
                (save.BEPLST[save.I] + (BIGSTP / 2 as f64)),
                &save.XREF,
                BIGCTR,
                save.STATE.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Set up the expected state vector.
            //
            spicelib::MOVED(
                save.BT0LST.subarray([1, save.I]),
                3,
                save.XSTATE.as_slice_mut(),
            );

            for J in 1..=3 {
                save.XSTATE[J] = (save.XSTATE[J] + (BIGSTP / 2 as f64));
                save.XSTATE[(J + 3)] = (1000000.0 * save.XSTATE[J]);
            }

            testutil::CHCKAD(
                b"type 18 position",
                save.STATE.as_slice(),
                b"~",
                save.XSTATE.as_slice(),
                3,
                TIGHT,
                OK,
                ctx,
            )?;

            testutil::CHCKAD(
                b"type 18 velocity",
                save.STATE.subarray(4),
                b"~",
                save.XSTATE.subarray(4),
                3,
                TIGHT,
                OK,
                ctx,
            )?;
            save.I += m3__;
        }
    }

    //
    // Unload the SPK file.
    //
    spicelib::SPKUEF(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     Create an SPK file suitable for use as an input file
    //     for SPKS18. This file will contain segments of subtypes
    //     0 and 1.
    //
    testutil::TCASE(b"Create a type 18 SPK for subset testing. This file contains segments of subtypes 0 and 1.", ctx)?;

    spicelib::SPKOPN(
        SP18SM,
        b"Type 18 SPK internal file name",
        4,
        &mut save.HANDLE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Add first segment, which has subtype 1 and degree 15.
    //
    save.DEGREE = 15;

    spicelib::SPKW18(
        save.HANDLE,
        S18TP1,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.DSCEPC[1],
        save.DSCEPC[NDSCRT],
        &save.SEGID,
        save.DEGREE,
        NDSCRT,
        save.DSCSTS.as_slice(),
        save.DSCEPC.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Add second segment, which has subtype 0 and degree 15.
    // This file contains only 5 packets.
    //
    // We must use a different body for this segment.
    //
    // Note the use of DSCPAK rather than DSKSTS. DSKPAK
    // was initialized earlier.
    //
    save.XBODY = 4;

    spicelib::SPKW18(
        save.HANDLE,
        S18TP0,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.DSCEPC[5],
        save.DSCEPC[7],
        &save.SEGID,
        save.DEGREE,
        5,
        save.DSCPAK.subarray([1, 4]),
        save.DSCEPC.subarray(4),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the SPK file.
    //
    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test SPKS18: write new file having a subtype 1 segment created by subsetting small segment from SP18SM.", ctx)?;

    save.XBODY = 3;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");

    //
    // Create a segment identifier.
    //
    fstr::assign(&mut save.SEGID, b"SPK type 18 test subset segment");

    //
    // Open a new SPK file.
    //
    if spicelib::EXISTS(SP18S1, ctx)? {
        spicelib::DELFIL(SP18S1, ctx)?;
    }

    spicelib::SPKOPN(
        SP18S1,
        b"Type 18 SPK internal file name",
        0,
        &mut save.NEWH,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Open SP18T1 and extract segment descriptor and ID of first
    // segment.
    //
    spicelib::DAFOPR(SP18SM, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGN(&mut save.SEGID2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a type 18 segment in new file.  Shorten the time
    // coverage by knocking off the coverage contributed by
    // the first and last packets of the source segment.
    //
    spicelib::SPKSUB(
        save.HANDLE,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.DSCEPC[2],
        save.DSCEPC[(NDSCRT - 1)],
        save.NEWH,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the new SPK file.
    //
    spicelib::SPKCLS(save.NEWH, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the old SPK file.
    //
    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Test SPKS18: check descriptor bounds on subsetted file (subtype 1).",
        ctx,
    )?;

    //
    // Open SP18S1 and extract segment descriptor and ID of first
    // segment.
    //
    spicelib::DAFOPR(SP18S1, &mut save.HANDLE, ctx)?;
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
    // Check the time bounds.
    //
    save.I = (NDSCRT - 1);

    testutil::CHCKSD(
        b"Segment start",
        save.DC[1],
        b"=",
        save.DSCEPC[2],
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"Segment end",
        save.DC[2],
        b"=",
        save.DSCEPC[save.I],
        0.0,
        OK,
        ctx,
    )?;

    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Test SPKS18: check state count in subsetted file (subtype 1).",
        ctx,
    )?;

    //
    // Load the SPK file.
    //
    spicelib::SPKLEF(SP18S1, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get descriptor of first segment.
    //
    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

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

    save.EADDR = save.IC[6];

    spicelib::DAFGDA(
        save.HANDLE,
        save.EADDR,
        save.EADDR,
        std::slice::from_mut(&mut save.DPVAL),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = intrinsics::IDNINT(save.DPVAL);

    testutil::CHCKSI(b"N", save.N, b"=", NDSCRT, 0, OK, ctx)?;

    //
    // Unload the SPK file.
    //
    spicelib::SPKUEF(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Test SPKS18: read states from subsetted file (subtype 1).",
        ctx,
    )?;

    //
    // Load the SPK file.
    //
    spicelib::SPKLEF(SP18S1, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XBODY = 3;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");

    //
    // Look up states for each epoch in our list.  Compare.
    //
    {
        let m1__: i32 = 4;
        let m2__: i32 = 8;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::SPKGEO(
                save.XBODY,
                save.DSCEPC[save.I],
                &save.XREF,
                save.XCENTR,
                save.STATE.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                b"type 18 position",
                save.STATE.as_slice(),
                b"~",
                save.DSCSTS.subarray([1, save.I]),
                3,
                TIGHT,
                OK,
                ctx,
            )?;

            testutil::CHCKAD(
                b"type 18 velocity",
                save.STATE.subarray(4),
                b"~",
                save.DSCSTS.subarray([4, save.I]),
                3,
                TIGHT,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // Unload the SPK file.
    //
    spicelib::SPKUEF(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //     Repeat subsetting tests for a subtype 0 segment.
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test SPKS18: write new file having a segment created by subsetting small segment from SP18SM.The output segment has subtype 0.", ctx)?;

    save.XBODY = 4;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");

    //
    // Create a segment identifier.
    //
    fstr::assign(&mut save.SEGID, b"SPK type 18 test subset segment");

    //
    // Open a new SPK file.
    //
    if spicelib::EXISTS(SP18S0, ctx)? {
        spicelib::DELFIL(SP18S0, ctx)?;
    }

    spicelib::SPKOPN(
        SP18S0,
        b"Type 18 SPK internal file name",
        0,
        &mut save.NEWH,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Open SP18SM and extract segment descriptor and ID of second
    // segment.
    //
    spicelib::DAFOPR(SP18SM, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 2;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::DAFFNA(&mut save.FOUND, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            save.I += m3__;
        }
    }

    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGN(&mut save.SEGID2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a type 18 segment in new file.  Shorten the time
    // coverage by knocking off the coverage contributed by
    // the first and last packets of the source segment.
    //
    spicelib::SPKSUB(
        save.HANDLE,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.DSCEPC[5],
        save.DSCEPC[7],
        save.NEWH,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the new SPK file.
    //
    spicelib::SPKCLS(save.NEWH, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the old SPK file.
    //
    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Test SPKS18: check descriptor bounds on subsetted file (subtype 0).",
        ctx,
    )?;

    //
    // Open SP18S0 and extract segment descriptor and ID of first
    // segment.
    //
    spicelib::DAFOPR(SP18S0, &mut save.HANDLE, ctx)?;
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
    // Check the time bounds.
    //
    testutil::CHCKSD(
        b"Segment start",
        save.DC[1],
        b"=",
        save.DSCEPC[5],
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"Segment end",
        save.DC[2],
        b"=",
        save.DSCEPC[7],
        0.0,
        OK,
        ctx,
    )?;

    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Test SPKS18: check state count in subsetted file (subtype 0).",
        ctx,
    )?;

    //
    // Load the SPK file.
    //
    spicelib::SPKLEF(SP18S0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get descriptor of first segment.
    //
    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

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

    save.EADDR = save.IC[6];

    spicelib::DAFGDA(
        save.HANDLE,
        save.EADDR,
        save.EADDR,
        std::slice::from_mut(&mut save.DPVAL),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = intrinsics::IDNINT(save.DPVAL);

    testutil::CHCKSI(b"N", save.N, b"=", 5, 0, OK, ctx)?;

    //
    // Unload the SPK file.
    //
    spicelib::SPKUEF(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Test SPKS18: read states from subsetted file (subtype 0).",
        ctx,
    )?;

    //
    // Load the SPK file.
    //
    spicelib::SPKLEF(SP18SM, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XBODY = 4;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");

    //
    // Look up states for each epoch in our list.  Compare.
    //
    {
        let m1__: i32 = 5;
        let m2__: i32 = 7;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::SPKGEO(
                save.XBODY,
                save.DSCEPC[save.I],
                &save.XREF,
                save.XCENTR,
                save.STATE.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                b"type 18 position",
                save.STATE.as_slice(),
                b"~",
                save.DSCPAK.subarray([1, save.I]),
                3,
                TIGHT,
                OK,
                ctx,
            )?;

            testutil::CHCKAD(
                b"type 18 velocity",
                save.STATE.subarray(4),
                b"~",
                save.DSCPAK.subarray([7, save.I]),
                3,
                TIGHT,
                OK,
                ctx,
            )?;
            save.I += m3__;
        }
    }

    //
    // Unload the SPK file.
    //
    spicelib::SPKUEF(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up:  delete SPK files.", ctx)?;

    spicelib::DELFIL(SPK18E, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SP18T0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SP18T1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SP18B0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SP18B1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SP18S0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SP18S1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SP18SM, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
