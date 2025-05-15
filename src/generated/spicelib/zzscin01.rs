//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NDELIM: i32 = 5;
const DELIMS: &[u8; NDELIM as usize] = &fstr::extend_const::<{ NDELIM as usize }>(b".:-, ");
const TDB: i32 = 1;
const TDT: i32 = 2;
const MXPART: i32 = 9999;
const MXCOEF: i32 = 100000;
const MXNFLD: i32 = 10;
const DPLEN: i32 = 30;
const IXNFLD: i32 = 1;
const IXDLIM: i32 = (IXNFLD + 1);
const IXTSYS: i32 = (IXDLIM + 1);
const IXNCOF: i32 = (IXTSYS + 1);
const IXNPRT: i32 = (IXNCOF + 1);
const IXBCOF: i32 = (IXNPRT + 1);
const IXBSTR: i32 = (IXBCOF + 1);
const IXBEND: i32 = (IXBSTR + 1);
const IXBMOD: i32 = (IXBEND + 1);
const IXBOFF: i32 = (IXBMOD + 1);
const NIVALS: i32 = IXBOFF;
const MXNCLK: i32 = 100;
const DBUFSZ: i32 = (((3 * MXCOEF) + (2 * MXPART)) + (2 * MXNFLD));
const IBUFSZ: i32 = (NIVALS * MXNCLK);
const LBSNGL: i32 = -5;
const CPLSIZ: i32 = ((MXNCLK - LBSNGL) + 1);

//$Procedure ZZSCIN01 ( Private, SCLK database initialization, type 01 )
pub fn ZZSCIN01(
    HDSCLK: &mut [i32],
    SCPOOL: &mut [i32],
    CLKLST: &mut [i32],
    DPFREE: &mut i32,
    IFREE: &mut i32,
    PRVSC: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut HDSCLK = DummyArrayMut::new(HDSCLK, 1..);
    let mut SCPOOL = DummyArrayMut::new(SCPOOL, LBSNGL..);
    let mut CLKLST = DummyArrayMut::new(CLKLST, 1..);

    //
    // This routine must execute when SPICE error handling code is in
    // the process of responding to an error, so this routine doesn't
    // participate in SPICE error handling or tracing. The routines
    // called here are error-free.
    //
    // Initialize SC01 data structures.
    //
    // ZZHSIINI won't signal an error as long as the parameter MXNCLK
    // is positive, so we don't check FAILED here.
    //
    ZZHSIINI(MXNCLK, HDSCLK.as_slice_mut(), SCPOOL.as_slice_mut(), ctx)?;
    CLEARI(MXNCLK, CLKLST.as_slice_mut());

    *DPFREE = 1;
    *IFREE = 1;
    *PRVSC = 0;

    Ok(())
}
