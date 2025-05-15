//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Map name of open file to its logical unit.
///
/// Map the name of an open file to its associated logical unit.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FILNAM     I   Name of the file to be mapped to its logical unit.
///  LUNIT      O   The logical unit associated with the filename.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FILNAM   is the filename that is to be mapped to its associated
///           Fortran logical unit.
/// ```
///
/// # Detailed Output
///
/// ```text
///  LUNIT    is the Fortran logical unit that is associated with the
///           filename FILNAM. The file must be open for this routine
///           to work properly.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the filename is blank, the error SPICE(BLANKFILENAME) is
///      signaled.
///
///  2)  If an error occurs during the execution of the Fortran INQUIRE
///      statement, the error SPICE(INQUIREFAILED) is signaled.
///
///  3)  If the filename is not associated with an open file, the
///      error SPICE(FILENOTOPEN) is signaled.
///
///  4)  If the filename is not associated with an existing file, the
///      error SPICE(FILEDOESNOTEXIST) is signaled.
///
///  5)  In the event of an error the contents of the variable LUNIT
///      are not defined, and should not be used.
/// ```
///
/// # Particulars
///
/// ```text
///  Use the Fortran INQUIRE statement to determine the filename
///  that is associated with the Fortran logical unit LUNIT.
/// ```
///
/// # Examples
///
/// ```text
///  The following code fragment illustrates the use of FN2LUN.
///
///  C
///  C      Convert the logical unit to its filename and display it.
///  C
///         CALL FN2LUN ( FNAME, LUNIT )
///         WRITE (*,*) 'The logical unit is: ', LUNIT
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
/// -    SPICELIB Version 1.1.0, 12-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 16-AUG-1994 (KRG)
/// ```
pub fn fn2lun(ctx: &mut SpiceContext, filnam: &str, lunit: &mut i32) -> crate::Result<()> {
    FN2LUN(filnam.as_bytes(), lunit, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure FN2LUN ( Map name of open file to its logical unit. )
pub fn FN2LUN(FILNAM: &[u8], LUNIT: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut IOSTAT: i32 = 0;
    let mut EXISTS: bool = false;
    let mut OPENED: bool = false;

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
        CHKIN(b"FN2LUN", ctx)?;
    }

    //
    // First we test to see if the filename is blank.
    //
    if fstr::eq(FILNAM, b" ") {
        SETMSG(b"The filename is blank.", ctx);
        SIGERR(b"SPICE(BLANKFILENAME)", ctx)?;
        CHKOUT(b"FN2LUN", ctx)?;
        return Ok(());
    }

    //
    // So simple, it defies explanation: just INQUIRE.
    //
    {
        use f2rust_std::io;

        let specs = io::InquireSpecs {
            file: Some(FILNAM),
            number: Some(LUNIT),
            exist: Some(&mut EXISTS),
            opened: Some(&mut OPENED),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.inquire(specs))?;
    }

    if (IOSTAT != 0) {
        SETMSG(
            b"INQUIRE error on file \'#\'. The value of IOSTAT is: #.",
            ctx,
        );
        ERRCH(b"#", FILNAM, ctx);
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(INQUIREFAILED)", ctx)?;
        CHKOUT(b"FN2LUN", ctx)?;
        return Ok(());
    }

    //
    // A file cannot be open if it does not exist. We need to check this
    // because for some environments files are considered to be open if
    // they do not exist.
    //
    if !EXISTS {
        SETMSG(b"No file with the name \'#\' was found.", ctx);
        ERRCH(b"#", FILNAM, ctx);
        SIGERR(b"SPICE(FILEDOESNOTEXIST)", ctx)?;
        CHKOUT(b"FN2LUN", ctx)?;
        return Ok(());
    }

    //
    // Now check to see if the file is opened. If not, then it is an
    // error, there cannot be a logical unit associated with it..
    //
    if !OPENED {
        SETMSG(
            b"There was not an open file associated with the filename \'#\'.",
            ctx,
        );
        ERRCH(b"#", FILNAM, ctx);
        SIGERR(b"SPICE(FILENOTOPEN)", ctx)?;
        CHKOUT(b"FN2LUN", ctx)?;
        return Ok(());
    }

    CHKOUT(b"FN2LUN", ctx)?;
    Ok(())
}
