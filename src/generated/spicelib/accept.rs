//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

struct SaveVars {
    SAVOK: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SAVOK: bool = false;

        SAVOK = true;

        Self { SAVOK }
    }
}

/// Accept New Long Error Message
///
/// Indicate to the SPICELIB error handling mechanism whether or not
/// a replacement or modification of the long error message can be
/// accepted.
///
/// DO NOT CALL THIS ROUTINE.
///
/// # Required Reading
///
/// * [ERROR](crate::required_reading::error)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  OK         I   Indicates whether long error msg changes are ok.
///
///  The function takes an UNSPECIFIED value on exit.
/// ```
///
/// # Detailed Input
///
/// ```text
///  OK       indicates to the error handling mechanism whether
///           replacement of or changes to the long error message
///           are to be allowed; for them to be allowed,
///           both of the following must be true:
///
///           1. No error condition exists, or the error response
///              action is not 'RETURN'.
///
///           2. The current error response mode is not 'IGNORE'.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function is assigned a value on output, but the
///  value is not meaningful.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  This routine does not detect any errors.
///
///      However, this routine is part of the SPICELIB error handling
///      mechanism.
/// ```
///
/// # Particulars
///
/// ```text
///  DO NOT CALL THIS ROUTINE.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  DO NOT CALL THIS ROUTINE.
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
/// -    SPICELIB Version 1.1.0, 26-OCT-2021 (JDR)
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
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (NJB)
/// ```
///
/// # Revisions
///
/// ```text
/// -     Beta Version 1.1.0, 13-DEC-1989 (NJB)
///
///          ACCEPT must return a value, in order to comply with the
///          Fortran standard. So, now it does. The value has no
///          meaning, as far as the specification of ACCEPT is
///          concerned.
///
/// -     Beta Version 1.0.1, 08-FEB-1989 (NJB)
///
///          Warnings added to discourage use of this routine in
///          non-error-handling code.
/// ```
pub fn accept(ctx: &mut SpiceContext, ok: bool) -> bool {
    let ret = ACCEPT(ok, ctx.raw_context());
    ret
}

//$Procedure ACCEPT ( Accept New Long Error Message )
pub fn ACCEPT(OK: bool, ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ACCEPT: bool = false;

    //
    // SPICELIB functions:
    //

    //
    // Local Variables:
    //

    //
    // Initial Values:
    //

    //
    // Executable Code:
    //

    save.SAVOK = OK;

    ACCEPT = false;

    ACCEPT
}

/// Are Changes of Long Error Message Allowed?
///
/// Return .TRUE. if replacement or modification of the long error
/// message is allowed.
///
/// DO NOT CALL THIS ROUTINE.
///
/// # Required Reading
///
/// * [ERROR](crate::required_reading::error)
///
/// # Brief I/O
///
/// ```text
///  The function takes the value, .TRUE., if replacement or
///  modification of the long error message is currently allowed.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function takes the value, .TRUE., if replacement of or
///  changes to the long error message are to be allowed; for them
///  to be allowed, both of the following must be true:
///
///  1. No error condition exists, or the error response
///     action is not 'RETURN'.
///
///  2. The current error response mode is not 'IGNORE'.
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
///  DO NOT CALL THIS ROUTINE.
///
///  Non-error handling routines should not call this routine. Such
///  routines can set the long error message using SETMSG, which
///  itself calls this routine to test whether an update is allowed.
///
///  The initial value returned by ALLOWD is .FALSE.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  DO NOT CALL THIS ROUTINE.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 26-OCT-2021 (JDR)
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
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (NJB) (HAN)
/// ```
///
/// # Revisions
///
/// ```text
/// -     Beta Version 1.1.0, 18-DEC-1989 (HAN)
///
///          Empty parentheses added to the ENTRY statement in order to
///          comply with the ANSI Fortran 77 Standard.
///
/// -     Beta Version 1.0.1, 08-FEB-1989 (NJB)
///
///          Warnings added to discourage use of this routine in
///          non-error-handling code.
/// ```
pub fn allowd(ctx: &mut SpiceContext) -> bool {
    let ret = ALLOWD(ctx.raw_context());
    ret
}

//$Procedure ALLOWD    (Are Changes of Long Error Message Allowed?)
pub fn ALLOWD(ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ALLOWD: bool = false;

    //
    // Executable Code:
    //

    ALLOWD = save.SAVOK;

    ALLOWD
}
