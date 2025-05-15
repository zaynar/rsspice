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
const BUFSIZ: i32 = CPSIZE;

//$Procedure     ZZEKAC06 ( EK, add class 6 column to segment )
pub fn ZZEKAC06(
    HANDLE: i32,
    SEGDSC: &[i32],
    COLDSC: &[i32],
    CVALS: CharArray,
    ENTSZS: &[i32],
    NLFLGS: &[bool],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let SEGDSC = DummyArray::new(SEGDSC, 1..);
    let COLDSC = DummyArray::new(COLDSC, 1..);
    let CVALS = DummyCharArray::new(CVALS, None, 1..);
    let ENTSZS = DummyArray::new(ENTSZS, 1..);
    let NLFLGS = DummyArray::new(NLFLGS, 1..);
    let mut COLUMN = [b' '; CNAMSZ as usize];
    let mut PAGE = [b' '; PGSIZC as usize];
    let mut ADRBUF = ActualArray::<i32>::new(1..=BUFSIZ);
    let mut BUFPTR: i32 = 0;
    let mut CLASS: i32 = 0;
    let mut COLIDX: i32 = 0;
    let mut CP: i32 = 0;
    let mut CURCHR: i32 = 0;
    let mut CURSIZ: i32 = 0;
    let mut CVLEN: i32 = 0;
    let mut FROM: i32 = 0;
    let mut L: i32 = 0;
    let mut N: i32 = 0;
    let mut NCHARS: i32 = 0;
    let mut NDATA: i32 = 0;
    let mut NLINK: i32 = 0;
    let mut NROWS: i32 = 0;
    let mut NULPTR: i32 = 0;
    let mut NW: i32 = 0;
    let mut NWRITE: i32 = 0;
    let mut P: i32 = 0;
    let mut P2: i32 = 0;
    let mut PADLEN: i32 = 0;
    let mut PBASE: i32 = 0;
    let mut REMAIN: i32 = 0;
    let mut ROOM: i32 = 0;
    let mut ROW: i32 = 0;
    let mut SIZE: i32 = 0;
    let mut STRLEN: i32 = 0;
    let mut TO: i32 = 0;
    let mut CNTINU: bool = false;
    let mut FIXSIZ: bool = false;
    let mut NEWENT: bool = false;
    let mut NEWREQ: bool = false;
    let mut NULLOK: bool = false;
    let mut PAD: bool = false;

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
        CHKIN(b"ZZEKAC06", ctx)?;
    }

    //
    // Grab the column's attributes.
    //
    CLASS = COLDSC[CLSIDX];
    NULPTR = COLDSC[NFLIDX];
    COLIDX = COLDSC[ORDIDX];
    SIZE = COLDSC[SIZIDX];
    STRLEN = COLDSC[LENIDX];

    NULLOK = (NULPTR != IFALSE);
    FIXSIZ = (SIZE != IFALSE);

    //
    // This column had better be class 6.
    //
    if (CLASS != 6) {
        ZZEKCNAM(HANDLE, COLDSC.as_slice(), &mut COLUMN, ctx)?;

        SETMSG(
            b"Column class code # found in descriptor for column #.  Class should be 6.",
            ctx,
        );
        ERRINT(b"#", CLASS, ctx);
        ERRCH(b"#", &COLUMN, ctx);
        SIGERR(b"SPICE(NOCLASS)", ctx)?;
        CHKOUT(b"ZZEKAC06", ctx)?;
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
                    NDATA = (NDATA + (STRLEN * SIZE));
                } else {
                    NDATA = (NDATA + (STRLEN * ENTSZS[I]));
                }
            }
        }
    } else {
        if FIXSIZ {
            NDATA = ((NROWS * STRLEN) * SIZE);
        } else {
            NDATA = 0;

            for I in 1..=NROWS {
                NDATA = (NDATA + (STRLEN * ENTSZS[I]));
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
            CHR,
            false,
            &mut P,
            &mut PBASE,
            ctx,
        )?;
        fstr::assign(&mut PAGE, b" ");

        //
        // Decide now whether we will need to pad the input entry
        // elements with trailing blanks, and if so how much padding
        // we'll need.
        //
        CVLEN = intrinsics::MIN0(&[intrinsics::LEN(&CVALS[1]), STRLEN]);
        PAD = (CVLEN < STRLEN);

        if PAD {
            PADLEN = (STRLEN - CVLEN);
        }
    }

    //
    // Write the input data out to the target file a page at a time.
    // Null values don't get written.
    //
    // While we're at it, we'll push onto the EK stack the addresses
    // of the column entries.  We use the constant NULL rather than an
    // address to represent null entries.
    //
    // We'll use FROM to indicate the element of CVALS we're
    // considering, TO to indicate the element of PAGE to write
    // to, and BUFPTR to indicate the element of ADRBUF to write
    // addresses to.   The variable N indicates the number of characters
    // written to the current page.  NCHARS is the number of characters
    // written in the current column entry. CP is the position in the
    // current input string of the character which we'll read next.
    //

    REMAIN = NDATA;
    FROM = 1;
    TO = 1;
    BUFPTR = 1;
    ROW = 1;
    CP = 1;
    N = 0;
    NCHARS = 0;
    NLINK = 0;
    NEWENT = true;

    while (ROW <= NROWS) {
        //
        // NEWREQ is set to TRUE if we discover that the next column
        // entry must start on a new page.
        //
        NEWREQ = false;

        //
        // FROM and TO are expected to be properly set at this point.
        //

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
            CNTINU = false;
            NEWENT = true;
        } else {
            if NEWENT {
                //
                // We're about to write out a new column entry.  We must
                // insert the element count into the page before writing the
                // data.  The link count for the current page must be
                // incremented to account for this new entry.
                //
                // At this point, we're guaranteed at least ENCSIZ+1 free
                // spaces in the current page.
                //
                if FIXSIZ {
                    CURSIZ = SIZE;
                } else {
                    CURSIZ = ENTSZS[ROW];
                }

                CURCHR = (CURSIZ * STRLEN);
                NCHARS = 0;
                CP = 1;

                ADRBUF[BUFPTR] = (TO + PBASE);
                BUFPTR = (BUFPTR + 1);

                PRTENC(
                    CURSIZ,
                    fstr::substr_mut(&mut PAGE, TO..=((TO + ENCSIZ) - 1)),
                    ctx,
                )?;

                TO = (TO + ENCSIZ);
                N = (N + ENCSIZ);
                NLINK = (NLINK + 1);
                NEWENT = false;
            }

            //
            // At this point, there's at least one free space in the
            // current page.  There's also at least one character to
            // write.  Transfer as much as possible of the current
            // column entry to the current page.
            //
            ROOM = (CPSIZE - N);
            NWRITE = intrinsics::MIN0(&[(CURCHR - NCHARS), ROOM]);
            NW = NWRITE;

            while (NW > 0) {
                //
                // At this point, we're guaranteed that
                //
                //    CP      <=  STRLEN
                //    TO      <   CPSIZE
                //    FROM is set correctly.
                //
                if PAD {
                    //
                    // The input strings must be padded with blanks up to
                    // a length of STRLEN characters.  The number of blanks
                    // used to pad the input is PADLEN.
                    //
                    if (CP < CVLEN) {
                        //
                        // Compute the number of `actual' characters of data
                        // left in the current input string.
                        //
                        // Transfer the characters we have room for from the
                        // current input string to the current page.
                        //
                        L = ((CVLEN - CP) + 1);
                        L = intrinsics::MIN0(&[L, NW]);

                        fstr::assign(
                            fstr::substr_mut(&mut PAGE, TO..=((TO + L) - 1)),
                            fstr::substr(CVALS.get(FROM), CP..=((CP + L) - 1)),
                        );
                        CP = (CP + L);
                        NW = (NW - L);
                        TO = (TO + L);
                    } else {
                        //
                        // The input character pointer is in the `pad' zone.
                        // Let L be the length of padding that is required
                        // and can fit in the page.
                        //
                        L = ((STRLEN - CP) + 1);
                        L = intrinsics::MIN0(&[L, NW]);

                        fstr::assign(fstr::substr_mut(&mut PAGE, TO..=((TO + L) - 1)), b" ");
                        CP = (CP + L);
                        NW = (NW - L);
                        TO = (TO + L);
                    }
                } else {
                    //
                    // The input data doesn't require padding.
                    //
                    // Compute the number of `actual' characters of data
                    // left in the current input string.
                    //
                    // Transfer the characters we have room for from the
                    // current input string to the current page.
                    //
                    L = ((STRLEN - CP) + 1);
                    L = intrinsics::MIN0(&[L, NW]);

                    fstr::assign(
                        fstr::substr_mut(&mut PAGE, TO..=((TO + L) - 1)),
                        fstr::substr(CVALS.get(FROM), CP..=((CP + L) - 1)),
                    );
                    CP = (CP + L);
                    NW = (NW - L);
                    TO = (TO + L);
                }

                //
                // If the input pointer is beyond the end of the declared
                // length of the target column's strings STRLEN, it's time
                // to look at the next input string.
                //
                if (CP > STRLEN) {
                    FROM = (FROM + 1);
                    CP = 1;
                }
            }

            //
            // We've written NWRITE characters to the current page.  FROM,
            // TO, and CP are set.
            //
            N = (N + NWRITE);
            REMAIN = (REMAIN - NWRITE);
            NCHARS = (NCHARS + NWRITE);

            //
            // Decide whether we must continue the current entry on another
            // data page.
            //
            CNTINU = ((NCHARS < CURCHR) && (N == CPSIZE));

            //
            // If we've finished writing out a column entry, get ready
            // to write the next one.
            //
            if (NCHARS == CURCHR) {
                //
                // The current character is the last of the current column
                // entry.
                //
                // Determine whether we must start the next column entry on
                // a new page.  To start a column entry on the current page,
                // we must have enough room for the element count and at
                // least one character of data.
                //
                if (REMAIN > 0) {
                    NEWREQ = (N > ((CPSIZE - ENCSIZ) - 1));
                }

                ROW = (ROW + 1);
                NEWENT = true;
            }
        }

        //
        // At this point, CNTINU indicates whether we need to continue
        // the current entry on another page.  If we finished writing out
        // the entry, CNTINU is .FALSE.
        //

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
            PRTENC(
                NLINK,
                fstr::substr_mut(&mut PAGE, CLCIDX..=((CLCIDX + ENCSIZ) - 1)),
                ctx,
            )?;

            //
            // Write out the data page.
            //
            ZZEKPGWC(HANDLE, P, &PAGE, ctx)?;

            //
            // If there's more data to write, allocate another page.
            //
            if (REMAIN > 0) {
                ZZEKAPS(
                    HANDLE,
                    SEGDSC.as_slice(),
                    CHR,
                    false,
                    &mut P2,
                    &mut PBASE,
                    ctx,
                )?;
                fstr::assign(&mut PAGE, b" ");

                N = 0;
                NLINK = 0;
                TO = 1;

                //
                // If we're continuing an element from the previous page,
                // link the previous page to the current one.
                //
                if CNTINU {
                    ZZEKSFWD(HANDLE, CHR, P, P2, ctx)?;
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

    CHKOUT(b"ZZEKAC06", ctx)?;
    Ok(())
}
