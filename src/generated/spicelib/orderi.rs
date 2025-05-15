//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Order of an integer array
///
/// Determine the order of elements in an integer array.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ARRAY      I    Input array.
///  NDIM       I    Dimension of ARRAY.
///  IORDER     O    Order vector for ARRAY.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ARRAY    is the input array.
///
///  NDIM     is the number of elements in the input array.
/// ```
///
/// # Detailed Output
///
/// ```text
///  IORDER   is the order vector for the input array.
///           IORDER(1) is the index of the smallest element
///           of ARRAY; IORDER(2) is the index of the next
///           smallest; and so on.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  A negative input dimension causes this routine to leave the
///      output order vector unchanged.
/// ```
///
/// # Particulars
///
/// ```text
///  ORDERI finds the index of the smallest element of the input
///  array. This becomes the first element of the order vector.
///  The process is repeated for the rest of the elements.
///
///  The order vector returned by ORDERI may be used by any of
///  the REORD routines to sort sets of related arrays, as shown
///  in the example below.
/// ```
///
/// # Examples
///
/// ```text
///  In the following example, the ORDER and REORD routines are
///  used to sort four related arrays (containing the names,
///  masses, integer ID codes, and visual magnitudes for a group
///  of satellites). This is representative of the typical use of
///  these routines.
///
///     C
///     C     Sort the object arrays by ID code.
///     C
///           CALL ORDERI ( CODES,  N, IORDER )
///
///           CALL REORDC ( IORDER, N, NAMES  )
///           CALL REORDD ( IORDER, N, MASSES )
///           CALL REORDI ( IORDER, N, CODES  )
///           CALL REORDR ( IORDER, N, VMAGS  )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 04-JUL-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.2, 23-MAR-2010 (NJB)
///
///         Header example was updated to show use of this routine.
///         $Exceptions section was updated. Header sections were
///         re-ordered.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn orderi(array: &[i32], ndim: i32, iorder: &mut [i32]) {
    ORDERI(array, ndim, iorder);
}

//$Procedure ORDERI ( Order of an integer array )
pub fn ORDERI(ARRAY: &[i32], NDIM: i32, IORDER: &mut [i32]) {
    let ARRAY = DummyArray::new(ARRAY, 1..);
    let mut IORDER = DummyArrayMut::new(IORDER, 1..);
    let mut GAP: i32 = 0;
    let mut J: i32 = 0;
    let mut JG: i32 = 0;

    //
    // Local variables
    //

    //
    // Begin with the initial ordering.
    //
    for I in 1..=NDIM {
        IORDER[I] = I;
    }

    //
    // Find the smallest element, then the next smallest, and so on.
    // This uses the Shell Sort algorithm, but swaps the elements of
    // the order vector instead of the array itself.
    //
    GAP = (NDIM / 2);

    while (GAP > 0) {
        for I in (GAP + 1)..=NDIM {
            J = (I - GAP);
            while (J > 0) {
                JG = (J + GAP);

                if (ARRAY[IORDER[J]] <= ARRAY[IORDER[JG]]) {
                    J = 0;
                } else {
                    SWAPI_ARRAY(
                        IORDER.subscript(J),
                        IORDER.subscript(JG),
                        IORDER.as_slice_mut(),
                    );
                }

                J = (J - GAP);
            }
        }

        GAP = (GAP / 2);
    }
}
