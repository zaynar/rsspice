//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CDOFF: i32 = 24;
const CDSCSZ: i32 = 11;
const CLSIDX: i32 = 1;
const TYPIDX: i32 = (CLSIDX + 1);
const LENIDX: i32 = (TYPIDX + 1);
const SIZIDX: i32 = (LENIDX + 1);
const NAMIDX: i32 = (SIZIDX + 1);
const IXTIDX: i32 = (NAMIDX + 1);
const IXPIDX: i32 = (IXTIDX + 1);
const NFLIDX: i32 = (IXPIDX + 1);
const ORDIDX: i32 = (NFLIDX + 1);
const METIDX: i32 = (ORDIDX + 1);
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

//$Procedure      ZZEKUE05 ( EK, update column entry, class 5 )
pub fn ZZEKUE05(
    HANDLE: i32,
    SEGDSC: &mut [i32],
    COLDSC: &[i32],
    RECPTR: i32,
    NVALS: i32,
    DVALS: &[f64],
    ISNULL: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut SEGDSC = DummyArrayMut::new(SEGDSC, 1..=SDSCSZ);
    let COLDSC = DummyArray::new(COLDSC, 1..=CDSCSZ);
    let DVALS = DummyArray::new(DVALS, 1..);

    //
    // SPICELIB functions
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZEKUE05", ctx)?;
    }

    //
    // Get rid of the old column entry first.
    //
    ZZEKDE05(
        HANDLE,
        SEGDSC.as_slice_mut(),
        COLDSC.as_slice(),
        RECPTR,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZEKUE05", ctx)?;
        return Ok(());
    }

    //
    // We've reduced the problem to a solved one:  that of adding
    // a column entry.
    //
    ZZEKAD05(
        HANDLE,
        SEGDSC.as_slice_mut(),
        COLDSC.as_slice(),
        RECPTR,
        NVALS,
        DVALS.as_slice(),
        ISNULL,
        ctx,
    )?;

    CHKOUT(b"ZZEKUE05", ctx)?;
    Ok(())
}
