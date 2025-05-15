//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

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
const ARCHID: i32 = 8;
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;

//$Procedure   ZZEKPGCH ( EK, paging system access check )
pub fn ZZEKPGCH(HANDLE: i32, ACCESS: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ID: i32 = 0;
    let mut LASTC: i32 = 0;
    let mut LASTD: i32 = 0;
    let mut LASTI: i32 = 0;
    let mut NPC: i32 = 0;
    let mut NPD: i32 = 0;
    let mut NPI: i32 = 0;
    let mut TOPC: i32 = 0;
    let mut TOPD: i32 = 0;
    let mut TOPI: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    CHKIN(b"ZZEKPGCH", ctx)?;

    //
    // Check whether the DAS is opened for the specified access method.
    //
    DASSIH(HANDLE, ACCESS, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZEKPGCH", ctx)?;
        return Ok(());
    }

    //
    // Make sure the DAS file is of the right type.
    //
    DASRDI(HANDLE, EPARCH, EPARCH, std::slice::from_mut(&mut ID), ctx)?;

    if (ID != ARCHID) {
        SETMSG(b"File # has architecture #, which is invalid for paged access.  You are using EK software version #.", ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        ERRINT(b"#", ID, ctx);
        ERRINT(b"#", ARCHID, ctx);
        SIGERR(b"SPICE(WRONGARCHITECTURE)", ctx)?;
        CHKOUT(b"ZZEKPGCH", ctx)?;
        return Ok(());
    }

    //
    // Obtain the page counts.  Set the `top' addresses.
    //
    DASRDI(HANDLE, EPNPC, EPNPC, std::slice::from_mut(&mut NPC), ctx)?;
    DASRDI(HANDLE, EPNPD, EPNPD, std::slice::from_mut(&mut NPD), ctx)?;
    DASRDI(HANDLE, EPNPI, EPNPI, std::slice::from_mut(&mut NPI), ctx)?;

    TOPC = (NPC * PGSIZC);
    TOPD = (NPD * PGSIZD);
    TOPI = ((NPI * PGSIZI) + PGBASI);

    //
    // Verify that the last addresses in use are consistent with the
    // `top' addresses known to this system.
    //
    DASLLA(HANDLE, &mut LASTC, &mut LASTD, &mut LASTI, ctx)?;

    if (LASTC > TOPC) {
        SETMSG(b"File # has last char address #; `top\' = #.", ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        ERRINT(b"#", LASTC, ctx);
        ERRINT(b"#", TOPC, ctx);
        SIGERR(b"SPICE(INVALIDFORMAT)", ctx)?;
        CHKOUT(b"ZZEKPGCH", ctx)?;
        return Ok(());
    } else if (LASTD > TOPD) {
        SETMSG(b"File # has last d.p. address #; `top\' = #.", ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        ERRINT(b"#", LASTD, ctx);
        ERRINT(b"#", TOPD, ctx);
        SIGERR(b"SPICE(INVALIDFORMAT)", ctx)?;
        CHKOUT(b"ZZEKPGCH", ctx)?;
        return Ok(());
    } else if (LASTI > TOPI) {
        SETMSG(b"File # has last int. address #; `top\' = #.", ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        ERRINT(b"#", LASTI, ctx);
        ERRINT(b"#", TOPI, ctx);
        SIGERR(b"SPICE(INVALIDFORMAT)", ctx)?;
        CHKOUT(b"ZZEKPGCH", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZEKPGCH", ctx)?;
    Ok(())
}
