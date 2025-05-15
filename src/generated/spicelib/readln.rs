//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Read a text line from a logical unit
///
/// Read a single line of text from the Fortran logical unit UNIT,
/// reporting the end of file if it occurs.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  UNIT       I   The Fortran unit number to use for input.
///  LINE       O   The line read from the file.
///  EOF        O   A logical flag indicating the end of file.
/// ```
///
/// # Detailed Input
///
/// ```text
///  UNIT     is the Fortran unit number for the input. This may
///           be either the unit number for the terminal, or the
///           unit number of a previously opened text file.
/// ```
///
/// # Detailed Output
///
/// ```text
///  LINE     is the next text line encountered when reading from UNIT.
///
///           If the length of the character string LINE is shorter
///           than the length of the current line in the text file, the
///           line is truncated on the right by the Fortran READ
///           statement, filling LINE with the first LEN(LINE)
///           characters from the current line in the file.
///
///           If an error or the end of file occurs during the
///           attempt to read from UNIT, the value of this variable
///           is not guaranteed.
///
///  EOF      is .TRUE. if the end of file ( IOSTAT < 0 ) is
///           encountered during the attempt to read from unit UNIT.
///           Otherwise, this variable will be set to .FALSE.
/// ```
///
/// # Exceptions
///
/// ```text
///  This routine only checks in with the error handler in the event
///  that an error occurred. (Discovery check in)
///
///  1)  If an error occurs while attempting to read from the text file
///      attached to UNIT, the error SPICE(FILEREADFAILED) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine will read a single line, a text record, from the
///  logical unit UNIT. UNIT may be the terminal, or it may be a
///  logical unit number obtained from a Fortran OPEN or INQUIRE
///  statement. This routine will set a logical flag, EOF, on output
///  if the end of the file is encountered during the read attempt.
/// ```
///
/// # Examples
///
/// ```text
///  CALL READLN ( UNIT, LINE, EOF )
///
///  IF ( EOF ) THEN
///     < The end of file, deal with it appropriately >
///  END IF
///
///  You now have a line of text from unit UNIT.
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
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Version history entries for Beta versions.
///
/// -    SPICELIB Version 1.0.0, 20-DEC-1995 (KRG)
/// ```
pub fn readln(
    ctx: &mut SpiceContext,
    unit: i32,
    line: &mut str,
    eof: &mut bool,
) -> crate::Result<()> {
    READLN(
        unit,
        fstr::StrBytes::new(line).as_mut(),
        eof,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure READLN ( Read a text line from a logical unit )
pub fn READLN(
    UNIT: i32,
    LINE: &mut [u8],
    EOF: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut IOSTAT: i32 = 0;

    //
    // Local variables
    //
    //
    // Standard SPICE error handling.
    //
    //
    // Read in the next line from the text file attached to UNIT.
    //
    {
        use f2rust_std::{
            data::Val,
            io::{self, Reader},
        };

        let mut reader = io::FormattedReader::new(ctx.io_unit(UNIT)?, None, b"(A)")?;
        IOSTAT = io::capture_iostat(|| {
            reader.start()?;
            reader.read_str(LINE)?;
            reader.finish()?;
            Ok(())
        })?;
    }
    //
    // Check to see if we got a read error, and signal it if we did.
    //
    if (IOSTAT > 0) {
        CHKIN(b"READLN", ctx)?;
        SETMSG(b"Error reading from file: #. IOSTAT = #.", ctx);
        ERRFNM(b"#", UNIT, ctx)?;
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
        CHKOUT(b"READLN", ctx)?;
        return Ok(());
    }
    //
    // Check to see if we got the end of file, and set the logical
    // flag EOF if we did.
    //
    if (IOSTAT < 0) {
        *EOF = true;
    } else {
        *EOF = false;
    }

    Ok(())
}
