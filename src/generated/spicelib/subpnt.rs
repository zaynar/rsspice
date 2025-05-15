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
const RNAME: &[u8] = b"SUBPNT";
const CNVLIM: f64 = 0.00000000000000001;
const MAXITR: i32 = 5;
const MAXL: i32 = 36;
const FRNMLN: i32 = 32;

struct SaveVars {
    PRVCOR: Vec<u8>,
    PRVMTH: Vec<u8>,
    SUBTYP: Vec<u8>,
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
        let mut SUBTYP = vec![b' '; SUBLEN as usize];
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
        SHAPE = -1;

        Self {
            PRVCOR,
            PRVMTH,
            SUBTYP,
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

/// Sub-observer point
///
/// Compute the rectangular coordinates of the sub-observer point on
/// a target body at a specified epoch, optionally corrected for
/// light time and stellar aberration.
///
/// The surface of the target body may be represented by a triaxial
/// ellipsoid or by topographic data provided by DSK files.
///
/// This routine supersedes SUBPT.
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
///  SPOINT     O   Sub-observer point on the target body.
///  TRGEPC     O   Sub-observer point epoch.
///  SRFVEC     O   Vector from observer to sub-observer point.
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
///                 The sub-observer point computation uses a
///                 triaxial ellipsoid to model the surface of the
///                 target body. The sub-observer point is defined
///                 as the nearest point on the target relative to
///                 the observer.
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
///                 The sub-observer point computation uses a
///                 triaxial ellipsoid to model the surface of the
///                 target body. The sub-observer point is defined
///                 as the target surface intercept of the line
///                 containing the observer and the target's
///                 center.
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
///                 The sub-observer point computation uses DSK data
///                 to model the surface of the target body. The
///                 sub-observer point is defined as the intercept, on
///                 the surface represented by the DSK data, of the
///                 line containing the observer and the nearest point
///                 on the target's reference ellipsoid. If multiple
///                 such intercepts exist, the one closest to the
///                 observer is selected.
///
///                 Note that this definition of the sub-observer
///                 point is not equivalent to the "nearest point on
///                 the surface to the observer." The phrase "NEAR
///                 POINT" may NOT be substituted for "NADIR" in the
///                 string above.
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
///                 The sub-observer point computation uses DSK data
///                 to model the surface of the target body. The
///                 sub-observer point is defined as the target
///                 surface intercept of the line containing the
///                 observer and the target's center.
///
///                 If multiple such intercepts exist, the one closest
///                 to the observer is selected.
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
///           the target body are computed.
///
///           When aberration corrections are used, the position
///           and orientation of the target body are computed at
///           ET-LT or ET+LT, where LT is the one-way light time
///           between the sub-observer point and the observer, and
///           the sign applied to LT depends on the selected
///           correction. See the description of ABCORR below for
///           details.
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
///           The output sub-observer point SPOINT and the
///           observer-to-sub-observer point vector SRFVEC will be
///           expressed relative to this reference frame.
///
///  ABCORR   indicates the aberration corrections to be applied
///           when computing the target's position and orientation.
///
///           For remote sensing applications, where the apparent
///           sub-observer point seen by the observer is desired,
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
///                         geometric sub-observer point on the
///                         target body.
///
///           Let LT represent the one-way light time between the
///           observer and the sub-observer point (note: NOT
///           between the observer and the target body's center).
///           The following values of ABCORR apply to the
///           "reception" case in which photons depart from the
///           sub-observer point's location at the light-time
///           corrected epoch ET-LT and *arrive* at the observer's
///           location at ET:
///
///
///              'LT'       Correct for one-way light time (also
///                         called "planetary aberration") using a
///                         Newtonian formulation. This correction
///                         yields the location of sub-observer
///                         point at the moment it emitted photons
///                         arriving at the observer at ET.
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
///                         sub-observer point obtained with the
///                         'LT' option to account for the
///                         observer's velocity relative to the
///                         solar system barycenter. These
///                         corrections yield the apparent
///                         sub-observer point.
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
///                         with the `LT+S' option. Whether the
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
///           "transmission" case in which photons *depart* from
///           the observer's location at ET and arrive at the
///           sub-observer point at the light-time corrected epoch
///           ET+LT:
///
///              'XLT'      "Transmission" case: correct for
///                         one-way light time using a Newtonian
///                         formulation. This correction yields the
///                         sub-observer location at the moment it
///                         receives photons emitted from the
///                         observer's location at ET.
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
///              'XLT+S'    "Transmission" case: correct for
///                         one-way light time and stellar
///                         aberration using a Newtonian
///                         formulation  This option modifies the
///                         sub-observer point obtained with the
///                         'XLT' option to account for the
///                         observer's velocity relative to the
///                         solar system barycenter.
///
///              'XCN'      Converged Newtonian light time
///                         correction. This is the same as 'XLT'
///                         correction but with further iterations
///                         to a converged Newtonian light time
///                         solution.
///
///              'XCN+S'    "Transmission" case: converged
///                         Newtonian light time and stellar
///                         aberration corrections.
///
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
/// ```
///
/// # Detailed Output
///
/// ```text
///  SPOINT   is the sub-observer point on the target body.
///
///           For target shapes modeled by ellipsoids, the
///           sub-observer point is defined either as the point on
///           the target body that is closest to the observer, or
///           the target surface intercept of the line from the
///           observer to the target's center.
///
///           For target shapes modeled by topographic data
///           provided by DSK files, the sub-observer point is
///           defined as the target surface intercept of the line
///           from the observer to either the nearest point on the
///           reference ellipsoid, or to the target's center. If
///           multiple such intercepts exist, the one closest to
///           the observer is selected.
///
///           The input argument METHOD selects the target shape
///           model and sub-observer point definition to be used.
///
///           SPOINT is expressed in Cartesian coordinates,
///           relative to the body-fixed target frame designated by
///           FIXREF. The body-fixed target frame is evaluated at
///           the sub-observer epoch TRGEPC (see description below).
///
///           When light time correction is used, the duration of
///           light travel between SPOINT to the observer is
///           considered to be the one way light time.
///
///           When aberration corrections are used, SPOINT is
///           computed using target body position and orientation
///           that have been adjusted for the corrections
///           applicable to SPOINT itself rather than to the target
///           body's center. In particular, if the stellar
///           aberration correction applicable to SPOINT is
///           represented by a shift vector S, then the light-time
///           corrected position of the target is shifted by S
///           before the sub-observer point is computed.
///
///           The components of SPOINT have units of km.
///
///
///  TRGEPC   is the "sub-observer point epoch." TRGEPC is defined
///           as follows: letting LT be the one-way light time
///           between the observer and the sub-observer point,
///           TRGEPC is the epoch ET-LT, ET+LT, or ET depending on
///           whether the requested aberration correction is,
///           respectively, for received radiation, transmitted
///           radiation, or omitted. LT is computed using the
///           method indicated by ABCORR.
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
///
///           The second example in the $Examples header section
///           below presents a complete program that demonstrates
///           this procedure.
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
///  6)  If the input argument METHOD is not recognized, the error
///      SPICE(INVALIDMETHOD) is signaled by this routine, or, the
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  7)  If the sub-observer point type is not specified or is not
///      recognized, the error SPICE(INVALIDSUBTYPE) is signaled.
///
///  8)  If the target and observer have distinct identities but are
///      at the same location (for example, the target is Mars and the
///      observer is the Mars barycenter), the error
///      SPICE(NOSEPARATION) is signaled.
///
///  9)  If insufficient ephemeris data have been loaded prior to
///      calling SUBPNT, an error is signaled by a
///      routine in the call tree of this routine. Note that when
///      light time correction is used, sufficient ephemeris data must
///      be available to propagate the states of both observer and
///      target to the solar system barycenter.
///
///  10) If the computation method specifies an ellipsoidal target
///      shape and triaxial radii of the target body have not been
///      loaded into the kernel pool prior to calling SUBPNT, an error
///      is signaled by a routine in the call tree of this routine.
///
///  11) The target must be an extended body, and must have a shape
///      for which a sub-observer point can be defined.
///
///      If the target body's shape is modeled by DSK data, the shape
///      must be such that the specified sub-observer point
///      definition is applicable. For example, if the target shape
///      is a torus, both the NADIR and INTERCEPT definitions might
///      be inapplicable, depending on the relative locations of the
///      observer and target.
///
///  12) If PCK data specifying the target body-fixed frame orientation
///      have not been loaded prior to calling SUBPNT, an error is
///      signaled by a routine in the call tree of this routine.
///
///  13) If METHOD specifies that the target surface is represented by
///      DSK data, and no DSK files are loaded for the specified
///      target, an error is signaled by a routine in the call tree
///      of this routine.
///
///  14) If METHOD specifies that the target surface is represented
///      by DSK data, and the ray from the observer to the
///      sub-observer point doesn't intersect the target body's
///      surface, the error SPICE(SUBPOINTNOTFOUND) is signaled.
///
///  15) If the surface intercept on the target body's reference
///      ellipsoid of the observer to target center vector cannot not
///      be computed, the error SPICE(DEGENERATECASE) is signaled. Note
///      that this is a very rare case.
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
///  -  SPK data: ephemeris data for target and observer must be
///     loaded. If aberration corrections are used, the states of
///     target and observer relative to the solar system barycenter
///     must be calculable from the available ephemeris data.
///     Typically ephemeris data are made available by loading one
///     or more SPK files via FURNSH.
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
///  For ellipsoidal target bodies, there are two different popular
///  ways to define the sub-observer point: "nearest point on the
///  target to the observer" or "target surface intercept of the line
///  containing observer and target." These coincide when the target
///  is spherical and generally are distinct otherwise.
///
///  For target body shapes modeled using topographic data provided by
///  DSK files, the "surface intercept" notion is valid, but the
///  "nearest point on the surface" computation is both inefficient to
///  execute and may fail to yield a result that is "under" the
///  observer in an intuitively clear way. The NADIR option for DSK
///  shapes instead finds the surface intercept of a ray that passes
///  through the nearest point on the target reference ellipsoid. For
///  shapes modeled using topography, there may be multiple
///  ray-surface intercepts; the closest one to the observer is
///  selected.
///
///  The NADIR definition makes sense only if the target shape is
///  reasonably close to the target's reference ellipsoid. If the
///  target is very different---the nucleus of comet
///  Churyumov-Gerasimenko is an example---the intercept definition
///  should be used.
///
///  This routine computes light time corrections using light time
///  between the observer and the sub-observer point, as opposed to
///  the center of the target. Similarly, stellar aberration
///  corrections done by this routine are based on the direction of
///  the vector from the observer to the light-time corrected
///  sub-observer point, not to the target center. This technique
///  avoids errors due to the differential between aberration
///  corrections across the target body. Therefore it's valid to use
///  aberration corrections with this routine even when the observer
///  is very close to the sub-observer point, in particular when the
///  observer to sub-observer point distance is much less than the
///  observer to target center distance.
///
///  When comparing sub-observer point computations with results from
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
///  1) Find the sub-Earth point on Mars for a specified time.
///
///     Compute the sub-Earth points using both triaxial ellipsoid
///     and topographic surface models. Topography data are provided by
///     a DSK file. For the ellipsoid model, use both the "intercept"
///     and "near point" sub-observer point definitions; for the DSK
///     case, use both the "intercept" and "nadir" definitions.
///
///     Display the locations of both the Earth and the sub-Earth
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
///        File: subpnt_ex1.tm
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
///           PROGRAM SUBPNT_EX1
///           IMPLICIT NONE
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      DPR
///           DOUBLE PRECISION      VNORM
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         META
///           PARAMETER           ( META   = 'subpnt_ex1.tm' )
///
///           CHARACTER*(*)         FM
///           PARAMETER           ( FM     =  '(A,F21.9)' )
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
///           DOUBLE PRECISION      OBSPOS ( 3 )
///           DOUBLE PRECISION      ODIST
///           DOUBLE PRECISION      OPCLAT
///           DOUBLE PRECISION      OPCLON
///           DOUBLE PRECISION      OPCRAD
///           DOUBLE PRECISION      OPGALT
///           DOUBLE PRECISION      OPGLAT
///           DOUBLE PRECISION      OPGLON
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
///
///     C
///     C     Load kernel files via the meta-kernel.
///     C
///           CALL FURNSH ( META )
///
///     C
///     C     Convert the UTC request time string seconds past
///     C     J2000, TDB.
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
///     C     Compute sub-observer point using light time and
///     C     stellar aberration corrections. Use both ellipsoid
///     C     and DSK shape models, and use all of the
///     C     "near point," "intercept," and "nadir" sub-observer
///     C     point definitions.
///     C
///           DO I = 1, NMETH
///
///              CALL SUBPNT ( METHOD(I),
///          .                'MARS',  ET,     'IAU_MARS', 'CN+S',
///          .                'EARTH', SPOINT, TRGEPC,     SRFVEC )
///     C
///     C        Compute the observer's distance from SPOINT.
///     C
///              ODIST  = VNORM ( SRFVEC )
///
///     C
///     C        Convert the sub-observer point's rectangular
///     C        coordinates to planetographic longitude, latitude
///     C        and altitude. Convert radians to degrees.
///     C
///              CALL RECPGR ( 'MARS', SPOINT, RE,    F,
///          .                 SPGLON, SPGLAT, SPGALT   )
///
///              SPGLON = SPGLON * DPR ()
///              SPGLAT = SPGLAT * DPR ()
///
///     C
///     C        Convert sub-observer point's rectangular coordinates
///     C        to planetocentric radius, longitude, and latitude.
///     C        Convert radians to degrees.
///     C
///              CALL RECLAT ( SPOINT, SPCRAD, SPCLON, SPCLAT )
///
///              SPCLON = SPCLON * DPR ()
///              SPCLAT = SPCLAT * DPR ()
///
///     C
///     C        Compute the observer's position relative to the center
///     C        of the target, where the center's location has been
///     C        adjusted using the aberration corrections applicable
///     C        to the sub-point. Express the observer's location in
///     C        planetographic coordinates.
///     C
///              CALL VSUB ( SPOINT, SRFVEC, OBSPOS )
///
///              CALL RECPGR ( 'MARS', OBSPOS, RE,    F,
///          .                 OPGLON, OPGLAT, OPGALT   )
///
///              OPGLON = OPGLON * DPR ()
///              OPGLAT = OPGLAT * DPR ()
///
///     C
///     C        Convert the observer's rectangular coordinates to
///     C        planetocentric radius, longitude, and latitude.
///     C        Convert radians to degrees.
///     C
///              CALL RECLAT ( OBSPOS, OPCRAD, OPCLON, OPCLAT )
///
///              OPCLON = OPCLON * DPR ()
///              OPCLAT = OPCLAT * DPR ()
///
///     C
///     C        Write the results.
///     C
///              WRITE(*,FM) ' '
///              WRITE(*,* ) 'Computation method = ', METHOD(I)
///              WRITE(*,FM) ' '
///              WRITE(*,FM)
///          .   '  Observer ALT relative to spheroid (km) =', OPGALT
///              WRITE(*,FM)
///          .   '  Length of SRFVEC                  (km) =', ODIST
///              WRITE(*,FM)
///          .   '  Sub-observer point ALT            (km) =', SPGALT
///              WRITE(*,FM)
///          .   '  Sub-observer planetographic LON  (deg) =', SPGLON
///              WRITE(*,FM)
///          .   '  Observer planetographic LON      (deg) =', OPGLON
///              WRITE(*,FM)
///          .   '  Sub-observer planetographic LAT  (deg) =', SPGLAT
///              WRITE(*,FM)
///          .   '  Observer planetographic LAT      (deg) =', OPGLAT
///              WRITE(*,FM)
///          .   '  Sub-observer planetocentric LON  (deg) =', SPCLON
///              WRITE(*,FM)
///          .   '  Observer planetocentric LON      (deg) =', OPCLON
///              WRITE(*,FM)
///          .   '  Sub-observer planetocentric LAT  (deg) =', SPCLAT
///              WRITE(*,FM)
///          .   '  Observer planetocentric LAT      (deg) =', OPCLAT
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
///       Observer ALT relative to spheroid (km) =  349199089.540947020
///       Length of SRFVEC                  (km) =  349199089.577642620
///       Sub-observer point ALT            (km) =          0.000000000
///       Sub-observer planetographic LON  (deg) =        199.302305028
///       Observer planetographic LON      (deg) =        199.302305028
///       Sub-observer planetographic LAT  (deg) =         26.262401237
///       Observer planetographic LAT      (deg) =         25.994936751
///       Sub-observer planetocentric LON  (deg) =        160.697694972
///       Observer planetocentric LON      (deg) =        160.697694972
///       Sub-observer planetocentric LAT  (deg) =         25.994934171
///       Observer planetocentric LAT      (deg) =         25.994934171
///
///
///      Computation method = Near point/ellipsoid
///
///       Observer ALT relative to spheroid (km) =  349199089.540938556
///       Length of SRFVEC                  (km) =  349199089.540938556
///       Sub-observer point ALT            (km) =          0.000000000
///       Sub-observer planetographic LON  (deg) =        199.302305029
///       Observer planetographic LON      (deg) =        199.302305029
///       Sub-observer planetographic LAT  (deg) =         25.994936751
///       Observer planetographic LAT      (deg) =         25.994936751
///       Sub-observer planetocentric LON  (deg) =        160.697694971
///       Observer planetocentric LON      (deg) =        160.697694971
///       Sub-observer planetocentric LAT  (deg) =         25.729407227
///       Observer planetocentric LAT      (deg) =         25.994934171
///
///
///      Computation method = Intercept/DSK/Unprioritized
///
///       Observer ALT relative to spheroid (km) =  349199089.541017115
///       Length of SRFVEC                  (km) =  349199091.785406590
///       Sub-observer point ALT            (km) =         -2.207669751
///       Sub-observer planetographic LON  (deg) =        199.302304999
///       Observer planetographic LON      (deg) =        199.302304999
///       Sub-observer planetographic LAT  (deg) =         26.262576677
///       Observer planetographic LAT      (deg) =         25.994936751
///       Sub-observer planetocentric LON  (deg) =        160.697695001
///       Observer planetocentric LON      (deg) =        160.697695001
///       Sub-observer planetocentric LAT  (deg) =         25.994934171
///       Observer planetocentric LAT      (deg) =         25.994934171
///
///
///      Computation method = Nadir/DSK/Unprioritized
///
///       Observer ALT relative to spheroid (km) =  349199089.541007578
///       Length of SRFVEC                  (km) =  349199091.707172215
///       Sub-observer point ALT            (km) =         -2.166164622
///       Sub-observer planetographic LON  (deg) =        199.302305000
///       Observer planetographic LON      (deg) =        199.302305000
///       Sub-observer planetographic LAT  (deg) =         25.994936751
///       Observer planetographic LAT      (deg) =         25.994936751
///       Sub-observer planetocentric LON  (deg) =        160.697695000
///       Observer planetocentric LON      (deg) =        160.697695000
///       Sub-observer planetocentric LAT  (deg) =         25.729237570
///       Observer planetocentric LAT      (deg) =         25.994934171
///
///
///  2) Use SUBPNT to find the sub-spacecraft point on Mars for the
///     Mars Reconnaissance Orbiter spacecraft (MRO) at a specified
///     time, using both the 'Ellipsoid/Near point' computation method
///     and an ellipsoidal target shape, and the
///     'DSK/Unprioritized/Nadir' method and a DSK-based shape model.
///
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
///        File: subpnt_ex2.tm
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
///           PROGRAM SUBPNT_EX2
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
///           PARAMETER           ( META   = 'subpnt_ex2.tm' )
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
///     c           expressed relative to the IAU_MARS frame at
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
///          .      // ABCORR(J)
///                 WRITE(*,F1) ' '
///                 WRITE(*,F2) '      MRO-to-sub-observer vector in'
///                 WRITE(*,F2) '      MRO HIRISE look direction frame'
///                 WRITE(*,F1) '        X-component             '
///          .      // '(km) = ', MROVEC(1)
///                 WRITE(*,F1) '        Y-component             '
///          .      // '(km) = ', MROVEC(2)
///                 WRITE(*,F1) '        Z-component             '
///          .      // '(km) = ', MROVEC(3)
///                 WRITE(*,F1) '      Sub-observer point radius '
///          .      // '(km) = ', RADIUS
///                 WRITE(*,F1) '      Planetocentric latitude  '
///          .      // '(deg) = ', LAT
///                 WRITE(*,F1) '      Planetocentric longitude '
///          .      // '(deg) = ', LON
///                 WRITE(*,F1) '      Observer altitude         '
///          .      // '(km) = ', ALT
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
/// -    SPICELIB Version 2.1.0, 01-NOV-2021 (JDR) (EDW) (NJB)
///
///         Bug fix: PRVCOR is no longer set to blank before
///         ABCORR is parsed.
///
///         Body radii accessed from kernel pool using ZZGFTREB.
///
///         Edited the header to comply with NAIF standard.
///         Changed code examples' output to comply with maximum
///         line length for header comments.
///
///         Bug fix: TRGEPC is now initialized prior to first use.
///         Previously the lack of initialization could cause this routine
///         to fail to find DSK data within the time bounds of a DSK
///         segment.
///
/// -    SPICELIB Version 2.0.0, 04-APR-2017 (NJB)
///
///         Added FAILED tests.
///
///         01-JUL-2016 (NJB)
///
///         Now uses surface mapping tracking capability.
///         Updated header. Changed aberration correction
///         in example 1 to CN+S.
///
///         09-FEB-2015 (NJB)
///
///         Updated code to support use of surface list.
///         Updated header to document DSK capabilities.
///         Added check for invalid sub-point type.
///
///         24-DEC-2014 (NJB)
///
///         Updated to support surfaces represented by DSK data.
///
///         Bug fix: set initial value of PRVMTH to a valid
///         value.
///
/// -    SPICELIB Version 1.3.0, 31-MAR-2014 (BVS)
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
/// -    SPICELIB Version 1.2.0, 02-APR-2012 (NJB) (SCK)
///
///         Bug fix: FIRST is now set to .FALSE. at the completion
///         of a successful initialization pass. This does not affect
///         the routine's outputs but improves efficiency.
///
///         References to the new PXFRM2 routine were added, which changed
///         the Detailed Output section and the second example.
///
///         Upgrade: this routine now uses ZZVALCOR rather than
///         ZZPRSCOR, simplifying the implementation.
///
/// -    SPICELIB Version 1.1.0, 18-MAY-2010 (NJB)
///
///         Bug fix: calls to FAILED() have been added after
///         SPK calls, target radius lookup, near point
///         and surface intercept computations.
///
/// -    SPICELIB Version 1.0.1, 06-FEB-2009 (NJB)
///
///         Typo correction: changed FIXFRM to FIXREF in header
///         documentation. Meta-kernel name suffix was changed to
///         ".tm" in header code example.
///
/// -    SPICELIB Version 1.0.0, 02-MAR-2008 (NJB)
/// ```
pub fn subpnt(
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
    SUBPNT(
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

//$Procedure SUBPNT ( Sub-observer point )
pub fn SUBPNT(
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
    let mut TRMSTR = [b' '; TMTLEN as usize];
    let mut ALT: f64 = 0.0;
    let mut CORPOS = StackArray::<f64, 3>::new(1..=3);
    let mut CORVJ2 = StackArray::<f64, 3>::new(1..=3);
    let mut DVEC = StackArray::<f64, 3>::new(1..=3);
    let mut ETDIFF: f64 = 0.0;
    let mut J2POS = StackArray::<f64, 3>::new(1..=3);
    let mut LT: f64 = 0.0;
    let mut LTDIFF: f64 = 0.0;
    let mut OBSPOS = StackArray::<f64, 3>::new(1..=3);
    let mut PREVET: f64 = 0.0;
    let mut PREVLT: f64 = 0.0;
    let mut RADII = StackArray::<f64, 3>::new(1..=3);
    let mut RANGE: f64 = 0.0;
    let mut S: f64 = 0.0;
    let mut SSBOST = StackArray::<f64, 6>::new(1..=6);
    let mut SSBTST = StackArray::<f64, 6>::new(1..=6);
    let mut STLOFF = StackArray::<f64, 3>::new(1..=3);
    let mut SUBVEC = StackArray::<f64, 3>::new(1..=3);
    let mut SUBVJ2 = StackArray::<f64, 3>::new(1..=3);
    let mut TPOS = StackArray::<f64, 3>::new(1..=3);
    let mut VTEMP = StackArray::<f64, 3>::new(1..=3);
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
    // Check the input body codes.  If they are equal, signal
    // an error.
    //
    if (OBSCDE == TRGCDE) {
        SETMSG(b"In computing the sub-observer point, the observing body and target body are the same. Both are #.", ctx);
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
        SETMSG(b"Reference frame # is not centered at the the target body #. The ID code of the frame center is #.", ctx);
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
    // If necessary, parse the method specification. PRVMTH records the
    // last valid value of METHOD; PRI, NEAR, SHAPE, NSURF, and SRFLST
    // are the corresponding saved variables.
    //
    if ((save.FIRST || SURFUP) || fstr::ne(METHOD, &save.PRVMTH)) {
        //
        // Make sure parsed values from METHOD can't be reused before
        // we've checked them.
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
            &mut save.SUBTYP,
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

        if fstr::eq(&save.SUBTYP, b" ") {
            SETMSG(
                b"Sub-observer point type was invalid or was not found in the method string #.",
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
            // Allow both "near point" and "nadir" expressions the
            // ellipsoid case, since in these case, these are equivalent.
            //
            save.NEAR = (EQSTR(&save.SUBTYP, b"NEAR POINT") || EQSTR(&save.SUBTYP, b"NADIR"));
        } else {
            //
            // "near point" is not supported for DSKs.
            //
            save.NEAR = EQSTR(&save.SUBTYP, b"NADIR");
        }

        if !save.NEAR {
            if !EQSTR(&save.SUBTYP, b"INTERCEPT") {
                SETMSG(
                    b"Invalid sub-observer point type <#> was found in the method string #.",
                    ctx,
                );
                ERRCH(b"#", &save.SUBTYP, ctx);
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
        SETMSG(b"Computation method argument was <#>; this string must specify a supported shape model and computation type. See the description of METHOD in the header of SUBPNT for details.", ctx);
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
        if save.XMIT {
            S = 1.0;
        } else {
            S = -1.0;
        }
    } else {
        S = 0.0;
    }

    //
    // Determine the position of the observer in the target body-fixed
    // frame. This is a first estimate.
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
    //        that corrected vector in order to compute the sub-observer
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
    // Find the sub-observer point given the target epoch,
    // observer-target position, and target body orientation we've
    // already computed. If we're not using light time correction, this
    // is all we need do. Otherwise, our result will give us an initial
    // estimate of the target epoch, which we'll then improve.
    //

    //
    // Get the radii of the target body from the kernel pool.
    //
    ZZGFTREB(TRGCDE, RADII.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    RANGE = VNORM(OBSPOS.as_slice());

    if (RANGE == 0.0) {
        //
        // We've already ensured that observer and target are
        // distinct, so this should be a very unusual occurrence.
        //
        SETMSG(
            b"Observer-target distance is zero. Observer is #; target is #.",
            ctx,
        );
        ERRCH(b"#", OBSRVR, ctx);
        ERRCH(b"#", TARGET, ctx);
        SIGERR(b"SPICE(NOSEPARATION)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Make a first estimate of the sub-observer point. The algorithm
    // we use depends on the sub-observer point definition.
    //
    if save.NEAR {
        //
        // Locate the nearest point to the observer on the target
        // body's reference ellipsoid.
        //
        NEARPT(
            OBSPOS.as_slice(),
            RADII[1],
            RADII[2],
            RADII[3],
            SPOINT.as_slice_mut(),
            &mut ALT,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // If the target is an ellipsoid, the NEARPT call above does
        // the trick. For DSKs, we define a ray emanating from the
        // observer and passing through the near point on the
        // reference ellipsoid. The closest ray-DSK surface intercept
        // to the observer is the initial estimate of the sub-point.
        //
        if (save.SHAPE == DSKSHP) {
            //
            // Generate the ray direction; find the DSK intercept.
            //
            VSUB(SPOINT.as_slice(), OBSPOS.as_slice(), DVEC.as_slice_mut());

            ZZSBFXR(
                TRGCDE,
                save.NSURF,
                save.SRFLST.as_slice(),
                *TRGEPC,
                FIXFID,
                OBSPOS.as_slice(),
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
                SETMSG(b"No sub-observer point was found on the surface defined by DSK data.Observer is #; target is #. This problem can occur for bodies having shapes not well modeled by ellipsoids. Consider using the \"Intercept: DSK\" computation method.", ctx);
                ERRCH(b"#", OBSRVR, ctx);
                ERRCH(b"#", TARGET, ctx);
                SIGERR(b"SPICE(SUBPOINTNOTFOUND)", ctx)?;
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            //
            // Re-compute the altitude using the intercept on the DSK
            // surface.
            //
            VSUB(SPOINT.as_slice(), OBSPOS.as_slice(), SRFVEC.as_slice_mut());

            ALT = VNORM(SRFVEC.as_slice());
        }
    } else {
        //
        // This is the case for the "intercept" sub-point definition.
        //
        if (save.SHAPE == ELLSHP) {
            //
            // Locate the surface intercept of the ray from the
            // observer to the target center.
            //
            SURFPT(
                OBSPOS.as_slice(),
                TPOS.as_slice(),
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
                SETMSG(b"No intercept of observer-target ray was found.", ctx);
                SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            ALT = VDIST(OBSPOS.as_slice(), SPOINT.as_slice());
        } else {
            //
            // Generate the ray direction; find the DSK intercept.
            //
            VMINUS(OBSPOS.as_slice(), DVEC.as_slice_mut());

            ZZSBFXR(
                TRGCDE,
                save.NSURF,
                save.SRFLST.as_slice(),
                *TRGEPC,
                FIXFID,
                OBSPOS.as_slice(),
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
                SETMSG(b"No sub-observer point was found on the surface defined by DSK data.Observer is #; target is #. This problem can occur for a body having an irregular shape such that the origin of the body-fixed reference frame is outside of the body. A torus is an example of such a shape.", ctx);
                ERRCH(b"#", OBSRVR, ctx);
                ERRCH(b"#", TARGET, ctx);
                SIGERR(b"SPICE(SUBPOINTNOTFOUND)", ctx)?;
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }
            //
            // Re-compute the altitude using the intercept on the DSK
            // surface.
            //
            VSUB(SPOINT.as_slice(), OBSPOS.as_slice(), SRFVEC.as_slice_mut());

            ALT = VNORM(SRFVEC.as_slice());
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

    if !save.USELT {
        *TRGEPC = ET;
        //
        // The TRGEPC value we'll return is just the input time.
        // The previous call to SPKEZP call yielded
        // the vector OBSPOS. SPOINT was set immediately above. The
        // only output left to compute is SRFVEC.
        //
        VSUB(SPOINT.as_slice(), OBSPOS.as_slice(), SRFVEC.as_slice_mut());

        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Compute the one-way light time and target epoch based on our
    // first computation of SPOINT. The coefficient S has been
    // set to give us the correct answer for each aberration
    // correction case.
    //
    LT = (ALT / CLIGHT());
    *TRGEPC = (ET + (S * LT));

    //
    // We'll now make an improved sub-observer point estimate using
    // the previous estimate of the sub-observer point. The number of
    // iterations depends on the light time correction type.

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
        // If we're using stellar aberration corrections, adjust the
        // observer position to account for the stellar aberration
        // correction applicable to SPOINT.
        //
        if save.USESTL {
            //
            // We want to apply the stellar aberration correction that
            // applies to our current estimate of the sub-observer point
            // location, NOT the correction for the target body's center.
            // In most cases the two corrections will be similar, but they
            // might not be---consider the case of a highly prolate target
            // body where the observer is close to one "end" of the body.
            //
            // Find the vector from the observer to the estimated
            // sub-observer point. Find the stellar aberration offset
            // STLOFF for this vector. Note that all vectors are expressed
            // relative to the target body-fixed frame at TRGEPC. We must
            // perform our corrections in an inertial frame.
            //
            VSUB(SPOINT.as_slice(), OBSPOS.as_slice(), SUBVEC.as_slice_mut());

            MTXV(XFORM.as_slice(), SUBVEC.as_slice(), SUBVJ2.as_slice_mut());

            if save.XMIT {
                STLABX(
                    SUBVJ2.as_slice(),
                    SSBOST.subarray(4),
                    CORVJ2.as_slice_mut(),
                    ctx,
                )?;
            } else {
                STELAB(
                    SUBVJ2.as_slice(),
                    SSBOST.subarray(4),
                    CORVJ2.as_slice_mut(),
                    ctx,
                )?;
            }

            MXV(XFORM.as_slice(), CORVJ2.as_slice(), CORPOS.as_slice_mut());
            VSUB(CORPOS.as_slice(), SUBVEC.as_slice(), STLOFF.as_slice_mut());

            //
            // In principle, we want to shift the target body position
            // relative to the solar system barycenter by STLOFF, but we
            // can skip this step and just re-compute the observer's
            // location relative to the target body's center by
            // subtracting off STLOFF.
            //
            VSUB(OBSPOS.as_slice(), STLOFF.as_slice(), VTEMP.as_slice_mut());
            VEQU(VTEMP.as_slice(), OBSPOS.as_slice_mut());
        }

        //
        // Find the sub-observer point using the current estimated
        // geometry.
        //
        if save.NEAR {
            //
            // Locate the nearest point to the observer on the target's
            // reference ellipsoid.
            //
            NEARPT(
                OBSPOS.as_slice(),
                RADII[1],
                RADII[2],
                RADII[3],
                SPOINT.as_slice_mut(),
                &mut ALT,
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            //
            // If the target is an ellipsoid, the NEARPT call above
            // does the trick. For DSKs, we define a ray emanating from
            // the observer and passing through the near point on the
            // reference ellipsoid. The closest ray-DSK surface
            // intercept to the observer is the initial estimate of the
            // sub-point.

            if (save.SHAPE == DSKSHP) {
                //
                // Generate the ray direction; find the DSK intercept.
                //
                VSUB(SPOINT.as_slice(), OBSPOS.as_slice(), DVEC.as_slice_mut());

                ZZSBFXR(
                    TRGCDE,
                    save.NSURF,
                    save.SRFLST.as_slice(),
                    *TRGEPC,
                    FIXFID,
                    OBSPOS.as_slice(),
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
                    SETMSG(b"No sub-observer point was found on the surface defined by DSK data.Observer is #; target is #. This problem can occur for bodies having shapes not well modeled by ellipsoids. Consider using the \"Intercept: DSK\" computation method.", ctx);
                    ERRCH(b"#", OBSRVR, ctx);
                    ERRCH(b"#", TARGET, ctx);
                    SIGERR(b"SPICE(SUBPOINTNOTFOUND)", ctx)?;
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }

                //
                // Re-compute the altitude using the intercept on the
                // DSK surface.
                //
                VSUB(SPOINT.as_slice(), OBSPOS.as_slice(), SRFVEC.as_slice_mut());

                ALT = VNORM(SRFVEC.as_slice());
            }
        } else {
            //
            // This is the "intercept" case.
            //
            // Generate the ray direction.
            //
            VMINUS(OBSPOS.as_slice(), DVEC.as_slice_mut());
            //
            // Locate the surface intercept of the ray from the
            // observer to the target center.
            //
            if (save.SHAPE == ELLSHP) {
                SURFPT(
                    OBSPOS.as_slice(),
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
                    // If there's no intercept, we have a numerical
                    // problem.
                    //
                    SETMSG(b"No intercept of observer-target ray was found.", ctx);
                    SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }

                ALT = VDIST(OBSPOS.as_slice(), SPOINT.as_slice());
            } else {
                //
                // Find the ray-DSK surface intercept.
                //
                ZZSBFXR(
                    TRGCDE,
                    save.NSURF,
                    save.SRFLST.as_slice(),
                    *TRGEPC,
                    FIXFID,
                    OBSPOS.as_slice(),
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
                    SETMSG(b"No sub-observer point was found on the surface defined by DSK data.Observer is #; target is #. This problem can occur for a body having an irregular shape such that the origin of the body-fixed reference frame is outside of the body. A torus is an example of such a shape.", ctx);
                    ERRCH(b"#", OBSRVR, ctx);
                    ERRCH(b"#", TARGET, ctx);
                    SIGERR(b"SPICE(SUBPOINTNOTFOUND)", ctx)?;
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }
                //
                // Compute the altitude using the intercept on the DSK
                // surface.
                //
                VSUB(SPOINT.as_slice(), OBSPOS.as_slice(), SRFVEC.as_slice_mut());

                ALT = VNORM(SRFVEC.as_slice());
            }
        }

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // Compute a new light time estimate and new target epoch.
        //
        LT = (ALT / CLIGHT());
        *TRGEPC = (ET + (S * LT));

        //
        // At this point, we have new estimates of the sub-observer
        // point SPOINT, the observer altitude ALT, the target epoch
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
    }

    //
    // SPOINT, TRGEPC, and OBSPOS have been set at this point. Compute
    // SRFVEC.
    //
    VSUB(SPOINT.as_slice(), OBSPOS.as_slice(), SRFVEC.as_slice_mut());

    CHKOUT(RNAME, ctx)?;
    Ok(())
}
