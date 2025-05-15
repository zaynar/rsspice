//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Swap double precision values
///
/// Swap the contents of two double precision variables.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  A         I-O  First variable.
///  B         I-O  Second variable.
/// ```
///
/// # Detailed Input
///
/// ```text
///  A,
///  B        are two variables, the contents of which are to
///           be swapped (exchanged).
/// ```
///
/// # Detailed Output
///
/// ```text
///  A,
///  B        are the same two variables, after their contents
///           have been exchanged.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
/// ```
///
/// # Particulars
///
/// ```text
///  This is just shorthand notation for the code fragment
///
///        TEMP = A
///        A    = B
///        B    = TEMP
/// ```
///
/// # Examples
///
/// ```text
///  Let
///        A = 11.D0
///        B = 22.D0
///
///  Then after calling SWAPD (A,B),
///
///        A = 22.D0
///        B = 11.D0
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 02-JUN-2021 (JDR)
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
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn swapd(a: &mut f64, b: &mut f64) {
    SWAPD(a, b);
}

//$Procedure SWAPD ( Swap double precision values )
pub fn SWAPD(A: &mut f64, B: &mut f64) {
    let mut TEMP: f64 = 0.0;

    //
    // Local variables
    //

    //
    // What is there to say?
    //
    TEMP = *A;
    *A = *B;
    *B = TEMP;
}
