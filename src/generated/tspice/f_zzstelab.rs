//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;
const NUMLIM: f64 = 0.000001;
const NRSNGL: f64 = 0.000001;
const ABSTOL: f64 = 0.000001;
const TIGHT: f64 = 0.0000000000001;
const VTIGHT: f64 = 0.00000000000001;
const MEDIUM: f64 = 0.0000000001;
const LOW: f64 = 0.0001;
const LNSIZE: i32 = 80;
const NCASES: i32 = 20;
const SSB: i32 = 0;
const NSAMP: i32 = 15;

//$Procedure      F_ZZSTELAB ( Tests for stellar aberration corrections )
pub fn F_ZZSTELAB(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut TITLE = [b' '; LNSIZE as usize];
    let mut ACC = StackArray::<f64, 3>::new(1..=3);
    let mut ANGLES = StackArray::<f64, 15>::new(1..=NSAMP);
    let mut DPCOR2 = StackArray::<f64, 3>::new(1..=3);
    let mut DPCORR = StackArray::<f64, 3>::new(1..=3);
    let mut E1 = StackArray::<f64, 3>::new(1..=3);
    let mut E2 = StackArray::<f64, 3>::new(1..=3);
    let mut E3 = StackArray::<f64, 3>::new(1..=3);
    let mut PCORR = StackArray::<f64, 3>::new(1..=3);
    let mut PCORR2 = StackArray::<f64, 3>::new(1..=3);
    let mut POBS = StackArray::<f64, 3>::new(1..=3);
    let mut POBS0 = StackArray::<f64, 3>::new(1..=3);
    let mut PTARG = StackArray::<f64, 3>::new(1..=3);
    let mut PTARG0 = StackArray::<f64, 3>::new(1..=3);
    let mut R: f64 = 0.0;
    let mut RMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut S: f64 = 0.0;
    let mut SEP: f64 = 0.0;
    let mut STARG = StackArray::<f64, 6>::new(1..=6);
    let mut T: f64 = 0.0;
    let mut T0: f64 = 0.0;
    let mut TAU: f64 = 0.0;
    let mut TDELTA: f64 = 0.0;
    let mut UPOBS = StackArray::<f64, 3>::new(1..=3);
    let mut UVOBS0 = StackArray::<f64, 3>::new(1..=3);
    let mut VOBS = StackArray::<f64, 3>::new(1..=3);
    let mut VTARG = StackArray::<f64, 3>::new(1..=3);
    let mut W: f64 = 0.0;
    let mut Q: i32 = 0;
    let mut RM: i32 = 0;
    let mut XMIT: bool = false;

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
    // NUMLIM is the approximate limiting value of the angular
    // separation of the observer's velocity and the observer-
    // target position vector used by ZZSTELAB to determine
    // whether to compute the stellar aberration correction
    // velocity analytically or numerically. The numerical
    // branch is taken when the separation angle is less than
    // NUMLIM.
    //

    //
    // Tolerance level that can be used with DE-418 on PC/Linux/g77
    // platform: 5.D-8
    //

    //
    // Tolerance level that can be used with DE-418 on PC/Linux/g77
    // platform: 5.D-8
    //

    //
    // Local Variables
    //

    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZSTELAB", ctx)?;

    //
    // We're going to do all of our work in a reference frame that's
    // not aligned with the standard basis. The basis vectors of our
    // working frame will be called E1, E2, and E3.
    //
    spicelib::EUL2M(0.2, -0.3, 0.4, 1, 2, 3, RMAT.as_slice_mut(), ctx)?;

    spicelib::VEQU(RMAT.subarray([1, 1]), E1.as_slice_mut());
    spicelib::VEQU(RMAT.subarray([1, 2]), E2.as_slice_mut());
    spicelib::VEQU(RMAT.subarray([1, 3]), E3.as_slice_mut());

    //
    // We're going to work with an artificial ephemeris defined
    // by a simple, analytic formula. There is an observer whose
    // orbit is circular. The observer's orbit is centered at
    // the origin of our frame, and the radius of the orbit is
    // about that of the Earth's orbit. There is a target whose
    // motion is either linear or which does not move.
    //
    // We'll let the observer's orbital radius be R and its
    // orbital period be TAU. The orbital plane will be that
    // spanned by E1 and E2. The observer's location at time T
    // will be given by
    //
    //    POS(T) = R * ( cos( W*T )*E1  +  sin ( W*T )*E2 )
    //
    // where
    //
    //    W = 2 * Pi / TAU
    //
    //
    // The observer's velocity has the formula
    //
    //    W * R * ( -sin( W*T )*E1  +  cos( W*T )*E2 )
    //
    // and the observer's acceleration has the formula
    //
    //       2
    //    - W  * R * ( cos( W*T )*E1 + sin( W*T )*E2 )
    //
    //
    // Since we wish to avoid using time values too close to
    // zero, we'll let the central epoch of each sequence of
    // test samples be
    //
    //    T0 = TAU / 6.
    //
    // At epoch TAU the observer is located at
    //
    //    ( R/2, R*SQRT(3)/2, 0 )
    //
    // and is moving in the direction
    //
    //    ( -R*SQRT(3)/2, R/2, 0 )
    //
    // For all target motion cases, we'll place the target at the point
    // at distance 2*R from the observer along the observer's velocity
    // vector at epoch T0. We'll call this location PTARG0.
    //
    R = 150000000.0;

    TAU = spicelib::JYEAR();

    W = (((2 as f64) * spicelib::PI(ctx)) / TAU);

    T0 = (TAU / 6 as f64);

    //
    // Compute the observer and target positions at T0.
    //
    spicelib::VLCOM(
        (R / 2.0),
        E1.as_slice(),
        ((R * f64::sqrt(3.0)) / 2.0),
        E2.as_slice(),
        POBS0.as_slice_mut(),
    );

    spicelib::VLCOM(
        -((R * f64::sqrt(3.0)) / 2.0),
        E1.as_slice(),
        (R / 2.0),
        E2.as_slice(),
        VOBS.as_slice_mut(),
    );

    spicelib::VHAT(VOBS.as_slice(), UVOBS0.as_slice_mut());

    spicelib::VLCOM(
        1.0,
        POBS0.as_slice(),
        ((2 as f64) * R),
        UVOBS0.as_slice(),
        PTARG0.as_slice_mut(),
    );

    //
    // For all of our test cases, we'll choose times such that the
    // angular separation of the observer's velocity and the observer-
    // target vector SEP passes through the set of values (units are
    // radians):
    //
    //    -1.D-5, -1.D-6, ..., -1.D-11, 0, 1.D-11, ..., 1.D5
    //
    // We'll store these values in the array ANGLES.
    //
    ANGLES[8] = 0.0;

    for I in 1..=((NSAMP - 1) / 2) {
        ANGLES[I] = -f64::powf(10.0, -(4.0 + I as f64));
        ANGLES[((NSAMP + 1) - I)] = f64::powf(10.0, -(4.0 + I as f64));
    }

    for CN in 1..=NCASES {
        //
        // Use reception corrections for the odd-numbered cases;
        // use transmission corrections for the even-numbered ones.
        //
        XMIT = spicelib::EVEN(CN);

        //
        // For the second set of cases, negate the observer's
        // velocity and acceleration.
        //
        if (CN <= (NCASES / 2)) {
            S = 1.0;
        } else {
            S = -1.0;
        }

        spicelib::RMAINI(CN, (NCASES / 2), &mut Q, &mut RM, ctx)?;
        //
        // For odd values of RM, we change the target's velocity.
        // For even values we keep the geometry constant and use
        // transmission corrections.
        //
        // Note that the 10th and 20th cases correspond to RM values
        // of zero.
        //

        if (RM == 1) {
            //
            // Set the target's velocity to zero. The target rests at
            // PTARG0.
            //
            spicelib::CLEARD(3, VTARG.as_slice_mut());
        } else if (RM == 3) {
            //
            // Set the target's velocity to 30 km/s in the direction
            // parallel to the observer's position vector (relative to the
            // origin) at T0 (hence orthogonal to the observer's velocity
            // and anti-parallel to the observer's acceleration).
            //
            spicelib::VHAT(PTARG0.as_slice(), UPOBS.as_slice_mut());

            spicelib::VSCL(30.0, UPOBS.as_slice(), VTARG.as_slice_mut());
        } else if (RM == 5) {
            //
            // Set the target's velocity to 30 km/s in the direction
            // anit-parallel to the observer's position vector (relative
            // to the origin) at T0 (hence orthogonal to the observer's
            // velocity and anti-parallel to the observer's acceleration).
            //
            spicelib::VHAT(PTARG0.as_slice(), UPOBS.as_slice_mut());

            spicelib::VSCL(-30.0, UPOBS.as_slice(), VTARG.as_slice_mut());
        } else if (RM == 7) {
            //
            // Set the target's velocity to 30 km/s in the direction
            // parallel to the observer's velocity vector (relative
            // to the origin) at T0.
            //
            spicelib::VSCL(30.0, UVOBS0.as_slice(), VTARG.as_slice_mut());
        } else if (RM == 9) {
            //
            // Set the target's velocity to 30 km/s in the direction
            // anti-parallel to the observer's velocity vector (relative
            // to the origin) at T0.
            //
            spicelib::VSCL(-30.0, UVOBS0.as_slice(), VTARG.as_slice_mut());
        }

        for SN in 1..=NSAMP {
            //
            //---- Case -------------------------------------------------------------
            //

            //
            // Note: the TCASE call setting the test case title will
            // be made AFTER we compute the separation angle; this
            // angle will appear in the title.
            //
            fstr::assign(&mut TITLE, b"Case #; sample number #, separation angle #");

            spicelib::REPMI(&TITLE.clone(), b"#", CN, &mut TITLE, ctx);
            spicelib::REPMI(&TITLE.clone(), b"#", SN, &mut TITLE, ctx);

            //
            // The time increment TDELTA refers to the difference
            //
            //    T - T0
            //
            // This difference is a function of the offset angle
            //
            //    ANGLES(SN)
            //
            // When the target is motionless, the offset angle is very
            // small, so the angle is primarily determined by the
            // observer's velocity direction, which changes with angular
            // velocity W. So we can approximate TDELTA by
            //
            //    ANGLES(SN) / W
            //
            // When the target is moving, this approximation still is
            // of the correct magnitude, so we'll use it for these
            // cases as well.
            //
            TDELTA = (ANGLES[SN] / W);

            T = (T0 + TDELTA);

            //
            // Compute the position, velocity, and acceleration of the
            // observer relative to the solar system barycenter (that
            // is, the origin of the frame).
            //
            spicelib::VLCOM(
                (R * f64::cos((W * T))),
                E1.as_slice(),
                (R * f64::sin((W * T))),
                E2.as_slice(),
                POBS.as_slice_mut(),
            );

            spicelib::VLCOM(
                -((W * R) * f64::sin((W * T))),
                E1.as_slice(),
                ((W * R) * f64::cos((W * T))),
                E2.as_slice(),
                VOBS.as_slice_mut(),
            );

            spicelib::VLCOM(
                -((f64::powi(W, 2) * R) * f64::cos((W * T))),
                E1.as_slice(),
                -((f64::powi(W, 2) * R) * f64::sin((W * T))),
                E2.as_slice(),
                ACC.as_slice_mut(),
            );

            //
            // Adjust the signs of the velocity and acceleration.
            //
            spicelib::VSCLIP(S, VOBS.as_slice_mut());
            spicelib::VSCLIP(S, ACC.as_slice_mut());

            //
            // Compute the target's position.
            //
            spicelib::VLCOM(
                1.0,
                PTARG0.as_slice(),
                TDELTA,
                VTARG.as_slice(),
                PTARG.as_slice_mut(),
            );

            //
            // Compute the target's state relative to the observer.
            //
            spicelib::VSUB(PTARG.as_slice(), POBS.as_slice(), STARG.as_slice_mut());
            spicelib::VSUB(VTARG.as_slice(), VOBS.as_slice(), STARG.subarray_mut(4));

            //
            // If this case is supposed to be singular, make sure
            // the observer's velocity is pointed directly toward
            // or away from the target. Due to round-off errors,
            // we cannot ensure that the angular separation of
            // the observer's velocity and the observer-target
            // position vector is exactly zero unless we play
            // some dirty tricks.
            //
            if (SN == 8) {
                if (spicelib::VDOT(VOBS.as_slice(), STARG.as_slice()) >= 0.0) {
                    spicelib::VEQU(VOBS.as_slice(), STARG.as_slice_mut());
                } else {
                    spicelib::VMINUS(VOBS.as_slice(), STARG.as_slice_mut());
                }
            }

            //
            // Check the angular separation of the observer's velocity
            // and the observer-target position vector.
            //
            SEP = spicelib::VSEP(VOBS.as_slice(), STARG.as_slice(), ctx);

            spicelib::REPMD(&TITLE.clone(), b"#", SEP, 14, &mut TITLE, ctx);

            testutil::TCASE(&TITLE, ctx)?;

            // WRITE (*,*) '----------'
            // WRITE (*,*) 'SN         = ', SN
            // WRITE (*,*) 'ANGLES(SN) = ', ANGLES(SN)
            // WRITE (*,*) 'SEP        = ', SEP

            //
            // Compute the stellar aberration offset and velocity.
            //
            spicelib::ZZSTELAB(
                XMIT,
                ACC.as_slice(),
                VOBS.as_slice(),
                STARG.as_slice(),
                PCORR.as_slice_mut(),
                DPCORR.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            // WRITE (*,*) '||DPCORR|| = ', VNORM(DPCORR)
            // WRITE (*,*) '||PCORR||  = ', VNORM(PCORR)

            if (((spicelib::PI(ctx) - SEP) > 0.000000000001) && (SEP > 0.000000000001)) {
                //
                // Compare the results to our analytic formulation.
                //
                testutil::T_ZZSTELAB(
                    XMIT,
                    ACC.as_slice(),
                    VOBS.as_slice(),
                    STARG.as_slice(),
                    PCORR2.as_slice_mut(),
                    DPCOR2.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Test the relative position error.
                //
                testutil::CHCKAD(
                    b"PCORR",
                    PCORR.as_slice(),
                    b"~~/",
                    PCORR2.as_slice(),
                    3,
                    0.0001,
                    OK,
                    ctx,
                )?;

                //
                // Test the absolute position error.
                //
                testutil::CHCKAD(
                    b"PCORR",
                    PCORR.as_slice(),
                    b"~",
                    PCORR2.as_slice(),
                    3,
                    0.000000001,
                    OK,
                    ctx,
                )?;

                //
                // Test the relative velocity error.
                //
                testutil::CHCKAD(
                    b"DPCORR",
                    DPCORR.as_slice(),
                    b"~~/",
                    DPCOR2.as_slice(),
                    3,
                    0.0000001,
                    OK,
                    ctx,
                )?;

                //
                // Test the absolute velocity error.
                //
                testutil::CHCKAD(
                    b"DPCORR",
                    DPCORR.as_slice(),
                    b"~",
                    DPCOR2.as_slice(),
                    3,
                    0.000000001,
                    OK,
                    ctx,
                )?;
            }

            if (((spicelib::PI(ctx) - SEP) < NUMLIM) || (SEP < NUMLIM)) {
                //
                // Compare the results to our numeric formulation.
                // Ideally, we would expect EXACT matches, but
                // on the PC-CYGWIN_C platform, we don't get them.
                // So instead, look for small relative
                // errors.
                //
                testutil::T_ZZSTLABN(
                    XMIT,
                    ACC.as_slice(),
                    VOBS.as_slice(),
                    STARG.as_slice(),
                    PCORR2.as_slice_mut(),
                    DPCOR2.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Test the relative position and velocity errors.
                //
                testutil::CHCKAD(
                    b"PCORR",
                    PCORR.as_slice(),
                    b"~~/",
                    PCORR2.as_slice(),
                    3,
                    MEDIUM,
                    OK,
                    ctx,
                )?;

                testutil::CHCKAD(
                    b"DPCORR",
                    DPCORR.as_slice(),
                    b"~~/",
                    DPCOR2.as_slice(),
                    3,
                    MEDIUM,
                    OK,
                    ctx,
                )?;
            }
        }
        //
        // End of time loop.
        //
    }
    //
    // End of case loop.
    //

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
