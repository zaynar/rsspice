//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure  ZZTKNERR ( Create ZZTOKNS overflow error message )
pub fn ZZTKNERR(TEMPLT: &[u8], STRING: &[u8], TOKEN: &[u8], ERROR: &mut [u8], STATUS: &mut bool) {
    REPMC(TEMPLT, b"#", STRING, ERROR);
    REPMC(&ERROR.to_vec(), b"#", TOKEN, ERROR);
    *STATUS = false;
}
