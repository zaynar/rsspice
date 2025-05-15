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

//$Procedure      ZZEKSRCP ( EK, set record companion pointer )
pub fn ZZEKSRCP(HANDLE: i32, RECPTR: i32, RECNO: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut LOC: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //
    //
    // Is this file handle valid--is the file open for paged write
    // access?  Signal an error if not.
    //
    ZZEKPGCH(HANDLE, b"WRITE", ctx)?;

    if FAILED(ctx) {
        return Ok(());
    }

    //
    // Compute the companion pointer and set the pointer.
    //
    LOC = (RECPTR + RCPIDX);

    DASUDI(HANDLE, LOC, LOC, &[RECNO], ctx)?;

    Ok(())
}
