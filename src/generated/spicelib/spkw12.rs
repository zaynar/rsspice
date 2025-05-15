//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const MAXDEG: i32 = 27;
const TOLSCL: f64 = 0.0000000000001;
const SIDLEN: i32 = 40;
const NS: i32 = 5;
const DTYPE: i32 = 12;
const FPRINT: i32 = 32;
const LPRINT: i32 = 126;
const STATSZ: i32 = 6;
const TIMLEN: i32 = 40;

/// SPK, write segment, type 12
///
/// Write a type 12 segment to an SPK file.
///
/// # Required Reading
///
/// * [NAIF_IDS](crate::required_reading::naif_ids)
/// * [SPC](crate::required_reading::spc)
/// * [SPK](crate::required_reading::spk)
/// * [TIME](crate::required_reading::time)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  MAXDEG     P   Maximum degree of interpolating polynomials.
///  TOLSCL     P   Scale factor used to compute time bound tolerance.
///  HANDLE     I   Handle of an SPK file open for writing.
///  BODY       I   NAIF code for an ephemeris object.
///  CENTER     I   NAIF code for center of motion of BODY.
///  FRAME      I   Reference frame name.
///  FIRST      I   Start time of interval covered by segment.
///  LAST       I   End time of interval covered by segment.
///  SEGID      I   Segment identifier.
///  DEGREE     I   Degree of interpolating polynomials.
///  N          I   Number of states.
///  STATES     I   Array of states.
///  BEGTIM     I   Epoch of first state in STATES array.
///  STEP       I   Time step separating epochs of states.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the file handle of an SPK file that has been
///           opened for writing.
///
///  BODY     is the NAIF integer code for an ephemeris object
///           whose state relative to another body is described
///           by the segment to be created.
///
///  CENTER   is the NAIF integer code for the center of motion
///           of the object identified by BODY.
///
///  FRAME    is the NAIF name for a reference frame
///           relative to which the state information for BODY
///           is specified.
///
///  FIRST,
///  LAST     are, respectively, the start and stop times of
///           the time interval over which the segment defines
///           the state of BODY.
///
///  SEGID    is the segment identifier. An SPK segment
///           identifier may contain up to 40 characters.
///
///  DEGREE   is the degree of the Lagrange polynomials used to
///           interpolate the states. All components of the
///           state vectors are interpolated by polynomials of
///           fixed degree.
///
///  N        is the number of states in the input state vector
///           array.
///
///  STATES   contains a time-ordered array of geometric states
///           ( x, y, z, dx/dt, dy/dt, dz/dt, in kilometers and
///           kilometers per second ) of BODY relative to CENTER,
///           specified relative to FRAME.
///
///  BEGTIM   is the epoch corresponding to the first state in
///           the state array. Because extra states are needed
///           at the beginning and end of the segment in order
///           for the interpolation method to work, BEGTIM will
///           normally precede FIRST.
///
///  STEP     is the time step separating the epochs of adjacent
///           states in the input state array. STEP is specified
///           in seconds.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None. See $Particulars for a description of the effect of this
///  routine.
/// ```
///
/// # Parameters
///
/// ```text
///  See the include file spk12.inc for declarations of the
///  parameters described below.
///
///
///  TOLSCL   is a tolerance scale factor (also called a
///           "relative tolerance") used for time coverage
///           bound checking. TOLSCL is unitless. TOLSCL
///           produces a tolerance value via the formula
///
///              TOL = TOLSCL * MAX( ABS(FIRST), ABS(LAST) )
///
///           where FIRST and LAST are the coverage time bounds
///           of a type 12 segment, expressed as seconds past
///           J2000 TDB.
///
///           The resulting parameter TOL is used as a tolerance
///           for comparing the input segment descriptor time
///           bounds to the first and last epoch covered by the
///           sequence of time intervals defined by the inputs
///           to SPKW12:
///
///              BEGTIM
///              STEP
///              N
///
///
///  MAXDEG   is the maximum degree of Hermite polynomials that
///           can be used to interpolate states from the segment
///           written by this routine.
/// ```
///
/// # Exceptions
///
/// ```text
///  If any of the following exceptions occur, this routine will return
///  without creating a new segment.
///
///  1)  If FRAME is not a recognized name, the error
///      SPICE(INVALIDREFFRAME) is signaled.
///
///  2)  If the last non-blank character of SEGID occurs past index 40,
///      the error SPICE(SEGIDTOOLONG) is signaled.
///
///  3)  If SEGID contains any nonprintable characters, the error
///      SPICE(NONPRINTABLECHARS) is signaled.
///
///  4)  If DEGREE is not at least 1 or is greater than MAXDEG, the
///      error SPICE(INVALIDDEGREE) is signaled.
///
///  5)  If DEGREE is not odd, the error SPICE(INVALIDDEGREE) is
///      signaled.
///
///  6)  If the number of states N is not at least (DEGREE+1)/2, the
///      error SPICE(TOOFEWSTATES) is signaled.
///
///  7)  If FIRST is greater than LAST, the error
///      SPICE(BADDESCRTIMES) is signaled.
///
///  8)  If STEP is non-positive, the error SPICE(INVALIDSTEPSIZE) is
///      signaled.
///
///  9)  If the first epoch BEGTIM is greater than FIRST, the error
///      SPICE(BADDESCRTIMES) is signaled.
///
///  10) If the start time of the first record exceeds the descriptor
///      begin time by more than a computed tolerance, or if the end
///      time of the last record precedes the descriptor end time by
///      more than a computed tolerance, the error SPICE(COVERAGEGAP)
///      is signaled. See the $Parameters section for a description of
///      the tolerance.
/// ```
///
/// # Files
///
/// ```text
///  A new type 12 SPK segment is written to the SPK file attached
///  to HANDLE.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine writes an SPK type 12 data segment to the open SPK
///  file according to the format described in the type 12 section of
///  the SPK Required Reading. The SPK file must have been opened with
///  write access.
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
///  1) This example demonstrates how to create an SPK type 12 kernel
///     containing only one segment, given an evenly-spaced
///     time-ordered set of discrete states.
///
///
///     Example code begins here.
///
///
///           PROGRAM SPKW12_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               NAMLEN
///           PARAMETER           ( NAMLEN = 40 )
///
///     C
///     C     Define the segment identifier parameters.
///     C
///           CHARACTER*(*)         SPK12
///           PARAMETER           ( SPK12 = 'spkw12_ex1.bsp' )
///
///           CHARACTER*(*)         REF
///           PARAMETER           ( REF    = 'J2000'          )
///
///           INTEGER               BODY
///           PARAMETER           ( BODY   = 3  )
///
///           INTEGER               CENTER
///           PARAMETER           ( CENTER = 10 )
///
///           INTEGER               DEGREE
///           PARAMETER           ( DEGREE = 3  )
///
///           INTEGER               NSTATS
///           PARAMETER           ( NSTATS = 9  )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(NAMLEN)    IFNAME
///           CHARACTER*(NAMLEN)    SEGID
///
///           DOUBLE PRECISION      FIRST
///           DOUBLE PRECISION      LAST
///           DOUBLE PRECISION      STATES ( 6, NSTATS )
///           DOUBLE PRECISION      STEP
///
///           INTEGER               HANDLE
///           INTEGER               NCOMCH
///
///     C
///     C     Define the states and epochs.
///     C
///           DATA                  STATES /
///          .       101.D0, 201.D0, 301.D0, 401.D0, 501.D0, 601.D0,
///          .       102.D0, 202.D0, 302.D0, 402.D0, 502.D0, 602.D0,
///          .       103.D0, 203.D0, 303.D0, 403.D0, 503.D0, 603.D0,
///          .       104.D0, 204.D0, 304.D0, 404.D0, 504.D0, 604.D0,
///          .       105.D0, 205.D0, 305.D0, 405.D0, 505.D0, 605.D0,
///          .       106.D0, 206.D0, 306.D0, 406.D0, 506.D0, 606.D0,
///          .       107.D0, 207.D0, 307.D0, 407.D0, 507.D0, 607.D0,
///          .       108.D0, 208.D0, 308.D0, 408.D0, 508.D0, 608.D0,
///          .       109.D0, 209.D0, 309.D0, 409.D0, 509.D0, 609.D0 /
///
///     C
///     C     Set the start and end times of interval covered by
///     C     segment, and the time step.
///     C
///           FIRST  = 100.D0
///           LAST   = 900.D0
///           STEP   = 100.D0
///
///     C
///     C     NCOMCH is the number of characters to reserve for the
///     C     kernel's comment area. This example doesn't write
///     C     comments, so set to zero.
///     C
///           NCOMCH = 0
///
///     C
///     C     Internal file name and segment ID.
///     C
///           IFNAME = 'Type 12 SPK internal file name.'
///           SEGID  = 'SPK type 12 test segment'
///
///     C
///     C     Open a new SPK file.
///     C
///           CALL SPKOPN( SPK12, IFNAME, NCOMCH, HANDLE )
///
///     C
///     C     Write the segment.
///     C
///           CALL SPKW12 (  HANDLE, BODY,   CENTER, REF,
///          .               FIRST,  LAST,   SEGID,  DEGREE,
///          .               NSTATS, STATES, FIRST,  STEP   )
///
///     C
///     C     Close the SPK file.
///     C
///           CALL SPKCLS ( HANDLE )
///
///           END
///
///
///     When this program is executed, no output is presented on
///     screen. After run completion, a new SPK type 12 exists in
///     the output directory.
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
/// -    SPICELIB Version 2.1.0, 03-JUN-2021 (JDR)
///
///         Changed the input argument name EPOCH1 to BEGTIM for
///         consistency with other routines.
///
///         Edited the header to comply with NAIF standard. Added
///         complete example code from existing fragment.
///
/// -    SPICELIB Version 2.0.0, 18-JAN-2013 (NJB)
///
///         Relaxed test on relationship between the time bounds of the
///         input record set (determined by BEGTIM, STEP, and N) and the
///         descriptor bounds FIRST and LAST. Now the descriptor bounds
///         may extend beyond the time bounds of the record set by a ratio
///         computed using the parameter TOLSCL (see $Parameters above for
///         details). MAXDEG was increased to 27.
///
///         Corrected long error message for even degree case.
///
/// -    SPICELIB Version 1.0.0, 25-FEB-2000 (NJB)
/// ```
pub fn spkw12(
    ctx: &mut SpiceContext,
    handle: i32,
    body: i32,
    center: i32,
    frame: &str,
    first: f64,
    last: f64,
    segid: &str,
    degree: i32,
    n: i32,
    states: &[[f64; 6]],
    begtim: f64,
    step: f64,
) -> crate::Result<()> {
    SPKW12(
        handle,
        body,
        center,
        frame.as_bytes(),
        first,
        last,
        segid.as_bytes(),
        degree,
        n,
        states.as_flattened(),
        begtim,
        step,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKW12 ( SPK, write segment, type 12 )
pub fn SPKW12(
    HANDLE: i32,
    BODY: i32,
    CENTER: i32,
    FRAME: &[u8],
    FIRST: f64,
    LAST: f64,
    SEGID: &[u8],
    DEGREE: i32,
    N: i32,
    STATES: &[f64],
    BEGTIM: f64,
    STEP: f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let STATES = DummyArray2D::new(STATES, 1..=6, 1..);
    let mut ETSTR = [b' '; TIMLEN as usize];
    let mut DESCR = StackArray::<f64, 5>::new(1..=NS);
    let mut LTIME: f64 = 0.0;
    let mut TOL: f64 = 0.0;
    let mut CHRCOD: i32 = 0;
    let mut REFCOD: i32 = 0;
    let mut WINSIZ: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // SIDLEN is the maximum number of characters allowed in an
    // SPK segment identifier.
    //
    // NS is the size of a packed SPK segment descriptor.
    //
    // ND is the number of double precision components in an SPK
    // segment descriptor.
    //
    // NI is the number of integer components in an SPK segment
    // descriptor.
    //
    // DTYPE is the data type.
    //
    // FPRINT is the integer value of the first printable ASCII
    // character.
    //
    // LPRINT is the integer value of the last printable ASCII character.
    //
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"SPKW12", ctx)?;
    }

    //
    // Set the window size corresponding to the input degree.  This
    // size will be used in various places below.
    //
    WINSIZ = ((DEGREE + 1) / 2);

    //
    // Get the NAIF integer code for the reference frame.
    //
    NAMFRM(FRAME, &mut REFCOD, ctx)?;

    if (REFCOD == 0) {
        SETMSG(b"The reference frame # is not supported.", ctx);
        ERRCH(b"#", FRAME, ctx);
        SIGERR(b"SPICE(INVALIDREFFRAME)", ctx)?;
        CHKOUT(b"SPKW12", ctx)?;
        return Ok(());
    }

    //
    // Check to see if the segment identifier is too long.
    //
    if (LASTNB(SEGID) > SIDLEN) {
        SETMSG(b"Segment identifier contains more than 40 characters.", ctx);
        SIGERR(b"SPICE(SEGIDTOOLONG)", ctx)?;
        CHKOUT(b"SPKW12", ctx)?;
        return Ok(());
    }

    //
    // Now check that all the characters in the segment identifier
    // can be printed.
    //
    for I in 1..=LASTNB(SEGID) {
        CHRCOD = intrinsics::ICHAR(fstr::substr(SEGID, I..=I));

        if ((CHRCOD < FPRINT) || (CHRCOD > LPRINT)) {
            SETMSG(
                b"The segment identifier contains nonprintable characters",
                ctx,
            );
            SIGERR(b"SPICE(NONPRINTABLECHARS)", ctx)?;
            CHKOUT(b"SPKW12", ctx)?;
            return Ok(());
        }
    }

    //
    // Make sure that the degree of the interpolating polynomials is
    // in range.
    //
    if ((DEGREE < 1) || (DEGREE > MAXDEG)) {
        SETMSG(
            b"The interpolating polynomials have degree #; the valid degree range is [1, #]",
            ctx,
        );
        ERRINT(b"#", DEGREE, ctx);
        ERRINT(b"#", MAXDEG, ctx);
        SIGERR(b"SPICE(INVALIDDEGREE)", ctx)?;
        CHKOUT(b"SPKW12", ctx)?;
        return Ok(());
    }

    //
    // Make sure that the degree of the interpolating polynomials is odd.
    //
    if EVEN(DEGREE) {
        SETMSG(b"The interpolating polynomials have degree #; for SPK type 12, the degree must be odd.", ctx);
        ERRINT(b"#", DEGREE, ctx);
        SIGERR(b"SPICE(INVALIDDEGREE)", ctx)?;
        CHKOUT(b"SPKW12", ctx)?;
        return Ok(());
    }

    //
    // Make sure that the number of states is sufficient to define a
    // polynomial whose degree is DEGREE.
    //
    if (N < WINSIZ) {
        SETMSG(b"At least # states are required to define a Hermite polynomial of degree #.  Number of states supplied:  #", ctx);
        ERRINT(b"#", WINSIZ, ctx);
        ERRINT(b"#", DEGREE, ctx);
        ERRINT(b"#", N, ctx);
        SIGERR(b"SPICE(TOOFEWSTATES)", ctx)?;
        CHKOUT(b"SPKW12", ctx)?;
        return Ok(());
    }

    //
    // The segment stop time should be greater then the begin time.
    //
    if (FIRST >= LAST) {
        SETMSG(
            b"The segment start time: # is greater then the segment end time: #",
            ctx,
        );
        ERRDP(b"#", FIRST, ctx);
        ERRDP(b"#", LAST, ctx);
        SIGERR(b"SPICE(BADDESCRTIMES)", ctx)?;
        CHKOUT(b"SPKW12", ctx)?;
        return Ok(());
    }

    //
    // The step size must be positive.
    //
    if (STEP <= 0.0) {
        SETMSG(b"The step size must be > 0 but was #. ", ctx);
        ERRDP(b"#", STEP, ctx);
        SIGERR(b"SPICE(INVALIDSTEPSIZE)", ctx)?;
        CHKOUT(b"SPKW12", ctx)?;
        return Ok(());
    }

    //
    // Compute the tolerance to use for descriptor time bound checks.
    //
    TOL = (TOLSCL * intrinsics::DMAX1(&[f64::abs(FIRST), f64::abs(LAST)]));

    if (FIRST < (BEGTIM - TOL)) {
        SETMSG(b"The segment descriptor start time # is too much less than the beginning time of the  segment data # (in seconds past J2000: #). The difference is # seconds; the  tolerance is # seconds.", ctx);
        ETCAL(FIRST, &mut ETSTR, ctx);
        ERRCH(b"#", &ETSTR, ctx);
        ETCAL(BEGTIM, &mut ETSTR, ctx);
        ERRCH(b"#", &ETSTR, ctx);
        ERRDP(b"#", FIRST, ctx);
        ERRDP(b"#", (BEGTIM - FIRST), ctx);
        ERRDP(b"#", TOL, ctx);
        SIGERR(b"SPICE(COVERAGEGAP)", ctx)?;
        CHKOUT(b"SPKW12", ctx)?;
        return Ok(());
    }

    //
    // The end time of the final record must be greater than or
    // equal to the end time of the segment.
    //
    LTIME = (BEGTIM + (((N - 1) as f64) * STEP));

    if (LAST > (LTIME + TOL)) {
        SETMSG(b"The segment descriptor end time # is too much greater than the end time of the segment data # (in seconds past J2000: #). The difference is # seconds; the tolerance is # seconds.", ctx);
        ETCAL(LAST, &mut ETSTR, ctx);
        ERRCH(b"#", &ETSTR, ctx);
        ETCAL(LTIME, &mut ETSTR, ctx);
        ERRCH(b"#", &ETSTR, ctx);
        ERRDP(b"#", LAST, ctx);
        ERRDP(b"#", (LAST - LTIME), ctx);
        ERRDP(b"#", TOL, ctx);
        SIGERR(b"SPICE(COVERAGEGAP)", ctx)?;
        CHKOUT(b"SPKW12", ctx)?;
        return Ok(());
    }

    //
    // If we made it this far, we're ready to start writing the segment.
    //
    //
    // Create the segment descriptor.
    //
    SPKPDS(
        BODY,
        CENTER,
        FRAME,
        DTYPE,
        FIRST,
        LAST,
        DESCR.as_slice_mut(),
        ctx,
    )?;

    //
    // Begin a new segment.
    //
    DAFBNA(HANDLE, DESCR.as_slice(), SEGID, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SPKW12", ctx)?;
        return Ok(());
    }

    //
    // The type 12 segment structure is eloquently described by this
    // diagram from the SPK Required Reading:
    //
    //    +-----------------------+
    //    | State 1               |
    //    +-----------------------+
    //    | State 2               |
    //    +-----------------------+
    //                .
    //                .
    //                .
    //    +-----------------------+
    //    | State N               |
    //    +-----------------------+
    //    | Epoch of state 1 (ET) |
    //    +-----------------------+
    //    | Step size             |
    //    +-----------------------+
    //    | Window size - 1       |
    //    +-----------------------+
    //    | Number of states      |
    //    +-----------------------+
    //
    //
    DAFADA(STATES.as_slice(), (N * STATSZ), ctx)?;
    DAFADA(&[BEGTIM], 1, ctx)?;
    DAFADA(&[STEP], 1, ctx)?;
    DAFADA(&[((WINSIZ - 1) as f64)], 1, ctx)?;
    DAFADA(&[(N as f64)], 1, ctx)?;

    //
    // As long as nothing went wrong, end the segment.
    //
    if !FAILED(ctx) {
        DAFENA(ctx)?;
    }

    CHKOUT(b"SPKW12", ctx)?;
    Ok(())
}
