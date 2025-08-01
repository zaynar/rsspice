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
const RNAME: &[u8] = b"SPKLTC";
const TOL: f64 = 0.0000000001;
const CNVLIM: f64 = 0.00000000000000001;
const MAXITR: i32 = 5;
const SSB: i32 = 0;

struct SaveVars {
    PRVCOR: Vec<u8>,
    PASS1: bool,
    USECN: bool,
    USELT: bool,
    XMIT: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PRVCOR = vec![b' '; CORLEN as usize];
        let mut PASS1: bool = false;
        let mut USECN: bool = false;
        let mut USELT: bool = false;
        let mut XMIT: bool = false;

        PASS1 = true;
        fstr::assign(&mut PRVCOR, b" ");

        Self {
            PRVCOR,
            PASS1,
            USECN,
            USELT,
            XMIT,
        }
    }
}

/// S/P Kernel, light time corrected state
///
/// Return the state (position and velocity) of a target body
/// relative to an observer, optionally corrected for light time,
/// expressed relative to an inertial reference frame.
///
/// # Required Reading
///
/// * [FRAMES](crate::required_reading::frames)
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
///           relative to the observer is to be computed. ET
///           refers to time at the observer's location.
///
///  REF      is the inertial reference frame with respect to which
///           the input state STOBS and the output state STARG are
///           expressed. REF must be recognized by the SPICE
///           Toolkit. The acceptable frames are listed in the
///           Frames Required Reading, as well as in the SPICELIB
///           routine CHGIRF.
///
///           Case and blanks are not significant in the string
///           REF.
///
///
///  ABCORR   indicates the aberration corrections to be applied to
///           the state of the target body to account for one-way
///           light time. See the discussion in the $Particulars
///           section for recommendations on how to choose
///           aberration corrections.
///
///           If ABCORR includes the stellar aberration correction
///           symbol '+S', this flag is simply ignored. Aside from
///           the possible presence of this symbol, ABCORR may be
///           any of the following:
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
///              'XCN'      "Transmission" case: converged
///                         Newtonian light time correction.
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
///  STOBS    is the geometric (uncorrected) state of the observer
///           relative to the solar system barycenter at epoch ET.
///           STOBS is a 6-vector: the first three components of
///           STOBS represent a Cartesian position vector; the last
///           three components represent the corresponding velocity
///           vector. STOBS is expressed relative to the inertial
///           reference frame designated by REF.
///
///           Units are always km and km/sec.
/// ```
///
/// # Detailed Output
///
/// ```text
///  STARG    is a Cartesian state vector representing the position
///           and velocity of the target body relative to the
///           specified observer. STARG is corrected for the
///           specified aberration, and is expressed with respect
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
///  1)  For the convenience of the caller, the input aberration
///      correction flag can call for stellar aberration correction via
///      inclusion of the '+S' suffix. This portion of the aberration
///      correction flag is ignored if present.
///
///  2)  If the value of ABCORR is not recognized, an error
///      is signaled by a routine in the call tree of this
///      routine.
///
///  3)  If the reference frame requested is not a recognized
///      inertial reference frame, the error SPICE(BADFRAME)
///      is signaled.
///
///  4)  If the state of the target relative to the solar system
///      barycenter cannot be computed, an error is signaled by a
///      routine in the call tree of this routine.
///
///  5)  If the observer and target are at the same position,
///      then DLT is set to zero. This situation could arise,
///      for example, when the observer is Mars and the target
///      is the Mars barycenter.
///
///  6)  If a division by zero error would occur in the computation
///      of DLT, the error SPICE(DIVIDEBYZERO) is signaled.
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
///  This routine supports higher-level SPK API routines that can
///  perform both light time and stellar aberration corrections.
///  User applications normally will not need to call this routine
///  directly.
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
///     Earth. Use light time corrections. Compute the first state for
///     the epoch 2000 JAN 1 12:00:00 TDB; compute subsequent states at
///     intervals of 1 hour. For each epoch, display the states, the
///     one way light time between target and observer, and the rate of
///     change of the one way light time.
///
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: spkltc_ex1.tm
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
///           pck00010.tpc                  Planet orientation and
///                                         radii
///           naif0010.tls                  Leapseconds
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de421.bsp',
///                               'pck00010.tpc',
///                               'naif0010.tls'  )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM SPKLTC_EX1
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
///           PARAMETER           ( META   = 'spkltc_ex1.tm' )
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
///           DOUBLE PRECISION      DLT
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      ET0
///           DOUBLE PRECISION      LT
///           DOUBLE PRECISION      STATE ( 6 )
///           DOUBLE PRECISION      STOBS ( 6 )
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
///     C           Aberration correction:  Light time ('LT')
///     C           Observer:               Earth (NAIF ID code 399)
///     C
///     C        Before we can execute this computation, we'll need the
///     C        geometric state of the observer relative to the solar
///     C        system barycenter at ET, expressed relative to the
///     C        J2000 reference frame:
///     C
///              CALL SPKSSB ( 399, ET,    'J2000', STOBS )
///     C
///     C        Now compute the desired state vector:
///     C
///              CALL SPKLTC ( 301,   ET,    'J2000', 'LT',
///          .                 STOBS, STATE, LT,      DLT     )
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
///      J2000 x-position (km):     -291569.26516582817
///      J2000 y-position (km):     -266709.18671506643
///      J2000 z-position (km):     -76099.155290968716
///      J2000 x-velocity (km/s):   0.64353061395009092
///      J2000 y-velocity (km/s):  -0.66608181647356979
///      J2000 z-velocity (km/s):  -0.30132283137339932
///      One-way light time (s):     1.3423106103603615
///      Light time rate:            1.0731690854241060E-007
///
///      ET =    3600.0000000000000
///      J2000 x-position (km):     -289240.78103223071
///      J2000 y-position (km):     -269096.44111447036
///      J2000 z-position (km):     -77180.899896450341
///      J2000 x-velocity (km/s):   0.65006211592321250
///      J2000 y-velocity (km/s):  -0.66016273867753217
///      J2000 z-velocity (km/s):  -0.29964267347917639
///      One-way light time (s):     1.3426939548981949
///      Light time rate:            1.0565259879591478E-007
///
///      ET =    7200.0000000000000
///      J2000 x-position (km):     -286888.88711488992
///      J2000 y-position (km):     -271462.30193841457
///      J2000 z-position (km):     -78256.555851273239
///      J2000 x-velocity (km/s):   0.65653599225917958
///      J2000 y-velocity (km/s):  -0.65419657625983696
///      J2000 z-velocity (km/s):  -0.29794027264402967
///      One-way light time (s):     1.3430713117678452
///      Light time rate:            1.0399045674252711E-007
///
///      ET =    10800.000000000000
///      J2000 x-position (km):     -284513.79148214310
///      J2000 y-position (km):     -273806.60054129362
///      J2000 z-position (km):     -79326.043350853026
///      J2000 x-velocity (km/s):   0.66295190125626391
///      J2000 y-velocity (km/s):  -0.64818380654817442
///      J2000 z-velocity (km/s):  -0.29621577893712070
///      One-way light time (s):     1.3434426891028646
///      Light time rate:            1.0233066508729246E-007
///
///      ET =    14400.000000000000
///      J2000 x-position (km):     -282115.70342658088
///      J2000 y-position (km):     -276129.16999696195
///      J2000 z-position (km):     -80389.283131733537
///      J2000 x-velocity (km/s):   0.66930950447965998
///      J2000 y-velocity (km/s):  -0.64212490750332751
///      J2000 z-velocity (km/s):  -0.29446934292511795
///      One-way light time (s):     1.3438080956889309
///      Light time rate:            1.0067340347415892E-007
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The routine SPKGEO should be used instead of this routine
///      to compute geometric states. SPKGEO introduces less
///      round-off error when the observer and target have common
///      center that is closer to both objects than is the solar
///      system barycenter.
///
///  2)  The kernel files to be used by SPKLTC must be loaded
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
/// -    SPICELIB Version 2.0.1, 05-JUL-2021 (JDR)
///
///         Edited the header to comply with NAIF standards.
///         Added FRAMES to the list of $Required_Reading
///
/// -    SPICELIB Version 2.0.0, 04-JUL-2014 (NJB)
///
///         Discussion of light time corrections was updated. Assertions
///         that converged light time corrections are unlikely to be
///         useful were removed.
///
///      Last update was 02-MAY-2012 (NJB)
///
///         Updated to ensure convergence when CN or XCN light time
///         corrections are used. The new algorithm also terminates early
///         (after fewer than three iterations) when convergence is
///         attained.
///
///         Call to ZZPRSCOR was replaced by a call to ZZVALCOR.
///
/// -    SPICELIB Version 1.0.0, 11-JAN-2008 (NJB)
/// ```
pub fn spkltc(
    ctx: &mut SpiceContext,
    targ: i32,
    et: f64,
    ref_: &str,
    abcorr: &str,
    stobs: &[f64; 6],
    starg: &mut [f64; 6],
    lt: &mut f64,
    dlt: &mut f64,
) -> crate::Result<()> {
    SPKLTC(
        targ,
        et,
        ref_.as_bytes(),
        abcorr.as_bytes(),
        stobs,
        starg,
        lt,
        dlt,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKLTC ( S/P Kernel, light time corrected state )
pub fn SPKLTC(
    TARG: i32,
    ET: f64,
    REF: &[u8],
    ABCORR: &[u8],
    STOBS: &[f64],
    STARG: &mut [f64],
    LT: &mut f64,
    DLT: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let STOBS = DummyArray::new(STOBS, 1..=6);
    let mut STARG = DummyArrayMut::new(STARG, 1..=6);
    let mut A: f64 = 0.0;
    let mut B: f64 = 0.0;
    let mut C: f64 = 0.0;
    let mut DIST: f64 = 0.0;
    let mut EPOCH: f64 = 0.0;
    let mut LTERR: f64 = 0.0;
    let mut PRVLT: f64 = 0.0;
    let mut SSBLT: f64 = 0.0;
    let mut SSBTRG = StackArray::<f64, 6>::new(1..=6);
    let mut I: i32 = 0;
    let mut LTSIGN: i32 = 0;
    let mut NUMITR: i32 = 0;
    let mut REFID: i32 = 0;
    let mut ATTBLK = StackArray::<bool, 15>::new(1..=NABCOR);
    let mut USESTL: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // TOL is the tolerance used for a division-by-zero test
    // performed prior to computation of DLT.
    //

    //
    // Convergence limit:
    //

    //
    // Maximum number of light time iterations for any
    // aberration correction:
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
        CHKIN(RNAME, ctx)?;
    }

    if (save.PASS1 || fstr::ne(ABCORR, &save.PRVCOR)) {
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
        //    USELT is .TRUE. when any type of light time correction
        //    (normal or converged Newtonian) is specified.
        //
        //    USECN indicates converged Newtonian light time correction.
        //
        // The above definitions are consistent with those used by
        // ZZVALCOR.
        //
        save.XMIT = ATTBLK[XMTIDX];
        save.USELT = ATTBLK[LTIDX];
        save.USECN = ATTBLK[CNVIDX];
        USESTL = ATTBLK[STLIDX];

        save.PASS1 = false;
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
    // Find the geometric state of the target body with respect to
    // the solar system barycenter. Subtract the state of the
    // observer to get the relative state. Use this to compute the
    // one-way light time.
    //
    SPKGEO(TARG, ET, REF, SSB, SSBTRG.as_slice_mut(), &mut SSBLT, ctx)?;

    if FAILED(ctx) {
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    VSUBG(SSBTRG.as_slice(), STOBS.as_slice(), 6, STARG.as_slice_mut());

    DIST = VNORM(STARG.as_slice());
    *LT = (DIST / CLIGHT());

    if (*LT == 0.0) {
        //
        // This can happen only if the observer and target are at the
        // same position. We don't consider this an error, but we're not
        // going to compute the light time derivative.
        //
        *DLT = 0 as f64;

        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    if !save.USELT {
        //
        // This is a special case: we're not using light time
        // corrections, so the derivative
        // of light time is just
        //
        //    (1/c) * d(VNORM(STARG))/dt
        //
        *DLT = (VDOT(STARG.as_slice(), STARG.subarray(4)) / (DIST * CLIGHT()));

        //
        // LT and DLT are both set, so we can return.
        //
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // To correct for light time, find the state of the target body
    // at the current epoch minus the one-way light time. Note that
    // the observer remains where it is.
    //
    // Determine the sign of the light time offset.
    //
    if save.XMIT {
        LTSIGN = 1;
    } else {
        LTSIGN = -1;
    }

    //
    // Let NUMITR be the number of iterations we'll perform to
    // compute the light time.
    //
    if save.USECN {
        NUMITR = MAXITR;
    } else {
        NUMITR = 1;
    }

    I = 0;
    LTERR = 1.0;

    while ((I < NUMITR) && (LTERR > CNVLIM)) {
        //
        // LT was set either prior to this loop or
        // during the previous loop iteration.
        //
        EPOCH = (ET + ((LTSIGN as f64) * *LT));

        SPKGEO(
            TARG,
            EPOCH,
            REF,
            SSB,
            SSBTRG.as_slice_mut(),
            &mut SSBLT,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        VSUBG(SSBTRG.as_slice(), STOBS.as_slice(), 6, STARG.as_slice_mut());

        PRVLT = *LT;
        *LT = TOUCHD((VNORM(STARG.as_slice()) / CLIGHT()));

        // LTERR is the magnitude of the change between the current
        // estimate of light time and the previous estimate, relative to
        // the previous light time corrected epoch.
        //
        LTERR = TOUCHD((f64::abs((*LT - PRVLT)) / intrinsics::DMAX1(&[1.0, f64::abs(EPOCH)])));
        I = (I + 1);
    }

    //
    // At this point, STARG contains the light time corrected
    // state of the target relative to the observer.
    //
    // Compute the derivative of light time with respect
    // to time: dLT/dt.  Below we derive the formula for
    // this quantity for the reception case. Let
    //
    //    POBS be the position of the observer relative to the
    //    solar system barycenter.
    //
    //    VOBS be the velocity of the observer relative to the
    //    solar system barycenter.
    //
    //    PTARG be the position of the target relative to the
    //    solar system barycenter.
    //
    //    VTARG be the velocity of the target relative to the
    //    solar system barycenter.
    //
    //    S be the sign of the light time correction. S is
    //    negative for the reception case.
    //
    // The light-time corrected position of the target relative to
    // the observer at observation time ET, given the one-way
    // light time LT is:
    //
    //     PTARG(ET+S*LT) - POBS(ET)
    //
    // The light-time corrected velocity of the target relative to
    // the observer at observation time ET is
    //
    //     VTARG(ET+S*LT)*( 1 + S*d(LT)/d(ET) ) - VOBS(ET)
    //
    // We need to compute dLT/dt. Below, we use the facts that,
    // for a time-dependent vector X(t),
    //
    //      ||X||     = <X,X> ** (1/2)
    //
    //    d(||X||)/dt = (1/2)<X,X>**(-1/2) * 2 * <X,dX/dt>
    //
    //                = <X,X>**(-1/2) *  <X,dX/dt>
    //
    //                = <X,dX/dt> / ||X||
    //
    // Newtonian light time equation:
    //
    //    LT     =   (1/c) * || PTARG(ET+S*LT) - POBS(ET)||
    //
    // Differentiate both sides:
    //
    //    dLT/dt =   (1/c) * ( 1 / || PTARG(ET+S*LT) - POBS(ET) || )
    //
    //              * < PTARG(ET+S*LT) - POBS(ET),
    //                  VTARG(ET+S*LT)*(1+S*d(LT)/d(ET)) - VOBS(ET) >
    //
    //
    //           = (1/c) * ( 1 / || PTARG(ET+S*LT) - POBS(ET) || )
    //
    //             * (  < PTARG(ET+S*LT) - POBS(ET),
    //                    VTARG(ET+S*LT) - VOBS(ET) >
    //
    //               +  < PTARG(ET+S*LT) - POBS(ET),
    //                    VTARG(ET+S*LT)           > * (S*d(LT)/d(ET))  )
    //
    // Let
    //
    //    A =   (1/c) * ( 1 / || PTARG(ET+S*LT) - POBS(ET) || )
    //
    //    B =   < PTARG(ET+S*LT) - POBS(ET), VTARG(ET+S*LT) - VOBS(ET) >
    //
    //    C =   < PTARG(ET+S*LT) - POBS(ET), VTARG(ET+S*LT) >
    //
    // Then
    //
    //    d(LT)/d(ET) =  A * ( B  +  C * S*d(LT)/d(ET) )
    //
    // which implies
    //
    //    d(LT)/d(ET) =  A*B / ( 1 - S*C*A )
    //
    //
    //
    A = (1.0 / (CLIGHT() * VNORM(STARG.as_slice())));

    B = VDOT(STARG.as_slice(), STARG.subarray(4));

    C = VDOT(STARG.as_slice(), SSBTRG.subarray(4));

    //
    // For physically realistic target velocities, S*C*A cannot equal 1.
    // We'll check for this case anyway.
    //
    if ((((LTSIGN as f64) * C) * A) > (1.0 - TOL)) {
        SETMSG(b"Target range rate magnitude is approximately the speed of light. The light time derivative cannot be computed.", ctx);
        SIGERR(b"SPICE(DIVIDEBYZERO)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Compute DLT: the rate of change of light time.
    //
    *DLT = ((A * B) / (1.0 - (((LTSIGN as f64) * C) * A)));

    //
    // Overwrite the velocity portion of the output state
    // with the light-time corrected velocity.
    //
    VLCOM(
        (1.0 + ((LTSIGN as f64) * *DLT)),
        SSBTRG.subarray(4),
        -1.0,
        STOBS.subarray(4),
        STARG.subarray_mut(4),
    );

    CHKOUT(RNAME, ctx)?;
    Ok(())
}
