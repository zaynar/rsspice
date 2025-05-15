//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// DSK, compute outward normal of plate
///
/// Compute an outward normal vector of a triangular plate.
/// The vector does not necessarily have unit length.
///
/// # Required Reading
///
/// * [DSK](crate::required_reading::dsk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  V1,
///  V2,
///  V3         I   Vertices of a plate.
///  NORMAL     O   Plate's outward normal vector.
/// ```
///
/// # Detailed Input
///
/// ```text
///  V1,
///  V2,
///  V3       are vertices of a triangular plate.
/// ```
///
/// # Detailed Output
///
/// ```text
///  NORMAL   is an outward normal vector of the plate defined by
///           the input vertices. The order of the vertices is
///           used to determine the choice of normal direction:
///           the normal vector is
///
///              ( V2 - V1 ) x ( V3 - V2 )
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  The input plate may be degenerate: it may be a line segment
///      or a point. These are not considered to be erroneous inputs.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine saves computation time by not scaling the output
///  vector to unit length. The caller can scale the vector using
///  the routine VHAT.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as input
///  (if any), the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Compute an upward normal of an equilateral triangle lying
///     in the X-Y plane and centered at the origin.
///
///     Example code begins here.
///
///
///           PROGRAM PLTNRM_EX1
///           IMPLICIT NONE
///
///           DOUBLE PRECISION      NORMAL ( 3 )
///           DOUBLE PRECISION      S
///           DOUBLE PRECISION      V1     ( 3 )
///           DOUBLE PRECISION      V2     ( 3 )
///           DOUBLE PRECISION      V3     ( 3 )
///
///
///           S = SQRT(3.D0)/2
///
///           CALL VPACK (    S,  -0.5D0,  0.D0, V1 )
///           CALL VPACK ( 0.D0,    1.D0,  0.D0, V2 )
///           CALL VPACK (   -S,  -0.5D0,  0.D0, V3 )
///
///
///           CALL PLTNRM ( V1, V2, V3, NORMAL )
///
///           WRITE (*, '(A,3F18.14)' ) 'NORMAL = ', NORMAL
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     NORMAL =   0.00000000000000  0.00000000000000  2.59807621135332
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 08-JUL-2020 (JDR)
///
///         Edited the header to comply with NAIF standard..
///         Changed code example output format for the solution to fit
///         within the $Examples section without modifications.
///
/// -    SPICELIB Version 1.0.0, 26-JAN-2016 (NJB)
/// ```
pub fn pltnrm(v1: &[f64; 3], v2: &[f64; 3], v3: &[f64; 3], normal: &mut [f64; 3]) {
    PLTNRM(v1, v2, v3, normal);
}

//$Procedure PLTNRM ( DSK, compute outward normal of plate )
pub fn PLTNRM(V1: &[f64], V2: &[f64], V3: &[f64], NORMAL: &mut [f64]) {
    let V1 = DummyArray::new(V1, 1..=3);
    let V2 = DummyArray::new(V2, 1..=3);
    let V3 = DummyArray::new(V3, 1..=3);
    let mut NORMAL = DummyArrayMut::new(NORMAL, 1..=3);
    let mut EDGE1 = StackArray::<f64, 3>::new(1..=3);
    let mut EDGE2 = StackArray::<f64, 3>::new(1..=3);

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // This routine is error-free.
    //

    //
    // Type 2 plate vertices are ordered in the positive
    // (right-handed) sense about the outward normal.
    //
    VSUB(V2.as_slice(), V1.as_slice(), EDGE1.as_slice_mut());
    VSUB(V3.as_slice(), V2.as_slice(), EDGE2.as_slice_mut());

    VCRSS(EDGE1.as_slice(), EDGE2.as_slice(), NORMAL.as_slice_mut());
}
