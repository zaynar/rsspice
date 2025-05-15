//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const WRDLEN: i32 = 9;
const MAXWRD: i32 = 10;
const OPLEN: i32 = 3;
const LSTLEN: i32 = 100;

/// Get/Set Error Output Items
///
/// Retrieve or set the list of error message items
/// to be output when an error is detected.
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
///  OP         I   The operation:  'GET' or 'SET'.
///  LIST      I-O  Specification of error messages to be output.
/// ```
///
/// # Detailed Input
///
/// ```text
///  OP       indicates the operation to be performed. Possible
///           values are 'GET' and 'SET'.
///
///           'SET' means, "the following list specifies the default
///           selection of error messages to be output." These are
///           the messages that will be output to the default error
///           output device (selected by ERRDEV) when an error is
///           detected.
///
///           'GET' means, "return the current list of error output
///           items." This is the exact list that was set by the
///           last call to this routine with the 'SET' option.
///
///           The option can be specified in mixed case. For example,
///           the following call will work:
///
///           CALL ERRPRT ( 'SeT' , 'ALL' )
///
///
///  LIST     is a list of error message items. The items
///           are delimited by commas. The items that can be
///           in the list are the words:
///
///           1.  SHORT        ...indicates the short error message
///           2.  EXPLAIN      ...the explanation of the short message
///           3.  LONG         ...the long error message
///           4.  TRACEBACK    ...the traceback
///           5.  ALL          ...indicates "output all messages"
///           6.  NONE         ...indicates "don't output any messages"
///           7.  DEFAULT      ...same as ALL, but includes default
///                               message
///
///           A "list" is a character string containing some or
///           all of the above words, delimited by commas. Examples
///           are:
///
///           1.  'SHORT, EXPLAIN'
///           2.  'SHORT, LONG'
///           3.  'ALL'
///           4.  'NONE'
///           5.  'ALL, NONE, ALL, SHORT, NONE'
///
///           Each word in the list can be thought of as
///           "flipping a switch" to enable or disable the output
///           of the message(s) indicated by the word. The
///           words are acted on in the order they occur in the
///           list, starting with the leftmost word. As examples,
///           consider the sample lists above.
///
///           The effect of the first list above, 'SHORT, EXPLAIN',
///           is to enable the output of the short error message
///           and the explanatory text corresponding to it.
///
///           The effect of the second list is to enable the output
///           of the short and long messages.
///
///           The effect of the third list is to enable the output of
///           all of the error messages (short, long, explanation
///           of the short message, and traceback).
///
///           The effect of the fourth list is to disable output of
///           all of the messages.
///
///           The effect of the fifth list is to disable output of
///           all of the messages. The reason for this is that
///           the words in the list are responded to in order,
///           from left to right, and "NONE" is the last word.
///
///           If any words other than SHORT, LONG, EXPLAIN, ALL,
///           DEFAULT, TRACEBACK or NONE appear in LIST, those words
///           that are recognized are responded to. The words
///           that are not recognized are diagnosed as
///           erroneous, and error messages are generated
///           for each such unrecognized word.
///
///           The length of LIST is caller-defined, but only
///           the first 100 characters of LIST will be saved
///           for later retrieval.
///
///           Only the first 10 items in the list are used;
///           the rest are ignored.
/// ```
///
/// # Detailed Output
///
/// ```text
///  LIST     is a list of error message items. The value of
///           LIST is that set by the last call to this routine
///           using the 'SET' option. See "Detailed Input"
///           for a description of the possible values and
///           meanings of LIST.
///
///           The initial value returned is 'DEFAULT'.
///
///           Only the first 100 characters of LIST are saved
///           when the list is set; any additional characters
///           are truncated. Therefore, the first 100
///           characters, at most, of the saved value of LIST
///           will be returned.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If an invalid value of the argument OP is supplied, the error
///      SPICE(INVALIDOPERATION) is signaled.
///
///  2)  If OP is 'SET' and an invalid word is detected within the list
///      of error message items LIST, the error SPICE(INVALIDLISTITEM)
///      is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is part of the SPICELIB error handling mechanism.
///
///  Please read the "required reading"!
///
///  This routine is intended to be used in conjunction with
///  ERRDEV, which selects the default output device to which
///  the error messages selected by this routine will be
///  output.
///
///  Additionally, the error response action must be
///  something other than 'IGNORE' if the error messages
///  are to be output. Possible choices of the error
///  response action are 'RETURN', 'REPORT', 'ABORT', 'DEFAULT', and
///  'IGNORE'.  Use ERRACT to set the error response action.
///
///
///  Only the first 100 characters of LIST are saved.
///
///  The default set of error messages that are output is the
///  set specified by 'DEFAULT'; i.e., all of them, including
///  the 'default' message.
/// ```
///
/// # Examples
///
/// ```text
///  1. In this example, we select as the output device
///     the file, SPUD.DAT, and then select the error
///     messages to be output. We choose the short
///     error message and the traceback. Since a
///     different set of messages may have been selected
///     previously, we clear the old setting by putting
///     the word, 'NONE', at the beginning of the list.
///
///     C
///     C      Set the error output device to SPUD.DAT:
///     C
///
///            CALL ERRDEV (  'SET',  'SPUD.DAT'  )
///
///     C
///     C      Choose error messages:
///     C
///
///            CALL ERRPRT (  'SET',  'NONE, SHORT, TRACEBACK'  )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The device to which the selected error messages will be
///      written must be selected via ERRDEV; otherwise, messages will
///      be written to the initial default device.
///
///  2)  Only the first 100 characters of LIST are saved.
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
/// -    SPICELIB Version 1.2.0, 19-APR-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. $Exceptions
///         section has been completely updated to provide only the list
///         of exceptions. Additional information provided there has been
///         moved to $Particulars.
///
/// -    SPICELIB Version 1.1.0, 28-AUG-1999 (NJB)
///
///         Output string is now built on the fly. The routine previously
///         returned a saved string which could fail to represent correctly
///         the set of selected message types.
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
/// -    SPICELIB Version 1.1.0, 28-AUG-1999 (NJB)
///
///         Output string is now built on the fly. The routine previously
///         returned a saved string which could fail to represent correctly
///         the set of selected message types.
///
/// -    Beta Version 1.2.0, 16-FEB-1988 (NJB)
///
///         Declaration of the unused variable TMPLST removed.
///         Trace participation added. This routine now checks in
///         and checks out. However, it does not test RETURN,
///         because it should be able to execute in RETURN mode when
///         an error condition exists.
///
/// -    Beta Version 1.1.0, 06-OCT-1988 (NJB)
///
///         Superfluous references to LASTNB removed. These references
///         were so many tonsils; they really had no function.
/// ```
pub fn errprt(ctx: &mut SpiceContext, op: &str, list: &mut str) -> crate::Result<()> {
    ERRPRT(
        op.as_bytes(),
        fstr::StrBytes::new(list).as_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure ERRPRT ( Get/Set Error Output Items )
pub fn ERRPRT(OP: &[u8], LIST: &mut [u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut WORDS = ActualCharArray::new(WRDLEN, 1..=MAXWRD);
    let mut UPWORD = [b' '; WRDLEN as usize];
    let mut NUMWRD: i32 = 0;
    let mut SHORT: bool = false;
    let mut LONG: bool = false;
    let mut EXPL: bool = false;
    let mut TRACE: bool = false;
    let mut DFAULT: bool = false;
    let mut STATUS: bool = false;
    let mut UPOP = [b' '; OPLEN as usize];
    let mut LOCOP = [b' '; OPLEN as usize];

    //
    // SPICELIB functions
    //

    //
    // Local Variables:
    //

    //
    // Executable Code:
    //

    CHKIN(b"ERRPRT", ctx)?;

    //
    // We first initialize the message selection flags to
    // correspond to the current selection of error messages:
    //
    SHORT = MSGSEL(b"SHORT", ctx)?;
    LONG = MSGSEL(b"LONG", ctx)?;
    EXPL = MSGSEL(b"EXPLAIN", ctx)?;
    TRACE = MSGSEL(b"TRACEBACK", ctx)?;
    DFAULT = MSGSEL(b"DEFAULT", ctx)?;

    //
    // We save the operation string as input, and get
    // an upper case version for our own use:
    //
    LJUST(OP, &mut UPOP);
    UCASE(&UPOP.clone(), &mut UPOP, ctx);

    if fstr::eq(&UPOP, b"GET") {
        //
        // Construct a string indicating which messages are enabled.
        //
        fstr::assign(LIST, b" ");

        if SHORT {
            fstr::assign(LIST, b"SHORT");
        }

        if LONG {
            if fstr::eq(LIST, b" ") {
                fstr::assign(LIST, b"LONG");
            } else {
                SUFFIX(b", LONG", 0, LIST);
            }
        }

        if EXPL {
            if fstr::eq(LIST, b" ") {
                fstr::assign(LIST, b"EXPLAIN");
            } else {
                SUFFIX(b", EXPLAIN", 0, LIST);
            }
        }

        if TRACE {
            if fstr::eq(LIST, b" ") {
                fstr::assign(LIST, b"TRACEBACK");
            } else {
                SUFFIX(b", TRACEBACK", 0, LIST);
            }
        }

        if DFAULT {
            if fstr::eq(LIST, b" ") {
                fstr::assign(LIST, b"DEFAULT");
            } else {
                SUFFIX(b", DEFAULT", 0, LIST);
            }
        }
    } else if fstr::eq(&UPOP, b"SET") {
        //
        // We parse the list of words, converting each word
        // to upper case, testing each word for validity,
        // and "flipping the switches" to enable or disable
        // the output of the various error messages as
        // directed by each word, starting with the leftmost.
        // We update local flags according to the words we
        // recognize, and update the global flags when we're
        // done parsing the list.
        //
        // If an invalid word is encountered, we signal an
        // error, and continue parsing the list.
        //
        //

        LPARSE(LIST, b",", MAXWRD, &mut NUMWRD, WORDS.as_arg_mut());

        for I in 1..=NUMWRD {
            UCASE(&WORDS[I], &mut UPWORD, ctx);

            if fstr::eq(&UPWORD, b"SHORT") {
                SHORT = true;
            } else if fstr::eq(&UPWORD, b"LONG") {
                LONG = true;
            } else if fstr::eq(&UPWORD, b"EXPLAIN") {
                EXPL = true;
            } else if fstr::eq(&UPWORD, b"TRACEBACK") {
                TRACE = true;
            } else if fstr::eq(&UPWORD, b"ALL") {
                SHORT = true;
                LONG = true;
                EXPL = true;
                TRACE = true;
            } else if fstr::eq(&UPWORD, b"DEFAULT") {
                SHORT = true;
                LONG = true;
                EXPL = true;
                TRACE = true;
                DFAULT = true;
            } else if fstr::eq(&UPWORD, b"NONE") {
                SHORT = false;
                LONG = false;
                EXPL = false;
                TRACE = false;
                DFAULT = false;
            } else if fstr::ne(&UPWORD, b" ") {
                //
                // Oops! Invalid word...
                //
                SETMSG(&fstr::concat(b"ERRPRT: An invalid list item was found in the error message list.  The word was:", WORDS.get(I)), ctx);

                SIGERR(b"SPICE(INVALIDLISTITEM)", ctx)?;
            }
            //
            // At this point, we have either set some set of
            // flags in response to WORD, or determined that
            // WORD was invalid.
            //
        }
        //
        // We've now responded to all words in LIST.
        //

        //
        // Now we store the flag values we've set, for global
        // consumption (SETPRT doesn't actually detect errors).
        //

        STATUS = SETPRT(SHORT, EXPL, LONG, TRACE, DFAULT, ctx);
    } else {
        //
        // An invalid value of OP was supplied.
        //

        fstr::assign(&mut LOCOP, OP);

        SETMSG(
            &fstr::concat(
                b"ERRPRT:  An invalid value of OP was supplied.  The value was: ",
                &LOCOP,
            ),
            ctx,
        );

        SIGERR(b"SPICE(INVALIDOPERATION)", ctx)?;
    }

    CHKOUT(b"ERRPRT", ctx)?;
    Ok(())
}
