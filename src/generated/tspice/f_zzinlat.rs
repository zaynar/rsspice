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
const NONE: i32 = 0;
const LONIDX: i32 = 1;
const LATIDX: i32 = 2;
const RADIDX: i32 = 3;
const POLMRG: f64 = 0.0001;
const NLAT: i32 = 6;
const NLON: i32 = 9;
const NR: i32 = 4;
const TITLSZ: i32 = 240;

struct SaveVars {
    STEM: Vec<u8>,
    TITLE: Vec<u8>,
    AEPS: f64,
    BOUNDS: StackArray2D<f64, 6>,
    EPS: f64,
    LAT: f64,
    LON: f64,
    MARGIN: f64,
    MAXLAT: StackArray<f64, 6>,
    MAXLON: StackArray<f64, 9>,
    MAXR: StackArray<f64, 4>,
    MIDLAT: f64,
    MIDLON: f64,
    MIDR: f64,
    MINLAT: StackArray<f64, 6>,
    MINLON: StackArray<f64, 9>,
    MINR: StackArray<f64, 4>,
    NRMMAX: f64,
    NRMMIN: f64,
    P: StackArray<f64, 3>,
    R: f64,
    TOL: f64,
    EXCLUD: i32,
    INSIDE: bool,
    LATLB: bool,
    LATUB: bool,
    LONBDS: bool,
    RLB: bool,
    XIN: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut STEM = vec![b' '; TITLSZ as usize];
        let mut TITLE = vec![b' '; TITLSZ as usize];
        let mut AEPS: f64 = 0.0;
        let mut BOUNDS = StackArray2D::<f64, 6>::new(1..=2, 1..=3);
        let mut EPS: f64 = 0.0;
        let mut LAT: f64 = 0.0;
        let mut LON: f64 = 0.0;
        let mut MARGIN: f64 = 0.0;
        let mut MAXLAT = StackArray::<f64, 6>::new(1..=NLAT);
        let mut MAXLON = StackArray::<f64, 9>::new(1..=NLON);
        let mut MAXR = StackArray::<f64, 4>::new(1..=NR);
        let mut MIDLAT: f64 = 0.0;
        let mut MIDLON: f64 = 0.0;
        let mut MIDR: f64 = 0.0;
        let mut MINLAT = StackArray::<f64, 6>::new(1..=NLAT);
        let mut MINLON = StackArray::<f64, 9>::new(1..=NLON);
        let mut MINR = StackArray::<f64, 4>::new(1..=NR);
        let mut NRMMAX: f64 = 0.0;
        let mut NRMMIN: f64 = 0.0;
        let mut P = StackArray::<f64, 3>::new(1..=3);
        let mut R: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut EXCLUD: i32 = 0;
        let mut INSIDE: bool = false;
        let mut LATLB: bool = false;
        let mut LATUB: bool = false;
        let mut LONBDS: bool = false;
        let mut RLB: bool = false;
        let mut XIN: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(-90.0),
                Val::D(-89.999),
                Val::D(-45.0),
                Val::D(0.0),
                Val::D(45.0),
                Val::D(89.999),
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
                Val::D(-89.999),
                Val::D(-45.0),
                Val::D(0.0),
                Val::D(45.0),
                Val::D(89.999),
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
                Val::D(179.999),
                Val::D(-179.999),
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
                Val::D(-179.999),
                Val::D(179.999),
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

            let mut clist =
                [Val::D(0.0), Val::D(0.001), Val::D(1000.0), Val::D(9999.999)].into_iter();
            MINR.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::D(10000.0), 4 as usize))
                .chain([]);

            MAXR.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            STEM,
            TITLE,
            AEPS,
            BOUNDS,
            EPS,
            LAT,
            LON,
            MARGIN,
            MAXLAT,
            MAXLON,
            MAXR,
            MIDLAT,
            MIDLON,
            MIDR,
            MINLAT,
            MINLON,
            MINR,
            NRMMAX,
            NRMMIN,
            P,
            R,
            TOL,
            EXCLUD,
            INSIDE,
            LATLB,
            LATUB,
            LONBDS,
            RLB,
            XIN,
        }
    }
}

//$Procedure F_ZZINLAT ( ZZINLAT tests )
pub fn F_ZZINLAT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    // For the radius boundaries, every valid combination of
    // minimum and maximum will be tested.
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_ZZINLAT", ctx)?;

    //***********************************************************************
    //
    //     Normal cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Initialize coordinate bounds", ctx)?;

    //
    // Convert angular bounds to radians.
    //
    for I in 1..=NLAT {
        save.MINLAT[I] = (save.MINLAT[I] * spicelib::RPD(ctx));
        save.MAXLAT[I] = (save.MAXLAT[I] * spicelib::RPD(ctx));
    }

    for I in 1..=NLON {
        save.MINLON[I] = (save.MINLON[I] * spicelib::RPD(ctx));
        save.MAXLON[I] = (save.MAXLON[I] * spicelib::RPD(ctx));
    }

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Near-origin case: input point is near the origin and the minimum radius bound nearly zero. Interior case.", ctx)?;

    //
    // Longitude bounds:
    //
    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::HALFPI(ctx);

    //
    // The longitude is out of range.
    //
    save.LON = spicelib::PI(ctx);

    //
    // Latitude bounds:
    //
    save.BOUNDS[[1, 2]] = 0.0;
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    //
    // Point's latitude is out of range.
    //
    save.LAT = spicelib::HALFPI(ctx);

    //
    // Radius bounds:
    //
    save.BOUNDS[[1, 3]] = 0.00000001;
    save.BOUNDS[[2, 3]] = 10000.0;

    save.R = 0.000000009;

    //
    // Margin:
    //
    save.MARGIN = 0.00000001;

    //
    // Excluded coordinate: none.
    //
    save.EXCLUD = NONE;

    //
    // Create the input point and test it.
    //
    spicelib::LATREC(save.R, save.LON, save.LAT, save.P.as_slice_mut());

    spicelib::ZZINLAT(
        save.P.as_slice(),
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

    testutil::TCASE(b"Near-origin case: input point is near the origin and the minimum radius bound nearly zero. Exterior case.", ctx)?;

    //
    // Longitude bounds:
    //
    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::HALFPI(ctx);

    //
    // The longitude is out of range.
    //
    save.LON = spicelib::PI(ctx);

    //
    // Latitude bounds:
    //
    save.BOUNDS[[1, 2]] = 0.0;
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    //
    // Point's latitude is out of range.
    //
    save.LAT = spicelib::HALFPI(ctx);

    //
    // Radius bounds:
    //
    save.BOUNDS[[1, 3]] = 0.00000001;
    save.BOUNDS[[2, 3]] = 10000.0;

    save.R = 0.000000009;

    //
    // Margin:
    //
    save.MARGIN = 0.00000000001;

    //
    // Excluded coordinate: none.
    //
    save.EXCLUD = NONE;

    //
    // Create the input point and test it.
    //
    spicelib::LATREC(save.R, save.LON, save.LAT, save.P.as_slice_mut());

    spicelib::ZZINLAT(
        save.P.as_slice(),
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

    testutil::TCASE(b"Near-polar latitude (North) case: radius, latitude, and longitude are within bounds (taking margin into account). No coordinate is excluded.", ctx)?;

    //
    // Excluded coordinate: none.
    //
    save.EXCLUD = NONE;

    //
    // "Standard" margin:
    //
    save.MARGIN = 0.00000001;

    //
    // Longitude bounds:
    //
    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::HALFPI(ctx);

    save.LON = (spicelib::PI(ctx) / 4 as f64);

    //
    // Latitude bounds:
    //
    save.BOUNDS[[1, 2]] = 0.0;
    save.BOUNDS[[2, 2]] = (spicelib::HALFPI(ctx) - 0.000000001);

    //
    // Point's latitude exceeds upper bound by less than MARGIN.
    //
    save.LAT = spicelib::HALFPI(ctx);

    //
    // Radius bounds:
    //
    save.BOUNDS[[1, 3]] = 5000.0;
    save.BOUNDS[[2, 3]] = 10000.0;

    save.R = 10000.0;

    //
    // Margin:
    //
    save.MARGIN = 0.00000001;

    //
    // Excluded coordinate: none.
    //
    save.EXCLUD = NONE;

    //
    // Create the input point and test it.
    //
    spicelib::LATREC(save.R, save.LON, save.LAT, save.P.as_slice_mut());

    spicelib::ZZINLAT(
        save.P.as_slice(),
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
    testutil::TCASE(b"Near-polar latitude (South) case: radius, latitude, and longitude are within bounds (taking margin into account). No coordinate is excluded.", ctx)?;

    //
    // Excluded coordinate: none.
    //
    save.EXCLUD = NONE;

    //
    // "Standard" margin:
    //
    save.MARGIN = 0.00000001;
    //
    // Latitude bounds:
    //
    save.BOUNDS[[1, 2]] = (-spicelib::HALFPI(ctx) + 0.000000001);
    save.BOUNDS[[2, 2]] = 0.0;
    //
    // Point's latitude exceeds lower bound by less than MARGIN.
    //
    save.LAT = -spicelib::HALFPI(ctx);

    //
    // Create the input point and test it.
    //
    spicelib::LATREC(save.R, save.LON, save.LAT, save.P.as_slice_mut());

    spicelib::ZZINLAT(
        save.P.as_slice(),
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

    testutil::TCASE(b"Near-polar latitude (North) case: radius and latitude are within bounds; longitude is not. No coordinate is excluded.", ctx)?;

    //
    // Excluded coordinate: none.
    //
    save.EXCLUD = NONE;

    //
    // "Standard" margin:
    //
    save.MARGIN = 0.00000001;

    //
    // Latitude bounds:
    //
    save.BOUNDS[[1, 2]] = 0.0;
    save.BOUNDS[[2, 2]] = (spicelib::HALFPI(ctx) - 0.000000001);

    //
    // Point's latitude exceeds lower bound by less than MARGIN.
    //
    save.LAT = spicelib::HALFPI(ctx);

    //
    // Set longitude to a value that's out of bounds. This should
    // not affect the comparison, due to the high latitude bound.
    //
    save.LON = spicelib::PI(ctx);
    //
    // Create the input point and test it.
    //
    spicelib::LATREC(save.R, save.LON, save.LAT, save.P.as_slice_mut());

    spicelib::ZZINLAT(
        save.P.as_slice(),
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

    testutil::TCASE(b"Near-polar latitude (South) case: radius and latitude are within bounds; longitude is not. No coordinate is excluded.", ctx)?;

    //
    // Excluded coordinate: none.
    //
    save.EXCLUD = NONE;
    //
    // "Standard" margin:
    //
    save.MARGIN = 0.00000001;

    //
    // Latitude bounds:
    //
    save.BOUNDS[[1, 2]] = (-spicelib::HALFPI(ctx) + 0.000000001);
    save.BOUNDS[[2, 2]] = 0.0;
    //
    // Point's latitude exceeds lower bound by less than MARGIN.
    //
    save.LAT = -spicelib::HALFPI(ctx);

    //
    // Set longitude to a value that's out of bounds. This should
    // not affect the comparison, due to the high latitude bound.
    //
    save.LON = spicelib::PI(ctx);
    //
    // Create the input point and test it.
    //
    spicelib::LATREC(save.R, save.LON, save.LAT, save.P.as_slice_mut());

    spicelib::ZZINLAT(
        save.P.as_slice(),
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

    testutil::TCASE(b"Near-polar latitude (North) case: radius is out of bounds; latitude and longitude are not. No coordinate is excluded.", ctx)?;

    //
    // Set EXCLUD to indicate no exclusion.
    //
    save.EXCLUD = NONE;

    //
    // "Standard" margin:
    //
    save.MARGIN = 0.00000001;

    //
    // Latitude bounds:
    //
    save.BOUNDS[[1, 2]] = 0.0;
    save.BOUNDS[[2, 2]] = (spicelib::HALFPI(ctx) - 0.000000001);

    //
    // Point's latitude exceeds lower bound by less than MARGIN.
    //
    save.LAT = spicelib::HALFPI(ctx);

    //
    // Set longitude to a value that's in bounds.
    //
    save.LON = (spicelib::PI(ctx) / 4 as f64);

    //
    // Radius bounds:
    //
    save.BOUNDS[[1, 3]] = 10000.0;
    save.BOUNDS[[2, 3]] = 30000.0;

    //
    // Create the input point and test it.
    //

    //
    // R is too large.
    //
    save.R = (save.BOUNDS[[2, 3]] * (1.0 + (1.1 * save.MARGIN)));

    spicelib::LATREC(save.R, save.LON, save.LAT, save.P.as_slice_mut());

    spicelib::ZZINLAT(
        save.P.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XIN = false;

    testutil::CHCKSL(b"INSIDE (large)", save.INSIDE, save.XIN, OK, ctx)?;

    //
    // R is too small.
    //
    save.R = (save.BOUNDS[[1, 3]] * (1.0 - (1.1 * save.MARGIN)));

    spicelib::LATREC(save.R, save.LON, save.LAT, save.P.as_slice_mut());

    spicelib::ZZINLAT(
        save.P.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XIN = false;

    testutil::CHCKSL(b"INSIDE (small)", save.INSIDE, save.XIN, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Near-polar latitude (South) case: radius is out of bounds; latitude and longitude are not. No coordinate is excluded.", ctx)?;

    //
    // Set EXCLUD to indicate no exclusion.
    //
    save.EXCLUD = NONE;

    //
    // "Standard" margin:
    //
    save.MARGIN = 0.00000001;

    //
    // Latitude bounds:
    //
    save.BOUNDS[[1, 2]] = (-spicelib::HALFPI(ctx) + 0.000000001);
    save.BOUNDS[[2, 2]] = 0.0;

    //
    // Point's latitude exceeds lower bound by less than MARGIN.
    //
    save.LAT = -spicelib::HALFPI(ctx);

    //
    // Set longitude to a value that's in bounds.
    //
    save.LON = (spicelib::PI(ctx) / 4 as f64);

    //
    // Radius bounds:
    //
    save.BOUNDS[[1, 3]] = 10000.0;
    save.BOUNDS[[2, 3]] = 30000.0;

    //
    // Create the input point and test it.
    //

    //
    // R is too large.
    //
    save.R = (save.BOUNDS[[2, 3]] * (1.0 + (1.1 * save.MARGIN)));

    spicelib::LATREC(save.R, save.LON, save.LAT, save.P.as_slice_mut());

    spicelib::ZZINLAT(
        save.P.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XIN = false;

    testutil::CHCKSL(b"INSIDE (large)", save.INSIDE, save.XIN, OK, ctx)?;

    //
    // R is too small.
    //
    save.R = (save.BOUNDS[[1, 3]] * (1.0 - (1.1 * save.MARGIN)));

    spicelib::LATREC(save.R, save.LON, save.LAT, save.P.as_slice_mut());

    spicelib::ZZINLAT(
        save.P.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XIN = false;

    testutil::CHCKSL(b"INSIDE (small)", save.INSIDE, save.XIN, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Near-polar latitude (North) case: radius and longitude are within bounds; latitude is not. No coordinate is excluded.", ctx)?;

    //
    // Longitude bounds:
    //
    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::HALFPI(ctx);

    save.LON = (spicelib::PI(ctx) / 4 as f64);

    //
    // Margin:
    //
    save.MARGIN = 0.00000001;

    //
    // Latitude bounds:
    //
    save.BOUNDS[[1, 2]] = 0.0;
    save.BOUNDS[[2, 2]] = (spicelib::HALFPI(ctx) - 0.0000001);

    //
    // Point's latitude exceeds upper bound by more than MARGIN.
    //
    save.LAT = (spicelib::HALFPI(ctx) - 0.0000000001);

    //
    // Radius bounds:
    //
    save.BOUNDS[[1, 3]] = 10000.0;
    save.BOUNDS[[2, 3]] = 30000.0;

    save.R = 20000.0;

    //
    // Set EXCLUD to indicate latitude no exclusion.
    //
    save.EXCLUD = NONE;

    //
    // Create the input point and test it.
    //
    spicelib::LATREC(save.R, save.LON, save.LAT, save.P.as_slice_mut());

    spicelib::ZZINLAT(
        save.P.as_slice(),
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

    testutil::TCASE(b"Near-polar latitude (South) case: radius and longitude are within bounds; latitude is not. No coordinate is excluded.", ctx)?;

    //
    // Longitude bounds:
    //
    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::HALFPI(ctx);

    save.LON = (spicelib::PI(ctx) / 4 as f64);

    //
    // Margin:
    //
    save.MARGIN = 0.00000001;

    //
    // Latitude bounds:
    //
    save.BOUNDS[[1, 2]] = (-spicelib::HALFPI(ctx) + 0.0000001);
    save.BOUNDS[[2, 2]] = 0.0;

    //
    // Point's latitude exceeds lower bound by more than MARGIN.
    //
    save.LAT = (-spicelib::HALFPI(ctx) + 0.0000000001);

    //
    // Radius bounds:
    //
    save.BOUNDS[[1, 3]] = 5000.0;
    save.BOUNDS[[2, 3]] = 10000.0;

    save.R = 10000.0;

    //
    // Set EXCLUD to indicate latitude no exclusion.
    //
    save.EXCLUD = NONE;

    //
    // Create the input point and test it.
    //
    spicelib::LATREC(save.R, save.LON, save.LAT, save.P.as_slice_mut());

    spicelib::ZZINLAT(
        save.P.as_slice(),
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

    testutil::TCASE(b"Near-polar latitude (North) case: radius and longitude are out of bounds; latitude is not. Radius is excluded.", ctx)?;

    //
    // Set EXCLUD to exclude radius testing.
    //
    save.EXCLUD = RADIDX;

    //
    // Margin:
    //
    save.MARGIN = 0.00000001;

    //
    // Latitude bounds:
    //
    save.BOUNDS[[1, 2]] = 0.0;
    save.BOUNDS[[2, 2]] = (spicelib::HALFPI(ctx) - 0.000000001);

    //
    // Point's latitude exceeds lower bound by less than MARGIN.
    //
    save.LAT = spicelib::HALFPI(ctx);

    //
    // Longitude bounds:
    //
    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::HALFPI(ctx);

    save.LON = spicelib::PI(ctx);

    //
    // Radius bounds:
    //
    save.BOUNDS[[1, 3]] = 10000.0;
    save.BOUNDS[[2, 3]] = 30000.0;

    //
    // Create the input point and test it.
    //

    //
    // R is too large.
    //
    save.R = (save.BOUNDS[[2, 3]] * (1.0 + (1.1 * save.MARGIN)));

    spicelib::LATREC(save.R, save.LON, save.LAT, save.P.as_slice_mut());

    spicelib::ZZINLAT(
        save.P.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XIN = true;

    testutil::CHCKSL(b"INSIDE (large)", save.INSIDE, save.XIN, OK, ctx)?;

    //
    // R is too small.
    //
    save.R = (save.BOUNDS[[1, 3]] * (1.0 - (1.1 * save.MARGIN)));

    spicelib::LATREC(save.R, save.LON, save.LAT, save.P.as_slice_mut());

    spicelib::ZZINLAT(
        save.P.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XIN = true;

    testutil::CHCKSL(b"INSIDE (small)", save.INSIDE, save.XIN, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Near-polar latitude (South) case: radius and longitude are out of bounds; latitude is not. Radius is excluded.", ctx)?;

    //
    // Set EXCLUD to exclude radius testing.
    //
    save.EXCLUD = RADIDX;

    //
    // Latitude bounds:
    //
    save.BOUNDS[[1, 2]] = (-spicelib::HALFPI(ctx) + 0.000000001);
    save.BOUNDS[[2, 2]] = 0.0;

    //
    // Point's latitude is less than lower bound by less than MARGIN.
    //
    save.LAT = -spicelib::HALFPI(ctx);

    //
    // Longitude bounds:
    //
    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::HALFPI(ctx);

    save.LON = spicelib::PI(ctx);

    //
    // Radius bounds:
    //
    save.BOUNDS[[1, 3]] = 10000.0;
    save.BOUNDS[[2, 3]] = 30000.0;

    //
    // Create the input point and test it.
    //

    //
    // R is too large.
    //
    save.R = (save.BOUNDS[[2, 3]] * (1.0 + (1.1 * save.MARGIN)));

    spicelib::LATREC(save.R, save.LON, save.LAT, save.P.as_slice_mut());

    spicelib::ZZINLAT(
        save.P.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XIN = true;

    testutil::CHCKSL(b"INSIDE (large)", save.INSIDE, save.XIN, OK, ctx)?;

    //
    // R is too small.
    //
    save.R = (save.BOUNDS[[1, 3]] * (1.0 - (1.1 * save.MARGIN)));

    spicelib::LATREC(save.R, save.LON, save.LAT, save.P.as_slice_mut());

    spicelib::ZZINLAT(
        save.P.as_slice(),
        save.BOUNDS.as_slice(),
        save.MARGIN,
        save.EXCLUD,
        &mut save.INSIDE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XIN = true;

    testutil::CHCKSL(b"INSIDE (small)", save.INSIDE, save.XIN, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Near-polar latitude (North) case: latitude is out of bounds; radius and longitude are not. Latitude is excluded.", ctx)?;

    //
    // Set EXCLUD to exclude latitude testing.
    //
    save.EXCLUD = LATIDX;

    //
    // Margin:
    //
    save.MARGIN = 0.00000001;

    //
    // Latitude bounds:
    //
    save.BOUNDS[[1, 2]] = 0.0;
    save.BOUNDS[[2, 2]] = (spicelib::HALFPI(ctx) - 0.00000002);

    //
    // Point's latitude exceeds upper bound by more than MARGIN.
    //
    save.LAT = spicelib::HALFPI(ctx);

    //
    // Longitude bounds:
    //
    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::HALFPI(ctx);

    save.LON = (spicelib::PI(ctx) / 4 as f64);

    //
    // R is in range.
    //
    save.R = ((save.BOUNDS[[1, 3]] + save.BOUNDS[[2, 3]]) / 2 as f64);
    //
    // Create the input point and test it.
    //

    spicelib::LATREC(save.R, save.LON, save.LAT, save.P.as_slice_mut());

    spicelib::ZZINLAT(
        save.P.as_slice(),
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

    testutil::TCASE(b"Near-polar latitude (South) case: latitude is out of bounds; radius and longitude are not. Latitude is excluded.", ctx)?;

    //
    // Set EXCLUD to exclude latitude testing.
    //
    save.EXCLUD = LATIDX;

    //
    // Margin:
    //
    save.MARGIN = 0.00000001;

    //
    // Latitude bounds:
    //
    save.BOUNDS[[1, 2]] = (-spicelib::HALFPI(ctx) + 0.00000002);
    save.BOUNDS[[2, 2]] = 0.0;

    //
    // Point's latitude is below lower bound by more than MARGIN.
    //
    save.LAT = -spicelib::HALFPI(ctx);

    //
    // Longitude bounds:
    //
    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::HALFPI(ctx);

    save.LON = (spicelib::PI(ctx) / 4 as f64);

    //
    // R is in range.
    //
    save.R = ((save.BOUNDS[[1, 3]] + save.BOUNDS[[2, 3]]) / 2 as f64);
    //
    // Create the input point and test it.
    //

    spicelib::LATREC(save.R, save.LON, save.LAT, save.P.as_slice_mut());

    spicelib::ZZINLAT(
        save.P.as_slice(),
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

    testutil::TCASE(b"Longitude acceptance test for longitude lower than lower bound by less than ANGMRG. Other coordinates are in range. Non-polar case.", ctx)?;

    //
    // Margin: positive but small enough so that ANGMRG is larger.
    //
    save.MARGIN = 0.00000000000001;

    //
    // Longitude bounds:
    //
    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::HALFPI(ctx);

    //
    // The point's longitude is outside the element by more than ANGMRG
    // but by less than (ANGMRG+LONALI).
    //
    save.LON = ((save.BOUNDS[[1, 1]] - ANGMRG) - 0.0000000000001);

    //
    // Latitude bounds:
    //
    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    //
    // Point's latitude is in range.
    //
    save.LAT = 0.0;

    //
    // Radius bounds:
    //
    save.BOUNDS[[1, 3]] = 10000.0;
    save.BOUNDS[[2, 3]] = 30000.0;

    //
    // Point's radius is in range.
    //
    save.R = 20000.0;

    //
    // Check all coordinates.
    //
    save.EXCLUD = NONE;

    //
    // Create the input point and test it.
    //
    spicelib::LATREC(save.R, save.LON, save.LAT, save.P.as_slice_mut());

    spicelib::ZZINLAT(
        save.P.as_slice(),
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

    testutil::TCASE(b"Longitude acceptance test for longitude greater than upper bound by less than ANGMRG. Other coordinates are in range. Non-polar case.", ctx)?;

    //
    // Margin: positive but small enough so that ANGMRG is larger.
    //
    save.MARGIN = 0.00000000000001;

    //
    // Longitude bounds:
    //
    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::HALFPI(ctx);

    //
    // The point's longitude is outside the element by more than ANGMRG
    // but by less than (ANGMRG+LONALI).
    //
    save.LON = ((save.BOUNDS[[2, 1]] + ANGMRG) + 0.0000000000001);

    //
    // Latitude bounds:
    //
    save.BOUNDS[[1, 2]] = -(spicelib::PI(ctx) / 4 as f64);
    save.BOUNDS[[2, 2]] = (spicelib::PI(ctx) / 4 as f64);

    //
    // Point's latitude is in range.
    //
    save.LAT = 0.0;

    //
    // Radius bounds:
    //
    save.BOUNDS[[1, 3]] = 10000.0;
    save.BOUNDS[[2, 3]] = 30000.0;

    //
    // Point's radius is in range.
    //
    save.R = 20000.0;

    //
    // Check all coordinates.
    //
    save.EXCLUD = NONE;

    //
    // Create the input point and test it.
    //
    spicelib::LATREC(save.R, save.LON, save.LAT, save.P.as_slice_mut());

    spicelib::ZZINLAT(
        save.P.as_slice(),
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
    //
    //     Systematic cases:
    //
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Choose a positive margin.
    //
    save.MARGIN = 0.000001;

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

                for RIX1 in 1..=NR {
                    save.BOUNDS[[1, 3]] = save.MINR[RIX1];
                    //
                    // Indicate whether the lower radius boundary is a
                    // surface.
                    //
                    save.RLB = (save.MINR[RIX1] > 0.0);

                    for RIX2 in RIX1..=NR {
                        save.BOUNDS[[2, 3]] = save.MAXR[RIX2];

                        {
                            let m1__: i32 = 0;
                            let m2__: i32 = 3;
                            let m3__: i32 = 1;
                            save.EXCLUD = m1__;
                            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                                //
                                //-- Case: ------------------------------------------------------
                                //

                                //
                                // Set the input point so that each coordinate
                                // is the midpoint of the element's range for
                                // that coordinate.
                                //

                                fstr::assign(
                                    &mut save.STEM,
                                    b"Lon #:#; Lat #:#; Rad #:#; EXCLUD = #;",
                                );

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

                                //
                                //-- Case: ------------------------------------------------------
                                //

                                fstr::assign(&mut save.TITLE, &save.STEM);
                                spicelib::SUFFIX(b"Midpoint case", 1, &mut save.TITLE);
                                testutil::TCASE(&save.TITLE, ctx)?;

                                //
                                // CALL TOSTDO ( TITLE )
                                //
                                save.MIDLON = ((save.NRMMIN + save.NRMMAX) / 2 as f64);
                                save.MIDLAT =
                                    ((save.BOUNDS[[1, 2]] + save.BOUNDS[[2, 2]]) / 2 as f64);
                                save.MIDR =
                                    ((save.BOUNDS[[1, 3]] + save.BOUNDS[[2, 3]]) / 2 as f64);

                                spicelib::LATREC(
                                    save.MIDR,
                                    save.MIDLON,
                                    save.MIDLAT,
                                    save.P.as_slice_mut(),
                                );

                                spicelib::ZZINLAT(
                                    save.P.as_slice(),
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
                                //-- Case: ------------------------------------------------------
                                //
                                //
                                //                       Check interior points near each corner of
                                //                       the volume element.
                                //
                                for L in 1..=2 {
                                    for M in 1..=2 {
                                        for N in 1..=2 {
                                            //
                                            //-- Case: ------------------------------------------------------
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
                                            // Set incremental offsets.
                                            //
                                            save.AEPS = ((1.0 + 0.000001) * save.MARGIN);
                                            save.EPS = ((1.0 + 0.000001) * save.MARGIN);

                                            //
                                            // Multiply the small increments by 1 or
                                            // -1 as needed, depending on whether they
                                            // are used as offsets from upper or lower
                                            // bounds.
                                            //
                                            save.LAT = (save.BOUNDS[[M, 2]]
                                                + (save.AEPS
                                                    * intrinsics::pow(-1, (M + 1)) as f64));
                                            save.R = (save.BOUNDS[[N, 3]]
                                                + (save.EPS * intrinsics::pow(-1, (N + 1)) as f64));

                                            if (L == 1) {
                                                save.LON = (save.NRMMIN + save.AEPS);
                                            } else {
                                                save.LON = (save.NRMMAX - save.AEPS);
                                            }

                                            spicelib::LATREC(
                                                save.R,
                                                save.LON,
                                                save.LAT,
                                                save.P.as_slice_mut(),
                                            );

                                            spicelib::ZZINLAT(
                                                save.P.as_slice(),
                                                save.BOUNDS.as_slice(),
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
                                            //-- Case: ------------------------------------------------------
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
                                            // Set incremental offsets.
                                            //
                                            save.AEPS = ((1.0 + 0.001) * save.MARGIN);
                                            save.EPS = ((1.0 + 0.001) * save.MARGIN);

                                            //
                                            // Multiply the small increments by 1 or
                                            // -1 as needed, depending on whether they
                                            // are used as offsets from upper or lower
                                            // bounds.
                                            //
                                            save.LAT = (save.BOUNDS[[M, 2]]
                                                + (save.AEPS
                                                    * intrinsics::pow(-1, (M + 1)) as f64));

                                            if (save.EXCLUD == 2) {
                                                //
                                                // Latitude is not considered in
                                                // the bounds comparison performed
                                                // by ZZINLAT.
                                                //
                                                if (M == 1) {
                                                    save.LAT = (-spicelib::HALFPI(ctx)
                                                        + ((10 as f64) * POLMRG));
                                                } else {
                                                    save.LAT = (spicelib::HALFPI(ctx)
                                                        - ((10 as f64) * POLMRG));
                                                }
                                            }

                                            save.R = (save.BOUNDS[[N, 3]]
                                                + (save.EPS * intrinsics::pow(-1, (N + 1)) as f64));

                                            if (save.EXCLUD == 3) {
                                                //
                                                // Radius is not considered in
                                                // the bounds comparison performed
                                                // by ZZINLAT.
                                                //
                                                if (N == 1) {
                                                    //
                                                    // Set R to a small value, but
                                                    // not to zero, because in that
                                                    // case the point's longitude
                                                    // and latitude won't be reliably
                                                    // recoverable.
                                                    //
                                                    save.R = 0.000001;
                                                } else {
                                                    save.R = (save.BOUNDS[[2, 3]] * 2 as f64);
                                                }
                                            }

                                            if (L == 1) {
                                                save.LON = (save.NRMMIN + save.AEPS);
                                            } else {
                                                save.LON = (save.NRMMAX - save.AEPS);
                                            }

                                            if (save.EXCLUD == 1) {
                                                //
                                                // Longitude is not considered in
                                                // the bounds comparison performed
                                                // by ZZINLAT.
                                                //
                                                if (N == 1) {
                                                    save.LON = (save.NRMMIN - save.AEPS);
                                                } else {
                                                    save.LON = (save.NRMMAX + save.AEPS);
                                                }
                                            }

                                            spicelib::LATREC(
                                                save.R,
                                                save.LON,
                                                save.LAT,
                                                save.P.as_slice_mut(),
                                            );

                                            spicelib::ZZINLAT(
                                                save.P.as_slice(),
                                                save.BOUNDS.as_slice(),
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
                                            //-- Case: ------------------------------------------------------
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
                                            // Give XIN its default value. In most
                                            // cases, we'll reset it to .FALSE.
                                            //
                                            save.XIN = true;

                                            //
                                            // Set incremental offsets.
                                            //
                                            save.AEPS = ((1.0 + 0.001) * save.MARGIN);
                                            save.EPS = ((1.0 + 0.001) * save.MARGIN);

                                            if (save.EXCLUD == 1) {
                                                //
                                                // Set the latitude value out of range.

                                                if (M == 1) {
                                                    save.LAT = -spicelib::HALFPI(ctx);
                                                    //
                                                    // If we managed to set the
                                                    // latitude to a value below the
                                                    // lower latitude bound, then we
                                                    // have an exterior point.
                                                    //
                                                    save.XIN = !save.LATLB;
                                                } else {
                                                    save.LAT = spicelib::HALFPI(ctx);

                                                    save.XIN = !save.LATUB;
                                                }
                                            } else {
                                                save.LAT = save.MIDLAT;
                                            }

                                            if (save.EXCLUD == 2) {
                                                //
                                                // Set the radius value out of range.
                                                //
                                                if (N == 1) {
                                                    //
                                                    // Set R to a small value, but
                                                    // not to zero, because in that
                                                    // case the point's longitude
                                                    // and latitude won't be reliably
                                                    // recoverable.
                                                    //
                                                    save.R = 0.000001;

                                                    save.XIN = !save.RLB;
                                                } else {
                                                    save.R = (save.BOUNDS[[2, 3]] * 2 as f64);

                                                    save.XIN = false;
                                                }
                                            } else {
                                                save.R = save.MIDR;
                                            }

                                            if (save.EXCLUD == 3) {
                                                //
                                                // Set the longitude value out of
                                                // range.
                                                //
                                                if (L == 1) {
                                                    if (f64::abs(save.LAT)
                                                        < (spicelib::HALFPI(ctx) - POLMRG))
                                                    {
                                                        save.LON = (save.NRMMIN
                                                            - (save.AEPS / f64::cos(save.LAT)));

                                                        save.XIN = !save.LONBDS;
                                                    } else {
                                                        save.XIN = true;
                                                    }
                                                } else {
                                                    if (f64::abs(save.LAT)
                                                        < (spicelib::HALFPI(ctx) - POLMRG))
                                                    {
                                                        save.LON = (save.NRMMAX
                                                            + (save.AEPS / f64::cos(save.LAT)));

                                                        save.XIN = !save.LONBDS;
                                                    } else {
                                                        save.XIN = true;
                                                    }
                                                }
                                            } else {
                                                save.LON = save.MIDLON;
                                            }

                                            spicelib::LATREC(
                                                save.R,
                                                save.LON,
                                                save.LAT,
                                                save.P.as_slice_mut(),
                                            );

                                            spicelib::ZZINLAT(
                                                save.P.as_slice(),
                                                save.BOUNDS.as_slice(),
                                                save.MARGIN,
                                                save.EXCLUD,
                                                &mut save.INSIDE,
                                                ctx,
                                            )?;
                                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                                            //
                                            // XIN has been set.
                                            //
                                            testutil::CHCKSL(
                                                b"INSIDE",
                                                save.INSIDE,
                                                save.XIN,
                                                OK,
                                                ctx,
                                            )?;

                                            //
                                            //-- Case: ------------------------------------------------------
                                            //
                                            fstr::assign(&mut save.TITLE, &save.STEM);
                                            spicelib::SUFFIX(b"Point near corner # # #; exterior; predecessor of excluded coordinate out of range.", 1, &mut save.TITLE);
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
                                            // Give XIN its default value. In most
                                            // cases, we'll reset it to .FALSE.
                                            //
                                            save.XIN = true;

                                            //
                                            // Set incremental offsets.
                                            //
                                            save.AEPS = ((1.0 + 0.001) * save.MARGIN);
                                            save.EPS = ((1.0 + 0.001) * save.MARGIN);

                                            if (save.EXCLUD == 3) {
                                                //
                                                // Set the latitude value out of range.

                                                if (M == 1) {
                                                    save.LAT = -spicelib::HALFPI(ctx);
                                                    //
                                                    // If we managed to set the
                                                    // latitude to a value below the
                                                    // lower latitude bound, then we
                                                    // have an exterior point.
                                                    //
                                                    save.XIN = !save.LATLB;
                                                } else {
                                                    save.LAT = spicelib::HALFPI(ctx);

                                                    save.XIN = !save.LATUB;
                                                }
                                            } else {
                                                save.LAT = save.MIDLAT;
                                            }

                                            if (save.EXCLUD == 1) {
                                                //
                                                // Set the radius value out of range.
                                                //
                                                if (N == 1) {
                                                    //
                                                    // Set R to a small value, but
                                                    // not to zero, because in that
                                                    // case the point's longitude
                                                    // and latitude won't be reliably
                                                    // recoverable.
                                                    //
                                                    save.R = 0.000001;

                                                    save.XIN = !save.RLB;
                                                } else {
                                                    save.R = (save.BOUNDS[[2, 3]] * 2 as f64);

                                                    save.XIN = false;
                                                }
                                            } else {
                                                save.R = save.MIDR;
                                            }

                                            if (save.EXCLUD == 2) {
                                                //
                                                // Set the longitude value out of
                                                // range.
                                                //
                                                if (L == 1) {
                                                    if (f64::abs(save.LAT)
                                                        < (spicelib::HALFPI(ctx) - POLMRG))
                                                    {
                                                        save.LON = (save.NRMMIN
                                                            - (save.AEPS / f64::cos(save.LAT)));

                                                        save.XIN = !save.LONBDS;
                                                    } else {
                                                        save.XIN = true;
                                                    }
                                                } else {
                                                    if (f64::abs(save.LAT)
                                                        < (spicelib::HALFPI(ctx) - POLMRG))
                                                    {
                                                        save.LON = (save.NRMMAX
                                                            + (save.AEPS / f64::cos(save.LAT)));

                                                        save.XIN = !save.LONBDS;
                                                    } else {
                                                        save.XIN = true;
                                                    }
                                                }
                                            } else {
                                                save.LON = save.MIDLON;
                                            }

                                            spicelib::LATREC(
                                                save.R,
                                                save.LON,
                                                save.LAT,
                                                save.P.as_slice_mut(),
                                            );

                                            spicelib::ZZINLAT(
                                                save.P.as_slice(),
                                                save.BOUNDS.as_slice(),
                                                save.MARGIN,
                                                save.EXCLUD,
                                                &mut save.INSIDE,
                                                ctx,
                                            )?;
                                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                                            //
                                            // XIN has been set.
                                            //
                                            testutil::CHCKSL(
                                                b"INSIDE",
                                                save.INSIDE,
                                                save.XIN,
                                                OK,
                                                ctx,
                                            )?;
                                        }
                                        //
                                        // End of "N" loop. N selects the radius
                                        // upper/lower bound.
                                        //
                                    }
                                    //
                                    // End of "M" loop. M selects the radius
                                    // upper/lower bound.
                                    //
                                }
                                //
                                // End of "L" loop. L selects the longitude
                                // upper/lower bound.

                                save.EXCLUD += m3__;
                            }
                        }
                        //
                        // End of coordinate exclusion (EXCLUD) loop.
                        //
                    }
                    //
                    // End of upper radius bound loop.
                    //
                }
                //
                // End of lower radius bound loop.
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

    //***********************************************************************
    //
    //     Error cases
    //
    //***********************************************************************

    //
    // ZZINLAT is "error free."
    //

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
