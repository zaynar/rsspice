//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Binary search for a character string
///
/// Do a binary search for a given value within a character array,
/// assumed to be in nondecreasing order. Return the index of the
/// matching array entry, or zero if the key value is not found.
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
///  The function returns the index of the first matching array element
///  or zero if the value is not found.
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
///  NDIM     is the number of elements in the input array.
///
///  ARRAY    is the array of character strings to be searched.
///           Trailing blanks in the strings in this array are not
///           significant. The elements in ARRAY are assumed to
///           sorted according to the ASCII collating sequence.
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
///  1)  If NDIM < 1, the value of the function is zero. This is not
///      considered an error.
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
///        'BOHR'
///        'EINSTEIN'
///        'FEYNMAN'
///        'GALILEO'
///        'NEWTON'
///
///  Then
///
///        BSRCHC ( 'NEWTON',   5, ARRAY )    = 5
///        BSRCHC ( 'EINSTEIN', 5, ARRAY )    = 2
///        BSRCHC ( 'GALILEO',  5, ARRAY )    = 4
///        BSRCHC ( 'Galileo',  5, ARRAY )    = 0
///        BSRCHC ( 'BETHE',    5, ARRAY )    = 0
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  ARRAY is assumed to be sorted in increasing order according to
///      the ASCII collating sequence. If this condition is not met,
///      the results of BSRCHC are unpredictable.
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
///         unnecessary $Revisions section. Improved $Detailed_Input and
///         $Detailed_Output section.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn bsrchc(value: &str, ndim: i32, array: CharArray) -> i32 {
    let ret = BSRCHC(value.as_bytes(), ndim, array);
    ret
}

//$Procedure BSRCHC ( Binary search for a character string )
pub fn BSRCHC(VALUE: &[u8], NDIM: i32, ARRAY: CharArray) -> i32 {
    let ARRAY = DummyCharArray::new(ARRAY, None, 1..);
    let mut BSRCHC: i32 = 0;
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
        if fstr::eq(VALUE, ARRAY.get(I)) {
            BSRCHC = I;
            return BSRCHC;

        //
        // Otherwise narrow the search area.
        //
        } else if fstr::lt(VALUE, &ARRAY[I]) {
            RIGHT = (I - 1);
        } else {
            LEFT = (I + 1);
        }
    }

    //
    // If the search area is empty, return zero.
    //
    BSRCHC = 0;

    BSRCHC
}
