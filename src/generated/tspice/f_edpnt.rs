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
    DSCALE: f64,
    LAT: f64,
    LON: f64,
    MAXSCL: f64,
    MINSCL: f64,
    ORIGIN: StackArray<f64, 3>,
    P: StackArray<f64, 3>,
    SCALE: f64,
    SP: StackArray<f64, 3>,
    TOL: f64,
    XPT: StackArray<f64, 3>,
    NSTEP: i32,
    NSCALE: i32,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TITLE = vec![b' '; TITLEN as usize];
        let mut A: f64 = 0.0;
        let mut B: f64 = 0.0;
        let mut C: f64 = 0.0;
        let mut DLAT: f64 = 0.0;
        let mut DLON: f64 = 0.0;
        let mut DSCALE: f64 = 0.0;
        let mut LAT: f64 = 0.0;
        let mut LON: f64 = 0.0;
        let mut MAXSCL: f64 = 0.0;
        let mut MINSCL: f64 = 0.0;
        let mut ORIGIN = StackArray::<f64, 3>::new(1..=3);
        let mut P = StackArray::<f64, 3>::new(1..=3);
        let mut SCALE: f64 = 0.0;
        let mut SP = StackArray::<f64, 3>::new(1..=3);
        let mut TOL: f64 = 0.0;
        let mut XPT = StackArray::<f64, 3>::new(1..=3);
        let mut NSTEP: i32 = 0;
        let mut NSCALE: i32 = 0;
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

        Self {
            TITLE,
            A,
            B,
            C,
            DLAT,
            DLON,
            DSCALE,
            LAT,
            LON,
            MAXSCL,
            MINSCL,
            ORIGIN,
            P,
            SCALE,
            SP,
            TOL,
            XPT,
            NSTEP,
            NSCALE,
            FOUND,
        }
    }
}

//$Procedure F_EDPNT ( EDPNT tests )
pub fn F_EDPNT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    testutil::TOPEN(b"F_EDPNT", ctx)?;

    //***********************************************************************
    //
    //     Normal cases
    //
    //***********************************************************************

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

    save.NSCALE = 20;
    save.MINSCL = -100.0;
    save.MAXSCL = 100.0;
    save.DSCALE = ((save.MAXSCL - save.MINSCL) / save.NSCALE as f64);

    for I in 1..=save.NSTEP {
        save.LON = ((I as f64) * save.DLON);

        for J in 0..=save.NSTEP {
            save.LAT = ((spicelib::PI(ctx) / 2 as f64) - ((J as f64) * save.DLAT));

            spicelib::LATREC(1.0, save.LON, save.LAT, save.P.as_slice_mut());

            spicelib::SURFPT(
                save.ORIGIN.as_slice(),
                save.P.as_slice(),
                save.A,
                save.B,
                save.C,
                save.XPT.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // We expect that a result will always be found.
            //
            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

            for K in 0..=save.NSCALE {
                save.SCALE = f64::powf(10.0, (save.MINSCL + ((K as f64) * save.DSCALE)));

                spicelib::VSCL(save.SCALE, save.XPT.as_slice(), save.SP.as_slice_mut());

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut save.TITLE, b"Ellipsoid: point = (#,#,#)");
                spicelib::REPMD(
                    &save.TITLE.to_vec(),
                    b"#",
                    save.SP[1],
                    14,
                    &mut save.TITLE,
                    ctx,
                );
                spicelib::REPMD(
                    &save.TITLE.to_vec(),
                    b"#",
                    save.SP[2],
                    14,
                    &mut save.TITLE,
                    ctx,
                );
                spicelib::REPMD(
                    &save.TITLE.to_vec(),
                    b"#",
                    save.SP[3],
                    14,
                    &mut save.TITLE,
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&save.TITLE, ctx)?;

                spicelib::EDPNT(
                    save.SP.as_slice(),
                    save.A,
                    save.B,
                    save.C,
                    save.P.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                //
                // Check P.
                //
                save.TOL = VTIGHT;

                testutil::CHCKAD(
                    b"P",
                    save.P.as_slice(),
                    b"~~/",
                    save.XPT.as_slice(),
                    3,
                    save.TOL,
                    OK,
                    ctx,
                )?;
            }
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
    testutil::TCASE(b"Invalid ellipsoid radii lengths.", ctx)?;

    save.A = 100.0;
    save.B = 50.0;
    save.C = 3.0;

    spicelib::VPACK(0.0, 0.0, 1.0, save.SP.as_slice_mut());

    spicelib::EDPNT(
        save.SP.as_slice(),
        -save.A,
        save.B,
        save.C,
        save.P.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDRADII)", OK, ctx)?;

    spicelib::EDPNT(
        save.SP.as_slice(),
        save.A,
        -save.B,
        save.C,
        save.P.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDRADII)", OK, ctx)?;

    spicelib::EDPNT(
        save.SP.as_slice(),
        save.A,
        save.B,
        -save.C,
        save.P.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDRADII)", OK, ctx)?;

    spicelib::EDPNT(
        save.SP.as_slice(),
        0.0,
        save.B,
        save.C,
        save.P.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDRADII)", OK, ctx)?;

    spicelib::EDPNT(
        save.SP.as_slice(),
        save.A,
        0.0,
        save.C,
        save.P.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDRADII)", OK, ctx)?;

    spicelib::EDPNT(
        save.SP.as_slice(),
        save.A,
        save.B,
        0.0,
        save.P.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDRADII)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Point too close to the origin.", ctx)?;

    save.A = 1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;
    save.B = save.A;
    save.C = save.A;

    spicelib::VPACK(1.0, 0.0, 0.0, save.SP.as_slice_mut());

    spicelib::EDPNT(
        save.SP.as_slice(),
        save.A,
        save.B,
        save.C,
        save.P.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(POINTTOOSMALL)", OK, ctx)?;

    spicelib::VPACK(0.0, 1.0, 0.0, save.SP.as_slice_mut());

    spicelib::EDPNT(
        save.SP.as_slice(),
        save.A,
        save.B,
        save.C,
        save.P.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(POINTTOOSMALL)", OK, ctx)?;

    spicelib::VPACK(0.0, 0.0, 1.0, save.SP.as_slice_mut());

    spicelib::EDPNT(
        save.SP.as_slice(),
        save.A,
        save.B,
        save.C,
        save.P.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(POINTTOOSMALL)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Input point was the zero vector.", ctx)?;

    save.A = 1.0;
    save.B = save.A;
    save.C = save.A;

    spicelib::CLEARD(3, save.SP.as_slice_mut());

    spicelib::EDPNT(
        save.SP.as_slice(),
        save.A,
        save.B,
        save.C,
        save.P.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ZEROVECTOR)", OK, ctx)?;

    //
    // ---------------------------------------------------------
    //

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
