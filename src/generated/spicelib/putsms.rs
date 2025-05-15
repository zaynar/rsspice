//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const LMSGLN: i32 = (23 * 80);
const SMSGLN: i32 = 25;

struct SaveVars {
    SAVMSG: Vec<u8>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SAVMSG = vec![b' '; SMSGLN as usize];

        fstr::assign(&mut SAVMSG, b" ");

        Self { SAVMSG }
    }
}

/// Store Short Error Message
///
/// Store a short error message.
///
/// PUTSMS is a low-level data structure access routine.
/// DO NOT CALL THIS ROUTINE. USE SIGERR, NOT PUTSMS, TO SIGNAL
/// ERRORS.
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
///  MSG        I   A short error message.
/// ```
///
/// # Detailed Input
///
/// ```text
///  MSG      is the current short error message. This value will be
///           saved.
/// ```
///
/// # Parameters
///
/// ```text
///  SMSGLN   is the maximum length of the short error message. See
///           the include file errhnd.inc for the value of SMSGLN.
/// ```
///
/// # Particulars
///
/// ```text
///  This is a data structure access routine for the
///  toolkit short error message. This routine should be
///  used for no other purpose; in particular, it should
///  not be used to signal errors. Use SIGERR for that.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  Calls to this routine by routines other than the
///      SPICELIB error handling routines may interfere
///      with error processing.
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
/// -    SPICELIB Version 1.2.0, 27-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added
///         $Index_Entries entry.
///
/// -    SPICELIB Version 1.1.0, 29-JUL-1997 (NJB)
///
///         Maximum length of the short error message is now represented
///         by the parameter SMSGLN. Miscellaneous header fixes were
///         made. Some indentation and vertical white space abnormalities
///         in the code were fixed. Some dubious comments were deleted
///         from the code.
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
/// -    SPICELIB Version 1.1.0, 29-JUL-1997 (NJB)
///
///         Maximum length of the short error message is now represented
///         by the parameter SMSGLN. Miscellaneous header fixes were
///         made. Some indentation and vertical white space abnormalities
///         in the code were fixed. Some dubious comments were deleted
///         from the code.
///
/// -    Beta Version 1.0.1, 08-FEB-1989 (NJB)
///
///         Warnings added to discourage use of this routine in
///         non-error-handling code.
/// ```
pub fn putsms(ctx: &mut SpiceContext, msg: &str) {
    PUTSMS(msg.as_bytes(), ctx.raw_context());
}

//$Procedure PUTSMS ( Store Short Error Message )
pub fn PUTSMS(MSG: &[u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Local Variables:
    //

    //
    // The current short error message:
    //

    //
    // Initial values:
    //

    //
    // Executable Code:
    //
    fstr::assign(&mut save.SAVMSG, MSG);
}

/// Get Short Error Message
///
/// Return the value of the current short error message.
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
///  MSG        O   The current short error message.
/// ```
///
/// # Detailed Output
///
/// ```text
///  MSG      is the current short error message. See the
///           "required reading" file for a detailed discussion
///           of error messages.
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
///  See the required reading file for details of error processing.
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
/// -    SPICELIB Version 1.2.0, 03-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added
///         $Index_Entries entry. Removed unnecessary $Revisions section.
///
/// -    SPICELIB Version 1.1.0, 29-JUL-1997 (NJB)
///
///         Maximum length of the short error message is now represented
///         by the parameter SMSGLN. Miscellaneous header fixes were
///         made. Some indentation and vertical white space abnormalities
///         in the code were fixed. Some dubious comments were deleted
///         from the code.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (NJB)
/// ```
pub fn getsms(ctx: &mut SpiceContext, msg: &mut str) {
    GETSMS(fstr::StrBytes::new(msg).as_mut(), ctx.raw_context());
}

//$Procedure GETSMS ( Get Short Error Message )
pub fn GETSMS(MSG: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Grab saved short message:
    //
    fstr::assign(MSG, &save.SAVMSG);
}
