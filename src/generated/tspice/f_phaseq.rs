//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CNVTOL: f64 = 0.000001;
const NWMAX: i32 = 15;
const NWDIST: i32 = 5;
const NWSEP: i32 = 5;
const NWRR: i32 = 5;
const NWUDS: i32 = 5;
const NWPA: i32 = 5;
const NWILUM: i32 = 5;
const ADDWIN: f64 = 0.5;
const FRMNLN: i32 = 32;
const FOVTLN: i32 = 40;
const FTCIRC: &[u8] = b"CIRCLE";
const FTELLI: &[u8] = b"ELLIPSE";
const FTPOLY: &[u8] = b"POLYGON";
const FTRECT: &[u8] = b"RECTANGLE";
const ANNULR: &[u8] = b"ANNULAR";
const ANY: &[u8] = b"ANY";
const PARTL: &[u8] = b"PARTIAL";
const FULL: &[u8] = b"FULL";
const DSSHAP: &[u8] = b"DSK";
const EDSHAP: &[u8] = b"ELLIPSOID";
const PTSHAP: &[u8] = b"POINT";
const RYSHAP: &[u8] = b"RAY";
const SPSHAP: &[u8] = b"SPHERE";
const NOCTYP: i32 = 4;
const OCLLN: i32 = 7;
const SHPLEN: i32 = 9;
const MAXVRT: i32 = 10000;
const CIRFOV: &[u8] = b"CIRCLE";
const ELLFOV: &[u8] = b"ELLIPSE";
const POLFOV: &[u8] = b"POLYGON";
const RECFOV: &[u8] = b"RECTANGLE";
const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;
const PCK: &[u8] = b"nat.pck";
const SPK: &[u8] = b"nat.spk";
const LSK: &[u8] = b"phaseq.tls";
const ANGTOL: f64 = 0.000000001;
const NRCORR: i32 = 5;
const NXCORR: i32 = 4;
const ABCLEN: i32 = 25;
const STRLEN: i32 = 124;
const LBCELL: i32 = -5;
const MAXWIN: i32 = 100;

struct SaveVars {
    RCORR: ActualCharArray,
    XCORR: ActualCharArray,
    TARGET: ActualCharArray,
    ILLUMN: ActualCharArray,
    OBSRVR: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut RCORR = ActualCharArray::new((2 * CORLEN), 1..=NRCORR);
        let mut XCORR = ActualCharArray::new((2 * CORLEN), 1..=NXCORR);
        let mut TARGET = ActualCharArray::new(ABCLEN, 1..=6);
        let mut ILLUMN = ActualCharArray::new(ABCLEN, 1..=6);
        let mut OBSRVR = ActualCharArray::new(ABCLEN, 1..=6);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"  nOne "),
                Val::C(b" lT"),
                Val::C(b"  Cn"),
                Val::C(b" Lt + s"),
                Val::C(b"cN + S"),
            ]
            .into_iter();
            RCORR
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b" xlT"),
                Val::C(b"  xCn"),
                Val::C(b" XLt + s"),
                Val::C(b"XcN + S"),
            ]
            .into_iter();
            XCORR
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"ALPHA"),
                Val::C(b"ALPHA"),
                Val::C(b"X"),
                Val::C(b"ALPHA"),
                Val::C(b"SUN"),
                Val::C(b"ALPHA"),
            ]
            .into_iter();
            TARGET
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"SUN"),
                Val::C(b"X"),
                Val::C(b"SUN"),
                Val::C(b"ALPHA"),
                Val::C(b"SUN"),
                Val::C(b"SUN"),
            ]
            .into_iter();
            ILLUMN
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"X"),
                Val::C(b"BETA"),
                Val::C(b"BETA"),
                Val::C(b"BETA"),
                Val::C(b"ALPHA"),
                Val::C(b"SUN"),
            ]
            .into_iter();
            OBSRVR
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            RCORR,
            XCORR,
            TARGET,
            ILLUMN,
            OBSRVR,
        }
    }
}

//$Procedure F_PHASEQ ( PHASEQ tests )
pub fn F_PHASEQ(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ABCORR = [b' '; ABCLEN as usize];
    let mut TARG = [b' '; ABCLEN as usize];
    let mut ILLUM = [b' '; ABCLEN as usize];
    let mut OBS = [b' '; ABCLEN as usize];
    let mut TXT = [b' '; STRLEN as usize];
    let mut V_PHAS = StackArray::<f64, 5>::new(1..=NRCORR);
    let mut PHAS: f64 = 0.0;
    let mut XPHAS: f64 = 0.0;
    let mut ET: f64 = 0.0;
    let mut POSA = StackArray::<f64, 3>::new(1..=3);
    let mut POSB = StackArray::<f64, 3>::new(1..=3);
    let mut ADJUST: f64 = 0.0;
    let mut CNFINE = StackArray::<f64, 106>::new(LBCELL..=MAXWIN);
    let mut ET0: f64 = 0.0;
    let mut ET1: f64 = 0.0;
    let mut TBEG: f64 = 0.0;
    let mut TEND: f64 = 0.0;
    let mut LT: f64 = 0.0;
    let mut REFVAL: f64 = 0.0;
    let mut RESULT = StackArray::<f64, 106>::new(LBCELL..=MAXWIN);
    let mut STEP: f64 = 0.0;
    let mut WORK = ActualArray2D::<f64>::new(LBCELL..=MAXWIN, 1..=NWSEP);
    let mut RLOG: f64 = 0.0;
    let mut T1: f64 = 0.0;
    let mut TOL: f64 = 0.0;
    let mut COUNT: i32 = 0;
    let mut HAN: i32 = 0;
    let mut SEED1: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Saved variables
    //

    //
    // Indices 1:3 for Invalid body name test, 4:6 for not distinct
    // body names test.
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_PHASEQ", ctx)?;

    spicelib::KCLEAR(ctx)?;
    SEED1 = -54290018;

    //
    // Case 1: Create test kernels.
    //
    testutil::TCASE(b"Setup: create and load SPK, PCK, LSK files.", ctx)?;

    //
    // Create the PCK for Nat's Solar System.
    //
    testutil::NATPCK(PCK, false, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(PCK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create the SPK for Nat's Solar System.
    //
    testutil::NATSPK(SPK, true, &mut HAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(SPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create an LSK, load using FURNSH.
    //
    testutil::ZZTSTLSK(LSK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(LSK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Case 2: Invalid body names.
    //

    //
    // Set appropriate values for ET, ABCORR.
    //
    ET = 0.0;
    fstr::assign(&mut ABCORR, save.RCORR.get(1));

    for I in 1..=3 {
        fstr::assign(&mut TARG, save.TARGET.get(I));
        fstr::assign(&mut ILLUM, save.ILLUMN.get(I));
        fstr::assign(&mut OBS, save.OBSRVR.get(I));

        fstr::assign(
            &mut TXT,
            b"Invalid body name test. TARG = #, ILLUM = #, OBS = #",
        );
        spicelib::REPMC(&TXT.clone(), b"#", &TARG, &mut TXT);
        spicelib::REPMC(&TXT.clone(), b"#", &ILLUM, &mut TXT);
        spicelib::REPMC(&TXT.clone(), b"#", &OBS, &mut TXT);

        testutil::TCASE(&TXT, ctx)?;

        PHAS = spicelib::PHASEQ(ET, &TARG, &ILLUM, &OBS, &ABCORR, ctx)?;
        testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;
    }

    //
    // Case 3: Invalid aberration corrections.
    //

    //
    // Set appropriate values for TARGET, ILLUMN, OBSRVR.
    //
    fstr::assign(&mut TARG, save.TARGET.get(1));
    fstr::assign(&mut ILLUM, save.ILLUMN.get(1));
    fstr::assign(&mut OBS, save.OBSRVR.get(2));

    for I in 1..=NXCORR {
        fstr::assign(&mut ABCORR, save.XCORR.get(I));

        fstr::assign(&mut TXT, b"Invalid aberration correction. ABCOR = #");
        spicelib::REPMC(&TXT.clone(), b"#", &ABCORR, &mut TXT);

        testutil::TCASE(&TXT, ctx)?;

        PHAS = spicelib::PHASEQ(ET, &TARG, &ILLUM, &OBS, &ABCORR, ctx)?;
        testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;
    }

    //
    // Case 4: Not distinct body names.
    //

    //
    // Set appropriate values for ABCORR.
    //
    fstr::assign(&mut ABCORR, save.RCORR.get(1));

    for I in 4..=6 {
        fstr::assign(&mut TARG, save.TARGET.get(I));
        fstr::assign(&mut ILLUM, save.ILLUMN.get(I));
        fstr::assign(&mut OBS, save.OBSRVR.get(I));

        fstr::assign(
            &mut TXT,
            b"Not distinct body name test. TARG = #, ILLUM = #, OBS = #",
        );
        spicelib::REPMC(&TXT.clone(), b"#", &TARG, &mut TXT);
        spicelib::REPMC(&TXT.clone(), b"#", &ILLUM, &mut TXT);
        spicelib::REPMC(&TXT.clone(), b"#", &OBS, &mut TXT);

        testutil::TCASE(&TXT, ctx)?;

        PHAS = spicelib::PHASEQ(ET, &TARG, &ILLUM, &OBS, &ABCORR, ctx)?;
        testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;
    }

    //
    // Case 5:
    //
    testutil::TCASE(b"Separation angle vs phase angle - 1", ctx)?;

    //
    // Calculate the phase angle for geometry with BETA as
    // the observer with ALPHA as the target. We perform
    // an angular separtion search for a geometry with an
    // identical result. This test requires use of NONE
    // correction.
    //

    //
    // Perform a simple search using ALPHA and BETA for times
    // of maximum angular separation as seen from the sun.
    // Recall ALPHA - BETA occultation occurs every day
    // at 12:00 PM TDB with correction NONE.
    //

    STEP = 60.0;
    ADJUST = 0.0;
    REFVAL = 0.0;

    spicelib::SSIZED(MAXWIN, CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Store the time bounds of our search interval in
    // the CNFINE confinement window.
    //
    spicelib::STR2ET(b"1999 DEC 31 21:00:00 TDB", &mut ET0, ctx)?;
    spicelib::STR2ET(b"2000 JAN 02 03:00:00 TDB", &mut ET1, ctx)?;

    spicelib::SCARDD(0, CNFINE.as_slice_mut(), ctx)?;
    spicelib::WNINSD(ET0, ET1, CNFINE.as_slice_mut(), ctx)?;

    spicelib::GFSEP(
        b"SUN",
        b"POINT",
        b"IAU_SUN",
        b"BETA",
        b"POINT",
        b"BETAFIXED",
        &ABCORR,
        b"ALPHA",
        b"ABSMAX",
        REFVAL,
        ADJUST,
        STEP,
        CNFINE.as_slice(),
        MAXWIN,
        NWSEP,
        WORK.as_slice_mut(),
        RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the number of intervals in the result window.
    //
    COUNT = 0;
    COUNT = spicelib::WNCARD(RESULT.as_slice(), ctx)?;
    testutil::CHCKSI(b"COUNT", COUNT, b"=", 1, 0, OK, ctx)?;

    spicelib::WNFETD(RESULT.as_slice(), 1, &mut TBEG, &mut TEND, ctx)?;

    spicelib::SPKPOS(
        b"SUN",
        TBEG,
        b"J2000",
        &ABCORR,
        b"ALPHA",
        POSA.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    spicelib::SPKPOS(
        b"BETA",
        TBEG,
        b"J2000",
        &ABCORR,
        b"ALPHA",
        POSB.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    fstr::assign(&mut TARG, b"ALPHA");
    fstr::assign(&mut ILLUM, b"SUN");
    fstr::assign(&mut OBS, b"BETA");

    PHAS = spicelib::PHASEQ(TBEG, &TARG, &ILLUM, &OBS, &ABCORR, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The angular separation should be within the ANGTOL
    // tolerance of the phase angle.
    //
    testutil::CHCKSD(
        b"SEP vs PHASE",
        spicelib::VSEP(POSA.as_slice(), POSB.as_slice(), ctx),
        b"~",
        PHAS,
        ANGTOL,
        OK,
        ctx,
    )?;

    //
    // Case 6:
    //
    testutil::TCASE(b"Separation angle vs phase angle - 2", ctx)?;

    //
    // Use ALPHA as the observer with BETA as the target.
    // The maximum value for this geometry phase angle is PI.
    // As before, perform an angular separtion search for a
    // geometry with an identical result. This test requires
    // use of NONE correction.
    //

    spicelib::SCARDD(0, CNFINE.as_slice_mut(), ctx)?;
    spicelib::WNINSD(ET0, ET1, CNFINE.as_slice_mut(), ctx)?;

    spicelib::GFSEP(
        b"SUN",
        b"POINT",
        b"IAU_SUN",
        b"ALPHA",
        b"POINT",
        b"ALPHAFIXED",
        &ABCORR,
        b"BETA",
        b"ABSMAX",
        REFVAL,
        ADJUST,
        STEP,
        CNFINE.as_slice(),
        MAXWIN,
        NWSEP,
        WORK.as_slice_mut(),
        RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the number of intervals in the result window.
    //
    COUNT = 0;
    COUNT = spicelib::WNCARD(RESULT.as_slice(), ctx)?;
    testutil::CHCKSI(b"COUNT", COUNT, b"=", 1, 0, OK, ctx)?;

    spicelib::WNFETD(RESULT.as_slice(), 1, &mut TBEG, &mut TEND, ctx)?;

    spicelib::SPKPOS(
        b"BETA",
        TBEG,
        b"J2000",
        &ABCORR,
        b"SUN",
        POSA.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    spicelib::SPKPOS(
        b"BETA",
        TBEG,
        b"J2000",
        &ABCORR,
        b"ALPHA",
        POSB.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    fstr::assign(&mut TARG, b"BETA");
    fstr::assign(&mut ILLUM, b"SUN");
    fstr::assign(&mut OBS, b"ALPHA");

    PHAS = spicelib::PHASEQ(TBEG, &TARG, &ILLUM, &OBS, &ABCORR, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The angular separation should be within the ANGTOL
    // tolerance of the phase angle, in this case, PI.
    //
    testutil::CHCKSD(
        b"SEP vs PHASE",
        spicelib::VSEP(POSA.as_slice(), POSB.as_slice(), ctx),
        b"~",
        PHAS,
        ANGTOL,
        OK,
        ctx,
    )?;

    //
    // Case 7:
    //
    // Specifically confirm the PHASEQ calculation returns numerically
    // different results for different valid aberration corrections with
    // all other arguments held constant.
    //
    for I in 1..=NRCORR {
        V_PHAS[I] = spicelib::PHASEQ(TBEG, &TARG, &ILLUM, &OBS, &save.RCORR[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=(NRCORR - 1) {
        for J in (I - 1)..=NRCORR {
            if (I < J) {
                fstr::assign(&mut TXT, b"Diff phase on diff abcorr, I = #, J = #");
                spicelib::REPMI(&TXT.clone(), b"#", I, &mut TXT, ctx);
                spicelib::REPMI(&TXT.clone(), b"#", J, &mut TXT, ctx);

                testutil::TCASE(&TXT, ctx)?;

                testutil::CHCKSD(&TXT, V_PHAS[I], b"!=", V_PHAS[J], 0.0, OK, ctx)?;
            }
        }
    }

    //
    // Case 8:
    //
    // Confirm the expected property PHASE(t) = PHASE(t + tau*n)
    // for the orbit of BETA relative to ALPHA for all allowed
    // aberration corrections.
    //
    // Recall ALPHA and BETA are in circular orbits.
    //
    // In this case with BETA orbit period of 21 hours, and
    // ALPHA orbit period of 7 days, we can derive the period of
    // BETA with respect to ALPHA.
    //
    //    omega = 2pi/21hours.tosecs - 2pi/7days.tosecs
    //    tau   = 2pi/omega
    //
    // This gives tau = 86400secs.
    //
    for I in 1..=NRCORR {
        //
        // The time coverage for nat.spk is 1899 DEC 31 12:00:00.000 TDB
        // to 2100 JAN 01 12:00:00.000 TDB. This corresponds to
        // approximately -3.1*10^9 TDB seconds from J2000 to 3.1*10^9
        // TDB seconds from J2000. Randomly select a positive time within
        // this interval as the test time.
        //
        // 10^9.477121254719663 ~ 3*10^9
        //
        RLOG = testutil::T_RANDD(0.0, 9.0, &mut SEED1, ctx)?;
        T1 = f64::powf(10.0, RLOG);

        //
        // At T1 near 10^8, we loose of accuracy due to the use of
        // double precision evaluations, so reduce the tolerance by
        // an order of magnitude. T1 will never exceed 10^9 in this test.
        //
        if (T1 > f64::powi(10.0, 8)) {
            TOL = (ANGTOL * 10.0);
        } else {
            TOL = ANGTOL;
        }

        XPHAS = spicelib::PHASEQ(T1, &TARG, &ILLUM, &OBS, &save.RCORR[I], ctx)?;

        for J in 1..=5 {
            fstr::assign(&mut TXT, b"PHASE(t) = PHASE(t + # * tau), CORR = #");
            spicelib::REPMI(&TXT.clone(), b"#", (J * 10), &mut TXT, ctx);
            spicelib::REPMC(&TXT.clone(), b"#", &save.RCORR[I], &mut TXT);

            testutil::TCASE(&TXT, ctx)?;

            PHAS = spicelib::PHASEQ(
                (T1 + (((J as f64) * spicelib::SPD()) * 10 as f64)),
                &TARG,
                &ILLUM,
                &OBS,
                &save.RCORR[I],
                ctx,
            )?;
            testutil::CHCKSD(&TXT, PHAS, b"~", XPHAS, TOL, OK, ctx)?;
        }
    }

    //
    // Case N:
    //
    testutil::TCASE(b"Clean up:  delete kernels.", ctx)?;

    spicelib::KCLEAR(ctx)?;

    spicelib::DELFIL(PCK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(LSK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
