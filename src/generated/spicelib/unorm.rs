//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Unit vector and norm, 3 dimensional
///
/// Normalize a double precision 3-vector and return its magnitude.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  V1         I   Vector to be normalized.
///  VOUT       O   Unit vector V1 / |V1|.
///  VMAG       O   Magnitude of V1, i.e. |V1|.
/// ```
///
/// # Detailed Input
///
/// ```text
///  V1       is an arbitrary 3-vector, including the
///           zero vector.
/// ```
///
/// # Detailed Output
///
/// ```text
///  VOUT     is the unit vector in the direction of V1. If V1 is
///           the zero vector, then VOUT will also be the zero
///           vector.
///
///  VMAG     is the magnitude of V1.
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
///  UNORM references a function called VNORM (which itself is
///  numerically stable) to calculate the norm of the input vector V1.
///  If the norm is equal to zero, then each component of the output
///  vector VOUT is set to zero. Otherwise, VOUT is calculated by
///  dividing V1 by the norm.
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
///  1) Define a set of vectors and compute their corresponding unit
///     vector and magnitudes.
///
///
///     Example code begins here.
///
///
///           PROGRAM UNORM_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               SETSIZ
///           PARAMETER           ( SETSIZ = 2 )
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      VMAG
///           DOUBLE PRECISION      SETA ( 3, SETSIZ )
///           DOUBLE PRECISION      VOUT ( 3 )
///
///           INTEGER               I
///           INTEGER               J
///
///     C
///     C     Define the vector set.
///     C
///           DATA                  SETA / 5.D0,  12.D0,  0.D0,
///          .                             1.D-7,  2.D-7, 2.D-7 /
///
///     C
///     C     Calculate the unit vectors and magnitudes.
///     C
///           DO I=1, SETSIZ
///
///              CALL UNORM ( SETA(1,I), VOUT, VMAG )
///
///              WRITE(*,'(A,3F13.8)') 'Vector     : ',
///          .                                   ( SETA(J,I), J=1, 3 )
///              WRITE(*,'(A,3F13.8)') 'Unit vector: ', VOUT
///              WRITE(*,'(A,F13.8)')  'Magnitude:   ', VMAG
///              WRITE(*,*) ' '
///
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Vector     :    5.00000000  12.00000000   0.00000000
///     Unit vector:    0.38461538   0.92307692   0.00000000
///     Magnitude:     13.00000000
///
///     Vector     :    0.00000010   0.00000020   0.00000020
///     Unit vector:    0.33333333   0.66666667   0.66666667
///     Magnitude:      0.00000030
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
/// -    SPICELIB Version 1.1.0, 05-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section. Added complete example code.
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
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WMO) (WLT)
/// ```
pub fn unorm(v1: &[f64; 3], vout: &mut [f64; 3], vmag: &mut f64) {
    UNORM(v1, vout, vmag);
}

//$Procedure UNORM ( Unit vector and norm, 3 dimensional )
pub fn UNORM(V1: &[f64], VOUT: &mut [f64], VMAG: &mut f64) {
    let V1 = DummyArray::new(V1, 1..=3);
    let mut VOUT = DummyArrayMut::new(VOUT, 1..=3);

    //
    // SPICELIB functions
    //

    //
    // Obtain the magnitude of V1
    //
    *VMAG = VNORM(V1.as_slice());

    //
    // If VMAG is nonzero, then normalize.  Note that this process is
    // numerically stable: overflow could only happen if VMAG were small,
    // but this could only happen if each component of V1 were small.
    // In fact, the magnitude of any vector is never less than the
    // magnitude of any component.
    //
    if (*VMAG > 0.0) {
        VOUT[1] = (V1[1] / *VMAG);
        VOUT[2] = (V1[2] / *VMAG);
        VOUT[3] = (V1[3] / *VMAG);
    } else {
        VOUT[1] = 0.0;
        VOUT[2] = 0.0;
        VOUT[3] = 0.0;
    }
}
