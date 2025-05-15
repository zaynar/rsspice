//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const VTIGHT: f64 = 0.000000000000001;
const TIGHT: f64 = 0.0000000000001;
const LNSIZE: i32 = 80;

//$Procedure      F_ZZELLBDS ( Test bounding ellipsoid routine )
pub fn F_ZZELLBDS(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut CASNAM = [b' '; LNSIZE as usize];
    let mut A: f64 = 0.0;
    let mut ALT: f64 = 0.0;
    let mut AMAX: f64 = 0.0;
    let mut AMIN: f64 = 0.0;
    let mut B: f64 = 0.0;
    let mut BMAX: f64 = 0.0;
    let mut BMIN: f64 = 0.0;
    let mut DELTA: f64 = 0.0;
    let mut HMAX: f64 = 0.0;
    let mut HMIN: f64 = 0.0;
    let mut LAT: f64 = 0.0;
    let mut LON: f64 = 0.0;
    let mut XAMAX: f64 = 0.0;
    let mut XAMIN: f64 = 0.0;
    let mut XBMAX: f64 = 0.0;
    let mut XBMIN: f64 = 0.0;
    let mut XLAT: f64 = 0.0;
    let mut XLON: f64 = 0.0;
    let mut P = StackArray::<f64, 3>::new(1..=3);
    let mut N: i32 = 0;

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

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZELLBDS", ctx)?;

    //
    // ZZELLBDS error cases:
    //

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Error case: B is non-positive.", ctx)?;

    A = 5.0;
    B = 0.0;
    HMAX = 1.0;
    HMIN = -1.0;

    spicelib::ZZELLBDS(
        A, B, HMAX, HMIN, &mut AMAX, &mut BMAX, &mut AMIN, &mut BMIN, ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NONPOSITIVERADIUS)", OK, ctx)?;

    B = -1.0;

    spicelib::ZZELLBDS(
        A, B, HMAX, HMIN, &mut AMAX, &mut BMAX, &mut AMIN, &mut BMIN, ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NONPOSITIVERADIUS)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Error case: B > A.", ctx)?;

    A = 5.0;
    B = 5.1;
    HMAX = 1.0;
    HMIN = -1.0;

    spicelib::ZZELLBDS(
        A, B, HMAX, HMIN, &mut AMAX, &mut BMAX, &mut AMIN, &mut BMIN, ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(RADIIOUTOFORDER)", OK, ctx)?;

    B = -1.0;

    spicelib::ZZELLBDS(
        A, B, HMAX, HMIN, &mut AMAX, &mut BMAX, &mut AMIN, &mut BMIN, ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NONPOSITIVERADIUS)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Error case: HMIN > HMAX.", ctx)?;

    A = 5.0;
    B = 4.0;
    HMAX = 1.0;
    HMIN = 1.1;

    spicelib::ZZELLBDS(
        A, B, HMAX, HMIN, &mut AMAX, &mut BMAX, &mut AMIN, &mut BMIN, ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BOUNDSOUTOFORDER)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Error case: HMIN <= -B.", ctx)?;

    A = 5.0;
    B = 4.0;
    HMAX = 1.0;
    HMIN = -B;

    spicelib::ZZELLBDS(
        A, B, HMAX, HMIN, &mut AMAX, &mut BMAX, &mut AMIN, &mut BMIN, ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(LOWERBOUNDTOOLOW)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Error case: B + (A/B)*HMIN <= 0", ctx)?;

    A = 5.0;
    B = 4.0;
    HMAX = 1.0;
    HMIN = -(0.9 * B);

    spicelib::ZZELLBDS(
        A, B, HMAX, HMIN, &mut AMAX, &mut BMAX, &mut AMIN, &mut BMIN, ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(LOWERBOUNDTOOLOW)", OK, ctx)?;

    //**********************************************************************
    //
    //     Normal cases
    //
    //**********************************************************************

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Normal case: HMAX > 0, HMIN < 0", ctx)?;

    A = 25.0;
    B = 20.0;
    HMAX = 1.0;
    HMIN = -1.0;

    XAMAX = (A + HMAX);
    XBMAX = (B + (HMAX * (A / B)));

    XAMIN = (A + HMIN);
    XBMIN = (B + (HMIN * (A / B)));

    spicelib::ZZELLBDS(
        A, B, HMAX, HMIN, &mut AMAX, &mut BMAX, &mut AMIN, &mut BMIN, ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"AMAX", AMAX, b"~", XAMAX, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"BMAX", BMAX, b"~", XBMAX, VTIGHT, OK, ctx)?;

    testutil::CHCKSD(b"AMIN", AMIN, b"~", XAMIN, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"BMIN", BMIN, b"~", XBMIN, VTIGHT, OK, ctx)?;

    //
    // Sample heights from the bounding ellipsoids. Verify that
    // the heights satisfy the expected conditions.
    //
    N = 10001;
    DELTA = (((2 as f64) * spicelib::PI(ctx)) / (N - 1) as f64);

    for I in 1..=N {
        XLAT = (((I - 1) as f64) * DELTA);

        XLON = (spicelib::PI(ctx) / 3 as f64);
        //
        // Generate the cartesian coordinates of a point P on
        // the outer bounding ellipsoid at geodetic latitude LAT.
        //
        spicelib::GEOREC(
            XLON,
            XLAT,
            0.0,
            AMAX,
            ((AMAX - BMAX) / AMAX),
            P.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        //
        // Find the altitude of P relative to the spheroid with
        // radii A, B. The altitude should be no smaller than HMAX.
        //
        spicelib::RECGEO(
            P.as_slice(),
            A,
            ((A - B) / A),
            &mut LON,
            &mut LAT,
            &mut ALT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (ALT < HMAX) {
            fstr::assign(&mut CASNAM, b"ALT for I = #");

            spicelib::REPMI(&CASNAM.clone(), b"#", I, &mut CASNAM, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSD(&CASNAM, ALT, b">=", (HMAX - TIGHT), 0.0, OK, ctx)?;
        }
    }

    for I in 1..=N {
        XLAT = (((I - 1) as f64) * DELTA);

        XLON = (spicelib::PI(ctx) / 3 as f64);
        //
        // Generate the cartesian coordinates of a point P on
        // the inner bounding ellipsoid at geodetic latitude LAT.
        //
        spicelib::GEOREC(
            XLON,
            XLAT,
            0.0,
            AMIN,
            ((AMIN - BMIN) / AMIN),
            P.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        //
        // Find the altitude of P relative to the spheroid with
        // radii A, B. The altitude should be no greater than HMIN.
        //
        spicelib::RECGEO(
            P.as_slice(),
            A,
            ((A - B) / A),
            &mut LON,
            &mut LAT,
            &mut ALT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (ALT > HMIN) {
            fstr::assign(&mut CASNAM, b"ALT for I = #");

            spicelib::REPMI(&CASNAM.clone(), b"#", I, &mut CASNAM, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSD(&CASNAM, ALT, b"<=", (HMIN + TIGHT), 0.0, OK, ctx)?;
        }
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Normal case: HMAX = 0, HMIN = 0", ctx)?;

    A = 25.0;
    B = 20.0;
    HMAX = 0.0;
    HMIN = 0.0;

    XAMAX = (A + HMAX);
    XBMAX = (B + (HMAX * (A / B)));

    XAMIN = (A + HMIN);
    XBMIN = (B + (HMIN * (A / B)));

    spicelib::ZZELLBDS(
        A, B, HMAX, HMIN, &mut AMAX, &mut BMAX, &mut AMIN, &mut BMIN, ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"AMAX", AMAX, b"~", XAMAX, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"BMAX", BMAX, b"~", XBMAX, VTIGHT, OK, ctx)?;

    testutil::CHCKSD(b"AMIN", AMIN, b"~", XAMIN, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"BMIN", BMIN, b"~", XBMIN, VTIGHT, OK, ctx)?;

    //
    // Sample heights from the bounding ellipsoids. Verify that
    // the heights satisfy the expected conditions.
    //
    N = 10001;
    DELTA = (((2 as f64) * spicelib::PI(ctx)) / (N - 1) as f64);

    for I in 1..=N {
        XLAT = (((I - 1) as f64) * DELTA);

        XLON = (spicelib::PI(ctx) / 3 as f64);
        //
        // Generate the cartesian coordinates of a point P on
        // the outer bounding ellipsoid at geodetic latitude LAT.
        //
        spicelib::GEOREC(
            XLON,
            XLAT,
            0.0,
            AMAX,
            ((AMAX - BMAX) / AMAX),
            P.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        //
        // Find the altitude of P relative to the spheroid with
        // radii A, B. The altitude should be no smaller than HMAX.
        //
        spicelib::RECGEO(
            P.as_slice(),
            A,
            ((A - B) / A),
            &mut LON,
            &mut LAT,
            &mut ALT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (ALT < HMAX) {
            fstr::assign(&mut CASNAM, b"ALT for I = #");

            spicelib::REPMI(&CASNAM.clone(), b"#", I, &mut CASNAM, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSD(&CASNAM, ALT, b">=", (HMAX - TIGHT), 0.0, OK, ctx)?;
        }
    }

    for I in 1..=N {
        XLAT = (((I - 1) as f64) * DELTA);

        XLON = (spicelib::PI(ctx) / 3 as f64);
        //
        // Generate the cartesian coordinates of a point P on
        // the inner bounding ellipsoid at geodetic latitude LAT.
        //
        spicelib::GEOREC(
            XLON,
            XLAT,
            0.0,
            AMIN,
            ((AMIN - BMIN) / AMIN),
            P.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        //
        // Find the altitude of P relative to the spheroid with
        // radii A, B. The altitude should be no greater than HMIN.
        //
        spicelib::RECGEO(
            P.as_slice(),
            A,
            ((A - B) / A),
            &mut LON,
            &mut LAT,
            &mut ALT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (ALT > HMIN) {
            fstr::assign(&mut CASNAM, b"ALT for I = #");

            spicelib::REPMI(&CASNAM.clone(), b"#", I, &mut CASNAM, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSD(&CASNAM, ALT, b"<=", (HMIN + TIGHT), 0.0, OK, ctx)?;
        }
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Normal case: HMAX > 0, HMIN > 0", ctx)?;

    A = 25.0;
    B = 20.0;
    HMAX = 5.0;
    HMIN = 1.0;

    XAMAX = (A + HMAX);
    XBMAX = (B + (HMAX * (A / B)));

    XAMIN = (A + (HMIN * (B / A)));
    XBMIN = (B + HMIN);

    spicelib::ZZELLBDS(
        A, B, HMAX, HMIN, &mut AMAX, &mut BMAX, &mut AMIN, &mut BMIN, ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"AMAX", AMAX, b"~", XAMAX, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"BMAX", BMAX, b"~", XBMAX, VTIGHT, OK, ctx)?;

    testutil::CHCKSD(b"AMIN", AMIN, b"~", XAMIN, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"BMIN", BMIN, b"~", XBMIN, VTIGHT, OK, ctx)?;

    //
    // Sample heights from the bounding ellipsoids. Verify that
    // the heights satisfy the expected conditions.
    //
    N = 10001;
    DELTA = (((2 as f64) * spicelib::PI(ctx)) / (N - 1) as f64);

    for I in 1..=N {
        XLAT = (((I - 1) as f64) * DELTA);

        XLON = (spicelib::PI(ctx) / 3 as f64);
        //
        // Generate the cartesian coordinates of a point P on
        // the outer bounding ellipsoid at geodetic latitude LAT.
        //
        spicelib::GEOREC(
            XLON,
            XLAT,
            0.0,
            AMAX,
            ((AMAX - BMAX) / AMAX),
            P.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        //
        // Find the altitude of P relative to the spheroid with
        // radii A, B. The altitude should be no smaller than HMAX.
        //
        spicelib::RECGEO(
            P.as_slice(),
            A,
            ((A - B) / A),
            &mut LON,
            &mut LAT,
            &mut ALT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (ALT < HMAX) {
            fstr::assign(&mut CASNAM, b"ALT for I = #");

            spicelib::REPMI(&CASNAM.clone(), b"#", I, &mut CASNAM, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSD(&CASNAM, ALT, b">=", (HMAX - TIGHT), 0.0, OK, ctx)?;
        }
    }

    for I in 1..=N {
        XLAT = (((I - 1) as f64) * DELTA);

        XLON = (spicelib::PI(ctx) / 3 as f64);
        //
        // Generate the cartesian coordinates of a point P on
        // the inner bounding ellipsoid at geodetic latitude LAT.
        //
        spicelib::GEOREC(
            XLON,
            XLAT,
            0.0,
            AMIN,
            ((AMIN - BMIN) / AMIN),
            P.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        //
        // Find the altitude of P relative to the spheroid with
        // radii A, B. The altitude should be no greater than HMIN.
        //
        spicelib::RECGEO(
            P.as_slice(),
            A,
            ((A - B) / A),
            &mut LON,
            &mut LAT,
            &mut ALT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (ALT > HMIN) {
            fstr::assign(&mut CASNAM, b"ALT for I = #");

            spicelib::REPMI(&CASNAM.clone(), b"#", I, &mut CASNAM, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSD(&CASNAM, ALT, b"<=", (HMIN + TIGHT), 0.0, OK, ctx)?;
        }
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Normal case: HMAX < 0, HMIN < 0", ctx)?;

    A = 25.0;
    B = 20.0;
    HMAX = -1.0;
    HMIN = -2.0;

    XAMAX = (A + (HMAX * (B / A)));
    XBMAX = (B + HMAX);

    XAMIN = (A + HMIN);
    XBMIN = (B + (HMIN * (A / B)));

    spicelib::ZZELLBDS(
        A, B, HMAX, HMIN, &mut AMAX, &mut BMAX, &mut AMIN, &mut BMIN, ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"AMAX", AMAX, b"~", XAMAX, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"BMAX", BMAX, b"~", XBMAX, VTIGHT, OK, ctx)?;

    testutil::CHCKSD(b"AMIN", AMIN, b"~", XAMIN, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"BMIN", BMIN, b"~", XBMIN, VTIGHT, OK, ctx)?;

    //
    // Sample heights from the bounding ellipsoids. Verify that
    // the heights satisfy the expected conditions.
    //
    N = 10001;
    DELTA = (((2 as f64) * spicelib::PI(ctx)) / (N - 1) as f64);

    for I in 1..=N {
        XLAT = (((I - 1) as f64) * DELTA);

        XLON = (spicelib::PI(ctx) / 3 as f64);
        //
        // Generate the cartesian coordinates of a point P on
        // the outer bounding ellipsoid at geodetic latitude LAT.
        //
        spicelib::GEOREC(
            XLON,
            XLAT,
            0.0,
            AMAX,
            ((AMAX - BMAX) / AMAX),
            P.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        //
        // Find the altitude of P relative to the spheroid with
        // radii A, B. The altitude should be no smaller than HMAX.
        //
        spicelib::RECGEO(
            P.as_slice(),
            A,
            ((A - B) / A),
            &mut LON,
            &mut LAT,
            &mut ALT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (ALT < HMAX) {
            fstr::assign(&mut CASNAM, b"ALT for I = #");

            spicelib::REPMI(&CASNAM.clone(), b"#", I, &mut CASNAM, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSD(&CASNAM, ALT, b">=", (HMAX - TIGHT), 0.0, OK, ctx)?;
        }
    }

    for I in 1..=N {
        XLAT = (((I - 1) as f64) * DELTA);

        XLON = (spicelib::PI(ctx) / 3 as f64);
        //
        // Generate the cartesian coordinates of a point P on
        // the inner bounding ellipsoid at geodetic latitude LAT.
        //
        spicelib::GEOREC(
            XLON,
            XLAT,
            0.0,
            AMIN,
            ((AMIN - BMIN) / AMIN),
            P.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        //
        // Find the altitude of P relative to the spheroid with
        // radii A, B. The altitude should be no greater than HMIN.
        //
        spicelib::RECGEO(
            P.as_slice(),
            A,
            ((A - B) / A),
            &mut LON,
            &mut LAT,
            &mut ALT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (ALT > HMIN) {
            fstr::assign(&mut CASNAM, b"ALT for I = #");

            spicelib::REPMI(&CASNAM.clone(), b"#", I, &mut CASNAM, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSD(&CASNAM, ALT, b"<=", (HMIN + TIGHT), 0.0, OK, ctx)?;
        }
    }

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
