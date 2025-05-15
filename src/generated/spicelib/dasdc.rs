//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const IDWLEN: i32 = 8;
const IFNLEN: i32 = 60;

/// DAS delete comments
///
/// Delete the entire comment area of a previously opened binary
/// DAS file.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   The handle of a binary DAS file opened for writing.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a binary DAS file that is to have its
///           entire comment area deleted. The DAS file should have
///           been opened with write access.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the binary DAS file attached to HANDLE is not open with
///      write access, an error is signaled by a routine in the call
///      tree of this routine.
/// ```
///
/// # Files
///
/// ```text
///  See argument HANDLE in $Detailed_Input.
/// ```
///
/// # Particulars
///
/// ```text
///  Binary DAS files contain an area which is reserved for storing
///  annotations or descriptive textual information about the data
///  contained in a file. This area is referred to as the ``comment
///  area'' of the file. The comment area of a DAS file is a line
///  oriented medium for storing textual information. The comment area
///  preserves any leading or embedded white space in the line(s) of
///  text which are stored, so that the appearance of the information
///  will be unchanged when it is retrieved (extracted) at some other
///  time. Trailing blanks, however, are NOT preserved, due to the way
///  that character strings are represented in standard Fortran 77.
///
///  This routine will delete the entire comment area from the binary
///  DAS file attached to HANDLE. The size of the binary DAS file will
///  remain unchanged. The space that was used by the comment records
///  is reclaimed.
/// ```
///
/// # Examples
///
/// ```text
///  Let
///
///        HANDLE   be the handle for a DAS file which has been opened
///                 with write access.
///
///  The call
///
///        CALL DASDC ( HANDLE )
///
///  will delete the entire comment area of the binary DAS file
///  attached to HANDLE.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 02-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE standard.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section.
///
/// -    SPICELIB Version 1.0.2, 11-NOV-2016 (NJB)
///
///         Fixed typo in $Particulars header section.
///
/// -    SPICELIB Version 1.0.1, 26-OCT-1993 (KRG)
///
///         Changed the $Brief_I/O description of handle. It now mentions
///         that the file must be open for writing. Also added a statement
///         to the $Detailed_Input section to the effect that the DAS file
///         should have been opened with write access.
///
/// -    SPICELIB Version 1.0.0, 24-NOV-1992 (KRG)
/// ```
pub fn dasdc(ctx: &mut SpiceContext, handle: i32) -> crate::Result<()> {
    DASDC(handle, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASDC    ( DAS delete comments )
pub fn DASDC(HANDLE: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut IDWORD = [b' '; IDWLEN as usize];
    let mut IFNAME = [b' '; IFNLEN as usize];
    let mut NCOMC: i32 = 0;
    let mut NCOMR: i32 = 0;
    let mut NRESVC: i32 = 0;
    let mut NRESVR: i32 = 0;

    //
    // SPICELIB functions
    //
    //
    // Local parameters
    //
    // Length of a DAS file ID word.
    //
    //
    // Length of a DAS file internal filename.
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
        CHKIN(b"DASDC", ctx)?;
    }
    //
    // Verify that the DAS file attached to HANDLE is opened with write
    // access.
    //
    DASSIH(HANDLE, b"WRITE", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DASDC", ctx)?;
        return Ok(());
    }
    //
    // Read the file record to obtain the current number of comment
    // records in the DAS file attached to HANDLE. We will also get
    // back some extra stuff that we do not use.
    //
    DASRFR(
        HANDLE,
        &mut IDWORD,
        &mut IFNAME,
        &mut NRESVR,
        &mut NRESVC,
        &mut NCOMR,
        &mut NCOMC,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"DASDC", ctx)?;
        return Ok(());
    }
    //
    // Now we will attempt to remove the comment records, if there are
    // any, otherwise we do nothing.
    //
    if (NCOMR > 0) {
        DASRCR(HANDLE, NCOMR, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"DASDC", ctx)?;
            return Ok(());
        }
        //
        // Now we need to update the DAS file record.
        //
        // Read in the updated file record since it has been modified:
        // we deleted all of the comment records.
        //
        DASRFR(
            HANDLE,
            &mut IDWORD,
            &mut IFNAME,
            &mut NRESVR,
            &mut NRESVC,
            &mut NCOMR,
            &mut NCOMC,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"DASDC", ctx)?;
            return Ok(());
        }
        //
        // Zero out the number of comment characters, and write the
        // updated file record to the file.
        //
        NCOMC = 0;

        DASWFR(HANDLE, &IDWORD, &IFNAME, NRESVR, NRESVC, NCOMR, NCOMC, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"DASDC", ctx)?;
            return Ok(());
        }
    }
    //
    // We're done now, so goodbye.
    //
    CHKOUT(b"DASDC", ctx)?;
    Ok(())
}
