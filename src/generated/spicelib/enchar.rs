//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const MINLEN: i32 = 5;

/// Encode a character string
///
/// Encode a nonnegative integer number into a character string
/// as the expansion of the number in base CHBASE (a function of
/// the size of the available character set).
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NUMBER     I   Number to be encoded.
///  STRING     O   Encoded string.
///  MINLEN     P   Minimum length of string.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NUMBER   is an arbitrary nonnegative integer.
/// ```
///
/// # Detailed Output
///
/// ```text
///  STRING   is the character string implied by the ASCII
///           interpretation of NUMBER when converted to its
///           base CHBASE representation.
///
///           Let L be the declared length of STRING, and let
///           NUMBER be given by
///
///                               0           1                 L-1
///              NUMBER = a CHBASE  + a CHBASE  + ... + a CHBASE
///                        1           2                 L
///
///           Then
///
///              STRING(i:i) = CHAR(a )   for i = 1, L
///                                  i
///
///           Note that, just as for any other "numbers",
///           the "digits" in STRING are arranged from right
///           to left in order of increasing significance.
///           The string is, in effect, "padded with nulls"
///           on the left.
/// ```
///
/// # Parameters
///
/// ```text
///  MINLEN   is the minimum length of a string into which a
///           number may be encoded. In order to avoid padding
///           long strings with hundreds, possibly thousands
///           of null characters, only the first MINLEN characters
///           of the string are actually used. Note that this
///           also allows the encoded number to be preserved
///           during assignments,
///
///              STR1 = STR2
///
///           so long as both strings are of length MINLEN or
///           greater.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the length of the output string is less than MINLEN,
///      the error SPICE(INSUFFLEN) is signaled.
///
///  2)  If the number to be encoded is negative, the error
///      SPICE(OUTOFRANGE) is signaled.
///
///                                                       MINLEN
///  3)  If the number to be encoded is larger than CHBASE       - 1,
///      the error SPICE(OUTOFRANGE) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  The value of CHBASE, which varies from machine to machine, is
///  returned by a constant function of the same name.
/// ```
///
/// # Examples
///
/// ```text
///  See: SCARDC, SSIZEC.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
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
/// -    SPICELIB Version 1.0.2, 31-JAN-2008 (BVS)
///
///         Changed header section title '$C Revision' to '$C Revisions'.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    Beta Version 2.0.0, 13-JAN-1989 (IMU)
///
///         Only the first MINLEN characters of the string are now
///         used to encode the value. Also, negative values are now
///         treated as errors.
/// ```
pub fn enchar(ctx: &mut SpiceContext, number: i32, string: &mut str) -> crate::Result<()> {
    ENCHAR(
        number,
        fstr::StrBytes::new(string).as_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure ENCHAR ( Encode a character string )
pub fn ENCHAR(NUMBER: i32, STRING: &mut [u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut BASE: i32 = 0;
    let mut NUM: i32 = 0;
    let mut REMAIN: i32 = 0;

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
    } else if (intrinsics::LEN(STRING) < MINLEN) {
        CHKIN(b"ENCHAR", ctx)?;
        SIGERR(b"SPICE(INSUFFLEN)", ctx)?;
        CHKOUT(b"ENCHAR", ctx)?;
        return Ok(());
    } else if (NUMBER < 0) {
        CHKIN(b"ENCHAR", ctx)?;
        SIGERR(b"SPICE(OUTOFRANGE)", ctx)?;
        CHKOUT(b"ENCHAR", ctx)?;
        return Ok(());
    }

    //
    // Generate the digits from right to left.
    //
    BASE = CHBASE();
    NUM = NUMBER;

    for I in intrinsics::range(MINLEN, 1, -1) {
        REMAIN = intrinsics::MOD(NUM, BASE);
        fstr::assign(fstr::substr_mut(STRING, I..=I), &intrinsics::CHAR(REMAIN));
        NUM = (NUM / BASE);
    }

    //
    // More error handling.
    //
    if (NUM > 0) {
        CHKIN(b"ENCHAR", ctx)?;
        SIGERR(b"SPICE(OUTOFRANGE)", ctx)?;
        CHKOUT(b"ENCHAR", ctx)?;
    }

    Ok(())
}

/// Decode a character string
///
/// Decode a character string encoded by ENCHAR.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  STRING     I   Encoded character string.
///  NUMBER     O   Decoded number.
/// ```
///
/// # Detailed Input
///
/// ```text
///  STRING   is a character string previously encoded by ENCHAR.
///           This contains an integer in base CHBASE notation,
///           where CHBASE is a function of the size of the
///           available character set. See ENCHAR for details
///           about the format of STRING.
/// ```
///
/// # Detailed Output
///
/// ```text
///  NUMBER   is the integer encoded in the input string.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the length of the input string is less than MINLEN,
///      the error SPICE(INSUFFLEN) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  DECHAR is the inverse of ENCHAR. In the example below,
///
///        CALL ENCHAR (      I, STRING )
///        CALL DECHAR ( STRING,      J )
///
///        IF ( I .EQ. J ) THEN
///         .
///         .
///        END IF
///
///  the logical test (I .EQ. J) is always true.
/// ```
///
/// # Examples
///
/// ```text
///  See: CARDC, SIZEC.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
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
/// -    SPICELIB Version 1.0.2, 31-JAN-2008 (BVS)
///
///         Changed header section title '$C Revision' to '$C Revisions'.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    Beta Version 2.0.0, 13-JAN-1989 (IMU)
///
///         Changed to reflect changes in ENCHAR. In particular,
///         it now checks the length of the input string. It is
///         also an entry point of ENCHAR, to make sure they always
///         have the same value of MINLEN. (Also, if CHBASE is
///         changed, ENCHAR and DECHAR will always be recompiled
///         simultaneously.)
/// ```
pub fn dechar(ctx: &mut SpiceContext, string: &str, number: &mut i32) -> crate::Result<()> {
    DECHAR(string.as_bytes(), number, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DECHAR ( Decode a character string )
pub fn DECHAR(STRING: &[u8], NUMBER: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut BASE: i32 = 0;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else if (intrinsics::LEN(STRING) < MINLEN) {
        CHKIN(b"DECHAR", ctx)?;
        SIGERR(b"SPICE(INSUFFLEN)", ctx)?;
        CHKOUT(b"DECHAR", ctx)?;
        return Ok(());
    }

    //
    // Sum the products of the 'digits' and the corresponding powers
    // of NDCHAR, just like any other base conversion.
    //
    BASE = CHBASE();
    *NUMBER = 0;

    for I in 1..=MINLEN {
        *NUMBER = ((BASE * *NUMBER) + intrinsics::ICHAR(fstr::substr(STRING, I..=I)));
    }

    Ok(())
}
