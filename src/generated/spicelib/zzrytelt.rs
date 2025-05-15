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

//$Procedure ZZRYTELT ( DSK, ray touches coordinate volume element )
pub fn ZZRYTELT(
    VERTEX: &[f64],
    RAYDIR: &[f64],
    DSKDSC: &[f64],
    MARGIN: f64,
    NXPTS: &mut i32,
    XPT: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let VERTEX = DummyArray::new(VERTEX, 1..=3);
    let RAYDIR = DummyArray::new(RAYDIR, 1..=3);
    let DSKDSC = DummyArray::new(DSKDSC, 1..=DSKDSZ);
    let mut XPT = DummyArrayMut::new(XPT, 1..=3);
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

    CHKIN(b"ZZRYTELT", ctx)?;

    CORSYS = intrinsics::IDNINT(DSKDSC[SYSIDX]);

    if (CORSYS == LATSYS) {
        ZZRYTLAT(
            VERTEX.as_slice(),
            RAYDIR.as_slice(),
            DSKDSC.subarray(MN1IDX),
            MARGIN,
            NXPTS,
            XPT.as_slice_mut(),
            ctx,
        )?;
    } else if (CORSYS == RECSYS) {
        ZZRYTREC(
            VERTEX.as_slice(),
            RAYDIR.as_slice(),
            DSKDSC.subarray(MN1IDX),
            MARGIN,
            NXPTS,
            XPT.as_slice_mut(),
            ctx,
        )?;
    } else if (CORSYS == PDTSYS) {
        ZZRYTPDT(
            VERTEX.as_slice(),
            RAYDIR.as_slice(),
            DSKDSC.subarray(MN1IDX),
            DSKDSC.subarray(PARIDX),
            MARGIN,
            NXPTS,
            XPT.as_slice_mut(),
            ctx,
        )?;
    } else {
        SETMSG(b"Coordinate system # is not supported.", ctx);
        ERRINT(b"#", CORSYS, ctx);
        SIGERR(b"SPICE(BADCOORDSYS)", ctx)?;
        CHKOUT(b"ZZRYTELT", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZRYTELT", ctx)?;
    Ok(())
}
