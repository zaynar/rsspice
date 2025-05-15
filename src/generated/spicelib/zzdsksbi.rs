//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const VERIDX: i32 = 1;
const LLBIDX: i32 = (VERIDX + 1);
const LLEIDX: i32 = (LLBIDX + 1);
const NULPTR: i32 = -1;
const BWDIDX: i32 = 1;
const FWDIDX: i32 = (BWDIDX + 1);
const IBSIDX: i32 = (FWDIDX + 1);
const ISZIDX: i32 = (IBSIDX + 1);
const DBSIDX: i32 = (ISZIDX + 1);
const DSZIDX: i32 = (DBSIDX + 1);
const CBSIDX: i32 = (DSZIDX + 1);
const CSZIDX: i32 = (CBSIDX + 1);
const DLADSZ: i32 = CSZIDX;
const FMTVER: i32 = 1000000;
const NCHREC: i32 = 1024;
const SRFIDX: i32 = 1;
const CTRIDX: i32 = (SRFIDX + 1);
const CLSIDX: i32 = (CTRIDX + 1);
const TYPIDX: i32 = (CLSIDX + 1);
const FRMIDX: i32 = (TYPIDX + 1);
const SYSIDX: i32 = (FRMIDX + 1);
const PARIDX: i32 = (SYSIDX + 1);
const NSYPAR: i32 = 10;
const MN1IDX: i32 = (PARIDX + NSYPAR);
const MX1IDX: i32 = (MN1IDX + 1);
const MN2IDX: i32 = (MX1IDX + 1);
const MX2IDX: i32 = (MN2IDX + 1);
const MN3IDX: i32 = (MX2IDX + 1);
const MX3IDX: i32 = (MN3IDX + 1);
const BTMIDX: i32 = (MX3IDX + 1);
const ETMIDX: i32 = (BTMIDX + 1);
const DSKDSZ: i32 = ETMIDX;
const SVFCLS: i32 = 1;
const GENCLS: i32 = 2;
const LATSYS: i32 = 1;
const CYLSYS: i32 = 2;
const RECSYS: i32 = 3;
const PDTSYS: i32 = 4;

//$Procedure ZZDSKSBI ( DSK, initialize API segment buffer )
pub fn ZZDSKSBI(
    MAXBOD: i32,
    STSIZE: i32,
    BTBODY: &mut [i32],
    BTNBOD: &mut i32,
    BTSEGP: &mut [i32],
    BTSTSZ: &mut [i32],
    STHAN: &mut [i32],
    STDSCR: &mut [f64],
    STDLAD: &mut [i32],
    STFREE: &mut i32,
    STOFF: &mut [f64],
    STCTR: &mut [f64],
    STRAD: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut BTBODY = DummyArrayMut::new(BTBODY, 1..);
    let mut BTSEGP = DummyArrayMut::new(BTSEGP, 1..);
    let mut BTSTSZ = DummyArrayMut::new(BTSTSZ, 1..);
    let mut STHAN = DummyArrayMut::new(STHAN, 1..);
    let mut STDSCR = DummyArrayMut2D::new(STDSCR, 1..=DSKDSZ, 1..);
    let mut STDLAD = DummyArrayMut2D::new(STDLAD, 1..=DLADSZ, 1..);
    let mut STOFF = DummyArrayMut2D::new(STOFF, 1..=3, 1..);
    let mut STCTR = DummyArrayMut2D::new(STCTR, 1..=3, 1..);
    let mut STRAD = DummyArrayMut::new(STRAD, 1..);

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZDSKSBI", ctx)?;

    //
    // Clear the body table.
    //
    *BTNBOD = 0;

    for I in 1..=MAXBOD {
        BTBODY[I] = 0;
        BTSEGP[I] = 0;
        BTSTSZ[I] = 0;
    }

    //
    // Clear the segment table.
    //
    for I in 1..=STSIZE {
        STHAN[I] = 0;

        CLEARD(DSKDSZ, STDSCR.subarray_mut([1, I]));
        CLEARI(DLADSZ, STDLAD.subarray_mut([1, I]));

        CLEARD(3, STOFF.subarray_mut([1, I]));
        CLEARD(3, STCTR.subarray_mut([1, I]));

        STRAD[I] = 0.0;
    }

    *STFREE = 1;

    CHKOUT(b"ZZDSKSBI", ctx)?;
    Ok(())
}
