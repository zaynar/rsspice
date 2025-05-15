//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

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

//$Procedure      ZZEKGRS ( EK, get record status )
pub fn ZZEKGRS(
    HANDLE: i32,
    RECPTR: i32,
    STATUS: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut LOC: i32 = 0;

    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //
    //
    // Compute the status word location, and read the status.
    //
    LOC = (RECPTR + STAIDX);

    DASRDI(HANDLE, LOC, LOC, std::slice::from_mut(STATUS), ctx)?;

    Ok(())
}
