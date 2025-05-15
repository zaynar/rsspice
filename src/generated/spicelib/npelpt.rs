//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const UBEL: i32 = 9;

/// Nearest point on ellipse to point
///
/// Find the nearest point on an ellipse to a specified point, both
/// in three-dimensional space, and find the distance between the
/// ellipse and the point.
///
/// # Required Reading
///
/// * [ELLIPSES](crate::required_reading::ellipses)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  POINT      I   Point whose distance to an ellipse is to be found.
///  ELLIPS     I   A SPICE ellipse.
///  PNEAR      O   Nearest point on ellipse to input point.
///  DIST       O   Distance of input point to ellipse.
/// ```
///
/// # Detailed Input
///
/// ```text
///  POINT    is a point in 3-dimensional space.
///
///  ELLIPS   is a SPICE ellipse that represents an ellipse
///           in three-dimensional space.
/// ```
///
/// # Detailed Output
///
/// ```text
///  PNEAR    is the nearest point on ELLIPS to POINT.
///
///  DIST     is the distance between POINT and PNEAR. This is
///           the distance between POINT and the ellipse.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input ellipse ELLIPS has one or both semi-axis lengths
///      equal to zero, the error SPICE(DEGENERATECASE) is signaled.
///
///  2)  If the geometric ellipse represented by ELLIPS does not
///      have a unique point nearest to the input point, any point
///      at which the minimum distance is attained may be returned
///      in PNEAR.
///
///  3)  If a ratio of non-zero ellipse radii violates the constraints
///      imposed by NEARPT, an error is signaled by a routine in the
///      call tree of this routine.
///
///  4)  The routine does not check for overflow when scaling or
///      translating the input point.
/// ```
///
/// # Particulars
///
/// ```text
///  Given an ellipse and a point in 3-dimensional space, if the
///  orthogonal projection of the point onto the plane of the ellipse
///  is on or outside of the ellipse, then there is a unique point on
///  the ellipse closest to the original point. This routine finds
///  that nearest point on the ellipse. If the projection falls inside
///  the ellipse, there may be multiple points on the ellipse that are
///  at the minimum distance from the original point. In this case,
///  one such closest point will be returned.
///
///  This routine returns a distance, rather than an altitude, in
///  contrast to the SPICELIB routine NEARPT. Because our ellipse is
///  situated in 3-space and not 2-space, the input point is not
///  "inside" or "outside" the ellipse, so the notion of altitude does
///  not apply to the problem solved by this routine. In the case of
///  NEARPT, the input point is on, inside, or outside the ellipsoid,
///  so it makes sense to speak of its altitude.
/// ```
///
/// # Examples
///
/// ```text
///  1)  For planetary rings that can be modeled as flat disks with
///      elliptical outer boundaries, the distance of a point in
///      space from a ring's outer boundary can be computed using this
///      routine. Suppose CENTER, SMAJOR, and SMINOR are the center,
///      semi-major axis, and semi-minor axis of the ring's boundary.
///      Suppose also that SCPOS is the position of a spacecraft.
///      SCPOS, CENTER, SMAJOR, and SMINOR must all be expressed in
///      the same coordinate system. We can find the distance from
///      the spacecraft to the ring using the code fragment
///
///         C
///         C     Make a SPICE ellipse representing the ring,
///         C     then use NPELPT to find the distance between
///         C     the spacecraft position and RING.
///         C
///               CALL CGV2EL ( CENTER, SMAJOR, SMINOR, RING )
///               CALL NPELPT ( SCPOS,  RING,   PNEAR,  DIST )
///
///
///  2)  The problem of finding the distance of a line from a tri-axial
///      ellipsoid can be reduced to the problem of finding the
///      distance between the same line and an ellipse; this problem in
///      turn can be reduced to the problem of finding the distance
///      between an ellipse and a point. The routine NPEDLN carries
///      out this process and uses NPELPT to find the ellipse-to-point
///      distance.
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
/// -    SPICELIB Version 1.3.0, 24-AUG-2021 (JDR) (NJB)
///
///         Added IMPLICT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
///         Added entries #3 and #4 to $Exceptions section.
///
/// -    SPICELIB Version 1.2.0, 02-SEP-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in VADD, VSCL, MTXV and MXV calls.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 02-NOV-1990 (NJB)
/// ```
pub fn npelpt(
    ctx: &mut SpiceContext,
    point: &[f64; 3],
    ellips: &[f64; 9],
    pnear: &mut [f64; 3],
    dist: &mut f64,
) -> crate::Result<()> {
    NPELPT(point, ellips, pnear, dist, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure NPELPT  ( Nearest point on ellipse to point )
pub fn NPELPT(
    POINT: &[f64],
    ELLIPS: &[f64],
    PNEAR: &mut [f64],
    DIST: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let POINT = DummyArray::new(POINT, 1..=3);
    let ELLIPS = DummyArray::new(ELLIPS, 1..=UBEL);
    let mut PNEAR = DummyArrayMut::new(PNEAR, 1..=3);
    let mut CENTER = StackArray::<f64, 3>::new(1..=3);
    let mut MAJLEN: f64 = 0.0;
    let mut MINLEN: f64 = 0.0;
    let mut ROTATE = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut SCALE: f64 = 0.0;
    let mut SMAJOR = StackArray::<f64, 3>::new(1..=3);
    let mut SMINOR = StackArray::<f64, 3>::new(1..=3);
    let mut TEMPV = StackArray::<f64, 3>::new(1..=3);
    let mut TMPPNT = StackArray::<f64, 3>::new(1..=3);
    let mut PRJPNT = StackArray::<f64, 3>::new(1..=3);

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"NPELPT", ctx)?;
    }

    //
    // Here's an overview of our solution:
    //
    //    Let ELPL be the plane containing the ELLIPS, and let PRJ be
    //    the orthogonal projection of the POINT onto ELPL.  Let X be
    //    any point in the plane ELPL.  According to the Pythagorean
    //    Theorem,
    //
    //                       2                       2                  2
    //       || POINT - X ||    =   || POINT - PRJ ||   +  || PRJ - X ||.
    //
    //    Then if we can find a point X on ELLIPS that minimizes the
    //    rightmost term, that point X is the closest point on the
    //    ellipse to POINT.
    //
    //    So, we find the projection PRJ, and then solve the problem of
    //    finding the closest point on ELLIPS to PRJ.  To solve this
    //    problem, we find a triaxial ellipsoid whose intersection with
    //    the plane ELPL is precisely ELLIPS, and two of whose axes lie
    //    in the plane ELPL.  The closest point on ELLIPS to PRJ is also
    //    the closest point on the ellipsoid to ELLIPS.  But we have the
    //    SPICELIB routine NEARPT on hand to find the closest point on an
    //    ellipsoid to a specified point, so we've reduced our problem to
    //    a solved problem.
    //
    //    There is a subtle point to worry about here:  if PRJ is outside
    //    of ELLIPS (PRJ is in the same plane as ELLIPS, so `outside'
    //    does make sense here), then the closest point on ELLIPS to PRJ
    //    coincides with the closest point on the ellipsoid to PRJ,
    //    regardless of how we choose the z-semi-axis length of the
    //    ellipsoid.  But the correspondence may be lost if PRJ is inside
    //    the ellipse, if we don't choose the z-semi-axis length
    //    correctly.
    //
    //    Though it takes some thought to verify this (and we won't prove
    //    it here), making the z-semi-axis of the ellipsoid longer than
    //    the other two semi-axes is sufficient to maintain the
    //    coincidence of the closest point on the ellipsoid to PRJPNT and
    //    the closest point on the ellipse to PRJPNT.
    //

    //
    // Find the ellipse's center and semi-axes.
    //
    EL2CGV(
        ELLIPS.as_slice(),
        CENTER.as_slice_mut(),
        SMAJOR.as_slice_mut(),
        SMINOR.as_slice_mut(),
    );

    //
    // Find the lengths of the semi-axes, and scale the vectors to try
    // to prevent arithmetic unpleasantness.  Degenerate ellipses are
    // turned away at the door.
    //
    MINLEN = VNORM(SMINOR.as_slice());
    MAJLEN = VNORM(SMAJOR.as_slice());

    if (intrinsics::DMIN1(&[MAJLEN, MINLEN]) == 0.0) {
        SETMSG(b"Semi-axis lengths: # #. ", ctx);
        ERRDP(b"#", MAJLEN, ctx);
        ERRDP(b"#", MINLEN, ctx);
        SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
        CHKOUT(b"NPELPT", ctx)?;
        return Ok(());
    }

    SCALE = (1.0 / MAJLEN);

    VSCLIP(SCALE, SMAJOR.as_slice_mut());
    VSCLIP(SCALE, SMINOR.as_slice_mut());

    //
    // Translate ellipse and point so that the ellipse is centered at
    // the origin.  Scale the point's coordinates to maintain the
    // correct relative position to the scaled ellipse.
    //
    VSUB(POINT.as_slice(), CENTER.as_slice(), TMPPNT.as_slice_mut());
    VSCLIP(SCALE, TMPPNT.as_slice_mut());

    //
    // We want to reduce the problem to a two-dimensional one.  We'll
    // work in a coordinate system whose x- and y- axes are aligned with
    // the semi-major and semi-minor axes of the input ellipse.  The
    // z-axis is picked to give us a right-handed system.  We find the
    // matrix that transforms coordinates to our new system using TWOVEC.
    //
    TWOVEC(
        SMAJOR.as_slice(),
        1,
        SMINOR.as_slice(),
        2,
        ROTATE.as_slice_mut(),
        ctx,
    )?;

    //
    // Apply the coordinate transformation to our scaled input point.
    //
    MXV(ROTATE.as_slice(), TMPPNT.as_slice(), TEMPV.as_slice_mut());
    VEQU(TEMPV.as_slice(), TMPPNT.as_slice_mut());

    //
    // We must find the distance between the orthogonal projection of
    // TMPPNT onto the x-y plane and the ellipse.  The projection is
    // just
    //
    //    ( TMPPNT(1), TMPPNT(2), 0 );
    //
    // we'll call this projection PRJPNT.
    //
    //
    VPACK(TMPPNT[1], TMPPNT[2], 0.0, PRJPNT.as_slice_mut());

    //
    // Now we're ready to find the distance between and a triaxial
    // ellipsoid whose intersection with the x-y plane is the ellipse
    // and whose third semi-axis lies on the z-axis.
    //
    // Because we've scaled the ellipse's axes so as to give the longer
    // axis length 1, a length of 2.D0 suffices for the ellipsoid's
    // z-semi-axis.
    //

    //
    // Find the nearest point to PRJPNT on the ellipsoid, PNEAR.
    //
    NEARPT(
        PRJPNT.as_slice(),
        1.0,
        (MINLEN / MAJLEN),
        2.0,
        PNEAR.as_slice_mut(),
        DIST,
        ctx,
    )?;

    //
    // Scale the near point coordinates back to the original scale.
    //
    VSCLIP(MAJLEN, PNEAR.as_slice_mut());

    //
    // Apply the required inverse rotation and translation to PNEAR.
    // Compute the point-to-ellipse distance.
    //
    MTXV(ROTATE.as_slice(), PNEAR.as_slice(), TEMPV.as_slice_mut());
    VADD(TEMPV.as_slice(), CENTER.as_slice(), PNEAR.as_slice_mut());

    *DIST = VDIST(PNEAR.as_slice(), POINT.as_slice());

    CHKOUT(b"NPELPT", ctx)?;
    Ok(())
}
