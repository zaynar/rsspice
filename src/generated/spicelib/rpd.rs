//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

struct SaveVars {
    VALUE: f64,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut VALUE: f64 = 0.0;

        VALUE = 0.0;

        Self { VALUE }
    }
}

/// Radians per degree
///
/// Return the number of radians per degree.
///
/// # Brief I/O
///
/// ```text
///  The function returns the number of radians per degree.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the number of radians per degree: pi/180.
///  The value of pi is determined by the ACOS function. That is,
///
///        RPD = ACOS ( -1.D0 ) / 180.D0
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
///  The first time the function is referenced, the value is computed
///  as shown above. The value is saved, and returned directly upon
///  subsequent reference.
/// ```
///
/// # Examples
///
/// ```text
///  The code fragment below illustrates the use of RPD.
///
///     C
///     C     Convert all input angles to radians.
///     C
///           CLOCK = CLOCK * RPD()
///           CONE  = CONE  * RPD()
///           TWIST = TWIST * RPD()
///
///  or equivalently,
///
///     C
///     C     Convert all input angles to radians.
///     C
///           CALL VPACK  ( CLOCK, CONE, CCTWIST, ALBTGAM )
///           CALL VSCL   ( RPD(), ALBTGAM, ALBTGAM )
///           CALL VUPACK ( ALBTGAM, CLOCK, CONE, CCTWIST )
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
pub fn rpd(ctx: &mut SpiceContext) -> f64 {
    let ret = RPD(ctx.raw_context());
    ret
}

//$Procedure RPD ( Radians per degree )
pub fn RPD(ctx: &mut Context) -> f64 {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut RPD: f64 = 0.0;

    //
    // Local variables
    //

    //
    // Initial values
    //

    //
    // What is there to say?
    //
    if (save.VALUE == 0.0) {
        save.VALUE = (f64::acos(-1.0) / 180.0);
    }

    RPD = save.VALUE;

    RPD
}
