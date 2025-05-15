//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

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

//$Procedure      F_OSCELT ( OSCELT routine tests )
pub fn F_OSCELT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut TSTDSC = [b' '; STRSIZ as usize];
    let mut ELTS = StackArray::<f64, 8>::new(1..=8);
    let mut ET: f64 = 0.0;
    let mut INELTS = StackArray::<f64, 8>::new(1..=8);
    let mut MU: f64 = 0.0;
    let mut STATE = StackArray::<f64, 6>::new(1..=6);
    let mut STATE2 = StackArray::<f64, 6>::new(1..=6);
    let mut TOL: f64 = 0.0;
    let mut XARGP = StackArray::<f64, 8>::new(1..=NARGP);
    let mut XECC = StackArray::<f64, 6>::new(1..=NECC);
    let mut XELTS = StackArray::<f64, 8>::new(1..=8);
    let mut XINC = StackArray::<f64, 7>::new(1..=NINC);
    let mut XLNODE = StackArray::<f64, 4>::new(1..=NLNODE);
    let mut XM0 = StackArray::<f64, 8>::new(1..=NM0);
    let mut XRP = StackArray::<f64, 1>::new(1..=NRP);
    let mut SUCCES: bool = false;

    //
    // Test Utility Functions
    //

    //
    // SPICELIB Functions
    //

    // DOUBLE PRECISION      VNORM

    //
    // Local Parameters
    //

    //
    // The value of CLOSE must match that used in OSCELT.
    //

    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_OSCELT", ctx)?;

    //
    // Error tests:
    //
    for I in 1..=3 {
        STATE[I] = (1000.0 * I as f64);
    }

    for I in 4..=6 {
        STATE[I] = -(10.0 * I as f64);
    }

    testutil::TCASE(b"Invalid GM.", ctx)?;

    spicelib::OSCELT(STATE.as_slice(), 100.0, 0.0, ELTS.as_slice_mut(), ctx)?;

    testutil::CHCKXC(true, b"SPICE(NONPOSITIVEMASS)", OK, ctx)?;

    testutil::TCASE(b"Specific angular momentum == 0", ctx)?;

    for I in 4..=6 {
        STATE[I] = 0.0;
    }

    spicelib::OSCELT(
        STATE.as_slice(),
        100.0,
        10000000000.0,
        ELTS.as_slice_mut(),
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
        XARGP[IARGP] = ((((IARGP - 1) as f64) * spicelib::TWOPI(ctx)) / NARGP as f64);
    }

    //
    // Expected eccentricity:
    //
    XECC[1] = 0.0;
    XECC[2] = 0.5;
    XECC[3] = 0.999;
    XECC[4] = 1.0;
    XECC[5] = 1.001;
    XECC[6] = 1.5;

    //
    // Expected inclination:
    //
    XINC[1] = 0.0;
    XINC[2] = (CLOSE / 2 as f64);
    XINC[3] = (0.25 * spicelib::PI(ctx));
    XINC[4] = (0.5 * spicelib::PI(ctx));
    XINC[5] = (0.75 * spicelib::PI(ctx));
    XINC[6] = (spicelib::PI(ctx) - (CLOSE / 2 as f64));
    XINC[7] = spicelib::PI(ctx);

    //
    // Expected longitude of the ascending node:
    //
    for ILNODE in 1..=NLNODE {
        XLNODE[ILNODE] = ((((ILNODE - 1) as f64) * spicelib::TWOPI(ctx)) / NLNODE as f64);
    }

    //
    // Expected mean anomaly:
    //
    for IM0 in 1..=NM0 {
        XM0[IM0] = ((((IM0 - 1) as f64) * spicelib::TWOPI(ctx)) / NM0 as f64);
    }

    //
    // Epoch:
    //
    ET = 100000000.0;

    //
    // Expected perifocal distance:
    //
    XRP[1] = f64::powi(2.0, 18);

    //
    // GM of central body (we make this the cube of RP so we may
    // recover an eccentricity of zero when we have a circular orbit):
    //
    MU = f64::powi(2.0, 54);

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
                        INELTS[1] = XRP[1];
                        INELTS[2] = XECC[IECC];
                        INELTS[3] = XINC[IINC];
                        INELTS[4] = XLNODE[ILNODE];
                        INELTS[5] = XARGP[IARGP];
                        INELTS[6] = XM0[IM0];
                        INELTS[7] = ET;
                        INELTS[8] = MU;

                        //
                        // Set the expected elements.
                        //
                        for I in 1..=8 {
                            XELTS[I] = INELTS[I];
                        }

                        // In some cases, these are not the input
                        // elements, so adjust accordingly:
                        //
                        // If the input inclination is close to 0 or pi,
                        // the output inclination will be rounded.  The
                        // longitude of the ascending node becomes 0.
                        //
                        if (f64::abs((INELTS[3] - 0.0)) < CLOSE) {
                            XELTS[3] = 0.0;
                            XELTS[4] = 0.0;
                        } else if (f64::abs((INELTS[3] - spicelib::PI(ctx))) < CLOSE) {
                            XELTS[3] = spicelib::PI(ctx);
                            XELTS[4] = 0.0;
                        }

                        //
                        // If the eccentricity is "close" to 1, make it 1.
                        //
                        XELTS[2] = spicelib::EXACT(XELTS[2], 1.0, CLOSE);

                        //
                        // Set up the test description for the TCASE call.
                        //
                        fstr::assign(&mut TSTDSC, b"RP = #; ECC = #; INC = #; LNODE = #; ARGP = #; M0 = #; ET = #;, MU = #");
                        for I in 1..=8 {
                            spicelib::REPMD(&TSTDSC.clone(), b"#", INELTS[I], 14, &mut TSTDSC, ctx);
                        }

                        testutil::TCASE(&TSTDSC, ctx)?;

                        //
                        // Obtain the equivalent state vector, then call
                        // OSCELT.
                        //
                        spicelib::CONICS(INELTS.as_slice(), ET, STATE.as_slice_mut(), ctx)?;

                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        spicelib::OSCELT(STATE.as_slice(), ET, MU, ELTS.as_slice_mut(), ctx)?;

                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        // WRITE (*,*) 'ELTS(2) = ', ELTS(2)
                        // WRITE (*,*) 'ELTS(3) = ', ELTS(3)

                        SUCCES = true;
                        //
                        // See how well we did.
                        //
                        // Set the tolerance to use for RP; acknowledge
                        // that we can't recover RP for nearly
                        // parabolic orbits.
                        //

                        if (INELTS[2] == 1.0) {
                            TOL = MEDIUM;
                        } else if (f64::abs((INELTS[2] - 1.0)) < CLOSE) {
                            TOL = SLOPPY;
                        } else if (f64::abs((INELTS[2] - 1.0)) < 0.0001) {
                            TOL = LOOSE;
                        } else {
                            TOL = EASY;
                        }

                        testutil::CHCKSD(b"RP", ELTS[1], b"~/", XELTS[1], TOL, OK, ctx)?;

                        SUCCES = (SUCCES && *OK);

                        //
                        // We use the same tolerances for eccentricity
                        // as for inclination.
                        //
                        testutil::CHCKSD(b"ECC", ELTS[2], b"~", XELTS[2], TOL, OK, ctx)?;

                        SUCCES = (SUCCES && *OK);

                        //
                        // We should always be able to recover inclination
                        // reasonably accurately.
                        //
                        TOL = EASY;

                        testutil::CHCKSD(b"INC", ELTS[3], b"~", XELTS[3], TOL, OK, ctx)?;

                        SUCCES = (SUCCES && *OK);

                        //
                        // When the inclination is not too close to 0
                        // or pi, we should be able to recover it
                        // reasonably accurately.
                        //
                        if (f64::sin(XELTS[4]) > CLOSE) {
                            TOL = EASY;
                        } else {
                            TOL = LOOSE;
                        }

                        if (f64::abs((ELTS[4] - XELTS[4])) <= spicelib::PI(ctx)) {
                            testutil::CHCKSD(b"LNODE", ELTS[4], b"~", XELTS[4], TOL, OK, ctx)?;
                        } else {
                            if (ELTS[4] > XELTS[4]) {
                                XELTS[4] = (XELTS[4] + spicelib::TWOPI(ctx));
                            } else {
                                XELTS[4] = (XELTS[4] - spicelib::TWOPI(ctx));
                            }

                            testutil::CHCKSD(b"LNODE", ELTS[4], b"~", XELTS[4], TOL, OK, ctx)?;
                        }

                        SUCCES = (SUCCES && *OK);

                        //
                        // Check ARGP:
                        //
                        // When the eccentricity is not too close to
                        // zero and the inclination is not too close to
                        // zero or pi, we can use a reasonably tight tolerance
                        // for ARGP.  When the eccentricity is determined
                        // by OSCELT to be zero, we know exactly what
                        // ARGP is supposed to be.  When the eccentricity
                        // is very close to but not equal to zero, ARGP
                        // can be almost anything.  For these cases all
                        // we can do is check ARGP+M0.
                        //
                        // Note that our path is governed by the eccentricity
                        // found by OSCELT, since an input eccentricity of
                        // zero may not be recovered as such.
                        //
                        if ((INELTS[3] > CLOSE) && (INELTS[3] < (spicelib::PI(ctx) - CLOSE))) {
                            //
                            // These are the normal inclination cases.
                            //
                            if (f64::abs((ELTS[2] - 1.0)) <= CLOSE) {
                                TOL = spicelib::DPMAX();
                            } else if (ELTS[2] > 0.00001) {
                                TOL = MEDIUM;
                            } else if (ELTS[2] > 0.000001) {
                                TOL = EASY;
                            } else if (ELTS[2] == 0.0) {
                                TOL = MEDIUM;
                            } else {
                                TOL = spicelib::DPMAX();
                            }
                        } else if ((INELTS[3] == 0.0) || (INELTS[3] == spicelib::PI(ctx))) {
                            //
                            // These are exceptional, but reasonably
                            // well-behaved inclination cases.
                            //
                            if (f64::abs((ELTS[2] - 1.0)) <= CLOSE) {
                                TOL = spicelib::DPMAX();
                            } else if (f64::abs((INELTS[2] - 1.0)) <= 0.01) {
                                TOL = LOOSE;
                            } else if (ELTS[2] > 0.000001) {
                                TOL = MEDIUM;
                            } else if (ELTS[2] == 0.0) {
                                TOL = MEDIUM;
                            } else {
                                TOL = spicelib::DPMAX();
                            }
                        } else {
                            if (f64::abs((ELTS[2] - 1.0)) <= CLOSE) {
                                TOL = spicelib::DPMAX();
                            } else if ((ELTS[2] > CLOSE) || (ELTS[2] == 0.0)) {
                                TOL = EASY;
                            } else {
                                TOL = spicelib::DPMAX();
                            }
                        }

                        //
                        // When the eccentricity is zero, the argument of
                        // periapse is absorbed into the mean anomaly.  This
                        // only happens if OSCELT is able to determine that
                        // the eccentricity is zero, so test ELTS(2).
                        //
                        if (ELTS[2] == 0.0) {
                            XELTS[6] = (XELTS[5] + XELTS[6]);
                            XELTS[5] = 0.0;
                        }
                        //
                        // Adjust our expected value of ARGP if
                        // LNODE has been set to zero and if the
                        // eccentricity is non-zero.  The sign of
                        // the adjustment depends on INC.
                        //
                        if ((((ELTS[4] == 0.0) && (INELTS[4] != 0.0)) && (ELTS[2] != 0.0))
                            && (ELTS[3] == 0.0))
                        {
                            XELTS[5] = (XELTS[5] + (INELTS[4] - ELTS[4]));
                        } else if ((((ELTS[4] == 0.0) && (INELTS[4] != 0.0)) && (ELTS[2] != 0.0))
                            && (ELTS[3] == spicelib::PI(ctx)))
                        {
                            XELTS[5] = (XELTS[5] + (ELTS[4] - INELTS[4]));
                        }

                        if (XELTS[5] >= spicelib::TWOPI(ctx)) {
                            XELTS[5] = (XELTS[5] - spicelib::TWOPI(ctx));
                        } else if (XELTS[5] < 0.0) {
                            XELTS[5] = (XELTS[5] + spicelib::TWOPI(ctx));
                        }

                        if (f64::abs((ELTS[5] - XELTS[5])) <= spicelib::PI(ctx)) {
                            testutil::CHCKSD(b"ARGP", ELTS[5], b"~", XELTS[5], TOL, OK, ctx)?;
                        } else {
                            if (ELTS[5] > XELTS[5]) {
                                XELTS[5] = (XELTS[5] + spicelib::TWOPI(ctx));
                            } else {
                                XELTS[5] = (XELTS[5] - spicelib::TWOPI(ctx));
                            }

                            testutil::CHCKSD(b"ARGP", ELTS[5], b"~", XELTS[5], TOL, OK, ctx)?;
                        }

                        SUCCES = (SUCCES && *OK);

                        //
                        // Check M0:
                        //
                        if (f64::abs((ELTS[2] - 1.0)) <= CLOSE) {
                            TOL = spicelib::DPMAX();
                        } else if (f64::abs((ELTS[2] - 1.0)) <= 0.01) {
                            TOL = SLOPPY;
                        } else if (ELTS[2] > CLOSE) {
                            TOL = MEDIUM;
                        } else if (ELTS[2] == 0.0) {
                            TOL = MEDIUM;
                        } else {
                            TOL = spicelib::DPMAX();
                        }

                        SUCCES = (SUCCES && *OK);

                        //
                        // If the eccentricity is zero and the inclination
                        // is zero or pi, the original
                        // longitude of the ascending node is absorbed into
                        // M0.
                        //
                        if ((ELTS[2] == 0.0) && (ELTS[3] == 0.0)) {
                            //
                            // Eccentricity and inclination were both
                            // found to be zero by OSCELT.  The input
                            // value of the argument of periapse is going
                            // to be added onto the mean anomaly.
                            //
                            XELTS[6] = (XELTS[6] + INELTS[4]);
                        } else if ((ELTS[2] == 0.0) && (ELTS[3] == spicelib::PI(ctx))) {
                            //
                            // Eccentricity was found to be zero and
                            // inclination was found to be pi by OSCELT.
                            // The input value of the argument of periapse
                            // is going to be subtracted from the mean anomaly.
                            //
                            XELTS[6] = (XELTS[6] - INELTS[4]);
                        }

                        if (f64::abs((ELTS[6] - XELTS[6])) <= spicelib::PI(ctx)) {
                            testutil::CHCKSD(b"M0", ELTS[6], b"~", XELTS[6], TOL, OK, ctx)?;
                        } else {
                            if (ELTS[6] > XELTS[6]) {
                                XELTS[6] = (XELTS[6] + spicelib::TWOPI(ctx));
                            } else {
                                XELTS[6] = (XELTS[6] - spicelib::TWOPI(ctx));
                            }

                            testutil::CHCKSD(b"M0", ELTS[6], b"~", XELTS[6], TOL, OK, ctx)?;
                        }

                        SUCCES = (SUCCES && *OK);

                        testutil::CHCKSD(b"MU", ELTS[8], b"=", XELTS[8], TIGHT, OK, ctx)?;

                        //
                        // Now that we've obtained elements, see whether
                        // we can use them to obtain the state we got
                        // back from CONICS the first time.
                        //
                        spicelib::CONICS(ELTS.as_slice(), ET, STATE2.as_slice_mut(), ctx)?;

                        //
                        // IF ( IECC .EQ. 3  .AND.VNORM(STATE(4)).GT.1D3) THEN
                        //    WRITE (*,*) '-----------------'
                        //    WRITE (*,*) 'XECC(IECC)   = ', XECC(IECC)
                        //    WRITE (*,*) 'INELTS(2)    = ', INELTS(2)
                        //    WRITE (*,*) 'STATE        = ', STATE
                        //    WRITE (*,*) 'STATE2       = ', STATE2
                        //    WRITE (*,*) 'XM0(IM0)     = ', XM0(IM0)
                        // END IF
                        //

                        if (f64::abs((INELTS[2] - 1.0)) <= 0.01) {
                            TOL = VLOOSE;
                        } else if ((INELTS[3] == 0.0) || (INELTS[3] >= spicelib::PI(ctx))) {
                            TOL = MEDIUM;
                        } else if ((INELTS[3] <= CLOSE)
                            || (INELTS[3] >= (spicelib::PI(ctx) - CLOSE)))
                        {
                            TOL = EASY;
                        } else {
                            TOL = MEDIUM;
                        }

                        testutil::CHCKAD(
                            b"Position",
                            STATE2.as_slice(),
                            b"~~/",
                            STATE.as_slice(),
                            3,
                            TOL,
                            OK,
                            ctx,
                        )?;

                        testutil::CHCKAD(
                            b"Velocity",
                            STATE2.subarray(4),
                            b"~~/",
                            STATE.subarray(4),
                            3,
                            TOL,
                            OK,
                            ctx,
                        )?;

                        if !SUCCES {
                            // WRITE (*, *) 'ELTS: '
                            // WRITE (*, '(8(1X,E25.16))') ELTS
                            SUCCES = true;
                        }
                    }
                }
            }
        }
    }

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
