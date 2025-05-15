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
pub const LATMRG: f64 = 0.00000001;
const WEST: i32 = 1;
const EAST: i32 = 2;
const SOUTH: i32 = 1;
const NORTH: i32 = 2;
const LOWER: i32 = 1;
const UPPER: i32 = 2;
const LT: i32 = -1;
const GT: i32 = 1;

struct SaveVars {
    ALTBDS: StackArray2D<f64, 6>,
    HPI: f64,
    PI2: f64,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ALTBDS = StackArray2D::<f64, 6>::new(1..=2, 1..=3);
        let mut HPI: f64 = 0.0;
        let mut PI2: f64 = 0.0;
        let mut FIRST: bool = false;

        FIRST = true;
        HPI = -1.0;

        Self {
            ALTBDS,
            HPI,
            PI2,
            FIRST,
        }
    }
}

//$Procedure ZZINPDT ( DSK, in planetodetic element? )
pub fn ZZINPDT(
    P: &[f64],
    BOUNDS: &[f64],
    CORPAR: &[f64],
    MARGIN: f64,
    EXCLUD: i32,
    INSIDE: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let P = DummyArray::new(P, 1..=3);
    let BOUNDS = DummyArray2D::new(BOUNDS, 1..=2, 1..=3);
    let CORPAR = DummyArray::new(CORPAR, 1..);
    let mut AMNALT: f64 = 0.0;
    let mut AMNLAT: f64 = 0.0;
    let mut AMNLON: f64 = 0.0;
    let mut AMXALT: f64 = 0.0;
    let mut AMXLAT: f64 = 0.0;
    let mut AMXLON: f64 = 0.0;
    let mut DLON: f64 = 0.0;
    let mut F: f64 = 0.0;
    let mut LON: f64 = 0.0;
    let mut LONMRG: f64 = 0.0;
    let mut MAXALT: f64 = 0.0;
    let mut MAXLAT: f64 = 0.0;
    let mut MAXLON: f64 = 0.0;
    let mut MINALT: f64 = 0.0;
    let mut MINLAT: f64 = 0.0;
    let mut MINLON: f64 = 0.0;
    let mut PCNLAT: f64 = 0.0;
    let mut R: f64 = 0.0;
    let mut RE: f64 = 0.0;
    let mut RELMIN: i32 = 0;
    let mut RELMAX: i32 = 0;
    let mut ALPASS: bool = false;

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
    // Numeric relation codes returned by ZZPDCMPL:
    //

    //
    // The code EQ can be returned by ZZPDCMPL, but we make no
    // references to it, so it's not declared here. For the
    // record, EQ is set to 0.
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

    CHKIN(b"ZZINPDT", ctx)?;

    if save.FIRST {
        save.HPI = HALFPI(ctx);
        save.PI2 = TWOPI(ctx);
        //
        // Initialize the local array used for altitude checks.
        //
        save.ALTBDS[[SOUTH, LATIDX]] = -HALFPI(ctx);
        save.ALTBDS[[NORTH, LATIDX]] = HALFPI(ctx);
        save.ALTBDS[[WEST, LONIDX]] = -PI(ctx);
        save.ALTBDS[[EAST, LONIDX]] = PI(ctx);
        save.ALTBDS[[LOWER, ALTIDX]] = 0.0;
        save.ALTBDS[[UPPER, ALTIDX]] = 0.0;

        save.FIRST = false;
    }

    if ((EXCLUD < 0) || (EXCLUD > 3)) {
        SETMSG(b"EXCLUD must be in the range 0:3 but was #.", ctx);
        ERRINT(b"#", EXCLUD, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZINPDT", ctx)?;
        return Ok(());
    }

    //
    // Get the planetocentric [sic] coordinates of the input point. The
    // latitude we obtain will be planetocentric. To emphasize this, we
    // use the name "PCNLAT."
    //
    RECLAT(P.as_slice(), &mut R, &mut LON, &mut PCNLAT);
    //
    // RECLAT is error free, so we don't call FAILED() here.
    //

    if (MARGIN == 0.0) {
        //
        // ZZINPDT0 contains the logic required to determine whether
        // the input point is contained in the element.
        //
        ZZINPDT0(
            P.as_slice(),
            LON,
            BOUNDS.as_slice(),
            CORPAR.as_slice(),
            EXCLUD,
            INSIDE,
            ctx,
        )?;

        CHKOUT(b"ZZINPDT", ctx)?;
        return Ok(());
    } else if (MARGIN < 0.0) {
        SETMSG(b"Margin must be non-negative but was #.", ctx);
        ERRDP(b"#", MARGIN, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZINPDT", ctx)?;
        return Ok(());
    }

    //
    // At this point a more detailed analysis is needed.
    //
    //
    // Assume the point is outside to start. This allows us
    // to skip setting INSIDE when we find a boundary test
    // failure.
    //
    *INSIDE = false;

    //
    // We'll use the shape parameters for latitude and altitude
    // comparisons.
    //
    RE = CORPAR[1];
    F = CORPAR[2];

    //
    // Get local copies of the coordinate bounds. Don't normalize the
    // longitude bounds until we know we need them.
    //
    MINLAT = BOUNDS[[SOUTH, LATIDX]];
    MAXLAT = BOUNDS[[NORTH, LATIDX]];

    MINALT = BOUNDS[[LOWER, ALTIDX]];
    MAXALT = BOUNDS[[UPPER, ALTIDX]];

    //
    // Compare coordinates to adjusted latitude boundaries.
    //
    if (EXCLUD != LATIDX) {
        //
        // Create adjusted latitude bounds.
        //
        AMNLAT = intrinsics::DMAX1(&[(-save.HPI - ANGMRG), (MINLAT - MARGIN)]);
        AMXLAT = intrinsics::DMIN1(&[(save.HPI + ANGMRG), (MAXLAT + MARGIN)]);

        //
        // Compare the latitude of the input point to the bounds.
        //
        ZZPDCMPL(RE, F, P.as_slice(), AMNLAT, &mut RELMIN, ctx)?;
        ZZPDCMPL(RE, F, P.as_slice(), AMXLAT, &mut RELMAX, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZINPDT", ctx)?;
            return Ok(());
        }

        if ((RELMIN == LT) || (RELMAX == GT)) {
            //
            // The latitude of P is strictly outside of the element's
            // latitude bounds.
            //
            CHKOUT(b"ZZINPDT", ctx)?;
            return Ok(());
        }
    }

    //
    // Test the point for inclusion in the region between the bounding
    // ellipsoids that act as proxies for altitude boundaries.
    //
    if (EXCLUD != ALTIDX) {
        //
        // Extract altitude bounds from the segment descriptor.
        //
        MINALT = BOUNDS[[LOWER, ALTIDX]];
        MAXALT = BOUNDS[[UPPER, ALTIDX]];
        //
        // Adjust altitude bounds to account for the margin.
        //
        AMNALT = (MINALT - (MARGIN * f64::abs(MINALT)));
        AMXALT = (MAXALT + (MARGIN * f64::abs(MAXALT)));
        //
        // Set up a "boundary" array so that we can use ZZINPDT0
        // to do the altitude check for us. We'll exclude longitude
        // tests; the latitude test is set up for an automatic pass
        // (the latitude range of ALTBDS is [-pi/2, pi/2]).
        //
        save.ALTBDS[[LOWER, ALTIDX]] = AMNALT;
        save.ALTBDS[[UPPER, ALTIDX]] = AMXALT;

        ZZINPDT0(
            P.as_slice(),
            LON,
            save.ALTBDS.as_slice(),
            CORPAR.as_slice(),
            LONIDX,
            &mut ALPASS,
            ctx,
        )?;

        if !ALPASS {
            CHKOUT(b"ZZINPDT", ctx)?;
            return Ok(());
        }
    }

    //
    // At this point, the input altitude and latitude are within the
    // adjusted bounds, if their tests haven't been excluded by
    // the caller.
    //
    // Perform longitude tests, unless they're excluded by the
    // caller.
    //
    if (EXCLUD != LONIDX) {
        //
        // Start out by normalizing the element's longitude bounds.
        //
        ZZNRMLON(
            BOUNDS[[LOWER, LONIDX]],
            BOUNDS[[UPPER, LONIDX]],
            ANGMRG,
            &mut MINLON,
            &mut MAXLON,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZINPDT", ctx)?;
            return Ok(());
        }
        //
        // Set the margin to be used for longitude interval
        // inclusion tests.
        //
        LONMRG = intrinsics::DMAX1(&[f64::abs(ANGMRG), f64::abs(MARGIN)]);

        //
        // We have a special case for segments that include the poles. If
        // the latitude and altitude of the input point are within
        // bounds, and if the latitude of the point is close enough to a
        // pole we consider the point to be included in the segment,
        // regardless of the point's longitude. All other points get the
        // normal longitude test.
        //
        // We use planetocentric latitude to determine whether the point
        // is "close" to a pole.
        //
        ZZPDCMPL(RE, F, P.as_slice(), (save.HPI - LATMRG), &mut RELMAX, ctx)?;
        ZZPDCMPL(RE, F, P.as_slice(), (-save.HPI + LATMRG), &mut RELMIN, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZINPDT", ctx)?;
            return Ok(());
        }

        if ((RELMAX != GT) && (RELMIN != LT)) {
            //
            // This is the usual case: the latitude of the input point is
            // bounded away from the poles.
            //
            // Check the point's longitude against the segment's longitude
            // bounds.
            //
            // We'll scale the longitude margin to compensate for the
            // latitude of the input point. Note that the division
            // below is safe; presuming a reasonable value of MARGIN,
            // we know that
            //
            //    DLON << 1
            //
            // Note that we use planetocentric latitude for scaling the
            // longitude margin. This substitution (for planetodetic
            // latitude) is adequate for this purpose.
            //
            DLON = (LONMRG / intrinsics::DMAX1(&[f64::abs(f64::cos(PCNLAT)), LATMRG]));

            AMNLON = (MINLON - DLON);
            AMXLON = (MAXLON + DLON);

            //
            // Now move the input point's longitude into range, if
            // necessary, so we can make a valid comparison against
            // the longitude bounds.
            //
            if (LON < AMNLON) {
                if (LON < (AMNLON - LONALI)) {
                    //
                    // See whether an aliased version of LON is a match.
                    //
                    LON = (LON + save.PI2);
                } else {
                    //
                    // Consider LON to be a match with the lower bound.
                    //
                    LON = AMNLON;
                }
            } else if (LON > AMXLON) {
                if (LON > (AMXLON + LONALI)) {
                    //
                    // See whether an aliased version of LON is a match.
                    //
                    LON = (LON - save.PI2);
                } else {
                    //
                    // Consider LON to be a match with the upper bound.
                    //
                    LON = AMXLON;
                }
            }

            if ((LON < AMNLON) || (LON > AMXLON)) {
                CHKOUT(b"ZZINPDT", ctx)?;
                return Ok(());
            }
        } else {

            //
            // The latitude of the input point is close to one of the
            // poles.
            //
            // This is a no-op case.
            //
            // The input point has already passed whichever of the
            // altitude and latitude tests that were not excluded.
            //
            // If the element has a non-degenerate latitude boundary
            // having the same sign as the latitude of the input point,
            // and if latitude is excluded because the input point is
            // already nominally on that boundary, then passing the
            // altitude check implies that the point is close to the
            // element.
            //
            // If the element has a degenerate latitude boundary having
            // the same sign as the latitude of the input point---namely,
            // the element contains the pole, and latitude is excluded,
            // then then passing the altitude check implies that the point
            // is close to the portion of the Z-axis contained in the
            // element.
            //
            // If the altitude check has been excluded because the point
            // is already nominally on one of the element's altitude
            // boundaries, the passing the latitude test implies the point
            // is close to the element.
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

    CHKOUT(b"ZZINPDT", ctx)?;
    Ok(())
}
