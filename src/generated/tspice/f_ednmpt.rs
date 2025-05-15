//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const VTIGHT: f64 = 0.00000000000001;
const TITLEN: i32 = 160;

struct SaveVars {
    TITLE: Vec<u8>,
    A: f64,
    B: f64,
    C: f64,
    DLAT: f64,
    DLON: f64,
    LAT: f64,
    LON: f64,
    NORMAL: StackArray<f64, 3>,
    P: StackArray<f64, 3>,
    R: f64,
    TOL: f64,
    XNORML: StackArray<f64, 3>,
    NSTEP: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TITLE = vec![b' '; TITLEN as usize];
        let mut A: f64 = 0.0;
        let mut B: f64 = 0.0;
        let mut C: f64 = 0.0;
        let mut DLAT: f64 = 0.0;
        let mut DLON: f64 = 0.0;
        let mut LAT: f64 = 0.0;
        let mut LON: f64 = 0.0;
        let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
        let mut P = StackArray::<f64, 3>::new(1..=3);
        let mut R: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut XNORML = StackArray::<f64, 3>::new(1..=3);
        let mut NSTEP: i32 = 0;

        Self {
            TITLE,
            A,
            B,
            C,
            DLAT,
            DLON,
            LAT,
            LON,
            NORMAL,
            P,
            R,
            TOL,
            XNORML,
            NSTEP,
        }
    }
}

//$Procedure F_EDNMPT ( EDNMPT tests )
pub fn F_EDNMPT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    testutil::TOPEN(b"F_EDNMPT", ctx)?;

    //***********************************************************************
    //
    //     Normal cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Sphere tests", ctx)?;

    save.R = 7.0;
    save.A = save.R;
    save.B = save.R;
    save.C = save.R;

    save.NSTEP = 10;
    save.DLON = (((2 as f64) * spicelib::PI(ctx)) / save.NSTEP as f64);
    save.DLAT = (spicelib::PI(ctx) / save.NSTEP as f64);

    for I in 1..=save.NSTEP {
        save.LON = ((I as f64) * save.DLON);

        for J in 0..=save.NSTEP {
            save.LAT = ((spicelib::PI(ctx) / 2 as f64) - ((J as f64) * save.DLAT));

            spicelib::LATREC(1.0, save.LON, save.LAT, save.XNORML.as_slice_mut());

            //
            // --- Case: ------------------------------------------------------
            //
            fstr::assign(&mut save.TITLE, b"Sphere: normal = (#,#,#)");
            spicelib::REPMD(
                &save.TITLE.to_vec(),
                b"#",
                save.XNORML[1],
                14,
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMD(
                &save.TITLE.to_vec(),
                b"#",
                save.XNORML[2],
                14,
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMD(
                &save.TITLE.to_vec(),
                b"#",
                save.XNORML[3],
                14,
                &mut save.TITLE,
                ctx,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::TCASE(&save.TITLE, ctx)?;

            spicelib::EDNMPT(
                save.A,
                save.B,
                save.C,
                save.XNORML.as_slice(),
                save.P.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // Check that we can recover the input normal
            // from P.
            //
            spicelib::SURFNM(
                save.A,
                save.B,
                save.C,
                save.P.as_slice(),
                save.NORMAL.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = VTIGHT;

            testutil::CHCKAD(
                b"NORMAL",
                save.NORMAL.as_slice(),
                b"~~/",
                save.XNORML.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Triaxial ellipsoid tests", ctx)?;

    save.A = 100.0;
    save.B = 50.0;
    save.C = 3.0;

    save.NSTEP = 10;
    save.DLON = (((2 as f64) * spicelib::PI(ctx)) / save.NSTEP as f64);
    save.DLAT = (spicelib::PI(ctx) / save.NSTEP as f64);

    for I in 1..=save.NSTEP {
        save.LON = ((I as f64) * save.DLON);

        for J in 0..=save.NSTEP {
            save.LAT = ((spicelib::PI(ctx) / 2 as f64) - ((J as f64) * save.DLAT));

            spicelib::LATREC(1.0, save.LON, save.LAT, save.XNORML.as_slice_mut());

            //
            // --- Case: ------------------------------------------------------
            //
            fstr::assign(&mut save.TITLE, b"Ellipsoid: normal = (#,#,#)");
            spicelib::REPMD(
                &save.TITLE.to_vec(),
                b"#",
                save.XNORML[1],
                14,
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMD(
                &save.TITLE.to_vec(),
                b"#",
                save.XNORML[2],
                14,
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMD(
                &save.TITLE.to_vec(),
                b"#",
                save.XNORML[3],
                14,
                &mut save.TITLE,
                ctx,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::TCASE(&save.TITLE, ctx)?;

            spicelib::EDNMPT(
                save.A,
                save.B,
                save.C,
                save.XNORML.as_slice(),
                save.P.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // Check that we can recover the input normal
            // from P.
            //
            spicelib::SURFNM(
                save.A,
                save.B,
                save.C,
                save.P.as_slice(),
                save.NORMAL.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = VTIGHT;

            testutil::CHCKAD(
                b"NORMAL",
                save.NORMAL.as_slice(),
                b"~~/",
                save.XNORML.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;
        }
    }

    //*********************************************************************
    //*
    //*    Error cases
    //*
    //*********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid semi-axis lengths.", ctx)?;

    save.A = 100.0;
    save.B = 50.0;
    save.C = 3.0;

    spicelib::VPACK(0.0, 0.0, 1.0, save.XNORML.as_slice_mut());

    spicelib::EDNMPT(
        -save.A,
        save.B,
        save.C,
        save.XNORML.as_slice(),
        save.P.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    spicelib::EDNMPT(
        save.A,
        -save.B,
        save.C,
        save.XNORML.as_slice(),
        save.P.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    spicelib::EDNMPT(
        save.A,
        save.B,
        -save.C,
        save.XNORML.as_slice(),
        save.P.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    spicelib::EDNMPT(
        0.0,
        save.B,
        save.C,
        save.XNORML.as_slice(),
        save.P.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    spicelib::EDNMPT(
        save.A,
        0.0,
        save.C,
        save.XNORML.as_slice(),
        save.P.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    spicelib::EDNMPT(
        save.A,
        save.B,
        0.0,
        save.XNORML.as_slice(),
        save.P.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid semi-axis length after scaling.", ctx)?;

    save.A = 1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;
    save.B = 0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001;
    save.C = 3.0;

    spicelib::VPACK(0.0, 0.0, 1.0, save.XNORML.as_slice_mut());

    spicelib::EDNMPT(
        save.A,
        save.B,
        save.C,
        save.XNORML.as_slice(),
        save.P.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(AXISUNDERFLOW)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Zero normal vector.", ctx)?;

    save.A = 1.0;
    save.B = save.A;
    save.C = save.A;

    spicelib::CLEARD(3, save.XNORML.as_slice_mut());

    spicelib::EDNMPT(
        save.A,
        save.B,
        save.C,
        save.XNORML.as_slice(),
        save.P.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ZEROVECTOR)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // The degenerate case error resulting from a non-positive
    // reciprocal of the square of lambda should never occur.
    // A disposable version of the code should be used to
    // exercise the error handling for this case.
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
