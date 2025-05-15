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

//$Procedure ZZCHRLAT ( Chord latitude  )
pub fn ZZCHRLAT(
    MIDLAT: f64,
    DLON: f64,
    EPTLAT: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut MLAT: f64 = 0.0;

    //
    // SPICELIB functions
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

    if ((DLON < 0.0) || (DLON >= PI(ctx))) {
        CHKIN(b"ZZCHRLAT", ctx)?;
        SETMSG(
            b"The input longitude extent was #; this value must be in the range [0 : pi ) radians.",
            ctx,
        );
        ERRDP(b"#", DLON, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZCHRLAT", ctx)?;
        return Ok(());
    }

    if (f64::abs(MIDLAT) > (HALFPI(ctx) + ANGMRG)) {
        CHKIN(b"ZZCHRLAT", ctx)?;
        SETMSG(
            b"The input latitude was #; this value must be in the interval -pi/2 : pi/2 (radians).",
            ctx,
        );
        ERRDP(b"#", MIDLAT, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZCHRLAT", ctx)?;
        return Ok(());
    }

    //
    // The input latitude is, at worst, slightly out of range.
    // Bracket it.
    //
    MLAT = BRCKTD(MIDLAT, -HALFPI(ctx), HALFPI(ctx));

    //
    // The endpoint latitude EPTLAT is defined by
    //
    //    EPTLAT = atan ( tan(MLAT) * cos( DLON/2 ) )
    //
    // For numerical robustness, we'll re-write this using
    // the two-argument arctangent function and well-behaved
    // trig functions as input arguments:
    //
    *EPTLAT = f64::atan2(
        (f64::sin(MLAT) * f64::cos((DLON / 2 as f64))),
        f64::cos(MLAT),
    );

    Ok(())
}
