//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
pub const MXSPEC: i32 = 512;
const MAXCHR: i32 = 255;
const BLANK: i32 = 32;
const MINPRT: i32 = 32;
const MAXPRT: i32 = 126;
const NHPOS: i32 = 1;
const NTPOS: i32 = 2;
const HCPOS: i32 = 3;

/// Lex names
///
/// Umbrella routine for name scanning entry points.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  ENTRY POINTS
///  --------  ---  --------------------------------------------------
///  HDCHRS     I   LXCSID
///  TLCHRS     I   LXCSID
///  STRING     I   LXIDNT
///  FIRST      I   LXIDNT
///  IDSPEC    I-O  LXDFID, LXCSID, LXIDNT
///  LAST       O   LXIDNT
///  NCHAR      O   LXIDNT
///  MXSPEC     P   LXDFID, LXCSID
///  LBCELL     P   LXIDNT, LXDFID, LXCSID
/// ```
///
/// # Detailed Input
///
/// ```text
///  See the entry points for descriptions of their inputs.
/// ```
///
/// # Detailed Output
///
/// ```text
///  See the entry points for descriptions of their outputs.
/// ```
///
/// # Parameters
///
/// ```text
///  See the entry points for descriptions of their parameters.
/// ```
///
/// # Exceptions
///
/// ```text
///  See the entry points for descriptions of the exceptions specific
///  to those entry points.
///
///  1)  If this routine is called directly, the error
///      SPICE(BOGUSENTRY) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  Many computer languages include tokens that represent names.
///  Examples of names include procedure names and variable names.
///  The term `identifier' is generally used to indicate this type
///  of token. Rules for constructing identifiers vary from
///  language to language, but identifiers conforming to the
///  following rules are widely recognized:
///
///     1)  The first character of the identifier is a letter.
///
///     2)  The remaining characters are letters or numbers.
///
///     3)  The length of the identifier is less than some specified
///         limit.
///
///  This suite of routines has its own set of default rules for
///  forming identifiers. These rules are somewhat more liberal
///  than those listed above. Rule (1) above still holds, but
///  trailing characters may include letters, numbers, and the
///  special characters
///
///     $
///     _  (underscore)
///
///  No mechanism for enforcing rule (3) is provided; this task is
///  left to the caller, since this routine would be unnecessarily
///  complicated by the need to construct diagnostic messages.
///
///  The entry point LXIDNT (Lex identifier) recognizes valid
///  identifier tokens, using either the default character sets
///  for the head and tail of the identifier, or character sets
///  specified in the last call to LXCSID.
///
///  In order to use this suite of routines to scan identifiers that
///  conform to the default rules, a program normally calls the entry
///  point LXDFID (Lex, default identifier specification) once to
///  obtain the default `identifier specification'. This specification
///  is an integer array in which the allowed head and tail character
///  sets are specified. This specification is then saved and supplied
///  to the entry point LXIDNT (Lex identifier) whenever LXIDNT is
///  called to scan an identifier. The entry point LXIDNT  recognizes
///  valid identifier tokens, using an input identifier specification
///  to decide which head and tail characters are allowed in an
///  identifier.
///
///  The scanning code using these routines might have the following
///  structure:
///
///
///           INTEGER               IDSPEC ( LBCELL : MXSPEC )
///              .
///              .
///              .
///     C
///     C     Initialize the identifier specification, using the
///     C     default:
///     C
///           CALL SSIZEI ( MXSPEC, IDSPEC )
///           CALL LXDFID ( IDSPEC )
///              .
///              .
///              .
///     C
///     C     Scan string:
///     C
///           DO WHILE ( <more tokens> )
///                    .
///                    .
///                    .
///              IF ( <test for identifier> ) THEN
///
///                 CALL LXIDNT ( IDSPEC, STRING, FIRST, LAST, NCHARS )
///
///                 IF ( NCHARS .GT. 0 ) THEN
///
///                    [Identifier was found--process result]
///
///                 ELSE
///
///                    [Token at starting at location FIRST was not
///                     an identifier--handle alternatives]
///
///                 END IF
///
///              ELSE
///
///                 [ perform tests for other tokens ]
///
///              END IF
///
///           END DO
///
///
///  It is possible to override the default rules by calling the
///  entry point LXCSID (Lex, custom identifier characters).  This
///  routine allows the caller to specify the precise set of
///  characters allowed as the first character (`head') of the
///  identifier, as well as those allowed in the remainder (`tail')
///  of the identifier.
///
///  If a custom identifier specification is desired, the call to
///  LXDFID in the pseudo code above would be replaced by a call to
///  LXCSID. After setting the strings HDCHRS and TLCHRS to contain,
///  respectively, the allowed head and tail identifier characters, the
///  following call would produce an identifier specification structure
///  IDSPEC representing these set of allowed characters.
///
///     CALL LXCSID ( HDCHRS, TLCHRS, IDSPEC )
///
///  The array IDSPEC obtained from LXCSID would be used as input to
///  LXIDNT, instead of using the array obtained by calling LXDFID.
/// ```
///
/// # Examples
///
/// ```text
///  1)  The following table illustrates the behavior of the scanning
///      entry point LXIDNT when the default identifier syntax is in
///      effect:
///
///      STRING CONTENTS             FIRST   LAST   NCHAR
///      ==========================================================
///      WHERE A LT B                1       5      5
///      WHERE A LT B                7       7      1
///      WHERE A.LT.B                7       7      1
///      WHERE (A0)LT(B8)            8       9      2
///      WHERE A0$LT_B7              7       14     8
///      WHERE A LT B                12      12     1
///      WHERE A .LT. B              9       8      0
///
///
///  2)  The following table illustrates the behavior of the scanning
///      entry point LXIDNT when a custom identifier syntax is used.
///      The call
///
///         CALL LXCSID ( HDCHRS, TLCHRS, IDSPEC )
///
///      where
///
///         HDCHRS = 'abcdefghijklmnopqrstuvwxyz'
///
///      and
///
///         TLCHRS = 'abcdefghijklmnopqrstuvwxyz012345.'
///
///     will produce an identifier specification IDSPEC that,
///     when supplied as an input to LXIDNT, will cause LXIDNT
///     to perform in accordance with the table shown below:
///
///
///      STRING CONTENTS             FIRST   LAST   NCHAR
///      ==========================================================
///      WHERE A LT B                1       0      0
///      where a lt b                1       5      5
///      WHERE a LT b                7       7      1
///      WHERE a.LT.b                7       8      2
///      WHERE (a0)LT(b8)            14      14     1
///      WHERE (a0)LT(b5)            14      15     2
///      WHERE a0.lt.b8              7       13     7
///      WHERE a0$lt_b7              7       8      2
///      where a .lt. b              9       12     4
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
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
/// -    SPICELIB Version 1.0.1, 10-FEB-2014 (BVS)
///
///         Added LBCELL to the $Brief_I/O section.
///
/// -    SPICELIB Version 1.0.0, 25-OCT-1995 (NJB)
/// ```
pub fn lxname(
    ctx: &mut SpiceContext,
    hdchrs: &str,
    tlchrs: &str,
    string: &str,
    first: i32,
    last: i32,
    idspec: &[i32],
    nchar: i32,
) -> crate::Result<()> {
    LXNAME(
        hdchrs.as_bytes(),
        tlchrs.as_bytes(),
        string.as_bytes(),
        first,
        last,
        idspec,
        nchar,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure LXNAME ( Lex names )
pub fn LXNAME(
    HDCHRS: &[u8],
    TLCHRS: &[u8],
    STRING: &[u8],
    FIRST: i32,
    LAST: i32,
    IDSPEC: &[i32],
    NCHAR: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // IDSPEC parameters:
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"LXNAME", ctx)?;
    }

    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;

    CHKOUT(b"LXNAME", ctx)?;
    Ok(())
}

/// Lex identifier
///
/// Scan an identifier, starting from a specified character
/// position.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  IDSPEC     I   Identifier character specification.
///  STRING     I   String to be scanned.
///  FIRST      I   Character position at which to start scanning.
///  LAST       O   Character position of end of token.
///  NCHAR      O   Number of characters in token.
///  LBCELL     P   The SPICE cell lower bound.
/// ```
///
/// # Detailed Input
///
/// ```text
///  IDSPEC   is an integer cell containing a specification of
///           the head and tail identifier character sets to be
///           used in scanning the input argument STRING. IDSPEC
///           should be obtained by calling LXDFID or LXCSID.
///           The structure of IDSPEC is not part of the
///           specification of this routine suite and should not
///           be relied upon by calling code.
///
///  STRING   is a character string that may contain an
///           `identifier' starting at the character position
///           indicated by the input argument FIRST (see
///           below). Identifier tokens are sequences of
///           characters that represent names. Syntactically, an
///           identifier is a sequence of characters that begins
///           with a character belonging to a set of valid `head'
///           characters and is followed by zero or more
///           characters belonging to a set of valid `tail'
///           characters.
///
///  FIRST    is the character position at which the routine
///           is to start scanning an identifier. Note
///           that the character STRING(FIRST:FIRST) must be a
///           valid head character if an identifier is to
///           be found; this routine does *not* attempt to locate
///           the first identifier following the position
///           FIRST.
/// ```
///
/// # Detailed Output
///
/// ```text
///  LAST     is the last character position such that the
///           substring STRING(FIRST:LAST) is an identifier, if
///           such a substring exists. Otherwise, the
///           returned value of LAST is FIRST-1.
///
///  NCHAR    is the length of the identifier found by this
///           routine, if such a token exists. If an identifier
///           is not found, the returned value of NCHAR is
///           zero.
/// ```
///
/// # Parameters
///
/// ```text
///  LBCELL   is the SPICE cell lower bound.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If the input argument FIRST is less than 1 or greater than
///      LEN(STRING)-1, the returned value of LAST is FIRST-1, and the
///      returned value of NCHAR is zero.
/// ```
///
/// # Particulars
///
/// ```text
///  The default syntax rules for valid identifiers are specified in
///  the $Particulars section of the umbrella routine LXNAME. These
///  rules may be overridden by calling LXCSID.
/// ```
///
/// # Examples
///
/// ```text
///  See the $Examples section of the umbrella routine LXNAME.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
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
/// -    SPICELIB Version 1.0.1, 10-FEB-2014 (BVS)
///
///         Added LBCELL to the $Declarations, $Brief_I/O, and $Parameters
///         sections.
///
/// -    SPICELIB Version 1.0.0, 25-OCT-1995 (NJB)
/// ```
pub fn lxidnt(idspec: &[i32], string: &str, first: i32, last: &mut i32, nchar: &mut i32) {
    LXIDNT(idspec, string.as_bytes(), first, last, nchar);
}

//$Procedure LXIDNT ( Lex identifier )
pub fn LXIDNT(IDSPEC: &[i32], STRING: &[u8], FIRST: i32, LAST: &mut i32, NCHAR: &mut i32) {
    let IDSPEC = DummyArray::new(IDSPEC, LBCELL..);
    let mut C: i32 = 0;
    let mut I: i32 = 0;
    let mut L: i32 = 0;
    let mut NHEAD: i32 = 0;
    let mut NTAIL: i32 = 0;
    let mut TCPOS: i32 = 0;

    //
    // No check-in required; this entry point is error-free.
    //

    //
    // Save the length of the non-blank prefix of the input string.
    //
    L = RTRIM(STRING);

    //
    // Handle the cases in which we can tell right away that
    // no token can be found.
    //
    if ((FIRST < 1) || (FIRST > L)) {
        *LAST = (FIRST - 1);
        *NCHAR = 0;
        return;
    }

    //
    // In order for there to be a match, the character at position
    // FIRST must be in the head character set.
    //
    NHEAD = IDSPEC[NHPOS];

    C = intrinsics::ICHAR(fstr::substr(STRING, FIRST..=FIRST));
    I = BSRCHI(C, NHEAD, IDSPEC.subarray(HCPOS));

    if (I == 0) {
        *LAST = (FIRST - 1);
        *NCHAR = 0;
        return;
    }

    //
    // We have an identifier.  The remaining question is how long it is.
    // Each subsequent character that is in the tail character set is
    // considered to be part of the identifier.
    //
    *NCHAR = 1;
    *LAST = FIRST;
    NTAIL = IDSPEC[NTPOS];
    TCPOS = (3 + NHEAD);

    while (*LAST < L) {
        C = intrinsics::ICHAR(fstr::substr(STRING, (*LAST + 1)..=(*LAST + 1)));
        I = BSRCHI(C, NTAIL, IDSPEC.subarray(TCPOS));

        if (I == 0) {
            return;
        } else {
            *NCHAR = (*NCHAR + 1);
            *LAST = (*LAST + 1);
        }
    }
}

/// Lex, default identifier characters
///
/// Return the default specification for the characters that may
/// appear in an identifier.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  IDSPEC    I-O  Identifier character specification.
///  MXSPEC     P   Recommended size for declaration of IDSPEC.
///  LBCELL     P   The SPICE cell lower bound.
/// ```
///
/// # Detailed Input
///
/// ```text
///  IDSPEC   is an integer cell. The caller must initialize
///           IDSPEC as a cell, and should use MXSPEC as the size
///           of IDSPEC.
/// ```
///
/// # Detailed Output
///
/// ```text
///  IDSPEC   is an integer cell containing a specification of
///           the head and tail identifier character sets to be
///           used the entry point LXIDNT in scanning strings.
/// ```
///
/// # Parameters
///
/// ```text
///  MXSPEC   is the recommended size for the declaration of
///           IDSPEC; the caller should declare IDSPEC as shown:
///
///              INTEGER       IDSPEC ( LBCELL : MXSPEC )
///
///           The caller should also initialize IDSPEC as shown:
///
///              CALL SSIZEI ( MXSPEC, IDSPEC )
///
///  LBCELL   is the SPICE cell lower bound.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If IDSPEC is not properly initialized on input, or if its size
///      is too small, an error is signaled by a routine in the call
///      tree of this routine. IDSPEC is undefined on output in this
///      case.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine allows a calling program to obtain the default set of
///  allowed patterns for identifiers recognized by LXIDNT.
///
///  Normally, this routine should be called once during the calling
///  program's initialization.
/// ```
///
/// # Examples
///
/// ```text
///  See the $Examples section of the umbrella routine LXNAME.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 27-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.1, 10-FEB-2014 (BVS)
///
///         Added LBCELL to the $Declarations, $Brief_I/O, and $Parameters
///         sections.
///
/// -    SPICELIB Version 1.0.0, 25-OCT-1995 (NJB)
/// ```
pub fn lxdfid(ctx: &mut SpiceContext, idspec: &mut [i32]) -> crate::Result<()> {
    LXDFID(idspec, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure LXDFID ( Lex, default identifier characters )
pub fn LXDFID(IDSPEC: &mut [i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut IDSPEC = DummyArrayMut::new(IDSPEC, LBCELL..);
    let mut HEADC = ActualArray::<i32>::new(LBCELL..=MAXCHR);
    let mut I: i32 = 0;
    let mut NHEAD: i32 = 0;
    let mut NTAIL: i32 = 0;
    let mut TAILC = ActualArray::<i32>::new(LBCELL..=MAXCHR);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"LXDFID", ctx)?;
    }

    //
    // Initialize our head and tail character sets.
    //
    SSIZEI(MAXCHR, HEADC.as_slice_mut(), ctx)?;
    SSIZEI(MAXCHR, TAILC.as_slice_mut(), ctx)?;

    //
    // Fill in the head and tail character arrays with their default
    // values.  User integer codes for the characters.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = 26;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            HEADC[I] = ((intrinsics::ICHAR(b"A") + I) - 1);
            HEADC[(I + 26)] = ((intrinsics::ICHAR(b"a") + I) - 1);
            TAILC[I] = HEADC[I];
            TAILC[(I + 26)] = HEADC[(I + 26)];

            I += m3__;
        }
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = 10;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            TAILC[(52 + I)] = ((intrinsics::ICHAR(b"0") + I) - 1);
            I += m3__;
        }
    }

    TAILC[63] = intrinsics::ICHAR(b"$");
    TAILC[64] = intrinsics::ICHAR(b"_");

    NHEAD = 52;
    NTAIL = 64;

    //
    // Turn the arrays into integer sets.
    //
    VALIDI(MAXCHR, NHEAD, HEADC.as_slice_mut(), ctx)?;
    VALIDI(MAXCHR, NTAIL, TAILC.as_slice_mut(), ctx)?;

    //
    // Create the output specification IDSPEC.  This is a cell
    // containing, in order,
    //
    //    - the number of head characters
    //    - the number of tail characters
    //    - integer codes for the head characters
    //    - integer codes for the tail characters
    //
    // IDSPEC is assumed to be initialized.
    //
    //
    SCARDI(0, IDSPEC.as_slice_mut(), ctx)?;

    APPNDI(NHEAD, IDSPEC.as_slice_mut(), ctx)?;
    APPNDI(NTAIL, IDSPEC.as_slice_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NHEAD;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            APPNDI(HEADC[I], IDSPEC.as_slice_mut(), ctx)?;
            I += m3__;
        }
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = NTAIL;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            APPNDI(TAILC[I], IDSPEC.as_slice_mut(), ctx)?;
            I += m3__;
        }
    }

    CHKOUT(b"LXDFID", ctx)?;
    Ok(())
}

/// Lex, custom identifier characters
///
/// Set the acceptable characters that may appear in an identifier
/// token.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HDCHRS     I   Allowed head characters for identifiers.
///  TLCHRS     I   Allowed tail characters for identifiers.
///  IDSPEC    I-O  Identifier character specification.
///  MXSPEC     P   Recommended size for declaration of IDSPEC.
///  LBCELL     P   The SPICE cell lower bound.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HDCHRS   is a string containing the set of characters
///           allowed as the first (`head') character of an
///           identifier token. Case is significant; if both
///           upper and lower case instances of a letter are
///           allowed, they must both be listed. White space is
///           ignored. Non-printing characters are not allowed.
///
///  TLCHRS   is a string containing the set of characters
///           allowed as tail characters (characters following
///           the head character) of an identifier token. Case
///           is significant; white space is ignored.
///           Non-printing characters are not allowed.
///
///  IDSPEC   is an integer cell. The caller must initialize
///           IDSPEC as a cell, and should use MXSPEC as the size
///           of IDSPEC.
/// ```
///
/// # Detailed Output
///
/// ```text
///  IDSPEC   is an integer cell containing a specification of
///           the head and tail identifier character sets to be
///           used the entry point LXIDNT in scanning strings.
///           The caller must initialize IDSPEC as a cell, and
///           should use MXSPEC as the size of IDSPEC.
/// ```
///
/// # Parameters
///
/// ```text
///  MXSPEC   is the recommended size for the declaration of
///           IDSPEC; the caller should declare IDSPEC as shown:
///
///              INTEGER       IDSPEC ( LBCELL : MXSPEC )
///
///           The caller should also initialize IDSPEC as shown:
///
///              CALL SSIZEI ( MXSPEC, IDSPEC )
///
///  LBCELL   is the SPICE cell lower bound.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If non-printing characters are found in either of the input
///      arguments HDCHRS or TLCHRS, the error SPICE(NONPRINTINGCHARS)
///      is signaled. The set of allowed identifier characters is not
///      modified.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine allows a calling program to customize the set of
///  allowed patterns for identifiers recognized by LXIDNT.
///
///  Normally, this routine should be called once during the calling
///  program's initialization, if this routine is called at all.
/// ```
///
/// # Examples
///
/// ```text
///  See the $Examples section of the umbrella routine LXNAME.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 27-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.1, 10-FEB-2014 (BVS)
///
///         Added LBCELL to the $Declarations, $Brief_I/O, and $Parameters
///         sections.
///
/// -    SPICELIB Version 1.0.0, 25-OCT-1995 (NJB)
/// ```
pub fn lxcsid(
    ctx: &mut SpiceContext,
    hdchrs: &str,
    tlchrs: &str,
    idspec: &mut [i32],
) -> crate::Result<()> {
    LXCSID(
        hdchrs.as_bytes(),
        tlchrs.as_bytes(),
        idspec,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure LXCSID ( Lex, custom identifier characters )
pub fn LXCSID(
    HDCHRS: &[u8],
    TLCHRS: &[u8],
    IDSPEC: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut IDSPEC = DummyArrayMut::new(IDSPEC, LBCELL..);
    let mut C: i32 = 0;
    let mut HEADC = ActualArray::<i32>::new(LBCELL..=MAXCHR);
    let mut HL: i32 = 0;
    let mut I: i32 = 0;
    let mut NHEAD: i32 = 0;
    let mut NTAIL: i32 = 0;
    let mut TAILC = ActualArray::<i32>::new(LBCELL..=MAXCHR);
    let mut TL: i32 = 0;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"LXCSID", ctx)?;
    }

    //
    // Initialize our head and tail character sets, every time.
    //
    SSIZEI(MAXCHR, HEADC.as_slice_mut(), ctx)?;
    SSIZEI(MAXCHR, TAILC.as_slice_mut(), ctx)?;

    //
    // Check the inputs before proceeding.
    //
    HL = RTRIM(HDCHRS);
    TL = RTRIM(TLCHRS);

    {
        let m1__: i32 = 1;
        let m2__: i32 = HL;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            C = intrinsics::ICHAR(fstr::substr(HDCHRS, I..=I));

            if ((C < MINPRT) || (C > MAXPRT)) {
                SETMSG(b"The character having integer code # in position # of the head character string HDCHRS is a non-printing character.", ctx);
                ERRINT(b"#", C, ctx);
                ERRINT(b"#", I, ctx);
                SIGERR(b"SPICE(NONPRINTINGCHARS)", ctx)?;
                CHKOUT(b"LXCSID", ctx)?;
                return Ok(());
            }

            I += m3__;
        }
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = TL;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            C = intrinsics::ICHAR(fstr::substr(TLCHRS, I..=I));

            if ((C < MINPRT) || (C > MAXPRT)) {
                SETMSG(b"The character having integer code # in position # of the tail character string TLCHRS is a non-printing character.", ctx);
                ERRINT(b"#", C, ctx);
                ERRINT(b"#", I, ctx);
                SIGERR(b"SPICE(NONPRINTINGCHARS)", ctx)?;
                CHKOUT(b"LXCSID", ctx)?;
                return Ok(());
            }

            I += m3__;
        }
    }

    //
    // The characters of HDCHRS become the set of acceptable
    // characters for the head identifier character---all except
    // the blanks.  Same deal goes for the tail characters.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = HL;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            C = intrinsics::ICHAR(fstr::substr(HDCHRS, I..=I));

            if (C != BLANK) {
                INSRTI(C, HEADC.as_slice_mut(), ctx)?;
            }

            I += m3__;
        }
    }

    NHEAD = CARDI(HEADC.as_slice(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = TL;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            C = intrinsics::ICHAR(fstr::substr(TLCHRS, I..=I));

            if (C != BLANK) {
                INSRTI(C, TAILC.as_slice_mut(), ctx)?;
            }

            I += m3__;
        }
    }

    NTAIL = CARDI(TAILC.as_slice(), ctx)?;

    //
    // Create the output specification IDSPEC.  This is a cell
    // containing, in order,
    //
    //    - the number of head characters
    //    - the number of tail characters
    //    - integer codes for the head characters
    //    - integer codes for the tail characters
    //
    // IDSPEC is assumed to be initialized.
    //
    //
    SCARDI(0, IDSPEC.as_slice_mut(), ctx)?;

    APPNDI(NHEAD, IDSPEC.as_slice_mut(), ctx)?;
    APPNDI(NTAIL, IDSPEC.as_slice_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NHEAD;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            APPNDI(HEADC[I], IDSPEC.as_slice_mut(), ctx)?;
            I += m3__;
        }
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = NTAIL;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            APPNDI(TAILC[I], IDSPEC.as_slice_mut(), ctx)?;
            I += m3__;
        }
    }

    CHKOUT(b"LXCSID", ctx)?;
    Ok(())
}
