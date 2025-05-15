//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

struct SaveVars {
    STDOUT: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut STDOUT: i32 = 0;
        let mut FIRST: bool = false;

        FIRST = true;

        Self { STDOUT, FIRST }
    }
}

/// To Standard Output
///
/// Write a line of text to standard output.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  LINE       I   is a line of text to be written to standard output
/// ```
///
/// # Detailed Input
///
/// ```text
///  LINE     is a character string containing text to be written
///           to standard output.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If an error occurs while attempting to write to the standard
///      output, the error is signaled by a routine in the call tree of
///      this routine.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is a macro for the subroutine call
///
///     CALL WRITLN ( LINE, STDOUT )
///
///  Where STDOUT is the logical unit connected to standard output.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose you need to create a message to be printed on the
///  user's terminal. Here is how to use TOSTDO to handle this
///  task.
///
///     CALL TOSTDO ( 'Hello. '         )
///     CALL TOSTDO ( 'My Name is HAL.' )
///     CALL TOSTDO ( 'I became operational January 12, 1997 on the ' )
///     CALL TOSTDO ( 'campus of the University of Illinois in '      )
///     CALL TOSTDO ( 'Urbana, Illinois.' )
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
/// -    SPICELIB Version 1.0.1, 03-JUN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 18-SEP-1996 (WLT)
/// ```
pub fn tostdo(ctx: &mut SpiceContext, line: &str) -> crate::Result<()> {
    TOSTDO(line.as_bytes(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure TOSTDO ( To Standard Output)
pub fn TOSTDO(LINE: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    if save.FIRST {
        STDIO(b"STDOUT", &mut save.STDOUT, ctx)?;
        save.FIRST = false;
    }

    WRITLN(LINE, save.STDOUT, ctx)?;
    Ok(())
}
