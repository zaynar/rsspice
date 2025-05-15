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
const RNAME: &[u8] = b"SPKEZ";
const FRNMLN: i32 = 32;

struct SaveVars {
    PRVCOR: Vec<u8>,
    DLT: f64,
    DLTCTR: f64,
    LTCENT: f64,
    STATE: StackArray<f64, 6>,
    STOBS: StackArray<f64, 6>,
    TEMP: StackArray<f64, 6>,
    XFORM: StackArray2D<f64, 36>,
    CENTER: i32,
    FJ2000: i32,
    LTSIGN: i32,
    REQFRM: i32,
    TYPE: i32,
    TYPEID: i32,
    ATTBLK: StackArray<bool, 15>,
    FOUND: bool,
    USEGEO: bool,
    XMIT: bool,
    SVCTR1: StackArray<i32, 2>,
    SVREF: Vec<u8>,
    SVREQF: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PRVCOR = vec![b' '; CORLEN as usize];
        let mut DLT: f64 = 0.0;
        let mut DLTCTR: f64 = 0.0;
        let mut LTCENT: f64 = 0.0;
        let mut STATE = StackArray::<f64, 6>::new(1..=6);
        let mut STOBS = StackArray::<f64, 6>::new(1..=6);
        let mut TEMP = StackArray::<f64, 6>::new(1..=6);
        let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
        let mut CENTER: i32 = 0;
        let mut FJ2000: i32 = 0;
        let mut LTSIGN: i32 = 0;
        let mut REQFRM: i32 = 0;
        let mut TYPE: i32 = 0;
        let mut TYPEID: i32 = 0;
        let mut ATTBLK = StackArray::<bool, 15>::new(1..=NABCOR);
        let mut FOUND: bool = false;
        let mut USEGEO: bool = false;
        let mut XMIT: bool = false;
        let mut SVCTR1 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVREF = vec![b' '; FRNMLN as usize];
        let mut SVREQF: i32 = 0;
        let mut FIRST: bool = false;

        FIRST = true;
        fstr::assign(&mut PRVCOR, b" ");

        Self {
            PRVCOR,
            DLT,
            DLTCTR,
            LTCENT,
            STATE,
            STOBS,
            TEMP,
            XFORM,
            CENTER,
            FJ2000,
            LTSIGN,
            REQFRM,
            TYPE,
            TYPEID,
            ATTBLK,
            FOUND,
            USEGEO,
            XMIT,
            SVCTR1,
            SVREF,
            SVREQF,
            FIRST,
        }
    }
}

/// S/P Kernel, easy reader
///
/// Return the state (position and velocity) of a target body
/// relative to an observing body, optionally corrected for light
/// time (planetary aberration) and stellar aberration.
///
/// # Required Reading
///
/// * [SPK](crate::required_reading::spk)
/// * [NAIF_IDS](crate::required_reading::naif_ids)
/// * [FRAMES](crate::required_reading::frames)
/// * [TIME](crate::required_reading::time)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  TARG       I   Target body.
///  ET         I   Observer epoch.
///  REF        I   Reference frame of output state vector.
///  ABCORR     I   Aberration correction flag.
///  OBS        I   Observing body.
///  STARG      O   State of target.
///  LT         O   One way light time between observer and target.
/// ```
///
/// # Detailed Input
///
/// ```text
///  TARG     is the NAIF ID code for a target body. The target
///           and observer define a state vector whose position
///           component points from the observer to the target.
///
///  ET       is the ephemeris time, expressed as seconds past J2000
///           TDB, at which the state of the target body relative to
///           the observer is to be computed. ET refers to time at
///           the observer's location.
///
///  REF      is the name of the reference frame relative to which
///           the output state vector should be expressed. This may
///           be any frame supported by the SPICE system, including
///           built-in frames (documented in the Frames Required
///           Reading) and frames defined by a loaded frame kernel
///           (FK).
///
///           When REF designates a non-inertial frame, the
///           orientation of the frame is evaluated at an epoch
///           dependent on the selected aberration correction.
///           See the description of the output state vector STARG
///           for details.
///
///  ABCORR   indicates the aberration corrections to be applied
///           to the state of the target body to account for one-way
///           light time and stellar aberration. See the discussion
///           in the $Particulars section for recommendations on
///           how to choose aberration corrections.
///
///           ABCORR may be any of the following:
///
///              'NONE'     Apply no correction. Return the
///                         geometric state of the target body
///                         relative to the observer.
///
///           The following values of ABCORR apply to the
///           "reception" case in which photons depart from the
///           target's location at the light-time corrected epoch
///           ET-LT and *arrive* at the observer's location at ET:
///
///              'LT'       Correct for one-way light time (also
///                         called "planetary aberration") using a
///                         Newtonian formulation. This correction
///                         yields the state of the target at the
///                         moment it emitted photons arriving at
///                         the observer at ET.
///
///                         The light time correction uses an
///                         iterative solution of the light time
///                         equation (see $Particulars for details).
///                         The solution invoked by the 'LT' option
///                         uses one iteration.
///
///              'LT+S'     Correct for one-way light time and
///                         stellar aberration using a Newtonian
///                         formulation. This option modifies the
///                         state obtained with the 'LT' option to
///                         account for the observer's velocity
///                         relative to the solar system
///                         barycenter. The result is the apparent
///                         state of the target---the position and
///                         velocity of the target as seen by the
///                         observer.
///
///              'CN'       Converged Newtonian light time
///                         correction. In solving the light time
///                         equation, the 'CN' correction iterates
///                         until the solution converges (three
///                         iterations on all supported platforms).
///                         Whether the 'CN+S' solution is
///                         substantially more accurate than the
///                         'LT' solution depends on the geometry
///                         of the participating objects and on the
///                         accuracy of the input data. In all
///                         cases this routine will execute more
///                         slowly when a converged solution is
///                         computed. See the $Particulars section
///                         below for a discussion of precision of
///                         light time corrections.
///
///              'CN+S'     Converged Newtonian light time
///                         correction and stellar aberration
///                         correction.
///
///
///           The following values of ABCORR apply to the
///           "transmission" case in which photons *depart* from
///           the observer's location at ET and arrive at the
///           target's location at the light-time corrected epoch
///           ET+LT:
///
///              'XLT'      "Transmission" case: correct for
///                         one-way light time using a Newtonian
///                         formulation. This correction yields the
///                         state of the target at the moment it
///                         receives photons emitted from the
///                         observer's location at ET.
///
///              'XLT+S'    "Transmission" case: correct for
///                         one-way light time and stellar
///                         aberration using a Newtonian
///                         formulation  This option modifies the
///                         state obtained with the 'XLT' option to
///                         account for the observer's velocity
///                         relative to the solar system
///                         barycenter. The position component of
///                         the computed target state indicates the
///                         direction that photons emitted from the
///                         observer's location must be "aimed" to
///                         hit the target.
///
///              'XCN'      "Transmission" case: converged
///                         Newtonian light time correction.
///
///              'XCN+S'    "Transmission" case: converged
///                         Newtonian light time correction and
///                         stellar aberration correction.
///
///
///           Neither special nor general relativistic effects are
///           accounted for in the aberration corrections applied
///           by this routine.
///
///           Case and blanks are not significant in the string
///           ABCORR.
///
///  OBS      is the NAIF ID code for an observing body.
/// ```
///
/// # Detailed Output
///
/// ```text
///  STARG    is a Cartesian state vector representing the position
///           and velocity of the target body relative to the
///           specified observer. STARG is corrected for the
///           specified aberrations, and is expressed with respect
///           to the reference frame specified by REF. The first
///           three components of STARG represent the x-, y- and
///           z-components of the target's position; the last three
///           components form the corresponding velocity vector.
///
///           The position component of STARG points from the
///           observer's location at ET to the aberration-corrected
///           location of the target. Note that the sense of the
///           position vector is independent of the direction of
///           radiation travel implied by the aberration
///           correction.
///
///           The velocity component of STARG is the derivative
///           with respect to time of the position component of
///           STARG.
///
///           Units are always km and km/sec.
///
///           Non-inertial frames are treated as follows: letting
///           LTCENT be the one-way light time between the observer
///           and the central body associated with the frame, the
///           orientation of the frame is evaluated at ET-LTCENT,
///           ET+LTCENT, or ET depending on whether the requested
///           aberration correction is, respectively, for received
///           radiation, transmitted radiation, or is omitted.
///           LTCENT is computed using the method indicated by
///           ABCORR.
///
///  LT       is the one-way light time between the observer and
///           target in seconds. If the target state is corrected
///           for aberrations, then LT is the one-way light time
///           between the observer and the light time corrected
///           target location.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the reference frame REF is not a recognized reference
///      frame, the error SPICE(UNKNOWNFRAME) is signaled.
///
///  2)  If the loaded kernels provide insufficient data to compute the
///      requested state vector, an error is signaled by a routine in
///      the call tree of this routine.
///
///  3)  If an error occurs while reading an SPK or other kernel file,
///      the error  is signaled by a routine in the call tree
///      of this routine.
///
///  4)  If any of the required attributes of the reference frame REF
///      cannot be determined, the error SPICE(UNKNOWNFRAME2) is
///      signaled.
/// ```
///
/// # Files
///
/// ```text
///  This routine computes states using SPK files that have been
///  loaded into the SPICE system, normally via the kernel loading
///  interface routine FURNSH. See the routine FURNSH and the SPK
///  and KERNEL Required Reading for further information on loading
///  (and unloading) kernels.
///
///  If the output state STARG is to be expressed relative to a
///  non-inertial frame, or if any of the ephemeris data used to
///  compute STARG are expressed relative to a non-inertial frame in
///  the SPK files providing those data, additional kernels may be
///  needed to enable the reference frame transformations required to
///  compute the state. Normally these additional kernels are PCK
///  files or frame kernels. Any such kernels must already be loaded
///  at the time this routine is called.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is part of the user interface to the SPICE ephemeris
///  system. It allows you to retrieve state information for any
///  ephemeris object relative to any other in a reference frame that
///  is convenient for further computations.
///
///
///  Aberration corrections
///  ======================
///
///  In space science or engineering applications one frequently
///  wishes to know where to point a remote sensing instrument, such
///  as an optical camera or radio antenna, in order to observe or
///  otherwise receive radiation from a target. This pointing problem
///  is complicated by the finite speed of light: one needs to point
///  to where the target appears to be as opposed to where it actually
///  is at the epoch of observation. We use the adjectives
///  "geometric," "uncorrected," or "true" to refer to an actual
///  position or state of a target at a specified epoch. When a
///  geometric position or state vector is modified to reflect how it
///  appears to an observer, we describe that vector by any of the
///  terms "apparent," "corrected," "aberration corrected," or "light
///  time and stellar aberration corrected." The SPICE Toolkit can
///  correct for two phenomena affecting the apparent location of an
///  object: one-way light time (also called "planetary aberration")
///  and stellar aberration.
///
///  One-way light time
///  ------------------
///
///  Correcting for one-way light time is done by computing, given an
///  observer and observation epoch, where a target was when the
///  observed photons departed the target's location. The vector from
///  the observer to this computed target location is called a "light
///  time corrected" vector. The light time correction depends on the
///  motion of the target relative to the solar system barycenter, but
///  it is independent of the velocity of the observer relative to the
///  solar system barycenter. Relativistic effects such as light
///  bending and gravitational delay are not accounted for in the
///  light time correction performed by this routine.
///
///  Stellar aberration
///  ------------------
///
///  The velocity of the observer also affects the apparent location
///  of a target: photons arriving at the observer are subject to a
///  "raindrop effect" whereby their velocity relative to the observer
///  is, using a Newtonian approximation, the photons' velocity
///  relative to the solar system barycenter minus the velocity of the
///  observer relative to the solar system barycenter. This effect is
///  called "stellar aberration." Stellar aberration is independent
///  of the velocity of the target. The stellar aberration formula
///  used by this routine does not include (the much smaller)
///  relativistic effects.
///
///  Stellar aberration corrections are applied after light time
///  corrections: the light time corrected target position vector is
///  used as an input to the stellar aberration correction.
///
///  When light time and stellar aberration corrections are both
///  applied to a geometric position vector, the resulting position
///  vector indicates where the target "appears to be" from the
///  observer's location.
///
///  As opposed to computing the apparent position of a target, one
///  may wish to compute the pointing direction required for
///  transmission of photons to the target. This also requires
///  correction of the geometric target position for the effects of
///  light time and stellar aberration, but in this case the
///  corrections are computed for radiation traveling *from* the
///  observer to the target.
///
///  The "transmission" light time correction yields the target's
///  location as it will be when photons emitted from the observer's
///  location at ET arrive at the target. The transmission stellar
///  aberration correction is the inverse of the traditional stellar
///  aberration correction: it indicates the direction in which
///  radiation should be emitted so that, using a Newtonian
///  approximation, the sum of the velocity of the radiation relative
///  to the observer and of the observer's velocity, relative to the
///  solar system barycenter, yields a velocity vector that points in
///  the direction of the light time corrected position of the target.
///
///  One may object to using the term "observer" in the transmission
///  case, in which radiation is emitted from the observer's location.
///  The terminology was retained for consistency with earlier
///  documentation.
///
///  Below, we indicate the aberration corrections to use for some
///  common applications:
///
///
///     1) Find the apparent direction of a target for a remote-sensing
///        observation.
///
///           Use 'LT+S' or 'CN+S: apply both light time and stellar
///           aberration corrections.
///
///        Note that using light time corrections alone ('LT' or 'CN')
///        is generally not a good way to obtain an approximation to
///        an apparent target vector: since light time and stellar
///        aberration corrections often partially cancel each other,
///        it may be more accurate to use no correction at all than to
///        use light time alone.
///
///
///     2) Find the corrected pointing direction to radiate a signal
///        to a target. This computation is often applicable for
///        implementing communications sessions.
///
///           Use 'XLT+S' or 'XCN+S: apply both light time and stellar
///           aberration corrections for transmission.
///
///
///     3) Compute the apparent position of a target body relative
///        to a star or other distant object.
///
///           Use 'LT', 'CN', 'LT+S', or 'CN+S' as needed to match the
///           correction applied to the position of the distant
///           object. For example, if a star position is obtained from
///           a catalog, the position vector may not be corrected for
///           stellar aberration. In this case, to find the angular
///           separation of the star and the limb of a planet, the
///           vector from the observer to the planet should be
///           corrected for light time but not stellar aberration.
///
///
///     4) Obtain an uncorrected state vector derived directly from
///        data in an SPK file.
///
///           Use 'NONE'.
///
///
///     5) Use a geometric state vector as a low-accuracy estimate
///        of the apparent state for an application where execution
///        speed is critical.
///
///           Use 'NONE'.
///
///
///     6) While this routine cannot perform the relativistic
///        aberration corrections required to compute states
///        with the highest possible accuracy, it can supply the
///        geometric states required as inputs to these computations.
///
///           Use 'NONE', then apply relativistic aberration
///           corrections (not available in the SPICE Toolkit).
///
///
///  Below, we discuss in more detail how the aberration corrections
///  applied by this routine are computed.
///
///     Geometric case
///     ==============
///
///     SPKEZ begins by computing the geometric position T(ET) of the
///     target body relative to the solar system barycenter (SSB).
///     Subtracting the geometric position of the observer O(ET) gives
///     the geometric position of the target body relative to the
///     observer. The one-way light time, LT, is given by
///
///               | T(ET) - O(ET) |
///        LT = -------------------
///                       c
///
///     The geometric relationship between the observer, target, and
///     solar system barycenter is as shown:
///
///
///        SSB ---> O(ET)
///         |      /
///         |     /
///         |    /
///         |   /  T(ET) - O(ET)
///         V  V
///        T(ET)
///
///
///     The returned state consists of the position vector
///
///        T(ET) - O(ET)
///
///     and a velocity obtained by taking the difference of the
///     corresponding velocities. In the geometric case, the
///     returned velocity is actually the time derivative of the
///     position.
///
///
///     Reception case
///     ==============
///
///     When any of the options 'LT', 'CN', 'LT+S', 'CN+S' is selected
///     for ABCORR, SPKEZ computes the position of the target body at
///     epoch ET-LT, where LT is the one-way light time. Let T(t) and
///     O(t) represent the positions of the target and observer
///     relative to the solar system barycenter at time t; then LT is
///     the solution of the light-time equation
///
///               | T(ET-LT) - O(ET) |
///        LT = ------------------------                            (1)
///                        c
///
///     The ratio
///
///         | T(ET) - O(ET) |
///       ---------------------                                     (2)
///                 c
///
///     is used as a first approximation to LT; inserting (2) into the
///     right hand side of the light-time equation (1) yields the
///     "one-iteration" estimate of the one-way light time ("LT").
///     Repeating the process until the estimates of LT converge
///     yields the "converged Newtonian" light time estimate ("CN").
///
///     Subtracting the geometric position of the observer O(ET) gives
///     the position of the target body relative to the observer:
///     T(ET-LT) - O(ET).
///
///        SSB ---> O(ET)
///         | \     |
///         |  \    |
///         |   \   | T(ET-LT) - O(ET)
///         |    \  |
///         V     V V
///        T(ET)  T(ET-LT)
///
///     The position component of the light time corrected state
///     is the vector
///
///        T(ET-LT) - O(ET)
///
///     The velocity component of the light time corrected state
///     is the difference
///
///        T_vel(ET-LT)*(1-dLT/dET) - O_vel(ET)
///
///     where T_vel and O_vel are, respectively, the velocities of the
///     target and observer relative to the solar system barycenter at
///     the epochs ET-LT and ET.
///
///     If correction for stellar aberration is requested, the target
///     position is rotated toward the solar system barycenter-
///     relative velocity vector of the observer. The rotation is
///     computed as follows:
///
///        Let r be the light time corrected vector from the observer
///        to the object, and v be the velocity of the observer with
///        respect to the solar system barycenter. Let w be the angle
///        between them. The aberration angle phi is given by
///
///           sin(phi) = v sin(w) / c
///
///        Let h be the vector given by the cross product
///
///           h = r X v
///
///        Rotate r by phi radians about h to obtain the apparent
///        position of the object.
///
///     When stellar aberration corrections are used, the rate of
///     change of the stellar aberration correction is accounted for
///     in the computation of the output velocity.
///
///
///     Transmission case
///     ==================
///
///     When any of the options 'XLT', 'XCN', 'XLT+S', 'XCN+S' is
///     selected, SPKEZ computes the position of the target body T at
///     epoch ET+LT, where LT is the one-way light time. LT is the
///     solution of the light-time equation
///
///               | T(ET+LT) - O(ET) |
///        LT = ------------------------                            (3)
///                         c
///
///     Subtracting the geometric position of the observer, O(ET),
///     gives the position of the target body relative to the
///     observer: T(ET-LT) - O(ET).
///
///                SSB --> O(ET)
///               / |    *
///              /  |  *  T(ET+LT) - O(ET)
///             /   |*
///            /   *|
///           V  V  V
///       T(ET+LT)  T(ET)
///
///     The position component of the light-time corrected state
///     is the vector
///
///        T(ET+LT) - O(ET)
///
///     The velocity component of the light-time corrected state
///     consists of the difference
///
///        T_vel(ET+LT)*(1+dLT/dET) - O_vel(ET)
///
///     where T_vel and O_vel are, respectively, the velocities of the
///     target and observer relative to the solar system barycenter at
///     the epochs ET+LT and ET.
///
///     If correction for stellar aberration is requested, the target
///     position is rotated away from the solar system barycenter-
///     relative velocity vector of the observer. The rotation is
///     computed as in the reception case, but the sign of the
///     rotation angle is negated. Velocities are adjusted to account
///     for the rate of change of the stellar aberration correction.
///
///
///  Precision of light time corrections
///  ===================================
///
///     Corrections using one iteration of the light time solution
///     ----------------------------------------------------------
///
///     When the requested aberration correction is 'LT', 'LT+S',
///     'XLT', or 'XLT+S', only one iteration is performed in the
///     algorithm used to compute LT.
///
///     The relative error in this computation
///
///        | LT_ACTUAL - LT_COMPUTED |  /  LT_ACTUAL
///
///     is at most
///
///         (V/C)**2
///        ----------
///         1 - (V/C)
///
///     which is well approximated by (V/C)**2, where V is the
///     velocity of the target relative to an inertial frame and C is
///     the speed of light.
///
///     For nearly all objects in the solar system V is less than 60
///     km/sec. The value of C is ~300000 km/sec. Thus the
///     one-iteration solution for LT has a potential relative error
///     of not more than 4e-8. This is a potential light time error of
///     approximately 2e-5 seconds per astronomical unit of distance
///     separating the observer and target. Given the bound on V cited
///     above:
///
///        As long as the observer and target are separated by less
///        than 50 astronomical units, the error in the light time
///        returned using the one-iteration light time corrections is
///        less than 1 millisecond.
///
///        The magnitude of the corresponding position error, given
///        the above assumptions, may be as large as (V/C)**2 * the
///        distance between the observer and the uncorrected target
///        position: 300 km or equivalently 6 km/AU.
///
///     In practice, the difference between positions obtained using
///     one-iteration and converged light time is usually much smaller
///     than the value computed above and can be insignificant. For
///     example, for the spacecraft Mars Reconnaissance Orbiter and
///     Mars Express, the position error for the one-iteration light
///     time correction, applied to the spacecraft-to-Mars center
///     vector, is at the 1 cm level.
///
///     Comparison of results obtained using the one-iteration and
///     converged light time solutions is recommended when adequacy of
///     the one-iteration solution is in doubt.
///
///
///     Converged corrections
///     ---------------------
///
///     When the requested aberration correction is 'CN', 'CN+S',
///     'XCN', or 'XCN+S', as many iterations as are required for
///     convergence are performed in the computation of LT. Usually
///     the solution is found after three iterations. The relative
///     error present in this case is at most
///
///         (V/C)**4
///        ----------
///         1 - (V/C)
///
///     which is well approximated by (V/C)**4.
///
///        The precision of this computation (ignoring round-off
///        error) is better than 4e-11 seconds for any pair of objects
///        less than 50 AU apart, and having speed relative to the
///        solar system barycenter less than 60 km/s.
///
///        The magnitude of the corresponding position error, given
///        the above assumptions, may be as large as (V/C)**4 * the
///        distance between the observer and the uncorrected target
///        position: 1.2 cm at 50 AU or equivalently 0.24 mm/AU.
///
///     However, to very accurately model the light time between
///     target and observer one must take into account effects due to
///     general relativity. These may be as high as a few hundredths
///     of a millisecond for some objects.
///
///
///  Relativistic Corrections
///  =========================
///
///  This routine does not attempt to perform either general or
///  special relativistic corrections in computing the various
///  aberration corrections. For many applications relativistic
///  corrections are not worth the expense of added computation
///  cycles. If however, your application requires these additional
///  corrections we suggest you consult the astronomical almanac (page
///  B36) for a discussion of how to carry out these corrections.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Load a planetary ephemeris SPK; then look up a series of
///      geometric states of the moon relative to the earth,
///      referenced to the J2000 frame.
///
///            IMPLICIT NONE
///      C
///      C     Local constants
///      C
///            CHARACTER*(*)         FRAME
///            PARAMETER           ( FRAME  = 'J2000' )
///
///            CHARACTER*(*)         ABCORR
///            PARAMETER           ( ABCORR = 'NONE' )
///
///      C
///      C     The name of the SPK file shown here is fictitious;
///      C     you must supply the name of an SPK file available
///      C     on your own computer system.
///      C
///            CHARACTER*(*)         SPK
///            PARAMETER           ( SPK    = 'planet.bsp' )
///
///      C
///      C     ET0 represents the date 2000 Jan 1 12:00:00 TDB.
///      C
///            DOUBLE PRECISION      ET0
///            PARAMETER           ( ET0    = 0.0D0 )
///
///      C
///      C     Use a time step of 1 hour; look up 100 states.
///      C
///            DOUBLE PRECISION      STEP
///            PARAMETER           ( STEP   = 3600.0D0 )
///
///            INTEGER               MAXITR
///            PARAMETER           ( MAXITR = 100 )
///
///      C
///      C     The NAIF IDs of the earth and moon are 399 and 301
///      C     respectively.
///      C
///            INTEGER               OBSRVR
///            PARAMETER           ( OBSRVR = 399 )
///
///            INTEGER               TARGET
///            PARAMETER           ( TARGET = 301 )
///
///      C
///      C     Local variables
///      C
///            DOUBLE PRECISION      ET
///            DOUBLE PRECISION      LT
///            DOUBLE PRECISION      STATE ( 6 )
///
///            INTEGER               I
///
///      C
///      C     Load the SPK file.
///      C
///            CALL FURNSH ( SPK )
///
///      C
///      C     Step through a series of epochs, looking up a
///      C     state vector at each one.
///      C
///            DO I = 1, MAXITR
///
///               ET = ET0 + (I-1)*STEP
///
///               CALL SPKEZ ( TARGET, ET, FRAME, ABCORR, OBSRVR,
///           .                STATE,  LT                        )
///
///               WRITE (*,*) 'ET = ', ET
///               WRITE (*,*) 'J2000 x-position (km):   ', STATE(1)
///               WRITE (*,*) 'J2000 y-position (km):   ', STATE(2)
///               WRITE (*,*) 'J2000 z-position (km):   ', STATE(3)
///               WRITE (*,*) 'J2000 x-velocity (km/s): ', STATE(4)
///               WRITE (*,*) 'J2000 y-velocity (km/s): ', STATE(5)
///               WRITE (*,*) 'J2000 z-velocity (km/s): ', STATE(6)
///               WRITE (*,*) ' '
///
///            END DO
///
///            END
/// ```
///
/// # Author and Institution
///
/// ```text
///  C.H. Acton         (JPL)
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  J.E. McLean        (JPL)
///  H.A. Neilan        (JPL)
///  B.V. Semenov       (JPL)
///  M.J. Spencer       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 5.1.1, 16-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 5.1.0, 03-JUL-2014 (NJB) (BVS)
///
///         Discussion of light time corrections was updated. Assertions
///         that converged light time corrections are unlikely to be
///         useful were removed.
///
///         Bug fix: replaced calls to ZZPRSCOR with calls to
///         ZZVALCOR. The latter routine rejects all aberration
///         corrections not supported by the SPK subsystem.
///
///         Bug fix: added a check and an exception for the FOUND flag
///         returned by FRINFO.
///
///         Updated to save the input frame name and POOL state counter
///         and to do frame name-ID conversion only if the counter has
///         changed.
///
///         Updated various in-line comments.
///
/// -    SPICELIB Version 5.0.1, 18-MAY-2010 (BVS)
///
///         Removed "C$" marker from text in the header.
///
/// -    SPICELIB Version 5.0.0, 27-DEC-2007 (NJB)
///
///         This routine was upgraded to more accurately compute
///         aberration-corrected velocity, and in particular, make it
///         more consistent with observer-target positions.
///
///         When light time corrections are used, the derivative of light
///         time with respect to time is now accounted for in the
///         computation of observer-target velocities. When the reference
///         frame associated with the output state is time-dependent, the
///         derivative of light time with respect to time is now accounted
///         for in the computation of the rate of change of orientation of
///         the reference frame.
///
///         When stellar aberration corrections are used, velocities
///         now reflect the rate of range of the stellar aberration
///         correction.
///
/// -    SPICELIB Version 4.1.0, 05-JAN-2005 (NJB)
///
///         Tests of routine FAILED() were added.
///         Minor header error was corrected.
///
/// -    SPICELIB Version 4.0.2, 20-OCT-2003 (EDW)
///
///         Added mention that LT returns in seconds.
///
/// -    SPICELIB Version 4.0.1, 29-JUL-2003 (NJB) (CHA)
///
///         Various minor header changes were made to improve clarity.
///
/// -    SPICELIB Version 4.0.0, 28-DEC-2001 (NJB)
///
///         Updated to handle aberration corrections for transmission
///         of radiation. Formerly, only the reception case was
///         supported. The header was revised and expanded to explain
///         the functionality of this routine in more detail.
///
/// -    SPICELIB Version 3.1.0, 09-JUL-1996 (WLT)
///
///         Corrected the description of LT in the Detailed Output
///         section of the header.
///
/// -    SPICELIB Version 3.0.0, 26-MAY-1995 (WLT)
///
///         The routine was upgraded to support non-inertial frames.
///
/// -    SPICELIB Version 2.1.1, 05-AUG-1994 (HAN) (MJS)
///
///         Added code so that routine accepts lower case, mixed case
///         and upper case versions of the string ABCORR.
///
/// -    SPICELIB Version 2.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 2.0.0, 18-JUL-1991 (JEM) (NJB)
///
///         The old SPKEZ did not compute the geometric state of one body
///         with respect to another unless data existed for each body with
///         respect to the solar system barycenter.
///
/// -    SPICELIB Version 1.0.1, 22-MAR-1990 (HAN)
///
///         Literature references added to the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 5.0.0, 27-DEC-2007 (NJB)
///
///         Routine was upgraded to more accurately compute aberration-
///         corrected velocity, and in particular, make it more consistent
///         with observer-target positions. When light time corrections
///         are used:
///
///            1) The derivative of light time with respect
///               to time is now accounted for in the computation
///               of observer-target velocities, for all types
///               of reference frames.
///
///            2) The derivative of light time with respect
///               to time is now accounted for in the computation of the
///               rate of change of orientation of time-dependent
///               reference frames for the output state. This rate of
///               change affects observer-target velocities.
///
///         When stellar aberration corrections are used, velocities
///         now reflect the rate of range of the stellar aberration
///         correction.
///
///         This routine was modified as follows:
///
///            - SPKAPP is no longer called; it has been superseded
///              by SPKACS. Aberration-corrected states relative to
///              inertial frames are computed by SPKACS.
///
///            - The effect of the rate of change of light time on the
///              rate of change of orientation of non-inertial output
///              frames is accounted for in this routine. See the code
///              near the end of this source file.
///
///         The header of this routine has been updated to reflect the
///         upgrades described here.
///
///         As a separate upgrade, the method by which the aberration
///         correction flag is parsed has been made more robust: parsing
///         is now done by the routine ZZZPRSCOR. The new parsing
///         technique calls for parsing the input string only when it
///         differs from the previous value.
///
/// -    SPICELIB Version 4.1.0, 05-JAN-2005 (NJB)
///
///         Tests of routine FAILED() were added. The new checks
///         are intended to prevent arithmetic operations from
///         being performed with uninitialized or invalid data.
///
///         Minor header error was corrected.
///
/// -    SPICELIB Version 3.1.0, 09-JUL-1996 (WLT)
///
///         Corrected the description of LT in the Detailed Output
///         section of the header.
///
/// -    SPICELIB Version 3.0.0, 26-MAY-1995 (WLT)
///
///         The routine was upgraded so that it can now support
///         non-inertial reference frames. In additions some
///         of the error messages were slightly enhanced.
///
/// -    SPICELIB Version 2.1.1, 5-AUG-1994 (HAN) (MJS)
///
///         Added code so that routine accepts lower case, mixed case
///         and upper case versions of the string ABCORR.
///
/// -    SPICELIB Version 2.0.0, 18-JUL-1991 (JEM) (NJB)
///
///         The previous version of SPKEZ could not
///         compute the geometric state (no aberration
///         correction) of one body with respect to
///         another if the ephemeris data for each
///         body relative to the Solar System Barycenter
///         (body 0) had not been loaded. Now, if
///         sufficient data is loaded, SPKEZ can always
///         compute the state.
///
///         For example, suppose the file GLL.BSP contains segments of SPK
///         data for the Galileo spacecraft (body -77) relative to the
///         Jupiter Barycenter (body 5) over a period of time. If the
///         previous version of SPKEZ was called to compute the geometric
///         state of -77 relative to 5 (or vice versa), a routine that
///         SPKEZ calls, SPKSSB, would signal an error stating that there
///         is insufficient data for computing the state of body 5
///         (relative to 0). Version 1.0.0 of SPKEZ could not compute the
///         requested state even though sufficient data had been loaded.
///
///         It is necessary to compute the states of each
///         of the target and observing bodies relative to
///         the solar system barycenter when aberration
///         corrections are being applied. However, when
///         computing geometric states, it is only necessary
///         to trace back to the first common node. Positive
///         side effects include the maintenance of precision
///         and reduction in number of look ups.
///
///         The changes to the code in SPKEZ involved calling a new
///         routine, SPKGEO, which computes the geometric state if
///         no aberration corrections are requested.
///
///         The other cosmetic changes include the removal of a reference
///         to the SPK User's Guide in $Literature_References because
///         the User's Guide is the same as SPK Required Reading.
///
///         Also, the item in $Restrictions previously said
///
///            1) The ephemeris files to be used by SPKEZ must be loaded
///               by SPKLEF before SPKSSB is called.
///
///         SPKSSB was replaced with SPKEZ.
///
///         The location of the position and velocity information in the
///         output state vector argument STARG is now spelled out.
///
///         Finally, the $Particulars section was updated. In the previous
///         version, it said that calling SPKEZ was equivalent to calling
///         SPKSSB and SPKAPP.
/// ```
pub fn spkez(
    ctx: &mut SpiceContext,
    targ: i32,
    et: f64,
    ref_: &str,
    abcorr: &str,
    obs: i32,
    starg: &mut [f64; 6],
    lt: &mut f64,
) -> crate::Result<()> {
    SPKEZ(
        targ,
        et,
        ref_.as_bytes(),
        abcorr.as_bytes(),
        obs,
        starg,
        lt,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKEZ ( S/P Kernel, easy reader )
pub fn SPKEZ(
    TARG: i32,
    ET: f64,
    REF: &[u8],
    ABCORR: &[u8],
    OBS: i32,
    STARG: &mut [f64],
    LT: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut STARG = DummyArrayMut::new(STARG, 1..=6);

    //
    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Saved frame name length.
    //

    //
    // Local variables
    //

    //
    // Saved frame name/ID item declarations.
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

    CHKIN(RNAME, ctx)?;

    //
    // Counter initialization is done separately.
    //
    if save.FIRST {
        //
        // Initialize counter.
        //
        ZZCTRUIN(save.SVCTR1.as_slice_mut(), ctx);
    }

    if (save.FIRST || fstr::ne(ABCORR, &save.PRVCOR)) {
        //
        // The aberration correction flag differs from the value it
        // had on the previous call, if any. Analyze the new flag.
        //
        ZZVALCOR(ABCORR, save.ATTBLK.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // The aberration correction flag is recognized; save it.
        //
        fstr::assign(&mut save.PRVCOR, ABCORR);

        //
        // Set logical flags indicating the attributes of the requested
        // correction:
        //
        //    XMIT is .TRUE. when the correction is for transmitted
        //    radiation.
        //
        //    USEGEO indicates geometric state computation.
        //
        // The above definitions are consistent with those used by
        // ZZVALCOR.
        //
        save.XMIT = save.ATTBLK[XMTIDX];
        save.USEGEO = save.ATTBLK[GEOIDX];

        //
        // Get the frame ID for J2000 on the first call to this routine.
        //
        if save.FIRST {
            NAMFRM(b"J2000", &mut save.FJ2000, ctx)?;

            save.FIRST = false;
        }
    }

    //
    // If we only want a geometric state, then use SPKGEO to compute
    // just that.
    //
    // Otherwise, if REF is inertial, compute the state of the target
    // relative to the observer via SPKACS. If REF is non-inertial,
    // compute the requested state in the J2000 frame, then transform it
    // to the frame designated by REF.
    //
    if save.USEGEO {
        SPKGEO(TARG, ET, REF, OBS, STARG.as_slice_mut(), LT, ctx)?;
    } else {
        //
        // Get the auxiliary information about the requested output
        // frame.
        //
        ZZNAMFRM(
            save.SVCTR1.as_slice_mut(),
            &mut save.SVREF,
            &mut save.SVREQF,
            REF,
            &mut save.REQFRM,
            ctx,
        )?;

        if (save.REQFRM == 0) {
            SETMSG(b"The requested output frame \'#\' is not recognized by the reference frame subsystem. Please check that the appropriate kernels have been loaded and that you have correctly entered the name of the output frame. ", ctx);
            ERRCH(b"#", REF, ctx);
            SIGERR(b"SPICE(UNKNOWNFRAME)", ctx)?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        FRINFO(
            save.REQFRM,
            &mut save.CENTER,
            &mut save.TYPE,
            &mut save.TYPEID,
            &mut save.FOUND,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        if !save.FOUND {
            SETMSG(b"The requested output frame \'#\' is not recognized by the reference frame subsystem. Please check that the appropriate kernels have been loaded and that you have correctly entered the name of the output frame. ", ctx);
            ERRCH(b"#", REF, ctx);
            SIGERR(b"SPICE(UNKNOWNFRAME2)", ctx)?;
            CHKOUT(RNAME, ctx)?;

            return Ok(());
        }

        //
        // If we are dealing with an inertial frame, we can simply
        // call SPKACS and return.
        //
        if (save.TYPE == INERTL) {
            SPKACS(
                TARG,
                ET,
                REF,
                ABCORR,
                OBS,
                STARG.as_slice_mut(),
                LT,
                &mut save.DLT,
                ctx,
            )?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }
        //
        // Still here?
        //
        // We are dealing with a non-inertial frame. But we need to do
        // light time and stellar aberration corrections in an inertial
        // frame. Get the "apparent" state of TARG in the intermediary
        // inertial reference frame J2000.
        //
        // We also need the light time to the center of the frame.
        // We compute that first so that we can re-use the temporary
        // variable STATE when we compute the inertial apparent state
        // of the target relative to the observer.
        //
        SPKACS(
            TARG,
            ET,
            b"J2000",
            ABCORR,
            OBS,
            save.STATE.as_slice_mut(),
            LT,
            &mut save.DLT,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        if (save.CENTER == OBS) {
            save.LTCENT = 0.0;
            save.DLTCTR = 0.0;
        } else if (save.CENTER == TARG) {
            save.LTCENT = *LT;
            save.DLTCTR = save.DLT;
        } else {
            SPKSSB(OBS, ET, b"J2000", save.STOBS.as_slice_mut(), ctx)?;
            SPKLTC(
                save.CENTER,
                ET,
                b"J2000",
                ABCORR,
                save.STOBS.as_slice(),
                save.TEMP.as_slice_mut(),
                &mut save.LTCENT,
                &mut save.DLTCTR,
                ctx,
            )?;
        }

        //
        // If something went wrong (like we couldn't get the state of
        // the center relative to the observer) now it is time to quit.
        //
        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // If the aberration corrections are for transmission, make the
        // sign of the light time positive, since we wish to compute the
        // orientation of the non-inertial frame at an epoch later than
        // ET by the one-way light time.
        //
        if save.XMIT {
            save.LTSIGN = 1;
        } else {
            save.LTSIGN = -1;
        }

        //
        // Get the state transformation from J2000 to the requested frame
        // and convert the state.
        //
        FRMCHG(
            save.FJ2000,
            save.REQFRM,
            (ET + ((save.LTSIGN as f64) * save.LTCENT)),
            save.XFORM.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // There's a tricky bit here:  since XFORM is evaluated
        // at time
        //
        //    ET + LTSIGN*LTCENT
        //
        // XFORM is actually dependent on LTCENT. We need to account for
        // this dependency in our velocity transformation.
        //
        // Let P and V be the target position and velocity respectively,
        // and R, DR be the rotation and rotation derivative
        // corresponding to XFORM.
        //
        // The state transformation we need to perform is not
        //
        //    R * V   +  DR * P
        //
        // but rather
        //
        //    R * V   +  ( (1 + LTSIGN*DLTCTR) * DR )  * P
        //
        // So we'll scale the derivative block of XFORM accordingly.
        //
        for I in 1..=3 {
            VSCLIP(
                (1.0 + ((save.LTSIGN as f64) * save.DLTCTR)),
                save.XFORM.subarray_mut([4, I]),
            );
        }

        //
        // Now apply the frame transformation XFORM to produce the
        // state expressed relative to the request frame REQFRM.
        //
        MXVG(
            save.XFORM.as_slice(),
            save.STATE.as_slice(),
            6,
            6,
            STARG.as_slice_mut(),
        );
    }

    CHKOUT(RNAME, ctx)?;
    Ok(())
}
