//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Besselian Date 1950.0
///
/// Return the Julian Date corresponding to Besselian Date 1950.0.
///
/// # Brief I/O
///
/// ```text
///  The function returns the Julian Date corresponding to Besselian
///  date 1950.0.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns 2433282.42345905, the Julian Date
///  corresponding to Besselian Date 1950.0.
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
///  The following code fragment illustrates the use of B1950.
///
///     C
///     C     Convert Julian Date to UTC seconds past the reference
///     C     epoch (B1950).
///     C
///           SPREF = ( JD - B1950() ) * SPD()
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
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.0.2, 08-JUL-2020 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.0.1, 18-AUG-2008 (EDW)
///
///         Edited the value stated in $Detailed_Output to match the
///         current return value. The edit changed:
///
///            2433282.423
///
///         to
///
///            2433282.42345905
///
/// -    SPICELIB Version 2.0.0, 30-JUL-1993 (WLT)
///
///         The value of B1950 was updated to reflect the value given
///         by Lieske in [1]
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (IMU)
/// ```
pub fn b1950() -> f64 {
    let ret = B1950();
    ret
}

//$Procedure B1950 ( Besselian Date 1950.0 )
pub fn B1950() -> f64 {
    let mut B1950: f64 = 0.0;

    B1950 = 2433282.42345905;
    B1950
}
