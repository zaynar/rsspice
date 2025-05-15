//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Closest integer array element
///
/// Find the index of the array element closest to a given integer X
/// in an array of non-decreasing integers.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  X          I   Search value.
///  N          I   Number of elements in ARRAY.
///  ARRAY      I   Array to be searched.
///
///  The function returns the index of the element of ARRAY
///  whose value is closest to X.
/// ```
///
/// # Detailed Input
///
/// ```text
///  X        is the value to be compared with the elements of ARRAY.
///
///  N        is the number of elements in ARRAY.
///
///  ARRAY    is an array of integers such that
///
///                      ARRAY( I ) <= ARRAY( J )
///
///           for all I < J.
/// ```
///
/// # Detailed Output
///
/// ```text
///  LSTCLI   is the index of the element of the non-decreasing
///           sequence: {ARRAY(I) : 1 <= I <= N} that is closest to
///           X. In other words, ARRAY( LSTCLI( X, N, ARRAY ) ) is the
///           closest element of ARRAY to X.
///
///           If X falls precisely on the midpoint of consecutive array
///           elements, the index of the larger of the two values is
///           returned.
///
///           If X is closest to a value which appears more than
///           once in the array (since the array is ordered, these
///           elements would have to be consecutive), the highest index
///           for that value will be returned.
///
///           LSTCLI = I for some I in the range 1 to N, unless N is
///           less than or equal to zero, in which case LSTCLI is zero.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If the value of N is non-positive, LSTCLI returns the value
///      zero.
/// ```
///
/// # Particulars
///
/// ```text
///  LSTCLI uses a binary search algorithm to locate the value closest
///  to X in the non-decreasing sequence of integers represented by
///  the elements of ARRAY.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose ARRAY contains the following integer elements:
///
///  ARRAY: -1    0    10  15    15    20    30   39  40  10
///
///  index:  1    2    3    4     5     6     7    8   9   10
///
///  The following table shows the values of LSTCLI that would be
///  returned for various values of X, and the corresponding closest
///  array element values.
///
///         X      LSTCLI( X,10,ARRAY )   ARRAY( LSTCLI( X,10,ARRAY ))
///       -----    --------------------   ---------------------------
///        -2               1                         -1
///        -1               1                         -1
///         1               2                          0
///        14               5                         15
///        17               5                         15
///        18               6                         20
///        60               9                         40
///       110              10                        100
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  If the sequence is not non-decreasing, the routine will run
///      to completion but the index found will not mean anything.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  R.E. Thurman       (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 26-OCT-2021 (JDR)
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
/// -    SPICELIB Version 1.0.0, 07-SEP-1990 (RET)
/// ```
pub fn lstcli(x: i32, n: i32, array: &[i32]) -> i32 {
    let ret = LSTCLI(x, n, array);
    ret
}

//$Procedure LSTCLI ( Closest integer array element )
pub fn LSTCLI(X: i32, N: i32, ARRAY: &[i32]) -> i32 {
    let ARRAY = DummyArray::new(ARRAY, 1..);
    let mut LSTCLI: i32 = 0;
    let mut J: i32 = 0;
    let mut BEGIN: i32 = 0;
    let mut END: i32 = 0;
    let mut MIDDLE: i32 = 0;
    let mut ITEMS: i32 = 0;

    //
    // Local variables
    //

    //
    // Save the size of the array and point to the beginning and ending
    // positions. The pointers delimit the current search interval.
    //
    ITEMS = N;
    BEGIN = 1;
    END = N;

    if (N <= 0) {
        //
        // There is nothing in the array to compare against. Zero is the
        // only sensible thing to return.
        //
        LSTCLI = 0;
        return LSTCLI;
    } else if (X <= ARRAY[BEGIN]) {
        //
        // All elements of the array are at least as big as X. So the
        // first element is the closest to X.
        //
        LSTCLI = 1;
    } else if (ARRAY[END] <= X) {
        //
        // X is at least as big as all elements of the array.  So the last
        // element is the closest to X.
        //
        LSTCLI = END;
    } else {
        //
        // X lies between some pair of elements of the array.
        //
        while (ITEMS > 2) {
            J = (ITEMS / 2);
            MIDDLE = (BEGIN + J);

            if (ARRAY[MIDDLE] <= X) {
                BEGIN = MIDDLE;
            } else {
                END = MIDDLE;
            }

            ITEMS = (1 + (END - BEGIN));
        }

        //
        // Which of the two is closest?
        //
        if ((X - ARRAY[BEGIN]) < (ARRAY[END] - X)) {
            LSTCLI = BEGIN;
        } else {
            LSTCLI = END;
        }
    }

    //
    // March down the array to find the last element equal to the
    // closet value.
    //
    while ((LSTCLI < N) && (ARRAY[LSTCLI] == ARRAY[(LSTCLI + 1)])) {
        LSTCLI = (LSTCLI + 1);
    }

    LSTCLI
}
