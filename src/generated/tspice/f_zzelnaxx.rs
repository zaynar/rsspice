//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const VTIGHT: f64 = 0.00000000000001;

struct SaveVars {
    A: f64,
    B: f64,
    EPS: f64,
    LAT: f64,
    M: f64,
    T: f64,
    TOL: f64,
    X: f64,
    XX: f64,
    XXPT: f64,
    XY: f64,
    Y: f64,
    YXPT: f64,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut A: f64 = 0.0;
        let mut B: f64 = 0.0;
        let mut EPS: f64 = 0.0;
        let mut LAT: f64 = 0.0;
        let mut M: f64 = 0.0;
        let mut T: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut X: f64 = 0.0;
        let mut XX: f64 = 0.0;
        let mut XXPT: f64 = 0.0;
        let mut XY: f64 = 0.0;
        let mut Y: f64 = 0.0;
        let mut YXPT: f64 = 0.0;

        Self {
            A,
            B,
            EPS,
            LAT,
            M,
            T,
            TOL,
            X,
            XX,
            XXPT,
            XY,
            Y,
            YXPT,
        }
    }
}

//$Procedure F_ZZELNAXX ( ZZELNAXX tests )
pub fn F_ZZELNAXX(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    testutil::TOPEN(b"F_ZZELNAXX", ctx)?;

    //***********************************************************************
    //
    //     Normal cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Oblate case, LAT = pi/6", ctx)?;

    save.A = 2.0;
    save.B = 1.0;
    save.LAT = (spicelib::PI(ctx) / 6 as f64);

    spicelib::ZZELNAXX(
        save.A,
        save.B,
        save.LAT,
        &mut save.XXPT,
        &mut save.YXPT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // M is the slope of the normal vector at (x,y).
    // This vector is the gradient of
    //
    //               2   2      2
    //    f(x,y) =  x / a   +  y / b
    //
    //
    save.M = f64::tan(save.LAT);

    //
    // Let T = (y/x) (the tangent of the latitudinal latitude)
    //
    save.T = ((f64::powi(save.B, 2) / f64::powi(save.A, 2)) * save.M);

    //
    // Since
    //
    //    y = T*x
    //
    // we have
    //
    //     2   2       2 2   2
    //    x / a    +  T x / b    =   1
    //
    // So
    //
    //     2        2     2   2  -1       2    2 2
    //    x  = ( 1/a  +  T / b  )  ,     y  = T x
    //
    // Both x, y are >= 0.
    //
    //
    // XX is the expected X-axis intercept, which is
    //
    //             2 2
    //     ( 1 - (b/a ) ) * x
    //
    //
    save.X = f64::sqrt(
        (1.0 / (((1 as f64) / f64::powi(save.A, 2))
            + (f64::powi(save.T, 2) / f64::powi(save.B, 2)))),
    );

    save.Y = f64::sqrt((f64::powi(save.T, 2) * f64::powi(save.X, 2)));

    save.XX = ((1.0 - (f64::powi(save.B, 2) / f64::powi(save.A, 2))) * save.X);

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"X intercept", save.XXPT, b"~", save.XX, save.TOL, OK, ctx)?;

    //
    // XY is the expected Y-axis intercept, which is
    //
    //            2   2
    //    ( 1 - (a / b ) ) y
    //

    save.XY = ((1.0 - (f64::powi(save.A, 2) / f64::powi(save.B, 2))) * save.Y);

    testutil::CHCKSD(b"Y intercept", save.YXPT, b"~", save.XY, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Oblate case, LAT = pi/3", ctx)?;

    save.A = 2.0;
    save.B = 1.0;
    save.LAT = (spicelib::PI(ctx) / 3 as f64);

    spicelib::ZZELNAXX(
        save.A,
        save.B,
        save.LAT,
        &mut save.XXPT,
        &mut save.YXPT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // See the pi/6 case for a discussion of what follows.
    //
    save.M = f64::tan(save.LAT);
    //
    save.T = ((f64::powi(save.B, 2) / f64::powi(save.A, 2)) * save.M);

    save.X = f64::sqrt(
        (1.0 / (((1 as f64) / f64::powi(save.A, 2))
            + (f64::powi(save.T, 2) / f64::powi(save.B, 2)))),
    );

    save.Y = f64::sqrt((f64::powi(save.T, 2) * f64::powi(save.X, 2)));

    save.XX = ((1.0 - (f64::powi(save.B, 2) / f64::powi(save.A, 2))) * save.X);

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"X intercept", save.XXPT, b"~", save.XX, save.TOL, OK, ctx)?;

    save.XY = ((1.0 - (f64::powi(save.A, 2) / f64::powi(save.B, 2))) * save.Y);

    testutil::CHCKSD(b"Y intercept", save.YXPT, b"~", save.XY, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Oblate case, LAT << 1", ctx)?;

    save.A = 2.0;
    save.B = 1.0;
    save.LAT = 0.0000000000001;

    spicelib::ZZELNAXX(
        save.A,
        save.B,
        save.LAT,
        &mut save.XXPT,
        &mut save.YXPT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // See the pi/3 case for a discussion of what follows.
    //
    save.M = f64::tan(save.LAT);
    //
    save.T = ((f64::powi(save.B, 2) / f64::powi(save.A, 2)) * save.M);

    save.X = f64::sqrt(
        (1.0 / (((1 as f64) / f64::powi(save.A, 2))
            + (f64::powi(save.T, 2) / f64::powi(save.B, 2)))),
    );

    save.Y = f64::sqrt((f64::powi(save.T, 2) * f64::powi(save.X, 2)));

    save.XX = ((1.0 - (f64::powi(save.B, 2) / f64::powi(save.A, 2))) * save.X);

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"X intercept", save.XXPT, b"~", save.XX, save.TOL, OK, ctx)?;

    save.XY = ((1.0 - (f64::powi(save.A, 2) / f64::powi(save.B, 2))) * save.Y);

    testutil::CHCKSD(b"Y intercept", save.YXPT, b"~", save.XY, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Oblate case, LAT = 0", ctx)?;

    save.A = 2.0;
    save.B = 1.0;
    save.LAT = 0.0;

    spicelib::ZZELNAXX(
        save.A,
        save.B,
        save.LAT,
        &mut save.XXPT,
        &mut save.YXPT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // See the pi/3 case for a discussion of what follows.
    //
    save.M = f64::tan(save.LAT);
    //
    save.T = ((f64::powi(save.B, 2) / f64::powi(save.A, 2)) * save.M);

    save.X = f64::sqrt(
        (1.0 / (((1 as f64) / f64::powi(save.A, 2))
            + (f64::powi(save.T, 2) / f64::powi(save.B, 2)))),
    );

    save.Y = f64::sqrt((f64::powi(save.T, 2) * f64::powi(save.X, 2)));

    save.XX = ((1.0 - (f64::powi(save.B, 2) / f64::powi(save.A, 2))) * save.X);

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"X intercept", save.XXPT, b"~", save.XX, save.TOL, OK, ctx)?;

    save.XY = ((1.0 - (f64::powi(save.A, 2) / f64::powi(save.B, 2))) * save.Y);

    testutil::CHCKSD(b"Y intercept", save.YXPT, b"~", save.XY, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Oblate case, LAT = pi/2", ctx)?;

    save.A = 2.0;
    save.B = 1.0;
    save.LAT = (spicelib::PI(ctx) / 2 as f64);

    spicelib::ZZELNAXX(
        save.A,
        save.B,
        save.LAT,
        &mut save.XXPT,
        &mut save.YXPT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // See the pi/3 case for a discussion of what follows.
    //
    save.X = 0.0;

    save.Y = save.B;

    save.XX = ((1.0 - (f64::powi(save.B, 2) / f64::powi(save.A, 2))) * save.X);

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"X intercept", save.XXPT, b"~", save.XX, save.TOL, OK, ctx)?;

    save.XY = ((1.0 - (f64::powi(save.A, 2) / f64::powi(save.B, 2))) * save.Y);

    testutil::CHCKSD(b"Y intercept", save.YXPT, b"~", save.XY, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Oblate case, LAT = (pi/2) - EPS, EPS << 1", ctx)?;

    save.A = 2.0;
    save.B = 1.0;
    save.EPS = 0.0000000000001;
    save.LAT = ((spicelib::PI(ctx) / 2 as f64) - save.EPS);

    spicelib::ZZELNAXX(
        save.A,
        save.B,
        save.LAT,
        &mut save.XXPT,
        &mut save.YXPT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // In principle, tan(LAT) can be very large. We want to
    // work with cot(LAT) and x/y instead. Let T = x/y.
    //
    //                 2   2
    //    tan(LAT) = (a / b ) * y/x
    //
    // so
    //
    //          2   2
    //    T = (a / b ) * cot(LAT)
    //
    //
    save.T =
        ((f64::powi(save.A, 2) / f64::powi(save.B, 2)) * (f64::cos(save.LAT) / f64::sin(save.LAT)));

    //
    // x = T*y, so
    //
    //          2        2
    //    (Ty/a)  + (y/b)  = 1
    //
    // Then
    //
    //      2      2  2       2  -1
    //     y  = ( T /a  +  1/b  )
    //
    //
    //
    save.Y = f64::sqrt(
        (1.0 / ((f64::powi(save.T, 2) / f64::powi(save.A, 2)) + (1.0 / f64::powi(save.B, 2)))),
    );

    save.X = (save.T * save.Y);

    save.XX = ((1.0 - (f64::powi(save.B, 2) / f64::powi(save.A, 2))) * save.X);

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"X intercept", save.XXPT, b"~", save.XX, save.TOL, OK, ctx)?;

    save.XY = ((1.0 - (f64::powi(save.A, 2) / f64::powi(save.B, 2))) * save.Y);

    testutil::CHCKSD(b"Y intercept", save.YXPT, b"~", save.XY, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Prolate case, LAT = pi/6", ctx)?;

    save.A = 1.0;
    save.B = 2.0;
    save.LAT = (spicelib::PI(ctx) / 6 as f64);

    spicelib::ZZELNAXX(
        save.A,
        save.B,
        save.LAT,
        &mut save.XXPT,
        &mut save.YXPT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The prolate geometry is just a reflection of an oblate case
    // in which the latitude is complemented and the axis lengths
    // are swapped. The X intercept in the reflected case is the
    // Y intercept of the original case; the relation is the same
    // for the other intercepts.
    //
    spicelib::ZZELNAXX(
        save.B,
        save.A,
        ((spicelib::PI(ctx) / 2 as f64) - save.LAT),
        &mut save.XY,
        &mut save.XX,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"X intercept", save.XXPT, b"~", save.XX, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"Y intercept", save.YXPT, b"~", save.XY, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Prolate case, LAT = pi/3", ctx)?;

    save.A = 1.0;
    save.B = 2.0;
    save.LAT = (spicelib::PI(ctx) / 3 as f64);

    spicelib::ZZELNAXX(
        save.A,
        save.B,
        save.LAT,
        &mut save.XXPT,
        &mut save.YXPT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZELNAXX(
        save.B,
        save.A,
        ((spicelib::PI(ctx) / 2 as f64) - save.LAT),
        &mut save.XY,
        &mut save.XX,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"X intercept", save.XXPT, b"~", save.XX, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"Y intercept", save.YXPT, b"~", save.XY, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Prolate case, LAT << 1", ctx)?;

    save.A = 1.0;
    save.B = 2.0;
    save.EPS = 0.0000000000001;
    save.LAT = save.EPS;

    spicelib::ZZELNAXX(
        save.A,
        save.B,
        save.LAT,
        &mut save.XXPT,
        &mut save.YXPT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZELNAXX(
        save.B,
        save.A,
        ((spicelib::PI(ctx) / 2 as f64) - save.LAT),
        &mut save.XY,
        &mut save.XX,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"X intercept", save.XXPT, b"~", save.XX, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"Y intercept", save.YXPT, b"~", save.XY, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Prolate case, LAT = 0", ctx)?;

    save.A = 1.0;
    save.B = 2.0;
    save.LAT = 0.0;

    spicelib::ZZELNAXX(
        save.A,
        save.B,
        save.LAT,
        &mut save.XXPT,
        &mut save.YXPT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZELNAXX(
        save.B,
        save.A,
        ((spicelib::PI(ctx) / 2 as f64) - save.LAT),
        &mut save.XY,
        &mut save.XX,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"X intercept", save.XXPT, b"~", save.XX, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"Y intercept", save.YXPT, b"~", save.XY, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Prolate case, LAT = pi/2", ctx)?;

    save.A = 1.0;
    save.B = 2.0;
    save.LAT = (spicelib::PI(ctx) / 2 as f64);

    spicelib::ZZELNAXX(
        save.A,
        save.B,
        save.LAT,
        &mut save.XXPT,
        &mut save.YXPT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZELNAXX(
        save.B,
        save.A,
        ((spicelib::PI(ctx) / 2 as f64) - save.LAT),
        &mut save.XY,
        &mut save.XX,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"X intercept", save.XXPT, b"~", save.XX, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"Y intercept", save.YXPT, b"~", save.XY, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Prolate case, LAT = (pi/2) - EPS, EPS << 1", ctx)?;

    save.A = 1.0;
    save.B = 2.0;
    save.EPS = 0.0000000000001;
    save.LAT = ((spicelib::PI(ctx) / 2 as f64) - save.EPS);

    spicelib::ZZELNAXX(
        save.A,
        save.B,
        save.LAT,
        &mut save.XXPT,
        &mut save.YXPT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZELNAXX(
        save.B,
        save.A,
        ((spicelib::PI(ctx) / 2 as f64) - save.LAT),
        &mut save.XY,
        &mut save.XX,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"X intercept", save.XXPT, b"~", save.XX, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"Y intercept", save.YXPT, b"~", save.XY, save.TOL, OK, ctx)?;

    //***********************************************************************
    //
    //     Error cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid axis length.", ctx)?;

    save.A = 0.0;
    save.B = 1.0;
    save.LAT = (spicelib::PI(ctx) / 2 as f64);

    spicelib::ZZELNAXX(
        save.A,
        save.B,
        save.LAT,
        &mut save.XXPT,
        &mut save.YXPT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NONPOSITIVEAXIS)", OK, ctx)?;

    save.A = -1.0;

    spicelib::ZZELNAXX(
        save.A,
        save.B,
        save.LAT,
        &mut save.XXPT,
        &mut save.YXPT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NONPOSITIVEAXIS)", OK, ctx)?;

    save.A = 1.0;
    save.B = 0.0;

    spicelib::ZZELNAXX(
        save.A,
        save.B,
        save.LAT,
        &mut save.XXPT,
        &mut save.YXPT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NONPOSITIVEAXIS)", OK, ctx)?;

    save.B = -1.0;

    spicelib::ZZELNAXX(
        save.A,
        save.B,
        save.LAT,
        &mut save.XXPT,
        &mut save.YXPT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NONPOSITIVEAXIS)", OK, ctx)?;

    //
    // ---------------------------------------------------------
    //

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
