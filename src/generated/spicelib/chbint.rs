//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Interpolate a Chebyshev expansion
///
/// Return the value of a polynomial and its derivative, evaluated at
/// the input X, using the coefficients of the Chebyshev expansion of
/// the polynomial.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  CP         I   DEGP+1 Chebyshev polynomial coefficients.
///  DEGP       I   Degree of polynomial.
///  X2S        I   Transformation parameters of polynomial.
///  X          I   Value for which the polynomial is to be evaluated
///  P          O   Value of the polynomial at X
///  DPDX       O   Value of the derivative of the polynomial at X
/// ```
///
/// # Detailed Input
///
/// ```text
///  CP       is an array of coefficients of a polynomial with
///           respect to the Chebyshev basis. The polynomial to be
///           evaluated is assumed to be of the form:
///
///              CP(DEGP+1)*T(DEGP,S) + CP(DEGP)*T(DEGP-1,S) + ...
///
///                                   + CP(2)*T(1,S) + CP(1)*T(0,S)
///
///           where T(I,S) is the I'th Chebyshev polynomial
///           evaluated at a number S whose double precision
///           value lies between -1 and 1. The value of S is
///           computed from the input variables X2S(1), X2S(2) and X.
///
///  DEGP     is the degree of the Chebyshev polynomial to be
///           evaluated.
///
///  X2S      is an array of two parameters. These parameters are
///           used to transform the domain of the input variable X
///           into the standard domain of the Chebyshev polynomial.
///           X2S(1) should be a reference point in the domain of X;
///           X2S(2) should be the radius by which points are
///           allowed to deviate from the reference point and while
///           remaining within the domain of X. The value of
///           X is transformed into the value S given by
///
///              S = ( X - X2S(1) ) / X2S(2)
///
///           Typically X2S(1) is the midpoint of the interval over
///           which X is allowed to vary and X2S(2) is the radius of
///           the interval.
///
///           The main reason for doing this is that a Chebyshev
///           expansion is usually fit to data over a span
///           from A to B where A and B are not -1 and 1
///           respectively. Thus to get the "best fit" the
///           data was transformed to the interval [-1,1] and
///           coefficients generated. These coefficients are
///           not rescaled to the interval of the data so that
///           the numerical "robustness" of the Chebyshev fit will
///           not be lost. Consequently, when the "best fitting"
///           polynomial needs to be evaluated at an intermediate
///           point, the point of evaluation must be transformed
///           in the same way that the generating points were
///           transformed.
///
///  X        is the value for which the polynomial is to be
///           evaluated.
/// ```
///
/// # Detailed Output
///
/// ```text
///  P        is the value of the polynomial to be evaluated. It
///           is given by
///
///              CP(DEGP+1)*T(DEGP,S) + CP(DEGP)*T(DEGP-1,S) + ...
///
///                                   + CP(2)*T(1,S) + CP(1)*T(0,S)
///
///           where T(I,S) is the I'th Chebyshev polynomial
///           evaluated  at a number S = ( X - X2S(1) )/X2S(2)
///
///  DPDX     is the value of the derivative of the polynomial at X.
///           It is given by
///
///              1/X2S(2) [    CP(DEGP+1)*T'(DEGP,S)
///
///                          + CP(DEGP)*T'(DEGP-1,S)
///
///                          + ...
///
///                          + CP(2)*T'(1,S)
///
///                          + CP(1)*T'(0,S) ]
///
///           where T(I,S) and T'(I,S) are the I'th Chebyshev
///           polynomial and its derivative, respectively,
///           evaluated  at a number S = ( X - X2S(1) )/X2S(2)
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  No tests are performed for exceptional values (DEGP negative,
///      etc.). This routine is expected to be used at a low level in
///      ephemeris evaluations. For that reason it has been elected as
///      a routine that will not participate in error handling.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine computes the value of a Chebyshev polynomial
///  expansion and the derivative of the expansion with respect to X.
///  The polynomial is given by
///
///     CP(DEGP+1)*T(DEGP,S) + CP(DEGP)*T(DEGP-1,S) + ...
///
///                          + CP(2)*T(1,S) + CP(1)*T(0,S)
///
///  where
///
///     S  =  ( X - X2S(1) ) / X2S(2)
///
///  and
///
///     T(I,S) is the I'th Chebyshev polynomial of the first kind
///     evaluated at S.
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
///  1) Depending upon the user's needs, there are 3 routines
///     available for evaluating Chebyshev polynomials.
///
///        CHBVAL   for evaluating a Chebyshev polynomial when no
///                 derivatives are desired.
///
///        CHBINT   for evaluating a Chebyshev polynomial and its
///                 first derivative.
///
///        CHBDER   for evaluating a Chebyshev polynomial and a user
///                 or application dependent number of derivatives.
///
///     Of these 3 the one most commonly employed by SPICE software
///     is CHBINT as it is used to interpolate ephemeris state
///     vectors; this requires the evaluation of a polynomial
///     and its derivative. When no derivatives are desired one
///     should use CHBVAL, or when more than one or an unknown
///     number of derivatives are desired one should use CHBDER.
///
///     The code example below illustrates how this routine might
///     be used to obtain points for plotting a polynomial
///     and its derivative.
///
///
///     Example code begins here.
///
///
///           PROGRAM CHBINT_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      CP     (7)
///           DOUBLE PRECISION      DPDX
///           DOUBLE PRECISION      X
///           DOUBLE PRECISION      P
///           DOUBLE PRECISION      X2S    (2)
///
///           INTEGER               DEGP
///           INTEGER               I
///
///     C
///     C     Set the coefficients of the polynomial and its
///     C     transformation parameters
///     C
///           DATA                  CP     / 1.D0,  3.D0,  0.5D0,
///          .                               1.D0,  0.5D0, -1.D0,
///          .                               1.D0               /
///           DATA                  X2S    / 0.5D0, 3.D0 /
///
///           DEGP   = 6
///           X      = 1.D0
///
///           CALL CHBINT ( CP, DEGP, X2S, X, P, DPDX )
///
///           WRITE(*,'(A,F10.6)')
///          .        'Value of the polynomial at X=1: ', P
///           WRITE(*,'(A,F10.6)') '   First derivative'
///          .                 //  ' at X=1    : ', DPDX
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Value of the polynomial at X=1:  -0.340878
///        First derivative at X=1    :   0.382716
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  One needs to be careful that the value
///
///         (X-X2S(1)) / X2S(2)
///
///      lies between -1 and 1. Otherwise, the routine may fail
///      spectacularly (for example with a floating point overflow).
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  W. Press, B. Flannery, S. Teukolsky and W. Vetterling,
///       "Numerical Recipes -- The Art of Scientific Computing,"
///       chapter 5.4, "Recurrence Relations and Clenshaw's Recurrence
///       Formula," p 161, Cambridge University Press, 1986.
///
///  [2]  T. Rivlin, "The Chebyshev Polynomials," Wiley, 1974.
///
///  [3]  R. Weast and S. Selby, "CRC Handbook of Tables for
///       Mathematics," 4th Edition, CRC Press, 1976.
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
/// -    SPICELIB Version 1.1.0, 26-OCT-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Updated the header to comply with NAIF standard. Added
///         full code example.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    Beta Version 1.0.1, 30-DEC-1988 (WLT)
///
///         The Error free specification was added to the routine as
///         well as an explanation for this designation. Examples added.
/// ```
pub fn chbint(cp: &[f64], degp: i32, x2s: &[f64; 2], x: f64, p: &mut f64, dpdx: &mut f64) {
    CHBINT(cp, degp, x2s, x, p, dpdx);
}

//$Procedure CHBINT ( Interpolate a Chebyshev expansion )
pub fn CHBINT(CP: &[f64], DEGP: i32, X2S: &[f64], X: f64, P: &mut f64, DPDX: &mut f64) {
    let CP = DummyArray::new(CP, 1..);
    let X2S = DummyArray::new(X2S, 1..=2);
    let mut J: i32 = 0;
    let mut W = StackArray::<f64, 3>::new(1..=3);
    let mut DW = StackArray::<f64, 3>::new(1..=3);
    let mut S: f64 = 0.0;
    let mut S2: f64 = 0.0;

    //
    // Local variables
    //

    //
    // Transform X to S and initialize temporary variables.
    //
    S = ((X - X2S[1]) / X2S[2]);
    S2 = (2.0 * S);
    J = (DEGP + 1);
    W[1] = 0.0;
    W[2] = 0.0;
    DW[1] = 0.0;
    DW[2] = 0.0;

    //
    // Evaluate the polynomial and its derivative using recursion.
    //
    while (J > 1) {
        W[3] = W[2];
        W[2] = W[1];
        W[1] = (CP[J] + ((S2 * W[2]) - W[3]));

        DW[3] = DW[2];
        DW[2] = DW[1];
        DW[1] = (((W[2] * 2.0) + (DW[2] * S2)) - DW[3]);

        J = (J - 1);
    }

    *P = (CP[1] + ((S * W[1]) - W[2]));
    *DPDX = ((W[1] + (S * DW[1])) - DW[2]);

    //
    // Scale the derivative by 1/X2S(2) so that we have the derivative
    //
    //                   d P(S)
    //                   ------
    //                     dX
    //
    *DPDX = (*DPDX / X2S[2]);
}
