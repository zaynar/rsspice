//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;

//$Procedure      F_TSTCK3 ( Family of tests for tstck3 and tstatd )
pub fn F_TSTCK3(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ERROR = [b' '; LNSIZE as usize];
    let mut ANGLE: f64 = 0.0;
    let mut ANGVEL = StackArray::<f64, 3>::new(1..=3);
    let mut AV = StackArray::<f64, 3>::new(1..=3);
    let mut AV1 = StackArray::<f64, 3>::new(1..=3);
    let mut AV2 = StackArray::<f64, 3>::new(1..=3);
    let mut AXIS = StackArray2D::<f64, 60>::new(1..=3, 1..=20);
    let mut COUT: f64 = 0.0;
    let mut COUT1: f64 = 0.0;
    let mut COUT2: f64 = 0.0;
    let mut DT: f64 = 0.0;
    let mut ET: f64 = 0.0;
    let mut ET1: f64 = 0.0;
    let mut ET2: f64 = 0.0;
    let mut MATRIX = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut MAXANG: f64 = 0.0;
    let mut MINANG: f64 = 0.0;
    let mut OFF: f64 = 0.0;
    let mut RATE: f64 = 0.0;
    let mut ROT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut ROT1 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut ROT2 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut SCLK1: f64 = 0.0;
    let mut SCLK2: f64 = 0.0;
    let mut SCLKDP: f64 = 0.0;
    let mut TAXIS = StackArray::<f64, 3>::new(1..=3);
    let mut TEMP = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut UAV1 = StackArray::<f64, 3>::new(1..=3);
    let mut UAXIS = StackArray::<f64, 3>::new(1..=3);
    let mut UTAXIS = StackArray::<f64, 3>::new(1..=3);
    let mut ZEROPT: f64 = 0.0;
    let mut CKHAND: i32 = 0;
    let mut FND: bool = false;
    let mut FND1: bool = false;
    let mut FND2: bool = false;

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
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_TSTCK3", ctx)?;

    testutil::TCASE(
        b"Check to make sure that we can get an attitude back from the C-kernel. ",
        ctx,
    )?;

    testutil::TSTCK3(
        b"TEST.CK",
        b"TEST.SCLK",
        true,
        true,
        false,
        &mut CKHAND,
        ctx,
    )?;
    SCLKDP = 0.0;

    spicelib::CKGPAV(
        -9999,
        SCLKDP,
        0.0,
        b"J2000",
        ROT.as_slice_mut(),
        AV.as_slice_mut(),
        &mut COUT,
        &mut FND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FND, true, OK, ctx)?;
    testutil::CHCKSD(b"CLKOUT", COUT, b"=", SCLKDP, 0.0, OK, ctx)?;

    SCLKDP = 1000000000.0;

    spicelib::CKGPAV(
        -9999,
        SCLKDP,
        0.0,
        b"J2000",
        ROT.as_slice_mut(),
        AV.as_slice_mut(),
        &mut COUT,
        &mut FND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FND, true, OK, ctx)?;
    testutil::CHCKSD(b"CLKOUT", COUT, b"=", SCLKDP, 0.0, OK, ctx)?;

    SCLKDP = 10000000000000.0;

    spicelib::CKGPAV(
        -9999,
        SCLKDP,
        0.0,
        b"J2000",
        ROT.as_slice_mut(),
        AV.as_slice_mut(),
        &mut COUT,
        &mut FND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FND, true, OK, ctx)?;
    testutil::CHCKSD(b"CLKOUT", COUT, b"=", SCLKDP, 0.0, OK, ctx)?;

    testutil::TCASE(
        b"Check to see that the rotations returned by TSTATD behave as advertised. ",
        ctx,
    )?;

    AXIS[[1, 1]] = 1.0;
    AXIS[[2, 1]] = 2.0;
    AXIS[[3, 1]] = 4.0;

    AXIS[[1, 2]] = 2.0;
    AXIS[[2, 2]] = 4.0;
    AXIS[[3, 2]] = 1.0;

    AXIS[[1, 3]] = 4.0;
    AXIS[[2, 3]] = 1.0;
    AXIS[[3, 3]] = 2.0;

    AXIS[[1, 4]] = 4.0;
    AXIS[[2, 4]] = 2.0;
    AXIS[[3, 4]] = 1.0;

    AXIS[[1, 5]] = 2.0;
    AXIS[[2, 5]] = 1.0;
    AXIS[[3, 5]] = 4.0;

    AXIS[[1, 6]] = 1.0;
    AXIS[[2, 6]] = 4.0;
    AXIS[[3, 6]] = 2.0;

    AXIS[[1, 7]] = 1.0;
    AXIS[[2, 7]] = 2.0;
    AXIS[[3, 7]] = 3.0;

    AXIS[[1, 8]] = 2.0;
    AXIS[[2, 8]] = 3.0;
    AXIS[[3, 8]] = 1.0;

    AXIS[[1, 9]] = 3.0;
    AXIS[[2, 9]] = 1.0;
    AXIS[[3, 9]] = 2.0;

    AXIS[[1, 10]] = 3.0;
    AXIS[[2, 10]] = 2.0;
    AXIS[[3, 10]] = 1.0;

    AXIS[[1, 11]] = 2.0;
    AXIS[[2, 11]] = 1.0;
    AXIS[[3, 11]] = 3.0;

    AXIS[[1, 12]] = 1.0;
    AXIS[[2, 12]] = 3.0;
    AXIS[[3, 12]] = 2.0;

    AXIS[[1, 13]] = 2.0;
    AXIS[[2, 13]] = 3.0;
    AXIS[[3, 13]] = 6.0;

    AXIS[[1, 14]] = 3.0;
    AXIS[[2, 14]] = 6.0;
    AXIS[[3, 14]] = 2.0;

    AXIS[[1, 15]] = 6.0;
    AXIS[[2, 15]] = 2.0;
    AXIS[[3, 15]] = 3.0;

    AXIS[[1, 16]] = 6.0;
    AXIS[[2, 16]] = 3.0;
    AXIS[[3, 16]] = 2.0;

    AXIS[[1, 17]] = 3.0;
    AXIS[[2, 17]] = 2.0;
    AXIS[[3, 17]] = 6.0;

    AXIS[[1, 18]] = 2.0;
    AXIS[[2, 18]] = 6.0;
    AXIS[[3, 18]] = 3.0;

    AXIS[[1, 19]] = 1.0;
    AXIS[[2, 19]] = 1.0;
    AXIS[[3, 19]] = 1.0;

    AXIS[[1, 20]] = 0.0;
    AXIS[[2, 20]] = 0.0;
    AXIS[[3, 20]] = 1.0;

    ET = -900000010.0;

    for I in 1..=20 {
        DT = 1000000.0;
        //
        // Compute the rotation of the structure over a 1 million
        // second interval.
        //
        testutil::TSTATD((ET - DT), ROT1.as_slice_mut(), AV1.as_slice_mut(), ctx);
        testutil::TSTATD(ET, ROT2.as_slice_mut(), AV2.as_slice_mut(), ctx);

        spicelib::MTXM(ROT2.as_slice(), ROT1.as_slice(), MATRIX.as_slice_mut());
        spicelib::RAXISA(MATRIX.as_slice(), TAXIS.as_slice_mut(), &mut ANGLE, ctx)?;

        spicelib::VHAT(TAXIS.as_slice(), UTAXIS.as_slice_mut());
        spicelib::VHAT(AV1.as_slice(), UAV1.as_slice_mut());
        spicelib::VHAT(AXIS.subarray([1, I]), UAXIS.as_slice_mut());

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"AV1",
            AV1.as_slice(),
            b"=",
            AV2.as_slice(),
            3,
            0.0,
            OK,
            ctx,
        )?;
        testutil::CHCKSD(b"ANGLE", ANGLE, b"~/", 0.1, 0.0000000000001, OK, ctx)?;
        testutil::CHCKAD(
            b"UAV1",
            UAV1.as_slice(),
            b"~",
            UAXIS.as_slice(),
            3,
            0.00000000000001,
            OK,
            ctx,
        )?;
        testutil::CHCKAD(
            b"UTAXIS",
            UTAXIS.as_slice(),
            b"~",
            UAXIS.as_slice(),
            3,
            0.0000000000001,
            OK,
            ctx,
        )?;
        testutil::CHCKAD(
            b"UAV1",
            UAV1.as_slice(),
            b"||",
            UAXIS.as_slice(),
            3,
            0.0000000000001,
            OK,
            ctx,
        )?;
        testutil::CHCKAD(
            b"UTAXIS",
            UTAXIS.as_slice(),
            b"||",
            UAXIS.as_slice(),
            3,
            0.0000000000001,
            OK,
            ctx,
        )?;

        ET = (ET + 100000000.0);
    }

    testutil::TCASE(b"Check for continuity of the C-kernel and TSTATD across the 100 million second boundaries refered to in TSTATD. ", ctx)?;

    ET = -900000000.0;

    spicelib::TPARSE(b"1-JAN-1980", &mut ZEROPT, &mut ERROR, ctx)?;

    while (ET < ZEROPT) {
        ET = (ET + 100000000.0);
    }

    RATE = (1.0 / 10000000.0);
    MAXANG = (2.0 * RATE);
    MINANG = (0.1 * RATE);

    while (ET < (ZEROPT + 999999999.0)) {
        ET1 = (ET - 1.0);
        ET2 = (ET + 1.0);

        spicelib::SCE2T(-9, ET1, &mut SCLK1, ctx)?;
        spicelib::SCE2T(-9, ET2, &mut SCLK2, ctx)?;
        spicelib::CKGPAV(
            -9999,
            SCLK1,
            0.0,
            b"J2000",
            ROT1.as_slice_mut(),
            AV1.as_slice_mut(),
            &mut COUT1,
            &mut FND1,
            ctx,
        )?;
        spicelib::CKGPAV(
            -9999,
            SCLK2,
            0.0,
            b"J2000",
            ROT2.as_slice_mut(),
            AV2.as_slice_mut(),
            &mut COUT2,
            &mut FND2,
            ctx,
        )?;

        spicelib::MTXM(ROT2.as_slice(), ROT1.as_slice(), TEMP.as_slice_mut());
        spicelib::RAXISA(TEMP.as_slice(), TAXIS.as_slice_mut(), &mut ANGLE, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(b"FND1", FND1, true, OK, ctx)?;
        testutil::CHCKSL(b"FND2", FND2, true, OK, ctx)?;
        testutil::CHCKSD(b"COUT1", COUT1, b"=", SCLK1, 0.0, OK, ctx)?;
        testutil::CHCKSD(b"COUT2", COUT2, b"=", SCLK2, 0.0, OK, ctx)?;
        testutil::CHCKSD(b"ANGLE", ANGLE, b">", MINANG, 0.0, OK, ctx)?;
        testutil::CHCKSD(b"ANGEL", ANGLE, b"<", MAXANG, 0.0, OK, ctx)?;

        testutil::TSTATD(ET1, ROT1.as_slice_mut(), AV1.as_slice_mut(), ctx);
        testutil::TSTATD(ET2, ROT2.as_slice_mut(), AV2.as_slice_mut(), ctx);

        spicelib::MTXM(ROT2.as_slice(), ROT1.as_slice(), TEMP.as_slice_mut());
        spicelib::RAXISA(TEMP.as_slice(), TAXIS.as_slice_mut(), &mut ANGLE, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSD(b"ANGLE", ANGLE, b">", MINANG, 0.0, OK, ctx)?;
        testutil::CHCKSD(b"ANGEL", ANGLE, b"<", MAXANG, 0.0, OK, ctx)?;

        ET = (ET + 100000000.0);
    }

    testutil::TCASE(
        b"Make sure the conversion between ET and ticks works as expected. ",
        ctx,
    )?;

    spicelib::TPARSE(b"1-JAN-1980", &mut ZEROPT, &mut ERROR, ctx)?;

    ET = ZEROPT;

    for I in 1..=999 {
        spicelib::SCE2T(-9, ET, &mut SCLKDP, ctx)?;

        COUT = ((ET - ZEROPT) * 10000.0);

        testutil::CHCKSD(b"SCLKDP", SCLKDP, b"=", COUT, 0.0, OK, ctx)?;

        ET = (ET + 10000000.0);
    }

    testutil::TCASE(b"Check to see if attitudes returned by the C-kernel for are very nearly the same as that returned by TSTATD (for body -9999). ", ctx)?;

    spicelib::TPARSE(b"1-JAN-1980", &mut ET, &mut ERROR, ctx)?;

    for I in 1..=100 {
        spicelib::SCE2T(-9, ET, &mut SCLKDP, ctx)?;

        testutil::TSTATD(ET, MATRIX.as_slice_mut(), ANGVEL.as_slice_mut(), ctx);
        spicelib::CKGPAV(
            -9999,
            SCLKDP,
            0.0,
            b"GALACTIC",
            ROT.as_slice_mut(),
            AV.as_slice_mut(),
            &mut COUT,
            &mut FND,
            ctx,
        )?;

        testutil::TSTMSG(b"#", b"Subcase: #. Angle between rotations: #  ", ctx);

        spicelib::MXMT(MATRIX.as_slice(), ROT.as_slice(), TEMP.as_slice_mut());
        spicelib::RAXISA(TEMP.as_slice(), TAXIS.as_slice_mut(), &mut ANGLE, ctx)?;

        testutil::TSTMSI(I, ctx);
        testutil::TSTMSD(ANGLE, ctx);

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", FND, true, OK, ctx)?;
        testutil::CHCKSD(b"ANGLE", ANGLE, b"~", 0.0, 0.00000000000001, OK, ctx)?;

        testutil::CHCKSD(b"CLKOUT", COUT, b"=", SCLKDP, 0.0, OK, ctx)?;
        testutil::CHCKAD(
            b"ROT",
            ROT.as_slice(),
            b"~/",
            MATRIX.as_slice(),
            9,
            0.00000000001,
            OK,
            ctx,
        )?;
        testutil::CHCKAD(
            b"AV",
            AV.as_slice(),
            b"||",
            ANGVEL.as_slice(),
            3,
            0.00000000001,
            OK,
            ctx,
        )?;
        testutil::CHCKAD(
            b"AV",
            AV.as_slice(),
            b"~/",
            ANGVEL.as_slice(),
            3,
            0.00000000000001,
            OK,
            ctx,
        )?;

        ET = (ET + 10000000.0);
    }

    testutil::TCASE(b"Check to see if attitudes returned by the C-kernel for agree with those returned by TSTATD (for body -10000). ", ctx)?;

    spicelib::TPARSE(b"1-JAN-1980", &mut ET, &mut ERROR, ctx)?;

    OFF = 0.0;

    for I in 1..=100 {
        spicelib::SCE2T(-9, ET, &mut SCLKDP, ctx)?;

        testutil::TSTATD(
            (ET + OFF),
            MATRIX.as_slice_mut(),
            ANGVEL.as_slice_mut(),
            ctx,
        );
        spicelib::CKGPAV(
            -10000,
            SCLKDP,
            0.0,
            b"FK4",
            ROT.as_slice_mut(),
            AV.as_slice_mut(),
            &mut COUT,
            &mut FND,
            ctx,
        )?;

        testutil::TSTMSG(b"#", b"Subcase: #. Angle between rotations: #  ", ctx);

        spicelib::MXMT(MATRIX.as_slice(), ROT.as_slice(), TEMP.as_slice_mut());
        spicelib::RAXISA(TEMP.as_slice(), TAXIS.as_slice_mut(), &mut ANGLE, ctx)?;

        testutil::TSTMSI(I, ctx);
        testutil::TSTMSD(ANGLE, ctx);

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", FND, true, OK, ctx)?;
        testutil::CHCKSD(b"ANGLE", ANGLE, b"~", 0.0, 0.00000000000001, OK, ctx)?;

        testutil::CHCKSD(b"CLKOUT", COUT, b"=", SCLKDP, 0.0, OK, ctx)?;
        testutil::CHCKAD(
            b"ROT",
            ROT.as_slice(),
            b"~/",
            MATRIX.as_slice(),
            9,
            0.00000000001,
            OK,
            ctx,
        )?;
        testutil::CHCKAD(
            b"AV",
            AV.as_slice(),
            b"||",
            ANGVEL.as_slice(),
            3,
            0.00000000001,
            OK,
            ctx,
        )?;
        testutil::CHCKAD(
            b"AV",
            AV.as_slice(),
            b"~/",
            ANGVEL.as_slice(),
            3,
            0.00000000000001,
            OK,
            ctx,
        )?;

        ET = (ET + 10000000.0);
    }

    testutil::TCASE(b"Check to see if attitudes returned by the C-kernel for agree with those returned by TSTATD (for body -10001 ). ", ctx)?;

    spicelib::TPARSE(b"1-JAN-1980", &mut ET, &mut ERROR, ctx)?;

    OFF = 0.0;

    for I in 1..=100 {
        spicelib::SCE2T(-9, ET, &mut SCLKDP, ctx)?;

        testutil::TSTATD(
            (ET + OFF),
            MATRIX.as_slice_mut(),
            ANGVEL.as_slice_mut(),
            ctx,
        );
        spicelib::CKGPAV(
            -10001,
            SCLKDP,
            0.0,
            b"J2000",
            ROT.as_slice_mut(),
            AV.as_slice_mut(),
            &mut COUT,
            &mut FND,
            ctx,
        )?;

        testutil::TSTMSG(b"#", b"Subcase: #. Angle between rotations: #  ", ctx);

        spicelib::MXMT(MATRIX.as_slice(), ROT.as_slice(), TEMP.as_slice_mut());
        spicelib::RAXISA(TEMP.as_slice(), TAXIS.as_slice_mut(), &mut ANGLE, ctx)?;

        testutil::TSTMSI(I, ctx);
        testutil::TSTMSD(ANGLE, ctx);

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", FND, true, OK, ctx)?;
        testutil::CHCKSD(b"ANGLE", ANGLE, b"~", 0.0, 0.00000000000001, OK, ctx)?;

        testutil::CHCKSD(b"CLKOUT", COUT, b"=", SCLKDP, 0.0, OK, ctx)?;
        testutil::CHCKAD(
            b"ROT",
            ROT.as_slice(),
            b"~/",
            MATRIX.as_slice(),
            9,
            0.00000000001,
            OK,
            ctx,
        )?;
        testutil::CHCKAD(
            b"AV",
            AV.as_slice(),
            b"||",
            ANGVEL.as_slice(),
            3,
            0.00000000001,
            OK,
            ctx,
        )?;
        testutil::CHCKAD(
            b"AV",
            AV.as_slice(),
            b"~/",
            ANGVEL.as_slice(),
            3,
            0.00000000000001,
            OK,
            ctx,
        )?;

        ET = (ET + 10000000.0);
    }

    spicelib::CKUPF(CKHAND, ctx)?;
    testutil::KILFIL(b"TEST.CK", ctx)?;
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
