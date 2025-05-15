//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const TIGHT: f64 = 0.000000000001;
const VTIGHT: f64 = 0.00000000000001;
const DBSIZE: i32 = 100;
const LNSIZE: i32 = 320;

struct SaveVars {
    TITLE: Vec<u8>,
    BOUNDS: StackArray2D<f64, 200>,
    MAXLON: f64,
    MINLON: f64,
    OUTBDS: StackArray2D<f64, 200>,
    TOL: f64,
    XBDS: StackArray2D<f64, 200>,
    MAXN: i32,
    NIVALS: i32,
    NOUT: i32,
    SIZE: i32,
    SRCS: StackArray<i32, 100>,
    XNOUT: i32,
    XSRCS: StackArray<i32, 100>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut BOUNDS = StackArray2D::<f64, 200>::new(1..=2, 1..=DBSIZE);
        let mut MAXLON: f64 = 0.0;
        let mut MINLON: f64 = 0.0;
        let mut OUTBDS = StackArray2D::<f64, 200>::new(1..=2, 1..=DBSIZE);
        let mut TOL: f64 = 0.0;
        let mut XBDS = StackArray2D::<f64, 200>::new(1..=2, 1..=DBSIZE);
        let mut MAXN: i32 = 0;
        let mut NIVALS: i32 = 0;
        let mut NOUT: i32 = 0;
        let mut SIZE: i32 = 0;
        let mut SRCS = StackArray::<i32, 100>::new(1..=DBSIZE);
        let mut XNOUT: i32 = 0;
        let mut XSRCS = StackArray::<i32, 100>::new(1..=DBSIZE);

        Self {
            TITLE,
            BOUNDS,
            MAXLON,
            MINLON,
            OUTBDS,
            TOL,
            XBDS,
            MAXN,
            NIVALS,
            NOUT,
            SIZE,
            SRCS,
            XNOUT,
            XSRCS,
        }
    }
}

//$Procedure F_REGLON ( REGLON tests )
pub fn F_REGLON(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    testutil::TOPEN(b"F_REGLON", ctx)?;

    //**********************************************************************
    //
    //     REGLON normal cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"Trivial case: one interval in [0,2pi], no change needed.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    save.NIVALS = 1;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::PI(ctx);

    save.MAXN = 1;

    support::REGLON(
        save.NIVALS,
        save.BOUNDS.as_slice(),
        save.MAXN,
        &mut save.NOUT,
        &mut save.MINLON,
        &mut save.MAXLON,
        save.OUTBDS.as_slice_mut(),
        save.SRCS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the output range.
    //
    testutil::CHCKSD(b"MINLON", save.MINLON, b"~", 0.0, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(
        b"MAXLON",
        save.MAXLON,
        b"~",
        spicelib::TWOPI(ctx),
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check number of output intervals.
    //
    save.XNOUT = 1;

    testutil::CHCKSI(b"NOUT", save.NOUT, b"=", save.XNOUT, 0, OK, ctx)?;

    //
    // Check output intervals.
    //
    save.SIZE = (2 * save.NOUT);
    spicelib::MOVED(save.BOUNDS.as_slice(), save.SIZE, save.XBDS.as_slice_mut());

    testutil::CHCKAD(
        b"OUTBDS",
        save.OUTBDS.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.SIZE,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Check source intervals.
    //
    save.XSRCS[1] = 1;

    testutil::CHCKAI(
        b"SRCS",
        save.SRCS.as_slice(),
        b"=",
        save.XSRCS.as_slice(),
        1,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"Trivial case: one interval in [-pi,pi], no change needed.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    save.NIVALS = 1;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 2 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 2 as f64);

    save.MAXN = 1;

    support::REGLON(
        save.NIVALS,
        save.BOUNDS.as_slice(),
        save.MAXN,
        &mut save.NOUT,
        &mut save.MINLON,
        &mut save.MAXLON,
        save.OUTBDS.as_slice_mut(),
        save.SRCS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the output range.
    //
    testutil::CHCKSD(
        b"MINLON",
        save.MINLON,
        b"~",
        -spicelib::PI(ctx),
        VTIGHT,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"MAXLON",
        save.MAXLON,
        b"~",
        spicelib::PI(ctx),
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check number of output intervals.
    //
    save.XNOUT = 1;

    testutil::CHCKSI(b"NOUT", save.NOUT, b"=", save.XNOUT, 0, OK, ctx)?;

    //
    // Check output intervals.
    //
    save.SIZE = (2 * save.NOUT);
    spicelib::MOVED(save.BOUNDS.as_slice(), save.SIZE, save.XBDS.as_slice_mut());

    testutil::CHCKAD(
        b"OUTBDS",
        save.OUTBDS.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.SIZE,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Check source intervals.
    //
    save.XSRCS[1] = 1;

    testutil::CHCKAI(
        b"SRCS",
        save.SRCS.as_slice(),
        b"=",
        save.XSRCS.as_slice(),
        1,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"One interval, endpoints are in reverse order, range is  [-pi, pi], bounds have mixed signs.");

    testutil::TCASE(&save.TITLE, ctx)?;

    save.NIVALS = 1;

    save.BOUNDS[[1, 1]] = (spicelib::PI(ctx) / 2 as f64);
    save.BOUNDS[[2, 1]] = -(spicelib::PI(ctx) / 2 as f64);

    save.MAXN = 2;

    support::REGLON(
        save.NIVALS,
        save.BOUNDS.as_slice(),
        save.MAXN,
        &mut save.NOUT,
        &mut save.MINLON,
        &mut save.MAXLON,
        save.OUTBDS.as_slice_mut(),
        save.SRCS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the output range.
    //
    testutil::CHCKSD(b"MINLON", save.MINLON, b"~", 0.0, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(
        b"MAXLON",
        save.MAXLON,
        b"~",
        spicelib::TWOPI(ctx),
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check number of output intervals.
    //
    save.XNOUT = 1;

    testutil::CHCKSI(b"NOUT", save.NOUT, b"=", save.XNOUT, 0, OK, ctx)?;

    //
    // Check output intervals.
    //
    save.TOL = VTIGHT;

    save.XBDS[[1, 1]] = (spicelib::PI(ctx) / 2 as f64);
    save.XBDS[[2, 1]] = (1.5 * spicelib::PI(ctx));

    save.SIZE = (2 * save.NOUT);

    testutil::CHCKAD(
        b"OUTBDS",
        save.OUTBDS.as_slice(),
        b"~~",
        save.XBDS.as_slice(),
        save.SIZE,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check source intervals.
    //
    save.XSRCS[1] = 1;

    testutil::CHCKAI(
        b"SRCS",
        save.SRCS.as_slice(),
        b"=",
        save.XSRCS.as_slice(),
        1,
        OK,
        ctx,
    )?;

    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"One interval, endpoints are in reverse order, range is  [-pi, pi], bounds are both negative.");

    testutil::TCASE(&save.TITLE, ctx)?;

    save.NIVALS = 1;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 1]] = -(spicelib::PI(ctx) / 2 as f64);

    save.MAXN = 2;

    support::REGLON(
        save.NIVALS,
        save.BOUNDS.as_slice(),
        save.MAXN,
        &mut save.NOUT,
        &mut save.MINLON,
        &mut save.MAXLON,
        save.OUTBDS.as_slice_mut(),
        save.SRCS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check number of output intervals.
    //
    save.XNOUT = 2;

    testutil::CHCKSI(b"NOUT", save.NOUT, b"=", save.XNOUT, 0, OK, ctx)?;

    //
    // Check output intervals.
    //
    save.TOL = VTIGHT;

    save.XBDS[[1, 1]] = -spicelib::PI(ctx);
    save.XBDS[[2, 1]] = -(spicelib::PI(ctx) / 2 as f64);

    save.XBDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.XBDS[[2, 2]] = spicelib::PI(ctx);

    save.SIZE = (2 * save.NOUT);

    testutil::CHCKAD(
        b"OUTBDS",
        save.OUTBDS.as_slice(),
        b"~~",
        save.XBDS.as_slice(),
        save.SIZE,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check source intervals.
    //
    save.XSRCS[1] = 1;
    save.XSRCS[2] = 1;

    testutil::CHCKAI(
        b"SRCS",
        save.SRCS.as_slice(),
        b"=",
        save.XSRCS.as_slice(),
        2,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"One interval, endpoints are in reverse order, range is  [-pi, pi], only interval on right of lower bound is produced.");

    testutil::TCASE(&save.TITLE, ctx)?;

    save.NIVALS = 1;

    save.BOUNDS[[1, 1]] = (spicelib::PI(ctx) / 2 as f64);
    save.BOUNDS[[2, 1]] = -spicelib::PI(ctx);

    save.MAXN = 2;

    support::REGLON(
        save.NIVALS,
        save.BOUNDS.as_slice(),
        save.MAXN,
        &mut save.NOUT,
        &mut save.MINLON,
        &mut save.MAXLON,
        save.OUTBDS.as_slice_mut(),
        save.SRCS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the output range. We expect it to have been
    // adjusted.
    //
    testutil::CHCKSD(b"MINLON", save.MINLON, b"~", 0.0, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(
        b"MAXLON",
        save.MAXLON,
        b"~",
        spicelib::TWOPI(ctx),
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check number of output intervals.
    //
    save.XNOUT = 1;

    testutil::CHCKSI(b"NOUT", save.NOUT, b"=", save.XNOUT, 0, OK, ctx)?;

    //
    // Check output intervals.
    //
    save.TOL = VTIGHT;

    save.XBDS[[1, 1]] = (spicelib::PI(ctx) / 2 as f64);
    save.XBDS[[2, 1]] = spicelib::PI(ctx);

    save.SIZE = (2 * save.NOUT);

    testutil::CHCKAD(
        b"OUTBDS",
        save.OUTBDS.as_slice(),
        b"~~",
        save.XBDS.as_slice(),
        save.SIZE,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check source intervals.
    //
    save.XSRCS[1] = 1;

    testutil::CHCKAI(
        b"SRCS",
        save.SRCS.as_slice(),
        b"=",
        save.XSRCS.as_slice(),
        1,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Two intervals, one interval has endpoints in reverse order, range is [-pi, pi]. For reversed interval, only interval on right of lower bound is produced.");

    testutil::TCASE(&save.TITLE, ctx)?;

    save.NIVALS = 2;

    save.BOUNDS[[1, 1]] = (spicelib::PI(ctx) / 2 as f64);
    save.BOUNDS[[2, 1]] = -spicelib::PI(ctx);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 2 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 2 as f64);

    save.MAXN = 2;

    support::REGLON(
        save.NIVALS,
        save.BOUNDS.as_slice(),
        save.MAXN,
        &mut save.NOUT,
        &mut save.MINLON,
        &mut save.MAXLON,
        save.OUTBDS.as_slice_mut(),
        save.SRCS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(
        b"MINLON",
        save.MINLON,
        b"~",
        -spicelib::PI(ctx),
        VTIGHT,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"MAXLON",
        save.MAXLON,
        b"~",
        spicelib::PI(ctx),
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check number of output intervals.
    //
    save.XNOUT = 2;

    testutil::CHCKSI(b"NOUT", save.NOUT, b"=", save.XNOUT, 0, OK, ctx)?;

    //
    // Check output intervals.
    //
    save.TOL = VTIGHT;

    save.XBDS[[1, 1]] = (spicelib::PI(ctx) / 2 as f64);
    save.XBDS[[2, 1]] = spicelib::PI(ctx);

    save.XBDS[[1, 2]] = -(spicelib::PI(ctx) / 2 as f64);
    save.XBDS[[2, 2]] = (spicelib::PI(ctx) / 2 as f64);

    save.SIZE = (2 * save.NOUT);

    testutil::CHCKAD(
        b"OUTBDS",
        save.OUTBDS.as_slice(),
        b"~~",
        save.XBDS.as_slice(),
        save.SIZE,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check source intervals.
    //
    save.XSRCS[1] = 1;
    save.XSRCS[2] = 2;

    testutil::CHCKAI(
        b"SRCS",
        save.SRCS.as_slice(),
        b"=",
        save.XSRCS.as_slice(),
        save.NOUT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"One interval, endpoints are in reverse order, range is  [-pi, pi], only interval on left of upper bound is produced.");

    testutil::TCASE(&save.TITLE, ctx)?;

    save.NIVALS = 1;

    save.BOUNDS[[1, 1]] = spicelib::PI(ctx);
    save.BOUNDS[[2, 1]] = -(spicelib::PI(ctx) / 2 as f64);

    save.MAXN = 2;

    support::REGLON(
        save.NIVALS,
        save.BOUNDS.as_slice(),
        save.MAXN,
        &mut save.NOUT,
        &mut save.MINLON,
        &mut save.MAXLON,
        save.OUTBDS.as_slice_mut(),
        save.SRCS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the output range.
    //
    testutil::CHCKSD(b"MINLON", save.MINLON, b"~", 0.0, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(
        b"MAXLON",
        save.MAXLON,
        b"~",
        spicelib::TWOPI(ctx),
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check number of output intervals.
    //
    save.XNOUT = 1;

    testutil::CHCKSI(b"NOUT", save.NOUT, b"=", save.XNOUT, 0, OK, ctx)?;

    //
    // Check output intervals.
    //
    save.TOL = VTIGHT;

    save.XBDS[[1, 1]] = spicelib::PI(ctx);
    save.XBDS[[2, 1]] = (1.5 * spicelib::PI(ctx));

    save.SIZE = (2 * save.NOUT);

    testutil::CHCKAD(
        b"OUTBDS",
        save.OUTBDS.as_slice(),
        b"~~",
        save.XBDS.as_slice(),
        save.SIZE,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check source intervals.
    //
    save.XSRCS[1] = 1;

    testutil::CHCKAI(
        b"SRCS",
        save.SRCS.as_slice(),
        b"=",
        save.XSRCS.as_slice(),
        1,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Two intervals, one interval has endpoints in reverse order, range is [-pi, pi]. For reversed interval, only interval on left of upper bound is produced.");

    testutil::TCASE(&save.TITLE, ctx)?;

    save.NIVALS = 2;

    save.BOUNDS[[1, 1]] = spicelib::PI(ctx);
    save.BOUNDS[[2, 1]] = -(spicelib::PI(ctx) / 2 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 2 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 2 as f64);

    save.MAXN = 2;

    support::REGLON(
        save.NIVALS,
        save.BOUNDS.as_slice(),
        save.MAXN,
        &mut save.NOUT,
        &mut save.MINLON,
        &mut save.MAXLON,
        save.OUTBDS.as_slice_mut(),
        save.SRCS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //
    //
    testutil::CHCKSD(
        b"MINLON",
        save.MINLON,
        b"~",
        -spicelib::PI(ctx),
        VTIGHT,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"MAXLON",
        save.MAXLON,
        b"~",
        spicelib::PI(ctx),
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check number of output intervals.
    //
    save.XNOUT = 2;

    testutil::CHCKSI(b"NOUT", save.NOUT, b"=", save.XNOUT, 0, OK, ctx)?;

    //
    // Check output intervals.
    //
    save.TOL = VTIGHT;

    save.XBDS[[1, 1]] = -spicelib::PI(ctx);
    save.XBDS[[2, 1]] = -(spicelib::PI(ctx) / 2 as f64);

    save.XBDS[[1, 2]] = -(spicelib::PI(ctx) / 2 as f64);
    save.XBDS[[2, 2]] = (spicelib::PI(ctx) / 2 as f64);

    save.SIZE = (2 * save.NOUT);

    testutil::CHCKAD(
        b"OUTBDS",
        save.OUTBDS.as_slice(),
        b"~~",
        save.XBDS.as_slice(),
        save.SIZE,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check source intervals.
    //
    save.XSRCS[1] = 1;
    save.XSRCS[2] = 2;

    testutil::CHCKAI(
        b"SRCS",
        save.SRCS.as_slice(),
        b"=",
        save.XSRCS.as_slice(),
        save.NOUT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"One interval, endpoints are in reverse order, range is  [0, 2*pi].",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    save.NIVALS = 1;

    save.BOUNDS[[1, 1]] = (spicelib::PI(ctx) / 2 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 4 as f64);

    save.MAXN = 2;

    support::REGLON(
        save.NIVALS,
        save.BOUNDS.as_slice(),
        save.MAXN,
        &mut save.NOUT,
        &mut save.MINLON,
        &mut save.MAXLON,
        save.OUTBDS.as_slice_mut(),
        save.SRCS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the output range.
    //
    testutil::CHCKSD(
        b"MINLON",
        save.MINLON,
        b"~",
        -spicelib::PI(ctx),
        VTIGHT,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"MAXLON",
        save.MAXLON,
        b"~",
        spicelib::PI(ctx),
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check number of output intervals.
    //
    save.XNOUT = 2;

    testutil::CHCKSI(b"NOUT", save.NOUT, b"=", save.XNOUT, 0, OK, ctx)?;

    //
    // Check output intervals.
    //
    save.TOL = VTIGHT;

    save.XBDS[[1, 1]] = -spicelib::PI(ctx);
    save.XBDS[[2, 1]] = (spicelib::PI(ctx) / 4 as f64);

    save.XBDS[[1, 2]] = (spicelib::PI(ctx) / 2 as f64);
    save.XBDS[[2, 2]] = spicelib::PI(ctx);

    save.SIZE = (2 * save.NOUT);

    testutil::CHCKAD(
        b"OUTBDS",
        save.OUTBDS.as_slice(),
        b"~~",
        save.XBDS.as_slice(),
        save.SIZE,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check source intervals.
    //
    save.XSRCS[1] = 1;
    save.XSRCS[2] = 1;

    testutil::CHCKAI(
        b"SRCS",
        save.SRCS.as_slice(),
        b"=",
        save.XSRCS.as_slice(),
        2,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"One interval, endpoints are in reverse order, range is  [0, 2*pi], only interval on right of lower bound is produced.");

    testutil::TCASE(&save.TITLE, ctx)?;

    save.NIVALS = 1;

    save.BOUNDS[[1, 1]] = (spicelib::PI(ctx) / 2 as f64);
    save.BOUNDS[[2, 1]] = 0.0;

    save.MAXN = 2;

    support::REGLON(
        save.NIVALS,
        save.BOUNDS.as_slice(),
        save.MAXN,
        &mut save.NOUT,
        &mut save.MINLON,
        &mut save.MAXLON,
        save.OUTBDS.as_slice_mut(),
        save.SRCS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the output range.
    //
    testutil::CHCKSD(b"MINLON", save.MINLON, b"~", 0.0, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(
        b"MAXLON",
        save.MAXLON,
        b"~",
        spicelib::TWOPI(ctx),
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check number of output intervals.
    //
    save.XNOUT = 1;

    testutil::CHCKSI(b"NOUT", save.NOUT, b"=", save.XNOUT, 0, OK, ctx)?;

    //
    // Check output intervals.
    //
    save.TOL = VTIGHT;

    save.XBDS[[1, 1]] = (spicelib::PI(ctx) / 2 as f64);
    save.XBDS[[2, 1]] = spicelib::TWOPI(ctx);

    save.SIZE = (2 * save.NOUT);

    testutil::CHCKAD(
        b"OUTBDS",
        save.OUTBDS.as_slice(),
        b"~~",
        save.XBDS.as_slice(),
        save.SIZE,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check source intervals.
    //
    save.XSRCS[1] = 1;

    testutil::CHCKAI(
        b"SRCS",
        save.SRCS.as_slice(),
        b"=",
        save.XSRCS.as_slice(),
        1,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Two intervals, one interval has endpoints in reverse order, range is [0, 2*pi]. For reversed interval, only interval on right of lower bound is produced.");

    testutil::TCASE(&save.TITLE, ctx)?;

    save.NIVALS = 2;

    save.BOUNDS[[1, 1]] = (1.5 * spicelib::PI(ctx));
    save.BOUNDS[[2, 1]] = 0.0;

    save.BOUNDS[[1, 2]] = spicelib::PI(ctx);
    save.BOUNDS[[2, 2]] = (1.5 * spicelib::PI(ctx));

    save.MAXN = 2;

    support::REGLON(
        save.NIVALS,
        save.BOUNDS.as_slice(),
        save.MAXN,
        &mut save.NOUT,
        &mut save.MINLON,
        &mut save.MAXLON,
        save.OUTBDS.as_slice_mut(),
        save.SRCS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"MINLON", save.MINLON, b"~", 0.0, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(
        b"MAXLON",
        save.MAXLON,
        b"~",
        spicelib::TWOPI(ctx),
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check number of output intervals.
    //
    save.XNOUT = 2;

    testutil::CHCKSI(b"NOUT", save.NOUT, b"=", save.XNOUT, 0, OK, ctx)?;

    //
    // Check output intervals.
    //
    save.TOL = VTIGHT;

    save.XBDS[[1, 1]] = (1.5 * spicelib::PI(ctx));
    save.XBDS[[2, 1]] = spicelib::TWOPI(ctx);

    save.XBDS[[1, 2]] = spicelib::PI(ctx);
    save.XBDS[[2, 2]] = (1.5 * spicelib::PI(ctx));

    save.SIZE = (2 * save.NOUT);

    testutil::CHCKAD(
        b"OUTBDS",
        save.OUTBDS.as_slice(),
        b"~~",
        save.XBDS.as_slice(),
        save.SIZE,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check source intervals.
    //
    save.XSRCS[1] = 1;
    save.XSRCS[2] = 2;

    testutil::CHCKAI(
        b"SRCS",
        save.SRCS.as_slice(),
        b"=",
        save.XSRCS.as_slice(),
        save.NOUT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"One interval, endpoints are in reverse order, range is  [0, 2*pi], only interval on left of upper bound is produced.");

    testutil::TCASE(&save.TITLE, ctx)?;

    save.NIVALS = 1;

    save.BOUNDS[[1, 1]] = spicelib::TWOPI(ctx);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 2 as f64);

    save.MAXN = 2;

    support::REGLON(
        save.NIVALS,
        save.BOUNDS.as_slice(),
        save.MAXN,
        &mut save.NOUT,
        &mut save.MINLON,
        &mut save.MAXLON,
        save.OUTBDS.as_slice_mut(),
        save.SRCS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the output range.
    //
    testutil::CHCKSD(
        b"MINLON",
        save.MINLON,
        b"~",
        -spicelib::PI(ctx),
        VTIGHT,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"MAXLON",
        save.MAXLON,
        b"~",
        spicelib::PI(ctx),
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check number of output intervals.
    //
    save.XNOUT = 1;

    testutil::CHCKSI(b"NOUT", save.NOUT, b"=", save.XNOUT, 0, OK, ctx)?;

    //
    // Check output intervals.
    //
    save.TOL = VTIGHT;

    save.XBDS[[1, 1]] = 0.0;
    save.XBDS[[2, 1]] = (spicelib::PI(ctx) / 2 as f64);

    save.SIZE = (2 * save.NOUT);

    testutil::CHCKAD(
        b"OUTBDS",
        save.OUTBDS.as_slice(),
        b"~~",
        save.XBDS.as_slice(),
        save.SIZE,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check source intervals.
    //
    save.XSRCS[1] = 1;

    testutil::CHCKAI(
        b"SRCS",
        save.SRCS.as_slice(),
        b"=",
        save.XSRCS.as_slice(),
        1,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Two intervals, one interval has endpoints in reverse order, range is [0, 2*pi]. For reversed interval, only interval on left of upper bound is produced.");

    testutil::TCASE(&save.TITLE, ctx)?;

    save.NIVALS = 2;

    save.BOUNDS[[1, 1]] = spicelib::TWOPI(ctx);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 2 as f64);

    save.BOUNDS[[1, 2]] = spicelib::PI(ctx);
    save.BOUNDS[[2, 2]] = (1.5 * spicelib::PI(ctx));

    save.MAXN = 2;

    support::REGLON(
        save.NIVALS,
        save.BOUNDS.as_slice(),
        save.MAXN,
        &mut save.NOUT,
        &mut save.MINLON,
        &mut save.MAXLON,
        save.OUTBDS.as_slice_mut(),
        save.SRCS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(
        b"MINLON",
        save.MINLON,
        b"~",
        -spicelib::PI(ctx),
        VTIGHT,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"MAXLON",
        save.MAXLON,
        b"~",
        spicelib::PI(ctx),
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check number of output intervals.
    //
    save.XNOUT = 2;

    testutil::CHCKSI(b"NOUT", save.NOUT, b"=", save.XNOUT, 0, OK, ctx)?;

    //
    // Check output intervals.
    //
    save.TOL = VTIGHT;

    save.XBDS[[1, 1]] = 0.0;
    save.XBDS[[2, 1]] = (spicelib::PI(ctx) / 2 as f64);

    save.XBDS[[1, 2]] = -spicelib::PI(ctx);
    save.XBDS[[2, 2]] = -(spicelib::PI(ctx) / 2 as f64);

    save.SIZE = (2 * save.NOUT);

    testutil::CHCKAD(
        b"OUTBDS",
        save.OUTBDS.as_slice(),
        b"~~",
        save.XBDS.as_slice(),
        save.SIZE,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check source intervals.
    //
    save.XSRCS[1] = 1;
    save.XSRCS[2] = 2;

    testutil::CHCKAI(
        b"SRCS",
        save.SRCS.as_slice(),
        b"=",
        save.XSRCS.as_slice(),
        save.NOUT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"Two intervals in [0,2pi], no change needed.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    save.NIVALS = 2;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::PI(ctx);

    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 3 as f64);

    save.MAXN = 2;

    support::REGLON(
        save.NIVALS,
        save.BOUNDS.as_slice(),
        save.MAXN,
        &mut save.NOUT,
        &mut save.MINLON,
        &mut save.MAXLON,
        save.OUTBDS.as_slice_mut(),
        save.SRCS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the output range.
    //
    testutil::CHCKSD(b"MINLON", save.MINLON, b"~", 0.0, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(
        b"MAXLON",
        save.MAXLON,
        b"~",
        spicelib::TWOPI(ctx),
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check number of output intervals.
    //
    save.XNOUT = 2;

    testutil::CHCKSI(b"NOUT", save.NOUT, b"=", save.XNOUT, 0, OK, ctx)?;

    //
    // Check output intervals.
    //
    save.SIZE = (2 * save.NOUT);
    spicelib::MOVED(save.BOUNDS.as_slice(), save.SIZE, save.XBDS.as_slice_mut());

    testutil::CHCKAD(
        b"OUTBDS",
        save.OUTBDS.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.SIZE,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Check source intervals.
    //
    save.XSRCS[1] = 1;
    save.XSRCS[2] = 2;

    testutil::CHCKAI(
        b"SRCS",
        save.SRCS.as_slice(),
        b"=",
        save.XSRCS.as_slice(),
        save.NOUT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"Two intervals in [0,2pi], one interval has reversed bounds.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    save.NIVALS = 2;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::PI(ctx);

    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.MAXN = 3;

    support::REGLON(
        save.NIVALS,
        save.BOUNDS.as_slice(),
        save.MAXN,
        &mut save.NOUT,
        &mut save.MINLON,
        &mut save.MAXLON,
        save.OUTBDS.as_slice_mut(),
        save.SRCS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the output range.
    //
    testutil::CHCKSD(
        b"MINLON",
        save.MINLON,
        b"~",
        -spicelib::PI(ctx),
        VTIGHT,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"MAXLON",
        save.MAXLON,
        b"~",
        spicelib::PI(ctx),
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check number of output intervals.
    //
    save.XNOUT = 3;

    testutil::CHCKSI(b"NOUT", save.NOUT, b"=", save.XNOUT, 0, OK, ctx)?;

    //
    // Check output intervals.
    //

    save.XBDS[[1, 1]] = 0.0;
    save.XBDS[[2, 1]] = spicelib::PI(ctx);

    save.XBDS[[1, 2]] = -spicelib::PI(ctx);
    save.XBDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.XBDS[[1, 3]] = (spicelib::PI(ctx) / 3 as f64);
    save.XBDS[[2, 3]] = spicelib::PI(ctx);

    testutil::CHCKAD(
        b"OUTBDS",
        save.OUTBDS.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.SIZE,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Check source intervals.
    //
    save.XSRCS[1] = 1;
    save.XSRCS[2] = 2;
    save.XSRCS[3] = 2;

    testutil::CHCKAI(
        b"SRCS",
        save.SRCS.as_slice(),
        b"=",
        save.XSRCS.as_slice(),
        save.NOUT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"Two intervals in [0,2pi], both intervals have reversed bounds.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    save.NIVALS = 2;

    save.BOUNDS[[1, 1]] = (spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 8 as f64);

    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.MAXN = 4;

    support::REGLON(
        save.NIVALS,
        save.BOUNDS.as_slice(),
        save.MAXN,
        &mut save.NOUT,
        &mut save.MINLON,
        &mut save.MAXLON,
        save.OUTBDS.as_slice_mut(),
        save.SRCS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the output range.
    //
    testutil::CHCKSD(
        b"MINLON",
        save.MINLON,
        b"~",
        -spicelib::PI(ctx),
        VTIGHT,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"MAXLON",
        save.MAXLON,
        b"~",
        spicelib::PI(ctx),
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check number of output intervals.
    //
    save.XNOUT = 4;

    testutil::CHCKSI(b"NOUT", save.NOUT, b"=", save.XNOUT, 0, OK, ctx)?;

    //
    // Check output intervals.
    //
    save.XBDS[[1, 1]] = -spicelib::PI(ctx);
    save.XBDS[[2, 1]] = (spicelib::PI(ctx) / 8 as f64);

    save.XBDS[[1, 2]] = (spicelib::PI(ctx) / 4 as f64);
    save.XBDS[[2, 2]] = spicelib::PI(ctx);

    save.XBDS[[1, 3]] = -spicelib::PI(ctx);
    save.XBDS[[2, 3]] = (spicelib::PI(ctx) / 4 as f64);

    save.XBDS[[1, 4]] = (spicelib::PI(ctx) / 3 as f64);
    save.XBDS[[2, 4]] = spicelib::PI(ctx);

    testutil::CHCKAD(
        b"OUTBDS",
        save.OUTBDS.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.SIZE,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Check source intervals.
    //
    save.XSRCS[1] = 1;
    save.XSRCS[2] = 1;
    save.XSRCS[3] = 2;
    save.XSRCS[4] = 2;

    testutil::CHCKAI(
        b"SRCS",
        save.SRCS.as_slice(),
        b"=",
        save.XSRCS.as_slice(),
        save.NOUT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"Two intervals in [-pi,pi], no change needed.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    save.NIVALS = 2;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 2 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 4 as f64);

    save.MAXN = 2;

    support::REGLON(
        save.NIVALS,
        save.BOUNDS.as_slice(),
        save.MAXN,
        &mut save.NOUT,
        &mut save.MINLON,
        &mut save.MAXLON,
        save.OUTBDS.as_slice_mut(),
        save.SRCS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the output range.
    //
    testutil::CHCKSD(
        b"MINLON",
        save.MINLON,
        b"~",
        -spicelib::PI(ctx),
        VTIGHT,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"MAXLON",
        save.MAXLON,
        b"~",
        spicelib::PI(ctx),
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check number of output intervals.
    //
    save.XNOUT = 2;

    testutil::CHCKSI(b"NOUT", save.NOUT, b"=", save.XNOUT, 0, OK, ctx)?;

    //
    // Check output intervals.
    //
    save.SIZE = (2 * save.NOUT);
    spicelib::MOVED(save.BOUNDS.as_slice(), save.SIZE, save.XBDS.as_slice_mut());

    testutil::CHCKAD(
        b"OUTBDS",
        save.OUTBDS.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.SIZE,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Check source intervals.
    //
    save.XSRCS[1] = 1;
    save.XSRCS[2] = 2;

    testutil::CHCKAI(
        b"SRCS",
        save.SRCS.as_slice(),
        b"=",
        save.XSRCS.as_slice(),
        save.NOUT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"Two intervals in [-2pi, 2pi], one interval must be moved into the range [-pi, pi].",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    save.NIVALS = 2;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::PI(ctx);

    save.BOUNDS[[1, 2]] = -(((3 as f64) * spicelib::PI(ctx)) / 2 as f64);
    save.BOUNDS[[2, 2]] = -spicelib::PI(ctx);

    save.MAXN = 2;

    support::REGLON(
        save.NIVALS,
        save.BOUNDS.as_slice(),
        save.MAXN,
        &mut save.NOUT,
        &mut save.MINLON,
        &mut save.MAXLON,
        save.OUTBDS.as_slice_mut(),
        save.SRCS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the output range.
    //
    testutil::CHCKSD(
        b"MINLON",
        save.MINLON,
        b"~",
        -spicelib::PI(ctx),
        VTIGHT,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"MAXLON",
        save.MAXLON,
        b"~",
        spicelib::PI(ctx),
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check number of output intervals.
    //
    save.XNOUT = 2;

    testutil::CHCKSI(b"NOUT", save.NOUT, b"=", save.XNOUT, 0, OK, ctx)?;

    //
    // Check output intervals.
    //
    save.XBDS[[1, 1]] = 0.0;
    save.XBDS[[2, 1]] = spicelib::PI(ctx);

    save.XBDS[[1, 2]] = (spicelib::PI(ctx) / 2 as f64);
    save.XBDS[[2, 2]] = spicelib::PI(ctx);

    save.SIZE = (2 * save.NOUT);

    testutil::CHCKAD(
        b"OUTBDS",
        save.OUTBDS.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.SIZE,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Check source intervals.
    //
    save.XSRCS[1] = 1;
    save.XSRCS[2] = 2;

    testutil::CHCKAI(
        b"SRCS",
        save.SRCS.as_slice(),
        b"=",
        save.XSRCS.as_slice(),
        save.NOUT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Two intervals in [-2pi, 2pi], one interval has reversed bounds and must be moved into the range [-pi, pi].");

    testutil::TCASE(&save.TITLE, ctx)?;

    save.NIVALS = 2;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = -(spicelib::PI(ctx) / 2 as f64);

    save.BOUNDS[[1, 2]] = -(((3 as f64) * spicelib::PI(ctx)) / 2 as f64);
    save.BOUNDS[[2, 2]] = -spicelib::PI(ctx);

    save.MAXN = 3;

    support::REGLON(
        save.NIVALS,
        save.BOUNDS.as_slice(),
        save.MAXN,
        &mut save.NOUT,
        &mut save.MINLON,
        &mut save.MAXLON,
        save.OUTBDS.as_slice_mut(),
        save.SRCS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the output range.
    //
    testutil::CHCKSD(
        b"MINLON",
        save.MINLON,
        b"~",
        -spicelib::PI(ctx),
        VTIGHT,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"MAXLON",
        save.MAXLON,
        b"~",
        spicelib::PI(ctx),
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check number of output intervals.
    //
    save.XNOUT = 3;

    testutil::CHCKSI(b"NOUT", save.NOUT, b"=", save.XNOUT, 0, OK, ctx)?;

    //
    // Check output intervals.
    //
    save.XBDS[[1, 1]] = -spicelib::PI(ctx);
    save.XBDS[[2, 1]] = -(spicelib::PI(ctx) / 2 as f64);

    save.XBDS[[1, 2]] = 0.0;
    save.XBDS[[2, 2]] = spicelib::PI(ctx);

    save.XBDS[[1, 3]] = (spicelib::PI(ctx) / 2 as f64);
    save.XBDS[[2, 3]] = spicelib::PI(ctx);

    save.SIZE = (2 * save.NOUT);

    testutil::CHCKAD(
        b"OUTBDS",
        save.OUTBDS.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.SIZE,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Check source intervals.
    //
    save.XSRCS[1] = 1;
    save.XSRCS[2] = 1;
    save.XSRCS[3] = 2;

    testutil::CHCKAI(
        b"SRCS",
        save.SRCS.as_slice(),
        b"=",
        save.XSRCS.as_slice(),
        save.NOUT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Two intervals in [-2pi, 2pi], both intervals have reversed bounds and must be moved into the range [-pi, pi].");

    testutil::TCASE(&save.TITLE, ctx)?;

    save.NIVALS = 2;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = -(spicelib::PI(ctx) / 2 as f64);

    save.BOUNDS[[1, 2]] = -(((3 as f64) * spicelib::PI(ctx)) / 2 as f64);
    save.BOUNDS[[2, 2]] = -(((7 as f64) * spicelib::PI(ctx)) / 4 as f64);

    save.MAXN = 4;

    support::REGLON(
        save.NIVALS,
        save.BOUNDS.as_slice(),
        save.MAXN,
        &mut save.NOUT,
        &mut save.MINLON,
        &mut save.MAXLON,
        save.OUTBDS.as_slice_mut(),
        save.SRCS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the output range.
    //
    testutil::CHCKSD(
        b"MINLON",
        save.MINLON,
        b"~",
        -spicelib::PI(ctx),
        VTIGHT,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"MAXLON",
        save.MAXLON,
        b"~",
        spicelib::PI(ctx),
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check number of output intervals.
    //
    save.XNOUT = 4;

    testutil::CHCKSI(b"NOUT", save.NOUT, b"=", save.XNOUT, 0, OK, ctx)?;

    //
    // Check output intervals.
    //
    save.XBDS[[1, 1]] = -spicelib::PI(ctx);
    save.XBDS[[2, 1]] = -(spicelib::PI(ctx) / 2 as f64);

    save.XBDS[[1, 2]] = 0.0;
    save.XBDS[[2, 2]] = spicelib::PI(ctx);

    save.XBDS[[1, 3]] = -spicelib::PI(ctx);
    save.XBDS[[2, 3]] = (spicelib::PI(ctx) / 4 as f64);

    save.XBDS[[1, 4]] = (spicelib::PI(ctx) / 2 as f64);
    save.XBDS[[2, 4]] = spicelib::PI(ctx);

    testutil::CHCKAD(
        b"OUTBDS",
        save.OUTBDS.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.SIZE,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Check source intervals.
    //
    save.XSRCS[1] = 1;
    save.XSRCS[2] = 1;
    save.XSRCS[3] = 2;
    save.XSRCS[4] = 2;

    testutil::CHCKAI(
        b"SRCS",
        save.SRCS.as_slice(),
        b"=",
        save.XSRCS.as_slice(),
        save.NOUT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Two intervals in [-2pi, 2pi], both intervals have reversed bounds and must be moved into the range [-pi, pi]. For the first input interval, only the interval to the right of the lower bound is produced. For the second input interval, only the interval to the left of the upper bound is produced.");

    testutil::TCASE(&save.TITLE, ctx)?;

    save.NIVALS = 2;

    save.BOUNDS[[1, 1]] = (((3 as f64) * spicelib::PI(ctx)) / 2 as f64);
    save.BOUNDS[[2, 1]] = spicelib::PI(ctx);

    save.BOUNDS[[1, 2]] = -spicelib::PI(ctx);
    save.BOUNDS[[2, 2]] = -(((7 as f64) * spicelib::PI(ctx)) / 4 as f64);

    save.MAXN = 4;

    support::REGLON(
        save.NIVALS,
        save.BOUNDS.as_slice(),
        save.MAXN,
        &mut save.NOUT,
        &mut save.MINLON,
        &mut save.MAXLON,
        save.OUTBDS.as_slice_mut(),
        save.SRCS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the output range.
    //
    testutil::CHCKSD(
        b"MINLON",
        save.MINLON,
        b"~",
        -spicelib::PI(ctx),
        VTIGHT,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"MAXLON",
        save.MAXLON,
        b"~",
        spicelib::PI(ctx),
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Check number of output intervals.
    //
    save.XNOUT = 2;

    testutil::CHCKSI(b"NOUT", save.NOUT, b"=", save.XNOUT, 0, OK, ctx)?;

    //
    // Check output intervals.
    //
    save.XBDS[[1, 1]] = -(spicelib::PI(ctx) / 2 as f64);
    save.XBDS[[2, 1]] = spicelib::PI(ctx);

    save.XBDS[[1, 2]] = -spicelib::PI(ctx);
    save.XBDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.SIZE = (2 * save.NOUT);

    testutil::CHCKAD(
        b"OUTBDS",
        save.OUTBDS.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.SIZE,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Check source intervals.
    //
    save.XSRCS[1] = 1;
    save.XSRCS[2] = 2;

    testutil::CHCKAI(
        b"SRCS",
        save.SRCS.as_slice(),
        b"=",
        save.XSRCS.as_slice(),
        save.NOUT,
        OK,
        ctx,
    )?;

    //**********************************************************************
    //
    //     REGLON error cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Output array is too small.", ctx)?;

    save.NIVALS = 2;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::PI(ctx);
    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 2 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 2 as f64);

    save.MAXN = 1;

    support::REGLON(
        save.NIVALS,
        save.BOUNDS.as_slice(),
        save.MAXN,
        &mut save.NOUT,
        &mut save.MINLON,
        &mut save.MAXLON,
        save.OUTBDS.as_slice_mut(),
        save.SRCS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(ARRAYTOOSMALL)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Longitudes are out of range.", ctx)?;

    save.NIVALS = 2;

    save.BOUNDS[[1, 1]] = -((3 as f64) * spicelib::PI(ctx));
    save.BOUNDS[[2, 1]] = spicelib::PI(ctx);
    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 2 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 2 as f64);

    save.MAXN = 3;

    support::REGLON(
        save.NIVALS,
        save.BOUNDS.as_slice(),
        save.MAXN,
        &mut save.NOUT,
        &mut save.MINLON,
        &mut save.MAXLON,
        save.OUTBDS.as_slice_mut(),
        save.SRCS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    save.BOUNDS[[1, 1]] = -((2 as f64) * spicelib::PI(ctx));
    save.BOUNDS[[2, 1]] = ((3 as f64) * spicelib::PI(ctx));
    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 2 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 2 as f64);

    save.MAXN = 3;

    support::REGLON(
        save.NIVALS,
        save.BOUNDS.as_slice(),
        save.MAXN,
        &mut save.NOUT,
        &mut save.MINLON,
        &mut save.MAXLON,
        save.OUTBDS.as_slice_mut(),
        save.SRCS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Longitude bounds are equal.", ctx)?;

    save.NIVALS = 2;

    save.BOUNDS[[1, 1]] = -((2 as f64) * spicelib::PI(ctx));
    save.BOUNDS[[2, 1]] = -((2 as f64) * spicelib::PI(ctx));
    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 2 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 2 as f64);

    save.MAXN = 3;

    support::REGLON(
        save.NIVALS,
        save.BOUNDS.as_slice(),
        save.MAXN,
        &mut save.NOUT,
        &mut save.MINLON,
        &mut save.MAXLON,
        save.OUTBDS.as_slice_mut(),
        save.SRCS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(ZEROBOUNDSEXTENT)", OK, ctx)?;

    save.BOUNDS[[1, 1]] = -((2 as f64) * spicelib::PI(ctx));
    save.BOUNDS[[2, 1]] = ((2 as f64) * spicelib::PI(ctx));
    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 2 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 2 as f64);

    save.MAXN = 3;

    support::REGLON(
        save.NIVALS,
        save.BOUNDS.as_slice(),
        save.MAXN,
        &mut save.NOUT,
        &mut save.MINLON,
        &mut save.MAXLON,
        save.OUTBDS.as_slice_mut(),
        save.SRCS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(ZEROBOUNDSEXTENT)", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
