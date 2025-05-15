//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
const MXLOOP: i32 = 1000;

//$Procedure ZZTANSLV ( Private --- tangent point solver )
pub fn ZZTANSLV(
    UDCOND: fn(f64, &mut bool, &mut [f64], &mut Context) -> f2rust_std::Result<()>,
    UDSTEP: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    UDREFN: fn(f64, f64, bool, bool, &mut f64) -> (),
    CSTEP: bool,
    STEP: f64,
    START: f64,
    FINISH: f64,
    TOL: f64,
    RESULT: &mut [f64],
    POINTS: &mut [f64],
    ENDFLG: &mut [bool],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut RESULT = DummyArrayMut::new(RESULT, LBCELL..);
    let mut POINTS = DummyArrayMut2D::new(POINTS, 1..=3, 1..);
    let mut ENDFLG = DummyArrayMut::new(ENDFLG, 1..=2);
    let mut CONTXT = [b' '; 256 as usize];
    let mut BEGIN: f64 = 0.0;
    let mut CURX: f64 = 0.0;
    let mut MAXMAG: f64 = 0.0;
    let mut PRVPNT = StackArray::<f64, 3>::new(1..=3);
    let mut SVDX: f64 = 0.0;
    let mut T: f64 = 0.0;
    let mut X1: f64 = 0.0;
    let mut X2: f64 = 0.0;
    let mut XSTEP: f64 = 0.0;
    let mut TRNSTN: f64 = 0.0;
    let mut XPOINT = StackArray::<f64, 3>::new(1..=3);
    let mut NLOOP: i32 = 0;
    let mut ROOM: i32 = 0;
    let mut TO: i32 = 0;
    let mut CURSTA: bool = false;
    let mut INSTAT: bool = false;
    let mut S: bool = false;
    let mut STATE1: bool = false;
    let mut STATE2: bool = false;
    let mut SAVSTA: bool = false;
    let mut PRVSET: bool = false;

    //
    // SPICELIB functions
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

    CHKIN(b"ZZTANSLV", ctx)?;

    //
    // Check the convergence tolerance.
    //
    if (TOL <= 0.0) {
        SETMSG(b"Tolerance must be positive but was #.", ctx);
        ERRDP(b"#", TOL, ctx);
        SIGERR(b"SPICE(INVALIDTOLERANCE)", ctx)?;
        CHKOUT(b"ZZTANSLV", ctx)?;
        return Ok(());
    }

    //
    // Make sure that START is not greater than FINISH. Signal an
    // error for START > FINISH.
    //
    if (START > FINISH) {
        SETMSG(b"Bad input interval: START = # > FINISH = #.", ctx);
        ERRDP(b"#", START, ctx);
        ERRDP(b"#", FINISH, ctx);
        SIGERR(b"SPICE(BOUNDSOUTOFORDER)", ctx)?;
        CHKOUT(b"ZZTANSLV", ctx)?;
        return Ok(());
    }

    //
    // Make sure that TOL is not too small, i.e. that neither
    // START + TOL nor START - TOL equals START.
    //
    if ((TOUCHD((START - TOL)) == START) || (TOUCHD((START + TOL)) == START)) {
        SETMSG(b"TOL has value #1. This value is too small to distinguish START - TOL or START + TOL from START, #2.", ctx);
        ERRDP(b"#1", TOL, ctx);
        ERRDP(b"#2", START, ctx);
        SIGERR(b"SPICE(INVALIDTOLERANCE)", ctx)?;
        CHKOUT(b"ZZTANSLV", ctx)?;
        return Ok(());
    }

    //
    // Make sure that TOL is not too small, i.e. that neither
    // FINISH + TOL nor FINISH - TOL equals FINISH.
    //
    if ((TOUCHD((FINISH - TOL)) == FINISH) || (TOUCHD((FINISH + TOL)) == FINISH)) {
        SETMSG(b"TOL has value #1. This value is too small to distinguish FINISH - TOL or FINISH + TOL from FINISH, #2.", ctx);
        ERRDP(b"#1", TOL, ctx);
        ERRDP(b"#2", FINISH, ctx);
        SIGERR(b"SPICE(INVALIDTOLERANCE)", ctx)?;
        CHKOUT(b"ZZTANSLV", ctx)?;
        return Ok(());
    }

    //
    // Make sure that STEP is not too small: it must be greater
    // than TOL.
    //
    if CSTEP {
        if (STEP <= 0.0) {
            SETMSG(b"STEP has value #1. The search step must be positive.", ctx);
            ERRDP(b"#1", STEP, ctx);
            SIGERR(b"SPICE(INVALIDCONSTSTEP)", ctx)?;
            CHKOUT(b"ZZTANSLV", ctx)?;
            return Ok(());
        }

        MAXMAG = intrinsics::DMAX1(&[f64::abs(START), f64::abs(FINISH)]);

        if (TOUCHD((MAXMAG + STEP)) == MAXMAG) {
            SETMSG(b"STEP has value #1. This value is too small to guarantee that the search will advance.", ctx);
            ERRDP(b"#1", STEP, ctx);
            SIGERR(b"SPICE(INVALIDCONSTSTEP)", ctx)?;
            CHKOUT(b"ZZTANSLV", ctx)?;
            return Ok(());
        }
    }

    //
    // This algorithm determines those intervals when a given state is
    // observed to occur within a specified search interval.
    //
    // Pairs of X values are recorded. The first member of each pair
    // denotes the X value at which the system changes to the state of
    // interest. The second denotes a transition out of that state.
    //
    // If the state is .TRUE. at the beginning of the interval, the
    // beginning of the X interval will be recorded. This may or may not
    // be a transition point.
    //
    // Similarly if the state is .TRUE. at the end of the interval, the
    // end of the interval will be recorded. Again, this may or may not
    // be a transition point.
    //
    // Initially the current X value is the beginning of the search
    // interval.
    //
    CURX = START;

    TO = 1;
    ROOM = SIZED(RESULT.as_slice(), ctx)?;
    PRVSET = false;

    //
    // Determine if the state at the current X value satisfies the
    // constraint.
    //
    UDCOND(CURX, &mut CURSTA, XPOINT.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZTANSLV", ctx)?;
        return Ok(());
    }

    if CURSTA {
        VEQU(XPOINT.as_slice(), PRVPNT.as_slice_mut());
        PRVSET = true;
    }

    //
    // If the system is in the state of interest, record the initial
    // X value of the search interval. The variable BEGIN will be
    // used to store the starting point of an interval over which
    // the state is .TRUE.
    //
    if CURSTA {
        INSTAT = true;
        BEGIN = CURX;
        ENDFLG[1] = false;

        //
        // BEGIN will be the first element of RESULT, presuming
        // a state transition is found later. We'll shift the
        // pointer for the output point so the Ith point will
        // correspond to the Ith element of RESULT.
        //
        // We don't have to check ROOM yet because we're not
        // inserting anything into POINTS.
        //
        TO = (TO + 1);
        ROOM = (ROOM - 1);
    } else {
        INSTAT = false;
        ENDFLG[1] = true;
    }

    //
    // If the step size is constant, use the value supplied.
    //
    if CSTEP {
        XSTEP = STEP;
    }

    //
    // Save the current X value and state.
    //
    SVDX = CURX;
    SAVSTA = CURSTA;

    //
    // Once initializations have been performed keep working
    // until the search interval has been exhausted.
    //
    // While the last X value precedes the end of the interval:
    //
    while (SVDX < FINISH) {
        //
        // Attempt to bracket a state change.
        //
        // Using the current window and internally stored information
        // about the current state, select a new current X.
        //
        if !CSTEP {
            UDSTEP(&mut CURX, &mut XSTEP, ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"ZZTANSLV", ctx)?;
                return Ok(());
            }
        }

        //
        // Add the X step to the current X.  Make sure that the
        // X does not move beyond the end of the search interval.
        //
        CURX = intrinsics::DMIN1(&[TOUCHD((CURX + XSTEP)), FINISH]);

        //
        // Compute the state at CURX.
        //
        UDCOND(CURX, &mut CURSTA, XPOINT.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZTANSLV", ctx)?;
            return Ok(());
        }

        if CURSTA {
            VEQU(XPOINT.as_slice(), PRVPNT.as_slice_mut());
            PRVSET = true;
        }

        //
        // While the state remains unchanged and the interval has not
        // been completely searched ...
        //
        while ((SAVSTA == CURSTA) && (SVDX < FINISH)) {
            //
            // Save the current X and state.
            //
            SVDX = CURX;
            SAVSTA = CURSTA;

            //
            // Compute a new current X so that we will not step
            // past the end of the interval.
            //
            if !CSTEP {
                UDSTEP(&mut CURX, &mut XSTEP, ctx)?;

                if FAILED(ctx) {
                    CHKOUT(b"ZZTANSLV", ctx)?;
                    return Ok(());
                }
            }

            CURX = intrinsics::DMIN1(&[TOUCHD((CURX + XSTEP)), FINISH]);
            //
            // Compute the current state.
            //
            UDCOND(CURX, &mut CURSTA, XPOINT.as_slice_mut(), ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"ZZTANSLV", ctx)?;
                return Ok(());
            }

            if CURSTA {
                //
                // Save the associated vector for the X value CURX. In
                // normal usage, XPOINT is a surface intercept point.
                //
                VEQU(XPOINT.as_slice(), PRVPNT.as_slice_mut());
                PRVSET = true;
            }
            //
            // Loop back to see if the state has changed.
            //
        }
        //
        // At this point, SVDX and CURX are the X-values at the previous
        // and latest steps, respectively. SAVSTA and CURSTA are the
        // states at these X-values, respectively.
        //
        // If we have detected a state change and not merely run out
        // of the search interval...
        //
        if (SAVSTA != CURSTA) {
            //
            // Call the previous state STATE1.
            // Call the current state STATE2.
            //
            // Let X1 be the X value at state STATE1.
            // Let X2 be the X value at state STATE2.
            //
            // Save the current X.
            //
            STATE1 = SAVSTA;
            STATE2 = CURSTA;
            X1 = SVDX;
            X2 = CURX;

            //
            // Make sure that X1 is not greater than X2. Signal an
            // error for X1 > X2.
            //
            if (X1 > X2) {
                SETMSG(b"Bad x interval result: X1 = # > X2 = #.", ctx);
                ERRDP(b"#", X1, ctx);
                ERRDP(b"#", X2, ctx);
                SIGERR(b"SPICE(INVALIDSTEP)", ctx)?;
                CHKOUT(b"ZZTANSLV", ctx)?;
                return Ok(());
            }

            //
            // Update the saved X and state values to those on the
            // right side of the bracketing interval. We'll use these
            // values for the next bracketing step after a root is
            // found.
            //
            SVDX = CURX;
            SAVSTA = CURSTA;

            //
            // X1 and X2 bracket the X value of transition. Squeeze this
            // interval down until it is less than some tolerance in
            // length. Do it as described below...
            //
            // Loop while the difference between the X values X1 and X2
            // exceeds a specified tolerance.
            //
            NLOOP = 0;

            while (TOUCHD((X2 - X1)) > TOL) {
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
                    SIGERR(b"SPICE(NOCONVERGENCE)", ctx)?;
                    CHKOUT(b"ZZTANSLV", ctx)?;
                    return Ok(());
                }

                //
                // Select an X value T, between X1 and X2 (possibly based
                // on the state values).
                //
                UDREFN(X1, X2, STATE1, STATE2, &mut T);

                //
                // Check for an error signal. The default refinement
                // routine, GFREFN, does not include error checks.
                //
                if FAILED(ctx) {
                    CHKOUT(b"ZZTANSLV", ctx)?;
                    return Ok(());
                }

                //
                // Check whether T is between X1 and X2.  If
                // not then assume that we have gone as far as
                // we can in refining our estimate of the transition
                // point. Set X1 and X2 equal to T.
                //

                T = BRCKTD(T, X1, X2);

                if (T == X1) {
                    //
                    // This assignment may break the invariant that
                    // the state at X2 is STATE2. This is allowed
                    // because we'll exit the loop immediately.
                    //
                    X2 = T;
                } else if (T == X2) {
                    //
                    // This assignment may break the invariant that
                    // the state at X1 is STATE1. This is allowed
                    // because we'll exit the loop immediately.

                    X1 = T;
                } else {
                    //
                    // Compute the state at X value T. If this state, S,
                    // equals STATE1, set X1 to T, otherwise set X2 to T.
                    //
                    UDCOND(T, &mut S, XPOINT.as_slice_mut(), ctx)?;

                    if S {
                        //
                        // Save the latest point associated with a
                        // .TRUE. state.
                        //
                        VEQU(XPOINT.as_slice(), PRVPNT.as_slice_mut());
                        PRVSET = true;
                    }

                    //
                    // Narrow the interval. Either increase X1 or decrease
                    // X2 by setting one of these endpoints to T. Maintain
                    // the invariant that the state is STATE1 at X1 and
                    // STATE2 at X2.
                    //
                    if (S == STATE1) {
                        X1 = T;
                    } else {
                        X2 = T;
                    }
                }
            }

            //
            // Let TRNSTN be the midpoint of [X1, X2]. Record this
            // abscissa value as marking the transition from STATE1 to
            // STATE2.
            //
            TRNSTN = BRCKTD(((X1 + X2) * 0.5), X1, X2);

            //
            // In state-of-interest or not? INSTAT indicates that STATE1
            // was .TRUE. We record intervals where the state is .TRUE.
            // when we detect the right hand endpoints of these intervals.
            //
            if INSTAT {
                //
                // We were in the state of interest. TRNSTN marks the point
                // on the X-axis when the state changed to .FALSE. We need
                // to record the interval from BEGIN to FINISH and note
                // that the state has become .FALSE.
                //
                // Add an interval starting at BEGIN and ending at TRNSTN
                // to the result window.
                //
                fstr::assign(&mut CONTXT, b"Adding interval [BEGIN,TRNSTN] to RESULT. TRNSTN represents time of passage out of the state-of-interest.");

                ZZWNINSD(BEGIN, TRNSTN, &CONTXT, RESULT.as_slice_mut(), ctx)?;

                if FAILED(ctx) {
                    CHKOUT(b"ZZTANSLV", ctx)?;
                    return Ok(());
                }
            } else {
                //
                // The previous state was .FALSE. As a result TRNSTN marks
                // the point where the state becomes .TRUE. Note that we
                // have transitioned to the state of interest and record
                // the X-value at which the transition occurred.
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
            // For all state transitions, record the last point found
            // by the state function.
            //
            if (ROOM > 0) {
                //
                // Add the last point found during the transition search to
                // the POINTS array.
                //
                if PRVSET {
                    VEQU(PRVPNT.as_slice(), POINTS.subarray_mut([1, TO]));

                    TO = (TO + 1);
                    ROOM = (ROOM - 1);
                    PRVSET = false;
                } else {
                    SETMSG(b"PRVPNT should always be set when a transition is detected. We found a transition at #, but PRVSET indicates we don\'t have a previous point saved.", ctx);
                    ERRDP(b"#", TRNSTN, ctx);
                    SIGERR(b"SPICE(BUG)", ctx)?;
                    CHKOUT(b"ZZTANSLV", ctx)?;
                    return Ok(());
                }
            } else {
                //
                // We ran out of room in the output point array. Note that
                // this error can occur before the result window insertion
                // fails, since that insertion takes place when the state
                // becomes .FALSE.
                //
                SETMSG(b"Out of room in the POINTS array. Room is assumed to be adequate for SIZED(RESULT) 3-vectors; this size is #.", ctx);
                ERRINT(b"#", SIZED(RESULT.as_slice(), ctx)?, ctx);
                SIGERR(b"SPICE(ARRAYTOOSMALL)", ctx)?;
                CHKOUT(b"ZZTANSLV", ctx)?;
                return Ok(());
            }
            //
            // That's it for this detection of state change.
            //
        }
        //
        // Continue if the search interval extends to the right
        // of the latest step.
        //
        // SVDX and SAVSTA are already set to the values at the
        // right side of the bracketing interval.
        //
    }

    //
    // Check if in-state at this abscissa value (FINISH). INSTAT is the
    // latest state value. If so record the interval.
    //
    if INSTAT {
        //
        // The state is .TRUE. at FINISH.
        //
        // Add an interval starting at BEGIN and ending at FINISH to the
        // window.
        //
        fstr::assign(&mut CONTXT, b"Adding interval [BEGIN,FINISH] to RESULT. FINISH represents end of the search interval.");

        ZZWNINSD(BEGIN, FINISH, &CONTXT, RESULT.as_slice_mut(), ctx)?;

        ENDFLG[2] = false;
    } else {
        ENDFLG[2] = true;
    }

    CHKOUT(b"ZZTANSLV", ctx)?;
    Ok(())
}
