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
const MSIZE: i32 = 3;

/// S/P Kernel, subset, type 18
///
/// Extract a subset of the data in an SPK segment of type 18
/// into a new segment.
///
/// # Required Reading
///
/// * [SPK](crate::required_reading::spk)
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of file containing source segment.
///  BADDR      I   Beginning address in file of source segment.
///  EADDR      I   Ending address in file of source segment.
///  BEGIN      I   Beginning (initial epoch) of subset.
///  END        I   End (final epoch) of subset.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE,
///  BADDR,
///  EADDR    are the file handle assigned to an SPK file, and the
///           beginning and ending addresses of a segment within
///           that file. Together they determine a complete set of
///           ephemeris data, from which a subset is to be
///           extracted.
///
///  BEGIN,
///  END      are the initial and final epochs (ephemeris time)
///           of the subset.
///
///           The first epoch for which there will be ephemeris
///           data in the new segment will be the greatest time
///           in the source segment that is less than or equal
///           to BEGIN.
///
///           The last epoch for which there will be ephemeris
///           data in the new segment will be the smallest time
///           in the source segment that is greater than or equal
///           to END.
/// ```
///
/// # Detailed Output
///
/// ```text
///  See $Files section.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  This routine relies on the caller to ensure that the
///      interval [BEGIN, END] is contained in the coverage
///      interval of the segment.
///
///  2)  If BEGIN > END, no data are written to the target file.
///
///  3)  If an unexpected SPK type 18 subtype is found in the input
///      segment, the error SPICE(INVALIDVALUE) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  Data are extracted from the file connected to the input
///  handle, and written to the current DAF open for writing.
///
///  The segment descriptor and summary must already have been written
///  prior to calling this routine. The segment must be ended
///  external to this routine.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is intended solely for use as a utility by the
///  routine SPKSUB.
///
///  It transfers a subset of a type 18 SPK data segment to
///  a properly initialized segment of a second SPK file.
///
///  The exact structure of a segment of data type 18 is described
///  in the section on type 18 in the SPK Required Reading.
/// ```
///
/// # Examples
///
/// ```text
///  This routine is intended only for use as a utility by SPKSUB.
///  To use this routine successfully, you must:
///
///     Open the SPK file from which to extract data.
///     Locate the segment from which data should be extracted.
///
///     Open the SPK file to which this data should be written.
///     Begin a new segment (array).
///     Write the summary information for the array.
///
///     Call this routine to extract the appropriate data from the
///     SPK open for read.
///
///     End the array to which this routine writes data.
///
///  Much of this procedure is carried out by the routine SPKSUB. The
///  examples of that routine illustrate more fully the process
///  described above.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.1, 03-JUN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.1.0, 22-DEC-2012 (NJB)
///
///         Bug fix: code applicable to SPK type 9 for
///         creating padding in the output segment was
///         deleted.
///
/// -    SPICELIB Version 1.0.0, 16-AUG-2002 (NJB) (WLT) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.1.0, 22-DEC-2012 (NJB)
///
///         Bug fix: code applicable to SPK type 9 for
///         creating padding in the output segment was
///         deleted.
///
///         The offending code was meant to ensure that
///         the output segment's size is at least the
///         window size corresponding to the segment's
///         interpolation degree. This is correct behavior
///         for SPK types 9 and 13; for these types,
///         segments are not allowed to have sizes less
///         than the nominal window size.
///
///         However, for type 18, segments can have as
///         few as two data packets, regardless of their
///         interpolation degree. The code that creates
///         padding packets in this case reads from
///         invalid locations.
///
///         Also, the variable WINSIZ was introduced, and
///         comments indicating that the stored size
///         parameter in the segment control area is the
///         window size minus one were corrected.
/// ```
pub fn spks18(
    ctx: &mut SpiceContext,
    handle: i32,
    baddr: i32,
    eaddr: i32,
    begin: f64,
    end: f64,
) -> crate::Result<()> {
    SPKS18(handle, baddr, eaddr, begin, end, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKS18 ( S/P Kernel, subset, type 18 )
pub fn SPKS18(
    HANDLE: i32,
    BADDR: i32,
    EADDR: i32,
    BEGIN: f64,
    END: f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut DATA = StackArray::<f64, 12>::new(1..=12);
    let mut NDIR: i32 = 0;
    let mut NREC: i32 = 0;
    let mut OFFE: i32 = 0;
    let mut OFFSET: i32 = 0;
    let mut PACKSZ: i32 = 0;
    let mut REC = StackArray::<i32, 2>::new(1..=2);
    let mut SUBTYP: i32 = 0;
    let mut WINSIZ: i32 = 0;
    let mut WNSZM1: i32 = 0;

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
        CHKIN(b"SPKS18", ctx)?;
    }

    //
    // See whether there's any work to do; return immediately if not.
    //
    if (BEGIN > END) {
        CHKOUT(b"SPKS18", ctx)?;
        return Ok(());
    }

    //
    // Read the segment structure metadata.
    //
    // Get the type 18 segment subtype.  Next get the quantity "window
    // size." Also get the number of records in the segment.
    //
    DAFGDA(
        HANDLE,
        ((EADDR - MSIZE) + 1),
        EADDR,
        DATA.as_slice_mut(),
        ctx,
    )?;

    SUBTYP = intrinsics::IDNINT(DATA[1]);
    WINSIZ = intrinsics::IDNINT(DATA[2]);
    NREC = intrinsics::IDNINT(DATA[3]);

    //
    // Set the packet size based on the subtype.
    //
    if (SUBTYP == S18TP0) {
        PACKSZ = S18PS0;
    } else if (SUBTYP == S18TP1) {
        PACKSZ = S18PS1;
    } else {
        SETMSG(
            b"Unexpected SPK type 18 subtype found in type 18 record.",
            ctx,
        );
        ERRINT(b"#", SUBTYP, ctx);
        SIGERR(b"SPICE(INVALIDVALUE)", ctx)?;
        CHKOUT(b"SPKS18", ctx)?;
        return Ok(());
    }

    //
    // From the number of records, we can compute
    //
    //    NDIR      The number of directory epochs.
    //
    //    OFFE      The offset of the first epoch.
    //
    NDIR = ((NREC - 1) / 100);
    OFFE = (((EADDR - NDIR) - NREC) - MSIZE);

    //
    // Examine the epochs in forward order, looking for the first
    // epoch greater than or equal to END (or the final epoch,
    // whichever comes first). This epoch corresponds to the last
    // state to be transferred.
    //
    REC[2] = 1;
    DAFGDA(
        HANDLE,
        (OFFE + REC[2]),
        (OFFE + REC[2]),
        DATA.as_slice_mut(),
        ctx,
    )?;

    while ((REC[2] < NREC) && (DATA[1] < END)) {
        REC[2] = (REC[2] + 1);
        DAFGDA(
            HANDLE,
            (OFFE + REC[2]),
            (OFFE + REC[2]),
            DATA.as_slice_mut(),
            ctx,
        )?;
    }

    //
    // Let WNSZM1 be one less than the window size.
    //
    // Make sure that there are WNSZM1/2 additional states to the right
    // of the one having index REC(2), if possible.  If not, take as
    // many states as we can.
    //
    WNSZM1 = (WINSIZ - 1);

    REC[2] = intrinsics::MIN0(&[NREC, (REC[2] + (WNSZM1 / 2))]);

    //
    // Now examine the epochs in reverse order, looking for the first
    // epoch less than or equal to BEGIN (or the initial epoch,
    // whichever comes first). This epoch corresponds to the first
    // state to be transferred.
    //
    REC[1] = NREC;
    DAFGDA(
        HANDLE,
        (OFFE + REC[1]),
        (OFFE + REC[1]),
        DATA.as_slice_mut(),
        ctx,
    )?;

    while ((REC[1] > 1) && (DATA[1] > BEGIN)) {
        REC[1] = (REC[1] - 1);
        DAFGDA(
            HANDLE,
            (OFFE + REC[1]),
            (OFFE + REC[1]),
            DATA.as_slice_mut(),
            ctx,
        )?;
    }

    //
    // Make sure that there are WNSZM1/2 additional states to the left
    // of the one having index REC(1), if possible.  If not, take as
    // many states as we can.
    //
    REC[1] = intrinsics::MAX0(&[1, (REC[1] - (WNSZM1 / 2))]);

    //
    // Copy states REC(1) through REC(2) to the output file.
    //
    for I in REC[1]..=REC[2] {
        OFFSET = ((BADDR - 1) + ((I - 1) * PACKSZ));

        DAFGDA(
            HANDLE,
            (OFFSET + 1),
            (OFFSET + PACKSZ),
            DATA.as_slice_mut(),
            ctx,
        )?;
        DAFADA(DATA.as_slice(), PACKSZ, ctx)?;
    }

    //
    // Copy epochs REC(1) through REC(2) to the output file.
    //
    for I in REC[1]..=REC[2] {
        DAFGDA(HANDLE, (OFFE + I), (OFFE + I), DATA.as_slice_mut(), ctx)?;
        DAFADA(DATA.as_slice(), 1, ctx)?;
    }

    //
    // Put every 100'th epoch into the directory, except the last
    // epoch, if that epoch's index would be a multiple of 100.
    //
    for I in intrinsics::range((REC[1] + 99), (REC[2] - 1), 100) {
        DAFGDA(HANDLE, (OFFE + I), (OFFE + I), DATA.as_slice_mut(), ctx)?;
        DAFADA(DATA.as_slice(), 1, ctx)?;
    }

    //
    // Store subtype, the window size, and the number of
    // records to end the segment.
    //
    DAFADA(&[(SUBTYP as f64)], 1, ctx)?;
    DAFADA(&[(WINSIZ as f64)], 1, ctx)?;
    DAFADA(&[(((REC[2] - REC[1]) + 1) as f64)], 1, ctx)?;

    CHKOUT(b"SPKS18", ctx)?;
    Ok(())
}
