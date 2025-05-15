//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXSRF: i32 = 100;
const ELLTRG: i32 = 1;
const DSKTRG: i32 = 2;

struct SaveVars {
    SAVMNR: f64,
    SAVMXR: f64,
    SAVRAD: StackArray<f64, 3>,
    SAVFID: i32,
    SAVNSF: i32,
    SAVSRF: StackArray<i32, 100>,
    SAVTRG: i32,
    SAVTYP: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SAVMNR: f64 = 0.0;
        let mut SAVMXR: f64 = 0.0;
        let mut SAVRAD = StackArray::<f64, 3>::new(1..=3);
        let mut SAVFID: i32 = 0;
        let mut SAVNSF: i32 = 0;
        let mut SAVSRF = StackArray::<i32, 100>::new(1..=MAXSRF);
        let mut SAVTRG: i32 = 0;
        let mut SAVTYP: i32 = 0;

        Self {
            SAVMNR,
            SAVMXR,
            SAVRAD,
            SAVFID,
            SAVNSF,
            SAVSRF,
            SAVTRG,
            SAVTYP,
        }
    }
}

//$Procedure ZZSINUTL ( Utilities for generalized ray intercept )
pub fn ZZSINUTL(
    TRGCDE: i32,
    NSURF: i32,
    SRFLST: &[i32],
    ET: f64,
    FIXFID: i32,
    VERTEX: &[f64],
    RAYDIR: &[f64],
    SPOINT: &[f64],
    FOUND: bool,
    MINRAD: f64,
    MAXRAD: f64,
    PNEAR: &[f64],
    DIST: f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    //
    // Target types:
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZSINUTL", ctx)?;
    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    CHKOUT(b"ZZSINUTL", ctx)?;
    Ok(())
}

//$Procedure ZZSUELIN ( Initialize SINCPT utilities for ellipsoid )
pub fn ZZSUELIN(TRGCDE: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZSUELIN", ctx)?;

    save.SAVTYP = ELLTRG;

    ZZGFTREB(TRGCDE, save.SAVRAD.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZSUELIN", ctx)?;
        return Ok(());
    }

    save.SAVMNR = intrinsics::DMIN1(&[save.SAVRAD[1], save.SAVRAD[2], save.SAVRAD[3]]);
    save.SAVMXR = intrinsics::DMAX1(&[save.SAVRAD[1], save.SAVRAD[2], save.SAVRAD[3]]);

    CHKOUT(b"ZZSUELIN", ctx)?;
    Ok(())
}

//$Procedure ZZSUDSKI ( DSK, initialize SINCPT utilities for DSK target )
pub fn ZZSUDSKI(
    TRGCDE: i32,
    NSURF: i32,
    SRFLST: &[i32],
    FIXFID: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let SRFLST = DummyArray::new(SRFLST, 1..);

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZSUDSKI", ctx)?;

    save.SAVTYP = DSKTRG;

    if ((NSURF < 0) || (NSURF > MAXSRF)) {
        SETMSG(b"Surface count must be in the range 0:# but was #.", ctx);
        ERRINT(b"#", MAXSRF, ctx);
        ERRINT(b"#", NSURF, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"ZZSUDSKI", ctx)?;
        return Ok(());
    }

    save.SAVNSF = NSURF;

    MOVEI(SRFLST.as_slice(), save.SAVNSF, save.SAVSRF.as_slice_mut());

    save.SAVFID = FIXFID;
    save.SAVTRG = TRGCDE;

    CLEARD(3, save.SAVRAD.as_slice_mut());

    if FAILED(ctx) {
        CHKOUT(b"ZZSUDSKI", ctx)?;
        return Ok(());
    }

    //
    // Fetch minimum and maximum radius of target body surface.
    //
    ZZDSKSPH(
        TRGCDE,
        save.SAVNSF,
        save.SAVSRF.as_slice(),
        &mut save.SAVMNR,
        &mut save.SAVMXR,
        ctx,
    )?;

    CHKOUT(b"ZZSUDSKI", ctx)?;
    Ok(())
}

//$Procedure ZZRAYSFX ( Callback for ray-surface intercept )
pub fn ZZRAYSFX(
    VERTEX: &[f64],
    RAYDIR: &[f64],
    ET: f64,
    SPOINT: &mut [f64],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let VERTEX = DummyArray::new(VERTEX, 1..=3);
    let RAYDIR = DummyArray::new(RAYDIR, 1..=3);
    let mut SPOINT = DummyArrayMut::new(SPOINT, 1..=3);

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZRAYSFX", ctx)?;

    if (save.SAVTYP == ELLTRG) {
        SURFPT(
            VERTEX.as_slice(),
            RAYDIR.as_slice(),
            save.SAVRAD[1],
            save.SAVRAD[2],
            save.SAVRAD[3],
            SPOINT.as_slice_mut(),
            FOUND,
            ctx,
        )?;
    } else if (save.SAVTYP == DSKTRG) {
        ZZSBFXR(
            save.SAVTRG,
            save.SAVNSF,
            save.SAVSRF.as_slice(),
            ET,
            save.SAVFID,
            VERTEX.as_slice(),
            RAYDIR.as_slice(),
            SPOINT.as_slice_mut(),
            FOUND,
            ctx,
        )?;
    } else {
        SETMSG(b"Surface type code # is not supported. This code branch is not supposed to be reached.", ctx);
        ERRINT(b"#", save.SAVTYP, ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZRAYSFX", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZRAYSFX", ctx)?;
    Ok(())
}

//$Procedure ZZMAXRAD ( Shape model, maximum radius )
pub fn ZZMAXRAD(MAXRAD: &mut f64, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    *MAXRAD = save.SAVMXR;
}

//$Procedure ZZMINRAD ( Shape model, minimum radius )
pub fn ZZMINRAD(MINRAD: &mut f64, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    *MINRAD = save.SAVMNR;
}

//$Procedure ZZRAYNP ( Shape model, callback for ray-surface near point )
pub fn ZZRAYNP(
    VERTEX: &[f64],
    RAYDIR: &[f64],
    ET: f64,
    PNEAR: &mut [f64],
    DIST: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let VERTEX = DummyArray::new(VERTEX, 1..=3);
    let RAYDIR = DummyArray::new(RAYDIR, 1..=3);
    let mut PNEAR = DummyArrayMut::new(PNEAR, 1..=3);

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZRAYNP", ctx)?;

    if (save.SAVTYP == ELLTRG) {
        //
        // Find the nearest point on the ellipsoid's surface to
        // to the ray.
        //
        NPEDLN(
            save.SAVRAD[1],
            save.SAVRAD[2],
            save.SAVRAD[3],
            VERTEX.as_slice(),
            RAYDIR.as_slice(),
            PNEAR.as_slice_mut(),
            DIST,
            ctx,
        )?;
    } else if (save.SAVTYP == DSKTRG) {
        //
        // Find the nearest point on the outer bounding sphere to
        // to the ray.
        //
        NPEDLN(
            save.SAVMXR,
            save.SAVMXR,
            save.SAVMXR,
            VERTEX.as_slice(),
            RAYDIR.as_slice(),
            PNEAR.as_slice_mut(),
            DIST,
            ctx,
        )?;
    } else {
        SETMSG(b"Surface type code # is not supported. This code branch is not supposed to be reached.", ctx);
        ERRINT(b"#", save.SAVTYP, ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZRAYNP", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZRAYNP", ctx)?;
    Ok(())
}
