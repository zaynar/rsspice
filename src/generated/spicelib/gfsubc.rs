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

/// GF, subpoint vector coordinate search
///
/// Determine time intervals for which a coordinate of an
/// subpoint position vector satisfies a numerical constraint.
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
///  FIXREF     I   Body fixed frame associated with TARGET.
///  METHOD     I   Name of method type for subpoint calculation.
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
///  TARGET   is the string name of a target body. Optionally, you may
///           supply the integer ID code for the object as an
///           integer string. For example both 'MOON' and '301'
///           are legitimate strings that indicate the moon is the
///           target body.
///
///           The target and observer define a position vector
///           that points from the observer to the target.
///
///  FIXREF   is the string name of the body-fixed, body-centered
///           reference frame associated with the target body TARGET.
///
///           The SPICE frame subsystem must recognize the 'fixref'
///           name.
///
///  METHOD   is the string name of the method to use for the subpoint
///           calculation. The accepted values for METHOD:
///
///              'Near point: ellipsoid'   The sub-observer point
///                                        computation uses a
///                                        triaxial ellipsoid to
///                                        model the surface of the
///                                        target body. The
///                                        sub-observer point is
///                                        defined as the nearest
///                                        point on the target
///                                        relative to the
///                                        observer.
///
///              'Intercept: ellipsoid'    The sub-observer point
///                                        computation uses a
///                                        triaxial ellipsoid to
///                                        model the surface of the
///                                        target body. The
///                                        sub-observer point is
///                                        defined as the target
///                                        surface intercept of the
///                                        line containing the
///                                        observer and the
///                                        target's center.
///
///           The METHOD string lacks sensitivity to case, embedded,
///           leading and trailing blanks.
///
///  ABCORR   is the string description of the aberration corrections
///           to apply to the state evaluations to account for one-way
///           light time and stellar aberration.
///
///           This routine accepts the same aberration corrections
///           as does the SPICE routine SPKEZR. See the header of
///           SPKEZR for a detailed description of the aberration
///           correction options. For convenience, the options are
///           listed below:
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
///  OBSRVR   is the string name of an observing body. Optionally, you
///           may supply the ID code of the object as an integer
///           string. For example, both 'EARTH' and '399' are
///           legitimate strings to indicate that the observer is the
///           Earth.
///
///  CRDSYS   is the string name of the coordinate system for which the
///           coordinate of interest is a member.
///
///  COORD    is the string name of the coordinate of interest in
///           CRDSYS.
///
///           The supported coordinate systems and coordinate names:
///
///              CRDSYS             COORD               Range
///              ----------------   -----------------   ------------
///
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
///           coordinates is the center of the frame FIXREF.
///
///  RELATE   is the string or character describing the relational
///           operator used to define a constraint on the selected
///           coordinate of the subpoint vector. The result
///           window found by this routine indicates the time intervals
///           where the constraint is satisfied. Supported values of
///           RELATE and corresponding meanings are shown below:
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
///           The caller may indicate that the region of interest
///           is the set of time intervals where the quantity is
///           within a specified measure of an absolute extremum.
///           The argument ADJUST (described below) is used to
///           specify this measure.
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
///           subpoint vector. See the discussion of RELATE above for
///           further information.
///
///           The units of REFVAL correspond to the type as defined
///           by COORD, radians for angular measures, kilometers for
///           distance measures.
///
///  ADJUST   is a double precision value used to modify searches for
///           absolute extrema: when RELATE is set to 'ABSMAX' or
///           'ABSMIN' and ADJUST is set to a positive value, GFSUBC
///           finds times when the subpoint position vector coordinate
///           is within ADJUST radians/kilometers of the specified
///           extreme value.
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
///           function of the subpoint vector is monotone increasing or
///           decreasing. However, STEP must not be *too* short, or
///           the search will take an unreasonable amount of time.
///
///           For coordinates other than 'LONGITUDE' and 'RIGHT
///           ASCENSION', the step size must be shorter than the
///           shortest interval, within the confinement window, over
///           which the coordinate is monotone increasing or
///           decreasing.
///
///           For 'LONGITUDE' and 'RIGHT ASCENSION', the step size must
///           be shorter than the shortest interval, within the
///           confinement window, over which either the sin or cos
///           of the coordinate is monotone increasing or decreasing.
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
///           discarded before GFSUBC conducts its search.
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
///  Such kernel data are normally loaded once per program run, NOT
///  every time this routine is called.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine provides a simpler, but less flexible interface
///  than does the routine GFEVNT for conducting searches for
///  subpoint position vector coordinate value events.
///  Applications that require support for progress reporting,
///  interrupt handling, non-default step or refinement functions, or
///  non-default convergence tolerance should call GFEVNT rather than
///  this routine.
///
///  This routine determines a set of one or more time intervals
///  within the confinement window when the selected coordinate of
///  the subpoint position vector satisfies a caller-specified
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
///  longitude. Round-off error may cause solutions near these branches
///  to cross the branch. Use of the SPICE routine WNCOND will contract
///  solution windows by some epsilon, reducing the measure of the
///  windows and eliminating the branch crossing. A one millisecond
///  contraction will in most cases eliminate numerical round-off
///  caused branch crossings.
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
///  1) Find the time during 2007 for which the subpoint position
///     vector of the Sun on Earth in the IAU_EARTH frame lies within
///     a geodetic latitude-longitude "box" defined as
///
///        16 degrees <= latitude  <= 17 degrees
///        85 degrees <= longitude <= 86 degrees
///
///     This problem requires four searches, each search on one of the
///     box restrictions. The user needs also realize the temporal
///     behavior of latitude greatly differs from that of the
///     longitude. The sub-observer point latitude varies between
///     approximately 23.44 degrees and -23.44 degrees during the
///     year. The sub-observer point longitude varies between -180
///     degrees and 180 degrees in one day.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: gfsubc_ex1.tm
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
///           de414.bsp                     Planetary ephemeris
///           pck00008.tpc                  Planet orientation and
///                                         radii
///           naif0008.tls                  Leapseconds
///
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de414.bsp',
///                               'pck00008.tpc',
///                               'naif0008.tls'  )
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM GFSUBC_EX1
///           IMPLICIT NONE
///
///     C
///     C     Include GF parameter declarations:
///     C
///           INCLUDE               'gf.inc'
///
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      SPD
///           DOUBLE PRECISION      DPR
///           DOUBLE PRECISION      RPD
///
///           INTEGER               WNCARD
///
///     C
///     C     Local parameters
///     C
///           INTEGER               LBCELL
///           PARAMETER           ( LBCELL = -5 )
///
///     C
///     C     Create 50 windows.
///     C
///           INTEGER               MAXWIN
///           PARAMETER           ( MAXWIN = 1000 )
///
///     C
///     C     One window consists of two intervals.
///     C
///           INTEGER               NINTRVL
///           PARAMETER           ( NINTRVL = MAXWIN *2 )
///
///           INTEGER               STRLEN
///           PARAMETER           ( STRLEN = 28 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(STRLEN)    TIMFMT
///           CHARACTER*(STRLEN)    BEGSTR
///           CHARACTER*(STRLEN)    ENDSTR
///           CHARACTER*(STRLEN)    TARGET
///           CHARACTER*(STRLEN)    OBSRVR
///           CHARACTER*(STRLEN)    ABCORR
///           CHARACTER*(STRLEN)    METHOD
///           CHARACTER*(STRLEN)    FIXREF
///           CHARACTER*(STRLEN)    CRDSYS
///           CHARACTER*(STRLEN)    COORD
///           CHARACTER*(STRLEN)    RELATE
///
///           DOUBLE PRECISION      STEP
///           DOUBLE PRECISION      CNFINE  ( LBCELL : 2       )
///           DOUBLE PRECISION      RESULT1 ( LBCELL : NINTRVL )
///           DOUBLE PRECISION      RESULT2 ( LBCELL : NINTRVL )
///           DOUBLE PRECISION      RESULT3 ( LBCELL : NINTRVL )
///           DOUBLE PRECISION      RESULT4 ( LBCELL : NINTRVL )
///           DOUBLE PRECISION      WORK    ( LBCELL : NINTRVL, NWMAX )
///           DOUBLE PRECISION      BEGTIM
///           DOUBLE PRECISION      ENDTIM
///           DOUBLE PRECISION      LEFT
///           DOUBLE PRECISION      RIGHT
///           DOUBLE PRECISION      REFVAL
///           DOUBLE PRECISION      ADJUST
///           DOUBLE PRECISION      RAD    ( 2 )
///           DOUBLE PRECISION      LON    ( 2 )
///           DOUBLE PRECISION      LAT    ( 2 )
///           DOUBLE PRECISION      TRGEPC
///           DOUBLE PRECISION      LPOS   ( 3 )
///           DOUBLE PRECISION      RPOS   ( 3 )
///           DOUBLE PRECISION      SRFVEC ( 3 )
///
///           INTEGER               COUNT
///           INTEGER               I
///
///     C
///     C     Saved variables
///     C
///     C     The confinement, workspace and result windows CNFINE,
///     C     WORK, RESULT1, RESULT2, RESULT3 and RESULT4 are saved
///     C     because this practice helps to prevent stack overflow.
///     C
///           SAVE                  CNFINE
///           SAVE                  RESULT1
///           SAVE                  RESULT2
///           SAVE                  RESULT3
///           SAVE                  RESULT4
///           SAVE                  WORK
///
///     C
///     C     Load kernels.
///     C
///           CALL FURNSH ('gfsubc_ex1.tm')
///
///           TIMFMT = 'YYYY-MON-DD HR:MN:SC.###### ::TDB ::RND'
///
///     C
///     C     Initialize windows RESULT and CNFINE.
///     C
///           CALL SSIZED ( NINTRVL, RESULT1 )
///           CALL SSIZED ( NINTRVL, RESULT2 )
///           CALL SSIZED ( NINTRVL, RESULT3 )
///           CALL SSIZED ( NINTRVL, RESULT4 )
///           CALL SSIZED ( 2,       CNFINE  )
///
///     C
///     C     Store the time bounds of our search interval in
///     C     the CNFINE confinement window.
///     C
///           CALL STR2ET ( '2007 JAN 01', BEGTIM )
///           CALL STR2ET ( '2008 JAN 01', ENDTIM )
///
///           CALL WNINSD ( BEGTIM, ENDTIM, CNFINE )
///
///     C
///     C     The latitude varies relatively slowly (46 degrees) during
///     C     the year. The extrema occur approximately every six
///     C     months. Search using a step size less than half that
///     C     value (180 days). For this example use ninety days (in
///     C     units of seconds).
///     C
///           STEP   = SPD()*90.D0
///
///     C
///     C     Perform four searches to determine the times when the
///     C     latitude- longitude box restriction conditions apply to
///     C     the subpoint vector.
///     C
///     C     Use geodetic coordinates.
///     C
///           ADJUST = 0.D0
///           TARGET = 'EARTH'
///           OBSRVR = 'SUN'
///           METHOD = 'Near point: ellipsoid'
///           FIXREF = 'IAU_EARTH'
///           CRDSYS = 'GEODETIC'
///           ABCORR = 'NONE'
///
///     C
///     C     Perform the searches such that the result window of a
///     C     search serves as the confinement window of the
///     C     subsequent search.
///     C
///     C     Since the latitude coordinate varies slowly and is well
///     C     behaved over the time of the confinement window, search
///     C     first for the windows satisfying the latitude
///     C     requirements, then use that result as confinement for
///     C     the longitude search.
///     C
///           COORD  = 'LATITUDE'
///           REFVAL = 16.D0 * RPD()
///           RELATE = '>'
///
///           CALL GFSUBC (  TARGET,  FIXREF,
///          .               METHOD,  ABCORR, OBSRVR,
///          .               CRDSYS,  COORD,
///          .               RELATE,  REFVAL,
///          .               ADJUST,  STEP,  CNFINE,
///          .               NINTRVL, NWMAX, WORK, RESULT1 )
///
///           REFVAL = 17.D0 * RPD()
///           RELATE = '<'
///
///           CALL GFSUBC (  TARGET,  FIXREF,
///          .               METHOD,  ABCORR, OBSRVR,
///          .               CRDSYS,  COORD,
///          .               RELATE,  REFVAL,
///          .               ADJUST,  STEP,  RESULT1,
///          .               NINTRVL, NWMAX, WORK, RESULT2 )
///
///     C
///     C     Now the longitude search.
///     C
///           COORD  = 'LONGITUDE'
///
///     C
///     C     Reset the step size to something appropriate for the 360
///     C     degrees in 24 hours domain. The longitude shows near
///     C     linear behavior so use a step size less than half the
///     C     period of twelve hours. Ten hours will suffice in this
///     C     case.
///     C
///           STEP   = SPD() * (10.D0/24.D0)
///
///           REFVAL = 85.D0 * RPD()
///           RELATE = '>'
///
///
///           CALL GFSUBC (  TARGET,  FIXREF,
///          .               METHOD,  ABCORR, OBSRVR,
///          .               CRDSYS,  COORD,
///          .               RELATE,  REFVAL,
///          .               ADJUST,  STEP,   RESULT2,
///          .               NINTRVL, NWMAX, WORK, RESULT3 )
///
///     C
///     C     Contract the endpoints of each window to account
///     C     for possible round-off error at the -180/180 degree
///     C     branch.
///     C
///     C     A contraction value of a millisecond should eliminate
///     C     any round-off caused branch crossing.
///     C
///           CALL WNCOND ( 1D-3, 1D-3, RESULT3 )
///
///           REFVAL = 86.D0 * RPD()
///           RELATE = '<'
///
///           CALL GFSUBC (  TARGET,  FIXREF,
///          .               METHOD,  ABCORR, OBSRVR,
///          .               CRDSYS,  COORD,
///          .               RELATE,  REFVAL,
///          .               ADJUST,  STEP,   RESULT3,
///          .               NINTRVL, NWMAX, WORK, RESULT4 )
///
///     C
///     C     Check the number of intervals in the result window.
///     C
///           COUNT = WNCARD(RESULT4)
///
///     C
///     C     List the beginning and ending points in each interval
///     C     if RESULT contains data.
///     C
///           IF ( COUNT .EQ. 0 ) THEN
///
///              WRITE(*, '(A)') 'Result window is empty.'
///
///           ELSE
///
///              WRITE(*, '(A)') '       Time (TDB)                 '
///          .   //              '     LAT (deg)      LON (deg)'
///              WRITE(*, '(A)') '       ---------------------------'
///          .   //              '    -----------   -----------'
///
///              DO I = 1, COUNT
///
///     C
///     C           Fetch the endpoints of the Ith interval
///     C           of the result window.
///     C
///                 CALL WNFETD ( RESULT4, I, LEFT, RIGHT  )
///
///                 CALL TIMOUT ( LEFT, TIMFMT, BEGSTR )
///                 CALL TIMOUT ( RIGHT, TIMFMT, ENDSTR )
///
///     C
///     C           Determine the latitude and longitude of the
///     C           subpoint at the event interval boundaries.
///     C
///                 CALL SUBPNT ( METHOD, TARGET, LEFT,
///          .                    FIXREF, ABCORR, OBSRVR,
///          .                    LPOS, TRGEPC, SRFVEC   )
///                 CALL RECLAT ( LPOS, RAD(1), LON(1), LAT(1) )
///
///                 CALL SUBPNT ( METHOD, TARGET, RIGHT,
///          .                    FIXREF, ABCORR, OBSRVR,
///          .                    RPOS, TRGEPC, SRFVEC   )
///                 CALL RECLAT ( RPOS, RAD(2), LON(2), LAT(2) )
///
///
///                 WRITE(*,'(2A,2F14.8)') 'From : ',    BEGSTR,
///          .                              LAT(1)*DPR(), LON(1)*DPR()
///                 WRITE(*,'(2A,2F14.8)') 'To   : ',    ENDSTR,
///          .                              LAT(2)*DPR(), LON(2)*DPR()
///                 WRITE(*,*) ' '
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
///            Time (TDB)                      LAT (deg)      LON (deg)
///            ---------------------------    -----------   -----------
///     From : 2007-MAY-05 06:12:59.452307    16.05435608   86.00000000
///     To   : 2007-MAY-05 06:16:59.436479    16.05514776   85.00000417
///
///     From : 2007-MAY-06 06:12:54.398070    16.33714720   86.00000000
///     To   : 2007-MAY-06 06:16:54.383826    16.33792651   85.00000417
///
///     From : 2007-MAY-07 06:12:49.917541    16.61544356   86.00000000
///     To   : 2007-MAY-07 06:16:49.904901    16.61621026   85.00000417
///
///     From : 2007-MAY-08 06:12:46.017221    16.88916258   86.00000000
///     To   : 2007-MAY-08 06:16:46.006200    16.88991646   85.00000417
///
///     From : 2007-AUG-06 06:22:12.099776    16.68071740   86.00000000
///     To   : 2007-AUG-06 06:26:12.080859    16.67996165   85.00000417
///
///     From : 2007-AUG-07 06:22:05.362314    16.40641076   86.00000000
///     To   : 2007-AUG-07 06:26:05.341799    16.40564259   85.00000417
///
///     From : 2007-AUG-08 06:21:58.050893    16.12767782   86.00000000
///     To   : 2007-AUG-08 06:25:58.028786    16.12689748   85.00000417
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
/// -    SPICELIB Version 1.2.0, 27-OCT-2021 (JDR) (NJB)
///
///         Edited the header to comply with NAIF standard.
///
///         Added initialization of QCPARS(10) to pacify Valgrind.
///
///         Modified code example's output to comply with maximum line
///         length of header comments. Added SAVE statements for CNFINE,
///         WORK, RESULT1, RESULT2, RESULT3 and RESULT4 variables in code
///         example.
///
///         Added entries #5 and #9 in $Exceptions section.
///
///         Updated description of WORK and RESULT arguments in $Brief_I/O,
///         $Detailed_Input and $Detailed_Output. Extended description of
///         COORD argument.
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
/// -    SPICELIB Version 1.0.1, 22-AUG-2009 (EDW)
///
///         Edited argument descriptions.
///
///         Edit to Example description, replaced "intercept" with
///         "sub-observer point."
///
/// -    SPICELIB Version 1.0.0, 17-FEB-2009 (NJB) (EDW)
/// ```
pub fn gfsubc(
    ctx: &mut SpiceContext,
    target: &str,
    fixref: &str,
    method: &str,
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
    GFSUBC(
        target.as_bytes(),
        fixref.as_bytes(),
        method.as_bytes(),
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

//$Procedure GFSUBC (GF, subpoint vector coordinate search )
pub fn GFSUBC(
    TARGET: &[u8],
    FIXREF: &[u8],
    METHOD: &[u8],
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
    CHKIN(b"GFSUBC", ctx)?;

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
        CHKOUT(b"GFSUBC", ctx)?;
        return Ok(());
    }

    if (SIZED(RESULT.as_slice(), ctx)? < 2) {
        SETMSG(b"Result window size was #; size must be at least 2.", ctx);
        ERRINT(b"#", SIZED(RESULT.as_slice(), ctx)?, ctx);
        SIGERR(b"SPICE(INVALIDDIMENSION)", ctx)?;
        CHKOUT(b"GFSUBC", ctx)?;
        return Ok(());
    }

    //
    // Set up a call to GFEVNT specific to the subpoint coordinate
    // search.
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
    fstr::assign(QCPARS.get_mut(6), FIXREF);

    fstr::assign(QPNAMS.get_mut(7), b"VECTOR DEFINITION");
    fstr::assign(QCPARS.get_mut(7), SOBDEF);

    fstr::assign(QPNAMS.get_mut(8), b"METHOD");
    fstr::assign(QCPARS.get_mut(8), METHOD);

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

    CHKOUT(b"GFSUBC", ctx)?;
    Ok(())
}
