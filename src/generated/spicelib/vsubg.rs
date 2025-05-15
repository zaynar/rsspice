//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Vector subtraction, general dimension
///
/// Compute the difference between two double precision vectors of
/// arbitrary dimension.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  V1         I   First vector (minuend).
///  V2         I   Second vector (subtrahend).
///  NDIM       I   Dimension of V1, V2, and VOUT.
///  VOUT       O   Difference vector, V1 - V2.
/// ```
///
/// # Detailed Input
///
/// ```text
///  V1       is a double precision vector of arbitrary dimension which
///           is the minuend (i.e. first or left-hand member) in the
///           vector subtraction.
///
///  V2       is a double precision vector of arbitrary dimension which
///           is the subtrahend (i.e. second or right-hand member) in
///           the vector subtraction.
///
///  NDIM     is the dimension of V1 and V2 (and VOUT).
/// ```
///
/// # Detailed Output
///
/// ```text
///  VOUT     is a double precision n-dimensional vector which
///           represents the vector difference, V1 - V2.
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
///  For each value of the index I from 1 to NDIM, this routine
///  performs the following subtraction:
///
///     VOUT(I) = V1(I) - V2(I)
///
///  No error checking is performed to guard against numeric overflow
///  or underflow.
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
///  1) Define two sets of n-dimensional vectors and compute the
///     difference from each vector in first set with the
///     corresponding vector in the second set.
///
///
///     Example code begins here.
///
///
///           PROGRAM VSUBG_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               NDIM
///           PARAMETER           ( NDIM   = 4 )
///
///           INTEGER               SETSIZ
///           PARAMETER           ( SETSIZ = 3 )
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      V1   ( NDIM, SETSIZ )
///           DOUBLE PRECISION      V2   ( NDIM, SETSIZ )
///           DOUBLE PRECISION      VOUT ( NDIM         )
///
///           INTEGER               I
///           INTEGER               J
///
///     C
///     C     Define the two vector sets.
///     C
///           DATA                  V1 /
///          .                      1.D0,  2.D0,  3.D0,  4.D0,
///          .                      1.D0,  2.D0,  3.D0,  4.D0,
///          .                      1.D0,  2.D0,  3.D0,  4.D0   /
///
///           DATA                  V2 /
///          .                      1.D0,  1.D0,  1.D0,  1.D0,
///          .                     -1.D0, -2.D0, -3.D0, -4.D0,
///          .                     -1.D0,  2.D0, -3.D0,  4.D0  /
///
///     C
///     C     Calculate the difference between each pair of vectors
///     C
///           DO I=1, SETSIZ
///
///              CALL VSUBG ( V1(1,I), V2(1,I), NDIM, VOUT )
///
///              WRITE(*,'(A,4F6.1)') 'First vector : ',
///          .                        ( V1(J,I), J=1,NDIM )
///              WRITE(*,'(A,4F6.1)') 'Second vector: ',
///          .                        ( V2(J,I), J=1,NDIM )
///              WRITE(*,'(A,4F6.1)') 'Difference   : ', VOUT
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
///     First vector :    1.0   2.0   3.0   4.0
///     Second vector:    1.0   1.0   1.0   1.0
///     Difference   :    0.0   1.0   2.0   3.0
///
///     First vector :    1.0   2.0   3.0   4.0
///     Second vector:   -1.0  -2.0  -3.0  -4.0
///     Difference   :    2.0   4.0   6.0   8.0
///
///     First vector :    1.0   2.0   3.0   4.0
///     Second vector:   -1.0   2.0  -3.0   4.0
///     Difference   :    2.0   0.0   6.0   0.0
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  No error checking is performed to guard against numeric
///      overflow. The programmer is thus required to insure that the
///      values in V1 and V2 are reasonable and will not cause
///      overflow. No error recovery or reporting scheme is
///      incorporated in this routine.
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
/// -    SPICELIB Version 1.1.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example based on existing example.
///
/// -    SPICELIB Version 1.0.3, 23-APR-2010 (NJB)
///
///         Header correction: assertions that the output
///         can overwrite the input have been removed.
///
/// -    SPICELIB Version 1.0.2, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.1, 09-MAY-1990 (HAN)
///
///         Several errors in the header documentation were corrected.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WMO)
/// ```
pub fn vsubg(v1: &[f64], v2: &[f64], ndim: i32, vout: &mut [f64]) {
    VSUBG(v1, v2, ndim, vout);
}

//$Procedure VSUBG ( Vector subtraction, general dimension )
pub fn VSUBG(V1: &[f64], V2: &[f64], NDIM: i32, VOUT: &mut [f64]) {
    let V1 = DummyArray::new(V1, 1..=NDIM);
    let V2 = DummyArray::new(V2, 1..=NDIM);
    let mut VOUT = DummyArrayMut::new(VOUT, 1..=NDIM);

    //
    // Local variables
    //

    for I in 1..=NDIM {
        VOUT[I] = (V1[I] - V2[I]);
    }
}
