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
        let mut SAVMSG = vec![b' '; LMSGLN as usize];

        fstr::assign(&mut SAVMSG, b" ");

        Self { SAVMSG }
    }
}

/// Store Long Error Message
///
/// Store a long error message.
///
/// PUTLMS is a low-level data structure access routine.
/// DO NOT CALL THIS ROUTINE. USE SETMSG, NOT PUTLMS, TO SET THE
/// CURRENT LONG ERROR MESSAGE.
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
///  MSG        I   A long error message.
/// ```
///
/// # Detailed Input
///
/// ```text
///  MSG      is the current long error message. This value will be
///           saved.
/// ```
///
/// # Parameters
///
/// ```text
///  LMSGLN   is the maximum length of the long error message. See
///           the include file errhnd.inc for the value of LMSGLN.
/// ```
///
/// # Particulars
///
/// ```text
///  DO NOT CALL THIS ROUTINE.
///
///  This routine should be used only by routines within the SPICELIB
///  error handling system. Other routines should use SETMSG to set
///  the long error message.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  Calls to this routine by routines outside of the SPICELIB
///      error handling system may interfere with error processing.
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
/// -    SPICELIB Version 1.1.1, 27-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Added
///         $Index_Entries entry.
///
/// -    SPICELIB Version 1.1.0, 29-JUL-1997 (NJB)
///
///         Maximum length of the long error message is now represented
///         by the parameter LMSGLN. Miscellaneous header fixes were
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
///         Maximum length of the long error message is now represented
///         by the parameter LMSGLN. Miscellaneous header fixes were
///         made. Some indentation and vertical white space abnormalities
///         in the code were fixed. Some dubious comments were deleted
///         from the code.
///
/// -    Beta Version 1.0.1, 08-FEB-1989 (NJB)
///
///         Warnings added to discourage use of this routine in
///         non-error-handling code.
/// ```
pub fn putlms(ctx: &mut SpiceContext, msg: &str) {
    PUTLMS(msg.as_bytes(), ctx.raw_context());
}

//$Procedure PUTLMS ( Store Long Error Message )
pub fn PUTLMS(MSG: &[u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Local Variables:
    //

    //
    // The current long error message:
    //

    //
    // Initial values:
    //

    //
    // Executable Code:
    //
    fstr::assign(&mut save.SAVMSG, MSG);
}

/// Get Long Error Message
///
/// Return the value of the current long error message.
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
///  MSG        O   The current long error message.
/// ```
///
/// # Detailed Output
///
/// ```text
///  MSG      is the current long error message. See the
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
///  See the required reading file for details of error
///  processing.
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
/// -    SPICELIB Version 1.0.3, 03-JUN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Added
///         $Index_Entries entry. Removed unnecessary $Revisions section.
///
/// -    SPICELIB Version 1.0.2, 29-JUL-1997 (NJB)
///
///         Maximum length of the long error message is now represented
///         by the parameter LMSGLN. Miscellaneous header fixes were
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
pub fn getlms(ctx: &mut SpiceContext, msg: &mut str) {
    GETLMS(fstr::StrBytes::new(msg).as_mut(), ctx.raw_context());
}

//$Procedure GETLMS ( Get Long Error Message )
pub fn GETLMS(MSG: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Grab the saved long message:
    //
    fstr::assign(MSG, &save.SAVMSG);
}
