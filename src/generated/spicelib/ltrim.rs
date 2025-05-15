//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Left trim
///
/// Return the maximum of 1 and the location of the first non-blank
/// character in the string.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///
///  STRING     I   String to be trimmed.
///
///  The function returns the maximum of 1 and the location of the
///  first non-blank character in STRING.
/// ```
///
/// # Detailed Input
///
/// ```text
///  STRING   is a string to be trimmed: the location of the
///           first non-blank character is desired.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the maximum of 1 and the location of the
///  first non-blank character in STRING.
///
///  In particular, when STRING is blank, the function returns the
///  value 1.
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
///  When writing a character string to a file, we may wish to omit
///  leading blanks. We'd like to use FRSTNB as a lower substring
///  bound, but we have to handle the case where FRSTNB returns 0,
///  so we write:
///
///
///     WRITE ( UNIT, '(A)' ),  STRING ( MAX (1, FRSTNB (STRING)) : )
///
///
///  This can be simplified using LTRIM:
///
///
///     WRITE ( UNIT, '(A)' ),  STRING ( LTRIM (STRING) : )
///
///
///  This routine has a counterpart, RTRIM, which finds the maximum of
///  1 and the position of the last non-blank character of a string.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Write the non-blank portion of each element of a character
///      cell to file SPUD.DAT:
///
///         DO I = 1,  CARDC (CELL)
///
///            CALL WRLINE ( 'SPUD.DAT',
///        .                  CELL(I) ( LTRIM (CELL) : RTRIM (CELL) ) )
///
///         END DO
///
///      When CELL(I) is blank, the string ' ' will be written.
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
/// -    SPICELIB Version 1.0.0, 02-MAY-1990 (NJB)
/// ```
pub fn ltrim(string: &str) -> i32 {
    let ret = LTRIM(string.as_bytes());
    ret
}

//$Procedure LTRIM ( Left trim )
pub fn LTRIM(STRING: &[u8]) -> i32 {
    let mut LTRIM: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // `Just do it'.
    //
    LTRIM = intrinsics::MAX0(&[1, FRSTNB(STRING)]);

    LTRIM
}
