//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const BLANK: &[u8; 1 as usize] = &fstr::extend_const::<{ 1 as usize }>(b" ");
const ISPACE: i32 = 32;

/// Parse items from a list
///
/// Parse a list of items delimited by a single character.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  LIST       I   List of items delimited by DELIM.
///  DELIM      I   Single character used to delimit items.
///  NMAX       I   Maximum number of items to return.
///  N          O   Number of items in the list.
///  ITEMS      O   Items in the list, left justified.
/// ```
///
/// # Detailed Input
///
/// ```text
///  LIST     is a list of items delimited by the single character
///           DELIM. Consecutive delimiters, and delimiters at the
///           beginning and end of the list, are considered to
///           delimit blank items. A blank list is considered to
///           contain a single (blank) item.
///
///  DELIM    is the character delimiting the items in the list.
///           This may be any ASCII character, including a blank.
///           However, by definition, consecutive blanks are NOT
///           considered to be consecutive delimiters. In addition,
///           leading and trailing blanks are ignored.
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
///           in the list too long to fit into an element of ITEMS
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
///     Example code begins here.
///
///
///           PROGRAM LPARSE_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local constants.
///     C
///           INTEGER                 NMAX
///           PARAMETER             ( NMAX   = 25  )
///
///           INTEGER                 STRLEN
///           PARAMETER             ( STRLEN = 255 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(1)           DELIM
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
///     C     DELIM is assigned to a space.
///     C
///           LIST  = 'Run and find out.'
///           DELIM = ' '
///
///     C
///     C     Parse the items from LIST.
///     C
///           CALL LPARSE ( LIST, DELIM, NMAX, N, ITEMS )
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
///  2) Repeat the previous example with different character
///     delimiting the items in the list and different maximum number
///     of items to return.
///
///     Example code begins here.
///
///
///           PROGRAM LPARSE_EX2
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
///           INTEGER                 NCASES
///           PARAMETER             ( NCASES = 2   )
///
///           INTEGER                 NMAXT
///           PARAMETER             ( NMAXT  = 25  )
///
///           INTEGER                 STRLEN
///           PARAMETER             ( STRLEN = 255 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(1)           DELIM  ( NCASES )
///           CHARACTER*(STRLEN)      ITEMS  ( NMAXT  )
///           CHARACTER*(STRLEN)      LIST   ( NCASES )
///
///           INTEGER                 I
///           INTEGER                 J
///           INTEGER                 N
///           INTEGER                 NMAX   ( NCASES )
///
///     C
///     C     Define the lists of delimited items, the delimiting
///     C     character and the maximum number of items to return.
///     C
///           LIST(1)  = '//option1//option2/ //'
///           DELIM(1) = '/'
///           NMAX(1)  = 20
///
///           LIST(2)  = ' ,bob,   carol,, ted,  alice'
///           DELIM(2) = ','
///           NMAX(2)  = 4
///
///           DO I = 1, NCASES
///
///              WRITE(*,'(A,I2,A)') 'Case', I, ':'
///              WRITE(*,'(3A)')   '   String: ''',
///          .                     LIST(I)(:RTRIM(LIST(I))), ''''
///              WRITE(*,'(3A)')   '   DELIM : ''', DELIM(I), ''''
///              WRITE(*,'(A,I3)') '   NMAX  :', NMAX(I)
///              WRITE(*,'(A)')    '   Output items:'
///
///     C
///     C        Parse the items from LIST.
///     C
///              CALL LPARSE ( LIST(I), DELIM(I), NMAX(I), N, ITEMS )
///
///     C
///     C        Output the ITEMS.
///     C
///              DO J = 1, N
///
///                 WRITE(*,'(A,I3,3A)') '      Item', J, ': ''',
///          .                  ITEMS(J)(:RTRIM(ITEMS(J))), ''''
///
///              END DO
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
///     Case 1:
///        String: '//option1//option2/ //'
///        DELIM : '/'
///        NMAX  : 20
///        Output items:
///           Item  1: ' '
///           Item  2: ' '
///           Item  3: 'option1'
///           Item  4: ' '
///           Item  5: 'option2'
///           Item  6: ' '
///           Item  7: ' '
///           Item  8: ' '
///     Case 2:
///        String: ' ,bob,   carol,, ted,  alice'
///        DELIM : ','
///        NMAX  :  4
///        Output items:
///           Item  1: ' '
///           Item  2: 'bob'
///           Item  3: 'carol'
///           Item  4: ' '
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
/// -    SPICELIB Version 1.2.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary entries from $Revisions section.
///
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
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU) (HAN) (NJB)
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
pub fn lparse(list: &str, delim: char, nmax: i32, n: &mut i32, items: CharArrayMut) {
    LPARSE(
        list.as_bytes(),
        &[u8::try_from(delim).unwrap()],
        nmax,
        n,
        items,
    );
}

//$Procedure LPARSE ( Parse items from a list )
pub fn LPARSE(LIST: &[u8], DELIM: &[u8], NMAX: i32, N: &mut i32, ITEMS: CharArrayMut) {
    let DELIM = &DELIM[..1 as usize];
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
    // LPARSE parses the input list in a single pass.
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

        while fstr::eq(fstr::substr(LIST, EOL..=EOL), BLANK) {
            EOL = (EOL - 1);
        }

        //
        // As the king said to Alice: 'Begin at the beginning.
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

            while ((E <= EOL) && fstr::ne(&ECHR, DELIM)) {
                E = (E + 1);

                if (E <= EOL) {
                    fstr::assign(&mut ECHR, fstr::substr(LIST, E..=E));
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
        // If the list ended with a (non-blank) delimiter, add a blank
        // item to the end.
        //
        if (fstr::eq(fstr::substr(LIST, EOL..=EOL), DELIM) && (*N < NMAX)) {
            *N = (*N + 1);
            fstr::assign(ITEMS.get_mut(*N), BLANK);
        }
    }
}
