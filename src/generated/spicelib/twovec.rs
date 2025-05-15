//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

struct SaveVars {
    SEQNCE: StackArray<i32, 5>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SEQNCE = StackArray::<i32, 5>::new(1..=5);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(1), Val::I(2), Val::I(3), Val::I(1), Val::I(2)].into_iter();
            SEQNCE
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { SEQNCE }
    }
}

/// Two vectors defining an orthonormal frame
///
/// Find the transformation to the right-handed frame having a
/// given vector as a specified axis and having a second given
/// vector lying in a specified coordinate plane.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  -------------------------------------------------
///  AXDEF      I   Vector defining a principal axis.
///  INDEXA     I   Principal axis number of AXDEF (X=1, Y=2, Z=3).
///  PLNDEF     I   Vector defining (with AXDEF) a principal plane.
///  INDEXP     I   Second axis number (with INDEXA) of principal
///                 plane.
///  MOUT       O   Output rotation matrix.
/// ```
///
/// # Detailed Input
///
/// ```text
///  AXDEF    is a vector defining one of the principle axes of a
///           coordinate frame.
///
///  INDEXA   is a number that determines which of the three
///           coordinate axes contains AXDEF.
///
///           If INDEXA is 1 then AXDEF defines the X axis of the
///           coordinate frame.
///
///           If INDEXA is 2 then AXDEF defines the Y axis of the
///           coordinate frame.
///
///           If INDEXA is 3 then AXDEF defines the Z axis of the
///           coordinate frame.
///
///  PLNDEF   is a vector defining (with AXDEF) a principal plane of
///           the coordinate frame. AXDEF and PLNDEF must be
///           linearly independent.
///
///  INDEXP   is the second axis of the principal frame determined
///           by AXDEF and PLNDEF.  INDEXA, INDEXP must be different
///           and be integers from 1 to 3.
///
///           If INDEXP is 1, the second axis of the principal
///           plane is the X-axis.
///
///           If INDEXP is 2, the second axis of the principal
///           plane is the Y-axis.
///
///           If INDEXP is 3, the second axis of the principal plane
///           is the Z-axis.
/// ```
///
/// # Detailed Output
///
/// ```text
///  MOUT     is a rotation matrix that transforms coordinates given
///           in the input frame to the frame determined by AXDEF,
///           PLNDEF, INDEXA and INDEXP.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If INDEXA or INDEXP is not in the set {1,2,3}, the error
///      SPICE(BADINDEX) is signaled.
///
///  2)  If INDEXA and INDEXP are the same, the error
///      SPICE(UNDEFINEDFRAME) is signaled.
///
///  3)  If the cross product of the vectors AXDEF and PLNDEF is zero,
///      the error SPICE(DEPENDENTVECTORS) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  Given two linearly independent vectors there is a unique
///  right-handed coordinate frame having:
///
///     AXDEF lying along the INDEXA axis.
///
///     PLNDEF lying in the INDEXA-INDEXP coordinate plane.
///
///  This routine determines the transformation matrix that transforms
///  from coordinates used to represent the input vectors to the
///  the system determined by AXDEF and PLNDEF. Thus a vector
///  (x,y,z) in the input coordinate system will have coordinates
///
///                  t
///     MOUT* (x,y,z)
///
///  in the frame determined by AXDEF and PLNDEF.
/// ```
///
/// # Examples
///
/// ```text
///  The rotation matrix TICC from inertial to Sun-Canopus
///  (celestial) coordinates is found by the call
///
///     CALL TWOVEC (Sun vector, 3, Canopus vector, 1, TICC)
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
/// -    SPICELIB Version 1.2.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section.
///
/// -    SPICELIB Version 1.1.0, 31-AUG-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in VSCL call.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WMO) (WLT)
/// ```
pub fn twovec(
    ctx: &mut SpiceContext,
    axdef: &[f64; 3],
    indexa: i32,
    plndef: &[f64; 3],
    indexp: i32,
    mout: &mut [[f64; 3]; 3],
) -> crate::Result<()> {
    TWOVEC(
        axdef,
        indexa,
        plndef,
        indexp,
        mout.as_flattened_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure TWOVEC ( Two vectors defining an orthonormal frame )
pub fn TWOVEC(
    AXDEF: &[f64],
    INDEXA: i32,
    PLNDEF: &[f64],
    INDEXP: i32,
    MOUT: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let AXDEF = DummyArray::new(AXDEF, 1..=3);
    let PLNDEF = DummyArray::new(PLNDEF, 1..=3);
    let mut MOUT = DummyArrayMut2D::new(MOUT, 1..=3, 1..=3);
    let mut MTEMP = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut I1: i32 = 0;
    let mut I2: i32 = 0;
    let mut I3: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Standard SPICE error handling
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"TWOVEC", ctx)?;
    }

    //
    // Check for obvious bad inputs.
    //
    if ((intrinsics::MAX0(&[INDEXP, INDEXA]) > 3) || (intrinsics::MIN0(&[INDEXP, INDEXA]) < 1)) {
        SETMSG(b"The definition indexes must lie in the range from 1 to 3.  The value of INDEXA was #. The value of INDEXP was #. ", ctx);

        ERRINT(b"#", INDEXA, ctx);
        ERRINT(b"#", INDEXP, ctx);
        SIGERR(b"SPICE(BADINDEX)", ctx)?;
        CHKOUT(b"TWOVEC", ctx)?;
        return Ok(());
    } else if (INDEXA == INDEXP) {
        SETMSG(b"The values of INDEXA and INDEXP were the same, namely #.  They are required to be different.", ctx);

        ERRINT(b"#", INDEXA, ctx);
        SIGERR(b"SPICE(UNDEFINEDFRAME)", ctx)?;
        CHKOUT(b"TWOVEC", ctx)?;
        return Ok(());
    }

    //
    // Get indices for right-handed axes
    //
    // First AXDEF ...
    //
    I1 = INDEXA;
    //
    // ... then the other two.
    //
    I2 = save.SEQNCE[(INDEXA + 1)];
    I3 = save.SEQNCE[(INDEXA + 2)];

    //
    // Row I1 contains normalized AXDEF (store in columns for now)
    //
    VHAT(AXDEF.as_slice(), MOUT.subarray_mut([1, I1]));

    //
    // Obtain rows I2 and I3 using cross products.  Which order to use
    // depends on whether INDEXP = I2 (next axis in right-handed order)
    // or INDEXP = I3 (previous axis in right-handed order).
    //
    if (INDEXP == I2) {
        UCRSS(
            AXDEF.as_slice(),
            PLNDEF.as_slice(),
            MOUT.subarray_mut([1, I3]),
        );
        UCRSS(
            &MOUT.subarray([1, I3]).to_vec(),
            AXDEF.as_slice(),
            MOUT.subarray_mut([1, I2]),
        );
    } else {
        UCRSS(
            PLNDEF.as_slice(),
            AXDEF.as_slice(),
            MOUT.subarray_mut([1, I2]),
        );
        UCRSS(
            AXDEF.as_slice(),
            &MOUT.subarray([1, I2]).to_vec(),
            MOUT.subarray_mut([1, I3]),
        );
    }

    //
    // Finally, check to see that we actually got something non-zero
    // in one of the one columns of MOUT(1,I2) and MOUT(1,I3) (we need
    // only check one of them since they are related by a cross product).
    //
    if (((MOUT[[1, I2]] == 0.0) && (MOUT[[2, I2]] == 0.0)) && (MOUT[[3, I2]] == 0.0)) {
        SETMSG(
            b"The input vectors AXDEF and PLNDEF are linearly dependent.",
            ctx,
        );
        SIGERR(b"SPICE(DEPENDENTVECTORS)", ctx)?;
    }
    //
    // Transpose MOUT.
    //
    XPOSE(MOUT.as_slice(), MTEMP.as_slice_mut());
    MOVED(MTEMP.as_slice(), 9, MOUT.as_slice_mut());

    CHKOUT(b"TWOVEC", ctx)?;
    Ok(())
}
