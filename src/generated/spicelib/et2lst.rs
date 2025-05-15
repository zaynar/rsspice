//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const BDNMLN: i32 = 36;
const SUN: i32 = 10;
const WDSIZE: i32 = 32;

/// ET to Local Solar Time
///
/// Compute the local solar time for a given ephemeris epoch ET
/// for an object on the surface of a body at a specified longitude.
///
/// # Required Reading
///
/// * [TIME](crate::required_reading::time)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ET         I   Epoch in seconds past J2000 epoch
///  BODY       I   ID-code of the body of interest
///  LON        I   Longitude of surface point (RADIANS)
///  TYPE       I   Type of longitude 'PLANETOCENTRIC', etc.
///  HR         O   Local hour on a "24 hour" clock
///  MN         O   Minutes past the hour
///  SC         O   Seconds past the minute
///  TIME       O   String giving local time on 24 hour clock
///  AMPM       O   String giving time on A.M./ P.M. scale
/// ```
///
/// # Detailed Input
///
/// ```text
///  ET       is the epoch expressed in TDB seconds past the J2000
///           epoch at which a local time is desired.
///
///  BODY     is the NAIF ID-code of a body on which local time is to
///           be measured.
///
///  LON      is the longitude (either planetocentric or
///           planetographic) in radians of the site on the surface of
///           body for which local time should be computed.
///
///  TYPE     is the form of longitude supplied by the variable LON.
///           Allowed values are:
///
///              'PLANETOCENTRIC'
///              'PLANETOGRAPHIC'
///
///           Note the case of the letters in TYPE is insignificant.
///           Both 'PLANETOCENTRIC' and 'planetocentric' are
///           recognized.
/// ```
///
/// # Detailed Output
///
/// ```text
///  HR       is the local "hour" of the site specified at the epoch
///           ET. Note that an "hour" of local time does not have the
///           same duration as an hour measured by conventional clocks.
///           It is simply a representation of an angle. See the
///           $Particulars section for a more complete discussion of
///           the meaning of local time.
///
///  MN       is the number of "minutes" past the hour of the local
///           time of the site at the epoch ET. Again note that a
///           "local minute" is not the same as a minute you would
///           measure with conventional clocks.
///
///  SC       is the number of "seconds" past the minute of the local
///           time of the site at the epoch ET. Again note that a
///           "local second" is not the same as a second you would
///           measure with conventional clocks.
///
///  TIME     is a string expressing the local time on a "24 hour"
///           local clock.
///
///  AMPM     is a string expressing the local time on a "12 hour"
///           local clock together with the traditional AM/PM label to
///           indicate whether the sun has crossed the local zenith
///           meridian.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  This routine defines local solar time for any point on the
///      surface of the Sun to be 12:00:00 noon.
///
///  2)  If the TYPE of the coordinates is not recognized, the
///      error SPICE(UNKNOWNSYSTEM) is signaled.
///
///  3)  If the body-fixed frame to associate with BODY cannot be
///      determined, the error SPICE(CANTFINDFRAME) is signaled.
///
///  4)  If insufficient data are available to compute the location of
///      the sun in body-fixed coordinates, an error is signaled by a
///      routine in the call tree of this routine.
///
///  5)  If the BODY#_PM keyword required to determine the body
///      rotation sense is not found in the POOL or if it is found but
///      is not a numeric keyword with at least two elements, the error
///      SPICE(CANTGETROTATIONTYPE) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  Suitable SPK and PCK files must be loaded prior to calling this
///  routine so that the body-fixed position of the sun relative to
///  BODY can be computed. The PCK files must contain the standard
///  BODY#_PM keyword need by this routine to determine the body
///  rotation sense.
///
///  When the input longitude is planetographic, the default
///  interpretation of this value can be overridden using the optional
///  kernel variable
///
///     BODY<body ID>_PGR_POSITIVE_LON
///
///  which is normally defined via loading a text kernel.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine returns the local solar time at a user
///  specified location on a user specified body.
///
///  Let SUNLNG be the planetocentric longitude (in degrees) of
///  the sun as viewed from the center of the body of interest.
///
///  Let SITLNG be the planetocentric longitude (in degrees) of
///  the site for which local time is desired.
///
///  We define local time to be 12 + (SITLNG - SUNLNG)/15
///
///  (where appropriate care is taken to map ( SITLNG - SUNLNG )
///  into the range from -180 to 180).
///
///  Using this definition, we see that from the point of view
///  of this routine, local solar time is simply a measure of angles
///  between meridians on the surface of a body. Consequently,
///  this routine is not appropriate for computing "local times"
///  in the sense of Pacific Standard Time. For computing times
///  relative to standard time zones on earth, see the routines
///  TIMOUT and STR2ET.
///
///
///  Regarding planetographic longitude
///  ----------------------------------
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
///  If you wish to override the default sense of positive
///  planetographic longitude for a particular body, you can do so by
///  defining the kernel variable
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
///  1) The following code example illustrates how to compute the
///     local time at a site on Mars with planetographic longitude
///     +326.17 deg at epoch ET.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: et2lst_ex1.tm
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
///           pck00010.tpc                  Planet orientation and
///                                         radii
///           naif0012.tls                  Leapseconds
///
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de421.bsp',
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
///           PROGRAM ET2LST_EX1
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
///           CHARACTER*(*)         FMT
///           PARAMETER           ( FMT    = '(A,F7.2,A)'    )
///
///           CHARACTER*(*)         META
///           PARAMETER           ( META   = 'et2lst_ex1.tm' )
///
///           CHARACTER*(*)         TYPE
///           PARAMETER           ( TYPE   = 'PLANETOGRAPHIC' )
///
///           INTEGER               AMPMLEN
///           PARAMETER           ( AMPMLEN = 51 )
///
///           INTEGER               MARS
///           PARAMETER           ( MARS   = 499 )
///
///           INTEGER               TIMLEN
///           PARAMETER           ( TIMLEN = 51 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(AMPMLEN)   AMPM
///           CHARACTER*(TIMLEN)    TIME
///           CHARACTER*(20)        UTCSTR
///
///           DOUBLE PRECISION      DLON
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      RLON
///
///           INTEGER               HR
///           INTEGER               MN
///           INTEGER               SC
///
///     C
///     C     Load the kernels.
///     C
///           CALL FURNSH ( META )
///
///           DLON   =  326.17D0
///           RLON   =  DLON * RPD( )
///           UTCSTR = '2002 SEP 02 00:00:00'
///
///           CALL STR2ET ( UTCSTR, ET )
///
///           CALL ET2LST ( ET, MARS, RLON, TYPE,
///          .              HR, MN,   SC,   TIME, AMPM )
///
///           WRITE(*,FMT) 'Local time at Mars', DLON,
///          .             ' degrees planetographic longitude:'
///           WRITE(*,*)   '   at UTC ', UTCSTR, ', LST = ', AMPM
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Local time at Mars 326.17 degrees planetographic longitude:
///         at UTC 2002 SEP 02 00:00:00, LST = 03:25:35 A.M.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine relies on being able to determine the name
///      of the body-fixed frame associated with BODY through the
///      frames subsystem. If the BODY specified is NOT one of the
///      nine planets or their satellites, you will need to load
///      an appropriate frame definition kernel that contains
///      the relationship between the body id and the body-fixed frame
///      name. See frames.req required reading for more details
///      on specifying this relationship.
///
///  2)  The routine determines the body rotation sense using the PCK
///      keyword BODY#_PM. Therefore, you will need to a text PCK file
///      defining the complete set of the standard PCK body rotation
///      keywords for the body of interest. The text PCK file must be
///      loaded independently of whether a binary PCK file providing
///      rotation data for the same body is loaded or not.
///
///  3)  Although it is not currently the case for any of the Solar
///      System bodies, it is possible that the retrograde rotation
///      rate of a body would be slower than the orbital rate of the
///      body rotation around the Sun. The routine does not account for
///      such cases; for them it will compute incorrect the local time
///      progressing backwards.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.1.0, 26-OCT-2021 (JDR)
///
///         Changed the input argument name LONG to LON for consistency
///         with other routines.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section. Added complete code example
///         from existing fragment.
///
/// -    SPICELIB Version 3.0.2, 18-APR-2014 (BVS)
///
///         Minor edits to long error messages.
///
/// -    SPICELIB Version 3.0.1, 09-SEP-2009 (EDW)
///
///         Header edits: deleted a spurious C$ marker from the
///         "Detailed_Output" section. The existence of the marker
///         caused a failure in the HTML documentation creation script.
///
///         Deleted the "Revisions" section as it contained several
///         identical entries from the "Version" section.
///
///         Corrected order of header sections.
///
/// -    SPICELIB Version 3.0.0, 28-OCT-2006 (BVS)
///
///         Bug fix: incorrect computation of the local time for the
///         bodies with the retrograde rotation causing the local time to
///         flow backwards has been fixed. The local time for all types of
///         bodies now progresses as expected -- midnight, increasing AM
///         hours, noon, increasing PM hours, next midnight, and so on.
///
/// -    SPICELIB Version 2.0.0, 03-NOV-2005 (NJB)
///
///         Bug fix: treatment of planetographic longitude has been
///         updated to be consistent with the SPICE planetographic/
///         rectangular coordinate conversion routines. The effect of
///         this change is that the default sense of positive longitude
///         for the moon is now east; also, the default sense of positive
///         planetographic longitude now may be overridden for any body
///         (see $Particulars above).
///
///         Updated to remove non-standard use of duplicate arguments
///         in RMAIND calls.
///
/// -    SPICELIB Version 1.1.0, 24-MAR-1998 (WLT)
///
///         The integer variable SUN was never initialized in the
///         previous version of the routine. Now it is set to
///         the proper value of 10.
///
/// -    SPICELIB Version 1.0.0, 09-JUL-1997 (WLT)
/// ```
pub fn et2lst(
    ctx: &mut SpiceContext,
    et: f64,
    body: i32,
    lon: f64,
    type_: &str,
    hr: &mut i32,
    mn: &mut i32,
    sc: &mut i32,
    time: &mut str,
    ampm: &mut str,
) -> crate::Result<()> {
    ET2LST(
        et,
        body,
        lon,
        type_.as_bytes(),
        hr,
        mn,
        sc,
        fstr::StrBytes::new(time).as_mut(),
        fstr::StrBytes::new(ampm).as_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure ET2LST ( ET to Local Solar Time )
pub fn ET2LST(
    ET: f64,
    BODY: i32,
    LON: f64,
    TYPE: &[u8],
    HR: &mut i32,
    MN: &mut i32,
    SC: &mut i32,
    TIME: &mut [u8],
    AMPM: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut AMORPM = [b' '; 4 as usize];
    let mut BODNAM = [b' '; BDNMLN as usize];
    let mut FRAME = [b' '; WDSIZE as usize];
    let mut H = [b' '; 2 as usize];
    let mut BPMKWD = [b' '; WDSIZE as usize];
    let mut M = [b' '; 2 as usize];
    let mut MYTYPE = [b' '; WDSIZE as usize];
    let mut S = [b' '; 2 as usize];
    let mut KWTYPE = [b' '; 1 as usize];
    let mut ANGLE: f64 = 0.0;
    let mut HOURS: f64 = 0.0;
    let mut LAT: f64 = 0.0;
    let mut LT: f64 = 0.0;
    let mut MINS: f64 = 0.0;
    let mut MYLONG: f64 = 0.0;
    let mut RANGE: f64 = 0.0;
    let mut RATE: f64 = 0.0;
    let mut Q: f64 = 0.0;
    let mut SECNDS: f64 = 0.0;
    let mut SLAT: f64 = 0.0;
    let mut SLONG: f64 = 0.0;
    let mut SPOINT = StackArray::<f64, 3>::new(1..=3);
    let mut STATE = StackArray::<f64, 6>::new(1..=6);
    let mut TMPANG: f64 = 0.0;
    let mut TMPSEC: f64 = 0.0;
    let mut FRCODE: i32 = 0;
    let mut HRAMPM: i32 = 0;
    let mut N: i32 = 0;
    let mut FOUND: bool = false;

    //
    // SPICELIB Functions
    //

    //
    // Local parameters
    //
    //

    //
    // Local Variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ET2LST", ctx)?;

    LJUST(TYPE, &mut MYTYPE);
    UCASE(&MYTYPE.clone(), &mut MYTYPE, ctx);

    if fstr::eq(&MYTYPE, b"PLANETOGRAPHIC") {
        //
        // Find planetocentric longitude corresponding to the input
        // longitude.  We first represent in rectangular coordinates
        // a surface point having zero latitude, zero altitude, and
        // the input planetographic longitude. We then find the
        // planetocentric longitude of this point.
        //
        // Since PGRREC accepts a body name, map the input code to
        // a name, if possible.  Otherwise, just convert the input code
        // to a string.
        //
        BODC2N(BODY, &mut BODNAM, &mut FOUND, ctx)?;

        if !FOUND {
            INTSTR(BODY, &mut BODNAM, ctx);
        }

        //
        // Convert planetographic coordinates to rectangular coordinates.
        // All we care about here is longitude.  Set the other inputs
        // as follows:
        //
        //     Latitude          = 0
        //     Altitude          = 0
        //     Equatorial radius = 1
        //     Flattening factor = 0
        //
        PGRREC(&BODNAM, LON, 0.0, 0.0, 1.0, 0.0, SPOINT.as_slice_mut(), ctx)?;

        //
        // The output MYLONG is planetocentric longitude.  The other
        // outputs are not used.  Note that the variable RANGE appears
        // later in another RECLAT call; it's not used after that.
        //
        RECLAT(SPOINT.as_slice(), &mut RANGE, &mut MYLONG, &mut LAT);
    } else if fstr::eq(&MYTYPE, b"PLANETOCENTRIC") {
        MYLONG = LON;
    } else {
        SETMSG(b"The coordinate system \'#\' is not a recognized system of longitude.  The recognized systems are \'PLANETOCENTRIC\' and \'PLANETOGRAPHIC\'. ", ctx);

        ERRCH(b"#", TYPE, ctx);
        SIGERR(b"SPICE(UNKNOWNSYSTEM)", ctx)?;
        CHKOUT(b"ET2LST", ctx)?;
        return Ok(());
    }
    //
    // It's always noon on the surface of the sun.
    //
    if (BODY == 10) {
        *HR = 12;
        *MN = 0;
        *SC = 0;

        fstr::assign(TIME, b"12:00:00");
        fstr::assign(AMPM, b"12:00:00 P.M.");

        CHKOUT(b"ET2LST", ctx)?;
        return Ok(());
    }

    //
    // Get the body-fixed position of the sun.
    //
    CIDFRM(BODY, &mut FRCODE, &mut FRAME, &mut FOUND, ctx)?;

    if !FOUND {
        SETMSG(b"The body-fixed frame associated with body # could not be determined.  This information needs to be \"loaded\" via a frames definition kernel.  See frames.req for more details. ", ctx);
        ERRINT(b"#", BODY, ctx);
        SIGERR(b"SPICE(CANTFINDFRAME)", ctx)?;
        CHKOUT(b"ET2LST", ctx)?;
        return Ok(());
    }

    SPKEZ(
        SUN,
        ET,
        &FRAME,
        b"LT+S",
        BODY,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    RECLAT(STATE.as_slice(), &mut RANGE, &mut SLONG, &mut SLAT);

    ANGLE = (MYLONG - SLONG);

    //
    // Force the angle into the region from -PI to PI
    //
    RMAIND(ANGLE, TWOPI(ctx), &mut Q, &mut TMPANG, ctx)?;
    ANGLE = TMPANG;

    if (ANGLE > PI(ctx)) {
        ANGLE = (ANGLE - TWOPI(ctx));
    }

    //
    // Get the rotation sense of the body and invert the angle if the
    // rotation sense is retrograde. Use the BODY#_PM PCK keyword to
    // determine the sense of the body rotation.
    //
    fstr::assign(&mut BPMKWD, b"BODY#_PM");
    REPMI(&BPMKWD.clone(), b"#", BODY, &mut BPMKWD, ctx);

    DTPOOL(&BPMKWD, &mut FOUND, &mut N, &mut KWTYPE, ctx)?;

    if ((!FOUND || fstr::ne(&KWTYPE, b"N")) || (N < 2)) {
        SETMSG(b"The rotation type for the body # could not be determined because the # keyword was either not found in the POOL or or it was not of the expected type and/or dimension. This keyword is usually provided via a planetary constants kernel. See pck.req for more details. ", ctx);
        ERRINT(b"#", BODY, ctx);
        ERRCH(b"#", &BPMKWD, ctx);
        SIGERR(b"SPICE(CANTGETROTATIONTYPE)", ctx)?;
        CHKOUT(b"ET2LST", ctx)?;
        return Ok(());
    } else {
        //
        // If the rotation rate is negative, invert the angle.
        //
        GDPOOL(
            &BPMKWD,
            2,
            1,
            &mut N,
            std::slice::from_mut(&mut RATE),
            &mut FOUND,
            ctx,
        )?;

        if (RATE < 0.0) {
            ANGLE = -ANGLE;
        }
    }

    //
    // Convert the angle to "angle seconds" before or after local noon.
    //
    SECNDS = ((86400.0 * ANGLE) / TWOPI(ctx));
    SECNDS = BRCKTD(SECNDS, -43200.0, 43200.0);

    //
    // Get the hour, and minutes components of the local time.
    //
    RMAIND(SECNDS, 3600.0, &mut HOURS, &mut TMPSEC, ctx)?;
    RMAIND(TMPSEC, 60.0, &mut MINS, &mut SECNDS, ctx)?;
    //
    // Construct the integer components of the local time.
    //
    *HR = (12 + (HOURS as i32));
    *MN = (MINS as i32);
    *SC = (SECNDS as i32);
    //
    // Set the A.M./P.M. components of local time.
    //
    if (*HR == 24) {
        *HR = 0;
        HRAMPM = 12;
        fstr::assign(&mut AMORPM, b"A.M.");
    } else if (*HR > 12) {
        HRAMPM = (*HR - 12);
        fstr::assign(&mut AMORPM, b"P.M.");
    } else if (*HR == 12) {
        HRAMPM = 12;
        fstr::assign(&mut AMORPM, b"P.M.");
    } else if (*HR == 0) {
        HRAMPM = 12;
        fstr::assign(&mut AMORPM, b"A.M.");
    } else {
        HRAMPM = *HR;
        fstr::assign(&mut AMORPM, b"A.M.");
    }

    //
    // Now construct the two strings we need.
    //
    HOURS = (*HR as f64);
    MINS = (*MN as f64);
    SECNDS = (*SC as f64);

    DPFMT(HOURS, b"0x", &mut H, ctx)?;
    DPFMT(MINS, b"0x", &mut M, ctx)?;
    DPFMT(SECNDS, b"0x", &mut S, ctx)?;

    fstr::assign(
        TIME,
        &fstr::concat(
            &fstr::concat(&fstr::concat(&fstr::concat(&H, b":"), &M), b":"),
            &S,
        ),
    );

    HOURS = (HRAMPM as f64);

    DPFMT(HOURS, b"0x", &mut H, ctx)?;

    fstr::assign(
        AMPM,
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(&fstr::concat(&fstr::concat(&H, b":"), &M), b":"),
                    &S,
                ),
                b" ",
            ),
            &AMORPM,
        ),
    );

    CHKOUT(b"ET2LST", ctx)?;
    Ok(())
}
