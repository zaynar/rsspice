//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ONAMES: i32 = 20;
const NNAMES: i32 = 21;
const WDSIZE: i32 = 32;
const BAD: i32 = -1;
const MAXNAM: i32 = 100;
const LBCELL: i32 = -5;

//$Procedure      F_CHGIRF (Family of tests for CHGIRF )
pub fn F_CHGIRF(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut NEWNAM = ActualCharArray::new(WDSIZE, 1..=20);
    let mut NAME = [b' '; WDSIZE as usize];
    let mut EXPCT = [b' '; WDSIZE as usize];
    let mut NEW = ActualCharArray::new(WDSIZE, LBCELL..=MAXNAM);
    let mut MATRIX = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut ROTATE = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut QUOTNT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut XFORM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut ANGLE: f64 = 0.0;
    let mut A = StackArray::<f64, 3>::new(1..=3);
    let mut AXIS = StackArray::<f64, 3>::new(1..=3);
    let mut ROTT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut ROT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut COUNT: i32 = 0;
    let mut I: i32 = 0;
    let mut ID: i32 = 0;
    let mut J: i32 = 0;
    let mut K: i32 = 0;

    //
    // The parameter ONAMES is the number of recognized reference
    // frames that were present in the previous version of CHGIRF
    //
    //
    // The parameter NNAMES is the number of recognized reference
    // frames that are present in the updated version of CHGIRF
    // that you are attempting to test.
    //
    //
    // The array NEWNAM contains the names of the reference frames
    // that are in the updated version of CHGIRF that were not
    // in the previous version of CHGIRF.
    //
    // Set the values of the entries of this array in the first block
    // of executable code below.
    //

    //
    // Bad is an ID code for a frame that should not be recognized
    // by CHGIRF.
    //

    //
    // Test Utility Functions
    //

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // The names that are new to CHGIRF should be placed here.
    //
    fstr::assign(NEWNAM.get_mut(1), b"DE-143");

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_CHGIRF", ctx)?;

    testutil::TCASE(b"Comparison of old and new transformations.", ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = ONAMES;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = ONAMES;
                let m3__: i32 = 1;
                J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    testutil::TSTMSG(b"#", b"Subcase I = #, J = #", ctx);
                    testutil::TSTMSI(I, ctx);
                    testutil::TSTMSI(J, ctx);

                    spicelib::IRFROT(I, J, ROTATE.as_slice_mut(), ctx)?;
                    IRFROT_O(I, J, MATRIX.as_slice_mut(), ctx)?;
                    spicelib::MTXM(MATRIX.as_slice(), ROTATE.as_slice(), QUOTNT.as_slice_mut());
                    spicelib::RAXISA(QUOTNT.as_slice(), AXIS.as_slice_mut(), &mut ANGLE, ctx)?;

                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    testutil::CHCKAD(
                        b"ROTATE",
                        ROTATE.as_slice(),
                        b"~",
                        MATRIX.as_slice(),
                        9,
                        0.000000000000001,
                        OK,
                        ctx,
                    )?;
                    testutil::CHCKSD(b"ANGLE", ANGLE, b"~", 0.0, 0.000000000000001, OK, ctx)?;

                    J += m3__;
                }
            }
            I += m3__;
        }
    }

    testutil::TCASE(b"Converting from id to name and back.", ctx)?;

    spicelib::SSIZEC(MAXNAM, NEW.as_arg_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NNAMES;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            J = I;
            spicelib::IRFNAM(J, &mut NAME, ctx)?;
            spicelib::IRFNUM(&NAME, &mut K, ctx);
            spicelib::INSRTC(&NAME, NEW.as_arg_mut(), ctx)?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSI(b"J", J, b"=", K, 0, OK, ctx)?;

            I += m3__;
        }
    }

    COUNT = spicelib::CARDC(NEW.as_arg(), ctx)?;

    testutil::CHCKSI(b"NameCount", COUNT, b"=", NNAMES, 0, OK, ctx)?;

    testutil::TCASE(b"Converting old names to ID\'s.", ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = ONAMES;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            J = I;
            IRFNAM_O(J, &mut NAME, ctx)?;
            spicelib::IRFNUM(&NAME, &mut K, ctx);

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSI(b"ID Code", K, b"=", J, 0, OK, ctx)?;

            I += m3__;
        }
    }

    testutil::TCASE(b"Comparing new and old names.", ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = ONAMES;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            J = I;
            spicelib::IRFNAM(J, &mut NAME, ctx)?;
            IRFNAM_O(J, &mut EXPCT, ctx)?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSC(b"NAME", &NAME, b"=", &EXPCT, OK, ctx)?;
            I += m3__;
        }
    }

    testutil::TCASE(b"Checking new names. ", ctx)?;
    //
    // To update this case simply put the new names in the array below
    // in the order of their id-codes.
    //
    K = 0;

    {
        let m1__: i32 = (ONAMES + 1);
        let m2__: i32 = NNAMES;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            J = I;
            K = (K + 1);
            testutil::TSTMSG(b"#", b"Subcase ID = #, NEWNAM = #", ctx);
            testutil::TSTMSI(J, ctx);
            testutil::TSTMSC(&NEWNAM[K], ctx);

            spicelib::IRFNAM(J, &mut NAME, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSC(b"NAME", &NAME, b"=", &NEWNAM[K], OK, ctx)?;

            I += m3__;
        }
    }

    testutil::TCASE(b"Unknown ID-code", ctx)?;

    spicelib::IRFNAM(BAD, &mut NAME, ctx)?;
    testutil::TSTMSG(b"#", b" ", ctx);
    testutil::CHCKSC(b"NAME", &NAME, b"=", b" ", OK, ctx)?;

    testutil::TCASE(b"Unknown Name", ctx)?;

    spicelib::IRFNUM(b"BOGUS", &mut ID, ctx);
    testutil::CHCKSI(b"ID", ID, b"=", 0, 0, OK, ctx)?;

    testutil::TCASE(b"DE-140 check", ctx)?;

    XFORM[[1, 1]] = 0.9999256765384668;
    XFORM[[2, 1]] = -0.0111817701797229;
    XFORM[[3, 1]] = -0.004858952020483;

    XFORM[[1, 2]] = 0.0111817701197967;
    XFORM[[2, 2]] = 0.9999374816848701;
    XFORM[[3, 2]] = -0.0000271791849815;

    XFORM[[1, 3]] = 0.0048589521583895;
    XFORM[[2, 3]] = -0.0000271545195858;
    XFORM[[3, 3]] = 0.9999881948535965;

    spicelib::IRFNUM(b"J2000", &mut J, ctx);
    spicelib::IRFNUM(b"DE-140", &mut I, ctx);
    spicelib::IRFROT(J, I, ROTT.as_slice_mut(), ctx)?;

    spicelib::MXMT(XFORM.as_slice(), ROTT.as_slice(), ROT.as_slice_mut());
    spicelib::RAXISA(ROT.as_slice(), A.as_slice_mut(), &mut ANGLE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"ROTATE",
        ROTT.as_slice(),
        b"~",
        XFORM.as_slice(),
        9,
        0.00000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"ANGLE", ANGLE, b"~", 0.0, 0.0000000000001, OK, ctx)?;

    testutil::TCASE(b"DE-142 check", ctx)?;

    XFORM[[1, 1]] = 0.9999256765402605;
    XFORM[[2, 1]] = -0.0111817697907755;
    XFORM[[3, 1]] = -0.0048589525464121;

    XFORM[[1, 2]] = 0.0111817697320531;
    XFORM[[2, 2]] = 0.9999374816892126;
    XFORM[[3, 2]] = -0.0000271789392288;

    XFORM[[1, 3]] = 0.0048589526815484;
    XFORM[[2, 3]] = -0.000027154769317;
    XFORM[[3, 3]] = 0.9999881948510477;

    spicelib::IRFNUM(b"J2000", &mut J, ctx);
    spicelib::IRFNUM(b"DE-142", &mut I, ctx);
    spicelib::IRFROT(J, I, ROTT.as_slice_mut(), ctx)?;

    spicelib::MXMT(XFORM.as_slice(), ROTT.as_slice(), ROT.as_slice_mut());
    spicelib::RAXISA(ROT.as_slice(), A.as_slice_mut(), &mut ANGLE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"ROTATE",
        ROTT.as_slice(),
        b"~",
        XFORM.as_slice(),
        9,
        0.00000000000002,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"ANGLE", ANGLE, b"~", 0.0, 0.0000000000001, OK, ctx)?;

    testutil::TCASE(b"DE-143 check", ctx)?;

    XFORM[[1, 1]] = 0.9999256765435852;
    XFORM[[2, 1]] = -0.0111817743300355;
    XFORM[[3, 1]] = -0.0048589414161348;

    XFORM[[1, 2]] = 0.0111817743077255;
    XFORM[[2, 2]] = 0.9999374816382505;
    XFORM[[3, 2]] = -0.0000271713942366;

    XFORM[[1, 3]] = 0.0048589414674762;
    XFORM[[2, 3]] = -0.0000271622115251;
    XFORM[[3, 3]] = 0.9999881949053349;

    spicelib::IRFNUM(b"J2000", &mut J, ctx);
    spicelib::IRFNUM(b"DE-143", &mut I, ctx);
    spicelib::IRFROT(J, I, ROTT.as_slice_mut(), ctx)?;

    spicelib::MXMT(XFORM.as_slice(), ROTT.as_slice(), ROT.as_slice_mut());
    spicelib::RAXISA(ROT.as_slice(), A.as_slice_mut(), &mut ANGLE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"ROTATE",
        ROTT.as_slice(),
        b"~",
        XFORM.as_slice(),
        9,
        0.00000000000002,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"ANGLE", ANGLE, b"~", 0.0, 0.0000000000001, OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
