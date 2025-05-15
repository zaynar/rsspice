//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// S/P Kernel, subset, type 3
///
/// Extract a subset of the data in a SPK segment of type 3 (Chebyshev
/// polynomials, position and velocity) into a new segment.
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
///  The exact structure of a segment of data type 3 (Chebyshev
///  polynomials, position and velocity) is detailed in the SPK
///  Required Reading file.
///
///  On not so close inspection, it will be noted that SPKS03 is
///  identical to SPKS02.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  R.E. Thurman       (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.2, 14-APR-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Moved SPK
///         required reading from $Literature_References to
///         $Required_Reading section.
///
/// -    SPICELIB Version 1.1.1, 31-DEC-2013 (NJB)
///
///         Enhanced header documentation.
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
///         SPK03 was removed from the $Required_Reading section of the
///         header. The information in the SPK03 Required Reading file
///         is now part of the SPK Required Reading file.
///
/// -    SPICELIB Version 1.0.1, 22-MAR-1990 (HAN)
///
///         Literature references added to the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (RET)
/// ```
pub fn spks03(
    ctx: &mut SpiceContext,
    handle: i32,
    baddr: i32,
    eaddr: i32,
    begin: f64,
    end: f64,
) -> crate::Result<()> {
    SPKS03(handle, baddr, eaddr, begin, end, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKS03 ( S/P Kernel, subset, type 3 )
pub fn SPKS03(
    HANDLE: i32,
    BADDR: i32,
    EADDR: i32,
    BEGIN: f64,
    END: f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut DATA = StackArray::<f64, 50>::new(1..=50);
    let mut INIT: f64 = 0.0;
    let mut INTLEN: f64 = 0.0;
    let mut RECSIZ: i32 = 0;
    let mut NREC: i32 = 0;
    let mut FIRST: i32 = 0;
    let mut LAST: i32 = 0;
    let mut REMAIN: i32 = 0;
    let mut ADDR: i32 = 0;
    let mut MOVE: i32 = 0;

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
        CHKIN(b"SPKS03", ctx)?;
    }

    //
    // The segment is made up of a number of logical records, each
    // having the same size, and covering the same length of time.
    //
    // We can determine which records to extract by comparing the input
    // epochs with the initial time of the segment and the length of the
    // interval covered by each record.  These final two constants are
    // located at the end of the segment, along with the size of each
    // logical record and the total number of records.
    //
    DAFGDA(HANDLE, (EADDR - 3), EADDR, DATA.as_slice_mut(), ctx)?;

    INIT = DATA[1];
    INTLEN = DATA[2];
    RECSIZ = (DATA[3] as i32);
    NREC = (DATA[4] as i32);

    FIRST = ((((BEGIN - INIT) / INTLEN) as i32) + 1);
    FIRST = intrinsics::MIN0(&[FIRST, NREC]);

    LAST = ((((END - INIT) / INTLEN) as i32) + 1);
    LAST = intrinsics::MIN0(&[LAST, NREC]);

    //
    // The number of records to be moved.
    //
    NREC = ((LAST - FIRST) + 1);

    //
    // We're going to move the data in chunks of 50 d.p. words.  Compute
    // the number of words left to move, the address of the beginning
    // of the records to move, and the number to move this time.
    //
    REMAIN = (NREC * RECSIZ);
    ADDR = (BADDR + ((FIRST - 1) * RECSIZ));
    MOVE = intrinsics::MIN0(&[50, REMAIN]);

    while (REMAIN > 0) {
        DAFGDA(HANDLE, ADDR, ((ADDR + MOVE) - 1), DATA.as_slice_mut(), ctx)?;
        DAFADA(DATA.as_slice(), MOVE, ctx)?;
        REMAIN = (REMAIN - MOVE);
        ADDR = (ADDR + MOVE);
        MOVE = intrinsics::MIN0(&[50, REMAIN]);
    }

    //
    // That's all the records we have to move. But there are still four
    // final numbers left to write:
    //
    //    1)  The initial time for the polynomials (INIT).
    //    2)  The time interval length for each polynomial (INTLEN).
    //    3)  The record size (RECSIZ).
    //    4)  The number of records (NREC).
    //
    // INIT and NREC will probably be different for the new segment (in
    // fact, NREC has already been changed), the other two will not.
    //
    INIT = (INIT + (((FIRST - 1) as f64) * INTLEN));

    DATA[1] = INIT;
    DATA[2] = INTLEN;
    DATA[3] = (RECSIZ as f64);
    DATA[4] = (NREC as f64);
    DAFADA(DATA.as_slice(), 4, ctx)?;

    CHKOUT(b"SPKS03", ctx)?;
    Ok(())
}
