//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const MAXLDP: i32 = 23;

/// Replace marker with double precision number
///
/// Replace a marker with a double precision number.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  IN         I   Input string.
///  MARKER     I   Marker to be replaced.
///  VALUE      I   Replacement value.
///  SIGDIG     I   Significant digits in replacement text.
///  OUT        O   Output string.
///  MAXLDP     P   Maximum length of a DP number.
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
///  VALUE    is an arbitrary double precision number.
///
///  SIGDIG   is the number of significant digits with which VALUE is
///           to be represented. SIGDIG must be greater than zero and
///           less than 15.
/// ```
///
/// # Detailed Output
///
/// ```text
///  OUT      is the string obtained by substituting the text
///           representation of VALUE for the first occurrence of
///           MARKER in the input string.
///
///           The text representation of VALUE is in scientific
///           notation, having the number of significant digits
///           specified by SIGDIG. The representation of VALUE is
///           produced by the SPICELIB routine DPSTR; see that routine
///           for details concerning the representation of double
///           precision numbers.
///
///           OUT and IN must be identical or disjoint.
/// ```
///
/// # Parameters
///
/// ```text
///  MAXLDP   is the maximum expected length of the text representation
///           of a double precision number. 23 characters are
///           sufficient to hold any result returned by the SPICELIB
///           routine DPSTR. (See $Restrictions)
///
///           This routine assumes that the input d.p. value is such
///           that its string representation contains no more than
///           MAXLDP characters.
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
///  1) The following example illustrate the use of REPMD to
///     replace a marker within a string with the text representation
///     of a double precision value.
///
///
///     Example code begins here.
///
///
///           PROGRAM REPMD_EX1
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
///     C     1. Single marker, two significant digits.
///     C
///           MARKER = '#'
///           INSTR  = 'Invalid value. The value was:  #'
///
///           CALL REPMD ( INSTR, MARKER, 5.0D1, 2, OUTSTR )
///
///           WRITE(*,*) 'Case 1: Single marker, two significant '
///          .        // 'digits.'
///           WRITE(*,*) '   Input : ', INSTR
///           WRITE(*,*) '   Output: ', OUTSTR
///           WRITE(*,*)
///
///     C
///     C     2. Multiple markers, three significant digits.
///     C
///           MARKER = ' XX '
///           INSTR  = 'Left > Right endpoint. Left: XX; Right: XX'
///
///           CALL REPMD ( INSTR, MARKER, -5.2D-9, 3, OUTSTR )
///
///           WRITE(*,*) 'Case 2: Multiple markers, three '
///          .        // 'significant digits.'
///           WRITE(*,*) '   Input : ', INSTR
///           WRITE(*,*) '   Output: ', OUTSTR
///           WRITE(*,*)
///
///     C
///     C     3. Excessive significant digits.
///     C
///           MARKER = '#'
///           INSTR  = 'Invalid value. The value was:  #'
///
///           CALL REPMD ( INSTR, MARKER, 5.0D1, 100, OUTSTR )
///
///           WRITE(*,*) 'Case 3: Excessive significant digits.'
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
///      Case 1: Single marker, two significant digits.
///         Input : Invalid value. The value was:  #
///         Output: Invalid value. The value was:  5.0E+01
///
///      Case 2: Multiple markers, three significant digits.
///         Input : Left > Right endpoint. Left: XX; Right: XX
///         Output: Left > Right endpoint. Left: -5.20E-09; Right: XX
///
///      Case 3: Excessive significant digits.
///         Input : Invalid value. The value was:  #
///         Output: Invalid value. The value was:  5.0000000000000E+01
///
///
///     Note that, in Case #3 even though 100 digits of precision were
///     requested, only 14 were returned.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The maximum number of significant digits returned is 14.
///
///  2)  This routine makes explicit use of the format of the string
///      returned by the SPICELIB routine DPSTR; should that routine
///      change, substantial work may be required to bring this routine
///      back up to snuff.
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
/// -    SPICELIB Version 1.3.0, 03-OCT-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example based on existing fragments.
///
///         Added REPML to the list of available replace marker routines in
///         $Particulars and extended the description of MAXLDP in
///         $Parameters.
///
/// -    SPICELIB Version 1.2.0, 23-SEP-2013 (BVS)
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
pub fn repmd(
    ctx: &mut SpiceContext,
    in_: &str,
    marker: &str,
    value: f64,
    sigdig: i32,
    out: &mut str,
) {
    REPMD(
        in_.as_bytes(),
        marker.as_bytes(),
        value,
        sigdig,
        fstr::StrBytes::new(out).as_mut(),
        ctx.raw_context(),
    );
}

//$Procedure REPMD  ( Replace marker with double precision number )
pub fn REPMD(IN: &[u8], MARKER: &[u8], VALUE: f64, SIGDIG: i32, OUT: &mut [u8], ctx: &mut Context) {
    let mut SUBSTR = [b' '; MAXLDP as usize];
    let mut MRKNBF: i32 = 0;
    let mut MRKNBL: i32 = 0;
    let mut MRKPSB: i32 = 0;
    let mut MRKPSE: i32 = 0;
    let mut SUBNBF: i32 = 0;
    let mut SUBNBL: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //
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
    // number to text, and substitute the text for the marker.
    //
    DPSTR(VALUE, SIGDIG, &mut SUBSTR, ctx);

    SUBNBF = FRSTNB(&SUBSTR);
    SUBNBL = LASTNB(&SUBSTR);

    if ((SUBNBF != 0) && (SUBNBL != 0)) {
        ZZREPSUB(
            IN,
            MRKPSB,
            MRKPSE,
            fstr::substr(&SUBSTR, SUBNBF..=SUBNBL),
            OUT,
        );
    }
}
