//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const CTRSIZ: i32 = 2;
const REF: &[u8] = b"J2000";
const SUN: i32 = 10;
const MAXL: i32 = 36;

struct SaveVars {
    SVCTR1: StackArray<i32, 2>,
    SVBODY: Vec<u8>,
    SVIDCD: i32,
    SVFND1: bool,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SVCTR1 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVBODY = vec![b' '; MAXL as usize];
        let mut SVIDCD: i32 = 0;
        let mut SVFND1: bool = false;
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            SVCTR1,
            SVBODY,
            SVIDCD,
            SVFND1,
            FIRST,
        }
    }
}

/// Longitude of the sun, planetocentric
///
/// Compute L_s, the planetocentric longitude of the sun, as seen
/// from a specified body.
///
/// # Required Reading
///
/// * [NAIF_IDS](crate::required_reading::naif_ids)
/// * [PCK](crate::required_reading::pck)
/// * [TIME](crate::required_reading::time)
/// * [SPK](crate::required_reading::spk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  BODY       I   Name of central body.
///  ET         I   Epoch in seconds past J2000 TDB.
///  ABCORR     I   Aberration correction.
///
///  The function returns the value of L_s for the specified body
///  at the specified time.
/// ```
///
/// # Detailed Input
///
/// ```text
///  BODY     is the name of the central body, typically a planet.
///
///  ET       is the epoch at which the longitude of the sun (L_s)
///           is to be computed. ET is expressed as seconds past
///           J2000 TDB (Barycentric Dynamical Time).
///
///  ABCORR   indicates the aberration corrections to be applied
///           when computing the longitude of the sun. ABCORR may
///           be any of the following.
///
///              'NONE'     Apply no correction.
///
///              'LT'       Correct the position of the sun,
///                         relative to the central body, for
///                         planetary (light time) aberration.
///
///              'LT+S'     Correct the position of the sun,
///                         relative to the central body, for
///                         planetary and stellar aberrations.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the planetocentric longitude of the Sun,
///  often called "L_s," for the specified body at the specified time.
///  This is the longitude of the body-Sun vector in a right-handed
///  frame whose basis vectors are defined as follows:
///
///  -  The positive Z direction is given by the instantaneous
///     angular velocity vector of the orbit of the body about
///     the Sun.
///
///  -  The positive X direction is that of the cross product of the
///     instantaneous north spin axis of the body with the positive
///     Z direction.
///
///  -  The positive Y direction is Z x X.
///
///  Units are radians; the range is 0 to 2*pi. Longitudes are
///  positive to the east.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input body name cannot be translated to an ID code,
///      and if the name is not a string representation of an integer
///      (for example, '399'), the error SPICE(NOTRANSLATION) is
///      signaled.
///
///  2)  If no SPK (ephemeris) file has been loaded prior to calling
///      this routine, or if the SPK data has insufficient coverage, an
///      error is signaled by a routine in the call
///      tree of this routine.
///
///  3)  If a PCK file containing rotational elements for the central
///      body has not been loaded prior to calling this routine, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  4)  If the instantaneous angular velocity and spin axis of BODY
///      are parallel, an error is signaled by a
///      routine in the call tree of this routine.
/// ```
///
/// # Files
///
/// ```text
///  Appropriate SPICE kernels must be loaded by the calling program
///  before this routine is called.
///
///  The following data are required:
///
///  -  An SPK file (or files) containing ephemeris data sufficient to
///     compute the geometric state of the central body relative to
///     the sun at ET must be loaded before this routine is called. If
///     light time correction is used, data must be available that
///     enable computation of the state the sun relative to the solar
///     system barycenter at the light-time corrected epoch. If
///     stellar aberration correction is used, data must be available
///     that enable computation of the state the central body relative
///     to the solar system barycenter at ET.
///
///  -  A PCK file containing rotational elements for the central body
///     must be loaded before this routine is called.
/// ```
///
/// # Particulars
///
/// ```text
///  The direction of the vernal equinox for the central body is
///  determined from the instantaneous equatorial and orbital planes
///  of the central body. This equinox definition is specified in
///  reference [1]. The "instantaneous orbital plane" is interpreted
///  in this routine as the plane normal to the cross product of the
///  position and velocity of the central body relative to the sun.
///  The geometric state of the central body relative to the sun is
///  used for this normal vector computation. The "instantaneous
///  equatorial plane" is normal to the central body's north pole
///  at the requested epoch. The pole direction is determined from
///  rotational elements loaded via a PCK file.
///
///  The result returned by this routine will depend on the
///  ephemeris data and rotational elements used. The result may
///  differ from that given in any particular version of the
///  Astronomical Almanac, due to differences in these input data,
///  and due to differences in precision of the computations.
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
///  1) A simple program that computes L_s for a body and time
///     supplied interactively. The geometric state of the Sun is
///     used.
///
///     Example code begins here.
///
///
///           PROGRAM LSPCN_EX1
///           IMPLICIT NONE
///
///           DOUBLE PRECISION      DPR
///           DOUBLE PRECISION      LSPCN
///
///           CHARACTER*(*)         ABCORR
///           PARAMETER           ( ABCORR = 'NONE' )
///
///           INTEGER               FILSIZ
///           PARAMETER           ( FILSIZ = 255 )
///
///           INTEGER               NAMLEN
///           PARAMETER           ( NAMLEN = 36 )
///
///           INTEGER               TIMLEN
///           PARAMETER           ( TIMLEN = 40 )
///
///           CHARACTER*(NAMLEN)    BODY
///           CHARACTER*(FILSIZ)    LSK
///           CHARACTER*(FILSIZ)    PCK
///           CHARACTER*(FILSIZ)    SPK
///           CHARACTER*(TIMLEN)    TIMSTR
///
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      LON
///
///
///           CALL PROMPT ( 'Enter name of LSK file > ', LSK )
///           CALL PROMPT ( 'Enter name of PCK file > ', PCK )
///           CALL PROMPT ( 'Enter name of SPK file > ', SPK )
///
///           CALL FURNSH ( LSK )
///           CALL FURNSH ( PCK )
///           CALL FURNSH ( SPK )
///
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Kernels have been loaded.'
///           WRITE (*,*) ' '
///
///           CALL PROMPT ( 'Enter name of central body       > ',
///          .               BODY                                  )
///           CALL PROMPT ( 'Enter calendar, JD, or DOY time  > ',
///          .               TIMSTR                                )
///
///           CALL STR2ET ( TIMSTR, ET )
///
///     C
///     C     Convert longitude to degrees.
///     C
///           LON = DPR() * LSPCN ( BODY, ET, ABCORR )
///
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Central body              = ',  BODY
///           WRITE (*,*) 'Time                      = ',  TIMSTR
///           WRITE (*,*) 'Planetocentric L_s (deg.) = ',  LON
///           WRITE (*,*) ' '
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, using the LSK file named naif0012.tls, the PCK file
///     named pck00010.tpc, the SPK file named de421.bsp; the 'EARTH'
///     as central body and the time string '2018 Mar 8  17:59 UTC',
///     the output was:
///
///
///     Enter name of LSK file > naif0012.tls
///     Enter name of PCK file > pck00010.tpc
///     Enter name of SPK file > de421.bsp
///
///      Kernels have been loaded.
///
///     Enter name of central body       > EARTH
///     Enter calendar, JD, or DOY time  > 2018 Mar 8  17:59 UTC
///
///      Central body              = EARTH
///      Time                      = 2018 Mar 8  17:59 UTC
///      Planetocentric L_s (deg.) =    348.11593978317080
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  "The Astronomical Almanac for the Year 2005," page L9,
///       United States Naval Observatory, U.S. Government Printing
///       Office, Washington, D.C., 2004.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.1, 25-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Removed WHILE
///         loop from example code, and added solution.
///
/// -    SPICELIB Version 1.1.0, 19-SEP-2013 (BVS)
///
///         Updated to save the input body name and ZZBODTRN state
///         counter and to do name-ID conversion only if the counter
///         has changed.
///
/// -    SPICELIB Version 1.0.0, 07-JAN-2005 (NJB)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.1.0, 19-SEP-2013 (BVS)
///
///         Updated to save the input body name and ZZBODTRN state
///         counter and to do name-ID conversion only if the counter
///         has changed.
/// ```
pub fn lspcn(ctx: &mut SpiceContext, body: &str, et: f64, abcorr: &str) -> crate::Result<f64> {
    let ret = LSPCN(body.as_bytes(), et, abcorr.as_bytes(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure LSPCN  ( Longitude of the sun, planetocentric )
pub fn LSPCN(BODY: &[u8], ET: f64, ABCORR: &[u8], ctx: &mut Context) -> f2rust_std::Result<f64> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut LSPCN: f64 = 0.0;
    let mut UAVEL = StackArray::<f64, 3>::new(1..=3);
    let mut LAT: f64 = 0.0;
    let mut LT: f64 = 0.0;
    let mut NPOLE = StackArray::<f64, 3>::new(1..=3);
    let mut POS = StackArray::<f64, 3>::new(1..=3);
    let mut RADIUS: f64 = 0.0;
    let mut BSTATE = StackArray::<f64, 6>::new(1..=6);
    let mut SSTATE = StackArray::<f64, 6>::new(1..=6);
    let mut TIPM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut TRANS = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut IDCODE: i32 = 0;
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
    // Give the function an initial value.
    //
    LSPCN = 0.0;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(LSPCN);
    }

    CHKIN(b"LSPCN", ctx)?;

    //
    // Initialization.
    //
    if save.FIRST {
        //
        // Initialize counters
        //
        ZZCTRUIN(save.SVCTR1.as_slice_mut(), ctx);

        save.FIRST = false;
    }

    //
    // Map the body name to an ID code.
    //
    ZZBODS2C(
        save.SVCTR1.as_slice_mut(),
        &mut save.SVBODY,
        &mut save.SVIDCD,
        &mut save.SVFND1,
        BODY,
        &mut IDCODE,
        &mut FOUND,
        ctx,
    )?;

    if !FOUND {
        SETMSG(b"The body name # could not be translated to a NAIF ID code.  The cause of this problem may be that you need an updated version of the SPICE Toolkit.", ctx);
        ERRCH(b"#", BODY, ctx);
        SIGERR(b"SPICE(NOTRANSLATION)", ctx)?;
        CHKOUT(b"LSPCN", ctx)?;
        return Ok(LSPCN);
    }

    //
    // Look up the direction of the North pole of the central body.
    // Note that TIPBOD does make use of binary PCK data if available.
    //
    TIPBOD(REF, IDCODE, ET, TIPM.as_slice_mut(), ctx)?;

    for I in 1..=3 {
        NPOLE[I] = TIPM[[3, I]];
    }

    //
    // Get the geometric state of the body relative to the sun.
    //
    SPKGEO(IDCODE, ET, REF, SUN, BSTATE.as_slice_mut(), &mut LT, ctx)?;

    //
    // Get the unit direction vector parallel to the angular velocity
    // vector of the orbit.  This is just the unitized cross product of
    // position and velocity.
    //
    UCRSS(BSTATE.as_slice(), BSTATE.subarray(4), UAVEL.as_slice_mut());

    //
    // We want to create a transformation matrix that maps vectors from
    // basis REF to the following frame:

    //    Z  =  UAVEL
    //
    //    X  =  NPOLE x UAVEL
    //
    //    Y  =  Z x X
    //
    // This is a "two-vector" frame with the unit orbital
    // angular velocity vector UAVEL as the primary vector and the
    // spin axis NPOLE as the secondary vector.  The primary
    // vector is associated with the +Z axis; the secondary vector
    // is associated with the +Y axis.
    //
    TWOVEC(
        UAVEL.as_slice(),
        3,
        NPOLE.as_slice(),
        2,
        TRANS.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"LSPCN", ctx)?;
        return Ok(LSPCN);
    }

    //
    // We'll find the position of the Sun relative to this frame.
    //
    // Get the state of the sun in frame REF.  Since we may be using
    // aberration corrections, this is not necessarily the negative of
    // the state we've just found.
    //
    SPKEZR(
        b"SUN",
        ET,
        REF,
        ABCORR,
        BODY,
        SSTATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    //
    // Now transform the position of the Sun into the "orbit plane
    // and equinox" frame.
    //
    MXV(TRANS.as_slice(), SSTATE.as_slice(), POS.as_slice_mut());

    //
    // Let RECRAD find the longitude LS for us.  RECRAD performs
    // the same coordinate transformation as the more commonly used
    // RECLAT, but the range of right ascension is 0:2*pi, which is
    // what we want for Ls.
    //
    RECRAD(POS.as_slice(), &mut RADIUS, &mut LSPCN, &mut LAT, ctx);

    CHKOUT(b"LSPCN", ctx)?;
    Ok(LSPCN)
}
