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
const CTRSIZ: i32 = 2;
const RNAME: &[u8] = b"ZZSPKEZ1";
const FRNMLN: i32 = 32;

struct SaveVars {
    PRVCOR: Vec<u8>,
    DLT: f64,
    DLTCTR: f64,
    LTCENT: f64,
    STATE: StackArray<f64, 6>,
    STOBS: StackArray<f64, 6>,
    TEMP: StackArray<f64, 6>,
    XFORM: StackArray2D<f64, 36>,
    CENTER: i32,
    FJ2000: i32,
    LTSIGN: i32,
    REQFRM: i32,
    TYPE: i32,
    TYPEID: i32,
    ATTBLK: StackArray<bool, 15>,
    FOUND: bool,
    USEGEO: bool,
    XMIT: bool,
    SVCTR1: StackArray<i32, 2>,
    SVREF: Vec<u8>,
    SVREQF: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PRVCOR = vec![b' '; CORLEN as usize];
        let mut DLT: f64 = 0.0;
        let mut DLTCTR: f64 = 0.0;
        let mut LTCENT: f64 = 0.0;
        let mut STATE = StackArray::<f64, 6>::new(1..=6);
        let mut STOBS = StackArray::<f64, 6>::new(1..=6);
        let mut TEMP = StackArray::<f64, 6>::new(1..=6);
        let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
        let mut CENTER: i32 = 0;
        let mut FJ2000: i32 = 0;
        let mut LTSIGN: i32 = 0;
        let mut REQFRM: i32 = 0;
        let mut TYPE: i32 = 0;
        let mut TYPEID: i32 = 0;
        let mut ATTBLK = StackArray::<bool, 15>::new(1..=NABCOR);
        let mut FOUND: bool = false;
        let mut USEGEO: bool = false;
        let mut XMIT: bool = false;
        let mut SVCTR1 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVREF = vec![b' '; FRNMLN as usize];
        let mut SVREQF: i32 = 0;
        let mut FIRST: bool = false;

        FIRST = true;
        fstr::assign(&mut PRVCOR, b" ");

        Self {
            PRVCOR,
            DLT,
            DLTCTR,
            LTCENT,
            STATE,
            STOBS,
            TEMP,
            XFORM,
            CENTER,
            FJ2000,
            LTSIGN,
            REQFRM,
            TYPE,
            TYPEID,
            ATTBLK,
            FOUND,
            USEGEO,
            XMIT,
            SVCTR1,
            SVREF,
            SVREQF,
            FIRST,
        }
    }
}

//$Procedure ZZSPKEZ1 ( S/P Kernel, easy reader )
pub fn ZZSPKEZ1(
    TARG: i32,
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
    // Saved frame name length.
    //

    //
    // Local variables
    //

    //
    // Saved frame name/ID item declarations.
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

    //
    // Counter initialization is done separately.
    //
    if save.FIRST {
        //
        // Initialize counter.
        //
        ZZCTRUIN(save.SVCTR1.as_slice_mut(), ctx);
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
        ZZSPKGO1(TARG, ET, REF, OBS, STARG.as_slice_mut(), LT, ctx)?;
    } else {
        //
        // Get the auxiliary information about the requested output
        // frame.
        //
        ZZNAMFRM(
            save.SVCTR1.as_slice_mut(),
            &mut save.SVREF,
            &mut save.SVREQF,
            REF,
            &mut save.REQFRM,
            ctx,
        )?;

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

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        if !save.FOUND {
            SETMSG(b"The requested output frame \'#\' is not recognized by the reference frame subsystem. Please check that the appropriate kernels have been loaded and that you have correctly entered the name of the output frame. ", ctx);
            ERRCH(b"#", REF, ctx);
            SIGERR(b"SPICE(UNKNOWNFRAME2)", ctx)?;
            CHKOUT(RNAME, ctx)?;

            return Ok(());
        }

        //
        // If we are dealing with an inertial frame, we can simply
        // call SPKACS and return.
        //
        if (save.TYPE == INERTL) {
            ZZSPKAC1(
                TARG,
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
        ZZSPKAC1(
            TARG,
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

        if (save.CENTER == OBS) {
            save.LTCENT = 0.0;
            save.DLTCTR = 0.0;
        } else if (save.CENTER == TARG) {
            save.LTCENT = *LT;
            save.DLTCTR = save.DLT;
        } else {
            ZZSPKSB1(OBS, ET, b"J2000", save.STOBS.as_slice_mut(), ctx)?;
            ZZSPKLT1(
                save.CENTER,
                ET,
                b"J2000",
                ABCORR,
                save.STOBS.as_slice(),
                save.TEMP.as_slice_mut(),
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
        ZZFRMCH1(
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
        // Let P and V be the target position and velocity respectively,
        // and R, DR be the rotation and rotation derivative
        // corresponding to XFORM.
        //
        // The state transformation we need to perform is not
        //
        //    R * V   +  DR * P
        //
        // but rather
        //
        //    R * V   +  ( (1 + LTSIGN*DLTCTR) * DR )  * P
        //
        // So we'll scale the derivative block of XFORM accordingly.
        //
        for I in 1..=3 {
            VSCLIP(
                (1.0 + ((save.LTSIGN as f64) * save.DLTCTR)),
                save.XFORM.subarray_mut([4, I]),
            );
        }

        //
        // Now apply the frame transformation XFORM to produce the
        // state expressed relative to the request frame REQFRM.
        //
        MXVG(
            save.XFORM.as_slice(),
            save.STATE.as_slice(),
            6,
            6,
            STARG.as_slice_mut(),
        );
    }

    CHKOUT(RNAME, ctx)?;
    Ok(())
}
