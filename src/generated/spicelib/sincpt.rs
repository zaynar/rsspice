//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const MAXSRF: i32 = 100;
const INERTL: i32 = 1;
const PCK: i32 = (INERTL + 1);
const CK: i32 = (PCK + 1);
const TK: i32 = (CK + 1);
const DYN: i32 = (TK + 1);
const SWTCH: i32 = (DYN + 1);
const ALL: i32 = -1;
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
const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;
const CTRSIZ: i32 = 2;
const DSKSHP: i32 = 2;
const ELLSHP: i32 = 1;
const MTHLEN: i32 = 500;
const SUBLEN: i32 = 20;
const CVTLEN: i32 = 20;
const TANGNT: i32 = 1;
const GUIDED: i32 = 2;
const TMTLEN: i32 = 20;
const LMBCRV: i32 = 0;
const UMBRAL: i32 = 1;
const PNMBRL: i32 = 2;
const ACLLEN: i32 = 25;
const CTRCOR: i32 = 1;
const ELLCOR: i32 = 2;
const RNAME: &[u8] = b"SINCPT";
const MAXL: i32 = 36;
const FRNMLN: i32 = 32;

struct SaveVars {
    PRVCOR: Vec<u8>,
    PRVMTH: Vec<u8>,
    NSURF: i32,
    SHAPE: i32,
    SRFLST: StackArray<i32, 100>,
    FIRST: bool,
    PRI: bool,
    USECN: bool,
    USELT: bool,
    USESTL: bool,
    XMIT: bool,
    SVCTR1: StackArray<i32, 2>,
    SVTARG: Vec<u8>,
    SVTCDE: i32,
    SVFND1: bool,
    SVCTR2: StackArray<i32, 2>,
    SVOBSR: Vec<u8>,
    SVOBSC: i32,
    SVFND2: bool,
    SVCTR3: StackArray<i32, 2>,
    SVFREF: Vec<u8>,
    SVFXFC: i32,
    SVCTR4: StackArray<i32, 2>,
    SVDREF: Vec<u8>,
    SVDFRC: i32,
    SVCTR5: StackArray<i32, 2>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PRVCOR = vec![b' '; CORLEN as usize];
        let mut PRVMTH = vec![b' '; MTHLEN as usize];
        let mut NSURF: i32 = 0;
        let mut SHAPE: i32 = 0;
        let mut SRFLST = StackArray::<i32, 100>::new(1..=MAXSRF);
        let mut FIRST: bool = false;
        let mut PRI: bool = false;
        let mut USECN: bool = false;
        let mut USELT: bool = false;
        let mut USESTL: bool = false;
        let mut XMIT: bool = false;
        let mut SVCTR1 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVTARG = vec![b' '; MAXL as usize];
        let mut SVTCDE: i32 = 0;
        let mut SVFND1: bool = false;
        let mut SVCTR2 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVOBSR = vec![b' '; MAXL as usize];
        let mut SVOBSC: i32 = 0;
        let mut SVFND2: bool = false;
        let mut SVCTR3 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVFREF = vec![b' '; FRNMLN as usize];
        let mut SVFXFC: i32 = 0;
        let mut SVCTR4 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVDREF = vec![b' '; FRNMLN as usize];
        let mut SVDFRC: i32 = 0;
        let mut SVCTR5 = StackArray::<i32, 2>::new(1..=CTRSIZ);

        FIRST = true;
        fstr::assign(&mut PRVCOR, b" ");
        fstr::assign(&mut PRVMTH, b" ");
        USECN = false;
        USELT = false;
        USESTL = false;
        XMIT = false;

        Self {
            PRVCOR,
            PRVMTH,
            NSURF,
            SHAPE,
            SRFLST,
            FIRST,
            PRI,
            USECN,
            USELT,
            USESTL,
            XMIT,
            SVCTR1,
            SVTARG,
            SVTCDE,
            SVFND1,
            SVCTR2,
            SVOBSR,
            SVOBSC,
            SVFND2,
            SVCTR3,
            SVFREF,
            SVFXFC,
            SVCTR4,
            SVDREF,
            SVDFRC,
            SVCTR5,
        }
    }
}

/// Surface intercept
///
/// Compute, for a given observer and a ray emanating from the
/// observer, the surface intercept of the ray on a target body at
/// a specified epoch, optionally corrected for light time and
/// stellar aberration.
///
/// The surface of the target body may be represented by a triaxial
/// ellipsoid or by topographic data provided by DSK files.
///
/// This routine supersedes SRFXPT.
///
/// # Required Reading
///
/// * [CK](crate::required_reading::ck)
/// * [DSK](crate::required_reading::dsk)
/// * [FRAMES](crate::required_reading::frames)
/// * [NAIF_IDS](crate::required_reading::naif_ids)
/// * [PCK](crate::required_reading::pck)
/// * [SCLK](crate::required_reading::sclk)
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
///  FIXREF     I   Body-fixed, body-centered target body frame.
///  ABCORR     I   Aberration correction.
///  OBSRVR     I   Name of observing body.
///  DREF       I   Reference frame of ray's direction vector.
///  DVEC       I   Ray's direction vector.
///  SPOINT     O   Surface intercept point on the target body.
///  TRGEPC     O   Intercept epoch.
///  SRFVEC     O   Vector from observer to intercept point.
///  FOUND      O   Flag indicating whether intercept was found.
/// ```
///
/// # Detailed Input
///
/// ```text
///  METHOD   is a short string providing parameters defining
///           the computation method to be used. In the syntax
///           descriptions below, items delimited by brackets
///           are optional.
///
///           METHOD may be assigned the following values:
///
///              'ELLIPSOID'
///
///                 The intercept computation uses a triaxial
///                 ellipsoid to model the surface of the target
///                 body. The ellipsoid's radii must be available
///                 in the kernel pool.
///
///
///              'DSK/UNPRIORITIZED[/SURFACES = <surface list>]'
///
///                 The intercept computation uses topographic data
///                 to model the surface of the target body. These
///                 data must be provided by loaded DSK files.
///
///                 The surface list specification is optional. The
///                 syntax of the list is
///
///                    <surface 1> [, <surface 2>...]
///
///                 If present, it indicates that data only for the
///                 listed surfaces are to be used; however, data
///                 need not be available for all surfaces in the
///                 list. If absent, loaded DSK data for any surface
///                 associated with the target body are used.
///
///                 The surface list may contain surface names or
///                 surface ID codes. Names containing blanks must
///                 be delimited by double quotes, for example
///
///                    SURFACES = "Mars MEGDR 128 PIXEL/DEG"
///
///                 If multiple surfaces are specified, their names
///                 or IDs must be separated by commas.
///
///                 See the $Particulars section below for details
///                 concerning use of DSK data.
///
///
///           Neither case nor white space are significant in
///           METHOD, except within double-quoted strings. For
///           example, the string ' eLLipsoid ' is valid.
///
///           Within double-quoted strings, blank characters are
///           significant, but multiple consecutive blanks are
///           considered equivalent to a single blank. Case is
///           not significant. So
///
///              "Mars MEGDR 128 PIXEL/DEG"
///
///           is equivalent to
///
///              " mars megdr  128  pixel/deg "
///
///           but not to
///
///              "MARS MEGDR128PIXEL/DEG"
///
///  TARGET   is the name of the target body. TARGET is
///           case-insensitive, and leading and trailing blanks in
///           TARGET are not significant. Optionally, you may
///           supply a string containing the integer ID code
///           for the object. For example both 'MOON' and '301'
///           are legitimate strings that indicate the Moon is the
///           target body.
///
///           When the target body's surface is represented by a
///           tri-axial ellipsoid, this routine assumes that a
///           kernel variable representing the ellipsoid's radii is
///           present in the kernel pool. Normally the kernel
///           variable would be defined by loading a PCK file.
///
///  ET       is the epoch of participation of the observer,
///           expressed as ephemeris seconds past J2000 TDB: ET is
///           the epoch at which the observer's state is computed.
///
///           When aberration corrections are not used, ET is also
///           the epoch at which the state and orientation of the
///           target body are computed.
///
///           When aberration corrections are used, the position
///           and orientation of the target body are computed at
///           ET-LT or ET+LT, where LT is the one-way light time
///           between the intercept point and the observer, and the
///           sign applied to LT depends on the selected
///           correction. See the description of ABCORR below for
///           details.
///
///  FIXREF   is the name of a body-fixed reference frame centered
///           on the target body. FIXREF may be any such frame
///           supported by the SPICE system, including built-in
///           frames (documented in the Frames Required Reading)
///           and frames defined by a loaded frame kernel (FK). The
///           string FIXREF is case-insensitive, and leading and
///           trailing blanks in FIXREF are not significant.
///
///           The output intercept point SPOINT and the observer-to-
///           intercept vector SRFVEC will be expressed relative to
///           this reference frame.
///
///  ABCORR   indicates the aberration corrections to be applied
///           when computing the target's position and orientation.
///
///           For remote sensing applications, where the apparent
///           surface intercept point seen by the observer is
///           desired, normally the correction
///
///              'CN+S'
///
///           should be used. This and the other supported options
///           are described below. ABCORR may be any of the
///           following:
///
///              'NONE'     Apply no correction. Return the
///                         geometric surface intercept point on the
///                         target body.
///
///           Let LT represent the one-way light time between the
///           observer and the surface intercept point (note: NOT
///           between the observer and the target body's center).
///           The following values of ABCORR apply to the
///           "reception" case in which photons depart from the
///           intercept point's location at the light-time
///           corrected epoch ET-LT and *arrive* at the observer's
///           location at ET:
///
///              'LT'       Correct for one-way light time (also
///                         called "planetary aberration") using a
///                         Newtonian formulation. This correction
///                         yields the location of the surface
///                         intercept point at the moment it
///                         emitted photons arriving at the
///                         observer at ET.
///
///                         The light time correction uses an
///                         iterative solution of the light time
///                         equation. The solution invoked by the
///                         'LT' option uses one iteration.
///
///                         Both the target position as seen by the
///                         observer, and rotation of the target
///                         body, are corrected for light time.
///
///              'LT+S'     Correct for one-way light time and
///                         stellar aberration using a Newtonian
///                         formulation. This option modifies the
///                         surface intercept obtained with the
///                         'LT' option to account for the
///                         observer's velocity relative to the
///                         solar system barycenter. These
///                         computations yield the apparent surface
///                         intercept point.
///
///              'CN'       Converged Newtonian light time
///                         correction. In solving the light time
///                         equation, the 'CN' correction iterates
///                         until the solution converges. Both the
///                         position and rotation of the target
///                         body are corrected for light time.
///
///              'CN+S'     Converged Newtonian light time and
///                         stellar aberration corrections. This
///                         option produces a solution that is at
///                         least as accurate at that obtainable
///                         with the 'LT+S' option. Whether the
///                         'CN+S' solution is substantially more
///                         accurate depends on the geometry of the
///                         participating objects and on the
///                         accuracy of the input data. In all
///                         cases this routine will execute more
///                         slowly when a converged solution is
///                         computed.
///
///                         For reception-case applications
///                         involving intercepts near the target
///                         body limb, this option should be used.
///
///           The following values of ABCORR apply to the
///           "transmission" case in which photons *depart* from
///           the observer's location at ET and arrive at the
///           intercept point at the light-time corrected epoch
///           ET+LT:
///
///              'XLT'      "Transmission" case: correct for
///                         one-way light time using a Newtonian
///                         formulation. This correction yields the
///                         intercept location at the moment it
///                         receives photons emitted from the
///                         observer's location at ET.
///
///                         The light time correction uses an
///                         iterative solution of the light time
///                         equation. The solution invoked by the
///                         'XLT' option uses one iteration.
///
///                         Both the target position as seen by the
///                         observer, and rotation of the target
///                         body, are corrected for light time.
///
///              'XLT+S'    "Transmission" case: correct for
///                         one-way light time and stellar
///                         aberration using a Newtonian
///                         formulation. This option modifies the
///                         intercept obtained with the 'XLT'
///                         option to account for the observer's
///                         velocity relative to the solar system
///                         barycenter.
///
///              'XCN'      Converged Newtonian light time
///                         correction. This is the same as 'XLT'
///                         correction but with further iterations
///                         to a converged Newtonian light time
///                         solution.
///
///              'XCN+S'    "Transmission" case: converged
///                         Newtonian light time and stellar
///                         aberration corrections. This option
///                         produces a solution that is at least as
///                         accurate at that obtainable with the
///                         'XLT+S' option. Whether the 'XCN+S'
///                         solution is substantially more accurate
///                         depends on the geometry of the
///                         participating objects and on the
///                         accuracy of the input data. In all
///                         cases this routine will execute more
///                         slowly when a converged solution is
///                         computed.
///
///                         For transmission-case applications
///                         involving intercepts near the target
///                         body limb, this option should be used.
///
///           Case and embedded blanks are not significant in
///           ABCORR. For example, the string
///
///             'Cn + s'
///
///           is valid.
///
///  OBSRVR   is the name of the observing body. This is typically
///           a spacecraft, the earth, or a surface point on the
///           earth or on another extended object.
///
///           The observer must be outside the target body.
///
///           OBSRVR is case-insensitive, and leading and
///           trailing blanks in OBSRVR are not significant.
///           Optionally, you may supply a string containing the
///           integer ID code for the object. For example both
///           'MOON' and '301' are legitimate strings that indicate
///           the Moon is the observer.
///
///  DREF     is the name of the reference frame relative to which
///           the ray's direction vector is expressed. This may be
///           any frame supported by the SPICE system, including
///           built-in frames (documented in the Frames Required
///           Reading) and frames defined by a loaded frame kernel
///           (FK). The string DREF is case-insensitive, and
///           leading and trailing blanks in DREF are not
///           significant.
///
///           When DREF designates a non-inertial frame, the
///           orientation of the frame is evaluated at an epoch
///           dependent on the frame's center and, if the center is
///           not the observer, on the selected aberration
///           correction. See the description of the direction
///           vector DVEC for details.
///
///  DVEC     is a ray direction vector emanating from the observer.
///           The intercept with the target body's surface of the ray
///           defined by the observer and DVEC is sought.
///
///           DVEC is specified relative to the reference frame
///           designated by DREF.
///
///           Non-inertial reference frames are treated as follows:
///           if the center of the frame is at the observer's
///           location, the frame is evaluated at ET. If the
///           frame's center is located elsewhere, then letting
///           LTCENT be the one-way light time between the observer
///           and the central body associated with the frame, the
///           orientation of the frame is evaluated at ET-LTCENT,
///           ET+LTCENT, or ET depending on whether the requested
///           aberration correction is, respectively, for received
///           radiation, transmitted radiation, or is omitted.
///           LTCENT is computed using the method indicated by
///           ABCORR.
/// ```
///
/// # Detailed Output
///
/// ```text
///  SPOINT   is the surface intercept point on the target body of
///           the ray defined by the observer and the direction
///           vector. If the ray intersects the target body in
///           multiple points, the selected intersection point is
///           the one closest to the observer. The output argument
///           FOUND (see below) indicates whether an intercept was
///           found.
///
///           SPOINT is expressed in Cartesian coordinates,
///           relative to the target body-fixed frame designated by
///           FIXREF. The body-fixed target frame is evaluated at
///           the intercept epoch TRGEPC (see description below).
///
///           When light time correction is used, the duration of
///           light travel between SPOINT to the observer is
///           considered to be the one way light time. When both
///           light time and stellar aberration corrections are
///           used, SPOINT is compute such that, when the vector
///           from the observer to SPOINT is corrected for light
///           time and stellar aberration, the resulting vector
///           lies on the ray defined by the observer's location
///           and DVEC.
///
///           The components of SPOINT are given in units of km.
///
///  TRGEPC   is the "intercept epoch." TRGEPC is defined as
///           follows: letting LT be the one-way light time between
///           the observer and the intercept point, TRGEPC is the
///           epoch ET-LT, ET+LT, or ET depending on whether the
///           requested aberration correction is, respectively, for
///           received radiation, transmitted radiation, or
///           omitted. LT is computed using the method indicated by
///           ABCORR.
///
///           TRGEPC is expressed as seconds past J2000 TDB.
///
///  SRFVEC   is the vector from the observer's position at ET to
///           the aberration-corrected (or optionally, geometric)
///           position of SPOINT, where the aberration corrections
///           are specified by ABCORR. SRFVEC is expressed in the
///           target body-fixed reference frame designated by
///           FIXREF, evaluated at TRGEPC.
///
///           The components of SRFVEC are given in units of km.
///
///           One can use the SPICELIB function VNORM to obtain the
///           distance between the observer and SPOINT:
///
///              DIST = VNORM ( SRFVEC )
///
///           The observer's position OBSPOS, relative to the
///           target body's center, where the center's position is
///           corrected for aberration effects as indicated by
///           ABCORR, can be computed via the call:
///
///              CALL VSUB ( SPOINT, SRFVEC, OBSPOS )
///
///           To transform the vector SRFVEC from a reference frame
///           FIXREF at time TRGEPC to a time-dependent reference
///           frame REF at time ET, the routine PXFRM2 should be
///           called. Let XFORM be the 3x3 matrix representing the
///           rotation from the reference frame FIXREF at time
///           TRGEPC to the reference frame REF at time ET. Then
///           SRFVEC can be transformed to the result REFVEC as
///           follows:
///
///               CALL PXFRM2 ( FIXREF, REF,    TRGEPC, ET, XFORM )
///               CALL MXV    ( XFORM,  SRFVEC, REFVEC )
///
///           The second example in the $Examples header section
///           below presents a complete program that demonstrates
///           this procedure.
///
///  FOUND    is a logical flag indicating whether or not the ray
///           intersects the target. If an intersection exists
///           FOUND will be returned as .TRUE. If the ray misses
///           the target, FOUND will be returned as .FALSE.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the specified aberration correction is unrecognized, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  2)  If either the target or observer input strings cannot be
///      converted to an integer ID code, the error
///      SPICE(IDCODENOTFOUND) is signaled.
///
///  3)  If OBSRVR and TARGET map to the same NAIF integer ID code,
///      the error SPICE(BODIESNOTDISTINCT) is signaled.
///
///  4)  If the input target body-fixed frame FIXREF is not
///      recognized, the error SPICE(NOFRAME) is signaled. A frame
///      name may fail to be recognized because a required frame
///      specification kernel has not been loaded; another cause is a
///      misspelling of the frame name.
///
///  5)  If the input frame FIXREF is not centered at the target body,
///      the error SPICE(INVALIDFRAME) is signaled.
///
///  6)  If the input argument METHOD cannot be parsed, an error
///      is signaled by either this routine or a routine in the
///      call tree of this routine.
///
///  7)  If the target and observer have distinct identities but are
///      at the same location (for example, the target is Mars and the
///      observer is the Mars barycenter), the error
///      SPICE(NOSEPARATION) is signaled.
///
///  8)  If insufficient ephemeris data have been loaded prior to
///      calling SINCPT, an error is signaled by a
///      routine in the call tree of this routine. Note that when
///      light time correction is used, sufficient ephemeris data must
///      be available to propagate the states of both observer and
///      target to the solar system barycenter.
///
///  9)  If the computation method specifies an ellipsoidal target
///      shape and triaxial radii of the target body have not been
///      loaded into the kernel pool prior to calling SINCPT, an error
///      is signaled by a routine in the call tree of this routine.
///
///  10) The target must be an extended body: if any of the radii of
///      the target body are non-positive, an error is signaled by a
///      routine in the call tree of this routine.
///
///  11) If PCK data specifying the target body-fixed frame orientation
///      have not been loaded prior to calling SINCPT, an error is
///      signaled by a routine in the call tree of this routine.
///
///  12) If the reference frame designated by DREF is not recognized
///      by the SPICE frame subsystem, the error SPICE(NOFRAME)
///      is signaled.
///
///  13) If the direction vector DVEC is the zero vector, the error
///      SPICE(ZEROVECTOR) is signaled.
///
///  14) If METHOD specifies that the target surface is represented by
///      DSK data, and no DSK files are loaded for the specified
///      target, an error is signaled by a routine in the call tree
///      of this routine.
///
///  15) If METHOD specifies that the target surface is represented
///      by DSK data, and DSK data are not available for a portion of
///      the target body's surface, an intercept might not be found.
///      This routine does not revert to using an ellipsoidal surface
///      in this case.
/// ```
///
/// # Files
///
/// ```text
///  Appropriate kernels must be loaded by the calling program before
///  this routine is called.
///
///  The following data are required:
///
///  -  SPK data: ephemeris data for target and observer must be
///     loaded. If aberration corrections are used, the states of
///     target and observer relative to the solar system barycenter
///     must be calculable from the available ephemeris data.
///     Typically ephemeris data are made available by loading one
///     or more SPK files via FURNSH.
///
///  -  PCK data: if the computation method is specified as
///     "Ellipsoid," triaxial radii for the target body must be
///     loaded into the kernel pool. Typically this is done by
///     loading a text PCK file via FURNSH.
///
///  -  Further PCK data: rotation data for the target body must
///     be loaded. These may be provided in a text or binary PCK
///     file.
///
///  The following data may be required:
///
///  -  DSK data: if METHOD indicates that DSK data are to be used,
///     DSK files containing topographic data for the target body
///     must be loaded. If a surface list is specified, data for
///     at least one of the listed surfaces must be loaded.
///
///  -  Surface name-ID associations: if surface names are specified
///     in METHOD, the association of these names with their
///     corresponding surface ID codes must be established by
///     assignments of the kernel variables
///
///        NAIF_SURFACE_NAME
///        NAIF_SURFACE_CODE
///        NAIF_SURFACE_BODY
///
///     Normally these associations are made by loading a text
///     kernel containing the necessary assignments. An example
///     of such an assignment is
///
///        NAIF_SURFACE_NAME += 'Mars MEGDR 128 PIXEL/DEG'
///        NAIF_SURFACE_CODE += 1
///        NAIF_SURFACE_BODY += 499
///
///  -  Frame data: if a frame definition is required to convert
///     the observer and target states to the body-fixed frame of
///     the target, that definition must be available in the kernel
///     pool. Similarly, the frame definition required to map
///     between the frame designated by DREF and the target
///     body-fixed frame must be available. Typically the
///     definitions of frames not already built-in to SPICE are
///     supplied by loading a frame kernel.
///
///  -  CK data: if the frame to which DREF refers is fixed to a
///     spacecraft instrument or structure, at least one CK file
///     will be needed to permit transformation of vectors between
///     that frame and both the J2000 and the target body-fixed
///     frames.
///
///  -  SCLK data: if a CK file is needed, an associated SCLK
///     kernel is required to enable conversion between encoded SCLK
///     (used to time-tag CK data) and barycentric dynamical time
///     (TDB).
///
///  In all cases, kernel data are normally loaded once per program
///  run, NOT every time this routine is called.
/// ```
///
/// # Particulars
///
/// ```text
///  Given a ray defined by a direction vector and the location of an
///  observer, SINCPT computes the surface intercept point of the ray
///  on a specified target body. SINCPT also determines the vector
///  from the observer to the surface intercept point. If the ray
///  intersects the target in multiple locations, the intercept
///  closest to the observer is selected.
///
///  When aberration corrections are used, this routine finds the
///  value of SPOINT such that, if SPOINT is regarded as an ephemeris
///  object, after the selected aberration corrections are applied to
///  the vector from the observer to SPOINT, the resulting vector is
///  parallel to the direction vector DVEC.
///
///  This routine computes light time corrections using light time
///  between the observer and the surface intercept point, as opposed
///  to the center of the target. Similarly, stellar aberration
///  corrections done by this routine are based on the direction of
///  the vector from the observer to the light-time corrected
///  intercept point, not to the target center. This technique avoids
///  errors due to the differential between aberration corrections
///  across the target body. Therefore it's valid to use aberration
///  corrections with this routine even when the observer is very
///  close to the intercept point, in particular when the
///  observer-intercept point distance is much less than the
///  observer-target center distance. It's also valid to use stellar
///  aberration corrections even when the intercept point is near or
///  on the limb (as may occur in occultation computations using a
///  point target).
///
///  When comparing surface intercept point computations with results
///  from sources other than SPICE, it's essential to make sure the
///  same geometric definitions are used.
///
///
///  Using DSK data
///  ==============
///
///     DSK loading and unloading
///     -------------------------
///
///     DSK files providing data used by this routine are loaded by
///     calling FURNSH and can be unloaded by calling UNLOAD or
///     KCLEAR. See the documentation of FURNSH for limits on numbers
///     of loaded DSK files.
///
///     For run-time efficiency, it's desirable to avoid frequent
///     loading and unloading of DSK files. When there is a reason to
///     use multiple versions of data for a given target body---for
///     example, if topographic data at varying resolutions are to be
///     used---the surface list can be used to select DSK data to be
///     used for a given computation. It is not necessary to unload
///     the data that are not to be used. This recommendation presumes
///     that DSKs containing different versions of surface data for a
///     given body have different surface ID codes.
///
///
///     DSK data priority
///     -----------------
///
///     A DSK coverage overlap occurs when two segments in loaded DSK
///     files cover part or all of the same domain---for example, a
///     given longitude-latitude rectangle---and when the time
///     intervals of the segments overlap as well.
///
///     When DSK data selection is prioritized, in case of a coverage
///     overlap, if the two competing segments are in different DSK
///     files, the segment in the DSK file loaded last takes
///     precedence. If the two segments are in the same file, the
///     segment located closer to the end of the file takes
///     precedence.
///
///     When DSK data selection is unprioritized, data from competing
///     segments are combined. For example, if two competing segments
///     both represent a surface as sets of triangular plates, the
///     union of those sets of plates is considered to represent the
///     surface.
///
///     Currently only unprioritized data selection is supported.
///     Because prioritized data selection may be the default behavior
///     in a later version of the routine, the UNPRIORITIZED keyword is
///     required in the METHOD argument.
///
///
///     Syntax of the METHOD input argument
///     -----------------------------------
///
///     The keywords and surface list in the METHOD argument
///     are called "clauses." The clauses may appear in any
///     order, for example
///
///        DSK/<surface list>/UNPRIORITIZED
///        DSK/UNPRIORITIZED/<surface list>
///        UNPRIORITIZED/<surface list>/DSK
///
///     The simplest form of the METHOD argument specifying use of
///     DSK data is one that lacks a surface list, for example:
///
///        'DSK/UNPRIORITIZED'
///
///     For applications in which all loaded DSK data for the target
///     body are for a single surface, and there are no competing
///     segments, the above string suffices. This is expected to be
///     the usual case.
///
///     When, for the specified target body, there are loaded DSK
///     files providing data for multiple surfaces for that body, the
///     surfaces to be used by this routine for a given call must be
///     specified in a surface list, unless data from all of the
///     surfaces are to be used together.
///
///     The surface list consists of the string
///
///        SURFACES =
///
///     followed by a comma-separated list of one or more surface
///     identifiers. The identifiers may be names or integer codes in
///     string format. For example, suppose we have the surface
///     names and corresponding ID codes shown below:
///
///        Surface Name                              ID code
///        ------------                              -------
///        'Mars MEGDR 128 PIXEL/DEG'                1
///        'Mars MEGDR 64 PIXEL/DEG'                 2
///        'Mars_MRO_HIRISE'                         3
///
///     If data for all of the above surfaces are loaded, then
///     data for surface 1 can be specified by either
///
///        'SURFACES = 1'
///
///     or
///
///        'SURFACES = "Mars MEGDR 128 PIXEL/DEG"'
///
///     Double quotes are used to delimit the surface name because
///     it contains blank characters.
///
///     To use data for surfaces 2 and 3 together, any
///     of the following surface lists could be used:
///
///        'SURFACES = 2, 3'
///
///        'SURFACES = "Mars MEGDR  64 PIXEL/DEG", 3'
///
///        'SURFACES = 2, Mars_MRO_HIRISE'
///
///        'SURFACES = "Mars MEGDR 64 PIXEL/DEG", Mars_MRO_HIRISE'
///
///     An example of a METHOD argument that could be constructed
///     using one of the surface lists above is
///
///        'DSK/UNPRIORITIZED/SURFACES = "Mars MEGDR 64 PIXEL/DEG", 3'
///
///
///     Round-off errors and mitigating algorithms
///     ------------------------------------------
///
///     When topographic data are used to represent the surface of a
///     target body, round-off errors can produce some results that
///     may seem surprising.
///
///     Note that, since the surface in question might have mountains,
///     valleys, and cliffs, the points of intersection found for
///     nearly identical sets of inputs may be quite far apart from
///     each other: for example, a ray that hits a mountain side in a
///     nearly tangent fashion may, on a different host computer, be
///     found to miss the mountain and hit a valley floor much farther
///     from the observer, or even miss the target altogether.
///
///     Round-off errors can affect segment selection: for example, a
///     ray that is expected to intersect the target body's surface
///     near the boundary between two segments might hit either
///     segment, or neither of them; the result may be
///     platform-dependent.
///
///     A similar situation exists when a surface is modeled by a set
///     of triangular plates, and the ray is expected to intersect the
///     surface near a plate boundary.
///
///     To avoid having the routine fail to find an intersection when
///     one clearly should exist, this routine uses two "greedy"
///     algorithms:
///
///        1) If the ray passes sufficiently close to any of the
///           boundary surfaces of a segment (for example, surfaces of
///           maximum and minimum longitude or latitude), that segment
///           is tested for an intersection of the ray with the
///           surface represented by the segment's data.
///
///           This choice prevents all of the segments from being
///           missed when at least one should be hit, but it could, on
///           rare occasions, cause an intersection to be found in a
///           segment other than the one that would be found if higher
///           precision arithmetic were used.
///
///        2) For type 2 segments, which represent surfaces as
///           sets of triangular plates, each plate is expanded very
///           slightly before a ray-plate intersection test is
///           performed. The default plate expansion factor is
///
///              1 + 1.E-10
///
///           In other words, the sides of the plate are lengthened by
///           1/10 of a micron per km. The expansion keeps the centroid
///           of the plate fixed.
///
///           Plate expansion prevents all plates from being missed
///           in cases where clearly at least one should be hit.
///
///           As with the greedy segment selection algorithm, plate
///           expansion can occasionally cause an intercept to be
///           found on a different plate than would be found if higher
///           precision arithmetic were used. It also can occasionally
///           cause an intersection to be found when the ray misses
///           the target by a very small distance.
///
///
///     Aberration corrections
///     ----------------------
///
///     For irregularly shaped target bodies, the distance between the
///     observer and the nearest surface intercept need not be a
///     continuous function of time; hence the one-way light time
///     between the intercept and the observer may be discontinuous as
///     well. In such cases, the computed light time, which is found
///     using an iterative algorithm, may converge slowly or not at
///     all. In all cases, the light time computation will terminate,
///     but the result may be less accurate than expected.
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
///
///  1) The following program computes surface intercept points on Mars
///     for the boresight and FOV boundary vectors of the MGS MOC
///     narrow angle camera. The intercepts are computed for a single
///     observation epoch. Light time and stellar aberration
///     corrections are used. For simplicity, camera distortion is
///     ignored.
///
///     Intercepts are computed using both triaxial ellipsoid and
///     topographic surface models.
///
///     The topographic model is based on data from the MGS MOLA DEM
///     megr90n000cb, which has a resolution of 4 pixels/degree. A
///     triangular plate model was produced by computing a 720 x 1440
///     grid of interpolated heights from this DEM, then tessellating
///     the height grid. The plate model is stored in a type 2 segment
///     in the referenced DSK file.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File: sincpt_ex1.tm
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
///           mgs_moc_v20.ti                   MGS MOC instrument
///                                            parameters
///           mgs_sclkscet_00061.tsc           MGS SCLK coefficients
///           mgs_sc_ext12.bc                  MGS s/c bus attitude
///           mgs_ext12_ipng_mgs95j.bsp        MGS ephemeris
///           megr90n000cb_plate.bds           Plate model based on
///                                            MEGDR DEM, resolution
///                                            4 pixels/degree.
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de430.bsp',
///                               'mar097.bsp',
///                               'pck00010.tpc',
///                               'naif0011.tls',
///                               'mgs_moc_v20.ti',
///                               'mgs_sclkscet_00061.tsc',
///                               'mgs_sc_ext12.bc',
///                               'mgs_ext12_ipng_mgs95j.bsp',
///                               'megr90n000cb_plate.bds'      )
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM SINCPT_EX1
///           IMPLICIT NONE
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      VNORM
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         META
///           PARAMETER           ( META   = 'sincpt_ex1.tm' )
///
///           INTEGER               ABCLEN
///           PARAMETER           ( ABCLEN = 20 )
///
///           INTEGER               LNSIZE
///           PARAMETER           ( LNSIZE = 78 )
///
///           INTEGER               METLEN
///           PARAMETER           ( METLEN = 40 )
///
///           INTEGER               NAMLEN
///           PARAMETER           ( NAMLEN = 32 )
///
///           INTEGER               TIMLEN
///           PARAMETER           ( TIMLEN = 50 )
///
///           INTEGER               SHPLEN
///           PARAMETER           ( SHPLEN = 80 )
///
///           INTEGER               NCORNR
///           PARAMETER           ( NCORNR = 4 )
///
///           INTEGER               NMETH
///           PARAMETER           ( NMETH  = 2 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(ABCLEN)    ABCORR
///           CHARACTER*(NAMLEN)    CAMERA
///           CHARACTER*(NAMLEN)    DREF
///           CHARACTER*(NAMLEN)    FIXREF
///           CHARACTER*(METLEN)    METHDS ( NMETH )
///           CHARACTER*(METLEN)    METHOD
///           CHARACTER*(NAMLEN)    OBSRVR
///           CHARACTER*(SHPLEN)    SHAPE
///           CHARACTER*(NAMLEN)    SRFTYP ( NMETH )
///           CHARACTER*(NAMLEN)    TARGET
///           CHARACTER*(LNSIZE)    TITLE
///           CHARACTER*(TIMLEN)    UTC
///
///           DOUBLE PRECISION      BOUNDS ( 3, NCORNR )
///           DOUBLE PRECISION      BSIGHT ( 3 )
///           DOUBLE PRECISION      DIST
///           DOUBLE PRECISION      DPR
///           DOUBLE PRECISION      DVEC   ( 3 )
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      LAT
///           DOUBLE PRECISION      LON
///           DOUBLE PRECISION      RADIUS
///           DOUBLE PRECISION      SPOINT ( 3 )
///           DOUBLE PRECISION      SRFVEC ( 3 )
///           DOUBLE PRECISION      TRGEPC
///
///           INTEGER               CAMID
///           INTEGER               I
///           INTEGER               J
///           INTEGER               K
///           INTEGER               N
///
///           LOGICAL               FOUND
///
///           DATA                  ABCORR / 'CN+S'              /
///           DATA                  CAMERA / 'MGS_MOC_NA'        /
///           DATA                  FIXREF / 'IAU_MARS'          /
///           DATA                  METHDS / 'ELLIPSOID',
///          .                               'DSK/UNPRIORITIZED' /
///           DATA                  OBSRVR / 'MGS'               /
///           DATA                  SRFTYP / 'Ellipsoid',
///          .               'MGS/MOLA topography, 4 pixel/deg'  /
///           DATA                  TARGET / 'Mars'              /
///           DATA                  UTC    /
///          .                        '2003 OCT 13 06:00:00 UTC' /
///
///     C
///     C     Load kernel files:
///     C
///           CALL FURNSH ( META )
///
///     C
///     C     Convert the UTC request time to ET (seconds past
///     C     J2000, TDB).
///     C
///           CALL STR2ET ( UTC, ET )
///
///     C
///     C     Get the MGS MOC Narrow angle camera (MGS_MOC_NA)
///     C     ID code. Then look up the field of view (FOV)
///     C     parameters by calling GETFOV.
///     C
///           CALL BODN2C ( CAMERA, CAMID, FOUND )
///
///           IF ( .NOT. FOUND ) THEN
///              CALL SETMSG ( 'Could not find ID code for ' //
///          .                 'instrument #.'               )
///              CALL ERRCH  ( '#', CAMERA                   )
///              CALL SIGERR ( 'SPICE(NOTRANSLATION)'        )
///           END IF
///
///     C
///     C     GETFOV will return the name of the camera-fixed frame
///     C     in the string DREF, the camera boresight vector in
///     C     the array BSIGHT, and the FOV corner vectors in the
///     C     array BOUNDS.
///     C
///           CALL GETFOV ( CAMID,  NCORNR, SHAPE,  DREF,
///          .              BSIGHT, N,      BOUNDS       )
///
///
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Surface Intercept Locations for Camera'
///           WRITE (*,*) 'FOV Boundary and Boresight Vectors'
///           WRITE (*,*) ' '
///           WRITE (*,*) '   Instrument:            ', CAMERA
///           WRITE (*,*) '   Epoch:                 ', UTC
///           WRITE (*,*) '   Aberration correction: ', ABCORR
///
///     C
///     C     Now compute and display the surface intercepts for the
///     C     boresight and all of the FOV boundary vectors.
///     C
///           DO I = 1, NCORNR+1
///
///              IF ( I .LE. NCORNR ) THEN
///
///                 TITLE = 'Corner vector #'
///                 CALL REPMI ( TITLE, '#', I, TITLE )
///
///                 CALL VEQU ( BOUNDS(1,I), DVEC )
///
///              ELSE
///
///                 TITLE = 'Boresight vector'
///                 CALL VEQU ( BSIGHT, DVEC )
///
///              END IF
///
///              WRITE (*,*) ' '
///              WRITE (*,*) TITLE
///
///              TITLE = '  Vector in # frame = '
///              CALL REPMC ( TITLE, '#', DREF, TITLE )
///
///              WRITE (*,*) ' '
///              WRITE (*,*) TITLE
///
///              IF ( I .LE. NCORNR ) THEN
///                 WRITE (*, '(1X,3F20.14)') ( BOUNDS(J,I), J=1,3 )
///              ELSE
///                 WRITE (*, '(1X,3F20.14)') BSIGHT
///              END IF
///
///              WRITE (*,*) ' '
///              WRITE (*,*) '  Intercept:'
///
///     C
///     C        Compute the surface intercept point using
///     C        the specified aberration corrections. Loop
///     C        over the set of computation methods.
///     C
///              DO K = 1, NMETH
///
///                 METHOD = METHDS(K)
///
///                 CALL SINCPT ( METHOD, TARGET, ET,
///          .                    FIXREF, ABCORR, OBSRVR,
///          .                    DREF,   DVEC,   SPOINT,
///          .                    TRGEPC, SRFVEC, FOUND   )
///
///                 IF ( FOUND ) THEN
///     C
///     C              Compute range from observer to apparent
///     C              intercept.
///     C
///                    DIST = VNORM ( SRFVEC )
///     C
///     C              Convert rectangular coordinates to
///     C              planetocentric latitude and longitude.
///     C              Convert radians to degrees.
///     C
///                    CALL RECLAT ( SPOINT, RADIUS, LON, LAT )
///
///                    LON = LON * DPR ()
///                    LAT = LAT * DPR ()
///     C
///     C              Display the results.
///     C
///
///                    WRITE (*,*) ' '
///                    CALL TOSTDO ( '     Surface representation: '
///          .         //            SRFTYP(K)                      )
///                    WRITE (*,*) ' '
///                    WRITE (*,*)
///          .         '     Radius                   (km)  = ', RADIUS
///                    WRITE (*,*)
///          .         '     Planetocentric Latitude  (deg) = ', LAT
///                    WRITE (*,*)
///          .         '     Planetocentric Longitude (deg) = ', LON
///                    WRITE (*,*)
///          .         '     Range                    (km)  = ', DIST
///
///                 ELSE
///
///                    CALL TOSTDO ( '   Surface representation: '
///          .         //            SRFTYP(K)                     )
///                    WRITE (*,*) '     Intercept not found.'
///                    WRITE (*,*) ' '
///
///                 END IF
///
///              END DO
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
///      Surface Intercept Locations for Camera
///      FOV Boundary and Boresight Vectors
///
///         Instrument:            MGS_MOC_NA
///         Epoch:                 2003 OCT 13 06:00:00 UTC
///         Aberration correction: CN+S
///
///      Corner vector 1
///
///        Vector in MGS_MOC_NA frame =
///          0.00000185713838   -0.00380156226592    0.99999277403434
///
///        Intercept:
///
///          Surface representation: Ellipsoid
///
///           Radius                   (km)  =    3384.9411357607282
///           Planetocentric Latitude  (deg) =   -48.477482367206768
///           Planetocentric Longitude (deg) =   -123.47407481971256
///           Range                    (km)  =    388.98308225698986
///
///          Surface representation: MGS/MOLA topography, 4 pixel/deg
///
///           Radius                   (km)  =    3387.6408267726060
///           Planetocentric Latitude  (deg) =   -48.492259559975267
///           Planetocentric Longitude (deg) =   -123.47541193495911
///           Range                    (km)  =    386.14510040407879
///
///      Corner vector 2
///
///        Vector in MGS_MOC_NA frame =
///          0.00000185713838    0.00380156226592    0.99999277403434
///
///        Intercept:
///
///          Surface representation: Ellipsoid
///
///           Radius                   (km)  =    3384.9396985743224
///           Planetocentric Latitude  (deg) =   -48.481636778911913
///           Planetocentric Longitude (deg) =   -123.39881874871132
///           Range                    (km)  =    388.97510005267708
///
///          Surface representation: MGS/MOLA topography, 4 pixel/deg
///
///           Radius                   (km)  =    3387.6403704507966
///           Planetocentric Latitude  (deg) =   -48.496386688872484
///           Planetocentric Longitude (deg) =   -123.40074354811055
///           Range                    (km)  =    386.13616443321536
///
///      Corner vector 3
///
///        Vector in MGS_MOC_NA frame =
///         -0.00000185713838    0.00380156226592    0.99999277403434
///
///        Intercept:
///
///          Surface representation: Ellipsoid
///
///           Radius                   (km)  =    3384.9396897286833
///           Planetocentric Latitude  (deg) =   -48.481662348858336
///           Planetocentric Longitude (deg) =   -123.39882195503854
///           Range                    (km)  =    388.97464113550637
///
///          Surface representation: MGS/MOLA topography, 4 pixel/deg
///
///           Radius                   (km)  =    3387.6403603146168
///           Planetocentric Latitude  (deg) =   -48.496412042429789
///           Planetocentric Longitude (deg) =   -123.40074672915324
///           Range                    (km)  =    386.13571069851986
///
///      Corner vector 4
///
///        Vector in MGS_MOC_NA frame =
///         -0.00000185713838   -0.00380156226592    0.99999277403434
///
///        Intercept:
///
///          Surface representation: Ellipsoid
///
///           Radius                   (km)  =    3384.9411269137695
///           Planetocentric Latitude  (deg) =   -48.477507940479093
///           Planetocentric Longitude (deg) =   -123.47407797517749
///           Range                    (km)  =    388.98262331952731
///
///          Surface representation: MGS/MOLA topography, 4 pixel/deg
///
///           Radius                   (km)  =    3387.6408166344654
///           Planetocentric Latitude  (deg) =   -48.492284916898356
///           Planetocentric Longitude (deg) =   -123.47541506563023
///           Range                    (km)  =    386.14464664863726
///
///      Boresight vector
///
///        Vector in MGS_MOC_NA frame =
///          0.00000000000000    0.00000000000000    1.00000000000000
///
///        Intercept:
///
///          Surface representation: Ellipsoid
///
///     [...]
///
///
///     Warning: incomplete output. Only 100 out of 112 lines have been
///     provided.
///
///
///  2) Use SUBPNT to find the sub-spacecraft point on Mars for the
///     Mars Reconnaissance Orbiter spacecraft (MRO) at a specified
///     time, using the "near point: ellipsoid" computation method.
///     Use both LT+S and CN+S aberration corrections to illustrate
///     the differences.
///
///     Convert the spacecraft to sub-observer point vector obtained
///     from SUBPNT into the MRO_HIRISE_LOOK_DIRECTION reference frame
///     at the observation time. Perform a consistency check with this
///     vector: compare the Mars surface intercept of the ray
///     emanating from the spacecraft and pointed along this vector
///     with the sub-observer point.
///
///     Perform the sub-observer point and surface intercept
///     computations using both triaxial ellipsoid and topographic
///     surface models.
///
///     For this example, the topographic model is based on the MGS
///     MOLA DEM megr90n000eb, which has a resolution of 16
///     pixels/degree. Eight DSKs, each covering longitude and
///     latitude ranges of 90 degrees, were made from this data set.
///     For the region covered by a given DSK, a grid of approximately
///     1500 x 1500 interpolated heights was produced, and this grid
///     was tessellated using approximately 4.5 million triangular
///     plates, giving a total plate count of about 36 million for the
///     entire DSK set.
///
///     All DSKs in the set use the surface ID code 499001, so there
///     is no need to specify the surface ID in the METHOD strings
///     passed to SINCPT and SUBPNT.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File: sincpt_ex2.tm
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
///           mro_psp4_ssd_mro95a.bsp          MRO ephemeris
///           mro_v11.tf                       MRO frame specifications
///           mro_sclkscet_00022_65536.tsc     MRO SCLK coefficients
///                                            parameters
///           mro_sc_psp_070925_071001.bc      MRO attitude
///           megr90n000eb_*_plate.bds         Plate model DSKs based
///                                            on MEGDR DEM, resolution
///                                            16 pixels/degree.
///
///        \begindata
///
///           KERNELS_TO_LOAD = (
///
///              'de430.bsp',
///              'mar097.bsp',
///              'pck00010.tpc',
///              'naif0011.tls',
///              'mro_psp4_ssd_mro95a.bsp',
///              'mro_v11.tf',
///              'mro_sclkscet_00022_65536.tsc',
///              'mro_sc_psp_070925_071001.bc',
///              'megr90n000eb_LL000E00N_UR090E90N_plate.bds'
///              'megr90n000eb_LL000E90S_UR090E00S_plate.bds'
///              'megr90n000eb_LL090E00N_UR180E90N_plate.bds'
///              'megr90n000eb_LL090E90S_UR180E00S_plate.bds'
///              'megr90n000eb_LL180E00N_UR270E90N_plate.bds'
///              'megr90n000eb_LL180E90S_UR270E00S_plate.bds'
///              'megr90n000eb_LL270E00N_UR360E90N_plate.bds'
///              'megr90n000eb_LL270E90S_UR360E00S_plate.bds'  )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM SINCPT_EX2
///           IMPLICIT NONE
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      DPR
///           DOUBLE PRECISION      VDIST
///           DOUBLE PRECISION      VNORM
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         META
///           PARAMETER           ( META   = 'sincpt_ex2.tm' )
///
///           CHARACTER*(*)         F1
///           PARAMETER           ( F1     = '(A,F21.9)' )
///
///           CHARACTER*(*)         F2
///           PARAMETER           ( F2     = '(A)' )
///
///           INTEGER               FRNMLN
///           PARAMETER           ( FRNMLN = 32 )
///
///           INTEGER               MTHLEN
///           PARAMETER           ( MTHLEN = 50 )
///
///           INTEGER               CORLEN
///           PARAMETER           ( CORLEN = 5 )
///
///           INTEGER               NCORR
///           PARAMETER           ( NCORR  = 2 )
///
///           INTEGER               NMETH
///           PARAMETER           ( NMETH  = 2 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(CORLEN)    ABCORR ( NCORR )
///           CHARACTER*(FRNMLN)    FIXREF
///           CHARACTER*(FRNMLN)    HIREF
///           CHARACTER*(MTHLEN)    SINMTH ( NMETH )
///           CHARACTER*(MTHLEN)    SUBMTH ( NMETH )
///
///           DOUBLE PRECISION      ALT
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      LAT
///           DOUBLE PRECISION      LON
///           DOUBLE PRECISION      MROVEC ( 3 )
///           DOUBLE PRECISION      RADIUS
///           DOUBLE PRECISION      SPOINT ( 3 )
///           DOUBLE PRECISION      SRFVEC ( 3 )
///           DOUBLE PRECISION      TRGEPC
///           DOUBLE PRECISION      XFORM  ( 3, 3 )
///           DOUBLE PRECISION      XEPOCH
///           DOUBLE PRECISION      XPOINT ( 3 )
///           DOUBLE PRECISION      XVEC   ( 3 )
///
///           INTEGER               I
///           INTEGER               J
///
///           LOGICAL               FOUND
///
///     C
///     C     Initial values
///     C
///           DATA                  ABCORR / 'LT+S', 'CN+S'         /
///           DATA                  FIXREF / 'IAU_MARS'             /
///           DATA                  SINMTH / 'Ellipsoid',
///          .                               'DSK/Unprioritized'    /
///           DATA                  SUBMTH / 'Ellipsoid/Near point',
///          .                            'DSK/Unprioritized/Nadir' /
///
///     C
///     C     Load kernel files via the meta-kernel.
///     C
///           CALL FURNSH ( META )
///
///     C
///     C     Convert the TDB request time string to seconds past
///     C     J2000, TDB.
///     C
///           CALL STR2ET ( '2007 SEP 30 00:00:00 TDB', ET )
///
///     C
///     C     Compute the sub-spacecraft point using the
///     C     "NEAR POINT: ELLIPSOID" definition.
///     C     Compute the results using both LT+S and CN+S
///     C     aberration corrections.
///     C
///     C     Repeat the computation for each method.
///     C
///     C
///           DO I = 1, NMETH
///
///              WRITE(*,F2) ' '
///              WRITE(*,F2) 'Sub-observer point computation method = '
///          .               // SUBMTH(I)
///
///              DO J = 1, NCORR
///
///                 CALL SUBPNT ( SUBMTH(I),
///          .                    'Mars', ET,     FIXREF, ABCORR(J),
///          .                    'MRO',  SPOINT, TRGEPC, SRFVEC    )
///     C
///     C           Compute the observer's altitude above SPOINT.
///     C
///                 ALT = VNORM ( SRFVEC )
///     C
///     C           Express SRFVEC in the MRO_HIRISE_LOOK_DIRECTION
///     C           reference frame at epoch ET. Since SRFVEC is
///     C           expressed relative to the IAU_MARS frame at
///     C           TRGEPC, we must call PXFRM2 to compute the position
///     C           transformation matrix from IAU_MARS at TRGEPC to
///     C           the MRO_HIRISE_LOOK_DIRECTION frame at time ET.
///     C
///     C           To make code formatting a little easier, we'll
///     C           store the long MRO reference frame name in a
///     C           variable:
///     C
///                 HIREF = 'MRO_HIRISE_LOOK_DIRECTION'
///
///                 CALL PXFRM2 ( FIXREF, HIREF,  TRGEPC, ET, XFORM )
///                 CALL MXV    ( XFORM,  SRFVEC, MROVEC )
///
///     C
///     C           Convert rectangular coordinates to planetocentric
///     C           latitude and longitude. Convert radians to degrees.
///     C
///                 CALL RECLAT ( SPOINT, RADIUS, LON, LAT  )
///
///                 LON = LON * DPR ()
///                 LAT = LAT * DPR ()
///     C
///     C           Write the results.
///     C
///                 WRITE(*,F2) ' '
///                 WRITE(*,F2) '   Aberration correction = '
///          .                // ABCORR(J)
///                 WRITE(*,F1) ' '
///                 WRITE(*,F2) '      MRO-to-sub-observer vector in'
///                 WRITE(*,F2) '      MRO HIRISE look direction frame'
///                 WRITE(*,F1) '        X-component             '
///          .               // '(km) = ', MROVEC(1)
///                 WRITE(*,F1) '        Y-component             '
///          .               // '(km) = ', MROVEC(2)
///                 WRITE(*,F1) '        Z-component             '
///          .               // '(km) = ', MROVEC(3)
///                 WRITE(*,F1) '      Sub-observer point radius '
///          .               // '(km) = ', RADIUS
///                 WRITE(*,F1) '      Planetocentric latitude  '
///          .               // '(deg) = ', LAT
///                 WRITE(*,F1) '      Planetocentric longitude '
///          .               // '(deg) = ', LON
///                 WRITE(*,F1) '      Observer altitude         '
///          .               // '(km) = ',  ALT
///
///     C
///     C           Consistency check: find the surface intercept on
///     C           Mars of the ray emanating from the spacecraft and
///     C           having direction vector MROVEC in the MRO HIRISE
///     C           reference frame at ET. Call the intercept point
///     C           XPOINT. XPOINT should coincide with SPOINT, up to
///     C           a small round-off error.
///     C
///                 CALL SINCPT ( SINMTH(I), 'Mars', ET,    FIXREF,
///          .                    ABCORR(J), 'MRO',  HIREF, MROVEC,
///          .                    XPOINT,    XEPOCH, XVEC,  FOUND  )
///
///                 IF ( .NOT. FOUND ) THEN
///                    WRITE (*,F1) 'Bug: no intercept'
///                 ELSE
///     C
///     C              Report the distance between XPOINT and SPOINT.
///     C
///                    WRITE (*,* ) ' '
///                    WRITE (*,F1) '   Intercept comparison '
///          .         //           'error (km) = ',
///          .                      VDIST( XPOINT, SPOINT )
///                 END IF
///
///              END DO
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
///     Sub-observer point computation method = Ellipsoid/Near point
///
///        Aberration correction = LT+S
///
///           MRO-to-sub-observer vector in
///           MRO HIRISE look direction frame
///             X-component             (km) =           0.286933229
///             Y-component             (km) =          -0.260425939
///             Z-component             (km) =         253.816326385
///           Sub-observer point radius (km) =        3388.299078378
///           Planetocentric latitude  (deg) =         -38.799836378
///           Planetocentric longitude (deg) =        -114.995297227
///           Observer altitude         (km) =         253.816622175
///
///        Intercept comparison error (km) =           0.000002144
///
///        Aberration correction = CN+S
///
///           MRO-to-sub-observer vector in
///           MRO HIRISE look direction frame
///             X-component             (km) =           0.286933107
///             Y-component             (km) =          -0.260426683
///             Z-component             (km) =         253.816315915
///           Sub-observer point radius (km) =        3388.299078376
///           Planetocentric latitude  (deg) =         -38.799836382
///           Planetocentric longitude (deg) =        -114.995297449
///           Observer altitude         (km) =         253.816611705
///
///        Intercept comparison error (km) =           0.000000001
///
///     Sub-observer point computation method = DSK/Unprioritized/Nadir
///
///        Aberration correction = LT+S
///
///           MRO-to-sub-observer vector in
///           MRO HIRISE look direction frame
///             X-component             (km) =           0.282372596
///             Y-component             (km) =          -0.256289313
///             Z-component             (km) =         249.784871247
///           Sub-observer point radius (km) =        3392.330239436
///           Planetocentric latitude  (deg) =         -38.800230156
///           Planetocentric longitude (deg) =        -114.995297338
///           Observer altitude         (km) =         249.785162334
///
///        Intercept comparison error (km) =           0.000002412
///
///        Aberration correction = CN+S
///
///           MRO-to-sub-observer vector in
///           MRO HIRISE look direction frame
///             X-component             (km) =           0.282372464
///             Y-component             (km) =          -0.256290075
///             Z-component             (km) =         249.784860121
///           Sub-observer point radius (km) =        3392.330239564
///           Planetocentric latitude  (deg) =         -38.800230162
///           Planetocentric longitude (deg) =        -114.995297569
///           Observer altitude         (km) =         249.785151209
///
///        Intercept comparison error (km) =           0.000000001
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  A cautionary note: if aberration corrections are used, and
///      if DREF is the target body-fixed frame, the epoch at which
///      that frame is evaluated is offset from ET by the light time
///      between the observer and the *center* of the target body.
///      This light time normally will differ from the light time
///      between the observer and intercept point. Consequently the
///      orientation of the target body-fixed frame at TRGEPC will
///      not match that of the target body-fixed frame at the epoch
///      associated with DREF. As a result, various derived quantities
///      may not be as expected: for example, SRFVEC would not be
///      parallel to DVEC.
///
///      In many applications the errors arising from this frame
///      discrepancy may be insignificant; however a safe approach is
///      to always use as DREF a frame other than the target
///      body-fixed frame.
///
///  2)  This routine must not be used for cases where the observer
///      is inside the target body. This routine does not attempt to
///      detect this condition.
///
///      If the observer is a point on a target surface described
///      by DSK data, care must be taken to ensure the observer is
///      sufficiently far outside the target. The routine should
///      not be used for surfaces for which "outside" cannot be
///      defined.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  S.C. Krening       (JPL)
///  B.V. Semenov       (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.1.0, 26-OCT-2021 (JDR) (NJB)
///
///         Bug fix: PRVCOR is no longer set to blank before
///         ABCORR is parsed.
///
///         ZZVALCOR is now used instead of ZZPRSCOR. This provides
///         better error handling.
///
///         Edits to $Examples section to comply with NAIF standard.
///
///         The header's $Detailed_Input and $Restrictions sections
///         were updated to state that the observer must be
///         outside the target body.
///
/// -    SPICELIB Version 3.0.0, 04-APR-2017 (NJB)
///
///         01-FEB-2016 (NJB)
///
///            Upgraded to support surfaces represented by DSKs.
///
///            Updated kernels are used in header example programs.
///
/// -    SPICELIB Version 2.0.0, 31-MAR-2014 (NJB) (SCK) (BVS)
///
///         Bug fix: FIRST is now set to .FALSE. at the completion
///         of a successful initialization pass. This does not affect
///         the routine's outputs but improves efficiency.
///
///         Bug fix: redundant call to SPKSSB was removed. This does not
///         affect the routine's outputs but improves efficiency.
///
///         References to the new PXFRM2 routine were added, which changed
///         the Detailed Output section and the second example. Some header
///         comment corrections were made.
///
///         Upgrade: this routine now uses ZZVALCOR rather than
///         ZZPRSCOR, simplifying the implementation.
///
///         Upgrade: this routine now saves the input body names and
///         ZZBODTRN state counters and does name-ID conversions only if
///         the counters have changed.
///
///         Upgrade: this routine now saves the input frame names and POOL
///         state counters and does frame name-ID conversions only if the
///         counters have changed.
///
/// -    SPICELIB Version 1.2.0, 07-APR-2010 (NJB)
///
///         Code style improvement: re-use of variables in
///         FRINFO calls has been eliminated. There is no impact
///         of the behavior of the routine.
///
/// -    SPICELIB Version 1.1.0, 17-MAR-2009 (NJB) (EDW)
///
///         Bug fix: quick test for non-intersection is
///         no longer performed when observer-target distance
///         is less than target's maximum radius.
///
///         Typos in the Detailed Input section's description of DREF
///         were corrected.
///
///         In the header examples, meta-kernel names were updated to use
///         the suffix
///
///            ".tm"
///
///         Incorrect frame name FIXFRM was changed to FIXREF in
///         documentation.
///
///         Typo correction in $Required_Reading, changed FRAME
///         to FRAMES.
///
/// -    SPICELIB Version 1.0.0, 02-MAR-2008 (NJB)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 3.0.0, 04-APR-2017 (NJB)
///
///         Upgraded to support surfaces represented by DSKs.
///
///         The routine was re-written so as to use a private
///         routine to implement the intersection algorithm.
///         That routine has been generalized so that it does
///         not depend on the target surface representation: it
///         uses callback routines to compute ray-surface intercepts
///         for a specified ray and time, the surface tangency point
///         for a given ray, and the radius of an outer bounding
///         sphere for the target.
/// ```
pub fn sincpt(
    ctx: &mut SpiceContext,
    method: &str,
    target: &str,
    et: f64,
    fixref: &str,
    abcorr: &str,
    obsrvr: &str,
    dref: &str,
    dvec: &[f64; 3],
    spoint: &mut [f64; 3],
    trgepc: &mut f64,
    srfvec: &mut [f64; 3],
    found: &mut bool,
) -> crate::Result<()> {
    SINCPT(
        method.as_bytes(),
        target.as_bytes(),
        et,
        fixref.as_bytes(),
        abcorr.as_bytes(),
        obsrvr.as_bytes(),
        dref.as_bytes(),
        dvec,
        spoint,
        trgepc,
        srfvec,
        found,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SINCPT ( Surface intercept )
pub fn SINCPT(
    METHOD: &[u8],
    TARGET: &[u8],
    ET: f64,
    FIXREF: &[u8],
    ABCORR: &[u8],
    OBSRVR: &[u8],
    DREF: &[u8],
    DVEC: &[f64],
    SPOINT: &mut [f64],
    TRGEPC: &mut f64,
    SRFVEC: &mut [f64],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DVEC = DummyArray::new(DVEC, 1..=3);
    let mut SPOINT = DummyArrayMut::new(SPOINT, 1..=3);
    let mut SRFVEC = DummyArrayMut::new(SRFVEC, 1..=3);
    let mut PNTDEF = [b' '; CVTLEN as usize];
    let mut SHPSTR = [b' '; SHPLEN as usize];
    let mut SUBTYP = [b' '; SUBLEN as usize];
    let mut TRMSTR = [b' '; TMTLEN as usize];
    let mut DCENTR: i32 = 0;
    let mut DCLASS: i32 = 0;
    let mut DFRCDE: i32 = 0;
    let mut DTYPID: i32 = 0;
    let mut FXCENT: i32 = 0;
    let mut FXCLSS: i32 = 0;
    let mut FXFCDE: i32 = 0;
    let mut FXTYID: i32 = 0;
    let mut OBSCDE: i32 = 0;
    let mut TRGCDE: i32 = 0;
    let mut ATTBLK = StackArray::<bool, 15>::new(1..=NABCOR);
    let mut FND: bool = false;
    let mut SURFUP: bool = false;

    //
    // SPICELIB functions
    //

    //
    // EXTERNAL routines
    //

    //
    // Local parameters
    //

    //
    // Saved body name length.
    //

    //
    // Saved frame name length.
    //

    //
    // Local variables
    //

    //
    // Saved name/ID item declarations.
    //

    //
    // Saved frame name/ID item declarations.
    //

    //
    // Saved surface name/ID item declarations.
    //

    //
    // Saved variables
    //

    //
    // Saved name/ID items.
    //

    //
    // Saved frame name/ID items.
    //

    //
    // Saved surface name/ID items.
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

    CHKIN(RNAME, ctx)?;

    //
    // Nothing has been found yet.
    //
    *FOUND = false;

    //
    // Counter initialization is done separately.
    //
    if save.FIRST {
        //
        // Initialize counters.
        //
        ZZCTRUIN(save.SVCTR1.as_slice_mut(), ctx);
        ZZCTRUIN(save.SVCTR2.as_slice_mut(), ctx);
        ZZCTRUIN(save.SVCTR3.as_slice_mut(), ctx);
        ZZCTRUIN(save.SVCTR4.as_slice_mut(), ctx);
        ZZCTRUIN(save.SVCTR5.as_slice_mut(), ctx);
    }

    if (save.FIRST || fstr::ne(ABCORR, &save.PRVCOR)) {
        //
        // The aberration correction flag differs from the value it
        // had on the previous call, if any. Analyze the new flag.
        //
        ZZVALCOR(ABCORR, ATTBLK.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }
        //
        // Set logical flags indicating the attributes of the requested
        // correction:
        //
        //    XMIT is .TRUE. when the correction is for transmitted
        //    radiation.
        //
        //    USELT is .TRUE. when any type of light time correction
        //    (normal or converged Newtonian) is specified.
        //
        //    USECN indicates converged Newtonian light time correction.
        //
        //    USESTL indicates stellar aberration corrections.
        //
        //
        // The above definitions are consistent with those used by
        // ZZVALCOR.
        //
        save.XMIT = ATTBLK[XMTIDX];
        save.USELT = ATTBLK[LTIDX];
        save.USECN = ATTBLK[CNVIDX];
        save.USESTL = ATTBLK[STLIDX];
        //
        // The aberration correction flag is valid; save it.
        //
        fstr::assign(&mut save.PRVCOR, ABCORR);
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
        &mut FND,
        ctx,
    )?;

    if !FND {
        SETMSG(b"The target, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE Toolkit, or that you failed to load a kernel containing a name-ID mapping for this body.", ctx);
        ERRCH(b"#", TARGET, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    ZZBODS2C(
        save.SVCTR2.as_slice_mut(),
        &mut save.SVOBSR,
        &mut save.SVOBSC,
        &mut save.SVFND2,
        OBSRVR,
        &mut OBSCDE,
        &mut FND,
        ctx,
    )?;

    if !FND {
        SETMSG(b"The observer, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE Toolkit, or that you failed to load a kernel containing a name-ID mapping for this body.", ctx);
        ERRCH(b"#", OBSRVR, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Check the input body codes. If they are equal, signal
    // an error.
    //
    if (OBSCDE == TRGCDE) {
        SETMSG(b"In computing the surface intercept point, the observing body and target body are the same. Both are #.", ctx);
        ERRCH(b"#", OBSRVR, ctx);
        SIGERR(b"SPICE(BODIESNOTDISTINCT)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Determine the attributes of the frame designated by FIXREF.
    //
    ZZNAMFRM(
        save.SVCTR3.as_slice_mut(),
        &mut save.SVFREF,
        &mut save.SVFXFC,
        FIXREF,
        &mut FXFCDE,
        ctx,
    )?;

    FRINFO(FXFCDE, &mut FXCENT, &mut FXCLSS, &mut FXTYID, &mut FND, ctx)?;

    if FAILED(ctx) {
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    if !FND {
        SETMSG(b"Reference frame # is not recognized by the SPICE frame subsystem. Possibly a required frame definition kernel has not been loaded.", ctx);
        ERRCH(b"#", FIXREF, ctx);
        SIGERR(b"SPICE(NOFRAME)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Make sure that FIXREF is centered at the target body's center.
    //
    if (FXCENT != TRGCDE) {
        SETMSG(b"Reference frame # is not centered at the target body #. The ID code of the frame center is #.", ctx);
        ERRCH(b"#", FIXREF, ctx);
        ERRCH(b"#", TARGET, ctx);
        ERRINT(b"#", FXCENT, ctx);
        SIGERR(b"SPICE(INVALIDFRAME)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Check for a zero ray direction vector.
    //
    if VZERO(DVEC.as_slice()) {
        SETMSG(
            b"Input ray direction was the zero vector; this vector must be non-zero.",
            ctx,
        );
        SIGERR(b"SPICE(ZEROVECTOR)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Determine the attributes of the frame designated by DREF.
    //
    ZZNAMFRM(
        save.SVCTR4.as_slice_mut(),
        &mut save.SVDREF,
        &mut save.SVDFRC,
        DREF,
        &mut DFRCDE,
        ctx,
    )?;

    FRINFO(DFRCDE, &mut DCENTR, &mut DCLASS, &mut DTYPID, &mut FND, ctx)?;

    if FAILED(ctx) {
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    if !FND {
        SETMSG(b"Reference frame # is not recognized by the SPICE frame subsystem. Possibly a required frame definition kernel has not been loaded.", ctx);
        ERRCH(b"#", DREF, ctx);
        SIGERR(b"SPICE(NOFRAME)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Check whether the surface name/ID mapping has been updated.
    //
    ZZSRFTRK(save.SVCTR5.as_slice_mut(), &mut SURFUP, ctx)?;

    //
    // Initialize the SINCPT utility package for the next computation.
    // The choice of initialization routine depends on the target
    // surface type.
    //
    if ((save.FIRST || SURFUP) || fstr::ne(METHOD, &save.PRVMTH)) {
        //
        // Set the previous method string to an invalid value, so it
        // cannot match any future, valid input. This will force this
        // routine to parse the input method on the next call if any
        // failure occurs in this branch. Once success is assured, we can
        // record the current method in the previous method string.
        //
        fstr::assign(&mut save.PRVMTH, b" ");

        //
        // Parse the method string. If the string is valid, the
        // outputs SHAPE and SUBTYP will always be be set. However,
        // SUBTYP is not used in this routine.
        //
        // For DSK shapes, the surface list array and count will be set
        // if the method string contains a surface list.
        //
        ZZPRSMET(
            TRGCDE,
            METHOD,
            MAXSRF,
            &mut SHPSTR,
            &mut SUBTYP,
            &mut save.PRI,
            &mut save.NSURF,
            save.SRFLST.as_slice_mut(),
            &mut PNTDEF,
            &mut TRMSTR,
            ctx,
        )?;
        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        if EQSTR(&SHPSTR, b"ELLIPSOID") {
            save.SHAPE = ELLSHP;
        } else if EQSTR(&SHPSTR, b"DSK") {
            save.SHAPE = DSKSHP;
        } else {
            //
            // This is a backstop check.
            //
            SETMSG(b"[1] Returned shape value from method string was <#>.", ctx);
            ERRCH(b"#", &SHPSTR, ctx);
            SIGERR(b"SPICE(BUG)", ctx)?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // There should be no subtype specification in the method
        // string.
        //
        if fstr::ne(&SUBTYP, b" ") {
            SETMSG(b"Spurious sub-observer point type <#> was present in the method string #. The sub-observer type is valid in the method strings for SUBPNT and SUBSLR, but is not applicable for SINCPT.", ctx);
            ERRCH(b"#", &SUBTYP, ctx);
            ERRCH(b"#", METHOD, ctx);
            SIGERR(b"SPICE(INVALIDMETHOD)", ctx)?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        fstr::assign(&mut save.PRVMTH, METHOD);
    }

    //
    // At this point, the first pass actions were successful.
    //
    save.FIRST = false;

    if (save.SHAPE == ELLSHP) {
        //
        // Initialize the intercept algorithm to use the reference
        // ellipsoid of the target body.
        //
        ZZSUELIN(TRGCDE, ctx)?;
    } else if (save.SHAPE == DSKSHP) {
        //
        // This is the DSK case.
        //
        // If the method string listed a set of surface IDs, NSURF is
        // positive and SRFLST contains those IDs.
        //
        // Initialize the intercept algorithm to use a DSK
        // model for the surface of the target body.
        //
        ZZSUDSKI(TRGCDE, save.NSURF, save.SRFLST.as_slice(), FXFCDE, ctx)?;
    } else {
        //
        // This is a backstop check.
        //
        SETMSG(b"[2] Returned shape value from method string was <#>.", ctx);
        ERRCH(b"#", &SHPSTR, ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Perform the intercept computation.
    //
    ZZSFXCOR(
        ZZRAYNP,
        ZZMAXRAD,
        ZZRAYSFX,
        TRGCDE,
        ET,
        ABCORR,
        save.USELT,
        save.USECN,
        save.USESTL,
        save.XMIT,
        FIXREF,
        OBSCDE,
        DFRCDE,
        DCLASS,
        DCENTR,
        DVEC.as_slice(),
        SPOINT.as_slice_mut(),
        TRGEPC,
        SRFVEC.as_slice_mut(),
        FOUND,
        ctx,
    )?;

    CHKOUT(RNAME, ctx)?;
    Ok(())
}
