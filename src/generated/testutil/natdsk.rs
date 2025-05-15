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
const IXNV: i32 = 1;
const IXNP: i32 = (IXNV + 1);
const IXNVXT: i32 = (IXNP + 1);
const IXVGRX: i32 = (IXNVXT + 1);
const IXCGSC: i32 = (IXVGRX + 3);
const IXVXPS: i32 = (IXCGSC + 1);
const IXVXLS: i32 = (IXVXPS + 1);
const IXVTLS: i32 = (IXVXLS + 1);
const IXPLAT: i32 = (IXVTLS + 1);
const IXDSCR: i32 = 1;
const DSCSZ2: i32 = 24;
const IXVTBD: i32 = (IXDSCR + DSCSZ2);
const IXVXOR: i32 = (IXVTBD + 6);
const IXVXSZ: i32 = (IXVXOR + 3);
const IXVERT: i32 = (IXVXSZ + 1);
const KWNV: i32 = 1;
const KWNP: i32 = (KWNV + 1);
const KWNVXT: i32 = (KWNP + 1);
const KWVGRX: i32 = (KWNVXT + 1);
const KWCGSC: i32 = (KWVGRX + 1);
const KWVXPS: i32 = (KWCGSC + 1);
const KWVXLS: i32 = (KWVXPS + 1);
const KWVTLS: i32 = (KWVXLS + 1);
const KWPLAT: i32 = (KWVTLS + 1);
const KWVXPT: i32 = (KWPLAT + 1);
const KWVXPL: i32 = (KWVXPT + 1);
const KWVTPT: i32 = (KWVXPL + 1);
const KWVTPL: i32 = (KWVTPT + 1);
const KWCGPT: i32 = (KWVTPL + 1);
const KWDSC: i32 = (KWCGPT + 1);
const KWVTBD: i32 = (KWDSC + 1);
const KWVXOR: i32 = (KWVTBD + 1);
const KWVXSZ: i32 = (KWVXOR + 1);
const KWVERT: i32 = (KWVXSZ + 1);
const MAXVRT: i32 = 16000002;
const MAXPLT: i32 = (2 * (MAXVRT - 2));
const MAXNPV: i32 = (((3 * MAXPLT) / 2) + 1);
const MAXVOX: i32 = 100000000;
const MAXCGR: i32 = 100000;
const MAXEDG: i32 = 120;
const SIVGRX: i32 = 1;
const SICGSC: i32 = (SIVGRX + 3);
const SIVXNP: i32 = (SICGSC + 1);
const SIVXNL: i32 = (SIVXNP + 1);
const SIVTNL: i32 = (SIVXNL + 1);
const SICGRD: i32 = (SIVTNL + 1);
const IXIFIX: i32 = (MAXCGR + 7);
const SIVTBD: i32 = 1;
const SIVXOR: i32 = (SIVTBD + 6);
const SIVXSZ: i32 = (SIVXOR + 3);
const IXDFIX: i32 = 10;
const MAXVXP: i32 = (MAXPLT / 2);
const MAXCEL: i32 = 60000000;
const MXNVLS: i32 = (MAXCEL + (MAXVXP / 2));
const SPAISZ: i32 = ((((IXIFIX + MAXVXP) + MXNVLS) + MAXVRT) + MAXNPV);
const ALPHA: i32 = 1000;
const BETA: i32 = 2000;
const FRNMLN: i32 = 32;

//$Procedure   NATDSK (Create a DSK for Nat's solar system)
pub fn NATDSK(
    DSK: &[u8],
    AFRAME: &[u8],
    ANLON: i32,
    ANLAT: i32,
    BFRAME: &[u8],
    BNLON: i32,
    BNLAT: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut ARAD = StackArray::<f64, 3>::new(1..=3);
    let mut BOUNDS = StackArray2D::<f64, 4>::new(1..=2, 1..=2);
    let mut BRAD = StackArray::<f64, 3>::new(1..=3);
    let mut CORPAR = StackArray::<f64, 10>::new(1..=NSYPAR);
    let mut FIRST: f64 = 0.0;
    let mut LAST: f64 = 0.0;
    let mut BODYID: i32 = 0;
    let mut CORSYS: i32 = 0;
    let mut N: i32 = 0;
    let mut SURFID: i32 = 0;
    let mut MAKVTL: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"NATDSK", ctx)?;

    //
    // Delete any existing file of the same name.
    //
    if spicelib::EXISTS(DSK, ctx)? {
        spicelib::DELFIL(DSK, ctx)?;

        if spicelib::FAILED(ctx) {
            spicelib::CHKOUT(b"NATDSK", ctx)?;
            return Ok(());
        }
    }

    //
    // Fetch body radii.
    //
    spicelib::BODVCD(ALPHA, b"RADII", 3, &mut N, ARAD.as_slice_mut(), ctx)?;
    spicelib::BODVCD(BETA, b"RADII", 3, &mut N, BRAD.as_slice_mut(), ctx)?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"NATDSK", ctx)?;
        return Ok(());
    }

    //
    // Create a tessellated plate model segment for body
    // Alpha.
    //
    BODYID = ALPHA;
    SURFID = 1;
    FIRST = -((100 as f64) * spicelib::JYEAR());
    LAST = ((100 as f64) * spicelib::JYEAR());
    CORSYS = LATSYS;

    spicelib::CLEARD(NSYPAR, CORPAR.as_slice_mut());

    //
    // Set longitude and latitude bounds for the segment.
    //
    BOUNDS[[1, 1]] = 0.0;
    BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);

    BOUNDS[[1, 2]] = -spicelib::HALFPI(ctx);
    BOUNDS[[2, 2]] = spicelib::HALFPI(ctx);

    //
    // Don't make a vertex-plate list.
    //
    MAKVTL = false;

    T_ELDSK2(
        BODYID,
        SURFID,
        AFRAME,
        FIRST,
        LAST,
        CORSYS,
        CORPAR.as_slice(),
        BOUNDS.as_slice(),
        ARAD[1],
        ARAD[2],
        ARAD[3],
        ANLON,
        ANLAT,
        MAKVTL,
        DSK,
        ctx,
    )?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"NATDSK", ctx)?;
        return Ok(());
    }

    //
    // Append to the DSK we just created a segment for body Beta.
    //
    BODYID = BETA;
    SURFID = 1;
    FIRST = -((100 as f64) * spicelib::JYEAR());
    LAST = ((100 as f64) * spicelib::JYEAR());
    CORSYS = LATSYS;

    spicelib::CLEARD(NSYPAR, CORPAR.as_slice_mut());

    //
    // Set longitude and latitude bounds for the segment.
    //
    BOUNDS[[1, 1]] = 0.0;
    BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);

    BOUNDS[[1, 2]] = -spicelib::HALFPI(ctx);
    BOUNDS[[2, 2]] = spicelib::HALFPI(ctx);

    //
    // Don't make a vertex-plate list.
    //
    MAKVTL = false;

    T_ELDSK2(
        BODYID,
        SURFID,
        BFRAME,
        FIRST,
        LAST,
        CORSYS,
        CORPAR.as_slice(),
        BOUNDS.as_slice(),
        BRAD[1],
        BRAD[2],
        BRAD[3],
        BNLON,
        BNLAT,
        MAKVTL,
        DSK,
        ctx,
    )?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"NATDSK", ctx)?;
        return Ok(());
    }

    spicelib::CHKOUT(b"NATDSK", ctx)?;
    Ok(())
}
