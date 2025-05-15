//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

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

//$Procedure      ZZEKAPS ( EK, allocate page for segment )
pub fn ZZEKAPS(
    HANDLE: i32,
    SEGDSC: &[i32],
    TYPE: i32,
    NEW: bool,
    P: &mut i32,
    BASE: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let SEGDSC = DummyArray::new(SEGDSC, 1..=SDSCSZ);
    let mut IDX: i32 = 0;
    let mut TREE: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //
    if NEW {
        //
        // We must allocate a new page.
        //
        ZZEKPGAN(HANDLE, TYPE, P, BASE, ctx)?;
    } else {
        //
        // We can allocate a page from the free list if one is available.
        // Otherwise take a new page.
        //
        ZZEKPGAL(HANDLE, TYPE, P, BASE, ctx)?;
    }

    if FAILED(ctx) {
        return Ok(());
    }

    //
    // Zero out the page's link count and forward pointer.
    //
    ZZEKSLNK(HANDLE, TYPE, *P, 0, ctx)?;
    ZZEKSFWD(HANDLE, TYPE, *P, 0, ctx)?;

    //
    // Update the segment's metadata.  For type 1 segments,
    // the new page into the page tree of the appropriate data type.
    //
    if (TYPE == CHR) {
        TREE = SEGDSC[CPTIDX];
    } else if (TYPE == DP) {
        TREE = SEGDSC[DPTIDX];
    } else if (TYPE == INT) {
        //
        // The remaining possibility is that TYPE is INT.  If we had had
        // an unrecognized type, one of the allocation routines would have
        // complained.
        //
        TREE = SEGDSC[IPTIDX];
    }

    ZZEKTRAP(HANDLE, TREE, *P, &mut IDX, ctx)?;

    Ok(())
}
