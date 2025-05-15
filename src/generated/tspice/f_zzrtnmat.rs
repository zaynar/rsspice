//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CNVTOL: f64 = 0.000001;
const NWMAX: i32 = 15;
const NWDIST: i32 = 5;
const NWSEP: i32 = 5;
const NWRR: i32 = 5;
const NWUDS: i32 = 5;
const NWPA: i32 = 5;
const NWILUM: i32 = 5;
const ADDWIN: f64 = 0.5;
const FRMNLN: i32 = 32;
const FOVTLN: i32 = 40;
const FTCIRC: &[u8] = b"CIRCLE";
const FTELLI: &[u8] = b"ELLIPSE";
const FTPOLY: &[u8] = b"POLYGON";
const FTRECT: &[u8] = b"RECTANGLE";
const ANNULR: &[u8] = b"ANNULAR";
const ANY: &[u8] = b"ANY";
const PARTL: &[u8] = b"PARTIAL";
const FULL: &[u8] = b"FULL";
const DSSHAP: &[u8] = b"DSK";
const EDSHAP: &[u8] = b"ELLIPSOID";
const PTSHAP: &[u8] = b"POINT";
const RYSHAP: &[u8] = b"RAY";
const SPSHAP: &[u8] = b"SPHERE";
const NOCTYP: i32 = 4;
const OCLLN: i32 = 7;
const SHPLEN: i32 = 9;
const MAXVRT: i32 = 10000;
const CIRFOV: &[u8] = b"CIRCLE";
const ELLFOV: &[u8] = b"ELLIPSE";
const POLFOV: &[u8] = b"POLYGON";
const RECFOV: &[u8] = b"RECTANGLE";
const RECSYS: &[u8] = b"RECTANGULAR";
const LATSYS: &[u8] = b"LATITUDINAL";
const SPHSYS: &[u8] = b"SPHERICAL";
const RADSYS: &[u8] = b"RA/DEC";
const CYLSYS: &[u8] = b"CYLINDRICAL";
const GEOSYS: &[u8] = b"GEODETIC";
const PGRSYS: &[u8] = b"PLANETOGRAPHIC";
const XCRD: &[u8] = b"X";
const YCRD: &[u8] = b"Y";
const ZCRD: &[u8] = b"Z";
const RADCRD: &[u8] = b"RADIUS";
const LONCRD: &[u8] = b"LONGITUDE";
const LATCRD: &[u8] = b"LATITUDE";
const RACRD: &[u8] = b"RIGHT ASCENSION";
const DECCRD: &[u8] = b"DECLINATION";
const RNGCRD: &[u8] = b"RANGE";
const CLTCRD: &[u8] = b"COLATITUDE";
const ALTCRD: &[u8] = b"ALTITUDE";
const POSDEF: &[u8] = b"POSITION";
const SOBDEF: &[u8] = b"SUB-OBSERVER POINT";
const SINDEF: &[u8] = b"SURFACE INTERCEPT POINT";
const NWREL: i32 = 5;
const NWLONG: i32 = 7;
const EXWIDX: i32 = ((NWREL + NWLONG) + 1);
const MXBEGM: i32 = 55;
const MXENDM: i32 = 13;
const MXMSG: i32 = ((MXBEGM + MXENDM) + 10);
const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;
const LNSIZE: i32 = 80;
const TIGHT: f64 = 0.00000000000001;
const EPSLAT: f64 = 0.1;
const NLATSM: i32 = 5;
const NLONSM: i32 = 8;

struct SaveVars {
    TITLE: Vec<u8>,
    DELTLA: f64,
    DELTLO: f64,
    M: StackArray2D<f64, 9>,
    LAT: f64,
    LON: f64,
    V: StackArray<f64, 3>,
    XM: StackArray2D<f64, 9>,
    Z: StackArray<f64, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut DELTLA: f64 = 0.0;
        let mut DELTLO: f64 = 0.0;
        let mut M = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut LAT: f64 = 0.0;
        let mut LON: f64 = 0.0;
        let mut V = StackArray::<f64, 3>::new(1..=3);
        let mut XM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut Z = StackArray::<f64, 3>::new(1..=3);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            Z.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            TITLE,
            DELTLA,
            DELTLO,
            M,
            LAT,
            LON,
            V,
            XM,
            Z,
        }
    }
}

//$Procedure      F_ZZRTNMAT ( Test ZZRTNMAT )
pub fn F_ZZRTNMAT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Small delta value used for near 0 and pi
    // latitudes.
    //

    //
    // Numbers of latitude and longitude samples.
    //

    //
    // Local Variables
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
    testutil::TOPEN(b"F_ZZRTNMAT", ctx)?;

    //*********************************************************************
    //*
    //*    Error cases
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Input position is on the Z axis", ctx)?;

    spicelib::ZZRTNMAT(save.Z.as_slice(), save.M.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(DEGENERATECASE)", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Normal cases
    //*
    //*********************************************************************

    //
    // We're going to loop over a variety of state vectors
    // and all coordinate system/coordinate combinations.
    // For each test, the derivative signs we find must match
    // those of the corresponding coordinate derivative.
    //

    //
    // DELTLA and DELTLO are, respectively the latitude and longitude
    // increments we use to create different direction vectors.
    //
    save.DELTLA = (spicelib::PI(ctx) / (NLATSM - 1) as f64);
    save.DELTLO = (((2 as f64) * spicelib::PI(ctx)) / NLONSM as f64);

    for NLON in 1..=NLONSM {
        for NLAT in 1..=NLATSM {
            //
            // Create the latitudinal coordinates of a position vector.
            //
            save.LON = (((NLON - 1) as f64) * save.DELTLO);

            save.LAT = ((spicelib::PI(ctx) / 2 as f64) - (((NLAT - 1) as f64) * save.DELTLA));

            //
            // Don't pick a singular latitude value.
            //
            save.LAT = intrinsics::DMIN1(&[save.LAT, ((spicelib::PI(ctx) / 2 as f64) - EPSLAT)]);
            save.LAT = intrinsics::DMAX1(&[save.LAT, (EPSLAT - (spicelib::PI(ctx) / 2 as f64))]);

            //
            // Fill in the position portion of the state vector.
            //
            spicelib::LATREC(1.0, save.LON, save.LAT, save.V.as_slice_mut());

            //
            //---- Case -------------------------------------------------------------
            //
            fstr::assign(&mut save.TITLE, b"lat #(deg); lon #(deg); ");

            spicelib::REPMD(
                &save.TITLE.to_vec(),
                b"#",
                (spicelib::DPR(ctx) * save.LAT),
                3,
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMD(
                &save.TITLE.to_vec(),
                b"#",
                (spicelib::DPR(ctx) * save.LON),
                3,
                &mut save.TITLE,
                ctx,
            );

            testutil::TCASE(&save.TITLE, ctx)?;

            //
            // Construct the expected matrix.
            //
            spicelib::TWOVEC(
                save.V.as_slice(),
                1,
                save.Z.as_slice(),
                3,
                save.XM.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Construct the RTN matrix.
            //
            spicelib::ZZRTNMAT(save.V.as_slice(), save.M.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                b"M",
                save.M.as_slice(),
                b"~~/",
                save.XM.as_slice(),
                9,
                TIGHT,
                OK,
                ctx,
            )?;
        }
        //
        // End of latitude loop.
        //
    }
    //
    // End of longitude loop.
    //

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
