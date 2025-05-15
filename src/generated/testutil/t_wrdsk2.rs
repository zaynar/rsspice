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
const MAXP: i32 = (MAXPLT / 8);
const MAXV: i32 = (MAXVRT / 8);
const WORKSZ: i32 = (2 * MAXP);
const VOXPSZ: i32 = MAXP;
const VOXLSZ: i32 = (4 * MAXP);
const MXIXSZ: i32 = ((IXIFIX + VOXPSZ) + VOXLSZ);
const SMLISZ: i32 = 100100;
const DCLASS: i32 = 2;

struct SaveVars {
    SMLIXD: StackArray<f64, 10>,
    VRTCES: ActualArray2D<f64>,
    PLATES: ActualArray2D<i32>,
    WORK: ActualArray2D<i32>,
    SMLIXI: ActualArray<i32>,
    SPAIXI: ActualArray<i32>,
    SMALL1: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SMLIXD = StackArray::<f64, 10>::new(1..=IXDFIX);
        let mut VRTCES = ActualArray2D::<f64>::new(1..=3, 1..=MAXV);
        let mut PLATES = ActualArray2D::<i32>::new(1..=3, 1..=MAXP);
        let mut WORK = ActualArray2D::<i32>::new(1..=2, 1..=WORKSZ);
        let mut SMLIXI = ActualArray::<i32>::new(1..=SMLISZ);
        let mut SPAIXI = ActualArray::<i32>::new(1..=MXIXSZ);
        let mut SMALL1: bool = false;

        SMALL1 = true;

        Self {
            SMLIXD,
            VRTCES,
            PLATES,
            WORK,
            SMLIXI,
            SPAIXI,
            SMALL1,
        }
    }
}

//$Procedure T_WRDSK2 ( DSK test utilities, type 2 writer umbrella )
pub fn T_WRDSK2(
    BODYID: i32,
    SURFID: i32,
    FRNAME: &[u8],
    FIRST: f64,
    LAST: f64,
    A: f64,
    B: f64,
    C: f64,
    CORSYS: i32,
    CORPAR: &[f64],
    BOUNDS: &[f64],
    NLON: i32,
    NLAT: i32,
    NV: i32,
    NP: i32,
    USRVRT: &[f64],
    USRPLT: &[i32],
    MAKVTL: bool,
    USEPAD: bool,
    NPOLYV: i32,
    NCROSS: i32,
    R: f64,
    RCROSS: f64,
    CENTER: &[f64],
    NORMAL: &[f64],
    DSK: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // EXTERNAL routines
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //
    // Save all large arrays to avoid stack problems.
    // Save spatial index variables for the small DSK writer.
    //

    //
    // Initial values
    //

    spicelib::CHKIN(b"T_WRDSK2", ctx)?;
    spicelib::SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    spicelib::CHKOUT(b"T_WRDSK2", ctx)?;
    Ok(())
}

//$Procedure T_WRTPLT ( DSK test, write caller-supplied plate data )
pub fn T_WRTPLT(
    BODYID: i32,
    SURFID: i32,
    FRNAME: &[u8],
    FIRST: f64,
    LAST: f64,
    CORSYS: i32,
    CORPAR: &[f64],
    BOUNDS: &[f64],
    NV: i32,
    NP: i32,
    USRVRT: &[f64],
    USRPLT: &[i32],
    MAKVTL: bool,
    DSK: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let CORPAR = DummyArray::new(CORPAR, 1..);
    let BOUNDS = DummyArray2D::new(BOUNDS, 1..=2, 1..=2);
    let USRVRT = DummyArray2D::new(USRVRT, 1..=3, 1..);
    let USRPLT = DummyArray2D::new(USRPLT, 1..=3, 1..);
    let mut FINSCL: f64 = 0.0;
    let mut MNCOR3: f64 = 0.0;
    let mut MXCOR3: f64 = 0.0;
    let mut SPAIXD = StackArray::<f64, 10>::new(1..=IXDFIX);
    let mut CORSCL: i32 = 0;
    let mut FRAMID: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut SPXISZ: i32 = 0;
    let mut VLSIZE: i32 = 0;
    let mut VMAPSZ: i32 = 0;
    let mut VPSIZE: i32 = 0;

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"T_WRTPLT", ctx)?;

    //
    // Convert the frame name to an ID code.
    //
    spicelib::NAMFRM(FRNAME, &mut FRAMID, ctx)?;

    if (FRAMID == 0) {
        spicelib::SETMSG(b"Could not convert frame name # to an ID code.", ctx);
        spicelib::ERRCH(b"#", FRNAME, ctx);
        spicelib::SIGERR(b"SPICE(FRAMEIDNOTFOUND)", ctx)?;
        spicelib::CHKOUT(b"T_WRTPLT", ctx)?;
        return Ok(());
    }

    //
    // Create a spatial index for the plate set.
    //
    // Use a heuristic formula for the fine scale.
    //
    FINSCL = intrinsics::DMAX1(&[1.0, (f64::powf(NP as f64, 0.23) / 8 as f64)]);

    //
    // Pick a one-size-fits-all value for the large scale.
    //
    CORSCL = 10;

    //
    // Set the spatial index integer component size.
    //
    if MAKVTL {
        VPSIZE = (VOXPSZ / 2);
        VLSIZE = (VOXLSZ / 2);

        VMAPSZ = ((2 * NV) + (3 * NP));

        SPXISZ = (((IXIFIX + VPSIZE) + VLSIZE) + VMAPSZ);
    } else {
        VPSIZE = VOXPSZ;
        VLSIZE = VOXLSZ;
        SPXISZ = MXIXSZ;
    }

    spicelib::DSKMI2(
        NV,
        USRVRT.as_slice(),
        NP,
        USRPLT.as_slice(),
        FINSCL,
        CORSCL,
        WORKSZ,
        VPSIZE,
        VLSIZE,
        MAKVTL,
        SPXISZ,
        save.WORK.as_slice_mut(),
        SPAIXD.as_slice_mut(),
        save.SPAIXI.as_slice_mut(),
        ctx,
    )?;

    //
    // Generate bounds for the 3rd coordinate.
    //
    spicelib::DSKRB2(
        NV,
        USRVRT.as_slice(),
        NP,
        USRPLT.as_slice(),
        CORSYS,
        CORPAR.as_slice(),
        &mut MNCOR3,
        &mut MXCOR3,
        ctx,
    )?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"T_WRTPLT", ctx)?;
        return Ok(());
    }

    //
    // Write the file.
    //
    if spicelib::EXISTS(DSK, ctx)? {
        //
        // Open the existing file for writing.
        //
        spicelib::DASOPW(DSK, &mut HANDLE, ctx)?;
    } else {
        //
        // Open a new file.
        //
        spicelib::DSKOPN(DSK, DSK, 0, &mut HANDLE, ctx)?;
    }

    spicelib::DSKW02(
        HANDLE,
        BODYID,
        SURFID,
        DCLASS,
        FRNAME,
        CORSYS,
        CORPAR.as_slice(),
        BOUNDS[[1, 1]],
        BOUNDS[[2, 1]],
        BOUNDS[[1, 2]],
        BOUNDS[[2, 2]],
        MNCOR3,
        MXCOR3,
        FIRST,
        LAST,
        NV,
        USRVRT.as_slice(),
        NP,
        USRPLT.as_slice(),
        SPAIXD.as_slice(),
        save.SPAIXI.as_slice(),
        ctx,
    )?;

    //
    // Close the file.
    //
    spicelib::DSKCLS(HANDLE, true, ctx)?;

    spicelib::CHKOUT(b"T_WRTPLT", ctx)?;
    Ok(())
}

//$Procedure T_WRTPLZ ( DSK test, write plate data, easy )
pub fn T_WRTPLZ(
    BODYID: i32,
    SURFID: i32,
    FRNAME: &[u8],
    NV: i32,
    NP: i32,
    USRVRT: &[f64],
    USRPLT: &[i32],
    DSK: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let USRVRT = DummyArray2D::new(USRVRT, 1..=3, 1..);
    let USRPLT = DummyArray2D::new(USRPLT, 1..=3, 1..);
    let mut ET0: f64 = 0.0;
    let mut ET1: f64 = 0.0;
    let mut FINSCL: f64 = 0.0;
    let mut LOCPAR = StackArray::<f64, 10>::new(1..=NSYPAR);
    let mut MNCOR1: f64 = 0.0;
    let mut MNCOR2: f64 = 0.0;
    let mut MNCOR3: f64 = 0.0;
    let mut MXCOR1: f64 = 0.0;
    let mut MXCOR2: f64 = 0.0;
    let mut MXCOR3: f64 = 0.0;
    let mut SPAIXD = StackArray::<f64, 10>::new(1..=IXDFIX);
    let mut CORSCL: i32 = 0;
    let mut FRAMID: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut SPXISZ: i32 = 0;
    let mut VLSIZE: i32 = 0;
    let mut VPSIZE: i32 = 0;

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"T_WRTPLZ", ctx)?;

    //
    // Convert the frame name to an ID code.
    //
    spicelib::NAMFRM(FRNAME, &mut FRAMID, ctx)?;

    if (FRAMID == 0) {
        spicelib::SETMSG(b"Could not convert frame name # to an ID code.", ctx);
        spicelib::ERRCH(b"#", FRNAME, ctx);
        spicelib::SIGERR(b"SPICE(FRAMEIDNOTFOUND)", ctx)?;
        spicelib::CHKOUT(b"T_WRTPLZ", ctx)?;
        return Ok(());
    }

    //
    // Create a spatial index for the plate set.
    //
    // Use a heuristic formula for the fine scale.
    //
    FINSCL = intrinsics::DMAX1(&[1.0, (f64::powf(NP as f64, 0.23) / 8 as f64)]);

    //
    // Pick a one-size-fits-all value for the large scale.
    //
    CORSCL = 10;

    //
    // Set the spatial index integer component size.
    //
    VPSIZE = VOXPSZ;
    VLSIZE = VOXLSZ;
    SPXISZ = MXIXSZ;

    spicelib::DSKMI2(
        NV,
        USRVRT.as_slice(),
        NP,
        USRPLT.as_slice(),
        FINSCL,
        CORSCL,
        WORKSZ,
        VOXPSZ,
        VOXLSZ,
        false,
        SPXISZ,
        save.WORK.as_slice_mut(),
        SPAIXD.as_slice_mut(),
        save.SPAIXI.as_slice_mut(),
        ctx,
    )?;

    //
    // Generate bounds for the 3rd coordinate.
    //
    spicelib::DSKRB2(
        NV,
        USRVRT.as_slice(),
        NP,
        USRPLT.as_slice(),
        LATSYS,
        LOCPAR.as_slice(),
        &mut MNCOR3,
        &mut MXCOR3,
        ctx,
    )?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"T_WRTPLZ", ctx)?;
        return Ok(());
    }

    //
    // Set default segment parameters.
    //
    MNCOR1 = 0.0;
    MXCOR1 = spicelib::TWOPI(ctx);
    MNCOR2 = -spicelib::HALFPI(ctx);
    MXCOR2 = spicelib::HALFPI(ctx);

    spicelib::CLEARD(NSYPAR, LOCPAR.as_slice_mut());

    ET0 = -((100 as f64) * spicelib::JYEAR());
    ET1 = ((100 as f64) * spicelib::JYEAR());

    //
    // Write the file.
    //
    if spicelib::EXISTS(DSK, ctx)? {
        //
        // Open the existing file for writing.
        //
        spicelib::DASOPW(DSK, &mut HANDLE, ctx)?;
    } else {
        //
        // Open a new file.
        //
        spicelib::DSKOPN(DSK, DSK, 0, &mut HANDLE, ctx)?;
    }

    spicelib::DSKW02(
        HANDLE,
        BODYID,
        SURFID,
        DCLASS,
        FRNAME,
        LATSYS,
        LOCPAR.as_slice(),
        MNCOR1,
        MXCOR1,
        MNCOR2,
        MXCOR2,
        MNCOR3,
        MXCOR3,
        ET0,
        ET1,
        NV,
        USRVRT.as_slice(),
        NP,
        USRPLT.as_slice(),
        SPAIXD.as_slice(),
        save.SPAIXI.as_slice(),
        ctx,
    )?;

    //
    // Close the file.
    //
    spicelib::DSKCLS(HANDLE, true, ctx)?;

    spicelib::CHKOUT(b"T_WRTPLZ", ctx)?;
    Ok(())
}

//$Procedure T_ELDSK2 ( DSK test, write tessellated ellipsoid DSK )
pub fn T_ELDSK2(
    BODYID: i32,
    SURFID: i32,
    FRNAME: &[u8],
    FIRST: f64,
    LAST: f64,
    CORSYS: i32,
    CORPAR: &[f64],
    BOUNDS: &[f64],
    A: f64,
    B: f64,
    C: f64,
    NLON: i32,
    NLAT: i32,
    MAKVTL: bool,
    DSK: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let CORPAR = DummyArray::new(CORPAR, 1..);
    let BOUNDS = DummyArray2D::new(BOUNDS, 1..=2, 1..=2);
    let mut FINSCL: f64 = 0.0;
    let mut MNCOR3: f64 = 0.0;
    let mut MXCOR3: f64 = 0.0;
    let mut SPAIXD = StackArray::<f64, 10>::new(1..=IXDFIX);
    let mut CORSCL: i32 = 0;
    let mut FRAMID: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut LNP: i32 = 0;
    let mut LNV: i32 = 0;
    let mut SPXISZ: i32 = 0;
    let mut VLSIZE: i32 = 0;
    let mut VMAPSZ: i32 = 0;
    let mut VPSIZE: i32 = 0;

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"T_ELDSK2", ctx)?;

    //
    // Convert the frame name to an ID code.
    //
    spicelib::NAMFRM(FRNAME, &mut FRAMID, ctx)?;

    if (FRAMID == 0) {
        spicelib::SETMSG(b"Could not convert frame name # to an ID code.", ctx);
        spicelib::ERRCH(b"#", FRNAME, ctx);
        spicelib::SIGERR(b"SPICE(FRAMEIDNOTFOUND)", ctx)?;
        spicelib::CHKOUT(b"T_ELDSK2", ctx)?;
        return Ok(());
    }

    //
    // Create vertices and plates.
    //
    support::ZZELLPLT(
        A,
        B,
        C,
        NLON,
        NLAT,
        MAXV,
        MAXP,
        &mut LNV,
        save.VRTCES.as_slice_mut(),
        &mut LNP,
        save.PLATES.as_slice_mut(),
        ctx,
    )?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"T_ELDSK2", ctx)?;
        return Ok(());
    }

    //
    // Create a spatial index for the plate set.
    //
    // Use a heuristic formula for the fine scale.
    //
    FINSCL = intrinsics::DMAX1(&[1.0, (f64::powf(LNP as f64, 0.23) / 8 as f64)]);

    //
    // Pick a one-size-fits-all value for the coarse scale.
    //
    CORSCL = 10;

    //
    // Set the spatial index integer component size.
    //
    if MAKVTL {
        VPSIZE = (VOXPSZ / 2);
        VLSIZE = (VOXLSZ / 2);

        VMAPSZ = ((2 * LNV) + (3 * LNP));

        SPXISZ = (((IXIFIX + VPSIZE) + VLSIZE) + VMAPSZ);
    } else {
        VPSIZE = VOXPSZ;
        VLSIZE = VOXLSZ;
        SPXISZ = MXIXSZ;
    }

    spicelib::DSKMI2(
        LNV,
        save.VRTCES.as_slice(),
        LNP,
        save.PLATES.as_slice(),
        FINSCL,
        CORSCL,
        WORKSZ,
        VPSIZE,
        VLSIZE,
        MAKVTL,
        SPXISZ,
        save.WORK.as_slice_mut(),
        SPAIXD.as_slice_mut(),
        save.SPAIXI.as_slice_mut(),
        ctx,
    )?;

    //
    // Generate bounds for the 3rd coordinate.
    //
    spicelib::DSKRB2(
        LNV,
        save.VRTCES.as_slice(),
        LNP,
        save.PLATES.as_slice(),
        CORSYS,
        CORPAR.as_slice(),
        &mut MNCOR3,
        &mut MXCOR3,
        ctx,
    )?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"T_ELDSK2", ctx)?;
        return Ok(());
    }

    //
    // Write the file.
    //
    if spicelib::EXISTS(DSK, ctx)? {
        //
        // Open the existing file for writing.
        //
        spicelib::DASOPW(DSK, &mut HANDLE, ctx)?;
    } else {
        //
        // Open a new file.
        //
        spicelib::DSKOPN(DSK, DSK, 0, &mut HANDLE, ctx)?;
    }

    spicelib::DSKW02(
        HANDLE,
        BODYID,
        SURFID,
        DCLASS,
        FRNAME,
        CORSYS,
        CORPAR.as_slice(),
        BOUNDS[[1, 1]],
        BOUNDS[[2, 1]],
        BOUNDS[[1, 2]],
        BOUNDS[[2, 2]],
        MNCOR3,
        MXCOR3,
        FIRST,
        LAST,
        LNV,
        save.VRTCES.as_slice(),
        LNP,
        save.PLATES.as_slice(),
        SPAIXD.as_slice(),
        save.SPAIXI.as_slice(),
        ctx,
    )?;

    //
    // Close the file.
    //
    spicelib::DSKCLS(HANDLE, true, ctx)?;

    spicelib::CHKOUT(b"T_ELDSK2", ctx)?;
    Ok(())
}

//$Procedure T_ELDS2Z ( write tessellated ellipsoid type 2 DSK, easy )
pub fn T_ELDS2Z(
    BODYID: i32,
    SURFID: i32,
    FRNAME: &[u8],
    NLON: i32,
    NLAT: i32,
    DSK: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ET0: f64 = 0.0;
    let mut ET1: f64 = 0.0;
    let mut FINSCL: f64 = 0.0;
    let mut LOCPAR = StackArray::<f64, 10>::new(1..=NSYPAR);
    let mut MNCOR1: f64 = 0.0;
    let mut MNCOR2: f64 = 0.0;
    let mut MNCOR3: f64 = 0.0;
    let mut MXCOR1: f64 = 0.0;
    let mut MXCOR2: f64 = 0.0;
    let mut MXCOR3: f64 = 0.0;
    let mut RADII = StackArray::<f64, 3>::new(1..=3);
    let mut SPAIXD = StackArray::<f64, 10>::new(1..=IXDFIX);
    let mut CORSCL: i32 = 0;
    let mut FRAMID: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut LNP: i32 = 0;
    let mut LNV: i32 = 0;
    let mut N: i32 = 0;
    let mut SPXISZ: i32 = 0;
    let mut VLSIZE: i32 = 0;
    let mut VPSIZE: i32 = 0;

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"T_ELDS2Z", ctx)?;

    //
    // Convert the frame name to an ID code.
    //
    spicelib::NAMFRM(FRNAME, &mut FRAMID, ctx)?;

    if (FRAMID == 0) {
        spicelib::SETMSG(b"Could not convert frame name # to an ID code.", ctx);
        spicelib::ERRCH(b"#", FRNAME, ctx);
        spicelib::SIGERR(b"SPICE(FRAMEIDNOTFOUND)", ctx)?;
        spicelib::CHKOUT(b"T_ELDS2Z", ctx)?;
        return Ok(());
    }

    //
    // Get target body radii from the kernel pool.
    //
    spicelib::BODVCD(BODYID, b"RADII", 3, &mut N, RADII.as_slice_mut(), ctx)?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"T_ELDS2Z", ctx)?;
        return Ok(());
    }

    //
    // Create vertices and plates.
    //
    support::ZZELLPLT(
        RADII[1],
        RADII[2],
        RADII[3],
        NLON,
        NLAT,
        MAXV,
        MAXP,
        &mut LNV,
        save.VRTCES.as_slice_mut(),
        &mut LNP,
        save.PLATES.as_slice_mut(),
        ctx,
    )?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"T_ELDS2Z", ctx)?;
        return Ok(());
    }

    //
    // Create a spatial index for the plate set.
    //
    // Use a heuristic formula for the fine scale.
    //
    FINSCL = intrinsics::DMAX1(&[1.0, (f64::powf(LNP as f64, 0.23) / 8 as f64)]);

    //
    // Pick a one-size-fits-all value for the coarse scale.
    //
    CORSCL = 10;

    //
    // Set the spatial index integer component size.
    //
    VPSIZE = VOXPSZ;
    VLSIZE = VOXLSZ;
    SPXISZ = MXIXSZ;

    spicelib::DSKMI2(
        LNV,
        save.VRTCES.as_slice(),
        LNP,
        save.PLATES.as_slice(),
        FINSCL,
        CORSCL,
        WORKSZ,
        VOXPSZ,
        VOXLSZ,
        false,
        SPXISZ,
        save.WORK.as_slice_mut(),
        SPAIXD.as_slice_mut(),
        save.SPAIXI.as_slice_mut(),
        ctx,
    )?;

    //
    // Generate bounds for the 3rd coordinate.
    //
    spicelib::DSKRB2(
        LNV,
        save.VRTCES.as_slice(),
        LNP,
        save.PLATES.as_slice(),
        LATSYS,
        LOCPAR.as_slice(),
        &mut MNCOR3,
        &mut MXCOR3,
        ctx,
    )?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"T_ELDS2Z", ctx)?;
        return Ok(());
    }

    //
    // Set default inputs.
    //
    MNCOR1 = 0.0;
    MXCOR1 = spicelib::TWOPI(ctx);
    MNCOR2 = -spicelib::HALFPI(ctx);
    MXCOR2 = spicelib::HALFPI(ctx);

    ET0 = -(spicelib::JYEAR() * 100 as f64);
    ET1 = (spicelib::JYEAR() * 100 as f64);

    spicelib::CLEARD(NSYPAR, LOCPAR.as_slice_mut());

    //
    // Write the file.
    //
    if spicelib::EXISTS(DSK, ctx)? {
        //
        // Open the existing file for writing.
        //
        spicelib::DASOPW(DSK, &mut HANDLE, ctx)?;
    } else {
        //
        // Open a new file.
        //
        spicelib::DSKOPN(DSK, DSK, 0, &mut HANDLE, ctx)?;
    }

    spicelib::DSKW02(
        HANDLE,
        BODYID,
        SURFID,
        DCLASS,
        FRNAME,
        LATSYS,
        LOCPAR.as_slice(),
        MNCOR1,
        MXCOR1,
        MNCOR2,
        MXCOR2,
        MNCOR3,
        MXCOR3,
        ET0,
        ET1,
        LNV,
        save.VRTCES.as_slice(),
        LNP,
        save.PLATES.as_slice(),
        SPAIXD.as_slice(),
        save.SPAIXI.as_slice(),
        ctx,
    )?;

    //
    // Close the file.
    //
    spicelib::DSKCLS(HANDLE, true, ctx)?;

    spicelib::CHKOUT(b"T_ELDS2Z", ctx)?;
    Ok(())
}

//$Procedure T_SMLDSK ( write small DSK )
pub fn T_SMLDSK(
    BODYID: i32,
    SURFID: i32,
    FRNAME: &[u8],
    DSK: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ET0: f64 = 0.0;
    let mut ET1: f64 = 0.0;
    let mut FINSCL: f64 = 0.0;
    let mut LOCPAR = StackArray::<f64, 10>::new(1..=NSYPAR);
    let mut MNCOR1: f64 = 0.0;
    let mut MNCOR2: f64 = 0.0;
    let mut MNCOR3: f64 = 0.0;
    let mut MXCOR1: f64 = 0.0;
    let mut MXCOR2: f64 = 0.0;
    let mut MXCOR3: f64 = 0.0;
    let mut CORSCL: i32 = 0;
    let mut FRAMID: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut LNP: i32 = 0;
    let mut LNV: i32 = 0;
    let mut SPXISZ: i32 = 0;
    let mut VLSIZE: i32 = 0;
    let mut VPSIZE: i32 = 0;

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"T_SMLDSK", ctx)?;

    //
    // Convert the frame name to an ID code.
    //
    spicelib::NAMFRM(FRNAME, &mut FRAMID, ctx)?;

    if (FRAMID == 0) {
        spicelib::SETMSG(b"Could not convert frame name # to an ID code.", ctx);
        spicelib::ERRCH(b"#", FRNAME, ctx);
        spicelib::SIGERR(b"SPICE(FRAMEIDNOTFOUND)", ctx)?;
        spicelib::CHKOUT(b"T_SMLDSK", ctx)?;
        return Ok(());
    }

    LNP = 1;
    LNV = 3;

    spicelib::VPACK(0.0, 0.0, 0.0, save.VRTCES.subarray_mut([1, 1]));
    spicelib::VPACK(1.0, 0.0, 0.0, save.VRTCES.subarray_mut([1, 2]));
    spicelib::VPACK(0.0, 1.0, 0.0, save.VRTCES.subarray_mut([1, 3]));

    save.PLATES[[1, 1]] = 1;
    save.PLATES[[2, 1]] = 2;
    save.PLATES[[3, 1]] = 3;

    VPSIZE = 32;
    VLSIZE = 40;
    SPXISZ = SMLISZ;

    FINSCL = 1.0;
    CORSCL = 1;

    //
    // Set default inputs.
    //
    MNCOR1 = 0.0;
    MXCOR1 = spicelib::TWOPI(ctx);
    MNCOR2 = -spicelib::HALFPI(ctx);
    MXCOR2 = spicelib::HALFPI(ctx);
    MNCOR3 = 0.0;
    MXCOR3 = 1.0;

    ET0 = -(spicelib::JYEAR() * 100 as f64);
    ET1 = (spicelib::JYEAR() * 100 as f64);

    spicelib::CLEARD(NSYPAR, LOCPAR.as_slice_mut());

    if save.SMALL1 {
        spicelib::DSKMI2(
            LNV,
            save.VRTCES.as_slice(),
            LNP,
            save.PLATES.as_slice(),
            FINSCL,
            CORSCL,
            WORKSZ,
            VPSIZE,
            VLSIZE,
            false,
            SPXISZ,
            save.WORK.as_slice_mut(),
            save.SMLIXD.as_slice_mut(),
            save.SMLIXI.as_slice_mut(),
            ctx,
        )?;

        if spicelib::FAILED(ctx) {
            spicelib::CHKOUT(b"T_SMLDSK", ctx)?;
            return Ok(());
        }

        save.SMALL1 = false;
    }

    //
    // Write the file.
    //
    if spicelib::EXISTS(DSK, ctx)? {
        //
        // Open the existing file for writing.
        //
        spicelib::DASOPW(DSK, &mut HANDLE, ctx)?;
    } else {
        //
        // Open a new file.
        //
        spicelib::DSKOPN(DSK, DSK, 0, &mut HANDLE, ctx)?;
    }

    spicelib::DSKW02(
        HANDLE,
        BODYID,
        SURFID,
        DCLASS,
        FRNAME,
        LATSYS,
        LOCPAR.as_slice(),
        MNCOR1,
        MXCOR1,
        MNCOR2,
        MXCOR2,
        MNCOR3,
        MXCOR3,
        ET0,
        ET1,
        LNV,
        save.VRTCES.as_slice(),
        LNP,
        save.PLATES.as_slice(),
        save.SMLIXD.as_slice(),
        save.SMLIXI.as_slice(),
        ctx,
    )?;
    //
    // Close the file.
    //
    spicelib::DSKCLS(HANDLE, false, ctx)?;

    spicelib::CHKOUT(b"T_SMLDSK", ctx)?;
    Ok(())
}

//$Procedure T_CG ( write simplified comet C-G DSK )
pub fn T_CG(
    BODYID: i32,
    SURFID: i32,
    FRNAME: &[u8],
    DSK: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ET0: f64 = 0.0;
    let mut ET1: f64 = 0.0;
    let mut FINSCL: f64 = 0.0;
    let mut LOCPAR = StackArray::<f64, 10>::new(1..=NSYPAR);
    let mut MNCOR1: f64 = 0.0;
    let mut MNCOR2: f64 = 0.0;
    let mut MNCOR3: f64 = 0.0;
    let mut MXCOR1: f64 = 0.0;
    let mut MXCOR2: f64 = 0.0;
    let mut MXCOR3: f64 = 0.0;
    let mut SPAIXD = StackArray::<f64, 10>::new(1..=IXDFIX);
    let mut CORSCL: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut LNP: i32 = 0;
    let mut LNV: i32 = 0;
    let mut LSIZEP: i32 = 0;
    let mut LSIZEV: i32 = 0;
    let mut SPXISZ: i32 = 0;
    let mut VLSIZE: i32 = 0;
    let mut VPSIZE: i32 = 0;

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"T_CG", ctx)?;

    //
    // Treat the local vertex and plate arrays as cells,
    // since the API of ZZT_CG requires this.
    //
    LSIZEV = 100;
    LSIZEP = 200;

    spicelib::SSIZED(LSIZEV, save.VRTCES.as_slice_mut(), ctx)?;
    spicelib::SSIZEI(LSIZEP, save.PLATES.as_slice_mut(), ctx)?;

    //
    // Create vertices and plates. In each array, data
    // start at 1-dimensional index LBCELL+1. This
    // location coincides with the 2-dimensional index (1,3).
    //
    ZZT_CG(save.VRTCES.as_slice_mut(), save.PLATES.as_slice_mut(), ctx)?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"T_CG", ctx)?;
        return Ok(());
    }

    LNV = (spicelib::CARDD(save.VRTCES.as_slice(), ctx)? / 3);
    LNP = (spicelib::CARDI(save.PLATES.as_slice(), ctx)? / 3);

    //
    // Create a spatial index for the plate set.
    //
    FINSCL = 1.0;

    //
    // We can use a small coarse scale.
    //
    CORSCL = 1;

    //
    // Set the spatial index integer component size.
    //
    VPSIZE = VOXPSZ;
    VLSIZE = VOXLSZ;
    SPXISZ = MXIXSZ;

    spicelib::DSKMI2(
        LNV,
        save.VRTCES.subarray([1, 3]),
        LNP,
        save.PLATES.subarray([1, 3]),
        FINSCL,
        CORSCL,
        WORKSZ,
        VOXPSZ,
        VOXLSZ,
        false,
        SPXISZ,
        save.WORK.as_slice_mut(),
        SPAIXD.as_slice_mut(),
        save.SPAIXI.as_slice_mut(),
        ctx,
    )?;

    //
    // Generate bounds for the 3rd coordinate.
    //
    spicelib::DSKRB2(
        LNV,
        save.VRTCES.subarray([1, 3]),
        LNP,
        save.PLATES.subarray([1, 3]),
        LATSYS,
        LOCPAR.as_slice(),
        &mut MNCOR3,
        &mut MXCOR3,
        ctx,
    )?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"T_CG", ctx)?;
        return Ok(());
    }

    //
    // Set default inputs.
    //
    MNCOR1 = 0.0;
    MXCOR1 = spicelib::TWOPI(ctx);
    MNCOR2 = -spicelib::HALFPI(ctx);
    MXCOR2 = spicelib::HALFPI(ctx);

    ET0 = -(spicelib::JYEAR() * 100 as f64);
    ET1 = (spicelib::JYEAR() * 100 as f64);

    spicelib::CLEARD(NSYPAR, LOCPAR.as_slice_mut());

    //
    // Write the file.
    //
    if spicelib::EXISTS(DSK, ctx)? {
        //
        // Open the existing file for writing.
        //
        spicelib::DASOPW(DSK, &mut HANDLE, ctx)?;
    } else {
        //
        // Open a new file.
        //
        spicelib::DSKOPN(DSK, DSK, 0, &mut HANDLE, ctx)?;
    }

    spicelib::DSKW02(
        HANDLE,
        BODYID,
        SURFID,
        DCLASS,
        FRNAME,
        LATSYS,
        LOCPAR.as_slice(),
        MNCOR1,
        MXCOR1,
        MNCOR2,
        MXCOR2,
        MNCOR3,
        MXCOR3,
        ET0,
        ET1,
        LNV,
        save.VRTCES.subarray([1, 3]),
        LNP,
        save.PLATES.subarray([1, 3]),
        SPAIXD.as_slice(),
        save.SPAIXI.as_slice(),
        ctx,
    )?;

    //
    // Close the file.
    //
    spicelib::DSKCLS(HANDLE, true, ctx)?;

    spicelib::CHKOUT(b"T_CG", ctx)?;
    Ok(())
}

//$Procedure T_SECDS2 ( DSK test, write ellipsoid section DSK )
pub fn T_SECDS2(
    BODYID: i32,
    SURFID: i32,
    FRNAME: &[u8],
    FIRST: f64,
    LAST: f64,
    CORSYS: i32,
    CORPAR: &[f64],
    BOUNDS: &[f64],
    A: f64,
    B: f64,
    C: f64,
    NLON: i32,
    NLAT: i32,
    MAKVTL: bool,
    USEPAD: bool,
    DSK: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let CORPAR = DummyArray::new(CORPAR, 1..);
    let BOUNDS = DummyArray2D::new(BOUNDS, 1..=2, 1..=2);
    let mut ADJLAT: f64 = 0.0;
    let mut DELTA: f64 = 0.0;
    let mut DLAT: f64 = 0.0;
    let mut DLON: f64 = 0.0;
    let mut FINSCL: f64 = 0.0;
    let mut MAXLAT: f64 = 0.0;
    let mut MAXLON: f64 = 0.0;
    let mut MINLAT: f64 = 0.0;
    let mut MINLON: f64 = 0.0;
    let mut MNCOR3: f64 = 0.0;
    let mut MXCOR3: f64 = 0.0;
    let mut Q: f64 = 0.0;
    let mut SPAIXD = StackArray::<f64, 10>::new(1..=IXDFIX);
    let mut CORSCL: i32 = 0;
    let mut FRAMID: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut LNLAT: i32 = 0;
    let mut LNLON: i32 = 0;
    let mut LNP: i32 = 0;
    let mut LNV: i32 = 0;
    let mut SPXISZ: i32 = 0;
    let mut VLSIZE: i32 = 0;
    let mut VMAPSZ: i32 = 0;
    let mut VPSIZE: i32 = 0;

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"T_SECDS2", ctx)?;

    //
    // Check coordinate system.
    //
    if (((CORSYS != LATSYS) && (CORSYS != PDTSYS)) && (CORSYS != RECSYS)) {
        spicelib::SETMSG(b"Input coordinate system code was #. Only latitudinal, planetodetic, and rectangular coordinates are currently supported by this routine.", ctx);
        spicelib::ERRINT(b"#", CORSYS, ctx);
        spicelib::SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        spicelib::CHKOUT(b"T_SECDS2", ctx)?;
        return Ok(());
    }

    //
    // Convert the frame name to an ID code.
    //
    spicelib::NAMFRM(FRNAME, &mut FRAMID, ctx)?;

    if (FRAMID == 0) {
        spicelib::SETMSG(b"Could not convert frame name # to an ID code.", ctx);
        spicelib::ERRCH(b"#", FRNAME, ctx);
        spicelib::SIGERR(b"SPICE(FRAMEIDNOTFOUND)", ctx)?;
        spicelib::CHKOUT(b"T_SECDS2", ctx)?;
        return Ok(());
    }

    //
    // Check band counts.
    //
    if (NLAT < 1) {
        spicelib::SETMSG(b"Latitude band count must be positive but was #.", ctx);
        spicelib::ERRINT(b"#", NLAT, ctx);
        spicelib::SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        spicelib::CHKOUT(b"T_SECDS2", ctx)?;
        return Ok(());
    }

    if (NLON < 1) {
        spicelib::SETMSG(b"Longitude band count must be positive but was #.", ctx);
        spicelib::ERRINT(b"#", NLON, ctx);
        spicelib::SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        spicelib::CHKOUT(b"T_SECDS2", ctx)?;
        return Ok(());
    }

    //
    // Reject invalid latitude bounds.
    //
    if (BOUNDS[[2, 2]] < BOUNDS[[1, 2]]) {
        spicelib::SETMSG(b"Latitude bounds are out of order: bounds are #:#.", ctx);
        spicelib::ERRDP(b"#", BOUNDS[[1, 2]], ctx);
        spicelib::ERRDP(b"#", BOUNDS[[2, 2]], ctx);
        spicelib::SIGERR(b"SPICE(BOUNDSOUTOFORDER)", ctx)?;
        spicelib::CHKOUT(b"T_SECDS2", ctx)?;
        return Ok(());
    }

    //
    // Make local copies of the band counts.
    //
    LNLON = NLON;
    LNLAT = NLAT;

    //
    // Create vertices and plates.
    //
    if USEPAD {
        if (CORSYS != LATSYS) {
            spicelib::SETMSG(b"Padding generation was requested for coordinate system #. This service is available only for planetocentric coordinates.", ctx);
            spicelib::ERRINT(b"#", CORSYS, ctx);
            spicelib::SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
            spicelib::CHKOUT(b"T_SECDS2", ctx)?;
            return Ok(());
        }

        //
        // Determine sizes of longitude and latitude bands.
        //
        MINLAT = BOUNDS[[1, 2]];
        MAXLAT = BOUNDS[[2, 2]];

        DLAT = ((MAXLAT - MINLAT) / NLAT as f64);

        MINLON = BOUNDS[[1, 1]];
        MAXLON = BOUNDS[[2, 1]];

        if (MAXLON <= MINLON) {
            MAXLON = (MAXLON + spicelib::TWOPI(ctx));
        }

        DLON = ((MAXLON - MINLON) / NLON as f64);

        //
        // Determine the latitude padding.
        //
        if (MAXLAT >= 0.0) {
            //
            // Add just one upper band, if it will fit. If not,
            // don't create a smaller band; just omit the band.
            //
            if ((MAXLAT + DLAT) < spicelib::HALFPI(ctx)) {
                MAXLAT = (MAXLAT + DLAT);
                LNLAT = (LNLAT + 1);
            }
        } else {
            //
            // Determine the adjusted latitude needed for full
            // latitude coverage.
            //
            spicelib::ZZCHRLAT(MAXLAT, DLON, &mut ADJLAT, ctx)?;

            if spicelib::FAILED(ctx) {
                spicelib::CHKOUT(b"T_SECDS2", ctx)?;
                return Ok(());
            }

            DELTA = (ADJLAT - MAXLAT);

            //
            // Determine the number of latitude bands that must
            // be added to give the required coverage.
            //
            Q = (DELTA / DLAT);

            if (Q > f64::trunc(Q)) {
                //
                // Q has a non-zero fractional part.
                //
                Q = (f64::trunc(Q) + 1.0);
            }

            MAXLAT = (MAXLAT + (Q * DLAT));
            LNLAT = (LNLAT + (Q as i32));
        }
        //
        // MAXLAT and LNLAT have been updated to account for
        // upper padding, if it has been added.
        //

        if (MINLAT <= 0.0) {
            //
            // Add just one lower band, if it will fit. If not,
            // don't create a smaller band; just omit the band.
            //
            if ((MINLAT - DLAT) > -spicelib::HALFPI(ctx)) {
                MINLAT = (MINLAT - DLAT);
                LNLAT = (LNLAT + 1);
            }
        } else {
            //
            // Determine the adjusted latitude needed for full
            // latitude coverage.
            //
            spicelib::ZZCHRLAT(MINLAT, DLON, &mut ADJLAT, ctx)?;

            if spicelib::FAILED(ctx) {
                spicelib::CHKOUT(b"T_SECDS2", ctx)?;
                return Ok(());
            }

            DELTA = (MINLAT - ADJLAT);

            //
            // Determine the number of latitude bands that must
            // be added to give the required coverage.
            //
            Q = (DELTA / DLAT);

            if (Q > f64::trunc(Q)) {
                //
                // Q has a non-zero fractional part.
                //
                Q = (f64::trunc(Q) + 1.0);
            }

            MINLAT = (MINLAT - (Q * DLAT));
            LNLAT = (LNLAT + (Q as i32));
        }
        //
        // MAXLAT and LNLAT have been updated to account for
        // lower padding, if it has been added.
        //

        //
        // Adjust the longitude boundaries and band count to
        // reflect longitude padding. Padded is added only if
        // the padded section would have 2*pi longitude extent or
        // less.
        //
        if ((MAXLON - MINLON) < (spicelib::TWOPI(ctx) - ((2 as f64) * DLON))) {
            //
            // Add longitude padding.
            //
            MINLON = (MINLON - DLON);
            MAXLON = (MAXLON + DLON);
            LNLON = (LNLON + 2);
        }

        //
        // Create a vertex and plate set that includes padding.
        //
        support::ZZELLSEC(
            A,
            B,
            C,
            MINLON,
            MAXLON,
            MINLAT,
            MAXLAT,
            LNLON,
            LNLAT,
            MAXV,
            MAXP,
            &mut LNV,
            save.VRTCES.as_slice_mut(),
            &mut LNP,
            save.PLATES.as_slice_mut(),
            ctx,
        )?;
    } else {
        //
        // Create a vertex and plate set only for the specified region.
        //
        support::ZZELLSEC(
            A,
            B,
            C,
            BOUNDS[[1, 1]],
            BOUNDS[[2, 1]],
            BOUNDS[[1, 2]],
            BOUNDS[[2, 2]],
            NLON,
            NLAT,
            MAXV,
            MAXP,
            &mut LNV,
            save.VRTCES.as_slice_mut(),
            &mut LNP,
            save.PLATES.as_slice_mut(),
            ctx,
        )?;
    }

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"T_SECDS2", ctx)?;
        return Ok(());
    }

    //
    // Create a spatial index for the plate set.
    //
    // Use a heuristic formula for the fine scale.
    //
    FINSCL = intrinsics::DMAX1(&[1.0, (f64::powf(LNP as f64, 0.23) / 8 as f64)]);

    //
    // Pick a one-size-fits-all value for the coarse scale.
    //
    CORSCL = 10;

    //
    // Set the spatial index integer component size.
    //
    if MAKVTL {
        VPSIZE = (VOXPSZ / 2);
        VLSIZE = (VOXLSZ / 2);

        VMAPSZ = ((2 * LNV) + (3 * LNP));

        SPXISZ = (((IXIFIX + VPSIZE) + VLSIZE) + VMAPSZ);
    } else {
        VPSIZE = VOXPSZ;
        VLSIZE = VOXLSZ;
        SPXISZ = MXIXSZ;
    }

    spicelib::DSKMI2(
        LNV,
        save.VRTCES.as_slice(),
        LNP,
        save.PLATES.as_slice(),
        FINSCL,
        CORSCL,
        WORKSZ,
        VPSIZE,
        VLSIZE,
        MAKVTL,
        SPXISZ,
        save.WORK.as_slice_mut(),
        SPAIXD.as_slice_mut(),
        save.SPAIXI.as_slice_mut(),
        ctx,
    )?;

    //
    // Generate bounds for the 3rd coordinate.
    //
    spicelib::DSKRB2(
        LNV,
        save.VRTCES.as_slice(),
        LNP,
        save.PLATES.as_slice(),
        CORSYS,
        CORPAR.as_slice(),
        &mut MNCOR3,
        &mut MXCOR3,
        ctx,
    )?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"T_SECDS2", ctx)?;
        return Ok(());
    }

    //
    // Write the file.
    //
    if spicelib::EXISTS(DSK, ctx)? {
        //
        // Open the existing file for writing.
        //
        spicelib::DASOPW(DSK, &mut HANDLE, ctx)?;
    } else {
        //
        // Open a new file.
        //
        spicelib::DSKOPN(DSK, DSK, 0, &mut HANDLE, ctx)?;
    }

    spicelib::DSKW02(
        HANDLE,
        BODYID,
        SURFID,
        DCLASS,
        FRNAME,
        CORSYS,
        CORPAR.as_slice(),
        BOUNDS[[1, 1]],
        BOUNDS[[2, 1]],
        BOUNDS[[1, 2]],
        BOUNDS[[2, 2]],
        MNCOR3,
        MXCOR3,
        FIRST,
        LAST,
        LNV,
        save.VRTCES.as_slice(),
        LNP,
        save.PLATES.as_slice(),
        SPAIXD.as_slice(),
        save.SPAIXI.as_slice(),
        ctx,
    )?;

    //
    // Close the file.
    //
    spicelib::DSKCLS(HANDLE, true, ctx)?;

    spicelib::CHKOUT(b"T_SECDS2", ctx)?;
    Ok(())
}

//$Procedure T_TORUS ( write DSK containing torus plate model )
pub fn T_TORUS(
    BODYID: i32,
    SURFID: i32,
    FRNAME: &[u8],
    NPOLYV: i32,
    NCROSS: i32,
    R: f64,
    RCROSS: f64,
    CENTER: &[f64],
    NORMAL: &mut [f64],
    DSK: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let CENTER = DummyArray::new(CENTER, 1..=3);
    let mut NORMAL = DummyArrayMut::new(NORMAL, 1..=3);
    let mut C0 = StackArray::<f64, 3>::new(1..=3);
    let mut D0 = StackArray::<f64, 3>::new(1..=3);
    let mut ET0: f64 = 0.0;
    let mut ET1: f64 = 0.0;
    let mut FINSCL: f64 = 0.0;
    let mut LOCPAR = StackArray::<f64, 10>::new(1..=NSYPAR);
    let mut MNCOR1: f64 = 0.0;
    let mut MNCOR2: f64 = 0.0;
    let mut MNCOR3: f64 = 0.0;
    let mut MXCOR1: f64 = 0.0;
    let mut MXCOR2: f64 = 0.0;
    let mut MXCOR3: f64 = 0.0;
    let mut SPAIXD = StackArray::<f64, 10>::new(1..=IXDFIX);
    let mut THETA: f64 = 0.0;
    let mut TWIST: f64 = 0.0;
    let mut V0 = StackArray::<f64, 3>::new(1..=3);
    let mut X = StackArray::<f64, 3>::new(1..=3);
    let mut Y = StackArray::<f64, 3>::new(1..=3);
    let mut BASE: i32 = 0;
    let mut CORSCL: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut LNP: i32 = 0;
    let mut LNV: i32 = 0;
    let mut SIZE: i32 = 0;
    let mut SPXISZ: i32 = 0;
    let mut VLSIZE: i32 = 0;
    let mut VPSIZE: i32 = 0;

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"T_TORUS", ctx)?;

    if (NPOLYV < 3) {
        spicelib::SETMSG(b"Polygon side count # must be at least three.", ctx);
        spicelib::ERRINT(b"#", NPOLYV, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        spicelib::CHKOUT(b"T_TORUS", ctx)?;
        return Ok(());
    }

    if (NCROSS < 3) {
        spicelib::SETMSG(b"Cross section count # must be at least three.", ctx);
        spicelib::ERRINT(b"#", NCROSS, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        spicelib::CHKOUT(b"T_TORUS", ctx)?;
        return Ok(());
    }

    //
    // Initialize the routine that generates the central curve
    // for the torus.
    //
    T_TORSET(R, CENTER.as_slice(), NORMAL.as_slice_mut(), ctx)?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"T_TORUS", ctx)?;
        return Ok(());
    }

    //
    // Generate the cross-section polygon for the torus. The polygon is
    // positively oriented about the first endpoint of the central
    // curve.
    //
    // Start by generating a first vertex on the polygon. V0 is a point
    // on the central curve.
    //
    T_TORCRV(0.0, C0.as_slice_mut(), D0.as_slice_mut(), &mut TWIST, ctx)?;

    spicelib::FRAME(D0.as_slice_mut(), X.as_slice_mut(), Y.as_slice_mut());

    spicelib::VSCL(RCROSS, X.as_slice(), V0.as_slice_mut());

    //
    // Generate the rest of the vertices.
    //
    spicelib::VEQU(V0.as_slice(), save.VRTCES.subarray_mut([1, 1]));

    for I in 2..=NPOLYV {
        //
        // NPOLYV has been checked to ensure it's non-zero.
        //
        THETA = ((((I - 1) as f64) * spicelib::TWOPI(ctx)) / NPOLYV as f64);

        spicelib::VROTV(
            V0.as_slice(),
            D0.as_slice(),
            THETA,
            save.VRTCES.subarray_mut([1, I]),
        );
    }

    //
    // Prepare to use the rest of the VRTCES array as a cell.
    //
    BASE = (NPOLYV + 1);

    SIZE = (3 * ((MAXV - NPOLYV) - 2));

    spicelib::SSIZED(SIZE, save.VRTCES.subarray_mut([1, BASE]), ctx)?;
    spicelib::SSIZEI((3 * MAXP), save.PLATES.as_slice_mut(), ctx)?;

    //
    // Create a tube centered on the central curve. This is the
    // torus.
    //
    ZZPSTUBE(
        NPOLYV,
        &save.VRTCES.as_slice().to_vec(),
        T_TORCRV,
        NCROSS,
        true,
        save.VRTCES.subarray_mut([1, BASE]),
        save.PLATES.as_slice_mut(),
        ctx,
    )?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"T_TORUS", ctx)?;
        return Ok(());
    }

    //
    // Get the vertex and plate counts.
    //
    LNV = (spicelib::CARDD(save.VRTCES.subarray([1, BASE]), ctx)? / 3);
    LNP = (spicelib::CARDI(save.PLATES.as_slice(), ctx)? / 3);

    //
    // Create a spatial index for the plate set.
    //
    // Use a heuristic formula for the fine scale.
    //
    FINSCL = intrinsics::DMAX1(&[1.0, (f64::powf(LNP as f64, 0.23) / 8 as f64)]);

    //
    // Pick a one-size-fits-all value for the coarse scale.
    //
    CORSCL = 10;

    //
    // Set the spatial index integer component size.
    //
    VPSIZE = VOXPSZ;
    VLSIZE = VOXLSZ;
    SPXISZ = MXIXSZ;

    spicelib::DSKMI2(
        LNV,
        save.VRTCES.subarray([1, (BASE + 2)]),
        LNP,
        save.PLATES.subarray([1, 3]),
        FINSCL,
        CORSCL,
        WORKSZ,
        VPSIZE,
        VLSIZE,
        false,
        SPXISZ,
        save.WORK.as_slice_mut(),
        SPAIXD.as_slice_mut(),
        save.SPAIXI.as_slice_mut(),
        ctx,
    )?;

    //
    // Generate bounds for the 3rd coordinate.
    //
    spicelib::DSKRB2(
        LNV,
        save.VRTCES.subarray([1, (BASE + 2)]),
        LNP,
        save.PLATES.subarray([1, 3]),
        LATSYS,
        LOCPAR.as_slice(),
        &mut MNCOR3,
        &mut MXCOR3,
        ctx,
    )?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"T_TORUS", ctx)?;
        return Ok(());
    }

    //
    // Write the file.
    //
    if spicelib::EXISTS(DSK, ctx)? {
        //
        // Open the existing file for writing.
        //
        spicelib::DASOPW(DSK, &mut HANDLE, ctx)?;
    } else {
        //
        // Open a new file.
        //
        spicelib::DSKOPN(DSK, DSK, 0, &mut HANDLE, ctx)?;
    }

    //
    // Set default inputs.
    //
    MNCOR1 = 0.0;
    MXCOR1 = spicelib::TWOPI(ctx);
    MNCOR2 = -spicelib::HALFPI(ctx);
    MXCOR2 = spicelib::HALFPI(ctx);

    ET0 = -(spicelib::JYEAR() * 100 as f64);
    ET1 = (spicelib::JYEAR() * 100 as f64);

    spicelib::CLEARD(NSYPAR, LOCPAR.as_slice_mut());

    //
    // Write the file.
    //
    spicelib::DSKW02(
        HANDLE,
        BODYID,
        SURFID,
        DCLASS,
        FRNAME,
        LATSYS,
        LOCPAR.as_slice(),
        MNCOR1,
        MXCOR1,
        MNCOR2,
        MXCOR2,
        MNCOR3,
        MXCOR3,
        ET0,
        ET1,
        LNV,
        save.VRTCES.subarray([1, (BASE + 2)]),
        LNP,
        save.PLATES.subarray([1, 3]),
        SPAIXD.as_slice(),
        save.SPAIXI.as_slice(),
        ctx,
    )?;

    //
    // Close the file.
    //
    spicelib::DSKCLS(HANDLE, true, ctx)?;

    spicelib::CHKOUT(b"T_TORUS", ctx)?;
    Ok(())
}
