//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

struct SaveVars {
    M: f64,
    A1: f64,
    B1: f64,
    C1: f64,
    MSSG: ActualCharArray,
    BAD: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut M: f64 = 0.0;
        let mut A1: f64 = 0.0;
        let mut B1: f64 = 0.0;
        let mut C1: f64 = 0.0;
        let mut MSSG = ActualCharArray::new(32, 1..=7);
        let mut BAD: i32 = 0;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"Axis A was nonpositive."),
                Val::C(b"Axis B was nonpositive."),
                Val::C(b"Axes A and B were nonpositive."),
                Val::C(b"Axis C was nonpositive."),
                Val::C(b"Axes A and C were nonpositive."),
                Val::C(b"Axes B and C were nonpositive."),
                Val::C(b"All three axes were nonpositive."),
            ]
            .into_iter();
            MSSG.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            M,
            A1,
            B1,
            C1,
            MSSG,
            BAD,
        }
    }
}

/// Surface normal vector on an ellipsoid
///
/// Compute the outward-pointing, unit normal vector at a point on
/// the surface of an ellipsoid.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  A          I   Length of the ellipsoid semi-axis along the X-axis.
///  B          I   Length of the ellipsoid semi-axis along the Y-axis.
///  C          I   Length of the ellipsoid semi-axis along the Z-axis.
///  POINT      I   Body-fixed coordinates of a point on the ellipsoid.
///  NORMAL     O   Outward pointing unit normal to ellipsoid at POINT.
/// ```
///
/// # Detailed Input
///
/// ```text
///  A        is the length of the semi-axis of the ellipsoid that is
///           parallel to the X-axis of the body-fixed reference frame.
///
///  B        is the length of the semi-axis of the ellipsoid that is
///           parallel to the Y-axis of the body-fixed reference frame.
///
///  C        is the length of the semi-axis of the ellipsoid that is
///           parallel to the Z-axis of the body-fixed reference frame.
///
///  POINT    is a 3-vector giving the bodyfixed coordinates of a point
///           on the ellipsoid. In bodyfixed coordinates, the semi-axes
///           of the ellipsoid are aligned with the X, Y, and Z-axes of
///           the reference frame.
/// ```
///
/// # Detailed Output
///
/// ```text
///  NORMAL   is the unit vector pointing away from the ellipsoid and
///           normal to the ellipsoid at POINT.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If any of the axes are non-positive, the error
///      SPICE(BADAXISLENGTH) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine computes the outward pointing unit normal vector to
///  the ellipsoid having semi-axes of length A, B, and C from the
///  point POINT.
/// ```
///
/// # Examples
///
/// ```text
///  A typical use of SURFNM would be to find the angle of incidence
///  of the light from the sun at a point on the surface of an
///  ellipsoid.
///
///  Let Q be a 3-vector representing the rectangular body-fixed
///  coordinates of a point on the ellipsoid (we are assuming that
///  the axes of the ellipsoid are aligned with the axes of the
///  body fixed frame.) Let V be the vector from Q to the sun in
///  bodyfixed coordinates. Then the following code fragment could
///  be used to compute angle of incidence of sunlight at Q.
///
///     CALL SURFNM   ( A, B, C, Q, NRML )
///
///     INCIDN = VSEP ( V,          NRML )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  It is assumed that the input POINT is indeed on the ellipsoid.
///      No checking for this is done.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.4.0, 26-OCT-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section.
///
///         Updated the documentation to refer to "reference frame" instead
///         of "coordinate system" as per NAIF conventions.
///
/// -    SPICELIB Version 1.3.2, 23-FEB-2016 (NJB)
///
///         Corrected some typos in the header.
///
/// -    SPICELIB Version 1.3.1, 18-MAY-2010 (BVS)
///
///         Removed "C$" marker from text in the header.
///
/// -    SPICELIB Version 1.3.0, 02-SEP-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in VHAT call.
///
/// -    SPICELIB Version 1.2.0, 07-AUG-1996 (WLT)
///
///         Added a SAVE statement so that the error message will
///         not be lost between separate invocations of the routine.
///
/// -    SPICELIB Version 1.1.0, 21-JUL-1995 (WLT)
///
///         A typo in the $Examples section was corrected
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT)
/// ```
pub fn surfnm(
    ctx: &mut SpiceContext,
    a: f64,
    b: f64,
    c: f64,
    point: &[f64; 3],
    normal: &mut [f64; 3],
) -> crate::Result<()> {
    SURFNM(a, b, c, point, normal, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SURFNM ( Surface normal vector on an ellipsoid )
pub fn SURFNM(
    A: f64,
    B: f64,
    C: f64,
    POINT: &[f64],
    NORMAL: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let POINT = DummyArray::new(POINT, 1..=3);
    let mut NORMAL = DummyArrayMut::new(NORMAL, 1..=3);

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Initial values
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"SURFNM", ctx)?;
    }

    //
    // Check the axes to make sure that none of them is less than or
    // equal to zero. If one is, signal an error and return.
    //
    save.BAD = 0;

    if (A <= 0 as f64) {
        save.BAD = (save.BAD + 1);
    }

    if (B <= 0 as f64) {
        save.BAD = (save.BAD + 2);
    }

    if (C <= 0 as f64) {
        save.BAD = (save.BAD + 4);
    }

    if (save.BAD > 0) {
        SETMSG(&fstr::concat(save.MSSG.get(save.BAD), b" ? "), ctx);
        ERRCH(
            b" ? ",
            b"The A,B, and C axes were #, #, and # respectively.",
            ctx,
        );

        ERRDP(b"#", A, ctx);
        ERRDP(b"#", B, ctx);
        ERRDP(b"#", C, ctx);

        SIGERR(b"SPICE(BADAXISLENGTH)", ctx)?;
        CHKOUT(b"SURFNM", ctx)?;
        return Ok(());
    }

    //
    // Mathematically we want to compute (Px/a**2, Py/b**2, Pz/c**2)
    // and then convert this to a unit vector. However, computationally
    // this can blow up in our faces.  But note that only the ratios
    // a/b, b/c and a/c are important in computing the unit normal.
    // We can use the trick below to avoid the unpleasantness of
    // multiplication and division overflows.
    //
    save.M = intrinsics::DMIN1(&[A, B, C]);

    //
    // M can be divided by A,B or C without fear of an overflow
    // occurring.
    //
    save.A1 = (save.M / A);
    save.B1 = (save.M / B);
    save.C1 = (save.M / C);

    //
    // All of the terms A1,B1,C1 are less than 1. Thus no overflows
    // can occur.
    //
    NORMAL[1] = (POINT[1] * (save.A1 * save.A1));
    NORMAL[2] = (POINT[2] * (save.B1 * save.B1));
    NORMAL[3] = (POINT[3] * (save.C1 * save.C1));

    VHATIP(NORMAL.as_slice_mut());

    CHKOUT(b"SURFNM", ctx)?;
    Ok(())
}
