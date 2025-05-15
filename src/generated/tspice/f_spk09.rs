//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const SPK09E: &[u8] = b"test09err.bsp";
const SPK09: &[u8] = b"test09.bsp";
const SPK09B: &[u8] = b"test09big.bsp";
const SPK09S: &[u8] = b"test09sub.bsp";
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
    DSCEPC: StackArray<f64, 9>,
    DSCSTS: StackArray2D<f64, 54>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
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

        Self { DSCEPC, DSCSTS }
    }
}

//$Procedure F_SPK09 ( SPK data type 09 tests )
pub fn F_SPK09(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut SEGID = [b' '; SIDLEN as usize];
    let mut SEGID2 = [b' '; SIDLEN as usize];
    let mut XREF = [b' '; FRNMLN as usize];
    let mut BEPLST = ActualArray::<f64>::new(1..=BIGN);
    let mut BSTLST = ActualArray2D::<f64>::new(1..=6, 1..=BIGN);
    let mut DC = StackArray::<f64, 2>::new(1..=ND);
    let mut DESCR = StackArray::<f64, 5>::new(1..=DSCSIZ);
    let mut LT: f64 = 0.0;
    let mut STATE = StackArray::<f64, 6>::new(1..=6);
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
    // Epochs and states:
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_SPK09", ctx)?;

    //
    //     Test SPKW09:  start out with error handling.
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKW09 error case: bad frame name.", ctx)?;

    XBODY = 3;
    XCENTR = 10;
    fstr::assign(&mut XREF, b"J2000");

    //
    // Create a segment identifier.
    //
    fstr::assign(&mut SEGID, b"SPK type 09 test segment");

    //
    // Open a new SPK file.
    //
    if spicelib::EXISTS(SPK09E, ctx)? {
        spicelib::DELFIL(SPK09E, ctx)?;
    }

    spicelib::SPKOPN(
        SPK09E,
        b"Type 09 SPK internal file name",
        4,
        &mut HANDLE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKW09(
        HANDLE,
        XBODY,
        XCENTR,
        b"SPUD",
        save.DSCEPC[1],
        save.DSCEPC[NDSCRT],
        &SEGID,
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
    testutil::TCASE(b"SPKW09 error case: SEGID too long.", ctx)?;

    spicelib::SPKW09(
        HANDLE,
        XBODY,
        XCENTR,
        &XREF,
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
    testutil::TCASE(b"SPKW09 error case: unprintable SEGID characters.", ctx)?;

    spicelib::SPKW09(
        HANDLE,
        XBODY,
        XCENTR,
        &XREF,
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
    testutil::TCASE(b"SPKW09 error case: polynomial degree too high.", ctx)?;

    spicelib::SPKW09(
        HANDLE,
        XBODY,
        XCENTR,
        &XREF,
        save.DSCEPC[1],
        save.DSCEPC[NDSCRT],
        &SEGID,
        31,
        NDSCRT,
        save.DSCSTS.as_slice(),
        save.DSCEPC.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDEGREE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPKW09 error case: too few states", ctx)?;

    spicelib::SPKW09(
        HANDLE,
        XBODY,
        XCENTR,
        &XREF,
        save.DSCEPC[1],
        save.DSCEPC[NDSCRT],
        &SEGID,
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
    testutil::TCASE(b"SPKW09 error case: descriptor times swapped.", ctx)?;

    spicelib::SPKW09(
        HANDLE,
        XBODY,
        XCENTR,
        &XREF,
        save.DSCEPC[NDSCRT],
        save.DSCEPC[1],
        &SEGID,
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
        b"SPKW09 error case: descriptor start time is too early.",
        ctx,
    )?;

    spicelib::SPKW09(
        HANDLE,
        XBODY,
        XCENTR,
        &XREF,
        (save.DSCEPC[1] - 1.0),
        save.DSCEPC[NDSCRT],
        &SEGID,
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
    testutil::TCASE(b"SPKW09 error case: descriptor end time is too late.", ctx)?;

    spicelib::SPKW09(
        HANDLE,
        XBODY,
        XCENTR,
        &XREF,
        save.DSCEPC[1],
        (save.DSCEPC[NDSCRT] + 1.0),
        &SEGID,
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
    spicelib::DAFCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //     Enough with the error cases; write a segment already.
    //
    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Create a type 09 segment.", ctx)?;

    spicelib::SPKOPN(
        SPK09,
        b"Type 09 SPK internal file name",
        4,
        &mut HANDLE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKW09(
        HANDLE,
        XBODY,
        XCENTR,
        &XREF,
        save.DSCEPC[1],
        save.DSCEPC[NDSCRT],
        &SEGID,
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
    spicelib::SPKCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load the SPK file.
    //
    spicelib::SPKLEF(SPK09, &mut HANDLE, ctx)?;
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
                b"type 09 state",
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
        b"SPKW09 test:  create a large segment with multiple directories.",
        ctx,
    )?;

    //
    // Create the state and epoch values we'll use.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = BIGN;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            for J in 1..=3 {
                BSTLST[[J, I]] = ((BIGSTP * I as f64) + J as f64);
                BSTLST[[(J + 3), I]] = -BSTLST[[J, I]];
            }

            BEPLST[I] = (10.0 * I as f64);

            I += m3__;
        }
    }

    //
    // Open a new type 09 SPK file.
    //
    if spicelib::EXISTS(SPK09B, ctx)? {
        spicelib::DELFIL(SPK09B, ctx)?;
    }

    spicelib::SPKOPN(
        SPK09B,
        b"Type 09 SPK internal file name",
        0,
        &mut HANDLE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKW09(
        HANDLE,
        BIGID,
        BIGCTR,
        &XREF,
        BEPLST[1],
        BEPLST[BIGN],
        &SEGID,
        BIGDEG,
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
    spicelib::SPKLEF(SPK09B, &mut HANDLE, ctx)?;
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
            // Set up the expected state vector.
            //
            spicelib::MOVED(BSTLST.subarray([1, I]), 6, XSTATE.as_slice_mut());

            for J in 1..=3 {
                XSTATE[J] = (XSTATE[J] + (BIGSTP / 2 as f64));
                XSTATE[(J + 3)] = -XSTATE[J];
            }

            testutil::CHCKAD(
                b"type 09 position",
                STATE.as_slice(),
                b"~",
                XSTATE.as_slice(),
                3,
                TIGHT,
                OK,
                ctx,
            )?;

            testutil::CHCKAD(
                b"type 09 velocity",
                STATE.subarray(4),
                b"~",
                XSTATE.subarray(4),
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
    testutil::TCASE(b"Test SPKS09: write new file having a segment created by subsetting small segment from SPK09.", ctx)?;

    XBODY = 3;
    XCENTR = 10;
    fstr::assign(&mut XREF, b"J2000");

    //
    // Create a segment identifier.
    //
    fstr::assign(&mut SEGID, b"SPK type 09 test subset segment");

    //
    // Open a new SPK file.
    //
    if spicelib::EXISTS(SPK09S, ctx)? {
        spicelib::DELFIL(SPK09S, ctx)?;
    }

    spicelib::SPKOPN(SPK09S, b"Type 09 SPK internal file name", 0, &mut NEWH, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Open SPK09 and extract segment descriptor and ID of first segment.
    //
    spicelib::DAFOPR(SPK09, &mut HANDLE, ctx)?;
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
    // Create a type 09 segment in new file.  Shorten the time
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
        b"Test SPKS09: check descriptor bounds on subsetted file.",
        ctx,
    )?;

    //
    // Open SPK09S and extract segment descriptor and ID of first
    // segment.
    //
    spicelib::DAFOPR(SPK09S, &mut HANDLE, ctx)?;
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
    spicelib::SPKLEF(SPK09S, &mut HANDLE, ctx)?;
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
                b"type 09 state",
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

    spicelib::DELFIL(SPK09E, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK09, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK09B, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK09S, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
