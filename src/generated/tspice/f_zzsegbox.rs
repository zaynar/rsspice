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
const VTIGHT: f64 = 0.00000000000001;
const FILSIZ: i32 = 255;
const NDSKS: i32 = 3;
const NAMLEN: i32 = 32;

struct SaveVars {
    DSKS: ActualCharArray,
    FRNAME: Vec<u8>,
    A: f64,
    B: f64,
    BOUNDS: StackArray2D<f64, 6>,
    BOXCTR: StackArray<f64, 3>,
    C: f64,
    CORPAR: StackArray<f64, 10>,
    DSKDSC: StackArray<f64, 24>,
    F: f64,
    FIRST: f64,
    LAST: f64,
    LR: f64,
    LT: f64,
    LZ: f64,
    MAXV: StackArray<f64, 3>,
    MINV: StackArray<f64, 3>,
    RADIUS: f64,
    TOL: f64,
    VERTEX: StackArray<f64, 3>,
    XCTR: StackArray<f64, 3>,
    XRAD: f64,
    BODYID: i32,
    CORSYS: i32,
    DLADSC: StackArray<i32, 8>,
    HANDLE: i32,
    N: i32,
    NLAT: i32,
    NLON: i32,
    NP: i32,
    NV: i32,
    SURFID: i32,
    FOUND: bool,
    MAKVTL: bool,
    USEPAD: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut DSKS = ActualCharArray::new(FILSIZ, 1..=NDSKS);
        let mut FRNAME = vec![b' '; NAMLEN as usize];
        let mut A: f64 = 0.0;
        let mut B: f64 = 0.0;
        let mut BOUNDS = StackArray2D::<f64, 6>::new(1..=2, 1..=3);
        let mut BOXCTR = StackArray::<f64, 3>::new(1..=3);
        let mut C: f64 = 0.0;
        let mut CORPAR = StackArray::<f64, 10>::new(1..=NSYPAR);
        let mut DSKDSC = StackArray::<f64, 24>::new(1..=DSKDSZ);
        let mut F: f64 = 0.0;
        let mut FIRST: f64 = 0.0;
        let mut LAST: f64 = 0.0;
        let mut LR: f64 = 0.0;
        let mut LT: f64 = 0.0;
        let mut LZ: f64 = 0.0;
        let mut MAXV = StackArray::<f64, 3>::new(1..=3);
        let mut MINV = StackArray::<f64, 3>::new(1..=3);
        let mut RADIUS: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut VERTEX = StackArray::<f64, 3>::new(1..=3);
        let mut XCTR = StackArray::<f64, 3>::new(1..=3);
        let mut XRAD: f64 = 0.0;
        let mut BODYID: i32 = 0;
        let mut CORSYS: i32 = 0;
        let mut DLADSC = StackArray::<i32, 8>::new(1..=DLADSZ);
        let mut HANDLE: i32 = 0;
        let mut N: i32 = 0;
        let mut NLAT: i32 = 0;
        let mut NLON: i32 = 0;
        let mut NP: i32 = 0;
        let mut NV: i32 = 0;
        let mut SURFID: i32 = 0;
        let mut FOUND: bool = false;
        let mut MAKVTL: bool = false;
        let mut USEPAD: bool = false;

        Self {
            DSKS,
            FRNAME,
            A,
            B,
            BOUNDS,
            BOXCTR,
            C,
            CORPAR,
            DSKDSC,
            F,
            FIRST,
            LAST,
            LR,
            LT,
            LZ,
            MAXV,
            MINV,
            RADIUS,
            TOL,
            VERTEX,
            XCTR,
            XRAD,
            BODYID,
            CORSYS,
            DLADSC,
            HANDLE,
            N,
            NLAT,
            NLON,
            NP,
            NV,
            SURFID,
            FOUND,
            MAKVTL,
            USEPAD,
        }
    }
}

//$Procedure F_ZZSEGBOX ( ZZSEGBOX tests )
pub fn F_ZZSEGBOX(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Saved values
    //

    //
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_ZZSEGBOX", ctx)?;

    //***********************************************************************
    //
    //     Normal cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Check bounding sphere for segment using the Latitudinal coordinate system.",
        ctx,
    )?;

    //
    // Strictly speaking, we don't need an actual DSK file; we just
    // need a DSK descriptor. However, we'll create a file in order
    // to work with a realistic descriptor.
    //

    save.BODYID = 499;
    save.SURFID = 1;
    fstr::assign(&mut save.FRNAME, b"IAU_MARS");
    save.FIRST = -((100 as f64) * spicelib::JYEAR());
    save.LAST = ((100 as f64) * spicelib::JYEAR());
    save.CORSYS = LATSYS;

    save.A = 3400.0;
    save.B = save.A;
    save.C = 3375.0;

    spicelib::CLEARD(NSYPAR, save.CORPAR.as_slice_mut());

    //
    // Longitude bounds:
    //
    save.BOUNDS[[1, 1]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 3 as f64);

    //
    // Latitude bounds:
    //
    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 4 as f64);

    //
    // Radius bounds are not used by T_SECDS2.
    //
    save.BOUNDS[[1, 3]] = 0.0;
    save.BOUNDS[[2, 3]] = 0.0;

    save.NLON = 100;
    save.NLAT = 50;

    save.MAKVTL = false;
    save.USEPAD = true;

    fstr::assign(save.DSKS.get_mut(1), b"zzsegbox_lat.bds");

    if spicelib::EXISTS(&save.DSKS[1], ctx)? {
        spicelib::DELFIL(&save.DSKS[1], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Create the DSK file.
    //
    testutil::T_SECDS2(
        save.BODYID,
        save.SURFID,
        &save.FRNAME,
        save.FIRST,
        save.LAST,
        save.CORSYS,
        save.CORPAR.as_slice(),
        save.BOUNDS.as_slice(),
        save.A,
        save.B,
        save.C,
        save.NLON,
        save.NLAT,
        save.MAKVTL,
        save.USEPAD,
        &save.DSKS[1],
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Fetch the DSK descriptor for the segment we just created.
    //
    spicelib::DASOPR(&save.DSKS[1], &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DLABFS(
        save.HANDLE,
        save.DLADSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"DLA segment found", save.FOUND, true, OK, ctx)?;

    spicelib::DSKGD(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set the segment's radius bounds from the DSK descriptor.
    //
    save.BOUNDS[[1, 3]] = save.DSKDSC[MN3IDX];
    save.BOUNDS[[2, 3]] = save.DSKDSC[MX3IDX];

    //
    // Create a bounding box and sphere for the segment using both
    // ZZSEGBOX and ZZLATBOX. The results should match.
    //
    spicelib::ZZSEGBOX(
        save.DSKDSC.as_slice(),
        save.BOXCTR.as_slice_mut(),
        &mut save.RADIUS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZLATBOX(
        save.BOUNDS.as_slice(),
        save.XCTR.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.XRAD,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"RADIUS", save.RADIUS, b"~/", save.XRAD, save.TOL, OK, ctx)?;
    testutil::CHCKAD(
        b"BOXCTR",
        save.BOXCTR.as_slice(),
        b"~~/",
        save.XCTR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Check bounding sphere for segment using the Planetodetic coordinate system.",
        ctx,
    )?;

    //
    // Strictly speaking, we don't need an actual DSK file; we just
    // need a DSK descriptor. However, we'll create a file in order
    // to work with a realistic descriptor.
    //

    save.BODYID = 599;
    save.SURFID = 1;
    fstr::assign(&mut save.FRNAME, b"IAU_JUPITER");
    save.FIRST = -((100 as f64) * spicelib::JYEAR());
    save.LAST = ((100 as f64) * spicelib::JYEAR());
    save.CORSYS = PDTSYS;

    save.A = 71492.0;
    save.B = save.A;
    save.C = 66854.0;

    save.F = ((save.A - save.C) / save.A);

    save.CORPAR[1] = save.A;
    save.CORPAR[2] = save.F;

    spicelib::CLEARD((NSYPAR - 2), save.CORPAR.subarray_mut(3));

    //
    // Longitude bounds:
    //
    save.BOUNDS[[1, 1]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 3 as f64);

    //
    // Latitude bounds:
    //
    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 4 as f64);

    //
    // Altitude bounds are not used by T_SECDS2.
    //
    save.BOUNDS[[1, 3]] = 0.0;
    save.BOUNDS[[2, 3]] = 0.0;

    save.NLON = 100;
    save.NLAT = 50;

    save.MAKVTL = false;
    save.USEPAD = false;

    fstr::assign(save.DSKS.get_mut(2), b"zzsegbox_pdt.bds");

    if spicelib::EXISTS(&save.DSKS[2], ctx)? {
        spicelib::DELFIL(&save.DSKS[2], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Create the DSK file.
    //
    testutil::T_SECDS2(
        save.BODYID,
        save.SURFID,
        &save.FRNAME,
        save.FIRST,
        save.LAST,
        save.CORSYS,
        save.CORPAR.as_slice(),
        save.BOUNDS.as_slice(),
        save.A,
        save.B,
        save.C,
        save.NLON,
        save.NLAT,
        save.MAKVTL,
        save.USEPAD,
        &save.DSKS[2],
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Fetch the DSK descriptor for the segment we just created.
    //
    spicelib::DASOPR(&save.DSKS[2], &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DLABFS(
        save.HANDLE,
        save.DLADSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"DLA segment found", save.FOUND, true, OK, ctx)?;

    spicelib::DSKGD(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set the segment's altitude bounds from the DSK descriptor.
    //
    save.BOUNDS[[1, 3]] = save.DSKDSC[MN3IDX];
    save.BOUNDS[[2, 3]] = save.DSKDSC[MX3IDX];

    //
    // Create a bounding box and sphere for the segment using both
    // ZZSEGBOX and ZZLATBOX. The results should match.
    //
    spicelib::ZZSEGBOX(
        save.DSKDSC.as_slice(),
        save.BOXCTR.as_slice_mut(),
        &mut save.RADIUS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZPDTBOX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.XCTR.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.XRAD,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"RADIUS", save.RADIUS, b"~/", save.XRAD, save.TOL, OK, ctx)?;
    testutil::CHCKAD(
        b"BOXCTR",
        save.BOXCTR.as_slice(),
        b"~~/",
        save.XCTR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Check bounding sphere for segment using the Rectangular coordinate system.",
        ctx,
    )?;

    //
    // Strictly speaking, we don't need an actual DSK file; we just
    // need a DSK descriptor. However, we'll create a file in order
    // to work with a realistic descriptor.
    //
    // We're going to start out as though we're using the latitudinal
    // system, since the plate set generation utilities work with a
    // lon/lat coverage specification.
    //
    save.BODYID = 699;
    save.SURFID = 1;
    fstr::assign(&mut save.FRNAME, b"IAU_SATURN");
    save.FIRST = -((100 as f64) * spicelib::JYEAR());
    save.LAST = ((100 as f64) * spicelib::JYEAR());
    save.CORSYS = LATSYS;

    save.A = 60268.0;
    save.B = save.A;
    save.C = 54364.0;

    spicelib::CLEARD(NSYPAR, save.CORPAR.as_slice_mut());

    //
    // Longitude bounds:
    //
    save.BOUNDS[[1, 1]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 3 as f64);

    //
    // Latitude bounds:
    //
    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 4 as f64);

    //
    // Radius bounds are not used by T_SECDS2.
    //
    save.BOUNDS[[1, 3]] = 0.0;
    save.BOUNDS[[2, 3]] = 0.0;

    save.NLON = 100;
    save.NLAT = 50;

    save.MAKVTL = false;
    save.USEPAD = true;

    fstr::assign(save.DSKS.get_mut(3), b"zzsegbox_rec.bds");

    if spicelib::EXISTS(&save.DSKS[3], ctx)? {
        spicelib::DELFIL(&save.DSKS[3], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Create the DSK file.
    //
    testutil::T_SECDS2(
        save.BODYID,
        save.SURFID,
        &save.FRNAME,
        save.FIRST,
        save.LAST,
        save.CORSYS,
        save.CORPAR.as_slice(),
        save.BOUNDS.as_slice(),
        save.A,
        save.B,
        save.C,
        save.NLON,
        save.NLAT,
        save.MAKVTL,
        save.USEPAD,
        &save.DSKS[1],
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Fetch the DSK descriptor for the segment we just created.
    //
    spicelib::DASOPR(&save.DSKS[1], &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DLABFS(
        save.HANDLE,
        save.DLADSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"DLA segment found", save.FOUND, true, OK, ctx)?;

    spicelib::DSKGD(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Here's where we deviate from the latitudinal case. We're going
    // to convert the descriptor to one using rectangular coordinates.
    //
    // Before closing the file, fetch the vertices of the file
    // and obtain the extents of the plate set.
    //
    spicelib::DSKZ02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        &mut save.NV,
        &mut save.NP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Let MINV(I) and MAXV(I) store the minimum and maximum values of
    // the Ith components of the vertices.
    //
    for I in 1..=3 {
        save.MINV[I] = spicelib::DPMAX();
        save.MAXV[I] = spicelib::DPMIN();
    }

    for I in 1..=save.NV {
        spicelib::DSKV02(
            save.HANDLE,
            save.DLADSC.as_slice(),
            I,
            1,
            &mut save.N,
            save.VERTEX.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        for J in 1..=3 {
            save.MINV[J] = intrinsics::DMIN1(&[save.VERTEX[J], save.MINV[J]]);
            save.MAXV[J] = intrinsics::DMAX1(&[save.VERTEX[J], save.MAXV[J]]);
        }
    }

    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Turn the descriptor into one for rectangular coordinates.
    //
    save.DSKDSC[SYSIDX] = RECSYS as f64;

    save.DSKDSC[MN1IDX] = save.MINV[1];
    save.DSKDSC[MX1IDX] = save.MAXV[1];
    save.DSKDSC[MN2IDX] = save.MINV[2];
    save.DSKDSC[MX2IDX] = save.MAXV[2];
    save.DSKDSC[MN3IDX] = save.MINV[3];
    save.DSKDSC[MX3IDX] = save.MAXV[3];

    //
    // Create a bounding box and sphere for the segment using both
    // ZZSEGBOX and ZZLATBOX. The results should match.
    //
    spicelib::ZZSEGBOX(
        save.DSKDSC.as_slice(),
        save.BOXCTR.as_slice_mut(),
        &mut save.RADIUS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=3 {
        save.BOUNDS[[1, I]] = save.MINV[I];
        save.BOUNDS[[2, I]] = save.MAXV[I];
    }

    spicelib::ZZRECBOX(
        save.BOUNDS.as_slice(),
        save.XCTR.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.XRAD,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"RADIUS", save.RADIUS, b"~/", save.XRAD, save.TOL, OK, ctx)?;
    testutil::CHCKAD(
        b"BOXCTR",
        save.BOXCTR.as_slice(),
        b"~~/",
        save.XCTR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //***********************************************************************
    //
    //     Error cases
    //
    //***********************************************************************

    //
    // ---------------------------------------------------------
    //
    testutil::TCASE(b"Error: try segment using cylindrical coordinates.", ctx)?;

    //
    // Use descriptor from last case; change the coordinate system
    // parameter to indicate cylindrical coordinates.
    //
    save.DSKDSC[SYSIDX] = (CYLSYS as f64);

    spicelib::ZZSEGBOX(
        save.DSKDSC.as_slice(),
        save.BOXCTR.as_slice_mut(),
        &mut save.RADIUS,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // Clean up.
    //

    //
    // ---------------------------------------------------------
    //
    testutil::TCASE(b"Clean up", ctx)?;

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=NDSKS {
        if spicelib::EXISTS(&save.DSKS[I], ctx)? {
            spicelib::DELFIL(&save.DSKS[I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
