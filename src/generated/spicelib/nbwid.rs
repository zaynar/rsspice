//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Non-blank width of a character array
///
/// Determine the non-blank width of a character array---that is,
/// the largest value of LASTNB for any element in the array.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ARRAY      I   Input array.
///  NELT       I   Number of elements in the array.
///
///  The function returns the index of the rightmost non-blank
///  character in the entire array of strings ARRAY.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ARRAY    is the input array.
///
///  NELT     is the number of elements in the input array.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the index of the rightmost non-blank
///  character in the entire array. This is equivalent to the maximum
///  value of LASTNB for the array, but somewhat more efficient to
///  compute. If NELT is not greater than zero, NBWID is zero.
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
///  Find the last non-blank character in the first element of the
///  array. Search the rest of the elements, starting at the end of
///  each string and moving back just far enough to determine if the
///  current string is wider than any of the previous ones. (This
///  makes NBWID somewhat more efficient than LASTNB.)
///
///  If any of the strings is found to contain no trailing blanks,
///  NBWID is just the length of the individual elements of the array,
///  and the search is terminated immediately.
/// ```
///
/// # Examples
///
/// ```text
///  Let ARRAY contain the following strings.
///
///        ARRAY(1) = 'A string of medium length                      '
///        ARRAY(2) = 'A very long string, much longer than the rest  '
///        ARRAY(3) = 'Shorter                                        '
///        ARRAY(4) = 'Short                                          '
///
///  Then the value returned by
///
///        WIDEST = NBWID ( ARRAY, 4 )
///
///  is 45.
///
///  If the word 'rest' in the second element is changed to 'others',
///  the value returned is 47, and the search is terminated after the
///  second element.
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
/// -    SPICELIB Version 1.1.0, 08-APR-2021 (JDR)
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
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn nbwid(array: CharArray, nelt: i32) -> i32 {
    let ret = NBWID(array, nelt);
    ret
}

//$Procedure NBWID ( Non-blank width of a character array )
pub fn NBWID(ARRAY: CharArray, NELT: i32) -> i32 {
    let ARRAY = DummyCharArray::new(ARRAY, None, 1..);
    let mut NBWID: i32 = 0;
    let mut STRLEN: i32 = 0;
    let mut I: i32 = 0;
    let mut J: i32 = 0;

    //
    // Local variables
    //

    //
    // Nonsense case: no elements.
    //
    if (NELT < 1) {
        NBWID = 0;

    //
    // Get the length of the individual elements of the string.
    // So far, we have no maximum width, because we haven't examined
    // any elements.
    //
    } else {
        STRLEN = intrinsics::LEN(&ARRAY[1]);
        NBWID = 0;
        I = 0;

        //
        // Continue until the end of the array is reached, or until
        // a string with no trailing blanks is found.
        //
        while ((I < NELT) && (NBWID < STRLEN)) {
            //
            // Search no further than the current value of NBWID.
            //
            I = (I + 1);
            J = STRLEN;

            while ((J > NBWID) && fstr::eq(fstr::substr(ARRAY.get(I), J..=J), b" ")) {
                J = (J - 1);
            }

            //
            // NBWID only increases if this string was wider than all
            // previous strings.
            //
            NBWID = intrinsics::MAX0(&[NBWID, J]);
        }
    }

    NBWID
}
