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

//$Procedure      ZZEKDE06 ( EK, delete column entry, class 6 )
pub fn ZZEKDE06(
    HANDLE: i32,
    SEGDSC: &mut [i32],
    COLDSC: &[i32],
    RECPTR: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut SEGDSC = DummyArrayMut::new(SEGDSC, 1..=SDSCSZ);
    let COLDSC = DummyArray::new(COLDSC, 1..=CDSCSZ);
    let mut BASE: i32 = 0;
    let mut DATPTR: i32 = 0;
    let mut NCHARS: i32 = 0;
    let mut NELTS: i32 = 0;
    let mut NEXT: i32 = 0;
    let mut NLINKS: i32 = 0;
    let mut NSEEN: i32 = 0;
    let mut P: i32 = 0;
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

    CHKIN(b"ZZEKDE06", ctx)?;

    //
    // Before trying to actually modify the file, do every error
    // check we can.
    //
    // Is this file handle valid--is the file open for paged write
    // access?  Signal an error if not.
    //
    ZZEKPGCH(HANDLE, b"WRITE", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZEKDE06", ctx)?;
        return Ok(());
    }

    //
    // Compute the data pointer location.  If the data pointer is
    // already set to `uninitialized', there's nothing to do.  If
    // the element is null, just set it to `uninitialized'.  The
    // presence of actual data obligates us to clean up, however.
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
        // Get the element count for the entry.  Compute the character
        // count for the entry.
        //
        ZZEKGEI(HANDLE, DATPTR, &mut NELTS, ctx)?;

        NCHARS = (COLDSC[LENIDX] * NELTS);

        //
        // Set the data pointer to indicate the item is uninitialized.
        //
        DASUDI(HANDLE, PTRLOC, PTRLOC, &[UNINIT], ctx)?;

        //
        // Find the number of the page containing the column entry.
        //
        ZZEKPGPG(CHR, DATPTR, &mut P, &mut BASE, ctx)?;

        //
        // Look up the forward pointer.  This pointer will be valid
        // if the column entry is continued on another page.
        //
        ZZEKGFWD(HANDLE, CHR, P, &mut NEXT, ctx)?;

        //
        // Get the link count for the current page.  If we have more
        // than one link to the page, decrement the link count.  If
        // we're down to one link, this deletion will finish off the
        // page:  we'll deallocate it.
        //
        ZZEKGLNK(HANDLE, CHR, P, &mut NLINKS, ctx)?;

        if (NLINKS > 1) {
            ZZEKSLNK(HANDLE, CHR, P, (NLINKS - 1), ctx)?;
        } else {
            //
            // If we removed the last item from the page, we can delete
            // the page.  ZZEKDPS adjusts the segment's metadata
            // to reflect the deallocation.
            //
            ZZEKDPS(HANDLE, SEGDSC.as_slice_mut(), CHR, P, ctx)?;
        }

        NSEEN = intrinsics::MIN0(&[NCHARS, ((BASE + CPSIZE) - DATPTR)]);

        while ((NSEEN < NCHARS) && !FAILED(ctx)) {
            //
            // The column entry is continued on the page indicated by
            // NEXT.
            //
            // Get the link count for the current page.  If we have more
            // than one link to the page, decrement the link count.  If
            // we're down to one link, this deletion will finish off the
            // page:  we'll deallocate it.
            //
            P = NEXT;

            ZZEKGFWD(HANDLE, CHR, P, &mut NEXT, ctx)?;
            ZZEKGLNK(HANDLE, CHR, P, &mut NLINKS, ctx)?;

            if (NLINKS > 1) {
                ZZEKSLNK(HANDLE, CHR, P, (NLINKS - 1), ctx)?;
            } else {
                //
                // If we removed the last item from the page, we can delete
                // the page.  ZZEKDPS adjusts the segment's metadata
                // to reflect the deallocation.
                //
                ZZEKDPS(HANDLE, SEGDSC.as_slice_mut(), CHR, P, ctx)?;
            }

            NSEEN = intrinsics::MIN0(&[NCHARS, (CPSIZE + NSEEN)]);
        }
    } else if (DATPTR == NULL) {
        //
        // Mark the entry as `uninitialized'.
        //
        DASUDI(HANDLE, PTRLOC, PTRLOC, &[UNINIT], ctx)?;
    } else if (DATPTR != UNINIT) {
        //
        // UNINIT was the last valid possibility.  The data pointer is
        // corrupted.
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
        CHKOUT(b"ZZEKDE06", ctx)?;
        return Ok(());
    }

    //
    // Set the record's status to indicate that this record is updated.
    //
    DASUDI(HANDLE, (RECPTR + STAIDX), (RECPTR + STAIDX), &[UPDATE], ctx)?;

    CHKOUT(b"ZZEKDE06", ctx)?;
    Ok(())
}
