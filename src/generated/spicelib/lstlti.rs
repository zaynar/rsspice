//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Last integer element less than
///
/// Find the index of the largest array element less than
/// a given integer X in an array of non-decreasing integers.
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
///  X        is an integer value acting as an upper bound: the element
///           of ARRAY that is the greatest element less than X is to
///           be found.
///
///  N        is the total number of elements in ARRAY.
///
///  ARRAY    is an array of integers that forms a non-decreasing
///           sequence. The elements of array need not be distinct.
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
///  steps to compute the value of LSTLTI.
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
///  Suppose that you have an reasonably large ordered array of
///  integers, into which you want to insert a few more without
///  destroying the ordering.
///
///  Depending upon your application, it may be desirable to
///  not insert duplicates, to insert duplicates before
///  existing entries or to insert them after existing entries.
///
///  The code fragment below, illustrates an insertion scheme
///  that will insert duplicate items before existing items
///  and simultaneously update a second parallel array of
///  double precision numbers.
///
///        get the pair to insert
///
///        READ (*,*) KEY, VALUE
///
///        locate the place to insert the new KEY into the sorted
///        array of keys.
///
///        LOC = LSTLTI ( KEY, NKEYS, KEYS ) + 1
///
///        insert the key and its associated value into the
///        KEYS and  VALUES arrays at location LOC
///
///        CALL INSLAI ( KEY,   1, LOC, NKEYS, KEYS   )
///        CALL INSLAD ( VALUE, 1, LOC, NVALS, VALUES )
///
///  If at the READ statement the arrays KEYS and VALUES looked like:
///
///        KEYS     VALUES     NKEYS = 6, NVALS = 6
///        ----     -------
///          2       3.00D0
///          5       1.00D0
///          7       3.14D0
///         16       7.11D0
///         18       2.14D0
///         23      12.12D0
///
///  and 9 and 33.33D3 were read into KEY and VALUE respectively
///  then LSTLEI (KEY, NKEYS, KEYS ) would be 3 and LOC would be 4.
///  After the calls to the routines INSLAI and INSLAD we would have
///
///        KEYS     VALUES     NKEYS = 7, NVALS = 7
///        ----     -------
///          2       3.00D0
///          5       1.00D0
///          7       3.14D0
///          9      33.33D3     <===== inserted items.
///         16       7.11D0
///         18       2.14D0
///         23      12.12D0
///
///  If 7 and 33.33D3 were read into KEY and VALUE respectively
///  then again LSTLEI (KEY, NKEYS, KEYS ) would be 2 and LOC would
///  be 3. After the calls to the routines INSLAI and INSLAD we
///  would have:
///
///        KEYS     VALUES     NKEYS = 7, NVALS = 7
///        ----     -------
///          2       3.00D0
///          5       1.00D0
///          7      33.33D3     <===== inserted items.
///          7       3.14D0
///         16       7.11D0
///         18       2.14D0
///         23      12.12D0
///
///  If we replaced the line of code
///
///        LOC = LSTLTI ( KEY, NKEYS, KEYS ) + 1
///  by
///
///        LOC = LSTLEI ( KEY, NKEYS, KEYS ) + 1
///
///  we would obtain a routine that inserted duplicates before
///  existing entries. (LSTLEI is similar to LSTLTI except it finds
///  the last occurrence of an integer less than or equal to a value.)
///  Using 7 and 33.33D3 for KEY and VALUE again, the modified code
///  fragment would yield the results shown below.
///
///        KEYS     VALUES     NKEYS = 7, NVALS = 7
///        ----     -------
///          2       3.00D0
///          5       1.00D0
///          7       3.14D0
///          7      33.33D3     <===== inserted items.
///         16       7.11D0
///         18       2.14D0
///         23      12.12D0
///
///
///  Note: you should NOT use the code outlined above as the basis of
///  a sorting algorithm. The SPICELIB routines SHELLI, SHELLD, SHELLC,
///  ORDERI, ORDERD, ORDERC, REORDI, REORDD and REORDC are much more
///  efficient routines for sorting arrays or sorting a set of
///  parallel arrays using one of the set as a key. The fragment
///  presented here is useful for performing update insertions into
///  previously ordered arrays.
///
///  For more ideas regarding the use of this routine, see LSTLTC
///  and LSTLTC.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  If the sequence of integer numbers in the input array ARRAY is
///      not non-decreasing, the program will run to completion but the
///      index found will not mean anything.
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
pub fn lstlti(x: i32, n: i32, array: &[i32]) -> i32 {
    let ret = LSTLTI(x, n, array);
    ret
}

//$Procedure LSTLTI ( Last integer element less than )
pub fn LSTLTI(X: i32, N: i32, ARRAY: &[i32]) -> i32 {
    let ARRAY = DummyArray::new(ARRAY, 1..);
    let mut LSTLTI: i32 = 0;
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
        LSTLTI = 0;
    } else if (X <= ARRAY[BEGIN]) {
        //
        // None of the array elements are less than X
        //
        LSTLTI = 0;
    } else if (ARRAY[END] < X) {
        //
        // X is greater than all elements of the array.  Thus the last
        // element of the array is the last item less than X.
        //
        LSTLTI = END;
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

        LSTLTI = BEGIN;
    }

    LSTLTI
}
