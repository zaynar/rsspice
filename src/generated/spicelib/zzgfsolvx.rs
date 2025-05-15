//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
const MSGLEN: i32 = 256;
const MXLOOP: i32 = 1000;

//$Procedure ZZGFSOLVX ( Private --- GF, event finding routine )
pub fn ZZGFSOLVX(
    UDFUNS: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    UDFUNB: fn(
        fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
        &mut f64,
        &mut bool,
        &mut Context,
    ) -> f2rust_std::Result<()>,
    UDSTEP: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    UDREFN: fn(f64, f64, bool, bool, &mut f64) -> (),
    BAIL: bool,
    UDBAIL: fn() -> bool,
    CSTEP: bool,
    STEP: f64,
    START: f64,
    FINISH: f64,
    TOL: f64,
    RPT: bool,
    UDREPU: fn(f64, f64, f64, &mut Context) -> f2rust_std::Result<()>,
    RESULT: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut RESULT = DummyArrayMut::new(RESULT, LBCELL..);
    let mut BEGIN: f64 = 0.0;
    let mut CURTIM: f64 = 0.0;
    let mut DIFF: f64 = 0.0;
    let mut PRVDIF: f64 = 0.0;
    let mut SVDTIM: f64 = 0.0;
    let mut T: f64 = 0.0;
    let mut T1: f64 = 0.0;
    let mut T2: f64 = 0.0;
    let mut TIMEST: f64 = 0.0;
    let mut TRNSTN: f64 = 0.0;
    let mut CURSTE: bool = false;
    let mut INSTAT: bool = false;
    let mut S: bool = false;
    let mut STATE1: bool = false;
    let mut SAVST: bool = false;
    let mut L1: bool = false;
    let mut L2: bool = false;
    let mut CONTXT = [b' '; MSGLEN as usize];
    let mut NLOOP: i32 = 0;

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
    // The maximum number of search loop iterations to execute.
    // The default refinement method is bisection, a very slow
    // method to convergence. Since 2**1000 ~ 10**301,
    // 1000 loop iterations represents enough effort to assume
    // either the search will not converge or that the refinement
    // function operates slower than would bisection, in which
    // case the user should use the default GFREFN function.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFSOLVX", ctx)?;

    //
    // Check the convergence tolerance.
    //
    if (TOL <= 0.0) {
        SETMSG(b"Tolerance must be positive but was #.", ctx);
        ERRDP(b"#", TOL, ctx);
        SIGERR(b"SPICE(INVALIDTOLERANCE)", ctx)?;
        CHKOUT(b"ZZGFSOLVX", ctx)?;
        return Ok(());
    }

    //
    // Make sure that START is not greater than FINISH. Signal an
    // error for START > FINISH.
    //
    if (START > FINISH) {
        SETMSG(b"Bad time interval result, START > FINISH.", ctx);
        SIGERR(b"SPICE(BADTIMECASE)", ctx)?;
        CHKOUT(b"ZZGFSOLVX", ctx)?;
        return Ok(());
    }

    //
    // If active, update the progress reporter.
    //
    if RPT {
        UDREPU(START, FINISH, START, ctx)?;
    }

    //
    // This algorithm determines those intervals when a given state
    // is observed to occur within a specified search interval.
    //
    // Pairs of times are recorded.  The first member of each pair
    // denotes the time when the system changes to the state of
    // interest.  The second denotes a transition out of that state.
    //
    // If the system is in the state of interest at the beginning of
    // the interval, the beginning of the time interval will be
    // recorded.  This may or may not be a transition point.
    //
    // Similarly if the system is in the state of interest at the end
    // of the interval, the end of the interval will be recorded.
    // Again, this may or may not be a transition point.
    //

    //
    // Initially the current time is the beginning of the search
    // interval.
    //
    CURTIM = START;

    //
    // Determine if the state at the current time satisfies some
    // constraint. This constraint may indicate only existence of
    // a state.
    //
    UDFUNB(UDFUNS, &mut CURTIM, &mut CURSTE, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFSOLVX", ctx)?;
        return Ok(());
    }

    //
    // If the system is in the state of interest, record the initial
    // time of the search interval.
    //
    if CURSTE {
        INSTAT = true;
        BEGIN = CURTIM;
    } else {
        INSTAT = false;
    }

    //
    // If the step size is constant, use the value supplied.
    //
    if CSTEP {
        TIMEST = STEP;
    }

    //
    // Save the current time and state somewhere.
    //
    SVDTIM = CURTIM;
    SAVST = CURSTE;

    //
    // Once initializations have been performed keep working
    // until the search interval has been exhausted.
    //
    // While time remains in the search interval.
    //
    while (SVDTIM < FINISH) {
        //
        // Using the current window and internally stored
        // information about the current state, select a new current
        // time.
        //
        if !CSTEP {
            UDSTEP(&mut CURTIM, &mut TIMEST, ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"ZZGFSOLVX", ctx)?;
                return Ok(());
            }
        }

        //
        // Add the time step to the current time.  Make sure that the
        // time does not move beyond the end of the search interval.
        //

        CURTIM = intrinsics::DMIN1(&[(CURTIM + TIMEST), FINISH]);

        //
        // Compute the state at time CURTIM.
        //
        UDFUNB(UDFUNS, &mut CURTIM, &mut CURSTE, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZGFSOLVX", ctx)?;
            return Ok(());
        }

        //
        // While the state remains unchanged and the interval is not
        // completely searched ...
        //

        while ((SAVST == CURSTE) && (SVDTIM < FINISH)) {
            //
            // First check for an interrupt signal if checking is enabled.
            //
            if BAIL {
                if UDBAIL() {
                    CHKOUT(b"ZZGFSOLVX", ctx)?;
                    return Ok(());
                }
            }

            //
            // Report the current time to the monitoring utility, if
            // appropriate.
            //
            if RPT {
                UDREPU(START, FINISH, SVDTIM, ctx)?;
            }

            //
            // Save the current time and state somewhere.
            //
            SVDTIM = CURTIM;
            SAVST = CURSTE;

            //
            // Compute a new current time so that we will not step
            // past the end of the interval.  This time will be
            // based on:
            //
            //       1. The kind of event we are looking for.
            //       2. The objects and observer class.
            //       3. Transition times already found.
            //       4. A minimum time step allowed.
            //
            if !CSTEP {
                UDSTEP(&mut CURTIM, &mut TIMEST, ctx)?;

                if FAILED(ctx) {
                    CHKOUT(b"ZZGFSOLVX", ctx)?;
                    return Ok(());
                }
            }

            CURTIM = intrinsics::DMIN1(&[(CURTIM + TIMEST), FINISH]);

            //
            // Compute the current state
            //
            UDFUNB(UDFUNS, &mut CURTIM, &mut CURSTE, ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"ZZGFSOLVX", ctx)?;
                return Ok(());
            }

            //
            // Loop back to see if the state has changed.
            //
        }

        //
        // If we have detected a state change and not merely run out
        // of the search interval...
        //

        if (SAVST != CURSTE) {
            //
            // Call the previous state STATE1
            // Call the current  state STATE2
            //
            // Call the time at state STATE1, T1
            // Call the time at state STATE2, T2
            //
            // Save the current time.
            //
            STATE1 = SAVST;
            T1 = SVDTIM;
            T2 = CURTIM;

            //
            // Set the states at T1 and T2 for use by the refinement
            // function, in case the caller has passed in a function
            // that uses them.
            //
            L1 = SAVST;
            L2 = CURSTE;

            //
            // Make sure that T1 is not greater than T2. Signal an
            // error for T1 > T2.
            //

            if (T1 > T2) {
                SETMSG(b"Bad time interval result, T1 > T2.", ctx);
                SIGERR(b"SPICE(BADTIMECASE)", ctx)?;
                CHKOUT(b"ZZGFSOLVX", ctx)?;
                return Ok(());
            }

            SVDTIM = CURTIM;
            SAVST = CURSTE;

            //
            // T1 and T2 bracket the time of transition.  Squeeze this
            // interval down until it is less than some tolerance in
            // length.  Do it as described below...
            //
            // Loop while the difference between the times T1 and T2
            // exceeds a specified tolerance, and while the magnitude
            // of the difference is decreasing from one loop iteration
            // to the next.
            //
            PRVDIF = DPMAX();
            DIFF = TOUCHD(f64::abs((T2 - T1)));
            NLOOP = 0;

            while ((DIFF > TOL) && (DIFF < PRVDIF)) {
                NLOOP = (NLOOP + 1);

                //
                // This loop count error exists to catch pathologies
                // in the refinement function. The default bisection
                // refinement will converge before 1000 iterations if
                // a convergence is numerically possible. Any other
                // refinement function should require fewer iterations
                // compared to bisection. If not, the user should
                // probably use bisection.
                //
                if (NLOOP >= MXLOOP) {
                    SETMSG(b"Loop run exceeds maximum loop count. Unable to converge to TOL value #1 within MXLOOP value #2 iterations.", ctx);
                    ERRDP(b"#1", TOL, ctx);
                    ERRINT(b"#2", MXLOOP, ctx);
                    SIGERR(b"SPICE(NOCONVERG)", ctx)?;
                    CHKOUT(b"ZZGFSOLVX", ctx)?;
                    return Ok(());
                }

                if BAIL {
                    if UDBAIL() {
                        CHKOUT(b"ZZGFSOLVX", ctx)?;
                        return Ok(());
                    }
                }

                //
                // Select a time T, between T1 and T2 (possibly based on the
                // values of L1 and L2).
                //
                UDREFN(T1, T2, L1, L2, &mut T);

                //
                // Check for an error signal. The default refinement
                // routine, GFREFN, does not include error checks.
                //
                if FAILED(ctx) {
                    CHKOUT(b"ZZGFSOLVX", ctx)?;
                    return Ok(());
                }

                //
                // Check whether T is between T1 and T2.  If
                // not then assume that we have gone as far as
                // we can in refining our estimate of the transition
                // point. Set T1 and T2 equal to T.
                //

                T = BRCKTD(T, T1, T2);

                if (T == T1) {
                    T2 = T;
                } else if (T == T2) {
                    T1 = T;
                } else {
                    //
                    // Compute the state time T. If this state, S,
                    // equals STATE1, set T1 to T, otherwise set
                    // T2 to T.
                    //
                    UDFUNB(UDFUNS, &mut T, &mut S, ctx)?;

                    if (S == STATE1) {
                        T1 = T;
                        L1 = S;
                    } else {
                        T2 = T;
                        L2 = S;
                    }
                }

                //
                // Update PRVDIF and DIFF for the next loop termination
                // test.
                //
                PRVDIF = DIFF;
                DIFF = TOUCHD(f64::abs((T2 - T1)));
            }

            //
            // Let TRNSTN be the midpoint of [T1, T2].  Record this
            // time as marking the transition from STATE1 to STATE2.
            //
            TRNSTN = BRCKTD(((T1 + T2) * 0.5), T1, T2);

            //
            // In state-of-interest or not?
            //
            if INSTAT {
                //
                // We were in the state of interest, TRNSTN marks the
                // point in time when the state changed to "not of
                // interest" We need to record the interval from BEGIN to
                // FINISH and note that we are no longer in the state of
                // interest.
                //

                //
                // Add an interval starting at BEGIN and ending at TRNSTN
                // to the result window.
                //

                fstr::assign(&mut CONTXT, b"Adding interval [BEGIN,TRNSTN] to RESULT. TRNSTN represents time of passage out of the state-of-interest.");

                ZZWNINSD(BEGIN, TRNSTN, &CONTXT, RESULT.as_slice_mut(), ctx)?;
            } else {
                //
                // We were not in the state of interest.  As a result
                // TRNSTN marks the point where we are changing to
                // the state of interest.  Note that we have transitioned
                // to the state of interest and record the time at
                // which the transition occurred.
                //
                BEGIN = TRNSTN;
            }

            //
            // A transition occurred either from from in-state to
            // out-of-state or the inverse. Reverse the value of the
            // INSTAT flag to signify the transition event.
            //
            INSTAT = !INSTAT;

            //
            // That's it for this detection of state change.
            //
        }

        //
        // Continue if there is more time in the search interval.
        //
    }

    //
    // Check if in-state at this time (FINISH). If so record the
    // interval.
    //

    if INSTAT {
        //
        // Add an interval starting at BEGIN and ending at FINISH to the
        // window.
        //
        fstr::assign(&mut CONTXT, b"Adding interval [BEGIN,FINISH] to RESULT. FINISH represents end of the search interval.");

        ZZWNINSD(BEGIN, FINISH, &CONTXT, RESULT.as_slice_mut(), ctx)?;
    }

    //
    // If active, update the progress reporter before exiting this
    // routine.
    //

    if RPT {
        UDREPU(START, FINISH, FINISH, ctx)?;
    }

    //
    // Check-out then return.
    //
    CHKOUT(b"ZZGFSOLVX", ctx)?;
    Ok(())
}
