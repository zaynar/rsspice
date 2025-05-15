//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//---    The user defined functions required by GFUDB.
//
//      gfb    for udfunb
//

//$Procedure GFB ( Time dependent boolean function )
pub fn GFB(
    UDFUNS: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    T: &mut f64,
    XBOOL: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // Return false unless otherwise assigned.
    //
    *XBOOL = false;

    //
    // An arbitrary boolean function with known boundaries
    // at step event.
    //
    //       -----           -           ----
    //      |     |         | |         |    |
    // -----|     |--- ~ ---| |--- ~ ---|    |----
    //     20     30       55 57       92   100
    //

    if ((*T > 20.0) && (*T < 30.0)) {
        *XBOOL = true;
    } else if ((*T > 55.0) && (*T < 57.0)) {
        *XBOOL = true;
    } else if ((*T > 92.0) && (*T < 100.0)) {
        *XBOOL = true;
    }

    Ok(())
}
