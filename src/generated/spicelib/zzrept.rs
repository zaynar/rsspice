//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      ZZREPT ( Private --- replace tokens )
pub fn ZZREPT(SUB: &[u8], REPLAC: &[u8], L2R: bool, ctx: &mut Context) -> bool {
    let mut ZZREPT: bool = false;
    let mut OK: bool = false;

    ZZREPT = ZZSUBT(SUB, REPLAC, L2R, ctx);
    OK = ZZREMT(b"*", ctx);

    ZZREPT
}
