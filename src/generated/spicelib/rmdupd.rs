//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Remove duplicates from a double precision array
///
/// Remove duplicate elements from a double precision array.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NELT      I-O  Number of elements in the array.
///  ARRAY     I-O  Input/output array.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NELT     on input is the number of elements in the input
///           array.
///
///  ARRAY    on input contains zero or more elements, from which
///           all duplicate elements are to be removed.
/// ```
///
/// # Detailed Output
///
/// ```text
///  NELT     on output is the number of elements in the output
///           array.
///
///  ARRAY    on output contains the distinct elements of the
///           input array, sorted in increasing order. (Character
///           arrays are sorted according to the ASCII collating
///           sequence).
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
/// ```
///
/// # Examples
///
/// ```text
///  Let the arrays C and I contain the following elements.
///
///        NC   = 7                NI   =   5
///        C(1) = 'Miranda'        I(1) =  13
///        C(2) = 'Ariel'          I(2) = -13
///        C(3) = 'Umbriel'        I(3) =   0
///        C(4) = 'Titania'        I(4) =   1
///        C(5) = 'Miranda'        I(5) =   0
///        C(6) = 'Oberon'
///        C(7) = 'Umbriel'
///
///  Then following the calls
///
///        CALL RMDUPC ( NC, C )
///        CALL RMDUPI ( NI, I )
///
///  C and I contain the following.
///
///        NC   = 5                NI   =   4
///        C(1) = 'Ariel'          I(1) = -13
///        C(2) = 'Miranda'        I(2) =   0
///        C(3) = 'Oberon'         I(3) =   1
///        C(4) = 'Titania'        I(4) =  13
///        C(5) = 'Umbriel'
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
/// -    SPICELIB Version 1.1.0, 13-AUG-2021 (JDR)
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
pub fn rmdupd(nelt: &mut i32, array: &mut [f64]) {
    RMDUPD(nelt, array);
}

//$Procedure RMDUPD ( Remove duplicates from a double precision array )
pub fn RMDUPD(NELT: &mut i32, ARRAY: &mut [f64]) {
    let mut ARRAY = DummyArrayMut::new(ARRAY, 1..);
    let mut J: i32 = 0;

    //
    // Local variables
    //

    //
    // Proceed only if the array actually contains more than one element.
    //
    if (*NELT > 1) {
        //
        // Sort the array in place.
        //
        SHELLD(*NELT, ARRAY.as_slice_mut());

        //
        // Drop duplicate entries. Compare adjacent entries, and move
        // duplicates forward. (Duplicates are now adjacent, because of
        // sorting.)
        //
        J = 1;

        for I in 2..=*NELT {
            if (ARRAY[I] != ARRAY[(I - 1)]) {
                J = (J + 1);
                ARRAY[J] = ARRAY[I];
            }
        }

        *NELT = J;
    }
}
