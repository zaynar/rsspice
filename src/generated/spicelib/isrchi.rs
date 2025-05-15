//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Search in an integer array
///
/// Search for a given value within a integer array. Return
/// the index of the first matching array entry, or zero if
/// the key value was not found.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  VALUE      I   Key value to be found in ARRAY.
///  NDIM       I   Dimension of ARRAY.
///  ARRAY      I   Integer array to search.
///
///  The function returns the index of the first matching array
///  element or zero if the value is not found.
/// ```
///
/// # Detailed Input
///
/// ```text
///  VALUE    is the key value to be found in the array.
///
///  NDIM     is the dimension of the array.
///
///  ARRAY    is the integer array to be searched.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the index of the first matching array
///  element in ARRAY. If VALUE is not found, ISRCHI is zero.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If NDIM < 1, the function value is zero.
/// ```
///
/// # Examples
///
/// ```text
///  The following table shows the value of ISRCHI given the contents
///  of ARRAY and VALUE:
///
///    ARRAY        VALUE   ISRCHI
///  ----------     -----   ------
///  1, 0, 4, 2       4        3
///  1, 0, 4, 2       2        4
///  1, 0, 4, 2       3        0
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
/// -    SPICELIB Version 1.1.0, 03-JUL-2021 (JDR)
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
pub fn isrchi(value: i32, ndim: i32, array: &[i32]) -> i32 {
    let ret = ISRCHI(value, ndim, array);
    ret
}

//$Procedure ISRCHI  ( Search in an integer array )
pub fn ISRCHI(VALUE: i32, NDIM: i32, ARRAY: &[i32]) -> i32 {
    let ARRAY = DummyArray::new(ARRAY, 1..);
    let mut ISRCHI: i32 = 0;

    //
    // Local variables
    //

    ISRCHI = 0;

    for I in 1..=NDIM {
        if (ARRAY[I] == VALUE) {
            ISRCHI = I;
            return ISRCHI;
        }
    }

    ISRCHI
}
