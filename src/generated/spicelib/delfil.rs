//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Delete a file
///
/// Delete a file.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FILNAM     I   The name of a file to be deleted.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FILNAM   is the name of a file that is to be deleted. Upon
///           successful completion of this routine this file will
///           no longer exist. The file to be deleted must be closed
///           when this routine is called.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the file name is blank, the error SPICE(BLANKFILENAME)
///      is signaled.
///
///  2)  If the inquire on the filename specified by FILNAM fails for
///      some reason, the error SPICE(INQUIREERROR) is signaled.
///
///  3)  If the file specified by FILNAM is already open, the error
///      SPICE(FILECURRENTLYOPEN) is signaled.
///
///  4)  If the file specified by FILNAM does not exist, the error
///      SPICE(NOSUCHFILE) is signaled.
///
///  5)  If the attempt to open the file specified by FILNAM fails,
///      the error SPICE(FILEOPENFAILED) is signaled.
///
///  6)  If the attempt to close the file with STATUS='DELETE' fails,
///      the error SPICE(FILEDELETEFAILED) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  The  file specified by FILNAM is opened and then closed by this
///  routine with STATUS = 'DELETE' to delete it. The file must be
///  closed for this routine to delete it.
/// ```
///
/// # Particulars
///
/// ```text
///  This subroutine is a support utility that deletes a file.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose you wish to delete a file named 'delete.me' in the
///  current directory. The code fragment below would accomplish this.
///
///     FILE = 'delete.me'
///     CALL DELFIL ( FILE )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The file to be deleted must be closed when this routine is
///      invoked.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 02-JUN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 20-DEC-1995 (KRG)
/// ```
pub fn delfil(ctx: &mut SpiceContext, filnam: &str) -> crate::Result<()> {
    DELFIL(filnam.as_bytes(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DELFIL ( Delete a file  )
pub fn DELFIL(FILNAM: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut IOSTAT: i32 = 0;
    let mut LUNIT: i32 = 0;
    let mut EXISTS: bool = false;
    let mut OPENED: bool = false;

    //
    // Spicelib Routines
    //
    //
    // Local Variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DELFIL", ctx)?;
    }
    //
    // Check to see if the filename we got is blank. If it is, signal an
    // error and return.
    //
    if fstr::eq(FILNAM, b" ") {
        SETMSG(b"The file name is blank.", ctx);
        SIGERR(b"SPICE(BLANKFILENAME)", ctx)?;
        CHKOUT(b"DELFIL", ctx)?;
        return Ok(());
    }
    //
    // We inquire before we try opening anything to see if the file
    // exists or is currently open.
    //
    {
        use f2rust_std::io;

        let specs = io::InquireSpecs {
            file: Some(FILNAM),
            exist: Some(&mut EXISTS),
            opened: Some(&mut OPENED),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.inquire(specs))?;
    }
    //
    // Not too likely, but if the INQUIRE statement fails signal an error
    // and return.
    //
    if (IOSTAT != 0) {
        SETMSG(b"INQUIRE statement failed for file \'#\'. IOSTAT = #.", ctx);
        ERRCH(b"#", FILNAM, ctx);
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(INQUIREFAILED)", ctx)?;
        CHKOUT(b"DELFIL", ctx)?;
        return Ok(());
    }
    //
    // The file ought to exist if you're trying to delete it. If not,
    // signal an error and return.
    //
    if !EXISTS {
        SETMSG(b"The file \'#\' does not exist.", ctx);
        ERRCH(b"#", FILNAM, ctx);
        SIGERR(b"SPICE(NOSUCHFILE)", ctx)?;
        CHKOUT(b"DELFIL", ctx)?;
        return Ok(());
    }
    //
    // The file that is to be deleted should not be in use, indicated by
    // it being open, by anything when we try to delete it. If it is
    // open, signal an error and return.
    //
    if OPENED {
        SETMSG(
            b"The file \'#\' is currently open and cannot be deleted.",
            ctx,
        );
        ERRCH(b"#", FILNAM, ctx);
        SIGERR(b"SPICE(FILECURRENTLYOPEN)", ctx)?;
        CHKOUT(b"DELFIL", ctx)?;
        return Ok(());
    }
    //
    // Get an available logical unit and attempt to open the file.
    //
    GETLUN(&mut LUNIT, ctx)?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(LUNIT),
            file: Some(FILNAM),
            status: Some(b"OLD"),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
    }
    //
    // If we had trouble opening the file, signal an appropriate error
    // and return.
    //
    if (IOSTAT != 0) {
        SETMSG(b"Attempt to open the file \'#\' failed.", ctx);
        ERRCH(b"#", FILNAM, ctx);
        SIGERR(b"SPICE(FILEOPENFAILED)", ctx)?;
        CHKOUT(b"DELFIL", ctx)?;
        return Ok(());
    }
    //
    // We opened the file successfully, so let's try to close it with
    // STATUS = 'DELETE'. If this fails, attempt to just close the file,
    // signal an error and return.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(LUNIT),
            status: Some(b"DELETE"),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.close(specs))?;
    }

    if (IOSTAT != 0) {
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(LUNIT),
                ..Default::default()
            };
            ctx.close(specs)?;
        }
        SETMSG(b"Attempt to delete the file \'#\' failed.", ctx);
        ERRCH(b"#", FILNAM, ctx);
        SIGERR(b"SPICE(FILEDELETEFAILED)", ctx)?;
        CHKOUT(b"DELFIL", ctx)?;
        return Ok(());
    }

    CHKOUT(b"DELFIL", ctx)?;
    Ok(())
}
