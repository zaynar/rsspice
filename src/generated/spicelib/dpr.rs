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

/// Degrees per radian
///
/// Return the number of degrees per radian.
///
/// # Brief I/O
///
/// ```text
///  The function returns the number of degrees per radian.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the number of degrees per radian: 180/pi.
///  The value of pi is determined by the ACOS function. That is,
///
///        DPR = 180.D0 / ACOS ( -1.D0 )
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
///  The code fragment below illustrates the use of DPR.
///
///     C
///     C     Convert all angles to degrees for output.
///     C
///           CLOCK = CLOCK * DPR()
///           CONE  = CONE  * DPR()
///           TWIST = TWIST * DPR()
///
///  or equivalently,
///
///     C
///     C     Convert all input angles to radians.
///     C
///           CALL VPACK  ( CLOCK, CONE, CCTWIST, ALBTGAM )
///           CALL VSCL   ( DPR(), ALBTGAM, ALBTGAM )
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
pub fn dpr(ctx: &mut SpiceContext) -> f64 {
    let ret = DPR(ctx.raw_context());
    ret
}

//$Procedure DPR ( Degrees per radian )
pub fn DPR(ctx: &mut Context) -> f64 {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut DPR: f64 = 0.0;

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
        save.VALUE = (180.0 / f64::acos(-1.0));
    }

    DPR = save.VALUE;

    DPR
}
