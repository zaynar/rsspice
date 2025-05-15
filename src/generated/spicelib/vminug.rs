//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Negate vector, "-V", general dimension
///
/// Negate a double precision vector of arbitrary dimension.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  VIN        I   n-dimensional vector to be negated.
///  NDIM       I   Dimension of VIN (and also VOUT).
///  VOUT       O   Negated vector -V1.
/// ```
///
/// # Detailed Input
///
/// ```text
///  VIN      is any double precision vector of arbitrary size.
///
///  NDIM     is the dimension of VIN and VOUT.
/// ```
///
/// # Detailed Output
///
/// ```text
///  VOUT     is a n-dimensional double precision vector which
///           contains the negation of VIN.
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
///  For each value of the index I from 1 to NDIM, VMINUG negates VIN
///  by the expression:
///
///     VOUT(I) = - VIN(I)
///
///  No error checking is performed since overflow can occur ONLY if
///  the dynamic range of positive floating point numbers is not the
///  same size as the dynamic range of negative floating point numbers
///  AND at least one component of VIN falls outside the common range.
///  The likelihood of this occurring is so small as to be of no
///  concern.
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
///  1) Define a set of n-dimensional vectors and negate each of them.
///
///
///     Example code begins here.
///
///
///           PROGRAM VMINUG_EX1
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
///           DOUBLE PRECISION      VIN  ( NDIM, SETSIZ )
///           DOUBLE PRECISION      VOUT ( NDIM         )
///
///           INTEGER               I
///           INTEGER               J
///
///     C
///     C     Define a set of n-dimensional vectors.
///     C
///           DATA                  VIN  /
///          .                    -10.D0, 15.D0, -5.D0, 20.D0,
///          .                      0.D0,  0.D0,  0.D0,  0.D0  /
///
///     C
///     C     Negate each vector
///     C
///           DO I=1, SETSIZ
///
///              CALL VMINUG ( VIN(1,I), NDIM, VOUT )
///
///              WRITE(*,'(A,4F7.1)') 'Input vector  : ',
///          .                        ( VIN(J,I), J=1,NDIM )
///              WRITE(*,'(A,4F7.1)') 'Negated vector: ', VOUT
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
///     Input vector  :   -10.0   15.0   -5.0   20.0
///     Negated vector:    10.0  -15.0    5.0  -20.0
///
///     Input vector  :     0.0    0.0    0.0    0.0
///     Negated vector:    -0.0   -0.0   -0.0   -0.0
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
/// -    SPICELIB Version 1.0.3, 02-OCT-2020 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example based on existing example.
///
///         Extended $Particulars section.
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
pub fn vminug(vin: &[f64], ndim: i32, vout: &mut [f64]) {
    VMINUG(vin, ndim, vout);
}

//$Procedure VMINUG ( Negate vector, "-V", general dimension )
pub fn VMINUG(VIN: &[f64], NDIM: i32, VOUT: &mut [f64]) {
    let VIN = DummyArray::new(VIN, 1..=NDIM);
    let mut VOUT = DummyArrayMut::new(VOUT, 1..=NDIM);

    //
    for I in 1..=NDIM {
        VOUT[I] = -VIN[I];
    }
}
