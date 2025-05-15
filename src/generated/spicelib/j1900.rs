//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Julian Date of 1900.0 JAN 0.5
///
/// Return the Julian Date of 1899 DEC 31 12:00:00 (1900 JAN 0.5).
///
/// # Brief I/O
///
/// ```text
///  The function returns the Julian Date of 1899 DEC 31 12:00:00
///  (1900 JAN 0.5).
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns 2415020.0, the Julian Date corresponding
///  to 1899 DEC 31 12:00:00 (1900 JAN 0.5).
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
///  The function always returns the constant value shown above.
/// ```
///
/// # Examples
///
/// ```text
///  The following code fragment illustrates the use of J1900.
///
///     C
///     C     Convert Julian Date to UTC seconds past the reference
///     C     epoch (J1900).
///     C
///           SPREF = ( JD - J1900() ) * SPD()
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
/// -    SPICELIB Version 1.0.2, 08-JUL-2020 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (IMU)
/// ```
pub fn j1900() -> f64 {
    let ret = J1900();
    ret
}

//$Procedure J1900 ( Julian Date of 1900.0 JAN 0.5 )
pub fn J1900() -> f64 {
    let mut J1900: f64 = 0.0;

    J1900 = 2415020.0;

    J1900
}
