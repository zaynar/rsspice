//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const PCK: &[u8] = b"edterm.tpc";
const SPK: &[u8] = b"edterm.bsp";
const TIGHT: f64 = 0.0000000000001;
const TOL: f64 = TIGHT;
const BDNMLN: i32 = 36;
const CORLEN: i32 = 10;
const FRNMLN: i32 = 32;
const LNSIZE: i32 = 80;
const MAXPT: i32 = 1000;

//$Procedure      F_TERM ( Test terminator routines )
pub fn F_TERM(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ABCORR = [b' '; CORLEN as usize];
    let mut FIXREF = [b' '; FRNMLN as usize];
    let mut OBSRVR = [b' '; BDNMLN as usize];
    let mut SOURCE = [b' '; BDNMLN as usize];
    let mut TARGET = [b' '; BDNMLN as usize];
    let mut TITLE = [b' '; LNSIZE as usize];
    let mut A: f64 = 0.0;
    let mut B: f64 = 0.0;
    let mut BADRAD = StackArray::<f64, 3>::new(1..=3);
    let mut C: f64 = 0.0;
    let mut D: f64 = 0.0;
    let mut ET: f64 = 0.0;
    let mut GRIDPT = StackArray::<f64, 3>::new(1..=3);
    let mut LAT: f64 = 0.0;
    let mut LON: f64 = 0.0;
    let mut LONDIF: f64 = 0.0;
    let mut LT: f64 = 0.0;
    let mut OBSPOS = StackArray::<f64, 3>::new(1..=3);
    let mut PRVLON: f64 = 0.0;
    let mut R: f64 = 0.0;
    let mut RADII = StackArray::<f64, 3>::new(1..=3);
    let mut RADIUS: f64 = 0.0;
    let mut SRADII = StackArray::<f64, 3>::new(1..=3);
    let mut SRCPOS = StackArray::<f64, 3>::new(1..=3);
    let mut TRANS = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut TRGEPC: f64 = 0.0;
    let mut TRGPOS = StackArray::<f64, 3>::new(1..=3);
    let mut TRMGRD = ActualArray2D::<f64>::new(1..=3, 1..=MAXPT);
    let mut XDIFF: f64 = 0.0;
    let mut XGRID = ActualArray2D::<f64>::new(1..=3, 1..=MAXPT);
    let mut XOBSPS = StackArray::<f64, 3>::new(1..=3);
    let mut HANDLE: i32 = 0;
    let mut N: i32 = 0;
    let mut NPTS: i32 = 0;
    let mut UMBRAL: bool = false;

    //
    // Test Utility Functions
    //

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local Variables
    //

    // DOUBLE PRECISION      XFORM  ( 3, 3 )

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_TERM", ctx)?;

    //******************************************************************
    //
    //     ZZEDTERM tests
    //
    //******************************************************************

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZEDTERM: Spherical target, D/R = 2, on z-axis, umbral case",
        ctx,
    )?;

    D = 2000000.0;

    spicelib::VPACK(0.0, 0.0, D, SRCPOS.as_slice_mut());

    A = 10000.0;
    B = A;
    C = A;

    R = ((D / 2 as f64) + A);

    NPTS = 360;

    spicelib::ZZEDTERM(
        b"UMBRAL",
        A,
        B,
        C,
        R,
        SRCPOS.as_slice(),
        NPTS,
        TRMGRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure each point found is a terminator point.
    //
    UMBRAL = true;
    PRVLON = 0.0;

    for I in 1..=NPTS {
        fstr::assign(&mut TITLE, b"Point *,");
        spicelib::REPMI(&TITLE.clone(), b"*", I, &mut TITLE, ctx);

        //
        // Check that the grid point is a tangency point of
        // a plane that is also tangent to the light source.
        // Also check that the center of the source is on the
        // correct side of the plane.
        //
        T_ETERM(
            &TITLE,
            UMBRAL,
            SRCPOS.as_slice(),
            R,
            A,
            B,
            C,
            TRMGRD.subarray([1, I]),
            OK,
            ctx,
        )?;

        //
        // Validate spacing of grid points:  longitude deltas
        // should be constant.  To avoid problems with singularities,
        // we work in a frame where SRCPOS is on the +Z axis and
        // the component of the first grid point orthogonal to the
        // +Z axis lies on the +X axis.
        //
        if (I == 1) {
            spicelib::TWOVEC(
                SRCPOS.as_slice(),
                3,
                TRMGRD.subarray([1, 1]),
                1,
                TRANS.as_slice_mut(),
                ctx,
            )?;
        }

        spicelib::MXV(
            TRANS.as_slice(),
            TRMGRD.subarray([1, I]),
            GRIDPT.as_slice_mut(),
        );

        spicelib::RECLAT(GRIDPT.as_slice(), &mut RADIUS, &mut LON, &mut LAT);

        if (LON < 0.0) {
            LON = (LON + spicelib::TWOPI(ctx));
        }

        if (I == 2) {
            //
            // We don't know the expected sign of the difference, but we
            // know the expected magnitude.
            //
            XDIFF = (LON - PRVLON);

            if (XDIFF > spicelib::PI(ctx)) {
                XDIFF = (XDIFF - spicelib::TWOPI(ctx));
            }

            testutil::CHCKSD(
                b"ABS(XDIFF)",
                f64::abs(XDIFF),
                b"~",
                (spicelib::TWOPI(ctx) / NPTS as f64),
                TOL,
                OK,
                ctx,
            )?;
        } else if (I > 2) {
            LONDIF = (LON - PRVLON);

            testutil::CHCKSD(b"LONDIF", LONDIF, b"~", XDIFF, TOL, OK, ctx)?;
        }

        // WRITE (*,*) RADIUS,LON*DPR(), LAT*DPR()

        PRVLON = LON;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZEDTERM: Spherical target, D/R = 2, on z-axis, penumbral case",
        ctx,
    )?;

    D = 2000000.0;

    spicelib::VPACK(0.0, 0.0, D, SRCPOS.as_slice_mut());

    A = 10000.0;
    B = A;
    C = A;

    R = ((D / 2 as f64) + A);

    NPTS = 6;

    spicelib::ZZEDTERM(
        b"PENUMBRAL",
        A,
        B,
        C,
        R,
        SRCPOS.as_slice(),
        NPTS,
        TRMGRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure each point found is a terminator point.
    //
    UMBRAL = false;

    for I in 1..=NPTS {
        fstr::assign(&mut TITLE, b"Point *,");
        spicelib::REPMI(&TITLE.clone(), b"*", I, &mut TITLE, ctx);

        T_ETERM(
            &TITLE,
            UMBRAL,
            SRCPOS.as_slice(),
            R,
            A,
            B,
            C,
            TRMGRD.subarray([1, I]),
            OK,
            ctx,
        )?;

        spicelib::RECLAT(TRMGRD.subarray([1, I]), &mut RADIUS, &mut LON, &mut LAT);
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZEDTERM: Slightly prolate target, D/R = 2, on z-axis",
        ctx,
    )?;

    D = 2000000.0;

    spicelib::VPACK(0.0, 0.0, D, SRCPOS.as_slice_mut());

    A = 10000.0;
    B = (A - 1000.0);
    C = (A - 1000.0);

    R = (1000000.0 + A);

    NPTS = MAXPT;
    UMBRAL = true;

    spicelib::ZZEDTERM(
        b"UMBRAL",
        A,
        B,
        C,
        R,
        SRCPOS.as_slice(),
        NPTS,
        TRMGRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=NPTS {
        fstr::assign(&mut TITLE, b"Point *,");
        spicelib::REPMI(&TITLE.clone(), b"*", I, &mut TITLE, ctx);

        T_ETERM(
            &TITLE,
            UMBRAL,
            SRCPOS.as_slice(),
            R,
            A,
            B,
            C,
            TRMGRD.subarray([1, I]),
            OK,
            ctx,
        )?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZEDTERM: Geometry comparable to Sun-Earth, umbral case",
        ctx,
    )?;

    D = 150000000.0;

    spicelib::VPACK(D, 0.0, 0.0, SRCPOS.as_slice_mut());

    A = 6378.0;
    B = A;
    C = 6357.0;

    R = 1000000.0;

    NPTS = 360;

    spicelib::ZZEDTERM(
        b"UMBRAL",
        A,
        B,
        C,
        R,
        SRCPOS.as_slice(),
        NPTS,
        TRMGRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure each point found is a terminator point.
    //
    UMBRAL = true;

    for I in 1..=NPTS {
        fstr::assign(&mut TITLE, b"Point *,");
        spicelib::REPMI(&TITLE.clone(), b"*", I, &mut TITLE, ctx);

        T_ETERM(
            &TITLE,
            UMBRAL,
            SRCPOS.as_slice(),
            R,
            A,
            B,
            C,
            TRMGRD.subarray([1, I]),
            OK,
            ctx,
        )?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZEDTERM: Geometry comparable to Sun-Earth, penumbral case",
        ctx,
    )?;

    D = 150000000.0;

    spicelib::VPACK(D, 0.0, 0.0, SRCPOS.as_slice_mut());

    A = 6378.0;
    B = A;
    C = 6357.0;

    R = 1000000.0;

    NPTS = 360;
    UMBRAL = false;

    spicelib::ZZEDTERM(
        b"PENUMBRAL",
        A,
        B,
        C,
        R,
        SRCPOS.as_slice(),
        NPTS,
        TRMGRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure each point found is a terminator point.
    //
    for I in 1..=NPTS {
        fstr::assign(&mut TITLE, b"Point *,");
        spicelib::REPMI(&TITLE.clone(), b"*", I, &mut TITLE, ctx);

        T_ETERM(
            &TITLE,
            UMBRAL,
            SRCPOS.as_slice(),
            R,
            A,
            B,
            C,
            TRMGRD.subarray([1, I]),
            OK,
            ctx,
        )?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZEDTERM: Source and target have equal size, umbral case",
        ctx,
    )?;

    D = 150000000.0;

    spicelib::VPACK(D, 0.0, 0.0, SRCPOS.as_slice_mut());

    A = 6378.0;
    B = A;
    C = A;

    R = A;

    NPTS = 360;
    UMBRAL = true;

    spicelib::ZZEDTERM(
        b"UMBRAL",
        A,
        B,
        C,
        R,
        SRCPOS.as_slice(),
        NPTS,
        TRMGRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure each point found is a terminator point.
    //
    for I in 1..=NPTS {
        fstr::assign(&mut TITLE, b"Point *,");
        spicelib::REPMI(&TITLE.clone(), b"*", I, &mut TITLE, ctx);

        T_ETERM(
            &TITLE,
            UMBRAL,
            SRCPOS.as_slice(),
            R,
            A,
            B,
            C,
            TRMGRD.subarray([1, I]),
            OK,
            ctx,
        )?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZEDTERM: Source and target have equal size, penumbral case",
        ctx,
    )?;

    D = 150000000.0;

    spicelib::VPACK(D, 0.0, 0.0, SRCPOS.as_slice_mut());

    A = 6378.0;
    B = A;
    C = A;

    R = A;

    NPTS = 360;
    UMBRAL = false;

    spicelib::ZZEDTERM(
        b"PENUMBRAL",
        A,
        B,
        C,
        R,
        SRCPOS.as_slice(),
        NPTS,
        TRMGRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure each point found is a terminator point.
    //
    for I in 1..=NPTS {
        fstr::assign(&mut TITLE, b"Point *,");
        spicelib::REPMI(&TITLE.clone(), b"*", I, &mut TITLE, ctx);

        T_ETERM(
            &TITLE,
            UMBRAL,
            SRCPOS.as_slice(),
            R,
            A,
            B,
            C,
            TRMGRD.subarray([1, I]),
            OK,
            ctx,
        )?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"ZZEDTERM: Source is smaller than target, umbral case", ctx)?;

    D = 150000000.0;

    spicelib::VPACK(D, 0.0, 0.0, SRCPOS.as_slice_mut());

    A = 6378.0;
    B = A;
    C = A;

    R = (A / 2 as f64);

    NPTS = 360;
    UMBRAL = true;

    spicelib::ZZEDTERM(
        b"UMBRAL",
        A,
        B,
        C,
        R,
        SRCPOS.as_slice(),
        NPTS,
        TRMGRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure each point found is a terminator point.
    //
    for I in 1..=NPTS {
        fstr::assign(&mut TITLE, b"Point *,");
        spicelib::REPMI(&TITLE.clone(), b"*", I, &mut TITLE, ctx);

        T_ETERM(
            &TITLE,
            UMBRAL,
            SRCPOS.as_slice(),
            R,
            A,
            B,
            C,
            TRMGRD.subarray([1, I]),
            OK,
            ctx,
        )?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZEDTERM: Source is smaller than target, penumbral case",
        ctx,
    )?;

    D = 150000000.0;

    spicelib::VPACK(D, 0.0, 0.0, SRCPOS.as_slice_mut());

    A = 6378.0;
    B = A;
    C = A;

    R = (A / 2 as f64);

    NPTS = 360;
    UMBRAL = true;

    spicelib::ZZEDTERM(
        b"UMBRAL",
        A,
        B,
        C,
        R,
        SRCPOS.as_slice(),
        NPTS,
        TRMGRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure each point found is a terminator point.
    //
    for I in 1..=NPTS {
        fstr::assign(&mut TITLE, b"Point *,");
        spicelib::REPMI(&TITLE.clone(), b"*", I, &mut TITLE, ctx);

        T_ETERM(
            &TITLE,
            UMBRAL,
            SRCPOS.as_slice(),
            R,
            A,
            B,
            C,
            TRMGRD.subarray([1, I]),
            OK,
            ctx,
        )?;
    }

    //
    //
    // Error handling tests follow.
    //
    //

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"ZZEDTERM: Bad terminator type.", ctx)?;

    spicelib::ZZEDTERM(
        b"PNUMBRAL",
        A,
        B,
        C,
        R,
        SRCPOS.as_slice(),
        NPTS,
        TRMGRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"ZZEDTERM: Bad grid size", ctx)?;

    NPTS = 0;
    spicelib::ZZEDTERM(
        b"PENUMBRAL",
        A,
        B,
        C,
        R,
        SRCPOS.as_slice(),
        NPTS,
        TRMGRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDSIZE)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"ZZEDTERM: Bad ellipsoid axis", ctx)?;

    NPTS = 4;

    spicelib::ZZEDTERM(
        b"PENUMBRAL",
        0.0,
        B,
        C,
        R,
        SRCPOS.as_slice(),
        NPTS,
        TRMGRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDAXISLENGTH)", OK, ctx)?;

    spicelib::ZZEDTERM(
        b"PENUMBRAL",
        A,
        0.0,
        C,
        R,
        SRCPOS.as_slice(),
        NPTS,
        TRMGRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDAXISLENGTH)", OK, ctx)?;

    spicelib::ZZEDTERM(
        b"PENUMBRAL",
        A,
        B,
        0.0,
        R,
        SRCPOS.as_slice(),
        NPTS,
        TRMGRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDAXISLENGTH)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"ZZEDTERM: Bad light source radius", ctx)?;

    NPTS = 4;

    spicelib::ZZEDTERM(
        b"PENUMBRAL",
        A,
        B,
        C,
        0.0,
        SRCPOS.as_slice(),
        NPTS,
        TRMGRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDRADIUS)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"ZZEDTERM: Light source and target are too close", ctx)?;

    NPTS = 4;

    spicelib::VPACK(0.0, 0.0, 3.0, SRCPOS.as_slice_mut());

    spicelib::ZZEDTERM(
        b"UMBRAL",
        2.0,
        1.0,
        1.0,
        1.5,
        SRCPOS.as_slice(),
        NPTS,
        TRMGRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(OBJECTSTOOCLOSE)", OK, ctx)?;

    //******************************************************************
    //
    //     EDTERM tests
    //
    //******************************************************************

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"EDTERM: setup: create and load kernels.", ctx)?;

    //
    // We'll need a generic test PCK and an SPK file.
    //
    testutil::TSTPCK(PCK, true, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTSPK(SPK, true, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"EDTERM: observer = moon, target = earth, source = sun. Umbral case.",
        ctx,
    )?;

    ET = 100000000.0;

    fstr::assign(&mut ABCORR, b"LT+S");
    fstr::assign(&mut OBSRVR, b"Moon");
    fstr::assign(&mut TARGET, b"399");
    fstr::assign(&mut SOURCE, b"10");
    fstr::assign(&mut FIXREF, b"IAU_EARTH");
    NPTS = 3;

    spicelib::EDTERM(
        b"UMBRAL",
        &SOURCE,
        &TARGET,
        ET,
        &FIXREF,
        &ABCORR,
        &OBSRVR,
        NPTS,
        &mut TRGEPC,
        OBSPOS.as_slice_mut(),
        TRMGRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get the observer-target vector and one-way light time
    // so we can check OBSPOS and TRGEPC.
    //
    spicelib::SPKPOS(
        &TARGET,
        ET,
        &FIXREF,
        &ABCORR,
        &OBSRVR,
        TRGPOS.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"TRGEPC", TRGEPC, b"~/", (ET - LT), TIGHT, OK, ctx)?;

    spicelib::VMINUS(TRGPOS.as_slice(), XOBSPS.as_slice_mut());

    testutil::CHCKAD(
        b"OBSPOS",
        OBSPOS.as_slice(),
        b"~~/",
        XOBSPS.as_slice(),
        3,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // In order the check the terminator points, we'll need
    // to express the target-source position in the body-fixed
    // frame at TRGEPC.
    //
    spicelib::SPKPOS(
        &SOURCE,
        TRGEPC,
        &FIXREF,
        &ABCORR,
        &TARGET,
        SRCPOS.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Fetch the target radii.
    //
    spicelib::BODVRD(&TARGET, b"RADII", 3, &mut N, RADII.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Fetch the source radii.
    //
    spicelib::BODVRD(&SOURCE, b"RADII", 3, &mut N, SRADII.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    R = SRADII[1];

    //
    // Get the expected terminator grid from ZZEDTERM. We
    // rely on ZZEDTERM to work correctly here.
    //
    spicelib::ZZEDTERM(
        b"UMBRAL",
        RADII[1],
        RADII[2],
        RADII[3],
        R,
        SRCPOS.as_slice(),
        NPTS,
        XGRID.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the terminator points.
    //
    for I in 1..=NPTS {
        testutil::CHCKAD(
            b"TRMPTS",
            TRMGRD.subarray([1, I]),
            b"~~/",
            XGRID.subarray([1, I]),
            3,
            TIGHT,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"EDTERM: observer = earth, target = earth, source = sun. Umbral case.",
        ctx,
    )?;

    ET = 100000000.0;

    fstr::assign(&mut ABCORR, b"LT+S");
    fstr::assign(&mut OBSRVR, b"Earth");
    fstr::assign(&mut TARGET, b"399");
    fstr::assign(&mut SOURCE, b"10");
    fstr::assign(&mut FIXREF, b"IAU_EARTH");
    NPTS = 3;

    spicelib::EDTERM(
        b"UMBRAL",
        &SOURCE,
        &TARGET,
        ET,
        &FIXREF,
        &ABCORR,
        &OBSRVR,
        NPTS,
        &mut TRGEPC,
        OBSPOS.as_slice_mut(),
        TRMGRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get the observer-target vector and one-way light time
    // so we can check OBSPOS and TRGEPC.
    //
    spicelib::SPKPOS(
        &TARGET,
        ET,
        &FIXREF,
        &ABCORR,
        &OBSRVR,
        TRGPOS.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"TRGEPC", TRGEPC, b"~/", (ET - LT), TIGHT, OK, ctx)?;

    spicelib::VMINUS(TRGPOS.as_slice(), XOBSPS.as_slice_mut());

    testutil::CHCKAD(
        b"OBSPOS",
        OBSPOS.as_slice(),
        b"~~/",
        XOBSPS.as_slice(),
        3,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // In order the check the terminator points, we'll need
    // to express the target-source position in the body-fixed
    // frame at TRGEPC.
    //
    spicelib::SPKPOS(
        &SOURCE,
        TRGEPC,
        &FIXREF,
        &ABCORR,
        &TARGET,
        SRCPOS.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Fetch the target radii.
    //
    spicelib::BODVRD(&TARGET, b"RADII", 3, &mut N, RADII.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Fetch the source radii.
    //
    spicelib::BODVRD(&SOURCE, b"RADII", 3, &mut N, SRADII.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    R = SRADII[1];

    //
    // Get the expected terminator grid from ZZEDTERM. We
    // rely on ZZEDTERM to work correctly here.
    //
    spicelib::ZZEDTERM(
        b"UMBRAL",
        RADII[1],
        RADII[2],
        RADII[3],
        R,
        SRCPOS.as_slice(),
        NPTS,
        XGRID.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the terminator points.
    //
    for I in 1..=NPTS {
        testutil::CHCKAD(
            b"TRMPTS",
            TRMGRD.subarray([1, I]),
            b"~~/",
            XGRID.subarray([1, I]),
            3,
            TIGHT,
            OK,
            ctx,
        )?;
    }

    //
    //
    // EDTERM error cases:
    //
    //

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"EDTERM: No frame name definition.", ctx)?;

    ET = 100000000.0;

    fstr::assign(&mut ABCORR, b"LT+S");
    fstr::assign(&mut OBSRVR, b"Moon");
    fstr::assign(&mut TARGET, b"399");
    fstr::assign(&mut SOURCE, b"10");
    fstr::assign(&mut FIXREF, b"IAU_EARTH");
    NPTS = 3;

    spicelib::EDTERM(
        b"UMBRAL",
        &SOURCE,
        &TARGET,
        ET,
        b"X",
        &ABCORR,
        &OBSRVR,
        NPTS,
        &mut TRGEPC,
        OBSPOS.as_slice_mut(),
        TRMGRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOTRANSLATION)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"EDTERM: No target name definition.", ctx)?;

    spicelib::EDTERM(
        b"UMBRAL",
        &SOURCE,
        b"X",
        ET,
        &FIXREF,
        &ABCORR,
        &OBSRVR,
        NPTS,
        &mut TRGEPC,
        OBSPOS.as_slice_mut(),
        TRMGRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOTRANSLATION)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"EDTERM: Target frame not centered at target.", ctx)?;

    spicelib::EDTERM(
        b"UMBRAL",
        &SOURCE,
        &TARGET,
        ET,
        b"J2000",
        &ABCORR,
        &OBSRVR,
        NPTS,
        &mut TRGEPC,
        OBSPOS.as_slice_mut(),
        TRMGRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDFIXREF)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"EDTERM: Bad terminator type.", ctx)?;

    spicelib::EDTERM(
        b"UMB",
        &SOURCE,
        &TARGET,
        ET,
        &FIXREF,
        &ABCORR,
        &OBSRVR,
        NPTS,
        &mut TRGEPC,
        OBSPOS.as_slice_mut(),
        TRMGRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"EDTERM: NPTS is non-positive", ctx)?;

    spicelib::EDTERM(
        b"UMBRAL",
        &SOURCE,
        &TARGET,
        ET,
        &FIXREF,
        &ABCORR,
        &OBSRVR,
        0,
        &mut TRGEPC,
        OBSPOS.as_slice_mut(),
        TRMGRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDSIZE)", OK, ctx)?;

    spicelib::EDTERM(
        b"UMBRAL",
        &SOURCE,
        &TARGET,
        ET,
        &FIXREF,
        &ABCORR,
        &OBSRVR,
        -1,
        &mut TRGEPC,
        OBSPOS.as_slice_mut(),
        TRMGRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDSIZE)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"EDTERM: target semi-axis length non-positive", ctx)?;

    //
    // Fetch the target radii.
    //
    spicelib::BODVRD(&TARGET, b"RADII", 3, &mut N, RADII.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create bad radii for the target and store these
    // in the pool.
    //
    spicelib::VEQU(RADII.as_slice(), BADRAD.as_slice_mut());

    BADRAD[1] = 0.0;

    spicelib::PDPOOL(b"BODY399_RADII", 3, BADRAD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::EDTERM(
        b"UMBRAL",
        &SOURCE,
        &TARGET,
        ET,
        &FIXREF,
        &ABCORR,
        &OBSRVR,
        0,
        &mut TRGEPC,
        OBSPOS.as_slice_mut(),
        TRMGRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    //
    // Restore good radii.
    //
    spicelib::PDPOOL(b"BODY399_RADII", 3, RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"EDTERM: source semi-axis length non-positive", ctx)?;

    //
    // Fetch the source radii.
    //
    spicelib::BODVRD(&SOURCE, b"RADII", 3, &mut N, SRADII.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create bad radii for the target and store these
    // in the pool.
    //
    spicelib::VEQU(RADII.as_slice(), BADRAD.as_slice_mut());

    BADRAD[1] = 0.0;

    spicelib::PDPOOL(b"BODY10_RADII", 3, BADRAD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::EDTERM(
        b"UMBRAL",
        &SOURCE,
        &TARGET,
        ET,
        &FIXREF,
        &ABCORR,
        &OBSRVR,
        0,
        &mut TRGEPC,
        OBSPOS.as_slice_mut(),
        TRMGRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    //
    // Restore good radii.
    //
    spicelib::PDPOOL(b"BODY10_RADII", 3, RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"EDTERM: target radius count != 3", ctx)?;

    //
    // Fetch the target radii.
    //
    spicelib::BODVRD(&TARGET, b"RADII", 3, &mut N, RADII.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set the radius count to 1.
    //
    spicelib::PDPOOL(b"BODY399_RADII", 1, RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::EDTERM(
        b"UMBRAL",
        &SOURCE,
        &TARGET,
        ET,
        &FIXREF,
        &ABCORR,
        &OBSRVR,
        0,
        &mut TRGEPC,
        OBSPOS.as_slice_mut(),
        TRMGRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDCOUNT)", OK, ctx)?;

    //
    // Set the radius count to 4.
    //
    spicelib::PDPOOL(b"BODY399_RADII", 1, RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::EDTERM(
        b"UMBRAL",
        &SOURCE,
        &TARGET,
        ET,
        &FIXREF,
        &ABCORR,
        &OBSRVR,
        0,
        &mut TRGEPC,
        OBSPOS.as_slice_mut(),
        TRMGRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDCOUNT)", OK, ctx)?;

    //
    // Restore good radii.
    //
    spicelib::PDPOOL(b"BODY399_RADII", 3, RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"EDTERM: source radius count != 3", ctx)?;

    //
    // Fetch the source radii.
    //
    spicelib::BODVRD(&SOURCE, b"RADII", 3, &mut N, RADII.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set the radius count to 1.
    //
    spicelib::PDPOOL(b"BODY10_RADII", 1, RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::EDTERM(
        b"UMBRAL",
        &SOURCE,
        &TARGET,
        ET,
        &FIXREF,
        &ABCORR,
        &OBSRVR,
        0,
        &mut TRGEPC,
        OBSPOS.as_slice_mut(),
        TRMGRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDCOUNT)", OK, ctx)?;

    //
    // Set the radius count to 4.
    //
    spicelib::PDPOOL(b"BODY10_RADII", 1, RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::EDTERM(
        b"UMBRAL",
        &SOURCE,
        &TARGET,
        ET,
        &FIXREF,
        &ABCORR,
        &OBSRVR,
        0,
        &mut TRGEPC,
        OBSPOS.as_slice_mut(),
        TRMGRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDCOUNT)", OK, ctx)?;

    //
    // Restore good radii.
    //
    spicelib::PDPOOL(b"BODY10_RADII", 3, RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"EDTERM: source is too close to target", ctx)?;

    //
    // Fetch the source radii.
    //
    spicelib::BODVRD(&SOURCE, b"RADII", 3, &mut N, RADII.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set the target radii to some awfully large values.
    //
    spicelib::VSCL(1000000.0, RADII.as_slice(), BADRAD.as_slice_mut());

    spicelib::PDPOOL(b"BODY399_RADII", 3, BADRAD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::EDTERM(
        b"UMBRAL",
        &SOURCE,
        &TARGET,
        ET,
        &FIXREF,
        &ABCORR,
        &OBSRVR,
        NPTS,
        &mut TRGEPC,
        OBSPOS.as_slice_mut(),
        TRMGRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(OBJECTSTOOCLOSE)", OK, ctx)?;

    //
    // Restore good radii.
    //
    spicelib::PDPOOL(b"BODY399_RADII", 3, RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"EDTERM: SPK lookup failure.", ctx)?;

    ET = 1000000000.0;

    spicelib::EDTERM(
        b"UMBRAL",
        &SOURCE,
        &TARGET,
        ET,
        &FIXREF,
        &ABCORR,
        &OBSRVR,
        NPTS,
        &mut TRGEPC,
        OBSPOS.as_slice_mut(),
        TRMGRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"EDTERM: clean up: unload and delete kernels.", ctx)?;

    spicelib::SPKUEF(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
