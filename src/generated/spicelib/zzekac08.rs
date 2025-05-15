//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const CTRUE: &[u8] = b"T";
const CFALSE: &[u8] = b"F";
const DBIX08: i32 = 1;
const NFIX08: i32 = 2;
const MDSZ08: i32 = 2;
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

//$Procedure     ZZEKAC08 ( EK, add class 8 column to segment )
pub fn ZZEKAC08(
    HANDLE: i32,
    SEGDSC: &[i32],
    COLDSC: &[i32],
    DVALS: &[f64],
    NLFLGS: &[bool],
    WKINDX: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let SEGDSC = DummyArray::new(SEGDSC, 1..);
    let COLDSC = DummyArray::new(COLDSC, 1..);
    let DVALS = DummyArray::new(DVALS, 1..);
    let NLFLGS = DummyArray::new(NLFLGS, 1..);
    let mut WKINDX = DummyArrayMut::new(WKINDX, 1..);
    let mut COLUMN = [b' '; CNAMSZ as usize];
    let mut PAGE = StackArray::<f64, 128>::new(1..=PGSIZD);
    let mut CLASS: i32 = 0;
    let mut CMBASE: i32 = 0;
    let mut COLIDX: i32 = 0;
    let mut DATBAS: i32 = 0;
    let mut DSCBAS: i32 = 0;
    let mut IDXBAS: i32 = 0;
    let mut IDXPAG: i32 = 0;
    let mut IDXTYP: i32 = 0;
    let mut P: i32 = 0;
    let mut MBASE: i32 = 0;
    let mut NFLBAS: i32 = 0;
    let mut NFLPAG: i32 = 0;
    let mut NPAGE: i32 = 0;
    let mut NROWS: i32 = 0;
    let mut NULPTR: i32 = 0;
    let mut TO: i32 = 0;
    let mut INDEXD: bool = false;
    let mut NULLOK: bool = false;

    //
    // SPICELIB functions
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
        CHKIN(b"ZZEKAC08", ctx)?;
    }

    //
    // Grab the column's attributes.
    //
    CLASS = COLDSC[CLSIDX];
    IDXTYP = COLDSC[IXTIDX];
    NULPTR = COLDSC[NFLIDX];
    COLIDX = COLDSC[ORDIDX];

    NULLOK = (NULPTR != IFALSE);
    INDEXD = (IDXTYP != IFALSE);

    //
    // This column had better be class 8.
    //
    if (CLASS != 8) {
        ZZEKCNAM(HANDLE, COLDSC.as_slice(), &mut COLUMN, ctx)?;

        SETMSG(
            b"Column class code # found in descriptor for column #.  Class should be 8.",
            ctx,
        );
        ERRINT(b"#", CLASS, ctx);
        ERRCH(b"#", &COLUMN, ctx);
        SIGERR(b"SPICE(NOCLASS)", ctx)?;
        CHKOUT(b"ZZEKAC08", ctx)?;
        return Ok(());
    }

    //
    // Find the number of rows in the segment.
    //
    NROWS = SEGDSC[NRIDX];

    //
    // Decide how many pages are required to hold the array, and
    // allocate that many new, contiguous pages.
    //
    NPAGE = (((NROWS + DPSIZE) - 1) / DPSIZE);

    ZZEKACPS(
        HANDLE,
        SEGDSC.as_slice(),
        DP,
        NPAGE,
        &mut P,
        &mut DATBAS,
        ctx,
    )?;

    //
    // We'll use FROM to indicate the element of DVALS we're
    // considering and TO to indicate the element of PAGE to write
    // to.
    //
    TO = 1;
    CLEARD(PGSIZD, PAGE.as_slice_mut());

    for FROM in 1..=NROWS {
        //
        // The Assignment.
        //
        if (!NULLOK || !NLFLGS[FROM]) {
            //
            // The current item is non-null.
            //
            PAGE[TO] = DVALS[FROM];
        }

        TO = (TO + 1);

        if ((TO > DPSIZE) || (FROM == NROWS)) {
            //
            // Either the current data page is full, or we've buffered
            // the last of the available data.  It's time to write out the
            // current page.  First set the link count.
            //
            PAGE[DLCIDX] = (TO - 1) as f64;

            //
            // Write out the data page.
            //
            ZZEKPGWD(HANDLE, P, PAGE.as_slice(), ctx)?;

            //
            // Next page.
            //
            P = (P + 1);
            TO = 1;
        }
    }

    //
    // Update the column's metadata area to point to the data array.
    //
    CMBASE = COLDSC[METIDX];

    DASUDI(HANDLE, (CMBASE + DBIX08), (CMBASE + DBIX08), &[DATBAS], ctx)?;

    //
    // If the column is supposed to have an index, now is the time to
    // build that index.  Type 2 indexes are just order vectors.
    //
    if INDEXD {
        //
        // Compute the order vector.
        //
        ZZEKORDD(
            DVALS.as_slice(),
            NULLOK,
            NLFLGS.as_slice(),
            NROWS,
            WKINDX.as_slice_mut(),
        );

        //
        // Write out the index.
        //
        ZZEKWPAI(
            HANDLE,
            SEGDSC.as_slice(),
            NROWS,
            WKINDX.as_slice(),
            &mut IDXPAG,
            &mut IDXBAS,
            ctx,
        )?;

        //
        // Update the column's metadata to point to the index.  The
        // pointer indicates base address of the index.  Also set the
        // index type in the column descriptor.
        //
        MBASE = SEGDSC[IMDIDX];
        DSCBAS = ((MBASE + SDSCSZ) + ((COLIDX - 1) * CDSCSZ));

        DASUDI(HANDLE, (DSCBAS + IXPIDX), (DSCBAS + IXPIDX), &[IDXBAS], ctx)?;
        DASUDI(HANDLE, (DSCBAS + IXTIDX), (DSCBAS + IXTIDX), &[2], ctx)?;
    }

    if NULLOK {
        //
        // Nulls are allowed.  Write out the null flag array.
        //
        ZZEKWPAL(
            HANDLE,
            SEGDSC.as_slice(),
            NROWS,
            NLFLGS.as_slice(),
            &mut NFLPAG,
            &mut NFLBAS,
            ctx,
        )?;

        //
        // Update the column's metadata area to point to the null flag
        // array.
        //
        DASUDI(HANDLE, (CMBASE + NFIX08), (CMBASE + NFIX08), &[NFLBAS], ctx)?;
    }

    CHKOUT(b"ZZEKAC08", ctx)?;
    Ok(())
}
