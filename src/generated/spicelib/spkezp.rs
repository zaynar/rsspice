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
const CTRSIZ: i32 = 2;
const RNAME: &[u8] = b"SPKEZP";
const FRNMLN: i32 = 32;

struct SaveVars {
    LTCENT: f64,
    SOBS: StackArray<f64, 6>,
    POSTN: StackArray<f64, 3>,
    TEMP: StackArray<f64, 3>,
    XFORM: StackArray2D<f64, 9>,
    CENTER: i32,
    FJ2000: i32,
    I: i32,
    REQFRM: i32,
    TYPE: i32,
    TYPEID: i32,
    FIRST: bool,
    FOUND: bool,
    XMIT: bool,
    SVCTR1: StackArray<i32, 2>,
    SVREF: Vec<u8>,
    SVREQF: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut LTCENT: f64 = 0.0;
        let mut SOBS = StackArray::<f64, 6>::new(1..=6);
        let mut POSTN = StackArray::<f64, 3>::new(1..=3);
        let mut TEMP = StackArray::<f64, 3>::new(1..=3);
        let mut XFORM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut CENTER: i32 = 0;
        let mut FJ2000: i32 = 0;
        let mut I: i32 = 0;
        let mut REQFRM: i32 = 0;
        let mut TYPE: i32 = 0;
        let mut TYPEID: i32 = 0;
        let mut FIRST: bool = false;
        let mut FOUND: bool = false;
        let mut XMIT: bool = false;
        let mut SVCTR1 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVREF = vec![b' '; FRNMLN as usize];
        let mut SVREQF: i32 = 0;

        FIRST = true;

        Self {
            LTCENT,
            SOBS,
            POSTN,
            TEMP,
            XFORM,
            CENTER,
            FJ2000,
            I,
            REQFRM,
            TYPE,
            TYPEID,
            FIRST,
            FOUND,
            XMIT,
            SVCTR1,
            SVREF,
            SVREQF,
        }
    }
}

/// S/P Kernel, easy position
///
/// Return the position of a target body relative to an observing
/// body, optionally corrected for light time (planetary aberration)
/// and stellar aberration.
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
///  TARG       I   Target body NAIF ID code.
///  ET         I   Observer epoch.
///  REF        I   Reference frame of output position vector.
///  ABCORR     I   Aberration correction flag.
///  OBS        I   Observing body NAIF ID code.
///  PTARG      O   Position of target.
///  LT         O   One way light time between observer and target.
/// ```
///
/// # Detailed Input
///
/// ```text
///  TARG     is the NAIF ID code for a target body. The target
///           and observer define a position vector which points
///           from the observer to the target.
///
///  ET       is the ephemeris time, expressed as seconds past
///           J2000 TDB, at which the position of the target body
///           relative to the observer is to be computed. ET
///           refers to time at the observer's location.
///
///  REF      is the name of the reference frame relative to which
///           the output position vector should be expressed. This
///           may be any frame supported by the SPICE system,
///           including built-in frames (documented in the Frames
///           Required Reading) and frames defined by a loaded
///           frame kernel (FK).
///
///           When REF designates a non-inertial frame, the
///           orientation of the frame is evaluated at an epoch
///           dependent on the selected aberration correction. See
///           the description of the output position vector PTARG
///           for details.
///
///  ABCORR   indicates the aberration corrections to be applied to
///           the position of the target body to account for
///           one-way light time and stellar aberration. See the
///           discussion in the $Particulars section for
///           recommendations on how to choose aberration
///           corrections.
///
///           ABCORR may be any of the following:
///
///              'NONE'     Apply no correction. Return the
///                         geometric position of the target body
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
///                         yields the position of the target at
///                         the moment it emitted photons arriving
///                         at the observer at ET.
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
///                         position obtained with the 'LT' option
///                         to account for the observer's velocity
///                         relative to the solar system
///                         barycenter. The result is the apparent
///                         position of the target---the position
///                         as seen by the observer.
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
///                         position of the target at the moment it
///                         receives photons emitted from the
///                         observer's location at ET.
///
///              'XLT+S'    "Transmission" case: correct for
///                         one-way light time and stellar
///                         aberration using a Newtonian
///                         formulation. This option modifies the
///                         position obtained with the 'XLT' option
///                         to account for the observer's velocity
///                         relative to the solar system
///                         barycenter. The computed target
///                         position indicates the direction that
///                         photons emitted from the observer's
///                         location must be "aimed" to hit the
///                         target.
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
///  OBS      is the NAIF ID code for the observing body.
/// ```
///
/// # Detailed Output
///
/// ```text
///  PTARG    is a Cartesian 3-vector representing the position of
///           the target body relative to the specified observer.
///           PTARG is corrected for the specified aberrations, and
///           is expressed with respect to the reference frame
///           specified by REF. The three components of PTARG
///           represent the x-, y- and z-components of the target's
///           position.
///
///           PTARG points from the observer's location at ET to
///           the aberration-corrected location of the target.
///           Note that the sense of this position vector is
///           independent of the direction of radiation travel
///           implied by the aberration correction.
///
///           Units are always km.
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
///           target in seconds. If the target position is
///           corrected for aberrations, then LT is the one-way
///           light time between the observer and the light time
///           corrected target location.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If name of target or observer cannot be translated to its
///      NAIF ID code, the error SPICE(IDCODENOTFOUND) is signaled.
///
///  2)  If the reference frame REF is not a recognized reference
///      frame, the error SPICE(UNKNOWNFRAME) is signaled.
///
///  3)  If the loaded kernels provide insufficient data to compute the
///      requested position vector, an error is signaled by a routine
///      in the call tree of this routine.
///
///  4)  If an error occurs while reading an SPK or other kernel file,
///      the error is signaled by a routine in the call tree of this
///      routine.
///
///  5)  If any of the required attributes of the reference frame REF
///      cannot be determined, the error SPICE(UNKNOWNFRAME2) is
///      signaled.
/// ```
///
/// # Files
///
/// ```text
///  This routine computes positions using SPK files that have been
///  loaded into the SPICE system, normally via the kernel loading
///  interface routine FURNSH. See the routine FURNSH and the SPK
///  and KERNEL Required Reading for further information on loading
///  (and unloading) kernels.
///
///  If the output position PTARG is to be expressed relative to a
///  non-inertial frame, or if any of the ephemeris data used to
///  compute PTARG are expressed relative to a non-inertial frame in
///  the SPK files providing those data, additional kernels may be
///  needed to enable the reference frame transformations required to
///  compute the position. Normally these additional kernels are PCK
///  files or frame kernels. Any such kernels must already be loaded
///  at the time this routine is called.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is part of the user interface to the SPICE ephemeris
///  system. It allows you to retrieve position information for any
///  ephemeris object relative to any other in a reference frame that
///  is convenient for further computations.
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
///     4) Obtain an uncorrected position vector derived directly from
///        data in an SPK file.
///
///           Use 'NONE'.
///
///
///     5) Use a geometric position vector as a low-accuracy estimate
///        of the apparent position for an application where execution
///        speed is critical.
///
///           Use 'NONE'.
///
///
///     6) While this routine cannot perform the relativistic
///        aberration corrections required to compute positions
///        with the highest possible accuracy, it can supply the
///        geometric positions required as inputs to these
///        computations.
///
///           Use 'NONE', then apply high-accuracy aberration
///           corrections (not available in the SPICE Toolkit).
///
///
///  Below, we discuss in more detail how the aberration corrections
///  applied by this routine are computed.
///
///     Geometric case
///     ==============
///
///     SPKEZP begins by computing the geometric position T(ET) of the
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
///     The returned position vector is
///
///        T(ET) - O(ET)
///
///
///
///     Reception case
///     ==============
///
///     When any of the options 'LT', 'CN', 'LT+S', 'CN+S' is selected
///     for ABCORR, SPKEZP computes the position of the target body at
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
///     The light time corrected position vector is
///
///        T(ET-LT) - O(ET)
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
///
///     Transmission case
///     ==================
///
///     When any of the options 'XLT', 'XCN', 'XLT+S', 'XCN+S' is
///     selected, SPKEZP computes the position of the target body T at
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
///     The light-time corrected position vector is
///
///        T(ET+LT) - O(ET)
///
///     If correction for stellar aberration is requested, the target
///     position is rotated away from the solar system barycenter-
///     relative velocity vector of the observer. The rotation is
///     computed as in the reception case, but the sign of the
///     rotation angle is negated.
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
///  1)  Load a planetary ephemeris SPK, then look up a series of
///      geometric positions of the moon relative to the earth,
///      referenced to the J2000 frame.
///
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
///      C     Use a time step of 1 hour; look up 100 positions.
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
///            DOUBLE PRECISION      POS ( 3 )
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
///      C     position vector at each one.
///      C
///            DO I = 1, MAXITR
///
///               ET = ET0 + (I-1)*STEP
///
///               CALL SPKEZP ( TARGET, ET, FRAME, ABCORR, OBSRVR,
///           .                 POS,    LT                        )
///
///               WRITE (*,*) 'ET = ', ET
///               WRITE (*,*) 'J2000 x-position (km):   ', POS(1)
///               WRITE (*,*) 'J2000 y-position (km):   ', POS(2)
///               WRITE (*,*) 'J2000 z-position (km):   ', POS(3)
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
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.2.1, 13-APR-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 3.2.0, 03-JUL-2014 (NJB) (BVS)
///
///         Discussion of light time corrections was updated. Assertions
///         that converged light time corrections are unlikely to be
///         useful were removed.
///
///         Bug fix: added a check and an exception for the FOUND flag
///         returned by FRINFO.
///
///         Updated to save the input frame name and POOL state counter
///         and to do frame name-ID conversion only if the counter has
///         changed.
///
/// -    SPICELIB Version 3.1.1, 04-APR-2008 (NJB)
///
///         Corrected minor error in description of XLT+S aberration
///         correction.
///
/// -    SPICELIB Version 3.1.0, 06-JAN-2005 (NJB)
///
///         Tests of routine FAILED() were added.
///
/// -    SPICELIB Version 3.0.3, 12-DEC-2004 (NJB)
///
///         Minor header error was corrected.
///
/// -    SPICELIB Version 3.0.2, 20-OCT-2003 (EDW)
///
///         Added mention that LT returns in seconds.
///
/// -    SPICELIB Version 3.0.1, 29-JUL-2003 (NJB) (CHA)
///
///         Various minor header changes were made to improve clarity.
///
/// -    SPICELIB Version 3.0.0, 31-DEC-2001 (NJB)
///
///         Updated to handle aberration corrections for transmission
///         of radiation. Formerly, only the reception case was
///         supported. The header was revised and expanded to explain
///         the functionality of this routine in more detail.
///
/// -    SPICELIB Version 1.0.0, 03-MAR-1999 (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 3.1.0, 06-JAN-2005 (NJB)
///
///         Tests of routine FAILED() were added. The new checks
///         are intended to prevent arithmetic operations from
///         being performed with uninitialized or invalid data.
///
/// -    SPICELIB Version 3.0.0, 31-DEC-2001 (NJB)
///
///         Updated to handle aberration corrections for transmission
///         of radiation. Formerly, only the reception case was
///         supported.
/// ```
pub fn spkezp(
    ctx: &mut SpiceContext,
    targ: i32,
    et: f64,
    ref_: &str,
    abcorr: &str,
    obs: i32,
    ptarg: &mut [f64; 3],
    lt: &mut f64,
) -> crate::Result<()> {
    SPKEZP(
        targ,
        et,
        ref_.as_bytes(),
        abcorr.as_bytes(),
        obs,
        ptarg,
        lt,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKEZP ( S/P Kernel, easy position )
pub fn SPKEZP(
    TARG: i32,
    ET: f64,
    REF: &[u8],
    ABCORR: &[u8],
    OBS: i32,
    PTARG: &mut [f64],
    LT: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut PTARG = DummyArrayMut::new(PTARG, 1..=3);

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
    } else {
        CHKIN(RNAME, ctx)?;
    }

    //
    // Get the frame id for J2000 on the first call to this routine.
    //
    if save.FIRST {
        NAMFRM(b"J2000", &mut save.FJ2000, ctx)?;

        //
        // Initialize counter.
        //
        ZZCTRUIN(save.SVCTR1.as_slice_mut(), ctx);

        save.FIRST = false;
    }

    //
    // Decide whether the aberration correction is for received or
    // transmitted radiation.
    //
    save.I = LTRIM(ABCORR);
    save.XMIT = EQCHR(fstr::substr(ABCORR, save.I..=save.I), b"X", ctx);

    //
    // If we only want geometric positions, then compute just that.
    //
    // Otherwise, compute the state of the observer relative to
    // the SSB.  Then feed that position into SPKAPO to compute the
    // apparent position of the target body relative to the observer
    // with the requested aberration corrections.
    //
    if EQSTR(ABCORR, b"NONE") {
        SPKGPS(TARG, ET, REF, OBS, PTARG.as_slice_mut(), LT, ctx)?;
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
        // call SPKSSB, SPKAPO and return.
        //
        if (save.TYPE == INERTL) {
            SPKSSB(OBS, ET, REF, save.SOBS.as_slice_mut(), ctx)?;
            SPKAPO(
                TARG,
                ET,
                REF,
                save.SOBS.as_slice(),
                ABCORR,
                PTARG.as_slice_mut(),
                LT,
                ctx,
            )?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // Still here?
        //
        // We are dealing with a non-inertial frame.  But we need to
        // do light time and stellar aberration in an inertial frame.
        // Get the "apparent" position of TARG in the intermediary
        // inertial reference frame J2000.
        //
        // We also need the light time to the center of the frame.
        //
        SPKSSB(OBS, ET, b"J2000", save.SOBS.as_slice_mut(), ctx)?;
        SPKAPO(
            TARG,
            ET,
            b"J2000",
            save.SOBS.as_slice(),
            ABCORR,
            save.POSTN.as_slice_mut(),
            LT,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        if (save.CENTER == OBS) {
            save.LTCENT = 0.0;
        } else if (save.CENTER == TARG) {
            save.LTCENT = *LT;
        } else {
            SPKAPO(
                save.CENTER,
                ET,
                b"J2000",
                save.SOBS.as_slice(),
                ABCORR,
                save.TEMP.as_slice_mut(),
                &mut save.LTCENT,
                ctx,
            )?;
        }

        //
        // If something went wrong (like we couldn't get the position of
        // the center relative to the observer) now it is time to quit.
        //
        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // If the aberration corrections are for transmission, negate
        // the light time, since we wish to compute the orientation
        // of the non-inertial frame at an epoch later than ET by
        // the one-way light time.
        //
        if save.XMIT {
            save.LTCENT = -save.LTCENT;
        }

        //
        // Get the rotation from J2000 to the requested frame
        // and convert the position.
        //
        REFCHG(
            save.FJ2000,
            save.REQFRM,
            (ET - save.LTCENT),
            save.XFORM.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        MXV(
            save.XFORM.as_slice(),
            save.POSTN.as_slice(),
            PTARG.as_slice_mut(),
        );
    }

    CHKOUT(RNAME, ctx)?;
    Ok(())
}
