//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ENCSIZ: i32 = 5;
const CPSIZE: i32 = 1014;
const CFPIDX: i32 = (CPSIZE + 1);
const CLCIDX: i32 = (CFPIDX + ENCSIZ);
const DPSIZE: i32 = 126;
const DFPIDX: i32 = (DPSIZE + 1);
const DLCIDX: i32 = (DFPIDX + 1);
const IPSIZE: i32 = 254;
const IFPIDX: i32 = (IPSIZE + 1);
const ILCIDX: i32 = (IFPIDX + 1);

//$Procedure  ZZEKSEI ( Private: EK, set encoded integer )
pub fn ZZEKSEI(HANDLE: i32, ADDRSS: i32, IVAL: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut CVAL = [b' '; ENCSIZ as usize];

    //
    // Local variables
    //

    //
    // Discovery error handling should be used in this utility.
    //
    //
    // Encode the number.
    //
    PRTENC(IVAL, &mut CVAL, ctx)?;

    //
    // Write the encoded value.
    //
    DASUDC(
        HANDLE,
        ADDRSS,
        ((ADDRSS + ENCSIZ) - 1),
        1,
        ENCSIZ,
        CharArray::from_ref(&CVAL),
        ctx,
    )?;

    Ok(())
}
