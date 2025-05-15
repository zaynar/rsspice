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
const NONE: i32 = 0;
const LONIDX: i32 = 1;
const LATIDX: i32 = 2;
const ALTIDX: i32 = 3;
const LATMRG: f64 = 0.00000001;
const PRVMRG: f64 = 0.000001;
const NLAT: i32 = 6;
const NLON: i32 = 9;
const NELL: i32 = 3;
const TITLSZ: i32 = 300;

struct SaveVars {
    STEM: Vec<u8>,
    TITLE: Vec<u8>,
    A: f64,
    AEPS: f64,
    ALT: f64,
    B: f64,
    BOUNDS: StackArray2D<f64, 6>,
    CORPAR: StackArray<f64, 10>,
    DIR: StackArray<f64, 3>,
    EPS: f64,
    EQHI: f64,
    EQLOW: f64,
    EQRAD: StackArray<f64, 3>,
    F: f64,
    HIALT: f64,
    HIGHPT: StackArray<f64, 3>,
    HILAT: f64,
    HILON: f64,
    LAT: f64,
    LON: f64,
    LOWALT: f64,
    LOWLAT: f64,
    LOWLON: f64,
    LOWPT: StackArray<f64, 3>,
    MARGIN: f64,
    MAXALT: StackArray<f64, 3>,
    MAXLAT: StackArray<f64, 6>,
    MAXLON: StackArray<f64, 9>,
    MIDLAT: f64,
    MIDLON: f64,
    MIDALT: f64,
    MINALT: StackArray<f64, 3>,
    MINLAT: StackArray<f64, 6>,
    MINLON: StackArray<f64, 9>,
    MODBDS: StackArray2D<f64, 6>,
    NORMAL: StackArray<f64, 3>,
    NRMMAX: f64,
    NRMMIN: f64,
    P: StackArray<f64, 3>,
    P2: StackArray<f64, 3>,
    P3: StackArray<f64, 3>,
    POLHI: f64,
    POLLOW: f64,
    POLRAD: StackArray<f64, 3>,
    POSMRG: f64,
    RE: f64,
    TOL: f64,
    VERTEX: StackArray<f64, 3>,
    EXCLUD: i32,
    I: i32,
    FIRST: bool,
    FOUND: bool,
    INSIDE: bool,
    LATLB: bool,
    LATUB: bool,
    LONBDS: bool,
    XIN: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut STEM = vec![b' '; TITLSZ as usize];
        let mut TITLE = vec![b' '; TITLSZ as usize];
        let mut A: f64 = 0.0;
        let mut AEPS: f64 = 0.0;
        let mut ALT: f64 = 0.0;
        let mut B: f64 = 0.0;
        let mut BOUNDS = StackArray2D::<f64, 6>::new(1..=2, 1..=3);
        let mut CORPAR = StackArray::<f64, 10>::new(1..=NSYPAR);
        let mut DIR = StackArray::<f64, 3>::new(1..=3);
        let mut EPS: f64 = 0.0;
        let mut EQHI: f64 = 0.0;
        let mut EQLOW: f64 = 0.0;
        let mut EQRAD = StackArray::<f64, 3>::new(1..=NELL);
        let mut F: f64 = 0.0;
        let mut HIALT: f64 = 0.0;
        let mut HIGHPT = StackArray::<f64, 3>::new(1..=3);
        let mut HILAT: f64 = 0.0;
        let mut HILON: f64 = 0.0;
        let mut LAT: f64 = 0.0;
        let mut LON: f64 = 0.0;
        let mut LOWALT: f64 = 0.0;
        let mut LOWLAT: f64 = 0.0;
        let mut LOWLON: f64 = 0.0;
        let mut LOWPT = StackArray::<f64, 3>::new(1..=3);
        let mut MARGIN: f64 = 0.0;
        let mut MAXALT = StackArray::<f64, 3>::new(1..=NELL);
        let mut MAXLAT = StackArray::<f64, 6>::new(1..=NLAT);
        let mut MAXLON = StackArray::<f64, 9>::new(1..=NLON);
        let mut MIDLAT: f64 = 0.0;
        let mut MIDLON: f64 = 0.0;
        let mut MIDALT: f64 = 0.0;
        let mut MINALT = StackArray::<f64, 3>::new(1..=NELL);
        let mut MINLAT = StackArray::<f64, 6>::new(1..=NLAT);
        let mut MINLON = StackArray::<f64, 9>::new(1..=NLON);
        let mut MODBDS = StackArray2D::<f64, 6>::new(1..=2, 1..=3);
        let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
        let mut NRMMAX: f64 = 0.0;
        let mut NRMMIN: f64 = 0.0;
        let mut P = StackArray::<f64, 3>::new(1..=3);
        let mut P2 = StackArray::<f64, 3>::new(1..=3);
        let mut P3 = StackArray::<f64, 3>::new(1..=3);
        let mut POLHI: f64 = 0.0;
        let mut POLLOW: f64 = 0.0;
        let mut POLRAD = StackArray::<f64, 3>::new(1..=NELL);
        let mut POSMRG: f64 = 0.0;
        let mut RE: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut VERTEX = StackArray::<f64, 3>::new(1..=3);
        let mut EXCLUD: i32 = 0;
        let mut I: i32 = 0;
        let mut FIRST: bool = false;
        let mut FOUND: bool = false;
        let mut INSIDE: bool = false;
        let mut LATLB: bool = false;
        let mut LATUB: bool = false;
        let mut LONBDS: bool = false;
        let mut XIN: bool = false;

        FIRST = true;
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(-90.0),
                Val::D(-89.999999),
                Val::D(-45.0),
                Val::D(0.0),
                Val::D(45.0),
                Val::D(89.999999),
            ]
            .into_iter();
            MINLAT
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(-89.999999),
                Val::D(-45.0),
                Val::D(0.0),
                Val::D(45.0),
                Val::D(89.999999),
                Val::D(90.0),
            ]
            .into_iter();
            MAXLAT
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(-10.0),
                Val::D(-10.0),
                Val::D(-180.0),
                Val::D(-360.0),
                Val::D(10.0),
                Val::D(179.999999),
                Val::D(-179.999999),
                Val::D(-260.0),
                Val::D(350.0),
            ]
            .into_iter();
            MINLON
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(-5.0),
                Val::D(20.0),
                Val::D(180.0),
                Val::D(0.0),
                Val::D(-10.0),
                Val::D(-179.999999),
                Val::D(179.999999),
                Val::D(200.0),
                Val::D(-350.0),
            ]
            .into_iter();
            MAXLON
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(-200.0), Val::D(-150.0), Val::D(200.0)].into_iter();
            MINALT
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(-100.0), Val::D(50.0), Val::D(300.0)].into_iter();
            MAXALT
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(10000.0), Val::D(8000.0), Val::D(20000.0)].into_iter();
            EQRAD
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(8000.0), Val::D(10000.0), Val::D(5000.0)].into_iter();
            POLRAD
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            STEM,
            TITLE,
            A,
            AEPS,
            ALT,
            B,
            BOUNDS,
            CORPAR,
            DIR,
            EPS,
            EQHI,
            EQLOW,
            EQRAD,
            F,
            HIALT,
            HIGHPT,
            HILAT,
            HILON,
            LAT,
            LON,
            LOWALT,
            LOWLAT,
            LOWLON,
            LOWPT,
            MARGIN,
            MAXALT,
            MAXLAT,
            MAXLON,
            MIDLAT,
            MIDLON,
            MIDALT,
            MINALT,
            MINLAT,
            MINLON,
            MODBDS,
            NORMAL,
            NRMMAX,
            NRMMIN,
            P,
            P2,
            P3,
            POLHI,
            POLLOW,
            POLRAD,
            POSMRG,
            RE,
            TOL,
            VERTEX,
            EXCLUD,
            I,
            FIRST,
            FOUND,
            INSIDE,
            LATLB,
            LATUB,
            LONBDS,
            XIN,
        }
    }
}

//$Procedure F_ZZINPDT ( ZZINPDT tests )
pub fn F_ZZINPDT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    // Set the initial angular bounds in degrees, since we can
    // do this without function calls. The bounds will be
    // converted to radians at run time.
    //

    //
    // For the latitude boundaries, every valid combination of
    // minimum and maximum will be tested.
    //

    //
    // For the longitude boundaries, each pair of bounds
    // consisting of the Ith minimum and Ith maximum will
    // be tested.
    //

    //
    // For the altitude boundaries, there is one valid combination of
    // minimum and maximum for each shape.
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_ZZINPDT", ctx)?;

    //***********************************************************************
    //
    //     Setup
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Initialize coordinate bounds", ctx)?;

    //
    // Convert angular bounds to radians.
    //

    //
    // First pass only!
    //
    if save.FIRST {
        {
            let m1__: i32 = 1;
            let m2__: i32 = NLAT;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.MINLAT[save.I] = (save.MINLAT[save.I] * spicelib::RPD(ctx));
                save.MAXLAT[save.I] = (save.MAXLAT[save.I] * spicelib::RPD(ctx));

                save.I += m3__;
            }
        }

        {
            let m1__: i32 = 1;
            let m2__: i32 = NLON;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.MINLON[save.I] = (save.MINLON[save.I] * spicelib::RPD(ctx));
                save.MAXLON[save.I] = (save.MAXLON[save.I] * spicelib::RPD(ctx));

                save.I += m3__;
            }
        }

        save.FIRST = false;
    }

    //
    // Initialize CORPAR.
    //
    spicelib::CLEARD(NSYPAR, save.CORPAR.as_slice_mut());

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

    spicelib::VPACK(3010.0, 3020.0, 2800.0, save.P.as_slice_mut());

    save.MARGIN = -0.000001;

    save.EXCLUD = NONE;

    spicelib::ZZINPDT(
        save.P.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error: invalid value of EXCLUD", ctx)?;

    save.MARGIN = 0.000001;

    save.EXCLUD = -1;

    spicelib::ZZINPDT(
        save.P.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    save.EXCLUD = 4;

    spicelib::ZZINPDT(
        save.P.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error: longitude bounds out of range.", ctx)?;

    save.MARGIN = 0.000001;

    save.EXCLUD = ALTIDX;

    save.BOUNDS[[1, 1]] = -((3 as f64) * spicelib::PI(ctx));
    save.BOUNDS[[2, 1]] = -spicelib::TWOPI(ctx);

    spicelib::ZZINPDT(
        save.P.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    spicelib::ZZINPDT(
        save.P.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //***********************************************************************
    //
    //
    //     Special non-error cases
    //
    //
    //***********************************************************************

    //
    // Test special logic for positions near the poles.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Segment includes north pole. Point\'s longitude is out of range; altitude is in range.",
        ctx,
    )?;

    save.RE = 3000.0;
    save.F = 0.1;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 2]] = 0.0;
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 2 as f64);

    save.BOUNDS[[1, 3]] = -100.0;
    save.BOUNDS[[2, 3]] = 100.0;

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.LAT = (save.BOUNDS[[2, 2]] - 0.00000001);
    save.LON = (spicelib::PI(ctx) / 2 as f64);
    save.ALT = 0.0;

    spicelib::GEOREC(
        save.LON,
        save.LAT,
        save.ALT,
        save.RE,
        save.F,
        save.P.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MARGIN = 0.0000001;

    save.EXCLUD = NONE;

    spicelib::ZZINPDT(
        save.P.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"INSIDE", save.INSIDE, true, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Segment includes north pole. Point\'s longitude is out of range; altitude is out of range.", ctx)?;

    save.MARGIN = 0.0000001;

    save.A = save.RE;
    save.B = (save.RE * (1.0 - save.F));

    spicelib::ZZELLBDS(
        save.A,
        save.B,
        (save.BOUNDS[[2, 3]] * ((1 as f64) + save.MARGIN)),
        (save.BOUNDS[[1, 3]] * ((1 as f64) - save.MARGIN)),
        &mut save.EQHI,
        &mut save.POLHI,
        &mut save.EQLOW,
        &mut save.POLLOW,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Start with a point on the reference ellipsoid. Find the point
    // directly above this on the outer bounding ellipsoid.
    //
    save.LAT = (save.BOUNDS[[2, 2]] - 0.00000001);
    save.LON = (spicelib::PI(ctx) / 2 as f64);
    save.ALT = 0.0;

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

    spicelib::SURFNM(
        save.A,
        save.A,
        save.B,
        save.VERTEX.as_slice(),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;

    //
    // Find the surface point on the lower bounding ellipsoid, at the
    // given planetodetic longitude and latitude (relative to the
    // reference ellipsoid).
    //

    spicelib::SURFPT(
        save.VERTEX.as_slice(),
        save.NORMAL.as_slice(),
        save.EQHI,
        save.EQHI,
        save.POLHI,
        save.HIGHPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We should never fail to find an intercept.
    //
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::VLCOM(
        1.0,
        save.HIGHPT.as_slice(),
        0.000001,
        save.NORMAL.as_slice(),
        save.P.as_slice_mut(),
    );

    save.EXCLUD = NONE;

    spicelib::ZZINPDT(
        save.P.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"INSIDE", save.INSIDE, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Segment\'s upper latitude bound is near north pole. Point\'s longitude is out of range; altitude and latitude are in range.", ctx)?;

    save.MARGIN = 0.00000001;

    save.RE = 3000.0;
    save.F = 0.1;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 2]] = 0.0;
    save.BOUNDS[[2, 2]] = ((spicelib::PI(ctx) / 2 as f64) - (1.5 * save.MARGIN));

    save.BOUNDS[[1, 3]] = -100.0;
    save.BOUNDS[[2, 3]] = 100.0;

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.LAT = (save.BOUNDS[[2, 2]] + (save.MARGIN / 2 as f64));
    save.LON = (spicelib::PI(ctx) / 2 as f64);
    save.ALT = 0.0;

    spicelib::GEOREC(
        save.LON,
        save.LAT,
        save.ALT,
        save.RE,
        save.F,
        save.P.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.EXCLUD = NONE;

    spicelib::ZZINPDT(
        save.P.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"INSIDE", save.INSIDE, true, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Segment\'s upper latitude bound is near north pole. Point\'s longitude is out of range; altitude is in range; latitude is out of range.", ctx)?;

    save.MARGIN = 0.00000001;

    save.RE = 3000.0;
    save.F = 0.1;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 2]] = 0.0;
    save.BOUNDS[[2, 2]] = ((spicelib::PI(ctx) / 2 as f64) - 0.0000001);

    save.BOUNDS[[1, 3]] = -100.0;
    save.BOUNDS[[2, 3]] = 100.0;

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.LAT = (save.BOUNDS[[2, 2]] + (save.MARGIN * 2 as f64));
    save.LON = (spicelib::PI(ctx) / 2 as f64);
    save.ALT = 0.0;

    spicelib::GEOREC(
        save.LON,
        save.LAT,
        save.ALT,
        save.RE,
        save.F,
        save.P.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.EXCLUD = NONE;

    spicelib::ZZINPDT(
        save.P.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"INSIDE", save.INSIDE, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Segment\'s upper latitude bound is near north pole. Point\'s longitude is out of range; latitude is in range; altitude is out of range.", ctx)?;

    save.MARGIN = 0.0000001;

    save.A = save.RE;
    save.B = (save.RE * (1.0 - save.F));

    spicelib::ZZELLBDS(
        save.A,
        save.B,
        (save.BOUNDS[[2, 3]] * ((1 as f64) + save.MARGIN)),
        (save.BOUNDS[[1, 3]] * ((1 as f64) - save.MARGIN)),
        &mut save.EQHI,
        &mut save.POLHI,
        &mut save.EQLOW,
        &mut save.POLLOW,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Start with a point on the reference ellipsoid. Find the point
    // directly above this on the outer bounding ellipsoid.
    //
    save.LAT = (save.BOUNDS[[2, 2]] + (save.MARGIN / 2 as f64));
    save.LON = (spicelib::PI(ctx) / 2 as f64);
    save.ALT = 0.0;

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

    spicelib::SURFNM(
        save.A,
        save.A,
        save.B,
        save.VERTEX.as_slice(),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;

    //
    // Find the surface point on the lower bounding ellipsoid, at the
    // given planetodetic longitude and latitude (relative to the
    // reference ellipsoid).
    //

    spicelib::SURFPT(
        save.VERTEX.as_slice(),
        save.NORMAL.as_slice(),
        save.EQHI,
        save.EQHI,
        save.POLHI,
        save.HIGHPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We should never fail to find an intercept.
    //
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::VLCOM(
        1.0,
        save.HIGHPT.as_slice(),
        0.000001,
        save.NORMAL.as_slice(),
        save.P.as_slice_mut(),
    );

    save.EXCLUD = NONE;

    spicelib::ZZINPDT(
        save.P.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"INSIDE", save.INSIDE, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Segment includes south pole. Point\'s longitude is out of range; altitude is in range.",
        ctx,
    )?;

    save.RE = 3000.0;
    save.F = 0.1;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 2 as f64);
    save.BOUNDS[[2, 2]] = 0.0;

    save.BOUNDS[[1, 3]] = -100.0;
    save.BOUNDS[[2, 3]] = 100.0;

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.LAT = (save.BOUNDS[[1, 2]] + 0.00000001);
    save.LON = (spicelib::PI(ctx) / 2 as f64);
    save.ALT = 0.0;

    spicelib::GEOREC(
        save.LON,
        save.LAT,
        save.ALT,
        save.RE,
        save.F,
        save.P.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MARGIN = 0.0000001;

    save.EXCLUD = NONE;

    spicelib::ZZINPDT(
        save.P.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"INSIDE", save.INSIDE, true, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Segment includes south pole. Point\'s longitude is out of range; altitude is out of range.", ctx)?;

    save.MARGIN = 0.0000001;

    save.A = save.RE;
    save.B = (save.RE * (1.0 - save.F));

    spicelib::ZZELLBDS(
        save.A,
        save.B,
        (save.BOUNDS[[2, 3]] * ((1 as f64) + save.MARGIN)),
        (save.BOUNDS[[1, 3]] * ((1 as f64) - save.MARGIN)),
        &mut save.EQHI,
        &mut save.POLHI,
        &mut save.EQLOW,
        &mut save.POLLOW,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Start with a point on the reference ellipsoid. Find the point
    // directly above this on the outer bounding ellipsoid.
    //
    save.LAT = (save.BOUNDS[[1, 2]] + 0.00000001);
    save.LON = (spicelib::PI(ctx) / 2 as f64);
    save.ALT = 0.0;

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

    spicelib::SURFNM(
        save.A,
        save.A,
        save.B,
        save.VERTEX.as_slice(),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;

    //
    // Find the surface point on the lower bounding ellipsoid, at the
    // given planetodetic longitude and latitude (relative to the
    // reference ellipsoid).
    //

    spicelib::SURFPT(
        save.VERTEX.as_slice(),
        save.NORMAL.as_slice(),
        save.EQHI,
        save.EQHI,
        save.POLHI,
        save.HIGHPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We should never fail to find an intercept.
    //
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::VLCOM(
        1.0,
        save.HIGHPT.as_slice(),
        0.000001,
        save.NORMAL.as_slice(),
        save.P.as_slice_mut(),
    );

    save.EXCLUD = NONE;

    spicelib::ZZINPDT(
        save.P.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"INSIDE", save.INSIDE, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Segment\'s lower latitude bound is near south pole. Point\'s longitude is out of range; altitude and latitude are in range.", ctx)?;

    save.MARGIN = 0.00000001;

    save.RE = 3000.0;
    save.F = 0.1;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 2]] = (-(spicelib::PI(ctx) / 2 as f64) + (1.5 * save.MARGIN));
    save.BOUNDS[[2, 2]] = 0.0;

    save.BOUNDS[[1, 3]] = -100.0;
    save.BOUNDS[[2, 3]] = 100.0;

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.LAT = (save.BOUNDS[[1, 2]] - (save.MARGIN / 2 as f64));
    save.LON = (spicelib::PI(ctx) / 2 as f64);
    save.ALT = 0.0;

    spicelib::GEOREC(
        save.LON,
        save.LAT,
        save.ALT,
        save.RE,
        save.F,
        save.P.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.EXCLUD = NONE;

    spicelib::ZZINPDT(
        save.P.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"INSIDE", save.INSIDE, true, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Segment\'s lower latitude bound is near north pole. Point\'s longitude is out of range; altitude is in range; latitude is out of range.", ctx)?;

    save.MARGIN = 0.00000001;

    save.RE = 3000.0;
    save.F = 0.1;

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = (spicelib::PI(ctx) / 4 as f64);

    save.BOUNDS[[1, 2]] = 0.0;
    save.BOUNDS[[2, 2]] = ((spicelib::PI(ctx) / 2 as f64) - 0.0000001);

    save.BOUNDS[[1, 3]] = -100.0;
    save.BOUNDS[[2, 3]] = 100.0;

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.LAT = (save.BOUNDS[[2, 2]] + (save.MARGIN * 2 as f64));
    save.LON = (spicelib::PI(ctx) / 2 as f64);
    save.ALT = 0.0;

    spicelib::GEOREC(
        save.LON,
        save.LAT,
        save.ALT,
        save.RE,
        save.F,
        save.P.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.EXCLUD = NONE;

    spicelib::ZZINPDT(
        save.P.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"INSIDE", save.INSIDE, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Segment\'s lower latitude bound is near south pole. Point\'s longitude is out of range; latitude is in range; altitude is out of range.", ctx)?;

    save.MARGIN = 0.0000001;

    save.A = save.RE;
    save.B = (save.RE * (1.0 - save.F));

    spicelib::ZZELLBDS(
        save.A,
        save.B,
        (save.BOUNDS[[2, 3]] * ((1 as f64) + save.MARGIN)),
        (save.BOUNDS[[1, 3]] * ((1 as f64) - save.MARGIN)),
        &mut save.EQHI,
        &mut save.POLHI,
        &mut save.EQLOW,
        &mut save.POLLOW,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Start with a point on the reference ellipsoid. Find the point
    // directly above this on the outer bounding ellipsoid.
    //
    save.LAT = (save.BOUNDS[[1, 2]] - (save.MARGIN / 2 as f64));
    save.LON = (spicelib::PI(ctx) / 2 as f64);
    save.ALT = 0.0;

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

    spicelib::SURFNM(
        save.A,
        save.A,
        save.B,
        save.VERTEX.as_slice(),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;

    //
    // Find the surface point on the lower bounding ellipsoid, at the
    // given planetodetic longitude and latitude (relative to the
    // reference ellipsoid).
    //

    spicelib::SURFPT(
        save.VERTEX.as_slice(),
        save.NORMAL.as_slice(),
        save.EQHI,
        save.EQHI,
        save.POLHI,
        save.HIGHPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We should never fail to find an intercept.
    //
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::VLCOM(
        1.0,
        save.HIGHPT.as_slice(),
        0.000001,
        save.NORMAL.as_slice(),
        save.P.as_slice_mut(),
    );

    save.EXCLUD = NONE;

    spicelib::ZZINPDT(
        save.P.as_slice(),
        save.BOUNDS.as_slice(),
        save.CORPAR.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"INSIDE", save.INSIDE, false, OK, ctx)?;

    //***********************************************************************
    //
    //     Normal cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Choose a positive margin. We'll vary the margin in some
    // test cases, so we use POSMRG to save the original value.
    //
    save.POSMRG = 0.000001;
    save.MARGIN = save.POSMRG;

    //
    // Loop over the volume element cases.
    //
    for LONIX in 1..=NLON {
        save.BOUNDS[[1, 1]] = save.MINLON[LONIX];
        save.BOUNDS[[2, 1]] = save.MAXLON[LONIX];

        //
        // Normalize the element's longitude bounds.
        //
        save.TOL = 0.0000000000001;

        spicelib::ZZNRMLON(
            save.BOUNDS[[1, 1]],
            save.BOUNDS[[2, 1]],
            save.TOL,
            &mut save.NRMMIN,
            &mut save.NRMMAX,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        //
        // Indicate whether the longitude boundaries exist (vs
        // 2*pi longitude extent).
        //
        save.LONBDS = ((save.NRMMAX - save.NRMMIN) < (((2 as f64) * spicelib::PI(ctx)) - LONALI));

        for LATIX1 in 1..=NLAT {
            save.BOUNDS[[1, 2]] = save.MINLAT[LATIX1];
            //
            // Indicate whether the lower latitude boundary is a surface.
            //
            save.LATLB = (save.MINLAT[LATIX1] > -spicelib::HALFPI(ctx));

            for LATIX2 in LATIX1..=NLAT {
                save.BOUNDS[[2, 2]] = save.MAXLAT[LATIX2];

                //
                // Indicate whether the upper latitude boundary is a
                // surface.
                //
                save.LATUB = (save.MAXLAT[LATIX2] < spicelib::HALFPI(ctx));

                for SHAPIX in 1..=NELL {
                    save.A = save.EQRAD[SHAPIX];
                    save.B = save.POLRAD[SHAPIX];
                    save.F = ((save.A - save.B) / save.A);

                    save.CORPAR[1] = save.A;
                    save.CORPAR[2] = save.F;

                    save.BOUNDS[[1, 3]] = save.MINALT[SHAPIX];
                    save.BOUNDS[[2, 3]] = save.MAXALT[SHAPIX];

                    {
                        let m1__: i32 = 0;
                        let m2__: i32 = 3;
                        let m3__: i32 = 1;
                        save.EXCLUD = m1__;
                        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                            //
                            // --- Case: ------------------------------------------------------
                            //

                            //
                            // Set the input point so that each coordinate
                            // is the midpoint of the element's range for
                            // that coordinate.
                            //

                            fstr::assign(&mut save.STEM, b"Lon #:#; Lat #:#; Alt #:#; EXCLUD = #;");

                            spicelib::REPMD(
                                &save.STEM.to_vec(),
                                b"#",
                                (save.BOUNDS[[1, 1]] * spicelib::DPR(ctx)),
                                9,
                                &mut save.STEM,
                                ctx,
                            );
                            spicelib::REPMD(
                                &save.STEM.to_vec(),
                                b"#",
                                (save.BOUNDS[[2, 1]] * spicelib::DPR(ctx)),
                                9,
                                &mut save.STEM,
                                ctx,
                            );
                            spicelib::REPMD(
                                &save.STEM.to_vec(),
                                b"#",
                                (save.BOUNDS[[1, 2]] * spicelib::DPR(ctx)),
                                9,
                                &mut save.STEM,
                                ctx,
                            );
                            spicelib::REPMD(
                                &save.STEM.to_vec(),
                                b"#",
                                (save.BOUNDS[[2, 2]] * spicelib::DPR(ctx)),
                                9,
                                &mut save.STEM,
                                ctx,
                            );
                            spicelib::REPMD(
                                &save.STEM.to_vec(),
                                b"#",
                                save.BOUNDS[[1, 3]],
                                9,
                                &mut save.STEM,
                                ctx,
                            );
                            spicelib::REPMD(
                                &save.STEM.to_vec(),
                                b"#",
                                save.BOUNDS[[2, 3]],
                                9,
                                &mut save.STEM,
                                ctx,
                            );
                            spicelib::REPMI(
                                &save.STEM.to_vec(),
                                b"#",
                                save.EXCLUD,
                                &mut save.STEM,
                                ctx,
                            );

                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            save.I = (save.I + 1);

                            //
                            // --- Case: ------------------------------------------------------
                            //

                            fstr::assign(&mut save.TITLE, &save.STEM);
                            spicelib::SUFFIX(
                                b"Midpoint case, positive margin ",
                                1,
                                &mut save.TITLE,
                            );
                            testutil::TCASE(&save.TITLE, ctx)?;

                            save.MIDLON = ((save.NRMMIN + save.NRMMAX) / 2 as f64);
                            save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);
                            save.MIDALT = ((save.BOUNDS[[1, 3]] + save.BOUNDS[[2, 3]]) / 2 as f64);

                            spicelib::GEOREC(
                                save.MIDLON,
                                save.MIDLAT,
                                save.MIDALT,
                                save.A,
                                save.F,
                                save.P.as_slice_mut(),
                                ctx,
                            )?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            spicelib::ZZINPDT(
                                save.P.as_slice(),
                                save.BOUNDS.as_slice(),
                                save.CORPAR.as_slice(),
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

                            fstr::assign(&mut save.TITLE, &save.STEM);
                            spicelib::SUFFIX(b"Midpoint case, zero margin", 1, &mut save.TITLE);
                            testutil::TCASE(&save.TITLE, ctx)?;

                            save.MARGIN = 0.0;

                            save.MIDLON = ((save.NRMMIN + save.NRMMAX) / 2 as f64);
                            save.MIDLAT = ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);
                            save.MIDALT = ((save.BOUNDS[[1, 3]] + save.BOUNDS[[2, 3]]) / 2 as f64);

                            spicelib::GEOREC(
                                save.MIDLON,
                                save.MIDLAT,
                                save.MIDALT,
                                save.A,
                                save.F,
                                save.P.as_slice_mut(),
                                ctx,
                            )?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            spicelib::ZZINPDT(
                                save.P.as_slice(),
                                save.BOUNDS.as_slice(),
                                save.CORPAR.as_slice(),
                                save.MARGIN,
                                save.EXCLUD,
                                &mut save.INSIDE,
                                ctx,
                            )?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            save.XIN = true;

                            testutil::CHCKSL(b"INSIDE", save.INSIDE, save.XIN, OK, ctx)?;

                            //
                            // Restore margin.
                            //
                            save.MARGIN = save.POSMRG;

                            //
                            // --- Case: ------------------------------------------------------
                            //
                            //
                            //                    Check interior points near each corner of
                            //                    the volume element.
                            //
                            for L in 1..=2 {
                                for M in 1..=2 {
                                    for N in 1..=2 {
                                        save.I = (save.I + 1);
                                        //
                                        // --- Case: ------------------------------------------------------
                                        //
                                        fstr::assign(&mut save.TITLE, &save.STEM);

                                        spicelib::SUFFIX(
                                            b"Point near corner # # #; interior",
                                            1,
                                            &mut save.TITLE,
                                        );
                                        spicelib::REPMI(
                                            &save.TITLE.to_vec(),
                                            b"#",
                                            L,
                                            &mut save.TITLE,
                                            ctx,
                                        );
                                        spicelib::REPMI(
                                            &save.TITLE.to_vec(),
                                            b"#",
                                            M,
                                            &mut save.TITLE,
                                            ctx,
                                        );
                                        spicelib::REPMI(
                                            &save.TITLE.to_vec(),
                                            b"#",
                                            N,
                                            &mut save.TITLE,
                                            ctx,
                                        );
                                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                                        testutil::TCASE(&save.TITLE, ctx)?;

                                        //
                                        // Set incremental offsets. AEPS is the
                                        // "angular epsilon." EPS is scaled to
                                        // yield a "distance epsilon."
                                        //
                                        save.AEPS = 0.00000000001;
                                        save.EPS = (0.000001 * save.A);

                                        //
                                        // Obtain the ellipsoid radii of the lower
                                        // and upper bounding surfaces. It is these,
                                        // not the surfaces of minimum and maximum
                                        // altitude relative to the body's reference
                                        // ellipsoid, that are used for comparison.
                                        //
                                        if (save.A > save.B) {
                                            spicelib::ZZELLBDS(
                                                save.A,
                                                save.B,
                                                (save.BOUNDS[[2, 3]] * ((1 as f64) + save.MARGIN)),
                                                (save.BOUNDS[[1, 3]] * ((1 as f64) - save.MARGIN)),
                                                &mut save.EQHI,
                                                &mut save.POLHI,
                                                &mut save.EQLOW,
                                                &mut save.POLLOW,
                                                ctx,
                                            )?;
                                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                                        } else {
                                            //
                                            // This is the prolate case. ZZELLBDS
                                            // works only with oblate spheroids,
                                            // so swap A and B on input.
                                            //
                                            spicelib::ZZELLBDS(
                                                save.B,
                                                save.A,
                                                (save.BOUNDS[[2, 3]] * ((1 as f64) + save.MARGIN)),
                                                (save.BOUNDS[[1, 3]] * ((1 as f64) - save.MARGIN)),
                                                &mut save.EQHI,
                                                &mut save.POLHI,
                                                &mut save.EQLOW,
                                                &mut save.POLLOW,
                                                ctx,
                                            )?;
                                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                                            //
                                            // The EQ* outputs are actually for
                                            // the polar radii of the bounding
                                            // ellipsoids, and the POL* outputs
                                            // are for the equatorial radii of
                                            // the bounding ellipsoids.
                                            //
                                            spicelib::SWAPD(&mut save.EQHI, &mut save.POLHI);
                                            spicelib::SWAPD(&mut save.EQLOW, &mut save.POLLOW);
                                        }
                                        //
                                        // Multiply the small increments by 1 or
                                        // -1 as needed, depending on whether they
                                        // are used as offsets from upper or lower
                                        // bounds.
                                        //
                                        save.LAT = (save.BOUNDS[[M, 2]]
                                            + (save.AEPS * intrinsics::pow(-1, (M + 1)) as f64));

                                        if (L == 1) {
                                            save.LON = (save.NRMMIN + save.AEPS);
                                        } else {
                                            save.LON = (save.NRMMAX - save.AEPS);
                                        }

                                        //
                                        // Find the point at the given longitude
                                        // and latitude on the reference ellipsoid.
                                        // We'll use this below.
                                        //
                                        spicelib::GEOREC(
                                            save.LON,
                                            save.LAT,
                                            0.0,
                                            save.A,
                                            save.F,
                                            save.VERTEX.as_slice_mut(),
                                            ctx,
                                        )?;
                                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                                        spicelib::SURFNM(
                                            save.A,
                                            save.A,
                                            save.B,
                                            save.VERTEX.as_slice(),
                                            save.NORMAL.as_slice_mut(),
                                            ctx,
                                        )?;

                                        if (N == 1) {
                                            //
                                            // Find the surface point on the lower
                                            // bounding ellipsoid, at the given
                                            // planetodetic longitude and latitude
                                            // (relative to the reference ellipsoid).

                                            if (save.BOUNDS[[1, 3]] < 0.0) {
                                                //
                                                // The direction to the inner bounding
                                                // ellipsoid is "down."
                                                //
                                                spicelib::VMINUS(
                                                    save.NORMAL.as_slice(),
                                                    save.DIR.as_slice_mut(),
                                                );
                                            } else {
                                                spicelib::VEQU(
                                                    save.NORMAL.as_slice(),
                                                    save.DIR.as_slice_mut(),
                                                );
                                            }

                                            spicelib::SURFPT(
                                                save.VERTEX.as_slice(),
                                                save.DIR.as_slice(),
                                                save.EQLOW,
                                                save.EQLOW,
                                                save.POLLOW,
                                                save.LOWPT.as_slice_mut(),
                                                &mut save.FOUND,
                                                ctx,
                                            )?;
                                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                                            //
                                            // We should never fail to find an
                                            // intercept.
                                            //
                                            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

                                            //
                                            // Find the geodetic coordinates of LOWPT
                                            // with respect to the reference
                                            // ellipsoid.
                                            //
                                            spicelib::RECGEO(
                                                save.LOWPT.as_slice(),
                                                save.A,
                                                save.F,
                                                &mut save.LOWLON,
                                                &mut save.LOWLAT,
                                                &mut save.LOWALT,
                                                ctx,
                                            )?;
                                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                                            //
                                            // Adjust the altitude of LOWPT by a
                                            // small increment.
                                            //
                                            save.ALT = (save.LOWALT
                                                + (save.EPS * intrinsics::pow(-1, (N + 1)) as f64));
                                        } else {
                                            //
                                            // This is the case for the outer
                                            // bounding ellipsoid (N = 2).
                                            //
                                            if (save.BOUNDS[[2, 3]] < 0.0) {
                                                //
                                                // The direction to the outer bounding
                                                // ellipsoid is "down."
                                                //
                                                spicelib::VMINUS(
                                                    save.NORMAL.as_slice(),
                                                    save.DIR.as_slice_mut(),
                                                );
                                            } else {
                                                spicelib::VEQU(
                                                    save.NORMAL.as_slice(),
                                                    save.DIR.as_slice_mut(),
                                                );
                                            }

                                            spicelib::SURFPT(
                                                save.VERTEX.as_slice(),
                                                save.DIR.as_slice(),
                                                save.EQHI,
                                                save.EQHI,
                                                save.POLHI,
                                                save.HIGHPT.as_slice_mut(),
                                                &mut save.FOUND,
                                                ctx,
                                            )?;
                                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                                            //
                                            // We should never fail to find an
                                            // intercept.
                                            //
                                            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
                                            //
                                            // Find the geodetic coordinates of
                                            // HIGHPT with respect to the reference
                                            // ellipsoid.
                                            //
                                            spicelib::RECGEO(
                                                save.HIGHPT.as_slice(),
                                                save.A,
                                                save.F,
                                                &mut save.HILON,
                                                &mut save.HILAT,
                                                &mut save.HIALT,
                                                ctx,
                                            )?;
                                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                                            //
                                            // Adjust the altitude of HIGHPT by a
                                            // small increment.
                                            //
                                            save.ALT = (save.HIALT
                                                + (save.EPS * intrinsics::pow(-1, (N + 1)) as f64));
                                        }

                                        //
                                        // Produce the perturbed input point using
                                        // the perturbed geodetic coordinates.
                                        //

                                        spicelib::GEOREC(
                                            save.LON,
                                            save.LAT,
                                            save.ALT,
                                            save.A,
                                            save.F,
                                            save.P2.as_slice_mut(),
                                            ctx,
                                        )?;
                                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                                        spicelib::ZZINPDT(
                                            save.P2.as_slice(),
                                            save.BOUNDS.as_slice(),
                                            save.CORPAR.as_slice(),
                                            save.MARGIN,
                                            save.EXCLUD,
                                            &mut save.INSIDE,
                                            ctx,
                                        )?;
                                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                                        save.XIN = true;

                                        testutil::CHCKSL(
                                            b"INSIDE",
                                            save.INSIDE,
                                            save.XIN,
                                            OK,
                                            ctx,
                                        )?;

                                        //
                                        // --- Case: ------------------------------------------------------
                                        //
                                        fstr::assign(&mut save.TITLE, &save.STEM);
                                        spicelib::SUFFIX(b"Point near corner # # #; interior; excluded coordinate out of range.", 1, &mut save.TITLE);
                                        spicelib::REPMI(
                                            &save.TITLE.to_vec(),
                                            b"#",
                                            L,
                                            &mut save.TITLE,
                                            ctx,
                                        );
                                        spicelib::REPMI(
                                            &save.TITLE.to_vec(),
                                            b"#",
                                            M,
                                            &mut save.TITLE,
                                            ctx,
                                        );
                                        spicelib::REPMI(
                                            &save.TITLE.to_vec(),
                                            b"#",
                                            N,
                                            &mut save.TITLE,
                                            ctx,
                                        );
                                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                                        testutil::TCASE(&save.TITLE, ctx)?;

                                        //
                                        // Set initial coordinates of the input
                                        // point.
                                        //
                                        spicelib::RECGEO(
                                            save.P2.as_slice(),
                                            save.A,
                                            save.F,
                                            &mut save.LON,
                                            &mut save.LAT,
                                            &mut save.ALT,
                                            ctx,
                                        )?;
                                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                                        //
                                        // Use the perturbed point produced for
                                        // the previous test case. Modify the
                                        // excluded coordinate of the point.
                                        //
                                        if (save.EXCLUD == 2) {
                                            //
                                            // Latitude is not considered in
                                            // the bounds comparison performed
                                            // by ZZINPDT.
                                            //
                                            // However, we can't move the latitude
                                            // too far out of range, or else the
                                            // relationship between the altitude
                                            // of the point and the bounding ellipsoid
                                            // will change too much.
                                            //
                                            if (M == 1) {
                                                save.LAT = intrinsics::DMAX1(&[
                                                    -spicelib::HALFPI(ctx),
                                                    (save.LAT - ((2 as f64) * save.MARGIN)),
                                                ]);
                                            } else {
                                                save.LAT = intrinsics::DMIN1(&[
                                                    spicelib::HALFPI(ctx),
                                                    (save.LAT + ((2 as f64) * save.MARGIN)),
                                                ]);
                                            }
                                        }

                                        if (save.EXCLUD == 3) {
                                            //
                                            // Altitude is not considered in
                                            // the bounds comparison performed
                                            // by ZZINPDT.
                                            //
                                            if (N == 1) {
                                                save.ALT = (save.ALT - (0.1 * f64::abs(save.ALT)));
                                            } else {
                                                save.ALT = (save.ALT + (0.1 * f64::abs(save.ALT)));
                                            }
                                        }

                                        if (save.EXCLUD == 1) {
                                            //
                                            // Longitude is not considered in
                                            // the bounds comparison performed
                                            // by ZZINPDT.
                                            //
                                            if (N == 1) {
                                                save.LON = (save.NRMMIN - save.AEPS);
                                            } else {
                                                save.LON = (save.NRMMAX + save.AEPS);
                                            }
                                        }

                                        spicelib::GEOREC(
                                            save.LON,
                                            save.LAT,
                                            save.ALT,
                                            save.A,
                                            save.F,
                                            save.P3.as_slice_mut(),
                                            ctx,
                                        )?;
                                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                                        spicelib::ZZINPDT(
                                            save.P3.as_slice(),
                                            save.BOUNDS.as_slice(),
                                            save.CORPAR.as_slice(),
                                            save.MARGIN,
                                            save.EXCLUD,
                                            &mut save.INSIDE,
                                            ctx,
                                        )?;
                                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                                        save.XIN = true;

                                        testutil::CHCKSL(
                                            b"INSIDE",
                                            save.INSIDE,
                                            save.XIN,
                                            OK,
                                            ctx,
                                        )?;

                                        //
                                        // --- Case: ------------------------------------------------------
                                        //
                                        fstr::assign(&mut save.TITLE, &save.STEM);
                                        spicelib::SUFFIX(b"Point near corner # # #; exterior; successor of excluded coordinate out of range.", 1, &mut save.TITLE);
                                        spicelib::REPMI(
                                            &save.TITLE.to_vec(),
                                            b"#",
                                            L,
                                            &mut save.TITLE,
                                            ctx,
                                        );
                                        spicelib::REPMI(
                                            &save.TITLE.to_vec(),
                                            b"#",
                                            M,
                                            &mut save.TITLE,
                                            ctx,
                                        );
                                        spicelib::REPMI(
                                            &save.TITLE.to_vec(),
                                            b"#",
                                            N,
                                            &mut save.TITLE,
                                            ctx,
                                        );
                                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                                        testutil::TCASE(&save.TITLE, ctx)?;

                                        //
                                        // Set initial coordinates of the input
                                        // point.
                                        //
                                        spicelib::RECGEO(
                                            save.P2.as_slice(),
                                            save.A,
                                            save.F,
                                            &mut save.LON,
                                            &mut save.LAT,
                                            &mut save.ALT,
                                            ctx,
                                        )?;
                                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                                        //
                                        // We're going to work with a modified
                                        // version of the input bounds. This will
                                        // make it easier to create coordinates that
                                        // are slightly out of range.
                                        //
                                        spicelib::MOVED(
                                            save.BOUNDS.as_slice(),
                                            6,
                                            save.MODBDS.as_slice_mut(),
                                        );

                                        //
                                        // Use the perturbed point produced for
                                        // the previous test case. Modify the
                                        // excluded coordinate of the point.
                                        //
                                        if (save.EXCLUD == 1) {
                                            //
                                            // Longitude is not considered in
                                            // the bounds comparison performed
                                            // by ZZINPDT. We'll move the latitude
                                            // out of bounds.
                                            //
                                            // However, we can't move the latitude
                                            // too far out of range, or else the
                                            // relationship between the altitude
                                            // of the point and the bounding ellipsoid
                                            // will change too much.
                                            //
                                            if (M == 1) {
                                                save.LAT = intrinsics::DMAX1(&[
                                                    -spicelib::HALFPI(ctx),
                                                    (save.LAT - ((2 as f64) * save.MARGIN)),
                                                ]);
                                            } else {
                                                save.LAT = intrinsics::DMIN1(&[
                                                    spicelib::HALFPI(ctx),
                                                    (save.LAT + ((2 as f64) * save.MARGIN)),
                                                ]);
                                            }
                                        }

                                        if ((save.EXCLUD == 2) || (save.EXCLUD == 0)) {
                                            //
                                            // Either all coordinates are considered,
                                            // or latitude is not considered in
                                            // the bounds comparison performed
                                            // by ZZINPDT. We'll move the altitude
                                            // out of bounds.
                                            //

                                            if (N == 1) {
                                                save.ALT = (save.ALT - (0.1 * f64::abs(save.ALT)));
                                            } else {
                                                save.ALT = (save.ALT + (0.1 * f64::abs(save.ALT)));
                                            }
                                        }

                                        if (save.EXCLUD == 3) {
                                            //
                                            // Altitude is not considered in
                                            // the bounds comparison performed
                                            // by ZZINPDT. We'll move the
                                            // longitude out of bounds.
                                            //
                                            // To make it possible to create
                                            // longitude values that are out
                                            // of bounds, adjust the longitude
                                            // bounds. We'll nudge the lower
                                            // longitude bound upwards for this
                                            // test.
                                            //
                                            // The magnitude of the "nudge" must
                                            // be great enough to overcome the
                                            // rounding margin used (even) by the
                                            // "no margin" routine ZZINPDT for
                                            // longitude comparisons.
                                            //
                                            save.MODBDS[[1, 1]] =
                                                (save.MODBDS[[1, 1]] + ((4 as f64) * save.MARGIN));
                                            save.MODBDS[[2, 1]] =
                                                (save.MODBDS[[2, 1]] - ((4 as f64) * save.MARGIN));

                                            if (N == 1) {
                                                save.LON =
                                                    (save.NRMMIN - ((2 as f64) * save.MARGIN));
                                            } else {
                                                save.LON =
                                                    (save.NRMMAX + ((2 as f64) * save.MARGIN));
                                            }
                                        }

                                        spicelib::GEOREC(
                                            save.LON,
                                            save.LAT,
                                            save.ALT,
                                            save.A,
                                            save.F,
                                            save.P3.as_slice_mut(),
                                            ctx,
                                        )?;
                                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                                        //
                                        // Perform the test using modified bounds.
                                        // See the EXCLUD = 3 case above.
                                        //

                                        spicelib::ZZINPDT(
                                            save.P3.as_slice(),
                                            save.MODBDS.as_slice(),
                                            save.CORPAR.as_slice(),
                                            save.MARGIN,
                                            save.EXCLUD,
                                            &mut save.INSIDE,
                                            ctx,
                                        )?;
                                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                                        //
                                        // Except for some special cases, the
                                        // point should be outside the element.
                                        //
                                        save.XIN = false;
                                        //
                                        // Our latitude modifications won't work for
                                        // the cases where the latitude is already
                                        // at an extreme value.
                                        //
                                        if (save.EXCLUD == 1) {
                                            //
                                            // Longitude is the excluded coordinate;
                                            // this is a case for which we try to
                                            // modify latitude. However, the
                                            // attempted modification may have no
                                            // effect if a boundary is already at
                                            // the extreme value.
                                            //
                                            if ((M == 1) && !save.LATLB) {
                                                save.XIN = true;
                                            } else if ((M == 2) && !save.LATUB) {
                                                save.XIN = true;
                                            }

                                            //
                                            // If the latitude is within the latitude
                                            // range extended by the margin, the
                                            // point is in the element.
                                            //
                                            if ((save.LAT >= (save.BOUNDS[[1, 2]] - save.MARGIN))
                                                && (save.LAT
                                                    <= (save.BOUNDS[[2, 2]] + save.MARGIN)))
                                            {
                                                save.XIN = true;
                                            }
                                        } else if (save.EXCLUD == 3) {
                                            //
                                            // Longitude will be set to an
                                            // out-of-range value. This won't
                                            // be considered if we're in the
                                            // "high latitude" case. We use the
                                            // old value of LATMRG here, because
                                            // with the new value one cannot create
                                            // longitudes that are out of range, due
                                            // to the longitude margin scaling.
                                            //
                                            if ((save.LAT > (spicelib::HALFPI(ctx) - PRVMRG))
                                                && (save.LAT <= save.BOUNDS[[2, 2]]))
                                            {
                                                save.XIN = true;
                                            }

                                            if ((save.LAT < (-spicelib::HALFPI(ctx) + PRVMRG))
                                                && (save.LAT >= save.BOUNDS[[1, 2]]))
                                            {
                                                save.XIN = true;
                                            }
                                        }

                                        testutil::CHCKSL(
                                            b"INSIDE",
                                            save.INSIDE,
                                            save.XIN,
                                            OK,
                                            ctx,
                                        )?;

                                        //***********************************************************************
                                        //
                                        //
                                        //     ZERO MARGIN CASES
                                        //
                                        //
                                        //***********************************************************************

                                        //
                                        // --- Case: ------------------------------------------------------
                                        //
                                        save.MARGIN = 0.0;

                                        fstr::assign(&mut save.TITLE, &save.STEM);

                                        spicelib::SUFFIX(
                                            b"Point near corner # # #; interior; zero margin ",
                                            1,
                                            &mut save.TITLE,
                                        );
                                        spicelib::REPMI(
                                            &save.TITLE.to_vec(),
                                            b"#",
                                            L,
                                            &mut save.TITLE,
                                            ctx,
                                        );
                                        spicelib::REPMI(
                                            &save.TITLE.to_vec(),
                                            b"#",
                                            M,
                                            &mut save.TITLE,
                                            ctx,
                                        );
                                        spicelib::REPMI(
                                            &save.TITLE.to_vec(),
                                            b"#",
                                            N,
                                            &mut save.TITLE,
                                            ctx,
                                        );
                                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                                        testutil::TCASE(&save.TITLE, ctx)?;

                                        //
                                        // Set incremental offsets. AEPS is the
                                        // "angular epsilon." EPS is scaled to
                                        // yield a "distance epsilon."
                                        //
                                        save.AEPS = 0.000000001;
                                        save.EPS = (0.000000001 * save.A);

                                        //
                                        // Obtain the ellipsoid radii of the lower
                                        // and upper bounding surfaces. It is these,
                                        // not the surfaces of minimum and maximum
                                        // altitude relative to the body's reference
                                        // ellipsoid, that are used for comparison.
                                        //
                                        if (save.A > save.B) {
                                            spicelib::ZZELLBDS(
                                                save.A,
                                                save.B,
                                                save.BOUNDS[[2, 3]],
                                                save.BOUNDS[[1, 3]],
                                                &mut save.EQHI,
                                                &mut save.POLHI,
                                                &mut save.EQLOW,
                                                &mut save.POLLOW,
                                                ctx,
                                            )?;
                                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                                        } else {
                                            //
                                            // This is the prolate case. ZZELLBDS
                                            // works only with oblate spheroids,
                                            // so swap A and B on input.
                                            //
                                            spicelib::ZZELLBDS(
                                                save.B,
                                                save.A,
                                                save.BOUNDS[[2, 3]],
                                                save.BOUNDS[[1, 3]],
                                                &mut save.EQHI,
                                                &mut save.POLHI,
                                                &mut save.EQLOW,
                                                &mut save.POLLOW,
                                                ctx,
                                            )?;
                                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                                            //
                                            // The EQ* outputs are actually for
                                            // the polar radii of the bounding
                                            // ellipsoids, and the POL* outputs
                                            // are for the equatorial radii of
                                            // the bounding ellipsoids.
                                            //
                                            spicelib::SWAPD(&mut save.EQHI, &mut save.POLHI);
                                            spicelib::SWAPD(&mut save.EQLOW, &mut save.POLLOW);
                                        }
                                        //
                                        // Multiply the small increments by 1 or
                                        // -1 as needed, depending on whether they
                                        // are used as offsets from upper or lower
                                        // bounds.
                                        //
                                        // Note the scaled increment for latitude.
                                        //
                                        save.LAT = (save.BOUNDS[[M, 2]]
                                            + (save.AEPS * intrinsics::pow(-1, (M + 1)) as f64));

                                        if (L == 1) {
                                            save.LON = (save.NRMMIN + save.AEPS);
                                        } else {
                                            save.LON = (save.NRMMAX - save.AEPS);
                                        }

                                        //
                                        // Find the point at the given longitude
                                        // and latitude on the reference ellipsoid.
                                        // We'll use this below.
                                        //
                                        spicelib::GEOREC(
                                            save.LON,
                                            save.LAT,
                                            0.0,
                                            save.A,
                                            save.F,
                                            save.VERTEX.as_slice_mut(),
                                            ctx,
                                        )?;
                                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                                        spicelib::SURFNM(
                                            save.A,
                                            save.A,
                                            save.B,
                                            save.VERTEX.as_slice(),
                                            save.NORMAL.as_slice_mut(),
                                            ctx,
                                        )?;

                                        if (N == 1) {
                                            //
                                            // Find the surface point on the lower
                                            // bounding ellipsoid, at the given
                                            // planetodetic longitude and latitude
                                            // (relative to the reference ellipsoid).

                                            if (save.BOUNDS[[1, 3]] < 0.0) {
                                                //
                                                // The direction to the inner bounding
                                                // ellipsoid is "down."
                                                //
                                                spicelib::VMINUS(
                                                    save.NORMAL.as_slice(),
                                                    save.DIR.as_slice_mut(),
                                                );
                                            } else {
                                                spicelib::VEQU(
                                                    save.NORMAL.as_slice(),
                                                    save.DIR.as_slice_mut(),
                                                );
                                            }

                                            spicelib::SURFPT(
                                                save.VERTEX.as_slice(),
                                                save.DIR.as_slice(),
                                                save.EQLOW,
                                                save.EQLOW,
                                                save.POLLOW,
                                                save.LOWPT.as_slice_mut(),
                                                &mut save.FOUND,
                                                ctx,
                                            )?;
                                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                                            //
                                            // We should never fail to find an
                                            // intercept.
                                            //
                                            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

                                            //
                                            // Find the geodetic coordinates of LOWPT
                                            // with respect to the reference
                                            // ellipsoid.
                                            //
                                            spicelib::RECGEO(
                                                save.LOWPT.as_slice(),
                                                save.A,
                                                save.F,
                                                &mut save.LOWLON,
                                                &mut save.LOWLAT,
                                                &mut save.LOWALT,
                                                ctx,
                                            )?;
                                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                                            //
                                            // Adjust the altitude of LOWPT by a
                                            // small increment.
                                            //
                                            save.ALT = (save.LOWALT
                                                + (save.EPS * intrinsics::pow(-1, (N + 1)) as f64));
                                        } else {
                                            //
                                            // This is the case for the outer
                                            // bounding ellipsoid (N = 2).
                                            //
                                            if (save.BOUNDS[[2, 3]] < 0.0) {
                                                //
                                                // The direction to the outer bounding
                                                // ellipsoid is "down."
                                                //
                                                spicelib::VMINUS(
                                                    save.NORMAL.as_slice(),
                                                    save.DIR.as_slice_mut(),
                                                );
                                            } else {
                                                spicelib::VEQU(
                                                    save.NORMAL.as_slice(),
                                                    save.DIR.as_slice_mut(),
                                                );
                                            }

                                            spicelib::SURFPT(
                                                save.VERTEX.as_slice(),
                                                save.DIR.as_slice(),
                                                save.EQHI,
                                                save.EQHI,
                                                save.POLHI,
                                                save.HIGHPT.as_slice_mut(),
                                                &mut save.FOUND,
                                                ctx,
                                            )?;
                                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                                            //
                                            // We should never fail to find an
                                            // intercept.
                                            //
                                            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
                                            //
                                            // Find the geodetic coordinates of
                                            // HIGHPT with respect to the reference
                                            // ellipsoid.
                                            //
                                            spicelib::RECGEO(
                                                save.HIGHPT.as_slice(),
                                                save.A,
                                                save.F,
                                                &mut save.HILON,
                                                &mut save.HILAT,
                                                &mut save.HIALT,
                                                ctx,
                                            )?;
                                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                                            //
                                            // Adjust the altitude of HIGHPT by a
                                            // small increment.
                                            //
                                            save.ALT = (save.HIALT
                                                + (save.EPS * intrinsics::pow(-1, (N + 1)) as f64));
                                        }

                                        //
                                        // Produce the perturbed input point using
                                        // the perturbed geodetic coordinates.
                                        //
                                        spicelib::GEOREC(
                                            save.LON,
                                            save.LAT,
                                            save.ALT,
                                            save.A,
                                            save.F,
                                            save.P2.as_slice_mut(),
                                            ctx,
                                        )?;
                                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                                        spicelib::ZZINPDT(
                                            save.P2.as_slice(),
                                            save.BOUNDS.as_slice(),
                                            save.CORPAR.as_slice(),
                                            save.MARGIN,
                                            save.EXCLUD,
                                            &mut save.INSIDE,
                                            ctx,
                                        )?;
                                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                                        save.XIN = true;

                                        testutil::CHCKSL(
                                            b"INSIDE",
                                            save.INSIDE,
                                            save.XIN,
                                            OK,
                                            ctx,
                                        )?;

                                        //
                                        // --- Case: ------------------------------------------------------
                                        //
                                        save.MARGIN = 0.0;

                                        fstr::assign(&mut save.TITLE, &save.STEM);

                                        spicelib::SUFFIX(b"Point near corner # # #; exterior; successor of excluded coordinate out of range; zero margin ", 1, &mut save.TITLE);
                                        spicelib::REPMI(
                                            &save.TITLE.to_vec(),
                                            b"#",
                                            L,
                                            &mut save.TITLE,
                                            ctx,
                                        );
                                        spicelib::REPMI(
                                            &save.TITLE.to_vec(),
                                            b"#",
                                            M,
                                            &mut save.TITLE,
                                            ctx,
                                        );
                                        spicelib::REPMI(
                                            &save.TITLE.to_vec(),
                                            b"#",
                                            N,
                                            &mut save.TITLE,
                                            ctx,
                                        );
                                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                                        testutil::TCASE(&save.TITLE, ctx)?;

                                        // Set initial coordinates of the input
                                        // point.
                                        //
                                        spicelib::RECGEO(
                                            save.P2.as_slice(),
                                            save.A,
                                            save.F,
                                            &mut save.LON,
                                            &mut save.LAT,
                                            &mut save.ALT,
                                            ctx,
                                        )?;
                                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                                        //
                                        // Set incremental offsets. AEPS is the
                                        // "angular epsilon." EPS is scaled to
                                        // yield a "distance epsilon."
                                        //
                                        save.AEPS = (0.0000001 * spicelib::RPD(ctx));
                                        save.EPS = (0.000000001 * save.A);

                                        //
                                        // Multiply the small increments by 1 or
                                        // -1 as needed, depending on whether they
                                        // are used as offsets from upper or lower
                                        // bounds.
                                        //
                                        // Note the scaled increment for latitude.
                                        //
                                        if (save.EXCLUD == LONIDX) {
                                            if (M == 1) {
                                                save.LAT = intrinsics::DMAX1(&[
                                                    (save.BOUNDS[[M, 2]]
                                                        - ((2 as f64) * save.AEPS)),
                                                    -spicelib::HALFPI(ctx),
                                                ]);
                                            } else {
                                                save.LAT = intrinsics::DMIN1(&[
                                                    (save.BOUNDS[[M, 2]]
                                                        + ((2 as f64) * save.AEPS)),
                                                    spicelib::HALFPI(ctx),
                                                ]);
                                            }
                                        } else {
                                            save.LAT = (save.BOUNDS[[M, 2]]
                                                + (save.AEPS
                                                    * intrinsics::pow(-1, (M + 1)) as f64));
                                        }

                                        if (save.EXCLUD == ALTIDX) {
                                            if (L == 1) {
                                                save.LON = (save.NRMMIN - save.AEPS);
                                            } else {
                                                save.LON = (save.NRMMAX + save.AEPS);
                                            }
                                        } else {
                                            if (L == 1) {
                                                save.LON = (save.NRMMIN + save.AEPS);
                                            } else {
                                                save.LON = (save.NRMMAX - save.AEPS);
                                            }
                                        }

                                        //
                                        // Find the point at the given longitude
                                        // and latitude on the reference ellipsoid.
                                        // We'll use this below.
                                        //
                                        spicelib::GEOREC(
                                            save.LON,
                                            save.LAT,
                                            0.0,
                                            save.A,
                                            save.F,
                                            save.VERTEX.as_slice_mut(),
                                            ctx,
                                        )?;
                                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                                        spicelib::SURFNM(
                                            save.A,
                                            save.A,
                                            save.B,
                                            save.VERTEX.as_slice(),
                                            save.NORMAL.as_slice_mut(),
                                            ctx,
                                        )?;

                                        if (N == 1) {
                                            //
                                            // Find the surface point on the lower
                                            // bounding ellipsoid, at the given
                                            // planetodetic longitude and latitude
                                            // (relative to the reference ellipsoid).

                                            if (save.BOUNDS[[1, 3]] < 0.0) {
                                                //
                                                // The direction to the inner bounding
                                                // ellipsoid is "down."
                                                //
                                                spicelib::VMINUS(
                                                    save.NORMAL.as_slice(),
                                                    save.DIR.as_slice_mut(),
                                                );
                                            } else {
                                                spicelib::VEQU(
                                                    save.NORMAL.as_slice(),
                                                    save.DIR.as_slice_mut(),
                                                );
                                            }

                                            spicelib::SURFPT(
                                                save.VERTEX.as_slice(),
                                                save.DIR.as_slice(),
                                                save.EQLOW,
                                                save.EQLOW,
                                                save.POLLOW,
                                                save.LOWPT.as_slice_mut(),
                                                &mut save.FOUND,
                                                ctx,
                                            )?;
                                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                                            //
                                            // We should never fail to find an
                                            // intercept.
                                            //
                                            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

                                            //
                                            // Find the geodetic coordinates of LOWPT
                                            // with respect to the reference
                                            // ellipsoid.
                                            //
                                            spicelib::RECGEO(
                                                save.LOWPT.as_slice(),
                                                save.A,
                                                save.F,
                                                &mut save.LOWLON,
                                                &mut save.LOWLAT,
                                                &mut save.LOWALT,
                                                ctx,
                                            )?;
                                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                                            //
                                            // Adjust the altitude of LOWPT by a
                                            // small increment.
                                            //
                                            if ((save.EXCLUD == LATIDX) || (save.EXCLUD == NONE)) {
                                                save.ALT = (save.LOWALT
                                                    + (save.EPS * intrinsics::pow(-1, N) as f64));
                                            } else {
                                                save.ALT = (save.LOWALT
                                                    + (save.EPS
                                                        * intrinsics::pow(-1, (N + 1)) as f64));
                                            }
                                        } else {
                                            //
                                            // This is the case for the outer
                                            // bounding ellipsoid (N = 2).
                                            //
                                            if (save.BOUNDS[[2, 3]] < 0.0) {
                                                //
                                                // The direction to the outer bounding
                                                // ellipsoid is "down."
                                                //
                                                spicelib::VMINUS(
                                                    save.NORMAL.as_slice(),
                                                    save.DIR.as_slice_mut(),
                                                );
                                            } else {
                                                spicelib::VEQU(
                                                    save.NORMAL.as_slice(),
                                                    save.DIR.as_slice_mut(),
                                                );
                                            }

                                            spicelib::SURFPT(
                                                save.VERTEX.as_slice(),
                                                save.DIR.as_slice(),
                                                save.EQHI,
                                                save.EQHI,
                                                save.POLHI,
                                                save.HIGHPT.as_slice_mut(),
                                                &mut save.FOUND,
                                                ctx,
                                            )?;
                                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                                            //
                                            // We should never fail to find an
                                            // intercept.
                                            //
                                            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
                                            //
                                            // Find the geodetic coordinates of
                                            // HIGHPT with respect to the reference
                                            // ellipsoid.
                                            //
                                            spicelib::RECGEO(
                                                save.HIGHPT.as_slice(),
                                                save.A,
                                                save.F,
                                                &mut save.HILON,
                                                &mut save.HILAT,
                                                &mut save.HIALT,
                                                ctx,
                                            )?;
                                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                                            //
                                            // Adjust the altitude of HIGHPT by a
                                            // small increment.
                                            //
                                            if ((save.EXCLUD == LATIDX) || (save.EXCLUD == NONE)) {
                                                save.ALT = (save.HIALT
                                                    + (save.EPS * intrinsics::pow(-1, N) as f64));
                                            } else {
                                                save.ALT = (save.HIALT
                                                    + (save.EPS
                                                        * intrinsics::pow(-1, (N + 1)) as f64));
                                            }
                                        }

                                        if ((save.EXCLUD == 2) || (save.EXCLUD == 0)) {
                                            //
                                            // Either all coordinates are considered,
                                            // or latitude is not considered in
                                            // the bounds comparison performed
                                            // by ZZINPDT. We'll move the altitude
                                            // out of bounds.
                                            //

                                            if (N == 1) {
                                                save.ALT = (save.ALT - (0.1 * f64::abs(save.ALT)));
                                            } else {
                                                save.ALT = (save.ALT + (0.1 * f64::abs(save.ALT)));
                                            }
                                        }

                                        //
                                        // Produce the perturbed input point using
                                        // the perturbed geodetic coordinates.
                                        //
                                        spicelib::GEOREC(
                                            save.LON,
                                            save.LAT,
                                            save.ALT,
                                            save.A,
                                            save.F,
                                            save.P2.as_slice_mut(),
                                            ctx,
                                        )?;

                                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                                        spicelib::ZZINPDT(
                                            save.P2.as_slice(),
                                            save.BOUNDS.as_slice(),
                                            save.CORPAR.as_slice(),
                                            save.MARGIN,
                                            save.EXCLUD,
                                            &mut save.INSIDE,
                                            ctx,
                                        )?;
                                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                                        save.XIN = false;

                                        if (save.EXCLUD == 1) {
                                            //
                                            // Longitude is the excluded coordinate;
                                            // this is a case for which we try to
                                            // modify latitude. However, the
                                            // attempted modification may have no
                                            // effect if a boundary is already at
                                            // the extreme value.
                                            //
                                            if ((save.LAT >= (save.BOUNDS[[1, 2]] - ANGMRG))
                                                && (save.LAT <= (save.BOUNDS[[2, 2]] + ANGMRG)))
                                            {
                                                save.XIN = true;
                                            }
                                        } else if (save.EXCLUD == 3) {
                                            if ((save.LON >= (save.BOUNDS[[1, 1]] - ANGMRG))
                                                && (save.LON <= (save.BOUNDS[[2, 1]] + ANGMRG)))
                                            {
                                                save.XIN = true;
                                            }

                                            if (save.BOUNDS[[1, 1]] == save.BOUNDS[[2, 1]]) {
                                                save.XIN = true;
                                            }

                                            if !save.LONBDS {
                                                save.XIN = true;
                                            }

                                            if (LONIX == (NLON - 1)) {
                                                if (L == 2) {
                                                    //
                                                    // In this special case, the input
                                                    // longitude will be outside the
                                                    // longitude boundaries.
                                                    //
                                                    save.XIN = false;
                                                }
                                            }
                                        }

                                        testutil::CHCKSL(
                                            b"INSIDE",
                                            save.INSIDE,
                                            save.XIN,
                                            OK,
                                            ctx,
                                        )?;

                                        //
                                        // Restore the positive margin.
                                        //
                                        save.MARGIN = save.POSMRG;
                                    }
                                    //
                                    // End of "N" loop. N selects the altitude
                                    // upper/lower bound.
                                    //
                                }
                                //
                                // End of "M" loop. M selects the altitude
                                // upper/lower bound.
                                //
                            }
                            //
                            // End of "L" loop. L selects the longitude
                            // upper/lower bound.
                            //

                            save.EXCLUD += m3__;
                        }
                    }
                    //
                    // End of coordinate exclusion (EXCLUD) loop.
                    //
                }
                //
                // End of altitude bound loop.
                //
            }
            //
            // End of upper latitude bound loop.
            //
        }
        //
        // End of lower latitude bound loop.
        //
    }
    //
    // End of longitude loop.
    //

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
