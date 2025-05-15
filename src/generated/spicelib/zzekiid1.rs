//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const CTRUE: &[u8] = b"T";
const CFALSE: &[u8] = b"F";
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
const EQ: i32 = 1;
const GE: i32 = (EQ + 1);
const GT: i32 = (GE + 1);
const LE: i32 = (GT + 1);
const LT: i32 = (LE + 1);
const NE: i32 = (LT + 1);
const LIKE: i32 = (NE + 1);
const UNLIKE: i32 = (LIKE + 1);
const ISNULL: i32 = (UNLIKE + 1);
const NOTNUL: i32 = (ISNULL + 1);
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

//$Procedure ZZEKIID1 ( EK, insert into index, d.p., type 1 )
pub fn ZZEKIID1(
    HANDLE: i32,
    SEGDSC: &[i32],
    COLDSC: &[i32],
    DKEY: f64,
    RECPTR: i32,
    NULL: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let SEGDSC = DummyArray::new(SEGDSC, 1..=SDSCSZ);
    let COLDSC = DummyArray::new(COLDSC, 1..=CDSCSZ);
    let mut COLUMN = [b' '; CNAMSZ as usize];
    let mut DTYPE: i32 = 0;
    let mut ITYPE: i32 = 0;
    let mut PRVIDX: i32 = 0;
    let mut PRVPTR: i32 = 0;
    let mut TREE: i32 = 0;
    let mut INDEXD: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //
    if FAILED(ctx) {
        return Ok(());
    }

    //
    // If the column's not indexed, we have no business being here.
    //
    INDEXD = (COLDSC[IXTIDX] != IFALSE);

    if !INDEXD {
        ZZEKCNAM(HANDLE, COLDSC.as_slice(), &mut COLUMN, ctx)?;

        CHKIN(b"ZZEKIID1", ctx)?;
        SETMSG(b"Column # is not indexed.", ctx);
        ERRCH(b"#", &COLUMN, ctx);
        SIGERR(b"SPICE(NOTINDEXED)", ctx)?;
        CHKOUT(b"ZZEKIID1", ctx)?;
        return Ok(());
    }

    //
    // Check the column's data type.
    //
    DTYPE = COLDSC[TYPIDX];

    if ((DTYPE != DP) && (DTYPE != TIME)) {
        ZZEKCNAM(HANDLE, COLDSC.as_slice(), &mut COLUMN, ctx)?;

        CHKIN(b"ZZEKIID1", ctx)?;
        SETMSG(b"Column # should be DP or TIME but has type #.", ctx);
        ERRCH(b"#", &COLUMN, ctx);
        ERRINT(b"#", DTYPE, ctx);
        SIGERR(b"SPICE(INVALIDTYPE)", ctx)?;
        CHKOUT(b"ZZEKIID1", ctx)?;
        return Ok(());
    }

    ITYPE = COLDSC[IXTIDX];

    if (ITYPE == 1) {
        //
        // Get the tree pointer from the column descriptor.
        //
        TREE = COLDSC[IXPIDX];

        //
        // Locate the predecessor of the input key, record pointer pair.
        //
        ZZEKLERD(
            HANDLE,
            SEGDSC.as_slice(),
            COLDSC.as_slice(),
            DKEY,
            RECPTR,
            NULL,
            &mut PRVIDX,
            &mut PRVPTR,
            ctx,
        )?;

        //
        // Insert the new record pointer right after the item we've found.
        //
        ZZEKTRIN(HANDLE, TREE, (PRVIDX + 1), RECPTR, ctx)?;
    } else {
        ZZEKCNAM(HANDLE, COLDSC.as_slice(), &mut COLUMN, ctx)?;

        CHKIN(b"ZZEKIID1", ctx)?;
        SETMSG(b"Column # has index type #.", ctx);
        ERRCH(b"#", &COLUMN, ctx);
        ERRINT(b"#", ITYPE, ctx);
        SIGERR(b"SPICE(INVALIDTYPE)", ctx)?;
        CHKOUT(b"ZZEKIID1", ctx)?;
        return Ok(());
    }

    Ok(())
}
