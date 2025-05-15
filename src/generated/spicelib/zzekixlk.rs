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

//$Procedure ZZEKIXLK ( EK, look up record pointer in EK index )
pub fn ZZEKIXLK(
    HANDLE: i32,
    COLDSC: &[i32],
    KEY: i32,
    RECPTR: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let COLDSC = DummyArray::new(COLDSC, 1..=CDSCSZ);
    let mut ADDRSS: i32 = 0;
    let mut BASE: i32 = 0;
    let mut ITYPE: i32 = 0;
    let mut Q: i32 = 0;
    let mut R: i32 = 0;
    let mut TREE: i32 = 0;

    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //
    ITYPE = COLDSC[IXTIDX];

    if (ITYPE == 1) {
        //
        // For type 1 indexes, the index pointer is the root node of
        // a B*-tree.  Just use the tree look up routine.
        //
        TREE = COLDSC[IXPIDX];

        ZZEKTRDP(HANDLE, TREE, KEY, RECPTR, ctx)?;
    } else if (ITYPE == 2) {
        //
        // For type 2 indexes, the index pointer is the base address
        // of the index.  We must compute the offset from this base to
        // the index element having ordinal position KEY.
        //
        BASE = COLDSC[IXPIDX];
        Q = ((KEY - 1) / IPSIZE);
        R = (KEY - (Q * IPSIZE));
        ADDRSS = ((BASE + (Q * PGSIZI)) + R);

        DASRDI(HANDLE, ADDRSS, ADDRSS, std::slice::from_mut(RECPTR), ctx)?;
    } else {
        //
        // Sorry, no other types of indexes are supported.
        //
        CHKIN(b"ZZEKIXLK", ctx)?;
        SETMSG(b"The index type # is not supported.", ctx);
        ERRINT(b"#", ITYPE, ctx);
        SIGERR(b"SPICE(INVALIDTYPE)", ctx)?;
        CHKOUT(b"ZZEKIXLK", ctx)?;
        return Ok(());
    }

    Ok(())
}
