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

//$Procedure     ZZEKIXDL ( EK, delete record from index )
pub fn ZZEKIXDL(
    HANDLE: i32,
    SEGDSC: &[i32],
    COLDSC: &[i32],
    RECPTR: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let SEGDSC = DummyArray::new(SEGDSC, 1..=SDSCSZ);
    let COLDSC = DummyArray::new(COLDSC, 1..=CDSCSZ);
    let mut IDX: i32 = 0;
    let mut IDXTYP: i32 = 0;
    let mut RECNO: i32 = 0;
    let mut TREE: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Non-SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZEKIXDL", ctx)?;

    IDXTYP = COLDSC[IXTIDX];

    if (IDXTYP != IFALSE) {
        //
        // This column is indexed.
        //
        // Some entry in the index points to RECPTR.  Find the entry
        // and delete it.
        //
        ZZEKFRX(
            HANDLE,
            SEGDSC.as_slice(),
            COLDSC.as_slice(),
            RECPTR,
            &mut IDX,
            ctx,
        )?;

        if (IDXTYP == 1) {
            //
            // For type 1 indexes, the index pointer is the root node of
            // a B*-tree.  Just use the tree deletion routine.
            //
            TREE = COLDSC[IXPIDX];

            ZZEKTRDL(HANDLE, TREE, IDX, ctx)?;
        } else {
            //
            // Sorry, no other types of indexes are supported.
            //
            SETMSG(b"The index type # is not supported.", ctx);
            ERRINT(b"#", IDXTYP, ctx);
            SIGERR(b"SPICE(INVALIDTYPE)", ctx)?;
            CHKOUT(b"ZZEKIXDL", ctx)?;
            return Ok(());
        }
    } else {
        //
        // This routine should not have been called if the column in
        // question is not indexed.
        //
        RECNO = ZZEKRP2N(HANDLE, SEGDSC[SNOIDX], RECPTR, ctx)?;

        SETMSG(
            b"Column was not indexed. File = #; RECNO = #; COLIDX = #.",
            ctx,
        );
        ERRHAN(b"#", HANDLE, ctx)?;
        ERRINT(b"#", RECNO, ctx);
        ERRINT(b"#", COLDSC[ORDIDX], ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZEKIXDL", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZEKIXDL", ctx)?;
    Ok(())
}
