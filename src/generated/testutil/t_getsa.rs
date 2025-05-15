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
const SSB: i32 = 0;

//$Procedure      T_GETSA ( Test utility, get stellar aberration )
pub fn T_GETSA(
    TARG: i32,
    ET: f64,
    REF: &[u8],
    ABCORR: &[u8],
    OBS: i32,
    PCORR: &mut [f64],
    DPCORR: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut PCORR = DummyArrayMut::new(PCORR, 1..=3);
    let mut DPCORR = DummyArrayMut::new(DPCORR, 1..=3);
    let mut ACC = StackArray::<f64, 3>::new(1..=3);
    let mut DELTA: f64 = 0.0;
    let mut DLT: f64 = 0.0;
    let mut DLTCTR: f64 = 0.0;
    let mut LT: f64 = 0.0;
    let mut LTCENT: f64 = 0.0;
    let mut LTSIGN: f64 = 0.0;
    let mut LTSSB: f64 = 0.0;
    let mut SASTAT = StackArray::<f64, 6>::new(1..=6);
    let mut SSBOBS = StackArray::<f64, 6>::new(1..=6);
    let mut STATE = StackArray::<f64, 6>::new(1..=6);
    let mut STCTR = StackArray::<f64, 6>::new(1..=6);
    let mut STOBS = StackArray2D::<f64, 12>::new(1..=6, 1..=2);
    let mut T: f64 = 0.0;
    let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XSTATE = StackArray::<f64, 6>::new(1..=6);
    let mut CENTER: i32 = 0;
    let mut REQFRM: i32 = 0;
    let mut TYPE: i32 = 0;
    let mut TYPEID: i32 = 0;
    let mut ATTBLK = StackArray::<bool, 6>::new(1..=ABATSZ);
    let mut FOUND: bool = false;
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

    spicelib::CHKIN(b"T_GETSA", ctx)?;

    //
    // Compute the corrections in the J2000 frame. We'll
    // transform them later to the requested frame.
    //
    spicelib::ZZPRSCOR(ABCORR, ATTBLK.as_slice_mut(), ctx)?;
    XMIT = ATTBLK[XMTIDX];

    DELTA = 1.0;

    //
    // Determine the acceleration of the observer with
    // respect to the solar system barycenter at ET.
    // Use a quadratic estimate.
    //
    for I in 1..=2 {
        T = (ET + ((((2 * I) - 3) as f64) * DELTA));

        spicelib::SPKGEO(
            OBS,
            T,
            b"J2000",
            SSB,
            STOBS.subarray_mut([1, I]),
            &mut LTSSB,
            ctx,
        )?;
    }

    spicelib::QDERIV(
        3,
        STOBS.subarray([4, 1]),
        STOBS.subarray([4, 2]),
        DELTA,
        ACC.as_slice_mut(),
        ctx,
    )?;

    //
    // Get the light-time corrected state of the target with respect
    // to the observer. Also get the geometric state of the observer
    // with respect to the solar system barycenter at ET.
    //
    spicelib::SPKSSB(OBS, ET, b"J2000", SSBOBS.as_slice_mut(), ctx)?;

    spicelib::SPKLTC(
        TARG,
        ET,
        b"J2000",
        ABCORR,
        SSBOBS.as_slice(),
        STATE.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;

    //
    // Get the stellar aberration correction and its time derivative.
    // Note that the input observer state relative to the solar system
    // barycenter was obtained from SPKLTC.
    //
    spicelib::ZZSTELAB(
        XMIT,
        ACC.as_slice(),
        SSBOBS.subarray(4),
        STATE.as_slice(),
        PCORR.as_slice_mut(),
        DPCORR.as_slice_mut(),
        ctx,
    )?;

    //
    // Get the transformation from J2000 to the request frame. This
    // transformation should be evaluated at the light-time corrected
    // epoch associated with the center of the request frame.
    //
    spicelib::NAMFRM(REF, &mut REQFRM, ctx)?;

    if (REQFRM == 0) {
        spicelib::SETMSG(b"The requested output frame \'#\' is not recognized by the reference frame subsystem.  Please check that the appropriate kernels have been loaded and that you have correctly entered the name of the output frame. ", ctx);
        spicelib::ERRCH(b"#", REF, ctx);
        spicelib::SIGERR(b"SPICE(UNKNOWNFRAME)", ctx)?;
        spicelib::CHKOUT(b"T_GETSA", ctx)?;
        return Ok(());
    }

    //
    // If we reach this point, FOUND will be .TRUE. after
    // the following call to FRINFO:
    //
    spicelib::FRINFO(REQFRM, &mut CENTER, &mut TYPE, &mut TYPEID, &mut FOUND, ctx)?;

    //
    // Get light time to the frame's center.
    //
    if (CENTER == OBS) {
        LTCENT = 0.0;
        DLTCTR = 0.0;
    } else if (CENTER == TARG) {
        LTCENT = LT;
        DLTCTR = DLT;
    } else {
        spicelib::SPKLTC(
            CENTER,
            ET,
            b"J2000",
            ABCORR,
            SSBOBS.as_slice(),
            STCTR.as_slice_mut(),
            &mut LTCENT,
            &mut DLTCTR,
            ctx,
        )?;
    }
    //
    // If something went wrong (like we couldn't get the state of
    // the center relative to the observer) now it is time to quit.
    //
    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"T_GETSA", ctx)?;
        return Ok(());
    }

    //
    // If the aberration corrections are for transmission, make the sign
    // of the light time positive, since we wish to compute the
    // orientation of the non-inertial frame at an epoch later than ET
    // by the one-way light time.
    //
    if XMIT {
        LTSIGN = 1 as f64;
    } else {
        LTSIGN = -1 as f64;
    }

    //
    // Look up the transformation from J2000 to REF at the light time
    // corrected epoch.
    //
    spicelib::SXFORM(
        b"J2000",
        REF,
        (ET + (LTSIGN * LTCENT)),
        XFORM.as_slice_mut(),
        ctx,
    )?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"T_GETSA", ctx)?;
        return Ok(());
    }

    //
    // The frame rotation we wish to apply is of the form
    //
    //    R(t)
    //
    // where
    //
    //    t = ET + LTSIGN*LTCENT(ET)
    //
    // since the light time from the frame center to the
    // observer LTCENT is a function of ET. So
    //
    //                          |
    //    d(R)/d(ET) = d(R)/d(t)|                  * (1 + d(t)/d(ET))
    //                          |t=ET+LTSIGN*LTCENT
    //
    //
    //                          |
    //               = d(R)/d(t)|                  * (1 + LTSIGN*DLTCTR)
    //                          |t=ET+LTSIGN*LTCENT
    //
    // Scale the derivative block of the transformation to account
    // for the rate of change of light time.
    //
    for I in 1..=3 {
        spicelib::VSCLIP((1.0 + (LTSIGN * DLTCTR)), XFORM.subarray_mut([4, I]));
    }

    //
    // Now apply the transformation to the stellar aberration
    // correction position and velocity. We'll pack these
    // components together as a state to simplify the multiplication.
    //
    spicelib::VEQU(PCORR.as_slice(), SASTAT.as_slice_mut());
    spicelib::VEQU(DPCORR.as_slice(), SASTAT.subarray_mut(4));

    spicelib::MXVG(
        XFORM.as_slice(),
        SASTAT.as_slice(),
        6,
        6,
        XSTATE.as_slice_mut(),
    );

    //
    // Now unpack the stellar aberration correction components.
    //
    spicelib::VEQU(XSTATE.as_slice(), PCORR.as_slice_mut());
    spicelib::VEQU(XSTATE.subarray(4), DPCORR.as_slice_mut());

    spicelib::CHKOUT(b"T_GETSA", ctx)?;
    Ok(())
}
