//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const MARGIN: f64 = 0.000000000001;

//$Procedure ZZLATBOX (Bounding box for latitudinal volume element)
pub fn ZZLATBOX(
    BOUNDS: &[f64],
    CENTER: &mut [f64],
    LR: &mut f64,
    LT: &mut f64,
    LZ: &mut f64,
    RADIUS: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let BOUNDS = DummyArray2D::new(BOUNDS, 1..=2, 1..=3);
    let mut CENTER = DummyArrayMut::new(CENTER, 1..=3);
    let mut DIAG = StackArray::<f64, 3>::new(1..=3);
    let mut HDLON: f64 = 0.0;
    let mut INRAD: f64 = 0.0;
    let mut MAXLAT: f64 = 0.0;
    let mut MAXLON: f64 = 0.0;
    let mut MAXR: f64 = 0.0;
    let mut MAXZ: f64 = 0.0;
    let mut MIDLON: f64 = 0.0;
    let mut MIDR: f64 = 0.0;
    let mut MINLAT: f64 = 0.0;
    let mut MINLON: f64 = 0.0;
    let mut MINR: f64 = 0.0;
    let mut MINZ: f64 = 0.0;
    let mut OUTRAD: f64 = 0.0;

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

    //
    // Get local copies of the bounds of the volume element.
    //
    MINLON = BOUNDS[[1, 1]];
    MAXLON = BOUNDS[[2, 1]];

    if (MAXLON <= MINLON) {
        MAXLON = (MAXLON + TWOPI(ctx));
    }

    if (MAXLON <= MINLON) {
        CHKIN(b"ZZLATBOX", ctx)?;
        SETMSG(
            b"Longitude bounds are #:#. Minimum longitude exceeds maximum by more than 2 pi.",
            ctx,
        );
        ERRDP(b"#", MINLON, ctx);
        ERRDP(b"#", BOUNDS[[2, 1]], ctx);
        SIGERR(b"SPICE(BADLONGITUDERANGE)", ctx)?;
        CHKOUT(b"ZZLATBOX", ctx)?;
        return Ok(());
    }

    MINLAT = BOUNDS[[1, 2]];
    MAXLAT = BOUNDS[[2, 2]];

    MINR = BOUNDS[[1, 3]];
    MAXR = BOUNDS[[2, 3]];

    if (MINLAT > MAXLAT) {
        CHKIN(b"ZZLATBOX", ctx)?;
        SETMSG(b"Latitude bounds #:# are out of order.", ctx);
        ERRDP(b"#", MINLAT, ctx);
        ERRDP(b"#", MAXLAT, ctx);
        SIGERR(b"SPICE(BADLATITUDEBOUNDS)", ctx)?;
        CHKOUT(b"ZZLATBOX", ctx)?;
        return Ok(());
    }

    if (MINLAT < (-HALFPI(ctx) - MARGIN)) {
        CHKIN(b"ZZLATBOX", ctx)?;
        SETMSG(b"Minimum latitude # is less than -pi/2.", ctx);
        ERRDP(b"#", MINLAT, ctx);
        SIGERR(b"SPICE(BADLATITUDERANGE)", ctx)?;
        CHKOUT(b"ZZLATBOX", ctx)?;
        return Ok(());
    }

    if (MAXLAT > (HALFPI(ctx) + MARGIN)) {
        CHKIN(b"ZZLATBOX", ctx)?;
        SETMSG(b"Maximum latitude # is more than -pi/2.", ctx);
        ERRDP(b"#", MAXLAT, ctx);
        SIGERR(b"SPICE(BADLATITUDERANGE)", ctx)?;
        CHKOUT(b"ZZLATBOX", ctx)?;
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
    // is that of the lower latitude bound on the outer bounding sphere,
    // and the inner radius is that of the upper latitude bound on the
    // inner bounding sphere.
    //
    // These relationships are reversed for elements that lie on or
    // below the X-Y plane.
    //
    // For elements that span the X-Y plane, the outer radius is that of
    // the outer bounding sphere. The inner radius is that of the
    // latitude circle on the inner bounding sphere for which the
    // absolute value of the latitude is maximum.
    //
    if (MINLAT >= 0.0) {
        OUTRAD = (f64::cos(MINLAT) * MAXR);
        INRAD = (f64::cos(MAXLAT) * MINR);
    } else if (MAXLAT <= 0.0) {
        OUTRAD = (f64::cos(MAXLAT) * MAXR);
        INRAD = (f64::cos(MINLAT) * MINR);
    } else {
        OUTRAD = MAXR;
        INRAD = (f64::cos(intrinsics::DMAX1(&[f64::abs(MAXLAT), f64::abs(MINLAT)])) * MINR);
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
        MAXZ = (MAXR * f64::sin(MAXLAT));
        MINZ = (MINR * f64::sin(MINLAT));
    } else if (MAXLAT <= 0.0) {
        MAXZ = (MINR * f64::sin(MAXLAT));
        MINZ = (MAXR * f64::sin(MINLAT));
    } else {
        MAXZ = (MAXR * f64::sin(MAXLAT));
        MINZ = (MAXR * f64::sin(MINLAT));
    }

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
