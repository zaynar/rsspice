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
const RNAME: &[u8] = b"SUBSLR";
const CNVLIM: f64 = 0.00000000000000001;
const MAXITR: i32 = 10;
const SUN: i32 = 10;
const MAXL: i32 = 36;
const FRNMLN: i32 = 32;

struct SaveVars {
    PRVCOR: Vec<u8>,
    PRVMTH: Vec<u8>,
    NSURF: i32,
    SHAPE: i32,
    SRFLST: StackArray<i32, 100>,
    FIRST: bool,
    NEAR: bool,
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
    SVREFC: i32,
    SVCTR4: StackArray<i32, 2>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PRVCOR = vec![b' '; CORLEN as usize];
        let mut PRVMTH = vec![b' '; MTHLEN as usize];
        let mut NSURF: i32 = 0;
        let mut SHAPE: i32 = 0;
        let mut SRFLST = StackArray::<i32, 100>::new(1..=MAXSRF);
        let mut FIRST: bool = false;
        let mut NEAR: bool = false;
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
        let mut SVREFC: i32 = 0;
        let mut SVCTR4 = StackArray::<i32, 2>::new(1..=CTRSIZ);

        FIRST = true;
        NEAR = true;
        fstr::assign(&mut PRVCOR, b" ");
        fstr::assign(&mut PRVMTH, b" ");
        SHAPE = ELLSHP;

        Self {
            PRVCOR,
            PRVMTH,
            NSURF,
            SHAPE,
            SRFLST,
            FIRST,
            NEAR,
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
            SVREFC,
            SVCTR4,
        }
    }
}

/// Sub-solar point
///
/// Compute the rectangular coordinates of the sub-solar point on
/// a target body at a specified epoch, optionally corrected for
/// light time and stellar aberration.
///
/// The surface of the target body may be represented by a triaxial
/// ellipsoid or by topographic data provided by DSK files.
///
/// This routine supersedes SUBSOL.
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
///  ET         I   Epoch in ephemeris seconds past J2000 TDB.
///  FIXREF     I   Body-fixed, body-centered target body frame.
///  ABCORR     I   Aberration correction.
///  OBSRVR     I   Name of observing body.
///  SPOINT     O   Sub-solar point on the target body.
///  TRGEPC     O   Sub-solar point epoch.
///  SRFVEC     O   Vector from observer to sub-solar point.
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
///              'NEAR POINT/ELLIPSOID'
///
///                 The sub-solar point computation uses a triaxial
///                 ellipsoid to model the surface of the target body.
///                 The sub-solar point is defined as the nearest
///                 point on the target relative to the sun.
///
///                 The word "NADIR" may be substituted for the phrase
///                 "NEAR POINT" in the string above.
///
///                 For backwards compatibility, the older syntax
///
///                    'Near point: ellipsoid'
///
///                 is accepted as well.
///
///
///              'INTERCEPT/ELLIPSOID'
///
///                 The sub-solar point computation uses a triaxial
///                 ellipsoid to model the surface of the target body.
///                 The sub-solar point is defined as the target
///                 surface intercept of the line containing the sun
///                 and the target's center.
///
///                 For backwards compatibility, the older syntax
///
///                    'Intercept: ellipsoid'
///
///                 is accepted as well.
///
///
///              'NADIR/DSK/UNPRIORITIZED[/SURFACES = <surface list>]'
///
///                 The sub-solar point computation uses DSK data to
///                 model the surface of the target body. The
///                 sub-solar point is defined as the intercept, on
///                 the surface represented by the DSK data, of the
///                 line containing the sun and the nearest point on
///                 the target's reference ellipsoid. If multiple such
///                 intercepts exist, the one closest to the sun is
///                 selected.
///
///                 Note that this definition of the sub-solar point
///                 is not equivalent to the "nearest point on the
///                 surface to the sun." The phrase "NEAR POINT" may
///                 NOT be substituted for "NADIR" in the string
///                 above.
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
///              'INTERCEPT/DSK/UNPRIORITIZED[/SURFACES =
///                                           <surface list>]'
///
///                 The sub-solar point computation uses DSK data to
///                 model the surface of the target body. The
///                 sub-solar point is defined as the target surface
///                 intercept of the line containing the sun and the
///                 target's center.
///
///                 If multiple such intercepts exist, the one closest
///                 to the sun is selected.
///
///                 The surface list specification is optional. The
///                 syntax of the list is identical to that for the
///                 NADIR option described above.
///
///
///              Neither case nor white space are significant in
///              METHOD, except within double-quoted strings. For
///              example, the string ' eLLipsoid/nearpoint ' is valid.
///
///              Within double-quoted strings, blank characters are
///              significant, but multiple consecutive blanks are
///              considered equivalent to a single blank. Case is
///              not significant. So
///
///                 "Mars MEGDR 128 PIXEL/DEG"
///
///              is equivalent to
///
///                 " mars megdr  128  pixel/deg "
///
///              but not to
///
///                 "MARS MEGDR128PIXEL/DEG"
///
///
///  TARGET   is the name of the target body. The target body is
///           an ephemeris object (its trajectory is given by
///           SPK data), and is an extended object.
///
///           The string TARGET is case-insensitive, and leading
///           and trailing blanks in TARGET are not significant.
///           Optionally, you may supply a string containing the
///           integer ID code for the object. For example both
///           'MOON' and '301' are legitimate strings that indicate
///           the Moon is the target body.
///
///           When the target body's surface is represented by a
///           tri-axial ellipsoid, this routine assumes that a
///           kernel variable representing the ellipsoid's radii is
///           present in the kernel pool. Normally the kernel
///           variable would be defined by loading a PCK file.
///
///
///  ET       is the epoch of participation of the observer,
///           expressed as ephemeris seconds past J2000 TDB: ET is
///           the epoch at which the observer's state is computed.
///
///           When aberration corrections are not used, ET is also
///           the epoch at which the position and orientation of
///           the target body and the position of the Sun are
///           computed.
///
///           When aberration corrections are used, ET is the epoch
///           at which the observer's state relative to the solar
///           system barycenter is computed; in this case the
///           position and orientation of the target body are
///           computed at ET-LT, where LT is the one-way light time
///           between the sub-solar point and the observer. See the
///           description of ABCORR below for details.
///
///
///  FIXREF   is the name of a body-fixed reference frame centered
///           on the target body. FIXREF may be any such frame
///           supported by the SPICE system, including built-in
///           frames (documented in the Frames Required Reading)
///           and frames defined by a loaded frame kernel (FK). The
///           string FIXREF is case-insensitive, and leading and
///           trailing blanks in FIXREF are not significant.
///
///           The output sub-solar point SPOINT and the
///           observer-to-sub-solar point vector SRFVEC will be
///           expressed relative to this reference frame.
///
///
///  ABCORR   indicates the aberration correction to be applied
///           when computing the target position and orientation
///           and the position of the Sun.
///
///           For remote sensing applications, where the apparent
///           sub-solar point seen by the observer is desired,
///           normally either of the corrections
///
///              'LT+S'
///              'CN+S'
///
///           should be used. These and the other supported options
///           are described below. ABCORR may be any of the
///           following:
///
///              'NONE'     Apply no correction. Return the
///                         geometric sub-solar point on the target
///                         body.
///
///           Let LT represent the one-way light time between the
///           observer and the sub-solar point (note: NOT between
///           the observer and the target body's center). The
///           following values of ABCORR apply to the "reception"
///           case in which photons depart from the sub-solar
///           point's location at the light-time corrected epoch
///           ET-LT and *arrive* at the observer's location at ET:
///
///              'LT'       Correct for one-way light time (also
///                         called "planetary aberration") using a
///                         Newtonian formulation. This correction
///                         yields the location of sub-solar
///                         point at the moment it emitted photons
///                         arriving at the observer at ET.
///
///                         The light time correction uses an
///                         iterative solution of the light time
///                         equation. The solution invoked by the
///                         'LT' option uses one iteration.
///
///                         The target position and orientation as
///                         seen by the observer are corrected for
///                         light time. The position of the Sun
///                         relative to the target is corrected for
///                         one-way light time between the Sun and
///                         target.
///
///              'LT+S'     Correct for one-way light time and
///                         stellar aberration using a Newtonian
///                         formulation. This option modifies the
///                         sub-solar point obtained with the 'LT'
///                         option to account for the observer's
///                         velocity relative to the solar system
///                         barycenter. These corrections yield
///                         the apparent sub-solar point.
///
///              'CN'       Converged Newtonian light time
///                         correction. In solving the light time
///                         equation, the 'CN' correction iterates
///                         until the solution converges. Both the
///                         position and rotation of the target
///                         body, and the position of the Sun, are
///                         corrected for light time.
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
///           Neither case nor white space are significant in
///           ABCORR. For example, the string
///
///             'Lt + s'
///
///           is valid.
///
///
///  OBSRVR   is the name of the observing body. The observing body
///           is an ephemeris object: it typically is a spacecraft,
///           the earth, or a surface point on the earth. OBSRVR is
///           case-insensitive, and leading and trailing blanks in
///           OBSRVR are not significant. Optionally, you may
///           supply a string containing the integer ID code for
///           the object. For example both 'MOON' and '301' are
///           legitimate strings that indicate the Moon is the
///           observer.
///
///           The observer may coincide with the target.
/// ```
///
/// # Detailed Output
///
/// ```text
///  SPOINT   is the sub-solar point on the target body.
///
///           For target shapes modeled by ellipsoids, the
///           sub-solar point is defined either as the point on the
///           target body that is closest to the sun, or the target
///           surface intercept of the line from the sun to the
///           target's center.
///
///           For target shapes modeled by topographic data
///           provided by DSK files, the sub-solar point is defined
///           as the target surface intercept of the line from the
///           sun to either the nearest point on the reference
///           ellipsoid, or to the target's center. If multiple
///           such intercepts exist, the one closest to the sun is
///           selected.
///
///           The input argument METHOD selects the target shape
///           model and sub-solar point definition to be used.
///
///           SPOINT is expressed in Cartesian coordinates,
///           relative to the body-fixed target frame designated by
///           FIXREF. The body-fixed target frame is evaluated at
///           the sub-solar point epoch TRGEPC (see description
///           below).
///
///           When aberration corrections are used, SPOINT is
///           computed using target body position and orientation
///           that have been adjusted for the corrections
///           applicable to SPOINT itself rather than to the target
///           body's center. In particular, if the stellar
///           aberration correction applicable to SPOINT is
///           represented by a shift vector S, then the light-time
///           corrected position of the target is shifted by S
///           before the sub-solar point is computed.
///
///           The components of SPOINT have units of km.
///
///
///  TRGEPC   is the "sub-solar point epoch." TRGEPC is defined as
///           follows: letting LT be the one-way light time between
///           the observer and the sub-solar point, TRGEPC is
///           either the epoch ET-LT or ET depending on whether the
///           requested aberration correction is, respectively, for
///           received radiation or omitted. LT is computed using
///           the method indicated by ABCORR.
///
///           TRGEPC is expressed as seconds past J2000 TDB.
///
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
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the specified aberration correction is unrecognized, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  2)  If transmission aberration corrections are specified, the
///      error SPICE(NOTSUPPORTED) is signaled.
///
///  3)  If either the target or observer input strings cannot be
///      converted to an integer ID code, the error
///      SPICE(IDCODENOTFOUND) is signaled.
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
///  6)  If the input argument METHOD is not recognized, the error
///      SPICE(INVALIDMETHOD) is signaled by this routine, or, the
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  7)  If the sub-solar point type is not specified or is not
///      recognized, the error SPICE(INVALIDSUBTYPE) is signaled.
///
///  8)  If insufficient ephemeris data have been loaded prior to
///      calling SUBSLR, an error is signaled by a
///      routine in the call tree of this routine. Note that when
///      light time correction is used, sufficient ephemeris data must
///      be available to propagate the states of observer, target, and
///      the Sun to the solar system barycenter.
///
///  9)  If the computation method specifies an ellipsoidal target
///      shape and triaxial radii of the target body have not been
///      loaded into the kernel pool prior to calling SUBSLR, an error
///      is signaled by a routine in the call tree of this routine.
///
///  10) The target must be an extended body, and must have a shape
///      for which a sub-solar point can be defined.
///
///      If the target body's shape is modeled by DSK data, the shape
///      must be such that the specified sub-solar point definition is
///      applicable. For example, if the target shape is a torus, both
///      the NADIR and INTERCEPT definitions might be inapplicable,
///      depending on the relative locations of the sun and target.
///
///  11) If PCK data specifying the target body-fixed frame orientation
///      have not been loaded prior to calling SUBSLR, an error is
///      signaled by a routine in the call tree of this routine.
///
///  12) If METHOD specifies that the target surface is represented by
///      DSK data, and no DSK files are loaded for the specified
///      target, an error is signaled by a routine in the call tree
///      of this routine.
///
///  13) If METHOD specifies that the target surface is represented
///      by DSK data, and the ray from the observer to the
///      sub-observer point doesn't intersect the target body's
///      surface, the error SPICE(SUBPOINTNOTFOUND) is signaled.
///
///  14) If the surface intercept on the target body's reference
///      ellipsoid of the observer to target center vector cannot not
///      be computed, the error SPICE(DEGENERATECASE) is signaled. Note
///      that this is a very rare case.
///
///  15) If the target body is the sun, the error SPICE(INVALIDTARGET)
///      is signaled.
///
///  16) If radii for TARGET are not found in the kernel pool, an error
///      is signaled by a routine in the call tree of this routine.
///
///  17) If the size of the TARGET body radii kernel variable is not
///      three, an error is signaled by a routine in the call tree of
///      this routine.
///
///  18) If any of the three TARGET body radii is less-than or equal to
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
///  -  SPK data: ephemeris data for target, observer, and Sun must
///     be loaded. If aberration corrections are used, the states of
///     target, observer, and the Sun relative to the solar system
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
///  There are two different popular ways to define the sub-solar
///  point: "nearest point on target to the Sun" or "target surface
///  intercept of the line containing the Sun and target." These
///  coincide when the target is spherical and generally are distinct
///  otherwise.
///
///  This routine computes light time corrections using light time
///  between the observer and the sub-solar point, as opposed to the
///  center of the target. Similarly, stellar aberration corrections
///  done by this routine are based on the direction of the vector
///  from the observer to the light-time corrected sub-solar point,
///  not to the target center. This technique avoids errors due to the
///  differential between aberration corrections across the target
///  body. Therefore it's valid to use aberration corrections with
///  this routine even when the observer is very close to the
///  sub-solar point, in particular when the observer to sub-solar
///  point distance is much less than the observer to target center
///  distance.
///
///  When comparing sub-solar point computations with results from
///  sources other than SPICE, it's essential to make sure the same
///  geometric definitions are used.
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
///        NADIR/DSK/UNPRIORITIZED/<surface list>
///        DSK/NADIR/<surface list>/UNPRIORITIZED
///        UNPRIORITIZED/<surface list>/DSK/NADIR
///
///     The simplest form of the METHOD argument specifying use of
///     DSK data is one that lacks a surface list, for example:
///
///        'NADIR/DSK/UNPRIORITIZED'
///        'INTERCEPT/DSK/UNPRIORITIZED'
///
///     For applications in which all loaded DSK data for the target
///     body are for a single surface, and there are no competing
///     segments, the above strings suffice. This is expected to be
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
///     'NADIR/DSK/UNPRIORITIZED/SURFACES= "Mars MEGDR 64 PIXEL/DEG",3'
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
///  platforms. The results depend on the SPICE kernels used as input,
///  the compiler and supporting libraries, and the machine specific
///  arithmetic implementation.
///
///
///  1) Find the sub-solar point on Mars as seen from the Earth for a
///     specified time.
///
///     Compute the sub-solar point using both triaxial ellipsoid
///     and topographic surface models. Topography data are provided by
///     a DSK file. For the ellipsoid model, use both the "intercept"
///     and "near point" sub-observer point definitions; for the DSK
///     case, use both the "intercept" and "nadir" definitions.
///
///     Display the locations of both the sun and the sub-solar
///     point relative to the center of Mars, in the IAU_MARS
///     body-fixed reference frame, using both planetocentric and
///     planetographic coordinates.
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
///        File: subslr_ex1.tm
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
///                               'megr90n000cb_plate.bds' )
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM SUBSLR_EX1
///           IMPLICIT NONE
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      DPR
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         META
///           PARAMETER           ( META   = 'subslr_ex1.tm' )
///
///           CHARACTER*(*)         FM
///           PARAMETER           ( FM     =  '(A,F18.9)' )
///
///           INTEGER               MTHLEN
///           PARAMETER           ( MTHLEN = 50 )
///
///           INTEGER               NMETH
///           PARAMETER           ( NMETH  = 4 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(MTHLEN)    METHOD ( NMETH )
///
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      F
///           DOUBLE PRECISION      RADII  ( 3 )
///           DOUBLE PRECISION      RE
///           DOUBLE PRECISION      RP
///           DOUBLE PRECISION      SPCLAT
///           DOUBLE PRECISION      SPCLON
///           DOUBLE PRECISION      SPCRAD
///           DOUBLE PRECISION      SPGALT
///           DOUBLE PRECISION      SPGLAT
///           DOUBLE PRECISION      SPGLON
///           DOUBLE PRECISION      SPOINT ( 3 )
///           DOUBLE PRECISION      SRFVEC ( 3 )
///           DOUBLE PRECISION      SUNLT
///           DOUBLE PRECISION      SUNPOS ( 3 )
///           DOUBLE PRECISION      SUNST  ( 6 )
///           DOUBLE PRECISION      SUPCLN
///           DOUBLE PRECISION      SUPCLT
///           DOUBLE PRECISION      SUPCRD
///           DOUBLE PRECISION      SUPGAL
///           DOUBLE PRECISION      SUPGLN
///           DOUBLE PRECISION      SUPGLT
///           DOUBLE PRECISION      TRGEPC
///
///           INTEGER               I
///           INTEGER               N
///     C
///     C     Saved variables
///     C
///           SAVE                  METHOD
///     C
///     C     Initial values
///     C
///           DATA                  METHOD / 'Intercept/ellipsoid',
///          .                               'Near point/ellipsoid',
///          .                      'Intercept/DSK/Unprioritized',
///          .                      'Nadir/DSK/Unprioritized'      /
///     C
///     C     Load kernel files via the meta-kernel.
///     C
///           CALL FURNSH ( META )
///
///     C
///     C     Convert the UTC request time to ET (seconds past
///     C     J2000, TDB).
///     C
///           CALL STR2ET ( '2008 AUG 11 00:00:00', ET )
///
///     C
///     C     Look up the target body's radii. We'll use these to
///     C     convert Cartesian to planetographic coordinates. Use
///     C     the radii to compute the flattening coefficient of
///     C     the reference ellipsoid.
///     C
///           CALL BODVRD ( 'MARS', 'RADII', 3, N, RADII )
///
///     C
///     C     Let RE and RP be, respectively, the equatorial and
///     C     polar radii of the target.
///     C
///           RE = RADII( 1 )
///           RP = RADII( 3 )
///
///           F  = ( RE - RP ) / RE
///
///     C
///     C     Compute the sub-solar point using light time and stellar
///     C     aberration corrections. Use the "target surface
///     C     intercept" definition of sub-solar point on the first
///     C     loop iteration, and use the "near point" definition on
///     C     the second.
///     C
///           DO I = 1, NMETH
///
///              CALL SUBSLR ( METHOD(I),
///          .                'MARS',  ET,     'IAU_MARS', 'CN+S',
///          .                'EARTH', SPOINT, TRGEPC,     SRFVEC )
///     C
///     C        Convert the sub-solar point's rectangular coordinates
///     C        to planetographic longitude, latitude and altitude.
///     C        Convert radians to degrees.
///     C
///              CALL RECPGR ( 'MARS', SPOINT, RE,    F,
///          .                 SPGLON, SPGLAT, SPGALT   )
///
///              SPGLON = SPGLON * DPR ()
///              SPGLAT = SPGLAT * DPR ()
///
///     C
///     C        Convert sub-solar point's rectangular coordinates to
///     C        planetocentric radius, longitude, and latitude.
///     C        Convert radians to degrees.
///     C
///              CALL RECLAT ( SPOINT, SPCRAD, SPCLON, SPCLAT )
///
///              SPCLON = SPCLON * DPR ()
///              SPCLAT = SPCLAT * DPR ()
///
///     C
///     C        Compute the Sun's apparent position relative to the
///     C        sub-solar point at TRGEPC. Add the position of
///     C        the sub-solar point relative to the target's center
///     C        to obtain the position of the sun relative to the
///     C        target's center. Express the latter position in
///     C        planetographic coordinates.
///     C
///              CALL SPKCPO ( 'SUN',  TRGEPC, 'IAU_MARS', 'OBSERVER',
///          .                 'CN+S', SPOINT, 'MARS',     'IAU_MARS',
///          .                 SUNST,  SUNLT                          )
///
///              CALL VADD ( SUNST, SPOINT, SUNPOS )
///
///              CALL RECPGR ( 'MARS', SUNPOS, RE,    F,
///          .                 SUPGLN, SUPGLT, SUPGAL   )
///
///              SUPGLN = SUPGLN * DPR ()
///              SUPGLT = SUPGLT * DPR ()
///
///     C
///     C        Convert the Sun's rectangular coordinates to
///     C        planetocentric radius, longitude, and latitude.
///     C        Convert radians to degrees.
///     C
///              CALL RECLAT ( SUNPOS, SUPCRD, SUPCLN, SUPCLT )
///
///              SUPCLN = SUPCLN * DPR ()
///              SUPCLT = SUPCLT * DPR ()
///
///     C
///     C        Write the results.
///     C
///              WRITE(*,FM) ' '
///              WRITE(*,* ) 'Computation method = ', METHOD(I)
///              WRITE(*,FM) ' '
///              WRITE(*,FM) '  Sub-solar point altitude            '
///          .   // '(km) = ', SPGALT
///              WRITE(*,FM) '  Sub-solar planetographic longitude '
///          .   // '(deg) = ', SPGLON
///              WRITE(*,FM) '  Sun''s planetographic longitude     '
///          .   // '(deg) = ', SUPGLN
///              WRITE(*,FM) '  Sub-solar planetographic latitude  '
///          .   // '(deg) = ', SPGLAT
///              WRITE(*,FM) '  Sun''s planetographic latitude      '
///          .   // '(deg) = ', SUPGLT
///              WRITE(*,FM) '  Sub-solar planetocentric longitude '
///          .   // '(deg) = ', SPCLON
///              WRITE(*,FM) '  Sun''s planetocentric longitude     '
///          .   // '(deg) = ', SUPCLN
///              WRITE(*,FM) '  Sub-solar planetocentric latitude  '
///          .   // '(deg) = ', SPCLAT
///              WRITE(*,FM) '  Sun''s planetocentric latitude      '
///          .   // '(deg) = ', SUPCLT
///              WRITE(*,FM) ' '
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
///      Computation method = Intercept/ellipsoid
///
///       Sub-solar point altitude            (km) =        0.000000000
///       Sub-solar planetographic longitude (deg) =      175.810675508
///       Sun's planetographic longitude     (deg) =      175.810675508
///       Sub-solar planetographic latitude  (deg) =       23.668550281
///       Sun's planetographic latitude      (deg) =       23.420823362
///       Sub-solar planetocentric longitude (deg) =     -175.810675508
///       Sun's planetocentric longitude     (deg) =     -175.810675508
///       Sub-solar planetocentric latitude  (deg) =       23.420819936
///       Sun's planetocentric latitude      (deg) =       23.420819936
///
///
///      Computation method = Near point/ellipsoid
///
///       Sub-solar point altitude            (km) =       -0.000000000
///       Sub-solar planetographic longitude (deg) =      175.810675408
///       Sun's planetographic longitude     (deg) =      175.810675408
///       Sub-solar planetographic latitude  (deg) =       23.420823362
///       Sun's planetographic latitude      (deg) =       23.420823362
///       Sub-solar planetocentric longitude (deg) =     -175.810675408
///       Sun's planetocentric longitude     (deg) =     -175.810675408
///       Sub-solar planetocentric latitude  (deg) =       23.175085578
///       Sun's planetocentric latitude      (deg) =       23.420819936
///
///
///      Computation method = Intercept/DSK/Unprioritized
///
///       Sub-solar point altitude            (km) =       -4.052254284
///       Sub-solar planetographic longitude (deg) =      175.810675512
///       Sun's planetographic longitude     (deg) =      175.810675512
///       Sub-solar planetographic latitude  (deg) =       23.668848891
///       Sun's planetographic latitude      (deg) =       23.420823362
///       Sub-solar planetocentric longitude (deg) =     -175.810675512
///       Sun's planetocentric longitude     (deg) =     -175.810675512
///       Sub-solar planetocentric latitude  (deg) =       23.420819936
///       Sun's planetocentric latitude      (deg) =       23.420819936
///
///
///      Computation method = Nadir/DSK/Unprioritized
///
///       Sub-solar point altitude            (km) =       -4.022302438
///       Sub-solar planetographic longitude (deg) =      175.810675412
///       Sun's planetographic longitude     (deg) =      175.810675412
///       Sub-solar planetographic latitude  (deg) =       23.420823362
///       Sun's planetographic latitude      (deg) =       23.420823362
///       Sub-solar planetocentric longitude (deg) =     -175.810675412
///       Sun's planetocentric longitude     (deg) =     -175.810675412
///       Sub-solar planetocentric latitude  (deg) =       23.174793924
///       Sun's planetocentric latitude      (deg) =       23.420819936
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
/// -    SPICELIB Version 3.1.0, 01-NOV-2021 (JDR) (NJB) (EDW)
///
///         Bug fix: PRVCOR is no longer set to blank before
///         ABCORR is parsed.
///
///         Body radii accessed from kernel pool using ZZGFTREB.
///
///         Edited the header to comply with NAIF standard.
///         Changed code example and its output to comply with maximum
///         line length for header comments.
///
/// -    SPICELIB Version 3.0.0, 04-APR-2017 (NJB)
///
///         Added FAILED tests.
///
///        14-JUL-2016 (NJB)
///
///         Now uses surface mapping tracking capability.
///         Updated header.
///
///        09-FEB-2015 (NJB)
///
///         Support for surface specification was added.
///         Header was updated to document DSK features.
///
///        24-DEC-2014 (NJB)
///
///         Updated to support DSK data.
///
/// -    SPICELIB Version 2.0.0, 31-MAR-2014 (NJB) (SCK) (BVS)
///
///         Bug fix: stellar aberration is no longer applied to the
///         observer-to-estimated sub-solar point vector while solving for
///         the sub-solar point. This correction involved unnecessary code
///         but did not affect this routine's outputs.
///
///         Bug fix: FIRST is now set to .FALSE. at the completion of a
///         successful initialization pass. This does not affect the
///         routine's outputs but improves efficiency.
///
///         $Exceptions removed: the observer and target are now
///         permitted to coincide.
///
///         Upgrade: the algorithm for finding the apparent state of the
///         sun as seen from the estimated sub-solar point has been
///         improved.
///
///         Upgrade: this routine now uses ZZVALCOR rather than ZZPRSCOR,
///         simplifying the implementation.
///
///         The header example program was updated to reflect the new
///         method of computing the apparent sun location, and the set
///         of kernels referenced by the example meta-kernel were updated.
///         The display of the program's output was updated accordingly.
///
///         References to the new PXFRM2 routine were added, which changed
///         the Detailed Output section.
///
///         Updated to save the input body names and ZZBODTRN state
///         counters and to do name-ID conversions only if the counters
///         have changed.
///
///         Updated to save the input frame name and POOL state counter
///         and to do frame name-ID conversion only if the counter has
///         changed.
///
///         Updated to call LJUCRS instead of CMPRSS/UCASE.
///
/// -    SPICELIB Version 1.1.0, 18-MAY-2010 (NJB)
///
///         Bug fix: calls to FAILED() have been added after
///         SPK calls, target radius lookup, near point
///         and surface intercept computations.
///
/// -    SPICELIB Version 1.0.1, 17-MAR-2009 (NJB)
///
///         Typo correction: changed FIXFRM to FIXREF in header
///         documentation. Meta-kernel name suffix was changed to
///         ".tm" in header code example.
///
///         Typo correction in $Required_Reading, changed
///         FRAME to FRAMES.
///
/// -    SPICELIB Version 1.0.0, 02-MAR-2008 (NJB)
/// ```
pub fn subslr(
    ctx: &mut SpiceContext,
    method: &str,
    target: &str,
    et: f64,
    fixref: &str,
    abcorr: &str,
    obsrvr: &str,
    spoint: &mut [f64; 3],
    trgepc: &mut f64,
    srfvec: &mut [f64; 3],
) -> crate::Result<()> {
    SUBSLR(
        method.as_bytes(),
        target.as_bytes(),
        et,
        fixref.as_bytes(),
        abcorr.as_bytes(),
        obsrvr.as_bytes(),
        spoint,
        trgepc,
        srfvec,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SUBSLR ( Sub-solar point )
pub fn SUBSLR(
    METHOD: &[u8],
    TARGET: &[u8],
    ET: f64,
    FIXREF: &[u8],
    ABCORR: &[u8],
    OBSRVR: &[u8],
    SPOINT: &mut [f64],
    TRGEPC: &mut f64,
    SRFVEC: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut SPOINT = DummyArrayMut::new(SPOINT, 1..=3);
    let mut SRFVEC = DummyArrayMut::new(SRFVEC, 1..=3);
    let mut PNTDEF = [b' '; CVTLEN as usize];
    let mut SHPSTR = [b' '; SHPLEN as usize];
    let mut SUBTYP = [b' '; SUBLEN as usize];
    let mut TRMSTR = [b' '; TMTLEN as usize];
    let mut ALTSUN: f64 = 0.0;
    let mut DVEC = StackArray::<f64, 3>::new(1..=3);
    let mut ETDIFF: f64 = 0.0;
    let mut J2POS = StackArray::<f64, 3>::new(1..=3);
    let mut LT: f64 = 0.0;
    let mut LTDIFF: f64 = 0.0;
    let mut OBSPOS = StackArray::<f64, 3>::new(1..=3);
    let mut OBSRNG: f64 = 0.0;
    let mut PREVET: f64 = 0.0;
    let mut PREVLT: f64 = 0.0;
    let mut RADII = StackArray::<f64, 3>::new(1..=3);
    let mut S: f64 = 0.0;
    let mut SLT: f64 = 0.0;
    let mut SPOS = StackArray::<f64, 3>::new(1..=3);
    let mut SSBOST = StackArray::<f64, 6>::new(1..=6);
    let mut SSBTST = StackArray::<f64, 6>::new(1..=6);
    let mut SSLRLT: f64 = 0.0;
    let mut SSLRST = StackArray::<f64, 6>::new(1..=6);
    let mut SUNST = StackArray::<f64, 6>::new(1..=6);
    let mut TPOS = StackArray::<f64, 3>::new(1..=3);
    let mut XFORM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut FIXCID: i32 = 0;
    let mut FIXCLS: i32 = 0;
    let mut FIXCTR: i32 = 0;
    let mut FIXFID: i32 = 0;
    let mut I: i32 = 0;
    let mut NITR: i32 = 0;
    let mut OBSCDE: i32 = 0;
    let mut TRGCDE: i32 = 0;
    let mut ATTBLK = StackArray::<bool, 15>::new(1..=NABCOR);
    let mut FND: bool = false;
    let mut SURFUP: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // This value will become system-dependent when systems
    // using 128-bit d.p. numbers are supported by SPICELIB.
    // CNVLIM, when added to 1.0D0, should yield 1.0D0.
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
        // Reject an aberration correction flag calling for transmission
        // corrections.
        //
        if ATTBLK[XMTIDX] {
            SETMSG(
                b"Aberration correction flag # calls for transmission-style corrections.",
                ctx,
            );
            ERRCH(b"#", ABCORR, ctx);
            SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
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
        // The above definitions are consistent with those used by
        // ZZPRSCOR.
        //
        save.XMIT = ATTBLK[XMTIDX];
        save.USELT = ATTBLK[LTIDX];
        save.USECN = ATTBLK[CNVIDX];
        save.USESTL = ATTBLK[STLIDX];

        //
        // The aberration correction flag is recognized; save it.
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
    // The target body may not be the sun.
    //
    if (TRGCDE == SUN) {
        SETMSG(
            b"The target body is the sun; the sub-solar point is undefined for this case.",
            ctx,
        );
        SIGERR(b"SPICE(INVALIDTARGET)", ctx)?;
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

    FRINFO(FIXFID, &mut FIXCTR, &mut FIXCLS, &mut FIXCID, &mut FND, ctx)?;

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
    if (FIXCTR != TRGCDE) {
        SETMSG(b"Reference frame # is not centered at the target body #. The ID code of the frame center is #.", ctx);
        ERRCH(b"#", FIXREF, ctx);
        ERRCH(b"#", TARGET, ctx);
        ERRINT(b"#", FIXCTR, ctx);
        SIGERR(b"SPICE(INVALIDFRAME)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Check whether the surface name/ID mapping has been updated.
    //
    ZZSRFTRK(save.SVCTR4.as_slice_mut(), &mut SURFUP, ctx)?;

    //
    // If necessary, parse the method specification. PRVMTH
    // and the derived variables NEAR and SHAPE start out with
    // valid values. PRVMTH records the last valid value of
    // METHOD; NEAR and SHAPE are the corresponding variables.
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

        if fstr::eq(&SUBTYP, b" ") {
            SETMSG(
                b"Sub-solar point type is required but was not found in the method string #.",
                ctx,
            );
            ERRCH(b"#", METHOD, ctx);
            SIGERR(b"SPICE(INVALIDSUBTYPE)", ctx)?;
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

        if (save.SHAPE == ELLSHP) {
            //
            // Allow both "near point" and "nadir" expressions
            // the ellipsoid case, since these are equivalent.
            //
            save.NEAR = (EQSTR(&SUBTYP, b"NEAR POINT") || EQSTR(&SUBTYP, b"NADIR"));
        } else {
            //
            // "near point" is not supported for DSKs.
            //
            save.NEAR = EQSTR(&SUBTYP, b"NADIR");
        }

        if !save.NEAR {
            if !EQSTR(&SUBTYP, b"INTERCEPT") {
                SETMSG(
                    b"Invalid sub-solar point type <#> was found in the method string #.",
                    ctx,
                );
                ERRCH(b"#", &SUBTYP, ctx);
                ERRCH(b"#", METHOD, ctx);
                SIGERR(b"SPICE(INVALIDSUBTYPE)", ctx)?;
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }
        }

        //
        // Save the current value of METHOD.
        //
        fstr::assign(&mut save.PRVMTH, METHOD);
    }

    //
    // At this point, the first pass actions were successful.
    //
    save.FIRST = false;

    if (save.SHAPE == DSKSHP) {
        //
        // This is the DSK case.
        //
        // Initialize the intercept algorithm to use a DSK
        // model for the surface of the target body.
        //
        ZZSUDSKI(TRGCDE, save.NSURF, save.SRFLST.as_slice(), FIXFID, ctx)?;
    } else if (save.SHAPE != ELLSHP) {
        SETMSG(b"Computation method argument was <#>; this string must specify a supported shape model and computation type. See the header of SUBSLR for details.", ctx);
        ERRCH(b"#", METHOD, ctx);
        SIGERR(b"SPICE(INVALIDMETHOD)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    if FAILED(ctx) {
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Get the sign S prefixing LT in the expression for TRGEPC.
    // When light time correction is not used, setting S = 0
    // allows us to seamlessly set TRGEPC equal to ET.
    //
    if save.USELT {
        S = -1.0;
    } else {
        S = 0.0;
    }

    //
    // Determine the position of the observer in target body-fixed
    // coordinates. This is a first estimate.
    //
    //     -  Call SPKEZP to compute the position of the target body as
    //        seen from the observing body and the light time (LT)
    //        between them. We request that the coordinates of POS be
    //        returned relative to the body fixed reference frame
    //        associated with the target body, using aberration
    //        corrections specified by the input argument ABCORR.
    //
    //     -  Call VMINUS to negate the direction of the vector (OBSPOS)
    //        so it will be the position of the observer as seen from
    //        the target body in target body fixed coordinates.
    //
    //        Note that this result is not the same as the result of
    //        calling SPKEZP with the target and observer switched. We
    //        computed the vector FROM the observer TO the target in
    //        order to get the proper light time and stellar aberration
    //        corrections (if requested). Now we need the inverse of
    //        that corrected vector in order to compute the sub-solar
    //        point.
    //
    SPKEZP(
        TRGCDE,
        ET,
        FIXREF,
        ABCORR,
        OBSCDE,
        TPOS.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Negate the target's position to obtain the position of the
    // observer relative to the target.
    //
    VMINUS(TPOS.as_slice(), OBSPOS.as_slice_mut());

    //
    // Make a first estimate of the target epoch.
    //
    *TRGEPC = (ET + (S * LT));

    //
    // Find the sub-solar point given the target epoch, observer-target
    // position, and target body orientation we've already computed. If
    // we're not using light time correction, this is all we need do.
    // Otherwise, our result will give us an initial estimate of the
    // target epoch, which we'll then improve.
    //

    //
    // Get the radii of the target body from the kernel pool.
    //
    ZZGFTREB(TRGCDE, RADII.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Get the position of the Sun SPOS as seen from the target
    // in the target body-fixed frame at TRGEPC.
    //
    SPKEZP(
        SUN,
        *TRGEPC,
        FIXREF,
        ABCORR,
        TRGCDE,
        SPOS.as_slice_mut(),
        &mut SLT,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Make a first estimate of the sub-solar point. The algorithm
    // we use depends on the sub-solar point definition.
    //
    if save.NEAR {
        //
        // Locate the nearest point to the Sun on the target's
        // reference ellipsoid.
        //
        NEARPT(
            SPOS.as_slice(),
            RADII[1],
            RADII[2],
            RADII[3],
            SPOINT.as_slice_mut(),
            &mut ALTSUN,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // If the target is an ellipsoid, the NEARPT call above does the
        // trick. For DSKs, we define a ray emanating from the sun and
        // passing through the near point on the reference ellipsoid. The
        // closest ray-DSK surface intercept to the sun is the initial
        // estimate of the sub-solar point.
        //
        if (save.SHAPE == DSKSHP) {
            //
            // Generate the ray direction; find the DSK intercept.
            //
            VSUB(SPOINT.as_slice(), SPOS.as_slice(), DVEC.as_slice_mut());

            ZZSBFXR(
                TRGCDE,
                save.NSURF,
                save.SRFLST.as_slice(),
                *TRGEPC,
                FIXFID,
                SPOS.as_slice(),
                DVEC.as_slice(),
                SPOINT.as_slice_mut(),
                &mut FND,
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            if !FND {
                SETMSG(b"No sub-solar point was found on the surface defined by DSK data. Observer is #; target is #. This problem can occur for bodies having shapes not well modeled by ellipsoids. Consider using the \"Intercept: DSK\" computation method.", ctx);
                ERRCH(b"#", OBSRVR, ctx);
                ERRCH(b"#", TARGET, ctx);
                SIGERR(b"SPICE(SUBPOINTNOTFOUND)", ctx)?;
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }
        }
    } else {
        //
        // This is the case for the "intercept" sub-solar point
        // definition.
        //
        // Generate the ray direction.
        //
        VMINUS(SPOS.as_slice(), DVEC.as_slice_mut());

        if (save.SHAPE == ELLSHP) {
            //
            // Locate the surface intercept of the ray from the
            // sun to the target center.
            //
            SURFPT(
                SPOS.as_slice(),
                DVEC.as_slice(),
                RADII[1],
                RADII[2],
                RADII[3],
                SPOINT.as_slice_mut(),
                &mut FND,
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            if !FND {
                //
                // If there's no intercept, we have a numerical problem.
                //
                SETMSG(b"No intercept of sun-target ray was found.", ctx);
                SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }
        } else {
            //
            // Find the DSK intercept.
            //
            ZZSBFXR(
                TRGCDE,
                save.NSURF,
                save.SRFLST.as_slice(),
                *TRGEPC,
                FIXFID,
                SPOS.as_slice(),
                DVEC.as_slice(),
                SPOINT.as_slice_mut(),
                &mut FND,
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            if !FND {
                SETMSG(b"No sub-solar point was found on the surface defined by DSK data. Observer is #; target is #. This problem can occur for a body having an irregular shape such that the origin of the body-fixed reference frame is outside of the body. A torus is an example of such a shape.", ctx);
                ERRCH(b"#", OBSRVR, ctx);
                ERRCH(b"#", TARGET, ctx);
                SIGERR(b"SPICE(SUBPOINTNOTFOUND)", ctx)?;
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }
        }
    }

    if FAILED(ctx) {
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // If we're not using light time and stellar aberration
    // corrections, we're almost done now. Note that we need only
    // check for use of light time corrections, because use of
    // stellar aberration corrections alone has been prevented by an
    // earlier check.
    //
    if !save.USELT {
        //
        // The TRGEPC value we'll return is simply the input time. The
        // previous call to SPKEZP call yielded the vector OBSPOS. SPOINT
        // was set immediately above. The only output left to compute is
        // SRFVEC.
        //
        *TRGEPC = ET;

        VSUB(SPOINT.as_slice(), OBSPOS.as_slice(), SRFVEC.as_slice_mut());

        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Compute the range from the observer to the sub-solar
    // point. We'll use this range for a light time estimate.
    //
    OBSRNG = VDIST(OBSPOS.as_slice(), SPOINT.as_slice());

    //
    // Compute the one-way light time and target epoch based on our
    // first computation of SPOINT. The coefficient S has been
    // set to give us the correct answer for each aberration
    // correction case.
    //
    LT = (OBSRNG / CLIGHT());
    *TRGEPC = (ET + (S * LT));

    //
    // We'll now make an improved sub-solar point estimate using the
    // previous estimate of the sub-solar point. The number of
    // iterations depends on the light time correction type.
    //
    if save.USECN {
        NITR = MAXITR;
    } else {
        NITR = 1;
    }

    //
    // Get the J2000-relative state of the observer relative to
    // the solar system barycenter at ET.
    //
    SPKSSB(OBSCDE, ET, b"J2000", SSBOST.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Initialize the variables required to evaluate the
    // loop termination condition.
    //
    I = 0;
    LTDIFF = 1.0;
    ETDIFF = 1.0;
    PREVLT = LT;
    PREVET = *TRGEPC;

    while (((I < NITR) && (LTDIFF > (CNVLIM * f64::abs(LT)))) && (ETDIFF > 0.0)) {
        //
        // Get the J2000-relative state of the target relative to
        // the solar system barycenter at the target epoch.
        //
        SPKSSB(TRGCDE, *TRGEPC, b"J2000", SSBTST.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // Find the position of the observer relative to the target.
        // Convert this vector from the J2000 frame to the target
        // frame at TRGEPC.
        //
        VSUB(SSBOST.as_slice(), SSBTST.as_slice(), J2POS.as_slice_mut());
        PXFORM(b"J2000", FIXREF, *TRGEPC, XFORM.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        MXV(XFORM.as_slice(), J2POS.as_slice(), OBSPOS.as_slice_mut());

        //
        // Note: if we're using stellar aberration correction, we do not
        // apply the stellar aberration correction of the estimated
        // sub-solar point as seen by the observer to the next estimate
        // of the sun-solar point. The location of this point depends on
        // the illumination of the target, which is a function of the
        // observer-surface point light time. This is the only way in
        // which the observer plays a role in determining the sub-solar
        // point.
        //
        // Stellar aberration of the sun's position relative to the
        // sub-solar point *is* used.
        //
        // First find the apparent position of the sun as seen from the
        // estimated sub-solar point.
        //

        SPKCPO(
            b"SUN",
            *TRGEPC,
            FIXREF,
            b"OBSERVER",
            ABCORR,
            SPOINT.as_slice(),
            TARGET,
            FIXREF,
            SUNST.as_slice_mut(),
            &mut SLT,
            ctx,
        )?;

        //
        // Create the target-center to sun vector.
        //
        VADD(SUNST.as_slice(), SPOINT.as_slice(), SPOS.as_slice_mut());

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // Find the sub-solar point using the current estimated
        // geometry.
        //
        if save.NEAR {
            //
            // Locate the nearest point to the sun on the target's
            // reference ellipsoid.
            //
            NEARPT(
                SPOS.as_slice(),
                RADII[1],
                RADII[2],
                RADII[3],
                SPOINT.as_slice_mut(),
                &mut ALTSUN,
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }
            //
            // If the target is an ellipsoid, the NEARPT call above does
            // the trick. For DSKs, we define a ray emanating from the sun
            // and passing through the near point on the reference
            // ellipsoid. The closest ray-DSK surface intercept to the sun
            // is the initial estimate of the sub-solar point.
            //
            if (save.SHAPE == DSKSHP) {
                //
                // Locate the surface intercept of the ray from the
                // Sun to the target center.
                //
                VSUB(SPOINT.as_slice(), SPOS.as_slice(), DVEC.as_slice_mut());

                ZZSBFXR(
                    TRGCDE,
                    save.NSURF,
                    save.SRFLST.as_slice(),
                    *TRGEPC,
                    FIXFID,
                    SPOS.as_slice(),
                    DVEC.as_slice(),
                    SPOINT.as_slice_mut(),
                    &mut FND,
                    ctx,
                )?;

                if FAILED(ctx) {
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }

                if !FND {
                    SETMSG(b"No sub-solar point was found on the surface defined by DSK data. Observer is #; target is #. This problem can occur for bodies having shapes not well modeled by ellipsoids. Consider using the \"Intercept: DSK\" computation method.", ctx);
                    ERRCH(b"#", OBSRVR, ctx);
                    ERRCH(b"#", TARGET, ctx);
                    SIGERR(b"SPICE(SUBPOINTNOTFOUND)", ctx)?;
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }
            }
        } else {
            // This is the "intercept" case.
            //
            // Generate the ray direction.
            //
            VMINUS(SPOS.as_slice(), DVEC.as_slice_mut());

            if (save.SHAPE == ELLSHP) {
                //
                // Locate the surface intercept of the ray from the
                // sun to the target center.
                //
                SURFPT(
                    SPOS.as_slice(),
                    DVEC.as_slice(),
                    RADII[1],
                    RADII[2],
                    RADII[3],
                    SPOINT.as_slice_mut(),
                    &mut FND,
                    ctx,
                )?;

                if FAILED(ctx) {
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }

                if !FND {
                    //
                    // If there's no intercept, we have a numerical problem.
                    //
                    SETMSG(b"No intercept of sun-target ray was found.", ctx);
                    SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }
            } else {
                //
                // Find the DSK intercept.
                //
                ZZSBFXR(
                    TRGCDE,
                    save.NSURF,
                    save.SRFLST.as_slice(),
                    *TRGEPC,
                    FIXFID,
                    SPOS.as_slice(),
                    DVEC.as_slice(),
                    SPOINT.as_slice_mut(),
                    &mut FND,
                    ctx,
                )?;

                if FAILED(ctx) {
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }

                if !FND {
                    SETMSG(b"No sub-solar point was found on the surface defined by DSK data. Observer is #; target is #. This problem can occur for a body having an irregular shape such that the origin of the body-fixed reference frame is outside of the body. A torus is an example of such a shape.", ctx);
                    ERRCH(b"#", OBSRVR, ctx);
                    ERRCH(b"#", TARGET, ctx);
                    SIGERR(b"SPICE(SUBPOINTNOTFOUND)", ctx)?;
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }
            }
        }

        //
        // Update the observer to sub-solar point range.
        //
        OBSRNG = VDIST(OBSPOS.as_slice(), SPOINT.as_slice());

        //
        // Compute a new light time estimate and new target epoch.
        //
        LT = (OBSRNG / CLIGHT());
        *TRGEPC = (ET + (S * LT));

        //
        // At this point, we have new estimates of the sub-solar point
        // SPOINT, the observer altitude OBSRNG, the target epoch
        // TRGEPC, and the position of the observer relative to the
        // target OBSPOS.
        //
        // We use the d.p. identity function TOUCHD to force the compiler
        // to create double precision arguments from the differences
        // LT-PREVLT and TRGEPC-PREVET. Some compilers will perform
        // extended-precision register arithmetic, which can prevent a
        // difference from rounding to zero. Simply storing the result of
        // the subtraction in a double precision variable doesn't solve
        // the problem, because that variable can be optimized out of
        // existence.
        //
        LTDIFF = f64::abs(TOUCHD((LT - PREVLT)));
        ETDIFF = f64::abs(TOUCHD((*TRGEPC - PREVET)));
        PREVLT = LT;
        PREVET = *TRGEPC;
        I = (I + 1);

        SPKSSB(TRGCDE, *TRGEPC, b"J2000", SSBTST.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // Find the position of the observer relative to the target.
        // Convert this vector from the J2000 frame to the target
        // frame at TRGEPC.
        //
        VSUB(SSBOST.as_slice(), SSBTST.as_slice(), J2POS.as_slice_mut());
        PXFORM(b"J2000", FIXREF, *TRGEPC, XFORM.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        MXV(XFORM.as_slice(), J2POS.as_slice(), OBSPOS.as_slice_mut());
    }

    //
    // SPOINT and TRGEPC have been set. Compute SRFVEC, using
    // both light time and stellar aberration corrections if
    // these have been requested by the caller. Note that
    // the position of the target body was computed without
    // stellar aberration corrections, so we may not be able
    // to use it for this computation.
    //
    if save.USESTL {
        SPKCPT(
            SPOINT.as_slice(),
            TARGET,
            FIXREF,
            ET,
            FIXREF,
            b"TARGET",
            ABCORR,
            OBSRVR,
            SSLRST.as_slice_mut(),
            &mut SSLRLT,
            ctx,
        )?;
        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        VEQU(SSLRST.as_slice(), SRFVEC.as_slice_mut());
    } else {
        VSUB(SPOINT.as_slice(), OBSPOS.as_slice(), SRFVEC.as_slice_mut());
    }

    CHKOUT(RNAME, ctx)?;
    Ok(())
}
