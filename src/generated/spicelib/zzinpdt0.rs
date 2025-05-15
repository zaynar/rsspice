//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

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
pub const NONE: i32 = 0;
pub const LONIDX: i32 = 1;
pub const LATIDX: i32 = 2;
pub const ALTIDX: i32 = 3;
const LOWER: i32 = 1;
const UPPER: i32 = 2;
const LT: i32 = -1;
const EQ: i32 = 0;
const GT: i32 = 1;

struct SaveVars {
    PI2: f64,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PI2: f64 = 0.0;
        let mut FIRST: bool = false;

        FIRST = true;
        PI2 = -1.0;

        Self { PI2, FIRST }
    }
}

//$Procedure ZZINPDT0 ( DSK, in planetodetic element, w/o margin? )
pub fn ZZINPDT0(
    P: &[f64],
    LON: f64,
    BOUNDS: &[f64],
    CORPAR: &[f64],
    EXCLUD: i32,
    INSIDE: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let P = DummyArray::new(P, 1..=3);
    let BOUNDS = DummyArray2D::new(BOUNDS, 1..=2, 1..=3);
    let CORPAR = DummyArray::new(CORPAR, 1..);
    let mut EMAX: f64 = 0.0;
    let mut EMIN: f64 = 0.0;
    let mut F: f64 = 0.0;
    let mut LEVEL: f64 = 0.0;
    let mut LOCLON: f64 = 0.0;
    let mut MAXLAT: f64 = 0.0;
    let mut MAXLON: f64 = 0.0;
    let mut MAXALT: f64 = 0.0;
    let mut MINLAT: f64 = 0.0;
    let mut MINLON: f64 = 0.0;
    let mut MINALT: f64 = 0.0;
    let mut PMAX: f64 = 0.0;
    let mut PMIN: f64 = 0.0;
    let mut RE: f64 = 0.0;
    let mut RP: f64 = 0.0;
    let mut MAXREL: i32 = 0;
    let mut MINREL: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Numeric relation codes returned by ZZPDCMPL:
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

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZINPDT0", ctx)?;

    if save.FIRST {
        save.PI2 = TWOPI(ctx);
        save.FIRST = false;
    }

    //
    // Unpack the shape parameters. Set the polar axis length for
    // later use.
    //
    RE = CORPAR[1];
    F = CORPAR[2];
    RP = (RE * (1.0 - F));

    //
    // Assume the point is outside to start. This allows us
    // to skip setting INSIDE when we find a boundary test
    // failure.
    //
    *INSIDE = false;
    //
    // Compare coordinates of the input point and the segment
    // bounds. Deal with altitude last, since it involves the
    // most work. We may be able to exit before performing the
    // altitude tests.
    //
    if (EXCLUD != LATIDX) {
        //
        // Compare the point's latitude to the segment's latitude bounds.
        //
        MINLAT = intrinsics::DMAX1(&[(BOUNDS[[LOWER, LATIDX]] - ANGMRG), -HALFPI(ctx)]);
        MAXLAT = intrinsics::DMIN1(&[(BOUNDS[[UPPER, LATIDX]] + ANGMRG), HALFPI(ctx)]);

        ZZPDCMPL(RE, F, P.as_slice(), MINLAT, &mut MINREL, ctx)?;
        ZZPDCMPL(RE, F, P.as_slice(), MAXLAT, &mut MAXREL, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZINPDT0", ctx)?;
            return Ok(());
        }

        if ((MINREL == LT) || (MAXREL == GT)) {
            //
            // The point's latitude is outside of the segment's range.
            //
            CHKOUT(b"ZZINPDT0", ctx)?;
            return Ok(());
        }
    }

    //
    // Move the longitude of the input point into the interval
    //
    //    [ MINLON, MAXLON ]
    //
    // if necessary and if possible.
    //
    if (EXCLUD != LONIDX) {
        //
        // Put the local longitude bounds in order, if necessary.
        //
        ZZNRMLON(
            BOUNDS[[LOWER, LONIDX]],
            BOUNDS[[UPPER, LONIDX]],
            ANGMRG,
            &mut MINLON,
            &mut MAXLON,
            ctx,
        )?;
        //
        // Compare the point's longitude to the segment's longitude
        // bounds.
        //
        LOCLON = LON;

        if (LON < (MINLON - LONALI)) {
            //
            // If the point's longitude is less than the segment's
            // longitude by more than a small margin, shift the longitude
            // right by 2*pi.

            LOCLON = (LON + save.PI2);
        } else if (LON > (MAXLON + LONALI)) {
            //
            // If the point's longitude is greater than the segment's
            // longitude by more than a small margin, shift the longitude
            // left by 2*pi.

            LOCLON = (LON - save.PI2);
        }

        if ((LOCLON < (MINLON - ANGMRG)) || (LOCLON > (MAXLON + ANGMRG))) {
            //
            // The point's longitude, adjusted if necessary for
            // comparison, is outside of the segment's range.
            //
            CHKOUT(b"ZZINPDT0", ctx)?;
            return Ok(());
        }
    }

    if (EXCLUD != ALTIDX) {
        //
        // Extract altitude bounds from the segment descriptor.
        //
        MINALT = BOUNDS[[LOWER, ALTIDX]];
        MAXALT = BOUNDS[[UPPER, ALTIDX]];
        //
        // Find the semi-axes of the bounding spheroids.
        //
        if (F >= 0.0) {
            //
            // This is the oblate case. Get the semi-axis lengths
            // for the inner and outer bounding spheroids.
            //
            ZZELLBDS(
                RE, RP, MAXALT, MINALT, &mut EMAX, &mut PMAX, &mut EMIN, &mut PMIN, ctx,
            )?;
        } else {
            //
            // This is the prolate case. RP is the longer axis. Get the
            // semi-axis lengths for the inner and outer bounding
            // spheroids.
            //
            // In this call, we'll store the radii associated with
            // the longer axis in PMAX and PMIN.
            //
            ZZELLBDS(
                RP, RE, MAXALT, MINALT, &mut PMAX, &mut EMAX, &mut PMIN, &mut EMIN, ctx,
            )?;
        }

        //
        // Compute the input point's level surface parameters
        // for the inner and outer bounding ellipsoids. Do these
        // computations one at a time, so the second one can be
        // skipped if the first one shows the point is outside
        // the outer ellipsoid.
        //
        LEVEL = ((f64::powi((P[1] / EMAX), 2) + f64::powi((P[2] / EMAX), 2))
            + f64::powi((P[3] / PMAX), 2));

        if (LEVEL > 1.0) {
            //
            // The point is outside of the outer bounding ellipsoid.
            //
            CHKOUT(b"ZZINPDT0", ctx)?;
            return Ok(());
        }

        LEVEL = ((f64::powi((P[1] / EMIN), 2) + f64::powi((P[2] / EMIN), 2))
            + f64::powi((P[3] / PMIN), 2));

        if (LEVEL < 1.0) {
            //
            // The point is inside the inner bounding ellipsoid, which
            // implies it is outside of the segment's boundaries.
            //
            CHKOUT(b"ZZINPDT0", ctx)?;
            return Ok(());
        }
    }

    //
    // Getting to this point means the input point is inside
    // the segment. Being on the boundary counts as inside.
    //
    *INSIDE = true;

    CHKOUT(b"ZZINPDT0", ctx)?;
    Ok(())
}
