//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const TOL: f64 = 0.000000000000001;

/// Elliptic time of flight
///
/// Solve the time of flight equation MA = E - e sin(E) for the
/// elliptic eccentric anomaly E, given mean anomaly the MA and
/// the eccentricity ECC.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  MA         I   Mean anomaly at epoch.
///  ECC        I   Eccentricity.
///  E          O   Elliptic eccentric anomaly.
/// ```
///
/// # Detailed Input
///
/// ```text
///  MA       is the elliptic mean anomaly of an orbiting body at
///           some epoch t,
///
///                             3 1/2
///              MA = (t-T)(mu/a )
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
///  E        is the corresponding eccentric anomaly. This is the
///           solution to the time of flight equation
///
///              MA = E - e sin(E)
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the eccentricity (ECC) is outside the range [0,1),
///      the error SPICE(WRONGCONIC) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  Iterate to solve
///
///     f(E,MA,e) = E - e sin(E) - MA = 0
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
///
///  [2]  E. W. Ng, "A General Algorithm for the Solution of Kepler's
///       Equation for Elliptic Orbits", Cel. Mech. 20, pp.243-249,
///       1979.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 14-APR-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.1.0, 29-FEB-1996 (KRG)
///
///         The declaration for the SPICELIB function PI is now
///         preceded by an EXTERNAL statement declaring PI to be an
///         external function. This removes a conflict with any
///         compilers that have a PI intrinsic function.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
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
///         proper range---[0,1)---before proceeding.
/// ```
pub fn elltof(ctx: &mut SpiceContext, ma: f64, ecc: f64, e: &mut f64) -> crate::Result<()> {
    ELLTOF(ma, ecc, e, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure ELLTOF ( Elliptic time of flight )
pub fn ELLTOF(MA: f64, ECC: f64, E: &mut f64, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut M: f64 = 0.0;
    let mut MPRIME: f64 = 0.0;
    let mut N: i32 = 0;
    let mut M0: f64 = 0.0;
    let mut A: f64 = 0.0;
    let mut B: f64 = 0.0;
    let mut Q: f64 = 0.0;
    let mut R: f64 = 0.0;
    let mut QR: f64 = 0.0;
    let mut Y: f64 = 0.0;
    let mut FN: f64 = 0.0;
    let mut DERIV: f64 = 0.0;
    let mut DERIV2: f64 = 0.0;
    let mut CHANGE: f64 = 0.0;

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
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ELLTOF", ctx)?;
    }

    if ((ECC < 0.0) || (ECC >= 1.0)) {
        SIGERR(b"SPICE(WRONGCONIC)", ctx)?;
        CHKOUT(b"ELLTOF", ctx)?;
        return Ok(());
    }

    //
    // For reasons of numerical stability, we would like to restrict
    // our solution to the interval [0,pi]. Because E, M, and sin E
    // are always positive or negative together, we can pretend that M
    // is positive and adjust the sign of the result. And for M, E > pi,
    // we can define
    //
    //       M = 2n pi + M'     and    E = 2n pi + E'
    //
    // where M' and E' are in the interval [-pi,pi]. Solving for E'
    // gives us E.
    //
    // So, we begin by reducing the input mean anomaly to [0,pi].
    //
    M = f64::abs(MA);

    if (M > PI(ctx)) {
        N = ((((M - PI(ctx)) / TWOPI(ctx)) as i32) + 1);
        MPRIME = (M - ((N as f64) * TWOPI(ctx)));
    } else {
        N = 0;
        MPRIME = M;
    }

    M = f64::abs(MPRIME);

    //
    // The convergence of the iterative scheme below depends on a good
    // initial estimate for E.
    //
    // For small eccentricity, the initial estimate E = M is sufficient.
    // However, as the eccentricity increases, so does the number of
    // iterations required for convergence. For sufficiently large
    // eccentricity, this estimate leads to divergence.
    //
    // Ng [2] notes that the function y(M,e)
    //
    //       E - M
    //      -------  =  sin(e y + M)
    //         e
    //
    // increases and decreases monotonically when M is in the ranges
    // [0,M0] and [m0,pi], respectively.
    //
    // When M0 < M < pi, where M0 = (pi/2) - e, the cubic
    //         -   -
    //
    //                          pi - M  2        pi - M    pi - M
    //       B(M,e) = 1 - (1 -  -------)  (1 + 2 ------- - -------)
    //                          pi - M0          pi - M0    1 + e
    //
    // provides a good initial estimate of y for all values of e.
    //
    //
    M0 = (HALFPI(ctx) - ECC);

    if (M >= M0) {
        A = (PI(ctx) - M);
        B = (PI(ctx) - M0);

        Y = (1.0 - (f64::powi((1.0 - (A / B)), 2) * ((1.0 + ((2.0 * A) / B)) - (A / (1.0 + ECC)))));
        *E = ((ECC * f64::sin(((ECC * Y) + M))) + M);

    //
    // The situation is a little more troublesome, however, when M < M0.
    // For small eccentricity, the cubic
    //
    //                              2
    //       A(M,e) = 1 - (1 - M/M0)  (1 + 2M/M0 - M/(1-e) )
    //
    // gives a reasonable first estimate of y. However, as e -> 1,
    // successive approximations of the form
    //
    //                         k           k
    //       C (M,e) = 1 - (-1)  (1 - M/M0)
    //        k
    //
    // are used, where k = 4 for e > 0.7, and k = 8 for e > 0.85.
    //
    // For high eccentricity (e > 0.96) and low mean anomaly (M < 0.05),
    // these successive approximations eventually fail. Fortunately, in
    // just these cases, the cubic
    //
    //                       3    2  1/3           3    2  1/3
    //       D(M,e) = [r + (q  + r )]     + [r - (q  + r )]
    //
    // where
    //
    //       r = 3M/e,   q = (2/e)(1 - e)
    //
    // provides a reasonable estimate of E directly.
    //
    //
    } else if (ECC <= 0.7) {
        Y = (1.0
            - (f64::powi((1.0 - (M / M0)), 2) * ((1.0 + ((2.0 * M) / M0)) - (M / (1.0 - ECC)))));
        *E = ((ECC * f64::sin(((ECC * Y) + M))) + M);
    } else if (ECC <= 0.85) {
        Y = (1.0 - f64::powi((1.0 - (M / M0)), 4));
        *E = ((ECC * f64::sin(((ECC * Y) + M))) + M);
    } else if ((ECC <= 0.96) || (M > 0.05)) {
        Y = (1.0 - f64::powi((1.0 - (M / M0)), 8));
        *E = ((ECC * f64::sin(((ECC * Y) + M))) + M);
    } else {
        Q = ((2.0 / ECC) * (1.0 - ECC));
        R = (3.0 * (M / ECC));
        QR = f64::sqrt((f64::powi(Q, 3) + f64::powi(R, 2)));

        *E = (DCBRT((R + QR)) + DCBRT((R - QR)));
    }

    //
    // Use the Newton second-order method,
    //
    //                                       2
    //      E    = E  - (f/f')*(1 + f*f''/2f' )
    //       i+1    i
    //
    // where
    //
    //      f   = E - e sin(E) - M
    //      f'  = 1 - e cos(E)
    //      f'' =     e sin(E)
    //
    CHANGE = 1.0;

    while (f64::abs(CHANGE) > TOL) {
        FN = ((*E - (ECC * f64::sin(*E))) - M);
        DERIV = (1.0 - (ECC * f64::cos(*E)));
        DERIV2 = (ECC * f64::sin(*E));

        CHANGE = ((FN / DERIV) * (1.0 + ((FN * DERIV2) / (2.0 * f64::powi(DERIV, 2)))));
        *E = (*E - CHANGE);
    }

    //
    // "Unwrap" E' into the actual value of E.
    //
    if (MPRIME < 0 as f64) {
        *E = -*E;
    }

    if (N > 0) {
        *E = (*E + ((N as f64) * TWOPI(ctx)));
    }

    if (MA < 0 as f64) {
        *E = -*E;
    }

    CHKOUT(b"ELLTOF", ctx)?;
    Ok(())
}
