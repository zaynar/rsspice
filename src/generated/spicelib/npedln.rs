//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const UBEL: i32 = 9;
pub const UBPL: i32 = 4;

/// Nearest point on ellipsoid to line
///
/// Find nearest point on a triaxial ellipsoid to a specified line,
/// and the distance from the ellipsoid to the line.
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
///  A          I   Length of ellipsoid's semi-axis in the x direction
///  B          I   Length of ellipsoid's semi-axis in the y direction
///  C          I   Length of ellipsoid's semi-axis in the z direction
///  LINEPT     I   Point on line
///  LINEDR     I   Direction vector of line
///  PNEAR      O   Nearest point on ellipsoid to line
///  DIST       O   Distance of ellipsoid from line
///  UBEL       P   Upper bound of array containing SPICE ellipse.
///  UBPL       P   Upper bound of array containing SPICE plane.
/// ```
///
/// # Detailed Input
///
/// ```text
///  A,
///  B,
///  C        are the lengths of the semi-axes of a triaxial
///           ellipsoid which is centered at the origin and
///           oriented so that its axes lie on the x-, y- and
///           z- coordinate axes.  A, B, and C are the lengths of
///           the semi-axes that point in the x, y, and z
///           directions respectively.
///
///  LINEPT
///  LINEDR   are, respectively, a point and a direction vector
///           that define a line. The line is the set of vectors
///
///              LINEPT   +   t * LINEDR
///
///           where t is any real number.
/// ```
///
/// # Detailed Output
///
/// ```text
///  PNEAR    is the point on the ellipsoid that is closest to
///           the line, if the line doesn't intersect the
///           ellipsoid.
///
///           If the line intersects the ellipsoid, PNEAR will
///           be a point of intersection. If LINEPT is outside
///           of the ellipsoid, PNEAR will be the closest point
///           of intersection. If LINEPT is inside the
///           ellipsoid, PNEAR will not necessarily be the
///           closest point of intersection.
///
///
///  DIST     is the distance of the line from the ellipsoid.
///           This is the minimum distance between any point on
///           the line and any point on the ellipsoid.
///
///           If the line intersects the ellipsoid, DIST is zero.
/// ```
///
/// # Parameters
///
/// ```text
///  UBEL     is the upper bound of the array used to contain
///           a SPICE ellipse. See the ELLIPSES Required
///           Reading for details.
///
///  UBPL     is the upper bound of the array used to contain
///           a SPICE plane. See the PLANES Required Reading
///           for details.
/// ```
///
/// # Exceptions
///
/// ```text
///  If this routine detects an error, the output arguments PNEAR and
///  DIST are not modified.
///
///  1)  If the length of any semi-axis of the ellipsoid is
///      non-positive, the error SPICE(INVALIDAXISLENGTH) is signaled.
///
///  2)  If the line's direction vector is the zero vector, the error
///      SPICE(ZEROVECTOR) is signaled.
///
///  3)  If the length of any semi-axis of the ellipsoid is zero after
///      the semi-axis lengths are scaled by the reciprocal of the
///      magnitude of the longest semi-axis and then squared, the error
///      SPICE(DEGENERATECASE) is signaled.
///
///  4)  If the input ellipsoid is extremely flat or needle-shaped
///      and has its shortest axis close to perpendicular to the input
///      line, numerical problems could cause this routine's algorithm
///      to fail, in which case, the error SPICE(DEGENERATECASE) is
///      signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  For any ellipsoid and line, if the line does not intersect the
///  ellipsoid, there is a unique point on the ellipsoid that is
///  closest to the line. Therefore, the distance DIST between
///  ellipsoid and line is well-defined. The unique line segment of
///  length DIST that connects the line and ellipsoid is normal to
///  both of these objects at its endpoints.
///
///  If the line intersects the ellipsoid, the distance between the
///  line and ellipsoid is zero.
/// ```
///
/// # Examples
///
/// ```text
///  1)   We can find the distance between an instrument optic axis ray
///       and the surface of a body modeled as a tri-axial ellipsoid
///       using this routine. If the instrument position and pointing
///       unit vector in body-fixed coordinates are
///
///          LINEPT = ( 1.0D6,  2.0D6,  3.0D6 )
///
///       and
///
///          LINEDR = ( -4.472091234D-1
///                     -8.944182469D-1,
///                     -4.472091234D-3  )
///
///       and the body semi-axes lengths are
///
///          A = 7.0D5
///          B = 7.0D5
///          C = 6.0D5,
///
///       then the call to NPEDLN
///
///          CALL NPEDLN ( A,      B,      C,
///         .              LINEPT, LINEDR,
///         .              PNEAR,  DIST        )
///
///       yields a value for PNEAR, the nearest point on the body to
///       the optic axis ray, of
///
///
///          (  -1.6333110792340931E+03,
///             -3.2666222157820771E+03,
///              5.9999183350006724E+05  )
///
///       and a value for DIST, the distance to the ray, of
///
///          2.3899679338299707E+06
///
///       (These results were obtained on a PC-Linux system under g77.)
///
///       In some cases, it may not be clear that the closest point
///       on the line containing an instrument boresight ray is on
///       the boresight ray itself; the point may lie on the ray
///       having the same vertex as the boresight ray and pointing in
///       the opposite direction. To rule out this possibility, we
///       can make the following test:
///
///          C
///          C     Find the difference vector between the closest point
///          C     on the ellipsoid to the line containing the
///          C     boresight ray and the boresight ray's vertex. Find
///          C     the angular separation between this difference
///          C     vector and the boresight ray. If the angular
///          C     separation does not exceed pi/2, we have the nominal
///          C     geometry. Otherwise, we have an error.
///          C
///                CALL  VSUB ( PNEAR,  LINEPT,  DIFF )
///                SEP = VSEP ( DIFF,   LINEDR        )
///
///                IF (  SEP .LE. HALFPI()  ) THEN
///
///                   [ perform normal processing ]
///
///                ELSE
///
///                   [ handle error case ]
///
///                END IF
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
/// -    SPICELIB Version 1.4.0, 24-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Corrected
///         argument name in $Exceptions section.
///
/// -    SPICELIB Version 1.3.0, 15-NOV-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in VSCL calls. Changed exponents to DOUBLE PRECISION type
///         in the test for underflow of squared, scaled axis lengths.
///
/// -    SPICELIB Version 1.2.1, 06-DEC-2002 (NJB)
///
///         Outputs shown in header example have been corrected to
///         be consistent with those produced by this routine.
///
/// -    SPICELIB Version 1.2.0, 25-NOV-1992 (NJB)
///
///         Bug fix: in the intercept case, PNEAR is now properly
///         re-scaled prior to output. Also, an error in the $Examples
///         section was corrected.
///
/// -    SPICELIB Version 1.1.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.1.0, 04-DEC-1990 (NJB)
///
///         Error message and description changed for non-positive
///         axis length error.
///
/// -    SPICELIB Version 1.0.0, 02-NOV-1990 (NJB)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.2.0, 25-NOV-1992 (NJB)
///
///         Bug fix: in the intercept case, PNEAR is now properly
///         re-scaled prior to output. Formerly, it was returned without
///         having been re-scaled.
///
///         Also, an error in the $Examples section was corrected: the
///         line
///
///            CALL  VSUB ( LINEPT,  PNEAR,  DIFF )
///
///         was replaced by
///
///            CALL  VSUB ( PNEAR,  LINEPT,  DIFF )
///
///         The in-line comments were re-arranged slightly, and the claim
///         that the inverse orthogonal projection of PRJNPT is guaranteed
///         to exist was removed. (The check for this exception was already
///         being done.)
///
///
/// -    SPICELIB Version 1.1.0, 04-DEC-1990 (NJB)
///
///         Error message and description changed for non-positive
///         axis length error. The former message and description did
///         not match, and the description was incorrect: it described
///         `zero-length', rather than `non-positive' axes as invalid.
/// ```
pub fn npedln(
    ctx: &mut SpiceContext,
    a: f64,
    b: f64,
    c: f64,
    linept: &[f64; 3],
    linedr: &[f64; 3],
    pnear: &mut [f64; 3],
    dist: &mut f64,
) -> crate::Result<()> {
    NPEDLN(a, b, c, linept, linedr, pnear, dist, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure NPEDLN ( Nearest point on ellipsoid to line )
pub fn NPEDLN(
    A: f64,
    B: f64,
    C: f64,
    LINEPT: &[f64],
    LINEDR: &[f64],
    PNEAR: &mut [f64],
    DIST: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let LINEPT = DummyArray::new(LINEPT, 1..=3);
    let LINEDR = DummyArray::new(LINEDR, 1..=3);
    let mut PNEAR = DummyArrayMut::new(PNEAR, 1..=3);
    let mut CANDPL = StackArray::<f64, 4>::new(1..=UBPL);
    let mut CAND = StackArray::<f64, 9>::new(1..=UBEL);
    let mut OPPDIR = StackArray::<f64, 3>::new(1..=3);
    let mut PRJPL = StackArray::<f64, 4>::new(1..=UBPL);
    let mut MAG: f64 = 0.0;
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
    let mut PRJEL = StackArray::<f64, 9>::new(1..=UBEL);
    let mut PRJPT = StackArray::<f64, 3>::new(1..=3);
    let mut PRJNPT = StackArray::<f64, 3>::new(1..=3);
    let mut PT = StackArray2D::<f64, 6>::new(1..=3, 1..=2);
    let mut SCALE: f64 = 0.0;
    let mut SCLA: f64 = 0.0;
    let mut SCLB: f64 = 0.0;
    let mut SCLC: f64 = 0.0;
    let mut SCLPT = StackArray::<f64, 3>::new(1..=3);
    let mut UDIR = StackArray::<f64, 3>::new(1..=3);
    let mut FOUND = StackArray::<bool, 2>::new(1..=2);
    let mut IFOUND: bool = false;
    let mut XFOUND: bool = false;

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
        CHKIN(b"NPEDLN", ctx)?;
    }

    //
    // The algorithm used in this routine has two parts.  The first
    // part handles the case where the input line and ellipsoid
    // intersect.  Our procedure is simple in that case; we just
    // call SURFPT twice, passing it first one ray determined by the
    // input line, then a ray pointing in the opposite direction.
    // The second part of the algorithm handles the case where SURFPT
    // doesn't find an intersection.
    //
    // Finding the nearest point on the ellipsoid to the line, when the
    // two do not intersect, is a matter of following four steps:
    //
    // 1)  Find the points on the ellipsoid where the surface normal
    //     is normal to the line's direction.  This set of points is
    //     an ellipse centered at the origin.  The point we seek MUST
    //     lie on this `candidate' ellipse.
    //
    // 2)  Project the candidate ellipse onto a plane that is normal
    //     to the line's direction.  This projection preserves
    //     distance from the line; the nearest point to the line on
    //     this new ellipse is the projection of the nearest point to
    //     the line on the candidate ellipse, and these two points are
    //     exactly the same distance from the line.  If computed using
    //     infinite-precision arithmetic, this projection would be
    //     guaranteed to be non-degenerate as long as the input
    //     ellipsoid were non-degenerate.  This can be verified by
    //     taking the inner product of the scaled normal to the candidate
    //     ellipse plane and the line's unitized direction vector
    //     (these vectors are called NORMAL and UDIR in the code below);
    //     the inner product is strictly greater than 1 if the ellipsoid
    //     is non-degenerate.
    //
    // 3)  The nearest point on the line to the projected ellipse will
    //     be contained in the plane onto which the projection is done;
    //     we find this point and then find the nearest point to it on
    //     the projected ellipse.  The distance between these two points
    //     is the distance between the line and the ellipsoid.
    //
    // 4)  Finally, we find the point on the candidate ellipse that was
    //     projected to the nearest point to the line on the projected
    //     ellipse that was found in step 3.  This is the nearest point
    //     on the ellipsoid to the line.
    //
    //
    //
    //
    //                  Glossary of Geometric Variables
    //
    //
    //        A,
    //        B,
    //        C           Input ellipsoid's semi-axis lengths.
    //
    //        POINT       Point of intersection of line and ellipsoid
    //                    if the intersection is non-empty.
    //
    //        CANDPL      Plane containing candidate ellipse.
    //
    //        NORMAL      Normal vector to the candidate plane CANDPL.
    //
    //        CAND        Candidate ellipse.
    //
    //        LINEPT,
    //        LINEDR,     Point and direction vector on input line.
    //
    //        UDIR        Unitized line direction vector.
    //
    //        PRJPL       Projection plane; the candidate ellipse is
    //                    projected onto this plane to yield PRJEL.
    //
    //        PRJEL       Projection of the candidate ellipse CAND onto
    //                    the projection plane PRJEL.
    //
    //        PRJPT       Projection of line point.
    //
    //        PRJNPT      Nearest point on projected ellipse to
    //                    projection of line point.
    //
    //        PNEAR       Nearest point on ellipsoid to line.
    //
    //

    //
    // We need a valid normal vector.
    //
    UNORM(LINEDR.as_slice(), UDIR.as_slice_mut(), &mut MAG);

    if (MAG == 0 as f64) {
        SETMSG(b"Line direction vector is the zero vector. ", ctx);
        SIGERR(b"SPICE(ZEROVECTOR)", ctx)?;
        CHKOUT(b"NPEDLN", ctx)?;
        return Ok(());

    //
    // The ellipsoid's semi-axes must have positive length.
    //
    } else if (((A <= 0.0) || (B <= 0.0)) || (C <= 0.0)) {
        SETMSG(b"Semi-axes: A = #,  B = #,  C = #.", ctx);
        ERRDP(b"#", A, ctx);
        ERRDP(b"#", B, ctx);
        ERRDP(b"#", C, ctx);
        SIGERR(b"SPICE(INVALIDAXISLENGTH)", ctx)?;
        CHKOUT(b"NPEDLN", ctx)?;
        return Ok(());
    }

    //
    // Scale the semi-axes lengths for better numerical behavior.
    // If squaring any one of the scaled lengths causes it to
    // underflow to zero, we have an error.  Otherwise, scale the
    // point on the input line too.
    //
    SCALE = intrinsics::DMAX1(&[f64::abs(A), f64::abs(B), f64::abs(C)]);

    SCLA = (A / SCALE);
    SCLB = (B / SCALE);
    SCLC = (C / SCALE);

    if (((f64::powf(SCLA, 2.0) == 0.0) || (f64::powf(SCLB, 2.0) == 0.0))
        || (f64::powf(SCLC, 2.0) == 0.0))
    {
        SETMSG(b"Semi-axis too small:  A = #, B = #, C = #. ", ctx);
        ERRDP(b"#", A, ctx);
        ERRDP(b"#", B, ctx);
        ERRDP(b"#", C, ctx);
        SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
        CHKOUT(b"NPEDLN", ctx)?;
        return Ok(());
    }

    //
    // Scale LINEPT.  Because SCALE might be a very small number,
    // we avoid computing 1/SCALE; that's why we don't call VSCL here.
    //
    SCLPT[1] = (LINEPT[1] / SCALE);
    SCLPT[2] = (LINEPT[2] / SCALE);
    SCLPT[3] = (LINEPT[3] / SCALE);

    //
    // Hand off the intersection case to SURFPT.  SURFPT determines
    // whether rays intersect a body, so we treat the line as a pair
    // of rays.
    //
    VMINUS(UDIR.as_slice(), OPPDIR.as_slice_mut());

    SURFPT(
        SCLPT.as_slice(),
        UDIR.as_slice(),
        SCLA,
        SCLB,
        SCLC,
        PT.subarray_mut([1, 1]),
        &mut FOUND[1],
        ctx,
    )?;
    SURFPT(
        SCLPT.as_slice(),
        OPPDIR.as_slice(),
        SCLA,
        SCLB,
        SCLC,
        PT.subarray_mut([1, 2]),
        &mut FOUND[2],
        ctx,
    )?;

    for I in 1..=2 {
        if FOUND[I] {
            *DIST = 0.0;

            VSCL(SCALE, PT.subarray([1, I]), PNEAR.as_slice_mut());
            CHKOUT(b"NPEDLN", ctx)?;
            return Ok(());
        }
    }

    //
    // Getting here means the line doesn't intersect the ellipsoid.
    //
    // Find the candidate ellipse CAND.  NORMAL is a normal vector to
    // the plane containing the candidate ellipse.   Mathematically the
    // ellipse must exist, since it's the intersection of an ellipsoid
    // centered at the origin and a plane containing the origin.  Only
    // numerical problems can prevent the intersection from being found.
    //
    //
    NORMAL[1] = (UDIR[1] / f64::powi(SCLA, 2));
    NORMAL[2] = (UDIR[2] / f64::powi(SCLB, 2));
    NORMAL[3] = (UDIR[3] / f64::powi(SCLC, 2));

    NVC2PL(NORMAL.as_slice(), 0.0, CANDPL.as_slice_mut(), ctx)?;

    INEDPL(
        SCLA,
        SCLB,
        SCLC,
        CANDPL.as_slice(),
        CAND.as_slice_mut(),
        &mut XFOUND,
        ctx,
    )?;

    if !XFOUND {
        SETMSG(b"Candidate ellipse could not be found.", ctx);
        SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
        CHKOUT(b"NPEDLN", ctx)?;
        return Ok(());
    }

    //
    // Project the candidate ellipse onto a plane orthogonal to the
    // line.  We'll call the plane PRJPL and the projected ellipse PRJEL.
    //
    NVC2PL(UDIR.as_slice(), 0.0, PRJPL.as_slice_mut(), ctx)?;
    PJELPL(CAND.as_slice(), PRJPL.as_slice(), PRJEL.as_slice_mut(), ctx)?;

    //
    // Find the point on the line lying in the projection plane, and
    // then find the near point PRJNPT on the projected ellipse.  Here
    // PRJPT is the point on the line lying in the projection plane.
    // The distance between PRJPT and PRJNPT is DIST.
    //
    //
    VPRJP(
        SCLPT.as_slice(),
        PRJPL.as_slice(),
        PRJPT.as_slice_mut(),
        ctx,
    )?;
    NPELPT(
        PRJPT.as_slice(),
        PRJEL.as_slice(),
        PRJNPT.as_slice_mut(),
        DIST,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"NPEDLN", ctx)?;
        return Ok(());
    }

    //
    // Find the near point PNEAR on the ellipsoid by taking the inverse
    // orthogonal projection of PRJNPT; this is the point on the
    // candidate ellipse that projects to PRJNPT.  Note that the
    // output DIST was computed in step 3 and needs only to be re-scaled.
    //
    // The inverse projection of PNEAR ought to exist, but may not
    // be calculable due to numerical problems (this can only happen
    // when the input ellipsoid is extremely flat or needle-shaped).
    //
    VPRJPI(
        PRJNPT.as_slice(),
        PRJPL.as_slice(),
        CANDPL.as_slice(),
        PNEAR.as_slice_mut(),
        &mut IFOUND,
        ctx,
    )?;

    if !IFOUND {
        SETMSG(b"Inverse projection could not be found.", ctx);
        SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
        CHKOUT(b"NPEDLN", ctx)?;
        return Ok(());
    }

    //
    // Undo the scaling.
    //
    VSCLIP(SCALE, PNEAR.as_slice_mut());

    *DIST = (SCALE * *DIST);

    CHKOUT(b"NPEDLN", ctx)?;
    Ok(())
}
