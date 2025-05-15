//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const VTIGHT: f64 = 0.00000000000001;
const TITLEN: i32 = 160;

struct SaveVars {
    TITLE: Vec<u8>,
    DIR: StackArray<f64, 3>,
    DIST: f64,
    DSCALE: f64,
    EP1: StackArray<f64, 3>,
    EP2: StackArray<f64, 3>,
    MAXSCL: f64,
    MINSCL: f64,
    P: StackArray<f64, 3>,
    PERP: StackArray<f64, 3>,
    PNEAR: StackArray<f64, 3>,
    SCALE: f64,
    SMALL: f64,
    TOL: f64,
    UDIR: StackArray<f64, 3>,
    X: StackArray<f64, 3>,
    XDIST: f64,
    XPNEAR: StackArray<f64, 3>,
    Y: StackArray<f64, 3>,
    Z: StackArray<f64, 3>,
    NSCALE: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TITLE = vec![b' '; TITLEN as usize];
        let mut DIR = StackArray::<f64, 3>::new(1..=3);
        let mut DIST: f64 = 0.0;
        let mut DSCALE: f64 = 0.0;
        let mut EP1 = StackArray::<f64, 3>::new(1..=3);
        let mut EP2 = StackArray::<f64, 3>::new(1..=3);
        let mut MAXSCL: f64 = 0.0;
        let mut MINSCL: f64 = 0.0;
        let mut P = StackArray::<f64, 3>::new(1..=3);
        let mut PERP = StackArray::<f64, 3>::new(1..=3);
        let mut PNEAR = StackArray::<f64, 3>::new(1..=3);
        let mut SCALE: f64 = 0.0;
        let mut SMALL: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut UDIR = StackArray::<f64, 3>::new(1..=3);
        let mut X = StackArray::<f64, 3>::new(1..=3);
        let mut XDIST: f64 = 0.0;
        let mut XPNEAR = StackArray::<f64, 3>::new(1..=3);
        let mut Y = StackArray::<f64, 3>::new(1..=3);
        let mut Z = StackArray::<f64, 3>::new(1..=3);
        let mut NSCALE: i32 = 0;

        Self {
            TITLE,
            DIR,
            DIST,
            DSCALE,
            EP1,
            EP2,
            MAXSCL,
            MINSCL,
            P,
            PERP,
            PNEAR,
            SCALE,
            SMALL,
            TOL,
            UDIR,
            X,
            XDIST,
            XPNEAR,
            Y,
            Z,
            NSCALE,
        }
    }
}

//$Procedure F_NPSGPT ( NPSGPT tests )
pub fn F_NPSGPT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    testutil::TOPEN(b"F_NPSGPT", ctx)?;

    //***********************************************************************
    //
    //     Normal cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Segment lies on X-axis between X = 1 and X = 2. Point has negative X.",
        ctx,
    )?;

    spicelib::VPACK(1.0, 0.0, 0.0, save.EP1.as_slice_mut());
    spicelib::VPACK(2.0, 0.0, 0.0, save.EP2.as_slice_mut());

    spicelib::VPACK(-1.0, 0.0, 0.0, save.P.as_slice_mut());

    spicelib::VEQU(save.EP1.as_slice(), save.XPNEAR.as_slice_mut());

    save.XDIST = 2.0;

    spicelib::NPSGPT(
        save.EP1.as_slice(),
        save.EP2.as_slice(),
        save.P.as_slice(),
        save.PNEAR.as_slice_mut(),
        &mut save.DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"PNEAR",
        save.PNEAR.as_slice(),
        b"~~/",
        save.XPNEAR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"DIST", save.DIST, b"~", save.XDIST, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Segment lies on X-axis between X = 1 and X = 2. Point has X = 1.",
        ctx,
    )?;

    spicelib::VPACK(1.0, 0.0, 0.0, save.EP1.as_slice_mut());
    spicelib::VPACK(2.0, 0.0, 0.0, save.EP2.as_slice_mut());

    spicelib::VPACK(1.0, 0.0, 0.0, save.P.as_slice_mut());

    spicelib::VEQU(save.EP1.as_slice(), save.XPNEAR.as_slice_mut());

    save.XDIST = 0.0;

    spicelib::NPSGPT(
        save.EP1.as_slice(),
        save.EP2.as_slice(),
        save.P.as_slice(),
        save.PNEAR.as_slice_mut(),
        &mut save.DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"PNEAR",
        save.PNEAR.as_slice(),
        b"~~/",
        save.XPNEAR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"DIST", save.DIST, b"~", save.XDIST, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Segment lies on X-axis between X = 1 and X = 2. Point has X = 2.",
        ctx,
    )?;

    spicelib::VPACK(1.0, 0.0, 0.0, save.EP1.as_slice_mut());
    spicelib::VPACK(2.0, 0.0, 0.0, save.EP2.as_slice_mut());

    spicelib::VPACK(2.0, 0.0, 0.0, save.P.as_slice_mut());

    spicelib::VEQU(save.EP2.as_slice(), save.XPNEAR.as_slice_mut());

    save.XDIST = 0.0;

    spicelib::NPSGPT(
        save.EP1.as_slice(),
        save.EP2.as_slice(),
        save.P.as_slice(),
        save.PNEAR.as_slice_mut(),
        &mut save.DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"PNEAR",
        save.PNEAR.as_slice(),
        b"~~/",
        save.XPNEAR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"DIST", save.DIST, b"~", save.XDIST, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Segment lies on X-axis between X = 1 and X = 2. Point has X > 2.",
        ctx,
    )?;

    spicelib::VPACK(1.0, 0.0, 0.0, save.EP1.as_slice_mut());
    spicelib::VPACK(2.0, 0.0, 0.0, save.EP2.as_slice_mut());

    spicelib::VPACK(3.0, 0.0, 0.0, save.P.as_slice_mut());

    spicelib::VEQU(save.EP2.as_slice(), save.XPNEAR.as_slice_mut());

    save.XDIST = 1.0;

    spicelib::NPSGPT(
        save.EP1.as_slice(),
        save.EP2.as_slice(),
        save.P.as_slice(),
        save.PNEAR.as_slice_mut(),
        &mut save.DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"PNEAR",
        save.PNEAR.as_slice(),
        b"~~/",
        save.XPNEAR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"DIST", save.DIST, b"~", save.XDIST, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Now try some more general cases.
    //
    save.MINSCL = -100.0;
    save.MAXSCL = 100.0;
    save.NSCALE = 30;
    save.DSCALE = ((save.MAXSCL - save.MINSCL) / save.NSCALE as f64);

    for I in 0..=save.NSCALE {
        //
        // Generate endpoints of the segment.
        //
        save.SCALE = f64::powf(10.0, (save.MINSCL + ((I as f64) * save.DSCALE)));

        spicelib::VPACK(1.0, 3.0, -7.0, save.EP1.as_slice_mut());
        spicelib::VPACK(5.0, -13.0, -0.5, save.EP2.as_slice_mut());

        spicelib::VSCLIP(save.SCALE, save.EP1.as_slice_mut());
        spicelib::VSCLIP(save.SCALE, save.EP2.as_slice_mut());

        //
        // --- Case: ------------------------------------------------------
        //
        fstr::assign(&mut save.TITLE, b"Point projection is off EP1 end; I = #.");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::TCASE(&save.TITLE, ctx)?;

        //
        // Let DIR be the vector from EP1 to EP2.
        //
        spicelib::VSUB(
            save.EP2.as_slice(),
            save.EP1.as_slice(),
            save.DIR.as_slice_mut(),
        );
        spicelib::VHAT(save.DIR.as_slice(), save.UDIR.as_slice_mut());
        spicelib::VEQU(save.UDIR.as_slice(), save.X.as_slice_mut());

        //
        // Get some vectors orthogonal to DIR; generate
        // a perpendicular component vector for P.
        //
        spicelib::FRAME(
            save.X.as_slice_mut(),
            save.Y.as_slice_mut(),
            save.Z.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::VLCOM(
            (2.0 * save.SCALE),
            save.Y.as_slice(),
            (5.0 * save.SCALE),
            save.Z.as_slice(),
            save.PERP.as_slice_mut(),
        );

        //
        // Generate an input point.
        //
        save.SMALL = 0.0000000000001;

        spicelib::VLCOM3(
            1.0,
            save.EP1.as_slice(),
            -save.SMALL,
            save.DIR.as_slice(),
            1.0,
            save.PERP.as_slice(),
            save.P.as_slice_mut(),
        );

        save.TOL = VTIGHT;

        spicelib::VEQU(save.EP1.as_slice(), save.XPNEAR.as_slice_mut());

        save.XDIST = spicelib::VDIST(save.P.as_slice(), save.EP1.as_slice());

        spicelib::NPSGPT(
            save.EP1.as_slice(),
            save.EP2.as_slice(),
            save.P.as_slice(),
            save.PNEAR.as_slice_mut(),
            &mut save.DIST,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"PNEAR",
            save.PNEAR.as_slice(),
            b"~~/",
            save.XPNEAR.as_slice(),
            3,
            save.TOL,
            OK,
            ctx,
        )?;
        testutil::CHCKSD(b"DIST", save.DIST, b"~/", save.XDIST, save.TOL, OK, ctx)?;

        //
        // --- Case: ------------------------------------------------------
        //
        fstr::assign(&mut save.TITLE, b"Point projection is EP1; I = #.");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::TCASE(&save.TITLE, ctx)?;

        //
        // Generate an input point.
        //
        spicelib::VLCOM3(
            1.0,
            save.EP1.as_slice(),
            0.0,
            save.DIR.as_slice(),
            1.0,
            save.PERP.as_slice(),
            save.P.as_slice_mut(),
        );

        save.TOL = VTIGHT;

        spicelib::VEQU(save.EP1.as_slice(), save.XPNEAR.as_slice_mut());

        save.XDIST = spicelib::VDIST(save.P.as_slice(), save.EP1.as_slice());

        spicelib::NPSGPT(
            save.EP1.as_slice(),
            save.EP2.as_slice(),
            save.P.as_slice(),
            save.PNEAR.as_slice_mut(),
            &mut save.DIST,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"PNEAR",
            save.PNEAR.as_slice(),
            b"~~/",
            save.XPNEAR.as_slice(),
            3,
            save.TOL,
            OK,
            ctx,
        )?;
        testutil::CHCKSD(b"DIST", save.DIST, b"~/", save.XDIST, save.TOL, OK, ctx)?;

        //
        // --- Case: ------------------------------------------------------
        //
        fstr::assign(
            &mut save.TITLE,
            b"Point projection is between EP1 and EP2; I = #.",
        );
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::TCASE(&save.TITLE, ctx)?;

        //
        // Generate an input point.
        //
        spicelib::VLCOM3(
            0.5,
            save.EP1.as_slice(),
            0.5,
            save.EP2.as_slice(),
            1.0,
            save.PERP.as_slice(),
            save.P.as_slice_mut(),
        );

        save.TOL = VTIGHT;

        spicelib::VLCOM(
            0.5,
            save.EP1.as_slice(),
            0.5,
            save.EP2.as_slice(),
            save.XPNEAR.as_slice_mut(),
        );

        save.XDIST = spicelib::VNORM(save.PERP.as_slice());

        spicelib::NPSGPT(
            save.EP1.as_slice(),
            save.EP2.as_slice(),
            save.P.as_slice(),
            save.PNEAR.as_slice_mut(),
            &mut save.DIST,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"PNEAR",
            save.PNEAR.as_slice(),
            b"~~/",
            save.XPNEAR.as_slice(),
            3,
            save.TOL,
            OK,
            ctx,
        )?;
        testutil::CHCKSD(b"DIST", save.DIST, b"~/", save.XDIST, save.TOL, OK, ctx)?;

        //
        // --- Case: ------------------------------------------------------
        //
        fstr::assign(&mut save.TITLE, b"Point projection is EP2; I = #.");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::TCASE(&save.TITLE, ctx)?;

        //
        // Generate an input point.
        //
        spicelib::VLCOM3(
            1.0,
            save.EP2.as_slice(),
            0.0,
            save.DIR.as_slice(),
            1.0,
            save.PERP.as_slice(),
            save.P.as_slice_mut(),
        );

        save.TOL = VTIGHT;

        spicelib::VEQU(save.EP2.as_slice(), save.XPNEAR.as_slice_mut());

        save.XDIST = spicelib::VDIST(save.P.as_slice(), save.EP2.as_slice());

        spicelib::NPSGPT(
            save.EP1.as_slice(),
            save.EP2.as_slice(),
            save.P.as_slice(),
            save.PNEAR.as_slice_mut(),
            &mut save.DIST,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"PNEAR",
            save.PNEAR.as_slice(),
            b"~~/",
            save.XPNEAR.as_slice(),
            3,
            save.TOL,
            OK,
            ctx,
        )?;
        testutil::CHCKSD(b"DIST", save.DIST, b"~/", save.XDIST, save.TOL, OK, ctx)?;

        //
        // --- Case: ------------------------------------------------------
        //
        fstr::assign(&mut save.TITLE, b"Point projection is off EP2 end; I = #.");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::TCASE(&save.TITLE, ctx)?;

        //
        // Generate an input point.
        //
        save.SMALL = 0.0000000000001;

        spicelib::VLCOM3(
            1.0,
            save.EP2.as_slice(),
            save.SMALL,
            save.DIR.as_slice(),
            1.0,
            save.PERP.as_slice(),
            save.P.as_slice_mut(),
        );

        save.TOL = VTIGHT;

        spicelib::VEQU(save.EP2.as_slice(), save.XPNEAR.as_slice_mut());

        save.XDIST = spicelib::VDIST(save.P.as_slice(), save.EP2.as_slice());

        spicelib::NPSGPT(
            save.EP1.as_slice(),
            save.EP2.as_slice(),
            save.P.as_slice(),
            save.PNEAR.as_slice_mut(),
            &mut save.DIST,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"PNEAR",
            save.PNEAR.as_slice(),
            b"~~/",
            save.XPNEAR.as_slice(),
            3,
            save.TOL,
            OK,
            ctx,
        )?;
        testutil::CHCKSD(b"DIST", save.DIST, b"~/", save.XDIST, save.TOL, OK, ctx)?;
    }

    //*********************************************************************
    //*
    //*    Non-error exceptional case
    //*
    //*********************************************************************

    testutil::TCASE(b"EP1 == EP2", ctx)?;

    spicelib::VLCOM(
        1.0,
        save.EP2.as_slice(),
        1.0,
        save.DIR.as_slice(),
        save.P.as_slice_mut(),
    );

    spicelib::NPSGPT(
        save.EP1.as_slice(),
        save.EP1.as_slice(),
        save.P.as_slice(),
        save.PNEAR.as_slice_mut(),
        &mut save.DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.EP1.as_slice(), save.XPNEAR.as_slice_mut());

    save.XDIST = spicelib::VDIST(save.P.as_slice(), save.EP1.as_slice());

    testutil::CHCKAD(
        b"PNEAR",
        save.PNEAR.as_slice(),
        b"~~/",
        save.XPNEAR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"DIST", save.DIST, b"~/", save.XDIST, save.TOL, OK, ctx)?;

    //*********************************************************************
    //*
    //*    Error cases
    //*
    //*********************************************************************

    //
    // None.
    //

    //
    // ---------------------------------------------------------
    //

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
