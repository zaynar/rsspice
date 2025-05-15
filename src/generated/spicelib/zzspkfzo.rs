//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const INERTL: i32 = 1;
const PCK: i32 = (INERTL + 1);
const CK: i32 = (PCK + 1);
const TK: i32 = (CK + 1);
const DYN: i32 = (TK + 1);
const SWTCH: i32 = (DYN + 1);
const ALL: i32 = -1;
const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;
const RNAME: &[u8] = b"ZZSPKFZO";

struct SaveVars {
    PRVCOR: Vec<u8>,
    FJ2000: i32,
    PASS1: bool,
    USEGEO: bool,
    XMIT: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PRVCOR = vec![b' '; CORLEN as usize];
        let mut FJ2000: i32 = 0;
        let mut PASS1: bool = false;
        let mut USEGEO: bool = false;
        let mut XMIT: bool = false;

        PASS1 = true;
        fstr::assign(&mut PRVCOR, b" ");

        Self {
            PRVCOR,
            FJ2000,
            PASS1,
            USEGEO,
            XMIT,
        }
    }
}

//$Procedure ZZSPKFZO ( SPK function, easy reader, observer )
pub fn ZZSPKFZO(
    TARG: i32,
    ET: f64,
    REF: &[u8],
    ABCORR: &[u8],
    OBSSUB: fn(f64, &[u8], &mut i32, &mut [f64], &mut Context) -> f2rust_std::Result<()>,
    STARG: &mut [f64],
    LT: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut STARG = DummyArrayMut::new(STARG, 1..=6);
    let mut CORXFM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut DLT: f64 = 0.0;
    let mut DLTCTR: f64 = 0.0;
    let mut LTCENT: f64 = 0.0;
    let mut STATE = StackArray::<f64, 6>::new(1..=6);
    let mut STEMP = StackArray::<f64, 6>::new(1..=6);
    let mut STOBS = StackArray::<f64, 6>::new(1..=6);
    let mut STOCTR = StackArray::<f64, 6>::new(1..=6);
    let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut CENTER: i32 = 0;
    let mut LTSIGN: i32 = 0;
    let mut OBSCTR: i32 = 0;
    let mut REQFRM: i32 = 0;
    let mut TYPE: i32 = 0;
    let mut TYPEID: i32 = 0;
    let mut ATTBLK = StackArray::<bool, 15>::new(1..=NABCOR);
    let mut FOUND: bool = false;

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

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(RNAME, ctx)?;

    if (save.PASS1 || fstr::ne(ABCORR, &save.PRVCOR)) {
        //
        // The aberration correction flag differs from the value it
        // had on the previous call, if any. Analyze the new flag.
        //
        ZZVALCOR(ABCORR, ATTBLK.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // The aberration correction flag is recognized; save it.
        //
        fstr::assign(&mut save.PRVCOR, ABCORR);

        //
        // Set logical flags indicating the attributes of the requested
        // correction:
        //
        //    XMIT is .TRUE. when the correction is for transmitted
        //    radiation.
        //
        //    USEGEO indicates geometric state computation.
        //
        // The above definitions are consistent with those used by
        // ZZVALCOR.
        //
        save.XMIT = ATTBLK[XMTIDX];
        save.USEGEO = ATTBLK[GEOIDX];

        //
        // Get the frame ID for J2000 on the first call to this routine.
        //
        if save.PASS1 {
            NAMFRM(b"J2000", &mut save.FJ2000, ctx)?;

            save.PASS1 = false;
        }
    }

    //
    // If we only want a geometric state, then use SPKGEO to compute
    // just that.
    //
    // Otherwise, if REF is inertial, compute the state of the target
    // relative to the observer via SPKACS. If REF is non-inertial,
    // compute the requested state in the J2000 frame, then transform it
    // to the frame designated by REF.
    //
    if save.USEGEO {
        //
        // Get the state of the target relative to its center at ET.
        //
        // Start by getting the state of the observer with respect to its
        // center of motion; subtract this from the state of the target
        // with respect to this center.
        //
        OBSSUB(ET, REF, &mut OBSCTR, STOCTR.as_slice_mut(), ctx)?;

        SPKGEO(
            TARG,
            ET,
            REF,
            OBSCTR,
            STEMP.as_slice_mut(),
            &mut LTCENT,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        VSUBG(STEMP.as_slice(), STOCTR.as_slice(), 6, STARG.as_slice_mut());

        *LT = (VNORM(STARG.as_slice()) / CLIGHT());
    } else {
        //
        // Get the auxiliary information about the requested output
        // frame.
        //
        NAMFRM(REF, &mut REQFRM, ctx)?;

        if (REQFRM == 0) {
            SETMSG(b"The requested output frame \'#\' is not recognized by the reference frame subsystem. Please check that the appropriate kernels have been loaded and that you have correctly entered the name of the output frame. ", ctx);
            ERRCH(b"#", REF, ctx);
            SIGERR(b"SPICE(UNKNOWNFRAME)", ctx)?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        FRINFO(REQFRM, &mut CENTER, &mut TYPE, &mut TYPEID, &mut FOUND, ctx)?;

        //
        // If we are dealing with an inertial frame, we can simply
        // call ZZSPKFAO and return.
        //
        if (TYPE == INERTL) {
            ZZSPKFAO(
                TARG,
                ET,
                REF,
                ABCORR,
                OBSSUB,
                STARG.as_slice_mut(),
                LT,
                &mut DLT,
                ctx,
            )?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // Still here?
        //
        // We are dealing with a non-inertial frame. But we need to do
        // light time and stellar aberration corrections in an inertial
        // frame. Get the "apparent" state of TARG in the intermediary
        // inertial reference frame J2000.
        //
        // We also need the light time to the center of the frame.
        // We compute that first so that we can re-use the temporary
        // variable STATE when we compute the inertial apparent state
        // of the target relative to the observer.
        //
        ZZSPKFAO(
            TARG,
            ET,
            b"J2000",
            ABCORR,
            OBSSUB,
            STATE.as_slice_mut(),
            LT,
            &mut DLT,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // If the frame is centered at the target, the light time and
        // light time rate have both been computed already. If the frame
        // is centered elsewhere, we'll need to obtain the light time
        // between the observer and the frame center. Unlike SPKEZ, we
        // have no case here for the frame centered at the observer,
        // since the observer isn't an ephemeris object, as far as this
        // routine can determine.
        //
        if (CENTER == TARG) {
            //
            // We already have the light time and light time rate
            // for the frame center as seen from the observer.
            //
            LTCENT = *LT;
            DLTCTR = DLT;
        } else {
            OBSSUB(ET, b"J2000", &mut OBSCTR, STOCTR.as_slice_mut(), ctx)?;
            SPKSSB(OBSCTR, ET, b"J2000", STEMP.as_slice_mut(), ctx)?;

            if FAILED(ctx) {
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            VADDG(STEMP.as_slice(), STOCTR.as_slice(), 6, STOBS.as_slice_mut());

            SPKLTC(
                CENTER,
                ET,
                b"J2000",
                ABCORR,
                STOBS.as_slice(),
                STEMP.as_slice_mut(),
                &mut LTCENT,
                &mut DLTCTR,
                ctx,
            )?;
        }

        //
        // If something went wrong (like we couldn't get the state of
        // the center relative to the observer) now it is time to quit.
        //
        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // If the aberration corrections are for transmission, make the
        // sign of the light time positive, since we wish to compute the
        // orientation of the non-inertial frame at an epoch later than
        // ET by the one-way light time.
        //
        if save.XMIT {
            LTSIGN = 1;
        } else {
            LTSIGN = -1;
        }

        //
        // Get the state transformation from J2000 to the requested frame
        // and convert the state.
        //
        FRMCHG(
            save.FJ2000,
            REQFRM,
            (ET + ((LTSIGN as f64) * LTCENT)),
            XFORM.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // There's a tricky bit here:  since XFORM is evaluated
        // at time
        //
        //    ET + LTSIGN*LTCENT
        //
        // XFORM is actually dependent on LTCENT. We need to account for
        // this dependency in our velocity transformation.
        //
        ZZCORSXF(save.XMIT, DLTCTR, XFORM.as_slice(), CORXFM.as_slice_mut());

        //
        // Now apply the frame transformation CORXFM to produce the
        // state expressed relative to the request frame REQFRM.
        //
        MXVG(
            CORXFM.as_slice(),
            STATE.as_slice(),
            6,
            6,
            STARG.as_slice_mut(),
        );
    }

    CHKOUT(RNAME, ctx)?;
    Ok(())
}
