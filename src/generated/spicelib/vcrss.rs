//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Vector cross product, 3 dimensions
///
/// Compute the cross product of two 3-dimensional vectors.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  V1         I   Left hand vector for cross product.
///  V2         I   Right hand vector for cross product.
///  VOUT       O   Cross product V1 x V2.
/// ```
///
/// # Detailed Input
///
/// ```text
///  V1,
///  V2       are two 3-dimensional vectors. Typically, these might
///           represent the (possibly unit) vector to a planet, Sun,
///           or a star which defines the orientation of axes of some
///           reference frame.
/// ```
///
/// # Detailed Output
///
/// ```text
///  VOUT     is the cross product of V1 and V2.
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
///  VCRSS calculates the three dimensional cross product of two
///  vectors according to the definition.
///
///  If V1 and V2 are large in magnitude (taken together, their
///  magnitude surpasses the limit allowed by the computer) then it
///  may be possible to generate a floating point overflow from an
///  intermediate computation even though the actual cross product may
///  be well within the range of double precision numbers. VCRSS does
///  NOT check the magnitude of V1 or V2 to insure that overflow will
///  not occur.
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
///  1) Define two sets of vectors and compute the cross product of
///     each vector in first set and the corresponding vector in
///     the second set.
///
///
///     Example code begins here.
///
///
///           PROGRAM VCRSS_EX1
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
///           DATA                  SETA / 0.D0,  1.D0,  0.D0,
///          .                             5.D0,  5.D0,  5.D0  /
///
///           DATA                  SETB / 1.D0,  0.D0,  0.D0,
///          .                            -1.D0, -1.D0, -1.D0  /
///
///     C
///     C     Calculate the cross product of each pair of vectors
///     C
///           DO I=1, SETSIZ
///
///              CALL VCRSS ( SETA(1,I), SETB(1,I), VOUT )
///
///              WRITE(*,'(A,3F5.1)') 'Vector A     : ',
///          .                        ( SETA(J,I), J=1,3 )
///              WRITE(*,'(A,3F5.1)') 'Vector B     : ',
///          .                        ( SETB(J,I), J=1,3 )
///              WRITE(*,'(A,3F5.1)') 'Cross product: ', VOUT
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
///     Vector A     :   0.0  1.0  0.0
///     Vector B     :   1.0  0.0  0.0
///     Cross product:   0.0  0.0 -1.0
///
///     Vector A     :   5.0  5.0  5.0
///     Vector B     :  -1.0 -1.0 -1.0
///     Cross product:   0.0  0.0  0.0
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  No checking of V1 or V2 is done to prevent floating point
///      overflow. The user is required to determine that the
///      magnitude of each component of the vectors is within an
///      appropriate range so as not to cause floating point overflow.
///      In almost every case there will be no problem and no checking
///      actually needs to be done.
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
pub fn vcrss(v1: &[f64; 3], v2: &[f64; 3], vout: &mut [f64; 3]) {
    VCRSS(v1, v2, vout);
}

//$Procedure VCRSS ( Vector cross product, 3 dimensions )
pub fn VCRSS(V1: &[f64], V2: &[f64], VOUT: &mut [f64]) {
    let V1 = DummyArray::new(V1, 1..=3);
    let V2 = DummyArray::new(V2, 1..=3);
    let mut VOUT = DummyArrayMut::new(VOUT, 1..=3);
    let mut VTEMP = StackArray::<f64, 3>::new(1..=3);

    //
    // Local variables
    //

    //
    // Calculate the cross product of V1 and V2, store in VTEMP
    //
    VTEMP[1] = ((V1[2] * V2[3]) - (V1[3] * V2[2]));
    VTEMP[2] = ((V1[3] * V2[1]) - (V1[1] * V2[3]));
    VTEMP[3] = ((V1[1] * V2[2]) - (V1[2] * V2[1]));
    //
    // Now move the result into VOUT
    //
    VOUT[1] = VTEMP[1];
    VOUT[2] = VTEMP[2];
    VOUT[3] = VTEMP[3];
    //
}
