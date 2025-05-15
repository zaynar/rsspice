//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Remove a substring
///
/// Remove the substring (LEFT:RIGHT) from a character string.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  IN         I   Input string.
///  LEFT       I   Position of first character to be removed.
///  RIGHT      I   Position of last character to be removed.
///  OUT        O   Output string.
/// ```
///
/// # Detailed Input
///
/// ```text
///  IN       is an input character string, from which a substring
///           is to be removed.
///
///  LEFT,
///  RIGHT    are the ends of the substring to be removed.
/// ```
///
/// # Detailed Output
///
/// ```text
///  OUT      is the output string. This is equivalent to the
///           string that would be created by the concatenation
///
///                 OUT = IN(1 : LEFT-1) // IN(RIGHT+1 : )
///
///           If the string is too long to fit into OUT, it is
///           truncated on the right.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If LEFT > RIGHT, RIGHT < 1, LEFT < 1, RIGHT > LEN(IN), or
///      LEFT > LEN(IN), the error SPICE(INVALIDINDEX) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  Move the characters, beginning with RIGHT, one at a time to the
///  positions immediately following LEFT. This has the same effect
///  as the concatenation
///
///        OUT = IN(1 : LEFT-1) // IN(RIGHT+1 : )
///
///  Because this operation is not standard for strings of length (*),
///  this routine does not use concatenation.
/// ```
///
/// # Examples
///
/// ```text
///  The following examples illustrate the use of REMSUB.
///
///  IN                 LEFT  RIGHT        OUT
///  -----------------  ----  -----        ------------------------
///  'ABCDEFGHIJ'          3      5        'ABFGHIJ'
///  'The best rabbit'     5      8        'The  rabbit'
///  'The other woman'     1      4        'other woman'
///  'An Apple a day'      2      2        'A apple a day'
///  'An Apple a day'      5      2         An error is signaled.
///  'An Apple a day'      0      0         An error is signaled.
///  'An Apple a day'     -3      3         An error is signaled.
///
///  Whenever an error has been signaled, the contents of OUT are
///  unpredictable.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
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
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (IMU) (HAN)
/// ```
///
/// # Revisions
///
/// ```text
/// -     Beta Version 2.0.0, 05-JAN-1989 (HAN)
///
///          Error handling was added to detect invalid character
///          positions. If LEFT > RIGHT, RIGHT < 1, LEFT < 1,
///          RIGHT > LEN(IN), or LEFT > LEN(IN), an error is signaled.
/// ```
pub fn remsub(
    ctx: &mut SpiceContext,
    in_: &str,
    left: i32,
    right: i32,
    out: &mut str,
) -> crate::Result<()> {
    REMSUB(
        in_.as_bytes(),
        left,
        right,
        fstr::StrBytes::new(out).as_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure REMSUB ( Remove a substring )
pub fn REMSUB(
    IN: &[u8],
    LEFT: i32,
    RIGHT: i32,
    OUT: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut INLEN: i32 = 0;
    let mut OUTLEN: i32 = 0;
    let mut L: i32 = 0;
    let mut R: i32 = 0;
    let mut I: i32 = 0;
    let mut J: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Other functions
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
        CHKIN(b"REMSUB", ctx)?;
    }

    //
    // If a character position is out of range, signal an error.
    //
    if (((((LEFT > RIGHT) || (RIGHT < 1)) || (LEFT < 1)) || (RIGHT > intrinsics::LEN(IN)))
        || (LEFT > intrinsics::LEN(IN)))
    {
        SETMSG(b"Left location was *. Right location was *.", ctx);
        ERRINT(b"*", LEFT, ctx);
        ERRINT(b"*", RIGHT, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"REMSUB", ctx)?;
        return Ok(());
    } else {
        L = LEFT;
        R = RIGHT;
    }

    //
    // How much of the input string will we use? And how big is the
    // output string?
    //
    INLEN = LASTNB(IN);
    OUTLEN = intrinsics::LEN(OUT);

    //
    // Copy the first part of the input string. (One character at a
    // time, in case this is being done in place.)
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = intrinsics::MIN0(&[(L - 1), OUTLEN]);
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(fstr::substr_mut(OUT, I..=I), fstr::substr(IN, I..=I));

            I += m3__;
        }
    }

    //
    // Now move the rest of the string over.
    //
    I = L;
    J = (R + 1);

    while ((I <= OUTLEN) && (J <= INLEN)) {
        fstr::assign(fstr::substr_mut(OUT, I..=I), fstr::substr(IN, J..=J));
        I = (I + 1);
        J = (J + 1);
    }

    //
    // Pad with blanks, if necessary.
    //
    if (I <= OUTLEN) {
        fstr::assign(fstr::substr_mut(OUT, I..), b" ");
    }

    CHKOUT(b"REMSUB", ctx)?;
    Ok(())
}
