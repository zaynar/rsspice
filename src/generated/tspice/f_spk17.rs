//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const SEGSIZ: i32 = 40;
const WDSIZE: i32 = 32;

//$Procedure F_SPK17 ( Family of tests for the SPK type 17 code)
pub fn F_SPK17(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut SEGID = [b' '; SEGSIZ as usize];
    let mut FRAME = [b' '; WDSIZE as usize];
    let mut A: f64 = 0.0;
    let mut ARGP: f64 = 0.0;
    let mut DECPOL: f64 = 0.0;
    let mut ECC: f64 = 0.0;
    let mut EQEL = StackArray::<f64, 9>::new(1..=9);
    let mut ET: f64 = 0.0;
    let mut FIVDPD: f64 = 0.0;
    let mut GM: f64 = 0.0;
    let mut INC: f64 = 0.0;
    let mut M0: f64 = 0.0;
    let mut N: f64 = 0.0;
    let mut NODE: f64 = 0.0;
    let mut P: f64 = 0.0;
    let mut RAPOL: f64 = 0.0;
    let mut T0: f64 = 0.0;
    let mut TENDPD: f64 = 0.0;
    let mut STATE1 = StackArray::<f64, 6>::new(1..=6);
    let mut STATE2 = StackArray::<f64, 6>::new(1..=6);
    let mut FIRST: f64 = 0.0;
    let mut LAST: f64 = 0.0;
    let mut DESCR = StackArray::<f64, 5>::new(1..=5);
    let mut BEGIN: i32 = 0;
    let mut BODY: i32 = 0;
    let mut CENTER: i32 = 0;
    let mut END: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut SPKHAN: i32 = 0;
    let mut SPKH2: i32 = 0;
    let mut NELTS: i32 = 0;
    let mut NEWH: i32 = 0;
    let mut REF: i32 = 0;
    let mut TYPE: i32 = 0;
    let mut FOUND: bool = false;

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //

    testutil::TOPEN(b"F_SPK17", ctx)?;

    //
    // First delete our sample files (if they exists) and
    // open our first new file for writing.
    //
    testutil::KILFIL(b"spk17.bsp", ctx)?;
    testutil::KILFIL(b"spk17_2.bsp", ctx)?;

    spicelib::SPKOPN(b"spk17.bsp", b"TEST", 1000, &mut HANDLE, ctx)?;

    //
    // Here are some constants we'll want later, 10 and
    // 5 degrees/day.
    //
    FIVDPD = ((5.0 / 86400.0) * spicelib::RPD(ctx));
    TENDPD = ((10.0 / 86400.0) * spicelib::RPD(ctx));

    testutil::TCASE(
        b"Make sure that an error is signaled if the semi-major axis is non-positive. ",
        ctx,
    )?;

    P = 10000.0;
    GM = 398600.436;
    ECC = 0.1;
    A = (P / (1.0 - ECC));
    N = (f64::sqrt((GM / A)) / A);
    A = -A;
    ARGP = (30.0 * spicelib::RPD(ctx));
    NODE = (15.0 * spicelib::RPD(ctx));
    INC = (10.0 * spicelib::RPD(ctx));
    M0 = (45.0 * spicelib::RPD(ctx));
    T0 = -100000000.0;

    EQEL[1] = A;
    EQEL[2] = (ECC * f64::sin((ARGP + NODE)));
    EQEL[3] = (ECC * f64::cos((ARGP + NODE)));
    EQEL[4] = ((M0 + ARGP) + NODE);
    EQEL[5] = (f64::tan((INC / 2.0)) * f64::sin(NODE));
    EQEL[6] = (f64::tan((INC / 2.0)) * f64::cos(NODE));
    EQEL[7] = (FIVDPD + TENDPD);
    EQEL[8] = ((N + FIVDPD) + TENDPD);
    EQEL[9] = TENDPD;

    RAPOL = (30.0 * spicelib::RPD(ctx));
    DECPOL = (60.0 * spicelib::RPD(ctx));

    BODY = -1000;
    fstr::assign(&mut SEGID, b"PHOENIX");
    CENTER = 399;
    fstr::assign(&mut FRAME, b"B1950");

    FIRST = -1000000000.0;
    LAST = 1000000000.0;
    ET = 0.0;

    spicelib::SPKW17(
        HANDLE,
        BODY,
        CENTER,
        &FRAME,
        FIRST,
        LAST,
        &SEGID,
        T0,
        EQEL.as_slice(),
        RAPOL,
        DECPOL,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADSEMIAXIS)", OK, ctx)?;

    testutil::TCASE(
        b"Make sure an error is signaled if the eccentricity is greater than 0.9 ",
        ctx,
    )?;

    P = 10000.0;
    GM = 398600.436;
    ECC = 0.91;
    A = (P / (1.0 - ECC));
    N = (f64::sqrt((GM / A)) / A);
    ARGP = (30.0 * spicelib::RPD(ctx));
    NODE = (15.0 * spicelib::RPD(ctx));
    INC = (10.0 * spicelib::RPD(ctx));
    M0 = (45.0 * spicelib::RPD(ctx));
    T0 = -100000000.0;

    EQEL[1] = A;
    EQEL[2] = (ECC * f64::sin((ARGP + NODE)));
    EQEL[3] = (ECC * f64::cos((ARGP + NODE)));
    EQEL[4] = ((M0 + ARGP) + NODE);
    EQEL[5] = (f64::tan((INC / 2.0)) * f64::sin(NODE));
    EQEL[6] = (f64::tan((INC / 2.0)) * f64::cos(NODE));
    EQEL[7] = (FIVDPD + TENDPD);
    EQEL[8] = ((N + FIVDPD) + TENDPD);
    EQEL[9] = TENDPD;

    RAPOL = (30.0 * spicelib::RPD(ctx));
    DECPOL = (60.0 * spicelib::RPD(ctx));

    BODY = -1000;
    fstr::assign(&mut SEGID, b"PHOENIX");
    CENTER = 399;
    fstr::assign(&mut FRAME, b"B1950");

    FIRST = -1000000000.0;
    LAST = 1000000000.0;
    ET = 0.0;

    spicelib::SPKW17(
        HANDLE,
        BODY,
        CENTER,
        &FRAME,
        FIRST,
        LAST,
        &SEGID,
        T0,
        EQEL.as_slice(),
        RAPOL,
        DECPOL,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADECCENTRICITY)", OK, ctx)?;

    testutil::TCASE(b"Write and read a type 17 kernel.", ctx)?;

    P = 10000.0;
    GM = 398600.436;
    ECC = 0.1;
    A = (P / (1.0 - ECC));
    N = (f64::sqrt((GM / A)) / A);
    ARGP = (30.0 * spicelib::RPD(ctx));
    NODE = (15.0 * spicelib::RPD(ctx));
    INC = (10.0 * spicelib::RPD(ctx));
    M0 = (45.0 * spicelib::RPD(ctx));
    T0 = -100000000.0;

    //
    // We want a rate for the node of 10 degrees/day and
    // for the argument of periapse of 5 degrees/day.
    //
    FIVDPD = ((5.0 / 86400.0) * spicelib::RPD(ctx));
    TENDPD = ((10.0 / 86400.0) * spicelib::RPD(ctx));

    EQEL[1] = A;
    EQEL[2] = (ECC * f64::sin((ARGP + NODE)));
    EQEL[3] = (ECC * f64::cos((ARGP + NODE)));
    EQEL[4] = ((M0 + ARGP) + NODE);
    EQEL[5] = (f64::tan((INC / 2.0)) * f64::sin(NODE));
    EQEL[6] = (f64::tan((INC / 2.0)) * f64::cos(NODE));
    EQEL[7] = (FIVDPD + TENDPD);
    EQEL[8] = ((N + FIVDPD) + TENDPD);
    EQEL[9] = TENDPD;

    RAPOL = (30.0 * spicelib::RPD(ctx));
    DECPOL = (60.0 * spicelib::RPD(ctx));

    BODY = -1000;
    fstr::assign(&mut SEGID, b"PHOENIX");
    CENTER = 399;
    fstr::assign(&mut FRAME, b"B1950");

    FIRST = -1000000000.0;
    LAST = 1000000000.0;
    ET = 0.0;

    spicelib::SPKW17(
        HANDLE,
        BODY,
        CENTER,
        &FRAME,
        FIRST,
        LAST,
        &SEGID,
        T0,
        EQEL.as_slice(),
        RAPOL,
        DECPOL,
        ctx,
    )?;

    spicelib::SPKCLS(HANDLE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKLEF(b"spk17.bsp", &mut SPKHAN, ctx)?;

    spicelib::SPKSFS(
        BODY,
        ET,
        &mut HANDLE,
        DESCR.as_slice_mut(),
        &mut SEGID,
        &mut FOUND,
        ctx,
    )?;

    spicelib::SPKUDS(
        DESCR.as_slice(),
        &mut BODY,
        &mut CENTER,
        &mut REF,
        &mut TYPE,
        &mut FIRST,
        &mut LAST,
        &mut BEGIN,
        &mut END,
        ctx,
    )?;

    NELTS = ((END - BEGIN) + 1);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"BODY", BODY, b"=", -1000, 0, OK, ctx)?;
    testutil::CHCKSI(b"CENTER", CENTER, b"=", 399, 0, OK, ctx)?;
    testutil::CHCKSI(b"FRAME", REF, b"=", 2, 0, OK, ctx)?;
    testutil::CHCKSI(b"TYPE", TYPE, b"=", 17, 0, OK, ctx)?;
    testutil::CHCKSI(b"NELTS", NELTS, b"=", 12, 0, OK, ctx)?;
    testutil::CHCKSD(b"FIRST", FIRST, b"=", -1000000000.0, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"LAST", LAST, b"=", 1000000000.0, 0.0, OK, ctx)?;

    ET = (T0 - 10000.0);

    for I in 1..=100 {
        ET = (ET + 250.0);

        spicelib::EQNCPV(
            ET,
            T0,
            EQEL.as_slice(),
            RAPOL,
            DECPOL,
            STATE1.as_slice_mut(),
            ctx,
        )?;
        spicelib::SPKPVN(
            HANDLE,
            DESCR.as_slice(),
            ET,
            &mut REF,
            STATE2.as_slice_mut(),
            &mut CENTER,
            ctx,
        )?;

        testutil::CHCKSI(b"CENTER", CENTER, b"=", 399, 0, OK, ctx)?;
        testutil::CHCKSI(b"FRAME", REF, b"=", 2, 0, OK, ctx)?;
        testutil::CHCKAD(
            b"STATE",
            STATE1.as_slice(),
            b"=",
            STATE2.as_slice(),
            6,
            0.0,
            OK,
            ctx,
        )?;
    }
    spicelib::SPKUEF(SPKHAN, ctx)?;

    testutil::TCASE(
        b"Make sure we can successfully subset this the segment we just created. ",
        ctx,
    )?;

    testutil::KILFIL(b"spk17_2.bsp", ctx)?;

    spicelib::SPKLEF(b"spk17.bsp", &mut SPKHAN, ctx)?;
    spicelib::SPKSFS(
        BODY,
        ET,
        &mut HANDLE,
        DESCR.as_slice_mut(),
        &mut SEGID,
        &mut FOUND,
        ctx,
    )?;
    spicelib::SPKOPN(b"spk17_2.bsp", b"TEST2", 1000, &mut NEWH, ctx)?;

    fstr::assign(&mut SEGID, b"PHOENIX-2");

    FIRST = -100000000.0;
    LAST = 100000000.0;

    spicelib::SPKSUB(HANDLE, DESCR.as_slice(), &SEGID, FIRST, LAST, NEWH, ctx)?;
    spicelib::SPKCLS(NEWH, ctx)?;
    spicelib::SPKUEF(SPKHAN, ctx)?;
    //
    // Make sure no errors have occurred in the subsetting
    // portion of the code.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now load the new file and make sure we get correct states.
    //

    spicelib::SPKLEF(b"spk17_2.bsp", &mut SPKH2, ctx)?;
    //
    // First make sure that we don't find a segment when one
    // is not available.
    //
    ET = 110000000.0;
    spicelib::SPKSFS(
        BODY,
        ET,
        &mut HANDLE,
        DESCR.as_slice_mut(),
        &mut SEGID,
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    ET = -110000000.0;
    spicelib::SPKSFS(
        BODY,
        ET,
        &mut HANDLE,
        DESCR.as_slice_mut(),
        &mut SEGID,
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    //
    // Next make sure we get correct states.
    //
    ET = 0.0;
    spicelib::SPKSFS(
        BODY,
        ET,
        &mut HANDLE,
        DESCR.as_slice_mut(),
        &mut SEGID,
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    spicelib::SPKUDS(
        DESCR.as_slice(),
        &mut BODY,
        &mut CENTER,
        &mut REF,
        &mut TYPE,
        &mut FIRST,
        &mut LAST,
        &mut BEGIN,
        &mut END,
        ctx,
    )?;

    NELTS = ((END - BEGIN) + 1);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"BODY", BODY, b"=", -1000, 0, OK, ctx)?;
    testutil::CHCKSI(b"CENTER", CENTER, b"=", 399, 0, OK, ctx)?;
    testutil::CHCKSI(b"FRAME", REF, b"=", 2, 0, OK, ctx)?;
    testutil::CHCKSI(b"TYPE", TYPE, b"=", 17, 0, OK, ctx)?;
    testutil::CHCKSI(b"NELTS", NELTS, b"=", 12, 0, OK, ctx)?;
    testutil::CHCKSD(b"FIRST", FIRST, b"=", -100000000.0, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"LAST", LAST, b"=", 100000000.0, 0.0, OK, ctx)?;

    ET = (T0 - 10000.0);

    for I in 1..=100 {
        ET = (ET + 250.0);

        spicelib::EQNCPV(
            ET,
            T0,
            EQEL.as_slice(),
            RAPOL,
            DECPOL,
            STATE1.as_slice_mut(),
            ctx,
        )?;
        spicelib::SPKPVN(
            HANDLE,
            DESCR.as_slice(),
            ET,
            &mut REF,
            STATE2.as_slice_mut(),
            &mut CENTER,
            ctx,
        )?;

        testutil::CHCKSI(b"CENTER", CENTER, b"=", 399, 0, OK, ctx)?;
        testutil::CHCKSI(b"FRAME", REF, b"=", 2, 0, OK, ctx)?;
        testutil::CHCKAD(
            b"STATE",
            STATE1.as_slice(),
            b"=",
            STATE2.as_slice(),
            6,
            0.0,
            OK,
            ctx,
        )?;
    }
    spicelib::SPKUEF(SPKH2, ctx)?;
    spicelib::SPKUEF(SPKHAN, ctx)?;

    testutil::KILFIL(b"spk17.bsp", ctx)?;
    testutil::KILFIL(b"spk17_2.bsp", ctx)?;
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
