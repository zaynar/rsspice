//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;
const RNAME: &[u8] = b"SPKAPS";

struct SaveVars {
    PRVCOR: Vec<u8>,
    FIRST: bool,
    USESTL: bool,
    XMIT: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PRVCOR = vec![b' '; CORLEN as usize];
        let mut FIRST: bool = false;
        let mut USESTL: bool = false;
        let mut XMIT: bool = false;

        FIRST = true;
        fstr::assign(&mut PRVCOR, b" ");

        Self {
            PRVCOR,
            FIRST,
            USESTL,
            XMIT,
        }
    }
}

/// SPK, apparent state
///
/// Return the state (position and velocity) of a target body
/// relative to an observer specified by its state and
/// acceleration relative to the solar system barycenter. The
/// returned state may be optionally corrected for light time
/// and stellar aberration. All input and output vectors are
/// expressed relative to an inertial reference frame.
///
/// This routine supersedes SPKAPP.
///
/// SPICE users normally should call the high-level API routines
/// SPKEZR or SPKEZ rather than this routine.
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
///  REF        I   Inertial reference frame of output state.
///  ABCORR     I   Aberration correction flag.
///  STOBS      I   State of the observer relative to the SSB.
///  ACCOBS     I   Acceleration of the observer relative to the SSB.
///  STARG      O   State of target.
///  LT         O   One way light time between observer and target.
///  DLT        O   Derivative of light time with respect to time.
/// ```
///
/// # Detailed Input
///
/// ```text
///  TARG     is the NAIF ID code for a target body. The target
///           and observer define a state vector whose position
///           component points from the observer to the target.
///
///  ET       is the ephemeris time, expressed as seconds past
///           J2000 TDB, at which the state of the target body
///           relative to the observer is to be computed.  ET
///           refers to time at the observer's location.
///
///  REF      is the inertial reference frame with respect to which
///           the input state STOBS, the input acceleration ACCOBS,
///           and the output state STARG are expressed. REF must be
///           recognized by the SPICE Toolkit. The acceptable
///           frames are listed in the Frames Required Reading, as
///           well as in the SPICELIB routine CHGIRF.
///
///           Case and blanks are not significant in the string
///           REF.
///
///  ABCORR   indicates the aberration corrections to be applied
///           to the state of the target body to account for one-way
///           light time and stellar aberration. See the discussion
///           in the header of SPKEZR for recommendations on
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
///                         computed. See the $Particulars section of
///                         SPKEZR for a discussion of precision of
///                         light time corrections.
///
///              'CN+S'     Converged Newtonian light time
///                         correction and stellar aberration
///                         correction.
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
///
///  STOBS    is the geometric state of the observer relative to
///           the solar system barycenter at ET. STOBS is expressed
///           relative to the reference frame designated by REF.
///           The target and observer define a state vector whose
///           position component points from the observer to the
///           target.
///
///  ACCOBS   is the geometric acceleration of the observer
///           relative to the solar system barycenter at ET. This
///           is the derivative with respect to time of the
///           velocity portion of STOBS. ACCOBS is expressed
///           relative to the reference frame designated by REF.
///
///           ACCOBS is used for computing stellar aberration
///           corrected velocity. If stellar aberration corrections
///           are not specified by ABCORR, ACCOBS is ignored; the
///           caller need not provide a valid input value in this
///           case.
/// ```
///
/// # Detailed Output
///
/// ```text
///  STARG    is a Cartesian state vector representing the position
///           and velocity of the target body relative to the
///           specified observer. STARG is corrected for the
///           specified aberrations, and is expressed with respect
///           to the inertial reference frame designated by REF.
///           The first three components of STARG represent the x-,
///           y- and z-components of the target's position; last
///           three components form the corresponding velocity
///           vector.
///
///           The position component of STARG points from the
///           observer's location at ET to the aberration-corrected
///           location of the target. Note that the sense of the
///           position vector is independent of the direction of
///           radiation travel implied by the aberration
///           correction.
///
///           Units are always km and km/sec.
///
///  LT       is the one-way light time between the observer and
///           target in seconds. If the target state is corrected
///           for light time, then LT is the one-way light time
///           between the observer and the light time-corrected
///           target location.
///
///  DLT      is the derivative with respect to barycentric
///           dynamical time of the one way light time between
///           target and observer:
///
///              DLT = d(LT)/d(ET)
///
///           DLT can also be described as the rate of change of
///           one way light time. DLT is unitless, since LT and
///           ET both have units of TDB seconds.
///
///           If the observer and target are at the same position,
///           then DLT is set to zero.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the value of ABCORR is not recognized, an error is signaled
///      by a routine in the call tree of this routine.
///
///  2)  If ABCORR calls for stellar aberration but not light
///      time corrections, the error SPICE(NOTSUPPORTED) is
///      signaled.
///
///  3)  If ABCORR calls for relativistic light time corrections, the
///      error SPICE(NOTSUPPORTED) is signaled.
///
///  4)  If the reference frame requested is not a recognized
///      inertial reference frame, the error SPICE(BADFRAME)
///      is signaled.
///
///  5)  If the state of the target relative to the solar system
///      barycenter cannot be computed, an error is signaled by a
///      routine in the call tree of this routine.
///
///  6)  If the observer and target are at the same position,
///      then DLT is set to zero. This situation could arise,
///      for example, when the observer is Mars and the target
///      is the Mars barycenter.
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
///  these additional kernels are PCK files or frame kernels. Any such
///  kernels must already be loaded at the time this routine is
///  called.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine supports higher-level SPK API routines that can
///  perform both light time and stellar aberration corrections.
///
///  User applications normally will not need to call this routine
///  directly. However, this routine can improve run-time efficiency
///  in situations where many targets are observed from the same
///  location at the same time. In such cases, the state and
///  acceleration of the observer relative to the solar system
///  barycenter need be computed only once per look-up epoch.
///
///  When apparent positions, rather than apparent states, are
///  required, consider using the high-level position-only API
///  routines
///
///     SPKPOS
///     SPKEZP
///
///  or the low-level, position-only analog of this routine
///
///     SPKAPO
///
///  In general, the position-only routines are more efficient.
///
///  See the header of the routine SPKEZR for a detailed discussion
///  of aberration corrections.
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
///  1) Look up a sequence of states of the Moon as seen from the
///     Earth. Use light time and stellar aberration corrections.
///     Compute the first state for the epoch 2000 JAN 1 12:00:00 TDB;
///     compute subsequent states at intervals of 1 hour. For each
///     epoch, display the states, the one way light time between
///     target and observer, and the rate of change of the one way
///     light time.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: spkaps_ex1.tm
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
///           pck00008.tpc                  Planet orientation and
///                                         radii
///           naif0008.tls                  Leapseconds
///
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de418.bsp',
///                               'pck00008.tpc',
///                               'naif0008.tls'  )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM SPKAPS_EX1
///           IMPLICIT NONE
///     C
///     C     Local constants
///     C
///     C     The meta-kernel name shown here refers to a file whose
///     C     contents are those shown above. This file and the kernels
///     C     it references must exist in your current working
///     C     directory.
///     C
///           CHARACTER*(*)         META
///           PARAMETER           ( META   = 'spkaps_ex1.tm' )
///     C
///     C     Use a time step of 1 hour; look up 5 states.
///     C
///           DOUBLE PRECISION      STEP
///           PARAMETER           ( STEP   = 3600.0D0 )
///
///           INTEGER               MAXITR
///           PARAMETER           ( MAXITR = 5 )
///     C
///     C     Local variables
///     C
///           DOUBLE PRECISION      ACC    ( 3 )
///           DOUBLE PRECISION      DLT
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      ET0
///           DOUBLE PRECISION      LT
///           DOUBLE PRECISION      STATE  ( 6 )
///           DOUBLE PRECISION      STATE0 ( 6 )
///           DOUBLE PRECISION      STATE2 ( 6 )
///           DOUBLE PRECISION      STOBS  ( 6 )
///           DOUBLE PRECISION      TDELTA
///           INTEGER               I
///
///     C
///     C     Load the SPK and LSK kernels via the meta-kernel.
///     C
///           CALL FURNSH ( META )
///     C
///     C     Convert the start time to seconds past J2000 TDB.
///     C
///           CALL STR2ET ( '2000 JAN 1 12:00:00 TDB', ET0 )
///     C
///     C     Step through a series of epochs, looking up a
///     C     state vector at each one.
///     C
///           DO I = 1, MAXITR
///
///              ET = ET0 + (I-1)*STEP
///
///     C
///     C        Look up a state vector at epoch ET using the
///     C        following inputs:
///     C
///     C           Target:                 Moon (NAIF ID code 301)
///     C           Reference frame:        J2000
///     C           Aberration correction:  Light time and stellar
///     C                                   aberration ('LT+S')
///     C           Observer:               Earth (NAIF ID code 399)
///     C
///     C        Before we can execute this computation, we'll need the
///     C        geometric state and acceleration of the observer
///     C        relative to the solar system barycenter at ET,
///     C        expressed relative to the J2000 reference frame. First
///     C        find the state:
///     C
///              CALL SPKSSB ( 399, ET, 'J2000', STOBS )
///     C
///     C        Next compute the acceleration. We numerically
///     C        differentiate the velocity using a quadratic
///     C        approximation:
///     C
///              TDELTA = 1.D0
///
///              CALL SPKSSB ( 399, ET-TDELTA, 'J2000', STATE0 )
///              CALL SPKSSB ( 399, ET+TDELTA, 'J2000', STATE2 )
///
///              CALL QDERIV ( 3, STATE0(4), STATE2(4), TDELTA, ACC )
///     C
///     C        Now compute the desired state vector:
///     C
///              CALL SPKAPS ( 301,   ET,  'J2000', 'LT+S',
///          .                 STOBS, ACC, STATE,    LT,   DLT )
///
///              WRITE (*,*) 'ET = ', ET
///              WRITE (*,*) 'J2000 x-position (km):   ', STATE(1)
///              WRITE (*,*) 'J2000 y-position (km):   ', STATE(2)
///              WRITE (*,*) 'J2000 z-position (km):   ', STATE(3)
///              WRITE (*,*) 'J2000 x-velocity (km/s): ', STATE(4)
///              WRITE (*,*) 'J2000 y-velocity (km/s): ', STATE(5)
///              WRITE (*,*) 'J2000 z-velocity (km/s): ', STATE(6)
///              WRITE (*,*) 'One-way light time (s):  ', LT
///              WRITE (*,*) 'Light time rate:         ', DLT
///              WRITE (*,*) ' '
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
///      ET =    0.0000000000000000
///      J2000 x-position (km):     -291584.61369497533
///      J2000 y-position (km):     -266693.40583162551
///      J2000 z-position (km):     -76095.653209237149
///      J2000 x-velocity (km/s):   0.64343915743508395
///      J2000 y-velocity (km/s):  -0.66606587365741410
///      J2000 z-velocity (km/s):  -0.30131006342946742
///      One-way light time (s):     1.3423106103251679
///      Light time rate:            1.0731690869897750E-007
///
///      ET =    3600.0000000000000
///      J2000 x-position (km):     -289256.45942322229
///      J2000 y-position (km):     -269080.60545907740
///      J2000 z-position (km):     -77177.352771302132
///      J2000 x-velocity (km/s):   0.64997032016926526
///      J2000 y-velocity (km/s):  -0.66014825329341664
///      J2000 z-velocity (km/s):  -0.29963041790724715
///      One-way light time (s):     1.3426939548635302
///      Light time rate:            1.0565259895222426E-007
///
///      ET =    7200.0000000000000
///      J2000 x-position (km):     -286904.89654239739
///      J2000 y-position (km):     -271446.41676468350
///      J2000 z-position (km):     -78252.965533623050
///      J2000 x-velocity (km/s):   0.65644388315539315
///      J2000 y-velocity (km/s):  -0.65418355204586442
///      J2000 z-velocity (km/s):  -0.29792853294482308
///      One-way light time (s):     1.3430713117337547
///      Light time rate:            1.0399045689875861E-007
///
///      ET =    10800.000000000000
///      J2000 x-position (km):     -284530.13302756584
///      J2000 y-position (km):     -273790.67111559171
///      J2000 z-position (km):     -79322.411703917489
///      J2000 x-velocity (km/s):   0.66285950473048116
///      J2000 y-velocity (km/s):  -0.64817224685146524
///      J2000 z-velocity (km/s):  -0.29620455846903732
///      One-way light time (s):     1.3434426890693671
///      Light time rate:            1.0233066524342374E-007
///
///      ET =    14400.000000000000
///      J2000 x-position (km):     -282132.37807791750
///      J2000 y-position (km):     -276113.20159697317
///      J2000 z-position (km):     -80385.612030562901
///      J2000 x-velocity (km/s):   0.66921684649247459
///      J2000 y-velocity (km/s):  -0.64211481528028158
///      J2000 z-velocity (km/s):  -0.29445864490384888
///      One-way light time (s):     1.3438080956559786
///      Light time rate:            1.0067340363005083E-007
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine should not be used to compute geometric states.
///      Instead, use SPKEZR, SPKEZ, or SPKGEO. SPKGEO, which is called
///      by SPKEZR and SPKEZ, introduces less round-off error when the
///      observer and target have a common center that is closer to
///      both objects than is the solar system barycenter.
///
///  2)  The kernel files to be used by SPKAPS must be loaded
///      (normally by the SPICELIB kernel loader FURNSH) before
///      this routine is called.
///
///  3)  Unlike most other SPK state computation routines, this
///      routine requires that the output state be relative to an
///      inertial reference frame.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 26-OCT-2021 (JDR) (NJB)
///
///         Edited the header to comply with NAIF standard. Updated
///         example's meta-kernel.
///
///         Bug fix: ABCORR now is parsed using ZZVALCOR. This improves
///         error checking. Removed unnecessary in-line error checking
///         now provided by ZZVALCOR.
///
/// -    SPICELIB Version 1.1.0, 04-JUL-2014 (NJB)
///
///         Discussion of light time corrections was updated. Assertions
///         that converged light time corrections are unlikely to be
///         useful were removed.
///
///      Last update was 15-APR-2014 (NJB)
///
///         Added a FAILED() call to prevent numeric problems
///         resulting from uninitialized values.
///
/// -    SPICELIB Version 1.0.0, 11-JAN-2008 (NJB)
/// ```
pub fn spkaps(
    ctx: &mut SpiceContext,
    targ: i32,
    et: f64,
    ref_: &str,
    abcorr: &str,
    stobs: &[f64; 6],
    accobs: &[f64; 3],
    starg: &mut [f64; 6],
    lt: &mut f64,
    dlt: &mut f64,
) -> crate::Result<()> {
    SPKAPS(
        targ,
        et,
        ref_.as_bytes(),
        abcorr.as_bytes(),
        stobs,
        accobs,
        starg,
        lt,
        dlt,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKAPS ( SPK, apparent state )
pub fn SPKAPS(
    TARG: i32,
    ET: f64,
    REF: &[u8],
    ABCORR: &[u8],
    STOBS: &[f64],
    ACCOBS: &[f64],
    STARG: &mut [f64],
    LT: &mut f64,
    DLT: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let STOBS = DummyArray::new(STOBS, 1..=6);
    let ACCOBS = DummyArray::new(ACCOBS, 1..=3);
    let mut STARG = DummyArrayMut::new(STARG, 1..=6);
    let mut CORPOS = StackArray::<f64, 3>::new(1..=3);
    let mut CORVEL = StackArray::<f64, 3>::new(1..=3);
    let mut DPCORR = StackArray::<f64, 3>::new(1..=3);
    let mut PCORR = StackArray::<f64, 3>::new(1..=3);
    let mut REFID: i32 = 0;
    let mut ATTBLK = StackArray::<bool, 15>::new(1..=NABCOR);

    //
    // SPICELIB functions
    //

    //
    // Local parameters
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
    }

    CHKIN(RNAME, ctx)?;

    if (save.FIRST || fstr::ne(ABCORR, &save.PRVCOR)) {
        //
        // The aberration correction flag differs from the value it
        // had on the previous call, if any.  Analyze the new flag.
        //
        ZZVALCOR(ABCORR, ATTBLK.as_slice_mut(), ctx)?;

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
        //    USECN indicates converged Newtonian light time correction.
        //
        // The above definitions are consistent with those used by
        // ZZPRSCOR.
        //
        save.XMIT = ATTBLK[XMTIDX];
        save.USESTL = ATTBLK[STLIDX];

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
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Get the state of the target relative to the observer,
    // optionally corrected for light time.
    //
    SPKLTC(
        TARG,
        ET,
        REF,
        ABCORR,
        STOBS.as_slice(),
        STARG.as_slice_mut(),
        LT,
        DLT,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // If stellar aberration corrections are not needed, we're
    // already done.
    //
    if !save.USESTL {
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Get the stellar aberration correction and its time derivative.
    //
    ZZSTELAB(
        save.XMIT,
        ACCOBS.as_slice(),
        STOBS.subarray(4),
        STARG.as_slice(),
        PCORR.as_slice_mut(),
        DPCORR.as_slice_mut(),
        ctx,
    )?;

    //
    // Adding the stellar aberration correction to the light
    // time-corrected target position yields the position corrected for
    // both light time and stellar aberration.
    //
    VADD(PCORR.as_slice(), STARG.as_slice(), CORPOS.as_slice_mut());
    VEQU(CORPOS.as_slice(), STARG.as_slice_mut());

    //
    // Velocity is treated in an analogous manner.
    //
    VADD(DPCORR.as_slice(), STARG.subarray(4), CORVEL.as_slice_mut());
    VEQU(CORVEL.as_slice(), STARG.subarray_mut(4));

    CHKOUT(RNAME, ctx)?;
    Ok(())
}
