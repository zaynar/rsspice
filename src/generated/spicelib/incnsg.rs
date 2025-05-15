//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const COSTOL: f64 = 0.0000000001;
const HSPTOL: f64 = 0.00000000000001;
const UBPL: i32 = 4;

struct SaveVars {
    ORIGIN: StackArray<f64, 3>,
    Y: StackArray<f64, 3>,
    Z: StackArray<f64, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ORIGIN = StackArray::<f64, 3>::new(1..=3);
        let mut Y = StackArray::<f64, 3>::new(1..=3);
        let mut Z = StackArray::<f64, 3>::new(1..=3);

        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::D(0.0), 3 as usize))
                .chain([]);

            ORIGIN
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(1.0), Val::D(0.0)].into_iter();
            Y.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            Z.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { ORIGIN, Y, Z }
    }
}

/// Intersection of cone and line segment
///
/// Compute the points of intersection of a specified nappe of a cone
/// and a line segment.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  APEX       I   Apex of cone.
///  AXIS       I   Axis of cone.
///  ANGLE      I   Angle of cone.
///  ENDPT1,
///  ENDPT2     I   Endpoints of line segment.
///  NXPTS      O   Number of intersection points.
///  XPT1       O   First intersection point, if it exists.
///  XPT2       O   Second intersection point, if it exists.
/// ```
///
/// # Detailed Input
///
/// ```text
///  APEX     is the apex (tip) of the cone. In this routine's
///           documentation, we'll consider the cone to be a
///           semi-infinite pyramid with circular cross-section. In
///           some contexts, this object is called one "nappe" of
///           the complete cone.
///
///  AXIS     is an axis vector of the cone.
///
///  ANGLE    is the angular separation from AXIS of the rays
///           comprising the cone. Let the notation
///
///              < A, B >
///
///           denote the dot product of vectors A and B, and let
///
///              ||A||
///
///           denote the norm of vector A. Then the cone is the set
///           of points
///
///                        X-APEX       AXIS
///              { X:  < ----------,  -------- >  =  cos(ANGLE) }
///                      ||X-APEX||   ||AXIS||
///
///
///  ENDPT1,
///  ENDPT2   are endpoints of a line segment. These points
///           must be distinct.
/// ```
///
/// # Detailed Output
///
/// ```text
///  NXPTS    is the number of points of intersection of the input
///           line segment and cone.
///
///  XPT1     is the point of intersection of the segment and cone
///           that is closest to ENDPT1, if an intersection exists.
///           If there are no intersections, XPT1 is undefined.
///
///  XPT2     is the point of intersection of the segment and cone
///           that is farthest from ENDPT1, if two points of
///           intersection exist. If there are not two
///           intersections, XPT2 is undefined.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If AXIS is the zero vector, the error SPICE(ZEROVECTOR)
///      is signaled.
///
///  2)  If ANGLE is less than zero, the error SPICE(INVALIDANGLE)
///      is signaled.
///
///  3)  If ENDPT1 and ENDPT2 coincide, the error
///      SPICE(ENDPOINTSMATCH) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is used by the SPICELIB DSK subsystem. In
///  particular, it is used to determine whether a ray contacts a
///  latitude boundary of a volume element in either planetocentric
///  latitudinal or planetodetic coordinates.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as input
///  (if any), the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Compute the intersection of a line segment and cone in
///     a simple case for which the results can easily be checked.
///
///     Let the apex of the cone be at the origin. Let the axis
///     of the cone lie on the +X axis. Let the angle of the cone
///     be 45 degrees. Let the line segment have endpoints
///
///        ENDPT1 = ( 1,   -2, sqrt(3)/2 )
///        ENDPT2 = ( 1,    2, sqrt(3)/2 )
///
///     We expect there to be two points of intersection:
///
///        XPT1   = ( 1, -1/2, sqrt(3)/2 )
///        XPT2   = ( 1,  1/2, sqrt(3)/2 )
///
///
///     Example code begins here.
///
///
///           PROGRAM INCNSG_EX1
///           IMPLICIT NONE
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      RPD
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         FMT1
///           PARAMETER           ( FMT1 = '(A,3F13.8)' )
///
///           CHARACTER*(*)         FMT2
///           PARAMETER           ( FMT2 = '(A,I2)' )
///     C
///     C     Local variables
///     C
///           DOUBLE PRECISION      ANGLE
///           DOUBLE PRECISION      APEX   ( 3 )
///           DOUBLE PRECISION      AXIS   ( 3 )
///           DOUBLE PRECISION      ENDPT1 ( 3 )
///           DOUBLE PRECISION      ENDPT2 ( 3 )
///           DOUBLE PRECISION      SQ3
///           DOUBLE PRECISION      XPT1   ( 3 )
///           DOUBLE PRECISION      XPT2   ( 3 )
///
///           INTEGER               NXPTS
///
///     C
///     C     Set up the cone's geometric attributes.
///     C
///           CALL VPACK ( 0.D0, 0.D0, 0.D0, APEX )
///           CALL VPACK ( 1.D0, 0.D0, 0.D0, AXIS )
///
///           ANGLE = 45.D0 * RPD()
///     C
///     C     Initialize the line segment's endpoints.
///     C
///           SQ3 = SQRT( 3.D0  )
///
///           CALL VPACK ( 1.D0, -2.D0, SQ3/2, ENDPT1 )
///           CALL VPACK ( 1.D0,  2.D0, SQ3/2, ENDPT2 )
///     C
///     C     Find the points of intersection.
///     C
///           CALL INCNSG ( APEX,   AXIS,  ANGLE, ENDPT1,
///          .              ENDPT2, NXPTS, XPT1,  XPT2   )
///
///           WRITE (*,*) ' '
///           WRITE (*,FMT1) 'Apex:        ', APEX
///           WRITE (*,FMT1) 'Axis:        ', AXIS
///           WRITE (*,FMT1) 'Angle (deg): ', ANGLE/RPD()
///           WRITE (*,FMT1) 'Endpoint 1:  ', ENDPT1
///           WRITE (*,FMT1) 'Endpoint 2:  ', ENDPT2
///           WRITE (*,*) ' '
///           WRITE (*,FMT2) 'Number of intersection points: ',
///          .            NXPTS
///           WRITE (*,*) ' '
///           WRITE (*,FMT1) 'Point 1:    ', XPT1
///           WRITE (*,FMT1) 'Point 2:    ', XPT2
///           WRITE (*,*) ' '
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Apex:           0.00000000   0.00000000   0.00000000
///     Axis:           1.00000000   0.00000000   0.00000000
///     Angle (deg):   45.00000000
///     Endpoint 1:     1.00000000  -2.00000000   0.86602540
///     Endpoint 2:     1.00000000   2.00000000   0.86602540
///
///     Number of intersection points:  2
///
///     Point 1:       1.00000000  -0.50000000   0.86602540
///     Point 2:       1.00000000   0.50000000   0.86602540
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine is designed to avoid arithmetic overflow in
///      normal cases, such as those in which the line segment is
///      nearly parallel to the cone. However, it is possible to cause
///      arithmetic overflow by using input vectors with extremely
///      large magnitudes.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 06-JUL-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 26-OCT-2016 (NJB)
/// ```
pub fn incnsg(
    ctx: &mut SpiceContext,
    apex: &[f64; 3],
    axis: &[f64; 3],
    angle: f64,
    endpt1: &[f64; 3],
    endpt2: &[f64; 3],
    nxpts: &mut i32,
    xpt1: &mut [f64; 3],
    xpt2: &mut [f64; 3],
) -> crate::Result<()> {
    INCNSG(
        apex,
        axis,
        angle,
        endpt1,
        endpt2,
        nxpts,
        xpt1,
        xpt2,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure INCNSG ( Intersection of cone and line segment )
pub fn INCNSG(
    APEX: &[f64],
    AXIS: &[f64],
    ANGLE: f64,
    ENDPT1: &[f64],
    ENDPT2: &[f64],
    NXPTS: &mut i32,
    XPT1: &mut [f64],
    XPT2: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let APEX = DummyArray::new(APEX, 1..=3);
    let AXIS = DummyArray::new(AXIS, 1..=3);
    let ENDPT1 = DummyArray::new(ENDPT1, 1..=3);
    let ENDPT2 = DummyArray::new(ENDPT2, 1..=3);
    let mut XPT1 = DummyArrayMut::new(XPT1, 1..=3);
    let mut XPT2 = DummyArrayMut::new(XPT2, 1..=3);
    let mut A: f64 = 0.0;
    let mut AXMAG: f64 = 0.0;
    let mut B: f64 = 0.0;
    let mut C: f64 = 0.0;
    let mut CA2: f64 = 0.0;
    let mut COLAT: f64 = 0.0;
    let mut COSANG: f64 = 0.0;
    let mut COSERR: f64 = 0.0;
    let mut DIR = StackArray::<f64, 3>::new(1..=3);
    let mut DMAG: f64 = 0.0;
    let mut DP1: f64 = 0.0;
    let mut DP2: f64 = 0.0;
    let mut LOCANG: f64 = 0.0;
    let mut MAXLAT: f64 = 0.0;
    let mut MAXP = StackArray::<f64, 3>::new(1..=3);
    let mut MINLAT: f64 = 0.0;
    let mut MINP = StackArray::<f64, 3>::new(1..=3);
    let mut NRMPLN = StackArray::<f64, 4>::new(1..=UBPL);
    let mut OFF1 = StackArray::<f64, 3>::new(1..=3);
    let mut OFF2 = StackArray::<f64, 3>::new(1..=3);
    let mut PLNX = StackArray::<f64, 3>::new(1..=3);
    let mut S1: f64 = 0.0;
    let mut S2: f64 = 0.0;
    let mut UAXIS = StackArray::<f64, 3>::new(1..=3);
    let mut UDIR = StackArray::<f64, 3>::new(1..=3);
    let mut UOFF1 = StackArray::<f64, 3>::new(1..=3);
    let mut UOFF2 = StackArray::<f64, 3>::new(1..=3);
    let mut UUAX: f64 = 0.0;
    let mut UV1 = StackArray::<f64, 3>::new(1..=3);
    let mut UV2 = StackArray::<f64, 3>::new(1..=3);
    let mut V1 = StackArray::<f64, 3>::new(1..=3);
    let mut V1MAG: f64 = 0.0;
    let mut V2 = StackArray::<f64, 3>::new(1..=3);
    let mut V2MAG: f64 = 0.0;
    let mut VTEMP = StackArray::<f64, 3>::new(1..=3);
    let mut VTEMP2 = StackArray::<f64, 3>::new(1..=3);
    let mut W2: f64 = 0.0;
    let mut WU: f64 = 0.0;
    let mut WUAX: f64 = 0.0;
    let mut X = StackArray::<f64, 3>::new(1..=3);
    let mut XOFF1 = StackArray::<f64, 3>::new(1..=3);
    let mut XOFF2 = StackArray::<f64, 3>::new(1..=3);
    let mut XFORM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut N: i32 = 0;
    let mut NPLNX: i32 = 0;
    let mut IN1: bool = false;
    let mut IN2: bool = false;
    let mut ISBRCK: bool = false;
    let mut NEG1: bool = false;
    let mut NEG2: bool = false;

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
    // Saved values
    //

    //
    // Initial values
    //

    //
    // Use quasi-discovery check-in. We'll check in before
    // code sections that can generate SPICE errors, and check
    // out afterward. When those code sections are skipped,
    // we avoid traceback participation.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // No intersection was found so far.
    //
    *NXPTS = 0;

    //
    // The cone's axis vector must be non-zero.
    //
    UNORM(AXIS.as_slice(), UAXIS.as_slice_mut(), &mut AXMAG);

    if (AXMAG == 0.0) {
        CHKIN(b"INCNSG", ctx)?;
        SETMSG(b"The cone\'s axis vector must be non-zero but sadly, it failed to meet this criterion.", ctx);
        SIGERR(b"SPICE(ZEROVECTOR)", ctx)?;
        CHKOUT(b"INCNSG", ctx)?;
        return Ok(());
    }

    //
    // The cone's angular radius must be non-negative.
    //
    if (ANGLE < 0.0) {
        CHKIN(b"INCNSG", ctx)?;
        SETMSG(
            b"The cone\'s angular radius must be  non-negative but was # (radians).",
            ctx,
        );
        ERRDP(b"#", ANGLE, ctx);
        SIGERR(b"SPICE(INVALIDANGLE)", ctx)?;
        CHKOUT(b"INCNSG", ctx)?;
        return Ok(());
    }

    //
    // The endpoints of the segment must be distinct. Check this after
    // computing a unit direction vector for the line segment.
    //
    VSUB(ENDPT2.as_slice(), ENDPT1.as_slice(), DIR.as_slice_mut());

    UNORM(DIR.as_slice(), UDIR.as_slice_mut(), &mut DMAG);

    if (DMAG == 0.0) {
        CHKIN(b"INCNSG", ctx)?;
        SETMSG(
            b"The distance between the segment\'s endpoints was zero. First endpoint: (# # #).",
            ctx,
        );
        ERRDP(b"#", ENDPT1[1], ctx);
        ERRDP(b"#", ENDPT1[2], ctx);
        ERRDP(b"#", ENDPT1[3], ctx);
        SIGERR(b"SPICE(ENDPOINTSMATCH)", ctx)?;
        CHKOUT(b"INCNSG", ctx)?;
        return Ok(());
    }

    //
    // Store the cosine of the cone's angular radius. We'll treat all
    // cases with COSANG equal to 0 as though the cone is actually a
    // plane normal to the axis and containing the apex.
    //
    COSANG = f64::cos(ANGLE);
    LOCANG = ANGLE;
    //
    // We'll work with a local axis that has angular separation of
    // no more than pi/2 from the nappe.
    //
    if (COSANG < 0.0) {
        COSANG = -COSANG;
        LOCANG = (PI(ctx) - ANGLE);

        UAXIS[1] = -UAXIS[1];
        UAXIS[2] = -UAXIS[2];
        UAXIS[3] = -UAXIS[3];
    }

    //
    // Compute the offsets of the endpoints of the segment from
    // the cone's apex.
    //
    VSUB(ENDPT1.as_slice(), APEX.as_slice(), OFF1.as_slice_mut());
    VSUB(ENDPT2.as_slice(), APEX.as_slice(), OFF2.as_slice_mut());

    //
    // Deal with some of the simple cases first.
    //
    VHAT(OFF1.as_slice(), UOFF1.as_slice_mut());
    VHAT(OFF2.as_slice(), UOFF2.as_slice_mut());

    DP1 = VDOT(UOFF1.as_slice(), UAXIS.as_slice());
    DP2 = VDOT(UOFF2.as_slice(), UAXIS.as_slice());

    //
    // The given axis is inside the nappe defined by the angular radius.
    //
    // There's no intersection if both endpoints are in the interior of
    // the nappe of the cone (since the nappe is convex).
    //
    IN1 = (DP1 >= COSANG);
    IN2 = (DP2 >= COSANG);
    //
    // If the line segment lies on the far side of the plane that
    // contains the apex and is orthogonal to the axis, there's no
    // intersection.
    //
    NEG1 = (DP1 < 0.0);
    NEG2 = (DP2 < 0.0);

    if ((IN1 && IN2) || (NEG1 && NEG2)) {
        //
        // The segment is in the interior of the cone or
        // on the far side of the plane.
        //
        *NXPTS = 0;

        return Ok(());
    }

    //
    // Here's where we handle the half-space case.
    //
    if (f64::abs(COSANG) < HSPTOL) {
        //
        // See whether the ray emanating from the first endpoint and
        // having direction UDIR hits the plane normal to the axis and
        // containing the apex. We'll call this plane NRMPLN.
        //
        // NVP2PL can signal an error only if the input axis is the
        // zero vector. We've ensured that it isn't.
        //
        NVP2PL(
            UAXIS.as_slice(),
            APEX.as_slice(),
            NRMPLN.as_slice_mut(),
            ctx,
        )?;
        INRYPL(
            ENDPT1.as_slice(),
            UDIR.as_slice(),
            NRMPLN.as_slice(),
            &mut NPLNX,
            PLNX.as_slice_mut(),
            ctx,
        )?;

        //
        // If the ray doesn't hit the plane, we're done. Otherwise,
        // check the intercept.
        //
        if (NPLNX == 1) {
            //
            // The ray does hit the plane. If the intersection is on the
            // line segment, we have a solution.
            //
            if (VDIST(PLNX.as_slice(), ENDPT1.as_slice()) <= DMAG) {
                //
                // The intercept is not further along the ray than the
                // second endpoint. It's a valid solution.
                //
                *NXPTS = 1;
                VEQU(PLNX.as_slice(), XPT1.as_slice_mut());
            }
        }
        //
        // This is the end of the half-space case.
        //
        return Ok(());
    }

    //
    // At this point we've disposed of the trivial cases. We'll
    // set up a quadratic equation for the intersection of the
    // line segment with the surface of the cone's nappe.
    //
    // Due to round-off errors, the solution of the quadratic may
    // either be inaccurate or may not be found at all. We'll
    // examine the solutions we find and solve the problem by
    // an alternate method if necessary. However, the quadratic
    // method is fast, so we give it priority.
    //
    // The equation of a ray starting at ENDPT1 and having unit
    // direction vector UDIR is
    //
    //    RAY  = { ENDPT1 + s*UDIR, s >= 0 }                          (1)
    //
    // The equation of the nappe of the cone is
    //
    //    CONE = { X: < X - APEX, UAXIS > = ||X-APEX|| * cos(ANGLE) } (2)
    //
    // where ANGLE is the angular radius of the cone and UAXIS is the
    // unit axis vector. Substituting the right hand side expression of
    // (1) for X in equation (2) and squaring both sides yields a
    // quadratic equation for S. We'll derive the coefficients of the
    // equation below.
    //
    // Let
    //
    //    Q  = X - APEX
    //    W  = ENDPT1 - APEX
    //    U  = UDIR
    //    CA = cos(ANGLE)
    //
    // We can translate the cone and ray by -APEX, and (1) and (2)
    // can be re-written as
    //
    //    RAY  = { W + s*U, s >= 0 }                                  (3)
    //
    //    CONE = { Q: < Q, UAXIS > = ||Q|| * cos(ANGLE) }             (4)
    //
    //
    //    Substituting the ray expression for Q, we obtain
    //
    //       < W + s*U, UAXIS > = ||W+s*U|| * CA                      (5)
    //
    //    and squaring both sides yields
    //
    //                  2                                      2   2
    //         <W,UAXIS>  + 2*<W,UAXIS>*<U,UAXIS>*s + <U,UAXIS> * s
    //
    //                2                2       2
    //       = ( ||W||  + 2*<W,U>*s + s  ) * CA                       (6)
    //
    //
    //   Collecting coefficients of powers of s, we have
    //
    //                     2     2     2
    //          ( <U,UAXIS>  - CA ) * s
    //
    //                                        2
    //        + 2 * ( <W,UAXIS>*<U,UAXIS> - CA * <W,U> ) * s
    //
    //                   2        2    2
    //        + <W,UAXIS>  - ||W|| * CA
    //
    //
    //     =  0                                                       (7)
    //
    //
    //  Before continuing, we observe that the only non-unit vector
    //  in (7) is W. So the coefficients in (7) have no possibility
    //  of overflowing unless the vertex of the ray is very far from
    //  the apex of the cone.
    //
    //  W has been computed above as OFF1.
    //
    //
    //     [ Consider adding check on OFF1 here. ]
    //
    //
    // Intermediate values:
    //
    UUAX = VDOT(UDIR.as_slice(), UAXIS.as_slice());
    WUAX = VDOT(OFF1.as_slice(), UAXIS.as_slice());
    WU = VDOT(OFF1.as_slice(), UDIR.as_slice());
    W2 = VDOT(OFF1.as_slice(), OFF1.as_slice());
    CA2 = (COSANG * COSANG);

    //
    // Quadratic coefficients:
    //
    A = ((UUAX * UUAX) - CA2);

    B = ((2 as f64) * ((WUAX * UUAX) - (CA2 * WU)));

    C = ((WUAX * WUAX) - (W2 * CA2));

    //
    // We're not interested in solutions that lie outside
    // of the line segment. The length of the segment is
    // DMAG.
    //
    // Solve the equation, using DMAG as an upper bound
    // on the magnitude of the roots.
    //
    ZZCNQUAD(A, B, C, DMAG, &mut N, &mut S1, &mut S2, ctx)?;

    //
    // Compute the possible intersection points and test them
    // to make sure they really are solutions.
    //
    if (N > 0) {
        //
        // Start with the solution closest to the ray's vertex.
        // Compute XPT1 and make sure it's on the correct nappe
        // of the cone.
        //
        if (S1 >= 0.0) {
            XPT1[1] = (ENDPT1[1] + (S1 * UDIR[1]));
            XPT1[2] = (ENDPT1[2] + (S1 * UDIR[2]));
            XPT1[3] = (ENDPT1[3] + (S1 * UDIR[3]));

            VSUB(XPT1.as_slice(), APEX.as_slice(), V1.as_slice_mut());

            //
            // See whether V1 is on the cone.
            //
            UNORM(V1.as_slice(), UV1.as_slice_mut(), &mut V1MAG);

            if (V1MAG > 0.0) {
                COSERR = f64::abs((VDOT(UV1.as_slice(), UAXIS.as_slice()) - COSANG));
            } else {
                COSERR = 0.0;
            }

            if ((V1MAG == 0.0) || (COSERR < COSTOL)) {
                //
                // The root is on the cone (on the apex if V1MAG is zero).
                //
                // We accept this root. Update NXPTS. Note that this is
                // not necessarily the final value of NXPTS; that
                // depends on the validity of the second root.

                *NXPTS = 1;
            }
        }

        if (N == 2) {
            //
            // Check the second root.
            //
            if (S2 >= 0.0) {
                XPT2[1] = (ENDPT1[1] + (S2 * UDIR[1]));
                XPT2[2] = (ENDPT1[2] + (S2 * UDIR[2]));
                XPT2[3] = (ENDPT1[3] + (S2 * UDIR[3]));

                VSUB(XPT2.as_slice(), APEX.as_slice(), V2.as_slice_mut());

                //
                // See whether V2 is on the cone.
                //
                UNORM(V2.as_slice(), UV2.as_slice_mut(), &mut V2MAG);

                if (V2MAG > 0.0) {
                    COSERR = f64::abs((VDOT(UV2.as_slice(), UAXIS.as_slice()) - COSANG));
                } else {
                    COSERR = 0.0;
                }

                if ((V2MAG == 0.0) || (COSERR < COSTOL)) {
                    //
                    // The root is on the cone (on the apex if V2MAG is
                    // zero).
                    //
                    // We accept this root.
                    //
                    *NXPTS = (*NXPTS + 1);

                    if (*NXPTS == 1) {
                        //
                        // This is the only valid root; overwrite XPT1.
                        //
                        VEQU(XPT2.as_slice(), XPT1.as_slice_mut());
                    }
                }
            }
        }
    }

    //
    // We're not done yet. If we have fewer roots than we should, we'll
    // need to solve the problem by an alternate method.
    //
    // If we have two roots, we're in good shape. Otherwise we must
    // determine how many roots should be found.
    //
    if (*NXPTS < 2) {
        //
        // We must determine the expected number of roots, and if
        // we didn't come up with them, we must find the roots
        // by an alternate method.
        //
        // We'll examine the containment of the endpoints within the
        // cone.
        //
        // The case where both endpoints are inside the cone was handled
        // earlier.
        //
        // If one endpoint is inside the cone and one is outside,
        // we expect to have one root.
        //
        if ((IN1 && !IN2) || (IN2 && !IN1)) {
            //
            // There's supposed to be one root. If we found none, find one
            // now.
            //
            if (*NXPTS == 0) {
                //
                // ZZCXBRUT signals an error if the axis is the zero
                // vector, but not otherwise. We've already ruled out this
                // situation. Therefore, we don't check in before the
                // following call.
                //
                ZZCXBRUT(
                    APEX.as_slice(),
                    UAXIS.as_slice(),
                    LOCANG,
                    ENDPT1.as_slice(),
                    ENDPT2.as_slice(),
                    XPT1.as_slice_mut(),
                    &mut ISBRCK,
                    ctx,
                )?;

                if ISBRCK {
                    //
                    // As long as the root was bracketed, XPT1 is a
                    // solution.
                    //
                    *NXPTS = 1;
                }
            }
        } else {
            CHKIN(b"INCNSG", ctx)?;
            //
            // Both endpoints are outside the cone. We could have zero to
            // two roots. If the minimum angular separation of the segment
            // from the axis is less than ANGLE, we expect to find two
            // roots; if it's equal to ANGLE, we expect to find one, and
            // if it's greater than ANGLE, none.
            //
            // We'll transform OFF1 and OFF2 into a reference frame in
            // which angular separation from the axis is equivalent to
            // colatitude. Then we'll find the maximum latitude attained
            // on the segment.
            //
            // We'll count the roots we find, so we'll start at zero.
            //
            *NXPTS = 0;

            FRAME(
                UAXIS.as_slice_mut(),
                X.as_slice_mut(),
                save.Y.as_slice_mut(),
            );

            for I in 1..=3 {
                XFORM[[1, I]] = X[I];
                XFORM[[2, I]] = save.Y[I];
                XFORM[[3, I]] = UAXIS[I];
            }

            MXV(XFORM.as_slice(), OFF1.as_slice(), XOFF1.as_slice_mut());
            MXV(XFORM.as_slice(), OFF2.as_slice(), XOFF2.as_slice_mut());

            ZZSGLATX(
                XOFF1.as_slice(),
                XOFF2.as_slice(),
                &mut MINLAT,
                MINP.as_slice_mut(),
                &mut MAXLAT,
                MAXP.as_slice_mut(),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"INCNSG", ctx)?;
                return Ok(());
            }

            //
            // COLAT is the colatitude of the point of maximum latitude.
            //
            COLAT = (HALFPI(ctx) - MAXLAT);

            if (COLAT < LOCANG) {
                //
                // MAXP is inside the cone. There should be an intersection
                // on the segment between XOFF1 and MAXP and another
                // between MAXP and XOFF2.
                //
                ZZCXBRUT(
                    save.ORIGIN.as_slice(),
                    save.Z.as_slice(),
                    LOCANG,
                    XOFF1.as_slice(),
                    MAXP.as_slice(),
                    VTEMP.as_slice_mut(),
                    &mut ISBRCK,
                    ctx,
                )?;

                if ISBRCK {
                    //
                    // Convert VTEMP to the original frame, then translate
                    // it so that it's represented as an offset from the
                    // origin.
                    //
                    MTXV(XFORM.as_slice(), VTEMP.as_slice(), VTEMP2.as_slice_mut());
                    VADD(VTEMP2.as_slice(), APEX.as_slice(), XPT1.as_slice_mut());

                    *NXPTS = 1;
                }

                ZZCXBRUT(
                    save.ORIGIN.as_slice(),
                    save.Z.as_slice(),
                    LOCANG,
                    MAXP.as_slice(),
                    XOFF2.as_slice(),
                    VTEMP.as_slice_mut(),
                    &mut ISBRCK,
                    ctx,
                )?;

                if ISBRCK {
                    //
                    // Convert VTEMP to the original frame, then translate
                    // it so that it's represented as an offset from the
                    // origin.
                    //
                    MTXV(XFORM.as_slice(), VTEMP.as_slice(), VTEMP2.as_slice_mut());
                    VADD(VTEMP2.as_slice(), APEX.as_slice(), XPT2.as_slice_mut());

                    if (*NXPTS == 1) {
                        //
                        // Both roots are valid.
                        //
                        *NXPTS = 2;
                    } else {
                        //
                        // The second root is the only valid root. Move it
                        // into XPT1.
                        //
                        VEQU(XPT2.as_slice(), XPT1.as_slice_mut());

                        *NXPTS = 1;
                    }
                }
            } else if (COLAT == LOCANG) {
                //
                // The root corresponds to a point of tangency of
                // the segment and cone. This occurs at the point
                // having maximum latitude: MAXP.
                //
                VEQU(MAXP.as_slice(), XPT1.as_slice_mut());

                *NXPTS = 1;

                //
                // Note that if COLAT > LOCANG, there are no roots.
                //
            }

            CHKOUT(b"INCNSG", ctx)?;
        }
        //
        // This is the end of portion of the "brute force" branch in
        // which both endpoints are outside the cone.
        //
    }

    //
    // NXPTS  has been set.
    //
    // If NXPTS is 1, then XPT1 is set.
    //
    // If NXPTS is 2, then both XPT1 and XPT2 are set.
    //

    Ok(())
}
