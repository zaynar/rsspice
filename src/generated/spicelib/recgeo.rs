//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Rectangular to geodetic
///
/// Convert from rectangular coordinates to geodetic coordinates.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  RECTAN     I   Rectangular coordinates of a point.
///  RE         I   Equatorial radius of the reference spheroid.
///  F          I   Flattening coefficient.
///  LON        O   Geodetic longitude of the point (radians).
///  LAT        O   Geodetic latitude  of the point (radians).
///  ALT        O   Altitude of the point above reference spheroid.
/// ```
///
/// # Detailed Input
///
/// ```text
///  RECTAN   are the rectangular coordinates of a point. RECTAN
///           must be in the same units as RE.
///
///  RE       is the equatorial radius of a reference spheroid. This
///           spheroid is a volume of revolution: its horizontal cross
///           sections are circular. The shape of the spheroid is
///           defined by an equatorial radius RE and a polar radius RP.
///           RE must be in the same units as RECTAN.
///
///  F        is the flattening coefficient = (RE-RP) / RE, where RP is
///           the polar radius of the spheroid.
/// ```
///
/// # Detailed Output
///
/// ```text
///  LON      is the geodetic longitude of the input point. This is the
///           angle between the prime meridian and the meridian
///           containing RECTAN. The direction of increasing longitude
///           is from the +X axis towards the +Y axis.
///
///           LON is output in radians. The range of LON is [-pi, pi].
///
///  LAT      is the geodetic latitude of the input point. For a point
///           P on the reference spheroid, this is the angle between
///           the XY plane and the outward normal vector at P. For a
///           point P not on the reference spheroid, the geodetic
///           latitude is that of the closest point to P on the
///           spheroid.
///
///           LAT is output in radians. The range of LAT is
///           [-pi/2, pi/2].
///
///  ALT      is the altitude of point above the reference spheroid.
///
///           The units associated with ALT are those associated with
///           the inputs RECTAN and RE.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the equatorial radius is non-positive, the error
///      SPICE(VALUEOUTOFRANGE) is signaled.
///
///  2)  If the flattening coefficient is greater than or equal to
///      one, the error SPICE(VALUEOUTOFRANGE) is signaled.
///
///  3)  For points inside the reference ellipsoid, the nearest
///      point on the ellipsoid to RECTAN may not be unique, so
///      latitude may not be well-defined.
/// ```
///
/// # Particulars
///
/// ```text
///  Given the body-fixed rectangular coordinates of a point, and the
///  constants describing the reference spheroid,  this routine
///  returns the geodetic coordinates of the point. The body-fixed
///  rectangular frame is that having the x-axis pass through the
///  0 degree latitude 0 degree longitude point. The y-axis passes
///  through the 0 degree latitude 90 degree longitude. The z-axis
///  passes through the 90 degree latitude point. For some bodies
///  this coordinate system may not be a right-handed coordinate
///  system.
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
///  1) Find the geodetic coordinates of the point having Earth
///     rectangular coordinates:
///
///        X (km) =  -2541.748162
///        Y (km) =   4780.333036
///        Z (km) =   3360.428190
///
///     Use the PCK kernel below to load the required triaxial
///     ellipsoidal shape model and orientation data for the Earth.
///
///        pck00010.tpc
///
///
///     Example code begins here.
///
///
///           PROGRAM RECGEO_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      DPR
///
///     C
///     C     Local variables
///     C
///           DOUBLE PRECISION      ALT
///           DOUBLE PRECISION      F
///           DOUBLE PRECISION      LAT
///           DOUBLE PRECISION      LON
///           DOUBLE PRECISION      RADII  ( 3 )
///           DOUBLE PRECISION      RE
///           DOUBLE PRECISION      RECTAN ( 3 )
///           DOUBLE PRECISION      RP
///
///           INTEGER               N
///
///     C
///     C     Load a PCK file containing a triaxial
///     C     ellipsoidal shape model and orientation
///     C     data for the Earth.
///     C
///           CALL FURNSH ( 'pck00010.tpc' )
///
///     C
///     C     Retrieve the triaxial radii of the Earth
///     C
///           CALL BODVRD ( 'EARTH', 'RADII', 3, N, RADII )
///
///     C
///     C     Compute flattening coefficient.
///     C
///           RE  =  RADII(1)
///           RP  =  RADII(3)
///           F   =  ( RE - RP ) / RE
///
///     C
///     C     Set a body-fixed position.
///     C
///           RECTAN(1) =  -2541.748162D0
///           RECTAN(2) =   4780.333036D0
///           RECTAN(3) =   3360.428190D0
///
///     C
///     C     Do the conversion. Output angles in degrees.
///     C
///           CALL RECGEO( RECTAN, RADII(1), F, LON, LAT, ALT )
///
///           WRITE (*,*) 'Rectangular coordinates in km (x, y, z)'
///           WRITE (*,'(A,3F14.6)') ' ', RECTAN
///           WRITE (*,*) 'Geodetic coordinates in deg and '
///          . //         'km (lon, lat, alt)'
///           WRITE (*,'(A,3F14.6)') ' ', LON * DPR(), LAT * DPR(), ALT
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Rectangular coordinates in km (x, y, z)
///        -2541.748162   4780.333036   3360.428190
///      Geodetic coordinates in deg and km (lon, lat, alt)
///          118.000000     31.999957      0.001916
///
///
///  2) Create a table showing a variety of rectangular coordinates
///     and the corresponding Earth geodetic coordinates. The
///     values are computed using the equatorial radius of the Clark
///     66 spheroid and the Clark 66 flattening factor:
///
///        radius: 6378.2064
///        flattening factor: 1./294.9787
///
///     Note: the values shown above may not be current or suitable
///           for your application.
///
///
///     Corresponding rectangular and geodetic coordinates are
///     listed to three decimal places. Output angles are in degrees.
///
///     Example code begins here.
///
///
///           PROGRAM RECGEO_EX2
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      DPR
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               NREC
///           PARAMETER           ( NREC = 11 )
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      ALT
///           DOUBLE PRECISION      CLARKR
///           DOUBLE PRECISION      CLARKF
///           DOUBLE PRECISION      LAT
///           DOUBLE PRECISION      LON
///           DOUBLE PRECISION      RECTAN ( 3, NREC )
///
///           INTEGER               I
///           INTEGER               J
///
///     C
///     C     Define the input rectangular coordinates.
///     C
///           DATA                 RECTAN /
///          .                  0.D0,         0.D0,         0.D0,
///          .               6378.2064D0,     0.D0,         0.D0,
///          .                  0.D0,      6378.2064D0,     0.D0,
///          .                  0.D0,         0.D0,      6378.2064D0,
///          .              -6378.2064D0,     0.D0,         0.D0,
///          .                  0.D0,     -6378.2064D0,     0.D0,
///          .                  0.D0,         0.D0,     -6378.2064D0,
///          .               6378.2064D0,  6378.2064D0,     0.D0,
///          .               6378.2064D0,     0.D0,      6378.2064D0,
///          .                  0.D0,      6378.2064D0,  6378.2064D0,
///          .               6378.2064D0,  6378.2064D0,  6378.2064D0 /
///
///     C
///     C     Using the equatorial radius of the Clark66 spheroid
///     C     (CLARKR = 6378.2064 km) and the Clark 66 flattening
///     C     factor (CLARKF = 1.0 / 294.9787 ) convert from
///     C     body fixed rectangular coordinates.
///     C
///           CLARKR = 6378.2064D0
///           CLARKF = 1.0D0 / 294.9787D0
///
///     C
///     C     Print the banner.
///     C
///           WRITE(*,*) ' RECTAN(1)  RECTAN(2)  RECTAN(3) '
///          . //        '   LON      LAT       ALT'
///           WRITE(*,*) ' ---------  ---------  --------- '
///          . //        ' -------  -------  ---------'
///
///     C
///     C     Do the conversion. Output angles in degrees.
///     C
///           DO I = 1, NREC
///
///              CALL RECGEO( RECTAN(1,I), CLARKR, CLARKF,
///          .                LON,         LAT,    ALT    )
///
///              WRITE (*,'(3F11.3,2F9.3,F11.3)')
///          .             ( RECTAN(J,I), J=1,3 ), LON * DPR(),
///          .             LAT * DPR(), ALT
///
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///       RECTAN(1)  RECTAN(2)  RECTAN(3)    LON      LAT       ALT
///       ---------  ---------  ---------  -------  -------  ---------
///           0.000      0.000      0.000    0.000   90.000  -6356.584
///        6378.206      0.000      0.000    0.000    0.000      0.000
///           0.000   6378.206      0.000   90.000    0.000      0.000
///           0.000      0.000   6378.206    0.000   90.000     21.623
///       -6378.206      0.000      0.000  180.000    0.000      0.000
///           0.000  -6378.206      0.000  -90.000    0.000      0.000
///           0.000      0.000  -6378.206    0.000  -90.000     21.623
///        6378.206   6378.206      0.000   45.000    0.000   2641.940
///        6378.206      0.000   6378.206    0.000   45.137   2652.768
///           0.000   6378.206   6378.206   90.000   45.137   2652.768
///        6378.206   6378.206   6378.206   45.000   35.370   4676.389
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  R. Bate, D. Mueller, and J. White, "Fundamentals of
///       Astrodynamics," Dover Publications Inc., 1971.
/// ```
///
/// # Author and Institution
///
/// ```text
///  C.H. Acton         (JPL)
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 01-OCT-2021 (JDR) (NJB)
///
///         Changed the output argument name LONG to LON for consistency
///         with other routines.
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code examples.
///
/// -    SPICELIB Version 1.1.0, 03-AUG-2016 (BVS) (NJB)
///
///         Re-implemented derivation of longitude to improve
///         accuracy.
///
///         Minor header edits.
///
/// -    SPICELIB Version 1.0.3, 02-JUL-2007 (NJB)
///
///         In $Examples section of header, description of right-hand
///         table was updated to use correct names of columns. Term
///         "bodyfixed" is now hyphenated.
///
/// -    SPICELIB Version 1.0.2, 30-JUL-2003 (NJB) (CHA)
///
///         Various header changes were made to improve clarity. Some
///         minor header corrections were made.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    Beta Version 3.0.1, 9-JUN-1989 (HAN)
///
///         Error handling was added to detect and equatorial radius
///         whose value is less than or equal to zero.
///
/// -    Beta Version 2.0.0, 21-DEC-1988 (HAN)
///
///         Error handling to detect invalid flattening coefficients
///         was added. Because the flattening coefficient is used to
///         compute the length of an axis, it must be checked so that
///         the length is greater than zero.
/// ```
pub fn recgeo(
    ctx: &mut SpiceContext,
    rectan: &[f64; 3],
    re: f64,
    f: f64,
    lon: &mut f64,
    lat: &mut f64,
    alt: &mut f64,
) -> crate::Result<()> {
    RECGEO(rectan, re, f, lon, lat, alt, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure RECGEO ( Rectangular to geodetic )
pub fn RECGEO(
    RECTAN: &[f64],
    RE: f64,
    F: f64,
    LON: &mut f64,
    LAT: &mut f64,
    ALT: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let RECTAN = DummyArray::new(RECTAN, 1..=3);
    let mut A: f64 = 0.0;
    let mut B: f64 = 0.0;
    let mut C: f64 = 0.0;
    let mut BASE = StackArray::<f64, 3>::new(1..=3);
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
    let mut RADIUS: f64 = 0.0;

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
        CHKIN(b"RECGEO", ctx)?;
    }

    //
    // The equatorial radius must be positive. If not, signal an error
    // and check out.
    //
    if (RE <= 0.0) {
        SETMSG(b"Equatorial radius was *.", ctx);
        ERRDP(b"*", RE, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"RECGEO", ctx)?;
        return Ok(());
    }

    //
    // If the flattening coefficient is greater than one, the length
    // of the 'C' axis computed below is negative. If it's equal to one,
    // the length of the axis is zero. Either case is a problem, so
    // signal an error and check out.
    //
    if (F >= 1 as f64) {
        SETMSG(b"Flattening coefficient was *.", ctx);
        ERRDP(b"*", F, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"RECGEO", ctx)?;
        return Ok(());
    }

    //
    // Determine the lengths of the axes of the reference ellipsoid.
    //
    A = RE;
    B = RE;
    C = (RE - (F * RE));

    //
    // Find the point on the reference spheroid closest to the input
    // point. From this closest point determine the surface normal.
    //
    NEARPT(RECTAN.as_slice(), A, B, C, BASE.as_slice_mut(), ALT, ctx)?;
    SURFNM(A, B, C, BASE.as_slice(), NORMAL.as_slice_mut(), ctx)?;
    //
    // Using the surface normal, determine the latitude and longitude
    // of the input point.
    //
    RECLAT(NORMAL.as_slice(), &mut RADIUS, LON, LAT);

    //
    // Compute longitude directly rather than from the normal vector.
    //
    if ((RECTAN[1] == 0.0) && (RECTAN[2] == 0.0)) {
        *LON = 0.0;
    } else {
        *LON = f64::atan2(RECTAN[2], RECTAN[1]);
    }

    CHKOUT(b"RECGEO", ctx)?;
    Ok(())
}
