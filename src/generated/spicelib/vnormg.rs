//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Vector norm, general dimension
///
/// Compute the magnitude of a double precision vector of arbitrary
/// dimension.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  V1         I   Vector whose magnitude is to be found.
///  NDIM       I   Dimension of V1.
///
///  The function returns the magnitude of V1.
/// ```
///
/// # Detailed Input
///
/// ```text
///  V1       is any double precision vector of arbitrary dimension.
///
///  NDIM     is the dimension of the input vector V1.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the magnitude of V1 calculated in a
///  numerically stable way.
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
///  VNORMG finds the component of V1 whose magnitude is the largest.
///  If the absolute magnitude of that component indicates that a
///  numeric overflow would occur when it is squared, or if it
///  indicates that an underflow would occur when squared (falsely
///  giving a magnitude of zero) then the following expression is
///  used:
///
///     VNORMG = V1MAX * MAGNITUDE OF [ (1/V1MAX)*V1 ]
///
///  Otherwise a simpler expression is used:
///
///     VNORMG = MAGNITUDE OF [ V1 ]
///
///  Beyond the logic described above, no further checking of the
///  validity of the input is performed.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for these examples may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Define a four-dimensional vector and calculate its magnitude.
///
///     Example code begins here.
///
///
///           PROGRAM VNORMG_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      VNORMG
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               NDIM
///           PARAMETER           ( NDIM   = 4 )
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      V1 ( NDIM )
///
///           DATA                  V1 / 12.3D0, -4.32D0,
///          .                           76.0D0,  1.87D0 /
///
///     C
///     C     Compute the magnitude of V1
///     C
///           WRITE(*,*) 'Magnitude of v1: ', VNORMG ( V1, NDIM )
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Magnitude of v1:    77.132673362201047
///
///
///  2) The following table show the correlation between various input
///     vectors V1 and VNORMG:
///
///        NDIM    V1(NDIM)                          VNORMG
///        -------------------------------------------------
///         1      (-7.0D20)                          7.D20
///         3      (1.D0, 2.D0, 2.D0)                 3.D0
///         4      (3.D0, 3.D0, 3.D0, 3.D0)           6.D0
///         5      (5.D0, 12.D0, 0.D0, 0.D0, 0.D0)   13.D0
///         3      (-5.D-17, 0.D0, 12.D-17)          13.D-17
/// ```
///
/// # Author and Institution
///
/// ```text
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
///         Edited the header comments to comply with NAIF standard. Added
///         complete code example.
///
///         Added IMPLICIT NONE statement.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WMO)
/// ```
pub fn vnormg(v1: &[f64], ndim: i32) -> f64 {
    let ret = VNORMG(v1, ndim);
    ret
}

//$Procedure VNORMG ( Vector norm, general dimension )
pub fn VNORMG(V1: &[f64], NDIM: i32) -> f64 {
    let V1 = DummyArray::new(V1, 1..=NDIM);
    let mut VNORMG: f64 = 0.0;
    let mut V1MAX: f64 = 0.0;
    let mut A: f64 = 0.0;

    //
    // Local variables.
    //

    //
    // Determine the maximum component of the vector.
    //
    V1MAX = 0.0;

    for I in 1..=NDIM {
        if (f64::abs(V1[I]) > V1MAX) {
            V1MAX = f64::abs(V1[I]);
        }
    }
    //
    // If the vector is zero, return zero; otherwise normalize first.
    // Normalizing helps in the cases where squaring would cause overflow
    // or underflow.  In the cases where such is not a problem it not worth
    // it to optimize further.
    //
    if (V1MAX == 0.0) {
        VNORMG = 0.0;
    } else {
        VNORMG = 0.0;

        for I in 1..=NDIM {
            A = (V1[I] / V1MAX);
            VNORMG = (VNORMG + (A * A));
        }

        VNORMG = (V1MAX * f64::sqrt(VNORMG));
    }
    //
    VNORMG
}
