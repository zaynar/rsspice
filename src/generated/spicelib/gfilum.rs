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
const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;
const ZZGET: i32 = -1;
const ZZPUT: i32 = -2;
const ZZRESET: i32 = -3;
const ZZNOP: i32 = 3;
const GEN: i32 = 1;
const GF_REF: i32 = 2;
const GF_TOL: i32 = 3;
const GF_DT: i32 = 4;
const NID: i32 = 4;
const LBCELL: i32 = -5;
const LNSIZE: i32 = 80;
const QNPARS: i32 = 8;
const NOBAIL: bool = false;
const NORPT: bool = false;

/// GF, illumination angle search
///
/// Determine time intervals over which a specified constraint on
/// the observed phase, solar incidence, or emission angle at
/// a specified target body surface point is met.
///
/// # Required Reading
///
/// * [GF](crate::required_reading::gf)
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
///  LBCELL     P   SPICE Cell lower bound.
///  CNVTOL     P   Convergence tolerance.
///  NWILUM     P   Number of workspace windows for angle search.
///  METHOD     I   Computation method.
///  ANGTYP     I   Type of illumination angle.
///  TARGET     I   Name of the target body.
///  ILLMN      I   Name of the illumination source.
///  FIXREF     I   Body-fixed, body-centered target body frame.
///  ABCORR     I   Aberration correction flag.
///  OBSRVR     I   Name of the observing body.
///  SPOINT     I   Body-fixed coordinates of a target surface point.
///  RELATE     I   Relational operator.
///  REFVAL     I   Reference value.
///  ADJUST     I   Adjustment value for absolute extrema searches.
///  STEP       I   Step size used for locating extrema and roots.
///  CNFINE     I   SPICE window to which the search is confined.
///  MW         I   Workspace window size.
///  NW         I   Workspace window count.
///  WORK       O   Array of workspace windows.
///  RESULT    I-O  SPICE window containing results.
/// ```
///
/// # Detailed Input
///
/// ```text
///  METHOD   is a short string providing parameters defining the
///           computation method to be used. Parameters include, but
///           are not limited to, the shape model used to represent the
///           surface of the target body.
///
///           The only choice currently supported is
///
///              'Ellipsoid'        The illumination angle
///                                 computation uses a triaxial
///                                 ellipsoid to model the surface
///                                 of the target body. The
///                                 ellipsoid's radii must be
///                                 available in the kernel pool.
///
///           Neither case nor whitespaces are significant in METHOD.
///           For example, the string ' eLLipsoid ' is valid.
///
///  ANGTYP   is a string specifying the type of illumination angle for
///           which a search is to be performed. The possible values of
///           ANGTYP are
///
///              'PHASE'
///              'INCIDENCE'
///              'EMISSION'
///
///           When the illumination source is the sun, the incidence
///           angle is commonly called the "solar incidence angle."
///
///           See the $Particulars section below for a detailed
///           description of these angles.
///
///           Neither case nor whitespaces are significant in ANGTYP.
///           For example, the string ' Incidence ' is valid.
///
///  TARGET   is the name of a target body. The point at which the
///           illumination angles are defined is located on the surface
///           of this body.
///
///           Optionally, you may supply the integer ID code for the
///           object as an integer string. For example both 'MOON' and
///           '301' are legitimate strings that indicate the moon is
///           the target body.
///
///           Neither case nor leading and trailing blanks are
///           significant in TARGET. For example, the string
///           ' Incidence ' is valid. Sequences of embedded blanks are
///           treated as a single blank.
///
///  ILLMN    is the name of the illumination source. This source may
///           be any ephemeris object. Case, blanks, and numeric values
///           are treated in the same way as for the input TARGET.
///
///  FIXREF   is the name of the body-fixed, body-centered reference
///           frame associated with the target body. The input surface
///           point SPOINT is expressed relative to this reference
///           frame, and this frame is used to define the orientation
///           of the target body as a function of time.
///
///           The string FIXREF is case-insensitive, and leading
///           and trailing blanks in FIXREF are not significant.
///
///  ABCORR   indicates the aberration corrections to be applied to the
///           observer-surface point vector, the surface point-
///           illumination source vector, and the target body
///           orientation to account for one-way light time and stellar
///           aberration.
///
///           Any "reception" correction accepted by SPKEZR can be used
///           here. See the header of SPKEZR for a detailed description
///           of the aberration correction options. For convenience,
///           the options are listed below:
///
///              'NONE'     Apply no correction.
///
///              'LT'       "Reception" case: correct for
///                         one-way light time using a Newtonian
///                         formulation.
///
///              'LT+S'     "Reception" case: correct for
///                         one-way light time and stellar
///                         aberration using a Newtonian
///                         formulation.
///
///              'CN'       "Reception" case: converged
///                         Newtonian light time correction.
///
///              'CN+S'     "Reception" case: converged
///                         Newtonian light time and stellar
///                         aberration corrections.
///
///           Case and blanks are not significant in the string ABCORR.
///
///  OBSRVR   is the name of an observing body. Case, blanks, and
///           numeric values are treated in the same way as for the
///           input TARGET.
///
///  SPOINT   is a surface point on the target body, expressed in
///           Cartesian coordinates, relative to the body-fixed target
///           frame designated by FIXREF.
///
///           SPOINT need not be visible from the observer's location
///           in order for the constraint specified by RELATE and
///           REFVAL (see descriptions below) to be satisfied.
///
///           The components of SPOINT have units of km.
///
///  RELATE   is a relational operator used to define a constraint on a
///           specified illumination angle. The result window found by
///           this routine indicates the time intervals where the
///           constraint is satisfied. Supported values of RELATE and
///           corresponding meanings are shown below:
///
///              '>'      The angle is greater than the reference
///                       value REFVAL.
///
///              '='      The angle is equal to the reference
///                       value REFVAL.
///
///              '<'      The angle is less than the reference
///                       value REFVAL.
///
///
///             'ABSMAX'  The angle is at an absolute maximum.
///
///             'ABSMIN'  The angle is at an absolute minimum.
///
///             'LOCMAX'  The angle is at a local maximum.
///
///             'LOCMIN'  The angle is at a local minimum.
///
///           The caller may indicate that the window of interest is
///           the set of time intervals where the angle is within a
///           specified separation from an absolute extremum. The
///           argument ADJUST (described below) is used to specify this
///           separation.
///
///           Local extrema are considered to exist only in the
///           interiors of the intervals comprising the confinement
///           window: a local extremum cannot exist at a boundary point
///           of the confinement window.
///
///           Case is not significant in the string RELATE.
///
///  REFVAL   is the reference value used together with the argument
///           RELATE to define an equality or inequality to be
///           satisfied by the specified illumination angle. See the
///           discussion of RELATE above for further information.
///
///           The units of REFVAL are radians.
///
///  ADJUST   is a parameter used to modify searches for absolute
///           extrema: when RELATE is set to 'ABSMAX' or 'ABSMIN' and
///           ADJUST is set to a positive value, GFILUM will find times
///           when the specified illumination angle is within ADJUST
///           radians of the specified extreme value.
///
///           If ADJUST is non-zero and a search for an absolute
///           minimum is performed, the result window contains time
///           intervals when the specified illumination angle has
///           values between the absolute minimum ABSMIN and
///           ABSMIN + ADJUST radians.
///
///           If ADJUST is non-zero and the search is for an absolute
///           maximum, the corresponding angle is between the absolute
///           maximum ABSMAX and ABSMAX - ADJUST radians.
///
///           ADJUST is not used for searches for local extrema,
///           equality or inequality conditions.
///
///  STEP     is the step size to be used in the search. STEP must be
///           short enough for a search using this step size to locate
///           the time intervals where the specified illumination angle
///           is monotone increasing or decreasing. However, STEP must
///           not be *too* short, or the search will take an
///           unreasonable amount of time.
///
///           The choice of STEP affects the completeness but not the
///           precision of solutions found by this routine; the
///           precision is controlled by the convergence tolerance. See
///           the discussion of the parameter CNVTOL for details.
///
///           STEP has units of seconds.
///
///  CNFINE   is a SPICE window that confines the time period over
///           which the specified search is conducted. CNFINE may
///           consist of a single interval or a collection of
///           intervals.
///
///           The endpoints of the time intervals comprising CNFINE are
///           interpreted as seconds past J2000 TDB.
///
///           In some cases the confinement window can be used to
///           greatly reduce the time window that must be searched for
///           the desired solution. See the $Particulars section below
///           for further discussion.
///
///           See the $Examples section below for a code example that
///           shows how to create a confinement window.
///
///           CNFINE must be initialized by the caller via the SPICELIB
///           routine SSIZED.
///
///           In some cases the observer's state may be computed at
///           times outside of CNFINE by as much as 2 seconds. See
///           $Particulars for details.
///
///  MW       is a parameter specifying the length of the workspace
///           array WORK (see description below) used by this routine.
///           MW should be at least as large as TWICE the number of
///           intervals within the search window on which the specified
///           illumination angle is monotone increasing or decreasing.
///           It does no harm to pick a value of MW larger than the
///           minimum required to execute the specified search, but if
///           MW is too small, the search will fail.
///
///  RESULT   is a double precision SPICE window which will contain
///           the search results. RESULT must be declared and
///           initialized with sufficient size to capture the full
///           set of time intervals within the search region on which
///           the specified condition is satisfied.
///
///           RESULT must be initialized by the caller via the
///           SPICELIB routine SSIZED.
///
///           If RESULT is non-empty on input, its contents will be
///           discarded before GFILUM conducts its search.
/// ```
///
/// # Detailed Output
///
/// ```text
///  WORK     is an array used to store workspace windows.
///
///           This array should be declared by the caller as shown:
///
///              INCLUDE 'gf.inc'
///                 ...
///
///              DOUBLE PRECISION    WORK ( LBCELL : MW, NWILUM )
///
///           where MW is a constant declared by the caller and NWILUM
///           is a constant defined in the SPICELIB INCLUDE file
///           gf.inc.
///
///           WORK need not be initialized by the caller.
///
///           WORK is modified by this routine. The caller should
///           re-initialize this array before attempting to use it for
///           any other purpose.
///
///  RESULT   is the SPICE window of intervals, contained within the
///           confinement window CNFINE, on which the specified
///           constraint is satisfied.
///
///           The endpoints of the time intervals comprising RESULT are
///           interpreted as seconds past J2000 TDB.
///
///           If the search is for local extrema, or for absolute
///           extrema with ADJUST set to zero, then normally each
///           interval of RESULT will be a singleton: the left and
///           right endpoints of each interval will be identical.
///
///           If no times within the confinement window satisfy the
///           search criteria, RESULT will be returned with a
///           cardinality of zero.
/// ```
///
/// # Parameters
///
/// ```text
///  LBCELL   is the lower bound for SPICE Cell arrays.
///
///  CNVTOL   is the default convergence tolerance used for finding
///           endpoints of the intervals comprising the result
///           window. CNVTOL is also used for finding intermediate
///           results; in particular, CNVTOL is used for finding the
///           windows on which the specified illumination angle is
///           increasing or decreasing. CNVTOL is used to determine
///           when binary searches for roots should terminate: when
///           a root is bracketed within an interval of length
///           CNVTOL, the root is considered to have been found.
///
///           The accuracy, as opposed to precision, of roots found
///           by this routine depends on the accuracy of the input
///           data. In most cases, the accuracy of solutions will be
///           inferior to their precision.
///
///           The calling program can reset the convergence
///           tolerance; see the $Particulars section below for
///           further information.
///
///  NWILUM   is the number of workspace windows required by
///           this routine.
///
///  See INCLUDE file gf.inc for declarations and descriptions of
///  parameters used throughout the GF subsystem.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  In order for this routine to produce correct results,
///      the step size must be appropriate for the problem at hand.
///      Step sizes that are too large may cause this routine to miss
///      roots; step sizes that are too small may cause this routine
///      to run unacceptably slowly and in some cases, find spurious
///      roots.
///
///      This routine does not diagnose invalid step sizes, except
///      that if the step size is non-positive, the error
///      SPICE(INVALIDSTEP) is signaled.
///
///  2)  Due to numerical errors, in particular,
///
///         - Truncation error in time values
///         - Finite tolerance value
///         - Errors in computed geometric quantities
///
///      it is *normal* for the condition of interest to not always be
///      satisfied near the endpoints of the intervals comprising the
///      result window.
///
///      The result window may need to be contracted slightly by the
///      caller to achieve desired results. The SPICE window routine
///      WNCOND can be used to contract the result window.
///
///  3)  If the window size MW is less than 2, the error
///      SPICE(INVALIDDIMENSION) is signaled.
///
///  4)  If the window count NW is less than NWILUM, the error
///      SPICE(INVALIDDIMENSION) is signaled.
///
///  5)  If an error (typically cell overflow) occurs while performing
///      window arithmetic, the error is signaled by a routine
///      in the call tree of this routine.
///
///  6)  If the output SPICE window RESULT has size less than 2, the
///      error SPICE(INVALIDDIMENSION) is signaled.
///
///  7)  If the output SPICE window RESULT has insufficient capacity to
///      hold the set of intervals on which the specified illumination
///      angle condition is met, an error is signaled by a routine in
///      the call tree of this routine.
///
///  8)  If the input target body-fixed frame FIXREF is not
///      recognized, an error is signaled by a routine in the call
///      tree of this routine. A frame name may fail to be recognized
///      because a required frame specification kernel has not been
///      loaded; another cause is a misspelling of the frame name.
///
///  9)  If the input frame FIXREF is not centered at the target body,
///      an error is signaled by a routine in the call tree of this
///      routine.
///
///  10) If the input argument METHOD is not recognized, an error is
///      signaled by a routine in the call tree of this routine.
///
///  11) If the illumination angle type ANGTYP is not recognized,
///      an error is signaled by a routine in the call tree
///      of this routine.
///
///  12) If the relational operator RELATE is not recognized, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  13) If the aberration correction specifier contains an
///      unrecognized value, an error is signaled by a routine in the
///      call tree of this routine.
///
///  14) If ADJUST is negative, an error is signaled by a routine in
///      the call tree of this routine.
///
///  15) If any of the input body names do not map to NAIF ID
///      codes, an error is signaled by a routine in the call tree of
///      this routine.
///
///  16) If the target coincides with the observer or the illumination
///      source, an error is signaled by a routine in the call tree
///      of this routine.
///
///  17) If required ephemerides or other kernel data are not
///      available, an error is signaled by a routine in the call tree
///      of this routine.
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
///  -  PCK data: if the target body shape is modeled as an
///     ellipsoid (currently no other shapes are supported),
///     triaxial radii for the target body must be loaded
///     into the kernel pool. Typically this is done by loading a
///     text PCK file via FURNSH.
///
///  -  Further PCK data: rotation data for the target body must be
///     loaded. These may be provided in a text or binary PCK file.
///
///  -  Frame data: if a frame definition not built into SPICE
///     is required to convert the observer and target states to the
///     body-fixed frame of the target, that definition must be
///     available in the kernel pool. Typically the definition is
///     supplied by loading a frame kernel via FURNSH.
///
///  -  In some cases the observer's state may be computed at times
///     outside of CNFINE by as much as 2 seconds; data required to
///     compute this state must be provided by loaded kernels. See
///     $Particulars for details.
///
///  In all cases, kernel data are normally loaded once per program
///  run, NOT every time this routine is called.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine determines a set of one or more time intervals
///  within the confinement window when the specified illumination
///  angle satisfies a caller-specified constraint. The resulting set
///  of intervals is returned as a SPICE window.
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
///                              illumination source. When the sun is
///                              the illumination source, this angle is
///                              commonly called the "solar incidence
///                              angle."
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
///        sensing measurement is made, and let ET - LT (LT stands
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
///        observer-to-surface point direction vector, which we'll
///        call SRFVEC, is adjusted so as to point to the apparent
///        position of SPOINT: considering SPOINT to be an ephemeris
///        object, SRFVEC points from the observer's position at ET to
///        the light time and stellar aberration
///        corrected position of SPOINT.
///
///        Target body-illumination source vector
///        --------------------------------------
///
///        The target body-illumination source vector is the apparent
///        position of the illumination source, corrected for light
///        time and stellar aberration, as seen from the surface point
///        SPOINT at time ET-LT.
///
///
///  Below we discuss in greater detail aspects of this routine's
///  solution process that are relevant to correct and efficient
///  use of this routine in user applications.
///
///
///  The Search Process
///  ==================
///
///  Regardless of the type of constraint selected by the caller, this
///  routine starts the search for solutions by determining the time
///  periods, within the confinement window, over which the specified
///  illumination angle is monotone increasing and monotone decreasing.
///  Each of these time periods is represented by a SPICE window.
///  Having found these windows, all of the illumination angle's local
///  extrema within the confinement window are known. Absolute extrema
///  then can be found very easily.
///
///  Within any interval of these "monotone" windows, there will be at
///  most one solution of any equality constraint. Since the boundary
///  of the solution set for any inequality constraint is contained in
///  the union of
///
///  -  the set of points where an equality constraint is met
///
///  -  the boundary points of the confinement window
///
///  the solutions of both equality and inequality constraints can be
///  found easily once the monotone windows have been found.
///
///
///  Step Size
///  =========
///
///  The monotone windows (described above) are found via a two-step
///  search process. Each interval of the confinement window is
///  searched as follows: first, the input step size is used to
///  determine the time separation at which the sign of the rate of
///  change of the illumination angle will be sampled. Starting at the
///  left endpoint of an interval, samples will be taken at each step.
///  If a change of sign is found, a root has been bracketed; at that
///  point, the time at which the rate of change of the selected
///  illumination angle is zero can be found by a refinement process,
///  for example, via binary search.
///
///  Note that the optimal choice of step size depends on the lengths
///  of the intervals over which the illumination angle is monotone:
///  the step size should be shorter than the shortest of these
///  intervals (within the confinement window).
///
///  The optimal step size is *not* necessarily related to the lengths
///  of the intervals comprising the result window. For example, if
///  the shortest monotone interval has length 10 days, and if the
///  shortest result window interval has length 5 minutes, a step size
///  of 9.9 days is still adequate to find all of the intervals in the
///  result window. In situations like this, the technique of using
///  monotone windows yields a dramatic efficiency improvement over a
///  state-based search that simply tests at each step whether the
///  specified constraint is satisfied. The latter type of search can
///  miss solution intervals if the step size is longer than the
///  shortest solution interval.
///
///  Having some knowledge of the relative geometry of the target,
///  observer, and illumination source can be a valuable aid in
///  picking a reasonable step size. In general, the user can
///  compensate for lack of such knowledge by picking a very short
///  step size; the cost is increased computation time.
///
///  Note that the step size is not related to the precision with which
///  the endpoints of the intervals of the result window are computed.
///  That precision level is controlled by the convergence tolerance.
///
///
///  Convergence Tolerance
///  =====================
///
///  As described above, the root-finding process used by this routine
///  involves first bracketing roots and then using a search process
///  to locate them. "Roots" are both times when local extrema are
///  attained and times when the illumination angle is equal to a
///  reference value. All endpoints of the intervals comprising the
///  result window are either endpoints of intervals of the
///  confinement window or roots.
///
///  Once a root has been bracketed, a refinement process is used to
///  narrow down the time interval within which the root must lie.
///  This refinement process terminates when the location of the root
///  has been determined to within an error margin called the
///  "convergence tolerance." The convergence tolerance used by this
///  routine is set via the parameter CNVTOL.
///
///  The value of CNVTOL is set to a "tight" value so that the
///  tolerance doesn't become the limiting factor in the accuracy of
///  solutions found by this routine. In general the accuracy of input
///  data will be the limiting factor.
///
///  The user may change the convergence tolerance from the default
///  CNVTOL value by calling the routine GFSTOL, e.g.
///
///     CALL GFSTOL( tolerance value in seconds )
///
///  Call GFSTOL prior to calling this routine. All subsequent
///  searches will use the updated tolerance value.
///
///  Searches over time windows of long duration may require use of
///  larger tolerance values than the default: the tolerance must be
///  large enough so that it, when added to or subtracted from the
///  confinement window's lower and upper bounds, yields distinct time
///  values.
///
///  Setting the tolerance tighter than CNVTOL is unlikely to be
///  useful, since the results are unlikely to be more accurate.
///  Making the tolerance looser will speed up searches somewhat,
///  since a few convergence steps will be omitted.
///
///
///  The Confinement Window
///  ======================
///
///  The simplest use of the confinement window is to specify a time
///  interval within which a solution is sought. However, the
///  confinement window can, in some cases, be used to make searches
///  more efficient. Sometimes it's possible to do an efficient search
///  to reduce the size of the time period over which a relatively
///  slow search of interest must be performed.
///
///  Certain types of searches require the state of the observer,
///  relative to the solar system barycenter, to be computed at times
///  slightly outside the confinement window CNFINE. The time window
///  that is actually used is the result of "expanding" CNFINE by a
///  specified amount "T": each time interval of CNFINE is expanded by
///  shifting the interval's left endpoint to the left and the right
///  endpoint to the right by T seconds. Any overlapping intervals are
///  merged. (The input argument CNFINE is not modified.)
///
///  The window expansions listed below are additive: if both
///  conditions apply, the window expansion amount is the sum of the
///  individual amounts.
///
///  -  If a search uses an equality constraint, the time window
///     over which the state of the observer is computed is expanded
///     by 1 second at both ends of all of the time intervals
///     comprising the window over which the search is conducted.
///
///  -  If a search uses stellar aberration corrections, the time
///     window over which the state of the observer is computed is
///     expanded as described above.
///
///  When light time corrections are used, expansion of the search
///  window also affects the set of times at which the light time-
///  corrected state of the target is computed.
///
///  In addition to the possible 2 second expansion of the search
///  window that occurs when both an equality constraint and stellar
///  aberration corrections are used, round-off error should be taken
///  into account when the need for data availability is analyzed.
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
///  1) Determine time intervals over which the MER-1 ("Opportunity")
///     rover's location satisfies certain constraints on its
///     illumination and visibility as seen from the Mars
///     Reconnaissance Orbiter (MRO) spacecraft.
///
///     In this case we require the emission angle to be less than
///     20 degrees and the solar incidence angle to be less than
///     60 degrees.
///
///     The reader can verify that the observation start times of the
///     MRO HIRISE images
///
///        Product ID              Image start time
///        ----------              ----------------
///        TRA_000873_1780_RED     2006-10-03T12:44:13.425
///        PSP_001414_1780_RED     2006-11-14T15:39:55.373
///        PSP_001612_1780_RED     2006-11-30T01:38:34.390
///
///     are contained within the result window found by the
///     example program shown below.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File: gfilum_ex1.tm
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
///           pck00010.tpc                  Planet orientation
///                                         and radii
///           naif0010.tls                  Leapseconds
///           mro_psp1.bsp                  MRO ephemeris
///           mer1_surf_rover_ext10_v1.bsp  MER-1 ephemeris
///           mer1_surf_rover_ext11_v1.bsp  MER-1 ephemeris
///           mer1_ls_040128_iau2000_v1.bsp MER-1 landing site
///                                         ephemeris
///           mer1_v10.tf                   MER-1 frame kernel
///
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de421.bsp',
///                               'pck00010.tpc',
///                               'naif0010.tls',
///                               'mro_psp1.bsp',
///                               'mer1_surf_rover_ext10_v1.bsp',
///                               'mer1_surf_rover_ext11_v1.bsp',
///                               'mer1_ls_040128_iau2000_v1.bsp',
///                               'mro_psp1.bsp',
///                               'mer1_v10.tf'                    )
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM GFILUM_EX1
///           IMPLICIT NONE
///
///     C
///     C     Global parameters
///     C
///           INCLUDE 'gf.inc'
///           INCLUDE 'zzabcorr.inc'
///
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      DPR
///           DOUBLE PRECISION      RPD
///
///           INTEGER               WNCARD
///
///     C
///     C     Local parameters
///     C
///     C
///     C     Output time format
///     C
///           CHARACTER*(*)         FMT
///           PARAMETER           ( FMT =
///          .             'YYYY MON DD HR:MN:SC.### UTC' )
///
///     C
///     C     Meta-kernel name
///     C
///           CHARACTER*(*)         META
///           PARAMETER           ( META   = 'gfilum_ex1.tm' )
///
///     C
///     C     SPICE cell lower bound
///     C
///           INTEGER               LBCELL
///           PARAMETER           ( LBCELL = -5 )
///
///     C
///     C     Maximum number of intervals in the windows
///     C     used in this program
///     C
///           INTEGER               MAXIVL
///           PARAMETER           ( MAXIVL = 1000 )
///
///           INTEGER               MAXWIN
///           PARAMETER           ( MAXWIN = 2 * MAXIVL )
///
///     C
///     C     Maximum length of reference frame name
///     C
///           INTEGER               FRNMLN
///           PARAMETER           ( FRNMLN = 32 )
///
///     C
///     C     Maximum length of body name
///     C
///           INTEGER               BDNMLN
///           PARAMETER           ( BDNMLN = 36 )
///
///     C
///     C     Maximum length of time string
///     C
///           INTEGER               TIMLEN
///           PARAMETER           ( TIMLEN = 40 )
///
///     C
///     C     Length of computation method string
///     C
///           INTEGER               METLEN
///           PARAMETER           ( METLEN = 80 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(CORLEN)    ABCORR
///           CHARACTER*(FRNMLN)    FIXREF
///           CHARACTER*(BDNMLN)    ILLMN
///           CHARACTER*(METLEN)    METHOD
///           CHARACTER*(BDNMLN)    OBSRVR
///           CHARACTER*(BDNMLN)    TARGET
///           CHARACTER*(TIMLEN)    TIMSTR
///           CHARACTER*(TIMLEN)    UTCBEG
///           CHARACTER*(TIMLEN)    UTCEND
///
///           DOUBLE PRECISION      ADJUST
///           DOUBLE PRECISION      CNFINE ( LBCELL : 2 )
///           DOUBLE PRECISION      EMISSN
///           DOUBLE PRECISION      ET0
///           DOUBLE PRECISION      ET1
///           DOUBLE PRECISION      FINISH
///           DOUBLE PRECISION      PHASE
///           DOUBLE PRECISION      REFVAL
///           DOUBLE PRECISION      RESULT ( LBCELL : MAXWIN )
///           DOUBLE PRECISION      ROVLT
///           DOUBLE PRECISION      ROVPOS ( 3 )
///           DOUBLE PRECISION      SOLAR
///           DOUBLE PRECISION      SRFVEC ( 3 )
///           DOUBLE PRECISION      START
///           DOUBLE PRECISION      STEP
///           DOUBLE PRECISION      TRGEPC
///           DOUBLE PRECISION      WORK   ( LBCELL : MAXWIN, NWILUM )
///           DOUBLE PRECISION      WNSOLR ( LBCELL : MAXWIN )
///
///           INTEGER               I
///
///     C
///     C     Saved variables
///     C
///     C     The confinement, workspace and result windows CNFINE,
///     C     WORK, WNSOLR and RESULT are saved because this practice
///     C     helps to prevent stack overflow.
///     C
///           SAVE                  CNFINE
///           SAVE                  RESULT
///           SAVE                  WORK
///           SAVE                  WNSOLR
///
///     C
///     C     Load kernels:
///     C
///           CALL FURNSH ( META )
///
///     C
///     C     Set window sizes:
///     C
///           CALL SSIZED ( 2,      CNFINE )
///           CALL SSIZED ( MAXWIN, RESULT )
///           CALL SSIZED ( MAXWIN, WNSOLR )
///
///     C
///     C     Set the search interval:
///     C
///           UTCBEG = '2006 OCT 02 00:00:00 UTC'
///           CALL STR2ET ( UTCBEG, ET0 )
///
///           UTCEND = '2006 NOV 30 12:00:00 UTC'
///           CALL STR2ET ( UTCEND, ET1 )
///
///           CALL WNINSD ( ET0, ET1, CNFINE )
///
///     C
///     C     Set observer, target, aberration correction, and the
///     C     Mars body-fixed, body-centered reference frame. The
///     C     lighting source is the sun.
///     C
///     C     Aberration corrections are set for remote observations.
///     C
///           ILLMN  = 'SUN'
///           OBSRVR = 'MRO'
///           TARGET = 'MARS'
///           ABCORR = 'CN+S'
///           FIXREF = 'IAU_MARS'
///
///     C
///     C     Use the rover position at the start of
///     C     the search interval as the surface point.
///     C
///           CALL SPKPOS ( 'MER-1', ET0,    FIXREF,
///          .              'NONE',  TARGET, ROVPOS, ROVLT )
///
///     C
///     C     Initialize the adjustment value for absolute
///     C     extremum searches. We're not performing
///     C     such searches in this example, but this input
///     C     to GFILUM must still be set.
///     C
///           ADJUST = 0.D0
///
///     C
///     C     The computation uses an ellipsoidal model for the
///     C     target body shape.
///     C
///           METHOD = 'Ellipsoid'
///
///     C
///     C     Set the reference value to use for the solar
///     C     incidence angle search.
///     C
///           REFVAL = 60.D0 * RPD()
///
///     C
///     C     Since the period of the solar incidence angle
///     C     is about one Martian day, we can safely use 6 hours
///     C     as the search step.
///     C
///           STEP   = 21600.D0
///
///     C
///     C     Search over the confinement window for times
///     C     when the solar incidence angle is less than
///     C     the reference value.
///     C
///           CALL GFILUM ( METHOD, 'INCIDENCE',    TARGET, ILLMN,
///          .              FIXREF, ABCORR,         OBSRVR, ROVPOS,
///          .              '<',    REFVAL, ADJUST, STEP,   CNFINE,
///          .              MAXWIN, NWILUM, WORK,   WNSOLR          )
///
///     C
///     C     With the search on the incidence angle complete, perform
///     C     a search on the emission angle.
///     C
///     C     Set the reference value for the emission angle search.
///     C
///           REFVAL = 20D0 * RPD()
///
///     C
///     C     We'll use 15 minutes as the search step. This step
///     C     is small enough to be suitable for Mars orbiters.
///     C     Units are seconds.
///     C
///           STEP   = 900.D0
///
///     C
///     C     Search over the previous result window for times when
///     C     the emission angle is less than the reference value.
///     C
///           CALL GFILUM ( METHOD, 'EMISSION', TARGET, ILLMN,
///          .              FIXREF, ABCORR,     OBSRVR, ROVPOS,
///          .              '<',    REFVAL,     ADJUST, STEP,
///          .              WNSOLR, MAXWIN,     NWILUM, WORK,
///          .              RESULT                             )
///
///     C
///     C     Display the result window. Show the solar incidence
///     C     and emission angles at the window's interval
///     C     boundaries.
///     C
///           WRITE (*,*) ' '
///
///           IF ( WNCARD( RESULT ) .EQ. 0 ) THEN
///
///              WRITE (*,*) '     Window is empty: condition '
///          .   //          'is not met.'
///
///           ELSE
///
///              WRITE (*,*) '                                   '
///          .   //          'Solar Incidence   Emission'
///              WRITE (*,*) '                                   '
///          .   //          '      (deg)         (deg)'
///              WRITE (*,*) ' '
///
///              DO I = 1, WNCARD( RESULT )
///
///                 CALL WNFETD ( RESULT, I, START, FINISH )
///
///                 CALL TIMOUT ( START,  FMT,   TIMSTR )
///     C
///     C           Compute the angles of interest at the boundary
///     C           epochs.
///     C
///                 CALL ILUMIN ( METHOD, TARGET, START,  FIXREF,
///          .                    ABCORR, OBSRVR, ROVPOS, TRGEPC,
///          .                    SRFVEC, PHASE,  SOLAR,  EMISSN )
///
///                 WRITE (*, '(A7, A28, 2F14.8)' )
///          .            'Start: ', TIMSTR, SOLAR*DPR(), EMISSN*DPR()
///
///
///                 CALL TIMOUT ( FINISH, FMT,   TIMSTR )
///
///                 CALL ILUMIN ( METHOD, TARGET, FINISH, FIXREF,
///          .                    ABCORR, OBSRVR, ROVPOS, TRGEPC,
///          .                    SRFVEC, PHASE,  SOLAR,  EMISSN )
///
///                 WRITE (*, '(A7, A28, 2F14.8)' )
///          .            'Stop:  ', TIMSTR, SOLAR*DPR(), EMISSN*DPR()
///
///                 WRITE (*,*) ' '
///
///              END DO
///
///           END IF
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///                                         Solar Incidence   Emission
///                                               (deg)         (deg)
///
///     Start: 2006 OCT 03 12:43:46.949 UTC   56.10415019   20.00000019
///     Stop:  2006 OCT 03 12:44:42.288 UTC   56.29996181   20.00000015
///
///     Start: 2006 OCT 08 16:03:33.956 UTC   56.48955485   20.00000021
///     Stop:  2006 OCT 08 16:04:29.495 UTC   56.68754510   19.99999997
///
///     Start: 2006 OCT 13 19:23:24.634 UTC   56.88741059   19.99999988
///     Stop:  2006 OCT 13 19:24:12.492 UTC   57.05931857   20.00000017
///
///     Start: 2006 OCT 18 22:43:21.631 UTC   57.30924467   20.00000012
///     Stop:  2006 OCT 18 22:43:47.966 UTC   57.40457272   20.00000004
///
///     Start: 2006 NOV 14 15:39:44.153 UTC   54.32875839   19.99999994
///     Stop:  2006 NOV 14 15:40:10.446 UTC   54.42668077   19.99999990
///
///     Start: 2006 NOV 19 18:59:10.190 UTC   54.63096111   20.00000007
///     Stop:  2006 NOV 19 18:59:54.776 UTC   54.79840753   19.99999985
///
///     Start: 2006 NOV 24 22:18:38.342 UTC   54.94960000   19.99999982
///     Stop:  2006 NOV 24 22:19:30.964 UTC   55.14883883   20.00000003
///
///     Start: 2006 NOV 30 01:38:07.309 UTC   55.28054784   19.99999983
///     Stop:  2006 NOV 30 01:39:03.296 UTC   55.49418925   19.99999999
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The kernel files to be used by this routine must be loaded
///      (normally using the SPICELIB routine FURNSH) before this
///      routine is called.
///
///  2)  This routine has the side effect of re-initializing the
///      illumination angle utility package. Callers may
///      need to re-initialize the package after calling this routine.
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
/// -    SPICELIB Version 1.1.0, 27-OCT-2021 (JDR) (NJB)
///
///         Edited the header to comply with NAIF standard.
///
///         Changed the code example for the solution to fit within the
///         $Examples section without modifications. Added SAVE statements
///         for CNFINE, WNSOLR, WORK and RESULT variables in code example.
///
///         Added initialization of QCPARS(8) to pacify Valgrind.
///
///         Updated description of WORK and RESULT arguments in $Brief_I/O,
///         $Detailed_Input and $Detailed_Output.
///
///         Updated header to describe use of expanded confinement window.
///
/// -    SPICELIB Version 1.0.0, 20-NOV-2012 (NJB) (BVS) (EDW)
/// ```
pub fn gfilum(
    ctx: &mut SpiceContext,
    method: &str,
    angtyp: &str,
    target: &str,
    illmn: &str,
    fixref: &str,
    abcorr: &str,
    obsrvr: &str,
    spoint: &[f64; 3],
    relate: &str,
    refval: f64,
    adjust: f64,
    step: f64,
    cnfine: &[f64],
    mw: i32,
    nw: i32,
    work: &mut [f64],
    result: &mut [f64],
) -> crate::Result<()> {
    GFILUM(
        method.as_bytes(),
        angtyp.as_bytes(),
        target.as_bytes(),
        illmn.as_bytes(),
        fixref.as_bytes(),
        abcorr.as_bytes(),
        obsrvr.as_bytes(),
        spoint,
        relate.as_bytes(),
        refval,
        adjust,
        step,
        cnfine,
        mw,
        nw,
        work,
        result,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure GFILUM ( GF, illumination angle search )
pub fn GFILUM(
    METHOD: &[u8],
    ANGTYP: &[u8],
    TARGET: &[u8],
    ILLMN: &[u8],
    FIXREF: &[u8],
    ABCORR: &[u8],
    OBSRVR: &[u8],
    SPOINT: &[f64],
    RELATE: &[u8],
    REFVAL: f64,
    ADJUST: f64,
    STEP: f64,
    CNFINE: &[f64],
    MW: i32,
    NW: i32,
    WORK: &mut [f64],
    RESULT: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let SPOINT = DummyArray::new(SPOINT, 1..=3);
    let CNFINE = DummyArray::new(CNFINE, LBCELL..);
    let mut WORK = DummyArrayMut2D::new(WORK, LBCELL..=MW, 1..=NW);
    let mut RESULT = DummyArrayMut::new(RESULT, LBCELL..);
    let mut QCPARS = ActualCharArray::new(LNSIZE, 1..=QNPARS);
    let mut QPNAMS = ActualCharArray::new(LNSIZE, 1..=QNPARS);
    let mut QDPARS = StackArray::<f64, 8>::new(1..=QNPARS);
    let QIPARS = StackArray::<i32, 8>::new(1..=QNPARS);
    let QLPARS = StackArray::<bool, 8>::new(1..=QNPARS);
    let mut TOL: f64 = 0.0;
    let mut OK: bool = false;

    //
    // SPICELIB functions
    //

    //
    // External functions
    //

    //
    // Interrupt indicator function:
    //

    //
    // Routines to set step size, refine transition times
    // and report work.
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Quantity definition parameter arrays:
    //

    //
    // Other local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"GFILUM", ctx)?;

    //
    // Check the result window's size.
    //
    if (SIZED(RESULT.as_slice(), ctx)? < 2) {
        SETMSG(b"Result window size must be at least 2 but was #.", ctx);
        ERRINT(b"#", SIZED(RESULT.as_slice(), ctx)?, ctx);
        SIGERR(b"SPICE(INVALIDDIMENSION)", ctx)?;
        CHKOUT(b"GFILUM", ctx)?;
        return Ok(());
    }

    //
    // Check the workspace window dimensions.
    //
    if (MW < 2) {
        SETMSG(
            b"Workspace window size was #; size must be at least 2.",
            ctx,
        );
        ERRINT(b"#", MW, ctx);
        SIGERR(b"SPICE(INVALIDDIMENSION)", ctx)?;
        CHKOUT(b"GFILUM", ctx)?;
        return Ok(());
    }

    if (NW < NWILUM) {
        SETMSG(
            b"Workspace window count was #; count must be at least #.",
            ctx,
        );
        ERRINT(b"#", NW, ctx);
        ERRINT(b"#", NWILUM, ctx);
        SIGERR(b"SPICE(INVALIDDIMENSION)", ctx)?;
        CHKOUT(b"GFILUM", ctx)?;
        return Ok(());
    }

    //
    // Set up a call to GFEVNT, which will handle the search.
    //
    fstr::assign(QPNAMS.get_mut(1), b"TARGET");
    fstr::assign(QCPARS.get_mut(1), TARGET);

    fstr::assign(QPNAMS.get_mut(2), b"ILLUM");
    fstr::assign(QCPARS.get_mut(2), ILLMN);

    fstr::assign(QPNAMS.get_mut(3), b"OBSERVER");
    fstr::assign(QCPARS.get_mut(3), OBSRVR);

    fstr::assign(QPNAMS.get_mut(4), b"ABCORR");
    fstr::assign(QCPARS.get_mut(4), ABCORR);

    fstr::assign(QPNAMS.get_mut(5), b"REFERENCE FRAME");
    fstr::assign(QCPARS.get_mut(5), FIXREF);

    fstr::assign(QPNAMS.get_mut(6), b"ANGTYP");
    fstr::assign(QCPARS.get_mut(6), ANGTYP);

    fstr::assign(QPNAMS.get_mut(7), b"METHOD");
    fstr::assign(QCPARS.get_mut(7), METHOD);

    //
    // Copy SPOINT to elements 1-3 of the QDPARS array.
    //
    fstr::assign(QPNAMS.get_mut(8), b"SPOINT");
    MOVED(SPOINT.as_slice(), 3, QDPARS.as_slice_mut());

    //
    // Initialize the corresponding character parameter, even though
    // we don't need it, since GFEVNT will left-justify it and convert
    // it to upper case. Memory usage checkers such as Valgrind dislike
    // code that operates on uninitialized variables.
    //
    fstr::assign(QCPARS.get_mut(8), b" ");

    //
    // Set the step size.
    //
    if (STEP <= 0.0) {
        SETMSG(b"Step size was #; step size must be positive.", ctx);
        ERRDP(b"#", STEP, ctx);
        SIGERR(b"SPICE(INVALIDSTEP)", ctx)?;
        CHKOUT(b"GFILUM", ctx)?;
        return Ok(());
    }

    GFSSTP(STEP, ctx)?;

    //
    // Retrieve the convergence tolerance, if set.
    //
    ZZHOLDD(ZZGET, GF_TOL, &mut OK, &mut TOL, ctx)?;

    //
    // Use the default value CNVTOL if there's no stored tolerance
    // value.
    //
    if !OK {
        TOL = CNVTOL;
    }

    //
    // Initialize the RESULT window.
    //
    SCARDD(0, RESULT.as_slice_mut(), ctx)?;

    //
    // Look for solutions.
    //
    // Progress report and bail-out options are set to .FALSE.
    //
    GFEVNT(
        GFSTEP,
        GFREFN,
        b"ILLUMINATION ANGLE",
        QNPARS,
        QPNAMS.as_arg(),
        QCPARS.as_arg(),
        QDPARS.as_slice(),
        QIPARS.as_slice(),
        QLPARS.as_slice(),
        RELATE,
        REFVAL,
        TOL,
        ADJUST,
        CNFINE.as_slice(),
        NORPT,
        GFREPI,
        GFREPU,
        GFREPF,
        MW,
        NWILUM,
        WORK.as_slice_mut(),
        NOBAIL,
        GFBAIL,
        RESULT.as_slice_mut(),
        ctx,
    )?;

    CHKOUT(b"GFILUM", ctx)?;
    Ok(())
}
