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
const PCK: &[u8] = b"phoenix.tpc";
const SPK: &[u8] = b"phoenix.bsp";
const NRSNGL: f64 = 0.000001;
const ABSTOL: f64 = 0.000001;
const TIGHT: f64 = 0.0000000000001;
const VTIGHT: f64 = 0.00000000000001;
const MEDIUM: f64 = 0.0000000001;
const LOW: f64 = 0.0001;
const FRNMLN: i32 = 32;
const MAXDEG: i32 = 12;
const TIMLEN: i32 = 40;
const LNSIZE: i32 = 80;
const NCORR: i32 = 9;
const NFRAME: i32 = 3;
const SSB: i32 = 0;
const NBAD: i32 = 6;

struct SaveVars {
    BADCOR: ActualCharArray,
    CORLST: ActualCharArray,
    FRMLST: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BADCOR = ActualCharArray::new(CORLEN, 1..=NBAD);
        let mut CORLST = ActualCharArray::new(CORLEN, 1..=NCORR);
        let mut FRMLST = ActualCharArray::new(FRNMLN, 1..=NFRAME);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"S"),
                Val::C(b"RL"),
                Val::C(b"RL+S"),
                Val::C(b"XS"),
                Val::C(b"XRL"),
                Val::C(b"XRL+S"),
            ]
            .into_iter();
            BADCOR
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"NONE"),
                Val::C(b"LT"),
                Val::C(b"LT+S"),
                Val::C(b"CN"),
                Val::C(b"CN+S"),
                Val::C(b"XLT"),
                Val::C(b"XLT+S"),
                Val::C(b"XCN"),
                Val::C(b"XCN+S"),
            ]
            .into_iter();
            CORLST
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist =
                [Val::C(b"J2000"), Val::C(b"ECLIPJ2000"), Val::C(b"IAU_MOON")].into_iter();
            FRMLST
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            BADCOR,
            CORLST,
            FRMLST,
        }
    }
}

//$Procedure      F_SPKCOR ( Tests for SPK aberration corrections )
pub fn F_SPKCOR(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ABCORR = [b' '; CORLEN as usize];
    let mut REF = [b' '; FRNMLN as usize];
    let mut TIMSTR = [b' '; TIMLEN as usize];
    let mut TITLE = [b' '; LNSIZE as usize];
    let mut ACC = StackArray::<f64, 6>::new(1..=6);
    let mut COEFFS = StackArray::<f64, 13>::new(0..=MAXDEG);
    let mut CORPOS = StackArray::<f64, 3>::new(1..=3);
    let mut DELTA: f64 = 0.0;
    let mut DERIVS = StackArray::<f64, 3>::new(0..=2);
    let mut DLT: f64 = 0.0;
    let mut DLT0: f64 = 0.0;
    let mut DLT1: f64 = 0.0;
    let mut DPCOR2 = StackArray::<f64, 3>::new(1..=3);
    let mut DPCORR = StackArray::<f64, 3>::new(1..=3);
    let mut DPOS = StackArray::<f64, 3>::new(1..=3);
    let mut ET: f64 = 0.0;
    let mut ET0: f64 = 0.0;
    let mut HALFWN: f64 = 0.0;
    let mut LEFT: f64 = 0.0;
    let mut LT: f64 = 0.0;
    let mut LT0: f64 = 0.0;
    let mut LT1: f64 = 0.0;
    let mut LTSSB: f64 = 0.0;
    let mut PARTDP = StackArray2D::<f64, 39>::new(1..=3, 1..=(MAXDEG + 1));
    let mut PCORR = StackArray::<f64, 3>::new(1..=3);
    let mut PCORR2 = StackArray::<f64, 3>::new(1..=3);
    let mut POS = StackArray::<f64, 3>::new(1..=3);
    let mut RETVAL: f64 = 0.0;
    let mut RIGHT: f64 = 0.0;
    let mut SSBOBS = StackArray2D::<f64, 12>::new(1..=6, 1..=2);
    let mut STATE = StackArray::<f64, 6>::new(1..=6);
    let mut STATE0 = StackArray::<f64, 6>::new(1..=6);
    let mut STATE1 = StackArray::<f64, 6>::new(1..=6);
    let mut STOBS = StackArray::<f64, 6>::new(1..=6);
    let mut T: f64 = 0.0;
    let mut TDELTA: f64 = 0.0;
    let mut WORK = StackArray::<f64, 13>::new(0..=MAXDEG);
    let mut X2S = StackArray::<f64, 2>::new(1..=2);
    let mut DEGREE: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut MAXN: i32 = 0;
    let mut NSAMP: i32 = 0;
    let mut OBSRVR: i32 = 0;
    let mut TARGET: i32 = 0;
    let mut ATTBLK = StackArray::<bool, 15>::new(1..=NABCOR);
    let mut XMIT: bool = false;

    //
    // Test Utility Functions
    //

    //
    // SPICELIB Functions
    //

    //
    // External functions
    //

    //
    // Local parameters
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

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_SPKCOR", ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Create kernel files.", ctx)?;

    testutil::KILFIL(SPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::KILFIL(PCK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTSPK(SPK, true, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTPCK(PCK, true, false, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // For improved numeric performance, use a DE-based SPK.
    //
    //  CALL FURNSH ( 'de418.bsp' )
    //

    //
    // The first set of tests covers the routine SPKEZ and
    // its frame system mirror routines.
    //
    for FR in 1..=NFRAME {
        for AC in 1..=NCORR {
            //
            //---- Case -------------------------------------------------------------
            //
            fstr::assign(
                &mut TITLE,
                b"SPKEZ: check Earth-Mars states; frame #, abcorr = #.",
            );

            OBSRVR = 399;
            TARGET = 499;
            fstr::assign(&mut REF, save.FRMLST.get(FR));
            fstr::assign(&mut ABCORR, save.CORLST.get(AC));

            spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
            spicelib::REPMC(&TITLE.clone(), b"#", &ABCORR, &mut TITLE);

            testutil::TCASE(&TITLE, ctx)?;
            //
            // Set up our ephemeris utilities to enable Cheby fitting.
            //
            RETVAL = testutil::T_STCINI(TARGET, &REF, &ABCORR, OBSRVR, ctx);

            //
            // Take four years' worth of samples, spaced 2 months apart.
            //
            TDELTA = (spicelib::JYEAR() / 6 as f64);
            NSAMP = 24;
            ET0 = 0.0;
            HALFWN = 300.0;
            DEGREE = 12;
            MAXN = (DEGREE + 1);

            for I in 1..=NSAMP {
                ET = (ET0 + (((I - 1) as f64) * TDELTA));

                spicelib::SPKEZ(
                    TARGET,
                    ET,
                    &REF,
                    &ABCORR,
                    OBSRVR,
                    STATE.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                //
                // Check ZZSPKEZ0: state should match that
                // from SPKEZ.
                //
                spicelib::ZZSPKEZ0(
                    TARGET,
                    ET,
                    &REF,
                    &ABCORR,
                    OBSRVR,
                    STATE0.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"STATE0",
                    STATE0.as_slice(),
                    b"=",
                    STATE.as_slice(),
                    6,
                    0.0,
                    OK,
                    ctx,
                )?;

                //
                // Check ZZSPKEZ1: state should match that
                // from SPKEZ.
                //
                spicelib::ZZSPKEZ1(
                    TARGET,
                    ET,
                    &REF,
                    &ABCORR,
                    OBSRVR,
                    STATE1.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"STATE1",
                    STATE1.as_slice(),
                    b"=",
                    STATE.as_slice(),
                    6,
                    0.0,
                    OK,
                    ctx,
                )?;

                //
                // We're going to test each position component of our state
                // vector to make sure our fitting approach is working.
                //
                for J in 1..=3 {
                    //
                    // Fit a Cheby expansion of degree MAXDEG to the Jth
                    // position component over a 2*HALFWN time span centered
                    // at ET. We'll retain an expansion of degree DEGREE.
                    //
                    if (J == 1) {
                        LEFT = (ET - HALFWN);
                        RIGHT = (ET + HALFWN);

                        support::CHBFIT(
                            testutil::T_GETX,
                            LEFT,
                            RIGHT,
                            (MAXDEG + 1),
                            WORK.as_slice_mut(),
                            COEFFS.as_slice_mut(),
                            ctx,
                        )?;
                    } else if (J == 2) {
                        support::CHBFIT(
                            testutil::T_GETY,
                            LEFT,
                            RIGHT,
                            (MAXDEG + 1),
                            WORK.as_slice_mut(),
                            COEFFS.as_slice_mut(),
                            ctx,
                        )?;
                    } else {
                        support::CHBFIT(
                            testutil::T_GETZ,
                            LEFT,
                            RIGHT,
                            (MAXDEG + 1),
                            WORK.as_slice_mut(),
                            COEFFS.as_slice_mut(),
                            ctx,
                        )?;
                    }

                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Find the derivatives of the current Cheby expansion.
                    //
                    X2S[1] = ET;
                    X2S[2] = HALFWN;

                    spicelib::CHBDER(
                        COEFFS.as_slice(),
                        (MAXN - 1),
                        X2S.as_slice(),
                        ET,
                        1,
                        PARTDP.as_slice_mut(),
                        DERIVS.as_slice_mut(),
                    );

                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    POS[1] = DERIVS[0];
                    DPOS[1] = DERIVS[1];

                    //
                    // Note: to compute acceleration, change the argument
                    // "1" above to "2" and capture acceleration from
                    //
                    //    DERIVS(2)
                    //

                    //
                    // Compare the expansion to the position from SPKEZ at
                    // ET.
                    //
                    spicelib::ETCAL(ET, &mut TIMSTR, ctx);
                    fstr::assign(&mut TITLE, b"Position component # at ET #.");

                    spicelib::REPMI(&TITLE.clone(), b"#", J, &mut TITLE, ctx);
                    spicelib::REPMC(&TITLE.clone(), b"#", &TIMSTR, &mut TITLE);

                    testutil::CHCKSD(&TITLE, STATE[J], b"~/", POS[1], TIGHT, OK, ctx)?;

                    //
                    // Compare the derivative of the expansion to the
                    // velocity from SPKEZ at ET.
                    //
                    spicelib::ETCAL(ET, &mut TIMSTR, ctx);
                    fstr::assign(&mut TITLE, b"Velocity component # at ET #.");

                    spicelib::REPMI(&TITLE.clone(), b"#", J, &mut TITLE, ctx);
                    spicelib::REPMC(&TITLE.clone(), b"#", &TIMSTR, &mut TITLE);

                    //
                    // For velocity components under 1 km/s, check
                    // the absolute error. We're looking for
                    // 1 mm/s agreement.
                    //
                    if (f64::abs(STATE[(J + 3)]) < 1.0) {
                        testutil::CHCKSD(&TITLE, STATE[(J + 3)], b"~", DPOS[1], ABSTOL, OK, ctx)?;
                    //
                    // For velocity components over 1 km/s, check
                    // the relative error.
                    //
                    } else {
                        testutil::CHCKSD(&TITLE, STATE[(J + 3)], b"~/", DPOS[1], NRSNGL, OK, ctx)?;
                    }
                }
                //
                // End of state component loop.
                //
            }
            //
            // End of epoch loop.
            //
        }
        //
        // End of aberration correction loop.
        //
    }
    //
    // End of frame loop.
    //

    //
    // The second set of tests covers the routine SPKLTC and its frame
    // system mirror routines. This routine requires an inertial input
    // frame, so we omit the cases for the last frame in the list.
    //
    for FR in 1..=(NFRAME - 1) {
        for AC in 1..=NCORR {
            //
            //---- Case -------------------------------------------------------------
            //
            fstr::assign(
                &mut TITLE,
                b"SPKLTC:  Earth-Mars light time/light time rate; frame #, abcorr = #.",
            );

            OBSRVR = 399;
            TARGET = 499;
            fstr::assign(&mut REF, save.FRMLST.get(FR));
            fstr::assign(&mut ABCORR, save.CORLST.get(AC));

            spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
            spicelib::REPMC(&TITLE.clone(), b"#", &ABCORR, &mut TITLE);

            testutil::TCASE(&TITLE, ctx)?;
            //
            // Set up our ephemeris utilities to enable Cheby fitting.
            //
            RETVAL = testutil::T_STCINI(TARGET, &REF, &ABCORR, OBSRVR, ctx);

            //
            // Take four years' worth of samples, spaced 2 months apart.
            //
            TDELTA = (spicelib::JYEAR() / 6 as f64);
            NSAMP = 24;
            ET0 = 0.0;
            HALFWN = 300.0;
            DEGREE = 12;
            MAXN = (DEGREE + 1);

            for I in 1..=NSAMP {
                ET = (ET0 + (((I - 1) as f64) * TDELTA));

                spicelib::SPKSSB(OBSRVR, ET, &REF, STOBS.as_slice_mut(), ctx)?;
                spicelib::SPKLTC(
                    TARGET,
                    ET,
                    &REF,
                    &ABCORR,
                    STOBS.as_slice(),
                    STATE.as_slice_mut(),
                    &mut LT,
                    &mut DLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Check SPKEZ: light time (but not state) should
                // match that from SPKLTC.
                //
                spicelib::SPKEZ(
                    TARGET,
                    ET,
                    &REF,
                    &ABCORR,
                    OBSRVR,
                    STATE0.as_slice_mut(),
                    &mut LT0,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::ETCAL(ET, &mut TIMSTR, ctx);
                fstr::assign(&mut TITLE, b"SPKEZ light time at ET #.");
                spicelib::REPMC(&TITLE.clone(), b"#", &TIMSTR, &mut TITLE);

                testutil::CHCKSD(&TITLE, LT0, b"~/", LT, TIGHT, OK, ctx)?;

                //
                // Check ZZSPKLT0: state, light time, light time rate
                // all should match those from SPKLTC.
                //
                spicelib::ZZSPKLT0(
                    TARGET,
                    ET,
                    &REF,
                    &ABCORR,
                    STOBS.as_slice(),
                    STATE0.as_slice_mut(),
                    &mut LT0,
                    &mut DLT0,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                fstr::assign(&mut TITLE, b"ZZSPKLT0 state at ET #.");
                spicelib::REPMC(&TITLE.clone(), b"#", &TIMSTR, &mut TITLE);
                testutil::CHCKAD(
                    &TITLE,
                    STATE0.as_slice(),
                    b"~~/",
                    STATE.as_slice(),
                    6,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                fstr::assign(&mut TITLE, b"ZZSPKLT0 light time at ET #.");
                spicelib::REPMC(&TITLE.clone(), b"#", &TIMSTR, &mut TITLE);
                testutil::CHCKSD(&TITLE, LT0, b"=", LT, 0.0, OK, ctx)?;

                fstr::assign(&mut TITLE, b"ZZSPKLT0 light time rate at ET #.");
                spicelib::REPMC(&TITLE.clone(), b"#", &TIMSTR, &mut TITLE);
                testutil::CHCKSD(&TITLE, DLT0, b"=", DLT, 0.0, OK, ctx)?;

                // Check ZZSPKLT1: state, light time, light time rate
                // all should match those from SPKLTC.
                //
                spicelib::ZZSPKLT1(
                    TARGET,
                    ET,
                    &REF,
                    &ABCORR,
                    STOBS.as_slice(),
                    STATE1.as_slice_mut(),
                    &mut LT1,
                    &mut DLT1,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                fstr::assign(&mut TITLE, b"ZZSPKLT1 state at ET #.");
                spicelib::REPMC(&TITLE.clone(), b"#", &TIMSTR, &mut TITLE);
                testutil::CHCKAD(
                    &TITLE,
                    STATE1.as_slice(),
                    b"~~/",
                    STATE.as_slice(),
                    6,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                fstr::assign(&mut TITLE, b"ZZSPKLT1 light time at ET #.");
                spicelib::REPMC(&TITLE.clone(), b"#", &TIMSTR, &mut TITLE);
                testutil::CHCKSD(&TITLE, LT1, b"=", LT, 0.0, OK, ctx)?;

                fstr::assign(&mut TITLE, b"ZZSPKLT1 light time rate at ET #.");
                spicelib::REPMC(&TITLE.clone(), b"#", &TIMSTR, &mut TITLE);
                testutil::CHCKSD(&TITLE, DLT1, b"=", DLT, 0.0, OK, ctx)?;

                //
                // Fit a Cheby expansion of degree MAXDEG to the light time
                // over a 2*HALFWN time span centered at ET. We'll retain
                // an expansion of degree DEGREE.
                //
                LEFT = (ET - HALFWN);
                RIGHT = (ET + HALFWN);

                support::CHBFIT(
                    testutil::T_GETLT,
                    LEFT,
                    RIGHT,
                    (MAXDEG + 1),
                    WORK.as_slice_mut(),
                    COEFFS.as_slice_mut(),
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Find the derivatives of the current Cheby expansion.
                //
                X2S[1] = ET;
                X2S[2] = HALFWN;

                spicelib::CHBDER(
                    COEFFS.as_slice(),
                    (MAXN - 1),
                    X2S.as_slice(),
                    ET,
                    1,
                    PARTDP.as_slice_mut(),
                    DERIVS.as_slice_mut(),
                );

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                POS[1] = DERIVS[0];
                DPOS[1] = DERIVS[1];

                //
                // Note: to compute acceleration, change the argument
                // "1" above to "2" and capture acceleration from
                //
                //    DERIVS(2)
                //

                //
                // Compare the expansion to the light time from SPKLTC at
                // ET.
                //
                fstr::assign(&mut TITLE, b"SPKLTC Light time at ET #.");
                spicelib::REPMC(&TITLE.clone(), b"#", &TIMSTR, &mut TITLE);

                testutil::CHCKSD(&TITLE, LT, b"~/", POS[1], TIGHT, OK, ctx)?;

                //
                // Compare the derivative of the expansion to the
                // light time from SPKLTC at ET.
                //
                fstr::assign(&mut TITLE, b"SPKLTC Light time rate at ET #.");
                spicelib::REPMC(&TITLE.clone(), b"#", &TIMSTR, &mut TITLE);

                testutil::CHCKSD(&TITLE, DLT, b"~/", DPOS[1], NRSNGL, OK, ctx)?;
            }
            //
            // End of epoch loop.
            //
        }
        //
        // End of aberration correction loop.
        //
    }
    //
    // End of frame loop.
    //

    //
    // The third set of tests covers the routines SPKACS, SPKAPS, and
    // their frame system mirror routines. These routines require an
    // inertial input frame, so we omit the cases for the last frame in
    // the list.
    //
    for FR in 1..=(NFRAME - 1) {
        for AC in 1..=NCORR {
            //
            //---- Case -------------------------------------------------------------
            //
            fstr::assign(
                &mut TITLE,
                b"SPKACS:  Earth-Mars state, light time/rate; frame #, abcorr = #.",
            );

            OBSRVR = 399;
            TARGET = 499;
            fstr::assign(&mut REF, save.FRMLST.get(FR));
            fstr::assign(&mut ABCORR, save.CORLST.get(AC));

            spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
            spicelib::REPMC(&TITLE.clone(), b"#", &ABCORR, &mut TITLE);

            testutil::TCASE(&TITLE, ctx)?;

            //
            // Take four years' worth of samples, spaced 2 months apart.
            //
            TDELTA = (spicelib::JYEAR() / 6 as f64);
            NSAMP = 24;
            ET0 = 0.0;

            for I in 1..=NSAMP {
                ET = (ET0 + (((I - 1) as f64) * TDELTA));

                //
                // Test SPKACS and its mirror routines:
                //
                spicelib::SPKACS(
                    TARGET,
                    ET,
                    &REF,
                    &ABCORR,
                    OBSRVR,
                    STATE.as_slice_mut(),
                    &mut LT,
                    &mut DLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::SPKSSB(OBSRVR, ET, &REF, STOBS.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::SPKLTC(
                    TARGET,
                    ET,
                    &REF,
                    &ABCORR,
                    STOBS.as_slice(),
                    STATE0.as_slice_mut(),
                    &mut LT0,
                    &mut DLT0,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                fstr::assign(&mut TITLE, b"SPKACS light time at ET #.");
                spicelib::REPMC(&TITLE.clone(), b"#", &TIMSTR, &mut TITLE);
                testutil::CHCKSD(&TITLE, LT, b"=", LT0, 0.0, OK, ctx)?;

                fstr::assign(&mut TITLE, b"SPKACS light time rate at ET #.");
                spicelib::REPMC(&TITLE.clone(), b"#", &TIMSTR, &mut TITLE);
                testutil::CHCKSD(&TITLE, DLT, b"=", DLT0, 0.0, OK, ctx)?;

                //
                // Check against SPKEZ: state should match that from SPKACS.
                //
                spicelib::SPKEZ(
                    TARGET,
                    ET,
                    &REF,
                    &ABCORR,
                    OBSRVR,
                    STATE1.as_slice_mut(),
                    &mut LT1,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::ETCAL(ET, &mut TIMSTR, ctx);
                fstr::assign(&mut TITLE, b"SPKACS state at ET #.");
                spicelib::REPMC(&TITLE.clone(), b"#", &TIMSTR, &mut TITLE);
                testutil::CHCKAD(
                    &TITLE,
                    STATE.as_slice(),
                    b"~~/",
                    STATE1.as_slice(),
                    6,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                //
                // Check against ZZSPKAC0: state, light time, and light
                // time rate should match those from SPKACS.
                //
                spicelib::ZZSPKAC0(
                    TARGET,
                    ET,
                    &REF,
                    &ABCORR,
                    OBSRVR,
                    STATE0.as_slice_mut(),
                    &mut LT0,
                    &mut DLT0,
                    ctx,
                )?;

                fstr::assign(&mut TITLE, b"ZZSPKAC0 light time at ET #.");
                spicelib::REPMC(&TITLE.clone(), b"#", &TIMSTR, &mut TITLE);
                testutil::CHCKSD(&TITLE, LT0, b"=", LT, 0.0, OK, ctx)?;

                fstr::assign(&mut TITLE, b"ZZSPKAC0 light time rate at ET #.");
                spicelib::REPMC(&TITLE.clone(), b"#", &TIMSTR, &mut TITLE);
                testutil::CHCKSD(&TITLE, DLT0, b"=", DLT, 0.0, OK, ctx)?;

                //
                // Check state from ZZSPKAC0: should match that from SPKACS.
                //
                spicelib::ETCAL(ET, &mut TIMSTR, ctx);
                fstr::assign(&mut TITLE, b"ZZSPKAC0 state at ET #.");
                spicelib::REPMC(&TITLE.clone(), b"#", &TIMSTR, &mut TITLE);
                testutil::CHCKAD(
                    &TITLE,
                    STATE0.as_slice(),
                    b"~~/",
                    STATE.as_slice(),
                    6,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                //
                // Check against ZZSPKAC1: state, light time, and light
                // time rate should match those from SPKACS.
                //
                spicelib::ZZSPKAC1(
                    TARGET,
                    ET,
                    &REF,
                    &ABCORR,
                    OBSRVR,
                    STATE1.as_slice_mut(),
                    &mut LT1,
                    &mut DLT1,
                    ctx,
                )?;

                fstr::assign(&mut TITLE, b"ZZSPKAC1 light time at ET #.");
                spicelib::REPMC(&TITLE.clone(), b"#", &TIMSTR, &mut TITLE);
                testutil::CHCKSD(&TITLE, LT1, b"=", LT, 0.0, OK, ctx)?;

                fstr::assign(&mut TITLE, b"ZZSPKAC1 light time rate at ET #.");
                spicelib::REPMC(&TITLE.clone(), b"#", &TIMSTR, &mut TITLE);
                testutil::CHCKSD(&TITLE, DLT1, b"=", DLT, 0.0, OK, ctx)?;

                //
                // Check state from ZZSPKAC1: should match that from SPKACS.
                //
                spicelib::ETCAL(ET, &mut TIMSTR, ctx);
                fstr::assign(&mut TITLE, b"ZZSPKAC1 state at ET #.");
                spicelib::REPMC(&TITLE.clone(), b"#", &TIMSTR, &mut TITLE);
                testutil::CHCKAD(
                    &TITLE,
                    STATE1.as_slice(),
                    b"~~/",
                    STATE.as_slice(),
                    6,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                //
                // Test SPKAPS and its mirror routines:
                //

                //
                // First look up the state and acceleration of the
                // observer relative to the solar system barycenter.
                //
                spicelib::SPKSSB(OBSRVR, ET, &REF, STOBS.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::SPKSSB(OBSRVR, (ET - 1 as f64), &REF, STATE0.as_slice_mut(), ctx)?;
                spicelib::SPKSSB(OBSRVR, (ET + 1 as f64), &REF, STATE1.as_slice_mut(), ctx)?;

                spicelib::QDERIV(
                    3,
                    STATE0.subarray(4),
                    STATE1.subarray(4),
                    1.0,
                    ACC.as_slice_mut(),
                    ctx,
                )?;

                //
                // Call SPKAPS; make sure its outputs are consistent with
                // those of SPKACS.
                //
                spicelib::SPKAPS(
                    TARGET,
                    ET,
                    &REF,
                    &ABCORR,
                    STOBS.as_slice(),
                    ACC.as_slice(),
                    STATE.as_slice_mut(),
                    &mut LT,
                    &mut DLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::SPKACS(
                    TARGET,
                    ET,
                    &REF,
                    &ABCORR,
                    OBSRVR,
                    STATE0.as_slice_mut(),
                    &mut LT0,
                    &mut DLT0,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                fstr::assign(&mut TITLE, b"SPKAPS light time at ET #.");
                spicelib::REPMC(&TITLE.clone(), b"#", &TIMSTR, &mut TITLE);
                testutil::CHCKSD(&TITLE, LT, b"~/", LT0, TIGHT, OK, ctx)?;

                fstr::assign(&mut TITLE, b"SPKAPS light time rate at ET #.");
                spicelib::REPMC(&TITLE.clone(), b"#", &TIMSTR, &mut TITLE);
                testutil::CHCKSD(&TITLE, DLT, b"~/", DLT0, TIGHT, OK, ctx)?;

                spicelib::ETCAL(ET, &mut TIMSTR, ctx);
                fstr::assign(&mut TITLE, b"SPKAPS state at ET #.");
                spicelib::REPMC(&TITLE.clone(), b"#", &TIMSTR, &mut TITLE);
                testutil::CHCKAD(
                    &TITLE,
                    STATE.as_slice(),
                    b"~~/",
                    STATE0.as_slice(),
                    6,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                //
                // Check against ZZSPKAS0: state, light time, and light
                // time rate should match those from SPKAPS.
                //
                spicelib::ZZSPKAS0(
                    TARGET,
                    ET,
                    &REF,
                    &ABCORR,
                    STOBS.as_slice(),
                    ACC.as_slice(),
                    STATE.as_slice_mut(),
                    &mut LT,
                    &mut DLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                fstr::assign(&mut TITLE, b"ZZSPKAS0 light time at ET #.");
                spicelib::REPMC(&TITLE.clone(), b"#", &TIMSTR, &mut TITLE);
                testutil::CHCKSD(&TITLE, LT0, b"=", LT, 0.0, OK, ctx)?;

                fstr::assign(&mut TITLE, b"ZZSPKAS0 light time rate at ET #.");
                spicelib::REPMC(&TITLE.clone(), b"#", &TIMSTR, &mut TITLE);
                testutil::CHCKSD(&TITLE, DLT0, b"=", DLT, 0.0, OK, ctx)?;

                spicelib::ETCAL(ET, &mut TIMSTR, ctx);
                fstr::assign(&mut TITLE, b"ZZSPKAS0 state at ET #.");
                spicelib::REPMC(&TITLE.clone(), b"#", &TIMSTR, &mut TITLE);
                testutil::CHCKAD(
                    &TITLE,
                    STATE0.as_slice(),
                    b"~~/",
                    STATE.as_slice(),
                    6,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                //
                // Check against ZZSPKAS1: state, light time, and light
                // time rate should match those from SPKAPS.
                //
                spicelib::ZZSPKAS1(
                    TARGET,
                    ET,
                    &REF,
                    &ABCORR,
                    STOBS.as_slice(),
                    ACC.as_slice(),
                    STATE.as_slice_mut(),
                    &mut LT,
                    &mut DLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                fstr::assign(&mut TITLE, b"ZZSPKAS1 light time at ET #.");
                spicelib::REPMC(&TITLE.clone(), b"#", &TIMSTR, &mut TITLE);
                testutil::CHCKSD(&TITLE, LT0, b"=", LT, 0.0, OK, ctx)?;

                fstr::assign(&mut TITLE, b"ZZSPKAS1 light time rate at ET #.");
                spicelib::REPMC(&TITLE.clone(), b"#", &TIMSTR, &mut TITLE);
                testutil::CHCKSD(&TITLE, DLT0, b"=", DLT, 0.0, OK, ctx)?;

                spicelib::ETCAL(ET, &mut TIMSTR, ctx);
                fstr::assign(&mut TITLE, b"ZZSPKAS1 state at ET #.");
                spicelib::REPMC(&TITLE.clone(), b"#", &TIMSTR, &mut TITLE);
                testutil::CHCKAD(
                    &TITLE,
                    STATE0.as_slice(),
                    b"~~/",
                    STATE.as_slice(),
                    6,
                    TIGHT,
                    OK,
                    ctx,
                )?;
            }
            //
            // End of epoch loop.
            //
        }
        //
        // End of aberration correction loop.
        //
    }
    //
    // End of frame loop.
    //

    //
    // The fourth set of tests covers the routine ZZSTELAB and its frame
    // system mirror routines. This routine requires an inertial input
    // frame, so we omit the cases for the last frame in the list.
    //
    for FR in 1..=(NFRAME - 1) {
        for AC in 1..=NCORR {
            //
            //---- Case -------------------------------------------------------------
            //
            fstr::assign(
                &mut TITLE,
                b"ZZSTELAB: Earth-Mars stellar aberration/velocity; frame #, abcorr #.",
            );

            OBSRVR = 399;
            TARGET = 499;
            fstr::assign(&mut REF, save.FRMLST.get(FR));
            fstr::assign(&mut ABCORR, save.CORLST.get(AC));
            spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
            spicelib::REPMC(&TITLE.clone(), b"#", &ABCORR, &mut TITLE);

            testutil::TCASE(&TITLE, ctx)?;

            //
            // Parse the aberration correction flag; determine whether we
            // have a specification for reception or transmission
            // corrections.
            //
            spicelib::ZZPRSCOR(&ABCORR, ATTBLK.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            XMIT = ATTBLK[XMTIDX];

            //
            // Set up our ephemeris utilities to enable Cheby fitting.
            //
            RETVAL = testutil::T_STCINI(TARGET, &REF, &ABCORR, OBSRVR, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Take four years' worth of samples, spaced 2 months apart.
            //
            TDELTA = (spicelib::JYEAR() / 6 as f64);
            NSAMP = 24;
            ET0 = 0.0;
            HALFWN = 300.0;
            DEGREE = 12;
            MAXN = (DEGREE + 1);
            DELTA = 1.0;

            for I in 1..=NSAMP {
                ET = (ET0 + (((I - 1) as f64) * TDELTA));
                //
                // Get the observer-target state without stellar
                // aberration correction. Also get the state of
                // the observer relative to the solar system barycenter.
                //
                spicelib::SPKSSB(OBSRVR, ET, &REF, STOBS.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::SPKLTC(
                    TARGET,
                    ET,
                    &REF,
                    &ABCORR,
                    STOBS.as_slice(),
                    STATE.as_slice_mut(),
                    &mut LT,
                    &mut DLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                //
                // Compute the acceleration of the observer with respect
                // to the solar system barycenter.
                //
                for J in 1..=2 {
                    T = (ET + ((((2 * J) - 3) as f64) * DELTA));

                    spicelib::SPKGEO(
                        OBSRVR,
                        T,
                        &REF,
                        SSB,
                        SSBOBS.subarray_mut([1, J]),
                        &mut LTSSB,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                spicelib::QDERIV(
                    3,
                    SSBOBS.subarray([4, 1]),
                    SSBOBS.subarray([4, 2]),
                    DELTA,
                    ACC.as_slice_mut(),
                    ctx,
                )?;

                //
                // Get the stellar aberration correction and its time
                // derivative. Note that the input observer state relative
                // to the solar system barycenter was obtained from SPKLTC.
                //
                spicelib::ZZSTELAB(
                    XMIT,
                    ACC.as_slice(),
                    STOBS.subarray(4),
                    STATE.as_slice(),
                    PCORR.as_slice_mut(),
                    DPCORR.as_slice_mut(),
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Check the stellar aberration offset and its time
                // derivative against those from an alternate
                // implementation. We expect a very close match.
                //
                testutil::T_ZZSTELAB(
                    XMIT,
                    ACC.as_slice(),
                    STOBS.subarray(4),
                    STATE.as_slice(),
                    PCORR2.as_slice_mut(),
                    DPCOR2.as_slice_mut(),
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"PCORR -0",
                    PCORR.as_slice(),
                    b"~~/",
                    PCORR2.as_slice(),
                    3,
                    VTIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKAD(
                    b"DPCORR -0",
                    DPCORR.as_slice(),
                    b"~~/",
                    DPCOR2.as_slice(),
                    3,
                    VTIGHT,
                    OK,
                    ctx,
                )?;

                //
                // Check the stellar aberration offset and its time
                // derivative against those from a numeric
                // implementation. We expect a very good match
                // for the position correction and a near-single
                // precision match for the velocity correction.
                //
                testutil::T_ZZSTLABN(
                    XMIT,
                    ACC.as_slice(),
                    STOBS.subarray(4),
                    STATE.as_slice(),
                    PCORR2.as_slice_mut(),
                    DPCOR2.as_slice_mut(),
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"PCORR -1",
                    PCORR.as_slice(),
                    b"~~/",
                    PCORR2.as_slice(),
                    3,
                    MEDIUM,
                    OK,
                    ctx,
                )?;

                testutil::CHCKAD(
                    b"DPCORR -1",
                    DPCORR.as_slice(),
                    b"~~/",
                    DPCOR2.as_slice(),
                    3,
                    LOW,
                    OK,
                    ctx,
                )?;

                //
                // Check the stellar aberration position offset against
                // that obtained from either of the legacy routines STELAB
                // and STLABX. We expect close agreement.
                //
                if XMIT {
                    spicelib::STLABX(
                        STATE.as_slice(),
                        STOBS.subarray(4),
                        CORPOS.as_slice_mut(),
                        ctx,
                    )?;
                    spicelib::VSUB(CORPOS.as_slice(), STATE.as_slice(), PCORR2.as_slice_mut());
                } else {
                    spicelib::STELAB(
                        STATE.as_slice(),
                        STOBS.subarray(4),
                        CORPOS.as_slice_mut(),
                        ctx,
                    )?;
                    spicelib::VSUB(CORPOS.as_slice(), STATE.as_slice(), PCORR2.as_slice_mut());
                }

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"PCORR -2",
                    PCORR.as_slice(),
                    b"~~/",
                    PCORR2.as_slice(),
                    3,
                    MEDIUM,
                    OK,
                    ctx,
                )?;

                //
                // Fit a Cheby expansion of degree MAXDEG to the each
                // component of the stellar aberration correction over a
                // 2*HALFWN time span centered at ET. We'll retain an
                // expansion of degree DEGREE.
                //
                LEFT = (ET - HALFWN);
                RIGHT = (ET + HALFWN);

                for J in 1..=3 {
                    if (J == 1) {
                        support::CHBFIT(
                            testutil::T_GETSX,
                            LEFT,
                            RIGHT,
                            (MAXDEG + 1),
                            WORK.as_slice_mut(),
                            COEFFS.as_slice_mut(),
                            ctx,
                        )?;
                    } else if (J == 2) {
                        support::CHBFIT(
                            testutil::T_GETSY,
                            LEFT,
                            RIGHT,
                            (MAXDEG + 1),
                            WORK.as_slice_mut(),
                            COEFFS.as_slice_mut(),
                            ctx,
                        )?;
                    } else {
                        support::CHBFIT(
                            testutil::T_GETSZ,
                            LEFT,
                            RIGHT,
                            (MAXDEG + 1),
                            WORK.as_slice_mut(),
                            COEFFS.as_slice_mut(),
                            ctx,
                        )?;
                    }

                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Find the derivatives of the current Cheby expansion.
                    //
                    X2S[1] = ET;
                    X2S[2] = HALFWN;

                    spicelib::CHBDER(
                        COEFFS.as_slice(),
                        (MAXN - 1),
                        X2S.as_slice(),
                        ET,
                        1,
                        PARTDP.as_slice_mut(),
                        DERIVS.as_slice_mut(),
                    );

                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    POS[1] = DERIVS[0];
                    DPOS[1] = DERIVS[1];

                    //
                    // Note: to compute acceleration, change the argument
                    // "1" above to "2" and capture acceleration from
                    //
                    //    DERIVS(2)
                    //
                    //
                    // Compare the expansion to the position correction
                    // from ZZSTELAB at ET.
                    //
                    spicelib::ETCAL(ET, &mut TIMSTR, ctx);
                    fstr::assign(&mut TITLE, b"Position component # at ET #.");

                    spicelib::REPMI(&TITLE.clone(), b"#", J, &mut TITLE, ctx);
                    spicelib::REPMC(&TITLE.clone(), b"#", &TIMSTR, &mut TITLE);

                    testutil::CHCKSD(&TITLE, PCORR[J], b"~/", POS[1], MEDIUM, OK, ctx)?;

                    //
                    // Compare the derivative of the expansion to the
                    // velocity from ZZSTELAB at ET.
                    //
                    spicelib::ETCAL(ET, &mut TIMSTR, ctx);
                    fstr::assign(&mut TITLE, b"Velocity component # at ET #.");

                    spicelib::REPMI(&TITLE.clone(), b"#", J, &mut TITLE, ctx);
                    spicelib::REPMC(&TITLE.clone(), b"#", &TIMSTR, &mut TITLE);

                    //
                    // The relative error should be less than
                    // 1.D-4.
                    //
                    testutil::CHCKSD(&TITLE, DPCORR[J], b"~/", DPOS[1], LOW, OK, ctx)?;

                    // IF ( .NOT. OK ) THEN
                    //    WRITE (*,*) 'PCORR  = ', PCORR
                    //    WRITE (*,*) 'DPCORR = ', DPCORR
                    // END IF

                    //
                    // Make sure the absolute velocity error is less
                    // than 10 microns/sec.
                    //
                    testutil::CHCKSD(&TITLE, DPCORR[J], b"~", DPOS[1], 0.00000001, OK, ctx)?;
                }
                //
                // End of correction component loop.
                //
            }
            //
            // End of epoch loop.
            //
        }
        //
        // End of aberration correction loop.
        //
    }
    //
    // End of frame loop.
    //

    //
    //     Error cases:
    //
    //
    //---- Case -------------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"SPKLTC, ZZSPKLT0, ZZSPKLT1: reject non-inertial frames",
    );

    OBSRVR = 399;
    TARGET = 499;
    fstr::assign(&mut REF, b"IAU_MOON");
    fstr::assign(&mut ABCORR, b"LT");

    spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
    spicelib::REPMC(&TITLE.clone(), b"#", &ABCORR, &mut TITLE);

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::SPKSSB(OBSRVR, ET, &REF, STOBS.as_slice_mut(), ctx)?;
    spicelib::SPKLTC(
        TARGET,
        ET,
        &REF,
        &ABCORR,
        STOBS.as_slice(),
        STATE.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADFRAME)", OK, ctx)?;

    spicelib::ZZSPKLT0(
        TARGET,
        ET,
        &REF,
        &ABCORR,
        STOBS.as_slice(),
        STATE.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADFRAME)", OK, ctx)?;

    spicelib::ZZSPKLT1(
        TARGET,
        ET,
        &REF,
        &ABCORR,
        STOBS.as_slice(),
        STATE.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADFRAME)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"SPKACS, ZZSPKAC0, ZZSPKAC1: reject non-inertial frames",
    );

    OBSRVR = 399;
    TARGET = 499;
    fstr::assign(&mut REF, b"IAU_MOON");
    fstr::assign(&mut ABCORR, b"LT");

    spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
    spicelib::REPMC(&TITLE.clone(), b"#", &ABCORR, &mut TITLE);

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::SPKACS(
        TARGET,
        ET,
        &REF,
        &ABCORR,
        OBSRVR,
        STATE.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADFRAME)", OK, ctx)?;

    spicelib::ZZSPKAC0(
        TARGET,
        ET,
        &REF,
        &ABCORR,
        OBSRVR,
        STATE.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADFRAME)", OK, ctx)?;

    spicelib::ZZSPKAC1(
        TARGET,
        ET,
        &REF,
        &ABCORR,
        OBSRVR,
        STATE.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADFRAME)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"SPKAPS, ZZSPKAS0, ZZSPKAS1: reject non-inertial frames",
    );

    OBSRVR = 399;
    TARGET = 499;
    fstr::assign(&mut REF, b"IAU_MOON");
    fstr::assign(&mut ABCORR, b"LT");

    spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
    spicelib::REPMC(&TITLE.clone(), b"#", &ABCORR, &mut TITLE);

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::SPKAPS(
        TARGET,
        ET,
        &REF,
        &ABCORR,
        STOBS.as_slice(),
        STATE.as_slice(),
        ACC.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADFRAME)", OK, ctx)?;

    spicelib::ZZSPKAS0(
        TARGET,
        ET,
        &REF,
        &ABCORR,
        STOBS.as_slice(),
        STATE.as_slice(),
        ACC.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADFRAME)", OK, ctx)?;

    spicelib::ZZSPKAS1(
        TARGET,
        ET,
        &REF,
        &ABCORR,
        STOBS.as_slice(),
        STATE.as_slice(),
        ACC.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADFRAME)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"SPKAPS: reject S, XS, relativistic ABCORR values",
    );

    OBSRVR = 399;
    TARGET = 499;
    fstr::assign(&mut REF, b"IAU_MOON");
    fstr::assign(&mut ABCORR, b"LT");

    spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
    spicelib::REPMC(&TITLE.clone(), b"#", &ABCORR, &mut TITLE);

    testutil::TCASE(&TITLE, ctx)?;

    for I in 1..=NBAD {
        spicelib::SPKAPS(
            TARGET,
            ET,
            &REF,
            &save.BADCOR[I],
            STOBS.as_slice(),
            STATE.as_slice(),
            ACC.as_slice_mut(),
            &mut LT,
            &mut DLT,
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"SPKAPS: reject random, invalid ABCORR value");
    OBSRVR = 399;
    TARGET = 499;
    fstr::assign(&mut REF, b"IAU_MOON");

    spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
    spicelib::REPMC(&TITLE.clone(), b"#", &ABCORR, &mut TITLE);

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::SPKAPS(
        TARGET,
        ET,
        &REF,
        b"XYZ",
        STOBS.as_slice(),
        STATE.as_slice(),
        ACC.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"SPKACS: reject S, XS, relativistic ABCORR values",
    );

    OBSRVR = 399;
    TARGET = 499;
    fstr::assign(&mut REF, b"J2000");

    spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
    spicelib::REPMC(&TITLE.clone(), b"#", &ABCORR, &mut TITLE);

    testutil::TCASE(&TITLE, ctx)?;

    for I in 1..=NBAD {
        spicelib::SPKACS(
            TARGET,
            ET,
            &REF,
            &save.BADCOR[I],
            OBSRVR,
            STATE.as_slice_mut(),
            &mut LT,
            &mut DLT,
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"SPKACS: reject random, invalid ABCORR value");
    OBSRVR = 399;
    TARGET = 499;
    fstr::assign(&mut REF, b"IAU_MOON");

    spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
    spicelib::REPMC(&TITLE.clone(), b"#", &ABCORR, &mut TITLE);

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::SPKACS(
        TARGET,
        ET,
        &REF,
        b"XYZ",
        OBSRVR,
        STATE.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"SPKLTC: reject S, XS, relativistic ABCORR values",
    );

    TARGET = 499;
    fstr::assign(&mut REF, b"J2000");

    spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
    spicelib::REPMC(&TITLE.clone(), b"#", &ABCORR, &mut TITLE);

    testutil::TCASE(&TITLE, ctx)?;

    for I in 1..=NBAD {
        spicelib::SPKLTC(
            TARGET,
            ET,
            &REF,
            &save.BADCOR[I],
            STOBS.as_slice(),
            STATE.as_slice_mut(),
            &mut LT,
            &mut DLT,
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"SPKLTC: reject random, invalid ABCORR value");
    OBSRVR = 399;
    TARGET = 499;
    fstr::assign(&mut REF, b"IAU_MOON");

    spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
    spicelib::REPMC(&TITLE.clone(), b"#", &ABCORR, &mut TITLE);

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::SPKLTC(
        TARGET,
        ET,
        &REF,
        b"XYZ",
        STOBS.as_slice(),
        STATE.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"SPKEZ: reject S, XS, relativistic ABCORR values",
    );

    OBSRVR = 399;
    TARGET = 499;
    fstr::assign(&mut REF, b"IAU_MOON");

    spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
    spicelib::REPMC(&TITLE.clone(), b"#", &ABCORR, &mut TITLE);

    testutil::TCASE(&TITLE, ctx)?;

    for I in 1..=NBAD {
        spicelib::SPKEZ(
            TARGET,
            ET,
            &REF,
            &save.BADCOR[I],
            OBSRVR,
            STATE.as_slice_mut(),
            &mut LT,
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"SPKEZ: reject random, invalid ABCORR value");
    OBSRVR = 399;
    TARGET = 499;
    fstr::assign(&mut REF, b"IAU_MOON");

    spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
    spicelib::REPMC(&TITLE.clone(), b"#", &ABCORR, &mut TITLE);

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::SPKEZ(
        TARGET,
        ET,
        &REF,
        b"XYZ",
        OBSRVR,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"SPKEZ: bad frame name");
    OBSRVR = 399;
    TARGET = 499;
    fstr::assign(&mut REF, b"IAU_MOOON");

    spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
    spicelib::REPMC(&TITLE.clone(), b"#", &ABCORR, &mut TITLE);

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::SPKEZ(
        TARGET,
        ET,
        &REF,
        b"LT+S",
        OBSRVR,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(UNKNOWNFRAME)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"SPKEZ: REFCHG failure");
    OBSRVR = 399;
    TARGET = 499;
    fstr::assign(&mut REF, b"ITRF93");

    spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
    spicelib::REPMC(&TITLE.clone(), b"#", &ABCORR, &mut TITLE);

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::SPKEZ(
        TARGET,
        ET,
        &REF,
        b"LT+S",
        OBSRVR,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(FRAMEDATANOTFOUND)", OK, ctx)?;

    //
    // Clean up the kernels.
    //
    spicelib::SPKUEF(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
