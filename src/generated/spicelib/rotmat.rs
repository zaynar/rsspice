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

/// Rotate a matrix
///
/// Apply a rotation of ANGLE radians about axis IAXIS to a matrix.
/// This rotation is thought of as rotating the coordinate system.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  M1         I   Matrix to be rotated.
///  ANGLE      I   Angle of rotation (radians).
///  IAXIS      I   Axis of rotation (X=1, Y=2, Z=3).
///  MOUT       O   Resulting rotated matrix [ANGLE]      * M1
///                                                IAXIS
/// ```
///
/// # Detailed Input
///
/// ```text
///  M1       is a 3x3 matrix to which a rotation is to be applied. In
///           matrix algebra, the components of the matrix are relevant
///           in one particular coordinate system. Applying ROTMAT
///           changes the components of M1 so that they are relevant to
///           a rotated coordinate system.
///
///  ANGLE    is the angle in radians through which the original
///           coordinate system is to be rotated.
///
///  IAXIS    is the index for the axis of the original coordinate
///           system about which the rotation by ANGLE is to be
///           performed. IAXIS = 1,2 or 3 designates the X-, Y- or
///           Z-axis, respectively.
/// ```
///
/// # Detailed Output
///
/// ```text
///  MOUT     is the matrix resulting from the application of the
///           specified rotation to the input matrix M1. If
///
///              [ANGLE]
///                     IAXIS
///
///           denotes the rotation matrix by ANGLE radians about IAXIS,
///           (refer to the routine ROTATE) then MOUT is given by the
///           following matrix equation:
///
///              MOUT = [ANGLE]      * M1
///                            IAXIS
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If the axis index is not in the range 1 to 3, it will be
///      treated the same as that integer 1, 2, or 3 that is congruent
///      to it mod 3.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that to rotate a set of inertial axes to body fixed
///  axes, one must first roll the coordinate axes about the x-axis by
///  angle R to get x', y', z'. From this one must pitch about the y'
///  axis by angle P to get x'', y'', z''.  And finally yaw the x'',
///  y'', z'' about the z'' axis by angle Y to obtain the
///  transformation to bodyfixed coordinates. If ID is the identity
///  matrix, then the following code fragment generates the
///  transformation from inertial to body fixed.
///
///     CALL ROTMAT ( ID, R,     1,     M1   )
///     CALL ROTMAT ( M1, P,     2,     M2   )
///     CALL ROTMAT ( M2, Y,     3,     TIBF )
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
/// -    SPICELIB Version 1.1.0, 27-MAY-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Reformatted
///         arguments' description.
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
///
/// # Revisions
///
/// ```text
/// -    Beta Version 1.1.0, 3-JAN-1989 (WLT)
///
///      Upgrade the routine to work with negative axis indexes. Also take
///      care of the funky way the indices (other than the input) were
///      obtained via the MOD function. It works but isn't as clear
///      (or fast) as just reading the axes from data.
/// ```
pub fn rotmat(
    ctx: &mut SpiceContext,
    m1: &[[f64; 3]; 3],
    angle: f64,
    iaxis: i32,
    mout: &mut [[f64; 3]; 3],
) {
    ROTMAT(
        m1.as_flattened(),
        angle,
        iaxis,
        mout.as_flattened_mut(),
        ctx.raw_context(),
    );
}

//$Procedure ROTMAT ( Rotate a matrix )
pub fn ROTMAT(M1: &[f64], ANGLE: f64, IAXIS: i32, MOUT: &mut [f64], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let M1 = DummyArray2D::new(M1, 1..=3, 1..=3);
    let mut MOUT = DummyArrayMut2D::new(MOUT, 1..=3, 1..=3);
    let mut S: f64 = 0.0;
    let mut C: f64 = 0.0;
    let mut TEMP: i32 = 0;
    let mut I1: i32 = 0;
    let mut I2: i32 = 0;
    let mut I3: i32 = 0;
    let mut PRODM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);

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
    TEMP = intrinsics::MOD((intrinsics::MOD(IAXIS, 3) + 3), 3);

    I1 = save.INDEXS[(TEMP + 1)];
    I2 = save.INDEXS[(TEMP + 2)];
    I3 = save.INDEXS[(TEMP + 3)];
    //
    // Calculate the output matrix column by column
    //
    for I in 1..=3 {
        PRODM[[I1, I]] = M1[[I1, I]];
        PRODM[[I2, I]] = ((C * M1[[I2, I]]) + (S * M1[[I3, I]]));
        PRODM[[I3, I]] = (-(S * M1[[I2, I]]) + (C * M1[[I3, I]]));
    }
    //
    // Move the buffered matrix into MOUT.
    //
    MOVED(PRODM.as_slice(), 9, MOUT.as_slice_mut());
    //
}
