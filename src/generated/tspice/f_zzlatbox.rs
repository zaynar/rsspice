//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const TIGHT: f64 = 0.000000000001;

struct SaveVars {
    BOUNDS: StackArray2D<f64, 6>,
    CENTER: StackArray<f64, 3>,
    LR: f64,
    LT: f64,
    LZ: f64,
    RADIUS: f64,
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
        let mut LR: f64 = 0.0;
        let mut LT: f64 = 0.0;
        let mut LZ: f64 = 0.0;
        let mut RADIUS: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut XCTR = StackArray::<f64, 3>::new(1..=3);
        let mut XLR: f64 = 0.0;
        let mut XLT: f64 = 0.0;
        let mut XLZ: f64 = 0.0;
        let mut XRAD: f64 = 0.0;

        Self {
            BOUNDS,
            CENTER,
            LR,
            LT,
            LZ,
            RADIUS,
            TOL,
            XCTR,
            XLR,
            XLT,
            XLZ,
            XRAD,
        }
    }
}

//$Procedure F_ZZLATBOX ( ZZLATBOX tests )
pub fn F_ZZLATBOX(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    testutil::TOPEN(b"F_ZZLATBOX", ctx)?;

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

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = (-spicelib::TWOPI(ctx) - 0.000000000000001);

    save.BOUNDS[[1, 2]] = -spicelib::HALFPI(ctx);
    save.BOUNDS[[2, 2]] = spicelib::HALFPI(ctx);

    save.BOUNDS[[1, 3]] = 10.0;
    save.BOUNDS[[2, 3]] = 20.0;

    spicelib::ZZLATBOX(
        save.BOUNDS.as_slice(),
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

    spicelib::ZZLATBOX(
        save.BOUNDS.as_slice(),
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

    spicelib::ZZLATBOX(
        save.BOUNDS.as_slice(),
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

    spicelib::ZZLATBOX(
        save.BOUNDS.as_slice(),
        save.CENTER.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.RADIUS,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADLATITUDERANGE)", OK, ctx)?;

    //**********************************************************************
    //
    //     Normal cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Trivial case: element is sphere.", ctx)?;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);

    save.BOUNDS[[1, 2]] = -spicelib::HALFPI(ctx);
    save.BOUNDS[[2, 2]] = spicelib::HALFPI(ctx);

    save.BOUNDS[[1, 3]] = 10.0;
    save.BOUNDS[[2, 3]] = 20.0;

    spicelib::ZZLATBOX(
        save.BOUNDS.as_slice(),
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

    save.XRAD = (save.BOUNDS[[2, 3]] * f64::sqrt(3.0));

    testutil::CHCKSD(b"RADIUS", save.RADIUS, b"~", save.XRAD, save.TOL, OK, ctx)?;

    save.XLR = ((2 as f64) * save.BOUNDS[[2, 3]]);
    save.XLT = save.XLR;
    save.XLZ = save.XLR;

    testutil::CHCKSD(b"LT", save.LT, b"~", save.XLT, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LR", save.LR, b"~", save.XLR, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LZ", save.LZ, b"~", save.XLZ, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Element is north polar cap with zero thickness.", ctx)?;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);

    save.BOUNDS[[1, 2]] = (spicelib::HALFPI(ctx) / 2 as f64);
    save.BOUNDS[[2, 2]] = spicelib::HALFPI(ctx);

    save.BOUNDS[[1, 3]] = 20.0;
    save.BOUNDS[[2, 3]] = 20.0;

    spicelib::ZZLATBOX(
        save.BOUNDS.as_slice(),
        save.CENTER.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.RADIUS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CLEARD(3, save.XCTR.as_slice_mut());
    save.XCTR[3] = (save.BOUNDS[[2, 3]] * ((1.0 + (f64::sqrt(2.0) / 2 as f64)) / 2 as f64));

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

    save.XLR = (((save.BOUNDS[[2, 3]] * f64::sqrt(2.0)) / 2 as f64) * 2 as f64);
    save.XLT = save.XLR;
    save.XLZ = (save.BOUNDS[[2, 3]] * (1.0 - (f64::sqrt(2.0) / 2 as f64)));

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

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);

    save.BOUNDS[[1, 2]] = (spicelib::HALFPI(ctx) / 2 as f64);
    save.BOUNDS[[2, 2]] = spicelib::HALFPI(ctx);

    save.BOUNDS[[1, 3]] = 19.0;
    save.BOUNDS[[2, 3]] = 20.0;

    spicelib::ZZLATBOX(
        save.BOUNDS.as_slice(),
        save.CENTER.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.RADIUS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CLEARD(3, save.XCTR.as_slice_mut());
    save.XCTR[3] =
        ((save.BOUNDS[[2, 3]] + ((save.BOUNDS[[1, 3]] * f64::sqrt(2.0)) / 2 as f64)) / 2 as f64);

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

    save.XLR = (((save.BOUNDS[[2, 3]] * f64::sqrt(2.0)) / 2 as f64) * 2 as f64);
    save.XLT = save.XLR;
    save.XLZ = (save.BOUNDS[[2, 3]] - ((save.BOUNDS[[1, 3]] * f64::sqrt(2.0)) / 2 as f64));

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

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);

    save.BOUNDS[[1, 2]] = -spicelib::HALFPI(ctx);
    save.BOUNDS[[2, 2]] = -(spicelib::HALFPI(ctx) / 2 as f64);

    save.BOUNDS[[1, 3]] = 20.0;
    save.BOUNDS[[2, 3]] = 20.0;

    spicelib::ZZLATBOX(
        save.BOUNDS.as_slice(),
        save.CENTER.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.RADIUS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CLEARD(3, save.XCTR.as_slice_mut());
    save.XCTR[3] = -(save.BOUNDS[[2, 3]] * ((1.0 + (f64::sqrt(2.0) / 2 as f64)) / 2 as f64));

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

    save.XLR = (((save.BOUNDS[[2, 3]] * f64::sqrt(2.0)) / 2 as f64) * 2 as f64);
    save.XLT = save.XLR;
    save.XLZ = (save.BOUNDS[[2, 3]] * (1.0 - (f64::sqrt(2.0) / 2 as f64)));

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

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);

    save.BOUNDS[[1, 2]] = -spicelib::HALFPI(ctx);
    save.BOUNDS[[2, 2]] = -(spicelib::HALFPI(ctx) / 2 as f64);

    save.BOUNDS[[1, 3]] = 19.0;
    save.BOUNDS[[2, 3]] = 20.0;

    spicelib::ZZLATBOX(
        save.BOUNDS.as_slice(),
        save.CENTER.as_slice_mut(),
        &mut save.LR,
        &mut save.LT,
        &mut save.LZ,
        &mut save.RADIUS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CLEARD(3, save.XCTR.as_slice_mut());
    save.XCTR[3] =
        -((save.BOUNDS[[2, 3]] + ((save.BOUNDS[[1, 3]] * f64::sqrt(2.0)) / 2 as f64)) / 2 as f64);

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

    save.XLR = (((save.BOUNDS[[2, 3]] * f64::sqrt(2.0)) / 2 as f64) * 2 as f64);
    save.XLT = save.XLR;
    save.XLZ = (save.BOUNDS[[2, 3]] - ((save.BOUNDS[[1, 3]] * f64::sqrt(2.0)) / 2 as f64));

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

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 3 as f64);

    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 3 as f64);

    save.BOUNDS[[1, 3]] = 10.0;
    save.BOUNDS[[2, 3]] = 20.0;

    spicelib::ZZLATBOX(
        save.BOUNDS.as_slice(),
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
    T_TSTBOX(
        save.BOUNDS.as_slice(),
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

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 3 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 10.0;
    save.BOUNDS[[2, 3]] = 20.0;

    spicelib::ZZLATBOX(
        save.BOUNDS.as_slice(),
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
    T_TSTBOX(
        save.BOUNDS.as_slice(),
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

    save.BOUNDS[[1, 1]] = (spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 1]] = -(spicelib::PI(ctx) / 3 as f64);

    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 3 as f64);

    save.BOUNDS[[1, 3]] = 10.0;
    save.BOUNDS[[2, 3]] = 20.0;

    spicelib::ZZLATBOX(
        save.BOUNDS.as_slice(),
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
    T_TSTBOX(
        save.BOUNDS.as_slice(),
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

    save.BOUNDS[[1, 1]] = (spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 1]] = -(spicelib::PI(ctx) / 3 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 10.0;
    save.BOUNDS[[2, 3]] = 20.0;

    spicelib::ZZLATBOX(
        save.BOUNDS.as_slice(),
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
    T_TSTBOX(
        save.BOUNDS.as_slice(),
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

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 3 as f64);

    save.BOUNDS[[1, 3]] = 10.0;
    save.BOUNDS[[2, 3]] = 20.0;

    spicelib::ZZLATBOX(
        save.BOUNDS.as_slice(),
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
    T_TSTBOX(
        save.BOUNDS.as_slice(),
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

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 3 as f64);

    save.BOUNDS[[1, 3]] = 10.0;
    save.BOUNDS[[2, 3]] = 20.0;

    spicelib::ZZLATBOX(
        save.BOUNDS.as_slice(),
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
    T_TSTBOX(
        save.BOUNDS.as_slice(),
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

    save.BOUNDS[[1, 1]] = (((3 as f64) * spicelib::PI(ctx)) / 4 as f64);
    save.BOUNDS[[2, 1]] = -(((3 as f64) * spicelib::PI(ctx)) / 4 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 2 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 2 as f64);

    save.BOUNDS[[1, 3]] = 1.0;
    save.BOUNDS[[2, 3]] = 20.0;

    spicelib::ZZLATBOX(
        save.BOUNDS.as_slice(),
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
    T_TSTBOX(
        save.BOUNDS.as_slice(),
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

    spicelib::ZZLATBOX(
        save.BOUNDS.as_slice(),
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
    T_TSTBOX(
        save.BOUNDS.as_slice(),
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
