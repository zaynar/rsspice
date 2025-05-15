//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const UBPL: i32 = 4;
const MARGIN: f64 = 3.0;

/// Intersection of ray and plane
///
/// Find the intersection of a ray and a plane.
///
/// # Required Reading
///
/// * [PLANES](crate::required_reading::planes)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  VERTEX,
///  DIR        I   Vertex and direction vector of ray.
///  PLANE      I   A SPICE plane.
///  NXPTS      O   Number of intersection points of ray and plane.
///  XPT        O   Intersection point, if NXPTS = 1.
/// ```
///
/// # Detailed Input
///
/// ```text
///  VERTEX,
///  DIR      are a point and direction vector that define a
///           ray in three-dimensional space.
///
///  PLANE    is a SPICE plane.
/// ```
///
/// # Detailed Output
///
/// ```text
///  NXPTS    is the number of points of intersection of the
///           input ray and plane. Values and meanings of
///           NXPTS are:
///
///              0     No intersection.
///
///              1     One point of intersection. Note that
///                    this case may occur when the ray's
///                    vertex is in the plane.
///
///             -1     An infinite number of points of
///                    intersection; the ray lies in the plane.
///
///
///  XPT      is the point of intersection of the input ray
///           and plane, when there is exactly one point of
///           intersection. Otherwise, XPT is the zero vector.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the ray's direction vector is the zero vector, the error
///      SPICE(ZEROVECTOR) is signaled. NXPTS and XPT are not
///      modified.
///
///  2)  If the ray's vertex is further than DPMAX / 3 from the
///      origin, the error SPICE(VECTORTOOBIG) is signaled.  NXPTS
///      and XPT are not modified.
///
///  3)  If the input plane is further than DPMAX / 3 from the
///      origin, the error SPICE(VECTORTOOBIG) is signaled.  NXPTS
///      and XPT are not modified.
///
///  4)  The input plane should be created by one of the SPICELIB
///      routines
///
///         NVC2PL
///         NVP2PL
///         PSV2PL
///
///      Invalid input planes will cause unpredictable results.
///
///  5)  In the interest of good numerical behavior, in the case
///      where the ray's vertex is not in the plane, this routine
///      considers that an intersection of the ray and plane occurs
///      only if the distance between the ray's vertex and the
///      intersection point is less than DPMAX / 3.
///
///      If VERTEX is not in the plane and this condition is not
///      met, then NXPTS is set to 0 and XPT is set to the zero
///      vector.
/// ```
///
/// # Particulars
///
/// ```text
///  The intersection of a ray and plane in three-dimensional space
///  can be a the empty set, a single point, or the ray itself.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Find the camera projection of the center of an extended
///      body. For simplicity, we assume:
///
///         -- The camera has no distortion;  the image of a point
///            is determined by the intersection of the focal plane
///            and the line determined by the point and the camera's
///            focal point.
///
///         -- The camera's pointing matrix (C-matrix) is available
///            in a C-kernel.
///
///
///         C
///         C     Load Leapseconds and SCLK kernels to support time
///         C     conversion.
///         C
///               CALL FURNSH ( 'LEAP.KER' )
///               CALL FURNSH ( 'SCLK.KER' )
///
///         C
///         C     Load an SPK file containing ephemeris data for
///         C     observer (a spacecraft, whose NAIF integer code
///         C     is SC) and target at the UTC epoch of observation.
///         C
///               CALL FURNSH ( 'SPK.BSP' )
///
///         C
///         C     Load a C-kernel containing camera pointing for
///         C     the UTC epoch of observation.
///         C
///               CALL FURNSH ( 'CK.BC' )
///
///         C
///         C     Find the ephemeris time (barycentric dynamical time)
///         C     and encoded spacecraft clock times corresponding to
///         C     the UTC epoch of observation.
///         C
///               CALL UTC2ET ( UTC, ET          )
///               CALL SCE2C  ( SC,  ET,  SCLKDP )
///
///         C
///         C     Encode the pointing lookup tolerance.
///         C
///               CALL SCTIKS ( SC, TOLCH,  TOLDP  )
///
///         C
///         C     Find the observer-target vector at the observation
///         C     epoch. In this example, we'll use a light-time
///         C     corrected state vector.
///         C
///               CALL SPKEZ ( TARGET,  ET,  'J2000',  'LT',  SC,
///              .             STATE,   LT                        )
///
///         C
///         C     Look up camera pointing.
///         C
///               CALL CKGP  ( CAMERA, SCLKDP, TOLDP, 'J2000', CMAT,
///              .             CLKOUT, FOUND                        )
///
///               IF ( .NOT. FOUND ) THEN
///
///                  [Handle this case...]
///
///               END IF
///
///         C
///         C     Negate the spacecraft-to-target body vector and
///         C     convert it to camera coordinates.
///         C
///               CALL VMINUS ( STATE, DIR       )
///               CALL MXV    ( CMAT,  DIR,  DIR )
///
///         C
///         C     If FL is the camera's focal length, the effective
///         C     focal point is
///         C
///         C        FL * ( 0, 0, 1 )
///         C
///               CALL VSCL ( FL, ZVEC, FOCUS )
///
///         C
///         C     The camera's focal plane contains the origin in
///         C     camera coordinates, and the z-vector is orthogonal
///         C     to the plane.  Make a SPICE plane representing
///         C     the focal plane.
///         C
///               CALL NVC2PL ( ZVEC, 0.D0, FPLANE )
///
///         C
///         C     The image of the target body's center in the focal
///         C     plane is defined by the intersection with the focal
///         C     plane of the ray whose vertex is the focal point and
///         C     whose direction is DIR.
///         C
///               CALL INRYPL ( FOCUS, DIR, FPLANE, NXPTS, IMAGE )
///
///               IF ( NXPTS .EQ. 1 ) THEN
///         C
///         C        The body center does project to the focal plane.
///         C        Check whether the image is actually in the
///         C        camera's field of view...
///         C
///                            .
///                            .
///                            .
///               ELSE
///
///         C
///         C        The body center does not map to the focal plane.
///         C        Handle this case...
///         C
///                            .
///                            .
///                            .
///               END IF
///
///
///
///  2)  Find the Saturn ring plane intercept of a spacecraft-mounted
///      instrument's boresight vector.  We want the find the point
///      in the ring plane that will be observed by an instrument
///      with a give boresight direction at a specified time.  We
///      must account for light time and stellar aberration in order
///      to find this point.  The intercept point will be expressed
///      in Saturn body-fixed coordinates.
///
///      In this example, we assume
///
///         -- The ring plane is equatorial.
///
///         -- Light travels in a straight line.
///
///         -- The light time correction for the ring plane intercept
///            can be obtained by performing three light-time
///            correction iterations.  If this assumption does not
///            lead to a sufficiently accurate result, additional
///            iterations can be performed.
///
///         -- A Newtonian approximation of stellar aberration
///            suffices.
///
///         -- The boresight vector is given in J2000 coordinates.
///
///         -- The observation epoch is ET ephemeris seconds past
///            J2000.
///
///         -- The boresight vector, spacecraft and planetary
///            ephemerides, and ring plane orientation are all known
///            with sufficient accuracy for the application.
///
///         -- All necessary kernels are loaded by the caller of
///            this example routine.
///
///
///         SUBROUTINE RING_XPT ( SC, ET, BORVEC, SBFXPT, FOUND )
///         IMPLICIT NONE
///
///         CHARACTER*(*)         SC
///         DOUBLE PRECISION      ET
///         DOUBLE PRECISION      BORVEC ( 3 )
///         DOUBLE PRECISION      SBFXPT ( 3 )
///         LOGICAL               FOUND
///
///   C
///   C     SPICELIB functions
///   C
///         DOUBLE PRECISION      CLIGHT
///         DOUBLE PRECISION      VDIST
///
///   C
///   C     Local parameters
///   C
///         INTEGER               UBPL
///         PARAMETER           ( UBPL = 4 )
///
///         INTEGER               SATURN
///         PARAMETER           ( SATURN = 699 )
///
///   C
///   C     Local variables
///   C
///         DOUBLE PRECISION      BORV2  ( 3 )
///         DOUBLE PRECISION      CORVEC ( 3 )
///         DOUBLE PRECISION      LT
///         DOUBLE PRECISION      PLANE  ( UBPL )
///         DOUBLE PRECISION      SATSSB ( 6 )
///         DOUBLE PRECISION      SCPOS  ( 3 )
///         DOUBLE PRECISION      SCSSB  ( 6 )
///         DOUBLE PRECISION      STATE  ( 6 )
///         DOUBLE PRECISION      STCORR ( 3 )
///         DOUBLE PRECISION      TAU
///         DOUBLE PRECISION      TPMI   ( 3,  3 )
///         DOUBLE PRECISION      XPT    ( 3 )
///         DOUBLE PRECISION      ZVEC   ( 3 )
///
///         INTEGER               I
///         INTEGER               NXPTS
///         INTEGER               SCID
///
///         LOGICAL               FND
///
///   C
///   C     First step:  account for stellar aberration.  Since the
///   C     instrument pointing is given, we need to find the intercept
///   C     point such that, when the stellar aberration correction is
///   C     applied to the vector from the spacecraft to that point,
///   C     the resulting vector is parallel to BORVEC.  An easy
///   C     solution is to apply the inverse of the normal stellar
///   C     aberration correction to BORVEC, and then solve the
///   C     intercept problem with this corrected boresight vector.
///   C
///   C     Find the position of the observer relative
///   C     to the solar system barycenter at ET.
///   C
///         CALL BODN2C ( SC, SCID, FND )
///
///         IF ( .NOT. FND ) THEN
///
///            CALL SETMSG ( 'ID code for body # was not found.' )
///            CALL ERRCH  ( '#',  SC                            )
///            CALL SIGERR ( 'SPICE(NOTRANSLATION'               )
///            RETURN
///
///         END IF
///
///         CALL SPKSSB ( SCID, ET, 'J2000', SCSSB )
///
///   C
///   C     We now wish to find the vector CORVEC that, when
///   C     corrected for stellar aberration, yields BORVEC.
///   C     A good first approximation is obtained by applying
///   C     the stellar aberration correction for transmission
///   C     to BORVEC.
///   C
///         CALL STLABX ( BORVEC, SCSSB(4), CORVEC )
///
///   C
///   C     The inverse of the stellar aberration correction
///   C     applicable to CORVEC should be a very good estimate of
///   C     the correction we need to apply to BORVEC. Apply
///   C     this correction to BORVEC to obtain an improved estimate
///   C     of CORVEC.
///   C
///         CALL STELAB ( CORVEC, SCSSB(4), BORV2  )
///         CALL VSUB   ( BORV2,  CORVEC,   STCORR )
///         CALL VSUB   ( BORVEC, STCORR,   CORVEC )
///
///   C
///   C     Because the ring plane intercept may be quite far from
///   C     Saturn's center, we cannot assume light time from the
///   C     intercept to the observer is well approximated by
///   C     light time from Saturn's center to the observer.
///   C     We compute the light time explicitly using an iterative
///   C     approach.
///   C
///   C     We can however use the light time from Saturn's center to
///   C     the observer to obtain a first estimate of the actual light
///   C     time.
///   C
///         CALL SPKEZR ( 'SATURN', ET, 'J2000', 'LT', SC,
///        .               STATE,   LT                       )
///         TAU = LT
///
///   C
///   C     Find the ring plane intercept and calculate the
///   C     light time from it to the spacecraft.
///   C     Perform three iterations.
///   C
///         I     = 1
///         FOUND = .TRUE.
///
///         DO WHILE (  ( I .LE. 3 )  .AND.  ( FOUND )  )
///   C
///   C        Find the position of Saturn relative
///   C        to the solar system barycenter at ET-TAU.
///   C
///            CALL SPKSSB ( SATURN, ET-TAU, 'J2000', SATSSB )
///
///   C
///   C        Find the Saturn-to-observer vector defined by these
///   C        two position vectors.
///   C
///            CALL VSUB ( SCSSB, SATSSB, SCPOS )
///
///   C
///   C        Look up Saturn's pole at ET-TAU; this is the third
///   C        column of the matrix that transforms Saturn body-fixed
///   C        coordinates to J2000 coordinates.
///   C
///            CALL PXFORM ( 'IAU_SATURN', 'J2000', ET-TAU, TPMI )
///
///            CALL MOVED  ( TPMI(1,3), 3, ZVEC )
///
///   C
///   C        Make a SPICE plane representing the ring plane.
///   C        We're treating Saturn's center as the origin, so
///   C        the plane constant is 0.
///   C
///            CALL NVC2PL ( ZVEC, 0.D0, PLANE )
///
///   C
///   C        Find the intersection of the ring plane and the
///   C        ray having vertex SCPOS and direction vector
///   C        CORVEC.
///   C
///            CALL INRYPL ( SCPOS, CORVEC, PLANE, NXPTS, XPT )
///
///   C
///   C        If the number of intersection points is 1,
///   C        find the next light time estimate.
///   C
///            IF ( NXPTS .EQ. 1 ) THEN
///   C
///   C           Find the light time (zero-order) from the
///   C           intercept point to the spacecraft.
///   C
///               TAU  =  VDIST ( SCPOS, XPT )  /  CLIGHT()
///               I    =  I + 1
///
///            ELSE
///
///               FOUND = .FALSE.
///
///            END IF
///
///         END DO
///
///   C
///   C     At this point, if FOUND is .TRUE., we iterated
///   C     3 times, and XPT is our estimate of the
///   C     position of the ring plane intercept point
///   C     relative to Saturn in the J2000 frame. This is the
///   C     point observed by an instrument pointed in direction
///   C     BORVEC at ET at mounted on the spacecraft SC.
///   C
///   C     If FOUND is .FALSE., the boresight ray does not
///   C     intersect the ring plane.
///   C
///   C     As a final step, transform XPT to Saturn body-fixed
///   C     coordinates.
///   C
///         IF ( FOUND ) THEN
///
///            CALL MTXV ( TPMI, XPT, SBFXPT )
///
///         END IF
///
///         END
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
/// -    SPICELIB Version 1.3.0, 24-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.2.0, 29-SEP-2016 (NJB)
///
///         Changed from standard to discovery check-in. Fixed typo
///         in header.
///
/// -    SPICELIB Version 1.1.1, 07-FEB-2008 (BVS)
///
///         Fixed a few typos in the header.
///
/// -    SPICELIB Version 1.1.0, 02-SEP-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in VSCL call.
///
/// -    SPICELIB Version 1.0.3, 12-DEC-2002 (NJB)
///
///         Header fix: ring plane intercept algorithm was corrected.
///         Now light time is computed accurately, and stellar aberration
///         is accounted for. Example was turned into a complete
///         subroutine.
///
/// -    SPICELIB Version 1.0.2, 09-MAR-1999 (NJB)
///
///         Reference to SCE2T replaced by reference to SCE2C. An
///         occurrence of ENDIF was replaced by END IF.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 01-APR-1991 (NJB) (WLT)
/// ```
pub fn inrypl(
    ctx: &mut SpiceContext,
    vertex: &[f64; 3],
    dir: &[f64; 3],
    plane: &[f64; 4],
    nxpts: &mut i32,
    xpt: &mut [f64; 3],
) -> crate::Result<()> {
    INRYPL(vertex, dir, plane, nxpts, xpt, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure INRYPL ( Intersection of ray and plane )
pub fn INRYPL(
    VERTEX: &[f64],
    DIR: &[f64],
    PLANE: &[f64],
    NXPTS: &mut i32,
    XPT: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let VERTEX = DummyArray::new(VERTEX, 1..=3);
    let DIR = DummyArray::new(DIR, 1..=3);
    let PLANE = DummyArray::new(PLANE, 1..=UBPL);
    let mut XPT = DummyArrayMut::new(XPT, 1..=3);
    let mut CONST: f64 = 0.0;
    let mut PRJDIF: f64 = 0.0;
    let mut PRJDIR: f64 = 0.0;
    let mut PRJVN: f64 = 0.0;
    let mut MSCALE: f64 = 0.0;
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
    let mut SCALE: f64 = 0.0;
    let mut SCLCON: f64 = 0.0;
    let mut SCLVTX = StackArray::<f64, 3>::new(1..=3);
    let mut TOOBIG: f64 = 0.0;
    let mut UDIR = StackArray::<f64, 3>::new(1..=3);

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

    //
    // We'll give the name TOOBIG to the bound DPMAX() / MARGIN.
    // If we let VTXPRJ be the orthogonal projection of VERTEX onto
    // PLANE, and let DIFF be the vector VTXPRJ - VERTEX, then
    // we know that
    //
    //    ||  DIFF  ||    <     2 * TOOBIG
    //
    // Check the distance of the ray's vertex from the origin.
    //
    TOOBIG = (DPMAX() / MARGIN);

    if (VNORM(VERTEX.as_slice()) >= TOOBIG) {
        CHKIN(b"INRYPL", ctx)?;
        SETMSG(b"Ray\'s vertex is too far from the origin.", ctx);
        SIGERR(b"SPICE(VECTORTOOBIG)", ctx)?;
        CHKOUT(b"INRYPL", ctx)?;
        return Ok(());
    }

    //
    // Check the distance of the plane from the origin.  (The returned
    // plane constant IS this distance.)
    //
    PL2NVC(PLANE.as_slice(), NORMAL.as_slice_mut(), &mut CONST);

    if (CONST >= TOOBIG) {
        CHKIN(b"INRYPL", ctx)?;
        SETMSG(b"Plane is too far from the origin.", ctx);
        SIGERR(b"SPICE(VECTORTOOBIG)", ctx)?;
        CHKOUT(b"INRYPL", ctx)?;
        return Ok(());
    }

    //
    // Check the ray's direction vector.
    //
    VHAT(DIR.as_slice(), UDIR.as_slice_mut());

    if VZERO(UDIR.as_slice()) {
        CHKIN(b"INRYPL", ctx)?;
        SETMSG(b"Ray\'s direction vector is the zero vector.", ctx);
        SIGERR(b"SPICE(ZEROVECTOR)", ctx)?;
        CHKOUT(b"INRYPL", ctx)?;
        return Ok(());
    }

    //
    // That takes care of the error cases.  Now scale the input vertex
    // and plane to improve numerical behavior.
    //
    MSCALE = intrinsics::DMAX1(&[CONST, VNORM(VERTEX.as_slice())]);

    if (MSCALE != 0.0) {
        VSCL((1.0 / MSCALE), VERTEX.as_slice(), SCLVTX.as_slice_mut());
        SCLCON = (CONST / MSCALE);
    } else {
        VEQU(VERTEX.as_slice(), SCLVTX.as_slice_mut());
        SCLCON = CONST;
    }

    if (MSCALE > 1.0) {
        TOOBIG = (TOOBIG / MSCALE);
    }

    // Find the projection (coefficient) of the ray's vertex along the
    // plane's normal direction.
    //
    PRJVN = VDOT(SCLVTX.as_slice(), NORMAL.as_slice());

    //
    // If this projection is the plane constant, the ray's vertex lies in
    // the plane.  We have one intersection or an infinite number of
    // intersections.  It all depends on whether the ray actually lies
    // in the plane.
    //
    // The absolute value of PRJDIF is the distance of the ray's vertex
    // from the plane.
    //
    PRJDIF = (SCLCON - PRJVN);

    if (PRJDIF == 0.0) {
        //
        // XPT is the original, unscaled vertex.
        //
        VEQU(VERTEX.as_slice(), XPT.as_slice_mut());

        if (VDOT(NORMAL.as_slice(), UDIR.as_slice()) == 0.0) {
            //
            // The ray's in the plane.
            //
            *NXPTS = -1;
        } else {
            *NXPTS = 1;
        }

        return Ok(());
    }

    //
    //  Ok, the ray's vertex is not in the plane.  The ray may still be
    //  parallel to or may point away from the plane.  If the ray does
    //  point towards the plane, mathematicians would say that the
    //  ray does intersect the plane, but the computer may disagree.
    //
    //  For this routine to find an intersection, both of the following
    //  conditions must be met:
    //
    //     -- The ray must point toward the plane; this happens when
    //        PRJDIF has the same sign as < UDIR, NORMAL >.
    //
    //     -- The vector difference XPT - SCLVTX must not overflow.
    //
    //   Qualitatively, the case of interest looks something like the
    //   picture below:
    //
    //
    //                   * SCLVTX
    //                   |\
    //                   | \   <-- UDIR
    //                   |  \
    // length of this    |   \|
    //   segment is      |   -*
    //                   |
    //  | PRJDIF |   --> | ___________________________
    //                   |/                          /
    //                   |       *                  /   <-- PLANE
    //                  /|        XPT              /
    //                 / ^                        /
    //                /  | NORMAL                /
    //               /   | .                    /
    //              /    |/|                   /
    //             / .---| /                  /
    //            /  |   |/                  /
    //           /   `---*                  /
    //          /          Projection of SCLVTX onto the plane
    //         /                          /
    //        /                          /
    //       ----------------------------
    //
    //
    //

    //
    // Find the projection of the direction vector along the plane's
    // normal vector.
    //
    PRJDIR = VDOT(UDIR.as_slice(), NORMAL.as_slice());

    //
    // We're done if the ray doesn't point toward the plane.  PRJDIF
    // has already been found to be non-zero at this point; PRJDIR is
    // zero if the ray and plane are parallel.  The SPICELIB routine
    // SMSGND will return a value of .FALSE. if PRJDIR is zero.
    //
    if !SMSGND(PRJDIR, PRJDIF) {
        //
        // The ray is parallel to or points away from the plane.
        //
        *NXPTS = 0;
        CLEARD(3, XPT.as_slice_mut());

        return Ok(());
    }

    //
    // The difference XPT - SCLVTX is the hypotenuse of a right triangle
    // formed by SCLVTX, XPT, and the orthogonal projection of SCLVTX
    // onto the plane.  We'll obtain the hypotenuse by scaling UDIR.
    // We must make sure that this hypotenuse does not overflow.  The
    // scale factor has magnitude
    //
    //     | PRJDIF |
    //   --------------
    //     | PRJDIR |
    //
    // and UDIR is a unit vector, so as long as
    //
    //     | PRJDIF |   <   | PRJDIR |  *  TOOBIG
    //
    // the hypotenuse is no longer than TOOBIG.  The product can be
    // computed safely since PRJDIR has magnitude 1 or less.
    //

    if (f64::abs(PRJDIF) >= (f64::abs(PRJDIR) * TOOBIG)) {
        //
        // If the hypotenuse is too long, we say that no intersection
        // exists.
        //
        *NXPTS = 0;
        CLEARD(3, XPT.as_slice_mut());

        return Ok(());
    }

    //
    // We conclude that it's safe to compute XPT.  Scale UDIR and add
    // the result to SCLVTX.  The addition is safe because both addends
    // have magnitude no larger than TOOBIG.  The vector thus obtained
    // is the intersection point.
    //
    *NXPTS = 1;
    SCALE = (f64::abs(PRJDIF) / f64::abs(PRJDIR));

    VLCOM(
        1.0,
        SCLVTX.as_slice(),
        SCALE,
        UDIR.as_slice(),
        XPT.as_slice_mut(),
    );

    //
    // Re-scale XPT.  This is safe, since TOOBIG has already been
    // scaled to allow for any growth of XPT at this step.
    //
    VSCLIP(MSCALE, XPT.as_slice_mut());

    Ok(())
}
