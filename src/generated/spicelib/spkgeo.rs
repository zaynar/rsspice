//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const NINERT: i32 = 21;
const CTRSIZ: i32 = 2;
const RNAME: &[u8] = b"SPKGEO";
const CHLEN: i32 = 20;
const SSB: i32 = 0;
const FRNMLN: i32 = 32;

struct SaveVars {
    SVCTR1: StackArray<i32, 2>,
    SVREF: Vec<u8>,
    SVREFI: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SVCTR1 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVREF = vec![b' '; FRNMLN as usize];
        let mut SVREFI: i32 = 0;
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            SVCTR1,
            SVREF,
            SVREFI,
            FIRST,
        }
    }
}

fn ISINRT(TMPFRM: i32, CFRAME: i32) -> bool {
    ((((CFRAME > 0) && (CFRAME <= NINERT)) && (TMPFRM > 0)) && (TMPFRM <= NINERT))
}

/// S/P Kernel, geometric state
///
/// Compute the geometric state (position and velocity) of a target
/// body relative to an observing body.
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
///  ET         I   Target epoch.
///  REF        I   Target reference frame.
///  OBS        I   Observing body.
///  STATE      O   State of target.
///  LT         O   Light time.
/// ```
///
/// # Detailed Input
///
/// ```text
///  TARG     is the standard NAIF ID code for a target body.
///
///  ET       is the epoch (ephemeris time) at which the state
///           of the target body is to be computed.
///
///  REF      is the name of the reference frame to
///           which the vectors returned by the routine should
///           be rotated. This may be any frame supported by
///           the SPICELIB subroutine FRMCHG.
///
///  OBS      is the standard NAIF ID code for an observing body.
/// ```
///
/// # Detailed Output
///
/// ```text
///  STATE    is a 6-dimensional vector that contains the geometric
///           position and velocity of the target body, relative to the
///           observing body, at epoch ET. STATE has six elements: the
///           first three contain the target's position; the last three
///           contain the target's velocity. These vectors are
///           transformed into the specified reference frame. Units are
///           always km and km/sec.
///
///  LT       is the one-way light time in seconds from the
///           observing body to the geometric position of the
///           target body at the specified epoch.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If insufficient ephemeris data has been loaded to compute
///      the necessary states, the error SPICE(SPKINSUFFDATA) is
///      signaled.
/// ```
///
/// # Files
///
/// ```text
///  See $Restrictions.
/// ```
///
/// # Particulars
///
/// ```text
///  SPKGEO computes the geometric state, T(t), of the target
///  body and the geometric state, O(t), of the observing body
///  relative to the first common center of motion. Subtracting
///  O(t) from T(t) gives the geometric state of the target
///  body relative to the observer.
///
///
///     CENTER ----- O(t)
///         |      /
///         |     /
///         |    /
///         |   /  T(t) - O(t)
///         |  /
///        T(t)
///
///
///  The one-way light time, tau, is given by
///
///
///            | T(t) - O(t) |
///     tau = -----------------
///                   c
///
///
///  For example, if the observing body is -94, the Mars Observer
///  spacecraft, and the target body is 401, Phobos, then the
///  first common center is probably 4, the Mars Barycenter.
///  O(t) is the state of -94 relative to 4 and T(t) is the
///  state of 401 relative to 4.
///
///  The center could also be the Solar System Barycenter, body 0.
///  For example, if the observer is 399, Earth, and the target
///  is 299, Venus, then O(t) would be the state of 399 relative
///  to 0 and T(t) would be the state of 299 relative to 0.
///
///  Ephemeris data from more than one segment may be required
///  to determine the states of the target body and observer
///  relative to a common center. SPKGEO reads as many segments
///  as necessary, from as many files as necessary, using files
///  that have been loaded by previous calls to FURNSH or SPKLEF
///  (load ephemeris file).
///
///  SPKGEO is similar to SPKEZ but returns geometric states
///  only, with no option to make planetary (light-time) nor
///  stellar aberration corrections. The geometric states
///  returned by SPKEZ and SPKGEO are the same.
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
///  1) Return the geometric state vector of Mars (499) as seen from
///     Earth (399) in the J2000 frame and the one-way light time
///     between them at the epoch July 4, 2003 11:00 AM PST.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File: spkgeo_ex1.tm
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
///           File name                        Contents
///           ---------                        --------
///           de430.bsp                        Planetary ephemeris
///           mar097.bsp                       Mars satellite ephemeris
///           naif0011.tls                     Leapseconds
///
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de430.bsp',
///                               'mar097.bsp',
///                               'naif0011.tls' )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM SPKGEO_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      VNORM
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               NAMLEN
///           PARAMETER           ( NAMLEN = 32 )
///
///           INTEGER               TIMLEN
///           PARAMETER           ( TIMLEN = 26 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(TIMLEN)    EPOCH
///           CHARACTER*(NAMLEN)    REFFRM
///
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      LT
///           DOUBLE PRECISION      STATE ( 6 )
///
///           INTEGER               I
///           INTEGER               OBSRVR
///           INTEGER               TARGET
///
///
///     C
///     C     Load a set of kernels: an SPK file, a PCK
///     C     file and a leapseconds file. Use a meta
///     C     kernel for convenience.
///     C
///           CALL FURNSH ( 'spkgeo_ex1.tm' )
///
///     C
///     C     Define parameters for a state lookup.
///     C
///           TARGET = 499
///           EPOCH  = 'July 4, 2003 11:00 AM PST'
///           REFFRM = 'J2000'
///           OBSRVR = 399
///
///     C
///     C     Convert the epoch to ephemeris time.
///     C
///           CALL STR2ET ( EPOCH, ET )
///
///     C
///     C     Look-up the state for the defined parameters.
///     C
///           CALL SPKGEO ( TARGET, ET, REFFRM, OBSRVR, STATE, LT )
///
///     C
///     C     Output...
///     C
///           WRITE(*,'(A,I3)') 'The position of    : ', TARGET
///           WRITE(*,'(A,I3)') 'As observed from   : ', OBSRVR
///           WRITE(*,'(2A)')   'In reference frame : ', REFFRM
///           WRITE(*,'(2A)')   'At epoch           : ', EPOCH
///           WRITE(*,*) ' '
///
///     C
///     C     The first three entries of state contain the
///     C     X, Y, Z position components. The final three contain
///     C     the Vx, Vy, Vz velocity components.
///     C
///           WRITE(*,'(A,3F18.6)') 'R   (km):', ( STATE(I), I=1,3 )
///           WRITE(*,'(A,3F18.6)') 'V (km/s):', ( STATE(I), I=4,6 )
///           WRITE(*,*) ' '
///           WRITE(*,'(A,F19.14)') 'Light time (s) between observer '
///          .                   // 'and target: ', LT
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     The position of    : 499
///     As observed from   : 399
///     In reference frame : J2000
///     At epoch           : July 4, 2003 11:00 AM PST
///
///     R   (km):   73826216.435288  -27128030.732406  -18741973.868287
///     V (km/s):         -6.809504          7.513814          3.001290
///
///     Light time (s) between observer and target:  269.70264776317532
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The ephemeris files to be used by SPKGEO must be loaded
///      by FURNSH or SPKLEF before SPKGEO is called.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  J.E. McLean        (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.1.0, 09-OCT-2021 (JDR) (NJB)
///
///         Bug fix: added calls to FAILED after calls to SPKPVN.
///         Previously only one call to SPKPVN was followed by a FAILED
///         call. Moved some FAILED checks so they will be hit whether
///         or not SPKSFS finds a segment.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example to $Examples section. Removed unnecessary
///         $Revisions section.
///
///         Added reference to FURNSH in $Particulars and $Restrictions
///         sections.
///
/// -    SPICELIB Version 3.0.0, 08-JAN-2014 (BVS)
///
///         Updated to save the input frame name and POOL state counter
///         and to do frame name-ID conversion only if the counter has
///         changed.
///
///         Updated to map the input frame name to its ID by first calling
///         ZZNAMFRM, and then calling IRFNUM. The side effect of this
///         change is that now the frame with the fixed name 'DEFAULT'
///         that can be associated with any code via CHGIRF's entry point
///         IRFDEF will be fully masked by a frame with identical name
///         defined via a text kernel. Previously the CHGIRF's 'DEFAULT'
///         frame masked the text kernel frame with the same name.
///
///         Fixed description of STATE in Detailed Output. Replaced
///         SPKLEF with FURNSH and fixed errors in $Examples.
///
/// -    SPICELIB Version 2.4.0, 01-SEP-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in VADDG calls.
///
/// -    SPICELIB Version 2.3.0, 05-JAN-2005 (NJB)
///
///         Tests of routine FAILED() were added.
///
/// -    SPICELIB Version 2.2.1, 20-OCT-2003 (EDW)
///
///         Added mention that LT returns in seconds.
///
/// -    SPICELIB Version 2.2.0, 11-APR-1997 (WLT)
///
///         The routine was modified to take advantage of the fact
///         that most state transformation are between inertial frames.
///         Looking up a transformation between inertial frames is
///         substantially faster than looking up non-inertial
///         transformations. Consequently slightly more
///         complex code produces about a 50% increase in speed for
///         many users.
///
/// -    SPICELIB Version 2.1.0, 26-JUL-1996 (WLT)
///
///         The routine was upgraded so that potentially redundant
///         computations are not performed.
///
/// -    SPICELIB Version 2.0.0, 19-SEP-1995 (WLT)
///
///         The routine was upgraded so that it can return states
///         relative to rotating frames.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 18-JUL-1991 (JEM)
/// ```
pub fn spkgeo(
    ctx: &mut SpiceContext,
    targ: i32,
    et: f64,
    ref_: &str,
    obs: i32,
    state: &mut [f64; 6],
    lt: &mut f64,
) -> crate::Result<()> {
    SPKGEO(targ, et, ref_.as_bytes(), obs, state, lt, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKGEO ( S/P Kernel, geometric state )
pub fn SPKGEO(
    TARG: i32,
    ET: f64,
    REF: &[u8],
    OBS: i32,
    STATE: &mut [f64],
    LT: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut STATE = DummyArrayMut::new(STATE, 1..=6);
    let mut IDENT = [b' '; 40 as usize];
    let mut TNAME = [b' '; 40 as usize];
    let mut ONAME = [b' '; 40 as usize];
    let mut TSTRING = [b' '; 80 as usize];
    let mut DESCR = StackArray::<f64, 5>::new(1..=5);
    let mut SOBS = StackArray::<f64, 6>::new(1..=6);
    let mut STARG = StackArray2D::<f64, 120>::new(1..=6, 1..=CHLEN);
    let mut STEMP = StackArray::<f64, 6>::new(1..=6);
    let mut STXFRM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut ROT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut VTEMP = StackArray::<f64, 6>::new(1..=6);
    let mut CFRAME: i32 = 0;
    let mut COBS: i32 = 0;
    let mut CTARG = StackArray::<i32, 20>::new(1..=CHLEN);
    let mut TFRAME = StackArray::<i32, 20>::new(1..=CHLEN);
    let mut CTPOS: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut I: i32 = 0;
    let mut LEGS: i32 = 0;
    let mut NCT: i32 = 0;
    let mut REFID: i32 = 0;
    let mut TMPFRM: i32 = 0;
    let mut FOUND: bool = false;
    let mut NOFRM: bool = false;

    //
    // This is the idea:
    //
    // Every body moves with respect to some center. The center
    // is itself a body, which in turn moves about some other
    // center.  If we begin at the target body (T), follow
    // the chain,
    //
    //                               T
    //                                 \
    //       SSB                        \
    //           \                     C[1]
    //            \                     /
    //             \                   /
    //              \                 /
    //               \               /
    //              C[3]-----------C[2]
    //
    // and avoid circular definitions (A moves about B, and B moves
    // about A), eventually we get the state relative to the solar
    // system barycenter (which, for our purposes, doesn't move).
    // Thus,
    //
    //    T    = T     + C[1]     + C[2]     + ... + C[n]
    //     SSB    C[1]       C[2]       [C3]             SSB
    //
    // where
    //
    //    X
    //     Y
    //
    // is the state of body X relative to body Y.
    //
    // However, we don't want to follow each chain back to the SSB
    // if it isn't necessary.  Instead we will just follow the chain
    // of the target body and follow the chain of the observing body
    // until we find a common node in the tree.
    //
    // In the example below, C is the first common node.  We compute
    // the state of TARG relative to C and the state of OBS relative
    // to C, then subtract the two states.
    //
    //                               TARG
    //                                 \
    //       SSB                        \
    //           \                       A
    //            \                     /            OBS
    //             \                   /              |
    //              \                 /               |
    //               \               /                |
    //                B-------------C-----------------D
    //
    //

    //
    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // CHLEN is the maximum length of a chain.  That is,
    // it is the maximum number of bodies in the chain from
    // the target or observer to the SSB.
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
    // Saved frame name/ID items.
    //

    //
    // Initial values.
    //

    //
    // In-line Function Definitions
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
    // Initialization.
    //
    if save.FIRST {
        //
        // Initialize counter.
        //
        ZZCTRUIN(save.SVCTR1.as_slice_mut(), ctx);

        save.FIRST = false;
    }

    //
    // We take care of the obvious case first.  It TARG and OBS are the
    // same we can just fill in zero.
    //
    if (TARG == OBS) {
        *LT = 0.0;

        CLEARD(6, STATE.as_slice_mut());
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // CTARG contains the integer codes of the bodies in the
    // target body chain, beginning with TARG itself and then
    // the successive centers of motion.
    //
    // STARG(1,I) is the state of the target body relative
    // to CTARG(I).  The id-code of the frame of this state is
    // stored in TFRAME(I).
    //
    // COBS and SOBS will contain the centers and states of the
    // observing body.  (They are single elements instead of arrays
    // because we only need the current center and state of the
    // observer relative to it.)
    //
    // First, we construct CTARG and STARG.  CTARG(1) is
    // just the target itself, and STARG(1,1) is just a zero
    // vector, that is, the state of the target relative
    // to itself.
    //
    // Then we follow the chain, filling up CTARG and STARG
    // as we go.  We use SPKSFS to search through loaded
    // files to find the first segment applicable to CTARG(1)
    // and time ET.  Then we use SPKPVN to compute the state
    // of the body CTARG(1) at ET in the segment that was found
    // and get its center and frame of motion (CTARG(2) and TFRAME(2).
    //
    // We repeat the process for CTARG(2) and so on, until
    // there is no data found for some CTARG(I) or until we
    // reach the SSB.
    //
    // Next, we find centers and states in a similar manner
    // for the observer.  It's a similar construction as
    // described above, but I is always 1.  COBS and SOBS
    // are overwritten with each new center and state,
    // beginning at OBS.  However, we stop when we encounter
    // a common center of motion, that is when COBS is equal
    // to CTARG(I) for some I.
    //
    // Finally, we compute the desired state of the target
    // relative to the observer by subtracting the state of
    // the observing body relative to the common node from
    // the state of the target body relative to the common
    // node.
    //
    // CTPOS is the position in CTARG of the common node.
    //
    // Since the upgrade to use hashes and counter bypass ZZNAMFRM
    // became more efficient in looking up frame IDs than IRFNUM. So the
    // original order of calls "IRFNUM first, NAMFRM second" was
    // switched to "ZZNAMFRM first, IRFNUM second".
    //
    // The call to IRFNUM, now redundant for built-in inertial frames,
    // was preserved to for a sole reason -- to still support the
    // ancient and barely documented ability for the users to associate
    // a frame with the fixed name 'DEFAULT' with any CHGIRF inertial
    // frame code via CHGIRF's entry point IRFDEF.
    //
    // Note that in the case of ZZNAMFRM's failure to resolve name and
    // IRFNUM's success to do so, the code returned by IRFNUM for
    // 'DEFAULT' frame is *not* copied to the saved code SVREFI (which
    // would be set to 0 by ZZNAMFRM) to make sure that on subsequent
    // calls ZZNAMFRM does not do a bypass (as SVREFI always forced look
    // up) and calls IRFNUM again to reset the 'DEFAULT's frame ID
    // should it change between the calls.
    //
    ZZNAMFRM(
        save.SVCTR1.as_slice_mut(),
        &mut save.SVREF,
        &mut save.SVREFI,
        REF,
        &mut REFID,
        ctx,
    )?;

    if (REFID == 0) {
        IRFNUM(REF, &mut REFID, ctx);
    }

    if (REFID == 0) {
        if (FRSTNP(REF) > 0) {
            SETMSG(b"The string supplied to specify the reference frame, (\'#\') contains non-printing characters.  The two most common causes for this kind of error are: 1. an error in the call to SPKGEO; 2. an uninitialized variable. ", ctx);
            ERRCH(b"#", REF, ctx);
        } else if fstr::eq(REF, b" ") {
            SETMSG(b"The string supplied to specify the reference frame is blank.  The most common cause for this kind of error is an uninitialized variable. ", ctx);
        } else {
            SETMSG(b"The string supplied to specify the reference frame was \'#\'.  This frame is not recognized. Possible causes for this error are: 1. failure to load the frame definition into the kernel pool; 2. An out-of-date edition of the toolkit. ", ctx);
            ERRCH(b"#", REF, ctx);
        }

        SIGERR(b"SPICE(UNKNOWNFRAME)", ctx)?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }
    }

    //
    // Fill in CTARG and STARG until no more data is found
    // or until we reach the SSB.  If the chain gets too
    // long to fit in CTARG, that is if I equals CHLEN,
    // then overwrite the last elements of CTARG and STARG.
    //
    // Note the check for FAILED in the loop.  If SPKSFS
    // or SPKPVN happens to fail during execution, and the
    // current error handling action is to NOT abort, then
    // FOUND may be stuck at TRUE, CTARG(I) will never
    // become zero, and the loop will execute indefinitely.
    //
    //
    // Construct CTARG and STARG.  Begin by assigning the
    // first elements:  TARG and the state of TARG relative
    // to itself.
    //
    I = 1;
    CTARG[I] = TARG;
    FOUND = true;

    CLEARD(6, STARG.subarray_mut([1, I]));

    while (((FOUND && (I < CHLEN)) && (CTARG[I] != OBS)) && (CTARG[I] != SSB)) {
        //
        // Find a file and segment that has state
        // data for CTARG(I).
        //
        SPKSFS(
            CTARG[I],
            ET,
            &mut HANDLE,
            DESCR.as_slice_mut(),
            &mut IDENT,
            &mut FOUND,
            ctx,
        )?;

        if FOUND {
            //
            // Get the state of CTARG(I) relative to some
            // center of motion.  This new center goes in
            // CTARG(I+1) and the state is called STEMP.
            //
            I = (I + 1);

            SPKPVN(
                HANDLE,
                DESCR.as_slice(),
                ET,
                &mut TFRAME[I],
                STARG.subarray_mut([1, I]),
                &mut CTARG[I],
                ctx,
            )?;
            //
            // Here's what we have.  STARG is the state of CTARG(I-1)
            // relative to CTARG(I) in reference frame TFRAME(I)
            //
        }
        //
        // If one of the routines above failed during
        // execution, we just give up and check out.
        //
        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }
    }

    TFRAME[1] = TFRAME[2];
    //
    // If the loop above ended because we ran out of
    // room in the arrays CTARG and STARG, then we
    // continue finding states but we overwrite the
    // last elements of CTARG and STARG.
    //
    // If, as a result, the first common node is
    // overwritten, we'll just have to settle for
    // the last common node.  This will cause a small
    // loss of precision, but it's better than other
    // alternatives.
    //
    if (I == CHLEN) {
        while ((FOUND && (CTARG[CHLEN] != SSB)) && (CTARG[CHLEN] != OBS)) {
            //
            // Find a file and segment that has state
            // data for CTARG(CHLEN).
            //
            SPKSFS(
                CTARG[CHLEN],
                ET,
                &mut HANDLE,
                DESCR.as_slice_mut(),
                &mut IDENT,
                &mut FOUND,
                ctx,
            )?;

            if FOUND {
                //
                // Get the state of CTARG(CHLEN) relative to
                // some center of motion.  The new center
                // overwrites the old.  The state is called
                // STEMP.
                //
                SPKPVN(
                    HANDLE,
                    DESCR.as_slice(),
                    ET,
                    &mut TMPFRM,
                    STEMP.as_slice_mut(),
                    &mut CTARG[CHLEN],
                    ctx,
                )?;

                if FAILED(ctx) {
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }

                //
                // Add STEMP to the state of TARG relative to
                // the old center to get the state of TARG
                // relative to the new center.  Overwrite
                // the last element of STARG.
                //
                if (TFRAME[CHLEN] == TMPFRM) {
                    MOVED(STARG.subarray([1, CHLEN]), 6, VTEMP.as_slice_mut());
                } else if ISINRT(TFRAME[CHLEN], TMPFRM) {
                    IRFROT(TFRAME[CHLEN], TMPFRM, ROT.as_slice_mut(), ctx)?;
                    MXV(
                        ROT.as_slice(),
                        STARG.subarray([1, CHLEN]),
                        VTEMP.subarray_mut(1),
                    );
                    MXV(
                        ROT.as_slice(),
                        STARG.subarray([4, CHLEN]),
                        VTEMP.subarray_mut(4),
                    );
                } else {
                    FRMCHG(TFRAME[CHLEN], TMPFRM, ET, STXFRM.as_slice_mut(), ctx)?;

                    if FAILED(ctx) {
                        CHKOUT(RNAME, ctx)?;
                        return Ok(());
                    }

                    MXVG(
                        STXFRM.as_slice(),
                        STARG.subarray([1, CHLEN]),
                        6,
                        6,
                        VTEMP.as_slice_mut(),
                    );
                }

                VADDG(
                    VTEMP.as_slice(),
                    STEMP.as_slice(),
                    6,
                    STARG.subarray_mut([1, CHLEN]),
                );

                TFRAME[CHLEN] = TMPFRM;
            }

            //
            // If one of the routines above failed during
            // execution, we just give up and check out.
            //
            if FAILED(ctx) {
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }
        }
    }

    NCT = I;
    //
    // NCT is the number of elements in CTARG,
    // the chain length.  We have in hand the following information
    //
    //    STARG(1...6,K)  state of body
    //    CTARG(K-1)      relative to body CTARG(K) in the frame
    //    TFRAME(K)
    //
    //
    // For K = 2,..., NCT.
    //
    // CTARG(1) = TARG
    // STARG(1...6,1) = ( 0, 0, 0, 0, 0, 0 )
    // TFRAME(1)      = TFRAME(2)
    //

    //
    // Now follow the observer's chain.  Assign
    // the first values for COBS and SOBS.
    //
    COBS = OBS;
    CLEARD(6, SOBS.as_slice_mut());

    //
    // Perhaps we have a common node already.
    // If so it will be the last node on the
    // list CTARG.
    //
    // We let CTPOS will be the position of the common
    // node in CTARG if one is found.  It will
    // be zero if COBS is not found in CTARG.
    //
    if (CTARG[NCT] == COBS) {
        CTPOS = NCT;
        CFRAME = TFRAME[CTPOS];
    } else {
        CTPOS = 0;
    }

    //
    // Repeat the same loop as above, but each time
    // we encounter a new center of motion, check to
    // see if it is a common node.  (When CTPOS is
    // not zero, CTARG(CTPOS) is the first common node.)
    //
    // Note that we don't need a centers array nor a
    // states array, just a single center and state
    // is sufficient --- we just keep overwriting them.
    // When the common node is found, we have everything
    // we need in that one center (COBS) and state
    // (SOBS-state of the target relative to COBS).
    //
    FOUND = true;
    NOFRM = true;
    LEGS = 0;

    while ((FOUND && (COBS != SSB)) && (CTPOS == 0)) {
        //
        // Find a file and segment that has state
        // data for COBS.
        //
        SPKSFS(
            COBS,
            ET,
            &mut HANDLE,
            DESCR.as_slice_mut(),
            &mut IDENT,
            &mut FOUND,
            ctx,
        )?;

        if FOUND {
            //
            // Get the state of COBS; call it STEMP.
            // The center of motion of COBS becomes the
            // new COBS.
            //
            if (LEGS == 0) {
                SPKPVN(
                    HANDLE,
                    DESCR.as_slice(),
                    ET,
                    &mut TMPFRM,
                    SOBS.as_slice_mut(),
                    &mut COBS,
                    ctx,
                )?;
            } else {
                SPKPVN(
                    HANDLE,
                    DESCR.as_slice(),
                    ET,
                    &mut TMPFRM,
                    STEMP.as_slice_mut(),
                    &mut COBS,
                    ctx,
                )?;
            }

            if FAILED(ctx) {
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            if NOFRM {
                NOFRM = false;
                CFRAME = TMPFRM;
            }
            //
            // Add STEMP to the state of OBS relative to
            // the old COBS to get the state of OBS
            // relative to the new COBS.
            //
            if (CFRAME == TMPFRM) {
                //
                // On the first leg of the state of the observer, we
                // don't have to add anything, the state of the observer
                // is already in SOBS.  We only have to add when the
                // number of legs in the observer state is one or greater.
                //
                if (LEGS > 0) {
                    VADDG(SOBS.as_slice(), STEMP.as_slice(), 6, VTEMP.as_slice_mut());
                    MOVED(VTEMP.as_slice(), 6, SOBS.as_slice_mut());
                }
            } else if ISINRT(CFRAME, TMPFRM) {
                IRFROT(CFRAME, TMPFRM, ROT.as_slice_mut(), ctx)?;
                MXV(ROT.as_slice(), SOBS.subarray(1), VTEMP.subarray_mut(1));
                MXV(ROT.as_slice(), SOBS.subarray(4), VTEMP.subarray_mut(4));
                VADDG(VTEMP.as_slice(), STEMP.as_slice(), 6, SOBS.as_slice_mut());
                CFRAME = TMPFRM;
            } else {
                FRMCHG(CFRAME, TMPFRM, ET, STXFRM.as_slice_mut(), ctx)?;

                if FAILED(ctx) {
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }

                MXVG(
                    STXFRM.as_slice(),
                    SOBS.as_slice(),
                    6,
                    6,
                    VTEMP.as_slice_mut(),
                );
                VADDG(VTEMP.as_slice(), STEMP.as_slice(), 6, SOBS.as_slice_mut());
                CFRAME = TMPFRM;
            }

            //
            // We now have one more leg of the path for OBS.  Set
            // LEGS to reflect this.  Then see if the new center
            // is a common node. If not, repeat the loop.
            //
            LEGS = (LEGS + 1);
            CTPOS = ISRCHI(COBS, NCT, CTARG.as_slice());
        }

        //
        // Check failed.  We don't want to loop indefinitely.
        //
        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }
    }

    //
    // If CTPOS is zero at this point, it means we
    // have not found a common node though we have
    // searched through all the available data.
    //
    if (CTPOS == 0) {
        BODC2N(TARG, &mut TNAME, &mut FOUND, ctx)?;

        if FOUND {
            PREFIX(b"# (", 0, &mut TNAME);
            SUFFIX(b")", 0, &mut TNAME);
            REPMI(&TNAME.clone(), b"#", TARG, &mut TNAME, ctx);
        } else {
            INTSTR(TARG, &mut TNAME, ctx);
        }

        BODC2N(OBS, &mut ONAME, &mut FOUND, ctx)?;

        if FOUND {
            PREFIX(b"# (", 0, &mut ONAME);
            SUFFIX(b")", 0, &mut ONAME);
            REPMI(&ONAME.clone(), b"#", OBS, &mut ONAME, ctx);
        } else {
            INTSTR(OBS, &mut ONAME, ctx);
        }

        SETMSG(b"Insufficient ephemeris data has been loaded to compute the state of TARG relative to OBS at the ephemeris epoch #. ", ctx);

        ETCAL(ET, &mut TSTRING, ctx);
        ERRCH(b"TARG", &TNAME, ctx);
        ERRCH(b"OBS", &ONAME, ctx);
        ERRCH(b"#", &TSTRING, ctx);
        SIGERR(b"SPICE(SPKINSUFFDATA)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // If CTPOS is not zero, then we have reached a
    // common node, specifically,
    //
    //    CTARG(CTPOS) = COBS = CENTER
    //
    // (in diagram below).  The STATE of the target
    // (TARG) relative to the observer (OBS) is just
    //
    //    STARG(1,CTPOS) - SOBS.
    //
    //
    //
    //                 SOBS
    //     CENTER ---------------->OBS
    //        |                  .
    //        |                .
    //     S  |              .   E
    //     T  |            .   T
    //     A  |          .   A
    //     R  |        .   T
    //     G  |      .   S
    //        |    .
    //        |  .
    //        V L
    //       TARG
    //
    //
    // And the light-time between them is just
    //
    //           | STATE |
    //      LT = ---------
    //               c
    //
    //
    // Compute the state of the target relative to CTARG(CTPOS)
    //
    if (CTPOS == 1) {
        TFRAME[1] = CFRAME;
    }

    {
        let m1__: i32 = 2;
        let m2__: i32 = (CTPOS - 1);
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            if (TFRAME[I] == TFRAME[(I + 1)]) {
                VADDG(
                    STARG.subarray([1, I]),
                    STARG.subarray([1, (I + 1)]),
                    6,
                    VTEMP.as_slice_mut(),
                );
                MOVED(VTEMP.as_slice(), 6, STARG.subarray_mut([1, (I + 1)]));
            } else if ISINRT(TFRAME[I], TFRAME[(I + 1)]) {
                IRFROT(TFRAME[I], TFRAME[(I + 1)], ROT.as_slice_mut(), ctx)?;
                MXV(
                    ROT.as_slice(),
                    STARG.subarray([1, I]),
                    STEMP.subarray_mut(1),
                );
                MXV(
                    ROT.as_slice(),
                    STARG.subarray([4, I]),
                    STEMP.subarray_mut(4),
                );
                VADDG(
                    STEMP.as_slice(),
                    STARG.subarray([1, (I + 1)]),
                    6,
                    VTEMP.as_slice_mut(),
                );
                MOVED(VTEMP.as_slice(), 6, STARG.subarray_mut([1, (I + 1)]));
            } else {
                FRMCHG(TFRAME[I], TFRAME[(I + 1)], ET, STXFRM.as_slice_mut(), ctx)?;

                if FAILED(ctx) {
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }

                MXVG(
                    STXFRM.as_slice(),
                    STARG.subarray([1, I]),
                    6,
                    6,
                    STEMP.as_slice_mut(),
                );
                VADDG(
                    STEMP.as_slice(),
                    STARG.subarray([1, (I + 1)]),
                    6,
                    VTEMP.as_slice_mut(),
                );
                MOVED(VTEMP.as_slice(), 6, STARG.subarray_mut([1, (I + 1)]));
            }

            I += m3__;
        }
    }

    //
    // To avoid unnecessary frame transformations we'll do
    // a bit of extra decision making here.  It's a lot
    // faster to make logical checks than it is to compute
    // frame transformations.
    //
    if (TFRAME[CTPOS] == CFRAME) {
        VSUBG(
            STARG.subarray([1, CTPOS]),
            SOBS.as_slice(),
            6,
            STATE.as_slice_mut(),
        );
    } else if (TFRAME[CTPOS] == REFID) {
        //
        // If the last frame associated with the target is already
        // in the requested output frame, we convert the state of
        // the observer to that frame and then subtract the state
        // of the observer from the state of the target.
        //
        if ISINRT(CFRAME, REFID) {
            IRFROT(CFRAME, REFID, ROT.as_slice_mut(), ctx)?;
            MXV(ROT.as_slice(), SOBS.subarray(1), STEMP.subarray_mut(1));
            MXV(ROT.as_slice(), SOBS.subarray(4), STEMP.subarray_mut(4));
        } else {
            FRMCHG(CFRAME, REFID, ET, STXFRM.as_slice_mut(), ctx)?;

            if FAILED(ctx) {
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            MXVG(
                STXFRM.as_slice(),
                SOBS.as_slice(),
                6,
                6,
                STEMP.as_slice_mut(),
            );
        }

        //
        // We've now transformed SOBS into the requested reference frame.
        // Set CFRAME to reflect this.
        //
        CFRAME = REFID;
        VSUBG(
            STARG.subarray([1, CTPOS]),
            STEMP.as_slice(),
            6,
            STATE.as_slice_mut(),
        );
    } else if ISINRT(TFRAME[CTPOS], CFRAME) {
        //
        // If both frames are inertial we use IRFROT instead of
        // FRMCHG to get things into a common frame.
        //
        IRFROT(TFRAME[CTPOS], CFRAME, ROT.as_slice_mut(), ctx)?;
        MXV(
            ROT.as_slice(),
            STARG.subarray([1, CTPOS]),
            STEMP.subarray_mut(1),
        );
        MXV(
            ROT.as_slice(),
            STARG.subarray([4, CTPOS]),
            STEMP.subarray_mut(4),
        );
        VSUBG(STEMP.as_slice(), SOBS.as_slice(), 6, STATE.as_slice_mut());
    } else {
        //
        // Use the more general routine FRMCHG to make the transformation.
        //
        FRMCHG(TFRAME[CTPOS], CFRAME, ET, STXFRM.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        MXVG(
            STXFRM.as_slice(),
            STARG.subarray([1, CTPOS]),
            6,
            6,
            STEMP.as_slice_mut(),
        );
        VSUBG(STEMP.as_slice(), SOBS.as_slice(), 6, STATE.as_slice_mut());
    }

    //
    // Finally, rotate as needed into the requested frame.
    //
    if (CFRAME == REFID) {

        //
        // We don't have to do anything in this case.
        //
    } else if ISINRT(CFRAME, REFID) {
        //
        // Since both frames are inertial, we use the more direct
        // routine IRFROT to get the transformation to REFID.
        //
        IRFROT(CFRAME, REFID, ROT.as_slice_mut(), ctx)?;
        MXV(ROT.as_slice(), STATE.subarray(1), STEMP.subarray_mut(1));
        MXV(ROT.as_slice(), STATE.subarray(4), STEMP.subarray_mut(4));
        MOVED(STEMP.as_slice(), 6, STATE.as_slice_mut());
    } else {
        FRMCHG(CFRAME, REFID, ET, STXFRM.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        MXVG(
            STXFRM.as_slice(),
            STATE.as_slice(),
            6,
            6,
            STEMP.as_slice_mut(),
        );
        MOVED(STEMP.as_slice(), 6, STATE.as_slice_mut());
    }

    *LT = (VNORM(STATE.as_slice()) / CLIGHT());

    CHKOUT(RNAME, ctx)?;
    Ok(())
}
