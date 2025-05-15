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
const SDSCSZ: i32 = 24;
const EKTIDX: i32 = 1;
const SNOIDX: i32 = (EKTIDX + 1);
const IMDIDX: i32 = (SNOIDX + 1);
const TNMIDX: i32 = (IMDIDX + 1);
const NCIDX: i32 = (TNMIDX + 1);
const NRIDX: i32 = (NCIDX + 1);
const RTIDX: i32 = (NRIDX + 1);
const CPTIDX: i32 = (RTIDX + 1);
const DPTIDX: i32 = (CPTIDX + 1);
const IPTIDX: i32 = (DPTIDX + 1);
const MFLIDX: i32 = (IPTIDX + 1);
const IFLIDX: i32 = (MFLIDX + 1);
const SHDIDX: i32 = (IFLIDX + 1);
const CFHIDX: i32 = (SHDIDX + 1);
const CSNIDX: i32 = (CFHIDX + 1);
const LCPIDX: i32 = (CSNIDX + 1);
const LDPIDX: i32 = (LCPIDX + 1);
const LIPIDX: i32 = (LDPIDX + 1);
const LCWIDX: i32 = (LIPIDX + 1);
const LDWIDX: i32 = (LCWIDX + 1);
const LIWIDX: i32 = (LDWIDX + 1);
const NMLIDX: i32 = (LIWIDX + 1);
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;

//$Procedure      ZZEKSLNK ( EK, set link count for data page )
pub fn ZZEKSLNK(
    HANDLE: i32,
    TYPE: i32,
    P: i32,
    NLINKS: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut DPLNK: f64 = 0.0;
    let mut BASE: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //
    // Look up the base address of the page.
    //
    ZZEKPGBS(TYPE, P, &mut BASE, ctx)?;

    if FAILED(ctx) {
        return Ok(());
    }

    if (TYPE == CHR) {
        //
        // Set the encoded count.
        //
        ZZEKSEI(HANDLE, (BASE + CLCIDX), NLINKS, ctx)?;
    } else if (TYPE == DP) {
        //
        // Convert the input count to d.p. type.
        //
        DPLNK = NLINKS as f64;

        DASUDD(HANDLE, (BASE + DLCIDX), (BASE + DLCIDX), &[DPLNK], ctx)?;
    } else {
        //
        // The remaining possibility is that TYPE is INT.  If we had had
        // an unrecognized type, ZZEKPGBS would have complained.
        //
        DASUDI(HANDLE, (BASE + ILCIDX), (BASE + ILCIDX), &[NLINKS], ctx)?;
    }

    Ok(())
}
