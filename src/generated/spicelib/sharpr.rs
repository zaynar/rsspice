//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Sharpen a rotation
///
/// Adjust the columns of a matrix that is "nearly" a rotation
/// so that they are numerically unit length and orthogonal,
/// going from left to right in the usual printed presentation
/// of a matrix.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ROT       I-O  The rotation matrix to be sharpened.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ROT      is a 3x3 matrix that is nearly a rotation matrix.
/// ```
///
/// # Detailed Output
///
/// ```text
///  ROT      is the input after sharpening the columns.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  This routine is not meant to be used on singular or near-
///      singular matrices (in other words, matrices with determinant
///      close to zero).
///
///      If the input matrix is singular, the output matrix may not
///      be a rotation matrix. In any case, the results should be
///      considered unreliable in this case.
///
///      No error handling is done for invalid input matrices.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine "sharpens" the orthogonality of a potential
///  rotation matrix. It is intended for use in those situations
///  in which you have a rotation matrix that may be derived
///  from single precision inputs or that may have experienced
///  round off errors in its construction.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that you have a rotation matrix that needs to be
///  converted to a quaternion. The SPICE matrix to quaternion
///  conversion routine M2Q performs error checks on the input
///  matrix and signals an error if it does not meet the checks
///  for a quaternion. By calling this routine you can ensure that
///  your rotation matrix (provided it's non-singular) will pass
///  the restrictions imposed by M2Q.
///
///      CALL SHARPR ( ROT )
///      CALL M2Q    ( ROT, Q )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  See the $Exceptions section above.
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
/// -    SPICELIB Version 1.1.1, 26-OCT-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section.
///
/// -    SPICELIB Version 1.1.0, 13-OCT-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in VHAT call. Some header updates were made.
///
/// -    SPICELIB Version 1.0.0, 16-SEP-1999 (WLT)
/// ```
pub fn sharpr(rot: &mut [[f64; 3]; 3]) {
    SHARPR(rot.as_flattened_mut());
}

//$Procedure SHARPR ( Sharpen a rotation )
pub fn SHARPR(ROT: &mut [f64]) {
    let mut ROT = DummyArrayMut2D::new(ROT, 1..=3, 1..=3);

    //
    // Unitize the first column of the rotation.
    //
    VHATIP(ROT.subarray_mut([1, 1]));

    //
    // Unitize the third column of the rotation and make it
    // orthogonal to the first two columns.
    //
    UCRSS(
        &ROT.subarray([1, 1]).to_vec(),
        &ROT.subarray([1, 2]).to_vec(),
        ROT.subarray_mut([1, 3]),
    );

    //
    // Unitize the second column of the rotation and make it
    // orthogonal to the first and third columns.
    //
    UCRSS(
        &ROT.subarray([1, 3]).to_vec(),
        &ROT.subarray([1, 1]).to_vec(),
        ROT.subarray_mut([1, 2]),
    );
}
