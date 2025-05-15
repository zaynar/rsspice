//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const ESCCHR: &[u8; 1 as usize] = &fstr::extend_const::<{ 1 as usize }>(b"@");
const MXESCD: i32 = 2;
const MAXENC: i32 = 64;
const ERRLEN: i32 = 80;

/// Read encoded characters from a text file
///
/// Read and decode encoded characters from a text file.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  UNIT       I   Fortran unit number of input text file.
///  N          I   Number of characters to be read and decoded.
///  DATA       O   List of decoded characters to be returned.
/// ```
///
/// # Detailed Input
///
/// ```text
///  UNIT     is the Fortran unit number for a previously opened text
///           file. All reading will begin at the CURRENT POSITION
///           in the text file.
///
///  N        is the number of characters to be read from the text file
///           attached to UNIT.
/// ```
///
/// # Detailed Output
///
/// ```text
///  DATA     is the list of characters which were read from the text
///           file attached to UNIT and decoded.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If N, the number of data items, is not positive, the error
///      SPICE(INVALIDARGUMENT) is signaled.
///
///  2)  If an error occurs while reading from the text file
///      attached to UNIT, the error SPICE(FILEREADFAILED) is signaled.
///
///  3)  If an error occurs while decoding a character, the error
///      SPICE(DECODINGERROR) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See the description of UNIT in $Detailed_Input.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine will read quoted character strings of length
///  MAXENC containing encoded characters produced by the routine
///  WRENCC, or some equivalent procedure. The reading begins at
///  the current position in a previously opened text file attached
///  to logical UNIT and continues until N contiguous characters
///  have been successfully decoded and placed in the data buffer
///  DATA or an error occurs. The current position in a file is
///  defined to be the text line immediately following the last text
///  line that was written or read.
///
///  The character strings are quoted so that a Fortran list directed
///  read may be used to read them, rather than a formatted read with
///  the format specifier FMT = '(A)'.
///
///  As the characters are decoded they are placed into the first N
///  contiguous positions in the data buffer DATA, where the first N
///  contiguous positions are determined by moving from the lowest
///  array indices to highest array indices, i.e., moving from ``left''
///  to ``right'' and ``top'' to ``bottom'' in the character array
///  DATA, beginning at the first character position, DATA(1)(1:1). So,
///  logically all of the quoted strings containing encoded data can
///  be thought of as being concatenated together into one long
///  character string.
///
///  This routine is one of a pair of routines which are used to
///  encode and decode ASCII characters:
///
///        WRENCC -- Encode and write ASCII characters to a file.
///        RDENCC -- Read and decode ASCII characters from a file.
///
///  The encoding/decoding of characters is performed to provide
///  a portable means for transferring character data values.
///
///  This routine is for use with the ASCII character set and
///  extensions to it. The supported characters must have decimal
///  values in the range from 0 to 255.
/// ```
///
/// # Examples
///
/// ```text
///  The following examples demonstrate the use of this routine. In
///  each of the examples, the variable UNIT is the Fortran logical
///  unit of a previously opened text file, and the variable N is
///  an integer which will represent the number of characters to be
///  read and decoded.
///
///  The first example demonstrates a typical correct usage of this
///  routine. The second example demonstrates what would probably be
///  the most common incorrect usage of this routine. These examples
///  are meant to be illustrative, so for the sake of brevity and
///  clarity, the length of the quoted strings expected in the input
///  text file has been shortened.
///
///  The examples use as data correctly and incorrectly encoded
///  versions of the following character string which has a length
///  of exactly 64 characters:
///
///     'Here is some data. What follows is more '//
///     'data. This is more data.                '
///
///  Example 1
///  ---------
///
///     This example demonstrates a typical usage of this routine.
///
///     Let the symbol '-->' denote the file pointer.
///
///     Let the current file pointer position and succeeding data be
///     the following:
///
///        --> 'Here is some data. W'
///            'hat follows is more '
///            'data. This is more d'
///            'ata.                '
///
///     There are exactly N = 64 characters of encoded data.
///
///     Let the character data buffer have the following
///     declaration in the calling program:
///
///        CHARACTER*(40)         DATA(2)
///
///     Then, the subroutine call
///
///        CALL RDENCC( UNIT, N, DATA )
///
///     with N = 64 would produce the following results:
///
///        DATA(1) = 'Here is some data. What follows is more '
///        DATA(2) = 'data. This is more data.'
///
///  Example 2
///  ---------
///
///     This example is meant to demonstrate what would probably be
///     a common misuse of this routine.
///
///     Let the symbol '-->' denote the file pointer.
///
///     Let the current file pointer position and succeeding data be
///     the following:
///
///        --> 'Here is some data.  '
///            'What follows is more'
///            'data. This is more  '
///            'data.               '
///
///     As in example 1, there are exactly N = 64 characters of
///     encoded data, but to make the data more ``readable'' two extra
///     spaces have been added: one at the end of the first line and
///     one at the end of the third line.
///
///     Let the character data buffer have the following
///     declaration in the calling program:
///
///        CHARACTER*(40)         DATA(2)
///
///     Then, the subroutine call
///
///        CALL RDENCC( UNIT, N, DATA )
///
///     with N = 64 would produce the following results:
///
///        DATA(1) = 'Here is some data.  What follows is more'
///        DATA(2) = ' data. This is  more dat'
///
///     This is probably not what was desired. The problem is that
///     the ``significant'' characters in the encoded string do not
///     appear contiguously; an ``extra'' blank appears at the end
///     of the first and third encoded quoted strings.
///
///  Example 3
///  ---------
///
///     This example demonstrates the use of WRENCC and RDENCC for
///     writing and subsequent reading of character data using data
///     buffers that are ``shaped'' differently, i.e., that have
///     different dimensions.
///
///     Let the input and output character data buffers have the
///     following declarations:
///
///        CHARACTER*(25)  OUTBUF(3)
///        CHARACTER*(10)  INPBUF(7)
///
///     Further, let the output buffer contain the following data:
///
///        OUTBUF(1) = 'Today is the first day of'
///        OUTBUF(2) = ' the rest of my life, so '
///        OUTBUF(3) = 'I will enjoy it.'
///
///     There are exactly N = 66 significant characters in the output
///     buffer. The code fragment
///
///        N = 66
///        CALL WRENCC ( UNIT, N, OUTBUF )
///        REWIND ( UNIT )
///        CALL RDENCC ( UNIT, N, INPBUF )
///
///     has the effect of placing the original data into the
///     differently ``shaped'' input buffer with the following
///     results:
///
///        INPBUF(1) = 'Today is t'
///        INPBUF(2) = 'he first d'
///        INPBUF(3) = 'ay of the '
///        INPBUF(4) = 'rest of my'
///        INPBUF(5) = ' life, so '
///        INPBUF(6) = 'I will enj'
///        INPBUF(7) = 'oy it.    '
///
///    No information has been lost, it is simply arranged differently.
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
/// -    SPICELIB Version 1.0.0, 20-OCT-1992 (KRG)
/// ```
pub fn rdencc(ctx: &mut SpiceContext, unit: i32, n: i32, data: CharArrayMut) -> crate::Result<()> {
    RDENCC(unit, n, data, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure RDENCC  ( Read encoded characters from a text file )
pub fn RDENCC(UNIT: i32, N: i32, DATA: CharArrayMut, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut DATA = DummyCharArrayMut::new(DATA, None, 1..);
    let mut CH = [b' '; 1 as usize];
    let mut ENCCHR = [b' '; MAXENC as usize];
    let mut ERRMSG = [b' '; ERRLEN as usize];
    let mut HEXNUM = [b' '; MXESCD as usize];
    let mut DTALEN: i32 = 0;
    let mut DTALIN: i32 = 0;
    let mut DTAPOS: i32 = 0;
    let mut ENCPOS: i32 = 0;
    let mut INTCH: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut NCHARS: i32 = 0;
    let mut NESCD: i32 = 0;
    let mut ERROR: bool = false;
    let mut ESCAPE: bool = false;

    //
    // SPICELIB functions
    //
    //
    // Local parameters
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
        CHKIN(b"RDENCC", ctx)?;
    }
    //
    // Check to see if the number of data items is less than or equal
    // to zero. If it is, signal an error.
    //
    if (N < 1) {
        SETMSG(
            b"The number of data items to be read was not positive: #.",
            ctx,
        );
        ERRINT(b"#", N, ctx);
        SIGERR(b"SPICE(INVALIDARGUMENT)", ctx)?;
        CHKOUT(b"RDENCC", ctx)?;
        return Ok(());
    }
    //
    // Initialize some stuff here
    //
    // Make sure that the encoding character string is empty when we
    // start.
    //
    fstr::assign(&mut ENCCHR, b" ");
    //
    // We have not encountered any errors yet, so set the error indicator
    // to .FALSE..
    //
    ERROR = false;
    //
    // Get the length of a data ``line'' in the data buffer DATA.
    //
    DTALEN = intrinsics::LEN(&DATA[1]);
    //
    // We are not currently parsing an escaped character, so set the
    // escape indicator to .FALSE. and set the number of escape digits
    // to zero.
    //
    ESCAPE = false;
    NESCD = 0;
    //
    // Set the initial line and position for the output data buffer.
    //
    DTAPOS = 1;
    DTALIN = 1;
    //
    // Set the initial position in the encoding buffer to be 1 too
    // big so that we read an encoded character string from the file
    // attached to UNIT on the first pass through the loop.
    //
    ENCPOS = (MAXENC + 1);
    //
    // Set the number of characters decoded to zero and begin the
    // decoding loop.
    //
    NCHARS = 0;
    while (NCHARS < N) {
        //
        // If the last character we processed was the last one in the
        // encoded character string, then we need to read in the next
        // encoded character string from the file. This also accomplishes
        // the task of reading in the first encoded character string.
        //
        if (ENCPOS > MAXENC) {
            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Reader},
                };

                let mut reader = io::ListDirectedReader::new(ctx.io_unit(UNIT)?, None)?;
                IOSTAT = io::capture_iostat(|| {
                    reader.start()?;
                    reader.read_str(&mut ENCCHR)?;
                    reader.finish()?;
                    Ok(())
                })?;
            }
            if (IOSTAT != 0) {
                SETMSG(b"Error reading from logical unit #, IOSTAT = #.", ctx);
                ERRINT(b"#", UNIT, ctx);
                ERRINT(b"#", IOSTAT, ctx);
                SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
                CHKOUT(b"RDENCC", ctx)?;
                return Ok(());
            }
            //
            // Set the pointer for the encoded character buffer to the
            // beginning of the buffer.
            //
            ENCPOS = 1;
        }

        fstr::assign(&mut CH, fstr::substr(&ENCCHR, ENCPOS..=ENCPOS));
        //
        // If we are processing a character which was escaped when it was
        // encoded, we need to do some special stuff.
        //
        if ESCAPE {
            NESCD = (NESCD + 1);

            if (NESCD == MXESCD) {
                //
                // If we have all of the digits in the encoded character,
                // then decode it.
                //
                fstr::assign(fstr::substr_mut(&mut HEXNUM, NESCD..=NESCD), &CH);
                HX2INT(&HEXNUM, &mut INTCH, &mut ERROR, &mut ERRMSG, ctx);

                if ERROR {
                    SETMSG(
                        b"Decoding error occurred while attempting to decode item #: @#. #",
                        ctx,
                    );
                    ERRINT(b"#", (NCHARS + 1), ctx);
                    ERRCH(b"#", &HEXNUM, ctx);
                    ERRCH(b"#", &ERRMSG, ctx);
                    SIGERR(b"SPICE(DECODINGERROR)", ctx)?;
                    CHKOUT(b"RDENCC", ctx)?;
                    return Ok(());
                }

                fstr::assign(&mut CH, &intrinsics::CHAR(INTCH));
                //
                // We now have the decoded character. We are no longer
                // processing an escaped character, so set the escape
                // indicator to .FALSE. and continue. The character we
                // just decoded will be placed into the data buffer DATA
                // below.
                //
                ESCAPE = false;
                NESCD = 0;
            } else if ((NESCD < MXESCD) && (NESCD > 0)) {
                //
                // Otherwise we are still collecting the digits of the
                // encoded character, so store the current character and
                // move on to the next one.
                //
                fstr::assign(fstr::substr_mut(&mut HEXNUM, NESCD..=NESCD), &CH);
            }
        } else {
            //
            // Check to see if the current character is the escape
            // character. If it is, we need to set the escape indicator
            // to .TRUE. so that we correctly process the encoded
            // digits.
            //
            if fstr::eq(&CH, ESCCHR) {
                ESCAPE = true;
            }
        }
        //
        // At this point one of the following is true:
        //
        //    (1) CH contains a character to be placed into the data
        //        buffer DATA.
        //
        //    (2) We are currently building an escaped character from
        //        its escape sequence, ESCAPE = .TRUE., and CH contains
        //        some part of the escape sequence.
        //
        // If we are not currently decoding an escaped character, then
        // we need to store the character value that we have in the data
        // buffer, and move on to the next character.
        //
        if !ESCAPE {
            NCHARS = (NCHARS + 1);
            //
            // If the position in the data buffer is greater than the
            // length  of a data line (DTALEN) then we need to increment
            // the current data line (DTALIN) and reset the current data
            // line buffer position (DTAPOS).
            //
            if (DTAPOS > DTALEN) {
                DTALIN = (DTALIN + 1);
                DTAPOS = 1;
            }
            //
            // Store the current character in the data buffer and
            // increment the buffer position.
            //
            fstr::assign(fstr::substr_mut(DATA.get_mut(DTALIN), DTAPOS..=DTAPOS), &CH);
            DTAPOS = (DTAPOS + 1);
        }
        //
        // Increment the encoded character buffer position
        //
        ENCPOS = (ENCPOS + 1);
        //
        // At this point, we know the following:
        //
        // (1)  1 <= ENCPOS <= MAXENC
        // (2)  1 <= NCHARS <= N
        // (3)  1 <= DTAPOS <= DTALEN
        // (4)  1 <= DTALIN
        // (5)  0 <= NESCD <= MXESCD
        // (6)  ESCAPE is .TRUE. if we are currently decoding an escaped
        //      character, otherwise it is .FALSE..
        //
    }

    CHKOUT(b"RDENCC", ctx)?;
    Ok(())
}
