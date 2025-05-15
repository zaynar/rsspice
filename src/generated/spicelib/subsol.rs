//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const CTRSIZ: i32 = 2;
const FRNMLN: i32 = 80;
const MAXL: i32 = 36;

struct SaveVars {
    ORIGIN: StackArray<f64, 3>,
    SVCTR1: StackArray<i32, 2>,
    SVTARG: Vec<u8>,
    SVTCDE: i32,
    SVFND1: bool,
    SVCTR2: StackArray<i32, 2>,
    SVOBSR: Vec<u8>,
    SVOBSC: i32,
    SVFND2: bool,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ORIGIN = StackArray::<f64, 3>::new(1..=3);
        let mut SVCTR1 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVTARG = vec![b' '; MAXL as usize];
        let mut SVTCDE: i32 = 0;
        let mut SVFND1: bool = false;
        let mut SVCTR2 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVOBSR = vec![b' '; MAXL as usize];
        let mut SVOBSC: i32 = 0;
        let mut SVFND2: bool = false;
        let mut FIRST: bool = false;

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
        FIRST = true;

        Self {
            ORIGIN,
            SVCTR1,
            SVTARG,
            SVTCDE,
            SVFND1,
            SVCTR2,
            SVOBSR,
            SVOBSC,
            SVFND2,
            FIRST,
        }
    }
}

/// Sub-solar point
///
/// Deprecated: This routine has been superseded by the SPICELIB
/// routine SUBSLR. This routine is supported for purposes of
/// backward compatibility only.
///
/// Determine the coordinates of the sub-solar point on a target
/// body as seen by a specified observer at a specified epoch,
/// optionally corrected for planetary (light time) and stellar
/// aberration.
///
/// # Required Reading
///
/// * [FRAMES](crate::required_reading::frames)
/// * [PCK](crate::required_reading::pck)
/// * [SPK](crate::required_reading::spk)
/// * [TIME](crate::required_reading::time)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  METHOD     I   Computation method.
///  TARGET     I   Name of target body.
///  ET         I   Epoch in ephemeris seconds past J2000 TDB.
///  ABCORR     I   Aberration correction.
///  OBSRVR     I   Name of observing body.
///  SPOINT     O   Sub-solar point on the target body.
/// ```
///
/// # Detailed Input
///
/// ```text
///  METHOD   is a short string specifying the computation method
///           to be used. The choices are:
///
///              'Near point'       The sub-solar point is defined
///                                 as the nearest point on the
///                                 target to the sun.
///
///              'Intercept'        The sub-observer point is
///                                 defined as the target surface
///                                 intercept of the line
///                                 containing the target's center
///                                 and the sun's center.
///
///           In both cases, the intercept computation treats the
///           surface of the target body as a triaxial ellipsoid.
///           The ellipsoid's radii must be available in the kernel
///           pool.
///
///           Neither case nor white space are significant in
///           METHOD. For example, the string ' NEARPOINT' is
///           valid.
///
///
///  TARGET   is the name of the target body. TARGET is
///           case-insensitive, and leading and trailing blanks in
///           TARGET are not significant. Optionally, you may
///           supply a string containing the integer ID code for
///           the object. For example both 'MOON' and '301' are
///           legitimate strings that indicate the moon is the
///           target body.
///
///           This routine assumes that the target body is modeled
///           by a tri-axial ellipsoid, and that a PCK file
///           containing its radii has been loaded into the kernel
///           pool via FURNSH.
///
///
///  ET       is the epoch in ephemeris seconds past J2000 at which
///           the sub-solar point on the target body is to be
///           computed.
///
///
///  ABCORR   indicates the aberration corrections to be applied
///           when computing the observer-target state.  ABCORR
///           may be any of the following.
///
///              'NONE'     Apply no correction. Return the
///                         geometric sub-solar point on the target
///                         body.
///
///              'LT'       Correct for planetary (light time)
///                         aberration. Both the state and rotation
///                         of the target body are corrected for one
///                         way light time from target to observer.
///
///                         The state of the sun relative to the
///                         target is corrected for one way light
///                         from the sun to the target; this state
///                         is evaluated at the epoch obtained by
///                         retarding ET by the one way light time
///                         from target to observer.
///
///              'LT+S'     Correct for planetary (light time) and
///                         stellar aberrations. Light time
///                         corrections are the same as in the 'LT'
///                         case above. The target state is
///                         additionally corrected for stellar
///                         aberration as seen by the observer, and
///                         the sun state is corrected for stellar
///                         aberration as seen from the target.
///
///              'CN'       Converged Newtonian light time
///                         correction. In solving the light time
///                         equation, the 'CN' correction iterates
///                         until the solution converges (three
///                         iterations on all supported platforms).
///                         Whether the 'CN+S' solution is
///                         substantially more accurate than the
///                         'LT' solution depends on the geometry
///                         of the participating objects and on the
///                         accuracy of the input data. In all
///                         cases this routine will execute more
///                         slowly when a converged solution is
///                         computed. See the $Particulars section
///                         of SPKEZR for a discussion of precision
///                         of light time corrections. Light time
///                         corrections are applied as in the 'LT'
///                         case.
///
///              'CN+S'     Converged Newtonian light time
///                         corrections and stellar aberration
///                         correction. Light time and stellar
///                         aberration corrections are applied as
///                         in the 'LT+S' case.
///
///
///  OBSRVR   is the name of the observing body, typically a
///           spacecraft, the earth, or a surface point on the
///           earth. OBSRVR is case-insensitive, and leading and
///           trailing blanks in OBSRVR are not significant.
///           Optionally, you may supply a string containing the
///           integer ID code for the object. For example both
///           'EARTH' and '399' are legitimate strings that indicate
///           the earth is the observer.
/// ```
///
/// # Detailed Output
///
/// ```text
///  SPOINT   is the sub-solar point on the target body at ET
///           expressed relative to the body-fixed frame of the
///           target body.
///
///           The sub-solar point is defined either as the point on
///           the target body that is closest to the sun, or the
///           target surface intercept of the line containing the
///           target's center and the sun's center; the input
///           argument METHOD selects the definition to be used.
///
///           The body-fixed frame, which is time-dependent, is
///           evaluated at ET if ABCORR is 'NONE'; otherwise the
///           frame is evaluated at ET-LT, where LT is the one way
///           light time from target to observer.
///
///           The state of the target body is corrected for
///           aberration as specified by ABCORR; the corrected
///           state is used in the geometric computation. As
///           indicated above, the rotation of the target is
///           retarded by one way light time if ABCORR specifies
///           that light time correction is to be done.
///
///           The state of the sun as seen from the observing
///           body is also corrected for aberration as specified
///           by ABCORR. The corrections, when selected, are
///           applied at the epoch ET-LT, where LT is the one way
///           light time from target to observer.
/// ```
///
/// # Exceptions
///
/// ```text
///  If any of the listed errors occur, the output arguments are
///  left unchanged.
///
///  1)  If the input argument METHOD is not recognized, the error
///      SPICE(DUBIOUSMETHOD) is signaled.
///
///  2)  If either of the input body names TARGET or OBSRVR cannot be
///      mapped to NAIF integer codes, the error SPICE(IDCODENOTFOUND)
///      is signaled.
///
///  3)  If OBSRVR and TARGET map to the same NAIF integer ID codes,
///      the error SPICE(BODIESNOTDISTINCT) is signaled.
///
///  4)  If frame definition data enabling the evaluation of the state
///      of the target relative to the observer in target body-fixed
///      coordinates have not been loaded prior to calling SUBSOL, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  5)  If the specified aberration correction is not recognized, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  6)  If insufficient ephemeris data have been loaded prior to
///      calling SUBSOL, an error is signaled by a
///      routine in the call tree of this routine.
///
///  7)  If the triaxial radii of the target body have not been loaded
///      into the kernel pool prior to calling SUBSOL, an error is
///      signaled by a routine in the call tree of this routine.
///
///  8)  If the size of the TARGET body radii kernel variable is not
///      three, an error is signaled by a routine in the call tree of
///      this routine.
///
///  9)  If any of the three TARGET body radii is less-than or equal to
///      zero, an error is signaled by a routine in the call tree of
///      this routine.
///
///  10) If PCK data supplying a rotation model for the target body
///      have not been loaded prior to calling SUBSOL, an error is
///      signaled by a routine in the call tree of this routine.
/// ```
///
/// # Files
///
/// ```text
///  Appropriate SPK, PCK, and frame data must be available to
///  the calling program before this routine is called. Typically
///  the data are made available by loading kernels; however the
///  data may be supplied via subroutine interfaces if applicable.
///
///  The following data are required:
///
///  -  SPK data: ephemeris data for sun, target, and observer must
///     be loaded. If aberration corrections are used, the states of
///     sun, target, and observer relative to the solar system
///     barycenter must be calculable from the available ephemeris
///     data. Ephemeris data are made available by loading
///     one or more SPK files via FURNSH.
///
///  -  PCK data: triaxial radii for the target body must be loaded
///     into the kernel pool. Typically this is done by loading a
///     text PCK file via FURNSH.
///
///  -  Further PCK data:  a rotation model for the target body must
///     be loaded. This may be provided in a text or binary PCK
///     file which is loaded via FURNSH.
///
///  -  Frame data: if a frame definition is required to convert
///     the sun, observer, and target states to the body-fixed frame
///     of the target, that definition must be available in the
///     kernel pool. Typically the definition is supplied by loading
///     a frame kernel via FURNSH.
///
///  In all cases, kernel data are normally loaded once per program
///  run, NOT every time this routine is called.
/// ```
///
/// # Particulars
///
/// ```text
///  SUBSOL computes the sub-solar point on a target body, as seen by
///  a specified observer.
///
///  There are two different popular ways to define the sub-solar
///  point:  "nearest point on target to the sun" or "target surface
///  intercept of line containing target and sun." These coincide
///  when the target is spherical and generally are distinct otherwise.
///
///  When comparing sub-point computations with results from sources
///  other than SPICE, it's essential to make sure the same geometric
///  definitions are used.
/// ```
///
/// # Examples
///
/// ```text
///  In the following example program, the file MGS.BSP is a
///  hypothetical binary SPK ephemeris file containing data for the
///  Mars Global Surveyor orbiter. The SPK file de405s.bsp contains
///  data for the planet barycenters as well as the Earth, Moon, and
///  Sun for the time period including the date 1997 Dec 31 12:000
///  UTC. MGS0000A.TPC is a planetary constants kernel file
///  containing radii and rotation model constants.  MGS00001.TLS is
///  a leapseconds file. (File names shown here that are specific
///  to MGS are not names of actual files.)
///
///        IMPLICIT NONE
///
///        CHARACTER*25          METHOD ( 2 )
///
///        INTEGER               I
///
///        DOUBLE PRECISION      DPR
///        DOUBLE PRECISION      ET
///        DOUBLE PRECISION      LAT
///        DOUBLE PRECISION      LON
///        DOUBLE PRECISION      RADIUS
///        DOUBLE PRECISION      SPOINT ( 3 )
///
///        DATA                  METHOD / 'Intercept', 'Near point' /
///
///  C
///  C     Load kernel files.
///  C
///        CALL FURNSH ( 'MGS00001.TLS' )
///        CALL FURNSH ( 'MGS0000A.TPC' )
///        CALL FURNSH ( 'de405s.bsp'   )
///        CALL FURNSH ( 'MGS.BSP'      )
///
///  C
///  C     Convert the UTC request time to ET (seconds past
///  C     J2000, TDB).
///  C
///        CALL STR2ET ( '1997 Dec 31 12:00:00', ET )
///
///  C
///  C     Compute sub-spacecraft point using light time and stellar
///  C     aberration corrections. Use the "target surface intercept"
///  C     definition of sub-spacecraft point on the first loop
///  C     iteration, and use the "near point" definition on the
///  C     second.
///  C
///        DO I = 1, 2
///
///           CALL SUBSOL ( METHOD(I),
///       .                 'MARS',  ET,  'LT+S',  'MGS',  SPOINT )
///
///  C
///  C        Convert rectangular coordinates to planetocentric
///  C        latitude and longitude. Convert radians to degrees.
///  C
///           CALL RECLAT ( SPOINT, RADIUS, LON, LAT  )
///
///           LON = LON * DPR ()
///           LAT = LAT * DPR ()
///
///  C
///  C        Write the results.
///  C
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Computation method: ', METHOD(I)
///           WRITE (*,*) ' '
///           WRITE (*,*) '  Radius                   (km)  = ', RADIUS
///           WRITE (*,*) '  Planetocentric Latitude  (deg) = ', LAT
///           WRITE (*,*) '  Planetocentric Longitude (deg) = ', LON
///           WRITE (*,*) ' '
///
///        END DO
///
///        END
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The appropriate kernel data must have been loaded before this
///      routine is called. See the $Files section above.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  J.E. McLean        (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.4.0, 01-NOV-2021 (EDW) (JDR)
///
///         Body radii accessed from kernel pool using ZZGFTREB.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.3.0, 04-JUL-2014 (NJB) (BVS)
///
///         Discussion of light time corrections was updated. Assertions
///         that converged light time corrections are unlikely to be
///         useful were removed.
///
///      Last update was 19-SEP-2013 (BVS)
///
///         Updated to save the input body names and ZZBODTRN state
///         counters and to do name-ID conversions only if the counters
///         have changed.
///
/// -    SPICELIB Version 1.2.3, 18-MAY-2010 (BVS)
///
///         Index line now states that this routine is deprecated.
///
/// -    SPICELIB Version 1.2.2, 17-MAR-2009 (EDW)
///
///         Typo correction in $Required_Reading, changed
///         FRAME to FRAMES.
///
/// -    SPICELIB Version 1.2.1, 07-FEB-2008 (NJB)
///
///         $Abstract now states that this routine is deprecated.
///
/// -    SPICELIB Version 1.2.0, 24-OCT-2005 (NJB)
///
///         Call to BODVAR was replaced with call to BODVCD.
///
/// -    SPICELIB Version 1.1.0, 22-JUL-2004 (NJB)
///
///         Updated to support representations of integers in the input
///         arguments TARGET and OBSRVR. Deleted references in header to
///         kernel-specific loaders. Made miscellaneous minor corrections
///         to header comments.
///
/// -    SPICELIB Version 1.0.2, 12-DEC-2002 (NJB)
///
///         Corrected and updated code example in header.
///
/// -    SPICELIB Version 1.0.1, 01-NOV-1999 (WLT)
///
///         Declared routine LTIME to be external.
///
/// -    SPICELIB Version 1.0.0, 03-SEP-1999 (NJB) (JEM)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.1.0, 22-JUL-2004 (NJB)
///
///         Updated to support representations of integers in the
///         input arguments TARGET and OBSRVR: calls to BODN2C
///         were replaced by calls to BODS2C.
/// ```
pub fn subsol(
    ctx: &mut SpiceContext,
    method: &str,
    target: &str,
    et: f64,
    abcorr: &str,
    obsrvr: &str,
    spoint: &mut [f64; 3],
) -> crate::Result<()> {
    SUBSOL(
        method.as_bytes(),
        target.as_bytes(),
        et,
        abcorr.as_bytes(),
        obsrvr.as_bytes(),
        spoint,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SUBSOL ( Sub-solar point )
pub fn SUBSOL(
    METHOD: &[u8],
    TARGET: &[u8],
    ET: f64,
    ABCORR: &[u8],
    OBSRVR: &[u8],
    SPOINT: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut SPOINT = DummyArrayMut::new(SPOINT, 1..=3);
    let mut FRNAME = [b' '; FRNMLN as usize];
    let mut ALT: f64 = 0.0;
    let mut ETTARG: f64 = 0.0;
    let mut LT: f64 = 0.0;
    let mut POS = StackArray::<f64, 3>::new(1..=3);
    let mut RADII = StackArray::<f64, 3>::new(1..=3);
    let mut SUNLT: f64 = 0.0;
    let mut FRCODE: i32 = 0;
    let mut OBSCDE: i32 = 0;
    let mut TRGCDE: i32 = 0;
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
    // Saved variables
    //

    //
    // Saved name/ID items.
    //

    //
    // Initial values
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
        CHKIN(b"SUBSOL", ctx)?;
    }

    //
    // Initialization.
    //
    if save.FIRST {
        //
        // Initialize counters.
        //
        ZZCTRUIN(save.SVCTR1.as_slice_mut(), ctx);
        ZZCTRUIN(save.SVCTR2.as_slice_mut(), ctx);

        save.FIRST = false;
    }

    //
    // Obtain integer codes for the target and observer.
    //
    ZZBODS2C(
        save.SVCTR1.as_slice_mut(),
        &mut save.SVTARG,
        &mut save.SVTCDE,
        &mut save.SVFND1,
        TARGET,
        &mut TRGCDE,
        &mut FOUND,
        ctx,
    )?;

    if !FOUND {
        SETMSG(b"The target, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE Toolkit. ", ctx);
        ERRCH(b"#", TARGET, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(b"SUBSOL", ctx)?;
        return Ok(());
    }

    ZZBODS2C(
        save.SVCTR2.as_slice_mut(),
        &mut save.SVOBSR,
        &mut save.SVOBSC,
        &mut save.SVFND2,
        OBSRVR,
        &mut OBSCDE,
        &mut FOUND,
        ctx,
    )?;

    if !FOUND {
        SETMSG(b"The observer, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE Toolkit. ", ctx);
        ERRCH(b"#", OBSRVR, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(b"SUBSOL", ctx)?;
        return Ok(());
    }

    //
    // Check the input body codes.  If they are equal, signal
    // an error.
    //
    if (OBSCDE == TRGCDE) {
        SETMSG(b"In computing the sub-observer point, the observing body and target body are the same. Both are #.", ctx);
        ERRCH(b"#", OBSRVR, ctx);
        SIGERR(b"SPICE(BODIESNOTDISTINCT)", ctx)?;
        CHKOUT(b"SUBSOL", ctx)?;
        return Ok(());
    }

    //
    // Get the radii of the target body from the kernel pool.
    //
    ZZGFTREB(TRGCDE, RADII.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SUBSOL", ctx)?;
        return Ok(());
    }

    //
    // Find the name of the body-fixed frame associated with the
    // target body.  We'll want the state of the target relative to
    // the observer in this body-fixed frame.
    //
    CIDFRM(TRGCDE, &mut FRCODE, &mut FRNAME, &mut FOUND, ctx)?;

    if !FOUND {
        SETMSG(b"No body-fixed frame is associated with target body #; a frame kernel must be loaded to make this association.  Consult the FRAMES Required Reading for details.", ctx);
        ERRCH(b"#", TARGET, ctx);
        SIGERR(b"SPICE(NOFRAME)", ctx)?;
        CHKOUT(b"SUBSOL", ctx)?;
        return Ok(());
    }

    //
    // If we're using aberration corrections, we'll need the
    // one way light time from the target to the observer.  Otherwise,
    // we set the time time to zero.
    //
    if EQSTR(ABCORR, b"NONE") {
        LT = 0.0;
        ETTARG = ET;
    } else {
        LTIME(ET, OBSCDE, b"<-", TRGCDE, &mut ETTARG, &mut LT, ctx)?;
    }

    //
    // Determine the position of the sun in target body-fixed
    // coordinates.
    //
    // Call SPKEZ to compute the position of the sun as seen from the
    // target body and the light time between them SUNLT.  This state is
    // evaluated at the target epoch ETTARG. We request that the
    // coordinates of the target-sun position vector POS be returned
    // relative to the body fixed reference frame associated with the
    // target body, using aberration corrections specified by the input
    // argument ABCORR.
    //
    SPKPOS(
        b"SUN",
        ETTARG,
        &FRNAME,
        ABCORR,
        TARGET,
        POS.as_slice_mut(),
        &mut SUNLT,
        ctx,
    )?;

    //
    // Find the sub-solar point using the specified geometric definition.
    //
    if EQSTR(METHOD, b"Near point") {
        //
        // Locate the nearest point to the sun on the target.
        //
        NEARPT(
            POS.as_slice(),
            RADII[1],
            RADII[2],
            RADII[3],
            SPOINT.as_slice_mut(),
            &mut ALT,
            ctx,
        )?;
    } else if EQSTR(METHOD, b"Intercept") {
        SURFPT(
            save.ORIGIN.as_slice(),
            POS.as_slice(),
            RADII[1],
            RADII[2],
            RADII[3],
            SPOINT.as_slice_mut(),
            &mut FOUND,
            ctx,
        )?;

        //
        // Since the line in question passes through the center of the
        // target, there will always be a surface intercept.  So we should
        // never have FOUND = .FALSE.
        //
        if !FOUND {
            SETMSG(b"Call to SURFPT returned FOUND=FALSE even though vertex of ray is at target center. This indicates a bug. Please contact NAIF.", ctx);
            SIGERR(b"SPICE(BUG)", ctx)?;
            CHKOUT(b"SUBSOL", ctx)?;
            return Ok(());
        }
    } else {
        SETMSG(b"The computation method # was not recognized. Allowed values are \"Near point\" and \"Intercept.\"", ctx);
        ERRCH(b"#", METHOD, ctx);
        SIGERR(b"SPICE(DUBIOUSMETHOD)", ctx)?;
        CHKOUT(b"SUBSOL", ctx)?;
        return Ok(());
    }

    CHKOUT(b"SUBSOL", ctx)?;
    Ok(())
}
