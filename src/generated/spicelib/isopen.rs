//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Is a file currently open?
///
/// Determine whether a named file is currently open.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FILE       I   Name of the file in question.
///
///  The function returns the value .TRUE. if the file is open, .FALSE.
///  otherwise.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FILE     is the name of the file in question.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the value .TRUE. if the file is open, .FALSE.
///  otherwise.
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
/// ```
///
/// # Particulars
///
/// ```text
///  Use the Fortran INQUIRE statement to determine the open status
///  of FILE.
/// ```
///
/// # Examples
///
/// ```text
///  The following code fragment illustrates the use of ISOPEN.
///
///        IF ( .NOT. ISOPEN ( FILE ) ) THEN
///           Open the file here
///        ELSE
///           ERROR = 'Input file is already open.'
///        END IF
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
/// -    SPICELIB Version 1.2.0, 12-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.1.0, 29-FEB-1996 (KRG)
///
///         Added a local logical variable that is used as temporary
///         storage for the results from the INQUIRE statement rather
///         than using the function name. This solved a problem on the
///         macintosh.
///
/// -    SPICELIB Version 1.0.0, 05-OCT-1994 (KRG)
/// ```
pub fn isopen(ctx: &mut SpiceContext, file: &str) -> crate::Result<bool> {
    let ret = ISOPEN(file.as_bytes(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure ISOPEN ( Is a file currently open? )
pub fn ISOPEN(FILE: &[u8], ctx: &mut Context) -> f2rust_std::Result<bool> {
    let mut ISOPEN: bool = false;
    let mut IOSTAT: i32 = 0;
    let mut EXISTS: bool = false;
    let mut MYOPEN: bool = false;

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
        ISOPEN = false;
        return Ok(ISOPEN);
    } else {
        CHKIN(b"ISOPEN", ctx)?;
    }

    //
    // First we test to see if the filename is blank.
    //
    if fstr::eq(FILE, b" ") {
        ISOPEN = false;
        SETMSG(b"The file name is blank. ", ctx);
        SIGERR(b"SPICE(BLANKFILENAME)", ctx)?;
        CHKOUT(b"ISOPEN", ctx)?;
        return Ok(ISOPEN);
    }

    //
    // So simple, it defies explanation.
    //
    {
        use f2rust_std::io;

        let specs = io::InquireSpecs {
            file: Some(FILE),
            exist: Some(&mut EXISTS),
            opened: Some(&mut MYOPEN),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.inquire(specs))?;
    }

    if (IOSTAT != 0) {
        ISOPEN = false;
        SETMSG(b"Value of IOSTAT was *.", ctx);
        ERRINT(b"*", IOSTAT, ctx);
        SIGERR(b"SPICE(INQUIREFAILED)", ctx)?;
        CHKOUT(b"ISOPEN", ctx)?;
        return Ok(ISOPEN);
    }
    //
    // A file cannot be open if it does not exist. We do actually need to
    // check this because some operating environments return .TRUE. for
    // the value of OPENED if a file does not exist.
    //
    if !EXISTS {
        MYOPEN = false;
    }
    //
    // Set the function value, check out, and return.
    //

    ISOPEN = MYOPEN;

    CHKOUT(b"ISOPEN", ctx)?;
    Ok(ISOPEN)
}
