//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXDEG: i32 = 15;
const TIGHT: f64 = 0.00000000000001;
const MEDIUM: f64 = 0.000000000001;

struct SaveVars {
    QID: StackArray<f64, 4>,
    QIDNEG: StackArray<f64, 4>,
    QI: StackArray<f64, 4>,
    QJ: StackArray<f64, 4>,
    QK: StackArray<f64, 4>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut QID = StackArray::<f64, 4>::new(0..=3);
        let mut QIDNEG = StackArray::<f64, 4>::new(0..=3);
        let mut QI = StackArray::<f64, 4>::new(0..=3);
        let mut QJ = StackArray::<f64, 4>::new(0..=3);
        let mut QK = StackArray::<f64, 4>::new(0..=3);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(-1.0), Val::D(0.0), Val::D(0.0), Val::D(0.0)].into_iter();
            QIDNEG
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(1.0), Val::D(0.0), Val::D(0.0), Val::D(0.0)].into_iter();
            QID.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(1.0), Val::D(0.0), Val::D(0.0)].into_iter();
            QI.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(1.0), Val::D(0.0)].into_iter();
            QJ.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            QK.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            QID,
            QIDNEG,
            QI,
            QJ,
            QK,
        }
    }
}

//$Procedure F_QUAT ( Quaternion routine tests )
pub fn F_QUAT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ANGLE = StackArray::<f64, 3>::new(1..=3);
    let mut AV = StackArray::<f64, 3>::new(1..=3);
    let mut AVX = StackArray::<f64, 3>::new(1..=3);
    let mut DM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut DQ = StackArray::<f64, 4>::new(0..=3);
    let mut EXPAV = StackArray::<f64, 3>::new(1..=3);
    let mut M1 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut M2 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut MEXP = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut MOUT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut Q = StackArray::<f64, 4>::new(0..=3);
    let mut Q1 = StackArray::<f64, 4>::new(0..=3);
    let mut Q2 = StackArray::<f64, 4>::new(0..=3);
    let mut QAV = StackArray::<f64, 4>::new(0..=3);
    let mut QEXP = StackArray::<f64, 4>::new(0..=3);
    let mut QOUT = StackArray::<f64, 4>::new(0..=3);
    let mut QTMP = StackArray::<f64, 4>::new(0..=3);
    let mut XTRANS = StackArray2D::<f64, 36>::new(1..=6, 1..=6);

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
    testutil::TOPEN(b"F_QUAT", ctx)?;

    //
    // QXQ tests follow:
    //
    testutil::TCASE(
        b"QXQ test:  Check compliance with  Hamilton\'s rules.  Test QI x QJ.",
        ctx,
    )?;

    spicelib::QXQ(save.QI.as_slice(), save.QJ.as_slice(), QOUT.as_slice_mut());
    testutil::CHCKAD(
        b"i*j",
        QOUT.as_slice(),
        b"~",
        save.QK.as_slice(),
        4,
        TIGHT,
        OK,
        ctx,
    )?;

    testutil::TCASE(
        b"Check compliance with Hamilton\'s rules.  Test QJ x QK.",
        ctx,
    )?;

    spicelib::QXQ(save.QJ.as_slice(), save.QK.as_slice(), QOUT.as_slice_mut());
    testutil::CHCKAD(
        b"j*k",
        QOUT.as_slice(),
        b"~",
        save.QI.as_slice(),
        4,
        TIGHT,
        OK,
        ctx,
    )?;

    testutil::TCASE(
        b"Check compliance with Hamilton\'s rules.  Test QK x QI.",
        ctx,
    )?;

    spicelib::QXQ(save.QK.as_slice(), save.QI.as_slice(), QOUT.as_slice_mut());
    testutil::CHCKAD(
        b"k*i",
        QOUT.as_slice(),
        b"~",
        save.QJ.as_slice(),
        4,
        TIGHT,
        OK,
        ctx,
    )?;

    testutil::TCASE(
        b"Check compliance with Hamilton\'s rules.  Test QI x QI.",
        ctx,
    )?;
    spicelib::QXQ(save.QI.as_slice(), save.QI.as_slice(), QOUT.as_slice_mut());
    testutil::CHCKAD(
        b"i*i",
        QOUT.as_slice(),
        b"~",
        save.QIDNEG.as_slice(),
        4,
        TIGHT,
        OK,
        ctx,
    )?;

    testutil::TCASE(
        b"Check compliance with Hamilton\'s rules.  Test QJ x QJ.",
        ctx,
    )?;

    spicelib::QXQ(save.QJ.as_slice(), save.QJ.as_slice(), QOUT.as_slice_mut());
    testutil::CHCKAD(
        b"j*j",
        QOUT.as_slice(),
        b"~",
        save.QIDNEG.as_slice(),
        4,
        TIGHT,
        OK,
        ctx,
    )?;

    testutil::TCASE(
        b"Check compliance with Hamilton\'s rules.  Test QK x QK.",
        ctx,
    )?;

    spicelib::QXQ(save.QK.as_slice(), save.QK.as_slice(), QOUT.as_slice_mut());
    testutil::CHCKAD(
        b"k*k",
        QOUT.as_slice(),
        b"~",
        save.QIDNEG.as_slice(),
        4,
        TIGHT,
        OK,
        ctx,
    )?;

    QEXP[0] = 1.0;
    QEXP[1] = 2.0;
    QEXP[2] = 3.0;
    QEXP[3] = 4.0;

    testutil::TCASE(b"Check right-multiplication by the identity.", ctx)?;
    spicelib::QXQ(QEXP.as_slice(), save.QID.as_slice(), QOUT.as_slice_mut());
    testutil::CHCKAD(
        b"QEXP * 1",
        QOUT.as_slice(),
        b"~",
        QEXP.as_slice(),
        4,
        TIGHT,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Check left-multiplication by the identity.", ctx)?;
    spicelib::QXQ(save.QID.as_slice(), QEXP.as_slice(), QOUT.as_slice_mut());
    testutil::CHCKAD(
        b"1 * QEXP",
        QOUT.as_slice(),
        b"~",
        QEXP.as_slice(),
        4,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // Try a more complex case:  multiply two rotation matrices
    // via quaternion multiplication.
    //
    testutil::TCASE(
        b"Multiply two rotations via quaternion multiplication.",
        ctx,
    )?;

    spicelib::EUL2M(
        (spicelib::RPD(ctx) * 20.0),
        (spicelib::RPD(ctx) * 10.0),
        (spicelib::RPD(ctx) * 70.0),
        3,
        1,
        3,
        M1.as_slice_mut(),
        ctx,
    )?;

    spicelib::EUL2M(
        (spicelib::RPD(ctx) * -20.0),
        (spicelib::RPD(ctx) * 30.0),
        (spicelib::RPD(ctx) * -10.0),
        3,
        1,
        3,
        M2.as_slice_mut(),
        ctx,
    )?;

    spicelib::M2Q(M1.as_slice(), Q1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::M2Q(M2.as_slice(), Q2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::QXQ(Q1.as_slice(), Q2.as_slice(), QOUT.as_slice_mut());
    spicelib::Q2M(QOUT.as_slice(), MOUT.as_slice_mut());

    spicelib::MXM(M1.as_slice(), M2.as_slice(), MEXP.as_slice_mut());

    testutil::CHCKAD(
        b"MOUT",
        MOUT.as_slice(),
        b"~",
        MEXP.as_slice(),
        9,
        0.000000000001,
        OK,
        ctx,
    )?;

    //
    // QDQ2AV tests follow:
    //
    testutil::TCASE(b"Produce quaternion and derivative from Euler angles and a.v.  Recover a.v. from QDQ2AV; compare to original a.v.", ctx)?;

    //
    // Start with a known rotation and angular velocity.  Find
    // the quaternion and quaternion derivative.  The latter is
    // computed from
    //
    //                    *
    //     AV  =   -2  * Q  * DQ
    //
    //     DQ  =  -1/2 * Q  * AV
    //

    ANGLE[1] = -(20.0 * spicelib::RPD(ctx));
    ANGLE[2] = (50.0 * spicelib::RPD(ctx));
    ANGLE[3] = -(60.0 * spicelib::RPD(ctx));

    spicelib::EUL2M(
        ANGLE[3],
        ANGLE[2],
        ANGLE[1],
        3,
        1,
        3,
        M1.as_slice_mut(),
        ctx,
    )?;

    spicelib::M2Q(M1.as_slice(), Q.as_slice_mut(), ctx)?;

    EXPAV[1] = 1.0;
    EXPAV[2] = 2.0;
    EXPAV[3] = 3.0;

    //
    // Form the quaternion derivative.
    //
    QAV[0] = 0.0;
    spicelib::VEQU(EXPAV.as_slice(), QAV.subarray_mut(1));

    spicelib::QXQ(Q.as_slice(), QAV.as_slice(), DQ.as_slice_mut());

    spicelib::VSCLG(-0.5, DQ.as_slice(), 4, QTMP.as_slice_mut());
    spicelib::MOVED(QTMP.as_slice(), 4, DQ.as_slice_mut());

    //
    // Recover angular velocity from Q and DQ.
    //
    spicelib::QDQ2AV(Q.as_slice(), DQ.as_slice(), AV.as_slice_mut());

    //
    // Do a consistency check against the original a.v.  This is
    // an intermediate check; it demonstrates invertibility but
    // not correctness of our formulas.
    //
    testutil::CHCKAD(
        b"AV from Q and DQ",
        AV.as_slice(),
        b"~",
        EXPAV.as_slice(),
        3,
        0.000000000001,
        OK,
        ctx,
    )?;

    //
    // Convert Q back to a rotation matrix.
    //
    spicelib::Q2M(Q.as_slice(), M1.as_slice_mut());

    testutil::TCASE(b"Map a quaternion and derivative to angular velocity via a transformation matrix and xf2rav.  Compare to result from QDQ2AV.", ctx)?;
    //
    // Convert Q and DQ to a rotation derivative matrix.  This
    // somewhat messy procedure is based on differentiating the
    // formula for deriving a rotation from a quaternion, then
    // substituting components of Q and DQ into the derivative
    // formula.
    //
    DM[[1, 1]] = -(4.0 * ((Q[2] * DQ[2]) + (Q[3] * DQ[3])));

    DM[[1, 2]] = (2.0 * ((((Q[1] * DQ[2]) + (Q[2] * DQ[1])) - (Q[0] * DQ[3])) - (Q[3] * DQ[0])));

    DM[[1, 3]] = (2.0 * ((((Q[1] * DQ[3]) + (Q[3] * DQ[1])) + (Q[0] * DQ[2])) + (Q[2] * DQ[0])));

    DM[[2, 1]] = (2.0 * ((((Q[1] * DQ[2]) + (Q[2] * DQ[1])) + (Q[0] * DQ[3])) + (Q[3] * DQ[0])));

    DM[[2, 2]] = -(4.0 * ((Q[1] * DQ[1]) + (Q[3] * DQ[3])));

    DM[[2, 3]] = (2.0 * ((((Q[2] * DQ[3]) + (Q[3] * DQ[2])) - (Q[0] * DQ[1])) - (Q[1] * DQ[0])));

    DM[[3, 1]] = (2.0 * ((((Q[3] * DQ[1]) + (Q[1] * DQ[3])) - (Q[0] * DQ[2])) - (Q[2] * DQ[0])));

    DM[[3, 2]] = (2.0 * ((((Q[2] * DQ[3]) + (Q[3] * DQ[2])) + (Q[0] * DQ[1])) + (Q[1] * DQ[0])));

    DM[[3, 3]] = -(4.0 * ((Q[1] * DQ[1]) + (Q[2] * DQ[2])));

    //
    // Form the state transformation matrix corresponding to M1
    // and DM.
    //
    spicelib::CLEARD(36, XTRANS.as_slice_mut());

    //
    // Upper left block:
    //
    for I in 1..=3 {
        for J in 1..=3 {
            XTRANS[[I, J]] = M1[[I, J]];
        }
    }

    //
    // Lower right block:
    //
    for I in 1..=3 {
        for J in 1..=3 {
            XTRANS[[(3 + I), (3 + J)]] = M1[[I, J]];
        }
    }

    //
    // Lower left block:
    //
    for I in 1..=3 {
        for J in 1..=3 {
            XTRANS[[(3 + I), J]] = DM[[I, J]];
        }
    }

    spicelib::XF2RAV(XTRANS.as_slice(), MOUT.as_slice_mut(), AVX.as_slice_mut());

    //
    // Compare the angular velocity obtained from the state
    // transformation with that from QDQ2AV.
    //
    testutil::CHCKAD(
        b"AV from Q and DQ",
        AV.as_slice(),
        b"~",
        AVX.as_slice(),
        3,
        0.000000000001,
        OK,
        ctx,
    )?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
