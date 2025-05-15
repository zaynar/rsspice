//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Set Error Status
///
/// Set the SPICELIB error status.  DO NOT CALL THIS ROUTINE.
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
///  STATUS     I   Status indicator.
///
///  The function takes an UNSPECIFIED (and meaningless) value
///  on exit.
/// ```
///
/// # Detailed Input
///
/// ```text
///  STATUS   is a flag that provides the new status. When .TRUE., it
///           means that an error condition exists.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None.
///
///  This purpose of this routine is to set status; the
///  function takes an UNSPECIFIED value on exit. The
///  assigned value does not have any meaning.
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
///  This is a data structure access routine for the
///  SPICELIB status. This routine should be used for no
///  other purpose; in particular, it should not be used
///  to signal errors. Use SIGERR or FAILED for that.
///
///  This routine assigns a value to SETERR on exit.
///  However, the value is not meaningful.
/// ```
///
/// # Examples
///
/// ```text
///  None.  DON'T CALL THIS ROUTINE.
///
///  No examples. If you don't know EXACTLY what a
///  ``data structure access routine'' is, don't call
///  this routine. If you do know, you don't need an
///  example.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  DON'T CALL THIS ROUTINE.
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
/// -    SPICELIB Version 1.1.0, 26-OCT-2021 (JDR) (NJB)
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
/// -     Beta Version 1.0.1, 08-FEB-1989 (NJB)
///
///          Warnings added to discourage use of this routine in
///          non-error-handling code.
/// ```
pub fn seterr(ctx: &mut SpiceContext, status: bool) -> bool {
    let ret = SETERR(status, ctx.raw_context());
    ret
}

//$Procedure SETERR ( Set Error Status )
pub fn SETERR(STATUS: bool, ctx: &mut Context) -> bool {
    let mut SETERR: bool = false;

    //
    // Declaration of the entry point, FAILED:
    //

    //
    // Executable Code:
    //

    // [f2rust] HACK: FAILED is called extremely frequently, and our
    // standard implementation of SAVE variables (using a hashmap) has
    // a noticeable performance cost, so we have some intrinsics
    // specifically to optimise this case.

    ctx.set_spice_failed(STATUS);

    //
    // Give SETERR a value; the value does not have any
    // meaning, but it appears standard FORTRAN requires this.
    //

    SETERR = true;

    SETERR
}

/// Error Status Indicator
///
/// Return .TRUE. if an error condition has been signaled via SIGERR.
/// FAILED is the SPICELIB status indicator.
///
/// # Required Reading
///
/// * [ERROR](crate::required_reading::error)
///
/// # Brief I/O
///
/// ```text
///  The function takes the value .TRUE. if an error condition
///  was detected; it is .FALSE. otherwise.
/// ```
///
/// # Detailed Output
///
/// ```text
///  Please read the required reading file before reading this!
///
///  The value taken by FAILED indicates status.
///
///  The status value applies to the SPICELIB routines,
///  and to any other routines which call the status-setting
///  routine, SIGERR.
///
///  When FAILED has the value, .TRUE., an error condition
///  exists.   .FALSE. means "no error."
///
///  More specifically, when FAILED has the value .TRUE.,
///  some routine has indicated an error by calling the
///  SPICELIB routine, SIGERR. All SPICELIB routines
///  which can detect errors do this. Non-SPICELIB
///  routines may also reference SIGERR if desired.
///
///  When FAILED has the value .FALSE., either no routine
///  has yet signaled an error via SIGERR, or the status
///  has been reset using, what else, RESET.
///
///  FAILED is initialized to have the value, .FALSE.
///  This indicates a  "no error" status.
///
///  See "particulars" below for (slightly) more information.
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
/// # Particulars
///
/// ```text
///  See the required reading file for details of error
///  processing. However, here are some notes:
///
///  When any SPICELIB routine detects an error, the
///  status is set to indicate an error condition via
///  a call to SIGERR. After SIGERR
///  returns, further calls to FAILED will return the
///  value, .TRUE., indicating an error condition.
///
///  Non-SPICELIB routines may also call SIGERR to indicate
///  an error condition; FAILED will reflect such calls
///  as well.
///
///  It is possible to re-set the error status to indicate
///  "no error" using the SPICELIB routine, RESET (see).
///
///  The effect on FAILED of resetting the status is
///  that FAILED will again return the value .FALSE.,
///  indicating "no error."
///
///  One of the main virtues of the SPICELIB error
///  handling mechanism is that you don't HAVE to test the
///  error status after every call to a SPICELIB routine.
///  If you set the error handling mode to 'RETURN', using
///  the routine, ERRACT, SPICELIB routines won't crash
///  when an error occurs; following the detection of the
///  error, each routine will return immediately upon entry.
///  Therefore, you call several SPICELIB routines in a
///  row, and just test status at the end of the sequence
///  of calls, if you wish. See "examples" below.
/// ```
///
/// # Examples
///
/// ```text
///  1. Here's an example of a simple call to RDTEXT, followed
///      by a test of the status.
///
///
///  C
///  C     We read a line of text from file SPUD.DAT:
///  C
///
///        CALL RDTEXT ( 'SPUD.DAT', LINE, EOF )
///
///        IF ( FAILED() ) THEN
///
///  C        An error occurred during the read.
///
///           [respond to error here]
///
///        END IF
///
///
///  2.    Here's an example in which we don't want to
///        put the error test inside our loop.  We just
///        test the error status after the loop terminates.
///        We can do this because we (that is, you, the user)
///        have made the call,
///
///               CALL ERRACT ( 'RETURN' )
///
///        prior to execution of the following code. If an
///        error does occur, the remaining calls to RDTEXT
///        will have no effect. Here's the example:
///
///  C
///  C     We read the first 5000 lines of a file, or until
///  C     EOF is reached, whichever comes first:
///  C
///  C     Note:  the "DO WHILE" construct is available in
///  C     VAX FORTRAN.
///  C
///
///        LCOUNT = 0
///        DO WHILE (  ( .NOT. EOF ) .AND. ( LCOUNT .LE. 5000 )  )
///
///           CALL RDTEXT ( 'SPUD.DAT', LINE(LCOUNT), EOF )
///
///           LCOUNT = LCOUNT + 1
///
///        END DO
///
///        IF ( FAILED() ) THEN
///  C
///  C        An error occurred during the read
///  C
///           [respond to error here]
///
///        END IF
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine automatically detects errors occurring in
///      the SPICELIB code. To make this routine work
///      for your own routines, your routines must call SIGERR
///      to report errors.
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
/// -    SPICELIB Version 1.1.0, 26-OCT-2021 (JDR) (NJB)
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
/// -     Beta Version 1.1.0, 18-DEC-1989 (HAN)
///
///          Empty parentheses added to the ENTRY statement in order to
///          comply with the ANSI Fortran 77 Standard.
/// ```
pub fn failed(ctx: &mut SpiceContext) -> bool {
    let ret = FAILED(ctx.raw_context());
    ret
}

//$Procedure FAILED ( Error Status Indicator )
pub fn FAILED(ctx: &mut Context) -> bool {
    let mut FAILED: bool = false;

    //
    // Executable Code:
    //

    //
    // Grab saved status value:
    //

    // [f2rust] HACK: FAILED is called extremely frequently, and our
    // standard implementation of SAVE variables (using a hashmap) has
    // a noticeable performance cost, so we have some intrinsics
    // specifically to optimise this case.

    FAILED = ctx.get_spice_failed();

    FAILED
}
