//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Julian Date of 1950.0 JAN 1.0
///
/// Return the Julian Date of 1950 JAN 01 00:00:00 (1950 JAN 1.0).
///
/// # Brief I/O
///
/// ```text
///  The function returns the Julian Date of 1950 JAN 01 00:00:00
///  (1950 JAN 1.0).
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns 2433282.5, the Julian Date corresponding
///  to 1950 JAN 01 00:00:00 (1950 JAN 1.0).
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
///  The following code fragment illustrates the use of J1950.
///
///     C
///     C     Convert Julian Date to UTC seconds past the reference
///     C     epoch (J1950).
///     C
///           SPREF = ( JD - J1950() ) * SPD()
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
pub fn j1950() -> f64 {
    let ret = J1950();
    ret
}

//$Procedure J1950 ( Julian Date of 1950.0 JAN 1.0 )
pub fn J1950() -> f64 {
    let mut J1950: f64 = 0.0;

    J1950 = 2433282.5;

    J1950
}
