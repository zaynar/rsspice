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
const CNAMSZ: i32 = 32;
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
const EPARCH: i32 = 1;
const EPNIPT: i32 = 5;
const EPPSZC: i32 = (EPARCH + 1);
const EPBASC: i32 = (EPPSZC + 1);
const EPNPC: i32 = (EPBASC + 1);
const EPNFPC: i32 = (EPNPC + 1);
const EPFPC: i32 = (EPNFPC + 1);
const EPPSZD: i32 = (EPPSZC + EPNIPT);
const EPBASD: i32 = (EPPSZD + 1);
const EPNPD: i32 = (EPBASD + 1);
const EPNFPD: i32 = (EPNPD + 1);
const EPFPD: i32 = (EPNFPD + 1);
const EPPSZI: i32 = (EPPSZD + EPNIPT);
const EPBASI: i32 = (EPPSZI + 1);
const EPNPI: i32 = (EPBASI + 1);
const EPNFPI: i32 = (EPNPI + 1);
const EPFPI: i32 = (EPNFPI + 1);
const EPMDSZ: i32 = (1 + (3 * EPNIPT));
const PGSIZC: i32 = 1024;
const PGSIZD: i32 = 128;
const PGSIZI: i32 = 256;
const PGBASC: i32 = 0;
const PGBASD: i32 = 0;
const PGBASI: i32 = 256;
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
const BUFSIZ: i32 = IPSIZE;

//$Procedure     ZZEKAC04 ( EK, add class 4 column to segment )
pub fn ZZEKAC04(
    HANDLE: i32,
    SEGDSC: &[i32],
    COLDSC: &[i32],
    IVALS: &[i32],
    ENTSZS: &[i32],
    NLFLGS: &[bool],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let SEGDSC = DummyArray::new(SEGDSC, 1..);
    let COLDSC = DummyArray::new(COLDSC, 1..);
    let IVALS = DummyArray::new(IVALS, 1..);
    let ENTSZS = DummyArray::new(ENTSZS, 1..);
    let NLFLGS = DummyArray::new(NLFLGS, 1..);
    let mut COLUMN = [b' '; CNAMSZ as usize];
    let mut ADRBUF = StackArray::<i32, 254>::new(1..=BUFSIZ);
    let mut BUFPTR: i32 = 0;
    let mut CLASS: i32 = 0;
    let mut COLIDX: i32 = 0;
    let mut CURSIZ: i32 = 0;
    let mut FROM: i32 = 0;
    let mut N: i32 = 0;
    let mut NDATA: i32 = 0;
    let mut NELT: i32 = 0;
    let mut NLINK: i32 = 0;
    let mut NROWS: i32 = 0;
    let mut NULPTR: i32 = 0;
    let mut P: i32 = 0;
    let mut P2: i32 = 0;
    let mut PAGE = StackArray::<i32, 256>::new(1..=PGSIZI);
    let mut PBASE: i32 = 0;
    let mut REMAIN: i32 = 0;
    let mut ROW: i32 = 0;
    let mut SIZE: i32 = 0;
    let mut TO: i32 = 0;
    let mut CNTINU: bool = false;
    let mut FIXSIZ: bool = false;
    let mut NEWREQ: bool = false;
    let mut NULLOK: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZEKAC04", ctx)?;
    }

    //
    // Grab the column's attributes.
    //
    CLASS = COLDSC[CLSIDX];
    NULPTR = COLDSC[NFLIDX];
    COLIDX = COLDSC[ORDIDX];
    SIZE = COLDSC[SIZIDX];

    NULLOK = (NULPTR != IFALSE);
    FIXSIZ = (SIZE != IFALSE);

    //
    // This column had better be class 4.
    //
    if (CLASS != 4) {
        ZZEKCNAM(HANDLE, COLDSC.as_slice(), &mut COLUMN, ctx)?;

        SETMSG(
            b"Column class code # found in descriptor for column #.  Class should be 4.",
            ctx,
        );
        ERRINT(b"#", CLASS, ctx);
        ERRCH(b"#", &COLUMN, ctx);
        SIGERR(b"SPICE(NOCLASS)", ctx)?;
        CHKOUT(b"ZZEKAC04", ctx)?;
        return Ok(());
    }

    //
    // Push the column's ordinal index on the stack.  This allows us
    // to identify the column the addresses belong to.
    //
    ZZEKSPSH(1, &[COLIDX], ctx)?;

    //
    // Find the number of rows in the segment.
    //
    NROWS = SEGDSC[NRIDX];

    //
    // Record the number of data values to write.
    //
    if NULLOK {
        //
        // Sum the sizes of the non-null column entries; these are the
        // ones that will take up space.
        //
        NDATA = 0;

        for I in 1..=NROWS {
            if !NLFLGS[I] {
                if FIXSIZ {
                    NDATA = (NDATA + SIZE);
                } else {
                    NDATA = (NDATA + ENTSZS[I]);
                }
            }
        }
    } else {
        if FIXSIZ {
            NDATA = (NROWS * SIZE);
        } else {
            NDATA = 0;

            for I in 1..=NROWS {
                NDATA = (NDATA + ENTSZS[I]);
            }
        }
    }

    if (NDATA > 0) {
        //
        // There's some data to write, so allocate a page.  Also
        // prepare a data buffer to be written out as a page.
        //
        ZZEKAPS(
            HANDLE,
            SEGDSC.as_slice(),
            INT,
            false,
            &mut P,
            &mut PBASE,
            ctx,
        )?;
        CLEARI(PGSIZI, PAGE.as_slice_mut());
    }

    //
    // Write the input data out to the target file a page at a time.
    // Null values don't get written.
    //
    // While we're at it, we'll push onto the EK stack the addresses
    // of the column entries.  We use the constant NULL rather than an
    // address to represent null entries.
    //
    // We'll use FROM to indicate the element of IVALS we're
    // considering, TO to indicate the element of PAGE to write
    // to, and BUFPTR to indicate the element of ADRBUF to write
    // addresses to.  The variable NELT is the count of the column entry
    // elements written for the current entry.  The variable N indicates
    // the number of integers written to the current page.
    //

    REMAIN = NDATA;
    FROM = 1;
    TO = 1;
    BUFPTR = 1;
    ROW = 1;
    NELT = 1;
    N = 0;
    NLINK = 0;

    while (ROW <= NROWS) {
        //
        // NEWREQ is set to TRUE if we discover that the next column
        // entry must start on a new page.
        //
        NEWREQ = false;

        if (NULLOK && NLFLGS[ROW]) {
            if FIXSIZ {
                CURSIZ = SIZE;
            } else {
                CURSIZ = ENTSZS[ROW];
            }

            FROM = (FROM + CURSIZ);
            ADRBUF[BUFPTR] = NULL;
            BUFPTR = (BUFPTR + 1);
            ROW = (ROW + 1);
            NELT = 1;
            CNTINU = false;
        } else {
            if (NELT == 1) {
                //
                // We're about to write out a new column entry.  We must
                // insert the element count into the page before writing the
                // data.  The link count for the current page must be
                // incremented to account for this new entry.
                //
                // At this point, we're guaranteed at least two free
                // spaces in the current page.
                //
                if FIXSIZ {
                    CURSIZ = SIZE;
                } else {
                    CURSIZ = ENTSZS[ROW];
                }

                ADRBUF[BUFPTR] = (TO + PBASE);
                BUFPTR = (BUFPTR + 1);
                PAGE[TO] = CURSIZ;
                TO = (TO + 1);
                N = (N + 1);
                NLINK = (NLINK + 1);
            }

            //
            // At this point, there's at least one free space in the
            // current page.
            //
            PAGE[TO] = IVALS[FROM];
            TO = (TO + 1);
            N = (N + 1);
            FROM = (FROM + 1);
            REMAIN = (REMAIN - 1);

            //
            // Decide whether we must continue the current entry on another
            // data page.
            //
            CNTINU = ((NELT < CURSIZ) && (N == IPSIZE));

            if (NELT == CURSIZ) {
                //
                // The current element is the last of the current column
                // entry.
                //
                // Determine whether we must start the next column entry on
                // a new page.  To start a column entry on the current page,
                // we must have enough room for the element count and at
                // least the first entry element.
                //
                if (REMAIN > 0) {
                    NEWREQ = (N > (IPSIZE - 2));
                }

                NELT = 1;
                ROW = (ROW + 1);
            } else {
                NELT = (NELT + 1);
            }
        }

        if ((BUFPTR > BUFSIZ) || (ROW > NROWS)) {
            //
            // The address buffer is full or we're out of input values
            // to look at, so push the buffer contents on the stack.
            //
            ZZEKSPSH((BUFPTR - 1), ADRBUF.as_slice(), ctx)?;
            BUFPTR = 1;
        }

        if ((CNTINU || NEWREQ) || ((ROW > NROWS) && (NDATA > 0))) {
            //
            // It's time to write out the current page.  First set the link
            // count.
            //
            PAGE[ILCIDX] = NLINK;

            //
            // Write out the data page.
            //
            ZZEKPGWI(HANDLE, P, PAGE.as_slice(), ctx)?;

            //
            // If there's more data to write, allocate another page.
            //
            if (REMAIN > 0) {
                ZZEKAPS(
                    HANDLE,
                    SEGDSC.as_slice(),
                    INT,
                    false,
                    &mut P2,
                    &mut PBASE,
                    ctx,
                )?;
                CLEARI(PGSIZI, PAGE.as_slice_mut());

                N = 0;
                NLINK = 0;
                TO = 1;

                //
                // If we're continuing an element from the previous page,
                // link the previous page to the current one.
                //
                if CNTINU {
                    ZZEKSFWD(HANDLE, INT, P, P2, ctx)?;
                }

                P = P2;
            }
            //
            // We've allocated a new data page if we needed one.
            //
        }
        //
        // We've written out the last completed data page.
        //
    }
    //
    // We've processed all entries of the input array.
    //

    CHKOUT(b"ZZEKAC04", ctx)?;
    Ok(())
}
