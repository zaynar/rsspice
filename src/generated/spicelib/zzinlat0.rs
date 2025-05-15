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
const LOWER: i32 = 1;
const UPPER: i32 = 2;

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

//$Procedure ZZINLAT0 ( DSK, in latitudinal element, w/o margin? )
pub fn ZZINLAT0(
    R: f64,
    LON: f64,
    LAT: f64,
    BOUNDS: &[f64],
    EXCLUD: i32,
    INSIDE: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let BOUNDS = DummyArray2D::new(BOUNDS, 1..=2, 1..=3);
    let mut LOCLON: f64 = 0.0;
    let mut MAXLAT: f64 = 0.0;
    let mut MAXLON: f64 = 0.0;
    let mut MAXR: f64 = 0.0;
    let mut MINLAT: f64 = 0.0;
    let mut MINLON: f64 = 0.0;
    let mut MINR: f64 = 0.0;

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

    if save.FIRST {
        save.PI2 = TWOPI(ctx);
        save.FIRST = false;
    }

    //
    // Assume the point is outside to start. This allows us
    // to skip setting INSIDE when we find a boundary test
    // failure.
    //
    *INSIDE = false;
    //
    // Compare coordinates of the input point and the segment
    // bounds.
    //
    // Special case: if the input point is at the origin, and the
    // volume element contains the origin, the point is inside.
    //
    if (R == 0.0) {
        if (BOUNDS[[LOWER, RADIDX]] == 0.0) {
            *INSIDE = true;

            return Ok(());
        }
    }

    if (EXCLUD != RADIDX) {
        //
        // Compare the point's radius to the segment's radius bounds.
        //
        MINR = BOUNDS[[LOWER, RADIDX]];
        MAXR = BOUNDS[[UPPER, RADIDX]];

        if ((R < MINR) || (R > MAXR)) {
            //
            // The point's radius is outside of the segment's range.
            //
            return Ok(());
        }
    }

    if (EXCLUD != LATIDX) {
        //
        // Compare the point's latitude to the segment's latitude bounds.
        //
        MINLAT = BOUNDS[[LOWER, LATIDX]];
        MAXLAT = BOUNDS[[UPPER, LATIDX]];

        if ((LAT < MINLAT) || (LAT > MAXLAT)) {
            //
            // The point's latitude is outside of the segment's range.
            //
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
            BOUNDS[[1, LONIDX]],
            BOUNDS[[2, LONIDX]],
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
            return Ok(());
        }
    }
    //
    // Getting to this point means the input point is inside
    // the segment. Being on the boundary counts as inside.
    //
    *INSIDE = true;

    Ok(())
}
