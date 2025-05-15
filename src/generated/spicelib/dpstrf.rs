//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Double Precision Number to Character
///
/// Take a double precision number and convert it to an
/// equivalent formatted character string representation (base 10).
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  X          I   A double precision number
///  SIGDIG     I   The number of significant digits saved for output
///  FORMAT     I   'E' for scientific, 'F' for floating point.
///  STRING     O   A character string representation of X
/// ```
///
/// # Detailed Input
///
/// ```text
///  X        is a double precision number.
///
///  SIGDIG   is the number of significant digits that are desired
///           for the output string.
///
///  FORMAT   is a character flag that indicates how the double
///           precision number should be represented. The two
///           acceptable inputs are 'E' and 'F'.  If the input
///           is 'E' then the number will be displayed with an
///           exponent in scientific notation. It will have the
///           form 'sx.xxx - - - xxxxxEsyy' where there are
///           SIGDIG x's and s is ' ' or '-' at its first occurrence
///           and '-' or '+' in the second.
///
///           If the input is 'F' then the number will be
///           displayed without an exponent --- the representation
///           will be strictly decimal. The first symbol will be
///           a sign ('-' or ' ').
/// ```
///
/// # Detailed Output
///
/// ```text
///  STRING   is a character representation of X to the number of
///           significant digits specified by SIGDIG. The number of
///           spaces required to return the requested character
///           string is SIGDIG + 6. If STRING is not declared to
///           have adequate length, the number returned will be
///           truncated on the right.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If SIGDIG is less than one, this routine returns one
///      significant digit in the output string.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine computes an approximate character representation
///  of the input string X. The maximum number of significant
///  digits returned is 14 (in F format there may be many extra
///  zeros returned but only a maximum of 14 digits will be
///  significant.
///
///  The output string is left justified.
///
///  This routine has the advantage that it does not use an internal
///  file and is about twice as fast as an internal write. It can
///  be used as part of character function without fear of introducing
///  recursive I/O conflicts. It is intended to be an approximate
///  inverse to the subroutine NPARSD.
///
///  IF you want the character string representation of a double
///  precision number to be the same as that produced by a formatted
///  write statement use a FORTRAN write statement.
///
///  For example the number represented by the string
///
///        1.245454545454545454545E+01
///
///  when read (via a FORTRAN READ statement) into the DP variable X
///  and converted back to a character string having 14 significant
///  digits by this routine yields
///
///        1.2454545454545E+01  in E format
///        12.454545454545      in F format
///
///  The FORTRAN write statement
///
///        WRITE ( 6, FMT='(P1E)' ) X
///
///  yields
///
///        1.2454545454545454E+01
///
///  If this is too much error for your application DO NOT use this
///  routine. You should be aware however, that a character string
///  read into a double precision number may not WRITE out with an
///  equivalent character representation as was input.
///
///  For example on a VAX 11/780 if you
///
///        READ  (5,*)         X
///        WRITE (6,FMT='(E)') X
///
///  and enter a value of 7.00000001 for the read statement
///  the output written will be 0.7000000010000001E+01
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that you wished to insert the character representation
///  of some DOUBLE PRECISION number into a line of text.
///
///  For example suppose X contains the double precision number
///  4.268176872928187 and you would like to insert the character
///  representation of this number to 2 places between the strings
///
///  'There are', 'meters between lamp posts'
///
///  You could perform the following sequence of steps
///
///
///        DOUBLE PRECISION  X
///        CHARACTER*5       DISTANCE
///        CHARACTER*80      MESSAGE
///
///        CALL DPSTRF ( X, 2, 'F', DISTANCE )
///
///        MESSAGE = 'There are '                //
///       .           DISTANCE                   //
///       .          'meters between lamp posts'
///       .
///
///  C
///  C     Squeeze any extra spaces out of the message string.
///  C
///        CALL CMPRSS ( ' ', 1, MESSAGE, MESSAGE )
///
///
///
///  The string MESSAGE would contain:
///
///       'There are 4.2 meters between lamp posts'
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The maximum number of significant digits returned is 14.
///
///  2)  If the output string is not declared to be adequately large
///      the numeric string will be truncated to the side opposite its
///      justification (At least SIGDIG + 6 characters are needed in E
///      format, in F format the size required is dependent upon the
///      input X and the number of significant digits requested. In
///      extreme cases up to 56 characters may be required.)
///
///  3)  This routine makes explicit use of the format of the string
///      returned by DPSTR, should that routine change, substantial
///      work may be required to bring this routine back up to snuff.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.1, 12-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.2.0, 17-SEP-1996 (WLT)
///
///         Upgraded routine to handle arbitrary magnitude d.p. numbers.
///
/// -    SPICELIB Version 1.1.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.1.0, 30-JUL-1990 (WLT)
///
///         The routine was repaired so that references to zero-length
///         strings ( for example STRING(4:3) ) are not made.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.1.0, 30-JUL-1990 (WLT)
///
///         As previously implemented, one would occasionally reference
///         a zero length substring of the variable NUMSTR. This was
///         O.K. under VAX Fortran, because it allows such references.
///         However, most implementations of Fortran are not as forgiving.
/// ```
pub fn dpstrf(ctx: &mut SpiceContext, x: f64, sigdig: i32, format: char, string: &mut str) {
    DPSTRF(
        x,
        sigdig,
        &[u8::try_from(format).unwrap()],
        fstr::StrBytes::new(string).as_mut(),
        ctx.raw_context(),
    );
}

//$Procedure DPSTRF ( Double Precision Number to Character )
pub fn DPSTRF(X: f64, SIGDIG: i32, FORMAT: &[u8], STRING: &mut [u8], ctx: &mut Context) {
    let FORMAT = &FORMAT[..1];
    let mut Y: f64 = 0.0;
    let mut EXP: i32 = 0;
    let mut FIRST: i32 = 0;
    let mut LAST: i32 = 0;
    let mut LASTCH: i32 = 0;
    let mut MAXDIG: i32 = 0;
    let mut J: i32 = 0;
    let mut OVFLOW: bool = false;

    //
    // Local variables
    //

    MAXDIG = intrinsics::MIN0(&[14, intrinsics::MAX0(&[1, SIGDIG])]);

    //
    // If the format is 'E' we just let DPSTR handle the problem.
    //
    if fstr::eq(FORMAT, b"E") {
        DPSTR(X, MAXDIG, STRING, ctx);
        return;
    }

    //
    // If we're still here, we have a decimal format requested.  Set
    // the sign for the number.
    //
    if (X < 0.0) {
        fstr::assign(STRING, b"-");
    } else {
        fstr::assign(STRING, b" ");
    }
    //
    // If X is zero, we can handle this without any regard to the
    // exponent.
    //
    if (X == 0.0) {
        ZZVSTSTR(X, b" ", &mut EXP, ctx);
        ZZVSBSTR(
            -1,
            MAXDIG,
            false,
            fstr::substr_mut(STRING, 2..),
            &mut OVFLOW,
            ctx,
        );
        return;
    }
    //
    // We've already set the sign, now we deal with the unsigned
    // portion of X.
    //
    Y = f64::abs(X);

    //
    // Create a virtual decimal string for Y.
    //
    ZZVSTSTR(Y, b" ", &mut EXP, ctx);

    //
    // Now we can just fill in the string by reading the appropriate
    // substring from the virtual decimal string.  We need to compute
    // the first and last virtual digits to retrieve.  To do this
    // we look at EXP.
    //
    if (EXP >= 0) {
        FIRST = (-EXP - 1);
    } else {
        FIRST = -EXP;
    }

    LAST = ((FIRST + MAXDIG) - 1);

    if ((FIRST < 0) && (LAST >= 0)) {
        LAST = (LAST + 1);
    }

    FIRST = intrinsics::MIN0(&[-1, FIRST]);

    ZZVSBSTR(
        FIRST,
        LAST,
        true,
        fstr::substr_mut(STRING, 2..),
        &mut OVFLOW,
        ctx,
    );

    if OVFLOW {
        FIRST = (FIRST - 1);

        ZZVSBSTR(
            FIRST,
            LAST,
            true,
            fstr::substr_mut(STRING, 2..),
            &mut OVFLOW,
            ctx,
        );
        //
        // We need to blank out the last digit of string.
        //
        LASTCH = ((LAST - FIRST) + 2);

        if ((LAST > 0) && (LASTCH <= intrinsics::LEN(STRING))) {
            fstr::assign(fstr::substr_mut(STRING, LASTCH..), b" ");
        }
    }

    if (LAST < 0) {
        J = ((LAST - FIRST) + 3);

        for I in (LAST + 1)..=-1 {
            if (J <= intrinsics::LEN(STRING)) {
                fstr::assign(fstr::substr_mut(STRING, J..=J), b"0");
            }
            J = (J + 1);
        }

        if (J <= intrinsics::LEN(STRING)) {
            fstr::assign(fstr::substr_mut(STRING, J..=J), b".");
        }
    }
}
