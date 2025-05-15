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

//$Procedure     ZZEKAD04 ( EK, add data to class 4 column )
pub fn ZZEKAD04(
    HANDLE: i32,
    SEGDSC: &mut [i32],
    COLDSC: &[i32],
    RECPTR: i32,
    NVALS: i32,
    IVALS: &[i32],
    ISNULL: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut SEGDSC = DummyArrayMut::new(SEGDSC, 1..);
    let COLDSC = DummyArray::new(COLDSC, 1..);
    let IVALS = DummyArray::new(IVALS, 1..);
    let mut COLIDX: i32 = 0;
    let mut DATPTR: i32 = 0;
    let mut LASTW: i32 = 0;
    let mut MBASE: i32 = 0;
    let mut NCOLS: i32 = 0;
    let mut NLINKS: i32 = 0;
    let mut NREC: i32 = 0;
    let mut NWRITE: i32 = 0;
    let mut P: i32 = 0;
    let mut P2: i32 = 0;
    let mut PBASE: i32 = 0;
    let mut PTRLOC: i32 = 0;
    let mut RECNO: i32 = 0;
    let mut REMAIN: i32 = 0;
    let mut ROOM: i32 = 0;
    let mut START: i32 = 0;
    let mut FSTPAG: bool = false;

    //
    // Non-SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //
    NREC = SEGDSC[NRIDX];
    COLIDX = COLDSC[ORDIDX];

    //
    // Make sure the column exists.
    //
    NCOLS = SEGDSC[NCIDX];

    if ((COLIDX < 1) || (COLIDX > NCOLS)) {
        CHKIN(b"ZZEKAD04", ctx)?;
        SETMSG(b"Column index = #; valid range is 1:#.", ctx);
        ERRINT(b"#", COLIDX, ctx);
        ERRINT(b"#", NREC, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"ZZEKAD04", ctx)?;
        return Ok(());
    }

    //
    // If the value is null, make sure that nulls are permitted
    // in this column.
    //
    if (ISNULL && (COLDSC[NFLIDX] != ITRUE)) {
        RECNO = ZZEKRP2N(HANDLE, SEGDSC[SNOIDX], RECPTR, ctx)?;

        CHKIN(b"ZZEKAD04", ctx)?;
        SETMSG(b"Column having index # in segment # does not allow nulls, but a null value was supplied for the element in record #.", ctx);
        ERRINT(b"#", COLIDX, ctx);
        ERRINT(b"#", SEGDSC[SNOIDX], ctx);
        ERRINT(b"#", RECNO, ctx);
        SIGERR(b"SPICE(BADATTRIBUTE)", ctx)?;
        CHKOUT(b"ZZEKAD04", ctx)?;
        return Ok(());
    }

    //
    // Check NVALS.  If the column has fixed-size entries, NVALS must
    // match the declared entry size.  In all cases, NVALS must be
    // positive.
    //
    if (NVALS < 1) {
        CHKIN(b"ZZEKAD04", ctx)?;
        SETMSG(
            b"COLIDX = #;  segment = #; NVALS = #;  NVALS must be positive ",
            ctx,
        );
        ERRINT(b"#", COLIDX, ctx);
        ERRINT(b"#", SEGDSC[SNOIDX], ctx);
        ERRINT(b"#", NVALS, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"ZZEKAD04", ctx)?;
        return Ok(());
    }

    if (COLDSC[SIZIDX] != IFALSE) {
        if (NVALS != COLDSC[SIZIDX]) {
            CHKIN(b"ZZEKAD04", ctx)?;
            SETMSG(
                b"COLIDX = #;  segment = #; NVALS = #; declared entry size = #.  Sizes must match.",
                ctx,
            );
            ERRINT(b"#", COLIDX, ctx);
            ERRINT(b"#", SEGDSC[SNOIDX], ctx);
            ERRINT(b"#", NVALS, ctx);
            ERRINT(b"#", COLDSC[SIZIDX], ctx);
            SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
            CHKOUT(b"ZZEKAD04", ctx)?;
            return Ok(());
        }
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
        LASTW = SEGDSC[LIWIDX];
        ROOM = (IPSIZE - LASTW);
        REMAIN = NVALS;
        START = 1;
        FSTPAG = true;

        while (REMAIN > 0) {
            //
            // Decide where to write the data values.  In order to write
            // to the current page, we require enough room for the count
            // and at least one column entry element.
            //
            if (ROOM >= 2) {
                //
                // There's room in the current page.  If this is the first
                // page this entry is written on, set the data pointer
                // and count.  Write as much of the value as possible to
                // the current page.
                //
                P = SEGDSC[LIPIDX];

                ZZEKPGBS(INT, P, &mut PBASE, ctx)?;

                DATPTR = ((PBASE + LASTW) + 1);

                if FSTPAG {
                    DASUDI(HANDLE, PTRLOC, PTRLOC, &[DATPTR], ctx)?;
                    DASUDI(HANDLE, DATPTR, DATPTR, &[NVALS], ctx)?;

                    ROOM = (ROOM - 1);
                    DATPTR = (DATPTR + 1);
                }

                NWRITE = intrinsics::MIN0(&[REMAIN, ROOM]);

                DASUDI(
                    HANDLE,
                    DATPTR,
                    ((DATPTR + NWRITE) - 1),
                    IVALS.subarray(START),
                    ctx,
                )?;

                REMAIN = (REMAIN - NWRITE);
                ROOM = (ROOM - NWRITE);
                START = (START + NWRITE);

                //
                // The page containing the data item gains a link.
                //
                ZZEKGLNK(HANDLE, INT, P, &mut NLINKS, ctx)?;
                ZZEKSLNK(HANDLE, INT, P, (NLINKS + 1), ctx)?;

                //
                // The last integer word in use must be updated.  Account
                // for the count, if this is the first page on which the
                // current entry is written.
                //
                if FSTPAG {
                    SEGDSC[LIWIDX] = ((LASTW + 1) + NWRITE);
                    FSTPAG = false;
                } else {
                    SEGDSC[LIWIDX] = (LASTW + NWRITE);
                }
            } else {
                //
                // Allocate a data page.  If this is not the first data
                // page written to, link the previous page to the current
                // one.
                //
                ZZEKAPS(
                    HANDLE,
                    SEGDSC.as_slice(),
                    INT,
                    false,
                    &mut P2,
                    &mut PBASE,
                    ctx,
                )?;

                if !FSTPAG {
                    ZZEKSFWD(HANDLE, INT, P, P2, ctx)?;
                }

                //
                // The last integer page and word in use must be updated.
                //
                P = P2;
                LASTW = 0;
                SEGDSC[LIPIDX] = P;
                SEGDSC[LIWIDX] = LASTW;
                ROOM = IPSIZE;

                //
                // Make sure the link count is zeroed out.
                //
                ZZEKSLNK(HANDLE, INT, P, 0, ctx)?;
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
    // Class 4 columns are not indexed, so we need not update any
    // index to account for the new element.
    //

    Ok(())
}
