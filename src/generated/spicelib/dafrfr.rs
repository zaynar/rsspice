//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const IDWLEN: i32 = 8;
const IFNLEN: i32 = 60;

/// DAF, read file record
///
/// Read the contents of the file record of a DAF.
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
///  HANDLE     I   Handle of an open DAF file.
///  ND         O   Number of double precision components in summaries.
///  NI         O   Number of integer components in summaries.
///  IFNAME     O   Internal file name.
///  FWARD      O   Forward list pointer.
///  BWARD      O   Backward list pointer.
///  FREE       O   Free address pointer.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle assigned to a DAF file opened for
///           reading.
/// ```
///
/// # Detailed Output
///
/// ```text
///  ND,
///  NI       are the numbers of double precision and integer
///           components, respectively, in each array summary in
///           the specified file.
///
///  IFNAME   is the internal file name stored in the first
///           (or file) record of the specified file.
///
///  FWARD    is the forward list pointer. This points to the
///           first summary record in the file. (Records between
///           the first record and the first summary record are
///           reserved when the file is created, and are invisible
///           to DAF routines.)
///
///  BWARD    is the backward list pointer. This points
///           to the final summary record in the file.
///
///  FREE     is the free address pointer. This contains the
///           first free address in the file. (That is, the
///           initial address of the next array to be added
///           to the file.)
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the handle passed to this routine is not the handle of an
///      open DAF file, an error is signaled by a routine in the call
///      tree of this routine.
///
///  2)  If the specified DAF file is not open for read access, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  3)  If the specified record cannot (for some reason) be read,
///      the error SPICE(DAFFRNOTFOUND) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  The file record of a DAF is the only record that contains
///  any global information about the file. This record is created
///  when the file is created, and is updated only when new arrays
///  are added.
///
///  Like character records, file records are not buffered.
/// ```
///
/// # Examples
///
/// ```text
///  In the following example, the value of the forward list
///  pointer is examined in order to determine the number of
///  reserved records in the DAF. These records are then read
///  and the contents printed to the screen.
///
///     CALL DAFRFR ( HANDLE, ND, NI, IFNAME, FWARD, BWARD, FREE )
///     CALL DAFHLU ( HANDLE, UNIT )
///
///     DO I = 2, FWARD - 1
///        READ  (UNIT,REC=I) PRIVATE(1:1000)
///        WRITE (*,*)        PRIVATE(1:1000)
///     END DO
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.2.0, 02-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 3.1.0, 30-DEC-2009 (EDW)
///
///         Expanded DAFFRNOTFOUND error message to identify the file
///         handle corresponding to the error condition.
///
///         Reordered header sections to conform to SPICE format.
///         Merged the $Revisions sections, now deleted, with $Version.
///
/// -    SPICELIB Version 3.0.0, 16-NOV-2001 (FST)
///
///         Updated this routine to utilize interfaces built on
///         the new handle manager to perform I/O operations.
///
///         This routine now utilizes ZZDAFGFR to retrieve information
///         from the file record. As this private interface takes a
///         handle and performs the necessary logical unit to handle
///         mapping, the call to DAFHLU was removed. The DAFSIH call
///         remains, since this insures that HANDLE is known to DAFAH.
///
/// -    SPICELIB Version 2.0.0, 04-OCT-1993 (KRG)
///
///         The error SPICE(DAFNOIDWORD) is no longer signaled by this
///         routine. The reason for this is that if DAFSIH returns OK then
///         the handle passed to this routine is indeed a valid DAF file
///         handle, otherwise the error is diagnosed by DAFSIH.
///
///         Added a call to DAFSIH to signal an invalid handle and a test
///         of FAILED () after it. This is to make sure that the DAF file
///         is open for reading. If this call succeeds, we know that we
///         have a valid DAF handle, so there is no need to check FAILED
///         after the call to DAFHLU.
///
///         The variable name DAFWRD was changed to IDWORD.
///
///         Added two new exceptions to the $Exceptions section: 1 and 2.
///         The remaining exception (3) was already present. The exceptions
///         that were added are not new, but are being documented for the
///         first time.
///
///
/// -    SPICELIB Version 1.0.3, 06-OCT-1992 (HAN)
///
///         Corrected a typo in the $Brief_I/O section. ND was listed
///         twice as an input, and NI was not listed.
///
/// -    SPICELIB Version 1.0.2, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.1, 22-MAR-1990 (HAN)
///
///         Literature references added to the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn dafrfr(
    ctx: &mut SpiceContext,
    handle: i32,
    nd: &mut i32,
    ni: &mut i32,
    ifname: &mut str,
    fward: &mut i32,
    bward: &mut i32,
    free: &mut i32,
) -> crate::Result<()> {
    DAFRFR(
        handle,
        nd,
        ni,
        fstr::StrBytes::new(ifname).as_mut(),
        fward,
        bward,
        free,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFRFR ( DAF, read file record )
pub fn DAFRFR(
    HANDLE: i32,
    ND: &mut i32,
    NI: &mut i32,
    IFNAME: &mut [u8],
    FWARD: &mut i32,
    BWARD: &mut i32,
    FREE: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut IDWORD = [b' '; IDWLEN as usize];
    let mut FOUND: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
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
        CHKIN(b"DAFRFR", ctx)?;
    }

    //
    // Do some initializations
    //
    fstr::assign(&mut IDWORD, b" ");

    //
    // Check to be sure that HANDLE is attached to a file that is open
    // with read access. If the call fails, check out and return.
    //
    DAFSIH(HANDLE, b"READ", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DAFRFR", ctx)?;
        return Ok(());
    }

    //
    // Retrieve all but the internal file name directly from the
    // file record.  Read the internal file name into a temporary
    // string, to be sure of the length. Check FOUND.
    //
    ZZDAFGFR(
        HANDLE,
        &mut IDWORD,
        ND,
        NI,
        IFNAME,
        FWARD,
        BWARD,
        FREE,
        &mut FOUND,
        ctx,
    )?;

    if !FOUND {
        SETMSG(b"File record not found for file handle #1. Check if program code uses handle #2 for a read or write operation.", ctx);
        ERRINT(b"#1", HANDLE, ctx);
        ERRINT(b"#2", HANDLE, ctx);
        SIGERR(b"SPICE(DAFFRNOTFOUND)", ctx)?;
        CHKOUT(b"DAFRFR", ctx)?;
        return Ok(());
    }

    CHKOUT(b"DAFRFR", ctx)?;
    Ok(())
}
