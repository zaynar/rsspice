//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const NITEMS: i32 = 16;

/// S/P Kernel, subset, type 15
///
/// Extract a subset of the data in an SPK segment of type 15
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
///  routine SPKSUB. It transfers a subset of a type 15 SPK data
///  segment to a properly initialized segment of a second SPK file.
///
///  The exact structure of a segment of data type 15 is described
///  in the section on type 15 in the SPK Required Reading.
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
///  W.L. Taber         (JPL)
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
/// -    SPICELIB Version 1.0.0, 07-NOV-1994 (WLT)
/// ```
pub fn spks15(
    ctx: &mut SpiceContext,
    handle: i32,
    baddr: i32,
    eaddr: i32,
    begin: f64,
    end: f64,
) -> crate::Result<()> {
    SPKS15(handle, baddr, eaddr, begin, end, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKS15 ( S/P Kernel, subset, type 15 )
pub fn SPKS15(
    HANDLE: i32,
    BADDR: i32,
    EADDR: i32,
    BEGIN: f64,
    END: f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut DATA = StackArray::<f64, 16>::new(1..=NITEMS);

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
        CHKIN(b"SPKS15", ctx)?;
    }

    //
    // See whether there's any work to do; return immediately if not.
    //
    if (BEGIN > END) {
        CHKOUT(b"SPKS15", ctx)?;
        return Ok(());
    }

    //
    // This couldn't be much easier.  First copy the entire
    // type 15 segment out of the file.
    //
    DAFGDA(HANDLE, BADDR, EADDR, DATA.as_slice_mut(), ctx)?;

    //
    // Now write the data into the output file.
    //
    DAFADA(DATA.as_slice(), NITEMS, ctx)?;

    CHKOUT(b"SPKS15", ctx)?;
    Ok(())
}
