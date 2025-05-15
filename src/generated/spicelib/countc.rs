//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Count characters in a text file
///
/// Count the characters in a group of lines in a text file.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  UNIT       I   Logical unit connected to text file.
///  BLINE      I   Beginning line number.
///  ELINE      I   Ending line number.
///  LINE      I-O  Workspace.
///
///  The function returns the number of characters in a group of lines
///  in a text file.
/// ```
///
/// # Detailed Input
///
/// ```text
///  UNIT     is a logical unit that has been connected to a
///           text file by the calling program. Use the routine
///           TXTOPR to open the file for read access and get its
///           logical unit. A text file is a formatted,
///           sequential file that contains only printable
///           characters:  ASCII 32-126.
///
///  BLINE,
///  ELINE    are line numbers in the text file. BLINE is
///           the line where the count will begin, and ELINE
///           is the line where the count will end. The
///           number of characters in the beginning and ending
///           lines are included in the total count.
///
///           By convention, line 1 is the first line of the file.
///
///  LINE     on input, is an arbitrary character string whose
///           contents are ignored. LINE is used to read lines
///           from the file connected to UNIT; its function
///           is to determine the maximum length of the lines
///           that can be read from the file. Lines longer
///           than the declared length of LINE are truncated
///           as they are read.
/// ```
///
/// # Detailed Output
///
/// ```text
///  LINE     on output, is undefined.
///
///  The function returns the number of characters in the group of
///  lines in the text file beginning with BLINE and ending with ELINE.
///  Trailing blanks on a line are not included in the count.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If an error occurs while reading from the input file,
///      the error SPICE(FILEREADFAILED) is signaled.
///
///  2)  If a non-printing ASCII character is encountered during
///      the count, the error SPICE(INVALIDTEXT) is signaled.
///
///  3)  If BLINE is greater than ELINE or if the file does not
///      contain both of this lines, the error SPICE(CANNOTFINDGRP)
///      is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See argument UNIT. COUNTC rewinds the text file connected to
///  UNIT and then steps through the file. The next read statement
///  after calling COUNTC would return the line after ELINE.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine counts characters in a group of lines in a text
///  file. Using COUNTC, you can determine in advance how much space
///  is required to store those characters.
/// ```
///
/// # Examples
///
/// ```text
///  The following code fragment opens an existing text file for
///  read access and counts the characters that it contains in
///  the first five lines. We'll assume that the longest line
///  in the file is 80 characters.
///
///     INTEGER               COUNTC
///     INTEGER               UNIT
///     INTEGER               N
///     CHARACTER*(80)        LINE
///
///     CALL TXTOPR ( 'DATA.TXT', UNIT )
///
///     N = COUNTC ( UNIT, 1, 5, LINE )
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  J.E. McLean        (JPL)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 18-MAR-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.1.0, 17-MAY-1994 (HAN)
///
///         Set the default function value to either 0, 0.0D0, .FALSE.,
///         or blank depending on the type of the function.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 05-APR-1991 (JEM)
/// ```
pub fn countc(
    ctx: &mut SpiceContext,
    unit: i32,
    bline: i32,
    eline: i32,
    line: &mut str,
) -> crate::Result<i32> {
    let ret = COUNTC(
        unit,
        bline,
        eline,
        fstr::StrBytes::new(line).as_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure COUNTC ( Count characters in a text file )
pub fn COUNTC(
    UNIT: i32,
    BLINE: i32,
    ELINE: i32,
    LINE: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<i32> {
    let mut COUNTC: i32 = 0;
    let mut CHARS: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut LINECT: i32 = 0;
    let mut DONE: bool = false;

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
        COUNTC = 0;
        return Ok(COUNTC);
    } else {
        CHKIN(b"COUNTC", ctx)?;
        COUNTC = 0;
    }

    //
    // First, see if the line numbers make sense.
    //
    if ((BLINE > ELINE) || (BLINE <= 0)) {
        SETMSG(
            b"The line numbers do not make sense:  BLINE = # and  ELINE = #.",
            ctx,
        );
        ERRINT(b"#", BLINE, ctx);
        ERRINT(b"#", ELINE, ctx);
        SIGERR(b"SPICE(CANNOTFINDGRP)", ctx)?;
        CHKOUT(b"COUNTC", ctx)?;
        return Ok(COUNTC);
    }

    //
    // Read through the file, line by line, beginning with the first
    // line in the file, checking for I/O errors, and counting
    // characters in the lines between and including BLINE and ELINE.
    //
    {
        use f2rust_std::io;

        let specs = io::PosSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.rewind(specs)?;
    }

    LINECT = 0;
    CHARS = 0;
    DONE = false;

    while !DONE {
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
        // An end-of-file condition is indicated by a negative value
        // for IOSTAT. Any other non-zero value indicates some other
        // error.  If IOSTAT is zero, the read was successful.
        //
        if (IOSTAT > 0) {
            SETMSG(
                b"Error reading text file named FILENAME.The value of IOSTAT is #.",
                ctx,
            );
            ERRINT(b"#", IOSTAT, ctx);
            ERRFNM(b"FILENAME", UNIT, ctx)?;
            SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
            CHKOUT(b"COUNTC", ctx)?;
            return Ok(COUNTC);
        } else if (IOSTAT < 0) {
            SETMSG(b"Reached end of file unexpectedly at line # in file FILE.  BLINE = # and ELINE = #.", ctx);
            ERRINT(b"#", LINECT, ctx);
            ERRINT(b"#", BLINE, ctx);
            ERRINT(b"#", ELINE, ctx);
            ERRFNM(b"FILE", UNIT, ctx)?;
            SIGERR(b"SPICE(CANNOTFINDGRP)", ctx)?;
            CHKOUT(b"COUNTC", ctx)?;
            return Ok(COUNTC);
        } else {
            //
            // We've read a line successfully, so add it to the line count.
            // If this line is in the group delimited by BLINE and ELINE,
            // count the characters in it, and if this line is ELINE, we're
            // done.
            //
            LINECT = (LINECT + 1);

            if ((LINECT >= BLINE) && (LINECT <= ELINE)) {
                //
                // Add the number of characters in this line to the count.
                // If LINE is blank, LASTNB will return 0 which is just
                // what we want.
                //
                CHARS = (CHARS + LASTNB(LINE));

                //
                // Remove the printable characters from the line.  If
                // any characters remain, signal an error.
                //
                ASTRIP(
                    &LINE.to_vec(),
                    &intrinsics::CHAR(32),
                    &intrinsics::CHAR(126),
                    LINE,
                );

                if fstr::ne(LINE, b" ") {
                    SETMSG(b"Non-printing ASCII characters were found when counting characters on line number # in file FILENAME.", ctx);
                    ERRINT(b"#", LINECT, ctx);
                    ERRFNM(b"FILENAME", UNIT, ctx)?;
                    SIGERR(b"SPICE(INVALIDTEXT)", ctx)?;
                    CHKOUT(b"COUNTC", ctx)?;
                    return Ok(COUNTC);
                }
            }

            if (LINECT == ELINE) {
                DONE = true;
            }
        }
    }

    //
    // Assign the final character count.
    //
    COUNTC = CHARS;

    CHKOUT(b"COUNTC", ctx)?;
    Ok(COUNTC)
}
