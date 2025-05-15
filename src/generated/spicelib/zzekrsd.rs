//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CNAMSZ: i32 = 32;
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

//$Procedure   ZZEKRSD ( EK, read scalar, double precision )
pub fn ZZEKRSD(
    HANDLE: i32,
    SEGDSC: &[i32],
    COLDSC: &[i32],
    RECPTR: i32,
    ELTIDX: i32,
    DVAL: &mut f64,
    ISNULL: &mut bool,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let SEGDSC = DummyArray::new(SEGDSC, 1..=SDSCSZ);
    let COLDSC = DummyArray::new(COLDSC, 1..=CDSCSZ);
    let mut COLUMN = [b' '; CNAMSZ as usize];
    let mut CLASS: i32 = 0;
    let mut DTYPE: i32 = 0;
    let mut RECNO: i32 = 0;
    let mut SEGNO: i32 = 0;

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
    // Nothing found to begin with.
    //
    *FOUND = false;

    //
    // This column had better be of d.p. or TIME type.
    //
    DTYPE = COLDSC[TYPIDX];

    if ((DTYPE != DP) && (DTYPE != TIME)) {
        ZZEKCNAM(HANDLE, COLDSC.as_slice(), &mut COLUMN, ctx)?;

        SEGNO = SEGDSC[SNOIDX];
        RECNO = ZZEKRP2N(HANDLE, SEGDSC[SNOIDX], RECPTR, ctx)?;

        CHKIN(b"ZZEKRSD", ctx)?;
        SETMSG(b"Column # is of type #; ZZEKRSD only works with DP or TIME columns.  RECNO = #; SEGNO = #; EK = #.", ctx);
        ERRCH(b"#", &COLUMN, ctx);
        ERRINT(b"#", DTYPE, ctx);
        ERRINT(b"#", RECNO, ctx);
        ERRINT(b"#", SEGNO, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        SIGERR(b"SPICE(WRONGDATATYPE)", ctx)?;
        CHKOUT(b"ZZEKRSD", ctx)?;
        return Ok(());
    }

    //
    // Now it's time to read data from the file.  Call the low-level
    // reader appropriate to the column's class.
    //
    CLASS = COLDSC[CLSIDX];

    if (CLASS == 2) {
        ZZEKRD02(
            HANDLE,
            SEGDSC.as_slice(),
            COLDSC.as_slice(),
            RECPTR,
            DVAL,
            ISNULL,
            ctx,
        )?;

        *FOUND = true;
    } else if (CLASS == 5) {
        //
        // Class 5 columns contain d.p. array entries.
        //
        ZZEKRD05(
            HANDLE,
            SEGDSC.as_slice(),
            COLDSC.as_slice(),
            RECPTR,
            ELTIDX,
            ELTIDX,
            std::slice::from_mut(DVAL),
            ISNULL,
            FOUND,
            ctx,
        )?;
    } else if (CLASS == 8) {
        ZZEKRD08(
            HANDLE,
            SEGDSC.as_slice(),
            COLDSC.as_slice(),
            RECPTR,
            DVAL,
            ISNULL,
            ctx,
        )?;

        *FOUND = true;
    } else {
        //
        // This is an unsupported d.p. column class.
        //
        ZZEKCNAM(HANDLE, COLDSC.as_slice(), &mut COLUMN, ctx)?;

        SEGNO = SEGDSC[SNOIDX];
        RECNO = ZZEKRP2N(HANDLE, SEGDSC[SNOIDX], RECPTR, ctx)?;

        CHKIN(b"ZZEKRSD", ctx)?;
        SETMSG(b"Class # from input column descriptor is not a supported d.p. class.  COLUMN = #; RECNO = #; SEGNO = #; EK = #.", ctx);
        ERRINT(b"#", CLASS, ctx);
        ERRCH(b"#", &COLUMN, ctx);
        ERRINT(b"#", RECNO, ctx);
        ERRINT(b"#", SEGNO, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        SIGERR(b"SPICE(NOCLASS)", ctx)?;
        CHKOUT(b"ZZEKRSD", ctx)?;
        return Ok(());
    }

    Ok(())
}
