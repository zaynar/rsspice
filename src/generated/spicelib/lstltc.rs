//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Last character element less than
///
/// Find the index of the largest array element less than a given
/// character string in an ordered array of character strings.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  STRING     I   Upper bound value to search against.
///  N          I   Number of elements in ARRAY.
///  ARRAY      I   Array of possible lower bounds.
///
///  The function returns the index of the last element of ARRAY that
///  is lexically less than STRING.
/// ```
///
/// # Detailed Input
///
/// ```text
///  STRING   is a string acting as an upper bound: the element of
///           ARRAY that is lexically the greatest element less than
///           STRING is to be found. Trailing blanks in this bound
///           value are not significant.
///
///  N        is the total number of elements in ARRAY.
///
///  ARRAY    is an array of character strings to be searched. Trailing
///           blanks in the strings in this array are not significant.
///           The strings in ARRAY must be sorted in non-decreasing
///           order. The elements of ARRAY need not be distinct.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the index of the highest-indexed element in
///  the input array that is lexically less than STRING. The routine
///  assumes the array elements are sorted in non-decreasing order.
///
///  Indices range from 1 to N.
///
///  If all elements of ARRAY are lexically greater than or equal to
///  STRING, the routine returns the value 0. If N is less than or
///  equal to zero, the routine returns the value 0.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If N is less than or equal to zero, the function returns 0.
///      This case is not treated as an error.
///
///  2)  If the input array is not sorted in non-decreasing order, the
///      output of this routine is undefined. No error is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine uses a binary search algorithm and so requires
///  at most on the order of
///
///     log (N)
///        2
///
///  steps to compute the value of LSTLTC.
///
///  Note: If you need to find the first element of the array that is
///  lexically greater than or equal to STRING, simply add 1 to the
///  result returned by this function and check to see if the result is
///  within the array bounds given by N.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that you have a long list of words, sorted alphabetically
///  and entirely in upper case. Furthermore suppose you wished to
///  find all words that begin the sequence of letters PLA,  then
///  you could execute the following code.
///
///        START = 0
///        I     = 1
///
///        DO I = 1, NWORDS
///
///           IF ( WORD(I)(1:3) .EQ. 'PLA' ) THEN
///
///              IF ( START .EQ. 0 ) THEN
///                 START = I
///              END IF
///
///              END = I
///           END IF
///
///        END DO
///
///  This can of course be improved by stopping the loop once START
///  is non-zero and END remains unchanged after a pass through the
///  loop. However, this is a linear search  and on average can be
///  expected to take NWORDS/2 comparisons. The above algorithm
///  fails to take advantage of the structure of the list of words
///  (they are sorted).
///
///  The code below is much simpler to code, simpler to check, and
///  much faster than the code above.
///
///        START = LSTLTC( 'PLA', NWORDS, WORDS ) + 1
///        END   = LSTLTC( 'PLB', NWORDS, WORDS )
///
///        do something in case there are no such words.
///
///        IF ( START .GT. END ) THEN
///           START = 0
///           END   = 0
///        END IF
///
///  This code will never exceed 2 * LOG_2 ( NWORDS ) comparisons.
///  For a large list of words (say 4096) the second method will
///  take 24 comparisons  the first method requires on average
///  2048 comparisons. About 200 times as much time. Its clear
///  that if searches such as this must be performed often, that
///  the second approach could make the difference between being
///  able to perform the task in a few minutes as opposed to
///  several hours.
///
///  For more ideas regarding the use of this routine see LSTLEI
///  and LSTLTI.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  If the sequence of character strings in the input array ARRAY
///      is not non-decreasing, the program will run to completion but
///      the index found will not mean anything.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
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
///         unnecessary $Revisions section. Improved $Detailed_Input,
///         $Detailed_Output, $Particulars, $Exceptions and $Restrictions
///         sections.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (HAN)
/// ```
pub fn lstltc(string: &str, n: i32, array: CharArray) -> i32 {
    let ret = LSTLTC(string.as_bytes(), n, array);
    ret
}

//$Procedure LSTLTC ( Last character element less than )
pub fn LSTLTC(STRING: &[u8], N: i32, ARRAY: CharArray) -> i32 {
    let ARRAY = DummyCharArray::new(ARRAY, None, 1..);
    let mut LSTLTC: i32 = 0;
    let mut J: i32 = 0;
    let mut BEGIN: i32 = 0;
    let mut END: i32 = 0;
    let mut MIDDLE: i32 = 0;
    let mut ITEMS: i32 = 0;

    //
    // Local variables
    //

    ITEMS = N;

    BEGIN = 1;
    END = N;

    if (N <= 0) {
        //
        // There's nobody home---that is there is nothing in the array
        // to compare against.  Zero is the only sensible thing to return
        //
        LSTLTC = 0;
    } else if fstr::le(STRING, &ARRAY[BEGIN]) {
        //
        // None of the array elements are less than STRING
        //
        LSTLTC = 0;
    } else if fstr::lt(&ARRAY[END], STRING) {
        //
        // STRING is greater than all elements of the array.  Thus the las
        // element of the array is the last item less than STRING.
        //
        LSTLTC = END;
    } else {
        //
        // STRING lies between some pair of elements of the array
        //

        while (ITEMS > 2) {
            J = (ITEMS / 2);
            MIDDLE = (BEGIN + J);

            if fstr::lt(&ARRAY[MIDDLE], STRING) {
                BEGIN = MIDDLE;
            } else {
                END = MIDDLE;
            }

            ITEMS = (1 + (END - BEGIN));
        }

        LSTLTC = BEGIN;
    }

    LSTLTC
}
