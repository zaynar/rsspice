//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const UBEL: i32 = 9;
pub const UBPL: i32 = 4;
const SEPLIM: f64 = 0.000000001;

/// Intersection of ellipse and plane
///
/// Find the intersection of an ellipse and a plane.
///
/// # Required Reading
///
/// * [ELLIPSES](crate::required_reading::ellipses)
/// * [PLANES](crate::required_reading::planes)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ELLIPS     I   A SPICE ellipse.
///  PLANE      I   A SPICE plane.
///  NXPTS      O   Number of intersection points of plane and ellipse.
///  XPT1,
///  XPT2       O   Intersection points.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ELLIPS   is a SPICE ellipse. The ellipse is allowed to
///           be degenerate: one or both semi-axes may have
///           zero length.
///
///  PLANE    is a SPICE plane.
/// ```
///
/// # Detailed Output
///
/// ```text
///  NXPTS    is the number of points of intersection of the
///           geometric plane and ellipse represented by PLANE
///           and ELLIPS. NXPTS may take the values 0, 1, 2 or
///           -1. The value -1 indicates that the ellipse
///           consists of more than one point and lies in the
///           plane, so the number of intersection points is
///           infinite.
///
///           When the ellipse consists of a single point and
///           lies in the plane, NXPTS is set to 1.
///
///  XPT1,
///  XPT2     are the points of intersection of the input plane
///           and ellipse. If there is only one intersection
///           point, both XPT1 and XPT2 contain that point. If
///           the number of intersection points is zero or
///           infinite, the contents of XPT1 and XPT2 are
///           undefined.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input plane is invalid, the error SPICE(INVALIDPLANE)
///      is signaled. The input plane must be a SPICE plane: the normal
///      vector must be non-zero and the constant must be non-negative.
///
///  2)  If the input ellipse has non-orthogonal axes, the error
///      SPICE(INVALIDELLIPSE) is signaled.
///
///  3)  The input ellipse is allowed to be a line segment or a point;
///      these cases are not considered to be errors. If the ellipse
///      consists of a single point and lies in the plane, the number
///      of intersection points is set to 1 (rather than -1) and
///      the output arguments XPT1 and XPT2 are assigned the value
///      of the ellipse's center.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine computes the intersection set of a non-degenerate
///  plane with a possibly degenerate ellipse. The ellipse is allowed
///  to consist of a line segment or a point.
///
///  A plane may intersect an ellipse in 0, 1, 2, or infinitely many
///  points. For there to be an infinite set of intersection points,
///  the ellipse must lie in the plane and consist of more than one
///  point.
/// ```
///
/// # Examples
///
/// ```text
///  1)  If we want to find the angle of some ray above the limb of an
///      ellipsoid, where the angle is measured in a plane containing
///      the ray and a `down' vector, we can follow the procedure
///      given below. We assume the ray does not intersect the
///      ellipsoid. The result we seek is called ANGLE, imaginatively
///      enough.
///
///      We assume that all vectors are given in body-fixed
///      coordinates.
///
///         C
///         C     Find the limb of the ellipsoid as seen from the
///         C     point OBSERV. Here A, B, and C are the lengths of
///         C     the semi-axes of the ellipsoid.
///         C
///               CALL EDLIMB ( A, B, C, OBSERV, LIMB )
///
///         C
///         C     The ray direction vector is RAYDIR, so the ray is the
///         C     set of points
///         C
///         C        OBSERV  +  t * RAYDIR
///         C
///         C     where t is any non-negative real number.
///         C
///         C     The `down' vector is just -OBSERV. The vectors
///         C     OBSERV and RAYDIR are spanning vectors for the plane
///         C     we're interested in. We can use PSV2PL to represent
///         C     this plane by a SPICE plane.
///         C
///               CALL PSV2PL ( OBSERV, OBSERV, RAYDIR, PLANE )
///
///         C
///         C     Find the intersection of the plane defined by OBSERV
///         C     and RAYDIR with the limb.
///         C
///               CALL INELPL ( LIMB, PLANE, NXPTS, XPT1, XPT2 )
///
///         C
///         C     We always expect two intersection points, if DOWN
///         C     is valid.
///         C
///               IF ( NXPTS .LT. 2 ) THEN
///
///                  [ do something about the error ]
///
///               ENDIF
///
///         C
///         C     Form the vectors from OBSERV to the intersection
///         C     points. Find the angular separation between the
///         C     boresight ray and each vector from OBSERV to the
///         C     intersection points.
///         C
///               CALL VSUB   ( XPT1, OBSERV, VEC1 )
///               CALL VSUB   ( XPT2, OBSERV, VEC2 )
///
///               SEP1 = VSEP ( VEC1, RAYDIR )
///               SEP2 = VSEP ( VEC2, RAYDIR )
///
///         C
///         C     The angular separation we're after is the minimum of
///         C     the two separations we've computed.
///         C
///               ANGLE = MIN ( SEP1, SEP2 )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.1.0, 24-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 3.0.0, 07-OCT-2011 (NJB)
///
///         Relaxed ellipse semi-axes orthogonality test limit
///         SEPLIM from 1.D-12 TO 1.D-9 radians. The angular
///         separation of the axes of the input ellipse must not
///         differ from pi/2 radians by more than this limit.
///
/// -    SPICELIB Version 2.0.0, 14-JAN-2008 (NJB)
///
///         Bug fix: the routine's specification and behavior have been
///         updated so the routine now returns a meaningful result for the
///         case of an ellipse consisting of a single point.
///
///         Bug fix: in the degenerate case where the input ellipse is a
///         line segment of positive length, and this segment intersects
///         the plane, the number of intersection points is set to 1
///         rather than 2.
///
///         Invalid input planes and ellipses are now diagnosed.
///
/// -    SPICELIB Version 1.2.0, 25-AUG-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in VSUB call.
///
/// -    SPICELIB Version 1.1.0, 24-MAR-1992 (NJB) (WLT)
///
///         Output arguments XPT1, XPT2 are now correctly declared
///         with length 3. Comment section for permuted index source
///         lines was added following the header.
///
/// -    SPICELIB Version 1.0.0, 02-NOV-1990 (NJB)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 2.0.0, 14-JAN-2008 (NJB)
///
///         Bug fix: the routine's specification and behavior have been
///         updated so the routine now returns a meaningful result for the
///         case of an ellipse consisting of a single point. In this case,
///         if an intersection is found, the number of intersection points
///         is set to 1 and both intersection arguments are set equal to
///         the ellipse's center.
///
///         Bug fix: in the degenerate case where the input ellipse is a
///         line segment of positive length, and this segment intersects
///         the plane, the number of intersection points is set to 1
///         rather than 2.
///
///         Invalid input planes and ellipses are now diagnosed.
///         Error handling code has been added to trap errors that had
///         been erroneously passed off to lower level routines for
///         diagnosis.
/// ```
pub fn inelpl(
    ctx: &mut SpiceContext,
    ellips: &[f64; 9],
    plane: &[f64; 4],
    nxpts: &mut i32,
    xpt1: &mut [f64; 3],
    xpt2: &mut [f64; 3],
) -> crate::Result<()> {
    INELPL(ellips, plane, nxpts, xpt1, xpt2, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure INELPL ( Intersection of ellipse and plane )
pub fn INELPL(
    ELLIPS: &[f64],
    PLANE: &[f64],
    NXPTS: &mut i32,
    XPT1: &mut [f64],
    XPT2: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let ELLIPS = DummyArray::new(ELLIPS, 1..=UBEL);
    let PLANE = DummyArray::new(PLANE, 1..=UBPL);
    let mut XPT1 = DummyArrayMut::new(XPT1, 1..=3);
    let mut XPT2 = DummyArrayMut::new(XPT2, 1..=3);
    let mut ALPHA: f64 = 0.0;
    let mut ANGLE1: f64 = 0.0;
    let mut ANGLE2: f64 = 0.0;
    let mut BETA: f64 = 0.0;
    let mut CENTER = StackArray::<f64, 3>::new(1..=3);
    let mut CONST: f64 = 0.0;
    let mut INPCON: f64 = 0.0;
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
    let mut POINT = StackArray::<f64, 3>::new(1..=3);
    let mut SEP: f64 = 0.0;
    let mut SMAJOR = StackArray::<f64, 3>::new(1..=3);
    let mut SMINOR = StackArray::<f64, 3>::new(1..=3);
    let mut TMPVEC = StackArray::<f64, 3>::new(1..=3);
    let mut TRANS = StackArray::<f64, 4>::new(1..=UBPL);
    let mut V = StackArray::<f64, 2>::new(1..=2);

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"INELPL", ctx)?;

    //
    // Check the input plane.
    //
    PL2NVC(PLANE.as_slice(), NORMAL.as_slice_mut(), &mut INPCON);

    if VZERO(NORMAL.as_slice()) {
        SETMSG(b"Input SPICE plane has zero normal vector.", ctx);
        SIGERR(b"SPICE(INVALIDPLANE)", ctx)?;
        CHKOUT(b"INELPL", ctx)?;
        return Ok(());
    } else if (INPCON < 0.0) {
        SETMSG(b"Input SPICE plane has non-positive constant #. Properly constructed SPICE planes always have non-negative constants.", ctx);
        ERRDP(b"#", INPCON, ctx);
        SIGERR(b"SPICE(INVALIDPLANE)", ctx)?;
        CHKOUT(b"INELPL", ctx)?;
        return Ok(());
    }

    //
    // Get the components of the input ellipse; check for
    // invalid semi-axes. The semi-axes may have zero length
    // but they must always be orthogonal. We require this
    // check only if both semi-axes have non-zero length.
    //
    EL2CGV(
        ELLIPS.as_slice(),
        CENTER.as_slice_mut(),
        SMAJOR.as_slice_mut(),
        SMINOR.as_slice_mut(),
    );

    if !VZERO(SMINOR.as_slice()) {
        SEP = VSEP(SMAJOR.as_slice(), SMINOR.as_slice(), ctx);

        if (f64::abs((SEP - HALFPI(ctx))) > SEPLIM) {
            SETMSG(b"Input SPICE ellipse has non-orthogonal semi-axes: (#,#,#) and (#,#,#). Angular separation of these vectors is # radians. Properly constructed SPICE ellipses always have orthogonal semi-axes.", ctx);
            ERRDP(b"#", SMAJOR[1], ctx);
            ERRDP(b"#", SMAJOR[2], ctx);
            ERRDP(b"#", SMAJOR[3], ctx);
            ERRDP(b"#", SMINOR[1], ctx);
            ERRDP(b"#", SMINOR[2], ctx);
            ERRDP(b"#", SMINOR[3], ctx);
            ERRDP(b"#", SEP, ctx);
            SIGERR(b"SPICE(INVALIDELLIPSE)", ctx)?;
            CHKOUT(b"INELPL", ctx)?;
            return Ok(());
        }
    }

    //
    // If the input ellipse is a single point, decide now
    // whether the ellipse lies in the plane.
    //
    if VZERO(SMAJOR.as_slice()) {
        //
        // The ellipse is a single point. If the ellipse's center
        // lies in the plane, the whole ellipse is the one
        // intersection point. Check the inner product of the
        // center and the plane's normal vector.
        //
        if (VDOT(CENTER.as_slice(), NORMAL.as_slice()) == INPCON) {
            //
            // The center does in fact lie in the plane.
            //
            *NXPTS = 1;

            VEQU(CENTER.as_slice(), XPT1.as_slice_mut());
            VEQU(CENTER.as_slice(), XPT2.as_slice_mut());
        } else {
            //
            // There's no intersection: the intersection arguments
            // are left undefined in this case.
            //
            *NXPTS = 0;
        }

        //
        // Return now; this simplifies the logic to follow.
        //
        CHKOUT(b"INELPL", ctx)?;
        return Ok(());
    }

    //
    // At this point the ellipse may still be degenerate: it can be a
    // line segment. We'll need to compute the intersection point or
    // points if we have a positive, finite intersection set.
    //
    // The first thing we want to do is translate the plane and the
    // ellipse so as to center the ellipse at the origin.  To translate
    // the plane, just get a point and normal vector, and translate
    // the point.  Find the plane constant of the translated plane.
    //
    PL2NVP(
        PLANE.as_slice(),
        NORMAL.as_slice_mut(),
        TMPVEC.as_slice_mut(),
    );
    VSUB(TMPVEC.as_slice(), CENTER.as_slice(), POINT.as_slice_mut());
    NVP2PL(
        NORMAL.as_slice(),
        POINT.as_slice(),
        TRANS.as_slice_mut(),
        ctx,
    )?;

    PL2NVC(TRANS.as_slice(), NORMAL.as_slice_mut(), &mut CONST);

    //
    // Ok, we can get to work.  The locus of the ellipse is
    //
    //    cos(theta) SMAJOR  +  sin(theta) SMINOR,
    //
    // and any point X of the ellipse that intersects the input plane
    // satisfies
    //
    //    < X, NORMAL >  =  CONST.
    //
    // Substituting our expression for points on the ellipse into the
    // second equation, we arrive at
    //
    //       cos(theta) < SMAJOR, NORMAL >
    //    +  sin(theta) < SMINOR, NORMAL >   =  CONST.        (1)
    //
    // This equation merits a little analysis. First, if NORMAL
    // is orthogonal to SMAJOR and SMINOR, the plane and ellipse must
    // be parallel. Also, the left side of the equation is zero in
    // this case. If CONST is non-zero, there are no solutions:
    // the ellipse and plane are parallel but do not intersect. If
    // CONST is zero, the ellipse lies in the plane: all values of
    // theta are solutions. Let's get this case out of the way
    // right now, shall we?
    //
    V[1] = VDOT(SMAJOR.as_slice(), NORMAL.as_slice());
    V[2] = VDOT(SMINOR.as_slice(), NORMAL.as_slice());

    //
    // Test whether the plane and ellipse are parallel:
    //
    if VZEROG(V.as_slice(), 2) {
        //
        // The ellipse lies in the plane if and only if CONST is zero.
        // In any case, we don't modify XPT1 or XPT2.
        //
        if (CONST == 0.0) {
            *NXPTS = -1;
        } else {
            *NXPTS = 0;
        }

        CHKOUT(b"INELPL", ctx)?;
        return Ok(());
    }

    //
    // Now if NORMAL is not orthogonal to both SMAJOR and SMINOR,
    // the vector
    //
    //    V = (  < SMAJOR, NORMAL >,  < SMINOR, NORMAL >  )
    //
    // is non-zero.  We can re-write (1) as
    //
    //    < U, V >  =  CONST,
    //
    // where
    //
    //    U = ( cos(theta), sin(theta) ).
    //
    // If alpha is the angle between U and V, we have
    //
    //    < U, V >  =  || U ||  *  || V ||  *  cos(alpha),
    //
    // so
    //
    //    || V ||  *  cos(alpha)  =  CONST.                   (2)
    //
    // CONST is positive, since PL2NVC returns the distance
    // of between its input plane and the origin as the output
    // plane constant.
    //
    // Equation (2) has solutions if and only if
    //
    //    || V ||  >    CONST.                                (3)
    //             -
    //
    // Let's return right now if there are no solutions.
    //
    if (VNORMG(V.as_slice(), 2) < CONST) {
        *NXPTS = 0;
        CHKOUT(b"INELPL", ctx)?;
        return Ok(());
    }

    //
    // Since (3) above is satisfied, the plane and ellipse intersect.
    // We can find alpha using the formula
    //
    //    alpha  =  +  arccos (  CONST  /  || V ||  )
    //
    // Since alpha is the angular separation between U and V, we
    // can find U once we have the angular position of V; let's
    // call that beta.  The angular position of U (which we called
    // theta earlier) will be
    //
    //    theta   =   beta  +  alpha.
    //                      -
    //
    // The values of theta are the angles we seek.
    //

    ALPHA = f64::acos((CONST / VNORMG(V.as_slice(), 2)));

    BETA = f64::atan2(V[2], V[1]);

    ANGLE1 = (BETA - ALPHA);
    ANGLE2 = (BETA + ALPHA);

    //
    // Determine the number of intersection points. We have a special
    // case if the semi-minor axis has length zero: in that case BETA is
    // zero or Pi, and although ANGLE1 and ANGLE2 may differ, the
    // cosines of these angles are identical. Since in this case
    // the solutions corresponding to ANGLE1 and ANGLE2 have the
    // form
    //
    //    CENTER + cos(ANGLE1)*SMAJOR
    //    CENTER + cos(ANGLE2)*SMAJOR
    //
    // the solutions are identical.
    //
    //
    if VZERO(SMINOR.as_slice()) {
        *NXPTS = 1;
    } else {
        if (ANGLE1 == ANGLE2) {
            //
            // This case occurs when ALPHA is zero.
            //
            *NXPTS = 1;
        } else {
            *NXPTS = 2;
        }
    }

    //
    // Compute the intersection points.
    //
    VLCOM3(
        1.0,
        CENTER.as_slice(),
        f64::cos(ANGLE1),
        SMAJOR.as_slice(),
        f64::sin(ANGLE1),
        SMINOR.as_slice(),
        XPT1.as_slice_mut(),
    );

    VLCOM3(
        1.0,
        CENTER.as_slice(),
        f64::cos(ANGLE2),
        SMAJOR.as_slice(),
        f64::sin(ANGLE2),
        SMINOR.as_slice(),
        XPT2.as_slice_mut(),
    );

    CHKOUT(b"INELPL", ctx)?;
    Ok(())
}
