//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const NONE: i32 = 0;
const LOWER: i32 = 1;
const UPPER: i32 = 2;

//$Procedure ZZINREC ( DSK, in rectangular element? )
pub fn ZZINREC(
    P: &[f64],
    BOUNDS: &[f64],
    MARGIN: f64,
    EXCLUD: i32,
    INSIDE: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let P = DummyArray::new(P, 1..=3);
    let BOUNDS = DummyArray2D::new(BOUNDS, 1..=2, 1..=3);
    let mut AMNCOR = StackArray::<f64, 3>::new(1..=3);
    let mut AMXCOR = StackArray::<f64, 3>::new(1..=3);
    let mut DELTA = StackArray::<f64, 3>::new(1..=3);
    let mut MAXCOR = StackArray::<f64, 3>::new(1..=3);
    let mut MINCOR = StackArray::<f64, 3>::new(1..=3);
    let mut L = StackArray::<f64, 3>::new(1..=3);

    //
    // SPICELIB functions
    //

    //
    // Element boundary indices:
    //

    //
    // Local variables
    //

    //
    // Check-in is discovery style.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // Assume the point is outside to start. This allows us
    // to skip setting INSIDE when we find a boundary test
    // failure.
    //
    *INSIDE = false;

    //
    // Reject negative margins.
    //
    if (MARGIN < 0.0) {
        CHKIN(b"ZZINREC", ctx)?;
        SETMSG(b"Margin must be non-negative but was #.", ctx);
        ERRDP(b"#", MARGIN, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZINREC", ctx)?;
        return Ok(());
    }

    //
    // Check the exclusion index.
    //
    if ((EXCLUD < NONE) || (EXCLUD > 3)) {
        CHKIN(b"ZZINREC", ctx)?;
        SETMSG(b"EXCLUD was #; allowed range is 0:3.", ctx);
        ERRINT(b"#", EXCLUD, ctx);
        SIGERR(b"SPICE(INDEXOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZINREC", ctx)?;
        return Ok(());
    }

    //
    // Get local copies of the coordinate bounds.
    //
    for I in 1..=3 {
        MINCOR[I] = BOUNDS[[LOWER, I]];
        MAXCOR[I] = BOUNDS[[UPPER, I]];
        L[I] = (MAXCOR[I] - MINCOR[I]);

        if (L[I] < 0.0) {
            CHKIN(b"ZZINREC", ctx)?;
            SETMSG(b"Bounds are out of order for index #; bounds are #:#.", ctx);
            ERRDP(b"#", BOUNDS[[LOWER, I]], ctx);
            ERRDP(b"#", BOUNDS[[UPPER, I]], ctx);
            SIGERR(b"SPICE(BOUNDSOUTOFORDER)", ctx)?;
            CHKOUT(b"ZZINREC", ctx)?;
            return Ok(());
        }
    }

    //
    // Compare coordinates to adjusted coordinate
    // boundaries.
    //
    for I in 1..=3 {
        if (EXCLUD != I) {
            //
            // Create adjusted bounds for the Ith coordinate.
            //
            DELTA[I] = (MARGIN * f64::abs(L[I]));

            AMNCOR[I] = (MINCOR[I] - DELTA[I]);
            AMXCOR[I] = (MAXCOR[I] + DELTA[I]);

            if ((P[I] < AMNCOR[I]) || (P[I] > AMXCOR[I])) {
                return Ok(());
            }
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
