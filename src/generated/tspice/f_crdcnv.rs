//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NGRID: i32 = 5;
const NB: i32 = 6;
const LSE: f64 = 0.000000001;
const MED: f64 = 0.00000000001;
const TGH: f64 = 0.000000000000001;
const AE: f64 = 6378.137;
const F: f64 = (1.0 / 298.257);
const BE: f64 = (AE * (1.0 - F));
const PCK: &[u8] = b"test_0008.tpc";

struct SaveVars {
    GRID: StackArray<f64, 5>,
    BODY: StackArray<i32, 6>,
    EX: f64,
    EY: f64,
    EZ: f64,
    ER: f64,
    ERHO: f64,
    ELAT: f64,
    ECLAT: f64,
    ELON: f64,
    ERA: f64,
    REC: StackArray<f64, 3>,
    RHO: f64,
    LON: f64,
    Z: f64,
    R: f64,
    CLAT: f64,
    LONG: f64,
    RADIUS: f64,
    LLON: f64,
    LLAT: f64,
    RANGE: f64,
    RA: f64,
    DEC: f64,
    GLON: f64,
    GLAT: f64,
    GALT: f64,
    SLAT: f64,
    SLON: f64,
    GVEC: StackArray<f64, 3>,
    SRFEQN: f64,
    NADIR: StackArray<f64, 3>,
    RADII: StackArray<f64, 3>,
    NDIM: i32,
    CASE: Vec<u8>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut GRID = StackArray::<f64, 5>::new(1..=NGRID);
        let mut BODY = StackArray::<i32, 6>::new(1..=NB);
        let mut EX: f64 = 0.0;
        let mut EY: f64 = 0.0;
        let mut EZ: f64 = 0.0;
        let mut ER: f64 = 0.0;
        let mut ERHO: f64 = 0.0;
        let mut ELAT: f64 = 0.0;
        let mut ECLAT: f64 = 0.0;
        let mut ELON: f64 = 0.0;
        let mut ERA: f64 = 0.0;
        let mut REC = StackArray::<f64, 3>::new(1..=3);
        let mut RHO: f64 = 0.0;
        let mut LON: f64 = 0.0;
        let mut Z: f64 = 0.0;
        let mut R: f64 = 0.0;
        let mut CLAT: f64 = 0.0;
        let mut LONG: f64 = 0.0;
        let mut RADIUS: f64 = 0.0;
        let mut LLON: f64 = 0.0;
        let mut LLAT: f64 = 0.0;
        let mut RANGE: f64 = 0.0;
        let mut RA: f64 = 0.0;
        let mut DEC: f64 = 0.0;
        let mut GLON: f64 = 0.0;
        let mut GLAT: f64 = 0.0;
        let mut GALT: f64 = 0.0;
        let mut SLAT: f64 = 0.0;
        let mut SLON: f64 = 0.0;
        let mut GVEC = StackArray::<f64, 3>::new(1..=3);
        let mut SRFEQN: f64 = 0.0;
        let mut NADIR = StackArray::<f64, 3>::new(1..=3);
        let mut RADII = StackArray::<f64, 3>::new(1..=3);
        let mut NDIM: i32 = 0;
        let mut CASE = vec![b' '; 80 as usize];

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.01),
                Val::D(1.0),
                Val::D(10000.0),
            ]
            .into_iter();
            GRID.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(299),
                Val::I(399),
                Val::I(499),
                Val::I(599),
                Val::I(699),
                Val::I(799),
            ]
            .into_iter();
            BODY.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            GRID,
            BODY,
            EX,
            EY,
            EZ,
            ER,
            ERHO,
            ELAT,
            ECLAT,
            ELON,
            ERA,
            REC,
            RHO,
            LON,
            Z,
            R,
            CLAT,
            LONG,
            RADIUS,
            LLON,
            LLAT,
            RANGE,
            RA,
            DEC,
            GLON,
            GLAT,
            GALT,
            SLAT,
            SLON,
            GVEC,
            SRFEQN,
            NADIR,
            RADII,
            NDIM,
            CASE,
        }
    }
}

//$Procedure F_CRDCNV (Family of tests on coordinate conversions)
pub fn F_CRDCNV(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB Functions
    //

    //
    // grid dimension of a cube ( NGRID x NGRID x NGRID )
    //

    //
    // number of NAIF bodies used for test
    //

    //
    // numerical tolerances
    //

    //
    // equatorial radius of a reference ellipsoid
    //

    //
    // flattening of the reference ellipsoid
    //

    //
    // polar radius of the reference ellipsoid
    //

    //
    // Test PCK.
    //

    //
    // Local variables
    //

    //
    // grid locations
    //

    //
    // NAIF body ID numbers
    //

    //
    // grid index on x-axis
    //

    //
    // grid index on y-axis
    //

    //
    // grid index on z-axis
    //

    //
    // Listed below are coordinate values computed in this routine at
    // each grid point. All angles are in radians.
    //

    //
    // x-component of the right-handed Cartesian frame
    //

    //
    // y-component of the right-handed Cartesian frame
    //

    //
    // z-component of the right-handed Cartesian frame
    //

    //
    // distance from origin
    //

    //
    // distance from z-axis
    //

    //
    // latitude in [ -HALFPI(), +HALFPI() ]
    //

    //
    // co-latitude in [ 0.0D0, +PI() ]
    //

    //
    // east longitude in ( -PI(), +PI() ]
    //

    //
    // right ascension in [ 0.0D0, TWOPI() )
    //

    //
    // Listed below are coordinate values returned from SPICELIB
    // routines.
    //

    //
    // rectangular coordinates
    //

    //
    // cylindrical coordinates
    //

    //
    // spherical coordinates
    //

    //
    // latitudinal coordinates
    //

    //
    // RAD ( range, right ascension, declination ) coordinates
    //

    //
    // geodetic coordinates
    //

    //
    // surface coordinates
    //

    //
    // naif body ID number
    //

    //
    // geodetic altitude vector
    //

    //
    // evaluation of spheroidal surface equation
    //

    //
    // origin to nadir vector
    //

    //
    // radii of tri-axial ellipsoid
    //

    //
    // number of radii
    //

    //
    // test case description
    //

    //
    // Save everything.
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_CRDCNV", ctx)?;

    //
    // Create the PCK file, load it, and delete it. We need it to get
    // values of radii of naif bodies.
    //
    testutil::T_PCK08(PCK, true, false, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Test loop over grid points of the NGRID x NGRID x NGRID cube.
    //
    for IX in 1..=NGRID {
        for IY in 1..=NGRID {
            for IZ in 1..=NGRID {
                //
                // rectangular coordinates of a grid point
                //
                save.EX = save.GRID[IX];
                save.EY = save.GRID[IY];
                save.EZ = save.GRID[IZ];

                //
                // Exclude the origin from test.
                //
                if (((save.EX == 0.0) && (save.EX == 0.0)) && (save.EX == 0.0)) {
                    save.EX = 1.0;
                    save.EY = 1.0;
                    save.EZ = 1.0;
                }

                //
                // radius from origin
                //
                save.ER = f64::sqrt(
                    ((f64::powi(save.EX, 2) + f64::powi(save.EY, 2)) + f64::powi(save.EZ, 2)),
                );

                //
                // radius from z-axis
                //
                save.ERHO = f64::sqrt((f64::powi(save.EX, 2) + f64::powi(save.EY, 2)));

                //
                // right ascension in [ 0.0D0, TWOPI() )
                //
                save.ERA = intrinsics::DMOD(f64::atan2(save.EY, save.EX), spicelib::TWOPI(ctx));

                if (save.ERA < 0.0) {
                    save.ERA = (save.ERA + spicelib::TWOPI(ctx));
                }

                //
                // east longitude in ( -PI(), PI() ]
                //
                save.ELON = f64::atan2(save.EY, save.EX);

                if (save.ELON == -spicelib::PI(ctx)) {
                    save.ELON = spicelib::PI(ctx);
                }

                //
                // co-latitude in [ 0.0D0, PI() ]
                //
                save.ECLAT = f64::atan2(save.ERHO, save.EZ);

                //
                // latitude in [ -HALFPI(), HALFPI() ]
                //
                save.ELAT = f64::atan2(save.EZ, save.ERHO);

                //
                // Identity the grid point on the test description.
                //
                fstr::assign(&mut save.CASE, b"# # #");
                spicelib::REPMD(&save.CASE.to_vec(), b"#", save.EX, 14, &mut save.CASE, ctx);
                spicelib::REPMD(&save.CASE.to_vec(), b"#", save.EY, 14, &mut save.CASE, ctx);
                spicelib::REPMD(&save.CASE.to_vec(), b"#", save.EZ, 14, &mut save.CASE, ctx);

                //
                // Test rectangular to cylindrical coordinate conversion.
                //

                testutil::TCASE(&fstr::concat(b"RECCYL: ", &save.CASE), ctx)?;

                //
                // Pack the grid point into a rectangular coordinate
                // vector.
                //
                spicelib::VPACK(save.EX, save.EY, save.EZ, save.REC.as_slice_mut());

                //
                // Convert to cylindrical coordinates.
                //
                spicelib::RECCYL(
                    save.REC.as_slice(),
                    &mut save.RHO,
                    &mut save.LON,
                    &mut save.Z,
                    ctx,
                );

                //
                // The cylindrical longitude, LON, returned from subroutine
                // RECCYL is in the range of [ 0.0D0, TWOPI() ). This
                // conflicts with the range of longitude, ( -PI(), +PI() ],
                // defined in some other subroutines. That is,
                //
                save.LON = f64::atan2(save.REC[2], save.REC[1]);

                //
                // The above line, inserted to suppress error messages,
                // should be removed after fixing the inconsistency in the
                // definition of longitude.
                //

                //
                // Check the converted coordinates.
                //
                testutil::CHCKSD(b"RHO", save.RHO, b"~", save.ERHO, MED, OK, ctx)?;
                testutil::CHCKSD(b"LON", save.LON, b"~", save.ELON, MED, OK, ctx)?;
                testutil::CHCKSD(b"Z", save.Z, b"~", save.EZ, MED, OK, ctx)?;

                //
                // Test cylindrical to rectangular coordinate conversion.
                //
                testutil::TCASE(&fstr::concat(b"CYLREC: ", &save.CASE), ctx)?;

                //
                // Revert cylindrical coordinates to rectangular
                // coordinates.
                //
                spicelib::CYLREC(save.RHO, save.LON, save.Z, save.REC.as_slice_mut());

                //
                // Check the reverted coordinates.
                //
                testutil::CHCKSD(b"REC(1)", save.REC[1], b"~", save.EX, MED, OK, ctx)?;
                testutil::CHCKSD(b"REC(2)", save.REC[2], b"~", save.EY, MED, OK, ctx)?;
                testutil::CHCKSD(b"REC(3)", save.REC[3], b"~", save.EZ, MED, OK, ctx)?;

                //
                // Test rectangular to spherical coordinate conversion.
                //
                testutil::TCASE(&fstr::concat(b"RECSPH: ", &save.CASE), ctx)?;

                //
                // Pack the grid point into a rectangular coordinate
                // vector.
                //
                spicelib::VPACK(save.EX, save.EY, save.EZ, save.REC.as_slice_mut());

                //
                // Convert to spherical coordinates.
                //
                spicelib::RECSPH(
                    save.REC.as_slice(),
                    &mut save.R,
                    &mut save.CLAT,
                    &mut save.LONG,
                );

                //
                // Check the converted coordinates.
                //
                testutil::CHCKSD(b"R", save.R, b"~", save.ER, MED, OK, ctx)?;
                testutil::CHCKSD(b"CLAT", save.CLAT, b"~", save.ECLAT, MED, OK, ctx)?;
                testutil::CHCKSD(b"LONG", save.LONG, b"~", save.ELON, MED, OK, ctx)?;

                //
                // Test spherical to rectangular coordinate conversion.
                //
                testutil::TCASE(&fstr::concat(b"SPHREC: ", &save.CASE), ctx)?;

                //
                // Revert spherical coordinates to rectangular coordinates.
                //
                spicelib::SPHREC(save.R, save.CLAT, save.LONG, save.REC.as_slice_mut());

                //
                // Check the reverted coordinates.
                //
                testutil::CHCKSD(b"REC(1)", save.REC[1], b"~", save.EX, MED, OK, ctx)?;
                testutil::CHCKSD(b"REC(2)", save.REC[2], b"~", save.EY, MED, OK, ctx)?;
                testutil::CHCKSD(b"REC(3)", save.REC[3], b"~", save.EZ, MED, OK, ctx)?;

                //
                // Test rectangular to latitudinal coordinate conversion.
                //
                testutil::TCASE(&fstr::concat(b"RECLAT: ", &save.CASE), ctx)?;

                //
                // Pack the grid point into a rectangular coordinate
                // vector.
                //
                spicelib::VPACK(save.EX, save.EY, save.EZ, save.REC.as_slice_mut());

                //
                // Convert to latitudinal coordinates.
                //
                spicelib::RECLAT(
                    save.REC.as_slice(),
                    &mut save.RADIUS,
                    &mut save.LLON,
                    &mut save.LLAT,
                );

                //
                // Check the converted coordinates.
                //
                testutil::CHCKSD(b"RADIUS", save.RADIUS, b"~", save.ER, MED, OK, ctx)?;
                testutil::CHCKSD(b"LLON", save.LLON, b"~", save.ELON, MED, OK, ctx)?;
                testutil::CHCKSD(b"LLAT", save.LLAT, b"~", save.ELAT, MED, OK, ctx)?;

                //
                // Test latitudinal to rectangular coordinate conversion.
                //
                testutil::TCASE(&fstr::concat(b"LATREC: ", &save.CASE), ctx)?;

                //
                // Revert latitudinal coordinates to rectangular
                // coordinates.
                //
                spicelib::LATREC(save.RADIUS, save.LLON, save.LLAT, save.REC.as_slice_mut());

                //
                // Check the reverted coordinates.
                //
                testutil::CHCKSD(b"REC(1)", save.REC[1], b"~", save.EX, MED, OK, ctx)?;
                testutil::CHCKSD(b"REC(2)", save.REC[2], b"~", save.EY, MED, OK, ctx)?;
                testutil::CHCKSD(b"REC(3)", save.REC[3], b"~", save.EZ, MED, OK, ctx)?;

                //
                // Test cylindrical to spherical coordinate conversion.
                //
                testutil::TCASE(&fstr::concat(b"CYLSPH: ", &save.CASE), ctx)?;

                //
                // Convert cylindrical coordinates to latitudinal
                // coordinates.
                //
                spicelib::CYLSPH(
                    save.RHO,
                    save.LON,
                    save.Z,
                    &mut save.R,
                    &mut save.CLAT,
                    &mut save.LONG,
                );

                //
                // Check the converted coordinates.
                //
                testutil::CHCKSD(b"R", save.R, b"~", save.ER, MED, OK, ctx)?;
                testutil::CHCKSD(b"CLAT", save.CLAT, b"~", save.ECLAT, MED, OK, ctx)?;
                testutil::CHCKSD(b"LONG", save.LONG, b"~", save.ELON, MED, OK, ctx)?;

                //
                // Test spherical to cylindrical coordinate conversion.
                //

                testutil::TCASE(&fstr::concat(b"SPHCYL: ", &save.CASE), ctx)?;

                //
                // Revert latitudinal coordinates to cylindrical
                // coordinates.
                //
                spicelib::SPHCYL(
                    save.R,
                    save.CLAT,
                    save.LONG,
                    &mut save.RHO,
                    &mut save.LON,
                    &mut save.Z,
                );

                //
                // Check the reverted coordinates.
                //
                testutil::CHCKSD(b"RHO", save.RHO, b"~", save.ERHO, MED, OK, ctx)?;
                testutil::CHCKSD(b"LON", save.LON, b"~", save.ELON, MED, OK, ctx)?;
                testutil::CHCKSD(b"Z", save.Z, b"~", save.EZ, MED, OK, ctx)?;

                //
                // Test latitudinal to spherical coordinate conversion.
                //

                testutil::TCASE(&fstr::concat(b"LATSPH: ", &save.CASE), ctx)?;

                //
                // Convert latitudinal coordinates to spherical
                // coordinates.
                //
                spicelib::LATSPH(
                    save.RADIUS,
                    save.LLON,
                    save.LLAT,
                    &mut save.R,
                    &mut save.CLAT,
                    &mut save.LONG,
                    ctx,
                );

                //
                // Check the converted coordinates.
                //
                testutil::CHCKSD(b"R", save.R, b"~", save.ER, MED, OK, ctx)?;
                testutil::CHCKSD(b"CLAT", save.CLAT, b"~", save.ECLAT, MED, OK, ctx)?;
                testutil::CHCKSD(b"LONG", save.LONG, b"~", save.ELON, MED, OK, ctx)?;

                //
                // Test spherical to latitudinal coordinate conversion.
                //
                testutil::TCASE(&fstr::concat(b"SPHLAT: ", &save.CASE), ctx)?;

                //
                // Revert spherical coordinates to latitudinal coordinates.
                //
                spicelib::SPHLAT(
                    save.R,
                    save.CLAT,
                    save.LONG,
                    &mut save.RADIUS,
                    &mut save.LLON,
                    &mut save.LLAT,
                    ctx,
                );

                //
                // Check the reverted coordinates.
                //
                testutil::CHCKSD(b"RADIUS", save.RADIUS, b"~", save.ER, MED, OK, ctx)?;
                testutil::CHCKSD(b"LLON", save.LLON, b"~", save.ELON, MED, OK, ctx)?;
                testutil::CHCKSD(b"LLAT", save.LLAT, b"~", save.ELAT, MED, OK, ctx)?;

                //
                // Test latitudinal to cylindrical coordinate conversion.
                //
                testutil::TCASE(&fstr::concat(b"LATCYL: ", &save.CASE), ctx)?;

                //
                // Convert latitudinal coordinates to cylindrical
                // coordinates.
                //
                spicelib::LATCYL(
                    save.RADIUS,
                    save.LLON,
                    save.LLAT,
                    &mut save.RHO,
                    &mut save.LON,
                    &mut save.Z,
                );

                //
                // Check the converted coordinates.
                //
                testutil::CHCKSD(b"RHO", save.RHO, b"~", save.ERHO, MED, OK, ctx)?;
                testutil::CHCKSD(b"LON", save.LON, b"~", save.ELON, MED, OK, ctx)?;
                testutil::CHCKSD(b"Z", save.Z, b"~", save.EZ, MED, OK, ctx)?;

                //
                // Test cylindrical to latitudinal coordinate conversion.
                //
                testutil::TCASE(&fstr::concat(b"CYLLAT: ", &save.CASE), ctx)?;

                //
                // Revert cylindrical coordinates to latitudinal
                // coordinates.
                //
                spicelib::CYLLAT(
                    save.RHO,
                    save.LON,
                    save.Z,
                    &mut save.RADIUS,
                    &mut save.LLON,
                    &mut save.LLAT,
                );

                //
                // Check the reverted coordinates.
                //
                testutil::CHCKSD(b"RADIUS", save.RADIUS, b"~", save.ER, MED, OK, ctx)?;
                testutil::CHCKSD(b"LLON", save.LLON, b"~", save.ELON, MED, OK, ctx)?;
                testutil::CHCKSD(b"LLAT", save.LLAT, b"~", save.ELAT, MED, OK, ctx)?;

                //
                // Test rectangular to RAD coordinate conversion.
                //
                testutil::TCASE(&fstr::concat(b"RECRAD: ", &save.CASE), ctx)?;

                //
                // Pack the grid point into a rectangular coordinate
                // vector.
                //
                spicelib::VPACK(save.EX, save.EY, save.EZ, save.REC.as_slice_mut());

                //
                // Convert rectangular coordinates to RAD coordinates.
                //
                spicelib::RECRAD(
                    save.REC.as_slice(),
                    &mut save.RANGE,
                    &mut save.RA,
                    &mut save.DEC,
                    ctx,
                );

                //
                // Check the converted coordinates.
                //
                testutil::CHCKSD(b"RANGE", save.RANGE, b"~", save.ER, MED, OK, ctx)?;
                testutil::CHCKSD(b"RA", save.RA, b"~", save.ERA, MED, OK, ctx)?;
                testutil::CHCKSD(b"DEC", save.DEC, b"~", save.ELAT, MED, OK, ctx)?;

                //
                // Test RAD to rectangular coordinate conversion.
                //
                testutil::TCASE(&fstr::concat(b"RADREC: ", &save.CASE), ctx)?;

                //
                // Revert RAD coordinates to rectangular coordinates.
                //
                spicelib::RADREC(save.RANGE, save.RA, save.DEC, save.REC.as_slice_mut());

                //
                // Check the converted coordinates.
                //
                testutil::CHCKSD(b"REC(1)", save.REC[1], b"~", save.EX, MED, OK, ctx)?;
                testutil::CHCKSD(b"REC(2)", save.REC[2], b"~", save.EY, MED, OK, ctx)?;
                testutil::CHCKSD(b"REC(3)", save.REC[3], b"~", save.EZ, MED, OK, ctx)?;

                //
                // Test rectangular to geodetic coordinate conversion.
                //
                testutil::TCASE(&fstr::concat(b"RECGEO: ", &save.CASE), ctx)?;

                //
                // Pack the grid point into a rectangular coordinate
                // vector.
                //
                spicelib::VPACK(save.EX, save.EY, save.EZ, save.REC.as_slice_mut());

                //
                // Convert rectangular coordinates to geodetic coordinates.
                //
                spicelib::RECGEO(
                    save.REC.as_slice(),
                    AE,
                    F,
                    &mut save.GLON,
                    &mut save.GLAT,
                    &mut save.GALT,
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // First, check the geodetic longitude, which should be the
                // same as longitudes in other coordinate system.
                //
                testutil::CHCKSD(b"GLON", save.GLON, b"~", save.ELON, MED, OK, ctx)?;

                //
                // Derive the geodetic altitude vector, GVEC, which is
                // vertically pointing from the nadir on the surface of the
                // reference ellipsoid to the grid point.

                spicelib::LATREC(save.GALT, save.GLON, save.GLAT, save.GVEC.as_slice_mut());

                //
                // Get rectangular coordinates of the nadir.
                //
                spicelib::VSUB(
                    save.REC.as_slice(),
                    save.GVEC.as_slice(),
                    save.NADIR.as_slice_mut(),
                );

                //
                // Being on the surface of the reference ellipsoid, the
                // rectangular coordinates of the nadir should satisfy the
                // spheroidal surface equation associated with the reference
                // ellipsoid.

                //
                // Evaluate the spheroidal surface equation.
                //
                save.SRFEQN = (((f64::powi((save.NADIR[1] / AE), 2)
                    + f64::powi((save.NADIR[2] / AE), 2))
                    + f64::powi((save.NADIR[3] / BE), 2))
                    - 1.0);

                //
                // Check the nadir is on the surface.
                //
                testutil::CHCKSD(b"SRFEQN", save.SRFEQN, b"~", 0.0, MED, OK, ctx)?;

                //
                // Test geodetic to rectangular coordinate conversion.
                //
                testutil::TCASE(&fstr::concat(b"GEOREC: ", &save.CASE), ctx)?;

                //
                // Revert geodetic coordinates to rectangular coordinates.
                //
                spicelib::GEOREC(
                    save.GLON,
                    save.GLAT,
                    save.GALT,
                    AE,
                    F,
                    save.REC.as_slice_mut(),
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Check the reverted coordinates.
                //
                testutil::CHCKSD(b"REC(1)", save.REC[1], b"~", save.EX, LSE, OK, ctx)?;
                testutil::CHCKSD(b"REC(2)", save.REC[2], b"~", save.EY, LSE, OK, ctx)?;
                testutil::CHCKSD(b"REC(3)", save.REC[3], b"~", save.EZ, LSE, OK, ctx)?;

                //
                // Test surface to rectangular coordinates conversion.
                //

                //
                // Set the surface coordinates ( longitude and latitude )
                // computed at the grid point.
                //
                save.SLON = save.ELON;
                save.SLAT = save.ELAT;

                //
                // Loop over NAIF bodies
                //
                for IB in 1..=NB {
                    //
                    // Identity NAIF body and surface point on the test
                    // description.
                    //
                    fstr::assign(&mut save.CASE, b"# # #");
                    spicelib::REPMI(
                        &save.CASE.to_vec(),
                        b"#",
                        save.BODY[IB],
                        &mut save.CASE,
                        ctx,
                    );
                    spicelib::REPMD(
                        &save.CASE.to_vec(),
                        b"#",
                        save.SLON,
                        14,
                        &mut save.CASE,
                        ctx,
                    );
                    spicelib::REPMD(
                        &save.CASE.to_vec(),
                        b"#",
                        save.SLAT,
                        14,
                        &mut save.CASE,
                        ctx,
                    );

                    testutil::TCASE(&fstr::concat(b"SRFREC: ", &save.CASE), ctx)?;

                    //
                    // Convert surface coordinates to rectangular
                    // coordinates.
                    //
                    spicelib::SRFREC(
                        save.BODY[IB],
                        save.SLON,
                        save.SLAT,
                        save.NADIR.as_slice_mut(),
                        ctx,
                    )?;

                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Convert rectangular coordinates to latitudinal
                    // coordinates.
                    //
                    spicelib::RECLAT(
                        save.NADIR.as_slice(),
                        &mut save.RADIUS,
                        &mut save.LLON,
                        &mut save.LLAT,
                    );

                    //
                    // The longitude and latitude pair of latitudinal
                    // coordinates should be that of surface coordinates.
                    //
                    testutil::CHCKSD(b"LLON", save.LLON, b"~", save.ELON, MED, OK, ctx)?;
                    testutil::CHCKSD(b"LLAT", save.LLAT, b"~", save.ELAT, MED, OK, ctx)?;

                    //
                    // Get semi-axes, RADII, of the naif body.
                    //
                    spicelib::BODVCD(
                        save.BODY[IB],
                        b"RADII",
                        3,
                        &mut save.NDIM,
                        save.RADII.as_slice_mut(),
                        ctx,
                    )?;

                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Being on the surface of the naif body, the recovered
                    // rectangular coordinates should satisfy the spheroidal
                    // surface equation associated with the naif body.
                    //
                    // Evaluate the spheroidal surface equation.
                    //
                    save.SRFEQN = (((f64::powi((save.NADIR[1] / save.RADII[1]), 2)
                        + f64::powi((save.NADIR[2] / save.RADII[2]), 2))
                        + f64::powi((save.NADIR[3] / save.RADII[3]), 2))
                        - 1.0);

                    //
                    // Check the nadir is on the surface.
                    //
                    testutil::CHCKSD(b"SRFEQN", save.SRFEQN, b"~", 0.0, MED, OK, ctx)?;
                }
            }
        }
    }

    //
    // Exception test cases.
    //
    testutil::TCASE(b"GEOREC exceptions", ctx)?;

    spicelib::GEOREC(1.0, 1.0, 1.0, 0.0, 0.5, save.REC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    spicelib::GEOREC(1.0, 1.0, 1.0, -1.0, 0.5, save.REC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    spicelib::GEOREC(1.0, 1.0, 1.0, 1.0, 1.0, save.REC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    spicelib::GEOREC(1.0, 1.0, 1.0, 1.0, 1.1, save.REC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    testutil::TCASE(b"RECGEO exceptions", ctx)?;

    spicelib::VPACK(1.0, 1.0, 1.0, save.REC.as_slice_mut());

    spicelib::RECGEO(
        save.REC.as_slice(),
        0.0,
        0.5,
        &mut save.GLON,
        &mut save.GLAT,
        &mut save.GALT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    spicelib::RECGEO(
        save.REC.as_slice(),
        -1.0,
        0.5,
        &mut save.GLON,
        &mut save.GLAT,
        &mut save.GALT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    spicelib::RECGEO(
        save.REC.as_slice(),
        1.0,
        1.0,
        &mut save.GLON,
        &mut save.GLAT,
        &mut save.GALT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    spicelib::RECGEO(
        save.REC.as_slice(),
        1.0,
        1.1,
        &mut save.GLON,
        &mut save.GLAT,
        &mut save.GALT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // (0,0,0) test cases.
    //
    spicelib::VPACK(0.0, 0.0, 0.0, save.REC.as_slice_mut());

    testutil::TCASE(b"RECCYL (0,0,0) input", ctx)?;
    spicelib::RECCYL(
        save.REC.as_slice(),
        &mut save.RHO,
        &mut save.LON,
        &mut save.Z,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"RHO", save.RHO, b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(b"LON", save.LON, b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(b"Z", save.Z, b"=", 0.0, TGH, OK, ctx)?;

    testutil::TCASE(b"RECGEO (0,0,0) input", ctx)?;
    spicelib::RECGEO(
        save.REC.as_slice(),
        2.0,
        0.5,
        &mut save.GLON,
        &mut save.GLAT,
        &mut save.GALT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"GLON", save.GLON, b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(
        b"GLAT",
        save.GLAT,
        b"=",
        spicelib::HALFPI(ctx),
        TGH,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"GALT", save.GALT, b"=", -1.0, TGH, OK, ctx)?;

    testutil::TCASE(b"RECLAT (0,0,0) input", ctx)?;
    spicelib::RECLAT(
        save.REC.as_slice(),
        &mut save.RADIUS,
        &mut save.LLON,
        &mut save.LLAT,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"RADIUS", save.RADIUS, b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(b"LLON", save.LLON, b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(b"LLAT", save.LLAT, b"=", 0.0, TGH, OK, ctx)?;

    testutil::TCASE(b"RECRAD (0,0,0) input", ctx)?;
    spicelib::RECRAD(
        save.REC.as_slice(),
        &mut save.RANGE,
        &mut save.RA,
        &mut save.DEC,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"RANGE", save.RANGE, b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(b"RA", save.RA, b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(b"DEC", save.DEC, b"=", 0.0, TGH, OK, ctx)?;

    testutil::TCASE(b"RECSPH (0,0,0) input", ctx)?;
    spicelib::RECSPH(
        save.REC.as_slice(),
        &mut save.R,
        &mut save.CLAT,
        &mut save.LONG,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"R", save.R, b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(b"CLAT", save.CLAT, b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(b"LONG", save.LONG, b"=", 0.0, TGH, OK, ctx)?;

    testutil::TCASE(b"CYLLAT (0,0,0) input", ctx)?;
    spicelib::CYLLAT(
        0.0,
        0.0,
        0.0,
        &mut save.RADIUS,
        &mut save.LLON,
        &mut save.LLAT,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"RADIUS", save.RADIUS, b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(b"LLON", save.LLON, b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(b"LLAT", save.LLAT, b"=", 0.0, TGH, OK, ctx)?;

    testutil::TCASE(b"CYLREC (0,0,0) input", ctx)?;
    spicelib::CYLREC(0.0, 0.0, 0.0, save.REC.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"REC(1)", save.REC[1], b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(b"REC(2)", save.REC[2], b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(b"REC(3)", save.REC[3], b"=", 0.0, TGH, OK, ctx)?;

    testutil::TCASE(b"CYLSPH (0,0,0) input", ctx)?;
    spicelib::CYLSPH(0.0, 0.0, 0.0, &mut save.R, &mut save.CLAT, &mut save.LONG);
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"R", save.R, b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(b"CLAT", save.CLAT, b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(b"LONG", save.LONG, b"=", 0.0, TGH, OK, ctx)?;

    testutil::TCASE(b"GEOREC (0,0,0) input", ctx)?;
    spicelib::GEOREC(0.0, 0.0, 0.0, 2.0, 0.5, save.REC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"REC(1)", save.REC[1], b"=", 2.0, TGH, OK, ctx)?;
    testutil::CHCKSD(b"REC(2)", save.REC[2], b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(b"REC(3)", save.REC[3], b"=", 0.0, TGH, OK, ctx)?;

    testutil::TCASE(b"LATCYL (0,0,0) input", ctx)?;
    spicelib::LATCYL(0.0, 0.0, 0.0, &mut save.RHO, &mut save.LON, &mut save.Z);
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"RHO", save.RHO, b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(b"LON", save.LON, b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(b"Z", save.Z, b"=", 0.0, TGH, OK, ctx)?;

    testutil::TCASE(b"LATREC (0,0,0) input", ctx)?;
    spicelib::LATREC(0.0, 0.0, 0.0, save.REC.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"REC(1)", save.REC[1], b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(b"REC(2)", save.REC[2], b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(b"REC(3)", save.REC[3], b"=", 0.0, TGH, OK, ctx)?;

    testutil::TCASE(b"LATSPH (0,0,0) input", ctx)?;
    spicelib::LATSPH(
        0.0,
        0.0,
        0.0,
        &mut save.R,
        &mut save.CLAT,
        &mut save.LONG,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"R", save.R, b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(
        b"CLAT",
        save.CLAT,
        b"=",
        spicelib::HALFPI(ctx),
        TGH,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LONG", save.LONG, b"=", 0.0, TGH, OK, ctx)?;

    testutil::TCASE(b"RADREC (0,0,0) input", ctx)?;
    spicelib::RADREC(0.0, 0.0, 0.0, save.REC.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"REC(1)", save.REC[1], b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(b"REC(2)", save.REC[2], b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(b"REC(3)", save.REC[3], b"=", 0.0, TGH, OK, ctx)?;

    testutil::TCASE(b"SPHCYL (0,0,0) input", ctx)?;
    spicelib::SPHCYL(0.0, 0.0, 0.0, &mut save.RHO, &mut save.LON, &mut save.Z);
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"RHO", save.RHO, b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(b"LON", save.LON, b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(b"Z", save.Z, b"=", 0.0, TGH, OK, ctx)?;

    testutil::TCASE(b"SPHLAT (0,0,0) input", ctx)?;
    spicelib::SPHLAT(
        0.0,
        0.0,
        0.0,
        &mut save.RADIUS,
        &mut save.LLON,
        &mut save.LLAT,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"RADIUS", save.RADIUS, b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(b"LLON", save.LLON, b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(
        b"LLAT",
        save.LLAT,
        b"=",
        spicelib::HALFPI(ctx),
        TGH,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"SPHREC (0,0,0) input", ctx)?;
    spicelib::SPHREC(0.0, 0.0, 0.0, save.REC.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"REC(1)", save.REC[1], b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(b"REC(2)", save.REC[2], b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(b"REC(3)", save.REC[3], b"=", 0.0, TGH, OK, ctx)?;

    testutil::TCASE(b"SRFREC (0,0) input", ctx)?;
    spicelib::BODVCD(
        399,
        b"RADII",
        3,
        &mut save.NDIM,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::SRFREC(399, 0.0, 0.0, save.REC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"REC(1)", save.REC[1], b"=", save.RADII[1], TGH, OK, ctx)?;
    testutil::CHCKSD(b"REC(2)", save.REC[2], b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(b"REC(3)", save.REC[3], b"=", 0.0, TGH, OK, ctx)?;

    //
    // Test cases, with one or more items set to 0.
    //
    testutil::TCASE(b"CYLLAT (0,HALFPI,0) input", ctx)?;
    spicelib::CYLLAT(
        0.0,
        spicelib::HALFPI(ctx),
        0.0,
        &mut save.RADIUS,
        &mut save.LLON,
        &mut save.LLAT,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"RADIUS", save.RADIUS, b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(
        b"LLON",
        save.LLON,
        b"=",
        spicelib::HALFPI(ctx),
        TGH,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LLAT", save.LLAT, b"=", 0.0, TGH, OK, ctx)?;

    testutil::TCASE(b"RECCYL (0,0,1) input", ctx)?;
    spicelib::VPACK(0.0, 0.0, 1.0, save.REC.as_slice_mut());
    spicelib::RECCYL(
        save.REC.as_slice(),
        &mut save.RHO,
        &mut save.LON,
        &mut save.Z,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"RHO", save.RHO, b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(b"LON", save.LON, b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(b"Z", save.Z, b"=", 1.0, TGH, OK, ctx)?;

    testutil::TCASE(b"RECSPH (0,0,1) input", ctx)?;
    spicelib::VPACK(0.0, 0.0, 1.0, save.REC.as_slice_mut());
    spicelib::RECSPH(
        save.REC.as_slice(),
        &mut save.R,
        &mut save.CLAT,
        &mut save.LONG,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"R", save.R, b"=", 1.0, TGH, OK, ctx)?;
    testutil::CHCKSD(b"CLAT", save.CLAT, b"=", 0.0, TGH, OK, ctx)?;
    testutil::CHCKSD(b"LONG", save.LONG, b"=", 0.0, TGH, OK, ctx)?;

    //
    // Test cases with angular inputs outside of normal ranges.
    //
    // THESE TEST HAVE NOT BEED IMPLEMENTED FOR N0066. THEY MIGHT
    // BE IMPLEMENTED AT A LATER TIME.
    //

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
