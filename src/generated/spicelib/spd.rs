//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Seconds per day
///
/// Return the number of seconds in a day.
///
/// # Brief I/O
///
/// ```text
///  The function returns the number of seconds in a day.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the number of seconds in a day: 86400.
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
///  The following code fragment illustrates the use of SPD.
///
///     C
///     C     Convert Julian Date to UTC seconds past the reference
///     C     epoch (J2000).
///     C
///           SPREF = ( JD - J2000() ) * SPD()
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
/// -    SPICELIB Version 1.0.2, 07-JUL-2020 (JDR)
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
pub fn spd() -> f64 {
    let ret = SPD();
    ret
}

//$Procedure SPD ( Seconds per day )
pub fn SPD() -> f64 {
    let mut SPD: f64 = 0.0;

    //
    // Just like it says.
    //
    SPD = 86400.0;

    SPD
}
