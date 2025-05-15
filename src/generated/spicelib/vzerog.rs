//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Is a vector the zero vector? -- general dim.
///
/// Indicate whether an n-dimensional vector is the zero vector.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  V          I   Vector to be tested.
///  NDIM       I   Dimension of V.
///
///  The function returns the value .TRUE. if and only if V is the
///  zero vector.
/// ```
///
/// # Detailed Input
///
/// ```text
///  V,
///  NDIM     are, respectively, an n-dimensional vector and its
///           dimension.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the value .TRUE. if and only if V is the
///  zero vector.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  When NDIM is non-positive, this function returns the value
///      .FALSE. (A vector of non-positive dimension cannot be the
///      zero vector.)
/// ```
///
/// # Particulars
///
/// ```text
///  This function has the same truth value as the logical expression
///
///     VNORMG ( V, NDIM )  .EQ.  0.D0
///
///  Replacing the above expression by
///
///     VZEROG ( V, NDIM )
///
///  has several advantages: the latter expresses the test more
///  clearly, looks better, and doesn't go through the work of scaling,
///  squaring, taking a square root, and re-scaling (all of which
///  VNORMG must do) just to find out that a vector is non-zero.
///
///  A related function is VZERO, which accepts three-dimensional
///  vectors.
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
///  1) Given a set of n-dimensional vectors, check which ones are
///     the zero vector.
///
///
///     Example code begins here.
///
///
///           PROGRAM VZEROG_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions.
///     C
///           LOGICAL               VZEROG
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
///           DOUBLE PRECISION      V    ( NDIM, SETSIZ )
///
///           INTEGER               I
///           INTEGER               J
///
///     C
///     C     Define the vector set.
///     C
///           DATA                  V   /
///          .                          0.D0,  0.D0,  0.D0,  2.D-7,
///          .                          0.D0,  0.D0,  0.D0,  0.D0  /
///
///     C
///     C     Check each n-dimensional vector within the set.
///     C
///           DO I=1, SETSIZ
///
///     C
///     C        Check if the I'th vector is the zero vector.
///     C
///              WRITE(*,*)
///              WRITE(*,'(A,4F11.7)') 'Input vector: ',
///          .                         ( V(J,I), J=1,NDIM )
///
///              IF ( VZEROG ( V(1,I), NDIM ) ) THEN
///
///                 WRITE(*,'(A)') '   The zero vector.'
///
///              ELSE
///
///                 WRITE(*,'(A)') '   Not all elements of the '
///          .                  // 'vector are zero.'
///
///              END IF
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
///     Input vector:   0.0000000  0.0000000  0.0000000  0.0000002
///        Not all elements of the vector are zero.
///
///     Input vector:   0.0000000  0.0000000  0.0000000  0.0000000
///        The zero vector.
///
///
///  2) Define a unit quaternion and confirm that it is non-zero
///     before converting it to a rotation matrix.
///
///
///     Example code begins here.
///
///
///           PROGRAM VZEROG_EX2
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions.
///     C
///           DOUBLE PRECISION      VNORMG
///           LOGICAL               VZEROG
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      Q    ( 0 : 3 )
///           DOUBLE PRECISION      M    ( 3,  3 )
///           DOUBLE PRECISION      S
///
///           INTEGER               I
///           INTEGER               J
///
///     C
///     C     Define a unit quaternion.
///     C
///           S = SQRT( 2.D0 ) / 2.D0
///
///           Q(0) = S
///           Q(1) = 0.D0
///           Q(2) = 0.D0
///           Q(3) = -S
///
///           WRITE(*,'(A,4F12.7)') 'Quaternion :', Q
///
///     C
///     C     Confirm that it is non-zero and
///     C
///           IF ( VZEROG ( Q, 4 ) ) THEN
///
///              WRITE(*,*) '   Quaternion is the zero vector.'
///
///           ELSE
///
///     C
///     C        Confirm q satisfies ||Q|| = 1.
///     C
///              WRITE(*,'(A,F12.7)') 'Norm       :', VNORMG ( Q, 4 )
///
///     C
///     C        Convert the quaternion to a matrix form.
///     C
///              CALL Q2M ( Q, M )
///
///              WRITE(*,'(A)') 'Matrix form:'
///              DO I = 1, 3
///
///                 WRITE(*,'(3F12.7)') ( M(I,J), J=1,3 )
///
///              END DO
///
///           END IF
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Quaternion :   0.7071068   0.0000000   0.0000000  -0.7071068
///     Norm       :   1.0000000
///     Matrix form:
///        0.0000000   1.0000000   0.0000000
///       -1.0000000   0.0000000  -0.0000000
///       -0.0000000   0.0000000   1.0000000
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
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
///         code examples.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 18-JUL-1990 (NJB) (IMU)
/// ```
pub fn vzerog(v: &[f64], ndim: i32) -> bool {
    let ret = VZEROG(v, ndim);
    ret
}

//$Procedure VZEROG ( Is a vector the zero vector? -- general dim. )
pub fn VZEROG(V: &[f64], NDIM: i32) -> bool {
    let V = DummyArray::new(V, 1..);
    let mut VZEROG: bool = false;

    //
    // Local variables
    //

    //
    // Leave as soon as we find a non-zero component.  If we get through
    // the loop, we have a zero vector, as long as the vector's dimension
    // is valid.
    //
    for I in 1..=NDIM {
        if (V[I] != 0.0) {
            VZEROG = false;
            return VZEROG;
        }
    }

    //
    // We have a zero vector if and only if the vector's dimension is at
    // least 1.
    //
    VZEROG = (NDIM >= 1);

    VZEROG
}
