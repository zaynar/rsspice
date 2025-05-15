//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure F_EQNCPV (Family of tests for EQNCPV)
pub fn F_EQNCPV(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut A: f64 = 0.0;
    let mut ARGP: f64 = 0.0;
    let mut DECPOL: f64 = 0.0;
    let mut ECC: f64 = 0.0;
    let mut ELTS = StackArray::<f64, 8>::new(1..=8);
    let mut EQEL = StackArray::<f64, 9>::new(1..=9);
    let mut ET: f64 = 0.0;
    let mut FIVDPD: f64 = 0.0;
    let mut GM: f64 = 0.0;
    let mut INC: f64 = 0.0;
    let mut M0: f64 = 0.0;
    let mut N: f64 = 0.0;
    let mut NODE: f64 = 0.0;
    let mut P: f64 = 0.0;
    let mut RAPOL: f64 = 0.0;
    let mut T0: f64 = 0.0;
    let mut TENDPD: f64 = 0.0;
    let mut THETA: f64 = 0.0;
    let mut ROT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut TOINRT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut STATE = StackArray::<f64, 6>::new(1..=6);
    let mut STATE1 = StackArray::<f64, 6>::new(1..=6);
    let mut STATE2 = StackArray::<f64, 6>::new(1..=6);
    let mut TMPSTA = StackArray::<f64, 6>::new(1..=6);
    let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut ROTTO = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut ROTFRM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut TEMP = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut K = StackArray::<f64, 3>::new(1..=3);
    let mut X = StackArray::<f64, 3>::new(1..=3);
    let mut Y = StackArray::<f64, 3>::new(1..=3);
    let mut Z = StackArray::<f64, 3>::new(1..=3);

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_EQNCPV", ctx)?;

    testutil::TCASE(b"This compares EQNCPV with CONICS for the case when the rates of node and argument of periapse are zero and the pole of the central frame is aligned with the pole of an inertial frame. ", ctx)?;

    P = 10000.0;
    GM = 398600.436;
    ECC = 0.1;
    A = (P / (1.0 - ECC));
    N = (f64::sqrt((GM / A)) / A);
    ARGP = (30.0 * spicelib::RPD(ctx));
    NODE = (15.0 * spicelib::RPD(ctx));
    INC = (10.0 * spicelib::RPD(ctx));
    M0 = (45.0 * spicelib::RPD(ctx));
    T0 = -100000000.0;

    ELTS[1] = P;
    ELTS[2] = ECC;
    ELTS[3] = INC;
    ELTS[4] = NODE;
    ELTS[5] = ARGP;
    ELTS[6] = M0;
    ELTS[7] = T0;
    ELTS[8] = GM;

    EQEL[1] = A;
    EQEL[2] = (ECC * f64::sin((ARGP + NODE)));
    EQEL[3] = (ECC * f64::cos((ARGP + NODE)));
    EQEL[4] = ((M0 + ARGP) + NODE);
    EQEL[5] = (f64::tan((INC / 2.0)) * f64::sin(NODE));
    EQEL[6] = (f64::tan((INC / 2.0)) * f64::cos(NODE));
    EQEL[7] = 0.0;
    EQEL[8] = N;
    EQEL[9] = 0.0;

    RAPOL = -spicelib::HALFPI(ctx);
    DECPOL = spicelib::HALFPI(ctx);

    ET = (T0 - 10000.0);

    for I in 1..=100 {
        ET = (ET + 250.0);

        spicelib::CONICS(ELTS.as_slice(), ET, STATE1.as_slice_mut(), ctx)?;
        spicelib::EQNCPV(
            ET,
            T0,
            EQEL.as_slice(),
            RAPOL,
            DECPOL,
            STATE2.as_slice_mut(),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"POSITION",
            STATE2.subarray(1),
            b"~/",
            STATE1.subarray(1),
            3,
            0.000000000005,
            OK,
            ctx,
        )?;
        testutil::CHCKAD(
            b"VELOCITY",
            STATE2.subarray(4),
            b"~/",
            STATE1.subarray(4),
            3,
            0.000000000005,
            OK,
            ctx,
        )?;
    }

    testutil::TCASE(b"Test to make sure we can accurately compute the state of an object that has non-zero rates for the longitude of the ascending node. ", ctx)?;

    P = 10000.0;
    GM = 398600.436;
    ECC = 0.1;
    A = (P / (1.0 - ECC));
    N = (f64::sqrt((GM / A)) / A);
    ARGP = (30.0 * spicelib::RPD(ctx));
    NODE = (15.0 * spicelib::RPD(ctx));
    INC = (10.0 * spicelib::RPD(ctx));
    M0 = (45.0 * spicelib::RPD(ctx));
    T0 = -100000000.0;

    ELTS[1] = P;
    ELTS[2] = ECC;
    ELTS[3] = INC;
    ELTS[4] = NODE;
    ELTS[5] = ARGP;
    ELTS[6] = M0;
    ELTS[7] = T0;
    ELTS[8] = GM;
    //
    // We want a rate for the node of 10 degrees/day.
    //
    TENDPD = ((10.0 / 86400.0) * spicelib::RPD(ctx));

    EQEL[1] = A;
    EQEL[2] = (ECC * f64::sin((ARGP + NODE)));
    EQEL[3] = (ECC * f64::cos((ARGP + NODE)));
    EQEL[4] = ((M0 + ARGP) + NODE);
    EQEL[5] = (f64::tan((INC / 2.0)) * f64::sin(NODE));
    EQEL[6] = (f64::tan((INC / 2.0)) * f64::cos(NODE));
    EQEL[7] = TENDPD;
    EQEL[8] = (N + TENDPD);
    EQEL[9] = TENDPD;

    RAPOL = -spicelib::HALFPI(ctx);
    DECPOL = spicelib::HALFPI(ctx);

    ET = (T0 - 10000.0);

    for I in 1..=100 {
        ET = (ET + 250.0);

        THETA = ((ET - T0) * TENDPD);

        XFORM[[1, 1]] = f64::cos(THETA);
        XFORM[[2, 1]] = f64::sin(THETA);
        XFORM[[3, 1]] = 0.0;
        XFORM[[4, 1]] = -(f64::sin(THETA) * TENDPD);
        XFORM[[5, 1]] = (f64::cos(THETA) * TENDPD);
        XFORM[[6, 1]] = 0.0;

        XFORM[[1, 2]] = -f64::sin(THETA);
        XFORM[[2, 2]] = f64::cos(THETA);
        XFORM[[3, 2]] = 0.0;
        XFORM[[4, 2]] = -(f64::cos(THETA) * TENDPD);
        XFORM[[5, 2]] = -(f64::sin(THETA) * TENDPD);
        XFORM[[6, 2]] = 0.0;

        XFORM[[1, 3]] = 0.0;
        XFORM[[2, 3]] = 0.0;
        XFORM[[3, 3]] = 1.0;
        XFORM[[4, 3]] = 0.0;
        XFORM[[5, 3]] = 0.0;
        XFORM[[6, 3]] = 0.0;

        XFORM[[1, 4]] = 0.0;
        XFORM[[2, 4]] = 0.0;
        XFORM[[3, 4]] = 0.0;
        XFORM[[4, 4]] = f64::cos(THETA);
        XFORM[[5, 4]] = f64::sin(THETA);
        XFORM[[6, 4]] = 0.0;

        XFORM[[1, 5]] = 0.0;
        XFORM[[2, 5]] = 0.0;
        XFORM[[3, 5]] = 0.0;
        XFORM[[4, 5]] = -f64::sin(THETA);
        XFORM[[5, 5]] = f64::cos(THETA);
        XFORM[[6, 5]] = 0.0;

        XFORM[[1, 6]] = 0.0;
        XFORM[[2, 6]] = 0.0;
        XFORM[[3, 6]] = 0.0;
        XFORM[[4, 6]] = 0.0;
        XFORM[[5, 6]] = 0.0;
        XFORM[[6, 6]] = 1.0;

        spicelib::CONICS(ELTS.as_slice(), ET, STATE.as_slice_mut(), ctx)?;
        spicelib::MXVG(
            XFORM.as_slice(),
            STATE.as_slice(),
            6,
            6,
            STATE1.as_slice_mut(),
        );

        spicelib::EQNCPV(
            ET,
            T0,
            EQEL.as_slice(),
            RAPOL,
            DECPOL,
            STATE2.as_slice_mut(),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"POSITION",
            STATE2.subarray(1),
            b"~/",
            STATE1.subarray(1),
            3,
            0.000000000005,
            OK,
            ctx,
        )?;
        testutil::CHCKAD(
            b"VELOCITY",
            STATE2.subarray(4),
            b"~/",
            STATE1.subarray(4),
            3,
            0.000000000005,
            OK,
            ctx,
        )?;
    }

    testutil::TCASE(b"Test to make sure that we can accurately compute the state of an object that has a non-zero rate for the argument of periapse. ", ctx)?;

    P = 10000.0;
    GM = 398600.436;
    ECC = 0.1;
    A = (P / (1.0 - ECC));
    N = (f64::sqrt((GM / A)) / A);
    ARGP = (30.0 * spicelib::RPD(ctx));
    NODE = (15.0 * spicelib::RPD(ctx));
    INC = (10.0 * spicelib::RPD(ctx));
    M0 = (45.0 * spicelib::RPD(ctx));
    T0 = -100000000.0;

    ELTS[1] = P;
    ELTS[2] = ECC;
    ELTS[3] = INC;
    ELTS[4] = NODE;
    ELTS[5] = ARGP;
    ELTS[6] = M0;
    ELTS[7] = T0;
    ELTS[8] = GM;
    //
    // We want a rate for the node of 10 degrees/day.
    //
    FIVDPD = ((5.0 / 86400.0) * spicelib::RPD(ctx));

    EQEL[1] = A;
    EQEL[2] = (ECC * f64::sin((ARGP + NODE)));
    EQEL[3] = (ECC * f64::cos((ARGP + NODE)));
    EQEL[4] = ((M0 + ARGP) + NODE);
    EQEL[5] = (f64::tan((INC / 2.0)) * f64::sin(NODE));
    EQEL[6] = (f64::tan((INC / 2.0)) * f64::cos(NODE));
    EQEL[7] = FIVDPD;
    EQEL[8] = (N + FIVDPD);
    EQEL[9] = 0.0;

    RAPOL = -spicelib::HALFPI(ctx);
    DECPOL = spicelib::HALFPI(ctx);

    ROT[[1, 1]] = f64::cos(NODE);
    ROT[[2, 1]] = f64::sin(NODE);
    ROT[[3, 1]] = 0.0;

    ROT[[1, 2]] = -(f64::cos(INC) * f64::sin(NODE));
    ROT[[2, 2]] = (f64::cos(INC) * f64::cos(NODE));
    ROT[[3, 2]] = f64::sin(INC);

    ROT[[1, 3]] = (f64::sin(INC) * f64::sin(NODE));
    ROT[[2, 3]] = -(f64::sin(INC) * f64::cos(NODE));
    ROT[[3, 3]] = f64::cos(INC);

    for I in 1..=3 {
        for J in 1..=3 {
            ROTTO[[I, J]] = ROT[[I, J]];
            ROTTO[[(I + 3), (J + 3)]] = ROT[[I, J]];
            ROTTO[[(I + 3), J]] = 0.0;
            ROTTO[[I, (J + 3)]] = 0.0;
        }
    }

    spicelib::XPOSEG(ROTTO.as_slice(), 6, 6, ROTFRM.as_slice_mut());

    ET = (T0 - 10000.0);

    for I in 1..=100 {
        ET = (ET + 250.0);
        THETA = ((ET - T0) * FIVDPD);

        XFORM[[1, 1]] = f64::cos(THETA);
        XFORM[[2, 1]] = f64::sin(THETA);
        XFORM[[3, 1]] = 0.0;
        XFORM[[4, 1]] = -(f64::sin(THETA) * FIVDPD);
        XFORM[[5, 1]] = (f64::cos(THETA) * FIVDPD);
        XFORM[[6, 1]] = 0.0;

        XFORM[[1, 2]] = -f64::sin(THETA);
        XFORM[[2, 2]] = f64::cos(THETA);
        XFORM[[3, 2]] = 0.0;
        XFORM[[4, 2]] = -(f64::cos(THETA) * FIVDPD);
        XFORM[[5, 2]] = -(f64::sin(THETA) * FIVDPD);
        XFORM[[6, 2]] = 0.0;

        XFORM[[1, 3]] = 0.0;
        XFORM[[2, 3]] = 0.0;
        XFORM[[3, 3]] = 1.0;
        XFORM[[4, 3]] = 0.0;
        XFORM[[5, 3]] = 0.0;
        XFORM[[6, 3]] = 0.0;

        XFORM[[1, 4]] = 0.0;
        XFORM[[2, 4]] = 0.0;
        XFORM[[3, 4]] = 0.0;
        XFORM[[4, 4]] = f64::cos(THETA);
        XFORM[[5, 4]] = f64::sin(THETA);
        XFORM[[6, 4]] = 0.0;

        XFORM[[1, 5]] = 0.0;
        XFORM[[2, 5]] = 0.0;
        XFORM[[3, 5]] = 0.0;
        XFORM[[4, 5]] = -f64::sin(THETA);
        XFORM[[5, 5]] = f64::cos(THETA);
        XFORM[[6, 5]] = 0.0;

        XFORM[[1, 6]] = 0.0;
        XFORM[[2, 6]] = 0.0;
        XFORM[[3, 6]] = 0.0;
        XFORM[[4, 6]] = 0.0;
        XFORM[[5, 6]] = 0.0;
        XFORM[[6, 6]] = 1.0;

        spicelib::MXMG(
            XFORM.as_slice(),
            ROTFRM.as_slice(),
            6,
            6,
            6,
            TEMP.as_slice_mut(),
        );
        spicelib::MXMG(
            ROTTO.as_slice(),
            TEMP.as_slice(),
            6,
            6,
            6,
            XFORM.as_slice_mut(),
        );

        spicelib::CONICS(ELTS.as_slice(), ET, STATE.as_slice_mut(), ctx)?;
        spicelib::MXVG(
            XFORM.as_slice(),
            STATE.as_slice(),
            6,
            6,
            STATE1.as_slice_mut(),
        );

        spicelib::EQNCPV(
            ET,
            T0,
            EQEL.as_slice(),
            RAPOL,
            DECPOL,
            STATE2.as_slice_mut(),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"POSITION",
            STATE2.subarray(1),
            b"~/",
            STATE1.subarray(1),
            3,
            0.000000000005,
            OK,
            ctx,
        )?;
        testutil::CHCKAD(
            b"VELOCITY",
            STATE2.subarray(4),
            b"~/",
            STATE1.subarray(4),
            3,
            0.000000000005,
            OK,
            ctx,
        )?;
    }

    testutil::TCASE(b"Test the equinoctial propagator when precession of both the node and argument of periapse are non-zero. ", ctx)?;

    P = 10000.0;
    GM = 398600.436;
    ECC = 0.1;
    A = (P / (1.0 - ECC));
    N = (f64::sqrt((GM / A)) / A);
    ARGP = (30.0 * spicelib::RPD(ctx));
    NODE = (15.0 * spicelib::RPD(ctx));
    INC = (10.0 * spicelib::RPD(ctx));
    M0 = (45.0 * spicelib::RPD(ctx));
    T0 = -100000000.0;

    ELTS[1] = P;
    ELTS[2] = ECC;
    ELTS[3] = INC;
    ELTS[4] = NODE;
    ELTS[5] = ARGP;
    ELTS[6] = M0;
    ELTS[7] = T0;
    ELTS[8] = GM;
    //
    // We want a rate for the node of 10 degrees/day.
    //
    FIVDPD = ((5.0 / 86400.0) * spicelib::RPD(ctx));

    EQEL[1] = A;
    EQEL[2] = (ECC * f64::sin((ARGP + NODE)));
    EQEL[3] = (ECC * f64::cos((ARGP + NODE)));
    EQEL[4] = ((M0 + ARGP) + NODE);
    EQEL[5] = (f64::tan((INC / 2.0)) * f64::sin(NODE));
    EQEL[6] = (f64::tan((INC / 2.0)) * f64::cos(NODE));
    EQEL[7] = (FIVDPD + TENDPD);
    EQEL[8] = ((N + FIVDPD) + TENDPD);
    EQEL[9] = TENDPD;

    RAPOL = -spicelib::HALFPI(ctx);
    DECPOL = spicelib::HALFPI(ctx);

    ROT[[1, 1]] = f64::cos(NODE);
    ROT[[2, 1]] = f64::sin(NODE);
    ROT[[3, 1]] = 0.0;

    ROT[[1, 2]] = -(f64::cos(INC) * f64::sin(NODE));
    ROT[[2, 2]] = (f64::cos(INC) * f64::cos(NODE));
    ROT[[3, 2]] = f64::sin(INC);

    ROT[[1, 3]] = (f64::sin(INC) * f64::sin(NODE));
    ROT[[2, 3]] = -(f64::sin(INC) * f64::cos(NODE));
    ROT[[3, 3]] = f64::cos(INC);

    for I in 1..=3 {
        for J in 1..=3 {
            ROTTO[[I, J]] = ROT[[I, J]];
            ROTTO[[(I + 3), (J + 3)]] = ROT[[I, J]];
            ROTTO[[(I + 3), J]] = 0.0;
            ROTTO[[I, (J + 3)]] = 0.0;
        }
    }

    spicelib::XPOSEG(ROTTO.as_slice(), 6, 6, ROTFRM.as_slice_mut());

    ET = (T0 - 10000.0);

    for I in 1..=100 {
        ET = (ET + 250.0);
        THETA = ((ET - T0) * FIVDPD);

        XFORM[[1, 1]] = f64::cos(THETA);
        XFORM[[2, 1]] = f64::sin(THETA);
        XFORM[[3, 1]] = 0.0;
        XFORM[[4, 1]] = -(f64::sin(THETA) * FIVDPD);
        XFORM[[5, 1]] = (f64::cos(THETA) * FIVDPD);
        XFORM[[6, 1]] = 0.0;

        XFORM[[1, 2]] = -f64::sin(THETA);
        XFORM[[2, 2]] = f64::cos(THETA);
        XFORM[[3, 2]] = 0.0;
        XFORM[[4, 2]] = -(f64::cos(THETA) * FIVDPD);
        XFORM[[5, 2]] = -(f64::sin(THETA) * FIVDPD);
        XFORM[[6, 2]] = 0.0;

        XFORM[[1, 3]] = 0.0;
        XFORM[[2, 3]] = 0.0;
        XFORM[[3, 3]] = 1.0;
        XFORM[[4, 3]] = 0.0;
        XFORM[[5, 3]] = 0.0;
        XFORM[[6, 3]] = 0.0;

        XFORM[[1, 4]] = 0.0;
        XFORM[[2, 4]] = 0.0;
        XFORM[[3, 4]] = 0.0;
        XFORM[[4, 4]] = f64::cos(THETA);
        XFORM[[5, 4]] = f64::sin(THETA);
        XFORM[[6, 4]] = 0.0;

        XFORM[[1, 5]] = 0.0;
        XFORM[[2, 5]] = 0.0;
        XFORM[[3, 5]] = 0.0;
        XFORM[[4, 5]] = -f64::sin(THETA);
        XFORM[[5, 5]] = f64::cos(THETA);
        XFORM[[6, 5]] = 0.0;

        XFORM[[1, 6]] = 0.0;
        XFORM[[2, 6]] = 0.0;
        XFORM[[3, 6]] = 0.0;
        XFORM[[4, 6]] = 0.0;
        XFORM[[5, 6]] = 0.0;
        XFORM[[6, 6]] = 1.0;

        spicelib::MXMG(
            XFORM.as_slice(),
            ROTFRM.as_slice(),
            6,
            6,
            6,
            TEMP.as_slice_mut(),
        );
        spicelib::MXMG(
            ROTTO.as_slice(),
            TEMP.as_slice(),
            6,
            6,
            6,
            XFORM.as_slice_mut(),
        );

        spicelib::CONICS(ELTS.as_slice(), ET, STATE1.as_slice_mut(), ctx)?;
        spicelib::MXVG(
            XFORM.as_slice(),
            STATE1.as_slice(),
            6,
            6,
            STATE.as_slice_mut(),
        );

        THETA = ((ET - T0) * TENDPD);

        XFORM[[1, 1]] = f64::cos(THETA);
        XFORM[[2, 1]] = f64::sin(THETA);
        XFORM[[3, 1]] = 0.0;
        XFORM[[4, 1]] = -(f64::sin(THETA) * TENDPD);
        XFORM[[5, 1]] = (f64::cos(THETA) * TENDPD);
        XFORM[[6, 1]] = 0.0;

        XFORM[[1, 2]] = -f64::sin(THETA);
        XFORM[[2, 2]] = f64::cos(THETA);
        XFORM[[3, 2]] = 0.0;
        XFORM[[4, 2]] = -(f64::cos(THETA) * TENDPD);
        XFORM[[5, 2]] = -(f64::sin(THETA) * TENDPD);
        XFORM[[6, 2]] = 0.0;

        XFORM[[1, 3]] = 0.0;
        XFORM[[2, 3]] = 0.0;
        XFORM[[3, 3]] = 1.0;
        XFORM[[4, 3]] = 0.0;
        XFORM[[5, 3]] = 0.0;
        XFORM[[6, 3]] = 0.0;

        XFORM[[1, 4]] = 0.0;
        XFORM[[2, 4]] = 0.0;
        XFORM[[3, 4]] = 0.0;
        XFORM[[4, 4]] = f64::cos(THETA);
        XFORM[[5, 4]] = f64::sin(THETA);
        XFORM[[6, 4]] = 0.0;

        XFORM[[1, 5]] = 0.0;
        XFORM[[2, 5]] = 0.0;
        XFORM[[3, 5]] = 0.0;
        XFORM[[4, 5]] = -f64::sin(THETA);
        XFORM[[5, 5]] = f64::cos(THETA);
        XFORM[[6, 5]] = 0.0;

        XFORM[[1, 6]] = 0.0;
        XFORM[[2, 6]] = 0.0;
        XFORM[[3, 6]] = 0.0;
        XFORM[[4, 6]] = 0.0;
        XFORM[[5, 6]] = 0.0;
        XFORM[[6, 6]] = 1.0;

        spicelib::MXVG(
            XFORM.as_slice(),
            STATE.as_slice(),
            6,
            6,
            STATE1.as_slice_mut(),
        );

        spicelib::EQNCPV(
            ET,
            T0,
            EQEL.as_slice(),
            RAPOL,
            DECPOL,
            STATE2.as_slice_mut(),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"POSITION",
            STATE2.subarray(1),
            b"~/",
            STATE1.subarray(1),
            3,
            0.000000000005,
            OK,
            ctx,
        )?;
        testutil::CHCKAD(
            b"VELOCITY",
            STATE2.subarray(4),
            b"~/",
            STATE1.subarray(4),
            3,
            0.000000000005,
            OK,
            ctx,
        )?;
    }

    testutil::TCASE(b"Apply the same test as the previous case with the RA and DEC of the pole of the equatorial frame set so that the axes of the equatorial frame are not aligned with the inertial frame. ", ctx)?;

    P = 10000.0;
    GM = 398600.436;
    ECC = 0.1;
    A = (P / (1.0 - ECC));
    N = (f64::sqrt((GM / A)) / A);
    ARGP = (30.0 * spicelib::RPD(ctx));
    NODE = (15.0 * spicelib::RPD(ctx));
    INC = (10.0 * spicelib::RPD(ctx));
    M0 = (45.0 * spicelib::RPD(ctx));
    T0 = -100000000.0;

    ELTS[1] = P;
    ELTS[2] = ECC;
    ELTS[3] = INC;
    ELTS[4] = NODE;
    ELTS[5] = ARGP;
    ELTS[6] = M0;
    ELTS[7] = T0;
    ELTS[8] = GM;
    //
    // We want a rate for the node of 10 degrees/day.
    //
    FIVDPD = ((5.0 / 86400.0) * spicelib::RPD(ctx));

    EQEL[1] = A;
    EQEL[2] = (ECC * f64::sin((ARGP + NODE)));
    EQEL[3] = (ECC * f64::cos((ARGP + NODE)));
    EQEL[4] = ((M0 + ARGP) + NODE);
    EQEL[5] = (f64::tan((INC / 2.0)) * f64::sin(NODE));
    EQEL[6] = (f64::tan((INC / 2.0)) * f64::cos(NODE));
    EQEL[7] = (FIVDPD + TENDPD);
    EQEL[8] = ((N + FIVDPD) + TENDPD);
    EQEL[9] = TENDPD;

    RAPOL = (30.0 * spicelib::RPD(ctx));
    DECPOL = (60.0 * spicelib::RPD(ctx));

    ROT[[1, 1]] = f64::cos(NODE);
    ROT[[2, 1]] = f64::sin(NODE);
    ROT[[3, 1]] = 0.0;

    ROT[[1, 2]] = -(f64::cos(INC) * f64::sin(NODE));
    ROT[[2, 2]] = (f64::cos(INC) * f64::cos(NODE));
    ROT[[3, 2]] = f64::sin(INC);

    ROT[[1, 3]] = (f64::sin(INC) * f64::sin(NODE));
    ROT[[2, 3]] = -(f64::sin(INC) * f64::cos(NODE));
    ROT[[3, 3]] = f64::cos(INC);

    Z[1] = (f64::cos(RAPOL) * f64::cos(DECPOL));
    Z[2] = (f64::sin(RAPOL) * f64::cos(DECPOL));
    Z[3] = f64::sin(DECPOL);

    K[1] = 0.0;
    K[2] = 0.0;
    K[3] = 1.0;

    spicelib::UCRSS(K.as_slice(), Z.as_slice(), X.as_slice_mut());
    spicelib::UCRSS(Z.as_slice(), X.as_slice(), Y.as_slice_mut());

    TOINRT[[1, 1]] = X[1];
    TOINRT[[2, 1]] = X[2];
    TOINRT[[3, 1]] = X[3];

    TOINRT[[1, 2]] = Y[1];
    TOINRT[[2, 2]] = Y[2];
    TOINRT[[3, 2]] = Y[3];

    TOINRT[[1, 3]] = Z[1];
    TOINRT[[2, 3]] = Z[2];
    TOINRT[[3, 3]] = Z[3];

    for I in 1..=3 {
        for J in 1..=3 {
            ROTTO[[I, J]] = ROT[[I, J]];
            ROTTO[[(I + 3), (J + 3)]] = ROT[[I, J]];
            ROTTO[[(I + 3), J]] = 0.0;
            ROTTO[[I, (J + 3)]] = 0.0;
        }
    }

    spicelib::XPOSEG(ROTTO.as_slice(), 6, 6, ROTFRM.as_slice_mut());

    ET = (T0 - 10000.0);

    for I in 1..=100 {
        ET = (ET + 250.0);
        THETA = ((ET - T0) * FIVDPD);

        XFORM[[1, 1]] = f64::cos(THETA);
        XFORM[[2, 1]] = f64::sin(THETA);
        XFORM[[3, 1]] = 0.0;
        XFORM[[4, 1]] = -(f64::sin(THETA) * FIVDPD);
        XFORM[[5, 1]] = (f64::cos(THETA) * FIVDPD);
        XFORM[[6, 1]] = 0.0;

        XFORM[[1, 2]] = -f64::sin(THETA);
        XFORM[[2, 2]] = f64::cos(THETA);
        XFORM[[3, 2]] = 0.0;
        XFORM[[4, 2]] = -(f64::cos(THETA) * FIVDPD);
        XFORM[[5, 2]] = -(f64::sin(THETA) * FIVDPD);
        XFORM[[6, 2]] = 0.0;

        XFORM[[1, 3]] = 0.0;
        XFORM[[2, 3]] = 0.0;
        XFORM[[3, 3]] = 1.0;
        XFORM[[4, 3]] = 0.0;
        XFORM[[5, 3]] = 0.0;
        XFORM[[6, 3]] = 0.0;

        XFORM[[1, 4]] = 0.0;
        XFORM[[2, 4]] = 0.0;
        XFORM[[3, 4]] = 0.0;
        XFORM[[4, 4]] = f64::cos(THETA);
        XFORM[[5, 4]] = f64::sin(THETA);
        XFORM[[6, 4]] = 0.0;

        XFORM[[1, 5]] = 0.0;
        XFORM[[2, 5]] = 0.0;
        XFORM[[3, 5]] = 0.0;
        XFORM[[4, 5]] = -f64::sin(THETA);
        XFORM[[5, 5]] = f64::cos(THETA);
        XFORM[[6, 5]] = 0.0;

        XFORM[[1, 6]] = 0.0;
        XFORM[[2, 6]] = 0.0;
        XFORM[[3, 6]] = 0.0;
        XFORM[[4, 6]] = 0.0;
        XFORM[[5, 6]] = 0.0;
        XFORM[[6, 6]] = 1.0;

        spicelib::MXMG(
            XFORM.as_slice(),
            ROTFRM.as_slice(),
            6,
            6,
            6,
            TEMP.as_slice_mut(),
        );
        spicelib::MXMG(
            ROTTO.as_slice(),
            TEMP.as_slice(),
            6,
            6,
            6,
            XFORM.as_slice_mut(),
        );

        spicelib::CONICS(ELTS.as_slice(), ET, STATE1.as_slice_mut(), ctx)?;
        spicelib::MXVG(
            XFORM.as_slice(),
            STATE1.as_slice(),
            6,
            6,
            STATE.as_slice_mut(),
        );

        THETA = ((ET - T0) * TENDPD);

        XFORM[[1, 1]] = f64::cos(THETA);
        XFORM[[2, 1]] = f64::sin(THETA);
        XFORM[[3, 1]] = 0.0;
        XFORM[[4, 1]] = -(f64::sin(THETA) * TENDPD);
        XFORM[[5, 1]] = (f64::cos(THETA) * TENDPD);
        XFORM[[6, 1]] = 0.0;

        XFORM[[1, 2]] = -f64::sin(THETA);
        XFORM[[2, 2]] = f64::cos(THETA);
        XFORM[[3, 2]] = 0.0;
        XFORM[[4, 2]] = -(f64::cos(THETA) * TENDPD);
        XFORM[[5, 2]] = -(f64::sin(THETA) * TENDPD);
        XFORM[[6, 2]] = 0.0;

        XFORM[[1, 3]] = 0.0;
        XFORM[[2, 3]] = 0.0;
        XFORM[[3, 3]] = 1.0;
        XFORM[[4, 3]] = 0.0;
        XFORM[[5, 3]] = 0.0;
        XFORM[[6, 3]] = 0.0;

        XFORM[[1, 4]] = 0.0;
        XFORM[[2, 4]] = 0.0;
        XFORM[[3, 4]] = 0.0;
        XFORM[[4, 4]] = f64::cos(THETA);
        XFORM[[5, 4]] = f64::sin(THETA);
        XFORM[[6, 4]] = 0.0;

        XFORM[[1, 5]] = 0.0;
        XFORM[[2, 5]] = 0.0;
        XFORM[[3, 5]] = 0.0;
        XFORM[[4, 5]] = -f64::sin(THETA);
        XFORM[[5, 5]] = f64::cos(THETA);
        XFORM[[6, 5]] = 0.0;

        XFORM[[1, 6]] = 0.0;
        XFORM[[2, 6]] = 0.0;
        XFORM[[3, 6]] = 0.0;
        XFORM[[4, 6]] = 0.0;
        XFORM[[5, 6]] = 0.0;
        XFORM[[6, 6]] = 1.0;

        spicelib::MXVG(
            XFORM.as_slice(),
            STATE.as_slice(),
            6,
            6,
            STATE1.as_slice_mut(),
        );
        spicelib::MXV(TOINRT.as_slice(), STATE1.as_slice(), TMPSTA.as_slice_mut());
        spicelib::MXV(
            TOINRT.as_slice(),
            STATE1.subarray(4),
            TMPSTA.subarray_mut(4),
        );
        spicelib::MOVED(TMPSTA.as_slice(), 6, STATE1.as_slice_mut());

        spicelib::EQNCPV(
            ET,
            T0,
            EQEL.as_slice(),
            RAPOL,
            DECPOL,
            STATE2.as_slice_mut(),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"POSITION",
            STATE2.subarray(1),
            b"~/",
            STATE1.subarray(1),
            3,
            0.000000000005,
            OK,
            ctx,
        )?;
        testutil::CHCKAD(
            b"VELOCITY",
            STATE2.subarray(4),
            b"~/",
            STATE1.subarray(4),
            3,
            0.000000000005,
            OK,
            ctx,
        )?;
    }

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
