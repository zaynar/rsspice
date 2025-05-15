//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const DLSIZE: i32 = 71;
const SIDLEN: i32 = 40;
const FPRINT: i32 = 32;
const LPRINT: i32 = 126;
const DSCSIZ: i32 = 5;
const TYPIDX: i32 = 4;
const DTYPE: i32 = 1;
const DIRSIZ: i32 = 100;

/// Write SPK segment, type 1
///
/// Write a type 1 segment to an SPK file.
///
/// # Required Reading
///
/// * [NAIF_IDS](crate::required_reading::naif_ids)
/// * [SPK](crate::required_reading::spk)
/// * [TIME](crate::required_reading::time)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of an SPK file open for writing.
///  BODY       I   NAIF code for an ephemeris object.
///  CENTER     I   NAIF code for center of motion of BODY.
///  FRAME      I   Reference frame name.
///  FIRST      I   Start time of interval covered by segment.
///  LAST       I   End time of interval covered by segment.
///  SEGID      I   Segment identifier.
///  N          I   Number of difference lines in segment.
///  DLINES     I   Array of difference lines.
///  EPOCHS     I   Coverage end times of difference lines.
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
///  FRAME    is the NAIF name for a reference frame relative to
///           which the state information for BODY is specified.
///
///  FIRST,
///  LAST     are, respectively, the start and stop times of
///           the time interval over which the segment defines
///           the state of BODY.
///
///  SEGID    is the segment identifier. An SPK segment
///           identifier may contain up to 40 characters.
///
///  N        is the number of difference lines in the input
///           difference line array.
///
///  DLINES   contains a time-ordered array of difference lines
///           The Ith difference line occupies elements (1,I)
///           through (71,I) of DLINES. Each difference line
///           represents the state (x, y, z, dx/dt, dy/dt,
///           dz/dt, in kilometers and kilometers per second)
///           of BODY relative to CENTER, specified relative to
///           FRAME, for an interval of time. The time interval
///           covered by the Ith difference line ends at the
///           Ith element of the array EPOCHS (described below).
///           The interval covered by the first difference line
///           starts at the segment start time.
///
///           The contents of a difference line are as shown
///           below:
///
///              Dimension  Description
///              ---------  ----------------------------------
///              1          Reference epoch of difference line
///              15         Stepsize function vector
///              1          Reference position vector,  x
///              1          Reference velocity vector,  x
///              1          Reference position vector,  y
///              1          Reference velocity vector,  y
///              1          Reference position vector,  z
///              1          Reference velocity vector,  z
///              15,3       Modified divided difference
///                         arrays (MDAs)
///              1          Maximum integration order plus 1
///              3          Integration order array
///
///           The reference position and velocity are those of
///           BODY relative to CENTER at the reference epoch.
///           (A difference line is essentially a polynomial
///           expansion of acceleration about the reference
///           epoch.)
///
///
///  EPOCHS   is an array of epochs corresponding to the members
///           of the state array. The epochs are specified as
///           seconds past J2000, TDB.
///
///           The first difference line covers the time interval
///           from the segment start time to EPOCHS(1).  For
///           I > 1, the Ith difference line covers the half-open
///           time interval from, but not including, EPOCHS(I-1)
///           through EPOCHS(I).
///
///           The elements of EPOCHS must be strictly increasing.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None. See $Particulars for a description of the effect of this
///  routine.
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
///  4)  If the number of difference lines N is not at least one,
///      the error SPICE(INVALIDCOUNT) is signaled.
///
///  5)  If FIRST is greater than or equal to LAST, the error
///      SPICE(BADDESCRTIMES) is signaled.
///
///  6)  If the elements of the array EPOCHS are not in strictly
///      increasing order, the error SPICE(TIMESOUTOFORDER) is
///      signaled.
///
///  7)  If the last epoch EPOCHS(N) is less than LAST, the error
///      SPICE(BADDESCRTIMES) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  A new type 1 SPK segment is written to the SPK file attached
///  to HANDLE.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine writes an SPK type 1 data segment to the open SPK
///  file according to the format described in the type 1 section of
///  the SPK Required Reading. The SPK file must have been opened with
///  write access.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that you have difference lines and are prepared to
///  produce a segment of type 1 in an SPK file.
///
///  The following code fragment could be used to add the new segment
///  to a previously opened SPK file attached to HANDLE. The file must
///  have been opened with write access.
///
///     C
///     C     Create a segment identifier.
///     C
///               SEGID = 'MY_SAMPLE_SPK_TYPE_1_SEGMENT'
///
///     C
///     C     Write the segment.
///     C
///           CALL SPKW01 (  HANDLE,  BODY,    CENTER,  FRAME,
///          .               FIRST,   LAST,    SEGID,   N,
///          .               DLINES,  EPOCHS                  )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The validity of the difference lines is not checked by
///      this routine.
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
/// -    SPICELIB Version 1.0.2, 03-JUN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.1, 07-APR-2010 (NJB)
///
///         Updated $Detailed_Input to state that the elements
///         of EPOCHS must be strictly increasing. The $Exceptions
///         section already described this error condition.
///
/// -    SPICELIB Version 1.0.0, 30-JAN-2003 (NJB)
/// ```
pub fn spkw01(
    ctx: &mut SpiceContext,
    handle: i32,
    body: i32,
    center: i32,
    frame: &str,
    first: f64,
    last: f64,
    segid: &str,
    n: i32,
    dlines: &[[f64; 71]],
    epochs: &[f64],
) -> crate::Result<()> {
    SPKW01(
        handle,
        body,
        center,
        frame.as_bytes(),
        first,
        last,
        segid.as_bytes(),
        n,
        dlines.as_flattened(),
        epochs,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKW01 ( Write SPK segment, type 1 )
pub fn SPKW01(
    HANDLE: i32,
    BODY: i32,
    CENTER: i32,
    FRAME: &[u8],
    FIRST: f64,
    LAST: f64,
    SEGID: &[u8],
    N: i32,
    DLINES: &[f64],
    EPOCHS: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DLINES = DummyArray2D::new(DLINES, 1..=DLSIZE, 1..);
    let EPOCHS = DummyArray::new(EPOCHS, 1..);
    let mut DESCR = StackArray::<f64, 5>::new(1..=DSCSIZ);
    let mut MAXTIM: f64 = 0.0;
    let mut CHRCOD: i32 = 0;
    let mut REFCOD: i32 = 0;

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
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"SPKW01", ctx)?;
    }

    //
    // Get the NAIF integer code for the reference frame.
    //
    NAMFRM(FRAME, &mut REFCOD, ctx)?;

    if (REFCOD == 0) {
        SETMSG(b"The reference frame # is not supported.", ctx);
        ERRCH(b"#", FRAME, ctx);
        SIGERR(b"SPICE(INVALIDREFFRAME)", ctx)?;
        CHKOUT(b"SPKW01", ctx)?;
        return Ok(());
    }

    //
    // Check to see if the segment identifier is too long.
    //
    if (LASTNB(SEGID) > SIDLEN) {
        SETMSG(b"Segment identifier contains more than 40 characters.", ctx);
        SIGERR(b"SPICE(SEGIDTOOLONG)", ctx)?;
        CHKOUT(b"SPKW01", ctx)?;
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
            CHKOUT(b"SPKW01", ctx)?;
            return Ok(());
        }
    }

    //
    // The difference line count must be at least one.
    //
    if (N < 1) {
        SETMSG(
            b"The difference line count was #; the count must be at least one.",
            ctx,
        );
        ERRINT(b"#", N, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"SPKW01", ctx)?;
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
        CHKOUT(b"SPKW01", ctx)?;
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
            CHKOUT(b"SPKW01", ctx)?;
            return Ok(());
        } else {
            MAXTIM = EPOCHS[I];
        }
    }

    //
    // Make sure there's no gap between the last difference line
    // epoch and the end of the time interval defined by the segment
    // descriptor.
    //
    if (EPOCHS[N] < LAST) {
        SETMSG(b"Segment end time # follows last epoch #.", ctx);
        ERRDP(b"#", LAST, ctx);
        ERRDP(b"#", EPOCHS[N], ctx);
        SIGERR(b"SPICE(BADDESCRTIMES)", ctx)?;
        CHKOUT(b"SPKW01", ctx)?;
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
        CHKOUT(b"SPKW01", ctx)?;
        return Ok(());
    }

    //
    // The type 1 segment structure is shown below:
    //
    //    +-----------------------+
    //    | Difference line 1     |
    //    +-----------------------+
    //    | Difference line 2     |
    //    +-----------------------+
    //                .
    //                .
    //                .
    //    +-----------------------+
    //    | Difference line N     |
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
    //    | Epoch (N/100)*100     | (Last directory)
    //    +-----------------------+
    //    | Number of diff lines  |
    //    +-----------------------+
    //
    //

    DAFADA(DLINES.as_slice(), (N * DLSIZE), ctx)?;
    DAFADA(EPOCHS.as_slice(), N, ctx)?;

    for I in 1..=(N / DIRSIZ) {
        DAFADA(EPOCHS.subarray((DIRSIZ * I)), 1, ctx)?;
    }

    DAFADA(&[(N as f64)], 1, ctx)?;

    //
    // As long as nothing went wrong, end the segment.
    //
    if !FAILED(ctx) {
        DAFENA(ctx)?;
    }

    CHKOUT(b"SPKW01", ctx)?;
    Ok(())
}
