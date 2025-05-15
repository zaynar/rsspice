//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;
const WDSIZE: i32 = 32;
const LWSIZE: i32 = 40;
const NLINES: i32 = 46;

//$Procedure      F_SPKGEO ( Family of tests for SPKGEO )
pub fn F_SPKGEO(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ERROR = [b' '; LNSIZE as usize];
    let mut LINES = ActualCharArray::new(LNSIZE, 1..=NLINES);
    let mut SEGID = [b' '; LWSIZE as usize];
    let mut REF = [b' '; WDSIZE as usize];
    let mut CENTER: i32 = 0;
    let mut CKHAN: i32 = 0;
    let mut IAUE: i32 = 0;
    let mut IDCODE = StackArray::<i32, 46>::new(1..=46);
    let mut OBS: i32 = 0;
    let mut REF1: i32 = 0;
    let mut REF2: i32 = 0;
    let mut REF3: i32 = 0;
    let mut REF4: i32 = 0;
    let mut REF5: i32 = 0;
    let mut REF6: i32 = 0;
    let mut REF7: i32 = 0;
    let mut SPKHAN: i32 = 0;
    let mut TARG: i32 = 0;
    let mut ET: f64 = 0.0;
    let mut GM: f64 = 0.0;
    let mut IAUST1 = StackArray::<f64, 6>::new(1..=6);
    let mut IAUST2 = StackArray::<f64, 6>::new(1..=6);
    let mut IAUST3 = StackArray::<f64, 6>::new(1..=6);
    let mut IAUST4 = StackArray::<f64, 6>::new(1..=6);
    let mut IAUST5 = StackArray::<f64, 6>::new(1..=6);
    let mut IAUST6 = StackArray::<f64, 6>::new(1..=6);
    let mut IAUST7 = StackArray::<f64, 6>::new(1..=6);
    let mut LT: f64 = 0.0;
    let mut LT2: f64 = 0.0;
    let mut STATE = StackArray::<f64, 6>::new(1..=6);
    let mut STATE1 = StackArray::<f64, 6>::new(1..=6);
    let mut STATE2 = StackArray::<f64, 6>::new(1..=6);
    let mut STATE3 = StackArray::<f64, 6>::new(1..=6);
    let mut STATE4 = StackArray::<f64, 6>::new(1..=6);
    let mut STATE5 = StackArray::<f64, 6>::new(1..=6);
    let mut STATE6 = StackArray::<f64, 6>::new(1..=6);
    let mut STATE7 = StackArray::<f64, 6>::new(1..=6);
    let mut TSIPM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut TSTATE = StackArray::<f64, 6>::new(1..=6);
    let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XFORM1 = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XFORM2 = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XFORM3 = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XFORM4 = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XFORM5 = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XFORM6 = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XFORM7 = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut STXFRM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);

    //
    // Spicelib Routines.
    //

    //
    // Local Variables
    //

    testutil::BEGTXT(&mut LINES[1]);
    fstr::assign(
        LINES.get_mut(2),
        b"This PCK file contains definitions for TOPOGRAPHIC reference",
    );
    fstr::assign(
        LINES.get_mut(3),
        b"frames at 3 different observatories around the world.  Note",
    );
    fstr::assign(
        LINES.get_mut(4),
        b"that the definition of these frames is approximate and that",
    );
    fstr::assign(
        LINES.get_mut(5),
        b"they are accurate to only about 0.1 degrees.",
    );
    fstr::assign(LINES.get_mut(6), b" ");
    testutil::BEGDAT(&mut LINES[7]);
    fstr::assign(LINES.get_mut(8), b" ");
    fstr::assign(LINES.get_mut(9), b"FRAME_CANBERRA_TOPO    = 1000002");
    fstr::assign(LINES.get_mut(10), b"FRAME_MADRID_TOPO      = 1000019");
    fstr::assign(LINES.get_mut(11), b"FRAME_GOLDSTONE_TOPO   = 1000023");
    fstr::assign(LINES.get_mut(12), b" ");
    fstr::assign(LINES.get_mut(13), b" ");
    fstr::assign(LINES.get_mut(14), b"FRAME_1000002_CENTER   =  399");
    fstr::assign(LINES.get_mut(15), b"FRAME_1000002_CLASS    =  4");
    fstr::assign(LINES.get_mut(16), b"FRAME_1000002_CLASS_ID = 1000002");
    fstr::assign(
        LINES.get_mut(17),
        b"FRAME_1000002_NAME     = \'CANBERRA_TOPO\'",
    );
    fstr::assign(LINES.get_mut(18), b" ");
    fstr::assign(LINES.get_mut(19), b"FRAME_1000019_CENTER   =  399");
    fstr::assign(LINES.get_mut(20), b"FRAME_1000019_CLASS    =  4");
    fstr::assign(LINES.get_mut(21), b"FRAME_1000019_CLASS_ID = 1000019");
    fstr::assign(
        LINES.get_mut(22),
        b"FRAME_1000019_NAME     = \'MADRID_TOPO\'",
    );
    fstr::assign(LINES.get_mut(23), b" ");
    fstr::assign(LINES.get_mut(24), b"FRAME_1000023_CENTER   =  399");
    fstr::assign(LINES.get_mut(25), b"FRAME_1000023_CLASS    =  4");
    fstr::assign(LINES.get_mut(26), b"FRAME_1000023_CLASS_ID = 1000023");
    fstr::assign(
        LINES.get_mut(27),
        b"FRAME_1000023_NAME     = \'GOLDSTONE_TOPO\'",
    );
    fstr::assign(LINES.get_mut(28), b" ");
    fstr::assign(LINES.get_mut(29), b" ");
    fstr::assign(
        LINES.get_mut(30),
        b"TKFRAME_1000002_ANGLES   = ( -149.0, -125.3, 180 )",
    );
    fstr::assign(
        LINES.get_mut(31),
        b"TKFRAME_1000002_AXES     = (       3,        2,   3 )",
    );
    fstr::assign(
        LINES.get_mut(32),
        b"TKFRAME_1000002_RELATIVE = \'IAU_EARTH\'",
    );
    fstr::assign(LINES.get_mut(33), b"TKFRAME_1000002_SPEC     = \'ANGLES\'");
    fstr::assign(LINES.get_mut(34), b"TKFRAME_1000002_UNITS    = \'DEGREES\'");
    fstr::assign(LINES.get_mut(35), b" ");
    fstr::assign(
        LINES.get_mut(36),
        b"TKFRAME_1000019_ANGLES   = ( 03.7, -49.6, 180 )",
    );
    fstr::assign(
        LINES.get_mut(37),
        b"TKFRAME_1000019_AXES     = (       3,        2,   3 )",
    );
    fstr::assign(
        LINES.get_mut(38),
        b"TKFRAME_1000019_RELATIVE = \'IAU_EARTH\'",
    );
    fstr::assign(LINES.get_mut(39), b"TKFRAME_1000019_SPEC     = \'ANGLES\'");
    fstr::assign(LINES.get_mut(40), b"TKFRAME_1000019_UNITS    = \'DEGREES\'");
    fstr::assign(LINES.get_mut(41), b" ");
    fstr::assign(
        LINES.get_mut(42),
        b"TKFRAME_1000023_ANGLES   = ( 116.8, -54.6, 180 )",
    );
    fstr::assign(
        LINES.get_mut(43),
        b"TKFRAME_1000023_AXES     = (       3,        2,   3 )",
    );
    fstr::assign(
        LINES.get_mut(44),
        b"TKFRAME_1000023_RELATIVE = \'IAU_EARTH\'",
    );
    fstr::assign(LINES.get_mut(45), b"TKFRAME_1000023_SPEC     = \'ANGLES\'");
    fstr::assign(LINES.get_mut(46), b"TKFRAME_1000023_UNITS    = \'DEGREES\'");

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_SPKGEO", ctx)?;

    //
    // Set up the data needed for testing.
    //

    testutil::KILFIL(b"test_pck.ker", ctx)?;
    testutil::TSTPCK(b"test_pck.ker", true, false, ctx)?;
    testutil::TSTCK3(
        b"phoenix.bc",
        b"phoenix.tsc",
        false,
        true,
        false,
        &mut CKHAN,
        ctx,
    )?;
    spicelib::CKLPF(b"phoenix.bc", &mut CKHAN, ctx)?;
    testutil::TSTSPK(b"test_spk.bsp", true, &mut SPKHAN, ctx)?;

    IDCODE[1] = 1;
    IDCODE[2] = 2;
    IDCODE[3] = 3;
    IDCODE[4] = 4;
    IDCODE[5] = 5;
    IDCODE[6] = 6;
    IDCODE[7] = 7;
    IDCODE[8] = 8;
    IDCODE[9] = 9;
    IDCODE[10] = 301;
    IDCODE[11] = 401;
    IDCODE[12] = 402;
    IDCODE[13] = 501;
    IDCODE[14] = 502;
    IDCODE[15] = 503;
    IDCODE[16] = 504;
    IDCODE[17] = 603;
    IDCODE[18] = 604;
    IDCODE[19] = 605;
    IDCODE[20] = 606;
    IDCODE[21] = 607;
    IDCODE[22] = 608;
    IDCODE[23] = 701;
    IDCODE[24] = 702;
    IDCODE[25] = 703;
    IDCODE[26] = 704;
    IDCODE[27] = 705;
    IDCODE[28] = 801;
    IDCODE[29] = 802;
    IDCODE[30] = 901;
    IDCODE[31] = 199;
    IDCODE[32] = 299;
    IDCODE[33] = 399;
    IDCODE[34] = 499;
    IDCODE[35] = 599;
    IDCODE[36] = 699;
    IDCODE[37] = 799;
    IDCODE[38] = 899;
    IDCODE[39] = 999;
    IDCODE[40] = 10;
    IDCODE[41] = 399001;
    IDCODE[42] = 399002;
    IDCODE[43] = 399003;
    IDCODE[44] = -9;
    IDCODE[45] = 401001;
    IDCODE[46] = 301001;

    ET = 0.0;

    testutil::TCASE(b"Check that exceptions behave as expected.", ctx)?;

    spicelib::SPKGEO(-10, ET, b"J2000", 399, STATE.as_slice_mut(), &mut LT, ctx)?;
    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    testutil::TCASE(b"Get the state of the moon in the earth bodyfixed frame and compare it to the old fashioned way of computing this. ", ctx)?;

    spicelib::SPKGEO(
        301,
        ET,
        b"IAU_EARTH",
        399,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    SPKGEO_O(301, ET, b"J2000", 399, STATE1.as_slice_mut(), &mut LT2, ctx)?;
    spicelib::TISBOD(b"J2000", 399, ET, TSIPM.as_slice_mut(), ctx)?;
    spicelib::MXVG(
        TSIPM.as_slice(),
        STATE1.as_slice(),
        6,
        6,
        STATE2.as_slice_mut(),
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"~/",
        STATE2.as_slice(),
        6,
        0.0000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"~/", LT2, 0.0000000001, OK, ctx)?;

    testutil::TCASE(b"Get the state of the moon relative to the earth in the mars bodyfixed frame and compare it to the old fashioned way of computing this. ", ctx)?;

    spicelib::SPKGEO(
        301,
        ET,
        b"IAU_MARS",
        399,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    SPKGEO_O(301, ET, b"J2000", 399, STATE1.as_slice_mut(), &mut LT2, ctx)?;
    spicelib::TISBOD(b"J2000", 499, ET, XFORM.as_slice_mut(), ctx)?;
    spicelib::MXVG(
        XFORM.as_slice(),
        STATE1.as_slice(),
        6,
        6,
        STATE2.as_slice_mut(),
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"~/",
        STATE2.as_slice(),
        6,
        0.0000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"~/", LT2, 0.0000000001, OK, ctx)?;

    testutil::TCASE(b"Compute the state of body -9 relative to 401 in the IAU_EARTH frame using SPKGEO and computed via TSTST. ", ctx)?;

    spicelib::SPKGEO(
        -9,
        ET,
        b"IAU_EARTH",
        401,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    spicelib::NAMFRM(b"IAU_EARTH", &mut IAUE, ctx)?;

    testutil::TSTST(
        -9,
        ET,
        &mut SEGID,
        &mut REF1,
        STATE1.as_slice_mut(),
        &mut CENTER,
        &mut GM,
        ctx,
    )?;
    testutil::TSTST(
        301,
        ET,
        &mut SEGID,
        &mut REF2,
        STATE2.as_slice_mut(),
        &mut CENTER,
        &mut GM,
        ctx,
    )?;
    testutil::TSTST(
        399,
        ET,
        &mut SEGID,
        &mut REF3,
        STATE3.as_slice_mut(),
        &mut CENTER,
        &mut GM,
        ctx,
    )?;
    testutil::TSTST(
        3,
        ET,
        &mut SEGID,
        &mut REF4,
        STATE4.as_slice_mut(),
        &mut CENTER,
        &mut GM,
        ctx,
    )?;

    testutil::TSTST(
        401,
        ET,
        &mut SEGID,
        &mut REF5,
        STATE5.as_slice_mut(),
        &mut CENTER,
        &mut GM,
        ctx,
    )?;
    testutil::TSTST(
        499,
        ET,
        &mut SEGID,
        &mut REF6,
        STATE6.as_slice_mut(),
        &mut CENTER,
        &mut GM,
        ctx,
    )?;
    testutil::TSTST(
        4,
        ET,
        &mut SEGID,
        &mut REF7,
        STATE7.as_slice_mut(),
        &mut CENTER,
        &mut GM,
        ctx,
    )?;

    spicelib::FRMCHG(REF1, IAUE, ET, XFORM1.as_slice_mut(), ctx)?;
    spicelib::FRMCHG(REF2, IAUE, ET, XFORM2.as_slice_mut(), ctx)?;
    spicelib::FRMCHG(REF3, IAUE, ET, XFORM3.as_slice_mut(), ctx)?;
    spicelib::FRMCHG(REF4, IAUE, ET, XFORM4.as_slice_mut(), ctx)?;
    spicelib::FRMCHG(REF5, IAUE, ET, XFORM5.as_slice_mut(), ctx)?;
    spicelib::FRMCHG(REF6, IAUE, ET, XFORM6.as_slice_mut(), ctx)?;
    spicelib::FRMCHG(REF7, IAUE, ET, XFORM7.as_slice_mut(), ctx)?;

    spicelib::MXVG(
        XFORM1.as_slice(),
        STATE1.as_slice(),
        6,
        6,
        IAUST1.as_slice_mut(),
    );
    spicelib::MXVG(
        XFORM2.as_slice(),
        STATE2.as_slice(),
        6,
        6,
        IAUST2.as_slice_mut(),
    );
    spicelib::MXVG(
        XFORM3.as_slice(),
        STATE3.as_slice(),
        6,
        6,
        IAUST3.as_slice_mut(),
    );
    spicelib::MXVG(
        XFORM4.as_slice(),
        STATE4.as_slice(),
        6,
        6,
        IAUST4.as_slice_mut(),
    );
    spicelib::MXVG(
        XFORM5.as_slice(),
        STATE5.as_slice(),
        6,
        6,
        IAUST5.as_slice_mut(),
    );
    spicelib::MXVG(
        XFORM6.as_slice(),
        STATE6.as_slice(),
        6,
        6,
        IAUST6.as_slice_mut(),
    );
    spicelib::MXVG(
        XFORM7.as_slice(),
        STATE7.as_slice(),
        6,
        6,
        IAUST7.as_slice_mut(),
    );

    spicelib::CLEARD(6, STATE1.as_slice_mut());
    spicelib::CLEARD(6, STATE2.as_slice_mut());

    spicelib::VADDG(
        STATE1.as_slice(),
        IAUST1.as_slice(),
        6,
        TSTATE.as_slice_mut(),
    );
    spicelib::VADDG(
        TSTATE.as_slice(),
        IAUST2.as_slice(),
        6,
        STATE1.as_slice_mut(),
    );
    spicelib::VADDG(
        STATE1.as_slice(),
        IAUST3.as_slice(),
        6,
        TSTATE.as_slice_mut(),
    );
    spicelib::VADDG(
        TSTATE.as_slice(),
        IAUST4.as_slice(),
        6,
        STATE1.as_slice_mut(),
    );

    spicelib::VADDG(
        STATE2.as_slice(),
        IAUST5.as_slice(),
        6,
        TSTATE.as_slice_mut(),
    );
    spicelib::VADDG(
        TSTATE.as_slice(),
        IAUST6.as_slice(),
        6,
        STATE2.as_slice_mut(),
    );
    spicelib::VADDG(
        STATE2.as_slice(),
        IAUST7.as_slice(),
        6,
        TSTATE.as_slice_mut(),
    );
    spicelib::MOVED(TSTATE.as_slice(), 6, STATE2.as_slice_mut());

    spicelib::VSUBG(
        STATE1.as_slice(),
        STATE2.as_slice(),
        6,
        STATE3.as_slice_mut(),
    );

    LT2 = (spicelib::VNORM(STATE3.as_slice()) / spicelib::CLIGHT());

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"~/",
        STATE3.as_slice(),
        6,
        0.0000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"~/", LT2, 0.0000000001, OK, ctx)?;

    testutil::TCASE(b"Compare states as generated with the old version of SPKGEO with the current version. This case uses a long ephemeris and a large variety of observer-target pairs.", ctx)?;
    //
    // This routine tests determines the states of every object
    // in a pair of SPK files with every other object in every
    // available inertial reference frame.
    //
    // This test is probably overkill, but is intended to demonstrate
    // that under normal circumstances, nothing has changed in the
    // latest upgrade of the SPK system.
    //

    spicelib::TPARSE(b"1 JAN 1995", &mut ET, &mut ERROR, ctx)?;

    for REFID in intrinsics::range(1, 18, 5) {
        spicelib::FRMNAM(REFID, &mut REF, ctx)?;

        for J in intrinsics::range(1, 46, 10) {
            for K in intrinsics::range(46, 1, -3) {
                testutil::TSTMSG(b"#", b"ET: #, Frame: #, Target: #, Observer: #. ", ctx);

                TARG = IDCODE[J];
                OBS = IDCODE[K];

                testutil::TSTMSD(ET, ctx);
                testutil::TSTMSC(&REF, ctx);
                testutil::TSTMSI(TARG, ctx);
                testutil::TSTMSI(OBS, ctx);

                spicelib::SPKGEO(TARG, ET, &REF, OBS, STATE.as_slice_mut(), &mut LT, ctx)?;
                SPKGEO_O(TARG, ET, &REF, OBS, STATE2.as_slice_mut(), &mut LT2, ctx)?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"STATE",
                    STATE.as_slice(),
                    b"~/",
                    STATE2.as_slice(),
                    6,
                    0.0000000001,
                    OK,
                    ctx,
                )?;
                testutil::CHCKSD(b"LT", LT, b"~/", LT2, 0.0000000001, OK, ctx)?;
            }
        }
    }

    testutil::TSTMSG(b"#", b" ", ctx);

    testutil::TCASE(b"Check to make sure that the logic that handles long chains of target center pairs functions as expected. ", ctx)?;

    spicelib::SPKGEO(
        401001,
        ET,
        b"IAU_EARTH",
        301001,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    SPKGEO_S(
        401001,
        ET,
        b"IAU_EARTH",
        301001,
        STATE2.as_slice_mut(),
        &mut LT2,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"~/",
        STATE2.as_slice(),
        6,
        0.0000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"~/", LT2, 0.0000000001, OK, ctx)?;

    testutil::TCASE(b"Check to make sure that SPKSSB returns the same state as SPKGEO when the observer in SPKGEO is set to be the solar system barycenter ", ctx)?;

    spicelib::SPKGEO(401, ET, b"J2000", 0, STATE2.as_slice_mut(), &mut LT, ctx)?;
    spicelib::SPKSSB(401, ET, b"J2000", STATE.as_slice_mut(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"=",
        STATE2.as_slice(),
        6,
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(
        b"Make sure we can get the state of an object relative to a topocentric frame. ",
        ctx,
    )?;

    testutil::KILFIL(b"topo.txt", ctx)?;
    testutil::TSTTXT(b"topo.txt", LINES.as_arg(), NLINES, true, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKGEO(
        401001,
        ET,
        b"IAU_EARTH",
        301001,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    spicelib::SXFORM(
        b"IAU_EARTH",
        b"GOLDSTONE_TOPO",
        ET,
        STXFRM.as_slice_mut(),
        ctx,
    )?;
    spicelib::MXVG(
        STXFRM.as_slice(),
        STATE.as_slice(),
        6,
        6,
        STATE2.as_slice_mut(),
    );

    spicelib::SPKGEO(
        401001,
        ET,
        b"GOLDSTONE_TOPO",
        301001,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"~/",
        STATE2.as_slice(),
        6,
        0.0000000001,
        OK,
        ctx,
    )?;
    //
    // That's all folks.
    //
    spicelib::SPKUEF(SPKHAN, ctx)?;
    spicelib::CKUPF(CKHAN, ctx)?;

    testutil::KILFIL(b"phoenix.bc", ctx)?;
    testutil::KILFIL(b"test_spk.bsp", ctx)?;
    testutil::KILFIL(b"topo.txt", ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
