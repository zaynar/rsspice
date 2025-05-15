//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const WDSIZE: i32 = 32;
const TXTSIZ: i32 = 80;
const LNSIZE: i32 = 240;

//$Procedure      F_FRAMEX ( Family of tests for FRAMEX)
pub fn F_FRAMEX(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut FRNAME = [b' '; WDSIZE as usize];
    let mut NAME = [b' '; WDSIZE as usize];
    let mut ENAME = [b' '; WDSIZE as usize];
    let mut NAMES = ActualCharArray::new(WDSIZE, 1..=40);
    let mut TEXT = ActualCharArray::new(TXTSIZ, 1..=20);
    let mut CODE: i32 = 0;
    let mut CODES = StackArray::<i32, 40>::new(1..=40);
    let mut ECODE: i32 = 0;
    let mut FRCODE: i32 = 0;
    let mut CENT: i32 = 0;
    let mut CLASS: i32 = 0;
    let mut CLSSID: i32 = 0;
    let mut FOUND: bool = false;
    let mut FOUND1: bool = false;
    let mut FOUND2: bool = false;
    let mut FRAME: i32 = 0;
    let mut IDCODE: i32 = 0;
    let mut SUNIT: i32 = 0;
    let mut ISTRUE: bool = false;
    let mut MESSGE = [b' '; LNSIZE as usize];
    let mut BEGDATC = [b' '; 16 as usize];

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
    testutil::TOPEN(b"F_FRAMEX", ctx)?;

    ISTRUE = true;

    testutil::TCASE(
        b"Test the exception that should be signalled if FRAMEX is called directly. ",
        ctx,
    )?;

    spicelib::FRAMEX(&ENAME, &FRNAME, FRCODE, CENT, CLASS, CLSSID, FOUND, ctx)?;

    testutil::CHCKXC(ISTRUE, b"SPICE(BOGUSENTRY)", OK, ctx)?;

    testutil::TCASE(
        b"Perform check to make sure that every inertial frame is recognized. ",
        ctx,
    )?;
    //
    // Note that the upper bound of the loop should be increased
    // if the number of inertial frames increases.
    //
    for I in 1..=18 {
        FRCODE = I;

        spicelib::FRINFO(FRCODE, &mut CENT, &mut CLASS, &mut CLSSID, &mut FOUND, ctx)?;

        fstr::assign(
            &mut MESSGE,
            b"The case failed when the inertial frame code supplied to FRINFO was #. ",
        );

        testutil::TSTMSG(b"#", &MESSGE, ctx);
        testutil::TSTMSI(FRCODE, ctx);

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(b"FOUND", FOUND, ISTRUE, OK, ctx)?;
        testutil::CHCKSI(b"CENTER", CENT, b"=", 0, 0, OK, ctx)?;
        testutil::CHCKSI(b"CLASS", CLASS, b"=", 1, 0, OK, ctx)?;
        testutil::CHCKSI(b"CLSSID", CLSSID, b"=", I, 0, OK, ctx)?;
    }

    testutil::TCASE(
        b"Check to make sure that the recognized non-inertial frames are all type PCK. ",
        ctx,
    )?;

    for I in 1..=79 {
        FRCODE = (10000 + I);
        spicelib::FRINFO(FRCODE, &mut CENT, &mut CLASS, &mut CLSSID, &mut FOUND, ctx)?;

        fstr::assign(
            &mut MESSGE,
            b"The case failed when the non-inertial frame code supplied to FRINFO was #. ",
        );
        testutil::TSTMSG(b"#", &MESSGE, ctx);
        testutil::TSTMSI(FRCODE, ctx);

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(b"FOUND", FOUND, ISTRUE, OK, ctx)?;
        testutil::CHCKSI(b"CLASS", CLASS, b"=", 2, 0, OK, ctx)?;
    }

    testutil::TCASE(
        b"Make sure that the id-code to name to id-code path works.",
        ctx,
    )?;

    for I in 1..=18 {
        FRCODE = I;

        fstr::assign(&mut NAME, b" ");
        spicelib::FRMNAM(FRCODE, &mut NAME, ctx)?;
        spicelib::NAMFRM(&NAME, &mut CODE, ctx)?;

        fstr::assign(&mut MESSGE, b"The case failed when the frame code was #. ");

        testutil::TSTMSG(b"#", &MESSGE, ctx);
        testutil::TSTMSI(FRCODE, ctx);

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(b"FRCODE", FRCODE, b"=", CODE, 0, OK, ctx)?;
        testutil::CHCKSC(b"NAME", &NAME, b"!=", b" ", OK, ctx)?;
    }

    for I in 1..=79 {
        FRCODE = (10000 + I);

        fstr::assign(&mut NAME, b" ");
        spicelib::FRMNAM(FRCODE, &mut NAME, ctx)?;
        spicelib::NAMFRM(&NAME, &mut CODE, ctx)?;

        fstr::assign(&mut MESSGE, b"The case failed when the frame code was #. ");

        testutil::TSTMSG(b"#", &MESSGE, ctx);
        testutil::TSTMSI(FRCODE, ctx);

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(b"FRCODE", FRCODE, b"=", CODE, 0, OK, ctx)?;
        testutil::CHCKSC(b"NAME", &NAME, b"!=", b" ", OK, ctx)?;
    }

    testutil::TCASE(
        b"Check to make sure we can get data from the kernel pool.",
        ctx,
    )?;

    testutil::KILFIL(b"framedat.cnk", ctx)?;
    spicelib::TXTOPN(b"framedat.cnk", &mut SUNIT, ctx)?;

    testutil::BEGDAT(&mut BEGDATC);
    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::ListDirectedWriter::new(ctx.io_unit(SUNIT)?, None)?;
        writer.start()?;
        writer.write_str(&BEGDATC)?;
        writer.finish()?;
    }
    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::ListDirectedWriter::new(ctx.io_unit(SUNIT)?, None)?;
        writer.start()?;
        writer.write_str(b"FRAME_20000_CLASS    = 2 ")?;
        writer.finish()?;
    }
    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::ListDirectedWriter::new(ctx.io_unit(SUNIT)?, None)?;
        writer.start()?;
        writer.write_str(b"FRAME_20000_CENTER   = -399 ")?;
        writer.finish()?;
    }
    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::ListDirectedWriter::new(ctx.io_unit(SUNIT)?, None)?;
        writer.start()?;
        writer.write_str(b"FRAME_20000_CLASS_ID = -499 ")?;
        writer.finish()?;
    }
    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::ListDirectedWriter::new(ctx.io_unit(SUNIT)?, None)?;
        writer.start()?;
        writer.write_str(b"FRAME_20000_NAME     = \'TESTFRAME\' ")?;
        writer.finish()?;
    }
    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::ListDirectedWriter::new(ctx.io_unit(SUNIT)?, None)?;
        writer.start()?;
        writer.write_str(b"FRAME_TESTFRAME      = 20000 ")?;
        writer.finish()?;
    }
    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::ListDirectedWriter::new(ctx.io_unit(SUNIT)?, None)?;
        writer.start()?;
        writer.write_str(b" ")?;
        writer.finish()?;
    }

    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(SUNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    fstr::assign(&mut MESSGE, b"This is before loading the kernel pool.");

    testutil::TSTMSG(b"#", &MESSGE, ctx);
    spicelib::FRINFO(20000, &mut CENT, &mut CLASS, &mut CLSSID, &mut FOUND1, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND1", FOUND1, false, OK, ctx)?;

    spicelib::LDPOOL(b"framedat.cnk", ctx)?;

    fstr::assign(&mut MESSGE, b"This is after loading the kernel pool.");

    testutil::TSTMSG(b"#", &MESSGE, ctx);
    spicelib::FRMNAM(20000, &mut NAME, ctx)?;
    spicelib::NAMFRM(b"TESTFRAME", &mut IDCODE, ctx)?;
    spicelib::FRINFO(20000, &mut CENT, &mut CLASS, &mut CLSSID, &mut FOUND2, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND2", FOUND2, ISTRUE, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &NAME, b"=", b"TESTFRAME", OK, ctx)?;
    testutil::CHCKSI(b"IDCODE", IDCODE, b"=", 20000, 0, OK, ctx)?;
    testutil::CHCKSI(b"CENT", CENT, b"=", -399, 0, OK, ctx)?;
    testutil::CHCKSI(b"CLASS", CLASS, b"=", 2, 0, OK, ctx)?;
    testutil::CHCKSI(b"CLSSID", CLSSID, b"=", -499, 0, OK, ctx)?;

    testutil::KILFIL(b"framedat.cnk", ctx)?;
    testutil::TSTMSG(b"#", b" ", ctx);

    testutil::TCASE(
        b"Make sure that unrecognized names don\'t produce a non-zero frame code. ",
        ctx,
    )?;

    spicelib::NAMFRM(b"SPUD", &mut IDCODE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"IDCODE", IDCODE, b"=", 0, 0, OK, ctx)?;

    testutil::TCASE(b"Make sure that unknown idcodes produce blank names and that they are not found by FRINFO ", ctx)?;

    spicelib::FRMNAM(1000000, &mut NAME, ctx)?;
    spicelib::FRINFO(1000000, &mut CENT, &mut CLASS, &mut CLSSID, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND2", FOUND, false, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &NAME, b"=", b" ", OK, ctx)?;

    testutil::TCASE(b"Make sure that frames with id-codes in the range from 13001 to 13999 produce correct CENT, CLASS, and CLSSID values. ", ctx)?;

    for I in intrinsics::range(13001, 13999, 37) {
        FRAME = I;
        spicelib::FRINFO(FRAME, &mut CENT, &mut CLASS, &mut CLSSID, &mut FOUND, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
        testutil::CHCKSI(b"CENT", CENT, b"=", 399, 0, OK, ctx)?;
        testutil::CHCKSI(b"CENT", CLASS, b"=", 2, 0, OK, ctx)?;
        testutil::CHCKSI(b"CENT", CLSSID, b"=", (FRAME - 10000), 0, OK, ctx)?;
    }

    testutil::TCASE(b"Make sure the frame \'ITRF93\' is a recognized frame and the frame information associated with it is correct. ", ctx)?;

    spicelib::NAMFRM(b"ITRF93", &mut FRAME, ctx)?;
    spicelib::FRMNAM(FRAME, &mut NAME, ctx)?;
    spicelib::FRINFO(FRAME, &mut CENT, &mut CLASS, &mut CLSSID, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CENT", CENT, b"=", 399, 0, OK, ctx)?;
    testutil::CHCKSI(b"CENT", CLASS, b"=", 2, 0, OK, ctx)?;
    testutil::CHCKSI(b"CENT", CLSSID, b"=", 3000, 0, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &NAME, b"=", b"ITRF93", OK, ctx)?;
    testutil::CHCKSI(b"FRAME", FRAME, b"=", 13000, 0, OK, ctx)?;

    testutil::TCASE(b"Make sure the frame \'EARTH_FIXED\' is a recognized frame and the frame information associated with it is correct. ", ctx)?;

    spicelib::NAMFRM(b"EARTH_FIXED", &mut FRAME, ctx)?;
    spicelib::FRMNAM(FRAME, &mut NAME, ctx)?;
    spicelib::FRINFO(FRAME, &mut CENT, &mut CLASS, &mut CLSSID, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CENT", CENT, b"=", 399, 0, OK, ctx)?;
    testutil::CHCKSI(b"CENT", CLASS, b"=", 4, 0, OK, ctx)?;
    testutil::CHCKSI(b"CENT", CLSSID, b"=", 10081, 0, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &NAME, b"=", b"EARTH_FIXED", OK, ctx)?;
    testutil::CHCKSI(b"FRAME", FRAME, b"=", 10081, 0, OK, ctx)?;

    testutil::TCASE(b"Make sure that the frame to associate with the planets and satellites are known from the built-in list. ", ctx)?;

    fstr::assign(NAMES.get_mut(1), b"MERCURY");
    fstr::assign(NAMES.get_mut(2), b"VENUS");
    fstr::assign(NAMES.get_mut(3), b"EARTH");
    fstr::assign(NAMES.get_mut(4), b"MARS");
    fstr::assign(NAMES.get_mut(5), b"JUPITER");
    fstr::assign(NAMES.get_mut(6), b"SATURN");
    fstr::assign(NAMES.get_mut(7), b"URANUS");
    fstr::assign(NAMES.get_mut(8), b"NEPTUNE");
    fstr::assign(NAMES.get_mut(9), b"PLUTO");
    fstr::assign(NAMES.get_mut(10), b"SUN");
    fstr::assign(NAMES.get_mut(11), b"MOON");
    fstr::assign(NAMES.get_mut(12), b"PHOBOS");
    fstr::assign(NAMES.get_mut(13), b"DEIMOS");
    fstr::assign(NAMES.get_mut(14), b"IO");
    fstr::assign(NAMES.get_mut(15), b"EUROPA");
    fstr::assign(NAMES.get_mut(16), b"GANYMEDE");
    fstr::assign(NAMES.get_mut(17), b"CALLISTO");
    fstr::assign(NAMES.get_mut(18), b"AMALTHEA");
    fstr::assign(NAMES.get_mut(19), b"ADRASTEA");
    fstr::assign(NAMES.get_mut(20), b"METIS");
    fstr::assign(NAMES.get_mut(21), b"TITAN");
    fstr::assign(NAMES.get_mut(22), b"OBERON");
    fstr::assign(NAMES.get_mut(23), b"TITANIA");
    fstr::assign(NAMES.get_mut(24), b"UMBRIEL");
    fstr::assign(NAMES.get_mut(25), b"PUCK");
    fstr::assign(NAMES.get_mut(26), b"MIRANDA");
    fstr::assign(NAMES.get_mut(27), b"HIMALIA");
    fstr::assign(NAMES.get_mut(28), b"ARIEL");
    fstr::assign(NAMES.get_mut(29), b"ELARA");
    fstr::assign(NAMES.get_mut(30), b"TRITON");
    fstr::assign(NAMES.get_mut(31), b"NEREID");
    fstr::assign(NAMES.get_mut(32), b"CHARON");

    for I in 1..=32 {
        spicelib::CNMFRM(&NAMES[I], &mut FRCODE, &mut FRNAME, &mut FOUND, ctx)?;

        fstr::assign(
            &mut ENAME,
            &fstr::concat(b"IAU_", fstr::substr(NAMES.get(I), 1..=(WDSIZE - 4))),
        );

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
        testutil::CHCKSC(b"FRNAME", &FRNAME, b"=", &ENAME, OK, ctx)?;

        spicelib::NAMFRM(&FRNAME, &mut ECODE, ctx)?;
        testutil::CHCKSI(b"FRCODE", FRCODE, b"=", ECODE, 0, OK, ctx)?;
    }

    testutil::TCASE(b"Verify that the mapping between centers input as id-codes and associated frames is correctly maintained for the default set of bodyfixed frames. ", ctx)?;

    CODES[1] = 199;
    CODES[2] = 299;
    CODES[3] = 399;
    CODES[4] = 499;
    CODES[5] = 599;
    CODES[6] = 699;
    CODES[7] = 799;
    CODES[8] = 899;
    CODES[9] = 999;
    CODES[10] = 301;
    CODES[11] = 10;
    CODES[12] = 401;
    CODES[13] = 402;
    CODES[14] = 501;
    CODES[15] = 502;
    CODES[16] = 503;
    CODES[17] = 504;
    CODES[18] = 505;
    CODES[19] = 506;
    CODES[20] = 507;
    CODES[21] = 508;
    CODES[22] = 509;
    CODES[23] = 510;
    CODES[24] = 601;
    CODES[25] = 602;
    CODES[26] = 603;
    CODES[27] = 604;
    CODES[28] = 605;
    CODES[29] = 606;
    CODES[30] = 607;
    CODES[31] = 701;
    CODES[32] = 702;
    CODES[33] = 703;
    CODES[34] = 704;
    CODES[35] = 705;
    CODES[36] = 801;
    CODES[37] = 802;
    CODES[38] = 901;
    CODES[39] = 801;

    for I in 1..=39 {
        spicelib::BODC2N(CODES[I], &mut NAME, &mut FOUND, ctx)?;

        fstr::assign(
            &mut ENAME,
            &fstr::concat(b"IAU_", fstr::substr(&NAME, 1..=(WDSIZE - 4))),
        );

        spicelib::CIDFRM(CODES[I], &mut FRCODE, &mut FRNAME, &mut FOUND, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
        testutil::CHCKSC(b"FRNAME", &FRNAME, b"=", &ENAME, OK, ctx)?;

        spicelib::NAMFRM(&FRNAME, &mut ECODE, ctx)?;
        testutil::CHCKSI(b"FRCODE", FRCODE, b"=", ECODE, 0, OK, ctx)?;

        spicelib::FRINFO(FRCODE, &mut CODE, &mut CLASS, &mut CLSSID, &mut FOUND, ctx)?;

        testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
        testutil::CHCKSI(b"CODE", CODE, b"=", CODES[I], 0, OK, ctx)?;
    }

    testutil::TCASE(b"Load a preferred frame into the kernel pool and make sure that the frame subsystem can locate the preferred frame. ", ctx)?;

    testutil::BEGDAT(&mut TEXT[1]);
    fstr::assign(TEXT.get_mut(2), b"OBJECT_EARTH_FRAME = \'ITRF93\'");
    fstr::assign(TEXT.get_mut(3), b"OBJECT_199_FRAME   = \'J2000\'");
    fstr::assign(TEXT.get_mut(4), b"FRAME_EROSFIXED        = 1000001");
    fstr::assign(TEXT.get_mut(5), b"FRAME_1000001_NAME     = \'EROSFIXED\'");
    fstr::assign(TEXT.get_mut(6), b"FRAME_1000001_CLASS    = 2");
    fstr::assign(TEXT.get_mut(7), b"FRAME_1000001_CLASS_ID = 2000433");
    fstr::assign(TEXT.get_mut(8), b"FRAME_1000001_CENTER   = 2000433");
    fstr::assign(TEXT.get_mut(9), b" ");
    fstr::assign(TEXT.get_mut(10), b"OBJECT_2000433_FRAME   = \'EROSFIXED\'");

    testutil::KILFIL(b"framedat.cnk", ctx)?;
    testutil::TSTTXT(b"framedat.cnk", TEXT.as_arg(), 10, true, false, ctx)?;

    //
    // Check CIDFRM and CNMFRM to make sure they can find
    // a built-in frame associated with the Earth via a
    // kernel pool assignment.
    //
    spicelib::CIDFRM(399, &mut FRCODE, &mut FRNAME, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"FRNAME", &FRNAME, b"=", b"ITRF93", OK, ctx)?;
    testutil::CHCKSI(b"FRCODE", FRCODE, b"=", 13000, 0, OK, ctx)?;

    spicelib::CNMFRM(b"EARTH", &mut FRCODE, &mut FRNAME, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"FRNAME", &FRNAME, b"=", b"ITRF93", OK, ctx)?;
    testutil::CHCKSI(b"FRCODE", FRCODE, b"=", 13000, 0, OK, ctx)?;

    spicelib::CIDFRM(199, &mut FRCODE, &mut FRNAME, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"FRNAME", &FRNAME, b"=", b"J2000", OK, ctx)?;
    testutil::CHCKSI(b"FRCODE", FRCODE, b"=", 1, 0, OK, ctx)?;

    spicelib::CNMFRM(b"MERCURY", &mut FRCODE, &mut FRNAME, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"FRNAME", &FRNAME, b"=", b"J2000", OK, ctx)?;
    testutil::CHCKSI(b"FRCODE", FRCODE, b"=", 1, 0, OK, ctx)?;

    spicelib::CNMFRM(b"MATHILDE", &mut FRCODE, &mut FRNAME, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    spicelib::CIDFRM(1000131, &mut FRCODE, &mut FRNAME, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    //
    // Check CIDFRM and CNMFRM to make sure they can find
    // a TK frame associated with Eros via a
    // kernel pool assignment.
    //
    spicelib::CIDFRM(2000433, &mut FRCODE, &mut FRNAME, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"FRNAME", &FRNAME, b"=", b"EROSFIXED", OK, ctx)?;
    testutil::CHCKSI(b"FRCODE", FRCODE, b"=", 1000001, 0, OK, ctx)?;

    spicelib::CNMFRM(b"EROS", &mut FRCODE, &mut FRNAME, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"FRNAME", &FRNAME, b"=", b"EROSFIXED", OK, ctx)?;
    testutil::CHCKSI(b"FRCODE", FRCODE, b"=", 1000001, 0, OK, ctx)?;

    testutil::TCASE(
        b"Repeat previous test; identify RHS frames in object-frame assignment via frame ID code.",
        ctx,
    )?;

    spicelib::CLPOOL(ctx)?;

    testutil::BEGDAT(&mut TEXT[1]);
    fstr::assign(TEXT.get_mut(2), b"OBJECT_EARTH_FRAME =  13000");
    fstr::assign(TEXT.get_mut(3), b"OBJECT_199_FRAME   =  1");
    fstr::assign(TEXT.get_mut(4), b"FRAME_EROSFIXED        = 1000001");
    fstr::assign(TEXT.get_mut(5), b"FRAME_1000001_NAME     = \'EROSFIXED\'");
    fstr::assign(TEXT.get_mut(6), b"FRAME_1000001_CLASS    = 2");
    fstr::assign(TEXT.get_mut(7), b"FRAME_1000001_CLASS_ID = 2000433");
    fstr::assign(TEXT.get_mut(8), b"FRAME_1000001_CENTER   = 2000433");
    fstr::assign(TEXT.get_mut(9), b" ");
    fstr::assign(TEXT.get_mut(10), b"OBJECT_2000433_FRAME   = \'EROSFIXED\'");

    testutil::KILFIL(b"framedat.cnk", ctx)?;
    testutil::TSTTXT(b"framedat.cnk", TEXT.as_arg(), 10, true, false, ctx)?;

    //
    // Check CIDFRM and CNMFRM to make sure they can find
    // a built-in frame associated with the Earth via a
    // kernel pool assignment.
    //
    spicelib::CIDFRM(399, &mut FRCODE, &mut FRNAME, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"FRNAME", &FRNAME, b"=", b"ITRF93", OK, ctx)?;
    testutil::CHCKSI(b"FRCODE", FRCODE, b"=", 13000, 0, OK, ctx)?;

    spicelib::CNMFRM(b"EARTH", &mut FRCODE, &mut FRNAME, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"FRNAME", &FRNAME, b"=", b"ITRF93", OK, ctx)?;
    testutil::CHCKSI(b"FRCODE", FRCODE, b"=", 13000, 0, OK, ctx)?;

    spicelib::CIDFRM(199, &mut FRCODE, &mut FRNAME, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"FRNAME", &FRNAME, b"=", b"J2000", OK, ctx)?;
    testutil::CHCKSI(b"FRCODE", FRCODE, b"=", 1, 0, OK, ctx)?;

    spicelib::CNMFRM(b"MERCURY", &mut FRCODE, &mut FRNAME, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"FRNAME", &FRNAME, b"=", b"J2000", OK, ctx)?;
    testutil::CHCKSI(b"FRCODE", FRCODE, b"=", 1, 0, OK, ctx)?;

    spicelib::CNMFRM(b"MATHILDE", &mut FRCODE, &mut FRNAME, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    spicelib::CIDFRM(1000131, &mut FRCODE, &mut FRNAME, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    //
    // Check CIDFRM and CNMFRM to make sure they can find
    // a TK frame associated with Eros via a
    // kernel pool assignment.
    //
    spicelib::CIDFRM(2000433, &mut FRCODE, &mut FRNAME, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"FRNAME", &FRNAME, b"=", b"EROSFIXED", OK, ctx)?;
    testutil::CHCKSI(b"FRCODE", FRCODE, b"=", 1000001, 0, OK, ctx)?;

    spicelib::CNMFRM(b"EROS", &mut FRCODE, &mut FRNAME, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"FRNAME", &FRNAME, b"=", b"EROSFIXED", OK, ctx)?;
    testutil::CHCKSI(b"FRCODE", FRCODE, b"=", 1000001, 0, OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
