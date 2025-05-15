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

struct SaveVars {
    TITLE: Vec<u8>,
    POINT: StackArray<f64, 2>,
    THETA: f64,
    VERTCS: ActualArray2D<f64>,
    M: i32,
    N: i32,
    W: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut POINT = StackArray::<f64, 2>::new(1..=2);
        let mut THETA: f64 = 0.0;
        let mut VERTCS = ActualArray2D::<f64>::new(1..=2, 1..=MAXVRT);
        let mut M: i32 = 0;
        let mut N: i32 = 0;
        let mut W: i32 = 0;

        Self {
            TITLE,
            POINT,
            THETA,
            VERTCS,
            M,
            N,
            W,
        }
    }
}

//$Procedure      F_ZZWIND2D ( Test ZZWIND2D )
pub fn F_ZZWIND2D(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

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
    // Save everything.
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZWIND2D", ctx)?;

    //*********************************************************************
    //*
    //*    Error cases
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Number of vertices is less than 3.", ctx)?;

    spicelib::CLEARD(2, save.POINT.as_slice_mut());

    save.N = 1;
    spicelib::CLEARD((2 * save.N), save.VERTCS.as_slice_mut());

    save.W = spicelib::ZZWIND2D(save.N, save.VERTCS.as_slice(), save.POINT.as_slice(), ctx)?;

    testutil::CHCKXC(true, b"SPICE(DEGENERATECASE)", OK, ctx)?;

    save.N = 2;
    spicelib::CLEARD((2 * save.N), save.VERTCS.as_slice_mut());

    save.W = spicelib::ZZWIND2D(save.N, save.VERTCS.as_slice(), save.POINT.as_slice(), ctx)?;

    testutil::CHCKXC(true, b"SPICE(DEGENERATECASE)", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Normal cases
    //*
    //*********************************************************************

    //
    // Try some regular polygons; path direction is positive.
    //
    {
        let m1__: i32 = 3;
        let m2__: i32 = 10;
        let m3__: i32 = 1;
        save.N = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            //---- Case -------------------------------------------------------------
            //
            fstr::assign(
                &mut save.TITLE,
                b"Positively oriented, regular polygon. Number of vertices is #; POINT is origin.",
            );

            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.N, &mut save.TITLE, ctx);

            testutil::TCASE(&save.TITLE, ctx)?;

            spicelib::CLEARD(2, save.POINT.as_slice_mut());

            for J in 1..=save.N {
                save.THETA =
                    (((J - 1) as f64) * (((2 as f64) * spicelib::PI(ctx)) / save.N as f64));

                save.VERTCS[[1, J]] = f64::cos(save.THETA);
                save.VERTCS[[2, J]] = f64::sin(save.THETA);
            }

            save.W =
                spicelib::ZZWIND2D(save.N, save.VERTCS.as_slice(), save.POINT.as_slice(), ctx)?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSI(b"W", save.W, b"=", 1, 0, OK, ctx)?;

            save.N += m3__;
        }
    }

    //
    // Try some regular polygons; path direction is negative.
    //
    {
        let m1__: i32 = 3;
        let m2__: i32 = 10;
        let m3__: i32 = 1;
        save.N = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            //---- Case -------------------------------------------------------------
            //
            fstr::assign(
                &mut save.TITLE,
                b"Negatively oriented, regular polygon. Number of vertices is #; POINT is origin.",
            );

            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.N, &mut save.TITLE, ctx);

            testutil::TCASE(&save.TITLE, ctx)?;

            spicelib::CLEARD(2, save.POINT.as_slice_mut());

            for J in 1..=save.N {
                save.THETA =
                    -(((J - 1) as f64) * (((2 as f64) * spicelib::PI(ctx)) / save.N as f64));

                save.VERTCS[[1, J]] = f64::cos(save.THETA);
                save.VERTCS[[2, J]] = f64::sin(save.THETA);
            }

            save.W =
                spicelib::ZZWIND2D(save.N, save.VERTCS.as_slice(), save.POINT.as_slice(), ctx)?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSI(b"W", save.W, b"=", -1, 0, OK, ctx)?;

            save.N += m3__;
        }
    }

    //
    // Check the cases above for an exterior point.
    //
    {
        let m1__: i32 = 3;
        let m2__: i32 = 10;
        let m3__: i32 = 1;
        save.N = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            //---- Case -------------------------------------------------------------
            //
            fstr::assign(
                &mut save.TITLE,
                b"Negatively oriented, regular polygon. Number of vertices is #; POINT is origin.",
            );

            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.N, &mut save.TITLE, ctx);

            testutil::TCASE(&save.TITLE, ctx)?;

            save.POINT[1] = 2.0;
            save.POINT[2] = 3.0;

            for J in 1..=save.N {
                save.THETA =
                    -(((J - 1) as f64) * (((2 as f64) * spicelib::PI(ctx)) / save.N as f64));

                save.VERTCS[[1, J]] = f64::cos(save.THETA);
                save.VERTCS[[2, J]] = f64::sin(save.THETA);
            }

            save.W =
                spicelib::ZZWIND2D(save.N, save.VERTCS.as_slice(), save.POINT.as_slice(), ctx)?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSI(b"W", save.W, b"=", 0, 0, OK, ctx)?;

            save.N += m3__;
        }
    }

    //
    // Repeat the previous cases with non-zero central point.
    //
    {
        let m1__: i32 = 3;
        let m2__: i32 = 10;
        let m3__: i32 = 1;
        save.N = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            //---- Case -------------------------------------------------------------
            //
            fstr::assign(
                &mut save.TITLE,
                b"Positively oriented, regular polygon. N verts is #; POINT is not origin.",
            );

            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.N, &mut save.TITLE, ctx);

            testutil::TCASE(&save.TITLE, ctx)?;

            save.POINT[1] = 2.0;
            save.POINT[2] = 3.0;

            for J in 1..=save.N {
                save.THETA =
                    (((J - 1) as f64) * (((2 as f64) * spicelib::PI(ctx)) / save.N as f64));

                save.VERTCS[[1, J]] = (f64::cos(save.THETA) + save.POINT[1]);
                save.VERTCS[[2, J]] = (f64::sin(save.THETA) + save.POINT[2]);
            }

            save.W =
                spicelib::ZZWIND2D(save.N, save.VERTCS.as_slice(), save.POINT.as_slice(), ctx)?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSI(b"W", save.W, b"=", 1, 0, OK, ctx)?;

            save.N += m3__;
        }
    }

    {
        let m1__: i32 = 3;
        let m2__: i32 = 10;
        let m3__: i32 = 1;
        save.N = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            //---- Case -------------------------------------------------------------
            //
            fstr::assign(
                &mut save.TITLE,
                b"Negatively oriented, regular polygon. N verts = #; POINT is not origin.",
            );

            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.N, &mut save.TITLE, ctx);

            testutil::TCASE(&save.TITLE, ctx)?;

            save.POINT[1] = 2.0;
            save.POINT[2] = 3.0;

            for J in 1..=save.N {
                save.THETA =
                    -(((J - 1) as f64) * (((2 as f64) * spicelib::PI(ctx)) / save.N as f64));

                save.VERTCS[[1, J]] = (f64::cos(save.THETA) + save.POINT[1]);
                save.VERTCS[[2, J]] = (f64::sin(save.THETA) + save.POINT[2]);
            }

            save.W =
                spicelib::ZZWIND2D(save.N, save.VERTCS.as_slice(), save.POINT.as_slice(), ctx)?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSI(b"W", save.W, b"=", -1, 0, OK, ctx)?;

            save.N += m3__;
        }
    }

    //
    // Try some polygons with multiple wraps; path direction is
    // positive.
    //
    {
        let m1__: i32 = 3;
        let m2__: i32 = 10;
        let m3__: i32 = 1;
        save.N = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            //---- Case -------------------------------------------------------------
            //
            fstr::assign(
                &mut save.TITLE,
                b"Positively oriented, regular polygon. Wrap > 1. N verts = #; POINT is not 0.",
            );

            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.N, &mut save.TITLE, ctx);

            testutil::TCASE(&save.TITLE, ctx)?;

            save.POINT[1] = 2.0;
            save.POINT[2] = 3.0;

            save.M = (save.N * save.N);

            for J in 1..=save.M {
                save.THETA =
                    (((J - 1) as f64) * (((2 as f64) * spicelib::PI(ctx)) / save.N as f64));

                save.VERTCS[[1, J]] = (f64::cos(save.THETA) + save.POINT[1]);
                save.VERTCS[[2, J]] = (f64::sin(save.THETA) + save.POINT[2]);
            }

            save.W =
                spicelib::ZZWIND2D(save.M, save.VERTCS.as_slice(), save.POINT.as_slice(), ctx)?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSI(b"W", save.W, b"=", save.N, 0, OK, ctx)?;

            save.N += m3__;
        }
    }

    //
    // Try some irregualr polygons with multiple wraps; path
    // direction is negative.
    //
    {
        let m1__: i32 = 3;
        let m2__: i32 = 10;
        let m3__: i32 = 1;
        save.N = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            //---- Case -------------------------------------------------------------
            //
            fstr::assign(
                &mut save.TITLE,
                b"Negatively oriented, irregular polygon. Wrap > 1. N verts = #; POINT is not 0.",
            );

            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.N, &mut save.TITLE, ctx);

            testutil::TCASE(&save.TITLE, ctx)?;

            save.POINT[1] = 2.0;
            save.POINT[2] = 3.0;

            save.M = (save.N * save.N);

            for J in 1..=save.M {
                save.THETA =
                    -(((J - 1) as f64) * (((2 as f64) * spicelib::PI(ctx)) / save.N as f64));
                //
                // Scale the length of the Jth vertex.
                //
                save.VERTCS[[1, J]] = (((J as f64) * f64::cos(save.THETA)) + save.POINT[1]);
                save.VERTCS[[2, J]] = (((J as f64) * f64::sin(save.THETA)) + save.POINT[2]);
            }

            save.W =
                spicelib::ZZWIND2D(save.M, save.VERTCS.as_slice(), save.POINT.as_slice(), ctx)?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSI(b"W", save.W, b"=", -save.N, 0, OK, ctx)?;

            save.N += m3__;
        }
    }

    //
    // Try some irregualr polygons with multiple wraps; path
    // direction is negative. POINT is outside polygon.
    //
    {
        let m1__: i32 = 3;
        let m2__: i32 = 10;
        let m3__: i32 = 1;
        save.N = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            //---- Case -------------------------------------------------------------
            //
            fstr::assign(
                &mut save.TITLE,
                b"Negatively oriented, irregular polygon. Wrap < -1. N verts = #; exterior POINT.",
            );

            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.N, &mut save.TITLE, ctx);

            testutil::TCASE(&save.TITLE, ctx)?;

            save.POINT[1] = 200.0;
            save.POINT[2] = 300.0;

            save.M = (save.N * save.N);

            for J in 1..=save.M {
                save.THETA =
                    -(((J - 1) as f64) * (((2 as f64) * spicelib::PI(ctx)) / save.N as f64));
                //
                // Scale the length of the Jth vertex.
                //
                save.VERTCS[[1, J]] = ((J as f64) * f64::cos(save.THETA));
                save.VERTCS[[2, J]] = ((J as f64) * f64::sin(save.THETA));
            }

            save.W =
                spicelib::ZZWIND2D(save.M, save.VERTCS.as_slice(), save.POINT.as_slice(), ctx)?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSI(b"W", save.W, b"=", 0, 0, OK, ctx)?;

            save.N += m3__;
        }
    }

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
