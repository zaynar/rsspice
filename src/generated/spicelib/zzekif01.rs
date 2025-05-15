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
const OLD: i32 = 1;
const UPDATE: i32 = (OLD + 1);
const NEW: i32 = (UPDATE + 1);
const DELOLD: i32 = (NEW + 1);
const DELNEW: i32 = (DELOLD + 1);
const DELUPD: i32 = (DELNEW + 1);
const STAIDX: i32 = 1;
const RCPIDX: i32 = (STAIDX + 1);
const DPTBAS: i32 = 2;
const MXRPSZ: i32 = 254;
const UNINIT: i32 = -1;
const NULL: i32 = (UNINIT - 1);
const NOBACK: i32 = (NULL - 1);
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

//$Procedure ZZEKIF01 ( EK, initialize type 1 segment for fast load )
pub fn ZZEKIF01(
    HANDLE: i32,
    SEGNO: i32,
    RCPTRS: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut RCPTRS = DummyArrayMut::new(RCPTRS, 1..);
    let mut BASE: i32 = 0;
    let mut NCOLS: i32 = 0;
    let mut NPAGE: i32 = 0;
    let mut NR: i32 = 0;
    let mut NROWS: i32 = 0;
    let mut NRP: i32 = 0;
    let mut P: i32 = 0;
    let mut PBASE: i32 = 0;
    let mut RECNO: i32 = 0;
    let mut REMAIN: i32 = 0;
    let mut RPSIZE: i32 = 0;
    let mut SEGDSC = StackArray::<i32, 24>::new(1..=SDSCSZ);
    let mut TOP: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZEKIF01", ctx)?;
    }

    //
    // Read in the segment descriptor.
    //
    ZZEKSDSC(HANDLE, SEGNO, SEGDSC.as_slice_mut(), ctx)?;

    NCOLS = SEGDSC[NCIDX];
    NROWS = SEGDSC[NRIDX];

    //
    // Empty the EK scratch area stack.
    //
    ZZEKSTOP(&mut TOP, ctx);
    ZZEKSDEC(TOP, ctx)?;

    //
    // Push the handle and segment number onto the stack.
    //
    ZZEKSPSH(1, &[HANDLE], ctx)?;
    ZZEKSPSH(1, &[SEGNO], ctx)?;

    //
    // The segment will require a record pointer structure for each row
    // in the segment.  Right now, all we're going to do is allocate
    // integer pages to hold these structures and save the base
    // addresses of each structure.
    //
    // We compute the number of record pointers that can fit on one page.
    // We also compute the number of pages we'll need to hold the
    // pointers.
    //
    RPSIZE = (DPTBAS + NCOLS);
    NRP = (IPSIZE / RPSIZE);
    NPAGE = (((NROWS + NRP) - 1) / NRP);

    //
    // We'll compute addresses of record pointers a pageful at a time.
    //
    REMAIN = NROWS;
    RECNO = 0;

    for I in 1..=NPAGE {
        //
        // Allocate a page to hold the record pointers.  A page from
        // the free list is acceptable, hence the argument .FALSE.
        // passed to ZZEKAPS.
        //
        ZZEKAPS(
            HANDLE,
            SEGDSC.as_slice(),
            INT,
            false,
            &mut P,
            &mut PBASE,
            ctx,
        )?;

        //
        // NR is the number of record pointers we'll eventually write to
        // this page.
        //
        NR = intrinsics::MIN0(&[NRP, REMAIN]);

        for J in 1..=NR {
            //
            // Record the base address of the current record pointer
            // in the record pointer array.
            //
            BASE = ((J - 1) * RPSIZE);
            RCPTRS[(RECNO + J)] = (PBASE + BASE);
        }

        RECNO = (RECNO + NR);
        REMAIN = (REMAIN - NR);
    }

    CHKOUT(b"ZZEKIF01", ctx)?;
    Ok(())
}
