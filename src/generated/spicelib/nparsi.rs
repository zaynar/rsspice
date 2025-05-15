//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

struct SaveVars {
    XMNINT: f64,
    XMXINT: f64,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut XMNINT: f64 = 0.0;
        let mut XMXINT: f64 = 0.0;
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            XMNINT,
            XMXINT,
            FIRST,
        }
    }
}

/// Integer parsing of a character string
///
/// Parse a character string that represents a number and return
/// the FORTRAN-truncated integer value.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  ---------------------------------------------------
///  STRING     I   Character string representing a numeric value.
///  N          O   Translated integer value of STRING.
///  ERROR      O   Message indicating what errors have occurred.
///  PNTER      O   Position in character string where an error
///                 occurred.
/// ```
///
/// # Detailed Input
///
/// ```text
///  STRING   is a character string that represents a numeric value.
///           Commas and spaces may be used in this string for
///           ease of reading and writing the number. They
///           are treated as insignificant but non-error-producing
///           characters.
///
///           For exponential representation and of the characters
///           'E','D','e','d' may be used.
///
///           The following are legitimate numeric expressions
///
///            +12.2 e-1
///            -3. 1415 9276
///            1e12
///            E10
///
///           The program also recognizes the following  mnemonics
///           'PI', 'pi', 'Pi', 'pI'
///           '+PI', '+pi', '+Pi', '+pI'
///           '-PI', '-pi', '-Pi', '-pI'
///           and returns the value ( + OR - ) 3 as appropriate.
/// ```
///
/// # Detailed Output
///
/// ```text
///  N        integer parsed value of input string  ( with
///           the implied limits on precision).  If an error is
///           encountered, N is not changed from whatever the
///           input value was. If the input string has a fractional
///           part, the fractional part will be truncated. Thus
///           3.18 is interpreted as 3.  -4.98 is interpreted as -4.
///
///  ERROR    this is a message indicating that the string could
///           not be parsed due to ambiguous use of symbols or
///           due to a string representing a number too large for
///           VAX double precision or integer variables. If no
///           error occurred, ERROR is blank.
///
///           In particular, blank strings, or strings that do not
///           contain either a digit or exponent character will
///           be regarded as errors.
///
///  PNTER    this indicates which character was being used when
///           the error occurred. If no error occurred, PNTER is 0.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If the string is non-numeric, PNTER indicates the location in
///      the string where the error occurred, and ERROR contains a
///      descriptive error message.
///
///  2)  If the string is blank, ERROR is returned with a message
///      indicating the problem and PNTER will have a non-zero value.
///
///  3)  If the string represents a number that is outside the range of
///      representable integers, as defined by INTMIN() and INTMAX(),
///      ERROR is returned with a message and PNTER is set to the value
///      1, as the entire numeric string is at fault.
/// ```
///
/// # Particulars
///
/// ```text
///  Basically, all this routine does is pass the input string to
///  NPARSD which does the parsing in double precision. If nothing
///  goes wrong in the double precision parsing of the number, the
///  returned value is checked to determine whether or not it will fit
///  into a VAX integer. If it doesn't, an error message is returned.
/// ```
///
/// # Examples
///
/// ```text
///  Let   LINE = 'DELTA_T_A       =   32'
///
///  The following code fragment parses the line and obtains the
///  integer value.
///
///
///     CALL NEXTWD ( LINE,  FIRST,  REST )
///     CALL NEXTWD ( REST, SECOND,  REST )
///     CALL NEXTWD ( REST,  THIRD,  REST )
///
///     CALL NPARSI (  THIRD,  VALUE, ERROR, POINTR )
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.2.0, 12-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section.
///
/// -    SPICELIB Version 2.1.0, 29-APR-1996 (KRG)
///
///         This subroutine was modified to return a non-zero value of
///         PNTER when the value returned by NPARSD is not a representable
///         integer, as defined by INTMIN() and INTMAX(). The value
///         returned is one (1), since the entire input string was not
///         correct.
///
///         The test for an error from NPARSD was also changed. It now
///         uses the integer PNTER returned from NPARSD rather then the
///         character string ERROR. This should pose no problems because
///         PNTER is non-zero if and only if there was an error and an
///         error message was assigned to ERROR.
///
///         Some extra, and unnecessary, assignments were deleted. The
///         assignments were:
///
///            X = DBLE ( N )
///
///            ERROR = ' '
///
///         which converted the input argument into a double before
///         calling NPARSD with X and initialized the error message
///         to be blank. NPARSD sets the value for X, ERROR, and PNTER
///         unless an error occurs, in which case X is not changed.
///         So, it is not necessary to initialize ERROR, PNTER, or X.
///
///         Finally, the values of INTMIN and INTMAX are only set on the
///         first call to the routine. They are now SAVEd.
///
/// -    SPICELIB Version 2.0.0, 15-OCT-1992 (WLT)
///
///         The abstract of this routine was modified to reflect what
///         the routine actually does---truncate the value to an
///         integer.
///
///         In addition, a blank string is no longer considered to be
///         valid input.
///
///         Finally the instances of DFLOAT in the previous version were
///         replaced by the standard intrinsic function DBLE and the
///         function DINT was replaced by IDINT in one place to make types
///         match up on both sides of an assignment.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (HAN)
/// ```
pub fn nparsi(ctx: &mut SpiceContext, string: &str, n: &mut i32, error: &mut str, pnter: &mut i32) {
    NPARSI(
        string.as_bytes(),
        n,
        fstr::StrBytes::new(error).as_mut(),
        pnter,
        ctx.raw_context(),
    );
}

//$Procedure NPARSI ( Integer parsing of a character string)
pub fn NPARSI(STRING: &[u8], N: &mut i32, ERROR: &mut [u8], PNTER: &mut i32, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut X: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local Variables
    //

    //
    // Saved Variables
    //
    //
    // Initial values
    //
    //
    // If this is the first time NPARSI has been called, initialize
    // bounds for the range of integers.
    //
    if save.FIRST {
        save.FIRST = false;
        save.XMXINT = (INTMAX() as f64);
        save.XMNINT = (INTMIN() as f64);
    }
    //
    // NPARSD will define ERROR and PNTER if there is an error,
    // so we do not need to initialize them here.
    //
    NPARSD(STRING, &mut X, ERROR, PNTER, ctx);

    if (*PNTER == 0) {
        if ((f64::trunc(X) < save.XMNINT) || (f64::trunc(X) > save.XMXINT)) {
            *PNTER = 1;
            fstr::assign(
                ERROR,
                b"NPARSI: Value entered is beyond the bounds of representable integers.",
            );
        } else {
            *N = (X as i32);
        }
    }
}
