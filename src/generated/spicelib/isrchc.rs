//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Search in a character array
///
/// Search for a given value within a character string array. Return
/// the index of the first matching array entry, or zero if the key
/// value was not found.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  VALUE      I   Key value to be found in ARRAY.
///  NDIM       I   Dimension of ARRAY.
///  ARRAY      I   Character string array to search.
///
///  The function returns the index of the first matching array
///  element or zero if the value is not found.
/// ```
///
/// # Detailed Input
///
/// ```text
///  VALUE    is the key value to be found in the array. Trailing
///           blanks in this key are not significant: string matches
///           found by this routine do not require trailing blanks in
///           value to match those in the corresponding element of
///           array.
///
///  NDIM     is the dimension of the array.
///
///  ARRAY    is the character array to be searched. Trailing
///           blanks in the strings in this array are not significant.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the index of the first matching array
///  element in ARRAY. If VALUE is not found, ISRCHC is zero.
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
///  The following table shows the value of ISRCHC given the contents
///  of ARRAY and VALUE:
///
///    ARRAY                 VALUE     ISRCHC
///  -----------------       -----     ------
///  '1', '0', '4', '2'       '4'        3
///  '1', '0', '4', '2'       '2'        4
///  '1', '0', '4', '2'       '3'        0
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
/// -    SPICELIB Version 1.1.0, 03-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Extended
///         description of input arguments.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WMO)
/// ```
pub fn isrchc(value: &str, ndim: i32, array: CharArray) -> i32 {
    let ret = ISRCHC(value.as_bytes(), ndim, array);
    ret
}

//$Procedure ISRCHC  ( Search in a character array )
pub fn ISRCHC(VALUE: &[u8], NDIM: i32, ARRAY: CharArray) -> i32 {
    let ARRAY = DummyCharArray::new(ARRAY, None, 1..);
    let mut ISRCHC: i32 = 0;

    //
    // Local variables
    //

    ISRCHC = 0;

    for I in 1..=NDIM {
        if fstr::eq(ARRAY.get(I), VALUE) {
            ISRCHC = I;
            return ISRCHC;
        }
    }

    ISRCHC
}
