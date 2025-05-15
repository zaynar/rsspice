//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const MAXLI: i32 = 11;

/// Replace marker with integer
///
/// Replace a marker with an integer.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  IN         I   Input string.
///  MARKER     I   Marker to be replaced.
///  VALUE      I   Replacement value.
///  OUT        O   Output string.
///  MAXLI      P   Maximum length of an integer.
/// ```
///
/// # Detailed Input
///
/// ```text
///  IN       is an arbitrary character string.
///
///  MARKER   is an arbitrary character string. The first occurrence of
///           MARKER in the input string is to be replaced by VALUE.
///
///           Leading and trailing blanks in MARKER are NOT
///           significant. In particular, no substitution is performed
///           if MARKER is blank.
///
///  VALUE    is an arbitrary integer.
/// ```
///
/// # Detailed Output
///
/// ```text
///  OUT      is the string obtained by substituting the text
///           representation of VALUE for the first occurrence of
///           MARKER in the input string.
///
///           OUT and IN must be identical or disjoint.
/// ```
///
/// # Parameters
///
/// ```text
///  MAXLI    is the maximum expected length of the text representation
///           of an integer. 11 characters are sufficient to hold any
///           integer whose absolute value is less than 10 billion.
///
///           This routine assumes that the input integer is such that
///           its string representation contains no more than MAXLI
///           characters.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If OUT does not have sufficient length to accommodate the
///      result of the substitution, the result will be truncated on
///      the right.
///
///  2)  If MARKER is blank, or if MARKER is not a substring of IN,
///      no substitution is performed. (OUT and IN are identical.)
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
///     'Fifty-one pictures were found in directory [USER.DATA].'
///
///  might be constructed from the fixed string
///
///     '#1 pictures were found in directory #2.'
///
///  by the calls
///
///     CALL REPMCT ( STRING, '#1',  51,           'C', STRING )
///     CALL REPMC  ( STRING, '#2', '[USER.DATA]',      STRING )
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
///  1) The following example illustrate the use of REPMI to
///     replace a marker within a string with the text representation
///     of an integer value.
///
///
///     Example code begins here.
///
///
///           PROGRAM REPMI_EX1
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
///     C
///     C     1. Single marker
///     C
///           MARKER = '#'
///           INSTR  = 'Invalid value. The value was:  #'
///
///           CALL REPMI ( INSTR, MARKER, 75, OUTSTR )
///
///           WRITE(*,*) 'Case 1: Single marker.'
///           WRITE(*,*) '   Input : ', INSTR
///           WRITE(*,*) '   Output: ', OUTSTR
///           WRITE(*,*)
///
///     C
///     C     2. Multiple markers
///     C
///           MARKER = ' XX '
///           INSTR  = 'Left > Right endpoint. Left: XX; Right: XX'
///
///           CALL REPMI ( INSTR, MARKER, 2035, OUTSTR )
///
///           WRITE(*,*) 'Case 2: Multiple markers.'
///           WRITE(*,*) '   Input : ', INSTR
///           WRITE(*,*) '   Output: ', OUTSTR
///           WRITE(*,*)
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Case 1: Single marker.
///         Input : Invalid value. The value was:  #
///         Output: Invalid value. The value was:  75
///
///      Case 2: Multiple markers.
///         Input : Left > Right endpoint. Left: XX; Right: XX
///         Output: Left > Right endpoint. Left: 2035; Right: XX
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
/// -    SPICELIB Version 1.3.0, 21-AUG-2020 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example based on existing fragments.
///
///         Added REPML to the list of available replace marker routines in
///         $Particulars and extended MAXLI parameter description in
///         $Parameters.
///
/// -    SPICELIB Version 1.2.0, 21-SEP-2013 (BVS)
///
///         Minor efficiency update: the routine now looks up the first
///         and last non-blank characters only once.
///
/// -    SPICELIB Version 1.1.0, 15-AUG-2002 (WLT)
///
///         The routine is now error free.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 30-AUG-1990 (NJB) (IMU)
/// ```
pub fn repmi(ctx: &mut SpiceContext, in_: &str, marker: &str, value: i32, out: &mut str) {
    REPMI(
        in_.as_bytes(),
        marker.as_bytes(),
        value,
        fstr::StrBytes::new(out).as_mut(),
        ctx.raw_context(),
    );
}

//$Procedure REPMI  ( Replace marker with integer )
pub fn REPMI(IN: &[u8], MARKER: &[u8], VALUE: i32, OUT: &mut [u8], ctx: &mut Context) {
    let mut SUBSTR = [b' '; MAXLI as usize];
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
    // If MARKER is blank, no substitution is possible.
    //
    if fstr::eq(MARKER, b" ") {
        fstr::assign(OUT, IN);
        return;
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
        return;
    }

    MRKPSE = ((MRKPSB + MRKNBL) - MRKNBF);

    //
    // Okay, MARKER is non-blank and has been found. Convert the
    // integer to text, and substitute the text for the marker.
    //
    INTSTR(VALUE, &mut SUBSTR, ctx);

    ZZREPSUB(
        IN,
        MRKPSB,
        MRKPSE,
        fstr::substr(&SUBSTR, 1..=LASTNB(&SUBSTR)),
        OUT,
    );
}
