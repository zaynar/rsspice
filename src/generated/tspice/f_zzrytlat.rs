//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

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
const TIGHT: f64 = 0.00000000001;
const POLMRG: f64 = 0.00000001;

struct SaveVars {
    BOUNDS: StackArray2D<f64, 6>,
    LAT: f64,
    LON: f64,
    MAG: f64,
    MARGIN: f64,
    MIDLAT: f64,
    MIDLON: f64,
    MIDR: f64,
    R: f64,
    RAYDIR: StackArray<f64, 3>,
    TOL: f64,
    VERTEX: StackArray<f64, 3>,
    X: StackArray<f64, 3>,
    XPT: StackArray<f64, 3>,
    XXPT: StackArray<f64, 3>,
    Y: StackArray<f64, 3>,
    Z: StackArray<f64, 3>,
    NXPTS: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BOUNDS = StackArray2D::<f64, 6>::new(1..=2, 1..=3);
        let mut LAT: f64 = 0.0;
        let mut LON: f64 = 0.0;
        let mut MAG: f64 = 0.0;
        let mut MARGIN: f64 = 0.0;
        let mut MIDLAT: f64 = 0.0;
        let mut MIDLON: f64 = 0.0;
        let mut MIDR: f64 = 0.0;
        let mut R: f64 = 0.0;
        let mut RAYDIR = StackArray::<f64, 3>::new(1..=3);
        let mut TOL: f64 = 0.0;
        let mut VERTEX = StackArray::<f64, 3>::new(1..=3);
        let mut X = StackArray::<f64, 3>::new(1..=3);
        let mut XPT = StackArray::<f64, 3>::new(1..=3);
        let mut XXPT = StackArray::<f64, 3>::new(1..=3);
        let mut Y = StackArray::<f64, 3>::new(1..=3);
        let mut Z = StackArray::<f64, 3>::new(1..=3);
        let mut NXPTS: i32 = 0;

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(1.0), Val::D(0.0), Val::D(0.0)].into_iter();
            X.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(1.0), Val::D(0.0)].into_iter();
            Y.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            Z.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            BOUNDS,
            LAT,
            LON,
            MAG,
            MARGIN,
            MIDLAT,
            MIDLON,
            MIDR,
            R,
            RAYDIR,
            TOL,
            VERTEX,
            X,
            XPT,
            XXPT,
            Y,
            Z,
            NXPTS,
        }
    }
}

//$Procedure F_ZZRYTLAT ( ZZRYTLAT tests )
pub fn F_ZZRYTLAT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //

    //
    // This value must be kept in sync with the parameter
    // LATMRG in ZZINLAT.
    //

    //
    // Local Variables
    //

    // CHARACTER*(TITLSZ)    TITLE

    // INTEGER               I

    //
    // Saved values
    //

    //
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_ZZRYTLAT", ctx)?;

    //***********************************************************************
    //
    //     Normal cases
    //
    //***********************************************************************

    //
    // The following tests use a volume element with a longitude extent
    // of less than 90 degrees and a latitude extent of 15 degrees. The
    // depth of the element is 50 km. The element is located above the
    // equator.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Hit on north polar cap. Margin = 0.", ctx)?;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = ((2 as f64) * spicelib::PI(ctx));

    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 2 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    spicelib::VPACK(0.0, 0.0, save.BOUNDS[[2, 3]], save.XXPT.as_slice_mut());

    save.MARGIN = 0.0;

    save.MAG = 10000000000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VMINUS(save.Z.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Hit on south polar cap. Margin = 0.", ctx)?;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = ((2 as f64) * spicelib::PI(ctx));

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 2 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    spicelib::VPACK(0.0, 0.0, -save.BOUNDS[[2, 3]], save.XXPT.as_slice_mut());

    save.MARGIN = 0.0;

    save.MAG = 10000000000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VEQU(save.Z.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(
        b"Small element in northern hemisphere. Vertex is inside element. Margin = 0.",
        ctx,
    )?;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = ((2 as f64) * spicelib::PI(ctx));

    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[2, 3]];

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);
    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);
    save.MIDR = ((save.BOUNDS[[1, 3]] + save.BOUNDS[[2, 3]]) / 2 as f64);

    spicelib::LATREC(
        save.MIDR,
        save.MIDLON,
        save.MIDLAT,
        save.XXPT.as_slice_mut(),
    );

    save.MARGIN = 0.0;

    spicelib::VEQU(save.XXPT.as_slice(), save.VERTEX.as_slice_mut());

    spicelib::CLEARD(3, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in northern hemisphere. Hit on outer sphere. Margin = 0. Longitude wrap-around case.", ctx)?;

    save.BOUNDS[[1, 1]] = (((5 as f64) * spicelib::PI(ctx)) / 6 as f64);
    save.BOUNDS[[2, 1]] = -(((5 as f64) * spicelib::PI(ctx)) / 6 as f64);

    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[2, 3]];

    save.MIDLON = (((((2 as f64) * spicelib::PI(ctx)) + save.BOUNDS[[1, 1]])
        + save.BOUNDS[[2, 1]])
        / 2 as f64);
    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);

    spicelib::LATREC(save.R, save.MIDLON, save.MIDLAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VSCL(save.MAG, save.XXPT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    save.MARGIN = 0.0;

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in northern hemisphere. Hit on inner sphere. Vertex is outside outer sphere. Margin = 0.", ctx)?;

    save.BOUNDS[[1, 1]] = (spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 2 as f64);

    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[1, 3]];

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);
    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);

    spicelib::LATREC(save.R, save.MIDLON, save.MIDLAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VSCL(-save.MAG, save.XXPT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    save.MARGIN = 0.0;

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in northern hemisphere. Hit on inner sphere. Vertex is inside inner sphere. Margin = 0.", ctx)?;

    save.BOUNDS[[1, 1]] = (spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 2 as f64);

    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[1, 3]];

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);
    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);

    spicelib::LATREC(save.R, save.MIDLON, save.MIDLAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::CLEARD(3, save.VERTEX.as_slice_mut());
    spicelib::VEQU(save.XXPT.as_slice(), save.RAYDIR.as_slice_mut());

    save.MARGIN = 0.0;

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in northern hemisphere. Hit on inner sphere. Margin = 0. Longitude wrap-around case.", ctx)?;

    save.BOUNDS[[1, 1]] = (((5 as f64) * spicelib::PI(ctx)) / 6 as f64);
    save.BOUNDS[[2, 1]] = -(((5 as f64) * spicelib::PI(ctx)) / 6 as f64);

    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[1, 3]];

    save.MIDLON = (((((2 as f64) * spicelib::PI(ctx)) + save.BOUNDS[[1, 1]])
        + save.BOUNDS[[2, 1]])
        / 2 as f64);
    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);

    spicelib::LATREC(save.R, save.MIDLON, save.MIDLAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VSCL(-save.MAG, save.XXPT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    save.MARGIN = 0.0;

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(
        b"Small element in northern hemisphere. Hit on upper latitude boundary. Margin = 0.",
        ctx,
    )?;

    //
    // This case produces a single intercept of the ray with
    // the upper latitude cone.
    //
    save.BOUNDS[[1, 1]] = (spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 2 as f64);

    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[2, 3]];

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);
    save.MIDR = ((save.BOUNDS[[1, 3]] + save.BOUNDS[[2, 3]]) / 2 as f64);

    spicelib::LATREC(
        save.MIDR,
        save.MIDLON,
        save.BOUNDS[[2, 2]],
        save.XXPT.as_slice_mut(),
    );

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VMINUS(save.Z.as_slice(), save.RAYDIR.as_slice_mut());

    save.MARGIN = 0.0;

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in northern hemisphere. Hit on upper latitude boundary. Intercept is second solution. Margin = 0.", ctx)?;

    //
    // This case produces a single intercept of the ray with
    // the upper latitude cone.
    //
    save.BOUNDS[[1, 1]] = (spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 2 as f64);

    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[2, 3]];

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);
    save.MIDR = ((save.BOUNDS[[1, 3]] + save.BOUNDS[[2, 3]]) / 2 as f64);

    spicelib::LATREC(
        save.MIDR,
        save.MIDLON,
        save.BOUNDS[[2, 2]],
        save.XXPT.as_slice_mut(),
    );

    //
    // The magnitude of the vertex offset must be large
    // enough to place the vertex outside of the upper
    // latitude cone.
    //
    save.MAG = 10000.0;

    //
    // Pick the ray's direction vector so that we can make the
    // ray hit the upper latitude cone once at pi radians away
    // from XXPT, then hit again at XXPT.
    //
    spicelib::LATREC(
        save.MAG,
        save.MIDLON,
        -(spicelib::PI(ctx) / 5 as f64),
        save.RAYDIR.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.RAYDIR.as_slice(),
        save.VERTEX.as_slice_mut(),
    );

    save.MARGIN = 0.0;

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(
        b"Small element in northern hemisphere. Hit on lower latitude boundary. Margin = 0.",
        ctx,
    )?;

    save.BOUNDS[[1, 1]] = (spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 2 as f64);

    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[2, 3]];

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);
    save.MIDR = ((save.BOUNDS[[1, 3]] + save.BOUNDS[[2, 3]]) / 2 as f64);

    spicelib::LATREC(
        save.MIDR,
        save.MIDLON,
        save.BOUNDS[[1, 2]],
        save.XXPT.as_slice_mut(),
    );

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VEQU(save.Z.as_slice(), save.RAYDIR.as_slice_mut());

    save.MARGIN = 0.0;

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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

    //
    // Note: there is no corresponding "second solution" case
    // for the lower latitude boundary of an element in the
    // northern hemisphere.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Small element in northern hemisphere. Hit on west longitude boundary. Margin = 0.",
        ctx,
    )?;

    save.BOUNDS[[1, 1]] = (spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 2 as f64);

    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[2, 3]];

    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);
    save.MIDR = ((save.BOUNDS[[1, 3]] + save.BOUNDS[[2, 3]]) / 2 as f64);

    spicelib::LATREC(
        save.MIDR,
        save.BOUNDS[[1, 1]],
        save.MIDLAT,
        save.XXPT.as_slice_mut(),
    );

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.Y.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VEQU(save.Y.as_slice(), save.RAYDIR.as_slice_mut());

    save.MARGIN = 0.0;

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(
        b"Small element in northern hemisphere. Hit on east longitude boundary. Margin = 0.",
        ctx,
    )?;

    save.BOUNDS[[1, 1]] = (spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 2 as f64);

    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[2, 3]];

    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);
    save.MIDR = ((save.BOUNDS[[1, 3]] + save.BOUNDS[[2, 3]]) / 2 as f64);

    spicelib::LATREC(
        save.MIDR,
        save.BOUNDS[[2, 1]],
        save.MIDLAT,
        save.XXPT.as_slice_mut(),
    );

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.X.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VEQU(save.X.as_slice(), save.RAYDIR.as_slice_mut());

    save.MARGIN = 0.0;

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    // The following tests use a volume element with a longitude extent
    // of less than degrees and a latitude extent of 15 degrees. The
    // depth of the element is 50 km. The element is located below the
    // equator.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Small element in southern hemisphere. Vertex is inside element. Margin = 0.",
        ctx,
    )?;

    save.BOUNDS[[1, 1]] = (spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 2 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[2, 3]];

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);
    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);
    save.MIDR = ((save.BOUNDS[[1, 3]] + save.BOUNDS[[2, 3]]) / 2 as f64);

    spicelib::LATREC(
        save.MIDR,
        save.MIDLON,
        save.MIDLAT,
        save.XXPT.as_slice_mut(),
    );

    save.MARGIN = 0.0;

    spicelib::VEQU(save.XXPT.as_slice(), save.VERTEX.as_slice_mut());

    spicelib::CLEARD(3, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in southern hemisphere. Hit on outer sphere. Margin = 0. Longitude wrap-around case.", ctx)?;

    save.BOUNDS[[1, 1]] = (((5 as f64) * spicelib::PI(ctx)) / 6 as f64);
    save.BOUNDS[[2, 1]] = -(((5 as f64) * spicelib::PI(ctx)) / 6 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[2, 3]];

    save.MIDLON = (((((2 as f64) * spicelib::PI(ctx)) + save.BOUNDS[[1, 1]])
        + save.BOUNDS[[2, 1]])
        / 2 as f64);
    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);

    spicelib::LATREC(save.R, save.MIDLON, save.MIDLAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VSCL(save.MAG, save.XXPT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    save.MARGIN = 0.0;

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in southern hemisphere. Hit on inner sphere. Vertex is outside outer sphere. Margin = 0.", ctx)?;

    save.BOUNDS[[1, 1]] = (spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 2 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[1, 3]];

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);
    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);

    spicelib::LATREC(save.R, save.MIDLON, save.MIDLAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VSCL(-save.MAG, save.XXPT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    save.MARGIN = 0.0;

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in southern hemisphere. Hit on inner sphere. Vertex is inside inner sphere. Margin = 0.", ctx)?;

    save.BOUNDS[[1, 1]] = (spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 2 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[1, 3]];

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);
    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);

    spicelib::LATREC(save.R, save.MIDLON, save.MIDLAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::CLEARD(3, save.VERTEX.as_slice_mut());
    spicelib::VEQU(save.XXPT.as_slice(), save.RAYDIR.as_slice_mut());

    save.MARGIN = 0.0;

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in southern hemisphere. Hit on inner sphere. Margin = 0. Longitude wrap-around case.", ctx)?;

    save.BOUNDS[[1, 1]] = (((5 as f64) * spicelib::PI(ctx)) / 6 as f64);
    save.BOUNDS[[2, 1]] = -(((5 as f64) * spicelib::PI(ctx)) / 6 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[1, 3]];

    save.MIDLON = (((((2 as f64) * spicelib::PI(ctx)) + save.BOUNDS[[1, 1]])
        + save.BOUNDS[[2, 1]])
        / 2 as f64);
    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);

    spicelib::LATREC(save.R, save.MIDLON, save.MIDLAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VSCL(-save.MAG, save.XXPT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    save.MARGIN = 0.0;

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(
        b"Small element in southern hemisphere. Hit on upper latitude boundary. Margin = 0.",
        ctx,
    )?;

    save.BOUNDS[[1, 1]] = (spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 2 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[2, 3]];

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);
    save.MIDR = ((save.BOUNDS[[1, 3]] + save.BOUNDS[[2, 3]]) / 2 as f64);

    spicelib::LATREC(
        save.MIDR,
        save.MIDLON,
        save.BOUNDS[[2, 2]],
        save.XXPT.as_slice_mut(),
    );

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VMINUS(save.Z.as_slice(), save.RAYDIR.as_slice_mut());

    save.MARGIN = 0.0;

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(
        b"Small element in southern hemisphere. Hit on lower latitude boundary. Margin = 0.",
        ctx,
    )?;

    save.BOUNDS[[1, 1]] = (spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 2 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[2, 3]];

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);
    save.MIDR = ((save.BOUNDS[[1, 3]] + save.BOUNDS[[2, 3]]) / 2 as f64);

    spicelib::LATREC(
        save.MIDR,
        save.MIDLON,
        save.BOUNDS[[1, 2]],
        save.XXPT.as_slice_mut(),
    );

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VEQU(save.Z.as_slice(), save.RAYDIR.as_slice_mut());

    save.MARGIN = 0.0;

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in southern hemisphere. Hit on lower latitude boundary. Intercept is second solution. Margin = 0.", ctx)?;

    //
    // This case produces a single intercept of the ray with
    // the upper latitude cone.
    //
    save.BOUNDS[[1, 1]] = (spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 2 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[2, 3]];

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);
    save.MIDR = ((save.BOUNDS[[1, 3]] + save.BOUNDS[[2, 3]]) / 2 as f64);

    spicelib::LATREC(
        save.MIDR,
        save.MIDLON,
        save.BOUNDS[[1, 2]],
        save.XXPT.as_slice_mut(),
    );

    //
    // The magnitude of the vertex offset must be large
    // enough to place the vertex outside of the lower
    // latitude cone.
    //
    save.MAG = 10000.0;

    //
    // Pick the ray's direction vector so that we can make the
    // ray hit the lower latitude cone once at pi radians away
    // from XXPT, then hit again at XXPT.
    //
    spicelib::LATREC(
        save.MAG,
        save.MIDLON,
        -(spicelib::PI(ctx) / 5 as f64),
        save.RAYDIR.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.RAYDIR.as_slice(),
        save.VERTEX.as_slice_mut(),
    );

    save.MARGIN = 0.0;

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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

    //
    // Note: there is no corresponding "second solution" case
    // for the upper latitude boundary of an element in the
    // southern hemisphere.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Small element in southern hemisphere. Hit on west longitude boundary. Margin = 0.",
        ctx,
    )?;

    save.BOUNDS[[1, 1]] = (spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 2 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[2, 3]];

    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);
    save.MIDR = ((save.BOUNDS[[1, 3]] + save.BOUNDS[[2, 3]]) / 2 as f64);

    spicelib::LATREC(
        save.MIDR,
        save.BOUNDS[[1, 1]],
        save.MIDLAT,
        save.XXPT.as_slice_mut(),
    );

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.Y.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VEQU(save.Y.as_slice(), save.RAYDIR.as_slice_mut());

    save.MARGIN = 0.0;

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(
        b"Small element in southern hemisphere. Hit on east longitude boundary. Margin = 0.",
        ctx,
    )?;

    save.BOUNDS[[1, 1]] = (spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 2 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[2, 3]];

    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);
    save.MIDR = ((save.BOUNDS[[1, 3]] + save.BOUNDS[[2, 3]]) / 2 as f64);

    spicelib::LATREC(
        save.MIDR,
        save.BOUNDS[[2, 1]],
        save.MIDLAT,
        save.XXPT.as_slice_mut(),
    );

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.X.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VEQU(save.X.as_slice(), save.RAYDIR.as_slice_mut());

    save.MARGIN = 0.0;

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    //
    // Following are tests for some cases in which the volume
    // element has special properties.
    //
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Small element with southern boundary on the equator. Margin = 0.",
        ctx,
    )?;

    save.BOUNDS[[1, 1]] = (spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 2 as f64);

    save.BOUNDS[[1, 2]] = 0.0;
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[2, 3]];

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);
    save.MIDR = ((save.BOUNDS[[1, 3]] + save.BOUNDS[[2, 3]]) / 2 as f64);

    spicelib::LATREC(
        save.MIDR,
        save.MIDLON,
        save.BOUNDS[[1, 2]],
        save.XXPT.as_slice_mut(),
    );

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VEQU(save.Z.as_slice(), save.RAYDIR.as_slice_mut());

    save.MARGIN = 0.0;

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(
        b"Small element with northern boundary on the equator. Margin = 0.",
        ctx,
    )?;

    save.BOUNDS[[1, 1]] = (spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 2 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = 0.0;

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[2, 3]];

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);
    save.MIDR = ((save.BOUNDS[[1, 3]] + save.BOUNDS[[2, 3]]) / 2 as f64);

    spicelib::LATREC(
        save.MIDR,
        save.MIDLON,
        save.BOUNDS[[2, 2]],
        save.XXPT.as_slice_mut(),
    );

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VMINUS(save.Z.as_slice(), save.RAYDIR.as_slice_mut());

    save.MARGIN = 0.0;

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(
        b"Equatorial band; longitude extent = 2pi. Margin = 0. Vertex is on +X axis.",
        ctx,
    )?;

    save.BOUNDS[[1, 1]] = -spicelib::PI(ctx);
    save.BOUNDS[[2, 1]] = spicelib::PI(ctx);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[2, 3]];
    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);

    spicelib::LATREC(save.R, 0.0, save.MIDLAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        save.MAG,
        save.X.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VMINUS(save.X.as_slice(), save.RAYDIR.as_slice_mut());

    save.MARGIN = 0.0;

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

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
    testutil::TCASE(
        b"Equatorial band; longitude extent = 2pi. Margin = 0. Vertex is on +Y axis.",
        ctx,
    )?;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = ((2 as f64) * spicelib::PI(ctx));

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[2, 3]];
    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);

    save.XXPT[1] = (save.R - 1.0);
    save.XXPT[2] = f64::sqrt((f64::powi(save.R, 2) - f64::powi((save.R - 1.0), 2)));
    save.XXPT[3] = 0.0;

    spicelib::LATREC(save.R, 0.0, save.MIDLAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        save.MAG,
        save.Y.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VMINUS(save.Y.as_slice(), save.RAYDIR.as_slice_mut());

    save.MARGIN = 0.0;

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

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
    testutil::TCASE(
        b"Equatorial band; longitude extent = 2pi. Margin = 0. Vertex is on +Z axis.",
        ctx,
    )?;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = ((2 as f64) * spicelib::PI(ctx));

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[2, 3]];
    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);

    save.XXPT[1] = (save.R - 1.0);
    save.XXPT[2] = 0.0;
    save.XXPT[3] = f64::sqrt((f64::powi(save.R, 2) - f64::powi((save.R - 1.0), 2)));

    spicelib::LATREC(save.R, 0.0, save.MIDLAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VMINUS(save.Y.as_slice(), save.RAYDIR.as_slice_mut());

    save.MARGIN = 0.0;

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

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
    testutil::TCASE(b"Small element with wrap-around longitude. Margin = 0. Hit on west (left) longitude boundary.", ctx)?;

    save.BOUNDS[[1, 1]] = (((5 as f64) * spicelib::PI(ctx)) / 6 as f64);
    save.BOUNDS[[2, 1]] = -(((5 as f64) * spicelib::PI(ctx)) / 6 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 3 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[2, 3]];
    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);
    save.MIDR = ((save.BOUNDS[[1, 3]] + save.BOUNDS[[2, 3]]) / 2 as f64);

    spicelib::LATREC(
        save.R,
        save.BOUNDS[[2, 1]],
        save.MIDLAT,
        save.XXPT.as_slice_mut(),
    );

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.X.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VEQU(save.X.as_slice(), save.RAYDIR.as_slice_mut());

    save.MARGIN = 0.0;

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

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
    testutil::TCASE(b"Small element with wrap-around longitude. Margin = 0. Hit on east (right) longitude boundary.", ctx)?;

    save.BOUNDS[[1, 1]] = (((5 as f64) * spicelib::PI(ctx)) / 6 as f64);
    save.BOUNDS[[2, 1]] = -(((5 as f64) * spicelib::PI(ctx)) / 6 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 3 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[2, 3]];
    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);
    save.MIDR = ((save.BOUNDS[[1, 3]] + save.BOUNDS[[2, 3]]) / 2 as f64);

    spicelib::LATREC(
        save.R,
        save.BOUNDS[[2, 1]],
        save.MIDLAT,
        save.XXPT.as_slice_mut(),
    );

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.X.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VEQU(save.X.as_slice(), save.RAYDIR.as_slice_mut());

    save.MARGIN = 0.0;

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

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
    //
    // The following cases use positive margins.
    //
    //    POSITIVE MARGINS
    //
    //

    //
    //     We start with cases involving hits on the outer sphere.
    //
    //        OUTER SPHERE
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in northern hemisphere. Vertex is inside element. Margin > 0. Interior case.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = (spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 2 as f64);

    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = (save.BOUNDS[[2, 3]] + ((save.MARGIN / 2 as f64) * save.R));

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);
    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);

    spicelib::LATREC(save.R, save.MIDLON, save.MIDLAT, save.XXPT.as_slice_mut());

    spicelib::VEQU(save.XXPT.as_slice(), save.VERTEX.as_slice_mut());

    spicelib::CLEARD(3, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in northern hemisphere. Hit on outer sphere. Margin > 0. Interior case. Hit is beyond east longitude boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[2, 3]];

    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);

    save.LON = (save.BOUNDS[[2, 1]] + (save.MARGIN / 2 as f64));

    spicelib::LATREC(save.R, save.LON, save.MIDLAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VSCL(save.MAG, save.XXPT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in northern hemisphere. Hit on outer sphere. Margin > 0. Exterior case. Hit is beyond east longitude boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.LON = (save.BOUNDS[[2, 1]] + ((2 as f64) * save.MARGIN));

    spicelib::LATREC(save.R, save.LON, save.MIDLAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VSCL(save.MAG, save.XXPT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in northern hemisphere. Hit on outer sphere. Margin > 0. Longitude wrap-around case. Interior case. Hit is beyond east longitude boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = (((5 as f64) * spicelib::PI(ctx)) / 6 as f64);
    save.BOUNDS[[2, 1]] = -(((5 as f64) * spicelib::PI(ctx)) / 6 as f64);

    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[2, 3]];

    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);

    save.LON = (save.BOUNDS[[2, 1]] + (save.MARGIN / 2 as f64));

    spicelib::LATREC(save.R, save.LON, save.MIDLAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VSCL(save.MAG, save.XXPT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in northern hemisphere. Hit on outer sphere. Margin > 0. Longitude wrap-around case. Exterior case. Hit is beyond east longitude boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = (((5 as f64) * spicelib::PI(ctx)) / 6 as f64);
    save.BOUNDS[[2, 1]] = -(((5 as f64) * spicelib::PI(ctx)) / 6 as f64);

    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[2, 3]];

    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);

    save.LON = (save.BOUNDS[[2, 1]] + (save.MARGIN * 2 as f64));

    spicelib::LATREC(save.R, save.LON, save.MIDLAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VSCL(save.MAG, save.XXPT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in northern hemisphere. Hit on outer sphere. Margin > 0. Interior case. Hit is beyond west longitude boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[2, 3]];

    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);

    save.LON = (save.BOUNDS[[1, 1]] - (save.MARGIN / 2 as f64));

    spicelib::LATREC(save.R, save.LON, save.MIDLAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VSCL(save.MAG, save.XXPT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in northern hemisphere. Hit on outer sphere. Margin > 0. Longitude wrap-around case. Interior case. Hit is beyond west longitude boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = (((5 as f64) * spicelib::PI(ctx)) / 6 as f64);
    save.BOUNDS[[2, 1]] = -(((5 as f64) * spicelib::PI(ctx)) / 6 as f64);

    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[2, 3]];

    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);

    save.LON = (save.BOUNDS[[1, 1]] - (save.MARGIN / 2 as f64));

    spicelib::LATREC(save.R, save.LON, save.MIDLAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VSCL(save.MAG, save.XXPT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in northern hemisphere. Hit on outer sphere. Margin > 0. Longitude wrap-around case. Exterior case. Hit is beyond west longitude boundary.", ctx)?;

    save.LON = (save.BOUNDS[[1, 1]] - (save.MARGIN * 2 as f64));

    spicelib::LATREC(save.R, save.LON, save.MIDLAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VSCL(save.MAG, save.XXPT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in northern hemisphere. Hit on outer sphere. Margin > 0. Interior case. Hit is beyond north latitude boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[2, 3]];

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);

    save.LAT = (save.BOUNDS[[2, 2]] + (save.MARGIN / 2 as f64));

    spicelib::LATREC(save.R, save.MIDLON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VSCL(save.MAG, save.XXPT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in northern hemisphere. Hit on outer sphere. Margin > 0. Exterior case. Hit is beyond north latitude boundary.", ctx)?;

    save.LAT = (save.BOUNDS[[2, 2]] + (save.MARGIN * 2 as f64));

    spicelib::LATREC(save.R, save.MIDLON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VSCL(save.MAG, save.XXPT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in southern hemisphere. Hit on outer sphere. Margin > 0. Interior case. Hit is beyond north latitude boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[2, 3]];

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);

    save.LAT = (save.BOUNDS[[2, 2]] + (save.MARGIN / 2 as f64));

    spicelib::LATREC(save.R, save.MIDLON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VSCL(save.MAG, save.XXPT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in southern hemisphere. Hit on outer sphere. Margin > 0. Exterior case. Hit is beyond north latitude boundary.", ctx)?;

    save.LAT = (save.BOUNDS[[2, 2]] + (save.MARGIN * 2 as f64));

    spicelib::LATREC(save.R, save.MIDLON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VSCL(save.MAG, save.XXPT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in southern hemisphere. Hit on outer sphere. Margin > 0. Interior case. Hit is beyond south latitude boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[2, 3]];

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);

    save.LAT = (save.BOUNDS[[1, 2]] - (save.MARGIN / 2 as f64));

    spicelib::LATREC(save.R, save.MIDLON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VSCL(save.MAG, save.XXPT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in southern hemisphere. Hit on outer sphere. Margin > 0. Exterior case. Hit is beyond south latitude boundary.", ctx)?;

    save.LAT = (save.BOUNDS[[1, 2]] - (save.MARGIN * 2 as f64));

    spicelib::LATREC(save.R, save.MIDLON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VSCL(save.MAG, save.XXPT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // The following cases deal with hits on longitude boundaries.
    //
    //    EAST LONGITUDE
    //
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in southern hemisphere. Hit on east longitude boundary. Margin > 0. Interior case. Hit is beyond south latitude boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.MIDR = ((save.BOUNDS[[1, 3]] + save.BOUNDS[[2, 3]]) / 2 as f64);

    save.LON = save.BOUNDS[[2, 1]];

    save.LAT = (save.BOUNDS[[1, 2]] - (save.MARGIN / 2 as f64));

    spicelib::LATREC(save.MIDR, save.LON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        save.MAG,
        save.Y.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VMINUS(save.Y.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in southern hemisphere. Hit on east longitude boundary. Margin > 0. Exterior case. Hit is beyond south latitude boundary.", ctx)?;

    save.LAT = (save.BOUNDS[[1, 2]] - (save.MARGIN * 2 as f64));

    spicelib::LATREC(save.MIDR, save.LON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        save.MAG,
        save.Y.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VMINUS(save.Y.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in southern hemisphere. Hit on east longitude boundary. Margin > 0. Interior case. Hit is beyond north latitude boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    //
    // Note the upper longitude bound. This value was selected to
    // simplify the choice of a ray that would not hit other
    // bounding surfaces before hitting the expected intercept.
    //

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = 0.0;

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.MIDR = ((save.BOUNDS[[1, 3]] + save.BOUNDS[[2, 3]]) / 2 as f64);

    save.LON = save.BOUNDS[[2, 1]];

    save.LAT = (save.BOUNDS[[2, 2]] + (save.MARGIN / 2 as f64));

    spicelib::LATREC(save.MIDR, save.LON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        save.MAG,
        save.Y.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in southern hemisphere. Hit on east longitude boundary. Margin > 0. Exterior case. Hit is beyond north latitude boundary.", ctx)?;

    save.LAT = (save.BOUNDS[[2, 2]] + (save.MARGIN * 2 as f64));

    spicelib::LATREC(save.MIDR, save.LON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        save.MAG,
        save.Y.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in southern hemisphere. Hit on east longitude boundary. Margin > 0. Interior case. Hit is beyond outer radius boundary.", ctx)?;

    //
    // This is a place holder. There is no possibility of a hit
    // outside of the outer bounding sphere (radius BOUNDS(2,3)).
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in southern hemisphere. Hit on east longitude boundary. Margin > 0. Exterior case. Hit is beyond outer radius boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    //
    // Note the upper longitude bound. This value was selected to
    // simplify the choice of a ray that would not hit other
    // bounding surfaces before hitting the expected intercept.
    //

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = 0.0;

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = (save.BOUNDS[[2, 3]] + ((save.BOUNDS[[2, 3]] * save.MARGIN) / 2 as f64));

    save.LON = save.BOUNDS[[2, 1]];

    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);

    spicelib::LATREC(save.R, save.LON, save.MIDLAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        save.MAG,
        save.Y.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in southern hemisphere. Hit on east longitude boundary. Margin > 0. Interior case. Hit is beyond inner radius boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    //
    // Note the upper longitude bound. This value was selected to
    // simplify the choice of a ray that would not hit other
    // bounding surfaces before hitting the expected intercept.
    //

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = 0.0;

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.LON = save.BOUNDS[[2, 1]];

    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);

    save.R = (save.BOUNDS[[1, 3]] - ((save.BOUNDS[[1, 3]] * save.MARGIN) / 2 as f64));

    spicelib::LATREC(save.R, save.LON, save.MIDLAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        save.MAG,
        save.Y.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in southern hemisphere. Hit on east longitude boundary. Margin > 0. Exterior case. Hit is beyond inner radius boundary.", ctx)?;

    //
    // Pick a radius value that makes it easy to miss the inner
    // bounding sphere.
    //
    save.R = (save.BOUNDS[[1, 3]] / 2 as f64);

    spicelib::LATREC(save.R, save.LON, save.MIDLAT, save.XXPT.as_slice_mut());

    //
    // Choose MAG so that the vertex is inside the inner bounding
    // sphere.
    //
    save.MAG = 1.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        save.MAG,
        save.Y.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    //        WEST LONGITUDE
    //
    //     The cases below are mirror images of the east longitude
    //     cases.
    //
    //
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in southern hemisphere. Hit on west longitude boundary. Margin > 0. Interior case. Hit is beyond south latitude boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.MIDR = ((save.BOUNDS[[1, 3]] + save.BOUNDS[[2, 3]]) / 2 as f64);

    save.LON = save.BOUNDS[[1, 1]];

    save.LAT = (save.BOUNDS[[1, 2]] - (save.MARGIN / 2 as f64));

    spicelib::LATREC(save.MIDR, save.LON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.Y.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VEQU(save.Y.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in southern hemisphere. Hit on west longitude boundary. Margin > 0. Exterior case. Hit is beyond south latitude boundary.", ctx)?;

    save.LAT = (save.BOUNDS[[1, 2]] - (save.MARGIN * 2 as f64));

    spicelib::LATREC(save.MIDR, save.LON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.Y.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VEQU(save.Y.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in southern hemisphere. Hit on west longitude boundary. Margin > 0. Interior case. Hit is beyond north latitude boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    //
    // Note the upper longitude bound. This value was selected to
    // simplify the choice of a ray that would not hit other
    // bounding surfaces before hitting the expected intercept.
    //

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.MIDR = ((save.BOUNDS[[1, 3]] + save.BOUNDS[[2, 3]]) / 2 as f64);

    save.LON = save.BOUNDS[[1, 1]];

    save.LAT = (save.BOUNDS[[2, 2]] + (save.MARGIN / 2 as f64));

    spicelib::LATREC(save.MIDR, save.LON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.Y.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in southern hemisphere. Hit on west longitude boundary. Margin > 0. Exterior case. Hit is beyond north latitude boundary.", ctx)?;

    save.LAT = (save.BOUNDS[[2, 2]] + (save.MARGIN * 2 as f64));

    spicelib::LATREC(save.MIDR, save.LON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.Y.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in southern hemisphere. Hit on west longitude boundary. Margin > 0. Interior case. Hit is beyond outer radius boundary.", ctx)?;

    //
    // This is a place holder. There is no possibility of a hit
    // outside of the outer bounding sphere (radius BOUNDS(2,3)).
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in southern hemisphere. Hit on west longitude boundary. Margin > 0. Exterior case. Hit is beyond outer radius boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    //
    // Note the upper longitude bound. This value was selected to
    // simplify the choice of a ray that would not hit other
    // bounding surfaces before hitting the expected intercept.
    //

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = (save.BOUNDS[[2, 3]] + ((save.BOUNDS[[2, 3]] * save.MARGIN) / 2 as f64));

    save.LON = save.BOUNDS[[1, 1]];

    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);

    spicelib::LATREC(save.R, save.LON, save.MIDLAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.Y.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in southern hemisphere. Hit on west longitude boundary. Margin > 0. Interior case. Hit is beyond inner radius boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    //
    // Note the upper longitude bound. This value was selected to
    // simplify the choice of a ray that would not hit other
    // bounding surfaces before hitting the expected intercept.
    //

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.LON = save.BOUNDS[[1, 1]];

    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);

    save.R = (save.BOUNDS[[1, 3]] - ((save.BOUNDS[[1, 3]] * save.MARGIN) / 2 as f64));

    spicelib::LATREC(save.R, save.LON, save.MIDLAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.Y.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in southern hemisphere. Hit on west longitude boundary. Margin > 0. Exterior case. Hit is beyond inner radius boundary.", ctx)?;

    //
    // Pick a radius value that makes it easy to miss the inner
    // bounding sphere.
    //
    save.R = (save.BOUNDS[[1, 3]] / 2 as f64);

    spicelib::LATREC(save.R, save.LON, save.MIDLAT, save.XXPT.as_slice_mut());

    //
    // Choose MAG so that the vertex is inside the inner bounding
    // sphere.
    //
    save.MAG = 1.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.Y.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // The following cases deal with hits on latitude boundaries.
    //
    //    Volume element is below equator.
    //
    //    SOUTHERN HEMISPHERE
    //
    //    NORTH LATITUDE
    //
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in southern hemisphere. Hit on north latitude boundary. Margin > 0. Interior case. Hit is beyond west longitude boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.MIDR = ((save.BOUNDS[[1, 3]] + save.BOUNDS[[2, 3]]) / 2 as f64);

    save.LAT = save.BOUNDS[[2, 2]];

    save.LON = (save.BOUNDS[[1, 1]] - (save.MARGIN / 2 as f64));

    spicelib::LATREC(save.MIDR, save.LON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in southern hemisphere. Hit on north latitude boundary. Margin > 0. Exterior case. Hit is beyond west longitude boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.LON = (save.BOUNDS[[1, 1]] - ((save.MARGIN * 2 as f64) * save.BOUNDS[[2, 3]]));

    spicelib::LATREC(save.MIDR, save.LON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in southern hemisphere. Hit on north latitude boundary. Margin > 0. Interior case. Hit is beyond east longitude boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.MIDR = ((save.BOUNDS[[1, 3]] + save.BOUNDS[[2, 3]]) / 2 as f64);

    save.LAT = save.BOUNDS[[2, 2]];

    save.LON = (save.BOUNDS[[2, 1]] + (save.MARGIN / 2 as f64));

    spicelib::LATREC(save.MIDR, save.LON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in southern hemisphere. Hit on north latitude boundary. Margin > 0. Exterior case. Hit is beyond east longitude boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.LON = (save.BOUNDS[[2, 1]] + ((save.MARGIN * 2 as f64) * save.BOUNDS[[2, 3]]));

    spicelib::LATREC(save.MIDR, save.LON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in southern hemisphere. Hit on north latitude boundary. Margin > 0. Interior case. Hit is beyond outer radius boundary.", ctx)?;

    //
    // This is a place holder. There's no such hit case.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in southern hemisphere. Hit on north latitude boundary. Margin > 0. Exterior case. Hit is beyond outer radius boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    //
    // Note the upper longitude bound. This value was selected to
    // simplify the choice of a ray that would not hit other
    // bounding surfaces before hitting the expected intercept.
    //

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = (save.BOUNDS[[2, 3]] + ((save.BOUNDS[[2, 3]] * save.MARGIN) * 2 as f64));

    save.LAT = save.BOUNDS[[2, 2]];

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);

    spicelib::LATREC(save.R, save.MIDLON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in southern hemisphere. Hit on north latitude boundary. Margin > 0. Interior case. Hit is beyond inner radius boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = (save.BOUNDS[[1, 3]] - ((save.BOUNDS[[1, 3]] * save.MARGIN) / 2 as f64));

    save.LAT = save.BOUNDS[[2, 2]];

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);

    spicelib::LATREC(save.R, save.MIDLON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.X.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in southern hemisphere. Hit on north latitude boundary. Margin > 0. Exterior case. Hit is beyond inner radius boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    //
    // Choose a radius that precludes a valid hit on the inner
    // sphere.
    //
    save.R = (save.BOUNDS[[1, 3]] / 2 as f64);

    save.LAT = save.BOUNDS[[2, 2]];

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);

    spicelib::LATREC(save.R, save.MIDLON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // SOUTHERN HEMISPHERE
    //
    // SOUTH LATITUDE
    //
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in southern hemisphere. Hit on south latitude boundary. Margin > 0. Interior case. Hit is beyond west longitude boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.MIDR = ((save.BOUNDS[[1, 3]] + save.BOUNDS[[2, 3]]) / 2 as f64);

    save.LAT = save.BOUNDS[[1, 2]];

    save.LON = (save.BOUNDS[[1, 1]] - (save.MARGIN / 2 as f64));

    spicelib::LATREC(save.MIDR, save.LON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in southern hemisphere. Hit on south latitude boundary. Margin > 0. Exterior case. Hit is beyond west longitude boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.LON = (save.BOUNDS[[1, 1]] - ((save.MARGIN * 2 as f64) * save.BOUNDS[[2, 3]]));

    spicelib::LATREC(save.MIDR, save.LON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in southern hemisphere. Hit on south latitude boundary. Margin > 0. Interior case. Hit is beyond east longitude boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.MIDR = ((save.BOUNDS[[1, 3]] + save.BOUNDS[[2, 3]]) / 2 as f64);

    save.LAT = save.BOUNDS[[1, 2]];

    save.LON = (save.BOUNDS[[2, 1]] + (save.MARGIN / 2 as f64));

    spicelib::LATREC(save.MIDR, save.LON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in southern hemisphere. Hit on south latitude boundary. Margin > 0. Exterior case. Hit is beyond east longitude boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.LON = (save.BOUNDS[[2, 1]] + ((save.MARGIN * 2 as f64) * save.BOUNDS[[2, 3]]));

    spicelib::LATREC(save.MIDR, save.LON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in southern hemisphere. Hit on south latitude boundary. Margin > 0. Interior case. Hit is beyond outer radius boundary.", ctx)?;

    //
    // This is a place holder. There's no such hit case.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in southern hemisphere. Hit on south latitude boundary. Margin > 0. Exterior case. Hit is beyond outer radius boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = (save.BOUNDS[[2, 3]] + ((save.BOUNDS[[2, 3]] * save.MARGIN) * 2 as f64));

    save.LAT = save.BOUNDS[[1, 2]];

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);

    spicelib::LATREC(save.R, save.MIDLON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    //
    // Choose the ray direction so that the ray skims past the
    // volume element.
    //
    spicelib::VPACK(1.0, 0.0, 1.0, save.RAYDIR.as_slice_mut());

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.RAYDIR.as_slice(),
        save.VERTEX.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in southern hemisphere. Hit on south latitude boundary. Margin > 0. Interior case. Hit is beyond inner radius boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = (save.BOUNDS[[1, 3]] - ((save.BOUNDS[[1, 3]] * save.MARGIN) / 2 as f64));

    save.LAT = save.BOUNDS[[1, 2]];

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);

    spicelib::LATREC(save.R, save.MIDLON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.X.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in southern hemisphere. Hit on south latitude boundary. Margin > 0. Exterior case. Hit is beyond inner radius boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = (save.BOUNDS[[1, 3]] / 2 as f64);

    save.LAT = save.BOUNDS[[2, 2]];

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);

    spicelib::LATREC(save.R, save.MIDLON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    //
    // Volume element is above equator.
    //
    // NORTHERN HEMISPHERE
    //
    // NORTH LATITUDE
    //
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in northern hemisphere. Hit on north latitude boundary. Margin > 0. Interior case. Hit is beyond west longitude boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.MIDR = ((save.BOUNDS[[1, 3]] + save.BOUNDS[[2, 3]]) / 2 as f64);

    save.LAT = save.BOUNDS[[2, 2]];

    save.LON = (save.BOUNDS[[1, 1]] - (save.MARGIN / 2 as f64));

    spicelib::LATREC(save.MIDR, save.LON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in northern hemisphere. Hit on north latitude boundary. Margin > 0. Exterior case. Hit is beyond west longitude boundary.", ctx)?;

    save.LON = (save.BOUNDS[[1, 1]] - (save.MARGIN * 2 as f64));

    spicelib::LATREC(save.MIDR, save.LON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in northern hemisphere. Hit on north latitude boundary. Margin > 0. Interior case. Hit is beyond east longitude boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.MIDR = ((save.BOUNDS[[1, 3]] + save.BOUNDS[[2, 3]]) / 2 as f64);

    save.LAT = save.BOUNDS[[2, 2]];

    save.LON = (save.BOUNDS[[2, 1]] + (save.MARGIN / 2 as f64));

    spicelib::LATREC(save.MIDR, save.LON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in northern hemisphere. Hit on north latitude boundary. Margin > 0. Exterior case. Hit is beyond east longitude boundary.", ctx)?;

    save.LON = (save.BOUNDS[[2, 1]] + (save.MARGIN * 2 as f64));

    spicelib::LATREC(save.MIDR, save.LON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in northern hemisphere. Hit on north latitude boundary. Margin > 0. Interior case. Hit is beyond outer radius boundary.", ctx)?;

    //
    // This is a place holder. There's no such hit case.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in northern hemisphere. Hit on north latitude boundary. Margin > 0. Exterior case. Hit is beyond outer radius boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.LAT = save.BOUNDS[[2, 2]];

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);

    save.R = (save.BOUNDS[[2, 3]] + ((save.BOUNDS[[2, 3]] * save.MARGIN) / 2 as f64));

    spicelib::LATREC(save.R, save.MIDLON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    //
    // Choose the ray direction so that the ray skims past the
    // volume element.
    //
    spicelib::VPACK(1.0, 0.0, -1.0, save.RAYDIR.as_slice_mut());

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.RAYDIR.as_slice(),
        save.VERTEX.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in northern hemisphere. Hit on north latitude boundary. Margin > 0. Interior case. Hit is beyond inner radius boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = (save.BOUNDS[[1, 3]] - ((save.BOUNDS[[1, 3]] * save.MARGIN) / 2 as f64));

    save.LAT = save.BOUNDS[[2, 2]];

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);

    spicelib::LATREC(save.R, save.MIDLON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.X.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in northern hemisphere. Hit on north latitude boundary. Margin > 0. Exterior case. Hit is beyond inner radius boundary.", ctx)?;

    //
    // Choose R to ensure a miss.
    //
    save.R = (save.BOUNDS[[1, 3]] / 2 as f64);

    save.LAT = save.BOUNDS[[2, 2]];

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);

    spicelib::LATREC(save.R, save.MIDLON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    //
    // Volume element is above equator.
    //
    // NORTHERN HEMISPHERE
    //
    // SOUTH LATITUDE
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in northern hemisphere. Hit on south latitude boundary. Margin > 0. Interior case. Hit is beyond west longitude boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.MIDR = ((save.BOUNDS[[1, 3]] + save.BOUNDS[[2, 3]]) / 2 as f64);

    save.LAT = save.BOUNDS[[1, 2]];

    save.LON = (save.BOUNDS[[1, 1]] - (save.MARGIN / 2 as f64));

    spicelib::LATREC(save.MIDR, save.LON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in northern hemisphere. Hit on south latitude boundary. Margin > 0. Exterior case. Hit is beyond west longitude boundary.", ctx)?;

    save.LON = (save.BOUNDS[[1, 1]] - (save.MARGIN * 2 as f64));

    spicelib::LATREC(save.MIDR, save.LON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in northern hemisphere. Hit on south latitude boundary. Margin > 0. Interior case. Hit is beyond east longitude boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.MIDR = ((save.BOUNDS[[1, 3]] + save.BOUNDS[[2, 3]]) / 2 as f64);

    save.LAT = save.BOUNDS[[1, 2]];

    save.LON = (save.BOUNDS[[2, 1]] + (save.MARGIN / 2 as f64));

    spicelib::LATREC(save.MIDR, save.LON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in northern hemisphere. Hit on south latitude boundary. Margin > 0. Exterior case. Hit is beyond east longitude boundary.", ctx)?;

    save.LON = (save.BOUNDS[[2, 1]] + (save.MARGIN * 2 as f64));

    spicelib::LATREC(save.MIDR, save.LON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in northern hemisphere. Hit on south latitude boundary. Margin > 0. Interior case. Hit is beyond outer radius boundary.", ctx)?;

    //
    // This is a place holder. There's no such hit case.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in northern hemisphere. Hit on south latitude boundary. Margin > 0. Exterior case. Hit is beyond outer radius boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = (spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.LAT = save.BOUNDS[[1, 2]];

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);

    save.R = (save.BOUNDS[[2, 3]] + ((save.BOUNDS[[2, 3]] * save.MARGIN) * 2 as f64));

    spicelib::LATREC(save.R, save.MIDLON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element in northern hemisphere. Hit on south latitude boundary. Margin > 0. Interior case. Hit is beyond inner radius boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.R = (save.BOUNDS[[1, 3]] - ((save.BOUNDS[[1, 3]] * save.MARGIN) / 2 as f64));

    save.LAT = save.BOUNDS[[1, 2]];

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);

    spicelib::LATREC(save.R, save.MIDLON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.X.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element in northern hemisphere. Hit on south latitude boundary. Margin > 0. Exterior case. Hit is beyond inner radius boundary.", ctx)?;

    //
    // Choose R to ensure a miss.
    //
    save.R = (save.BOUNDS[[1, 3]] / 2 as f64);

    save.LAT = save.BOUNDS[[1, 2]];

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);

    spicelib::LATREC(save.R, save.MIDLON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    //
    //     The following cases deal with hits on the inner bounding sphere.
    //
    //
    //        INNER RADIUS BOUNDARY
    //
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element that spans the equator. Hit on inner radius boundary. Margin > 0. Interior case. Hit is beyond east longitude boundary. Vertex is inside inner sphere; this is the \"first endpoint\" case.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);

    save.LON = (save.BOUNDS[[2, 1]] + (save.MARGIN / 2 as f64));

    save.R = save.BOUNDS[[1, 3]];

    spicelib::LATREC(save.R, save.LON, save.MIDLAT, save.XXPT.as_slice_mut());

    spicelib::CLEARD(3, save.VERTEX.as_slice_mut());
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element that spans the equator. Hit on inner radius boundary. Margin > 0. Exterior case. Hit is beyond east longitude boundary. Vertex is inside inner sphere; this is the \"first endpoint\" case.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);

    save.LON = (save.BOUNDS[[2, 1]] + (((2 as f64) * save.BOUNDS[[2, 3]]) * save.MARGIN));

    save.R = save.BOUNDS[[1, 3]];

    spicelib::LATREC(save.R, save.LON, save.MIDLAT, save.XXPT.as_slice_mut());

    spicelib::CLEARD(3, save.VERTEX.as_slice_mut());
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element that spans the equator. Hit on inner radius boundary. Margin > 0. Interior case. Hit is beyond west longitude boundary. Vertex is inside inner sphere; this is the \"first endpoint\" case.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);

    save.LON = (save.BOUNDS[[1, 1]] - (save.MARGIN / 2 as f64));

    save.R = save.BOUNDS[[1, 3]];

    spicelib::LATREC(save.R, save.LON, save.MIDLAT, save.XXPT.as_slice_mut());

    spicelib::CLEARD(3, save.VERTEX.as_slice_mut());
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element that spans the equator. Hit on inner radius boundary. Margin > 0. Exterior case. Hit is beyond west longitude boundary. Vertex is inside inner sphere; this is the \"first endpoint\" case.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);

    save.LON = (save.BOUNDS[[1, 1]] - (((2 as f64) * save.BOUNDS[[2, 3]]) * save.MARGIN));

    save.R = save.BOUNDS[[1, 3]];

    spicelib::LATREC(save.R, save.LON, save.MIDLAT, save.XXPT.as_slice_mut());

    spicelib::CLEARD(3, save.VERTEX.as_slice_mut());
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element that spans the equator. Hit on inner radius boundary. Margin > 0. Interior case. Hit is beyond north latitude boundary. Vertex is inside inner sphere; this is the \"first endpoint\" case.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);

    save.LAT = (save.BOUNDS[[2, 2]] + (save.MARGIN / 2 as f64));

    save.R = save.BOUNDS[[1, 3]];

    spicelib::LATREC(save.R, save.MIDLON, save.LAT, save.XXPT.as_slice_mut());

    spicelib::CLEARD(3, save.VERTEX.as_slice_mut());
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element that spans the equator. Hit on inner radius boundary. Margin > 0. Exterior case. Hit is beyond north latitude boundary. Vertex is inside inner sphere; this is the \"first endpoint\" case.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);

    save.LAT = (save.BOUNDS[[2, 2]] + (save.MARGIN * 2 as f64));

    save.R = save.BOUNDS[[1, 3]];

    spicelib::LATREC(save.R, save.MIDLON, save.LAT, save.XXPT.as_slice_mut());

    spicelib::CLEARD(3, save.VERTEX.as_slice_mut());
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element that spans the equator. Hit on inner radius boundary. Margin > 0. Interior case. Hit is beyond south latitude boundary. Vertex is inside inner sphere; this is the \"first endpoint\" case.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);

    save.LAT = (save.BOUNDS[[1, 2]] - (save.MARGIN / 2 as f64));

    save.R = save.BOUNDS[[1, 3]];

    spicelib::LATREC(save.R, save.MIDLON, save.LAT, save.XXPT.as_slice_mut());

    spicelib::CLEARD(3, save.VERTEX.as_slice_mut());
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element that spans the equator. Hit on inner radius boundary. Margin > 0. Exterior case. Hit is beyond south latitude boundary. Vertex is inside inner sphere; this is the \"first endpoint\" case.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);

    save.LAT = (save.BOUNDS[[1, 2]] - (save.MARGIN * 2 as f64));

    save.R = save.BOUNDS[[1, 3]];

    spicelib::LATREC(save.R, save.MIDLON, save.LAT, save.XXPT.as_slice_mut());

    spicelib::CLEARD(3, save.VERTEX.as_slice_mut());
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element that spans the equator. Hit on inner radius boundary. Margin > 0. Interior case. Hit is beyond east longitude boundary. Vertex is outside inner sphere; this is the \"second endpoint\" case.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);

    save.LON = (save.BOUNDS[[2, 1]] + (save.MARGIN / 2 as f64));

    save.R = save.BOUNDS[[1, 3]];

    spicelib::LATREC(save.R, save.LON, save.MIDLAT, save.XXPT.as_slice_mut());

    save.VERTEX[1] = -((2 as f64) * save.XXPT[1]);
    save.VERTEX[2] = -((2 as f64) * save.XXPT[2]);
    save.VERTEX[3] = save.XXPT[3];

    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element that spans the equator. Hit on inner radius boundary. Margin > 0. Exterior case. Hit is beyond east longitude boundary. Vertex is outside inner sphere; this is the \"second endpoint\" case.", ctx)?;

    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);

    save.LON = (save.BOUNDS[[2, 1]] + (((2 as f64) * save.BOUNDS[[2, 3]]) * save.MARGIN));

    save.R = save.BOUNDS[[1, 3]];

    spicelib::LATREC(save.R, save.LON, save.MIDLAT, save.XXPT.as_slice_mut());

    save.VERTEX[1] = -((2 as f64) * save.XXPT[1]);
    save.VERTEX[2] = -((2 as f64) * save.XXPT[2]);
    save.VERTEX[3] = save.XXPT[3];

    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element that spans the equator. Hit on inner radius boundary. Margin > 0. Interior case. Hit is beyond west longitude boundary. Vertex is outside inner sphere; this is the \"second endpoint\" case.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);

    save.LON = (save.BOUNDS[[1, 1]] - (save.MARGIN / 2 as f64));

    save.R = save.BOUNDS[[1, 3]];

    spicelib::LATREC(save.R, save.LON, save.MIDLAT, save.XXPT.as_slice_mut());

    save.VERTEX[1] = -((2 as f64) * save.XXPT[1]);
    save.VERTEX[2] = -((2 as f64) * save.XXPT[2]);
    save.VERTEX[3] = save.XXPT[3];
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element that spans the equator. Hit on inner radius boundary. Margin > 0. Exterior case. Hit is beyond west longitude boundary. Vertex is outside inner sphere; this is the \"second endpoint\" case.", ctx)?;

    save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);

    save.LON = (save.BOUNDS[[1, 1]] - (((2 as f64) * save.BOUNDS[[2, 3]]) * save.MARGIN));

    save.R = save.BOUNDS[[1, 3]];

    spicelib::LATREC(save.R, save.LON, save.MIDLAT, save.XXPT.as_slice_mut());

    save.VERTEX[1] = -((2 as f64) * save.XXPT[1]);
    save.VERTEX[2] = -((2 as f64) * save.XXPT[2]);
    save.VERTEX[3] = save.XXPT[3];
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element that spans the equator. Hit on inner radius boundary. Margin > 0. Interior case. Hit is beyond north latitude boundary. Vertex is outside inner sphere; this is the \"second endpoint\" case.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);

    save.LAT = (save.BOUNDS[[2, 2]] + (save.MARGIN / 2 as f64));

    save.R = save.BOUNDS[[1, 3]];

    spicelib::LATREC(save.R, save.MIDLON, save.LAT, save.XXPT.as_slice_mut());

    save.VERTEX[1] = -((2 as f64) * save.XXPT[1]);
    save.VERTEX[2] = -((2 as f64) * save.XXPT[2]);
    save.VERTEX[3] = save.XXPT[3];
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element that spans the equator. Hit on inner radius boundary. Margin > 0. Exterior case. Hit is beyond north latitude boundary. Vertex is outside inner sphere; this is the \"second endpoint\" case.", ctx)?;

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);

    save.LAT = (save.BOUNDS[[2, 2]] + (save.MARGIN * 2 as f64));

    save.R = save.BOUNDS[[1, 3]];

    spicelib::LATREC(save.R, save.MIDLON, save.LAT, save.XXPT.as_slice_mut());

    save.VERTEX[1] = -((2 as f64) * save.XXPT[1]);
    save.VERTEX[2] = -((2 as f64) * save.XXPT[2]);
    save.VERTEX[3] = -((2 as f64) * save.XXPT[3]);
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Small element that spans the equator. Hit on inner radius boundary. Margin > 0. Interior case. Hit is beyond south latitude boundary. Vertex is outside inner sphere; this is the \"second endpoint\" case.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);

    save.LAT = (save.BOUNDS[[1, 2]] - (save.MARGIN / 2 as f64));

    save.R = save.BOUNDS[[1, 3]];

    spicelib::LATREC(save.R, save.MIDLON, save.LAT, save.XXPT.as_slice_mut());

    save.VERTEX[1] = -((2 as f64) * save.XXPT[1]);
    save.VERTEX[2] = -((2 as f64) * save.XXPT[2]);
    save.VERTEX[3] = -((2 as f64) * save.XXPT[3]);
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Small element that spans the equator. Hit on inner radius boundary. Margin > 0. Exterior case. Hit is beyond south latitude boundary. Vertex is outside inner sphere; this is the \"second endpoint\" case.", ctx)?;

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);

    save.LAT = (save.BOUNDS[[1, 2]] - (save.MARGIN * 2 as f64));

    save.R = save.BOUNDS[[1, 3]];

    spicelib::LATREC(save.R, save.MIDLON, save.LAT, save.XXPT.as_slice_mut());

    save.VERTEX[1] = -((2 as f64) * save.XXPT[1]);
    save.VERTEX[2] = -((2 as f64) * save.XXPT[2]);
    save.VERTEX[3] = -((2 as f64) * save.XXPT[3]);
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    //
    // SPECIAL CASES
    //
    //
    // The following cases deal with hits near the poles
    //
    //
    //    NEAR NORTH POLE, OUTER SPHERE
    //
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Element ranging from equator to near north pole. Hit on outer sphere. Margin > 0. Interior case. Hit is beyond north latitude boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = 0.0;
    save.BOUNDS[[2, 2]] = (spicelib::HALFPI(ctx) - (POLMRG / 1.001));

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);

    save.LAT = (save.BOUNDS[[2, 2]] + (save.MARGIN / 2 as f64));

    save.R = save.BOUNDS[[2, 3]];

    spicelib::LATREC(save.R, save.MIDLON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VSCL(save.MAG, save.XXPT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Element ranging from equator to near north pole. Hit on outer sphere. Margin > 0. Interior case. Hit is below north latitude boundary, but above high latitude limit. Longitude is off by pi radians.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = 0.0;
    save.BOUNDS[[2, 2]] = (spicelib::HALFPI(ctx) - (POLMRG / 4 as f64));

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.MIDLON = (spicelib::PI(ctx) + ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64));

    save.LAT = (save.BOUNDS[[2, 2]] - (save.MARGIN / 4 as f64));

    save.R = save.BOUNDS[[2, 3]];

    spicelib::LATREC(save.R, save.MIDLON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VSCL(save.MAG, save.XXPT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Element ranging from equator to near north pole. Hit on outer sphere. Margin > 0. Exterior case. Hit is below north latitude boundary, and below high latitude limit. Longitude is off by pi radians.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = 0.0;
    save.BOUNDS[[2, 2]] = (spicelib::HALFPI(ctx) - (POLMRG / 2 as f64));

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.MIDLON = (spicelib::PI(ctx) + ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64));

    save.LAT = (save.BOUNDS[[2, 2]] - ((2 as f64) * POLMRG));

    save.R = save.BOUNDS[[2, 3]];

    spicelib::LATREC(save.R, save.MIDLON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VSCL(save.MAG, save.XXPT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Element ranging from equator to near south pole. Hit on outer sphere. Margin > 0. Interior case. Hit is beyond south latitude boundary.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = (-spicelib::HALFPI(ctx) + (POLMRG / 1.001));
    save.BOUNDS[[2, 2]] = 0.0;

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.MIDLON = ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64);

    save.LAT = (save.BOUNDS[[1, 2]] - (save.MARGIN / 2 as f64));

    save.R = save.BOUNDS[[2, 3]];

    spicelib::LATREC(save.R, save.MIDLON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VSCL(save.MAG, save.XXPT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Element ranging from equator to near south pole. Hit on outer sphere. Margin > 0. Interior case. Hit is above south latitude boundary, but below low latitude limit. Longitude is off by pi radians.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = (-spicelib::HALFPI(ctx) + (POLMRG / 4 as f64));
    save.BOUNDS[[2, 2]] = 0.0;

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.MIDLON = (spicelib::PI(ctx) + ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64));

    save.LAT = (save.BOUNDS[[1, 2]] + (save.MARGIN / 4 as f64));

    save.R = save.BOUNDS[[2, 3]];

    spicelib::LATREC(save.R, save.MIDLON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VSCL(save.MAG, save.XXPT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Element ranging from equator to near north pole. Hit on outer sphere. Margin > 0. Exterior case. Hit is above south latitude boundary, and above low latitude limit. Longitude is off by pi radians.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = (-spicelib::HALFPI(ctx) + (POLMRG / 2 as f64));
    save.BOUNDS[[2, 2]] = 0.0;

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.MIDLON = (spicelib::PI(ctx) + ((save.BOUNDS[[1, 1]] + save.BOUNDS[[2, 1]]) / 2 as f64));

    save.LAT = (save.BOUNDS[[1, 2]] + ((2 as f64) * POLMRG));

    save.R = save.BOUNDS[[2, 3]];

    spicelib::LATREC(save.R, save.MIDLON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VSCL(save.MAG, save.XXPT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Element is \"orange slice\" ranging from south to north pole. Vertex is on +Z axis; ray points in -Z direction.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = -spicelib::HALFPI(ctx);
    save.BOUNDS[[2, 2]] = spicelib::HALFPI(ctx);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.MAG = 10000000000.0;

    spicelib::VPACK(0.0, 0.0, save.BOUNDS[[2, 3]], save.XXPT.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, save.MAG, save.VERTEX.as_slice_mut());
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Element is \"orange slice\" ranging from south to north pole. Vertex is on -Z axis; ray points in +Z direction.", ctx)?;

    spicelib::VPACK(0.0, 0.0, -save.BOUNDS[[2, 3]], save.XXPT.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, -save.MAG, save.VERTEX.as_slice_mut());
    spicelib::VSUB(
        save.XXPT.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    // The following cases deal with hits near the poles but outside
    // of the latitude margins.
    //
    //
    //    HIGH LATITUDE, OUTER SPHERE
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Element is high latitude patch. Ray hits beyond east longitude boundary. Test of longitude margin scaling. Interior case.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = (((5 as f64) * spicelib::PI(ctx)) / 12 as f64);
    save.BOUNDS[[2, 2]] = ((spicelib::PI(ctx) / 2 as f64) - ((2 as f64) * POLMRG));

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[2, 3]];
    save.LAT = (save.BOUNDS[[2, 2]] - save.MARGIN);
    save.LON = (save.BOUNDS[[2, 1]] + ((save.MARGIN / 2 as f64) / f64::cos(save.LAT)));

    spicelib::LATREC(save.R, save.LON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VMINUS(save.Z.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Element is high latitude patch. Ray hits beyond east longitude boundary. Test of longitude margin scaling. Exterior case.", ctx)?;

    save.R = save.BOUNDS[[2, 3]];
    save.LAT = (save.BOUNDS[[2, 2]] - save.MARGIN);
    save.LON = (save.BOUNDS[[2, 1]] + ((save.MARGIN * 2 as f64) / f64::cos(save.LAT)));

    spicelib::LATREC(save.R, save.LON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VMINUS(save.Z.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Element is high latitude patch. Ray hits beyond west longitude boundary. Test of longitude margin scaling. Interior case.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = (((5 as f64) * spicelib::PI(ctx)) / 12 as f64);
    save.BOUNDS[[2, 2]] = ((spicelib::PI(ctx) / 2 as f64) - ((2 as f64) * POLMRG));

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[2, 3]];
    save.LAT = (save.BOUNDS[[2, 2]] - save.MARGIN);
    save.LON = (save.BOUNDS[[1, 1]] - ((save.MARGIN / 2 as f64) / f64::cos(save.LAT)));

    spicelib::LATREC(save.R, save.LON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VMINUS(save.Z.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Element is high latitude patch. Ray hits beyond west longitude boundary. Test of longitude margin scaling. Exterior case.", ctx)?;

    save.R = save.BOUNDS[[2, 3]];
    save.LAT = (save.BOUNDS[[2, 2]] - save.MARGIN);
    save.LON = (save.BOUNDS[[1, 1]] - ((save.MARGIN * 2 as f64) / f64::cos(save.LAT)));

    spicelib::LATREC(save.R, save.LON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VMINUS(save.Z.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // LOW LATITUDE, OUTER SPHERE
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Element is low latitude patch. Ray hits beyond east longitude boundary. Test of longitude margin scaling. Interior case.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = (-(spicelib::PI(ctx) / 2 as f64) + ((2 as f64) * POLMRG));
    save.BOUNDS[[2, 2]] = -(((5 as f64) * spicelib::PI(ctx)) / 12 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[2, 3]];
    save.LAT = (save.BOUNDS[[1, 2]] + save.MARGIN);
    save.LON = (save.BOUNDS[[2, 1]] + ((save.MARGIN / 2 as f64) / f64::cos(save.LAT)));

    spicelib::LATREC(save.R, save.LON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VEQU(save.Z.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Element is low latitude patch. Ray hits beyond east longitude boundary. Test of longitude margin scaling. Exterior case.", ctx)?;

    save.R = save.BOUNDS[[2, 3]];
    save.LAT = (save.BOUNDS[[1, 2]] + save.MARGIN);
    save.LON = (save.BOUNDS[[2, 1]] + ((save.MARGIN * 2 as f64) / f64::cos(save.LAT)));

    spicelib::LATREC(save.R, save.LON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VEQU(save.Z.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Element is low latitude patch. Ray hits beyond west longitude boundary. Test of longitude margin scaling. Interior case.", ctx)?;

    save.MARGIN = 0.00000001;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 6 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 6 as f64);

    save.BOUNDS[[1, 2]] = (-(spicelib::PI(ctx) / 2 as f64) + ((2 as f64) * POLMRG));
    save.BOUNDS[[2, 2]] = -(((5 as f64) * spicelib::PI(ctx)) / 12 as f64);

    save.BOUNDS[[1, 3]] = 3000.0;
    save.BOUNDS[[2, 3]] = (3000.0 + 50 as f64);

    save.R = save.BOUNDS[[2, 3]];
    save.LAT = (save.BOUNDS[[1, 2]] + save.MARGIN);
    save.LON = (save.BOUNDS[[1, 1]] - ((save.MARGIN / 2 as f64) / f64::cos(save.LAT)));

    spicelib::LATREC(save.R, save.LON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VEQU(save.Z.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

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
    testutil::TCASE(b"Element is low latitude patch. Ray hits beyond west longitude boundary. Test of longitude margin scaling. Exterior case.", ctx)?;

    save.R = save.BOUNDS[[2, 3]];
    save.LAT = (save.BOUNDS[[1, 2]] + save.MARGIN);
    save.LON = (save.BOUNDS[[1, 1]] - ((save.MARGIN * 2 as f64) / f64::cos(save.LAT)));

    spicelib::LATREC(save.R, save.LON, save.LAT, save.XXPT.as_slice_mut());

    save.MAG = 1000.0;

    spicelib::VLCOM(
        1.0,
        save.XXPT.as_slice(),
        -save.MAG,
        save.Z.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VEQU(save.Z.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTLAT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //***********************************************************************
    //
    //     Error cases
    //
    //***********************************************************************

    //
    // ZZRYTLAT is declared to be error free, in the interest of
    // speed. This decision may need to be revisited.
    //

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
