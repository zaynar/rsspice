//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Same substrings
///
/// Determine whether or not two substrings are the same
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  -------------------------------------------------
///  STR1       I   A string
///  B1         I   Beginning of a substring in STR1
///  E1         I   End of s substring in STR1
///  STR2       I   A second string
///  B2         I   The beginning of a substring in STR2
///  E2         I   The end  of s substring in STR2
///
///  The function returns .TRUE. if the substrings are identical
/// ```
///
/// # Detailed Input
///
/// ```text
///  STR1     is a character string
///
///  B1       are integers giving the beginning and ending of a
///  E1       substring in STR1
///
///  STR2     is a character string
///
///  B2       are integers giving the beginning and ending of a
///  E2       substring in STR2
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns .TRUE. if the two substrings STR(B1:E1) and
///  STR(B2:E2) have the same length and the same characters.
///
///  If any of the indices B1, E1, B2, E2 are out of range or out
///  of order the function returns .FALSE.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If any of the B1, E1, B2, E2 are out of range or if an
///      ending substring index is before a beginning substring
///      index, the function returns false.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is a macro for comparing two substrings of
///  strings and handles all of the bounds checking to avoid
///  out of range errors with string indices.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose a string contains a number of occurrences of some
///  particular substring in sequence and that you need to locate
///  the first character that is out of this sequence or the
///  end of the string.
///
///  If one ignores boundary constraints this can easily be
///  coded as shown here: We assume the particular substring is
///
///  '/beg'
///
///     B = 1
///     E = B + LEN('/beg' )
///
///     DO WHILE (       E           .LE. LEN(STR)
///                .AND. STRING(B:E) .EQ. '/beg' )
///
///        B = B + LEN('/beg')
///        E = E + LEN('/beg')
///
///     END DO
///
///     IF ( B .LT. LEN(STR) ) THEN
///
///        we've found the start of a substring of interest
///
///     ELSE
///
///        there is no substring to find.
///
///     END IF
///
///  Unfortunately, you can't rely upon FORTRAN to check the boundary
///  condition: E .LE. LEN(STR) and skip the second test if the first
///  condition if false. As a result you can get an out of range
///  error.
///
///  Instead you could code:
///
///  B = 1
///  E = B + LEN('/beg')
///
///  IF ( E .LE. LEN(STR) ) THEN
///     ALIKE = STRINB(B:E) .EQ. '/beg'
///  ELSE
///     ALIKE = .FALSE.
///  END IF
///
///  DO WHILE ( ALIKE )
///
///        B = B + LEN('/beg')
///        E = E + LEN('/beg')
///
///     IF ( E .LE. LEN(STR) ) THEN
///        ALIKE = STRINB(B:E) .EQ. '/beg'
///     ELSE
///        ALIKE = .FALSE.
///     END IF
///
///  END DO
///
///
///  However, this is code is far more effort. Using this routine
///  you can make a much simpler block of code.
///
///  B = 1
///  E = B + LEN('/beg' )
///
///  DO WHILE ( SAMSUB(STR,B,E, '/beg',1,4 ) )
///
///     B = B + LEN('/beg')
///     E = E + LEN('/beg')
///
///  END DO
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
pub fn samsub(str1: &str, b1: i32, e1: i32, str2: &str, b2: i32, e2: i32) -> bool {
    let ret = SAMSUB(str1.as_bytes(), b1, e1, str2.as_bytes(), b2, e2);
    ret
}

//$Procedure SAMSUB (Same substrings)
pub fn SAMSUB(STR1: &[u8], B1: i32, E1: i32, STR2: &[u8], B2: i32, E2: i32) -> bool {
    let mut SAMSUB: bool = false;

    if (((((((E1 < B1) || (E2 < B2)) || (B1 < 1)) || (B2 < 1)) || (E1 > intrinsics::LEN(STR1)))
        || (E2 > intrinsics::LEN(STR2)))
        || ((E1 - B1) != (E2 - B2)))
    {
        SAMSUB = false;
        return SAMSUB;
    }

    SAMSUB = fstr::eq(fstr::substr(STR1, B1..=E1), fstr::substr(STR2, B2..=E2));

    SAMSUB
}
