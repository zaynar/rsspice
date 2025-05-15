//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Move a logical array to another
///
/// Copy the elements of one logical array into another
/// array.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ARRFRM     I   Logical array to be moved.
///  NDIM       I   Number of elements to copy, i.e. the dimension
///                 of ARRFRM and ARRTO.
///  ARRTO      O   Destination array.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ARRFRM   is an array from which to copy items.
///
///  NDIM     is the number of items to copy.
/// ```
///
/// # Detailed Output
///
/// ```text
///  ARRTO    is the array to which items should be copied.
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
///  This routine is simply shorthand for the following 3 lines of
///  code.
///
///          DO I = 1, NDIM
///             ARRTO(I) = ARRFRM(I)
///          END DO
/// ```
///
/// # Examples
///
/// ```text
///  Often one needs to make a temporary copy of an array so that
///  it can be manipulated without altering the original array.
///  As pointed out in particulars, you could just do this within
///  the code that needs the copy. However, if you have several
///  arrays to copy, you can cut the number of lines of code that
///  are needed by a third.
///
///  For example:
///
///       DO I = 1, 19
///          TEMPA(I) = A(I)
///       END DO
///
///       DO I = 1, 38
///          TEMPB(I) = B(I)
///       END DO
///
///  Can be rewritten as
///
///       CALL MOVEI ( A, 19, TEMPA )
///       CALL MOVEI ( B, 38, TEMPB )
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.M. Owen          (JPL)
///  W.L. Taber         (JPL)
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
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WMO)
/// ```
///
/// # Revisions
///
/// ```text
/// -     Beta Version 1.0.1, 4-FEB-1989 (WLT)
///
///       Header fully filled out.
/// ```
pub fn movel(arrfrm: &[bool], ndim: i32, arrto: &mut [bool]) {
    MOVEL(arrfrm, ndim, arrto);
}

//$Procedure MOVEL  ( Move a logical array to another )
pub fn MOVEL(ARRFRM: &[bool], NDIM: i32, ARRTO: &mut [bool]) {
    let ARRFRM = DummyArray::new(ARRFRM, 1..);
    let mut ARRTO = DummyArrayMut::new(ARRTO, 1..);

    //
    // Local variables
    //

    for I in 1..=NDIM {
        ARRTO[I] = ARRFRM[I];
    }
}
