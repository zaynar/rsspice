//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Vector addition, 3 dimensional
///
/// Add two double precision 3-dimensional vectors.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  V1         I   First vector to be added.
///  V2         I   Second vector to be added.
///  VOUT       O   Sum vector, V1 + V2.
/// ```
///
/// # Detailed Input
///
/// ```text
///  V1,
///  V2       are two arbitrary double precision 3-dimensional
///           vectors.
/// ```
///
/// # Detailed Output
///
/// ```text
///  VOUT     is the double precision 3-dimensional vector sum of V1
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
///  1) Define two sets of 3-dimensional vectors and compute the sum
///     of each vector in first set with the corresponding vector in
///     the second set.
///
///
///     Example code begins here.
///
///
///           PROGRAM VADD_EX1
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
///           DOUBLE PRECISION      SETA ( 3, SETSIZ )
///           DOUBLE PRECISION      SETB ( 3, SETSIZ )
///           DOUBLE PRECISION      VOUT ( 3 )
///
///           INTEGER               I
///           INTEGER               J
///
///     C
///     C     Define the two vector sets.
///     C
///           DATA                  SETA / 1.D0,  2.D0,  3.D0,
///          .                             1.D-7, 1.D23, 1.D-9  /
///
///           DATA                  SETB / 4.D0,  5.D0,   6.D0,
///          .                             1.D24, 1.D23,  0.D0  /
///
///     C
///     C     Calculate the sum of each pair of vectors
///     C
///           DO I=1, SETSIZ
///
///              CALL VADD ( SETA(1,I), SETB(1,I), VOUT )
///
///              WRITE(*,'(A,3E11.2)') 'Vector A  : ',
///          .                        ( SETA(J,I), J=1,3 )
///              WRITE(*,'(A,3E11.2)') 'Vector B  : ',
///          .                        ( SETB(J,I), J=1,3 )
///              WRITE(*,'(A,3E11.2)') 'Sum vector: ', VOUT
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
///     Vector A  :    0.10E+01   0.20E+01   0.30E+01
///     Vector B  :    0.40E+01   0.50E+01   0.60E+01
///     Sum vector:    0.50E+01   0.70E+01   0.90E+01
///
///     Vector A  :    0.10E-06   0.10E+24   0.10E-08
///     Vector B  :    0.10E+25   0.10E+24   0.00E+00
///     Sum vector:    0.10E+25   0.20E+24   0.10E-08
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
pub fn vadd(v1: &[f64; 3], v2: &[f64; 3], vout: &mut [f64; 3]) {
    VADD(v1, v2, vout);
}

//$Procedure VADD ( Vector addition, 3 dimensional )
pub fn VADD(V1: &[f64], V2: &[f64], VOUT: &mut [f64]) {
    let V1 = DummyArray::new(V1, 1..=3);
    let V2 = DummyArray::new(V2, 1..=3);
    let mut VOUT = DummyArrayMut::new(VOUT, 1..=3);

    VOUT[1] = (V1[1] + V2[1]);
    VOUT[2] = (V1[2] + V2[2]);
    VOUT[3] = (V1[3] + V2[3]);
}
