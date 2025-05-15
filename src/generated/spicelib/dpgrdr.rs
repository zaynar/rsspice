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

/// Derivative of planetographic w.r.t. rectangular
///
/// Compute the Jacobian matrix of the transformation from
/// rectangular to planetographic coordinates.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  BODY       I   Body with which coordinate system is associated.
///  X          I   X-coordinate of point.
///  Y          I   Y-coordinate of point.
///  Z          I   Z-coordinate of point.
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
///  X,
///  Y,
///  Z        are the rectangular coordinates of the point at
///           which the Jacobian of the map from rectangular
///           to planetographic coordinates is desired.
///
///  RE       is the equatorial radius of the reference spheroid.
///
///  F        is the flattening coefficient = (RE-RP) / RE,  where RP
///           is the polar radius of the spheroid. (More importantly
///           RP = RE*(1-F).)
/// ```
///
/// # Detailed Output
///
/// ```text
///  JACOBI   is the matrix of partial derivatives of the conversion
///           from rectangular to planetographic coordinates. It
///           has the form
///
///              .-                               -.
///              |  DLON/DX    DLON/DY   DLON/DZ   |
///              |  DLAT/DX    DLAT/DY   DLAT/DZ   |
///              |  DALT/DX    DALT/DY   DALT/DZ   |
///              `-                               -'
///
///           evaluated at the input values of X, Y, and Z.
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
///  5)  If the flattening coefficient is greater than or equal to
///      one, the error SPICE(VALUEOUTOFRANGE) is signaled.
///
///  6)  If the input point is on the Z-axis (X = 0 and Y = 0), the
///      Jacobian matrix is undefined, an error is signaled by a
///      routine in the call tree of this routine.
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
///  When performing vector calculations with velocities it is usually
///  most convenient to work in rectangular coordinates. However, once
///  the vector manipulations have been performed, it is often
///  desirable to convert the rectangular representations into
///  planetographic coordinates to gain insights about phenomena in
///  this coordinate frame.
///
///  To transform rectangular velocities to derivatives of coordinates
///  in a planetographic system, one uses the Jacobian of the
///  transformation between the two systems.
///
///  Given a state in rectangular coordinates
///
///     ( x, y, z, dx, dy, dz )
///
///  the velocity in planetographic coordinates is given by the matrix
///  equation:
///                       t          |                     t
///     (dlon, dlat, dalt)   = JACOBI|       * (dx, dy, dz)
///                                  |(x,y,z)
///
///  This routine computes the matrix
///
///           |
///     JACOBI|
///           |(x, y, z)
///
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
///        File name: dpgrdr_ex1.tm
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
///           PROGRAM DPGRDR_EX1
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
///           CALL FURNSH ( 'dpgrdr_ex1.tm' )
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
///         Edited the header to comply with NAIF standard.
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
pub fn dpgrdr(
    ctx: &mut SpiceContext,
    body: &str,
    x: f64,
    y: f64,
    z: f64,
    re: f64,
    f: f64,
    jacobi: &mut [[f64; 3]; 3],
) -> crate::Result<()> {
    DPGRDR(
        body.as_bytes(),
        x,
        y,
        z,
        re,
        f,
        jacobi.as_flattened_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DPGRDR ( Derivative of planetographic w.r.t. rectangular )
pub fn DPGRDR(
    BODY: &[u8],
    X: f64,
    Y: f64,
    Z: f64,
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
    } else {
        CHKIN(b"DPGRDR", ctx)?;
    }

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
        CHKOUT(b"DPGRDR", ctx)?;
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
        CHKOUT(b"DPGRDR", ctx)?;
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
        CHKOUT(b"DPGRDR", ctx)?;
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
            CHKOUT(b"DPGRDR", ctx)?;
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
            CHKOUT(b"DPGRDR", ctx)?;
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
    // To obtain the Jacobian matrix we want, first find the Jacobian
    // matrix of rectangular coordinates with respect to geodetic
    // coordinates.
    //
    DGEODR(X, Y, Z, RE, F, JACOBI.as_slice_mut(), ctx)?;

    //
    // Letting GLON represent geodetic longitude, the matrix JACOBI is
    //
    //    .-                             -.
    //    |  DGLON/DX  DGLON/DY  DGLON/DZ |
    //    |  DLAT/DX   DLAT/DY   DLAT/DZ  |
    //    |  DALT/DX   DALT/DY   DALT/DZ  |
    //    `-                             -'
    //
    // evaluated at the input values of X, Y, and Z.
    //
    // Since planetographic longitude LON satisfies
    //
    //    LON = SENSE * GLON
    //
    // applying the chain rule to D(*)/DGLON, the above is equivalent to
    //
    //    .-                                                         -.
    //    |  (1/SENSE)*DLON/DX  (1/SENSE)*DLON/DY  (1/SENSE)*DLON/DZ  |
    //    |            DLAT/DX            DLAT/DY            DLAT/DZ  |
    //    |            DALT/DX            DALT/DY            DALT/DZ  |
    //    `-                                                         -'
    //
    // So, multiplying the first row of JACOBI by SENSE gives us the
    // matrix we actually want to compute:  the Jacobian matrix of
    // rectangular coordinates with respect to planetographic
    // coordinates.
    //
    for I in 1..=3 {
        JACOBI[[1, I]] = ((SENSE as f64) * JACOBI[[1, I]]);
    }

    CHKOUT(b"DPGRDR", ctx)?;
    Ok(())
}
