//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Unit vector and norm, general dimension
///
/// Normalize a double precision vector of arbitrary dimension and
/// return its magnitude.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  V1         I   Vector to be normalized.
///  NDIM       I   Dimension of V1 (and also VOUT).
///  VOUT       O   Unit vector V1 / ||V1||.
///  VMAG       O   Magnitude of V1, i.e. ||V1||.
/// ```
///
/// # Detailed Input
///
/// ```text
///  V1       is an arbitrary double precision n-dimensional vector,
///           including the zero vector.
///
///  NDIM     is the dimension of V1 and VOUT.
/// ```
///
/// # Detailed Output
///
/// ```text
///  VOUT     is the double precision n-dimensional unit vector in the
///           direction of V1. If V1 is the zero vector, then VOUT
///           will also be the zero vector.
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
///  UNORMG references a function called VNORMG (which itself is
///  numerically stable) to calculate the norm of the input vector V1.
///  If the norm is equal to zero, then each component of the output
///  vector VOUT is set to zero. Otherwise, VOUT is calculated by
///  dividing V1 by the norm. No error detection or correction is
///  implemented.
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
///  1) Define a set of n-dimensional vectors and compute their
///     corresponding unit vectors and magnitudes.
///
///
///     Example code begins here.
///
///
///           PROGRAM UNORMG_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               NDIM
///           PARAMETER           ( NDIM   = 4 )
///
///           INTEGER               SETSIZ
///           PARAMETER           ( SETSIZ = 2 )
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      VMAG
///           DOUBLE PRECISION      V1   ( NDIM, SETSIZ )
///           DOUBLE PRECISION      VOUT ( NDIM         )
///
///           INTEGER               I
///           INTEGER               J
///
///     C
///     C     Define the vector set.
///     C
///           DATA                  V1    /
///          .                         5.D0,  12.D0,  0.D0,  4.D0,
///          .                         1.D-6,  2.D-6, 2.D-6, 0.D0  /
///
///     C
///     C     Calculate the unit vectors and magnitudes.
///     C
///           DO I=1, SETSIZ
///
///              CALL UNORMG ( V1(1,I), NDIM, VOUT, VMAG )
///
///              WRITE(*,'(A,4F12.7)') 'Vector     :',
///          .                         ( V1(J,I), J=1,NDIM )
///              WRITE(*,'(A,4F12.7)') 'Unit vector:', VOUT
///              WRITE(*,'(A,F12.7)')  'Magnitude  :', VMAG
///              WRITE(*,*)
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
///     Vector     :   5.0000000  12.0000000   0.0000000   4.0000000
///     Unit vector:   0.3676073   0.8822575   0.0000000   0.2940858
///     Magnitude  :  13.6014705
///
///     Vector     :   0.0000010   0.0000020   0.0000020   0.0000000
///     Unit vector:   0.3333333   0.6666667   0.6666667   0.0000000
///     Magnitude  :   0.0000030
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  No error checking is implemented in this subroutine to guard
///      against numeric overflow.
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
///         unnecessary $Revisions section. Updated code comments.
///
///         Added complete code example based on existing example.
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
pub fn unormg(v1: &[f64], ndim: i32, vout: &mut [f64], vmag: &mut f64) {
    UNORMG(v1, ndim, vout, vmag);
}

//$Procedure UNORMG ( Unit vector and norm, general dimension )
pub fn UNORMG(V1: &[f64], NDIM: i32, VOUT: &mut [f64], VMAG: &mut f64) {
    let V1 = DummyArray::new(V1, 1..=NDIM);
    let mut VOUT = DummyArrayMut::new(VOUT, 1..=NDIM);

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Obtain the magnitude of V1
    //
    *VMAG = VNORMG(V1.as_slice(), NDIM);

    //
    // If VMAG is nonzero, then normalize. Note that this process is
    // numerically stable: overflow could only happen if VMAG were small,
    // but this could only happen if each component of V1 were also small.
    // In fact, the magnitude of any vector is never less than the
    // magnitude of any component.
    //
    if (*VMAG > 0.0) {
        for I in 1..=NDIM {
            VOUT[I] = (V1[I] / *VMAG);
        }
    } else {
        for I in 1..=NDIM {
            VOUT[I] = 0.0;
        }
    }
    //
}
