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
const RNAME: &[u8] = b"ZZSPKFZT";

struct SaveVars {
    PRVCOR: Vec<u8>,
    CORXFM: StackArray2D<f64, 36>,
    DLT: f64,
    DLTCTR: f64,
    LTCENT: f64,
    STATE: StackArray<f64, 6>,
    STEMP: StackArray<f64, 6>,
    STOBS: StackArray<f64, 6>,
    STTCTR: StackArray<f64, 6>,
    XFORM: StackArray2D<f64, 36>,
    CENTER: i32,
    FJ2000: i32,
    LTSIGN: i32,
    REQFRM: i32,
    TRGCTR: i32,
    TYPE: i32,
    TYPEID: i32,
    ATTBLK: StackArray<bool, 15>,
    FIRST: bool,
    FOUND: bool,
    USEGEO: bool,
    XMIT: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PRVCOR = vec![b' '; CORLEN as usize];
        let mut CORXFM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
        let mut DLT: f64 = 0.0;
        let mut DLTCTR: f64 = 0.0;
        let mut LTCENT: f64 = 0.0;
        let mut STATE = StackArray::<f64, 6>::new(1..=6);
        let mut STEMP = StackArray::<f64, 6>::new(1..=6);
        let mut STOBS = StackArray::<f64, 6>::new(1..=6);
        let mut STTCTR = StackArray::<f64, 6>::new(1..=6);
        let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
        let mut CENTER: i32 = 0;
        let mut FJ2000: i32 = 0;
        let mut LTSIGN: i32 = 0;
        let mut REQFRM: i32 = 0;
        let mut TRGCTR: i32 = 0;
        let mut TYPE: i32 = 0;
        let mut TYPEID: i32 = 0;
        let mut ATTBLK = StackArray::<bool, 15>::new(1..=NABCOR);
        let mut FIRST: bool = false;
        let mut FOUND: bool = false;
        let mut USEGEO: bool = false;
        let mut XMIT: bool = false;

        FIRST = true;
        fstr::assign(&mut PRVCOR, b" ");

        Self {
            PRVCOR,
            CORXFM,
            DLT,
            DLTCTR,
            LTCENT,
            STATE,
            STEMP,
            STOBS,
            STTCTR,
            XFORM,
            CENTER,
            FJ2000,
            LTSIGN,
            REQFRM,
            TRGCTR,
            TYPE,
            TYPEID,
            ATTBLK,
            FIRST,
            FOUND,
            USEGEO,
            XMIT,
        }
    }
}

//$Procedure ZZSPKFZT ( SPK function, easy reader, target )
pub fn ZZSPKFZT(
    TRGSUB: fn(f64, &[u8], &mut i32, &mut [f64], &mut Context) -> f2rust_std::Result<()>,
    ET: f64,
    REF: &[u8],
    ABCORR: &[u8],
    OBS: i32,
    STARG: &mut [f64],
    LT: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut STARG = DummyArrayMut::new(STARG, 1..=6);

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
    } else {
        CHKIN(RNAME, ctx)?;
    }

    if (save.FIRST || fstr::ne(ABCORR, &save.PRVCOR)) {
        //
        // The aberration correction flag differs from the value it
        // had on the previous call, if any. Analyze the new flag.
        //
        ZZVALCOR(ABCORR, save.ATTBLK.as_slice_mut(), ctx)?;

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
        save.XMIT = save.ATTBLK[XMTIDX];
        save.USEGEO = save.ATTBLK[GEOIDX];

        //
        // Get the frame ID for J2000 on the first call to this routine.
        //
        if save.FIRST {
            NAMFRM(b"J2000", &mut save.FJ2000, ctx)?;

            save.FIRST = false;
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
        // Add the state of the target's center to obtain the state
        // of the target with respect to the observer.
        //
        TRGSUB(ET, REF, &mut save.TRGCTR, save.STTCTR.as_slice_mut(), ctx)?;

        SPKGEO(
            save.TRGCTR,
            ET,
            REF,
            OBS,
            save.STEMP.as_slice_mut(),
            &mut save.LTCENT,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        VADDG(
            save.STTCTR.as_slice(),
            save.STEMP.as_slice(),
            6,
            STARG.as_slice_mut(),
        );

        *LT = (VNORM(STARG.as_slice()) / CLIGHT());
    } else {
        //
        // Get the auxiliary information about the requested output
        // frame.
        //
        NAMFRM(REF, &mut save.REQFRM, ctx)?;

        if (save.REQFRM == 0) {
            SETMSG(b"The requested output frame \'#\' is not recognized by the reference frame subsystem. Please check that the appropriate kernels have been loaded and that you have correctly entered the name of the output frame. ", ctx);
            ERRCH(b"#", REF, ctx);
            SIGERR(b"SPICE(UNKNOWNFRAME)", ctx)?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        FRINFO(
            save.REQFRM,
            &mut save.CENTER,
            &mut save.TYPE,
            &mut save.TYPEID,
            &mut save.FOUND,
            ctx,
        )?;

        //
        // If we are dealing with an inertial frame, we can simply
        // call ZZSPKFAT and return.
        //
        if (save.TYPE == INERTL) {
            ZZSPKFAT(
                TRGSUB,
                ET,
                REF,
                ABCORR,
                OBS,
                STARG.as_slice_mut(),
                LT,
                &mut save.DLT,
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
        ZZSPKFAT(
            TRGSUB,
            ET,
            b"J2000",
            ABCORR,
            OBS,
            save.STATE.as_slice_mut(),
            LT,
            &mut save.DLT,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // If the frame is centered at the observer, the light time and
        // light time rate are both zero. If the frame is centered
        // elsewhere, we'll need to obtain the light time between the
        // observer and the frame center. Unlike SPKEZ, we have no case
        // here for the frame centered at the target, since the target
        // isn't an ephemeris object, as far as this routine can
        // determine.
        //
        if (save.CENTER == OBS) {
            save.LTCENT = 0.0;
            save.DLTCTR = 0.0;
        } else {
            SPKSSB(OBS, ET, b"J2000", save.STOBS.as_slice_mut(), ctx)?;
            SPKLTC(
                save.CENTER,
                ET,
                b"J2000",
                ABCORR,
                save.STOBS.as_slice(),
                save.STEMP.as_slice_mut(),
                &mut save.LTCENT,
                &mut save.DLTCTR,
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
            save.LTSIGN = 1;
        } else {
            save.LTSIGN = -1;
        }

        //
        // Get the state transformation from J2000 to the requested frame
        // and convert the state.
        //
        FRMCHG(
            save.FJ2000,
            save.REQFRM,
            (ET + ((save.LTSIGN as f64) * save.LTCENT)),
            save.XFORM.as_slice_mut(),
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
        ZZCORSXF(
            save.XMIT,
            save.DLTCTR,
            save.XFORM.as_slice(),
            save.CORXFM.as_slice_mut(),
        );

        //
        // Now apply the frame transformation CORXFM to produce the
        // state expressed relative to the request frame REQFRM.
        //
        MXVG(
            save.CORXFM.as_slice(),
            save.STATE.as_slice(),
            6,
            6,
            STARG.as_slice_mut(),
        );
    }

    CHKOUT(RNAME, ctx)?;
    Ok(())
}
