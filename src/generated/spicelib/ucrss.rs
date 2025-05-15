//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Unitized cross product, 3x3
///
/// Compute the normalized cross product of two 3-vectors.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  V1         I   Left vector for cross product.
///  V2         I   Right vector for cross product.
///  VOUT       O   Normalized cross product of V1 and V2.
/// ```
///
/// # Detailed Input
///
/// ```text
///  V1,
///  V2       are two double precision 3-dimensional vectors.
///           Typically, these might represent the (possibly unit)
///           vector to a planet, Sun, or a star which defines the
///           orientation of axes of some reference frame.
/// ```
///
/// # Detailed Output
///
/// ```text
///  VOUT     is the double precision 3-dimensional normalized cross
///           product of V1 and V2. VOUT is the result of the
///           computation
///
///                  V1 x V2
///              ---------------
///               || V1 x V2 ||
///
///           where "x" denotes the cross product and ||X||| the norm
///           of a vector X.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If the cross product of V1 and V2 yields the zero-vector,
///      then the zero-vector is returned instead of a vector of
///      unit length.
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
///  1) Define two sets of vectors and compute the normalized cross
///     product of each vector in first set and the corresponding
///     vector in the second set.
///
///
///     Example code begins here.
///
///
///           PROGRAM UCRSS_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               NDIM
///           PARAMETER           ( NDIM   = 3 )
///
///           INTEGER               SETSIZ
///           PARAMETER           ( SETSIZ = 2 )
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      V1   ( NDIM, SETSIZ )
///           DOUBLE PRECISION      V2   ( NDIM, SETSIZ )
///           DOUBLE PRECISION      VOUT ( NDIM )
///
///           INTEGER               I
///           INTEGER               J
///
///     C
///     C     Define the two vector sets.
///     C
///           DATA                  V1   / 0.D0,  1.D0,  0.D0,
///          .                             5.D0,  5.D0,  5.D0  /
///
///           DATA                  V2   / 3.D0,  0.D0,  0.D0,
///          .                            -2.D0, -2.D0, -2.D0  /
///
///     C
///     C     Calculate the cross product of each pair of vectors
///     C
///           DO I=1, SETSIZ
///
///              CALL UCRSS ( V1(1,I), V2(1,I), VOUT )
///
///              WRITE(*,'(A,3F5.1)') 'Vector A                : ',
///          .                                  ( V1(J,I), J=1,3 )
///              WRITE(*,'(A,3F5.1)') 'Vector B                : ',
///          .                                  ( V2(J,I), J=1,3 )
///              WRITE(*,'(A,3F5.1)') 'Normalized cross product: ',
///          .                                                VOUT
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
///     Vector A                :   0.0  1.0  0.0
///     Vector B                :   3.0  0.0  0.0
///     Normalized cross product:   0.0  0.0 -1.0
///
///     Vector A                :   5.0  5.0  5.0
///     Vector B                :  -2.0 -2.0 -2.0
///     Normalized cross product:   0.0  0.0  0.0
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
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WMO) (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    Beta Version 1.1.0, 10-JAN-1989 (WLT)
///
///         Error free specification added. In addition the algorithm was
///         made more robust in the sense that floating point overflows
///         cannot occur.
/// ```
pub fn ucrss(v1: &[f64; 3], v2: &[f64; 3], vout: &mut [f64; 3]) {
    UCRSS(v1, v2, vout);
}

//$Procedure UCRSS ( Unitized cross product, 3x3 )
pub fn UCRSS(V1: &[f64], V2: &[f64], VOUT: &mut [f64]) {
    let V1 = DummyArray::new(V1, 1..=3);
    let V2 = DummyArray::new(V2, 1..=3);
    let mut VOUT = DummyArrayMut::new(VOUT, 1..=3);
    let mut VCROSS = StackArray::<f64, 3>::new(1..=3);
    let mut VMAG: f64 = 0.0;
    let mut MAXV1: f64 = 0.0;
    let mut MAXV2: f64 = 0.0;
    let mut TV1 = StackArray::<f64, 3>::new(1..=3);
    let mut TV2 = StackArray::<f64, 3>::new(1..=3);

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Get the biggest component of each of the two vectors.
    //
    MAXV1 = intrinsics::DMAX1(&[f64::abs(V1[1]), f64::abs(V1[2]), f64::abs(V1[3])]);
    MAXV2 = intrinsics::DMAX1(&[f64::abs(V2[1]), f64::abs(V2[2]), f64::abs(V2[3])]);

    //
    // Scale V1 and V2 by 1/MAXV1 and 1/MAXV2 respectively
    //
    if (MAXV1 != 0 as f64) {
        TV1[1] = (V1[1] / MAXV1);
        TV1[2] = (V1[2] / MAXV1);
        TV1[3] = (V1[3] / MAXV1);
    } else {
        TV1[1] = 0.0;
        TV1[2] = 0.0;
        TV1[3] = 0.0;
    }

    if (MAXV2 != 0 as f64) {
        TV2[1] = (V2[1] / MAXV2);
        TV2[2] = (V2[2] / MAXV2);
        TV2[3] = (V2[3] / MAXV2);
    } else {
        TV2[1] = 0.0;
        TV2[2] = 0.0;
        TV2[3] = 0.0;
    }

    //
    // Calculate the cross product of V1 and V2
    //
    VCROSS[1] = ((TV1[2] * TV2[3]) - (TV1[3] * TV2[2]));
    VCROSS[2] = ((TV1[3] * TV2[1]) - (TV1[1] * TV2[3]));
    VCROSS[3] = ((TV1[1] * TV2[2]) - (TV1[2] * TV2[1]));

    //
    // Get the magnitude of VCROSS and normalize it
    //
    VMAG = VNORM(VCROSS.as_slice());

    if (VMAG > 0.0) {
        VOUT[1] = (VCROSS[1] / VMAG);
        VOUT[2] = (VCROSS[2] / VMAG);
        VOUT[3] = (VCROSS[3] / VMAG);
    } else {
        VOUT[1] = 0.0;
        VOUT[2] = 0.0;
        VOUT[3] = 0.0;
    }
}
