//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Chebyshev expansion integral
///
/// Evaluate an indefinite integral of a Chebyshev expansion at a
/// specified point X and return the value of the input expansion at
/// X as well. The constant of integration is selected to make the
/// integral zero when X equals the abscissa value X2S(1).
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  DEGP       I   Degree of input Chebyshev expansion.
///  CP         I   Chebyshev coefficients of input expansion.
///  X2S        I   Transformation parameters.
///  X          I   Abscissa value of evaluation.
///  P          O   Input expansion evaluated at X.
///  ITGRLP     O   Integral evaluated at X.
/// ```
///
/// # Detailed Input
///
/// ```text
///  DEGP     is the degree of the input Chebyshev expansion.
///
///  CP       is an array containing the coefficients of the input
///           Chebyshev expansion. The coefficient of the I'th
///           Chebyshev polynomial is contained in element CP(I+1),
///           for I = 0 : DEGP.
///
///  X2S      is an array containing the "transformation parameters"
///           of the domain of the expansion. Element X2S(1)
///           contains the midpoint of the interval on which the
///           input expansion is defined; X2S(2) is one-half of the
///           length of this interval; this value is called the
///           interval's "radius."
///
///           The input expansion defines a function f(X) on the
///           interval
///
///              [ X2S(1)-X2S(2),  X2S(1)+X2S(2) ]
///
///           as follows:
///
///              Define S = ( X - X2S(1) ) / X2S(2)
///
///
///                                DEGP+1
///                                __
///                                \
///                 f(X) = g(S)  = /  CP(k)  T   (S)
///                                --         k-1
///                                k=1
///
///
///  X        is the abscissa value at which the function defined by
///           the input expansion and its integral are to be
///           evaluated. Normally X should lie in the closed
///           interval
///
///              [ X2S(1)-X2S(2),  X2S(1)+X2S(2) ]
///
///           See the $Restrictions section below.
/// ```
///
/// # Detailed Output
///
/// ```text
///  P,
///  ITGRLP   define S and f(X) as above in the description of the
///           input argument X2S. Then P is f(X), and ITGRLP is
///           an indefinite integral of f(X), evaluated at X.
///
///           The indefinite integral satisfies
///
///              d(ITGRLP)/dX     = f(X)
///
///           and
///
///              ITGRLP( X2S(1) ) = 0
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input degree is negative, the error
///      SPICE(INVALDDEGREE) is signaled.
///
///  2)  If the input interval radius is non-positive, the error
///      SPICE(INVALIDRADIUS) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  Let
///
///     T ,  n = 0, ...
///      n
///
///  represent the nth Chebyshev polynomial of the first kind:
///
///     T (x) = cos( n*arccos(x) )
///      n
///
///  The input coefficients represent the Chebyshev expansion
///
///                    DEGP+1
///                    __
///                    \
///     f(X) = g(S)  = /  CP(k)  T   (S)
///                    --         k-1
///                    k=1
///
///  where
///
///     S = ( X - X2S(1) ) / X2S(2)
///
///  This routine evaluates and returns the value at X of an
///  indefinite integral F(X), where
///
///     dF(X)/dX    = f(X)  for all X in
///                         [X2S(1)-X2S(2), X2S(1)+X2S(2)]
///
///     F( X2S(1) ) = 0
///
///  The value at X of the input expansion
///
///     f(X)
///
///  is returned as well.
///
///  Note that numerical problems may result from applying this
///  routine to abscissa values outside of the interval defined
///  by the input parameters X2S(*). See the $Restrictions section.
///
///  To evaluate Chebyshev expansions and their derivatives, use the
///  SPICELIB routines CHBINT or CHBDER.
///
///  This routine supports the SPICELIB SPK type 20 and PCK type 20
///  evaluators SPKE20 and PCKE20.
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
///  1) Let the domain of a polynomial to be evaluated be the
///     closed interval
///
///        [20, 30]
///
///     Let the input expansion represent the polynomial
///
///                          6
///        f(X)  = g(S) = 5*S
///
///     where
///
///        S     = (X - 20)/10
///
///     Let F(X) be an indefinite integral of f(X) such that
///
///        F(20) = 0
///
///     Evaluate
///
///        f(30) and F(30)
///
///
///     Example code begins here.
///
///
///           PROGRAM CHBIGR_EX1
///           IMPLICIT NONE
///     C
///     C     Local variables
///     C
///           DOUBLE PRECISION      CP    ( 6 )
///           DOUBLE PRECISION      X
///           DOUBLE PRECISION      X2S   ( 2 )
///           DOUBLE PRECISION      P
///           DOUBLE PRECISION      ITGRLP
///
///           INTEGER               DEGP
///
///     C
///     C     Let our domain be the interval [10, 30].
///     C
///           X2S(1) = 20.D0
///           X2S(2) = 10.D0
///
///     C
///     C     Assign the expansion coefficients.
///     C
///           DEGP  = 5
///
///           CP(1) = 0.D0
///           CP(2) = 3.75D0
///           CP(3) = 0.D0
///           CP(4) = 1.875D0
///           CP(5) = 0.D0
///           CP(6) = 0.375D0
///
///     C
///     C     Evaluate the function and its integral at X = 30.
///     C
///           X = 30.D0
///
///           CALL CHBIGR ( DEGP, CP, X2S, X, P, ITGRLP )
///
///     C
///     C     We make the change of variables
///     C
///     C        S(X) = (1/10) * ( X - 20 )
///     C
///     C     The expansion represents the polynomial
///     C
///     C                         5
///     C        f(X) = g(S) = 6*S
///     C
///     C     An indefinite integral of the expansion is
///     C
///     C                                    6
///     C        F(X) = G(S) * dX/dS = 10 * S
///     C
///     C     where G is defined on the interval [-1, 1]. The result
///     C     should be (due to the change of variables)
///     C
///     C          (G(1)  - G(0) ) * dX/dS
///     C
///     C        = (F(30) - F(20)) * 10
///     C
///     C        = 10
///     C
///     C     The value of the expansion at X should be
///     C
///     C        f(30) = g(1) = 6
///     C
///           WRITE (*,*) 'ITGRLP = ', ITGRLP
///           WRITE (*,*) 'P      = ', P
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      ITGRLP =    10.000000000000000
///      P      =    6.0000000000000000
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The value (X-X2S(1)) / X2S(2) normally should lie within the
///      interval -1:1 inclusive, that is, the closed interval
///      [-1, 1]. Chebyshev polynomials increase rapidly in magnitude
///      as a function of distance of abscissa values from this
///      interval.
///
///      In typical SPICE applications, where the input expansion
///      represents position, velocity, or orientation, abscissa
///      values that map to points outside of [-1, 1] due to round-off
///      error will not cause numeric exceptions.
///
///  2)  No checks for floating point overflow are performed.
///
///  3)  Significant accumulated round-off error can occur for input
///      expansions of excessively high degree. This routine imposes
///      no limits on the degree of the input expansion; users must
///      verify that the requested computation provides appropriate
///      accuracy.
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
///  [2]  "Chebyshev polynomials," Wikipedia, The Free Encyclopedia.
///       Retrieved 01:23, November 23, 2013, from
///       http://en.wikipedia.org/w/index.php?title=
///       Chebyshev_polynomials&oldid=574881046
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
/// -    SPICELIB Version 1.0.1, 12-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///         Corrected error in $Detailed_Input description of CP.
///
///         Fixed range of Chebyshev coefficients of input expansion in
///         the description of argument CP in $Detailed_Input.
///
/// -    SPICELIB Version 1.0.0, 03-DEC-2013 (NJB)
/// ```
pub fn chbigr(
    ctx: &mut SpiceContext,
    degp: i32,
    cp: &[f64],
    x2s: &[f64; 2],
    x: f64,
    p: &mut f64,
    itgrlp: &mut f64,
) -> crate::Result<()> {
    CHBIGR(degp, cp, x2s, x, p, itgrlp, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CHBIGR ( Chebyshev expansion integral )
pub fn CHBIGR(
    DEGP: i32,
    CP: &[f64],
    X2S: &[f64],
    X: f64,
    P: &mut f64,
    ITGRLP: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let CP = DummyArray::new(CP, 1..);
    let X2S = DummyArray::new(X2S, 1..=2);
    let mut A2: f64 = 0.0;
    let mut ADEGP1: f64 = 0.0;
    let mut ADEGP2: f64 = 0.0;
    let mut AI: f64 = 0.0;
    let mut C0: f64 = 0.0;
    let mut F = StackArray::<f64, 3>::new(1..=3);
    let mut S: f64 = 0.0;
    let mut S2: f64 = 0.0;
    let mut W = StackArray::<f64, 3>::new(1..=3);
    let mut Z = StackArray::<f64, 3>::new(1..=3);
    let mut I: i32 = 0;
    let mut NTERMS: i32 = 0;

    //
    // SPICELIB functions
    //
    //
    // Local variables
    //

    //
    // Test RETURN but don't check in. Use discovery check-in.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // Check the expansion degree.
    //
    if (DEGP < 0) {
        CHKIN(b"CHBIGR", ctx)?;
        SETMSG(b"Expansion degree must be non-negative but was #.", ctx);
        ERRINT(b"#", DEGP, ctx);
        SIGERR(b"SPICE(INVALIDDEGREE)", ctx)?;
        CHKOUT(b"CHBIGR", ctx)?;
        return Ok(());
    }

    //
    // Check the radius of the domain interval.
    //
    if (X2S[2] <= 0.0) {
        CHKIN(b"CHBIGR", ctx)?;
        SETMSG(b"Interval radius must be positive but was #.", ctx);
        ERRDP(b"#", X2S[2], ctx);
        SIGERR(b"SPICE(INVALIDRADIUS)", ctx)?;
        CHKOUT(b"CHBIGR", ctx)?;
        return Ok(());
    }

    NTERMS = (DEGP + 1);

    //
    // Background
    // ==========
    //
    //
    // Let
    //
    //    T ,  n = 0, ...
    //     n
    //
    // represent the nth Chebyshev polynomial of the first kind:
    //
    //    T (x) = cos( n*arccos(x) )
    //     n
    //
    // These polynomials satisfy the recurrence relationship
    //
    //    T   (x) = 2x T (x)  -  T   (x),  n = 2, ...
    //     n+1          n         n-1
    //
    // The Chebyshev polynomials of the second kind are denoted by
    //
    //    U ,  n = 0, ...
    //     n
    //
    // where
    //
    //    U (x) = 1
    //     0
    //
    //    U (x) = 2x
    //     1
    //
    //    U   (x) = 2x U (x)  -  U   (x),  n = 2, ...
    //     n+1          n         n-1
    //
    //
    // The integration formula (1) below is based on several
    // identities:
    //
    //
    //    T (x)        = (1/2) * ( U (x) - U   (x) ),  n = 2, ...    (B1)
    //     n                        n       n-2
    //
    //
    //    d(T (x))/dx  =  n U   (x),   n = 1, ...                    (B2)
    //       n               n-1
    //
    //
    //                             d(T   (x))/dx   d(T   (x))/dx
    //                                n+1             n-1
    //    T (x)        = (1/2) * ( ------------- - ------------- ),
    //     n                             n+1             n-1
    //
    //                                 n = 2, ...                    (B3)
    //
    //
    // Identity (B1) can be proved via mathematical induction. Using
    // (B1) and the Chebyshev recurrence formulas for both kinds of
    // polynomials, identity (B2) can also be proved via mathematical
    // induction. Identity (B3) follows directly from the combination of
    // (B1) and (B2).
    //
    // Formula (1) below follows from (B3).
    //
    //
    // Algorithm
    // =========
    //
    // In the discussion below, all Chebyshev polynomials are of the
    // first kind.
    //
    // Let the notation
    //
    //    I( f )
    //
    // represent the indefinite integral of a function f.
    //
    // The key formula we use below is
    //
    //                          T          T
    //                           n+1        n-1
    //    I ( T  ) = (1/2) * ( ------  -  ------ )  +  C,  for n > 1  (1)
    //         n                 n+1        n-1
    //
    // where C is a constant of integration. Applying (1) to a Chebyshev
    // expansion
    //
    //           N+1
    //           __
    //           \
    //    f(x) = /  a  T   (x)                                        (2)
    //           --  k  k-1
    //           k=1
    //
    // we have
    //
    //                           N+1
    //                           __       T (x)    T   (x)
    //                           \         k        k-2
    //    I( f(x) ) =      (1/2) /  a (   ------ - -------  )
    //                           --  k       k       k-2
    //                           k=3
    //
    //                + a T (x) + ( a  / 4 ) T (x) + C ,   for N > 1 (3a)
    //                   1 1         2        2       0
    //
    // or
    //
    //    I( f(x) ) = C  + a T (x),   for N = 1                      (3b)
    //                 0    1 1
    //
    // where
    //
    //    C
    //     0
    //
    // is a constant of integration. Then by grouping coefficients of
    // the Chebyshev polynomials, we have
    //
    //
    //                       N+2
    //                       __
    //                       \
    //    I( f(x) ) = C  +   /  A  T   (x),  for N >= 0               (4)
    //                 0     --  k  k-1
    //                       k=2
    //
    // where
    //
    //    A    = a   -  (1/2)a           for N >= 2,    or
    //     2      1           3
    //
    //    A    = a                       for N <= 1                  (5a)
    //     2      1
    //
    //             1
    //    A    = ------ ( a  -  a    )   for k = 3, ... , N (N>=3)   (5b)
    //     k     2(k-1)    k-1   k+1
    //
    //            1
    //    A    = ---- a                  for N >= 2                  (5c)
    //     N+1    2N   N
    //
    //             1
    //    A    = ------ a                for N >= 1                  (5d)
    //     N+2   2(N+1)  N+1
    //
    //
    // Note that (5b) does not conflict with (5c) or (5d) for N < 3,
    // since (5b) is applicable only when N >= 3.
    //
    //
    // We'll compute the sums
    //
    //                       N+2
    //                       __
    //                       \
    //    I( f(x) ) - C  =   /  A  T   (x)                            (6)
    //                 0     --  k  k-1
    //                       k=2
    //
    //           N+2
    //           __
    //           \
    //    C  = - /  A  T   (0)                                        (7)
    //     0     --  k  k-1
    //           k=2
    //
    // and
    //
    //           N+1
    //           __
    //           \
    //    f(x) = /  a  T   (x)
    //           --  k  k-1
    //           k=1
    //
    //
    // using Clenshaw's recurrence formula. Note that in the above
    // equations, N is the degree of the input expansion, which is
    // given by the input argument DEGP.
    //
    // Transform the independent variable X to the interval
    //
    //    [-1, 1]
    //
    // Call the result S.
    //
    // Note we've already checked that X2S(2) is positive.
    //
    S = ((X - X2S[1]) / X2S[2]);
    S2 = (2.0 * S);

    //
    // Pre-compute the coefficients of the integral expansion
    // that are known at this point. The terms A2, ADEGP1, and
    // ADEGP2 correspond to the variables
    //
    //    A , A   , A
    //     2   N+1   N+2
    //
    // above and are set according to equations 5a, 5c, and 5d.
    //
    if (NTERMS >= 3) {
        A2 = (CP[1] - (0.5 * CP[3]));
    } else {
        A2 = CP[1];
    }

    //
    // Initialize the two highest-indexed coefficients of
    // the integral expansion.
    //
    ADEGP1 = 0.0;
    ADEGP2 = 0.0;

    if (DEGP >= 2) {
        ADEGP1 = ((0.5 * CP[DEGP]) / DEGP as f64);
    }

    if (DEGP >= 1) {
        ADEGP2 = ((0.5 * CP[(DEGP + 1)]) / (DEGP + 1) as f64);
    }

    //
    // The three quantities we'll compute require different numbers of
    // loop iterations: the integrals at X and at 0 require that I be
    // initialized to DEGP+2, while the input expansion requires that I
    // be initialized to DEGP+1. Since we wish to save on loop overhead
    // by performing the respective loop body actions in parallel, we
    // perform the integral computations for I = DEGP+2 prior to the
    // start of the loop.
    //
    // Since F(1) and F(2) would normally be initialized to 0 prior to
    // the start of the loop, after the first loop pass, F would have
    // the contents assigned below.
    //
    F[3] = 0.0;
    F[2] = 0.0;
    //
    // The initial value of F(1) is the highest-order coefficient of
    // the integral's expansion.
    //
    if (DEGP == 0) {
        F[1] = A2;
    } else {
        F[1] = ADEGP2;
    }
    //
    // We also want to evaluate [I(f)](0); we'll use this to
    // make the integral 0 at x = 0. We'll use the terms
    // Z(*) to evaluate the expansion of the integral at x = 0.
    //
    Z[3] = 0.0;
    Z[2] = 0.0;
    Z[1] = F[1];
    //
    // We'll use the terms W(*) to evaluate the input expansion.
    //
    W[1] = 0.0;
    W[2] = 0.0;

    //
    // NTERMS is DEGP+1.
    //
    I = NTERMS;

    while (I > 1) {
        //
        // The variable AI represents A(I), which is the Ith coefficient
        // of the Chebyshev expansion of the indefinite integral. AI is
        // set according to formulas 5a-c above.
        //
        if (I == 2) {
            AI = A2;
        } else if (I < NTERMS) {
            //
            // AI is A ; I >= 3; DEGP >= 3.
            //        I
            //
            AI = ((0.5 * (CP[(I - 1)] - CP[(I + 1)])) / (I - 1) as f64);
        } else {
            //
            // AI is A      ; I = NTERMS.
            //        DEGP+1
            //
            AI = ADEGP1;
        }

        F[3] = F[2];
        F[2] = F[1];
        F[1] = (AI + ((S2 * F[2]) - F[3]));

        Z[3] = Z[2];
        Z[2] = Z[1];
        Z[1] = (AI - Z[3]);

        W[3] = W[2];
        W[2] = W[1];
        W[1] = (CP[I] + ((S2 * W[2]) - W[3]));

        I = (I - 1);
    }

    //
    // C0 is the negative of the input expansion evaluated at x=0, and
    // that expansion value is
    //
    //    0*Z(1) - Z(2)
    //
    // The other terms are computed as in CHBINT.
    //
    C0 = Z[2];
    *ITGRLP = ((C0 + (S * F[1])) - F[2]);
    *P = (CP[1] + ((S * W[1]) - W[2]));

    //
    // Scale the integral to account for the change of variables
    // (from the original domain to [-1,1]). The scale factor is
    //
    //    dX/dS = X2S(2)
    //
    *ITGRLP = (X2S[2] * *ITGRLP);

    Ok(())
}
