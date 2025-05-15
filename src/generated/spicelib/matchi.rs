//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Match string against wildcard template
///
/// Determine whether a string is matched by a template containing
/// wild cards. This routine is case-insensitive.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  STRING     I   String to be tested.
///  TEMPL      I   Template (with wild cards) to test against STRING.
///  WSTR       I   Wild string token.
///  WCHR       I   Wild character token.
///
///  The function returns .TRUE. if STRING matches TEMPL and otherwise
///  returns .FALSE.
/// ```
///
/// # Detailed Input
///
/// ```text
///  STRING   is the input character string to be tested for a match
///           against the input template. Leading and trailing blanks
///           are ignored.
///
///  TEMPL    is the input template to be tested for a match against
///           the input string. TEMPL may contain wild cards. Leading
///           and trailing blanks are ignored.
///
///  WSTR     is the wild string token used in the input template. The
///           wild string token may represent from zero to any number
///           of characters.
///
///  WCHR     is the wild character token used in the input template.
///           The wild character token represents exactly one
///           character.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns .TRUE. when the input string matches the
///  input template, and .FALSE. otherwise. The string and template
///  match whenever the template can expand (through replacement of its
///  wild cards) to become the input string.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
/// ```
///
/// # Particulars
///
/// ```text
///  MATCHI ignores leading and trailing blanks in both the string
///  and the template. All of the following are equivalent (they
///  all return .TRUE.).
///
///     MATCHI ( 'ALCATRAZ',     'A*Z',      '*', '%' )
///     MATCHI ( '  ALCATRAZ  ', 'A*Z',      '*', '%' )
///     MATCHI ( 'ALCATRAZ',     '  A*Z  ',  '*', '%' )
///     MATCHI ( '  ALCATRAZ  ', '  A*Z  ',  '*', '%' )
///
///  MATCHI is case-insensitive:  uppercase characters match
///  lowercase characters, and vice versa. Wild characters match
///  characters of both cases.
/// ```
///
/// # Examples
///
/// ```text
///  Let
///
///     STRING  = '  ABCDEFGHIJKLMNOPQRSTUVWXYZ  '
///     WSTR    = '*'
///     WCHR    = '%'
///
///  Then
///
///     if TEMPL is  '*A*'        MATCHI is   T
///                  'A%D*'                     F
///                  'A%C*'                   T
///                  '%A*'                      F
///                  '%%CD*Z'                 T
///                  '%%CD'                     F
///                  'A*MN*Y*Z'               T
///                  'A*MN*Y*%Z'                F
///                  '*BCD*Z*'                T
///                  '*bdc*z*'                  F
///                  ' *bcD*Z*  '             T
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
/// -    SPICELIB Version 1.4.0, 17-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.3.1, 11-NOV-2005 (NJB)
///
///         Corrected example calls in header; made other minor
///         edits to header.
///
/// -    SPICELIB Version 1.1.0, 08-JUN-1999 (WLT)
///
///          Fixed comments in detailed output and example sections.
///
/// -    SPICELIB Version 1.0.0, 01-DEC-1995 (WLT)
/// ```
pub fn matchi(ctx: &mut SpiceContext, string: &str, templ: &str, wstr: char, wchr: char) -> bool {
    let ret = MATCHI(
        string.as_bytes(),
        templ.as_bytes(),
        &[u8::try_from(wstr).unwrap()],
        &[u8::try_from(wchr).unwrap()],
        ctx.raw_context(),
    );
    ret
}

//$Procedure MATCHI ( Match string against wildcard template )
pub fn MATCHI(STRING: &[u8], TEMPL: &[u8], WSTR: &[u8], WCHR: &[u8], ctx: &mut Context) -> bool {
    let WSTR = &WSTR[..1];
    let WCHR = &WCHR[..1];
    let mut MATCHI: bool = false;
    let mut SFIRST: i32 = 0;
    let mut TFIRST: i32 = 0;
    let mut SLAST: i32 = 0;
    let mut TLAST: i32 = 0;
    let mut SLEN: i32 = 0;
    let mut TLEN: i32 = 0;
    let mut SCUR: i32 = 0;
    let mut TCUR: i32 = 0;
    let mut LEFT: i32 = 0;
    let mut RIGHT: i32 = 0;
    let mut I: i32 = 0;
    let mut J: i32 = 0;
    let mut NOSUBM: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local Variables
    //

    //
    // Give the function an initial value of .FALSE.
    //
    MATCHI = false;

    //
    // First let's get everybody's measurements.
    //
    SFIRST = FRSTNB(STRING);
    SLAST = LASTNB(STRING);
    TFIRST = FRSTNB(TEMPL);
    TLAST = LASTNB(TEMPL);

    TLEN = ((TLAST - TFIRST) + 1);
    SLEN = ((SLAST - SFIRST) + 1);

    SCUR = intrinsics::MAX0(&[1, SFIRST]);
    TCUR = TFIRST;

    //
    // A blank template matches a blank string, and nothing else.
    //
    if ((TLAST == 0) && (SLAST == 0)) {
        MATCHI = true;
        return MATCHI;
    } else if (TLAST == 0) {
        MATCHI = false;
        return MATCHI;
    }

    //
    // The beginning of the string and template must be identical
    // up to the first occurrence of a wild string.
    //

    while (((TCUR <= TLAST) && (SCUR <= SLAST)) && !SAMCH(TEMPL, TCUR, WSTR, 1)) {
        if (NECHR(
            fstr::substr(TEMPL, TCUR..=TCUR),
            fstr::substr(STRING, SCUR..=SCUR),
            ctx,
        ) && fstr::ne(fstr::substr(TEMPL, TCUR..=TCUR), WCHR))
        {
            MATCHI = false;
            return MATCHI;
        } else {
            TCUR = (TCUR + 1);
            SCUR = (SCUR + 1);
        }
    }

    //
    // There are a three ways we could have finished the loop above
    // without hitting a wild string.
    //
    // Case 1.  Both the string and template ran out of characters at
    // the same time without running into a wild string in the template.
    //
    if ((TCUR > TLAST) && (SCUR > SLAST)) {
        MATCHI = true;
        return MATCHI;
    }

    //
    // Case 2. The template ran out of characters while there were still
    // characters remaining in the in the string.  No match.
    //
    if ((TCUR > TLAST) && (SCUR <= SLAST)) {
        MATCHI = false;
        return MATCHI;
    }

    //
    // Case 3. The string ran out of characters while non-wild characters
    // remain in the template.
    //
    // We have to check to see if any non-wild-string characters
    // remain.  If so, we DO NOT have a match.  On the other hand if
    // only wild string characters remain we DO have a match.
    //
    if ((TCUR <= TLAST) && (SCUR > SLAST)) {
        MATCHI = true;

        {
            let m1__: i32 = TCUR;
            let m2__: i32 = TLAST;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                MATCHI = (MATCHI && fstr::eq(fstr::substr(TEMPL, I..=I), WSTR));
                I += m3__;
            }
        }

        return MATCHI;
    }

    //
    // OK. There is only one way that you can get to this point.
    // It must be the case that characters remain in both the template
    // (TCUR .LE. TLAST) and the string (SCUR .LE. SLAST).  Moreover,
    // to get out of the first loop you had to hit a wild string
    // character.  Find the first non-wild-string character in the
    // template. (If there isn't one, we have a match.)
    //

    while ((TCUR <= TLAST) && SAMCH(TEMPL, TCUR, WSTR, 1)) {
        TCUR = (TCUR + 1);
    }

    if (TCUR > TLAST) {
        MATCHI = true;
        return MATCHI;
    }

    //
    // Still here? Ok. We have a non-wild-string character at TCUR. Call
    // this position left and look for the right end of the maximum
    // length substring of TEMPL (starting at left) that does not have
    // a wild string character.
    //
    LEFT = TCUR;

    while ((TCUR <= TLAST) && !SAMCH(TEMPL, TCUR, WSTR, 1)) {
        TCUR = (TCUR + 1);
    }

    RIGHT = (TCUR - 1);

    while (LEFT <= TLAST) {
        //
        // First see if there is enough room left in the string to
        // match TEMPL(LEFT:RIGHT)
        //
        if ((SLAST - SCUR) < (RIGHT - LEFT)) {
            MATCHI = false;
            return MATCHI;
        }

        //
        // The substring TEMPL(LEFT:RIGHT) might be the end of the
        // string.  In such a case the ends of STRING must match
        // exactly with the end of TEMPL.
        //
        if (RIGHT == TLAST) {
            I = SLAST;
            J = TLAST;

            while (J >= LEFT) {
                if (SAMCH(TEMPL, J, WCHR, 1) || SAMCHI(TEMPL, J, STRING, I, ctx)) {
                    J = (J - 1);
                    I = (I - 1);
                } else {
                    MATCHI = false;
                    return MATCHI;
                }
            }

            //
            // If we made it through the loop, we've got a match.
            //
            MATCHI = true;
            return MATCHI;
        } else {
            //
            // In this case TEMPL(LEFT:RIGHT) is in between wild string
            // characters.  Try to find a substring at or to the right
            // of SCUR in STRING that matches TEMPL(LEFT:RIGHT)
            //
            NOSUBM = true;

            while NOSUBM {
                I = SCUR;
                J = LEFT;

                while ((J <= RIGHT)
                    && (SAMCHI(STRING, I, TEMPL, J, ctx) || SAMCH(WCHR, 1, TEMPL, J)))
                {
                    I = (I + 1);
                    J = (J + 1);
                }

                //
                // If J made it past RIGHT, we have a substring match
                //
                if (J > RIGHT) {
                    SCUR = I;
                    NOSUBM = false;

                //
                // Otherwise, try the substring starting 1 to the right
                // of where our last try began.
                //
                } else {
                    SCUR = (SCUR + 1);

                    //
                    // Make sure there's room to even attempt a match.
                    //
                    if ((SLAST - SCUR) < (RIGHT - LEFT)) {
                        MATCHI = false;
                        return MATCHI;
                    }
                }
            }
        }

        //
        // If you have reached this point there must be something left
        // in the template and that something must begin with a wild
        // string character.  Hunt for the next substring that doesn't
        // contain a wild string character.
        //
        while ((TCUR <= TLAST) && SAMCH(TEMPL, TCUR, WSTR, 1)) {
            TCUR = (TCUR + 1);
        }

        if (TCUR > TLAST) {
            //
            // All that was left was wild string characters.  We've
            // got a match.
            //
            MATCHI = true;
            return MATCHI;
        }

        //
        // Still here? Ok. We have a non-wild-string character at TCUR.
        // Call this position left and look for the right end of the
        // maximum length substring of TEMPL (starting at left) that
        // does not have a wild string character.
        //
        LEFT = TCUR;

        while ((TCUR <= TLAST) && !SAMCH(TEMPL, TCUR, WSTR, 1)) {
            TCUR = (TCUR + 1);
        }

        RIGHT = (TCUR - 1);
    }

    MATCHI
}
