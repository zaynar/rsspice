//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;
const IREF: &[u8] = b"J2000";

//$Procedure ZZILUSTA ( Illumination angle states )
pub fn ZZILUSTA(
    METHOD: &[u8],
    TARGET: &[u8],
    ILLUM: &[u8],
    ET: f64,
    FIXREF: &[u8],
    ABCORR: &[u8],
    OBSRVR: &[u8],
    SPOINT: &[f64],
    NORMAL: &[f64],
    PHSSTA: &mut [f64],
    INCSTA: &mut [f64],
    EMISTA: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let SPOINT = DummyArray::new(SPOINT, 1..=3);
    let NORMAL = DummyArray::new(NORMAL, 1..=3);
    let mut PHSSTA = DummyArrayMut::new(PHSSTA, 1..=2);
    let mut INCSTA = DummyArrayMut::new(INCSTA, 1..=2);
    let mut EMISTA = DummyArrayMut::new(EMISTA, 1..=2);
    let mut DLT: f64 = 0.0;
    let mut ETSURF: f64 = 0.0;
    let mut FXNSTA = StackArray::<f64, 6>::new(1..=6);
    let mut LT: f64 = 0.0;
    let mut LTSRC: f64 = 0.0;
    let mut NRMSTA = StackArray::<f64, 6>::new(1..=6);
    let mut OBSSTA = StackArray::<f64, 6>::new(1..=6);
    let mut SRCSTA = StackArray::<f64, 6>::new(1..=6);
    let mut STARG = StackArray::<f64, 6>::new(1..=6);
    let mut TMPXFM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut UVEC = StackArray::<f64, 3>::new(1..=3);
    let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut ATTBLK = StackArray::<bool, 6>::new(1..=ABATSZ);
    let mut USELT: bool = false;
    let mut XMIT: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZILUSTA", ctx)?;

    //
    // For now, only ellipsoids are supported as target shapes.
    //
    if !EQSTR(METHOD, b"ELLIPSOID") {
        SETMSG(b"The computation method # was not recognized. ", ctx);
        ERRCH(b"#", METHOD, ctx);
        SIGERR(b"SPICE(INVALIDMETHOD)", ctx)?;
        CHKOUT(b"ZZILUSTA", ctx)?;
        return Ok(());
    }

    //
    // Reject zero normal vectors.
    //
    if VZERO(NORMAL.as_slice()) {
        SETMSG(
            b"The input normal vector must not be zero, but sadly, it was.",
            ctx,
        );
        SIGERR(b"SPICE(ZEROVECTOR)", ctx)?;
        CHKOUT(b"ZZILUSTA", ctx)?;
        return Ok(());
    }

    //
    // Look up the state of the target with respect to the
    // observer. We'll represent the state in an inertial
    // reference frame.
    //
    SPKCPT(
        SPOINT.as_slice(),
        TARGET,
        FIXREF,
        ET,
        IREF,
        b"TARGET",
        ABCORR,
        OBSRVR,
        STARG.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    //
    // Compute the epoch associated with the surface point.
    //
    ZZCOREPC(ABCORR, ET, LT, &mut ETSURF, ctx)?;

    //
    // Now let the surface point be the observer, let the observation
    // epoch be ETSURF, and find the apparent state of the illumination
    // source as seen from the surface point.
    //
    SPKCPO(
        ILLUM,
        ETSURF,
        IREF,
        b"OBSERVER",
        ABCORR,
        SPOINT.as_slice(),
        TARGET,
        FIXREF,
        SRCSTA.as_slice_mut(),
        &mut LTSRC,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZILUSTA", ctx)?;
        return Ok(());
    }

    //
    // We will need to transform the state of the normal vector to
    // the inertial frame. The epoch at which the transformation must be
    // evaluated is that associated with the surface point.

    SXFORM(FIXREF, IREF, ETSURF, XFORM.as_slice_mut(), ctx)?;

    //
    // Correct the body-fixed to inertial frame transformation for the
    // rate of change with respect to ET of observer-surface point light
    // time, if light time corrections are used.
    //
    // Start out by parsing ABCORR.
    //
    ZZVALCOR(ABCORR, ATTBLK.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZILUSTA", ctx)?;
        return Ok(());
    }

    USELT = ATTBLK[LTIDX];
    XMIT = ATTBLK[XMTIDX];

    if XMIT {
        SETMSG(b"Aberration correction # is for transmission; only reception corrections are supported by this routine.", ctx);
        ERRCH(b"#", ABCORR, ctx);
        SIGERR(b"SPICE(INVALIDOPTION)", ctx)?;
        CHKOUT(b"ZZILUSTA", ctx)?;
        return Ok(());
    }

    if USELT {
        //
        // Compute the rate of change with respect to ET of the
        // observer-surface point light time. This rate is the range rate
        // divided by the speed of light.
        //
        VHAT(STARG.as_slice(), UVEC.as_slice_mut());

        DLT = (VDOT(STARG.subarray(4), UVEC.as_slice()) / CLIGHT());
        //
        // Correct the state transformation.
        //
        ZZCORSXF(false, DLT, XFORM.as_slice(), TMPXFM.as_slice_mut());

        MOVED(TMPXFM.as_slice(), 36, XFORM.as_slice_mut());
    }

    //
    // Create a body-fixed state vector for the normal vector.
    // Convert the normal vector to unit length for safety.
    //
    VHAT(NORMAL.as_slice(), FXNSTA.as_slice_mut());
    CLEARD(3, FXNSTA.subarray_mut(4));

    //
    // Transform the state of the normal vector to the inertial
    // frame.
    //
    MXVG(
        XFORM.as_slice(),
        FXNSTA.as_slice(),
        6,
        6,
        NRMSTA.as_slice_mut(),
    );

    //
    // We also must adjust the state of the illumination source for the
    // rate of change with respect to ET of the observer-surface point
    // light time. The velocity portion of the state we've computed is
    // the derivative with respect to ETSURF (time at the surface point)
    // of the surface point-illumination source vector. We must convert
    // this to a derivative with respect to ET.
    //
    // This code assumes reception corrections.
    //
    if USELT {
        //
        // ETSURF = ET - LT, so
        //
        // d(ETSURF) / d(ET) = ( 1 - DLT )
        //
        VSCLIP((1.0 - DLT), SRCSTA.subarray_mut(4));
    }

    //
    // The surface-point observer state we wish to use is the negative
    // of the observer-surface point state.
    //
    VMINUG(STARG.as_slice(), 6, OBSSTA.as_slice_mut());

    //
    // Compute the state (value and rate of change )
    // of the phase angle.
    //
    PHSSTA[1] = VSEP(OBSSTA.as_slice(), SRCSTA.as_slice(), ctx);
    PHSSTA[2] = DVSEP(OBSSTA.as_slice(), SRCSTA.as_slice(), ctx)?;

    //
    // Compute the state of the illumination source
    // incidence angle.
    //
    INCSTA[1] = VSEP(NRMSTA.as_slice(), SRCSTA.as_slice(), ctx);
    INCSTA[2] = DVSEP(NRMSTA.as_slice(), SRCSTA.as_slice(), ctx)?;

    //
    // Compute the state of the emission angle.
    //
    EMISTA[1] = VSEP(NRMSTA.as_slice(), OBSSTA.as_slice(), ctx);
    EMISTA[2] = DVSEP(NRMSTA.as_slice(), OBSSTA.as_slice(), ctx)?;

    CHKOUT(b"ZZILUSTA", ctx)?;
    Ok(())
}
