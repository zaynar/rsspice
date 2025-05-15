//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const IDEFLT: i32 = 5;

struct SaveVars {
    SAVACT: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SAVACT: i32 = 0;

        SAVACT = IDEFLT;

        Self { SAVACT }
    }
}

/// Store Error Response Action
///
/// Store the error response action.
///
/// PUTACT is a low-level data structure access routine.
/// DO NOT CALL THIS ROUTINE. USE ERRACT, NOT PUTACT, TO SET THE
/// CURRENT ERROR RESPONSE ACTION.
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
///  ACTION     I   The integer code for the error response action.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ACTION   is the new integer code for the error response action.
///           This code is saved for use by the error handling
///           system.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
/// ```
///
/// # Particulars
///
/// ```text
///  DO NOT CALL THIS ROUTINE.
///
///  This is a data structure access routine for the SPICELIB
///  error response action. This routine should be used for
///  no other purpose. In particular, it should not be used
///  by non-SPICELIB routines to set up an error response;
///  use ERRACT for that.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  DO NOT CALL THIS ROUTINE.
///
///  2)  Calls to this routine by routines other than the SPICELIB
///      error handling routines may interfere with error processing.
///
///  3)  See the subroutine ERRACT for the definitions of the error
///      action codes.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 27-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added
///         $Index_Entries entry.
///
/// -    SPICELIB Version 2.0.0, 22-APR-1996 (KRG)
///
///         This subroutine has been modified in an attempt to improve
///         the general performance of the SPICELIB error handling
///         mechanism. The specific modification has been to change the
///         type of the saved error action from a short character string
///         to an integer. This change is backwardly incompatible
///         because the type of the input argument has changed. This
///         should pose no difficulties because it is a private subroutine
///         used by the error handling system, and hence isolated from
///         direct use.
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
/// -     SPICELIB Version 2.0.0, 22-APR-1996 (KRG)
///
///          This subroutine has been modified in an attempt to improve
///          the general performance of the SPICELIB error handling
///          mechanism. The specific modification has been to change the
///          type of the saved error action from a short character string
///          to an integer. This change is backwardly incompatible
///          because the type of the input argument has changed. This
///          should pose no difficulties because it is a private subroutine
///          used by the error handling system, and hence isolated from
///          direct use.
///
/// -     SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///          Comment section for permuted index source lines was added
///          following the header.
///
/// -     Beta Version 1.0.1, 08-FEB-1989 (NJB)
///
///          Warnings added to discourage use of this routine in
///          non-error-handling code.
/// ```
pub fn putact(ctx: &mut SpiceContext, action: i32) {
    PUTACT(action, ctx.raw_context());
}

//$Procedure PUTACT ( Store Error Response Action )
pub fn PUTACT(ACTION: i32, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Local Parameters:
    //
    // Define the mnemonic for the default error action.
    //
    //
    // Local Variables:
    //
    // The current error response action:
    //

    //
    // Initial values:
    //

    //
    // Executable Code:
    //

    save.SAVACT = ACTION;
}

/// Get Error Response Action
///
/// Return the value of the current error response action.
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
///  ACTION     O   The integer code for the error response action.
/// ```
///
/// # Detailed Output
///
/// ```text
///  ACTION   is the integer code for the current error response
///           action. See the ERRACT subroutine and the "required
///           reading" file for a detailed discussion of error
///           response actions.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  However, this routine is part of the SPICELIB error
///      handling mechanism.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 03-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added
///         $Index_Entries entry.
///
/// -    SPICELIB Version 2.0.0, 22-APR-1996 (KRG)
///
///         This subroutine has been modified in an attempt to improve
///         the general performance of the SPICELIB error handling
///         mechanism. The specific modification has been to change the
///         type of the saved error action from a short character string
///         to an integer. This change is backwardly incompatible
///         because the type of the input argument has changed. This
///         should pose no difficulties because it is a private subroutine
///         used by the error handling system, and hence isolated from
///         direct use.
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
/// -     SPICELIB Version 2.0.0, 22-APR-1996 (KRG)
///
///          This subroutine has been modified in an attempt to improve
///          the general performance of the SPICELIB error handling
///          mechanism. The specific modification has been to change the
///          type of the saved error action from a short character string
///          to an integer. This change is backwardly incompatible
///          because the type of the input argument has changed. This
///          should pose no difficulties because it is a private subroutine
///          used by the error handling system, and hence isolated from
///          direct use.
///
/// -     SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///          Comment section for permuted index source lines was added
///          following the header.
/// ```
pub fn getact(ctx: &mut SpiceContext, action: &mut i32) {
    GETACT(action, ctx.raw_context());
}

//$Procedure GETACT ( Get Error Response Action )
pub fn GETACT(ACTION: &mut i32, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Executable Code:
    //

    //
    // Grab saved error response action:
    //

    *ACTION = save.SAVACT;
}
