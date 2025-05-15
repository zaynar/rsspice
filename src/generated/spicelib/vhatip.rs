//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// "V-Hat", 3-d unit vector along V, in place
///
/// Scale a three-dimensional vector to unit length.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  V         I-O  Vector to be normalized/unit vector.
/// ```
///
/// # Detailed Input
///
/// ```text
///  V        is any double precision, 3-dimensional vector. If this
///           vector is the zero vector, this routine will detect it,
///           and will not attempt to divide by zero.
/// ```
///
/// # Detailed Output
///
/// ```text
///  V        is the unit vector in the direction of the input vector.
///           If on input V represents the zero vector, then V will be
///           returned as the zero vector.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  The zero vector is returned if the input value of V is the
///      zero vector.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is provided for situation where it is convenient
///  to scale a vector to unit length in place rather than store
///  the result in a separate variable. Note that the call
///
///     CALL VHAT ( V, V )
///
///  is not permitted by the ANSI Fortran 77 standard; this routine
///  can be called instead to achieve the same result.
///
///  VHATIP determines the magnitude of V and then, if the magnitude
///  is non-zero, divides each component of V by the magnitude. This
///  process is highly stable over the whole range of 3-dimensional
///  vectors.
/// ```
///
/// # Examples
///
/// ```text
///  The following table shows how selected vectors are mapped to
///  unit vectors
///
///  V on input            V on output
///  ------------------    ------------------
///  (5, 12, 0)            (5/13, 12/13, 0)
///  (1D-7, 2D-7, 2D-7)    (1/3, 2/3, 2/3)
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  There is no known case whereby floating point overflow may
///      occur. Thus, no error recovery or reporting scheme is
///      incorporated into this subroutine.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.M. Owen          (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 20-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.1.0, 01-SEP-2005 (NJB) (HAN) (WMO) (WLT)
/// ```
pub fn vhatip(v: &mut [f64; 3]) {
    VHATIP(v);
}

//$Procedure VHATIP ( "V-Hat", 3-d unit vector along V, in place )
pub fn VHATIP(V: &mut [f64]) {
    let mut V = DummyArrayMut::new(V, 1..=3);
    let mut VMAG: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Obtain the magnitude of V.
    //
    VMAG = VNORM(V.as_slice());

    //
    // If VMAG is nonzero, then normalize. Note that this process is
    // numerically stable: overflow could only happen if VMAG were
    // small, but this could only happen if each component of V1 were
    // small. In fact, the magnitude of any vector is never less than
    // the magnitude of any component.
    //
    if (VMAG > 0.0) {
        V[1] = (V[1] / VMAG);
        V[2] = (V[2] / VMAG);
        V[3] = (V[3] / VMAG);
    } else {
        V[1] = 0.0;
        V[2] = 0.0;
        V[3] = 0.0;
    }
}
