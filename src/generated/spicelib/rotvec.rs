//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

struct SaveVars {
    INDEXS: StackArray<i32, 5>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut INDEXS = StackArray::<i32, 5>::new(1..=5);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(3), Val::I(1), Val::I(2), Val::I(3), Val::I(1)].into_iter();
            INDEXS
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { INDEXS }
    }
}

/// Transform a vector via a rotation
///
/// Transform a vector to a new reference frame rotated by ANGLE
/// radians about axis IAXIS. This transformation rotates V1 by
/// -ANGLE radians about the specified axis.
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
///  V1         I   Vector whose coordinate system is to be rotated.
///  ANGLE      I   Angle of rotation in radians.
///  IAXIS      I   Axis of rotation (X=1, Y=2, Z=3).
///  VOUT       O   Resulting vector expressed in the new frame.
/// ```
///
/// # Detailed Input
///
/// ```text
///  V1       is a vector (typically representing a vector fixed in
///           inertial space) which is to be expressed in another
///           reference frame. The vector remains fixed but the
///           reference frame changes.
///
///  ANGLE    is an angle given in radians, through which the rotation
///           is performed.
///
///  IAXIS    is the index of the axis of rotation. The X, Y, and Z
///           axes have indices 1, 2 and 3 respectively.
/// ```
///
/// # Detailed Output
///
/// ```text
///  VOUT     is the vector expressed in the new reference frame
///           specified by the angle of rotation and axis. If
///
///              M = [ANGLE]
///                         IAXIS
///
///           represents the rotation matrix described by the ANGLE
///           and IAXIS, (refer to the routine ROTATE) then
///
///              VOUT =  M * V1  = [ANGLE]      * V1
///                                       IAXIS
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If the IAXIS index is not in the range 1 to 3, it will be
///      treated the same as that integer 1, 2, or 3 that is congruent
///      to it mod 3.
/// ```
///
/// # Particulars
///
/// ```text
///  A rotation about the first, i.e. X-axis, is described by
///
///     .-                            -.
///     |   1        0          0      |
///     |   0   cos(theta) sin(theta)  |
///     |   0  -sin(theta) cos(theta)  |
///     `-                            -'
///
///  A rotation about the second, i.e. Y-axis, is described by
///
///     .-                            -.
///     |  cos(theta)  0  -sin(theta)  |
///     |      0       1        0      |
///     |  sin(theta)  1   cos(theta)  |
///     `-                            -'
///
///  A rotation about the third, i.e. Z-axis, is described by
///
///     .-                            -.
///     |  cos(theta) sin(theta)   0   |
///     | -sin(theta) cos(theta)   0   |
///     |       0          0       1   |
///     `-                            -'
///
///  ROTVEC decides which form is appropriate according to the value
///  of IAXIS and applies the rotation to the input vector.
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
///  1) Apply a rotation of -45.D0 degrees about the +Z axis to
///     a 3 dimensional vector.
///
///     Example code begins here.
///
///
///           PROGRAM ROTVEC_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      PI
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      ANGLE
///           DOUBLE PRECISION      V1    ( 3 )
///           DOUBLE PRECISION      VOUT  ( 3 )
///
///           INTEGER               I
///           INTEGER               IAXIS
///
///     C
///     C     Input values.
///     C
///           DATA                  V1  / 1.414D0, 0.D0, 0.D0 /
///
///           ANGLE = PI( )/4
///           IAXIS = 3
///
///     C
///     C     Rotate V1 by ANGLE radians about IAXIS
///     C
///           CALL ROTVEC (V1, ANGLE, IAXIS, VOUT)
///
///           WRITE(*,'(A,3F10.3)') 'Input vector  :',
///          .                        ( V1(I), I=1,3 )
///           WRITE(*,'(A,3F10.3)') 'Rotated vector:',
///          .                      ( VOUT(I), I=1,3 )
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Input vector  :     1.414     0.000     0.000
///     Rotated vector:     1.000    -1.000     0.000
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
///         Edited the header to comply with NAIF standard. Created
///         complete code example from existing code fragments.
///
///         Changed "coordinate system" to "reference frame" to follow
///         NAIF conventions. Added ROTATION required reading.
///
/// -    SPICELIB Version 1.0.3, 23-APR-2010 (NJB)
///
///         Header correction: assertions that the output
///         can overwrite the input have been removed.
///
/// -    SPICELIB Version 1.0.2, 04-OCT-1999 (NJB)
///
///         Procedure line and abstract were changed to dispel the
///         impression that the input vector is rotated by +ANGLE
///         radians about the specified axis.
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
/// -    Beta Version 1.1.0, 04-JAN-1989 (WLT)
///
///         Upgrade the routine to work with negative axis indexes. Also
///         take care of the funky way the indices (other than the input)
///         were obtained via the MOD function. It works but isn't as
///         clear (or fast) as just reading the axes from data.
/// ```
pub fn rotvec(ctx: &mut SpiceContext, v1: &[f64; 3], angle: f64, iaxis: i32, vout: &mut [f64; 3]) {
    ROTVEC(v1, angle, iaxis, vout, ctx.raw_context());
}

//$Procedure ROTVEC ( Transform a vector via a rotation )
pub fn ROTVEC(V1: &[f64], ANGLE: f64, IAXIS: i32, VOUT: &mut [f64], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let V1 = DummyArray::new(V1, 1..=3);
    let mut VOUT = DummyArrayMut::new(VOUT, 1..=3);
    let mut S: f64 = 0.0;
    let mut C: f64 = 0.0;
    let mut TMP: i32 = 0;
    let mut I1: i32 = 0;
    let mut I2: i32 = 0;
    let mut I3: i32 = 0;
    let mut TEMP = StackArray::<f64, 3>::new(1..=3);

    //
    // Local variables
    //

    //
    // Get the sine and cosine of ANGLE
    //
    S = f64::sin(ANGLE);
    C = f64::cos(ANGLE);

    //
    // Get indices for axes. The first index is for the axis of rotation.
    // The next two axes follow in right hand order (XYZ).  First get the
    // non-negative value of IAXIS mod 3 .
    //
    TMP = intrinsics::MOD((intrinsics::MOD(IAXIS, 3) + 3), 3);

    I1 = save.INDEXS[(TMP + 1)];
    I2 = save.INDEXS[(TMP + 2)];
    I3 = save.INDEXS[(TMP + 3)];

    //
    // The coordinate along the axis of rotation does not change.
    //
    TEMP[1] = V1[I1];
    TEMP[2] = ((C * V1[I2]) + (S * V1[I3]));
    TEMP[3] = (-(S * V1[I2]) + (C * V1[I3]));

    //
    // Move the buffered vector to the output
    //
    VOUT[I1] = TEMP[1];
    VOUT[I2] = TEMP[2];
    VOUT[I3] = TEMP[3];
}
