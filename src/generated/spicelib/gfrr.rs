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
const RECSYS: &[u8] = b"RECTANGULAR";
const LATSYS: &[u8] = b"LATITUDINAL";
const SPHSYS: &[u8] = b"SPHERICAL";
const RADSYS: &[u8] = b"RA/DEC";
const CYLSYS: &[u8] = b"CYLINDRICAL";
const GEOSYS: &[u8] = b"GEODETIC";
const PGRSYS: &[u8] = b"PLANETOGRAPHIC";
const XCRD: &[u8] = b"X";
const YCRD: &[u8] = b"Y";
const ZCRD: &[u8] = b"Z";
const RADCRD: &[u8] = b"RADIUS";
const LONCRD: &[u8] = b"LONGITUDE";
const LATCRD: &[u8] = b"LATITUDE";
const RACRD: &[u8] = b"RIGHT ASCENSION";
const DECCRD: &[u8] = b"DECLINATION";
const RNGCRD: &[u8] = b"RANGE";
const CLTCRD: &[u8] = b"COLATITUDE";
const ALTCRD: &[u8] = b"ALTITUDE";
const POSDEF: &[u8] = b"POSITION";
const SOBDEF: &[u8] = b"SUB-OBSERVER POINT";
const SINDEF: &[u8] = b"SURFACE INTERCEPT POINT";
const NWREL: i32 = 5;
const NWLONG: i32 = 7;
const EXWIDX: i32 = ((NWREL + NWLONG) + 1);
const MXBEGM: i32 = 55;
const MXENDM: i32 = 13;
const MXMSG: i32 = ((MXBEGM + MXENDM) + 10);
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
const QNPARS: i32 = 3;
const NOBAIL: bool = false;
const NORPT: bool = false;

/// GF, range rate search
///
/// Determine time intervals for which a specified constraint
/// on the observer-target range rate is met.
///
/// # Required Reading
///
/// * [GF](crate::required_reading::gf)
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
///  CNVTOL     P   Convergence tolerance.
///  TARGET     I   Name of the target body.
///  ABCORR     I   Aberration correction flag.
///  OBSRVR     I   Name of the observing body.
///  RELATE     I   Relational operator.
///  REFVAL     I   Reference value.
///  ADJUST     I   Adjustment value for absolute extrema searches.
///  STEP       I   Step size used for locating extrema and roots.
///  CNFINE     I   SPICE window to which the search is confined.
///  MW         I   Workspace window size.
///  NW         I   The number of workspace windows needed for
///                 the search.
///  WORK       O   Array of workspace windows.
///  RESULT    I-O  SPICE window containing results.
/// ```
///
/// # Detailed Input
///
/// ```text
///  TARGET   is the name of a target body. Optionally, you may supply
///           the integer ID code for the object as an integer string.
///           For example both 'MOON' and '301' are legitimate strings
///           that indicate the moon is the target body.
///
///           The target and observer define a position vector that
///           points from the observer to the target. The derivative
///           with respect to time of the length of this vector is the
///           "range rate" used by this routine as the geometric
///           quantity of interest.
///
///           Case and leading or trailing blanks are not significant
///           in the string TARGET.
///
///  ABCORR   is the description of the aberration corrections to apply
///           to the state evaluations to account for one-way light
///           time and stellar aberration.
///
///           This routine accepts the same aberration corrections as
///           does the SPICE routine SPKEZR. See the header of SPKEZR
///           for a detailed description of the aberration correction
///           options. For convenience, the options are listed below:
///
///              'NONE'     Apply no correction. Returns the "true"
///                         geometric state.
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
///              'XLT'      "Transmission" case: correct for
///                         one-way light time using a Newtonian
///                         formulation.
///
///              'XLT+S'    "Transmission" case: correct for
///                         one-way light time and stellar
///                         aberration using a Newtonian
///                         formulation.
///
///              'XCN'      "Transmission" case: converged
///                         Newtonian light time correction.
///
///              'XCN+S'    "Transmission" case: converged
///                         Newtonian light time and stellar
///                         aberration corrections.
///
///           Case and leading or trailing blanks are not significant
///           in the string ABCORR.
///
///  OBSRVR   is the name of an observing body. Optionally, you may
///           supply the ID code of the object as an integer string.
///           For example, both 'EARTH' and '399' are legitimate
///           strings to indicate that the observer is the Earth.
///
///           Case and leading or trailing blanks are not significant
///           in the string OBSRVR.
///
///  RELATE   is the relational operator that defines the constraint on
///           the range rate of the observer-target vector. The result
///           window found by this routine indicates the time intervals
///           where the constraint is satisfied. Supported values of
///           RELATE and corresponding meanings are shown below:
///
///              '>'        The range rate value is greater than the
///                         reference value REFVAL.
///
///              '='        The range rate value is equal to the
///                         reference value REFVAL.
///
///              '<'        The range rate value is less than the
///                         reference value REFVAL.
///
///              'ABSMAX'   The range rate value is at an absolute
///                         maximum.
///
///              'ABSMIN'   The range rate value is at an absolute
///                         minimum.
///
///              'LOCMAX'   The range rate value is at a local
///                         maximum.
///
///              'LOCMIN'   The range rate value is at a local
///                         minimum.
///
///           RELATE may be used to specify an "adjusted" absolute
///           extremum constraint: this requires the range rate to be
///           within a specified offset relative to an absolute
///           extremum. The argument ADJUST (described below) is used
///           to specify this offset.
///
///           Local extrema are considered to exist only in the
///           interiors of the intervals comprising the confinement
///           window:  a local extremum cannot exist at a boundary
///           point of the confinement window.
///
///           Case and leading or trailing blanks are not
///           significant in the string RELATE.
///
///  REFVAL   is the double precision reference value used together
///           with the argument RELATE to define an equality or
///           inequality to satisfy by the range rate of the
///           observer-target vector. See the discussion of RELATE
///           above for further information.
///
///           The units of REFVAL are km/s.
///
///  ADJUST   is a double precision value used to modify searches for
///           absolute extrema: when RELATE is set to 'ABSMAX' or
///           'ABSMIN' and ADJUST is set to a positive value, GFRR
///           finds times when the range rate is within ADJUST
///           kilometers/second of the specified extreme value.
///
///           For RELATE set to 'ABSMAX', the RESULT window contains
///           time intervals when the range rate has
///           values between ABSMAX - ADJUST and ABSMAX.
///
///           For RELATE set to 'ABSMIN', the RESULT window contains
///           time intervals when the range rate has
///           values between ABSMIN and ABSMIN + ADJUST.
///
///           ADJUST is not used for searches for local extrema,
///           equality or inequality conditions.
///
///  STEP     is the double precision time step size to use in the
///           search.
///
///           STEP must be short enough for a search using this step
///           size to locate the time intervals where the range rate
///           function is monotone increasing or decreasing. However,
///           STEP must not be *too* short, or the search will take an
///           unreasonable amount of time.
///
///           The choice of STEP affects the completeness but not
///           the precision of solutions found by this routine; the
///           precision is controlled by the convergence tolerance.
///           See the discussion of the parameter CNVTOL for
///           details.
///
///           STEP has units of TDB seconds.
///
///  CNFINE   is a double precision SPICE window that confines the time
///           period over which the specified search is conducted.
///           CNFINE may consist of a single interval or a collection
///           of intervals.
///
///           In some cases the confinement window can be used to
///           greatly reduce the time period that must be searched
///           for the desired solution. See the $Particulars section
///           below for further discussion.
///
///           See the $Examples section below for a code example
///           that shows how to create a confinement window.
///
///           CNFINE must be initialized by the caller using the
///           SPICELIB routine SSIZED.
///
///           In some cases the observer's state may be computed at
///           times outside of CNFINE by as much as 2 seconds. See
///           $Particulars for details.
///
///  MW       is a parameter specifying the length of the SPICE
///           windows in the workspace array WORK (see description
///           below) used by this routine.
///
///           MW should be set to a number at least twice as large
///           as the maximum number of intervals required by any
///           workspace window. In many cases, it's not necessary to
///           compute an accurate estimate of how many intervals are
///           needed; rather, the user can pick a size considerably
///           larger than what's really required.
///
///           However, since excessively large arrays can prevent
///           applications from compiling, linking, or running
///           properly, sometimes MW must be set according to
///           the actual workspace requirement. A rule of thumb
///           for the number of intervals NINTVLS needed is
///
///               NINTVLS  =  2*N  +  ( M / STEP )
///
///           where
///
///               N     is the number of intervals in the confinement
///                     window
///
///               M     is the measure of the confinement window, in
///                     units of seconds
///
///               STEP  is the search step size in seconds
///
///           MW should then be set to
///
///               2 * NINTVLS
///
///  NW       is a parameter specifying the number of SPICE windows
///           in the workspace array WORK (see description below)
///           used by this routine. NW should be set to the
///           parameter NWRR; this parameter is declared in the
///           include file gf.inc. (The reason this dimension is
///           an input argument is that this allows run-time
///           error checking to be performed.)
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
///           discarded before GFRR conducts its search.
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
///              DOUBLE PRECISION    WORK ( LBCELL : MW, NWRR )
///
///           where MW is a constant declared by the caller and NWRR is
///           a constant defined in the SPICELIB INCLUDE file gf.inc.
///           See the discussion of MW above.
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
///  LBCELL   is the integer value defining the lower bound for
///           SPICE Cell arrays (a SPICE window is a kind of cell).
///
///  CNVTOL   is the convergence tolerance used for finding
///           endpoints of the intervals comprising the result
///           window. CNVTOL is also used for finding intermediate
///           results; in particular, CNVTOL is used for finding the
///           windows on which the range rate is increasing
///           or decreasing. CNVTOL is used to determine when binary
///           searches for roots should terminate: when a root is
///           bracketed within an interval of length CNVTOL; the
///           root is considered to have been found.
///
///           The accuracy, as opposed to precision, of roots found
///           by this routine depends on the accuracy of the input
///           data. In most cases, the accuracy of solutions will be
///           inferior to their precision.
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
///         - truncation error in time values
///         - finite tolerance value
///         - errors in computed geometric quantities
///
///      it is *normal* for the condition of interest to not always be
///      satisfied near the endpoints of the intervals comprising the
///      RESULT window. One technique to handle such a situation,
///      slightly contract RESULT using the window routine WNCOND.
///
///  3)  If the workspace window size MW is less than 2 or not an even
///      value, the error SPICE(INVALIDDIMENSION) is signaled.
///
///  4)  If the size of the workspace WORK is too small, an error is
///      signaled by a routine in the call tree of this routine.
///
///  5)  If the size of the SPICE window RESULT is less than 2 or not
///      an even value, the error SPICE(INVALIDDIMENSION) is signaled.
///
///  6)  If the SPICE window RESULT has insufficient capacity to
///      contain the number of intervals on which the specified
///      distance condition is met, an error is signaled by a routine
///      in the call tree of this routine.
///
///  7)  If the window count NW is less than NWRR, the error
///      SPICE(INVALIDDIMENSION) is signaled.
///
///  8)  If an error (typically cell overflow) occurs during
///      window arithmetic, the error is signaled by a routine
///      in the call tree of this routine.
///
///  9)  If the relational operator RELATE is not recognized, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  10) If the aberration correction specifier contains an
///      unrecognized value, an error is signaled by a routine in the
///      call tree of this routine.
///
///  11) If ADJUST is negative, an error is signaled by a routine in
///      the call tree of this routine.
///
///  12) If ADJUST has a non-zero value when RELATE has any value other
///      than 'ABSMIN' or 'ABSMAX', an error is signaled by a routine
///      in the call tree of this routine.
///
///  13) If either of the input body names do not map to NAIF ID
///      codes, an error is signaled by a routine in the call tree of
///      this routine.
///
///  14) If required ephemerides or other kernel data are not
///      available, an error is signaled by a routine in the call tree
///      of this routine.
/// ```
///
/// # Files
///
/// ```text
///  Appropriate SPK and PCK kernels must be loaded by the calling
///  program before this routine is called.
///
///  The following data are required:
///
///  -  SPK data: the calling application must load ephemeris data
///     for the targets, observer, and any intermediate objects in
///     a chain connecting the targets and observer that cover the
///     time period specified by the window CNFINE. If aberration
///     corrections are used, the states of target and observer
///     relative to the solar system barycenter must be calculable
///     from the available ephemeris data. Typically ephemeris data
///     are made available by loading one or more SPK files using
///     FURNSH.
///
///  -  In some cases the observer's state may be computed at times
///     outside of CNFINE by as much as 2 seconds; data required to
///     compute this state must be provided by loaded kernels. See
///     $Particulars for details.
///
///  Kernel data are normally loaded once per program run, NOT every
///  time this routine is called.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine determines if the caller-specified constraint
///  condition on the geometric event (range rate) is satisfied for
///  any time intervals within the confinement window CNFINE. If one
///  or more such time intervals exist, those intervals are added
///  to the RESULT window.
///
///  This routine provides a simpler, but less flexible interface
///  than does the routine GFEVNT for conducting searches for
///  observer-target range rate value events. Applications that
///  require support for progress reporting, interrupt handling,
///  non-default step or refinement functions, or non-default
///  convergence tolerance should call GFEVNT rather than this routine.
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
///  periods, within the confinement window, over which the
///  range rate function is monotone increasing and monotone
///  decreasing. Each of these time periods is represented by a SPICE
///  window. Having found these windows, all of the range rate
///  function's local extrema within the confinement window are known.
///  Absolute extrema then can be found very easily.
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
///  The monotone windows (described above) are found using a two-step
///  search process. Each interval of the confinement window is
///  searched as follows: first, the input step size is used to
///  determine the time separation at which the sign of the rate of
///  change of range rate will be sampled. Starting at
///  the left endpoint of an interval, samples will be taken at each
///  step. If a change of sign is found, a root has been bracketed; at
///  that point, the time at which the time derivative of the
///  range rate is zero can be found by a refinement process, for
///  example, using a binary search.
///
///  Note that the optimal choice of step size depends on the lengths
///  of the intervals over which the range rate function is monotone:
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
///  As described above, the root-finding process used by this routine
///  involves first bracketing roots and then using a search process
///  to locate them. "Roots" are both times when local extrema are
///  attained and times when the range rate function is equal to a
///  reference value. All endpoints of the intervals comprising the
///  result window are either endpoints of intervals of the
///  confinement window or roots.
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
///  1) Determine the time windows from January 1, 2007 UTC to
///     April 1, 2007 UTC for which the sun-moon range rate satisfies
///     the relation conditions with respect to a reference value of
///     0.3365 km/s radians (this range rate known to occur within the
///     search interval). Also determine the time windows corresponding
///     to the local maximum and minimum range rate, and the absolute
///     maximum and minimum range rate during the search interval.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: gfrr_ex1.tm
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
///           pck00009.tpc                  Planet orientation and
///                                         radii
///           naif0009.tls                  Leapseconds
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de421.bsp',
///                               'pck00009.tpc',
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
///           PROGRAM GFRR_EX1
///           IMPLICIT NONE
///
///     C
///     C     Include GF parameter declarations:
///     C
///           INCLUDE 'gf.inc'
///
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      DVNORM
///           DOUBLE PRECISION      SPD
///
///           INTEGER               WNCARD
///
///     C
///     C     Local parameters
///     C
///           INTEGER               LBCELL
///           PARAMETER           ( LBCELL = -5 )
///
///           CHARACTER*(*)         TIMFMT
///           PARAMETER           ( TIMFMT =
///          .   'YYYY-MON-DD HR:MN:SC.###' )
///
///     C
///     C     Use the parameter MAXWIN for both the result window size
///     C     and the workspace size.
///     C
///           INTEGER               MAXWIN
///           PARAMETER           ( MAXWIN = 20000 )
///
///     C
///     C     Length of strings:
///     C
///           INTEGER               TIMLEN
///           PARAMETER           ( TIMLEN = 26 )
///
///           INTEGER               NLOOPS
///           PARAMETER           ( NLOOPS = 7 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(TIMLEN)    TIMSTR
///           CHARACTER*(TIMLEN)    RELATE (NLOOPS)
///
///           DOUBLE PRECISION      ADJUST
///           DOUBLE PRECISION      CNFINE ( LBCELL : 2 )
///           DOUBLE PRECISION      DRDT
///           DOUBLE PRECISION      ET0
///           DOUBLE PRECISION      ET1
///           DOUBLE PRECISION      FINISH
///           DOUBLE PRECISION      LT
///           DOUBLE PRECISION      POS    ( 6 )
///           DOUBLE PRECISION      REFVAL
///           DOUBLE PRECISION      RESULT ( LBCELL : MAXWIN )
///           DOUBLE PRECISION      START
///           DOUBLE PRECISION      STEP
///           DOUBLE PRECISION      WORK   ( LBCELL : MAXWIN, NWRR )
///
///           INTEGER               I
///           INTEGER               J
///
///     C
///     C     Saved variables
///     C
///     C     The confinement, workspace and result windows CNFINE,
///     C     WORK and RESULT are saved because this practice helps to
///     C     prevent stack overflow.
///     C
///           SAVE                  CNFINE
///           SAVE                  RESULT
///           SAVE                  WORK
///
///           DATA                  RELATE / '=',
///          .                               '<',
///          .                               '>',
///          .                               'LOCMIN',
///          .                               'ABSMIN',
///          .                               'LOCMAX',
///          .                               'ABSMAX'  /
///
///     C
///     C     Load kernels.
///     C
///           CALL FURNSH ( 'gfrr_ex1.tm' )
///
///     C
///     C     Initialize windows.
///     C
///           CALL SSIZED ( MAXWIN, RESULT )
///           CALL SSIZED ( 2,      CNFINE )
///
///     C
///     C     Store the time bounds of our search interval in
///     C     the confinement window.
///     C
///           CALL STR2ET ( '2007 JAN 1', ET0 )
///           CALL STR2ET ( '2007 APR 1', ET1 )
///
///           CALL WNINSD ( ET0, ET1, CNFINE )
///
///     C
///     C     Search using a step size of 1 day (in units of seconds).
///     C     The reference value is .3365 km/s. We're not using the
///     C     adjustment feature, so we set ADJUST to zero.
///     C
///           STEP   = SPD()
///           REFVAL = .3365D0
///           ADJUST = 0.D0
///
///           DO J=1, NLOOPS
///
///              WRITE(*,*) 'Relation condition: ', RELATE(J)
///
///     C
///     C        Perform the search. The SPICE window RESULT contains
///     C        the set of times when the condition is met.
///     C
///              CALL GFRR (  'MOON', 'NONE', 'SUN', RELATE(J),
///          .                 REFVAL, ADJUST, STEP,    CNFINE,
///          .                 MAXWIN, NWRR,   WORK,    RESULT )
///
///     C
///     C        Display the results.
///     C
///              IF ( WNCARD(RESULT) .EQ. 0 ) THEN
///
///                 WRITE (*, '(A)') 'Result window is empty.'
///
///              ELSE
///
///                 DO I = 1, WNCARD(RESULT)
///
///     C
///     C              Fetch the endpoints of the Ith interval
///     C              of the result window.
///     C
///                    CALL WNFETD ( RESULT, I, START, FINISH )
///
///                    CALL SPKEZR ( 'MOON',  START, 'J2000', 'NONE',
///          .                       'SUN', POS,   LT              )
///                    DRDT = DVNORM(POS)
///
///                    CALL TIMOUT ( START, TIMFMT, TIMSTR )
///
///                    WRITE (*, '(A,F16.9)' ) 'Start time, drdt = '//
///          .                                 TIMSTR, DRDT
///
///                    CALL SPKEZR ( 'MOON',  FINISH, 'J2000', 'NONE',
///          .                       'SUN', POS,     LT              )
///                    DRDT = DVNORM(POS)
///
///                    CALL TIMOUT ( FINISH, TIMFMT, TIMSTR )
///
///                    WRITE (*, '(A,F16.9)' ) 'Stop time,  drdt = '//
///          .                              TIMSTR, DRDT
///                 END DO
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
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Relation condition: =
///     Start time, drdt = 2007-JAN-02 00:35:19.571       0.336500000
///     Stop time,  drdt = 2007-JAN-02 00:35:19.571       0.336500000
///     Start time, drdt = 2007-JAN-19 22:04:54.897       0.336500000
///     Stop time,  drdt = 2007-JAN-19 22:04:54.897       0.336500000
///     Start time, drdt = 2007-FEB-01 23:30:13.427       0.336500000
///     Stop time,  drdt = 2007-FEB-01 23:30:13.427       0.336500000
///     Start time, drdt = 2007-FEB-17 11:10:46.538       0.336500000
///     Stop time,  drdt = 2007-FEB-17 11:10:46.538       0.336500000
///     Start time, drdt = 2007-MAR-04 15:50:19.929       0.336500000
///     Stop time,  drdt = 2007-MAR-04 15:50:19.929       0.336500000
///     Start time, drdt = 2007-MAR-18 09:59:05.957       0.336500000
///     Stop time,  drdt = 2007-MAR-18 09:59:05.957       0.336500000
///
///      Relation condition: <
///     Start time, drdt = 2007-JAN-02 00:35:19.571       0.336500000
///     Stop time,  drdt = 2007-JAN-19 22:04:54.897       0.336500000
///     Start time, drdt = 2007-FEB-01 23:30:13.427       0.336500000
///     Stop time,  drdt = 2007-FEB-17 11:10:46.538       0.336500000
///     Start time, drdt = 2007-MAR-04 15:50:19.929       0.336500000
///     Stop time,  drdt = 2007-MAR-18 09:59:05.957       0.336500000
///
///      Relation condition: >
///     Start time, drdt = 2007-JAN-01 00:00:00.000       0.515522361
///     Stop time,  drdt = 2007-JAN-02 00:35:19.571       0.336500000
///     Start time, drdt = 2007-JAN-19 22:04:54.897       0.336500000
///     Stop time,  drdt = 2007-FEB-01 23:30:13.427       0.336500000
///     Start time, drdt = 2007-FEB-17 11:10:46.538       0.336500000
///     Stop time,  drdt = 2007-MAR-04 15:50:19.929       0.336500000
///     Start time, drdt = 2007-MAR-18 09:59:05.957       0.336500000
///     Stop time,  drdt = 2007-APR-01 00:00:00.000       0.793546220
///
///      Relation condition: LOCMIN
///     Start time, drdt = 2007-JAN-11 07:03:58.991      -0.803382745
///     Stop time,  drdt = 2007-JAN-11 07:03:58.991      -0.803382745
///     Start time, drdt = 2007-FEB-10 06:26:15.441      -0.575837627
///     Stop time,  drdt = 2007-FEB-10 06:26:15.441      -0.575837627
///     Start time, drdt = 2007-MAR-12 03:28:36.404      -0.441800451
///     Stop time,  drdt = 2007-MAR-12 03:28:36.404      -0.441800451
///
///      Relation condition: ABSMIN
///     Start time, drdt = 2007-JAN-11 07:03:58.991      -0.803382745
///     Stop time,  drdt = 2007-JAN-11 07:03:58.991      -0.803382745
///
///      Relation condition: LOCMAX
///     Start time, drdt = 2007-JAN-26 02:27:33.762       1.154648992
///     Stop time,  drdt = 2007-JAN-26 02:27:33.762       1.154648992
///     Start time, drdt = 2007-FEB-24 09:35:07.812       1.347132236
///     Stop time,  drdt = 2007-FEB-24 09:35:07.812       1.347132236
///     Start time, drdt = 2007-MAR-25 17:26:56.148       1.428141706
///     Stop time,  drdt = 2007-MAR-25 17:26:56.148       1.428141706
///
///      Relation condition: ABSMAX
///     Start time, drdt = 2007-MAR-25 17:26:56.148       1.428141706
///     Stop time,  drdt = 2007-MAR-25 17:26:56.148       1.428141706
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
///      range rate quantity utility package. Callers may themselves
///      need to re-initialize the range rate quantity utility
///      package after calling this routine.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.1, 27-OCT-2021 (JDR) (NJB)
///
///         Edited the header to comply with NAIF standard.
///
///         Modified code example to use "TIMFMT" to provide the format to
///         TIMOUT. Added SAVE statements for CNFINE, WORK and RESULT
///         variables in code example.
///
///         Updated description of WORK and RESULT arguments in $Brief_I/O,
///         $Detailed_Input and $Detailed_Output.
///
///         Added entry #10 in $Exceptions section.
///
///         Updated header to describe use of expanded confinement window.
///
/// -    SPICELIB Version 1.1.0, 05-SEP-2012 (EDW)
///
///         Edit to comments to correct search description.
///
///         Implemented use of ZZHOLDD to allow user to alter convergence
///         tolerance.
///
///         Removed the STEP > 0 error check. The GFSSTP call includes
///         the check.
///
///         Edits to Example section, proper description of "standard.tm"
///         meta kernel.
///
/// -    SPICELIB Version 1.0.0, 24-JUN-2009 (EDW)
/// ```
pub fn gfrr(
    ctx: &mut SpiceContext,
    target: &str,
    abcorr: &str,
    obsrvr: &str,
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
    GFRR(
        target.as_bytes(),
        abcorr.as_bytes(),
        obsrvr.as_bytes(),
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

//$Procedure GFRR ( GF, range rate search )
pub fn GFRR(
    TARGET: &[u8],
    ABCORR: &[u8],
    OBSRVR: &[u8],
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
    let CNFINE = DummyArray::new(CNFINE, LBCELL..);
    let mut WORK = DummyArrayMut2D::new(WORK, LBCELL..=MW, 1..=NW);
    let mut RESULT = DummyArrayMut::new(RESULT, LBCELL..);
    let mut TOL: f64 = 0.0;
    let mut OK: bool = false;
    let mut QCPARS = ActualCharArray::new(LNSIZE, 1..=QNPARS);
    let mut QPNAMS = ActualCharArray::new(LNSIZE, 1..=QNPARS);
    let QDPARS = StackArray::<f64, 3>::new(1..=QNPARS);
    let QIPARS = StackArray::<i32, 3>::new(1..=QNPARS);
    let QLPARS = StackArray::<bool, 3>::new(1..=QNPARS);

    //
    // SPICELIB functions
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
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // Check into the error subsystem.
    //
    CHKIN(b"GFRR", ctx)?;

    //
    // Confirm minimum window sizes.
    //
    if ((MW < 2) || ODD(MW)) {
        SETMSG(
            b"Workspace window size was #; size must be at least 2 and an even value.",
            ctx,
        );
        ERRINT(b"#", MW, ctx);
        SIGERR(b"SPICE(INVALIDDIMENSION)", ctx)?;
        CHKOUT(b"GFRR", ctx)?;
        return Ok(());
    }

    if (NW < NWRR) {
        SETMSG(
            b"Workspace window count was #; count must be at least #.",
            ctx,
        );
        ERRINT(b"#", NW, ctx);
        ERRINT(b"#", NWRR, ctx);
        SIGERR(b"SPICE(INVALIDDIMENSION)", ctx)?;
        CHKOUT(b"GFRR", ctx)?;
        return Ok(());
    }

    //
    // Check the result window size.
    //
    if ((SIZED(RESULT.as_slice(), ctx)? < 2) || ODD(SIZED(RESULT.as_slice(), ctx)?)) {
        SETMSG(
            b"Result window size was #; size must be at least 2 and an even value.",
            ctx,
        );
        ERRINT(b"#", SIZED(RESULT.as_slice(), ctx)?, ctx);
        SIGERR(b"SPICE(INVALIDDIMENSION)", ctx)?;
        CHKOUT(b"GFRR", ctx)?;
        return Ok(());
    }

    //
    // Set up a call to GFEVNT specific to the range rate search.
    //
    fstr::assign(QPNAMS.get_mut(1), b"TARGET");
    fstr::assign(QCPARS.get_mut(1), TARGET);

    fstr::assign(QPNAMS.get_mut(2), b"OBSERVER");
    fstr::assign(QCPARS.get_mut(2), OBSRVR);

    fstr::assign(QPNAMS.get_mut(3), b"ABCORR");
    fstr::assign(QCPARS.get_mut(3), ABCORR);

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
    // Initialize the RESULT window to empty.
    //
    SCARDD(0, RESULT.as_slice_mut(), ctx)?;

    //
    // Look for solutions.
    //
    // Progress report and interrupt options are set to .FALSE.
    //
    GFEVNT(
        GFSTEP,
        GFREFN,
        b"RANGE RATE",
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
        NWRR,
        WORK.as_slice_mut(),
        NOBAIL,
        GFBAIL,
        RESULT.as_slice_mut(),
        ctx,
    )?;

    CHKOUT(b"GFRR", ctx)?;
    Ok(())
}
