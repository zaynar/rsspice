//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Shell sort a double precision array
///
/// Sort a double precision array using the Shell Sort algorithm.
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
///        99.D0
///        33.D0
///        55.D0
///        44.D0
///       -77.D0
///        66.D0
///
///  Then after a call to SHELLD, the array would be ordered as
///  follows:
///
///       -77.D0
///        33.D0
///        44.D0
///        55.D0
///        66.D0
///        99.D0
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
pub fn shelld(ndim: i32, array: &mut [f64]) {
    SHELLD(ndim, array);
}

//$Procedure SHELLD ( Shell sort a double precision array )
pub fn SHELLD(NDIM: i32, ARRAY: &mut [f64]) {
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
                    SWAPD_ARRAY(
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
