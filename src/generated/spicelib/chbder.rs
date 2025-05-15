//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Derivatives of a Chebyshev expansion
///
/// Return the value of a polynomial and its first NDERIV
/// derivatives, evaluated at the input X, using the coefficients of
/// the Chebyshev expansion of the polynomial.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  CP         I   DEGP+1 Chebyshev polynomial coefficients.
///  DEGP       I   Degree of polynomial.
///  X2S        I   Transformation parameters of polynomial.
///  X          I   Value for which the polynomial is to be evaluated.
///  NDERIV     I   The number of derivatives to compute.
///  PARTDP    I-O  Workspace provided for computing derivatives.
///  DPDXS      O   Array of the derivatives of the polynomial.
/// ```
///
/// # Detailed Input
///
/// ```text
///  CP       is an array of coefficients a polynomial with respect
///           to the Chebyshev basis. The polynomial to be
///           evaluated is assumed to be of the form:
///
///              CP(DEGP+1)*T(DEGP,S) + CP(DEGP)*T(DEGP-1,S) + ...
///
///                                   + CP(2)*T(1,S) + CP(1)*T(0,S)
///
///           where T(I,S) is the I'th Chebyshev polynomial
///           evaluated at a number S whose double precision
///           value lies between -1 and 1. The value of S is
///           computed from the input variables X2S(1), X2S(2)
///           and X.
///
///  DEGP     is the degree of the Chebyshev polynomial to be
///           evaluated.
///
///  X2S      is an array of two parameters. These parameters are
///           used to transform the domain of the input variable X
///           into the standard domain of the Chebyshev polynomial.
///           X2S(1) should be a reference point in the domain of
///           X; X2S(2) should be the radius by which points are
///           allowed to deviate from the reference point and while
///           remaining within the domain of X. The value of
///           X is transformed into the value S given by
///
///              S = ( X - X2S(1) ) / X2S(2)
///
///           Typically X2S(1) is the midpoint of the interval over
///           which X is allowed to vary and X2S(2) is the radius
///           of the interval.
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
///
///  NDERIV   is the number of derivatives to be computed by the
///           routine. NDERIV should be non-negative.
///
///  PARTDP   is a work space used by the program to compute
///           all of the desired derivatives. It should be declared
///           in the calling program as
///
///              DOUBLE PRECISION    PARTDP(3, 0:NDERIV)
/// ```
///
/// # Detailed Output
///
/// ```text
///  DPDXS    is an array containing the value of the polynomial and
///           its derivatives evaluated at X.
///
///           DPDXS(0) is the value of the polynomial to be evaluated.
///           It is given by
///
///              CP(DEGP+1)*T(DEGP,S) + CP(DEGP)*T(DEGP-1,S) + ...
///
///                                   + CP(2)*T(1,S) + CP(1)*T(0,S)
///
///           where T(I,S) is the I'th Chebyshev polynomial
///           evaluated  at a number S = ( X - X2S(1) )/X2S(2).
///
///           DPDXS(I) is the value of the I'th derivative of the
///           polynomial at X (I ranges from 1 to NDERIV). It is given
///           by
///
///                                          [i]
///              (1/X2S(2)**I) ( CP(DEGP+1)*T   (DEGP,S)
///
///                                        [i]
///                            + CP(DEGP)*T   (DEGP-1,S)
///
///                            + ...
///
///                                     [i]
///                            + CP(2)*T   (1,S)
///
///                                     [i]
///                            + CP(1)*T   (0,S) )
///
///           where T(k,S) is the K'th Chebyshev polynomial and the
///           superscript [i] indicates its I'th derivative,
///           evaluated at the number S = ( X - X2S(1) )/X2S(2).
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  No tests are performed for exceptional values (NDERIV
///      negative, DEGP negative, etc.). This routine is expected to
///      be used at a low level in ephemeris evaluations. For that
///      reason it has been elected as a routine that will not
///      participate in error handling.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine computes the value of a Chebyshev polynomial
///  expansion and the derivatives of the expansion with respect to X.
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
///     and its derivatives.
///
///
///     Example code begins here.
///
///
///           PROGRAM CHBDER_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      CP     (7)
///
///     C
///     C     Dimension DPDXS as NDERIV + 1.
///     C
///           DOUBLE PRECISION      DPDXS  (3+1)
///
///     C
///     C     Dimension partdp as 3 * (NDERIV + 1)
///     C
///           DOUBLE PRECISION      PARTDP (3 * 4)
///           DOUBLE PRECISION      X
///           DOUBLE PRECISION      X2S    (2)
///
///           INTEGER               DEGP
///           INTEGER               I
///           INTEGER               NDERIV
///
///
///           DATA                  CP     / 1.D0,  3.D0,  0.5D0,
///          .                               1.D0,  0.5D0, -1.D0,
///          .                               1.D0               /
///           DATA                  X2S    / 0.5D0, 3.D0 /
///
///           DEGP   = 6
///           NDERIV = 3
///           X      = 1.D0
///
///           CALL CHBDER ( CP, DEGP, X2S, X, NDERIV, PARTDP, DPDXS )
///
///           WRITE(*,'(A,F10.6)') 'Value of the polynomial at X=1: ',
///          .                                               DPDXS(1)
///
///           DO I=2, NDERIV+1
///              WRITE(*,'(A,I1,A,F10.6)') '   Derivative ', I-1,
///          .                      ' at X=1        : ', DPDXS(I)
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Value of the polynomial at X=1:  -0.340878
///        Derivative 1 at X=1        :   0.382716
///        Derivative 2 at X=1        :   4.288066
///        Derivative 3 at X=1        :  -1.514403
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The user must be sure that the provided workspace is declared
///      properly in the calling routine. The proper declaration is:
///
///         INTEGER            NDERIV
///         PARAMETER        ( NDERIV = desired number of derivatives )
///
///         DOUBLE PRECISION   PARTDP (3, 0:NDERIV)
///
///      If for some reason a parameter is not passed to this routine
///      in NDERIV, the user should make sure that the value of NDERIV
///      is not so large that the work space provided is inadequate.
///
///  2)  One needs to be careful that the value
///
///         (X-X2S(1)) / X2S(2)
///
///      lies between -1 and 1. Otherwise, the routine may fail
///      spectacularly (for example with a floating point overflow).
///
///  3)  While this routine will compute derivatives of the input
///      polynomial, the user should consider how accurately the
///      derivatives of the Chebyshev fit, match the derivatives of the
///      function it approximates.
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
///         Fixed formulae in $Detailed_Output: replaced P(2) and P(1) by
///         X2S(2) and X2S(1) respectively.
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
/// -    Beta Version 1.0.1, 16-FEB-1988 (WLT) (NJB)
///
///         The Error free specification was added to the routine as
///         well as an explanation for this designation. Examples added.
///         Declaration of unused variable RECIP removed.
/// ```
pub fn chbder(
    cp: &[f64],
    degp: i32,
    x2s: &[f64; 2],
    x: f64,
    nderiv: i32,
    partdp: &mut [[f64; 3]],
    dpdxs: &mut [f64],
) {
    CHBDER(cp, degp, x2s, x, nderiv, partdp.as_flattened_mut(), dpdxs);
}

//$Procedure CHBDER ( Derivatives of a Chebyshev expansion )
pub fn CHBDER(
    CP: &[f64],
    DEGP: i32,
    X2S: &[f64],
    X: f64,
    NDERIV: i32,
    PARTDP: &mut [f64],
    DPDXS: &mut [f64],
) {
    let CP = DummyArray::new(CP, 1..);
    let X2S = DummyArray::new(X2S, 1..=2);
    let mut PARTDP = DummyArrayMut2D::new(PARTDP, 1..=3, 0..);
    let mut DPDXS = DummyArrayMut::new(DPDXS, 0..);
    let mut SCALE: f64 = 0.0;
    let mut S: f64 = 0.0;
    let mut S2: f64 = 0.0;
    let mut J: i32 = 0;

    //
    // Local variables
    //

    //
    // Transform X to S and initialize temporary variables.
    //
    S = ((X - X2S[1]) / X2S[2]);
    S2 = (2.0 * S);
    J = (DEGP + 1);

    for I in 0..=NDERIV {
        PARTDP[[1, I]] = 0.0;
        PARTDP[[2, I]] = 0.0;
    }

    //
    // Evaluate the polynomial ...
    //
    while (J > 1) {
        PARTDP[[3, 0]] = PARTDP[[2, 0]];
        PARTDP[[2, 0]] = PARTDP[[1, 0]];
        PARTDP[[1, 0]] = (CP[J] + ((S2 * PARTDP[[2, 0]]) - PARTDP[[3, 0]]));

        //
        // ... and its derivatives using recursion.
        //
        SCALE = 2.0;

        for I in 1..=NDERIV {
            PARTDP[[3, I]] = PARTDP[[2, I]];
            PARTDP[[2, I]] = PARTDP[[1, I]];

            PARTDP[[1, I]] =
                (((PARTDP[[2, (I - 1)]] * SCALE) + (PARTDP[[2, I]] * S2)) - PARTDP[[3, I]]);

            SCALE = (SCALE + 2.0);
        }

        J = (J - 1);
    }

    DPDXS[0] = (CP[1] + ((S * PARTDP[[1, 0]]) - PARTDP[[2, 0]]));

    SCALE = 1.0;

    for I in 1..=NDERIV {
        DPDXS[I] = (((PARTDP[[1, (I - 1)]] * SCALE) + (PARTDP[[1, I]] * S)) - PARTDP[[2, I]]);

        SCALE = (SCALE + 1 as f64);
    }

    //
    // Scale the k'th derivative w.r.t S by (1/X2S(2)**k) so that we have
    // the derivatives
    //
    //                2          3          4          5
    //    d P(S)     d P(S)     d P(S)     d P(S)     d P(S)
    //    ------     ------     ------     ------     ------
    //                   2          3          4          5
    //      dX         dX         dX         dX         dX
    //
    //
    // NOTE: In the loop that follows we perform division instead of
    //       multiplying by reciprocals so that the algorithm matches
    //       CHBINT.  If multiplication by reciprocals is performed
    //       CHBINT and CHBDER (although mathematically equivalent) will
    //       not produce identical results for the first derivative.
    //
    //
    SCALE = X2S[2];

    for I in 1..=NDERIV {
        DPDXS[I] = (DPDXS[I] / SCALE);
        SCALE = (X2S[2] * SCALE);
    }
}
