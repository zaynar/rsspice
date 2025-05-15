//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Build a list in a character string
///
/// Build a list of items delimited by a character.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ITEMS      I   Items in the list.
///  N          I   Number of items in the list.
///  DELIM      I   String used to delimit items.
///  LIST       O   List of items delimited by DELIM.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ITEMS    are the items to be combined to make the output
///           list. Leading and trailing blanks are ignored.
///           (Only the non-blank parts of the items are used.)
///
///  N        is the number of items.
///
///  DELIM    is the string used to delimit the items in the
///           output list. DELIM may contain any number of
///           characters, including blanks.
/// ```
///
/// # Detailed Output
///
/// ```text
///  LIST     is the output list, containing the N elements of
///           ITEMS delimited by DELIM. If LIST is not long enough
///           to contain the output list, it is truncated on the
///           right.
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
///  The non-blank parts of the elements of the ITEMS array are
///  appended to the list, one at a time, separated by DELIM.
/// ```
///
/// # Examples
///
/// ```text
///  The following examples illustrate the operation of LBUILD.
///
///  1) Let
///           DELIM    = ' '
///
///           ITEMS(1) = 'A'
///           ITEMS(2) = '  number'
///           ITEMS(3) = 'of'
///           ITEMS(4) = ' words'
///           ITEMS(5) = 'separated'
///           ITEMS(6) = '  by'
///           ITEMS(7) = 'spaces'
///
///     Then
///           LIST  = 'A number of words separated by spaces'
///
///  2) Let
///           DELIM    = '/'
///
///           ITEMS(1) = ' '
///           ITEMS(2) = ' '
///           ITEMS(3) = 'option1'
///           ITEMS(4) = ' '
///           ITEMS(5) = 'option2'
///           ITEMS(6) = ' '
///           ITEMS(7) = ' '
///           ITEMS(8) = ' '
///
///     Then
///           LIST  = '//option1//option2///'
///
///  3) Let
///           DELIM    = ' and '
///
///           ITEMS(1) = 'Bob'
///           ITEMS(2) = 'Carol'
///           ITEMS(3) = 'Ted'
///           ITEMS(4) = 'Alice'
///
///     Then
///           LIST  = 'Bob and Carol and Ted and Alice'
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
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
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn lbuild(items: CharArray, n: i32, delim: &str, list: &mut str) {
    LBUILD(
        items,
        n,
        delim.as_bytes(),
        fstr::StrBytes::new(list).as_mut(),
    );
}

//$Procedure LBUILD ( Build a list in a character string )
pub fn LBUILD(ITEMS: CharArray, N: i32, DELIM: &[u8], LIST: &mut [u8]) {
    let ITEMS = DummyCharArray::new(ITEMS, None, 1..);
    let mut LPOS: i32 = 0;
    let mut FIRST: i32 = 0;
    let mut LAST: i32 = 0;
    let mut DLEN: i32 = 0;
    let mut LLEN: i32 = 0;
    let mut ILEN: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Find the non-blank part of each item. Move it to the
    // end of the list, followed by a delimiter. If the item is
    // blank, don't move anything but the delimiter.
    //
    // LPOS is the next position in the output list to be filled.
    // LLEN is the length of the output list.
    // DLEN is the length of DELIM.
    // ILEN is the length of the next item in the list.
    //
    fstr::assign(LIST, b" ");
    LPOS = 1;
    LLEN = intrinsics::LEN(LIST);
    DLEN = intrinsics::LEN(DELIM);

    if (N > 0) {
        for I in 1..=N {
            if (LPOS <= LLEN) {
                if fstr::eq(ITEMS.get(I), b" ") {
                    fstr::assign(fstr::substr_mut(LIST, LPOS..), DELIM);
                    LPOS = (LPOS + DLEN);
                } else {
                    FIRST = FRSTNB(&ITEMS[I]);
                    LAST = LASTNB(&ITEMS[I]);
                    ILEN = ((LAST - FIRST) + 1);

                    fstr::assign(
                        fstr::substr_mut(LIST, LPOS..),
                        fstr::substr(ITEMS.get(I), FIRST..=LAST),
                    );
                    SUFFIX(DELIM, 0, LIST);

                    LPOS = ((LPOS + ILEN) + DLEN);
                }
            }
        }

        //
        // We're at the end of the list. Right now, the list ends in
        // a delimiter. Drop it.
        //
        if ((LPOS - DLEN) <= LLEN) {
            fstr::assign(fstr::substr_mut(LIST, (LPOS - DLEN)..), b" ");
        }
    }
}
