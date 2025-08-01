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

            let mut clist = [Val::I(1), Val::I(2), Val::I(3), Val::I(1), Val::I(2)].into_iter();
            INDEXS
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { INDEXS }
    }
}

/// Derivative of a rotation matrix
///
/// Calculate the derivative with respect to the angle of rotation
/// of a 3x3 coordinate system rotation matrix generated by a
/// rotation of a specified angle about a specified axis.
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
///  ANGLE      I   Angle of rotation (radians).
///  IAXIS      I   Coordinate axis number (X=1, Y=2, Z=3).
///  DMOUT      O   Derivative of rotation matrix [ANGLE] w.r.t. angle
///                                                      IAXIS
/// ```
///
/// # Detailed Input
///
/// ```text
///  ANGLE    is the angle given in radians, through which the rotation
///           is performed.
///
///  IAXIS    is the coordinate axis number of the rotation. The X, Y,
///           and Z axes have indices 1, 2 and 3 respectively.
///
///           Together ANGLE and IAXIS define the coordinate system
///           rotation [ANGLE]     .
///                           IAXIS
/// ```
///
/// # Detailed Output
///
/// ```text
///  DMOUT    is the derivative of the rotation matrix with respect to
///           the angle of rotation. That is, DMOUT is the derivative
///           with respect to ANGLE of the matrix [ANGLE]     .
///                                                      IAXIS
///
///           (The rotation matrix being differentiated describes
///           the rotation of the COORDINATE system through ANGLE
///           radians about the axis whose index is IAXIS.)
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the axis index is not in the range 1 to 3, the error
///      SPICE(BADAXIS) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  A coordinate system rotation by ANGLE radians rotation about
///  the first, i.e. x-axis, [ANGLE]  is described by
///                                 1
///
///        |  1        0          0      |
///        |  0   cos(ANGLE) sin(ANGLE)  |
///        |  0  -sin(ANGLE) cos(ANGLE)  |
///
///  A coordinate system rotation by ANGLE radians rotation about the
///  second, i.e. y-axis, [ANGLE]  is described by
///                              2
///
///        |  cos(ANGLE)  0  -sin(ANGLE)  |
///        |      0       1        0      |
///        |  sin(ANGLE)  0   cos(ANGLE)  |
///
///  A coordinate system rotation by ANGLE radians rotation about
///  the third, i.e. z-axis, [ANGLE]  is described by
///                      3
///
///        |  cos(ANGLE) sin(ANGLE)   0   |
///        | -sin(ANGLE) cos(ANGLE)   0   |
///        |       0          0       1   |
///
///  The derivatives of these matrices are:
///
///  about the x-axis
///
///        |  0        0           0      |
///        |  0  -sin(ANGLE)  cos(ANGLE)  |
///        |  0  -cos(ANGLE) -sin(ANGLE)  |
///
///  about the y-axis
///
///        | -sin(ANGLE)  0  -cos(ANGLE)  |
///        |      0       0        0      |
///        |  cos(ANGLE)  0  -sin(ANGLE)  |
///
///  about the z-axis
///
///        | -sin(ANGLE)  cos(ANGLE)   0   |
///        | -cos(ANGLE) -sin(ANGLE)   0   |
///        |       0           0       0   |
/// ```
///
/// # Examples
///
/// ```text
///  If ROTATE is called from a FORTRAN program as follows:
///
///        CALL DROTAT (PI()/4, 3, DMOUT)
///
///  then DMOUT will be
///
///        |-SQRT(2)/2   SQRT(2)/2   0  |
///        |-SQRT(2)/2  -SQRT(2)/2   0  |
///        |     0           0       0  |
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 12-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 05-NOV-1990 (WLT)
/// ```
pub fn drotat(
    ctx: &mut SpiceContext,
    angle: f64,
    iaxis: i32,
    dmout: &mut [[f64; 3]; 3],
) -> crate::Result<()> {
    DROTAT(angle, iaxis, dmout.as_flattened_mut(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DROTAT ( Derivative of a rotation matrix )
pub fn DROTAT(
    ANGLE: f64,
    IAXIS: i32,
    DMOUT: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut DMOUT = DummyArrayMut2D::new(DMOUT, 1..=3, 1..=3);
    let mut S: f64 = 0.0;
    let mut C: f64 = 0.0;
    let mut I1: i32 = 0;
    let mut I2: i32 = 0;
    let mut I3: i32 = 0;

    //
    // First make sure the input axis is reasonable.
    //
    if ((IAXIS > 3) || (IAXIS < 1)) {
        CHKIN(b"DROTAT", ctx)?;
        SETMSG(b"The input axis is out of range.  Its value is #.", ctx);
        ERRINT(b"#", IAXIS, ctx);
        SIGERR(b"SPICE(BADAXIS)", ctx)?;
        CHKOUT(b"DROTAT", ctx)?;
        return Ok(());
    }

    //
    // Get the sine and cosine of ANGLE
    //
    S = f64::sin(ANGLE);
    C = f64::cos(ANGLE);

    //
    // Get indices for axes. The first index is for the axis of rotation.
    // The next two axes follow in right hand order (XYZ).
    //
    I1 = save.INDEXS[IAXIS];
    I2 = save.INDEXS[(IAXIS + 1)];
    I3 = save.INDEXS[(IAXIS + 2)];

    //
    // Construct the rotation matrix
    //
    DMOUT[[I1, I1]] = 0.0;
    DMOUT[[I2, I1]] = 0.0;
    DMOUT[[I3, I1]] = 0.0;
    DMOUT[[I1, I2]] = 0.0;
    DMOUT[[I2, I2]] = -S;
    DMOUT[[I3, I2]] = -C;
    DMOUT[[I1, I3]] = 0.0;
    DMOUT[[I2, I3]] = C;
    DMOUT[[I3, I3]] = -S;
    //
    Ok(())
}
