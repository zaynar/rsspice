//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// "V-Hat", unit vector along V, general dimension
///
/// Find the unit vector along a double precision vector of
/// arbitrary dimension.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  V1         I   Vector to be normalized.
///  NDIM       I   Dimension of V1 (and also VOUT).
///  VOUT       O   Unit vector along V1.
/// ```
///
/// # Detailed Input
///
/// ```text
///  V1       is any double precision vector of arbitrary dimension.
///
///  NDIM     is the dimension of V1 (and also VOUT).
/// ```
///
/// # Detailed Output
///
/// ```text
///  VOUT     is the unit vector in the direction of V1:
///
///                        V1
///              VOUT = --------
///                      ||V1||
///
///           If V1 represents the zero vector, then VOUT will also be
///           the zero vector.
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
///  VHATG determines the magnitude of V1 and then divides each
///  component of V1 by the magnitude. This process is highly stable
///  over the whole range of multi-dimensional vectors.
///
///  This routine will detect if V1 the zero vector, and will not
///  attempt to divide by zero.
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
///  1) Define a set of n-dimensional vectors and find the unit vector
///     along each of them.
///
///
///     Example code begins here.
///
///
///           PROGRAM VHATG_EX1
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
///           DOUBLE PRECISION      V1   ( NDIM, SETSIZ )
///           DOUBLE PRECISION      VOUT ( NDIM         )
///
///           INTEGER               I
///           INTEGER               J
///
///     C
///     C     Define the vector set.
///     C
///           DATA                  V1  /
///          .                          5.D0,  12.D0,  0.D0,  0.D0,
///          .                          1.D-7,  2.D-7, 2.D-7, 0.D0  /
///
///     C
///     C     Calculate the unit vectors.
///     C
///           DO I=1, SETSIZ
///
///              CALL VHATG ( V1(1,I), NDIM, VOUT )
///
///              WRITE(*,'(A,4F12.7)') 'Input vector: ',
///          .                         ( V1(J,I), J=1,NDIM )
///              WRITE(*,'(A,4F12.7)') 'Unit vector : ', VOUT
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
///     Input vector:    5.0000000  12.0000000   0.0000000   0.0000000
///     Unit vector :    0.3846154   0.9230769   0.0000000   0.0000000
///
///     Input vector:    0.0000001   0.0000002   0.0000002   0.0000000
///     Unit vector :    0.3333333   0.6666667   0.6666667   0.0000000
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The relative number of cases whereby floating point overflow
///      may occur is negligible. Thus, no error recovery or reporting
///      scheme is incorporated into this subroutine.
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
///         Edited the header to comply with NAIF standard. Added complete
///         code example based on existing example.
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
pub fn vhatg(v1: &[f64], ndim: i32, vout: &mut [f64]) {
    VHATG(v1, ndim, vout);
}

//$Procedure VHATG ( "V-Hat", unit vector along V, general dimension )
pub fn VHATG(V1: &[f64], NDIM: i32, VOUT: &mut [f64]) {
    let V1 = DummyArray::new(V1, 1..=NDIM);
    let mut VOUT = DummyArrayMut::new(VOUT, 1..=NDIM);
    let mut VMAG: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Obtain the magnitude of V1
    //
    VMAG = VNORMG(V1.as_slice(), NDIM);

    //
    // If VMAG is nonzero, then normalize.  Note that this process is
    // numerically stable: overflow could only happen if VMAG were small,
    // but this could only happen if each component of V1 were small.
    // In fact, the magnitude of any vector is never less than the
    // magnitude of any component.
    //
    if (VMAG > 0.0) {
        for I in 1..=NDIM {
            VOUT[I] = (V1[I] / VMAG);
        }
    } else {
        for I in 1..=NDIM {
            VOUT[I] = 0.0;
        }
    }
    //
}
