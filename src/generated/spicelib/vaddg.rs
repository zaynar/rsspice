//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Vector addition, general dimension
///
/// Add two vectors of arbitrary dimension.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  V1         I   First vector to be added.
///  V2         I   Second vector to be added.
///  NDIM       I   Dimension of V1, V2, and VOUT.
///  VOUT       O   Sum vector, V1 + V2.
/// ```
///
/// # Detailed Input
///
/// ```text
///  V1,
///  V2       are two arbitrary double precision n-dimensional
///           vectors.
///
///  NDIM     is the dimension of V1, V2 and VOUT.
/// ```
///
/// # Detailed Output
///
/// ```text
///  VOUT     is the double precision n-dimensional vector sum of V1
///           and V2.
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
///  This routine simply performs addition between components of V1
///  and V2. No checking is performed to determine whether floating
///  point overflow has occurred.
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
///  1) Define two sets of n-dimensional vectors and compute the sum
///     of each vector in first set with the corresponding vector in
///     the second set.
///
///
///     Example code begins here.
///
///
///           PROGRAM VADDG_EX1
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
///           DOUBLE PRECISION      SETA ( NDIM, SETSIZ )
///           DOUBLE PRECISION      SETB ( NDIM, SETSIZ )
///           DOUBLE PRECISION      VOUT ( NDIM )
///
///           INTEGER               I
///           INTEGER               J
///
///     C
///     C     Define the two vector sets.
///     C
///           DATA                  SETA /
///          .                      1.D0,  2.D0,   3.D0,  4.D0,
///          .                      1.D-7, 1.D23, 1.D-9,  0.D0   /
///
///           DATA                  SETB /
///          .                      4.D0,  5.D0,   6.D0,  7.D0,
///          .                      1.D24, 1.D23,  0.D0,  3.D-23  /
///
///     C
///     C     Calculate the sum of each pair of vectors
///     C
///           DO I=1, SETSIZ
///
///              CALL VADDG ( SETA(1,I), SETB(1,I), NDIM, VOUT )
///
///              WRITE(*,'(A,4E11.2)') 'Vector A  : ',
///          .                        ( SETA(J,I), J=1,NDIM )
///              WRITE(*,'(A,4E11.2)') 'Vector B  : ',
///          .                        ( SETB(J,I), J=1,NDIM )
///              WRITE(*,'(A,4E11.2)') 'Sum vector: ', VOUT
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
///     Vector A  :    0.10E+01   0.20E+01   0.30E+01   0.40E+01
///     Vector B  :    0.40E+01   0.50E+01   0.60E+01   0.70E+01
///     Sum vector:    0.50E+01   0.70E+01   0.90E+01   0.11E+02
///
///     Vector A  :    0.10E-06   0.10E+24   0.10E-08   0.00E+00
///     Vector B  :    0.10E+25   0.10E+24   0.00E+00   0.30E-22
///     Sum vector:    0.10E+25   0.20E+24   0.10E-08   0.30E-22
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The user is required to determine that the magnitude each
///      component of the vectors is within the appropriate range so as
///      not to cause floating point overflow.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.M. Owen          (JPL)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
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
/// -    SPICELIB Version 1.0.2, 07-NOV-2003 (EDW)
///
///         Corrected a mistake in the second example's value
///         for VOUT, i.e. replaced (1D24, 2D23, 0.0) with
///         (1D24, 2D23).
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WMO)
/// ```
pub fn vaddg(v1: &[f64], v2: &[f64], ndim: i32, vout: &mut [f64]) {
    VADDG(v1, v2, ndim, vout);
}

//$Procedure VADDG ( Vector addition, general dimension )
pub fn VADDG(V1: &[f64], V2: &[f64], NDIM: i32, VOUT: &mut [f64]) {
    let V1 = DummyArray::new(V1, 1..=NDIM);
    let V2 = DummyArray::new(V2, 1..=NDIM);
    let mut VOUT = DummyArrayMut::new(VOUT, 1..=NDIM);

    //
    // Local variables
    //

    for I in 1..=NDIM {
        VOUT[I] = (V1[I] + V2[I]);
    }
}
