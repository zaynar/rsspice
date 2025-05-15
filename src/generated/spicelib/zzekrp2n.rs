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

//$Procedure      ZZEKRP2N ( EK, record pointer to number )
pub fn ZZEKRP2N(
    HANDLE: i32,
    SEGNO: i32,
    RECPTR: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<i32> {
    let mut ZZEKRP2N: i32 = 0;
    let mut SEGDSC = StackArray::<i32, 24>::new(1..=SDSCSZ);
    let mut STYPE: i32 = 0;

    //
    // Non-SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //
    ZZEKRP2N = 0;

    ZZEKSDSC(HANDLE, SEGNO, SEGDSC.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        return Ok(ZZEKRP2N);
    }

    STYPE = SEGDSC[EKTIDX];

    if (STYPE == 1) {
        ZZEKRP2N = ZZEKTRLS(HANDLE, SEGDSC[RTIDX], RECPTR, ctx)?;

        if (ZZEKRP2N == 0) {
            CHKIN(b"ZZEKRP2N", ctx)?;
            SETMSG(
                b"Record having pointer # not found in segment # of file #",
                ctx,
            );
            ERRINT(b"#", RECPTR, ctx);
            ERRINT(b"#", SEGNO, ctx);
            ERRHAN(b"#", HANDLE, ctx)?;
            SIGERR(b"SPICE(BUG)", ctx)?;
            CHKOUT(b"ZZEKRP2N", ctx)?;
        }
    } else if (STYPE == 2) {
        ZZEKRP2N = RECPTR;
    } else {
        CHKIN(b"ZZEKRP2N", ctx)?;
        SETMSG(
            b"Segment type # is not supported.  SEGNO = #. File = #.",
            ctx,
        );
        ERRINT(b"#", STYPE, ctx);
        ERRINT(b"#", SEGNO, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZEKRP2N", ctx)?;
    }

    Ok(ZZEKRP2N)
}
