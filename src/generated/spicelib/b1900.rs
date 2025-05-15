//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Besselian Date 1900.0
///
/// Return the Julian Date corresponding to Besselian Date 1900.0.
///
/// # Brief I/O
///
/// ```text
///  The function returns the Julian Date corresponding to Besselian
///  date 1900.0.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns 2415020.31352, the Julian Date corresponding
///  to Besselian Date 1900.0.
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
///  The following code fragment illustrates the use of B1900.
///
///     C
///     C     Convert Julian Date to UTC seconds past the reference
///     C     epoch (B1900).
///     C
///           SPREF = ( JD - B1900() ) * SPD()
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  J. Lieske, "Precession Matrix Based on IAU (1976) System of
///       Astronomical Constants," Astron. Astrophys. 73, 282-284,
///       1979.
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
/// -    SPICELIB Version 1.0.2, 13-APR-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Added entry [1]
///         to $Literature_References section.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (IMU)
/// ```
pub fn b1900() -> f64 {
    let ret = B1900();
    ret
}

//$Procedure B1900 ( Besselian Date 1900.0 )
pub fn B1900() -> f64 {
    let mut B1900: f64 = 0.0;

    B1900 = 2415020.31352;

    B1900
}
