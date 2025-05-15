//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ERA: i32 = 1;
const WDAY: i32 = (ERA + 1);
const ZONE: i32 = (WDAY + 1);
const AMPM: i32 = (ZONE + 1);
const SYSTEM: i32 = (AMPM + 1);
const NMODS: i32 = SYSTEM;
const LNSIZE: i32 = 80;
const LONGSZ: i32 = 300;
const WDSIZE: i32 = 32;

//$Procedure F_TPARTV2 ( Family of tests for TPARTV )
pub fn F_TPARTV2(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut STRING = [b' '; LNSIZE as usize];
    let mut ERROR = [b' '; LONGSZ as usize];
    let mut EERROR = [b' '; LONGSZ as usize];
    let mut PICTUR = [b' '; LNSIZE as usize];
    let mut PIC = [b' '; LNSIZE as usize];
    let mut TYPE = [b' '; WDSIZE as usize];
    let mut ETYPE = [b' '; WDSIZE as usize];
    let mut MODIFY = ActualCharArray::new(WDSIZE, 1..=NMODS);
    let mut EMODFY = ActualCharArray::new(WDSIZE, 1..=NMODS);
    let mut TVEC = StackArray::<f64, 7>::new(1..=7);
    let mut ETVEC = StackArray::<f64, 7>::new(1..=7);
    let mut NTVEC: i32 = 0;
    let mut ENTVEC: i32 = 0;
    let mut MODS: bool = false;
    let mut EMODS: bool = false;
    let mut SUCCES: bool = false;
    let mut ESUCCS: bool = false;
    let mut YABBRV: bool = false;

    //
    // Test Utility Functions
    //

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    testutil::TOPEN(b"F_TPARTV2", ctx)?;

    testutil::TCASE(b"Y-M-DTH-M-S ", ctx)?;

    fstr::assign(&mut STRING, b"1827-12-27T02:28:27");
    fstr::assign(&mut PIC, b"YYYY-MM-DDTHR:MN:SC");
    ENTVEC = 6;

    ETVEC[1] = 1827.0;
    ETVEC[2] = 12.0;
    ETVEC[3] = 27.0;
    ETVEC[4] = 2.0;
    ETVEC[5] = 28.0;
    ETVEC[6] = 27.0;

    fstr::assign(EMODFY.get_mut(1), b" ");
    fstr::assign(EMODFY.get_mut(2), b" ");
    fstr::assign(EMODFY.get_mut(3), b" ");
    fstr::assign(EMODFY.get_mut(4), b" ");
    fstr::assign(EMODFY.get_mut(5), b" ");

    EMODS = false;
    ESUCCS = true;
    fstr::assign(&mut EERROR, b" ");
    fstr::assign(&mut ETYPE, b"YMD");

    spicelib::TPARTV(
        &STRING,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TYPE,
        MODIFY.as_arg_mut(),
        &mut MODS,
        &mut YABBRV,
        &mut SUCCES,
        &mut PICTUR,
        &mut ERROR,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"TVEC",
        TVEC.as_slice(),
        b"=",
        ETVEC.as_slice(),
        6,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"NTVEC", NTVEC, b"=", ENTVEC, 0, OK, ctx)?;
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", &ETYPE, OK, ctx)?;
    testutil::CHCKSC(b"PICTUR", &PICTUR, b"=", &PIC, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EERROR, OK, ctx)?;
    testutil::CHCKSL(b"MODS", MODS, EMODS, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCCS, OK, ctx)?;

    testutil::CHCKSC(b"MODIFY(1)", &MODIFY[1], b"=", &EMODFY[1], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(2)", &MODIFY[2], b"=", &EMODFY[2], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(3)", &MODIFY[3], b"=", &EMODFY[3], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(4)", &MODIFY[4], b"=", &EMODFY[4], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(5)", &MODIFY[5], b"=", &EMODFY[5], OK, ctx)?;

    testutil::TCASE(b"Y-M-DTH-M.xxx ", ctx)?;

    fstr::assign(&mut STRING, b"1827-12-27T02:28.281");
    fstr::assign(&mut PIC, b"YYYY-MM-DDTHR:MN.### ::RND");
    ENTVEC = 5;

    ETVEC[1] = 1827.0;
    ETVEC[2] = 12.0;
    ETVEC[3] = 27.0;
    ETVEC[4] = 2.0;
    ETVEC[5] = 28.281;
    ETVEC[6] = 0.0;

    fstr::assign(EMODFY.get_mut(1), b" ");
    fstr::assign(EMODFY.get_mut(2), b" ");
    fstr::assign(EMODFY.get_mut(3), b" ");
    fstr::assign(EMODFY.get_mut(4), b" ");
    fstr::assign(EMODFY.get_mut(5), b" ");

    EMODS = false;
    ESUCCS = true;
    fstr::assign(&mut EERROR, b" ");
    fstr::assign(&mut ETYPE, b"YMD");

    spicelib::TPARTV(
        &STRING,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TYPE,
        MODIFY.as_arg_mut(),
        &mut MODS,
        &mut YABBRV,
        &mut SUCCES,
        &mut PICTUR,
        &mut ERROR,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"TVEC",
        TVEC.as_slice(),
        b"=",
        ETVEC.as_slice(),
        6,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"NTVEC", NTVEC, b"=", ENTVEC, 0, OK, ctx)?;
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", &ETYPE, OK, ctx)?;
    testutil::CHCKSC(b"PICTUR", &PICTUR, b"=", &PIC, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EERROR, OK, ctx)?;
    testutil::CHCKSL(b"MODS", MODS, EMODS, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCCS, OK, ctx)?;

    testutil::CHCKSC(b"MODIFY(1)", &MODIFY[1], b"=", &EMODFY[1], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(2)", &MODIFY[2], b"=", &EMODFY[2], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(3)", &MODIFY[3], b"=", &EMODFY[3], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(4)", &MODIFY[4], b"=", &EMODFY[4], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(5)", &MODIFY[5], b"=", &EMODFY[5], OK, ctx)?;

    testutil::TCASE(b"Y-M-DTH-M.xxx (2)", ctx)?;

    fstr::assign(&mut STRING, b"1827-12-27T02:28.277");
    fstr::assign(&mut PIC, b"YYYY-MM-DDTHR:MN.### ::RND");
    ENTVEC = 5;

    ETVEC[1] = 1827.0;
    ETVEC[2] = 12.0;
    ETVEC[3] = 27.0;
    ETVEC[4] = 2.0;
    ETVEC[5] = 28.277;
    ETVEC[6] = 0.0;

    fstr::assign(EMODFY.get_mut(1), b" ");
    fstr::assign(EMODFY.get_mut(2), b" ");
    fstr::assign(EMODFY.get_mut(3), b" ");
    fstr::assign(EMODFY.get_mut(4), b" ");
    fstr::assign(EMODFY.get_mut(5), b" ");

    EMODS = false;
    ESUCCS = true;
    fstr::assign(&mut EERROR, b" ");
    fstr::assign(&mut ETYPE, b"YMD");

    spicelib::TPARTV(
        &STRING,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TYPE,
        MODIFY.as_arg_mut(),
        &mut MODS,
        &mut YABBRV,
        &mut SUCCES,
        &mut PICTUR,
        &mut ERROR,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"TVEC",
        TVEC.as_slice(),
        b"=",
        ETVEC.as_slice(),
        6,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"NTVEC", NTVEC, b"=", ENTVEC, 0, OK, ctx)?;
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", &ETYPE, OK, ctx)?;
    testutil::CHCKSC(b"PICTUR", &PICTUR, b"=", &PIC, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EERROR, OK, ctx)?;
    testutil::CHCKSL(b"MODS", MODS, EMODS, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCCS, OK, ctx)?;

    testutil::CHCKSC(b"MODIFY(1)", &MODIFY[1], b"=", &EMODFY[1], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(2)", &MODIFY[2], b"=", &EMODFY[2], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(3)", &MODIFY[3], b"=", &EMODFY[3], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(4)", &MODIFY[4], b"=", &EMODFY[4], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(5)", &MODIFY[5], b"=", &EMODFY[5], OK, ctx)?;

    testutil::TCASE(b"Y-DOY/", ctx)?;

    fstr::assign(&mut STRING, b"1996-27/");
    fstr::assign(&mut PIC, b"YYYY-DOY/");
    ENTVEC = 2;

    ETVEC[1] = 1996.0;
    ETVEC[2] = 27.0;
    ETVEC[3] = 0.0;
    ETVEC[4] = 0.0;
    ETVEC[5] = 0.0;

    fstr::assign(EMODFY.get_mut(1), b" ");
    fstr::assign(EMODFY.get_mut(2), b" ");
    fstr::assign(EMODFY.get_mut(3), b" ");
    fstr::assign(EMODFY.get_mut(4), b" ");
    fstr::assign(EMODFY.get_mut(5), b" ");

    EMODS = false;
    ESUCCS = true;
    fstr::assign(&mut EERROR, b" ");
    fstr::assign(&mut ETYPE, b"YD");

    spicelib::TPARTV(
        &STRING,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TYPE,
        MODIFY.as_arg_mut(),
        &mut MODS,
        &mut YABBRV,
        &mut SUCCES,
        &mut PICTUR,
        &mut ERROR,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"TVEC",
        TVEC.as_slice(),
        b"=",
        ETVEC.as_slice(),
        5,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"NTVEC", NTVEC, b"=", ENTVEC, 0, OK, ctx)?;
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", &ETYPE, OK, ctx)?;
    testutil::CHCKSC(b"PICTUR", &PICTUR, b"=", &PIC, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EERROR, OK, ctx)?;
    testutil::CHCKSL(b"MODS", MODS, EMODS, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCCS, OK, ctx)?;

    testutil::CHCKSC(b"MODIFY(1)", &MODIFY[1], b"=", &EMODFY[1], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(2)", &MODIFY[2], b"=", &EMODFY[2], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(3)", &MODIFY[3], b"=", &EMODFY[3], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(4)", &MODIFY[4], b"=", &EMODFY[4], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(5)", &MODIFY[5], b"=", &EMODFY[5], OK, ctx)?;

    testutil::TCASE(b"YYYY-DOY/H:M ", ctx)?;

    fstr::assign(&mut STRING, b"1996-288/16:03");
    fstr::assign(&mut PIC, b"YYYY-DOY/HR:MN");
    ENTVEC = 4;

    ETVEC[1] = 1996.0;
    ETVEC[2] = 288.0;
    ETVEC[3] = 16.0;
    ETVEC[4] = 3.0;
    ETVEC[5] = 0.0;

    fstr::assign(EMODFY.get_mut(1), b" ");
    fstr::assign(EMODFY.get_mut(2), b" ");
    fstr::assign(EMODFY.get_mut(3), b" ");
    fstr::assign(EMODFY.get_mut(4), b" ");
    fstr::assign(EMODFY.get_mut(5), b" ");

    EMODS = false;
    ESUCCS = true;
    fstr::assign(&mut EERROR, b" ");
    fstr::assign(&mut ETYPE, b"YD");

    spicelib::TPARTV(
        &STRING,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TYPE,
        MODIFY.as_arg_mut(),
        &mut MODS,
        &mut YABBRV,
        &mut SUCCES,
        &mut PICTUR,
        &mut ERROR,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"TVEC",
        TVEC.as_slice(),
        b"=",
        ETVEC.as_slice(),
        5,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"NTVEC", NTVEC, b"=", ENTVEC, 0, OK, ctx)?;
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", &ETYPE, OK, ctx)?;
    testutil::CHCKSC(b"PICTUR", &PICTUR, b"=", &PIC, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EERROR, OK, ctx)?;
    testutil::CHCKSL(b"MODS", MODS, EMODS, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCCS, OK, ctx)?;

    testutil::CHCKSC(b"MODIFY(1)", &MODIFY[1], b"=", &EMODFY[1], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(2)", &MODIFY[2], b"=", &EMODFY[2], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(3)", &MODIFY[3], b"=", &EMODFY[3], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(4)", &MODIFY[4], b"=", &EMODFY[4], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(5)", &MODIFY[5], b"=", &EMODFY[5], OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
