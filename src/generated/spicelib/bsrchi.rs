//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Binary search for an integer value
///
/// Do a binary search for a given value within an integer array,
/// assumed to be in nondecreasing order. Return the index of the
/// matching array entry, or zero if the key value is not found.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  VALUE      I   Value to find in ARRAY.
///  NDIM       I   Dimension of ARRAY.
///  ARRAY      I   Array to be searched.
///
///  The function returns the index of VALUE in ARRAY, or zero if not
///  found.
/// ```
///
/// # Detailed Input
///
/// ```text
///  VALUE    is the value to be found in the input array.
///
///  NDIM     is the number of elements in the input array.
///
///  ARRAY    is the integer array to be searched. The elements in
///           ARRAY are assumed to sorted in increasing order.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the index of the specified value in the input
///  array. Indices range from 1 to NDIM.
///
///  If the input array does not contain the specified value, the
///  function returns zero.
///
///  If the input array contains more than one occurrence of the
///  specified value, the returned index may point to any of the
///  occurrences.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If NDIM < 1, the value of the function is zero.
/// ```
///
/// # Particulars
///
/// ```text
///  A binary search is performed on the input array. If an element of
///  the array is found to match the input value, the index of that
///  element is returned. If no matching element is found, zero is
///  returned.
/// ```
///
/// # Examples
///
/// ```text
///  Let ARRAY contain the following elements:
///
///          -11
///            0
///           22
///          750
///
///  Then
///
///        BSRCHI ( -11, 4, ARRAY )    = 1
///        BSRCHI (  22, 4, ARRAY )    = 3
///        BSRCHI ( 751, 4, ARRAY )    = 0
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  ARRAY is assumed to be sorted in increasing order. If this
///      condition is not met, the results of BSRCHI are unpredictable.
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
/// -    SPICELIB Version 1.1.0, 26-OCT-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section. Improved $Detailed_Output
///         section.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn bsrchi(value: i32, ndim: i32, array: &[i32]) -> i32 {
    let ret = BSRCHI(value, ndim, array);
    ret
}

//$Procedure BSRCHI ( Binary search for an integer value )
pub fn BSRCHI(VALUE: i32, NDIM: i32, ARRAY: &[i32]) -> i32 {
    let ARRAY = DummyArray::new(ARRAY, 1..);
    let mut BSRCHI: i32 = 0;
    let mut LEFT: i32 = 0;
    let mut RIGHT: i32 = 0;
    let mut I: i32 = 0;

    //
    // Local variables
    //

    //
    // Set the initial bounds for the search area.
    //
    LEFT = 1;
    RIGHT = NDIM;

    while (LEFT <= RIGHT) {
        //
        // Check the middle element.
        //
        I = ((LEFT + RIGHT) / 2);

        //
        // If the middle element matches, return its location.
        //
        if (VALUE == ARRAY[I]) {
            BSRCHI = I;
            return BSRCHI;

        //
        // Otherwise narrow the search area.
        //
        } else if (VALUE < ARRAY[I]) {
            RIGHT = (I - 1);
        } else {
            LEFT = (I + 1);
        }
    }

    //
    // If the search area is empty, return zero.
    //
    BSRCHI = 0;

    BSRCHI
}
