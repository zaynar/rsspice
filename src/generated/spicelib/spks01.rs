//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// S/P Kernel, subset, type 1
///
/// Extract a subset of the data in a SPK segment of type 1
/// into a new segment.
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
///  HANDLE     I   Handle of source segment.
///  BADDR      I   Beginning address of source segment.
///  EADDR      I   Ending address of source segment.
///  BEGIN      I   Beginning (initial epoch) of subset.
///  END        I   End (final epoch) of subset.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE,
///  BADDR,
///  EADDR    are the file handle assigned to a SPK file, and the
///           beginning and ending addresses of a segment within
///           the file. Together they determine a complete set of
///           ephemeris data, from which a subset is to be
///           extracted.
///
///  BEGIN,
///  END      are the initial and final epochs (ephemeris time)
///           of the subset.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If an error occurs while reading data from the source SPK
///      file, the error is signaled by a routine in the call tree of
///      this routine.
///
///  2)  If an error occurs while writing data to the output SPK file,
///      the error is signaled by a routine in the call tree of this
///      routine.
/// ```
///
/// # Files
///
/// ```text
///  See argument HANDLE.
/// ```
///
/// # Particulars
///
/// ```text
///  The exact structure of a segment of data type 1 is detailed in
///  the SPK Required Reading file.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.1, 14-APR-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Completed
///         $Exceptions section. Moved SPK required reading from
///         $Literature_References to $Required_Reading section.
///
/// -    SPICELIB Version 1.1.0, 07-SEP-2001 (EDW)
///
///         Replaced DAFRDA call with DAFGDA.
///         Added IMPLICIT NONE.
///
/// -    SPICELIB Version 1.0.3, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.2, 23-AUG-1991 (HAN)
///
///         SPK01 was removed from the $Required_Reading section of the
///         header. The information in the SPK01 Required Reading file
///         is now part of the SPK Required Reading file.
///
/// -    SPICELIB Version 1.0.1, 22-MAR-1990 (HAN)
///
///         Literature references added to the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn spks01(
    ctx: &mut SpiceContext,
    handle: i32,
    baddr: i32,
    eaddr: i32,
    begin: f64,
    end: f64,
) -> crate::Result<()> {
    SPKS01(handle, baddr, eaddr, begin, end, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKS01 ( S/P Kernel, subset, type 1 )
pub fn SPKS01(
    HANDLE: i32,
    BADDR: i32,
    EADDR: i32,
    BEGIN: f64,
    END: f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut DATA = StackArray::<f64, 71>::new(1..=71);
    let mut NREC: i32 = 0;
    let mut NDIR: i32 = 0;
    let mut OFFE: i32 = 0;
    let mut FIRST: i32 = 0;
    let mut LAST: i32 = 0;
    let mut OFFSET: i32 = 0;

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
        CHKIN(b"SPKS01", ctx)?;
    }

    //
    // Get the number of records in the segment. From that, we can
    // compute
    //
    //    NDIR      The number of directory epochs.
    //
    //    OFFE      The offset of the first epoch.
    //
    //
    // the number of directory epochs.
    //
    DAFGDA(HANDLE, EADDR, EADDR, DATA.as_slice_mut(), ctx)?;
    NREC = (DATA[1] as i32);

    NDIR = (NREC / 100);
    OFFE = (((EADDR - NDIR) - NREC) - 1);

    //
    // Well, the new segment has already been begun. We just have to
    // decide what to move, and move it (using DAFADA).
    //
    // Let's agree right now that speed is not of the greatest
    // importance here. We can probably do this with two passes
    // through the record epochs, and one pass through the records.
    //
    //    1) Determine the first and last records to be included
    //       in the subset.
    //
    //    2) Move the records.
    //
    //    3) Write the epochs.
    //
    // We can leap through the epochs one last time to get the
    // directory epochs.
    //

    //
    // First pass: which records are to be moved?
    //
    FIRST = 0;
    LAST = 0;

    for I in 1..=NREC {
        DAFGDA(HANDLE, (OFFE + I), (OFFE + I), DATA.as_slice_mut(), ctx)?;

        if ((FIRST == 0) && (DATA[1] >= BEGIN)) {
            FIRST = I;
        }

        if (((FIRST != 0) && (LAST == 0)) && (DATA[1] >= END)) {
            LAST = I;
        }
    }

    //
    // Second pass. Move the records.
    //
    OFFSET = ((BADDR - 1) + ((FIRST - 1) * 71));

    for I in FIRST..=LAST {
        DAFGDA(
            HANDLE,
            (OFFSET + 1),
            (OFFSET + 71),
            DATA.as_slice_mut(),
            ctx,
        )?;
        DAFADA(DATA.as_slice(), 71, ctx)?;

        OFFSET = (OFFSET + 71);
    }

    //
    // Third pass. Move the epochs.
    //
    for I in FIRST..=LAST {
        DAFGDA(HANDLE, (OFFE + I), (OFFE + I), DATA.as_slice_mut(), ctx)?;
        DAFADA(DATA.as_slice(), 1, ctx)?;
    }

    //
    // Get every 100'th epoch for the directory.
    //
    for I in intrinsics::range((FIRST + 99), LAST, 100) {
        DAFGDA(HANDLE, (OFFE + I), (OFFE + I), DATA.as_slice_mut(), ctx)?;
        DAFADA(DATA.as_slice(), 1, ctx)?;
    }

    //
    // Add the number of records, and we're done.
    //
    DATA[1] = (((LAST - FIRST) + 1) as f64);
    DAFADA(DATA.as_slice(), 1, ctx)?;

    CHKOUT(b"SPKS01", ctx)?;
    Ok(())
}
