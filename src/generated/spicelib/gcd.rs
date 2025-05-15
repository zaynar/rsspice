//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Greatest Common Divisor
///
/// Return the greatest common divisor of two integers.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  A          I   Any integer
///  B          I   Any integer
///
///  The function returns the greatest common divisor of A and B.
/// ```
///
/// # Detailed Input
///
/// ```text
///  A        is any integer.
///
///  B        is any integer.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the greatest common divisor of A and B.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If both A and B are zero, then GCD returns zero.
///
///  2)  If exactly one of A and B is zero, then GCD is by definition
///      the maximum of the absolute values of A and B.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine uses Euclid's Algorithm to find the greatest common
///  divisor (GCD) of the integers A and B. In other words the
///  largest integer, G, such that A = k*G for some k and B = j*G for
///  some G. Note if either A or B is zero, then we return the
///  maximum of the two integers ABS(A) and ABS(B).  If one is
///  non-zero we have just what the definition says. If both are zero
///  the definition above does not give us a GCD, so we take the GCD
///  of 0 and 0 to be 0.
/// ```
///
/// # Examples
///
/// ```text
///  A      B            GCD
///  -----  -----         -----
///    8      4             4
///   120    44             4
///   15    135            15
///  101     97             1
///  119    221            17
///  144     81             9
///    0    111           111
///    0      0             0
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  D. Knuth, "The Art of Computer Programming Vol 1. --
///       Fundamental Algorithms," 2nd Ed., Addison-Wesley, 1973
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
/// -    SPICELIB Version 1.1.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT)
/// ```
pub fn gcd(a: i32, b: i32) -> i32 {
    let ret = GCD(a, b);
    ret
}

//$Procedure GCD ( Greatest Common Divisor )
pub fn GCD(A: i32, B: i32) -> i32 {
    let mut GCD: i32 = 0;
    let mut REMNDR: i32 = 0;
    let mut ABSA: i32 = 0;
    let mut ABSB: i32 = 0;
    let mut P: i32 = 0;
    let mut Q: i32 = 0;

    //
    // Local variables
    //

    ABSA = i32::abs(A);
    ABSB = i32::abs(B);

    if (ABSA > ABSB) {
        P = ABSA;
        Q = ABSB;
    } else {
        P = ABSB;
        Q = ABSA;
    }

    REMNDR = 1;

    if (Q != 0) {
        while (REMNDR != 0) {
            GCD = Q;
            REMNDR = (P - ((P / Q) * Q));
            P = Q;
            Q = REMNDR;
        }
    } else {
        GCD = P;
    }

    GCD
}
