//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Replace one substring with another
///
/// Replace the substring (LEFT:RIGHT) with a string of any length.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  IN         I   Input string.
///  LEFT,
///  RIGHT      I   Ends of substring to be replaced.
///  STRING     I   Replacement string.
///  OUT        O   Resulting string.
/// ```
///
/// # Detailed Input
///
/// ```text
///  IN       is an arbitrary character string.
///
///  LEFT,
///  RIGHT    are the ends of the substring to be replaced.
///           Legitimate substrings satisfy the following
///           conditions
///
///               RIGHT > LEFT - 2
///               LEFT  > 1
///               RIGHT < LEN(STRING) + 1
///
///           This allows users to refer to zero-length substrings
///           (null substrings) of IN.
///
///  STRING   is the replacement string. Essentially, the
///           substring (LEFT:RIGHT) is removed from the
///           input string, and STRING is inserted at the
///           point of removal.
/// ```
///
/// # Detailed Output
///
/// ```text
///  OUT      is the resulting string. OUT may overwrite IN.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If RIGHT is one less than LEFT, the substring to
///      replace will be the null substring. In this case,
///      STRING will be inserted between IN(:RIGHT) and IN(LEFT:).
///
///  2)  If LEFT is smaller than one, the error SPICE(BEFOREBEGSTR)
///      is signaled.
///
///  3)  If RIGHT is greater than the length of the input string,
///      the error SPICE(PASTENDSTR) is signaled.
///
///  4)  If RIGHT is less than LEFT-1, the error SPICE(BADSUBSTR)
///      is signaled.
///
///  5)  Whenever the output string is too small to hold the result,
///      the result is truncated on the right.
/// ```
///
/// # Particulars
///
/// ```text
///  Ideally, replacement could be done with simple concatenation,
///
///     OUT = IN(1:LEFT-1) // STRING // IN(RIGHT+1: )
///
///  but the Fortran 77 standard makes this illegal for strings of
///  unknown length.
/// ```
///
/// # Examples
///
/// ```text
///  A typical use for this routine might be to replace all
///  occurrences of one word in a string with another word.
///  For example, the following code fragment replaces every
///  occurrence of the word 'AND' with the word 'OR' in the
///  character string LINE.
///
///     LEFT = WDINDX ( LINE, 'AND' )
///
///     DO WHILE ( LEFT .NE. 0 )
///        CALL   REPSUB ( LINE, LEFT, LEFT+2, 'OR', LINE )
///        LEFT = WDINDX ( LINE, 'AND' )
///     END DO
///
///  This routine can also be used to insert substring between
///  two characters. Consider the string:
///
///      IN   = 'The defendant,, was found innocent.'
///
///  to insert ' Imelda Marcos' between the first and second commas
///  determine the location of the pair ',,'
///
///     RIGHT = POS ( IN, ',,', 1 )
///     LEFT  = RIGHT + 1
///
///  then
///
///     CALL REPSUB ( IN, LEFT, RIGHT, ' Imelda Marcos', OUT )
///
///  The output (OUT) will have the value:
///
///     'The defendant, Imelda Marcos, was found innocent.'
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The memory used by STRING and OUT must be disjoint. The memory
///      used by IN and OUT must be identical or disjoint.
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
/// -    SPICELIB Version 1.1.0, 20-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.2, 17-JUN-1999 (WLT)
///
///         Fixed example code fragment.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 24-AUG-1990 (WLT) (IMU)
/// ```
pub fn repsub(
    ctx: &mut SpiceContext,
    in_: &str,
    left: i32,
    right: i32,
    string: &str,
    out: &mut str,
) -> crate::Result<()> {
    REPSUB(
        in_.as_bytes(),
        left,
        right,
        string.as_bytes(),
        fstr::StrBytes::new(out).as_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure REPSUB ( Replace one substring with another )
pub fn REPSUB(
    IN: &[u8],
    LEFT: i32,
    RIGHT: i32,
    STRING: &[u8],
    OUT: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut INLEN: i32 = 0;
    let mut STRLEN: i32 = 0;
    let mut OUTLEN: i32 = 0;
    let mut REMAIN: i32 = 0;
    let mut USE = StackArray::<i32, 3>::new(1..=3);
    let mut END: i32 = 0;
    let mut NEXT: i32 = 0;

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
        CHKIN(b"REPSUB", ctx)?;
    }

    //
    // Get the lengths of all the strings involved in this transaction.
    //
    INLEN = intrinsics::LEN(IN);
    STRLEN = intrinsics::LEN(STRING);
    OUTLEN = intrinsics::LEN(OUT);

    //
    // Reject bad inputs.
    //
    if (LEFT < 1) {
        SETMSG(b"REPSUB error: LEFT (#) must not be less than 1.", ctx);

        ERRINT(b"#", LEFT, ctx);
        SIGERR(b"SPICE(BEFOREBEGSTR)", ctx)?;
        CHKOUT(b"REPSUB", ctx)?;
        return Ok(());
    } else if (RIGHT > INLEN) {
        SETMSG(
            b"REPSUB error: RIGHT (#) must not exceed length of IN (#).",
            ctx,
        );
        ERRINT(b"#", RIGHT, ctx);
        ERRINT(b"#", INLEN, ctx);
        SIGERR(b"SPICE(PASTENDSTR)", ctx)?;
        CHKOUT(b"REPSUB", ctx)?;
        return Ok(());
    } else if (RIGHT < (LEFT - 1)) {
        SETMSG(
            b"REPSUB error: LEFT (#) must not exceed RIGHT+1 (# + 1). ",
            ctx,
        );
        ERRINT(b"#", LEFT, ctx);
        ERRINT(b"#", RIGHT, ctx);
        SIGERR(b"SPICE(BADSUBSTR)", ctx)?;
        CHKOUT(b"REPSUB", ctx)?;
        return Ok(());
    }

    //
    // Consider three separate sections:
    //
    //    1) The front of the original string.
    //
    //    2) The replacement string.
    //
    //    3) The end of the original string.
    //
    // Determine how much of each section to use in the output string.
    // REMAIN is the number of characters that will fit in the output
    // string.
    //
    REMAIN = OUTLEN;
    USE[1] = intrinsics::MIN0(&[REMAIN, (LEFT - 1)]);

    REMAIN = (REMAIN - USE[1]);
    USE[2] = intrinsics::MIN0(&[REMAIN, STRLEN]);

    REMAIN = (REMAIN - USE[2]);
    USE[3] = intrinsics::MIN0(&[REMAIN, (INLEN - RIGHT)]);

    //
    // Move the third section first. It gets moved back to front
    // or front to back, depending on whether the replacement string
    // is longer than the original substring. The main thing is to
    // avoid overwriting characters that have yet to be moved.
    //
    END = SUMAI(USE.as_slice(), 3);

    if ((LEFT + STRLEN) > RIGHT) {
        NEXT = END;

        for I in intrinsics::range(USE[3], 1, -1) {
            fstr::assign(
                fstr::substr_mut(OUT, NEXT..=NEXT),
                fstr::substr(IN, (RIGHT + I)..=(RIGHT + I)),
            );
            NEXT = (NEXT - 1);
        }
    } else {
        NEXT = (LEFT + STRLEN);

        for I in 1..=USE[3] {
            fstr::assign(
                fstr::substr_mut(OUT, NEXT..=NEXT),
                fstr::substr(IN, (RIGHT + I)..=(RIGHT + I)),
            );
            NEXT = (NEXT + 1);
        }
    }

    //
    // The first two sections can be moved directly to the front of
    // the output string.
    //
    NEXT = 1;

    for I in 1..=USE[1] {
        fstr::assign(fstr::substr_mut(OUT, NEXT..=NEXT), fstr::substr(IN, I..=I));
        NEXT = (NEXT + 1);
    }

    for I in 1..=USE[2] {
        fstr::assign(
            fstr::substr_mut(OUT, NEXT..=NEXT),
            fstr::substr(STRING, I..=I),
        );
        NEXT = (NEXT + 1);
    }

    //
    // Pad with blanks, if the output string was not filled.
    //
    if (END < OUTLEN) {
        fstr::assign(fstr::substr_mut(OUT, (END + 1)..), b" ");
    }

    CHKOUT(b"REPSUB", ctx)?;
    Ok(())
}
