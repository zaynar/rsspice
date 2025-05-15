//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Julian Date of 2100 JAN 1.5
///
/// Return the Julian Date of 2100 JAN 01 12:00:00 (2100 JAN 1.5).
///
/// # Brief I/O
///
/// ```text
///  The function returns the Julian Date of 2100 JAN 01 12:00:00
///  (2100 JAN 1.5).
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns 2488070.0, the Julian Date corresponding
///  to 2100 JAN 01 12:00:00 (2100 JAN 1.5).
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
///  The following code fragment illustrates the use of J2100.
///
///     C
///     C     Convert Julian Date to UTC seconds past the reference
///     C     epoch (J2100).
///     C
///           SPREF = ( JD - J2100() ) * SPD()
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
pub fn j2100() -> f64 {
    let ret = J2100();
    ret
}

//$Procedure J2100 ( Julian Date of 2100 JAN 1.5 )
pub fn J2100() -> f64 {
    let mut J2100: f64 = 0.0;

    J2100 = 2488070.0;

    J2100
}
