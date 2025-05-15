//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Shell sort an integer array
///
/// Sort an integer array using the Shell Sort algorithm.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NDIM       I   Dimension of the array.
///  ARRAY     I-O  The array.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NDIM     is the number of elements in the array to be sorted.
///
///  ARRAY    on input, is the array to be sorted.
/// ```
///
/// # Detailed Output
///
/// ```text
///  ARRAY    on output, contains the same elements, sorted
///           in increasing order. The actual sorting is done
///           in place in ARRAY.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If NDIM < 2, this routine does not modify the array.
/// ```
///
/// # Particulars
///
/// ```text
///  The Shell Sort Algorithm is well known.
/// ```
///
/// # Examples
///
/// ```text
///  Let ARRAY contain the following elements:
///
///        99
///        33
///        55
///        44
///       -77
///        66
///
///  Then after a call to SHELLI, the array would be ordered as
///  follows:
///
///       -77
///        33
///        44
///        55
///        66
///        99
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
/// -    SPICELIB Version 1.1.0, 02-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
///         Added entry #1 in $Exceptions section.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn shelli(ndim: i32, array: &mut [i32]) {
    SHELLI(ndim, array);
}

//$Procedure SHELLI ( Shell sort an integer array )
pub fn SHELLI(NDIM: i32, ARRAY: &mut [i32]) {
    let mut ARRAY = DummyArrayMut::new(ARRAY, 1..);
    let mut GAP: i32 = 0;
    let mut J: i32 = 0;
    let mut JG: i32 = 0;

    //
    // Local variables
    //

    //
    // This is a straightforward implementation of the Shell Sort
    // algorithm.
    //
    GAP = (NDIM / 2);

    while (GAP > 0) {
        for I in (GAP + 1)..=NDIM {
            J = (I - GAP);
            while (J > 0) {
                JG = (J + GAP);

                if (ARRAY[J] <= ARRAY[JG]) {
                    J = 0;
                } else {
                    SWAPI_ARRAY(
                        ARRAY.subscript(J),
                        ARRAY.subscript(JG),
                        ARRAY.as_slice_mut(),
                    );
                }

                J = (J - GAP);
            }
        }

        GAP = (GAP / 2);
    }
}
