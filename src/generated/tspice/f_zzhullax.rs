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
const ANGLIM: f64 = 0.000000000001;
const TIGHT: f64 = 0.0000000000001;
const LNSIZE: i32 = 80;

struct SaveVars {
    TITLE: Vec<u8>,
    QNAME: Vec<u8>,
    AXIS: StackArray<f64, 3>,
    BOUNDS: ActualArray2D<f64>,
    CENTER: StackArray<f64, 3>,
    DELTA: f64,
    E: f64,
    L: f64,
    LIMIT: f64,
    S: f64,
    SEP: f64,
    SUM: StackArray<f64, 3>,
    THETA: f64,
    V: StackArray<f64, 3>,
    W: f64,
    X: f64,
    XFORM: StackArray2D<f64, 9>,
    Y: f64,
    J: i32,
    N: i32,
    SEED: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut QNAME = vec![b' '; LNSIZE as usize];
        let mut AXIS = StackArray::<f64, 3>::new(1..=3);
        let mut BOUNDS = ActualArray2D::<f64>::new(1..=3, 1..=MAXVRT);
        let mut CENTER = StackArray::<f64, 3>::new(1..=3);
        let mut DELTA: f64 = 0.0;
        let mut E: f64 = 0.0;
        let mut L: f64 = 0.0;
        let mut LIMIT: f64 = 0.0;
        let mut S: f64 = 0.0;
        let mut SEP: f64 = 0.0;
        let mut SUM = StackArray::<f64, 3>::new(1..=3);
        let mut THETA: f64 = 0.0;
        let mut V = StackArray::<f64, 3>::new(1..=3);
        let mut W: f64 = 0.0;
        let mut X: f64 = 0.0;
        let mut XFORM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut Y: f64 = 0.0;
        let mut J: i32 = 0;
        let mut N: i32 = 0;
        let mut SEED: i32 = 0;

        Self {
            TITLE,
            QNAME,
            AXIS,
            BOUNDS,
            CENTER,
            DELTA,
            E,
            L,
            LIMIT,
            S,
            SEP,
            SUM,
            THETA,
            V,
            W,
            X,
            XFORM,
            Y,
            J,
            N,
            SEED,
        }
    }
}

//$Procedure      F_ZZHULLAX ( Test ZZHULLAX )
pub fn F_ZZHULLAX(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Test utility functions
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
    testutil::TOPEN(b"F_ZZHULLAX", ctx)?;

    //*********************************************************************
    //*
    //*    Error cases
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Too few boundary vectors", ctx)?;

    spicelib::CLEARD((3 * MAXVRT), save.BOUNDS.as_slice_mut());

    spicelib::ZZHULLAX(
        b"Test",
        2,
        save.BOUNDS.as_slice(),
        save.AXIS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDCOUNT)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Linearly dependent consecutive boundary vectors", ctx)?;

    spicelib::VPACK(0.0, 0.0, 1.0, save.BOUNDS.subarray_mut([1, 1]));
    spicelib::VPACK(1.0, 0.0, 0.0, save.BOUNDS.subarray_mut([1, 2]));
    spicelib::VPACK(2.0, 0.0, 0.0, save.BOUNDS.subarray_mut([1, 3]));

    spicelib::ZZHULLAX(
        b"Test",
        3,
        save.BOUNDS.as_slice(),
        save.AXIS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(DEGENERATECASE)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"FOV region too close to half-space.", ctx)?;

    save.E = 0.0000000000001;
    save.N = 4;

    spicelib::VPACK(1.0, 1.0, save.E, save.BOUNDS.subarray_mut([1, 1]));
    spicelib::VPACK(-1.0, 1.0, save.E, save.BOUNDS.subarray_mut([1, 2]));
    spicelib::VPACK(-1.0, -1.0, save.E, save.BOUNDS.subarray_mut([1, 3]));
    spicelib::VPACK(1.0, -1.0, save.E, save.BOUNDS.subarray_mut([1, 4]));

    spicelib::ZZHULLAX(
        b"Test",
        4,
        save.BOUNDS.as_slice(),
        save.AXIS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(FACENOTFOUND)", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Normal cases
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Narrow square FOV", ctx)?;

    save.E = 0.000001;
    save.N = 4;

    spicelib::VPACK(save.E, save.E, 1.0, save.BOUNDS.subarray_mut([1, 1]));
    spicelib::VPACK(-save.E, save.E, 1.0, save.BOUNDS.subarray_mut([1, 2]));
    spicelib::VPACK(-save.E, -save.E, 1.0, save.BOUNDS.subarray_mut([1, 3]));
    spicelib::VPACK(save.E, -save.E, 1.0, save.BOUNDS.subarray_mut([1, 4]));

    spicelib::ZZHULLAX(
        b"Test",
        save.N,
        save.BOUNDS.as_slice(),
        save.AXIS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Validate the axis.
    //
    for I in 1..=save.N {
        save.SEP = spicelib::VSEP(save.BOUNDS.subarray([1, I]), save.AXIS.as_slice(), ctx);

        fstr::assign(&mut save.QNAME, b"Boundary # sep");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

        save.LIMIT = (spicelib::HALFPI(ctx) - ANGLIM);

        testutil::CHCKSD(&save.QNAME, save.SEP, b"<", save.LIMIT, 0.0, OK, ctx)?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Narrow square FOV, boundary order reversed", ctx)?;

    save.E = 0.000001;
    save.N = 4;

    spicelib::VPACK(save.E, save.E, 1.0, save.BOUNDS.subarray_mut([1, 4]));
    spicelib::VPACK(-save.E, save.E, 1.0, save.BOUNDS.subarray_mut([1, 3]));
    spicelib::VPACK(-save.E, -save.E, 1.0, save.BOUNDS.subarray_mut([1, 2]));
    spicelib::VPACK(save.E, -save.E, 1.0, save.BOUNDS.subarray_mut([1, 1]));

    spicelib::ZZHULLAX(
        b"Test",
        save.N,
        save.BOUNDS.as_slice(),
        save.AXIS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Validate the axis.
    //
    for I in 1..=save.N {
        save.SEP = spicelib::VSEP(save.BOUNDS.subarray([1, I]), save.AXIS.as_slice(), ctx);

        fstr::assign(&mut save.QNAME, b"Boundary # sep");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

        save.LIMIT = (spicelib::HALFPI(ctx) - ANGLIM);

        testutil::CHCKSD(&save.QNAME, save.SEP, b"<", save.LIMIT, 0.0, OK, ctx)?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Long, narrow rectangular FOV", ctx)?;

    save.W = 0.000001;
    save.L = (1.0 - save.W);

    save.N = 4;

    spicelib::VPACK(save.L, save.E, 1.0, save.BOUNDS.subarray_mut([1, 1]));
    spicelib::VPACK(-save.L, save.E, 1.0, save.BOUNDS.subarray_mut([1, 2]));
    spicelib::VPACK(-save.L, -save.E, 1.0, save.BOUNDS.subarray_mut([1, 3]));
    spicelib::VPACK(save.L, -save.E, 1.0, save.BOUNDS.subarray_mut([1, 4]));

    spicelib::ZZHULLAX(
        b"Test",
        save.N,
        save.BOUNDS.as_slice(),
        save.AXIS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Validate the axis.
    //
    for I in 1..=save.N {
        save.SEP = spicelib::VSEP(save.BOUNDS.subarray([1, I]), save.AXIS.as_slice(), ctx);

        fstr::assign(&mut save.QNAME, b"Boundary # sep");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

        save.LIMIT = (spicelib::HALFPI(ctx) - ANGLIM);

        testutil::CHCKSD(&save.QNAME, save.SEP, b"<", save.LIMIT, 0.0, OK, ctx)?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Pentagonal FOV consisting of \"dented\" rectangle", ctx)?;

    save.E = 0.1;
    save.N = 5;

    spicelib::VPACK(save.E, save.E, 1.0, save.BOUNDS.subarray_mut([1, 1]));
    spicelib::VPACK(
        0.0,
        (save.E / 2 as f64),
        1.0,
        save.BOUNDS.subarray_mut([1, 2]),
    );
    spicelib::VPACK(-save.E, save.E, 1.0, save.BOUNDS.subarray_mut([1, 3]));
    spicelib::VPACK(-save.E, -save.E, 1.0, save.BOUNDS.subarray_mut([1, 4]));
    spicelib::VPACK(save.E, -save.E, 1.0, save.BOUNDS.subarray_mut([1, 5]));

    spicelib::ZZHULLAX(
        b"Test",
        save.N,
        save.BOUNDS.as_slice(),
        save.AXIS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Validate the axis.
    //
    for I in 1..=save.N {
        save.SEP = spicelib::VSEP(save.BOUNDS.subarray([1, I]), save.AXIS.as_slice(), ctx);

        fstr::assign(&mut save.QNAME, b"Boundary # sep");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

        save.LIMIT = (spicelib::HALFPI(ctx) - ANGLIM);

        testutil::CHCKSD(&save.QNAME, save.SEP, b"<", save.LIMIT, 0.0, OK, ctx)?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"5-pointed star", ctx)?;

    save.E = 0.1;
    save.N = 10;

    save.DELTA = (spicelib::TWOPI(ctx) / save.N as f64);

    for I in 1..=save.N {
        if spicelib::ODD(I) {
            //
            // This boundary vector corresponds to a point
            // of the star.
            //
            save.S = save.E;
        } else {
            save.S = (save.E / 3 as f64);
        }

        save.THETA = (((I - 1) as f64) * save.DELTA);

        spicelib::VPACK(
            (save.S * f64::cos(save.THETA)),
            (save.S * f64::sin(save.THETA)),
            1.0,
            save.BOUNDS.subarray_mut([1, I]),
        );
    }

    spicelib::ZZHULLAX(
        b"Test",
        save.N,
        save.BOUNDS.as_slice(),
        save.AXIS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Validate the axis.
    //
    for I in 1..=save.N {
        save.SEP = spicelib::VSEP(save.BOUNDS.subarray([1, I]), save.AXIS.as_slice(), ctx);

        fstr::assign(&mut save.QNAME, b"Boundary # sep");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

        save.LIMIT = (spicelib::HALFPI(ctx) - ANGLIM);

        testutil::CHCKSD(&save.QNAME, save.SEP, b"<", save.LIMIT, 0.0, OK, ctx)?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"5-pointed star, reversed boundary order", ctx)?;

    save.E = 0.1;
    save.N = 10;

    save.DELTA = (spicelib::TWOPI(ctx) / save.N as f64);

    for I in 1..=save.N {
        if spicelib::ODD(I) {
            //
            // This boundary vector corresponds to a point
            // of the star.
            //
            save.S = save.E;
        } else {
            save.S = (save.E / 3 as f64);
        }

        save.THETA = (((I - 1) as f64) * save.DELTA);

        save.J = ((save.N + 1) - I);

        spicelib::VPACK(
            (save.S * f64::cos(save.THETA)),
            (save.S * f64::sin(save.THETA)),
            1.0,
            save.BOUNDS.subarray_mut([1, save.J]),
        );
    }

    spicelib::ZZHULLAX(
        b"Test",
        save.N,
        save.BOUNDS.as_slice(),
        save.AXIS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Validate the axis.
    //
    for I in 1..=save.N {
        save.SEP = spicelib::VSEP(save.BOUNDS.subarray([1, I]), save.AXIS.as_slice(), ctx);

        fstr::assign(&mut save.QNAME, b"Boundary # sep");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

        save.LIMIT = (spicelib::HALFPI(ctx) - ANGLIM);

        testutil::CHCKSD(&save.QNAME, save.SEP, b"<", save.LIMIT, 0.0, OK, ctx)?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"50-gon star with 5 points", ctx)?;

    save.E = 0.1;
    save.N = 50;

    save.DELTA = (spicelib::TWOPI(ctx) / save.N as f64);

    for I in 1..=save.N {
        if (intrinsics::MOD(I, 10) == 0) {
            //
            // This boundary vector corresponds to a point
            // of the star.
            //
            save.S = save.E;
        } else {
            save.S = (save.E / 3 as f64);
        }

        save.THETA = (((I - 1) as f64) * save.DELTA);

        spicelib::VPACK(
            (save.S * f64::cos(save.THETA)),
            (save.S * f64::sin(save.THETA)),
            1.0,
            save.BOUNDS.subarray_mut([1, I]),
        );
    }

    spicelib::ZZHULLAX(
        b"Test",
        save.N,
        save.BOUNDS.as_slice(),
        save.AXIS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Validate the axis.
    //
    for I in 1..=save.N {
        save.SEP = spicelib::VSEP(save.BOUNDS.subarray([1, I]), save.AXIS.as_slice(), ctx);

        fstr::assign(&mut save.QNAME, b"Boundary # sep");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

        save.LIMIT = (spicelib::HALFPI(ctx) - ANGLIM);

        testutil::CHCKSD(&save.QNAME, save.SEP, b"<", save.LIMIT, 0.0, OK, ctx)?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"50-gon star with 5 points, wide FOV", ctx)?;

    save.E = 1.5;
    save.N = 50;

    save.DELTA = (spicelib::TWOPI(ctx) / save.N as f64);

    for I in 1..=save.N {
        if (intrinsics::MOD(I, 10) == 0) {
            //
            // This boundary vector corresponds to a point
            // of the star.
            //
            save.S = save.E;
        } else {
            save.S = (save.E / 3 as f64);
        }

        save.THETA = (((I - 1) as f64) * save.DELTA);

        spicelib::VPACK(
            (save.S * f64::cos(save.THETA)),
            (save.S * f64::sin(save.THETA)),
            1.0,
            save.BOUNDS.subarray_mut([1, I]),
        );
    }

    spicelib::ZZHULLAX(
        b"Test",
        save.N,
        save.BOUNDS.as_slice(),
        save.AXIS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Validate the axis.
    //
    for I in 1..=save.N {
        save.SEP = spicelib::VSEP(save.BOUNDS.subarray([1, I]), save.AXIS.as_slice(), ctx);

        fstr::assign(&mut save.QNAME, b"Boundary # sep");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

        save.LIMIT = (spicelib::HALFPI(ctx) - ANGLIM);

        testutil::CHCKSD(&save.QNAME, save.SEP, b"<", save.LIMIT, 0.0, OK, ctx)?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Random vectors, constrained within rectangle", ctx)?;

    save.E = 1.0;
    save.N = 1000;

    save.SEED = -1;
    //
    // Burn the first two values.
    //
    save.X = testutil::T_RANDD(
        -((2 as f64) * save.E),
        ((2 as f64) * save.E),
        &mut save.SEED,
        ctx,
    )?;
    save.Y = testutil::T_RANDD(-save.E, save.E, &mut save.SEED, ctx)?;

    for I in 1..=save.N {
        save.X = testutil::T_RANDD(
            -((2 as f64) * save.E),
            ((2 as f64) * save.E),
            &mut save.SEED,
            ctx,
        )?;
        save.Y = testutil::T_RANDD(-save.E, save.E, &mut save.SEED, ctx)?;

        spicelib::VPACK(save.X, save.Y, 1.0, save.BOUNDS.subarray_mut([1, I]));
    }

    spicelib::ZZHULLAX(
        b"Test",
        save.N,
        save.BOUNDS.as_slice(),
        save.AXIS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Validate the axis.
    //
    for I in 1..=save.N {
        save.SEP = spicelib::VSEP(save.BOUNDS.subarray([1, I]), save.AXIS.as_slice(), ctx);

        fstr::assign(&mut save.QNAME, b"Boundary # sep");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

        save.LIMIT = (spicelib::HALFPI(ctx) - ANGLIM);

        testutil::CHCKSD(&save.QNAME, save.SEP, b"<", save.LIMIT, 0.0, OK, ctx)?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    //
    //     Test some regular polygons. Let the number of
    //     sides range from 3 to 100. Let the angular extent
    //     of the FOVs increase with the number of sides.
    //
    //
    //     We'll use a transformation to a frame that's not lined
    //     up with the original one.
    //
    spicelib::EUL2M(0.6, -0.4, 1.2, 1, 2, 3, save.XFORM.as_slice_mut(), ctx)?;

    {
        let m1__: i32 = 3;
        let m2__: i32 = 100;
        let m3__: i32 = 1;
        save.N = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.E = (0.01 * save.N as f64);

            //
            //---- Case -------------------------------------------------------------
            //
            fstr::assign(&mut save.TITLE, b"Regular polygon: N = #; E = #");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.N, &mut save.TITLE, ctx);
            spicelib::REPMD(&save.TITLE.to_vec(), b"#", save.E, 6, &mut save.TITLE, ctx);

            testutil::TCASE(&save.TITLE, ctx)?;

            save.DELTA = (spicelib::TWOPI(ctx) / save.N as f64);

            for I in 1..=save.N {
                save.THETA = (((I - 1) as f64) * save.DELTA);

                spicelib::VPACK(
                    (save.E * f64::cos(save.THETA)),
                    (save.E * f64::sin(save.THETA)),
                    1.0,
                    save.V.as_slice_mut(),
                );

                spicelib::MXV(
                    save.XFORM.as_slice(),
                    save.V.as_slice(),
                    save.BOUNDS.subarray_mut([1, I]),
                );
            }

            spicelib::ZZHULLAX(
                b"Test",
                save.N,
                save.BOUNDS.as_slice(),
                save.AXIS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Validate the axis.
            //
            for I in 1..=save.N {
                save.SEP = spicelib::VSEP(save.BOUNDS.subarray([1, I]), save.AXIS.as_slice(), ctx);

                fstr::assign(&mut save.QNAME, b"Boundary # sep");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

                save.LIMIT = (spicelib::HALFPI(ctx) - ANGLIM);

                testutil::CHCKSD(&save.QNAME, save.SEP, b"<", save.LIMIT, 0.0, OK, ctx)?;
            }

            //
            // For polygons with even numbers of edges, the axis
            // should be very close to the average of the boundary
            // vectors.
            //
            if spicelib::EVEN(save.N) {
                spicelib::CLEARD(3, save.SUM.as_slice_mut());

                for I in 1..=save.N {
                    spicelib::VADD(
                        save.SUM.as_slice(),
                        save.BOUNDS.subarray([1, I]),
                        save.V.as_slice_mut(),
                    );
                    spicelib::VEQU(save.V.as_slice(), save.SUM.as_slice_mut());
                }

                spicelib::VSCL(
                    (1.0 / save.N as f64),
                    save.SUM.as_slice(),
                    save.CENTER.as_slice_mut(),
                );

                testutil::CHCKAD(
                    b"AXIS",
                    save.AXIS.as_slice(),
                    b"~",
                    save.CENTER.as_slice(),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;
            }

            save.N += m3__;
        }
    }

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
