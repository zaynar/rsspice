//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const MAXLON: i32 = 147;

/// Replace marker with ordinal text
///
/// Replace a marker with the text representation of an ordinal
/// number.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  IN         I   Input string.
///  MARKER     I   Marker to be replaced.
///  VALUE      I   Replacement value.
///  RTCASE     I   Case of replacement text.
///  OUT        O   Output string.
///  MAXLON     P   Maximum length of an ordinal number.
/// ```
///
/// # Detailed Input
///
/// ```text
///  IN       is an arbitrary character string.
///
///  MARKER   is an arbitrary character string. The first occurrence of
///           MARKER in the input string is to be replaced by the text
///           representation of the ordinal number VALUE.
///
///           Leading and trailing blanks in MARKER are NOT
///           significant. In particular, no substitution is performed
///           if MARKER is blank.
///
///  VALUE    is an arbitrary integer.
///
///  RTCASE   indicates the case of the replacement text. RTCASE may be
///           any of the following:
///
///              RTCASE   Meaning        Example
///              ------   -----------    -----------------------
///              U, u     Uppercase      ONE HUNDRED FIFTY-THIRD
///
///              L, l     Lowercase      one hundred fifty-third
///
///              C, c     Capitalized    One hundred fifty-third
/// ```
///
/// # Detailed Output
///
/// ```text
///  OUT      is the string obtained by substituting the text
///           representation of the ordinal number VALUE for the first
///           occurrence of MARKER in the input string.
///
///           OUT and IN must be identical or disjoint.
/// ```
///
/// # Parameters
///
/// ```text
///  MAXLON   is the maximum expected length of any ordinal text.
///           147 characters are sufficient to hold the text
///           representing any ordinal value whose corresponding
///           ordinal value is in the range
///
///             ( -10**12, 10**12 )
///
///           An example of a number whose ordinal text representation
///           is of maximum length is
///
///              - 777 777 777 777
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If OUT does not have sufficient length to accommodate the
///      result of the substitution, the result will be truncated on
///      the right.
///
///  2)  If MARKER is blank, or if MARKER is not a substring of IN,
///      no substitution is performed. (OUT and IN are identical.)
///
///  3)  If the value of RTCASE is not recognized, the error
///      SPICE(INVALIDCASE) is signaled. OUT is not changed.
/// ```
///
/// # Particulars
///
/// ```text
///  This is one of a family of related routines for inserting values
///  into strings. They are typically to construct messages that
///  are partly fixed, and partly determined at run time. For example,
///  a message like
///
///     'The fifty-first picture was found in directory [USER.DATA].'
///
///  might be constructed from the fixed string
///
///     'The #1 picture was found in directory #2.'
///
///  by the calls
///
///     CALL REPMOT ( STRING, '#1',  51,           'L', STRING )
///     CALL REPMC  ( STRING, '#2', '[USER.DATA]',      STRING )
///
///  which substitute the ordinal text 'fifty-first' and the character
///  string '[USER.DATA]' for the markers '#1' and '#2' respectively.
///
///  The complete list of routines is shown below.
///
///     REPMC    ( Replace marker with character string value )
///     REPMD    ( Replace marker with double precision value )
///     REPMF    ( Replace marker with formatted d.p. value   )
///     REPMI    ( Replace marker with integer value          )
///     REPML    ( Replace marker with logical value          )
///     REPMCT   ( Replace marker with cardinal text          )
///     REPMOT   ( Replace marker with ordinal text           )
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) The following example illustrate the use of REPMOT to
///     replace a marker within a string with the ordinal text
///     corresponding to an integer.
///
///
///     Example code begins here.
///
///
///           PROGRAM REPMOT_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           INTEGER                 STRLEN
///           PARAMETER             ( STRLEN = 80 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(STRLEN)      INSTR
///           CHARACTER*(STRLEN)      MARKER
///           CHARACTER*(STRLEN)      OUTSTR
///
///
///     C
///     C     1. Uppercase
///     C
///           MARKER = '#'
///           INSTR  = 'INVALID COMMAND. # WORD NOT RECOGNIZED.'
///
///           CALL REPMOT ( INSTR, MARKER, 5, 'U', OUTSTR )
///
///           WRITE(*,*) 'Case 1: Replacement text in uppercase.'
///           WRITE(*,*) '   Input : ', INSTR
///           WRITE(*,*) '   Output: ', OUTSTR
///           WRITE(*,*) ' '
///
///     C
///     C     2. Lowercase
///     C
///           MARKER = ' XX '
///           INSTR  = 'The XX word of the XX sentence was ...'
///
///           CALL REPMOT ( INSTR, MARKER, 5, 'L', OUTSTR )
///
///           WRITE(*,*) 'Case 2: Replacement text in lowercase.'
///           WRITE(*,*) '   Input : ', INSTR
///           WRITE(*,*) '   Output: ', OUTSTR
///           WRITE(*,*) ' '
///
///     C
///     C     2. Capitalized
///     C
///           MARKER = ' XX '
///           INSTR  = 'Name:  YY.  Rank:  XX.'
///
///           CALL REPMC  ( INSTR,  'YY',  'Moriarty', OUTSTR )
///           CALL REPMOT ( OUTSTR, MARKER, 5,    'C', OUTSTR )
///
///           WRITE(*,*) 'Case 3: Replacement text capitalized.'
///           WRITE(*,*) '   Input : ', INSTR
///           WRITE(*,*) '   Output: ', OUTSTR
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Case 1: Replacement text in uppercase.
///         Input : INVALID COMMAND. # WORD NOT RECOGNIZED.
///         Output: INVALID COMMAND. FIFTH WORD NOT RECOGNIZED.
///
///      Case 2: Replacement text in lowercase.
///         Input : The XX word of the XX sentence was ...
///         Output: The fifth word of the XX sentence was ...
///
///      Case 3: Replacement text capitalized.
///         Input : Name:  YY.  Rank:  XX.
///         Output: Name:  Moriarty.  Rank:  Fifth.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  VALUE must be in the range accepted by the SPICELIB routine
///      INTORD. This range is currently
///
///         ( -10**12, 10**12 )
///
///      Note that the endpoints of the interval are excluded.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.3.0, 06-JUL-2021 (JDR)
///
///         Changed input argument name CASE to RTCASE for consistency
///         with other routines.
///
///         Added IMPLICIT NONE statement.
///
///         Updated code to remove unnecessary lines of code in the
///         Standard SPICE error handling CHKIN statements.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example from existing fragments. Improved example in
///         $Particulars section.
///
///         Added REPML to the list of available replace marker routines.
///
/// -    SPICELIB Version 1.2.0, 21-SEP-2013 (BVS)
///
///         Minor efficiency update: the routine now looks up the first
///         and last non-blank characters only once.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 30-AUG-1990 (NJB) (IMU)
/// ```
pub fn repmot(
    ctx: &mut SpiceContext,
    in_: &str,
    marker: &str,
    value: i32,
    rtcase: char,
    out: &mut str,
) -> crate::Result<()> {
    REPMOT(
        in_.as_bytes(),
        marker.as_bytes(),
        value,
        &[u8::try_from(rtcase).unwrap()],
        fstr::StrBytes::new(out).as_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure REPMOT  ( Replace marker with ordinal text )
pub fn REPMOT(
    IN: &[u8],
    MARKER: &[u8],
    VALUE: i32,
    RTCASE: &[u8],
    OUT: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let RTCASE = &RTCASE[..1];
    let mut ORD = [b' '; MAXLON as usize];
    let mut TMPCAS = [b' '; 1 as usize];
    let mut MRKNBF: i32 = 0;
    let mut MRKNBL: i32 = 0;
    let mut MRKPSB: i32 = 0;
    let mut MRKPSE: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"REPMOT", ctx)?;

    //
    // Bail out if RTCASE is not recognized.
    //
    LJUST(RTCASE, &mut TMPCAS);
    UCASE(&TMPCAS.clone(), &mut TMPCAS, ctx);

    if ((fstr::ne(&TMPCAS, b"U") && fstr::ne(&TMPCAS, b"L")) && fstr::ne(&TMPCAS, b"C")) {
        SETMSG(b"Case (#) must be U, L, or C.", ctx);
        ERRCH(b"#", RTCASE, ctx);
        SIGERR(b"SPICE(INVALIDCASE)", ctx)?;
        CHKOUT(b"REPMOT", ctx)?;
        return Ok(());
    }

    //
    // If MARKER is blank, no substitution is possible.
    //
    if fstr::eq(MARKER, b" ") {
        fstr::assign(OUT, IN);

        CHKOUT(b"REPMOT", ctx)?;
        return Ok(());
    }

    //
    // Locate the leftmost occurrence of MARKER, if there is one
    // (ignoring leading and trailing blanks). If MARKER is not
    // a substring of IN, no substitution can be performed.
    //
    MRKNBF = FRSTNB(MARKER);
    MRKNBL = LASTNB(MARKER);

    MRKPSB = intrinsics::INDEX(IN, fstr::substr(MARKER, MRKNBF..=MRKNBL));

    if (MRKPSB == 0) {
        fstr::assign(OUT, IN);

        CHKOUT(b"REPMOT", ctx)?;

        return Ok(());
    }

    MRKPSE = ((MRKPSB + MRKNBL) - MRKNBF);

    //
    // Okay, RTCASE is recognized and MARKER has been found.
    // Generate the ordinal text corresponding to VALUE.
    //
    INTORD(VALUE, &mut ORD, ctx);

    //
    // CARD is always returned in upper case; change to the specified
    // case, if required.
    //
    if fstr::eq(&TMPCAS, b"L") {
        LCASE(&ORD.clone(), &mut ORD, ctx);
    } else if fstr::eq(&TMPCAS, b"C") {
        LCASE(
            &fstr::substr(&ORD, 2..).to_vec(),
            fstr::substr_mut(&mut ORD, 2..),
            ctx,
        );
    }

    //
    // Replace MARKER with CARD.
    //
    REPSUB(
        IN,
        MRKPSB,
        MRKPSE,
        fstr::substr(&ORD, 1..=LASTNB(&ORD)),
        OUT,
        ctx,
    )?;

    CHKOUT(b"REPMOT", ctx)?;
    Ok(())
}
