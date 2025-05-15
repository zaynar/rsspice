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

//$Procedure   ZZEKRD05 ( EK, read class 5 column entry elements )
pub fn ZZEKRD05(
    HANDLE: i32,
    SEGDSC: &[i32],
    COLDSC: &[i32],
    RECPTR: i32,
    BEG: i32,
    END: i32,
    DVALS: &mut [f64],
    ISNULL: &mut bool,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let SEGDSC = DummyArray::new(SEGDSC, 1..=SDSCSZ);
    let COLDSC = DummyArray::new(COLDSC, 1..=CDSCSZ);
    let mut DVALS = DummyArrayMut::new(DVALS, 1..);
    let mut DPNELT: f64 = 0.0;
    let mut BASE: i32 = 0;
    let mut COLIDX: i32 = 0;
    let mut DATPTR: i32 = 0;
    let mut MAXIDX: i32 = 0;
    let mut MINIDX: i32 = 0;
    let mut NCOLS: i32 = 0;
    let mut NELT: i32 = 0;
    let mut NREAD: i32 = 0;
    let mut P: i32 = 0;
    let mut PTEMP: i32 = 0;
    let mut PTRLOC: i32 = 0;
    let mut RECNO: i32 = 0;
    let mut REMAIN: i32 = 0;
    let mut START: i32 = 0;

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
    //
    // Make sure the column exists.
    //
    NCOLS = SEGDSC[NCIDX];
    COLIDX = COLDSC[ORDIDX];

    if ((COLIDX < 1) || (COLIDX > NCOLS)) {
        CHKIN(b"ZZEKRD05", ctx)?;
        SETMSG(b"Column index = #; valid range is 1:#.", ctx);
        ERRINT(b"#", COLIDX, ctx);
        ERRINT(b"#", NCOLS, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"ZZEKRD05", ctx)?;
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
        // The entry is non-null.
        //
        *ISNULL = false;

        //
        // Get the element count.  Check for range specifications that
        // can't be met.
        //
        DASRDD(
            HANDLE,
            DATPTR,
            DATPTR,
            std::slice::from_mut(&mut DPNELT),
            ctx,
        )?;

        NELT = intrinsics::IDNINT(DPNELT);

        if ((BEG < 1) || (BEG > NELT)) {
            *FOUND = false;
            return Ok(());
        } else if ((END < 1) || (END > NELT)) {
            *FOUND = false;
            return Ok(());
        } else if (END < BEG) {
            *FOUND = false;
            return Ok(());
        }

        //
        // The request is valid, so read the data.  The first step is to
        // locate the element at index BEG.
        //
        ZZEKPGPG(DP, DATPTR, &mut P, &mut BASE, ctx)?;

        MINIDX = 1;
        MAXIDX = ((BASE + DPSIZE) - DATPTR);
        DATPTR = (DATPTR + BEG);

        while (MAXIDX < BEG) {
            //
            // Locate the page on which the element is continued.
            //
            ZZEKGFWD(HANDLE, DP, P, &mut PTEMP, ctx)?;
            P = PTEMP;

            ZZEKPGBS(DP, P, &mut BASE, ctx)?;

            //
            // Determine the highest-indexed element of the column entry
            // located on the current page.
            //
            MINIDX = (MAXIDX + 1);
            MAXIDX = intrinsics::MIN0(&[(MAXIDX + DPSIZE), NELT]);

            //
            // The following assignment will set DATPTR to the correct
            // value on the last pass through this loop.
            //
            DATPTR = ((BASE + 1) + (BEG - MINIDX));
        }

        //
        // At this point, P is the page on which the element having index
        // BEG is located.  BASE is the base address of this page.
        // MAXIDX is the highest index of any element on the current page.
        //
        REMAIN = ((END - BEG) + 1);
        START = 1;

        //
        // Decide how many elements to read from the current page, and
        // read them.
        //
        NREAD = intrinsics::MIN0(&[REMAIN, (((BASE + DPSIZE) - DATPTR) + 1)]);

        DASRDD(
            HANDLE,
            DATPTR,
            ((DATPTR + NREAD) - 1),
            DVALS.subarray_mut(START),
            ctx,
        )?;

        REMAIN = (REMAIN - NREAD);

        while ((REMAIN > 0) && !FAILED(ctx)) {
            //
            // Locate the page on which the element is continued.
            //
            ZZEKGFWD(HANDLE, DP, P, &mut PTEMP, ctx)?;
            P = PTEMP;

            ZZEKPGBS(DP, P, &mut BASE, ctx)?;

            DATPTR = (BASE + 1);
            START = (START + NREAD);
            NREAD = intrinsics::MIN0(&[REMAIN, DPSIZE]);

            DASRDD(
                HANDLE,
                DATPTR,
                ((DATPTR + NREAD) - 1),
                DVALS.subarray_mut(START),
                ctx,
            )?;

            REMAIN = (REMAIN - NREAD);
        }

        *FOUND = !FAILED(ctx);
    } else if (DATPTR == NULL) {
        //
        // The value is null.
        //
        *ISNULL = true;
        *FOUND = true;
    } else if (DATPTR == UNINIT) {
        //
        // The data value is absent.  This is an error.
        //
        RECNO = ZZEKRP2N(HANDLE, SEGDSC[SNOIDX], RECPTR, ctx)?;

        CHKIN(b"ZZEKRD05", ctx)?;
        SETMSG(b"Attempted to read uninitialized column entry.  SEGNO = #; COLIDX = #; RECNO = #; EK = #", ctx);
        ERRINT(b"#", SEGDSC[SNOIDX], ctx);
        ERRINT(b"#", COLIDX, ctx);
        ERRINT(b"#", RECNO, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        SIGERR(b"SPICE(UNINITIALIZEDVALUE)", ctx)?;
        CHKOUT(b"ZZEKRD05", ctx)?;
        return Ok(());
    } else {
        //
        // The data pointer is corrupted.
        //
        RECNO = ZZEKRP2N(HANDLE, SEGDSC[SNOIDX], RECPTR, ctx)?;

        CHKIN(b"ZZEKRD05", ctx)?;
        SETMSG(
            b"Data pointer is corrupted. SEGNO = #; COLIDX =  #; RECNO = #; EK = #",
            ctx,
        );
        ERRINT(b"#", SEGDSC[SNOIDX], ctx);
        ERRINT(b"#", COLIDX, ctx);
        ERRINT(b"#", RECNO, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZEKRD05", ctx)?;
        return Ok(());
    }

    Ok(())
}
