//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const OSCXSZ: i32 = 20;
const NARGP: i32 = 8;
const NECC: i32 = 6;
const NINC: i32 = 7;
const NLNODE: i32 = 4;
const NM0: i32 = 8;
const NRP: i32 = 1;
const STRSIZ: i32 = 400;
const TIGHT: f64 = 0.0000000000001;
const MEDIUM: f64 = 0.000000000001;
const EASY: f64 = 0.0000000001;
const LOOSE: f64 = 0.00000001;
const VLOOSE: f64 = 0.000001;
const SLOPPY: f64 = 0.001;
const CLOSE: f64 = 0.0000000001;

struct SaveVars {
    TSTDSC: Vec<u8>,
    A: f64,
    COSEA: f64,
    COSHF: f64,
    D: f64,
    E: f64,
    EA: f64,
    ECC: f64,
    ELTS: StackArray<f64, 20>,
    ET: f64,
    F: f64,
    H: StackArray<f64, 3>,
    INELTS: StackArray<f64, 8>,
    LAT: f64,
    MU: f64,
    NODE: StackArray<f64, 3>,
    NU: f64,
    PERIM: StackArray2D<f64, 9>,
    PERIP: StackArray<f64, 3>,
    PERIX: StackArray<f64, 3>,
    PERIY: StackArray<f64, 3>,
    R: f64,
    SINEA: f64,
    STATE: StackArray<f64, 6>,
    STATE2: StackArray<f64, 6>,
    TAU: f64,
    TOL: f64,
    XA: f64,
    XARGP: StackArray<f64, 8>,
    XECC: StackArray<f64, 6>,
    XELTS: StackArray<f64, 20>,
    XINC: StackArray<f64, 7>,
    XLNODE: StackArray<f64, 4>,
    XM: f64,
    XM0: StackArray<f64, 8>,
    XNU: f64,
    XRP: StackArray<f64, 1>,
    XTAU: f64,
    Z: StackArray<f64, 3>,
    SUCCES: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TSTDSC = vec![b' '; STRSIZ as usize];
        let mut A: f64 = 0.0;
        let mut COSEA: f64 = 0.0;
        let mut COSHF: f64 = 0.0;
        let mut D: f64 = 0.0;
        let mut E: f64 = 0.0;
        let mut EA: f64 = 0.0;
        let mut ECC: f64 = 0.0;
        let mut ELTS = StackArray::<f64, 20>::new(1..=OSCXSZ);
        let mut ET: f64 = 0.0;
        let mut F: f64 = 0.0;
        let mut H = StackArray::<f64, 3>::new(1..=3);
        let mut INELTS = StackArray::<f64, 8>::new(1..=8);
        let mut LAT: f64 = 0.0;
        let mut MU: f64 = 0.0;
        let mut NODE = StackArray::<f64, 3>::new(1..=3);
        let mut NU: f64 = 0.0;
        let mut PERIM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut PERIP = StackArray::<f64, 3>::new(1..=3);
        let mut PERIX = StackArray::<f64, 3>::new(1..=3);
        let mut PERIY = StackArray::<f64, 3>::new(1..=3);
        let mut R: f64 = 0.0;
        let mut SINEA: f64 = 0.0;
        let mut STATE = StackArray::<f64, 6>::new(1..=6);
        let mut STATE2 = StackArray::<f64, 6>::new(1..=6);
        let mut TAU: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut XA: f64 = 0.0;
        let mut XARGP = StackArray::<f64, 8>::new(1..=NARGP);
        let mut XECC = StackArray::<f64, 6>::new(1..=NECC);
        let mut XELTS = StackArray::<f64, 20>::new(1..=OSCXSZ);
        let mut XINC = StackArray::<f64, 7>::new(1..=NINC);
        let mut XLNODE = StackArray::<f64, 4>::new(1..=NLNODE);
        let mut XM: f64 = 0.0;
        let mut XM0 = StackArray::<f64, 8>::new(1..=NM0);
        let mut XNU: f64 = 0.0;
        let mut XRP = StackArray::<f64, 1>::new(1..=NRP);
        let mut XTAU: f64 = 0.0;
        let mut Z = StackArray::<f64, 3>::new(1..=3);
        let mut SUCCES: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            Z.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            TSTDSC,
            A,
            COSEA,
            COSHF,
            D,
            E,
            EA,
            ECC,
            ELTS,
            ET,
            F,
            H,
            INELTS,
            LAT,
            MU,
            NODE,
            NU,
            PERIM,
            PERIP,
            PERIX,
            PERIY,
            R,
            SINEA,
            STATE,
            STATE2,
            TAU,
            TOL,
            XA,
            XARGP,
            XECC,
            XELTS,
            XINC,
            XLNODE,
            XM,
            XM0,
            XNU,
            XRP,
            XTAU,
            Z,
            SUCCES,
        }
    }
}

//$Procedure      F_OSCLTX ( OSCLTX routine tests )
pub fn F_OSCLTX(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Test Utility Functions
    //

    //
    // SPICELIB Functions
    //

    //
    // Local Parameters
    //

    //
    // The value of CLOSE must match that used in OSCLTX.
    //

    //
    // Local Variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_OSCLTX", ctx)?;

    //
    // Error tests:
    //
    for I in 1..=3 {
        save.STATE[I] = (1000.0 * I as f64);
    }

    for I in 4..=6 {
        save.STATE[I] = -(10.0 * I as f64);
    }

    testutil::TCASE(b"Invalid GM.", ctx)?;

    spicelib::OSCLTX(
        save.STATE.as_slice(),
        100.0,
        0.0,
        save.ELTS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NONPOSITIVEMASS)", OK, ctx)?;

    testutil::TCASE(b"Specific angular momentum == 0", ctx)?;

    for I in 4..=6 {
        save.STATE[I] = 0.0;
    }

    spicelib::OSCLTX(
        save.STATE.as_slice(),
        100.0,
        10000000000.0,
        save.ELTS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(DEGENERATECASE)", OK, ctx)?;

    //
    // Normal cases:
    //

    //
    // Set up the expected element values.
    //
    //
    // Expected argument of periapse:
    //
    for IARGP in 1..=NARGP {
        save.XARGP[IARGP] = ((((IARGP - 1) as f64) * spicelib::TWOPI(ctx)) / NARGP as f64);
    }

    //
    // Expected eccentricity:
    //
    save.XECC[1] = 0.0;
    save.XECC[2] = 0.5;
    save.XECC[3] = 0.999;
    save.XECC[4] = 1.0;
    save.XECC[5] = 1.001;
    save.XECC[6] = 1.5;

    //
    // Expected inclination:
    //
    save.XINC[1] = 0.0;
    save.XINC[2] = (CLOSE / 2 as f64);
    save.XINC[3] = (0.25 * spicelib::PI(ctx));
    save.XINC[4] = (0.5 * spicelib::PI(ctx));
    save.XINC[5] = (0.75 * spicelib::PI(ctx));
    save.XINC[6] = (spicelib::PI(ctx) - (CLOSE / 2 as f64));
    save.XINC[7] = spicelib::PI(ctx);

    //
    // Expected longitude of the ascending node:
    //
    for ILNODE in 1..=NLNODE {
        save.XLNODE[ILNODE] = ((((ILNODE - 1) as f64) * spicelib::TWOPI(ctx)) / NLNODE as f64);
    }

    //
    // Expected mean anomaly:
    //
    for IM0 in 1..=NM0 {
        save.XM0[IM0] = ((((IM0 - 1) as f64) * spicelib::TWOPI(ctx)) / NM0 as f64);
    }

    //
    // Epoch:
    //
    save.ET = 100000000.0;

    //
    // Expected perifocal distance:
    //
    save.XRP[1] = f64::powi(2.0, 18);

    //
    // GM of central body (we make this the cube of RP so we may
    // recover an eccentricity of zero when we have a circular orbit):
    //
    save.MU = f64::powi(2.0, 54);

    //
    // For a variety of element sets, we'll use CONICS to produce
    // the equivalent state vector.  We'll then try to recover
    // the elements from the state vector.
    //
    for IARGP in 1..=NARGP {
        for IECC in 1..=NECC {
            for IINC in 1..=NINC {
                for ILNODE in 1..=NLNODE {
                    for IM0 in 1..=NM0 {
                        //
                        // Assign the input elements for this test case.
                        //
                        save.INELTS[1] = save.XRP[1];
                        save.INELTS[2] = save.XECC[IECC];
                        save.INELTS[3] = save.XINC[IINC];
                        save.INELTS[4] = save.XLNODE[ILNODE];
                        save.INELTS[5] = save.XARGP[IARGP];
                        save.INELTS[6] = save.XM0[IM0];
                        save.INELTS[7] = save.ET;
                        save.INELTS[8] = save.MU;

                        //
                        // Set the expected elements.
                        //
                        for I in 1..=8 {
                            save.XELTS[I] = save.INELTS[I];
                        }

                        // In some cases, these are not the input
                        // elements, so adjust accordingly:
                        //
                        // If the input inclination is close to 0 or pi,
                        // the output inclination will be rounded.  The
                        // longitude of the ascending node becomes 0.
                        //
                        if (f64::abs((save.INELTS[3] - 0.0)) < CLOSE) {
                            save.XELTS[3] = 0.0;
                            save.XELTS[4] = 0.0;
                        } else if (f64::abs((save.INELTS[3] - spicelib::PI(ctx))) < CLOSE) {
                            save.XELTS[3] = spicelib::PI(ctx);
                            save.XELTS[4] = 0.0;
                        }

                        //
                        // If the eccentricity is "close" to 1, make it 1.
                        //
                        save.XELTS[2] = spicelib::EXACT(save.XELTS[2], 1.0, CLOSE);

                        //
                        // Set up the test description for the TCASE call.
                        //
                        fstr::assign(&mut save.TSTDSC, b"RP = #; ECC = #; INC = #; LNODE = #; ARGP = #; M0 = #; ET = #;, MU = #");
                        for I in 1..=8 {
                            spicelib::REPMD(
                                &save.TSTDSC.to_vec(),
                                b"#",
                                save.INELTS[I],
                                14,
                                &mut save.TSTDSC,
                                ctx,
                            );
                        }

                        testutil::TCASE(&save.TSTDSC, ctx)?;

                        //
                        // Obtain the equivalent state vector, then call
                        // OSCLTX.
                        //
                        spicelib::CONICS(
                            save.INELTS.as_slice(),
                            save.ET,
                            save.STATE.as_slice_mut(),
                            ctx,
                        )?;

                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        spicelib::OSCLTX(
                            save.STATE.as_slice(),
                            save.ET,
                            save.MU,
                            save.ELTS.as_slice_mut(),
                            ctx,
                        )?;

                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        // WRITE (*,*) 'ELTS(2) = ', ELTS(2)
                        // WRITE (*,*) 'ELTS(3) = ', ELTS(3)

                        save.SUCCES = true;
                        //
                        // See how well we did.
                        //
                        // Set the tolerance to use for RP; acknowledge
                        // that we can't recover RP for nearly
                        // parabolic orbits.
                        //

                        if (save.INELTS[2] == 1.0) {
                            save.TOL = MEDIUM;
                        } else if (f64::abs((save.INELTS[2] - 1.0)) < CLOSE) {
                            save.TOL = SLOPPY;
                        } else if (f64::abs((save.INELTS[2] - 1.0)) < 0.0001) {
                            save.TOL = LOOSE;
                        } else {
                            save.TOL = EASY;
                        }

                        testutil::CHCKSD(
                            b"RP",
                            save.ELTS[1],
                            b"~/",
                            save.XELTS[1],
                            save.TOL,
                            OK,
                            ctx,
                        )?;

                        save.SUCCES = (save.SUCCES && *OK);

                        //
                        // We use the same tolerances for eccentricity
                        // as for inclination.
                        //
                        testutil::CHCKSD(
                            b"ECC",
                            save.ELTS[2],
                            b"~",
                            save.XELTS[2],
                            save.TOL,
                            OK,
                            ctx,
                        )?;

                        save.SUCCES = (save.SUCCES && *OK);

                        //
                        // We should always be able to recover inclination
                        // reasonably accurately.
                        //
                        save.TOL = EASY;

                        testutil::CHCKSD(
                            b"INC",
                            save.ELTS[3],
                            b"~",
                            save.XELTS[3],
                            save.TOL,
                            OK,
                            ctx,
                        )?;

                        save.SUCCES = (save.SUCCES && *OK);

                        //
                        // When the inclination is not too close to 0
                        // or pi, we should be able to recover it
                        // reasonably accurately.
                        //
                        if (f64::sin(save.XELTS[4]) > CLOSE) {
                            save.TOL = EASY;
                        } else {
                            save.TOL = LOOSE;
                        }

                        if (f64::abs((save.ELTS[4] - save.XELTS[4])) <= spicelib::PI(ctx)) {
                            testutil::CHCKSD(
                                b"LNODE",
                                save.ELTS[4],
                                b"~",
                                save.XELTS[4],
                                save.TOL,
                                OK,
                                ctx,
                            )?;
                        } else {
                            if (save.ELTS[4] > save.XELTS[4]) {
                                save.XELTS[4] = (save.XELTS[4] + spicelib::TWOPI(ctx));
                            } else {
                                save.XELTS[4] = (save.XELTS[4] - spicelib::TWOPI(ctx));
                            }

                            testutil::CHCKSD(
                                b"LNODE",
                                save.ELTS[4],
                                b"~",
                                save.XELTS[4],
                                save.TOL,
                                OK,
                                ctx,
                            )?;
                        }

                        save.SUCCES = (save.SUCCES && *OK);

                        //
                        // Check ARGP:
                        //
                        // When the eccentricity is not too close to
                        // zero and the inclination is not too close to
                        // zero or pi, we can use a reasonably tight tolerance
                        // for ARGP.  When the eccentricity is determined
                        // by OSCLTX to be zero, we know exactly what
                        // ARGP is supposed to be.  When the eccentricity
                        // is very close to but not equal to zero, ARGP
                        // can be almost anything.  For these cases all
                        // we can do is check ARGP+M0.
                        //
                        // Note that our path is governed by the eccentricity
                        // found by OSCLTX, since an input eccentricity of
                        // zero may not be recovered as such.
                        //
                        if ((save.INELTS[3] > CLOSE)
                            && (save.INELTS[3] < (spicelib::PI(ctx) - CLOSE)))
                        {
                            //
                            // These are the normal inclination cases.
                            //
                            if (f64::abs((save.ELTS[2] - 1.0)) <= CLOSE) {
                                save.TOL = spicelib::DPMAX();
                            } else if (save.ELTS[2] > 0.00001) {
                                save.TOL = MEDIUM;
                            } else if (save.ELTS[2] > 0.000001) {
                                save.TOL = EASY;
                            } else if (save.ELTS[2] == 0.0) {
                                save.TOL = MEDIUM;
                            } else {
                                save.TOL = spicelib::DPMAX();
                            }
                        } else if ((save.INELTS[3] == 0.0) || (save.INELTS[3] == spicelib::PI(ctx)))
                        {
                            //
                            // These are exceptional, but reasonably
                            // well-behaved inclination cases.
                            //
                            if (f64::abs((save.ELTS[2] - 1.0)) <= CLOSE) {
                                save.TOL = spicelib::DPMAX();
                            } else if (f64::abs((save.INELTS[2] - 1.0)) <= 0.01) {
                                save.TOL = LOOSE;
                            } else if (save.ELTS[2] > 0.000001) {
                                save.TOL = MEDIUM;
                            } else if (save.ELTS[2] == 0.0) {
                                save.TOL = MEDIUM;
                            } else {
                                save.TOL = spicelib::DPMAX();
                            }
                        } else {
                            if (f64::abs((save.ELTS[2] - 1.0)) <= CLOSE) {
                                save.TOL = spicelib::DPMAX();
                            } else if ((save.ELTS[2] > CLOSE) || (save.ELTS[2] == 0.0)) {
                                save.TOL = EASY;
                            } else {
                                save.TOL = spicelib::DPMAX();
                            }
                        }

                        //
                        // When the eccentricity is zero, the argument of
                        // periapse is absorbed into the mean anomaly.  This
                        // only happens if OSCLTX is able to determine that
                        // the eccentricity is zero, so test ELTS(2).
                        //
                        if (save.ELTS[2] == 0.0) {
                            save.XELTS[6] = (save.XELTS[5] + save.XELTS[6]);
                            save.XELTS[5] = 0.0;
                        }
                        //
                        // Adjust our expected value of ARGP if
                        // LNODE has been set to zero and if the
                        // eccentricity is non-zero.  The sign of
                        // the adjustment depends on INC.
                        //
                        if ((((save.ELTS[4] == 0.0) && (save.INELTS[4] != 0.0))
                            && (save.ELTS[2] != 0.0))
                            && (save.ELTS[3] == 0.0))
                        {
                            save.XELTS[5] = (save.XELTS[5] + (save.INELTS[4] - save.ELTS[4]));
                        } else if ((((save.ELTS[4] == 0.0) && (save.INELTS[4] != 0.0))
                            && (save.ELTS[2] != 0.0))
                            && (save.ELTS[3] == spicelib::PI(ctx)))
                        {
                            save.XELTS[5] = (save.XELTS[5] + (save.ELTS[4] - save.INELTS[4]));
                        }

                        if (save.XELTS[5] >= spicelib::TWOPI(ctx)) {
                            save.XELTS[5] = (save.XELTS[5] - spicelib::TWOPI(ctx));
                        } else if (save.XELTS[5] < 0.0) {
                            save.XELTS[5] = (save.XELTS[5] + spicelib::TWOPI(ctx));
                        }

                        if (f64::abs((save.ELTS[5] - save.XELTS[5])) <= spicelib::PI(ctx)) {
                            testutil::CHCKSD(
                                b"ARGP",
                                save.ELTS[5],
                                b"~",
                                save.XELTS[5],
                                save.TOL,
                                OK,
                                ctx,
                            )?;
                        } else {
                            if (save.ELTS[5] > save.XELTS[5]) {
                                save.XELTS[5] = (save.XELTS[5] + spicelib::TWOPI(ctx));
                            } else {
                                save.XELTS[5] = (save.XELTS[5] - spicelib::TWOPI(ctx));
                            }

                            testutil::CHCKSD(
                                b"ARGP",
                                save.ELTS[5],
                                b"~",
                                save.XELTS[5],
                                save.TOL,
                                OK,
                                ctx,
                            )?;
                        }

                        save.SUCCES = (save.SUCCES && *OK);

                        //
                        // Check M0:
                        //
                        if (f64::abs((save.ELTS[2] - 1.0)) <= CLOSE) {
                            save.TOL = spicelib::DPMAX();
                        } else if (f64::abs((save.ELTS[2] - 1.0)) <= 0.01) {
                            save.TOL = SLOPPY;
                        } else if (save.ELTS[2] > CLOSE) {
                            save.TOL = MEDIUM;
                        } else if (save.ELTS[2] == 0.0) {
                            save.TOL = MEDIUM;
                        } else {
                            save.TOL = spicelib::DPMAX();
                        }

                        save.SUCCES = (save.SUCCES && *OK);

                        //
                        // If the eccentricity is zero and the inclination
                        // is zero or pi, the original
                        // longitude of the ascending node is absorbed into
                        // M0.
                        //
                        if ((save.ELTS[2] == 0.0) && (save.ELTS[3] == 0.0)) {
                            //
                            // Eccentricity and inclination were both
                            // found to be zero by OSCLTX.  The input
                            // value of the argument of periapse is going
                            // to be added onto the mean anomaly.
                            //
                            save.XELTS[6] = (save.XELTS[6] + save.INELTS[4]);
                        } else if ((save.ELTS[2] == 0.0) && (save.ELTS[3] == spicelib::PI(ctx))) {
                            //
                            // Eccentricity was found to be zero and
                            // inclination was found to be pi by OSCLTX.
                            // The input value of the argument of periapse
                            // is going to be subtracted from the mean anomaly.
                            //
                            save.XELTS[6] = (save.XELTS[6] - save.INELTS[4]);
                        }

                        if (f64::abs((save.ELTS[6] - save.XELTS[6])) <= spicelib::PI(ctx)) {
                            testutil::CHCKSD(
                                b"M0",
                                save.ELTS[6],
                                b"~",
                                save.XELTS[6],
                                save.TOL,
                                OK,
                                ctx,
                            )?;
                        } else {
                            if (save.ELTS[6] > save.XELTS[6]) {
                                save.XELTS[6] = (save.XELTS[6] + spicelib::TWOPI(ctx));
                            } else {
                                save.XELTS[6] = (save.XELTS[6] - spicelib::TWOPI(ctx));
                            }

                            testutil::CHCKSD(
                                b"M0",
                                save.ELTS[6],
                                b"~",
                                save.XELTS[6],
                                save.TOL,
                                OK,
                                ctx,
                            )?;
                        }

                        save.SUCCES = (save.SUCCES && *OK);

                        testutil::CHCKSD(b"MU", save.ELTS[8], b"=", save.XELTS[8], TIGHT, OK, ctx)?;

                        //
                        // Now that we've obtained elements, see whether
                        // we can use them to obtain the state we got
                        // back from CONICS the first time.
                        //
                        spicelib::CONICS(
                            save.ELTS.as_slice(),
                            save.ET,
                            save.STATE2.as_slice_mut(),
                            ctx,
                        )?;

                        if (f64::abs((save.INELTS[2] - 1.0)) <= 0.01) {
                            save.TOL = VLOOSE;
                        } else if ((save.INELTS[3] == 0.0) || (save.INELTS[3] >= spicelib::PI(ctx)))
                        {
                            save.TOL = MEDIUM;
                        } else if ((save.INELTS[3] <= CLOSE)
                            || (save.INELTS[3] >= (spicelib::PI(ctx) - CLOSE)))
                        {
                            save.TOL = EASY;
                        } else {
                            save.TOL = MEDIUM;
                        }

                        testutil::CHCKAD(
                            b"Position",
                            save.STATE2.as_slice(),
                            b"~~/",
                            save.STATE.as_slice(),
                            3,
                            save.TOL,
                            OK,
                            ctx,
                        )?;

                        testutil::CHCKAD(
                            b"Velocity",
                            save.STATE2.subarray(4),
                            b"~~/",
                            save.STATE.subarray(4),
                            3,
                            save.TOL,
                            OK,
                            ctx,
                        )?;

                        //
                        // Check NU:
                        //
                        save.NU = save.ELTS[9];

                        if (save.INELTS[2] == 0.0) {
                            //
                            // The orbit is circular. We expect NU to
                            // be equal to M0.
                            //
                            save.XNU = save.ELTS[6];

                            if (f64::abs(save.NU) > 1.0) {
                                testutil::CHCKSD(b"NU", save.NU, b"~/", save.XNU, MEDIUM, OK, ctx)?;
                            } else {
                                testutil::CHCKSD(b"NU", save.NU, b"~", save.XNU, MEDIUM, OK, ctx)?;
                            }
                        } else {
                            //
                            // This is a non-circular orbit.
                            //
                            // We'll use the state and elements we already
                            // computed to find the basis vectors of the
                            // perifocal frame.
                            //
                            //
                            // Start with the specific angular momentum vector:
                            //
                            spicelib::VCRSS(
                                save.STATE.as_slice(),
                                save.STATE.subarray(4),
                                save.H.as_slice_mut(),
                            );

                            //
                            // The ascending node:
                            //
                            if (f64::abs(f64::sin(save.INELTS[3])) < CLOSE) {
                                //
                                // Handle the case of inclination equal to zero
                                // or pi.
                                //
                                spicelib::VPACK(1.0, 0.0, 0.0, save.NODE.as_slice_mut());
                            } else {
                                spicelib::UCRSS(
                                    save.Z.as_slice(),
                                    save.H.as_slice(),
                                    save.NODE.as_slice_mut(),
                                );
                            }

                            //
                            // Rotate the node vector about H by the argument
                            // of periapse to obtain the perifocal X vector:
                            //
                            spicelib::VROTV(
                                save.NODE.as_slice(),
                                save.H.as_slice(),
                                save.ELTS[5],
                                save.PERIX.as_slice_mut(),
                            );

                            spicelib::UCRSS(
                                save.H.as_slice(),
                                save.PERIX.as_slice(),
                                save.PERIY.as_slice_mut(),
                            );

                            //
                            // Create a matrix to transform vectors from the
                            // base frame to the perifocal frame:
                            //
                            spicelib::TWOVEC(
                                save.PERIX.as_slice(),
                                1,
                                save.PERIY.as_slice(),
                                2,
                                save.PERIM.as_slice_mut(),
                                ctx,
                            )?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            if *OK {
                                //
                                // Map the position to the perifocal frame.
                                // The longitude of the transformed vector is
                                // the expected true anomaly.
                                //
                                spicelib::MXV(
                                    save.PERIM.as_slice(),
                                    save.STATE.as_slice(),
                                    save.PERIP.as_slice_mut(),
                                );
                                spicelib::RECRAD(
                                    save.PERIP.as_slice(),
                                    &mut save.R,
                                    &mut save.XNU,
                                    &mut save.LAT,
                                    ctx,
                                );

                                if (save.XNU < (save.NU - spicelib::PI(ctx))) {
                                    save.XNU = (save.XNU + spicelib::TWOPI(ctx));
                                } else if (save.XNU > (save.NU + spicelib::PI(ctx))) {
                                    save.XNU = (save.XNU - spicelib::TWOPI(ctx));
                                }
                            }

                            if (f64::abs(save.XNU) > 0.01) {
                                testutil::CHCKSD(b"NU", save.NU, b"~/", save.XNU, EASY, OK, ctx)?;
                            } else {
                                testutil::CHCKSD(b"NU", save.NU, b"~", save.XNU, EASY, OK, ctx)?;
                            }
                        }

                        //
                        // Check the compatibility of NU and M0: derive one of
                        //
                        //    - the eccentric anomaly
                        //    - the parabolic anomaly
                        //    - the hyperbolic anomaly
                        //
                        // from NU, then compare to M0.
                        //

                        save.ECC = save.ELTS[2];

                        if (save.ECC < 1.0) {
                            //
                            // This is the elliptical case.
                            //
                            // The mean anomaly can be computed as shown below.
                            //
                            //               e + cos(nu)
                            //    cos(E)  = ---------------         (ellipse)
                            //               1 + e cos(nu)
                            //
                            //    M0      = E - e sin(E)
                            //
                            //

                            save.COSEA = ((save.ECC + f64::cos(save.NU))
                                / (1.0 + (save.ECC * f64::cos(save.NU))));

                            //
                            // From the definition of the eccentric anomaly,
                            // we have
                            //
                            //    a sin(EA) = (a/b) * r * sin(nu)
                            //
                            // Here b is the semi-minor axis, so
                            //
                            //     2    2    2    2
                            //    a  - b  = a  ecc
                            //
                            //
                            // and
                            //
                            //     2    2        2
                            //    b  = a (1 - ecc )
                            //
                            //       = [a(1 - ecc)] * a * (1 + ecc)
                            //
                            //           2          2
                            //       = [a  (1 - ecc) ] * (1 + ecc)/(1 - ecc)
                            //
                            //            2
                            //       = r_p  * (1 + ecc) / (1 - ecc)
                            //
                            // so
                            //
                            //    1/b = 1/r_p  * sqrt( (1 - ecc)/(1 + ecc) )
                            //
                            // Then
                            //
                            //    sin(EA) = (1/b) * r *sin(nu)
                            //
                            //            = (r/r_p) * sin(nu)
                            //                      * sqrt((1-ecc)/(1+ecc))
                            //
                            //
                            save.SINEA = (((spicelib::VNORM(save.STATE.as_slice())
                                / save.ELTS[1])
                                * f64::sin(save.NU))
                                * f64::sqrt(((1.0 - save.ECC) / (1.0 + save.ECC))));

                            save.EA = f64::atan2(save.SINEA, save.COSEA);

                            save.XM = (save.EA - (save.ECC * save.SINEA));

                            if (save.XM < 0.0) {
                                save.XM = (save.XM + spicelib::TWOPI(ctx));
                            }

                            if (save.XM < (save.ELTS[6] - spicelib::PI(ctx))) {
                                save.XM = (save.XM + spicelib::TWOPI(ctx));
                            } else if (save.XM > (save.ELTS[6] + spicelib::PI(ctx))) {
                                save.XM = (save.XM - spicelib::TWOPI(ctx));
                            }

                            if (f64::abs(save.XM) > 0.01) {
                                testutil::CHCKSD(
                                    b"M",
                                    save.ELTS[6],
                                    b"~/",
                                    save.XM,
                                    EASY,
                                    OK,
                                    ctx,
                                )?;
                            } else {
                                testutil::CHCKSD(b"M", save.ELTS[6], b"~", save.XM, EASY, OK, ctx)?;
                            }
                        } else if (save.ECC == 1.0) {
                            //
                            // This is the parabolic case.
                            //
                            // The mean anomaly can be computed as shown
                            // below.
                            //
                            //    D   = tan(nu/2)
                            //
                            //                 3
                            //    M0  = D  +  D / 3
                            //

                            save.D = f64::tan((save.NU / 2 as f64));

                            save.XM = (save.D + (f64::powi(save.D, 3) / 3 as f64));

                            if (f64::abs(save.XM) > 0.01) {
                                testutil::CHCKSD(
                                    b"M",
                                    save.ELTS[6],
                                    b"~/",
                                    save.XM,
                                    EASY,
                                    OK,
                                    ctx,
                                )?;
                            } else {
                                testutil::CHCKSD(b"M", save.ELTS[6], b"~", save.XM, EASY, OK, ctx)?;
                            }
                        } else {
                            //
                            // ECC > 1
                            //
                            //
                            // This is the hyperbolic case.
                            //
                            // The mean anomaly can be computed as shown
                            // below.
                            //
                            //
                            //                e + cos(nu)
                            //    cosh(F) = ---------------         (hyperbola)
                            //                1 + e cos(nu)
                            //
                            //    M       = e sinh(F) - F
                            //

                            save.COSHF = ((save.ECC + f64::cos(save.NU))
                                / (1.0 + (save.ECC * f64::cos(save.NU))));

                            //
                            // Make sure we have a valid value of COSHF.
                            //
                            save.COSHF = intrinsics::DMAX1(&[1.0, save.COSHF]);

                            save.F = spicelib::DACOSH(save.COSHF, ctx)?;
                            //
                            // Note that DACOSH is a SPICELIB function, not a
                            // Fortran intrinsic. DACOSH can signal an error.
                            //
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            save.XM = ((save.ECC * f64::sinh(save.F)) - save.F);

                            if (save.NU < 0.0) {
                                save.XM = -save.XM;
                            }

                            if (f64::abs(save.XM) > 0.01) {
                                testutil::CHCKSD(
                                    b"M",
                                    save.ELTS[6],
                                    b"~/",
                                    save.XM,
                                    EASY,
                                    OK,
                                    ctx,
                                )?;
                            } else {
                                testutil::CHCKSD(b"M", save.ELTS[6], b"~", save.XM, EASY, OK, ctx)?;
                            }
                        }

                        //
                        // Check A:
                        //
                        save.A = save.ELTS[10];

                        if (save.ELTS[2] < 1.0) {
                            //
                            // For elliptical orbits, A = r_p / (1-ecc):
                            //
                            save.XA = (save.ELTS[1] / (1.0 - save.ELTS[2]));

                            save.TOL = MEDIUM;

                            testutil::CHCKSD(
                                b"A (ecc<1)",
                                save.A,
                                b"~/",
                                save.XA,
                                save.TOL,
                                OK,
                                ctx,
                            )?;
                        } else if (save.ELTS[2] == 1.0) {
                            //
                            // For parabolic orbits, A should be set to zero.
                            //
                            save.XA = 0.0;

                            testutil::CHCKSD(
                                b"A (ecc==1)",
                                save.A,
                                b"=",
                                save.XA,
                                save.TOL,
                                OK,
                                ctx,
                            )?;
                        }

                        //
                        // Check of A for all non-parabolic orbits: compute
                        // the energy E; then let
                        //
                        //    E = - mu / (2A)
                        //
                        // and solve for A:
                        //
                        //    A = - mu / (2E)
                        //
                        //
                        // To compute E, we note that E is constant
                        // over the orbit, so it suffices to compute
                        // E at periapse:
                        //
                        //             2
                        //    E = ( v_p / 2 )  -  mu / r_p
                        //
                        // where v_p and r_p are the speed and distance
                        // of the orbiting object, relative to the center
                        // of motion, at periapse.
                        //
                        // We also know that the semi-latus rectum p
                        // is equal to both
                        //
                        //     2             2      2
                        //    h / mu  =  (r_p  * v_p ) / mu
                        //
                        //
                        // and to
                        //
                        //    r_p ( 1 + ecc )
                        //
                        // Then
                        //
                        //       2
                        //    v_p  = mu * ( 1 + ecc ) / r_p
                        //
                        // So
                        //
                        //    E    = mu * ( ecc - 1 ) / ( 2 * r_p )
                        //
                        //
                        save.E = ((save.MU * (save.ELTS[2] - 1.0)) / (2.0 * save.ELTS[1]));

                        if (save.E != 0.0) {
                            save.XA = -(save.MU / ((2 as f64) * save.E));

                            save.TOL = EASY;

                            testutil::CHCKSD(
                                b"A (ecc!=1)",
                                save.A,
                                b"~/",
                                save.XA,
                                save.TOL,
                                OK,
                                ctx,
                            )?;
                        }

                        save.SUCCES = (save.SUCCES && *OK);

                        //
                        // Check TAU:
                        //
                        // If we have an elliptical orbit, compute the
                        // expected value of TAU. Otherwise, TAU should
                        // be set to zero.
                        //
                        save.TAU = save.ELTS[11];

                        if (save.ELTS[2] < 1.0) {
                            //
                            // For elliptical orbits, A = r_p / (1-ecc):
                            //
                            save.XA = (save.ELTS[1] / (1.0 - save.ELTS[2]));

                            save.XTAU = (spicelib::TWOPI(ctx)
                                * f64::sqrt((f64::powi(save.XA, 3) / save.MU)));

                            save.TOL = MEDIUM;

                            testutil::CHCKSD(
                                b"TAU", save.TAU, b"~/", save.XTAU, save.TOL, OK, ctx,
                            )?;
                        } else {
                            save.TOL = 0.0;

                            save.XTAU = 0.0;

                            testutil::CHCKSD(b"TAU", save.TAU, b"=", save.XTAU, save.TOL, OK, ctx)?;
                        }

                        save.SUCCES = (save.SUCCES && *OK);

                        if !save.SUCCES {
                            // WRITE (*, *) 'ELTS: '
                            // WRITE (*, '(8(1X,E25.16))') ELTS
                            save.SUCCES = true;
                        }
                    }
                }
            }
        }
    }

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
