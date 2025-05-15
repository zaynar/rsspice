//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const CTRUE: &[u8] = b"T";
const CFALSE: &[u8] = b"F";
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

//$Procedure   ZZEKRD02 ( EK, read class 2 column entry )
pub fn ZZEKRD02(
    HANDLE: i32,
    SEGDSC: &[i32],
    COLDSC: &[i32],
    RECPTR: i32,
    DVAL: &mut f64,
    ISNULL: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let SEGDSC = DummyArray::new(SEGDSC, 1..=SDSCSZ);
    let COLDSC = DummyArray::new(COLDSC, 1..=CDSCSZ);
    let mut COLIDX: i32 = 0;
    let mut DATPTR: i32 = 0;
    let mut NCOLS: i32 = 0;
    let mut PTRLOC: i32 = 0;
    let mut RECNO: i32 = 0;

    //
    // Non-SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //
    //
    // Make sure the column exists.
    //
    NCOLS = SEGDSC[NCIDX];
    COLIDX = COLDSC[ORDIDX];

    if ((COLIDX < 1) || (COLIDX > NCOLS)) {
        CHKIN(b"ZZEKRD02", ctx)?;
        SETMSG(b"Column index = #; valid range is 1:#.", ctx);
        ERRINT(b"#", COLIDX, ctx);
        ERRINT(b"#", NCOLS, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"ZZEKRD02", ctx)?;
        return Ok(());
    }

    //
    // Compute the data pointer location, and read the pointer.
    //
    PTRLOC = ((RECPTR + DPTBAS) + COLIDX);

    DASRDI(
        HANDLE,
        PTRLOC,
        PTRLOC,
        std::slice::from_mut(&mut DATPTR),
        ctx,
    )?;

    if (DATPTR > 0) {
        //
        // Just read the value.
        //
        DASRDD(HANDLE, DATPTR, DATPTR, std::slice::from_mut(DVAL), ctx)?;
        *ISNULL = false;
    } else if (DATPTR == NULL) {
        //
        // The value is null.
        //
        *ISNULL = true;
    } else if ((DATPTR == UNINIT) || (DATPTR == NOBACK)) {
        //
        // The data value is absent.  This is an error.
        //
        RECNO = ZZEKRP2N(HANDLE, SEGDSC[SNOIDX], RECPTR, ctx)?;

        CHKIN(b"ZZEKRD02", ctx)?;
        SETMSG(b"Attempted to read uninitialized column entry.  SEGNO = #; COLIDX = #; RECNO = #; EK = #", ctx);
        ERRINT(b"#", SEGDSC[SNOIDX], ctx);
        ERRINT(b"#", COLIDX, ctx);
        ERRINT(b"#", RECNO, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        SIGERR(b"SPICE(UNINITIALIZEDVALUE)", ctx)?;
        CHKOUT(b"ZZEKRD02", ctx)?;
        return Ok(());
    } else {
        //
        // The data pointer is corrupted.
        //
        CHKIN(b"ZZEKRD02", ctx)?;
        SETMSG(
            b"Data pointer is corrupted. SEGNO = #; COLIDX =  #; RECNO = #; EK = #",
            ctx,
        );
        ERRINT(b"#", SEGDSC[SNOIDX], ctx);
        ERRINT(b"#", COLIDX, ctx);
        ERRINT(b"#", RECNO, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZEKRD02", ctx)?;
        return Ok(());
    }

    Ok(())
}
