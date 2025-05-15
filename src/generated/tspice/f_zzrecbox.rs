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
const VTIGHT: f64 = 0.00000000000001;

struct SaveVars {
    BOUNDS: StackArray2D<f64, 6>,
    CENTER: StackArray<f64, 3>,
    LX: f64,
    LY: f64,
    LZ: f64,
    RADIUS: f64,
    TOL: f64,
    XCTR: StackArray<f64, 3>,
    XR: f64,
    XLX: f64,
    XLY: f64,
    XLZ: f64,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BOUNDS = StackArray2D::<f64, 6>::new(1..=2, 1..=3);
        let mut CENTER = StackArray::<f64, 3>::new(1..=3);
        let mut LX: f64 = 0.0;
        let mut LY: f64 = 0.0;
        let mut LZ: f64 = 0.0;
        let mut RADIUS: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut XCTR = StackArray::<f64, 3>::new(1..=3);
        let mut XR: f64 = 0.0;
        let mut XLX: f64 = 0.0;
        let mut XLY: f64 = 0.0;
        let mut XLZ: f64 = 0.0;

        Self {
            BOUNDS,
            CENTER,
            LX,
            LY,
            LZ,
            RADIUS,
            TOL,
            XCTR,
            XR,
            XLX,
            XLY,
            XLZ,
        }
    }
}

//$Procedure F_ZZRECBOX ( ZZRECBOX tests )
pub fn F_ZZRECBOX(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    testutil::TOPEN(b"F_ZZRECBOX", ctx)?;

    //***********************************************************************
    //
    //     Normal cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Box includes origin.", ctx)?;

    save.BOUNDS[[1, 1]] = -1.0;
    save.BOUNDS[[2, 1]] = 3.0;

    save.BOUNDS[[1, 2]] = -3.0;
    save.BOUNDS[[2, 2]] = 4.0;

    save.BOUNDS[[1, 3]] = -6.0;
    save.BOUNDS[[2, 3]] = 8.0;

    for I in 1..=3 {
        save.XCTR[I] = ((save.BOUNDS[[1, I]] + save.BOUNDS[[2, I]]) / 2 as f64);
    }

    save.XLX = (save.BOUNDS[[2, 1]] - save.BOUNDS[[1, 1]]);
    save.XLY = (save.BOUNDS[[2, 2]] - save.BOUNDS[[1, 2]]);
    save.XLZ = (save.BOUNDS[[2, 3]] - save.BOUNDS[[1, 3]]);

    save.XR =
        (f64::sqrt(((f64::powi(save.XLX, 2) + f64::powi(save.XLY, 2)) + f64::powi(save.XLZ, 2)))
            / 2 as f64);

    spicelib::ZZRECBOX(
        save.BOUNDS.as_slice(),
        save.CENTER.as_slice_mut(),
        &mut save.LX,
        &mut save.LY,
        &mut save.LZ,
        &mut save.RADIUS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"CENTER",
        save.CENTER.as_slice(),
        b"~~/",
        save.XCTR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    testutil::CHCKSD(b"LX", save.LX, b"~", save.XLX, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LY", save.LY, b"~", save.XLY, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LZ", save.LZ, b"~", save.XLZ, save.TOL, OK, ctx)?;

    testutil::CHCKSD(b"RADIUS", save.RADIUS, b"~", save.XR, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Box excludes origin.", ctx)?;

    save.BOUNDS[[1, 1]] = 3.0;
    save.BOUNDS[[2, 1]] = 5.0;

    save.BOUNDS[[1, 2]] = -3.0;
    save.BOUNDS[[2, 2]] = -1.0;

    save.BOUNDS[[1, 3]] = -6.0;
    save.BOUNDS[[2, 3]] = -5.0;

    for I in 1..=3 {
        save.XCTR[I] = ((save.BOUNDS[[1, I]] + save.BOUNDS[[2, I]]) / 2 as f64);
    }

    save.XLX = (save.BOUNDS[[2, 1]] - save.BOUNDS[[1, 1]]);
    save.XLY = (save.BOUNDS[[2, 2]] - save.BOUNDS[[1, 2]]);
    save.XLZ = (save.BOUNDS[[2, 3]] - save.BOUNDS[[1, 3]]);

    save.XR =
        (f64::sqrt(((f64::powi(save.XLX, 2) + f64::powi(save.XLY, 2)) + f64::powi(save.XLZ, 2)))
            / 2 as f64);

    spicelib::ZZRECBOX(
        save.BOUNDS.as_slice(),
        save.CENTER.as_slice_mut(),
        &mut save.LX,
        &mut save.LY,
        &mut save.LZ,
        &mut save.RADIUS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"CENTER",
        save.CENTER.as_slice(),
        b"~~/",
        save.XCTR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    testutil::CHCKSD(b"LX", save.LX, b"~", save.XLX, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LY", save.LY, b"~", save.XLY, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"LZ", save.LZ, b"~", save.XLZ, save.TOL, OK, ctx)?;

    testutil::CHCKSD(b"RADIUS", save.RADIUS, b"~", save.XR, save.TOL, OK, ctx)?;

    //***********************************************************************
    //
    //     Error cases
    //
    //***********************************************************************

    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error: invalid rectangular coordinate bounds.", ctx)?;

    save.BOUNDS[[1, 1]] = 1.0;
    save.BOUNDS[[2, 1]] = -3.0;

    save.BOUNDS[[1, 2]] = -3.0;
    save.BOUNDS[[2, 2]] = -1.0;

    save.BOUNDS[[1, 3]] = 6.0;
    save.BOUNDS[[2, 3]] = 8.0;

    spicelib::ZZRECBOX(
        save.BOUNDS.as_slice(),
        save.CENTER.as_slice_mut(),
        &mut save.LX,
        &mut save.LY,
        &mut save.LZ,
        &mut save.RADIUS,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BOUNDSOUTOFORDER)", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
