//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const CNVTOL: f64 = 0.000001;
const NWMAX: i32 = 15;
const NWDIST: i32 = 5;
const NWSEP: i32 = 5;
const NWRR: i32 = 5;
const NWUDS: i32 = 5;
const NWPA: i32 = 5;
const NWILUM: i32 = 5;
const ADDWIN: f64 = 0.5;
const FRMNLN: i32 = 32;
const FOVTLN: i32 = 40;
const FTCIRC: &[u8] = b"CIRCLE";
const FTELLI: &[u8] = b"ELLIPSE";
const FTPOLY: &[u8] = b"POLYGON";
const FTRECT: &[u8] = b"RECTANGLE";
const ANNULR: &[u8] = b"ANNULAR";
const ANY: &[u8] = b"ANY";
const PARTL: &[u8] = b"PARTIAL";
const FULL: &[u8] = b"FULL";
const DSSHAP: &[u8] = b"DSK";
const EDSHAP: &[u8] = b"ELLIPSOID";
const PTSHAP: &[u8] = b"POINT";
const RYSHAP: &[u8] = b"RAY";
const SPSHAP: &[u8] = b"SPHERE";
const NOCTYP: i32 = 4;
const OCLLN: i32 = 7;
const SHPLEN: i32 = 9;
const MAXVRT: i32 = 10000;
const CIRFOV: &[u8] = b"CIRCLE";
const ELLFOV: &[u8] = b"ELLIPSE";
const POLFOV: &[u8] = b"POLYGON";
const RECFOV: &[u8] = b"RECTANGLE";

struct SaveVars {
    RAYDIR: StackArray<f64, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut RAYDIR = StackArray::<f64, 3>::new(1..=3);

        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::D(0.0), 3 as usize))
                .chain([]);

            RAYDIR
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { RAYDIR }
    }
}

/// Is target in FOV at time?
///
/// Determine if a specified ephemeris object is within the
/// field-of-view (FOV) of a specified instrument at a given time.
///
/// # Required Reading
///
/// * [CK](crate::required_reading::ck)
/// * [FRAMES](crate::required_reading::frames)
/// * [KERNEL](crate::required_reading::kernel)
/// * [NAIF_IDS](crate::required_reading::naif_ids)
/// * [PCK](crate::required_reading::pck)
/// * [SPK](crate::required_reading::spk)
/// * [TIME](crate::required_reading::time)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  -------------------------------------------------
///  INST       I   Name or ID code string of the instrument.
///  TARGET     I   Name or ID code string of the target.
///  TSHAPE     I   Type of shape model used for the target.
///  TFRAME     I   Body-fixed, body-centered frame for target body.
///  ABCORR     I   Aberration correction flag.
///  OBSRVR     I   Name or ID code string of the observer.
///  ET         I   Time of the observation (seconds past J2000).
///  VISIBL     O   Visibility flag (.TRUE./.FALSE.).
/// ```
///
/// # Detailed Input
///
/// ```text
///  INST     indicates the name of an instrument, such as a
///           spacecraft-mounted framing camera. The field of view
///           (FOV) of the instrument will be used to determine if
///           the target is visible with respect to the instrument.
///
///           The position of the instrument INST is considered to
///           coincide with that of the ephemeris object OBSRVR (see
///           description below).
///
///           The size of the instrument's FOV is constrained by the
///           following: There must be a vector A such that all of
///           the instrument's FOV boundary vectors have an angular
///           separation from A of less than (pi/2)-MARGIN radians
///           (see description below). For FOVs that are circular or
///           elliptical, the vector A is the boresight. For FOVs
///           that are rectangular or polygonal, the vector A is
///           calculated.
///
///           See the header of the SPICELIB routine GETFOV for a
///           description of the required parameters associated with
///           an instrument.
///
///           Both object names and NAIF IDs are accepted. For
///           example, both 'CASSINI_ISS_NAC' and '-82360' are
///           accepted. Case and leading or trailing blanks are not
///           significant in the string.
///
///  TARGET   is the name of the target body. This routine determines
///           if the target body appears in the instrument's field of
///           view.
///
///           Both object names and NAIF IDs are accepted. For
///           example, both 'Moon' and '301' are accepted. Case and
///           leading or trailing blanks are not significant in the
///           string.
///
///  TSHAPE   is a string indicating the geometric model used to
///           represent the shape of the target body. The supported
///           options are:
///
///              'ELLIPSOID'     Use a triaxial ellipsoid model,
///                              with radius values provided via the
///                              kernel pool. A kernel variable
///                              having a name of the form
///
///                                 'BODYnnn_RADII'
///
///                              where nnn represents the NAIF
///                              integer code associated with the
///                              body, must be present in the kernel
///                              pool. This variable must be
///                              associated with three numeric
///                              values giving the lengths of the
///                              ellipsoid's X, Y, and Z semi-axes.
///
///              'POINT'         Treat the body as a single point.
///
///           Case and leading or trailing blanks are not
///           significant in the string.
///
///  TFRAME   is the name of the body-fixed, body-centered reference
///           frame associated with the target body. Examples of
///           such names are 'IAU_SATURN' (for Saturn) and 'ITRF93'
///           (for Earth).
///
///           If the target body is modeled as a point, TFRAME
///           is ignored and should be left blank. (Ex: ' ').
///
///           Case and leading or trailing blanks bracketing a
///           non-blank frame name are not significant in the string.
///
///  ABCORR   indicates the aberration corrections to be applied
///           when computing the target's position and orientation.
///
///           For remote sensing applications, where the apparent
///           position and orientation of the target seen by the
///           observer are desired, normally either of the
///           corrections:
///
///              'LT+S'
///              'CN+S'
///
///           should be used. These and the other supported options
///           are described below.
///
///           Supported aberration correction options for
///           observation (the case where radiation is received by
///           observer at ET) are:
///
///              'NONE'         No correction.
///              'LT'           Light time only
///              'LT+S'         Light time and stellar aberration.
///              'CN'           Converged Newtonian (CN) light time.
///              'CN+S'         CN light time and stellar aberration.
///
///           Supported aberration correction options for
///           transmission (the case where radiation is emitted from
///           observer at ET) are:
///
///              'XLT'          Light time only.
///              'XLT+S'        Light time and stellar aberration.
///              'XCN'          Converged Newtonian (CN) light time.
///              'XCN+S'        CN light time and stellar aberration.
///
///           Case, leading and trailing blanks are not significant
///           in the string.
///
///  OBSRVR   is the name of the body from which the target is
///           observed. The instrument INST is treated as if it were
///           co-located with the observer.
///
///           Both object names and NAIF IDs are accepted. For
///           example, both 'CASSINI' and '-82' are accepted. Case
///           and leading or trailing blanks are not significant in
///           the string.
///
///  ET       is the observation time in seconds past the J2000
///           epoch.
/// ```
///
/// # Detailed Output
///
/// ```text
///  VISIBL   is .TRUE. if TARGET is fully or partially in the
///           field-of-view of INST at the time ET. Otherwise,
///           VISIBL is .FALSE.
/// ```
///
/// # Parameters
///
/// ```text
///  MAXVRT   is the maximum number of vertices that may be used
///           to define the boundary of the specified instrument's
///           field of view. See the INCLUDE file gf.inc for details.
///
///  MARGIN   is a small positive number used to constrain the
///           orientation of the boundary vectors of polygonal
///           FOVs. Such FOVs must satisfy the following constraints:
///
///              1)  The boundary vectors must be contained within
///                  a right circular cone of angular radius less
///                  than (pi/2) - MARGIN radians; in other
///                  words, there must be a vector A such that all
///                  boundary vectors have angular separation from
///                  A of less than (pi/2)-MARGIN radians.
///
///              2)  There must be a pair of boundary vectors U, V
///                  such that all other boundary vectors lie in
///                  the same half space bounded by the plane
///                  containing U and V. Furthermore, all other
///                  boundary vectors must have orthogonal
///                  projections onto a specific plane normal to
///                  this plane (the normal plane contains the angle
///                  bisector defined by U and V) such that the
///                  projections have angular separation of at least
///                  2*MARGIN radians from the plane spanned by U
///                  and V.
///
///           MARGIN is currently set to 1.D-12.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the name of either the target or observer cannot be
///      translated to a NAIF ID code, an error is signaled by
///      a routine in the call tree of this routine.
///
///  2)  If the specified aberration correction is an unrecognized
///      value, an error is signaled by a routine
///      in the call tree of this routine.
///
///  3)  If the radii of a target body modeled as an ellipsoid cannot
///      be determined by searching the kernel pool for a kernel
///      variable having a name of the form
///
///         'BODYnnn_RADII'
///
///      where nnn represents the NAIF integer code associated with
///      the body, an error is signaled by a routine in the
///      call tree of this routine.
///
///  4)  If the target and observer bodies are the same, an error is
///      signaled by a routine in the call tree of this routine.
///
///  5)  If the body model specifier TSHAPE is invalid, an error is
///      signaled by either this routine or a routine in the call tree
///      of this routine.
///
///  6)  If a target body-fixed reference frame associated with a
///      non-point target is not recognized, an error is signaled by a
///      routine in the call tree of this routine.
///
///  7)  If a target body-fixed reference frame is not centered at the
///      corresponding target body, an error is signaled by a routine
///      in the call tree of this routine.
///
///  8)  If the instrument name INST does not have a corresponding NAIF
///      ID code, an error is signaled by a routine in the call
///      tree of this routine.
///
///  9)  If the FOV parameters of the instrument are not present in
///      the kernel pool, an error is signaled by a routine
///      in the call tree of this routine.
///
///  10) If the FOV boundary has more than MAXVRT vertices, an error
///      is signaled by a routine in the call tree of this
///      routine.
///
///  11) If the instrument FOV shape is a polygon or rectangle, and
///      this routine cannot find a ray R emanating from the FOV vertex
///      such that maximum angular separation of R and any FOV boundary
///      vector is within the limit (pi/2)-MARGIN radians, an error is
///      signaled by a routine in the call tree of this routine. If the
///      FOV is any other shape, the same error check will be applied
///      with the instrument boresight vector serving the role of R.
///
///  12) If the loaded kernels provide insufficient data to compute a
///      requested state vector, an error is signaled by a
///      routine in the call tree of this routine.
///
///  13) If an error occurs while reading an SPK or other kernel file,
///      the error is signaled by a routine in the call tree
///      of this routine.
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
///  -  SPK data: ephemeris data for target and observer that
///     describe the ephemerides of these objects at the time ET.
///     If aberration corrections are used, the states of
///     target and observer relative to the solar system barycenter
///     must be calculable from the available ephemeris data.
///
///  -  Frame data: if a frame definition is required to convert
///     the observer and target states to the body-fixed frame of
///     the target, that definition must be available in the kernel
///     pool. Typically the definitions of frames not already
///     built-in to SPICE are supplied by loading a frame kernel.
///
///  -  Data defining the reference frame in which the instrument's
///     FOV is defined must be available in the kernel pool.
///     Additionally the name INST must be associated with an ID
///     code.
///
///  -  IK data: the kernel pool must contain data such that
///     the SPICELIB routine GETFOV may be called to obtain
///     parameters for INST.
///
///  The following data may be required:
///
///  -  PCK data: bodies modeled as triaxial ellipsoids must have
///     orientation data provided by variables in the kernel pool.
///
///     Bodies modeled as triaxial ellipsoids must have radii
///     lengths provided by variables in the kernel pool.
///
///  -  CK data: if the frame in which the instrument's FOV is
///     defined is fixed to a spacecraft, at least one CK file will
///     be needed to permit transformation of vectors between that
///     frame and both J2000 and the target body-fixed frame.
///
///  -  SCLK data: if a CK file is needed, an associated SCLK
///     kernel is required to enable conversion between encoded SCLK
///     (used to time-tag CK data) and barycentric dynamical time
///     (TDB).
///
///  Kernel data are normally loaded via FURNSH once per program run,
///  NOT every time this routine is called.
/// ```
///
/// # Particulars
///
/// ```text
///  To treat the target as a ray rather than as an ephemeris object,
///  use the higher-level SPICELIB routine FOVRAY. FOVRAY may be used
///  to determine if distant target objects such as stars are visible
///  in an instrument's FOV at a given time, as long as the direction
///  from the observer to the target can be modeled as a ray.
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
///  1) A spectacular picture was taken by Cassini's
///     narrow-angle camera on Oct. 6, 2010 that shows
///     six of Saturn's moons. Let's verify that the moons
///     in the picture are Epimetheus, Atlas, Daphnis, Pan,
///     Janus, and Enceladus.
///
///     To see this picture, visit:
///     http://photojournal.jpl.nasa.gov/catalog/PIA12741
///     or go to the PDS Image Node's Image Atlas at
///     http://pds-imaging.jpl.nasa.gov/search/search.html.
///     Select Cassini as the mission, ISS as the instrument,
///     and enter 1_N1665078907.122 as the Product ID in the
///     Product tab. Note: these directions may change as the
///     PDS Imaging Node changes.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: fovtrg_ex1.tm
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
///           naif0010.tls                  Leapseconds
///           cpck*.tpc                     Satellite orientation and
///                                         radii
///           pck00010.tpc                  Planet orientation and
///                                         radii
///           cas_rocks_v18.tf              FK for small satellites
///                                         around Saturn
///           cas_v40.tf                    Cassini FK
///           cas_iss_v10.ti                Cassini ISS IK
///           cas00149.tsc                  Cassini SCLK
///           *.bsp                         Ephemeris for Cassini,
///                                         planets, and satellites
///           10279_10284ra.bc              Orientation for Cassini
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'naif0010.tls'
///                               'cpck14Oct2010.tpc'
///                               'cpck_rock_21Jan2011_merged.tpc'
///                               'pck00010.tpc'
///                               'cas_rocks_v18.tf'
///                               'cas_v40.tf'
///                               'cas_iss_v10.ti'
///                               'cas00149.tsc'
///                               '110317AP_RE_90165_18018.bsp'
///                               '110120BP_IRRE_00256_25017.bsp'
///                               '101210R_SCPSE_10256_10302.bsp'
///                               '10279_10284ra.bc'              )
///
///        \begintext
///
///        For project meta-kernels similar to the one shown
///        here, please see the CASSINI SPICE PDS archive.
///
///
///     Example code begins here.
///
///
///           PROGRAM FOVTRG_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         META
///           PARAMETER           ( META   =  'fovtrg_ex1.tm' )
///
///           CHARACTER*(*)         TIMFMT
///           PARAMETER           ( TIMFMT =
///          .      'YYYY-MON-DD HR:MN:SC.#####::TDB (TDB)' )
///
///     C
///     C     This is the spacecraft clock time of the image.
///     C
///           CHARACTER*(*)         SCLK
///           PARAMETER           ( SCLK = '1665078907.122' )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(32)        BODY
///           CHARACTER*(32)        FRNAME
///           CHARACTER*(32)        TIME
///           DOUBLE PRECISION      ET
///           INTEGER               BODYID
///           INTEGER               CAS_ID
///           INTEGER               FRCODE
///           LOGICAL               FOUND
///           LOGICAL               VISIBL
///
///     C
///     C     Load the kernels.
///     C
///           CALL FURNSH ( META )
///
///     C
///     C     Retrieve Cassini's NAIF ID.
///     C
///           CALL BODN2C ( 'CASSINI', CAS_ID, FOUND )
///
///           IF (.NOT. FOUND) THEN
///              CALL SETMSG ( 'Could not find ID code for Cassini.' )
///              CALL SIGERR ( 'SPICE(NOTRANSLATION)' )
///           END IF
///
///     C
///     C     Convert the image tag SCLK to ET.
///     C
///           CALL SCS2E ( CAS_ID, SCLK, ET )
///
///     C
///     C     Convert the ET to a string format for the output.
///     C
///           CALL TIMOUT ( ET, TIMFMT, TIME )
///
///     C
///     C     Search through all of Saturn's moons to see if each
///     C     satellite was in the ISS NAC's field-of-view at
///     C     the image time. We're going to take advantage of the
///     C     fact that all Saturn's moons have a NAIF ID of 6xx.
///     C
///           WRITE (*,*) 'At time ', TIME, ' the following were '
///           WRITE (*,*) 'in the field of view of CASSINI_ISS_NAC'
///
///           DO BODYID = 600, 699
///     C
///     C        Check to see if the BODYID has a translation.
///     C
///              CALL BODC2N ( BODYID, BODY, FOUND )
///
///              IF ( FOUND ) THEN
///     C
///     C           Check to see if a body-fixed frame for this ID
///     C           exists.  If the frame is not in the kernel pool,
///     C           we cannot perform the visibility test.  The main
///     C           cause of a failure is a missing kernel.
///     C
///                 CALL CIDFRM ( BODYID, FRCODE, FRNAME, FOUND )
///
///                 IF ( FOUND ) THEN
///     C
///     C              Is this body in the field-of-view of Cassini's
///     C              ISS narrow-angle camera?
///     C
///                    CALL FOVTRG ( 'CASSINI_ISS_NAC',
///          .                        BODY,  'ellipsoid', FRNAME,
///          .                       'CN+S', 'CASSINI', ET, VISIBL )
///
///     C
///     C              Report results.
///     C
///                    IF ( VISIBL ) THEN
///                       WRITE (*,*) '  ', BODY
///                    END IF
///
///                 END IF
///
///              END IF
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
///      At time 2010-OCT-06 17:09:45.34695 (TDB) the following were
///      in the field of view of CASSINI_ISS_NAC
///        ENCELADUS
///        JANUS
///        EPIMETHEUS
///        ATLAS
///        PAN
///        DAPHNIS
///        ANTHE
///
///
///     Note: there were actually 7 of Saturn's satellites in the
///     field-of-view of Cassini's narrow-angle camera. However, Anthe
///     is very small and was probably obscured by other objects or
///     shadow.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The reference frame associated with INST must be centered at
///      the observer or must be inertial. No check is done to ensure
///      this.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  S.C. Krening       (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 13-AUG-2021 (JDR)
///
///         Edited header to comply with NAIF standard. Corrected the
///         value of MARGIN in the $Parameters section.
///
/// -    SPICELIB Version 1.0.0, 15-FEB-2012 (SCK) (NJB)
/// ```
pub fn fovtrg(
    ctx: &mut SpiceContext,
    inst: &str,
    target: &str,
    tshape: &str,
    tframe: &str,
    abcorr: &str,
    obsrvr: &str,
    et: f64,
    visibl: &mut bool,
) -> crate::Result<()> {
    FOVTRG(
        inst.as_bytes(),
        target.as_bytes(),
        tshape.as_bytes(),
        tframe.as_bytes(),
        abcorr.as_bytes(),
        obsrvr.as_bytes(),
        et,
        visibl,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure FOVTRG ( Is target in FOV at time? )
pub fn FOVTRG(
    INST: &[u8],
    TARGET: &[u8],
    TSHAPE: &[u8],
    TFRAME: &[u8],
    ABCORR: &[u8],
    OBSRVR: &[u8],
    ET: f64,
    VISIBL: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //
    // Ray direction vector required by ZZGFFVIN. This is
    // an unused variable as far is this routine is concerned:
    // the target is an ephemeris object. We initialize the
    // ray to prevent portability problems.
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"FOVTRG", ctx)?;

    //
    // Reject the target shape 'RAY'.
    //
    if EQSTR(TSHAPE, RYSHAP) {
        SETMSG(b"The target shape RAY is not supported by this routine. Use the routine FOVRAY instead.", ctx);
        SIGERR(b"SPICE(INVALIDOPTION)", ctx)?;
        CHKOUT(b"FOVTRG", ctx)?;
        return Ok(());
    }

    //
    // Note to maintenance programmer: input exception checks
    // are delegated to ZZGFFVU. If the implementation of that
    // routine changes, or if this routine is modified to call
    // a different routine in place of ZZGFFVU, then the error
    // handling performed by ZZGFFVU will have to be performed
    // here or in a routine called by this routine.
    //
    // Initialize the visibility calculation.
    //
    ZZGFFVIN(
        INST,
        TSHAPE,
        save.RAYDIR.as_slice(),
        TARGET,
        TFRAME,
        ABCORR,
        OBSRVR,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"FOVTRG", ctx)?;
        return Ok(());
    }
    //
    // Calculate the visibility state.
    //
    ZZGFFVST(ET, VISIBL, ctx)?;

    CHKOUT(b"FOVTRG", ctx)?;
    Ok(())
}
