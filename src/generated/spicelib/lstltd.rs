//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Last double precision element less than
///
/// Find the index of the largest array element less than
/// a given number X in an array of non-decreasing numbers.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  X          I   Upper bound value to search against.
///  N          I   Number of elements in ARRAY.
///  ARRAY      I   Array of possible lower bounds.
///
///  The function returns the index of the last element of ARRAY that
///  is less than X.
/// ```
///
/// # Detailed Input
///
/// ```text
///  X        is a double precision value acting as an upper bound: the
///           element of ARRAY that is the greatest element less than X
///           is to be found.
///
///  N        is the total number of elements in ARRAY.
///
///  ARRAY    is an array of double precision numbers that forms a
///           non-decreasing sequence. The elements of array need not
///           be distinct.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the index of the highest-indexed element in
///  the input array that is less than X. The routine assumes the array
///  elements are sorted in non-decreasing order.
///
///  Indices range from 1 to N.
///
///  If all elements of ARRAY are greater than or equal to X, the
///  routine returns the value 0. If N is less than or equal to zero,
///  the routine returns the value 0.
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
///  steps to compute the value of LSTLTD.
///
///  Note: If you need to find the first element of the array that is
///  greater than or equal to X, simply add 1 to the result returned by
///  this function and check to see if the result is within the array
///  bounds given by N.
/// ```
///
/// # Examples
///
/// ```text
///  If ARRAY(I) = -1 + 4*I/3 (real arithmetic implied here)
///
///     N        = 10
///     X        = 7.12
///
///  then
///
///     LSTLTD will be I where
///
///          (4*I/3) - 1       <      7.12
///
///  but
///
///          (4*(I+1)/3) - 1   > or = 7.12 .
///
///  In this case our subsequence is:
///
///         1/3, 5/3, 9/3, 13/3, 17/3, 21/3, 25/3, .... 37/3
///
///  index:  1    2    3    4     5     6     7    ....  10
///
///  Thus LSTLTD will be returned as 6
///
///  The following table shows the values of LSTLTD that would be
///  returned for various values of X
///
///         X       LSTLTD
///       -----     -------
///        0.12        0
///        1.34        1
///        5.13        4
///        8.00        6
///       15.10       10
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  If the sequence of double precision numbers in the input array
///      ARRAY is not non-decreasing, the program will run to
///      completion but the index found will not mean anything.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
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
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (NJB)
/// ```
pub fn lstltd(x: f64, n: i32, array: &[f64]) -> i32 {
    let ret = LSTLTD(x, n, array);
    ret
}

//$Procedure LSTLTD ( Last double precision element less than )
pub fn LSTLTD(X: f64, N: i32, ARRAY: &[f64]) -> i32 {
    let ARRAY = DummyArray::new(ARRAY, 1..);
    let mut LSTLTD: i32 = 0;
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
        LSTLTD = 0;
    } else if (X <= ARRAY[BEGIN]) {
        //
        // None of the array elements are less than X
        //
        LSTLTD = 0;
    } else if (ARRAY[END] < X) {
        //
        // X is greater than all elements of the array.  Thus the last
        // element of the array is the last item less than X.
        //
        LSTLTD = END;
    } else {
        //
        // X lies between some pair of elements of the array
        //

        while (ITEMS > 2) {
            J = (ITEMS / 2);
            MIDDLE = (BEGIN + J);

            if (ARRAY[MIDDLE] < X) {
                BEGIN = MIDDLE;
            } else {
                END = MIDDLE;
            }

            ITEMS = (1 + (END - BEGIN));
        }

        LSTLTD = BEGIN;
    }

    LSTLTD
}
