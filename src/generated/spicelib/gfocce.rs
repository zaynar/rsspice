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
const LBCELL: i32 = -5;
const STEP: f64 = 1.0;
const CSTEP: bool = false;

/// GF, occultation event
///
/// Determine time intervals when an observer sees one target
/// occulted by another. Report progress and handle interrupts
/// if so commanded.
///
/// # Required Reading
///
/// * [FRAMES](crate::required_reading::frames)
/// * [GF](crate::required_reading::gf)
/// * [KERNEL](crate::required_reading::kernel)
/// * [NAIF_IDS](crate::required_reading::naif_ids)
/// * [SPK](crate::required_reading::spk)
/// * [TIME](crate::required_reading::time)
/// * [WINDOWS](crate::required_reading::windows)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  LBCELL     P   SPICE Cell lower bound.
///  OCCTYP     I   Type of occultation.
///  FRONT      I   Name of body occulting the other.
///  FSHAPE     I   Type of shape model used for front body.
///  FFRAME     I   Body-fixed, body-centered frame for front body.
///  BACK       I   Name of body occulted by the other.
///  BSHAPE     I   Type of shape model used for back body.
///  BFRAME     I   Body-fixed, body-centered frame for back body.
///  ABCORR     I   Aberration correction flag.
///  OBSRVR     I   Name of the observing body.
///  TOL        I   Convergence tolerance in seconds.
///  UDSTEP     I   Name of the routine that returns a time step.
///  UDREFN     I   Name of the routine that computes a refined time.
///  RPT        I   Progress report flag.
///  UDREPI     I   Function that initializes progress reporting.
///  UDREPU     I   Function that updates the progress report.
///  UDREPF     I   Function that finalizes progress reporting.
///  BAIL       I   Logical indicating program interrupt monitoring.
///  UDBAIL     I   Name of a routine that signals a program interrupt.
///  CNFINE     I   SPICE window to which the search is restricted.
///  RESULT    I-O  SPICE window containing results.
/// ```
///
/// # Detailed Input
///
/// ```text
///  OCCTYP   indicates the type of occultation that is to be found.
///           Supported values and corresponding definitions are:
///
///              'FULL'      denotes the full occultation of the body
///                          designated by BACK by the body designated
///                          by FRONT, as seen from the location of the
///                          observer. In other words, the occulted
///                          body is completely invisible as seen from
///                          the observer's location.
///
///              'ANNULAR'   denotes an annular occultation: the body
///                          designated by FRONT blocks part of, but
///                          not the limb of, the body designated by
///                          BACK, as seen from the location of the
///                          observer.
///
///              'PARTIAL'   denotes an partial, non-annular
///                          occultation: the body designated by FRONT
///                          blocks part, but not all, of the limb of
///                          the body designated by BACK, as seen from
///                          the location of the observer.
///
///              'ANY'       denotes any of the above three types of
///                          occultations: 'PARTIAL', 'ANNULAR', or
///                          'FULL'.
///
///                          'ANY' should be used to search for times
///                          when the body designated by FRONT blocks
///                          any part of the body designated by BACK.
///
///                          The option 'ANY' must be used if either
///                          the front or back target body is modeled
///                          as a point.
///
///           Case and leading or trailing blanks are not significant
///           in the string OCCTYP.
///
///  FRONT    is the name of the target body that occults --- that is,
///           passes in front of --- the other. Optionally, you may
///           supply the integer NAIF ID code for the body as a string.
///           For example both 'MOON' and '301' are legitimate strings
///           that designate the Moon.
///
///           Case and leading or trailing blanks are not significant
///           in the string FRONT.
///
///  FSHAPE   is a string indicating the geometric model used to
///           represent the shape of the front target body. The
///           supported options are:
///
///              'ELLIPSOID'
///
///                  Use a triaxial ellipsoid model with radius values
///                  provided via the kernel pool. A kernel variable
///                  having a name of the form
///
///                     BODYnnn_RADII
///
///                  where nnn represents the NAIF integer code
///                  associated with the body, must be present in the
///                  kernel pool. This variable must be associated with
///                  three numeric values giving the lengths of the
///                  ellipsoid's X, Y, and Z semi-axes.
///
///              'POINT'
///
///                  Treat the body as a single point. When a point
///                  target is specified, the occultation type must be
///                  set to 'ANY'.
///
///              'DSK/UNPRIORITIZED[/SURFACES = <surface list>]'
///
///                  Use topographic data provided by DSK files to
///                  model the body's shape. These data must be
///                  provided by loaded DSK files.
///
///                  The surface list specification is optional. The
///                  syntax of the list is
///
///                     <surface 1> [, <surface 2>...]
///
///                  If present, it indicates that data only for the
///                  listed surfaces are to be used; however, data need
///                  not be available for all surfaces in the list. If
///                  absent, loaded DSK data for any surface associated
///                  with the target body are used.
///
///                  The surface list may contain surface names or
///                  surface ID codes. Names containing blanks must be
///                  delimited by double quotes, for example
///
///                     SURFACES = "Mars MEGDR 128 PIXEL/DEG"
///
///                  If multiple surfaces are specified, their names or
///                  IDs must be separated by commas.
///
///                  See the $Particulars section below for details
///                  concerning use of DSK data.
///
///           The combinations of the shapes of the target bodies
///           FRONT and BACK must be one of:
///
///              One ELLIPSOID, one POINT
///              Two ELLIPSOIDs
///              One DSK, one POINT
///
///           Case and leading or trailing blanks are not
///           significant in the string FSHAPE.
///
///  FFRAME   is the name of the body-fixed, body-centered reference
///           frame associated with the front target body. Examples
///           of such names are 'IAU_SATURN' (for Saturn) and
///           'ITRF93' (for the Earth).
///
///           If the front target body is modeled as a point, FFRAME
///           should be left blank.
///
///           Case and leading or trailing blanks are not
///           significant in the string FFRAME.
///
///  BACK     is the name of the target body that is occulted by ---
///           that is, passes in back of --- the other. Optionally, you
///           may supply the integer NAIF ID code for the body as a
///           string. For example both 'MOON' and '301' are legitimate
///           strings that designate the Moon.
///
///           Case and leading or trailing blanks are not
///           significant in the string BACK.
///
///  BSHAPE   is the shape specification for the body designated by
///           BACK. The supported options are those for FSHAPE. See the
///           description of FSHAPE above for details.
///
///  BFRAME   is the name of the body-fixed, body-centered reference
///           frame associated with the "back" target body. See the
///           description of FFRAME above for details. Examples of such
///           names are 'IAU_SATURN' (for Saturn) and 'ITRF93' (for the
///           Earth).
///
///           If the back target body is modeled as a point, BFRAME
///           should be left blank.
///
///           Case and leading or trailing blanks bracketing a
///           non-blank frame name are not significant in the string
///           BFRAME.
///
///  ABCORR   indicates the aberration corrections to be applied to the
///           state of the target body to account for one-way light
///           time. Stellar aberration corrections are ignored if
///           specified, since these corrections don't improve the
///           accuracy of the occultation determination.
///
///           See the header of the SPICE routine SPKEZR for a detailed
///           description of the aberration correction options. For
///           convenience, the options supported by this routine are
///           listed below:
///
///              'NONE'     Apply no correction.
///
///              'LT'       "Reception" case: correct for
///                         one-way light time using a Newtonian
///                         formulation.
///
///              'CN'       "Reception" case: converged
///                         Newtonian light time correction.
///
///              'XLT'      "Transmission" case: correct for
///                         one-way light time using a Newtonian
///                         formulation.
///
///              'XCN'      "Transmission" case: converged
///                         Newtonian light time correction.
///
///           Case and blanks are not significant in the string
///           ABCORR.
///
///  OBSRVR   is the name of the body from which the occultation is
///           observed. Optionally, you may supply the integer NAIF
///           ID code for the body as a string.
///
///           Case and leading or trailing blanks are not
///           significant in the string OBSRVR.
///
///  TOL      is a tolerance value used to determine convergence of
///           root-finding operations. TOL is measured in TDB seconds
///           and must be greater than zero.
///
///  UDSTEP   is an externally specified routine that computes a
///           time step used to find transitions of the state being
///           considered. A state transition occurs where the state
///           changes from being "in occultation" to being "not in
///           occultation" or vice versa.
///
///           This routine relies on UDSTEP returning step sizes small
///           enough so that state transitions within the confinement
///           window are not overlooked. There must never be two roots
///           A and B separated by less than STEP, where STEP is the
///           minimum step size returned by UDSTEP for any value of ET;
///           in the interval [A, B].
///
///           The calling sequence for UDSTEP is:
///
///              CALL UDSTEP ( ET, STEP )
///
///           where:
///
///              ET      is the input start time from which the
///                      algorithm is to search forward for a state
///                      transition. ET is expressed as seconds past
///                      J2000 TDB. ET is a DOUBLE PRECISION number.
///
///              STEP    is the output step size. STEP indicates
///                      how far to advance ET so that ET and
///                      ET+STEP may bracket a state transition and
///                      definitely do not bracket more than one
///                      state transition. STEP is a DOUBLE
///                      PRECISION number. Units are TDB seconds.
///
///           If a constant step size is desired, the SPICELIB routine
///
///              GFSTEP
///
///           may be used as the step size function. If GFSTEP is used,
///           the step size must be set by calling GFSSTP prior to
///           calling this routine.
///
///  UDREFN   is the name of the externally specified routine that
///           refines the times that bracket a transition point. In
///           other words, once a pair of times, T1 and T2, that
///           bracket a state transition have been found, UDREFN
///           computes an intermediate time T such that either [T1, T]
///           or [T, T2] contains the time of the state transition. The
///           calling sequence for UDREFN is:
///
///              CALL UDREFN ( T1, T2, S1, S2, T )
///
///           where the inputs are:
///
///              T1    is a time when the visibility state is S1. T1
///                    is expressed as seconds past J2000 TDB.
///
///              T2    is a time when the visibility state is S2. T2 is
///                    expressed as seconds past J2000 TDB. T2 is
///                    assumed to be larger than T1.
///
///              S1    is the visibility state at time T1. S1 is a
///                    LOGICAL value.
///
///              S2    is the visibility state at time T2. S2 is a
///                    LOGICAL value.
///
///           The output is:
///
///              T     is the next time to check for a state
///                    transition. T is expressed as seconds past
///                    J2000 TDB and is between T1 and T2.
///
///           If a simple bisection method is desired, the SPICELIB
///           routine GFREFN may be used.
///
///  RPT      is a logical variable which controls whether progress
///           reporting is enabled. When RPT is .TRUE., progress
///           reporting is enabled and the routines UDREPI, UDREPU, and
///           UDREPF (see descriptions below) are used to report
///           progress.
///
///  UDREPI   is a user-defined subroutine that initializes a progress
///           report. When progress reporting is enabled, UDREPI is
///           called at the start of a search. The calling sequence of
///           UDREPI is
///
///              UDREPI ( CNFINE, SRCPRE, SRCSUF )
///
///              DOUBLE PRECISION    CNFINE ( LBCELL : * )
///              CHARACTER*(*)       SRCPRE
///              CHARACTER*(*)       SRCSUF
///
///           where
///
///              CNFINE
///
///           is the confinement window specifying the time period over
///           which a search is conducted, and
///
///              SRCPRE
///              SRCSUF
///
///           are prefix and suffix strings used in the progress
///           report: these strings are intended to bracket a
///           representation of the fraction of work done. For example,
///           when the CSPICE progress reporting functions are used, if
///           srcpre and srcsuf are, respectively,
///
///              "Occultation/transit search"
///              "done."
///
///           the progress report display at the end of the
///           search will be:
///
///              Occultation/transit search 100.00% done.
///
///           The SPICELIB routine GFREPI may be used as the actual
///           argument corresponding to UDREPI. If so, the SPICELIB
///           routines GFREPU and GFREPF must be the actual arguments
///           corresponding to UDREPU and UDREPF.
///
///  UDREPU   is a user-defined subroutine that updates the progress
///           report for a search. The calling sequence of UDREPU is
///
///              UDREPU ( IVBEG, IVEND, ET )
///
///              DOUBLE PRECISION      IVBEG
///              DOUBLE PRECISION      IVEND
///              DOUBLE PRECISION      ET
///
///           Here IVBEG, IVEND are the bounds of an interval that is
///           contained in some interval belonging to the confinement
///           window. The confinement window is associated with some
///           root finding activity. It is used to determine how much
///           total time is being searched in order to find the events
///           of interest.
///
///           ET is an epoch belonging to the interval
///           [IVBEG, IVEND].
///
///           In order for a meaningful progress report to be
///           displayed, IVBEG and IVEND must satisfy the following
///           constraints:
///
///            - IVBEG must be less than or equal to IVEND.
///
///            - The interval [ IVBEG, IVEND ] must be contained in
///              some interval of the confinement window. It can be
///              a proper subset of the containing interval; that
///              is, it can be smaller than the interval of the
///              confinement window that contains it.
///
///            - Over a search, the sum of the differences
///
///                 IVEND - IVBEG
///
///              for all calls to this routine made during the search
///              must equal the measure of the confinement window.
///
///           The SPICELIB routine GFREPU may be used as the actual
///           argument corresponding to UDREPU. If so, the SPICELIB
///           routines GFREPI and GFREPF must be the actual arguments
///           corresponding to UDREPI and UDREPF.
///
///  UDREPF   is a user-defined subroutine that finalizes a
///           progress report. UDREPF has no arguments.
///
///           The SPICELIB routine GFREPF may be used as the actual
///           argument corresponding to UDREPF. If so, the SPICELIB
///           routines GFREPI and GFREPU must be the actual arguments
///           corresponding to UDREPI and UDREPU.
///
///  BAIL     is a logical variable indicating whether or not interrupt
///           handling is enabled. When BAIL is set to .TRUE., the
///           input function UDBAIL (see description below) is used to
///           determine whether an interrupt has been issued.
///
///  UDBAIL   is the name of a user defined logical function that
///           indicates whether an interrupt signal has been issued
///           (for example, from the keyboard).  UDBAIL has no
///           arguments and returns a LOGICAL value. The return value
///           is .TRUE. if an interrupt has been issued; otherwise the
///           value is .FALSE.
///
///           GFOCCE uses UDBAIL only when BAIL (see above) is set to
///           .TRUE., indicating that interrupt handling is enabled.
///           When interrupt handling is enabled, GFOCCE and routines
///           in its call tree will call UDBAIL to determine whether to
///           terminate processing and return immediately.
///
///           If interrupt handing is not enabled, a logical function
///           must still be passed to GFOCCE as an input argument. The
///           SPICELIB function
///
///              GFBAIL
///
///           may be used for this purpose.
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
///           discarded before GFOCCE conducts its search.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RESULT   is a SPICE window representing the set of time intervals,
///           within the confinement period, when the specified
///           occultation occurs.
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
///  LBCELL   is the SPICE cell lower bound.
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
///      that if the step size is non-positive, an error is signaled
///      by a routine in the call tree of this routine.
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
///  3)  If name of either target or the observer cannot be translated
///      to a NAIF ID code, an error is signaled by a routine in the
///      call tree of this routine.
///
///  4)  If the radii of a target body modeled as an ellipsoid cannot
///      be determined by searching the kernel pool for a kernel
///      variable having a name of the form
///
///         'BODYnnn_RADII'
///
///      where nnn represents the NAIF integer code associated with
///      the body, an error is signaled by a routine in the call tree
///      of this routine.
///
///  5)  If either of the target bodies FRONT or BACK coincides with
///      the observer body OBSRVR, an error is signaled by a routine in
///      the call tree of this routine.
///
///  6)  If the body designated by FRONT coincides with that designated
///      by BACK, an error is signaled by a routine in the call tree of
///      this routine.
///
///  7)  If either of the body model specifiers FSHAPE or BSHAPE is not
///      recognized, an error is signaled by a routine in the call tree
///      of this routine.
///
///  8)  If both of the body model specifiers FSHAPE and BSHAPE
///      specify point targets, the error SPICE(INVALIDSHAPECOMBO)
///      is signaled.
///
///  9)  If a target body-fixed reference frame associated with a
///      non-point target is not recognized, an error is signaled by a
///      routine in the call tree of this routine.
///
///  10) If a target body-fixed reference frame is not centered at the
///      corresponding target body, an error is signaled by a routine
///      in the call tree of this routine.
///
///  11) If the loaded kernels provide insufficient data to compute the
///      requested state vector, an error is signaled by a routine in
///      the call tree of this routine.
///
///  12) If an error occurs while reading an SPK or other kernel file,
///      the error is signaled by a routine in the call tree
///      of this routine.
///
///  13) If a point target is specified and the occultation type is set
///      to a valid value other than 'ANY', an error is signaled by a
///      routine in the call tree of this routine.
///
///  14) If the output SPICE window RESULT has insufficient capacity to
///      contain the number of intervals on which the specified
///      occultation condition is met, an error is signaled by a
///      routine in the call tree of this routine.
///
///  15) If the result window has size less than 2, the error
///      SPICE(WINDOWTOOSMALL) is signaled.
///
///  16) If the occultation type OCCTYP is invalid, an error is
///      signaled by a routine in the call tree of this routine.
///
///  17) If the aberration correction specification ABCORR is invalid,
///      an error is signaled by a routine in the call tree of this
///      routine.
///
///  18) If the convergence tolerance size is non-positive, the error
///      SPICE(INVALIDTOLERANCE) is signaled.
///
///  19) If either FSHAPE or BSHAPE specifies that the target surface
///      is represented by DSK data, and no DSK files are loaded for
///      the specified target, an error is signaled by a routine in
///      the call tree of this routine.
///
///  20) If either FSHAPE or BSHAPE specifies that the target surface
///      is represented by DSK data, but the shape specification is
///      invalid, an error is signaled by a routine in the call tree
///      of this routine.
///
///  21) If operation of this routine is interrupted, the output result
///      window will be invalid.
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
///  -  SPK data: the calling application must load ephemeris data
///     for the target, source and observer that cover the time
///     period specified by the window CNFINE. If aberration
///     corrections are used, the states of the target bodies and of
///     the observer relative to the solar system barycenter must be
///     calculable from the available ephemeris data. Typically
///     ephemeris data
///     are made available by loading one or more SPK files via
///     FURNSH.
///
///  -  PCK data: bodies modeled as triaxial ellipsoids must have
///     semi-axis lengths provided by variables in the kernel pool.
///     Typically these data are made available by loading a text
///     PCK file via FURNSH.
///
///  -  FK data: if either of the reference frames designated by
///     BFRAME or FFRAME are not built in to the SPICE system,
///     one or more FKs specifying these frames must be loaded.
///
///  The following data may be required:
///
///  -  DSK data: if either FSHAPE or BSHAPE indicates that DSK
///     data are to be used, DSK files containing topographic data
///     for the target body must be loaded. If a surface list is
///     specified, data for at least one of the listed surfaces must
///     be loaded.
///
///  -  Surface name-ID associations: if surface names are specified
///     in FSHAPE or BSHAPE, the association of these names with
///     their corresponding surface ID codes must be established by
///     assignments of the kernel variables
///
///        NAIF_SURFACE_NAME
///        NAIF_SURFACE_CODE
///        NAIF_SURFACE_BODY
///
///     Normally these associations are made by loading a text
///     kernel containing the necessary assignments. An example
///     of such a set of assignments is
///
///        NAIF_SURFACE_NAME += 'Mars MEGDR 128 PIXEL/DEG'
///        NAIF_SURFACE_CODE += 1
///        NAIF_SURFACE_BODY += 499
///
///  -  CK data: either of the body-fixed frames to which FFRAME or
///     BFRAME refer might be a CK frame. If so, at least one CK
///     file will be needed to permit transformation of vectors
///     between that frame and the J2000 frame.
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
///  This routine provides the SPICE GF system's most flexible
///  interface for searching for occultation events.
///
///  Applications that require do not require support for progress
///  reporting, interrupt handling, non-default step or refinement
///  functions, or non-default convergence tolerance normally should
///  call GFOCLT rather than this routine.
///
///  This routine determines a set of one or more time intervals
///  within the confinement window when a specified type of
///  occultation occurs. The resulting set of intervals is returned as
///  a SPICE window.
///
///  Below we discuss in greater detail aspects of this routine's
///  solution process that are relevant to correct and efficient
///  use of this routine in user applications.
///
///
///  The Search Process
///  ==================
///
///  The search for occultations is treated as a search for state
///  transitions: times are sought when the state of the BACK body
///  changes from "not occulted" to "occulted" or vice versa.
///
///  Step Size
///  =========
///
///  Each interval of the confinement window is searched as follows:
///  first, the input step size is used to determine the time
///  separation at which the occultation state will be sampled.
///  Starting at the left endpoint of an interval, samples will be
///  taken at each step. If a state change is detected, a root has
///  been bracketed; at that point, the "root"--the time at which the
///  state change occurs---is found by a refinement process, for
///  example, via binary search.
///
///  Note that the optimal choice of step size depends on the lengths
///  of the intervals over which the occultation state is constant:
///  the step size should be shorter than the shortest occultation
///  duration and the shortest period between occultations, within
///  the confinement window.
///
///  Having some knowledge of the relative geometry of the targets and
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
///  "convergence tolerance."
///
///  The convergence tolerance used by high-level GF routines that
///  call this routine is set via the parameter CNVTOL, which is
///  declared in the INCLUDE file gf.inc. The value of CNVTOL is set
///  to a "tight" value so that the tolerance doesn't become the
///  limiting factor in the accuracy of solutions found by this
///  routine. In general the accuracy of input data will be the
///  limiting factor.
///
///  Setting the input tolerance TOL tighter than CNVTOL is unlikely
///  to be useful, since the results are unlikely to be more accurate.
///  Making the tolerance looser will speed up searches somewhat,
///  since a few convergence steps will be omitted. However, in most
///  cases, the step size is likely to have a much greater effect on
///  processing time than would the convergence tolerance.
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
///     required in the FSHAPE and BSHAPE arguments.
///
///
///     Syntax of the shape input arguments for the DSK case
///     ----------------------------------------------------
///
///     The keywords and surface list in the target shape arguments
///     FSHAPE and BSHAPE, when DSK shape models are specified, are
///     called "clauses." The clauses may appear in any order, for
///     example
///
///        DSK/<surface list>/UNPRIORITIZED
///        DSK/UNPRIORITIZED/<surface list>
///        UNPRIORITIZED/<surface list>/DSK
///
///     The simplest form of a target argument specifying use of
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
///     An example of a shape argument that could be constructed
///     using one of the surface lists above is
///
///           'DSK/UNPRIORITIZED/SURFACES = '
///        // '"Mars MEGDR 64 PIXEL/DEG", 499003'
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
///  1) Conduct a search using the default GF progress reporting
///     capability.
///
///     The program will use console I/O to display a simple
///     ASCII-based progress report.
///
///     The program will find occultations of the Sun by the Moon as
///     seen from the center of the Earth over the month December,
///     2001.
///
///     We use light time corrections to model apparent positions of
///     Sun and Moon. Stellar aberration corrections are not specified
///     because they don't affect occultation computations.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: gfocce_ex1.tm
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
///           pck00008.tpc                  Planet orientation and
///                                         radii
///           naif0009.tls                  Leapseconds
///
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de421.bsp',
///                               'pck00008.tpc',
///                               'naif0009.tls'  )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM GFOCCE_EX1
///           IMPLICIT NONE
///
///           EXTERNAL              GFSTEP
///           EXTERNAL              GFREFN
///           EXTERNAL              GFREPI
///           EXTERNAL              GFREPU
///           EXTERNAL              GFREPF
///
///           INTEGER               WNCARD
///           LOGICAL               GFBAIL
///           EXTERNAL              GFBAIL
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         TIMFMT
///           PARAMETER           ( TIMFMT =
///          .   'YYYY MON DD HR:MN:SC.###### ::TDB (TDB)' )
///
///           DOUBLE PRECISION      CNVTOL
///           PARAMETER           ( CNVTOL = 1.D-6 )
///
///           INTEGER               MAXWIN
///           PARAMETER           ( MAXWIN = 2 * 100 )
///
///           INTEGER               TIMLEN
///           PARAMETER           ( TIMLEN = 40 )
///
///           INTEGER               LBCELL
///           PARAMETER           ( LBCELL = -5 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(TIMLEN)    WIN0
///           CHARACTER*(TIMLEN)    WIN1
///           CHARACTER*(TIMLEN)    BEGSTR
///           CHARACTER*(TIMLEN)    ENDSTR
///
///           DOUBLE PRECISION      CNFINE ( LBCELL : 2 )
///           DOUBLE PRECISION      ET0
///           DOUBLE PRECISION      ET1
///           DOUBLE PRECISION      LEFT
///           DOUBLE PRECISION      RESULT ( LBCELL : MAXWIN )
///           DOUBLE PRECISION      RIGHT
///
///           INTEGER               I
///
///           LOGICAL               BAIL
///           LOGICAL               RPT
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
///           CALL FURNSH ( 'gfocce_ex1.tm' )
///
///     C
///     C     Initialize the confinement and result windows.
///     C
///           CALL SSIZED ( 2,      CNFINE )
///           CALL SSIZED ( MAXWIN, RESULT )
///
///     C
///     C     Obtain the TDB time bounds of the confinement
///     C     window, which is a single interval in this case.
///     C
///           WIN0 = '2001 DEC 01 00:00:00 TDB'
///           WIN1 = '2002 JAN 01 00:00:00 TDB'
///
///           CALL STR2ET ( WIN0, ET0 )
///           CALL STR2ET ( WIN1, ET1 )
///
///     C
///     C     Insert the time bounds into the confinement
///     C     window.
///     C
///           CALL WNINSD ( ET0, ET1, CNFINE )
///
///     C
///     C     Select a 20 second step. We'll ignore any occultations
///     C     lasting less than 20 seconds.
///     C
///           CALL GFSSTP ( 20.D0 )
///
///     C
///     C     Turn on progress reporting; turn off interrupt
///     C     handling.
///     C
///           RPT  = .TRUE.
///           BAIL = .FALSE.
///
///     C
///     C     Perform the search.
///     C
///           CALL GFOCCE ( 'ANY',
///          .              'MOON',   'ellipsoid',  'IAU_MOON',
///          .              'SUN',    'ellipsoid',  'IAU_SUN',
///          .              'LT',     'EARTH',      CNVTOL,
///          .              GFSTEP,   GFREFN,       RPT,
///          .              GFREPI,   GFREPU,       GFREPF,
///          .              BAIL,     GFBAIL,       CNFINE,  RESULT )
///
///
///           IF ( WNCARD(RESULT) .EQ. 0 ) THEN
///
///              WRITE (*,*) 'No occultation was found.'
///
///           ELSE
///
///              DO I = 1, WNCARD(RESULT)
///
///     C
///     C           Fetch and display each occultation interval.
///     C
///                 CALL WNFETD ( RESULT, I, LEFT, RIGHT )
///
///                 CALL TIMOUT ( LEFT,  TIMFMT, BEGSTR )
///                 CALL TIMOUT ( RIGHT, TIMFMT, ENDSTR )
///
///                 WRITE (*,*) 'Interval ', I
///                 WRITE (*,*) '   Start time: '//BEGSTR
///                 WRITE (*,*) '   Stop time:  '//ENDSTR
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
///     Occultation/transit search 100.00% done.
///
///      Interval            1
///         Start time: 2001 DEC 14 20:10:14.195952  (TDB)
///         Stop time:  2001 DEC 14 21:35:50.317994  (TDB)
///
///
///     Note that the progress report has the format shown below:
///
///        Occultation/transit search   6.02% done.
///
///     The completion percentage was updated approximately once per
///     second.
///
///     When the program was interrupted at an arbitrary time,
///     the output was:
///
///        Occultation/transit search  13.63% done.
///        Search was interrupted.
///
///     This message was written after an interrupt signal
///     was trapped. By default, the program would have terminated
///     before this message could be written.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  If the caller passes in the default, constant step
///      size routine, GFSTEP, the caller must set the step
///      size by calling the entry point GFSSTP before
///      calling GFOCCE. The call syntax for GFSSTP is
///
///         CALL GFSSTP ( STEP )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  L.S. Elson         (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.0.1, 27-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
///         Added note on program interruption in $Examples section.
///         Renamed example's meta-kernel. Added SAVE statements for CNFINE
///         and RESULT variables in code example.
///
///         Updated description of UDSTEP, UDREPI and RESULT arguments.
///
///         Added entries #15 and #21 to the $Exceptions section.
///
///         Corrected reporting message in UDREPI description.
///
/// -    SPICELIB Version 2.0.0, 24-FEB-2016 (NJB)
///
///         Now supports DSK target shapes.
///
///         Updated lengths of saved shape variables to accommodate
///         DSK "method" specifications.
///
/// -    SPICELIB Version 1.0.0, 15-APR-2009 (NJB) (LSE) (WLT) (IMU) (EDW)
/// ```
pub fn gfocce(
    ctx: &mut SpiceContext,
    occtyp: &str,
    front: &str,
    fshape: &str,
    fframe: &str,
    back: &str,
    bshape: &str,
    bframe: &str,
    abcorr: &str,
    obsrvr: &str,
    tol: f64,
    udstep: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    udrefn: fn(f64, f64, bool, bool, &mut f64) -> (),
    rpt: bool,
    udrepi: fn(&[f64], &[u8], &[u8], &mut Context) -> f2rust_std::Result<()>,
    udrepu: fn(f64, f64, f64, &mut Context) -> f2rust_std::Result<()>,
    udrepf: fn(&mut Context) -> f2rust_std::Result<()>,
    bail: bool,
    udbail: fn() -> bool,
    cnfine: &[f64],
    result: &mut [f64],
) -> crate::Result<()> {
    GFOCCE(
        occtyp.as_bytes(),
        front.as_bytes(),
        fshape.as_bytes(),
        fframe.as_bytes(),
        back.as_bytes(),
        bshape.as_bytes(),
        bframe.as_bytes(),
        abcorr.as_bytes(),
        obsrvr.as_bytes(),
        tol,
        udstep,
        udrefn,
        rpt,
        udrepi,
        udrepu,
        udrepf,
        bail,
        udbail,
        cnfine,
        result,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure GFOCCE ( GF, occultation event )
pub fn GFOCCE(
    OCCTYP: &[u8],
    FRONT: &[u8],
    FSHAPE: &[u8],
    FFRAME: &[u8],
    BACK: &[u8],
    BSHAPE: &[u8],
    BFRAME: &[u8],
    ABCORR: &[u8],
    OBSRVR: &[u8],
    TOL: f64,
    UDSTEP: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    UDREFN: fn(f64, f64, bool, bool, &mut f64) -> (),
    RPT: bool,
    UDREPI: fn(&[f64], &[u8], &[u8], &mut Context) -> f2rust_std::Result<()>,
    UDREPU: fn(f64, f64, f64, &mut Context) -> f2rust_std::Result<()>,
    UDREPF: fn(&mut Context) -> f2rust_std::Result<()>,
    BAIL: bool,
    UDBAIL: fn() -> bool,
    CNFINE: &[f64],
    RESULT: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let CNFINE = DummyArray::new(CNFINE, LBCELL..);
    let mut RESULT = DummyArrayMut::new(RESULT, LBCELL..);
    let mut LBSHAP = [b' '; MTHLEN as usize];
    let mut LFSHAP = [b' '; MTHLEN as usize];
    let mut FINISH: f64 = 0.0;
    let mut START: f64 = 0.0;
    let mut COUNT: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // External routines
    //

    //
    // Local parameters
    //

    //
    // STEP is a step size initializer for the unused, dummy step size
    // argument to ZZGFSOLV. The routine UDSTEP, which is passed to
    // ZZGFSOLV, will be used by that routine to obtain the step size.
    //

    //
    // CSTEP indicates whether a constant step size, provided
    // via the input argument STEP, is to be used by ZZGFSOLV.
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"GFOCCE", ctx)?;

    //
    // Check the result window's size.
    //
    if (SIZED(RESULT.as_slice(), ctx)? < 2) {
        SETMSG(b"Result window size must be at least 2 but was #.", ctx);
        ERRINT(b"#", SIZED(RESULT.as_slice(), ctx)?, ctx);
        SIGERR(b"SPICE(WINDOWTOOSMALL)", ctx)?;
        CHKOUT(b"GFOCCE", ctx)?;
        return Ok(());
    }

    //
    // Empty the RESULT window.
    //
    SCARDD(0, RESULT.as_slice_mut(), ctx)?;

    //
    // Check the convergence tolerance.
    //
    if (TOL <= 0.0) {
        SETMSG(b"Tolerance must be positive but was #.", ctx);
        ERRDP(b"#", TOL, ctx);
        SIGERR(b"SPICE(INVALIDTOLERANCE)", ctx)?;
        CHKOUT(b"GFOCCE", ctx)?;
        return Ok(());
    }

    //
    // Check the target shape specifications.
    //
    LJUST(BSHAPE, &mut LBSHAP);
    UCASE(&LBSHAP.clone(), &mut LBSHAP, ctx);

    LJUST(FSHAPE, &mut LFSHAP);
    UCASE(&LFSHAP.clone(), &mut LFSHAP, ctx);

    //
    // Note for maintenance programmer: these checks will
    // require modification to handle DSK-based shapes.
    //

    if (fstr::eq(&LFSHAP, PTSHAP) && fstr::eq(&LBSHAP, PTSHAP)) {
        SETMSG(b"The front and back target shape specifications are both PTSHAP; at least one of these targets must be an extended object.", ctx);
        SIGERR(b"SPICE(INVALIDSHAPECOMBO)", ctx)?;
        CHKOUT(b"GFOCCE", ctx)?;
        return Ok(());
    }

    //
    // Initialize the occultation calculation.
    //
    ZZGFOCIN(
        OCCTYP, FRONT, &LFSHAP, FFRAME, BACK, &LBSHAP, BFRAME, OBSRVR, ABCORR, ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"GFOCCE", ctx)?;
        return Ok(());
    }

    //
    // Prepare the progress reporter if appropriate.
    //
    if RPT {
        UDREPI(
            CNFINE.as_slice(),
            b"Occultation/transit search ",
            b"done.",
            ctx,
        )?;
    }

    //
    // Cycle over the intervals in the confining window.
    //
    COUNT = WNCARD(CNFINE.as_slice(), ctx)?;

    for I in 1..=COUNT {
        //
        // Retrieve the bounds for the Ith interval of the confinement
        // window. Search this interval for occultation events. Union the
        // result with the contents of the RESULT window.
        //
        WNFETD(CNFINE.as_slice(), I, &mut START, &mut FINISH, ctx)?;

        ZZGFSOLV(
            ZZGFOCST,
            UDSTEP,
            UDREFN,
            BAIL,
            UDBAIL,
            CSTEP,
            STEP,
            START,
            FINISH,
            TOL,
            RPT,
            UDREPU,
            RESULT.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"GFOCCE", ctx)?;
            return Ok(());
        }

        if BAIL {
            //
            // Interrupt handling is enabled.
            //
            if UDBAIL() {
                //
                // An interrupt has been issued. Return now regardless of
                // whether the search has been completed.
                //
                CHKOUT(b"GFOCCE", ctx)?;
                return Ok(());
            }
        }
    }

    //
    // End the progress report.
    //
    if RPT {
        UDREPF(ctx)?;
    }

    CHKOUT(b"GFOCCE", ctx)?;
    Ok(())
}
