//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure F_XFRAV ( Frame conversion to angular velocity )
pub fn F_XFRAV(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut AV1 = StackArray::<f64, 3>::new(1..=3);
    let mut AV2 = StackArray::<f64, 3>::new(1..=3);
    let mut DLAT: f64 = 0.0;
    let mut DLONG: f64 = 0.0;
    let mut DMAG: f64 = 0.0;
    let mut LAT: f64 = 0.0;
    let mut LONG: f64 = 0.0;
    let mut MAG: f64 = 0.0;
    let mut R: f64 = 0.0;
    let mut ROT1 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut ROT2 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut STATE = StackArray::<f64, 6>::new(1..=6);
    let mut STATE1 = StackArray::<f64, 6>::new(1..=6);
    let mut STATE2 = StackArray::<f64, 6>::new(1..=6);
    let mut V1 = StackArray::<f64, 3>::new(1..=3);
    let mut V2 = StackArray::<f64, 3>::new(1..=3);
    let mut VTEMP = StackArray::<f64, 3>::new(1..=3);
    let mut X = StackArray::<f64, 3>::new(1..=3);
    let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XFORM1 = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XFORM2 = StackArray2D::<f64, 36>::new(1..=6, 1..=6);

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_XFRAV", ctx)?;

    testutil::TCASE(b"This case tests that converting from a rotation and angular velocity to a state transformation and then back to rotation and angular velocity is very nearly the identity. 200 different sub-cases are examined.", ctx)?;

    R = 1.0;
    LAT = -spicelib::HALFPI(ctx);
    LONG = 0.0;
    MAG = 0.1;

    DLAT = (spicelib::PI(ctx) / 11.0);
    DLONG = (spicelib::PI(ctx) / 10.0);
    DMAG = 0.1;

    for I in 1..=10 {
        LAT = (LAT + DLAT);
        LONG = 0.0;

        for J in 1..=20 {
            testutil::TSTMSG(
                b"#",
                b"The values of I and J are # and # respectively. ",
                ctx,
            );
            testutil::TSTMSI(I, ctx);
            testutil::TSTMSI(J, ctx);

            spicelib::LATREC(R, LONG, LAT, ROT1.subarray_mut([1, 1]));
            let [arg0, arg1, arg2] = ROT1
                .get_disjoint_slices_mut([[1, 1], [1, 2], [1, 3]])
                .unwrap();
            spicelib::FRAME(arg0, arg1, arg2);

            spicelib::LATREC(R, LAT, LONG, X.as_slice_mut());
            spicelib::VSCL(MAG, X.as_slice(), AV1.as_slice_mut());

            spicelib::RAV2XF(ROT1.as_slice(), AV1.as_slice(), XFORM.as_slice_mut());
            spicelib::XF2RAV(XFORM.as_slice(), ROT2.as_slice_mut(), AV2.as_slice_mut());

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKAD(
                b"ROT2",
                ROT2.as_slice(),
                b"~",
                ROT1.as_slice(),
                9,
                0.00000000000001,
                OK,
                ctx,
            )?;
            testutil::CHCKAD(
                b"AV2",
                AV2.as_slice(),
                b"~",
                AV1.as_slice(),
                3,
                0.00000000000001,
                OK,
                ctx,
            )?;

            spicelib::XPOSE(ROT1.as_slice(), ROT2.as_slice_mut());
            spicelib::MXV(ROT1.as_slice(), AV1.as_slice(), AV2.as_slice_mut());
            spicelib::VSCLIP(-1.0, AV2.as_slice_mut());
            spicelib::RAV2XF(ROT2.as_slice(), AV2.as_slice(), XFORM2.as_slice_mut());
            spicelib::INVSTM(XFORM.as_slice(), XFORM1.as_slice_mut(), ctx)?;

            testutil::CHCKAD(
                b"XFORM1",
                XFORM1.as_slice(),
                b"~",
                XFORM2.as_slice(),
                9,
                0.00000000000001,
                OK,
                ctx,
            )?;

            LONG = (LONG + DLONG);
            MAG = (MAG + DMAG);
        }
    }

    testutil::TSTMSG(b"#", b" ", ctx);

    testutil::TCASE(b"In this case we construct a rotation, angular velocity and state transformation matrix.  We use these to compute velocities in two different ways and compare the results. 200 sub-cases are examined.", ctx)?;

    R = 1.0;
    LAT = -spicelib::HALFPI(ctx);
    LONG = 0.0;
    MAG = 0.1;

    DLAT = (spicelib::PI(ctx) / 11.0);
    DLONG = (spicelib::PI(ctx) / 10.0);
    DMAG = 0.1;

    STATE[1] = 6.0;
    STATE[2] = 5.0;
    STATE[3] = 4.0;
    STATE[4] = 3.0;
    STATE[5] = 2.0;
    STATE[6] = 1.0;

    for I in 1..=10 {
        LAT = (LAT + DLAT);
        LONG = 0.0;

        for J in 1..=20 {
            testutil::TSTMSG(
                b"#",
                b"The values of I and J are # and # respectively. ",
                ctx,
            );
            testutil::TSTMSI(I, ctx);
            testutil::TSTMSI(J, ctx);

            spicelib::LATREC(R, LONG, LAT, ROT1.subarray_mut([1, 1]));
            let [arg0, arg1, arg2] = ROT1
                .get_disjoint_slices_mut([[1, 1], [1, 2], [1, 3]])
                .unwrap();
            spicelib::FRAME(arg0, arg1, arg2);

            spicelib::LATREC(R, LAT, LONG, X.as_slice_mut());
            spicelib::VSCL(MAG, X.as_slice(), AV1.as_slice_mut());

            spicelib::RAV2XF(ROT1.as_slice(), AV1.as_slice(), XFORM.as_slice_mut());
            //
            // First transform states using the state transformation
            // matrix.
            //

            spicelib::MXVG(
                XFORM.as_slice(),
                STATE.as_slice(),
                6,
                6,
                STATE1.as_slice_mut(),
            );
            //
            // Now transform states using the rotation and angular
            // velocity.  Recall that the angular velocity of
            // FRAME1 with respect to FRAME2 is the opposite of
            // the angular velocity of FRAME2 with respect to FRAME1.
            //
            spicelib::VMINUS(AV1.as_slice(), AV2.as_slice_mut());
            //
            // A part of the position and velocity is obtained by
            // simply rotating the original position and velocity.
            //
            spicelib::MXV(ROT1.as_slice(), STATE.as_slice(), STATE2.as_slice_mut());
            spicelib::MXV(ROT1.as_slice(), STATE.subarray(4), V1.as_slice_mut());

            //
            // The rest of the velocity is obtained by crossing
            // the angular velocity with the current position
            // in  FRAME2.
            //
            spicelib::VCRSS(AV2.as_slice(), STATE.as_slice(), VTEMP.as_slice_mut());
            spicelib::MXV(ROT1.as_slice(), VTEMP.as_slice(), V2.as_slice_mut());
            //
            // Add the results together to get the full velocity.
            //
            spicelib::VADD(V1.as_slice(), V2.as_slice(), STATE2.subarray_mut(4));

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKAD(
                b"POSITION",
                STATE1.as_slice(),
                b"~~/",
                STATE2.as_slice(),
                3,
                0.00000000000001,
                OK,
                ctx,
            )?;
            testutil::CHCKAD(
                b"VELOCITY",
                STATE1.subarray(4),
                b"~~/",
                STATE2.subarray(4),
                3,
                0.00000000000001,
                OK,
                ctx,
            )?;

            spicelib::MOVED(STATE2.as_slice(), 6, STATE.as_slice_mut());

            LONG = (LONG + DLONG);
            MAG = (MAG + DMAG);
        }
    }

    testutil::TSTMSG(b"#", b" ", ctx);

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
