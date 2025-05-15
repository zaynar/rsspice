//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const VTIGHT: f64 = 0.00000000000001;
const TIGHT: f64 = 0.000000000001;

//$Procedure      F_ZZSGLATX ( Test segment latitude extent routine )
pub fn F_ZZSGLATX(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut DIFF = StackArray::<f64, 3>::new(1..=3);
    let mut DIR = StackArray::<f64, 3>::new(1..=3);
    let mut DIST: f64 = 0.0;
    let mut DP: f64 = 0.0;
    let mut MAG: f64 = 0.0;
    let mut MAXLAT: f64 = 0.0;
    let mut MAXP = StackArray::<f64, 3>::new(1..=3);
    let mut MINLAT: f64 = 0.0;
    let mut MINP = StackArray::<f64, 3>::new(1..=3);
    let mut P1 = StackArray::<f64, 3>::new(1..=3);
    let mut P2 = StackArray::<f64, 3>::new(1..=3);
    let mut SQ2: f64 = 0.0;
    let mut SQ22: f64 = 0.0;
    let mut XMAXLT: f64 = 0.0;
    let mut XMAXP = StackArray::<f64, 3>::new(1..=3);
    let mut XMINLT: f64 = 0.0;
    let mut XMINP = StackArray::<f64, 3>::new(1..=3);

    //
    // Test Utility Functions
    //

    //
    // SPICELIB functions
    //
    //
    // Local parameters
    //

    // INTEGER               LNSIZE
    // PARAMETER           ( LNSIZE = 80 )

    //
    // Local Variables
    //
    //  CHARACTER*(LNSIZE)    CASNAM

    // INTEGER               I
    // INTEGER               N

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZSGLATX", ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Tangent case: segment parallel to X-Y plane, Z>0", ctx)?;

    SQ2 = f64::sqrt(2.0);
    SQ22 = (SQ2 / 2.0);

    spicelib::VPACK(1.0, -SQ2, 1.0, P1.as_slice_mut());
    spicelib::VPACK(1.0, SQ2, 1.0, P2.as_slice_mut());

    spicelib::ZZSGLATX(
        P1.as_slice(),
        P2.as_slice(),
        &mut MINLAT,
        MINP.as_slice_mut(),
        &mut MAXLAT,
        MAXP.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The maximum latitude should occur at the segment midpoint.
    //
    spicelib::VPACK(1.0, 0.0, 1.0, XMAXP.as_slice_mut());

    testutil::CHCKAD(
        b"MAXP",
        MAXP.as_slice(),
        b"~~/",
        XMAXP.as_slice(),
        3,
        VTIGHT,
        OK,
        ctx,
    )?;

    XMAXLT = (spicelib::PI(ctx) / 4 as f64);

    testutil::CHCKSD(b"MAXLAT", MAXLAT, b"~", XMAXLT, VTIGHT, OK, ctx)?;

    //
    // The minimum latitude occurs at an endpoint, but which one is
    // not specified. We use the sign of the Y component of MINP
    // to indicate which endpoint was selected.
    //
    spicelib::VEQU(P1.as_slice(), XMINP.as_slice_mut());

    if (MINP[2] > 0.0) {
        XMINP[2] = -XMINP[2];
    }

    testutil::CHCKAD(
        b"MINP",
        MINP.as_slice(),
        b"~~/",
        XMINP.as_slice(),
        3,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // The minimum latitude has sine = 1/2, so the latitude is 30 deg.
    //
    XMINLT = (spicelib::PI(ctx) / 6 as f64);

    testutil::CHCKSD(b"MINLAT", MINLAT, b"~", XMINLT, VTIGHT, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"Tangent case: segment parallel to X-Y plane, Z>0, segment max Y is at X-Z plane.",
        ctx,
    )?;

    spicelib::VPACK(1.0, -SQ2, 1.0, P1.as_slice_mut());
    spicelib::VPACK(1.0, 0.0, 1.0, P2.as_slice_mut());

    spicelib::ZZSGLATX(
        P1.as_slice(),
        P2.as_slice(),
        &mut MINLAT,
        MINP.as_slice_mut(),
        &mut MAXLAT,
        MAXP.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The maximum latitude should occur at the segment midpoint.
    //
    spicelib::VPACK(1.0, 0.0, 1.0, XMAXP.as_slice_mut());

    testutil::CHCKAD(
        b"MAXP",
        MAXP.as_slice(),
        b"~~/",
        XMAXP.as_slice(),
        3,
        VTIGHT,
        OK,
        ctx,
    )?;

    XMAXLT = (spicelib::PI(ctx) / 4 as f64);

    testutil::CHCKSD(b"MAXLAT", MAXLAT, b"~", XMAXLT, VTIGHT, OK, ctx)?;

    //
    // The minimum latitude occurs at the endpoint where Y is
    // minimized.
    //
    spicelib::VEQU(P1.as_slice(), XMINP.as_slice_mut());

    testutil::CHCKAD(
        b"MINP",
        MINP.as_slice(),
        b"~~/",
        XMINP.as_slice(),
        3,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // The minimum latitude has sine = 1/2, so the latitude is 30 deg.
    //
    XMINLT = (spicelib::PI(ctx) / 6 as f64);

    testutil::CHCKSD(b"MINLAT", MINLAT, b"~", XMINLT, VTIGHT, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"Tangent case: segment parallel to X-Y plane, Z>0, segment min Y is at X-Z plane.",
        ctx,
    )?;

    spicelib::VPACK(1.0, 0.0, 1.0, P1.as_slice_mut());
    spicelib::VPACK(1.0, SQ2, 1.0, P2.as_slice_mut());

    spicelib::ZZSGLATX(
        P1.as_slice(),
        P2.as_slice(),
        &mut MINLAT,
        MINP.as_slice_mut(),
        &mut MAXLAT,
        MAXP.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The maximum latitude should occur at the segment starting
    // point.
    //
    spicelib::VPACK(1.0, 0.0, 1.0, XMAXP.as_slice_mut());

    testutil::CHCKAD(
        b"MAXP",
        MAXP.as_slice(),
        b"~~/",
        XMAXP.as_slice(),
        3,
        VTIGHT,
        OK,
        ctx,
    )?;

    XMAXLT = (spicelib::PI(ctx) / 4 as f64);

    testutil::CHCKSD(b"MAXLAT", MAXLAT, b"~", XMAXLT, VTIGHT, OK, ctx)?;

    //
    // The minimum latitude occurs at the endpoint where Y is
    // maximized.
    //
    spicelib::VEQU(P2.as_slice(), XMINP.as_slice_mut());

    testutil::CHCKAD(
        b"MINP",
        MINP.as_slice(),
        b"~~/",
        XMINP.as_slice(),
        3,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // The minimum latitude has sine = 1/2, so the latitude is 30 deg.
    //
    XMINLT = (spicelib::PI(ctx) / 6 as f64);

    testutil::CHCKSD(b"MINLAT", MINLAT, b"~", XMINLT, VTIGHT, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //

    //
    // Adding multiples of the tangent point to the line segment
    // generates new segments in the same tangent plane. Try such
    // a case.
    //
    testutil::TCASE(
        b"Tangent case: segment non parallel to X-Y plane, Z>0, segment min Y is negative.",
        ctx,
    )?;

    spicelib::VPACK(1.0, -SQ2, 1.0, P1.as_slice_mut());

    spicelib::VPACK(3.0, SQ2, 3.0, P2.as_slice_mut());

    spicelib::ZZSGLATX(
        P1.as_slice(),
        P2.as_slice(),
        &mut MINLAT,
        MINP.as_slice_mut(),
        &mut MAXLAT,
        MAXP.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The maximum latitude should occur at the segment midpoint.
    //
    spicelib::VLCOM(0.5, P1.as_slice(), 0.5, P2.as_slice(), XMAXP.as_slice_mut());

    testutil::CHCKAD(
        b"MAXP",
        MAXP.as_slice(),
        b"~~/",
        XMAXP.as_slice(),
        3,
        VTIGHT,
        OK,
        ctx,
    )?;

    XMAXLT = (spicelib::PI(ctx) / 4 as f64);

    testutil::CHCKSD(b"MAXLAT", MAXLAT, b"~", XMAXLT, VTIGHT, OK, ctx)?;

    //
    // The minimum latitude will occur at the endpoint at which Y
    // is minimum.
    //
    spicelib::VEQU(P1.as_slice(), XMINP.as_slice_mut());

    testutil::CHCKAD(
        b"MINP",
        MINP.as_slice(),
        b"~~/",
        XMINP.as_slice(),
        3,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // The minimum latitude has sine = 1/2, so the latitude is 30 deg.
    //
    XMINLT = (spicelib::PI(ctx) / 6 as f64);

    testutil::CHCKSD(b"MINLAT", MINLAT, b"~", XMINLT, VTIGHT, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"Tangent case: segment parallel to X-Y plane, Z>0, segment max Y is at X-Z plane.",
        ctx,
    )?;

    spicelib::VPACK(1.0, -SQ2, 1.0, P1.as_slice_mut());
    spicelib::VPACK(1.0, 0.0, 1.0, P2.as_slice_mut());

    spicelib::ZZSGLATX(
        P1.as_slice(),
        P2.as_slice(),
        &mut MINLAT,
        MINP.as_slice_mut(),
        &mut MAXLAT,
        MAXP.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The maximum latitude should occur at the segment midpoint.
    //
    spicelib::VPACK(1.0, 0.0, 1.0, XMAXP.as_slice_mut());

    testutil::CHCKAD(
        b"MAXP",
        MAXP.as_slice(),
        b"~~/",
        XMAXP.as_slice(),
        3,
        VTIGHT,
        OK,
        ctx,
    )?;

    XMAXLT = (spicelib::PI(ctx) / 4 as f64);

    testutil::CHCKSD(b"MAXLAT", MAXLAT, b"~", XMAXLT, VTIGHT, OK, ctx)?;

    //
    // The minimum latitude occurs at the endpoint where Y is
    // minimized.
    //
    spicelib::VEQU(P1.as_slice(), XMINP.as_slice_mut());

    testutil::CHCKAD(
        b"MINP",
        MINP.as_slice(),
        b"~~/",
        XMINP.as_slice(),
        3,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // The minimum latitude has sine = 1/2, so the latitude is 30 deg.
    //
    XMINLT = (spicelib::PI(ctx) / 6 as f64);

    testutil::CHCKSD(b"MINLAT", MINLAT, b"~", XMINLT, VTIGHT, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"Tangent *miss* case: segment parallel to X-Y plane, Z>0, segment max Y < 0.",
        ctx,
    )?;

    spicelib::VPACK(1.0, -2.0, 1.0, P1.as_slice_mut());
    spicelib::VPACK(1.0, -SQ2, 1.0, P2.as_slice_mut());

    spicelib::ZZSGLATX(
        P1.as_slice(),
        P2.as_slice(),
        &mut MINLAT,
        MINP.as_slice_mut(),
        &mut MAXLAT,
        MAXP.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The maximum latitude should occur at P2.
    //
    spicelib::VEQU(P2.as_slice(), XMAXP.as_slice_mut());

    testutil::CHCKAD(
        b"MAXP",
        MAXP.as_slice(),
        b"~~/",
        XMAXP.as_slice(),
        3,
        VTIGHT,
        OK,
        ctx,
    )?;

    XMAXLT = (spicelib::PI(ctx) / 6 as f64);

    testutil::CHCKSD(b"MAXLAT", MAXLAT, b"~", XMAXLT, VTIGHT, OK, ctx)?;

    //
    // The minimum latitude occurs at the endpoint where Y is
    // minimized.
    //
    spicelib::VEQU(P1.as_slice(), XMINP.as_slice_mut());

    testutil::CHCKAD(
        b"MINP",
        MINP.as_slice(),
        b"~~/",
        XMINP.as_slice(),
        3,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // The minimum latitude has sine = 1/sqrt(6).
    //
    XMINLT = f64::asin((1.0 / f64::sqrt(6.0)));

    testutil::CHCKSD(b"MINLAT", MINLAT, b"~", XMINLT, VTIGHT, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"P2 = 2 * P1", ctx)?;

    spicelib::VPACK(SQ22, -SQ22, 1.0, P1.as_slice_mut());
    spicelib::VSCL(2.0, P1.as_slice(), P2.as_slice_mut());

    spicelib::ZZSGLATX(
        P1.as_slice(),
        P2.as_slice(),
        &mut MINLAT,
        MINP.as_slice_mut(),
        &mut MAXLAT,
        MAXP.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The maximum latitude will be found somewhere on the
    // line segment. Find the nearest point to MAXP on the line
    // containing the segment; make sure this point is actually
    // on the segment.
    //
    spicelib::VSUB(P2.as_slice(), P1.as_slice(), DIFF.as_slice_mut());
    spicelib::UNORM(DIFF.as_slice(), DIR.as_slice_mut(), &mut MAG);

    spicelib::NPLNPT(
        P1.as_slice(),
        DIR.as_slice(),
        MAXP.as_slice(),
        XMAXP.as_slice_mut(),
        &mut DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"DIST", DIST, b"~", 0.0, TIGHT, OK, ctx)?;

    spicelib::VSUB(MAXP.as_slice(), P1.as_slice(), DIFF.as_slice_mut());

    DP = spicelib::VDOT(DIFF.as_slice(), DIR.as_slice());

    testutil::CHCKSD(b"DP", DP, b">", -VTIGHT, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"DP", DP, b"<", (MAG + VTIGHT), 0.0, OK, ctx)?;

    //
    // The same conditions apply to MINP.
    //
    spicelib::NPLNPT(
        P1.as_slice(),
        DIR.as_slice(),
        MINP.as_slice(),
        XMINP.as_slice_mut(),
        &mut DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"DIST", DIST, b"~", 0.0, TIGHT, OK, ctx)?;

    spicelib::VSUB(MINP.as_slice(), P1.as_slice(), DIFF.as_slice_mut());

    DP = spicelib::VDOT(DIFF.as_slice(), DIR.as_slice());

    testutil::CHCKSD(b"DP", DP, b">", -VTIGHT, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"DP", DP, b"<", (MAG + VTIGHT), 0.0, OK, ctx)?;

    //
    // Latitude is pi/4 all along this segment.
    //
    XMAXLT = (spicelib::PI(ctx) / 4 as f64);

    testutil::CHCKSD(b"MAXLAT", MAXLAT, b"~", XMAXLT, VTIGHT, OK, ctx)?;

    XMINLT = (spicelib::PI(ctx) / 4 as f64);

    testutil::CHCKSD(b"MINLAT", MINLAT, b"~", XMINLT, VTIGHT, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"P2 = - P1", ctx)?;

    spicelib::VPACK(SQ22, -SQ22, 1.0, P1.as_slice_mut());
    spicelib::VMINUS(P1.as_slice(), P2.as_slice_mut());

    spicelib::ZZSGLATX(
        P1.as_slice(),
        P2.as_slice(),
        &mut MINLAT,
        MINP.as_slice_mut(),
        &mut MAXLAT,
        MAXP.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The maximum latitude will be found somewhere on the
    // line segment. Find the nearest point to MAXP on the line
    // containing the segment; make sure this point is actually
    // on the segment.
    //
    spicelib::VSUB(P2.as_slice(), P1.as_slice(), DIFF.as_slice_mut());
    spicelib::UNORM(DIFF.as_slice(), DIR.as_slice_mut(), &mut MAG);

    spicelib::NPLNPT(
        P1.as_slice(),
        DIR.as_slice(),
        MAXP.as_slice(),
        XMAXP.as_slice_mut(),
        &mut DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"DIST", DIST, b"~", 0.0, TIGHT, OK, ctx)?;

    spicelib::VSUB(MAXP.as_slice(), P1.as_slice(), DIFF.as_slice_mut());

    DP = spicelib::VDOT(DIFF.as_slice(), DIR.as_slice());

    testutil::CHCKSD(b"DP", DP, b">", -VTIGHT, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"DP", DP, b"<", (MAG + VTIGHT), 0.0, OK, ctx)?;

    //
    // The same conditions apply to MINP.
    //
    spicelib::NPLNPT(
        P1.as_slice(),
        DIR.as_slice(),
        MINP.as_slice(),
        XMINP.as_slice_mut(),
        &mut DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"DIST", DIST, b"~", 0.0, TIGHT, OK, ctx)?;

    spicelib::VSUB(MINP.as_slice(), P1.as_slice(), DIFF.as_slice_mut());

    DP = spicelib::VDOT(DIFF.as_slice(), DIR.as_slice());

    testutil::CHCKSD(b"DP", DP, b">", -VTIGHT, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"DP", DP, b"<", (MAG + VTIGHT), 0.0, OK, ctx)?;

    //
    // Latitude is pi/4 on portion of this segment above the plane Z=0,
    // -pi/4 below it.
    //
    XMAXLT = (spicelib::PI(ctx) / 4 as f64);

    testutil::CHCKSD(b"MAXLAT", MAXLAT, b"~", XMAXLT, VTIGHT, OK, ctx)?;

    XMINLT = -(spicelib::PI(ctx) / 4 as f64);

    testutil::CHCKSD(b"MINLAT", MINLAT, b"~", XMINLT, VTIGHT, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Segment is in X-Y plane.", ctx)?;

    spicelib::VPACK(SQ22, -SQ22, 0.0, P1.as_slice_mut());
    spicelib::VPACK(SQ22, SQ22, 0.0, P2.as_slice_mut());

    spicelib::ZZSGLATX(
        P1.as_slice(),
        P2.as_slice(),
        &mut MINLAT,
        MINP.as_slice_mut(),
        &mut MAXLAT,
        MAXP.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The maximum latitude will be found somewhere on the
    // line segment. Find the nearest point to MAXP on the line
    // containing the segment; make sure this point is actually
    // on the segment.
    //
    spicelib::VSUB(P2.as_slice(), P1.as_slice(), DIFF.as_slice_mut());
    spicelib::UNORM(DIFF.as_slice(), DIR.as_slice_mut(), &mut MAG);

    spicelib::NPLNPT(
        P1.as_slice(),
        DIR.as_slice(),
        MAXP.as_slice(),
        XMAXP.as_slice_mut(),
        &mut DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"DIST", DIST, b"~", 0.0, TIGHT, OK, ctx)?;

    spicelib::VSUB(MAXP.as_slice(), P1.as_slice(), DIFF.as_slice_mut());

    DP = spicelib::VDOT(DIFF.as_slice(), DIR.as_slice());

    testutil::CHCKSD(b"DP", DP, b">", -VTIGHT, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"DP", DP, b"<", (MAG + VTIGHT), 0.0, OK, ctx)?;

    //
    // The same conditions apply to MINP.
    //
    spicelib::NPLNPT(
        P1.as_slice(),
        DIR.as_slice(),
        MINP.as_slice(),
        XMINP.as_slice_mut(),
        &mut DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"DIST", DIST, b"~", 0.0, TIGHT, OK, ctx)?;

    spicelib::VSUB(MINP.as_slice(), P1.as_slice(), DIFF.as_slice_mut());

    DP = spicelib::VDOT(DIFF.as_slice(), DIR.as_slice());

    testutil::CHCKSD(b"DP", DP, b">", -VTIGHT, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"DP", DP, b"<", (MAG + VTIGHT), 0.0, OK, ctx)?;

    //
    // Latitude is 0 on the whole segment.
    //
    XMAXLT = 0.0;

    testutil::CHCKSD(b"MAXLAT", MAXLAT, b"~", XMAXLT, VTIGHT, OK, ctx)?;

    XMINLT = 0.0;

    testutil::CHCKSD(b"MINLAT", MINLAT, b"~", XMINLT, VTIGHT, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Segment is contained in +Z axis", ctx)?;

    spicelib::VPACK(0.0, 0.0, 1.0, P2.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 2.0, P1.as_slice_mut());

    spicelib::ZZSGLATX(
        P1.as_slice(),
        P2.as_slice(),
        &mut MINLAT,
        MINP.as_slice_mut(),
        &mut MAXLAT,
        MAXP.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The maximum latitude will be found somewhere on the
    // line segment. Find the nearest point to MAXP on the line
    // containing the segment; make sure this point is actually
    // on the segment.
    //
    spicelib::VSUB(P2.as_slice(), P1.as_slice(), DIFF.as_slice_mut());
    spicelib::UNORM(DIFF.as_slice(), DIR.as_slice_mut(), &mut MAG);

    spicelib::NPLNPT(
        P1.as_slice(),
        DIR.as_slice(),
        MAXP.as_slice(),
        XMAXP.as_slice_mut(),
        &mut DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"DIST", DIST, b"~", 0.0, TIGHT, OK, ctx)?;

    spicelib::VSUB(MAXP.as_slice(), P1.as_slice(), DIFF.as_slice_mut());

    DP = spicelib::VDOT(DIFF.as_slice(), DIR.as_slice());

    testutil::CHCKSD(b"DP", DP, b">", -VTIGHT, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"DP", DP, b"<", (MAG + VTIGHT), 0.0, OK, ctx)?;

    //
    // The same conditions apply to MINP.
    //
    spicelib::NPLNPT(
        P1.as_slice(),
        DIR.as_slice(),
        MINP.as_slice(),
        XMINP.as_slice_mut(),
        &mut DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"DIST", DIST, b"~", 0.0, TIGHT, OK, ctx)?;

    spicelib::VSUB(MINP.as_slice(), P1.as_slice(), DIFF.as_slice_mut());

    DP = spicelib::VDOT(DIFF.as_slice(), DIR.as_slice());

    testutil::CHCKSD(b"DP", DP, b">", -VTIGHT, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"DP", DP, b"<", (MAG + VTIGHT), 0.0, OK, ctx)?;

    //
    // Latitude is pi/2 on the whole segment.
    //
    XMAXLT = (spicelib::PI(ctx) / 2 as f64);

    testutil::CHCKSD(b"MAXLAT", MAXLAT, b"~", XMAXLT, VTIGHT, OK, ctx)?;

    XMINLT = (spicelib::PI(ctx) / 2 as f64);

    testutil::CHCKSD(b"MINLAT", MINLAT, b"~", XMINLT, VTIGHT, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Segment is contained in -Z axis", ctx)?;

    spicelib::VPACK(0.0, 0.0, -1.0, P1.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, -2.0, P2.as_slice_mut());

    spicelib::ZZSGLATX(
        P1.as_slice(),
        P2.as_slice(),
        &mut MINLAT,
        MINP.as_slice_mut(),
        &mut MAXLAT,
        MAXP.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The maximum latitude will be found somewhere on the
    // line segment. Find the nearest point to MAXP on the line
    // containing the segment; make sure this point is actually
    // on the segment.
    //
    spicelib::VSUB(P2.as_slice(), P1.as_slice(), DIFF.as_slice_mut());
    spicelib::UNORM(DIFF.as_slice(), DIR.as_slice_mut(), &mut MAG);

    spicelib::NPLNPT(
        P1.as_slice(),
        DIR.as_slice(),
        MAXP.as_slice(),
        XMAXP.as_slice_mut(),
        &mut DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"DIST", DIST, b"~", 0.0, TIGHT, OK, ctx)?;

    spicelib::VSUB(MAXP.as_slice(), P1.as_slice(), DIFF.as_slice_mut());

    DP = spicelib::VDOT(DIFF.as_slice(), DIR.as_slice());

    testutil::CHCKSD(b"DP", DP, b">", -VTIGHT, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"DP", DP, b"<", (MAG + VTIGHT), 0.0, OK, ctx)?;

    //
    // The same conditions apply to MINP.
    //
    spicelib::NPLNPT(
        P1.as_slice(),
        DIR.as_slice(),
        MINP.as_slice(),
        XMINP.as_slice_mut(),
        &mut DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"DIST", DIST, b"~", 0.0, TIGHT, OK, ctx)?;

    spicelib::VSUB(MINP.as_slice(), P1.as_slice(), DIFF.as_slice_mut());

    DP = spicelib::VDOT(DIFF.as_slice(), DIR.as_slice());

    testutil::CHCKSD(b"DP", DP, b">", -VTIGHT, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"DP", DP, b"<", (MAG + VTIGHT), 0.0, OK, ctx)?;

    //
    // Latitude is -pi/2 on the whole segment.
    //
    XMAXLT = -(spicelib::PI(ctx) / 2 as f64);

    testutil::CHCKSD(b"MAXLAT", MAXLAT, b"~", XMAXLT, VTIGHT, OK, ctx)?;

    XMINLT = -(spicelib::PI(ctx) / 2 as f64);

    testutil::CHCKSD(b"MINLAT", MINLAT, b"~", XMINLT, VTIGHT, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Segment intersects but is not contained in +Z axis", ctx)?;

    spicelib::VPACK(1.0, 0.0, 1.0, P2.as_slice_mut());
    spicelib::VPACK(-1.0, 0.0, 1.0, P1.as_slice_mut());

    spicelib::ZZSGLATX(
        P1.as_slice(),
        P2.as_slice(),
        &mut MINLAT,
        MINP.as_slice_mut(),
        &mut MAXLAT,
        MAXP.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The maximum latitude will be found on the Z axis.
    //
    spicelib::VPACK(0.0, 0.0, 1.0, XMAXP.as_slice_mut());

    testutil::CHCKAD(
        b"MAXP",
        MAXP.as_slice(),
        b"~~/",
        XMAXP.as_slice(),
        3,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // The minimum latitude will be found at one of the
    // segment's endpoints.
    //
    spicelib::VEQU(P1.as_slice(), XMINP.as_slice_mut());

    if (MINP[1] > 0.0) {
        XMINP[1] = -XMINP[1];
    }

    testutil::CHCKAD(
        b"MINP",
        MINP.as_slice(),
        b"~~/",
        XMINP.as_slice(),
        3,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Latitude is pi/2 at the maximum.
    //
    XMAXLT = (spicelib::PI(ctx) / 2 as f64);

    testutil::CHCKSD(b"MAXLAT", MAXLAT, b"~", XMAXLT, VTIGHT, OK, ctx)?;

    //
    // Latitude is pi/4 at the minimum.
    //
    XMINLT = (spicelib::PI(ctx) / 4 as f64);

    testutil::CHCKSD(b"MINLAT", MINLAT, b"~", XMINLT, VTIGHT, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Segment intersects but is not contained in -Z axis", ctx)?;

    spicelib::VPACK(1.0, 0.0, -1.0, P2.as_slice_mut());
    spicelib::VPACK(-1.0, 0.0, -1.0, P1.as_slice_mut());

    spicelib::ZZSGLATX(
        P1.as_slice(),
        P2.as_slice(),
        &mut MINLAT,
        MINP.as_slice_mut(),
        &mut MAXLAT,
        MAXP.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The minimum latitude will be found on the Z axis.
    //
    spicelib::VPACK(0.0, 0.0, -1.0, XMINP.as_slice_mut());

    testutil::CHCKAD(
        b"MINP",
        MINP.as_slice(),
        b"~~/",
        XMINP.as_slice(),
        3,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // The maximum latitude will be found at one of the
    // segment's endpoints.
    //
    spicelib::VEQU(P1.as_slice(), XMAXP.as_slice_mut());

    if (MAXP[1] > 0.0) {
        XMAXP[1] = -XMAXP[1];
    }

    testutil::CHCKAD(
        b"MAXP",
        MAXP.as_slice(),
        b"~~/",
        XMAXP.as_slice(),
        3,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Latitude is -pi/2 at the minimum.
    //
    XMINLT = -(spicelib::PI(ctx) / 2 as f64);

    testutil::CHCKSD(b"MINLAT", MINLAT, b"~", XMINLT, VTIGHT, OK, ctx)?;

    //
    // Latitude is -pi/4 at the maximum.
    //
    XMAXLT = -(spicelib::PI(ctx) / 4 as f64);

    testutil::CHCKSD(b"MAXLAT", MAXLAT, b"~", XMAXLT, VTIGHT, OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
