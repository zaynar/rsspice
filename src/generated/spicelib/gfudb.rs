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
const NOBAIL: bool = false;
const NORPT: bool = false;

/// GF, user defined boolean
///
/// Perform a GF search on a user defined boolean quantity.
///
/// # Required Reading
///
/// * [GF](crate::required_reading::gf)
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
///  UDFUNS     I   Name of the routine that computes a scalar
///                 quantity corresponding to an ET.
///  UDFUNB     I   Name of the routine returning the boolean value
///                 corresponding to an ET.
///  STEP       I   Constant step size in seconds for finding geometric
///                 events.
///  CNFINE     I   SPICE window to which the search is restricted.
///  RESULT    I-O  SPICE window containing results.
/// ```
///
/// # Detailed Input
///
/// ```text
///  UDFUNS   is the routine that returns the value of the scalar
///           quantity of interest at time ET. The calling sequence for
///           UDFUNC is:
///
///              CALL UDFUNS ( ET, VALUE )
///
///           where:
///
///              ET      a double precision value representing
///                      ephemeris time, expressed as seconds past
///                      J2000 TDB at which to evaluate UDFUNS.
///
///              VALUE   is the value of the scalar quantity
///                      at ET.
///
///  UDFUNB   is the user defined routine returning a boolean value for
///           an epoch ET. The calling sequence for UNFUNB is:
///
///              CALL UDFUNB ( UDFUNS, ET, BOOL )
///
///           where:
///
///              UDFUNS   the name of the scalar function as
///                       defined above.
///
///              ET       a double precision value representing
///                       ephemeris time, expressed as seconds past
///                       J2000 TDB, at which to evaluate UDFUNB.
///
///              BOOL     the boolean value at ET.
///
///           GFUDB will correctly operate only for boolean functions
///           with true conditions defining non zero measure time
///           intervals.
///
///           Note, UDFUNB need not call UDFUNS. The use of UDFUNS is
///           determined by the needs of the calculation and the user's
///           design.
///
///  STEP     is the step size to be used in the search. STEP must be
///           shorter than any interval, within the confinement window,
///           over which the user defined boolean function is met. In
///           other words, STEP must be shorter than the shortest time
///           interval for which the boolean function is .TRUE.; STEP
///           must also be shorter than the shortest time interval
///           between two boolean function true events occurring within
///           the confinement window (see below). However, STEP must
///           not be *too* short, or the search will take an
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
///  CNFINE   is a SPICE window that confines the time period over
///           which the specified search is conducted. CNFINE may
///           consist of a single interval or a collection of
///           intervals.
///
///           In some cases the confinement window can be used to
///           greatly reduce the time period that must be searched
///           for the desired solution. See the $Particulars section
///           below for further discussion.
///
///           See the $Examples section below for a code example
///           that shows how to create a confinement window.
///
///           CNFINE must be initialized by the caller via the
///           SPICELIB routine SSIZED.
///
///           Certain computations can expand the time window over
///           which UDFUNS and UDFUNB require data. See $Particulars
///           for details.
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
///           discarded before GFUDB conducts its search.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RESULT   is a SPICE window containing the time intervals within
///           the confinement window, during which the specified
///           boolean quantity is .TRUE.
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
///  LBCELL   is the integer value defining the lower bound for
///           SPICE Cell arrays (a SPICE window is a kind of cell).
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
///  3)  If an error (typically cell overflow) occurs while performing
///      window arithmetic, the error is signaled by a routine
///      in the call tree of this routine.
///
///  4)  If the size of the SPICE window RESULT is less than 2 or not
///      an even value, the error SPICE(INVALIDDIMENSION) is signaled.
///
///  5)  If RESULT has insufficient capacity to contain the number of
///      intervals on which the specified condition is met, an error is
///      signaled by a routine in the call tree of this routine.
///
///  6)  If required ephemerides or other kernel data are not
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
///  If the boolean function requires access to ephemeris data:
///
///  -  SPK data: ephemeris data for any body over the
///     time period defined by the confinement window must be
///     loaded. If aberration corrections are used, the states of
///     target and observer relative to the solar system barycenter
///     must be calculable from the available ephemeris data.
///     Typically ephemeris data are made available by loading one
///     or more SPK files via FURNSH.
///
///  -  If non-inertial reference frames are used, then PCK
///     files, frame kernels, C-kernels, and SCLK kernels may be
///     needed.
///
///  -  Certain computations can expand the time window over which
///     UDFUNS and UDFUNB require data; such data must be provided by
///     loaded kernels. See $Particulars for details.
///
///  In all cases, kernel data are normally loaded once per program
///  run, NOT every time this routine is called.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine determines a set of one or more time intervals
///  within the confinement window when the boolean function
///  evaluates to true. The resulting set of intervals is returned
///  as a SPICE window.
///
///  Below we discuss in greater detail aspects of this routine's
///  solution process that are relevant to correct and efficient
///  use of this routine in user applications.
///
///  UDFUNS Default Template
///  =======================
///
///  The boolean function includes an argument for an input scalar
///  function. Use of a scalar function during the evaluation of
///  the boolean function is not required. SPICE provides a no-op
///  scalar routine, UDF, as a dummy argument for instances when
///  the boolean function does not need to call the scalar function.
///
///  The Search Process
///  ==================
///
///  The search for boolean events is treated as a search for state
///  transitions: times are sought when the boolean function value
///  changes from true to false or vice versa.
///
///  Step Size
///  =========
///
///  Each interval of the confinement window is searched as follows:
///  first, the input step size is used to determine the time
///  separation at which the boolean function will be sampled.
///  Starting at the left endpoint of the interval, samples of the
///  boolean function will be taken at each step. If a state change
///  is detected, a root has been bracketed; at that point, the
///  "root"--the time at which the state change occurs---is found by a
///  refinement process, for example, via binary search.
///
///  Note that the optimal choice of step size depends on the lengths
///  of the intervals over which the boolean function is constant:
///  the step size should be shorter than the shortest such interval
///  and the shortest separation between the intervals, within
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
///  interval within which a solution is sought.
///
///  The confinement window also can be used to restrict a search to
///  a time window over which required data are known to be
///  available.
///
///  In some cases, the confinement window can be used to make
///  searches more efficient. Sometimes it's possible to do an
///  efficient search to reduce the size of the time period over
///  which a relatively slow search of interest must be performed.
///  See the "CASCADE" example program in gf.req for a demonstration.
///
///  Certain user-defined computations may expand the window over
///  which computations are performed. Here "expansion" of a window by
///  an amount "T" means that the left endpoint of each interval
///  comprising the window is shifted left by T, the right endpoint of
///  each interval is shifted right by T, and any overlapping
///  intervals are merged. Note that the input window CNFINE itself is
///  not modified.
///
///  Computation of observer-target states by SPKEZR or SPKEZ, using
///  stellar aberration corrections, requires the state of the
///  observer, relative to the solar system barycenter, to be computed
///  at times offset from the input time by +/- 1 second. If the input
///  time ET is used by UDFUNS or UDFUNB to compute such a state, the
///  window over which the observer state is computed is expanded by 1
///  second.
///
///  When light time corrections are used in the computation of
///  observer-target states, expansion of the search window also
///  affects the set of times at which the light time-corrected states
///  of the targets are computed.
///
///  In addition to possible expansion of the search window when
///  stellar aberration corrections are used, round-off error should
///  be taken into account when the need for data availability is
///  analyzed.
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
///  1) Calculate the time intervals when the position of the Moon
///     relative to the Earth in the IAU_EARTH frame has a positive
///     value for the Z position component, also with a positive value
///     for the Vz velocity component.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: gfudb_ex1.tm
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
///           de418.bsp                     Planetary ephemeris
///           pck00009.tpc                  Planet orientation and
///                                         radii
///           naif0009.tls                  Leapseconds
///
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de418.bsp',
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
///           PROGRAM GFUDB_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           INTEGER               WNCARD
///           DOUBLE PRECISION      SPD
///
///     C
///     C     User defined external routines
///     C
///           EXTERNAL              UDF
///           EXTERNAL              GFB
///
///     C
///     C     Local parameters
///     C
///           INTEGER               LBCELL
///           PARAMETER           ( LBCELL = -5 )
///
///     C
///     C     Use the parameter MAXWIN for both the result window size
///     C     and the workspace size.
///     C
///           INTEGER               MAXWIN
///           PARAMETER           ( MAXWIN = 100 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(32)        UTC
///
///           DOUBLE PRECISION      LEFT
///           DOUBLE PRECISION      RIGHT
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      ETS
///           DOUBLE PRECISION      ETE
///           DOUBLE PRECISION      LT
///           DOUBLE PRECISION      STEP
///           DOUBLE PRECISION      STATE  (6)
///           DOUBLE PRECISION      CNFINE ( LBCELL : 2      )
///           DOUBLE PRECISION      RESULT ( LBCELL : MAXWIN )
///
///           INTEGER               I
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
///     C     Load needed kernels.
///     C
///           CALL FURNSH ( 'gfudb_ex1.tm' )
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
///           CALL STR2ET ( 'Jan 1 2011', ETS )
///           CALL STR2ET ( 'Apr 1 2011', ETE )
///           CALL WNINSD ( ETS, ETE, CNFINE  )
///
///     C
///     C     The moon orbit about the earth-moon barycenter is
///     C     twenty-eight days. The event condition occurs
///     C     during (very) approximately a quarter of the orbit. Use
///     C     a step of five days.
///     C
///           STEP = 5.D0 * SPD()
///
///           CALL GFUDB ( UDF, GFB, STEP, CNFINE, RESULT )
///
///           IF ( WNCARD(RESULT) .EQ. 0 ) THEN
///
///                 WRITE (*, '(A)') 'Result window is empty.'
///
///           ELSE
///
///              DO I = 1, WNCARD(RESULT)
///
///     C
///     C           Fetch and display each RESULT interval.
///     C
///                 CALL WNFETD ( RESULT, I, LEFT, RIGHT )
///                 WRITE (*,*) 'Interval ', I
///
///                 CALL ET2UTC ( LEFT, 'C', 4, UTC )
///                 WRITE (*, *) '   Interval start: ', UTC
///
///                 CALL SPKEZ ( 301, LEFT, 'IAU_EARTH', 'NONE', 399,
///          .                   STATE, LT )
///                 WRITE (*, *) '                Z= ', STATE(3)
///                 WRITE (*, *) '               Vz= ', STATE(6)
///
///                 CALL ET2UTC ( RIGHT, 'C', 4, UTC )
///                 WRITE (*, *) '   Interval end  : ', UTC
///
///                 CALL SPKEZ ( 301, RIGHT, 'IAU_EARTH', 'NONE', 399,
///          .                   STATE, LT )
///                 WRITE (*, *) '                Z= ', STATE(3)
///                 WRITE (*, *) '               Vz= ', STATE(6)
///                 WRITE (*, *) ' '
///
///              END DO
///
///           END IF
///
///           END
///
///
///
///     C-Procedure GFB
///     C
///     C     User defined boolean routine.
///     C
///
///           SUBROUTINE GFB ( UDFUNS, ET, BOOL )
///           IMPLICIT NONE
///
///     C- Abstract
///     C
///     C     User defined geometric boolean function:
///     C
///     C        Z >= 0 with dZ/dt > 0.
///     C
///
///           EXTERNAL              UDFUNS
///
///           DOUBLE PRECISION      ET
///           LOGICAL               BOOL
///
///     C
///     C     Local variables.
///     C
///           INTEGER               TARG
///           INTEGER               OBS
///
///           CHARACTER*(12)        REF
///           CHARACTER*(12)        ABCORR
///
///           DOUBLE PRECISION      STATE ( 6 )
///           DOUBLE PRECISION      LT
///
///     C
///     C     Initialization. Retrieve the vector from the earth to
///     C     the moon in the IAU_EARTH frame, without aberration
///     C     correction.
///     C
///           TARG   = 301
///           REF    = 'IAU_EARTH'
///           ABCORR = 'NONE'
///           OBS    = 399
///
///     C
///     C     Evaluate the state of TARG from OBS at ET with
///     C     correction ABCORR.
///     C
///           CALL SPKEZ ( TARG, ET, REF, ABCORR, OBS, STATE, LT )
///
///     C
///     C     Calculate the boolean value.
///     C
///           BOOL = (STATE(3) .GE. 0.D0) .AND. (STATE(6) .GT. 0.D0 )
///
///           RETURN
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Interval            1
///         Interval start: 2011 JAN 09 15:24:23.4165
///                      Z=   -1.1251040632487275E-007
///                     Vz=   0.39698408454587081
///         Interval end  : 2011 JAN 16 16:08:28.5642
///                      Z=    156247.48804193645
///                     Vz=    4.0992339730983041E-013
///
///      Interval            2
///         Interval start: 2011 FEB 05 23:17:57.3600
///                      Z=   -1.2467506849134224E-007
///                     Vz=   0.39678128284337311
///         Interval end  : 2011 FEB 13 01:38:28.4265
///                      Z=    157016.05500077485
///                     Vz=    1.7374578338558155E-013
///
///      Interval            3
///         Interval start: 2011 MAR 05 06:08:17.6689
///                      Z=   -7.7721836078126216E-008
///                     Vz=   0.39399025363429169
///         Interval end  : 2011 MAR 12 10:27:45.1896
///                      Z=    157503.77377718856
///                     Vz=   -2.9786351336824612E-013
///
///
///  2) Calculate the time intervals when the Z component of the
///     Earth to Moon position vector in the IAU_EARTH frame has
///     value between -1000 km and 1000 km (e.g. above and below
///     the equatorial plane).
///
///     Use the meta-kernel from the first example.
///
///
///     Example code begins here.
///
///
///           PROGRAM GFUDB_EX2
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions.
///     C
///           INTEGER               WNCARD
///           DOUBLE PRECISION      SPD
///
///     C
///     C     User defined external routines
///     C
///           EXTERNAL              GFB
///           EXTERNAL              GFQ
///
///     C
///     C     Local parameters
///     C
///           INTEGER               LBCELL
///           PARAMETER           ( LBCELL = -5 )
///
///     C
///     C     Use the parameter MAXWIN for both the result window size
///     C     and the workspace size.
///     C
///           INTEGER               MAXWIN
///           PARAMETER           ( MAXWIN = 100 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(32)        UTC
///
///           DOUBLE PRECISION      LEFT
///           DOUBLE PRECISION      RIGHT
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      ETS
///           DOUBLE PRECISION      ETE
///           DOUBLE PRECISION      LT
///           DOUBLE PRECISION      STEP
///           DOUBLE PRECISION      POS (3)
///           DOUBLE PRECISION      CNFINE ( LBCELL : 2      )
///           DOUBLE PRECISION      RESULT ( LBCELL : MAXWIN )
///
///           INTEGER               I
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
///     C     Load needed kernels.
///     C
///           CALL FURNSH ( 'gfudb_ex1.tm' )
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
///           CALL STR2ET ( 'Jan 1 2011', ETS )
///           CALL STR2ET ( 'Apr 1 2011', ETE )
///           CALL WNINSD ( ETS, ETE, CNFINE )
///
///     C
///     C     The duration of the event is approximately ninety
///     C     minutes. Use a step of one hour.
///     C
///           STEP = 60.D0*60.D0
///
///           CALL GFUDB ( GFQ, GFB, STEP, CNFINE, RESULT )
///
///           IF ( WNCARD(RESULT) .EQ. 0 ) THEN
///
///                 WRITE (*, '(A)') 'Result window is empty.'
///
///           ELSE
///
///              DO I = 1, WNCARD(RESULT)
///
///     C
///     C           Fetch and display each RESULT interval.
///     C
///                 CALL WNFETD ( RESULT, I, LEFT, RIGHT )
///                 WRITE (*,*) 'Interval ', I
///
///                 CALL ET2UTC ( LEFT, 'C', 4, UTC )
///                 WRITE (*, *) '   Interval start: ', UTC
///
///                 CALL SPKEZP ( 301, LEFT, 'IAU_EARTH', 'NONE', 399,
///          .                   POS, LT )
///                 WRITE (*, *) '                Z= ', POS(3)
///
///                 CALL ET2UTC ( RIGHT, 'C', 4, UTC )
///                 WRITE (*, *) '   Interval end  : ', UTC
///
///                 CALL SPKEZP ( 301, RIGHT, 'IAU_EARTH', 'NONE', 399,
///          .                   POS, LT )
///                 WRITE (*, *) '                Z= ', POS(3)
///                 WRITE (*, *) ' '
///
///              END DO
///
///           END IF
///
///           END
///
///
///
///     C-Procedure GFQ
///     C
///     C     User defined scalar routine.
///     C
///
///           SUBROUTINE GFQ ( ET, VALUE )
///           IMPLICIT NONE
///
///     C- Abstract
///     C
///     C     Return the Z component of the POS vector.
///     C
///
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      VALUE
///
///     C
///     C     Local variables.
///     C
///           INTEGER               TARG
///           INTEGER               OBS
///
///           CHARACTER*(12)        REF
///           CHARACTER*(12)        ABCORR
///
///           DOUBLE PRECISION      POS ( 3 )
///           DOUBLE PRECISION      LT
///
///     C
///     C     Initialization. Retrieve the vector from the earth to
///     C     the moon in the IAU_EARTH frame, without aberration
///     C     correction.
///     C
///           TARG   = 301
///           REF    = 'IAU_EARTH'
///           ABCORR = 'NONE'
///           OBS    = 399
///
///     C
///     C     Evaluate the position of TARG from OBS at ET with
///     C     correction ABCORR.
///     C
///           CALL SPKEZP ( TARG, ET, REF, ABCORR, OBS, POS, LT )
///
///           VALUE = POS(3)
///
///           RETURN
///           END
///
///
///
///     C-Procedure GFB
///     C
///     C     User defined boolean routine.
///     C
///
///           SUBROUTINE GFB ( UDFUNS, ET, BOOL )
///           IMPLICIT NONE
///
///     C- Abstract
///     C
///     C     User defined boolean function:
///     C
///     C        VALUE >= LIM1 with VALUE <= LIM2.
///     C
///
///           EXTERNAL              UDFUNS
///
///           DOUBLE PRECISION      ET
///           LOGICAL               BOOL
///           DOUBLE PRECISION      VALUE
///
///
///           DOUBLE PRECISION      LIM1
///           DOUBLE PRECISION      LIM2
///
///           LIM1 = -1000.D0
///           LIM2 =  1000.D0
///
///           CALL UDFUNS ( ET, VALUE )
///
///     C
///     C     Calculate the boolean value.
///     C
///           BOOL = (VALUE .GE. LIM1) .AND. (VALUE .LE. LIM2 )
///
///           RETURN
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Interval            1
///         Interval start: 2011 JAN 09 14:42:24.4855
///                      Z=   -999.99999984083206
///         Interval end  : 2011 JAN 09 16:06:22.5030
///                      Z=    999.99999987627757
///
///      Interval            2
///         Interval start: 2011 JAN 23 04:07:44.4563
///                      Z=    999.99999992179255
///         Interval end  : 2011 JAN 23 05:23:06.2446
///                      Z=   -1000.0000001340870
///
///      Interval            3
///         Interval start: 2011 FEB 05 22:35:57.1570
///                      Z=   -1000.0000000961383
///         Interval end  : 2011 FEB 05 23:59:57.7497
///                      Z=    999.99999984281567
///
///      Interval            4
///         Interval start: 2011 FEB 19 14:11:28.2944
///                      Z=    1000.0000000983686
///         Interval end  : 2011 FEB 19 15:26:01.7199
///                      Z=   -999.99999985420800
///
///      Interval            5
///         Interval start: 2011 MAR 05 05:25:59.5621
///                      Z=   -1000.0000000277355
///         Interval end  : 2011 MAR 05 06:50:35.8628
///                      Z=    1000.0000000934349
///
///      Interval            6
///         Interval start: 2011 MAR 19 01:30:19.1660
///                      Z=    999.99999982956138
///         Interval end  : 2011 MAR 19 02:45:21.1121
///                      Z=   -1000.0000000146936
///
///
///     Note that the default convergence tolerance for the GF system
///     has value 10^-6 seconds.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  Any kernel files required by this routine must be loaded
///      (normally via the SPICELIB routine FURNSH) before this routine
///      is called.
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
/// -    SPICELIB Version 1.0.1, 21-OCT-2021 (JDR) (NJB)
///
///         Edited the header to comply with NAIF standard.
///
///         Added "IMPLICIT NONE" to example code and declared "LT"
///         variable. Reduced the search interval to limit the length of
///         the solutions. Added SAVE statements for CNFINE and RESULT
///         variables in code examples.
///
///         Updated description of RESULT argument in $Brief_I/O,
///         $Detailed_Input and $Detailed_Output.
///
///         Added entry #3 in $Exceptions section.
///
///         Updated header to describe use of expanded confinement window.
///
/// -    SPICELIB Version 1.0.0, 15-JUL-2014 (EDW) (NJB)
/// ```
pub fn gfudb(
    ctx: &mut SpiceContext,
    udfuns: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    udfunb: fn(
        fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
        &mut f64,
        &mut bool,
        &mut Context,
    ) -> f2rust_std::Result<()>,
    step: f64,
    cnfine: &[f64],
    result: &mut [f64],
) -> crate::Result<()> {
    GFUDB(udfuns, udfunb, step, cnfine, result, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure GFUDB ( GF, user defined boolean )
pub fn GFUDB(
    UDFUNS: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    UDFUNB: fn(
        fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
        &mut f64,
        &mut bool,
        &mut Context,
    ) -> f2rust_std::Result<()>,
    STEP: f64,
    CNFINE: &[f64],
    RESULT: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let CNFINE = DummyArray::new(CNFINE, LBCELL..);
    let mut RESULT = DummyArrayMut::new(RESULT, LBCELL..);
    let mut TOL: f64 = 0.0;
    let mut OK: bool = false;

    //
    // SPICELIB functions.
    //

    //
    // Local variables.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"GFUDB", ctx)?;

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
        CHKOUT(b"GFUDB", ctx)?;
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
    // Initialize the RESULT window to empty.
    //
    SCARDD(0, RESULT.as_slice_mut(), ctx)?;

    ZZGFUDB(
        UDFUNS,
        UDFUNB,
        TOL,
        GFSTEP,
        GFREFN,
        NORPT,
        GFREPI,
        GFREPU,
        GFREPF,
        NOBAIL,
        GFBAIL,
        CNFINE.as_slice(),
        RESULT.as_slice_mut(),
        ctx,
    )?;

    CHKOUT(b"GFUDB", ctx)?;

    Ok(())
}
