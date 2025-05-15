//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Same character
///
/// Determine if two characters from different strings are the
/// same.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  STR1       I   A character string
///  L1         I   The location (index) of a character in STR1
///  STR2       I   A character string
///  L2         I   The location (index) of a character in STR2
///
///  The function returns .TRUE. if the two characters are the
///  same.
/// ```
///
/// # Detailed Input
///
/// ```text
///  STR1     is a character string
///
///  L1       is the location (index) of a character in STR1
///
///  STR2     is a character string
///
///  L2       is the location (index) of a character in STR2
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns .TRUE. if the characters STR1(L1:L1) and
///  STR2(L2:L2) are the same.
///
///  If the characters are different or L1 or L2 is out of range the
///  function returns .FALSE.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If either L1 or L2 is out of range the function returns
///      .FALSE.
/// ```
///
/// # Particulars
///
/// ```text
///  This is a utility function for determining whether or not
///  two characters in different strings are the same. This
///  function is intended for situation in which you need to
///  search two strings for a match (or mismatch).
/// ```
///
/// # Examples
///
/// ```text
///  Often you need to scan through two string comparing character
///  by character until a mismatch occurs. The usual way to code
///  this is
///
///     DO WHILE (        L1 .LE. LEN(STR1)
///                 .AND. L2 .LE. LEN(STR2)
///                 .AND. STR1(L1:L1) .EQ. STR2(L2:L2) )
///
///        L1 = L1 + 1
///        L2 = L2 + 1
///
///     END DO
///
///     Check L1, L2 to make sure we are still in range, etc.
///
///  The problem with this loop is that even though the check to make
///  sure that L1 and L2 are in range is performed, FORTRAN may
///  go ahead and compute the equality condition even though one of the
///  first two steps failed. This can lead to out of range errors
///  and possible halting of your program depending upon how
///  the routine is compiled. An alternative way to code this is
///
///     IF ( L1 .LE. LEN(STR1) .AND. L2 .LE. LEN(STR2) ) THEN
///        ALIKE = STR1(L1:L1) .EQ. STR2(L2:L2)
///     ELSE
///        ALIKE = .FALSE.
///     END IF
///
///     DO WHILE ( ALIKE )
///
///        L1 = L1 + 1
///        L2 = L2 + 1
///
///        IF ( L1 .LE. LEN(STR1) .AND. L2 .LE. LEN(STR2) ) THEN
///           ALIKE = STR1(L1:L1) .EQ. STR2(L2:L2)
///        ELSE
///           ALIKE = .FALSE.
///        END IF
///     END DO
///
///  However this is a much more complicated section of code. This
///  routine allows you to code the above loops as:
///
///
///     DO WHILE ( SAMCH ( STR1,L1, STR2,L2 )  )
///        L1 = L1 + 1
///        L2 = L2 + 1
///     END DO
///
///  The boundary checks are automatically performed and out
///  of range errors are avoided.
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
/// -    SPICELIB Version 1.0.1, 12-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 31-MAR-1995 (WLT)
/// ```
pub fn samch(str1: &str, l1: i32, str2: &str, l2: i32) -> bool {
    let ret = SAMCH(str1.as_bytes(), l1, str2.as_bytes(), l2);
    ret
}

//$Procedure SAMCH ( Same character )
pub fn SAMCH(STR1: &[u8], L1: i32, STR2: &[u8], L2: i32) -> bool {
    let mut SAMCH: bool = false;

    if ((((L1 < 1) || (L2 < 1)) || (L1 > intrinsics::LEN(STR1))) || (L2 > intrinsics::LEN(STR2))) {
        SAMCH = false;
        return SAMCH;
    }

    SAMCH = (intrinsics::ICHAR(fstr::substr(STR1, L1..=L1))
        == intrinsics::ICHAR(fstr::substr(STR2, L2..=L2)));
    SAMCH
}
