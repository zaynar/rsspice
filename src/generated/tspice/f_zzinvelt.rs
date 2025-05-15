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

struct SaveVars {
    BOUNDS: StackArray2D<f64, 6>,
    CORPAR: StackArray<f64, 10>,
    L: f64,
    MARGIN: f64,
    P: StackArray<f64, 3>,
    CORSYS: i32,
    EXCLUD: i32,
    INSIDE: bool,
    XIN: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BOUNDS = StackArray2D::<f64, 6>::new(1..=2, 1..=3);
        let mut CORPAR = StackArray::<f64, 10>::new(1..=NSYPAR);
        let mut L: f64 = 0.0;
        let mut MARGIN: f64 = 0.0;
        let mut P = StackArray::<f64, 3>::new(1..=3);
        let mut CORSYS: i32 = 0;
        let mut EXCLUD: i32 = 0;
        let mut INSIDE: bool = false;
        let mut XIN: bool = false;

        Self {
            BOUNDS,
            CORPAR,
            L,
            MARGIN,
            P,
            CORSYS,
            EXCLUD,
            INSIDE,
            XIN,
        }
    }
}

//$Procedure F_ZZINVELT ( ZZINVELT tests )
pub fn F_ZZINVELT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    testutil::TOPEN(b"F_ZZINVELT", ctx)?;

    //***********************************************************************
    //
    //     Normal cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"RECSYS: interior case. Exclude none. Zero MARGIN.", ctx)?;

    save.CORSYS = RECSYS;

    spicelib::CLEARD(NSYPAR, save.CORPAR.as_slice_mut());

    save.EXCLUD = 0;
    save.MARGIN = 0.0;

    save.BOUNDS[[1, 1]] = 1.0;
    save.BOUNDS[[2, 1]] = 3.0;

    save.BOUNDS[[1, 2]] = -3.0;
    save.BOUNDS[[2, 2]] = -1.0;

    save.BOUNDS[[1, 3]] = 6.0;
    save.BOUNDS[[2, 3]] = 8.0;

    spicelib::VPACK(2.0, -2.0, 7.0, save.P.as_slice_mut());

    spicelib::ZZINVELT(
        save.P.as_slice(),
        save.CORSYS,
        save.CORPAR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XIN = true;

    testutil::CHCKSL(b"INSIDE", save.INSIDE, save.XIN, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"RECSYS: interior case. Exclude none. MARGIN > 0.", ctx)?;

    save.CORSYS = RECSYS;

    spicelib::CLEARD(NSYPAR, save.CORPAR.as_slice_mut());

    save.EXCLUD = 0;
    save.MARGIN = 0.01;

    save.BOUNDS[[1, 1]] = 1.0;
    save.BOUNDS[[2, 1]] = 3.0;

    save.BOUNDS[[1, 2]] = -3.0;
    save.BOUNDS[[2, 2]] = -1.0;

    save.BOUNDS[[1, 3]] = 6.0;
    save.BOUNDS[[2, 3]] = 8.0;

    save.L = (save.BOUNDS[[2, 3]] - save.BOUNDS[[1, 3]]);

    spicelib::VPACK(
        2.0,
        -2.0,
        (8.0 + ((0.5 * save.MARGIN) * save.L)),
        save.P.as_slice_mut(),
    );

    spicelib::ZZINVELT(
        save.P.as_slice(),
        save.CORSYS,
        save.CORPAR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XIN = true;

    testutil::CHCKSL(b"INSIDE", save.INSIDE, save.XIN, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"RECSYS: exterior case. Exclude none. MARGIN > 0.", ctx)?;

    save.CORSYS = RECSYS;

    spicelib::CLEARD(NSYPAR, save.CORPAR.as_slice_mut());

    save.EXCLUD = 0;
    save.MARGIN = 0.01;

    save.BOUNDS[[1, 1]] = 1.0;
    save.BOUNDS[[2, 1]] = 3.0;

    save.BOUNDS[[1, 2]] = -3.0;
    save.BOUNDS[[2, 2]] = -1.0;

    save.BOUNDS[[1, 3]] = 6.0;
    save.BOUNDS[[2, 3]] = 8.0;

    save.L = (save.BOUNDS[[2, 3]] - save.BOUNDS[[1, 3]]);

    spicelib::VPACK(
        2.0,
        -2.0,
        (8.0 + ((1.5 * save.MARGIN) * save.L)),
        save.P.as_slice_mut(),
    );

    spicelib::ZZINVELT(
        save.P.as_slice(),
        save.CORSYS,
        save.CORPAR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XIN = false;

    testutil::CHCKSL(b"INSIDE", save.INSIDE, save.XIN, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"RECSYS: interior case. Exclude Z. MARGIN > 0.", ctx)?;

    save.CORSYS = RECSYS;

    spicelib::CLEARD(NSYPAR, save.CORPAR.as_slice_mut());

    save.EXCLUD = 3;
    save.MARGIN = 0.01;

    save.BOUNDS[[1, 1]] = 1.0;
    save.BOUNDS[[2, 1]] = 3.0;

    save.BOUNDS[[1, 2]] = -3.0;
    save.BOUNDS[[2, 2]] = -1.0;

    save.BOUNDS[[1, 3]] = 6.0;
    save.BOUNDS[[2, 3]] = 8.0;

    save.L = (save.BOUNDS[[2, 3]] - save.BOUNDS[[1, 3]]);

    spicelib::VPACK(
        2.0,
        -2.0,
        (8.0 + ((1.5 * save.MARGIN) * save.L)),
        save.P.as_slice_mut(),
    );

    spicelib::ZZINVELT(
        save.P.as_slice(),
        save.CORSYS,
        save.CORPAR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XIN = true;

    testutil::CHCKSL(b"INSIDE", save.INSIDE, save.XIN, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"LATSYS: interior case. Exclude none. Zero MARGIN.", ctx)?;

    save.CORSYS = LATSYS;

    spicelib::CLEARD(NSYPAR, save.CORPAR.as_slice_mut());

    save.EXCLUD = 0;
    save.MARGIN = 0.0;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 3 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 6.0;
    save.BOUNDS[[2, 3]] = 8.0;

    spicelib::VPACK(7.0, 0.0, 0.0, save.P.as_slice_mut());

    spicelib::ZZINVELT(
        save.P.as_slice(),
        save.CORSYS,
        save.CORPAR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XIN = true;

    testutil::CHCKSL(b"INSIDE", save.INSIDE, save.XIN, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"LATSYS: interior case. Exclude none. MARGIN > 0.", ctx)?;

    save.CORSYS = LATSYS;

    spicelib::CLEARD(NSYPAR, save.CORPAR.as_slice_mut());

    save.EXCLUD = 0;
    save.MARGIN = 0.01;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 3 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 6.0;
    save.BOUNDS[[2, 3]] = 8.0;

    spicelib::VPACK(
        (8.0 + ((0.5 * save.MARGIN) * 8.0)),
        0.0,
        0.0,
        save.P.as_slice_mut(),
    );

    spicelib::ZZINVELT(
        save.P.as_slice(),
        save.CORSYS,
        save.CORPAR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XIN = true;

    testutil::CHCKSL(b"INSIDE", save.INSIDE, save.XIN, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"LATSYS: exterior case. Exclude none. MARGIN > 0.", ctx)?;

    save.CORSYS = LATSYS;

    spicelib::CLEARD(NSYPAR, save.CORPAR.as_slice_mut());

    save.EXCLUD = 0;
    save.MARGIN = 0.01;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 3 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 6.0;
    save.BOUNDS[[2, 3]] = 8.0;

    spicelib::VPACK(
        (8.0 + ((1.5 * save.MARGIN) * 8.0)),
        0.0,
        0.0,
        save.P.as_slice_mut(),
    );

    spicelib::ZZINVELT(
        save.P.as_slice(),
        save.CORSYS,
        save.CORPAR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XIN = false;

    testutil::CHCKSL(b"INSIDE", save.INSIDE, save.XIN, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"LATSYS: interior case. Exclude radius. MARGIN > 0.", ctx)?;

    save.CORSYS = LATSYS;

    spicelib::CLEARD(NSYPAR, save.CORPAR.as_slice_mut());

    save.EXCLUD = 3;
    save.MARGIN = 0.01;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 3 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 6.0;
    save.BOUNDS[[2, 3]] = 8.0;

    spicelib::VPACK(
        (8.0 + ((1.5 * save.MARGIN) * 8.0)),
        0.0,
        0.0,
        save.P.as_slice_mut(),
    );

    spicelib::ZZINVELT(
        save.P.as_slice(),
        save.CORSYS,
        save.CORPAR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XIN = true;

    testutil::CHCKSL(b"INSIDE", save.INSIDE, save.XIN, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"PDTSYS: interior case. Exclude none. Zero MARGIN.", ctx)?;

    save.CORSYS = PDTSYS;

    spicelib::CLEARD(NSYPAR, save.CORPAR.as_slice_mut());

    save.CORPAR[1] = 8.0;
    save.CORPAR[2] = 0.5;

    save.EXCLUD = 0;
    save.MARGIN = 0.0;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 3 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = -1.0;
    save.BOUNDS[[2, 3]] = 1.0;

    spicelib::GEOREC(
        0.0,
        0.0,
        0.5,
        save.CORPAR[1],
        save.CORPAR[2],
        save.P.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZINVELT(
        save.P.as_slice(),
        save.CORSYS,
        save.CORPAR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XIN = true;

    testutil::CHCKSL(b"INSIDE", save.INSIDE, save.XIN, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"PDTSYS: interior case. Exclude none. MARGIN > 0.", ctx)?;

    save.CORPAR[1] = 8.0;
    save.CORPAR[2] = 0.5;

    save.EXCLUD = 0;
    save.MARGIN = 0.01;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 3 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = -1.0;
    save.BOUNDS[[2, 3]] = 1.0;

    save.L = (save.BOUNDS[[2, 3]] - save.BOUNDS[[1, 3]]);

    spicelib::GEOREC(
        0.0,
        0.0,
        (1.0 + ((0.5 * save.L) * save.MARGIN)),
        save.CORPAR[1],
        save.CORPAR[2],
        save.P.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZINVELT(
        save.P.as_slice(),
        save.CORSYS,
        save.CORPAR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XIN = true;

    testutil::CHCKSL(b"INSIDE", save.INSIDE, save.XIN, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"PDTSYS: exterior case. Exclude none. MARGIN > 0.", ctx)?;

    save.CORSYS = PDTSYS;

    spicelib::CLEARD(NSYPAR, save.CORPAR.as_slice_mut());

    save.CORPAR[1] = 8.0;
    save.CORPAR[2] = 0.5;

    save.EXCLUD = 0;
    save.MARGIN = 0.01;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 3 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = -1.0;
    save.BOUNDS[[2, 3]] = 1.0;

    save.L = (save.BOUNDS[[2, 3]] - save.BOUNDS[[1, 3]]);

    spicelib::GEOREC(
        0.0,
        0.0,
        (1.0 + ((1.5 * save.L) * save.MARGIN)),
        save.CORPAR[1],
        save.CORPAR[2],
        save.P.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZINVELT(
        save.P.as_slice(),
        save.CORSYS,
        save.CORPAR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XIN = false;

    testutil::CHCKSL(b"INSIDE", save.INSIDE, save.XIN, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"PDTSYS: interior case. Exclude altitude. MARGIN > 0.", ctx)?;

    save.CORSYS = PDTSYS;

    spicelib::CLEARD(NSYPAR, save.CORPAR.as_slice_mut());

    save.CORPAR[1] = 8.0;
    save.CORPAR[2] = 0.5;

    save.EXCLUD = 3;
    save.MARGIN = 0.01;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 3 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 6.0;
    save.BOUNDS[[2, 3]] = 8.0;

    spicelib::VPACK(
        (8.0 + ((1.5 * save.MARGIN) * 8.0)),
        0.0,
        0.0,
        save.P.as_slice_mut(),
    );

    spicelib::ZZINVELT(
        save.P.as_slice(),
        save.CORSYS,
        save.CORPAR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XIN = true;

    testutil::CHCKSL(b"INSIDE", save.INSIDE, save.XIN, OK, ctx)?;

    //***********************************************************************
    //
    //     Error cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error: Negative margin.", ctx)?;

    save.CORSYS = PDTSYS;

    spicelib::CLEARD(NSYPAR, save.CORPAR.as_slice_mut());

    save.CORPAR[1] = 8.0;
    save.CORPAR[2] = 0.5;

    save.EXCLUD = 3;
    save.MARGIN = -0.01;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 3 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 6.0;
    save.BOUNDS[[2, 3]] = 8.0;

    spicelib::VPACK(
        (8.0 + ((1.5 * save.MARGIN) * 8.0)),
        0.0,
        0.0,
        save.P.as_slice_mut(),
    );

    spicelib::ZZINVELT(
        save.P.as_slice(),
        save.CORSYS,
        save.CORPAR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error: Invalid coordinate system code.", ctx)?;

    save.MARGIN = 0.0;

    spicelib::ZZINVELT(
        save.P.as_slice(),
        -1,
        save.CORPAR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    spicelib::ZZINVELT(
        save.P.as_slice(),
        5,
        save.CORPAR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error: Invalid exclude index.", ctx)?;

    save.MARGIN = 0.0;

    save.EXCLUD = -1;

    spicelib::ZZINVELT(
        save.P.as_slice(),
        PDTSYS,
        save.CORPAR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    save.EXCLUD = 4;

    spicelib::ZZINVELT(
        save.P.as_slice(),
        PDTSYS,
        save.CORPAR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error: invalid rectangular coordinate bounds.", ctx)?;

    save.CORSYS = RECSYS;

    spicelib::CLEARD(NSYPAR, save.CORPAR.as_slice_mut());

    save.EXCLUD = 0;
    save.MARGIN = 0.0;

    save.BOUNDS[[1, 1]] = 1.0;
    save.BOUNDS[[2, 1]] = -3.0;

    save.BOUNDS[[1, 2]] = -3.0;
    save.BOUNDS[[2, 2]] = -1.0;

    save.BOUNDS[[1, 3]] = 6.0;
    save.BOUNDS[[2, 3]] = 8.0;

    spicelib::VPACK(2.0, -2.0, 7.0, save.P.as_slice_mut());

    spicelib::ZZINVELT(
        save.P.as_slice(),
        save.CORSYS,
        save.CORPAR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BOUNDSOUTOFORDER)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error: invalid planetodetic coordinate parameters.", ctx)?;

    save.CORSYS = PDTSYS;

    spicelib::CLEARD(NSYPAR, save.CORPAR.as_slice_mut());

    save.CORPAR[1] = -8.0;
    save.CORPAR[2] = 0.5;

    save.EXCLUD = 3;
    save.MARGIN = -0.01;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 3 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 6.0;
    save.BOUNDS[[2, 3]] = 8.0;

    spicelib::VPACK(
        (8.0 + ((1.5 * save.MARGIN) * 8.0)),
        0.0,
        0.0,
        save.P.as_slice_mut(),
    );

    spicelib::ZZINVELT(
        save.P.as_slice(),
        save.CORSYS,
        save.CORPAR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    save.CORPAR[1] = 8.0;
    save.CORPAR[2] = 1.1;

    spicelib::ZZINVELT(
        save.P.as_slice(),
        save.CORSYS,
        save.CORPAR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
