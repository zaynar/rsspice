//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Swap character values
///
/// Swap the contents of two character strings.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  A         I-O  First string.
///  B         I-O  Second string.
/// ```
///
/// # Detailed Input
///
/// ```text
///  A,
///  B        are two character strings, the contents of which
///           are to be swapped (exchanged).
/// ```
///
/// # Detailed Output
///
/// ```text
///  A,
///  B        are the same two character strings, after their
///           contents have been exchanged.
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
///  This is just shorthand notation for the code fragment
///
///        TEMP = A
///        A    = B
///        B    = TEMP
///
///  The characters in the string are swapped one at a time, so
///  no intermediate string (TEMP) is needed. This means that the
///  strings may be of any length.
/// ```
///
/// # Examples
///
/// ```text
///  Let
///        A = 11.D0
///        B = 22.D0
///
///  Then after calling SWAPD (A,B),
///
///        A = 22.D0
///        B = 11.D0
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
/// -    SPICELIB Version 1.1.0, 18-MAR-2021 (JDR)
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
pub fn swapc(a: &mut str, b: &mut str) {
    SWAPC(
        fstr::StrBytes::new(a).as_mut(),
        fstr::StrBytes::new(b).as_mut(),
    );
}

//$Procedure SWAPC ( Swap character values )
pub fn SWAPC(A: &mut [u8], B: &mut [u8]) {
    let mut ALEN: i32 = 0;
    let mut BLEN: i32 = 0;
    let mut SHORT: i32 = 0;
    let mut TEMP = [b' '; 1];

    //
    // Local variables
    //

    //
    // Get the lengths of the strings.
    //
    ALEN = intrinsics::LEN(A);
    BLEN = intrinsics::LEN(B);
    SHORT = intrinsics::MIN0(&[ALEN, BLEN]);

    //
    // Keep going until the end of the shorter string is reached.
    //
    for I in 1..=SHORT {
        fstr::assign(&mut TEMP, fstr::substr(A, I..=I));
        fstr::assign(fstr::substr_mut(A, I..=I), fstr::substr(B, I..=I));
        fstr::assign(fstr::substr_mut(B, I..=I), &TEMP);
    }

    //
    // If either string is longer than the shortest one, pad it
    // with blanks.
    //
    if (ALEN > SHORT) {
        fstr::assign(fstr::substr_mut(A, (SHORT + 1)..), b" ");
    } else if (BLEN > SHORT) {
        fstr::assign(fstr::substr_mut(B, (SHORT + 1)..), b" ");
    }
}
