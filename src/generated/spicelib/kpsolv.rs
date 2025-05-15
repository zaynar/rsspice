//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const MXNEWT: i32 = 5;

/// Solve Kepler's Equation --- Vector Form
///
/// Solve the equation X = \< EVEC, U(X) > where U(X) is the unit
/// vector \[ COS(X), SIN(X) ] and  \< , > denotes the two-dimensional
/// dot product.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  EVEC       I   A 2-vector whose magnitude is less than 1.
///
///  The function returns the solution to X = < EVEC, U(X) >
/// ```
///
/// # Detailed Input
///
/// ```text
///  EVEC     is any two dimensional vector whose magnitude is
///           less than 1.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the value X such that the equation
///
///     X = EVEC(1)COS(X) + EVEC(2)SIN(X).
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the magnitude of EVEC is greater than or equal to 1,
///      the error SPICE(EVECOUTOFRANGE) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine uses bisection and Newton's method to find
///  the root of the equation
///
///     X = EVEC(1)COS(X) + EVEC(2)SIN(X).
///
///  This equation is just a "vector form" of Kepler's equation.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose you need to solve the equation
///
///      M = E - e SIN(E)                           [ 1 ]
///
///  for E. If we let X = E - M the equation is transformed to
///
///     0 = X - e SIN( X + M )
///
///       = X - e SIN( M ) COS(X) - e COS(M) SIN ( X )
///
///  Thus if we solve the equation
///
///     X = e SIN(M) COS(X) + e COS(M) SIN(X)
///
///  we can find the value of X we can compute E.
///
///  The code fragment below illustrates how this routine can
///  be used to solve equation [1].
///
///      EVEC(1) = ECC * DSIN(M)
///      EVEC(2) = ECC * DCOS(M)
///      E       = M   + KPSOLV( EVEC )
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
/// -    SPICELIB Version 1.1.1, 26-OCT-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.1.0, 26-AUG-1997 (WLT)
///
///         KPSOLV is now given an initial value of zero so that
///         if an error condition is detected, KPSOLV will have
///         a return value.
///
/// -    SPICELIB Version 1.0.0, 03-JAN-1997 (WLT)
/// ```
pub fn kpsolv(ctx: &mut SpiceContext, evec: &[f64; 2]) -> crate::Result<f64> {
    let ret = KPSOLV(evec, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure KPSOLV ( Solve Kepler's Equation --- Vector Form )
pub fn KPSOLV(EVEC: &[f64], ctx: &mut Context) -> f2rust_std::Result<f64> {
    let EVEC = DummyArray::new(EVEC, 1..=2);
    let mut KPSOLV: f64 = 0.0;
    let mut COSX: f64 = 0.0;
    let mut ECC: f64 = 0.0;
    let mut ECC2: f64 = 0.0;
    let mut H: f64 = 0.0;
    let mut K: f64 = 0.0;
    let mut SINX: f64 = 0.0;
    let mut X: f64 = 0.0;
    let mut XL: f64 = 0.0;
    let mut XM: f64 = 0.0;
    let mut XU: f64 = 0.0;
    let mut Y0: f64 = 0.0;
    let mut YPX: f64 = 0.0;
    let mut YX: f64 = 0.0;
    let mut YXM: f64 = 0.0;
    let mut MAXIT: i32 = 0;

    //
    // MXNEWT is the number of iterations we will perform
    // in the Newtons method for finding the solution to
    // the vector form of Kepler's equation.  It has been
    // empirically determined that 5 iterations is always
    // sufficient on computers have 64 bit double precision
    // numbers.
    //

    //
    // We give the function an initial value, just in case
    // we exit without solving Kepler's equation.
    //
    KPSOLV = 0.0;

    H = EVEC[1];
    K = EVEC[2];

    ECC2 = ((H * H) + (K * K));

    if (ECC2 >= 1.0) {
        CHKIN(b"KPSOLV", ctx)?;
        SETMSG(b"The magnitude of the vector EVEC = ( #, # ) must be less than 1.  However, the magnitude of this vector is #.", ctx);

        ERRDP(b"#", H, ctx);
        ERRDP(b"#", K, ctx);
        ERRDP(b"#", f64::sqrt(ECC2), ctx);

        SIGERR(b"SPICE(EVECOUTOFRANGE)", ctx)?;
        CHKOUT(b"KPSOLV", ctx)?;
        return Ok(KPSOLV);
    }

    //
    // We first approximate the equation 0 = X - H * COS(X) - K * SIN(X)
    // using bisection.  If we let Y(X) = X - H * COS(X) - K * SIN(X)
    //
    //    Y( ECC) =  ECC - <EVEC,U(X)>  =   ECC - ECC*COS(ANGLE_X) > 0
    //    Y(-ECC) = -ECC - <EVEC,U(X)>  =  -ECC - ECC*COS(ANGLE_X) < 0
    //
    // where ANGLE_X is the angle between U(X) and EVEC. Thus -ECC
    // and ECC necessarily bracket the root of the equation Y(X) = 0.
    //
    // Also note that Y'(X) = 1 - < EVEC, V(X) > where V(X) is the
    // unit vector given by U'(X).  Thus Y is an increasing function
    // over the interval from -ECC to ECC.
    //
    // The mid point of ECC and -ECC is 0 and Y(0) = -H.  Thus
    // we can do the first bisection step without doing
    // much in the way of computations.
    //

    Y0 = -H;
    XM = 0.0;
    ECC = f64::sqrt(ECC2);

    if (Y0 > 0.0) {
        XU = 0.0;
        XL = -ECC;
    } else if (Y0 < 0.0) {
        XU = ECC;
        XL = 0.0;
    } else {
        KPSOLV = 0.0;
        return Ok(KPSOLV);
    }
    //
    // Iterate until we are assured of being in a region where
    // Newton's method will converge quickly.  The formula
    // below was empirically determined to give good results.
    //
    MAXIT = intrinsics::MIN0(&[
        32,
        intrinsics::MAX0(&[1, intrinsics::IDNINT((1.0 / (1.0 - ECC)))]),
    ]);

    for I in 1..=MAXIT {
        //
        // Compute the next midpoint.  We bracket XM by XL and XU just in
        // case some kind of strange rounding occurs in the computation
        // of the midpoint.
        //
        XM = intrinsics::DMAX1(&[XL, intrinsics::DMIN1(&[XU, (0.5 * (XL + XU))])]);

        //
        // Compute Y at the midpoint of XU and XL
        //
        YXM = ((XM - (H * f64::cos(XM))) - (K * f64::sin(XM)));
        //
        // Determine the new upper and lower bounds.
        //
        if (YXM > 0.0) {
            XU = XM;
        } else {
            XL = XM;
        }
    }

    //
    // We've bisected into a region where we can now get rapid
    // convergence using Newton's method.
    //
    X = XM;

    for I in 1..=MXNEWT {
        COSX = f64::cos(X);
        SINX = f64::sin(X);
        //
        // Compute Y and Y' at X.  Use these to get the next
        // iteration for X.
        //
        // For those of you who might be wondering, "Why not put
        // in a check for YX .EQ. 0 and return early if we get
        // an exact solution?"  Here's why.  An empirical check
        // of those cases where you can actually escape from the
        // Do-loop  showed that the test YX .EQ. 0 is true
        // only about once in every 10000 case of random inputs
        // of EVEC.  Thus on average the check is a waste of
        // time and we don't bother with it.
        //
        YX = ((X - (H * COSX)) - (K * SINX));
        YPX = ((1.0 + (H * SINX)) - (K * COSX));
        X = (X - (YX / YPX));
    }

    KPSOLV = X;

    Ok(KPSOLV)
}
