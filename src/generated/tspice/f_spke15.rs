//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const EPOCH: i32 = 1;
const TP: i32 = 2;
const PA: i32 = 3;
const P: i32 = 4;
const ECC: i32 = 5;
const TZERO: i32 = 6;
const J2FLG: i32 = 7;
const PV: i32 = 8;
const GM: i32 = 9;
const OJ2: i32 = 10;
const RPL: i32 = 11;
const TYPE15: i32 = 1;
const TYPE53: i32 = 2;

//$Procedure F_SPKE15  (  Test routine SPKE15  )
pub fn F_SPKE15(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ITEM = StackArray2D::<i32, 22>::new(1..=RPL, 1..=TYPE53);
    let mut MYJ2: f64 = 0.0;
    let mut MYP: f64 = 0.0;
    let mut MYGM: f64 = 0.0;
    let mut MYECC: f64 = 0.0;
    let mut RADIUS: f64 = 0.0;
    let mut EDPERI: f64 = 0.0;
    let mut ADPERI: f64 = 0.0;
    let mut RECORD = StackArray2D::<f64, 34>::new(1..=17, 1..=2);
    let mut STATE = StackArray::<f64, 6>::new(1..=6);
    let mut STATE2 = StackArray::<f64, 6>::new(1..=6);
    let mut ADNODE: f64 = 0.0;
    let mut EDNODE: f64 = 0.0;
    let mut ET: f64 = 0.0;
    let mut SMA: f64 = 0.0;
    let mut DMDT: f64 = 0.0;
    let mut PERIOD: f64 = 0.0;
    let mut ELTS0 = StackArray::<f64, 8>::new(1..=8);
    let mut ELTS1 = StackArray::<f64, 8>::new(1..=8);
    let mut ANGMO1 = StackArray::<f64, 3>::new(1..=3);
    let mut ANGMO2 = StackArray::<f64, 3>::new(1..=3);
    let mut POS = StackArray::<f64, 3>::new(1..=3);

    //
    // Spicelib Functions
    //

    //
    // Check out the error handling first.
    //

    //
    // Open the family of tests for SPKE15.
    //
    testutil::TOPEN(b"F_SPKE15", ctx)?;

    ITEM[[EPOCH, TYPE15]] = 1;
    ITEM[[TP, TYPE15]] = 2;
    ITEM[[PA, TYPE15]] = 5;
    ITEM[[P, TYPE15]] = 8;
    ITEM[[ECC, TYPE15]] = 9;
    ITEM[[J2FLG, TYPE15]] = 10;
    ITEM[[PV, TYPE15]] = 11;
    ITEM[[GM, TYPE15]] = 14;
    ITEM[[OJ2, TYPE15]] = 15;
    ITEM[[RPL, TYPE15]] = 16;
    ITEM[[TZERO, TYPE15]] = 17;

    ITEM[[EPOCH, TYPE53]] = 1;
    ITEM[[TP, TYPE53]] = 2;
    ITEM[[PA, TYPE53]] = 5;
    ITEM[[P, TYPE53]] = 8;
    ITEM[[ECC, TYPE53]] = 9;
    ITEM[[TZERO, TYPE53]] = 10;
    ITEM[[J2FLG, TYPE53]] = 11;
    ITEM[[PV, TYPE53]] = 12;
    ITEM[[GM, TYPE53]] = 15;
    ITEM[[OJ2, TYPE53]] = 16;
    ITEM[[RPL, TYPE53]] = 17;

    for I in 1..=17 {
        RECORD[[I, TYPE15]] = 0.0;
        RECORD[[I, TYPE53]] = 0.0;
    }

    ET = 7200.0;

    //
    // Here's the basic strategy for each test case.
    //
    //    We set up conditions in the input
    //    records that should yield exceptions.
    //
    //    We check for the exception
    //
    //    Finally set the exceptional item to
    //    something realistic so that future tests
    //    won't be tripped up by this exceptional
    //    value.
    //
    // Note: we set the values for both the type 53 record and
    // type 15 record at the same time so that they always
    // represent the same orbit.  We do this even when the type 53
    // record is not used.
    //

    //
    // The semi-latus rectum is supposed to be positive.  Start
    // out at zero and then set it to something reasonable.
    //
    testutil::TCASE(b"Semi-latus rectum exception", ctx)?;

    for I in TYPE15..=TYPE53 {
        RECORD[[ITEM[[P, I]], I]] = 0.0;
    }

    spicelib::SPKE15(ET, RECORD.subarray([1, TYPE15]), STATE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADLATUSRECTUM)", OK, ctx)?;

    for I in TYPE15..=TYPE53 {
        RECORD[[ITEM[[P, I]], I]] = 20000.0;
    }

    //
    // Negative eccentricities should produce exceptions.  After
    // checking that this is so set the eccentricity to something
    // yielding a periodic orbit.
    //
    testutil::TCASE(b"Eccentricity Exception", ctx)?;

    for I in TYPE15..=TYPE53 {
        RECORD[[ITEM[[ECC, I]], I]] = -1.0;
    }

    spicelib::SPKE15(ET, RECORD.subarray([1, TYPE15]), STATE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADECCENTRICITY)", OK, ctx)?;

    for I in TYPE15..=TYPE53 {
        RECORD[[ITEM[[ECC, I]], I]] = 0.1;
    }

    //
    // The central mass must be positive.  Zero or less should
    // trigger an exception. Try zero and -1.  After that we
    // use the mass of the earth.
    //
    testutil::TCASE(b"Central Mass Exception --- mass 0", ctx)?;

    for I in TYPE15..=TYPE53 {
        RECORD[[ITEM[[GM, I]], I]] = 0.0;
    }

    spicelib::SPKE15(ET, RECORD.subarray([1, TYPE15]), STATE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(NONPOSITIVEMASS)", OK, ctx)?;

    testutil::TCASE(b"Central Mass Exception --- mass -1", ctx)?;

    for I in TYPE15..=TYPE53 {
        RECORD[[ITEM[[GM, I]], I]] = -1.0;
    }

    spicelib::SPKE15(ET, RECORD.subarray([1, TYPE15]), STATE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(NONPOSITIVEMASS)", OK, ctx)?;

    for I in TYPE15..=TYPE53 {
        RECORD[[ITEM[[GM, I]], I]] = 398600.44770326116;
    }

    //
    // Only a zero trajectory pole can produce a problem.  By
    // construction we already have one.
    //
    testutil::TCASE(b"Trajectory Pole Exception", ctx)?;

    spicelib::SPKE15(ET, RECORD.subarray([1, TYPE15]), STATE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADVECTOR)", OK, ctx)?;

    //
    // Set the trajectory pole to 45 degree inclination
    //
    for I in TYPE15..=TYPE53 {
        RECORD[[(ITEM[[TP, I]] + 1), I]] = f64::cos((spicelib::PI(ctx) / 4.0));
        RECORD[[(ITEM[[TP, I]] + 2), I]] = f64::sin((spicelib::PI(ctx) / 4.0));
    }

    //
    // Only a zero periapsis vector yields an exception.  We
    // already have this by construction.  After testing make
    // a periapsis vector that is orthogonal to the trajectory
    // pole vector.
    //
    testutil::TCASE(b"Periapsis Vector Exception", ctx)?;

    spicelib::SPKE15(ET, RECORD.subarray([1, TYPE15]), STATE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADVECTOR)", OK, ctx)?;

    for I in TYPE15..=TYPE53 {
        RECORD[[(ITEM[[PA, I]] + 1), I]] = f64::sin((spicelib::PI(ctx) / 4.0));
        RECORD[[(ITEM[[PA, I]] + 2), I]] = -f64::cos((spicelib::PI(ctx) / 4.0));
    }

    //
    // Only a zero central body pole vector can yield an exception.
    // We have such a situation by construction.  After checking
    // this, align the pole with the Z axis.
    //
    testutil::TCASE(b"Pole Vector Exception", ctx)?;

    spicelib::SPKE15(ET, RECORD.subarray([1, TYPE15]), STATE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADVECTOR)", OK, ctx)?;

    for I in TYPE15..=TYPE53 {
        RECORD[[(ITEM[[PV, I]] + 2), I]] = 1.0;
    }

    //
    // Anything less than zero should trigger an exception.  After
    // checking, set the equatorial radius to that of the earth.
    //
    testutil::TCASE(b"Equatorial Radius Exception", ctx)?;

    for I in TYPE15..=TYPE53 {
        RECORD[[ITEM[[RPL, I]], I]] = -1.0;
    }

    spicelib::SPKE15(ET, RECORD.subarray([1, TYPE15]), STATE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADRADIUS)", OK, ctx)?;

    for I in TYPE15..=TYPE53 {
        RECORD[[ITEM[[RPL, I]], I]] = 6378.184;
    }

    //
    // If the periapse is not nearly perpendicular to the
    // trajectory pole, we should get an exception.  Create
    // a vector that isn't perpendicular to the trajectory pole
    // by messing up the sign on the z-component.
    //
    testutil::TCASE(b"Bad Initial Conditions", ctx)?;

    for I in TYPE15..=TYPE53 {
        RECORD[[(ITEM[[PA, I]] + 1), I]] = 1.0;
        RECORD[[(ITEM[[PA, I]] + 2), I]] = 0.0;
    }

    spicelib::SPKE15(ET, RECORD.subarray([1, TYPE15]), STATE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADINITSTATE)", OK, ctx)?;

    for I in TYPE15..=TYPE53 {
        RECORD[[(ITEM[[PA, I]] + 1), I]] = f64::sin((spicelib::PI(ctx) / 4.0));
        RECORD[[(ITEM[[PA, I]] + 2), I]] = -f64::cos((spicelib::PI(ctx) / 4.0));
    }

    //
    // That takes care of all the exception tests.  Next see if
    // we get the same results with the two different implementations.
    //
    // Check to make sure that type 53 and type 15 agree
    // when J2 is zero.
    //
    testutil::TCASE(b"Elliptic Orbit, J2 = 0", ctx)?;

    for I in TYPE15..=TYPE53 {
        RECORD[[ITEM[[OJ2, I]], I]] = 0.0;
    }

    SPKE53(ET, RECORD.subarray([1, TYPE53]), STATE2.as_slice_mut(), ctx)?;
    spicelib::SPKE15(ET, RECORD.subarray([1, TYPE15]), STATE.as_slice_mut(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"~/",
        STATE2.as_slice(),
        6,
        0.000000000001,
        OK,
        ctx,
    )?;

    //
    // Check to make sure that type 53 and type 15 agree for
    // J2 non-zero.  More specifically when J2 is that of the
    // earth.
    //
    testutil::TCASE(b"Eliptic Orbit, J2 of Earth", ctx)?;

    for I in TYPE15..=TYPE53 {
        RECORD[[ITEM[[OJ2, I]], I]] = 0.001082616;
        RECORD[[ITEM[[J2FLG, I]], I]] = 0.0;
    }

    SPKE53(ET, RECORD.subarray([1, TYPE53]), STATE.as_slice_mut(), ctx)?;
    spicelib::SPKE15(ET, RECORD.subarray([1, TYPE15]), STATE2.as_slice_mut(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"~/",
        STATE2.as_slice(),
        6,
        0.0000000000001,
        OK,
        ctx,
    )?;

    //
    // Check that the precession is what is predicted from the
    // equations.  We use oscelt to determine how far the node
    // moves in one orbit.
    //
    testutil::TCASE(b"Check Precession", ctx)?;

    MYP = RECORD[[ITEM[[P, TYPE15]], TYPE15]];
    MYGM = RECORD[[ITEM[[GM, TYPE15]], TYPE15]];
    MYECC = RECORD[[ITEM[[ECC, TYPE15]], TYPE15]];
    MYJ2 = RECORD[[ITEM[[OJ2, TYPE15]], TYPE15]];
    RADIUS = RECORD[[ITEM[[RPL, TYPE15]], TYPE15]];

    SMA = (MYP / ((1 as f64) - f64::powi(MYECC, 2)));
    DMDT = f64::sqrt((MYGM / f64::powi(SMA, 3)));
    PERIOD = (spicelib::TWOPI(ctx) / DMDT);

    spicelib::SPKE15(ET, RECORD.subarray([1, TYPE15]), STATE.as_slice_mut(), ctx)?;
    spicelib::OSCELT(STATE.as_slice(), ET, MYGM, ELTS0.as_slice_mut(), ctx)?;

    ET = (ET + PERIOD);

    spicelib::SPKE15(ET, RECORD.subarray([1, TYPE15]), STATE2.as_slice_mut(), ctx)?;
    spicelib::OSCELT(STATE2.as_slice(), ET, MYGM, ELTS1.as_slice_mut(), ctx)?;

    //
    // ELTS        are equivalent conic elements describing the orbit
    //             of the body around its primary. The elements are,
    //             in order:
    //
    //                   RP      Perifocal distance.
    //                   ECC     Eccentricity.
    //                   INC     Inclination.
    //                   LNODE   Longitude of the ascending node.
    //                   ARGP    Argument of periapsis.
    //                   M0      Mean anomaly at epoch.
    //                   T0      Epoch.
    //                   MU      Gravitational parameter.
    //

    EDNODE = -((((spicelib::TWOPI(ctx) * 1.5) * MYJ2) * f64::cos(ELTS0[3]))
        * f64::powi((RADIUS / MYP), 2));
    ADNODE = (ELTS1[4] - ELTS0[4]);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"DNODE", ADNODE, b"~/", EDNODE, 0.00000000001, OK, ctx)?;

    //
    // Next Check to see that periapse has moved by the "right"
    // amount.
    //
    EDPERI = ((((spicelib::TWOPI(ctx) * 1.5) * MYJ2)
        * ((2.5 * f64::powi(f64::cos(ELTS0[3]), 2)) - 0.5))
        * f64::powi((RADIUS / MYP), 2));
    ADPERI = (ELTS1[5] - ELTS0[5]);

    testutil::CHCKSD(b"DPERI", ADPERI, b"~/", EDPERI, 0.0000000001, OK, ctx)?;

    //
    // See that we get the same results for the hyperbolic case.
    //
    testutil::TCASE(b"Hyperbolic Orbit", ctx)?;

    for I in TYPE15..=TYPE53 {
        RECORD[[ITEM[[ECC, I]], I]] = 2.0;
    }

    ET = 7200.0;

    SPKE53(ET, RECORD.subarray([1, TYPE53]), STATE.as_slice_mut(), ctx)?;
    spicelib::SPKE15(ET, RECORD.subarray([1, TYPE15]), STATE2.as_slice_mut(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"~/",
        STATE2.as_slice(),
        6,
        0.000000000001,
        OK,
        ctx,
    )?;

    //
    // Make sure the angular momentum is constant.
    //
    testutil::TCASE(b"Hyperbolic Angular Momentum", ctx)?;

    RECORD[[ITEM[[ECC, TYPE15]], TYPE15]] = 2.0;

    ET = 0.0;

    spicelib::SPKE15(ET, RECORD.subarray([1, TYPE15]), STATE.as_slice_mut(), ctx)?;
    spicelib::VCRSS(STATE.as_slice(), STATE.subarray(4), ANGMO1.as_slice_mut());

    ET = (ET + 72000.0);

    spicelib::SPKE15(ET, RECORD.subarray([1, TYPE15]), STATE.as_slice_mut(), ctx)?;
    spicelib::VCRSS(STATE.as_slice(), STATE.subarray(4), ANGMO2.as_slice_mut());

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"ANGMO",
        ANGMO1.as_slice(),
        b"~~/",
        ANGMO2.as_slice(),
        3,
        0.000000000001,
        OK,
        ctx,
    )?;

    //
    // In this case we simply check that adding the velocity times
    // a delta time gives approximately the position.
    //
    testutil::TCASE(b"Discrete Hyperbolic Integration", ctx)?;

    ET = 0.0;

    spicelib::SPKE15(ET, RECORD.subarray([1, TYPE15]), STATE.as_slice_mut(), ctx)?;
    spicelib::SPKE15(
        (ET + 1.0),
        RECORD.subarray([1, TYPE15]),
        STATE2.as_slice_mut(),
        ctx,
    )?;

    spicelib::VADD(STATE.subarray(1), STATE.subarray(4), POS.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"Position",
        POS.as_slice(),
        b"~/",
        STATE2.as_slice(),
        3,
        0.0001,
        OK,
        ctx,
    )?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
