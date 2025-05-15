//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Lagrange polynomial interpolation
///
/// Evaluate a Lagrange interpolating polynomial for a specified
/// set of coordinate pairs, at a specified abscissa value.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  N          I   Number of points defining the polynomial.
///  XVALS      I   Abscissa values.
///  YVALS      I   Ordinate values.
///  WORK      I-O  Work space array.
///  X          I   Point at which to interpolate the polynomial.
///
///  The function returns the value at X of the unique polynomial of
///  degree N-1 that fits the points in the plane defined by XVALS and
///  YVALS.
/// ```
///
/// # Detailed Input
///
/// ```text
///  N        is the number of points defining the polynomial.
///           The arrays XVALS and YVALS contain N elements.
///
///  XVALS,
///  YVALS    are arrays of abscissa and ordinate values that
///           together define N ordered pairs. The set of points
///
///              ( XVALS(I), YVALS(I) )
///
///           define the Lagrange polynomial used for
///           interpolation. The elements of XVALS must be
///           distinct and in increasing order.
///
///  WORK     is a work space array of the same dimension as
///           XVALS and YVALS. It is used by this routine as a
///           scratch area to hold intermediate results.
///
///  X        is the abscissa value at which the interpolating
///           polynomial is to be evaluated.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the value at X of the unique polynomial of
///  degree N-1 that fits the points in the plane defined by XVALS and
///  YVALS.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If any two elements of the array XVALS are equal, the error
///      SPICE(DIVIDEBYZERO) is signaled. The function will return the
///      value 0.D0.
///
///  2)  If N is less than 1, the error SPICE(INVALIDSIZE) is
///      signaled. The function will return the value 0.D0.
///
///  3)  This routine does not attempt to ward off or diagnose
///      arithmetic overflows.
/// ```
///
/// # Particulars
///
/// ```text
///  Given a set of N distinct abscissa values and corresponding
///  ordinate values, there is a unique polynomial of degree N-1, often
///  called the "Lagrange polynomial", that fits the graph defined by
///  these values. The Lagrange polynomial can be used to interpolate
///  the value of a function at a specified point, given a discrete
///  set of values of the function.
///
///  Users of this routine must choose the number of points to use
///  in their interpolation method. The authors of Reference [1] have
///  this to say on the topic:
///
///     Unless there is solid evidence that the interpolating function
///     is close in form to the true function F, it is a good idea to
///     be cautious about high-order interpolation. We
///     enthusiastically endorse interpolations with 3 or 4 points, we
///     are perhaps tolerant of 5 or 6; but we rarely go higher than
///     that unless there is quite rigorous monitoring of estimated
///     errors.
///
///  The same authors offer this warning on the use of the
///  interpolating function for extrapolation:
///
///     ...the dangers of extrapolation cannot be overemphasized:
///     An interpolating function, which is perforce an extrapolating
///     function, will typically go berserk when the argument X is
///     outside the range of tabulated values by more than the typical
///     spacing of tabulated points.
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
///  1) Fit a cubic polynomial through the points
///
///         ( -1, -2 )
///         (  0, -7 )
///         (  1, -8 )
///         (  3, 26 )
///
///     and evaluate this polynomial at X = 2.
///
///     The returned value of LGRINT should be 1.D0, since the
///     unique cubic polynomial that fits these points is
///
///                  3      2
///        F(X)  =  X  + 2*X  - 4*X - 7
///
///
///     Example code begins here.
///
///
///           PROGRAM LGRINT_EX1
///           IMPLICIT NONE
///
///           DOUBLE PRECISION      LGRINT
///           DOUBLE PRECISION      ANSWER
///           DOUBLE PRECISION      XVALS (4)
///           DOUBLE PRECISION      YVALS (4)
///           DOUBLE PRECISION      WORK  (4)
///           INTEGER               N
///
///           N         =   4
///
///           XVALS(1)  =  -1.D0
///           XVALS(2)  =   0.D0
///           XVALS(3)  =   1.D0
///           XVALS(4)  =   3.D0
///
///           YVALS(1)  =  -2.D0
///           YVALS(2)  =  -7.D0
///           YVALS(3)  =  -8.D0
///           YVALS(4)  =  26.D0
///
///           ANSWER    =   LGRINT ( N, XVALS, YVALS, WORK, 2.D0 )
///
///           WRITE (*,*) 'ANSWER = ', ANSWER
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      ANSWER =    1.0000000000000000
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  W. Press, B. Flannery, S. Teukolsky and W. Vetterling,
///       "Numerical Recipes -- The Art of Scientific Computing,"
///       chapters 3.0 and 3.1, Cambridge University Press, 1986.
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
/// -    SPICELIB Version 1.1.0, 19-MAY-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Updated the header to comply with NAIF standard.
///         Added "IMPLICIT NONE" to code example.
///
/// -    SPICELIB Version 1.0.1, 10-JAN-2014 (NJB)
///
///         Updated description of the workspace array: now the array WORK
///         is not described as being allowed to coincide with the input
///         YVALS. Such overlap would be a violation of the ANSI Fortran
///         77 standard. Corrected several spelling errors in header
///         documentation.
///
/// -    SPICELIB Version 1.0.0, 16-AUG-1993 (NJB)
/// ```
pub fn lgrint(
    ctx: &mut SpiceContext,
    n: i32,
    xvals: &[f64],
    yvals: &[f64],
    work: &mut [f64],
    x: f64,
) -> crate::Result<f64> {
    let ret = LGRINT(n, xvals, yvals, work, x, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure LGRINT ( Lagrange polynomial interpolation )
pub fn LGRINT(
    N: i32,
    XVALS: &[f64],
    YVALS: &[f64],
    WORK: &mut [f64],
    X: f64,
    ctx: &mut Context,
) -> f2rust_std::Result<f64> {
    let XVALS = DummyArray::new(XVALS, 1..);
    let YVALS = DummyArray::new(YVALS, 1..);
    let mut WORK = DummyArrayMut::new(WORK, 1..);
    let mut LGRINT: f64 = 0.0;
    let mut C1: f64 = 0.0;
    let mut C2: f64 = 0.0;
    let mut DENOM: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Check in only if an error is detected.
    //
    if RETURN(ctx) {
        LGRINT = 0.0;
        return Ok(LGRINT);
    }

    //
    // No data, no interpolation.
    //
    if (N < 1) {
        LGRINT = 0.0;

        CHKIN(b"LGRINT", ctx)?;
        SETMSG(b"Array size must be positive; was #.", ctx);
        ERRINT(b"#", N, ctx);
        SIGERR(b"SPICE(INVALIDSIZE)", ctx)?;
        CHKOUT(b"LGRINT", ctx)?;
        return Ok(LGRINT);
    }

    //
    // We're going to compute the value of our interpolating polynomial
    // at X by taking advantage of a recursion relation between
    // Lagrange polynomials of order n+1 and order n.  The method works
    // as follows:
    //
    //    Define
    //
    //       P               (x)
    //        i(i+1)...(i+j)
    //
    //    to be the unique Lagrange polynomial that interpolates our
    //    input function at the abscissa values
    //
    //       x ,  x   , ... x   .
    //        i    i+1       i+j
    //
    //
    //    Then we have the recursion relation
    //
    //       P              (x)  =
    //        i(i+1)...(i+j)
    //
    //                              x - x
    //                               i
    //                             -----------  *  P                (x)
    //                              x - x           (i+1)...(i+j)
    //                               i   i+j
    //
    //
    //                              x  -  x
    //                                     i+j
    //                           + -----------  *  P                (x)
    //                              x  -  x         i(i+1)...(i+j-1)
    //                               i     i+j
    //
    //
    //    Repeated application of this relation allows us to build
    //    successive columns, in left-to-right order, of the
    //    triangular table
    //
    //
    //       P (x)
    //        1
    //                P  (x)
    //                 12
    //       P (x)             P   (x)
    //        2                 123
    //                P  (x)
    //                 23               .
    //                         P   (x)
    //       .                  234            .
    //       .
    //       .        .                               .
    //                .
    //                .        .                           P      (x)
    //                         .                      .     12...N
    //                         .
    //                                         .
    //
    //                                  .
    //
    //
    //                         P           (x)
    //                          (N-2)(N-1)N
    //                P     (x)
    //                 (N-1)N
    //       P (x)
    //        N
    //
    //
    //    and after N-1 steps arrive at our desired result,
    //
    //
    //       P       (x).
    //        12...N
    //
    //
    // The computation is easier to do than to describe.
    //
    //
    // We'll use the scratch array WORK to contain the current column of
    // our interpolation table.  To start out with, WORK(I) will contain
    //
    //    P (x).
    //     I
    //
    //
    for I in 1..=N {
        WORK[I] = YVALS[I];
    }

    //
    // Compute columns 2 through N of the table.  Note that DENOM must
    // be non-zero, or else a divide-by-zero error will occur.
    //
    for J in 1..=(N - 1) {
        for I in 1..=(N - J) {
            DENOM = (XVALS[I] - XVALS[(I + J)]);

            if (DENOM == 0.0) {
                LGRINT = 0.0;
                CHKIN(b"LGRINT", ctx)?;
                SETMSG(b"XVALS(#) = XVALS(#) = #", ctx);
                ERRINT(b"#", I, ctx);
                ERRINT(b"#", (I + J), ctx);
                ERRDP(b"#", XVALS[I], ctx);
                SIGERR(b"SPICE(DIVIDEBYZERO)", ctx)?;
                CHKOUT(b"LGRINT", ctx)?;
                return Ok(LGRINT);
            }

            C1 = (X - XVALS[(I + J)]);
            C2 = (XVALS[I] - X);

            WORK[I] = (((C1 * WORK[I]) + (C2 * WORK[(I + 1)])) / DENOM);
        }
    }

    //
    // Our result is sitting in WORK(1) at this point.
    //
    LGRINT = WORK[1];

    Ok(LGRINT)
}
