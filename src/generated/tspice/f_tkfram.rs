//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LBFSIZ: i32 = 300;
const LNSIZE: i32 = 132;
const SMLSIZ: i32 = 30;
const BUFSIZ: i32 = 200;
const MAXSIZ: i32 = ((BUFSIZ * 2) + 5);

struct SaveVars {
    BUFFER: ActualCharArray,
    LINES: ActualCharArray,
    QNAME: Vec<u8>,
    ANGIDN: StackArray<f64, 3>,
    ANGLE1: ActualArray<f64>,
    ANGLE2: ActualArray<f64>,
    ANGLE3: ActualArray<f64>,
    EROT: StackArray<f64, 9>,
    QIDN: StackArray<f64, 4>,
    R: ActualArray3D<f64>,
    ROT: StackArray2D<f64, 9>,
    ROTIDN: StackArray2D<f64, 9>,
    XROT: StackArray2D<f64, 9>,
    AXES: StackArray<i32, 3>,
    EFRAME: i32,
    FRAME: i32,
    ID: i32,
    J: i32,
    MAXVAR: i32,
    NLINES: i32,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BUFFER = ActualCharArray::new(LNSIZE, 1..=SMLSIZ);
        let mut LINES = ActualCharArray::new(LNSIZE, 1..=LBFSIZ);
        let mut QNAME = vec![b' '; LNSIZE as usize];
        let mut ANGIDN = StackArray::<f64, 3>::new(1..=3);
        let mut ANGLE1 = ActualArray::<f64>::new(1..=MAXSIZ);
        let mut ANGLE2 = ActualArray::<f64>::new(1..=MAXSIZ);
        let mut ANGLE3 = ActualArray::<f64>::new(1..=MAXSIZ);
        let mut EROT = StackArray::<f64, 9>::new(1..=9);
        let mut QIDN = StackArray::<f64, 4>::new(1..=4);
        let mut R = ActualArray3D::<f64>::new(1..=3, 1..=3, 1..=MAXSIZ);
        let mut ROT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut ROTIDN = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut XROT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut AXES = StackArray::<i32, 3>::new(1..=3);
        let mut EFRAME: i32 = 0;
        let mut FRAME: i32 = 0;
        let mut ID: i32 = 0;
        let mut J: i32 = 0;
        let mut MAXVAR: i32 = 0;
        let mut NLINES: i32 = 0;
        let mut FOUND: bool = false;

        Self {
            BUFFER,
            LINES,
            QNAME,
            ANGIDN,
            ANGLE1,
            ANGLE2,
            ANGLE3,
            EROT,
            QIDN,
            R,
            ROT,
            ROTIDN,
            XROT,
            AXES,
            EFRAME,
            FRAME,
            ID,
            J,
            MAXVAR,
            NLINES,
            FOUND,
        }
    }
}

//$Procedure F_TKFRAM ( Family Test of TKFRAM )
pub fn F_TKFRAM(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Test Utility Functions
    //

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // BUFSIZ must be set to BUFSIZ used in TKFRAM.
    //

    //
    // Maximum size of angle, matrix lists.
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_TKFRAM", ctx)?;
    //
    // Clear the kernel pool so that we don't have to worry
    // about previous test cases contaminating the kernel pool.
    //
    spicelib::CLPOOL(ctx)?;

    //
    // Get rid of any existing frame kernels.
    //
    testutil::KILFIL(b"phoenix.tk", ctx)?;
    testutil::KILFIL(b"phoenix.prt", ctx)?;

    //
    // Create an I-kernel that we can load later on.
    //
    fstr::assign(
        save.LINES.get_mut(1),
        b"This is a test TK Frame kernel for the fictional instrument TST_PHOENIX",
    );
    fstr::assign(
        save.LINES.get_mut(2),
        b"on board the fictional spacecraft PHOENIX.  A C-kernel for",
    );
    fstr::assign(
        save.LINES.get_mut(3),
        b"the platform on which TST_PHOENIX is mounted can be generated",
    );
    fstr::assign(
        save.LINES.get_mut(4),
        b"by calling the test utility TSTCK3.",
    );
    fstr::assign(save.LINES.get_mut(5), b" ");
    fstr::assign(
        save.LINES.get_mut(6),
        b"This kernel describes only the orientation attributes of the",
    );
    fstr::assign(save.LINES.get_mut(7), b"TST_PHOENIX instrument.");
    fstr::assign(save.LINES.get_mut(8), b" ");
    fstr::assign(
        save.LINES.get_mut(9),
        b"This kernel is intended only for test purposes.  It is primarily",
    );
    fstr::assign(
        save.LINES.get_mut(10),
        b"useful for testing the TKFRAM data fetching routine.",
    );
    fstr::assign(save.LINES.get_mut(11), b" ");
    testutil::BEGDAT(&mut save.LINES[12]);
    fstr::assign(save.LINES.get_mut(13), b" ");
    fstr::assign(
        save.LINES.get_mut(14),
        b"TKFRAME_-111111_SPEC      = \'MATRIX\'",
    );
    fstr::assign(
        save.LINES.get_mut(15),
        b"TKFRAME_-111111_RELATIVE  = \'PHOENIX\'",
    );
    fstr::assign(save.LINES.get_mut(16), b" ");
    fstr::assign(
        save.LINES.get_mut(17),
        b"TKFRAME_-111111_MATRIX    = ( 0.48",
    );
    fstr::assign(save.LINES.get_mut(18), b" 0.60");
    fstr::assign(save.LINES.get_mut(19), b" 0.64");
    fstr::assign(save.LINES.get_mut(20), b"-0.8");
    fstr::assign(save.LINES.get_mut(21), b" 0.0");
    fstr::assign(save.LINES.get_mut(22), b" 0.6");
    fstr::assign(save.LINES.get_mut(23), b" 0.36");
    fstr::assign(save.LINES.get_mut(24), b"-0.80");
    fstr::assign(save.LINES.get_mut(25), b" 0.48 )");
    fstr::assign(save.LINES.get_mut(26), b" ");
    fstr::assign(save.LINES.get_mut(27), b" ");
    fstr::assign(
        save.LINES.get_mut(28),
        b"TKFRAME_TST2-PHOENIX_SPEC      = \'ROTATION\'",
    );
    fstr::assign(
        save.LINES.get_mut(29),
        b"TKFRAME_TST2-PHOENIX_RELATIVE  = \'PHOENIX\'",
    );
    fstr::assign(save.LINES.get_mut(30), b" ");
    fstr::assign(
        save.LINES.get_mut(31),
        b"TKFRAME_-111112_ROTATION  = ( 0.48",
    );
    fstr::assign(save.LINES.get_mut(32), b" 0.60");
    fstr::assign(save.LINES.get_mut(33), b" 0.64");
    fstr::assign(save.LINES.get_mut(34), b"-0.8");
    fstr::assign(save.LINES.get_mut(35), b" 0.0");
    fstr::assign(save.LINES.get_mut(36), b" 0.6");
    fstr::assign(save.LINES.get_mut(37), b" 0.36");
    fstr::assign(save.LINES.get_mut(38), b"-0.80");
    fstr::assign(save.LINES.get_mut(39), b" 0.48 )");
    fstr::assign(save.LINES.get_mut(40), b" ");
    fstr::assign(save.LINES.get_mut(41), b" ");
    testutil::BEGTXT(&mut save.LINES[42]);
    fstr::assign(save.LINES.get_mut(43), b" ");
    fstr::assign(
        save.LINES.get_mut(44),
        b"Next we need to supply the various bits of frame identification for",
    );
    fstr::assign(save.LINES.get_mut(45), b"this instrument.");
    fstr::assign(save.LINES.get_mut(46), b" ");
    testutil::BEGDAT(&mut save.LINES[47]);
    fstr::assign(save.LINES.get_mut(48), b" ");
    fstr::assign(save.LINES.get_mut(49), b"FRAME_-111111_CLASS    =  4");
    fstr::assign(save.LINES.get_mut(50), b"FRAME_-111111_CENTER   = -9");
    fstr::assign(save.LINES.get_mut(51), b"FRAME_-111111_CLASS_ID = -111111");
    fstr::assign(
        save.LINES.get_mut(52),
        b"FRAME_-111111_NAME     = \'TST-PHOENIX\'",
    );
    fstr::assign(save.LINES.get_mut(53), b"FRAME_TST-PHOENIX      = -111111");
    fstr::assign(save.LINES.get_mut(54), b" ");
    fstr::assign(save.LINES.get_mut(55), b"FRAME_-9999_CLASS      =  3");
    fstr::assign(save.LINES.get_mut(56), b"FRAME_-9999_CENTER     = -9");
    fstr::assign(save.LINES.get_mut(57), b"FRAME_-9999_CLASS_ID   = -9999");
    fstr::assign(
        save.LINES.get_mut(58),
        b"FRAME_-9999_NAME       = \'PHOENIX\'",
    );
    fstr::assign(save.LINES.get_mut(59), b"FRAME_PHOENIX          = -9999");
    fstr::assign(save.LINES.get_mut(60), b" ");
    fstr::assign(save.LINES.get_mut(61), b"FRAME_-111112_CLASS    =  4");
    fstr::assign(save.LINES.get_mut(62), b"FRAME_-111112_CENTER   = -9");
    fstr::assign(save.LINES.get_mut(63), b"FRAME_-111112_CLASS_ID = -111112");
    fstr::assign(
        save.LINES.get_mut(64),
        b"FRAME_-111112_NAME     = \'TST2-PHOENIX\'",
    );
    fstr::assign(save.LINES.get_mut(65), b"FRAME_TST2-PHOENIX     = -111112");
    fstr::assign(save.LINES.get_mut(66), b" ");
    fstr::assign(save.LINES.get_mut(67), b" ");
    fstr::assign(save.LINES.get_mut(68), b" ");
    fstr::assign(save.LINES.get_mut(69), b"CK_-9999_SCLK          =  -9");
    fstr::assign(save.LINES.get_mut(70), b"CK_-9999_SPK           =  -9");
    fstr::assign(save.LINES.get_mut(71), b" ");
    testutil::BEGTXT(&mut save.LINES[72]);
    fstr::assign(save.LINES.get_mut(73), b" ");
    fstr::assign(
        save.LINES.get_mut(74),
        b"This frame is defined relative to itself. A no-no.",
    );
    fstr::assign(save.LINES.get_mut(75), b" ");
    testutil::BEGDAT(&mut save.LINES[76]);
    fstr::assign(save.LINES.get_mut(77), b" ");
    fstr::assign(
        save.LINES.get_mut(78),
        b"FRAME_-111113_NAME        = \'TST3-PHOENIX\'",
    );
    fstr::assign(
        save.LINES.get_mut(79),
        b"FRAME_TST3-PHOENIX        = -111113",
    );
    fstr::assign(save.LINES.get_mut(80), b"FRAME_-111113_CLASS       =  4");
    fstr::assign(save.LINES.get_mut(81), b"FRAME_-111113_CENTER      = -9");
    fstr::assign(
        save.LINES.get_mut(82),
        b"FRAME_-111113_CLASS_ID    = -111113",
    );
    fstr::assign(
        save.LINES.get_mut(83),
        b"TKFRAME_-111113_SPEC      = \'MATRIX\'",
    );
    fstr::assign(
        save.LINES.get_mut(84),
        b"TKFRAME_-111113_RELATIVE  = \'TST3-PHOENIX\'",
    );
    fstr::assign(save.LINES.get_mut(85), b"TKFRAME_-111113_ROTATION  = (");
    fstr::assign(
        save.LINES.get_mut(86),
        b"0.48 0.60 0.64 -0.8 0.0 0.6 0.36 -0.80 0.48 )",
    );

    spicelib::IDENT(save.ROTIDN.as_slice_mut());
    spicelib::M2Q(save.ROTIDN.as_slice(), save.QIDN.as_slice_mut(), ctx)?;
    spicelib::VPACK(0.0, 0.0, 0.0, save.ANGIDN.as_slice_mut());
    save.AXES[1] = 1;
    save.AXES[2] = 2;
    save.AXES[3] = 3;

    //
    // *****************************************************************
    //
    //    Error cases
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Check that a zero instrument ID causes an error to be signaled. ",
        ctx,
    )?;

    save.ID = 0;
    spicelib::TKFRAM(
        save.ID,
        save.ROT.as_slice_mut(),
        &mut save.FRAME,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ZEROFRAMEID)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check that a proper error is signaled if there is no frame data for the requested frame. ", ctx)?;

    save.ID = -10;
    spicelib::TKFRAM(
        save.ID,
        save.ROT.as_slice_mut(),
        &mut save.FRAME,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INCOMPLETEFRAME)", OK, ctx)?;
    testutil::CHCKAD(
        b"ROT",
        save.ROT.as_slice(),
        b"=",
        save.ROTIDN.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"FRAME", save.FRAME, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Load a TK frame that is missing the orientation information and check that the deficiency is properly diagnosed.", ctx)?;

    save.ID = -111111;

    spicelib::CLPOOL(ctx)?;
    testutil::TSTTXT(b"phoenix.prt", save.LINES.as_arg(), 53, true, true, ctx)?;

    spicelib::TKFRAM(
        save.ID,
        save.ROT.as_slice_mut(),
        &mut save.FRAME,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADFRAMESPEC)", OK, ctx)?;
    testutil::CHCKAD(
        b"ROT",
        save.ROT.as_slice(),
        b"=",
        save.ROTIDN.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"FRAME", save.FRAME, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Make sure that the proper error is signaled if the frame \'SPEC\' is not recognized. ",
        ctx,
    )?;

    save.ID = -111112;
    spicelib::CLPOOL(ctx)?;
    testutil::TSTTXT(b"phoenix.tk", save.LINES.as_arg(), 86, true, true, ctx)?;
    spicelib::TKFRAM(
        save.ID,
        save.ROT.as_slice_mut(),
        &mut save.FRAME,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(UNKNOWNFRAMESPEC)", OK, ctx)?;
    testutil::CHCKAD(
        b"ROT",
        save.ROT.as_slice(),
        b"=",
        save.ROTIDN.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"FRAME", save.FRAME, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Make sure that the proper error is signaled if the frame is defined relative to itself. ",
        ctx,
    )?;

    save.ID = -111113;
    spicelib::TKFRAM(
        save.ID,
        save.ROT.as_slice_mut(),
        &mut save.FRAME,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADFRAMESPEC2)", OK, ctx)?;
    testutil::CHCKAD(
        b"ROT",
        save.ROT.as_slice(),
        b"=",
        save.ROTIDN.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"FRAME", save.FRAME, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Make sure that the proper error is signaled if there are competing _SPEC keywords",
        ctx,
    )?;

    spicelib::PCPOOL(
        b"TKFRAME_TST3-PHOENIX_SPEC",
        1,
        CharArray::from_ref(b"MATRIX"),
        ctx,
    )?;

    save.ID = -111113;
    spicelib::TKFRAM(
        save.ID,
        save.ROT.as_slice_mut(),
        &mut save.FRAME,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(COMPETINGFRAMESPEC)", OK, ctx)?;
    testutil::CHCKAD(
        b"ROT",
        save.ROT.as_slice(),
        b"=",
        save.ROTIDN.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"FRAME", save.FRAME, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    spicelib::DVPOOL(b"TKFRAME_TST3-PHOENIX_SPEC", ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Make sure that the proper error is signaled if there are competing _RELATIVE keywords",
        ctx,
    )?;

    spicelib::PCPOOL(
        b"TKFRAME_-111113_RELATIVE",
        1,
        CharArray::from_ref(b"J2000"),
        ctx,
    )?;
    spicelib::PCPOOL(
        b"TKFRAME_TST3-PHOENIX_RELATIVE",
        1,
        CharArray::from_ref(b"J2000"),
        ctx,
    )?;

    save.ID = -111113;
    spicelib::TKFRAM(
        save.ID,
        save.ROT.as_slice_mut(),
        &mut save.FRAME,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(COMPETINGFRAMESPEC)", OK, ctx)?;
    testutil::CHCKAD(
        b"ROT",
        save.ROT.as_slice(),
        b"=",
        save.ROTIDN.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"FRAME", save.FRAME, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    spicelib::DVPOOL(b"TKFRAME_TST3-PHOENIX_RELATIVE", ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Make sure that the proper error is signaled if there are competing _MATRIX keywords",
        ctx,
    )?;

    spicelib::PCPOOL(
        b"TKFRAME_-111113_SPEC",
        1,
        CharArray::from_ref(b"MATRIX"),
        ctx,
    )?;

    spicelib::PDPOOL(b"TKFRAME_-111113_MATRIX", 9, save.ROTIDN.as_slice(), ctx)?;
    spicelib::PDPOOL(
        b"TKFRAME_TST3-PHOENIX_MATRIX",
        9,
        save.ROTIDN.as_slice(),
        ctx,
    )?;

    save.ID = -111113;
    spicelib::TKFRAM(
        save.ID,
        save.ROT.as_slice_mut(),
        &mut save.FRAME,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(COMPETINGFRAMESPEC)", OK, ctx)?;
    testutil::CHCKAD(
        b"ROT",
        save.ROT.as_slice(),
        b"=",
        save.ROTIDN.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"FRAME", save.FRAME, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    spicelib::DVPOOL(b"TKFRAME_-111113_MATRIX", ctx)?;
    spicelib::DVPOOL(b"TKFRAME_TST3-PHOENIX_MATRIX", ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Make sure that the proper error is signaled if there are competing _Q keywords",
        ctx,
    )?;

    spicelib::PCPOOL(
        b"TKFRAME_-111113_SPEC",
        1,
        CharArray::from_ref(b"QUATERNION"),
        ctx,
    )?;

    spicelib::PDPOOL(b"TKFRAME_-111113_Q", 4, save.QIDN.as_slice(), ctx)?;
    spicelib::PDPOOL(b"TKFRAME_TST3-PHOENIX_Q", 4, save.QIDN.as_slice(), ctx)?;

    save.ID = -111113;
    spicelib::TKFRAM(
        save.ID,
        save.ROT.as_slice_mut(),
        &mut save.FRAME,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(COMPETINGFRAMESPEC)", OK, ctx)?;
    testutil::CHCKAD(
        b"ROT",
        save.ROT.as_slice(),
        b"=",
        save.ROTIDN.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"FRAME", save.FRAME, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    spicelib::DVPOOL(b"TKFRAME_-111113_Q", ctx)?;
    spicelib::DVPOOL(b"TKFRAME_TST3-PHOENIX_Q", ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Make sure that the proper error is signaled if there are competing _ANGLES keywords",
        ctx,
    )?;

    spicelib::PCPOOL(
        b"TKFRAME_-111113_SPEC",
        1,
        CharArray::from_ref(b"ANGLES"),
        ctx,
    )?;

    spicelib::PDPOOL(b"TKFRAME_-111113_ANGLES", 3, save.ANGIDN.as_slice(), ctx)?;
    spicelib::PDPOOL(
        b"TKFRAME_TST3-PHOENIX_ANGLES",
        3,
        save.ANGIDN.as_slice(),
        ctx,
    )?;
    spicelib::PIPOOL(b"TKFRAME_-111113_AXES", 3, save.AXES.as_slice(), ctx)?;
    spicelib::PCPOOL(
        b"TKFRAME_-111113_UNITS",
        1,
        CharArray::from_ref(b"DEGREES"),
        ctx,
    )?;

    save.ID = -111113;
    spicelib::TKFRAM(
        save.ID,
        save.ROT.as_slice_mut(),
        &mut save.FRAME,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(COMPETINGFRAMESPEC)", OK, ctx)?;
    testutil::CHCKAD(
        b"ROT",
        save.ROT.as_slice(),
        b"=",
        save.ROTIDN.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"FRAME", save.FRAME, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    spicelib::DVPOOL(b"TKFRAME_TST3-PHOENIX_ANGLES", ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Make sure that the proper error is signaled if there are competing _AXES keywords",
        ctx,
    )?;

    spicelib::PCPOOL(
        b"TKFRAME_-111113_SPEC",
        1,
        CharArray::from_ref(b"ANGLES"),
        ctx,
    )?;

    spicelib::PDPOOL(b"TKFRAME_-111113_ANGLES", 3, save.ANGIDN.as_slice(), ctx)?;
    spicelib::PIPOOL(b"TKFRAME_-111113_AXES", 3, save.AXES.as_slice(), ctx)?;
    spicelib::PIPOOL(b"TKFRAME_TST3-PHOENIX_AXES", 3, save.AXES.as_slice(), ctx)?;
    spicelib::PCPOOL(
        b"TKFRAME_-111113_UNITS",
        1,
        CharArray::from_ref(b"DEGREES"),
        ctx,
    )?;

    save.ID = -111113;
    spicelib::TKFRAM(
        save.ID,
        save.ROT.as_slice_mut(),
        &mut save.FRAME,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(COMPETINGFRAMESPEC)", OK, ctx)?;
    testutil::CHCKAD(
        b"ROT",
        save.ROT.as_slice(),
        b"=",
        save.ROTIDN.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"FRAME", save.FRAME, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    spicelib::DVPOOL(b"TKFRAME_TST3-PHOENIX_AXES", ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Make sure that the proper error is signaled if there are competing _UNITS keywords",
        ctx,
    )?;

    spicelib::PCPOOL(
        b"TKFRAME_-111113_SPEC",
        1,
        CharArray::from_ref(b"ANGLES"),
        ctx,
    )?;

    spicelib::PDPOOL(b"TKFRAME_-111113_ANGLES", 3, save.ANGIDN.as_slice(), ctx)?;
    spicelib::PIPOOL(b"TKFRAME_-111113_AXES", 3, save.AXES.as_slice(), ctx)?;
    spicelib::PCPOOL(
        b"TKFRAME_-111113_UNITS",
        1,
        CharArray::from_ref(b"DEGREES"),
        ctx,
    )?;
    spicelib::PCPOOL(
        b"TKFRAME_TST3-PHOENIX_UNITS",
        1,
        CharArray::from_ref(b"DEGREES"),
        ctx,
    )?;

    save.ID = -111113;
    spicelib::TKFRAM(
        save.ID,
        save.ROT.as_slice_mut(),
        &mut save.FRAME,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(COMPETINGFRAMESPEC)", OK, ctx)?;
    testutil::CHCKAD(
        b"ROT",
        save.ROT.as_slice(),
        b"=",
        save.ROTIDN.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"FRAME", save.FRAME, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    spicelib::DVPOOL(b"TKFRAME_TST3-PHOENIX_UNITS", ctx)?;

    //
    // *****************************************************************
    //
    //    Normal cases
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Determine the orientation of the instrument frame relative to its C-kernel frame. ",
        ctx,
    )?;

    save.ID = -111111;

    spicelib::TKFRAM(
        save.ID,
        save.ROT.as_slice_mut(),
        &mut save.FRAME,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.EROT[1] = 0.48;
    save.EROT[2] = 0.6;
    save.EROT[3] = 0.64;
    save.EROT[4] = -0.8;
    save.EROT[5] = 0.0;
    save.EROT[6] = 0.6;
    save.EROT[7] = 0.36;
    save.EROT[8] = -0.8;
    save.EROT[9] = 0.48;

    save.EFRAME = -9999;

    testutil::CHCKAD(
        b"ROT",
        save.ROT.as_slice(),
        b"~",
        save.EROT.as_slice(),
        9,
        0.00000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"FRAME", save.FRAME, b"=", save.EFRAME, 0, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Make sure that we can successfully retrieve rotations for 25 realistic frames.",
        ctx,
    )?;

    fstr::assign(save.LINES.get_mut(1), b" ");
    fstr::assign(
        save.LINES.get_mut(2),
        b"This PCK file contains definitions for TOPOGRAPHIC reference",
    );
    fstr::assign(
        save.LINES.get_mut(3),
        b"frames at 25 different observatories around the world.  Note",
    );
    fstr::assign(
        save.LINES.get_mut(4),
        b"that the definition of these frames is approximate and that",
    );
    fstr::assign(
        save.LINES.get_mut(5),
        b"they are accurate to only about 0.1 degrees.",
    );
    fstr::assign(save.LINES.get_mut(6), b" ");
    testutil::BEGDAT(&mut save.LINES[7]);
    fstr::assign(save.LINES.get_mut(8), b" ");
    fstr::assign(save.LINES.get_mut(9), b"FRAME_LAPLATA_TOPO     = 1000001");
    fstr::assign(save.LINES.get_mut(10), b"FRAME_CANBERRA_TOPO    = 1000002");
    fstr::assign(save.LINES.get_mut(11), b"FRAME_URANIA_TOPO      = 1000003");
    fstr::assign(save.LINES.get_mut(12), b"FRAME_VALONGO_TOPO     = 1000004");
    fstr::assign(save.LINES.get_mut(13), b"FRAME_LASCAMPANAS_TOPO = 1000005");
    fstr::assign(save.LINES.get_mut(14), b"FRAME_YUNNAN_TOPO      = 1000006");
    fstr::assign(save.LINES.get_mut(15), b"FRAME_QUITO_TOPO       = 1000007");
    fstr::assign(save.LINES.get_mut(16), b"FRAME_TUORLA_TOPO      = 1000008");
    fstr::assign(save.LINES.get_mut(17), b"FRAME_GRENOBLE_TOPO    = 1000009");
    fstr::assign(save.LINES.get_mut(18), b"FRAME_HAMBURG_TOPO     = 1000010");
    fstr::assign(save.LINES.get_mut(19), b"FRAME_MUNICH_TOPO      = 1000011");
    fstr::assign(save.LINES.get_mut(20), b"FRAME_KODAIKANAL_TOPO  = 1000012");
    fstr::assign(save.LINES.get_mut(21), b"FRAME_DUNSINK_TOPO     = 1000013");
    fstr::assign(save.LINES.get_mut(22), b"FRAME_TURIN_TOPO       = 1000014");
    fstr::assign(save.LINES.get_mut(23), b"FRAME_HIDA_TOPO        = 1000015");
    fstr::assign(save.LINES.get_mut(24), b"FRAME_ARECIBO_TOPO     = 1000016");
    fstr::assign(save.LINES.get_mut(25), b"FRAME_PULKOVO_TOPO     = 1000017");
    fstr::assign(save.LINES.get_mut(26), b"FRAME_BOYDEN_TOPO      = 1000018");
    fstr::assign(save.LINES.get_mut(27), b"FRAME_MADRID_TOPO      = 1000019");
    fstr::assign(save.LINES.get_mut(28), b"FRAME_AROSA_TOPO       = 1000020");
    fstr::assign(save.LINES.get_mut(29), b"FRAME_GREENWICH_TOPO   = 1000021");
    fstr::assign(save.LINES.get_mut(30), b"FRAME_KITTPEAK_TOPO    = 1000022");
    fstr::assign(save.LINES.get_mut(31), b"FRAME_GOLDSTONE_TOPO   = 1000023");
    fstr::assign(save.LINES.get_mut(32), b"FRAME_PALOMAR_TOPO     = 1000024");
    fstr::assign(save.LINES.get_mut(33), b"FRAME_USNO_TOPO        = 1000025");
    fstr::assign(save.LINES.get_mut(34), b" ");
    fstr::assign(save.LINES.get_mut(35), b" ");
    fstr::assign(save.LINES.get_mut(36), b" ");
    fstr::assign(save.LINES.get_mut(37), b"FRAME_1000001_CLASS    =  4");
    fstr::assign(save.LINES.get_mut(38), b"FRAME_1000002_CLASS    =  4");
    fstr::assign(save.LINES.get_mut(39), b"FRAME_1000003_CLASS    =  4");
    fstr::assign(save.LINES.get_mut(40), b"FRAME_1000004_CLASS    =  4");
    fstr::assign(save.LINES.get_mut(41), b"FRAME_1000005_CLASS    =  4");
    fstr::assign(save.LINES.get_mut(42), b"FRAME_1000006_CLASS    =  4");
    fstr::assign(save.LINES.get_mut(43), b"FRAME_1000007_CLASS    =  4");
    fstr::assign(save.LINES.get_mut(44), b"FRAME_1000008_CLASS    =  4");
    fstr::assign(save.LINES.get_mut(45), b"FRAME_1000009_CLASS    =  4");
    fstr::assign(save.LINES.get_mut(46), b"FRAME_1000010_CLASS    =  4");
    fstr::assign(save.LINES.get_mut(47), b"FRAME_1000011_CLASS    =  4");
    fstr::assign(save.LINES.get_mut(48), b"FRAME_1000012_CLASS    =  4");
    fstr::assign(save.LINES.get_mut(49), b"FRAME_1000013_CLASS    =  4");
    fstr::assign(save.LINES.get_mut(50), b"FRAME_1000014_CLASS    =  4");
    fstr::assign(save.LINES.get_mut(51), b"FRAME_1000015_CLASS    =  4");
    fstr::assign(save.LINES.get_mut(52), b"FRAME_1000016_CLASS    =  4");
    fstr::assign(save.LINES.get_mut(53), b"FRAME_1000017_CLASS    =  4");
    fstr::assign(save.LINES.get_mut(54), b"FRAME_1000018_CLASS    =  4");
    fstr::assign(save.LINES.get_mut(55), b"FRAME_1000019_CLASS    =  4");
    fstr::assign(save.LINES.get_mut(56), b"FRAME_1000020_CLASS    =  4");
    fstr::assign(save.LINES.get_mut(57), b"FRAME_1000021_CLASS    =  4");
    fstr::assign(save.LINES.get_mut(58), b"FRAME_1000022_CLASS    =  4");
    fstr::assign(save.LINES.get_mut(59), b"FRAME_1000023_CLASS    =  4");
    fstr::assign(save.LINES.get_mut(60), b"FRAME_1000024_CLASS    =  4");
    fstr::assign(save.LINES.get_mut(61), b"FRAME_1000025_CLASS    =  4");
    fstr::assign(save.LINES.get_mut(62), b" ");
    fstr::assign(save.LINES.get_mut(63), b" ");
    fstr::assign(save.LINES.get_mut(64), b"FRAME_1000001_CENTER   =  399");
    fstr::assign(save.LINES.get_mut(65), b"FRAME_1000002_CENTER   =  399");
    fstr::assign(save.LINES.get_mut(66), b"FRAME_1000003_CENTER   =  399");
    fstr::assign(save.LINES.get_mut(67), b"FRAME_1000004_CENTER   =  399");
    fstr::assign(save.LINES.get_mut(68), b"FRAME_1000005_CENTER   =  399");
    fstr::assign(save.LINES.get_mut(69), b"FRAME_1000006_CENTER   =  399");
    fstr::assign(save.LINES.get_mut(70), b"FRAME_1000007_CENTER   =  399");
    fstr::assign(save.LINES.get_mut(71), b"FRAME_1000008_CENTER   =  399");
    fstr::assign(save.LINES.get_mut(72), b"FRAME_1000009_CENTER   =  399");
    fstr::assign(save.LINES.get_mut(73), b"FRAME_1000010_CENTER   =  399");
    fstr::assign(save.LINES.get_mut(74), b"FRAME_1000011_CENTER   =  399");
    fstr::assign(save.LINES.get_mut(75), b"FRAME_1000012_CENTER   =  399");
    fstr::assign(save.LINES.get_mut(76), b"FRAME_1000013_CENTER   =  399");
    fstr::assign(save.LINES.get_mut(77), b"FRAME_1000014_CENTER   =  399");
    fstr::assign(save.LINES.get_mut(78), b"FRAME_1000015_CENTER   =  399");
    fstr::assign(save.LINES.get_mut(79), b"FRAME_1000016_CENTER   =  399");
    fstr::assign(save.LINES.get_mut(80), b"FRAME_1000017_CENTER   =  399");
    fstr::assign(save.LINES.get_mut(81), b"FRAME_1000018_CENTER   =  399");
    fstr::assign(save.LINES.get_mut(82), b"FRAME_1000019_CENTER   =  399");
    fstr::assign(save.LINES.get_mut(83), b"FRAME_1000020_CENTER   =  399");
    fstr::assign(save.LINES.get_mut(84), b"FRAME_1000021_CENTER   =  399");
    fstr::assign(save.LINES.get_mut(85), b"FRAME_1000022_CENTER   =  399");
    fstr::assign(save.LINES.get_mut(86), b"FRAME_1000023_CENTER   =  399");
    fstr::assign(save.LINES.get_mut(87), b"FRAME_1000024_CENTER   =  399");
    fstr::assign(save.LINES.get_mut(88), b"FRAME_1000025_CENTER   =  399");
    fstr::assign(save.LINES.get_mut(89), b" ");
    fstr::assign(save.LINES.get_mut(90), b" ");
    fstr::assign(save.LINES.get_mut(91), b"FRAME_1000001_CLASS_ID = 1000001");
    fstr::assign(save.LINES.get_mut(92), b"FRAME_1000002_CLASS_ID = 1000002");
    fstr::assign(save.LINES.get_mut(93), b"FRAME_1000003_CLASS_ID = 1000003");
    fstr::assign(save.LINES.get_mut(94), b"FRAME_1000004_CLASS_ID = 1000004");
    fstr::assign(save.LINES.get_mut(95), b"FRAME_1000005_CLASS_ID = 1000005");
    fstr::assign(save.LINES.get_mut(96), b"FRAME_1000006_CLASS_ID = 1000006");
    fstr::assign(save.LINES.get_mut(97), b"FRAME_1000007_CLASS_ID = 1000007");
    fstr::assign(save.LINES.get_mut(98), b"FRAME_1000008_CLASS_ID = 1000008");
    fstr::assign(save.LINES.get_mut(99), b"FRAME_1000009_CLASS_ID = 1000009");
    fstr::assign(save.LINES.get_mut(100), b"FRAME_1000010_CLASS_ID = 1000010");
    fstr::assign(save.LINES.get_mut(101), b"FRAME_1000011_CLASS_ID = 1000011");
    fstr::assign(save.LINES.get_mut(102), b"FRAME_1000012_CLASS_ID = 1000012");
    fstr::assign(save.LINES.get_mut(103), b"FRAME_1000013_CLASS_ID = 1000013");
    fstr::assign(save.LINES.get_mut(104), b"FRAME_1000014_CLASS_ID = 1000014");
    fstr::assign(save.LINES.get_mut(105), b"FRAME_1000015_CLASS_ID = 1000015");
    fstr::assign(save.LINES.get_mut(106), b"FRAME_1000016_CLASS_ID = 1000016");
    fstr::assign(save.LINES.get_mut(107), b"FRAME_1000017_CLASS_ID = 1000017");
    fstr::assign(save.LINES.get_mut(108), b"FRAME_1000018_CLASS_ID = 1000018");
    fstr::assign(save.LINES.get_mut(109), b"FRAME_1000019_CLASS_ID = 1000019");
    fstr::assign(save.LINES.get_mut(110), b"FRAME_1000020_CLASS_ID = 1000020");
    fstr::assign(save.LINES.get_mut(111), b"FRAME_1000021_CLASS_ID = 1000021");
    fstr::assign(save.LINES.get_mut(112), b"FRAME_1000022_CLASS_ID = 1000022");
    fstr::assign(save.LINES.get_mut(113), b"FRAME_1000023_CLASS_ID = 1000023");
    fstr::assign(save.LINES.get_mut(114), b"FRAME_1000024_CLASS_ID = 1000024");
    fstr::assign(save.LINES.get_mut(115), b"FRAME_1000025_CLASS_ID = 1000025");
    fstr::assign(save.LINES.get_mut(116), b" ");
    fstr::assign(save.LINES.get_mut(117), b" ");
    fstr::assign(
        save.LINES.get_mut(118),
        b"FRAME_1000001_NAME     = \'LAPLATA_TOPO\'",
    );
    fstr::assign(
        save.LINES.get_mut(119),
        b"FRAME_1000002_NAME     = \'CANBERRA_TOPO\'",
    );
    fstr::assign(
        save.LINES.get_mut(120),
        b"FRAME_1000003_NAME     = \'URANIA_TOPO\'",
    );
    fstr::assign(
        save.LINES.get_mut(121),
        b"FRAME_1000004_NAME     = \'VALONGO_TOPO\'",
    );
    fstr::assign(
        save.LINES.get_mut(122),
        b"FRAME_1000005_NAME     = \'LASCAMPANAS_TOPO\'",
    );
    fstr::assign(
        save.LINES.get_mut(123),
        b"FRAME_1000006_NAME     = \'YUNNAN_TOPO\'",
    );
    fstr::assign(
        save.LINES.get_mut(124),
        b"FRAME_1000007_NAME     = \'QUITO_TOPO\'",
    );
    fstr::assign(
        save.LINES.get_mut(125),
        b"FRAME_1000008_NAME     = \'TUORLA_TOPO\'",
    );
    fstr::assign(
        save.LINES.get_mut(126),
        b"FRAME_1000009_NAME     = \'GRENOBLE_TOPO\'",
    );
    fstr::assign(
        save.LINES.get_mut(127),
        b"FRAME_1000010_NAME     = \'HAMBURG_TOPO\'",
    );
    fstr::assign(
        save.LINES.get_mut(128),
        b"FRAME_1000011_NAME     = \'MUNICH_TOPO\'",
    );
    fstr::assign(
        save.LINES.get_mut(129),
        b"FRAME_1000012_NAME     = \'KODAIKANAL_TOPO\'",
    );
    fstr::assign(
        save.LINES.get_mut(130),
        b"FRAME_1000013_NAME     = \'DUNSINK_TOPO\'",
    );
    fstr::assign(
        save.LINES.get_mut(131),
        b"FRAME_1000014_NAME     = \'TURIN_TOPO\'",
    );
    fstr::assign(
        save.LINES.get_mut(132),
        b"FRAME_1000015_NAME     = \'HIDA_TOPO\'",
    );
    fstr::assign(
        save.LINES.get_mut(133),
        b"FRAME_1000016_NAME     = \'ARECIBO_TOPO\'",
    );
    fstr::assign(
        save.LINES.get_mut(134),
        b"FRAME_1000017_NAME     = \'PULKOVO_TOPO\'",
    );
    fstr::assign(
        save.LINES.get_mut(135),
        b"FRAME_1000018_NAME     = \'BOYDEN_TOPO\'",
    );
    fstr::assign(
        save.LINES.get_mut(136),
        b"FRAME_1000019_NAME     = \'MADRID_TOPO\'",
    );
    fstr::assign(
        save.LINES.get_mut(137),
        b"FRAME_1000020_NAME     = \'AROSA_TOPO\'",
    );
    fstr::assign(
        save.LINES.get_mut(138),
        b"FRAME_1000021_NAME     = \'GREENWICH_TOPO\'",
    );
    fstr::assign(
        save.LINES.get_mut(139),
        b"FRAME_1000022_NAME     = \'KITTPEAK_TOPO\'",
    );
    fstr::assign(
        save.LINES.get_mut(140),
        b"FRAME_1000023_NAME     = \'GOLDSTONE_TOPO\'",
    );
    fstr::assign(
        save.LINES.get_mut(141),
        b"FRAME_1000024_NAME     = \'PALOMAR_TOPO\'",
    );
    fstr::assign(
        save.LINES.get_mut(142),
        b"FRAME_1000025_NAME     = \'USNO_TOPO\'",
    );
    fstr::assign(save.LINES.get_mut(143), b" ");
    fstr::assign(save.LINES.get_mut(144), b" ");
    fstr::assign(
        save.LINES.get_mut(145),
        b"TKFRAME_1000001_SPEC     = \'ANGLES\'",
    );
    fstr::assign(
        save.LINES.get_mut(146),
        b"TKFRAME_1000002_SPEC     = \'ANGLES\'",
    );
    fstr::assign(
        save.LINES.get_mut(147),
        b"TKFRAME_1000003_SPEC     = \'ANGLES\'",
    );
    fstr::assign(
        save.LINES.get_mut(148),
        b"TKFRAME_1000004_SPEC     = \'ANGLES\'",
    );
    fstr::assign(
        save.LINES.get_mut(149),
        b"TKFRAME_1000005_SPEC     = \'ANGLES\'",
    );
    fstr::assign(
        save.LINES.get_mut(150),
        b"TKFRAME_1000006_SPEC     = \'ANGLES\'",
    );
    fstr::assign(
        save.LINES.get_mut(151),
        b"TKFRAME_1000007_SPEC     = \'ANGLES\'",
    );
    fstr::assign(
        save.LINES.get_mut(152),
        b"TKFRAME_1000008_SPEC     = \'ANGLES\'",
    );
    fstr::assign(
        save.LINES.get_mut(153),
        b"TKFRAME_1000009_SPEC     = \'ANGLES\'",
    );
    fstr::assign(
        save.LINES.get_mut(154),
        b"TKFRAME_1000010_SPEC     = \'ANGLES\'",
    );
    fstr::assign(
        save.LINES.get_mut(155),
        b"TKFRAME_1000011_SPEC     = \'ANGLES\'",
    );
    fstr::assign(
        save.LINES.get_mut(156),
        b"TKFRAME_1000012_SPEC     = \'ANGLES\'",
    );
    fstr::assign(
        save.LINES.get_mut(157),
        b"TKFRAME_1000013_SPEC     = \'ANGLES\'",
    );
    fstr::assign(
        save.LINES.get_mut(158),
        b"TKFRAME_1000014_SPEC     = \'ANGLES\'",
    );
    fstr::assign(
        save.LINES.get_mut(159),
        b"TKFRAME_1000015_SPEC     = \'ANGLES\'",
    );
    fstr::assign(
        save.LINES.get_mut(160),
        b"TKFRAME_1000016_SPEC     = \'ANGLES\'",
    );
    fstr::assign(
        save.LINES.get_mut(161),
        b"TKFRAME_1000017_SPEC     = \'ANGLES\'",
    );
    fstr::assign(
        save.LINES.get_mut(162),
        b"TKFRAME_1000018_SPEC     = \'ANGLES\'",
    );
    fstr::assign(
        save.LINES.get_mut(163),
        b"TKFRAME_1000019_SPEC     = \'ANGLES\'",
    );
    fstr::assign(
        save.LINES.get_mut(164),
        b"TKFRAME_AROSA_TOPO_SPEC  = \'ANGLES\'",
    );
    fstr::assign(
        save.LINES.get_mut(165),
        b"TKFRAME_1000021_SPEC     = \'ANGLES\'",
    );
    fstr::assign(
        save.LINES.get_mut(166),
        b"TKFRAME_1000022_SPEC     = \'ANGLES\'",
    );
    fstr::assign(
        save.LINES.get_mut(167),
        b"TKFRAME_1000023_SPEC     = \'ANGLES\'",
    );
    fstr::assign(
        save.LINES.get_mut(168),
        b"TKFRAME_1000024_SPEC     = \'ANGLES\'",
    );
    fstr::assign(
        save.LINES.get_mut(169),
        b"TKFRAME_1000025_SPEC     = \'ANGLES\'",
    );
    fstr::assign(save.LINES.get_mut(170), b" ");
    fstr::assign(save.LINES.get_mut(171), b" ");
    fstr::assign(
        save.LINES.get_mut(172),
        b"TKFRAME_1000001_RELATIVE = \'IAU_EARTH\'",
    );
    fstr::assign(
        save.LINES.get_mut(173),
        b"TKFRAME_1000002_RELATIVE = \'IAU_EARTH\'",
    );
    fstr::assign(
        save.LINES.get_mut(174),
        b"TKFRAME_1000003_RELATIVE = \'IAU_EARTH\'",
    );
    fstr::assign(
        save.LINES.get_mut(175),
        b"TKFRAME_1000004_RELATIVE = \'IAU_EARTH\'",
    );
    fstr::assign(
        save.LINES.get_mut(176),
        b"TKFRAME_1000005_RELATIVE = \'IAU_EARTH\'",
    );
    fstr::assign(
        save.LINES.get_mut(177),
        b"TKFRAME_1000006_RELATIVE = \'IAU_EARTH\'",
    );
    fstr::assign(
        save.LINES.get_mut(178),
        b"TKFRAME_1000007_RELATIVE = \'IAU_EARTH\'",
    );
    fstr::assign(
        save.LINES.get_mut(179),
        b"TKFRAME_1000008_RELATIVE = \'IAU_EARTH\'",
    );
    fstr::assign(
        save.LINES.get_mut(180),
        b"TKFRAME_1000009_RELATIVE = \'IAU_EARTH\'",
    );
    fstr::assign(
        save.LINES.get_mut(181),
        b"TKFRAME_1000010_RELATIVE = \'IAU_EARTH\'",
    );
    fstr::assign(
        save.LINES.get_mut(182),
        b"TKFRAME_1000011_RELATIVE = \'IAU_EARTH\'",
    );
    fstr::assign(
        save.LINES.get_mut(183),
        b"TKFRAME_1000012_RELATIVE = \'IAU_EARTH\'",
    );
    fstr::assign(
        save.LINES.get_mut(184),
        b"TKFRAME_1000013_RELATIVE = \'IAU_EARTH\'",
    );
    fstr::assign(
        save.LINES.get_mut(185),
        b"TKFRAME_1000014_RELATIVE = \'IAU_EARTH\'",
    );
    fstr::assign(
        save.LINES.get_mut(186),
        b"TKFRAME_1000015_RELATIVE = \'IAU_EARTH\'",
    );
    fstr::assign(
        save.LINES.get_mut(187),
        b"TKFRAME_1000016_RELATIVE = \'IAU_EARTH\'",
    );
    fstr::assign(
        save.LINES.get_mut(188),
        b"TKFRAME_1000017_RELATIVE = \'IAU_EARTH\'",
    );
    fstr::assign(
        save.LINES.get_mut(189),
        b"TKFRAME_1000018_RELATIVE = \'IAU_EARTH\'",
    );
    fstr::assign(
        save.LINES.get_mut(190),
        b"TKFRAME_1000019_RELATIVE = \'IAU_EARTH\'",
    );
    fstr::assign(
        save.LINES.get_mut(191),
        b"TKFRAME_AROSA_TOPO_RELATIVE = \'IAU_EARTH\'",
    );
    fstr::assign(
        save.LINES.get_mut(192),
        b"TKFRAME_1000021_RELATIVE = \'IAU_EARTH\'",
    );
    fstr::assign(
        save.LINES.get_mut(193),
        b"TKFRAME_1000022_RELATIVE = \'IAU_EARTH\'",
    );
    fstr::assign(
        save.LINES.get_mut(194),
        b"TKFRAME_1000023_RELATIVE = \'IAU_EARTH\'",
    );
    fstr::assign(
        save.LINES.get_mut(195),
        b"TKFRAME_1000024_RELATIVE = \'IAU_EARTH\'",
    );
    fstr::assign(
        save.LINES.get_mut(196),
        b"TKFRAME_1000025_RELATIVE = \'IAU_EARTH\'",
    );
    fstr::assign(save.LINES.get_mut(197), b" ");
    fstr::assign(save.LINES.get_mut(198), b" ");
    fstr::assign(
        save.LINES.get_mut(199),
        b"TKFRAME_1000001_AXES     = (       3,        2,   3 )",
    );
    fstr::assign(
        save.LINES.get_mut(200),
        b"TKFRAME_1000002_AXES     = (       3,        2,   3 )",
    );
    fstr::assign(
        save.LINES.get_mut(201),
        b"TKFRAME_1000003_AXES     = (       3,        2,   3 )",
    );
    fstr::assign(
        save.LINES.get_mut(202),
        b"TKFRAME_1000004_AXES     = (       3,        2,   3 )",
    );
    fstr::assign(
        save.LINES.get_mut(203),
        b"TKFRAME_1000005_AXES     = (       3,        2,   3 )",
    );
    fstr::assign(
        save.LINES.get_mut(204),
        b"TKFRAME_1000006_AXES     = (       3,        2,   3 )",
    );
    fstr::assign(
        save.LINES.get_mut(205),
        b"TKFRAME_1000007_AXES     = (       3,        2,   3 )",
    );
    fstr::assign(
        save.LINES.get_mut(206),
        b"TKFRAME_1000008_AXES     = (       3,        2,   3 )",
    );
    fstr::assign(
        save.LINES.get_mut(207),
        b"TKFRAME_1000009_AXES     = (       3,        2,   3 )",
    );
    fstr::assign(
        save.LINES.get_mut(208),
        b"TKFRAME_1000010_AXES     = (       3,        2,   3 )",
    );
    fstr::assign(
        save.LINES.get_mut(209),
        b"TKFRAME_1000011_AXES     = (       3,        2,   3 )",
    );
    fstr::assign(
        save.LINES.get_mut(210),
        b"TKFRAME_1000012_AXES     = (       3,        2,   3 )",
    );
    fstr::assign(
        save.LINES.get_mut(211),
        b"TKFRAME_1000013_AXES     = (       3,        2,   3 )",
    );
    fstr::assign(
        save.LINES.get_mut(212),
        b"TKFRAME_1000014_AXES     = (       3,        2,   3 )",
    );
    fstr::assign(
        save.LINES.get_mut(213),
        b"TKFRAME_1000015_AXES     = (       3,        2,   3 )",
    );
    fstr::assign(
        save.LINES.get_mut(214),
        b"TKFRAME_1000016_AXES     = (       3,        2,   3 )",
    );
    fstr::assign(
        save.LINES.get_mut(215),
        b"TKFRAME_1000017_AXES =     (       3,        2,   3 )",
    );
    fstr::assign(
        save.LINES.get_mut(216),
        b"TKFRAME_BOYDEN_TOPO_AXES = (       3,        2,   3 )",
    );
    fstr::assign(
        save.LINES.get_mut(217),
        b"TKFRAME_1000019_AXES     = (       3,        2,   3 )",
    );
    fstr::assign(
        save.LINES.get_mut(218),
        b"TKFRAME_1000020_AXES     = (       3,        2,   3 )",
    );
    fstr::assign(
        save.LINES.get_mut(219),
        b"TKFRAME_1000021_AXES     = (       3,        2,   3 )",
    );
    fstr::assign(
        save.LINES.get_mut(220),
        b"TKFRAME_1000022_AXES     = (       3,        2,   3 )",
    );
    fstr::assign(
        save.LINES.get_mut(221),
        b"TKFRAME_1000023_AXES     = (       3,        2,   3 )",
    );
    fstr::assign(
        save.LINES.get_mut(222),
        b"TKFRAME_1000024_AXES     = (       3,        2,   3 )",
    );
    fstr::assign(
        save.LINES.get_mut(223),
        b"TKFRAME_1000025_AXES     = (       3,        2,   3 )",
    );
    fstr::assign(save.LINES.get_mut(224), b" ");
    fstr::assign(save.LINES.get_mut(225), b" ");
    fstr::assign(
        save.LINES.get_mut(226),
        b"TKFRAME_1000001_UNITS    = \'DEGREES\'",
    );
    fstr::assign(
        save.LINES.get_mut(227),
        b"TKFRAME_1000002_UNITS    = \'DEGREES\'",
    );
    fstr::assign(
        save.LINES.get_mut(228),
        b"TKFRAME_1000003_UNITS    = \'DEGREES\'",
    );
    fstr::assign(
        save.LINES.get_mut(229),
        b"TKFRAME_1000004_UNITS    = \'DEGREES\'",
    );
    fstr::assign(
        save.LINES.get_mut(230),
        b"TKFRAME_1000005_UNITS    = \'DEGREES\'",
    );
    fstr::assign(
        save.LINES.get_mut(231),
        b"TKFRAME_1000006_UNITS    = \'DEGREES\'",
    );
    fstr::assign(
        save.LINES.get_mut(232),
        b"TKFRAME_1000007_UNITS    = \'DEGREES\'",
    );
    fstr::assign(
        save.LINES.get_mut(233),
        b"TKFRAME_1000008_UNITS    = \'DEGREES\'",
    );
    fstr::assign(
        save.LINES.get_mut(234),
        b"TKFRAME_1000009_UNITS    = \'DEGREES\'",
    );
    fstr::assign(
        save.LINES.get_mut(235),
        b"TKFRAME_1000010_UNITS    = \'DEGREES\'",
    );
    fstr::assign(
        save.LINES.get_mut(236),
        b"TKFRAME_1000011_UNITS    = \'DEGREES\'",
    );
    fstr::assign(
        save.LINES.get_mut(237),
        b"TKFRAME_1000012_UNITS    = \'DEGREES\'",
    );
    fstr::assign(
        save.LINES.get_mut(238),
        b"TKFRAME_1000013_UNITS    = \'DEGREES\'",
    );
    fstr::assign(
        save.LINES.get_mut(239),
        b"TKFRAME_1000014_UNITS    = \'DEGREES\'",
    );
    fstr::assign(
        save.LINES.get_mut(240),
        b"TKFRAME_1000015_UNITS    = \'DEGREES\'",
    );
    fstr::assign(
        save.LINES.get_mut(241),
        b"TKFRAME_1000016_UNITS    = \'DEGREES\'",
    );
    fstr::assign(
        save.LINES.get_mut(242),
        b"TKFRAME_1000017_UNITS    = \'DEGREES\'",
    );
    fstr::assign(
        save.LINES.get_mut(243),
        b"TKFRAME_BOYDEN_TOPO_UNITS = \'DEGREES\'",
    );
    fstr::assign(
        save.LINES.get_mut(244),
        b"TKFRAME_1000019_UNITS    = \'DEGREES\'",
    );
    fstr::assign(
        save.LINES.get_mut(245),
        b"TKFRAME_1000020_UNITS    = \'DEGREES\'",
    );
    fstr::assign(
        save.LINES.get_mut(246),
        b"TKFRAME_1000021_UNITS    = \'DEGREES\'",
    );
    fstr::assign(
        save.LINES.get_mut(247),
        b"TKFRAME_1000022_UNITS    = \'DEGREES\'",
    );
    fstr::assign(
        save.LINES.get_mut(248),
        b"TKFRAME_1000023_UNITS    = \'DEGREES\'",
    );
    fstr::assign(
        save.LINES.get_mut(249),
        b"TKFRAME_1000024_UNITS    = \'DEGREES\'",
    );
    fstr::assign(
        save.LINES.get_mut(250),
        b"TKFRAME_1000025_UNITS    = \'DEGREES\'",
    );
    fstr::assign(save.LINES.get_mut(251), b" ");
    fstr::assign(save.LINES.get_mut(252), b" ");
    fstr::assign(
        save.LINES.get_mut(253),
        b"TKFRAME_1000001_ANGLES   = ( 57.9, -124.9, 180 )",
    );
    fstr::assign(
        save.LINES.get_mut(254),
        b"TKFRAME_1000002_ANGLES   = ( -149.0, -125.3, 180 )",
    );
    fstr::assign(
        save.LINES.get_mut(255),
        b"TKFRAME_1000003_ANGLES   = ( -16.4, -41.8, 180 )",
    );
    fstr::assign(
        save.LINES.get_mut(256),
        b"TKFRAME_1000004_ANGLES   = ( 43.2, -112.9, 180 )",
    );
    fstr::assign(
        save.LINES.get_mut(257),
        b"TKFRAME_1000005_ANGLES   = ( 70.7, -123.5, 180 )",
    );
    fstr::assign(
        save.LINES.get_mut(258),
        b"TKFRAME_1000006_ANGLES   = ( -102.8, -65.0, 180 )",
    );
    fstr::assign(
        save.LINES.get_mut(259),
        b"TKFRAME_1000007_ANGLES   = ( 78.5, -90.2, 180 )",
    );
    fstr::assign(
        save.LINES.get_mut(260),
        b"TKFRAME_1000008_ANGLES   = ( -22.4, -29.6, 180 )",
    );
    fstr::assign(
        save.LINES.get_mut(261),
        b"TKFRAME_1000009_ANGLES   = ( -05.9, -45.4, 180 )",
    );
    fstr::assign(
        save.LINES.get_mut(262),
        b"TKFRAME_1000010_ANGLES   = ( -10.2, -36.5, 180 )",
    );
    fstr::assign(
        save.LINES.get_mut(263),
        b"TKFRAME_1000011_ANGLES   = ( -11.6, -41.9, 180 )",
    );
    fstr::assign(
        save.LINES.get_mut(264),
        b"TKFRAME_1000012_ANGLES   = ( -77.5, -79.8, 180 )",
    );
    fstr::assign(
        save.LINES.get_mut(265),
        b"TKFRAME_1000013_ANGLES   = ( 06.3, -36.6, 180 )",
    );
    fstr::assign(
        save.LINES.get_mut(266),
        b"TKFRAME_1000014_ANGLES   = ( -07.8, -45.0, 180 )",
    );
    fstr::assign(
        save.LINES.get_mut(267),
        b"TKFRAME_1000015_ANGLES   = ( -137.6, -53.7, 180 )",
    );
    fstr::assign(
        save.LINES.get_mut(268),
        b"TKFRAME_1000016_ANGLES   = ( 66.8, -71.6, 180 )",
    );
    fstr::assign(
        save.LINES.get_mut(269),
        b"TKFRAME_1000017_ANGLES   = ( -42.5, -46.3, 180 )",
    );
    fstr::assign(
        save.LINES.get_mut(270),
        b"TKFRAME_1000018_ANGLES   = ( -26.6, -119.0, 180 )",
    );
    fstr::assign(
        save.LINES.get_mut(271),
        b"TKFRAME_1000019_ANGLES   = ( 03.7, -49.6, 180 )",
    );
    fstr::assign(
        save.LINES.get_mut(272),
        b"TKFRAME_1000020_ANGLES   = ( -09.7, -43.2, 180 )",
    );
    fstr::assign(
        save.LINES.get_mut(273),
        b"TKFRAME_1000021_ANGLES   = ( -00.1, -37.8, 180 )",
    );
    fstr::assign(
        save.LINES.get_mut(274),
        b"TKFRAME_1000022_ANGLES   = ( 111.6, -58.0, 180 )",
    );
    fstr::assign(
        save.LINES.get_mut(275),
        b"TKFRAME_1000023_ANGLES   = ( 116.8, -54.6, 180 )",
    );
    fstr::assign(
        save.LINES.get_mut(276),
        b"TKFRAME_1000024_ANGLES   = ( 116.8, -56.7, 180 )",
    );
    fstr::assign(
        save.LINES.get_mut(277),
        b"TKFRAME_1000025_ANGLES   = ( 77.1, -51.1, 180 )",
    );

    testutil::KILFIL(b"topocentrc.frm", ctx)?;
    testutil::TSTTXT(b"topocentrc.frm", save.LINES.as_arg(), 277, true, true, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ANGLE1[1] = (57.9 * spicelib::RPD(ctx));
    save.ANGLE1[2] = -(149.0 * spicelib::RPD(ctx));
    save.ANGLE1[3] = -(16.4 * spicelib::RPD(ctx));
    save.ANGLE1[4] = (43.2 * spicelib::RPD(ctx));
    save.ANGLE1[5] = (70.7 * spicelib::RPD(ctx));
    save.ANGLE1[6] = -(102.8 * spicelib::RPD(ctx));
    save.ANGLE1[7] = (78.5 * spicelib::RPD(ctx));
    save.ANGLE1[8] = -(22.4 * spicelib::RPD(ctx));
    save.ANGLE1[9] = -(5.9 * spicelib::RPD(ctx));
    save.ANGLE1[10] = -(10.2 * spicelib::RPD(ctx));
    save.ANGLE1[11] = -(11.6 * spicelib::RPD(ctx));
    save.ANGLE1[12] = -(77.5 * spicelib::RPD(ctx));
    save.ANGLE1[13] = (6.3 * spicelib::RPD(ctx));
    save.ANGLE1[14] = -(7.8 * spicelib::RPD(ctx));
    save.ANGLE1[15] = -(137.6 * spicelib::RPD(ctx));
    save.ANGLE1[16] = (66.8 * spicelib::RPD(ctx));
    save.ANGLE1[17] = -(42.5 * spicelib::RPD(ctx));
    save.ANGLE1[18] = -(26.6 * spicelib::RPD(ctx));
    save.ANGLE1[19] = (3.7 * spicelib::RPD(ctx));
    save.ANGLE1[20] = -(9.7 * spicelib::RPD(ctx));
    save.ANGLE1[21] = -(0.1 * spicelib::RPD(ctx));
    save.ANGLE1[22] = (111.6 * spicelib::RPD(ctx));
    save.ANGLE1[23] = (116.8 * spicelib::RPD(ctx));
    save.ANGLE1[24] = (116.8 * spicelib::RPD(ctx));
    save.ANGLE1[25] = (77.1 * spicelib::RPD(ctx));

    save.ANGLE2[1] = -(124.9 * spicelib::RPD(ctx));
    save.ANGLE2[2] = -(125.3 * spicelib::RPD(ctx));
    save.ANGLE2[3] = -(41.8 * spicelib::RPD(ctx));
    save.ANGLE2[4] = -(112.9 * spicelib::RPD(ctx));
    save.ANGLE2[5] = -(123.5 * spicelib::RPD(ctx));
    save.ANGLE2[6] = -(65.0 * spicelib::RPD(ctx));
    save.ANGLE2[7] = -(90.2 * spicelib::RPD(ctx));
    save.ANGLE2[8] = -(29.6 * spicelib::RPD(ctx));
    save.ANGLE2[9] = -(45.4 * spicelib::RPD(ctx));
    save.ANGLE2[10] = -(36.5 * spicelib::RPD(ctx));
    save.ANGLE2[11] = -(41.9 * spicelib::RPD(ctx));
    save.ANGLE2[12] = -(79.8 * spicelib::RPD(ctx));
    save.ANGLE2[13] = -(36.6 * spicelib::RPD(ctx));
    save.ANGLE2[14] = -(45.0 * spicelib::RPD(ctx));
    save.ANGLE2[15] = -(53.7 * spicelib::RPD(ctx));
    save.ANGLE2[16] = -(71.6 * spicelib::RPD(ctx));
    save.ANGLE2[17] = -(46.3 * spicelib::RPD(ctx));
    save.ANGLE2[18] = -(119.0 * spicelib::RPD(ctx));
    save.ANGLE2[19] = -(49.6 * spicelib::RPD(ctx));
    save.ANGLE2[20] = -(43.2 * spicelib::RPD(ctx));
    save.ANGLE2[21] = -(37.8 * spicelib::RPD(ctx));
    save.ANGLE2[22] = -(58.0 * spicelib::RPD(ctx));
    save.ANGLE2[23] = -(54.6 * spicelib::RPD(ctx));
    save.ANGLE2[24] = -(56.7 * spicelib::RPD(ctx));
    save.ANGLE2[25] = -(51.1 * spicelib::RPD(ctx));

    for I in 1..=25 {
        save.ANGLE3[I] = spicelib::PI(ctx);
    }

    for I in 1..=25 {
        spicelib::EUL2M(
            save.ANGLE1[I],
            save.ANGLE2[I],
            save.ANGLE3[I],
            3,
            2,
            3,
            save.R.subarray_mut([1, 1, I]),
            ctx,
        )?;
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.J = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            for I in 1..=25 {
                save.ID = (1000000 + I);
                spicelib::TKFRAM(
                    save.ID,
                    save.ROT.as_slice_mut(),
                    &mut save.FRAME,
                    &mut save.FOUND,
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;
                testutil::CHCKSI(b"FRAME", save.FRAME, b"=", 10013, 0, OK, ctx)?;
                testutil::CHCKAD(
                    b"ROT",
                    save.ROT.as_slice(),
                    b"~",
                    save.R.subarray([1, 1, I]),
                    9,
                    0.0000000000001,
                    OK,
                    ctx,
                )?;
            }
            save.J += m3__;
        }
    }

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Make sure we can get the right answer if we ask for the same frame several times in a row. ", ctx)?;

    for I in 1..=3 {
        spicelib::TKFRAM(
            1000003,
            save.ROT.as_slice_mut(),
            &mut save.FRAME,
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(b"FRAME", save.FRAME, b"=", 10013, 0, OK, ctx)?;
        testutil::CHCKAD(
            b"ROT",
            save.ROT.as_slice(),
            b"~",
            save.R.subarray([1, 1, 3]),
            9,
            0.0000000000001,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Make sure we don\'t overflow the watcher system if we define a very large number of frames.", ctx)?;

    //
    // Fetch the number of POOL variables.
    //
    spicelib::SZPOOL(b"MAXVAR", &mut save.MAXVAR, &mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"SZPOOL FOUND", save.FOUND, true, OK, ctx)?;

    save.NLINES = 17;

    fstr::assign(save.LINES.get_mut(1), b"FRAME_#_NAME        = \'TKF_#\'");
    fstr::assign(save.LINES.get_mut(2), b"FRAME_TKF_#         = #");
    fstr::assign(save.LINES.get_mut(3), b"FRAME_#_CLASS       = 4");
    fstr::assign(save.LINES.get_mut(4), b"FRAME_#_CENTER      = 9");
    fstr::assign(save.LINES.get_mut(5), b"FRAME_#_CLASS_ID    = #");
    fstr::assign(save.LINES.get_mut(6), b"TKFRAME_#_SPEC      = \'MATRIX\'");
    fstr::assign(save.LINES.get_mut(7), b"TKFRAME_#_RELATIVE  = \'J2000\'");
    fstr::assign(save.LINES.get_mut(8), b" ");
    fstr::assign(save.LINES.get_mut(9), b"TKFRAME_#_MATRIX    = ( 0.48");
    fstr::assign(save.LINES.get_mut(10), b" 0.60");
    fstr::assign(save.LINES.get_mut(11), b" 0.64");
    fstr::assign(save.LINES.get_mut(12), b"-0.8");
    fstr::assign(save.LINES.get_mut(13), b" 0.0");
    fstr::assign(save.LINES.get_mut(14), b" 0.6");
    fstr::assign(save.LINES.get_mut(15), b" 0.36");
    fstr::assign(save.LINES.get_mut(16), b"-0.80");
    fstr::assign(save.LINES.get_mut(17), b" 0.48 )");

    //
    // We can fit up to MAXVAR/8 TK frames in the POOL before
    // we run out POOL variables.
    //
    for I in 1..=(save.MAXVAR / 8) {
        save.ID = (5000000 + I);
        //
        // Create a frame definition by filling the frame
        // ID into the template.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NLINES;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                spicelib::REPMI(
                    &save.LINES[save.J],
                    b"#",
                    save.ID,
                    &mut save.BUFFER[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(
                    &save.BUFFER[save.J].to_vec(),
                    b"#",
                    save.ID,
                    &mut save.BUFFER[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.J += m3__;
            }
        }

        //
        // Load the frame specification into the kernel pool.
        //
        spicelib::LMPOOL(save.BUFFER.as_arg(), save.NLINES, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Look up frame data for the current frame.
        //
        spicelib::TKFRAM(
            save.ID,
            save.ROT.as_slice_mut(),
            &mut save.FRAME,
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (intrinsics::MOD(I, BUFSIZ) == 0) {
            //
            // Clear the kernel pool so we don't run out of
            // room for kernel variables before we run out
            // of watchers.
            //
            spicelib::CLPOOL(ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Verify that buffered frame data, for the number of frames equal to the TKFRAM buffer size, are updated when watchers indicate a kernel pool update.", ctx)?;

    //
    // Clear the kernel pool, then insert definitions
    // of BUFSIZ TK frames into the pool.
    //
    spicelib::CLPOOL(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NLINES = 11;

    fstr::assign(save.LINES.get_mut(1), b"FRAME_#_NAME        = \'TKF_#\'");
    fstr::assign(save.LINES.get_mut(2), b"FRAME_TKF_#         = #");
    fstr::assign(save.LINES.get_mut(3), b"FRAME_#_CLASS       = 4");
    fstr::assign(save.LINES.get_mut(4), b"FRAME_#_CENTER      = 9");
    fstr::assign(save.LINES.get_mut(5), b"FRAME_#_CLASS_ID    = #");
    fstr::assign(save.LINES.get_mut(6), b"TKFRAME_#_SPEC      = \'ANGLES\'");
    fstr::assign(save.LINES.get_mut(7), b"TKFRAME_#_RELATIVE  = \'J2000\'");
    fstr::assign(save.LINES.get_mut(8), b" ");
    fstr::assign(save.LINES.get_mut(9), b"TKFRAME_#_AXES   = ( 3, 1, 3 )");
    fstr::assign(save.LINES.get_mut(10), b"TKFRAME_#_UNITS  = \'DEGREES\'");
    fstr::assign(save.LINES.get_mut(11), b"TKFRAME_#_ANGLES = ( @ @ @ )");

    for I in 1..=BUFSIZ {
        save.ID = (5000000 + I);
        //
        // Create a frame definition by filling the frame
        // ID into the template.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NLINES;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                spicelib::REPMI(
                    &save.LINES[save.J],
                    b"#",
                    save.ID,
                    &mut save.BUFFER[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(
                    &save.BUFFER[save.J].to_vec(),
                    b"#",
                    save.ID,
                    &mut save.BUFFER[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.J += m3__;
            }
        }

        //
        // Give each frame a different orientation.
        //
        save.ANGLE1[I] = (((100 + I) as f64) / 100.0);
        save.ANGLE2[I] = (((200 + I) as f64) / 100.0);
        save.ANGLE3[I] = (((300 + I) as f64) / 100.0);

        save.J = 11;
        spicelib::REPMD(
            &save.BUFFER[save.J].to_vec(),
            b"@",
            save.ANGLE1[I],
            14,
            &mut save.BUFFER[save.J],
            ctx,
        );
        spicelib::REPMD(
            &save.BUFFER[save.J].to_vec(),
            b"@",
            save.ANGLE2[I],
            14,
            &mut save.BUFFER[save.J],
            ctx,
        );
        spicelib::REPMD(
            &save.BUFFER[save.J].to_vec(),
            b"@",
            save.ANGLE3[I],
            14,
            &mut save.BUFFER[save.J],
            ctx,
        );
        //
        // Load the frame specification into the kernel pool.
        //
        spicelib::LMPOOL(save.BUFFER.as_arg(), save.NLINES, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Verify that the expected frame definitions are
    // available.
    //
    for I in 1..=BUFSIZ {
        {
            let m1__: i32 = 1;
            let m2__: i32 = 2;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                //
                // Look up frame data for the current frame.
                //
                save.ID = (5000000 + I);
                spicelib::TKFRAM(
                    save.ID,
                    save.ROT.as_slice_mut(),
                    &mut save.FRAME,
                    &mut save.FOUND,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Create the expected matrix.
                //
                spicelib::EUL2M(
                    (save.ANGLE1[I] * spicelib::RPD(ctx)),
                    (save.ANGLE2[I] * spicelib::RPD(ctx)),
                    (save.ANGLE3[I] * spicelib::RPD(ctx)),
                    3,
                    1,
                    3,
                    save.XROT.as_slice_mut(),
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                fstr::assign(&mut save.QNAME, b"ROT(#,#) (pass 1)");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.J, &mut save.QNAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    &save.QNAME,
                    save.ROT.as_slice(),
                    b"~",
                    save.XROT.as_slice(),
                    9,
                    0.000000000001,
                    OK,
                    ctx,
                )?;

                save.J += m3__;
            }
        }
    }

    //
    // Now update all of the frame definitions.
    //
    for I in 1..=BUFSIZ {
        save.ID = (5000000 + I);
        //
        // Create a frame definition by filling the frame
        // ID into the template.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NLINES;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                spicelib::REPMI(
                    &save.LINES[save.J],
                    b"#",
                    save.ID,
                    &mut save.BUFFER[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(
                    &save.BUFFER[save.J].to_vec(),
                    b"#",
                    save.ID,
                    &mut save.BUFFER[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.J += m3__;
            }
        }

        //
        // Give each frame a different orientation.
        //
        save.ANGLE1[I] = (((100 + I) as f64) / 100.0);
        save.ANGLE2[I] = (((400 + I) as f64) / 100.0);
        save.ANGLE3[I] = (((800 + I) as f64) / 100.0);

        save.J = 11;
        spicelib::REPMD(
            &save.BUFFER[save.J].to_vec(),
            b"@",
            save.ANGLE1[I],
            14,
            &mut save.BUFFER[save.J],
            ctx,
        );
        spicelib::REPMD(
            &save.BUFFER[save.J].to_vec(),
            b"@",
            save.ANGLE2[I],
            14,
            &mut save.BUFFER[save.J],
            ctx,
        );
        spicelib::REPMD(
            &save.BUFFER[save.J].to_vec(),
            b"@",
            save.ANGLE3[I],
            14,
            &mut save.BUFFER[save.J],
            ctx,
        );
        //
        // Load the frame specification into the kernel pool.
        //
        spicelib::LMPOOL(save.BUFFER.as_arg(), save.NLINES, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Verify that the expected frame definitions are
    // available. At this point, we're relying on the
    // watchers set in TKFRAM to indicate the need to
    // fetch new data from the kernel pool.
    //
    for I in 1..=BUFSIZ {
        {
            let m1__: i32 = 1;
            let m2__: i32 = 2;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                //
                // Look up frame data for the current frame.
                //
                save.ID = (5000000 + I);
                spicelib::TKFRAM(
                    save.ID,
                    save.ROT.as_slice_mut(),
                    &mut save.FRAME,
                    &mut save.FOUND,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Create the expected matrix.
                //
                spicelib::EUL2M(
                    (save.ANGLE1[I] * spicelib::RPD(ctx)),
                    (save.ANGLE2[I] * spicelib::RPD(ctx)),
                    (save.ANGLE3[I] * spicelib::RPD(ctx)),
                    3,
                    1,
                    3,
                    save.XROT.as_slice_mut(),
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                fstr::assign(&mut save.QNAME, b"ROT(#,#) (pass 2)");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.J, &mut save.QNAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    &save.QNAME,
                    save.ROT.as_slice(),
                    b"~",
                    save.XROT.as_slice(),
                    9,
                    0.000000000001,
                    OK,
                    ctx,
                )?;

                save.J += m3__;
            }
        }
    }

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Repeat watcher test for the number of frames greater than the TKFRAM buffer size",
        ctx,
    )?;
    //
    // Clear the kernel pool, then insert definitions
    // of MAXSIZ TK frames into the pool.
    //
    spicelib::CLPOOL(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NLINES = 11;

    fstr::assign(save.LINES.get_mut(1), b"FRAME_#_NAME        = \'TKF_#\'");
    fstr::assign(save.LINES.get_mut(2), b"FRAME_TKF_#         = #");
    fstr::assign(save.LINES.get_mut(3), b"FRAME_#_CLASS       = 4");
    fstr::assign(save.LINES.get_mut(4), b"FRAME_#_CENTER      = 9");
    fstr::assign(save.LINES.get_mut(5), b"FRAME_#_CLASS_ID    = #");
    fstr::assign(save.LINES.get_mut(6), b"TKFRAME_#_SPEC      = \'ANGLES\'");
    fstr::assign(save.LINES.get_mut(7), b"TKFRAME_#_RELATIVE  = \'J2000\'");
    fstr::assign(save.LINES.get_mut(8), b" ");
    fstr::assign(save.LINES.get_mut(9), b"TKFRAME_#_AXES   = ( 3, 1, 3 )");
    fstr::assign(save.LINES.get_mut(10), b"TKFRAME_#_UNITS  = \'DEGREES\'");
    fstr::assign(save.LINES.get_mut(11), b"TKFRAME_#_ANGLES = ( @ @ @ )");

    for I in 1..=MAXSIZ {
        save.ID = (5000000 + I);
        //
        // Create a frame definition by filling the frame
        // ID into the template.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NLINES;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                spicelib::REPMI(
                    &save.LINES[save.J],
                    b"#",
                    save.ID,
                    &mut save.BUFFER[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(
                    &save.BUFFER[save.J].to_vec(),
                    b"#",
                    save.ID,
                    &mut save.BUFFER[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.J += m3__;
            }
        }

        //
        // Give each frame a different orientation.
        //
        save.ANGLE1[I] = (((100 + I) as f64) / 100.0);
        save.ANGLE2[I] = (((200 + I) as f64) / 100.0);
        save.ANGLE3[I] = (((300 + I) as f64) / 100.0);

        save.J = 11;
        spicelib::REPMD(
            &save.BUFFER[save.J].to_vec(),
            b"@",
            save.ANGLE1[I],
            14,
            &mut save.BUFFER[save.J],
            ctx,
        );
        spicelib::REPMD(
            &save.BUFFER[save.J].to_vec(),
            b"@",
            save.ANGLE2[I],
            14,
            &mut save.BUFFER[save.J],
            ctx,
        );
        spicelib::REPMD(
            &save.BUFFER[save.J].to_vec(),
            b"@",
            save.ANGLE3[I],
            14,
            &mut save.BUFFER[save.J],
            ctx,
        );
        //
        // Load the frame specification into the kernel pool.
        //
        spicelib::LMPOOL(save.BUFFER.as_arg(), save.NLINES, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Verify that the expected frame definitions are
    // available.
    //
    for I in 1..=MAXSIZ {
        {
            let m1__: i32 = 1;
            let m2__: i32 = 2;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                //
                // Look up frame data for the current frame.
                //
                save.ID = (5000000 + I);
                spicelib::TKFRAM(
                    save.ID,
                    save.ROT.as_slice_mut(),
                    &mut save.FRAME,
                    &mut save.FOUND,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                fstr::assign(&mut save.QNAME, b"ROT(#,#) (pass 1)");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.J, &mut save.QNAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSL(
                    &fstr::concat(&save.QNAME, b" FOUND"),
                    save.FOUND,
                    true,
                    OK,
                    ctx,
                )?;

                if *OK {
                    //
                    // Create the expected matrix.
                    //
                    spicelib::EUL2M(
                        (save.ANGLE1[I] * spicelib::RPD(ctx)),
                        (save.ANGLE2[I] * spicelib::RPD(ctx)),
                        (save.ANGLE3[I] * spicelib::RPD(ctx)),
                        3,
                        1,
                        3,
                        save.XROT.as_slice_mut(),
                        ctx,
                    )?;

                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::CHCKAD(
                        &save.QNAME,
                        save.ROT.as_slice(),
                        b"~",
                        save.XROT.as_slice(),
                        9,
                        0.000000000001,
                        OK,
                        ctx,
                    )?;
                }

                save.J += m3__;
            }
        }
    }

    //
    // Now update all of the frame definitions.
    //
    for I in 1..=MAXSIZ {
        save.ID = (5000000 + I);
        //
        // Create a frame definition by filling the frame
        // ID into the template.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NLINES;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                spicelib::REPMI(
                    &save.LINES[save.J],
                    b"#",
                    save.ID,
                    &mut save.BUFFER[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(
                    &save.BUFFER[save.J].to_vec(),
                    b"#",
                    save.ID,
                    &mut save.BUFFER[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.J += m3__;
            }
        }

        //
        // Give each frame a different orientation.
        //
        save.ANGLE1[I] = (((100 + I) as f64) / 100.0);
        save.ANGLE2[I] = (((400 + I) as f64) / 100.0);
        save.ANGLE3[I] = (((800 + I) as f64) / 100.0);

        save.J = 11;
        spicelib::REPMD(
            &save.BUFFER[save.J].to_vec(),
            b"@",
            save.ANGLE1[I],
            14,
            &mut save.BUFFER[save.J],
            ctx,
        );
        spicelib::REPMD(
            &save.BUFFER[save.J].to_vec(),
            b"@",
            save.ANGLE2[I],
            14,
            &mut save.BUFFER[save.J],
            ctx,
        );
        spicelib::REPMD(
            &save.BUFFER[save.J].to_vec(),
            b"@",
            save.ANGLE3[I],
            14,
            &mut save.BUFFER[save.J],
            ctx,
        );
        //
        // Load the frame specification into the kernel pool.
        //
        spicelib::LMPOOL(save.BUFFER.as_arg(), save.NLINES, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Verify that the expected frame definitions are
    // available. At this point, we're relying on the
    // watchers set in TKFRAM to indicate the need to
    // fetch new data from the kernel pool.
    //
    for I in 1..=MAXSIZ {
        {
            let m1__: i32 = 1;
            let m2__: i32 = 2;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                //
                // Look up frame data for the current frame.
                //
                save.ID = (5000000 + I);
                spicelib::TKFRAM(
                    save.ID,
                    save.ROT.as_slice_mut(),
                    &mut save.FRAME,
                    &mut save.FOUND,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Create the expected matrix.
                //
                spicelib::EUL2M(
                    (save.ANGLE1[I] * spicelib::RPD(ctx)),
                    (save.ANGLE2[I] * spicelib::RPD(ctx)),
                    (save.ANGLE3[I] * spicelib::RPD(ctx)),
                    3,
                    1,
                    3,
                    save.XROT.as_slice_mut(),
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                fstr::assign(&mut save.QNAME, b"ROT(#,#) (pass 2)");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.J, &mut save.QNAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    &save.QNAME,
                    save.ROT.as_slice(),
                    b"~",
                    save.XROT.as_slice(),
                    9,
                    0.000000000001,
                    OK,
                    ctx,
                )?;

                save.J += m3__;
            }
        }
    }

    //
    // That's all.  Clean up the kernel files we created.
    //
    testutil::KILFIL(b"phoenix.tk", ctx)?;
    testutil::KILFIL(b"phoenix.prt", ctx)?;
    testutil::KILFIL(b"topocentrc.frm", ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
