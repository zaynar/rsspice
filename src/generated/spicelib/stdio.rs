//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const WDSIZE: i32 = 8;

/// Standard IO
///
/// Return the logical unit associated with some standard input or
/// standard output.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NAME       I   is the name of a logical unit to return.
///  UNIT       O   is the logical unit associated with NAME.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NAME     is the "name" of a FORTRAN unit to return.
///           Recognized names are 'STDIN' and 'STDOUT'.
///           The routine is case insensitive to NAME.
///
///           If NAME is not recognized the error
///           SPICE(BADSTDIONAME) is signaled and UNIT is
///           set to -100.
/// ```
///
/// # Detailed Output
///
/// ```text
///  UNIT     is the logical unit associated with NAME. If
///           NAME is not recognized, UNIT is set to -100.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If NAME is not recognized, the error SPICE(BADSTDIONAME) is
///      signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This is a low level utility for retrieving the logical units
///  associated with standard input and output. It exists to
///  isolate SPICE based code from compiler writer choices in the
///  implementation of standard input and output.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose you would like to send a message to standard output
///  and that this message is contained in the array of N character
///  strings MESSGE. The code below would handle the task.
///
///     CALL STDIO ( 'STDOUT', STDOUT )
///
///     DO I = 1, N
///        CALL WRITLN ( MESSGE(I), STDOUT )
///     END DO
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 20-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Spelled out I/O
///         in $Keywords section.
///
/// -    SPICELIB Version 1.0.0, 18-SEP-1996 (WLT)
/// ```
pub fn stdio(ctx: &mut SpiceContext, name: &str, unit: &mut i32) -> crate::Result<()> {
    STDIO(name.as_bytes(), unit, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure STDIO ( Standard IO )
pub fn STDIO(NAME: &[u8], UNIT: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut MYNAME = [b' '; WDSIZE as usize];

    //
    // Spicelib Functions
    //

    //
    // Local Variables
    //

    LJUST(NAME, &mut MYNAME);
    UCASE(&MYNAME.clone(), &mut MYNAME, ctx);

    if fstr::eq(&MYNAME, b"STDIN") {
        *UNIT = 5;
    } else if fstr::eq(&MYNAME, b"STDOUT") {
        *UNIT = 6;
    } else if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"STDIO", ctx)?;
        SETMSG(b"The only \"names\" recognized by STDIO are \'STDIN\' and \'STDOUT\' you requested a unit for \'#\'. ", ctx);
        ERRCH(b"#", NAME, ctx);
        SIGERR(b"SPICE(BADSTDIONAME)", ctx)?;
        CHKOUT(b"STDIO", ctx)?;
    }

    Ok(())
}
