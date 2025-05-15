//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const S18TP0: i32 = 0;
const S18TP1: i32 = (S18TP0 + 1);
const S18PS0: i32 = 12;
const S18PS1: i32 = 6;
const MAXDEG: i32 = 15;
const SIDLEN: i32 = 40;
const FPRINT: i32 = 32;
const LPRINT: i32 = 126;
const ND: i32 = 2;
const NI: i32 = 6;
const DSCSIZ: i32 = 5;
const TYPIDX: i32 = 4;
const DTYPE: i32 = 18;
const DIRSIZ: i32 = 100;
const STATSZ: i32 = 6;

/// Write SPK segment, type 18
///
/// Write a type 18 segment to an SPK file.
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
///  HANDLE     I   Handle of an SPK file open for writing.
///  SUBTYP     I   SPK type 18 subtype code.
///  BODY       I   NAIF code for an ephemeris object.
///  CENTER     I   NAIF code for center of motion of BODY.
///  FRAME      I   Reference frame name.
///  FIRST      I   Start time of interval covered by segment.
///  LAST       I   End time of interval covered by segment.
///  SEGID      I   Segment identifier.
///  DEGREE     I   Degree of interpolating polynomials.
///  N          I   Number of packets.
///  PACKTS     I   Array of packets.
///  EPOCHS     I   Array of epochs corresponding to packets.
///  MAXDEG     P   Maximum allowed degree of interpolating polynomial.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the file handle of an SPK file that has been opened
///           for writing.
///
///  SUBTYP   is an integer code indicating the subtype of the
///           segment to be created.
///
///  BODY     is the NAIF integer code for an ephemeris object whose
///           state relative to another body is described by the
///           segment to be created.
///
///  CENTER   is the NAIF integer code for the center of motion of the
///           object identified by BODY.
///
///  FRAME    is the NAIF name for a reference frame relative to which
///           the state information for BODY is specified.
///
///  FIRST,
///  LAST     are, respectively, the start and stop times of the time
///           interval over which the segment defines the state of
///           BODY.
///
///  SEGID    is the segment identifier. An SPK segment identifier may
///           contain up to 40 characters.
///
///  DEGREE   is the nominal degree of the polynomials used to
///           interpolate the states contained in the input packets.
///           All components of the state vectors are interpolated by
///           polynomials of the specified degree, except near the
///           segment boundaries, or if the total number of states in
///           the segment is too few to allow interpolation using the
///           specified degree.
///
///           If the actual interpolation degree is reduced, the
///           highest degree feasible degree valid for the
///           interpolation type is used.
///
///  N        is the number of packets in the input packet array.
///
///  PACKTS   is a time-ordered array of data packets representing
///           geometric states of BODY relative to CENTER, specified
///           relative to FRAME. The packet structure depends on the
///           segment subtype as follows:
///
///              Type 0 (indicated by code S18TP0):
///
///                 x,  y,  z,  dx/dt,  dy/dt,  dz/dt,
///                 vx, vy, vz, dvx/dt, dvy/dt, dvz/dt
///
///              where x, y, z represent Cartesian position components
///              and  vx, vy, vz represent Cartesian velocity
///              components. Note well: vx, vy, and vz *are not
///              necessarily equal* to the time derivatives of x, y,
///              and z. This packet structure mimics that of the
///              Rosetta/MEX orbit file from which the data are taken.
///
///              Type 1 (indicated by code S18TP1):
///
///                 x,  y,  z,  dx/dt,  dy/dt,  dz/dt
///
///              where x, y, z represent Cartesian position components
///              and  vx, vy, vz represent Cartesian velocity
///              components.
///
///           Position units are kilometers, velocity units are
///           kilometers per second, and acceleration units are
///           kilometers per second per second.
///
///  EPOCHS   is an array of epochs corresponding to the members of the
///           packets array. The epochs are specified as seconds past
///           J2000, TDB.
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
///  MAXDEG   is the maximum allowed degree of the interpolating
///           polynomial. If the value of MAXDEG is increased, the
///           SPICELIB routine SPKPVN must be changed accordingly. In
///           particular, the size of the record passed to SPKRnn and
///           SPKEnn must be increased, and comments describing the
///           record size must be changed.
/// ```
///
/// # Exceptions
///
/// ```text
///  If any of the following exceptions occur, this routine will
///  return without creating a new segment.
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
///  5)  If the window size implied by DEGREE is odd, the error
///      SPICE(INVALIDDEGREE) is signaled.
///
///  6)  If the number of packets N is not at least 2, the error
///      SPICE(TOOFEWSTATES) is signaled.
///
///  7)  If FIRST is greater than or equal to LAST, the error
///      SPICE(BADDESCRTIMES) is signaled.
///
///  8)  If the elements of the array EPOCHS are not in strictly
///      increasing order, the error SPICE(TIMESOUTOFORDER) is
///      signaled.
///
///  9)  If the first epoch EPOCHS(1) is greater than FIRST, the error
///      SPICE(BADDESCRTIMES) is signaled.
///
///  10) If the last epoch EPOCHS(N) is less than LAST, the error
///      SPICE(BADDESCRTIMES) is signaled.
///
///  11) If the subtype code is not recognized, the error
///      SPICE(INVALIDVALUE) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  A new type 18 SPK segment is written to the SPK file attached
///  to HANDLE.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine writes an SPK type 18 data segment to the open SPK
///  file according to the format described in the type 18 section of
///  the SPK Required Reading. The SPK file must have been opened with
///  write access.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that you have states and are prepared to produce
///  a segment of type 18 in an SPK file.
///
///  The following code fragment could be used to add the new segment
///  to a previously opened SPK file attached to HANDLE. The file must
///  have been opened with write access.
///
///     C
///     C     Create a segment identifier.
///     C
///               SEGID = 'MY_SAMPLE_SPK_TYPE_18_SEGMENT'
///
///     C
///     C     Write the segment.
///     C
///           CALL SPKW18 (  HANDLE,  BODY,    CENTER,  FRAME,
///          .               FIRST,   LAST,    SEGID,   DEGREE,
///          .               N,       STATES,  EPOCHS          )
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
/// -    SPICELIB Version 1.1.1, 09-APR-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.1.0, 21-DEC-2012 (NJB)
///
///         Increased the minimum packet count from 1 to 2.
///
/// -    SPICELIB Version 1.0.1, 29-APR-2003 (NJB)
///
///         Description of error condition arising from invalid window
///         size was corrected.
///
/// -    SPICELIB Version 1.0.0, 13-MAY-2002 (NJB)
/// ```
pub fn spkw18(
    ctx: &mut SpiceContext,
    handle: i32,
    subtyp: i32,
    body: i32,
    center: i32,
    frame: &str,
    first: f64,
    last: f64,
    segid: &str,
    degree: i32,
    n: i32,
    packts: &[f64],
    epochs: &[f64],
) -> crate::Result<()> {
    SPKW18(
        handle,
        subtyp,
        body,
        center,
        frame.as_bytes(),
        first,
        last,
        segid.as_bytes(),
        degree,
        n,
        packts,
        epochs,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKW18 ( Write SPK segment, type 18 )
pub fn SPKW18(
    HANDLE: i32,
    SUBTYP: i32,
    BODY: i32,
    CENTER: i32,
    FRAME: &[u8],
    FIRST: f64,
    LAST: f64,
    SEGID: &[u8],
    DEGREE: i32,
    N: i32,
    PACKTS: &[f64],
    EPOCHS: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let PACKTS = DummyArray::new(PACKTS, 1..);
    let EPOCHS = DummyArray::new(EPOCHS, 1..);
    let mut DC = StackArray::<f64, 2>::new(1..=ND);
    let mut DESCR = StackArray::<f64, 5>::new(1..=DSCSIZ);
    let mut MAXTIM: f64 = 0.0;
    let mut CHRCOD: i32 = 0;
    let mut IC = StackArray::<i32, 6>::new(1..=NI);
    let mut PACKSZ: i32 = 0;
    let mut REFCOD: i32 = 0;
    let mut WINSIZ: i32 = 0;

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
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"SPKW18", ctx)?;
    }

    //
    // Set the packet size, which is a function of the subtype.
    //
    if (SUBTYP == S18TP0) {
        PACKSZ = S18PS0;
    } else if (SUBTYP == S18TP1) {
        PACKSZ = S18PS1;
    } else {
        SETMSG(b"Unexpected SPK type 18 subtype requested: #", ctx);
        ERRINT(b"#", SUBTYP, ctx);
        SIGERR(b"SPICE(INVALIDVALUE)", ctx)?;
        CHKOUT(b"SPKW18", ctx)?;
        return Ok(());
    }

    //
    // Set the window size corresponding to the input degree.  This
    // size will be used in various places below.
    //
    if (SUBTYP == S18TP0) {
        WINSIZ = ((DEGREE + 1) / 2);
    } else if (SUBTYP == S18TP1) {
        WINSIZ = (DEGREE + 1);
    } else {
        SETMSG(b"This point should not be reached. Getting here may indicate that the code needs to updated to handle new subtypes.", ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"SPKW18", ctx)?;
        return Ok(());
    }

    //
    // Get the NAIF integer code for the reference frame.
    //
    NAMFRM(FRAME, &mut REFCOD, ctx)?;

    if (REFCOD == 0) {
        SETMSG(b"The reference frame # is not supported.", ctx);
        ERRCH(b"#", FRAME, ctx);
        SIGERR(b"SPICE(INVALIDREFFRAME)", ctx)?;
        CHKOUT(b"SPKW18", ctx)?;
        return Ok(());
    }

    //
    // Check to see if the segment identifier is too long.
    //
    if (LASTNB(SEGID) > SIDLEN) {
        SETMSG(b"Segment identifier contains more than 40 characters.", ctx);
        SIGERR(b"SPICE(SEGIDTOOLONG)", ctx)?;
        CHKOUT(b"SPKW18", ctx)?;
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
            CHKOUT(b"SPKW18", ctx)?;
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
        CHKOUT(b"SPKW18", ctx)?;
        return Ok(());
    }

    //
    // Make sure that the window size is even.  If not, the input
    // DEGREE is incompatible with the subtype.
    //
    if ODD(WINSIZ) {
        SETMSG(b"The interpolating polynomials have degree #; for SPK type 18, the degree must be equivalent to 3 mod 4 for Hermite interpolation and odd for for Lagrange interpolation.", ctx);
        ERRINT(b"#", DEGREE, ctx);
        SIGERR(b"SPICE(INVALIDDEGREE)", ctx)?;
        CHKOUT(b"SPKW18", ctx)?;
        return Ok(());
    }

    //
    // Make sure that the number of packets is sufficient to define a
    // polynomial whose degree is DEGREE.
    //
    if (N < 2) {
        SETMSG(
            b"At least 2 packets are required for SPK type 18.  Number of packets supplied:  #",
            ctx,
        );
        ERRINT(b"#", N, ctx);
        SIGERR(b"SPICE(TOOFEWSTATES)", ctx)?;
        CHKOUT(b"SPKW18", ctx)?;
        return Ok(());
    }

    //
    // The segment stop time should be greater than or equal to
    // the begin time.
    //
    if (FIRST > LAST) {
        SETMSG(
            b"The segment start time: # is greater then the segment end time: #",
            ctx,
        );
        ERRDP(b"#", FIRST, ctx);
        ERRDP(b"#", LAST, ctx);
        SIGERR(b"SPICE(BADDESCRTIMES)", ctx)?;
        CHKOUT(b"SPKW18", ctx)?;
        return Ok(());
    }

    //
    // Make sure the epochs form a strictly increasing sequence.
    //
    MAXTIM = EPOCHS[1];

    for I in 2..=N {
        if (EPOCHS[I] <= MAXTIM) {
            SETMSG(
                b"EPOCH # having index # is not greater than its predecessor #.",
                ctx,
            );
            ERRDP(b"#", EPOCHS[I], ctx);
            ERRINT(b"#", I, ctx);
            ERRDP(b"#", EPOCHS[(I - 1)], ctx);
            SIGERR(b"SPICE(TIMESOUTOFORDER)", ctx)?;
            CHKOUT(b"SPKW18", ctx)?;
            return Ok(());
        } else {
            MAXTIM = EPOCHS[I];
        }
    }

    //
    // Make sure that the span of the input epochs includes the interval
    // defined by the segment descriptor.
    //
    if (EPOCHS[1] > FIRST) {
        SETMSG(b"Segment start time # precedes first epoch #.", ctx);
        ERRDP(b"#", FIRST, ctx);
        ERRDP(b"#", EPOCHS[1], ctx);
        SIGERR(b"SPICE(BADDESCRTIMES)", ctx)?;
        CHKOUT(b"SPKW18", ctx)?;
        return Ok(());
    } else if (EPOCHS[N] < LAST) {
        SETMSG(b"Segment end time # follows last epoch #.", ctx);
        ERRDP(b"#", LAST, ctx);
        ERRDP(b"#", EPOCHS[N], ctx);
        SIGERR(b"SPICE(BADDESCRTIMES)", ctx)?;
        CHKOUT(b"SPKW18", ctx)?;
        return Ok(());
    }

    //
    // If we made it this far, we're ready to start writing the segment.
    //
    //
    // Create the segment descriptor.  We don't use SPKPDS because
    // that routine doesn't allow creation of a singleton segment.
    //
    IC[1] = BODY;
    IC[2] = CENTER;

    NAMFRM(FRAME, &mut IC[3], ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SPKW18", ctx)?;
        return Ok(());
    }

    IC[4] = DTYPE;

    DC[1] = FIRST;
    DC[2] = LAST;

    DAFPS(ND, NI, DC.as_slice(), IC.as_slice(), DESCR.as_slice_mut());

    //
    // Begin a new segment.
    //
    DAFBNA(HANDLE, DESCR.as_slice(), SEGID, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SPKW18", ctx)?;
        return Ok(());
    }

    //
    // The type 18 segment structure is eloquently described by this
    // diagram from the SPK Required Reading:
    //
    //    +-----------------------+
    //    | Packet 1              |
    //    +-----------------------+
    //    | Packet 2              |
    //    +-----------------------+
    //                .
    //                .
    //                .
    //    +-----------------------+
    //    | Packet N              |
    //    +-----------------------+
    //    | Epoch 1               |
    //    +-----------------------+
    //    | Epoch 2               |
    //    +-----------------------+
    //                .
    //                .
    //                .
    //    +-----------------------+
    //    | Epoch N               |
    //    +-----------------------+
    //    | Epoch 100             | (First directory)
    //    +-----------------------+
    //                .
    //                .
    //                .
    //    +-----------------------+
    //    | Epoch ((N-1)/100)*100 | (Last directory)
    //    +-----------------------+
    //    | Subtype code          |
    //    +-----------------------+
    //    | Window size           |
    //    +-----------------------+
    //    | Number of packets     |
    //    +-----------------------+
    //
    //
    DAFADA(PACKTS.as_slice(), (N * PACKSZ), ctx)?;
    DAFADA(EPOCHS.as_slice(), N, ctx)?;

    for I in 1..=((N - 1) / DIRSIZ) {
        DAFADA(EPOCHS.subarray((DIRSIZ * I)), 1, ctx)?;
    }

    DAFADA(&[(SUBTYP as f64)], 1, ctx)?;
    DAFADA(&[(WINSIZ as f64)], 1, ctx)?;
    DAFADA(&[(N as f64)], 1, ctx)?;

    //
    // As long as nothing went wrong, end the segment.
    //
    if !FAILED(ctx) {
        DAFENA(ctx)?;
    }

    CHKOUT(b"SPKW18", ctx)?;
    Ok(())
}
