//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const LMSGLN: i32 = (23 * 80);
const SMSGLN: i32 = 25;
const FILEN: i32 = 255;
const LL: i32 = 80;
const NAMLEN: i32 = 32;
const BORCHR: &[u8] = b"=";
const DEFLEN: i32 = 4;
const ENDLEN: i32 = 1;
const MAXWRD: i32 = 5;
const WORDLN: i32 = 9;
const STRLEN: i32 = 3;
const VERLEN: i32 = 80;
const XLEN: i32 = 80;
const TMPLEN: i32 = (SMSGLN + XLEN);

struct SaveVars {
    BORDER: Vec<u8>,
    DEFMSG: ActualCharArray,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BORDER = vec![b' '; LL as usize];
        let mut DEFMSG = ActualCharArray::new(LL, 1..=DEFLEN);
        let mut FIRST: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"Oh, by the way:  The SPICELIB error hand")].into_iter();
            fstr::assign(
                fstr::substr_mut(DEFMSG.get_mut(1), 1..=40),
                clist.next().unwrap().into_str(),
            );

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"ling actions are USER-TAILORABLE.  You")].into_iter();
            fstr::assign(
                fstr::substr_mut(DEFMSG.get_mut(1), 41..=LL),
                clist.next().unwrap().into_str(),
            );

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"can choose whether the Toolkit aborts or")].into_iter();
            fstr::assign(
                fstr::substr_mut(DEFMSG.get_mut(2), 1..=40),
                clist.next().unwrap().into_str(),
            );

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b" continues when errors occur, which   ")].into_iter();
            fstr::assign(
                fstr::substr_mut(DEFMSG.get_mut(2), 41..=LL),
                clist.next().unwrap().into_str(),
            );

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"error messages to output, and where to s")].into_iter();
            fstr::assign(
                fstr::substr_mut(DEFMSG.get_mut(3), 1..=40),
                clist.next().unwrap().into_str(),
            );

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"end the output.  Please read the ERROR")].into_iter();
            fstr::assign(
                fstr::substr_mut(DEFMSG.get_mut(3), 41..=LL),
                clist.next().unwrap().into_str(),
            );

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"\"Required Reading\" file, or see the rout")].into_iter();
            fstr::assign(
                fstr::substr_mut(DEFMSG.get_mut(4), 1..=40),
                clist.next().unwrap().into_str(),
            );

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"ines ERRACT, ERRDEV, and ERRPRT.      ")].into_iter();
            fstr::assign(
                fstr::substr_mut(DEFMSG.get_mut(4), 41..=LL),
                clist.next().unwrap().into_str(),
            );

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        FIRST = true;

        Self {
            BORDER,
            DEFMSG,
            FIRST,
        }
    }
}

/// Output Error Messages
///
/// Output error messages.
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
///  LIST       I   A list of error message types.
///  FILEN      P   Maximum length of file name.
///  NAMLEN     P   Maximum length of module name. See TRCPKG.
///  LL         P   Output line length.
/// ```
///
/// # Detailed Input
///
/// ```text
///  LIST     is a list of error message types. A list is a
///           character string containing one or more words
///           from the following list, separated by commas.
///
///              SHORT
///              EXPLAIN
///              LONG
///              TRACEBACK
///              DEFAULT
///
///           Each type of error message specified in LIST will
///           be output when an error is detected, if it is
///           enabled for output. Note that DEFAULT does
///           NOT refer to the "default message selection,"
///           but rather to a special message that is output
///           when the error action is 'DEFAULT'.  This message
///           is a statement referring the user to the error
///           handling documentation.
///
///           Messages are never duplicated in the output; for
///           instance, supplying a value of LIST such as
///
///              'SHORT, SHORT'
///
///           does NOT result in the output of two short
///           messages.
///
///           The words in LIST may appear in mixed case;
///           for example, the call
///
///              CALL OUTMSG ( 'ShOrT' )
///
///           will work.
/// ```
///
/// # Parameters
///
/// ```text
///  FILEN    is the maximum device name length that can be
///           accommodated by this routine.
///
///  NAMELN   is the maximum length of an individual module name.
///
///  LL       is the maximum line length for the output message.
///           If the output message string is very long, it is
///           displayed over several lines, each of which has a
///           maximum length of LL characters.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If an invalid message type is provided in LIST, the error
///      SPICE(INVALIDLISTITEM) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is part of the SPICELIB error handling
///  mechanism.
///
///  This routine outputs the error messages specified in LIST that
///  have been enabled for output (use the SPICELIB routine ERRPRT
///  to enable or disable output of specified types of error
///  messages).  A border is written out preceding and following the
///  messages. Output is directed to the current error output device.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Output the short and long error messages:
///
///     C
///     C     Output short and long messages:
///     C
///           CALL OUTMSG ( 'SHORT, LONG' )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine is intended for use by the SPICELIB error
///      handling mechanism. SPICELIB users are not expected to
///      need to call this routine.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  H.A. Neilan        (JPL)
///  B.V. Semenov       (JPL)
///  M.J. Spencer       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 5.29.0, 28-NOV-2021 (BVS)
///
///         Updated for MAC-OSX-M1-64BIT-CLANG_C.
///
/// -    SPICELIB Version 5.28.0, 13-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 5.27.0, 10-MAR-2014 (BVS)
///
///         Updated for SUN-SOLARIS-64BIT-INTEL.
///
/// -    SPICELIB Version 5.26.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-LINUX-64BIT-IFORT.
///
/// -    SPICELIB Version 5.25.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-GFORTRAN.
///
/// -    SPICELIB Version 5.24.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 5.23.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-64BIT-GCC_C.
///
/// -    SPICELIB Version 5.22.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-INTEL.
///
/// -    SPICELIB Version 5.21.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-INTEL-CC_C.
///
/// -    SPICELIB Version 5.20.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-INTEL-64BIT-CC_C.
///
/// -    SPICELIB Version 5.19.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-64BIT-NATIVE_C.
///
/// -    SPICELIB Version 5.18.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-WINDOWS-64BIT-IFORT.
///
/// -    SPICELIB Version 5.17.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-LINUX-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 5.16.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-64BIT-MS_C.
///
/// -    SPICELIB Version 5.15.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-INTEL_C.
///
/// -    SPICELIB Version 5.14.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-IFORT.
///
/// -    SPICELIB Version 5.13.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 5.12.0, 18-MAR-2009 (BVS)
///
///         Updated for PC-LINUX-GFORTRAN.
///
/// -    SPICELIB Version 5.11.0, 18-MAR-2009 (BVS)
///
///         Updated for MAC-OSX-GFORTRAN.
///
/// -    SPICELIB Version 5.10.0, 01-MAR-2009 (NJB)
///
///         Bug fix: truncation of long words in
///         output has been corrected. Local parameter
///         TMPLEN was added and is used in declaration
///         of TMPMSG.
///
/// -    SPICELIB Version 5.9.0, 19-FEB-2008 (BVS)
///
///         Updated for PC-LINUX-IFORT.
///
/// -    SPICELIB Version 5.8.0, 14-NOV-2006 (BVS)
///
///         Updated for PC-LINUX-64BIT-GCC_C.
///
/// -    SPICELIB Version 5.7.0, 14-NOV-2006 (BVS)
///
///         Updated for MAC-OSX-INTEL_C.
///
/// -    SPICELIB Version 5.6.0, 14-NOV-2006 (BVS)
///
///         Updated for MAC-OSX-IFORT.
///
/// -    SPICELIB Version 5.5.0, 14-NOV-2006 (BVS)
///
///         Updated for PC-WINDOWS-IFORT.
///
/// -    SPICELIB Version 5.4.0, 26-OCT-2005 (BVS)
///
///         Updated for SUN-SOLARIS-64BIT-GCC_C.
///
/// -    SPICELIB Version 5.3.0, 03-JAN-2005 (BVS)
///
///         Updated for PC-CYGWIN_C.
///
/// -    SPICELIB Version 5.2.0, 03-JAN-2005 (BVS)
///
///         Updated for PC-CYGWIN.
///
/// -    SPICELIB Version 5.1.5, 17-JUL-2002 (BVS)
///
///         Added MAC-OSX environments.
///
/// -    SPICELIB Version 5.1.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 5.1.3, 24-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 5.1.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 5.1.1, 18-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 5.1.0, 13-JAN-1999 (BVS)
///
///         ``errhnd.inc'' file was included. Long and short error
///         message lengths parameter declarations were deleted. Long
///         and short error message string sizes were changed to those
///         declared in ``errhnd.inc''.
///
/// -    SPICELIB Version 5.0.0, 08-APR-1998 (NJB)
///
///         Module was updated for the PC-LINUX platform.
///
/// -    SPICELIB Version 4.0.0, 09-MAY-1996 (KRG)
///
///         Added the toolkit version to the output error message.
///
///         Updated this routine to be consistent with the trace package
///         revisions. This primarily affects the creation of the
///         traceback string.
///
///         Long error messages are now wrapped on word boundaries when
///         they are longer than the output line length. Note that this
///         only happens for long error messages obtained from GETLMS,
///         and not for the error messages displayed by this subroutine
///         and other error handling subroutines that write their own
///         error messages.
///
/// -    SPICELIB Version 3.0.0, 09-NOV-1993 (HAN)
///
///         Module was updated to include the value for FILEN
///         for the Silicon Graphics, DEC Alpha-OSF/1, and
///         NeXT platforms. Also, the previous value of 256 for
///         Unix platforms was changed to 255.
///
/// -    SPICELIB Version 2.2.0, 12-OCT-1992 (HAN)
///
///         Updated module for multiple environments. Moved the parameter
///         LL to the $Declarations section of the header since it's
///         environment dependent.
///
///         The code was also reformatted so that a utility program can
///         create the source file for a specific environment given a
///         master source file.
///
/// -    SPICELIB Version 2.1.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 2.1.0, 15-MAY-1991 (MJS)
///
///         Module was updated to include the value of LL for the
///         Macintosh.
///
/// -    SPICELIB Version 2.0.0, 28-MAR-1991 (NJB)
///
///         Work-around for MS Fortran compiler error under DOS 3.10
///         was made. Some substring bounds were simplified using RTRIM.
///         Updates were made to the header to clarify the text and
///         improve the header's appearance. The default error message
///         was slightly de-uglified.
///
///         The IBM PC version of this routine now uses an output line
///         length of 78 characters rather than 80. This prevents
///         wrapping of the message borders and default error message.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (NJB)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 5.1.0, 13-JAN-1999 (BVS)
///
///         ``errhnd.inc'' file was included. Long and short error
///         message lengths parameter declarations were deleted. Long
///         and short error message string size were changed to those
///         declared in ``errhnd.inc''.
///
/// -    SPICELIB Version 5.0.0, 08-APR-1998 (NJB)
///
///         Module was updated for the PC-LINUX platform.
///
/// -    SPICELIB Version 4.0.0, 09-MAY-1996 (KRG)
///
///         Added the toolkit version to the output error message.
///
///         Updated this routine to be consistent with the trace package
///         revisions. This primarily affects the creation of the
///         traceback string.
///
///         Long error messages are now wrapped on word boundaries when
///         they are longer than the output line length. Note that this
///         only happens for long error messages obtained from GETLMS,
///         and not for the error messages displayed by this subroutine
///         and other error handling subroutines that write their own
///         error messages.
///
/// -    SPICELIB Version 3.0.0, 9-NOV-1993 (HAN)
///
///         Module was updated to include the value for FILEN
///         for the Silicon Graphics, DEC Alpha-OSF/1, and
///         NeXT platforms. Also, the previous value of 256 for
///         Unix platforms was changed to 255.
///
/// -    SPICELIB Version 2.2.0, 12-OCT-1992 (HAN)
///
///         Updated module for multiple environments. Moved the
///         parameter LL to the $Declarations section of the header since
///         it's environment dependent.
///
///         The code was also reformatted so that a utility program can
///         create the source file for a specific environment given a
///         master source file.
///
/// -    SPICELIB Version 2.1.0, 15-MAY-1991 (MJS)
///
///         Module was updated to include the value of LL for the
///         Macintosh.
///
/// -    SPICELIB Version 2.0.0, 28-MAR-1991 (NJB)
///
///         1)  Work-around for MS Fortran compiler error under DOS 3.10
///             was made. The compiler did not correctly handle code that
///             concatenated strings whose bounds involved the intrinsic
///             MAX function.
///
///         2)  Some substring bounds were simplified using RTRIM.
///
///         3)  Updates were made to the header to clarify the text and
///             improve the header's appearance.
///
///         4)  $Declarations were re-organized.
///
///         5)  The default error message was slightly de-uglified.
///
///         6)  The IBM PC version of this routine now uses an output line
///             length of 78 characters rather than 80. This prevents
///             wrapping of the message borders and default error message.
///
/// -    Beta Version 1.3.0, 19-JUL-1989 (NJB)
///
///         Calls to REMSUB removed; blanking and left-justifying used
///         instead. This was done because REMSUB handles substring
///         bounds differently than in previous versions, and no longer
///         handles all possible inputs as required by this routine.
///         LJUST, which is used now, is error free.
///
///         Also, an instance of .LT. was changed to .LE. The old code
///         caused a line break one character too soon. A minor bug, but
///         a bug nonetheless.
///
///         Also, two substring bounds were changed to ensure that they
///         remain greater than zero.
///
/// -    Beta Version 1.2.0, 16-FEB-1989 (NJB)
///
///         Warnings added to discourage use of this routine in
///         non-error-handling code. $Parameters section updated to
///         describe FILEN and NAMLEN.
///
///         Declaration of unused function FAILED removed.
///
/// -    Beta Version 1.1.0, 06-OCT-1988 (NJB)
///
///         Test added to ensure substring upper bound is greater than 0.
///         REMAIN must be greater than 0 when used as the upper bound
///         for a substring of NAME. Also, substring upper bound in
///         WRLINE call is now forced to be greater than 0.
/// ```
pub fn outmsg(ctx: &mut SpiceContext, list: &str) -> crate::Result<()> {
    OUTMSG(list.as_bytes(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure OUTMSG ( Output Error Messages )
pub fn OUTMSG(LIST: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut DEVICE = [b' '; FILEN as usize];
    let mut LINE = [b' '; LL as usize];
    let mut LMSG = [b' '; LMSGLN as usize];
    let mut NAME = [b' '; NAMLEN as usize];
    let mut OUTWRD = [b' '; LMSGLN as usize];
    let mut SMSG = [b' '; SMSGLN as usize];
    let mut TMPMSG = [b' '; TMPLEN as usize];
    let mut UPWORD = [b' '; WORDLN as usize];
    let mut VERSN = [b' '; VERLEN as usize];
    let mut WORDS = ActualCharArray::new(WORDLN, 1..=MAXWRD);
    let mut XMSG = [b' '; XLEN as usize];
    let mut DEPTH: i32 = 0;
    let mut LENGTH: i32 = 0;
    let mut NUMWRD: i32 = 0;
    let mut REMAIN: i32 = 0;
    let mut START: i32 = 0;
    let mut WRDLEN: i32 = 0;
    let mut DFAULT: bool = false;
    let mut EXPL: bool = false;
    let mut LONG: bool = false;
    let mut OUTPUT: bool = false;
    let mut SHORT: bool = false;
    let mut TRACE: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // These parameters are system-independent.
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Initial Values:
    //

    //
    // Executable Code:
    //

    //
    // The first time through, set up the output borders.
    //
    if save.FIRST {
        save.FIRST = false;

        for I in 1..=LL {
            fstr::assign(fstr::substr_mut(&mut save.BORDER, I..=I), BORCHR);
        }
    }

    //
    // No messages are to be output which are not specified
    // in LIST:
    //
    SHORT = false;
    EXPL = false;
    LONG = false;
    TRACE = false;
    DFAULT = false;

    // We parse the list of message types, and set local flags
    // indicating which ones are to be output.  If we find
    // a word we don't recognize in the list, we signal an error
    // and continue parsing the list.
    //
    LPARSE(LIST, b",", MAXWRD, &mut NUMWRD, WORDS.as_arg_mut());

    for I in 1..=NUMWRD {
        UCASE(&WORDS[I], &mut UPWORD, ctx);

        if fstr::eq(&UPWORD, b"SHORT") {
            SHORT = true;
        } else if fstr::eq(&UPWORD, b"EXPLAIN") {
            EXPL = true;
        } else if fstr::eq(&UPWORD, b"LONG") {
            LONG = true;
        } else if fstr::eq(&UPWORD, b"TRACEBACK") {
            TRACE = true;
        } else if fstr::eq(&UPWORD, b"DEFAULT") {
            DFAULT = true;
        } else {
            //
            // Unrecognized word!  This is an error...
            //
            // We have a special case on our hands; this routine
            // is itself called by SIGERR, so a recursion error will
            // result if this routine calls SIGERR.  So we output
            // the error message directly:
            //
            GETDEV(&mut DEVICE, ctx);

            WRLINE(&DEVICE, b"SPICE(INVALIDLISTITEM)", ctx)?;
            WRLINE(&DEVICE, b" ", ctx)?;
            WRLINE(
                &DEVICE,
                b"OUTMSG:  An invalid message type was specified in the type list. ",
                ctx,
            )?;
            WRLINE(
                &DEVICE,
                &fstr::concat(b"The invalid message type was ", WORDS.get(I)),
                ctx,
            )?;
        }
    }

    //
    // LIST has been parsed.
    //
    // Now, we output those error messages that were specified by LIST
    // and which belong to the set of messages selected for output.
    //

    //
    // We get the default error output device:
    //
    GETDEV(&mut DEVICE, ctx);

    OUTPUT = (((((SHORT && MSGSEL(b"SHORT", ctx)?) || (EXPL && MSGSEL(b"EXPLAIN", ctx)?))
        || (LONG && MSGSEL(b"LONG", ctx)?))
        || (TRACE && MSGSEL(b"TRACEBACK", ctx)?))
        || ((DFAULT && MSGSEL(b"DEFAULT", ctx)?) && fstr::ne(&DEVICE, b"NULL")));

    //
    // We go ahead and output those messages that have been specified
    // in the list and also are enabled for output. The order of the
    // cases below IS significant; the order in which the messages
    // appear in the output depends on it.
    //

    //
    // If there's nothing to output, we can leave now.
    //
    if !OUTPUT {
        return Ok(());
    }

    //
    // Write the starting border: skip a line, write the border,
    // skip a line.
    //
    WRLINE(&DEVICE, b" ", ctx)?;
    WRLINE(&DEVICE, &save.BORDER, ctx)?;
    WRLINE(&DEVICE, b" ", ctx)?;
    //
    // Output the toolkit version and skip a line.
    //
    TKVRSN(b"TOOLKIT", &mut VERSN);

    fstr::assign(&mut LINE, &fstr::concat(b"Toolkit version: ", &VERSN));

    WRLINE(&DEVICE, &LINE, ctx)?;
    WRLINE(&DEVICE, b" ", ctx)?;
    //
    // Next, we output the messages specified in the list
    // that have been enabled.
    //
    // We start with the short message and its accompanying
    // explanation.  If both are to be output, they are
    // concatenated into a single message.
    //
    if ((SHORT && MSGSEL(b"SHORT", ctx)?) && (EXPL && MSGSEL(b"EXPLAIN", ctx)?)) {
        //
        // Extract the short message from global storage; then get
        // the corresponding explanation.
        //
        GETSMS(&mut SMSG, ctx);
        EXPLN(&SMSG, &mut XMSG);

        fstr::assign(
            &mut TMPMSG,
            &fstr::concat(
                &fstr::concat(fstr::substr(&SMSG, 1..=RTRIM(&SMSG)), b" -- "),
                &XMSG,
            ),
        );

        WRLINE(&DEVICE, &TMPMSG, ctx)?;
        WRLINE(&DEVICE, b" ", ctx)?;
    } else if (SHORT && MSGSEL(b"SHORT", ctx)?) {
        //
        // Output the short error message without the explanation.
        //
        GETSMS(&mut SMSG, ctx);

        WRLINE(&DEVICE, &SMSG, ctx)?;
        WRLINE(&DEVICE, b" ", ctx)?;
    } else if (EXPL && MSGSEL(b"EXPLAIN", ctx)?) {
        //
        // Obtain the explanatory text for the short error
        // message and output it:
        //
        GETSMS(&mut SMSG, ctx);
        EXPLN(&SMSG, &mut XMSG);

        WRLINE(&DEVICE, &XMSG, ctx)?;
        WRLINE(&DEVICE, b" ", ctx)?;
    }

    if (LONG && MSGSEL(b"LONG", ctx)?) {
        //
        // Extract the long message from global storage and
        // output it:
        //
        GETLMS(&mut LMSG, ctx);
        //
        // Get the number of words in the error message.
        //
        NUMWRD = WDCNT(&LMSG);
        fstr::assign(&mut LINE, b" ");
        START = 1;
        //
        // Format the words into output lines and display them as
        // needed.
        //
        for I in 1..=NUMWRD {
            NEXTWD(&LMSG.clone(), &mut OUTWRD, &mut LMSG);

            WRDLEN = RTRIM(&OUTWRD);

            if ((START + WRDLEN) <= LL) {
                fstr::assign(fstr::substr_mut(&mut LINE, START..), &OUTWRD);
                START = ((START + WRDLEN) + 1);
            } else {
                if (WRDLEN <= LL) {
                    //
                    // We had a short word, so just write the line and
                    // continue.
                    //
                    WRLINE(&DEVICE, &LINE, ctx)?;
                    START = (WRDLEN + 2);
                    fstr::assign(&mut LINE, &OUTWRD);
                } else {
                    //
                    // We got a very long word here, so we break it up and
                    // write it out. We fit as much of it as we an into line
                    // as possible before writing it.
                    //
                    // Get the remaining space. If START is > 1 we have at
                    // least one word already in the line, including it's
                    // trailing space, otherwise the line is blank. If line
                    // is empty, we have all of the space available.
                    //
                    if (START > 1) {
                        REMAIN = (LL - START);
                    } else {
                        REMAIN = LL;
                    }
                    //
                    // Now we stuff bits of the word into the output line
                    // until we're done, i.e., until we have a word part
                    // that is less than the output length. First, we
                    // check to see if there is a "significant" amount of
                    // room left in the current output line. If not, we
                    // write it and then begin stuffing the long word into
                    // output lines.
                    //
                    if (REMAIN < 10) {
                        WRLINE(&DEVICE, &LINE, ctx)?;
                        fstr::assign(&mut LINE, b" ");
                        REMAIN = LL;
                        START = 1;
                    }
                    //
                    // Stuff the word a chunk at a time into output lines
                    // and write them. After writing a line, we clear the
                    // part of the long word that we just wrote, left
                    // justifying the remaining part before proceeding.
                    //
                    while (WRDLEN > LL) {
                        fstr::assign(
                            fstr::substr_mut(&mut LINE, START..),
                            fstr::substr(&OUTWRD, 1..=REMAIN),
                        );
                        WRLINE(&DEVICE, &LINE, ctx)?;

                        fstr::assign(fstr::substr_mut(&mut OUTWRD, 1..=REMAIN), b" ");

                        LJUST(&OUTWRD.clone(), &mut OUTWRD);

                        fstr::assign(&mut LINE, b" ");
                        WRDLEN = (WRDLEN - REMAIN);
                        REMAIN = LL;
                        START = 1;
                    }
                    //
                    // If we had a part of the long word left, get set up to
                    // append more words from the error message to the output
                    // line. If we finished the word, WRDLEN .EQ. 0, then
                    // START and LINE have already been initialized.
                    //
                    if (WRDLEN > 0) {
                        START = (WRDLEN + 2);
                        fstr::assign(&mut LINE, &OUTWRD);
                    }
                }
            }
        }
        //
        // We may need to write the remaining part of a line.
        //
        if fstr::ne(&LINE, b" ") {
            WRLINE(&DEVICE, &LINE, ctx)?;
        }

        WRLINE(&DEVICE, b" ", ctx)?;
    }

    if (TRACE && MSGSEL(b"TRACEBACK", ctx)?) {
        //
        // Extract the traceback from global storage and
        // output it:
        //
        TRCDEP(&mut DEPTH, ctx);

        if (DEPTH > 0) {
            //
            // We know we'll be outputting some trace information.
            // So, write a line telling the reader what's coming.
            //
            WRLINE(
                &DEVICE,
                b"A traceback follows.  The name of the highest level module is first.",
                ctx,
            )?;
            //
            // While there are more names in the traceback
            // representation, we stuff them into output lines and
            // write the lines out when they are full.
            //
            fstr::assign(&mut LINE, b" ");
            REMAIN = LL;

            for INDEX in 1..=DEPTH {
                //
                // For each module name in the traceback representation,
                // retrieve module name and stuff it into one or more
                // lines for output.
                //
                // Get a name and add the call order sign.  We
                // indicate calling order by a ' --> ' delimiter; e.g.
                // "A calls B" is indicated by 'A --> B'.
                //
                TRCNAM(INDEX, &mut NAME, ctx)?;

                LENGTH = LASTNB(&NAME);
                //
                // If it's the first name, just put it into the output
                // line, otherwise, add the call order sign and put the
                // name into the output line.
                //
                if (INDEX == 1) {
                    SUFFIX(&NAME, 0, &mut LINE);
                    REMAIN = (REMAIN - LENGTH);
                } else {
                    //
                    // Add the calling order indicator, if it will fit.
                    // If not, write the line and put the indicator as
                    // the first thing on the next line.
                    //
                    if (REMAIN >= 4) {
                        SUFFIX(b"-->", 1, &mut LINE);
                        REMAIN = (REMAIN - 4);
                    } else {
                        WRLINE(&DEVICE, &LINE, ctx)?;
                        fstr::assign(&mut LINE, b"-->");
                        REMAIN = (LL - 3);
                    }
                    //
                    // The name fits or it doesn't. If it does, just add
                    // it, if it doesn't, write it, then make the name
                    // the first thing on the next line.
                    //
                    if (REMAIN >= LENGTH) {
                        SUFFIX(&NAME, 1, &mut LINE);
                        REMAIN = ((REMAIN - LENGTH) - 1);
                    } else {
                        WRLINE(&DEVICE, &LINE, ctx)?;
                        fstr::assign(&mut LINE, &NAME);
                        REMAIN = (LL - LENGTH);
                    }
                }
            }

            //
            // At this point, no more names are left in the
            // trace representation.  LINE may still contain
            // names, or part of a long name.  If it does,
            // we now write it out.
            //
            if fstr::ne(&LINE, b" ") {
                WRLINE(&DEVICE, &LINE, ctx)?;
            }

            WRLINE(&DEVICE, b" ", ctx)?;
        }
        //
        // At this point, either we have output the trace
        // representation, or the trace representation was
        // empty.
        //
    }

    if (DFAULT && MSGSEL(b"DEFAULT", ctx)?) {
        //
        // Output the default message:
        //
        for I in 1..=DEFLEN {
            WRLINE(&DEVICE, &save.DEFMSG[I], ctx)?;
        }

        WRLINE(&DEVICE, b" ", ctx)?;
    }

    //
    // At this point, we've output all of the enabled messages
    // that were specified in LIST.  At least one message that
    // was specified was enabled.
    //
    // Write the ending border out:
    //
    WRLINE(&DEVICE, &save.BORDER, ctx)?;

    Ok(())
}
