//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Seconds per tropical year
///
/// Return the number of seconds in a tropical year.
///
/// # Brief I/O
///
/// ```text
///  The function returns the number of seconds per tropical year.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the number of seconds per tropical
///  year. This value is taken from the 1992 Explanatory Supplement
///  to the Astronomical Almanac (see [1]).
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
///  The tropical year is often used as a fundamental unit
///  of time when dealing with older ephemeris data. For this
///  reason its value in terms of ephemeris seconds is
///  recorded in this function.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose you wish to compute the number of tropical centuries
///  that have elapsed since the ephemeris epoch B1950 (beginning
///  of the Besselian year 1950) at a particular ET epoch. The
///  following line of code will do the trick.
///
///
///     CENTRY = ( ET - UNITIM ( B1950(), 'JED', 'ET' ) )
///    .       / ( 100.0D0 * TYEAR() )
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  P. Kenneth Seidelmann (Ed.), "Explanatory Supplement to the
///       Astronomical Almanac," p 80, University Science Books, 1992.
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
/// -    SPICELIB Version 1.0.1, 13-APR-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 13-JUL-1993 (WLT)
/// ```
pub fn tyear() -> f64 {
    let ret = TYEAR();
    ret
}

//$Procedure TYEAR ( Seconds per tropical year )
pub fn TYEAR() -> f64 {
    let mut TYEAR: f64 = 0.0;

    TYEAR = 31556925.9747;
    TYEAR
}
