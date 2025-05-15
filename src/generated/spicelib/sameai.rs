//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Are two integer arrays the same?
///
/// Indicate whether two integer arrays are equal.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  A1         I    First array to be compared.
///  A2         I    Second array to be compared.
///  NDIM       I    Dimension of A1 and A2.
///
///  The function returns the value .TRUE. if and only if A1 = A2.
/// ```
///
/// # Detailed Input
///
/// ```text
///  A1,
///  A2       are two integer arrays to be compared.  A1 and
///           A2 must have the same dimension.
///
///  NDIM     is the common dimension of A1 and A2.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function takes the value .TRUE. if and only if A1 equals A2.
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
///  This function can be thought of as a macro. It replaces the
///  loop
///
///     SAME  = .TRUE.
///     I     =  1
///
///     DO WHILE (  ( I .LE. NDIM )  .AND.  SAME  )
///
///        IF ( A1(I) .NE.  A2(I)  )
///           SAME  = .FALSE.
///        ELSE
///           I     =  I + 1
///        END IF
///
///     END DO
/// ```
///
/// # Examples
///
/// ```text
///  1)  Test two integer arrays A1 and A2 for equality, where both
///      arrays have declared length 10:
///
///         SAME  =  SAMEAI ( A1, A2, 10 )
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
/// -    SPICELIB Version 1.1.0, 12-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 19-DEC-1995 (NJB)
/// ```
pub fn sameai(a1: &[i32], a2: &[i32], ndim: i32) -> bool {
    let ret = SAMEAI(a1, a2, ndim);
    ret
}

//$Procedure SAMEAI ( Are two integer arrays the same? )
pub fn SAMEAI(A1: &[i32], A2: &[i32], NDIM: i32) -> bool {
    let A1 = DummyArray::new(A1, 1..);
    let A2 = DummyArray::new(A2, 1..);
    let mut SAMEAI: bool = false;

    //
    // Local variables
    //

    //
    // Executable code
    //
    SAMEAI = true;

    for I in 1..=NDIM {
        if (A1[I] != A2[I]) {
            SAMEAI = false;
            return SAMEAI;
        }
    }

    SAMEAI
}
