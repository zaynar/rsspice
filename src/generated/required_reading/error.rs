//! #  Error Handling in SPICELIB
//!
//!  Last revised on 2006 NOV 22 by N.J. Bachman.
//!
//!  
//!
//!
//!  
//! ##  Abstract
//!
//!  This Required Reading document details the use of SPICELIB error
//!    handling facilities.
//!
//!  
//!
//!
//!  
//! #  Introduction
//!
//!  Most of this information in this document is OPTIONAL reading, not
//!    "required" reading. To get started using SPICELIB quickly, it suffices
//!    to read up through the section titled "The First Thing You Should
//!    Know."
//!
//!  About the organization of this Required Reading document:
//!
//!  The second chapter describes "Using SPICELIB Error Handling." We begin
//!    with brief descriptions of the error handling features, and many
//!    examples of their use.
//!
//!  The third chapter describes "Definitions and Concepts." This chapter
//!    provides a more detailed and complete discussion of the concepts used in
//!    describing the SPICELIB error handling mechanism, but generally does not
//!    give examples. Here we define what we mean by "errors," describe error
//!    messages and their use, and describe the possible error processing
//!    "actions" -- ways of responding to error conditions.
//!
//!  The fourth chapter describes "Advanced Programming With Error
//!    Handling"; that chapter discusses extension of the error reporting
//!    scheme to your own application code.
//!
//!  The document covers the following topics:
//!
//!  
//!
//! * How to make your program test whether a SPICELIB routine has detected an
//! error, and how to find out what kind of error it was.
//!
//!  * How to direct SPICELIB software to continue running after an error has
//! occurred, rather than stopping.
//!
//!  * How to select the messages you want to output when an error is detected.
//!
//!  * How to get a log of errors detected by your own application code.
//!
//!  * How to direct SPICELIB software to send error messages to a file.
//!
//!  * How to get a traceback of your call chain when an error occurs in your own
//! code.
//!
//!  Error handling topics beyond the scope of this document:
//!
//!  
//!
//! * Information about specific errors detected by SPICELIB.
//!
//!  * How to take corrective action when an error is detected.
//!
//!  * How to debug programs that use SPICELIB.
//!
//!  * How to determine whether to treat a condition arising in your own code as
//! an error.
//!
//!  * How to write code that performs the "minimum possible" amount of error
//! checking.
//!
//!  Before starting our discussion of how to use the SPICELIB error handling
//!    facilities, let's briefly answer the question, "Why perform error
//!    handling at all?" Here are a few reasons:
//!
//!  
//!
//! * It makes your program MUCH easier to debug. Most run-time diagnostics only
//! tell you about low-level types of errors such as arithmetic overflow,
//! attempts to divide by zero, or attempts to write protected memory
//! locations. These run-time errors may occur in very different locations from
//! the original coding mistakes causing them, so the original mistakes are
//! hard to find. SPICELIB error handling helps bound the section of code where
//! a coding error is located, greatly simplifying the search for it.
//!
//!  *  Also, a good deal of understanding of a program without error handling is
//! often required to fix problems, since problems are diagnosed in low-level
//! terms, not conceptually: it's like the difference between being told that
//! the current through a certain wire in your television is zero, instead of
//! being told that your set isn't plugged in. Understand many mistakes may not
//! cause run-time diagnostics to be produced.
//!
//!  * Error handling makes your program much more robust. Your program can
//! respond to errors and continue execution in many cases, with SPICELIB error
//! handling.
//!
//!  * It makes user-friendliness possible. When errors occur, your program can
//! retain control and present the problem to the user as you choose. Contrast
//! this to an abort with a run-time error message.
//!
//!  * It makes error handling portable and uniform across systems. SPICELIB error
//! handling is accomplished using standard FORTRAN. Different systems do not
//! handle run time errors in the same way.
//!
//!     
//! #  Using SPICELIB Error Handling
//!
//!  This chapter tells you how to use most of the features of the SPICELIB
//!    error handling mechanism. Many examples are given, but details are saved
//!    for the next chapter "Concepts and Definitions."
//!
//!  The material in this chapter covers three areas:
//!
//!  
//!
//! * Controlling SPICELIB's automatic error handling features
//!
//!  * How to enable your own application to handle errors detected by SPICELIB
//! code
//!
//!  * How to use SPICELIB error handling to respond to errors detected by YOUR
//! OWN code
//!
//!     
//! ##  The First Thing You Should Know
//!
//!  The first thing to know about SPICELIB error handling is, you don't HAVE
//!    to do anything to use it. As a default, when a SPICELIB routine detects
//!    an error, SPICELIB will print error messages to the default output
//!    device (the screen, on most systems), and then STOP.
//!
//!  This capability is built into SPICELIB software; you get it
//!    automatically when you link your program with SPICELIB. No additional
//!    action of any kind is needed to make it work.
//!
//!  If this behavior is adequate for your application, you don't need to
//!    read the rest of this document.
//!
//!  
//!
//!
//!  
//! ##  What SPICELIB Does About Errors, Automatically
//!
//!  When a SPICELIB routine detects an error (we call this "signaling" an
//!    error), the SPICELIB error handling mechanism will write a set of
//!    descriptive error messages to the default output device, which is the
//!    terminal screen on most systems. The messages are:
//!
//!  
//!
//! * A short, descriptive message.
//!
//!  * An expanded form of the short message.
//!
//!  * Optionally, a long message, possibly containing data.
//!
//!  * A "traceback," showing the sequence of calls leading to the routine that
//! detected the error.
//!
//!  * A default message, informing the user that the error handling behavior is
//! adjustable.
//!
//!  After writing these messages, the error handling mechanism will halt
//!    execution of the program.
//!
//!  See the section "About Error Messages" below for details.
//!
//!  
//!
//!
//!  
//! ##  Changing What SPICELIB Does About Errors
//!
//!  You can change the set of error messages written when an error occurs.
//!    You can re-direct the output to a file, or suppress it. You can choose
//!    to have your program continue after an detecting an error rather than
//!    stopping.
//!
//!  
//!
//!
//!  
//! ##  Choosing Where the Error Messages Are Sent
//!
//!  Suppose you want to have any error messages written to a file, rather
//!    than to the default output device (the screen, on most systems). You can
//!    do this by calling the SPICELIB routine, ERRDEV. The first argument
//!    should be 'SET'. The second should be the file specification.
//!
//!  For example, to have error messages go to the file, ERROR.DAT, you could
//!    use the following code:
//!
//!  
//!
//! ```text
//!    C
//!    C     Set the error output device for SPICELIB
//!    C     error messages to the file ERROR.DAT:
//!    C
//!          CALL ERRDEV ( 'SET', 'ERROR.DAT' )
//! ```
//!
//!  This call should precede calls to other SPICELIB routines, except [ERRACT](crate::raw::erract) and [ERRPRT](crate::raw::errprt). If your program has an initialization portion, this call
//!    should go there.
//!
//!  
//!
//!
//!  
//! ##  Choosing Which Error Messages to Write
//!
//!  By default, when an error is detected by a SPICELIB routine, up to five
//!    different types of error messages are written. You can tell SPICELIB's
//!    error handling mechanism to write any combination, including "all" and
//!    "none" of these messages.
//!
//!  Change the set of written messages via a call to the SPICELIB routine,
//!    [ERRPRT](crate::raw::errprt).
//!
//!  The first argument should be 'SET'.
//!
//!  The second argument is a list of message types that you want to ADD to
//!    the set of messages that currently are selected to be output when an
//!    error occurs.
//!
//!  The keywords which may be used in the list:
//!
//!  
//!
//! ```text
//!    SHORT
//!    LONG
//!    EXPLAIN
//!    TRACEBACK
//!    DEFAULT
//!    ALL         ( write all messages )
//!    NONE        ( write no messages  )
//! ```
//!
//!  The list of message choices is read from left to right, with each word
//!    ADDING to the previous set of messages (except for the word NONE, which
//!    subtracts all of the messages from your selection).
//!
//!  Some examples may help clarify this.
//!
//!  
//!
//!
//!  
//! ###  Example 1
//!
//!  Suppose that currently, the short message has been selected for output.
//!    To ADD the long message, make the call:
//!
//!  
//!
//! ```text
//!    C
//!    C     Add the long message to the set selected for output
//!    C     when an error occurs:
//!    C
//!          CALL ERRPRT ( 'SET', 'LONG' )
//! ```
//!
//!  Now the short and long messages will be output if SPICELIB detects an
//!    error.
//!
//!  
//!
//!
//!  
//! ###  Example 2
//!
//!  What if you want JUST the long message? Put the word NONE at the
//!    beginning of the list; this cancels the previous selection:
//!
//!  
//!
//! ```text
//!    C
//!    C     Just output the long message
//!    C     when an error occurs:
//!    C
//!          CALL ERRPRT (  'SET',   'NONE, LONG'  )
//! ```
//!
//!     
//! ###  Example 3
//!
//!  What if you want just the traceback and long message? Put the word NONE
//!    at the beginning of the list; this cancels the previous selection:
//!
//!  
//!
//! ```text
//!    C
//!    C     Just output the long message and traceback
//!    C     on error:
//!    C
//!          CALL ERRPRT (  'SET',   'NONE, TRACEBACK, LONG'  )
//! ```
//!
//!     
//! ###  Example 4
//!
//!  How about no messages?
//!
//!  
//!
//! ```text
//!    C
//!    C     Don't output ANY SPICELIB error messages
//!    C     on error:
//!    C
//!          CALL ERRPRT (  'SET',   'NONE'  )
//! ```
//!
//!     
//! ###  Example 5
//!
//!  All messages?
//!
//!  
//!
//! ```text
//!    C
//!    C     Output ALL SPICELIB error messages
//!    C     when an error occurs:
//!    C
//!          CALL ERRPRT (  'SET',   'ALL'  )
//! ```
//!
//!  See the section "About Error Messages" below for details.
//!
//!  
//!
//!
//!  
//! ##  Choosing the Error Response Action
//!
//!  SPICELIB's default response to error detection includes halting the
//!    program. What if you don't want that?
//!
//!  SPICELIB's error handling mechanism allows you to choose one of several
//!    different error response "actions." In almost all cases, the only
//!    reasonable alternative to the default action is RETURN. Briefly, the
//!    full set of choices consists of:
//!
//!  
//!
//! *  DEFAULT
//!
//!
//!  The action you start with, automatically. This is the same as "abort"
//! (below), except that all of the error messages, including the
//! "default" message, are written.
//!
//!  *  ABORT
//!
//!
//!  Print messages and halts the program. With this action, you can select
//! the error messages that are written.
//!
//!  *  REPORT
//!
//!
//!  Write messages and CONTINUE EXECUTION.
//!
//!  This action may be useful for debugging. Caution: Once an error has
//!    occurred, your program may not execute as expected.
//!
//!  
//!
//! *  RETURN
//!
//!
//!  Write messages, AFTER WHICH ALL SPICELIB ROUTINES THAT DETECT ERRORS OR
//! CALL OTHER ROUTINES WILL RETURN IMMEDIATELY UPON ENTRY.
//!
//!  This is the action to use if you want to perform error handling for
//!    SPICELIB errors in your own code.
//!
//!  
//!
//! *  IGNORE
//!
//!
//!  DO NOTHING in response to errors. No error messages will be output; the
//! SPICELIB status functions RETURN and FAILED will not indicate that an
//! error has occurred.
//!
//!  These choices are mutually exclusive; only one action can be in effect
//!    at a given time.
//!
//!  You use [ERRACT](crate::raw::erract) to set the error response action. The first argument
//!    should be 'SET'. The second argument should be one of the above choices.
//!    For example:
//!
//!  
//!
//! ```text
//!    C
//!    C     Set the CSPICE error response action
//!    C     to 'RETURN':
//!    C
//!          CALL ERRACT (  'SET',   'RETURN'  )
//! ```
//!
//!     
//! ##  Handling SPICELIB Errors in Your Own Program
//!
//!  So far we've talked about how to control automatic reporting of errors
//!    detected in SPICELIB routines.
//!
//!  The automatic error reporting feature is meant to produce human-
//!    readable error diagnostic information. However, you may also wish to
//!    have your program respond to SPICELIB errors.
//!
//!  To do this, you will need to know about three more basic functions:
//!    testing and resetting the SPICELIB error status, and retrieving SPICELIB
//!    error messages.
//!
//!  
//!
//!
//!  
//! ##  Testing the Error Status
//!
//!  You use the SPICELIB function, FAILED, to tell if any SPICELIB routine
//!    has detected an error. FAILED is a function which returns a boolean. It
//!    takes the 'true' value, if any SPICELIB routine has detected an error.
//!
//!  For example, suppose you call the SPICELIB routine, RDTEXT, to read a
//!    line of text from a file. You want your program to test whether an error
//!    occurred on the read. You can write:
//!
//!  
//!
//! ```text
//!    C
//!    C     Read a line from USERFILE.TXT; check for
//!    C     errors:
//!    C
//!          CALL RDTEXT ( 'USERFILE.TXT', LINE, EOF )
//!  
//!          IF (  FAILED()  )  THEN
//!  
//!    C        LINE is not valid, so quit:
//!  
//!             RETURN
//!  
//!          END IF
//! ```
//!
//!  If you're used to routines that have error arguments, you might note
//!    that the code is similar to what you would write if [FAILED](crate::raw::failed) were an
//!    output argument for RDTEXT, instead of a function.
//!
//!  However, there are a number of advantages to the SPICELIB method, one of
//!    which is that if you don't wish to write any error handling code to
//!    handle SPICELIB errors, you don't have to, and you'll still get helpful
//!    error messages automatically. Also, if you use SPICELIB error handling
//!    in your own code, you don't need error arguments, which makes for
//!    simpler code.
//!
//!  
//!
//!
//!  
//! ##  Retrieving Error Messages
//!
//!  SPICELIB provides routines to retrieve the error messages generated
//!    whenever a SPICELIB routine detects an error.
//!
//!  This feature is useful for two reasons. First, if you want your program
//!    to take different actions depending on what error occurred, it gives
//!    your program a way to find out. Second, if you want to generate your own
//!    error reports instead of using those generated by SPICELIB, you need to
//!    be able to retrieve the information SPICELIB has generated about the
//!    error.
//!
//!  
//!
//!
//!  
//! ###  Getting the short message
//!
//!  Because of its brief format, the short message is the one to use in your
//!    code in any logical tests you might want to do.
//!
//!  To retrieve the short error message, call [GETMSG](crate::raw::getmsg) or [GETSMS](crate::raw::getsms). For example:
//!
//!  
//!
//! ```text
//!    C
//!    C     Call PROMPT to prompt the user for the name of file to
//!    C     read from.  Read a line from it. Check for errors:
//!    C
//!          CALL PROMPT ( 'Enter file name > ', FILE )
//!          CALL GETFIL ( FILE )
//!  
//!          DO WHILE ( FILE .NE. ' ' )
//!  
//!             CALL RDTEXT ( FILE, LINE, EOF )
//!  
//!             IF ( FAILED() ) THEN
//!    C
//!    C           An error occurred.
//!    C           Find out what the short message was:
//!    C
//!                CALL GETMSG ( 'SHORT', SHRTMS )
//!  
//!                IF (   ( SHRTMS .EQ. 'SPICE(NOFREELOGICALUNIT)')
//!         .         .OR.( SHRTMS .EQ. 'SPICE(TOOMANYFILESOPEN)' ) ) THEN
//!    C
//!    C              We won't succeed in reading any file.
//!    C              So, quit.
//!    C
//!                   RETURN
//!  
//!                ELSE
//!    C
//!    C              Get name of a different file:
//!    C
//!                   CALL PROMPT ( 'Enter file name > ', FILE )
//!  
//!                END IF
//!  
//!             END IF
//!  
//!          END DO
//! ```
//!
//!     
//! ###  Getting the long message
//!
//!  The long error message and traceback aren't useful for program logic,
//!    but you may want them if you're going to produce error reports in your
//!    own format.
//!
//!  To get the long error message, call [GETMSG](crate::raw::getmsg) or [GETLMS](crate::raw::getlms). For example,
//!
//!  
//!
//! ```text
//!          CALL RDTEXT ( FILE, LINE, EOF )
//!  
//!          IF ( FAILED() ) THEN
//!    C
//!    C        Get long message and output it using USROUT
//!    C        (not a SPICELIB routine)
//!    C
//!             CALL GETMSG ( 'LONG', LONGMS )
//!             CALL USROUT ( LONGMS )
//!  
//!          END IF
//! ```
//!
//!  The argument supplied to [GETMSG](crate::raw::getmsg) should be declared CHARACTER*1840.
//!
//!  
//!
//!
//!  
//! ###  Getting the explanation for the short message
//!
//!  The SPICELIB routine [EXPLN](crate::raw::expln) can obtain a line of text explaining each
//!    SPICELIB short error message. This text is an expansion of the short
//!    error message, since the short message is frequently abbreviated.
//!
//!  Here's an example:
//!
//!  
//!
//! ```text
//!    C
//!    C     After this call, EXPL will take the value:
//!    C
//!    C     'Invalid Radius--Equatorial or Polar Radius is Zero'
//!    C
//!          CALL EXPLN ( 'SPICE(ZERORADIUS)', EXPL )
//!  
//!          CALL TOSTDO ( EXPLN )
//! ```
//!
//!     
//! ###  Getting the traceback
//!
//!  Two ways of getting the traceback are provided in SPICELIB:
//!
//!  
//!
//! * You can call the routine QCKTRC which returns a character string containing
//! a traceback. Specifically, the string contains a list of module names,
//! separated by arrows. The first name in the string is the name of the
//! highest level module in the active call chain that has "checked in."
//!
//!  Example: Suppose MAIN has called SUBA, which in turn has called SUBB.
//!    The following code is in SUBB:
//!
//!  
//!
//! ```text
//!    C
//!    C     Get traceback.  After the call to QCKTRC,
//!    C     TRACE should have the value,
//!    C
//!    C     'MAIN --> SUBA --> SUBB'
//!    C
//!          CALL QCKTRC ( TRACE )
//!  
//!          CALL STDIO( TRACE )
//! ```
//!
//!  * You can find out how many routines are in SPICELIB's traceback
//! representation by calling TRCDEP ("traceback depth"), and then extract
//! any given name from the traceback representation by calling TRCNAM.
//!
//!  For example, you could print out the names of all of the modules in the
//!    active call chain with the following code:
//!
//!  
//!
//! ```text
//!     CALL TRCDEP ( DEPTH )
//!  
//!     DO I = 1, DEPTH
//!  
//!        CALL TRCNAM ( I, NAME )
//!  
//!        CALL STDIO( NAME )
//!  
//!     END DO
//! ```
//!
//!     
//! ##  Resetting the Error Status
//!
//!  If your program encounters a recoverable error, you may wish to have it
//!    report the error and continue processing.
//!
//!  An example would be the case where you have an interactive program that
//!    prompts the user for the name of a file to read. Your program uses the
//!    SPICELIB function RDTEXT to read the file. If the file isn't found,
//!    RDTEXT signals an error. The inability to locate the file need not stop
//!    the program; your program could just display a message saying the file
//!    wasn't found and ask for another file name.
//!
//!  The problem here is that the SPICELIB functions FAILED and RETURN will
//!    return the 'true' value after RDTEXT signals the error, so any code
//!    whose logic depends on the value of those functions will behave as if an
//!    error has occurred, even though the error was recoverable. To solve this
//!    problem, SPICELIB provides the routine, [RESET](crate::raw::reset), which "resets" the
//!    error handling mechanism, so that it acts as if no error had occurred.
//!    Calling [RESET](crate::raw::reset) has the following effects:
//!
//!  
//!
//! * The SPICELIB function FAILED will return the 'false' value until another
//! error is signaled.
//!
//!  * The SPICELIB function RETURN will return the 'false' value until another
//! error is signaled.
//!
//!  * [GETMSG](crate::raw::getmsg), GETSMS, and [GETLMS](crate::raw::getlms) will return blank strings.
//!
//!  * The traceback routines will show a traceback of the current active call
//! chain, not the active call chain at the time of the last error.
//!
//!  When your program detects a recoverable error, it should fetch the error
//!    messages, call [RESET](crate::raw::reset), then complete its response to the error. The
//!    diagnostic messages must be fetched before [RESET](crate::raw::reset) is called. RESET should
//!    be called before other SPICE routines are called, because these routines
//!    may return upon entry while an error condition exists.
//!
//!  
//!
//!
//!  
//! ##  Handling Errors Detected in Your Own Program
//!
//!  When you use SPICELIB, you can use the error handling mechanism to
//!    respond to errors detected by your own code, as well as by SPICELIB
//!    code.
//!
//!  Some of the capabilities that you get with SPICELIB are:
//!
//!  
//!
//! * You can signal errors and have your error messages output to a chosen
//! device, without writing code to do the output.
//!
//!  * You can get a traceback of your own call chain, which can be retrieved at
//! any time, not just when an error occurs.
//!
//!  * You can control how your program behaves when it encounters an error, just
//! as when SPICELIB encounters one.
//!
//!  * You can use FAILED as a global status indicator, not just for SPICELIB
//! code.
//!
//!  * You can use RETURN to prevent your routines from executing when an error
//! condition exists, thereby obviating the need to test FAILED after each call
//! to a subroutine.
//!
//!  In the following sections we explain:
//!
//!  
//!
//! * How to "signal" (indicate) an error condition
//!
//!  * How to create error messages
//!
//!  * How to use the RETURN error response action and the SPICELIB function
//! RETURN to help your program handle recoverable errors
//!
//!  * How to use the SPICELIB routines CHKIN and CHKOUT to create a traceback of
//! your own subroutine calls
//!
//!  * How to code using FAILED so as to reduce the amount of error checking you
//! need in your code
//!
//!  * How to retrieve the settings of the error output device, error response
//! action, and error message selection
//!
//!     
//! ##  Signaling Errors
//!
//!  A routine calls [SIGERR](crate::raw::sigerr) to signal an error condition to the SPICELIB
//!    error mechanism.
//!
//!  When [SIGERR](crate::raw::sigerr) is called, all of the types of error messages that have been
//!    selected for automatic output are written out, and SPICELIB takes
//!    whatever additional actions are required by the current setting of the
//!    error response "action."
//!
//!  [SIGERR](crate::raw::sigerr) takes one input argument, a short (25 character maximum) error
//!    message. This message will be output if the "short message" has been
//!    selected for output. It is strongly recommended that your code supply a
//!    descriptive (at least non-blank) error message when it calls [SIGERR](crate::raw::sigerr).
//!
//!  The short message, if used, indicates the type of error which occurred,
//!    so the program can respond appropriately.
//!
//!  A capability exists to set a long, human-readable, error message. The
//!    next section discusses setting the long message.
//!
//!  Here's an example in which the routine, [DACOSH](crate::raw::dacosh), signals an error.
//!
//!  
//!
//! ```text
//!    C
//!    C  DACOSH computes an arc hyperbolic cosine of X; X must
//!    C  be greater than or equal to 1 to be in the domain of
//!    C  DACOSH.
//!    C
//!    C  Check that X >= 1.
//!    C
//!  
//!          IF ( X .LT. 1.D0 ) THEN
//!  
//!             CALL SETMSG ( 'DACOSH: Invalid argument, ' //
//!         .                 'X is less than one.' )
//!  
//!             CALL SIGERR ( 'SPICE(INVALIDARGUMENT)' )
//!  
//!             RETURN
//!  
//!          END IF
//! ```
//!
//!  You may note a call to the routine [SETMSG](crate::raw::setmsg) precedes the call to [SIGERR](crate::raw::sigerr) as [SETMSG](crate::raw::setmsg) sets the long error message. SETMSG is discussed in the next
//!    section, but we'll note here that if you wish to call [SETMSG](crate::raw::setmsg), it should
//!    be called BEFORE calling [SIGERR](crate::raw::sigerr), since SIGERR causes the current long
//!    error message to be output.
//!
//!  
//!
//!
//!  
//! ##  Setting the Long Error Message
//!
//!  The long error message is intended to inform the human reader about an
//!    error that has occurred.
//!
//!  You may supply a character string of length up to 1840 characters as the
//!    input argument to [SETMSG](crate::raw::setmsg). Strictly speaking, the long message is
//!    optional, but it's recommended that you call [SETMSG](crate::raw::setmsg) before every call to
//!    [SIGERR](crate::raw::sigerr), supplying a blank string if you don't wish to set a long
//!    message.
//!
//!  As an example, the calls to [SETMSG](crate::raw::setmsg) and [SIGERR](crate::raw::sigerr) from the example in the
//!    last section are repeated here:
//!
//!  
//!
//! ```text
//!     CALL SETMSG ( 'DACOSH: Invalid argument, ' //
//!    .                 'X is less than one.' )
//!  
//!  
//!     CALL SIGERR ( 'SPICE(INVALIDARGUMENT)' )
//! ```
//!
//!  Frequently, one would like to insert variable strings into a long
//!    message. In the above example, it might be nice to convert X, a double
//!    precision number, to a character string and put it in the error message.
//!    SPICELIB provides the routine, [ERRDP](crate::raw::errdp), for just this purpose.
//!
//!  [ERRDP](crate::raw::errdp) takes two arguments. The first is a character string to appear in
//!    the long error message. It marks the place where the result of the
//!    conversion is to be placed in the long error message.
//!
//!  The second argument is the value to be converted to a character string.
//!    The resulting string is substituted for the first occurrence of the
//!    first argument found in the long message.
//!
//!  Here's the previous example, re-written using [ERRDP](crate::raw::errdp).
//!
//!  
//!
//! ```text
//!    C
//!    C        Set long error message, with a MARKER where
//!    C        the value of X will go.  Our marker is '#'.
//!    C
//!             CALL SETMSG ( 'DACOSH: Invalid argument, ' //
//!         .                 'X is less than one.  The '  //
//!         .                 'value is #.'   )
//!  
//!    C
//!    C        Convert X to characters, and insert the result
//!    C        in the long message where the # is now:
//!    C
//!             CALL ERRDP  ( '#',  X )
//!  
//!    C
//!    C        If X happened to be -5.5D0, for example,
//!    C        the long message would now be:
//!    C
//!    C        'DACOSH: Invalid argument, X is less than one. '
//!    C        'The value is -5.5D0.'
//!    C
//!  
//!    C
//!    C         Signal the error:
//!    C
//!              CALL SIGERR ( 'SPICE(INVALIDARGUMENT)' )
//! ```
//!
//!  In addition to [ERRDP](crate::raw::errdp), ERRINT and [ERRCH](crate::raw::errch) are provided for inserting
//!    integers and character strings into the long message.
//!
//!  
//!
//!
//!  
//! ##  Using the RETURN Action and the SPICELIB Function, RETURN
//!
//!  If you want your program to do any error handling in addition to
//!    SPICELIB's automatic error handling, you probably would like to prevent
//!    your program from crashing as a result of an error, since this
//!    unfortunate event may prevent control from ever being given to your
//!    error handling code.
//!
//!  SPICELIB solves this problem with the boolean function RETURN, and the
//!    RETURN error response action.
//!
//!  The first two lines of executable code of every SPICELIB routine that
//!    can detect errors, or that calls another routine, are:
//!
//!  
//!
//! ```text
//!    IF ( RETURN() ) THEN
//!       RETURN
//!    END IF
//! ```
//!
//!  When the error action is RETURN and an error has been signaled, RETURN
//!    takes the 'true' value. So every SPICELIB routine that can detect
//!    errors, or that calls another routine, returns without doing anything.
//!    This greatly reduces the chance of an error causing a program crash.
//!
//!  You can use the function RETURN in your own code to achieve the same
//!    effect.
//!
//!  RETURN always takes the 'false' value if the error action is not RETURN.
//!
//!  See the next section to find out what the rest of the IF block should
//!    be.
//!
//!  
//!
//!
//!  
//! ##  Maintaining Traceback Information -- Checking In and Checking Out
//!
//!  SPICELIB can give you a "picture" of your call chain at the time an
//!    error is signaled, and at any time before. We call this a "traceback."
//!    A traceback is provided in the default selection of error messages. When
//!    an error is signaled, a traceback helps you find out what calls were
//!    made before the call to the routine which detected an error.
//!
//!  As an example, suppose the following figure shows the calling hierarchy
//!    for a program, and that currently, subroutine "E" is executing, after
//!    being called by "C."
//!
//!  
//!
//! ```text
//!  
//!         MAIN
//!       /  |  \
//!      B   C   D
//!     / \ /|   |
//!    E   F E   H
//!  
//! ```
//!
//!  The active call chain would consist of "MAIN," "C," and "E." The
//!    traceback message, if retrieved at this point, would be:
//!
//!  
//!
//! ```text
//!    MAIN --> C --> E
//! ```
//!
//!  To make your own code participate in the traceback scheme, every routine
//!    in your program (except those that call nothing else and don't detect
//!    errors) should "check in" on entry and "check out" on exit. These
//!    actions tell the error handling mechanism whether your routine is in the
//!    active call chain or not.
//!
//!  To check in, call CHKIN, supplying the name of your routine. To check
//!    out, call CHKOUT, also supplying the name of your routine. The call to
//!    CHKIN should come immediately after each entry into your code. A call to
//!    CHKOUT should precede each exit made after checking in. For example:
//!
//!  
//!
//! ```text
//!    C
//!    C     Here's a skeleton of code for a mock routine, SUBA:
//!    C
//!    C     Executable code follows
//!    C
//!    C
//!          IF ( RETURN() ) THEN
//!  
//!    C        No check out here, since we haven't checked in.
//!  
//!             RETURN
//!          ELSE
//!             CALL CHKIN ( 'SUBA' )
//!          END IF
//!  
//!                .
//!                .
//!                .
//!  
//!          IF ( X .LT. 1 ) THEN
//!    C        First exit following check in:
//!             CALL CHKOUT ( 'SUBA' )
//!             RETURN
//!           END IF
//!                .
//!                .
//!                .
//!  
//!    C     Normal exit:
//!          CALL CHKOUT ( 'SUBA' )
//!          RETURN
//!          END
//! ```
//!
//!  The traceback storage can accommodate a stack depth of 100 routines; the
//!    maximum length of each stored name is 32 characters.
//!
//!  
//!
//!
//!  
//! ##  Clean Coding Using FAILED
//!
//!  When you set the action to RETURN, all SPICELIB routines that can detect
//!    errors simply RETURN UPON ENTRY. So, they can't cause a run-time error.
//!    Therefore, you may safely call a number of SPICELIB routines
//!    consecutively, and just test FAILED after the last call in the sequence.
//!
//!  
//!
//!
//!  
//! ###  Example 1
//!
//!  
//!
//! ```text
//!    C
//!    C     Read a line from USERFILE1.TXT, USERFILE2.TXT,
//!    C     and USERFILE3.TXT; check for errors:
//!    C
//!  
//!          CALL RDTEXT ( 'USERFILE1.TXT', LINE1, EOF )
//!          CALL RDTEXT ( 'USERFILE2.TXT', LINE2, EOF )
//!          CALL RDTEXT ( 'USERFILE3.TXT', LINE3, EOF )
//!  
//!          IF (  FAILED()  )  THEN
//!  
//!    C        Not all of LINE1, LINE2, LINE3 are valid, so quit:
//!  
//!             CALL CHKOUT ( 'MYSUB' )
//!             RETURN
//!  
//!          END IF
//! ```
//!
//!     
//! ###  Example 2
//!
//!  
//!
//! ```text
//!    C
//!    C     In the following code, COPYC is used to copy the result
//!    C     of the union of two sets (ordered cells) from a temporary
//!    C     working set back into one of the original sets.
//!    C
//!          CALL UNIONC ( BODIES, PLANETS, TEMP )
//!          CALL COPYC  ( TEMP,   BODIES        )
//!  
//!          IF ( FAILED() ) THEN
//!  
//!             CALL CHKOUT ( 'MYSUB' )
//!             RETURN
//!  
//!          END IF
//!  
//!    C
//!    C     If the size of the temporary cell is greater than the size
//!    C     of the original set, FAILED should be checked to be
//!    C     sure that no overflow occurred. If BODIES is at least as
//!    C     large as TEMP, no such check is necessary.
//!    C
//! ```
//!
//!  You can also use this coding technique with calls to your own routines,
//!    if your use the function RETURN.
//!
//!  
//!
//!
//!  
//! ##  Finding Out What the Current Error Handling Settings Are
//!
//!  Below we explain how to set the error output device, the selection of
//!    error messages to output, and the error response action. You can
//!    retrieve the current settings of these items by calling the same
//!    routines you used to choose the settings, supplying the value "GET" as
//!    the first argument, instead of 'SET'.
//!
//!  For example, you find out which device error output is sent to:
//!
//!  
//!
//! ```text
//!          CALL ERRDEV ( 'GET', DEVICE )
//!  
//!    C
//!    C     DEVICE now contains the name of the output device.
//!    C
//! ```
//!
//!  To find out what the current error response action is:
//!
//!  
//!
//! ```text
//!          CALL ERRACT ( 'GET', ACTION )
//!  
//!    C
//!    C     ACTION now contains the current error response action.
//!    C
//! ```
//!
//!  To find out what the current error message selection is:
//!
//!  
//!
//! ```text
//!          CALL ERRPRT ( 'GET', LIST )
//!  
//!    C
//!    C     LIST now contains the last list of messages that
//!    C     was input by a call to ERRPRT.  If no call was made,
//!    C     LIST has the value 'DEFAULT'.
//!    C
//! ```
//!
//!     
//! #  Concepts and Definitions
//!
//!  
//!
//!
//!  
//! ##  About Errors
//!
//!  What do we mean by "errors"? In general, we mean "asking routines to
//!    do something they can't do." Examples include supplying invalid values
//!    of input arguments to a subroutine, exceeding program limits such as the
//!    maximum number of files open simultaneously, or trying to read from a
//!    file that doesn't exist.
//!
//!  When an error is detected because a routine has been used improperly,
//!    information about the context of the error is desirable. It's useful to
//!    know in which routine the error was detected, what the call chain was at
//!    the time of the error, and what the inputs to the routine that detected
//!    the error were. The SPICELIB error handling mechanism is designed to
//!    provide this type of information.
//!
//!  On the other hand, when it's the program's job to determine the
//!    correctness of data, information about the program is not what is wanted
//!    when an error occurs. In this case, information about the data is what's
//!    needed. SPICELIB's automatic error handling is not appropriate for
//!    dealing with this type of error. However, it is possible to shut off the
//!    automatic error handling, using the IGNORE error action, and use
//!    non-SPICELIB code to handle these errors.
//!
//!  In general, SPICELIB's automatic error handling is most useful for
//!    diagnosing programming errors.
//!
//!  The only errors that the SPICELIB error handling mechanism deals with
//!    are DETECTABLE ones. SPICELIB can test whether a calling routine has
//!    supplied an argument that's in the domain of a function, but it can't
//!    tell if the calling routine has the order of the arguments in a calling
//!    sequence reversed. By coincidence, an error may be detected in that
//!    case, but the diagnosis will point to the error in an indirect way, at
//!    best. And if an application uses a faulty algorithm, but nonetheless
//!    uses the SPICELIB routines correctly, SPICELIB can't tell you about it.
//!
//!  Some detectable errors exist which SPICELIB does not detect. While
//!    attempted division-by-zero errors are prevented, floating overflow is
//!    generally not prevented (because doing so is too inefficient).
//!
//!  When a SPICELIB routine detects an error, it may mean the routine is
//!    being used improperly. One of the most likely causes is an interface
//!    error: inputs may be supplied to the routine that it can't handle, or
//!    there may be an error in the coding of the call to the routine itself.
//!    It's a good idea to thoroughly understand the descriptions of inputs and
//!    outputs given in the module headers of each SPICELIB routine called by
//!    one's application.
//!
//!  Some other possible causes of errors may be: bugs in application
//!    software, bugs in SPICELIB software, or bad inputs to the application
//!    program. Errors can also occur due to problems with the program's
//!    environment. For example, an attempt to open a file could fail because
//!    the application program didn't have the privileges necessary to perform
//!    the open, or on some systems, because the file was in use by another
//!    user.
//!
//!  
//!
//!
//!  
//! ##  About Error Messages
//!
//!  SPICELIB uses error messages to inform both the application program
//!    using SPICELIB, and the user, about what type of error has occurred and
//!    where in the program it occurred.
//!
//!  SPICELIB provides routines for setting and retrieving error messages.
//!    When a routine detects an error, it may "set," or store, error
//!    messages, which then can be retrieved and examined by other routines.
//!
//!  There are five types of error messages:
//!
//!  
//!
//! * The short error message.
//!
//!  * The explanation of the short error message.
//!
//!  * The long error message.
//!
//!  * Traceback.
//!
//!  * The default message.
//!
//!     
//! ###  The short error message
//!
//!  This message is a character string containing a very terse, usually
//!    abbreviated, description of the problem. The message is a character
//!    string of length not more than 25 characters. It always has the form:
//!
//!  
//!
//! ```text
//!     SPICE(...)
//! ```
//!
//!  where the message text goes between the parentheses. An example is:
//!
//!  
//!
//! ```text
//!     SPICE(FILEOPENFAILED)
//! ```
//!
//!  The text is always composed of upper case letters and numbers.
//!
//!  Short error messages used in SPICELIB are CONSTANT, since they are
//!    intended to be used in code. That is, they don't contain any data which
//!    varies with the specific instance of the error they indicate.
//!
//!  Because of the brief format of the short error messages, it is practical
//!    to use them in a test to determine which type of error has occurred.
//!
//!  For example:
//!
//!  
//!
//! ```text
//!          CALL RDTEXT ( FILE, LINE, EOF )
//!  
//!          IF ( FAILED() ) THEN
//!    C
//!    C        An error occurred.
//!    C        Find out what the short message was:
//!    C
//!             CALL GETMSG ( 'SHORT', SHRTMS )
//!  
//!             IF (    ( SHRTMS .EQ. 'SPICE(NOFREELOGICALUNIT)' )
//!         .      .OR. ( SHRTMS .EQ. 'SPICE(TOOMANYFILESOPEN)'  ) ) THEN
//!    C
//!    C           We won't succeed in reading any file. So, quit.
//!    C
//!                RETURN
//!  
//!             ELSE
//!  
//!               .
//!               .
//!               .
//! ```
//!
//!  If you use the SPICELIB error mechanism to respond to errors detected in
//!    your own code, you may wish to use your own short error messages. The
//!    SPICELIB error handling mechanism doesn't make use of the actual content
//!    of the messages, so you may use any values that are practical. It may be
//!    of use to make up your own prefix (analogous to SPICELIB's "SPICE" ),
//!    to identify the errors as detected by your own code. Recall the
//!    25-character limit; excess characters will be truncated.
//!
//!  We recommend that you do NOT use blank short error messages. While the
//!    error handling mechanism allows it, the short error messages would no
//!    longer be useful for enabling code to determine the type of error that
//!    has occurred.
//!
//!  The short message is "set" by supplying it as an input argument to the
//!    SPICELIB routine [SIGERR](crate::raw::sigerr). It is retrieved by calling [GETMSG](crate::raw::getmsg) or [GETSMS](crate::raw::getsms).
//!
//!  
//!
//!
//!  
//! ###  The explanation of the short error message
//!
//!  An 80-character expansion of the short message exists for a subset of
//!    SPICELIB's short messages. Each explanation message is constant. While
//!    the short message is meant to be used in code, the explanation is
//!    supposed to be easily interpreted by the human reader.
//!
//!  SPICELIB provides the routine [EXPLN](crate::raw::expln) to map short error messages to their
//!    explanations.
//!
//!  Currently, there is no provision to extend the mapping to user-defined
//!    short messages.
//!
//!  In future versions of SPICELIB, more space may be allocated for
//!    explanations. However, [EXPLN](crate::raw::expln) will continue to return the first
//!    80-character line of the explanation text in that case.
//!
//!  Here's an example of a short message and the corresponding explanation:
//!
//!  
//!
//! ```text
//!    Short message: 'SPICE(ZERORADIUS)'
//!  
//!    Explanation:   'Invalid Radius--Equatorial or Polar Radius is Zero'
//! ```
//!
//!     
//! ###  The long error message
//!
//!  This message may be up to 1840 characters long. The SPICELIB error
//!    handling mechanism makes no use of its contents. Its purpose is to
//!    provide human-readable information about errors.
//!
//!  Long error messages generated by SPICELIB routines often contain data
//!    relevant to the specific error they describe. Here's an example of a
//!    long error message generated by a misspelled body name provided as an
//!    input to the SPICELIB routine [SPKEZR](crate::raw::spkezr):
//!
//!  
//!
//! ```text
//!    The observer, 'earthh', is not a recognized name for an
//!    ephemeris object. The cause of this problem may be that
//!    you need an updated version of the SPICE toolkit. Alternatively
//!    you may call SPKEZ directly if you know the SPICE ID codes
//!    for both 'moon' and 'earthh'
//! ```
//!
//!  The long message is "set" by supplying it as an input argument to the
//!    SPICELIB routine [SETMSG](crate::raw::setmsg). It may be retrieved by calling [GETMSG](crate::raw::getmsg) or [GETLMS](crate::raw::getlms).
//!
//!  The routines, [ERRDP](crate::raw::errdp), ERRINT, and [ERRCH](crate::raw::errch) provide the capability to insert
//!    data into the long message. Their respective purposes are to insert
//!    DOUBLE PRECISION, INTEGER, and CHARACTER data into the long message.
//!    They (except [ERRCH](crate::raw::errch)) convert the input data to a character string, and
//!    insert it in the current long message at a location indicated by a
//!    user-specified marker.
//!
//!  We strongly recommend that you DO NOT write code that tests for
//!    particular values of SPICELIB long error messages; this is a very
//!    error-prone practice.
//!
//!  
//!
//!
//!  
//! ###  The traceback
//!
//!  The purpose of this message is to represent the active call chain, that
//!    is, the set of subroutines in your program that have been called and
//!    have not yet returned. The traceback will always show the SPICELIB
//!    routines in the active call chain.
//!
//!  You can have your own code participate in the SPICELIB traceback scheme,
//!    so that the traceback will represent the entire active call chain.
//!
//!  Knowledge of the active call chain can be a valuable debugging aid,
//!    because it helps you determine where in your program an error occurred.
//!    For example, if your program calls subroutine SUBX from ten different
//!    subroutines, it's not enough to know that SUBX detected an error; you
//!    want to know which subroutine made the offending call to SUBX. The
//!    traceback contains that information. Another example: suppose a SPICELIB
//!    routine in your program detects an error, but your own code does not
//!    call that routine. You need to know which call, made by your own code to
//!    a SPICELIB routine, eventually resulted in the call to the routine that
//!    detected the error. The traceback shows you this.
//!
//!  The SPICELIB error handling mechanism automatically keeps track of which
//!    SPICELIB routines are in the active call chain. In order for the
//!    SPICELIB error handling mechanism to represent the entire active call
//!    chain, including non-SPICELIB routines, it is necessary for each routine
//!    to tell the error handling mechanism: 1) when it has been called and 2)
//!    when it is about to return. In SPICELIB documentation, these two
//!    operations are called "checking in" and "checking out." The set of
//!    routines that have checked in but have not yet checked out constitute
//!    the portion of the active call chain that the SPICELIB error handling
//!    mechanism can represent.
//!
//!  SPICELIB provides the two routines CHKIN and CHKOUT for this purpose;
//!    CHKIN for checking in, and CHKOUT for checking out. CHKIN and CHKOUT
//!    take one input argument: the name of the calling routine.
//!
//!  The traceback message has the form of a list of subroutine names,
//!    delimited by arrows. The first name in the list is the highest-level
//!    routine in the portion of the active call chain that is known to the
//!    SPICELIB error handling mechanism.
//!
//!  As an example, suppose the following figure shows the calling hierarchy
//!    for a program, and that currently, subroutine "E" is executing, after
//!    being called by "C."
//!
//!  
//!
//! ```text
//!  
//!         MAIN
//!       /  |  \
//!      B   C   D
//!     / \ /|   |
//!    E   F E   H
//!  
//! ```
//!
//!  The active call chain would consist of "MAIN," "C," and "E." The
//!    traceback message, if retrieved at this point, would be:
//!
//!  
//!
//! ```text
//!    MAIN --> C --> E
//! ```
//!
//!  The particular traceback information made available by SPICELIB is
//!    dependent on the error action and on whether an error has occurred. In
//!    general, the routines QCKTRC, TRCDEP, and TRCNAM return information
//!    about the current active call chain. But when the error response action
//!    is RETURN, and an error occurs, the error handling mechanism captures a
//!    "frozen" copy of the traceback; from this point on, the trace
//!    information provided by the above routines will be based on the frozen
//!    copy. The purpose of this behavior is to make the traceback that existed
//!    at the time the error occurred available to the application program.
//!    Changing the error action to a value other than RETURN, or resetting the
//!    error status, will cause the traceback routines QCKTRC, TRCDEP, and
//!    TRCNAM to return data based on the current traceback again.
//!
//!  The overhead of checking in and checking out may be prohibitive for some
//!    routines. An intermediate position between not using CHKIN and CHKOUT
//!    and using them in every routine is to use them only in routines that
//!    call other routines, or that detect errors. Routines which do neither
//!    would never appear in a traceback anyway. Note that absence of error
//!    detection in a routine is not sufficient grounds for exclusion from
//!    checking in and checking out, since a routine called by the routine in
//!    question could detect an error, and then the caller should appear in the
//!    traceback.
//!
//!  It is important to note that ONLY routines which have checked in can
//!    possibly have their names appear in the traceback. So the traceback will
//!    not be accurate if any routines in the active call chain have not
//!    "checked in," when a traceback message is produced.
//!
//!  The traceback mechanism requires any "checked-in" routine to check out
//!    before exit.
//!
//!  
//!
//!
//!  
//! ###  The default message
//!
//!  This message is output when an error occurs, and the error response
//!    action is DEFAULT. It informs the user that the behavior of SPICELIB
//!    error handling is user-tailorable, and it refers the user to this
//!    document.
//!
//!  
//!
//!
//!  
//! ##  About Error Response Actions
//!
//!  The term "error response action" refers to the action that the
//!    SPICELIB error handling mechanism takes automatically when an error is
//!    signaled. SPICELIB provides the routine [ERRACT](crate::raw::erract) to enable you to select
//!    the error response action.
//!
//!  There are five different error response actions, each appropriate to a
//!    different type of programming situation:
//!
//!  
//!
//! *  DEFAULT
//!
//!
//!  This is what you get if you don't choose an error response action. When
//! the error action is DEFAULT, and an error is signaled, the SPICELIB
//! error handling mechanism outputs all of the available types of error
//! messages to the error output device, and then stops program execution.
//!
//!  The default action is probably acceptable as a response to detection of
//!    program bugs. However, if you wish to allow your own program to respond
//!    to errors, you must choose one of REPORT, RETURN, or IGNORE as the error
//!    response action.
//!
//!  
//!
//! *  ABORT
//!
//!
//!  This action is the same as the default, except that the set of error
//! messages is selectable. This action is useful for running programs in
//! batch mode, to prevent them from continuing execution after
//! encountering an error. The ABORT action is also useful if your program
//! does not perform any error handling itself.
//!
//!  *  REPORT
//!
//!
//!  This action can be useful for debugging. With the REPORT action, the
//! SPICELIB error handling mechanism outputs error messages when an error
//! is signaled, but doesn't stop your program. So, more information can be
//! gained from a single run of your program than with the ABORT action.
//! Using the SPICELIB routine ERRDEV, you can direct the error messages
//! output by the SPICELIB error handling mechanism to a file, thus
//! creating a log of your errors.
//!
//!  However, the function [FAILED](crate::raw::failed) will indicate an error condition after an
//!    error has been signaled. Therefore, any program logic that depends on
//!    the value returned by [FAILED](crate::raw::failed) may be affected by an error. You can use
//!    the SPICELIB routine [RESET](crate::raw::reset) to set the error status to "no error," so
//!    that your program will attempt to follow a normal thread of execution,
//!    after an error is signaled.
//!
//!  
//!
//! *  RETURN
//!
//!
//!  The RETURN action's intend use is in situations where you want your
//! program to recover from errors detected by SPICELIB (or by your own
//! code). Like the REPORT action, the RETURN action also includes writing
//! error messages to the chosen output device. But with the RETURN action,
//! the SPICELIB function RETURN takes the 'true' value when an error is
//! signaled. Any routine that tests the function RETURN on entry, and that
//! returns immediately if RETURN takes the 'true' value, will not crash in
//! this case. All SPICELIB routines that can detect errors, or that call
//! other routines, do test RETURN on entry and return if the 'true' value.
//! This behavior maximizes the chance that SPICELIB routines will return
//! control to the application program using them, in the event of an
//! error.
//!
//!  It is also possible to use the function RETURN in the same way in your
//!    own routines. We note here that use of RETURN mode allows you to call
//!    multiple SPICELIB routines consecutively, without testing the error
//!    status until after the last call. This can greatly enhance the
//!    readability of your code, and increase its efficiency, as well.
//!
//!  Returning control to the non SPICELIB portion of the program allows for
//!    a response to the error condition. The nature of the response is
//!    application-dependent. However, in the case that the error does not
//!    preclude further successful operation of the program, it is necessary to
//!    instruct the SPICELIB error handling mechanism that normal operation is
//!    desired, after the error has been responded to. The SPICELIB function
//!    [RESET](crate::raw::reset) is provided for this purpose. Calling RESET blanks out all stored
//!    error messages, and causes the functions FAILED and RETURN to take the
//!    'false' value.
//!
//!  
//!
//! *  IGNORE
//!
//!
//!  This action is provided to allow you to completely "shut off"
//! SPICELIB's automatic error handling. When this action is selected, and
//! any routine signals an error, no action is taken by the SPICELIB error
//! handling mechanism. The error messages cannot be updated by [SIGERR](crate::raw::sigerr) and
//! [SETMSG](crate::raw::setmsg), and RETURN and FAILED will not change their values.
//!
//!  This action may be useful in the case where you know exactly which
//!    errors will be detected by SPICELIB routines, and you know that no
//!    response is required.
//!
//!  Note that the IGNORE action is not appropriate for the case where you
//!    wish to suppress automatic output of SPICELIB error messages, and to
//!    handle SPICELIB errors in your own code. To do that, use the RETURN
//!    error action and set the selection of error messages to 'NONE', using
//!    [ERRPRT](crate::raw::errprt).
//!
//!  
//!
//!
//!  
//! #  Advanced Programming with Error Handling
//!
//!  This chapter discusses some further error handling topics.
//!
//!  
//!
//!
//!  
//! ##  Using the SPICELIB Functions FAILED and RETURN
//!
//!  First, note that the SPICELIB function RETURN takes the 'false' value
//!    unless the error response action is RETURN, so all of the following
//!    discussion assumes that the error action has been set to RETURN.
//!
//!  Which routines should use the function RETURN, and where?
//!
//!  The "which?" question is somewhat tricky: there's a trade-off between
//!    speed and resistance to program crashes. It takes time (not much) to
//!    test RETURN, and in a very short routine, such as one that computes a
//!    dot product, the execution time can be increased by a substantial factor
//!    if RETURN is tested. In most routines, the percentage increase in
//!    execution time is small.
//!
//!  So, if you don't want to test RETURN in every routine, how do you
//!    decide? The SPICELIB answer to this question is, test RETURN in routines
//!    that call other routines or detect errors (SPICELIB routines that are
//!    specifically intended to perform error handling are exempt from testing
//!    RETURN). If a routine calls another routine already, it's unlikely that
//!    testing RETURN will slow its execution intolerably. Similarly for
//!    routines that test for error conditions.
//!
//!  Our final answer is still a hedge: the proper use of RETURN depends on
//!    your speed and reliability requirements. The SPICELIB method may be a
//!    good zero-order approximation to what's optimal.
//!
//!  NOTE: You must be very careful about using RETURN in application code
//!    that DOES error handling, otherwise your error handling code itself may
//!    not function when an error is signaled.
//!
//!  The "where?" question also eludes a straightforward answer. The idea
//!    behind the RETURN action is to ensure that control is returned to
//!    whatever part of your program handles errors, when an error is signaled.
//!    According to this idea, once an error has been signaled, you have to do
//!    something about the error before you can finish the job. So, routines
//!    that do use RETURN ought to test it at the very beginning of their
//!    executable code; there's no reason for them to proceed further until the
//!    error condition has been acted upon.
//!
//!  What about using the function RETURN in other parts of the code, for
//!    instance following calls to SPICELIB routines? The same speed vs crash
//!    resistance trade-off applies, but there is another consideration: RETURN
//!    has an effect only when the error action is RETURN. So, in cases where
//!    an error condition makes it likely that a portion of code won't work,
//!    it's usually better to test the function FAILED, rather than RETURN,
//!    since a program crash is usually undesirable, even when using the REPORT
//!    action for debugging. But if you do want to retain the option of
//!    executing the code when an error has already occurred, use RETURN.
//!
//!  We repeat here that the SPICELIB function FAILED indicates whether an
//!    error has been signaled, regardless of the error response action. It can
//!    be used to avoid executing any code that may cause a run-time error if
//!    an error has already occurred. Note that testing FAILED instead of
//!    RETURN means that whether the error response action is RETURN or REPORT
//!    doesn't affect the program logic in question; the logic depends on
//!    whether or not an error has been signaled.
//!
//!  
//!
//!
//!  
//! ##  Using the SPICELIB routines CHKIN and CHKOUT
//!
//!  The simplest approach to the use of these routines is to use them in
//!    every routine you write. But as with the case of RETURN, your routines
//!    will be slightly slower if they call CHKIN and CHKOUT, and for very
//!    short routines, the percentage difference may be great.
//!
//!  The SPICELIB approach is to call CHKIN and CHKOUT in routines which call
//!    other routines or which can detect errors. This way, every routine that
//!    can possibly be in the active call chain at the time an error is
//!    detected can appear in the traceback message.
//!
//!  As mentioned in the previous section, these routines are less likely to
//!    suffer a large percentage increase in execution time as a result of
//!    making extra calls.
//!
//!  We note that some routines that do make external references but do not
//!    signal errors may have no routines capable of signaling errors below
//!    them in the calling hierarchy; these routines won't appear in a
//!    traceback. Why should they call CHKIN and CHKOUT? They should do so for
//!    maintenance reasons, specifically, if any routine below them in the
//!    calling hierarchy is changed so as to be able to detect an error, the
//!    higher level routines don't need to be changed.
//!
//!  
//!
//!
//!  
//! ##  Using Multiple Settings of the Automatic Error Handling Features
//!
//!  In the second chapter, we suggested that you choose the error output
//!    device, error response action, and selection of error messages, during
//!    program initialization and leave them unchanged for the duration of the
//!    program run. This is probably a good procedure to stick to for
//!    inexperienced users. However, it's possible and reasonable to do some
//!    fancier things with these settings, in some cases:
//!
//!  
//!
//! * Resetting the output device. This isn't particularly fancy; you may simply
//! want to write error messages from different parts of your program to
//! different devices. ERRDEV may be called from anywhere in your program, as
//! many times as your wish. Note that the error response action should not be
//! DEFAULT or ABORT, for in this case the first error signaled terminates
//! execution.
//!
//!  * Resetting the error message selection. Nothing to it, just call [ERRPRT](crate::raw::errprt) any
//! time. Again, note that the error response action should not be DEFAULT or
//! ABORT, for in this case the first error signaled terminates execution.
//!
//!  * Resetting the error response action. [ERRACT](crate::raw::erract) can also be called at any time.
//! However, it is important to be aware of the special effects of these error
//! response action "transitions." Specifically:
//!
//!  *  When making a transition to a new action, the value of FAILED does not
//! change. If FAILED has the 'true' value, and you set the action to IGNORE,
//! any NEW errors that are signaled will be ignored, but FAILED will remain
//! 'true'.
//!
//!  *  The value of the function RETURN depends on FAILED and the current action.
//! Specifically, RETURN has the 'true' value if and only if FAILED has the
//! 'true' value and the action is RETURN. Therefore, note that if FAILED has
//! the 'true' value and you change the action to RETURN, the function RETURN
//! will now have the 'true' value, regardless of its previous value. Setting
//! the action to something other than RETURN will cause RETURN to take the
//! 'false' value, regardless of its previous value.
//!
//!  *  Changing the error action doesn't affect the stored short and long error
//! messages, if any. If there are stored messages, they will still be
//! available if the action is changed to IGNORE.
//!
//!  *  The traceback routines return information about the "frozen" traceback if
//! and only if RETURN has the 'true' value. So the rule about which traceback
//! you get is the same as the rule for the value of RETURN.
//!
//!  *  Changing the action to ABORT or DEFAULT doesn't automatically abort your
//! program if FAILED has the 'true' value; an error must be signaled while the
//! action is one of these values to achieve an abort.
//!
//!  * Temporary resetting of the error handling selections. What if you want to
//! set the error action to ABORT or IGNORE while a certain section of code
//! executes, and then set it back to its original value? Easy: call [ERRACT](crate::raw::erract)
//! with the "GET" option to get the current error action. Save this value,
//! call [ERRACT](crate::raw::erract) with the "SET" option to set the new value, and finally set
//! the action back to the saved value.
//!
//!  For example:
//!
//!  
//!
//! ```text
//!    C
//!    C     All errors in the following section of code are
//!    C     fatal.  Temporarily set the error action to
//!    C     'ABORT'; restore old action after end of code
//!    C     section.
//!    C
//!  
//!          CALL ERRACT ( 'GET', SAVACT  )
//!  
//!          CALL ERRACT ( 'SET', 'ABORT' )
//!  
//!  
//!     { Section of code in which errors are fatal starts here }
//!  
//!                           .
//!                           .
//!                           .
//!  
//!  
//!     { Section of code in which errors are fatal ends here }
//!  
//!  
//!    C
//!    C     Restore old error action:
//!    C
//!          CALL ERRACT ( 'SET', SAVACT )
//! ```
//!
//!  Changes to the error output device or error message selection can be
//!    made in the same way; both ERRDEV and [ERRPRT](crate::raw::errprt) accept the values "GET"
//!    and "SET" for their first argument.
//!
//!  
//!
//!
//!     
