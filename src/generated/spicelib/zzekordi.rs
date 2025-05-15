//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

fn CMP1(I1: i32, I2: i32, I3: i32, I4: i32) -> bool {
    ((I1 < I2) || ((I1 == I2) && (I3 < I4)))
}

fn CMP2(I3: i32, I4: i32, N1: bool, N2: bool) -> bool {
    ((N1 && !N2) || ((N1 && N2) && (I3 < I4)))
}

fn CMP3(I1: i32, I2: i32, I3: i32, I4: i32, N1: bool, N2: bool) -> bool {
    (!(N1 || N2) && CMP1(I1, I2, I3, I4))
}

fn CMP4(I1: i32, I2: i32, I3: i32, I4: i32, N1: bool, N2: bool) -> bool {
    (CMP2(I3, I4, N1, N2) || CMP3(I1, I2, I3, I4, N1, N2))
}

fn LE(I1: i32, I2: i32, I3: i32, I4: i32, OK: bool, N1: bool, N2: bool) -> bool {
    ((!OK && CMP1(I1, I2, I3, I4)) || (OK && CMP4(I1, I2, I3, I4, N1, N2)))
}

//$Procedure      ZZEKORDI ( Order of an integer EK column )
pub fn ZZEKORDI(IVALS: &[i32], NULLOK: bool, NLFLGS: &[bool], NVALS: i32, IORDER: &mut [i32]) {
    let IVALS = DummyArray::new(IVALS, 1..);
    let NLFLGS = DummyArray::new(NLFLGS, 1..);
    let mut IORDER = DummyArrayMut::new(IORDER, 1..);
    let mut GAP: i32 = 0;
    let mut J: i32 = 0;
    let mut JG: i32 = 0;

    //
    // Local variables
    //

    //
    // Statement functions
    //

    //
    // Begin with the initial ordering.
    //
    for I in 1..=NVALS {
        IORDER[I] = I;
    }

    //
    // Find the smallest element, then the next smallest, and so on.
    // This uses the Shell Sort algorithm, but swaps the elements of
    // the order vector instead of the array itself.
    //
    GAP = (NVALS / 2);

    while (GAP > 0) {
        for I in (GAP + 1)..=NVALS {
            J = (I - GAP);

            while (J > 0) {
                JG = (J + GAP);

                if LE(
                    IVALS[IORDER[J]],
                    IVALS[IORDER[JG]],
                    IORDER[J],
                    IORDER[JG],
                    NULLOK,
                    NLFLGS[IORDER[J]],
                    NLFLGS[IORDER[JG]],
                ) {
                    //
                    // Getting here means that
                    //
                    //    IVALS(IORDER(J)) .LE. IVALS(IORDER(JG))
                    //
                    // according to our order relation.
                    //
                    J = 0;
                } else {
                    SWAPI_ARRAY(
                        IORDER.subscript(J),
                        IORDER.subscript(JG),
                        IORDER.as_slice_mut(),
                    );
                }

                J = (J - GAP);
            }
        }

        GAP = (GAP / 2);
    }
}
