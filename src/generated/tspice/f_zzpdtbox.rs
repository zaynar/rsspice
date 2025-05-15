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
const TIGHT: f64 = 0.00000000001;

struct SaveVars {
    BOUNDS: StackArray2D<f64, 6>,
    CENTER: StackArray<f64, 3>,
    CORPAR: StackArray<f64, 10>,
    F: f64,
    P: StackArray<f64, 3>,
    PMAX: StackArray<f64, 3>,
    PMAXXY: StackArray<f64, 3>,
    PMIN: StackArray<f64, 3>,
    PMINXY: StackArray<f64, 3>,
    PXY: StackArray<f64, 3>,
    LR: f64,
    LT: f64,
    LZ: f64,
    RADIUS: f64,
    RE: f64,
    RP: f64,
    TOL: f64,
    XCTR: StackArray<f64, 3>,
    XLR: f64,
    XLT: f64,
    XLZ: f64,
    XRAD: f64,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BOUNDS = StackArray2D::<f64, 6>::new(1..=2, 1..=3);
        let mut CENTER = StackArray::<f64, 3>::new(1..=3);
        let mut CORPAR = StackArray::<f64, 10>::new(1..=NSYPAR);
        let mut F: f64 = 0.0;
        let mut P = StackArray::<f64, 3>::new(1..=3);
        let mut PMAX = StackArray::<f64, 3>::new(1..=3);
        let mut PMAXXY = StackArray::<f64, 3>::new(1..=3);
        let mut PMIN = StackArray::<f64, 3>::new(1..=3);
        let mut PMINXY = StackArray::<f64, 3>::new(1..=3);
        let mut PXY = StackArray::<f64, 3>::new(1..=3);
        let mut LR: f64 = 0.0;
        let mut LT: f64 = 0.0;
        let mut LZ: f64 = 0.0;
        let mut RADIUS: f64 = 0.0;
        let mut RE: f64 = 0.0;
        let mut RP: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut XCTR = StackArray::<f64, 3>::new(1..=3);
        let mut XLR: f64 = 0.0;
        let mut XLT: f64 = 0.0;
        let mut XLZ: f64 = 0.0;
        let mut XRAD: f64 = 0.0;

        Self {
            BOUNDS,
            CENTER,
            CORPAR,
            F,
            P,
            PMAX,
            PMAXXY,
            PMIN,
            PMINXY,
            PXY,
            LR,
            LT,
            LZ,
            RADIUS,
            RE,
            RP,
            TOL,
            XCTR,
            XLR,
            XLT,
            XLZ,
            XRAD,
        }
    }
}

//$Procedure F_ZZPDTBOX ( ZZPDTBOX tests )
pub fn F_ZZPDTBOX(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    // Save variables in order to avoid stack room problems.
    //

    //
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_ZZPDTBOX", ctx)?;

    //**********************************************************************
    //
    //     Error cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Bad longitude range: max is less than min by more than 2*pi.",
        ctx,
    )?;

    spicelib::CLEARD(NSYPAR, save.CORPAR.as_slice_mut());

    save.RE = 3000.0;
    save.F = 0.5;

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = (-spicelib::TWOPI(ctx) - 0.000000000000001);

    save.BOUNDS[[1, 2]] = -spicelib::HALFPI(ctx);
    save.BOUNDS[[2, 2]] = spicelib::HALFPI(ctx);

    save.BOUNDS[[1, 3]] = 10.0;
    save.BOUNDS[[2, 3]] = 20.0;

    spicelib::ZZPDTBOX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.CENTER.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.RADIUS,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADLONGITUDERANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Latitude bounds out of order", ctx)?;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);

    save.BOUNDS[[1, 2]] = spicelib::HALFPI(ctx);
    save.BOUNDS[[2, 2]] = -spicelib::HALFPI(ctx);

    save.BOUNDS[[1, 3]] = 10.0;
    save.BOUNDS[[2, 3]] = 20.0;

    spicelib::ZZPDTBOX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.CENTER.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.RADIUS,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADLATITUDEBOUNDS)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Minimum latitude is too small", ctx)?;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);

    save.BOUNDS[[1, 2]] = (-spicelib::HALFPI(ctx) - 0.0000000001);
    save.BOUNDS[[2, 2]] = spicelib::HALFPI(ctx);

    save.BOUNDS[[1, 3]] = 10.0;
    save.BOUNDS[[2, 3]] = 20.0;

    spicelib::ZZPDTBOX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.CENTER.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.RADIUS,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADLATITUDERANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Maximum latitude is too large", ctx)?;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);

    save.BOUNDS[[1, 2]] = -spicelib::HALFPI(ctx);
    save.BOUNDS[[2, 2]] = (spicelib::HALFPI(ctx) + 0.0000000001);

    save.BOUNDS[[1, 3]] = 10.0;
    save.BOUNDS[[2, 3]] = 20.0;

    spicelib::ZZPDTBOX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.CENTER.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.RADIUS,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADLATITUDERANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid shape parameters.", ctx)?;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);

    save.BOUNDS[[1, 2]] = -spicelib::HALFPI(ctx);
    save.BOUNDS[[2, 2]] = spicelib::HALFPI(ctx);

    save.BOUNDS[[1, 3]] = -10.0;
    save.BOUNDS[[2, 3]] = 20.0;

    save.RE = 0.0;
    save.F = 0.0;

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    spicelib::ZZPDTBOX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.CENTER.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.RADIUS,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    save.CORPAR[2] = 1.5;

    spicelib::ZZPDTBOX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.CENTER.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.RADIUS,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //**********************************************************************
    //
    //     Normal cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Trivial case: element is spherical shell.", ctx)?;

    save.RE = 3000.0;
    save.F = 0.0;

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);

    save.BOUNDS[[1, 2]] = -spicelib::HALFPI(ctx);
    save.BOUNDS[[2, 2]] = spicelib::HALFPI(ctx);

    save.BOUNDS[[1, 3]] = -10.0;
    save.BOUNDS[[2, 3]] = 20.0;

    spicelib::ZZPDTBOX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.CENTER.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.RADIUS,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CLEARD(3, save.XCTR.as_slice_mut());

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"CENTER",
        save.CENTER.as_slice(),
        b"~",
        save.XCTR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    save.XRAD = ((save.RE + save.BOUNDS[[2, 3]]) * f64::sqrt(3.0));

    testutil::CHCKSD(b"RADIUS", save.RADIUS, b"~", save.XRAD, save.TOL, OK, ctx)?;

    save.XLR = ((2 as f64) * (save.RE + save.BOUNDS[[2, 3]]));
    save.XLT = save.XLR;
    save.XLZ = save.XLR;

    testutil::CHCKSD(b"LT", save.LT, b"~", save.XLT, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LR", save.LR, b"~", save.XLR, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LZ", save.LZ, b"~", save.XLZ, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Element is north polar cap with zero thickness.", ctx)?;

    save.RE = 3000.0;
    save.F = 0.5;
    save.RP = (save.RE * (1.0 - save.F));

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);

    save.BOUNDS[[1, 2]] = (spicelib::HALFPI(ctx) / 2 as f64);
    save.BOUNDS[[2, 2]] = spicelib::HALFPI(ctx);

    save.BOUNDS[[1, 3]] = 20.0;
    save.BOUNDS[[2, 3]] = 20.0;

    //
    // Find the coordinates of the bound of minimum longitude,
    // minimum latitude, and maximum altitude.
    //
    spicelib::GEOREC(
        save.BOUNDS[[1, 1]],
        save.BOUNDS[[1, 2]],
        save.BOUNDS[[2, 3]],
        save.RE,
        save.F,
        save.P.as_slice_mut(),
        ctx,
    )?;

    spicelib::VEQU(save.P.as_slice(), save.PXY.as_slice_mut());
    save.PXY[3] = 0.0;

    spicelib::ZZPDTBOX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.CENTER.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.RADIUS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CLEARD(3, save.XCTR.as_slice_mut());

    save.XCTR[3] = ((save.P[3] + (save.RP + save.BOUNDS[[2, 3]])) / 2 as f64);

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"CENTER",
        save.CENTER.as_slice(),
        b"~",
        save.XCTR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    save.XLR = (spicelib::VNORM(save.PXY.as_slice()) * 2 as f64);
    save.XLT = save.XLR;
    save.XLZ = ((save.RP + save.BOUNDS[[2, 3]]) - save.P[3]);

    testutil::CHCKSD(b"LT", save.LT, b"~", save.XLT, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LR", save.LR, b"~", save.XLR, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LZ", save.LZ, b"~", save.XLZ, save.TOL, OK, ctx)?;

    save.XRAD = f64::sqrt(
        ((f64::powi((save.XLR / 2 as f64), 2) + f64::powi((save.XLT / 2 as f64), 2))
            + f64::powi((save.XLZ / 2 as f64), 2)),
    );

    testutil::CHCKSD(b"RADIUS", save.RADIUS, b"~", save.XRAD, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Element is north polar cap with zero thickness. Prolate case.",
        ctx,
    )?;

    save.RE = 3000.0;
    save.F = -0.5;
    save.RP = (save.RE * (1.0 - save.F));

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);

    save.BOUNDS[[1, 2]] = (spicelib::HALFPI(ctx) / 2 as f64);
    save.BOUNDS[[2, 2]] = spicelib::HALFPI(ctx);

    save.BOUNDS[[1, 3]] = 20.0;
    save.BOUNDS[[2, 3]] = 20.0;

    //
    // Find the coordinates of the bound of minimum longitude,
    // minimum latitude, and maximum altitude.
    //
    spicelib::GEOREC(
        save.BOUNDS[[1, 1]],
        save.BOUNDS[[1, 2]],
        save.BOUNDS[[2, 3]],
        save.RE,
        save.F,
        save.P.as_slice_mut(),
        ctx,
    )?;

    spicelib::VEQU(save.P.as_slice(), save.PXY.as_slice_mut());
    save.PXY[3] = 0.0;

    spicelib::ZZPDTBOX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.CENTER.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.RADIUS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CLEARD(3, save.XCTR.as_slice_mut());

    save.XCTR[3] = ((save.P[3] + (save.RP + save.BOUNDS[[2, 3]])) / 2 as f64);

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"CENTER",
        save.CENTER.as_slice(),
        b"~",
        save.XCTR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    save.XLR = (spicelib::VNORM(save.PXY.as_slice()) * 2 as f64);
    save.XLT = save.XLR;
    save.XLZ = ((save.RP + save.BOUNDS[[2, 3]]) - save.P[3]);

    testutil::CHCKSD(b"LT", save.LT, b"~", save.XLT, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LR", save.LR, b"~", save.XLR, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LZ", save.LZ, b"~", save.XLZ, save.TOL, OK, ctx)?;

    save.XRAD = f64::sqrt(
        ((f64::powi((save.XLR / 2 as f64), 2) + f64::powi((save.XLT / 2 as f64), 2))
            + f64::powi((save.XLZ / 2 as f64), 2)),
    );

    testutil::CHCKSD(b"RADIUS", save.RADIUS, b"~", save.XRAD, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Element is north polar cap with non-zero thickness.", ctx)?;

    save.RE = 3000.0;
    save.F = 0.5;
    save.RP = (save.RE * (1.0 - save.F));

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);

    save.BOUNDS[[1, 2]] = (spicelib::HALFPI(ctx) / 2 as f64);
    save.BOUNDS[[2, 2]] = spicelib::HALFPI(ctx);

    save.BOUNDS[[1, 3]] = 19.0;
    save.BOUNDS[[2, 3]] = 20.0;

    //
    // Find the coordinates of the bound of minimum longitude,
    // minimum latitude, and maximum altitude.
    //
    spicelib::GEOREC(
        save.BOUNDS[[1, 1]],
        save.BOUNDS[[1, 2]],
        save.BOUNDS[[2, 3]],
        save.RE,
        save.F,
        save.PMAX.as_slice_mut(),
        ctx,
    )?;

    spicelib::VEQU(save.PMAX.as_slice(), save.PMAXXY.as_slice_mut());
    save.PMAXXY[3] = 0.0;

    //
    // Find the coordinates of the bound of minimum longitude,
    // minimum latitude, and minimum altitude.
    //
    spicelib::GEOREC(
        save.BOUNDS[[1, 1]],
        save.BOUNDS[[1, 2]],
        save.BOUNDS[[1, 3]],
        save.RE,
        save.F,
        save.PMIN.as_slice_mut(),
        ctx,
    )?;

    spicelib::VEQU(save.PMIN.as_slice(), save.PMINXY.as_slice_mut());
    save.PMINXY[3] = 0.0;

    spicelib::ZZPDTBOX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.CENTER.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.RADIUS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CLEARD(3, save.XCTR.as_slice_mut());
    save.XCTR[3] = ((save.PMIN[3] + (save.RP + save.BOUNDS[[2, 3]])) / 2 as f64);

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"CENTER",
        save.CENTER.as_slice(),
        b"~",
        save.XCTR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    save.XLR = (spicelib::VNORM(save.PMAXXY.as_slice()) * 2 as f64);
    save.XLT = save.XLR;
    save.XLZ = ((save.RP + save.BOUNDS[[2, 3]]) - save.PMIN[3]);

    testutil::CHCKSD(b"LT", save.LT, b"~", save.XLT, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LR", save.LR, b"~", save.XLR, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LZ", save.LZ, b"~", save.XLZ, save.TOL, OK, ctx)?;

    save.XRAD = f64::sqrt(
        ((f64::powi((save.XLR / 2 as f64), 2) + f64::powi((save.XLT / 2 as f64), 2))
            + f64::powi((save.XLZ / 2 as f64), 2)),
    );

    testutil::CHCKSD(b"RADIUS", save.RADIUS, b"~", save.XRAD, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Element is north polar cap with non-zero thickness. Prolate case.",
        ctx,
    )?;

    save.RE = 3000.0;
    save.F = -0.5;
    save.RP = (save.RE * (1.0 - save.F));

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);

    save.BOUNDS[[1, 2]] = (spicelib::HALFPI(ctx) / 2 as f64);
    save.BOUNDS[[2, 2]] = spicelib::HALFPI(ctx);

    save.BOUNDS[[1, 3]] = 19.0;
    save.BOUNDS[[2, 3]] = 20.0;

    //
    // Find the coordinates of the bound of minimum longitude,
    // minimum latitude, and maximum altitude.
    //
    spicelib::GEOREC(
        save.BOUNDS[[1, 1]],
        save.BOUNDS[[1, 2]],
        save.BOUNDS[[2, 3]],
        save.RE,
        save.F,
        save.PMAX.as_slice_mut(),
        ctx,
    )?;

    spicelib::VEQU(save.PMAX.as_slice(), save.PMAXXY.as_slice_mut());
    save.PMAXXY[3] = 0.0;

    //
    // Find the coordinates of the bound of minimum longitude,
    // minimum latitude, and minimum altitude.
    //
    spicelib::GEOREC(
        save.BOUNDS[[1, 1]],
        save.BOUNDS[[1, 2]],
        save.BOUNDS[[1, 3]],
        save.RE,
        save.F,
        save.PMIN.as_slice_mut(),
        ctx,
    )?;

    spicelib::VEQU(save.PMIN.as_slice(), save.PMINXY.as_slice_mut());
    save.PMINXY[3] = 0.0;

    spicelib::ZZPDTBOX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.CENTER.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.RADIUS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CLEARD(3, save.XCTR.as_slice_mut());
    save.XCTR[3] = ((save.PMIN[3] + (save.RP + save.BOUNDS[[2, 3]])) / 2 as f64);

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"CENTER",
        save.CENTER.as_slice(),
        b"~",
        save.XCTR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    save.XLR = (spicelib::VNORM(save.PMAXXY.as_slice()) * 2 as f64);
    save.XLT = save.XLR;
    save.XLZ = ((save.RP + save.BOUNDS[[2, 3]]) - save.PMIN[3]);

    testutil::CHCKSD(b"LT", save.LT, b"~", save.XLT, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LR", save.LR, b"~", save.XLR, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LZ", save.LZ, b"~", save.XLZ, save.TOL, OK, ctx)?;

    save.XRAD = f64::sqrt(
        ((f64::powi((save.XLR / 2 as f64), 2) + f64::powi((save.XLT / 2 as f64), 2))
            + f64::powi((save.XLZ / 2 as f64), 2)),
    );

    testutil::CHCKSD(b"RADIUS", save.RADIUS, b"~", save.XRAD, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Element is south polar cap with zero thickness.", ctx)?;

    save.RE = 3000.0;
    save.F = 0.5;
    save.RP = (save.RE * (1.0 - save.F));

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);

    save.BOUNDS[[1, 2]] = -spicelib::HALFPI(ctx);
    save.BOUNDS[[2, 2]] = -(spicelib::HALFPI(ctx) / 2 as f64);

    save.BOUNDS[[1, 3]] = 20.0;
    save.BOUNDS[[2, 3]] = 20.0;

    //
    // Find the coordinates of the bound of minimum longitude,
    // maximum latitude, and maximum altitude.
    //
    spicelib::GEOREC(
        save.BOUNDS[[1, 1]],
        save.BOUNDS[[2, 2]],
        save.BOUNDS[[2, 3]],
        save.RE,
        save.F,
        save.P.as_slice_mut(),
        ctx,
    )?;

    spicelib::VEQU(save.P.as_slice(), save.PXY.as_slice_mut());
    save.PXY[3] = 0.0;

    spicelib::ZZPDTBOX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.CENTER.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.RADIUS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CLEARD(3, save.XCTR.as_slice_mut());
    save.XCTR[3] = ((save.P[3] + (-save.RP - save.BOUNDS[[2, 3]])) / 2 as f64);

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"CENTER",
        save.CENTER.as_slice(),
        b"~",
        save.XCTR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    save.XLR = ((2 as f64) * spicelib::VNORM(save.PXY.as_slice()));
    save.XLT = save.XLR;
    save.XLZ = ((2 as f64) * f64::abs((save.XCTR[3] - save.P[3])));

    testutil::CHCKSD(b"LT", save.LT, b"~", save.XLT, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LR", save.LR, b"~", save.XLR, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LZ", save.LZ, b"~", save.XLZ, save.TOL, OK, ctx)?;

    save.XRAD = f64::sqrt(
        ((f64::powi((save.XLR / 2 as f64), 2) + f64::powi((save.XLT / 2 as f64), 2))
            + f64::powi((save.XLZ / 2 as f64), 2)),
    );

    testutil::CHCKSD(b"RADIUS", save.RADIUS, b"~", save.XRAD, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Element is south polar cap with zero thickness. Prolate case.",
        ctx,
    )?;

    save.RE = 3000.0;
    save.F = -0.5;
    save.RP = (save.RE * (1.0 - save.F));

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);

    save.BOUNDS[[1, 2]] = -spicelib::HALFPI(ctx);
    save.BOUNDS[[2, 2]] = -(spicelib::HALFPI(ctx) / 2 as f64);

    save.BOUNDS[[1, 3]] = 20.0;
    save.BOUNDS[[2, 3]] = 20.0;

    //
    // Find the coordinates of the bound of minimum longitude,
    // maximum latitude, and maximum altitude.
    //
    spicelib::GEOREC(
        save.BOUNDS[[1, 1]],
        save.BOUNDS[[2, 2]],
        save.BOUNDS[[2, 3]],
        save.RE,
        save.F,
        save.P.as_slice_mut(),
        ctx,
    )?;

    spicelib::VEQU(save.P.as_slice(), save.PXY.as_slice_mut());
    save.PXY[3] = 0.0;

    spicelib::ZZPDTBOX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.CENTER.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.RADIUS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CLEARD(3, save.XCTR.as_slice_mut());
    save.XCTR[3] = ((save.P[3] + (-save.RP - save.BOUNDS[[2, 3]])) / 2 as f64);

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"CENTER",
        save.CENTER.as_slice(),
        b"~",
        save.XCTR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    save.XLR = ((2 as f64) * spicelib::VNORM(save.PXY.as_slice()));
    save.XLT = save.XLR;
    save.XLZ = ((2 as f64) * f64::abs((save.XCTR[3] - save.P[3])));

    testutil::CHCKSD(b"LT", save.LT, b"~", save.XLT, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LR", save.LR, b"~", save.XLR, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LZ", save.LZ, b"~", save.XLZ, save.TOL, OK, ctx)?;

    save.XRAD = f64::sqrt(
        ((f64::powi((save.XLR / 2 as f64), 2) + f64::powi((save.XLT / 2 as f64), 2))
            + f64::powi((save.XLZ / 2 as f64), 2)),
    );

    testutil::CHCKSD(b"RADIUS", save.RADIUS, b"~", save.XRAD, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Element is south polar cap with non-zero thickness.", ctx)?;

    save.RE = 3000.0;
    save.F = 0.5;
    save.RP = (save.RE * (1.0 - save.F));

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);

    save.BOUNDS[[1, 2]] = -spicelib::HALFPI(ctx);
    save.BOUNDS[[2, 2]] = -(spicelib::HALFPI(ctx) / 2 as f64);

    save.BOUNDS[[1, 3]] = 19.0;
    save.BOUNDS[[2, 3]] = 20.0;

    //
    // Find the coordinates of the bound of minimum longitude,
    // maximum latitude, and maximum altitude.
    //
    spicelib::GEOREC(
        save.BOUNDS[[1, 1]],
        save.BOUNDS[[2, 2]],
        save.BOUNDS[[2, 3]],
        save.RE,
        save.F,
        save.PMAX.as_slice_mut(),
        ctx,
    )?;

    spicelib::VEQU(save.PMAX.as_slice(), save.PMAXXY.as_slice_mut());
    save.PMAXXY[3] = 0.0;

    //
    // Find the coordinates of the bound of minimum longitude,
    // maximum latitude, and minimum altitude.
    //
    spicelib::GEOREC(
        save.BOUNDS[[1, 1]],
        save.BOUNDS[[2, 2]],
        save.BOUNDS[[1, 3]],
        save.RE,
        save.F,
        save.PMIN.as_slice_mut(),
        ctx,
    )?;

    spicelib::VEQU(save.PMIN.as_slice(), save.PMINXY.as_slice_mut());
    save.PMINXY[3] = 0.0;

    spicelib::ZZPDTBOX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.CENTER.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.RADIUS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CLEARD(3, save.XCTR.as_slice_mut());

    save.XCTR[3] = ((save.PMIN[3] + (-save.RP - save.BOUNDS[[2, 3]])) / 2 as f64);

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"CENTER",
        save.CENTER.as_slice(),
        b"~",
        save.XCTR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    save.XLR = ((2 as f64) * spicelib::VNORM(save.PMAXXY.as_slice()));
    save.XLT = save.XLR;
    save.XLZ = (save.PMIN[3] - (-save.RP - save.BOUNDS[[2, 3]]));

    testutil::CHCKSD(b"LT", save.LT, b"~", save.XLT, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LR", save.LR, b"~", save.XLR, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LZ", save.LZ, b"~", save.XLZ, save.TOL, OK, ctx)?;

    save.XRAD = f64::sqrt(
        ((f64::powi((save.XLR / 2 as f64), 2) + f64::powi((save.XLT / 2 as f64), 2))
            + f64::powi((save.XLZ / 2 as f64), 2)),
    );

    testutil::CHCKSD(b"RADIUS", save.RADIUS, b"~", save.XRAD, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Element is south polar cap with non-zero thickness. Prolate case.",
        ctx,
    )?;

    save.RE = 3000.0;
    save.F = -0.5;
    save.RP = (save.RE * (1.0 - save.F));

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);

    save.BOUNDS[[1, 2]] = -spicelib::HALFPI(ctx);
    save.BOUNDS[[2, 2]] = -(spicelib::HALFPI(ctx) / 2 as f64);

    save.BOUNDS[[1, 3]] = 19.0;
    save.BOUNDS[[2, 3]] = 20.0;

    //
    // Find the coordinates of the bound of minimum longitude,
    // maximum latitude, and maximum altitude.
    //
    spicelib::GEOREC(
        save.BOUNDS[[1, 1]],
        save.BOUNDS[[2, 2]],
        save.BOUNDS[[2, 3]],
        save.RE,
        save.F,
        save.PMAX.as_slice_mut(),
        ctx,
    )?;

    spicelib::VEQU(save.PMAX.as_slice(), save.PMAXXY.as_slice_mut());
    save.PMAXXY[3] = 0.0;

    //
    // Find the coordinates of the bound of minimum longitude,
    // maximum latitude, and minimum altitude.
    //
    spicelib::GEOREC(
        save.BOUNDS[[1, 1]],
        save.BOUNDS[[2, 2]],
        save.BOUNDS[[1, 3]],
        save.RE,
        save.F,
        save.PMIN.as_slice_mut(),
        ctx,
    )?;

    spicelib::VEQU(save.PMIN.as_slice(), save.PMINXY.as_slice_mut());
    save.PMINXY[3] = 0.0;

    spicelib::ZZPDTBOX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.CENTER.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.RADIUS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CLEARD(3, save.XCTR.as_slice_mut());

    save.XCTR[3] = ((save.PMIN[3] + (-save.RP - save.BOUNDS[[2, 3]])) / 2 as f64);

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"CENTER",
        save.CENTER.as_slice(),
        b"~",
        save.XCTR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    save.XLR = ((2 as f64) * spicelib::VNORM(save.PMAXXY.as_slice()));
    save.XLT = save.XLR;
    save.XLZ = (save.PMIN[3] - (-save.RP - save.BOUNDS[[2, 3]]));

    testutil::CHCKSD(b"LT", save.LT, b"~", save.XLT, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LR", save.LR, b"~", save.XLR, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LZ", save.LZ, b"~", save.XLZ, save.TOL, OK, ctx)?;

    save.XRAD = f64::sqrt(
        ((f64::powi((save.XLR / 2 as f64), 2) + f64::powi((save.XLT / 2 as f64), 2))
            + f64::powi((save.XLZ / 2 as f64), 2)),
    );

    testutil::CHCKSD(b"RADIUS", save.RADIUS, b"~", save.XRAD, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Element is chunk above the X-Y plane, lying over the +X axis.",
        ctx,
    )?;

    save.RE = 3000.0;
    save.F = 0.5;
    save.RP = (save.RE * (1.0 - save.F));

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 3 as f64);

    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 3 as f64);

    save.BOUNDS[[1, 3]] = 10.0;
    save.BOUNDS[[2, 3]] = 20.0;

    spicelib::ZZPDTBOX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.CENTER.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.RADIUS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Generate expected outputs.
    //
    T_TSTPBX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.XCTR.as_slice_mut(),
        &mut save.XLR,
        &mut save.XLT,
        &mut save.XLZ,
        &mut save.XRAD,
        ctx,
    )?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"CENTER",
        save.CENTER.as_slice(),
        b"~",
        save.XCTR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    testutil::CHCKSD(b"LT", save.LT, b"~", save.XLT, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LR", save.LR, b"~", save.XLR, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LZ", save.LZ, b"~", save.XLZ, save.TOL, OK, ctx)?;

    save.XRAD = f64::sqrt(
        ((f64::powi((save.XLR / 2 as f64), 2) + f64::powi((save.XLT / 2 as f64), 2))
            + f64::powi((save.XLZ / 2 as f64), 2)),
    );

    testutil::CHCKSD(b"RADIUS", save.RADIUS, b"~", save.XRAD, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Element is chunk above the X-Y plane, lying over the +X axis. Prolate case.",
        ctx,
    )?;

    save.RE = 3000.0;
    save.F = -0.5;
    save.RP = (save.RE * (1.0 - save.F));

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 3 as f64);

    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 3 as f64);

    save.BOUNDS[[1, 3]] = 10.0;
    save.BOUNDS[[2, 3]] = 20.0;

    spicelib::ZZPDTBOX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.CENTER.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.RADIUS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Generate expected outputs.
    //
    T_TSTPBX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.XCTR.as_slice_mut(),
        &mut save.XLR,
        &mut save.XLT,
        &mut save.XLZ,
        &mut save.XRAD,
        ctx,
    )?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"CENTER",
        save.CENTER.as_slice(),
        b"~",
        save.XCTR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    testutil::CHCKSD(b"LT", save.LT, b"~", save.XLT, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LR", save.LR, b"~", save.XLR, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LZ", save.LZ, b"~", save.XLZ, save.TOL, OK, ctx)?;

    save.XRAD = f64::sqrt(
        ((f64::powi((save.XLR / 2 as f64), 2) + f64::powi((save.XLT / 2 as f64), 2))
            + f64::powi((save.XLZ / 2 as f64), 2)),
    );

    testutil::CHCKSD(b"RADIUS", save.RADIUS, b"~", save.XRAD, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Element is chunk below the X-Y plane, lying under the +X axis.",
        ctx,
    )?;

    save.RE = 3000.0;
    save.F = 0.5;
    save.RP = (save.RE * (1.0 - save.F));

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 3 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 10.0;
    save.BOUNDS[[2, 3]] = 20.0;

    spicelib::ZZPDTBOX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.CENTER.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.RADIUS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Generate expected outputs.
    //
    T_TSTPBX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.XCTR.as_slice_mut(),
        &mut save.XLR,
        &mut save.XLT,
        &mut save.XLZ,
        &mut save.XRAD,
        ctx,
    )?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"CENTER",
        save.CENTER.as_slice(),
        b"~",
        save.XCTR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    testutil::CHCKSD(b"LT", save.LT, b"~", save.XLT, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LR", save.LR, b"~", save.XLR, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LZ", save.LZ, b"~", save.XLZ, save.TOL, OK, ctx)?;

    save.XRAD = f64::sqrt(
        ((f64::powi((save.XLR / 2 as f64), 2) + f64::powi((save.XLT / 2 as f64), 2))
            + f64::powi((save.XLZ / 2 as f64), 2)),
    );

    testutil::CHCKSD(b"RADIUS", save.RADIUS, b"~", save.XRAD, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Element is chunk below the X-Y plane, lying under the +X axis. Prolate case.",
        ctx,
    )?;

    save.RE = 3000.0;
    save.F = -0.5;
    save.RP = (save.RE * (1.0 - save.F));

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 3 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 10.0;
    save.BOUNDS[[2, 3]] = 20.0;

    spicelib::ZZPDTBOX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.CENTER.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.RADIUS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Generate expected outputs.
    //
    T_TSTPBX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.XCTR.as_slice_mut(),
        &mut save.XLR,
        &mut save.XLT,
        &mut save.XLZ,
        &mut save.XRAD,
        ctx,
    )?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"CENTER",
        save.CENTER.as_slice(),
        b"~",
        save.XCTR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    testutil::CHCKSD(b"LT", save.LT, b"~", save.XLT, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LR", save.LR, b"~", save.XLR, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LZ", save.LZ, b"~", save.XLZ, save.TOL, OK, ctx)?;

    save.XRAD = f64::sqrt(
        ((f64::powi((save.XLR / 2 as f64), 2) + f64::powi((save.XLT / 2 as f64), 2))
            + f64::powi((save.XLZ / 2 as f64), 2)),
    );

    testutil::CHCKSD(b"RADIUS", save.RADIUS, b"~", save.XRAD, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Element is chunk above the X-Y plane, lying over the -X axis. Longitude extent is > pi.",
        ctx,
    )?;

    save.RE = 3000.0;
    save.F = 0.5;
    save.RP = (save.RE * (1.0 - save.F));

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.BOUNDS[[1, 1]] = (spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 1]] = -(spicelib::PI(ctx) / 3 as f64);

    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 3 as f64);

    save.BOUNDS[[1, 3]] = 10.0;
    save.BOUNDS[[2, 3]] = 20.0;

    spicelib::ZZPDTBOX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.CENTER.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.RADIUS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Generate expected outputs.
    //
    T_TSTPBX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.XCTR.as_slice_mut(),
        &mut save.XLR,
        &mut save.XLT,
        &mut save.XLZ,
        &mut save.XRAD,
        ctx,
    )?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"CENTER",
        save.CENTER.as_slice(),
        b"~",
        save.XCTR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    testutil::CHCKSD(b"LT", save.LT, b"~", save.XLT, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LR", save.LR, b"~", save.XLR, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LZ", save.LZ, b"~", save.XLZ, save.TOL, OK, ctx)?;

    save.XRAD = f64::sqrt(
        ((f64::powi((save.XLR / 2 as f64), 2) + f64::powi((save.XLT / 2 as f64), 2))
            + f64::powi((save.XLZ / 2 as f64), 2)),
    );

    testutil::CHCKSD(b"RADIUS", save.RADIUS, b"~", save.XRAD, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Element is chunk below the X-Y plane, lying under the -X axis. Longitude extent is > pi.",
        ctx,
    )?;

    save.RE = 3000.0;
    save.F = 0.5;
    save.RP = (save.RE * (1.0 - save.F));

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.BOUNDS[[1, 1]] = (spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 1]] = -(spicelib::PI(ctx) / 3 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 10.0;
    save.BOUNDS[[2, 3]] = 20.0;

    spicelib::ZZPDTBOX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.CENTER.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.RADIUS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Generate expected outputs.
    //
    T_TSTPBX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.XCTR.as_slice_mut(),
        &mut save.XLR,
        &mut save.XLT,
        &mut save.XLZ,
        &mut save.XRAD,
        ctx,
    )?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"CENTER",
        save.CENTER.as_slice(),
        b"~",
        save.XCTR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    testutil::CHCKSD(b"LT", save.LT, b"~", save.XLT, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LR", save.LR, b"~", save.XLR, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LZ", save.LZ, b"~", save.XLZ, save.TOL, OK, ctx)?;

    save.XRAD = f64::sqrt(
        ((f64::powi((save.XLR / 2 as f64), 2) + f64::powi((save.XLT / 2 as f64), 2))
            + f64::powi((save.XLZ / 2 as f64), 2)),
    );

    testutil::CHCKSD(b"RADIUS", save.RADIUS, b"~", save.XRAD, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Element is an equatorial belt that is symmetric about the X-Y plane.",
        ctx,
    )?;

    save.RE = 3000.0;
    save.F = 0.5;
    save.RP = (save.RE * (1.0 - save.F));

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 3 as f64);

    save.BOUNDS[[1, 3]] = 10.0;
    save.BOUNDS[[2, 3]] = 20.0;

    spicelib::ZZPDTBOX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.CENTER.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.RADIUS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Generate expected outputs.
    //
    T_TSTPBX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.XCTR.as_slice_mut(),
        &mut save.XLR,
        &mut save.XLT,
        &mut save.XLZ,
        &mut save.XRAD,
        ctx,
    )?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"CENTER",
        save.CENTER.as_slice(),
        b"~",
        save.XCTR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    testutil::CHCKSD(b"LT", save.LT, b"~", save.XLT, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LR", save.LR, b"~", save.XLR, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LZ", save.LZ, b"~", save.XLZ, save.TOL, OK, ctx)?;

    save.XRAD = f64::sqrt(
        ((f64::powi((save.XLR / 2 as f64), 2) + f64::powi((save.XLT / 2 as f64), 2))
            + f64::powi((save.XLZ / 2 as f64), 2)),
    );

    testutil::CHCKSD(b"RADIUS", save.RADIUS, b"~", save.XRAD, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Element is an equatorial belt that is asymmetric about the X-Y plane.",
        ctx,
    )?;

    save.RE = 3000.0;
    save.F = 0.5;
    save.RP = (save.RE * (1.0 - save.F));

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 3 as f64);

    save.BOUNDS[[1, 3]] = 10.0;
    save.BOUNDS[[2, 3]] = 20.0;

    spicelib::ZZPDTBOX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.CENTER.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.RADIUS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Generate expected outputs.
    //
    T_TSTPBX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.XCTR.as_slice_mut(),
        &mut save.XLR,
        &mut save.XLT,
        &mut save.XLZ,
        &mut save.XRAD,
        ctx,
    )?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"CENTER",
        save.CENTER.as_slice(),
        b"~",
        save.XCTR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    testutil::CHCKSD(b"LT", save.LT, b"~", save.XLT, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LR", save.LR, b"~", save.XLR, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LZ", save.LZ, b"~", save.XLZ, save.TOL, OK, ctx)?;

    save.XRAD = f64::sqrt(
        ((f64::powi((save.XLR / 2 as f64), 2) + f64::powi((save.XLT / 2 as f64), 2))
            + f64::powi((save.XLZ / 2 as f64), 2)),
    );

    testutil::CHCKSD(b"RADIUS", save.RADIUS, b"~", save.XRAD, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Element is a slice ranging from the north to the south pole. Longitude extent is < pi.",
        ctx,
    )?;

    save.RE = 3000.0;
    save.F = 0.5;
    save.RP = (save.RE * (1.0 - save.F));

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.BOUNDS[[1, 1]] = (((3 as f64) * spicelib::PI(ctx)) / 4 as f64);
    save.BOUNDS[[2, 1]] = -(((3 as f64) * spicelib::PI(ctx)) / 4 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 2 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 2 as f64);

    save.BOUNDS[[1, 3]] = 1.0;
    save.BOUNDS[[2, 3]] = 20.0;

    spicelib::ZZPDTBOX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.CENTER.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.RADIUS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Generate expected outputs.
    //
    T_TSTPBX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.XCTR.as_slice_mut(),
        &mut save.XLR,
        &mut save.XLT,
        &mut save.XLZ,
        &mut save.XRAD,
        ctx,
    )?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"CENTER",
        save.CENTER.as_slice(),
        b"~",
        save.XCTR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    testutil::CHCKSD(b"LT", save.LT, b"~", save.XLT, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LR", save.LR, b"~", save.XLR, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LZ", save.LZ, b"~", save.XLZ, save.TOL, OK, ctx)?;

    save.XRAD = f64::sqrt(
        ((f64::powi((save.XLR / 2 as f64), 2) + f64::powi((save.XLT / 2 as f64), 2))
            + f64::powi((save.XLZ / 2 as f64), 2)),
    );

    testutil::CHCKSD(b"RADIUS", save.RADIUS, b"~", save.XRAD, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Element is a slice ranging from the north to the south pole. Longitude extent is > pi.",
        ctx,
    )?;

    save.BOUNDS[[1, 1]] = (spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 1]] = -(spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 2 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 2 as f64);

    save.BOUNDS[[1, 3]] = 1.0;
    save.BOUNDS[[2, 3]] = 20.0;

    spicelib::ZZPDTBOX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.CENTER.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.RADIUS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Generate expected outputs.
    //
    T_TSTPBX(
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.XCTR.as_slice_mut(),
        &mut save.XLR,
        &mut save.XLT,
        &mut save.XLZ,
        &mut save.XRAD,
        ctx,
    )?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"CENTER",
        save.CENTER.as_slice(),
        b"~",
        save.XCTR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    testutil::CHCKSD(b"LT", save.LT, b"~", save.XLT, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LR", save.LR, b"~", save.XLR, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LZ", save.LZ, b"~", save.XLZ, save.TOL, OK, ctx)?;

    save.XRAD = f64::sqrt(
        ((f64::powi((save.XLR / 2 as f64), 2) + f64::powi((save.XLT / 2 as f64), 2))
            + f64::powi((save.XLZ / 2 as f64), 2)),
    );

    testutil::CHCKSD(b"RADIUS", save.RADIUS, b"~", save.XRAD, save.TOL, OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
