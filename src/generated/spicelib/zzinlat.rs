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
pub const RADIDX: i32 = 3;
pub const LATMRG: f64 = 0.00000001;
const WEST: i32 = 1;
const EAST: i32 = 2;
const SOUTH: i32 = 1;
const NORTH: i32 = 2;
const INNER: i32 = 1;
const OUTER: i32 = 2;

struct SaveVars {
    HPI: f64,
    PI2: f64,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut HPI: f64 = 0.0;
        let mut PI2: f64 = 0.0;
        let mut FIRST: bool = false;

        FIRST = true;
        PI2 = -1.0;
        HPI = -1.0;

        Self { HPI, PI2, FIRST }
    }
}

//$Procedure ZZINLAT ( DSK, in latitudinal element? )
pub fn ZZINLAT(
    P: &[f64],
    BOUNDS: &[f64],
    MARGIN: f64,
    EXCLUD: i32,
    INSIDE: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let P = DummyArray::new(P, 1..=3);
    let BOUNDS = DummyArray2D::new(BOUNDS, 1..=2, 1..=3);
    let mut AMAXR: f64 = 0.0;
    let mut AMINR: f64 = 0.0;
    let mut AMAXLT: f64 = 0.0;
    let mut AMINLT: f64 = 0.0;
    let mut AMAXLO: f64 = 0.0;
    let mut AMINLO: f64 = 0.0;
    let mut DLON: f64 = 0.0;
    let mut LAT: f64 = 0.0;
    let mut LON: f64 = 0.0;
    let mut LONMRG: f64 = 0.0;
    let mut MAXLAT: f64 = 0.0;
    let mut MAXLON: f64 = 0.0;
    let mut MAXR: f64 = 0.0;
    let mut MINLAT: f64 = 0.0;
    let mut MINLON: f64 = 0.0;
    let mut MINR: f64 = 0.0;
    let mut R: f64 = 0.0;
    let mut SMAX: f64 = 0.0;
    let mut SMIN: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Element boundary indices:
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    if save.FIRST {
        save.PI2 = TWOPI(ctx);
        save.HPI = HALFPI(ctx);

        save.FIRST = false;
    }

    //
    // Get the latitudinal coordinates of the input point.
    //
    RECLAT(P.as_slice(), &mut R, &mut LON, &mut LAT);

    //
    // Handle the simpler zero-margin case separately.
    //
    if (MARGIN == 0.0) {
        ZZINLAT0(R, LON, LAT, BOUNDS.as_slice(), EXCLUD, INSIDE, ctx)?;

        return Ok(());
    } else if (MARGIN < 0.0) {
        CHKIN(b"ZZINLAT", ctx)?;
        SETMSG(b"Margin must be non-negative but was #.", ctx);
        ERRDP(b"#", MARGIN, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZINLAT", ctx)?;
        return Ok(());
    }

    if ((EXCLUD < 0) || (EXCLUD > 3)) {
        CHKIN(b"ZZINLAT", ctx)?;
        SETMSG(b"EXCLUD must be in the range 0:3 but was #.", ctx);
        ERRINT(b"#", EXCLUD, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZINLAT", ctx)?;
        return Ok(());
    }

    //
    // Special case: if the input point is within distance MARGIN
    // from the origin, and the minimum radius of the volume element
    // is less than or equal to MARGIN, the point is inside.
    //
    if (R <= MARGIN) {
        if (BOUNDS[[INNER, RADIDX]] <= MARGIN) {
            *INSIDE = true;

            return Ok(());
        }
    }

    //
    // Assume the point is outside to start. This allows us
    // to skip setting INSIDE when we find a boundary test
    // failure.
    //
    *INSIDE = false;

    //
    // Get local copies of the coordinate bounds. Don't normalize the
    // longitude bounds until we know we need them.
    //
    MINR = BOUNDS[[INNER, RADIDX]];
    MAXR = BOUNDS[[OUTER, RADIDX]];

    MINLAT = BOUNDS[[SOUTH, LATIDX]];
    MAXLAT = BOUNDS[[NORTH, LATIDX]];

    //
    // Compare coordinates to adjusted latitude and radius
    // boundaries.
    //
    if (EXCLUD != RADIDX) {
        //
        // Create adjusted radius bounds.
        //
        SMAX = (1.0 + MARGIN);
        SMIN = (1.0 - MARGIN);

        AMINR = intrinsics::DMAX1(&[0.0, (MINR * SMIN)]);
        AMAXR = (MAXR * SMAX);

        if ((R < AMINR) || (R > AMAXR)) {
            return Ok(());
        }
    }

    if (EXCLUD != LATIDX) {
        //
        // Create adjusted latitude bounds.
        //
        AMINLT = intrinsics::DMAX1(&[-save.HPI, (MINLAT - MARGIN)]);
        AMAXLT = intrinsics::DMIN1(&[save.HPI, (MAXLAT + MARGIN)]);

        if ((LAT < AMINLT) || (LAT > AMAXLT)) {
            return Ok(());
        }
    }

    //
    // At this point, the input radius and latitude are within the
    // adjusted bounds, if their tests haven't been excluded by
    // the caller.
    //
    // Perform longitude tests, unless they're excluded by the
    // caller.
    //
    if (EXCLUD != LONIDX) {
        ZZNRMLON(
            BOUNDS[[WEST, LONIDX]],
            BOUNDS[[EAST, LONIDX]],
            ANGMRG,
            &mut MINLON,
            &mut MAXLON,
            ctx,
        )?;
        //
        // Set the margin to be used for longitude interval
        // inclusion tests.
        //
        LONMRG = intrinsics::DMAX1(&[f64::abs(ANGMRG), f64::abs(MARGIN)]);

        //
        // We have a special case for segments that include the poles. If
        // the input point is close enough to a pole contained in the
        // segment, we consider the point to be included in the segment,
        // regardless of the point's longitude. All other points get the
        // normal longitude test.
        //
        if ((LAT <= (save.HPI - LATMRG)) && (LAT >= (-save.HPI + LATMRG))) {
            //
            // This is the usual case: the latitude of the input point
            // is bounded away from the poles.
            //
            // Check the point's longitude against the segment's
            // longitude bounds.
            //
            // We'll scale the longitude margin to compensate for the
            // latitude of the input point. Note that the division
            // below is safe; presuming a reasonable value of MARGIN;
            // we know that
            //
            //    DLON << 1
            //
            DLON = (LONMRG / intrinsics::DMAX1(&[f64::abs(f64::cos(LAT)), LATMRG]));

            AMINLO = (MINLON - DLON);
            AMAXLO = (MAXLON + DLON);

            //
            // Now move the input point's longitude into range, if
            // necessary.
            //
            if (LON < AMINLO) {
                if (LON < (AMINLO - LONALI)) {
                    //
                    // See whether an aliased version of LON is a match.
                    //
                    LON = (LON + save.PI2);
                } else {
                    //
                    // Consider LON to be a match with the lower bound.
                    //
                    LON = AMINLO;
                }
            } else if (LON > AMAXLO) {
                if (LON > (AMAXLO + LONALI)) {
                    //
                    // See whether an aliased version of LON is a match.
                    //
                    LON = (LON - save.PI2);
                } else {
                    //
                    // Consider LON to be a match with the upper bound.
                    //
                    LON = AMAXLO;
                }
            }

            //
            // Compare the adjusted longitude of the input point to the
            // adjusted longitude bounds.
            //
            if ((LON < AMINLO) || (LON > AMAXLO)) {
                return Ok(());
            }
        } else {
            //
            // The latitude of the input point is close to one of the
            // poles.
            //
            // This is a no-op case.
            //
            // The input point has already passed whichever of the radius
            // and latitude tests that were not excluded.
            //
            // If the element has a non-degenerate latitude boundary
            // having the same sign as the latitude of the input point,
            // and if latitude is excluded because the input point is
            // already nominally on that boundary, then passing the radius
            // check implies that the point is close to the element.
            //
            // If the element has a degenerate latitude boundary having
            // the same sign as the latitude of the input point---namely,
            // the element contains the pole, and latitude is excluded,
            // then then passing the radius check implies that the point
            // is close to the portion of the Z-axis contained in the
            // element.
            //
            // If the radius check has been excluded because the point is
            // already nominally on one of the element's radius boundaries,
            // the passing the latitude test implies the point is close
            // to the element.
            //
            // In all cases, as long as EXCLUD has been set appropriately,
            // the point is close to the element. We consider the point to
            // be in the expanded element.
            //
        }
    }

    //
    // All tests that were commanded have been passed. The input
    // point is considered to be contained in the expanded volume
    // element.
    //
    *INSIDE = true;

    Ok(())
}
