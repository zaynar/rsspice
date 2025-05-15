//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const XFRACT: f64 = 0.0000000001;
const KEYXFR: i32 = 1;
const SGREED: f64 = 0.00000001;
const KEYSGR: i32 = (KEYXFR + 1);
const SGPADM: f64 = 0.0000000001;
const KEYSPM: i32 = (KEYSGR + 1);
const PTMEMM: f64 = 0.0000001;
const KEYPTM: i32 = (KEYSPM + 1);
const ANGMRG: f64 = 0.000000000001;
const KEYAMG: i32 = (KEYPTM + 1);
const LONALI: f64 = 0.000000000001;
const KEYLAL: i32 = (KEYAMG + 1);
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

//$Procedure ZZINVELT ( DSK, in volume element? )
pub fn ZZINVELT(
    P: &[f64],
    CORSYS: i32,
    CORPAR: &[f64],
    BOUNDS: &[f64],
    MARGIN: f64,
    EXCLUD: i32,
    INSIDE: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let P = DummyArray::new(P, 1..=3);
    let CORPAR = DummyArray::new(CORPAR, 1..);
    let BOUNDS = DummyArray2D::new(BOUNDS, 1..=2, 1..=3);

    //
    // SPICELIB functions
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZINVELT", ctx)?;

    if (MARGIN < 0.0) {
        SETMSG(b"Margin must be non-negative but was #.", ctx);
        ERRDP(b"#", MARGIN, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZINVELT", ctx)?;
        return Ok(());
    }

    //
    // Delegate the job to one of the coordinate system-specific
    // routines.
    //
    if (CORSYS == LATSYS) {
        ZZINLAT(P.as_slice(), BOUNDS.as_slice(), MARGIN, EXCLUD, INSIDE, ctx)?;
    } else if (CORSYS == PDTSYS) {
        ZZINPDT(
            P.as_slice(),
            BOUNDS.as_slice(),
            CORPAR.as_slice(),
            MARGIN,
            EXCLUD,
            INSIDE,
            ctx,
        )?;
    } else if (CORSYS == RECSYS) {
        ZZINREC(P.as_slice(), BOUNDS.as_slice(), MARGIN, EXCLUD, INSIDE, ctx)?;
    } else {
        SETMSG(b"Coordinate system code # was not recognized.", ctx);
        ERRINT(b"#", CORSYS, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"ZZINVELT", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZINVELT", ctx)?;
    Ok(())
}
