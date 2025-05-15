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
const RNAME: &[u8] = b"ZZSPKLT0";
const TOL: f64 = 0.0000000001;
const CNVLIM: f64 = 0.00000000000000001;
const MAXITR: i32 = 5;
const SSB: i32 = 0;

struct SaveVars {
    PRVCOR: Vec<u8>,
    PASS1: bool,
    USECN: bool,
    USELT: bool,
    XMIT: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PRVCOR = vec![b' '; CORLEN as usize];
        let mut PASS1: bool = false;
        let mut USECN: bool = false;
        let mut USELT: bool = false;
        let mut XMIT: bool = false;

        PASS1 = true;
        fstr::assign(&mut PRVCOR, b" ");

        Self {
            PRVCOR,
            PASS1,
            USECN,
            USELT,
            XMIT,
        }
    }
}

//$Procedure ZZSPKLT0 ( S/P Kernel, light time corrected state )
pub fn ZZSPKLT0(
    TARG: i32,
    ET: f64,
    REF: &[u8],
    ABCORR: &[u8],
    STOBS: &[f64],
    STARG: &mut [f64],
    LT: &mut f64,
    DLT: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let STOBS = DummyArray::new(STOBS, 1..=6);
    let mut STARG = DummyArrayMut::new(STARG, 1..=6);
    let mut A: f64 = 0.0;
    let mut B: f64 = 0.0;
    let mut C: f64 = 0.0;
    let mut DIST: f64 = 0.0;
    let mut EPOCH: f64 = 0.0;
    let mut LTERR: f64 = 0.0;
    let mut PRVLT: f64 = 0.0;
    let mut SSBLT: f64 = 0.0;
    let mut SSBTRG = StackArray::<f64, 6>::new(1..=6);
    let mut I: i32 = 0;
    let mut LTSIGN: i32 = 0;
    let mut NUMITR: i32 = 0;
    let mut REFID: i32 = 0;
    let mut ATTBLK = StackArray::<bool, 15>::new(1..=NABCOR);
    let mut USESTL: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // TOL is the tolerance used for a division-by-zero test
    // performed prior to computation of DLT.
    //

    //
    // Convergence limit:
    //

    //
    // Maximum number of light time iterations for any
    // aberration correction:
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

    if (save.PASS1 || fstr::ne(ABCORR, &save.PRVCOR)) {
        //
        // The aberration correction flag differs from the value it
        // had on the previous call, if any.  Analyze the new flag.
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
        //    USELT is .TRUE. when any type of light time correction
        //    (normal or converged Newtonian) is specified.
        //
        //    USECN indicates converged Newtonian light time correction.
        //
        // The above definitions are consistent with those used by
        // ZZVALCOR.
        //
        save.XMIT = ATTBLK[XMTIDX];
        save.USELT = ATTBLK[LTIDX];
        save.USECN = ATTBLK[CNVIDX];
        USESTL = ATTBLK[STLIDX];

        save.PASS1 = false;
    }

    //
    // See if the reference frame is a recognized inertial frame.
    //
    IRFNUM(REF, &mut REFID, ctx);

    if (REFID == 0) {
        SETMSG(
            b"The requested frame \'#\' is not a recognized inertial frame. ",
            ctx,
        );
        ERRCH(b"#", REF, ctx);
        SIGERR(b"SPICE(BADFRAME)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Find the geometric state of the target body with respect to
    // the solar system barycenter. Subtract the state of the
    // observer to get the relative state. Use this to compute the
    // one-way light time.
    //
    ZZSPKGO0(TARG, ET, REF, SSB, SSBTRG.as_slice_mut(), &mut SSBLT, ctx)?;

    if FAILED(ctx) {
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    VSUBG(SSBTRG.as_slice(), STOBS.as_slice(), 6, STARG.as_slice_mut());

    DIST = VNORM(STARG.as_slice());
    *LT = (DIST / CLIGHT());

    if (*LT == 0.0) {
        //
        // This can happen only if the observer and target are at the
        // same position. We don't consider this an error, but we're not
        // going to compute the light time derivative.
        //
        *DLT = 0 as f64;

        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    if !save.USELT {
        //
        // This is a special case: we're not using light time
        // corrections, so the derivative
        // of light time is just
        //
        //    (1/c) * d(VNORM(STARG))/dt
        //
        *DLT = (VDOT(STARG.as_slice(), STARG.subarray(4)) / (DIST * CLIGHT()));

        //
        // LT and DLT are both set, so we can return.
        //
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // To correct for light time, find the state of the target body
    // at the current epoch minus the one-way light time. Note that
    // the observer remains where it is.
    //
    // Determine the sign of the light time offset.
    //
    if save.XMIT {
        LTSIGN = 1;
    } else {
        LTSIGN = -1;
    }

    //
    // Let NUMITR be the number of iterations we'll perform to
    // compute the light time.
    //
    if save.USECN {
        NUMITR = MAXITR;
    } else {
        NUMITR = 1;
    }

    I = 0;
    LTERR = 1.0;

    while ((I < NUMITR) && (LTERR > CNVLIM)) {
        //
        // LT was set either prior to this loop or
        // during the previous loop iteration.
        //
        EPOCH = (ET + ((LTSIGN as f64) * *LT));

        ZZSPKGO0(
            TARG,
            EPOCH,
            REF,
            SSB,
            SSBTRG.as_slice_mut(),
            &mut SSBLT,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        VSUBG(SSBTRG.as_slice(), STOBS.as_slice(), 6, STARG.as_slice_mut());

        PRVLT = *LT;
        *LT = TOUCHD((VNORM(STARG.as_slice()) / CLIGHT()));

        // LTERR is the magnitude of the change between the current
        // estimate of light time and the previous estimate, relative to
        // the previous light time corrected epoch.
        //
        LTERR = TOUCHD((f64::abs((*LT - PRVLT)) / intrinsics::DMAX1(&[1.0, f64::abs(EPOCH)])));
        I = (I + 1);
    }

    //
    // At this point, STARG contains the light time corrected
    // state of the target relative to the observer.
    //
    // Compute the derivative of light time with respect
    // to time: dLT/dt.  Below we derive the formula for
    // this quantity for the reception case. Let
    //
    //    POBS be the position of the observer relative to the
    //    solar system barycenter.
    //
    //    VOBS be the velocity of the observer relative to the
    //    solar system barycenter.
    //
    //    PTARG be the position of the target relative to the
    //    solar system barycenter.
    //
    //    VTARG be the velocity of the target relative to the
    //    solar system barycenter.
    //
    //    S be the sign of the light time correction. S is
    //    negative for the reception case.
    //
    // The light-time corrected position of the target relative to
    // the observer at observation time ET, given the one-way
    // light time LT is:
    //
    //     PTARG(ET+S*LT) - POBS(ET)
    //
    // The light-time corrected velocity of the target relative to
    // the observer at observation time ET is
    //
    //     VTARG(ET+S*LT)*( 1 + S*d(LT)/d(ET) ) - VOBS(ET)
    //
    // We need to compute dLT/dt. Below, we use the facts that,
    // for a time-dependent vector X(t),
    //
    //      ||X||     = <X,X> ** (1/2)
    //
    //    d(||X||)/dt = (1/2)<X,X>**(-1/2) * 2 * <X,dX/dt>
    //
    //                = <X,X>**(-1/2) *  <X,dX/dt>
    //
    //                = <X,dX/dt> / ||X||
    //
    // Newtonian light time equation:
    //
    //    LT     =   (1/c) * || PTARG(ET+S*LT) - POBS(ET)||
    //
    // Differentiate both sides:
    //
    //    dLT/dt =   (1/c) * ( 1 / || PTARG(ET+S*LT) - POBS(ET) || )
    //
    //              * < PTARG(ET+S*LT) - POBS(ET),
    //                  VTARG(ET+S*LT)*(1+S*d(LT)/d(ET)) - VOBS(ET) >
    //
    //
    //           = (1/c) * ( 1 / || PTARG(ET+S*LT) - POBS(ET) || )
    //
    //             * (  < PTARG(ET+S*LT) - POBS(ET),
    //                    VTARG(ET+S*LT) - VOBS(ET) >
    //
    //               +  < PTARG(ET+S*LT) - POBS(ET),
    //                    VTARG(ET+S*LT)           > * (S*d(LT)/d(ET))  )
    //
    // Let
    //
    //    A =   (1/c) * ( 1 / || PTARG(ET+S*LT) - POBS(ET) || )
    //
    //    B =   < PTARG(ET+S*LT) - POBS(ET), VTARG(ET+S*LT) - VOBS(ET) >
    //
    //    C =   < PTARG(ET+S*LT) - POBS(ET), VTARG(ET+S*LT) >
    //
    // Then
    //
    //    d(LT)/d(ET) =  A * ( B  +  C * S*d(LT)/d(ET) )
    //
    // which implies
    //
    //    d(LT)/d(ET) =  A*B / ( 1 - S*C*A )
    //
    //
    //
    A = (1.0 / (CLIGHT() * VNORM(STARG.as_slice())));

    B = VDOT(STARG.as_slice(), STARG.subarray(4));

    C = VDOT(STARG.as_slice(), SSBTRG.subarray(4));

    //
    // For physically realistic target velocities, S*C*A cannot equal 1.
    // We'll check for this case anyway.
    //
    if ((((LTSIGN as f64) * C) * A) > (1.0 - TOL)) {
        SETMSG(b"Target range rate magnitude is approximately the speed of light. The light time derivative cannot be computed.", ctx);
        SIGERR(b"SPICE(DIVIDEBYZERO)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Compute DLT: the rate of change of light time.
    //
    *DLT = ((A * B) / (1.0 - (((LTSIGN as f64) * C) * A)));

    //
    // Overwrite the velocity portion of the output state
    // with the light-time corrected velocity.
    //
    VLCOM(
        (1.0 + ((LTSIGN as f64) * *DLT)),
        SSBTRG.subarray(4),
        -1.0,
        STOBS.subarray(4),
        STARG.subarray_mut(4),
    );

    CHKOUT(RNAME, ctx)?;
    Ok(())
}
