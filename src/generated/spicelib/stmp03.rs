//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const TRUNC: i32 = 11;
const HALF: f64 = (1.0 / 2.0);
const EIGHTH: f64 = (1.0 / 8.0);
const NPAIRS: i32 = ((2 * TRUNC) - 2);
const LPAIR3: i32 = NPAIRS;
const LPAIR2: i32 = (NPAIRS - 1);

struct SaveVars {
    LBOUND: f64,
    PAIRS: StackArray<f64, 20>,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut LBOUND: f64 = 0.0;
        let mut PAIRS = StackArray::<f64, 20>::new(1..=NPAIRS);
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            LBOUND,
            PAIRS,
            FIRST,
        }
    }
}

/// Stumpff functions 0 through 3
///
/// Compute the values of the Stumpff functions C_0 through C_3 at
/// a specified point.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  X          I   Argument to each Stumpff function C_0 to C_3.
///  C0         O   Value of C_0(X)
///  C1         O   Value of C_1(X)
///  C2         O   Value of C_2(X)
///  C3         O   Value of C_3(X)
///  TRUNC      P   Number of terms needed in Maclaurin series for C_3.
/// ```
///
/// # Detailed Input
///
/// ```text
///  X        is the argument to use in each of the Stumpff functions
///           C_0, C_1, C_2, and C_3.
/// ```
///
/// # Detailed Output
///
/// ```text
///  C0,
///  C1,
///  C2,
///  C3       are the values of the Stumpff functions C_0(X), C_1(X),
///           C_2(X), and C_3(X).
/// ```
///
/// # Parameters
///
/// ```text
///  TRUNC    is the Maclaurin series for C_3 and C_2 respectively are:
///
///                                  2     3                 k
///                     1     X     X     X              (-X)
///           C_3(X) =  --- - --- + --- - --- + . . . + ----------. . .
///                     3!    5!    7!    9!           (3 + 2*K)!
///
///           and
///
///                                  2     3                 k
///                     1     X     X     X              (-X)
///           C_2(X) =  --- - --- + --- - --- + . . . + ----------. . .
///                     2!    4!    6!    8!           (2 + 2*K)!
///
///           These series are used in the evaluation of C_3 and C_2.
///           Thus, it is necessary to make a decision about where to
///           truncate the series in our evaluation of C_3 and C_2.
///
///           TRUNC is used to tell this routine where to truncate
///           the Maclaurin series for C_3 and C_2.
///
///           The value of TRUNC for your machine  is the smallest
///           integer such that
///
///                             1
///               1.0D0  +  ----------        =  1.0D0
///                         (2*TRUNC)!
///
///           The following program will (if compiled and linked)
///           will produce the values of TRUNC for your machine.
///
///           INTEGER               TRUNC
///
///           DOUBLE PRECISION      DENOM
///           DOUBLE PRECISION      FACTR
///
///           DOUBLE PRECISION      X
///
///           DENOM = 2.0D0
///           FACTR = 2.0D0
///           TRUNC = 1
///
///           X      = 1.0D0 / DENOM
///
///           DO WHILE ( 1.0D0 + X .GT. 1.0D0 )
///              DENOM = DENOM * (2.0D0+FACTR) * (1.0D0+FACTR)
///              FACTR = FACTR +  2.0D0
///              TRUNC = TRUNC +  1
///              X     = 1.0D0 /  DENOM
///           END DO
///
///           WRITE (*,*) 'The value of TRUNC is: ', TRUNC
///
///           END
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input value of X is not in the domain of values
///      for which the Stumpff functions can be computed, the error
///      SPICE(VALUEOUTOFRANGE) is signaled.
///
///      The range of valid inputs is from  -[ln(2) + ln(DPMAX)]**2
///      to DPMAX.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine computes the values of the Stumpff functions C_0,
///  C_1, C_2, and C_3 at the input X.
///
///  The Stumpff function C_k(X) for k = 0, 1, ... is given by the
///  series:
///
///                              2        3                  m
///             1      X        X        X               (-X)
///  C_k(X) =  --- - ------ + ------ - ------ + . . . + ------- + . . .
///             k!   (k+2)!   (k+4)!   (k+6)!           (k+2m)!
///
///
///  These series converge for all real values of X.
/// ```
///
/// # Examples
///
/// ```text
///  For positive X,
///
///     C_0(X)   =  COS ( DSQRT(X) )
///
///
///                 SIN ( DSQRT(X) )
///     C_1(X)   =  ---------------
///                       DSQRT(X)
///
///
///                 1 - COS ( DSQRT(X) )
///     C_2(X)   = ---------------------
///                        X
///
///
///
///                 1  -  SIN ( DSQRT(X) ) / DSQRT(X)
///     C_3(X)   =  ----------------------------------
///                           X
///
///  Thus the following block of code can be used to check this
///  routine for reasonableness:
///
///  INTEGER               I
///
///  DOUBLE PRECISION      X
///  DOUBLE PRECISION      ROOTX
///
///  DOUBLE PRECISION      TC0
///  DOUBLE PRECISION      TC1
///  DOUBLE PRECISION      TC2
///  DOUBLE PRECISION      TC3
///
///  DOUBLE PRECISION      C0
///  DOUBLE PRECISION      C1
///  DOUBLE PRECISION      C2
///  DOUBLE PRECISION      C3
///
///  DO I = 1, 10
///
///     X     = DBLE (I)
///     ROOTX = DSQRT(X)
///
///     TC0   = COS ( ROOTX )
///     TC1   = SIN ( ROOTX ) / ROOTX
///
///     TC2   = ( 1.0D0 - COS( ROOTX )         ) / X
///     TC3   = ( 1.0D0 - SIN( ROOTX ) / ROOTX ) / X
///
///     CALL STMP03 ( X, C0, C1, C2, C3 )
///
///     WRITE (*,*)
///     WRITE (*,*) 'Expected - Computed for X = ', X
///     WRITE (*,*)
///     WRITE (*,*) 'Delta C0 :', TC0 - C0
///     WRITE (*,*) 'Delta C1 :', TC1 - C1
///     WRITE (*,*) 'Delta C2 :', TC2 - C2
///     WRITE (*,*) 'Delta C3 :', TC3 - C3
///
///  END DO
///
///  END
///
///  You should expect all of the differences to be on the order of
///  the precision of the machine on which this program is executed.
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  J. Danby, "Fundamentals of Celestial Mechanics," 2nd Edition,
///       pp.345-347, Willman-Bell, 1989.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.29.0, 28-NOV-2021 (BVS)
///
///         Updated for MAC-OSX-M1-64BIT-CLANG_C.
///
/// -    SPICELIB Version 3.28.0, 12-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 3.27.0, 08-APR-2014 (NJB)
///
///         Updated in-line documentation and cleaned up
///         code following changes made in version 3.21.0.
///
/// -    SPICELIB Version 3.26.0, 10-MAR-2014 (BVS)
///
///         Updated for SUN-SOLARIS-64BIT-INTEL.
///
/// -    SPICELIB Version 3.25.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-LINUX-64BIT-IFORT.
///
/// -    SPICELIB Version 3.24.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-GFORTRAN.
///
/// -    SPICELIB Version 3.23.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 3.22.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-64BIT-GCC_C.
///
/// -    SPICELIB Version 3.21.0, 09-APR-2012 (WLT)
///
///         Code was updated to correct excessive round-off
///         errors in the case where |X| > 1.
///
/// -    SPICELIB Version 3.20.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-INTEL.
///
/// -    SPICELIB Version 3.19.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-INTEL-CC_C.
///
/// -    SPICELIB Version 3.18.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-INTEL-64BIT-CC_C.
///
/// -    SPICELIB Version 3.17.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-64BIT-NATIVE_C.
///
/// -    SPICELIB Version 3.16.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-WINDOWS-64BIT-IFORT.
///
/// -    SPICELIB Version 3.15.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-LINUX-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 3.14.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-64BIT-MS_C.
///
/// -    SPICELIB Version 3.13.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-INTEL_C.
///
/// -    SPICELIB Version 3.12.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-IFORT.
///
/// -    SPICELIB Version 3.11.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 3.10.0, 18-MAR-2009 (BVS)
///
///         Updated for PC-LINUX-GFORTRAN.
///
/// -    SPICELIB Version 3.9.0, 18-MAR-2009 (BVS)
///
///         Updated for MAC-OSX-GFORTRAN.
///
/// -    SPICELIB Version 3.8.0, 19-FEB-2008 (BVS)
///
///         Updated for PC-LINUX-IFORT.
///
/// -    SPICELIB Version 3.7.0, 14-NOV-2006 (BVS)
///
///         Updated for PC-LINUX-64BIT-GCC_C.
///
/// -    SPICELIB Version 3.6.0, 14-NOV-2006 (BVS)
///
///         Updated for MAC-OSX-INTEL_C.
///
/// -    SPICELIB Version 3.5.0, 14-NOV-2006 (BVS)
///
///         Updated for MAC-OSX-IFORT.
///
/// -    SPICELIB Version 3.4.0, 14-NOV-2006 (BVS)
///
///         Updated for PC-WINDOWS-IFORT.
///
/// -    SPICELIB Version 3.3.0, 26-OCT-2005 (BVS)
///
///         Updated for SUN-SOLARIS-64BIT-GCC_C.
///
/// -    SPICELIB Version 3.2.0, 03-JAN-2005 (BVS)
///
///         Updated for PC-CYGWIN_C.
///
/// -    SPICELIB Version 3.1.0, 03-JAN-2005 (BVS)
///
///         Updated for PC-CYGWIN.
///
/// -    SPICELIB Version 3.0.5, 17-JUL-2002 (BVS)
///
///         Added MAC-OSX environments.
///
/// -    SPICELIB Version 3.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 3.0.3, 24-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 3.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 3.0.1, 18-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 3.0.0, 08-APR-1998 (NJB)
///
///         Module was updated for the PC-LINUX platform.
///
/// -    SPICELIB Version 2.0.0, 11-NOV-1993 (HAN)
///
///         The file was modified to include values for other platforms.
///         Also, the file was formatted for use by the program that
///         creates the environment specific source files.
///
/// -    SPICELIB Version 1.0.0, 17-FEB-1992 (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 3.27.0, 08-APR-2014 (NJB) (WLT)
///
///         In version 3.21.0, the routine was re-written to use the
///         standard trigonometric and hyperbolic trigonometric formulas
///         for the Stumpff functions for input arguments having magnitude
///         greater than or equal to 1. This was done to prevent loss of
///         accuracy for some input values.
///
///         In version 3.27.0, the code was cleaned up: unreachable code
///         was deleted, and comments were changed to match the updated
///         code.
///
///         The derivation of the argument mapping formulas has been
///         retained as an appendix in a comment section at the end of
///         this source file. These formulas may be used a future revision
///         of this routine.
///
/// -    SPICELIB Version 3.0.0, 08-APR-1998 (NJB)
///
///         Module was updated for the PC-LINUX platform.
///
/// -    SPICELIB Version 2.0.0, 11-NOV-1993 (HAN)
///
///         The file was modified to include values for other platforms.
///         Also, the file was formatted for use by the program that
///         creates the environment specific source files.
/// ```
pub fn stmp03(
    ctx: &mut SpiceContext,
    x: f64,
    c0: &mut f64,
    c1: &mut f64,
    c2: &mut f64,
    c3: &mut f64,
) -> crate::Result<()> {
    STMP03(x, c0, c1, c2, c3, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure STMP03 ( Stumpff functions 0 through 3 )
pub fn STMP03(
    X: f64,
    C0: &mut f64,
    C1: &mut f64,
    C2: &mut f64,
    C3: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut Y: f64 = 0.0;
    let mut Z: f64 = 0.0;

    //
    // SPICELIB Functions
    //

    //
    // Local Parameters
    //

    //
    // The integers NPAIRS, LPAIR2, and LPAIR3 are used to declare
    // space for Maclaurin series coefficients and for determining how
    // many terms of these series to use in the computation of
    // C_2 and C_3.
    //
    // Here's what is supposed to be true.
    //
    //    1/(TRUNC*2)!  + 1.0D0 = 1.0D0
    //
    // using this machine's double precision arithmetic.
    //
    // We will map the input X to a value y between -1 and 1 and then
    // construct the values of the functions at X from their values at y.
    // Since we will only evaluate the series expansion for C_2 and C_3
    // for values of y between -1 and 1, its easy to show that we don't
    // need to consider terms in the series whose coefficients have
    // magnitudes less than or equal 1/(2*TRUNC)! .
    //
    // If the value of TRUNC is 10, then the series expansions for
    // C_2(y) and C_3(y) are can be truncated as shown here:
    //
    //                               2             7       8
    //          .    1      y       y             y       y
    //   C_3(y) =   --- -  ---  +  ---  +  ... - ---  +  ---
    //               3!     5!      7!           17!     19!
    //
    //
    //             1        y         y            y           y
    //          = ---( 1 - --- ( 1 - --- (...( 1- ----- ( 1 - ----- )...)
    //            2*3      4*5       6*7          16*17       18*19
    //
    //
    //
    //
    //          .    1      y       y             y       y
    //   C_2(y) =   --- -  ---  +  ---  +  ... + ---  -  ---
    //               2!     4!      6!           16!     18!
    //
    //
    //             1        y         y            y           y
    //          = ---( 1 - --- ( 1 - --- (...( 1- ----- ( 1 - ----- )...)
    //            1*2      3*4       5*6          15*16       17*18
    //
    // As is evident from the above, we are going to need the
    // "reciprocal pairs"
    //
    //   1/(1*2),  1/(2*3),  1/(3*4), 1/(4*5), ...
    //
    // The number of such fractions be computed directly from
    // TRUNC. LPAIR3 and LPAIR2 indicate which of these pairs
    // (counting 1/(1*2) as the first) will be the last one needed in
    // the evaluation of C_2 and C_3.
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
    // We are going to need the numbers
    //
    //    1/(2*3), 1/(3*4), 1/(4*5), ...
    //
    // but we don't want to compute them every time this routine is
    // called.  So the first time this routine is called we compute
    // them and put them in the array PAIRS for use on subsequent
    // calls. (This could be done via parameters, but computing them
    // at run time seems to have a better chance of being
    // easily maintained.)
    //
    // In addition we will need to compute the lower bound for which
    // C_0,...,C_3 can be computed.  This lower bound is computed by
    // noting that C_0 has the largest magnitude of all the Stumpff
    // functions over the domain from -infinity to -1.  Moreover, in this
    // range
    //
    //    C_0(X) = Cosh( SQRT(-X) )
    //
    // Thus the range of X for which the Stumpff functions can be
    // computed is bounded below by the value of X for which
    //
    //    Cosh ( SQRT(-X) ) = DPMAX
    //
    // Which implies the lower bound for valid inputs is at
    //
    //    X = - ( DLOG ( 2.0 ) + DLOG( DPMAX ) ) ** 2
    //
    //      = - ( DLOG ( 2*N ) + DLOG ( DPMAX/N  ) ) ** 2
    //
    // We point out the second formulation of the bound just in case
    // your compiler can't handle the computation of DLOG ( DPMAX ).
    // If this unfortunate situation should arise, complain to the
    // company that produces your compiler and in the code below
    // compute LBOUND using the second form above with N equal to
    // some large power of 2 (say 2**20).
    //
    if save.FIRST {
        save.FIRST = false;

        for I in 1..=NPAIRS {
            save.PAIRS[I] = (1.0 / ((I as f64) * ((I + 1) as f64)));
        }

        Y = (f64::ln(2.0) + f64::ln(DPMAX()));
        save.LBOUND = -(Y * Y);
    }

    //
    // First we make sure that the input value of X is within the
    // range that we are confident we can use to compute the Stumpff
    // functions.
    //
    if (X <= save.LBOUND) {
        CHKIN(b"STMP03", ctx)?;
        SETMSG(
            b"The input value of X must be greater than #.  The input value was #",
            ctx,
        );
        ERRDP(b"#", save.LBOUND, ctx);
        ERRDP(b"#", X, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"STMP03", ctx)?;
        return Ok(());
    }

    //
    // From the definition of the Stumpff functions it can be seen that
    // C_0(X), C_1(X) are given by
    //
    //      COS ( DSQRT(X) )   and   SIN ( DSQRT(X) ) / DSQRT(X)
    //
    // for positive X. Moreover, the series used to define them converges
    // for all real X.
    //
    // These functions have a number of simple relationships that make
    // their computations practical.  Among these are:
    //
    //                     1
    //      x*C_k+2(x) =  ---  -  C_k(x)
    //                     k!
    //
    //
    //
    //                             2
    //       C_0(4x)   =  2*[ C_0(x) ]  -  1
    //
    //
    //
    //
    //       C_1(4x)   =    C_1(x)*C_0(x)
    //
    //
    //
    //                             2
    //       C_2(4x)   =   [C_1(x)]  / 2
    //
    //
    //
    //
    //       C_3(4x)   = [ C_2(x) + C_0(x)*C_3(x) ] / 4
    //
    //  These can be used to derive formulae for C_0(16x) ... C_3(16x)
    //  that involve only C_0(x) ... C_3(x).  If we let
    //
    //                                  2
    //                 Z        = C_0(x)  - 0.5
    //
    //  and
    //
    //                 W        = 2*C_0(x)*C_1(x)
    //
    //  then
    //
    //                               2
    //                 C_0(16x) = 8*Z  -  1
    //
    //
    //                 C_1(16x) = W*Z
    //
    //
    //                             2
    //                 C_2(16x) = W  / 8
    //
    //
    //                                   2
    //                             C_1(x)  + Z*[C_2(x) + C_0(x)*C_3(x)]
    //                 C_3(16x) =  ----------------------------------
    //                                              8
    //
    //
    if (X < -1.0) {
        Z = f64::sqrt(-X);
        *C0 = f64::cosh(Z);
        *C1 = (f64::sinh(Z) / Z);
        *C2 = (((1 as f64) - *C0) / X);
        *C3 = (((1 as f64) - *C1) / X);
        return Ok(());
    }

    if (X > 1.0) {
        Z = f64::sqrt(X);
        *C0 = f64::cos(Z);
        *C1 = (f64::sin(Z) / Z);
        *C2 = (((1 as f64) - *C0) / X);
        *C3 = (((1 as f64) - *C1) / X);
        return Ok(());
    }

    //
    // If the magnitude of X is less than or equal to 1, we compute
    // the function values directly from their power series
    // representations.
    //
    //
    // Compute C_3 of x :
    //
    //                               2             7       8
    //          .    1      x       x             x       x
    //   C_3(x) =   --- -  ---  +  ---  +  ... - ---  +  ---
    //               3!     5!      7!           17!     19!
    //
    //
    //             1        x         x            x           x
    //          = ---( 1 - --- ( 1 - --- (...( 1- ----- ( 1 - ----- )...)
    //            2*3      4*5       6*7          16*17       18*19
    //
    //             ^        ^         ^             ^           ^
    //             |        |         |             |           |
    //             |        |         |             |           |
    //          PAIR(2)  PAIR(4)   PAIR(6)  ...  PAIR(16)    PAIR(18)
    //
    // Assuming that we don't need to go beyond the term with 1/19!,
    // LPAIR3 will be 18.
    //

    *C3 = 1.0;

    for I in intrinsics::range(LPAIR3, 4, -2) {
        *C3 = (1.0 - ((X * save.PAIRS[I]) * *C3));
    }

    *C3 = (save.PAIRS[2] * *C3);

    //
    // Compute C_2 of x  :
    //
    //    Here's how we do it.
    //                               2             7       8
    //          .    1      x       x             x       x
    //   C_2(x) =   --- -  ---  +  ---  +  ... + ---  -  ---
    //               2!     4!      6!           16!     18!
    //
    //
    //             1        x         x            x           x
    //          = ---( 1 - --- ( 1 - --- (...( 1- ----- ( 1 - ----- )...)
    //            1*2      3*4       5*6          15*16       17*18
    //
    //             ^        ^         ^             ^           ^
    //             |        |         |             |           |
    //             |        |         |             |           |
    //          PAIR(1)  PAIR(3)   PAIR(5)  ...  PAIR(15)    PAIR(17)
    //
    // Assuming that we don't need to go beyond  the term with 1/18!,
    // LPAIR2 will be 17.
    //
    *C2 = 1.0;

    for I in intrinsics::range(LPAIR2, 3, -2) {
        *C2 = (1.0 - ((X * save.PAIRS[I]) * *C2));
    }

    *C2 = (save.PAIRS[1] * *C2);

    //
    // Get C1 and C0 via the recursion formula:
    //
    //                     1
    //      x*C_k+2(y) =  ---  -  C_k(x)
    //                     k!
    //
    *C1 = (1.0 - (X * *C3));
    *C0 = (1.0 - (X * *C2));

    Ok(())
}
