//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const CTRSIZ: i32 = 2;
const PMTMP: &[u8] = b"BODY#_PM";
const OVRTMP: &[u8] = b"BODY#_PGR_POSITIVE_LON";
const EARTH: i32 = 399;
const KVNMLN: i32 = 32;
const LNSIZE: i32 = 80;
const MOON: i32 = 301;
const SENSLN: i32 = 4;
const SUN: i32 = 10;
const MAXL: i32 = 36;

struct SaveVars {
    SVCTR1: StackArray<i32, 2>,
    SVBODY: Vec<u8>,
    SVBDID: i32,
    SVFND1: bool,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SVCTR1 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVBODY = vec![b' '; MAXL as usize];
        let mut SVBDID: i32 = 0;
        let mut SVFND1: bool = false;
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            SVCTR1,
            SVBODY,
            SVBDID,
            SVFND1,
            FIRST,
        }
    }
}

/// Rectangular to planetographic
///
/// Convert rectangular coordinates to planetographic coordinates.
///
/// # Required Reading
///
/// * [KERNEL](crate::required_reading::kernel)
/// * [NAIF_IDS](crate::required_reading::naif_ids)
/// * [PCK](crate::required_reading::pck)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  BODY       I   Body with which coordinate system is associated.
///  RECTAN     I   Rectangular coordinates of a point.
///  RE         I   Equatorial radius of the reference spheroid.
///  F          I   Flattening coefficient.
///  LON        O   Planetographic longitude of the point (radians).
///  LAT        O   Planetographic latitude of the point (radians).
///  ALT        O   Altitude of the point above reference spheroid.
/// ```
///
/// # Detailed Input
///
/// ```text
///  BODY     is the name of the body with which the planetographic
///           coordinate system is associated.
///
///           BODY is used by this routine to look up from the kernel
///           pool the prime meridian rate coefficient giving the
///           body's spin sense. See the $Files and $Particulars header
///           sections below for details.
///
///  RECTAN   are the rectangular coordinates of a point. Units are
///           arbitrary, except that the input RE must be expressed in
///           the same units.
///
///  RE       is the equatorial radius of a reference spheroid. This
///           spheroid is a volume of revolution: its horizontal cross
///           sections are circular. The shape of the spheroid is
///           defined by an equatorial radius RE and a polar radius RP.
///           Units of RE must match those of RECTAN.
///
///  F        is the flattening coefficient =
///
///              (RE-RP) / RE
///
///           where RP is the polar radius of the spheroid, and the
///           units of RP match those of RE.
/// ```
///
/// # Detailed Output
///
/// ```text
///  LON      is the planetographic longitude of the input point. This
///           is the angle between the prime meridian and the meridian
///           containing RECTAN. For bodies having prograde (aka
///           direct) rotation, the direction of increasing longitude
///           is positive west: from the +X axis of the rectangular
///           coordinate system toward the -Y axis. For bodies having
///           retrograde rotation, the direction of increasing
///           longitude is positive east: from the +X axis toward the
///           +Y axis.
///
///           The earth, moon, and sun are exceptions: planetographic
///           longitude is measured positive east for these bodies.
///
///           The default interpretation of longitude by this and the
///           other planetographic coordinate conversion routines can
///           be overridden; see the discussion in $Particulars below
///           for details.
///
///           LON is output in radians. The nominal range of LON is
///           given by:
///
///              0  <  LON  <  2*pi
///                 -
///
///           However, round-off error could cause LON to equal 2*pi.
///
///  LAT      is the planetographic latitude of the input point. For a
///           point P on the reference spheroid, this is the angle
///           between the XY plane and the outward normal vector at P.
///           For a point P not on the reference spheroid, the
///           planetographic latitude is that of the closest point to P
///           on the spheroid.
///
///           LAT is output in radians. The range of LAT is given by:
///
///              -pi/2  <  LAT  <  pi/2
///                     -       -
///
///  ALT      is the altitude of point above the reference spheroid.
///
///           The units associated with ALT are those associated with
///           the input RECTAN and RE.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the body name BODY cannot be mapped to a NAIF ID code,
///      and if BODY is not a string representation of an integer,
///      the error SPICE(IDCODENOTFOUND) is signaled.
///
///  2)  If the kernel variable
///
///         BODY<ID code>_PGR_POSITIVE_LON
///
///      is present in the kernel pool but has a value other than one
///      of
///
///          'EAST'
///          'WEST'
///
///      the error SPICE(INVALIDOPTION) is signaled. Case
///      and blanks are ignored when these values are interpreted.
///
///  3)  If polynomial coefficients for the prime meridian of BODY
///      are not available in the kernel pool, and if the kernel
///      variable BODY<ID code>_PGR_POSITIVE_LON is not present in
///      the kernel pool, the error SPICE(MISSINGDATA) is signaled.
///
///  4)  If the equatorial radius is non-positive, the error
///      SPICE(VALUEOUTOFRANGE) is signaled.
///
///  5)  If the flattening coefficient is greater than or equal to one,
///      the error SPICE(VALUEOUTOFRANGE) is signaled.
///
///  6)  For points inside the reference ellipsoid, the nearest point
///      on the ellipsoid to RECTAN may not be unique, so latitude may
///      not be well-defined.
/// ```
///
/// # Files
///
/// ```text
///  This routine expects a kernel variable giving BODY's prime
///  meridian angle as a function of time to be available in the
///  kernel pool. Normally this item is provided by loading a PCK
///  file. The required kernel variable is named
///
///     BODY<body ID>_PM
///
///  where <body ID> represents a string containing the NAIF integer
///  ID code for BODY. For example, if BODY is 'JUPITER', then
///  the name of the kernel variable containing the prime meridian
///  angle coefficients is
///
///     BODY599_PM
///
///  The optional kernel variable
///
///     BODY<body ID>_PGR_POSITIVE_LON
///
///  also is normally defined via loading a text kernel. When this
///  variable is present in the kernel pool, the prime meridian
///  coefficients for BODY are not required by this routine. See the
///  $Particulars section for details.
/// ```
///
/// # Particulars
///
/// ```text
///  Given the body-fixed rectangular coordinates of a point, this
///  routine returns the planetographic coordinates of the point. The
///  body-fixed rectangular frame is that having the X-axis pass
///  through the 0 degree latitude 0 degree longitude direction, the
///  Z-axis pass through the 90 degree latitude direction, and the
///  Y-axis equal to the cross product of the unit Z-axis and X-axis
///  vectors.
///
///  The planetographic definition of latitude is identical to the
///  planetodetic (also called "geodetic" in SPICE documentation)
///  definition. In the planetographic coordinate system, latitude is
///  defined using a reference spheroid. The spheroid is
///  characterized by an equatorial radius and a polar radius. For a
///  point P on the spheroid, latitude is defined as the angle between
///  the X-Y plane and the outward surface normal at P. For a point P
///  off the spheroid, latitude is defined as the latitude of the
///  nearest point to P on the spheroid. Note if P is an interior
///  point, for example, if P is at the center of the spheroid, there
///  may not be a unique nearest point to P.
///
///  In the planetographic coordinate system, longitude is defined
///  using the spin sense of the body. Longitude is positive to the
///  west if the spin is prograde and positive to the east if the spin
///  is retrograde. The spin sense is given by the sign of the first
///  degree term of the time-dependent polynomial for the body's prime
///  meridian Euler angle "W":  the spin is retrograde if this term is
///  negative and prograde otherwise. For the sun, planets, most
///  natural satellites, and selected asteroids, the polynomial
///  expression for W may be found in a SPICE PCK kernel.
///
///  The earth, moon, and sun are exceptions: planetographic longitude
///  is measured positive east for these bodies.
///
///  If you wish to override the default sense of positive longitude
///  for a particular body, you can do so by defining the kernel
///  variable
///
///     BODY<body ID>_PGR_POSITIVE_LON
///
///  where <body ID> represents the NAIF ID code of the body. This
///  variable may be assigned either of the values
///
///     'WEST'
///     'EAST'
///
///  For example, you can have this routine treat the longitude
///  of the earth as increasing to the west using the kernel
///  variable assignment
///
///     BODY399_PGR_POSITIVE_LON = 'WEST'
///
///  Normally such assignments are made by placing them in a text
///  kernel and loading that kernel via FURNSH.
///
///  The definition of this kernel variable controls the behavior of
///  the SPICELIB planetographic routines
///
///     PGRREC
///     RECPGR
///     DPGRDR
///     DRDPGR
///
///  It does not affect the other SPICELIB coordinate conversion
///  routines.
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
///  1) Find the planetographic coordinates of the point having Mars
///     rectangular coordinates:
///
///        X (km) =      0.0
///        Y (km) =  -2620.678914818178
///        Z (km) =   2592.408908856967
///
///     (These input values have been chosen to create "simple" output
///     values.)
///
///     Use the PCK kernel below to load the required triaxial
///     ellipsoidal shape model and orientation data for Mars.
///
///        pck00008.tpc
///
///
///     Example code begins here.
///
///
///           PROGRAM RECPGR_EX1
///           IMPLICIT NONE
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      RPD
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
///     C
///     C     Load a PCK file containing a triaxial
///     C     ellipsoidal shape model and orientation
///     C     data for Mars.
///     C
///           CALL FURNSH ( 'pck00008.tpc' )
///
///     C
///     C     Look up the radii for Mars.  Although we
///     C     omit it here, we could first call BADKPV
///     C     to make sure the variable BODY499_RADII
///     C     has three elements and numeric data type.
///     C     If the variable is not present in the kernel
///     C     pool, BODVRD will signal an error.
///     C
///           CALL BODVRD ( 'MARS', 'RADII', 3, N, RADII )
///
///     C
///     C     Compute flattening coefficient.
///     C
///           RE  =  RADII(1)
///           RP  =  RADII(3)
///           F   =  ( RE - RP ) / RE
///
///     C
///     C     Do the conversion.
///     C
///           RECTAN(1) =      0.D0
///           RECTAN(2) =  -2620.678914818178D0
///           RECTAN(3) =   2592.408908856967D0
///
///           CALL RECPGR ( 'MARS', RECTAN, RE, F, LON, LAT, ALT )
///
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Rectangular coordinates:'
///           WRITE (*,*) ' '
///           WRITE (*,*) '  X (km)                 = ', RECTAN(1)
///           WRITE (*,*) '  Y (km)                 = ', RECTAN(2)
///           WRITE (*,*) '  Z (km)                 = ', RECTAN(3)
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Ellipsoid shape parameters: '
///           WRITE (*,*) ' '
///           WRITE (*,*) '  Equatorial radius (km) = ', RE
///           WRITE (*,*) '  Polar radius      (km) = ', RP
///           WRITE (*,*) '  Flattening coefficient = ', F
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Planetographic coordinates:'
///           WRITE (*,*) ' '
///           WRITE (*,*) '  Longitude (deg)        = ', LON / RPD()
///           WRITE (*,*) '  Latitude  (deg)        = ', LAT / RPD()
///           WRITE (*,*) '  Altitude  (km)         = ', ALT
///           WRITE (*,*) ' '
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Rectangular coordinates:
///
///        X (km)                 =    0.0000000000000000
///        Y (km)                 =   -2620.6789148181779
///        Z (km)                 =    2592.4089088569672
///
///      Ellipsoid shape parameters:
///
///        Equatorial radius (km) =    3396.1900000000001
///        Polar radius      (km) =    3376.1999999999998
///        Flattening coefficient =    5.8860075555255261E-003
///
///      Planetographic coordinates:
///
///        Longitude (deg)        =    90.000000000000000
///        Latitude  (deg)        =    45.000000000000014
///        Altitude  (km)         =    300.00000000000057
///
///
///  2) Below is a table showing a variety of rectangular coordinates
///     and the corresponding Mars planetographic coordinates. The
///     values are computed using the reference spheroid having radii
///
///        Equatorial radius:    3396.190
///        Polar radius:         3376.200
///
///     Note:  the values shown above may not be current or suitable
///            for your application.
///
///
///     Corresponding rectangular and planetographic coordinates are
///     listed to three decimal places.
///
///
///     RECTAN(1)  RECTAN(2)  RECTAN(3)       LON       LAT        ALT
///     --------------------------------------------------------------
///      3396.190      0.000      0.000     0.000     0.000      0.000
///     -3396.190      0.000      0.000   180.000     0.000      0.000
///     -3406.190      0.000      0.000   180.000     0.000     10.000
///     -3386.190      0.000      0.000   180.000     0.000    -10.000
///         0.000  -3396.190      0.000    90.000     0.000      0.000
///         0.000   3396.190      0.000   270.000     0.000      0.000
///         0.000      0.000   3376.200     0.000    90.000      0.000
///         0.000      0.000  -3376.200     0.000   -90.000      0.000
///         0.000      0.000      0.000     0.000    90.000  -3376.200
///
///
///  3) Below we show the analogous relationships for the earth,
///     using the reference ellipsoid radii
///
///        Equatorial radius:    6378.140
///        Polar radius:         6356.750
///
///     Note the change in longitudes for points on the +/- Y axis
///     for the earth vs the Mars values.
///
///
///     RECTAN(1)  RECTAN(2)  RECTAN(3)     LON       LAT        ALT
///     --------------------------------------------------------------
///      6378.140      0.000      0.000     0.000     0.000      0.000
///     -6378.140      0.000      0.000   180.000     0.000      0.000
///     -6388.140      0.000      0.000   180.000     0.000     10.000
///     -6368.140      0.000      0.000   180.000     0.000    -10.000
///         0.000  -6378.140      0.000   270.000     0.000      0.000
///         0.000   6378.140      0.000    90.000     0.000      0.000
///         0.000      0.000   6356.750     0.000    90.000      0.000
///         0.000      0.000  -6356.750     0.000   -90.000      0.000
///         0.000      0.000      0.000     0.000    90.000  -6356.750
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
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.1, 06-JUL-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.1.0, 21-SEP-2013 (BVS)
///
///         Updated to save the input body name and ZZBODTRN state
///         counter and to do name-ID conversion only if the counter
///         has changed.
///
///         Updated to call LJUCRS instead of CMPRSS/UCASE.
///
/// -    SPICELIB Version 1.0.1, 23-JAN-2008 (EDW)
///
///         Corrected typo in LAT range description, from:
///
///                    -pi/2  <  LAT  <  pi
///                           -       -
///
///         to:
///
///                    -pi/2  <  LAT  <  pi/2
///                           -       -
///
/// -    SPICELIB Version 1.0.0, 26-DEC-2004 (CHA) (NJB) (HAN) (BVS) (WLT)
/// ```
pub fn recpgr(
    ctx: &mut SpiceContext,
    body: &str,
    rectan: &[f64; 3],
    re: f64,
    f: f64,
    lon: &mut f64,
    lat: &mut f64,
    alt: &mut f64,
) -> crate::Result<()> {
    RECPGR(
        body.as_bytes(),
        rectan,
        re,
        f,
        lon,
        lat,
        alt,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure RECPGR ( Rectangular to planetographic )
pub fn RECPGR(
    BODY: &[u8],
    RECTAN: &[f64],
    RE: f64,
    F: f64,
    LON: &mut f64,
    LAT: &mut f64,
    ALT: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let RECTAN = DummyArray::new(RECTAN, 1..=3);
    let mut KVALUE = [b' '; LNSIZE as usize];
    let mut PMKVAR = [b' '; KVNMLN as usize];
    let mut PGRLON = [b' '; SENSLN as usize];
    let mut BODYID: i32 = 0;
    let mut N: i32 = 0;
    let mut SENSE: i32 = 0;
    let mut FOUND: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Saved body name length.
    //

    //
    // Local variables
    //

    //
    // Saved name/ID item declarations.
    //

    //
    // Saved name/ID items.
    //

    //
    // Initial values.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"RECPGR", ctx)?;

    //
    // Initialization.
    //
    if save.FIRST {
        //
        // Initialize counter.
        //
        ZZCTRUIN(save.SVCTR1.as_slice_mut(), ctx);

        save.FIRST = false;
    }

    //
    // Convert the body name to an ID code.
    //
    ZZBODS2C(
        save.SVCTR1.as_slice_mut(),
        &mut save.SVBODY,
        &mut save.SVBDID,
        &mut save.SVFND1,
        BODY,
        &mut BODYID,
        &mut FOUND,
        ctx,
    )?;

    if !FOUND {
        SETMSG(b"The value of the input argument BODY is #, this is not a recognized name of an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE Toolkit. ", ctx);
        ERRCH(b"#", BODY, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(b"RECPGR", ctx)?;
        return Ok(());
    }

    //
    // The equatorial radius must be positive. If not, signal an error
    // and check out.
    //
    if (RE <= 0.0) {
        SETMSG(b"Equatorial radius was #.", ctx);
        ERRDP(b"#", RE, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"RECPGR", ctx)?;
        return Ok(());
    }

    //
    // If the flattening coefficient is greater than 1, the polar radius
    // is negative. If F is equal to 1, the polar radius is zero. Either
    // case is a problem, so signal an error and check out.
    //
    if (F >= 1.0) {
        SETMSG(b"Flattening coefficient was #.", ctx);
        ERRDP(b"#", F, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"RECPGR", ctx)?;
        return Ok(());
    }

    //
    // Look up the longitude sense override variable from the
    // kernel pool.
    //
    REPMI(OVRTMP, b"#", BODYID, &mut PMKVAR, ctx);
    GCPOOL(
        &PMKVAR,
        1,
        1,
        &mut N,
        CharArrayMut::from_mut(&mut KVALUE),
        &mut FOUND,
        ctx,
    )?;

    if FOUND {
        //
        // Make sure we recognize the value of PGRLON.
        //
        LJUCRS(0, &KVALUE, &mut PGRLON, ctx);

        if fstr::eq(&PGRLON, b"EAST") {
            SENSE = 1;
        } else if fstr::eq(&PGRLON, b"WEST") {
            SENSE = -1;
        } else {
            SETMSG(
                b"Kernel variable # may have the values EAST or WEST.  Actual value was #.",
                ctx,
            );
            ERRCH(b"#", &PMKVAR, ctx);
            ERRCH(b"#", &KVALUE, ctx);
            SIGERR(b"SPICE(INVALIDOPTION)", ctx)?;
            CHKOUT(b"RECPGR", ctx)?;
            return Ok(());
        }
    } else {
        //
        // Look up the spin sense of the body's prime meridian.
        //
        SENSE = PLNSNS(BODYID, ctx)?;

        //
        // If the required prime meridian rate was not available,
        // PLNSNS returns the code 0.  Here we consider this situation
        // to be an error.
        //
        if (SENSE == 0) {
            REPMI(PMTMP, b"#", BODYID, &mut PMKVAR, ctx);

            SETMSG(b"Prime meridian rate coefficient defined by kernel variable # is required but not available for body #. ", ctx);
            ERRCH(b"#", &PMKVAR, ctx);
            ERRCH(b"#", BODY, ctx);
            SIGERR(b"SPICE(MISSINGDATA)", ctx)?;
            CHKOUT(b"RECPGR", ctx)?;
            return Ok(());
        }

        //
        // Handle the special cases:  earth, moon, and sun.
        //
        if (((BODYID == EARTH) || (BODYID == MOON)) || (BODYID == SUN)) {
            SENSE = 1;
        }
    }

    //
    // At this point, SENSE is set to +/- 1.
    //
    // Convert the input coordinates first to geodetic coordinates.
    //
    RECGEO(RECTAN.as_slice(), RE, F, LON, LAT, ALT, ctx)?;

    // Adjust the longitude according to the sense of the body's
    // spin, or according to the override value if one is provided.
    //
    *LON = ((SENSE as f64) * *LON);

    //
    // Convert the longitude from the range (-pi, pi] to [0, 2*pi),
    // the latter being the range of planetographic longitude.
    //
    if (*LON < 0.0) {
        *LON = (*LON + TWOPI(ctx));
    }

    //
    // Make sure round-off error doesn't take LON out of range.
    //
    *LON = BRCKTD(*LON, 0.0, TWOPI(ctx));

    CHKOUT(b"RECPGR", ctx)?;
    Ok(())
}
