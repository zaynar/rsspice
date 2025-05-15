//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const IREF: &[u8] = b"J2000";

//*****************************************************************
//*****************************************************************
//*
//*
//*
//*
//*    T E S T   U T I L I T I E S
//*
//*
//*
//*
//*****************************************************************
//*****************************************************************

//*****************************************************************
//
//     T_GENC06
//
//*****************************************************************

//
// Generate C-matrices for CK type 06 software testing.
//
//    Version 1.0.0 03-FEB-2014 (NJB)
//
//
// Output AVV has units of radians/s.
//
pub fn T_GENC06(
    SEGNO: i32,
    MNSGNO: i32,
    BEGREC: i32,
    N: i32,
    FRAME: &[u8],
    RATE: f64,
    EPOCHS: &mut [f64],
    QUATS: &mut [f64],
    AVVS: &mut [f64],
    OK: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut EPOCHS = DummyArrayMut::new(EPOCHS, 1..);
    let mut QUATS = DummyArrayMut2D::new(QUATS, 0..=3, 1..);
    let mut AVVS = DummyArrayMut2D::new(AVVS, 1..=3, 1..);
    let mut ANGLE1: f64 = 0.0;
    let mut ANGLE: f64 = 0.0;
    let mut AXIS = StackArray::<f64, 3>::new(1..=3);
    let mut CMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut DANGL1: f64 = 0.0;
    let mut M: f64 = 0.0;
    let mut RESX = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut S: f64 = 0.0;
    let mut XF = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut J: i32 = 0;
    let mut SAMFRM: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"T_GENC06", ctx)?;

    //
    // We'll construct a sequence of C-matrices and angular velocities
    // representing rotations about a constant axis in the frame
    // designated by IREF. Those rotations and corresponding angular
    // velocities will be transformed to the requested output frame.
    //
    SAMFRM = spicelib::EQSTR(IREF, FRAME);

    //
    // Below we define some parameters used for rotation angle
    // construction.
    //
    // S is a scale factor for the segment number.
    //
    S = 2500000000.0;
    //
    // M is a scale factor for the mini-segment number.
    //
    M = 50000.0;

    //
    // AXIS is the rotation axis in the J2000 frame.
    //
    spicelib::VPACK(0.0, 0.0, 1.0, AXIS.as_slice_mut());

    //
    // ANGLE1 is the base angle used to generate all of the rotation
    // angles. These angles are computed using the formula
    //
    //                                      S*SEGNO + M*MNSGNO + I
    //    ANGLE( SEGNO, MNSGNO, I ) = ANGLE1
    //

    ANGLE1 = f64::powf(
        spicelib::DPR(ctx),
        (1.0 / (((S * SEGNO as f64) + (M * MNSGNO as f64)) + 100000 as f64)),
    );

    for I in 1..=N {
        J = ((BEGREC - 1) + I);

        EPOCHS[I] = (J as f64);

        ANGLE = f64::powf(
            ANGLE1,
            (((S * SEGNO as f64) + (M * MNSGNO as f64)) + J as f64),
        );

        spicelib::AXISAR(AXIS.as_slice(), -ANGLE, CMAT.as_slice_mut());
        //
        // Create an angular velocity vector that is roughly consistent
        // with the C-matrix sequence. We'll treat the angle as a
        // function of I and differentiate the angle formula with respect
        // to I. Note that
        //
        //    ANGLE = EXP( LOG(ANGLE1) * (S*SEGNO + M*MNSGNO + I) )
        //
        //
        DANGL1 = (ANGLE * f64::ln(ANGLE1));
        //
        // Scale the angular rate from radians/tick to radians/s.
        //
        // RATE has units of seconds/tick.
        //
        DANGL1 = (DANGL1 / RATE);

        spicelib::VSCL(DANGL1, AXIS.as_slice(), AVVS.subarray_mut([1, I]));
        //
        // Transform the C-matrix and AVV to the output frame
        // if necessary. The output frame is the new base frame.
        //
        if !SAMFRM {
            spicelib::SXFORM(FRAME, IREF, EPOCHS[I], XFORM.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::RAV2XF(CMAT.as_slice(), AVVS.subarray([1, I]), XF.as_slice_mut());
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::MXMG(
                XF.as_slice(),
                XFORM.as_slice(),
                6,
                6,
                6,
                RESX.as_slice_mut(),
            );

            spicelib::XF2RAV(
                RESX.as_slice(),
                CMAT.as_slice_mut(),
                AVVS.subarray_mut([1, I]),
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        spicelib::M2Q(CMAT.as_slice(), QUATS.subarray_mut([0, I]), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (QUATS[[0, I]] < 0.0) {
            for K in 0..=3 {
                QUATS[[K, I]] = -QUATS[[K, I]];
            }
        }
    }

    spicelib::CHKOUT(b"T_GENC06", ctx)?;
    Ok(())
}
