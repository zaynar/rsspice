//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;
const NLINES: i32 = 66;
const DEFSIZ: i32 = 15;

//$Procedure      F_FRMGET ( Family of tests for FRMGET )
pub fn F_FRMGET(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut DEFTXT = ActualCharArray::new(LNSIZE, 1..=DEFSIZ);
    let mut ERROR = [b' '; LNSIZE as usize];
    let mut LINES = ActualCharArray::new(LNSIZE, 1..=NLINES);
    let mut AV = StackArray::<f64, 3>::new(1..=3);
    let mut ET: f64 = 0.0;
    let mut ROT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut TSIPM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut TSPMI = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut TMPXFM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut TXFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut CENT: i32 = 0;
    let mut CLASS: i32 = 0;
    let mut CLSSID: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut ID: i32 = 0;
    let mut J: i32 = 0;
    let mut NTEXT: i32 = 0;
    let mut OUTFRM: i32 = 0;
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
    testutil::TOPEN(b"F_FRMGET", ctx)?;

    fstr::assign(
        LINES.get_mut(1),
        b"This is a test I-kernel for the fictional instrument TST_PHEONIX ",
    );
    fstr::assign(
        LINES.get_mut(2),
        b"on board the fictional spacecraft PHEONIX. A C-kernel for ",
    );
    fstr::assign(
        LINES.get_mut(3),
        b"the platform on which TST_PHEONIX is mounted can be generated ",
    );
    fstr::assign(LINES.get_mut(4), b"by calling the test utility TSTCK3. ");
    fstr::assign(LINES.get_mut(5), b" ");
    fstr::assign(
        LINES.get_mut(6),
        b"This kernel describes only the mode independent attributes of the ",
    );
    fstr::assign(LINES.get_mut(7), b"TST_PHOENIX instrument. ");
    fstr::assign(LINES.get_mut(8), b" ");
    fstr::assign(
        LINES.get_mut(9),
        b"This kernel is intended only for test purposes.  It is primarily ",
    );
    fstr::assign(
        LINES.get_mut(10),
        b"useful for testing the I-kernel data fetching routines ",
    );
    fstr::assign(LINES.get_mut(11), b" ");
    testutil::BEGDAT(&mut LINES[12]);
    fstr::assign(LINES.get_mut(13), b"TKFRAME_-111111_SPEC = \'MATRIX\' ");
    fstr::assign(LINES.get_mut(14), b" ");
    fstr::assign(LINES.get_mut(15), b" ");
    fstr::assign(LINES.get_mut(16), b"  ");
    fstr::assign(LINES.get_mut(17), b" ");
    fstr::assign(LINES.get_mut(18), b" ");
    fstr::assign(LINES.get_mut(19), b" ");
    fstr::assign(LINES.get_mut(20), b" ");
    fstr::assign(LINES.get_mut(21), b" ");
    fstr::assign(
        LINES.get_mut(22),
        b"TKFRAME_-111111_RELATIVE     = \'PHOENIX\' ",
    );
    fstr::assign(LINES.get_mut(23), b" ");
    fstr::assign(LINES.get_mut(24), b" ");
    fstr::assign(LINES.get_mut(25), b" ");
    fstr::assign(LINES.get_mut(26), b" ");
    fstr::assign(LINES.get_mut(27), b" ");
    fstr::assign(LINES.get_mut(28), b" ");
    fstr::assign(LINES.get_mut(29), b"TKFRAME_-111111_MATRIX    = ( 0.48 ");
    fstr::assign(LINES.get_mut(30), b"0.60 ");
    fstr::assign(LINES.get_mut(31), b"0.64 ");
    fstr::assign(LINES.get_mut(32), b"-0.8 ");
    fstr::assign(LINES.get_mut(33), b"0.0 ");
    fstr::assign(LINES.get_mut(34), b"0.6 ");
    fstr::assign(LINES.get_mut(35), b"0.36 ");
    fstr::assign(LINES.get_mut(36), b"-0.80 ");
    fstr::assign(LINES.get_mut(37), b"0.48 ) ");
    fstr::assign(LINES.get_mut(38), b" ");
    fstr::assign(LINES.get_mut(39), b" ");
    fstr::assign(LINES.get_mut(40), b" ");
    testutil::BEGTXT(&mut LINES[41]);
    fstr::assign(LINES.get_mut(42), b" ");
    fstr::assign(
        LINES.get_mut(43),
        b"Next we need to supply the various bits of frame identification for ",
    );
    fstr::assign(LINES.get_mut(44), b"this instrument. ");
    fstr::assign(LINES.get_mut(45), b" ");
    testutil::BEGDAT(&mut LINES[46]);
    fstr::assign(LINES.get_mut(47), b"FRAME_-222222_CLASS    =  7 ");
    fstr::assign(LINES.get_mut(48), b"FRAME_-222222_CENTER   = -9 ");
    fstr::assign(LINES.get_mut(49), b"FRAME_-222222_CLASS_ID = -1 ");
    fstr::assign(
        LINES.get_mut(50),
        b"FRAME_-222222_NAME     = \'UNSUPPORTED\' ",
    );
    fstr::assign(LINES.get_mut(51), b"FRAME_UNSUPPORTED      = -222222 ");
    fstr::assign(LINES.get_mut(52), b" ");
    fstr::assign(LINES.get_mut(53), b"FRAME_-111111_CLASS    =  4 ");
    fstr::assign(LINES.get_mut(54), b"FRAME_-111111_CENTER   = -9 ");
    fstr::assign(LINES.get_mut(55), b"FRAME_-111111_CLASS_ID = -111111 ");
    fstr::assign(
        LINES.get_mut(56),
        b"FRAME_-111111_NAME     = \'TST-PHOENIX\' ",
    );
    fstr::assign(LINES.get_mut(57), b"FRAME_TST-PHOENIX      = -111111 ");
    fstr::assign(LINES.get_mut(58), b" ");
    fstr::assign(LINES.get_mut(59), b"FRAME_-9999_CLASS      =  3 ");
    fstr::assign(LINES.get_mut(60), b"FRAME_-9999_CENTER     = -9 ");
    fstr::assign(LINES.get_mut(61), b"FRAME_-9999_CLASS_ID   = -9999 ");
    fstr::assign(LINES.get_mut(62), b"FRAME_-9999_NAME       = \'PHOENIX\' ");
    fstr::assign(LINES.get_mut(63), b"FRAME_PHOENIX          = -9999 ");
    fstr::assign(LINES.get_mut(64), b" ");
    fstr::assign(LINES.get_mut(65), b"CK_-9999_SCLK          =  -9 ");
    fstr::assign(LINES.get_mut(66), b"CK_-9999_SPK           =  -9 ");

    //
    // Clean out the kernel pool. Just in case we have something
    // already loaded there.
    //
    spicelib::CLPOOL(ctx)?;
    //
    // Create and load the test kernels that will be needed during
    // this test.
    //
    testutil::TCASE(b"Make sure we can load everything.", ctx)?;

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
    testutil::TSTTXT(b"phoenix2.ik", LINES.as_arg(), NLINES, true, false, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CKLPF(b"phoenix.bc", &mut HANDLE, ctx)?;
    spicelib::TPARSE(b"1 Jan 1995", &mut ET, &mut ERROR, ctx)?;

    testutil::TCASE(b"Determine that a frame of unrecognized class causes the correct exception to be signalled. ", ctx)?;

    spicelib::FRMGET(
        -222222,
        ET,
        XFORM.as_slice_mut(),
        &mut OUTFRM,
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(UNKNOWNFRAMETYPE)", OK, ctx)?;

    testutil::TCASE(b"Verify that all inertial frames are recognized and that they produce the expected transformation. ", ctx)?;

    for I in 1..=18 {
        spicelib::FRMGET(I, ET, XFORM.as_slice_mut(), &mut OUTFRM, &mut FOUND, ctx)?;
        spicelib::IRFROT(I, 1, ROT.as_slice_mut(), ctx)?;

        for K in 4..=6 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = 3;
                let m3__: i32 = 1;
                J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    TXFORM[[K, J]] = 0.0;
                    TXFORM[[J, K]] = 0.0;
                    J += m3__;
                }
            }
        }

        for K in 1..=3 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = 3;
                let m3__: i32 = 1;
                J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    TXFORM[[K, J]] = ROT[[K, J]];
                    TXFORM[[(K + 3), (J + 3)]] = ROT[[K, J]];
                    J += m3__;
                }
            }
        }

        J = I;
        testutil::TSTMSG(b"#", b"Subcase. Frame ID = # ", ctx);
        testutil::TSTMSI(J, ctx);

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
        testutil::CHCKSI(b"OUTFRM", OUTFRM, b"=", 1, 0, OK, ctx)?;
        testutil::CHCKAD(
            b"XFORM",
            XFORM.as_slice(),
            b"=",
            TXFORM.as_slice(),
            36,
            0.0,
            OK,
            ctx,
        )?;
    }
    //
    // Clear the extra message.
    //
    testutil::TSTMSG(b"#", b" ", ctx);

    testutil::TCASE(
        b"Verify that inertial class frames that are not yet supported are not found by FRMGET. ",
        ctx,
    )?;

    spicelib::FRMGET(99, ET, XFORM.as_slice_mut(), &mut OUTFRM, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    testutil::TCASE(b"Verify that the planetary frames are recognized. ", ctx)?;

    for I in 10010..=10027 {
        J = I;
        testutil::TSTMSG(b"#", b"Subcase: Frame ID = # ", ctx);
        testutil::TSTMSI(J, ctx);

        spicelib::FRINFO(J, &mut CENT, &mut CLASS, &mut CLSSID, &mut FOUND, ctx)?;
        spicelib::TISBOD(b"J2000", CLSSID, ET, TSIPM.as_slice_mut(), ctx)?;
        spicelib::INVSTM(TSIPM.as_slice(), TSPMI.as_slice_mut(), ctx)?;
        spicelib::FRMGET(I, ET, XFORM.as_slice_mut(), &mut OUTFRM, &mut FOUND, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
        testutil::CHCKSI(b"OUTFRM", OUTFRM, b"=", 1, 0, OK, ctx)?;
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

    testutil::TCASE(b"Verify that loaded C-kernels are recognized.", ctx)?;

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
        spicelib::FRMGET(ID, ET, XFORM.as_slice_mut(), &mut OUTFRM, &mut FOUND, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
        testutil::CHCKSI(b"OUTFRM", OUTFRM, b"=", XPCTD[I], 0, OK, ctx)?;
        testutil::CHCKAD(
            b"XFORM",
            XFORM.as_slice(),
            b"~",
            TXFORM.as_slice(),
            36,
            0.000000000001,
            OK,
            ctx,
        )?;
    }

    testutil::TCASE(b"Verify that I-kernel frames are recognized. ", ctx)?;

    spicelib::FRMGET(
        -111111,
        ET,
        XFORM.as_slice_mut(),
        &mut OUTFRM,
        &mut FOUND,
        ctx,
    )?;
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
                TXFORM[[I, J]] = 0.0;
                J += m3__;
            }
        }
    }

    TXFORM[[1, 1]] = 0.48;
    TXFORM[[2, 1]] = 0.6;
    TXFORM[[3, 1]] = 0.64;

    TXFORM[[1, 2]] = -0.8;
    TXFORM[[2, 2]] = 0.0;
    TXFORM[[3, 2]] = 0.6;

    TXFORM[[1, 3]] = 0.36;
    TXFORM[[2, 3]] = -0.8;
    TXFORM[[3, 3]] = 0.48;

    TXFORM[[4, 4]] = 0.48;
    TXFORM[[5, 4]] = 0.6;
    TXFORM[[6, 4]] = 0.64;

    TXFORM[[4, 5]] = -0.8;
    TXFORM[[5, 5]] = 0.0;
    TXFORM[[6, 5]] = 0.6;

    TXFORM[[4, 6]] = 0.36;
    TXFORM[[5, 6]] = -0.8;
    TXFORM[[6, 6]] = 0.48;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"OUTFRM", OUTFRM, b"=", -9999, 0, OK, ctx)?;
    testutil::CHCKAD(
        b"XFORM",
        XFORM.as_slice(),
        b"~",
        TXFORM.as_slice(),
        36,
        0.00000000000001,
        OK,
        ctx,
    )?;

    //
    // Finally clean up the loaded data.
    //
    spicelib::CKUPF(HANDLE, ctx)?;
    spicelib::CLPOOL(ctx)?;
    testutil::KILFIL(b"phoenix.bc", ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
