//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Lagrange polynomial interpolation with derivative
///
/// Evaluate a Lagrange interpolating polynomial, for a specified
/// set of coordinate pairs, at a specified abscissa value. Return
/// both the value of the polynomial and its derivative.
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
///  P          O   Polynomial value at X.
///  DP         O   Polynomial derivative at X.
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
///  WORK     is an N * 2 work space array, where N is the same
///           dimension as that of XVALS and YVALS. It is used
///           by this routine as a scratch area to hold
///           intermediate results.
///
///  X        is the abscissa value at which the interpolating
///           polynomial is to be evaluated.
/// ```
///
/// # Detailed Output
///
/// ```text
///  P        is the value at X of the unique polynomial of
///           degree N-1 that fits the points in the plane
///           defined by XVALS and YVALS.
///
///  DP       is the derivative at X of the interpolating
///           polynomial described above.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If any two elements of the array XVALS are equal, the error
///      SPICE(DIVIDEBYZERO) is signaled.
///
///  2)  If N is less than 1, the error SPICE(INVALIDSIZE) is
///      signaled.
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
///     The returned value of P should be 1.0, since the
///     unique cubic polynomial that fits these points is
///
///                    3      2
///        F(X)   =   X  + 2*X  - 4*X - 7
///
///     The returned value of DP should be 16.0, since the
///     derivative of f(x) is
///
///         '           2
///        F (X)  =  3*X  + 4*X - 4
///
///
///     Example code begins here.
///
///
///           PROGRAM LGRIND_EX1
///           IMPLICIT NONE
///
///           DOUBLE PRECISION      P
///           DOUBLE PRECISION      DP
///           DOUBLE PRECISION      XVALS (4)
///           DOUBLE PRECISION      YVALS (4)
///           DOUBLE PRECISION      WORK  (4,2)
///           INTEGER               N
///
///           N         =   4
///
///           XVALS(1)  =  -1
///           XVALS(2)  =   0
///           XVALS(3)  =   1
///           XVALS(4)  =   3
///
///           YVALS(1)  =  -2
///           YVALS(2)  =  -7
///           YVALS(3)  =  -8
///           YVALS(4)  =  26
///
///           CALL LGRIND ( N, XVALS, YVALS, WORK, 2.D0, P, DP )
///
///           WRITE (*,*) 'P, DP = ', P, DP
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      P, DP =    1.0000000000000000        16.000000000000000
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
/// -    SPICELIB Version 1.1.0, 26-OCT-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Updated the header to comply with NAIF standard. Added
///         "IMPLICIT NONE" to code example.
///
/// -    SPICELIB Version 1.0.1, 10-JAN-2014 (NJB)
///
///         Updated description of the workspace array: now the array WORK
///         is not described as being allowed to coincide with the input
///         YVALS. Such overlap would be a violation of the ANSI Fortran
///         77 standard. Corrected a spelling error in header
///         documentation.
///
/// -    SPICELIB Version 1.0.0, 20-AUG-2002 (NJB)
/// ```
pub fn lgrind(
    ctx: &mut SpiceContext,
    n: i32,
    xvals: &[f64],
    yvals: &[f64],
    work: &mut [f64],
    x: f64,
    p: &mut f64,
    dp: &mut f64,
) -> crate::Result<()> {
    LGRIND(n, xvals, yvals, work, x, p, dp, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure LGRIND (Lagrange polynomial interpolation with derivative)
pub fn LGRIND(
    N: i32,
    XVALS: &[f64],
    YVALS: &[f64],
    WORK: &mut [f64],
    X: f64,
    P: &mut f64,
    DP: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let XVALS = DummyArray::new(XVALS, 1..=N);
    let YVALS = DummyArray::new(YVALS, 1..=N);
    let mut WORK = DummyArrayMut2D::new(WORK, 1..=N, 1..=2);
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
        return Ok(());
    }

    //
    // No data, no interpolation.
    //
    if (N < 1) {
        CHKIN(b"LGRIND", ctx)?;
        SETMSG(b"Array size must be positive; was #.", ctx);
        ERRINT(b"#", N, ctx);
        SIGERR(b"SPICE(INVALIDSIZE)", ctx)?;
        CHKOUT(b"LGRIND", ctx)?;
        return Ok(());
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
    // For columns 2...N of the table, we'll also carry along the
    // derivative at X of each interpolating polynomial.  This will
    // allow us to find the derivative of the Lagrange polynomial
    // at X.
    //

    for I in 1..=N {
        WORK[[I, 1]] = YVALS[I];
        WORK[[I, 2]] = 0.0;
    }

    //
    // Compute columns 2 through N of the table.  Note that DENOM must
    // be non-zero, or else a divide-by-zero error will occur.
    //
    for J in 1..=(N - 1) {
        for I in 1..=(N - J) {
            DENOM = (XVALS[I] - XVALS[(I + J)]);

            if (DENOM == 0.0) {
                CHKIN(b"LGRIND", ctx)?;
                SETMSG(b"XVALS(#) = XVALS(#) = #", ctx);
                ERRINT(b"#", I, ctx);
                ERRINT(b"#", (I + J), ctx);
                ERRDP(b"#", XVALS[I], ctx);
                SIGERR(b"SPICE(DIVIDEBYZERO)", ctx)?;
                CHKOUT(b"LGRIND", ctx)?;
                return Ok(());
            }

            C1 = (X - XVALS[(I + J)]);
            C2 = (XVALS[I] - X);

            //
            // Use the chain rule to compute the derivatives.  Do this
            // before computing the function value, because the latter
            // computation will overwrite the first column of WORK.
            //
            WORK[[I, 2]] = ((((C1 * WORK[[I, 2]]) + (C2 * WORK[[(I + 1), 2]]))
                + (WORK[[I, 1]] - WORK[[(I + 1), 1]]))
                / DENOM);

            //
            // Compute the Ith entry in the Jth column.
            //
            WORK[[I, 1]] = (((C1 * WORK[[I, 1]]) + (C2 * WORK[[(I + 1), 1]])) / DENOM);
        }
    }

    //
    // Our results are sitting in WORK(1,1) and WORK(1,2) at this point.
    //
    *P = WORK[[1, 1]];
    *DP = WORK[[1, 2]];

    Ok(())
}
