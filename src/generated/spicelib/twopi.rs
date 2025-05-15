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

/// Twice the value of pi
///
/// Return twice the value of pi (the ratio of the circumference of
/// a circle to its diameter).
///
/// # Brief I/O
///
/// ```text
///  The function returns twice the value of pi.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns twice the value of pi (the ratio of
///  a circle's circumference to its diameter), determined by
///  the ACOS function. That is,
///
///        TWOPI = ACOS ( -1.D0 ) * 2.D0
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
///  The code fragment below illustrates the use of TWOPI.
///
///     C
///     C      The longitude of the ascending node is the angle
///     C      between the x-axis and the node vector, n.
///     C                                              -
///     C
///            NODE = ACOS ( N(1) / VNORM(N) )
///
///            IF ( NODE .LT. 0.D0 ) THEN
///               NODE = NODE + TWOPI()
///            END IF
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
/// -    SPICELIB Version 1.0.3, 07-JUL-2020 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.2, 28-AUG-1997 (WLT)
///
///         Fixed the description in the detailed output section
///         of the header.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (IMU)
/// ```
pub fn twopi(ctx: &mut SpiceContext) -> f64 {
    let ret = TWOPI(ctx.raw_context());
    ret
}

//$Procedure TWOPI ( Twice the value of pi )
pub fn TWOPI(ctx: &mut Context) -> f64 {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TWOPI: f64 = 0.0;

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
        save.VALUE = (f64::acos(-1.0) * 2.0);
    }

    TWOPI = save.VALUE;

    TWOPI
}
