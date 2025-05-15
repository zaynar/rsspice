//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const CTRUE: &[u8] = b"T";
const CFALSE: &[u8] = b"F";
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

//$Procedure     ZZEKWPAL ( EK, write paged array, logical )
pub fn ZZEKWPAL(
    HANDLE: i32,
    SEGDSC: &[i32],
    NVALS: i32,
    LVALS: &[bool],
    P: &mut i32,
    BASE: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let SEGDSC = DummyArray::new(SEGDSC, 1..);
    let LVALS = DummyArray::new(LVALS, 1..);
    let mut PAGE = [b' '; PGSIZC as usize];
    let mut NPAGE: i32 = 0;
    let mut TO: i32 = 0;

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
        CHKIN(b"ZZEKWPAL", ctx)?;
    }

    //
    // Decide how many pages are required to hold the array, and
    // allocate that many new, contiguous pages.
    //
    NPAGE = (((NVALS + CPSIZE) - 1) / CPSIZE);

    ZZEKACPS(HANDLE, SEGDSC.as_slice(), CHR, NPAGE, P, BASE, ctx)?;

    //
    // Write the input data out to the target file a page at a time.
    //
    // We'll use FROM to indicate the element of LVALS we're
    // considering and TO to indicate the element of PAGE to write
    // to.
    //
    TO = 1;
    fstr::assign(&mut PAGE, b" ");

    for FROM in 1..=NVALS {
        //
        // The Assignment.
        //
        if LVALS[FROM] {
            fstr::assign(fstr::substr_mut(&mut PAGE, TO..=TO), CTRUE);
        } else {
            fstr::assign(fstr::substr_mut(&mut PAGE, TO..=TO), CFALSE);
        }

        TO = (TO + 1);

        if ((TO > CPSIZE) || (FROM == NVALS)) {
            //
            // Either the current data page is full, or we've buffered
            // the last of the available data.  It's time to write out the
            // current page.
            //
            // Write out the data page.
            //
            ZZEKPGWC(HANDLE, *P, &PAGE, ctx)?;

            //
            // Set the link count.
            //
            ZZEKSLNK(HANDLE, CHR, *P, (TO - 1), ctx)?;

            //
            // Next page.
            //
            *P = (*P + 1);
            TO = 1;
        }
    }

    CHKOUT(b"ZZEKWPAL", ctx)?;
    Ok(())
}
