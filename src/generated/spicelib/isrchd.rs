//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Search in a double precision array
///
/// Search for a given value within a double precision array. Return
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
///  ARRAY      I   Double precision array to search.
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
///  ARRAY    is the double precision array to be searched.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the index of the first matching array
///  element in ARRAY. If VALUE is not found, ISRCHD is zero.
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
///  The following table shows the value of ISRCHD given the contents
///  of ARRAY and VALUE:
///
///    ARRAY                         VALUE   ISRCHD
///  ---------------------------     -----   ------
///  1.0D0, 0.0D0, 4.0D0, 2.0D0      4.0D0     3
///  1.0D0, 0.0D0, 4.0D0, 2.0D0      2.OD0     4
///  1.0D0, 0.0D0, 4.0D0, 2.0D0      3.0D0     0
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  CAUTION must be exercised when comparing floating point
///      numbers for equality. If the numbers in ARRAY or the number in
///      VALUE are the result of computations, then it is likely that
///      strict equality between VALUE and some element of ARRAY will
///      NOT hold (even if the two numbers are very close) unless the
///      numbers are the result of exactly the same computations.
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
pub fn isrchd(value: f64, ndim: i32, array: &[f64]) -> i32 {
    let ret = ISRCHD(value, ndim, array);
    ret
}

//$Procedure ISRCHD  ( Search in a double precision array )
pub fn ISRCHD(VALUE: f64, NDIM: i32, ARRAY: &[f64]) -> i32 {
    let ARRAY = DummyArray::new(ARRAY, 1..);
    let mut ISRCHD: i32 = 0;

    //
    // Local variables
    //

    ISRCHD = 0;

    for I in 1..=NDIM {
        if (ARRAY[I] == VALUE) {
            ISRCHD = I;
            return ISRCHD;
        }
    }

    ISRCHD
}
