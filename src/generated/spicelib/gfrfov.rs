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

/// GF, is ray in FOV?
///
/// Determine time intervals when a specified ray intersects the
/// space bounded by the field-of-view (FOV) of a specified
/// instrument.
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
///  ZZGET      P   ZZHOLDD retrieves a stored DP value.
///  GF_TOL     P   ZZHOLDD acts on the GF subsystem tolerance.
///  INST       I   Name of the instrument.
///  RAYDIR     I   Ray's direction vector.
///  RFRAME     I   Reference frame of ray's direction vector.
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
///           (FOV) of which is to be used for an target intersection
///           search: the direction from the observer to a target
///           is represented by a ray, and times when the specified
///           ray intersects the region of space bounded by the FOV
///           are sought.
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
///  RAYDIR   is the direction vector associated with a ray
///           representing a target. The ray emanates from the
///           location of the ephemeris object designated by the
///           input argument OBSRVR and is expressed relative to the
///           reference frame designated by RFRAME (see descriptions
///           below).
///
///  RFRAME   is the name of the reference frame associated with
///           the input ray's direction vector RAYDIR.
///
///           Since light time corrections are not supported for
///           rays, the orientation of the frame is always evaluated
///           at the epoch associated with the observer, as opposed
///           to the epoch associated with the light-time corrected
///           position of the frame center.
///
///           Case and leading or trailing blanks bracketing a
///           non-blank frame name are not significant in the string
///           RFRAME.
///
///  ABCORR   indicates the aberration corrections to be applied
///           when computing the ray's direction.
///
///           The supported aberration correction options are
///
///              'NONE'          No correction.
///
///              'S'             Stellar aberration correction,
///                              reception case.
///
///              'XS'            Stellar aberration correction,
///                              transmission case.
///
///           For detailed information, see the geometry finder
///           required reading, gf.req.
///
///           Case, leading and trailing blanks are not significant
///           in the string ABCORR.
///
///  OBSRVR   is the name of the body from which the target
///           represented by RAYDIR is observed. The instrument
///           designated by INST is treated as if it were co-located
///           with the observer.
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
///           discarded before GFRFOV conducts its search.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RESULT   is a SPICE window representing the set of time intervals,
///           within the confinement period, when the input ray is
///           "visible"; that is, when the ray is contained in the
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
///           bracketed within an interval of length CNVTOL; the
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
///  3)  If the observer's name cannot be mapped to an ID code, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  4)  If the aberration correction flag calls for light time
///      correction, an error is signaled by a routine in the call tree
///      of this routine.
///
///  5)  If the ray's direction vector is zero, an error is signaled by
///      a routine in the call tree of this routine.
///
///  6)  If the instrument name INST does not have corresponding NAIF
///      ID code, an error is signaled by a routine in the call
///      tree of this routine.
///
///  7)  If the FOV parameters of the instrument are not present in
///      the kernel pool, an error is signaled by a routine
///      in the call tree of this routine.
///
///  8)  If the FOV boundary has more than MAXVRT vertices, an error
///      is signaled by a routine in the call tree of this
///      routine.
///
///  9)  If the instrument FOV is polygonal, and this routine cannot
///      find a ray R emanating from the FOV vertex such that maximum
///      angular separation of R and any FOV boundary vector is within
///      the limit (pi/2)-MARGIN radians, an error is signaled
///      by a routine in the call tree of this routine. If the FOV
///      is any other shape, the same error check will be applied with
///      the instrument boresight vector serving the role of R.
///
///  10) If the loaded kernels provide insufficient data to compute a
///      requested state vector, an error is signaled by a
///      routine in the call tree of this routine.
///
///  11) If an error occurs while reading an SPK or other kernel file,
///      the error is signaled by a routine in the call tree
///      of this routine.
///
///  12) If the output SPICE window RESULT has size less than 2, the
///      error SPICE(WINDOWTOOSMALL) is signaled.
///
///  13) If the output SPICE window RESULT has insufficient capacity
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
///  -  SPK data: ephemeris data for the observer for the period
///     defined by the confinement window CNFINE must be loaded.
///     If aberration corrections are used, the state of the
///     observer relative to the solar system barycenter must be
///     calculable from the available ephemeris data. Typically
///     ephemeris data are made available by loading one or more SPK
///     files via FURNSH.
///
///  -  Data defining the reference frame associated with the
///     instrument designated by INST must be available in the kernel
///     pool. Additionally the name INST must be associated with an
///     ID code. Normally these data are  made available by loading
///     a frame kernel via FURNSH.
///
///  -  IK data: the kernel pool must contain data such that
///     the SPICELIB routine GETFOV may be called to obtain
///     parameters for INST. Normally such data are provided by
///     an IK via FURNSH.
///
///  The following data may be required:
///
///  -  CK data: if the instrument frame is fixed to a spacecraft,
///     at least one CK file will be needed to permit transformation
///     of vectors between that frame and the J2000 frame.
///
///  -  SCLK data: if a CK file is needed, an associated SCLK
///     kernel is required to enable conversion between encoded SCLK
///     (used to time-tag CK data) and barycentric dynamical time
///     (TDB).
///
///  -  Since the input ray direction may be expressed in any
///     frame, FKs, CKs, SCLK kernels, PCKs, and SPKs may be
///     required to map the direction to the J2000 frame.
///
///  Kernel data are normally loaded once per program run, NOT every
///  time this routine is called.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine determines a set of one or more time intervals when
///  the specified ray in contained within the field of view of a
///  specified instrument. We'll use the term "visibility event" to
///  designate such an appearance. The set of time intervals resulting
///  from the search is returned as a SPICE window.
///
///  This routine provides a simpler, but less flexible, interface
///  than does the SPICELIB routine GFFOVE for conducting searches for
///  visibility events. Applications that require support for progress
///  reporting, interrupt handling, non-default step or refinement
///  functions, or non-default convergence tolerance should call
///  GFFOVE rather than this routine.
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
///  transitions: times are sought when the state of the ray
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
///  Having some knowledge of the relative geometry of the ray and
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
///  1) This example is an extension of example #1 in the
///     header of
///
///        GFTFOV
///
///     The problem statement for that example is
///
///        Search for times when Saturn's satellite Phoebe is within
///        the FOV of the Cassini narrow angle camera
///        (CASSINI_ISS_NAC). To simplify the problem, restrict the
///        search to a short time period where continuous Cassini bus
///        attitude data are available.
///
///        Use a step size of 10 seconds to reduce chances of missing
///        short visibility events.
///
///     Here we search the same confinement window for times when a
///     selected background star is visible. We use the FOV of the
///     Cassini ISS wide angle camera (CASSINI_ISS_WAC) to enhance the
///     probability of viewing the star.
///
///     The star we'll use has catalog number 6000 in the Hipparcos
///     Catalog. The star's J2000 right ascension and declination,
///     proper motion, and parallax are taken from that catalog.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: gfrfov_ex1.tm
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
///           PROGRAM GFRFOV_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      J1950
///           DOUBLE PRECISION      J2000
///           DOUBLE PRECISION      JYEAR
///           DOUBLE PRECISION      RPD
///
///           INTEGER               WNCARD
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         META
///           PARAMETER           ( META   = 'gfrfov_ex1.tm' )
///
///           CHARACTER*(*)         TIMFMT
///           PARAMETER           ( TIMFMT =
///          .      'YYYY-MON-DD HR:MN:SC.######::TDB' )
///
///
///           DOUBLE PRECISION      AU
///           PARAMETER           ( AU     = 149597870.693D0 )
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
///           CHARACTER*(FRNMLN)    RFRAME
///           CHARACTER*(TIMLEN)    TIMSTR ( 2 )
///
///           DOUBLE PRECISION      CNFINE ( LBCELL : 2 )
///           DOUBLE PRECISION      DEC
///           DOUBLE PRECISION      DECEPC
///           DOUBLE PRECISION      DECPM
///           DOUBLE PRECISION      DECDEG
///           DOUBLE PRECISION      DECDG0
///           DOUBLE PRECISION      DTDEC
///           DOUBLE PRECISION      DTRA
///           DOUBLE PRECISION      ENDPT  ( 2 )
///           DOUBLE PRECISION      ET0
///           DOUBLE PRECISION      ET1
///           DOUBLE PRECISION      LT
///           DOUBLE PRECISION      PARLAX
///           DOUBLE PRECISION      PLXDEG
///           DOUBLE PRECISION      POS    ( 3 )
///           DOUBLE PRECISION      PSTAR  ( 3 )
///           DOUBLE PRECISION      RA
///           DOUBLE PRECISION      RADEG
///           DOUBLE PRECISION      RADEG0
///           DOUBLE PRECISION      RAEPC
///           DOUBLE PRECISION      RAPM
///           DOUBLE PRECISION      RAYDIR ( 3 )
///           DOUBLE PRECISION      RESULT ( LBCELL : MAXWIN )
///           DOUBLE PRECISION      RSTAR
///           DOUBLE PRECISION      STEPSZ
///           DOUBLE PRECISION      T
///
///           INTEGER               CATNO
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
///           INST   = 'CASSINI_ISS_WAC'
///
///     C
///     C     Create a unit direction vector pointing from
///     c     observer to star. We'll assume the direction
///     C     is constant during the confinement window, and
///     C     we'll use et0 as the epoch at which to compute the
///     C     direction from the spacecraft to the star.
///     C
///     C     The data below are for the star with catalog
///     C     number 6000 in the Hipparcos catalog. Angular
///     C     units are degrees; epochs have units of Julian
///     C     years and have a reference epoch of J1950.
///     C     The reference frame is J2000.
///     C
///           CATNO  = 6000
///
///           PLXDEG = 0.000001056D0
///
///           RADEG0 = 19.290789927D0
///           RAPM   = -0.000000720D0
///           RAEPC  = 41.2000D0
///
///           DECDG0 =  2.015271007D0
///           DECPM  =  0.000001814D0
///           DECEPC = 41.1300D0
///
///           RFRAME = 'J2000'
///
///     C
///     C     Correct the star's direction for proper motion.
///     C
///     C     The argument t represents et0 as Julian years
///     C     past J1950.
///     C
///           T      =      ET0/JYEAR()
///          .         +  ( J2000()- J1950() ) / 365.25D0
///
///           DTRA   = T - RAEPC
///           DTDEC  = T - DECEPC
///
///           RADEG  = RADEG0  +  DTRA  * RAPM
///           DECDEG = DECDG0  +  DTDEC * DECPM
///
///           RA     = RADEG  * RPD()
///           DEC    = DECDEG * RPD()
///
///           CALL RADREC ( 1.D0, RA, DEC, PSTAR )
///
///     C
///     C     Correct star position for parallax applicable at
///     C     the Cassini orbiter's position. (The parallax effect
///     C     is negligible in this case; we're simply demonstrating
///     C     the computation.)
///     C
///           PARLAX = PLXDEG * RPD()
///           RSTAR  = AU / TAN(PARLAX)
///
///     C
///     C     Scale the star's direction vector by its distance from
///     C     the solar system barycenter. Subtract off the position
///     C     of the spacecraft relative to the solar system
///     C     barycenter; the result is the ray's direction vector.
///     C
///           CALL VSCLIP ( RSTAR, PSTAR )
///
///           CALL SPKPOS ( 'CASSINI', ET0, 'J2000',  'NONE',
///          .              'SOLAR SYSTEM BARYCENTER', POS,  LT )
///
///           CALL VSUB   ( PSTAR, POS, RAYDIR )
///
///     C
///     C     Correct the star direction for stellar aberration when
///     C     we conduct the search.
///     C
///           ABCORR = 'S'
///           OBSRVR = 'CASSINI'
///           STEPSZ = 10.D0
///
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Instrument:              '//INST
///           WRITE (*,*) 'Star''s catalog number:  ', CATNO
///           WRITE (*,*) ' '
///
///     C
///     C     Perform the search.
///     C
///           CALL GFRFOV ( INST,   RAYDIR, RFRAME, ABCORR,
///          .              OBSRVR, STEPSZ, CNFINE, RESULT )
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
///      Instrument:              CASSINI_ISS_WAC
///      Star's catalog number:          6000
///
///       Visibility start time (TDB)           Stop time (TDB)
///       ---------------------------     ---------------------------
///       2004-JUN-11 06:30:00.000000     2004-JUN-11 12:00:00.000000
///
///
///     Note that the star is visible throughout the confinement
///     window.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The kernel files to be used by GFRFOV must be loaded (normally
///      via the SPICELIB routine FURNSH) before GFRFOV is called.
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
///         Modified code examples' output to comply with maximum line
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
pub fn gfrfov(
    ctx: &mut SpiceContext,
    inst: &str,
    raydir: &[f64; 3],
    rframe: &str,
    abcorr: &str,
    obsrvr: &str,
    step: f64,
    cnfine: &[f64],
    result: &mut [f64],
) -> crate::Result<()> {
    GFRFOV(
        inst.as_bytes(),
        raydir,
        rframe.as_bytes(),
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

//$Procedure GFRFOV ( GF, is ray in FOV? )
pub fn GFRFOV(
    INST: &[u8],
    RAYDIR: &[f64],
    RFRAME: &[u8],
    ABCORR: &[u8],
    OBSRVR: &[u8],
    STEP: f64,
    CNFINE: &[f64],
    RESULT: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let RAYDIR = DummyArray::new(RAYDIR, 1..=3);
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
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"GFRFOV", ctx)?;

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
        CHKOUT(b"GFRFOV", ctx)?;
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
        RYSHAP,
        RAYDIR.as_slice(),
        b" ",
        RFRAME,
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

    CHKOUT(b"GFRFOV", ctx)?;
    Ok(())
}
