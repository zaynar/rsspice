//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Quadratic derivative
///
/// Estimate the derivative of a function by finding the derivative
/// of a quadratic approximating function. This derivative estimate
/// is equivalent to that found by computing the average of forward
/// and backward differences.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  -------------------------------------------------
///  NDIM       I   Dimension of function to be differentiated.
///  F0         I   Function values at left endpoint.
///  F2         I   Function values at right endpoint.
///  DELTA      I   Separation of abscissa points.
///  DFDT       O   Derivative vector.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NDIM     is the dimension of the function to be
///           differentiated. The derivative of each
///           function component will be found.
///
///  F0       is an array of NDIM function values at a point on
///           the real line; we'll refer to this point as X0.
///
///  F2       is an array of NDIM function values at a second
///           point on the real line; we'll refer to this point
///           as X2. The points X0 and X2 must satisfy
///
///              X2 = X0 + 2 * DELTA
///
///
///  DELTA    is one half of the difference between X2 and X0:
///
///              DELTA = ( X2 - X0 ) / 2
///
///           DELTA may be negative but must be non-zero.
/// ```
///
/// # Detailed Output
///
/// ```text
///  DFDT     is an N-dimensional vector representing an estimate
///           of the derivative of the input function at the
///           midpoint X1 of the interval between X0 and X2.
///
///           The Ith component of DFDT is
///
///              ( 1 / (2*DELTA) ) * ( F2(I) - F0(I) )
///
///           We may regard this estimate as the derivative
///           at X1 of a parabola fitted to the points
///
///               ( X0, F0(I) ),  ( X2, F2(I) )
///
///           We may also regard this derivative as the average
///           of the forward and backward first-order
///           differences of the input function defined by
///           F0(I), F2(I), and DELTA.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If DELTA is zero, the error SPICE(DIVIDEBYZERO) is signaled.
///
///  2)  If NDIM is less than 1, this routine will fail in a
///      system-dependent manner.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine estimates the derivative of a vector-valued function
///  using the average of forward and backward differences.
///
///  The derivative estimate computed by this routine is equivalent to
///  that obtained by fitting each component of the function with a
///  parabola at the points
///
///     (X0, f(X0)), (X1, f(X1)), (X2, f(X2))
///
///  where
///
///      X0  =  X1 - DELTA
///      X2  =  X1 + DELTA
///
///  and finding the derivative of the parabolas at X1.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///
///  1) Estimate the derivative of x**2 at x = 2.
///
///     Example code begins here.
///
///
///           PROGRAM QDERIV_EX1
///           IMPLICIT NONE
///
///           DOUBLE PRECISION     DELTA
///           DOUBLE PRECISION     DFDT  (1)
///           DOUBLE PRECISION     F0    (1)
///           DOUBLE PRECISION     F2    (1)
///           INTEGER              N
///
///           N     = 1
///           DELTA = 1.D-3
///           F0(1) = ( 2.D0 - DELTA ) ** 2.D0
///           F2(1) = ( 2.D0 + DELTA ) ** 2.D0
///
///           CALL QDERIV ( N, F0, F2, DELTA, DFDT )
///
///           WRITE ( *, '(1X,A,E25.16)'  ) '4 - DFDT(1) = ',
///          .                               4 - DFDT(1)
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      4 - DFDT(1) =    0.4547473508864641E-12
///
///
///     Note that the difference displayed is platform-dependent, but
///     should be on the order of 1.E-12.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 05-AUG-2020 (JDR)
///
///         Changed input argument name "N" to "NDIM" for consistency with
///         other routines.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 18-DEC-2004 (NJB)
/// ```
pub fn qderiv(
    ctx: &mut SpiceContext,
    ndim: i32,
    f0: &[f64],
    f2: &[f64],
    delta: f64,
    dfdt: &mut [f64],
) -> crate::Result<()> {
    QDERIV(ndim, f0, f2, delta, dfdt, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure QDERIV ( Quadratic derivative )
pub fn QDERIV(
    NDIM: i32,
    F0: &[f64],
    F2: &[f64],
    DELTA: f64,
    DFDT: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let F0 = DummyArray::new(F0, 1..=NDIM);
    let F2 = DummyArray::new(F2, 1..=NDIM);
    let mut DFDT = DummyArrayMut::new(DFDT, 1..=NDIM);

    //
    // Use discovery check-in.
    //
    if (DELTA == 0.0) {
        CHKIN(b"QDERIV", ctx)?;
        SETMSG(
            b"Delta abscissa value is zero; a non-zero value is required.",
            ctx,
        );
        SIGERR(b"SPICE(DIVIDEBYZERO)", ctx)?;
        CHKOUT(b"QDERIV", ctx)?;
        return Ok(());
    }
    //
    //
    // Our derivative estimate is
    //
    //        1/2 * (   Backward_difference / DELTA
    //                + Forward_difference  / DELTA )
    //
    //    =   ( 1/(2*DELTA) ) * ( ( F(X2) - F(X1) ) +  ( F(X1) - F(X0) )
    //
    //    =   ( 1/(2*DELTA) ) * ( ( F(X2) - F(X0) )
    //
    //    =    (0.5/DELTA) * F(X2)  +  (-0.5/DELTA) * F(X0)
    //
    //
    VLCOMG(
        NDIM,
        (0.5 / DELTA),
        F2.as_slice(),
        -(0.5 / DELTA),
        F0.as_slice(),
        DFDT.as_slice_mut(),
    );

    Ok(())
}
