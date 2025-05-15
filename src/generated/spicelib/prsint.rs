//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const MSGLEN: i32 = 320;

/// Parse integer with error checking
///
/// Parse a string as an integer, encapsulating error handling.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  STRING     I   String representing a numeric value.
///  INTVAL     O   Integer value obtained by parsing STRING.
/// ```
///
/// # Detailed Input
///
/// ```text
///  STRING   is a string representing a numeric value. Commas and
///           spaces may be used in this string for ease of reading
///           and writing the number. They are treated as
///           insignificant but non-error-producing characters.
///
///           For exponential representation any of the characters
///           'E','D','e','d' may be used.
///
///           The following are legitimate numeric expressions
///
///              +12.2 e-1
///              -3. 1415 9276
///              1e6
///              E8
///
///           The program also recognizes the following  mnemonics
///
///              'PI',  'pi',  'Pi',  'pI'
///              '+PI', '+pi', '+Pi', '+pI'
///              '-PI', '-pi', '-Pi', '-pI'
///
///           and returns the value ( + OR - ) 3 as appropriate.
/// ```
///
/// # Detailed Output
///
/// ```text
///  INTVAL   is the integer obtained by parsing STRING. If an error is
///           encountered, INTVAL is not changed from whatever the
///           input value was. If the input string has a fractional
///           part, the fractional part will be truncated. Thus
///           3.18 is interpreted as 3. -4.98 is interpreted as -4.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input string cannot be parsed or if the string
///      represents a number that is outside the range of
///      representable integers, as defined by INTMIN and INTMAX, the
///      error SPICE(NOTANINTEGER) is signaled. The value of INTVAL is
///      not changed from whatever the input value was.
/// ```
///
/// # Particulars
///
/// ```text
///  The purpose of this routine is to enable safe parsing of numeric
///  values into an INTEGER variable without the necessity of in-line
///  error checking.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Parse into an INTEGER variable a set of strings representing
///     numeric values.
///
///
///     Example code begins here.
///
///
///           PROGRAM PRSINT_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               SETSIZ
///           PARAMETER           ( SETSIZ = 10 )
///
///           INTEGER               STRLEN
///           PARAMETER           ( STRLEN = 11 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(STRLEN)    STRVAL ( SETSIZ )
///
///           INTEGER               I
///           INTEGER               INTVAL
///
///     C
///     C     Initialize the array of strings.
///     C
///           DATA                  STRVAL / '100,000,000',
///          .                               ' -2 690 192',
///          .                               '  +12.2 e-1',
///          .                               '-3. 141 592',
///          .                               '      1.2e8',
///          .                               '         E6',
///          .                               '         Pi',
///          .                               '        -PI',
///          .                               '-2147483648',
///          .                               ' 2147483647' /
///
///     C
///     C     Parse each string into an INTEGER variable.
///     C
///           WRITE(*,'(A)') '   STRVAL       INTVAL'
///           WRITE(*,'(A)') '-----------  ------------'
///           DO I = 1, SETSIZ
///
///              CALL PRSINT ( STRVAL(I), INTVAL )
///
///              WRITE(*,'(A11,2X,I12)') STRVAL(I), INTVAL
///
///           END DO
///
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///       STRVAL        INTVAL
///     -----------  ------------
///     100,000,000     100000000
///      -2 690 192      -2690192
///       +12.2 e-1             1
///     -3. 141 592            -3
///           1.2e8     120000000
///              E6       1000000
///              Pi             3
///             -PI            -3
///     -2147483648   -2147483648
///      2147483647    2147483647
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 04-JUL-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example.
///
///         Updated the header to properly describe its input, output,
///         exceptions and particulars.
///
/// -    SPICELIB Version 1.0.0, 22-JUL-1997 (NJB)
/// ```
pub fn prsint(ctx: &mut SpiceContext, string: &str, intval: &mut i32) -> crate::Result<()> {
    PRSINT(string.as_bytes(), intval, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure PRSINT   ( Parse integer with error checking )
pub fn PRSINT(STRING: &[u8], INTVAL: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ERRMSG = [b' '; MSGLEN as usize];
    let mut PTR: i32 = 0;

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //
    NPARSI(STRING, INTVAL, &mut ERRMSG, &mut PTR, ctx);

    if fstr::ne(&ERRMSG, b" ") {
        CHKIN(b"PRSINT", ctx)?;
        SETMSG(&ERRMSG, ctx);
        SIGERR(b"SPICE(NOTANINTEGER)", ctx)?;
        CHKOUT(b"PRSINT", ctx)?;
        return Ok(());
    }

    Ok(())
}
