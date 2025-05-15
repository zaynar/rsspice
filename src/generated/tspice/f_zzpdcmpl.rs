//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LT: i32 = -1;
const EQ: i32 = 0;
const GT: i32 = 1;

//$Procedure F_ZZPDCMPL ( ZZPDCMPL tests )
pub fn F_ZZPDCMPL(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ALT: f64 = 0.0;
    let mut F: f64 = 0.0;
    let mut LAT: f64 = 0.0;
    let mut LON: f64 = 0.0;
    let mut P = StackArray::<f64, 3>::new(1..=3);
    let mut PLAT: f64 = 0.0;
    let mut RE: f64 = 0.0;
    let mut RP: f64 = 0.0;
    let mut REL: i32 = 0;
    let mut XREL: i32 = 0;

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

    // CHARACTER*(LNSIZE)    LABEL
    // CHARACTER*(MSGLEN)    TITLE

    // INTEGER               I
    // INTEGER               J

    //
    // Saved values
    //
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_ZZPDCMPL", ctx)?;

    //***********************************************************************
    //
    //     Special cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Point is at origin, LAT > 0.", ctx)?;

    RE = 4.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = (spicelib::PI(ctx) / 4 as f64);

    P[1] = 0.0;
    P[2] = 0.0;
    P[3] = 0.0;

    //
    // The latitude of P is considered to be zero.
    //
    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XREL = LT;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Point is at origin, LAT < 0.", ctx)?;

    RE = 4.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = -(spicelib::PI(ctx) / 4 as f64);

    P[1] = 0.0;
    P[2] = 0.0;
    P[3] = 0.0;

    //
    // The latitude of P is considered to be zero.
    //
    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XREL = GT;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Point is at origin, LAT = 0.", ctx)?;

    RE = 4.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = 0.0;

    P[1] = 0.0;
    P[2] = 0.0;
    P[3] = 0.0;

    //
    // The latitude of P is considered to be zero.
    //
    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XREL = EQ;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Point is on -Z axis, LAT > 0.", ctx)?;

    RE = 4.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = (spicelib::PI(ctx) / 4 as f64);

    P[1] = 0.0;
    P[2] = 0.0;
    P[3] = -1.0;

    //
    // The latitude of P is -pi/2.
    //
    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XREL = LT;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Point is on -Z axis, -pi/2 < LAT < 0.", ctx)?;

    RE = 4.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = -(spicelib::PI(ctx) / 4 as f64);

    P[1] = 0.0;
    P[2] = 0.0;
    P[3] = -1.0;

    //
    // The latitude of P is -pi/2.
    //
    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XREL = LT;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Point is on -Z axis, LAT = 0.", ctx)?;

    RE = 4.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = 0.0;

    P[1] = 0.0;
    P[2] = 0.0;
    P[3] = -1.0;

    //
    // The latitude of P is -pi/2.
    //
    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XREL = LT;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //***********************************************************************
    //
    //     LAT = 0 tests
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"RP = 2*RE; LAT = 0 deg; P(3) > 0", ctx)?;

    RE = 2.0;
    RP = 4.0;
    F = ((RE - RP) / RE);
    LAT = 0.0;

    P[1] = 1.0;
    P[2] = 1.0;
    P[3] = 1.0;

    XREL = GT;

    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"RP = 2*RE; LAT = 0 deg; P(3) = 0", ctx)?;

    RE = 2.0;
    RP = 4.0;
    F = ((RE - RP) / RE);
    LAT = 0.0;

    P[1] = 1.0;
    P[2] = 1.0;
    P[3] = 0.0;

    XREL = EQ;

    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"RP = 2*RE; LAT = 0 deg; P(3) < 0", ctx)?;

    RE = 2.0;
    RP = 4.0;
    F = ((RE - RP) / RE);
    LAT = 0.0;

    P[1] = 1.0;
    P[2] = 1.0;
    P[3] = -1.0;

    XREL = LT;

    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"RP = RE/2; LAT = 0 deg; P(3) > 0", ctx)?;

    RE = 2.0;
    RP = 1.0;
    F = ((RE - RP) / RE);
    LAT = 0.0;

    P[1] = 1.0;
    P[2] = 1.0;
    P[3] = 1.0;

    XREL = GT;

    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"RP = RE/2; LAT = 0 deg; P(3) = 0", ctx)?;

    RE = 2.0;
    RP = 1.0;
    F = ((RE - RP) / RE);
    LAT = 0.0;

    P[1] = 1.0;
    P[2] = 1.0;
    P[3] = 0.0;

    XREL = EQ;

    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"RP = RE/2; LAT = 0 deg; P(3) < 0", ctx)?;

    RE = 2.0;
    RP = 1.0;
    F = ((RE - RP) / RE);
    LAT = 0.0;

    P[1] = 1.0;
    P[2] = 1.0;
    P[3] = -1.0;

    XREL = LT;

    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //***********************************************************************
    //
    //     LAT = pi/2, -pi/2 tests
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"RP = 2*RE; LAT = 90 deg; P(3) > 0, P(1)=P(2)=0", ctx)?;

    RE = 2.0;
    RP = 4.0;
    F = ((RE - RP) / RE);
    LAT = spicelib::HALFPI(ctx);

    P[1] = 0.0;
    P[2] = 0.0;
    P[3] = 1.0;

    XREL = EQ;

    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"RP = 2*RE; LAT = 90 deg; P(3) > 0, P(1) != 0", ctx)?;

    RE = 2.0;
    RP = 4.0;
    F = ((RE - RP) / RE);
    LAT = spicelib::HALFPI(ctx);

    P[1] = 1.0;
    P[2] = 0.0;
    P[3] = 1.0;

    XREL = LT;

    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"RP = 2*RE; LAT = 90 deg; P(3) > 0, P(2) != 0", ctx)?;

    RE = 2.0;
    RP = 4.0;
    F = ((RE - RP) / RE);
    LAT = spicelib::HALFPI(ctx);

    P[1] = 0.0;
    P[2] = 1.0;
    P[3] = 1.0;

    XREL = LT;

    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"RP = 2*RE; LAT = -90 deg; P(3) < 0, P(1)=P(2)=0", ctx)?;

    RE = 2.0;
    RP = 4.0;
    F = ((RE - RP) / RE);
    LAT = -spicelib::HALFPI(ctx);

    P[1] = 0.0;
    P[2] = 0.0;
    P[3] = -1.0;

    XREL = EQ;

    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"RP = 2*RE; LAT = -90 deg; P(3) < 0, P(1) != 0", ctx)?;

    RE = 2.0;
    RP = 4.0;
    F = ((RE - RP) / RE);
    LAT = -spicelib::HALFPI(ctx);

    P[1] = 1.0;
    P[2] = 0.0;
    P[3] = -1.0;

    XREL = GT;

    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"RP = 2*RE; LAT = -90 deg; P(3) < 0, P(2) != 0", ctx)?;

    RE = 2.0;
    RP = 4.0;
    F = ((RE - RP) / RE);
    LAT = -spicelib::HALFPI(ctx);

    P[1] = 0.0;
    P[2] = 1.0;
    P[3] = -1.0;

    XREL = GT;

    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //***********************************************************************
    //
    //     Basic tests for prolate spheroids
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"RP = 2*RE; LAT = 60 deg; lat(P) = 70 deg", ctx)?;

    RE = 2.0;
    RP = 4.0;
    F = ((RE - RP) / RE);
    LAT = (60.0 * spicelib::RPD(ctx));
    PLAT = (70.0 * spicelib::RPD(ctx));
    LON = (30.0 * spicelib::RPD(ctx));
    ALT = 1.0;

    spicelib::GEOREC(LON, PLAT, ALT, RE, F, P.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XREL = GT;

    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"RP = 2*RE; LAT = 60 deg; lat(P) = 50 deg", ctx)?;

    RE = 2.0;
    RP = 4.0;
    F = ((RE - RP) / RE);
    LAT = (60.0 * spicelib::RPD(ctx));
    PLAT = (50.0 * spicelib::RPD(ctx));
    LON = (30.0 * spicelib::RPD(ctx));
    ALT = 1.0;

    spicelib::GEOREC(LON, PLAT, ALT, RE, F, P.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XREL = LT;

    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"RP = 2*RE; LAT = 60 deg; lat(P) = -50 deg", ctx)?;

    RE = 2.0;
    RP = 4.0;
    F = ((RE - RP) / RE);
    LAT = (60.0 * spicelib::RPD(ctx));
    PLAT = -(50.0 * spicelib::RPD(ctx));
    LON = (30.0 * spicelib::RPD(ctx));
    ALT = 1.0;

    spicelib::GEOREC(LON, PLAT, ALT, RE, F, P.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XREL = LT;

    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"RP = 2*RE; LAT = 60 deg; lat(P) = 60+1e-13 deg", ctx)?;

    RE = 2.0;
    RP = 4.0;
    F = ((RE - RP) / RE);
    LAT = (60.0 * spicelib::RPD(ctx));
    PLAT = ((60.0 + 0.0000000000001) * spicelib::RPD(ctx));
    LON = (30.0 * spicelib::RPD(ctx));
    ALT = 1.0;

    spicelib::GEOREC(LON, PLAT, ALT, RE, F, P.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XREL = GT;

    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"RP = 2*RE; LAT = 60 deg; lat(P) = 60-1e-13 deg", ctx)?;

    RE = 2.0;
    RP = 4.0;
    F = ((RE - RP) / RE);
    LAT = (60.0 * spicelib::RPD(ctx));
    PLAT = ((60.0 - 0.0000000000001) * spicelib::RPD(ctx));
    LON = (30.0 * spicelib::RPD(ctx));
    ALT = 1.0;

    spicelib::GEOREC(LON, PLAT, ALT, RE, F, P.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XREL = LT;

    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"RP = 2*RE; LAT = -60 deg; lat(P) = -70 deg", ctx)?;

    RE = 2.0;
    RP = 4.0;
    F = ((RE - RP) / RE);
    LAT = -(60.0 * spicelib::RPD(ctx));
    PLAT = -(70.0 * spicelib::RPD(ctx));
    LON = (30.0 * spicelib::RPD(ctx));
    ALT = 1.0;

    spicelib::GEOREC(LON, PLAT, ALT, RE, F, P.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XREL = LT;

    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"RP = 2*RE; LAT = -60 deg; lat(P) = -50 deg", ctx)?;

    RE = 2.0;
    RP = 4.0;
    F = ((RE - RP) / RE);
    LAT = -(60.0 * spicelib::RPD(ctx));
    PLAT = -(50.0 * spicelib::RPD(ctx));
    LON = (30.0 * spicelib::RPD(ctx));
    ALT = 1.0;

    spicelib::GEOREC(LON, PLAT, ALT, RE, F, P.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XREL = GT;

    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"RP = 2*RE; LAT = -60 deg; lat(P) = 50 deg", ctx)?;

    RE = 2.0;
    RP = 4.0;
    F = ((RE - RP) / RE);
    LAT = -(60.0 * spicelib::RPD(ctx));
    PLAT = (50.0 * spicelib::RPD(ctx));
    LON = (30.0 * spicelib::RPD(ctx));
    ALT = 1.0;

    spicelib::GEOREC(LON, PLAT, ALT, RE, F, P.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XREL = GT;

    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"RP = 2*RE; LAT = -60 deg; lat(P) = -60-1e-13 deg", ctx)?;

    RE = 2.0;
    RP = 4.0;
    F = ((RE - RP) / RE);
    LAT = -(60.0 * spicelib::RPD(ctx));
    PLAT = -((60.0 + 0.0000000000001) * spicelib::RPD(ctx));
    LON = (30.0 * spicelib::RPD(ctx));
    ALT = 1.0;

    spicelib::GEOREC(LON, PLAT, ALT, RE, F, P.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XREL = LT;

    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"RP = 2*RE; LAT = 60 deg; lat(P) = -60+1e-13 deg", ctx)?;

    RE = 2.0;
    RP = 4.0;
    F = ((RE - RP) / RE);
    LAT = -(60.0 * spicelib::RPD(ctx));
    PLAT = -((60.0 - 0.0000000000001) * spicelib::RPD(ctx));
    LON = (30.0 * spicelib::RPD(ctx));
    ALT = 1.0;

    spicelib::GEOREC(LON, PLAT, ALT, RE, F, P.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XREL = GT;

    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //***********************************************************************
    //
    //     Basic tests for oblate spheroids
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"RP = RE/2; LAT = 60 deg; lat(P) = 70 deg", ctx)?;

    RE = 8.0;
    RP = 4.0;
    F = ((RE - RP) / RE);
    LAT = (60.0 * spicelib::RPD(ctx));
    PLAT = (70.0 * spicelib::RPD(ctx));
    LON = (30.0 * spicelib::RPD(ctx));
    ALT = 1.0;

    spicelib::GEOREC(LON, PLAT, ALT, RE, F, P.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XREL = GT;

    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"RP = RE/2; LAT = 60 deg; lat(P) = 50 deg", ctx)?;

    RE = 8.0;
    RP = 4.0;
    F = ((RE - RP) / RE);
    LAT = (60.0 * spicelib::RPD(ctx));
    PLAT = (50.0 * spicelib::RPD(ctx));
    LON = (30.0 * spicelib::RPD(ctx));
    ALT = 1.0;

    spicelib::GEOREC(LON, PLAT, ALT, RE, F, P.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XREL = LT;

    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"RP = RE/2; LAT = 60 deg; lat(P) = -50 deg", ctx)?;

    RE = 8.0;
    RP = 4.0;
    F = ((RE - RP) / RE);
    LAT = (60.0 * spicelib::RPD(ctx));
    PLAT = -(50.0 * spicelib::RPD(ctx));
    LON = (30.0 * spicelib::RPD(ctx));
    ALT = 1.0;

    spicelib::GEOREC(LON, PLAT, ALT, RE, F, P.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XREL = LT;

    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"RP = RE/2; LAT = 60 deg; lat(P) = 60+1e-13 deg", ctx)?;

    RE = 8.0;
    RP = 4.0;
    F = ((RE - RP) / RE);
    LAT = (60.0 * spicelib::RPD(ctx));
    PLAT = ((60.0 + 0.0000000000001) * spicelib::RPD(ctx));
    LON = (30.0 * spicelib::RPD(ctx));
    ALT = 1.0;

    spicelib::GEOREC(LON, PLAT, ALT, RE, F, P.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XREL = GT;

    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"RP = RE/2; LAT = 60 deg; lat(P) = 60-1e-13 deg", ctx)?;

    RE = 8.0;
    RP = 4.0;
    F = ((RE - RP) / RE);
    LAT = (60.0 * spicelib::RPD(ctx));
    PLAT = ((60.0 - 0.0000000000001) * spicelib::RPD(ctx));
    LON = (30.0 * spicelib::RPD(ctx));
    ALT = 1.0;

    spicelib::GEOREC(LON, PLAT, ALT, RE, F, P.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XREL = LT;

    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"RP = RE/2; LAT = -60 deg; lat(P) = -70 deg", ctx)?;

    RE = 8.0;
    RP = 4.0;
    F = ((RE - RP) / RE);
    LAT = -(60.0 * spicelib::RPD(ctx));
    PLAT = -(70.0 * spicelib::RPD(ctx));
    LON = (30.0 * spicelib::RPD(ctx));
    ALT = 1.0;

    spicelib::GEOREC(LON, PLAT, ALT, RE, F, P.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XREL = LT;

    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"RP = RE/2; LAT = -60 deg; lat(P) = -50 deg", ctx)?;

    RE = 8.0;
    RP = 4.0;
    F = ((RE - RP) / RE);
    LAT = -(60.0 * spicelib::RPD(ctx));
    PLAT = -(50.0 * spicelib::RPD(ctx));
    LON = (30.0 * spicelib::RPD(ctx));
    ALT = 1.0;

    spicelib::GEOREC(LON, PLAT, ALT, RE, F, P.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XREL = GT;

    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"RP = RE/2; LAT = -60 deg; lat(P) = 50 deg", ctx)?;

    RE = 8.0;
    RP = 4.0;
    F = ((RE - RP) / RE);
    LAT = -(60.0 * spicelib::RPD(ctx));
    PLAT = (50.0 * spicelib::RPD(ctx));
    LON = (30.0 * spicelib::RPD(ctx));
    ALT = 1.0;

    spicelib::GEOREC(LON, PLAT, ALT, RE, F, P.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XREL = GT;

    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"RP = RE/2; LAT = -60 deg; lat(P) = -60-1e-13 deg", ctx)?;

    RE = 8.0;
    RP = 4.0;
    F = ((RE - RP) / RE);
    LAT = -(60.0 * spicelib::RPD(ctx));
    PLAT = -((60.0 + 0.0000000000001) * spicelib::RPD(ctx));
    LON = (30.0 * spicelib::RPD(ctx));
    ALT = 1.0;

    spicelib::GEOREC(LON, PLAT, ALT, RE, F, P.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XREL = LT;

    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"RP = RE/2; LAT = 60 deg; lat(P) = -60+1e-13 deg", ctx)?;

    RE = 8.0;
    RP = 4.0;
    F = ((RE - RP) / RE);
    LAT = -(60.0 * spicelib::RPD(ctx));
    PLAT = -((60.0 - 0.0000000000001) * spicelib::RPD(ctx));
    LON = (30.0 * spicelib::RPD(ctx));
    ALT = 1.0;

    spicelib::GEOREC(LON, PLAT, ALT, RE, F, P.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XREL = GT;

    spicelib::ZZPDCMPL(RE, F, P.as_slice(), LAT, &mut REL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"REL", REL, b"=", XREL, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
