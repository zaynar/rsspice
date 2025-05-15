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
const TIGHT: f64 = 0.000000000001;
const VTIGHT: f64 = 0.00000000000001;

struct SaveVars {
    A: f64,
    ALT: f64,
    AMAX: f64,
    AMIN: f64,
    B: f64,
    BMAX: f64,
    BMIN: f64,
    BOUNDS: StackArray2D<f64, 6>,
    CORPAR: StackArray<f64, 10>,
    DELTA: f64,
    DSKDSC: StackArray<f64, 24>,
    F: f64,
    H: f64,
    L: f64,
    LAT: f64,
    LON: f64,
    MARGIN: f64,
    RAYDIR: StackArray<f64, 3>,
    RE: f64,
    TOL: f64,
    VERTEX: StackArray<f64, 3>,
    W: f64,
    XPT: StackArray<f64, 3>,
    XXPT: StackArray<f64, 3>,
    NXPTS: i32,
    XNXPTS: i32,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut A: f64 = 0.0;
        let mut ALT: f64 = 0.0;
        let mut AMAX: f64 = 0.0;
        let mut AMIN: f64 = 0.0;
        let mut B: f64 = 0.0;
        let mut BMAX: f64 = 0.0;
        let mut BMIN: f64 = 0.0;
        let mut BOUNDS = StackArray2D::<f64, 6>::new(1..=2, 1..=3);
        let mut CORPAR = StackArray::<f64, 10>::new(1..=NSYPAR);
        let mut DELTA: f64 = 0.0;
        let mut DSKDSC = StackArray::<f64, 24>::new(1..=DSKDSZ);
        let mut F: f64 = 0.0;
        let mut H: f64 = 0.0;
        let mut L: f64 = 0.0;
        let mut LAT: f64 = 0.0;
        let mut LON: f64 = 0.0;
        let mut MARGIN: f64 = 0.0;
        let mut RAYDIR = StackArray::<f64, 3>::new(1..=3);
        let mut RE: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut VERTEX = StackArray::<f64, 3>::new(1..=3);
        let mut W: f64 = 0.0;
        let mut XPT = StackArray::<f64, 3>::new(1..=3);
        let mut XXPT = StackArray::<f64, 3>::new(1..=3);
        let mut NXPTS: i32 = 0;
        let mut XNXPTS: i32 = 0;
        let mut FOUND: bool = false;

        Self {
            A,
            ALT,
            AMAX,
            AMIN,
            B,
            BMAX,
            BMIN,
            BOUNDS,
            CORPAR,
            DELTA,
            DSKDSC,
            F,
            H,
            L,
            LAT,
            LON,
            MARGIN,
            RAYDIR,
            RE,
            TOL,
            VERTEX,
            W,
            XPT,
            XXPT,
            NXPTS,
            XNXPTS,
            FOUND,
        }
    }
}

//$Procedure F_ZZRYTELT ( ZZRYTELT tests )
pub fn F_ZZRYTELT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    testutil::TOPEN(b"F_ZZRYTELT", ctx)?;

    //***********************************************************************
    //
    //     Normal cases
    //
    //***********************************************************************

    //***********************************************************************
    //
    //
    //     Rectangular coordinates
    //
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Rectangular coordinate hit case. Zero MARGIN.", ctx)?;

    //
    // Set the coordinate system and bounds in the DSK descriptor.
    //
    spicelib::CLEARD(DSKDSZ, save.DSKDSC.as_slice_mut());

    save.L = 40.0;
    save.W = 20.0;
    save.H = 10.0;

    save.BOUNDS[[1, 1]] = -(save.L / 2 as f64);
    save.BOUNDS[[2, 1]] = (save.L / 2 as f64);
    save.BOUNDS[[1, 2]] = -(save.W / 2 as f64);
    save.BOUNDS[[2, 2]] = (save.W / 2 as f64);
    save.BOUNDS[[1, 3]] = -(save.H / 2 as f64);
    save.BOUNDS[[2, 3]] = (save.H / 2 as f64);

    spicelib::MOVED(save.BOUNDS.as_slice(), 6, save.DSKDSC.subarray_mut(MN1IDX));

    save.DSKDSC[SYSIDX] = RECSYS as f64;

    spicelib::VPACK(0.0, 0.0, ((2 as f64) * save.H), save.VERTEX.as_slice_mut());
    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    save.MARGIN = 0.0;

    spicelib::ZZRYTELT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.DSKDSC.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an intercept.
    //
    save.XNXPTS = 1;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", save.XNXPTS, 0, OK, ctx)?;

    //
    // Create the expected intercept point.
    //
    spicelib::VPACK(0.0, 0.0, (save.H / 2 as f64), save.XXPT.as_slice_mut());

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"XPT",
        save.XPT.as_slice(),
        b"~~/",
        save.XXPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Rectangular coordinate miss case. Zero MARGIN.", ctx)?;

    //
    // Use the descriptor from the previous case.
    //
    //
    // Shift the vertex so the ray just misses the volume element on
    // the +X side.
    //
    save.DELTA = 0.000000000001;

    spicelib::VPACK(
        ((save.L / 2 as f64) + save.DELTA),
        0.0,
        ((2 as f64) * save.H),
        save.VERTEX.as_slice_mut(),
    );

    spicelib::VPACK(0.0, 0.0, -1.0, save.RAYDIR.as_slice_mut());

    save.MARGIN = 0.0;

    spicelib::ZZRYTELT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.DSKDSC.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect not to find an intercept.
    //
    save.XNXPTS = 0;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", save.XNXPTS, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Rectangular coordinate hit case. Positive MARGIN.", ctx)?;

    //
    // Use the descriptor from the previous case.
    //
    //
    // Shift the vertex so the ray just misses the volume element on
    // the +X side, but hits the extended element.
    //
    save.DELTA = 0.000000000001;

    spicelib::VPACK(
        ((save.L / 2 as f64) + save.DELTA),
        0.0,
        ((2 as f64) * save.H),
        save.VERTEX.as_slice_mut(),
    );

    spicelib::VPACK(0.0, 0.0, -1.0, save.RAYDIR.as_slice_mut());

    save.MARGIN = ((2 as f64) * save.DELTA);

    spicelib::ZZRYTELT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.DSKDSC.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an intercept.
    //
    save.XNXPTS = 1;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", save.XNXPTS, 0, OK, ctx)?;

    //
    // Create the expected intercept point.
    //
    spicelib::VPACK(
        save.VERTEX[1],
        0.0,
        ((save.H / 2 as f64) + (save.H * save.MARGIN)),
        save.XXPT.as_slice_mut(),
    );

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"XPT",
        save.XPT.as_slice(),
        b"~~/",
        save.XXPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Rectangular coordinate miss case. Positive MARGIN.", ctx)?;

    //
    // Use the descriptor from the previous case.
    //
    //
    // Shift the vertex so the ray just misses the volume element on
    // the +X side, but hits the extended element.
    //
    save.DELTA = 0.000000000001;

    spicelib::VPACK(
        ((save.L / 2 as f64) + (save.DELTA * save.L)),
        0.0,
        ((2 as f64) * save.H),
        save.VERTEX.as_slice_mut(),
    );

    spicelib::VPACK(0.0, 0.0, -1.0, save.RAYDIR.as_slice_mut());

    save.MARGIN = (save.DELTA / 2 as f64);

    spicelib::ZZRYTELT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.DSKDSC.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We don't expect an intercept.
    //
    save.XNXPTS = 0;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", save.XNXPTS, 0, OK, ctx)?;

    //***********************************************************************
    //
    //
    //     Latitudinal coordinates
    //
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Latitudinal coordinate hit case. Zero MARGIN.", ctx)?;

    //
    // Set the coordinate system and bounds in the DSK descriptor.
    //
    spicelib::CLEARD(DSKDSZ, save.DSKDSC.as_slice_mut());

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[1, 3]] = 1000.0;
    save.BOUNDS[[2, 3]] = 2000.0;

    spicelib::MOVED(save.BOUNDS.as_slice(), 6, save.DSKDSC.subarray_mut(MN1IDX));

    save.DSKDSC[SYSIDX] = LATSYS as f64;

    spicelib::LATREC(
        1000000.0,
        -(spicelib::PI(ctx) / 4 as f64),
        (spicelib::PI(ctx) / 4 as f64),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    save.MARGIN = 0.0;

    spicelib::ZZRYTELT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.DSKDSC.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an intercept.
    //
    save.XNXPTS = 1;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", save.XNXPTS, 0, OK, ctx)?;

    //
    // Create the expected intercept point.
    //
    spicelib::LATREC(
        save.BOUNDS[[2, 3]],
        -(spicelib::PI(ctx) / 4 as f64),
        (spicelib::PI(ctx) / 4 as f64),
        save.XXPT.as_slice_mut(),
    );

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"XPT",
        save.XPT.as_slice(),
        b"~~/",
        save.XXPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Latitudinal coordinate miss case. Zero MARGIN.", ctx)?;

    //
    // Set the coordinate system and bounds in the DSK descriptor.
    //
    spicelib::CLEARD(DSKDSZ, save.DSKDSC.as_slice_mut());

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[1, 3]] = 1000.0;
    save.BOUNDS[[2, 3]] = 2000.0;

    spicelib::MOVED(save.BOUNDS.as_slice(), 6, save.DSKDSC.subarray_mut(MN1IDX));

    save.DSKDSC[SYSIDX] = LATSYS as f64;

    spicelib::LATREC(
        1000000.0,
        0.0,
        (spicelib::PI(ctx) / 4 as f64),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    save.MARGIN = 0.0;

    spicelib::ZZRYTELT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.DSKDSC.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We don't expect an intercept.
    //
    save.XNXPTS = 0;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", save.XNXPTS, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Latitudinal coordinate hit case. Positive MARGIN.", ctx)?;

    save.MARGIN = 0.000000000001;

    //
    // Set the coordinate system and bounds in the DSK descriptor.
    //
    spicelib::CLEARD(DSKDSZ, save.DSKDSC.as_slice_mut());

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[1, 3]] = 1000.0;
    save.BOUNDS[[2, 3]] = 2000.0;

    spicelib::MOVED(save.BOUNDS.as_slice(), 6, save.DSKDSC.subarray_mut(MN1IDX));

    save.DSKDSC[SYSIDX] = LATSYS as f64;

    save.DELTA = (save.MARGIN / 2 as f64);

    spicelib::LATREC(
        1000000.0,
        -(spicelib::PI(ctx) / 4 as f64),
        ((spicelib::PI(ctx) / 4 as f64) + save.DELTA),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    save.MARGIN = 0.0;

    spicelib::ZZRYTELT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.DSKDSC.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an intercept.
    //
    save.XNXPTS = 1;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", save.XNXPTS, 0, OK, ctx)?;

    //
    // Create the expected intercept point.
    //
    spicelib::LATREC(
        save.BOUNDS[[2, 3]],
        -(spicelib::PI(ctx) / 4 as f64),
        (spicelib::PI(ctx) / 4 as f64),
        save.XXPT.as_slice_mut(),
    );

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"XPT",
        save.XPT.as_slice(),
        b"~~/",
        save.XXPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Latitudinal coordinate miss case. Positive MARGIN.", ctx)?;

    save.MARGIN = 0.000000000001;

    //
    // Set the coordinate system and bounds in the DSK descriptor.
    //
    spicelib::CLEARD(DSKDSZ, save.DSKDSC.as_slice_mut());

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[1, 3]] = 1000.0;
    save.BOUNDS[[2, 3]] = 2000.0;

    spicelib::MOVED(save.BOUNDS.as_slice(), 6, save.DSKDSC.subarray_mut(MN1IDX));

    save.DSKDSC[SYSIDX] = LATSYS as f64;

    save.DELTA = (save.MARGIN * 2 as f64);

    spicelib::LATREC(
        1000000.0,
        -(spicelib::PI(ctx) / 4 as f64),
        (save.BOUNDS[[2, 2]] + save.DELTA),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    save.MARGIN = 0.0;

    spicelib::ZZRYTELT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.DSKDSC.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We don't expect an intercept.
    //
    save.XNXPTS = 0;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", save.XNXPTS, 0, OK, ctx)?;

    //***********************************************************************
    //
    //
    //     Planetodetic coordinates
    //
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Planetodetic coordinate hit case. Zero MARGIN.", ctx)?;

    //
    // Set the coordinate system and bounds in the DSK descriptor.
    //
    spicelib::CLEARD(DSKDSZ, save.DSKDSC.as_slice_mut());
    spicelib::CLEARD(NSYPAR, save.CORPAR.as_slice_mut());

    save.RE = 2000.0;
    save.F = 0.2;

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[1, 3]] = -10.0;
    save.BOUNDS[[2, 3]] = 10.0;

    spicelib::MOVED(save.BOUNDS.as_slice(), 6, save.DSKDSC.subarray_mut(MN1IDX));

    save.DSKDSC[SYSIDX] = PDTSYS as f64;
    save.DSKDSC[PARIDX] = save.RE;
    save.DSKDSC[(PARIDX + 1)] = save.F;

    save.LON = -(spicelib::PI(ctx) / 4 as f64);
    save.LAT = (spicelib::PI(ctx) / 4 as f64);
    save.ALT = 10000.0;

    spicelib::GEOREC(
        save.LON,
        save.LAT,
        save.ALT,
        save.RE,
        save.F,
        save.VERTEX.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LATREC(
        1.0,
        (save.LON + spicelib::PI(ctx)),
        -save.LAT,
        save.RAYDIR.as_slice_mut(),
    );

    save.MARGIN = 0.0;

    spicelib::ZZRYTELT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.DSKDSC.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an intercept.
    //
    save.XNXPTS = 1;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", save.XNXPTS, 0, OK, ctx)?;

    //
    // Create the expected intercept point. We must first get
    // the radii of the outer bounding ellipsoid.
    //
    save.A = save.RE;
    save.B = (save.RE * (1.0 - save.F));

    spicelib::ZZELLBDS(
        save.A,
        save.B,
        save.BOUNDS[[2, 3]],
        save.BOUNDS[[1, 3]],
        &mut save.AMAX,
        &mut save.BMAX,
        &mut save.AMIN,
        &mut save.BMIN,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SURFPT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.AMAX,
        save.AMAX,
        save.BMAX,
        save.XXPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"SURFPT FOUND", save.FOUND, true, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"XPT",
        save.XPT.as_slice(),
        b"~~/",
        save.XXPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Planetodetic coordinate miss case. Zero MARGIN.", ctx)?;

    save.LON = -(spicelib::PI(ctx) / 4 as f64);
    save.LAT = 0.0;
    save.ALT = 1000000.0;

    spicelib::GEOREC(
        save.LON,
        save.LAT,
        save.ALT,
        save.RE,
        save.F,
        save.VERTEX.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LATREC(
        1.0,
        (save.LON + spicelib::PI(ctx)),
        -save.LAT,
        save.RAYDIR.as_slice_mut(),
    );

    save.MARGIN = 0.0;

    spicelib::ZZRYTELT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.DSKDSC.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We don't expect an intercept.
    //
    save.XNXPTS = 0;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", save.XNXPTS, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Planetodetic coordinate hit case. Positive MARGIN.", ctx)?;

    //
    // Set the coordinate system and bounds in the DSK descriptor.
    //
    spicelib::CLEARD(DSKDSZ, save.DSKDSC.as_slice_mut());
    spicelib::CLEARD(NSYPAR, save.CORPAR.as_slice_mut());

    save.RE = 2000.0;
    save.F = 0.2;

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[1, 3]] = -10.0;
    save.BOUNDS[[2, 3]] = 10.0;

    spicelib::MOVED(save.BOUNDS.as_slice(), 6, save.DSKDSC.subarray_mut(MN1IDX));

    save.DSKDSC[SYSIDX] = PDTSYS as f64;
    save.DSKDSC[PARIDX] = save.RE;
    save.DSKDSC[(PARIDX + 1)] = save.F;

    save.MARGIN = 0.000000000001;

    save.DELTA = (save.MARGIN / 2 as f64);

    save.LON = -(spicelib::PI(ctx) / 4 as f64);
    save.LAT = ((spicelib::PI(ctx) / 3 as f64) + save.DELTA);
    save.ALT = 1000000.0;

    spicelib::GEOREC(
        save.LON,
        save.LAT,
        save.ALT,
        save.RE,
        save.F,
        save.VERTEX.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LATREC(
        1.0,
        (save.LON + spicelib::PI(ctx)),
        -save.LAT,
        save.RAYDIR.as_slice_mut(),
    );

    save.MARGIN = 0.0;

    spicelib::ZZRYTELT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.DSKDSC.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an intercept.
    //
    save.XNXPTS = 1;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", save.XNXPTS, 0, OK, ctx)?;

    //
    // Create the expected intercept point. We must first get
    // the radii of the outer bounding ellipsoid.
    //
    save.A = save.RE;
    save.B = (save.RE * (1.0 - save.F));

    spicelib::ZZELLBDS(
        save.A,
        save.B,
        save.BOUNDS[[2, 3]],
        save.BOUNDS[[1, 3]],
        &mut save.AMAX,
        &mut save.BMAX,
        &mut save.AMIN,
        &mut save.BMIN,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SURFPT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.AMAX,
        save.AMAX,
        save.BMAX,
        save.XXPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"SURFPT FOUND", save.FOUND, true, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"XPT",
        save.XPT.as_slice(),
        b"~~/",
        save.XXPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Planetodetic coordinate miss case. Positive MARGIN.", ctx)?;

    //
    // Set the coordinate system and bounds in the DSK descriptor.
    //
    spicelib::CLEARD(DSKDSZ, save.DSKDSC.as_slice_mut());
    spicelib::CLEARD(NSYPAR, save.CORPAR.as_slice_mut());

    save.RE = 2000.0;
    save.F = 0.2;

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[1, 3]] = -10.0;
    save.BOUNDS[[2, 3]] = 10.0;

    spicelib::MOVED(save.BOUNDS.as_slice(), 6, save.DSKDSC.subarray_mut(MN1IDX));

    save.DSKDSC[SYSIDX] = PDTSYS as f64;
    save.DSKDSC[PARIDX] = save.RE;
    save.DSKDSC[(PARIDX + 1)] = save.F;

    save.MARGIN = 0.000000000001;

    save.DELTA = (save.MARGIN * 2 as f64);

    save.LON = -(spicelib::PI(ctx) / 4 as f64);
    save.LAT = ((spicelib::PI(ctx) / 3 as f64) + save.DELTA);
    save.ALT = 1000000.0;

    spicelib::GEOREC(
        save.LON,
        save.LAT,
        save.ALT,
        save.RE,
        save.F,
        save.VERTEX.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LATREC(
        1.0,
        (save.LON + spicelib::PI(ctx)),
        -save.LAT,
        save.RAYDIR.as_slice_mut(),
    );

    save.MARGIN = 0.0;

    spicelib::ZZRYTELT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.DSKDSC.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We don't expect an intercept.
    //
    save.XNXPTS = 0;

    //***********************************************************************
    //
    //     Normal cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad coordinate system.", ctx)?;

    save.DSKDSC[SYSIDX] = -1 as f64;

    spicelib::ZZRYTELT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.DSKDSC.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADCOORDSYS)", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
