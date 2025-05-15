//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const VTIGHT: f64 = 0.00000000000001;
const TIGHT: f64 = 0.000000001;
const MSGLEN: i32 = 320;
const UBPL: i32 = 4;
const NSHAPE: i32 = 3;

struct SaveVars {
    TITLE: Vec<u8>,
    ALPHA: f64,
    BETA: f64,
    GAMMA: f64,
    DLAT: f64,
    DLON: f64,
    E1: StackArray<f64, 3>,
    E2: StackArray<f64, 3>,
    E3: StackArray<f64, 3>,
    LAT: f64,
    LON: f64,
    ORIGIN: StackArray<f64, 3>,
    PLANE: StackArray<f64, 4>,
    PLATES: StackArray3D<f64, 27>,
    R: f64,
    S: f64,
    SCALE: f64,
    SCLPLT: StackArray3D<f64, 27>,
    SCLVTX: StackArray<f64, 3>,
    SIDES: StackArray2D<f64, 9>,
    TOL: f64,
    V: StackArray<f64, 3>,
    VERTEX: StackArray<f64, 3>,
    VTEMP: StackArray<f64, 3>,
    XFORM: StackArray2D<f64, 9>,
    XPT: StackArray<f64, 3>,
    XXPT: StackArray<f64, 3>,
    ZVEC: StackArray<f64, 3>,
    NLAT: i32,
    NLON: i32,
    NRAD: i32,
    NSCALE: i32,
    NXPTS: i32,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TITLE = vec![b' '; MSGLEN as usize];
        let mut ALPHA: f64 = 0.0;
        let mut BETA: f64 = 0.0;
        let mut GAMMA: f64 = 0.0;
        let mut DLAT: f64 = 0.0;
        let mut DLON: f64 = 0.0;
        let mut E1 = StackArray::<f64, 3>::new(1..=3);
        let mut E2 = StackArray::<f64, 3>::new(1..=3);
        let mut E3 = StackArray::<f64, 3>::new(1..=3);
        let mut LAT: f64 = 0.0;
        let mut LON: f64 = 0.0;
        let mut ORIGIN = StackArray::<f64, 3>::new(1..=3);
        let mut PLANE = StackArray::<f64, 4>::new(1..=UBPL);
        let mut PLATES = StackArray3D::<f64, 27>::new(1..=3, 1..=3, 1..=NSHAPE);
        let mut R: f64 = 0.0;
        let mut S: f64 = 0.0;
        let mut SCALE: f64 = 0.0;
        let mut SCLPLT = StackArray3D::<f64, 27>::new(1..=3, 1..=3, 1..=NSHAPE);
        let mut SCLVTX = StackArray::<f64, 3>::new(1..=3);
        let mut SIDES = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut TOL: f64 = 0.0;
        let mut V = StackArray::<f64, 3>::new(1..=3);
        let mut VERTEX = StackArray::<f64, 3>::new(1..=3);
        let mut VTEMP = StackArray::<f64, 3>::new(1..=3);
        let mut XFORM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut XPT = StackArray::<f64, 3>::new(1..=3);
        let mut XXPT = StackArray::<f64, 3>::new(1..=3);
        let mut ZVEC = StackArray::<f64, 3>::new(1..=3);
        let mut NLAT: i32 = 0;
        let mut NLON: i32 = 0;
        let mut NRAD: i32 = 0;
        let mut NSCALE: i32 = 0;
        let mut NXPTS: i32 = 0;
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
            ZVEC.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            TITLE,
            ALPHA,
            BETA,
            GAMMA,
            DLAT,
            DLON,
            E1,
            E2,
            E3,
            LAT,
            LON,
            ORIGIN,
            PLANE,
            PLATES,
            R,
            S,
            SCALE,
            SCLPLT,
            SCLVTX,
            SIDES,
            TOL,
            V,
            VERTEX,
            VTEMP,
            XFORM,
            XPT,
            XXPT,
            ZVEC,
            NLAT,
            NLON,
            NRAD,
            NSCALE,
            NXPTS,
            FOUND,
        }
    }
}

//$Procedure F_INSANG ( INSANG tests )
pub fn F_INSANG(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    // Open the test family.
    //
    testutil::TOPEN(b"F_INSANG", ctx)?;

    //**********************************************************************
    //
    //     Normal cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Simple intersection case.", ctx)?;

    spicelib::VPACK(0.0, 0.0, 10.0, save.E1.as_slice_mut());
    spicelib::VPACK(4.0, 0.0, 10.0, save.E2.as_slice_mut());
    spicelib::VPACK(0.0, 4.0, 10.0, save.E3.as_slice_mut());
    spicelib::VPACK(1.0, 1.0, 10.0, save.V.as_slice_mut());

    spicelib::INSANG(
        save.V.as_slice(),
        save.E1.as_slice(),
        save.E2.as_slice(),
        save.E3.as_slice(),
        &mut save.FOUND,
        &mut save.SCALE,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VSCL(save.SCALE, save.V.as_slice(), save.XPT.as_slice_mut());

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // Create a SPICELIB plane containing the plate defined
    // by E1, E2, E3.
    //
    spicelib::NVC2PL(save.ZVEC.as_slice(), 10.0, save.PLANE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Find the expected intercept.
    //
    spicelib::INRYPL(
        save.ORIGIN.as_slice(),
        save.V.as_slice(),
        save.PLANE.as_slice(),
        &mut save.NXPTS,
        save.XXPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    save.TOL = VTIGHT;

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
    testutil::TCASE(b"Simple non-intersection case.", ctx)?;

    //
    // Use the edge vectors from the previous case. Update V.
    //
    spicelib::VPACK(0.0, 5.0, 10.0, save.V.as_slice_mut());

    spicelib::INSANG(
        save.V.as_slice(),
        save.E1.as_slice(),
        save.E2.as_slice(),
        save.E3.as_slice(),
        &mut save.FOUND,
        &mut save.SCALE,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    //
    // The following set of cases covers a variety of plate shapes,
    // vertex positions, and scales.
    //
    //

    //
    // Create a rotation to be applied to all inputs to INSANG.
    //
    save.ALPHA = (30.0 * spicelib::RPD(ctx));
    save.BETA = -(10.0 * spicelib::RPD(ctx));
    save.GAMMA = (70.0 * spicelib::RPD(ctx));

    spicelib::EUL2M(
        save.GAMMA,
        save.BETA,
        save.ALPHA,
        3,
        2,
        3,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;

    spicelib::IDENT(save.XFORM.as_slice_mut());

    //
    // Create the plate shapes to be used in the following tests.
    //
    //
    //    Plate 1 is an acute triangle.
    //
    spicelib::VPACK(0.0, 0.0, 0.0, save.PLATES.subarray_mut([1, 1, 1]));
    spicelib::VPACK(2.0, 0.0, 0.0, save.PLATES.subarray_mut([1, 2, 1]));
    spicelib::VPACK(1.0, 4.0, 0.0, save.PLATES.subarray_mut([1, 3, 1]));

    //
    // Plate 2 is a right triangle.
    //
    spicelib::VPACK(0.0, 0.0, 0.0, save.PLATES.subarray_mut([1, 1, 2]));
    spicelib::VPACK(4.0, 0.0, 0.0, save.PLATES.subarray_mut([1, 2, 2]));
    spicelib::VPACK(4.0, 3.0, 0.0, save.PLATES.subarray_mut([1, 3, 2]));

    //
    // Plate 3 is an obtuse triangle.
    //
    spicelib::VPACK(0.0, 0.0, 0.0, save.PLATES.subarray_mut([1, 1, 3]));
    spicelib::VPACK(4.0, 0.0, 0.0, save.PLATES.subarray_mut([1, 2, 3]));
    spicelib::VPACK(2.0, 1.0, 0.0, save.PLATES.subarray_mut([1, 3, 3]));

    //
    // Set parameters used for vertex placement.
    //
    save.NLON = 5;
    save.DLON = (spicelib::TWOPI(ctx) / save.NLON as f64);

    save.NLAT = 10;
    save.DLAT = (spicelib::PI(ctx) / (save.NLAT - 1) as f64);

    save.NRAD = 5;

    //
    // Set parameters used to determine the scale of the inputs.
    //
    save.NSCALE = 7;

    for I in 1..=save.NLON {
        save.LON = (((I - 1) as f64) * save.DLON);

        for J in 0..=save.NLAT {
            save.LAT = (spicelib::HALFPI(ctx) - ((J as f64) * save.DLAT));

            for K in 1..=save.NRAD {
                save.R = f64::powi(10.0, (K - 2));

                for L in 1..=save.NSCALE {
                    save.SCALE = f64::powf(10.0, (20.0 * ((L - (save.NSCALE / 2)) as f64)));

                    // SCALE = 1.D0

                    //
                    // Create the vertex.
                    //
                    spicelib::LATREC(save.R, save.LON, save.LAT, save.VERTEX.as_slice_mut());

                    //
                    // Scale the vertex.
                    //
                    spicelib::VSCL(
                        save.SCALE,
                        save.VERTEX.as_slice(),
                        save.SCLVTX.as_slice_mut(),
                    );

                    //
                    // Loop over the shapes.
                    //
                    for M in 1..=3 {
                        for N in 1..=3 {
                            spicelib::VSCL(
                                save.SCALE,
                                save.PLATES.subarray([1, N, M]),
                                save.SCLPLT.subarray_mut([1, N, M]),
                            );
                        }

                        //
                        // Transform vertex and plate using XFORM.
                        //
                        spicelib::MXV(
                            save.XFORM.as_slice(),
                            save.VERTEX.as_slice(),
                            save.VTEMP.as_slice_mut(),
                        );
                        spicelib::VEQU(save.VTEMP.as_slice(), save.VERTEX.as_slice_mut());

                        for N in 1..=3 {
                            spicelib::MXV(
                                save.XFORM.as_slice(),
                                save.PLATES.subarray([1, N, M]),
                                save.VTEMP.as_slice_mut(),
                            );
                            spicelib::VEQU(
                                save.VTEMP.as_slice(),
                                save.PLATES.subarray_mut([1, N, M]),
                            );
                        }

                        //
                        // Create the edges of the pyramid for the current
                        // plate.
                        //
                        spicelib::VSUB(
                            save.SCLPLT.subarray([1, 1, M]),
                            save.SCLVTX.as_slice(),
                            save.E1.as_slice_mut(),
                        );
                        spicelib::VSUB(
                            save.SCLPLT.subarray([1, 2, M]),
                            save.SCLVTX.as_slice(),
                            save.E2.as_slice_mut(),
                        );
                        spicelib::VSUB(
                            save.SCLPLT.subarray([1, 3, M]),
                            save.SCLVTX.as_slice(),
                            save.E3.as_slice_mut(),
                        );

                        //
                        // --- Case: ------------------------------------------------------
                        //
                        fstr::assign(
                            &mut save.TITLE,
                            b"I = #; J = #, K = #, L = #, M = #. Aim at plate\'s centroid.",
                        );
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", K, &mut save.TITLE, ctx);
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", L, &mut save.TITLE, ctx);
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", M, &mut save.TITLE, ctx);
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        testutil::TCASE(&save.TITLE, ctx)?;

                        spicelib::VLCOM3(
                            1.0,
                            save.E1.as_slice(),
                            1.0,
                            save.E2.as_slice(),
                            1.0,
                            save.E3.as_slice(),
                            save.V.as_slice_mut(),
                        );

                        spicelib::INSANG(
                            save.V.as_slice(),
                            save.E1.as_slice(),
                            save.E2.as_slice(),
                            save.E3.as_slice(),
                            &mut save.FOUND,
                            &mut save.S,
                        );

                        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

                        spicelib::VSCL(save.S, save.V.as_slice(), save.XPT.as_slice_mut());

                        //
                        // Create a SPICELIB plane containing the plate
                        // defined by E1, E2, E3.
                        //
                        spicelib::VSUB(
                            save.E2.as_slice(),
                            save.E1.as_slice(),
                            save.SIDES.subarray_mut([1, 1]),
                        );
                        spicelib::VSUB(
                            save.E3.as_slice(),
                            save.E1.as_slice(),
                            save.SIDES.subarray_mut([1, 3]),
                        );

                        spicelib::PSV2PL(
                            save.E1.as_slice(),
                            save.SIDES.subarray([1, 1]),
                            save.SIDES.subarray([1, 3]),
                            save.PLANE.as_slice_mut(),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        //
                        // Find the expected intercept.
                        //
                        spicelib::INRYPL(
                            save.ORIGIN.as_slice(),
                            save.V.as_slice(),
                            save.PLANE.as_slice(),
                            &mut save.NXPTS,
                            save.XXPT.as_slice_mut(),
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
                        fstr::assign(&mut save.TITLE, b"I = #; J = #, K = #, L = #, M = #. Aim at interior point close to the E1-E2 side.");
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", K, &mut save.TITLE, ctx);
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", L, &mut save.TITLE, ctx);
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", M, &mut save.TITLE, ctx);
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        testutil::TCASE(&save.TITLE, ctx)?;

                        spicelib::VLCOM3(
                            0.5,
                            save.E1.as_slice(),
                            0.5,
                            save.E2.as_slice(),
                            0.001,
                            save.E3.as_slice(),
                            save.V.as_slice_mut(),
                        );

                        spicelib::INSANG(
                            save.V.as_slice(),
                            save.E1.as_slice(),
                            save.E2.as_slice(),
                            save.E3.as_slice(),
                            &mut save.FOUND,
                            &mut save.S,
                        );

                        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

                        spicelib::VSCL(save.S, save.V.as_slice(), save.XPT.as_slice_mut());

                        //
                        // Create a SPICELIB plane containing the plate
                        // defined by E1, E2, E3.
                        //
                        spicelib::VSUB(
                            save.E2.as_slice(),
                            save.E1.as_slice(),
                            save.SIDES.subarray_mut([1, 1]),
                        );
                        spicelib::VSUB(
                            save.E3.as_slice(),
                            save.E1.as_slice(),
                            save.SIDES.subarray_mut([1, 3]),
                        );

                        spicelib::PSV2PL(
                            save.E1.as_slice(),
                            save.SIDES.subarray([1, 1]),
                            save.SIDES.subarray([1, 3]),
                            save.PLANE.as_slice_mut(),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        //
                        // Find the expected intercept.
                        //
                        spicelib::INRYPL(
                            save.ORIGIN.as_slice(),
                            save.V.as_slice(),
                            save.PLANE.as_slice(),
                            &mut save.NXPTS,
                            save.XXPT.as_slice_mut(),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

                        if !*OK {
                            ctx.stop()?;
                        }

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
                        fstr::assign(&mut save.TITLE, b"I = #; J = #, K = #, L = #, M = #. Aim at exterior point close to the E1-E2 side.");
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", K, &mut save.TITLE, ctx);
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", L, &mut save.TITLE, ctx);
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", M, &mut save.TITLE, ctx);
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        testutil::TCASE(&save.TITLE, ctx)?;

                        spicelib::VLCOM3(
                            0.5,
                            save.E1.as_slice(),
                            0.5,
                            save.E2.as_slice(),
                            -0.001,
                            save.E3.as_slice(),
                            save.V.as_slice_mut(),
                        );

                        spicelib::INSANG(
                            save.V.as_slice(),
                            save.E1.as_slice(),
                            save.E2.as_slice(),
                            save.E3.as_slice(),
                            &mut save.FOUND,
                            &mut save.S,
                        );

                        testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

                        //
                        // --- Case: ------------------------------------------------------
                        //
                        fstr::assign(&mut save.TITLE, b"I = #; J = #, K = #, L = #, M = #. Aim at interior point close to the E1-E3 side.");
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", K, &mut save.TITLE, ctx);
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", L, &mut save.TITLE, ctx);
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", M, &mut save.TITLE, ctx);
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        testutil::TCASE(&save.TITLE, ctx)?;

                        spicelib::VLCOM3(
                            0.5,
                            save.E1.as_slice(),
                            0.5,
                            save.E3.as_slice(),
                            0.001,
                            save.E2.as_slice(),
                            save.V.as_slice_mut(),
                        );

                        spicelib::INSANG(
                            save.V.as_slice(),
                            save.E1.as_slice(),
                            save.E2.as_slice(),
                            save.E3.as_slice(),
                            &mut save.FOUND,
                            &mut save.S,
                        );

                        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

                        spicelib::VSCL(save.S, save.V.as_slice(), save.XPT.as_slice_mut());

                        //
                        // Create a SPICELIB plane containing the plate
                        // defined by E1, E2, E3.
                        //
                        spicelib::VSUB(
                            save.E2.as_slice(),
                            save.E1.as_slice(),
                            save.SIDES.subarray_mut([1, 1]),
                        );
                        spicelib::VSUB(
                            save.E3.as_slice(),
                            save.E1.as_slice(),
                            save.SIDES.subarray_mut([1, 3]),
                        );

                        spicelib::PSV2PL(
                            save.E1.as_slice(),
                            save.SIDES.subarray([1, 1]),
                            save.SIDES.subarray([1, 3]),
                            save.PLANE.as_slice_mut(),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        //
                        // Find the expected intercept.
                        //
                        spicelib::INRYPL(
                            save.ORIGIN.as_slice(),
                            save.V.as_slice(),
                            save.PLANE.as_slice(),
                            &mut save.NXPTS,
                            save.XXPT.as_slice_mut(),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

                        if !*OK {
                            ctx.stop()?;
                        }

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
                        fstr::assign(&mut save.TITLE, b"I = #; J = #, K = #, L = #, M = #. Aim at exterior point close to the E1-E3 side.");
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", K, &mut save.TITLE, ctx);
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", L, &mut save.TITLE, ctx);
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", M, &mut save.TITLE, ctx);
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        testutil::TCASE(&save.TITLE, ctx)?;

                        spicelib::VLCOM3(
                            0.5,
                            save.E1.as_slice(),
                            0.5,
                            save.E3.as_slice(),
                            -0.001,
                            save.E2.as_slice(),
                            save.V.as_slice_mut(),
                        );

                        spicelib::INSANG(
                            save.V.as_slice(),
                            save.E1.as_slice(),
                            save.E2.as_slice(),
                            save.E3.as_slice(),
                            &mut save.FOUND,
                            &mut save.S,
                        );

                        testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

                        //
                        // --- Case: ------------------------------------------------------
                        //
                        fstr::assign(&mut save.TITLE, b"I = #; J = #, K = #, L = #, M = #. Aim at interior point close to the E2-E3 side.");
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", K, &mut save.TITLE, ctx);
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", L, &mut save.TITLE, ctx);
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", M, &mut save.TITLE, ctx);
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        testutil::TCASE(&save.TITLE, ctx)?;

                        spicelib::VLCOM3(
                            0.5,
                            save.E2.as_slice(),
                            0.5,
                            save.E3.as_slice(),
                            0.001,
                            save.E1.as_slice(),
                            save.V.as_slice_mut(),
                        );

                        spicelib::INSANG(
                            save.V.as_slice(),
                            save.E1.as_slice(),
                            save.E2.as_slice(),
                            save.E3.as_slice(),
                            &mut save.FOUND,
                            &mut save.S,
                        );

                        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

                        spicelib::VSCL(save.S, save.V.as_slice(), save.XPT.as_slice_mut());

                        //
                        // Create a SPICELIB plane containing the plate
                        // defined by E1, E2, E3.
                        //
                        spicelib::VSUB(
                            save.E2.as_slice(),
                            save.E1.as_slice(),
                            save.SIDES.subarray_mut([1, 1]),
                        );
                        spicelib::VSUB(
                            save.E3.as_slice(),
                            save.E1.as_slice(),
                            save.SIDES.subarray_mut([1, 3]),
                        );

                        spicelib::PSV2PL(
                            save.E1.as_slice(),
                            save.SIDES.subarray([1, 1]),
                            save.SIDES.subarray([1, 3]),
                            save.PLANE.as_slice_mut(),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        //
                        // Find the expected intercept.
                        //
                        spicelib::INRYPL(
                            save.ORIGIN.as_slice(),
                            save.V.as_slice(),
                            save.PLANE.as_slice(),
                            &mut save.NXPTS,
                            save.XXPT.as_slice_mut(),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

                        if !*OK {
                            ctx.stop()?;
                        }

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
                        fstr::assign(&mut save.TITLE, b"I = #; J = #, K = #, L = #, M = #. Aim at exterior point close to the E2-E3 side.");
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", K, &mut save.TITLE, ctx);
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", L, &mut save.TITLE, ctx);
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", M, &mut save.TITLE, ctx);
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        testutil::TCASE(&save.TITLE, ctx)?;

                        spicelib::VLCOM3(
                            0.5,
                            save.E2.as_slice(),
                            0.5,
                            save.E3.as_slice(),
                            -0.001,
                            save.E1.as_slice(),
                            save.V.as_slice_mut(),
                        );

                        spicelib::INSANG(
                            save.V.as_slice(),
                            save.E1.as_slice(),
                            save.E2.as_slice(),
                            save.E3.as_slice(),
                            &mut save.FOUND,
                            &mut save.S,
                        );

                        testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;
                    }
                }
            }
        }
    }

    //**********************************************************************
    //
    //     Exceptional cases
    //
    //**********************************************************************

    //
    // INSANG is error-free.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Ray direction is zero vector.", ctx)?;

    spicelib::VPACK(0.0, 0.0, 10.0, save.E1.as_slice_mut());
    spicelib::VPACK(4.0, 0.0, 10.0, save.E2.as_slice_mut());
    spicelib::VPACK(0.0, 4.0, 10.0, save.E3.as_slice_mut());
    spicelib::CLEARD(3, save.V.as_slice_mut());

    spicelib::INSANG(
        save.V.as_slice(),
        save.E1.as_slice(),
        save.E2.as_slice(),
        save.E3.as_slice(),
        &mut save.FOUND,
        &mut save.SCALE,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Edge vectors are linearly dependent.", ctx)?;

    //
    // Create independent edge vectors, but pass dependent
    // selections of the them to INSANG.
    //
    spicelib::VPACK(0.0, 0.0, 10.0, save.E1.as_slice_mut());
    spicelib::VPACK(4.0, 0.0, 10.0, save.E2.as_slice_mut());
    spicelib::VPACK(0.0, 4.0, 10.0, save.E3.as_slice_mut());
    spicelib::VPACK(1.0, 1.0, 10.0, save.V.as_slice_mut());

    //
    // E2 = E1:
    //
    spicelib::INSANG(
        save.V.as_slice(),
        save.E1.as_slice(),
        save.E1.as_slice(),
        save.E3.as_slice(),
        &mut save.FOUND,
        &mut save.SCALE,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;
    testutil::CHCKSD(b"SCALE", save.SCALE, b"=", 0.0, 0.0, OK, ctx)?;

    //
    // E3 = E1:
    //
    spicelib::INSANG(
        save.V.as_slice(),
        save.E1.as_slice(),
        save.E2.as_slice(),
        save.E1.as_slice(),
        &mut save.FOUND,
        &mut save.SCALE,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;
    testutil::CHCKSD(b"SCALE", save.SCALE, b"=", 0.0, 0.0, OK, ctx)?;

    //
    // E3 = E2:
    //
    spicelib::INSANG(
        save.V.as_slice(),
        save.E1.as_slice(),
        save.E2.as_slice(),
        save.E2.as_slice(),
        &mut save.FOUND,
        &mut save.SCALE,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;
    testutil::CHCKSD(b"SCALE", save.SCALE, b"=", 0.0, 0.0, OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
