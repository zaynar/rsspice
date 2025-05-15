//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Seconds per julian year
///
/// Return the number of seconds in a julian year.
///
/// # Brief I/O
///
/// ```text
///  The function returns the number of seconds per julian year.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the number of seconds per julian year.
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
///  The julian year is often used as a fundamental unit of time when
///  dealing with ephemeris data. For this reason its value in terms of
///  ephemeris seconds is recorded in this function.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose you wish to compute the number of julian centuries
///  that have elapsed since the ephemeris epoch J1950 (beginning
///  of the julian year 1950) at a particular ET epoch. The
///  following line of code will do the trick.
///
///
///     CENTRY = ( ET - UNITIM ( J1950(), 'JED', 'ET' ) )
///    .       / ( 100.0D0 * JYEAR() )
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  P. Kenneth Seidelmann (Ed.), "Explanatory Supplement to the
///       Astronomical Almanac," Page 8, University Science Books,
///       1992.
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
/// -    SPICELIB Version 1.0.1, 08-APR-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 13-JUL-1993 (WLT)
/// ```
pub fn jyear() -> f64 {
    let ret = JYEAR();
    ret
}

//$Procedure JYEAR ( Seconds per julian year )
pub fn JYEAR() -> f64 {
    let mut JYEAR: f64 = 0.0;

    JYEAR = 31557600.0;

    JYEAR
}
