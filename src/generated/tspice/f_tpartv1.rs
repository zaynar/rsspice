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

//$Procedure F_TPARTV1 ( Family of tests for TPARTV )
pub fn F_TPARTV1(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_TPARTV1", ctx)?;

    //
    // We are first going to check on q group of strings that we
    // think should be parsable.
    //
    testutil::TCASE(b"YMDHMS", ctx)?;

    fstr::assign(&mut STRING, b"1994 Mar 12 12:28:18.281 ");

    ETVEC[1] = 1994.0;
    ETVEC[2] = 3.0;
    ETVEC[3] = 12.0;
    ETVEC[4] = 12.0;
    ETVEC[5] = 28.0;
    ETVEC[6] = 18.281;

    ENTVEC = 6;
    fstr::assign(&mut ETYPE, b"YMD");

    fstr::assign(EMODFY.get_mut(1), b" ");
    fstr::assign(EMODFY.get_mut(2), b" ");
    fstr::assign(EMODFY.get_mut(3), b" ");
    fstr::assign(EMODFY.get_mut(4), b" ");
    fstr::assign(EMODFY.get_mut(5), b" ");

    EMODS = false;
    ESUCCS = true;
    fstr::assign(&mut EERROR, b" ");

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
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", b"YMD", OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EERROR, OK, ctx)?;
    testutil::CHCKSL(b"MODS", MODS, EMODS, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCCS, OK, ctx)?;

    testutil::CHCKSC(b"MODIFY(1)", &MODIFY[1], b"=", &EMODFY[1], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(2)", &MODIFY[2], b"=", &EMODFY[2], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(3)", &MODIFY[3], b"=", &EMODFY[3], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(4)", &MODIFY[4], b"=", &EMODFY[4], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(5)", &MODIFY[5], b"=", &EMODFY[5], OK, ctx)?;

    testutil::TCASE(b"YMDHMS", ctx)?;

    fstr::assign(&mut STRING, b"1994 3 12 12:18:18.282 TDT ");
    ETVEC[1] = 1994.0;
    ETVEC[2] = 3.0;
    ETVEC[3] = 12.0;
    ETVEC[4] = 12.0;
    ETVEC[5] = 18.0;
    ETVEC[6] = 18.282;

    ENTVEC = 6;
    fstr::assign(&mut ETYPE, b"YMD");

    fstr::assign(EMODFY.get_mut(1), b" ");
    fstr::assign(EMODFY.get_mut(2), b" ");
    fstr::assign(EMODFY.get_mut(3), b" ");
    fstr::assign(EMODFY.get_mut(4), b" ");
    fstr::assign(EMODFY.get_mut(5), b"TDT");

    EMODS = true;
    ESUCCS = true;
    fstr::assign(&mut EERROR, b" ");

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
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", b"YMD", OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EERROR, OK, ctx)?;
    testutil::CHCKSL(b"MODS", MODS, EMODS, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCCS, OK, ctx)?;

    testutil::CHCKSC(b"MODIFY(1)", &MODIFY[1], b"=", &EMODFY[1], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(2)", &MODIFY[2], b"=", &EMODFY[2], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(3)", &MODIFY[3], b"=", &EMODFY[3], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(4)", &MODIFY[4], b"=", &EMODFY[4], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(5)", &MODIFY[5], b"=", &EMODFY[5], OK, ctx)?;

    testutil::TCASE(b"DMYHMS", ctx)?;

    fstr::assign(&mut STRING, b"3/APR/57 A.D. 12:18.1981 TDB ");
    ETVEC[1] = 57.0;
    ETVEC[2] = 4.0;
    ETVEC[3] = 3.0;
    ETVEC[4] = 12.0;
    ETVEC[5] = 18.1981;
    ETVEC[6] = 0.0;

    ENTVEC = 5;
    fstr::assign(&mut ETYPE, b"YMD");

    fstr::assign(EMODFY.get_mut(1), b"A.D.");
    fstr::assign(EMODFY.get_mut(2), b" ");
    fstr::assign(EMODFY.get_mut(3), b" ");
    fstr::assign(EMODFY.get_mut(4), b" ");
    fstr::assign(EMODFY.get_mut(5), b"TDB");

    EMODS = true;
    ESUCCS = true;
    fstr::assign(&mut EERROR, b" ");

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
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", b"YMD", OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EERROR, OK, ctx)?;
    testutil::CHCKSL(b"MODS", MODS, EMODS, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCCS, OK, ctx)?;

    testutil::CHCKSC(b"MODIFY(1)", &MODIFY[1], b"=", &EMODFY[1], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(2)", &MODIFY[2], b"=", &EMODFY[2], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(3)", &MODIFY[3], b"=", &EMODFY[3], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(4)", &MODIFY[4], b"=", &EMODFY[4], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(5)", &MODIFY[5], b"=", &EMODFY[5], OK, ctx)?;

    testutil::TCASE(b"MDYHMS", ctx)?;

    fstr::assign(&mut STRING, b"3/28/\'96 18:28:28.289 PDT ");

    ETVEC[1] = 96.0;
    ETVEC[2] = 3.0;
    ETVEC[3] = 28.0;
    ETVEC[4] = 18.0;
    ETVEC[5] = 28.0;
    ETVEC[6] = 28.289;

    ENTVEC = 6;
    fstr::assign(&mut ETYPE, b"YMD");

    fstr::assign(EMODFY.get_mut(1), b" ");
    fstr::assign(EMODFY.get_mut(2), b" ");
    fstr::assign(EMODFY.get_mut(3), b" ");
    fstr::assign(EMODFY.get_mut(4), b" ");
    fstr::assign(EMODFY.get_mut(5), b" ");
    fstr::assign(EMODFY.get_mut(ZONE), b"UTC-7");

    EMODS = true;
    ESUCCS = true;
    fstr::assign(&mut EERROR, b" ");

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
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", b"YMD", OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EERROR, OK, ctx)?;
    testutil::CHCKSL(b"MODS", MODS, EMODS, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCCS, OK, ctx)?;

    testutil::CHCKSC(b"MODIFY(1)", &MODIFY[1], b"=", &EMODFY[1], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(2)", &MODIFY[2], b"=", &EMODFY[2], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(3)", &MODIFY[3], b"=", &EMODFY[3], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(4)", &MODIFY[4], b"=", &EMODFY[4], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(5)", &MODIFY[5], b"=", &EMODFY[5], OK, ctx)?;

    testutil::TCASE(b"MDHMSY", ctx)?;

    fstr::assign(&mut STRING, b"Mon Apr 22 09:40:36 PDT 1996 ");

    ETVEC[1] = 1996.0;
    ETVEC[2] = 4.0;
    ETVEC[3] = 22.0;
    ETVEC[4] = 9.0;
    ETVEC[5] = 40.0;
    ETVEC[6] = 36.0;

    ENTVEC = 6;
    fstr::assign(&mut ETYPE, b"YMD");

    fstr::assign(EMODFY.get_mut(1), b" ");
    fstr::assign(EMODFY.get_mut(2), b" ");
    fstr::assign(EMODFY.get_mut(3), b" ");
    fstr::assign(EMODFY.get_mut(4), b" ");
    fstr::assign(EMODFY.get_mut(5), b" ");
    fstr::assign(EMODFY.get_mut(ZONE), b"UTC-7");
    fstr::assign(EMODFY.get_mut(WDAY), b"MON");

    EMODS = true;
    ESUCCS = true;
    fstr::assign(&mut EERROR, b" ");

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
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", b"YMD", OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EERROR, OK, ctx)?;
    testutil::CHCKSL(b"MODS", MODS, EMODS, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCCS, OK, ctx)?;

    testutil::CHCKSC(b"MODIFY(1)", &MODIFY[1], b"=", &EMODFY[1], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(2)", &MODIFY[2], b"=", &EMODFY[2], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(3)", &MODIFY[3], b"=", &EMODFY[3], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(4)", &MODIFY[4], b"=", &EMODFY[4], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(5)", &MODIFY[5], b"=", &EMODFY[5], OK, ctx)?;

    testutil::TCASE(b"YDHMS", ctx)?;

    fstr::assign(&mut STRING, b"1918-171/ 03:28:57.1819 (UTC) ");

    ETVEC[1] = 1918.0;
    ETVEC[2] = 171.0;
    ETVEC[3] = 3.0;
    ETVEC[4] = 28.0;
    ETVEC[5] = 57.1819;

    ENTVEC = 5;
    fstr::assign(&mut ETYPE, b"YD");

    fstr::assign(EMODFY.get_mut(1), b" ");
    fstr::assign(EMODFY.get_mut(2), b" ");
    fstr::assign(EMODFY.get_mut(3), b" ");
    fstr::assign(EMODFY.get_mut(4), b" ");
    fstr::assign(EMODFY.get_mut(5), b" ");
    fstr::assign(EMODFY.get_mut(SYSTEM), b"UTC");

    EMODS = true;
    ESUCCS = true;
    fstr::assign(&mut EERROR, b" ");

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
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EERROR, OK, ctx)?;
    testutil::CHCKSL(b"MODS", MODS, EMODS, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCCS, OK, ctx)?;

    testutil::CHCKSC(b"MODIFY(1)", &MODIFY[1], b"=", &EMODFY[1], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(2)", &MODIFY[2], b"=", &EMODFY[2], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(3)", &MODIFY[3], b"=", &EMODFY[3], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(4)", &MODIFY[4], b"=", &EMODFY[4], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(5)", &MODIFY[5], b"=", &EMODFY[5], OK, ctx)?;

    testutil::TCASE(b"YDHMS", ctx)?;
    fstr::assign(&mut STRING, b"1986 32// 02:18:09. CDT ");

    ETVEC[1] = 1986.0;
    ETVEC[2] = 32.0;
    ETVEC[3] = 2.0;
    ETVEC[4] = 18.0;
    ETVEC[5] = 9.0;

    ENTVEC = 5;
    fstr::assign(&mut ETYPE, b"YD");

    fstr::assign(EMODFY.get_mut(1), b" ");
    fstr::assign(EMODFY.get_mut(2), b" ");
    fstr::assign(EMODFY.get_mut(3), b" ");
    fstr::assign(EMODFY.get_mut(4), b" ");
    fstr::assign(EMODFY.get_mut(5), b" ");
    fstr::assign(EMODFY.get_mut(ZONE), b"UTC-5");

    EMODS = true;
    ESUCCS = true;
    fstr::assign(&mut EERROR, b" ");

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
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EERROR, OK, ctx)?;
    testutil::CHCKSL(b"MODS", MODS, EMODS, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCCS, OK, ctx)?;

    testutil::CHCKSC(b"MODIFY(1)", &MODIFY[1], b"=", &EMODFY[1], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(2)", &MODIFY[2], b"=", &EMODFY[2], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(3)", &MODIFY[3], b"=", &EMODFY[3], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(4)", &MODIFY[4], b"=", &EMODFY[4], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(5)", &MODIFY[5], b"=", &EMODFY[5], OK, ctx)?;

    testutil::TCASE(b"JD", ctx)?;

    fstr::assign(&mut STRING, b"2441889.18997917 JDUTC ");

    ETVEC[1] = 2441889.18997917;

    ENTVEC = 1;
    fstr::assign(&mut ETYPE, b"JD");

    fstr::assign(EMODFY.get_mut(1), b" ");
    fstr::assign(EMODFY.get_mut(2), b" ");
    fstr::assign(EMODFY.get_mut(3), b" ");
    fstr::assign(EMODFY.get_mut(4), b" ");
    fstr::assign(EMODFY.get_mut(5), b" ");
    fstr::assign(EMODFY.get_mut(SYSTEM), b"UTC");

    EMODS = true;
    ESUCCS = true;
    fstr::assign(&mut EERROR, b" ");

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
        1,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"NTVEC", NTVEC, b"=", ENTVEC, 0, OK, ctx)?;
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", &ETYPE, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EERROR, OK, ctx)?;
    testutil::CHCKSL(b"MODS", MODS, EMODS, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCCS, OK, ctx)?;

    testutil::CHCKSC(b"MODIFY(1)", &MODIFY[1], b"=", &EMODFY[1], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(2)", &MODIFY[2], b"=", &EMODFY[2], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(3)", &MODIFY[3], b"=", &EMODFY[3], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(4)", &MODIFY[4], b"=", &EMODFY[4], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(5)", &MODIFY[5], b"=", &EMODFY[5], OK, ctx)?;

    testutil::TCASE(b"JD", ctx)?;
    fstr::assign(&mut STRING, b"2451545.5000000 JDTDB ");

    ETVEC[1] = 2451545.5;

    ENTVEC = 1;
    fstr::assign(&mut ETYPE, b"JD");

    fstr::assign(EMODFY.get_mut(1), b" ");
    fstr::assign(EMODFY.get_mut(2), b" ");
    fstr::assign(EMODFY.get_mut(3), b" ");
    fstr::assign(EMODFY.get_mut(4), b" ");
    fstr::assign(EMODFY.get_mut(5), b" ");
    fstr::assign(EMODFY.get_mut(SYSTEM), b"TDB");

    EMODS = true;
    ESUCCS = true;
    fstr::assign(&mut EERROR, b" ");

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
        1,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"NTVEC", NTVEC, b"=", ENTVEC, 0, OK, ctx)?;
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", &ETYPE, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EERROR, OK, ctx)?;
    testutil::CHCKSL(b"MODS", MODS, EMODS, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCCS, OK, ctx)?;

    testutil::CHCKSC(b"MODIFY(1)", &MODIFY[1], b"=", &EMODFY[1], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(2)", &MODIFY[2], b"=", &EMODFY[2], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(3)", &MODIFY[3], b"=", &EMODFY[3], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(4)", &MODIFY[4], b"=", &EMODFY[4], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(5)", &MODIFY[5], b"=", &EMODFY[5], OK, ctx)?;

    testutil::TCASE(b"JD", ctx)?;
    fstr::assign(&mut STRING, b"2451792.1827191 JD ");

    ETVEC[1] = 2451792.1827191;

    ENTVEC = 1;
    fstr::assign(&mut ETYPE, b"JD");

    fstr::assign(EMODFY.get_mut(1), b" ");
    fstr::assign(EMODFY.get_mut(2), b" ");
    fstr::assign(EMODFY.get_mut(3), b" ");
    fstr::assign(EMODFY.get_mut(4), b" ");
    fstr::assign(EMODFY.get_mut(5), b" ");

    EMODS = false;
    ESUCCS = true;
    fstr::assign(&mut EERROR, b" ");

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
        1,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"NTVEC", NTVEC, b"=", ENTVEC, 0, OK, ctx)?;
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", &ETYPE, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EERROR, OK, ctx)?;
    testutil::CHCKSL(b"MODS", MODS, EMODS, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCCS, OK, ctx)?;

    testutil::CHCKSC(b"MODIFY(1)", &MODIFY[1], b"=", &EMODFY[1], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(2)", &MODIFY[2], b"=", &EMODFY[2], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(3)", &MODIFY[3], b"=", &EMODFY[3], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(4)", &MODIFY[4], b"=", &EMODFY[4], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(5)", &MODIFY[5], b"=", &EMODFY[5], OK, ctx)?;

    testutil::TCASE(b"JD", ctx)?;

    fstr::assign(&mut STRING, b"2431829.28719 (JD) ");
    ETVEC[1] = 2431829.28719;

    ENTVEC = 1;
    fstr::assign(&mut ETYPE, b"JD");

    fstr::assign(EMODFY.get_mut(1), b" ");
    fstr::assign(EMODFY.get_mut(2), b" ");
    fstr::assign(EMODFY.get_mut(3), b" ");
    fstr::assign(EMODFY.get_mut(4), b" ");
    fstr::assign(EMODFY.get_mut(5), b" ");

    EMODS = false;
    ESUCCS = true;
    fstr::assign(&mut EERROR, b" ");

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
        1,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(b"NTVEC", NTVEC, b"=", ENTVEC, 0, OK, ctx)?;
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", &ETYPE, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EERROR, OK, ctx)?;
    testutil::CHCKSL(b"MODS", MODS, EMODS, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCCS, OK, ctx)?;

    testutil::CHCKSC(b"MODIFY(1)", &MODIFY[1], b"=", &EMODFY[1], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(2)", &MODIFY[2], b"=", &EMODFY[2], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(3)", &MODIFY[3], b"=", &EMODFY[3], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(4)", &MODIFY[4], b"=", &EMODFY[4], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(5)", &MODIFY[5], b"=", &EMODFY[5], OK, ctx)?;

    testutil::TCASE(b"ISO1", ctx)?;

    fstr::assign(&mut STRING, b"1992-12-18T12:18:18 ");

    ETVEC[1] = 1992.0;
    ETVEC[2] = 12.0;
    ETVEC[3] = 18.0;
    ETVEC[4] = 12.0;
    ETVEC[5] = 18.0;
    ETVEC[6] = 18.0;

    ENTVEC = 6;
    fstr::assign(&mut ETYPE, b"YMD");

    fstr::assign(EMODFY.get_mut(1), b" ");
    fstr::assign(EMODFY.get_mut(2), b" ");
    fstr::assign(EMODFY.get_mut(3), b" ");
    fstr::assign(EMODFY.get_mut(4), b" ");
    fstr::assign(EMODFY.get_mut(5), b" ");

    EMODS = false;
    ESUCCS = true;
    fstr::assign(&mut EERROR, b" ");

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
    testutil::CHCKSC(b"TYPE", &TYPE, b"=", b"YMD", OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EERROR, OK, ctx)?;
    testutil::CHCKSL(b"MODS", MODS, EMODS, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCCS, OK, ctx)?;

    testutil::CHCKSC(b"MODIFY(1)", &MODIFY[1], b"=", &EMODFY[1], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(2)", &MODIFY[2], b"=", &EMODFY[2], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(3)", &MODIFY[3], b"=", &EMODFY[3], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(4)", &MODIFY[4], b"=", &EMODFY[4], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(5)", &MODIFY[5], b"=", &EMODFY[5], OK, ctx)?;

    testutil::TCASE(b"ISO2", ctx)?;

    fstr::assign(&mut STRING, b"1995-12T ");

    ETVEC[1] = 1995.0;
    ETVEC[2] = 12.0;
    ETVEC[3] = 0.0;
    ETVEC[4] = 0.0;
    ETVEC[5] = 0.0;

    ENTVEC = 2;
    fstr::assign(&mut ETYPE, b"YD");

    fstr::assign(EMODFY.get_mut(1), b" ");
    fstr::assign(EMODFY.get_mut(2), b" ");
    fstr::assign(EMODFY.get_mut(3), b" ");
    fstr::assign(EMODFY.get_mut(4), b" ");
    fstr::assign(EMODFY.get_mut(5), b" ");

    EMODS = false;
    ESUCCS = true;
    fstr::assign(&mut EERROR, b" ");

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
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EERROR, OK, ctx)?;
    testutil::CHCKSL(b"MODS", MODS, EMODS, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCCS, OK, ctx)?;

    testutil::CHCKSC(b"MODIFY(1)", &MODIFY[1], b"=", &EMODFY[1], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(2)", &MODIFY[2], b"=", &EMODFY[2], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(3)", &MODIFY[3], b"=", &EMODFY[3], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(4)", &MODIFY[4], b"=", &EMODFY[4], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(5)", &MODIFY[5], b"=", &EMODFY[5], OK, ctx)?;

    testutil::TCASE(b"ISO3", ctx)?;

    fstr::assign(&mut STRING, b"1996-1-18T ");

    ETVEC[1] = 1996.0;
    ETVEC[2] = 1.0;
    ETVEC[3] = 18.0;
    ETVEC[4] = 0.0;
    ETVEC[5] = 0.0;
    ETVEC[6] = 0.0;

    ENTVEC = 3;
    fstr::assign(&mut ETYPE, b"YMD");

    fstr::assign(EMODFY.get_mut(1), b" ");
    fstr::assign(EMODFY.get_mut(2), b" ");
    fstr::assign(EMODFY.get_mut(3), b" ");
    fstr::assign(EMODFY.get_mut(4), b" ");
    fstr::assign(EMODFY.get_mut(5), b" ");

    EMODS = false;
    ESUCCS = true;
    fstr::assign(&mut EERROR, b" ");

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
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EERROR, OK, ctx)?;
    testutil::CHCKSL(b"MODS", MODS, EMODS, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCCS, OK, ctx)?;

    testutil::CHCKSC(b"MODIFY(1)", &MODIFY[1], b"=", &EMODFY[1], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(2)", &MODIFY[2], b"=", &EMODFY[2], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(3)", &MODIFY[3], b"=", &EMODFY[3], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(4)", &MODIFY[4], b"=", &EMODFY[4], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(5)", &MODIFY[5], b"=", &EMODFY[5], OK, ctx)?;

    testutil::TCASE(b"ISO4", ctx)?;

    fstr::assign(&mut STRING, b"1996-3-18T12 ");

    ETVEC[1] = 1996.0;
    ETVEC[2] = 3.0;
    ETVEC[3] = 18.0;
    ETVEC[4] = 12.0;
    ETVEC[5] = 0.0;
    ETVEC[6] = 0.0;

    ENTVEC = 4;
    fstr::assign(&mut ETYPE, b"YMD");

    fstr::assign(EMODFY.get_mut(1), b" ");
    fstr::assign(EMODFY.get_mut(2), b" ");
    fstr::assign(EMODFY.get_mut(3), b" ");
    fstr::assign(EMODFY.get_mut(4), b" ");
    fstr::assign(EMODFY.get_mut(5), b" ");

    EMODS = false;
    ESUCCS = true;
    fstr::assign(&mut EERROR, b" ");

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

    testutil::TSTMSG(b"#", &STRING, ctx);
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
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EERROR, OK, ctx)?;
    testutil::CHCKSL(b"MODS", MODS, EMODS, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCCS, OK, ctx)?;

    testutil::CHCKSC(b"MODIFY(1)", &MODIFY[1], b"=", &EMODFY[1], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(2)", &MODIFY[2], b"=", &EMODFY[2], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(3)", &MODIFY[3], b"=", &EMODFY[3], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(4)", &MODIFY[4], b"=", &EMODFY[4], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(5)", &MODIFY[5], b"=", &EMODFY[5], OK, ctx)?;
    testutil::TSTMSG(b"#", b" ", ctx);

    testutil::TCASE(b"ISO5", ctx)?;
    fstr::assign(&mut STRING, b"1996-3-18T12:28.187 ");

    ETVEC[1] = 1996.0;
    ETVEC[2] = 3.0;
    ETVEC[3] = 18.0;
    ETVEC[4] = 12.0;
    ETVEC[5] = 28.187;
    ETVEC[6] = 0.0;

    ENTVEC = 5;
    fstr::assign(&mut ETYPE, b"YMD");

    fstr::assign(EMODFY.get_mut(1), b" ");
    fstr::assign(EMODFY.get_mut(2), b" ");
    fstr::assign(EMODFY.get_mut(3), b" ");
    fstr::assign(EMODFY.get_mut(4), b" ");
    fstr::assign(EMODFY.get_mut(5), b" ");

    EMODS = false;
    ESUCCS = true;
    fstr::assign(&mut EERROR, b" ");

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
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EERROR, OK, ctx)?;
    testutil::CHCKSL(b"MODS", MODS, EMODS, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCCS, OK, ctx)?;

    testutil::CHCKSC(b"MODIFY(1)", &MODIFY[1], b"=", &EMODFY[1], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(2)", &MODIFY[2], b"=", &EMODFY[2], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(3)", &MODIFY[3], b"=", &EMODFY[3], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(4)", &MODIFY[4], b"=", &EMODFY[4], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(5)", &MODIFY[5], b"=", &EMODFY[5], OK, ctx)?;

    testutil::TCASE(b"ISO6", ctx)?;

    fstr::assign(&mut STRING, b"1993-172T12:18:18.1879292 ");

    ETVEC[1] = 1993.0;
    ETVEC[2] = 172.0;
    ETVEC[3] = 12.0;
    ETVEC[4] = 18.0;
    ETVEC[5] = 18.1879292;

    ENTVEC = 5;
    fstr::assign(&mut ETYPE, b"YD");

    fstr::assign(EMODFY.get_mut(1), b" ");
    fstr::assign(EMODFY.get_mut(2), b" ");
    fstr::assign(EMODFY.get_mut(3), b" ");
    fstr::assign(EMODFY.get_mut(4), b" ");
    fstr::assign(EMODFY.get_mut(5), b" ");

    EMODS = false;
    ESUCCS = true;
    fstr::assign(&mut EERROR, b" ");

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
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EERROR, OK, ctx)?;
    testutil::CHCKSL(b"MODS", MODS, EMODS, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCCS, OK, ctx)?;

    testutil::CHCKSC(b"MODIFY(1)", &MODIFY[1], b"=", &EMODFY[1], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(2)", &MODIFY[2], b"=", &EMODFY[2], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(3)", &MODIFY[3], b"=", &EMODFY[3], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(4)", &MODIFY[4], b"=", &EMODFY[4], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(5)", &MODIFY[5], b"=", &EMODFY[5], OK, ctx)?;

    testutil::TCASE(b"YMD", ctx)?;
    fstr::assign(&mut STRING, b"105 B.C. 3/4 ");

    ETVEC[1] = 105.0;
    ETVEC[2] = 3.0;
    ETVEC[3] = 4.0;
    ETVEC[4] = 0.0;
    ETVEC[5] = 0.0;
    ETVEC[6] = 0.0;

    ENTVEC = 3;
    fstr::assign(&mut ETYPE, b"YMD");

    fstr::assign(EMODFY.get_mut(1), b" ");
    fstr::assign(EMODFY.get_mut(2), b" ");
    fstr::assign(EMODFY.get_mut(3), b" ");
    fstr::assign(EMODFY.get_mut(4), b" ");
    fstr::assign(EMODFY.get_mut(5), b" ");
    fstr::assign(EMODFY.get_mut(ERA), b"B.C.");

    EMODS = true;
    ESUCCS = true;
    fstr::assign(&mut EERROR, b" ");

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
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EERROR, OK, ctx)?;
    testutil::CHCKSL(b"MODS", MODS, EMODS, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCCS, OK, ctx)?;

    testutil::CHCKSC(b"MODIFY(1)", &MODIFY[1], b"=", &EMODFY[1], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(2)", &MODIFY[2], b"=", &EMODFY[2], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(3)", &MODIFY[3], b"=", &EMODFY[3], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(4)", &MODIFY[4], b"=", &EMODFY[4], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(5)", &MODIFY[5], b"=", &EMODFY[5], OK, ctx)?;

    testutil::TCASE(b"YDM", ctx)?;
    fstr::assign(&mut STRING, b"18 A.D. 4 March ");

    ETVEC[1] = 18.0;
    ETVEC[2] = 3.0;
    ETVEC[3] = 4.0;
    ETVEC[4] = 0.0;
    ETVEC[5] = 0.0;
    ETVEC[6] = 0.0;

    ENTVEC = 3;
    fstr::assign(&mut ETYPE, b"YMD");

    fstr::assign(EMODFY.get_mut(1), b" ");
    fstr::assign(EMODFY.get_mut(2), b" ");
    fstr::assign(EMODFY.get_mut(3), b" ");
    fstr::assign(EMODFY.get_mut(4), b" ");
    fstr::assign(EMODFY.get_mut(5), b" ");
    fstr::assign(EMODFY.get_mut(ERA), b"A.D.");

    EMODS = true;
    ESUCCS = true;
    fstr::assign(&mut EERROR, b" ");

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
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EERROR, OK, ctx)?;
    testutil::CHCKSL(b"MODS", MODS, EMODS, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCCS, OK, ctx)?;

    testutil::CHCKSC(b"MODIFY(1)", &MODIFY[1], b"=", &EMODFY[1], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(2)", &MODIFY[2], b"=", &EMODFY[2], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(3)", &MODIFY[3], b"=", &EMODFY[3], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(4)", &MODIFY[4], b"=", &EMODFY[4], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(5)", &MODIFY[5], b"=", &EMODFY[5], OK, ctx)?;

    testutil::TCASE(b"YMDHMS", ctx)?;
    fstr::assign(&mut STRING, b"1919 12 02 12 18 19.19 ");

    ETVEC[1] = 1919.0;
    ETVEC[2] = 12.0;
    ETVEC[3] = 2.0;
    ETVEC[4] = 12.0;
    ETVEC[5] = 18.0;
    ETVEC[6] = 19.19;

    ENTVEC = 6;
    fstr::assign(&mut ETYPE, b"YMD");

    fstr::assign(EMODFY.get_mut(1), b" ");
    fstr::assign(EMODFY.get_mut(2), b" ");
    fstr::assign(EMODFY.get_mut(3), b" ");
    fstr::assign(EMODFY.get_mut(4), b" ");
    fstr::assign(EMODFY.get_mut(5), b" ");

    EMODS = false;
    ESUCCS = true;
    fstr::assign(&mut EERROR, b" ");

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
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EERROR, OK, ctx)?;
    testutil::CHCKSL(b"MODS", MODS, EMODS, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCCS, OK, ctx)?;

    testutil::CHCKSC(b"MODIFY(1)", &MODIFY[1], b"=", &EMODFY[1], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(2)", &MODIFY[2], b"=", &EMODFY[2], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(3)", &MODIFY[3], b"=", &EMODFY[3], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(4)", &MODIFY[4], b"=", &EMODFY[4], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(5)", &MODIFY[5], b"=", &EMODFY[5], OK, ctx)?;

    testutil::TCASE(b"YHMSDM", ctx)?;
    fstr::assign(&mut STRING, b"1991 12:18:03.182 7 JAN ");

    ETVEC[1] = 1991.0;
    ETVEC[2] = 1.0;
    ETVEC[3] = 7.0;
    ETVEC[4] = 12.0;
    ETVEC[5] = 18.0;
    ETVEC[6] = 3.182;

    ENTVEC = 6;
    fstr::assign(&mut ETYPE, b"YMD");

    fstr::assign(EMODFY.get_mut(1), b" ");
    fstr::assign(EMODFY.get_mut(2), b" ");
    fstr::assign(EMODFY.get_mut(3), b" ");
    fstr::assign(EMODFY.get_mut(4), b" ");
    fstr::assign(EMODFY.get_mut(5), b" ");

    EMODS = false;
    ESUCCS = true;
    fstr::assign(&mut EERROR, b" ");

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
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EERROR, OK, ctx)?;
    testutil::CHCKSL(b"MODS", MODS, EMODS, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCCS, OK, ctx)?;

    testutil::CHCKSC(b"MODIFY(1)", &MODIFY[1], b"=", &EMODFY[1], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(2)", &MODIFY[2], b"=", &EMODFY[2], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(3)", &MODIFY[3], b"=", &EMODFY[3], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(4)", &MODIFY[4], b"=", &EMODFY[4], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(5)", &MODIFY[5], b"=", &EMODFY[5], OK, ctx)?;

    testutil::TCASE(b"HMSMDY", ctx)?;
    fstr::assign(&mut STRING, b"12:18:07 March 18, 1828 ");

    ETVEC[1] = 1828.0;
    ETVEC[2] = 3.0;
    ETVEC[3] = 18.0;
    ETVEC[4] = 12.0;
    ETVEC[5] = 18.0;
    ETVEC[6] = 7.0;

    ENTVEC = 6;
    fstr::assign(&mut ETYPE, b"YMD");

    fstr::assign(EMODFY.get_mut(1), b" ");
    fstr::assign(EMODFY.get_mut(2), b" ");
    fstr::assign(EMODFY.get_mut(3), b" ");
    fstr::assign(EMODFY.get_mut(4), b" ");
    fstr::assign(EMODFY.get_mut(5), b" ");

    EMODS = false;
    ESUCCS = true;
    fstr::assign(&mut EERROR, b" ");

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
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EERROR, OK, ctx)?;
    testutil::CHCKSL(b"MODS", MODS, EMODS, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCCS, OK, ctx)?;

    testutil::CHCKSC(b"MODIFY(1)", &MODIFY[1], b"=", &EMODFY[1], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(2)", &MODIFY[2], b"=", &EMODFY[2], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(3)", &MODIFY[3], b"=", &EMODFY[3], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(4)", &MODIFY[4], b"=", &EMODFY[4], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(5)", &MODIFY[5], b"=", &EMODFY[5], OK, ctx)?;

    testutil::TCASE(b"DYHMS", ctx)?;
    fstr::assign(&mut STRING, b"261-1998/17:18:21.1879 ");

    ETVEC[1] = 1998.0;
    ETVEC[2] = 261.0;
    ETVEC[3] = 17.0;
    ETVEC[4] = 18.0;
    ETVEC[5] = 21.1879;

    ENTVEC = 5;
    fstr::assign(&mut ETYPE, b"YD");

    fstr::assign(EMODFY.get_mut(1), b" ");
    fstr::assign(EMODFY.get_mut(2), b" ");
    fstr::assign(EMODFY.get_mut(3), b" ");
    fstr::assign(EMODFY.get_mut(4), b" ");
    fstr::assign(EMODFY.get_mut(5), b" ");

    EMODS = false;
    ESUCCS = true;
    fstr::assign(&mut EERROR, b" ");

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
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EERROR, OK, ctx)?;
    testutil::CHCKSL(b"MODS", MODS, EMODS, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCCS, OK, ctx)?;

    testutil::CHCKSC(b"MODIFY(1)", &MODIFY[1], b"=", &EMODFY[1], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(2)", &MODIFY[2], b"=", &EMODFY[2], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(3)", &MODIFY[3], b"=", &EMODFY[3], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(4)", &MODIFY[4], b"=", &EMODFY[4], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(5)", &MODIFY[5], b"=", &EMODFY[5], OK, ctx)?;

    testutil::TCASE(b"DYHMS", ctx)?;
    fstr::assign(&mut STRING, b"217-1998::12:18:21.2987 ");

    ETVEC[1] = 1998.0;
    ETVEC[2] = 217.0;
    ETVEC[3] = 12.0;
    ETVEC[4] = 18.0;
    ETVEC[5] = 21.2987;

    ENTVEC = 5;
    fstr::assign(&mut ETYPE, b"YD");

    fstr::assign(EMODFY.get_mut(1), b" ");
    fstr::assign(EMODFY.get_mut(2), b" ");
    fstr::assign(EMODFY.get_mut(3), b" ");
    fstr::assign(EMODFY.get_mut(4), b" ");
    fstr::assign(EMODFY.get_mut(5), b" ");

    EMODS = false;
    ESUCCS = true;
    fstr::assign(&mut EERROR, b" ");

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
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EERROR, OK, ctx)?;
    testutil::CHCKSL(b"MODS", MODS, EMODS, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCCS, OK, ctx)?;

    testutil::CHCKSC(b"MODIFY(1)", &MODIFY[1], b"=", &EMODFY[1], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(2)", &MODIFY[2], b"=", &EMODFY[2], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(3)", &MODIFY[3], b"=", &EMODFY[3], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(4)", &MODIFY[4], b"=", &EMODFY[4], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(5)", &MODIFY[5], b"=", &EMODFY[5], OK, ctx)?;

    testutil::TCASE(b"DYHMS", ctx)?;

    fstr::assign(&mut STRING, b"178-1872//12:18:17.1879 ");

    ETVEC[1] = 1872.0;
    ETVEC[2] = 178.0;
    ETVEC[3] = 12.0;
    ETVEC[4] = 18.0;
    ETVEC[5] = 17.1879;

    ENTVEC = 5;
    fstr::assign(&mut ETYPE, b"YD");

    fstr::assign(EMODFY.get_mut(1), b" ");
    fstr::assign(EMODFY.get_mut(2), b" ");
    fstr::assign(EMODFY.get_mut(3), b" ");
    fstr::assign(EMODFY.get_mut(4), b" ");
    fstr::assign(EMODFY.get_mut(5), b" ");

    EMODS = false;
    ESUCCS = true;
    fstr::assign(&mut EERROR, b" ");

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
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EERROR, OK, ctx)?;
    testutil::CHCKSL(b"MODS", MODS, EMODS, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCCS, OK, ctx)?;

    testutil::CHCKSC(b"MODIFY(1)", &MODIFY[1], b"=", &EMODFY[1], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(2)", &MODIFY[2], b"=", &EMODFY[2], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(3)", &MODIFY[3], b"=", &EMODFY[3], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(4)", &MODIFY[4], b"=", &EMODFY[4], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(5)", &MODIFY[5], b"=", &EMODFY[5], OK, ctx)?;

    testutil::TCASE(b"YMDHMS", ctx)?;

    fstr::assign(&mut STRING, b"1982-JAN 57 12:72:93.2987 ");

    ETVEC[1] = 1982.0;
    ETVEC[2] = 1.0;
    ETVEC[3] = 57.0;
    ETVEC[4] = 12.0;
    ETVEC[5] = 72.0;
    ETVEC[6] = 93.2987;

    ENTVEC = 6;
    fstr::assign(&mut ETYPE, b"YMD");

    fstr::assign(EMODFY.get_mut(1), b" ");
    fstr::assign(EMODFY.get_mut(2), b" ");
    fstr::assign(EMODFY.get_mut(3), b" ");
    fstr::assign(EMODFY.get_mut(4), b" ");
    fstr::assign(EMODFY.get_mut(5), b" ");

    EMODS = false;
    ESUCCS = true;
    fstr::assign(&mut EERROR, b" ");

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
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EERROR, OK, ctx)?;
    testutil::CHCKSL(b"MODS", MODS, EMODS, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCCS, OK, ctx)?;

    testutil::CHCKSC(b"MODIFY(1)", &MODIFY[1], b"=", &EMODFY[1], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(2)", &MODIFY[2], b"=", &EMODFY[2], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(3)", &MODIFY[3], b"=", &EMODFY[3], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(4)", &MODIFY[4], b"=", &EMODFY[4], OK, ctx)?;
    testutil::CHCKSC(b"MODIFY(5)", &MODIFY[5], b"=", &EMODFY[5], OK, ctx)?;

    testutil::TCASE(b"UNIX Date/Time", ctx)?;

    fstr::assign(&mut STRING, b"Tue Apr 30 09:08:46 PDT 1996");
    fstr::assign(&mut PIC, b"Wkd Mon DD HR:MN:SC PDT YYYY ::UTC-7");
    ENTVEC = 6;

    ETVEC[1] = 1996.0;
    ETVEC[2] = 4.0;
    ETVEC[3] = 30.0;
    ETVEC[4] = 9.0;
    ETVEC[5] = 8.0;
    ETVEC[6] = 46.0;

    fstr::assign(EMODFY.get_mut(1), b" ");
    fstr::assign(EMODFY.get_mut(2), b"TUE");
    fstr::assign(EMODFY.get_mut(3), b"UTC-7");
    fstr::assign(EMODFY.get_mut(4), b" ");
    fstr::assign(EMODFY.get_mut(5), b" ");

    EMODS = true;
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

    testutil::TCASE(b"UNIX MAIL Time Stamp", ctx)?;

    fstr::assign(&mut STRING, b"Tue, Apr 30 09:08:46 PDT 1996");
    fstr::assign(&mut PIC, b"Wkd, Mon DD HR:MN:SC PDT YYYY ::UTC-7");
    ENTVEC = 6;

    ETVEC[1] = 1996.0;
    ETVEC[2] = 4.0;
    ETVEC[3] = 30.0;
    ETVEC[4] = 9.0;
    ETVEC[5] = 8.0;
    ETVEC[6] = 46.0;

    fstr::assign(EMODFY.get_mut(1), b" ");
    fstr::assign(EMODFY.get_mut(2), b"TUE");
    fstr::assign(EMODFY.get_mut(3), b"UTC-7");
    fstr::assign(EMODFY.get_mut(4), b" ");
    fstr::assign(EMODFY.get_mut(5), b" ");

    EMODS = true;
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

    testutil::TCASE(b"UNIX Date/Time Abbreviated", ctx)?;

    fstr::assign(&mut STRING, b"Tue Apr 30 09:08:46 PDT 96");
    fstr::assign(&mut PIC, b" ");

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
    testutil::CHCKSI(b"NTVEC", NTVEC, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, false, OK, ctx)?;
    testutil::CHCKSC(b"PICTUR", &PICTUR, b"=", &PIC, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    testutil::TCASE(b"Leading Delimiter ", ctx)?;

    fstr::assign(&mut STRING, b"-1996 Jan 12");
    fstr::assign(&mut PIC, b" ");

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
    testutil::CHCKSI(b"NTVEC", NTVEC, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, false, OK, ctx)?;
    testutil::CHCKSC(b"PICTUR", &PICTUR, b"=", &PIC, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    testutil::TCASE(b"Trailing Delimiter ", ctx)?;

    fstr::assign(&mut STRING, b"1997 Jan 12 ,");
    fstr::assign(&mut PIC, b" ");

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
    testutil::CHCKSI(b"NTVEC", NTVEC, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, false, OK, ctx)?;
    testutil::CHCKSC(b"PICTUR", &PICTUR, b"=", &PIC, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    testutil::TCASE(b"YMDT ", ctx)?;

    fstr::assign(&mut STRING, b"1996-12-18T");
    fstr::assign(&mut PIC, b"YYYY-MM-DDT");
    ENTVEC = 3;

    ETVEC[1] = 1996.0;
    ETVEC[2] = 12.0;
    ETVEC[3] = 18.0;
    ETVEC[4] = 0.0;
    ETVEC[5] = 0.0;
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

    testutil::TCASE(b"YMDTHM ", ctx)?;

    fstr::assign(&mut STRING, b"1827-12-27T02:28");
    fstr::assign(&mut PIC, b"YYYY-MM-DDTHR:MN");
    ENTVEC = 5;

    ETVEC[1] = 1827.0;
    ETVEC[2] = 12.0;
    ETVEC[3] = 27.0;
    ETVEC[4] = 2.0;
    ETVEC[5] = 28.0;
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

    testutil::TCASE(b"YMDTHM and trailing colon ", ctx)?;

    fstr::assign(&mut STRING, b"1827-12-27T02:28:");
    fstr::assign(&mut PIC, b" ");

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
    testutil::CHCKSI(b"NTVEC", NTVEC, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, false, OK, ctx)?;
    testutil::CHCKSC(b"PICTUR", &PICTUR, b"=", &PIC, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
