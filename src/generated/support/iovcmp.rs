//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure IOVCMP ( Inverse order vector with compressed range )
pub fn IOVCMP(DARRAY: &[f64], NDIM: i32, IORDER: &mut [i32], INVORD: &mut [i32], RNGMAX: &mut i32) {
    let DARRAY = DummyArray::new(DARRAY, 1..);
    let mut IORDER = DummyArrayMut::new(IORDER, 1..);
    let mut INVORD = DummyArrayMut::new(INVORD, 1..);
    let mut NUPRED: i32 = 0;

    //
    // Local variables
    //

    //
    // First step: create an order vector for DARRAY.
    //
    spicelib::ORDERD(DARRAY.as_slice(), NDIM, IORDER.as_slice_mut());

    //
    // Produce the corresponding inverse order vector.
    //
    for I in 1..=NDIM {
        INVORD[IORDER[I]] = I;
    }

    //
    // Step through the order vector, keeping track of the count of
    // unique predecessors, in the array that would be produced by
    // sorting DARRAY, of each element pointed to by an element of the
    // order vector.
    //
    // The element of DARRAY at index IORDER(1) has no predecessors,
    // and the element INVORD( IORDER(1) ) is already correct. So
    // we start at the second element of IORDER (if it exists).
    //
    // Initialize NUPRED to the number of unique predecessors of
    // the first value.
    //
    NUPRED = 0;

    for I in 2..=NDIM {
        //
        // At this point, NUPRED is the number of unique predecessors of
        // DARRAY(I). I is greater than or equal to 2.
        //
        if (DARRAY[IORDER[I]] > DARRAY[IORDER[(I - 1)]]) {
            //
            // DARRAY( IORDER(I) ) is strictly greater than, and hence not
            // a copy of, its predecessor. It has NUPRED + 1 unique
            // predecessors in the array produced by sorting DARRAY.
            //
            NUPRED = (NUPRED + 1);

            //
            // The position of DARRAY( IORDER(I) ) in the sorted,
            // compressed set derived from DARRAY is one more than the
            // count of unique predecessors.
            //
            INVORD[IORDER[I]] = (NUPRED + 1);
        } else {
            //
            // DARRAY( IORDER(I) ) is a duplicate. Its position in the
            // sorted, compressed array derived from DARRAY is the same
            // as that of DARRAY( IORDER(I-1) ).
            //
            INVORD[IORDER[I]] = INVORD[IORDER[(I - 1)]];
        }
    }

    //
    // INVORD has been updated so that its elements belong to the
    // set { 1 : NUPRED+1 }.
    //
    // Set the maximum range value of INVORD.
    //
    *RNGMAX = (NUPRED + 1);
}
