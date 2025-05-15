//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Is a number odd?
///
/// Determine whether an integer is odd.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  IVAL       I   The integer in question.
///
///  The function returns .TRUE. if IVAL is odd, otherwise .FALSE.
/// ```
///
/// # Detailed Input
///
/// ```text
///  IVAL     is the integer to be tested for oddness.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns .TRUE. if IVAL is odd, .FALSE. if IVAL is
///  even.
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
///  Divide IVAL by two. If the remainder is one, IVAL is odd.
/// ```
///
/// # Examples
///
/// ```text
///  Let ENDPTS contain a series of endpoints,
///
///     a , b , ..., a , b
///      1   1        n   n
///
///  representing an ordered collection of disjoint intervals,
///
///     a   <  b   < a
///      i  -   i     i+1
///
///  The following code fragment uses ODD to determine whether
///  an arbitrary value X is contained in any of the intervals.
///
///     CONTAINED = .FALSE.
///
///     DO I = 1, N-1
///        IF ( X .GE. ENDPTS(I)  .AND.  X .LE. ENDPTS(I+1) ) THEN
///           CONTAINED = ( ODD ( I ) )
///        END IF
///     END DO
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 08-APR-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Changed the input argument name "I" to "IVAL" for consistency
///         with other routines.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section.
///
/// -    SPICELIB Version 1.0.2, 07-NOV-2005 (BVS)
///
///         Fixed a few typos in the header.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn odd(ival: i32) -> bool {
    let ret = ODD(ival);
    ret
}

//$Procedure ODD ( Is a number odd? )
pub fn ODD(IVAL: i32) -> bool {
    let mut ODD: bool = false;

    //
    // Self-explanatory.
    //
    ODD = (intrinsics::MOD(IVAL, 2) != 0);

    ODD
}
