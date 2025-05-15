//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const MAXSRF: i32 = 100;
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
const RNAME: &[u8] = b"ILLUMF";
const TOLSCL: f64 = 0.0000000001;
const MAXL: i32 = 36;
const FRNMLN: i32 = 32;

struct SaveVars {
    PRVMTH: Vec<u8>,
    PRVCOR: Vec<u8>,
    CENTER: i32,
    NSURF: i32,
    SHAPE: i32,
    SRFLST: StackArray<i32, 100>,
    TRGCDE: i32,
    FIRST: bool,
    PRI: bool,
    USELT: bool,
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
    SVREFC: i32,
    SVCTR4: StackArray<i32, 2>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PRVMTH = vec![b' '; MTHLEN as usize];
        let mut PRVCOR = vec![b' '; CORLEN as usize];
        let mut CENTER: i32 = 0;
        let mut NSURF: i32 = 0;
        let mut SHAPE: i32 = 0;
        let mut SRFLST = StackArray::<i32, 100>::new(1..=MAXSRF);
        let mut TRGCDE: i32 = 0;
        let mut FIRST: bool = false;
        let mut PRI: bool = false;
        let mut USELT: bool = false;
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
        let mut SVREFC: i32 = 0;
        let mut SVCTR4 = StackArray::<i32, 2>::new(1..=CTRSIZ);

        FIRST = true;
        PRI = false;
        NSURF = 0;
        fstr::assign(&mut PRVCOR, b" ");
        fstr::assign(&mut PRVMTH, b" ");
        SHAPE = 0;

        Self {
            PRVMTH,
            PRVCOR,
            CENTER,
            NSURF,
            SHAPE,
            SRFLST,
            TRGCDE,
            FIRST,
            PRI,
            USELT,
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
            SVREFC,
            SVCTR4,
        }
    }
}

/// Illumination angles, general source, return flags
///
/// Compute the illumination angles---phase, incidence, and
/// emission---at a specified point on a target body. Return logical
/// flags indicating whether the surface point is visible from
/// the observer's position and whether the surface point is
/// illuminated.
///
/// The target body's surface is represented using topographic data
/// provided by DSK files, or by a reference ellipsoid.
///
/// The illumination source is a specified ephemeris object.
///
/// # Required Reading
///
/// * [DSK](crate::required_reading::dsk)
/// * [FRAMES](crate::required_reading::frames)
/// * [NAIF_IDS](crate::required_reading::naif_ids)
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
///  ILUSRC     I   Name of illumination source.
///  ET         I   Epoch in ephemeris seconds past J2000 TDB.
///  FIXREF     I   Body-fixed, body-centered target body frame.
///  ABCORR     I   Desired aberration correction.
///  OBSRVR     I   Name of observing body.
///  SPOINT     I   Body-fixed coordinates of a target surface point.
///  TRGEPC     O   Target surface point epoch.
///  SRFVEC     O   Vector from observer to target surface point.
///  PHASE      O   Phase angle at the surface point.
///  INCDNC     O   Source incidence angle at the surface point.
///  EMISSN     O   Emission angle at the surface point.
///  VISIBL     O   Visibility flag (.TRUE. = visible).
///  LIT        O   Illumination flag (.TRUE. = illuminated).
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
///                 The illumination angle computation uses a
///                 triaxial ellipsoid to model the surface of the
///                 target body. The ellipsoid's radii must be
///                 available in the kernel pool.
///
///
///              'DSK/UNPRIORITIZED[/SURFACES = <surface list>]'
///
///                 The illumination angle computation uses
///                 topographic data to model the surface of the
///                 target body. These data must be provided by
///                 loaded DSK files.
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
///                    'SURFACES = "Mars MEGDR 128 PIXEL/DEG"'
///
///                 If multiple surfaces are specified, their names
///                 or IDs must be separated by commas.
///
///                 See the $Particulars section below for details
///                 concerning use of DSK data.
///
///
///           Neither case nor white space are significant in METHOD,
///           except within double-quoted strings representing
///           surfaces. For example, the string ' eLLipsoid ' is valid.
///
///           Within double-quoted strings representing surfaces, blank
///           characters are significant, but multiple consecutive
///           blanks are considered equivalent to a single blank. Case
///           is not significant. So
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
///           supply a string containing the integer ID code for
///           the object. For example both 'MOON' and '301' are
///           legitimate strings that indicate the Moon is the
///           target body.
///
///  ILUSRC   is the name of the illumination source. This source
///           may be any ephemeris object. Case, blanks, and
///           numeric values are treated in the same way as for the
///           input TARGET.
///
///  ET       is the epoch, expressed as seconds past J2000 TDB,
///           for which the apparent illumination angles at the
///           specified surface point on the target body, as seen
///           from the observing body, are to be computed.
///
///  FIXREF   is the name of a body-fixed reference frame centered
///           on the target body. FIXREF may be any such frame
///           supported by the SPICE system, including built-in
///           frames (documented in the Frames Required Reading)
///           and frames defined by a loaded frame kernel (FK). The
///           string FIXREF is case-insensitive, and leading and
///           trailing blanks in FIXREF are not significant.
///
///           The input surface point SPOINT and the output vector
///           SRFVEC are expressed relative to this reference
///           frame.
///
///  ABCORR   is the aberration correction to be used in computing
///           the position and orientation of the target body and
///           the location of the illumination source.
///
///           For remote sensing applications, where the apparent
///           illumination angles seen by the observer are desired,
///           normally either of the corrections
///
///              'LT+S'
///              'CN+S'
///
///           should be used. These and the other supported options
///           are described below. ABCORR may be any of the
///           following:
///
///              'NONE'     No aberration correction.
///
///           Let LT represent the one-way light time between the
///           observer and the input surface point SPOINT (note: NOT
///           between the observer and the target body's center). The
///           following values of ABCORR apply to the "reception" case
///           in which photons depart from SPOINT at the light-time
///           corrected epoch ET-LT and *arrive* at the observer's
///           location at ET:
///
///              'LT'       Correct both the position of SPOINT as
///                         seen by the observer, and the position
///                         of the illumination source as seen by
///                         the target, for light time. Correct the
///                         orientation of the target for light
///                         time.
///
///              'LT+S'     Correct both the position of SPOINT as
///                         seen by the observer, and the position
///                         of the illumination source as seen by
///                         the target, for light time and stellar
///                         aberration. Correct the orientation of
///                         the target for light time.
///
///              'CN'       Converged Newtonian light time
///                         correction. In solving the light time
///                         equations for SPOINT and the
///                         illumination source, the 'CN'
///                         correction iterates until the solution
///                         converges.
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
///
///           The following values of ABCORR apply to the
///           "transmission" case in which photons *arrive* at
///           SPOINT at the light-time corrected epoch ET+LT and
///           *depart* from the observer's location at ET:
///
///              'XLT'      "Transmission" case: correct for
///                         one-way light time using a Newtonian
///                         formulation. This correction yields the
///                         illumination angles at the moment that
///                         SPOINT receives photons emitted from the
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
///                         formulation  This option modifies the
///                         angles obtained with the 'XLT' option
///                         to account for the observer's and
///                         target's velocities relative to the
///                         solar system barycenter (the latter
///                         velocity is used in computing the
///                         direction to the apparent illumination
///                         source).
///
///              'XCN'      Converged Newtonian light time
///                         correction. This is the same as XLT
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
///           Neither case nor white space are significant in
///           ABCORR. For example, the string
///
///             'Lt + s'
///
///           is valid.
///
///  OBSRVR   is the name of the observing body. The observing body is
///           an ephemeris object: it typically is a spacecraft, an
///           extended body, or a surface point for which ephemeris
///           data are available. OBSRVR is case-insensitive, and
///           leading and trailing blanks in OBSRVR are not
///           significant. Optionally, you may supply a string
///           containing the integer ID code for the object. For
///           example both 'MOON' and '301' are legitimate strings that
///           indicate the Moon is the observer.
///
///           OBSRVR may be not be identical to TARGET.
///
///  SPOINT   is a surface point on the target body, expressed in
///           Cartesian coordinates, relative to the body-fixed
///           target frame designated by FIXREF.
///
///           SPOINT need not be visible from the observer's
///           location at the epoch ET.
///
///           The components of SPOINT have units of km.
/// ```
///
/// # Detailed Output
///
/// ```text
///  TRGEPC   is the "target surface point epoch." TRGEPC is defined as
///           follows: letting LT be the one-way light time between the
///           observer and the input surface point SPOINT, TRGEPC is
///           either the epoch ET-LT, ET+LT or ET depending on whether
///           the requested aberration correction is, respectively, for
///           received radiation, transmitted radiation or omitted. LT
///           is computed using the method indicated by ABCORR.
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
///           FIXREF at time TRGEPC to a time-dependent reference frame
///           REF at ET, the routine PXFRM2 should be called. For
///           example, let XFORM be 3x3 matrix representing the
///           rotation from the body-fixed reference frame FIXREF at
///           time TRGEPC to the time-dependent frame REF at time ET.
///           Then SRFVEC can be transformed to the result REFVEC as
///           follows:
///
///               CALL PXFRM2 ( FIXREF, REF,    TRGEPC, ET, XFORM )
///               CALL MXV    ( XFORM,  SRFVEC, REFVEC )
///
///
///  The following outputs depend on the existence of a well-defined
///  outward normal vector to the surface at SPOINT. See restriction 1.
///
///
///  PHASE    is the phase angle at SPOINT, as seen from OBSRVR at time
///           ET. This is the angle between the negative of the vector
///           SRFVEC and the SPOINT-illumination source vector at
///           TRGEPC. Units are radians. The range of PHASE is [0, pi].
///           See $Particulars below for a detailed discussion of the
///           definition.
///
///  INCDNC   is the illumination source incidence angle at SPOINT, as
///           seen from OBSRVR at time ET. This is the angle between
///           the surface normal vector at SPOINT and the SPOINT-source
///           vector at TRGEPC. Units are radians. The range of INCDNC
///           is [0, pi]. See $Particulars below for a detailed
///           discussion of the definition.
///
///  EMISSN   is the emission angle at SPOINT, as seen from OBSRVR at
///           time ET. This is the angle between the surface normal
///           vector at SPOINT and the negative of the vector SRFVEC.
///           Units are radians. The range of EMISSN is [0, pi]. See
///           $Particulars below for a detailed discussion of the
///           definition.
///
///  VISIBL   is a logical flag indicating whether the surface
///           point is visible to the observer. VISIBL takes into
///           account whether the target surface occults SPOINT,
///           regardless of the emission angle at SPOINT. VISIBL is
///           returned with the value .TRUE. if SPOINT is visible;
///           otherwise it is .FALSE.
///
///  LIT      is a logical flag indicating whether the surface
///           point is illuminated; the point is considered to be
///           illuminated if the vector from the point to the
///           center of the illumination source doesn't intersect
///           the target surface. LIT takes into account whether
///           the target surface casts a shadow on SPOINT,
///           regardless of the incidence angle at SPOINT. LIT is
///           returned with the value .TRUE. if SPOINT is
///           illuminated; otherwise it is .FALSE.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the specified aberration correction is relativistic or
///      calls for stellar aberration but not light time correction,
///      the error SPICE(NOTSUPPORTED) is signaled.
///
///  2)  If the specified aberration correction is any other
///      unrecognized value, an error is signaled
///      by a routine in the call tree of this routine.
///
///  3)  If any of the target, observer, or illumination source
///      input strings cannot be converted to an integer ID code, the
///      error SPICE(IDCODENOTFOUND) is signaled.
///
///  4)  If OBSRVR and TARGET map to the same NAIF integer ID code,
///      the error SPICE(BODIESNOTDISTINCT) is signaled.
///
///  5)  If the input target body-fixed frame FIXREF is not
///      recognized, the error SPICE(NOFRAME) is signaled. A frame
///      name may fail to be recognized because a required frame
///      specification kernel has not been loaded; another cause is a
///      misspelling of the frame name.
///
///  6)  If the input frame FIXREF is not centered at the target body,
///      the error SPICE(INVALIDFRAME) is signaled.
///
///  7)  If the input argument METHOD cannot be parsed, an error
///      is signaled by either this routine or a routine in the
///      call tree of this routine.
///
///  8)  If insufficient ephemeris data have been loaded prior to
///      calling ILLUMF, an error is signaled by a
///      routine in the call tree of this routine. Note that when
///      light time correction is used, sufficient ephemeris data must
///      be available to propagate the states of observer, target, and
///      the illumination source to the solar system barycenter.
///
///  9)  If the computation method specifies an ellipsoidal target
///      shape and triaxial radii of the target body have not been
///      loaded into the kernel pool prior to calling ILLUMF, an error
///      is signaled by a routine in the call tree of this routine.
///
///  10) If PCK data specifying the target body-fixed frame orientation
///      have not been loaded prior to calling ILLUMF, an error is
///      signaled by a routine in the call tree of this routine.
///
///  11) If METHOD specifies that the target surface is represented by
///      DSK data, and no DSK files are loaded for the specified
///      target, an error is signaled by a routine in the call tree
///      of this routine.
///
///  12) If METHOD specifies that the target surface is represented by
///      DSK data, and data representing the portion of the surface on
///      which SPOINT is located are not available, an error is
///      signaled by a routine in the call tree of this routine.
///
///  13) If METHOD specifies that the target surface is represented
///      by DSK data, SPOINT must lie on the target surface, not above
///      or below it. A small tolerance is used to allow for round-off
///      error in the calculation determining whether SPOINT is on the
///      surface.
///
///      If, in the DSK case, SPOINT is too far from the surface, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///      If the surface is represented by a triaxial ellipsoid, SPOINT
///      is not required to be close to the ellipsoid; however, the
///      results computed by this routine will be unreliable if SPOINT
///      is too far from the ellipsoid.
///
///  14) If radii for TARGET are not found in the kernel pool, an error
///      is signaled by a routine in the call tree of this routine.
///
///  15) If the size of the TARGET body radii kernel variable is not
///      three, an error is signaled by a routine in the call tree of
///      this routine.
///
///  16) If any of the three TARGET body radii is less-than or equal to
///      zero, an error is signaled by a routine in the call tree of
///      this routine.
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
///  -  SPK data: ephemeris data for target, observer, and the
///     illumination source must be loaded. If aberration
///     corrections are used, the states of target, observer, and
///     the illumination source relative to the solar system
///     barycenter must be calculable from the available ephemeris
///     data. Typically ephemeris data are made available by loading
///     one or more SPK files via FURNSH.
///
///  -  PCK data: rotation data for the target body must be
///     loaded. These may be provided in a text or binary PCK file.
///
///  -  Shape data for the target body:
///
///        PCK data:
///
///           If the target body shape is modeled as an ellipsoid,
///           triaxial radii for the target body must be loaded into
///           the kernel pool. Typically this is done by loading a
///           text PCK file via FURNSH.
///
///           Triaxial radii are also needed if the target shape is
///           modeled by DSK data, but the DSK NADIR method is
///           selected.
///
///        DSK data:
///
///           If the target shape is modeled by DSK data, DSK files
///           containing topographic data for the target body must be
///           loaded. If a surface list is specified, data for at
///           least one of the listed surfaces must be loaded.
///
///  The following data may be required:
///
///  -  Frame data: if a frame definition is required to convert the
///     observer and target states to the body-fixed frame of the
///     target, that definition must be available in the kernel
///     pool. Typically the definition is supplied by loading a
///     frame kernel via FURNSH.
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
///  In all cases, kernel data are normally loaded once per program
///  run, NOT every time this routine is called.
/// ```
///
/// # Particulars
///
/// ```text
///  SPICELIB contains four routines that compute illumination angles:
///
///     ILLUMF   (this routine)
///
///     ILLUMG   (same as this routine, except that
///              output flags are not returned.)
///
///     ILUMIN   (same as ILLUMG, except that the sun is fixed
///              as the illumination source.)
///
///     ILLUM    (deprecated)
///
///  This routine is the most capable of the set.
///
///
///  Illumination angles
///  ===================
///
///  The term "illumination angles" refers to the following set of
///  angles:
///
///
///     phase angle              Angle between the vectors from the
///                              surface point to the observer and
///                              from the surface point to the
///                              illumination source.
///
///     incidence angle          Angle between the surface normal at
///                              the specified surface point and the
///                              vector from the surface point to the
///                              illumination source.
///
///     emission angle           Angle between the surface normal at
///                              the specified surface point and the
///                              vector from the surface point to the
///                              observer.
///
///  The diagram below illustrates the geometric relationships
///  defining these angles. The labels for the incidence, emission,
///  and phase angles are "inc.", "e.", and "phase".
///
///
///
///                                                   *
///                                           illumination source
///
///                 surface normal vector
///                           ._                 _.
///                           |\                 /|  illumination
///                             \    phase      /    source vector
///                              \   .    .    /
///                              .            .
///                                \   ___   /
///                           .     \/     \/
///                                 _\ inc./
///                          .    /   \   /
///                          .   |  e. \ /
///      *             <--------------- *  surface point on
///   viewing            vector            target body
///   location           to viewing
///   (observer)         location
///
///
///
///  Note that if the target-observer vector, the target normal vector
///  at the surface point, and the target-illumination source vector
///  are coplanar, then phase is the sum of the incidence and emission
///  angles. This rarely occurs; usually
///
///     phase angle  <  incidence angle + emission angle
///
///  All of the above angles can be computed using light time
///  corrections, light time and stellar aberration corrections, or no
///  aberration corrections. In order to describe apparent geometry as
///  observed by a remote sensing instrument, both light time and
///  stellar aberration corrections should be used.
///
///  The way aberration corrections are applied by this routine
///  is described below.
///
///     Light time corrections
///     ======================
///
///        Observer-target surface point vector
///        ------------------------------------
///
///        Let ET be the epoch at which an observation or remote
///        sensing measurement is made, and let ET - LT ("LT" stands
///        for "light time") be the epoch at which the photons
///        received at ET were emitted from the surface point SPOINT.
///        Note that the light time between the surface point and
///        observer will generally differ from the light time between
///        the target body's center and the observer.
///
///
///        Target body's orientation
///        -------------------------
///
///        Using the definitions of ET and LT above, the target body's
///        orientation at ET - LT is used. The surface normal is
///        dependent on the target body's orientation, so the body's
///        orientation model must be evaluated for the correct epoch.
///
///
///        Target body -- illumination source vector
///        -----------------------------------------
///
///        The surface features on the target body near SPOINT will
///        appear in a measurement made at ET as they were at ET-LT.
///        In particular, lighting on the target body is dependent on
///        the apparent location of the illumination source as seen
///        from the target body at ET-LT. So, a second light time
///        correction is used to compute the position of the
///        illumination source relative to the surface point.
///
///
///     Stellar aberration corrections
///     ==============================
///
///     Stellar aberration corrections are applied only if
///     light time corrections are applied as well.
///
///        Observer-target surface point body vector
///        -----------------------------------------
///
///        When stellar aberration correction is performed, the
///        direction vector SRFVEC is adjusted so as to point to the
///        apparent position of SPOINT: considering SPOINT to be an
///        ephemeris object, SRFVEC points from the observer's
///        position at ET to the light time and stellar aberration
///        corrected position of SPOINT.
///
///        Target body-illumination source vector
///        --------------------------------------
///
///        The target body-illumination source vector is the apparent
///        position of the illumination source, corrected for light
///        time and stellar aberration, as seen from the target body
///        at time ET-LT.
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
///     'DSK/UNPRIORITIZED/SURFACES = "Mars MEGDR 64 PIXEL/DEG", 3'
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
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Find the phase, solar incidence, and emission angles at the
///     sub-solar and sub-spacecraft points on Mars as seen from the
///     Mars Global Surveyor spacecraft at a specified UTC time. Use
///     light time and stellar aberration corrections.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File: illumf_ex1.tm
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
///           PROGRAM ILLUMF_EX1
///           IMPLICIT NONE
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      DPR
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         F1
///           PARAMETER           ( F1     = '(A,F15.9)' )
///
///           CHARACTER*(*)         F2
///           PARAMETER           ( F2     = '(A)' )
///
///           CHARACTER*(*)         F3
///           PARAMETER           ( F3     = '(A,2(2X,L))' )
///
///           CHARACTER*(*)         META
///           PARAMETER           ( META   = 'illumf_ex1.tm' )
///
///           INTEGER               NAMLEN
///           PARAMETER           ( NAMLEN = 32 )
///
///           INTEGER               TIMLEN
///           PARAMETER           ( TIMLEN = 25 )
///
///           INTEGER               CORLEN
///           PARAMETER           ( CORLEN = 5 )
///
///           INTEGER               MTHLEN
///           PARAMETER           ( MTHLEN = 50 )
///
///           INTEGER               NMETH
///           PARAMETER           ( NMETH  = 2 )
///     C
///     C     Local variables
///     C
///           CHARACTER*(CORLEN)    ABCORR
///           CHARACTER*(NAMLEN)    FIXREF
///           CHARACTER*(MTHLEN)    ILUMTH ( NMETH )
///           CHARACTER*(NAMLEN)    OBSRVR
///           CHARACTER*(MTHLEN)    SUBMTH ( NMETH )
///           CHARACTER*(NAMLEN)    TARGET
///           CHARACTER*(TIMLEN)    UTC
///
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      SRFVEC ( 3 )
///           DOUBLE PRECISION      SSCEMI
///           DOUBLE PRECISION      SSCPHS
///           DOUBLE PRECISION      SSCPT  ( 3 )
///           DOUBLE PRECISION      SSCSOL
///           DOUBLE PRECISION      SSLEMI
///           DOUBLE PRECISION      SSLPHS
///           DOUBLE PRECISION      SSLSOL
///           DOUBLE PRECISION      SSOLPT ( 3 )
///           DOUBLE PRECISION      TRGEPC
///
///           INTEGER               I
///
///           LOGICAL               SSCLIT
///           LOGICAL               SSCVIS
///           LOGICAL               SSLLIT
///           LOGICAL               SSLVIS
///
///     C
///     C     Initial values
///     C
///           DATA                  ILUMTH / 'Ellipsoid',
///          .                               'DSK/Unprioritized' /
///
///           DATA                  SUBMTH / 'Near Point/Ellipsoid',
///          .                            'DSK/Nadir/Unprioritized' /
///
///     C
///     C     Load kernel files.
///     C
///           CALL FURNSH ( META )
///     C
///     C     Convert the UTC request time string to seconds past
///     C     J2000 TDB.
///     C
///           UTC = '2003 OCT 13 06:00:00 UTC'
///
///           CALL UTC2ET ( UTC, ET )
///
///           WRITE (*,F2) ' '
///           WRITE (*,F2) 'UTC epoch is '//UTC
///     C
///     C     Assign observer and target names. The acronym MGS
///     C     indicates Mars Global Surveyor. See NAIF_IDS for a
///     C     list of names recognized by SPICE. Also set the
///     C     aberration correction flag.
///     C
///           TARGET = 'Mars'
///           OBSRVR = 'MGS'
///           FIXREF = 'IAU_MARS'
///           ABCORR = 'CN+S'
///
///           DO I = 1, NMETH
///     C
///     C        Find the sub-solar point on Mars as
///     C        seen from the MGS spacecraft at ET. Use the
///     C        "near point" style of sub-point definition
///     C        when the shape model is an ellipsoid, and use
///     C        the "nadir" style when the shape model is
///     C        provided by DSK data. This makes it easy to
///     C        verify the solar incidence angle when
///     C        the target is modeled as an  ellipsoid.
///     C
///              CALL SUBSLR ( SUBMTH(I),  TARGET,  ET,
///          .                 FIXREF,     ABCORR,  OBSRVR,
///          .                 SSOLPT,     TRGEPC,  SRFVEC  )
///     C
///     C        Now find the sub-spacecraft point.
///     C
///              CALL SUBPNT ( SUBMTH(I),  TARGET,  ET,
///          .                 FIXREF,     ABCORR,  OBSRVR,
///          .                 SSCPT,      TRGEPC,  SRFVEC )
///     C
///     C        Find the phase, solar incidence, and emission
///     C        angles at the sub-solar point on Mars as
///     C        seen from MGS at time ET.
///     C
///              CALL ILLUMF ( ILUMTH(I), TARGET,  'SUN',
///          .                 ET,        FIXREF,  ABCORR,
///          .                 OBSRVR,    SSOLPT,  TRGEPC,
///          .                 SRFVEC,    SSLPHS,  SSLSOL,
///          .                 SSLEMI,    SSLVIS,  SSLLIT )
///     C
///     C        Do the same for the sub-spacecraft point.
///     C
///              CALL ILLUMF ( ILUMTH(I), TARGET,  'SUN',
///          .                 ET,        FIXREF,  ABCORR,
///          .                 OBSRVR,    SSCPT,   TRGEPC,
///          .                 SRFVEC,    SSCPHS,  SSCSOL,
///          .                 SSCEMI,    SSCVIS,  SSCLIT  )
///     C
///     C        Convert the angles to degrees and write them out.
///     C
///              SSLPHS = DPR() * SSLPHS
///              SSLSOL = DPR() * SSLSOL
///              SSLEMI = DPR() * SSLEMI
///
///              SSCPHS = DPR() * SSCPHS
///              SSCSOL = DPR() * SSCSOL
///              SSCEMI = DPR() * SSCEMI
///
///              WRITE (*,F2) ' '
///              WRITE (*,F2) '   ILLUMF method: '//ILUMTH(I)
///              WRITE (*,F2) '   SUBPNT method: '//SUBMTH(I)
///              WRITE (*,F2) '   SUBSLR method: '//SUBMTH(I)
///              WRITE (*,F2) ' '
///              WRITE (*,F2) '      Illumination angles at the '
///          .   //           'sub-solar point:'
///              WRITE (*,F2) ' '
///
///              WRITE (*,F1) '      Phase angle           (deg.): ',
///          .                SSLPHS
///              WRITE (*,F1) '      Solar incidence angle (deg.): ',
///          .                SSLSOL
///              WRITE (*,F1) '      Emission angle        (deg.): ',
///          .                SSLEMI
///              WRITE (*,F3) '      Visible, Lit flags:           ',
///          .                SSLVIS, SSLLIT
///              WRITE (*,F2) ' '
///
///              IF ( I .EQ. 1 ) THEN
///                 WRITE (*,F2) '        The solar incidence angle '
///          .      //           'should be 0.'
///                 WRITE (*,F2) '        The emission and phase '
///          .      //           'angles should be equal.'
///                 WRITE (*,F2) ' '
///              END IF
///
///
///              WRITE (*,F2) '      Illumination angles at the '
///          .   //          'sub-s/c point:'
///              WRITE (*,F2) ' '
///              WRITE (*,F1) '      Phase angle           (deg.): ',
///          .               SSCPHS
///              WRITE (*,F1) '      Solar incidence angle (deg.): ',
///          .               SSCSOL
///              WRITE (*,F1) '      Emission angle        (deg.): ',
///          .               SSCEMI
///              WRITE (*,F3) '      Visible, Lit flags:           ',
///          .                SSCVIS, SSCLIT
///              WRITE (*,F2) ' '
///
///              IF ( I .EQ. 1 ) THEN
///                 WRITE (*,F2) '        The emission angle '
///          .      //           'should be 0.'
///                 WRITE (*,F2) '        The solar incidence '
///          .      //           'and phase angles should be equal.'
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
///     UTC epoch is 2003 OCT 13 06:00:00 UTC
///
///        ILLUMF method: Ellipsoid
///        SUBPNT method: Near Point/Ellipsoid
///        SUBSLR method: Near Point/Ellipsoid
///
///           Illumination angles at the sub-solar point:
///
///           Phase angle           (deg.):   138.370270685
///           Solar incidence angle (deg.):     0.000000000
///           Emission angle        (deg.):   138.370270685
///           Visible, Lit flags:             F  T
///
///             The solar incidence angle should be 0.
///             The emission and phase angles should be equal.
///
///           Illumination angles at the sub-s/c point:
///
///           Phase angle           (deg.):   101.439331040
///           Solar incidence angle (deg.):   101.439331041
///           Emission angle        (deg.):     0.000000002
///           Visible, Lit flags:             T  F
///
///             The emission angle should be 0.
///             The solar incidence and phase angles should be equal.
///
///        ILLUMF method: DSK/Unprioritized
///        SUBPNT method: DSK/Nadir/Unprioritized
///        SUBSLR method: DSK/Nadir/Unprioritized
///
///           Illumination angles at the sub-solar point:
///
///           Phase angle           (deg.):   138.387071677
///           Solar incidence angle (deg.):     0.967122745
///           Emission angle        (deg.):   137.621480599
///           Visible, Lit flags:             F  T
///
///           Illumination angles at the sub-s/c point:
///
///           Phase angle           (deg.):   101.439331359
///           Solar incidence angle (deg.):   101.555993667
///           Emission angle        (deg.):     0.117861156
///           Visible, Lit flags:             T  F
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  Results from this routine are not meaningful if the input
///      point lies on a ridge or vertex of a surface represented by
///      DSK data, or if for any other reason the direction of the
///      outward normal vector at the point is undefined.
///
///  2)  The illumination state indicated by the output argument `lit'
///      is computed treating the illumination source as a single
///      point. Surface points that are illuminated by part of the
///      source are classified as "lit" or not depending on whether the
///      center of the source is visible from those points.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 20-NOV-2021 (JDR) (EDW) (NJB)
///
///         Bug fix: PRVCOR is no longer set to blank before
///         ABCORR is parsed.
///
///         Body radii accessed from kernel pool using ZZGFTREB.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.0.0, 04-APR-2017 (NJB) (BVS)
///
///         07-APR-2016 (NJB) (BVS)
///
///         Now uses surface mapping tracking capability.
///
///         Updated surface ID codes in header comments.
///
///        30-MAR-2015 (NJB)
///
///         Now uses illumination angles to determine whether
///         self-intersection tests are necessary, for the DSK
///         case. Now imports SHPLEN parameter from gf.inc.
///
///         Original version 09-FEB-2015 (NJB) (BVS)
/// ```
pub fn illumf(
    ctx: &mut SpiceContext,
    method: &str,
    target: &str,
    ilusrc: &str,
    et: f64,
    fixref: &str,
    abcorr: &str,
    obsrvr: &str,
    spoint: &[f64; 3],
    trgepc: &mut f64,
    srfvec: &mut [f64; 3],
    phase: &mut f64,
    incdnc: &mut f64,
    emissn: &mut f64,
    visibl: &mut bool,
    lit: &mut bool,
) -> crate::Result<()> {
    ILLUMF(
        method.as_bytes(),
        target.as_bytes(),
        ilusrc.as_bytes(),
        et,
        fixref.as_bytes(),
        abcorr.as_bytes(),
        obsrvr.as_bytes(),
        spoint,
        trgepc,
        srfvec,
        phase,
        incdnc,
        emissn,
        visibl,
        lit,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure ILLUMF ( Illumination angles, general source, return flags )
pub fn ILLUMF(
    METHOD: &[u8],
    TARGET: &[u8],
    ILUSRC: &[u8],
    ET: f64,
    FIXREF: &[u8],
    ABCORR: &[u8],
    OBSRVR: &[u8],
    SPOINT: &[f64],
    TRGEPC: &mut f64,
    SRFVEC: &mut [f64],
    PHASE: &mut f64,
    INCDNC: &mut f64,
    EMISSN: &mut f64,
    VISIBL: &mut bool,
    LIT: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let SPOINT = DummyArray::new(SPOINT, 1..=3);
    let mut SRFVEC = DummyArrayMut::new(SRFVEC, 1..=3);
    let mut PNTDEF = [b' '; CVTLEN as usize];
    let mut SHPSTR = [b' '; SHPLEN as usize];
    let mut SUBTYP = [b' '; SUBLEN as usize];
    let mut TRMSTR = [b' '; TMTLEN as usize];
    let mut LT: f64 = 0.0;
    let mut LTI: f64 = 0.0;
    let mut MAXRAD: f64 = 0.0;
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
    let mut OBSPOS = StackArray::<f64, 3>::new(1..=3);
    let mut OPSTAT = StackArray::<f64, 6>::new(1..=6);
    let mut RADII = StackArray::<f64, 3>::new(1..=3);
    let mut S: f64 = 0.0;
    let mut SCALE: f64 = 0.0;
    let mut TISTAT = StackArray::<f64, 6>::new(1..=6);
    let mut VERTEX = StackArray::<f64, 3>::new(1..=3);
    let mut XPT = StackArray::<f64, 3>::new(1..=3);
    let mut FIXFID: i32 = 0;
    let mut OBSCDE: i32 = 0;
    let mut SVNSRF: i32 = 0;
    let mut TYPE: i32 = 0;
    let mut TYPEID: i32 = 0;
    let mut ATTBLK = StackArray::<bool, 15>::new(1..=NABCOR);
    let mut FND: bool = false;
    let mut FOUND: bool = false;
    let mut SURFUP: bool = false;

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
    // Note: XMIT need not be saved, since it's used only
    // for error checking when an aberration correction flag
    // is parsed.
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
    // Counter initialization is done separately.
    //
    if save.FIRST {
        //
        // Initialize counters.
        //
        ZZCTRUIN(save.SVCTR1.as_slice_mut(), ctx);
        ZZCTRUIN(save.SVCTR2.as_slice_mut(), ctx);
        ZZCTRUIN(save.SVCTR3.as_slice_mut(), ctx);
    }

    //
    // If necessary, parse the aberration correction flag.
    //
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

        // Set logical flags indicating the attributes of the requested
        // correction:
        //
        //    XMIT is .TRUE. when the correction is for transmitted
        //    radiation.
        //
        //    USELT is .TRUE. when any type of light time correction
        //    (normal or converged Newtonian) is specified.
        //
        //    USESTL indicates stellar aberration corrections.
        //
        //
        // The above definitions are consistent with those used by
        // ZZVALCOR.
        //
        save.XMIT = ATTBLK[XMTIDX];
        save.USELT = ATTBLK[LTIDX];

        //
        // The aberration correction flag is recognized; save it.
        //
        fstr::assign(&mut save.PRVCOR, ABCORR);
        //
        // We do NOT set FIRST to .FALSE. here, since we're not
        // yet done with it.
        //
    }

    //
    // Get the target ID code here, since it will be needed
    // for the initialization calls below.
    //
    ZZBODS2C(
        save.SVCTR1.as_slice_mut(),
        &mut save.SVTARG,
        &mut save.SVTCDE,
        &mut save.SVFND1,
        TARGET,
        &mut save.TRGCDE,
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

    //
    // Check whether the surface name/ID mapping has been updated.
    //
    ZZSRFTRK(save.SVCTR4.as_slice_mut(), &mut SURFUP, ctx)?;

    //
    // If necessary, parse the method specification. PRVMTH
    // and the derived flags NEAR and ELIPSD start out with
    // valid values. PRVMTH records the last valid value of
    // METHOD; ELIPSD is the corresponding shape flag.
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
            save.TRGCDE,
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
            SETMSG(b"Returned shape value from method string was <#>.", ctx);
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
            SETMSG(b"Spurious sub-observer point type <#> was present in the method string #. The sub-observer type is valid in the method strings for SUBPNT and SUBSLR, but is not applicable for ILLUMF.", ctx);
            ERRCH(b"#", &SUBTYP, ctx);
            ERRCH(b"#", METHOD, ctx);
            SIGERR(b"SPICE(INVALIDMETHOD)", ctx)?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        fstr::assign(&mut save.PRVMTH, METHOD);
    }

    //
    // We're done with all tasks that must be executed on the first
    // pass.
    //
    save.FIRST = false;

    //
    // Obtain integer codes for the observer and
    // illumination source.
    //
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
    // Check the observer and target body codes. If they are equal,
    // signal an error.
    //
    if (OBSCDE == save.TRGCDE) {
        SETMSG(b"In computing illumination angles, the observing body and target body are the same. Both are #.", ctx);
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
        &mut save.SVREFC,
        FIXREF,
        &mut FIXFID,
        ctx,
    )?;

    FRINFO(
        FIXFID,
        &mut save.CENTER,
        &mut TYPE,
        &mut TYPEID,
        &mut FND,
        ctx,
    )?;

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
    if (save.CENTER != save.TRGCDE) {
        SETMSG(b"Reference frame # is not centered at the target body #. The ID code of the frame center is #.", ctx);
        ERRCH(b"#", FIXREF, ctx);
        ERRCH(b"#", TARGET, ctx);
        ERRINT(b"#", save.CENTER, ctx);
        SIGERR(b"SPICE(INVALIDFRAME)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Get the sign S prefixing LT in the expression for TRGEPC.
    // When light time correction is not used, setting S = 0
    // allows us to seamlessly set TRGEPC equal to ET.
    //
    if save.USELT {
        if save.XMIT {
            S = 1.0;
        } else {
            S = -1.0;
        }
    } else {
        S = 0.0;
    }

    //
    // Look up the state of the surface point relative to the observer.
    // The body-fixed frame of the surface point is to be evaluated
    // at the epoch of the surface point, not at the epoch of the
    // center of the frame; we indicate this by setting the input
    // argument REFLOC to 'TARGET'.
    //
    SPKCPT(
        SPOINT.as_slice(),
        TARGET,
        FIXREF,
        ET,
        FIXREF,
        b"TARGET",
        ABCORR,
        OBSRVR,
        OPSTAT.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // TRGEPC is the epoch associated with the surface point. Below,
    // since S is set to 0.D0 if no aberration corrections are used, we
    // require no logical branch.
    //
    *TRGEPC = (ET + (S * LT));

    //
    // Now find the state of the illumination source as seen by
    // the surface point at TRGEPC. We want to evaluate the orientation
    // of the body-fixed frame of the surface point at the epoch
    // associated with the surface point, not at the epoch associated
    // with the frame's center;  we indicate this by setting the input
    // argument REFLOC to 'OBSERVER'.
    //
    SPKCPO(
        ILUSRC,
        *TRGEPC,
        FIXREF,
        b"OBSERVER",
        ABCORR,
        SPOINT.as_slice(),
        TARGET,
        FIXREF,
        TISTAT.as_slice_mut(),
        &mut LTI,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // We'll need the negative of the observer-surface point position
    // for the following angle computation. Set the output SRFVEC while
    // we're at it.
    //
    VEQU(OPSTAT.as_slice(), SRFVEC.as_slice_mut());
    VMINUS(SRFVEC.as_slice(), OBSPOS.as_slice_mut());

    //
    // Find the surface normal at SPOINT. This computation depends
    // on target body shape model.
    //
    if (save.SHAPE == ELLSHP) {
        //
        // We'll need the radii of the target body.
        //
        ZZGFTREB(save.TRGCDE, RADII.as_slice_mut(), ctx)?;

        SURFNM(
            RADII[1],
            RADII[2],
            RADII[3],
            SPOINT.as_slice(),
            NORMAL.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }
    } else if (save.SHAPE == DSKSHP) {
        //
        // Initialize the normal vector algorithm to use a DSK model
        // for the surface of the target body. This initialization is
        // required to enable later use of ZZRAYSFX and ZZMAXRAD.
        //
        SVNSRF = 0;

        ZZSUDSKI(save.TRGCDE, SVNSRF, save.SRFLST.as_slice(), FIXFID, ctx)?;

        //
        // Compute the outward unit normal at SPOINT on the surface
        // defined by the designated DSK data.
        //
        ZZSBFNRM(
            save.TRGCDE,
            SVNSRF,
            save.SRFLST.as_slice(),
            *TRGEPC,
            FIXFID,
            SPOINT.as_slice(),
            NORMAL.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        VHATIP(NORMAL.as_slice_mut());
    } else {
        //
        // We've already checked the computation method input argument,
        // so we don't expect to arrive here. This code is present for
        // safety.
        //
        SETMSG(b"The computation method # was not recognized. ", ctx);
        ERRCH(b"#", METHOD, ctx);
        SIGERR(b"SPICE(INVALIDMETHOD)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Check for errors before calling math routines.
    //

    //
    // Find the illumination angles. VSEP will give us angular
    // separation in radians.
    //
    *PHASE = VSEP(OBSPOS.as_slice(), TISTAT.as_slice(), ctx);
    *INCDNC = VSEP(NORMAL.as_slice(), TISTAT.as_slice(), ctx);
    *EMISSN = VSEP(NORMAL.as_slice(), OBSPOS.as_slice(), ctx);

    //
    // Set the visibility and illumination flags.
    //
    //
    // Set default values of VISIBL and LIT.
    //
    *VISIBL = (*EMISSN <= HALFPI(ctx));
    *LIT = (*INCDNC <= HALFPI(ctx));

    if (save.SHAPE == DSKSHP) {
        //
        // There is a possibility that the surface-observer vector
        // or the surface-illumination source vector intersects the
        // surface. This is possible only if these vectors have
        // positive elevation.

        if (*LIT || *VISIBL) {
            //
            // We need to check for self-occultation of at least one of
            // the lines-of-sight from surface to observer or illumination
            // source.
            //
            // We'll produce a point slightly above the surface point,
            // from which the visibility of the observer and light source
            // can be determined. We want to avoid detection of spurious
            // intersections near the input surface point.
            //
            // Obtain an upper bound on the target body radius.
            //
            ZZMAXRAD(&mut MAXRAD, ctx);

            if FAILED(ctx) {
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            //
            // Compute an offset that can be used to produce a point
            // slightly above the surface point, and compute the "raised"
            // point. NORMAL is a unit vector here.
            //
            SCALE = (TOLSCL * MAXRAD);

            VLCOM(
                1.0,
                SPOINT.as_slice(),
                SCALE,
                NORMAL.as_slice(),
                VERTEX.as_slice_mut(),
            );

            if *VISIBL {
                //
                // Find the surface intercept, if any, of a ray emanating
                // from VERTEX and pointing toward the observer.
                //
                ZZRAYSFX(
                    VERTEX.as_slice(),
                    OBSPOS.as_slice(),
                    *TRGEPC,
                    XPT.as_slice_mut(),
                    &mut FOUND,
                    ctx,
                )?;

                *VISIBL = !FOUND;
            }

            if *LIT {
                //
                // Find the surface intercept, if any, of a ray emanating
                // from VERTEX and pointing toward the illumination source.
                //
                ZZRAYSFX(
                    VERTEX.as_slice(),
                    TISTAT.as_slice(),
                    *TRGEPC,
                    XPT.as_slice_mut(),
                    &mut FOUND,
                    ctx,
                )?;

                *LIT = !FOUND;
            }
        }
    }

    //
    // TRGEPC and SRVFEC were already set before the illumination
    // angle computation.
    //
    CHKOUT(RNAME, ctx)?;
    Ok(())
}
