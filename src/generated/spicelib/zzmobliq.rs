//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const C0: f64 = 84381.448;
const C1: f64 = -46.815;
const C2: f64 = -0.00059;
const C3: f64 = 0.001813;

struct SaveVars {
    T: f64,
    RAD: f64,
    YEAR: f64,
    PERSEC: f64,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut T: f64 = 0.0;
        let mut RAD: f64 = 0.0;
        let mut YEAR: f64 = 0.0;
        let mut PERSEC: f64 = 0.0;
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            T,
            RAD,
            YEAR,
            PERSEC,
            FIRST,
        }
    }
}

//$Procedure   ZZMOBLIQ   ( Mean obliquity of date )
pub fn ZZMOBLIQ(ET: f64, MOB: &mut f64, DMOB: &mut f64, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    //
    // Coefficients for the mean obliquity:
    //

    //
    // Local variables
    //

    if save.FIRST {
        save.FIRST = false;
        save.YEAR = JYEAR();
        save.RAD = RPD(ctx);
        save.PERSEC = (1.0 / (save.YEAR * 100.0));
    }

    //
    // Convert the input epoch to Julian centuries past J2000:
    //
    save.T = ((ET / save.YEAR) / 100.0);

    //
    // Compute the obliquity at epoch.  The polynomial yields arcseconds;
    // convert the units to radians.
    //
    *MOB = ((save.RAD / 3600.0) * (C0 + (save.T * (C1 + (save.T * (C2 + (save.T * C3)))))));

    *DMOB = (((save.RAD / 3600.0)
        * (C1 + (save.T * (((2 as f64) * C2) + ((save.T * 3 as f64) * C3)))))
        * save.PERSEC);
}
