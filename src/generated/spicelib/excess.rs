//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const LONGLN: i32 = 320;

/// Report an excess of elements in a cell
///
/// Set the long error message so as to indicate the number of excess
/// elements encountered by a routine operating on cells or on data
/// structures based on cells.
///
/// # Required Reading
///
/// * [CELLS](crate::required_reading::cells)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NUMBER     I   Number of excess elements.
///  STRUCT     I   Name of the data structure.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NUMBER   is the number of excess elements encountered.
///           This may be zero or negative, which indicates
///           no excess.
///
///  STRUCT   is the name of the data structure being manipulated.
///           Typically, this is one of the strings: 'cell', 'set',
///           or 'symbol table'. However, it may be any character
///           string. STRUCT should NOT end in a period.
///           The period at the end of the message is supplied
///           automatically.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  This routine does not detect any errors.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is part of the SPICELIB error handling mechanism.
///
///  EXCESS sets the long error message. The message has the form:
///
///              An excess of <NUMBER> element(s) could
///              not be accommodated in the output <STRUCT>.
///
///  Leading and trailing blanks in STRUCT are removed. If there is
///  no excess (NUMBER is zero or negative), then is blank.
/// ```
///
/// # Examples
///
/// ```text
///  The response of EXCESS to a variety of inputs is illustrated
///  below.
///
///        NUMBER = 1
///        STRUCT = 'set'
///        ERROR  = 'An excess of 1 element could not
///                  be accommodated in the output set.'
///
///        NUMBER = 5
///        STRUCT = 'stack'
///        ERROR  =  An excess of 5 elements could not
///                  be accommodated in the output stack.'
///
///        NUMBER = 0
///        STRUCT =
///        ERROR  = ' '
///
///        NUMBER = -6
///        STRUCT =
///        ERROR  = ' '
///
///  In particular, note that EXCESS does not set the long error
///  message when the number of excess elements is not positive. Also,
///  the singular 'element' is used for an excess of one, while
///  the plural 'elements' is used for all other positive excesses.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  C.A. Curzon        (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 13-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (CAC) (WLT) (IMU) (NJB)
/// ```
///
/// # Revisions
///
/// ```text
/// -    Beta Version 2.0.0, 11-JAN-1989 (NJB)
///
///         Sets the long error message directly. No longer returns
///         an error message. Message no longer contains name of
///         routine which detected the error.
/// ```
pub fn excess(ctx: &mut SpiceContext, number: i32, struct_: &str) -> crate::Result<()> {
    EXCESS(number, struct_.as_bytes(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EXCESS ( Report an excess of elements in a cell )
pub fn EXCESS(NUMBER: i32, STRUCT: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ERROR = [b' '; LONGLN as usize];

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //
    //
    // Set up the error processing.
    //
    if RETURN(ctx) {
        return Ok(());
    }
    CHKIN(b"EXCESS", ctx)?;
    //
    // If there is no excess, don't report one.
    //
    if (NUMBER > 0) {
        //
        // Begin with the number. We will build the rest of the
        // message around it.
        //
        INTSTR(NUMBER, &mut ERROR, ctx);

        //
        // A short blurb goes in front of the number.
        //
        PREFIX(b"An excess of", 1, &mut ERROR);

        //
        // Singular or plural?
        //
        if (NUMBER == 1) {
            SUFFIX(b"element", 1, &mut ERROR);
        } else {
            SUFFIX(b"elements", 1, &mut ERROR);
        }

        //
        // Another short blurb.
        //
        SUFFIX(b"could not be accommodated in the output", 1, &mut ERROR);

        //
        // And the name of the structure.
        //
        SUFFIX(STRUCT, 1, &mut ERROR);

        //
        // And a period at the end, to complete the sentence.
        //
        SUFFIX(b".", 0, &mut ERROR);

        //
        // Set the long error message:
        //
        SETMSG(&ERROR, ctx);
    } else {
        fstr::assign(&mut ERROR, b" ");
    }

    CHKOUT(b"EXCESS", ctx)?;

    Ok(())
}
