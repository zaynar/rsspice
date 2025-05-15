//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const DBIX07: i32 = 1;
const NFIX07: i32 = 2;
const MDSZ07: i32 = 2;
const DBIX08: i32 = 1;
const NFIX08: i32 = 2;
const MDSZ08: i32 = 2;
const DBIX09: i32 = 1;
const NFIX09: i32 = 2;
const MDSZ09: i32 = 2;
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

//$Procedure ZZEKIF02 ( EK, initialize type 2 segment for fast load )
pub fn ZZEKIF02(HANDLE: i32, SEGNO: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut BASE: i32 = 0;
    let mut CLASS: i32 = 0;
    let mut DSCBAS: i32 = 0;
    let mut MBASE: i32 = 0;
    let mut NCOLS: i32 = 0;
    let mut NROWS: i32 = 0;
    let mut OFFSET: i32 = 0;
    let mut P: i32 = 0;
    let mut PAGE = StackArray::<i32, 256>::new(1..=PGSIZI);
    let mut SEGDSC = StackArray::<i32, 24>::new(1..=SDSCSZ);

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
        CHKIN(b"ZZEKIF02", ctx)?;
    }

    //
    // Read in the segment descriptor.
    //
    ZZEKMLOC(HANDLE, SEGNO, PAGE.first_mut(), &mut MBASE, ctx)?;
    DASRDI(
        HANDLE,
        (MBASE + 1),
        (MBASE + SDSCSZ),
        SEGDSC.as_slice_mut(),
        ctx,
    )?;

    NCOLS = SEGDSC[NCIDX];
    NROWS = SEGDSC[NRIDX];

    //
    // Allocate space for column metadata.  We assume that one page
    // of IPSIZE integers is enough room.
    //
    ZZEKAPS(
        HANDLE,
        SEGDSC.as_slice(),
        INT,
        false,
        &mut P,
        &mut BASE,
        ctx,
    )?;

    OFFSET = BASE;

    for I in 1..=NCOLS {
        //
        // Read the class from the descriptor of the Ith column directly
        // from the file.  We'll need the descriptor's address in order to
        // update the descriptor in the file.
        //
        DSCBAS = ((MBASE + CDOFF) + ((I - 1) * CDSCSZ));

        DASRDI(
            HANDLE,
            (DSCBAS + CLSIDX),
            (DSCBAS + CLSIDX),
            std::slice::from_mut(&mut CLASS),
            ctx,
        )?;

        //
        // Update the file.  Set the column descriptor's metadata pointer
        // to the base address of the metadata area.
        //
        DASUDI(HANDLE, (DSCBAS + METIDX), (DSCBAS + METIDX), &[OFFSET], ctx)?;

        //
        // Increment the metadata offset by the size of the metadata
        // for the current column.  The classes of interest range from
        // 7 to 9.
        //
        if (CLASS == 7) {
            OFFSET = (OFFSET + MDSZ07);
        } else if (CLASS == 8) {
            OFFSET = (OFFSET + MDSZ08);
        } else if (CLASS == 9) {
            OFFSET = (OFFSET + MDSZ09);
        } else {
            SETMSG(b"Class # is not supported.", ctx);
            ERRINT(b"#", CLASS, ctx);
            SIGERR(b"SPICE(NOCLASS)", ctx)?;
            CHKOUT(b"ZZEKIF02", ctx)?;
            return Ok(());
        }
    }

    CHKOUT(b"ZZEKIF02", ctx)?;
    Ok(())
}
