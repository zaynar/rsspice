//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Shift a character string
///
/// Shift the contents of a character string to the left or right.
/// Characters moved past the beginning or end of the string are
/// lost. Vacant spaces are filled with a specified character.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  IN         I   Input string.
///  DIR        I   Direction to shift.
///  NSHIFT     I   Number of times to shift.
///  FILLC      I   Character to fill spaces left vacant.
///  OUT        O   Shifted string.
/// ```
///
/// # Detailed Input
///
/// ```text
///  IN       is the input character string.
///
///  DIR      is the direction in which the characters in the
///           string are to be shifted.
///
///                 'L' or 'l'  to shift left.
///                 'R' or 'r'  to shift right.
///
///  NSHIFT   is the number of times the string is to be
///           shifted.
///
///  FILLC    is the character with which spaces left vacant by
///           the shift are to be filled.
/// ```
///
/// # Detailed Output
///
/// ```text
///  OUT      is the output string. This is the input string,
///           shifted N times, filled with FILLC.
///
///           OUT may overwrite IN.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  A negative shift in one direction is equal to a positive
///      shift in the other.
///
///  2)  If a legal direction ('L', 'l', 'R', 'r') is not supplied,
///      the error SPICE(ILLEGSHIFTDIR) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  The first NSHIFT characters of the output string are filled
///  with the fill character, and the input string is appended.
/// ```
///
/// # Examples
///
/// ```text
///  If FILLC = ' '
///
///         'abcde'   shifted left twice becomes     'cde  '
///         'abcde'   shifted right once becomes     ' abcd'
///
///  If FILLC = '.'
///
///         '12345 '  shifted right once becomes     '.12345'
///         'Apple '  shifted left ten times becomes '......'
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  SHIFTC is being maintained for historical reasons only.
///      To avoid the overhead imposed by the error handling in this
///      routine, use the equivalent routines SHIFTL and SHIFTR.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
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
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    Beta Version 1.1.0, 17-OCT-1988 (IMU)
///
///         Dick Simpson reported that the statement
///
///            OUT(N+1: ) = IN
///
///         which began the right-shift section failed on his Data
///         General, presumably because it requires temporary buffering
///         of characters. The new version seems to work for all cases.
///         It has been tested on the VAX and on the Sun (f77 compiler).
/// ```
pub fn shiftc(
    ctx: &mut SpiceContext,
    in_: &str,
    dir: char,
    nshift: i32,
    fillc: char,
    out: &mut str,
) -> crate::Result<()> {
    SHIFTC(
        in_.as_bytes(),
        &[u8::try_from(dir).unwrap()],
        nshift,
        &[u8::try_from(fillc).unwrap()],
        fstr::StrBytes::new(out).as_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SHIFTC ( Shift a character string )
pub fn SHIFTC(
    IN: &[u8],
    DIR: &[u8],
    NSHIFT: i32,
    FILLC: &[u8],
    OUT: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DIR = &DIR[..1];
    let FILLC = &FILLC[..1];

    //
    // SPICELIB functions
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"SHIFTC", ctx)?;
    }

    //
    // Hand off to one of the other routines.
    //
    if (fstr::eq(DIR, b"L") || fstr::eq(DIR, b"l")) {
        if (NSHIFT >= 0) {
            SHIFTL(IN, NSHIFT, FILLC, OUT);
        } else {
            SHIFTR(IN, -NSHIFT, FILLC, OUT);
        }
    } else if (fstr::eq(DIR, b"R") || fstr::eq(DIR, b"r")) {
        if (NSHIFT >= 0) {
            SHIFTR(IN, NSHIFT, FILLC, OUT);
        } else {
            SHIFTL(IN, -NSHIFT, FILLC, OUT);
        }
    } else {
        SETMSG(b"Shift direction (#) must be L, l, R, or r.", ctx);
        ERRCH(b"#", DIR, ctx);

        SIGERR(b"SPICE(ILLEGSHIFTDIR)", ctx)?;
    }

    CHKOUT(b"SHIFTC", ctx)?;
    Ok(())
}
