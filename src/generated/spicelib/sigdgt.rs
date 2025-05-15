//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Retain significant digits
///
/// Retain only the significant digits in a numeric string.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  IN         I   Input numeric string.
///  OUT        O   Numeric string, with insignificant digits removed.
/// ```
///
/// # Detailed Input
///
/// ```text
///  IN       is a numeric string.
/// ```
///
/// # Detailed Output
///
/// ```text
///  OUT      is the same numeric string with insignificant
///           zeros and spaces removed. The special case '.000...'
///           becomes just '0'. OUT may overwrite IN. If the
///           output string is too long, it is truncated on the
///           right.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If IN is a non-numeric string, the contents of OUT are
///      unpredictable.
/// ```
///
/// # Particulars
///
/// ```text
///  There are only two interesting cases:
///
///     1) There is a decimal point and an exponent immediately
///        preceded by zero ('...0E', '...0D', '...0e', '...0d')
///        or by a space ('... E', '... D', '... e', '... d').
///
///     2) There is a decimal point and no exponent, and the last non-
///        blank character is a zero ('...0').
///
///  In each of these cases, go to the zero in question, and step
///  backwards until you find something other than a blank or a zero.
///
///  Finally, remove all leading spaces, and all occurrences of more
///  than one consecutive space within the string.
/// ```
///
/// # Examples
///
/// ```text
///  The following examples illustrate the use of SIGDGT.
///
///  '0.123456000000D-04'        becomes     '0.123456D-04'
///  '  -9.2100000000000'                    '-9.21'
///  '       13'                             '13'
///  '    00013'                             '00013'
///  ' .314 159 265 300 000 e1'              '.314 159 265 3e1'
///  '   123    45     6'                    '123 45 6'
///  '  .000000000'                          '0'
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 12-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU) (HAN) (NJB) (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -     Beta Version 1.3.0, 21-MAR-1989 (WLT)
///
///          Previous fix was unbelievably bad, very buggy. This
///          has been fixed along with other bugs and non-standard
///          code has been removed.
///
/// -     Beta Version 1.2.0, 28-FEB-1989 (WLT)
///
///          Reference to INSSUB replaced by SUFFIX
///
/// -     Beta Version 1.1.1, 17-FEB-1989 (HAN) (NJB)
///
///          Declaration of the unused function ISRCHC removed.
/// ```
pub fn sigdgt(in_: &str, out: &mut str) {
    SIGDGT(in_.as_bytes(), fstr::StrBytes::new(out).as_mut());
}

//$Procedure SIGDGT ( Retain significant digits )
pub fn SIGDGT(IN: &[u8], OUT: &mut [u8]) {
    let mut BEGIN: i32 = 0;
    let mut END: i32 = 0;
    let mut LCHAR = [b' '; 1];
    let mut ZERO: i32 = 0;
    let mut I: i32 = 0;
    let mut J: i32 = 0;
    let mut K: i32 = 0;
    let mut L: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Find the first and last non-blank characters in the string.
    //
    BEGIN = intrinsics::MAX0(&[1, FRSTNB(IN)]);
    END = intrinsics::MAX0(&[1, LASTNB(IN)]);
    fstr::assign(&mut LCHAR, b" ");
    //
    // Trivial case.
    //
    if (BEGIN == END) {
        fstr::assign(
            fstr::substr_mut(OUT, 1..=1),
            fstr::substr(IN, BEGIN..=BEGIN),
        );

        if (intrinsics::LEN(OUT) > 1) {
            fstr::assign(fstr::substr_mut(OUT, 2..), b" ");
        }

    //
    // If there is no decimal point, all zeros are significant.
    //
    } else if (intrinsics::INDEX(IN, b".") == 0) {
        L = 1;
        K = BEGIN;

        while ((L <= intrinsics::LEN(OUT)) && (K <= END)) {
            fstr::assign(fstr::substr_mut(OUT, L..=L), fstr::substr(IN, K..=K));

            //
            // Don't increment L if the last item copied was a space
            // (we don't want to copy extra spaces).
            //
            if (fstr::ne(fstr::substr(IN, K..=K), b" ") || fstr::ne(&LCHAR, b" ")) {
                L = (L + 1);
            }
            fstr::assign(&mut LCHAR, fstr::substr(IN, K..=K));
            K = (K + 1);
        }

        if (L <= intrinsics::LEN(OUT)) {
            fstr::assign(fstr::substr_mut(OUT, L..), b" ");
        }
    } else {
        //
        // Is there is a decimal point and an exponent immediately
        // preceded by zero ('...0E', '...0D', '...0e', '...0d') or
        // by a space ('... E', '... D', '... e', '... d')?
        //
        ZERO = intrinsics::INDEX(IN, b"0E");

        if (ZERO == 0) {
            ZERO = intrinsics::INDEX(IN, b"0D");
        }
        if (ZERO == 0) {
            ZERO = intrinsics::INDEX(IN, b"0e");
        }
        if (ZERO == 0) {
            ZERO = intrinsics::INDEX(IN, b"0d");
        }
        if (ZERO == 0) {
            ZERO = intrinsics::INDEX(IN, b" E");
        }
        if (ZERO == 0) {
            ZERO = intrinsics::INDEX(IN, b" D");
        }
        if (ZERO == 0) {
            ZERO = intrinsics::INDEX(IN, b" e");
        }
        if (ZERO == 0) {
            ZERO = intrinsics::INDEX(IN, b" d");
        }

        //
        // Begin there, and move toward the front of the string until
        // something other than a blank or a zero is encountered. Then
        // remove the superfluous characters.
        //
        if (ZERO > 0) {
            J = (ZERO + 1);
            I = ZERO;

            while (fstr::eq(fstr::substr(IN, I..=I), b"0")
                || fstr::eq(fstr::substr(IN, I..=I), b" "))
            {
                I = (I - 1);
            }

            L = 1;
            K = BEGIN;

            while ((L <= intrinsics::LEN(OUT)) && (K <= I)) {
                fstr::assign(fstr::substr_mut(OUT, L..=L), fstr::substr(IN, K..=K));

                //
                // Don't increment L if the last item copied was a space.
                //
                if (fstr::ne(fstr::substr(IN, K..=K), b" ") || fstr::ne(&LCHAR, b" ")) {
                    L = (L + 1);
                }
                fstr::assign(&mut LCHAR, fstr::substr(IN, K..=K));
                K = (K + 1);
            }

            K = J;

            while ((L <= intrinsics::LEN(OUT)) && (K <= END)) {
                fstr::assign(fstr::substr_mut(OUT, L..=L), fstr::substr(IN, K..=K));

                //
                // Increment L only if we don't have two consecutive
                // spaces.
                //
                if (fstr::ne(fstr::substr(IN, K..=K), b" ") || fstr::ne(&LCHAR, b" ")) {
                    L = (L + 1);
                }
                fstr::assign(&mut LCHAR, fstr::substr(IN, K..=K));
                K = (K + 1);
            }

            if (L <= intrinsics::LEN(OUT)) {
                fstr::assign(fstr::substr_mut(OUT, L..), b" ");
            }
        //
        //
        // Is there is a decimal point and no exponent, and is the last
        // non-blank character a zero ('...0')? Then truncate the string
        // after the last character that is neither a blank nor a zero.
        //
        } else if (fstr::eq(fstr::substr(IN, END..=END), b"0") && (CPOS(IN, b"EeDd", 1) == 0)) {
            I = END;

            while (fstr::eq(fstr::substr(IN, I..=I), b"0")
                || fstr::eq(fstr::substr(IN, I..=I), b" "))
            {
                I = (I - 1);
            }

            L = 1;
            K = BEGIN;

            while ((L <= intrinsics::LEN(OUT)) && (K <= I)) {
                fstr::assign(fstr::substr_mut(OUT, L..=L), fstr::substr(IN, K..=K));

                //
                // Increment L only if we don't have two consecutive
                // spaces.
                //
                if (fstr::ne(fstr::substr(IN, K..=K), b" ") || fstr::ne(&LCHAR, b" ")) {
                    L = (L + 1);
                }
                fstr::assign(&mut LCHAR, fstr::substr(IN, K..=K));
                K = (K + 1);
            }

            if (L <= intrinsics::LEN(OUT)) {
                fstr::assign(fstr::substr_mut(OUT, L..), b" ");
            }
        } else {
            L = 1;
            K = BEGIN;

            while ((L <= intrinsics::LEN(OUT)) && (K <= END)) {
                fstr::assign(fstr::substr_mut(OUT, L..=L), fstr::substr(IN, K..=K));

                //
                // Increment L only if we don't have two consecutive spaces.
                //
                if (fstr::ne(fstr::substr(IN, K..=K), b" ") || fstr::ne(&LCHAR, b" ")) {
                    L = (L + 1);
                }
                fstr::assign(&mut LCHAR, fstr::substr(IN, K..=K));
                K = (K + 1);
            }

            if (L <= intrinsics::LEN(OUT)) {
                fstr::assign(fstr::substr_mut(OUT, L..), b" ");
            }
        }
    }

    //
    // Special case. The string '.0000....' reduces to '.' after the
    // zeros are removed.
    //
    if fstr::eq(OUT, b".") {
        fstr::assign(OUT, b"0");
    }
}
