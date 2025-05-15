//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const BND: f64 = 0.5;

//$Procedure      T_RANDSIGN ( Random sign )
pub fn T_RANDSIGN(SEED: &mut i32, ctx: &mut Context) -> i32 {
    let mut T_RANDSIGN: i32 = 0;

    //
    // Other functions
    //

    //
    // Local parameters
    //

    //
    // Call T_URAND to retrieve a random number between 0 and 1.
    // If the random number is greater than or equal to BND, return 1.
    // Otherwise, return -1.
    //
    if (T_URAND(SEED, ctx) >= BND) {
        T_RANDSIGN = 1;
    } else {
        T_RANDSIGN = -1;
    }

    T_RANDSIGN
}
