//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Fill a character array
///
/// Fill a character string array with a specified string.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  -------------------------------------------------
///  VALUE      I   Character string value to be placed in ARRAY.
///  NDIM       I   The number of elements in ARRAY.
///  ARRAY      O   Character string array which is to be filled.
/// ```
///
/// # Detailed Input
///
/// ```text
///  VALUE    is the value to be assigned to the array elements
///           1 through NDIM.
///
///  NDIM     is the number of elements in the array.
/// ```
///
/// # Detailed Output
///
/// ```text
///  ARRAY    is a character string array whose elements are to be
///           set to VALUE.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If NDIM < 1, the array is not modified.
/// ```
///
/// # Examples
///
/// ```text
///  Let  VALUE = '*'
///       NDIM  =  4
///
///  then the contents of ARRAY are:
///
///       ARRAY (1) = '*'
///       ARRAY (2) = '*'
///       ARRAY (3) = '*'
///       ARRAY (4) = '*'
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
pub fn fillc(value: &str, ndim: i32, array: CharArrayMut) {
    FILLC(value.as_bytes(), ndim, array);
}

//$Procedure FILLC ( Fill a character array )
pub fn FILLC(VALUE: &[u8], NDIM: i32, ARRAY: CharArrayMut) {
    let mut ARRAY = DummyCharArrayMut::new(ARRAY, None, 1..);

    //
    // Local variables
    //

    for I in 1..=NDIM {
        fstr::assign(ARRAY.get_mut(I), VALUE);
    }
}
