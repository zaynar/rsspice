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

//$Procedure   ZZEKRD06 ( EK, read class 6 column entry elements )
pub fn ZZEKRD06(
    HANDLE: i32,
    SEGDSC: &[i32],
    COLDSC: &[i32],
    RECPTR: i32,
    BEG: i32,
    END: i32,
    CVALS: CharArrayMut,
    ISNULL: &mut bool,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let SEGDSC = DummyArray::new(SEGDSC, 1..=SDSCSZ);
    let COLDSC = DummyArray::new(COLDSC, 1..=CDSCSZ);
    let mut CVALS = DummyCharArrayMut::new(CVALS, None, 1..);
    let mut COLUMN = [b' '; CNAMSZ as usize];
    let mut AVAIL: i32 = 0;
    let mut BASE: i32 = 0;
    let mut COLIDX: i32 = 0;
    let mut CVLEN: i32 = 0;
    let mut D: i32 = 0;
    let mut DATPTR: i32 = 0;
    let mut DELTA: i32 = 0;
    let mut ELTIDX: i32 = 0;
    let mut ELTOFF: i32 = 0;
    let mut MAXELT: i32 = 0;
    let mut NCOLS: i32 = 0;
    let mut NELT: i32 = 0;
    let mut NREAD: i32 = 0;
    let mut NSKIP: i32 = 0;
    let mut OFFSET: i32 = 0;
    let mut P: i32 = 0;
    let mut PAGNUM: i32 = 0;
    let mut PG: i32 = 0;
    let mut PTRLOC: i32 = 0;
    let mut PTROFF: i32 = 0;
    let mut RECNO: i32 = 0;
    let mut REMAIN: i32 = 0;
    let mut START: i32 = 0;
    let mut STRLEN: i32 = 0;

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
        CHKIN(b"ZZEKRD06", ctx)?;
        SETMSG(b"Column index = #; valid range is 1:#.", ctx);
        ERRINT(b"#", COLIDX, ctx);
        ERRINT(b"#", NCOLS, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"ZZEKRD06", ctx)?;
        return Ok(());
    }

    //
    // Make sure the output buffer is wide enough to hold the returned
    // strings.
    //
    CVLEN = intrinsics::LEN(&CVALS[1]);
    STRLEN = COLDSC[LENIDX];

    if (STRLEN > CVLEN) {
        //
        // We have a string truncation error.  Look up the column
        // name, record number, and file name before signaling an
        // error.
        //
        ZZEKCNAM(HANDLE, COLDSC.as_slice(), &mut COLUMN, ctx)?;

        RECNO = ZZEKRP2N(HANDLE, SEGDSC[SNOIDX], RECPTR, ctx)?;

        CHKIN(b"ZZEKRD06", ctx)?;
        SETMSG(b"String value has length #; output string can hold only # characters.  COLUMN = #; SEGNO = #; RECNO = #; EK = #", ctx);
        ERRINT(b"#", STRLEN, ctx);
        ERRINT(b"#", CVLEN, ctx);
        ERRCH(b"#", &COLUMN, ctx);
        ERRINT(b"#", SEGDSC[SNOIDX], ctx);
        ERRINT(b"#", RECNO, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        SIGERR(b"SPICE(STRINGTRUNCATED)", ctx)?;
        CHKOUT(b"ZZEKRD06", ctx)?;
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
        ZZEKGEI(HANDLE, DATPTR, &mut NELT, ctx)?;

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
        // locate the element at index BEG.  We'll first decide on which
        // page the desired element starts.  The first page holds up to
        // CPSIZE - ENCSIZ characters; the rest hold CPSIZE characters.
        // While we're at it, we'll compute the offset ELTOFF of the
        // element from the base of the page on which the element starts.
        // We'll use the name OFFSET to represent the character offset
        // of the element from the base of the page on which the column
        // entry starts.
        //
        ZZEKPGPG(CHR, DATPTR, &mut P, &mut BASE, ctx)?;

        PTROFF = (DATPTR - BASE);
        OFFSET = ((PTROFF + ENCSIZ) + (STRLEN * (BEG - 1)));

        if (OFFSET <= CPSIZE) {
            PAGNUM = 1;
            ELTOFF = OFFSET;
        } else {
            PAGNUM = (((OFFSET + CPSIZE) - 1) / CPSIZE);
            ELTOFF = (OFFSET - ((PAGNUM - 1) * CPSIZE));
        }

        //
        // Get the absolute page number and base address of the page
        // on which the element starts.  If this is not the page on
        // which the column entry starts, we'll chain along using
        // the page's forward links until we arrive at the correct page.
        //
        PG = 1;

        while (PG < PAGNUM) {
            //
            // Get the link to the next page, then look up the base
            // address of that page.
            //
            ZZEKGEI(HANDLE, (BASE + CFPIDX), &mut P, ctx)?;
            ZZEKPGBS(CHR, P, &mut BASE, ctx)?;

            PG = (PG + 1);
        }

        //
        // The desired element starts at address BASE + ELTOFF.
        //
        DATPTR = (BASE + ELTOFF);

        //
        // At this point, P is the page on which the element having index
        // BEG is located.  BASE is the base address of this page.
        //
        // Read the strings one at a time.
        //
        ELTIDX = 1;
        MAXELT = ((END - BEG) + 1);

        while ((ELTIDX <= MAXELT) && !FAILED(ctx)) {
            //
            // Read the current string.  The string may be continued over
            // multiple pages.  Read only as many characters as will fit
            // in the output buffer element CVALS(ELTIDX).
            //
            REMAIN = intrinsics::MIN0(&[CVLEN, STRLEN]);
            START = 1;

            while ((REMAIN > 0) && !FAILED(ctx)) {
                AVAIL = (((BASE + CPSIZE) - DATPTR) + 1);
                NREAD = intrinsics::MIN0(&[REMAIN, AVAIL]);

                if (NREAD > 0) {
                    DASRDC(
                        HANDLE,
                        DATPTR,
                        ((DATPTR + NREAD) - 1),
                        START,
                        ((START + NREAD) - 1),
                        CVALS.subarray_mut(ELTIDX),
                        ctx,
                    )?;

                    START = (START + NREAD);
                    REMAIN = (REMAIN - NREAD);
                    DATPTR = (DATPTR + NREAD);
                } else {
                    //
                    // Go to the next page for the continuation of the
                    // current string.
                    //
                    ZZEKGEI(HANDLE, (BASE + CFPIDX), &mut P, ctx)?;
                    ZZEKPGBS(CHR, P, &mut BASE, ctx)?;

                    DATPTR = (BASE + 1);
                }
            }

            //
            // If we did not read all of the current array element,
            // we'll need to advance DATPTR past the end of the element.
            // If this advance moved DATPTR beyond the last character
            // of the current page, the logic above will set DATPTR to
            // indicate the first character of the next continuation page.
            //
            DELTA = (STRLEN - CVLEN);

            if (DELTA > 0) {
                D = DELTA;

                while (D > 0) {
                    AVAIL = (((BASE + CPSIZE) - DATPTR) + 1);
                    NSKIP = intrinsics::MIN0(&[D, AVAIL]);

                    if (NSKIP > 0) {
                        D = (D - NSKIP);
                        DATPTR = (DATPTR + NSKIP);
                    } else {
                        //
                        // Go to the next page for the continuation of the
                        // current string.
                        //
                        ZZEKGEI(HANDLE, (BASE + CFPIDX), &mut P, ctx)?;
                        ZZEKPGBS(CHR, P, &mut BASE, ctx)?;

                        DATPTR = (BASE + 1);
                    }
                }
            }

            //
            // Blank-pad the output string if necessary.
            //
            if (CVLEN > STRLEN) {
                fstr::assign(
                    fstr::substr_mut(CVALS.get_mut(ELTIDX), (STRLEN + 1)..),
                    b" ",
                );
            }

            ELTIDX = (ELTIDX + 1);
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

        ZZEKCNAM(HANDLE, COLDSC.as_slice(), &mut COLUMN, ctx)?;

        CHKIN(b"ZZEKRD06", ctx)?;
        SETMSG(b"Attempted to read uninitialized column entry.  SEGNO = #; COLUMN = #; RECNO = #; EK = #", ctx);
        ERRINT(b"#", SEGDSC[SNOIDX], ctx);
        ERRCH(b"#", &COLUMN, ctx);
        ERRINT(b"#", RECNO, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        SIGERR(b"SPICE(UNINITIALIZED)", ctx)?;
        CHKOUT(b"ZZEKRD06", ctx)?;
        return Ok(());
    } else {
        //
        // The data pointer is corrupted.
        //
        RECNO = ZZEKRP2N(HANDLE, SEGDSC[SNOIDX], RECPTR, ctx)?;

        ZZEKCNAM(HANDLE, COLDSC.as_slice(), &mut COLUMN, ctx)?;

        CHKIN(b"ZZEKRD06", ctx)?;
        SETMSG(
            b"Data pointer is corrupted. SEGNO = #; COLUMN =  #; RECNO = #; EK = #",
            ctx,
        );
        ERRINT(b"#", SEGDSC[SNOIDX], ctx);
        ERRCH(b"#", &COLUMN, ctx);
        ERRINT(b"#", RECNO, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZEKRD06", ctx)?;
        return Ok(());
    }

    Ok(())
}
