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

//$Procedure     ZZEKAC03 ( EK, add class 3 column to segment )
pub fn ZZEKAC03(
    HANDLE: i32,
    SEGDSC: &[i32],
    COLDSC: &[i32],
    CVALS: CharArray,
    NLFLGS: &[bool],
    RCPTRS: &[i32],
    WKINDX: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let SEGDSC = DummyArray::new(SEGDSC, 1..);
    let COLDSC = DummyArray::new(COLDSC, 1..);
    let CVALS = DummyCharArray::new(CVALS, None, 1..);
    let NLFLGS = DummyArray::new(NLFLGS, 1..);
    let RCPTRS = DummyArray::new(RCPTRS, 1..);
    let mut WKINDX = DummyArrayMut::new(WKINDX, 1..);
    let mut COLUMN = [b' '; CNAMSZ as usize];
    let mut PAGE = [b' '; PGSIZC as usize];
    let mut ADRBUF = ActualArray::<i32>::new(1..=BUFSIZ);
    let mut BUFPTR: i32 = 0;
    let mut CLASS: i32 = 0;
    let mut COLIDX: i32 = 0;
    let mut COLWID: i32 = 0;
    let mut DSCBAS: i32 = 0;
    let mut FROM: i32 = 0;
    let mut IDXTYP: i32 = 0;
    let mut P: i32 = 0;
    let mut P2: i32 = 0;
    let mut PBASE: i32 = 0;
    let mut POS: i32 = 0;
    let mut MBASE: i32 = 0;
    let mut N: i32 = 0;
    let mut NCHARS: i32 = 0;
    let mut NDATA: i32 = 0;
    let mut NLINKS: i32 = 0;
    let mut NNULL: i32 = 0;
    let mut NROWS: i32 = 0;
    let mut NULPTR: i32 = 0;
    let mut NWRITE: i32 = 0;
    let mut REMAIN: i32 = 0;
    let mut ROOM: i32 = 0;
    let mut STRLEN: i32 = 0;
    let mut TO: i32 = 0;
    let mut TREE: i32 = 0;
    let mut FIXLEN: bool = false;
    let mut INDEXD: bool = false;
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
        CHKIN(b"ZZEKAC03", ctx)?;
    }

    //
    // Grab the column's attributes.  Initialize the maximum non-blank
    // width of the column.
    //
    CLASS = COLDSC[CLSIDX];
    IDXTYP = COLDSC[IXTIDX];
    NULPTR = COLDSC[NFLIDX];
    COLIDX = COLDSC[ORDIDX];
    COLWID = COLDSC[LENIDX];

    NULLOK = (NULPTR != IFALSE);
    INDEXD = (IDXTYP != IFALSE);
    FIXLEN = (COLWID != IFALSE);

    //
    // This column had better be class 3.
    //
    if (CLASS != 3) {
        ZZEKCNAM(HANDLE, COLDSC.as_slice(), &mut COLUMN, ctx)?;

        SETMSG(
            b"Column class code # found in descriptor for column #.  Class should be 3.",
            ctx,
        );
        ERRINT(b"#", CLASS, ctx);
        ERRCH(b"#", &COLUMN, ctx);
        SIGERR(b"SPICE(NOCLASS)", ctx)?;
        CHKOUT(b"ZZEKAC03", ctx)?;
        return Ok(());
    }

    //
    // If the column is indexed, the index type should be 1; we don't
    // know how to create any other type of index.
    //
    if (INDEXD && (IDXTYP != 1)) {
        ZZEKCNAM(HANDLE, COLDSC.as_slice(), &mut COLUMN, ctx)?;

        SETMSG(
            b"Index type code # found in descriptor for column #.  Code should be 1.",
            ctx,
        );
        ERRINT(b"#", IDXTYP, ctx);
        ERRCH(b"#", &COLUMN, ctx);
        SIGERR(b"SPICE(UNRECOGNIZEDTYPE)", ctx)?;
        CHKOUT(b"ZZEKAC03", ctx)?;
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
    // Count the number of strings to write.
    //
    if NULLOK {
        //
        // Count the non-null column entries; these are the
        // ones that will take up space.
        //
        NNULL = 0;
        NDATA = 0;

        for I in 1..=NROWS {
            if NLFLGS[I] {
                NNULL = (NNULL + 1);
            }
        }

        NDATA = (NROWS - NNULL);
    } else {
        NDATA = NROWS;
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
        // The link count starts out at zero.
        //
        PRTENC(
            0,
            fstr::substr_mut(&mut PAGE, CLCIDX..=((CLCIDX + ENCSIZ) - 1)),
            ctx,
        )?;
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
    // considering, TO to indicate the first character of PAGE to write
    // to, and BUFPTR to indicate the element of ADRBUF to write
    // addresses to.  The variable N indicates the number of characters
    // written to the current page.  NCHARS indicates the number of
    // characters left to write from the current input element.  NWRITE
    // will be used to count the column entries written so far.
    //

    REMAIN = NROWS;
    FROM = 0;
    TO = 1;
    BUFPTR = 1;
    NWRITE = 0;
    N = 0;

    while (REMAIN > 0) {
        //
        // Examine a column entry.  Write it out if it's non-null.
        //
        FROM = (FROM + 1);

        if (NULLOK && NLFLGS[FROM]) {
            ADRBUF[BUFPTR] = NULL;
        } else {
            //
            // Write out the current column entry.  The entry
            // might span multiple pages.  However, we're guaranteed
            // enough room to write out to the current page the encoded
            // character count and at least one character of data.
            //
            // Update the non-blank width for the column each time we
            // determine the length of an input string.
            //
            if FIXLEN {
                STRLEN = intrinsics::MIN0(&[RTRIM(&CVALS[FROM]), COLWID]);
            } else {
                STRLEN = RTRIM(&CVALS[FROM]);
            }

            ADRBUF[BUFPTR] = (TO + PBASE);
            POS = 1;

            //
            // Start out with the string length.
            //
            PRTENC(
                STRLEN,
                fstr::substr_mut(&mut PAGE, TO..=((TO + ENCSIZ) - 1)),
                ctx,
            )?;

            N = (N + ENCSIZ);
            TO = (N + 1);
            NCHARS = STRLEN;

            while (NCHARS > 0) {
                ROOM = (CPSIZE - N);

                if (NCHARS <= ROOM) {
                    //
                    // The remaining portion of the string will fit on the
                    // current page.
                    //
                    fstr::assign(
                        fstr::substr_mut(&mut PAGE, TO..=((TO + NCHARS) - 1)),
                        fstr::substr(CVALS.get(FROM), POS..=((POS + NCHARS) - 1)),
                    );
                    N = (N + NCHARS);
                    TO = (N + 1);
                    NCHARS = 0;

                    //
                    // Add a link to the current page.
                    //

                    PRTDEC(
                        fstr::substr(&PAGE, CLCIDX..=((CLCIDX + ENCSIZ) - 1)),
                        &mut NLINKS,
                        ctx,
                    )?;

                    PRTENC(
                        (NLINKS + 1),
                        fstr::substr_mut(&mut PAGE, CLCIDX..=((CLCIDX + ENCSIZ) - 1)),
                        ctx,
                    )?;
                } else {
                    //
                    // The string will have to be continued on another page.
                    // Write out the first ROOM characters to the current
                    // page first.
                    //
                    fstr::assign(
                        fstr::substr_mut(&mut PAGE, TO..=CPSIZE),
                        fstr::substr(CVALS.get(FROM), POS..=((POS + ROOM) - 1)),
                    );
                    POS = (POS + ROOM);
                    NCHARS = (NCHARS - ROOM);

                    //
                    // Add a link to the current page.
                    //
                    PRTDEC(
                        fstr::substr(&PAGE, CLCIDX..=((CLCIDX + ENCSIZ) - 1)),
                        &mut NLINKS,
                        ctx,
                    )?;

                    PRTENC(
                        (NLINKS + 1),
                        fstr::substr_mut(&mut PAGE, CLCIDX..=((CLCIDX + ENCSIZ) - 1)),
                        ctx,
                    )?;

                    //
                    // Allocate another page.  Fill in the forward pointer
                    // in the previous page.
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

                    PRTENC(
                        P2,
                        fstr::substr_mut(&mut PAGE, CFPIDX..=((CFPIDX + ENCSIZ) - 1)),
                        ctx,
                    )?;

                    //
                    // Write out the full data page.  Get ready to write
                    // to the new page.
                    //
                    ZZEKPGWC(HANDLE, P, &PAGE, ctx)?;

                    P = P2;
                    fstr::assign(&mut PAGE, b" ");
                    PRTENC(
                        0,
                        fstr::substr_mut(&mut PAGE, CLCIDX..=((CLCIDX + ENCSIZ) - 1)),
                        ctx,
                    )?;

                    N = 0;
                    TO = 1;
                }
            }
            //
            // We've written out a column entry.
            //
            NWRITE = (NWRITE + 1);
        }
        //
        // We're done with the current column entry, null or not.
        //

        if (NWRITE < NDATA) {
            //
            // There is at least one more column entry to write.
            // If there's not enough room on the current page to begin
            // writing another column entry, write out the page and
            // allocate another.
            //
            ROOM = (CPSIZE - N);

            if (ROOM < (1 + ENCSIZ)) {
                ZZEKPGWC(HANDLE, P, &PAGE, ctx)?;
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
                PRTENC(
                    0,
                    fstr::substr_mut(&mut PAGE, CLCIDX..=((CLCIDX + ENCSIZ) - 1)),
                    ctx,
                )?;

                N = 0;
                TO = 1;
            }
        } else if (N > 0) {
            //
            // We've written the last of the non-null data to the current
            // page.  Write out this page.
            //
            ZZEKPGWC(HANDLE, P, &PAGE, ctx)?;
            N = 0;
        }

        REMAIN = (REMAIN - 1);

        if ((BUFPTR == BUFSIZ) || (REMAIN == 0)) {
            //
            // The address buffer is full or we're out of input values
            // to look at, so push the buffer contents on the stack.
            //
            ZZEKSPSH(BUFPTR, ADRBUF.as_slice(), ctx)?;
            BUFPTR = 1;
        } else {
            BUFPTR = (BUFPTR + 1);
        }
    }

    //
    // If the column is supposed to have an index, now is the time to
    // build that index.  We'll find the order vector for the input
    // values, overwrite the elements of the order vector with the
    // corresponding elements of the input array of record pointers, then
    // load this sorted copy of the record pointer array into a tree in
    // one shot.
    //
    if INDEXD {
        ZZEKORDC(
            CVALS.as_arg(),
            NULLOK,
            NLFLGS.as_slice(),
            NROWS,
            WKINDX.as_slice_mut(),
        );

        for I in 1..=NROWS {
            WKINDX[I] = RCPTRS[WKINDX[I]];
        }

        ZZEKTRIT(HANDLE, &mut TREE, ctx)?;
        ZZEKTR1S(HANDLE, TREE, NROWS, WKINDX.as_slice(), ctx)?;

        //
        // Update the segment's metadata to point to the index.  The
        // pointer indicates the root page of the tree.
        //
        MBASE = SEGDSC[IMDIDX];
        DSCBAS = ((MBASE + SDSCSZ) + ((COLIDX - 1) * CDSCSZ));

        DASUDI(HANDLE, (DSCBAS + IXPIDX), (DSCBAS + IXPIDX), &[TREE], ctx)?;
    }

    CHKOUT(b"ZZEKAC03", ctx)?;
    Ok(())
}
