//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const TIGHT: f64 = 0.00000000000001;
const MEDTOL: f64 = 0.0000000001;
const LOOSE: f64 = 0.0000001;
const SLOPPY: f64 = 0.0001;
const UBEL: i32 = 9;
const LNSIZE: i32 = 255;
const MAXN: i32 = 100;

//$Procedure  F_ZZASRYEL ( Test ray-ellipse angular separation routine )
pub fn F_ZZASRYEL(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut TITLE = [b' '; LNSIZE as usize];
    let mut A: f64 = 0.0;
    let mut ANGLE: f64 = 0.0;
    let mut B: f64 = 0.0;
    let mut C: f64 = 0.0;
    let mut D = StackArray::<f64, 3>::new(1..=3);
    let mut EXTPT = StackArray::<f64, 3>::new(1..=3);
    let mut LIMB = StackArray::<f64, 9>::new(1..=UBEL);
    let mut PTPERP = StackArray::<f64, 3>::new(1..=3);
    let mut RAISE: f64 = 0.0;
    let mut V = StackArray::<f64, 3>::new(1..=3);
    let mut XANG: f64 = 0.0;
    let mut XPT = StackArray::<f64, 3>::new(1..=3);

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZASRYEL", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Simple inspection case using sphere. Unit sphere is centered at the origin.  View point is along +x axis.  Ray lies in the x-z plane and slants up at a 60 degree angle, passing over sphere. Find min.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Assign vertex and direction of ray.
    //
    spicelib::VPACK(2.0, 0.0, 0.0, V.as_slice_mut());
    spicelib::VPACK(-1.0, 0.0, f64::sqrt(3.0), D.as_slice_mut());

    //
    // Semi-axis lengths of "ellipsoid" whose limb is to be found.
    //
    A = 1.0;
    B = 1.0;
    C = 1.0;

    spicelib::EDLIMB(A, B, C, V.as_slice(), LIMB.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZASRYEL(
        b"MIN",
        LIMB.as_slice(),
        V.as_slice(),
        D.as_slice(),
        &mut ANGLE,
        EXTPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The expected angular separation from the limb is 30 degrees.
    //
    XANG = (spicelib::PI(ctx) / 6.0);

    testutil::CHCKSD(b"ANGLE,", ANGLE, b"~", XANG, TIGHT, OK, ctx)?;

    //
    // The expected limb point at which the minimum angular separation
    // occurs is at ( 1/2, 0, sqrt(3)/2 ).  Since we find the point
    // by searching for a minimum, we expect only about single
    // precision agreement.
    //
    spicelib::VPACK(0.5, 0.0, (f64::sqrt(3.0) / 2.0), XPT.as_slice_mut());

    testutil::CHCKAD(
        b"EXTPT,",
        EXTPT.as_slice(),
        b"~",
        XPT.as_slice(),
        3,
        LOOSE,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Simple inspection case using sphere. Unit sphere is centered at the origin.  View point is along +x axis.  Ray lies in the x-z plane and slants up at a 60 degree angle, passing over sphere.  Find max.");

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::ZZASRYEL(
        b"MAX",
        LIMB.as_slice(),
        V.as_slice(),
        D.as_slice(),
        &mut ANGLE,
        EXTPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The expected angular separation from the limb is 90 degrees.
    //
    XANG = (spicelib::PI(ctx) / 2.0);

    testutil::CHCKSD(b"ANGLE,", ANGLE, b"~", XANG, TIGHT, OK, ctx)?;

    //
    // The expected limb point at which the minimum angular separation
    // occurs is at ( 1/2, 0, -sqrt(3)/2 ).  Since we find the point
    // by searching for a minimum, we expect only about single
    // precision agreement.
    //
    spicelib::VPACK(0.5, 0.0, -(f64::sqrt(3.0) / 2.0), XPT.as_slice_mut());

    testutil::CHCKAD(
        b"EXTPT,",
        EXTPT.as_slice(),
        b"~",
        XPT.as_slice(),
        3,
        LOOSE,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Simple inspection case using sphere. Unit sphere is centered at the origin.  View point is along +x axis.  Ray lies along the -x axis.  Find min.");

    testutil::TCASE(&TITLE, ctx)?;
    //
    // Now take the ray along the x-axis, pointing in the -x direction.
    //
    spicelib::VPACK(2.0, 0.0, 0.0, V.as_slice_mut());
    spicelib::VPACK(-1.0, 0.0, 0.0, D.as_slice_mut());

    spicelib::ZZASRYEL(
        b"MIN",
        LIMB.as_slice(),
        V.as_slice(),
        D.as_slice(),
        &mut ANGLE,
        EXTPT.as_slice_mut(),
        ctx,
    )?;

    //
    // The expected angular separation from the limb is 30 degrees.
    // Since the ray penetrates the interior of the plane region
    // bounded by the limb, the sign of the angle is negative.
    //
    XANG = -(spicelib::PI(ctx) / 6.0);

    testutil::CHCKSD(b"ANGLE,", ANGLE, b"~", XANG, TIGHT, OK, ctx)?;

    //
    // The expected limb point at which the minimum angular separation
    // occurs could be anywhere on the limb.  All limb points have
    // x-component -1/2 and an orthogonal component with length
    // sqrt(3)/2.
    //
    testutil::CHCKSD(b"EXTPT(1)", EXTPT[1], b"~", 0.5, LOOSE, OK, ctx)?;

    spicelib::VPACK(0.0, XPT[2], XPT[3], PTPERP.as_slice_mut());

    testutil::CHCKSD(
        b"||perp||",
        spicelib::VNORM(PTPERP.as_slice()),
        b"~",
        (f64::sqrt(3.0) / 2.0),
        LOOSE,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Simple inspection case using sphere. Unit sphere is centered at the origin.  View point is along +x axis.  Ray lies along the -x axis.  Find max.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // In this case, all limb points correspond to both minima and
    // maxima.
    //
    spicelib::ZZASRYEL(
        b"MAX",
        LIMB.as_slice(),
        V.as_slice(),
        D.as_slice(),
        &mut ANGLE,
        EXTPT.as_slice_mut(),
        ctx,
    )?;

    //
    // The expected angular separation from the limb is 30 degrees.
    // Since the ray penetrates the interior of the plane region
    // bounded by the limb, the sign of the angle is negative.
    //
    XANG = -(spicelib::PI(ctx) / 6.0);

    testutil::CHCKSD(b"ANGLE,", ANGLE, b"~", XANG, TIGHT, OK, ctx)?;

    //
    // The expected limb point at which the minimum angular separation
    // occurs could be anywhere on the limb.  All limb points have
    // x-component -1/2 and an orthogonal component with length
    // sqrt(3)/2.
    //
    testutil::CHCKSD(b"EXTPT(1)", EXTPT[1], b"~", 0.5, LOOSE, OK, ctx)?;

    spicelib::VPACK(0.0, XPT[2], XPT[3], PTPERP.as_slice_mut());

    testutil::CHCKSD(
        b"||perp||",
        spicelib::VNORM(PTPERP.as_slice()),
        b"~",
        (f64::sqrt(3.0) / 2.0),
        LOOSE,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Simple inspection case using sphere. Unit sphere is centered at the origin.  View point is on the +x axis.  Ray points slightly above parallel to the -x axis.  Find min.");

    testutil::TCASE(&TITLE, ctx)?;
    //
    // Raise the vertex a bit and repeat.
    //
    RAISE = 0.000001;

    spicelib::VPACK(2.0, 0.0, 0.0, V.as_slice_mut());
    spicelib::VPACK(-1.0, 0.0, RAISE, D.as_slice_mut());

    spicelib::ZZASRYEL(
        b"MIN",
        LIMB.as_slice(),
        V.as_slice(),
        D.as_slice(),
        &mut ANGLE,
        EXTPT.as_slice_mut(),
        ctx,
    )?;

    //
    // The expected magnitude of the angular separation from the limb is
    // still about 30 degrees, since the limb shifts slightly when
    // the view point is raised.
    //
    // Since the ray penetrates the interior of the plane region
    // bounded by the limb, the sign of the angle is negative.
    //
    XANG = -((spicelib::PI(ctx) / 6.0) - f64::atan((RAISE / 1.0)));

    testutil::CHCKSD(b"ANGLE,", ANGLE, b"~", XANG, TIGHT, OK, ctx)?;

    //
    // The expected limb point is again at the top of the limb.  Because
    // this is a near-degenerate case, the determination of the extreme
    // point will be quite inaccurate.
    //
    spicelib::VPACK(0.5, 0.0, (f64::sqrt(3.0) / 2.0), XPT.as_slice_mut());

    testutil::CHCKAD(
        b"EXTPT,",
        EXTPT.as_slice(),
        b"~",
        XPT.as_slice(),
        3,
        SLOPPY,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Simple inspection case using sphere. Unit sphere is centered at the origin.  View point is on the +x axis.  Ray points slightly above parallel to the -x axis.  Find max.");

    spicelib::ZZASRYEL(
        b"MAX",
        LIMB.as_slice(),
        V.as_slice(),
        D.as_slice(),
        &mut ANGLE,
        EXTPT.as_slice_mut(),
        ctx,
    )?;

    //
    // The expected magnitude of the angular separation from the limb is
    // still about 30 degrees, since the limb shifts slightly when
    // the view point is raised.
    //
    // Since the ray penetrates the interior of the plane region
    // bounded by the limb, the sign of the angle is negative.
    //
    // This time the extreme point we seek is at the bottom of the limb.
    // Recall that "max" refers to maximum *magnitude* in this context.
    //
    XANG = -((spicelib::PI(ctx) / 6.0) + f64::atan((RAISE / 1.0)));

    testutil::CHCKSD(b"ANGLE,", ANGLE, b"~", XANG, TIGHT, OK, ctx)?;

    //
    // Because this is a near-degenerate case, the determination of the
    // extreme point will be quite inaccurate.
    //
    spicelib::VPACK(0.5, 0.0, -(f64::sqrt(3.0) / 2.0), XPT.as_slice_mut());

    testutil::CHCKAD(
        b"EXTPT,",
        EXTPT.as_slice(),
        b"~",
        XPT.as_slice(),
        3,
        SLOPPY,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Simple inspection case derived from first unit sphere case.  This time ellipsoid is very wide in the +/- y directions.  Find min.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Assign vertex and direction of ray.
    //
    spicelib::VPACK(2.0, 0.0, 0.0, V.as_slice_mut());
    spicelib::VPACK(-1.0, 0.0, f64::sqrt(3.0), D.as_slice_mut());

    //
    // Semi-axis lengths of ellipsoid whose limb is to be found.
    // Note length of y semi-axis.
    //
    A = 1.0;
    B = 1000.0;
    C = 1.0;

    spicelib::EDLIMB(A, B, C, V.as_slice(), LIMB.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZASRYEL(
        b"MIN",
        LIMB.as_slice(),
        V.as_slice(),
        D.as_slice(),
        &mut ANGLE,
        EXTPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The expected angular separation from the limb is 30 degrees.
    //
    XANG = (spicelib::PI(ctx) / 6.0);

    testutil::CHCKSD(b"ANGLE,", ANGLE, b"~", XANG, MEDTOL, OK, ctx)?;

    //
    // The expected limb point at which the minimum angular separation
    // occurs is at ( 1/2, 0, sqrt(3)/2 ).  Since we find the point
    // by searching for a minimum, and because of the rather extreme
    // geometry, we expect less than single precision agreement.
    //
    spicelib::VPACK(0.5, 0.0, (f64::sqrt(3.0) / 2.0), XPT.as_slice_mut());

    testutil::CHCKAD(
        b"EXTPT,",
        EXTPT.as_slice(),
        b"~",
        XPT.as_slice(),
        3,
        SLOPPY,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Extreme case:  this time ellipsoid is very wide in the +/- y directions and flat in z-direction. Find max.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Assign vertex and direction of ray.
    //
    spicelib::VPACK(2.0, 0.0, 0.0, V.as_slice_mut());
    spicelib::VPACK(-1.0, 0.0, f64::sqrt(3.0), D.as_slice_mut());

    //
    // Semi-axis lengths of ellipsoid whose limb is to be found.
    // Note length of y and z semi-axes.
    //
    A = 1.0;
    B = 1000.0;
    C = 0.00001;

    spicelib::VPACK(-1.0, 0.0, 0.0001, D.as_slice_mut());

    spicelib::EDLIMB(A, B, C, V.as_slice(), LIMB.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZASRYEL(
        b"MAX",
        LIMB.as_slice(),
        V.as_slice(),
        D.as_slice(),
        &mut ANGLE,
        EXTPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The expected angular separation from the limb is close to the
    // angular extent of the largest semi-axis of the limb.  This
    // is a rough estimate.
    //
    XANG = f64::atan((((B * f64::sqrt(3.0)) / 2.0) / 1.5));

    testutil::CHCKSD(b"ANGLE,", ANGLE, b"~", XANG, MEDTOL, OK, ctx)?;

    //
    // The expected limb point at which the maximum angular separation
    // occurs is roughly at either of ( 1/2, +/- B*sqrt(3)/2, 0 ).
    // Since the actual point will have negative z-component, this is a
    // very rough estimate.
    //
    if (EXTPT[2] > 0 as f64) {
        spicelib::VPACK(0.5, ((B * f64::sqrt(3.0)) / 2.0), 0.0, XPT.as_slice_mut());
    } else {
        spicelib::VPACK(0.5, -((B * f64::sqrt(3.0)) / 2.0), 0.0, XPT.as_slice_mut());
    }

    testutil::CHCKAD(
        b"EXTPT,",
        EXTPT.as_slice(),
        b"~~/",
        XPT.as_slice(),
        3,
        SLOPPY,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Interior case with ray pointing in -x direction. This time ellipsoid is very wide in the +/- y directions.  Find min.");

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::VPACK(2.0, 0.0, 0.0, V.as_slice_mut());
    spicelib::VPACK(-1.0, 0.0, 0.0, D.as_slice_mut());

    A = 1.0;
    B = 1000.0;
    C = 1.0;

    spicelib::EDLIMB(A, B, C, V.as_slice(), LIMB.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZASRYEL(
        b"MIN",
        LIMB.as_slice(),
        V.as_slice(),
        D.as_slice(),
        &mut ANGLE,
        EXTPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The expected angular separation from the limb is 30 degrees.
    // Since the ray penetrates the interior of the plane region
    // bounded by the limb, the sign of the angle is negative.
    //
    XANG = -(spicelib::PI(ctx) / 6.0);

    testutil::CHCKSD(b"ANGLE,", ANGLE, b"~", XANG, MEDTOL, OK, ctx)?;

    //
    // The expected limb point at which the minimum angular separation
    // occurs is at either of ( 1/2, 0, +/- sqrt(3)/2 ).  Since we find
    // the point by searching for a minimum, and because of the rather
    // extreme geometry, we expect less than single precision agreement.
    //
    if (EXTPT[3] > 0 as f64) {
        spicelib::VPACK(0.5, 0.0, (f64::sqrt(3.0) / 2.0), XPT.as_slice_mut());
    } else {
        spicelib::VPACK(0.5, 0.0, -(f64::sqrt(3.0) / 2.0), XPT.as_slice_mut());
    }

    testutil::CHCKAD(
        b"EXTPT,",
        EXTPT.as_slice(),
        b"~",
        XPT.as_slice(),
        3,
        SLOPPY,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Interior case with ray pointing in -x direction. This time ellipsoid is very wide in the +/- y directions.  Find max.");

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::VPACK(2.0, 0.0, 0.0, V.as_slice_mut());
    spicelib::VPACK(-1.0, 0.0, 0.0, D.as_slice_mut());

    spicelib::ZZASRYEL(
        b"MAX",
        LIMB.as_slice(),
        V.as_slice(),
        D.as_slice(),
        &mut ANGLE,
        EXTPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The expected angular separation from the limb the angular
    // extent of the largest-semi-axis, which lies on the y-axis.
    // Since the ray penetrates the interior of the plane region
    // bounded by the limb, the sign of the angle is negative.
    //
    XANG = -f64::atan((((B * f64::sqrt(3.0)) / 2.0) / 1.5));

    testutil::CHCKSD(b"ANGLE,", ANGLE, b"~", XANG, MEDTOL, OK, ctx)?;

    //
    // The expected limb point at which the minimum angular separation
    // occurs is at either of ( 1/2, +/- B*sqrt(3)/2, 0 ).  Since we
    // find the point by searching for a minimum, and because of the
    // rather extreme geometry, we expect less than single precision
    // agreement.
    //
    if (EXTPT[2] > 0 as f64) {
        spicelib::VPACK(0.5, ((B * f64::sqrt(3.0)) / 2.0), 0.0, XPT.as_slice_mut());
    } else {
        spicelib::VPACK(0.5, -((B * f64::sqrt(3.0)) / 2.0), 0.0, XPT.as_slice_mut());
    }

    testutil::CHCKAD(
        b"EXTPT,",
        EXTPT.as_slice(),
        b"~",
        XPT.as_slice(),
        3,
        SLOPPY,
        OK,
        ctx,
    )?;

    //
    // Error cases:
    //

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Direction vector is zero vector.");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Assign vertex and direction of ray.
    //
    spicelib::VPACK(2.0, 0.0, 0.0, V.as_slice_mut());

    spicelib::CLEARD(3, D.as_slice_mut());

    //
    // Semi-axis lengths of "ellipsoid" whose limb is to be found.
    //
    A = 1.0;
    B = 1.0;
    C = 1.0;

    spicelib::EDLIMB(A, B, C, V.as_slice(), LIMB.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZASRYEL(
        b"MAX",
        LIMB.as_slice(),
        V.as_slice(),
        D.as_slice(),
        &mut ANGLE,
        EXTPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ZEROVECTOR)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Limb has zero-length semi-minor axis");

    testutil::TCASE(&TITLE, ctx)?;

    LIMB[7] = 0.0;
    LIMB[8] = 0.0;
    LIMB[9] = 0.0;

    spicelib::VPACK(-1.0, 0.0, f64::sqrt(3.0), D.as_slice_mut());

    spicelib::ZZASRYEL(
        b"MAX",
        LIMB.as_slice(),
        V.as_slice(),
        D.as_slice(),
        &mut ANGLE,
        EXTPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDAXISLENGTH)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Vertex lies in plane of ellipse.");

    LIMB[4] = 0.0;
    LIMB[5] = 2.0;
    LIMB[6] = 0.0;
    LIMB[7] = 1.0;
    LIMB[8] = 0.0;
    LIMB[9] = 0.0;

    spicelib::ZZASRYEL(
        b"MAX",
        LIMB.as_slice(),
        V.as_slice(),
        D.as_slice(),
        &mut ANGLE,
        EXTPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DEGENERATECASE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"Unrecognized extremum specifier");

    A = 1.0;
    B = 1.0;
    C = 1.0;

    spicelib::EDLIMB(A, B, C, V.as_slice(), LIMB.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZASRYEL(
        b"MX",
        LIMB.as_slice(),
        V.as_slice(),
        D.as_slice(),
        &mut ANGLE,
        EXTPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
