//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

struct SaveVars {
    MAXLOG: f64,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut MAXLOG: f64 = 0.0;
        let mut FIRST: bool = false;

        FIRST = true;

        Self { MAXLOG, FIRST }
    }
}

/// Hyperbolic time of flight
///
/// Solve the time of flight equation MA = e sinh(F) - F for the
/// hyperbolic eccentric anomaly F, given the mean anomaly, MA,
/// and the eccentricity, e.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  MA         I   Mean anomaly at epoch.
///  ECC        I   Eccentricity.
///  F          O   Hyperbolic eccentric anomaly.
/// ```
///
/// # Detailed Input
///
/// ```text
///  MA       is the hyperbolic mean anomaly of an orbiting body at
///           some epoch t,
///
///                                3 1/2
///              MA = (t-T)(mu/(-a) )
///
///           where T is the time of periapsis passage, a is
///           the semi-major axis of the orbit, and mu is the
///           gravitational parameter of the primary body.
///
///  ECC      is the eccentricity of the orbit.
/// ```
///
/// # Detailed Output
///
/// ```text
///  F        is the corresponding eccentric anomaly. This is the
///           solution to the time of flight equation
///
///              MA = e sinh(F) - F
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the eccentricity (ECC) is less than one, the error
///      SPICE(WRONGCONIC) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  Iterate to solve
///
///     f(F,MA,e) = e sinh(F) - F - MA = 0
/// ```
///
/// # Examples
///
/// ```text
///  ELLTOF, HYPTOF, and PARTOF are used by CONICS.
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  R. Bate, D. Mueller, and J. White, "Fundamentals of
///       Astrodynamics," Dover Publications Inc., 1971.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.2.0, 14-APR-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 3.1.0, 13-JUL-2007 (NJB)
///
///         Bug fix: MAXLOG is now saved.
///
/// -    SPICELIB Version 3.0.0, 14-DEC-1994 (WLT)
///
///         A counter was placed in the loop which bisects to a
///         solution to the hyperbolic version of Kepler's equation.
///         This addition forces the loop to terminate. On some platforms
///         the loop would not terminate without this additional
///         check. This was due to the compiler performing tests on
///         extended precision registers.
///
/// -    SPICELIB Version 2.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 2.0.0, 19-APR-1990 (WLT)
///
///         A bad initial guess at bracketing the solution to the
///         hyperbolic time of flight equation was corrected so that
///         floating point overflows are now avoided. In addition, the
///         Newton's method used before has been replaced by simply
///         bisection.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    Beta Version 1.1.0, 8-JAN-1989 (IMU)
///
///         The routine now verifies that the eccentricity is in the
///         proper range---(1,+infinity)---before proceeding.
/// ```
pub fn hyptof(ctx: &mut SpiceContext, ma: f64, ecc: f64, f: &mut f64) -> crate::Result<()> {
    HYPTOF(ma, ecc, f, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure HYPTOF ( Hyperbolic time of flight )
pub fn HYPTOF(MA: f64, ECC: f64, F: &mut f64, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut COUNT: i32 = 0;
    let mut MCOUNT: i32 = 0;
    let mut M: f64 = 0.0;
    let mut LOWER: f64 = 0.0;
    let mut MIDDLE: f64 = 0.0;
    let mut MIDVAL: f64 = 0.0;
    let mut UPPER: f64 = 0.0;
    let mut DIFF: f64 = 0.0;
    let mut LASTDF: f64 = 0.0;

    //
    // SPICELIB functions
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
        CHKIN(b"HYPTOF", ctx)?;
    }

    if save.FIRST {
        save.FIRST = false;
        save.MAXLOG = f64::ln(DPMAX());
    }

    if (ECC < 1.0) {
        SIGERR(b"SPICE(WRONGCONIC)", ctx)?;
        CHKOUT(b"HYPTOF", ctx)?;
        return Ok(());
    }

    //
    // For reasons of numerical stability, we have to intercept cases
    // where the mean anomaly is zero or negative (since log x is not
    // defined for non-positive x). If the mean anomaly is zero, the
    // eccentric anomaly is also zero (by inspection).
    //
    // Since the function e sinh(F) - F is an odd function, we can
    // solve the equation ABS(MA) = e sinh(F) - F for F and get
    // the solution to MA = e sinh(F) - F by negating F if MA is
    // less than 0.
    //
    if (MA == 0.0) {
        *F = 0.0;
        CHKOUT(b"HYPTOF", ctx)?;
        return Ok(());
    } else {
        M = f64::abs(MA);
    }

    //
    //  The initial bounds for the eccentric anomaly F are determined
    //  as follows:
    //
    //  For the value of F we seek,
    //
    //     M = e sinh F - F
    //
    //  Thus
    //
    //     M < e sinh F =    (e/2) { Exp(F) - Exp(-F)}
    //
    //  Hence
    //
    //     2 M                1
    //     ---   < Exp(F) - -----
    //      e               Exp(F)
    //
    //  which yields
    //
    //
    //     2 M Exp(F)
    //     ----------   < Exp(F)**2 - 1
    //         e
    //
    //  and
    //
    //        M**2                2M Exp(F)     M**2
    //    1 + ---- <  Exp(F)**2 - ---------  +  ---- = {Exp(F) - (M/e)}**2
    //        e**2                   e          e**2
    //
    //
    //  Therefore we must have one of the following be true.
    //
    //
    //   SQRT( 1 + (M/e)**2 )  <  Exp(F) - (M/e)
    //
    // or
    //
    //  - SQRT( 1 + (M/e)**2 )  >  Exp(F) - (M/e)
    //
    // The second case implies that
    //
    //  0 > (M/e) - SQRT( 1 + (M/e)**2 ) > Exp(F)
    //
    // but since Exp(F) > 0 for all F it must be the case that
    //
    //   (M/e) + SQRT( 1 + (M/e)**2 ) < Exp(F)
    //
    //
    // Hence
    //
    //    Log ( (M/e) + SQRT(1 + (M/e)**2) )  < F
    //
    //
    //
    //  Returning to our initial equation:
    //
    //     M = e sinh F - F
    //
    //                      3        5
    //                     F        F
    //       =  e ( F  +  ---   +  --- + ...   )  -  F
    //                     3!       5!
    //
    //            3
    //       >  eF / 6
    //
    //  Thus
    //
    //
    //          3 __________
    //     F <  \/  6M / e
    //
    //
    //  Thus our solution must satisfy the inequalities
    //
    //
    //                                                   3 __________
    //   LOG ( (M/e) + SQRT(1 + (M/e)**2) )  <   F   <   \/  6M/e
    //
    //
    //  In addition we know that the solution must lie somewhere
    //  in the region between 0 and the maximum value of F for which
    //  (e sinh F - F) can be computed.  This turns out to be
    //  approximately LOG( DPMAX() / e ) = LOG(DPMAX()) - LOG(e) .
    //
    //
    LOWER = f64::ln(((M / ECC) + f64::sqrt((1.0 + f64::powi((M / ECC), 2)))));
    UPPER = intrinsics::DMIN1(&[DCBRT(((6.0 * M) / ECC)), (save.MAXLOG - f64::ln(ECC))]);
    UPPER = intrinsics::DMAX1(&[LOWER, UPPER]);
    //
    // Perform some simple checks first to avoid problems with
    // convergence of the loop below.  If LOWER is zero, then
    // M/ECC is so small that when added to 1 it doesn't make
    // any difference ( dLOG/dt = 1 at 1 after all).  So in this
    // case we will just solve the linear portion of the
    // expansion of e SINH(F) - F = M
    //
    //
    // Now we simply perform bisection to locate the root.
    //
    MIDDLE = intrinsics::DMAX1(&[
        LOWER,
        intrinsics::DMIN1(&[UPPER, ((0.5 * UPPER) + (0.5 * LOWER))]),
    ]);
    MIDVAL = (((ECC * f64::sinh(MIDDLE)) - MIDDLE) - M);

    DIFF = (UPPER - LOWER);
    //
    // Finally pick a reasonable upper bound on the number of loop
    // iterations we shall need to perform.
    //
    MCOUNT = 100;
    COUNT = 0;
    while ((DIFF > 0.0) && (COUNT < MCOUNT)) {
        //
        // Move one of the endpoints to the middle.
        //
        if (MIDVAL > 0.0) {
            UPPER = MIDDLE;
        } else if (MIDVAL < 0.0) {
            LOWER = MIDDLE;
        } else {
            LOWER = MIDDLE;
            UPPER = MIDDLE;
        }

        //
        // Compute the next middle point.
        //
        MIDDLE = intrinsics::DMAX1(&[
            LOWER,
            intrinsics::DMIN1(&[UPPER, ((0.5 * UPPER) + (0.5 * LOWER))]),
        ]);
        LASTDF = DIFF;
        COUNT = (COUNT + 1);

        //
        // If we are on an endpoint, we are ready to call it quits.
        //
        if ((MIDDLE == LOWER) || (MIDDLE == UPPER)) {
            DIFF = 0.0;
        } else {
            DIFF = (UPPER - LOWER);
            MIDVAL = (((ECC * f64::sinh(MIDDLE)) - MIDDLE) - M);
        }
    }

    //
    // Restore the proper sign, if necessary.
    //
    if (MA < 0 as f64) {
        *F = -MIDDLE;
    } else {
        *F = MIDDLE;
    }

    CHKOUT(b"HYPTOF", ctx)?;
    Ok(())
}
