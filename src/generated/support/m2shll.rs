//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      M2SHLL ( Shell sort an array of Meta/2 syntaxs )
pub fn M2SHLL(NDIM: i32, ARRAY: CharArrayMut) {
    let mut ARRAY = DummyCharArrayMut::new(ARRAY, None, 1..);
    let mut GAP: i32 = 0;
    let mut EJ: i32 = 0;
    let mut EJG: i32 = 0;
    let mut J: i32 = 0;
    let mut K: i32 = 0;
    let mut KG: i32 = 0;
    let mut JG: i32 = 0;
    let mut SWAP: bool = false;

    //
    // SPICELIB Functions
    //

    //
    // Local variables
    //

    //
    // This is a straightforward implementation of the Shell Sort
    // algorithm.
    //
    GAP = (NDIM / 2);

    while (GAP > 0) {
        for I in (GAP + 1)..=NDIM {
            J = (I - GAP);
            while (J > 0) {
                JG = (J + GAP);
                K = J;
                KG = JG;

                EJ = spicelib::POS(&ARRAY[J], b"[", 1);
                EJG = spicelib::POS(&ARRAY[JG], b"[", 1);

                if (EJ > 1) {
                    fstr::assign(fstr::substr_mut(ARRAY.get_mut(J), EJ..=EJ), b" ");
                }

                if (EJG > 1) {
                    fstr::assign(fstr::substr_mut(ARRAY.get_mut(JG), EJG..=EJG), b" ");
                }

                if fstr::le(&ARRAY[J], &ARRAY[JG]) {
                    J = 0;
                    SWAP = false;
                } else {
                    SWAP = true;
                }

                if (EJ > 1) {
                    fstr::assign(fstr::substr_mut(ARRAY.get_mut(K), EJ..=EJ), b"[");
                }

                if (EJG > 1) {
                    fstr::assign(fstr::substr_mut(ARRAY.get_mut(KG), EJG..=EJG), b"[");
                }

                if SWAP {
                    spicelib::SWAPC_ARRAY(
                        ARRAY.subscript(J),
                        ARRAY.subscript(JG),
                        ARRAY.as_arg_mut(),
                    );
                }

                J = (J - GAP);
            }
        }

        GAP = (GAP / 2);
    }
}
