//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const MAXLLT: i32 = 5;
const NCASE: i32 = 3;

struct SaveVars {
    VALSTR: ActualCharArray2D,
    CASSTR: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut VALSTR = ActualCharArray2D::new(MAXLLT, 1..=NCASE, 1..=2);
        let mut CASSTR = ActualCharArray::new(1, 1..=NCASE);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"U"), Val::C(b"L"), Val::C(b"C")].into_iter();
            CASSTR
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"TRUE"),
                Val::C(b"true"),
                Val::C(b"True"),
                Val::C(b"FALSE"),
                Val::C(b"false"),
                Val::C(b"False"),
            ]
            .into_iter();
            VALSTR
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { VALSTR, CASSTR }
    }
}

/// Replace marker with logical value text
///
/// Replace a marker with the text representation of a logical value.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  IN         I   Input string.
///  MARKER     I   Marker to be replaced.
///  VALUE      I   Replacement logical value.
///  RTCASE     I   Case of replacement text.
///  OUT        O   Output string.
/// ```
///
/// # Detailed Input
///
/// ```text
///  IN       is an arbitrary character string.
///
///  MARKER   is an arbitrary character string. The first occurrence
///           of MARKER in the input string is to be replaced by
///           VALUE.
///
///           MARKER is case-sensitive.
///
///           Leading and trailing blanks in MARKER are NOT
///           significant. In particular, no substitution is
///           performed if MARKER is blank.
///
///  VALUE    is an arbitrary logical value, either .TRUE. or
///           .FALSE.
///
///  RTCASE   indicates the case of the replacement text. RTCASE may
///           be any of the following:
///
///              RTCASE    Meaning        Output values
///              ------    -----------    ---------------
///              U, u      Uppercase      'TRUE', 'FALSE'
///
///              L, l      Lowercase      'true', 'false'
///
///              C, c      Capitalized    'True', 'False'
/// ```
///
/// # Detailed Output
///
/// ```text
///  OUT      is the string obtained by substituting the text
///           representation of VALUE for the first occurrence of
///           MARKER in the input string.
///
///           OUT and IN must be disjoint.
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
///  into strings. They are typically used to construct messages that
///  are partly fixed, and partly determined at run time. For example,
///  a message like
///
///     'Fifty-one pictures were found in directory [USER.DATA].'
///
///  might be constructed from the fixed string
///
///     '#1 pictures were found in directory #2.'
///
///  by the calls
///
///     CALL REPMCT ( STRING, '#1',  51,           'C', TMPSTR )
///     CALL REPMC  ( TMPSTR, '#2', '[USER.DATA]',      STRING )
///
///  which substitute the cardinal text 'Fifty-one' and the character
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
///  1) The following example illustrates the use of REPML to replace
///     a marker within a string with the text representation of a
///     logical value.
///
///
///     Example code begins here.
///
///
///           PROGRAM REPML_EX1
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
///           INSTR  = 'Invalid value. The value was:  #.'
///
///           CALL REPML ( INSTR, MARKER, .FALSE. , 'U' , OUTSTR )
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
///           INSTR  = 'Invalid value. The value was:  XX.'
///
///           CALL REPML ( INSTR, MARKER, .TRUE. , 'l' , OUTSTR )
///
///           WRITE(*,*) 'Case 2: Replacement text in lowercase.'
///           WRITE(*,*) '   Input : ', INSTR
///           WRITE(*,*) '   Output: ', OUTSTR
///           WRITE(*,*) ' '
///
///     C
///     C     2. Capitalized
///     C
///           MARKER = '#'
///           INSTR  = 'Invalid value. The value was:  #.'
///
///           CALL REPML ( INSTR, MARKER, .FALSE. , 'c' , OUTSTR )
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
///         Input : Invalid value. The value was:  #.
///         Output: Invalid value. The value was:  FALSE.
///
///      Case 2: Replacement text in lowercase.
///         Input : Invalid value. The value was:  XX.
///         Output: Invalid value. The value was:  true.
///
///      Case 3: Replacement text capitalized.
///         Input : Invalid value. The value was:  #.
///         Output: Invalid value. The value was:  False.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.0, 08-JAN-2021 (JDR) (NJB)
/// ```
pub fn repml(
    ctx: &mut SpiceContext,
    in_: &str,
    marker: &str,
    value: bool,
    rtcase: char,
    out: &mut str,
) -> crate::Result<()> {
    REPML(
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

//$Procedure REPML ( Replace marker with logical value text )
pub fn REPML(
    IN: &[u8],
    MARKER: &[u8],
    VALUE: bool,
    RTCASE: &[u8],
    OUT: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let RTCASE = &RTCASE[..1];
    let mut LVALUE = [b' '; MAXLLT as usize];
    let mut TMPCAS = [b' '; 1 as usize];
    let mut CASIDX: i32 = 0;
    let mut MRKNBF: i32 = 0;
    let mut MRKNBL: i32 = 0;
    let mut MRKPSB: i32 = 0;
    let mut MRKPSE: i32 = 0;
    let mut VALIDX: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"REPML", ctx)?;

    //
    // Identify the case string and find its index in the
    // array of uppercase case strings. Bail out if RTCASE is not
    // recognized.
    //
    // RTCASE has length 1, so we need not be concerned with leading
    // blanks.
    //
    UCASE(RTCASE, &mut TMPCAS, ctx);

    CASIDX = ISRCHC(&TMPCAS, NCASE, save.CASSTR.as_arg());

    if (CASIDX == 0) {
        SETMSG(b"Case (#) must be U, L, or C.", ctx);
        ERRCH(b"#", RTCASE, ctx);
        SIGERR(b"SPICE(INVALIDCASE)", ctx)?;
        CHKOUT(b"REPML", ctx)?;
        return Ok(());
    }

    //
    // If MARKER is blank, no substitution is possible.
    //
    if fstr::eq(MARKER, b" ") {
        fstr::assign(OUT, IN);

        CHKOUT(b"REPML", ctx)?;
        return Ok(());
    }

    //
    // Locate the leftmost occurrence of MARKER, if there is one
    // (ignoring leading and trailing blanks). If MARKER is not
    // a substring of IN, no substitution can be performed.
    //
    MRKNBF = FRSTNB(MARKER);
    MRKNBL = LASTNB(MARKER);

    //
    // MARKER is non-blank, so the index range below is valid.
    //
    MRKPSB = intrinsics::INDEX(IN, fstr::substr(MARKER, MRKNBF..=MRKNBL));

    if (MRKPSB == 0) {
        fstr::assign(OUT, IN);

        CHKOUT(b"REPML", ctx)?;
        return Ok(());
    }

    MRKPSE = ((MRKPSB + MRKNBL) - MRKNBF);

    //
    // Okay, MARKER is non-blank and has been found.
    //
    if VALUE {
        VALIDX = 1;
    } else {
        VALIDX = 2;
    }

    //
    // Set the value string based on the case specification and
    // the input logical value.
    //
    fstr::assign(&mut LVALUE, save.VALSTR.get([CASIDX, VALIDX]));

    //
    // Replace MARKER with LVALUE.
    //
    REPSUB(
        IN,
        MRKPSB,
        MRKPSE,
        fstr::substr(&LVALUE, 1..=LASTNB(&LVALUE)),
        OUT,
        ctx,
    )?;

    CHKOUT(b"REPML", ctx)?;
    Ok(())
}
