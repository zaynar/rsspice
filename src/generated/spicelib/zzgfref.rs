//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ZZGET: i32 = -1;
const ZZPUT: i32 = -2;
const ZZRESET: i32 = -3;
const ZZNOP: i32 = 3;
const GEN: i32 = 1;
const GF_REF: i32 = 2;
const GF_TOL: i32 = 3;
const GF_DT: i32 = 4;
const NID: i32 = 4;

//$Procedure ZZGFREF ( Private --- GF, update REFVAL )
pub fn ZZGFREF(REFVAL: f64, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut OK: bool = false;

    //
    // Local Variables
    //

    //
    // Store the REFVAL value for use in ZZGFUDLT.
    //
    ZZHOLDD_ZZPUT(ZZPUT, GF_REF, &mut OK, REFVAL, ctx)?;

    Ok(())
}
