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
const TIGHT: f64 = 0.00000000001;

struct SaveVars {
    A: f64,
    ALT: f64,
    ANGLE: f64,
    APEX: StackArray<f64, 3>,
    B: f64,
    BOUNDS: StackArray2D<f64, 6>,
    COLAT: f64,
    CONST: f64,
    CORPAR: StackArray<f64, 10>,
    EMAX: f64,
    EMIN: f64,
    ENDPT1: StackArray<f64, 3>,
    ENDPT2: StackArray<f64, 3>,
    F: f64,
    H: f64,
    H2: f64,
    LAT: f64,
    LON: f64,
    MARGIN: f64,
    MAXALT: f64,
    MAXD: f64,
    MINALT: f64,
    NEGDIR: StackArray<f64, 3>,
    OFFSET: StackArray<f64, 3>,
    ORIGIN: StackArray<f64, 3>,
    P: StackArray<f64, 3>,
    PMAX: f64,
    PMIN: f64,
    RAYDIR: StackArray<f64, 3>,
    RE: f64,
    RP: f64,
    S: f64,
    TARGET: StackArray<f64, 3>,
    TARGT2: StackArray<f64, 3>,
    TOL: f64,
    UDIR: StackArray<f64, 3>,
    UPLNML: StackArray<f64, 3>,
    VEAST: StackArray<f64, 3>,
    VERTEX: StackArray<f64, 3>,
    VWEST: StackArray<f64, 3>,
    XINCPT: f64,
    XPT: StackArray<f64, 3>,
    XPT1: StackArray<f64, 3>,
    XPT2: StackArray<f64, 3>,
    XXPT: StackArray<f64, 3>,
    XYX: StackArray<f64, 3>,
    YINCPT: f64,
    Z: StackArray<f64, 3>,
    NXPTS: i32,
    XNXPTS: i32,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut A: f64 = 0.0;
        let mut ALT: f64 = 0.0;
        let mut ANGLE: f64 = 0.0;
        let mut APEX = StackArray::<f64, 3>::new(1..=3);
        let mut B: f64 = 0.0;
        let mut BOUNDS = StackArray2D::<f64, 6>::new(1..=2, 1..=3);
        let mut COLAT: f64 = 0.0;
        let mut CONST: f64 = 0.0;
        let mut CORPAR = StackArray::<f64, 10>::new(1..=NSYPAR);
        let mut EMAX: f64 = 0.0;
        let mut EMIN: f64 = 0.0;
        let mut ENDPT1 = StackArray::<f64, 3>::new(1..=3);
        let mut ENDPT2 = StackArray::<f64, 3>::new(1..=3);
        let mut F: f64 = 0.0;
        let mut H: f64 = 0.0;
        let mut H2: f64 = 0.0;
        let mut LAT: f64 = 0.0;
        let mut LON: f64 = 0.0;
        let mut MARGIN: f64 = 0.0;
        let mut MAXALT: f64 = 0.0;
        let mut MAXD: f64 = 0.0;
        let mut MINALT: f64 = 0.0;
        let mut NEGDIR = StackArray::<f64, 3>::new(1..=3);
        let mut OFFSET = StackArray::<f64, 3>::new(1..=3);
        let mut ORIGIN = StackArray::<f64, 3>::new(1..=3);
        let mut P = StackArray::<f64, 3>::new(1..=3);
        let mut PMAX: f64 = 0.0;
        let mut PMIN: f64 = 0.0;
        let mut RAYDIR = StackArray::<f64, 3>::new(1..=3);
        let mut RE: f64 = 0.0;
        let mut RP: f64 = 0.0;
        let mut S: f64 = 0.0;
        let mut TARGET = StackArray::<f64, 3>::new(1..=3);
        let mut TARGT2 = StackArray::<f64, 3>::new(1..=3);
        let mut TOL: f64 = 0.0;
        let mut UDIR = StackArray::<f64, 3>::new(1..=3);
        let mut UPLNML = StackArray::<f64, 3>::new(1..=3);
        let mut VEAST = StackArray::<f64, 3>::new(1..=3);
        let mut VERTEX = StackArray::<f64, 3>::new(1..=3);
        let mut VWEST = StackArray::<f64, 3>::new(1..=3);
        let mut XINCPT: f64 = 0.0;
        let mut XPT = StackArray::<f64, 3>::new(1..=3);
        let mut XPT1 = StackArray::<f64, 3>::new(1..=3);
        let mut XPT2 = StackArray::<f64, 3>::new(1..=3);
        let mut XXPT = StackArray::<f64, 3>::new(1..=3);
        let mut XYX = StackArray::<f64, 3>::new(1..=3);
        let mut YINCPT: f64 = 0.0;
        let mut Z = StackArray::<f64, 3>::new(1..=3);
        let mut NXPTS: i32 = 0;
        let mut XNXPTS: i32 = 0;
        let mut FOUND: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::D(0.0), 3 as usize))
                .chain([]);

            ORIGIN
                .iter_mut()
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
            A,
            ALT,
            ANGLE,
            APEX,
            B,
            BOUNDS,
            COLAT,
            CONST,
            CORPAR,
            EMAX,
            EMIN,
            ENDPT1,
            ENDPT2,
            F,
            H,
            H2,
            LAT,
            LON,
            MARGIN,
            MAXALT,
            MAXD,
            MINALT,
            NEGDIR,
            OFFSET,
            ORIGIN,
            P,
            PMAX,
            PMIN,
            RAYDIR,
            RE,
            RP,
            S,
            TARGET,
            TARGT2,
            TOL,
            UDIR,
            UPLNML,
            VEAST,
            VERTEX,
            VWEST,
            XINCPT,
            XPT,
            XPT1,
            XPT2,
            XXPT,
            XYX,
            YINCPT,
            Z,
            NXPTS,
            XNXPTS,
            FOUND,
        }
    }
}

//$Procedure F_ZZRYTPDT ( ZZRYTPDT tests )
pub fn F_ZZRYTPDT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    // Initial values
    //

    //
    // Saved variables
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_ZZRYTPDT", ctx)?;

    //***********************************************************************
    //
    //     Error cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error: negative margin.", ctx)?;

    save.RE = 3000.0;
    save.F = 0.1;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 2 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 2 as f64);

    save.BOUNDS[[1, 3]] = -(save.RE / 2.0);
    save.BOUNDS[[2, 3]] = (save.RE / 2.0);

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    spicelib::VPACK(3010.0, 3020.0, 2800.0, save.VERTEX.as_slice_mut());

    save.MARGIN = -0.000001;

    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error: zero direction vector.", ctx)?;

    save.MARGIN = 0.000001;

    spicelib::CLEARD(3, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ZEROVECTOR)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error: longitude bound out of range.", ctx)?;

    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    save.BOUNDS[[1, 1]] = -((3 as f64) * spicelib::PI(ctx));

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    save.BOUNDS[[1, 1]] = -((2 as f64) * spicelib::PI(ctx));
    save.BOUNDS[[2, 1]] = ((3 as f64) * spicelib::PI(ctx));

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //***********************************************************************
    //
    //     Normal cases
    //
    //***********************************************************************

    //***********************************************************************
    //
    //
    //     OBLATE CASES
    //
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Setup for simple oblate cases.", ctx)?;
    //
    //
    // The following simple cases all use the same volume element:
    //
    //    RE:      3000.0
    //    F:          0.5
    //
    //    Lon:     -PI()/4 : PI()/4
    //    Lat:     -PI()/6 : PI()/3
    //    Alt:     -100.0  : 100.0
    //

    save.RE = 3000.0;
    save.F = 0.5;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 3 as f64);

    save.BOUNDS[[1, 3]] = -100.0;
    save.BOUNDS[[2, 3]] = 100.0;

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    //
    // Use a positive margin.
    //
    save.MARGIN = 0.00001;

    //
    // Get the radii of the outer and inner bounding ellipsoids.
    // Take margin into account.
    //
    // This is the oblate case.
    //
    save.RP = (save.RE * (1.0 - save.F));

    save.MINALT = (save.BOUNDS[[1, 3]] - (save.MARGIN * f64::abs(save.BOUNDS[[1, 3]])));
    save.MAXALT = (save.BOUNDS[[2, 3]] + (save.MARGIN * f64::abs(save.BOUNDS[[2, 3]])));

    spicelib::ZZELLBDS(
        save.RE,
        save.RP,
        save.MAXALT,
        save.MINALT,
        &mut save.EMAX,
        &mut save.PMAX,
        &mut save.EMIN,
        &mut save.PMIN,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Ray\'s vertex is inside element.", ctx)?;

    //
    // Create a vertex at maximum altitude. This point will be
    // inside the outer bounding ellipsoid, so it will be
    // considered to be inside the volume element.
    //
    save.LON = 0.0;
    save.LAT = 0.0;
    save.ALT = save.BOUNDS[[2, 3]];

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

    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"XPT",
        save.XPT.as_slice(),
        b"~~/",
        save.VERTEX.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Ray misses outer bounding spheroid.", ctx)?;

    spicelib::VPACK(save.RE, 0.0, save.RE, save.VERTEX.as_slice_mut());
    spicelib::VPACK(-1.0, 0.0, 0.0, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
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
    testutil::TCASE(b"Ray hits outer bounding spheroid but misses element.", ctx)?;

    spicelib::VPACK(save.RE, 0.0, (save.RP * 0.99), save.VERTEX.as_slice_mut());
    spicelib::VPACK(-1.0, 0.0, 0.0, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
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
    testutil::TCASE(b"Ray hits element on outer bounding spheroid.", ctx)?;

    spicelib::VPACK(
        ((2 as f64) * save.RE),
        0.0,
        (save.RP / 2 as f64),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VPACK(-1.0, 0.0, 0.0, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    //
    // Compute the expected intercept.
    //
    spicelib::SURFPT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.EMAX,
        save.EMAX,
        save.PMAX,
        save.XXPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"SURFPT found", save.FOUND, true, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"XPT",
        save.XPT.as_slice(),
        b"~~/",
        save.XPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(
        b"Ray hits element on inner bounding spheroid. Vertex is outside outer spheroid.",
        ctx,
    )?;

    spicelib::VPACK(
        -((4 as f64) * save.RE),
        0.0,
        (save.RP / 2 as f64),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VPACK(1.0, 0.0, 0.0, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    //
    // Compute the expected intercept. We need to approach the
    // inner ellipsoid from the opposite side, since the vertex
    // and element are on opposite sides of the Z axis.
    //
    spicelib::VEQU(save.VERTEX.as_slice(), save.ENDPT2.as_slice_mut());

    save.ENDPT2[1] = -save.ENDPT2[1];

    spicelib::VMINUS(save.RAYDIR.as_slice(), save.NEGDIR.as_slice_mut());

    spicelib::SURFPT(
        save.ENDPT2.as_slice(),
        save.NEGDIR.as_slice(),
        save.EMIN,
        save.EMIN,
        save.PMIN,
        save.XXPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"SURFPT found", save.FOUND, true, OK, ctx)?;

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
        b"Ray hits element on inner bounding spheroid. Vertex is inside inner spheroid.",
        ctx,
    )?;

    spicelib::VPACK(0.0, 0.0, (save.RP / 2 as f64), save.VERTEX.as_slice_mut());
    spicelib::VPACK(1.0, 0.0, 0.0, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    //
    // Compute the expected intercept.
    //
    spicelib::SURFPT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.EMIN,
        save.EMIN,
        save.PMIN,
        save.XXPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"SURFPT found", save.FOUND, true, OK, ctx)?;

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

    testutil::TCASE(b"Ray hits element from west side.", ctx)?;

    save.S = (f64::sqrt(2.0) / 2 as f64);

    spicelib::VPACK(
        ((save.S * save.RE) - 10.0),
        -((2 as f64) * save.RE),
        0.0,
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 0.0, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    //
    // Compute the expected intercept.
    //
    //
    // Create the normal vector for the western boundary.
    //
    spicelib::LATREC(1.0, save.BOUNDS[[1, 1]], 0.0, save.VWEST.as_slice_mut());
    spicelib::UCRSS(
        save.VWEST.as_slice(),
        save.Z.as_slice(),
        save.UPLNML.as_slice_mut(),
    );

    //
    // The western plane contains the origin.
    //
    save.CONST = 0.0;

    spicelib::VHAT(save.RAYDIR.as_slice(), save.UDIR.as_slice_mut());

    save.MAXD = ((4 as f64) * save.RE);

    spicelib::ZZINRYPL(
        save.VERTEX.as_slice(),
        save.UDIR.as_slice(),
        save.UPLNML.as_slice(),
        save.CONST,
        save.MAXD,
        &mut save.XNXPTS,
        save.XXPT.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"ZZINRYPL XNXTPS", save.XNXPTS, b"=", 1, 0, OK, ctx)?;

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

    testutil::TCASE(b"Ray hits element from east side.", ctx)?;

    save.S = (f64::sqrt(2.0) / 2 as f64);

    spicelib::VPACK(
        ((save.S * save.RE) - 10.0),
        ((2 as f64) * save.RE),
        0.0,
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VPACK(0.0, -1.0, 0.0, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    //
    // Compute the expected intercept.
    //
    //
    // Create the normal vector for the eastern boundary.
    //
    spicelib::LATREC(1.0, save.BOUNDS[[2, 1]], 0.0, save.VEAST.as_slice_mut());
    spicelib::UCRSS(
        save.Z.as_slice(),
        save.VEAST.as_slice(),
        save.UPLNML.as_slice_mut(),
    );

    //
    // The eastern plane contains the origin.
    //
    save.CONST = 0.0;

    spicelib::VHAT(save.RAYDIR.as_slice(), save.UDIR.as_slice_mut());

    save.MAXD = ((4 as f64) * save.RE);

    spicelib::ZZINRYPL(
        save.VERTEX.as_slice(),
        save.UDIR.as_slice(),
        save.UPLNML.as_slice(),
        save.CONST,
        save.MAXD,
        &mut save.XNXPTS,
        save.XXPT.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"ZZINRYPL XNXTPS", save.XNXPTS, b"=", 1, 0, OK, ctx)?;

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

    testutil::TCASE(b"Ray hits element from north side.", ctx)?;

    spicelib::GEOREC(
        0.0,
        save.BOUNDS[[2, 2]],
        0.0,
        save.RE,
        save.F,
        save.VERTEX.as_slice_mut(),
        ctx,
    )?;

    spicelib::RECGEO(
        save.VERTEX.as_slice(),
        save.RE,
        save.F,
        &mut save.LON,
        &mut save.LAT,
        &mut save.ALT,
        ctx,
    )?;

    spicelib::GEOREC(
        0.0,
        save.BOUNDS[[2, 2]],
        save.BOUNDS[[2, 3]],
        save.RE,
        save.F,
        save.VERTEX.as_slice_mut(),
        ctx,
    )?;

    //
    // Shift the vertex toward the origin in the X direction and
    // raise it above the outer bounding sphere in the Z direction.

    save.VERTEX[1] = (save.VERTEX[1] - 100.0);
    save.VERTEX[3] = ((2 as f64) * save.RE);

    spicelib::VPACK(0.0, 0.0, -1.0, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    //
    // Compute the expected intercept.
    //
    // The intercept should be on the upper latitude cone.
    //
    spicelib::VEQU(save.VERTEX.as_slice(), save.ENDPT1.as_slice_mut());
    spicelib::VEQU(save.ENDPT1.as_slice(), save.ENDPT2.as_slice_mut());

    save.ENDPT2[3] = -save.ENDPT2[3];

    //
    // Caution! The apex of the latitude cone is not the origin.
    // It lies on the -Z axis. Compute the intercept.
    //
    save.ANGLE = (spicelib::HALFPI(ctx) - save.BOUNDS[[2, 2]]);

    spicelib::ZZELNAXX(
        save.RE,
        save.RP,
        save.LAT,
        &mut save.XINCPT,
        &mut save.YINCPT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.ORIGIN.as_slice(), save.APEX.as_slice_mut());
    save.APEX[3] = save.YINCPT;

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.Z.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.XNXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKSI(b"INCNSG XNXPTS", save.XNXPTS, b"=", 1, 0, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"XPT",
        save.XPT.as_slice(),
        b"~~/",
        save.XPT1.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Ray hits element from south side.", ctx)?;

    spicelib::GEOREC(
        0.0,
        save.BOUNDS[[1, 2]],
        save.BOUNDS[[2, 3]],
        save.RE,
        save.F,
        save.VERTEX.as_slice_mut(),
        ctx,
    )?;

    //
    // Shift the vertex toward the origin in the X direction and
    // lower it below the outer bounding sphere in the Z direction.

    save.VERTEX[1] = (save.VERTEX[1] - 100.0);
    save.VERTEX[3] = -((2 as f64) * save.RE);

    spicelib::VPACK(0.0, 0.0, 1.0, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    //
    // Compute the expected intercept.
    //
    // The intercept should be on the lower latitude cone.
    //
    spicelib::VEQU(save.VERTEX.as_slice(), save.ENDPT1.as_slice_mut());
    spicelib::VEQU(save.ENDPT1.as_slice(), save.ENDPT2.as_slice_mut());

    save.ENDPT2[3] = -save.ENDPT2[3];

    //
    // Caution! The apex of the latitude cone is not the origin.
    // It lies on the +Z axis. Compute the intercept.
    //
    save.ANGLE = (spicelib::HALFPI(ctx) - save.BOUNDS[[1, 2]]);

    save.LAT = save.BOUNDS[[1, 2]];

    spicelib::ZZELNAXX(
        save.RE,
        save.RP,
        save.LAT,
        &mut save.XINCPT,
        &mut save.YINCPT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.ORIGIN.as_slice(), save.APEX.as_slice_mut());
    save.APEX[3] = save.YINCPT;

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.Z.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.XNXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKSI(b"INCNSG XNXPTS", save.XNXPTS, b"=", 1, 0, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"XPT",
        save.XPT.as_slice(),
        b"~~/",
        save.XPT1.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // In the following cases, we change the latitude bounds of
    // the element.
    //

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(
        b"North element boundary is at 0 latitude. Ray hits element from north side.",
        ctx,
    )?;

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 2]] = 0.0;

    //
    // Find the point on the reference ellipsoid at maximum altitude
    // and latitude, longitude zero.
    //
    spicelib::GEOREC(
        0.0,
        save.BOUNDS[[2, 2]],
        save.BOUNDS[[2, 3]],
        save.RE,
        save.F,
        save.VERTEX.as_slice_mut(),
        ctx,
    )?;

    //
    // Shift the vertex toward the origin in the X direction and
    // raise it above the outer bounding sphere in the Z direction.

    save.VERTEX[1] = (save.VERTEX[1] - 100.0);
    save.VERTEX[3] = ((2 as f64) * save.RE);

    spicelib::VPACK(0.0, 0.0, -1.0, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    //
    // Compute the expected intercept.
    //
    // The intercept should be on the "upper latitude cone."
    //
    spicelib::VEQU(save.VERTEX.as_slice(), save.ENDPT1.as_slice_mut());
    spicelib::VEQU(save.ENDPT1.as_slice(), save.ENDPT2.as_slice_mut());

    save.ENDPT2[3] = -save.ENDPT2[3];

    //
    // Caution! The apex of the latitude cone usually is not the origin.
    // It usually lies on the -Z axis. However, in this case the apex
    // should be the origin. Compute the intercept by the usual means.
    //
    save.ANGLE = (spicelib::HALFPI(ctx) - save.BOUNDS[[2, 2]]);

    save.LAT = save.BOUNDS[[2, 2]];

    spicelib::ZZELNAXX(
        save.RE,
        save.RP,
        save.LAT,
        &mut save.XINCPT,
        &mut save.YINCPT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.ORIGIN.as_slice(), save.APEX.as_slice_mut());
    save.APEX[3] = save.YINCPT;

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.Z.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.XNXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKSI(b"INCNSG XNXPTS", save.XNXPTS, b"=", 1, 0, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"XPT",
        save.XPT.as_slice(),
        b"~~/",
        save.XPT1.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(
        b"North element boundary is at negative latitude. Ray hits element from north side.",
        ctx,
    )?;

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 6 as f64);

    //
    // Find the point on the reference ellipsoid at maximum altitude
    // and latitude, longitude zero.
    //
    spicelib::GEOREC(
        0.0,
        save.BOUNDS[[2, 2]],
        save.BOUNDS[[2, 3]],
        save.RE,
        save.F,
        save.VERTEX.as_slice_mut(),
        ctx,
    )?;

    //
    // Shift the vertex toward the origin in the X direction and
    // raise it above the outer bounding sphere in the Z direction.

    save.VERTEX[1] = (save.VERTEX[1] - 100.0);
    save.VERTEX[3] = ((2 as f64) * save.RE);

    spicelib::VPACK(0.0, 0.0, -1.0, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    //
    // Compute the expected intercept.
    //
    // The intercept should be on the "upper latitude cone."
    //
    spicelib::VEQU(save.VERTEX.as_slice(), save.ENDPT1.as_slice_mut());
    spicelib::VEQU(save.ENDPT1.as_slice(), save.ENDPT2.as_slice_mut());

    save.ENDPT2[3] = -save.ENDPT2[3];

    //
    // Caution! The apex of the latitude cone usually is not the origin.
    // It usually lies on the -Z axis. However, in this case the apex
    // should be the origin. Compute the intercept by the usual means.
    //
    save.ANGLE = (spicelib::HALFPI(ctx) - save.BOUNDS[[2, 2]]);

    save.LAT = save.BOUNDS[[2, 2]];

    spicelib::ZZELNAXX(
        save.RE,
        save.RP,
        save.LAT,
        &mut save.XINCPT,
        &mut save.YINCPT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.ORIGIN.as_slice(), save.APEX.as_slice_mut());
    save.APEX[3] = save.YINCPT;

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.Z.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.XNXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKSI(b"INCNSG XNXPTS", save.XNXPTS, b"=", 1, 0, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"XPT",
        save.XPT.as_slice(),
        b"~~/",
        save.XPT1.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(
        b"South element boundary is at 0 latitude. Ray hits element from south side.",
        ctx,
    )?;

    save.BOUNDS[[1, 2]] = 0.0;
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 3 as f64);

    //
    // Find the point on the reference ellipsoid at maximum altitude
    // and minimum latitude, longitude zero.
    //
    spicelib::GEOREC(
        0.0,
        save.BOUNDS[[1, 2]],
        save.BOUNDS[[2, 3]],
        save.RE,
        save.F,
        save.VERTEX.as_slice_mut(),
        ctx,
    )?;

    //
    // Shift the vertex toward the origin in the X direction and
    // lower it below the outer bounding sphere in the -Z direction.

    save.VERTEX[1] = (save.VERTEX[1] - 100.0);
    save.VERTEX[3] = -((2 as f64) * save.RE);

    spicelib::VPACK(0.0, 0.0, 1.0, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    //
    // Compute the expected intercept.
    //
    // The intercept should be on the "lower latitude cone."
    //
    spicelib::VEQU(save.VERTEX.as_slice(), save.ENDPT1.as_slice_mut());
    spicelib::VEQU(save.ENDPT1.as_slice(), save.ENDPT2.as_slice_mut());

    save.ENDPT2[3] = -save.ENDPT2[3];

    //
    // Caution! The apex of the latitude cone usually is not the origin.
    // It usually lies on the +Z axis (for cones of negative latitude).
    // However, in this case the apex should be the origin. Compute
    // the intercept by the usual means.
    //
    save.ANGLE = (spicelib::HALFPI(ctx) - save.BOUNDS[[1, 2]]);

    save.LAT = save.BOUNDS[[1, 2]];

    spicelib::ZZELNAXX(
        save.RE,
        save.RP,
        save.LAT,
        &mut save.XINCPT,
        &mut save.YINCPT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.ORIGIN.as_slice(), save.APEX.as_slice_mut());
    save.APEX[3] = save.YINCPT;

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.Z.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.XNXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKSI(b"INCNSG XNXPTS", save.XNXPTS, b"=", 1, 0, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"XPT",
        save.XPT.as_slice(),
        b"~~/",
        save.XPT1.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(
        b"South element boundary is at negative latitude. Ray hits element from south side.",
        ctx,
    )?;

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 3 as f64);

    //
    // Find the point on the reference ellipsoid at maximum altitude
    // and minimum latitude, longitude zero.
    //
    spicelib::GEOREC(
        0.0,
        save.BOUNDS[[1, 2]],
        save.BOUNDS[[2, 3]],
        save.RE,
        save.F,
        save.VERTEX.as_slice_mut(),
        ctx,
    )?;

    //
    // Shift the vertex toward the origin in the X direction and
    // lower it below the outer bounding sphere in the -Z direction.

    save.VERTEX[1] = (save.VERTEX[1] - 100.0);
    save.VERTEX[3] = -((2 as f64) * save.RE);

    spicelib::VPACK(0.0, 0.0, 1.0, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    //
    // Compute the expected intercept.
    //
    // The intercept should be on the lower latitude cone.
    //
    spicelib::VEQU(save.VERTEX.as_slice(), save.ENDPT1.as_slice_mut());
    spicelib::VEQU(save.ENDPT1.as_slice(), save.ENDPT2.as_slice_mut());

    save.ENDPT2[3] = -save.ENDPT2[3];

    //
    // Caution! The apex of the latitude cone usually is not the origin.
    // It usually lies on the +Z axis (for cones of negative latitude).
    // However, in this case the apex should be the origin. Compute
    // the intercept by the usual means.
    //
    save.ANGLE = (spicelib::HALFPI(ctx) - save.BOUNDS[[1, 2]]);

    save.LAT = save.BOUNDS[[1, 2]];

    spicelib::ZZELNAXX(
        save.RE,
        save.RP,
        save.LAT,
        &mut save.XINCPT,
        &mut save.YINCPT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.ORIGIN.as_slice(), save.APEX.as_slice_mut());
    save.APEX[3] = save.YINCPT;

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.Z.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.XNXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKSI(b"INCNSG XNXPTS", save.XNXPTS, b"=", 1, 0, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"XPT",
        save.XPT.as_slice(),
        b"~~/",
        save.XPT1.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The following latitude boundary intercept cases test the handling
    // of multiple roots in the ray-cone intercept computation. In these
    // cases, the second root should be chosen.
    //

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(
        b"North element boundary is at positive latitude. Ray hits element from the -X direction.",
        ctx,
    )?;

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 3 as f64);

    //
    // Find the point on the reference ellipsoid at maximum altitude
    // and maximum latitude, longitude zero.
    //
    spicelib::GEOREC(
        0.0,
        save.BOUNDS[[2, 2]],
        save.BOUNDS[[2, 3]],
        save.RE,
        save.F,
        save.VERTEX.as_slice_mut(),
        ctx,
    )?;

    //
    // Shift the vertex in the -X direction. and lower it below the outer
    // bounding sphere in the -Z direction.
    //
    save.VERTEX[1] = (save.VERTEX[1] - ((10 as f64) * save.RE));
    save.VERTEX[3] = (save.VERTEX[3] - 10.0);

    //
    // This ray should hit the upper latitude cone twice. The second
    // intercept is the one we want.
    //
    spicelib::VPACK(1.0, 0.0, 0.0, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    //
    // Compute the expected intercept.
    //
    // The intercept should be on the upper latitude cone.
    //
    spicelib::VEQU(save.VERTEX.as_slice(), save.ENDPT1.as_slice_mut());
    spicelib::VEQU(save.ENDPT1.as_slice(), save.ENDPT2.as_slice_mut());

    save.ENDPT2[1] = -save.ENDPT2[1];

    save.ANGLE = (spicelib::HALFPI(ctx) - save.BOUNDS[[2, 2]]);

    save.LAT = save.BOUNDS[[2, 2]];

    //
    // The cone's apex should be on the -Z axis.
    //
    spicelib::ZZELNAXX(
        save.RE,
        save.RP,
        save.LAT,
        &mut save.XINCPT,
        &mut save.YINCPT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.ORIGIN.as_slice(), save.APEX.as_slice_mut());
    save.APEX[3] = save.YINCPT;

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.Z.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.XNXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;

    //
    // Note: we expect two roots. The second one is the correct one.
    //
    testutil::CHCKSI(b"INCNSG XNXPTS", save.XNXPTS, b"=", 2, 0, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"XPT",
        save.XPT.as_slice(),
        b"~~/",
        save.XPT2.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(
        b"South element boundary is at negative latitude. Ray hits element from the -X direction.",
        ctx,
    )?;

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 3 as f64);

    //
    // Find the point on the reference ellipsoid at maximum altitude
    // and minimum latitude, longitude zero.
    //
    spicelib::GEOREC(
        0.0,
        save.BOUNDS[[1, 2]],
        save.BOUNDS[[2, 3]],
        save.RE,
        save.F,
        save.VERTEX.as_slice_mut(),
        ctx,
    )?;

    //
    // Shift the vertex in the -X direction. and raise it below the outer
    // bounding sphere in the +Z direction.
    //
    save.VERTEX[1] = (save.VERTEX[1] - ((10 as f64) * save.RE));
    save.VERTEX[3] = (save.VERTEX[3] + 10.0);

    //
    // This ray should hit the lower latitude cone twice. The second
    // intercept is the one we want.
    //
    spicelib::VPACK(1.0, 0.0, 0.0, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    //
    // Compute the expected intercept.
    //
    // The intercept should be on the lower latitude cone.
    //
    spicelib::VEQU(save.VERTEX.as_slice(), save.ENDPT1.as_slice_mut());
    spicelib::VEQU(save.ENDPT1.as_slice(), save.ENDPT2.as_slice_mut());

    save.ENDPT2[1] = -save.ENDPT2[1];

    save.ANGLE = (spicelib::HALFPI(ctx) - save.BOUNDS[[1, 2]]);

    save.LAT = save.BOUNDS[[1, 2]];

    //
    // The cone's apex should be on the +Z axis.
    //
    spicelib::ZZELNAXX(
        save.RE,
        save.RP,
        save.LAT,
        &mut save.XINCPT,
        &mut save.YINCPT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.ORIGIN.as_slice(), save.APEX.as_slice_mut());
    save.APEX[3] = save.YINCPT;

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.Z.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.XNXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;

    //
    // Note: we expect two roots. The second one is the correct one.
    //
    testutil::CHCKSI(b"INCNSG XNXPTS", save.XNXPTS, b"=", 2, 0, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"XPT",
        save.XPT.as_slice(),
        b"~~/",
        save.XPT2.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //***********************************************************************
    //
    //
    //     PROLATE CASES
    //
    //
    //***********************************************************************

    //
    // Repeat the cases above using a prolate reference ellipsoid.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Setup for simple prolate cases.", ctx)?;
    //
    //
    // The following simple cases all use the same volume element:
    //
    //    RE:      3000.0
    //    F:         -1.0
    //
    //    Lon:     -PI()/4 : PI()/4
    //    Lat:     -PI()/6 : PI()/3
    //    Alt:     -100.0  : 100.0
    //

    save.RE = 3000.0;
    save.F = -1.0;

    save.BOUNDS[[1, 1]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 3 as f64);

    save.BOUNDS[[1, 3]] = -100.0;
    save.BOUNDS[[2, 3]] = 100.0;

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    //
    // Use a positive margin.
    //
    save.MARGIN = 0.00001;

    //
    // Get the radii of the outer and inner bounding ellipsoids.
    // Take margin into account.
    //
    // This is the prolate case.
    //
    save.RP = (save.RE * (1.0 - save.F));

    save.MINALT = (save.BOUNDS[[1, 3]] - (save.MARGIN * f64::abs(save.BOUNDS[[1, 3]])));
    save.MAXALT = (save.BOUNDS[[2, 3]] + (save.MARGIN * f64::abs(save.BOUNDS[[2, 3]])));

    //
    // Careful...see the header of ZZELLBDS for info on use
    // with prolate ellipsoids.
    //
    spicelib::ZZELLBDS(
        save.RP,
        save.RE,
        save.MAXALT,
        save.MINALT,
        &mut save.PMAX,
        &mut save.EMAX,
        &mut save.PMIN,
        &mut save.EMIN,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Ray\'s vertex is inside element.", ctx)?;

    //
    // Create a vertex at maximum altitude. This point will be
    // inside the outer bounding ellipsoid, so it will be
    // considered to be inside the volume element.
    //
    save.LON = 0.0;
    save.LAT = 0.0;
    save.ALT = save.BOUNDS[[2, 3]];

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

    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"XPT",
        save.XPT.as_slice(),
        b"~~/",
        save.VERTEX.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Ray misses outer bounding spheroid.", ctx)?;

    spicelib::VPACK(save.RP, 0.0, save.RP, save.VERTEX.as_slice_mut());
    spicelib::VPACK(-1.0, 0.0, 0.0, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
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
    testutil::TCASE(b"Ray hits outer bounding spheroid but misses element.", ctx)?;

    spicelib::VPACK((save.RP * 0.99), 0.0, save.RE, save.VERTEX.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, -1.0, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
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
    testutil::TCASE(b"Ray hits element on outer bounding spheroid.", ctx)?;

    spicelib::VPACK(
        ((2 as f64) * save.RE),
        0.0,
        (save.RP / 2 as f64),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VPACK(-1.0, 0.0, 0.0, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    //
    // Compute the expected intercept.
    //
    spicelib::SURFPT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.EMAX,
        save.EMAX,
        save.PMAX,
        save.XXPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"SURFPT found", save.FOUND, true, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"XPT",
        save.XPT.as_slice(),
        b"~~/",
        save.XPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(
        b"Ray hits element on inner bounding spheroid. Vertex is outside outer spheroid.",
        ctx,
    )?;

    spicelib::VPACK(
        -((4 as f64) * save.RE),
        0.0,
        (save.RP / 2 as f64),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VPACK(1.0, 0.0, 0.0, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    //
    // Compute the expected intercept. We need to approach the
    // inner ellipsoid from the opposite side, since the vertex
    // and element are on opposite sides of the Z axis.
    //
    spicelib::VEQU(save.VERTEX.as_slice(), save.ENDPT2.as_slice_mut());

    save.ENDPT2[1] = -save.ENDPT2[1];

    spicelib::VMINUS(save.RAYDIR.as_slice(), save.NEGDIR.as_slice_mut());

    spicelib::SURFPT(
        save.ENDPT2.as_slice(),
        save.NEGDIR.as_slice(),
        save.EMIN,
        save.EMIN,
        save.PMIN,
        save.XXPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    // CALL SURFPT ( VERTEX, RAYDIR, EMAX, EMAX, PMAX, XXPT, FOUND )

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"SURFPT found", save.FOUND, true, OK, ctx)?;

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
        b"Ray hits element on inner bounding spheroid. Vertex is inside inner spheroid.",
        ctx,
    )?;

    spicelib::VPACK(0.0, 0.0, (save.RP / 2 as f64), save.VERTEX.as_slice_mut());
    spicelib::VPACK(1.0, 0.0, 0.0, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    //
    // Compute the expected intercept.
    //
    spicelib::SURFPT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.EMIN,
        save.EMIN,
        save.PMIN,
        save.XXPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"SURFPT found", save.FOUND, true, OK, ctx)?;

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

    testutil::TCASE(b"Ray hits element from west side.", ctx)?;

    save.S = (f64::sqrt(2.0) / 2 as f64);

    spicelib::VPACK(
        ((save.S * save.RE) - 10.0),
        -((2 as f64) * save.RE),
        0.0,
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 0.0, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    //
    // Compute the expected intercept.
    //
    //
    // Create the normal vector for the western boundary.
    //
    spicelib::LATREC(1.0, save.BOUNDS[[1, 1]], 0.0, save.VWEST.as_slice_mut());
    spicelib::UCRSS(
        save.VWEST.as_slice(),
        save.Z.as_slice(),
        save.UPLNML.as_slice_mut(),
    );

    //
    // The western plane contains the origin.
    //
    save.CONST = 0.0;

    spicelib::VHAT(save.RAYDIR.as_slice(), save.UDIR.as_slice_mut());

    save.MAXD = ((4 as f64) * save.RE);

    spicelib::ZZINRYPL(
        save.VERTEX.as_slice(),
        save.UDIR.as_slice(),
        save.UPLNML.as_slice(),
        save.CONST,
        save.MAXD,
        &mut save.XNXPTS,
        save.XXPT.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"ZZINRYPL XNXTPS", save.XNXPTS, b"=", 1, 0, OK, ctx)?;

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

    testutil::TCASE(b"Ray hits element from east side.", ctx)?;

    save.S = (f64::sqrt(2.0) / 2 as f64);

    spicelib::VPACK(
        ((save.S * save.RE) - 10.0),
        ((2 as f64) * save.RE),
        0.0,
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VPACK(0.0, -1.0, 0.0, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    //
    // Compute the expected intercept.
    //
    //
    // Create the normal vector for the eastern boundary.
    //
    spicelib::LATREC(1.0, save.BOUNDS[[2, 1]], 0.0, save.VEAST.as_slice_mut());
    spicelib::UCRSS(
        save.Z.as_slice(),
        save.VEAST.as_slice(),
        save.UPLNML.as_slice_mut(),
    );

    //
    // The eastern plane contains the origin.
    //
    save.CONST = 0.0;

    spicelib::VHAT(save.RAYDIR.as_slice(), save.UDIR.as_slice_mut());

    save.MAXD = ((4 as f64) * save.RE);

    spicelib::ZZINRYPL(
        save.VERTEX.as_slice(),
        save.UDIR.as_slice(),
        save.UPLNML.as_slice(),
        save.CONST,
        save.MAXD,
        &mut save.XNXPTS,
        save.XXPT.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"ZZINRYPL XNXTPS", save.XNXPTS, b"=", 1, 0, OK, ctx)?;

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

    testutil::TCASE(b"Ray hits element from north side.", ctx)?;

    spicelib::GEOREC(
        0.0,
        save.BOUNDS[[2, 2]],
        0.0,
        save.RE,
        save.F,
        save.VERTEX.as_slice_mut(),
        ctx,
    )?;

    spicelib::RECGEO(
        save.VERTEX.as_slice(),
        save.RE,
        save.F,
        &mut save.LON,
        &mut save.LAT,
        &mut save.ALT,
        ctx,
    )?;

    spicelib::GEOREC(
        0.0,
        save.BOUNDS[[2, 2]],
        save.BOUNDS[[2, 3]],
        save.RE,
        save.F,
        save.VERTEX.as_slice_mut(),
        ctx,
    )?;

    //
    // Shift the vertex toward the origin in the X direction and
    // raise it above the outer bounding sphere in the Z direction.

    save.VERTEX[1] = (save.VERTEX[1] - 100.0);
    save.VERTEX[3] = ((2 as f64) * save.RE);

    spicelib::VPACK(0.0, 0.0, -1.0, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    //
    // Compute the expected intercept.
    //
    // The intercept should be on the upper latitude cone.
    //
    spicelib::VEQU(save.VERTEX.as_slice(), save.ENDPT1.as_slice_mut());
    spicelib::VEQU(save.ENDPT1.as_slice(), save.ENDPT2.as_slice_mut());

    save.ENDPT2[3] = -save.ENDPT2[3];

    //
    // Caution! The apex of the latitude cone is not the origin.
    // It lies on the -Z axis. Compute the intercept.
    //
    save.ANGLE = (spicelib::HALFPI(ctx) - save.BOUNDS[[2, 2]]);

    spicelib::ZZELNAXX(
        save.RE,
        save.RP,
        save.LAT,
        &mut save.XINCPT,
        &mut save.YINCPT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.ORIGIN.as_slice(), save.APEX.as_slice_mut());
    save.APEX[3] = save.YINCPT;

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.Z.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.XNXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKSI(b"INCNSG XNXPTS", save.XNXPTS, b"=", 1, 0, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"XPT",
        save.XPT.as_slice(),
        b"~~/",
        save.XPT1.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Ray hits element from south side.", ctx)?;

    spicelib::GEOREC(
        0.0,
        save.BOUNDS[[1, 2]],
        save.BOUNDS[[2, 3]],
        save.RE,
        save.F,
        save.VERTEX.as_slice_mut(),
        ctx,
    )?;

    //
    // Shift the vertex toward the origin in the X direction and
    // lower it below the outer bounding sphere in the Z direction.

    save.VERTEX[1] = (save.VERTEX[1] - 100.0);
    save.VERTEX[3] = -((2 as f64) * save.RE);

    spicelib::VPACK(0.0, 0.0, 1.0, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    //
    // Compute the expected intercept.
    //
    // The intercept should be on the lower latitude cone.
    //
    spicelib::VEQU(save.VERTEX.as_slice(), save.ENDPT1.as_slice_mut());
    spicelib::VEQU(save.ENDPT1.as_slice(), save.ENDPT2.as_slice_mut());

    save.ENDPT2[3] = -save.ENDPT2[3];

    //
    // Caution! The apex of the latitude cone is not the origin.
    // It lies on the +Z axis. Compute the intercept.
    //
    save.ANGLE = (spicelib::HALFPI(ctx) - save.BOUNDS[[1, 2]]);

    save.LAT = save.BOUNDS[[1, 2]];

    spicelib::ZZELNAXX(
        save.RE,
        save.RP,
        save.LAT,
        &mut save.XINCPT,
        &mut save.YINCPT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.ORIGIN.as_slice(), save.APEX.as_slice_mut());
    save.APEX[3] = save.YINCPT;

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.Z.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.XNXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKSI(b"INCNSG XNXPTS", save.XNXPTS, b"=", 1, 0, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"XPT",
        save.XPT.as_slice(),
        b"~~/",
        save.XPT1.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // In the following cases, we change the latitude bounds of
    // the element.
    //

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(
        b"North element boundary is at 0 latitude. Ray hits element from north side.",
        ctx,
    )?;

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 2]] = 0.0;

    //
    // Find the point on the reference ellipsoid at maximum altitude
    // and latitude, longitude zero.
    //
    spicelib::GEOREC(
        0.0,
        save.BOUNDS[[2, 2]],
        save.BOUNDS[[2, 3]],
        save.RE,
        save.F,
        save.VERTEX.as_slice_mut(),
        ctx,
    )?;

    //
    // Shift the vertex toward the origin in the X direction and
    // raise it above the outer bounding sphere in the Z direction.

    save.VERTEX[1] = (save.VERTEX[1] - 100.0);
    save.VERTEX[3] = ((2 as f64) * save.RE);

    spicelib::VPACK(0.0, 0.0, -1.0, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    //
    // Compute the expected intercept.
    //
    // The intercept should be on the "upper latitude cone."
    //
    spicelib::VEQU(save.VERTEX.as_slice(), save.ENDPT1.as_slice_mut());
    spicelib::VEQU(save.ENDPT1.as_slice(), save.ENDPT2.as_slice_mut());

    save.ENDPT2[3] = -save.ENDPT2[3];

    //
    // Caution! The apex of the latitude cone usually is not the origin.
    // It usually lies on the -Z axis. However, in this case the apex
    // should be the origin. Compute the intercept by the usual means.
    //
    save.ANGLE = (spicelib::HALFPI(ctx) - save.BOUNDS[[2, 2]]);

    save.LAT = save.BOUNDS[[2, 2]];

    spicelib::ZZELNAXX(
        save.RE,
        save.RP,
        save.LAT,
        &mut save.XINCPT,
        &mut save.YINCPT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.ORIGIN.as_slice(), save.APEX.as_slice_mut());
    save.APEX[3] = save.YINCPT;

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.Z.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.XNXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKSI(b"INCNSG XNXPTS", save.XNXPTS, b"=", 1, 0, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"XPT",
        save.XPT.as_slice(),
        b"~~/",
        save.XPT1.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(
        b"North element boundary is at negative latitude. Ray hits element from north side.",
        ctx,
    )?;

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 2]] = -(spicelib::PI(ctx) / 6 as f64);

    //
    // Find the point on the reference ellipsoid at maximum altitude
    // and latitude, longitude zero.
    //
    spicelib::GEOREC(
        0.0,
        save.BOUNDS[[2, 2]],
        save.BOUNDS[[2, 3]],
        save.RE,
        save.F,
        save.VERTEX.as_slice_mut(),
        ctx,
    )?;

    //
    // Shift the vertex toward the origin in the X direction and
    // raise it above the outer bounding sphere in the Z direction.

    save.VERTEX[1] = (save.VERTEX[1] - 100.0);
    save.VERTEX[3] = ((2 as f64) * save.RE);

    spicelib::VPACK(0.0, 0.0, -1.0, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    //
    // Compute the expected intercept.
    //
    // The intercept should be on the "upper latitude cone."
    //
    spicelib::VEQU(save.VERTEX.as_slice(), save.ENDPT1.as_slice_mut());
    spicelib::VEQU(save.ENDPT1.as_slice(), save.ENDPT2.as_slice_mut());

    save.ENDPT2[3] = -save.ENDPT2[3];

    //
    // Caution! The apex of the latitude cone usually is not the origin.
    // It usually lies on the -Z axis. However, in this case the apex
    // should be the origin. Compute the intercept by the usual means.
    //
    save.ANGLE = (spicelib::HALFPI(ctx) - save.BOUNDS[[2, 2]]);

    save.LAT = save.BOUNDS[[2, 2]];

    spicelib::ZZELNAXX(
        save.RE,
        save.RP,
        save.LAT,
        &mut save.XINCPT,
        &mut save.YINCPT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.ORIGIN.as_slice(), save.APEX.as_slice_mut());
    save.APEX[3] = save.YINCPT;

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.Z.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.XNXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKSI(b"INCNSG XNXPTS", save.XNXPTS, b"=", 1, 0, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"XPT",
        save.XPT.as_slice(),
        b"~~/",
        save.XPT1.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(
        b"South element boundary is at 0 latitude. Ray hits element from south side.",
        ctx,
    )?;

    save.BOUNDS[[1, 2]] = 0.0;
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 3 as f64);

    //
    // Find the point on the reference ellipsoid at maximum altitude
    // and minimum latitude, longitude zero.
    //
    spicelib::GEOREC(
        0.0,
        save.BOUNDS[[1, 2]],
        save.BOUNDS[[2, 3]],
        save.RE,
        save.F,
        save.VERTEX.as_slice_mut(),
        ctx,
    )?;

    //
    // Shift the vertex toward the origin in the X direction and
    // lower it below the outer bounding sphere in the -Z direction.

    save.VERTEX[1] = (save.VERTEX[1] - 100.0);
    save.VERTEX[3] = -((2 as f64) * save.RE);

    spicelib::VPACK(0.0, 0.0, 1.0, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    //
    // Compute the expected intercept.
    //
    // The intercept should be on the "lower latitude cone."
    //
    spicelib::VEQU(save.VERTEX.as_slice(), save.ENDPT1.as_slice_mut());
    spicelib::VEQU(save.ENDPT1.as_slice(), save.ENDPT2.as_slice_mut());

    save.ENDPT2[3] = -save.ENDPT2[3];

    //
    // Caution! The apex of the latitude cone usually is not the origin.
    // It usually lies on the +Z axis (for cones of negative latitude).
    // However, in this case the apex should be the origin. Compute
    // the intercept by the usual means.
    //
    save.ANGLE = (spicelib::HALFPI(ctx) - save.BOUNDS[[1, 2]]);

    save.LAT = save.BOUNDS[[1, 2]];

    spicelib::ZZELNAXX(
        save.RE,
        save.RP,
        save.LAT,
        &mut save.XINCPT,
        &mut save.YINCPT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.ORIGIN.as_slice(), save.APEX.as_slice_mut());
    save.APEX[3] = save.YINCPT;

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.Z.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.XNXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKSI(b"INCNSG XNXPTS", save.XNXPTS, b"=", 1, 0, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"XPT",
        save.XPT.as_slice(),
        b"~~/",
        save.XPT1.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(
        b"South element boundary is at negative latitude. Ray hits element from south side.",
        ctx,
    )?;

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 3 as f64);

    //
    // Find the point on the reference ellipsoid at maximum altitude
    // and minimum latitude, longitude zero.
    //
    spicelib::GEOREC(
        0.0,
        save.BOUNDS[[1, 2]],
        save.BOUNDS[[2, 3]],
        save.RE,
        save.F,
        save.VERTEX.as_slice_mut(),
        ctx,
    )?;

    //
    // Shift the vertex toward the origin in the X direction and
    // lower it below the outer bounding sphere in the -Z direction.

    save.VERTEX[1] = (save.VERTEX[1] - 100.0);
    save.VERTEX[3] = -((2 as f64) * save.RE);

    spicelib::VPACK(0.0, 0.0, 1.0, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    //
    // Compute the expected intercept.
    //
    // The intercept should be on the lower latitude cone.
    //
    spicelib::VEQU(save.VERTEX.as_slice(), save.ENDPT1.as_slice_mut());
    spicelib::VEQU(save.ENDPT1.as_slice(), save.ENDPT2.as_slice_mut());

    save.ENDPT2[3] = -save.ENDPT2[3];

    //
    // Caution! The apex of the latitude cone usually is not the origin.
    // It usually lies on the +Z axis (for cones of negative latitude).
    // However, in this case the apex should be the origin. Compute
    // the intercept by the usual means.
    //
    save.ANGLE = (spicelib::HALFPI(ctx) - save.BOUNDS[[1, 2]]);

    save.LAT = save.BOUNDS[[1, 2]];

    spicelib::ZZELNAXX(
        save.RE,
        save.RP,
        save.LAT,
        &mut save.XINCPT,
        &mut save.YINCPT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.ORIGIN.as_slice(), save.APEX.as_slice_mut());
    save.APEX[3] = save.YINCPT;

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.Z.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.XNXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKSI(b"INCNSG XNXPTS", save.XNXPTS, b"=", 1, 0, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"XPT",
        save.XPT.as_slice(),
        b"~~/",
        save.XPT1.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The following latitude boundary intercept cases test the handling
    // of multiple roots in the ray-cone intercept computation. In these
    // cases, the second root should be chosen.
    //

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(
        b"North element boundary is at positive latitude. Ray hits element from the -X direction.",
        ctx,
    )?;

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 3 as f64);

    //
    // Find the point on the reference ellipsoid at maximum altitude
    // and maximum latitude, longitude zero.
    //
    spicelib::GEOREC(
        0.0,
        save.BOUNDS[[2, 2]],
        save.BOUNDS[[2, 3]],
        save.RE,
        save.F,
        save.VERTEX.as_slice_mut(),
        ctx,
    )?;

    //
    // Shift the vertex in the -X direction. and lower it below the outer
    // bounding sphere in the -Z direction.
    //
    save.VERTEX[1] = (save.VERTEX[1] - ((10 as f64) * save.RE));
    save.VERTEX[3] = (save.VERTEX[3] - 10.0);

    //
    // This ray should hit the upper latitude cone twice. The second
    // intercept is the one we want.
    //
    spicelib::VPACK(1.0, 0.0, 0.0, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    //
    // Compute the expected intercept.
    //
    // The intercept should be on the upper latitude cone.
    //
    spicelib::VEQU(save.VERTEX.as_slice(), save.ENDPT1.as_slice_mut());
    spicelib::VEQU(save.ENDPT1.as_slice(), save.ENDPT2.as_slice_mut());

    save.ENDPT2[1] = -save.ENDPT2[1];

    save.ANGLE = (spicelib::HALFPI(ctx) - save.BOUNDS[[2, 2]]);

    save.LAT = save.BOUNDS[[2, 2]];

    //
    // The cone's apex should be on the -Z axis.
    //
    spicelib::ZZELNAXX(
        save.RE,
        save.RP,
        save.LAT,
        &mut save.XINCPT,
        &mut save.YINCPT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.ORIGIN.as_slice(), save.APEX.as_slice_mut());
    save.APEX[3] = save.YINCPT;

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.Z.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.XNXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;

    //
    // Note: we expect two roots. The second one is the correct one.
    //
    testutil::CHCKSI(b"INCNSG XNXPTS", save.XNXPTS, b"=", 2, 0, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"XPT",
        save.XPT.as_slice(),
        b"~~/",
        save.XPT2.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(
        b"South element boundary is at negative latitude. Ray hits element from the -X direction.",
        ctx,
    )?;

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 3 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 3 as f64);

    //
    // Find the point on the reference ellipsoid at maximum altitude
    // and minimum latitude, longitude zero.
    //
    spicelib::GEOREC(
        0.0,
        save.BOUNDS[[1, 2]],
        save.BOUNDS[[2, 3]],
        save.RE,
        save.F,
        save.VERTEX.as_slice_mut(),
        ctx,
    )?;

    //
    // Shift the vertex in the -X direction. and raise it below the outer
    // bounding sphere in the +Z direction.
    //
    save.VERTEX[1] = (save.VERTEX[1] - ((10 as f64) * save.RE));
    save.VERTEX[3] = (save.VERTEX[3] + 10.0);

    //
    // This ray should hit the lower latitude cone twice. The second
    // intercept is the one we want.
    //
    spicelib::VPACK(1.0, 0.0, 0.0, save.RAYDIR.as_slice_mut());

    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    //
    // Compute the expected intercept.
    //
    // The intercept should be on the lower latitude cone.
    //
    spicelib::VEQU(save.VERTEX.as_slice(), save.ENDPT1.as_slice_mut());
    spicelib::VEQU(save.ENDPT1.as_slice(), save.ENDPT2.as_slice_mut());

    save.ENDPT2[1] = -save.ENDPT2[1];

    save.ANGLE = (spicelib::HALFPI(ctx) - save.BOUNDS[[1, 2]]);

    save.LAT = save.BOUNDS[[1, 2]];

    //
    // The cone's apex should be on the +Z axis.
    //
    spicelib::ZZELNAXX(
        save.RE,
        save.RP,
        save.LAT,
        &mut save.XINCPT,
        &mut save.YINCPT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.ORIGIN.as_slice(), save.APEX.as_slice_mut());
    save.APEX[3] = save.YINCPT;

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.Z.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.XNXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;

    //
    // Note: we expect two roots. The second one is the correct one.
    //
    testutil::CHCKSI(b"INCNSG XNXPTS", save.XNXPTS, b"=", 2, 0, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"XPT",
        save.XPT.as_slice(),
        b"~~/",
        save.XPT2.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(
        b"Oblate case: ray hits positive latitude cone below X-Y plane. No valid roots.",
        ctx,
    )?;

    //
    // In the following case, the ray hits a latitude cone at
    // a point having longitude and altitude within the element's
    // bounds, but the intercept is nevertheless outside the element.
    //
    // This type of case should not arise if we have a planetodetic
    // volume element boundary created using DSKW02, because that
    // routine limits the lower altitude. However, it is possible to
    // pass to ZZRYTPDT inputs that define such a case, and
    // we want to make sure that it's handled properly.
    //
    // Use a highly eccentric spheroid.
    //

    save.RE = 3000.0;
    save.F = 0.8;

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);

    save.BOUNDS[[1, 2]] = 0.0;
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 8 as f64);
    //
    // Create a point on the spheroid having maximum latitude.
    //
    save.LAT = save.BOUNDS[[2, 2]];

    spicelib::GEOREC(
        0.0,
        save.LAT,
        0.0,
        save.RE,
        save.F,
        save.P.as_slice_mut(),
        ctx,
    )?;

    save.A = save.RE;
    save.B = (save.RE * (1.0 - save.F));

    spicelib::ZZELNAXX(
        save.A,
        save.B,
        save.LAT,
        &mut save.XINCPT,
        &mut save.YINCPT,
        ctx,
    )?;

    spicelib::VPACK(0.0, 0.0, save.YINCPT, save.APEX.as_slice_mut());

    spicelib::VPACK(save.XINCPT, 0.0, 0.0, save.XYX.as_slice_mut());

    //
    // Pick a lower altitude bound that will get past ZZELLBDS.
    //
    save.BOUNDS[[1, 3]] = (-(f64::powi(save.B, 2) / save.A) + 1.0);
    save.BOUNDS[[2, 3]] = 0.0;

    //
    // Consider the vector from the cone's apex to P. We'll pick a
    // target point on this vector, such that the target has negative Z
    // component. The target's altitude is within the element's altitude
    // bounds, because the altitude is measured from the surface on the
    // -Z side.
    //
    save.COLAT = (spicelib::HALFPI(ctx) - save.LAT);

    save.H = (-save.B - (save.BOUNDS[[1, 3]] / 2 as f64));

    spicelib::VPACK(
        -(f64::tan(save.COLAT) * (save.YINCPT - save.H)),
        0.0,
        save.H,
        save.TARGET.as_slice_mut(),
    );

    //
    // Check target's off-axis angle: the target should lie on the cone.
    //
    spicelib::VSUB(
        save.TARGET.as_slice(),
        save.APEX.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    //
    // Create a horizontal ray that contacts the cone at TARGET.
    //
    spicelib::VPACK(
        ((2 as f64) * save.A),
        0.0,
        save.H,
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VPACK(-1.0, 0.0, 0.0, save.RAYDIR.as_slice_mut());

    //
    // Find the intercept of the ray with the boundary of the
    // volume element. The ray will hit the upper latitude cone
    // below the X-Y plane. This should be considered a non-intercept
    // case.
    //
    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
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

    testutil::TCASE(b"Oblate case: ray hits positive latitude cone below X-Y plane, then above X-Y plane. First root is invalid; second root is valid.", ctx)?;

    //
    // In the following case, the ray hits a latitude cone at
    // a point having longitude and altitude within the element's
    // bounds, but the intercept is nevertheless outside the element.
    //
    // This type of case should not arise if we have a planetodetic
    // volume element boundary created using DSKW02, because that
    // routine limits the lower altitude. However, it is possible to
    // pass to ZZRYTPDT inputs that define such a case, and
    // we want to make sure that it's handled properly.
    //
    // Use a highly eccentric spheroid.
    //

    save.RE = 3000.0;
    save.F = 0.8;

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);

    save.BOUNDS[[1, 2]] = 0.0;
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 8 as f64);
    //
    // Create a point on the spheroid having maximum latitude.
    //
    save.LAT = save.BOUNDS[[2, 2]];

    spicelib::GEOREC(
        0.0,
        save.LAT,
        0.0,
        save.RE,
        save.F,
        save.P.as_slice_mut(),
        ctx,
    )?;

    save.A = save.RE;
    save.B = (save.RE * (1.0 - save.F));

    spicelib::ZZELNAXX(
        save.A,
        save.B,
        save.LAT,
        &mut save.XINCPT,
        &mut save.YINCPT,
        ctx,
    )?;

    spicelib::VPACK(0.0, 0.0, save.YINCPT, save.APEX.as_slice_mut());

    spicelib::VPACK(save.XINCPT, 0.0, 0.0, save.XYX.as_slice_mut());

    //
    // Pick a lower altitude bound that will get past ZZELLBDS.
    //
    save.BOUNDS[[1, 3]] = (-(f64::powi(save.B, 2) / save.A) + 1.0);
    save.BOUNDS[[2, 3]] = 100.0;

    //
    // Consider the vector from the cone's apex to P. We'll pick a
    // target point on this vector, such that the target has negative Z
    // component. The target's altitude is within the element's altitude
    // bounds, because the altitude is measured from the surface on the
    // -Z side.
    //
    save.COLAT = (spicelib::HALFPI(ctx) - save.LAT);

    save.H = (-save.B - (save.BOUNDS[[1, 3]] / 2 as f64));

    spicelib::VPACK(
        -(f64::tan(save.COLAT) * (save.YINCPT - save.H)),
        0.0,
        save.H,
        save.TARGET.as_slice_mut(),
    );

    //
    // Create a second target point that does have latitude within
    // bounds.
    //
    save.H2 = (save.B / 8 as f64);

    spicelib::VPACK(
        -(f64::tan(save.COLAT) * (save.H2 - save.YINCPT)),
        0.0,
        save.H2,
        save.TARGT2.as_slice_mut(),
    );

    //
    // Check second target's off-axis angle: the target should lie on
    // the cone.
    //
    spicelib::VSUB(
        save.TARGT2.as_slice(),
        save.APEX.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    //
    // Create a ray that contacts the cone first at TARGET, then at
    // TARGT2.
    //
    spicelib::VSUB(
        save.TARGT2.as_slice(),
        save.TARGET.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    spicelib::VSUB(
        save.TARGET.as_slice(),
        save.RAYDIR.as_slice(),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VHATIP(save.RAYDIR.as_slice_mut());

    //
    // Find the intercept of the ray with the boundary of the
    // volume element. The ray will hit the upper latitude cone
    // below the X-Y plane. This should be considered a non-intercept
    // case.
    //
    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    //
    // The intercept that was found should be the second target.
    //

    if *OK {
        testutil::CHCKAD(
            b"XPT",
            save.XPT.as_slice(),
            b"~~/",
            save.TARGT2.as_slice(),
            3,
            TIGHT,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(
        b"Oblate case: ray hits negative latitude cone above X-Y plane. No valid roots.",
        ctx,
    )?;

    //
    // In the following case, the ray hits a latitude cone at
    // a point having longitude and altitude within the element's
    // bounds, but the intercept is nevertheless outside the element.
    //
    // This type of case should not arise if we have a planetodetic
    // volume element boundary created using DSKW02, because that
    // routine limits the lower altitude. However, it is possible to
    // pass to ZZRYTPDT inputs that define such a case, and
    // we want to make sure that it's handled properly.
    //
    // Use a highly eccentric spheroid.
    //

    save.RE = 3000.0;
    save.F = 0.8;

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 8 as f64);
    save.BOUNDS[[2, 2]] = 0.0;

    //
    // Create a point on the spheroid having minimum latitude.
    //
    save.LAT = save.BOUNDS[[1, 2]];

    spicelib::GEOREC(
        0.0,
        save.LAT,
        0.0,
        save.RE,
        save.F,
        save.P.as_slice_mut(),
        ctx,
    )?;

    save.A = save.RE;
    save.B = (save.RE * (1.0 - save.F));

    spicelib::ZZELNAXX(
        save.A,
        save.B,
        save.LAT,
        &mut save.XINCPT,
        &mut save.YINCPT,
        ctx,
    )?;

    spicelib::VPACK(0.0, 0.0, save.YINCPT, save.APEX.as_slice_mut());

    spicelib::VPACK(save.XINCPT, 0.0, 0.0, save.XYX.as_slice_mut());

    //
    // Pick a lower altitude bound that will get past ZZELLBDS.
    //
    save.BOUNDS[[1, 3]] = (-(f64::powi(save.B, 2) / save.A) + 1.0);
    save.BOUNDS[[2, 3]] = 100.0;

    //
    // Consider the vector from the cone's apex to P. We'll pick a
    // target point on this vector, such that the target has positive Z
    // component. The target's altitude is within the element's altitude
    // bounds, because the altitude is measured from the surface on the
    // +Z side.
    //
    save.COLAT = (spicelib::HALFPI(ctx) - save.LAT);

    save.H = (save.B - (save.BOUNDS[[1, 3]] / 2 as f64));

    spicelib::VPACK(
        -(f64::tan(save.COLAT) * (save.YINCPT - save.H)),
        0.0,
        save.H,
        save.TARGET.as_slice_mut(),
    );

    //
    // Check target's off-axis angle: the target should lie on the cone.
    //
    spicelib::VSUB(
        save.TARGET.as_slice(),
        save.APEX.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    //
    // Create a horizontal ray that contacts the cone at TARGET.
    //
    spicelib::VPACK(
        ((2 as f64) * save.A),
        0.0,
        save.H,
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VPACK(-1.0, 0.0, 0.0, save.RAYDIR.as_slice_mut());

    //
    // Find the intercept of the ray with the boundary of the
    // volume element. The ray will hit the upper latitude cone
    // below the X-Y plane. This should be considered a non-intercept
    // case.
    //
    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
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

    testutil::TCASE(b"Oblate case: ray hits negative latitude cone above X-Y plane, then below X-Y plane. First root is invalid; second root is valid.", ctx)?;

    //
    // In the following case, the ray hits a latitude cone at
    // a point having longitude and altitude within the element's
    // bounds, but the intercept is nevertheless outside the element.
    //
    // This type of case should not arise if we have a planetodetic
    // volume element boundary created using DSKW02, because that
    // routine limits the lower altitude. However, it is possible to
    // pass to ZZRYTPDT inputs that define such a case, and
    // we want to make sure that it's handled properly.
    //
    // Use a highly eccentric spheroid.
    //

    save.RE = 3000.0;
    save.F = 0.8;

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 8 as f64);
    save.BOUNDS[[2, 2]] = 0.0;
    //
    // Create a point on the spheroid having minimum latitude.
    //
    save.LAT = save.BOUNDS[[1, 2]];

    spicelib::GEOREC(
        0.0,
        save.LAT,
        0.0,
        save.RE,
        save.F,
        save.P.as_slice_mut(),
        ctx,
    )?;

    save.A = save.RE;
    save.B = (save.RE * (1.0 - save.F));

    spicelib::ZZELNAXX(
        save.A,
        save.B,
        save.LAT,
        &mut save.XINCPT,
        &mut save.YINCPT,
        ctx,
    )?;

    spicelib::VPACK(0.0, 0.0, save.YINCPT, save.APEX.as_slice_mut());

    spicelib::VPACK(save.XINCPT, 0.0, 0.0, save.XYX.as_slice_mut());

    //
    // Pick a lower altitude bound that will get past ZZELLBDS.
    //
    save.BOUNDS[[1, 3]] = (-(f64::powi(save.B, 2) / save.A) + 1.0);
    save.BOUNDS[[2, 3]] = 100.0;

    //
    // Consider the vector from the cone's apex to P. We'll pick a
    // target point on this vector, such that the target has negative Z
    // component. The target's altitude is within the element's altitude
    // bounds, because the altitude is measured from the surface on the
    // -Z side.
    //
    save.COLAT = (spicelib::HALFPI(ctx) - save.LAT);

    save.H = (save.B - (save.BOUNDS[[1, 3]] / 2 as f64));

    spicelib::VPACK(
        -(f64::tan(save.COLAT) * (save.YINCPT - save.H)),
        0.0,
        save.H,
        save.TARGET.as_slice_mut(),
    );

    //
    // Create a second target point that does have latitude within
    // bounds.
    //
    save.H2 = -(save.B / 8 as f64);

    spicelib::VPACK(
        (f64::tan(save.COLAT) * (save.YINCPT - save.H2)),
        0.0,
        save.H2,
        save.TARGT2.as_slice_mut(),
    );

    //
    // Check second target's off-axis angle: the target should lie on
    // the cone.
    //
    spicelib::VSUB(
        save.TARGT2.as_slice(),
        save.APEX.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    //
    // Create a ray that contacts the cone first at TARGET, then at
    // TARGT2.
    //
    spicelib::VSUB(
        save.TARGT2.as_slice(),
        save.TARGET.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );
    spicelib::VSUB(
        save.TARGET.as_slice(),
        save.RAYDIR.as_slice(),
        save.VERTEX.as_slice_mut(),
    );

    spicelib::VHATIP(save.RAYDIR.as_slice_mut());

    //
    // Find the intercept of the ray with the boundary of the
    // volume element. The ray will hit the upper latitude cone
    // below the X-Y plane. This should be considered a non-intercept
    // case.
    //
    spicelib::ZZRYTPDT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        &mut save.NXPTS,
        save.XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    //
    // The intercept that was found should be the second target.
    //

    if *OK {
        testutil::CHCKAD(
            b"XPT",
            save.XPT.as_slice(),
            b"~~/",
            save.TARGT2.as_slice(),
            3,
            TIGHT,
            OK,
            ctx,
        )?;
    }

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
