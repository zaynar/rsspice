//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;
const NLINES: i32 = 83;
const DEFSIZ: i32 = 15;

//$Procedure      F_FRMCHG ( Family of tests for FRMCHG )
pub fn F_FRMCHG(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut DEFTXT = ActualCharArray::new(LNSIZE, 1..=DEFSIZ);
    let mut ERROR = [b' '; LNSIZE as usize];
    let mut LINES = ActualCharArray::new(LNSIZE, 1..=NLINES);
    let mut AV = StackArray::<f64, 3>::new(1..=3);
    let mut ET: f64 = 0.0;
    let mut ROT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut ROTT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut TSIPM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut TSPMI = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut TXFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut MODIFY = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut EXPECT = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut TMPXFM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut TK2CK = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut TK2IN = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut TK2J2 = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut CK2IN = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut EARTH: i32 = 0;
    let mut CENT: i32 = 0;
    let mut CLASS: i32 = 0;
    let mut CLSSID: i32 = 0;
    let mut FRAME: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut ID: i32 = 0;
    let mut J: i32 = 0;
    let mut NTEXT: i32 = 0;
    let mut XPCTD = StackArray::<i32, 3>::new(1..=3);
    let mut XCENT: i32 = 0;
    let mut XCLASS: i32 = 0;
    let mut XCLSID: i32 = 0;
    let mut FOUND: bool = false;

    //
    // Test Utility Functions
    //

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_FRMCHG", ctx)?;

    fstr::assign(
        LINES.get_mut(1),
        b"This is a test frame kernel for the fictional instrument TST_PHEONIX",
    );
    fstr::assign(
        LINES.get_mut(2),
        b"on board the fictional spacecraft PHEONIX. A C-kernel for",
    );
    fstr::assign(
        LINES.get_mut(3),
        b"the platform on which TST_PHEONIX is mounted can be generated",
    );
    fstr::assign(LINES.get_mut(4), b"by calling the test utility TSTCK3.");
    fstr::assign(LINES.get_mut(5), b" ");
    fstr::assign(
        LINES.get_mut(6),
        b"This kernel describes only the orientation attributes of the",
    );
    fstr::assign(LINES.get_mut(7), b"TST_PHOENIX instrument.");
    fstr::assign(LINES.get_mut(8), b" ");
    fstr::assign(
        LINES.get_mut(9),
        b"This kernel is intended only for test purposes.  It is primarily",
    );
    fstr::assign(LINES.get_mut(10), b"useful for testing frame routines");
    fstr::assign(LINES.get_mut(11), b" ");
    fstr::assign(LINES.get_mut(12), b" ");
    fstr::assign(
        LINES.get_mut(13),
        b"Next we need to supply the various bits of frame identification for",
    );
    fstr::assign(LINES.get_mut(14), b"this instrument.");
    fstr::assign(LINES.get_mut(15), b" ");
    testutil::BEGDAT(&mut LINES[16]);
    fstr::assign(LINES.get_mut(17), b"FRAME_-399999_CLASS    =  4");
    fstr::assign(LINES.get_mut(18), b"FRAME_-399999_CENTER   =  399");
    fstr::assign(LINES.get_mut(19), b"FRAME_-399999_CLASS_ID = -399999");
    fstr::assign(
        LINES.get_mut(20),
        b"FRAME_-399999_NAME     = \'TOPOCENTRIC\'",
    );
    fstr::assign(LINES.get_mut(21), b"FRAME_TOPOCENTRIC      = -399999");
    fstr::assign(LINES.get_mut(22), b" ");
    fstr::assign(LINES.get_mut(23), b"FRAME_-111111_CLASS    =  4");
    fstr::assign(LINES.get_mut(24), b"FRAME_-111111_CENTER   = -9");
    fstr::assign(LINES.get_mut(25), b"FRAME_-111111_CLASS_ID = -111111");
    fstr::assign(
        LINES.get_mut(26),
        b"FRAME_-111111_NAME     = \'TST-PHOENIX\'",
    );
    fstr::assign(LINES.get_mut(27), b"FRAME_TST-PHOENIX      = -111111");
    fstr::assign(LINES.get_mut(28), b" ");
    fstr::assign(LINES.get_mut(29), b"FRAME_-9999_CLASS      =  3");
    fstr::assign(LINES.get_mut(30), b"FRAME_-9999_CENTER     = -9");
    fstr::assign(LINES.get_mut(31), b"FRAME_-9999_CLASS_ID   = -9999");
    fstr::assign(LINES.get_mut(32), b"FRAME_-9999_NAME       = \'PHOENIX\'");
    fstr::assign(LINES.get_mut(33), b"FRAME_PHOENIX          = -9999");
    fstr::assign(LINES.get_mut(34), b" ");
    fstr::assign(LINES.get_mut(35), b"CK_-9999_SCLK          =  -9");
    fstr::assign(LINES.get_mut(36), b"CK_-9999_SPK           =  -9");
    fstr::assign(LINES.get_mut(37), b" ");
    fstr::assign(LINES.get_mut(38), b"TKFRAME_-111111_SPEC      = \'MATRIX\'");
    fstr::assign(
        LINES.get_mut(39),
        b"TKFRAME_-111111_RELATIVE  = \'PHOENIX\'",
    );
    fstr::assign(LINES.get_mut(40), b"TKFRAME_-111111_MATRIX    =  ( 0.48");
    fstr::assign(LINES.get_mut(41), b"                               0.60");
    fstr::assign(LINES.get_mut(42), b"                               0.64");
    fstr::assign(LINES.get_mut(43), b"                              -0.8");
    fstr::assign(LINES.get_mut(44), b"                               0.0");
    fstr::assign(LINES.get_mut(45), b"                               0.6");
    fstr::assign(LINES.get_mut(46), b"                               0.36");
    fstr::assign(LINES.get_mut(47), b"                              -0.80");
    fstr::assign(LINES.get_mut(48), b"                               0.48 )");
    fstr::assign(LINES.get_mut(49), b" ");
    fstr::assign(
        LINES.get_mut(50),
        b"TKFRAME_-399999_SPEC       = \'ANGLES\'",
    );
    fstr::assign(
        LINES.get_mut(51),
        b"TKFRAME_-399999_RELATIVE   = \'IAU_EARTH\'",
    );
    fstr::assign(
        LINES.get_mut(52),
        b"TKFRAME_-399999_ANGLES     = ( 90, 56.1829, -118.0 )",
    );
    fstr::assign(
        LINES.get_mut(53),
        b"TKFRAME_-399999_AXES       = ( 3, 2, 3 )",
    );
    fstr::assign(
        LINES.get_mut(54),
        b"TKFRAME_-399999_UNITS      = \'DEGREES\'",
    );
    fstr::assign(LINES.get_mut(55), b" ");
    testutil::BEGTXT(&mut LINES[56]);
    fstr::assign(LINES.get_mut(57), b" ");
    fstr::assign(LINES.get_mut(58), b"  ");
    fstr::assign(LINES.get_mut(59), b"  ");
    fstr::assign(LINES.get_mut(60), b" ");
    fstr::assign(LINES.get_mut(61), b" ");
    fstr::assign(LINES.get_mut(62), b" ");
    testutil::BEGDAT(&mut LINES[63]);
    fstr::assign(LINES.get_mut(64), b" ");
    fstr::assign(LINES.get_mut(65), b"FRAME_-222222_CLASS    =  4");
    fstr::assign(LINES.get_mut(66), b"FRAME_-222222_CENTER   = -9");
    fstr::assign(LINES.get_mut(67), b"FRAME_-222222_CLASS_ID = -222222");
    fstr::assign(
        LINES.get_mut(68),
        b"FRAME_-222222_NAME     = \'TST_PHOENIX2\'",
    );
    fstr::assign(LINES.get_mut(69), b"FRAME_TST_PHOENIX2     = -222222");
    fstr::assign(LINES.get_mut(70), b" ");
    fstr::assign(LINES.get_mut(71), b" ");
    fstr::assign(
        LINES.get_mut(72),
        b"TKFRAME_-222222_RELATIVE   =  \'PHOENIX\'",
    );
    fstr::assign(
        LINES.get_mut(73),
        b"TKFRAME_-222222_SPEC       =  \'MATRIX\'",
    );
    fstr::assign(LINES.get_mut(74), b"TKFRAME_-222222_MATRIX     = ( 0.48");
    fstr::assign(LINES.get_mut(75), b"                               0.60");
    fstr::assign(LINES.get_mut(76), b"                               0.64");
    fstr::assign(LINES.get_mut(77), b"                              -0.8");
    fstr::assign(LINES.get_mut(78), b"                               0.0");
    fstr::assign(LINES.get_mut(79), b"                               0.6");
    fstr::assign(LINES.get_mut(80), b"                               0.36");
    fstr::assign(LINES.get_mut(81), b"                              -0.80");
    fstr::assign(LINES.get_mut(82), b"                               0.48 )");
    fstr::assign(LINES.get_mut(83), b" ");

    //
    // Clean out the kernel pool. Just in case we have something
    // already loaded there.
    //
    spicelib::CLPOOL(ctx)?;
    //
    // Create and load the test kernels that will be needed during
    // this test.
    //
    testutil::TSTTXT(b"phoenix2.tk", LINES.as_arg(), NLINES, true, true, ctx)?;
    testutil::TSTPCK(b"test.pck", true, false, ctx)?;
    testutil::TSTCK3(
        b"phoenix.bc",
        b"phoenix.tsc",
        false,
        true,
        false,
        &mut HANDLE,
        ctx,
    )?;

    spicelib::CKLPF(b"phoenix.bc", &mut HANDLE, ctx)?;
    spicelib::TPARSE(b"1 Jan 2195", &mut ET, &mut ERROR, ctx)?;

    testutil::TCASE(b"Exercise the exception handling for the case in which information is not provided that is sufficient to transform from one frame to another. ", ctx)?;

    spicelib::FRMCHG(-222222, 1, ET, XFORM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(NOFRAMECONNECT)", OK, ctx)?;

    testutil::TCASE(
        b"Verify that if a frame is not recognized, that an error is signalled that says so. ",
        ctx,
    )?;

    spicelib::TPARSE(b"1 Jan 1995", &mut ET, &mut ERROR, ctx)?;
    spicelib::FRMCHG(27687628, -1987291, ET, XFORM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(UNKNOWNFRAME)", OK, ctx)?;

    testutil::TCASE(b"Verify that all inertial frames are recognized and that they produce the expected transformation. ", ctx)?;

    for I in 1..=18 {
        {
            let m1__: i32 = 1;
            let m2__: i32 = 18;
            let m3__: i32 = 1;
            J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                spicelib::FRMCHG(I, J, ET, XFORM.as_slice_mut(), ctx)?;
                spicelib::IRFROT(I, J, ROT.as_slice_mut(), ctx)?;

                for K in 4..=6 {
                    for M in 1..=3 {
                        TXFORM[[K, M]] = 0.0;
                        TXFORM[[M, K]] = 0.0;
                    }
                }

                for K in 1..=3 {
                    for M in 1..=3 {
                        TXFORM[[K, M]] = ROT[[K, M]];
                        TXFORM[[(K + 3), (M + 3)]] = ROT[[K, M]];
                    }
                }

                testutil::TSTMSG(b"#", b"Subcase. Frame 1 = #, Frame 2 = #.", ctx);
                testutil::TSTMSI(I, ctx);
                testutil::TSTMSI(J, ctx);

                testutil::CHCKXC(false, b" ", OK, ctx)?;
                testutil::CHCKAD(
                    b"XFORM",
                    XFORM.as_slice(),
                    b"~",
                    TXFORM.as_slice(),
                    36,
                    0.0000000000001,
                    OK,
                    ctx,
                )?;
                J += m3__;
            }
        }
    }
    //
    // Clear the extra message.
    //
    testutil::TSTMSG(b"#", b" ", ctx);

    testutil::TCASE(
        b"Verify that the transformations from inertial to bodyfixed frames behave as expected. ",
        ctx,
    )?;

    for I in 10010..=10027 {
        J = I;
        testutil::TSTMSG(b"#", b"Subcase: Frame ID = # ", ctx);
        testutil::TSTMSI(J, ctx);

        spicelib::FRINFO(J, &mut CENT, &mut CLASS, &mut CLSSID, &mut FOUND, ctx)?;
        spicelib::TISBOD(b"J2000", CLSSID, ET, TSIPM.as_slice_mut(), ctx)?;
        spicelib::FRMCHG(1, J, ET, XFORM.as_slice_mut(), ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKAD(
            b"XFORM",
            XFORM.as_slice(),
            b"=",
            TSIPM.as_slice(),
            36,
            0.0,
            OK,
            ctx,
        )?;
    }

    testutil::TSTMSG(b"#", b" ", ctx);

    testutil::TCASE(
        b"Verify that the transformations from bodyfixed to inertial frames behave as expected. ",
        ctx,
    )?;

    for I in 10010..=10027 {
        J = I;
        testutil::TSTMSG(b"#", b"Subcase: Frame ID = # ", ctx);
        testutil::TSTMSI(J, ctx);

        spicelib::FRINFO(J, &mut CENT, &mut CLASS, &mut CLSSID, &mut FOUND, ctx)?;
        spicelib::TISBOD(b"J2000", CLSSID, ET, TSIPM.as_slice_mut(), ctx)?;
        spicelib::INVSTM(TSIPM.as_slice(), TSPMI.as_slice_mut(), ctx)?;
        spicelib::FRMCHG(J, 1, ET, XFORM.as_slice_mut(), ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKAD(
            b"XFORM",
            XFORM.as_slice(),
            b"=",
            TSPMI.as_slice(),
            36,
            0.0,
            OK,
            ctx,
        )?;
    }

    testutil::TSTMSG(b"#", b" ", ctx);

    testutil::TCASE(b"Verify that C-kernel frames to inertial frame transformations are carried out correctly. ", ctx)?;
    //
    // First construct the transformation from the C-kernel
    // frame to its native frame.  And look up the native
    // frame for each of the segments in the C-kernel.
    //
    testutil::TSTATD(ET, ROT.as_slice_mut(), AV.as_slice_mut(), ctx);
    spicelib::RAV2XF(ROT.as_slice(), AV.as_slice(), TMPXFM.as_slice_mut());
    spicelib::INVSTM(TMPXFM.as_slice(), TXFORM.as_slice_mut(), ctx)?;

    spicelib::NAMFRM(b"J2000", &mut XPCTD[1], ctx)?;
    spicelib::NAMFRM(b"FK4", &mut XPCTD[2], ctx)?;
    spicelib::NAMFRM(b"GALACTIC", &mut XPCTD[3], ctx)?;

    for I in 1..=3 {
        ID = (-10002 + I);

        spicelib::FRINFO(
            XPCTD[I],
            &mut XCENT,
            &mut XCLASS,
            &mut XCLSID,
            &mut FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(b"Expected frame info", FOUND, true, OK, ctx)?;

        //
        // Define a CK frame associated with instrument having ID code
        // "ID"; the pointing for this instrument is given by the CK
        // created by the test utility TSTCK3.
        //
        fstr::assign(
            DEFTXT.get_mut(1),
            b"FRAME_CK_INSTID                  =  INSTID",
        );
        fstr::assign(
            DEFTXT.get_mut(2),
            b"FRAME_INSTID_NAME                = \'CK_INSTID\'",
        );
        fstr::assign(DEFTXT.get_mut(3), b"FRAME_INSTID_CLASS               =  3");
        fstr::assign(
            DEFTXT.get_mut(4),
            b"FRAME_INSTID_CLASS_ID            =  INSTID",
        );
        fstr::assign(
            DEFTXT.get_mut(5),
            b"FRAME_INSTID_CENTER              =  XCENT",
        );
        fstr::assign(
            DEFTXT.get_mut(6),
            b"FRAME_INSTID_RELATIVE            =  XPCT",
        );

        NTEXT = 6;

        {
            let m1__: i32 = 1;
            let m2__: i32 = NTEXT;
            let m3__: i32 = 1;
            J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                //
                // Do two replacements for the ID code; if the second
                // replacement is unnecessary, no harm's done.
                //
                spicelib::REPMI(&DEFTXT[J].to_vec(), b"INSTID", ID, &mut DEFTXT[J], ctx);
                spicelib::REPMI(&DEFTXT[J].to_vec(), b"INSTID", ID, &mut DEFTXT[J], ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                //
                // Replace the center ID when it comes up.
                //
                spicelib::REPMI(&DEFTXT[J].to_vec(), b"XCENT", XCENT, &mut DEFTXT[J], ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                J += m3__;
            }
        }
        //
        // Insert the base frame ID.
        //
        spicelib::REPMI(
            &DEFTXT[NTEXT].to_vec(),
            b"XPCT",
            XPCTD[I],
            &mut DEFTXT[NTEXT],
            ctx,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Load the CK_-10001 frame definition.
        //
        spicelib::LMPOOL(DEFTXT.as_arg(), 6, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::TSTMSG(b"#", b"Subcase: Frame ID = # ", ctx);
        testutil::TSTMSI(ID, ctx);
        spicelib::FRMCHG(ID, 1, ET, XFORM.as_slice_mut(), ctx)?;
        //
        // Construct the expected transfromation from the C-kernel
        // native frame to J2000. And then construct the transformation
        // from the C-kernel frame to J2000.
        //
        spicelib::FRMCHG(XPCTD[I], 1, ET, MODIFY.as_slice_mut(), ctx)?;
        spicelib::MXMG(
            MODIFY.as_slice(),
            TXFORM.as_slice(),
            6,
            6,
            6,
            EXPECT.as_slice_mut(),
        );

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKAD(
            b"XFORM",
            XFORM.as_slice(),
            b"~",
            EXPECT.as_slice(),
            36,
            0.000000000001,
            OK,
            ctx,
        )?;
    }

    testutil::TSTMSG(b"#", b" ", ctx);

    testutil::TCASE(b"Verify that we can perform a complete transformation from an instrument frame to a body fixed frame. TK -> CK ->INERTIAL ->PCK ", ctx)?;

    spicelib::NAMFRM(b"IAU_EARTH", &mut EARTH, ctx)?;
    spicelib::FRMCHG(-111111, EARTH, ET, XFORM.as_slice_mut(), ctx)?;

    //
    // Construct the expected output state transformation matrix.
    //
    for I in 1..=6 {
        {
            let m1__: i32 = 1;
            let m2__: i32 = 6;
            let m3__: i32 = 1;
            J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                TK2CK[[I, J]] = 0.0;
                J += m3__;
            }
        }
    }

    TK2CK[[1, 1]] = 0.48;
    TK2CK[[2, 1]] = 0.6;
    TK2CK[[3, 1]] = 0.64;

    TK2CK[[1, 2]] = -0.8;
    TK2CK[[2, 2]] = 0.0;
    TK2CK[[3, 2]] = 0.6;

    TK2CK[[1, 3]] = 0.36;
    TK2CK[[2, 3]] = -0.8;
    TK2CK[[3, 3]] = 0.48;

    TK2CK[[4, 4]] = 0.48;
    TK2CK[[5, 4]] = 0.6;
    TK2CK[[6, 4]] = 0.64;

    TK2CK[[4, 5]] = -0.8;
    TK2CK[[5, 5]] = 0.0;
    TK2CK[[6, 5]] = 0.6;

    TK2CK[[4, 6]] = 0.36;
    TK2CK[[5, 6]] = -0.8;
    TK2CK[[6, 6]] = 0.48;

    spicelib::FRMCHG(XPCTD[3], 1, ET, MODIFY.as_slice_mut(), ctx)?;

    //
    // TSTATD can reproduce the attitude in the C-kernel so we
    // use that instead of CKGPAV.
    //
    testutil::TSTATD(ET, ROT.as_slice_mut(), AV.as_slice_mut(), ctx);
    spicelib::RAV2XF(ROT.as_slice(), AV.as_slice(), TMPXFM.as_slice_mut());
    spicelib::INVSTM(TMPXFM.as_slice(), CK2IN.as_slice_mut(), ctx)?;

    spicelib::TISBOD(b"J2000", 399, ET, TSIPM.as_slice_mut(), ctx)?;

    spicelib::MXMG(
        CK2IN.as_slice(),
        TK2CK.as_slice(),
        6,
        6,
        6,
        TK2IN.as_slice_mut(),
    );
    spicelib::MXMG(
        MODIFY.as_slice(),
        TK2IN.as_slice(),
        6,
        6,
        6,
        TK2J2.as_slice_mut(),
    );
    spicelib::MXMG(
        TSIPM.as_slice(),
        TK2J2.as_slice(),
        6,
        6,
        6,
        TXFORM.as_slice_mut(),
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"XFORM",
        XFORM.as_slice(),
        b"~",
        TXFORM.as_slice(),
        36,
        0.0000000000001,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Perform test a long chain of transformations on both ends. TK -> CK -> INERTIAL -> PCK -> TK ", ctx)?;

    //
    // Get the transformation from frame -399999 to the
    // underlying bodyfixed frame.
    //
    spicelib::TKFRAM(-399999, ROT.as_slice_mut(), &mut FRAME, &mut FOUND, ctx)?;
    spicelib::XPOSE(ROT.as_slice(), ROTT.as_slice_mut());

    for I in 1..=3 {
        {
            let m1__: i32 = 1;
            let m2__: i32 = 3;
            let m3__: i32 = 1;
            J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                MODIFY[[I, J]] = ROTT[[I, J]];
                MODIFY[[(I + 3), J]] = 0.0;
                MODIFY[[I, (J + 3)]] = 0.0;
                MODIFY[[(I + 3), (J + 3)]] = ROTT[[I, J]];
                J += m3__;
            }
        }
    }

    spicelib::MXMG(
        MODIFY.as_slice(),
        TXFORM.as_slice(),
        6,
        6,
        6,
        EXPECT.as_slice_mut(),
    );

    spicelib::FRMCHG(-111111, -399999, ET, XFORM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"FRAME", FRAME, b"=", EARTH, 0, OK, ctx)?;

    testutil::CHCKAD(
        b"XFORM",
        XFORM.as_slice(),
        b"~",
        EXPECT.as_slice(),
        36,
        0.0000000000001,
        OK,
        ctx,
    )?;

    //
    // Finally clean up the loaded data.
    //
    spicelib::CKUPF(HANDLE, ctx)?;
    testutil::KILFIL(b"phoenix.bc", ctx)?;
    testutil::KILFIL(b"phoenix2.tk", ctx)?;
    spicelib::CLPOOL(ctx)?;
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
