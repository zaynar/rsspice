//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const BLANK: &[u8; 1 as usize] = &fstr::extend_const::<{ 1 as usize }>(b" ");
const ISPACE: i32 = 32;

/// Parse a list of items
///
/// Parse a list of items separated by multiple delimiters.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  LIST       I    List of items delimited by DELIMS.
///  DELIMS     I    Single characters which delimit items.
///  NMAX       I    Maximum number of items to return.
///  N          O    Number of items in the list.
///  ITEMS      O    Items in the list, left justified.
/// ```
///
/// # Detailed Input
///
/// ```text
///  LIST     is a list of items delimited by any one of the
///           characters in the string DELIMS. Consecutive
///           delimiters, and delimiters at the beginning and
///           end of the list, are considered to delimit blank
///           items. A blank list is considered to contain
///           a single (blank) item.
///
///  DELIMS   contains the individual characters which delimit
///           the items in the list. These may be any ASCII
///           characters, including blanks.
///
///           However, by definition, consecutive blanks are NOT
///           considered to be consecutive delimiters. Nor are
///           a blank and any other delimiter considered to be
///           consecutive delimiters. In addition, leading and
///           trailing blanks are ignored.
///
///  NMAX     is the maximum number of items to be returned from
///           the list. This allows the user to guard against
///           overflow from a list containing more items than
///           expected.
/// ```
///
/// # Detailed Output
///
/// ```text
///  N        is the number of items in the list. N may be
///           any number between one and NMAX. N is always the
///           number of delimiters plus one.
///
///  ITEMS    are the items in the list, left justified. Any item
///           in the list to long to fit into an element of ITEMS
///           is truncated on the right.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If the string length of ITEMS is too short to accommodate
///      an item, the item will be truncated on the right.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for these examples may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Parse a character string to retrieve the words contained
///     within.
///
///
///     Example code begins here.
///
///
///           PROGRAM LPARSM_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local constants.
///     C
///           INTEGER                 DELMLN
///           PARAMETER             ( DELMLN = 1   )
///
///           INTEGER                 NMAX
///           PARAMETER             ( NMAX   = 25  )
///
///           INTEGER                 STRLEN
///           PARAMETER             ( STRLEN = 255 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(DELMLN)      DELIMS
///           CHARACTER*(STRLEN)      ITEMS  ( NMAX )
///           CHARACTER*(STRLEN)      LIST
///
///           INTEGER                 I
///           INTEGER                 N
///
///     C
///     C     Define the list of delimited items.
///     C
///     C     Think of a sentence as a list delimited by a space.
///     C     DELIMS is assigned to a space.
///     C
///           LIST   = 'Run and find out.'
///           DELIMS = ' '
///
///     C
///     C     Parse the items from LIST.
///     C
///           CALL LPARSM ( LIST, DELIMS, NMAX, N, ITEMS )
///
///     C
///     C     Output the ITEMS.
///     C
///           DO I = 1, N
///
///              WRITE(*,'(A,I3,2A)') 'Item', I, ': ', ITEMS(I)
///
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Item  1: Run
///     Item  2: and
///     Item  3: find
///     Item  4: out.
///
///
///  2) Parse a character string to retrieve the items contained
///     within, when then items are separated by multiple delimiters.
///
///
///     Example code begins here.
///
///
///           PROGRAM LPARSM_EX2
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions.
///     C
///           INTEGER                 RTRIM
///
///     C
///     C     Local constants.
///     C
///           INTEGER                 DELMLN
///           PARAMETER             ( DELMLN = 5   )
///
///           INTEGER                 NMAX
///           PARAMETER             ( NMAX   = 25  )
///
///           INTEGER                 STRLEN
///           PARAMETER             ( STRLEN = 255 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(DELMLN)      DELIMS
///           CHARACTER*(STRLEN)      ITEMS  ( NMAX )
///           CHARACTER*(STRLEN)      LIST
///
///           INTEGER                 I
///           INTEGER                 N
///
///     C
///     C     Define the list of delimited items.
///     C
///     C     Think of a sentence as a list delimited by a space.
///     C     DELIMS is assigned to a space.
///     C
///           LIST   = '  1986-187// 13:15:12.184 '
///           DELIMS = ' ,/-:'
///
///     C
///     C     Parse the items from LIST.
///     C
///           CALL LPARSM ( LIST, DELIMS, NMAX, N, ITEMS )
///
///     C
///     C     Output the ITEMS.
///     C
///           DO I = 1, N
///
///              WRITE(*,'(A,I3,3A)') 'Item', I, ': ''',
///          .                        ITEMS(I)(:RTRIM(ITEMS(I))), ''''
///
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Item  1: '1986'
///     Item  2: '187'
///     Item  3: ' '
///     Item  4: '13'
///     Item  5: '15'
///     Item  6: '12.184'
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.1, 13-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///         Added complete code example.
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
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
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
/// ```
pub fn lparsm(list: &str, delims: &str, nmax: i32, n: &mut i32, items: CharArrayMut) {
    LPARSM(list.as_bytes(), delims.as_bytes(), nmax, n, items);
}

//$Procedure LPARSM ( Parse a list of items )
pub fn LPARSM(LIST: &[u8], DELIMS: &[u8], NMAX: i32, N: &mut i32, ITEMS: CharArrayMut) {
    let mut ITEMS = DummyCharArrayMut::new(ITEMS, None, 1..);
    let mut BCHR = [b' '; 1 as usize];
    let mut ECHR = [b' '; 1 as usize];
    let mut B: i32 = 0;
    let mut E: i32 = 0;
    let mut EOL: i32 = 0;

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Because speed is essential in many list parsing applications,
    // LPARSM parses the input list in a single pass. What follows
    // is nearly identical to LPARSE, except the Fortran INDEX function
    // is used to test for delimiters, instead of testing each character
    // for simple equality.
    //

    //
    // Nothing yet.
    //
    *N = 0;

    //
    // Blank list contains a blank item.
    //
    if fstr::eq(LIST, BLANK) {
        *N = 1;
        fstr::assign(ITEMS.get_mut(1), BLANK);
    } else {
        //
        // Eliminate trailing blanks. EOL is the last non-blank
        // character in the list.
        //
        EOL = intrinsics::LEN(LIST);

        while (intrinsics::ICHAR(fstr::substr(LIST, EOL..=EOL)) == ISPACE) {
            EOL = (EOL - 1);
        }

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
            *N = (*N + 1);

            if (E > B) {
                fstr::assign(ITEMS.get_mut(*N), fstr::substr(LIST, B..=(E - 1)));
            } else {
                fstr::assign(ITEMS.get_mut(*N), BLANK);
            }

            //
            // If there are more items to be found, continue with
            // character following E (which is a delimiter).
            //
            if (*N < NMAX) {
                B = (E + 1);
            } else {
                return;
            }
        }

        //
        // If the list ended with a (non-blank) delimiter, add a
        // blank item to the end.
        //
        if ((intrinsics::INDEX(DELIMS, fstr::substr(LIST, EOL..=EOL)) != 0) && (*N < NMAX)) {
            *N = (*N + 1);
            fstr::assign(ITEMS.get_mut(*N), BLANK);
        }
    }
}
