//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const CTRSIZ: i32 = 2;
const MARGIN: f64 = 100.0;
const CHSIZ: i32 = 40;
const NCOSYS: i32 = 6;
const RECTAN: i32 = 1;
const CYL: i32 = 2;
const LATNL: i32 = 3;
const SPHCL: i32 = 4;
const GEODET: i32 = 5;
const PLNTGR: i32 = 6;
const MAXL: i32 = 36;

struct SaveVars {
    COSYS: ActualCharArray,
    SVCTR1: StackArray<i32, 2>,
    SVBODY: Vec<u8>,
    SVBDID: i32,
    SVFND1: bool,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut COSYS = ActualCharArray::new(CHSIZ, 1..=NCOSYS);
        let mut SVCTR1 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVBODY = vec![b' '; MAXL as usize];
        let mut SVBDID: i32 = 0;
        let mut SVFND1: bool = false;
        let mut FIRST: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"RECTANGULAR")].into_iter();
            fstr::assign(COSYS.get_mut(RECTAN), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"CYLINDRICAL")].into_iter();
            fstr::assign(COSYS.get_mut(CYL), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"LATITUDINAL")].into_iter();
            fstr::assign(COSYS.get_mut(LATNL), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"SPHERICAL")].into_iter();
            fstr::assign(COSYS.get_mut(SPHCL), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"GEODETIC")].into_iter();
            fstr::assign(COSYS.get_mut(GEODET), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"PLANETOGRAPHIC")].into_iter();
            fstr::assign(COSYS.get_mut(PLNTGR), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        FIRST = true;

        Self {
            COSYS,
            SVCTR1,
            SVBODY,
            SVBDID,
            SVFND1,
            FIRST,
        }
    }
}

/// Transform state between coordinate systems
///
/// Transform a state between coordinate systems.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  -------------------------------------------------
///  ISTATE     I   Input state.
///  ICOSYS     I   Current (input) coordinate system.
///  OCOSYS     I   Desired (output) coordinate system.
///  BODY       I   Name or NAIF ID of body with which
///                 coordinates are associated (if applicable).
///  OSTATE     O   Converted output state.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ISTATE   is a state vector in the input ICOSYS coordinate
///           system representing position and velocity.
///
///           All angular measurements must be in radians.
///
///           Note: body radii values taken from the kernel
///           pool are used when converting to or from geodetic or
///           planetographic coordinates. It is the user's
///           responsibility to verify the distance inputs are in
///           the same units as the radii in the kernel pool,
///           typically kilometers.
///
///  ICOSYS   is the name of the coordinate system that the input
///           state vector ISTATE is currently in.
///
///           ICOSYS may be any of the following:
///
///              'RECTANGULAR'
///              'CYLINDRICAL'
///              'LATITUDINAL'
///              'SPHERICAL'
///              'GEODETIC'
///              'PLANETOGRAPHIC'
///
///           Leading spaces, trailing spaces, and letter case
///           are ignored. For example, ' cyLindRical  ' would be
///           accepted.
///
///  OCOSYS   is the name of the coordinate system that the state
///           should be converted to.
///
///           Please see the description of ICOSYS for details.
///
///  BODY     is the name or NAIF ID of the body associated with the
///           planetographic or geodetic coordinate system.
///
///           If neither of the coordinate system choices are
///           geodetic or planetographic, BODY is ignored. It may
///           be a blank string.
///
///           Examples of accepted body names or IDs are:
///
///              'Earth'
///              '399'
///
///           Leading spaces, trailing spaces, and letter case are
///           ignored.
/// ```
///
/// # Detailed Output
///
/// ```text
///  OSTATE   is the state vector that has been converted to the
///           output coordinate system OCOSYS.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If either the input or output coordinate system is not
///      recognized, the error SPICE(COORDSYSNOTREC) is signaled.
///
///  2)  If the input body name cannot be converted to a NAIF ID
///      (applies to geodetic and planetographic coordinate
///      systems), the error SPICE(IDCODENOTFOUND) is signaled.
///
///  3)  If the input state ISTATE is not valid, meaning the position
///      but not the velocity is along the z-axis, the error
///      SPICE(INVALIDSTATE) is signaled.
///
///      Note: If both the input position and velocity are along
///      the z-axis and the output coordinate system is not
///      rectangular, the velocity can still be calculated even
///      though the Jacobian is undefined. This case will not
///      signal an error. An example of the input position and
///      velocity along the z-axis is below.
///
///                    Term    Value
///                    -----   ------
///                      x       0
///                      y       0
///                      z       z
///                    dx/dt     0
///                    dy/dt     0
///                    dz/dt   dz_dt
///
///  4)  If either the input or output coordinate system is
///      geodetic or planetographic and at least one of the body's
///      radii is less than or equal to zero, the error
///      SPICE(INVALIDRADIUS) is signaled.
///
///  5)  If either the input or output coordinate system is
///      geodetic or planetographic and the difference of the
///      equatorial and polar radii divided by the equatorial radius
///      would produce numeric overflow, the error
///      SPICE(INVALIDRADIUS) is signaled.
///
///  6)  If the product of the Jacobian and velocity components
///      may lead to numeric overflow, the error
///      SPICE(NUMERICOVERFLOW) is signaled.
///
///  7)  If radii for BODY are not found in the kernel pool, an error
///      is signaled by a routine in the call tree of this routine.
///
///  8)  If the size of the BODY radii kernel variable is not three,
///      an error is signaled by a routine in the call tree of this
///      routine.
///
///  9)  If any of the three BODY radii is less-than or equal to zero,
///      an error is signaled by a routine in the call tree of this
///      routine.
///
///  10) If body's equatorial radii are not equal and either the
///      input or output coordinate system is geodetic or
///      planetographic, the error SPICE(NOTSUPPORTED) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  SPK, PCK, CK, and FK kernels may be required.
///
///  If the input or output coordinate systems are either geodetic or
///  planetographic, a PCK providing the radii of the body
///  name BODY must be loaded via FURNSH.
///
///  Kernel data are normally loaded once per program run, NOT every
///  time this routine is called.
/// ```
///
/// # Particulars
///
/// ```text
///  Input Order
///  -------------------------------------------
///
///  The input and output states will be structured by the
///  following descriptions.
///
///  For rectangular coordinates, the state vector is the following
///  in which X, Y, and Z are the rectangular position components and
///  DX, DY, and DZ are the time derivatives of each position
///  component.
///
///          ISTATE = ( X, Y, Z, DX, DY, DZ )
///
///  For cylindrical coordinates, the state vector is the following
///  in which R is the radius, LONG is the longitudes, Z is the
///  height, and DR, DLONG, and DZ are the time derivatives of each
///  position component.
///
///          ISTATE = ( R, LONG, Z, DR, DLONG, DZ )
///
///  For latitudinal coordinates, the state vector is the following
///  in which R is the radius, LONG is the longitude, LAT is the
///  latitude, and DR, DLONG, and DLAT are the time derivatives of
///  each position component.
///
///          ISTATE = ( R, LONG, LAT, DR, DLONG, DLAT )
///
///  For spherical coordinates, the state vector is the following in
///  which R is the radius, COLAT is the colatitude, LONG is the
///  longitude, and DR, DCOLAT, and DLONG are the time derivatives of
///  each position component.
///
///          ISTATE = ( R, COLAT, LONG, DR, DCOLAT, DLONG )
///
///  For geodetic coordinates, the state vector is the following in
///  which LONG is the longitude, LAT is the latitude, ALT is the
///  altitude, and DLONG, DLAT, and DALT are the time derivatives of
///  each position component.
///
///          ISTATE = ( LONG, LAT, ALT, DLONG, DLAT, DALT )
///
///  For planetographic coordinates, the state vector is the
///  following in which LONG is the longitude, LAT is the latitude,
///  ALT is the altitude, and DLONG, DLAT, and DALT are the time
///  derivatives of each position component.
///
///          ISTATE = ( LONG, LAT, ALT, DLONG, DLAT, DALT )
///
///
///  Input Boundaries
///  -------------------------------------------
///
///  There are intervals the input angles must fall within if
///  the input coordinate system is not rectangular. These
///  intervals are provided below.
///
///     Input variable    Input meaning   Input interval [rad]
///     --------------    -------------   ------------------------
///         LONG           Longitude        0     <= LONG  <  2*pi
///         LAT            Latitude        -pi/2  <= LAT   <= pi/2
///         COLAT          Colatitude       0     <= COLAT <= pi
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
///  1) Find the apparent state of Phoebe as seen by CASSINI in the
///     J2000 frame at 2004 Jun 11 19:32:00. Transform the state
///     from rectangular to latitudinal coordinates. For verification,
///     transform the state back from latitudinal to rectangular
///     coordinates.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: xfmsta_ex1.tm
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
///           File name                        Contents
///           ---------                        --------
///           pck00010.tpc                     Planet orientation and
///                                            radii
///           naif0012.tls                     Leapseconds
///           041014R_SCPSE_01066_04199.bsp    CASSINI, planetary and
///                                            Saturn Satellite
///                                            ephemeris
///
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'naif0012.tls',
///                               '041014R_SCPSE_01066_04199.bsp',
///                               'pck00010.tpc'                 )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM XFMSTA_EX1
///           IMPLICIT NONE
///     C
///     C     Local parameters
///     C
///     C     METAKR is the meta-kernel's filename.
///     C
///           CHARACTER*(*)         METAKR
///           PARAMETER           ( METAKR = 'xfmsta_ex1.tm' )
///
///           CHARACTER*(*)         FORM
///           PARAMETER           ( FORM = '(F16.6, F16.6, F16.6)' )
///
///     C
///     C     Local variables
///     C
///     C     STAREC is the state of Phoebe with respect to CASSINI in
///     C     rectangular coordinates. STALAT is the state rotated into
///     C     latitudinal coordinates. STREC2 is the state transformed
///     C     back into rectangular coordinates from latitudinal.
///     C
///           DOUBLE PRECISION      STAREC (6)
///           DOUBLE PRECISION      STALAT (6)
///           DOUBLE PRECISION      STREC2 (6)
///
///     C
///     C     ET is the ephemeris time (TDB) corresponding to the
///     C     observation.
///     C
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      LT
///
///           INTEGER               I
///
///     C
///     C     The required kernels must be loaded.
///     C
///           CALL FURNSH ( METAKR )
///
///     C
///     C     Calculate the state at 2004 Jun 11 19:32:00 UTC.
///     C
///           CALL STR2ET ( '2004-JUN-11-19:32:00', ET )
///
///     C
///     C     Calculate the apparent state of Phoebe as seen by
///     C     CASSINI in the J2000 frame.
///     C
///           CALL SPKEZR ( 'PHOEBE',  ET, 'IAU_PHOEBE', 'LT+S',
///          .              'CASSINI', STAREC, LT              )
///
///     C
///     C     Transform the state from rectangular to latitudinal.
///     C     Notice that since neither the input nor output
///     C     coordinate frames are 'geodetic' or 'planetographic',
///     C     the input for the body name is a blank string.
///     C
///           CALL XFMSTA ( STAREC, 'RECTANGULAR', 'LATITUDINAL', ' ',
///          .              STALAT )
///
///     C
///     C     Transform the state back to rectangular from latitudinal
///     C     for verification. This result should be very similar to
///     C     STAREC.
///     C
///           CALL XFMSTA ( STALAT, 'LATITUDINAL', 'RECTANGULAR',' ',
///          .              STREC2 )
///
///     C
///     C     Report the results.
///     C
///           WRITE (*,*)    ' '
///           WRITE (*,*)    'Phoebe as seen by CASSINI - rectangular'
///           WRITE (*,*)    '  Position [km]:'
///           WRITE (*,FORM) (STAREC(I), I = 1, 3)
///           WRITE (*,*)    '  Velocity [km/s]:'
///           WRITE (*,FORM) (STAREC(I), I = 4, 6)
///           WRITE (*,*)    ' '
///           WRITE (*,*)    'Phoebe as seen by CASSINI - latitudinal'
///           WRITE (*,*)    '  Position [km, rad, rad]:'
///           WRITE (*,FORM) (STALAT(I), I = 1, 3)
///           WRITE (*,*)    '  Velocity [km/s, rad/s, rad/s]:'
///           WRITE (*,FORM) (STALAT(I), I = 4, 6)
///           WRITE (*,*)    ' '
///           WRITE (*,*)    'Verification: '
///           WRITE (*,*)    'Phoebe as seen by CASSINI - rectangular'
///           WRITE (*,*)    '  Position [km]:'
///           WRITE (*,FORM) (STREC2(I), I = 1, 3)
///           WRITE (*,*)    '  Velocity [km/s]:'
///           WRITE (*,FORM) (STREC2(I), I = 4, 6)
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Phoebe as seen by CASSINI - rectangular
///        Position [km]:
///         -2059.271283     -942.128329      -95.837672
///        Velocity [km/s]:
///             3.910113       -4.228139       -1.526561
///
///      Phoebe as seen by CASSINI - latitudinal
///        Position [km, rad, rad]:
///          2266.580876       -2.712515       -0.042296
///        Velocity [km/s, rad/s, rad/s]:
///            -1.730462        0.002416       -0.000706
///
///      Verification:
///      Phoebe as seen by CASSINI - rectangular
///        Position [km]:
///         -2059.271283     -942.128329      -95.837672
///        Velocity [km/s]:
///             3.910113       -4.228139       -1.526561
///
///
///  2) Transform a given state from cylindrical to planetographic
///     coordinates with respect to Earth.
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
///           PROGRAM XFMSTA_EX2
///           IMPLICIT NONE
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         FORM
///           PARAMETER           ( FORM = '(F16.6, F16.6, F16.6)' )
///
///     C
///     C     Local variables
///     C
///     C     STACYL is the state in cylindrical coordinates.
///     C
///           DOUBLE PRECISION      STACYL (6)
///     C
///     C     STAPLN is the state transformed into planetographic
///     C     coordinates.
///     C
///           DOUBLE PRECISION      STAPLN (6)
///     C
///     C     STCYL2 is the state transformed back into
///     C     cylindrical coordinates from planetographic.
///     C
///           DOUBLE PRECISION      STCYL2 (6)
///
///           INTEGER               I
///
///           DATA STACYL / 1.0D0, 0.5D0, 0.5D0, 0.2D0, 0.1D0, -0.2D0 /
///
///     C
///     C     The required kernels must be loaded.
///     C
///           CALL FURNSH ( 'pck00010.tpc' )
///
///     C
///     C     Transform the state from cylindrical to planetographic.
///     C     Note that since one of the coordinate systems is
///     C     planetographic, the body name must be input.
///     C
///           CALL XFMSTA ( STACYL, 'CYLINDRICAL', 'PLANETOGRAPHIC',
///          .              'EARTH', STAPLN )
///
///     C
///     C     Transform the state back to cylindrical from
///     C     planetographic for verification. The result should be
///     C     very close to STACYL.
///     C
///           CALL XFMSTA ( STAPLN, 'PLANETOGRAPHIC', 'CYLINDRICAL',
///          .              'EARTH', STCYL2 )
///
///     C
///     C     Report the results.
///     C
///           WRITE (*,*)    'Cylindrical state'
///           WRITE (*,*)    '  Position [km, rad, km]:'
///           WRITE (*,FORM) (STACYL(I), I = 1, 3)
///           WRITE (*,*)    '  Velocity [km/s, rad/s, km/s]:'
///           WRITE (*,FORM) (STACYL(I), I = 4, 6)
///           WRITE (*,*)    ' '
///           WRITE (*,*) 'Planetographic state'
///           WRITE (*,*)    '  Position [rad, rad, km]:'
///           WRITE (*,FORM) (STAPLN(I), I = 1, 3)
///           WRITE (*,*)    '  Velocity [rad/s, rad/s, km/s]:'
///           WRITE (*,FORM) (STAPLN(I), I = 4, 6)
///           WRITE (*,*)    ' '
///           WRITE (*,*)    'Verification:  Cylindrical state'
///           WRITE (*,*)    '  Position [km, rad, km]:'
///           WRITE (*,FORM) (STCYL2(I), I = 1, 3)
///           WRITE (*,*)    '  Velocity [km/s, rad/s, km/s]:'
///           WRITE (*,FORM) (STCYL2(I), I = 4, 6)
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Cylindrical state
///        Position [km, rad, km]:
///             1.000000        0.500000        0.500000
///        Velocity [km/s, rad/s, km/s]:
///             0.200000        0.100000       -0.200000
///
///      Planetographic state
///        Position [rad, rad, km]:
///             0.500000        1.547722    -6356.240364
///        Velocity [rad/s, rad/s, km/s]:
///             0.100000       -0.004722       -0.195332
///
///      Verification:  Cylindrical state
///        Position [km, rad, km]:
///             1.000000        0.500000        0.500000
///        Velocity [km/s, rad/s, km/s]:
///             0.200000        0.100000       -0.200000
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  S.C. Krening       (JPL)
///  B.V. Semenov       (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 01-NOV-2021 (EDW) (JDR)
///
///         Body radii accessed from kernel pool using ZZGFTREB.
///
///         Edited he header to comply with NAIF standard.
///         Added missing begintext tag to the meta-kernel of example #1.
///         Modified example #2 to furnish a single kernel.
///
///         Updated Examples' kernels set to use PDS archived data.
///
/// -    SPICELIB Version 1.1.0, 09-FEB-2017 (BVS)
///
///         BUG FIX: the routine no longer allows converting to and from
///         geodetic and planetographic coordinates for bodies with
///         unequal equatorial radii. Previously it arbitrarily picked the
///         first and the third radii to compute body's flattening
///         coefficient.
///
/// -    SPICELIB Version 1.0.0, 22-APR-2014 (SCK) (BVS)
/// ```
pub fn xfmsta(
    ctx: &mut SpiceContext,
    istate: &[f64; 6],
    icosys: &str,
    ocosys: &str,
    body: &str,
    ostate: &mut [f64; 6],
) -> crate::Result<()> {
    XFMSTA(
        istate,
        icosys.as_bytes(),
        ocosys.as_bytes(),
        body.as_bytes(),
        ostate,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure XFMSTA ( Transform state between coordinate systems )
pub fn XFMSTA(
    ISTATE: &[f64],
    ICOSYS: &[u8],
    OCOSYS: &[u8],
    BODY: &[u8],
    OSTATE: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let ISTATE = DummyArray::new(ISTATE, 1..=6);
    let mut OSTATE = DummyArrayMut::new(OSTATE, 1..=6);
    let mut ISYSU = [b' '; CHSIZ as usize];
    let mut OSYSU = [b' '; CHSIZ as usize];
    let mut IPOS = StackArray::<f64, 3>::new(1..=3);
    let mut IVEL = StackArray::<f64, 3>::new(1..=3);
    let mut RADII = StackArray::<f64, 3>::new(1..=3);
    let mut JACOBI = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut F: f64 = 0.0;
    let mut SQTMP: f64 = 0.0;
    let mut TOOBIG: f64 = 0.0;
    let mut BODYID: i32 = 0;
    let mut ISYS: i32 = 0;
    let mut OSYS: i32 = 0;
    let mut FOUND: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    // Potentially large numbers produced by transforming the
    // velocity using the Jacobian must not exceed DPMAX()/MARGIN:
    //

    //
    // The size of each coordinate system name must not exceed
    // CHSIZ characters.
    //

    //
    // NCOSYS is the number of coordinate systems supported by
    // this routine.
    //

    //
    // The following integer parameters represent the coordinate
    // systems supported by this routine.
    //

    //
    // Saved body name length.
    //

    //
    // Local variables
    //
    // COSYS is the array of supported coordinate system names.
    // ISYSU and OSYSU are the input and output coordinate systems
    // from the user that are made insensitive to case or leading and
    // trailing spaces.
    //

    //
    // IPOS and IVEL are the input position and velocity translated
    // into rectangular.
    //

    //
    // For transformations including either geodetic or planetographic
    // coordinate systems, RADII is an array of the radii values
    // associated with the input body. These values will be loaded
    // from the kernel pool.
    //

    //
    // JACOBI is the Jacobian matrix that converts the velocity
    // coordinates between systems.
    //

    //
    // The flattening coefficient, F, is calculated when either
    // geodetic or planetographic coordinate systems are included
    // in the transformation.
    //

    //
    // SQTMP and TOOBIG are used to check for possible numeric
    // overflow situations.
    //

    //
    // BODYID is only used when the input or output coordinate
    // systems are geodetic or planetographic. The BODYID is the NAID ID
    // associated with the input body name.
    //

    //
    // ISYS and OSYS are the integer codes corresponding to the
    // input and output coordinate systems. I and J are iterators.
    //

    //
    // Saved name/ID item declarations.
    //

    //
    // Saved variables
    //

    //
    // Saved name/ID items.
    //

    //
    // Assign the names of the coordinate systems to a character
    // array in which each coordinate system name is located at
    // the index of the integer ID of the coordinate system.
    //

    //
    // Initial values.
    //

    //
    // There are three main sections of this routine:
    //
    //   1)  Error handling and initialization.
    //   2)  Conversion of the input to rectangular coordinates.
    //   3)  Conversion from rectangular to the output coordinates.
    //
    // Error handling and initialization
    // ----------------------------------------------------------------
    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"XFMSTA", ctx)?;

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
    // Remove initial and trailing spaces.
    // Convert the input coordinate systems to upper case.
    //
    LJUCRS(0, ICOSYS, &mut ISYSU, ctx);
    LJUCRS(0, OCOSYS, &mut OSYSU, ctx);

    //
    // Check to see if the input and output coordinate systems
    // provided by the user are acceptable. Store the integer
    // code of the input and output coordinate systems into
    // ISYS and OSYS.
    //
    ISYS = ISRCHC(&ISYSU, NCOSYS, save.COSYS.as_arg());
    OSYS = ISRCHC(&OSYSU, NCOSYS, save.COSYS.as_arg());

    //
    // If the coordinate systems are not acceptable, an error is
    // signaled.
    //
    if ((ISYS == 0) || (OSYS == 0)) {
        if ((ISYS == 0) && (OSYS == 0)) {
            //
            // Both the input and the output coordinate systems were not
            // recognized.
            //
            SETMSG(
                b"Input coordinate system # and output coordinate system # are not recognized.",
                ctx,
            );
            ERRCH(b"#", ICOSYS, ctx);
            ERRCH(b"#", OCOSYS, ctx);
            SIGERR(b"SPICE(COORDSYSNOTREC)", ctx)?;
            CHKOUT(b"XFMSTA", ctx)?;
            return Ok(());
        } else if (ISYS == 0) {
            //
            // The input coordinate system was not recognized.
            //
            SETMSG(b"Input coordinate system # was not recognized", ctx);
            ERRCH(b"#", ICOSYS, ctx);
            SIGERR(b"SPICE(COORDSYSNOTREC)", ctx)?;
            CHKOUT(b"XFMSTA", ctx)?;
            return Ok(());
        } else {
            //
            // The output coordinate system was not recognized.
            //
            SETMSG(b"Output coordinate system # was not recognized", ctx);
            ERRCH(b"#", OCOSYS, ctx);
            SIGERR(b"SPICE(COORDSYSNOTREC)", ctx)?;
            CHKOUT(b"XFMSTA", ctx)?;
            return Ok(());
        }
    }

    //
    // If the input and output coordinate systems are equal, set the
    // output equal to the input since no conversion needs to take
    // place.
    //
    if (ISYS == OSYS) {
        VEQUG(ISTATE.as_slice(), 6, OSTATE.as_slice_mut());
        CHKOUT(b"XFMSTA", ctx)?;
        return Ok(());
    }

    //
    // If converting to or from either geodetic or planetographic, the
    // NAIF ID must be found from the input body name BODY. If the
    // body name does not have a valid NAIF ID code, an error is
    // signaled. If the NAIF ID is valid, the radii of the body are
    // located and the flattening coefficient is calculated.
    //
    if ((((OSYS == GEODET) || (OSYS == PLNTGR)) || (ISYS == GEODET)) || (ISYS == PLNTGR)) {
        //
        // Find the NAIF ID code
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
        //
        // If the body's name was found, find the body's radii and
        // compute flattening coefficient. Otherwise, signal an error.
        //
        if FOUND {
            ZZGFTREB(BODYID, RADII.as_slice_mut(), ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"XFMSTA", ctx)?;
                return Ok(());
            }

            //
            // If the difference of the equatorial and polar radii
            // divided by the equatorial radius is greater than DPMAX,
            // a numeric overflow may occur, so an error is signaled.
            //
            if ((f64::sqrt(f64::abs((RADII[1] - RADII[3]))) / f64::sqrt(f64::abs(RADII[1])))
                >= f64::sqrt(DPMAX()))
            {
                SETMSG(b"The equatorial radius for # has a value of # and a polar radius of #. The flattening coefficient cannot be calculated due to numeric overflow.", ctx);
                ERRCH(b"#", BODY, ctx);
                ERRDP(b"#", RADII[1], ctx);
                ERRDP(b"#", RADII[3], ctx);
                SIGERR(b"SPICE(INVALIDRADIUS)", ctx)?;
                CHKOUT(b"XFMSTA", ctx)?;
                return Ok(());
            }

            //
            // At this point, we also check for unequal equatorial radii,
            // which are not allowed with geodetic or planetographic
            // coordinates.
            //
            if (RADII[1] != RADII[2]) {
                SETMSG(b"The body # has radii (#, #, #). Unequal equatorial ellipsoid radii are not supported for # and # coordinates.", ctx);
                ERRCH(b"#", BODY, ctx);
                ERRDP(b"#", RADII[1], ctx);
                ERRDP(b"#", RADII[2], ctx);
                ERRDP(b"#", RADII[3], ctx);
                ERRCH(b"#", &save.COSYS[GEODET], ctx);
                ERRCH(b"#", &save.COSYS[PLNTGR], ctx);
                SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
                CHKOUT(b"XFMSTA", ctx)?;
                return Ok(());
            }

            //
            // Calculate the flattening coefficient, F.
            //
            F = ((RADII[1] - RADII[3]) / RADII[1]);
        } else {
            SETMSG(
                b"The input body name # does not have a valid NAIF ID code.",
                ctx,
            );
            ERRCH(b"#", BODY, ctx);
            SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
            CHKOUT(b"XFMSTA", ctx)?;
            return Ok(());
        }
    }

    //
    // Conversion of the input to rectangular coordinates
    // ----------------------------------------------------------------
    //
    // First, the position and velocity coordinates will be converted
    // into rectangular coordinates. If the input system is not
    // rectangular, then the velocity coordinates must be translated
    // into rectangular using the Jacobian. If the input system is
    // rectangular, then the input state must simply be saved into IPOS
    // and IVEL.
    //
    // TOOBIG is used for preventing numerical overflow. The square
    // roots of values are used to safely check if overflow will occur.
    //
    TOOBIG = f64::sqrt((DPMAX() / MARGIN));

    if (ISYS != RECTAN) {
        //
        // To rectangular...
        //
        if (ISYS == CYL) {
            //
            // ... from cylindrical
            //
            CYLREC(ISTATE[1], ISTATE[2], ISTATE[3], IPOS.as_slice_mut());

            DRDCYL(ISTATE[1], ISTATE[2], ISTATE[3], JACOBI.as_slice_mut());
        } else if (ISYS == LATNL) {
            //
            // ... from latitudinal
            //
            LATREC(ISTATE[1], ISTATE[2], ISTATE[3], IPOS.as_slice_mut());

            DRDLAT(ISTATE[1], ISTATE[2], ISTATE[3], JACOBI.as_slice_mut());
        } else if (ISYS == SPHCL) {
            //
            // ... from spherical
            //
            SPHREC(ISTATE[1], ISTATE[2], ISTATE[3], IPOS.as_slice_mut());

            DRDSPH(ISTATE[1], ISTATE[2], ISTATE[3], JACOBI.as_slice_mut());
        } else if (ISYS == GEODET) {
            //
            // ... from geodetic
            //
            GEOREC(
                ISTATE[1],
                ISTATE[2],
                ISTATE[3],
                RADII[1],
                F,
                IPOS.as_slice_mut(),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"XFMSTA", ctx)?;
                return Ok(());
            }

            DRDGEO(
                ISTATE[1],
                ISTATE[2],
                ISTATE[3],
                RADII[1],
                F,
                JACOBI.as_slice_mut(),
                ctx,
            )?;
        } else if (ISYS == PLNTGR) {
            //
            // ... from planetographic
            //
            PGRREC(
                BODY,
                ISTATE[1],
                ISTATE[2],
                ISTATE[3],
                RADII[1],
                F,
                IPOS.as_slice_mut(),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"XFMSTA", ctx)?;
                return Ok(());
            }

            DRDPGR(
                BODY,
                ISTATE[1],
                ISTATE[2],
                ISTATE[3],
                RADII[1],
                F,
                JACOBI.as_slice_mut(),
                ctx,
            )?;
        } else {
            SETMSG(b"This error should never occur. This is an intermediate step in which a non-rectangular input state should be transferred to rectangular.  The input coordinate system is not recognized, yet was not caught by an earlier check.", ctx);
            SIGERR(b"SPICE(BUG1)", ctx)?;
            CHKOUT(b"XFMSTA", ctx)?;
            return Ok(());
        }

        //
        // Some DRD* routines are not error free. Be safe and check
        // FAILED to not use un-initialized JACOBI.
        //
        if FAILED(ctx) {
            CHKOUT(b"XFMSTA", ctx)?;
            return Ok(());
        }

        //
        // If the multiplication of the Jacobian and velocity can cause
        // overflow, signal an error.
        //
        for I in 1..=3 {
            for J in 1..=3 {
                SQTMP =
                    (f64::sqrt(f64::abs(JACOBI[[I, J]])) * f64::sqrt(f64::abs(ISTATE[(J + 3)])));

                if (SQTMP > TOOBIG) {
                    SETMSG(
                        b"The product of the Jacobian and velocity may cause numeric overflow.",
                        ctx,
                    );
                    SIGERR(b"SPICE(NUMERICOVERFLOW)", ctx)?;
                    CHKOUT(b"XFMSTA", ctx)?;
                    return Ok(());
                }
            }
        }

        //
        // Transform the velocity into rectangular coordinates.
        //
        MXV(JACOBI.as_slice(), ISTATE.subarray(4), IVEL.as_slice_mut());
    } else if (ISYS == RECTAN) {
        //
        // If the input coordinate system is rectangular, the input
        // position does not need to be translated into rectangular.
        //
        VEQU(ISTATE.subarray(1), IPOS.as_slice_mut());
        VEQU(ISTATE.subarray(4), IVEL.as_slice_mut());
    } else {
        SETMSG(b"This error should never occur. This is an ELSE statement. If the input coordinate system is not rectangular, the IF should be executed. If the input coordinate system is rectangular, the ELSE IF should be executed.", ctx);
        SIGERR(b"SPICE(BUG2)", ctx)?;
        CHKOUT(b"XFMSTA", ctx)?;
        return Ok(());
    }

    //
    // Conversion from rectangular into the output coordinates
    // ----------------------------------------------------------------
    //
    // Convert to the output coordinate system. If the output
    // coordinate system is not rectangular, four calculations must
    // be made:
    //
    //   1)  Verify the position and velocity are not along the z-axis.
    //       If the position and velocity are along the z-axis, the
    //       velocity can still be converted even though the
    //       Jacobian is not defined. If the position is along the
    //       z-axis but the velocity is not, the velocity cannot be
    //       converted to the output coordinate system.
    //
    //   2)  Calculate the Jacobian from rectangular to the output
    //       coordinate system and verify the product of the Jacobian
    //       and velocity will not cause numeric overflow.
    //
    //   3)  Transform the position to the output coordinate system.
    //
    //   4)  Transform the velocity to the output coordinates using
    //       the Jacobian and the rectangular velocity IVEL.
    //
    if (OSYS != RECTAN) {
        //
        // From rectangular for the case when the input position is along
        // the z-axis ...
        //
        if ((f64::abs(IPOS[1]) + f64::abs(IPOS[2])) == 0.0) {
            if ((f64::abs(IVEL[1]) + f64::abs(IVEL[2])) == 0.0) {
                //
                // If the velocity is along the z-axis, then the velocity
                // can be computed in the output coordinate frame even
                // though the Jacobian is not defined.
                //
                if (OSYS == CYL) {
                    //
                    // ... to cylindrical
                    //
                    VPACK(0.0, 0.0, IVEL[3], OSTATE.subarray_mut(4));

                    let [arg1, arg2, arg3] = OSTATE.get_disjoint_mut([1, 2, 3]).expect(
                        "mutable array elements passed to function must have disjoint indexes",
                    );
                    RECCYL(IPOS.subarray(1), arg1, arg2, arg3, ctx);
                } else if (OSYS == LATNL) {
                    //
                    // ... to latitudinal
                    //
                    VPACK(IVEL[3], 0.0, 0.0, OSTATE.subarray_mut(4));

                    let [arg1, arg2, arg3] = OSTATE.get_disjoint_mut([1, 2, 3]).expect(
                        "mutable array elements passed to function must have disjoint indexes",
                    );
                    RECLAT(IPOS.subarray(1), arg1, arg2, arg3);
                } else if (OSYS == SPHCL) {
                    //
                    // ... to spherical
                    //
                    VPACK(IVEL[3], 0.0, 0.0, OSTATE.subarray_mut(4));

                    let [arg1, arg2, arg3] = OSTATE.get_disjoint_mut([1, 2, 3]).expect(
                        "mutable array elements passed to function must have disjoint indexes",
                    );
                    RECSPH(IPOS.subarray(1), arg1, arg2, arg3);
                } else if (OSYS == GEODET) {
                    //
                    // ... to geodetic
                    //
                    VPACK(0.0, 0.0, IVEL[3], OSTATE.subarray_mut(4));

                    let [arg3, arg4, arg5] = OSTATE.get_disjoint_mut([1, 2, 3]).expect(
                        "mutable array elements passed to function must have disjoint indexes",
                    );
                    RECGEO(IPOS.subarray(1), RADII[1], F, arg3, arg4, arg5, ctx)?;
                } else if (OSYS == PLNTGR) {
                    //
                    // ... to planetographic
                    //
                    VPACK(0.0, 0.0, IVEL[3], OSTATE.subarray_mut(4));

                    let [arg4, arg5, arg6] = OSTATE.get_disjoint_mut([1, 2, 3]).expect(
                        "mutable array elements passed to function must have disjoint indexes",
                    );
                    RECPGR(BODY, IPOS.subarray(1), RADII[1], F, arg4, arg5, arg6, ctx)?;
                } else {
                    SETMSG(b"This error should never occur. This is an intermediate step in which a position and velocity along the z-axis are converted to a non-rectangular coordinate system from rectangular. The output coordinate system is not recognized, yet was not caught by an earlier check.", ctx);
                    SIGERR(b"SPICE(BUG3)", ctx)?;
                    CHKOUT(b"XFMSTA", ctx)?;
                    return Ok(());
                }
                //
                // The output state has been calculated for the special
                // case of the position and velocity existing along the
                // z-axis.
                //
                CHKOUT(b"XFMSTA", ctx)?;
                return Ok(());
            } else {
                //
                // The Jacobian is undefined and the velocity cannot be
                // converted since it is not along the z-axis.
                // Signal an error.
                //
                SETMSG(b"Invalid input state: z axis.", ctx);
                SIGERR(b"SPICE(INVALIDSTATE)", ctx)?;
                CHKOUT(b"XFMSTA", ctx)?;
                return Ok(());
            }
        }

        //
        // From rectangular for cases when the input position is not along
        // the z-axis ...
        //
        if (OSYS == CYL) {
            //
            // ... to cylindrical
            //
            DCYLDR(IPOS[1], IPOS[2], IPOS[3], JACOBI.as_slice_mut(), ctx)?;

            let [arg1, arg2, arg3] = OSTATE
                .get_disjoint_mut([1, 2, 3])
                .expect("mutable array elements passed to function must have disjoint indexes");
            RECCYL(IPOS.subarray(1), arg1, arg2, arg3, ctx);
        } else if (OSYS == LATNL) {
            //
            // ... to latitudinal
            //
            DLATDR(IPOS[1], IPOS[2], IPOS[3], JACOBI.as_slice_mut(), ctx)?;

            let [arg1, arg2, arg3] = OSTATE
                .get_disjoint_mut([1, 2, 3])
                .expect("mutable array elements passed to function must have disjoint indexes");
            RECLAT(IPOS.subarray(1), arg1, arg2, arg3);
        } else if (OSYS == SPHCL) {
            //
            // ... to spherical
            //
            DSPHDR(IPOS[1], IPOS[2], IPOS[3], JACOBI.as_slice_mut(), ctx)?;

            let [arg1, arg2, arg3] = OSTATE
                .get_disjoint_mut([1, 2, 3])
                .expect("mutable array elements passed to function must have disjoint indexes");
            RECSPH(IPOS.subarray(1), arg1, arg2, arg3);
        } else if (OSYS == GEODET) {
            //
            // ... to geodetic
            //
            DGEODR(
                IPOS[1],
                IPOS[2],
                IPOS[3],
                RADII[1],
                F,
                JACOBI.as_slice_mut(),
                ctx,
            )?;

            let [arg3, arg4, arg5] = OSTATE
                .get_disjoint_mut([1, 2, 3])
                .expect("mutable array elements passed to function must have disjoint indexes");
            RECGEO(IPOS.subarray(1), RADII[1], F, arg3, arg4, arg5, ctx)?;
        } else if (OSYS == PLNTGR) {
            //
            // ... to planetographic
            //
            DPGRDR(
                BODY,
                IPOS[1],
                IPOS[2],
                IPOS[3],
                RADII[1],
                F,
                JACOBI.as_slice_mut(),
                ctx,
            )?;

            let [arg4, arg5, arg6] = OSTATE
                .get_disjoint_mut([1, 2, 3])
                .expect("mutable array elements passed to function must have disjoint indexes");
            RECPGR(BODY, IPOS.subarray(1), RADII[1], F, arg4, arg5, arg6, ctx)?;
        } else {
            SETMSG(b"This error should never occur. This is an intermediate step in which a state is converted to a non-rectangular coordinate system from rectangular. The output coordinate system is not recognized, yet was not caught by an earlier check.", ctx);
            SIGERR(b"SPICE(BUG4)", ctx)?;
            CHKOUT(b"XFMSTA", ctx)?;
            return Ok(());
        }

        //
        // Many D*DR and REC* routines are not error free. Be safe and
        // check FAILED to not use un-initialized JACOBI.
        //
        if FAILED(ctx) {
            CHKOUT(b"XFMSTA", ctx)?;
            return Ok(());
        }

        //
        // If the multiplication of the Jacobian and velocity can cause
        // overflow, signal an error.
        //
        for I in 1..=3 {
            for J in 1..=3 {
                SQTMP = (f64::sqrt(f64::abs(JACOBI[[I, J]])) * f64::sqrt(f64::abs(IVEL[J])));

                if (SQTMP > TOOBIG) {
                    SETMSG(
                        b"The product of the Jacobian and velocity may cause numeric overflow.",
                        ctx,
                    );
                    SIGERR(b"SPICE(NUMERICOVERFLOW)", ctx)?;
                    CHKOUT(b"XFMSTA", ctx)?;
                    return Ok(());
                }
            }
        }

        //
        // Calculate the velocity in the output coordinate system.
        //
        MXV(JACOBI.as_slice(), IVEL.as_slice(), OSTATE.subarray_mut(4));
    } else if (OSYS == RECTAN) {
        //
        // If the output coordinate system is rectangular, the position
        // and velocity components of the output state are set equal to
        // the rectangular IPOS and IVEL, respectively, because the
        // components have already been converted to rectangular.
        //
        VEQU(IPOS.as_slice(), OSTATE.subarray_mut(1));
        VEQU(IVEL.as_slice(), OSTATE.subarray_mut(4));
    } else {
        SETMSG(b"This error should never occur. This is an ELSE statement. If the output coordinate system is not rectangular, the IF should be executed. If the output coordinate system is rectangular, the ELSE IF should be executed.", ctx);
        SIGERR(b"SPICE(BUG5)", ctx)?;
        CHKOUT(b"XFMSTA", ctx)?;
        return Ok(());
    }

    CHKOUT(b"XFMSTA", ctx)?;
    Ok(())
}
