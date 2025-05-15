//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const VTIGHT: f64 = 0.00000000000001;

struct SaveVars {
    ATOL: f64,
    INMAX: f64,
    INMIN: f64,
    OUTMAX: f64,
    OUTMIN: f64,
    SMALL: f64,
    TOL: f64,
    XOMAX: f64,
    XOMIN: f64,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ATOL: f64 = 0.0;
        let mut INMAX: f64 = 0.0;
        let mut INMIN: f64 = 0.0;
        let mut OUTMAX: f64 = 0.0;
        let mut OUTMIN: f64 = 0.0;
        let mut SMALL: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut XOMAX: f64 = 0.0;
        let mut XOMIN: f64 = 0.0;

        Self {
            ATOL,
            INMAX,
            INMIN,
            OUTMAX,
            OUTMIN,
            SMALL,
            TOL,
            XOMAX,
            XOMIN,
        }
    }
}

//$Procedure F_ZZNRMLON ( ZZNRMLON tests )
pub fn F_ZZNRMLON(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //
    //
    // Local Parameters
    //

    // Local Variables
    //
    //

    //
    // Saved values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_ZZNRMLON", ctx)?;

    //***********************************************************************
    //
    //     Normal cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"INMIN = INMAX + <small>  < 0; ATOL = 0", ctx)?;

    save.SMALL = 0.00000000000001;

    save.ATOL = 0.0;
    save.INMIN = -(1.7 * spicelib::PI(ctx));
    save.INMAX = (save.INMIN - save.SMALL);

    save.XOMIN = save.INMIN;
    save.XOMAX = ((save.INMAX - save.SMALL) + spicelib::TWOPI(ctx));

    spicelib::ZZNRMLON(
        save.INMIN,
        save.INMAX,
        save.ATOL,
        &mut save.OUTMIN,
        &mut save.OUTMAX,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect tight agreement.
    //
    save.TOL = VTIGHT;

    testutil::CHCKSD(b"OUTMIN", save.OUTMIN, b"~", save.XOMIN, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"OUTMAX", save.OUTMAX, b"~", save.XOMAX, save.TOL, OK, ctx)?;
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"-2 pi < INMIN < INMAX < -1.5 pi; ATOL = 0", ctx)?;

    save.ATOL = 0.0;
    save.INMIN = -(1.7 * spicelib::PI(ctx));
    save.INMAX = -(1.6 * spicelib::PI(ctx));

    save.XOMIN = save.INMIN;
    save.XOMAX = save.INMAX;

    spicelib::ZZNRMLON(
        save.INMIN,
        save.INMAX,
        save.ATOL,
        &mut save.OUTMIN,
        &mut save.OUTMAX,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect equality.
    //
    save.TOL = 0.0;

    testutil::CHCKSD(b"OUTMIN", save.OUTMIN, b"=", save.XOMIN, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"OUTMAX", save.OUTMAX, b"=", save.XOMAX, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"-2 pi < INMIN < INMAX < -1.5 pi; ATOL > 0", ctx)?;

    save.ATOL = 0.0000000000001;
    save.INMIN = -(1.7 * spicelib::PI(ctx));
    save.INMAX = -(1.6 * spicelib::PI(ctx));

    save.XOMIN = save.INMIN;
    save.XOMAX = save.INMAX;

    spicelib::ZZNRMLON(
        save.INMIN,
        save.INMAX,
        save.ATOL,
        &mut save.OUTMIN,
        &mut save.OUTMAX,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect equality.
    //
    save.TOL = 0.0;

    testutil::CHCKSD(b"OUTMIN", save.OUTMIN, b"=", save.XOMIN, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"OUTMAX", save.OUTMAX, b"=", save.XOMAX, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"1.5 pi < INMIN, INMAX < 2 pi; ATOL = 0", ctx)?;

    save.ATOL = 0.0;
    save.INMIN = (1.6 * spicelib::PI(ctx));
    save.INMAX = (1.7 * spicelib::PI(ctx));

    save.XOMIN = save.INMIN;
    save.XOMAX = save.INMAX;

    spicelib::ZZNRMLON(
        save.INMIN,
        save.INMAX,
        save.ATOL,
        &mut save.OUTMIN,
        &mut save.OUTMAX,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect equality.
    //
    save.TOL = 0.0;

    testutil::CHCKSD(b"OUTMIN", save.OUTMIN, b"=", save.XOMIN, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"OUTMAX", save.OUTMAX, b"=", save.XOMAX, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"1.5 pi < INMIN, INMAX < 2 pi; ATOL > 0", ctx)?;

    save.ATOL = 0.0;
    save.INMIN = (1.6 * spicelib::PI(ctx));
    save.INMAX = (1.7 * spicelib::PI(ctx));

    save.XOMIN = save.INMIN;
    save.XOMAX = save.INMAX;

    spicelib::ZZNRMLON(
        save.INMIN,
        save.INMAX,
        save.ATOL,
        &mut save.OUTMIN,
        &mut save.OUTMAX,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect equality.
    //
    save.TOL = 0.0;

    testutil::CHCKSD(b"OUTMIN", save.OUTMIN, b"=", save.XOMIN, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"OUTMAX", save.OUTMAX, b"=", save.XOMAX, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"1.5 pi < INMIN = INMAX - <small>  < 2 pi; ATOL = 0", ctx)?;

    save.SMALL = 0.00000000000001;

    save.ATOL = 0.0;
    save.INMIN = (1.7 * spicelib::PI(ctx));
    save.INMAX = (save.INMIN - save.SMALL);

    save.XOMIN = (save.INMIN - spicelib::TWOPI(ctx));
    save.XOMAX = save.INMAX;

    spicelib::ZZNRMLON(
        save.INMIN,
        save.INMAX,
        save.ATOL,
        &mut save.OUTMIN,
        &mut save.OUTMAX,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect tight agreement.
    //
    save.TOL = VTIGHT;

    testutil::CHCKSD(b"OUTMIN", save.OUTMIN, b"~", save.XOMIN, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"OUTMAX", save.OUTMAX, b"~", save.XOMAX, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"-2 pi < INMIN < INMAX < INMIN + ATOL < -1.5 pi; ATOL > 0",
        ctx,
    )?;
    //
    // The upper bound exceeds the lower bound by less than
    // the input tolerance. The upper bound is negative.
    // The upper bound should be shifted.
    //
    save.ATOL = 0.0000000000001;
    save.INMIN = -(1.7 * spicelib::PI(ctx));
    save.INMAX = (save.INMIN + (save.ATOL / 10 as f64));

    save.XOMIN = save.INMIN;
    save.XOMAX = (save.INMAX + spicelib::TWOPI(ctx));

    spicelib::ZZNRMLON(
        save.INMIN,
        save.INMAX,
        save.ATOL,
        &mut save.OUTMIN,
        &mut save.OUTMAX,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect tight agreement.
    //
    save.TOL = VTIGHT;

    testutil::CHCKSD(b"OUTMIN", save.OUTMIN, b"~", save.XOMIN, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"OUTMAX", save.OUTMAX, b"~", save.XOMAX, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"0 < INMIN < INMAX < INMIN + ATOL < 1.5 pi; ATOL > 0", ctx)?;
    //
    // The upper bound exceeds the lower bound by less than
    // the input tolerance. The lower bound is positive.
    // The lower bound should be shifted.
    //
    save.ATOL = 0.0000000000001;
    save.INMIN = 0.0000000000001;
    save.INMAX = (save.INMIN + (save.ATOL / 10 as f64));

    save.XOMIN = (save.INMIN - spicelib::TWOPI(ctx));
    save.XOMAX = save.INMAX;

    spicelib::ZZNRMLON(
        save.INMIN,
        save.INMAX,
        save.ATOL,
        &mut save.OUTMIN,
        &mut save.OUTMAX,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect tight agreement.
    //
    save.TOL = VTIGHT;

    testutil::CHCKSD(b"OUTMIN", save.OUTMIN, b"~", save.XOMIN, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"OUTMAX", save.OUTMAX, b"~", save.XOMAX, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"-2 pi < INMIN < INMAX < INMIN + ATOL < -1.5 pi; ATOL > 0",
        ctx,
    )?;
    //
    // The upper bound exceeds the lower bound by a little more than
    // the input tolerance.
    //
    save.ATOL = 0.0000000000001;
    save.INMIN = -(1.7 * spicelib::PI(ctx));
    save.INMAX = (save.INMIN + (1.1 * save.ATOL));

    save.XOMIN = save.INMIN;
    save.XOMAX = save.INMAX;

    spicelib::ZZNRMLON(
        save.INMIN,
        save.INMAX,
        save.ATOL,
        &mut save.OUTMIN,
        &mut save.OUTMAX,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect exact agreement.
    //
    save.TOL = 0.0;

    testutil::CHCKSD(b"OUTMIN", save.OUTMIN, b"=", save.XOMIN, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"OUTMAX", save.OUTMAX, b"=", save.XOMAX, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"INMAX < INMIN; INMAX < 0; |INMAX-INMIN| < 2pi; ATOL = 0",
        ctx,
    )?;
    //
    // The bounds are out of order. The magnitude of the difference
    // is less than 2*pi. Tolerance is 0.
    //
    save.ATOL = 0.0;
    save.INMIN = 0.0;
    save.INMAX = -(1.7 * spicelib::PI(ctx));

    save.XOMIN = save.INMIN;
    save.XOMAX = (save.INMAX + spicelib::TWOPI(ctx));

    spicelib::ZZNRMLON(
        save.INMIN,
        save.INMAX,
        save.ATOL,
        &mut save.OUTMIN,
        &mut save.OUTMAX,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect tight agreement.
    //
    save.TOL = VTIGHT;

    testutil::CHCKSD(b"OUTMIN", save.OUTMIN, b"~", save.XOMIN, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"OUTMAX", save.OUTMAX, b"~", save.XOMAX, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"INMAX < INMIN; INMAX < 0; |INMAX-INMIN| < 2pi; ATOL > 0",
        ctx,
    )?;
    //
    // The bounds are out of order. The upper bound is negative. The
    // magnitude of the difference is less than 2*pi. Tolerance is > 0.
    //

    save.ATOL = 0.0000000000001;
    save.INMIN = 0.0;
    save.INMAX = -(1.7 * spicelib::PI(ctx));

    save.XOMIN = save.INMIN;
    save.XOMAX = (save.INMAX + spicelib::TWOPI(ctx));

    spicelib::ZZNRMLON(
        save.INMIN,
        save.INMAX,
        save.ATOL,
        &mut save.OUTMIN,
        &mut save.OUTMAX,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect tight agreement.
    //
    save.TOL = VTIGHT;

    testutil::CHCKSD(b"OUTMIN", save.OUTMIN, b"~", save.XOMIN, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"OUTMAX", save.OUTMAX, b"~", save.XOMAX, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"INMAX < INMIN; INMAX > 0; |INMAX-INMIN| < 2pi; ATOL > 0",
        ctx,
    )?;
    //
    // The bounds are out of order. The upper bound is positive. The
    // magnitude of the difference is less than 2*pi. Tolerance is > 0.
    //
    // The lower bound should be shifted left.
    //
    save.ATOL = 0.0000000000001;
    save.INMIN = (1.8 * spicelib::PI(ctx));
    save.INMAX = (0.1 * spicelib::PI(ctx));

    save.XOMIN = (save.INMIN - spicelib::TWOPI(ctx));
    save.XOMAX = save.INMAX;

    spicelib::ZZNRMLON(
        save.INMIN,
        save.INMAX,
        save.ATOL,
        &mut save.OUTMIN,
        &mut save.OUTMAX,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect tight agreement.
    //
    save.TOL = VTIGHT;

    testutil::CHCKSD(b"OUTMIN", save.OUTMIN, b"~", save.XOMIN, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"OUTMAX", save.OUTMAX, b"~", save.XOMAX, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"INMAX < INMIN; INMAX < 0; |INMAX-INMIN| > 2pi; ATOL = 0.",
        ctx,
    )?;

    //
    // This case requires both bounds to be shifted.
    //
    save.ATOL = 0.0;
    save.INMIN = (1.0 * spicelib::PI(ctx));
    save.INMAX = -(1.7 * spicelib::PI(ctx));

    save.XOMIN = (save.INMIN - spicelib::TWOPI(ctx));
    save.XOMAX = (save.INMAX + spicelib::TWOPI(ctx));

    spicelib::ZZNRMLON(
        save.INMIN,
        save.INMAX,
        save.ATOL,
        &mut save.OUTMIN,
        &mut save.OUTMAX,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect tight agreement.
    //
    save.TOL = VTIGHT;

    testutil::CHCKSD(b"OUTMIN", save.OUTMIN, b"~", save.XOMIN, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"OUTMAX", save.OUTMAX, b"~", save.XOMAX, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"INMIN + 2*pi < INMAX; ATOL = 0", ctx)?;

    //
    // This case requires the upper bound to be shifted.
    //
    save.ATOL = 0.0;
    save.INMIN = -(1.0 * spicelib::PI(ctx));
    save.INMAX = (1.7 * spicelib::PI(ctx));

    save.XOMIN = save.INMIN;
    save.XOMAX = (save.INMAX - spicelib::TWOPI(ctx));

    spicelib::ZZNRMLON(
        save.INMIN,
        save.INMAX,
        save.ATOL,
        &mut save.OUTMIN,
        &mut save.OUTMAX,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect tight agreement.
    //
    save.TOL = VTIGHT;

    testutil::CHCKSD(b"OUTMIN", save.OUTMIN, b"~", save.XOMIN, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"OUTMAX", save.OUTMAX, b"~", save.XOMAX, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"INMIN + 2*pi + ATOL < INMAX; ATOL > 0", ctx)?;

    //
    // This case requires the upper bound to be shifted.
    //
    save.ATOL = 0.0000000000001;
    save.INMIN = -(1.0 * spicelib::PI(ctx));
    save.INMAX = ((save.INMIN + spicelib::TWOPI(ctx)) + (1.1 * save.ATOL));

    save.XOMIN = save.INMIN;
    save.XOMAX = (save.INMAX - spicelib::TWOPI(ctx));

    spicelib::ZZNRMLON(
        save.INMIN,
        save.INMAX,
        save.ATOL,
        &mut save.OUTMIN,
        &mut save.OUTMAX,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect tight agreement.
    //
    save.TOL = VTIGHT;

    testutil::CHCKSD(b"OUTMIN", save.OUTMIN, b"~", save.XOMIN, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"OUTMAX", save.OUTMAX, b"~", save.XOMAX, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"INMIN + 2*pi < INMAX < INMIN + 2*pi + ATOL; ATOL > 0", ctx)?;

    //
    // This case requires the upper bound to be left as is.
    //
    save.ATOL = 0.0000000000001;
    save.INMIN = -(1.0 * spicelib::PI(ctx));
    save.INMAX = ((save.INMIN + spicelib::TWOPI(ctx)) + (0.5 * save.ATOL));

    save.XOMIN = save.INMIN;
    save.XOMAX = save.INMAX;

    spicelib::ZZNRMLON(
        save.INMIN,
        save.INMAX,
        save.ATOL,
        &mut save.OUTMIN,
        &mut save.OUTMAX,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect tight agreement.
    //
    save.TOL = VTIGHT;

    testutil::CHCKSD(b"OUTMIN", save.OUTMIN, b"~", save.XOMIN, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"OUTMAX", save.OUTMAX, b"~", save.XOMAX, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"-2pi - ATOL < INMIN; ATOL > 0.", ctx)?;

    //
    // This is a ridiculously large tolerance, but it is supposed
    // to be accepted.
    //
    save.ATOL = 1.0;

    save.INMIN = ((-spicelib::TWOPI(ctx) - save.ATOL) + 0.0000000000001);

    save.INMAX = (save.INMIN + spicelib::TWOPI(ctx));

    save.XOMIN = -spicelib::TWOPI(ctx);
    save.XOMAX = save.INMAX;

    spicelib::ZZNRMLON(
        save.INMIN,
        save.INMAX,
        save.ATOL,
        &mut save.OUTMIN,
        &mut save.OUTMAX,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // We expect tight agreement.
    //
    save.TOL = VTIGHT;

    testutil::CHCKSD(b"OUTMIN", save.OUTMIN, b"~", save.XOMIN, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"OUTMAX", save.OUTMAX, b"~", save.XOMAX, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"INMAX < 2pi + ATOL; ATOL > 0.", ctx)?;

    //
    // This is a ridiculously large tolerance, but it is supposed
    // to be accepted.
    //
    save.ATOL = 1.0;

    save.INMAX = ((spicelib::TWOPI(ctx) + save.ATOL) - 0.0000000000001);

    save.INMIN = (save.INMAX - spicelib::TWOPI(ctx));

    save.XOMIN = save.INMIN;
    save.XOMAX = spicelib::TWOPI(ctx);

    spicelib::ZZNRMLON(
        save.INMIN,
        save.INMAX,
        save.ATOL,
        &mut save.OUTMIN,
        &mut save.OUTMAX,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // We expect tight agreement.
    //
    save.TOL = VTIGHT;

    testutil::CHCKSD(b"OUTMIN", save.OUTMIN, b"~", save.XOMIN, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"OUTMAX", save.OUTMAX, b"~", save.XOMAX, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"-2pi - ATOL < INMIN; INMAX < INMIN; ATOL > 0.", ctx)?;

    //
    // This is a ridiculously large tolerance, but it is supposed
    // to be accepted.
    //
    save.ATOL = 1.0;

    save.INMIN = ((-spicelib::TWOPI(ctx) - save.ATOL) + 0.0000000000001);

    save.INMAX = (save.INMIN - 0.00000000000001);

    save.XOMIN = -spicelib::TWOPI(ctx);
    save.XOMAX = (save.XOMIN + spicelib::TWOPI(ctx));

    spicelib::ZZNRMLON(
        save.INMIN,
        save.INMAX,
        save.ATOL,
        &mut save.OUTMIN,
        &mut save.OUTMAX,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // We expect tight agreement.
    //
    save.TOL = VTIGHT;

    testutil::CHCKSD(b"OUTMIN", save.OUTMIN, b"~", save.XOMIN, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"OUTMAX", save.OUTMAX, b"~", save.XOMAX, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"INMAX < 2pi + ATOL; ATOL > 0.", ctx)?;

    //
    // This is a ridiculously large tolerance, but it is supposed
    // to be accepted.
    //
    save.ATOL = 1.0;

    save.INMAX = ((spicelib::TWOPI(ctx) + save.ATOL) - 0.0000000000001);

    save.INMIN = (save.INMAX + 0.00000000000001);

    save.XOMAX = spicelib::TWOPI(ctx);
    save.XOMIN = (save.XOMAX - spicelib::TWOPI(ctx));

    spicelib::ZZNRMLON(
        save.INMIN,
        save.INMAX,
        save.ATOL,
        &mut save.OUTMIN,
        &mut save.OUTMAX,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // We expect tight agreement.
    //
    save.TOL = VTIGHT;

    testutil::CHCKSD(b"OUTMIN", save.OUTMIN, b"~", save.XOMIN, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"OUTMAX", save.OUTMAX, b"~", save.XOMAX, save.TOL, OK, ctx)?;

    //***********************************************************************
    //
    //     Error cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Negative tolerance.", ctx)?;

    save.ATOL = -0.0000000000001;
    save.INMIN = -(1.0 * spicelib::PI(ctx));
    save.INMAX = (save.INMIN + spicelib::TWOPI(ctx));

    save.XOMIN = save.INMIN;
    save.XOMAX = save.INMAX;

    spicelib::ZZNRMLON(
        save.INMIN,
        save.INMAX,
        save.ATOL,
        &mut save.OUTMIN,
        &mut save.OUTMAX,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"INMIN < -2pi - ATOL", ctx)?;

    save.ATOL = 0.0000000000001;
    save.INMIN = (-spicelib::TWOPI(ctx) - (1.1 * save.ATOL));
    save.INMAX = (save.INMIN + spicelib::TWOPI(ctx));

    spicelib::ZZNRMLON(
        save.INMIN,
        save.INMAX,
        save.ATOL,
        &mut save.OUTMIN,
        &mut save.OUTMAX,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"INMIN >  2pi + ATOL", ctx)?;

    save.ATOL = 0.0000000000001;
    save.INMIN = (spicelib::TWOPI(ctx) + (1.1 * save.ATOL));
    save.INMAX = spicelib::TWOPI(ctx);

    spicelib::ZZNRMLON(
        save.INMIN,
        save.INMAX,
        save.ATOL,
        &mut save.OUTMIN,
        &mut save.OUTMAX,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"INMAX < -2pi - ATOL", ctx)?;

    save.ATOL = 0.0000000000001;
    save.INMAX = (-spicelib::TWOPI(ctx) - (1.1 * save.ATOL));
    save.INMAX = (save.INMIN + spicelib::TWOPI(ctx));

    spicelib::ZZNRMLON(
        save.INMIN,
        save.INMAX,
        save.ATOL,
        &mut save.OUTMIN,
        &mut save.OUTMAX,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"INMAX >  2pi + ATOL", ctx)?;

    save.ATOL = 0.0000000000001;
    save.INMAX = (spicelib::TWOPI(ctx) + (1.1 * save.ATOL));
    save.INMIN = spicelib::TWOPI(ctx);

    spicelib::ZZNRMLON(
        save.INMIN,
        save.INMAX,
        save.ATOL,
        &mut save.OUTMIN,
        &mut save.OUTMAX,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"INMIN = INMAX < 0; ATOL = 0", ctx)?;

    save.ATOL = 0.0;
    save.INMIN = -(1.7 * spicelib::PI(ctx));
    save.INMAX = save.INMIN;

    save.XOMIN = save.INMIN;
    save.XOMAX = (save.INMAX + spicelib::TWOPI(ctx));

    spicelib::ZZNRMLON(
        save.INMIN,
        save.INMAX,
        save.ATOL,
        &mut save.OUTMIN,
        &mut save.OUTMAX,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ZEROBOUNDSEXTENT)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"-2 pi < INMIN = INMAX < -1.5 pi; ATOL = 0", ctx)?;

    save.ATOL = 0.0;
    save.INMIN = -(1.7 * spicelib::PI(ctx));
    save.INMAX = save.INMIN;

    save.XOMIN = save.INMIN;
    save.XOMAX = (save.INMAX + spicelib::TWOPI(ctx));

    spicelib::ZZNRMLON(
        save.INMIN,
        save.INMAX,
        save.ATOL,
        &mut save.OUTMIN,
        &mut save.OUTMAX,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ZEROBOUNDSEXTENT)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
