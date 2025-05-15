//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure TOUCHC ( Touch a variable )
pub fn TOUCHC(STRING: &[u8], TOUCHC: &mut [u8]) {
    let TOUCHC = &mut TOUCHC[..1];

    fstr::assign(TOUCHC, fstr::substr(STRING, 1..=1));
}
