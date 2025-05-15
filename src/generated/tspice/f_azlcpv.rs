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
const GENPCK: &[u8] = b"test.tpc";
const GENSPK: &[u8] = b"test.bsp";
const FIXSPK: &[u8] = b"azlcpv_fix_test.bsp";
const TIGHT: f64 = 0.000000000001;
const MED: f64 = 0.0000000001;
const ABCLEN: i32 = 25;
const BDNMLN: i32 = 36;
const FRNMLN: i32 = 32;
const LNSIZE: i32 = 160;
const MTHLEN: i32 = 9;
const NCORR: i32 = 9;
const NFLAGS: i32 = 4;
const NLINES: i32 = 12;
const NTIMES: i32 = 5;
const SIDLEN: i32 = 40;

struct SaveVars {
    ABCORR: ActualCharArray,
    AZCCW: StackArray<bool, 4>,
    ELPLSZ: StackArray<bool, 4>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ABCORR = ActualCharArray::new(ABCLEN, 1..=NCORR);
        let mut AZCCW = StackArray::<bool, 4>::new(1..=NFLAGS);
        let mut ELPLSZ = StackArray::<bool, 4>::new(1..=NFLAGS);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b" nOne"),
                Val::C(b" lT"),
                Val::C(b" xlT"),
                Val::C(b"  Cn"),
                Val::C(b"  xCn"),
                Val::C(b" Lt + s"),
                Val::C(b" XLt + s"),
                Val::C(b"cN + S"),
                Val::C(b"XcN + S"),
            ]
            .into_iter();
            ABCORR
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::L(true),
                Val::L(true),
                Val::L(true),
                Val::L(false),
                Val::L(false),
                Val::L(true),
                Val::L(false),
                Val::L(false),
            ]
            .into_iter();
            for I in intrinsics::range(1, 4, 1) {
                AZCCW[I] = clist.next().unwrap().into_bool();
                ELPLSZ[I] = clist.next().unwrap().into_bool();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            ABCORR,
            AZCCW,
            ELPLSZ,
        }
    }
}

//$Procedure F_AZLCPV ( Test AZ/EL constant position/velocity routines )
pub fn F_AZLCPV(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut CORR = [b' '; ABCLEN as usize];
    let mut FRAME = [b' '; FRNMLN as usize];
    let mut METHOD = [b' '; MTHLEN as usize];
    let mut OBSCTR = [b' '; BDNMLN as usize];
    let mut OBSREF = [b' '; FRNMLN as usize];
    let mut SEGID = [b' '; SIDLEN as usize];
    let mut TARGET = [b' '; BDNMLN as usize];
    let mut TEXT = ActualCharArray::new(LNSIZE, 1..=NLINES);
    let mut TITLE = [b' '; LNSIZE as usize];
    let mut AZLSTA = StackArray::<f64, 6>::new(1..=6);
    let mut BADPOS = StackArray::<f64, 3>::new(1..=3);
    let mut DVALS = StackArray::<f64, 4>::new(1..=4);
    let mut EPOCH1: f64 = 0.0;
    let mut ERADI = StackArray::<f64, 3>::new(1..=3);
    let mut ET: f64 = 0.0;
    let mut FIRST: f64 = 0.0;
    let mut JACOBI = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut LAST: f64 = 0.0;
    let mut LAT: f64 = 0.0;
    let mut LON: f64 = 0.0;
    let mut LT: f64 = 0.0;
    let mut OBSPOS = StackArray::<f64, 3>::new(1..=3);
    let mut RADII = StackArray::<f64, 3>::new(1..=3);
    let mut STATE = StackArray::<f64, 6>::new(1..=6);
    let mut STATE0 = StackArray::<f64, 6>::new(1..=6);
    let mut STATES = StackArray2D::<f64, 12>::new(1..=6, 1..=2);
    let mut STEP: f64 = 0.0;
    let mut TMPVEC = StackArray::<f64, 3>::new(1..=3);
    let mut XLT: f64 = 0.0;
    let mut XSTATE = StackArray::<f64, 6>::new(1..=6);
    let mut CTRCDE: i32 = 0;
    let mut DEGREE: i32 = 0;
    let mut FIXHAN: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut N: i32 = 0;
    let mut SRFCDE: i32 = 0;
    let mut TRGCDE: i32 = 0;
    let mut AZFLAG: bool = false;
    let mut ELFLAG: bool = false;
    let mut FOUND: bool = false;

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
    // Local variables
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
    testutil::TOPEN(b"F_AZLCPV", ctx)?;

    //
    // *****************************************************************
    //
    // Test set-up
    //
    // *****************************************************************
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create generic test kernels.", ctx)?;

    //
    // TSTSPK will remove any file named GENSPK from the current
    // environment and register it so that it gets deleted once this
    // test family is finished.
    //
    testutil::TSTSPK(GENSPK, true, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTPCK(GENPCK, true, false, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // *****************************************************************
    //
    // Normal cases: AZLCPO
    //
    // *****************************************************************
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Create a fixed-point SPK and local topocentric frame at that point for AZLCPO testing.",
        ctx,
    )?;

    //
    // Create pinpoint-style SPK file containing data for
    // a fixed "surface point" on the earth. This
    // point has constant position in the IAU_EARTH frame.
    //
    CTRCDE = 399;

    LON = (spicelib::RPD(ctx) * 60.0);
    LAT = (spicelib::RPD(ctx) * 30.0);

    spicelib::SRFREC(CTRCDE, LON, LAT, STATE0.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CLEARD(3, STATE0.subarray_mut(4));

    testutil::KILFIL(FIXSPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKOPN(FIXSPK, FIXSPK, 0, &mut FIXHAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    SRFCDE = 777;
    fstr::assign(&mut FRAME, b"IAU_EARTH");
    FIRST = -((10 as f64) * spicelib::JYEAR());
    LAST = ((10 as f64) * spicelib::JYEAR());
    fstr::assign(&mut SEGID, b"Surface object, constant position.");
    DEGREE = 1;
    N = 2;
    EPOCH1 = FIRST;
    STEP = (LAST - FIRST);
    LAST = intrinsics::DMIN1(&[LAST, STEP]);

    spicelib::MOVED(STATE0.as_slice(), 6, STATES.subarray_mut([1, 1]));
    spicelib::MOVED(STATE0.as_slice(), 6, STATES.subarray_mut([1, 2]));

    spicelib::SPKW08(
        FIXHAN,
        SRFCDE,
        CTRCDE,
        &FRAME,
        FIRST,
        LAST,
        &SEGID,
        DEGREE,
        N,
        STATES.as_slice(),
        EPOCH1,
        STEP,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKCLS(FIXHAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create the topocentric frames centered at the fixed
    // location described by the pinpoint-style SPK file,
    // where:
    //
    //   -  The Z axis of this frame points toward the zenith.
    //
    //   -  The X axis of this frame points North.
    //
    fstr::assign(TEXT.get_mut(1), b"FRAME_777_TOPO           =  1000777");
    fstr::assign(TEXT.get_mut(2), b"FRAME_1000777_NAME       =  \'777_TOPO\'");
    fstr::assign(TEXT.get_mut(3), b"FRAME_1000777_CLASS      =  4");
    fstr::assign(TEXT.get_mut(4), b"FRAME_1000777_CLASS_ID   =  1000777");
    fstr::assign(TEXT.get_mut(5), b"FRAME_1000777_CENTER     =  777");
    fstr::assign(
        TEXT.get_mut(6),
        b"TKFRAME_1000777_RELATIVE =  \'IAU_EARTH\'",
    );
    fstr::assign(TEXT.get_mut(7), b"TKFRAME_1000777_SPEC     =  \'ANGLES\'");
    fstr::assign(TEXT.get_mut(8), b"TKFRAME_1000777_UNITS    =  \'DEGREES\'");
    fstr::assign(TEXT.get_mut(9), b"TKFRAME_1000777_AXES     =  ( 3, 2, 3 )");
    fstr::assign(
        TEXT.get_mut(10),
        b"TKFRAME_1000777_ANGLES   =  (  -60.0000000000000,",
    );
    fstr::assign(
        TEXT.get_mut(11),
        b"                               -59.8330346084233,",
    );
    fstr::assign(
        TEXT.get_mut(12),
        b"                               180.0000000000000 )",
    );

    //
    // Load the SPK and the topocentric frames.
    //
    spicelib::FURNSH(FIXSPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LMPOOL(TEXT.as_arg(), NLINES, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    fstr::assign(&mut METHOD, b"ELLIPSOID");

    SRFCDE = 777;
    fstr::assign(&mut FRAME, b"777_TOPO");

    TRGCDE = 301;
    fstr::assign(&mut TARGET, b"MOON");

    fstr::assign(&mut OBSCTR, b"EARTH");
    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    //
    // Loop through the different aberration corrections, the different
    // AZCCW and ELPLSZ flag configurations and different times.
    //
    for CORIDX in 1..=NCORR {
        for FLGIDX in 1..=NFLAGS {
            for TIMIDX in 1..=NTIMES {
                ET = (((TIMIDX - 10) as f64) * spicelib::JYEAR());

                fstr::assign(
                    &mut TITLE,
                    b"AZLCPO: Abcorr = #; ET = #; AZCCW = #; ELPLSZ = #.",
                );

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                spicelib::REPML(
                    &TITLE.clone(),
                    b"#",
                    save.AZCCW[FLGIDX],
                    b"C",
                    &mut TITLE,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                spicelib::REPML(
                    &TITLE.clone(),
                    b"#",
                    save.ELPLSZ[FLGIDX],
                    b"C",
                    &mut TITLE,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                //
                // Get the position of MOON with respect to our location
                // in the corresponding local-topocentric frame using the
                // SPK and the frame within the kernel pool.
                //
                spicelib::SPKEZ(
                    TRGCDE,
                    ET,
                    &FRAME,
                    &save.ABCORR[CORIDX],
                    SRFCDE,
                    XSTATE.as_slice_mut(),
                    &mut XLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Get the same information in azimuth/elevation coordinates
                // using AZLCPO
                //
                spicelib::AZLCPO(
                    &METHOD,
                    &TARGET,
                    ET,
                    &save.ABCORR[CORIDX],
                    save.AZCCW[FLGIDX],
                    save.ELPLSZ[FLGIDX],
                    STATE0.as_slice(),
                    &OBSCTR,
                    &OBSREF,
                    AZLSTA.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Convert azimuth/elevation to rectangular coordinates.
                //
                spicelib::AZLREC(
                    AZLSTA[1],
                    AZLSTA[2],
                    AZLSTA[3],
                    save.AZCCW[FLGIDX],
                    save.ELPLSZ[FLGIDX],
                    STATE.as_slice_mut(),
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::DRDAZL(
                    AZLSTA[1],
                    AZLSTA[2],
                    AZLSTA[3],
                    save.AZCCW[FLGIDX],
                    save.ELPLSZ[FLGIDX],
                    JACOBI.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::MXV(JACOBI.as_slice(), AZLSTA.subarray(4), STATE.subarray_mut(4));

                //
                // Compare the results.
                //
                testutil::CHCKAD(
                    b"Position",
                    STATE.as_slice(),
                    b"~~/",
                    XSTATE.as_slice(),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKAD(
                    b"Velocity",
                    STATE.subarray(4),
                    b"~~/",
                    XSTATE.subarray(4),
                    3,
                    MED,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, TIGHT, OK, ctx)?;
            }
        }
    }

    //
    // --- Case: -------------------------------------------------------
    //
    // Observer position is on the +Z-axis of the body-fixed. Testing of
    // OBSPOS at the center of the OBSCTR body.
    //
    fstr::assign(&mut METHOD, b"ELLIPSOID");
    fstr::assign(&mut TARGET, b"MARS");
    ET = 0.0;
    fstr::assign(&mut OBSCTR, b"EARTH");
    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    spicelib::CLEARD(3, OBSPOS.as_slice_mut());

    //
    // Loop through the different aberration correction flags and
    // AZCCW and ELPLSZ flag configurations.
    //
    for CORIDX in 1..=NCORR {
        for FLGIDX in 1..=NFLAGS {
            fstr::assign(
                &mut TITLE,
                b"AZLCPO: Abcorr = #; AZCCW = #; ELPLSZ = #. OBSPOS = (0, 0, 0 )",
            );

            spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            spicelib::REPML(
                &TITLE.clone(),
                b"#",
                save.AZCCW[FLGIDX],
                b"C",
                &mut TITLE,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            spicelib::REPML(
                &TITLE.clone(),
                b"#",
                save.ELPLSZ[FLGIDX],
                b"C",
                &mut TITLE,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::TCASE(&TITLE, ctx)?;

            spicelib::SPKEZR(
                &TARGET,
                ET,
                &OBSREF,
                &save.ABCORR[CORIDX],
                &OBSCTR,
                XSTATE.as_slice_mut(),
                &mut XLT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::AZLCPO(
                &METHOD,
                &TARGET,
                ET,
                &save.ABCORR[CORIDX],
                save.AZCCW[FLGIDX],
                save.ELPLSZ[FLGIDX],
                OBSPOS.as_slice(),
                &OBSCTR,
                &OBSREF,
                AZLSTA.as_slice_mut(),
                &mut LT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Convert azimuth/elevation to rectangular coordinates.
            //
            spicelib::AZLREC(
                AZLSTA[1],
                AZLSTA[2],
                AZLSTA[3],
                save.AZCCW[FLGIDX],
                save.ELPLSZ[FLGIDX],
                STATE.as_slice_mut(),
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::DRDAZL(
                AZLSTA[1],
                AZLSTA[2],
                AZLSTA[3],
                save.AZCCW[FLGIDX],
                save.ELPLSZ[FLGIDX],
                JACOBI.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::MXV(JACOBI.as_slice(), AZLSTA.subarray(4), STATE.subarray_mut(4));

            //
            // Compare the results.
            //
            testutil::CHCKAD(
                b"Position",
                STATE.as_slice(),
                b"~~/",
                XSTATE.as_slice(),
                3,
                TIGHT,
                OK,
                ctx,
            )?;

            testutil::CHCKAD(
                b"Velocity",
                STATE.subarray(4),
                b"~~/",
                XSTATE.subarray(4),
                3,
                MED,
                OK,
                ctx,
            )?;

            // CALL CHCKSD ( 'Light time', LT, '~/', XLT,  TIGHT, OK )
        }
    }

    //
    // --- Case: -------------------------------------------------------
    //
    // Observer position is on the -Z-axis of the body-fixed. Testing of
    // OBSPOS very close (~10e-15) to the center of the OBSCTR body.
    //
    fstr::assign(&mut METHOD, b"ELLIPSOID");
    fstr::assign(&mut TARGET, b"MARS");
    ET = 0.0;
    fstr::assign(&mut CORR, b"NONE");
    fstr::assign(&mut OBSCTR, b"EARTH");
    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    OBSPOS[3] = -0.000000000000001;

    //
    // Loop through the different aberration correction flags and
    // AZCCW and ELPLSZ flag configurations.
    //
    for CORIDX in 1..=NCORR {
        for FLGIDX in 1..=NFLAGS {
            fstr::assign(
                &mut TITLE,
                b"AZLCPO: Abcorr = #; AZCCW = #; ELPLSZ = #. OBSPOS = (0, 0, -10e-15 )",
            );

            spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            spicelib::REPML(
                &TITLE.clone(),
                b"#",
                save.AZCCW[FLGIDX],
                b"C",
                &mut TITLE,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            spicelib::REPML(
                &TITLE.clone(),
                b"#",
                save.ELPLSZ[FLGIDX],
                b"C",
                &mut TITLE,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::TCASE(&TITLE, ctx)?;

            spicelib::SPKEZR(
                &TARGET,
                ET,
                &OBSREF,
                &save.ABCORR[CORIDX],
                &OBSCTR,
                XSTATE.as_slice_mut(),
                &mut XLT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Rotate XSTATE 180 degrees about IAU_EARTH +X axis
            //
            spicelib::ROTVEC(
                XSTATE.as_slice(),
                spicelib::PI(ctx),
                1,
                TMPVEC.as_slice_mut(),
                ctx,
            );
            spicelib::VEQU(TMPVEC.as_slice(), XSTATE.as_slice_mut());
            spicelib::ROTVEC(
                XSTATE.subarray(4),
                spicelib::PI(ctx),
                1,
                TMPVEC.as_slice_mut(),
                ctx,
            );
            spicelib::VEQU(TMPVEC.as_slice(), XSTATE.subarray_mut(4));

            spicelib::AZLCPO(
                &METHOD,
                &TARGET,
                ET,
                &save.ABCORR[CORIDX],
                save.AZCCW[FLGIDX],
                save.ELPLSZ[FLGIDX],
                OBSPOS.as_slice(),
                &OBSCTR,
                &OBSREF,
                AZLSTA.as_slice_mut(),
                &mut LT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Convert azimuth/elevation to rectangular coordinates.
            //
            spicelib::AZLREC(
                AZLSTA[1],
                AZLSTA[2],
                AZLSTA[3],
                save.AZCCW[FLGIDX],
                save.ELPLSZ[FLGIDX],
                STATE.as_slice_mut(),
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::DRDAZL(
                AZLSTA[1],
                AZLSTA[2],
                AZLSTA[3],
                save.AZCCW[FLGIDX],
                save.ELPLSZ[FLGIDX],
                JACOBI.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::MXV(JACOBI.as_slice(), AZLSTA.subarray(4), STATE.subarray_mut(4));

            //
            // Compare the results.
            //
            testutil::CHCKAD(
                b"Position",
                STATE.as_slice(),
                b"~~/",
                XSTATE.as_slice(),
                3,
                TIGHT,
                OK,
                ctx,
            )?;

            testutil::CHCKAD(
                b"Velocity",
                STATE.subarray(4),
                b"~~/",
                XSTATE.subarray(4),
                3,
                MED,
                OK,
                ctx,
            )?;

            // CALL CHCKSD ( 'Light time', LT, '~/', XLT,  TIGHT, OK )
        }
    }

    //
    // *****************************************************************
    //
    // Error cases: AZLCPO
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"AZLCPO error case: bad computation method.", ctx)?;

    fstr::assign(&mut METHOD, b"XYZ");
    fstr::assign(&mut TARGET, b"MARS");
    ET = 0.0;
    fstr::assign(&mut CORR, b"NONE");
    AZFLAG = true;
    ELFLAG = true;
    fstr::assign(&mut OBSCTR, b"EARTH");
    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    spicelib::AZLCPO(
        &METHOD,
        &TARGET,
        ET,
        &CORR,
        AZFLAG,
        ELFLAG,
        STATE0.as_slice(),
        &OBSCTR,
        &OBSREF,
        AZLSTA.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDMETHOD)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"AZLCPO error case: bad center name.", ctx)?;

    fstr::assign(&mut METHOD, b"ELLIPSOID");
    fstr::assign(&mut TARGET, b"MARS");
    ET = 0.0;
    fstr::assign(&mut CORR, b"NONE");
    AZFLAG = true;
    ELFLAG = true;
    fstr::assign(&mut OBSCTR, b"XYZ");
    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    spicelib::AZLCPO(
        &METHOD,
        &TARGET,
        ET,
        &CORR,
        AZFLAG,
        ELFLAG,
        STATE0.as_slice(),
        &OBSCTR,
        &OBSREF,
        AZLSTA.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"AZLCPO error case: bad reference frame name.", ctx)?;

    fstr::assign(&mut METHOD, b"ELLIPSOID");
    fstr::assign(&mut TARGET, b"MARS");
    ET = 0.0;
    fstr::assign(&mut CORR, b"NONE");
    AZFLAG = true;
    ELFLAG = true;
    fstr::assign(&mut OBSCTR, b"EARTH");
    fstr::assign(&mut OBSREF, b"IAU_FRAME");

    spicelib::AZLCPO(
        &METHOD,
        &TARGET,
        ET,
        &CORR,
        AZFLAG,
        ELFLAG,
        STATE0.as_slice(),
        &OBSCTR,
        &OBSREF,
        AZLSTA.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(UNKNOWNFRAME)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"AZLCPO error case: frame center is not observer\'s center of motion.",
        ctx,
    )?;

    fstr::assign(&mut METHOD, b"ELLIPSOID");
    fstr::assign(&mut TARGET, b"MARS");
    ET = 0.0;
    fstr::assign(&mut CORR, b"NONE");
    AZFLAG = true;
    ELFLAG = true;
    fstr::assign(&mut OBSCTR, b"EARTH");
    fstr::assign(&mut OBSREF, b"IAU_SATURN");

    spicelib::AZLCPO(
        &METHOD,
        &TARGET,
        ET,
        &CORR,
        AZFLAG,
        ELFLAG,
        STATE0.as_slice(),
        &OBSCTR,
        &OBSREF,
        AZLSTA.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDFRAME)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    // Use as the observer center an integer code that is sure to
    // not be within the test PCK. Integer numbers should pass the "bad
    // center name" test.
    //
    testutil::TCASE(b"AZLCPO error case: radii not present in kernel pool.", ctx)?;

    fstr::assign(&mut METHOD, b"ELLIPSOID");
    fstr::assign(&mut TARGET, b"MARS");
    ET = 0.0;
    fstr::assign(&mut CORR, b"NONE");
    AZFLAG = true;
    ELFLAG = true;
    fstr::assign(&mut OBSCTR, b"399");
    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    //
    // Save the values of the Earth's radii to restore them later on,
    // and delete them from the kernel pool. These values will be
    // restored once we are done with the "bad radii" error cases.
    //
    spicelib::GDPOOL(
        b"BODY399_RADII",
        1,
        3,
        &mut N,
        ERADI.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DVPOOL(b"BODY399_RADII", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::AZLCPO(
        &METHOD,
        &TARGET,
        ET,
        &CORR,
        AZFLAG,
        ELFLAG,
        STATE0.as_slice(),
        &OBSCTR,
        &OBSREF,
        AZLSTA.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"AZLCPO error case: fewer than 3 radii present in kernel pool.",
        ctx,
    )?;

    fstr::assign(&mut METHOD, b"ELLIPSOID");
    fstr::assign(&mut TARGET, b"MARS");
    ET = 0.0;
    fstr::assign(&mut CORR, b"NONE");
    AZFLAG = true;
    ELFLAG = true;
    fstr::assign(&mut OBSCTR, b"earth");
    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    RADII[1] = 3650.0;

    spicelib::DVPOOL(b"BODY399_RADII", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(b"BODY399_RADII", 1, RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::AZLCPO(
        &METHOD,
        &TARGET,
        ET,
        &CORR,
        AZFLAG,
        ELFLAG,
        STATE0.as_slice(),
        &OBSCTR,
        &OBSREF,
        AZLSTA.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDCOUNT)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"AZLCPO error case: more than 3 radii present in kernel pool.",
        ctx,
    )?;

    fstr::assign(&mut METHOD, b"ELLIPSOID");
    fstr::assign(&mut TARGET, b"MARS");
    ET = 0.0;
    fstr::assign(&mut CORR, b"NONE");
    AZFLAG = true;
    ELFLAG = true;
    fstr::assign(&mut OBSCTR, b"earth");
    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    RADII[1] = 3650.0;

    spicelib::DVPOOL(b"BODY399_RADII", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FILLD(1.0, 4, DVALS.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(b"BODY399_RADII", 4, DVALS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::AZLCPO(
        &METHOD,
        &TARGET,
        ET,
        &CORR,
        AZFLAG,
        ELFLAG,
        STATE0.as_slice(),
        &OBSCTR,
        &OBSREF,
        AZLSTA.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(ARRAYTOOSMALL)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"AZLCPO error case: non-positive radii present in kernel pool.",
        ctx,
    )?;

    fstr::assign(&mut METHOD, b"ELLIPSOID");
    fstr::assign(&mut TARGET, b"MARS");
    ET = 0.0;
    fstr::assign(&mut CORR, b"NONE");
    AZFLAG = true;
    ELFLAG = true;
    fstr::assign(&mut OBSCTR, b"Earth");
    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    RADII[1] = 3650.0;
    RADII[2] = 0.0;
    RADII[3] = 3650.0;

    spicelib::DVPOOL(b"BODY399_RADII", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(b"BODY399_RADII", 3, RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::AZLCPO(
        &METHOD,
        &TARGET,
        ET,
        &CORR,
        AZFLAG,
        ELFLAG,
        STATE0.as_slice(),
        &OBSCTR,
        &OBSREF,
        AZLSTA.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"AZLCPO error case: ratio of longest to shortest radii present in kernel pool too large.",
        ctx,
    )?;

    fstr::assign(&mut METHOD, b"ELLIPSOID");
    fstr::assign(&mut TARGET, b"MARS");
    ET = 0.0;
    fstr::assign(&mut CORR, b"NONE");
    AZFLAG = true;
    ELFLAG = true;
    fstr::assign(&mut OBSCTR, b"EARTH");
    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    RADII[1] = spicelib::DPMAX();
    RADII[2] = 1.0;
    RADII[3] = 3650.0;

    spicelib::DVPOOL(b"BODY399_RADII", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(b"BODY399_RADII", 3, RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::AZLCPO(
        &METHOD,
        &TARGET,
        ET,
        &CORR,
        AZFLAG,
        ELFLAG,
        STATE0.as_slice(),
        &OBSCTR,
        &OBSREF,
        AZLSTA.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"AZLCPO error case: ratio of radii vs. observer position magnitude too large.",
        ctx,
    )?;

    //
    // Note: this is an error detected by NEARPT.
    //
    fstr::assign(&mut METHOD, b"ELLIPSOID");
    fstr::assign(&mut TARGET, b"MARS");
    ET = 0.0;
    fstr::assign(&mut CORR, b"NONE");
    AZFLAG = true;
    ELFLAG = true;
    fstr::assign(&mut OBSCTR, b"EaRTH");
    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    RADII[1] = 10.0;
    RADII[2] = 10.0;
    RADII[3] = 10.0;

    BADPOS[1] = spicelib::DPMAX();
    BADPOS[2] = STATE0[2];
    BADPOS[3] = STATE0[3];

    spicelib::DVPOOL(b"BODY399_RADII", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(b"BODY399_RADII", 3, RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::AZLCPO(
        &METHOD,
        &TARGET,
        ET,
        &CORR,
        AZFLAG,
        ELFLAG,
        BADPOS.as_slice(),
        &OBSCTR,
        &OBSREF,
        AZLSTA.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INPUTSTOOLARGE)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"AZLCPO error case: bad target name.", ctx)?;

    //
    // Restore the values of the Earth's radii
    //
    spicelib::PDPOOL(b"BODY399_RADII", 3, ERADI.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut METHOD, b"ELLIPSOID");
    fstr::assign(&mut TARGET, b"XYZ");
    ET = 0.0;
    fstr::assign(&mut CORR, b"NONE");
    AZFLAG = true;
    ELFLAG = true;
    fstr::assign(&mut OBSCTR, b"EARTH");
    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    spicelib::AZLCPO(
        &METHOD,
        &TARGET,
        ET,
        &CORR,
        AZFLAG,
        ELFLAG,
        STATE0.as_slice(),
        &OBSCTR,
        &OBSREF,
        AZLSTA.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"AZLCPO error case: bad aberration correction.", ctx)?;

    fstr::assign(&mut METHOD, b"ELLIPSOID");
    fstr::assign(&mut TARGET, b"MARS");
    ET = 0.0;
    fstr::assign(&mut CORR, b"L+S");
    AZFLAG = true;
    ELFLAG = true;
    fstr::assign(&mut OBSCTR, b"EARTH");
    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    spicelib::AZLCPO(
        &METHOD,
        &TARGET,
        ET,
        &CORR,
        AZFLAG,
        ELFLAG,
        STATE0.as_slice(),
        &OBSCTR,
        &OBSREF,
        AZLSTA.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"AZLCPO error case: no target data.", ctx)?;

    fstr::assign(&mut METHOD, b"ELLIPSOID");
    fstr::assign(&mut TARGET, b"666");
    ET = 0.0;
    fstr::assign(&mut CORR, b"NONE");
    AZFLAG = true;
    ELFLAG = true;
    fstr::assign(&mut OBSCTR, b"EARTH");
    fstr::assign(&mut OBSREF, b"IAU_EARTH");

    spicelib::AZLCPO(
        &METHOD,
        &TARGET,
        ET,
        &CORR,
        AZFLAG,
        ELFLAG,
        STATE0.as_slice(),
        &OBSCTR,
        &OBSREF,
        AZLSTA.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    // In order to test the insufficient data for the center of motion,
    // we need an SPK object that has data in the test PCK, but not in
    // the test SPK: use 'BELINDA'
    //
    testutil::TCASE(
        b"AZLCPO error case: no observer\'s center of motion data.",
        ctx,
    )?;

    fstr::assign(&mut METHOD, b"ELLIPSOID");
    fstr::assign(&mut TARGET, b"MARS");
    ET = 0.0;
    fstr::assign(&mut CORR, b"NONE");
    AZFLAG = true;
    ELFLAG = true;
    fstr::assign(&mut OBSCTR, b"BELINDA");
    fstr::assign(&mut OBSREF, b"IAU_BELINDA");

    spicelib::AZLCPO(
        &METHOD,
        &TARGET,
        ET,
        &CORR,
        AZFLAG,
        ELFLAG,
        STATE0.as_slice(),
        &OBSCTR,
        &OBSREF,
        AZLSTA.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    // The Jacobian matrix of the transformation from rectangular to
    // azimuth/elevation coordinates has a singularity for points
    // located on the Z-axis, which triggers an error.
    //
    // In order to trigger this error, set the observer on the MOON
    // equator, with Y = 0 and look for the AZ/EL of the Moon's center.
    //
    testutil::TCASE(b"AZLCPO error case: singularity of Jacobian matrix of the transformation from rectangular to AZ/EL.", ctx)?;

    fstr::assign(&mut METHOD, b"ELLIPSOID");
    fstr::assign(&mut TARGET, b"MOON");
    ET = 0.0;
    fstr::assign(&mut CORR, b"NONE");
    AZFLAG = true;
    ELFLAG = true;
    fstr::assign(&mut OBSCTR, b"MOON");
    fstr::assign(&mut OBSREF, b"IAU_MOON");

    OBSPOS[1] = 1737.4;
    OBSPOS[2] = 0.0;
    OBSPOS[3] = 0.0;

    spicelib::AZLCPO(
        &METHOD,
        &TARGET,
        ET,
        &CORR,
        AZFLAG,
        ELFLAG,
        OBSPOS.as_slice(),
        &OBSCTR,
        &OBSREF,
        AZLSTA.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(POINTONZAXIS)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up SPK files", ctx)?;

    //
    // Clean the environment: remove SPK files and clear the kernel
    // pool. Note that GENSPK and GENPCK are supposed to be removed
    // automatically.
    //
    spicelib::UNLOAD(FIXSPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(FIXSPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
