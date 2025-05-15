//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const MINLEN: i32 = 5;
const CHBASE: i32 = 128;

/// Encode a character string, portably
///
/// Encode a nonnegative integer number into a character string,
/// portably, using 128 as the base for encoding.
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
///           base 128 representation.
///
///           Let L be the declared length of STRING, and let
///           NUMBER be given by
///
///                               0           1                 L-1
///              NUMBER = a    128  + a    128  + ... + a    128
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
///                                                    MINLEN
///  3)  If the number to be encoded is larger than 128       - 1,
///      the error SPICE(OUTOFRANGE) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is identical to ENCHAR, except that this routine
///  does not use the machine-dependent encoding base returned by
///  the SPICELIB routine CHBASE. Instead, the base 128 is used.
///  This base is expected to work on all systems supporting ASCII
///  encoding of characters.
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
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
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
/// -    SPICELIB Version 1.0.0, 19-DEC-1995 (NJB) (WLT)
/// ```
pub fn prtenc(ctx: &mut SpiceContext, number: i32, string: &mut str) -> crate::Result<()> {
    PRTENC(
        number,
        fstr::StrBytes::new(string).as_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure PRTENC ( Encode a character string, portably )
pub fn PRTENC(NUMBER: i32, STRING: &mut [u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut BASE: i32 = 0;
    let mut NUM: i32 = 0;
    let mut REMAIN: i32 = 0;

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if (intrinsics::LEN(STRING) < MINLEN) {
        CHKIN(b"PRTENC", ctx)?;
        SIGERR(b"SPICE(INSUFFLEN)", ctx)?;
        CHKOUT(b"PRTENC", ctx)?;
        return Ok(());
    } else if (NUMBER < 0) {
        CHKIN(b"PRTENC", ctx)?;
        SIGERR(b"SPICE(OUTOFRANGE)", ctx)?;
        CHKOUT(b"PRTENC", ctx)?;
        return Ok(());
    }

    //
    // Generate the digits from right to left.
    //
    BASE = CHBASE;
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
        CHKIN(b"PRTENC", ctx)?;
        SIGERR(b"SPICE(OUTOFRANGE)", ctx)?;
        CHKOUT(b"PRTENC", ctx)?;
    }

    Ok(())
}

/// Decode a character string
///
/// Decode a character string encoded by PRTENC.
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
///  STRING   is a character string previously encoded by PRTENC.
///           This contains an integer in base 128 notation,
///           where 128 is a function of the size of the
///           available character set. See PRTENC for details
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
///  PRTDEC is the inverse of PRTENC. In the example below,
///
///        CALL PRTENC (      I, STRING )
///        CALL PRTDEC ( STRING,      J )
///
///        IF ( I .EQ. J ) THEN
///         .
///         .
///        END IF
///
///  the logical test (I .EQ. J) is always true.
///
///  This routine is identical to DECHAR, except that this routine
///  does not use the machine-dependent encoding base returned by
///  the SPICELIB routine CHBASE. Instead, the base 128 is used.
///  This base is expected to work on all systems supporting ASCII
///  encoding of characters.
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
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
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
/// -    SPICELIB Version 1.0.0, 19-DEC-1995 (NJB) (WLT)
/// ```
pub fn prtdec(ctx: &mut SpiceContext, string: &str, number: &mut i32) -> crate::Result<()> {
    PRTDEC(string.as_bytes(), number, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure PRTDEC ( Decode a character string )
pub fn PRTDEC(STRING: &[u8], NUMBER: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut BASE: i32 = 0;

    if (intrinsics::LEN(STRING) < MINLEN) {
        CHKIN(b"PRTDEC", ctx)?;
        SIGERR(b"SPICE(INSUFFLEN)", ctx)?;
        CHKOUT(b"PRTDEC", ctx)?;
        return Ok(());
    }

    //
    // Sum the products of the 'digits' and the corresponding powers
    // of NDCHAR, just like any other base conversion.
    //
    BASE = CHBASE;
    *NUMBER = 0;

    for I in 1..=MINLEN {
        *NUMBER = ((BASE * *NUMBER) + intrinsics::ICHAR(fstr::substr(STRING, I..=I)));
    }

    Ok(())
}
