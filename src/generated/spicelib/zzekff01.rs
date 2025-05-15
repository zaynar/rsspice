//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

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
const MXCLSG: i32 = 100;
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
const BUFSIZ: i32 = 100;

//$Procedure      ZZEKFF01 ( EK, finish fast load, segment type 1 )
pub fn ZZEKFF01(
    HANDLE: i32,
    SEGNO: i32,
    RCPTRS: &[i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let RCPTRS = DummyArray::new(RCPTRS, 1..);
    let mut ADDRSS: i32 = 0;
    let mut ADRBUF = StackArray::<i32, 100>::new(1..=BUFSIZ);
    let mut BASE: i32 = 0;
    let mut COLIDX: i32 = 0;
    let mut COLORD = StackArray::<i32, 100>::new(1..=MXCLSG);
    let mut IPAGE = StackArray::<i32, 256>::new(1..=PGSIZI);
    let mut J: i32 = 0;
    let mut LOC: i32 = 0;
    let mut MBASE: i32 = 0;
    let mut NCOLS: i32 = 0;
    let mut NPAGE: i32 = 0;
    let mut NR: i32 = 0;
    let mut NROWS: i32 = 0;
    let mut NRP: i32 = 0;
    let mut P: i32 = 0;
    let mut PAGLOC: i32 = 0;
    let mut PBASE: i32 = 0;
    let mut RECNO: i32 = 0;
    let mut REMAIN: i32 = 0;
    let mut RPSIZE: i32 = 0;
    let mut SEGDSC = StackArray::<i32, 24>::new(1..=SDSCSZ);
    let mut STKBAS: i32 = 0;
    let mut STKHAN: i32 = 0;
    let mut STKSEG: i32 = 0;
    let mut TREE: i32 = 0;

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
    }

    CHKIN(b"ZZEKFF01", ctx)?;

    //
    // Initialize all arrays.
    //
    CLEARI(BUFSIZ, ADRBUF.as_slice_mut());
    CLEARI(MXCLSG, COLORD.as_slice_mut());
    CLEARI(PGSIZI, IPAGE.as_slice_mut());
    CLEARI(SDSCSZ, SEGDSC.as_slice_mut());

    //
    // Dig the handle and segment number out of the EK stack.  If the
    // stacked values don't match the inputs HANDLE and SEGNO, we've
    // got trouble.
    //
    ZZEKSRD(1, 1, std::slice::from_mut(&mut STKHAN), ctx)?;
    ZZEKSRD(2, 2, std::slice::from_mut(&mut STKSEG), ctx)?;

    if ((STKHAN != HANDLE) || (STKSEG != SEGNO)) {
        SETMSG(b"Attempt to finish fast load of wrong segment.  Input segment number is #; stacked segment number is #.  Input handle is #; stacked handle is #.", ctx);
        ERRINT(b"#", SEGNO, ctx);
        ERRINT(b"#", STKSEG, ctx);
        ERRINT(b"#", HANDLE, ctx);
        ERRINT(b"#", STKHAN, ctx);
        SIGERR(b"SPICE(WRONGSEGMENT)", ctx)?;
        CHKOUT(b"ZZEKFF01", ctx)?;
        return Ok(());
    }

    //
    // Look up the segment descriptor for the indicated segment.  Find
    // out how many rows and columns the segment contains.
    //
    ZZEKMLOC(HANDLE, SEGNO, &mut P, &mut MBASE, ctx)?;
    DASRDI(
        HANDLE,
        (MBASE + 1),
        (MBASE + SDSCSZ),
        SEGDSC.as_slice_mut(),
        ctx,
    )?;

    NROWS = SEGDSC[NRIDX];
    NCOLS = SEGDSC[NCIDX];

    //
    // Determine the order in which the columns were added.  The order
    // may differ from that in which the columns were declared.  The
    // ordinal position of each column is stored on the stack right
    // before its address data.  COLORD will map ordinal positions given
    // by a column declaration to ordinal positions on the stack.
    //
    //
    for I in 1..=NCOLS {
        LOC = (((I - 1) * (NROWS + 1)) + 3);

        ZZEKSRD(LOC, LOC, std::slice::from_mut(&mut COLIDX), ctx)?;

        COLORD[COLIDX] = I;
    }

    //
    // We'll need to create a record pointer structure for each row
    // in the segment.  We compute the number of record pointers that
    // can fit on one page.  We also compute the number of pages we'll
    // need to hold the pointers.
    //
    RPSIZE = (DPTBAS + NCOLS);
    NRP = (IPSIZE / RPSIZE);
    NPAGE = (((NROWS + NRP) - 1) / NRP);

    //
    // We'll write out record pointers a pageful at a time.  Each
    // record pointer is initialized to indicate that the record is
    // old, and that there is no corresponding modified record.
    //
    REMAIN = NROWS;
    RECNO = 0;

    for I in 1..=NPAGE {
        //
        // Get the base address of the current page.  The address
        // can be derived from the address of the first record pointer
        // structure on the page.
        //
        ADDRSS = (RCPTRS[(RECNO + 1)] + 1);

        ZZEKPGPG(INT, ADDRSS, &mut P, &mut PBASE, ctx)?;

        CLEARI(IPSIZE, IPAGE.as_slice_mut());

        //
        // NR is the number of record pointers we'll write to this page.
        //
        NR = intrinsics::MIN0(&[NRP, REMAIN]);

        {
            let m1__: i32 = 1;
            let m2__: i32 = NR;
            let m3__: i32 = 1;
            J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                //
                // Initialize the modified record pointer and status for
                // each record pointer on the page.
                //
                BASE = ((J - 1) * RPSIZE);
                IPAGE[(BASE + STAIDX)] = OLD;
                IPAGE[(BASE + RCPIDX)] = UNINIT;

                J += m3__;
            }
        }

        //
        // For each column, take NR addresses off the stack and
        // write them into the page.
        //
        for COL in 1..=NCOLS {
            //
            // The stack starts out with the target file handle and
            // segment number.  Next comes the data for each column.
            // Each column is identified by its ordinal position.  The
            // addresses for the data of each column follow.  The addresses
            // for each column are stored contiguously.
            //
            J = COLORD[COL];
            STKBAS = (((J - 1) * (NROWS + 1)) + 3);
            LOC = (STKBAS + RECNO);

            ZZEKSRD((LOC + 1), (LOC + NR), ADRBUF.as_slice_mut(), ctx)?;

            for ROW in 1..=NR {
                BASE = ((ROW - 1) * RPSIZE);
                PAGLOC = ((BASE + DPTBAS) + COL);
                IPAGE[PAGLOC] = ADRBUF[ROW];
            }
        }

        //
        // Write out the initialized pointer page.
        //
        ZZEKPGWI(HANDLE, P, IPAGE.as_slice(), ctx)?;

        RECNO = (RECNO + NR);
        REMAIN = (REMAIN - NR);
    }

    //
    // Create the record pointer tree for this segment.
    //
    ZZEKTRIT(HANDLE, &mut TREE, ctx)?;
    ZZEKTR1S(HANDLE, TREE, NROWS, RCPTRS.as_slice(), ctx)?;

    //
    // Update the record tree pointer and row count in the segment
    // descriptor.  Set the records of the last DAS words in use
    // to their maximum values, to ensure allocation of new pages
    // if further writes are done.
    //
    ZZEKMLOC(HANDLE, SEGNO, &mut P, &mut BASE, ctx)?;
    DASUDI(HANDLE, (BASE + RTIDX), (BASE + RTIDX), &[TREE], ctx)?;
    DASUDI(HANDLE, (BASE + NRIDX), (BASE + NRIDX), &[NROWS], ctx)?;
    DASUDI(HANDLE, (BASE + LCWIDX), (BASE + LCWIDX), &[CPSIZE], ctx)?;
    DASUDI(HANDLE, (BASE + LDWIDX), (BASE + LDWIDX), &[DPSIZE], ctx)?;
    DASUDI(HANDLE, (BASE + LIWIDX), (BASE + LIWIDX), &[IPSIZE], ctx)?;

    CHKOUT(b"ZZEKFF01", ctx)?;
    Ok(())
}
