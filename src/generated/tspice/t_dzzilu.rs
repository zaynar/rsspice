//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const DELTA: f64 = 240.0;

//$Procedure T_DZZILU ( Illumination angle rates )
pub fn T_DZZILU(
    METHOD: &[u8],
    TARGET: &[u8],
    ILLUM: &[u8],
    ET: f64,
    FIXREF: &[u8],
    ABCORR: &[u8],
    OBSRVR: &[u8],
    SPOINT: &[f64],
    NORMAL: &[f64],
    DPHASE: &mut f64,
    DINCDN: &mut f64,
    DEMSSN: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let SPOINT = DummyArray::new(SPOINT, 1..=3);
    let NORMAL = DummyArray::new(NORMAL, 1..=3);
    let mut F: f64 = 0.0;
    let mut RETVAL: f64 = 0.0;
    let mut NTERMS: i32 = 0;
    let mut NRTAIN: i32 = 0;

    //
    // External routines
    //

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"T_DZZILU", ctx)?;

    //
    // Initialize angle computation functions:
    // store inputs to ZZILUMIN.
    //
    RETVAL = T_ILUINI(
        METHOD,
        TARGET,
        ILLUM,
        FIXREF,
        ABCORR,
        OBSRVR,
        SPOINT.as_slice(),
        NORMAL.as_slice(),
        ctx,
    )?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"T_DZZILU", ctx)?;
        return Ok(());
    }

    //
    // Set the number of terms in the Chebyshev expansions
    // we'll use. Set the number of retained terms as well.
    //
    NTERMS = 25;
    NRTAIN = 6;
    //
    // Estimate the derivatives of the illumination angles
    // at ET.
    //
    testutil::T_DCHEB(T_ILUPHS, ET, DELTA, NTERMS, NRTAIN, &mut F, DPHASE, ctx)?;
    testutil::T_DCHEB(T_ILUINC, ET, DELTA, NTERMS, NRTAIN, &mut F, DINCDN, ctx)?;
    testutil::T_DCHEB(T_ILUEMI, ET, DELTA, NTERMS, NRTAIN, &mut F, DEMSSN, ctx)?;

    spicelib::CHKOUT(b"T_DZZILU", ctx)?;
    Ok(())
}
