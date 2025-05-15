//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Binary search with order vector, integer
///
/// Do a binary search for a given value within an integer array,
/// accompanied by an order vector. Return the index of the
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
///  ORDER      I   Order vector.
///
///  The function returns the index of the first matching array element
///  or zero if the value is not found.
/// ```
///
/// # Detailed Input
///
/// ```text
///  VALUE    is the value to be found in the input array.
///
///  NDIM     is the number of elements in the input array.
///
///  ARRAY    is the array to be searched.
///
///  ORDER    is an order vector which can be used to access the
///           elements of ARRAY in order. The contents of order are a
///           permutation of the sequence of integers ranging from 1 to
///           NDIM.
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
///  A binary search is performed on the input array, whose order is
///  given by an associated order vector. If an element of the array is
///  found to match the input value, the index of that element is
///  returned. If no matching element is found, zero is returned.
/// ```
///
/// # Examples
///
/// ```text
///  Let ARRAY and ORDER contain the following elements:
///
///        ARRAY         ORDER
///        -----------   -----
///          100             2
///            1             3
///           10             1
///        10000             5
///         1000             4
///
///  Then
///
///        BSCHOI (  1000, 5, ARRAY, ORDER )  = 5
///        BSCHOI (     1, 5, ARRAY, ORDER )  = 2
///        BSCHOI ( 10000, 5, ARRAY, ORDER )  = 4
///        BSCHOI (    -1, 5, ARRAY, ORDER )  = 0
///        BSCHOI (    17, 5, ARRAY, ORDER )  = 0
///
///  That is,
///
///        ARRAY(5) =  1000
///        ARRAY(2) =     1
///        ARRAY(4) = 10000
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  ORDER is assumed to give the order of the elements of ARRAY
///      in increasing order. If this condition is not met, the results
///      of BSCHOI are unpredictable.
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
/// -    SPICELIB Version 1.0.1, 09-APR-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 18-SEP-1995 (IMU) (WLT)
/// ```
pub fn bschoi(value: i32, ndim: i32, array: &[i32], order: &[i32]) -> i32 {
    let ret = BSCHOI(value, ndim, array, order);
    ret
}

//$Procedure BSCHOI ( Binary search with order vector, integer )
pub fn BSCHOI(VALUE: i32, NDIM: i32, ARRAY: &[i32], ORDER: &[i32]) -> i32 {
    let ARRAY = DummyArray::new(ARRAY, 1..);
    let ORDER = DummyArray::new(ORDER, 1..);
    let mut BSCHOI: i32 = 0;
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
        if (VALUE == ARRAY[ORDER[I]]) {
            BSCHOI = ORDER[I];
            return BSCHOI;

        //
        // Otherwise narrow the search area.
        //
        } else if (VALUE < ARRAY[ORDER[I]]) {
            RIGHT = (I - 1);
        } else {
            LEFT = (I + 1);
        }
    }

    //
    // If the search area is empty, return zero.
    //
    BSCHOI = 0;

    BSCHOI
}
