//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure ZZRBRKST ( Private --- Reverse Bracketed String Extractor )
pub fn ZZRBRKST(
    STRING: &[u8],
    LFTEND: &[u8],
    RGTEND: &[u8],
    SUBSTR: &mut [u8],
    LENGTH: &mut i32,
    BKPRES: &mut bool,
) {
    let mut BSIZE: i32 = 0;
    let mut LINDEX: i32 = 0;
    let mut LSIZE: i32 = 0;
    let mut RINDEX: i32 = 0;
    let mut RSIZE: i32 = 0;

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Compute the sizes of the bracketing substrings and the text
    // block.
    //
    LSIZE = intrinsics::LEN(LFTEND);
    RSIZE = intrinsics::LEN(RGTEND);
    BSIZE = intrinsics::LEN(STRING);

    //
    // Search from the right for RGTEND.
    //
    RINDEX = POSR(STRING, RGTEND, BSIZE);

    //
    // Now continue the search from RINDEX to the right, this time
    // looking for LFTEND. If RINDEX comes back as 0, then the right
    // bracketing substring is not present, so search the entire string
    // for LFTEND. Otherwise, search from where the right bracket
    // search left off.
    //
    if (RINDEX == 0) {
        LINDEX = POSR(STRING, LFTEND, BSIZE);
    } else {
        LINDEX = POSR(STRING, LFTEND, (RINDEX - LSIZE));
    }

    //
    // Interpret the results.  If RINDEX and LINDEX are both non-zero,
    // then return the substring they bracket, otherwise handle the
    // failed case.
    //
    if ((RINDEX != 0) && (LINDEX != 0)) {
        //
        // Check to see whether or not the brackets are adjacent, and
        // thus have no characters between them.
        //
        if ((LINDEX + LSIZE) > (RINDEX - 1)) {
            *BKPRES = true;
            *LENGTH = 0;

        //
        // If they aren't adjacent, then compute the length and prepare
        // SUBSTR.
        //
        } else {
            *LENGTH = (RINDEX - (LINDEX + LSIZE));
            *BKPRES = true;
            fstr::assign(
                SUBSTR,
                fstr::substr(STRING, (LINDEX + LSIZE)..=(RINDEX - 1)),
            );
        }
    } else {
        //
        // Set BKPRES to TRUE only if LINDEX or RINDEX is non-zero,
        // indicating one was found by POSR. Set LENGTH to 0, since we
        // will not be changing SUBSTR.
        //
        *BKPRES = ((LINDEX + RINDEX) > 0);
        *LENGTH = 0;
    }
}
