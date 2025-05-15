//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const MSGLEN: i32 = 320;

/// Parse d.p. number with error checking
///
/// Parse a string as a double precision number, encapsulating error
/// handling.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  STRING     I   String representing a numeric value.
///  DPVAL      O   D.p. value obtained by parsing STRING.
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
///              1e12
///              E10
///
///           The program also recognizes the following  mnemonics
///
///              'PI',  'pi',  'Pi',  'pI'
///              '+PI', '+pi', '+Pi', '+pI'
///              '-PI', '-pi', '-Pi', '-pI'
///
///           and returns the value
///
///              ( + OR - ) 3.1415 9265 3589 7932 3846 26 ...
///
///           as appropriate.
/// ```
///
/// # Detailed Output
///
/// ```text
///  DPVAL    is the double precision number obtained by parsing
///           STRING.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input string cannot be parsed  due to use of an
///      unexpected or misplaced character or due to a string
///      representing a number too large for double precision, the
///      error SPICE(NOTADPNUMBER) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  The purpose of this routine is to enable safe parsing of double
///  precision numbers without the necessity of in-line error checking.
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
///  1) Parse into a DOUBLE PRECISION variable a set of strings
///     representing numeric values.
///
///
///     Example code begins here.
///
///
///           PROGRAM PRSDP_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               SETSIZ
///           PARAMETER           ( SETSIZ = 8  )
///
///           INTEGER               STRLEN
///           PARAMETER           ( STRLEN = 11 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(STRLEN)    STRVAL ( SETSIZ )
///
///           DOUBLE PRECISION      DPVAL
///
///           INTEGER               I
///
///     C
///     C     Initialize the array of strings.
///     C
///           DATA                  STRVAL / '100,000,000',
///          .                               ' -2 690 192',
///          .                               '  +12.2 e-1',
///          .                               '-3. 141 592',
///          .                               '     1.2e12',
///          .                               '        E10',
///          .                               '         Pi',
///          .                               '        -PI' /
///
///     C
///     C     Parse each string into a DOUBLE PRECISION variable.
///     C
///           WRITE(*,'(A)') '   STRVAL               DPVAL'
///           WRITE(*,'(A)') '-----------  --------------------------'
///           DO I = 1, SETSIZ
///
///              CALL PRSDP ( STRVAL(I), DPVAL )
///
///              WRITE(*,'(A11,F28.12)') STRVAL(I), DPVAL
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
///        STRVAL               DPVAL
///     -----------  --------------------------
///     100,000,000      100000000.000000000000
///      -2 690 192       -2690192.000000000000
///       +12.2 e-1              1.220000000000
///     -3. 141 592             -3.141592000000
///          1.2e12  1200000000000.000000000000
///             E10    10000000000.000000000000
///              Pi              3.141592653590
///             -PI             -3.141592653590
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
/// -    SPICELIB Version 1.1.1, 28-MAY-2020 (JDR)
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example.
///
///         Updated the header to properly describe its input, output,
///         exceptions and particulars.
///
/// -    SPICELIB Version 1.1.0, 15-SEP-1997 (NJB)
///
///         Bug fix: output argument declaration changed from INTEGER
///         to DOUBLE PRECISION.
///
/// -    SPICELIB Version 1.0.0, 22-JUL-1997 (NJB)
/// ```
pub fn prsdp(ctx: &mut SpiceContext, string: &str, dpval: &mut f64) -> crate::Result<()> {
    PRSDP(string.as_bytes(), dpval, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure PRSDP   ( Parse d.p. number with error checking )
pub fn PRSDP(STRING: &[u8], DPVAL: &mut f64, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    NPARSD(STRING, DPVAL, &mut ERRMSG, &mut PTR, ctx);

    if fstr::ne(&ERRMSG, b" ") {
        CHKIN(b"PRSDP", ctx)?;
        SETMSG(&ERRMSG, ctx);
        SIGERR(b"SPICE(NOTADPNUMBER)", ctx)?;
        CHKOUT(b"PRSDP", ctx)?;
        return Ok(());
    }

    Ok(())
}
