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
const BAIL: bool = false;
const RPT: bool = false;

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

/// GF, is target in FOV?
///
/// Determine time intervals when a specified ephemeris object
/// intersects the space bounded by the field-of-view (FOV) of a
/// specified instrument.
///
/// # Required Reading
///
/// * [CK](crate::required_reading::ck)
/// * [FRAMES](crate::required_reading::frames)
/// * [GF](crate::required_reading::gf)
/// * [KERNEL](crate::required_reading::kernel)
/// * [NAIF_IDS](crate::required_reading::naif_ids)
/// * [PCK](crate::required_reading::pck)
/// * [SPK](crate::required_reading::spk)
/// * [TIME](crate::required_reading::time)
/// * [WINDOWS](crate::required_reading::windows)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  MARGIN     P   Minimum complement of FOV cone angle.
///  LBCELL     P   SPICE Cell lower bound.
///  CNVTOL     P   Convergence tolerance.
///  MAXVRT     P   Maximum number of FOV boundary vertices.
///  INST       I   Name of the instrument.
///  TARGET     I   Name of the target body.
///  TSHAPE     I   Type of shape model used for target body.
///  TFRAME     I   Body-fixed, body-centered frame for target body.
///  ABCORR     I   Aberration correction flag.
///  OBSRVR     I   Name of the observing body.
///  STEP       I   Step size in seconds for finding FOV events.
///  CNFINE     I   SPICE window to which the search is restricted.
///  RESULT    I-O  SPICE window containing results.
/// ```
///
/// # Detailed Input
///
/// ```text
///  INST     indicates the name of an instrument, such as a
///           spacecraft-mounted framing camera, the field of view
///           (FOV) of which is to be used for a target intersection
///           search: times when the specified target intersects the
///           region of space corresponding to the FOV are sought.
///
///           The position of the instrument designated by INST is
///           considered to coincide with that of the ephemeris
///           object designated by the input argument OBSRVR (see
///           description below).
///
///           INST must have a corresponding NAIF ID and a frame
///           defined, as is normally done in a frame kernel. It
///           must also have an associated reference frame and a FOV
///           shape, boresight and boundary vertices (or reference
///           vector and reference angles) defined, as is usually
///           done in an instrument kernel.
///
///           See the header of the SPICELIB routine GETFOV for a
///           description of the required parameters associated with
///           an instrument.
///
///  TARGET   is the name of the target body, the appearances of
///           which in the specified instrument's field of view are
///           sought. The body must be an ephemeris object.
///
///           Optionally, you may supply the integer NAIF ID code
///           for the body as a string. For example both 'MOON' and
///           '301' are legitimate strings that designate the Moon.
///
///           Case and leading or trailing blanks are not
///           significant in the string TARGET.
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
///                                 BODYnnn_RADII
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
///           significant in the string TSHAPE.
///
///  TFRAME   is the name of the body-fixed, body-centered reference
///           frame associated with the target body. Examples of
///           such names are 'IAU_SATURN' (for Saturn) and 'ITRF93'
///           (for the Earth).
///
///           If the target body is modeled as a point, TFRAME
///           is ignored and should be left blank.
///
///           Case and leading or trailing blanks bracketing a
///           non-blank frame name are not significant in the string
///           TFRAME.
///
///  ABCORR   indicates the aberration corrections to be applied
///           when computing the target's position and orientation.
///
///           For remote sensing applications, where the apparent
///           position and orientation of the target seen by the
///           observer are desired, normally either of the
///           corrections
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
///
///              'LT'           Light time only
///
///              'LT+S'         Light time and stellar aberration.
///
///              'CN'           Converged Newtonian (CN) light time.
///
///              'CN+S'         CN light time and stellar aberration.
///
///           Supported aberration correction options for
///           transmission (the case where radiation is emitted from
///           observer at ET) are:
///
///              'XLT'          Light time only.
///
///              'XLT+S'        Light time and stellar aberration.
///
///              'XCN'          Converged Newtonian (CN) light time.
///
///              'XCN+S'        CN light time and stellar aberration.
///
///           For detailed information, see the GF Required Reading,
///           gf.req.
///
///           Case, leading and trailing blanks are not significant
///           in the string ABCORR.
///
///  OBSRVR   is the name of the body from which the target is
///           observed. The instrument designated by INST is treated
///           as if it were co-located with the observer.
///
///           Optionally, you may supply the integer NAIF ID code
///           for the body as a string.
///
///           Case and leading or trailing blanks are not
///           significant in the string OBSRVR.
///
///  STEP     is the step size to be used in the search. STEP must
///           be shorter than any interval, within the confinement
///           window, over which the specified condition is met. In
///           other words, STEP must be shorter than the shortest
///           visibility event that the user wishes to detect. STEP
///           also must be shorter than the minimum duration
///           separating any two visibility events. However, STEP
///           must not be *too* short, or the search will take an
///           unreasonable amount of time.
///
///           The choice of STEP affects the completeness but not
///           the precision of solutions found by this routine; the
///           precision is controlled by the convergence tolerance.
///           See the discussion of the parameter CNVTOL for
///           details.
///
///           STEP has units of seconds.
///
///  CNFINE   is a SPICE window that confines the time period over
///           which the specified search is conducted. CNFINE may
///           consist of a single interval or a collection of
///           intervals.
///
///           The endpoints of the time intervals comprising CNFINE
///           are interpreted as seconds past J2000 TDB.
///
///           See the $Examples section below for a code example
///           that shows how to create a confinement window.
///
///           CNFINE must be initialized by the caller via the
///           SPICELIB routine SSIZED.
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
///           discarded before GFTFOV conducts its search.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RESULT   is a SPICE window representing the set of time intervals,
///           within the confinement period, when the target body is
///           visible; that is, when the target body intersects the
///           space bounded by the specified instrument's field of
///           view.
///
///           The endpoints of the time intervals comprising RESULT are
///           interpreted as seconds past J2000 TDB.
///
///           If no times within the confinement window satisfy the
///           search criteria, RESULT will be returned with a
///           cardinality of zero.
/// ```
///
/// # Parameters
///
/// ```text
///  LBCELL   is the lower bound for SPICE cell arrays.
///
///  CNVTOL   is the convergence tolerance used for finding
///           endpoints of the intervals comprising the result
///           window. CNVTOL is used to determine when binary
///           searches for roots should terminate: when a root is
///           bracketed within an interval of length CNVTOL, the
///           root is considered to have been found.
///
///           The accuracy, as opposed to precision, of roots found
///           by this routine depends on the accuracy of the input
///           data. In most cases, the accuracy of solutions will be
///           inferior to their precision.
///
///  MAXVRT   is the maximum number of vertices that may be used
///           to define the boundary of the specified instrument's
///           field of view.
///
///  MARGIN   is a small positive number used to constrain the
///           orientation of the boundary vectors of polygonal
///           FOVs. Such FOVs must satisfy the following constraints:
///
///              1)  The boundary vectors must be contained within
///                  a right circular cone of angular radius less
///                  than than (pi/2) - MARGIN radians; in other
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
///            MARGIN is currently set to 1.D-12.
///
///
///  See INCLUDE file gf.inc for declarations and descriptions of
///  parameters used throughout the GF system.
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
///      This routine does not diagnose invalid step sizes, except that
///      if the step size is non-positive, an error is signaled by a
///      routine in the call tree of this routine.
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
///  3)  If the name of either the target or observer cannot be
///      translated to a NAIF ID code, an error is signaled by
///      a routine in the call tree of this routine.
///
///  4)  If the specified aberration correction is an unrecognized
///      value, an error is signaled by a routine
///      in the call tree of this routine.
///
///  5)  If the radii of a target body modeled as an ellipsoid cannot
///      be determined by searching the kernel pool for a kernel
///      variable having a name of the form
///
///         'BODYnnn_RADII'
///
///      where nnn represents the NAIF integer code associated with
///      the body, an error is signaled by a routine in the
///      call tree of this routine.
///
///  6)  If the target body coincides with the observer body OBSRVR, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  7)  If the body model specifier TSHAPE is invalid, an error is
///      signaled by either this routine or a routine in the call tree
///      of this routine.
///
///  8)  If a target body-fixed reference frame associated with a
///      non-point target is not recognized, an error is signaled by a
///      routine in the call tree of this routine.
///
///  9)  If a target body-fixed reference frame is not centered at the
///      corresponding target body, an error is signaled by a routine
///      in the call tree of this routine.
///
///  10) If the instrument name INST does not have corresponding NAIF
///      ID code, an error is signaled by a routine in the call
///      tree of this routine.
///
///  11) If the FOV parameters of the instrument are not present in
///      the kernel pool, an error is signaled by a routine
///      in the call tree of this routine.
///
///  12) If the FOV boundary has more than MAXVRT vertices, an error
///      is signaled by a routine in the call tree of this
///      routine.
///
///  13) If the instrument FOV is polygonal, and this routine cannot
///      find a ray R emanating from the FOV vertex such that maximum
///      angular separation of R and any FOV boundary vector is within
///      the limit (pi/2)-MARGIN radians, an error is signaled
///      by a routine in the call tree of this routine. If the FOV
///      is any other shape, the same error check will be applied with
///      the instrument boresight vector serving the role of R.
///
///  14) If the loaded kernels provide insufficient data to compute a
///      requested state vector, an error is signaled by a
///      routine in the call tree of this routine.
///
///  15) If an error occurs while reading an SPK or other kernel file,
///      the error is signaled by a routine in the call tree
///      of this routine.
///
///  16) If the output SPICE window RESULT has size less than 2, the
///      error SPICE(WINDOWTOOSMALL) is signaled.
///
///  17) If the output SPICE window RESULT has insufficient capacity
///      to contain the number of intervals on which the specified
///      visibility condition is met, an error is signaled
///      by a routine in the call tree of this routine.
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
///     describes the ephemeris of these objects for the period
///     defined by the confinement window, CNFINE must be
///     loaded. If aberration corrections are used, the states of
///     target and observer relative to the solar system barycenter
///     must be calculable from the available ephemeris data.
///     Typically ephemeris data are made available by loading one
///     or more SPK files via FURNSH.
///
///  -  Frame data: if a frame definition is required to convert
///     the observer and target states to the body-fixed frame of
///     the target, that definition must be available in the kernel
///     pool. Typically the definitions of frames not already
///     built-in to SPICE are supplied by loading a frame kernel.
///
///     Data defining the reference frame associated with the
///     instrument designated by INST must be available in the
///     kernel pool. Additionally the name INST must be associated
///     with an ID code. Normally these data are  made available by
///     loading a frame kernel via FURNSH.
///
///  -  IK data: the kernel pool must contain data such that
///     the SPICELIB routine GETFOV may be called to obtain
///     parameters for INST. Normally such data are provided by
///     an IK via FURNSH.
///
///  The following data may be required:
///
///  -  PCK data: bodies modeled as triaxial ellipsoids must have
///     orientation data provided by variables in the kernel pool.
///     Typically these data are made available by loading a text
///     PCK file via FURNSH.
///
///     Bodies modeled as triaxial ellipsoids must have semi-axis
///     lengths provided by variables in the kernel pool. Typically
///     these data are made available by loading a text PCK file via
///     FURNSH.
///
///  -  CK data: if the instrument frame is fixed to a spacecraft,
///     at least one CK file will be needed to permit transformation
///     of vectors between that frame and both J2000 and the target
///     body-fixed frame.
///
///  -  SCLK data: if a CK file is needed, an associated SCLK
///     kernel is required to enable conversion between encoded SCLK
///     (used to time-tag CK data) and barycentric dynamical time
///     (TDB).
///
///  Kernel data are normally loaded once per program run, NOT every
///  time this routine is called.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine determines a set of one or more time intervals
///  within the confinement window when any portion of a specified
///  target body appears within the field of view of a specified
///  instrument. We'll use the term "visibility event" to designate
///  such an appearance. The set of time intervals resulting from the
///  search is returned as a SPICE window.
///
///  This routine provides a simpler, but less flexible, interface
///  than does the SPICELIB routine GFFOVE for conducting searches for
///  visibility events. Applications that require support for progress
///  reporting, interrupt handling, non-default step or refinement
///  functions, or non-default convergence tolerance should call
///  GFFOVE rather than this routine.
///
///  To treat the target as a ray rather than as an ephemeris object,
///  use either the higher-level SPICELIB routine GFRFOV or GFFOVE.
///  Those routines may be used to search for times when distant
///  target objects such as stars are visible in an instrument FOV, as
///  long the direction from the observer to the target can be modeled
///  as a ray.
///
///  Below we discuss in greater detail aspects of this routine's
///  solution process that are relevant to correct and efficient use
///  of this routine in user applications.
///
///
///  The Search Process
///  ==================
///
///  The search for visibility events is treated as a search for state
///  transitions: times are sought when the state of the target body
///  changes from "not visible" to "visible" or vice versa.
///
///  Step Size
///  =========
///
///  Each interval of the confinement window is searched as follows:
///  first, the input step size is used to determine the time
///  separation at which the visibility state will be sampled.
///  Starting at the left endpoint of an interval, samples will be
///  taken at each step. If a state change is detected, a root has
///  been bracketed; at that point, the "root"--the time at which the
///  state change occurs---is found by a refinement process, for
///  example, via binary search.
///
///  Note that the optimal choice of step size depends on the lengths
///  of the intervals over which the visibility state is constant:
///  the step size should be shorter than the shortest visibility event
///  duration and the shortest period between visibility events, within
///  the confinement window.
///
///  Having some knowledge of the relative geometry of the target and
///  observer can be a valuable aid in picking a reasonable step size.
///  In general, the user can compensate for lack of such knowledge by
///  picking a very short step size; the cost is increased computation
///  time.
///
///  Note that the step size is not related to the precision with which
///  the endpoints of the intervals of the result window are computed.
///  That precision level is controlled by the convergence tolerance.
///
///
///  Convergence Tolerance
///  =====================
///
///  Once a root has been bracketed, a refinement process is used to
///  narrow down the time interval within which the root must lie.
///  This refinement process terminates when the location of the root
///  has been determined to within an error margin called the
///  "convergence tolerance." The default convergence tolerance
///  used by this routine is set by the parameter CNVTOL (defined
///  in gf.inc).
///
///  The value of CNVTOL is set to a "tight" value so that the
///  tolerance doesn't become the limiting factor in the accuracy of
///  solutions found by this routine. In general the accuracy of input
///  data will be the limiting factor.
///
///  The user may change the convergence tolerance from the default
///  CNVTOL value by calling the routine GFSTOL, e.g.
///
///     CALL GFSTOL( tolerance value )
///
///  Call GFSTOL prior to calling this routine. All subsequent
///  searches will use the updated tolerance value.
///
///  Setting the tolerance tighter than CNVTOL is unlikely to be
///  useful, since the results are unlikely to be more accurate.
///  Making the tolerance looser will speed up searches somewhat,
///  since a few convergence steps will be omitted. However, in most
///  cases, the step size is likely to have a much greater effect
///  on processing time than would the convergence tolerance.
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
///  slow search of interest must be performed. For an example, see
///  the program CASCADE in the GF Example Programs chapter of the GF
///  Required Reading, gf.req.
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
///  1) Search for times when Saturn's satellite Phoebe is within
///     the FOV of the Cassini narrow angle camera (CASSINI_ISS_NAC).
///     To simplify the problem, restrict the search to a short time
///     period where continuous Cassini bus attitude data are
///     available.
///
///     Use a step size of 10 seconds to reduce chances of missing
///     short visibility events.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: gftfov_ex1.tm
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
///           File name                       Contents
///           -----------------------------   ----------------------
///           naif0012.tls                    Leapseconds
///           pck00010.tpc                    Satellite orientation
///                                           and radii
///           041014R_SCPSE_01066_04199.bsp   CASSINI, planetary and
///                                           Saturn satellite
///                                           ephemeris
///           cas_v40.tf                      Cassini FK
///           04161_04164ra.bc                Cassini bus CK
///           cas00071.tsc                    Cassini SCLK kernel
///           cas_iss_v10.ti                  Cassini IK
///
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'naif0012.tls',
///                               'pck00010.tpc',
///                               '041014R_SCPSE_01066_04199.bsp',
///                               'cas_v40.tf',
///                               '04161_04164ra.bc',
///                               'cas00071.tsc',
///                               'cas_iss_v10.ti'            )
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM GFTFOV_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           INTEGER               WNCARD
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         META
///           PARAMETER           ( META   = 'gftfov_ex1.tm' )
///
///           CHARACTER*(*)         TIMFMT
///           PARAMETER           ( TIMFMT =
///          .      'YYYY-MON-DD HR:MN:SC.######::TDB' )
///
///           INTEGER               LBCELL
///           PARAMETER           ( LBCELL = -5 )
///
///           INTEGER               MAXWIN
///           PARAMETER           ( MAXWIN = 10000 )
///
///           INTEGER               CORLEN
///           PARAMETER           ( CORLEN = 10 )
///
///           INTEGER               BDNMLN
///           PARAMETER           ( BDNMLN = 36 )
///
///           INTEGER               FRNMLN
///           PARAMETER           ( FRNMLN = 32 )
///
///           INTEGER               SHPLEN
///           PARAMETER           ( SHPLEN = 25 )
///
///           INTEGER               TIMLEN
///           PARAMETER           ( TIMLEN = 35 )
///
///           INTEGER               LNSIZE
///           PARAMETER           ( LNSIZE = 80 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(CORLEN)    ABCORR
///           CHARACTER*(BDNMLN)    INST
///           CHARACTER*(LNSIZE)    LINE
///           CHARACTER*(BDNMLN)    OBSRVR
///           CHARACTER*(BDNMLN)    TARGET
///           CHARACTER*(FRNMLN)    TFRAME
///           CHARACTER*(TIMLEN)    TIMSTR ( 2 )
///           CHARACTER*(SHPLEN)    TSHAPE
///
///           DOUBLE PRECISION      CNFINE ( LBCELL : 2 )
///           DOUBLE PRECISION      ENDPT  ( 2 )
///           DOUBLE PRECISION      ET0
///           DOUBLE PRECISION      ET1
///           DOUBLE PRECISION      RESULT ( LBCELL : MAXWIN )
///           DOUBLE PRECISION      STEPSZ
///
///           INTEGER               I
///           INTEGER               J
///           INTEGER               N
///
///     C
///     C     Saved variables
///     C
///     C     The confinement and result windows CNFINE and RESULT are
///     C     saved because this practice helps to prevent stack
///     C     overflow.
///     C
///           SAVE                  CNFINE
///           SAVE                  RESULT
///
///     C
///     C     Load kernels.
///     C
///           CALL FURNSH ( META )
///
///     C
///     C     Initialize windows.
///     C
///           CALL SSIZED ( 2,      CNFINE )
///           CALL SSIZED ( MAXWIN, RESULT )
///
///     C
///     C     Insert search time interval bounds into the
///     C     confinement window.
///     C
///           CALL STR2ET ( '2004 JUN 11 06:30:00 TDB', ET0 )
///           CALL STR2ET ( '2004 JUN 11 12:00:00 TDB', ET1 )
///
///           CALL WNINSD ( ET0, ET1, CNFINE )
///
///     C
///     C     Initialize inputs for the search.
///     C
///           INST   = 'CASSINI_ISS_NAC'
///           TARGET = 'PHOEBE'
///           TSHAPE = 'ELLIPSOID'
///           TFRAME = 'IAU_PHOEBE'
///           ABCORR = 'LT+S'
///           OBSRVR = 'CASSINI'
///           STEPSZ = 10.D0
///
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Instrument: '//INST
///           WRITE (*,*) 'Target:     '//TARGET
///           WRITE (*,*) ' '
///
///     C
///     C     Perform the search.
///     C
///           CALL GFTFOV ( INST,   TARGET, TSHAPE, TFRAME,
///          .              ABCORR, OBSRVR, STEPSZ, CNFINE, RESULT )
///
///           N = WNCARD( RESULT )
///
///           IF ( N .EQ. 0 ) THEN
///
///              WRITE (*,*) 'No FOV intersection found.'
///
///           ELSE
///
///              WRITE (*, '(A)' ) '  Visibility start time (TDB)'
///          .    //               '           Stop time (TDB)'
///              WRITE (*, '(A)' ) '  ---------------------------'
///          .    //               '     ---------------------------'
///
///              DO I = 1, N
///
///                 CALL WNFETD ( RESULT, I, ENDPT(1), ENDPT(2) )
///
///                 DO J = 1, 2
///                    CALL TIMOUT ( ENDPT(J), TIMFMT, TIMSTR(J) )
///                 END DO
///
///                 LINE( :3) = ' '
///                 LINE(2: ) = TIMSTR(1)
///                 LINE(34:) = TIMSTR(2)
///
///                 WRITE (*,*) LINE
///
///              END DO
///
///           END IF
///
///           WRITE (*,*) ' '
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Instrument: CASSINI_ISS_NAC
///      Target:     PHOEBE
///
///       Visibility start time (TDB)           Stop time (TDB)
///       ---------------------------     ---------------------------
///       2004-JUN-11 07:35:27.066980     2004-JUN-11 08:48:03.954696
///       2004-JUN-11 09:02:56.580045     2004-JUN-11 09:35:04.038509
///       2004-JUN-11 09:49:56.476397     2004-JUN-11 10:22:04.242879
///       2004-JUN-11 10:36:56.283772     2004-JUN-11 11:09:04.397165
///       2004-JUN-11 11:23:56.020645     2004-JUN-11 11:56:04.733535
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The reference frame associated with INST must be
///      centered at the observer or must be inertial. No check is done
///      to ensure this.
///
///  2)  The kernel files to be used by GFTFOV must be loaded (normally
///      via the SPICELIB routine FURNSH) before GFTFOV is called.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  L.S. Elson         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.1, 06-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
///         Modified code example's output to comply with maximum line
///         length of header comments. Updated Example's kernels set to use
///         PDS archived data. Added SAVE statements for CNFINE and RESULT
///         variables in code example.
///
///         Updated description of RESULT argument in $Brief_I/O,
///         $Detailed_Input and $Detailed_Output.
///
/// -    SPICELIB Version 1.1.0, 28-FEB-2012 (EDW)
///
///         Implemented use of ZZHOLDD to allow user to alter convergence
///         tolerance.
///
///         Removed the STEP > 0 error check. The GFSSTP call includes
///         the check.
///
/// -    SPICELIB Version 1.0.0, 15-APR-2009 (NJB) (LSE) (EDW)
/// ```
pub fn gftfov(
    ctx: &mut SpiceContext,
    inst: &str,
    target: &str,
    tshape: &str,
    tframe: &str,
    abcorr: &str,
    obsrvr: &str,
    step: f64,
    cnfine: &[f64],
    result: &mut [f64],
) -> crate::Result<()> {
    GFTFOV(
        inst.as_bytes(),
        target.as_bytes(),
        tshape.as_bytes(),
        tframe.as_bytes(),
        abcorr.as_bytes(),
        obsrvr.as_bytes(),
        step,
        cnfine,
        result,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure GFTFOV ( GF, is target in FOV? )
pub fn GFTFOV(
    INST: &[u8],
    TARGET: &[u8],
    TSHAPE: &[u8],
    TFRAME: &[u8],
    ABCORR: &[u8],
    OBSRVR: &[u8],
    STEP: f64,
    CNFINE: &[f64],
    RESULT: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let CNFINE = DummyArray::new(CNFINE, LBCELL..);
    let mut RESULT = DummyArrayMut::new(RESULT, LBCELL..);
    let mut TOL: f64 = 0.0;
    let mut OK: bool = false;

    //
    // SPICELIB functions
    //

    //
    // External routines
    //
    //
    // Interrupt handler:
    //

    //
    // Routines to set step size, refine transition times
    // and report work:
    //

    //
    // Local parameters
    //
    //
    // Geometric quantity  bail switch:
    //

    //
    // Progress report switch:
    //

    //
    // Local variables
    //

    //
    // Ray direction vector required by GFFOVE. This is
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

    CHKIN(b"GFTFOV", ctx)?;

    //
    // Reject the target shape 'RAY'.
    //
    if EQSTR(TSHAPE, RYSHAP) {
        SETMSG(b"The target shape RAY is not supported by this routine. Use the routine GFRFOV instead.", ctx);
        SIGERR(b"SPICE(INVALIDOPTION)", ctx)?;
        CHKOUT(b"GFTFOV", ctx)?;
        return Ok(());
    }

    //
    // Note to maintenance programmer: input exception checks
    // are delegated to GFFOVE. If the implementation of that
    // routine changes, or if this routine is modified to call
    // a different routine in place of GFFOVE, then the error
    // handling performed by GFFOVE will have to be performed
    // here or in a routine called by this routine.
    //
    // Check the result window's size.
    //
    if (SIZED(RESULT.as_slice(), ctx)? < 2) {
        SETMSG(b"Result window size must be at least 2 but was #.", ctx);
        ERRINT(b"#", SIZED(RESULT.as_slice(), ctx)?, ctx);
        SIGERR(b"SPICE(WINDOWTOOSMALL)", ctx)?;
        CHKOUT(b"GFTFOV", ctx)?;
        return Ok(());
    }

    //
    // Set the step size.
    //
    GFSSTP(STEP, ctx)?;

    //
    // Retrieve the convergence tolerance, if set.
    //
    ZZHOLDD(ZZGET, GF_TOL, &mut OK, &mut TOL, ctx)?;

    //
    // Use the default value CNVTOL if no stored tolerance value.
    //
    if !OK {
        TOL = CNVTOL;
    }

    //
    // Look for solutions.
    //
    GFFOVE(
        INST,
        TSHAPE,
        save.RAYDIR.as_slice(),
        TARGET,
        TFRAME,
        ABCORR,
        OBSRVR,
        TOL,
        GFSTEP,
        GFREFN,
        RPT,
        GFREPI,
        GFREPU,
        GFREPF,
        BAIL,
        GFBAIL,
        CNFINE.as_slice(),
        RESULT.as_slice_mut(),
        ctx,
    )?;

    CHKOUT(b"GFTFOV", ctx)?;
    Ok(())
}
