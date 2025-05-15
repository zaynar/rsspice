//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// S/P Kernel, subset, type 13
///
/// Extract a subset of the data in an SPK segment of type 13
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
///           The output segment will be padded to the left of
///           BEGIN and the right of END with sufficient states to
///           ensure that the segment yields an ephemeris identical
///           to that given by the source segment.
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
///  It transfers a subset of a type 13 SPK data segment to
///  a properly initialized segment of a second SPK file.
///
///  The exact structure of a segment of data type 13 is described
///  in the section on type 13 in the SPK Required Reading.
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
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 03-JUN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 25-FEB-2000 (NJB)
/// ```
pub fn spks13(
    ctx: &mut SpiceContext,
    handle: i32,
    baddr: i32,
    eaddr: i32,
    begin: f64,
    end: f64,
) -> crate::Result<()> {
    SPKS13(handle, baddr, eaddr, begin, end, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKS13 ( S/P Kernel, subset, type 13 )
pub fn SPKS13(
    HANDLE: i32,
    BADDR: i32,
    EADDR: i32,
    BEGIN: f64,
    END: f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"SPKS13", ctx)?;
    }

    //
    // The type 9 subsetter knows how to do this job.
    //
    SPKS09(HANDLE, BADDR, EADDR, BEGIN, END, ctx)?;

    CHKOUT(b"SPKS13", ctx)?;
    Ok(())
}
