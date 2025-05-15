//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Matrix transpose times vector, 3x3
///
/// Multiply the transpose of a 3x3 matrix on the left with a vector
/// on the right.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  M          I   3X3 double precision matrix.
///  VIN        I   3-dimensional double precision vector.
///  VOUT       O   3-dimensional double precision vector. VOUT is
///                 the product M**T * VIN.
/// ```
///
/// # Detailed Input
///
/// ```text
///  M        is an arbitrary 3x3 double precision matrix.
///           Typically, M will be a rotation matrix since
///           then its transpose is its inverse (but this is NOT
///           a requirement).
///
///  VIN      is an arbitrary 3-dimensional double precision
///           vector.
/// ```
///
/// # Detailed Output
///
/// ```text
///  VOUT     is a 3-dimensional double precision vector. VOUT is
///           the product VOUT = (M**T)  x (VIN).
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
/// ```
///
/// # Particulars
///
/// ```text
///  The code reflects precisely the following mathematical expression
///
///     For each value of the subscript I from 1 to 3:
///
///                     3
///                  .-----
///                   \
///        VOUT(I) =   )  M(K,I) * VIN(K)
///                   /
///                  '-----
///                    K=1
///
///  Note that the reversal of the K and I subscripts in the left-hand
///  matrix M is what makes VOUT the product of the TRANSPOSE of
///  and not simply of M itself.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Given a 3x3 matrix and a 3-vector, multiply the transpose of
///     the matrix by the vector.
///
///
///     Example code begins here.
///
///
///           PROGRAM MTXV_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      M    ( 3, 3 )
///           DOUBLE PRECISION      VIN  ( 3    )
///           DOUBLE PRECISION      VOUT ( 3    )
///
///           INTEGER               I
///           INTEGER               J
///
///     C
///     C     Define M and VIN.
///     C
///           DATA                  M    /  1.0D0, -1.0D0,  0.0D0,
///          .                              1.0D0,  1.0D0,  0.0D0,
///          .                              0.0D0,  0.0D0,  1.0D0  /
///
///           DATA                  VIN  /  5.0D0, 10.0D0, 15.0D0  /
///
///     C
///     C     Multiply the transpose of M by VIN.
///     C
///           CALL MTXV ( M, VIN, VOUT )
///
///           WRITE(*,'(A)') 'Transpose of M times VIN:'
///           WRITE(*,'(3F10.3)') VOUT
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Transpose of M times VIN:
///         -5.000    15.000    15.000
///
///
///     Note that typically the matrix M will be a rotation matrix.
///     Because the transpose of an orthogonal matrix is equivalent to
///     its inverse, applying the rotation to the vector is
///     accomplished by multiplying the vector by the transpose of the
///     matrix.
///
///     Let
///
///            -1
///           M   * VIN = VOUT
///
///     If M is an orthogonal matrix, then (M**T) * VIN = VOUT.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The user is responsible for checking the magnitudes of the
///      elements of M and VIN so that a floating point overflow does
///      not occur.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.M. Owen          (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 25-AUG-2021 (JDR)
///
///         Changed input argument name MATRIX to M for consistency with
///         other routines.
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///         Added complete code example based on the existing example.
///
/// -    SPICELIB Version 1.0.2, 23-APR-2010 (NJB)
///
///         Header correction: assertions that the output
///         can overwrite the input have been removed.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WMO)
/// ```
pub fn mtxv(m: &[[f64; 3]; 3], vin: &[f64; 3], vout: &mut [f64; 3]) {
    MTXV(m.as_flattened(), vin, vout);
}

//$Procedure MTXV ( Matrix transpose times vector, 3x3 )
pub fn MTXV(M: &[f64], VIN: &[f64], VOUT: &mut [f64]) {
    let M = DummyArray2D::new(M, 1..=3, 1..=3);
    let VIN = DummyArray::new(VIN, 1..=3);
    let mut VOUT = DummyArrayMut::new(VOUT, 1..=3);
    let mut PRODV = StackArray::<f64, 3>::new(1..=3);

    //
    // Local variables
    //

    //
    // Perform the matrix-vector multiplication
    //
    for I in 1..=3 {
        PRODV[I] = (((M[[1, I]] * VIN[1]) + (M[[2, I]] * VIN[2])) + (M[[3, I]] * VIN[3]));
    }

    //
    // Move the result into VOUT
    //
    VOUT[1] = PRODV[1];
    VOUT[2] = PRODV[2];
    VOUT[3] = PRODV[3];
}
