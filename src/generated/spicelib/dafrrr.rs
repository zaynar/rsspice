//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const IFNLEN: i32 = 60;
const WPR: i32 = 128;
const MAXD: i32 = (WPR - 3);
const MAXI: i32 = (MAXD * 2);
const MAXC: i32 = (MAXD * 8);

/// DAF, remove reserved records
///
/// Remove a specified number of reserved records from a Double
/// Precision Array File (DAF).
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   DAF, opened for writing.
///  RESV       I   Number of records to remove.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle associated with a DAF that has been
///           opened with write access.
///
///  RESV     is the number of reserved records to be removed
///           from the specified file.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If RESV is less than one, the file is not changed.
///
///  2)  If RESV is greater than the number of reserved records in the
///      file, all of the reserved records are removed.
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
///  Normally, the reserved records in an array file are reserved
///  when the file is created. However, it may occasionally become
///  desirable to remove reserved records---when their contents are
///  significantly reduced, for example.
///
///  The records nearest the end of the file are removed. Note
///  that the physical size of the file is not reduced when reserved
///  records are removed.
/// ```
///
/// # Examples
///
/// ```text
///  For the following call to DAFRRR, assume that HANDLE is the file
///  handle for a DAF file that has been opened for write access, and
///  that the DAF file already contains 12 reserved records (located in
///  records 2-13 of the physical file).
///
///     CALL DAFRRR ( HANDLE, 7 )
///
///  After this call to DAFRRR, the number of reserved records has been
///  decreased by 7, leaving only the first five of the original
///  reserved records, physical records 2-6.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine will only remove reserve records from DAFs open
///      for write. These files are implicitly of the native binary
///      file format.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.3.0, 02-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.2.0, 16-NOV-2001 (FST)
///
///         Added a call to DAFSIH to prevent this routine from
///         attempting to write to non-native binary file formats.
///         This will provide a more useful error diagnostic with
///         little impact on performance.
///
/// -    SPICELIB Version 1.1.0, 30-SEP-1993 (KRG)
///
///         $Detailed_Input and $Examples section of the header were
///         modified.
///
///         Added calls to the FORTRAN intrinsic functions INT and
///         DBLE in the code that updates the summary record.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 18-JUL-1990 (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.1.0, 30-SEP-1993 (KRG)
///
///         $Detailed_Input section was modified. References to any
///         specific routines by name as a method for opening a DAF file
///         for write access were removed. The assumption is that a person
///         using DAF files would already know something about opening and
///         closing the files.
///
///         $Examples section was modified. References to any specific
///         routines by name as a method for opening a DAF file for writing
///         were removed, and the example was reworded in such a way that
///         the use of the subroutine remained clear.
///
///         Added calls to the INT intrinsic function to convert a DP
///         number to an integer before assigning it to NEXT or ENDBLK,
///         both of which are integer variables. Also added calls to INT
///         in IF statements where comparisons were made between DP numbers
///         and INTEGERs, when integral values were actually being
///         compared.
///
///         Added calls to the intrinsic function DBLE to convert an
///         integer, REMOVE, into a DP number when doing some arithmetic.
/// ```
pub fn dafrrr(ctx: &mut SpiceContext, handle: i32, resv: i32) -> crate::Result<()> {
    DAFRRR(handle, resv, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFRRR ( DAF, remove reserved records )
pub fn DAFRRR(HANDLE: i32, RESV: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut CREC = [b' '; MAXC as usize];
    let mut IFNAME = [b' '; IFNLEN as usize];
    let mut DC = StackArray::<f64, 125>::new(1..=MAXD);
    let mut DREC = StackArray::<f64, 128>::new(1..=WPR);
    let mut SUM = StackArray::<f64, 125>::new(1..=MAXD);
    let mut BEGBLK: i32 = 0;
    let mut BWARD: i32 = 0;
    let mut DECR: i32 = 0;
    let mut ENDBLK: i32 = 0;
    let mut FREE: i32 = 0;
    let mut FWARD: i32 = 0;
    let mut IC = StackArray::<i32, 250>::new(1..=MAXI);
    let mut ND: i32 = 0;
    let mut NEXT: i32 = 0;
    let mut NI: i32 = 0;
    let mut RECNO: i32 = 0;
    let mut REMOVE: i32 = 0;
    let mut WORD: i32 = 0;
    let mut FOUND: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    // IFNLEN      is the length of a DAF internal file name.
    //

    //
    // WPR         is the maximum number of double precision
    //             numbers per record.  WPR stands for words
    //             per record.
    //

    //
    // MAXD,       are the maximum number of double precision
    // MAXI,       numbers, integers, and characters, respectively,
    // MAXC        not including space reserved for control information.
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
        CHKIN(b"DAFRRR", ctx)?;
    }

    //
    // Before proceeding any further, check that the DAF associated
    // with HANDLE is available for write access.
    //
    DAFSIH(HANDLE, b"WRITE", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DAFRRR", ctx)?;
        return Ok(());
    }

    //
    // Get the contents of the file record. If it fails, then just check
    // out and return, as an appropriate error message should have
    // already been set.
    //
    DAFRFR(
        HANDLE,
        &mut ND,
        &mut NI,
        &mut IFNAME,
        &mut FWARD,
        &mut BWARD,
        &mut FREE,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"DAFRRR", ctx)?;
        return Ok(());
    }

    //
    // Don't remove more than the current number of reserved records!
    // If there are none, check out.
    //
    REMOVE = intrinsics::MIN0(&[RESV, (FWARD - 2)]);

    if (REMOVE < 1) {
        CHKOUT(b"DAFRRR", ctx)?;
        return Ok(());
    }

    //
    // Okay, here's the plan. We are just going to move records
    // forward, starting with the first summary record in the file
    // and ending with the last data record.
    //
    // After everything has been moved, the initial and final
    // addresses of all the arrays have to be decremented by the
    // same amount: the number of words per record (128) times
    // the number of records removed.
    //
    DECR = (WPR * REMOVE);

    //
    // Records will be moved in `blocks', where each block contains
    //
    //    -- a summary record
    //
    //    -- a name record
    //
    //    -- one or more data records
    //
    // Most blocks lie between one summary record and the next.
    // The final block lies between the final summary record and
    // whatever data record contains the first free address.
    //
    // BEGBLK is initially the first summary record location.
    //
    BEGBLK = FWARD;

    while ((BEGBLK > 0) && !FAILED(ctx)) {
        //
        // Move the summary record first. The location of the next
        // summary record determines the end of this block, and the
        // beginning of the next.
        //
        // Be sure to adjust the forward and backward pointers;
        // otherwise, we won't be able to find the summaries again.
        //
        RECNO = BEGBLK;
        DAFRDR(HANDLE, RECNO, 1, WPR, DREC.as_slice_mut(), &mut FOUND, ctx)?;

        if ((DREC[1] as i32) > 0) {
            ENDBLK = ((DREC[1] as i32) - 1);
            NEXT = (DREC[1] as i32);
        } else {
            DAFARW(FREE, &mut ENDBLK, &mut WORD, ctx)?;
            NEXT = 0;
        }

        if ((DREC[1] as i32) > 0) {
            DREC[1] = (DREC[1] - (REMOVE as f64));
        }

        if ((DREC[2] as i32) > 0) {
            DREC[2] = (DREC[2] - (REMOVE as f64));
        }

        DAFWDR(HANDLE, (RECNO - REMOVE), DREC.as_slice(), ctx)?;

        //
        // Then the name record.
        //
        RECNO = (BEGBLK + 1);
        DAFRCR(HANDLE, RECNO, &mut CREC, ctx)?;
        DAFWCR(HANDLE, (RECNO - REMOVE), &CREC, ctx)?;

        //
        // Finally, the data records.
        //
        {
            let m1__: i32 = (BEGBLK + 2);
            let m2__: i32 = ENDBLK;
            let m3__: i32 = 1;
            RECNO = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                DAFRDR(HANDLE, RECNO, 1, WPR, DREC.as_slice_mut(), &mut FOUND, ctx)?;
                DAFWDR(HANDLE, (RECNO - REMOVE), DREC.as_slice(), ctx)?;
                RECNO += m3__;
            }
        }

        //
        // Start the next block, if one exists.
        //
        BEGBLK = NEXT;
    }

    //
    // Rewrite the file record, to reflect the new organization of
    // the file.
    //
    FWARD = (FWARD - REMOVE);
    BWARD = (BWARD - REMOVE);
    FREE = (FREE - DECR);

    DAFWFR(HANDLE, ND, NI, &IFNAME, FWARD, BWARD, FREE, ctx)?;

    //
    // Get the summary for each array, decrement the addresses (stored
    // in the final two integer components), and replace the summary.
    //
    DAFBFS(HANDLE, ctx)?;
    DAFFNA(&mut FOUND, ctx)?;

    while (FOUND && !FAILED(ctx)) {
        DAFGS(SUM.as_slice_mut(), ctx)?;
        DAFUS(SUM.as_slice(), ND, NI, DC.as_slice_mut(), IC.as_slice_mut());

        IC[(NI - 1)] = (IC[(NI - 1)] - DECR);
        IC[NI] = (IC[NI] - DECR);

        DAFPS(ND, NI, DC.as_slice(), IC.as_slice(), SUM.as_slice_mut());
        DAFWS(SUM.as_slice(), ctx)?;

        DAFFNA(&mut FOUND, ctx)?;
    }

    CHKOUT(b"DAFRRR", ctx)?;
    Ok(())
}
