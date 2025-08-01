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
const QNPARS: i32 = 10;
const NOBAIL: bool = false;
const NORPT: bool = false;

struct SaveVars {
    DREF: Vec<u8>,
    DVEC: StackArray<f64, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut DREF = vec![b' '; LNSIZE as usize];
        let mut DVEC = StackArray::<f64, 3>::new(1..=3);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(0.0)].into_iter();
            DVEC.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        fstr::assign(&mut DREF, b" ");

        Self { DREF, DVEC }
    }
}

/// GF, observer-target vector coordinate search
///
/// Determine time intervals for which a coordinate of an
/// observer-target position vector satisfies a numerical constraint.
///
/// # Required Reading
///
/// * [GF](crate::required_reading::gf)
/// * [SPK](crate::required_reading::spk)
/// * [CK](crate::required_reading::ck)
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
///  FRAME      I   Name of the reference frame for coordinate
///                 calculations.
///  ABCORR     I   Aberration correction flag.
///  OBSRVR     I   Name of the observing body.
///  CRDSYS     I   Name of the coordinate system containing COORD.
///  COORD      I   Name of the coordinate of interest.
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
///           The target and observer define a position vector
///           that points from the observer to the target.
///
///  FRAME    is the name of the reference frame in which to perform
///           state look-ups and coordinate calculations.
///
///           The SPICE frame subsystem must recognize the FRAME
///           name.
///
///  ABCORR   is the description of the aberration corrections
///           to apply to the state evaluations to account for one-way
///           light time and stellar aberration.
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
///           The ABCORR string lacks sensitivity to case, leading
///           and trailing blanks.
///
///  OBSRVR   is the name of an observing body. Optionally, you may
///           supply the ID code of the object as an integer string.
///           For example, both 'EARTH' and '399' are legitimate
///           strings to indicate the observer is the Earth.
///
///  CRDSYS   is the name of the coordinate system for which the
///           coordinate of interest is a member.
///
///  COORD    is the string name of the coordinate of interest in
///           CRDSYS.
///
///           The supported coordinate systems and coordinate names:
///
///              CRDSYS             COORD               Range
///              ----------------   -----------------   ------------
///              'RECTANGULAR'      'X'
///                                 'Y'
///                                 'Z'
///
///              'LATITUDINAL'      'RADIUS'
///                                 'LONGITUDE'         (-Pi,Pi]
///                                 'LATITUDE'          [-Pi/2,Pi/2]
///
///              'RA/DEC'           'RANGE'
///                                 'RIGHT ASCENSION'   [0,2Pi)
///                                 'DECLINATION'       [-Pi/2,Pi/2]
///
///              'SPHERICAL'        'RADIUS'
///                                 'COLATITUDE'        [0,Pi]
///                                 'LONGITUDE'         (-Pi,Pi]
///
///              'CYLINDRICAL'      'RADIUS'
///                                 'LONGITUDE'         [0,2Pi)
///                                 'Z'
///
///              'GEODETIC'         'LONGITUDE'         (-Pi,Pi]
///                                 'LATITUDE'          [-Pi/2,Pi/2]
///                                 'ALTITUDE'
///
///              'PLANETOGRAPHIC'   'LONGITUDE'         [0,2Pi)
///                                 'LATITUDE'          [-Pi/2,Pi/2]
///                                 'ALTITUDE'
///
///           The 'ALTITUDE' coordinates have a constant value of
///           zero +/- roundoff for ellipsoid targets.
///
///           Limit searches for coordinate events in the 'GEODETIC'
///           and 'PLANETOGRAPHIC' coordinate systems to TARGET bodies
///           with axial symmetry in the equatorial plane, i.e.
///           equality of the body X and Y radii (oblate or prolate
///           spheroids).
///
///           Searches on 'GEODETIC' or 'PLANETOGRAPHIC' coordinates
///           requires body shape data, and in the case of
///           'PLANETOGRAPHIC' coordinates, body rotation data.
///
///           The body associated with 'GEODETIC' or 'PLANETOGRAPHIC'
///           coordinates is the center of the frame FRAME.
///
///  RELATE   is the relational operator used to define a constraint on
///           the selected coordinate of the observer-target vector.
///           The result window found by this routine indicates the
///           time intervals where the constraint is satisfied.
///           Supported values of RELATE and corresponding meanings are
///           shown below:
///
///              '>'        The coordinate value is greater than the
///                         reference value REFVAL.
///
///              '='        The coordinate value is equal to the
///                         reference value REFVAL.
///
///              '<'        The coordinate value is less than the
///                         reference value REFVAL.
///
///              'ABSMAX'   The coordinate value is at an absolute
///                         maximum.
///
///              'ABSMIN'   The coordinate value is at an absolute
///                         minimum.
///
///              'LOCMAX'   The coordinate value is at a local
///                         maximum.
///
///              'LOCMIN'   The coordinate value is at a local
///                         minimum.
///
///           RELATE may be used to specify an "adjusted" absolute
///           extremum constraint: this requires the quantity to be
///           within a specified offset relative to an absolute
///           extremum. The argument ADJUST (described below) is used
///           to specify this offset.
///
///           Local extrema are considered to exist only in the
///           interiors of the intervals comprising the confinement
///           window:  a local extremum cannot exist at a boundary
///           point of the confinement window.
///
///           The RELATE string lacks sensitivity to case, leading
///           and trailing blanks.
///
///  REFVAL   is the double precision reference value used together
///           with the argument RELATE to define an equality or
///           inequality to satisfy by the selected coordinate of the
///           observer- target vector. See the discussion of RELATE
///           above for further information.
///
///           The units of REFVAL correspond to the type as defined
///           by COORD, radians for angular measures, kilometers for
///           distance measures.
///
///  ADJUST   is a double precision value used to modify searches for
///           absolute extrema: when RELATE is set to 'ABSMAX' or
///           'ABSMIN' and ADJUST is set to a positive value, GFPOSC
///           finds times when the position vector coordinate is within
///           ADJUST radians/kilometers of the specified extreme value.
///
///           For RELATE set to 'ABSMAX', the RESULT window contains
///           time intervals when the position vector coordinate has
///           values between ABSMAX - ADJUST and ABSMAX.
///
///           For RELATE set to 'ABSMIN', the RESULT window contains
///           time intervals when the position vector coordinate has
///           values between ABSMIN and ABSMIN + ADJUST.
///
///           ADJUST is not used for searches for local extrema,
///           equality or inequality conditions.
///
///  STEP     is the double precision time step size to use in the
///           search.
///
///           STEP must be short enough to for a search using this step
///           size to locate the time intervals where coordinate
///           function of the position vector is monotone increasing or
///           decreasing. However, STEP must not be *too* short, or the
///           search will take an unreasonable amount of time.
///
///           For coordinates other than 'LONGITUDE' and 'RIGHT
///           ASCENSION', the step size must be shorter than the
///           shortest interval, within the confinement window, over
///           which the coordinate is monotone increasing or
///           decreasing.
///
///           For 'LONGITUDE' and 'RIGHT ASCENSION', the step size must
///           be shorter than the shortest interval, within the
///           confinement window, over which either the sine or cosine
///           of the coordinate is monotone increasing or decreasing.
///
///           The choice of STEP affects the completeness but not the
///           precision of solutions found by this routine; the
///           precision is controlled by the convergence tolerance. See
///           the discussion of the parameter CNVTOL for details.
///
///           STEP has units of seconds.
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
///           parameter NWMAX; this parameter is declared in the
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
///           discarded before GFPOSC conducts its search.
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
///              DOUBLE PRECISION    WORK ( LBCELL : MW, NWMAX )
///
///           where MW is a constant declared by the caller and
///           NWMAX is a constant defined in the SPICELIB INCLUDE
///           file gf.inc. See the discussion of MW above.
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
///           windows on which the specified coordinate is increasing
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
///      This routine does not diagnose invalid step sizes, except
///      that if the step size is non-positive, an error is signaled
///      by a routine in the call tree of this routine.
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
///  3)  If the window size MW is less than 2 or not an even value,
///      the error SPICE(INVALIDDIMENSION) is signaled.
///
///  4)  If the window size of RESULT is less than 2, the error
///      SPICE(INVALIDDIMENSION) is signaled.
///
///  5)  If the output SPICE window RESULT has insufficient capacity
///      to contain the number of intervals on which the specified
///      distance condition is met, an error is signaled
///      by a routine in the call tree of this routine.
///
///  6)  If an error (typically cell overflow) occurs during
///      window arithmetic, the error is signaled by a routine
///      in the call tree of this routine.
///
///  7)  If the relational operator RELATE is not recognized, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  8)  If the size of the workspace WORK is too small, an error is
///      signaled by a routine in the call tree of this routine.
///
///  9)  If the aberration correction specifier contains an
///      unrecognized value, an error is signaled by a routine in the
///      call tree of this routine.
///
///  10) If ADJUST is negative, an error is signaled by a routine in
///      the call tree of this routine.
///
///  11) If either of the input body names do not map to NAIF ID
///      codes, an error is signaled by a routine in the call tree of
///      this routine.
///
///  12) If required ephemerides or other kernel data are not
///      available, an error is signaled by a routine in the call tree
///      of this routine.
///
///  13) If the search uses GEODETIC or PLANETOGRAPHIC coordinates, and
///      the center body of the reference frame has unequal equatorial
///      radii, an error is signaled by a routine in the call tree of
///      this routine.
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
///  -  If non-inertial reference frames are used, then PCK
///     files, frame kernels, C-kernels, and SCLK kernels may be
///     needed.
///
///  -  In some cases the observer's state may be computed at times
///     outside of CNFINE by as much as 2 seconds; data required to
///     compute this state must be provided by loaded kernels. See
///     $Particulars for details.
///
///  Such kernel data are normally loaded once per program
///  run, NOT every time this routine is called.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine provides a simpler, but less flexible interface
///  than does the routine GFEVNT for conducting searches for
///  observer-target position vector coordinate value events.
///  Applications that require support for progress reporting,
///  interrupt handling, non-default step or refinement functions,
///  or non-default convergence tolerance should call GFEVNT rather
///  than this routine.
///
///  This routine determines a set of one or more time intervals
///  within the confinement window when the selected coordinate of
///  the observer-target position vector satisfies a caller-specified
///  constraint. The resulting set of intervals is returned as a SPICE
///  window.
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
///  coordinate function is monotone increasing and monotone
///  decreasing. Each of these time periods is represented by a SPICE
///  window. Having found these windows, all of the coordinate
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
///  change of coordinate will be sampled. Starting at
///  the left endpoint of an interval, samples will be taken at each
///  step. If a change of sign is found, a root has been bracketed; at
///  that point, the time at which the time derivative of the
///  coordinate is zero can be found by a refinement process, for
///  example, using a binary search.
///
///  Note that the optimal choice of step size depends on the lengths
///  of the intervals over which the coordinate function is monotone:
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
///  attained and times when the coordinate function is equal to a
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
///  Practical use of the coordinate search capability would likely
///  consist of searches over multiple coordinate constraints to find
///  time intervals that satisfies the constraints. An
///  effective technique to accomplish such a search is
///  to use the result window from one search as the confinement window
///  of the next.
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
///
///  Longitude and Right Ascension
///  =============================
///
///  The cyclic nature of the longitude and right ascension coordinates
///  produces branch cuts at +/- 180 degrees longitude and 0-360
///  right ascension. Round-off error may cause solutions near these
///  branches to cross the branch. Use of the SPICE routine WNCOND
///  will contract solution windows by some epsilon, reducing the
///  measure of the windows and eliminating the branch crossing. A
///  one millisecond contraction will in most cases eliminate
///  numerical round-off caused branch crossings.
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
///  1) Find the time during 2007 for which the latitude of the
///     Earth-Sun vector in IAU_EARTH frame has the maximum value,
///     i.e. the latitude of the Tropic of Cancer.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: gfposc_ex1.tm
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
///           PROGRAM GFPOSC_EX1
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
///           DOUBLE PRECISION      SPD
///           DOUBLE PRECISION      RPD
///           INTEGER               WNCARD
///
///     C
///     C     Local parameters
///     C
///           INTEGER               LBCELL
///           PARAMETER           ( LBCELL = -5 )
///
///     C
///     C     Use the parameter MAXWIN for both
///     C     the result window size and the workspace
///     C     size.
///     C
///           INTEGER               MAXWIN
///           PARAMETER           ( MAXWIN = 750 )
///
///     C
///     C     String length.
///     C
///           INTEGER               STRLEN
///           PARAMETER           ( STRLEN = 64 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(STRLEN)    TIMSTR
///           CHARACTER*(STRLEN)    TIMFIN
///           CHARACTER*(STRLEN)    RELATE
///           CHARACTER*(STRLEN)    CRDSYS
///           CHARACTER*(STRLEN)    COORD
///           CHARACTER*(STRLEN)    ABCORR
///           CHARACTER*(STRLEN)    TARG
///           CHARACTER*(STRLEN)    OBSRVR
///           CHARACTER*(STRLEN)    FRAME
///           CHARACTER*(STRLEN)   TIMFMT
///
///
///           DOUBLE PRECISION      ADJUST
///           DOUBLE PRECISION      CNFINE ( LBCELL : 2 )
///           DOUBLE PRECISION      ET0
///           DOUBLE PRECISION      ET1
///           DOUBLE PRECISION      FINISH
///           DOUBLE PRECISION      REFVAL
///           DOUBLE PRECISION      RESULT ( LBCELL : MAXWIN )
///           DOUBLE PRECISION      START
///           DOUBLE PRECISION      STEP
///           DOUBLE PRECISION      WORK   ( LBCELL : MAXWIN, NWMAX )
///
///           INTEGER               I
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
///     C
///     C     Load kernels.
///     C
///           CALL FURNSH ( 'gfposc_ex1.tm' )
///
///     C
///     C     Initialize windows.
///     C
///           CALL SSIZED ( MAXWIN, RESULT )
///           CALL SSIZED ( 2,      CNFINE )
///
///           TIMFMT = 'YYYY-MON-DD HR:MN:SC.###### (TDB) ::TDB ::RND'
///
///     C
///     C     Store the time bounds of our search interval in
///     C     the confinement window.
///     C
///
///           CALL STR2ET ( '2007 JAN 1', ET0 )
///           CALL STR2ET ( '2008 JAN 1', ET1 )
///
///           CALL WNINSD ( ET0, ET1, CNFINE )
///
///     C
///     C     The latitude varies relatively slowly, ~46 degrees
///     C     during the year. The extrema occur approximately every
///     C     six months. Search using a step size less than half
///     C     that value (180 days). For this example use ninety days
///     C     (in units of seconds).
///     C
///           STEP   = SPD() * 90.D0
///           ADJUST = 0.D0
///           REFVAL = 0.D0
///
///     C
///     C     Search for the date on which the CRDSYS system
///     C     coordinate COORD satisfies the RELATE constraint.
///     C
///           RELATE = 'ABSMAX'
///           CRDSYS = 'LATITUDINAL'
///           COORD  = 'LATITUDE'
///           TARG   = 'SUN'
///           OBSRVR = 'EARTH'
///           FRAME  = 'IAU_EARTH'
///           ABCORR = 'NONE'
///
///     C
///     C     Perform this search using the geometric position
///     C     of the bodies; set the aberration correction to 'NONE'.
///     C
///           CALL GFPOSC ( TARG,   FRAME, ABCORR,
///          .              OBSRVR, CRDSYS, COORD,
///          .              RELATE, REFVAL, ADJUST,
///          .              STEP,   CNFINE,  MAXWIN,
///          .              NWMAX,  WORK,   RESULT )
///
///     C
///     C     Display the results.
///     C
///           IF ( WNCARD(RESULT) .EQ. 0 ) THEN
///
///              WRITE (*,*) 'Result window is empty.'
///
///           ELSE
///
///              DO I = 1, WNCARD(RESULT)
///
///     C
///     C           Fetch the endpoints of the Ith interval
///     C           of the result window.
///     C
///                 CALL WNFETD ( RESULT, I, START, FINISH )
///
///                 IF( START .EQ. FINISH ) THEN
///
///     C
///     C              The result window contains singleton
///     C              intervals, so we need display only the
///     C              start times.
///     C
///                    CALL TIMOUT ( START, TIMFMT, TIMSTR )
///                    WRITE (*, *) 'Event time: ', TIMSTR
///
///                 ELSE
///
///                    CALL TIMOUT ( START,  TIMFMT, TIMSTR )
///                    CALL TIMOUT ( FINISH, TIMFMT, TIMFIN )
///
///                    WRITE(*, *) 'From : ', TIMSTR
///                    WRITE(*, *) 'To   : ', TIMFIN
///                    WRITE(*, *) ' '
///
///                 END IF
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
///      Event time: 2007-JUN-21 17:54:13.172475 (TDB)
///
///
///  2) A minor modification of the program listed in Example 1; find
///     the time during 2007 for which the latitude of the Earth-Sun
///     vector in IAU_EARTH frame has the minimum value, i.e. the
///     latitude of the Tropic of Capricorn.
///
///
///     Use the meta-kernel from the first example.
///
///
///     Example code begins here.
///
///
///           PROGRAM GFPOSC_EX2
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
///           DOUBLE PRECISION      SPD
///           DOUBLE PRECISION      RPD
///           INTEGER               WNCARD
///
///     C
///     C     Local parameters
///     C
///           INTEGER               LBCELL
///           PARAMETER           ( LBCELL = -5 )
///
///     C
///     C     Use the parameter MAXWIN for both
///     C     the result window size and the workspace
///     C     size.
///     C
///           INTEGER               MAXWIN
///           PARAMETER           ( MAXWIN = 750 )
///
///     C
///     C     String length.
///     C
///           INTEGER               STRLEN
///           PARAMETER           ( STRLEN = 64 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(STRLEN)    TIMSTR
///           CHARACTER*(STRLEN)    TIMFIN
///           CHARACTER*(STRLEN)    RELATE
///           CHARACTER*(STRLEN)    CRDSYS
///           CHARACTER*(STRLEN)    COORD
///           CHARACTER*(STRLEN)    ABCORR
///           CHARACTER*(STRLEN)    TARG
///           CHARACTER*(STRLEN)    OBSRVR
///           CHARACTER*(STRLEN)    FRAME
///           CHARACTER*(STRLEN)   TIMFMT
///
///
///           DOUBLE PRECISION      ADJUST
///           DOUBLE PRECISION      CNFINE ( LBCELL : 2 )
///           DOUBLE PRECISION      ET0
///           DOUBLE PRECISION      ET1
///           DOUBLE PRECISION      FINISH
///           DOUBLE PRECISION      REFVAL
///           DOUBLE PRECISION      RESULT ( LBCELL : MAXWIN )
///           DOUBLE PRECISION      START
///           DOUBLE PRECISION      STEP
///           DOUBLE PRECISION      WORK   ( LBCELL : MAXWIN, NWMAX )
///
///           INTEGER               I
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
///     C
///     C     Load kernels.
///     C
///           CALL FURNSH ( 'gfposc_ex1.tm' )
///
///     C
///     C     Initialize windows.
///     C
///           CALL SSIZED ( MAXWIN, RESULT )
///           CALL SSIZED ( 2,      CNFINE )
///
///           TIMFMT = 'YYYY-MON-DD HR:MN:SC.###### (TDB) ::TDB ::RND'
///
///     C
///     C     Store the time bounds of our search interval in
///     C     the confinement window.
///     C
///
///           CALL STR2ET ( '2007 JAN 1', ET0 )
///           CALL STR2ET ( '2008 JAN 1', ET1 )
///
///           CALL WNINSD ( ET0, ET1, CNFINE )
///
///     C
///     C     The latitude varies relatively slowly, ~46 degrees
///     C     during the year. The extrema occur approximately every
///     C     six months. Search using a step size less than half
///     C     that value (180 days). For this example use ninety days
///     C     (in units of seconds).
///     C
///           STEP   = SPD() * 90.D0
///           ADJUST = 0.D0
///           REFVAL = 0.D0
///
///     C
///     C     Search for the date on which the CRDSYS system
///     C     coordinate COORD satisfies the RELATE constraint.
///     C
///           RELATE = 'ABSMIN'
///           CRDSYS = 'LATITUDINAL'
///           COORD  = 'LATITUDE'
///           TARG   = 'SUN'
///           OBSRVR = 'EARTH'
///           FRAME  = 'IAU_EARTH'
///           ABCORR = 'NONE'
///
///     C
///     C     Perform this search using the geometric position
///     C     of the bodies; set the aberration correction to 'NONE'.
///     C
///           CALL GFPOSC ( TARG,   FRAME, ABCORR,
///          .              OBSRVR, CRDSYS, COORD,
///          .              RELATE, REFVAL, ADJUST,
///          .              STEP,   CNFINE,  MAXWIN,
///          .              NWMAX,  WORK,   RESULT )
///
///     C
///     C     Display the results.
///     C
///           IF ( WNCARD(RESULT) .EQ. 0 ) THEN
///
///              WRITE (*,*) 'Result window is empty.'
///
///           ELSE
///
///              DO I = 1, WNCARD(RESULT)
///
///     C
///     C           Fetch the endpoints of the Ith interval
///     C           of the result window.
///     C
///                 CALL WNFETD ( RESULT, I, START, FINISH )
///
///                 IF( START .EQ. FINISH ) THEN
///
///     C
///     C              The result window contains singleton
///     C              intervals, so we need display only the
///     C              start times.
///     C
///                    CALL TIMOUT ( START, TIMFMT, TIMSTR )
///                    WRITE (*, *) 'Event time: ', TIMSTR
///
///                 ELSE
///
///                    CALL TIMOUT ( START,  TIMFMT, TIMSTR )
///                    CALL TIMOUT ( FINISH, TIMFMT, TIMFIN )
///
///                    WRITE(*, *) 'From : ', TIMSTR
///                    WRITE(*, *) 'To   : ', TIMFIN
///                    WRITE(*, *) ' '
///
///                 END IF
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
///      Event time: 2007-DEC-22 06:04:32.635539 (TDB)
///
///
///  3) Find the time during 2007 for which the Z component of the
///     Earth-Sun vector in IAU_EARTH frame has value 0, i.e. crosses
///     the equatorial plane (this also defines a zero latitude).
///     The search should return two times, one for an ascending
///     passage and one for descending.
///
///     Use the meta-kernel from the first example.
///
///
///     Example code begins here.
///
///
///           PROGRAM GFPOSC_EX3
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
///           DOUBLE PRECISION      SPD
///           DOUBLE PRECISION      RPD
///           INTEGER               WNCARD
///
///     C
///     C     Local parameters
///     C
///           INTEGER               LBCELL
///           PARAMETER           ( LBCELL = -5 )
///
///     C
///     C     Use the parameter MAXWIN for both
///     C     the result window size and the workspace
///     C     size.
///     C
///           INTEGER               MAXWIN
///           PARAMETER           ( MAXWIN = 750 )
///
///     C
///     C     String length.
///     C
///           INTEGER               STRLEN
///           PARAMETER           ( STRLEN = 64 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(STRLEN)    TIMSTR
///           CHARACTER*(STRLEN)    TIMFIN
///           CHARACTER*(STRLEN)    RELATE
///           CHARACTER*(STRLEN)    CRDSYS
///           CHARACTER*(STRLEN)    COORD
///           CHARACTER*(STRLEN)    ABCORR
///           CHARACTER*(STRLEN)    TARG
///           CHARACTER*(STRLEN)    OBSRVR
///           CHARACTER*(STRLEN)    FRAME
///           CHARACTER*(STRLEN)   TIMFMT
///
///
///           DOUBLE PRECISION      ADJUST
///           DOUBLE PRECISION      CNFINE ( LBCELL : 2 )
///           DOUBLE PRECISION      ET0
///           DOUBLE PRECISION      ET1
///           DOUBLE PRECISION      FINISH
///           DOUBLE PRECISION      REFVAL
///           DOUBLE PRECISION      RESULT ( LBCELL : MAXWIN )
///           DOUBLE PRECISION      START
///           DOUBLE PRECISION      STEP
///           DOUBLE PRECISION      WORK   ( LBCELL : MAXWIN, NWMAX )
///
///           INTEGER               I
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
///     C
///     C     Load kernels.
///     C
///           CALL FURNSH ( 'gfposc_ex1.tm' )
///
///     C
///     C     Initialize windows.
///     C
///           CALL SSIZED ( MAXWIN, RESULT )
///           CALL SSIZED ( 2,      CNFINE )
///
///           TIMFMT = 'YYYY-MON-DD HR:MN:SC.###### (TDB) ::TDB ::RND'
///
///     C
///     C     Store the time bounds of our search interval in
///     C     the confinement window.
///     C
///
///           CALL STR2ET ( '2007 JAN 1', ET0 )
///           CALL STR2ET ( '2008 JAN 1', ET1 )
///
///           CALL WNINSD ( ET0, ET1, CNFINE )
///
///     C
///     C     The latitude varies relatively slowly, ~46 degrees
///     C     during the year. The extrema occur approximately every
///     C     six months. Search using a step size less than half
///     C     that value (180 days). For this example use ninety days
///     C     (in units of seconds).
///     C
///           STEP   = SPD() * 90.D0
///           ADJUST = 0.D0
///           REFVAL = 0.D0
///
///     C
///     C     Search for the date on which the CRDSYS system
///     C     coordinate COORD satisfies the RELATE constraint.
///     C
///           RELATE = '='
///           CRDSYS = 'RECTANGULAR'
///           COORD  = 'Z'
///           TARG   = 'SUN'
///           OBSRVR = 'EARTH'
///           FRAME  = 'IAU_EARTH'
///           ABCORR = 'NONE'
///
///     C
///     C     Perform this search using the geometric position
///     C     of the bodies; set the aberration correction to 'NONE'.
///     C
///           CALL GFPOSC ( TARG,   FRAME, ABCORR,
///          .              OBSRVR, CRDSYS, COORD,
///          .              RELATE, REFVAL, ADJUST,
///          .              STEP,   CNFINE,  MAXWIN,
///          .              NWMAX,  WORK,   RESULT )
///
///     C
///     C     Display the results.
///     C
///           IF ( WNCARD(RESULT) .EQ. 0 ) THEN
///
///              WRITE (*,*) 'Result window is empty.'
///
///           ELSE
///
///              DO I = 1, WNCARD(RESULT)
///
///     C
///     C           Fetch the endpoints of the Ith interval
///     C           of the result window.
///     C
///                 CALL WNFETD ( RESULT, I, START, FINISH )
///
///                 IF( START .EQ. FINISH ) THEN
///
///     C
///     C              The result window contains singleton
///     C              intervals, so we need display only the
///     C              start times.
///     C
///                    CALL TIMOUT ( START, TIMFMT, TIMSTR )
///                    WRITE (*, *) 'Event time: ', TIMSTR
///
///                 ELSE
///
///                    CALL TIMOUT ( START,  TIMFMT, TIMSTR )
///                    CALL TIMOUT ( FINISH, TIMFMT, TIMFIN )
///
///                    WRITE(*, *) 'From : ', TIMSTR
///                    WRITE(*, *) 'To   : ', TIMFIN
///                    WRITE(*, *) ' '
///
///                 END IF
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
///      Event time: 2007-MAR-21 00:01:25.500673 (TDB)
///      Event time: 2007-SEP-23 09:46:39.579484 (TDB)
///
///
///  4) Find the times between Jan 1, 2007 and Jan 1, 2008
///     corresponding to the apoapsis on the Moon's orbit around the
///     Earth (note, the GFDIST routine can also perform this search).
///
///     Use the meta-kernel from the first example.
///
///
///     Example code begins here.
///
///
///           PROGRAM GFPOSC_EX4
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
///           DOUBLE PRECISION      SPD
///           DOUBLE PRECISION      RPD
///           INTEGER               WNCARD
///
///     C
///     C     Local parameters
///     C
///           INTEGER               LBCELL
///           PARAMETER           ( LBCELL = -5 )
///
///     C
///     C     Use the parameter MAXWIN for both
///     C     the result window size and the workspace
///     C     size.
///     C
///           INTEGER               MAXWIN
///           PARAMETER           ( MAXWIN = 750 )
///
///     C
///     C     String length.
///     C
///           INTEGER               STRLEN
///           PARAMETER           ( STRLEN = 64 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(STRLEN)    TIMSTR
///           CHARACTER*(STRLEN)    TIMFIN
///           CHARACTER*(STRLEN)    RELATE
///           CHARACTER*(STRLEN)    CRDSYS
///           CHARACTER*(STRLEN)    COORD
///           CHARACTER*(STRLEN)    ABCORR
///           CHARACTER*(STRLEN)    TARG
///           CHARACTER*(STRLEN)    OBSRVR
///           CHARACTER*(STRLEN)    FRAME
///           CHARACTER*(STRLEN)   TIMFMT
///
///
///           DOUBLE PRECISION      ADJUST
///           DOUBLE PRECISION      CNFINE ( LBCELL : 2 )
///           DOUBLE PRECISION      ET0
///           DOUBLE PRECISION      ET1
///           DOUBLE PRECISION      FINISH
///           DOUBLE PRECISION      REFVAL
///           DOUBLE PRECISION      RESULT ( LBCELL : MAXWIN )
///           DOUBLE PRECISION      START
///           DOUBLE PRECISION      STEP
///           DOUBLE PRECISION      WORK   ( LBCELL : MAXWIN, NWMAX )
///
///           INTEGER               I
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
///     C
///     C     Load kernels.
///     C
///           CALL FURNSH ( 'gfposc_ex1.tm' )
///
///     C
///     C     Initialize windows.
///     C
///           CALL SSIZED ( MAXWIN, RESULT )
///           CALL SSIZED ( 2,      CNFINE )
///
///           TIMFMT = 'YYYY-MON-DD HR:MN:SC.###### (TDB) ::TDB ::RND'
///
///     C
///     C     Store the time bounds of our search interval in
///     C     the confinement window.
///     C
///
///           CALL STR2ET ( '2007 JAN 1', ET0 )
///           CALL STR2ET ( '2008 JAN 1', ET1 )
///
///           CALL WNINSD ( ET0, ET1, CNFINE )
///
///     C
///     C     This search requires a change in the step size since the
///     C     Moon's orbit about the earth (earth-moon barycenter) has
///     C     a twenty-eight day period. Use a step size something
///     C     less than half that value. In this case, we use twelve
///     C     days.
///     C
///           STEP   = SPD() * 12.D0
///           ADJUST = 0.D0
///           REFVAL = 0.D0
///
///     C
///     C     Search for the date on which the CRDSYS system
///     C     coordinate COORD satisfies the RELATE constraint.
///     C
///           RELATE = 'LOCMAX'
///           CRDSYS = 'SPHERICAL'
///           COORD  = 'RADIUS'
///           TARG   = 'MOON'
///           OBSRVR = 'EARTH'
///           FRAME  = 'J2000'
///           ABCORR = 'NONE'
///
///     C
///     C     Perform this search using the geometric position
///     C     of the bodies; set the aberration correction to 'NONE'.
///     C
///           CALL GFPOSC ( TARG,   FRAME, ABCORR,
///          .              OBSRVR, CRDSYS, COORD,
///          .              RELATE, REFVAL, ADJUST,
///          .              STEP,   CNFINE,  MAXWIN,
///          .              NWMAX,  WORK,   RESULT )
///
///     C
///     C     Display the results.
///     C
///           IF ( WNCARD(RESULT) .EQ. 0 ) THEN
///
///              WRITE (*,*) 'Result window is empty.'
///
///           ELSE
///
///              DO I = 1, WNCARD(RESULT)
///
///     C
///     C           Fetch the endpoints of the Ith interval
///     C           of the result window.
///     C
///                 CALL WNFETD ( RESULT, I, START, FINISH )
///
///                 IF( START .EQ. FINISH ) THEN
///
///     C
///     C              The result window contains singleton
///     C              intervals, so we need display only the
///     C              start times.
///     C
///                    CALL TIMOUT ( START, TIMFMT, TIMSTR )
///                    WRITE (*, *) 'Event time: ', TIMSTR
///
///                 ELSE
///
///                    CALL TIMOUT ( START,  TIMFMT, TIMSTR )
///                    CALL TIMOUT ( FINISH, TIMFMT, TIMFIN )
///
///                    WRITE(*, *) 'From : ', TIMSTR
///                    WRITE(*, *) 'To   : ', TIMFIN
///                    WRITE(*, *) ' '
///
///                 END IF
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
///      Event time: 2007-JAN-10 16:26:18.784521 (TDB)
///      Event time: 2007-FEB-07 12:39:35.055710 (TDB)
///      Event time: 2007-MAR-07 03:38:07.308330 (TDB)
///      Event time: 2007-APR-03 08:38:55.191516 (TDB)
///      Event time: 2007-APR-30 10:56:49.819340 (TDB)
///      Event time: 2007-MAY-27 22:03:28.834302 (TDB)
///      Event time: 2007-JUN-24 14:26:23.617432 (TDB)
///      Event time: 2007-JUL-22 08:43:50.113902 (TDB)
///      Event time: 2007-AUG-19 03:28:33.515939 (TDB)
///      Event time: 2007-SEP-15 21:07:13.940711 (TDB)
///      Event time: 2007-OCT-13 09:52:30.791223 (TDB)
///      Event time: 2007-NOV-09 12:32:50.039258 (TDB)
///      Event time: 2007-DEC-06 16:54:31.199770 (TDB)
///
///
///  5) Find times between Jan 1, 2007 and Jan 1, 2008 when the
///     latitude (elevation) of the observer-target vector between
///     DSS 17 and the Moon, as observed in the DSS 17 topocentric
///     (station) frame, exceeds 83 degrees.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: gfposc_ex5.tm
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
///           earthstns_itrf93_050714.bsp   SPK for DSN Station
///                                         Locations
///           earth_topo_050714.tf          Topocentric DSN stations
///                                         frame definitions
///           earth_latest_high_prec.bpc    High precision earth PCK
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de421.bsp',
///                               'pck00009.tpc',
///                               'naif0009.tls',
///                               'earthstns_itrf93_050714.bsp',
///                               'earth_topo_050714.tf',
///                               'earth_latest_high_prec.bpc'  )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM GFPOSC_EX5
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
///           DOUBLE PRECISION      SPD
///           DOUBLE PRECISION      RPD
///           INTEGER               WNCARD
///
///     C
///     C     Local parameters
///     C
///           INTEGER               LBCELL
///           PARAMETER           ( LBCELL = -5 )
///
///     C
///     C     Use the parameter MAXWIN for both
///     C     the result window size and the workspace
///     C     size.
///     C
///           INTEGER               MAXWIN
///           PARAMETER           ( MAXWIN = 750 )
///
///     C
///     C     String length.
///     C
///           INTEGER               STRLEN
///           PARAMETER           ( STRLEN = 64 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(STRLEN)    TIMSTR
///           CHARACTER*(STRLEN)    TIMFIN
///           CHARACTER*(STRLEN)    RELATE
///           CHARACTER*(STRLEN)    CRDSYS
///           CHARACTER*(STRLEN)    COORD
///           CHARACTER*(STRLEN)    ABCORR
///           CHARACTER*(STRLEN)    TARG
///           CHARACTER*(STRLEN)    OBSRVR
///           CHARACTER*(STRLEN)    FRAME
///           CHARACTER*(STRLEN)   TIMFMT
///
///
///           DOUBLE PRECISION      ADJUST
///           DOUBLE PRECISION      CNFINE ( LBCELL : 2 )
///           DOUBLE PRECISION      ET0
///           DOUBLE PRECISION      ET1
///           DOUBLE PRECISION      FINISH
///           DOUBLE PRECISION      REFVAL
///           DOUBLE PRECISION      RESULT ( LBCELL : MAXWIN )
///           DOUBLE PRECISION      START
///           DOUBLE PRECISION      STEP
///           DOUBLE PRECISION      WORK   ( LBCELL : MAXWIN, NWMAX )
///
///           INTEGER               I
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
///     C
///     C     Load kernels.
///     C
///           CALL FURNSH ( 'gfposc_ex5.tm' )
///
///     C
///     C     Initialize windows.
///     C
///           CALL SSIZED ( MAXWIN, RESULT )
///           CALL SSIZED ( 2,      CNFINE )
///
///           TIMFMT = 'YYYY-MON-DD HR:MN:SC.###### (TDB) ::TDB ::RND'
///
///     C
///     C     Store the time bounds of our search interval in
///     C     the confinement window.
///     C
///
///           CALL STR2ET ( '2007 JAN 1', ET0 )
///           CALL STR2ET ( '2008 JAN 1', ET1 )
///
///           CALL WNINSD ( ET0, ET1, CNFINE )
///
///     C
///     C     This search uses a step size of four hours since the
///     C     time for all declination zero-to-max-to-zero passes
///     C     within the search window exceeds eight hours.
///     C
///     C     The example uses an 83 degree elevation because of its
///     C     rare occurrence and short duration.
///     C
///           STEP   = SPD() * (4.D0/24.D0)
///           ADJUST = 0.D0
///           REFVAL = 83.D0 * RPD()
///
///     C
///     C     Search for the date on which the CRDSYS system
///     C     coordinate COORD satisfies the RELATE constraint.
///     C
///           RELATE = '>'
///           CRDSYS = 'LATITUDINAL'
///           COORD  = 'LATITUDE'
///           TARG   = 'MOON'
///           OBSRVR = 'DSS-17'
///           FRAME  = 'DSS-17_TOPO'
///           ABCORR = 'NONE'
///
///     C
///     C     Perform this search using the geometric position
///     C     of the bodies; set the aberration correction to 'NONE'.
///     C
///           CALL GFPOSC ( TARG,   FRAME, ABCORR,
///          .              OBSRVR, CRDSYS, COORD,
///          .              RELATE, REFVAL, ADJUST,
///          .              STEP,   CNFINE,  MAXWIN,
///          .              NWMAX,  WORK,   RESULT )
///
///     C
///     C     Display the results.
///     C
///           IF ( WNCARD(RESULT) .EQ. 0 ) THEN
///
///              WRITE (*,*) 'Result window is empty.'
///
///           ELSE
///
///              DO I = 1, WNCARD(RESULT)
///
///     C
///     C           Fetch the endpoints of the Ith interval
///     C           of the result window.
///     C
///                 CALL WNFETD ( RESULT, I, START, FINISH )
///
///                 IF( START .EQ. FINISH ) THEN
///
///     C
///     C              The result window contains singleton
///     C              intervals, so we need display only the
///     C              start times.
///     C
///                    CALL TIMOUT ( START, TIMFMT, TIMSTR )
///                    WRITE (*, *) 'Event time: ', TIMSTR
///
///                 ELSE
///
///                    CALL TIMOUT ( START,  TIMFMT, TIMSTR )
///                    CALL TIMOUT ( FINISH, TIMFMT, TIMFIN )
///
///                    WRITE(*, *) 'From : ', TIMSTR
///                    WRITE(*, *) 'To   : ', TIMFIN
///                    WRITE(*, *) ' '
///
///                 END IF
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
///      From : 2007-FEB-26 03:18:48.229281 (TDB)
///      To   : 2007-FEB-26 03:31:29.734931 (TDB)
///
///      From : 2007-MAR-25 01:12:38.550572 (TDB)
///      To   : 2007-MAR-25 01:23:53.909469 (TDB)
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
///      coordinate quantity utility package. Callers may
///      need to re-initialize the package after calling this routine.
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
/// -    SPICELIB Version 1.2.0, 03-NOV-2021 (JDR) (NJB)
///
///         Edited the header to comply with NAIF standard.
///
///         Added initialization of QCPARS(10) to pacify Valgrind.
///
///         Added entries #5 and #9 to $Exceptions section.
///
///         Updated description of WORK and RESULT arguments in $Brief_I/O,
///         $Detailed_Input and $Detailed_Output.
///
///         Added SAVE statements for CNFINE, WORK and RESULT variables in
///         code examples.
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
///         Header edits. COORD description to clarify the body with which
///         GEODETIC and PLANETOGRAPHIC coordinates are associated.
///         Clarified exception SPICE(NOTSUPPORTED) description.
///
///         Edits to Example section, proper description of "standard.tm"
///         meta kernel.
///
/// -    SPICELIB Version 1.0.1, 10-JUN-2009 (NJB) (EDW)
///
///         Edited argument descriptions.
///
/// -    SPICELIB Version 1.0.0, 17-FEB-2009 (NJB) (EDW)
/// ```
pub fn gfposc(
    ctx: &mut SpiceContext,
    target: &str,
    frame: &str,
    abcorr: &str,
    obsrvr: &str,
    crdsys: &str,
    coord: &str,
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
    GFPOSC(
        target.as_bytes(),
        frame.as_bytes(),
        abcorr.as_bytes(),
        obsrvr.as_bytes(),
        crdsys.as_bytes(),
        coord.as_bytes(),
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

//$Procedure GFPOSC (GF, observer-target vector coordinate search )
pub fn GFPOSC(
    TARGET: &[u8],
    FRAME: &[u8],
    ABCORR: &[u8],
    OBSRVR: &[u8],
    CRDSYS: &[u8],
    COORD: &[u8],
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
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let CNFINE = DummyArray::new(CNFINE, LBCELL..);
    let mut WORK = DummyArrayMut2D::new(WORK, LBCELL..=MW, 1..=NW);
    let mut RESULT = DummyArrayMut::new(RESULT, LBCELL..);
    let mut TOL: f64 = 0.0;
    let mut OK: bool = false;
    let mut QCPARS = ActualCharArray::new(LNSIZE, 1..=QNPARS);
    let mut QPNAMS = ActualCharArray::new(LNSIZE, 1..=QNPARS);
    let mut QDPARS = StackArray::<f64, 10>::new(1..=QNPARS);
    let QIPARS = StackArray::<i32, 10>::new(1..=QNPARS);
    let QLPARS = StackArray::<bool, 10>::new(1..=QNPARS);

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
    // Define no-use values for DVEC and DREF
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
    CHKIN(b"GFPOSC", ctx)?;

    //
    // Confirm minimum window sizes.
    //
    if ((MW < 2) || !EVEN(MW)) {
        SETMSG(
            b"Workspace window size was #; size must be at least 2 and an even value.",
            ctx,
        );
        ERRINT(b"#", MW, ctx);
        SIGERR(b"SPICE(INVALIDDIMENSION)", ctx)?;
        CHKOUT(b"GFPOSC", ctx)?;
        return Ok(());
    }

    if (SIZED(RESULT.as_slice(), ctx)? < 2) {
        SETMSG(b"Result window size was #; size must be at least 2.", ctx);
        ERRINT(b"#", SIZED(RESULT.as_slice(), ctx)?, ctx);
        SIGERR(b"SPICE(INVALIDDIMENSION)", ctx)?;
        CHKOUT(b"GFPOSC", ctx)?;
        return Ok(());
    }

    //
    // Set up a call to GFEVNT specific to the observer-target
    // coordinate search.
    //
    fstr::assign(QPNAMS.get_mut(1), b"TARGET");
    fstr::assign(QCPARS.get_mut(1), TARGET);

    fstr::assign(QPNAMS.get_mut(2), b"OBSERVER");
    fstr::assign(QCPARS.get_mut(2), OBSRVR);

    fstr::assign(QPNAMS.get_mut(3), b"ABCORR");
    fstr::assign(QCPARS.get_mut(3), ABCORR);

    fstr::assign(QPNAMS.get_mut(4), b"COORDINATE SYSTEM");
    fstr::assign(QCPARS.get_mut(4), CRDSYS);

    fstr::assign(QPNAMS.get_mut(5), b"COORDINATE");
    fstr::assign(QCPARS.get_mut(5), COORD);

    fstr::assign(QPNAMS.get_mut(6), b"REFERENCE FRAME");
    fstr::assign(QCPARS.get_mut(6), FRAME);

    fstr::assign(QPNAMS.get_mut(7), b"VECTOR DEFINITION");
    fstr::assign(QCPARS.get_mut(7), POSDEF);

    fstr::assign(QPNAMS.get_mut(8), b"METHOD");
    fstr::assign(QCPARS.get_mut(8), b" ");

    fstr::assign(QPNAMS.get_mut(9), b"DREF");
    fstr::assign(QCPARS.get_mut(9), &save.DREF);

    fstr::assign(QPNAMS.get_mut(10), b"DVEC");
    QDPARS[1] = save.DVEC[1];
    QDPARS[2] = save.DVEC[2];
    QDPARS[3] = save.DVEC[3];

    //
    // Initialize QCPARS(10) since GFEVNT will try to
    // left-justify it and convert it to upper case.
    //
    fstr::assign(QCPARS.get_mut(10), b" ");

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
        b"COORDINATE",
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
        NW,
        WORK.as_slice_mut(),
        NOBAIL,
        GFBAIL,
        RESULT.as_slice_mut(),
        ctx,
    )?;

    CHKOUT(b"GFPOSC", ctx)?;
    Ok(())
}
