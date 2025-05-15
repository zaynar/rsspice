//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const SRFIDX: i32 = 1;
const CTRIDX: i32 = (SRFIDX + 1);
const CLSIDX: i32 = (CTRIDX + 1);
const TYPIDX: i32 = (CLSIDX + 1);
const FRMIDX: i32 = (TYPIDX + 1);
const SYSIDX: i32 = (FRMIDX + 1);
const PARIDX: i32 = (SYSIDX + 1);
const NSYPAR: i32 = 10;
const MN1IDX: i32 = (PARIDX + NSYPAR);
const MX1IDX: i32 = (MN1IDX + 1);
const MN2IDX: i32 = (MX1IDX + 1);
const MX2IDX: i32 = (MN2IDX + 1);
const MN3IDX: i32 = (MX2IDX + 1);
const MX3IDX: i32 = (MN3IDX + 1);
const BTMIDX: i32 = (MX3IDX + 1);
const ETMIDX: i32 = (BTMIDX + 1);
const DSKDSZ: i32 = ETMIDX;
const SVFCLS: i32 = 1;
const GENCLS: i32 = 2;
const LATSYS: i32 = 1;
const CYLSYS: i32 = 2;
const RECSYS: i32 = 3;
const PDTSYS: i32 = 4;
const XFRACT: f64 = 0.0000000001;
const KEYXFR: i32 = 1;
const SGREED: f64 = 0.00000001;
const KEYSGR: i32 = (KEYXFR + 1);
const SGPADM: f64 = 0.0000000001;
const KEYSPM: i32 = (KEYSGR + 1);
const PTMEMM: f64 = 0.0000001;
const KEYPTM: i32 = (KEYSPM + 1);
const ANGMRG: f64 = 0.000000000001;
const KEYAMG: i32 = (KEYPTM + 1);
const LONALI: f64 = 0.000000000001;
const KEYLAL: i32 = (KEYAMG + 1);

//$Procedure ZZPDTBOX (Bounding box for planetodetic volume element)
pub fn ZZPDTBOX(
    BOUNDS: &[f64],
    CORPAR: &[f64],
    CENTER: &mut [f64],
    LR: &mut f64,
    LT: &mut f64,
    LZ: &mut f64,
    RADIUS: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let BOUNDS = DummyArray2D::new(BOUNDS, 1..=2, 1..=3);
    let CORPAR = DummyArray::new(CORPAR, 1..);
    let mut CENTER = DummyArrayMut::new(CENTER, 1..=3);
    let mut BOTV = StackArray::<f64, 3>::new(1..=3);
    let mut DIAG = StackArray::<f64, 3>::new(1..=3);
    let mut F: f64 = 0.0;
    let mut HDLON: f64 = 0.0;
    let mut INRAD: f64 = 0.0;
    let mut MAXABS: f64 = 0.0;
    let mut MAXALT: f64 = 0.0;
    let mut MAXLAT: f64 = 0.0;
    let mut MAXLON: f64 = 0.0;
    let mut MAXV = StackArray::<f64, 3>::new(1..=3);
    let mut MAXZ: f64 = 0.0;
    let mut MIDLON: f64 = 0.0;
    let mut MIDR: f64 = 0.0;
    let mut MINALT: f64 = 0.0;
    let mut MINLAT: f64 = 0.0;
    let mut MINLON: f64 = 0.0;
    let mut MINV = StackArray::<f64, 3>::new(1..=3);
    let mut MINZ: f64 = 0.0;
    let mut OUTRAD: f64 = 0.0;
    let mut RE: f64 = 0.0;
    let mut TOPV = StackArray::<f64, 3>::new(1..=3);

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
    // This routine uses discovery check-in. We check RETURN in order to
    // avoid performing math operations using invalid operands.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    RE = CORPAR[1];
    F = CORPAR[2];

    //
    // The equatorial radius must be greater than zero.
    //
    if (RE <= 0.0) {
        CHKIN(b"ZZPDTBOX", ctx)?;
        SETMSG(b"Equatorial radius from CORPAR array was #.", ctx);
        ERRDP(b"#", RE, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZPDTBOX", ctx)?;
        return Ok(());
    }

    //
    // If the flattening coefficient is greater than one, the polar
    // radius computed below is negative. If it's equal to one, the
    // polar radius is zero. Either case is a problem, so signal an
    // error and check out.
    //
    if (F >= 1.0) {
        CHKIN(b"ZZPDTBOX", ctx)?;
        SETMSG(b"Flattening coefficient from CORPAR array was #.", ctx);
        ERRDP(b"*", F, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZPDTBOX", ctx)?;
        return Ok(());
    }

    //
    // Get local copies of the bounds of the volume element.
    //
    MINLON = BOUNDS[[1, 1]];
    MAXLON = BOUNDS[[2, 1]];

    if (MAXLON <= MINLON) {
        MAXLON = (MAXLON + TWOPI(ctx));
    }

    if (MAXLON <= MINLON) {
        CHKIN(b"ZZPDTBOX", ctx)?;
        SETMSG(
            b"Longitude bounds are #:#. Minimum longitude exceeds maximum by more than 2 pi.",
            ctx,
        );
        ERRDP(b"#", MINLON, ctx);
        ERRDP(b"#", BOUNDS[[2, 1]], ctx);
        SIGERR(b"SPICE(BADLONGITUDERANGE)", ctx)?;
        CHKOUT(b"ZZPDTBOX", ctx)?;
        return Ok(());
    }

    MINLAT = BOUNDS[[1, 2]];
    MAXLAT = BOUNDS[[2, 2]];

    MINALT = BOUNDS[[1, 3]];
    MAXALT = BOUNDS[[2, 3]];

    if (MINLAT > MAXLAT) {
        CHKIN(b"ZZPDTBOX", ctx)?;
        SETMSG(b"Latitude bounds #:# are out of order.", ctx);
        ERRDP(b"#", MINLAT, ctx);
        ERRDP(b"#", MAXLAT, ctx);
        SIGERR(b"SPICE(BADLATITUDEBOUNDS)", ctx)?;
        CHKOUT(b"ZZPDTBOX", ctx)?;
        return Ok(());
    }

    if (MINLAT < (-HALFPI(ctx) - ANGMRG)) {
        CHKIN(b"ZZPDTBOX", ctx)?;
        SETMSG(b"Minimum latitude # is less than -pi/2.", ctx);
        ERRDP(b"#", MINLAT, ctx);
        SIGERR(b"SPICE(BADLATITUDERANGE)", ctx)?;
        CHKOUT(b"ZZPDTBOX", ctx)?;
        return Ok(());
    }

    if (MAXLAT > (HALFPI(ctx) + ANGMRG)) {
        CHKIN(b"ZZPDTBOX", ctx)?;
        SETMSG(b"Maximum latitude # is more than -pi/2.", ctx);
        ERRDP(b"#", MAXLAT, ctx);
        SIGERR(b"SPICE(BADLATITUDERANGE)", ctx)?;
        CHKOUT(b"ZZPDTBOX", ctx)?;
        return Ok(());
    }

    MINLAT = intrinsics::DMAX1(&[MINLAT, -HALFPI(ctx)]);
    MAXLAT = intrinsics::DMIN1(&[MAXLAT, HALFPI(ctx)]);

    //
    // Let INRAD and OUTRAD be, respectively, the radii of the
    // orthogonal projections onto the X-Y plane of the element's arcs
    // of minimum and maximum distance from the Z axis.
    //
    // If the element lies on or above the X-Y plane, the outer radius
    // is that of the lower latitude bound on the surface of maximum
    // altitude and the inner radius is that of the upper latitude bound
    // on the surface of minimum altitude.
    //
    // These relationships are reversed for elements that lie on or
    // below the X-Y plane.
    //
    // For elements that span the X-Y plane, the outer radius is that of
    // the coordinate system's equatorial radius plus the maximum
    // altitude. The inner radius is that of the latitude circle on the
    // surface of minimum altitude for which the absolute value of the
    // latitude is maximum.
    //
    if (MINLAT >= 0.0) {
        GEOREC(0.0, MINLAT, MAXALT, RE, F, MAXV.as_slice_mut(), ctx)?;
        GEOREC(0.0, MAXLAT, MINALT, RE, F, MINV.as_slice_mut(), ctx)?;

        MAXV[3] = 0.0;
        MINV[3] = 0.0;

        OUTRAD = VNORM(MAXV.as_slice());
        INRAD = VNORM(MINV.as_slice());
    } else if (MAXLAT <= 0.0) {
        GEOREC(0.0, MAXLAT, MAXALT, RE, F, MAXV.as_slice_mut(), ctx)?;
        GEOREC(0.0, MINLAT, MINALT, RE, F, MINV.as_slice_mut(), ctx)?;

        MAXV[3] = 0.0;
        MINV[3] = 0.0;

        OUTRAD = VNORM(MAXV.as_slice());
        INRAD = VNORM(MINV.as_slice());
    } else {
        OUTRAD = (RE + MAXALT);

        MAXABS = intrinsics::DMAX1(&[f64::abs(MAXLAT), f64::abs(MINLAT)]);

        GEOREC(0.0, MAXABS, MINALT, RE, F, MINV.as_slice_mut(), ctx)?;

        MINV[3] = 0.0;

        INRAD = VNORM(MINV.as_slice());
    }

    //
    // Let MIDLON be the longitude of the midpoint of the element's
    // longitude coverage. Let HDLON be one half of the extent of the
    // longitude coverage.
    //
    HDLON = ((MAXLON - MINLON) / 2 as f64);
    MIDLON = (MINLON + HDLON);

    //
    // LR is the length of the bounding box in the radial direction,
    // where "radius" is defined as the distance from the Z axis.
    //
    if (HDLON <= HALFPI(ctx)) {
        *LR = (OUTRAD - (INRAD * f64::cos(HDLON)));
    } else {
        *LR = ((1.0 - f64::cos(HDLON)) * OUTRAD);
    }

    //
    // The tangential length of bounding box depends on the longitude
    // extent. For any extent larger than Pi radians, the width
    // is just that of the outer radius.
    //
    if (HDLON > HALFPI(ctx)) {
        *LT = (2.0 * OUTRAD);
    } else {
        *LT = ((2.0 * OUTRAD) * f64::sin(HDLON));
    }

    //
    // The height bounds are derived from the lowest and highest points
    // on the element.
    //
    if (MINLAT >= 0.0) {
        GEOREC(0.0, MAXLAT, MAXALT, RE, F, TOPV.as_slice_mut(), ctx)?;
        GEOREC(0.0, MINLAT, MINALT, RE, F, BOTV.as_slice_mut(), ctx)?;
    } else if (MAXLAT < 0.0) {
        GEOREC(0.0, MAXLAT, MINALT, RE, F, TOPV.as_slice_mut(), ctx)?;
        GEOREC(0.0, MINLAT, MAXALT, RE, F, BOTV.as_slice_mut(), ctx)?;
    } else {
        GEOREC(0.0, MAXLAT, MAXALT, RE, F, TOPV.as_slice_mut(), ctx)?;
        GEOREC(0.0, MINLAT, MAXALT, RE, F, BOTV.as_slice_mut(), ctx)?;
    }

    MAXZ = TOPV[3];
    MINZ = BOTV[3];

    *LZ = (MAXZ - MINZ);

    //
    // Make sure all dimensions are non-negative.
    //
    *LR = intrinsics::DMAX1(&[0.0, *LR]);
    *LT = intrinsics::DMAX1(&[0.0, *LT]);
    *LZ = intrinsics::DMAX1(&[0.0, *LZ]);

    //
    // Compute the coordinates of the center of the box.
    //
    // The box center lies on the meridian of central
    // longitude. The outer tangential edge is at radius
    // OUTRAD. Let MIDR be the radius of the center.
    //
    MIDR = (OUTRAD - (*LR / 2 as f64));

    CYLREC(
        MIDR,
        MIDLON,
        (MINZ + (*LZ / 2 as f64)),
        CENTER.as_slice_mut(),
    );

    //
    // The radius is the distance from the center of the box
    // to any corner.
    //
    VPACK(
        (*LR / 2 as f64),
        (*LT / 2 as f64),
        (*LZ / 2 as f64),
        DIAG.as_slice_mut(),
    );

    *RADIUS = VNORM(DIAG.as_slice());

    Ok(())
}
