//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

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

//$Procedure ZZSEGBOX (Bounding box for DSK segment volume element)
pub fn ZZSEGBOX(
    DSKDSC: &[f64],
    BOXCTR: &mut [f64],
    MAXR: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DSKDSC = DummyArray::new(DSKDSC, 1..);
    let mut BOXCTR = DummyArrayMut::new(BOXCTR, 1..=3);
    let mut L1: f64 = 0.0;
    let mut L2: f64 = 0.0;
    let mut L3: f64 = 0.0;
    let mut CORSYS: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZSEGBOX", ctx)?;

    CORSYS = intrinsics::IDNINT(DSKDSC[SYSIDX]);

    if (CORSYS == LATSYS) {
        ZZLATBOX(
            DSKDSC.subarray(MN1IDX),
            BOXCTR.as_slice_mut(),
            &mut L1,
            &mut L2,
            &mut L3,
            MAXR,
            ctx,
        )?;
    } else if (CORSYS == RECSYS) {
        ZZRECBOX(
            DSKDSC.subarray(MN1IDX),
            BOXCTR.as_slice_mut(),
            &mut L1,
            &mut L2,
            &mut L3,
            MAXR,
            ctx,
        )?;
    } else if (CORSYS == PDTSYS) {
        ZZPDTBOX(
            DSKDSC.subarray(MN1IDX),
            DSKDSC.subarray(PARIDX),
            BOXCTR.as_slice_mut(),
            &mut L1,
            &mut L2,
            &mut L3,
            MAXR,
            ctx,
        )?;
    } else {
        SETMSG(b"Coordinate system # is not supported.", ctx);
        ERRINT(b"#", CORSYS, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"ZZSEGBOX", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZSEGBOX", ctx)?;
    Ok(())
}
