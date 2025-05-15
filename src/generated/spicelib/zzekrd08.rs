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

//$Procedure   ZZEKRD08 ( EK, read class 8 column entry )
pub fn ZZEKRD08(
    HANDLE: i32,
    SEGDSC: &[i32],
    COLDSC: &[i32],
    RECPTR: i32,
    DVAL: &mut f64,
    ISNULL: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let SEGDSC = DummyArray::new(SEGDSC, 1..=SDSCSZ);
    let COLDSC = DummyArray::new(COLDSC, 1..=CDSCSZ);
    let mut CFLAG = [b' '; 1 as usize];
    let mut ADDRSS: i32 = 0;
    let mut COLIDX: i32 = 0;
    let mut DATBAS: i32 = 0;
    let mut MDAT = StackArray::<i32, 2>::new(1..=MDSZ08);
    let mut METLOC: i32 = 0;
    let mut NCOLS: i32 = 0;
    let mut NFLBAS: i32 = 0;
    let mut OFFSET: i32 = 0;
    let mut Q: i32 = 0;
    let mut R: i32 = 0;
    let mut RECNO: i32 = 0;
    let mut NULLOK: bool = false;

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
    METLOC = COLDSC[METIDX];
    NULLOK = (COLDSC[NFLIDX] == ITRUE);

    if ((COLIDX < 1) || (COLIDX > NCOLS)) {
        RECNO = ZZEKRP2N(HANDLE, SEGDSC[SNOIDX], RECPTR, ctx)?;

        CHKIN(b"ZZEKRD08", ctx)?;
        SETMSG(
            b"Column index = #; valid range is 1:#.SEGNO = #; RECNO = #; EK = #",
            ctx,
        );
        ERRINT(b"#", COLIDX, ctx);
        ERRINT(b"#", NCOLS, ctx);
        ERRINT(b"#", SEGDSC[SNOIDX], ctx);
        ERRINT(b"#", RECNO, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"ZZEKRD08", ctx)?;
        return Ok(());
    }

    //
    // Read the metadata block.  There are two items in the block:
    //
    //    1) The base address of the first page of the data
    //    2) The base address of the null flag array, if nulls are
    //       permitted.
    //
    DASRDI(
        HANDLE,
        (METLOC + 1),
        (METLOC + MDSZ08),
        MDAT.as_slice_mut(),
        ctx,
    )?;

    DATBAS = MDAT[DBIX08];
    NFLBAS = MDAT[NFIX08];

    //
    // If null values are permitted, the first step is to get
    // the null flag for the value of interest.  Compute the
    // address of this flag.
    //
    // There are CPSIZE null flags per page, and each page has size
    // PGSIZC.  The null flags start at the beginning of the page.
    //
    if NULLOK {
        Q = ((RECPTR - 1) / CPSIZE);
        R = (RECPTR - (Q * CPSIZE));
        OFFSET = (R + (Q * PGSIZC));
        ADDRSS = (NFLBAS + OFFSET);

        DASRDC(
            HANDLE,
            ADDRSS,
            ADDRSS,
            1,
            1,
            CharArrayMut::from_mut(&mut CFLAG),
            ctx,
        )?;

        *ISNULL = fstr::eq(&CFLAG, CTRUE);

        if *ISNULL {
            return Ok(());
        }
    }

    //
    // If we're still here, we'll read the data value.
    //
    *ISNULL = false;

    //
    // The address calculation for the value is analogous to that
    // for the null flag.
    //
    Q = ((RECPTR - 1) / DPSIZE);
    R = (RECPTR - (Q * DPSIZE));
    OFFSET = (R + (Q * PGSIZD));
    ADDRSS = (DATBAS + OFFSET);

    DASRDD(HANDLE, ADDRSS, ADDRSS, std::slice::from_mut(DVAL), ctx)?;

    Ok(())
}
