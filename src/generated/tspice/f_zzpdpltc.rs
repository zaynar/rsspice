//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LIMIT: f64 = 0.01;

//$Procedure F_ZZPDPLTC ( ZZPDPLTC tests )
pub fn F_ZZPDPLTC(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut F: f64 = 0.0;
    let mut LAT: f64 = 0.0;
    let mut P = StackArray::<f64, 3>::new(1..=3);
    let mut RE: f64 = 0.0;
    let mut RP: f64 = 0.0;
    let mut S: f64 = 0.0;
    let mut XXPT: f64 = 0.0;
    let mut XYPT: f64 = 0.0;
    let mut ISON: bool = false;
    let mut XISON: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //

    //
    // This value must match that in ZZPDPLTC.
    //

    //
    // Local Variables
    //
    //

    //
    // Saved values
    //
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_ZZPDPLTC", ctx)?;

    //***********************************************************************
    //
    //     Error cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"RE is zero.", ctx)?;

    RE = 0.0;
    F = 0.0;
    LAT = 0.0;

    P[1] = 1.0;
    P[2] = 1.0;
    P[3] = 1.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"RE is negative.", ctx)?;

    RE = -1.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = (spicelib::PI(ctx) / 4 as f64);

    P[1] = 1.0;
    P[2] = 1.0;
    P[3] = 1.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"F > 1.", ctx)?;

    RE = 3.0;
    RP = 2.0;
    F = 2.0;
    LAT = (spicelib::PI(ctx) / 4 as f64);

    P[1] = 1.0;
    P[2] = 1.0;
    P[3] = 1.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //***********************************************************************
    //
    //     Oblate tests
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Oblate case; LAT = PI/4; P(3) > 0", ctx)?;

    RE = 4.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = (spicelib::PI(ctx) / 4 as f64);

    P[1] = 1.0;
    P[2] = 1.0;
    P[3] = 1.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Oblate case; LAT = -PI/4; P(3) > 0", ctx)?;

    LAT = -(spicelib::PI(ctx) / 4 as f64);

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = false;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Oblate case; LAT = 0; P(3) > 0", ctx)?;

    LAT = 0.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Oblate case; LAT = PI/4; P(3) < 0", ctx)?;

    RE = 4.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = (spicelib::PI(ctx) / 4 as f64);

    P[1] = 1.0;
    P[2] = 1.0;
    P[3] = -1.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = false;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Oblate case; LAT = -PI/4; P(3) < 0", ctx)?;

    LAT = -(spicelib::PI(ctx) / 4 as f64);

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Oblate case; LAT = 0; P(3) < 0", ctx)?;

    LAT = 0.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Oblate case; LAT = PI/4; P(3) = 0", ctx)?;

    RE = 4.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = (spicelib::PI(ctx) / 4 as f64);

    P[1] = 1.0;
    P[2] = 1.0;
    P[3] = 0.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Oblate case; LAT = -PI/4; P(3) = 0", ctx)?;

    LAT = -(spicelib::PI(ctx) / 4 as f64);

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Oblate case; LAT = 0; P(3) = 0", ctx)?;

    LAT = 0.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Oblate case; LAT = LIMIT/2; P(3) > 0, radius > XXPT.", ctx)?;

    RE = 4.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = (LIMIT / 2 as f64);

    spicelib::ZZELNAXX(RE, F, LAT, &mut XXPT, &mut XYPT, ctx)?;

    S = (f64::sqrt(2.0) / 2 as f64);

    P[1] = ((S * XXPT) * 2 as f64);
    P[2] = ((S * XXPT) * 2 as f64);
    P[3] = 1.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Oblate case; LAT = LIMIT/2; P(3) > 0, radius < XXPT.", ctx)?;

    RE = 4.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = (LIMIT / 2 as f64);

    spicelib::ZZELNAXX(RE, F, LAT, &mut XXPT, &mut XYPT, ctx)?;

    S = (f64::sqrt(2.0) / 2 as f64);

    P[1] = ((S * XXPT) / 2 as f64);
    P[2] = ((S * XXPT) / 2 as f64);
    P[3] = 1.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The point is accepted on the basis of its Z component
    // having the correct sign.
    //
    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Oblate case; LAT = LIMIT/2; P(3) < 0, radius > XXPT.", ctx)?;

    RE = 4.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = (LIMIT / 2 as f64);

    spicelib::ZZELNAXX(RE, F, LAT, &mut XXPT, &mut XYPT, ctx)?;

    S = (f64::sqrt(2.0) / 2 as f64);

    P[1] = ((S * XXPT) * 2 as f64);
    P[2] = ((S * XXPT) * 2 as f64);
    P[3] = -1.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Oblate case; LAT = LIMIT/2; P(3) < 0, radius < XXPT.", ctx)?;

    RE = 4.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = (LIMIT / 2 as f64);

    spicelib::ZZELNAXX(RE, F, LAT, &mut XXPT, &mut XYPT, ctx)?;

    S = (f64::sqrt(2.0) / 2 as f64);

    P[1] = ((S * XXPT) / 2 as f64);
    P[2] = ((S * XXPT) / 2 as f64);
    P[3] = -1.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = false;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Oblate case; LAT = LIMIT/2; P(3) = 0, radius > XXPT.", ctx)?;

    RE = 4.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = (LIMIT / 2 as f64);

    spicelib::ZZELNAXX(RE, F, LAT, &mut XXPT, &mut XYPT, ctx)?;

    S = (f64::sqrt(2.0) / 2 as f64);

    P[1] = ((S * XXPT) * 2 as f64);
    P[2] = ((S * XXPT) * 2 as f64);
    P[3] = 0.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Oblate case; LAT = LIMIT/2; P(3) = 0, radius < XXPT.", ctx)?;

    RE = 4.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = (LIMIT / 2 as f64);

    spicelib::ZZELNAXX(RE, F, LAT, &mut XXPT, &mut XYPT, ctx)?;

    S = (f64::sqrt(2.0) / 2 as f64);

    P[1] = ((S * XXPT) / 2 as f64);
    P[2] = ((S * XXPT) / 2 as f64);
    P[3] = 0.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The point is accepted because its Z component is zero.
    //
    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(
        b"Oblate case; LAT = -LIMIT/2; P(3) = 0, radius > XXPT.",
        ctx,
    )?;

    RE = 4.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = -(LIMIT / 2 as f64);

    spicelib::ZZELNAXX(RE, F, LAT, &mut XXPT, &mut XYPT, ctx)?;

    S = (f64::sqrt(2.0) / 2 as f64);

    P[1] = ((S * XXPT) * 2 as f64);
    P[2] = ((S * XXPT) * 2 as f64);
    P[3] = 0.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(
        b"Oblate case; LAT = -LIMIT/2; P(3) = 0, radius < XXPT.",
        ctx,
    )?;

    RE = 4.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = -(LIMIT / 2 as f64);

    spicelib::ZZELNAXX(RE, F, LAT, &mut XXPT, &mut XYPT, ctx)?;

    S = (f64::sqrt(2.0) / 2 as f64);

    P[1] = ((S * XXPT) / 2 as f64);
    P[2] = ((S * XXPT) / 2 as f64);
    P[3] = 0.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The point is accepted because its Z component is zero.
    //
    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Oblate case; LAT = 0; P(3) = 0, radius > XXPT.", ctx)?;

    RE = 4.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = 0.0;

    spicelib::ZZELNAXX(RE, F, LAT, &mut XXPT, &mut XYPT, ctx)?;

    S = (f64::sqrt(2.0) / 2 as f64);

    P[1] = ((S * XXPT) * 2 as f64);
    P[2] = ((S * XXPT) * 2 as f64);
    P[3] = 0.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Oblate case; LAT = 0.D0; P(3) = 0, radius < XXPT.", ctx)?;

    RE = 4.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = 0.0;

    spicelib::ZZELNAXX(RE, F, LAT, &mut XXPT, &mut XYPT, ctx)?;

    S = (f64::sqrt(2.0) / 2 as f64);

    P[1] = ((S * XXPT) / 2 as f64);
    P[2] = ((S * XXPT) / 2 as f64);
    P[3] = 0.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(
        b"Oblate case; LAT = -LIMIT/2; P(3) > 0, radius > XXPT.",
        ctx,
    )?;

    RE = 4.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = -(LIMIT / 2 as f64);

    spicelib::ZZELNAXX(RE, F, LAT, &mut XXPT, &mut XYPT, ctx)?;

    S = (f64::sqrt(2.0) / 2 as f64);

    P[1] = ((S * XXPT) * 2 as f64);
    P[2] = ((S * XXPT) * 2 as f64);
    P[3] = 1.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(
        b"Oblate case; LAT = -LIMIT/2; P(3) > 0, radius < XXPT.",
        ctx,
    )?;

    RE = 4.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = -(LIMIT / 2 as f64);

    spicelib::ZZELNAXX(RE, F, LAT, &mut XXPT, &mut XYPT, ctx)?;

    S = (f64::sqrt(2.0) / 2 as f64);

    P[1] = ((S * XXPT) / 2 as f64);
    P[2] = ((S * XXPT) / 2 as f64);
    P[3] = 1.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = false;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(
        b"Oblate case; LAT = -LIMIT/2; P(3) < 0, radius > XXPT.",
        ctx,
    )?;

    RE = 4.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = -(LIMIT / 2 as f64);

    spicelib::ZZELNAXX(RE, F, LAT, &mut XXPT, &mut XYPT, ctx)?;

    S = (f64::sqrt(2.0) / 2 as f64);

    P[1] = ((S * XXPT) * 2 as f64);
    P[2] = ((S * XXPT) * 2 as f64);
    P[3] = -1.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(
        b"Oblate case; LAT = -LIMIT/2; P(3) < 0, radius < XXPT.",
        ctx,
    )?;

    RE = 4.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = -(LIMIT / 2 as f64);

    spicelib::ZZELNAXX(RE, F, LAT, &mut XXPT, &mut XYPT, ctx)?;

    S = (f64::sqrt(2.0) / 2 as f64);

    P[1] = ((S * XXPT) / 2 as f64);
    P[2] = ((S * XXPT) / 2 as f64);
    P[3] = -1.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The point is accepted on the basis of the sign of its Z
    // component.
    //
    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Oblate case; LAT = 0; P(3) > 0, radius > XXPT.", ctx)?;

    RE = 4.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = 0.0;

    spicelib::ZZELNAXX(RE, F, LAT, &mut XXPT, &mut XYPT, ctx)?;

    S = (f64::sqrt(2.0) / 2 as f64);

    P[1] = ((S * XXPT) * 2 as f64);
    P[2] = ((S * XXPT) * 2 as f64);
    P[3] = 1.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Oblate case; LAT = 0; P(3) > 0, radius < XXPT.", ctx)?;

    RE = 4.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = 0.0;

    spicelib::ZZELNAXX(RE, F, LAT, &mut XXPT, &mut XYPT, ctx)?;

    S = (f64::sqrt(2.0) / 2 as f64);

    P[1] = ((S * XXPT) / 2 as f64);
    P[2] = ((S * XXPT) / 2 as f64);
    P[3] = 1.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Oblate case; LAT = 0; P(3) < 0, radius > XXPT.", ctx)?;

    RE = 4.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = 0.0;

    spicelib::ZZELNAXX(RE, F, LAT, &mut XXPT, &mut XYPT, ctx)?;

    S = (f64::sqrt(2.0) / 2 as f64);

    P[1] = ((S * XXPT) * 2 as f64);
    P[2] = ((S * XXPT) * 2 as f64);
    P[3] = -1.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Oblate case; LAT = 0; P(3) < 0, radius < XXPT.", ctx)?;

    RE = 4.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = 0.0;

    spicelib::ZZELNAXX(RE, F, LAT, &mut XXPT, &mut XYPT, ctx)?;

    S = (f64::sqrt(2.0) / 2 as f64);

    P[1] = ((S * XXPT) / 2 as f64);
    P[2] = ((S * XXPT) / 2 as f64);
    P[3] = -1.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Oblate case; LAT = 0; P(3) = 0, radius > XXPT.", ctx)?;

    RE = 4.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = 0.0;

    spicelib::ZZELNAXX(RE, F, LAT, &mut XXPT, &mut XYPT, ctx)?;

    S = (f64::sqrt(2.0) / 2 as f64);

    P[1] = ((S * XXPT) * 2 as f64);
    P[2] = ((S * XXPT) * 2 as f64);
    P[3] = 0.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Oblate case; LAT = 0; P(3) = 0, radius < XXPT.", ctx)?;

    RE = 4.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = 0.0;

    spicelib::ZZELNAXX(RE, F, LAT, &mut XXPT, &mut XYPT, ctx)?;

    S = (f64::sqrt(2.0) / 2 as f64);

    P[1] = ((S * XXPT) / 2 as f64);
    P[2] = ((S * XXPT) / 2 as f64);
    P[3] = 0.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //***********************************************************************
    //
    //     Prolate tests
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Prolate case; LAT = PI/4; P(3) > 0", ctx)?;

    RE = 2.0;
    RP = 4.0;
    F = ((RE - RP) / RE);
    LAT = (spicelib::PI(ctx) / 4 as f64);

    P[1] = 1.0;
    P[2] = 1.0;
    P[3] = 1.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Prolate case; LAT = -PI/4; P(3) > 0", ctx)?;

    LAT = -(spicelib::PI(ctx) / 4 as f64);

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note: the prolate case ALWAYS returns .TRUE.
    //
    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Prolate case; LAT = 0; P(3) > 0", ctx)?;

    LAT = 0.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Prolate case; LAT = PI/4; P(3) < 0", ctx)?;

    LAT = (spicelib::PI(ctx) / 4 as f64);

    P[1] = 1.0;
    P[2] = 1.0;
    P[3] = -1.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Prolate case; LAT = -PI/4; P(3) < 0", ctx)?;

    LAT = -(spicelib::PI(ctx) / 4 as f64);

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note: the prolate case ALWAYS returns .TRUE.
    //
    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Prolate case; LAT = 0; P(3) < 0", ctx)?;

    LAT = 0.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Prolate case; LAT = PI/4; P(3) = 0", ctx)?;

    LAT = (spicelib::PI(ctx) / 4 as f64);

    P[1] = 1.0;
    P[2] = 1.0;
    P[3] = 0.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Prolate case; LAT = -PI/4; P(3) = 0", ctx)?;

    LAT = -(spicelib::PI(ctx) / 4 as f64);

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note: the prolate case ALWAYS returns .TRUE.
    //
    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Prolate case; LAT = 0; P(3) = 0", ctx)?;

    LAT = 0.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //***********************************************************************
    //
    //     Sphere tests
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Sphere case; LAT = PI/4; P(3) > 0", ctx)?;

    RE = 2.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = (spicelib::PI(ctx) / 4 as f64);

    P[1] = 1.0;
    P[2] = 1.0;
    P[3] = 1.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Sphere case; LAT = -PI/4; P(3) > 0", ctx)?;

    RE = 2.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = -(spicelib::PI(ctx) / 4 as f64);

    P[1] = 1.0;
    P[2] = 1.0;
    P[3] = 1.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note: the prolate case ALWAYS returns .TRUE.
    //
    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Sphere case; LAT = 0; P(3) > 0", ctx)?;

    RE = 2.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = 0.0;

    P[1] = 1.0;
    P[2] = 1.0;
    P[3] = 1.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Sphere case; LAT = PI/4; P(3) < 0", ctx)?;

    RE = 2.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = (spicelib::PI(ctx) / 4 as f64);

    P[1] = 1.0;
    P[2] = 1.0;
    P[3] = -1.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Sphere case; LAT = -PI/4; P(3) < 0", ctx)?;

    RE = 2.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = -(spicelib::PI(ctx) / 4 as f64);

    P[1] = 1.0;
    P[2] = 1.0;
    P[3] = -1.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note: the prolate case ALWAYS returns .TRUE.
    //
    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Sphere case; LAT = 0; P(3) < 0", ctx)?;

    RE = 2.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = 0.0;

    P[1] = 1.0;
    P[2] = 1.0;
    P[3] = -1.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Sphere case; LAT = PI/4; P(3) = 0", ctx)?;

    RE = 2.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = (spicelib::PI(ctx) / 4 as f64);

    P[1] = 1.0;
    P[2] = 1.0;
    P[3] = 0.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Sphere case; LAT = -PI/4; P(3) = 0", ctx)?;

    RE = 2.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = -(spicelib::PI(ctx) / 4 as f64);

    P[1] = 1.0;
    P[2] = 1.0;
    P[3] = 0.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note: the prolate case ALWAYS returns .TRUE.
    //
    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Sphere case; LAT = 0; P(3) = 0", ctx)?;

    RE = 2.0;
    RP = 2.0;
    F = ((RE - RP) / RE);
    LAT = 0.0;

    P[1] = 1.0;
    P[2] = 1.0;
    P[3] = 0.0;

    ISON = spicelib::ZZPDPLTC(RE, F, P.as_slice(), LAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XISON = true;

    testutil::CHCKSL(b"ISON", ISON, XISON, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
