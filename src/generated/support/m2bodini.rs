//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      M2BODINI ()
pub fn M2BODINI(
    NAMES: CharArray,
    NNAM: i32,
    CODES: &[i32],
    NCOD: &mut i32,
    ORDNAM: &mut [i32],
    ORDCOD: &mut [i32],
) {
    let NAMES = DummyCharArray::new(NAMES, None, 1..);
    let CODES = DummyArray::new(CODES, 1..);
    let mut ORDNAM = DummyArrayMut::new(ORDNAM, 1..);
    let mut ORDCOD = DummyArrayMut::new(ORDCOD, 1..);
    let mut I: i32 = 0;
    let mut N: i32 = 0;

    //
    // Local variables
    //

    //
    // Create order vectors ORDNAM and ORDCOD
    //
    spicelib::ORDERC(NAMES.as_arg(), NNAM, ORDNAM.as_slice_mut());
    spicelib::ORDERI(CODES.as_slice(), NNAM, ORDCOD.as_slice_mut());

    //
    // Remove duplicate entries in the code order table. The entry that
    // points to the highest entry in CODES should remain.
    //
    N = 1;
    I = 2;

    //
    // Now for some very funky manuevering.  We are going to take our
    // order vector for the id-codes and modify it!
    //
    // Here's what is true now.
    //
    // CODES(ORDCOD(1)) <= CODES(ORDCOD(2)) <=...<= CODES(ORDCOD(NNAM)
    //
    // For each element such that CODES(ORDCOD(I)) = CODES(ORDCOD(I+1))
    // we are going to "shift" the items ORDCOD(I+1), ORDCOD(I+2), ...
    // left by one.  We will then repeat the test and shift as needed.
    // When we get done we will have a possibly shorter array ORDCOD
    // and the array will satisfy
    //
    //    CODES(ORDCOD(1)) < CODES(ORDCOD(2)) < ... < CODES(ORDCOD(NNAM)
    //
    // We can still use the resulting "ordered vector" (as opposed to
    // order vector) in the BSCHOI routine because it only relies
    // upon the indexes to ORDCOD and not to CODES itself.  This is
    // making very heavy use of the implementation of BSCHOI but we
    // are going to let it go for the momemt because this is a private
    // routine.
    //
    while (I <= NNAM) {
        if (CODES[ORDCOD[I]] == CODES[ORDCOD[N]]) {
            if (ORDCOD[I] > ORDCOD[N]) {
                ORDCOD[N] = ORDCOD[I];
            }
        } else {
            N = (N + 1);
            ORDCOD[N] = ORDCOD[I];
        }

        I = (I + 1);
    }

    *NCOD = N;
}
