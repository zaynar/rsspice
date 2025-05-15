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

/// DAF, add reserved records
///
/// Add a specified number of reserved records to a Double Precision
/// Array File (DAF).
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
///  HANDLE     I   Handle of a DAF file opened for writing.
///  RESV       I   Number of records to reserve.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle associated with a DAF file that has
///           been opened with write access.
///
///  RESV     is the number of reserved records to be added
///           to the specified file.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If RESV is less than one, the file is not changed.
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
///  necessary to add reserved records---when the contents of one
///  file are appended to another, for example. (In this case, any
///  information in the reserved records of either file should
///  be included in the resulting file.)
///
///  The new reserved records are appended to the old ones. The new
///  reserved records are also NULL filled.
/// ```
///
/// # Examples
///
/// ```text
///  In the following call to DAFARR, assume that HANDLE is the file
///  handle for a DAF file that has been opened for write access, and
///  that the DAF file already contains 12 reserved records (located in
///  records 2-13 of the physical file).
///
///     CALL DAFARR ( HANDLE, 7 )
///
///  After this call, the DAF file attached to HANDLE will contain 19
///  reserved records. The new reserved records are located in
///  records 14-20 of the physical file.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine will only add reserved records to DAFs open for
///      write. These files are implicitly of the native binary file
///      format.
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
/// -    SPICELIB Version 1.6.0, 13-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.5.0, 16-NOV-2001 (FST)
///
///         Added a call to DAFSIH to prevent this routine from
///         attempting to write to non-native binary file formats.
///         This will provide a more useful error diagnostic with
///         little impact on performance.
///
/// -    SPICELIB Version 1.4.0, 08-MAR-1996 (KRG)
///
///         Added code to write NULL filled records to the file for the
///         new reserved records.
///
/// -    SPICELIB Version 1.3.0, 12-MAY-1994 (KRG)
///
///         Added a missing call to CHKOUT before the RETURN statement in
///         the test
///
///               IF ( RESV .LT. 1 ) THEN
///                  RETURN
///               END IF
///
/// -    SPICELIB Version 1.2.0, 30-SEP-1993 (KRG)
///
///         $Detailed_Input and $Examples section of the header were
///         modified.
///
///         Added calls to the FORTRAN intrinsic functions INT and
///         DBLE in the code that updates the summary record.
///
///         Modified an IF loop to make logic clearer.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 17-JUL-1990 (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.4.0, 08-MAR-1996 (KRG)
///
///         Added code to write NULL filled records to the file for the
///         new reserved records.
///
/// -    SPICELIB Version 1.3.0, 12-MAY-1994 (KRG)
///
///         Added a missing call to CHKOUT before the RETURN statement in
///         the test
///
///               IF ( RESV .LT. 1 ) THEN
///                  RETURN
///               END IF
///
/// -    SPICELIB Version 1.2.0, 30-SEP-1993 (KRG)
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
///         number to an integer before assigning it to NEXT, which is an
///         integer variable. Also added calls to INT in IF statements
///         where comparisons were made between DP numbers and INTEGERs,
///         when integral values were actually being compared.
///
///         Added calls to the intrinsic function DBLE to convert an
///         integer, RESV, into a DP number when doing some arithmetic.
///
///         Took an ELSE IF clause out of the initial IF return  ELSE
///         check in END IF at the beginning of the routine. Replaced the
///         code:
///
///               IF ( RETURN () ) THEN
///                  RETURN
///
///               ELSE IF ( RESV .LT. 1 ) THEN
///                  RETURN
///
///               ELSE
///                  CALL CHKIN ( 'DAFARR' )
///               END IF
///
///         with the equivalent code:
///
///               IF ( RETURN () ) THEN
///                  RETURN
///               ELSE
///                  CALL CHKIN ( 'DAFARR' )
///               END IF
///
///         C
///         C     Check to see if the number of records to be reserved is
///         C     less than one. If so, just return without changing
///         C     anything.
///         C
///               IF ( RESV .LT. 1 ) THEN
///                  RETURN
///               END IF
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 17-JUL-1990 (IMU)
/// ```
pub fn dafarr(ctx: &mut SpiceContext, handle: i32, resv: i32) -> crate::Result<()> {
    DAFARR(handle, resv, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFARR ( DAF, add reserved records )
pub fn DAFARR(HANDLE: i32, RESV: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut CREC = [b' '; MAXC as usize];
    let mut IFNAME = [b' '; IFNLEN as usize];
    let mut DC = StackArray::<f64, 125>::new(1..=MAXD);
    let mut DREC = StackArray::<f64, 128>::new(1..=WPR);
    let mut SUM = StackArray::<f64, 125>::new(1..=MAXD);
    let mut BEGBLK: i32 = 0;
    let mut BWARD: i32 = 0;
    let mut ENDBLK: i32 = 0;
    let mut FREE: i32 = 0;
    let mut FWARD: i32 = 0;
    let mut I: i32 = 0;
    let mut INCR: i32 = 0;
    let mut IC = StackArray::<i32, 250>::new(1..=MAXI);
    let mut ND: i32 = 0;
    let mut NEXT: i32 = 0;
    let mut NI: i32 = 0;
    let mut RECNO: i32 = 0;
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
    // WPR         is the maximum number of double precision numbers
    //             (words) per record.
    //
    // MAXD,       are the maximum number of double precision
    // MAXI,       numbers, integers, and characters, respectively,
    // MAXC        per record, not including space reserved for
    //             control information (3 dp numbers are reserved).
    //             There are two integers per double precision word,
    //             and eight characters per word.
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
        CHKIN(b"DAFARR", ctx)?;
    }
    //
    //
    // Check to see if the number of records to be reserved is less than
    // one. If so, just return without changing anything.
    //
    if (RESV < 1) {
        CHKOUT(b"DAFARR", ctx)?;
        return Ok(());
    }

    //
    // Before proceeding any further, check that the DAF associated
    // with HANDLE is available for write access.
    //
    DAFSIH(HANDLE, b"WRITE", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DAFARR", ctx)?;
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
        CHKOUT(b"DAFARR", ctx)?;
        return Ok(());
    }

    //
    // Okay, here's the plan. We are just going to move records
    // in the direction of the end of the file, starting
    // with the last record in the file and ending with the first
    // summary record.
    //
    // After everything has been moved, the initial and final
    // addresses of all the arrays have to be incremented by the
    // same amount: the number of words per record (128) times
    // the number of new records.
    //
    INCR = (WPR * RESV);

    //
    // Before we do that, however, we should write some bogus records
    // to the end of the file, to make sure we don't run out of space
    // later on. If this doesn't work, we will leave the logical
    // contents of the file uncorrupted (although it may get larger).
    //
    DAFARW(FREE, &mut RECNO, &mut WORD, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = RESV;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            DAFWDR(HANDLE, (RECNO + I), DREC.as_slice(), ctx)?;

            I += m3__;
        }
    }

    if FAILED(ctx) {
        CHKOUT(b"DAFARR", ctx)?;
        return Ok(());
    }

    //
    // Records will be moved in `blocks', where each block contains
    //
    //    -- a summary record
    //
    //    -- a name record
    //
    //    -- one or more data records
    //
    // The first block to be moved (that is, the last block in
    // the file) lies between the final summary record (BWARD) and
    // whatever record contains the first free address in the file.
    //
    BEGBLK = BWARD;
    DAFARW(FREE, &mut ENDBLK, &mut WORD, ctx)?;

    while ((BEGBLK > 0) && !FAILED(ctx)) {
        //
        // Move the data records first.
        //
        {
            let m1__: i32 = ENDBLK;
            let m2__: i32 = (BEGBLK + 2);
            let m3__: i32 = -1;
            RECNO = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                DAFRDR(HANDLE, RECNO, 1, WPR, DREC.as_slice_mut(), &mut FOUND, ctx)?;
                DAFWDR(HANDLE, (RECNO + RESV), DREC.as_slice(), ctx)?;

                RECNO += m3__;
            }
        }

        //
        // Then the name record.
        //
        RECNO = (BEGBLK + 1);
        DAFRCR(HANDLE, RECNO, &mut CREC, ctx)?;
        DAFWCR(HANDLE, (RECNO + RESV), &CREC, ctx)?;

        //
        // Finally, the summary record.
        //
        // To find the beginning of the next block, look at the backward
        // pointer from the summary record of the current block.
        //
        // Be sure to adjust the forward and backward pointers;
        // otherwise, we won't be able to find the summaries again.
        //
        RECNO = BEGBLK;
        DAFRDR(HANDLE, RECNO, 1, WPR, DREC.as_slice_mut(), &mut FOUND, ctx)?;

        NEXT = (DREC[2] as i32);

        if ((DREC[1] as i32) > 0) {
            DREC[1] = (DREC[1] + (RESV as f64));
        }

        if ((DREC[2] as i32) > 0) {
            DREC[2] = (DREC[2] + (RESV as f64));
        }

        DAFWDR(HANDLE, (RECNO + RESV), DREC.as_slice(), ctx)?;

        //
        // The next block ends just before the current block begins.
        //
        ENDBLK = (BEGBLK - 1);
        BEGBLK = NEXT;
    }

    //
    // Rewrite the file record, to reflect the new organization of
    // the file.
    //
    FWARD = (FWARD + RESV);
    BWARD = (BWARD + RESV);
    FREE = (FREE + INCR);

    DAFWFR(HANDLE, ND, NI, &IFNAME, FWARD, BWARD, FREE, ctx)?;

    //
    // Get the summary for each array, increment the addresses (stored
    // in the final two integer components), and replace the summary.
    //
    DAFBFS(HANDLE, ctx)?;
    DAFFNA(&mut FOUND, ctx)?;

    while (FOUND && !FAILED(ctx)) {
        DAFGS(SUM.as_slice_mut(), ctx)?;
        DAFUS(SUM.as_slice(), ND, NI, DC.as_slice_mut(), IC.as_slice_mut());

        IC[(NI - 1)] = (IC[(NI - 1)] + INCR);
        IC[NI] = (IC[NI] + INCR);

        DAFPS(ND, NI, DC.as_slice(), IC.as_slice(), SUM.as_slice_mut());
        DAFWS(SUM.as_slice(), ctx)?;

        DAFFNA(&mut FOUND, ctx)?;
    }
    //
    // Write NULL filled records to the reserved record area.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXC;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(fstr::substr_mut(&mut CREC, I..=I), &intrinsics::CHAR(0));
            I += m3__;
        }
    }

    I = (FWARD - RESV);
    {
        let m1__: i32 = I;
        let m2__: i32 = ((I + RESV) - 1);
        let m3__: i32 = 1;
        RECNO = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            DAFWCR(HANDLE, RECNO, &CREC, ctx)?;
            RECNO += m3__;
        }
    }

    CHKOUT(b"DAFARR", ctx)?;
    Ok(())
}
