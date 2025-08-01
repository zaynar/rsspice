//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const CORLEN: i32 = 5;
const NFLAGS: i32 = 9;
const IXNONE: i32 = 1;
const IXLT: i32 = (IXNONE + 1);
const IXLTS: i32 = (IXLT + 1);
const IXCN: i32 = (IXLTS + 1);
const IXCNS: i32 = (IXCN + 1);
const IXXLT: i32 = (IXCNS + 1);
const IXXLTS: i32 = (IXXLT + 1);
const IXXCN: i32 = (IXXLTS + 1);
const IXXCNS: i32 = (IXXCN + 1);

struct SaveVars {
    FLAGS: ActualCharArray,
    PRVCOR: Vec<u8>,
    FIRST: bool,
    USECN: bool,
    USESTL: bool,
    USELT: bool,
    XMIT: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FLAGS = ActualCharArray::new(CORLEN, 1..=NFLAGS);
        let mut PRVCOR = vec![b' '; CORLEN as usize];
        let mut FIRST: bool = false;
        let mut USECN: bool = false;
        let mut USESTL: bool = false;
        let mut USELT: bool = false;
        let mut XMIT: bool = false;

        FIRST = true;
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"NONE"),
                Val::C(b"LT"),
                Val::C(b"LT+S"),
                Val::C(b"CN"),
                Val::C(b"CN+S"),
                Val::C(b"XLT"),
                Val::C(b"XLT+S"),
                Val::C(b"XCN"),
                Val::C(b"XCN+S"),
            ]
            .into_iter();
            FLAGS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        fstr::assign(&mut PRVCOR, b" ");

        Self {
            FLAGS,
            PRVCOR,
            FIRST,
            USECN,
            USESTL,
            USELT,
            XMIT,
        }
    }
}

/// S/P Kernel, apparent state
///
/// Deprecated: This routine has been superseded by the SPICELIB
/// routine SPKAPS. This routine is supported for purposes of
/// backward compatibility only.
///
/// Return the state (position and velocity) of a target body
/// relative to an observer, optionally corrected for light time and
/// stellar aberration.
///
/// # Required Reading
///
/// * [SPK](crate::required_reading::spk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  TARG       I   Target body.
///  ET         I   Observer epoch.
///  REF        I   Inertial reference frame of observer's state.
///  SOBS       I   State of observer wrt. solar system barycenter.
///  ABCORR     I   Aberration correction flag.
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
///  REF      is the inertial reference frame with respect to which
///           the observer's state SOBS is expressed. REF must be
///           recognized by the SPICE Toolkit. The acceptable
///           frames are listed in the Frames Required Reading, as
///           well as in the SPICELIB routine CHGIRF.
///
///           Case and blanks are not significant in the string REF.
///
///  SOBS     is the geometric (uncorrected) state of the observer
///           relative to the solar system barycenter at epoch ET.
///           SOBS is a 6-vector:  the first three components of
///           SOBS represent a Cartesian position vector; the last
///           three components represent the corresponding velocity
///           vector. SOBS is expressed relative to the inertial
///           reference frame designated by REF.
///
///           Units are always km and km/sec.
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
///                         The light time correction involves
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
///                         of SPKEZR for a discussion of precision
///                         of light time corrections.
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
///           Neither special nor general relativistic effects are
///           accounted for in the aberration corrections applied
///           by this routine.
///
///           Case and blanks are not significant in the string
///           ABCORR.
/// ```
///
/// # Detailed Output
///
/// ```text
///  STARG    is a Cartesian state vector representing the position
///           and velocity of the target body relative to the
///           specified observer. STARG is corrected for the
///           specified aberrations, and is expressed with respect
///           to the specified inertial reference frame. The first
///           three components of STARG represent the x-, y- and
///           z-components of the target's position; last three
///           components form the corresponding velocity vector.
///
///           The position component of STARG points from the
///           observer's location at ET to the aberration-corrected
///           location of the target. Note that the sense of the
///           position vector is independent of the direction of
///           radiation travel implied by the aberration
///           correction.
///
///           The velocity component of STARG is obtained by
///           evaluating the target's geometric state at the light
///           time corrected epoch, so for aberration-corrected
///           states, the velocity is not precisely equal to the
///           time derivative of the position.
///
///           Units are always km and km/sec.
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
///  1)  If the value of ABCORR is not recognized, the error
///      SPICE(SPKINVALIDOPTION) is signaled.
///
///  2)  If the reference frame requested is not a recognized
///      inertial reference frame, the error SPICE(BADFRAME)
///      is signaled.
///
///  3)  If the state of the target relative to the solar system
///      barycenter cannot be computed, an error is signaled by a
///      routine in the call tree of this routine.
/// ```
///
/// # Files
///
/// ```text
///  This routine computes states using SPK files that have been
///  loaded into the SPICE system, normally via the kernel loading
///  interface routine FURNSH. Application programs typically load
///  kernels once before this routine is called, for example during
///  program initialization; kernels need not be loaded repeatedly.
///  See the routine FURNSH and the SPK and KERNEL Required Reading
///  for further information on loading (and unloading) kernels.
///
///  If any of the ephemeris data used to compute STARG are expressed
///  relative to a non-inertial frame in the SPK files providing those
///  data, additional kernels may be needed to enable the reference
///  frame transformations required to compute the state. Normally
///  these additional kernels are PCK files or frame kernels. Any
///  such kernels must already be loaded at the time this routine is
///  called.
/// ```
///
/// # Particulars
///
/// ```text
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
///  time and stellar aberration corrected."
///
///  The SPICE Toolkit can correct for two phenomena affecting the
///  apparent location of an object: one-way light time (also called
///  "planetary aberration") and stellar aberration. Correcting for
///  one-way light time is done by computing, given an observer and
///  observation epoch, where a target was when the observed photons
///  departed the target's location. The vector from the observer to
///  this computed target location is called a "light time corrected"
///  vector. The light time correction depends on the motion of the
///  target, but it is independent of the velocity of the observer
///  relative to the solar system barycenter. Relativistic effects
///  such as light bending and gravitational delay are not accounted
///  for in the light time correction performed by this routine.
///
///  The velocity of the observer also affects the apparent location
///  of a target: photons arriving at the observer are subject to a
///  "raindrop effect" whereby their velocity relative to the observer
///  is, using a Newtonian approximation, the photons' velocity
///  relative to the solar system barycenter minus the velocity of the
///  observer relative to the solar system barycenter. This effect is
///  called "stellar aberration." Stellar aberration is independent
///  of the velocity of the target. The stellar aberration formula
///  used by this routine is non-relativistic.
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
///  transmission of photons to the target. This requires correction
///  of the geometric target position for the effects of light time and
///  stellar aberration, but in this case the corrections are computed
///  for radiation traveling from the observer to the target.
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
///  The traditional aberration corrections applicable to observation
///  and those applicable to transmission are related in a simple way:
///  one may picture the geometry of the "transmission" case by
///  imagining the "observation" case running in reverse time order,
///  and vice versa.
///
///  One may reasonably object to using the term "observer" in the
///  transmission case, in which radiation is emitted from the
///  observer's location. The terminology was retained for
///  consistency with earlier documentation.
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
///     4) Obtain an uncorrected state vector derived directly from
///        data in an SPK file.
///
///           Use 'NONE'.
///
///
///     5) Use a geometric state vector as a low-accuracy estimate
///        of the apparent state for an application where execution
///        speed is critical:
///
///           Use 'NONE'.
///
///
///     6) While this routine cannot perform the relativistic
///        aberration corrections required to compute states
///        with the highest possible accuracy, it can supply the
///        geometric states required as inputs to these computations:
///
///           Use 'NONE', then apply high-accuracy aberration
///           corrections (not available in the SPICE Toolkit).
///
///
///  Below, we discuss in more detail how the aberration corrections
///  applied by this routine are computed.
///
///
///  Geometric case
///  ==============
///
///     SPKAPP begins by computing the geometric position T(ET) of the
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
///     corresponding velocities.  In the geometric case, the
///     returned velocity is actually the time derivative of the
///     position.
///
///
///  Reception case
///  ==============
///
///     When any of the options 'LT', 'CN', 'LT+S', 'CN+S' is
///     selected, SPKAPP computes the position of the target body at
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
///     RHS of the light-time equation (1) yields the "one-iteration"
///     estimate of the one-way light time. Repeating the process
///     until the estimates of LT converge yields the "converged
///     Newtonian" light time estimate.
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
///     The position component of the light-time corrected state
///     is the vector
///
///        T(ET-LT) - O(ET)
///
///     The velocity component of the light-time corrected state
///     is the difference
///
///        T_vel(ET-LT) - O_vel(ET)
///
///     where T_vel and O_vel are, respectively, the velocities of
///     the target and observer relative to the solar system
///     barycenter at the epochs ET-LT and ET.
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
///     The velocity component of the output state STARG is
///     not corrected for stellar aberration.
///
///
///  Transmission case
///  ==================
///
///     When any of the options 'XLT', 'XCN', 'XLT+S', 'XCN+S' are
///     selected, SPKAPP computes the position of the target body T at
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
///     is the difference
///
///        T_vel(ET+LT) - O_vel(ET)
///
///     where T_vel and O_vel are, respectively, the velocities of
///     the target and observer relative to the solar system
///     barycenter at the epochs ET+LT and ET.
///
///     If correction for stellar aberration is requested, the target
///     position is rotated away from the solar system barycenter-
///     relative velocity vector of the observer. The rotation is
///     computed as in the reception case, but the sign of the
///     rotation angle is negated.
///
///     The velocity component of the output state STARG is
///     not corrected for stellar aberration.
///
///  Neither special nor general relativistic effects are accounted
///  for in the aberration corrections performed by this routine.
/// ```
///
/// # Examples
///
/// ```text
///  In the following code fragment, SPKSSB and SPKAPP are used
///  to display the position of Io (body 501) as seen from the
///  Voyager 2 spacecraft (Body -32) at a series of epochs.
///
///  Normally, one would call the high-level reader SPKEZR to obtain
///  state vectors. The example below illustrates the interface
///  of this routine but is not intended as a recommendation on
///  how to use the SPICE SPK subsystem.
///
///  The use of integer ID codes is necessitated by the low-level
///  interface of this routine.
///
///     IO    = 501
///     VGR2  = -32
///
///     DO WHILE ( EPOCH .LE. END )
///
///        CALL SPKSSB (  VGR2,   EPOCH,  'J2000',  STVGR2  )
///        CALL SPKAPP (  IO,     EPOCH,  'J2000',  STVGR2,
///    .                 'LT+S',  STIO,    LT               )
///
///        CALL RECRAD (  STIO,   RANGE,   RA,      DEC     )
///        WRITE (*,*)  RA * DPR(),  DEC * DPR()
///
///        EPOCH = EPOCH + DELTA
///
///     END DO
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The kernel files to be used by SPKAPP must be loaded
///      (normally by the SPICELIB kernel loader FURNSH) before
///      this routine is called.
///
///  2)  Unlike most other SPK state computation routines, this
///      routine requires that the input state be relative to an
///      inertial reference frame. Non-inertial frames are not
///      supported by this routine.
///
///  3)  In a future version of this routine, the implementation
///      of the aberration corrections may be enhanced to improve
///      accuracy.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  J.M. Lynch         (JPL)
///  H.A. Neilan        (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.1.1, 26-OCT-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 3.1.0, 04-JUL-2014 (NJB) (BVS)
///
///         Discussion of light time corrections was updated. Assertions
///         that converged light time corrections are unlikely to be
///         useful were removed.
///
///      Last update was 21-SEP-2013 (BVS)
///
///         Updated to call LJUCRS instead of CMPRSS/UCASE.
///
/// -    SPICELIB Version 3.0.3, 18-MAY-2010 (BVS)
///
///         Index lines now state that this routine is deprecated.
///
/// -    SPICELIB Version 3.0.2, 08-JAN-2008 (NJB)
///
///         The $Abstract section of the header was updated to
///         indicate that this routine has been deprecated.
///
/// -    SPICELIB Version 3.0.1, 20-OCT-2003 (EDW)
///
///         Added mention that LT returns in seconds.
///         Corrected spelling errors.
///
/// -    SPICELIB Version 3.0.0, 18-DEC-2001 (NJB)
///
///         Updated to handle aberration corrections for transmission
///         of radiation. Formerly, only the reception case was
///         supported. The header was revised and expanded to explain
///         the functionality of this routine in more detail.
///
/// -    SPICELIB Version 2.1.0, 09-JUL-1996 (WLT)
///
///         Corrected the description of LT in the Detailed Output
///         section of the header.
///
/// -    SPICELIB Version 2.0.0, 22-MAY-1995 (WLT)
///
///         The routine was modified to support the options 'CN' and
///         'CN+S' aberration corrections. Moreover, diagnostics were
///         added to check for reference frames that are not recognized
///         inertial frames.
///
/// -    SPICELIB Version 1.1.2, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.1.1, 06-MAR-1991 (JML)
///
///         In the example program, the calling sequence of SPKAPP
///         was corrected.
///
/// -    SPICELIB Version 1.1.0, 25-MAY-1990 (HAN)
///
///         The local variable CORR was added to eliminate a
///         run-time error that occurred when SPKAPP was determining
///         what corrections to apply to the state.
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
/// -    SPICELIB Version 2.0.0, 22-MAY-1995 (WLT)
///
///         The routine was modified to support the options 'CN' and
///         'CN+S' aberration corrections. Moreover, diagnostics were
///         added to check for reference frames that are not recognized
///         inertial frames.
///
/// -    SPICELIB Version 1.1.0, 25-MAY-1990 (HAN)
///
///         The local variable CORR was added to eliminate a run-time
///         error that occurred when SPKAPP was determining what
///         corrections to apply to the state. If the literal string
///         'LT' was assigned to ABCORR, SPKAPP attempted to look at
///         ABCORR(3:4). Because ABCORR is a passed length argument, its
///         length is not guaranteed, and those positions may not exist.
///         Searching beyond the bounds of a string resulted in a
///         run-time error at NAIF because NAIF compiles SPICELIB using the
///         CHECK=BOUNDS option for the DEC VAX/VMX DCL FORTRAN command.
///         Also, without the local variable CORR, SPKAPP would have to
///         modify the value of a passed argument, ABCORR. That's a no no.
/// ```
pub fn spkapp(
    ctx: &mut SpiceContext,
    targ: i32,
    et: f64,
    ref_: &str,
    sobs: &[f64; 6],
    abcorr: &str,
    starg: &mut [f64; 6],
    lt: &mut f64,
) -> crate::Result<()> {
    SPKAPP(
        targ,
        et,
        ref_.as_bytes(),
        sobs,
        abcorr.as_bytes(),
        starg,
        lt,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKAPP ( S/P Kernel, apparent state )
pub fn SPKAPP(
    TARG: i32,
    ET: f64,
    REF: &[u8],
    SOBS: &[f64],
    ABCORR: &[u8],
    STARG: &mut [f64],
    LT: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let SOBS = DummyArray::new(SOBS, 1..=6);
    let mut STARG = DummyArrayMut::new(STARG, 1..=6);
    let mut CORR = [b' '; CORLEN as usize];
    let mut SAPOS = StackArray::<f64, 3>::new(1..=3);
    let mut TSTATE = StackArray::<f64, 6>::new(1..=6);
    let mut I: i32 = 0;
    let mut LTSIGN: i32 = 0;
    let mut MAXITR: i32 = 0;
    let mut REFID: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Indices of flags in the FLAGS array:
    //

    //
    // Local variables
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
        CHKIN(b"SPKAPP", ctx)?;
    }

    if (save.FIRST || fstr::ne(ABCORR, &save.PRVCOR)) {
        //
        // The aberration correction flag differs from the value it
        // had on the previous call, if any.  Analyze the new flag.
        //
        // Remove leading and embedded white space from the aberration
        // correction flag and convert to upper case.
        //
        LJUCRS(0, ABCORR, &mut CORR, ctx);

        //
        // Locate the flag in our list of flags.
        //
        I = ISRCHC(&CORR, NFLAGS, save.FLAGS.as_arg());

        if (I == 0) {
            SETMSG(b"Requested aberration correction # is not supported.", ctx);
            ERRCH(b"#", ABCORR, ctx);
            SIGERR(b"SPICE(SPKINVALIDOPTION)", ctx)?;
            CHKOUT(b"SPKAPP", ctx)?;
            return Ok(());
        }

        //
        // The aberration correction flag is recognized; save it.
        //
        fstr::assign(&mut save.PRVCOR, ABCORR);

        //
        // Set logical flags indicating the attributes of the requested
        // correction.
        //
        save.XMIT = (I > IXCNS);

        save.USELT = ((((I == IXLT) || (I == IXLTS)) || (I == IXXLT)) || (I == IXXLTS));

        save.USESTL = ((I > 1) && ODD(I));

        save.USECN = ((((I == IXCN) || (I == IXCNS)) || (I == IXXCN)) || (I == IXXCNS));

        save.FIRST = false;
    }

    //
    // See if the reference frame is a recognized inertial frame.
    //
    IRFNUM(REF, &mut REFID, ctx);

    if (REFID == 0) {
        SETMSG(
            b"The requested frame \'#\' is not a recognized inertial frame. ",
            ctx,
        );
        ERRCH(b"#", REF, ctx);
        SIGERR(b"SPICE(BADFRAME)", ctx)?;
        CHKOUT(b"SPKAPP", ctx)?;
        return Ok(());
    }

    //
    // Determine the sign of the light time offset.
    //
    if save.XMIT {
        LTSIGN = 1;
    } else {
        LTSIGN = -1;
    }

    //
    // Find the geometric state of the target body with respect to the
    // solar system barycenter. Subtract the state of the observer
    // to get the relative state. Use this to compute the one-way
    // light time.
    //
    SPKSSB(TARG, ET, REF, STARG.as_slice_mut(), ctx)?;
    VSUBG(STARG.as_slice(), SOBS.as_slice(), 6, TSTATE.as_slice_mut());
    MOVED(TSTATE.as_slice(), 6, STARG.as_slice_mut());

    *LT = (VNORM(STARG.as_slice()) / CLIGHT());

    //
    // To correct for light time, find the state of the target body
    // at the current epoch minus the one-way light time. Note that
    // the observer remains where he is.
    //
    if save.USELT {
        MAXITR = 1;
    } else if save.USECN {
        MAXITR = 3;
    } else {
        MAXITR = 0;
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXITR;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            SPKSSB(
                TARG,
                (ET + ((LTSIGN as f64) * *LT)),
                REF,
                STARG.as_slice_mut(),
                ctx,
            )?;
            VSUBG(STARG.as_slice(), SOBS.as_slice(), 6, TSTATE.as_slice_mut());
            MOVED(TSTATE.as_slice(), 6, STARG.as_slice_mut());
            *LT = (VNORM(STARG.as_slice()) / CLIGHT());

            I += m3__;
        }
    }

    //
    // At this point, STARG contains the light time corrected
    // state of the target relative to the observer.
    //
    // If stellar aberration correction is requested, perform it now.
    //
    // Stellar aberration corrections are not applied to the target's
    // velocity.
    //
    if save.USESTL {
        if save.XMIT {
            //
            // This is the transmission case.
            //
            // Compute the position vector obtained by applying
            // "reception" stellar aberration to STARG.
            //
            STLABX(
                STARG.as_slice(),
                SOBS.subarray(4),
                SAPOS.as_slice_mut(),
                ctx,
            )?;
            VEQU(SAPOS.as_slice(), STARG.as_slice_mut());
        } else {
            //
            // This is the reception case.
            //
            // Compute the position vector obtained by applying
            // "reception" stellar aberration to STARG.
            //
            STELAB(
                STARG.as_slice(),
                SOBS.subarray(4),
                SAPOS.as_slice_mut(),
                ctx,
            )?;
            VEQU(SAPOS.as_slice(), STARG.as_slice_mut());
        }
    }

    CHKOUT(b"SPKAPP", ctx)?;
    Ok(())
}
