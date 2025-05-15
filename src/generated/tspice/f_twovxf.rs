//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MSGLEN: i32 = 240;
const SEPTOL: f64 = 0.001;
const VTIGHT: f64 = 0.00000000000001;

struct SaveVars {
    NEXT: StackArray<i32, 3>,
    PREV: StackArray<i32, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut NEXT = StackArray::<i32, 3>::new(1..=3);
        let mut PREV = StackArray::<i32, 3>::new(1..=3);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(2), Val::I(3), Val::I(1)].into_iter();
            NEXT.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(3), Val::I(1), Val::I(2)].into_iter();
            PREV.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { NEXT, PREV }
    }
}

//$Procedure F_TWOVXF ( TWOVXF tests )
pub fn F_TWOVXF(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TITLE = [b' '; MSGLEN as usize];
    let mut AV = StackArray::<f64, 3>::new(1..=3);
    let mut DR = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut LB: f64 = 0.0;
    let mut PSCAL1: f64 = 0.0;
    let mut PSCAL2: f64 = 0.0;
    let mut R = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut STATE1 = StackArray::<f64, 6>::new(1..=6);
    let mut STATE2 = StackArray::<f64, 6>::new(1..=6);
    let mut UB: f64 = 0.0;
    let mut UXSTA2 = StackArray::<f64, 6>::new(1..=6);
    let mut UXSTAT = StackArray::<f64, 6>::new(1..=6);
    let mut VSCAL1: f64 = 0.0;
    let mut VSCAL2: f64 = 0.0;
    let mut XAV = StackArray::<f64, 3>::new(1..=3);
    let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XFORMI = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XR = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut XXFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut CASENO: i32 = 0;
    let mut INDEX1: i32 = 0;
    let mut INDEX2: i32 = 0;
    let mut NCASE: i32 = 0;
    let mut SEED: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Other functions
    //

    //
    // Local Parameters
    //

    // DOUBLE PRECISION      TIGHT
    // PARAMETER           ( TIGHT  = 1.D-12 )

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
    // Open the test family.
    //
    testutil::TOPEN(b"F_TWOVXF", ctx)?;

    //
    // *****************************************************************
    //
    // Error cases: TWOVXF
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"TWOVXF: Bad index", ctx)?;

    spicelib::VPACK(0.0, 0.0, 1.0, STATE1.subarray_mut(1));
    spicelib::VPACK(-1.0, 0.0, 0.0, STATE1.subarray_mut(4));

    spicelib::VPACK(1.0, 0.0, 1.0, STATE2.subarray_mut(1));
    spicelib::VPACK(0.0, 0.0, -1.0, STATE2.subarray_mut(4));

    INDEX1 = 0;
    INDEX2 = 2;

    spicelib::TWOVXF(
        STATE1.as_slice(),
        INDEX1,
        STATE2.as_slice(),
        INDEX2,
        XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADINDEX)", OK, ctx)?;

    INDEX1 = 4;
    INDEX2 = 2;

    spicelib::TWOVXF(
        STATE1.as_slice(),
        INDEX1,
        STATE2.as_slice(),
        INDEX2,
        XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADINDEX)", OK, ctx)?;

    INDEX1 = 1;
    INDEX2 = 0;

    spicelib::TWOVXF(
        STATE1.as_slice(),
        INDEX1,
        STATE2.as_slice(),
        INDEX2,
        XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADINDEX)", OK, ctx)?;

    INDEX1 = 1;
    INDEX2 = 4;

    spicelib::TWOVXF(
        STATE1.as_slice(),
        INDEX1,
        STATE2.as_slice(),
        INDEX2,
        XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADINDEX)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"TWOVXF: indices match", ctx)?;

    spicelib::VPACK(0.0, 0.0, 1.0, STATE1.subarray_mut(1));
    spicelib::VPACK(-1.0, 0.0, 0.0, STATE1.subarray_mut(4));

    spicelib::VPACK(1.0, 0.0, 1.0, STATE2.subarray_mut(1));
    spicelib::VPACK(0.0, 0.0, -1.0, STATE2.subarray_mut(4));

    INDEX1 = 1;
    INDEX2 = 1;

    spicelib::TWOVXF(
        STATE1.as_slice(),
        INDEX1,
        STATE2.as_slice(),
        INDEX2,
        XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(UNDEFINEDFRAME)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"TWOVXF: linearly dependent positions", ctx)?;

    spicelib::VPACK(0.0, 0.0, 1.0, STATE1.subarray_mut(1));
    spicelib::VPACK(-1.0, 0.0, 0.0, STATE1.subarray_mut(4));

    INDEX1 = 1;
    INDEX2 = 2;

    spicelib::TWOVXF(
        STATE1.as_slice(),
        INDEX1,
        STATE1.as_slice(),
        INDEX2,
        XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DEPENDENTVECTORS)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"ZZTWOVXF: Bad index", ctx)?;

    spicelib::VPACK(0.0, 0.0, 1.0, STATE1.subarray_mut(1));
    spicelib::VPACK(-1.0, 0.0, 0.0, STATE1.subarray_mut(4));

    spicelib::VPACK(1.0, 0.0, 1.0, STATE2.subarray_mut(1));
    spicelib::VPACK(0.0, 0.0, -1.0, STATE2.subarray_mut(4));

    INDEX1 = 0;
    INDEX2 = 2;

    spicelib::ZZTWOVXF(
        STATE1.as_slice(),
        INDEX1,
        STATE2.as_slice(),
        INDEX2,
        XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADINDEX)", OK, ctx)?;

    INDEX1 = 4;
    INDEX2 = 2;

    spicelib::ZZTWOVXF(
        STATE1.as_slice(),
        INDEX1,
        STATE2.as_slice(),
        INDEX2,
        XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADINDEX)", OK, ctx)?;

    INDEX1 = 1;
    INDEX2 = 0;

    spicelib::ZZTWOVXF(
        STATE1.as_slice(),
        INDEX1,
        STATE2.as_slice(),
        INDEX2,
        XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADINDEX)", OK, ctx)?;

    INDEX1 = 1;
    INDEX2 = 4;

    spicelib::ZZTWOVXF(
        STATE1.as_slice(),
        INDEX1,
        STATE2.as_slice(),
        INDEX2,
        XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADINDEX)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"ZZTWOVXF: indices match", ctx)?;

    spicelib::VPACK(0.0, 0.0, 1.0, STATE1.subarray_mut(1));
    spicelib::VPACK(-1.0, 0.0, 0.0, STATE1.subarray_mut(4));

    spicelib::VPACK(1.0, 0.0, 1.0, STATE2.subarray_mut(1));
    spicelib::VPACK(0.0, 0.0, -1.0, STATE2.subarray_mut(4));

    INDEX1 = 1;
    INDEX2 = 1;

    spicelib::ZZTWOVXF(
        STATE1.as_slice(),
        INDEX1,
        STATE2.as_slice(),
        INDEX2,
        XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(UNDEFINEDFRAME)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"ZZTWOVXF: linearly dependent positions", ctx)?;

    spicelib::VPACK(0.0, 0.0, 1.0, STATE1.subarray_mut(1));
    spicelib::VPACK(-1.0, 0.0, 0.0, STATE1.subarray_mut(4));

    INDEX1 = 1;
    INDEX2 = 2;

    spicelib::ZZTWOVXF(
        STATE1.as_slice(),
        INDEX1,
        STATE1.as_slice(),
        INDEX2,
        XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DEPENDENTVECTORS)", OK, ctx)?;

    //
    // *****************************************************************
    //
    // Normal cases: TWOVXF
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"TWOVXF: identity orientation, rotation about Z", ctx)?;

    spicelib::VPACK(1.0, 0.0, 0.0, STATE1.subarray_mut(1));
    spicelib::VPACK(0.0, 1.0, 0.0, STATE1.subarray_mut(4));

    spicelib::VPACK(0.0, 1.0, 0.0, STATE2.subarray_mut(1));
    spicelib::VPACK(-1.0, 0.0, 0.0, STATE2.subarray_mut(4));

    INDEX1 = 1;
    INDEX2 = 2;

    spicelib::TWOVXF(
        STATE1.as_slice(),
        INDEX1,
        STATE2.as_slice(),
        INDEX2,
        XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::IDENT(XR.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::XF2RAV(XFORM.as_slice(), R.as_slice_mut(), AV.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(b"R", R.as_slice(), b"~", XR.as_slice(), 9, VTIGHT, OK, ctx)?;

    //
    // Construct the expected angular velocity vector of
    // the reference frame constructed by TWOVXF. Units
    // are radians/s.
    //
    spicelib::VPACK(0.0, 0.0, 1.0, XAV.as_slice_mut());

    testutil::CHCKAD(
        b"AV",
        AV.as_slice(),
        b"~",
        XAV.as_slice(),
        3,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"ZZTWOVXF: identity orientation, rotation about Z", ctx)?;

    //
    // Compare against results from TWOVXF. The matrix constructed
    // by ZZTWOVXF should be the inverse of that produced by TWOVXF.
    //
    spicelib::INVSTM(XFORM.as_slice(), XXFORM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZTWOVXF(
        STATE1.as_slice(),
        INDEX1,
        STATE2.as_slice(),
        INDEX2,
        XFORM.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKAD(
        b"XFORM",
        XFORM.as_slice(),
        b"~",
        XXFORM.as_slice(),
        36,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // --- Case: -------------------------------------------------------
    //
    // Normal cases using pseudo-random vectors
    //
    testutil::TCASE(b"Random setup", ctx)?;

    //
    // Initialize the pseudo-random number generator.
    //
    SEED = -1;

    //
    // UB and LB are bounds for the exponent scale.
    //
    UB = 150.0;
    LB = -150.0;

    NCASE = 1000;
    CASENO = 0;

    for J in 1..=NCASE {
        //
        // Loop over the index of the primary vector.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = 3;
            let m3__: i32 = 1;
            INDEX1 = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                CASENO = (CASENO + 1);
                //
                // Let the secondary vector be that following the primary in
                // the right-handed sense.
                //
                INDEX2 = save.NEXT[INDEX1];

                fstr::assign(
                    &mut TITLE,
                    b"Random case #: TWOVXF/ZZTWOVXF random: primary = #, secondary = #.",
                );
                spicelib::REPMI(&TITLE.clone(), b"#", CASENO, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", INDEX1, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", INDEX2, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                //
                // Generate position and velocity components for the primary
                // state vector.
                //
                PSCAL1 = f64::powf(10.0, testutil::T_RANDD(LB, UB, &mut SEED, ctx)?);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                for I in 1..=3 {
                    STATE1[I] = (testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)? * PSCAL1);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                VSCAL1 = f64::powf(10.0, testutil::T_RANDD(LB, UB, &mut SEED, ctx)?);

                for I in 4..=6 {
                    STATE1[I] = (testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)? * VSCAL1);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                //
                // Generate position and velocity components for the secondary
                // state vector.
                //
                PSCAL2 = f64::powf(10.0, testutil::T_RANDD(LB, UB, &mut SEED, ctx)?);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                for I in 1..=3 {
                    STATE2[I] = (testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)? * PSCAL2);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                VSCAL2 = f64::powf(10.0, testutil::T_RANDD(LB, UB, &mut SEED, ctx)?);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                for I in 4..=6 {
                    STATE2[I] = (testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)? * VSCAL2);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                //
                // We may have inadvertently created nearly linearly
                // dependent vectors. Alter the second position vector
                // if this is the case.
                //
                if (spicelib::VSEP(STATE1.as_slice(), STATE2.as_slice(), ctx) < SEPTOL) {
                    STATE2[1] = (STATE2[1] * 0.9);
                    STATE2[2] = (STATE2[2] * 0.8);
                    STATE2[3] = (STATE2[3] * 0.7);
                }

                //
                // Compute the state transformation matrix.
                //
                spicelib::TWOVXF(
                    STATE1.as_slice(),
                    INDEX1,
                    STATE2.as_slice(),
                    INDEX2,
                    XFORM.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Compare results to those from ZZTWOVXF.
                //
                spicelib::ZZTWOVXF(
                    STATE1.as_slice(),
                    INDEX1,
                    STATE2.as_slice(),
                    INDEX2,
                    XFORMI.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::INVSTM(XFORMI.as_slice(), XXFORM.as_slice_mut(), ctx)?;

                testutil::CHCKAD(
                    b"XFORM (vs ZZ)",
                    XFORM.as_slice(),
                    b"~",
                    XXFORM.as_slice(),
                    36,
                    VTIGHT,
                    OK,
                    ctx,
                )?;
                //
                // Check the rotation portions of the transformation.
                // Check both rotation blocks.
                //
                spicelib::TWOVEC(
                    STATE1.as_slice(),
                    INDEX1,
                    STATE2.as_slice(),
                    INDEX2,
                    XR.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                for I in 1..=3 {
                    spicelib::VEQU(XFORM.subarray([1, I]), R.subarray_mut([1, I]));
                }

                testutil::CHCKAD(
                    b"R (UL)",
                    R.as_slice(),
                    b"~",
                    XR.as_slice(),
                    9,
                    VTIGHT,
                    OK,
                    ctx,
                )?;

                for I in 1..=3 {
                    spicelib::VEQU(XFORM.subarray([4, (3 + I)]), R.subarray_mut([1, I]));
                }

                testutil::CHCKAD(
                    b"R (LR)",
                    R.as_slice(),
                    b"~",
                    XR.as_slice(),
                    9,
                    VTIGHT,
                    OK,
                    ctx,
                )?;

                //
                // Check the upper right block.
                //
                spicelib::CLEARD(9, XR.as_slice_mut());

                for I in 1..=3 {
                    spicelib::VEQU(XFORM.subarray([1, (3 + I)]), R.subarray_mut([1, I]));
                }

                testutil::CHCKAD(
                    b"R (UR)",
                    R.as_slice(),
                    b"~",
                    XR.as_slice(),
                    9,
                    VTIGHT,
                    OK,
                    ctx,
                )?;

                //
                // Check the derivative block.
                //
                for I in 1..=3 {
                    spicelib::VEQU(XFORM.subarray([4, I]), DR.subarray_mut([1, I]));
                }

                //
                // We'll need to compute the expected derivative. For that,
                // we'll need the velocities of basis vectors of the "to"
                // frame.
                //
                // Note that the axis order here is dependent on the
                // selection of primary and secondary vectors. In this
                // case the secondary vector follows the primary one
                // in the right-handed sense.
                //
                // Start with the primary axis.
                //
                spicelib::DVHAT(STATE1.as_slice(), UXSTAT.as_slice_mut());

                for I in 1..=3 {
                    XR[[INDEX1, I]] = UXSTAT[(3 + I)];
                }

                //
                // Next find the state of the unit cross product
                // of the primary and secondary axes.
                //
                spicelib::DUCRSS(STATE1.as_slice(), STATE2.as_slice(), UXSTAT.as_slice_mut());
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                for I in 1..=3 {
                    XR[[save.PREV[INDEX1], I]] = UXSTAT[(3 + I)];
                }

                //
                // Next find the state of the unit cross product
                // of the third axis and the primary vector.
                //
                spicelib::DUCRSS(UXSTAT.as_slice(), STATE1.as_slice(), UXSTA2.as_slice_mut());
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                for I in 1..=3 {
                    XR[[INDEX2, I]] = UXSTA2[(3 + I)];
                }

                testutil::CHCKAD(
                    b"(d/dt)R",
                    DR.as_slice(),
                    b"~",
                    XR.as_slice(),
                    9,
                    VTIGHT,
                    OK,
                    ctx,
                )?;

                //
                // --- Case: -------------------------------------------------------
                //
                //       Let the secondary vector be that preceding the primary in
                //       the right-handed sense.
                //
                INDEX2 = save.PREV[INDEX1];

                fstr::assign(
                    &mut TITLE,
                    b"Random case #: TWOVXF/ZZTWOVXF random: primary = #, secondary = #.",
                );
                spicelib::REPMI(&TITLE.clone(), b"#", CASENO, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", INDEX1, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", INDEX2, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                //
                // Generate position and velocity components for the primary
                // state vector.
                //
                PSCAL1 = f64::powf(10.0, testutil::T_RANDD(LB, UB, &mut SEED, ctx)?);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                for I in 1..=3 {
                    STATE1[I] = (testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)? * PSCAL1);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                VSCAL1 = f64::powf(10.0, testutil::T_RANDD(LB, UB, &mut SEED, ctx)?);

                for I in 4..=6 {
                    STATE1[I] = (testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)? * VSCAL1);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                //
                // Generate position and velocity components for the secondary
                // state vector.
                //
                PSCAL2 = f64::powf(10.0, testutil::T_RANDD(LB, UB, &mut SEED, ctx)?);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                for I in 1..=3 {
                    STATE2[I] = (testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)? * PSCAL2);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                VSCAL2 = f64::powf(10.0, testutil::T_RANDD(LB, UB, &mut SEED, ctx)?);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                for I in 4..=6 {
                    STATE2[I] = (testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)? * VSCAL2);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                //
                // We may have inadvertently created nearly linearly
                // dependent vectors. Alter the second position vector
                // if this is the case.
                //
                if (spicelib::VSEP(STATE1.as_slice(), STATE2.as_slice(), ctx) < SEPTOL) {
                    STATE2[1] = (STATE2[1] * 0.9);
                    STATE2[2] = (STATE2[2] * 0.8);
                    STATE2[3] = (STATE2[3] * 0.7);
                }

                //
                // Compute the state transformation matrix.
                //
                spicelib::TWOVXF(
                    STATE1.as_slice(),
                    INDEX1,
                    STATE2.as_slice(),
                    INDEX2,
                    XFORM.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Compare results to those from ZZTWOVXF.
                //
                spicelib::ZZTWOVXF(
                    STATE1.as_slice(),
                    INDEX1,
                    STATE2.as_slice(),
                    INDEX2,
                    XFORMI.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::INVSTM(XFORMI.as_slice(), XXFORM.as_slice_mut(), ctx)?;

                testutil::CHCKAD(
                    b"XFORM (vs ZZ)",
                    XFORM.as_slice(),
                    b"~",
                    XXFORM.as_slice(),
                    36,
                    VTIGHT,
                    OK,
                    ctx,
                )?;

                //
                // Check the rotation portions of the transformation.
                // Check both rotation blocks.
                //
                spicelib::TWOVEC(
                    STATE1.as_slice(),
                    INDEX1,
                    STATE2.as_slice(),
                    INDEX2,
                    XR.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                for I in 1..=3 {
                    spicelib::VEQU(XFORM.subarray([1, I]), R.subarray_mut([1, I]));
                }

                testutil::CHCKAD(
                    b"R (UL)",
                    R.as_slice(),
                    b"~",
                    XR.as_slice(),
                    9,
                    VTIGHT,
                    OK,
                    ctx,
                )?;

                for I in 1..=3 {
                    spicelib::VEQU(XFORM.subarray([4, (3 + I)]), R.subarray_mut([1, I]));
                }

                testutil::CHCKAD(
                    b"R (LR)",
                    R.as_slice(),
                    b"~",
                    XR.as_slice(),
                    9,
                    VTIGHT,
                    OK,
                    ctx,
                )?;

                //
                // Check the upper right block.
                //
                spicelib::CLEARD(9, XR.as_slice_mut());

                for I in 1..=3 {
                    spicelib::VEQU(XFORM.subarray([1, (3 + I)]), R.subarray_mut([1, I]));
                }

                testutil::CHCKAD(
                    b"R (UR)",
                    R.as_slice(),
                    b"~",
                    XR.as_slice(),
                    9,
                    VTIGHT,
                    OK,
                    ctx,
                )?;

                //
                // Check the derivative block.
                //
                for I in 1..=3 {
                    spicelib::VEQU(XFORM.subarray([4, I]), DR.subarray_mut([1, I]));
                }

                //
                // We'll need to compute the expected derivative. For that,
                // we'll need the velocities of basis vectors of the "to"
                // frame.
                //
                // Note that the axis order here is dependent on the
                // selection of primary and secondary vectors. In this
                // case the secondary vector precedes the primary one
                // in the right-handed sense.
                //
                // Start with the primary axis.
                //
                spicelib::DVHAT(STATE1.as_slice(), UXSTAT.as_slice_mut());

                for I in 1..=3 {
                    XR[[INDEX1, I]] = UXSTAT[(3 + I)];
                }

                //
                // Next find the state of the unit cross product
                // of the secondary and primary axes.
                //
                spicelib::DUCRSS(STATE2.as_slice(), STATE1.as_slice(), UXSTAT.as_slice_mut());
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                for I in 1..=3 {
                    XR[[save.NEXT[INDEX1], I]] = UXSTAT[(3 + I)];
                }

                //
                // Next find the state of the unit cross product of the
                // primary vector and the third axis.
                //
                spicelib::DUCRSS(STATE1.as_slice(), UXSTAT.as_slice(), UXSTA2.as_slice_mut());
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                for I in 1..=3 {
                    XR[[INDEX2, I]] = UXSTA2[(3 + I)];
                }

                testutil::CHCKAD(
                    b"(d/dt)R",
                    DR.as_slice(),
                    b"~",
                    XR.as_slice(),
                    9,
                    VTIGHT,
                    OK,
                    ctx,
                )?;

                INDEX1 += m3__;
            }
        }
    }

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
