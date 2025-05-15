//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const BUFSIZ: i32 = 100;

/// S/P Kernel, subset, type 20
///
/// Extract a subset of the data in a SPK segment of type 20
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
///           of the subset to be extracted.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None. This routine writes data to the SPK file currently
///  open for write access.
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
///  The exact structure of a segment of data type 20 is detailed in
///  the SPK Required Reading file.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  R.E. Thurman       (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 14-APR-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Moved SPK
///         required reading from $Literature_References to
///         $Required_Reading section.
///
/// -    SPICELIB Version 1.0.0, 23-DEC-2013 (NJB) (RET)
/// ```
pub fn spks20(
    ctx: &mut SpiceContext,
    handle: i32,
    baddr: i32,
    eaddr: i32,
    begin: f64,
    end: f64,
) -> crate::Result<()> {
    SPKS20(handle, baddr, eaddr, begin, end, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKS20 ( S/P Kernel, subset, type 20 )
pub fn SPKS20(
    HANDLE: i32,
    BADDR: i32,
    EADDR: i32,
    BEGIN: f64,
    END: f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut BTIME: f64 = 0.0;
    let mut DATA = StackArray::<f64, 100>::new(1..=BUFSIZ);
    let mut DSCALE: f64 = 0.0;
    let mut TSCALE: f64 = 0.0;
    let mut INITJD: f64 = 0.0;
    let mut INITFR: f64 = 0.0;
    let mut INTLEN: f64 = 0.0;
    let mut INTRVL: f64 = 0.0;
    let mut SUBBEG: f64 = 0.0;
    let mut SUBIFR: f64 = 0.0;
    let mut SUBIJD: f64 = 0.0;
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

    CHKIN(b"SPKS20", ctx)?;

    //
    // The segment is made up of a number of logical records, each
    // having the same size, and covering the same length of time.
    //
    // We can determine which record to return using the input epoch,
    // the integer and fractional parts of the initial time of the first
    // record's coverage interval, and the length of the interval
    // covered by each record. These constants are located at the end of
    // the segment, along with the size of each logical record and the
    // total number of records.
    //
    // For convenience, we'll fetch the segment's distance and time
    // scales in the same call.
    //
    DAFGDA(HANDLE, (EADDR - 6), EADDR, DATA.as_slice_mut(), ctx)?;

    DSCALE = DATA[1];
    TSCALE = DATA[2];
    INITJD = DATA[3];
    INITFR = DATA[4];
    INTLEN = DATA[5];
    RECSIZ = (DATA[6] as i32);
    NREC = (DATA[7] as i32);

    BTIME = (((INITJD - J2000()) + INITFR) * SPD());
    INTRVL = (INTLEN * SPD());

    FIRST = ((((BEGIN - BTIME) / INTRVL) as i32) + 1);
    FIRST = intrinsics::MAX0(&[1, intrinsics::MIN0(&[FIRST, NREC])]);

    LAST = ((((END - BTIME) / INTRVL) as i32) + 1);
    LAST = intrinsics::MAX0(&[1, intrinsics::MIN0(&[LAST, NREC])]);

    //
    // The number of records to be moved.
    //
    NREC = ((LAST - FIRST) + 1);

    //
    // We're going to move the data in chunks of BUFSIZ d.p. words.
    // Compute the number of words left to move, the address of the
    // beginning of the records to move, and the number to move this
    // time.
    //
    REMAIN = (NREC * RECSIZ);
    ADDR = (BADDR + ((FIRST - 1) * RECSIZ));
    MOVE = intrinsics::MIN0(&[BUFSIZ, REMAIN]);

    while (REMAIN > 0) {
        DAFGDA(HANDLE, ADDR, ((ADDR + MOVE) - 1), DATA.as_slice_mut(), ctx)?;
        DAFADA(DATA.as_slice(), MOVE, ctx)?;
        REMAIN = (REMAIN - MOVE);
        ADDR = (ADDR + MOVE);
        MOVE = intrinsics::MIN0(&[BUFSIZ, REMAIN]);
    }

    //
    // That's all the records we have to move. But there are still seven
    // final numbers left to write:
    //
    //    1)  The distance scale (DSCALE).
    //    2)  The time scale (TSCALE).
    //    3)  The initial integer Julian date of the start time of the
    //        first record.
    //    4)  The fractional part of the state time of the first
    //        record.
    //    5)  The time interval length for each polynomial in days
    //        (INTLEN).
    //    6)  The record size (RECSIZ).
    //    7)  The number of records (NREC).
    //
    //
    //
    // Let SUBBEG be the subset begin time expressed as a TDB Julian
    // date.
    //
    SUBBEG = (J2000() + ((BTIME + (((FIRST - 1) as f64) * INTRVL)) / SPD()));

    SUBIJD = f64::trunc(SUBBEG);
    SUBIFR = (SUBBEG - SUBIJD);

    DATA[1] = DSCALE;
    DATA[2] = TSCALE;
    DATA[3] = SUBIJD;
    DATA[4] = SUBIFR;
    DATA[5] = INTLEN;
    DATA[6] = (RECSIZ as f64);
    DATA[7] = (NREC as f64);

    DAFADA(DATA.as_slice(), 7, ctx)?;

    CHKOUT(b"SPKS20", ctx)?;
    Ok(())
}
