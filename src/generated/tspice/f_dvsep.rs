//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const TIGHT: f64 = 0.000000000001;
const PCK: &[u8] = b"dvsep.pck";

//$Procedure F_DVSEP ( DVSEP family tests )
pub fn F_DVSEP(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut VAL: f64 = 0.0;
    let mut COLAT: f64 = 0.0;
    let mut LON: f64 = 0.0;
    let mut MAG_LOG: f64 = 0.0;
    let mut MAG: f64 = 0.0;
    let mut VEL = StackArray::<f64, 3>::new(1..=3);
    let mut REC1 = StackArray::<f64, 3>::new(1..=3);
    let mut REC2 = StackArray::<f64, 3>::new(1..=3);
    let mut CRSS1 = StackArray::<f64, 3>::new(1..=3);
    let mut CRSS2 = StackArray::<f64, 3>::new(1..=3);
    let mut S1 = StackArray::<f64, 6>::new(1..=6);
    let mut S2 = StackArray::<f64, 6>::new(1..=6);
    let mut S1_T = StackArray::<f64, 6>::new(1..=6);
    let mut S2_T = StackArray::<f64, 6>::new(1..=6);
    let mut TRANS = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut SEED1: i32 = 0;
    let mut SEED2: i32 = 0;
    let mut TXT = [b' '; 128 as usize];

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_DVSEP", ctx)?;

    //
    // Create a PCK, load using FURNSH.
    //
    testutil::T_PCK08(PCK, false, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(PCK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Case 1
    //
    testutil::TCASE(b"DVSEP overflow.", ctx)?;

    //
    // Two state vectors, one rather odd.
    //
    S1[1] = 3.0;
    S1[2] = 4.0;
    S1[3] = 0.0;
    S1[4] = -9.0;
    S1[5] = 2.0;
    S1[6] = 0.1;

    //
    // S2 has a very high derivative of the unit vector.
    //
    S2[1] = 1.0;
    S2[2] = 0.0;
    S2[3] = 1.0;
    S2[4] = 1.0;
    S2[5] = (0.9 * spicelib::DPMAX());
    S2[6] = 0.0;

    VAL = spicelib::DVSEP(S1.as_slice(), S2.as_slice(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(NUMERICOVERFLOW)", OK, ctx)?;

    //
    // Case 2
    //
    testutil::TCASE(b"DVSEP S1xS2 = 0", ctx)?;

    S1[1] = 25.0;
    S1[2] = 0.0;
    S1[3] = 0.0;
    S1[4] = 2.0;
    S1[5] = -1.0;
    S1[6] = 1.0;

    S2[1] = -S1[1];
    S2[2] = -S1[2];
    S2[3] = -S1[3];
    S2[4] = -S1[4];
    S2[5] = -S1[5];
    S2[6] = -S1[6];

    VAL = spicelib::DVSEP(S1.as_slice(), S2.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Expect a separation angle rate of zero.
    //
    testutil::CHCKSD(b"DVSEP S1xS2 = 0 (1)", VAL, b"=", 0.0, TIGHT, OK, ctx)?;

    //
    // Case 3
    //

    //
    // Create two random vectors in R3, REC1 and REC2, each with
    // magnitude MAG. Use the cross product operations
    //
    //    CRSS1 = REC1  x REC2
    //    CRSS2 = CRSS1 x REC1
    //
    // to construct a velocity unit vector orthogonal to REC1 in the
    // plane of REC1 and REC2 oriented such that REC1 rotates towards
    // REC2 with REC2 constant.
    //
    // In this case
    //
    //        d(theta)
    //    v = -------- * || REC1 ||
    //        dt
    //
    // with v = 1, then
    //
    //    d(theta)     -1           -1
    //    -------- =  ______    =   ---
    //    dt
    //              || REC1 ||      MAG
    //

    SEED1 = -82763653;
    SEED2 = -273661;

    //
    // Define an arbitrary rotation from J2000 to something
    // not referenced against earth rotation at an arbitrary
    // ephemeris time, ET = 0 in this case.
    //
    spicelib::SXFORM(b"J2000", b"IAU_MOON", 0.0, TRANS.as_slice_mut(), ctx)?;

    //
    // Note, 5000 is also an arbitrary value.
    //
    for I in 1..=5000 {
        fstr::assign(&mut TXT, b"S1, S2 random test #, MAG_LOG #");

        spicelib::REPMI(&TXT.clone(), b"#", I, &mut TXT, ctx);

        MAG_LOG = testutil::T_RANDD(-2.0, 25.0, &mut SEED1, ctx)?;
        MAG = f64::powf(10 as f64, MAG_LOG);

        spicelib::REPMD(&TXT.clone(), b"#", MAG_LOG, 8, &mut TXT, ctx);

        testutil::TCASE(&TXT, ctx)?;

        COLAT = testutil::T_RANDD(0.0, spicelib::PI(ctx), &mut SEED1, ctx)?;
        LON = testutil::T_RANDD(0.0, spicelib::TWOPI(ctx), &mut SEED2, ctx)?;

        spicelib::SPHREC(MAG, COLAT, LON, REC1.as_slice_mut());

        COLAT = testutil::T_RANDD(0.0, spicelib::PI(ctx), &mut SEED1, ctx)?;
        LON = testutil::T_RANDD(0.0, spicelib::TWOPI(ctx), &mut SEED2, ctx)?;

        spicelib::SPHREC(MAG, COLAT, LON, REC2.as_slice_mut());

        spicelib::VCRSS(REC1.as_slice(), REC2.as_slice(), CRSS1.as_slice_mut());
        spicelib::VCRSS(CRSS1.as_slice(), REC1.as_slice(), CRSS2.as_slice_mut());

        spicelib::VHAT(CRSS2.as_slice(), VEL.as_slice_mut());

        spicelib::VPACK(REC1[1], REC1[2], REC1[3], S1.subarray_mut(1));
        spicelib::VPACK(VEL[1], VEL[2], VEL[3], S1.subarray_mut(4));

        spicelib::VPACK(REC2[1], REC2[2], REC2[3], S2.subarray_mut(1));
        spicelib::VPACK(0.0, 0.0, 0.0, S2.subarray_mut(4));

        //
        // Apply the transformation, TRANS, to S1 and S2, creating new
        // vectors without or with fewer zero components, particularly
        // the S2 velocity components.
        //
        spicelib::MXVG(TRANS.as_slice(), S1.as_slice(), 6, 6, S1_T.as_slice_mut());
        spicelib::MXVG(TRANS.as_slice(), S2.as_slice(), 6, 6, S2_T.as_slice_mut());

        VAL = spicelib::DVSEP(S1_T.as_slice(), S2_T.as_slice(), ctx)?;

        testutil::CHCKSD(&TXT, VAL, b"~", -(1.0 / MAG), TIGHT, OK, ctx)?;
    }

    //
    // Case n
    //
    testutil::TCASE(b"Clean up:  delete kernels.", ctx)?;

    spicelib::KCLEAR(ctx)?;

    spicelib::DELFIL(PCK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
