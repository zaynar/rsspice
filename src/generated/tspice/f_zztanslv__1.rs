//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//
// State function for use by F_ZZTANSLV.
//
// Let LOWER be defined as
//
//    LOWER = floor( X ).
//
// The function returns .TRUE. if
//
//    LOWER
//
// is odd and
//
//    0  <  LOWER  <  X  <  LOWER+1
//       -         -     -
//
// For all other X, the function returns .FALSE.
//
// When the function returns .TRUE., the returned
// value of POINT is
//
//    ( LOWER, LOWER+1, LOWER+2 )
//
// When the function returns .FALSE., the returned
// value of POINT is
//
//    ( 0, 0, 0 )
//
//
pub fn T_TANSTA(
    X: f64,
    STATE: &mut bool,
    POINT: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut POINT = DummyArrayMut::new(POINT, 1..=3);
    let mut DVAL: f64 = 0.0;
    let mut FLOOR: f64 = 0.0;
    let mut IFLOOR: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    if (X < 1.0) {
        *STATE = false;
        spicelib::CLEARD(3, POINT.as_slice_mut());
    } else {
        FLOOR = f64::trunc(X);
        IFLOOR = (FLOOR as i32);

        if spicelib::ODD(IFLOOR) {
            *STATE = true;
            DVAL = FLOOR;
        } else if (X == FLOOR) {
            //
            // IFLOOR is even.
            //
            *STATE = true;
            DVAL = (FLOOR - 1.0);
        } else {
            *STATE = false;
        }
    }

    if *STATE {
        spicelib::VPACK(DVAL, (DVAL + 1.0), (DVAL + 2.0), POINT.as_slice_mut());
    } else {
        spicelib::CLEARD(3, POINT.as_slice_mut());
    }

    Ok(())
}
