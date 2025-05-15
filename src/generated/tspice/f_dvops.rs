//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const TIGHT: f64 = 0.0000000000001;

//$Procedure F_DVOPS (Family of vector operations derivative tests)
pub fn F_DVOPS(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut CMP: f64 = 0.0;
    let mut EXP: f64 = 0.0;
    let mut LENGTH: f64 = 0.0;
    let mut MAG_LOG: f64 = 0.0;
    let mut MAG: f64 = 0.0;
    let mut LLIM: f64 = 0.0;
    let mut ULIM: f64 = 0.0;
    let mut DVMAG: f64 = 0.0;
    let mut VTEMP = StackArray::<f64, 3>::new(1..=3);
    let mut X1 = StackArray::<f64, 3>::new(1..=3);
    let mut PRT1 = StackArray::<f64, 3>::new(1..=3);
    let mut PRT2 = StackArray::<f64, 3>::new(1..=3);
    let mut EXPV = StackArray::<f64, 6>::new(1..=6);
    let mut S1 = StackArray::<f64, 6>::new(1..=6);
    let mut S2 = StackArray::<f64, 6>::new(1..=6);
    let mut SOUT = StackArray::<f64, 6>::new(1..=6);
    let mut TXT = [b' '; 64 as usize];
    let mut SEED1: i32 = 0;

    //
    // Test Utility Functions
    //

    //
    // SPICELIB Functions
    //

    //
    // Local parameters
    //

    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_DVOPS", ctx)?;

    S1[1] = 1.0;
    S1[2] = 2.0;
    S1[3] = 1.0;
    S1[4] = 1.0;
    S1[5] = -1.0;
    S1[6] = 3.0;

    S2[1] = 2.0;
    S2[2] = 3.0;
    S2[3] = 4.0;
    S2[4] = 1.0;
    S2[5] = 2.0;
    S2[6] = 3.0;

    //
    // Case 1
    //
    testutil::TCASE(b"Test DVDOT.", ctx)?;

    EXP = ((((((S1[1] * S2[4]) + (S1[4] * S2[1])) + (S1[2] * S2[5])) + (S1[5] * S2[2]))
        + (S1[3] * S2[6]))
        + (S1[6] * S2[3]));

    CMP = spicelib::DVDOT(S1.as_slice(), S2.as_slice());

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"DVDOT", CMP, b"=", EXP, 0.0, OK, ctx)?;

    //
    // Case 2
    //
    testutil::TCASE(b"Test DVCRSS.", ctx)?;

    spicelib::VCRSS(S1.subarray(1), S2.subarray(1), EXPV.subarray_mut(1));
    spicelib::VCRSS(S1.subarray(1), S2.subarray(4), PRT1.subarray_mut(1));
    spicelib::VCRSS(S1.subarray(4), S2.subarray(1), PRT2.subarray_mut(1));
    spicelib::VADD(PRT1.subarray(1), PRT2.subarray(1), EXPV.subarray_mut(4));

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DVCRSS(S1.as_slice(), S2.as_slice(), SOUT.as_slice_mut());

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"SOUT",
        SOUT.as_slice(),
        b"=",
        EXPV.as_slice(),
        6,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Case 3
    //
    testutil::TCASE(b"Test DVHAT with a state having non-zero position.", ctx)?;

    spicelib::UNORM(S1.as_slice(), EXPV.as_slice_mut(), &mut LENGTH);
    spicelib::VPERP(
        S1.subarray(4),
        &EXPV.as_slice().to_vec(),
        EXPV.subarray_mut(4),
    );
    spicelib::VSCLIP((1.0 / LENGTH), EXPV.subarray_mut(4));

    spicelib::DVHAT(S1.as_slice(), SOUT.as_slice_mut());

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"SOUT",
        SOUT.as_slice(),
        b"=",
        EXPV.as_slice(),
        6,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Case 4
    //
    testutil::TCASE(b"Test DVHAT with a state having zero position.", ctx)?;

    S1[1] = 0.0;
    S1[2] = 0.0;
    S1[3] = 0.0;

    EXPV[1] = 0.0;
    EXPV[2] = 0.0;
    EXPV[3] = 0.0;
    EXPV[4] = S1[4];
    EXPV[5] = S1[5];
    EXPV[6] = S1[6];

    spicelib::DVHAT(S1.as_slice(), SOUT.as_slice_mut());

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"SOUT",
        SOUT.as_slice(),
        b"=",
        EXPV.as_slice(),
        6,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Case 5
    //
    testutil::TCASE(b"Test DUCRSS", ctx)?;

    S1[1] = 1.0;
    S1[2] = 2.0;
    S1[3] = 1.0;

    spicelib::VCRSS(S1.subarray(1), S2.subarray(1), EXPV.subarray_mut(1));
    spicelib::VCRSS(S1.subarray(1), S2.subarray(4), PRT1.subarray_mut(1));
    spicelib::VCRSS(S1.subarray(4), S2.subarray(1), PRT2.subarray_mut(1));
    spicelib::VADD(PRT1.subarray(1), PRT2.subarray(1), EXPV.subarray_mut(4));
    spicelib::UNORM(EXPV.as_slice(), VTEMP.as_slice_mut(), &mut LENGTH);
    spicelib::VEQU(VTEMP.as_slice(), EXPV.as_slice_mut());
    spicelib::VPERP(EXPV.subarray(4), EXPV.subarray(1), VTEMP.as_slice_mut());
    spicelib::VSCL((1.0 / LENGTH), VTEMP.as_slice(), EXPV.subarray_mut(4));

    spicelib::DUCRSS(S1.as_slice(), S2.as_slice(), SOUT.as_slice_mut());

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"SOUT",
        SOUT.as_slice(),
        b"=",
        EXPV.as_slice(),
        6,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Case 6
    //
    // Test DVNORM, parallel, anti-parallel, zero vector cases.
    //

    SEED1 = -82763653;
    LLIM = -2.0;
    ULIM = 25.0;

    X1[1] = 1.0;
    X1[2] = f64::sqrt(2.0);
    X1[3] = f64::sqrt(3.0);

    fstr::assign(&mut TXT, b"DVNORM ZERO VECTOR CASE");
    testutil::TCASE(&TXT, ctx)?;

    S1[1] = 0.0;
    S1[2] = 0.0;
    S1[3] = 0.0;
    S1[4] = X1[1];
    S1[5] = X1[2];
    S1[6] = X1[3];

    DVMAG = spicelib::DVNORM(S1.as_slice());

    testutil::CHCKSD(b"DVNORM", DVMAG, b"=", 0.0, TIGHT, OK, ctx)?;

    //
    // Create a set of 6x1 arrays with, the first three components
    // describing a vector with a random magnitude.
    //
    //
    // Note, 5000 is an arbitrary value.
    //
    for I in 1..=5000 {
        MAG_LOG = testutil::T_RANDD(LLIM, ULIM, &mut SEED1, ctx)?;
        MAG = f64::powf(10.0, MAG_LOG);

        fstr::assign(&mut TXT, b"DVNORM parallel %d, MAG_LOG %f");
        spicelib::REPMI(&TXT.clone(), b"%d", I, &mut TXT, ctx);
        spicelib::REPMD(&TXT.clone(), b"%f", MAG_LOG, 6, &mut TXT, ctx);

        testutil::TCASE(&TXT, ctx)?;

        S1[1] = (X1[1] * MAG);
        S1[2] = (X1[2] * MAG);
        S1[3] = (X1[3] * MAG);
        S1[4] = X1[1];
        S1[5] = X1[2];
        S1[6] = X1[3];

        DVMAG = spicelib::DVNORM(S1.as_slice());

        testutil::CHCKSD(
            &TXT,
            DVMAG,
            b"~",
            spicelib::VNORM(X1.as_slice()),
            TIGHT,
            OK,
            ctx,
        )?;

        MAG_LOG = testutil::T_RANDD(LLIM, ULIM, &mut SEED1, ctx)?;
        MAG = f64::powf(10.0, MAG_LOG);

        fstr::assign(&mut TXT, b"DVNORM anti-parallel %d, MAG_LOG %f");
        spicelib::REPMI(&TXT.clone(), b"%d", I, &mut TXT, ctx);
        spicelib::REPMD(&TXT.clone(), b"%f", MAG_LOG, 6, &mut TXT, ctx);

        testutil::TCASE(&TXT, ctx)?;

        S1[1] = (X1[1] * MAG);
        S1[2] = (X1[2] * MAG);
        S1[3] = (X1[3] * MAG);
        S1[4] = -X1[1];
        S1[5] = -X1[2];
        S1[6] = -X1[3];

        DVMAG = spicelib::DVNORM(S1.as_slice());

        testutil::CHCKSD(
            &TXT,
            DVMAG,
            b"~",
            -spicelib::VNORM(X1.as_slice()),
            TIGHT,
            OK,
            ctx,
        )?;

        MAG_LOG = testutil::T_RANDD(LLIM, ULIM, &mut SEED1, ctx)?;
        MAG = f64::powf(10.0, MAG_LOG);

        fstr::assign(&mut TXT, b"DVNORM orthogonal %d, MAG_LOG %f");
        spicelib::REPMI(&TXT.clone(), b"%d", I, &mut TXT, ctx);
        spicelib::REPMD(&TXT.clone(), b"%f", MAG_LOG, 6, &mut TXT, ctx);

        testutil::TCASE(&TXT, ctx)?;

        //
        // Create a 6x1 array with the property the dot product of the
        // first three components with the second three components
        // has value zero.
        //
        S2[1] = (X1[1] * MAG);
        S2[2] = (X1[2] * MAG);
        S2[3] = (X1[3] * MAG);
        S2[4] = (1.0 / S2[1]);
        S2[5] = (1.0 / S2[2]);
        S2[6] = -(2.0 / S2[3]);

        DVMAG = spicelib::DVNORM(S2.as_slice());

        testutil::CHCKSD(&TXT, DVMAG, b"~", 0.0, TIGHT, OK, ctx)?;
    }

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
