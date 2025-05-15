//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// S/P Kernel, subset, type 5
///
/// Extract a subset of the data in an SPK segment of type 5
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
/// # Files
///
/// ```text
///  Data is extracted from the file connected to the input
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
///  It transfers a subset of a type 05 SPK data segment to
///  a properly initialized segment of a second SPK file.
///
///  The exact structure of a segment of data type 05 is described
///  in the section on type 05 in the SPK Required Reading.
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
///  J. Diaz del Rio    (ODC Space)
///  J.M. Lynch         (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.1, 03-JUN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.1.0, 07-SEP-2001 (EDW)
///
///         Replaced DAFRDA call with DAFGDA.
///         Added IMPLICIT NONE.
///
/// -    SPICELIB Version 1.0.0, 01-APR-1992 (JML) (WLT) (IMU)
/// ```
pub fn spks05(
    ctx: &mut SpiceContext,
    handle: i32,
    baddr: i32,
    eaddr: i32,
    begin: f64,
    end: f64,
) -> crate::Result<()> {
    SPKS05(handle, baddr, eaddr, begin, end, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKS05 ( S/P Kernel, subset, type 5 )
pub fn SPKS05(
    HANDLE: i32,
    BADDR: i32,
    EADDR: i32,
    BEGIN: f64,
    END: f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut DATA = StackArray::<f64, 6>::new(1..=6);
    let mut GM: f64 = 0.0;
    let mut NDIR: i32 = 0;
    let mut NREC: i32 = 0;
    let mut OFFE: i32 = 0;
    let mut OFFSET: i32 = 0;
    let mut REC = StackArray::<i32, 2>::new(1..=2);

    //
    // SPICELIB functions
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
        CHKIN(b"SPKS05", ctx)?;
    }

    //
    // Get the number of records in the segment. While we're at it,
    // get the GM of the central body as well (it's adjacent to NREC)
    // since we'll need it anyway.
    //
    DAFGDA(HANDLE, (EADDR - 1), EADDR, DATA.as_slice_mut(), ctx)?;

    NREC = (DATA[2] as i32);
    GM = DATA[1];

    //
    // From the number of records, we can compute
    //
    //    NDIR      The number of directory epochs.
    //
    //    OFFE      The offset of the first epoch.
    //
    NDIR = (NREC / 100);
    OFFE = (((EADDR - NDIR) - NREC) - 2);

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
    // Now examine them in reverse order, looking for the first
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
    // Copy states REC(1) through REC(2) to the output file.
    //
    for I in REC[1]..=REC[2] {
        OFFSET = ((BADDR - 1) + ((I - 1) * 6));

        DAFGDA(HANDLE, (OFFSET + 1), (OFFSET + 6), DATA.as_slice_mut(), ctx)?;
        DAFADA(DATA.as_slice(), 6, ctx)?;
    }

    //
    // Copy epochs REC(1) through REC(2) to the output file.
    //
    for I in REC[1]..=REC[2] {
        DAFGDA(HANDLE, (OFFE + I), (OFFE + I), DATA.as_slice_mut(), ctx)?;
        DAFADA(DATA.as_slice(), 1, ctx)?;
    }

    //
    // Put every 100'th epoch into the directory.
    //
    for I in intrinsics::range((REC[1] + 99), REC[2], 100) {
        DAFGDA(HANDLE, (OFFE + I), (OFFE + I), DATA.as_slice_mut(), ctx)?;
        DAFADA(DATA.as_slice(), 1, ctx)?;
    }

    //
    // Store the GM of the central body and the number of records
    // to end the segment.
    //
    DAFADA(&[GM], 1, ctx)?;
    DAFADA(&[(((REC[2] - REC[1]) + 1) as f64)], 1, ctx)?;

    CHKOUT(b"SPKS05", ctx)?;
    Ok(())
}
