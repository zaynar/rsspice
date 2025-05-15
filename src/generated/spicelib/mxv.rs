//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Matrix times vector, 3x3
///
/// Multiply a 3x3 double precision matrix with a 3-dimensional
/// double precision vector.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  M          I   3x3 double precision matrix.
///  VIN        I   3-dimensional double precision vector.
///  VOUT       O   3-dimensional double precision vector. VOUT is
///                 the product M*VIN.
/// ```
///
/// # Detailed Input
///
/// ```text
///  M        is an arbitrary 3x3 double precision matrix.
///
///  VIN      is an arbitrary 3-dimensional double precision vector.
/// ```
///
/// # Detailed Output
///
/// ```text
///  VOUT     is a 3-dimensional double precision vector. VOUT is
///           the product M * V.
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
///        VOUT(I) =   )  M(I,K) * VIN(K)
///                   /
///                  '-----
///                    K=1
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
///  1) Given a 3x3 matrix and a 3-vector, multiply the matrix by
///     the vector.
///
///
///     Example code begins here.
///
///
///           PROGRAM MXV_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      M    ( 3, 3 )
///           DOUBLE PRECISION      VIN  ( 3    )
///           DOUBLE PRECISION      VOUT ( 3    )
///
///     C
///     C     Define M and VIN.
///     C
///           DATA                  M    /  0.0D0, -1.0D0,  0.0D0,
///          .                              1.0D0,  0.0D0,  0.0D0,
///          .                              0.0D0,  0.0D0,  1.0D0  /
///
///           DATA                  VIN  /  1.0D0,  2.0D0,  3.0D0  /
///
///     C
///     C     Multiply M by VIN.
///     C
///           CALL MXV ( M, VIN, VOUT )
///
///           WRITE(*,'(A)') 'M times VIN:'
///           WRITE(*,'(3F10.3)') VOUT
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     M times VIN:
///          2.000    -1.000     3.000
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
/// -    SPICELIB Version 1.1.0, 13-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example based on the existing example.
///
///         Changed input argument name MATRIX to M for consistency with
///         other routines.
///
///         Added IMPLICIT NONE statement.
///
/// -    SPICELIB Version 1.0.2, 22-APR-2010 (NJB)
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
pub fn mxv(m: &[[f64; 3]; 3], vin: &[f64; 3], vout: &mut [f64; 3]) {
    MXV(m.as_flattened(), vin, vout);
}

//$Procedure MXV ( Matrix times vector, 3x3 )
pub fn MXV(M: &[f64], VIN: &[f64], VOUT: &mut [f64]) {
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
        PRODV[I] = (((M[[I, 1]] * VIN[1]) + (M[[I, 2]] * VIN[2])) + (M[[I, 3]] * VIN[3]));
    }
    //
    // Move the buffered vector into the output vector VOUT.
    //
    VOUT[1] = PRODV[1];
    VOUT[2] = PRODV[2];
    VOUT[3] = PRODV[3];
}
