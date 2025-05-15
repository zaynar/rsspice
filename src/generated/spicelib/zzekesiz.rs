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
const CNAMSZ: i32 = 32;
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

//$Procedure ZZEKESIZ ( EK, element entry size )
pub fn ZZEKESIZ(
    HANDLE: i32,
    SEGDSC: &[i32],
    COLDSC: &[i32],
    RECPTR: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<i32> {
    let SEGDSC = DummyArray::new(SEGDSC, 1..=SDSCSZ);
    let COLDSC = DummyArray::new(COLDSC, 1..=CDSCSZ);
    let mut ZZEKESIZ: i32 = 0;
    let mut COLUMN = [b' '; CNAMSZ as usize];
    let mut CLASS: i32 = 0;
    let mut RECNO: i32 = 0;
    let mut SEGNO: i32 = 0;

    //
    // Non-SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Initialize the function's return value.
    //
    ZZEKESIZ = 0;

    //
    // Use discovery check-in.
    //
    // Delegate the problem to the routine of the appropriate class.
    // The first three classes are scalars.
    //
    //
    CLASS = COLDSC[CLSIDX];

    if (CLASS == 1) {
        ZZEKESIZ = 1;
    } else if (CLASS == 2) {
        ZZEKESIZ = 1;
    } else if (CLASS == 3) {
        ZZEKESIZ = 1;
    } else if (CLASS == 4) {
        ZZEKESIZ = ZZEKSZ04(HANDLE, SEGDSC.as_slice(), COLDSC.as_slice(), RECPTR, ctx)?;
    } else if (CLASS == 5) {
        ZZEKESIZ = ZZEKSZ05(HANDLE, SEGDSC.as_slice(), COLDSC.as_slice(), RECPTR, ctx)?;
    } else if (CLASS == 6) {
        ZZEKESIZ = ZZEKSZ06(HANDLE, SEGDSC.as_slice(), COLDSC.as_slice(), RECPTR, ctx)?;
    } else if (CLASS == 7) {
        ZZEKESIZ = 1;
    } else if (CLASS == 8) {
        ZZEKESIZ = 1;
    } else if (CLASS == 9) {
        ZZEKESIZ = 1;
    } else {
        //
        // This is an unsupported column class.
        //
        ZZEKCNAM(HANDLE, COLDSC.as_slice(), &mut COLUMN, ctx)?;

        RECNO = ZZEKRP2N(HANDLE, SEGDSC[SNOIDX], RECPTR, ctx)?;
        SEGNO = SEGDSC[SNOIDX];

        CHKIN(b"ZZEKESIZ", ctx)?;
        SETMSG(b"Class # from input column descriptor is not a supported integer class.  COLUMN = #; RECNO = #; SEGNO = #; EK = #.", ctx);
        ERRINT(b"#", CLASS, ctx);
        ERRCH(b"#", &COLUMN, ctx);
        ERRINT(b"#", RECNO, ctx);
        ERRINT(b"#", SEGNO, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        SIGERR(b"SPICE(NOCLASS)", ctx)?;
        CHKOUT(b"ZZEKESIZ", ctx)?;
        return Ok(ZZEKESIZ);
    }

    Ok(ZZEKESIZ)
}
