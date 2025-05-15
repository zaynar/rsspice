//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;

//$Procedure      ZZEKTRBS ( EK tree, base address )
pub fn ZZEKTRBS(NODE: i32, ctx: &mut Context) -> f2rust_std::Result<i32> {
    let mut ZZEKTRBS: i32 = 0;

    //
    // Just use the mapping supplied by the paging system.
    //
    ZZEKPGBS(INT, NODE, &mut ZZEKTRBS, ctx)?;

    Ok(ZZEKTRBS)
}
