//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const MAXTRM: i32 = 25;
const DIRSIZ: i32 = 100;
const MAXDSZ: i32 = ((4 * MAXTRM) + 11);

/// S/P Kernel, subset, type 21
///
/// Extract a subset of the data in a SPK segment of type 21
/// into a new segment.
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
/// * [SPK](crate::required_reading::spk)
/// * [TIME](crate::required_reading::time)
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
///           of the subset to be extracted.
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
///  The exact structure of a segment of data type 21 is detailed in
///  the SPK Required Reading file.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  F.T. Krogh         (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 14-OCT-2021 (JDR) (BVS)
///
///         Bug fix: fixed routine name in CHKIN/CHKOUT calls (SPKS01
///         -> SPKS21).
///
///         Edited the header to comply with NAIF standard. Moved SPK
///         required reading from $Literature_References to
///         $Required_Reading section.
///
/// -    SPICELIB Version 1.0.0, 16-JAN-2014 (NJB) (FTK) (WLT) (IMU)
/// ```
pub fn spks21(
    ctx: &mut SpiceContext,
    handle: i32,
    baddr: i32,
    eaddr: i32,
    begin: f64,
    end: f64,
) -> crate::Result<()> {
    SPKS21(handle, baddr, eaddr, begin, end, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKS21 ( S/P Kernel, subset, type 21 )
pub fn SPKS21(
    HANDLE: i32,
    BADDR: i32,
    EADDR: i32,
    BEGIN: f64,
    END: f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut DATA = StackArray::<f64, 111>::new(1..=MAXDSZ);
    let mut DLSIZE: i32 = 0;
    let mut MAXDIM: i32 = 0;
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
    }

    CHKIN(b"SPKS21", ctx)?;

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
    DAFGDA(HANDLE, (EADDR - 1), EADDR, DATA.as_slice_mut(), ctx)?;

    MAXDIM = intrinsics::IDNINT(DATA[1]);
    NREC = intrinsics::IDNINT(DATA[2]);

    NDIR = (NREC / DIRSIZ);
    OFFE = (((EADDR - NDIR) - NREC) - 2);

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
    DLSIZE = ((4 * MAXDIM) + 11);

    OFFSET = ((BADDR - 1) + ((FIRST - 1) * DLSIZE));

    for I in FIRST..=LAST {
        DAFGDA(
            HANDLE,
            (OFFSET + 1),
            (OFFSET + DLSIZE),
            DATA.as_slice_mut(),
            ctx,
        )?;
        DAFADA(DATA.as_slice(), DLSIZE, ctx)?;

        OFFSET = (OFFSET + DLSIZE);
    }

    //
    // Third pass. Move the epochs.
    //
    for I in FIRST..=LAST {
        DAFGDA(HANDLE, (OFFE + I), (OFFE + I), DATA.as_slice_mut(), ctx)?;
        DAFADA(DATA.as_slice(), 1, ctx)?;
    }

    //
    // Get every DIRSIZ'th epoch for the directory.
    //
    for I in intrinsics::range(((FIRST + DIRSIZ) - 1), LAST, DIRSIZ) {
        DAFGDA(HANDLE, (OFFE + I), (OFFE + I), DATA.as_slice_mut(), ctx)?;
        DAFADA(DATA.as_slice(), 1, ctx)?;
    }

    //
    // Add the maximum difference line dimension and the
    // number of records, and we're done.
    //
    DAFADA(&[(MAXDIM as f64)], 1, ctx)?;

    DATA[1] = (((LAST - FIRST) + 1) as f64);

    DAFADA(DATA.as_slice(), 1, ctx)?;

    CHKOUT(b"SPKS21", ctx)?;
    Ok(())
}
