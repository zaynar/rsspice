//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Geodetic to rectangular coordinates
///
/// Convert geodetic coordinates to rectangular coordinates.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  LON        I   Geodetic longitude of point (radians).
///  LAT        I   Geodetic latitude  of point (radians).
///  ALT        I   Altitude of point above the reference spheroid.
///  RE         I   Equatorial radius of the reference spheroid.
///  F          I   Flattening coefficient.
///  RECTAN     O   Rectangular coordinates of point.
/// ```
///
/// # Detailed Input
///
/// ```text
///  LON      is the geodetic longitude of the input point. This is
///           the angle between the prime meridian and the meridian
///           containing RECTAN. The direction of increasing
///           longitude is from the +X axis towards the +Y axis.
///
///           Longitude is measured in radians. On input, the
///           range of longitude is unrestricted.
///
///  LAT      is the geodetic latitude of the input point. For a
///           point P on the reference spheroid, this is the angle
///           between the XY plane and the outward normal vector at
///           P. For a point P not on the reference spheroid, the
///           geodetic latitude is that of the closest point to P on
///           the spheroid.
///
///           Latitude is measured in radians. On input, the
///           range of latitude is unrestricted.
///
///  ALT      is the altitude of point above the reference spheroid.
///           ALT must be in the same units as RE.
///
///  RE       is the equatorial radius of a reference spheroid. This
///           spheroid is a volume of revolution: its horizontal
///           cross sections are circular. The shape of the
///           spheroid is defined by an equatorial radius RE and
///           a polar radius RP. RE must be in the same units
///           as ALT.
///
///  F        is the flattening coefficient = (RE-RP) / RE,  where
///           RP is the polar radius of the spheroid.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RECTAN   are the rectangular coordinates of a point.
///
///           The units associated with RECTAN are those associated
///           with the inputs ALT and RE.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the flattening coefficient is greater than or equal to
///      one, the error SPICE(VALUEOUTOFRANGE) is signaled.
///
///  2)  If the equatorial radius is less than or equal to zero,
///      the error SPICE(VALUEOUTOFRANGE) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  Given the geodetic coordinates of a point, and the constants
///  describing the reference spheroid,  this routine returns the
///  bodyfixed rectangular coordinates of the point. The bodyfixed
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
///  This routine can be used to convert body fixed geodetic
///  coordinates (such as the used for United States Geological
///  Survey topographic maps) to bodyfixed rectangular coordinates
///  such as the Satellite Tracking and Data Network of 1973.
///
///  1) Find the rectangular coordinates of the point having Earth
///     geodetic coordinates:
///
///        LON (deg) =  118.0
///        LAT (deg) =   32.0
///        ALT (km)  =    0.0
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
///           PROGRAM GEOREC_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      DPR
///           DOUBLE PRECISION      RPD
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
///     C     Set a geodetic position.
///     C
///           LON = 118.D0 * RPD()
///           LAT =  30.D0 * RPD()
///           ALT =   0.D0
///
///     C
///     C     Do the conversion.
///     C
///           CALL GEOREC( LON, LAT, ALT, RADII(1), F, RECTAN )
///
///           WRITE (*,*) 'Geodetic coordinates in deg and '
///          . //         'km (lon, lat, alt)'
///           WRITE (*,'(A,3F14.6)') ' ', LON * DPR(), LAT * DPR(), ALT
///           WRITE (*,*) 'Rectangular coordinates in km (x, y, z)'
///           WRITE (*,'(A,3F14.6)') ' ', RECTAN
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Geodetic coordinates in deg and km (lon, lat, alt)
///          118.000000     30.000000      0.000000
///      Rectangular coordinates in km (x, y, z)
///        -2595.359123   4881.160589   3170.373523
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
///     listed to three decimal places. Input angles are in degrees.
///
///
///     Example code begins here.
///
///
///           PROGRAM GEOREC_EX2
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      RPD
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
///           DOUBLE PRECISION      ALT    ( NREC )
///           DOUBLE PRECISION      CLARKR
///           DOUBLE PRECISION      CLARKF
///           DOUBLE PRECISION      LAT    ( NREC )
///           DOUBLE PRECISION      LON    ( NREC )
///           DOUBLE PRECISION      RECTAN ( 3    )
///           DOUBLE PRECISION      RLAT
///           DOUBLE PRECISION      RLON
///
///           INTEGER               I
///
///     C
///     C     Define the input geodetic coordinates. Angles in
///     C     degrees.
///     C
///           DATA                 LON /  0.D0,   0.D0,  90.D0,
///          .                            0.D0, 180.D0, -90.D0,
///          .                            0.D0,  45.D0,   0.D0,
///          .                           90.D0,  45.D0         /
///
///           DATA                 LAT / 90.D0,     0.D0,     0.D0,
///          .                           90.D0,     0.D0,     0.D0,
///          .                          -90.D0,     0.D0,    88.707D0,
///          .                           88.707D0, 88.1713D0         /
///
///           DATA                 ALT /   -6356.5838D0,     0.D0,
///          .                   0.D0,         0.D0,         0.D0,
///          .                   0.D0,         0.D0,         0.D0,
///          .               -6355.5725D0, -6355.5725D0, -6355.5612D0 /
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
///           WRITE(*,*) '   LON      LAT       ALT    '
///          . //        ' RECTAN(1)  RECTAN(2)  RECTAN(3)'
///           WRITE(*,*) ' -------  -------  --------- '
///          . //        ' ---------  ---------  ---------'
///
///     C
///     C     Do the conversion.
///     C
///           DO I = 1, NREC
///
///              RLON = LON(I) * RPD()
///              RLAT = LAT(I) * RPD()
///
///              CALL GEOREC( RLON,   RLAT,   ALT(I),
///          .                CLARKR, CLARKF, RECTAN )
///
///              WRITE (*,'(2F9.3,F11.3,3F11.3)')
///          .             LON(I), LAT(I), ALT(I), RECTAN
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
///         LON      LAT       ALT     RECTAN(1)  RECTAN(2)  RECTAN(3)
///       -------  -------  ---------  ---------  ---------  ---------
///         0.000   90.000  -6356.584      0.000      0.000      0.000
///         0.000    0.000      0.000   6378.206      0.000      0.000
///        90.000    0.000      0.000      0.000   6378.206      0.000
///         0.000   90.000      0.000      0.000      0.000   6356.584
///       180.000    0.000      0.000  -6378.206      0.000      0.000
///       -90.000    0.000      0.000      0.000  -6378.206      0.000
///         0.000  -90.000      0.000      0.000      0.000  -6356.584
///        45.000    0.000      0.000   4510.073   4510.073      0.000
///         0.000   88.707  -6355.573      1.000      0.000      1.000
///        90.000   88.707  -6355.573      0.000      1.000      1.000
///        45.000   88.171  -6355.561      1.000      1.000      1.000
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
///  H.A. Neilan        (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 01-OCT-2021 (JDR) (NJB)
///
///         Changed the input argument name LONG to LON for consistency
///         with other routines.
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code examples.
///
/// -    SPICELIB Version 1.0.3, 26-JUL-2016 (BVS)
///
///         Minor headers edits.
///
/// -    SPICELIB Version 1.0.2, 29-JUL-2003 (NJB) (CHA)
///
///         Various header changes were made to improve clarity. Some
///         minor header corrections were made.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (HAN)
/// ```
///
/// # Revisions
///
/// ```text
/// -    Beta Version 3.0.0, 09-JUN-1989  (HAN)
///
///         Error handling added to detect equatorial radius out of
///         range. If the equatorial radius is less than or equal to
///         zero, an error is signaled.
///
/// -    Beta Version 2.0.0, 21-DEC-1988 (HAN)
///
///         Error handling to detect invalid flattening coefficients
///         was added. Because the flattening coefficient is used to
///         compute the polar radius, it must be checked so that the
///         polar radius greater than zero.
/// ```
pub fn georec(
    ctx: &mut SpiceContext,
    lon: f64,
    lat: f64,
    alt: f64,
    re: f64,
    f: f64,
    rectan: &mut [f64; 3],
) -> crate::Result<()> {
    GEOREC(lon, lat, alt, re, f, rectan, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure GEOREC ( Geodetic to rectangular coordinates )
pub fn GEOREC(
    LON: f64,
    LAT: f64,
    ALT: f64,
    RE: f64,
    F: f64,
    RECTAN: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut RECTAN = DummyArrayMut::new(RECTAN, 1..=3);
    let mut HEIGHT: f64 = 0.0;
    let mut RP: f64 = 0.0;
    let mut CLMBDA: f64 = 0.0;
    let mut SLMBDA: f64 = 0.0;
    let mut CPHI: f64 = 0.0;
    let mut SPHI: f64 = 0.0;
    let mut BIG: f64 = 0.0;
    let mut X: f64 = 0.0;
    let mut Y: f64 = 0.0;
    let mut SCALE: f64 = 0.0;
    let mut BASE = StackArray::<f64, 3>::new(1..=3);
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);

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
        CHKIN(b"GEOREC", ctx)?;
    }

    //
    // The equatorial radius must be greater than zero.
    //
    if (RE <= 0.0) {
        SETMSG(b"Equatorial radius was *.", ctx);
        ERRDP(b"*", RE, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"GEOREC", ctx)?;
        return Ok(());
    }

    //
    // If the flattening coefficient is greater than one, the polar
    // radius computed below is negative. If it's equal to one, the
    // polar radius is zero. Either case is a problem, so signal an
    // error and check out.
    //
    if (F >= 1 as f64) {
        SETMSG(b"Flattening coefficient was *.", ctx);
        ERRDP(b"*", F, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"GEOREC", ctx)?;
        return Ok(());
    }

    //
    // Move the altitude to a temporary variable.
    //
    HEIGHT = ALT;

    //
    // Compute the polar radius of the spheroid.
    //
    RP = (RE - (F * RE));

    //
    // Compute a scale factor needed for finding the rectangular
    // coordinates of a point with altitude 0 but the same geodetic
    // latitude and longitude as the input point.
    //
    CPHI = f64::cos(LAT);
    SPHI = f64::sin(LAT);
    CLMBDA = f64::cos(LON);
    SLMBDA = f64::sin(LON);

    BIG = intrinsics::DMAX1(&[f64::abs((RE * CPHI)), f64::abs((RP * SPHI))]);

    X = ((RE * CPHI) / BIG);
    Y = ((RP * SPHI) / BIG);

    SCALE = (1.0 / (BIG * f64::sqrt(((X * X) + (Y * Y)))));

    //
    // Compute the rectangular coordinates of the point with zero
    // altitude.
    //
    BASE[1] = ((((SCALE * RE) * RE) * CLMBDA) * CPHI);
    BASE[2] = ((((SCALE * RE) * RE) * SLMBDA) * CPHI);
    BASE[3] = (((SCALE * RP) * RP) * SPHI);

    //
    // Fetch the normal to the ellipsoid at this point.
    //
    SURFNM(RE, RE, RP, BASE.as_slice(), NORMAL.as_slice_mut(), ctx)?;

    //
    // Move along the normal to the input point.
    //
    VLCOM(
        1.0,
        BASE.as_slice(),
        HEIGHT,
        NORMAL.as_slice(),
        RECTAN.as_slice_mut(),
    );

    CHKOUT(b"GEOREC", ctx)?;
    Ok(())
}
