//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Minimum element of array, DP
///
/// Locate the minimum element of a DP array.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  -------------------------------------------------
///  ARRAY      I   Array.
///  NDIM       I   Number of elements in ARRAY.
///  MINVAL     O   Minimum value in ARRAY.
///  LOC        O   Location of MINVAL in ARRAY.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ARRAY    is an arbitrary array.
///
///  NDIM     is the number of elements in ARRAY.
/// ```
///
/// # Detailed Output
///
/// ```text
///  MINVAL   is the value in array that is less than or equal
///           to all other values in the array. If the array
///           contains more than one element with this value,
///           the first one is returned.
///
///  LOC      is the location of the minimum element. That is,
///           MINVAL contains element ARRAY(LOC).
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If the array is empty (NDIM is less than one), LOC is zero,
///      and MINVAL is not changed.
/// ```
///
/// # Examples
///
/// ```text
///  Let array A contain the following elements.
///
///     A(1) = 16.D0
///     A(2) =  4.D0
///     A(3) = 32.D0
///     A(4) = 64.D0
///     A(5) =  2.D0
///     A(6) =  8.D0
///
///  Then following the call
///
///     CALL MINAD ( A, 6, MINVAL, LOC )
///
///  the values of MINVAL and LOC are 2.D0 and 5 respectively.
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
/// -    SPICELIB Version 1.1.0, 14-APR-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn minad(array: &[f64], ndim: i32, minval: &mut f64, loc: &mut i32) {
    MINAD(array, ndim, minval, loc);
}

//$Procedure MINAD  ( Minimum element of array, DP )
pub fn MINAD(ARRAY: &[f64], NDIM: i32, MINVAL: &mut f64, LOC: &mut i32) {
    let ARRAY = DummyArray::new(ARRAY, 1..);

    //
    // Local variables
    //

    if (NDIM <= 0) {
        *LOC = 0;
        return;
    }

    *MINVAL = ARRAY[1];
    *LOC = 1;

    for I in 2..=NDIM {
        if (ARRAY[I] < *MINVAL) {
            *MINVAL = ARRAY[I];
            *LOC = I;
        }
    }
}
