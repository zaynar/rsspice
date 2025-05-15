//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const CTRSIZ: i32 = 2;
const SUN: i32 = 10;
const FRNMLN: i32 = 80;
const MAXL: i32 = 36;

struct SaveVars {
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
        let mut SVCTR1 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVTARG = vec![b' '; MAXL as usize];
        let mut SVTCDE: i32 = 0;
        let mut SVFND1: bool = false;
        let mut SVCTR2 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVOBSR = vec![b' '; MAXL as usize];
        let mut SVOBSC: i32 = 0;
        let mut SVFND2: bool = false;
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
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

/// Illumination angles
///
/// Deprecated: This routine has been superseded by the SPICELIB
/// routine ILUMIN. This routine is supported for purposes of
/// backward compatibility only.
///
/// Find the illumination angles at a specified surface point of a
/// target body.
///
/// # Required Reading
///
/// * [KERNEL](crate::required_reading::kernel)
/// * [NAIF_IDS](crate::required_reading::naif_ids)
/// * [SPK](crate::required_reading::spk)
/// * [TIME](crate::required_reading::time)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  TARGET     I   Name of target body.
///  ET         I   Epoch in ephemeris seconds past J2000.
///  ABCORR     I   Desired aberration correction.
///  OBSRVR     I   Name of observing body.
///  SPOINT     I   Body-fixed coordinates of a target surface point.
///  PHASE      O   Phase angle at the surface point.
///  SOLAR      O   Solar incidence angle at the surface point.
///  EMISSN     O   Emission angle at the surface point.
/// ```
///
/// # Detailed Input
///
/// ```text
///  TARGET   is the name of the target body. TARGET is
///           case-insensitive, and leading and trailing blanks
///           in TARGET are not significant. Optionally, you may
///           supply a string containing the integer ID code for
///           the object. For example both 'MOON' and '301' are
///           legitimate strings that indicate the moon is the
///           target body.
///
///  ET       is the epoch, specified in ephemeris seconds past
///           J2000, at which the apparent illumination angles at
///           the specified surface point on the target body, as
///           seen from the observing body, are to be computed.
///
///  ABCORR   is the aberration correction to be used in
///           computing the location and orientation of the
///           target body and the location of the Sun. Possible
///           values are:
///
///              'NONE'        No aberration correction.
///
///              'LT'          Correct the position and
///                            orientation of target body for
///                            light time, and correct the
///                            position of the Sun for light
///                            time.
///
///              'LT+S'        Correct the observer-target vector
///                            for light time and stellar
///                            aberration, correct the
///                            orientation of the target body
///                            for light time, and correct the
///                            target-Sun vector for light time
///                            and stellar aberration.
///
///              'CN'          Converged Newtonian light time
///                            correction. In solving the light
///                            time equation, the 'CN'
///                            correction iterates until the
///                            solution converges (three
///                            iterations on all supported
///                            platforms). Whether the 'CN+S'
///                            solution is substantially more
///                            accurate than the 'LT' solution
///                            depends on the geometry of the
///                            participating objects and on the
///                            accuracy of the input data. In
///                            all cases this routine will
///                            execute more slowly when a
///                            converged solution is computed.
///                            See the $Particulars section of
///                            SPKEZR for a discussion of
///                            precision of light time
///                            corrections.
///
///                            Both the state and rotation of
///                            the target body are corrected for
///                            light time.
///
///                 'CN+S'     Converged Newtonian light time
///                            correction and stellar aberration
///                            correction.
///
///                            Both the state and rotation of
///                            the target body are corrected for
///                            light time.
///
///  OBSRVR   is the name of the observing body, typically a
///           spacecraft, the earth, or a surface point on the
///           earth. OBSRVR is case-insensitive, and leading
///           and trailing blanks in OBSRVR are not significant.
///           Optionally, you may supply a string containing the
///           integer ID code for the object. For example both
///           'EARTH' and '399' are legitimate strings that
///           indicate the earth is the observer.
///
///           OBSRVR may be not be identical to TARGET.
///
///  SPOINT   is a surface point on the target body, expressed
///           in rectangular body-fixed (body equator and prime
///           meridian) coordinates. SPOINT need not be visible
///           from the observer's location at time ET.
/// ```
///
/// # Detailed Output
///
/// ```text
///  PHASE    is the phase angle at SPOINT, as seen from OBSRVR
///           at time ET. This is the angle between the
///           SPOINT-OBSRVR vector and the SPOINT-Sun vector.
///           Units are radians. The range of  PHASE is [0, pi].
///           See $Particulars below for a detailed discussion of
///           the definition.
///
///  SOLAR    is the solar incidence angle at SPOINT, as seen
///           from OBSRVR at time ET. This is the angle
///           between the surface normal vector at SPOINT and the
///           SPOINT-Sun vector. Units are radians. The range
///           of SOLAR is [0, pi]. See $Particulars below for a
///           detailed discussion of the definition.
///
///  EMISSN   is the emission angle at SPOINT, as seen from
///           OBSRVR at time ET. This is the angle between the
///           surface normal vector at SPOINT and the
///           SPOINT-observer vector. Units are radians. The
///           range of EMISSN is [0, pi]. See $Particulars below
///           for a detailed discussion of the definition.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If TARGET and OBSRVR are not distinct, the error
///      SPICE(BODIESNOTDISTINCT) is signaled.
///
///  2)  If no SPK (ephemeris) data are available for the observer,
///      target, and Sun at the time specified by ET, an error is
///      signaled by a routine in the call tree of this routine. If
///      light time corrections are used, SPK data for the target body
///      must be available at the time ET - LT, where LT is the one-way
///      light time from the target to the observer at ET.
///      Additionally, SPK data must be available for the Sun at the
///      time ET - LT - LT2, where LT2 is the light time from the Sun
///      to the target body at time ET - LT.
///
///  3)  If PCK data defining the orientation or shape of the target
///      body are unavailable, an error is signaled by a routine in the
///      call tree of this routine.
///
///  4)  If no body-fixed frame is associated with the target body,
///      the error SPICE(NOFRAME) is signaled.
///
///  5)  If name of target or observer cannot be translated to its
///      NAIF ID code, the error SPICE(IDCODENOTFOUND) is signaled.
///
///  6)  If radii for TARGET are not found in the kernel pool, an error
///      is signaled by a routine in the call tree of this routine.
///
///  7)  If the size of the TARGET body radii kernel variable is not
///      three, an error is signaled by a routine in the call tree of
///      this routine.
///
///  8)  If any of the three TARGET body radii is less-than or equal to
///      zero, an error is signaled by a routine in the call tree of
///      this routine.
/// ```
///
/// # Files
///
/// ```text
///  No files are input to this routine. However, ILLUM expects
///  that the appropriate SPK and PCK files have been loaded via
///  FURNSH.
/// ```
///
/// # Particulars
///
/// ```text
///  The term "illumination angles" refers to following set of
///  angles:
///
///
///     solar incidence angle    Angle between the surface normal at
///                              the specified surface point and the
///                              vector from the surface point to the
///                              Sun.
///
///     emission angle           Angle between the surface normal at
///                              the specified surface point and the
///                              vector from the surface point to the
///                              observer.
///
///     phase angle              Angle between the vectors from the
///                              surface point to the observing body's
///                              location and from the surface point
///                              to the Sun.
///
///
///  The diagram below illustrates the geometrical relationships
///  defining these angles. The labels for the solar incidence,
///  emission, and phase angles are "s.i.", "e.", and "phase".
///
///
///                                                   *
///                                                  Sun
///
///                 surface normal vector
///                           ._                 _.
///                           |\                 /|  Sun vector
///                             \    phase      /
///                              \   .    .    /
///                              .            .
///                                \   ___   /
///                           .     \/     \/
///                                 _\ s.i./
///                          .    /   \   /
///                          .   |  e. \ /
///      *             <--------------- *  surface point on
///   viewing            vector            target body
///   location           to viewing
///   (observer)         location
///
///
///  Note that if the target-observer vector, the target normal vector
///  at the surface point, and the target-sun vector are coplanar,
///  then phase is the sum of incidence and emission. This is rarely
///  true; usually
///
///     phase angle  <  solar incidence angle + emission angle
///
///  All of the above angles can be computed using light time
///  corrections, light time and stellar aberration corrections, or
///  no aberration corrections. The way aberration corrections
///  are used is described below.
///
///  Care must be used in computing light time corrections. The
///  guiding principle used here is "describe what appears in
///  an image." We ignore differential light time; the light times
///  from all points on the target to the observer are presumed to be
///  equal.
///
///
///     Observer-target body vector
///     ---------------------------
///
///     Let ET be the epoch at which an observation or remote
///     sensing measurement is made, and let ET - LT ("LT" stands
///     for "light time") be the epoch at which the photons received
///     at ET were emitted from the body (we use the term "emitted"
///     loosely here).
///
///     The correct observer-target vector points from the observer's
///     location at ET to the target body's location at ET - LT.
///     The target-observer vector points in the opposite direction.
///
///     Since light time corrections are not symmetric, the correct
///     target-observer vector CANNOT be found by computing the light
///     time corrected position of the observer as seen from the
///     target body.
///
///
///     Target body's orientation
///     -------------------------
///
///     Using the definitions of ET and LT above, the target
///     body's orientation at ET - LT is used. The surface
///     normal is dependent on the target body's orientation, so
///     the body's orientation model must be evaluated for the correct
///     epoch.
///
///
///     Target body -- Sun vector
///     -------------------------
///
///     All surface features on the target body will appear in
///     a measurement made at ET as they were at ET-LT. In
///     particular, lighting on the target body is dependent on
///     the apparent location of the Sun as seen from the target
///     body at ET-LT. So, a second light time correction is used
///     in finding the apparent location of the Sun.
///
///
///  Stellar aberration corrections, when used, are applied as follows:
///
///
///     Observer-target body vector
///     ---------------------------
///
///     In addition to light time correction, stellar aberration is
///     used in computing the apparent target body position as seen
///     from the observer's location at time ET. This apparent
///     position defines the observer-target body vector.
///
///
///     Target body-Sun vector
///     ----------------------
///
///     The target body-Sun vector is the apparent position of the Sun,
///     corrected for light time and stellar aberration, as seen from
///     the target body at time ET-LT. Note that the target body's
///     position is not affected by the stellar aberration correction
///     applied in finding its apparent position as seen by the
///     observer.
///
///
///  Once all of the vectors, as well as the target body's
///  orientation, have been computed with the proper aberration
///  corrections, the element of time is eliminated from the
///  computation. The problem becomes a purely geometrical one,
///  and is described by the diagram above.
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
///  1) Find the phase, solar incidence, and emission angles at the
///     sub-solar and sub-spacecraft points on Mars as seen from the
///     Mars Global Surveyor spacecraft at a specified UTC time.
///     Use light time and stellar aberration corrections.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File: illum_ex1.tm
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
///           de430.bsp                        Planetary ephemeris
///           mar097.bsp                       Mars satellite ephemeris
///           pck00010.tpc                     Planet orientation and
///                                            radii
///           naif0011.tls                     Leapseconds
///           mgs_ext12_ipng_mgs95j.bsp        MGS ephemeris
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de430.bsp',
///                               'mar097.bsp',
///                               'pck00010.tpc',
///                               'naif0011.tls',
///                               'mgs_ext12_ipng_mgs95j.bsp' )
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM ILLUM_EX1
///           IMPLICIT NONE
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      DPR
///
///     C
///     C     Local parameters
///     C
///           INTEGER               NAMLEN
///           PARAMETER           ( NAMLEN = 32 )
///
///           INTEGER               TIMLEN
///           PARAMETER           ( TIMLEN = 25 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(NAMLEN)    OBSRVR
///           CHARACTER*(NAMLEN)    TARGET
///           CHARACTER*(TIMLEN)    UTC
///
///           DOUBLE PRECISION      ALT
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      SSCEMI
///           DOUBLE PRECISION      SSCPHS
///           DOUBLE PRECISION      SSCSOL
///           DOUBLE PRECISION      SSLEMI
///           DOUBLE PRECISION      SSLPHS
///           DOUBLE PRECISION      SSLSOL
///           DOUBLE PRECISION      SSOLPT ( 3 )
///           DOUBLE PRECISION      SSCPT  ( 3 )
///
///     C
///     C     Load kernel files.
///     C
///           CALL FURNSH ( 'illum_ex1.tm' )
///
///     C
///     C     Convert our UTC time to ephemeris seconds past J2000.
///     C
///           UTC = '2003 OCT 13 06:00:00'
///           CALL UTC2ET ( UTC, ET )
///
///     C
///     C     Assign observer and target names.  The acronym MGS
///     C     indicates Mars Global Surveyor.  See NAIF_IDS for a
///     C     list of names recognized by SPICE.
///     C
///           TARGET = 'Mars'
///           OBSRVR = 'MGS'
///
///     C
///     C     Find the sub-solar point on the Earth as seen from
///     C     the MGS spacecraft at ET.  Use the "surface intercept"
///     C     style of sub-point definition. This makes it easy
///     C     to verify the solar incidence angle.
///     C
///           CALL SUBSOL ( 'Near point', TARGET,  ET,
///          .              'LT+S',       OBSRVR,  SSOLPT  )
///
///     C
///     C     Now find the sub-spacecraft point.  Use the
///     C     "nearest point" definition of the sub-point
///     C     here---this makes it easy to verify the emission angle.
///     C
///           CALL SUBPT ( 'Near point',  TARGET,  ET,
///          .             'LT+S',        OBSRVR,  SSCPT,  ALT )
///
///     C
///     C     Find the phase, solar incidence, and emission
///     C     angles at the sub-solar point on the Earth as seen
///     C     from Mars Global Surveyor at time ET.
///     C
///           CALL ILLUM ( TARGET, ET,     'LT+S', OBSRVR,
///          .             SSOLPT, SSLPHS, SSLSOL, SSLEMI )
///
///     C
///     C     Do the same for the sub-spacecraft point.
///     C
///           CALL ILLUM ( TARGET, ET,     'LT+S', OBSRVR,
///          .             SSCPT,  SSCPHS, SSCSOL, SSCEMI )
///
///     C
///     C     Convert the angles to degrees and write them out.
///     C
///           SSLPHS = DPR() * SSLPHS
///           SSLSOL = DPR() * SSLSOL
///           SSLEMI = DPR() * SSLEMI
///
///           SSCPHS = DPR() * SSCPHS
///           SSCSOL = DPR() * SSCSOL
///           SSCEMI = DPR() * SSCEMI
///
///           WRITE (*,*) ' '
///           WRITE (*,*) 'UTC epoch is ', UTC
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Illumination angles at the sub-solar point:'
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Phase angle           (deg.): ', SSLPHS
///           WRITE (*,*) 'Solar incidence angle (deg.): ', SSLSOL
///           WRITE (*,*) 'Emission angle        (deg.): ', SSLEMI
///           WRITE (*,*) ' '
///           WRITE (*,*) 'The solar incidence angle should be 0.'
///           WRITE (*,*) 'The emission and phase angles should '//
///          .            'should be equal.'
///
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Illumination angles at the sub-s/c point:'
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Phase angle           (deg.): ', SSCPHS
///           WRITE (*,*) 'Solar incidence angle (deg.): ', SSCSOL
///           WRITE (*,*) 'Emission angle        (deg.): ', SSCEMI
///           WRITE (*,*) ' '
///           WRITE (*,*) 'The emission angle should be 0.'
///           WRITE (*,*) 'The solar incidence and phase angles '//
///          .            'should be equal.'
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      UTC epoch is 2003 OCT 13 06:00:00
///
///      Illumination angles at the sub-solar point:
///
///      Phase angle           (deg.):    138.36942994721122
///      Solar incidence angle (deg.):    7.1119364739076138E-015
///      Emission angle        (deg.):    138.36942994721122
///
///      The solar incidence angle should be 0.
///      The emission and phase angles should should be equal.
///
///      Illumination angles at the sub-s/c point:
///
///      Phase angle           (deg.):    101.43985783237235
///      Solar incidence angle (deg.):    101.43985783237240
///      Emission angle        (deg.):    5.0087425211137720E-014
///
///      The emission angle should be 0.
///      The solar incidence and phase angles should be equal.
/// ```
///
/// # Author and Institution
///
/// ```text
///  C.H. Acton         (JPL)
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
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
///         Edited the header to comply with NAIF standard. Modified
///         code example to use meta-kernel.
///
///         Removed unnecessary $Revisions section.
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
/// -    SPICELIB Version 1.2.2, 18-MAY-2010 (BVS)
///
///         Index lines now state that this routine is deprecated.
///
/// -    SPICELIB Version 1.2.1, 07-FEB-2008 (NJB)
///
///         $Abstract now states that this routine is deprecated.
///
/// -    SPICELIB Version 1.2.0, 23-OCT-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in VSUB calls. Replaced call to BODVAR with call to BODVCD.
///
/// -    SPICELIB Version 1.1.0, 22-JUL-2004 (NJB)
///
///         Updated to support representations of integers in the input
///         arguments TARGET and OBSRVR: calls to BODN2C were replaced by
///         calls to BODS2C.
///
/// -    SPICELIB Version 1.0.2, 27-JUL-2003 (NJB) (CHA)
///
///         Various header corrections were made. The example program
///         was upgraded to use real kernels, and the program's output is
///         shown.
///
/// -    SPICELIB Version 1.0.1, 10-JUL-2002 (NJB)
///
///         Updated $Index_Entries header section.
///
/// -    SPICELIB Version 1.0.0, 21-MAR-1999 (NJB)
///
///         Adapted from the MGSSPICE version dated 10-MAR-1992.
/// ```
pub fn illum(
    ctx: &mut SpiceContext,
    target: &str,
    et: f64,
    abcorr: &str,
    obsrvr: &str,
    spoint: &[f64; 3],
    phase: &mut f64,
    solar: &mut f64,
    emissn: &mut f64,
) -> crate::Result<()> {
    ILLUM(
        target.as_bytes(),
        et,
        abcorr.as_bytes(),
        obsrvr.as_bytes(),
        spoint,
        phase,
        solar,
        emissn,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure ILLUM ( Illumination angles )
pub fn ILLUM(
    TARGET: &[u8],
    ET: f64,
    ABCORR: &[u8],
    OBSRVR: &[u8],
    SPOINT: &[f64],
    PHASE: &mut f64,
    SOLAR: &mut f64,
    EMISSN: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let SPOINT = DummyArray::new(SPOINT, 1..=3);
    let mut FRNAME = [b' '; FRNMLN as usize];
    let mut LT: f64 = 0.0;
    let mut LTS: f64 = 0.0;
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
    let mut OBSVEC = StackArray::<f64, 3>::new(1..=3);
    let mut OFFOBS = StackArray::<f64, 3>::new(1..=3);
    let mut OFFSUN = StackArray::<f64, 3>::new(1..=3);
    let mut RADII = StackArray::<f64, 3>::new(1..=3);
    let mut SSTATE = StackArray::<f64, 6>::new(1..=6);
    let mut SUNVEC = StackArray::<f64, 3>::new(1..=3);
    let mut TEPOCH: f64 = 0.0;
    let mut TSTATE = StackArray::<f64, 6>::new(1..=6);
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
        CHKIN(b"ILLUM", ctx)?;
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
        CHKOUT(b"ILLUM", ctx)?;
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
        CHKOUT(b"ILLUM", ctx)?;
        return Ok(());
    }

    //
    // The observer and target must be distinct.
    //
    if (TRGCDE == OBSCDE) {
        SETMSG(b"Target is #; observer is #.", ctx);
        ERRCH(b"#", TARGET, ctx);
        ERRCH(b"#", OBSRVR, ctx);
        SIGERR(b"SPICE(BODIESNOTDISTINCT)", ctx)?;
        CHKOUT(b"ILLUM", ctx)?;
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
        CHKOUT(b"ILLUM", ctx)?;
        return Ok(());
    }

    //
    // Find the body-fixed state of the target as seen from the observer
    // at ET.  The appropriate aberration corrections will be used in
    // evaluating this state.
    //
    SPKEZ(
        TRGCDE,
        ET,
        &FRNAME,
        ABCORR,
        OBSCDE,
        TSTATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    //
    // Determine the epoch to be used in computing the target-Sun vector.
    //
    if EQSTR(ABCORR, b"NONE") {
        TEPOCH = ET;
    } else {
        TEPOCH = (ET - LT);
    }

    //
    // Find the body-fixed state of the Sun as seen from the target at
    // TEPOCH.
    //
    SPKEZ(
        SUN,
        TEPOCH,
        &FRNAME,
        ABCORR,
        TRGCDE,
        SSTATE.as_slice_mut(),
        &mut LTS,
        ctx,
    )?;

    //
    // Grab the position portions of the states (the first three
    // elements of each state).  Negate the observer-target vector,
    // since the vector required for the illumination angle
    // computation is the target-observer vector.  The vectors we've
    // found point from the target body center to the observer and
    // Sun, and already take light time corrections into account.
    //
    VMINUS(TSTATE.as_slice(), OBSVEC.as_slice_mut());
    VEQU(SSTATE.as_slice(), SUNVEC.as_slice_mut());

    //
    // Now we'll modify target-observer and target-Sun vectors to
    // take into account the offset between the target center and the
    // surface point of interest; we want the vectors to point from
    // the surface point to the observer and Sun respectively.
    //
    VSUB(OBSVEC.as_slice(), SPOINT.as_slice(), OFFOBS.as_slice_mut());
    VSUB(SUNVEC.as_slice(), SPOINT.as_slice(), OFFSUN.as_slice_mut());

    //
    // Find the surface normal at SPOINT.  We'll need the radii of the
    // target body.
    //
    ZZGFTREB(TRGCDE, RADII.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ILLUM", ctx)?;
        return Ok(());
    }

    SURFNM(
        RADII[1],
        RADII[2],
        RADII[3],
        SPOINT.as_slice(),
        NORMAL.as_slice_mut(),
        ctx,
    )?;

    //
    // Find the illumination angles.  VSEP will give us angular
    // separation in radians.
    //
    *PHASE = VSEP(OFFSUN.as_slice(), OFFOBS.as_slice(), ctx);
    *SOLAR = VSEP(NORMAL.as_slice(), OFFSUN.as_slice(), ctx);
    *EMISSN = VSEP(NORMAL.as_slice(), OFFOBS.as_slice(), ctx);

    CHKOUT(b"ILLUM", ctx)?;
    Ok(())
}
