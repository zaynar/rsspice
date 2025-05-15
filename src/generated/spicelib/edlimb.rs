//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const UBEL: i32 = 9;
const UBPL: i32 = 4;

/// Ellipsoid Limb
///
/// Find the limb of a triaxial ellipsoid, viewed from a specified
/// point.
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
///  A          I   Length of ellipsoid semi-axis lying on the x-axis.
///  B          I   Length of ellipsoid semi-axis lying on the y-axis.
///  C          I   Length of ellipsoid semi-axis lying on the z-axis.
///  VIEWPT     I   Location of viewing point.
///  LIMB       O   Limb of ellipsoid as seen from viewing point.
/// ```
///
/// # Detailed Input
///
/// ```text
///  A,
///  B,
///  C        are the lengths of the semi-axes of a triaxial
///           ellipsoid. The ellipsoid is centered at the
///           origin and oriented so that its axes lie on the
///           x, y and z axes.  A, B, and C are the lengths of
///           the semi-axes that point in the x, y, and z
///           directions respectively.
///
///  VIEWPT   is a point from which the ellipsoid is viewed.
///           VIEWPT must be outside of the ellipsoid.
/// ```
///
/// # Detailed Output
///
/// ```text
///  LIMB     is a SPICE ellipse that represents the limb of
///           the ellipsoid.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the length of any semi-axis of the ellipsoid is
///      non-positive, the error SPICE(INVALIDAXISLENGTH) is signaled.
///      LIMB is not modified.
///
///  2)  If the length of any semi-axis of the ellipsoid is zero after
///      the semi-axis lengths are scaled by the reciprocal of the
///      magnitude of the longest semi-axis and then squared, the error
///      SPICE(DEGENERATECASE) is signaled. LIMB is not modified.
///
///  3)  If the viewing point VIEWPT is inside the ellipse, the error
///      SPICE(INVALIDPOINT) is signaled. LIMB is not modified.
///
///  4)  If the geometry defined by the input ellipsoid and viewing
///      point is so extreme that the limb cannot be found, the error
///      SPICE(DEGENERATECASE) is signaled.
///
///  5)  If the shape of the ellipsoid and the viewing geometry are
///      such that the limb is an excessively flat ellipsoid, the
///      limb may be a degenerate ellipse. You must determine whether
///      this possibility poses a problem for your application.
/// ```
///
/// # Particulars
///
/// ```text
///  The limb of a body, as seen from a viewing point, is the boundary
///  of the portion of the body's surface that is visible from that
///  viewing point. In this definition, we consider a surface point
///  to be `visible' if it can be connected to the viewing point by a
///  line segment that doesn't pass through the body. This is a purely
///  geometrical definition that ignores the matter of which portions
///  of the surface are illuminated, or whether the view is obscured by
///  any additional objects.
///
///  If a body is modeled as a triaxial ellipsoid, the limb is always
///  an ellipse. The limb is determined by its center, a semi-major
///  axis vector, and a semi-minor axis vector.
///
///  We note that the problem of finding the limb of a triaxial
///  ellipsoid is mathematically identical to that of finding its
///  terminator, if one makes the simplifying assumption that the
///  terminator is the limb of the body as seen from the vertex of the
///  umbra. So, this routine can be used to solve this simplified
///  version of the problem of finding the terminator.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for these examples may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Given an ellipsoid and a viewpoint exterior to it, calculate
///     the limb ellipse as seen from that viewpoint.
///
///
///     Example code begins here.
///
///
///           PROGRAM EDLIMB_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local constants.
///     C
///           INTEGER                 UBEL
///           PARAMETER             ( UBEL =   9 )
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION        A
///           DOUBLE PRECISION        B
///           DOUBLE PRECISION        C
///           DOUBLE PRECISION        ECENTR ( 3    )
///           DOUBLE PRECISION        LIMB   ( UBEL )
///           DOUBLE PRECISION        SMAJOR ( 3    )
///           DOUBLE PRECISION        SMINOR ( 3    )
///           DOUBLE PRECISION        VIEWPT ( 3    )
///
///     C
///     C     Define a viewpoint exterior to the ellipsoid.
///     C
///           DATA                    VIEWPT /  2.D0,  0.D0,  0.D0 /
///
///     C
///     C     Define an ellipsoid.
///     C
///           A = SQRT( 2.D0 )
///           B = 2.D0 * SQRT( 2.D0 )
///           C = SQRT( 2.D0 )
///
///     C
///     C     Calculate the limb ellipse as seen by from the
///     C     viewpoint.
///     C
///           CALL EDLIMB ( A, B, C, VIEWPT, LIMB )
///
///     C
///     C     Output the structure components.
///     C
///           CALL EL2CGV ( LIMB, ECENTR, SMAJOR, SMINOR )
///
///           WRITE(*,'(A)') 'Limb ellipse as seen from viewpoint:'
///           WRITE(*,'(A,3F11.6)') '   Semi-minor axis:', SMINOR
///           WRITE(*,'(A,3F11.6)') '   Semi-major axis:', SMAJOR
///           WRITE(*,'(A,3F11.6)') '   Center         :', ECENTR
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Limb ellipse as seen from viewpoint:
///        Semi-minor axis:   0.000000   0.000000  -1.000000
///        Semi-major axis:   0.000000   2.000000  -0.000000
///        Center         :   1.000000   0.000000   0.000000
///
///
///  2) We'd like to find the apparent limb of Jupiter, corrected for
///     light time and stellar aberration, as seen from JUNO
///     spacecraft's position at a given UTC time.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: edlimb_ex2.tm
///
///        This meta-kernel is intended to support operation of SPICE
///        example programs. The kernels shown here should not be
///        assumed to contain adequate or correct versions of data
///        required by SPICE-based user applications.
///
///        In order for an application to use this meta-kernel, the
///        kernels referenced here must be present in the user's
///        current working directory.
///
///        The names and contents of the kernels referenced
///        by this meta-kernel are as follows:
///
///           File name                           Contents
///           ---------                           --------
///           juno_rec_160522_160729_160909.bsp   JUNO s/c ephemeris
///           pck00010.tpc                        Planet orientation
///                                               and radii
///           naif0012.tls                        Leapseconds
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'juno_rec_160522_160729_160909.bsp',
///                               'pck00010.tpc',
///                               'naif0012.tls'  )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM EDLIMB_EX2
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           CHARACTER*(*)           UTCSTR
///           PARAMETER             ( UTCSTR = '2016 Jul 14 19:45:00' )
///
///           INTEGER                 UBEL
///           PARAMETER             ( UBEL =   9 )
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION        CENTER ( 3    )
///           DOUBLE PRECISION        ET
///           DOUBLE PRECISION        JPOS   ( 3    )
///           DOUBLE PRECISION        LIMB   ( UBEL )
///           DOUBLE PRECISION        LT
///           DOUBLE PRECISION        RAD    ( 3    )
///           DOUBLE PRECISION        SMAJOR ( 3    )
///           DOUBLE PRECISION        SMINOR ( 3    )
///           DOUBLE PRECISION        SCPJFC ( 3    )
///           DOUBLE PRECISION        SCPOS  ( 3    )
///           DOUBLE PRECISION        TIPM   ( 3, 3 )
///
///           INTEGER                 N
///
///     C
///     C     Load the required kernels.
///     C
///           CALL FURNSH ( 'edlimb_ex2.tm' )
///
///     C
///     C     Find the viewing point in Jupiter-fixed coordinates. To
///     C     do this, find the apparent position of Jupiter as seen
///     C     from the spacecraft in Jupiter-fixed coordinates and
///     C     negate this vector. In this case we'll use light time
///     C     and stellar aberration corrections to arrive at the
///     C     apparent limb. JPOS is the Jupiter's position as seen
///     C     from the spacecraft.  SCPOS is the spacecraft's position
///     C     relative to Jupiter.
///     C
///           CALL STR2ET ( UTCSTR,    ET )
///           CALL SPKPOS ( 'JUPITER', ET, 'J2000', 'LT+S', 'JUNO',
///          .               JPOS,     LT                         )
///
///           CALL VMINUS ( JPOS, SCPOS )
///
///     C
///     C     Get Jupiter's semi-axis lengths...
///     C
///           CALL BODVRD ( 'JUPITER', 'RADII', 3, N, RAD )
///
///     C
///     C     ...and the transformation from J2000 to Jupiter
///     C     equator and prime meridian coordinates. Note that we
///     C     use the orientation of Jupiter at the time of
///     C     emission of the light that arrived at the
///     C     spacecraft at time ET.
///     C
///           CALL PXFORM ( 'J2000', 'IAU_JUPITER', ET-LT, TIPM )
///
///     C
///     C     Transform the spacecraft's position into Jupiter-
///     C     fixed coordinates.
///     C
///           CALL MXV ( TIPM, SCPOS, SCPJFC )
///
///     C
///     C     Find the apparent limb.  LIMB is a SPICE ellipse
///     C     representing the limb.
///     C
///           CALL EDLIMB ( RAD(1), RAD(2), RAD(3), SCPJFC, LIMB )
///
///     C
///     C     CENTER, SMAJOR, and SMINOR are the limb's center,
///     C     semi-major axis of the limb, and a semi-minor axis
///     C     of the limb.  We obtain these from LIMB using the
///     C     SPICELIB routine EL2CGV ( Ellipse to center and
///     C     generating vectors ).
///     C
///           CALL EL2CGV ( LIMB, CENTER, SMAJOR, SMINOR )
///
///     C
///     C     Output the structure components.
///     C
///           WRITE(*,'(A)') 'Apparent limb of Jupiter as seen '
///          .            // 'from JUNO:'
///           WRITE(*,'(2A)')       '   UTC time       : ', UTCSTR
///           WRITE(*,'(A,3F14.6)') '   Semi-minor axis:', SMINOR
///           WRITE(*,'(A,3F14.6)') '   Semi-major axis:', SMAJOR
///           WRITE(*,'(A,3F14.6)') '   Center         :', CENTER
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Apparent limb of Jupiter as seen from JUNO:
///        UTC time       : 2016 Jul 14 19:45:00
///        Semi-minor axis:  12425.547643  -5135.572410  65656.053303
///        Semi-major axis:  27305.667297  66066.222576      0.000000
///        Center         :    791.732472   -327.228993   -153.408849
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
/// -    SPICELIB Version 1.4.0, 24-AUG-2021 (NJB) (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///         Added complete code example.
///
///         Corrected several header comment typos.
///
/// -    SPICELIB Version 1.3.0, 23-OCT-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in VSCLG call. Updated header to refer to BODVCD instead
///         of BODVAR.
///
/// -    SPICELIB Version 1.2.0, 06-OCT-1993 (NJB)
///
///         Declaration of unused local variable NEAR was removed.
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
/// -    SPICELIB Version 1.1.0, 04-DEC-1990 (NJB)
///
///         Error message and description changed for non-positive
///         axis length error. The former message and description did
///         not match, and the description was incorrect: it described
///         `zero-length', rather than `non-positive' axes as invalid.
/// ```
pub fn edlimb(
    ctx: &mut SpiceContext,
    a: f64,
    b: f64,
    c: f64,
    viewpt: &[f64; 3],
    limb: &mut [f64; 9],
) -> crate::Result<()> {
    EDLIMB(a, b, c, viewpt, limb, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EDLIMB   ( Ellipsoid Limb )
pub fn EDLIMB(
    A: f64,
    B: f64,
    C: f64,
    VIEWPT: &[f64],
    LIMB: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let VIEWPT = DummyArray::new(VIEWPT, 1..=3);
    let mut LIMB = DummyArrayMut::new(LIMB, 1..=UBEL);
    let mut LEVEL: f64 = 0.0;
    let mut LPLANE = StackArray::<f64, 4>::new(1..=UBPL);
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
    let mut SCALE: f64 = 0.0;
    let mut SCLA: f64 = 0.0;
    let mut SCLA2: f64 = 0.0;
    let mut SCLB: f64 = 0.0;
    let mut SCLB2: f64 = 0.0;
    let mut SCLC: f64 = 0.0;
    let mut SCLC2: f64 = 0.0;
    let mut TMPEL = StackArray::<f64, 9>::new(1..=UBEL);
    let mut V = StackArray::<f64, 3>::new(1..=3);
    let mut FOUND: bool = false;

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
    } else {
        CHKIN(b"EDLIMB", ctx)?;
    }

    //
    // The semi-axes must have positive length.
    //
    if (((A <= 0.0) || (B <= 0.0)) || (C <= 0.0)) {
        SETMSG(b"Semi-axis lengths:  A = #, B = #, C = #. ", ctx);
        ERRDP(b"#", A, ctx);
        ERRDP(b"#", B, ctx);
        ERRDP(b"#", C, ctx);
        SIGERR(b"SPICE(INVALIDAXISLENGTH)", ctx)?;
        CHKOUT(b"EDLIMB", ctx)?;
        return Ok(());
    }

    //
    // Scale the semi-axes lengths for better numerical behavior.
    // If squaring any one of the scaled lengths causes it to
    // underflow to zero, we cannot continue the computation. Otherwise,
    // scale the viewing point too.
    //
    SCALE = intrinsics::DMAX1(&[f64::abs(A), f64::abs(B), f64::abs(C)]);

    SCLA = (A / SCALE);
    SCLB = (B / SCALE);
    SCLC = (C / SCALE);

    SCLA2 = f64::powi(SCLA, 2);
    SCLB2 = f64::powi(SCLB, 2);
    SCLC2 = f64::powi(SCLC, 2);

    if (((SCLA2 == 0.0) || (SCLB2 == 0.0)) || (SCLC2 == 0.0)) {
        SETMSG(b"Semi-axis too small:  A = #, B = #, C = #. ", ctx);
        ERRDP(b"#", A, ctx);
        ERRDP(b"#", B, ctx);
        ERRDP(b"#", C, ctx);
        SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
        CHKOUT(b"EDLIMB", ctx)?;
        return Ok(());
    }

    VSCL((1.0 / SCALE), VIEWPT.as_slice(), V.as_slice_mut());

    //
    // The viewing point must be outside of the ellipsoid.  LEVEL is the
    // constant of the level surface that V lies on.  The ellipsoid
    // itself is the level surface corresponding to LEVEL = 1.
    //
    LEVEL = (((f64::powi(V[1], 2) / SCLA2) + (f64::powi(V[2], 2) / SCLB2))
        + (f64::powi(V[3], 2) / SCLC2));

    if (LEVEL < 1.0) {
        SETMSG(b"Viewing point is inside the ellipsoid.", ctx);
        SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
        CHKOUT(b"EDLIMB", ctx)?;
        return Ok(());
    }

    //
    // Find a normal vector for the limb plane.
    //
    // To compute this vector, we use the fact that the surface normal at
    // each limb point is orthogonal to the line segment connecting the
    // viewing point and the limb point.   Let the notation
    //
    //    < a, b >
    //
    // indicate the dot product of the vectors a and b.  If we call the
    // viewing point V and the limb point X, then
    //
    //
    //
    //                        X(1)         X(2)         X(3)
    //    0  =   < V - X,  ( -------- ,   -------- ,   --------  )  >
    //                            2           2             2
    //                        SCLA        SCLB          SCLC
    //
    //
    //                        X(1)         X(2)         X(3)
    //       =   <   V,    ( -------- ,   -------- ,   --------  )  >
    //                            2           2             2
    //                        SCLA        SCLB          SCLC
    //
    //
    //                        X(1)         X(2)         X(3)
    //        - <   X,    ( -------- ,   -------- ,   --------  )  >
    //                            2           2             2
    //                        SCLA        SCLB          SCLC
    //
    //                            2           2             2
    //                        X(1)        X(2)          X(3)
    //       =             --------  +   --------  +  --------
    //                           2            2             2
    //                       SCLA         SCLB          SCLC
    //
    //
    //       =   1
    //
    //
    // This last equation is just the equation of the scaled ellipsoid.
    // We can combine the last two equalities and interchange the
    // positions of X and V to obtain
    //
    //
    //                  V(1)         V(2)         V(3)
    //    <   X,    ( -------- ,   -------- ,   --------  )  >   =   1
    //                      2           2             2
    //                  SCLA        SCLB          SCLC
    //
    //
    // This is the equation of the limb plane.
    //

    //
    // Put together a SPICE plane, LPLANE, that represents the limb
    // plane.
    //
    NORMAL[1] = (V[1] / SCLA2);
    NORMAL[2] = (V[2] / SCLB2);
    NORMAL[3] = (V[3] / SCLC2);

    NVC2PL(NORMAL.as_slice(), 1.0, LPLANE.as_slice_mut(), ctx)?;

    //
    // Find the limb by intersecting the limb plane with the ellipsoid.
    //
    INEDPL(
        SCLA,
        SCLB,
        SCLC,
        LPLANE.as_slice(),
        LIMB.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    //
    // FOUND should be true unless we've encountered numerical problems.
    //
    if !FOUND {
        SETMSG(
            b"Ellipsoid shape and viewing geometry are too extreme; the limb was not found. ",
            ctx,
        );
        SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
        CHKOUT(b"EDLIMB", ctx)?;
        return Ok(());
    }

    //
    // Undo the scaling before returning the limb.
    //
    VSCLG(SCALE, LIMB.as_slice(), UBEL, TMPEL.as_slice_mut());
    MOVED(TMPEL.as_slice(), UBEL, LIMB.as_slice_mut());

    CHKOUT(b"EDLIMB", ctx)?;
    Ok(())
}
