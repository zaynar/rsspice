//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const IRETRN: i32 = 3;

/// Immediate Return Indicator
///
/// Return .TRUE. if SPICELIB routines should return immediately upon
/// entry.
///
/// # Required Reading
///
/// * [ERROR](crate::required_reading::error)
///
/// # Brief I/O
///
/// ```text
///  The function returns the value .TRUE. if and only if SPICELIB
///  routines should return immediately upon entry.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the value .TRUE. if and only if SPICELIB
///  routines should return immediately upon entry. The criterion
///  for this is that the error response action is set to
///  'RETURN', and an error condition exists.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  This routine does not detect any errors.
///
///      However, this routine is part of the SPICELIB error
///      handling mechanism.
/// ```
///
/// # Particulars
///
/// ```text
///  Please read the "required reading" first!
///
///  This routine can be referenced in non-toolkit code; in
///  fact, its use is encouraged. Its purpose is to signal
///  to the routine calling it that the caller should
///  return immediately. The reference to RETURN should
///  be the first executable line of the calling program.
///
///  In 'RETURN' mode, SPICELIB routines
///  that have external references, or that can
///  detect errors, return immediately upon entry when an
///  error condition exists. They use RETURN to determine
///  when these conditions are met. Non--toolkit routines
///  can do the same.
///
///  Additionally, when an error is signaled in 'RETURN' mode,
///  no further errors can be signaled until the error condition
///  is reset by a call to RESET. Calls to SIGERR simply have
///  no effect. Therefore, the error messages set in response
///  to the FIRST error that was detected will be saved until
///  RESET is called. These messages can be retrieved by
///  calls to GETMSG.
///
///  There are a number of advantages to using this mechanism.
///  First, the likelihood of an error resulting in crash
///  in a different routine is greatly reduced. Second,
///  a program does not have to test the error status
///  (using a reference to FAILED) after each call to a toolkit
///  routine, but rather can make one test of status at the end
///  of a series of calls. See "Examples" below.
///
///  See the subroutine ERRACT for definitions of the error action
///  codes.
/// ```
///
/// # Examples
///
/// ```text
///  1. In this example, we show how to place a reference
///      to RETURN in your code:
///
///      C
///      C     No executable lines precede this one.
///      C
///      C     Test whether to return before doing
///      C     anything else.
///      C
///
///            IF ( RETURN() )  RETURN
///
///
///            [ rest of code goes here]
///
///                      .
///                      .
///                      .
///
///
///  2. Here's how one might code a sequence of calls
///      to routines with code that follows the pattern
///      given in example #1 above:
///
///                     .
///                     .
///                     .
///
///            [ code may go here ]
///
///      C
///      C     We call routines A, B, and C;  then we
///      C     test for errors, using the SPICELIB error
///      C     status indicator, FAILED:
///      C
///
///            CALL  A
///            CALL  B
///            CALL  C
///
///            IF ( FAILED() ) THEN
///
///      C
///      C        If we're here, an error occurred. The
///      C        error might have been detected by A, B, C,
///      C        or by a routine called by one of them.
///      C        Get the explanation of the short error message
///      C        and output it using the routine, USER_OUT
///      C        [USER_OUT is a fictitious routine]:
///      C
///
///               CALL GETMSG ( 'EXPLAIN', MSG )
///
///               CALL USER_OUT ( MSG )
///
///            END IF
///
///            [ rest of code goes here ]
///
///                       .
///                       .
///                       .
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine has no effect unless the error action is
///      'RETURN'!
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
/// -    SPICELIB Version 2.1.1, 26-OCT-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.1.0, 04-APR-2014 (NJB)
///
///         Re-organized code to improve efficiency in the non-error
///         case.
///
/// -    SPICELIB Version 2.0.0, 22-APR-1996 (KRG)
///
///         This subroutine has been modified in an attempt to improve
///         the general performance of the SPICELIB error handling
///         mechanism. The specific modification has been to change the
///         type of error action from a short character string to an
///         integer. This change is backwardly incompatible because the
///         type has changed.
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
/// -    SPICELIB Version 2.0.0, 22-APR-1996 (KRG)
///
///         This subroutine has been modified in an attempt to improve
///         the general performance of the SPICELIB error handling
///         mechanism. The specific modification has been to change the
///         type of error action from a short character string to an
///         integer. This change is backwardly incompatible because the
///         type has changed.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    Beta Version 1.1.0, 17-FEB-1989 (NJB)
///
///         Added parentheses to the declaration of RETURN.
/// ```
pub fn return_(ctx: &mut SpiceContext) -> bool {
    let ret = RETURN(ctx.raw_context());
    ret
}

//$Procedure RETURN ( Immediate Return Indicator )
pub fn RETURN(ctx: &mut Context) -> bool {
    let mut RETURN: bool = false;
    let mut ACTION: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //
    // Define the mnemonic for the return action.
    //

    //
    // Local Variables
    //

    //
    // Immediate return is indicated only in 'RETURN' mode,
    // when an error condition is in effect:
    //
    if !FAILED(ctx) {
        RETURN = false;
        return RETURN;
    }

    //
    // At this point, we know a SPICE error condition exists.
    //
    GETACT(&mut ACTION, ctx);

    RETURN = (ACTION == IRETRN);

    RETURN
}
