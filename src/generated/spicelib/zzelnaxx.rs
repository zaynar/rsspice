//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure ZZELNAXX ( ellipse normal axis intercepts )
pub fn ZZELNAXX(
    A: f64,
    B: f64,
    LAT: f64,
    XXPT: &mut f64,
    YXPT: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut EPT = StackArray::<f64, 3>::new(1..=3);
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    if RETURN(ctx) {
        return Ok(());
    }

    if ((A <= 0.0) || (B <= 0.0)) {
        CHKIN(b"ZZELNAXX", ctx)?;
        SETMSG(
            b"Semi-axis lengths were A = #; B = #. Both must be positive.",
            ctx,
        );
        ERRDP(b"#", A, ctx);
        ERRDP(b"#", B, ctx);
        SIGERR(b"SPICE(NONPOSITIVEAXIS)", ctx)?;
        CHKOUT(b"ZZELNAXX", ctx)?;
        return Ok(());
    }

    //
    // Find the point lying on the positive X portion of the ellipsoid
    // and having the input planetodetic latitude.
    //
    // To start, create a normal vector pointing in the direction
    // indicated by the latitude. We'll work in three dimensions in
    // order to take advantage of existing code. The third coordinates
    // of all participating vectors will be zero.
    //
    NORMAL[1] = f64::cos(LAT);
    NORMAL[2] = f64::sin(LAT);
    NORMAL[3] = 0.0;

    EDNMPT(A, B, B, NORMAL.as_slice(), EPT.as_slice_mut(), ctx)?;

    //
    // Compute the X-axis and Y-axis intercepts of the line
    // passing through EPT and parallel to a normal vector
    // at EPT. Refer to the Particulars above for details.
    //
    *XXPT = ((1.0 - f64::powi((B / A), 2)) * EPT[1]);

    *YXPT = ((1.0 - f64::powi((A / B), 2)) * EPT[2]);

    Ok(())
}
