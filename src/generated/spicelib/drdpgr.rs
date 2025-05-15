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

/// Derivative of rectangular w.r.t. planetographic
///
/// Compute the Jacobian matrix of the transformation from
/// planetographic to rectangular coordinates.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  BODY       I   Name of body with which coordinates are associated.
///  LON        I   Planetographic longitude of a point (radians).
///  LAT        I   Planetographic latitude of a point (radians).
///  ALT        I   Altitude of a point above reference spheroid.
///  RE         I   Equatorial radius of the reference spheroid.
///  F          I   Flattening coefficient.
///  JACOBI     O   Matrix of partial derivatives.
/// ```
///
/// # Detailed Input
///
/// ```text
///  BODY     is the name of the body with which the planetographic
///           coordinate system is associated.
///
///           BODY is used by this routine to look up from the
///           kernel pool the prime meridian rate coefficient giving
///           the body's spin sense. See the $Files and $Particulars
///           header sections below for details.
///
///  LON      is the planetographic longitude of the input point. This
///           is the angle between the prime meridian and the meridian
///           containing the input point. For bodies having prograde
///           (aka direct) rotation, the direction of increasing
///           longitude is positive west: from the +X axis of the
///           rectangular coordinate system toward the -Y axis. For
///           bodies having retrograde rotation, the direction of
///           increasing longitude is positive east: from the +X axis
///           toward the +Y axis.
///
///           The earth, moon, and sun are exceptions:
///           planetographic longitude is measured positive east for
///           these bodies.
///
///           The default interpretation of longitude by this
///           and the other planetographic coordinate conversion
///           routines can be overridden; see the discussion in
///           $Particulars below for details.
///
///           Longitude is measured in radians. On input, the range
///           of longitude is unrestricted.
///
///  LAT      is the planetographic latitude of the input point. For a
///           point P on the reference spheroid, this is the angle
///           between the XY plane and the outward normal vector at
///           P. For a point P not on the reference spheroid, the
///           planetographic latitude is that of the closest point
///           to P on the spheroid.
///
///           Latitude is measured in radians. On input, the
///           range of latitude is unrestricted.
///
///  ALT      is the altitude of point above the reference spheroid.
///           Units of ALT must match those of RE.
///
///  RE       is the equatorial radius of a reference spheroid. This
///           spheroid is a volume of revolution: its horizontal
///           cross sections are circular. The shape of the
///           spheroid is defined by an equatorial radius RE and
///           a polar radius RP. Units of RE must match those of
///           ALT.
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
///  JACOBI   is the matrix of partial derivatives of the conversion
///           from planetographic to rectangular coordinates. It
///           has the form
///
///              .-                              -.
///              |  DX/DLON   DX/DLAT   DX/DALT   |
///              |  DY/DLON   DY/DLAT   DY/DALT   |
///              |  DZ/DLON   DZ/DLAT   DZ/DALT   |
///              `-                              -'
///
///           evaluated at the input values of LON, LAT and ALT.
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
///      is present in the kernel pool but has a value other
///      than one of
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
///  See the PCK Required Reading for details concerning the prime
///  meridian kernel variable.
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
///  It is often convenient to describe the motion of an object in the
///  planetographic coordinate system. However, when performing
///  vector computations it's hard to beat rectangular coordinates.
///
///  To transform states given with respect to planetographic
///  coordinates to states with respect to rectangular coordinates,
///  one makes use of the Jacobian of the transformation between the
///  two systems.
///
///  Given a state in planetographic coordinates
///
///     ( lon, lat, alt, dlon, dlat, dalt )
///
///  the velocity in rectangular coordinates is given by the matrix
///  equation:
///
///                 t          |                                  t
///     (dx, dy, dz)   = JACOBI|              * (dlon, dlat, dalt)
///                            |(lon,lat,alt)
///
///
///  This routine computes the matrix
///
///           |
///     JACOBI|
///           |(lon,lat,alt)
///
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
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///
///  1) Find the planetographic state of the earth as seen from
///     Mars in the J2000 reference frame at January 1, 2005 TDB.
///     Map this state back to rectangular coordinates as a check.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: drdpgr_ex1.tm
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
///           File name                     Contents
///           ---------                     --------
///           de421.bsp                     Planetary ephemeris
///           pck00008.tpc                  Planet orientation and
///                                         radii
///           naif0009.tls                  Leapseconds
///
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de421.bsp',
///                               'pck00008.tpc',
///                               'naif0009.tls'  )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM DRDPGR_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      RPD
///     C
///     C     Local variables
///     C
///           DOUBLE PRECISION      ALT
///           DOUBLE PRECISION      DRECTN ( 3 )
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      F
///           DOUBLE PRECISION      JACOBI ( 3, 3 )
///           DOUBLE PRECISION      LAT
///           DOUBLE PRECISION      LON
///           DOUBLE PRECISION      LT
///           DOUBLE PRECISION      PGRVEL ( 3 )
///           DOUBLE PRECISION      RADII  ( 3 )
///           DOUBLE PRECISION      RE
///           DOUBLE PRECISION      RECTAN ( 3 )
///           DOUBLE PRECISION      RP
///           DOUBLE PRECISION      STATE  ( 6 )
///
///           INTEGER               N
///
///     C
///     C     Load SPK, PCK, and LSK kernels, use a meta kernel for
///     C     convenience.
///     C
///           CALL FURNSH ( 'drdpgr_ex1.tm' )
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
///     C     Look up the geometric state of earth as seen from Mars at
///     C     January 1, 2005 TDB, relative to the J2000 reference
///     C     frame.
///     C
///           CALL STR2ET ( 'January 1, 2005 TDB', ET )
///
///           CALL SPKEZR ( 'Earth', ET,    'J2000', 'LT+S',
///          .              'Mars',  STATE, LT               )
///
///     C
///     C     Convert position to planetographic coordinates.
///     C
///           CALL RECPGR ( 'MARS', STATE, RE, F, LON, LAT, ALT )
///
///     C
///     C     Convert velocity to planetographic coordinates.
///     C
///
///           CALL DPGRDR ( 'MARS', STATE(1), STATE(2), STATE(3),
///          .               RE,    F,        JACOBI             )
///
///           CALL MXV ( JACOBI, STATE(4), PGRVEL )
///
///     C
///     C     As a check, convert the planetographic state back to
///     C     rectangular coordinates.
///     C
///           CALL PGRREC ( 'MARS', LON, LAT, ALT, RE, F, RECTAN )
///
///           CALL DRDPGR ( 'MARS', LON, LAT, ALT, RE, F, JACOBI )
///
///           CALL MXV ( JACOBI, PGRVEL, DRECTN )
///
///
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Rectangular coordinates:'
///           WRITE(*,*) ' '
///           WRITE(*,*) '  X (km)                 = ', STATE(1)
///           WRITE(*,*) '  Y (km)                 = ', STATE(2)
///           WRITE(*,*) '  Z (km)                 = ', STATE(3)
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Rectangular velocity:'
///           WRITE(*,*) ' '
///           WRITE(*,*) '  dX/dt (km/s)           = ', STATE(4)
///           WRITE(*,*) '  dY/dt (km/s)           = ', STATE(5)
///           WRITE(*,*) '  dZ/dt (km/s)           = ', STATE(6)
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Ellipsoid shape parameters: '
///           WRITE(*,*) ' '
///           WRITE(*,*) '  Equatorial radius (km) = ', RE
///           WRITE(*,*) '  Polar radius      (km) = ', RP
///           WRITE(*,*) '  Flattening coefficient = ', F
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Planetographic coordinates:'
///           WRITE(*,*) ' '
///           WRITE(*,*) '  Longitude (deg)        = ', LON / RPD()
///           WRITE(*,*) '  Latitude  (deg)        = ', LAT / RPD()
///           WRITE(*,*) '  Altitude  (km)         = ', ALT
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Planetographic velocity:'
///           WRITE(*,*) ' '
///           WRITE(*,*) '  d Longitude/dt (deg/s) = ', PGRVEL(1)/RPD()
///           WRITE(*,*) '  d Latitude/dt  (deg/s) = ', PGRVEL(2)/RPD()
///           WRITE(*,*) '  d Altitude/dt  (km/s)  = ', PGRVEL(3)
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Rectangular coordinates from inverse ' //
///          .           'mapping:'
///           WRITE(*,*) ' '
///           WRITE(*,*) '  X (km)                 = ', RECTAN(1)
///           WRITE(*,*) '  Y (km)                 = ', RECTAN(2)
///           WRITE(*,*) '  Z (km)                 = ', RECTAN(3)
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Rectangular velocity from inverse mapping:'
///           WRITE(*,*) ' '
///           WRITE(*,*) '  dX/dt (km/s)           = ', DRECTN(1)
///           WRITE(*,*) '  dY/dt (km/s)           = ', DRECTN(2)
///           WRITE(*,*) '  dZ/dt (km/s)           = ', DRECTN(3)
///           WRITE(*,*) ' '
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Rectangular coordinates:
///
///        X (km)                 =    146039733.67043769
///        Y (km)                 =    278546605.40670651
///        Z (km)                 =    119750317.58721757
///
///      Rectangular velocity:
///
///        dX/dt (km/s)           =   -47.043272004450600
///        dY/dt (km/s)           =    9.0732615496727291
///        dZ/dt (km/s)           =    4.7579169009979010
///
///      Ellipsoid shape parameters:
///
///        Equatorial radius (km) =    3396.1900000000001
///        Polar radius      (km) =    3376.1999999999998
///        Flattening coefficient =    5.8860075555255261E-003
///
///      Planetographic coordinates:
///
///        Longitude (deg)        =    297.66765938292673
///        Latitude  (deg)        =    20.844504443932596
///        Altitude  (km)         =    336531825.52621418
///
///      Planetographic velocity:
///
///        d Longitude/dt (deg/s) =   -8.3577066632519065E-006
///        d Latitude/dt  (deg/s) =    1.5935566850478802E-006
///        d Altitude/dt  (km/s)  =   -11.211600779360412
///
///      Rectangular coordinates from inverse mapping:
///
///        X (km)                 =    146039733.67043760
///        Y (km)                 =    278546605.40670651
///        Z (km)                 =    119750317.58721757
///
///      Rectangular velocity from inverse mapping:
///
///        dX/dt (km/s)           =   -47.043272004450600
///        dY/dt (km/s)           =    9.0732615496727167
///        dZ/dt (km/s)           =    4.7579169009978992
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
/// -    SPICELIB Version 1.1.1, 12-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard..
///         Modified code example to use meta-kernel to load kernels.
///
/// -    SPICELIB Version 1.1.0, 21-SEP-2013 (BVS)
///
///         Updated to save the input body name and ZZBODTRN state counter
///         and to do name-ID conversion only if the counter has changed.
///
///         Updated to call LJUCRS instead of CMPRSS/UCASE.
///
/// -    SPICELIB Version 1.0.0, 26-DEC-2004 (NJB) (WLT)
/// ```
pub fn drdpgr(
    ctx: &mut SpiceContext,
    body: &str,
    lon: f64,
    lat: f64,
    alt: f64,
    re: f64,
    f: f64,
    jacobi: &mut [[f64; 3]; 3],
) -> crate::Result<()> {
    DRDPGR(
        body.as_bytes(),
        lon,
        lat,
        alt,
        re,
        f,
        jacobi.as_flattened_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DRDPGR ( Derivative of rectangular w.r.t. planetographic )
pub fn DRDPGR(
    BODY: &[u8],
    LON: f64,
    LAT: f64,
    ALT: f64,
    RE: f64,
    F: f64,
    JACOBI: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut JACOBI = DummyArrayMut2D::new(JACOBI, 1..=3, 1..=3);
    let mut KVALUE = [b' '; LNSIZE as usize];
    let mut PMKVAR = [b' '; KVNMLN as usize];
    let mut PGRLON = [b' '; SENSLN as usize];
    let mut GEOLON: f64 = 0.0;
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

    CHKIN(b"DRDPGR", ctx)?;

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
        CHKOUT(b"DRDPGR", ctx)?;
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
        CHKOUT(b"DRDPGR", ctx)?;
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
        CHKOUT(b"DRDPGR", ctx)?;
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
            CHKOUT(b"DRDPGR", ctx)?;
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
            CHKOUT(b"DRDPGR", ctx)?;
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
    // Adjust the longitude according to the sense of the body's
    // spin, or according to the override value if one is provided.
    // We want positive east longitude.
    //
    GEOLON = ((SENSE as f64) * LON);

    //
    // Now that we have geodetic longitude in hand, use the
    // geodetic equivalent of the input coordinates to find the
    // Jacobian matrix of rectangular coordinates with respect
    // to geodetic coordinates.
    //
    DRDGEO(GEOLON, LAT, ALT, RE, F, JACOBI.as_slice_mut(), ctx)?;

    //
    // The matrix JACOBI is
    //
    //    .-                              -.
    //    |  DX/DGEOLON  DX/DLAT  DX/DALT  |
    //    |  DY/DGEOLON  DY/DLAT  DY/DALT  |
    //    |  DZ/DGEOLON  DZ/DLAT  DZ/DALT  |
    //    `-                              -'
    //
    // which, applying the chain rule to D(*)/DGEOLON, is equivalent to
    //
    //    .-                                       -.
    //    |  (1/SENSE) * DX/DLON  DX/DLAT  DX/DALT  |
    //    |  (1/SENSE) * DY/DLON  DY/DLAT  DY/DALT  |
    //    |  (1/SENSE) * DZ/DLON  DZ/DLAT  DZ/DALT  |
    //    `-                                       -'
    //
    // So, multiplying the first column of JACOBI by SENSE gives us the
    // matrix we actually want to compute:  the Jacobian matrix of
    // rectangular coordinates with respect to planetographic
    // coordinates.
    //
    for I in 1..=3 {
        JACOBI[[I, 1]] = ((SENSE as f64) * JACOBI[[I, 1]]);
    }

    CHKOUT(b"DRDPGR", ctx)?;
    Ok(())
}
