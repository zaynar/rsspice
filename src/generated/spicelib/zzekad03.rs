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

//$Procedure     ZZEKAD03 ( EK, add data to class 3 column )
pub fn ZZEKAD03(
    HANDLE: i32,
    SEGDSC: &mut [i32],
    COLDSC: &[i32],
    RECPTR: i32,
    CVAL: &[u8],
    ISNULL: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut SEGDSC = DummyArrayMut::new(SEGDSC, 1..);
    let COLDSC = DummyArray::new(COLDSC, 1..);
    let mut COLIDX: i32 = 0;
    let mut DATPTR: i32 = 0;
    let mut ITYPE: i32 = 0;
    let mut LASTW: i32 = 0;
    let mut LNB: i32 = 0;
    let mut MBASE: i32 = 0;
    let mut N: i32 = 0;
    let mut NCOLS: i32 = 0;
    let mut NLINKS: i32 = 0;
    let mut NWRITE: i32 = 0;
    let mut P: i32 = 0;
    let mut P2: i32 = 0;
    let mut PBASE: i32 = 0;
    let mut PCOUNT: i32 = 0;
    let mut POS: i32 = 0;
    let mut PRVBAS: i32 = 0;
    let mut PTRLOC: i32 = 0;
    let mut RECNO: i32 = 0;
    let mut STRLEN: i32 = 0;
    let mut FIXLEN: bool = false;

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
    // Use discovery check-in.
    //
    // Make sure the column exists.
    //
    NCOLS = SEGDSC[NCIDX];
    COLIDX = COLDSC[ORDIDX];

    if ((COLIDX < 1) || (COLIDX > NCOLS)) {
        CHKIN(b"ZZEKAD03", ctx)?;
        SETMSG(b"Column index = #; valid range is 1:#.", ctx);
        ERRINT(b"#", COLIDX, ctx);
        ERRINT(b"#", NCOLS, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"ZZEKAD03", ctx)?;
        return Ok(());
    }

    //
    // If the value is null, make sure that nulls are permitted
    // in this column.
    //
    if (ISNULL && (COLDSC[NFLIDX] != ITRUE)) {
        RECNO = ZZEKRP2N(HANDLE, SEGDSC[SNOIDX], RECPTR, ctx)?;

        CHKIN(b"ZZEKAD03", ctx)?;
        SETMSG(b"Column having index # in segment # does not allow nulls, but a null value was supplied for the element in record #.", ctx);
        ERRINT(b"#", COLIDX, ctx);
        ERRINT(b"#", SEGDSC[SNOIDX], ctx);
        ERRINT(b"#", RECNO, ctx);
        SIGERR(b"SPICE(BADATTRIBUTE)", ctx)?;
        CHKOUT(b"ZZEKAD03", ctx)?;
        return Ok(());
    }

    //
    // Decide the length of the string value.  If the column contains
    // variable-length strings, the effective length of the string is
    // just the non-blank length of CVAL.  Otherwise, the effective
    // string length is the minimum of the non-blank length and the
    // column's declared string length.  We don't store trailing blanks.
    //
    FIXLEN = (COLDSC[LENIDX] != IFALSE);
    LNB = RTRIM(CVAL);

    if FIXLEN {
        STRLEN = intrinsics::MIN0(&[COLDSC[LENIDX], LNB]);
    } else {
        STRLEN = LNB;
    }

    //
    // Compute the data pointer location.
    //
    PTRLOC = ((RECPTR + DPTBAS) + COLIDX);

    if ISNULL {
        //
        // All we need do is set the data pointer.  The segment's
        // metadata are not affected.
        //
        DASUDI(HANDLE, PTRLOC, PTRLOC, &[NULL], ctx)?;
    } else {
        //
        // Write out the data value.  If we run out of room in the
        // page we're writing to, we allocate a new page and link
        // the previous page to it.
        //
        N = STRLEN;
        POS = 1;
        LASTW = SEGDSC[LCWIDX];
        P = SEGDSC[LCPIDX];
        PCOUNT = 0;

        while (N > 0) {
            //
            // Write as much data as possible into the current page.
            //

            if (LASTW < (CPSIZE - ENCSIZ)) {
                //
                // There's room in the current page.  We never split an
                // encoded character count across pages, and we always
                // write at least one data character to the current page.
                // This practice is slightly wasteful of space but greatly
                // simplifies our logic.
                //
                // Keep track of the number of pages our string spans.
                //
                PCOUNT = (PCOUNT + 1);

                //
                // If this is the first data page, write the data pointer
                // into the record pointer and the character count into
                // the data page.
                //
                if (PCOUNT == 1) {
                    ZZEKPGBS(CHR, P, &mut PBASE, ctx)?;

                    DATPTR = ((PBASE + LASTW) + 1);

                    DASUDI(HANDLE, PTRLOC, PTRLOC, &[DATPTR], ctx)?;
                    ZZEKSEI(HANDLE, DATPTR, STRLEN, ctx)?;
                    //
                    // Advance the data pointer to the first data
                    // character's position.  The last word in use
                    // increases as well.
                    //
                    DATPTR = (DATPTR + ENCSIZ);
                    LASTW = (LASTW + ENCSIZ);
                } else {
                    //
                    // We still need the data pointer.
                    //
                    DATPTR = (PBASE + 1);
                }

                //
                // Compute the number of characters to write into this page,
                // and write that number of characters.
                //
                NWRITE = intrinsics::MIN0(&[(CPSIZE - LASTW), N]);

                DASUDC(
                    HANDLE,
                    DATPTR,
                    ((DATPTR + NWRITE) - 1),
                    1,
                    NWRITE,
                    CharArray::from_ref(fstr::substr(CVAL, POS..)),
                    ctx,
                )?;

                N = (N - NWRITE);
                POS = (POS + NWRITE);

                //
                // The page containing the data item gains a link.
                //
                ZZEKGLNK(HANDLE, CHR, P, &mut NLINKS, ctx)?;
                ZZEKSLNK(HANDLE, CHR, P, (NLINKS + 1), ctx)?;

                //
                // The last character word in use must be updated.
                //
                LASTW = (LASTW + NWRITE);
                SEGDSC[LCWIDX] = LASTW;

                //
                // Retain the base address of this data page.
                //
                PRVBAS = PBASE;
            } else {
                //
                // Allocate a data page.  If this is not the first data
                // page written to, link the previous page to the current
                // one.
                //
                ZZEKAPS(
                    HANDLE,
                    SEGDSC.as_slice(),
                    CHR,
                    false,
                    &mut P2,
                    &mut PBASE,
                    ctx,
                )?;

                if (PCOUNT > 0) {
                    ZZEKSFWD(HANDLE, CHR, P, P2, ctx)?;
                }

                //
                // The last character page and word in use must be updated.
                //
                P = P2;
                LASTW = 0;
                SEGDSC[LCPIDX] = P;
                SEGDSC[LCWIDX] = LASTW;

                //
                // Make sure the link count is zeroed out.
                //
                ZZEKSLNK(HANDLE, CHR, P, 0, ctx)?;
            }
        }
    }

    //
    // Write out the updated segment descriptor.
    //
    MBASE = SEGDSC[IMDIDX];

    DASUDI(
        HANDLE,
        (MBASE + 1),
        (MBASE + SDSCSZ),
        SEGDSC.as_slice(),
        ctx,
    )?;

    //
    // If the column is indexed, we must update the index to account
    // for the new element.
    //
    ITYPE = COLDSC[IXTIDX];

    if (ITYPE != IFALSE) {
        //
        // The column is indexed.
        //
        if (ITYPE == 1) {
            //
            // The column has a type 1 index.  Insert the record number
            // of the current element at the appropriate location.
            //
            ZZEKIIC1(
                HANDLE,
                SEGDSC.as_slice(),
                COLDSC.as_slice(),
                CVAL,
                RECPTR,
                ISNULL,
                ctx,
            )?;
        } else {
            CHKIN(b"ZZEKAD03", ctx)?;
            SETMSG(b"Column having index # in segment # has index type #.", ctx);
            ERRINT(b"#", COLIDX, ctx);
            ERRINT(b"#", SEGDSC[SNOIDX], ctx);
            ERRINT(b"#", ITYPE, ctx);
            SIGERR(b"SPICE(INVALIDTYPE)", ctx)?;
            CHKOUT(b"ZZEKAD03", ctx)?;
            return Ok(());
        }
    }

    Ok(())
}
