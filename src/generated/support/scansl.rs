//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      SCANSL ( Scan --- select tokens )
pub fn SCANSL(
    IDS: &[i32],
    N: i32,
    NTOKNS: &mut i32,
    IDENT: &mut [i32],
    BEG: &mut [i32],
    END: &mut [i32],
) {
    let IDS = DummyArray::new(IDS, 1..);
    let mut IDENT = DummyArrayMut::new(IDENT, 1..);
    let mut BEG = DummyArrayMut::new(BEG, 1..);
    let mut END = DummyArrayMut::new(END, 1..);
    let mut J: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // There's not much to do, shift forward the token attributes for
    // tokens whose identities belong to the selection list.
    //
    J = 0;

    for I in 1..=*NTOKNS {
        if (spicelib::ISRCHI(IDENT[I], N, IDS.as_slice()) > 0) {
            J = (J + 1);
            IDENT[J] = IDENT[I];
            BEG[J] = BEG[I];
            END[J] = END[I];
        }
    }

    *NTOKNS = J;
}
