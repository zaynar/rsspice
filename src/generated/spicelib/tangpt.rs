//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const INERTL: i32 = 1;
const PCK: i32 = (INERTL + 1);
const CK: i32 = (PCK + 1);
const TK: i32 = (CK + 1);
const DYN: i32 = (TK + 1);
const SWTCH: i32 = (DYN + 1);
const ALL: i32 = -1;
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
const CNVLIM: f64 = 0.0000000000000001;
const MXCVIT: i32 = 10;
const MXNCIT: i32 = 3;
const BDNMLN: i32 = 36;
const FRNMLN: i32 = 32;
const LOCLEN: i32 = 15;
const J2CODE: i32 = 1;

struct SaveVars {
    PRVCOR: Vec<u8>,
    PRVLOC: Vec<u8>,
    FIRST: bool,
    TANLOC: bool,
    USECN: bool,
    USELT: bool,
    USESTL: bool,
    XMIT: bool,
    PRVTCD: i32,
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
    SVRADI: StackArray<f64, 3>,
    SVNRAD: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PRVCOR = vec![b' '; CORLEN as usize];
        let mut PRVLOC = vec![b' '; LOCLEN as usize];
        let mut FIRST: bool = false;
        let mut TANLOC: bool = false;
        let mut USECN: bool = false;
        let mut USELT: bool = false;
        let mut USESTL: bool = false;
        let mut XMIT: bool = false;
        let mut PRVTCD: i32 = 0;
        let mut SVCTR1 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVTARG = vec![b' '; BDNMLN as usize];
        let mut SVTCDE: i32 = 0;
        let mut SVFND1: bool = false;
        let mut SVCTR2 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVOBSR = vec![b' '; BDNMLN as usize];
        let mut SVOBSC: i32 = 0;
        let mut SVFND2: bool = false;
        let mut SVCTR3 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVFREF = vec![b' '; FRNMLN as usize];
        let mut SVFXFC: i32 = 0;
        let mut SVCTR4 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVDREF = vec![b' '; FRNMLN as usize];
        let mut SVDFRC: i32 = 0;
        let mut SVCTR5 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVRADI = StackArray::<f64, 3>::new(1..=3);
        let mut SVNRAD: i32 = 0;

        FIRST = true;
        fstr::assign(&mut PRVCOR, b" ");
        fstr::assign(&mut PRVLOC, b" ");
        PRVTCD = 0;
        TANLOC = false;
        USECN = false;
        USELT = false;
        USESTL = false;
        XMIT = false;

        Self {
            PRVCOR,
            PRVLOC,
            FIRST,
            TANLOC,
            USECN,
            USELT,
            USESTL,
            XMIT,
            PRVTCD,
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
            SVRADI,
            SVNRAD,
        }
    }
}

/// Ray-ellipsoid tangent point
///
/// Compute, for a given observer, ray emanating from the observer,
/// and target, the "tangent point": the point on the ray nearest
/// to the target's surface. Also compute the point on the target's
/// surface nearest to the tangent point.
///
/// The locations of both points are optionally corrected for light
/// time and stellar aberration.
///
/// The surface shape is modeled as a triaxial ellipsoid.
///
/// # Required Reading
///
/// * [ABCORR](crate::required_reading::abcorr)
/// * [CK](crate::required_reading::ck)
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
///  CORLOC     I   Aberration correction locus: 'TANGENT POINT' or
///                 'SURFACE POINT'.
///  OBSRVR     I   Name of observing body.
///  DREF       I   Reference frame of ray direction vector.
///  DVEC       I   Ray direction vector.
///  TANPT      O   "Tangent point": point on ray nearest to surface.
///  ALT        O   Altitude of tangent point above surface.
///  RANGE      O   Distance of tangent point from observer.
///  SRFPT      O   Point on surface nearest to tangent point.
///  TRGEPC     O   Epoch associated with correction locus.
///  SRFVEC     O   Vector from observer to surface point SRFPT.
/// ```
///
/// # Detailed Input
///
/// ```text
///  METHOD   is a short string providing parameters defining
///           the computation method to be used.
///
///           METHOD is currently restricted to the value
///
///              'ELLIPSOID'
///
///           This value indicates that the target shape is
///           modeled as a triaxial ellipsoid.
///
///           METHOD is case-insensitive, and leading and trailing
///           blanks in METHOD are not significant.
///
///  TARGET   is the name of the target body. TARGET is
///           case-insensitive, and leading and trailing blanks in
///           TARGET are not significant. Optionally, you may
///           supply a string containing the integer ID code
///           for the object. For example both 'MOON' and '301'
///           are legitimate strings that indicate the Moon is the
///           target body.
///
///           If the target is identified by name rather than ID code,
///           the target name must be recognized by SPICE. Radii
///           defining a triaxial ellipsoid target shape model must be
///           available in the kernel pool. See the $Files section
///           below.
///
///  ET       is the epoch associated with the observer, expressed as
///           ephemeris seconds past J2000 TDB. ET is the epoch at
///           which radiation is received by the observer, when an
///           observation is made, or in the case of transmission from
///           the observer, at which radiation is emitted.
///
///           ET is the epoch at which the state of the observer
///           relative to the solar system barycenter is computed.
///
///           When aberration corrections are not used, ET is also
///           the epoch at which the state and orientation of the
///           target body are computed.
///
///           When aberration corrections are used, the position
///           and orientation of the target body are computed at
///           ET-LT or ET+LT, where LT is the one-way light time
///           between the aberration correction locus and the
///           observer. The sign applied to LT depends on the
///           selected correction. See the descriptions of ABCORR
///           and CORLOC below for details.
///
///  FIXREF   is the name of a body-fixed reference frame centered on
///           the target body. FIXREF may be any such frame supported
///           by the SPICE system, including built-in frames
///           (documented in frames.req) and frames defined by a
///           loaded frame kernel (FK). The string FIXREF is
///           case-insensitive, and leading and trailing blanks in
///           FIXREF are not significant.
///
///           The output points TANPT and SRFPT, and the
///           observer-to-surface point vector SRFVEC will be
///           expressed relative to this reference frame.
///
///  ABCORR   indicates the aberration corrections to be applied
///           when computing the target's position and orientation.
///
///           See the description of the aberration correction
///           locus CORLOC for further details on how aberration
///           corrections are applied.
///
///           For remote sensing applications, where the apparent
///           tangent or surface point seen by the observer is
///           desired, normally one of the corrections
///
///              'CN+S' or 'NONE'
///
///           should be selected. For applications involving
///           transmission from the observer, normally 'XCN+S' or
///           'NONE' should be selected.
///
///           Light-time-only corrections can be useful for
///           testing but generally don't accurately model geometry
///           applicable to remote sensing observations or signal
///           transmission.
///
///           The supported options are described below.
///
///           ABCORR may be any of the following:
///
///              'NONE'     Compute outputs without applying
///                         aberration corrections.
///
///                         'NONE' may be suitable when the
///                         magnitudes of the aberration
///                         corrections are negligible.
///
///           Let LT represent the one-way light time between the
///           observer and the aberration correction locus specified
///           by CORLOC. The following values of ABCORR apply to the
///           "reception" case in which radiation departs from the
///           aberration correction locus at the light-time corrected
///           epoch ET-LT and arrives at the observer's location at
///           ET:
///
///              'LT'       Correct for one-way light time between
///                         the aberration correction locus and
///                         the observer, using a Newtonian
///                         formulation. This correction yields the
///                         position of the aberration correction
///                         locus at the moment it emitted radiation
///                         arriving at the observer at ET.
///
///                         The light time correction uses an
///                         iterative solution of the light time
///                         equation. The solution invoked by the
///                         'LT' option uses several iterations
///                         but does not guarantee convergence.
///
///                         Both the target position as seen by the
///                         observer, and rotation of the target
///                         body, are corrected for light time.
///
///              'LT+S'     Correct for one-way light time and
///                         stellar aberration using a Newtonian
///                         formulation. This option modifies the
///                         aberration correction locus solution
///                         obtained with the 'LT' option to
///                         account for the observer's velocity
///                         relative to the solar system
///                         barycenter. These corrections yield the
///                         apparent aberration correction locus.
///
///              'CN'       Converged Newtonian light time
///                         correction. In solving the light time
///                         equation, the 'CN' correction iterates
///                         until either the solution converges or
///                         a large iteration limit is reached.
///                         Both the position and orientation of
///                         the target body are corrected for light
///                         time.
///
///              'CN+S'     Converged Newtonian light time and stellar
///                         aberration corrections. This option
///                         produces a solution that is at least as
///                         accurate at that obtainable with the
///                         'LT+S' option. Whether the 'CN+S' solution
///                         is substantially more accurate depends on
///                         the geometry of the participating objects
///                         and on the accuracy of the input data. In
///                         some cases this routine will execute more
///                         slowly when a converged solution is
///                         computed.
///
///                         For reception-case applications where
///                         aberration corrections are applied, this
///                         option should be used, unless the
///                         magnitudes of the corrections are
///                         negligible.
///
///           The following values of ABCORR apply to the
///           "transmission" case in which radiation *departs* from
///           the observer's location at ET and arrives at the
///           aberration correction locus at the light-time
///           corrected epoch ET+LT:
///
///              'XLT'      "Transmission" case: correct for
///                         one-way light time between the
///                         aberration correction locus and the
///                         observer, using a Newtonian
///                         formulation. This correction yields the
///                         position of the aberration correction
///                         locus at the moment it receives radiation
///                         emitted from the observer's location at
///                         ET.
///
///                         The light time correction uses an
///                         iterative solution of the light time
///                         equation. The solution invoked by the
///                         'XLT' option uses several iterations
///                         but does not guarantee convergence.
///
///                         Both the target position as seen by the
///                         observer, and rotation of the target
///                         body, are corrected for light time.
///
///              'XLT+S'    "Transmission" case: correct for
///                         one-way light time and stellar
///                         aberration using a Newtonian
///                         formulation. This option modifies the
///                         aberration correction locus solution
///                         obtained with the 'XLT' option to
///                         account for the observer's velocity
///                         relative to the solar system
///                         barycenter.
///
///                         Stellar aberration is computed for
///                         transmitted, rather than received,
///                         radiation.
///
///                         These corrections yield the analog for
///                         the transmission case of the apparent
///                         aberration correction locus.
///
///              'XCN'      "Transmission" case: converged Newtonian
///                         light time correction. In solving the
///                         light time equation, the 'XCN' correction
///                         iterates until either the solution
///                         converges or a large iteration limit is
///                         reached. Both the position and rotation of
///                         the target body are corrected for light
///                         time.
///
///              'XCN+S'    "Transmission" case: converged Newtonian
///                         light time and stellar aberration
///                         corrections. This option produces a
///                         solution that is at least as accurate at
///                         that obtainable with the 'XLT+S' option.
///                         Whether the 'XCN+S' solution is
///                         substantially more accurate depends on the
///                         geometry of the participating objects and
///                         on the accuracy of the input data. In some
///                         cases this routine will execute more
///                         slowly when a converged solution is
///                         computed.
///
///                         For transmission-case applications where
///                         aberration corrections are applied, this
///                         option should be used, unless the
///                         magnitudes of the corrections are
///                         negligible.
///
///           Case and embedded blanks are not significant in
///           ABCORR. For example, the string
///
///              'Cn + s'
///
///           is valid.
///
///  CORLOC   specifies the aberration correction "locus," which is
///           the fixed point in the frame designated by FIXREF for
///           which light time and stellar aberration corrections are
///           computed.
///
///           Differential aberration effects across the surface of
///           the target body are not considered by this routine. When
///           aberration corrections are used, the effective positions
///           of the observer and target, and the orientation of the
///           target, are computed according to the corrections
///           determined for the aberration correction locus.
///
///           The light time used to determine the position and
///           orientation of the target body is that between the
///           aberration correction locus and the observer.
///
///           The stellar aberration correction applied to the
///           position of the target is that computed for the
///           aberration correction locus.
///
///           The descriptions below apply only when aberration
///           corrections are used.
///
///           The values and meanings of CORLOC are:
///
///              'TANGENT POINT'    Compute corrections at the
///                                 "tangent point," which is the
///                                 point on the ray, defined by DREF
///                                 and DVEC, nearest to the target's
///                                 surface.
///
///              'SURFACE POINT'    Compute corrections at the
///                                 point on the target's surface
///                                 nearest to the tangent point.
///
///           Case and leading and trailing blanks are not significant
///           in CORLOC.
///
///  OBSRVR   is the name of the observing body. This is typically
///           a spacecraft or a surface point on an extended
///           ephemeris object. OBSRVR is case-insensitive, and
///           leading and trailing blanks in OBSRVR are not
///           significant. Optionally, you may supply a string
///           containing the integer ID code for the object. For
///           example both 'MOON' and '301' are legitimate strings
///           that indicate the Moon is the observer.
///
///           If the observer is identified by name rather than ID
///           code, the observer name must be recognized by SPICE. See
///           the $Files section below.
///
///  DREF     is the name of the reference frame relative to which
///           the ray direction vector is expressed. This may be
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
///           The tangent point on the ray and the point on the target
///           body's surface nearest to the tangent point are sought.
///
///           DVEC is specified relative to the reference frame
///           designated by DREF.
///
///           Non-inertial reference frames are treated as follows:
///           if the center of the frame is at the observer's
///           location, the frame's orientation is evaluated at ET.
///           If the frame's center is located elsewhere, then
///           letting LTCENT be the one-way light time between the
///           observer and the central body associated with the
///           frame, the orientation of the frame is evaluated at
///           ET-LTCENT, ET+LTCENT, or ET depending on whether the
///           requested aberration correction is, respectively, for
///           received radiation, transmitted radiation, or is
///           omitted. LTCENT is computed using the method
///           indicated by ABCORR.
/// ```
///
/// # Detailed Output
///
/// ```text
///  TANPT    is the "tangent point": the point on the ray defined by
///           DREF and DVEC nearest to the target body's surface.
///
///           TANPT is a vector originating at the target body's
///           center, expressed in the reference frame designated
///           by FIXREF, the orientation of which is evaluated at
///           TRGEPC (see description below). Units are km.
///
///           If the ray intersects the surface, TANPT is the
///           nearest point of intersection to the observer.
///
///           If the ray points away from the surface---that is, if
///           the angle between the ray and the outward normal at the
///           target surface point nearest to the observer, computed
///           using the specified aberration corrections, is less than
///           or equal to 90 degrees---then TANPT is set to the
///           position of the observer relative to the target center.
///
///           TANPT is computed using the aberration corrections
///           specified by ABCORR and CORLOC.
///
///           When the aberration correction locus is set to
///           'TANGENT POINT', and the position of TANPT is
///           corrected for aberration as specified by ABCORR, the
///           resulting point will lie on the input ray.
///
///  ALT      is the altitude of the tangent point above the
///           target body's surface. This is the distance between
///           TANPT and SRFPT. Units are km.
///
///           If the ray intersects the surface, ALT is set to the
///           exact double precision value 0.D0. ALT may be used as
///           an indicator of whether a ray-surface intersection
///           exists.
///
///  RANGE    is the distance between the observer and the tangent
///           point. Units are km.
///
///           If the ray points away from the surface (see the
///           description of TANPT above), RANGE is set to the
///           exact double precision value 0.D0. RANGE may be used
///           as an indicator of whether this geometric condition
///           exists.
///
///  SRFPT    is the point on the target body's surface nearest to the
///           tangent point.
///
///           SRFPT is a vector originating at the target body's
///           center, expressed in the reference frame designated
///           by FIXREF, the orientation of which is evaluated at
///           TRGEPC (see description below). Units are km.
///
///           SRFPT is computed using the aberration corrections
///           specified by ABCORR and CORLOC.
///
///           When the aberration correction locus is set to
///           'SURFACE POINT', and the position of SRFPT is
///           corrected for aberration as specified by ABCORR, the
///           resulting point will lie on the ray emanating from
///           the observer and pointing in the direction of SRFVEC.
///
///           If the ray intersects the surface, SRFPT is the point of
///           intersection nearest to the observer.
///
///           If the ray points away from the surface (see the
///           description of TANPT above), SRFPT is set to the target
///           surface point nearest to the observer.
///
///  TRGEPC   is the epoch associated with the aberration correction
///           locus. TRGEPC is defined as follows: letting LT be the
///           one-way light time between the observer and the
///           aberration correction locus, TRGEPC is the epoch ET-LT,
///           ET+LT, or ET depending on whether the requested
///           aberration correction is, respectively, for received
///           radiation, transmitted radiation, or omitted. LT is
///           computed using the method indicated by ABCORR.
///
///           TRGEPC is expressed as seconds past J2000 TDB.
///
///           The name TRGEPC, which stands for "target epoch,"
///           is used for compatibility with other SPICE high-level
///           geometry routines. Note that the epoch it designates
///           is not associated with the target body's center.
///
///  SRFVEC   is the vector from the observer's position at ET to
///           the surface point SRFPT, where the position of SRFPT
///           is corrected for aberrations as specified by ABCORR
///           and CORLOC. SRFVEC is expressed in the target
///           body-fixed reference frame designated by FIXREF,
///           evaluated at TRGEPC. Units are km.
///
///           One can use the SPICELIB function VNORM to obtain the
///           distance between the observer and SRFPT:
///
///              DIST = VNORM ( SRFVEC )
///
///           The observer's position OBSPOS, relative to the
///           target body's center, where the center's position is
///           corrected for aberration effects as indicated by
///           ABCORR and CORLOC, can be computed via the call:
///
///              CALL VSUB ( SRFPT, SRFVEC, OBSPOS )
///
///           To transform the vector SRFVEC from the reference frame
///           FIXREF at time TRGEPC to a time-dependent reference
///           frame REF at time ET, the routine PXFRM2 should be
///           called. Let XFORM be the 3x3 matrix representing the
///           rotation from the reference frame FIXREF at time
///           TRGEPC to the reference frame REF at time ET. Then
///           SRFVEC can be transformed to the result REFVEC as
///           follows:
///
///              CALL PXFRM2 ( FIXREF, REF,    TRGEPC, ET, XFORM )
///              CALL MXV    ( XFORM,  SRFVEC, REFVEC )
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the specified aberration correction is unrecognized, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  2)  If METHOD is not equivalent to 'ELLIPSOID', when case and
///      blanks are ignored in the comparison, the error
///      SPICE(NOTSUPPORTED) is signaled.
///
///  3)  If CORLOC is not equivalent to either 'TANGENT POINT' or
///      'SURFACE POINT', when case and blanks are ignored, the
///      error SPICE(NOTSUPPORTED) is signaled.
///
///  4)  If the direction vector DVEC is the zero vector, the error
///      SPICE(ZEROVECTOR) is signaled.
///
///  5)  If either the target or observer input strings cannot be
///      converted to an integer ID code, the error
///      SPICE(IDCODENOTFOUND) is signaled.
///
///  6)  If OBSRVR and TARGET map to the same NAIF integer ID code,
///      the error SPICE(BODIESNOTDISTINCT) is signaled.
///
///  7)  If triaxial radii of the target body have not been loaded
///      into the kernel pool prior to a call to this routine, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  8)  If the number of radii associated with the target body is not
///      three, the error SPICE(INVALIDCOUNT) is signaled.
///
///  9)  If the input target body-fixed frame FIXREF is not
///      recognized, the error SPICE(NOFRAME) is signaled. A frame
///      name may fail to be recognized because a required frame
///      specification kernel has not been loaded; another cause is a
///      misspelling of the frame name.
///
///  10) If the input frame FIXREF is not centered at the target body,
///      the error SPICE(INVALIDFRAME) is signaled.
///
///  11) If the reference frame designated by DREF is not recognized
///      by the SPICE frame subsystem, the error SPICE(NOFRAME) is
///      signaled.
///
///  12) If insufficient ephemeris data have been loaded prior to
///      calling TANGPT, an error is signaled by a routine in the call
///      tree of this routine. Note that when light time correction is
///      used, sufficient ephemeris data must be available to
///      propagate the states of both observer and target to the solar
///      system barycenter. If light time correction is used and
///      the ray's frame DREF is non-inertial, sufficient ephemeris
///      data must be available to compute the state of that frame's
///      center relative to the solar system barycenter.
///
///  13) If the target and observer have distinct identities but are
///      at the same location (for example, the target is Mars and the
///      observer is the Mars barycenter), the error
///      SPICE(NOSEPARATION) is signaled.
///
///  14) The target must be an extended body: if any of the radii of
///      the target body are non-positive, an error is signaled by a
///      routine in the call tree of this routine.
///
///  15) If the observer does not coincide with the target, but the
///      observer is located inside the ellipsoid modeling the
///      target body's shape, the error SPICE(INVALIDGEOMETRY) is
///      signaled.
///
///  16) If the transformation between the ray frame DREF and the
///      J2000 frame cannot be computed, an error is signaled by a
///      routine in the call tree of this routine.
///
///  17) If the transformation between the J2000 frame and the
///      target body-fixed, body-centered frame FIXREF cannot be
///      computed, an error is signaled by a routine in the call tree
///      of this routine.
///
///  18) If the nearest point to the target on the line containing
///      the input ray cannot be computed, an error is signaled by a
///      routine in the call tree of this routine. This type of error
///      may result from degenerate geometry; for example, if after
///      scaling the reference ellipsoid axes to make the longest
///      semi-axis a unit vector, another scaled axis is so short that
///      its squared length underflows to zero, no result can be
///      computed.
///
///  19) It is not an error for the ray to intersect the target body
///      or to point away from it so that the nearest point
///      to the ellipsoid on the line containing the ray lies behind
///      the ray's vertex.
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
///     Typically ephemeris data are made available by loading one or
///     more SPK files via FURNSH.
///
///  -  PCK data: triaxial radii for the target body must be loaded
///     into the kernel pool. Typically this is done by loading a text
///     PCK file via FURNSH.
///
///  -  Target orientation data: rotation data for the target body
///     must be loaded. These may be provided in a text or binary PCK
///     file, or by a CK file.
///
///  The following data may be required:
///
///  -  SPK data: if aberration corrections are used, and if the ray
///     frame DREF is non-inertial, ephemeris data for that frame's
///     center must be loaded. The state of that object relative to
///     the solar system barycenter must be calculable from the
///     available ephemeris data.
///
///  -  Frame specifications: if a frame definition is required to
///     convert the observer and target states to the body-fixed frame
///     of the target, that definition must be available in the kernel
///     pool. Similarly, the frame definition required to map between
///     the frame designated by DREF and the target body-fixed frame
///     must be available. Typically the definitions of frames not
///     already built-in to SPICE are supplied by loading a frame
///     kernel.
///
///  -  Ray frame orientation data: if the frame to which DREF refers
///     is non-inertial, PCK or CK data for the frame's orientation
///     are required. If the frame is fixed to a spacecraft instrument
///     or structure, at least one CK file will be needed to permit
///     transformation of vectors between that frame and both the
///     J2000 and the target body-fixed frames.
///
///  -  Ray direction data: if the ray direction is defined by a
///     vector expressed in a spacecraft reference frame, an IK may be
///     required to provide the coordinates of the ray's direction in
///     that frame.
///
///  -  SCLK data: if a CK file is needed, an associated SCLK kernel
///     is required to enable conversion between encoded SCLK (used to
///     time-tag CK data) and barycentric dynamical time (TDB).
///
///  -  Leapseconds data: if SCLK data are needed, a leapseconds
///     kernel usually is needed as well.
///
///  -  Body name-ID mappings: if the target or observer name is
///     not built into the SPICE software, the mapping between the
///     name and the corresponding ID code must be present in the
///     kernel pool. Such mappings are usually introduced by loading
///     a frame kernel or other text kernel containing them.
///
///  In all cases, kernel data are normally loaded once per program
///  run, NOT every time this routine is called.
/// ```
///
/// # Particulars
///
/// ```text
///  Given an observer, the direction vector of a ray emanating from
///  the observer, and an extended target body represented by a
///  triaxial ellipsoid, TANGPT computes the "tangent point": a point
///  nearest to the target body's surface nearest to the ray. The
///  corresponding surface point nearest to the tangent point is
///  computed as well.
///
///  For remote sensing observations, for maximum accuracy, reception
///  light time and stellar aberration corrections should be used.
///  These corrections model observer-target-ray geometry as it is
///  observed.
///
///  For signal transmission applications, for maximum accuracy,
///  transmission light time and stellar aberration corrections should
///  be used. These corrections model the observer-target-ray geometry
///  that applies to the transmitted signal. For example, these
///  corrections are needed to calculate the minimum altitude of the
///  signal's path over the target body.
///
///  In some cases, the magnitudes of light time and stellar
///  aberration corrections are negligible. When these corrections
///  can be ignored, significantly faster execution can be achieved
///  by setting the input ABCORR to 'NONE'.
///
///  This routine ignores differential aberration effects over the
///  target body's surface: it computes corrections only at a
///  user-specified point, which is called the "aberration correction
///  locus." The user may select either the tangent point or
///  corresponding surface point as the locus. In many cases, the
///  differences between corrections for these points are very small.
///
///  The $Examples header section below presents geometric cases for
///  which aberration correction magnitudes are significant, and cases
///  for which they are not.
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
///  1) The following program computes tangent and surface points for
///     the MAVEN IUVS instrument. The observer is the MAVEN
///     spacecraft; the target body is Mars. The ray direction is
///     that of the boresight of the MAVEN IUVS instrument.
///
///     The aberration corrections used in this example are often
///     suitable for remote sensing observations: converged Newtonian
///     light time and stellar aberration "reception" corrections. In
///     some cases it is reasonable to omit aberration corrections;
///     see the second and third example programs below for
///     demonstrations of the effects of different aberration
///     correction choices.
///
///     In this example, the aberration correction locus is the
///     tangent point, meaning that converged light time and stellar
///     aberration corrections are computed for that point. The epoch
///     TRGEPC is used to compute the light time-corrected target
///     position and orientation, and the stellar aberration
///     correction applicable to the tangent point is applied to the
///     observer-target position vector, in order to model apparent
///     observation geometry.
///
///     Three geometric cases are covered by this example:
///
///        - The "normal" case, in which the ray defined by the
///          MAVEN IUVS boresight passes over Mars at low altitude.
///
///          In the example code, there are two computations that fall
///          into this category.
///
///        - The "intercept" case, in which the ray intersects Mars.
///
///        - The "look away" case, in which the elevation of the ray's
///          direction vector, measured from the local level plane at
///          the sub-spacecraft point, is greater than or equal to 0.
///          The aberration corrections used to compute the
///          sub-observer point for this case are those applicable to
///          the aberration correction locus.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: tangpt_ex1.tm
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
///        All kernels referenced by this meta-kernel are available
///        from the MAVEN SPICE PDS archive.
///
///        The names and contents of the kernels referenced
///        by this meta-kernel are as follows:
///
///        File name                          Contents
///        ---------                          --------
///        mar097s.bsp                        Mars satellite ephemeris
///        maven_iuvs_v11.ti                  MAVEN IUVS instrument
///                                           information
///        maven_orb_rec_201001_210101_v1.bsp MAVEN s/c ephemeris
///        mvn_v09.tf                         MAVEN frame
///                                           specifications
///        mvn_app_rel_201005_201011_v01.bc   MAVEN Articulated
///                                           Payload Platform
///                                           attitude
///        mvn_iuvs_rem_201001_201231_v01.bc  MAVEN IUVS instrument
///                                           internal mirror
///                                           attitude
///        mvn_sc_rel_201005_201011_v01.bc    MAVEN s/c attitude
///        mvn_sclkscet_00086.tsc             MAVEN SCLK coefficients
///        naif0012.tls                       Leapseconds
///        pck00010.tpc                       Planet and satellite
///                                           orientation and radii
///
///        \begindata
///
///           KERNELS_TO_LOAD = (
///              'mar097s.bsp',
///              'maven_iuvs_v11.ti',
///              'maven_orb_rec_201001_210101_v1.bsp',
///              'maven_v09.tf',
///              'mvn_app_rel_201005_201011_v01.bc',
///              'mvn_iuvs_rem_201001_201231_v01.bc',
///              'mvn_sc_rel_201005_201011_v01.bc',
///              'mvn_sclkscet_00086.tsc',
///              'naif0012.tls',
///              'pck00010.tpc' )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM TANGPT_EX1
///           IMPLICIT NONE
///
///     C
///     C     Parameters
///     C
///           CHARACTER*(*)         FMT1F7
///           PARAMETER           ( FMT1F7 = '(1X,A,F15.7)' )
///
///           CHARACTER*(*)         FMT3F7
///           PARAMETER           ( FMT3F7 = '(1X,A,3F15.7)' )
///
///           CHARACTER*(*)         META
///           PARAMETER           ( META   = 'tangpt_ex1.tm' )
///
///           CHARACTER*(*)         TIMFMT
///           PARAMETER           ( TIMFMT =
///          .          'YYYY-MM-DD HR:MN:SC.###### UTC ::RND' )
///
///           INTEGER               BDNMLN
///           PARAMETER           ( BDNMLN = 36 )
///
///           INTEGER               CORLEN
///           PARAMETER           ( CORLEN = 10 )
///
///           INTEGER               FRNMLN
///           PARAMETER           ( FRNMLN = 32 )
///
///           INTEGER               LNSIZE
///           PARAMETER           ( LNSIZE = 72 )
///
///           INTEGER               LOCLEN
///           PARAMETER           ( LOCLEN = 25 )
///
///           INTEGER               NCASE
///           PARAMETER           ( NCASE  =  3 )
///
///           INTEGER               NTIMES
///           PARAMETER           ( NTIMES =  4 )
///
///           INTEGER               ROOM
///           PARAMETER           ( ROOM   = 12 )
///
///           INTEGER               SHPLEN
///           PARAMETER           ( SHPLEN = 25 )
///
///           INTEGER               TIMLEN
///           PARAMETER           ( TIMLEN = 35 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(CORLEN)    ABCORR
///           CHARACTER*(LNSIZE)    CASENM
///           CHARACTER*(LNSIZE)    CASES  ( NCASE )
///           CHARACTER*(FRNMLN)    FIXREF
///           CHARACTER*(BDNMLN)    INSNAM
///           CHARACTER*(LOCLEN)    LOCUS
///           CHARACTER*(BDNMLN)    OBSRVR
///           CHARACTER*(FRNMLN)    RAYFRM
///           CHARACTER*(SHPLEN)    SHAPE
///           CHARACTER*(BDNMLN)    TARGET
///           CHARACTER*(TIMLEN)    TIMSTR
///           CHARACTER*(TIMLEN)    UTCTIM ( NTIMES )
///
///           DOUBLE PRECISION      ALT
///           DOUBLE PRECISION      BOUNDS ( 3, ROOM )
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      RANGE
///           DOUBLE PRECISION      RAYDIR ( 3 )
///           DOUBLE PRECISION      SRFPT  ( 3 )
///           DOUBLE PRECISION      SRFVEC ( 3 )
///           DOUBLE PRECISION      TANPT  ( 3 )
///           DOUBLE PRECISION      TRGEPC
///
///           INTEGER               I
///           INTEGER               NVEC
///
///     C
///     C     Initial values
///     C
///           DATA                  CASES /
///          .                      'Ray slightly above limb',
///          .                      'Intercept',
///          .                      'Look-away'    /
///
///           DATA                  INSNAM / 'MAVEN_IUVS' /
///
///           DATA                  UTCTIM /
///          .            '2020-10-11 16:01:43.000000 UTC',
///          .            '2020-10-11 16:17:43.000000 UTC',
///          .            '2020-10-11 16:49:07.000000 UTC',
///          .            '2020-10-11 17:12:08.000000 UTC' /
///
///     C
///     C     Load kernels.
///     C
///           CALL FURNSH ( META )
///
///           WRITE(*,*) ' '
///           WRITE(*, '(A)') 'Instrument: ' // INSNAM
///           WRITE(*,*) ' '
///
///     C
///     C     Get the instrument reference frame name and
///     C     the instrument boresight direction in the
///     C     instrument frame.
///     C
///           CALL GETFVN ( INSNAM, ROOM,   SHAPE,
///          .              RAYFRM, RAYDIR, NVEC, BOUNDS )
///
///     C
///     C     Initialize inputs to TANGPT, except for time.
///     C
///           TARGET = 'MARS'
///           OBSRVR = 'MAVEN'
///           FIXREF = 'IAU_MARS'
///           ABCORR = 'CN+S'
///           LOCUS  = 'TANGENT POINT'
///
///     C
///     C     Compute the apparent tangent point for each time.
///     C
///           WRITE(*,'(A)') 'Aberration correction:       ' // ABCORR
///           WRITE(*,'(A)') 'Aberration correction locus: ' // LOCUS
///
///           DO I = 1, NTIMES
///
///              CALL STR2ET ( UTCTIM(I), ET )
///
///              CALL TANGPT ( 'ELLIPSOID',
///          .                 TARGET, ET,     FIXREF, ABCORR,
///          .                 LOCUS,  OBSRVR, RAYFRM, RAYDIR,
///          .                 TANPT,  ALT,    RANGE,  SRFPT,
///          .                 TRGEPC, SRFVEC                 )
///
///     C
///     C        Set the label for the geometric case.
///     C
///              IF ( ALT .EQ. 0 ) THEN
///
///                 CASENM = CASES(2)
///
///              ELSE IF ( RANGE .EQ. 0.D0 ) THEN
///
///                 CASENM = CASES(3)
///              ELSE
///                 CASENM = CASES(1)
///              END IF
///
///     C
///     C        Convert the target epoch to a string for output.
///     C
///              CALL TIMOUT ( TRGEPC, TIMFMT, TIMSTR )
///
///              WRITE(*,*) ' '
///
///              WRITE( *, '(A)') '  Observation Time = ' // UTCTIM(I)
///              WRITE( *, '(A)') '  Target Time      = ' // TIMSTR
///
///              WRITE(*, FMT1F7) '   ALT    (km) = ', ALT
///              WRITE(*, FMT1F7) '   RANGE  (km) = ', RANGE
///              WRITE(*, FMT3F7) '   TANPT  (km) = ', TANPT
///              WRITE(*, FMT3F7) '   SRFPT  (km) = ', SRFPT
///              WRITE(*, FMT3F7) '   SRFVEC (km) = ', SRFVEC
///
///              WRITE( *, '(A)') '    Geometric case = ' // CASENM
///
///           END DO
///
///           WRITE(*,*) ' '
///
///           END
///
///
///     When this program was executed on a PC/Linux/gfortran/64-bit
///     platform, the output was:
///
///
///     Instrument: MAVEN_IUVS
///
///     Aberration correction:       CN+S
///     Aberration correction locus: TANGENT POINT
///
///       Observation Time = 2020-10-11 16:01:43.000000 UTC
///       Target Time      = 2020-10-11 16:01:42.983021 UTC
///         ALT    (km) =      99.4262977
///         RANGE  (km) =    5090.1928435
///         TANPT  (km) =   -2273.0408575   1072.4423944  -2415.6104827
///         SRFPT  (km) =   -2208.5678350   1042.0234063  -2346.3031728
///         SRFVEC (km) =   -2138.0677257   3050.4078643   3470.3929222
///         Geometric case = Ray slightly above limb
///
///       Observation Time = 2020-10-11 16:17:43.000000 UTC
///       Target Time      = 2020-10-11 16:17:42.993820 UTC
///         ALT    (km) =       0.0000000
///         RANGE  (km) =    1852.8381880
///         TANPT  (km) =     752.0909507  -1781.3912506  -2775.5390159
///         SRFPT  (km) =     752.0909507  -1781.3912506  -2775.5390159
///         SRFVEC (km) =    -700.9743439   1162.4766255   1261.0679662
///         Geometric case = Intercept
///
///       Observation Time = 2020-10-11 16:49:07.000000 UTC
///       Target Time      = 2020-10-11 16:49:06.998907 UTC
///         ALT    (km) =     218.2661426
///         RANGE  (km) =     327.7912133
///         TANPT  (km) =    2479.8672359  -1772.2350525   1931.8678816
///         SRFPT  (km) =    2330.3561559  -1665.3870838   1814.0966731
///         SRFVEC (km) =      77.3692694    325.9571470   -207.0099587
///         Geometric case = Ray slightly above limb
///
///       Observation Time = 2020-10-11 17:12:08.000000 UTC
///       Target Time      = 2020-10-11 17:12:08.000000 UTC
///         ALT    (km) =     969.2772042
///         RANGE  (km) =       0.0000000
///         TANPT  (km) =     -58.1087763   2034.6474343   3844.2010767
///         SRFPT  (km) =     -45.2530638   1584.5115999   2985.8825113
///         SRFVEC (km) =      12.8557125   -450.1358344   -858.3185654
///         Geometric case = Look-away
///
///
///  2) The following program computes tangent and surface points for
///     the MRO MCS A1 instrument, for a single epoch. The observer is
///     the MRO spacecraft; the target body is Mars. The ray direction
///     is that of the boresight of the MRO MCS A1 instrument.
///
///     The aberration corrections used in this example are converged
///     Newtonian light time and stellar aberration corrections,
///     converged Newtonian light time alone, and "none."
///
///     For remote sensing observations made by a spacecraft in low
///     orbit about Mars, both the combination of light time and
///     stellar aberration corrections and omission of aberration
///     corrections may be valid. See the output of this program and
///     of the third example program below for examples of how results
///     differ due to the choice of aberration corrections.
///
///     Use of light time corrections alone is presented to
///     illustrate, by way of contrast, the effect of this choice.
///     This choice can be useful for testing but is unlikely to be
///     correct for modeling actual observation geometry.
///
///     Separate computations are performed using both the tangent
///     point and the corresponding surface point---the nearest point
///     on the target surface to the tangent point---as the aberration
///     correction locus.
///
///     Three geometric cases are covered by this example:
///
///        - The "normal" case, in which the ray defined by the
///          MRO MCS A1 boresight passes over Mars at low altitude.
///
///          In the example code, there are two computations that fall
///          into this category.
///
///        - The "intercept" case, in which the ray intersects Mars.
///
///        - The "look away" case, in which the elevation of the ray's
///          direction vector, measured from the local level plane at
///          the sub-spacecraft point, is greater than or equal to 0.
///          The target position and orientation used for this
///          computation are the same as those used to compute the
///          aberration correction locus.
///
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File: tangpt_ex2.tm
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
///        All kernels referenced by this meta-kernel are available
///        from the MRO SPICE PDS archive.
///
///        The names and contents of the kernels referenced
///        by this meta-kernel are as follows:
///
///           File name                       Contents
///           ---------                       --------
///           mar097.bsp                      Mars satellite ephemeris
///           mro_mcs_psp_201001_201031.bc    MRO MCS attitude
///           mro_mcs_v10.ti                  MRO MCS instrument
///                                           information
///           mro_psp57_ssd_mro95a.bsp        MRO s/c ephemeris
///           mro_sc_psp_201027_201102.bc     MRO s/c bus attitude
///           mro_sclkscet_00095_65536.tsc    MRO SCLK coefficients
///           mro_v16.tf                      MRO frame specifications
///           naif0012.tls                    Leapseconds
///           pck00008.tpc                    Planet and satellite
///                                           orientation and radii
///
///        \begindata
///
///           KERNELS_TO_LOAD = (
///              'mar097.bsp',
///              'mro_mcs_psp_201001_201031.bc',
///              'mro_mcs_v10.ti',
///              'mro_psp57_ssd_mro95a.bsp',
///              'mro_sc_psp_201027_201102.bc',
///              'mro_sclkscet_00095_65536.tsc',
///              'mro_v16.tf',
///              'naif0012.tls',
///              'pck00008.tpc' )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM TANGPT_EX2
///           IMPLICIT NONE
///
///     C
///     C     Parameters
///     C
///           CHARACTER*(*)         FMT1F7
///           PARAMETER           ( FMT1F7  = '(1X,A,F15.7)' )
///
///           CHARACTER*(*)         FMT3F7
///           PARAMETER           ( FMT3F7  = '(1X,A,3F15.7)' )
///
///           CHARACTER*(*)         FMT1F4
///           PARAMETER           ( FMT1F4 = '(1X,A,F10.4)' )
///
///           CHARACTER*(*)         FMT3F4
///           PARAMETER           ( FMT3F4 = '(1X,A,3F10.4)' )
///
///           CHARACTER*(*)         META
///           PARAMETER           ( META   = 'tangpt_ex2.tm' )
///
///           CHARACTER*(*)         TIMFMT
///           PARAMETER           ( TIMFMT =
///          .          'YYYY-MM-DD HR:MN:SC.###### UTC ::RND' )
///
///           INTEGER               BDNMLN
///           PARAMETER           ( BDNMLN = 36 )
///
///           INTEGER               CORLEN
///           PARAMETER           ( CORLEN = 10 )
///
///           INTEGER               FRNMLN
///           PARAMETER           ( FRNMLN = 32 )
///
///           INTEGER               LOCLEN
///           PARAMETER           ( LOCLEN = 25 )
///
///           INTEGER               NCASE
///           PARAMETER           ( NCASE  =  5 )
///
///           INTEGER               ROOM
///           PARAMETER           ( ROOM   =  4 )
///
///           INTEGER               SHPLEN
///           PARAMETER           ( SHPLEN = 25 )
///
///           INTEGER               TIMLEN
///           PARAMETER           ( TIMLEN = 35 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(CORLEN)    ABCORR
///           CHARACTER*(CORLEN)    CORRS  ( NCASE )
///           CHARACTER*(FRNMLN)    FIXREF
///           CHARACTER*(BDNMLN)    INSNAM
///           CHARACTER*(LOCLEN)    LOCI   ( NCASE )
///           CHARACTER*(LOCLEN)    LOCUS
///           CHARACTER*(BDNMLN)    OBSRVR
///           CHARACTER*(FRNMLN)    RAYFRM
///           CHARACTER*(SHPLEN)    SHAPE
///           CHARACTER*(BDNMLN)    TARGET
///           CHARACTER*(TIMLEN)    TIMSTR
///           CHARACTER*(TIMLEN)    UTCTIM
///
///           DOUBLE PRECISION      ALT
///           DOUBLE PRECISION      BOUNDS ( 3, ROOM )
///           DOUBLE PRECISION      DIFF   ( 3 )
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      RANGE
///           DOUBLE PRECISION      RAYDIR ( 3 )
///           DOUBLE PRECISION      SRFPT  ( 3 )
///           DOUBLE PRECISION      SRFVEC ( 3 )
///           DOUBLE PRECISION      SVALT
///           DOUBLE PRECISION      SVEPOC
///           DOUBLE PRECISION      SVRANG
///           DOUBLE PRECISION      SVSRFP ( 3 )
///           DOUBLE PRECISION      SVSRFV ( 3 )
///           DOUBLE PRECISION      SVTANP ( 3 )
///           DOUBLE PRECISION      TANPT  ( 3 )
///           DOUBLE PRECISION      TRGEPC
///
///           INTEGER               I
///           INTEGER               NVEC
///
///     C
///     C     Initial values
///     C
///           DATA                  CORRS  / 'CN+S', 'CN+S',
///          .                               'CN',   'CN',
///          .                               'NONE'           /
///
///           DATA                  INSNAM / 'MRO_MCS_A1'     /
///
///           DATA                  LOCI   / 'TANGENT POINT',
///          .                               'SURFACE POINT',
///          .                               'TANGENT POINT',
///          .                               'SURFACE POINT',
///          .                               'TANGENT POINT'  /
///
///           DATA                  UTCTIM /
///          .              '2020-10-31 00:01:23.111492 UTC'  /
///
///     C
///     C     Load kernels.
///     C
///           CALL FURNSH ( META )
///
///           WRITE( *, * ) ' '
///           WRITE( *, '(A)') 'Instrument: ' // INSNAM
///
///     C
///     C     Get the instrument reference frame name and
///     C     the instrument boresight direction in the
///     C     instrument frame.
///     C
///           CALL GETFVN ( INSNAM, ROOM,   SHAPE,
///          .              RAYFRM, RAYDIR, NVEC, BOUNDS )
///
///     C
///     C     Initialize inputs to TANGPT that are common to all
///     C     cases.
///     C
///           TARGET = 'MARS'
///           OBSRVR = 'MRO'
///           FIXREF = 'IAU_MARS'
///
///     C
///     C     Compute the apparent tangent point for each case.
///     C
///           DO I = 1, NCASE
///
///              WRITE(*,*) ' '
///
///              ABCORR = CORRS(I)
///              LOCUS  = LOCI(I)
///
///              WRITE(*, '(A)') 'Aberration correction:       '
///          .   //               ABCORR
///
///              WRITE(*, '(A)') 'Aberration correction locus: '
///          .   //               LOCUS
///
///              WRITE(*,*) ' '
///
///              CALL STR2ET ( UTCTIM, ET )
///
///              CALL TANGPT ( 'ELLIPSOID',
///          .                 TARGET, ET,     FIXREF, ABCORR,
///          .                 LOCUS,  OBSRVR, RAYFRM, RAYDIR,
///          .                 TANPT,  ALT,    RANGE,  SRFPT,
///          .                 TRGEPC, SRFVEC                 )
///
///     C
///     C        Convert the target epoch to a string for output.
///     C
///              CALL TIMOUT ( TRGEPC, TIMFMT, TIMSTR )
///
///              WRITE(*, '(A)') '  Observation Time = '
///          .   //               UTCTIM
///
///              WRITE(*, '(A)') '  Target Time      = '
///          .   //                 TIMSTR
///
///              WRITE(*, FMT1F7) '   ALT    (km) = ', ALT
///              WRITE(*, FMT1F7) '   RANGE  (km) = ', RANGE
///              WRITE(*, FMT3F7) '   TANPT  (km) = ', TANPT
///              WRITE(*, FMT3F7) '   SRFPT  (km) = ', SRFPT
///              WRITE(*, FMT3F7) '   SRFVEC (km) = ', SRFVEC
///
///              IF ( I .EQ. 1 ) THEN
///     C
///     C           Save results for comparison.
///     C
///                 SVALT  = ALT
///                 SVEPOC = TRGEPC
///                 SVRANG = RANGE
///                 CALL VEQU( TANPT,  SVTANP )
///                 CALL VEQU( SRFPT,  SVSRFP )
///                 CALL VEQU( SRFVEC, SVSRFV )
///
///              ELSE
///     C
///     C           Compare results to CN+S, tangent point
///     C           locus case.
///     C
///                 WRITE(*,*) ' '
///
///                 WRITE(*, '(A)')
///          .      '  Differences from case 1 outputs:'
///
///                 WRITE(*, FMT1F4)
///          .      '   Target time delta (ms) = ',
///          .      1.D3 * ( TRGEPC - SVEPOC )
///
///                 WRITE(*, FMT1F4)
///          .      '   ALT    delta (m) = ',
///          .      1.D3 * ( ALT - SVALT )
///
///                 WRITE(*, FMT1F4)
///          .      '   RANGE  delta (m) = ',
///          .      1.D3 * ( RANGE - SVRANG  )
///
///                 CALL VSUB   ( TANPT, SVTANP, DIFF )
///                 CALL VSCLIP ( 1.D3,  DIFF )
///                 WRITE(*, FMT3F4)
///          .      '   TANPT  delta (m) = ', DIFF
///
///                 CALL VSUB   ( SRFPT, SVSRFP, DIFF )
///                 CALL VSCLIP ( 1.D3,  DIFF )
///                 WRITE(*, FMT3F4)
///          .      '   SRFPT  delta (m) = ', DIFF
///
///                 CALL VSUB   ( SRFVEC, SVSRFV, DIFF )
///                 CALL VSCLIP ( 1.D3,   DIFF )
///                 WRITE(*, FMT3F4)
///          .      '   SRFVEC delta (m) = ', DIFF
///
///              END IF
///
///              WRITE(*,*) ' '
///
///           END DO
///
///           END
///
///
///     When this program was executed on a PC/Linux/gfortran/64-bit
///     platform, the output was:
///
///
///     Instrument: MRO_MCS_A1
///
///     Aberration correction:       CN+S
///     Aberration correction locus: TANGENT POINT
///
///       Observation Time = 2020-10-31 00:01:23.111492 UTC
///       Target Time      = 2020-10-31 00:01:23.106946 UTC
///         ALT    (km) =      39.1034486
///         RANGE  (km) =    1362.8659249
///         TANPT  (km) =   -2530.9040220  -1630.9806346   1644.3612074
///         SRFPT  (km) =   -2502.1342299  -1612.4406294   1625.4496512
///         SRFVEC (km) =    -589.3842679   -234.0892764  -1206.9635473
///
///
///     Aberration correction:       CN+S
///     Aberration correction locus: SURFACE POINT
///
///       Observation Time = 2020-10-31 00:01:23.111492 UTC
///       Target Time      = 2020-10-31 00:01:23.106944 UTC
///         ALT    (km) =      39.1014434
///         RANGE  (km) =    1362.8679108
///         TANPT  (km) =   -2530.9025464  -1630.9796845   1644.3602376
///         SRFPT  (km) =   -2502.1342295  -1612.4406300   1625.4496511
///         SRFVEC (km) =    -589.3866439   -234.0905954  -1206.9643086
///
///       Differences from case 1 outputs:
///         Target time delta (ms) =    -0.0019
///         ALT    delta (m) =    -2.0052
///         RANGE  delta (m) =     1.9859
///         TANPT  delta (m) =     1.4757    0.9501   -0.9698
///         SRFPT  delta (m) =     0.0004   -0.0006   -0.0000
///         SRFVEC delta (m) =    -2.3760   -1.3189   -0.7614
///
///
///     Aberration correction:       CN
///     Aberration correction locus: TANGENT POINT
///
///       Observation Time = 2020-10-31 00:01:23.111492 UTC
///       Target Time      = 2020-10-31 00:01:23.106946 UTC
///         ALT    (km) =      39.1714711
///         RANGE  (km) =    1362.8658567
///         TANPT  (km) =   -2530.9135880  -1631.0820975   1644.3878335
///         SRFPT  (km) =   -2502.0942100  -1612.5090527   1625.4434517
///         SRFVEC (km) =    -589.3346511   -234.0562242  -1206.9963133
///
///       Differences from case 1 outputs:
///         Target time delta (ms) =     0.0000
///         ALT    delta (m) =    68.0225
///         RANGE  delta (m) =    -0.0683
///         TANPT  delta (m) =    -9.5660 -101.4629   26.6261
///         SRFPT  delta (m) =    40.0199  -68.4233   -6.1994
///         SRFVEC delta (m) =    49.6168   33.0522  -32.7661
///
///
///     Aberration correction:       CN
///     Aberration correction locus: SURFACE POINT
///
///       Observation Time = 2020-10-31 00:01:23.111492 UTC
///       Target Time      = 2020-10-31 00:01:23.106944 UTC
///         ALT    (km) =      39.1714973
///         RANGE  (km) =    1362.8658326
///         TANPT  (km) =   -2530.9135902  -1631.0821391   1644.3878436
///         SRFPT  (km) =   -2502.0941931  -1612.5090815   1625.4434492
///         SRFVEC (km) =    -589.3346210   -234.0562071  -1206.9963050
///
///       Differences from case 1 outputs:
///         Target time delta (ms) =    -0.0019
///         ALT    delta (m) =    68.0487
///         RANGE  delta (m) =    -0.0924
///         TANPT  delta (m) =    -9.5682 -101.5045   26.6362
///         SRFPT  delta (m) =    40.0368  -68.4521   -6.2020
///         SRFVEC delta (m) =    49.6469   33.0694  -32.7577
///
///
///     Aberration correction:       NONE
///     Aberration correction locus: TANGENT POINT
///
///       Observation Time = 2020-10-31 00:01:23.111492 UTC
///       Target Time      = 2020-10-31 00:01:23.111492 UTC
///         ALT    (km) =      39.1090103
///         RANGE  (km) =    1362.9233525
///         TANPT  (km) =   -2530.9082604  -1630.9831041   1644.3638384
///         SRFPT  (km) =   -2502.1343747  -1612.4404639   1625.4495931
///         SRFVEC (km) =    -589.4063032   -234.0970874  -1207.0162978
///
///       Differences from case 1 outputs:
///         Target time delta (ms) =     4.5460
///         ALT    delta (m) =     5.5616
///         RANGE  delta (m) =    57.4276
///         TANPT  delta (m) =    -4.2384   -2.4695    2.6310
///         SRFPT  delta (m) =    -0.1448    0.1655   -0.0581
///         SRFVEC delta (m) =   -22.0352   -7.8109  -52.7505
///
///
///  3) The following program computes tangent and surface points for
///     a ray pointing from the Goldstone DSN station DSS-14 to the
///     location of the MRO spacecraft, for a single epoch. The target
///     body is Mars.
///
///     The aberration corrections used in this example are
///
///        CN+S
///        XCN+S
///        CN
///        NONE
///
///     Results using CN+S corrections are computed for both locus
///     choices: TANGENT POINT and SURFACE POINT.
///
///     For each case other than the one using CN+S corrections for
///     the TANGENT POINT locus, differences between results for the
///     former and latter case are shown.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File: tangpt_ex3.tm
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
///        All kernels referenced by this meta-kernel are available
///        from the NAIF SPICE server in the generic kernels area
///        or from the MRO SPICE PDS archive.
///
///        The names and contents of the kernels referenced
///        by this meta-kernel are as follows:
///
///           File name                       Contents
///           ---------                       --------
///           mar097.bsp                      Mars satellite ephemeris
///           mro_psp57_ssd_mro95a.bsp        MRO s/c ephemeris
///           earthstns_itrf93_201023.bsp     DSN station locations
///           naif0012.tls                    Leapseconds
///           pck00010.tpc                    Planet and satellite
///                                           orientation and radii
///           earth_latest_high_prec.bpc      High accuracy Earth
///                                           attitude
///
///        \begindata
///
///           KERNELS_TO_LOAD = (
///              'mar097.bsp'
///              'mro_psp57_ssd_mro95a.bsp'
///              'earthstns_itrf93_201023.bsp'
///              'naif0012.tls'
///              'pck00010.tpc'
///              'earth_latest_high_prec.bpc' )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM TANGPT_EX3
///           IMPLICIT NONE
///
///     C
///     C     Parameters
///     C
///           CHARACTER*(*)         FMT1F3
///           PARAMETER           ( FMT1F3  = '(1X,A,F14.3)' )
///
///           CHARACTER*(*)         FMT3F3
///           PARAMETER           ( FMT3F3  = '(1X,A,3F14.3)' )
///
///           CHARACTER*(*)         FMT1F6
///           PARAMETER           ( FMT1F6  = '(1X,A,F14.6)' )
///
///           CHARACTER*(*)         FMTMF3
///           PARAMETER           ( FMTMF3 = '(1X,A,2F13.3,F10.3)' )
///
///           CHARACTER*(*)         META
///           PARAMETER           ( META   = 'tangpt_ex3.tm' )
///
///           CHARACTER*(*)         TIMFMT
///           PARAMETER           ( TIMFMT =
///          .          'YYYY-MM-DD HR:MN:SC.###### UTC ::RND' )
///
///           INTEGER               BDNMLN
///           PARAMETER           ( BDNMLN = 36 )
///
///           INTEGER               CORLEN
///           PARAMETER           ( CORLEN = 10 )
///
///           INTEGER               FRNMLN
///           PARAMETER           ( FRNMLN = 32 )
///
///           INTEGER               LOCLEN
///           PARAMETER           ( LOCLEN = 25 )
///
///           INTEGER               NCASE
///           PARAMETER           ( NCASE  =  5 )
///
///           INTEGER               TIMLEN
///           PARAMETER           ( TIMLEN = 35 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(CORLEN)    ABCORR
///           CHARACTER*(CORLEN)    CORRS  ( NCASE )
///           CHARACTER*(FRNMLN)    FIXREF
///           CHARACTER*(LOCLEN)    LOCI   ( NCASE )
///           CHARACTER*(LOCLEN)    LOCUS
///           CHARACTER*(BDNMLN)    OBSRVR
///           CHARACTER*(FRNMLN)    RAYFRM
///           CHARACTER*(BDNMLN)    SC
///           CHARACTER*(BDNMLN)    TARGET
///           CHARACTER*(TIMLEN)    TIMSTR
///           CHARACTER*(TIMLEN)    UTCTIM
///
///           DOUBLE PRECISION      ALT
///           DOUBLE PRECISION      DIFF   ( 3 )
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      RANGE
///           DOUBLE PRECISION      RAYDIR ( 3 )
///           DOUBLE PRECISION      RAYLT
///           DOUBLE PRECISION      SRFPT  ( 3 )
///           DOUBLE PRECISION      SRFVEC ( 3 )
///           DOUBLE PRECISION      SVALT
///           DOUBLE PRECISION      SVEPOC
///           DOUBLE PRECISION      SVRANG
///           DOUBLE PRECISION      SVSRFP ( 3 )
///           DOUBLE PRECISION      SVSRFV ( 3 )
///           DOUBLE PRECISION      SVTANP ( 3 )
///           DOUBLE PRECISION      TANPT  ( 3 )
///           DOUBLE PRECISION      TRGEPC
///
///           INTEGER               I
///
///     C
///     C     Initial values
///     C
///           DATA                  CORRS  / 'CN+S', 'XCN+S',
///          .                               'CN',   'NONE',
///          .                               'CN+S'          /
///
///           DATA                  LOCI   / 'TANGENT POINT',
///          .                               'TANGENT POINT',
///          .                               'TANGENT POINT',
///          .                               'TANGENT POINT',
///          .                               'SURFACE POINT'  /
///
///           DATA                  UTCTIM /
///          .              '2020-12-30 00:00:00 UTC'  /
///
///     C
///     C     Load kernels.
///     C
///           CALL FURNSH ( META )
///
///     C
///     C     Set name of spacecraft used to define ray direction.
///     C
///           SC = 'MRO'
///
///     C
///     C     Initialize inputs to TANGPT that are common to all
///     C     cases.
///     C
///           TARGET = 'MARS'
///           OBSRVR = 'DSS-14'
///           FIXREF = 'IAU_MARS'
///           RAYFRM = 'J2000'
///
///     C
///     C     Convert observation time to TDB seconds past J2000.
///     C
///           CALL STR2ET ( UTCTIM, ET )
///
///     C
///     C     Generate ray direction vector. Use apparent position
///     C     of the MRO spacecraft.
///     C
///           CALL SPKPOS ( SC,     ET,     RAYFRM,
///          .              'CN+S', OBSRVR, RAYDIR, RAYLT )
///
///           WRITE(*,*) ' '
///           WRITE( *, '(A)') 'Observer:   ' // OBSRVR
///           WRITE( *, '(A)') 'Target:     ' // TARGET
///           WRITE( *, '(A)') 'Spacecraft: ' // SC
///
///     C
///     C     Compute the apparent tangent point for each case.
///     C
///           DO I = 1, NCASE
///
///              WRITE(*,*) ' '
///
///              ABCORR = CORRS(I)
///              LOCUS  = LOCI(I)
///
///              WRITE(*, '(A)') 'Aberration correction:       '
///          .   //               ABCORR
///
///              WRITE(*, '(A)') 'Aberration correction locus: '
///          .   //               LOCUS
///
///              WRITE(*,*) ' '
///
///     C
///     C        Compute tangent point.
///     C
///              CALL TANGPT ( 'ELLIPSOID',
///          .                 TARGET, ET,     FIXREF, ABCORR,
///          .                 LOCUS,  OBSRVR, RAYFRM, RAYDIR,
///          .                 TANPT,  ALT,    RANGE,  SRFPT,
///          .                 TRGEPC, SRFVEC                 )
///
///     C
///     C        Convert the target epoch to a string for output.
///     C
///              CALL TIMOUT ( TRGEPC, TIMFMT, TIMSTR )
///
///              WRITE(*, '(A)') '  Observation Time = '
///          .   //               UTCTIM
///
///              WRITE(*, '(A)') '  Target Time      = '
///          .   //                 TIMSTR
///
///              WRITE(*, FMT1F3) '   ALT    (km) = ', ALT
///              WRITE(*, FMT1F3) '   RANGE  (km) = ', RANGE
///              WRITE(*, FMT3F3) '   TANPT  (km) = ', TANPT
///              WRITE(*, FMT3F3) '   SRFPT  (km) = ', SRFPT
///              WRITE(*, FMT3F3) '   SRFVEC (km) = ', SRFVEC
///
///              IF ( I .EQ. 1 ) THEN
///     C
///     C           Save results for comparison.
///     C
///                 SVALT  = ALT
///                 SVEPOC = TRGEPC
///                 SVRANG = RANGE
///                 CALL VEQU( TANPT,  SVTANP )
///                 CALL VEQU( SRFPT,  SVSRFP )
///                 CALL VEQU( SRFVEC, SVSRFV )
///
///              ELSE
///     C
///     C           Compare results to CN+S, tangent point
///     C           locus case.
///     C
///                 WRITE(*,*) ' '
///
///                 WRITE(*, '(A)')
///          .      '  Differences from case 1 outputs:'
///
///                 WRITE(*, FMT1F6)
///          .      '   Target time delta (s) = ',
///          .      TRGEPC - SVEPOC
///
///                 WRITE(*, FMTMF3)
///          .      '   ALT    delta (km) = ', ALT - SVALT
///
///                 WRITE(*, FMTMF3)
///          .      '   RANGE  delta (km) = ', RANGE - SVRANG
///
///                 CALL VSUB   ( TANPT, SVTANP, DIFF )
///                 WRITE(*, FMTMF3)
///          .      '   TANPT  delta (km) = ', DIFF
///
///                 CALL VSUB   ( SRFPT, SVSRFP, DIFF )
///                 WRITE(*, FMTMF3)
///          .      '   SRFPT  delta (km) = ', DIFF
///
///                 CALL VSUB   ( SRFVEC, SVSRFV, DIFF )
///                 WRITE(*, FMTMF3)
///          .      '   SRFVEC delta (km) = ', DIFF
///
///              END IF
///
///              WRITE(*,*) ' '
///
///           END DO
///
///           END
///
///
///     When this program was executed on a PC/Linux/gfortran/64-bit
///     platform, the output was:
///
///
///     Observer:   DSS-14
///     Target:     MARS
///     Spacecraft: MRO
///
///     Aberration correction:       CN+S
///     Aberration correction locus: TANGENT POINT
///
///       Observation Time = 2020-12-30 00:00:00 UTC
///       Target Time      = 2020-12-29 23:52:40.613204 UTC
///         ALT    (km) =        140.295
///         RANGE  (km) =  131724847.608
///         TANPT  (km) =       1351.574      1182.155     -3029.495
///         SRFPT  (km) =       1298.181      1135.455     -2908.454
///         SRFVEC (km) =  121233989.354  -5994858.328  51164606.676
///
///
///     Aberration correction:       XCN+S
///     Aberration correction locus: TANGENT POINT
///
///       Observation Time = 2020-12-30 00:00:00 UTC
///       Target Time      = 2020-12-30 00:07:19.347692 UTC
///         ALT    (km) =       4921.539
///         RANGE  (km) =  131713124.520
///         TANPT  (km) =       -413.404     -8220.856     -1193.471
///         SRFPT  (km) =       -168.808     -3356.879      -483.938
///         SRFVEC (km) =  120615301.766 -13523495.083  51160641.665
///
///       Differences from case 1 outputs:
///         Target time delta (s) =     878.734488
///         ALT    delta (km) =      4781.244
///         RANGE  delta (km) =    -11723.089
///         TANPT  delta (km) =     -1764.978    -9403.011  1836.024
///         SRFPT  delta (km) =     -1466.989    -4492.334  2424.517
///         SRFVEC delta (km) =   -618687.588 -7528636.755 -3965.012
///
///
///     Aberration correction:       CN
///     Aberration correction locus: TANGENT POINT
///
///       Observation Time = 2020-12-30 00:00:00 UTC
///       Target Time      = 2020-12-29 23:52:40.613219 UTC
///         ALT    (km) =       3409.162
///         RANGE  (km) =  131724843.177
///         TANPT  (km) =       1933.641      5183.696     -3951.091
///         SRFPT  (km) =        965.945      2589.501     -1962.095
///         SRFVEC (km) =  121233070.966  -5997405.747  51166472.910
///
///       Differences from case 1 outputs:
///         Target time delta (s) =       0.000015
///         ALT    delta (km) =      3268.868
///         RANGE  delta (km) =        -4.431
///         TANPT  delta (km) =       582.067     4001.541  -921.596
///         SRFPT  delta (km) =      -332.236     1454.046   946.360
///         SRFVEC delta (km) =      -918.388    -2547.420  1866.234
///
///
///     Aberration correction:       NONE
///     Aberration correction locus: TANGENT POINT
///
///       Observation Time = 2020-12-30 00:00:00 UTC
///       Target Time      = 2020-12-30 00:00:00.000000 UTC
///         ALT    (km) =        781.382
///         RANGE  (km) =  131718986.013
///         TANPT  (km) =        615.190     -3545.867     -2111.285
///         SRFPT  (km) =        500.266     -2883.463     -1713.075
///         SRFVEC (km) =  120983074.323  -9765994.151  51162607.074
///
///       Differences from case 1 outputs:
///         Target time delta (s) =     439.386796
///         ALT    delta (km) =       641.087
///         RANGE  delta (km) =     -5861.595
///         TANPT  delta (km) =      -736.384    -4728.022   918.210
///         SRFPT  delta (km) =      -797.915    -4018.919  1195.379
///         SRFVEC delta (km) =   -250915.031 -3771135.823 -1999.603
///
///
///     Aberration correction:       CN+S
///     Aberration correction locus: SURFACE POINT
///
///       Observation Time = 2020-12-30 00:00:00 UTC
///       Target Time      = 2020-12-29 23:52:40.613204 UTC
///         ALT    (km) =        140.308
///         RANGE  (km) =  131724847.611
///         TANPT  (km) =       1351.579      1182.159     -3029.507
///         SRFPT  (km) =       1298.181      1135.455     -2908.454
///         SRFVEC (km) =  121233989.351  -5994858.332  51164606.689
///
///       Differences from case 1 outputs:
///         Target time delta (s) =       0.000000
///         ALT    delta (km) =         0.013
///         RANGE  delta (km) =         0.003
///         TANPT  delta (km) =         0.005        0.004    -0.012
///         SRFPT  delta (km) =        -0.000        0.000     0.000
///         SRFVEC delta (km) =        -0.003       -0.005     0.013
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine is applicable only to computations for which
///      radiation paths can be modeled as straight lines.
///
///  2)  This routine does not account for differential aberration
///      corrections across the target body surface: when aberration
///      corrections are used, the entire target ellipsoid's position
///      and orientation are modified by the corrections that apply at
///      the aberration correction locus.
///
///  3)  A cautionary note: if aberration corrections are used, and if
///      DREF is the target body-fixed frame, the epoch at which that
///      frame is evaluated is offset from ET by the light time
///      between the observer and the *center* of the target body.
///      This light time normally will differ from the light time
///      between the observer and the tangent or surface point.
///      Consequently the orientation of the target body-fixed frame
///      at TRGEPC will not match that of the target body-fixed frame
///      at the epoch associated with DREF. As a result, various
///      derived quantities may not be as expected: for example,
///      SRFVEC would not be parallel to DVEC.
///
///      In many applications the errors arising from this frame
///      discrepancy may be insignificant; however a safe approach is
///      to always use as DREF a frame other than the target
///      body-fixed frame.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  M. Costa Sitja     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.0, 20-OCT-2021 (NJB) (MCS)
/// ```
pub fn tangpt(
    ctx: &mut SpiceContext,
    method: &str,
    target: &str,
    et: f64,
    fixref: &str,
    abcorr: &str,
    corloc: &str,
    obsrvr: &str,
    dref: &str,
    dvec: &[f64; 3],
    tanpt: &mut [f64; 3],
    alt: &mut f64,
    range: &mut f64,
    srfpt: &mut [f64; 3],
    trgepc: &mut f64,
    srfvec: &mut [f64; 3],
) -> crate::Result<()> {
    TANGPT(
        method.as_bytes(),
        target.as_bytes(),
        et,
        fixref.as_bytes(),
        abcorr.as_bytes(),
        corloc.as_bytes(),
        obsrvr.as_bytes(),
        dref.as_bytes(),
        dvec,
        tanpt,
        alt,
        range,
        srfpt,
        trgepc,
        srfvec,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure TANGPT ( Ray-ellipsoid tangent point )
pub fn TANGPT(
    METHOD: &[u8],
    TARGET: &[u8],
    ET: f64,
    FIXREF: &[u8],
    ABCORR: &[u8],
    CORLOC: &[u8],
    OBSRVR: &[u8],
    DREF: &[u8],
    DVEC: &[f64],
    TANPT: &mut [f64],
    ALT: &mut f64,
    RANGE: &mut f64,
    SRFPT: &mut [f64],
    TRGEPC: &mut f64,
    SRFVEC: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DVEC = DummyArray::new(DVEC, 1..=3);
    let mut TANPT = DummyArrayMut::new(TANPT, 1..=3);
    let mut SRFPT = DummyArrayMut::new(SRFPT, 1..=3);
    let mut SRFVEC = DummyArrayMut::new(SRFVEC, 1..=3);
    let mut LOCSTR = [b' '; LOCLEN as usize];
    let mut CTRPOS = StackArray::<f64, 3>::new(1..=3);
    let mut DIST: f64 = 0.0;
    let mut DVAL: f64 = 0.0;
    let mut EPCDIF: f64 = 0.0;
    let mut FIXDIR = StackArray::<f64, 3>::new(1..=3);
    let mut FIXOBS = StackArray::<f64, 3>::new(1..=3);
    let mut J2DIR = StackArray::<f64, 3>::new(1..=3);
    let mut J2FIXM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut J2LCUS = StackArray::<f64, 3>::new(1..=3);
    let mut J2LPOS = StackArray::<f64, 3>::new(1..=3);
    let mut J2OPOS = StackArray::<f64, 3>::new(1..=3);
    let mut J2TPOS = StackArray::<f64, 3>::new(1..=3);
    let mut LT: f64 = 0.0;
    let mut LTCENT: f64 = 0.0;
    let mut LTDIFF: f64 = 0.0;
    let mut P = StackArray::<f64, 3>::new(1..=3);
    let mut PREVLT: f64 = 0.0;
    let mut PRVEPC: f64 = 0.0;
    let mut PRVSRF = StackArray::<f64, 3>::new(1..=3);
    let mut PRVTAN = StackArray::<f64, 3>::new(1..=3);
    let mut R2JMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut REFEPC: f64 = 0.0;
    let mut S: f64 = 0.0;
    let mut SSBOST = StackArray::<f64, 6>::new(1..=6);
    let mut SSBTST = StackArray::<f64, 6>::new(1..=6);
    let mut STLFIX = StackArray::<f64, 3>::new(1..=3);
    let mut STLLOC = StackArray::<f64, 3>::new(1..=3);
    let mut STLOBS = StackArray::<f64, 3>::new(1..=3);
    let mut STLOFF = StackArray::<f64, 3>::new(1..=3);
    let mut TANOFF = StackArray::<f64, 3>::new(1..=3);
    let mut TPOS = StackArray::<f64, 3>::new(1..=3);
    let mut TRGPOS = StackArray::<f64, 3>::new(1..=3);
    let mut DCENTR: i32 = 0;
    let mut DCLASS: i32 = 0;
    let mut DFRCDE: i32 = 0;
    let mut DTYPID: i32 = 0;
    let mut FXCENT: i32 = 0;
    let mut FXCLSS: i32 = 0;
    let mut FXFCDE: i32 = 0;
    let mut FXTYID: i32 = 0;
    let mut I: i32 = 0;
    let mut NITR: i32 = 0;
    let mut OBSCDE: i32 = 0;
    let mut TRGCDE: i32 = 0;
    let mut ATTBLK = StackArray::<bool, 15>::new(1..=NABCOR);
    let mut FND: bool = false;
    let mut LTCNV: bool = false;
    let mut STLCNV: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Limit on relative error in light time; this is used to terminate
    // the solution loop.
    //
    // This value is meant to ensure timely loop termination in cases
    // where extended precision computations cause successive values to
    // differ by amounts having magnitude less than the minimum
    // relative value representable by IEEE-754 conforming, 64-bit
    // double precision floating point numbers.
    //
    // The convergence tests used here are not necessarily sensitive to
    // use of extended precision, but it is possible that future changes
    // to the code could make them so.
    //
    // In most situations, use of this value enforces convergence. In
    // rare cases, successive approximate solutions will differ by
    // small, non-zero amounts but will not converge. In those cases,
    // iteration will be terminated when the iteration count limit,
    // which is dependent on the choice of aberration correction, is
    // reached.
    //

    //
    // Upper bound on converged solution iterations.
    //

    //
    // Upper bound on non-converged solution iterations.
    //

    //
    // Saved body name length.
    //

    //
    // Saved frame name length.
    //

    //
    // Locus string length.
    //

    //
    // Code for the frame J2000.
    //

    //
    // Local variables
    //

    //
    // Saved body name/ID item declarations.
    //

    //
    // Saved frame name/ID item declarations.
    //

    //
    // Saved target radii declarations.
    //

    //
    // Saved surface name/ID item declarations. To be used if DSK
    // shapes are supported.
    //
    //  INTEGER               SVCTR6 ( CTRSIZ )

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
    // Saved target radii items.
    //

    //
    // To be used if DSK shapes are supported:
    //
    // Saved surface name/ID items.
    //
    //  SAVE                  SVCTR6
    //

    //
    // Initial values
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"TANGPT", ctx)?;

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
        //
        // To be used if DSK shapes are supported:
        //
        //  CALL ZZCTRUIN( SVCTR6 )
    }

    //
    // Parse the aberration correction specifier, if it's new.
    //
    if (save.FIRST || fstr::ne(ABCORR, &save.PRVCOR)) {
        //
        // PRVCOR is updated only when a valid correction has been
        // recognized. PRVCOR is blank on the first pass; afterward
        // it is always valid.
        //
        // The aberration correction flag differs from the value it
        // had on the previous call, if any. Analyze the new flag.
        //
        ZZVALCOR(ABCORR, ATTBLK.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"TANGPT", ctx)?;
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

        //
        // FIRST will be set to .FALSE. later, after all first-pass
        // actions have been performed.
        //
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
    // The method cannot be anything other than 'ELLIPSOID'.
    //
    if !EQSTR(METHOD, b"ELLIPSOID") {
        SETMSG(
            b"Method is currently restricted to ELLIPSOID, but input value was #.",
            ctx,
        );
        ERRCH(b"#", METHOD, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"TANGPT", ctx)?;
        return Ok(());
    }

    //
    // Code from this point onward assumes the target shape is modeled
    // as a triaxial ellipsoid.
    //
    // If we're using aberration corrections, the aberration correction
    // locus must be equivalent to one of 'TANGENT POINT' or 'SURFACE
    // POINT'. TANLOC is set to .TRUE. if and only if the locus is the
    // tangent point.
    //
    if (save.FIRST || fstr::ne(CORLOC, &save.PRVLOC)) {
        //
        // Left justify the input locus string, convert to upper case,
        // and compress all embedded blanks for comparison.
        //
        LJUCRS(0, CORLOC, &mut LOCSTR, ctx);

        if fstr::eq(&LOCSTR, b"TANGENTPOINT") {
            save.TANLOC = true;
        } else if fstr::eq(&LOCSTR, b"SURFACEPOINT") {
            save.TANLOC = false;
        } else {
            SETMSG(b"Aberration correction locus must be one of TANGENT POINT or SURFACE POINT but was #.", ctx);
            ERRCH(b"#", CORLOC, ctx);
            SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
            CHKOUT(b"TANGPT", ctx)?;
            return Ok(());
        }

        //
        // At this point we have a valid locus. TANLOC is set.
        // Save the input locus string so we can check for
        // a change on the next call.
        //
        fstr::assign(&mut save.PRVLOC, CORLOC);
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
        CHKOUT(b"TANGPT", ctx)?;
        return Ok(());
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

    if FAILED(ctx) {
        CHKOUT(b"TANGPT", ctx)?;
        return Ok(());
    }

    if !FND {
        SETMSG(b"The target, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE Toolkit, or that you failed to load a kernel containing a name-ID mapping for this body.", ctx);
        ERRCH(b"#", TARGET, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(b"TANGPT", ctx)?;
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

    if FAILED(ctx) {
        CHKOUT(b"TANGPT", ctx)?;
        return Ok(());
    }

    if !FND {
        SETMSG(b"The observer, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE Toolkit, or that you failed to load a kernel containing a name-ID mapping for this body.", ctx);
        ERRCH(b"#", OBSRVR, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(b"TANGPT", ctx)?;
        return Ok(());
    }

    //
    // Check the input body codes. If they are equal, signal
    // an error.
    //
    if (OBSCDE == TRGCDE) {
        SETMSG(
            b"The observing body and target body are the same. Both are #.",
            ctx,
        );
        ERRCH(b"#", OBSRVR, ctx);
        SIGERR(b"SPICE(BODIESNOTDISTINCT)", ctx)?;
        CHKOUT(b"TANGPT", ctx)?;
        return Ok(());
    }

    //
    // Get the target body's ellipsoid radii.
    //
    if (save.FIRST || (TRGCDE != save.PRVTCD)) {
        //
        // This the first pass, or else the target body has changed. We
        // need to get radii for the new target body.
        //
        // Re-initialize the counter used to detect changes to the target
        // body radii.
        //
        ZZCTRUIN(save.SVCTR5.as_slice_mut(), ctx);

        save.PRVTCD = TRGCDE;
    }

    ZZBODVCD(
        TRGCDE,
        b"RADII",
        3,
        save.SVCTR5.as_slice_mut(),
        &mut save.SVNRAD,
        save.SVRADI.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"TANGPT", ctx)?;
        return Ok(());
    }

    if (save.SVNRAD != 3) {
        SETMSG(
            b"Number of radii associated with target body # is #; number must be 3.",
            ctx,
        );
        ERRCH(b"#", TARGET, ctx);
        ERRINT(b"#", save.SVNRAD, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"TANGPT", ctx)?;
        return Ok(());
    }

    //
    // At this point, we've performed all first-pass actions.
    //
    save.FIRST = false;

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
        CHKOUT(b"TANGPT", ctx)?;
        return Ok(());
    }

    if !FND {
        SETMSG(b"Reference frame # is not recognized by the SPICE frame subsystem. Possibly a required frame definition kernel has not been loaded.", ctx);
        ERRCH(b"#", FIXREF, ctx);
        SIGERR(b"SPICE(NOFRAME)", ctx)?;
        CHKOUT(b"TANGPT", ctx)?;
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
        CHKOUT(b"TANGPT", ctx)?;
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
        CHKOUT(b"TANGPT", ctx)?;
        return Ok(());
    }

    if !FND {
        SETMSG(b"Reference frame # is not recognized by the SPICE frame subsystem. Possibly a required frame definition kernel has not been loaded.", ctx);
        ERRCH(b"#", DREF, ctx);
        SIGERR(b"SPICE(NOFRAME)", ctx)?;
        CHKOUT(b"TANGPT", ctx)?;
        return Ok(());
    }

    //
    // Get the position of the target relative to the observer. If
    // light time corrections are used, this gives us an initial
    // light time estimate and initial target epoch.
    //
    SPKPOS(
        TARGET,
        ET,
        FIXREF,
        ABCORR,
        OBSRVR,
        TPOS.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"TANGPT", ctx)?;
        return Ok(());
    }

    if VZERO(TPOS.as_slice()) {
        SETMSG(b"Observer # and target # have distinct ID codes but the distance between these objects is zero.", ctx);
        ERRCH(b"#", OBSRVR, ctx);
        ERRCH(b"#", TARGET, ctx);
        SIGERR(b"SPICE(NOSEPARATION)", ctx)?;
        CHKOUT(b"TANGPT", ctx)?;
        return Ok(());
    }

    //
    // Negate the target's position to obtain the position of the
    // observer relative to the target.
    //
    VMINUS(TPOS.as_slice(), FIXOBS.as_slice_mut());

    //
    // Now we can check whether the observer is inside the ellipsoid.
    // Find the point on the ellipsoid that lies on the line between
    // FIXOBS and the ellipsoid's center.
    //
    EDPNT(
        FIXOBS.as_slice(),
        save.SVRADI[1],
        save.SVRADI[2],
        save.SVRADI[3],
        P.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"TANGPT", ctx)?;
        return Ok(());
    }

    if (VNORM(P.as_slice()) >= VNORM(FIXOBS.as_slice())) {
        SETMSG(
            b"Observer # is inside ellipsoid representing target body # shape.",
            ctx,
        );
        ERRCH(b"#", OBSRVR, ctx);
        ERRCH(b"#", TARGET, ctx);
        SIGERR(b"SPICE(INVALIDGEOMETRY)", ctx)?;
        CHKOUT(b"TANGPT", ctx)?;
        return Ok(());
    }

    //
    // The target epoch is dependent on the aberration correction. The
    // coefficient S has been set to give us the correct answer for each
    // case.
    //
    *TRGEPC = (ET + (S * LT));

    //
    // Transform the direction vector from frame DREF to the body-fixed
    // frame associated with the target. The epoch TRGEPC associated
    // with the body-fixed frame has been set already.
    //
    // We'll compute the transformation in two parts: first
    // from frame DREF to J2000, then from J2000 to the target
    // frame.
    //
    // The orientation of the ray's frame is evaluated at ET in any
    // of the following situations:
    //
    //    - The frame is inertial
    //    - Light time corrections are not used
    //    - The frame is centered at the observer
    //
    // Let REFEPC be the epoch of participation of the observer.
    //
    if (((DCLASS == INERTL) || (DCENTR == OBSCDE)) || !save.USELT) {
        REFEPC = ET;
    } else {
        //
        // The epoch at which the orientation of ray frame is evaluated
        // is the epoch of participation of the center of that frame.
        //
        // Find the light time from the observer to the center of
        // frame DREF.
        //
        SPKEZP(
            DCENTR,
            ET,
            b"J2000",
            ABCORR,
            OBSCDE,
            CTRPOS.as_slice_mut(),
            &mut LTCENT,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"TANGPT", ctx)?;
            return Ok(());
        }

        REFEPC = (ET + (S * LTCENT));
    }

    //
    // The epoch REFEPC associated with frame DREF has been set.
    //
    // Compute the ray direction in the J2000 frame.
    //
    if (DFRCDE == J2CODE) {
        VEQU(DVEC.as_slice(), J2DIR.as_slice_mut());
    } else {
        REFCHG(DFRCDE, J2CODE, REFEPC, R2JMAT.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"TANGPT", ctx)?;
            return Ok(());
        }

        MXV(R2JMAT.as_slice(), DVEC.as_slice(), J2DIR.as_slice_mut());
    }

    //
    // Now transform the ray direction from the J2000 frame to the
    // target body-fixed frame at TRGEPC.
    //
    REFCHG(J2CODE, FXFCDE, *TRGEPC, J2FIXM.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"TANGPT", ctx)?;
        return Ok(());
    }

    MXV(J2FIXM.as_slice(), J2DIR.as_slice(), FIXDIR.as_slice_mut());

    //
    // We have all the inputs needed to make initial estimates of
    // our outputs.
    //
    NPEDLN(
        save.SVRADI[1],
        save.SVRADI[2],
        save.SVRADI[3],
        FIXOBS.as_slice(),
        FIXDIR.as_slice(),
        SRFPT.as_slice_mut(),
        ALT,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"TANGPT", ctx)?;
        return Ok(());
    }

    //
    // Compute the observer-to-surface point vector for the initial
    // estimated solution.
    //
    VSUB(SRFPT.as_slice(), FIXOBS.as_slice(), SRFVEC.as_slice_mut());

    //
    // Now compute the tangent point.
    //
    // Start by finding the nearest point to SRFPT on the line
    // containing the input ray. Note that although we're setting the
    // value of the outputs TANPT here, we're not done yet.
    //
    // We retain the altitude found by NPEDLN, since the following
    // call can introduce round-off error in the intercept case.
    //
    NPLNPT(
        FIXOBS.as_slice(),
        FIXDIR.as_slice(),
        SRFPT.as_slice(),
        TANPT.as_slice_mut(),
        &mut DVAL,
        ctx,
    )?;

    //
    // Note that TANPT might not be valid here, if we're in the look-
    // away case. We'll deal with this case below.
    //
    // A SPICE error should not be possible here but we check anyway
    // for safety.
    //
    if FAILED(ctx) {
        CHKOUT(b"TANGPT", ctx)?;
        return Ok(());
    }

    if !save.USELT {
        //
        // Aberration corrections are not used.
        //
        *TRGEPC = ET;

        //
        // If TANPT is on or behind the ray's vertex, reset TANPT to be
        // the vertex, and set the range to zero. Reset the surface point
        // SRFPT to the nearest point on the target to the observer, and
        // set ALT to the altitude of the observer with respect to that
        // point.
        //
        // Note that if aberration corrections were used, then the test
        // for the ray pointing away from the target could give a
        // different result. We handle that case separately later on.
        //
        VSUB(TANPT.as_slice(), FIXOBS.as_slice(), TANOFF.as_slice_mut());

        if (VDOT(TANOFF.as_slice(), FIXDIR.as_slice()) <= 0.0) {
            //
            // TANPT is on or behind the ray's vertex.
            //
            VEQU(FIXOBS.as_slice(), TANPT.as_slice_mut());
            *RANGE = 0.0;

            NEARPT(
                FIXOBS.as_slice(),
                save.SVRADI[1],
                save.SVRADI[2],
                save.SVRADI[3],
                SRFPT.as_slice_mut(),
                ALT,
                ctx,
            )?;
            if FAILED(ctx) {
                CHKOUT(b"TANGPT", ctx)?;
                return Ok(());
            }

            //
            // Compute SRFVEC using our newly computed value of SRFPT.
            //
            VSUB(SRFPT.as_slice(), FIXOBS.as_slice(), SRFVEC.as_slice_mut());
        } else {
            //
            // The tangent point lies ahead of the observer.
            //
            if (*ALT == 0.0) {
                //
                // This is the geometric intercept case. ALT, SRFPT, TANPT,
                // and SRFVEC are already set. To eliminate any error in
                // TANPT, we set it equal to SRFPT.
                //
                VEQU(SRFPT.as_slice(), TANPT.as_slice_mut());

                *RANGE = VNORM(SRFVEC.as_slice());
            } else {
                //
                // This is the normal geometric case. All outputs
                // other than range are already set.
                //
                *RANGE = VDIST(FIXOBS.as_slice(), TANPT.as_slice());
            }
        }

        CHKOUT(b"TANGPT", ctx)?;
        return Ok(());
    }

    //
    // Still here? Then we are using aberration corrections. The outputs
    // we've computed serve as first estimates for converged values.
    //
    // Since we're using light time corrections, we're going to make an
    // estimate of light time to the aberration correction locus, then
    // re-do our computation of the target position and orientation
    // using the new light time value.
    //
    // Note that for non-converged light time, we perform several more
    // iterations. The initial light time correction was for the target
    // center.
    //
    if save.USECN {
        NITR = MXCVIT;
    } else {
        NITR = MXNCIT;
    }
    //
    // Compute new light time estimate and new target epoch.
    //
    if save.TANLOC {
        //
        // Compute distance to the tangent point.
        //
        DIST = VDIST(FIXOBS.as_slice(), TANPT.as_slice());
    } else {
        //
        // Compute distance to the surface point.
        //
        DIST = VNORM(SRFVEC.as_slice());
    }

    //
    // We'll need the state of the observer relative to the solar system
    // barycenter. This state need be computed just once. The position
    // of the target relative to the solar system barycenter will need
    // to be re-computed on each loop iteration.
    //
    SPKSSB(OBSCDE, ET, b"J2000", SSBOST.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"TANGPT", ctx)?;
        return Ok(());
    }

    //
    // Compute light time based on distance to the aberration correction
    // locus; compute the new target epoch based on the light time.
    //
    LT = (DIST / CLIGHT());
    *TRGEPC = (ET + (S * LT));

    PREVLT = 0.0;
    PRVEPC = *TRGEPC;

    I = 0;
    LTDIFF = 1.0;
    EPCDIF = 1.0;

    //
    // Initialize STLCNV, the flag that indicates whether stellar
    // aberration corrections have converged.
    //
    if save.USESTL {
        STLCNV = false;
    } else {
        STLCNV = true;
    }

    //
    // Initialize LTCNV, the flag that indicates whether light time
    // corrections have converged.
    //
    LTCNV = false;

    //
    // The loop terminates if both light time and stellar aberration
    // correction have converged or if the maximum number of iterations
    // has been reached.
    //
    // Light time correction convergence is indicated when either of
    // these conditions are met:
    //
    //     - The relative difference between successive light time
    //       estimates becomes less than CNVLIM
    //
    //     - The target epoch doesn't change
    //
    // Stellar aberration convergence is indicated when both of
    // these conditions are met:
    //
    //    - The relative difference between successive values of TANPT
    //      becomes less than CNVLIM
    //
    //    - The relative difference between successive values of SRFPT
    //      becomes less than CNVLIM
    //
    //
    while ((I < NITR) && !(LTCNV && STLCNV)) {
        if save.USESTL {
            //
            // Track the output points in order to test convergence of
            // the stellar aberration correction.
            //
            VEQU(TANPT.as_slice(), PRVTAN.as_slice_mut());
            VEQU(SRFPT.as_slice(), PRVSRF.as_slice_mut());
        }

        //
        // Get the J2000-relative state of the target relative to
        // the solar system barycenter at the target epoch. The
        // observer's position doesn't change.
        //
        SPKSSB(TRGCDE, *TRGEPC, b"J2000", SSBTST.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"TANGPT", ctx)?;
            return Ok(());
        }

        //
        // Convert the position of the observer relative to the solar
        // system barycenter from the J2000 frame to the target frame at
        // TRGEPC.
        //
        // SSBOST contains the J2000-relative state of the observer
        // relative to the solar system barycenter at ET.
        //
        VSUB(SSBOST.as_slice(), SSBTST.as_slice(), J2OPOS.as_slice_mut());
        PXFORM(b"J2000", FIXREF, *TRGEPC, J2FIXM.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"TANGPT", ctx)?;
            return Ok(());
        }

        //
        // If we're using stellar aberration corrections, which we
        // normally should do if we're using light time corrections,
        // compute the stellar aberration correction for the light time
        // corrected aberration correction locus.
        //
        if save.USESTL {
            //
            // Get the position of the aberration correction locus
            // relative to the observer in the J2000 frame. The locus is
            // expressed as an offset from the target center.
            //
            // First convert the locus from the target body-fixed frame to
            // the J2000 frame.
            //
            if save.TANLOC {
                MTXV(J2FIXM.as_slice(), TANPT.as_slice(), J2LCUS.as_slice_mut());
            } else {
                MTXV(J2FIXM.as_slice(), SRFPT.as_slice(), J2LCUS.as_slice_mut());
            }

            //
            // Compute the position of the locus relative to the observer
            // in the J2000 frame.
            //
            VMINUS(J2OPOS.as_slice(), J2TPOS.as_slice_mut());
            VADD(J2TPOS.as_slice(), J2LCUS.as_slice(), J2LPOS.as_slice_mut());

            //
            // Correct the vector from the observer to the aberration
            // correction locus for stellar aberration and retain the
            // offset STLOFF from the uncorrected vector to the corrected
            // vector.
            //
            if save.XMIT {
                STLABX(
                    J2LPOS.as_slice(),
                    SSBOST.subarray(4),
                    STLLOC.as_slice_mut(),
                    ctx,
                )?;
            } else {
                STELAB(
                    J2LPOS.as_slice(),
                    SSBOST.subarray(4),
                    STLLOC.as_slice_mut(),
                    ctx,
                )?;
            }

            VSUB(STLLOC.as_slice(), J2LPOS.as_slice(), STLOFF.as_slice_mut());

            //
            // Convert the stellar aberration correction offset to the
            // target body-fixed frame at TRGEPC.
            //
            MXV(J2FIXM.as_slice(), STLOFF.as_slice(), STLFIX.as_slice_mut());
        } else {
            //
            // We're not using stellar aberration correction, so just
            // zero out the offset.
            //
            CLEARD(3, STLFIX.as_slice_mut());
        }

        //
        // Convert the observer's position relative to the target from
        // the J2000 frame to the target frame at the target epoch. Let
        // TRGPOS be the negative of this vector.
        //
        MXV(J2FIXM.as_slice(), J2OPOS.as_slice(), FIXOBS.as_slice_mut());
        VMINUS(FIXOBS.as_slice(), TRGPOS.as_slice_mut());

        //
        // Convert the ray direction vector from the J2000 frame
        // to the target frame at the target epoch.
        //
        MXV(J2FIXM.as_slice(), J2DIR.as_slice(), FIXDIR.as_slice_mut());

        //
        // The ray-ellipsoid near point computation must be performed
        // using the apparent target. We've accounted for light time,
        // but stellar aberration must be accounted for as well. The
        // apparent target is shifted due to stellar aberration by the
        // body-fixed vector STLFIX. Equivalently, we can shift the
        // observer position by -STLFIX.
        //
        // If stellar aberration correction was not commanded, then
        // STLFIX is the zero vector.
        //
        VSUB(FIXOBS.as_slice(), STLFIX.as_slice(), STLOBS.as_slice_mut());

        //
        // Re-compute the surface point and ray altitude.
        //
        NPEDLN(
            save.SVRADI[1],
            save.SVRADI[2],
            save.SVRADI[3],
            STLOBS.as_slice(),
            FIXDIR.as_slice(),
            SRFPT.as_slice_mut(),
            ALT,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"TANGPT", ctx)?;
            return Ok(());
        }

        //
        // Now compute the tangent point.
        //
        // Start by finding the nearest point to SRFPT on the line
        // containing the input ray.
        //
        // We retain the altitude found by NPEDLN, since the following
        // call can introduce round-off error in the intercept case.
        //
        if (*ALT != 0.0) {
            NPLNPT(
                STLOBS.as_slice(),
                FIXDIR.as_slice(),
                SRFPT.as_slice(),
                TANPT.as_slice_mut(),
                &mut DVAL,
                ctx,
            )?;
        } else {
            VEQU(SRFPT.as_slice(), TANPT.as_slice_mut());
        }

        //
        // The output SRFVEC extends from the observer to the apparent
        // position of the surface point.
        //
        VSUB(SRFPT.as_slice(), STLOBS.as_slice(), SRFVEC.as_slice_mut());

        //
        // We may need to update TANPT, SRFPT and SRFVEC if the tangent
        // point is behind the observer (the look-away case).
        //
        VSUB(TANPT.as_slice(), STLOBS.as_slice(), TANOFF.as_slice_mut());

        if (VDOT(TANOFF.as_slice(), FIXDIR.as_slice()) <= 0.0) {
            //
            // TANPT is on or behind the ray's vertex. Reset TANPT to be
            // the vertex.
            //
            VEQU(STLOBS.as_slice(), TANPT.as_slice_mut());
            *RANGE = 0.0;

            //
            // In this case, the surface point is considered to be the
            // nearest point on the target to the observer. The altitude
            // of the observer above this point is the tangent point's
            // altitude.
            //
            NEARPT(
                STLOBS.as_slice(),
                save.SVRADI[1],
                save.SVRADI[2],
                save.SVRADI[3],
                SRFPT.as_slice_mut(),
                ALT,
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"TANGPT", ctx)?;
                return Ok(());
            }

            VSUB(SRFPT.as_slice(), TANPT.as_slice(), SRFVEC.as_slice_mut());

            //
            // Set the light time and the target epoch, based on the
            // locus.
            //
            if save.TANLOC {
                LT = 0.0;
                *TRGEPC = ET;
            } else {
                LT = (*ALT / CLIGHT());
                *TRGEPC = (ET + (S * LT));
            }
        } else {
            //
            // This is the normal case.
            //
            // Compute a new light time estimate and new target epoch.
            // Light time estimates are computed using the light-time
            // corrected position of the aberration correction locus;
            // stellar aberration does not apply. Therefore we use FIXOBS
            // as the observer position for the distance computations.
            //
            if save.TANLOC {
                //
                // Compute distance to the tangent point.
                //
                DIST = VDIST(FIXOBS.as_slice(), TANPT.as_slice());
            } else {
                //
                // Compute distance to the surface point.
                //
                DIST = VDIST(FIXOBS.as_slice(), SRFPT.as_slice());
            }
            //
            // Compute a new light time estimate and a new target epoch.
            //
            LT = (DIST / CLIGHT());
            *TRGEPC = (ET + (S * LT));
        }

        //
        // Compute the changes in the light time and target epoch for
        // this loop pass. Determine whether light time and stellar
        // aberration have converged.
        //
        LT = TOUCHD(LT);
        LTDIFF = f64::abs((LT - PREVLT));
        EPCDIF = f64::abs((*TRGEPC - PRVEPC));

        PREVLT = LT;
        PRVEPC = *TRGEPC;
        I = (I + 1);

        LTCNV = ((LTDIFF < (CNVLIM * f64::abs(LT))) || (EPCDIF == 0.0));

        if save.USESTL {
            STLCNV = ((VREL(TANPT.as_slice(), PRVTAN.as_slice()) < CNVLIM)
                && (VREL(SRFPT.as_slice(), PRVSRF.as_slice()) < CNVLIM));
        }
    }

    *RANGE = VDIST(STLOBS.as_slice(), TANPT.as_slice());

    CHKOUT(b"TANGPT", ctx)?;
    Ok(())
}
