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

//$Procedure      ZZEKUE02 ( EK, update column entry, class 2 )
pub fn ZZEKUE02(
    HANDLE: i32,
    SEGDSC: &mut [i32],
    COLDSC: &[i32],
    RECPTR: i32,
    DVAL: f64,
    ISNULL: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut SEGDSC = DummyArrayMut::new(SEGDSC, 1..=SDSCSZ);
    let COLDSC = DummyArray::new(COLDSC, 1..=CDSCSZ);
    let mut DATPTR: i32 = 0;
    let mut IDXTYP: i32 = 0;
    let mut NLINKS: i32 = 0;
    let mut P: i32 = 0;
    let mut PBASE: i32 = 0;
    let mut PTRLOC: i32 = 0;
    let mut RECNO: i32 = 0;

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

    CHKIN(b"ZZEKUE02", ctx)?;

    //
    // Is this file handle valid--is the file open for paged write
    // access?  Signal an error if not.
    //
    ZZEKPGCH(HANDLE, b"WRITE", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZEKUE02", ctx)?;
        return Ok(());
    }

    //
    // Compute the data pointer location.
    //
    PTRLOC = ((RECPTR + DPTBAS) + COLDSC[ORDIDX]);

    DASRDI(
        HANDLE,
        PTRLOC,
        PTRLOC,
        std::slice::from_mut(&mut DATPTR),
        ctx,
    )?;

    if (DATPTR > 0) {
        //
        // The column entry is non-null.  Determine whether the column is
        // indexed.
        //
        IDXTYP = COLDSC[IXTIDX];

        if (IDXTYP == 1) {
            //
            // The column has a type 1 index.  Delete the index entry
            // for this column.  Create an index entry for the new value.
            //
            ZZEKIXDL(HANDLE, SEGDSC.as_slice(), COLDSC.as_slice(), RECPTR, ctx)?;

            ZZEKIID1(
                HANDLE,
                SEGDSC.as_slice(),
                COLDSC.as_slice(),
                DVAL,
                RECPTR,
                ISNULL,
                ctx,
            )?;
        } else if (IDXTYP != IFALSE) {
            SETMSG(b"Column having index # in segment # has index type #.", ctx);
            ERRINT(b"#", COLDSC[ORDIDX], ctx);
            ERRINT(b"#", SEGDSC[SNOIDX], ctx);
            ERRINT(b"#", IDXTYP, ctx);
            SIGERR(b"SPICE(INVALIDTYPE)", ctx)?;
            CHKOUT(b"ZZEKUE02", ctx)?;
            return Ok(());
        }

        //
        // If the new value is null, set the data pointer to indicate a
        // null value.  Otherwise, overwrite the old value with the new
        // one.
        //
        if ISNULL {
            //
            // The data location used by the previous value is no longer
            // needed, so we have one less link to this page.
            //
            ZZEKPGPG(DP, DATPTR, &mut P, &mut PBASE, ctx)?;
            ZZEKGLNK(HANDLE, DP, P, &mut NLINKS, ctx)?;
            ZZEKSLNK(HANDLE, DP, P, (NLINKS - 1), ctx)?;

            DASUDI(HANDLE, PTRLOC, PTRLOC, &[NULL], ctx)?;
        } else {
            //
            // No link counts change; we just have a new value.
            //
            DASUDD(HANDLE, DATPTR, DATPTR, &[DVAL], ctx)?;
        }
    } else if (DATPTR == NULL) {
        //
        // If the new entry is null too, there's nothing to do.
        // We don't have to adjust link counts or indexes.
        //
        // If the new entry is non-null, we must add a new column entry,
        // since no space was reserved for the old one.  The column
        // index entry must be cleaned up, if the column is indexed.
        //
        if !ISNULL {
            IDXTYP = COLDSC[IXTIDX];

            if (IDXTYP == 1) {
                //
                // The column has a type 1 index.  Delete the index entry
                // for this column.
                //
                ZZEKIXDL(HANDLE, SEGDSC.as_slice(), COLDSC.as_slice(), RECPTR, ctx)?;
            } else if (IDXTYP != IFALSE) {
                SETMSG(b"Column having index # in segment # has index type #.", ctx);
                ERRINT(b"#", COLDSC[ORDIDX], ctx);
                ERRINT(b"#", SEGDSC[SNOIDX], ctx);
                ERRINT(b"#", IDXTYP, ctx);
                SIGERR(b"SPICE(INVALIDTYPE)", ctx)?;
                CHKOUT(b"ZZEKUE02", ctx)?;
                return Ok(());
            }

            //
            // We don't need to decrement the link count for this page.
            // Just add the new value to the column.  But first, set the
            // data pointer to indicate an uninitialized value, so the
            // data addition routine doesn't choke.
            //
            DASUDI(HANDLE, PTRLOC, PTRLOC, &[UNINIT], ctx)?;

            ZZEKAD02(
                HANDLE,
                SEGDSC.as_slice_mut(),
                COLDSC.as_slice(),
                RECPTR,
                DVAL,
                ISNULL,
                ctx,
            )?;
        }
    } else if ((DATPTR == UNINIT) || (DATPTR == NOBACK)) {
        //
        // There is no current column entry.  Just add a new entry.
        //
        ZZEKAD02(
            HANDLE,
            SEGDSC.as_slice_mut(),
            COLDSC.as_slice(),
            RECPTR,
            DVAL,
            ISNULL,
            ctx,
        )?;
    } else {
        //
        // The data pointer is corrupted.
        //
        RECNO = ZZEKRP2N(HANDLE, SEGDSC[SNOIDX], RECPTR, ctx)?;

        SETMSG(
            b"Data pointer is corrupted. SEGNO = #; COLIDX =  #; RECNO = #; EK = #",
            ctx,
        );
        ERRINT(b"#", SEGDSC[SNOIDX], ctx);
        ERRINT(b"#", COLDSC[ORDIDX], ctx);
        ERRINT(b"#", RECNO, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZEKUE02", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZEKUE02", ctx)?;
    Ok(())
}
