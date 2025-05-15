//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NLINES: i32 = 56;
const LNSIZE: i32 = 80;
const WDSIZE: i32 = 32;
const NBODS: i32 = 24;

//$Procedure      F_PXFORM ( Family of tests for PXFORM )
pub fn F_PXFORM(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut LINES = ActualCharArray::new(LNSIZE, 1..=NLINES);
    let mut NAME = ActualCharArray::new(WDSIZE, 1..=NBODS);
    let mut NAMEI = [b' '; WDSIZE as usize];
    let mut NAMEJ = [b' '; WDSIZE as usize];
    let mut EFORM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut ET: f64 = 0.0;
    let mut ROT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut TMPMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut TSIPM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut TSPMI = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut XFORM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut HANDLE: i32 = 0;
    let mut IDCODE = StackArray::<i32, 24>::new(1..=NBODS);
    let mut IDI: i32 = 0;
    let mut IDJ: i32 = 0;
    let mut PHNX: i32 = 0;
    let mut TOPO: i32 = 0;

    //
    // Test Utility Functions
    //

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    fstr::assign(LINES.get_mut(1), b" ");
    fstr::assign(
        LINES.get_mut(2),
        b"This is a test kernel for the fictional instrument TST_PHEONIX",
    );
    fstr::assign(
        LINES.get_mut(3),
        b"on board the fictional spacecraft PHEONIX. A C-kernel for",
    );
    fstr::assign(
        LINES.get_mut(4),
        b"the platform on which TST_PHEONIX is mounted can be generated",
    );
    fstr::assign(LINES.get_mut(5), b"by calling the test utility TSTCK3.");
    fstr::assign(LINES.get_mut(6), b" ");
    fstr::assign(
        LINES.get_mut(7),
        b"This kernel describes only the orientation attributes of the",
    );
    fstr::assign(LINES.get_mut(8), b"TST_PHOENIX instrument.");
    fstr::assign(LINES.get_mut(9), b" ");
    fstr::assign(
        LINES.get_mut(10),
        b"This kernel is intended only for test purposes.  It is primarily",
    );
    fstr::assign(
        LINES.get_mut(11),
        b"useful for testing the TK-frames data fetching routines",
    );
    fstr::assign(LINES.get_mut(12), b" ");
    testutil::BEGDAT(&mut LINES[13]);
    fstr::assign(
        LINES.get_mut(14),
        b"TKFRAME_-111111_SPEC              = \'MATRIX\'",
    );
    fstr::assign(
        LINES.get_mut(15),
        b"TKFRAME_-111111_RELATIVE =  \'PHOENIX\'",
    );
    fstr::assign(LINES.get_mut(16), b"TKFRAME_-111111_MATRIX   = ( 0.48");
    fstr::assign(LINES.get_mut(17), b"0.60");
    fstr::assign(LINES.get_mut(18), b"0.64");
    fstr::assign(LINES.get_mut(19), b"-0.8");
    fstr::assign(LINES.get_mut(20), b"0.0");
    fstr::assign(LINES.get_mut(21), b"0.6");
    fstr::assign(LINES.get_mut(22), b"0.36");
    fstr::assign(LINES.get_mut(23), b"-0.80");
    fstr::assign(LINES.get_mut(24), b"0.48 )");
    fstr::assign(LINES.get_mut(25), b" ");
    fstr::assign(
        LINES.get_mut(26),
        b"TKFRAME_-399999_SPEC              = \'ANGLES\'",
    );
    fstr::assign(
        LINES.get_mut(27),
        b"TKFRAME_-399999_RELATIVE          = \'IAU_EARTH\'",
    );
    fstr::assign(
        LINES.get_mut(28),
        b"TKFRAME_-399999_AXES              = ( 3, 2, 3 )",
    );
    fstr::assign(
        LINES.get_mut(29),
        b"TKFRAME_-399999_ANGLES            = ( 90, 56.1829, -118.0 )",
    );
    fstr::assign(
        LINES.get_mut(30),
        b"TKFRAME_-399999_UNITS             = \'DEGREES\'",
    );
    testutil::BEGTXT(&mut LINES[31]);
    fstr::assign(LINES.get_mut(32), b" ");
    fstr::assign(
        LINES.get_mut(33),
        b"Next we need to supply the various bits of frame identification for",
    );
    fstr::assign(LINES.get_mut(34), b"this instrument.");
    fstr::assign(LINES.get_mut(35), b" ");
    testutil::BEGDAT(&mut LINES[36]);
    fstr::assign(LINES.get_mut(37), b"FRAME_-399999_CLASS    =  4");
    fstr::assign(LINES.get_mut(38), b"FRAME_-399999_CENTER   =  399");
    fstr::assign(LINES.get_mut(39), b"FRAME_-399999_CLASS_ID = -399999");
    fstr::assign(
        LINES.get_mut(40),
        b"FRAME_-399999_NAME     = \'TOPOCENTRIC\'",
    );
    fstr::assign(LINES.get_mut(41), b"FRAME_TOPOCENTRIC      = -399999");
    fstr::assign(LINES.get_mut(42), b" ");
    fstr::assign(LINES.get_mut(43), b"FRAME_-111111_CLASS    =  4");
    fstr::assign(LINES.get_mut(44), b"FRAME_-111111_CENTER   = -9");
    fstr::assign(LINES.get_mut(45), b"FRAME_-111111_CLASS_ID = -111111");
    fstr::assign(
        LINES.get_mut(46),
        b"FRAME_-111111_NAME     = \'TST-PHOENIX\'",
    );
    fstr::assign(LINES.get_mut(47), b"FRAME_TST-PHOENIX      = -111111");
    fstr::assign(LINES.get_mut(48), b" ");
    fstr::assign(LINES.get_mut(49), b"FRAME_-9999_CLASS      =  3");
    fstr::assign(LINES.get_mut(50), b"FRAME_-9999_CENTER     = -9");
    fstr::assign(LINES.get_mut(51), b"FRAME_-9999_CLASS_ID   = -9999");
    fstr::assign(LINES.get_mut(52), b"FRAME_-9999_NAME       = \'PHOENIX\'");
    fstr::assign(LINES.get_mut(53), b"FRAME_PHOENIX          = -9999");
    fstr::assign(LINES.get_mut(54), b" ");
    fstr::assign(LINES.get_mut(55), b"CK_-9999_SCLK          =  -9");
    fstr::assign(LINES.get_mut(56), b"CK_-9999_SPK           =  -9");

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_PXFORM", ctx)?;

    //
    // Create the C-kernels and PCK files needed for the
    // rest of this test.
    //
    spicelib::CLPOOL(ctx)?;

    testutil::KILFIL(b"phoenix.bc", ctx)?;
    testutil::KILFIL(b"phoenix.tsc", ctx)?;
    testutil::KILFIL(b"phoenix.ik", ctx)?;
    testutil::KILFIL(b"test_pck.ker", ctx)?;

    testutil::TSTCKN(
        b"phoenix.bc",
        b"phoenix.tsc",
        false,
        true,
        false,
        &mut HANDLE,
        ctx,
    )?;
    testutil::TSTPCK(b"test_pck.ker", true, false, ctx)?;
    spicelib::CKLPF(b"phoenix.bc", &mut HANDLE, ctx)?;
    testutil::TSTTXT(b"phoenix.ik", LINES.as_arg(), NLINES, true, false, ctx)?;

    testutil::TCASE(b"Examine the exception produced by PXFORM", ctx)?;

    ET = -100000000.0;

    spicelib::PXFORM(b"SPUD", b"SPAM", ET, XFORM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(UNKNOWNFRAME)", OK, ctx)?;

    testutil::TCASE(
        b"Make sure that a sample of inertial frame transformations behave as expected. ",
        ctx,
    )?;

    for I in 1..=18 {
        for J in 1..=18 {
            IDI = I;
            IDJ = J;
            fstr::assign(&mut NAMEI, b" ");
            fstr::assign(&mut NAMEJ, b" ");

            spicelib::IRFNAM(IDI, &mut NAMEI, ctx)?;
            spicelib::IRFNAM(IDJ, &mut NAMEJ, ctx)?;

            testutil::TSTMSG(b"#", b"Frames: #, # ", ctx);
            testutil::TSTMSC(&NAMEI, ctx);
            testutil::TSTMSC(&NAMEJ, ctx);

            spicelib::PXFORM(&NAMEI, &NAMEJ, ET, XFORM.as_slice_mut(), ctx)?;

            spicelib::IRFROT(IDI, IDJ, ROT.as_slice_mut(), ctx)?;

            for K in 1..=3 {
                for L in 1..=3 {
                    EFORM[[K, L]] = ROT[[K, L]];
                }
            }

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKAD(
                b"XFORM",
                XFORM.as_slice(),
                b"~",
                EFORM.as_slice(),
                9,
                0.00000000000001,
                OK,
                ctx,
            )?;
        }
    }

    testutil::TSTMSG(b"#", b" ", ctx);

    testutil::TCASE(b"Make sure that the PCK frames are recognized ", ctx)?;

    fstr::assign(NAME.get_mut(1), b"IAU_MERCURY");
    fstr::assign(NAME.get_mut(2), b"IAU_VENUS");
    fstr::assign(NAME.get_mut(3), b"IAU_EARTH");
    fstr::assign(NAME.get_mut(4), b"IAU_MARS");
    fstr::assign(NAME.get_mut(5), b"IAU_JUPITER");
    fstr::assign(NAME.get_mut(6), b"IAU_SATURN");
    fstr::assign(NAME.get_mut(7), b"IAU_URANUS");
    fstr::assign(NAME.get_mut(8), b"IAU_NEPTUNE");
    fstr::assign(NAME.get_mut(9), b"IAU_PLUTO");
    fstr::assign(NAME.get_mut(10), b"IAU_MOON");
    fstr::assign(NAME.get_mut(11), b"IAU_PHOBOS");
    fstr::assign(NAME.get_mut(12), b"IAU_DEIMOS");
    fstr::assign(NAME.get_mut(13), b"IAU_IO");
    fstr::assign(NAME.get_mut(14), b"IAU_EUROPA");
    fstr::assign(NAME.get_mut(15), b"IAU_GANYMEDE");
    fstr::assign(NAME.get_mut(16), b"IAU_CALLISTO");
    fstr::assign(NAME.get_mut(17), b"IAU_ARIEL");
    fstr::assign(NAME.get_mut(18), b"IAU_OBERON");
    fstr::assign(NAME.get_mut(19), b"IAU_MIRANDA");
    fstr::assign(NAME.get_mut(20), b"IAU_UMBRIEL");
    fstr::assign(NAME.get_mut(21), b"IAU_TITANIA");
    fstr::assign(NAME.get_mut(22), b"IAU_TITAN");
    fstr::assign(NAME.get_mut(23), b"IAU_TRITON");
    fstr::assign(NAME.get_mut(24), b"IAU_CHARON");

    IDCODE[1] = 199;
    IDCODE[2] = 299;
    IDCODE[3] = 399;
    IDCODE[4] = 499;
    IDCODE[5] = 599;
    IDCODE[6] = 699;
    IDCODE[7] = 799;
    IDCODE[8] = 899;
    IDCODE[9] = 999;
    IDCODE[10] = 301;
    IDCODE[11] = 401;
    IDCODE[12] = 402;
    IDCODE[13] = 501;
    IDCODE[14] = 502;
    IDCODE[15] = 503;
    IDCODE[16] = 504;
    IDCODE[17] = 701;
    IDCODE[18] = 704;
    IDCODE[19] = 705;
    IDCODE[20] = 702;
    IDCODE[21] = 703;
    IDCODE[22] = 606;
    IDCODE[23] = 801;
    IDCODE[24] = 901;

    for I in 1..=24 {
        testutil::TSTMSG(b"#", b"Body: # ", ctx);
        testutil::TSTMSC(&NAME[I], ctx);

        spicelib::PXFORM(b"J2000", &NAME[I], ET, XFORM.as_slice_mut(), ctx)?;
        spicelib::TIPBOD(b"J2000", IDCODE[I], ET, TSIPM.as_slice_mut(), ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKAD(
            b"XFORM",
            XFORM.as_slice(),
            b"~",
            TSIPM.as_slice(),
            9,
            0.00000000000001,
            OK,
            ctx,
        )?;
    }

    testutil::TSTMSG(b"#", b" ", ctx);

    testutil::TCASE(
        b"Make sure the transformation from bodyfixed to J2000 works. ",
        ctx,
    )?;

    for I in 1..=24 {
        testutil::TSTMSG(b"#", b"Body: # ", ctx);
        testutil::TSTMSC(&NAME[I], ctx);

        spicelib::PXFORM(&NAME[I], b"J2000", ET, XFORM.as_slice_mut(), ctx)?;
        spicelib::TIPBOD(b"J2000", IDCODE[I], ET, TSIPM.as_slice_mut(), ctx)?;
        spicelib::XPOSE(TSIPM.as_slice(), TMPMAT.as_slice_mut());
        spicelib::MOVED(TMPMAT.as_slice(), 9, TSIPM.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKAD(
            b"XFORM",
            XFORM.as_slice(),
            b"~",
            TSIPM.as_slice(),
            9,
            0.00000000000001,
            OK,
            ctx,
        )?;
    }

    testutil::TSTMSG(b"#", b" ", ctx);

    testutil::TCASE(
        b"Make sure that we can tranform from one bodyfixed frame to another. ",
        ctx,
    )?;

    for I in 1..=24 {
        for J in 1..=24 {
            testutil::TSTMSG(b"#", b"Body  # to Body #", ctx);
            testutil::TSTMSC(&NAME[I], ctx);
            testutil::TSTMSC(&NAME[J], ctx);

            spicelib::TIPBOD(b"J2000", IDCODE[I], ET, TSPMI.as_slice_mut(), ctx)?;
            spicelib::XPOSE(TSPMI.as_slice(), TMPMAT.as_slice_mut());
            spicelib::MOVED(TMPMAT.as_slice(), 9, TSPMI.as_slice_mut());

            spicelib::TIPBOD(b"J2000", IDCODE[J], ET, TSIPM.as_slice_mut(), ctx)?;

            spicelib::MXM(TSIPM.as_slice(), TSPMI.as_slice(), EFORM.as_slice_mut());

            spicelib::PXFORM(&NAME[I], &NAME[J], ET, XFORM.as_slice_mut(), ctx)?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKAD(
                b"XFORM",
                XFORM.as_slice(),
                b"~",
                EFORM.as_slice(),
                9,
                0.00000000000001,
                OK,
                ctx,
            )?;
        }
    }

    testutil::TSTMSG(b"#", b" ", ctx);

    testutil::TCASE(
        b"Make sure a long tranformation chain is properly computed. ",
        ctx,
    )?;

    spicelib::NAMFRM(b"TOPOCENTRIC", &mut TOPO, ctx)?;
    spicelib::NAMFRM(b"TST-PHOENIX", &mut PHNX, ctx)?;

    spicelib::PXFORM(
        b"TOPOCENTRIC",
        b"TST-PHOENIX",
        ET,
        XFORM.as_slice_mut(),
        ctx,
    )?;
    spicelib::REFCHG(TOPO, PHNX, ET, EFORM.as_slice_mut(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"XFORM",
        XFORM.as_slice(),
        b"=",
        EFORM.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;

    testutil::TSTMSG(b"#", b" ", ctx);

    spicelib::CKUPF(HANDLE, ctx)?;

    spicelib::CLPOOL(ctx)?;
    testutil::KILFIL(b"phoenix.bc", ctx)?;
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
