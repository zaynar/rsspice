//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Solve Kepler's Equation --- Equinoctial Form
///
/// Solve the equinoctial version of Kepler's equation.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ML         I   Mean longitude.
///  H          I   h component of equinoctial elements.
///  K          I   k component of equinoctial elements.
///
///  The function returns the solution to the equinoctial version of
///  Kepler's equation, given the mean longitude and the h and k
///  components of the equinoctial elements.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ML       is the mean longitude of some body following two body
///           motion. (Mean longitude = Mean anomaly + argument of
///           periapse + longitude of ascending node.)
///
///  H        is the h component of the equinoctial element set
///           ( h = ECC*SIN( arg of periapse + long ascending node) )
///
///  K        is the k component of the equinoctial element set
///           ( k = ECC*COS( arg of periapse + long ascending node) )
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the solution to the equinoctial version of
///  Kepler's equation, given the mean longitude and the h and k
///  components of the equinoctial elements.
///
///  The solution is the value of F such that
///
///     ML = F + H * COS(F) - K * SIN(F)
///
///  Note that ECC = DSQRT ( K*K + H*H )
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the sum of the squares of H and K is not less than .9,
///      the error SPICE(ECCOUTOFBOUNDS) is signaled.
///
///  2)  If the iteration for a solution to the equinoctial Kepler's
///      equation does not converge in 10 or fewer steps, the error
///      SPICE(NOCONVERGENCE) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine solves the equinoctial element version of
///  Kepler's equation.
///
///     ML = F + H * COS(F) - K * SIN(F)
///
///  Here F is an offset from the eccentric anomaly E.
///
///     F = E - argument of periapse - longitude of ascending node.
///
///  where E is eccentric anomaly.
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  W. Owen and R. Vaughan, "Optical Navigation Program
///       Mathematical Models," JPL Engineering Memorandum 314-513,
///       August 9, 1991.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 26-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Updated
///         $Procedure section for consistency with KPSOLV routine.
///
/// -    SPICELIB Version 1.0.0, 11-DEC-1996 (WLT)
/// ```
pub fn kepleq(ctx: &mut SpiceContext, ml: f64, h: f64, k: f64) -> crate::Result<f64> {
    let ret = KEPLEQ(ml, h, k, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure KEPLEQ ( Solve Kepler's Equation --- Equinoctial Form )
pub fn KEPLEQ(ML: f64, H: f64, K: f64, ctx: &mut Context) -> f2rust_std::Result<f64> {
    let mut KEPLEQ: f64 = 0.0;
    let mut EVEC = StackArray::<f64, 2>::new(1..=2);
    let mut E2: f64 = 0.0;

    //
    // SPICELIB Functions
    //

    //
    // Local variables
    //

    //
    // Make sure that H and K are in the expected range.
    //
    E2 = ((H * H) + (K * K));

    if (E2 >= 0.81) {
        KEPLEQ = 0.0;
        CHKIN(b"KEPLEQ", ctx)?;
        SETMSG(b"The values of H and K supplied to KEPLEQ must satisfy the inequality H*H + K*K < ECC**2 where ECC is the eccentricity threshold of 0.9.  The values of H and K are: # and # respectively. H*H + K*K = #. ", ctx);

        ERRDP(b"#", H, ctx);
        ERRDP(b"#", K, ctx);
        ERRDP(b"#", E2, ctx);
        SIGERR(b"SPICE(ECCOUTOFBOUNDS)", ctx)?;
        CHKOUT(b"KEPLEQ", ctx)?;
        return Ok(KEPLEQ);
    }

    //
    // Instead of solving the equation
    //
    //        ML  = F + H*DCOS(F) - K*DSIN(F)
    //
    // We set X equal to F - ML and solve the equivalent equation
    //
    //        0   = X + H*DCOS(ML+X) - K*DSIN(ML+X)
    //
    //            = X + H*{DCOS(ML)*DCOS(X) - DSIN(ML)*DSIN(X)}
    //                - K*{DSIN(ML)*DCOS(X) + DCOS(ML)*DSIN(X)}
    //
    //            = X + { H*DCOS(ML) - K*DSIN(ML) }*DCOS(X)
    //                - { H*DSIN(ML) + K*DCOS(ML) }*DSIN(X)
    //
    //
    // We can rearrange this to:
    //
    //                             -                    -     -       -
    //                            |  DCOS(ML)  -DSIN(ML) |   | DCOS(X) |
    //        0 = X + [ H  -K ] * |  DSIN(ML)   DCOS(ML) | * | DSIN(X) |
    //                             -                    -     -       -
    //
    // Finally if we let
    //
    //                                    -                    -
    //                                   |  DCOS(ML)  -DSIN(ML) |
    //  EVEC =  [ EX  EY ] = [ -H  K ] * |  DSIN(ML)   DCOS(ML) |
    //                                    -                    -
    //
    // and
    //
    //          DCOS(X)
    //  U(X) =  DSIN(X)
    //
    // Then we can rewrite the equation as:
    //
    //    0  =  X - < EVEC, U(X) >
    //
    // where <,> denotes the dot product operation.  Note that X
    // is necessarily in the range from -ECC to ECC where ECC = | EVEC |
    //
    // Once we've computed X, F is just ML + X.
    //
    // For those of you who are fans of the classical keplerian
    // elements:
    //
    //    x = F - ML = E - M
    //
    // where E denotes eccentric anomaly and M denotes mean anomaly.
    //
    // The routine KPEVEC returns the value of X that solves
    // the equation X - < EVEC, UVEC(X) >
    //

    EVEC[1] = (-(H * f64::cos(ML)) + (K * f64::sin(ML)));
    EVEC[2] = ((H * f64::sin(ML)) + (K * f64::cos(ML)));
    KEPLEQ = (ML + KPSOLV(EVEC.as_slice(), ctx)?);

    Ok(KEPLEQ)
}
