//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// S/P Kernel, subset, type 8
///
/// Extract a subset of the data in an SPK segment of type 8
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
///  2)  If BEGIN > END, no data is written to the target file.
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
///  It transfers a subset of a type 08 SPK data segment to
///  a properly initialized segment of a second SPK file.
///
///  The exact structure of a segment of data type 08 is described
///  in the section on type 08 in the SPK Required Reading.
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
///  J.M. Lynch         (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.1, 03-JUN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.1.0, 07-SEP-2001 (EDW)
///
///         Replaced DAFRDA call with DAFGDA.
///         Added IMPLICIT NONE.
///
/// -    SPICELIB Version 2.0.0, 20-AUG-1994 (NJB)
///
///         Bug fix: START value for output segment has been corrected.
///         Bug fix: Sufficient bracketing states are now included in the
///         output segment to ensure duplication of states given by source
///         segment.
///
/// -    SPICELIB Version 1.0.0, 08-AUG-1993 (NJB) (JML) (WLT) (IMU)
/// ```
pub fn spks08(
    ctx: &mut SpiceContext,
    handle: i32,
    baddr: i32,
    eaddr: i32,
    begin: f64,
    end: f64,
) -> crate::Result<()> {
    SPKS08(handle, baddr, eaddr, begin, end, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKS08 ( S/P Kernel, subset, type 8 )
pub fn SPKS08(
    HANDLE: i32,
    BADDR: i32,
    EADDR: i32,
    BEGIN: f64,
    END: f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut DATA = StackArray::<f64, 6>::new(1..=6);
    let mut RATIO: f64 = 0.0;
    let mut START: f64 = 0.0;
    let mut STEP: f64 = 0.0;
    let mut DEGREE: i32 = 0;
    let mut NREC: i32 = 0;
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
        CHKIN(b"SPKS08", ctx)?;
    }

    //
    // Look up the following items:
    //
    //    -- The start epoch
    //    -- The step size
    //    -- The polynomial degree
    //    -- The number of records in the segment
    //
    DAFGDA(HANDLE, (EADDR - 3), EADDR, DATA.as_slice_mut(), ctx)?;

    START = DATA[1];
    STEP = DATA[2];
    DEGREE = intrinsics::IDNINT(DATA[3]);
    NREC = intrinsics::IDNINT(DATA[4]);

    //
    // See whether there's any work to do; return immediately if not.
    //
    if (((END < BEGIN) || (END < START)) || (BEGIN > (START + (((NREC - 1) as f64) * STEP)))) {
        CHKOUT(b"SPKS08", ctx)?;
        return Ok(());
    }

    //
    // Compute the index of the state having the last epoch
    // epoch less than or equal to BEGIN (or the initial epoch,
    // whichever comes last). This epoch corresponds to the first
    // state to be transferred.
    //

    RATIO = intrinsics::DMAX1(&[0.0, ((BEGIN - START) / STEP)]);

    REC[1] = (1 + intrinsics::MIN0(&[(RATIO as i32), (NREC - 1)]));

    //
    // Make sure that there are DEGREE/2 additional states to the left
    // of the one having index REC(1), if possible.  If not, take as
    // many states as we can.
    //
    REC[1] = intrinsics::MAX0(&[1, (REC[1] - (DEGREE / 2))]);

    //
    // Make sure that REC(1) is small enough so that there are are at
    // least DEGREE+1 states in the segment.
    //
    REC[1] = intrinsics::MIN0(&[REC[1], (NREC - DEGREE)]);

    //
    // Now compute the index of the state having the first epoch greater
    // than or equal to END (or the final epoch, whichever comes first).
    // This epoch corresponds to the last state to be transferred.
    //
    RATIO = ((END - START) / STEP);

    if (RATIO == f64::trunc(RATIO)) {
        REC[2] = (1 + intrinsics::MIN0(&[(RATIO as i32), (NREC - 1)]));
    } else {
        REC[2] = (1 + intrinsics::MIN0(&[((RATIO as i32) + 1), (NREC - 1)]));
    }

    //
    // Make sure that there are DEGREE/2 additional states to the right
    // of the one having index REC(2), if possible.  If not, take as
    // many states as we can.
    //
    REC[2] = intrinsics::MIN0(&[NREC, (REC[2] + (DEGREE / 2))]);

    //
    // Make sure that REC(2) is large enough so that there are are at
    // least DEGREE+1 states in the segment.
    //
    REC[2] = intrinsics::MAX0(&[REC[2], (DEGREE + 1)]);

    //
    // Copy states REC(1) through REC(2) to the output file.
    //
    for I in REC[1]..=REC[2] {
        OFFSET = ((BADDR - 1) + ((I - 1) * 6));

        DAFGDA(HANDLE, (OFFSET + 1), (OFFSET + 6), DATA.as_slice_mut(), ctx)?;
        DAFADA(DATA.as_slice(), 6, ctx)?;
    }

    //
    // Store the start time, step size, polynomial degree and the
    // number of records to end the segment.
    //
    DAFADA(&[(START + (((REC[1] - 1) as f64) * STEP))], 1, ctx)?;
    DAFADA(&[STEP], 1, ctx)?;
    DAFADA(&[(DEGREE as f64)], 1, ctx)?;
    DAFADA(&[(((REC[2] - REC[1]) + 1) as f64)], 1, ctx)?;

    CHKOUT(b"SPKS08", ctx)?;
    Ok(())
}
