//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
const BLANK: &[u8; 1 as usize] = &fstr::extend_const::<{ 1 as usize }>(b" ");
const ISPACE: i32 = 32;

/// Parse a list of items; return a set.
///
/// Parse a list of items delimited by multiple delimiters,
/// placing the resulting items into a set.
///
/// # Required Reading
///
/// * [CELLS](crate::required_reading::cells)
/// * [SETS](crate::required_reading::sets)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  LIST       I   List of items delimited by DELIMS on input.
///  DELIMS     I   Single characters which delimit items.
///  SET        O   Items in the list, validated, left justified.
/// ```
///
/// # Detailed Input
///
/// ```text
///  LIST     is a list of items delimited by any one of the characters
///           in the string DELIMS. Consecutive delimiters, and
///           delimiters at the beginning and end of the list, are
///           considered to delimit blank items. A blank list is
///           considered to contain a single, blank item. Leading and
///           trailing blanks in list are ignored.
///
///  DELIMS   contains the individual characters which delimit the
///           items in the list. These may be any ASCII characters,
///           including blanks.
///
///           However, by definition, consecutive blanks are NOT
///           considered to be consecutive delimiters. Nor are a blank
///           and any other delimiter considered to be consecutive
///           delimiters. In addition, leading and trailing blanks are
///           ignored.
/// ```
///
/// # Detailed Output
///
/// ```text
///  SET      is a SPICE set containing the items in the list, left
///           justified. Any item in the list too long to fit into an
///           element of SET is truncated on the right.
///
///           The strings in SET will be sorted in increasing order,
///           and duplicates will be removed. Trailing blanks are
///           ignored in string comparisons.
///
///           The size of the set must be initialized prior to calling
///           LPARSS.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the size of the set is not large enough to accommodate all
///      of the items in the set, an error is signaled by a routine in
///      the call tree of this routine.
///
///  2)  If the string length of SET is too short to accommodate an
///      item, the item will be truncated on the right.
///
///  3)  If the string length of SET is too short to permit encoding of
///      integers via the SPICELIB routine ENCHAR, an error is signaled
///      by a routine in the call tree of this routine.
/// ```
///
/// # Examples
///
/// ```text
///  The following examples illustrate the operation of LPARSS.
///
///  1) Let
///           LIST        = 'A number of words   separated   by
///                           spaces.'
///           DELIMS      = ' ,.'
///           SIZE (SET)  = 20
///
///     Then
///
///           CARDC (SET) = 8
///
///           SET (1)     = ' '
///           SET (2)     = 'A'
///           SET (3)     = 'by'
///           SET (4)     = 'number'
///           SET (5)     = 'of'
///           SET (6)     = 'separated'
///           SET (7)     = 'spaces'
///           SET (8)     = 'words'
///
///
///  2) Let
///
///           LIST        = '  1986-187// 13:15:12.184 '
///           DELIMS      = ' ,/-:'
///           SIZE (SET)  = 20
///
///     Then
///
///           CARDC (SET) = 6
///
///           SET (1)     = ' '
///           SET (2)     = '12.184'
///           SET (3)     = '13'
///           SET (4)     = '15'
///           SET (5)     = '187'
///           SET (6)     = '1986'
///
///
///  3) Let   LIST        = '  ,This,  is, ,an,, example, '
///           DELIMS      = ' ,'
///           SIZE (SET)  = 20
///
///     Then
///           CARDC (SET) = 5
///
///           SET (1)     = ' '
///           SET (2)     = 'This'
///           SET (3)     = 'an'
///           SET (4)     = 'example'
///           SET (5)     = 'is'
///
///
///  4) Let   LIST        = 'Mary had a little lamb, little lamb
///                          whose fleece was white      as snow.'
///           DELIMS      = ' ,.'
///           SIZE (SET)  = 6
///
///     An error would be signaled because the set is not
///     large enough to accommodate all of the items in the
///     list.
///
///
///  5) Let   LIST        = '1 2 3 4 5 6 7 8 9 10.'
///           DELIMS      = ' .'
///           SIZE (SET)  = 10
///
///     An error would be signaled because the set is not
///     large enough to accommodate all of the items in the
///     list. Note that delimiters at the end (or beginning)
///     of list are considered to delimit blank items.
///
///
///  6) Let   LIST        = '1 2 3 4 5 6 7 8 9 10.'
///           DELIMS      = '.'
///           SIZE (SET)  = 10
///
///     Then
///
///           CARDC (SET) = 2
///
///           SET (1)     = ' '
///           SET (2)     = '1 2 3 4 5 6 7 8 9 10'
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
/// -    SPICELIB Version 1.2.0, 24-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Improved
///         documentation of arguments LIST, DELIM and SET.
///
///         Updated entries #2 and #3 in $Exceptions section: changed
///         wrong argument name, and indicated that the routine used
///         for encoding is part of SPICELIB.
///
/// -    SPICELIB Version 1.1.0, 26-OCT-2005 (NJB)
///
///         Bug fix: code was modified to avoid out-of-range
///         substring bound conditions.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (HAN) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.1.0, 26-OCT-2005 (NJB)
///
///         Bug fix: code was modified to avoid out-of-range
///         substring bound conditions. The previous version
///         of this routine used DO WHILE statements of the form
///
///                   DO WHILE (      ( B         .LE. EOL   )
///            .                .AND. ( LIST(B:B) .EQ. BLANK ) )
///
///         Such statements can cause index range violations when the
///         index B is greater than the length of the string LIST.
///         Whether or not such violations occur is platform-dependent.
///
///
/// -    Beta Version 2.0.0, 10-JAN-1989 (HAN)
///
///         Error handling was added, and old error flags and their
///         checks were removed. An error is signaled if the set
///         is not large enough to accommodate all of the items in
///         the list.
///
///         The header documentation was updated to reflect the error
///         handling changes, and more examples were added.
/// ```
pub fn lparss(
    ctx: &mut SpiceContext,
    list: &str,
    delims: &str,
    set: CharArrayMut,
) -> crate::Result<()> {
    LPARSS(list.as_bytes(), delims.as_bytes(), set, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure LPARSS ( Parse a list of items; return a set. )
pub fn LPARSS(
    LIST: &[u8],
    DELIMS: &[u8],
    SET: CharArrayMut,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut SET = DummyCharArrayMut::new(SET, None, LBCELL..);
    let mut BCHR = [b' '; 1 as usize];
    let mut ECHR = [b' '; 1 as usize];
    let mut B: i32 = 0;
    let mut E: i32 = 0;
    let mut EOL: i32 = 0;
    let mut N: i32 = 0;
    let mut NMAX: i32 = 0;
    let mut VALID: bool = false;

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
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"LPARSS", ctx)?;
    }

    //
    // Because speed is essential in many list parsing applications,
    // LPARSS, like LPARSE, parses the input list in a single pass.
    // What follows is nearly identical to LPARSE, except the FORTRAN
    // INDEX function is used to test for delimiters, instead of testing
    // each character for simple equality. Also, the items are inserted
    // into a set instead of simply placed at the end of an array.
    //
    // No items yet.
    //
    N = 0;

    //
    // What is the size of the set?
    //
    NMAX = SIZEC(SET.as_arg(), ctx)?;

    //
    // The array has not been validated yet.
    //
    VALID = false;

    //
    // Blank list contains a blank item.  No need to validate.
    //
    if fstr::eq(LIST, BLANK) {
        SCARDC(0, SET.as_arg_mut(), ctx)?;
        INSRTC(BLANK, SET.as_arg_mut(), ctx)?;

        VALID = true;
    } else {
        //
        // Eliminate trailing blanks.  EOL is the last non-blank
        // character in the list.
        //
        EOL = LASTNB(LIST);

        //
        // As the King said to Alice: 'Begin at the beginning.
        // Continue until you reach the end. Then stop.'
        //
        // When searching for items, B is the beginning of the current
        // item; E is the end.  E points to the next non-blank delimiter,
        // if any; otherwise E points to either the last character
        // preceding the next item, or to the last character of the list.
        //
        B = 1;

        while (B <= EOL) {
            //
            // Skip any blanks before the next item or delimiter.
            //
            // At this point in the loop, we know
            //
            //    B <= EOL
            //
            fstr::assign(&mut BCHR, fstr::substr(LIST, B..=B));

            while ((B <= EOL) && (intrinsics::ICHAR(&BCHR) == ISPACE)) {
                B = (B + 1);

                if (B <= EOL) {
                    fstr::assign(&mut BCHR, fstr::substr(LIST, B..=B));
                }
            }

            //
            // At this point B is the index of the next non-blank
            // character BCHR, or else
            //
            //    B == EOL + 1
            //
            // The item ends at the next delimiter.
            //
            E = B;

            if (E <= EOL) {
                fstr::assign(&mut ECHR, fstr::substr(LIST, E..=E));
            } else {
                fstr::assign(&mut ECHR, BLANK);
            }

            while ((E <= EOL) && (intrinsics::INDEX(DELIMS, &ECHR) == 0)) {
                E = (E + 1);

                if (E <= EOL) {
                    fstr::assign(&mut ECHR, fstr::substr(LIST, E..=E));
                }
            }

            //
            // (This is different from LPARSE. If the delimiter was
            // a blank, find the next non-blank character. If it's not
            // a delimiter, back up. This prevents constructions
            // like 'a , b', where the delimiters are blank and comma,
            // from being interpreted as three items instead of two.
            // By definition, consecutive blanks, or a blank and any
            // other delimiter, do not count as consecutive delimiters.)
            //
            if ((E <= EOL) && (intrinsics::ICHAR(&ECHR) == ISPACE)) {
                //
                // Find the next non-blank character.
                //
                while ((E <= EOL) && (intrinsics::ICHAR(&ECHR) == ISPACE)) {
                    E = (E + 1);

                    if (E <= EOL) {
                        fstr::assign(&mut ECHR, fstr::substr(LIST, E..=E));
                    }
                }

                if (E <= EOL) {
                    if (intrinsics::INDEX(DELIMS, &ECHR) == 0) {
                        //
                        // We're looking at a non-delimiter character.
                        //
                        // E is guaranteed to be > 1 if we're here, so the
                        // following subtraction is valid.
                        //
                        E = (E - 1);
                    }
                }
            }

            //
            // The item now lies between B and E. Unless, of course, B and
            // E are the same character; this can happen if the list
            // starts or ends with a non-blank delimiter, or if we have
            // stumbled upon consecutive delimiters.
            //
            if !VALID {
                //
                // If the array has not been validated, it's just an
                // array, and we can insert items directly into it.
                // Unless it's full, in which case we validate now and
                // insert later.
                //
                if (N < NMAX) {
                    N = (N + 1);

                    if (E > B) {
                        fstr::assign(SET.get_mut(N), fstr::substr(LIST, B..=(E - 1)));
                    } else {
                        fstr::assign(SET.get_mut(N), BLANK);
                    }
                } else {
                    VALIDC(NMAX, NMAX, SET.as_arg_mut(), ctx)?;
                    VALID = true;
                }
            }

            //
            // Once the set has been validated, the strings are inserted
            // into the set if there's room. If there is not enough room
            // in the set, let INSRTC signal the error.
            //
            if VALID {
                if (E > B) {
                    INSRTC(fstr::substr(LIST, B..=(E - 1)), SET.as_arg_mut(), ctx)?;
                } else {
                    INSRTC(BLANK, SET.as_arg_mut(), ctx)?;
                }

                if FAILED(ctx) {
                    CHKOUT(b"LPARSS", ctx)?;
                    return Ok(());
                }
            }

            //
            // If there are more items to be found, continue with the
            // character following E (which is a delimiter).
            //
            B = (E + 1);
        }

        //
        // If the array has not yet been validated, validate it before
        // returning.
        //
        if !VALID {
            VALIDC(NMAX, N, SET.as_arg_mut(), ctx)?;
        }

        //
        // If the list ended with a (non-blank) delimiter, insert a
        // blank item into the set. If there isn't any room, signal
        // an error.
        //
        if (intrinsics::INDEX(DELIMS, fstr::substr(LIST, EOL..=EOL)) != 0) {
            INSRTC(BLANK, SET.as_arg_mut(), ctx)?;
            //
            // If INSRTC failed to insert the blank because the set
            // was already full, INSRTC will have signaled an error.
            // No action is necessary here.
            //
        }
    }

    CHKOUT(b"LPARSS", ctx)?;
    Ok(())
}
