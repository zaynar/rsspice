//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Rotation axis of a matrix
///
/// Compute the axis of the rotation given by an input matrix
/// and the angle of the rotation about that axis.
///
/// # Required Reading
///
/// * [ROTATION](crate::required_reading::rotation)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  MATRIX     I   3x3 rotation matrix in double precision.
///  AXIS       O   Axis of the rotation.
///  ANGLE      O   Angle through which the rotation is performed.
/// ```
///
/// # Detailed Input
///
/// ```text
///  MATRIX   is a 3x3 rotation matrix in double precision.
/// ```
///
/// # Detailed Output
///
/// ```text
///  AXIS     is a unit vector pointing along the axis of the
///           rotation. In other words, AXIS is a unit eigenvector
///           of the input matrix, corresponding to the eigenvalue
///           1. If the input matrix is the identity matrix, AXIS
///           will be the vector (0, 0, 1). If the input rotation is
///           a rotation by PI radians, both AXIS and -AXIS may be
///           regarded as the axis of the rotation.
///
///  ANGLE    is the angle between V and MATRIX*V for any non-zero
///           vector V orthogonal to AXIS. Angle is given in
///           radians. The angle returned will be in the range from
///           0 to PI.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input matrix is not a rotation matrix (where a fairly
///      loose tolerance is used to check this), an error is signaled
///      by a routine in the call tree of this routine.
///
///  2)  If the input matrix is the identity matrix, this routine
///      returns an angle of 0.0, and an axis of ( 0.0, 0.0, 1.0 ).
/// ```
///
/// # Particulars
///
/// ```text
///  Every rotation matrix has an axis A such any vector, V, parallel
///  to that axis satisfies the equation
///
///     V = MATRIX * V
///
///  This routine returns a unit vector AXIS parallel to the axis of
///  the input rotation matrix. Moreover for any vector W orthogonal
///  to the axis of the rotation
///
///     AXIS  and  W x MATRIX*W
///
///     (where "x" denotes the cross product operation)
///
///  will be positive scalar multiples of one another (at least to
///  within the ability to make such computations with double
///  precision arithmetic, and under the assumption that the MATRIX
///  does not represent a rotation by zero or Pi radians).
///
///  The angle returned will be the angle between W and MATRIX*W for
///  any vector orthogonal to AXIS.
///
///  If the input matrix is a rotation by 0 or PI radians some choice
///  must be made for the AXIS returned. In the case of a rotation by
///  0 radians, AXIS is along the positive z-axis. In the case of a
///  rotation by 180 degrees, two choices are
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
///  1) Given an axis and an angle of rotation about that axis,
///     determine the rotation matrix. Using this matrix as input,
///     compute the axis and angle of rotation, and verify that
///     the later are equivalent by subtracting the original matrix
///     and the one resulting from using the computed axis and angle
///     of rotation on the AXISAR call.
///
///
///     Example code begins here.
///
///
///           PROGRAM RAXISA_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      TWOPI
///
///     C
///     C     Local variables
///     C
///           DOUBLE PRECISION      ANGLE
///           DOUBLE PRECISION      ANGOUT
///           DOUBLE PRECISION      AXIS   ( 3    )
///           DOUBLE PRECISION      AXOUT  ( 3    )
///           DOUBLE PRECISION      R      ( 3, 3 )
///           DOUBLE PRECISION      ROUT   ( 3, 3 )
///
///           INTEGER               I
///
///     C
///     C     Define an axis and an angle for rotation.
///     C
///           DATA                  AXIS  /  1.D0, 2.D0, 3.D0  /
///
///           ANGLE = 0.1D0 * TWOPI()
///
///     C
///     C     Determine the rotation matrix.
///     C
///           CALL AXISAR ( AXIS, ANGLE, R )
///
///     C
///     C     Now calculate the rotation axis and angle based on the
///     C     matrix as input.
///     C
///           CALL RAXISA ( R, AXOUT, ANGOUT )
///
///           WRITE(*,'(A,3F12.8)') 'Axis :', AXOUT
///           WRITE(*,'(A,F12.8)')  'Angle:', ANGOUT
///           WRITE(*,*) ' '
///
///     C
///     C     Now input the AXOUT and ANGOUT to AXISAR to
///     C     compare against the original rotation matrix R.
///     C
///           WRITE(*,'(A)') 'Difference between input and output '
///          .           //  'matrices:'
///
///           CALL AXISAR ( AXOUT, ANGOUT, ROUT )
///
///           DO I = 1, 3
///
///              WRITE(*,'(3F20.16)') ROUT(I,1) - R(I,1),
///          .                        ROUT(I,2) - R(I,2),
///          .                        ROUT(I,3) - R(I,3)
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
///     Axis :  0.26726124  0.53452248  0.80178373
///     Angle:  0.62831853
///
///     Difference between input and output matrices:
///      -0.0000000000000001  0.0000000000000000  0.0000000000000000
///       0.0000000000000001 -0.0000000000000001  0.0000000000000000
///       0.0000000000000000  0.0000000000000001  0.0000000000000000
///
///
///     Note, the zero matrix is accurate to round-off error. A
///     numerical demonstration of equality.
///
///
///  2) This routine can be used to numerically approximate the
///     instantaneous angular velocity vector of a rotating object.
///
///     Suppose that R(t) is the rotation matrix whose columns
///     represent the inertial pointing vectors of the body-fixed axes
///     of an object at time t.
///
///     Then the angular velocity vector points along the vector given
///     by:
///
///                             T
///         limit  AXIS( R(t+h)R )
///         h-->0
///
///     And the magnitude of the angular velocity at time t is given
///     by:
///
///                            T
///        d ANGLE ( R(t+h)R(t) )
///        ----------------------   at   h = 0
///                  dh
///
///     This code example computes the instantaneous angular velocity
///     vector of the Earth at 2000 Jan 01 12:00:00 TDB.
///
///     Use the PCK kernel below to load the required triaxial
///     ellipsoidal shape model and orientation data for the Earth.
///
///        pck00010.tpc
///
///
///     Example code begins here.
///
///
///           PROGRAM RAXISA_EX2
///           IMPLICIT NONE
///
///     C
///     C     Local variables
///     C
///           DOUBLE PRECISION      ANGLE
///           DOUBLE PRECISION      ANGVEL ( 3    )
///           DOUBLE PRECISION      AXIS   ( 3    )
///           DOUBLE PRECISION      INFROT ( 3, 3 )
///           DOUBLE PRECISION      H
///           DOUBLE PRECISION      RT     ( 3, 3 )
///           DOUBLE PRECISION      RTH    ( 3, 3 )
///           DOUBLE PRECISION      T
///
///     C
///     C     Load a PCK file containing a triaxial
///     C     ellipsoidal shape model and orientation
///     C     data for the Earth.
///     C
///           CALL FURNSH ( 'pck00010.tpc' )
///
///     C
///     C     Load time into the double precision variable T
///     C     and the delta time (1 ms) into the double precision
///     C     variable H
///     C
///           T = 0.D0
///           H = 1D-3
///
///     C
///     C     Get the rotation matrices from IAU_EARTH to J2000
///     C     at T and T+H.
///     C
///           CALL PXFORM ( 'IAU_EARTH', 'J2000', T,   RT  )
///           CALL PXFORM ( 'IAU_EARTH', 'J2000', T+H, RTH )
///
///     C
///     C     Compute the infinitesimal rotation R(t+h)R(t)**T
///     C
///           CALL MXMT ( RTH, RT, INFROT )
///
///     C
///     C     Compute the AXIS and ANGLE of the infinitesimal rotation
///     C
///           CALL RAXISA ( INFROT, AXIS, ANGLE )
///
///     C
///     C     Scale AXIS to get the angular velocity vector
///     C
///           CALL VSCL ( ANGLE/H, AXIS, ANGVEL )
///
///     C
///     C     Output the results.
///     C
///           WRITE(*,*) 'Instantaneous angular velocity vector:'
///           WRITE(*,'(3F15.10)') ANGVEL
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Instantaneous angular velocity vector:
///        0.0000000000   0.0000000000   0.0000729212
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  If the input matrix is not a rotation matrix but is close
///      enough to pass the tests this routine performs on it, no error
///      will be signaled, but the results may have poor accuracy.
///
///  2)  The input matrix is taken to be an object that acts on
///      (rotates) vectors---it is not regarded as a coordinate
///      transformation. To find the axis and angle of a coordinate
///      transformation, input the transpose of that matrix to this
///      routine.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.2.0, 05-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///         Added complete code examples.
///
/// -    SPICELIB Version 2.1.2, 02-JAN-2008 (EDW)
///
///         Minor edit to the ANGLE declaration strictly
///         identifying the constant as a double.
///
///         From:
///
///            ANGLE = 2.0 * DATAN2( VNORM(Q(1)), Q(0) )
///
///         To:
///
///            ANGLE = 2.D0 * DATAN2( VNORM(Q(1)), Q(0) )
///
/// -    SPICELIB Version 2.1.1, 05-JAN-2005 (NJB)
///
///         Minor edits and formatting changes were made.
///
/// -    SPICELIB Version 2.1.0, 30-MAY-2002 (FST)
///
///         This routine now participates in error handling properly.
///
/// -    SPICELIB Version 2.0.0, 19-SEP-1999 (WLT)
///
///         The routine was re-written so as to avoid the numerical
///         instabilities present in the previous implementation for
///         rotations very near zero or 180 degrees.
///
/// -    SPICELIB Version 1.1.0, 29-FEB-1996 (KRG)
///
///         The declaration for the SPICELIB function PI is now
///         preceded by an EXTERNAL statement declaring PI to be an
///         external function. This removes a conflict with any
///         compilers that have a PI intrinsic function.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 2.1.0, 30-MAY-2002 (FST)
///
///         Calls to CHKIN and CHKOUT in the standard SPICE error
///         handling style were added. Versions prior to 2.0.0
///         were error free, however the call to M2Q introduced in
///         version 2.0.0 signals an error if the input matrix is
///         not sufficiently close to a rotation.
///
///         Additionally, FAILED is now checked after the call to
///         M2Q. This prevents garbage from being placed into the
///         output arguments.
///
/// -    Beta Version 1.1.0, 03-JAN-1989 (WLT)
///
///         Even though the routine stipulates that the input matrix
///         should be a rotation matrix, it might not be. As a result
///         we could have negative numbers showing up where we need
///         to take square roots. This fix simply bounds these values
///         so that Fortran intrinsics always get reasonable input values.
///
///         Add and example to the header.
/// ```
pub fn raxisa(
    ctx: &mut SpiceContext,
    matrix: &[[f64; 3]; 3],
    axis: &mut [f64; 3],
    angle: &mut f64,
) -> crate::Result<()> {
    RAXISA(matrix.as_flattened(), axis, angle, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure RAXISA ( Rotation axis of a matrix )
pub fn RAXISA(
    MATRIX: &[f64],
    AXIS: &mut [f64],
    ANGLE: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let MATRIX = DummyArray2D::new(MATRIX, 1..=3, 1..=3);
    let mut AXIS = DummyArrayMut::new(AXIS, 1..=3);
    let mut Q = StackArray::<f64, 4>::new(0..=3);

    //
    // SPICELIB functions
    //

    //
    // Local Variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"RAXISA", ctx)?;
    }

    //
    // Construct the quaternion corresponding to the input rotation
    // matrix
    //
    M2Q(MATRIX.as_slice(), Q.as_slice_mut(), ctx)?;

    //
    // Check FAILED and return if an error has occurred.
    //
    if FAILED(ctx) {
        CHKOUT(b"RAXISA", ctx)?;
        return Ok(());
    }

    //
    // The quaternion we've just constructed is of the form:
    //
    //     cos(ANGLE/2) + sin(ANGLE/2) * AXIS
    //
    // We take a few precautions to handle the case of an identity
    // rotation.
    //
    if VZERO(Q.subarray(1)) {
        *ANGLE = 0 as f64;
        AXIS[1] = 0.0;
        AXIS[2] = 0.0;
        AXIS[3] = 1.0;
    } else if (Q[0] == 0.0) {
        *ANGLE = PI(ctx);
        AXIS[1] = Q[1];
        AXIS[2] = Q[2];
        AXIS[3] = Q[3];
    } else {
        VHAT(Q.subarray(1), AXIS.as_slice_mut());
        *ANGLE = (2.0 * f64::atan2(VNORM(Q.subarray(1)), Q[0]));
    }

    CHKOUT(b"RAXISA", ctx)?;
    Ok(())
}
