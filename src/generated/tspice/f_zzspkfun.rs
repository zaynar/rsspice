//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const TIGHT: f64 = 0.00000000001;
const VTIGHT: f64 = 0.0000000000001;
const LNSIZE: i32 = 320;
const FRNMLN: i32 = 32;
const NCORR: i32 = 9;
const NTIMES: i32 = 10;
const CORLEN: i32 = 25;
const NFRAME: i32 = 5;

struct SaveVars {
    ABCORR: ActualCharArray,
    FRAMES: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ABCORR = ActualCharArray::new(CORLEN, 1..=NCORR);
        let mut FRAMES = ActualCharArray::new(FRNMLN, 1..=NFRAME);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b" nOne"),
                Val::C(b" lT"),
                Val::C(b" Lt + s"),
                Val::C(b"  Cn"),
                Val::C(b"cN + S"),
                Val::C(b" xlT"),
                Val::C(b" XLt + s"),
                Val::C(b"  xCn"),
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
                Val::C(b"J2000"),
                Val::C(b"MARSIAU"),
                Val::C(b"IAU_EARTH"),
                Val::C(b"IAU_MARS"),
                Val::C(b"IAU_JUPITER"),
            ]
            .into_iter();
            FRAMES
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { ABCORR, FRAMES }
    }
}

//$Procedure F_ZZSPKFUN ( Family of tests for SPK function routines )
pub fn F_ZZSPKFUN(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ERROR = [b' '; LNSIZE as usize];
    let mut REF = [b' '; FRNMLN as usize];
    let mut TITLE = [b' '; LNSIZE as usize];
    let mut ACCOBS = StackArray::<f64, 3>::new(1..=3);
    let mut CLT: f64 = 0.0;
    let mut DELTA: f64 = 0.0;
    let mut DLT: f64 = 0.0;
    let mut ELT: f64 = 0.0;
    let mut ESTATE = StackArray::<f64, 6>::new(1..=6);
    let mut ET: f64 = 0.0;
    let mut ISTATE = StackArray::<f64, 6>::new(1..=6);
    let mut LT: f64 = 0.0;
    let mut OBSSSB = StackArray2D::<f64, 12>::new(1..=6, 1..=2);
    let mut STATE = StackArray::<f64, 6>::new(1..=6);
    let mut STOBS = StackArray::<f64, 6>::new(1..=6);
    let mut TSTATE = StackArray::<f64, 6>::new(1..=6);
    let mut XDLT: f64 = 0.0;
    let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XLT: f64 = 0.0;
    let mut XSTATE = StackArray::<f64, 6>::new(1..=6);
    let mut CTRCDE: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut OBSCDE: i32 = 0;
    let mut OUTCTR: i32 = 0;
    let mut TRGCDE: i32 = 0;

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
    testutil::TOPEN(b"F_ZZSPKFUN", ctx)?;

    //*****************************************************************
    //*                                                               *
    //*                                                               *
    //*    Set-up                                                     *
    //*                                                               *
    //*                                                               *
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Create and load SPK and PCK.
    //
    testutil::TCASE(b"Create kernels.", ctx)?;

    testutil::KILFIL(b"spkfun.bsp", ctx)?;
    testutil::KILFIL(b"spkfun.ker", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTSPK(b"spkfun.bsp", true, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTPCK(b"spkfun.ker", true, false, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::TPARSE(b"1 JAN 1995", &mut ET, &mut ERROR, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Exercise the test utility routines located at the bottom
    // of this file.
    //
    testutil::TCASE(b"Exercise the local SPK set/get utilities.", ctx)?;

    TRGCDE = 399;
    CTRCDE = 3;

    F_ZZSPKSET(TRGCDE, CTRCDE, ctx);

    ET = ((10 as f64) * spicelib::JYEAR());
    fstr::assign(&mut REF, b"IAU_EARTH");

    spicelib::SPKGEO(
        TRGCDE,
        ET,
        &REF,
        CTRCDE,
        XSTATE.as_slice_mut(),
        &mut XLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    F_ZZSPKGET(ET, &REF, &mut OUTCTR, STATE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure the correct center of motion is returned.
    //
    testutil::CHCKSI(b"Center", OUTCTR, b"=", CTRCDE, 0, OK, ctx)?;

    //
    // Check the output state. We expect exact equality.
    //
    testutil::CHCKAD(
        b"Position",
        STATE.as_slice(),
        b"=",
        XSTATE.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"Velocity",
        STATE.subarray(4),
        b"=",
        XSTATE.subarray(4),
        3,
        0.0,
        OK,
        ctx,
    )?;

    //*****************************************************************
    //*                                                               *
    //*                                                               *
    //*    ZZSPKFZT tests                                             *
    //*                                                               *
    //*                                                               *
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: bad aberration correction.", ctx)?;

    TRGCDE = 399;
    CTRCDE = 3;
    OBSCDE = 4;

    F_ZZSPKSET(TRGCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    spicelib::ZZSPKFZT(
        F_ZZSPKGET,
        ET,
        &REF,
        b"L+S",
        OBSCDE,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: bad frame name", ctx)?;

    TRGCDE = 399;
    CTRCDE = 3;
    OBSCDE = 4;

    F_ZZSPKSET(TRGCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut REF, b"J200");

    ET = spicelib::JYEAR();

    spicelib::ZZSPKFZT(
        F_ZZSPKGET,
        ET,
        &REF,
        b"LT+S",
        OBSCDE,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(UNKNOWNFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: no target data.", ctx)?;

    TRGCDE = 777;
    CTRCDE = 3;
    OBSCDE = 4;

    F_ZZSPKSET(TRGCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    spicelib::ZZSPKFZT(
        F_ZZSPKGET,
        ET,
        &REF,
        b"LT+S",
        OBSCDE,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: no target center data.", ctx)?;

    TRGCDE = 7;
    CTRCDE = 777;
    OBSCDE = 4;

    F_ZZSPKSET(TRGCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    spicelib::ZZSPKFZT(
        F_ZZSPKGET,
        ET,
        &REF,
        b"LT+S",
        OBSCDE,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: no observer data.", ctx)?;

    TRGCDE = 5;
    CTRCDE = 3;
    OBSCDE = 444;

    F_ZZSPKSET(TRGCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    spicelib::ZZSPKFZT(
        F_ZZSPKGET,
        ET,
        &REF,
        b"LT+S",
        OBSCDE,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    //
    // ZZSPKFZT normal cases:
    //
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check ZZSPKFZT for every aberration correction. Use different frames. Target = Earth. Center = Earth-Moon barycenter. Observer = Mars.", ctx)?;

    TRGCDE = 399;
    CTRCDE = 3;
    OBSCDE = 499;

    F_ZZSPKSET(TRGCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        for FRMIDX in 1..=NFRAME {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = ((TIMIDX as f64) * spicelib::JYEAR());

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(
                    &mut TITLE,
                    b"ZZSPKFZT: Target = #; Observer = #; Ref = # Abcorr = #; ET = #.",
                );

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                spicelib::SPKEZ(
                    TRGCDE,
                    ET,
                    &REF,
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    XSTATE.as_slice_mut(),
                    &mut XLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::ZZSPKFZT(
                    F_ZZSPKGET,
                    ET,
                    &REF,
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    STATE.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

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
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, TIGHT, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Check ZZSPKFZT: change observer, target, and center, and repeat previous tests.",
        ctx,
    )?;

    TRGCDE = 599;
    CTRCDE = 5;
    OBSCDE = 399;

    F_ZZSPKSET(TRGCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        for FRMIDX in 1..=NFRAME {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = ((TIMIDX as f64) * spicelib::JYEAR());

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(
                    &mut TITLE,
                    b"ZZSPKFZT: Target = #; Observer = #; Ref = # Abcorr = #; ET = #.",
                );

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                spicelib::SPKEZ(
                    TRGCDE,
                    ET,
                    &REF,
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    XSTATE.as_slice_mut(),
                    &mut XLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::ZZSPKFZT(
                    F_ZZSPKGET,
                    ET,
                    &REF,
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    STATE.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

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
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, TIGHT, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //*****************************************************************
    //*                                                               *
    //*                                                               *
    //*    ZZSPKFZO tests                                             *
    //*                                                               *
    //*                                                               *
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: bad aberration correction.", ctx)?;

    TRGCDE = 399;
    CTRCDE = 3;
    OBSCDE = 4;

    F_ZZSPKSET(OBSCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    spicelib::ZZSPKFZO(
        TRGCDE,
        ET,
        &REF,
        b"L+S",
        F_ZZSPKGET,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: bad frame name", ctx)?;

    TRGCDE = 399;
    CTRCDE = 3;
    OBSCDE = 4;

    F_ZZSPKSET(OBSCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut REF, b"J200");

    ET = spicelib::JYEAR();

    spicelib::ZZSPKFZO(
        TRGCDE,
        ET,
        &REF,
        b"Lt+S",
        F_ZZSPKGET,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(UNKNOWNFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: no target data.", ctx)?;

    TRGCDE = 777;
    CTRCDE = 3;
    OBSCDE = 4;

    F_ZZSPKSET(OBSCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    spicelib::ZZSPKFZO(
        TRGCDE,
        ET,
        &REF,
        b"Lt+S",
        F_ZZSPKGET,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: no observer data.", ctx)?;

    TRGCDE = 5;
    CTRCDE = 3;
    OBSCDE = 444;

    F_ZZSPKSET(OBSCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    spicelib::ZZSPKFZO(
        TRGCDE,
        ET,
        &REF,
        b"Lt+S",
        F_ZZSPKGET,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: no observer center data.", ctx)?;

    TRGCDE = 5;
    CTRCDE = 455;
    OBSCDE = 4;

    F_ZZSPKSET(OBSCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    spicelib::ZZSPKFZO(
        TRGCDE,
        ET,
        &REF,
        b"Lt+S",
        F_ZZSPKGET,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    //
    // ZZSPKFZO normal cases:
    //
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check ZZSPKFZO for every aberration correction. Use different frames. Target = Earth. Center = Earth-Moon barycenter. Observer = Mars.", ctx)?;

    TRGCDE = 399;
    CTRCDE = 3;
    OBSCDE = 499;

    F_ZZSPKSET(OBSCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        for FRMIDX in 1..=NFRAME {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = ((TIMIDX as f64) * spicelib::JYEAR());

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(
                    &mut TITLE,
                    b"ZZSPKFZO: Target = #; Observer = #; Ref = # Abcorr = #; ET = #.",
                );

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                spicelib::SPKEZ(
                    TRGCDE,
                    ET,
                    &REF,
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    XSTATE.as_slice_mut(),
                    &mut XLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::ZZSPKFZO(
                    TRGCDE,
                    ET,
                    &REF,
                    &save.ABCORR[CORIDX],
                    F_ZZSPKGET,
                    STATE.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

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
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, TIGHT, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Check ZZSPKFZO: change observer, target, and center, and repeat previous tests.",
        ctx,
    )?;

    TRGCDE = 599;
    CTRCDE = 5;
    OBSCDE = 399;

    F_ZZSPKSET(OBSCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        for FRMIDX in 1..=NFRAME {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = ((TIMIDX as f64) * spicelib::JYEAR());

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(
                    &mut TITLE,
                    b"ZZSPKFZO: Target = #; Observer = #; Ref = # Abcorr = #; ET = #.",
                );

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                spicelib::SPKEZ(
                    TRGCDE,
                    ET,
                    &REF,
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    XSTATE.as_slice_mut(),
                    &mut XLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::ZZSPKFZO(
                    TRGCDE,
                    ET,
                    &REF,
                    &save.ABCORR[CORIDX],
                    F_ZZSPKGET,
                    STATE.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

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
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, TIGHT, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //*****************************************************************
    //*                                                               *
    //*                                                               *
    //*    ZZSPKFAT tests                                             *
    //*                                                               *
    //*                                                               *
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: bad aberration correction.", ctx)?;

    TRGCDE = 399;
    CTRCDE = 3;
    OBSCDE = 4;

    F_ZZSPKSET(TRGCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    spicelib::ZZSPKFAT(
        F_ZZSPKGET,
        ET,
        &REF,
        b"L+S",
        OBSCDE,
        STATE.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: bad frame name", ctx)?;

    TRGCDE = 399;
    CTRCDE = 3;
    OBSCDE = 4;

    F_ZZSPKSET(TRGCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut REF, b"J200");

    ET = spicelib::JYEAR();

    spicelib::ZZSPKFAT(
        F_ZZSPKGET,
        ET,
        &REF,
        b"LT+S",
        OBSCDE,
        STATE.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: non-inertial frame", ctx)?;

    TRGCDE = 399;
    CTRCDE = 3;
    OBSCDE = 4;

    F_ZZSPKSET(TRGCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut REF, b"IAU_EARTH");

    ET = spicelib::JYEAR();

    spicelib::ZZSPKFAT(
        F_ZZSPKGET,
        ET,
        &REF,
        b"LT+S",
        OBSCDE,
        STATE.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: no target data.", ctx)?;

    TRGCDE = 777;
    CTRCDE = 3;
    OBSCDE = 4;

    F_ZZSPKSET(TRGCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    spicelib::ZZSPKFAT(
        F_ZZSPKGET,
        ET,
        &REF,
        b"LT+S",
        OBSCDE,
        STATE.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: no target center data.", ctx)?;

    TRGCDE = 7;
    CTRCDE = 777;
    OBSCDE = 4;

    F_ZZSPKSET(TRGCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    spicelib::ZZSPKFAT(
        F_ZZSPKGET,
        ET,
        &REF,
        b"LT+S",
        OBSCDE,
        STATE.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: no observer data.", ctx)?;

    TRGCDE = 5;
    CTRCDE = 3;
    OBSCDE = 444;

    F_ZZSPKSET(TRGCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    spicelib::ZZSPKFAT(
        F_ZZSPKGET,
        ET,
        &REF,
        b"LT+S",
        OBSCDE,
        STATE.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    //
    // ZZSPKFAT normal cases:
    //
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check ZZSPKFAT for every aberration correction. Use different frames. Target = Earth. Center = Earth-Moon barycenter. Observer = Mars.", ctx)?;

    TRGCDE = 399;
    CTRCDE = 3;
    OBSCDE = 499;

    F_ZZSPKSET(TRGCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        //
        // Use only inertial frames.
        //
        for FRMIDX in 1..=2 {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = ((TIMIDX as f64) * spicelib::JYEAR());

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(
                    &mut TITLE,
                    b"ZZSPKFAT: Target = #; Observer = #; Ref = # Abcorr = #; ET = #.",
                );

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                spicelib::SPKACS(
                    TRGCDE,
                    ET,
                    &REF,
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    XSTATE.as_slice_mut(),
                    &mut XLT,
                    &mut XDLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::ZZSPKFAT(
                    F_ZZSPKGET,
                    ET,
                    &REF,
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    STATE.as_slice_mut(),
                    &mut LT,
                    &mut DLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

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
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, TIGHT, OK, ctx)?;

                testutil::CHCKSD(b"Light time rate", DLT, b"~/", XDLT, TIGHT, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Check ZZSPKFAT: change observer, target, and center, and repeat previous tests.",
        ctx,
    )?;

    TRGCDE = 599;
    CTRCDE = 5;
    OBSCDE = 399;

    F_ZZSPKSET(TRGCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        //
        // Use only inertial frames.
        //
        for FRMIDX in 1..=2 {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = ((TIMIDX as f64) * spicelib::JYEAR());

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(
                    &mut TITLE,
                    b"ZZSPKFAT: Target = #; Observer = #; Ref = # Abcorr = #; ET = #.",
                );

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                spicelib::SPKACS(
                    TRGCDE,
                    ET,
                    &REF,
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    XSTATE.as_slice_mut(),
                    &mut XLT,
                    &mut XDLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::ZZSPKFAT(
                    F_ZZSPKGET,
                    ET,
                    &REF,
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    STATE.as_slice_mut(),
                    &mut LT,
                    &mut DLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

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
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, TIGHT, OK, ctx)?;

                testutil::CHCKSD(b"Light time rate", DLT, b"~/", XDLT, TIGHT, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //*****************************************************************
    //*                                                               *
    //*                                                               *
    //*    ZZSPKFAO tests                                             *
    //*                                                               *
    //*                                                               *
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: bad aberration correction.", ctx)?;

    TRGCDE = 399;
    CTRCDE = 3;
    OBSCDE = 4;

    F_ZZSPKSET(OBSCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    spicelib::ZZSPKFAO(
        TRGCDE,
        ET,
        &REF,
        b"L+S",
        F_ZZSPKGET,
        STATE.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: bad frame name", ctx)?;

    TRGCDE = 399;
    CTRCDE = 3;
    OBSCDE = 4;

    F_ZZSPKSET(OBSCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut REF, b"J200");

    ET = spicelib::JYEAR();

    spicelib::ZZSPKFAO(
        TRGCDE,
        ET,
        &REF,
        b"Lt+S",
        F_ZZSPKGET,
        STATE.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: non-inertial frame.", ctx)?;

    TRGCDE = 399;
    CTRCDE = 3;
    OBSCDE = 4;

    F_ZZSPKSET(OBSCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut REF, b"IAU_EARTH");

    ET = spicelib::JYEAR();

    spicelib::ZZSPKFAO(
        TRGCDE,
        ET,
        &REF,
        b"Lt+S",
        F_ZZSPKGET,
        STATE.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: no target data.", ctx)?;

    TRGCDE = 777;
    CTRCDE = 3;
    OBSCDE = 4;

    F_ZZSPKSET(OBSCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    spicelib::ZZSPKFAO(
        TRGCDE,
        ET,
        &REF,
        b"Lt+S",
        F_ZZSPKGET,
        STATE.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: no observer data.", ctx)?;

    TRGCDE = 5;
    CTRCDE = 3;
    OBSCDE = 444;

    F_ZZSPKSET(OBSCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    spicelib::ZZSPKFAO(
        TRGCDE,
        ET,
        &REF,
        b"Lt+S",
        F_ZZSPKGET,
        STATE.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: no observer center data.", ctx)?;

    TRGCDE = 5;
    CTRCDE = 455;
    OBSCDE = 4;

    F_ZZSPKSET(OBSCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    spicelib::ZZSPKFAO(
        TRGCDE,
        ET,
        &REF,
        b"Lt+S",
        F_ZZSPKGET,
        STATE.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    //
    // ZZSPKFAO normal cases:
    //
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check ZZSPKFAO for every aberration correction. Use different frames. Target = Earth. Center = Earth-Moon barycenter. Observer = Mars.", ctx)?;

    TRGCDE = 399;
    CTRCDE = 3;
    OBSCDE = 499;

    F_ZZSPKSET(OBSCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        //
        // Use only inertial frames.
        //
        for FRMIDX in 1..=2 {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = ((TIMIDX as f64) * spicelib::JYEAR());

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(
                    &mut TITLE,
                    b"ZZSPKFAO: Target = #; Observer = #; Ref = # Abcorr = #; ET = #.",
                );

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                spicelib::SPKACS(
                    TRGCDE,
                    ET,
                    &REF,
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    XSTATE.as_slice_mut(),
                    &mut XLT,
                    &mut XDLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::ZZSPKFAO(
                    TRGCDE,
                    ET,
                    &REF,
                    &save.ABCORR[CORIDX],
                    F_ZZSPKGET,
                    STATE.as_slice_mut(),
                    &mut LT,
                    &mut DLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

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
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, TIGHT, OK, ctx)?;

                testutil::CHCKSD(b"Light time rate", DLT, b"~/", XDLT, TIGHT, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Check ZZSPKFAO: change observer, target, and center, and repeat previous tests.",
        ctx,
    )?;

    TRGCDE = 599;
    CTRCDE = 5;
    OBSCDE = 399;

    F_ZZSPKSET(OBSCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        //
        // Use only inertial frames.
        //
        for FRMIDX in 1..=2 {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = ((TIMIDX as f64) * spicelib::JYEAR());

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(
                    &mut TITLE,
                    b"ZZSPKFAO: Target = #; Observer = #; Ref = # Abcorr = #; ET = #.",
                );

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                spicelib::SPKACS(
                    TRGCDE,
                    ET,
                    &REF,
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    XSTATE.as_slice_mut(),
                    &mut XLT,
                    &mut XDLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::ZZSPKFAO(
                    TRGCDE,
                    ET,
                    &REF,
                    &save.ABCORR[CORIDX],
                    F_ZZSPKGET,
                    STATE.as_slice_mut(),
                    &mut LT,
                    &mut DLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

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
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, TIGHT, OK, ctx)?;

                testutil::CHCKSD(b"Light time rate", DLT, b"~/", XDLT, TIGHT, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //*****************************************************************
    //*                                                               *
    //*                                                               *
    //*    ZZSPKFAP tests                                             *
    //*                                                               *
    //*                                                               *
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: bad aberration correction.", ctx)?;

    TRGCDE = 399;
    CTRCDE = 3;
    OBSCDE = 4;

    F_ZZSPKSET(TRGCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    spicelib::SPKSSB(OBSCDE, ET, &REF, STOBS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CLEARD(3, ACCOBS.as_slice_mut());

    spicelib::ZZSPKFAP(
        F_ZZSPKGET,
        ET,
        &REF,
        b"L+S",
        STOBS.as_slice(),
        ACCOBS.as_slice(),
        STATE.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: bad frame name", ctx)?;

    TRGCDE = 399;
    CTRCDE = 3;
    OBSCDE = 4;

    F_ZZSPKSET(TRGCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut REF, b"J200");

    ET = spicelib::JYEAR();

    spicelib::SPKSSB(OBSCDE, ET, b"J2000", STOBS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZSPKFAP(
        F_ZZSPKGET,
        ET,
        &REF,
        b"LT+S",
        STOBS.as_slice(),
        ACCOBS.as_slice(),
        STATE.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: non-inertial frame", ctx)?;

    TRGCDE = 399;
    CTRCDE = 3;
    OBSCDE = 4;

    F_ZZSPKSET(TRGCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut REF, b"IAU_EARTH");

    ET = spicelib::JYEAR();

    spicelib::SPKSSB(OBSCDE, ET, b"J2000", STOBS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZSPKFAP(
        F_ZZSPKGET,
        ET,
        &REF,
        b"LT+S",
        STOBS.as_slice(),
        ACCOBS.as_slice(),
        STATE.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: no target data.", ctx)?;

    TRGCDE = 777;
    CTRCDE = 3;
    OBSCDE = 4;

    F_ZZSPKSET(TRGCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    spicelib::SPKSSB(OBSCDE, ET, &REF, STOBS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZSPKFAP(
        F_ZZSPKGET,
        ET,
        &REF,
        b"LT+S",
        STOBS.as_slice(),
        ACCOBS.as_slice(),
        STATE.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: no target center data.", ctx)?;

    TRGCDE = 5;
    CTRCDE = 333;
    OBSCDE = 3;

    F_ZZSPKSET(TRGCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    spicelib::SPKSSB(3, ET, &REF, STOBS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZSPKFAP(
        F_ZZSPKGET,
        ET,
        &REF,
        b"LT+S",
        STOBS.as_slice(),
        ACCOBS.as_slice(),
        STATE.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    //
    // ZZSPKFAP normal cases:
    //
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check ZZSPKFAP for every aberration correction. Use the J2000 frame. Target = Earth. Center = Moon. Observer = Jupiter barycenter.", ctx)?;

    TRGCDE = 399;
    CTRCDE = 301;
    OBSCDE = 5;

    F_ZZSPKSET(TRGCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        //
        // Use J2000 and a second inertial frame.
        //
        for FRMIDX in 1..=2 {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = ((TIMIDX as f64) * spicelib::JYEAR());
                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(
                    &mut TITLE,
                    b"ZZSPKFAP: Target = #; Observer = #; Center = #; Ref = # Abcorr = #; ET = #.",
                );

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", CTRCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                //
                // We need the state of the observer relative to the
                // SSB at ET.
                //
                spicelib::SPKSSB(OBSCDE, ET, &REF, STOBS.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Compute observer acceleration relative to the
                // solar system barycenter.
                //
                for I in 1..=2 {
                    DELTA = (((2 * I) - 3) as f64);

                    spicelib::SPKSSB(OBSCDE, (ET + DELTA), &REF, OBSSSB.subarray_mut([1, I]), ctx)?;
                }

                spicelib::QDERIV(
                    3,
                    OBSSSB.subarray([4, 1]),
                    OBSSSB.subarray([4, 2]),
                    1.0,
                    ACCOBS.as_slice_mut(),
                    ctx,
                )?;

                //
                // Get expected state, light time, and light time rate.
                //
                spicelib::SPKACS(
                    TRGCDE,
                    ET,
                    &REF,
                    &save.ABCORR[CORIDX],
                    OBSCDE,
                    XSTATE.as_slice_mut(),
                    &mut XLT,
                    &mut XDLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::ZZSPKFAP(
                    F_ZZSPKGET,
                    ET,
                    &REF,
                    &save.ABCORR[CORIDX],
                    STOBS.as_slice(),
                    ACCOBS.as_slice(),
                    STATE.as_slice_mut(),
                    &mut LT,
                    &mut DLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

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
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, TIGHT, OK, ctx)?;

                testutil::CHCKSD(b"Light time rate", DLT, b"~/", XDLT, TIGHT, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //*****************************************************************
    //*                                                               *
    //*                                                               *
    //*    ZZSPKFLT tests                                             *
    //*                                                               *
    //*                                                               *
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: bad aberration correction.", ctx)?;

    TRGCDE = 399;
    CTRCDE = 3;
    OBSCDE = 4;

    F_ZZSPKSET(TRGCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    spicelib::SPKSSB(OBSCDE, ET, &REF, STOBS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZSPKFLT(
        F_ZZSPKGET,
        ET,
        &REF,
        b"L+S",
        STOBS.as_slice(),
        STATE.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: bad frame name", ctx)?;

    TRGCDE = 399;
    CTRCDE = 3;
    OBSCDE = 4;

    F_ZZSPKSET(TRGCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut REF, b"J200");

    ET = spicelib::JYEAR();

    spicelib::SPKSSB(OBSCDE, ET, b"J2000", STOBS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZSPKFLT(
        F_ZZSPKGET,
        ET,
        &REF,
        b"LT+S",
        STOBS.as_slice(),
        STATE.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(UNKNOWNFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: no target data.", ctx)?;

    TRGCDE = 777;
    CTRCDE = 3;
    OBSCDE = 4;

    F_ZZSPKSET(TRGCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    spicelib::SPKSSB(OBSCDE, ET, &REF, STOBS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZSPKFLT(
        F_ZZSPKGET,
        ET,
        &REF,
        b"LT+S",
        STOBS.as_slice(),
        STATE.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: no target center data.", ctx)?;

    TRGCDE = 5;
    CTRCDE = 333;
    OBSCDE = 3;

    F_ZZSPKSET(TRGCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut REF, b"J2000");

    ET = spicelib::JYEAR();

    spicelib::SPKSSB(3, ET, &REF, STOBS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZSPKFLT(
        F_ZZSPKGET,
        ET,
        &REF,
        b"LT+S",
        STOBS.as_slice(),
        STATE.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    //
    // ZZSPKFLT normal cases:
    //
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check ZZSPKFLT for every aberration correction. Use the J2000 frame. Target = Earth. Center = Moon. Observer = Jupiter barycenter.", ctx)?;

    TRGCDE = 399;
    CTRCDE = 301;
    OBSCDE = 5;

    F_ZZSPKSET(TRGCDE, CTRCDE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for CORIDX in 1..=NCORR {
        //
        // Use J2000 and a second inertial frame.
        //
        for FRMIDX in 1..=2 {
            fstr::assign(&mut REF, save.FRAMES.get(FRMIDX));

            for TIMIDX in 1..=NTIMES {
                ET = ((TIMIDX as f64) * spicelib::JYEAR());
                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(
                    &mut TITLE,
                    b"ZZSPKFLT: Target = #; Observer = #; Center = #; Ref = # Abcorr = #; ET = #.",
                );

                spicelib::REPMI(&TITLE.clone(), b"#", TRGCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", OBSCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(&TITLE.clone(), b"#", CTRCDE, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &REF, &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[CORIDX], &mut TITLE);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&TITLE, ctx)?;

                //
                // We need the state of the observer relative to the
                // SSB at ET.
                //
                spicelib::SPKSSB(OBSCDE, ET, &REF, STOBS.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Get expected state, light time, and light time rate.
                //
                spicelib::SPKLTC(
                    TRGCDE,
                    ET,
                    &REF,
                    &save.ABCORR[CORIDX],
                    STOBS.as_slice(),
                    XSTATE.as_slice_mut(),
                    &mut XLT,
                    &mut XDLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::ZZSPKFLT(
                    F_ZZSPKGET,
                    ET,
                    &REF,
                    &save.ABCORR[CORIDX],
                    STOBS.as_slice(),
                    STATE.as_slice_mut(),
                    &mut LT,
                    &mut DLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

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
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(b"Light time", LT, b"~/", XLT, TIGHT, OK, ctx)?;

                testutil::CHCKSD(b"Light time rate", DLT, b"~/", XDLT, TIGHT, OK, ctx)?;
            }
            //
            // End of time loop.
            //
        }
        //
        // End of frame loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //*****************************************************************
    //*                                                               *
    //*                                                               *
    //*    SPKACS tests                                             *
    //*                                                               *
    //*                                                               *
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Make sure that SPKEZ returns the same thing as SPKACS when an inertial frame is the requested output frame. Converged Newtonian plus stellar aberration. Reception case.", ctx)?;

    spicelib::SPKEZ(
        301001,
        ET,
        b"J2000",
        b"CN+S",
        399001,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKACS(
        301001,
        ET,
        b"J2000",
        b"CN+S",
        399001,
        ESTATE.as_slice_mut(),
        &mut ELT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"=",
        ESTATE.as_slice(),
        6,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"=", ELT, 0.0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Make sure that SPKEZ returns the same thing as SPKACS when an inertial frame is the requested output frame. Converged Newtonian plus stellar aberration. Transmission case.", ctx)?;

    spicelib::SPKEZ(
        301001,
        ET,
        b"J2000",
        b"XCN+S",
        399001,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKACS(
        301001,
        ET,
        b"J2000",
        b"XCN+S",
        399001,
        ESTATE.as_slice_mut(),
        &mut ELT,
        &mut DLT,
        ctx,
    )?;

    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"=",
        ESTATE.as_slice(),
        6,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"=", ELT, 0.0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Perform an independent test to see if \"apparent\" positions in non-inertial frames are properly computed. Frame center is not target or observer. CN+S correction. Reception case.", ctx)?;

    spicelib::SPKEZ(
        401001,
        ET,
        b"IAU_EARTH",
        b"CN+S",
        301001,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKACS(
        401001,
        ET,
        b"J2000",
        b"CN+S",
        301001,
        ISTATE.as_slice_mut(),
        &mut ELT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKSSB(301001, ET, b"J2000", STOBS.as_slice_mut(), ctx)?;
    spicelib::SPKLTC(
        399,
        ET,
        b"J2000",
        b"CN+S",
        STOBS.as_slice(),
        TSTATE.as_slice_mut(),
        &mut CLT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(
        b"J2000",
        b"IAU_EARTH",
        (ET - CLT),
        XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Scale the derivative block to account for the rate of change
    // of light time.
    //
    for J in 4..=6 {
        for I in 1..=3 {
            XFORM[[J, I]] = ((1.0 - DLT) * XFORM[[J, I]]);
        }
    }

    spicelib::MXVG(
        XFORM.as_slice(),
        ISTATE.as_slice(),
        6,
        6,
        ESTATE.as_slice_mut(),
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"~/",
        ESTATE.as_slice(),
        6,
        VTIGHT,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"~/", ELT, VTIGHT, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Perform an independent test to see if \"apparent\" positions in non-inertial frames are properly computed. Frame center is not target or observer. CN+S correction. Transmission case.", ctx)?;

    spicelib::SPKEZ(
        401001,
        ET,
        b"IAU_EARTH",
        b"XCN+S",
        301001,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKACS(
        401001,
        ET,
        b"J2000",
        b"XCN+S",
        301001,
        ISTATE.as_slice_mut(),
        &mut ELT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKSSB(301001, ET, b"J2000", STOBS.as_slice_mut(), ctx)?;
    spicelib::SPKLTC(
        399,
        ET,
        b"J2000",
        b"XCN+S",
        STOBS.as_slice(),
        TSTATE.as_slice_mut(),
        &mut CLT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(
        b"J2000",
        b"IAU_EARTH",
        (ET + CLT),
        XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Scale the derivative block to account for the rate of change
    // of light time.
    //
    for J in 4..=6 {
        for I in 1..=3 {
            XFORM[[J, I]] = ((1.0 + DLT) * XFORM[[J, I]]);
        }
    }

    spicelib::MXVG(
        XFORM.as_slice(),
        ISTATE.as_slice(),
        6,
        6,
        ESTATE.as_slice_mut(),
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"~/",
        ESTATE.as_slice(),
        6,
        VTIGHT,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"~/", ELT, VTIGHT, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Perform an independent test to see if \"apparent\" positions in non-inertial frames are properly computed. Frame center is target. CN+S correction. Reception case.", ctx)?;

    spicelib::SPKEZ(
        399,
        ET,
        b"IAU_EARTH",
        b"CN+S",
        301001,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKACS(
        399,
        ET,
        b"J2000",
        b"CN+S",
        301001,
        ISTATE.as_slice_mut(),
        &mut ELT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(
        b"J2000",
        b"IAU_EARTH",
        (ET - ELT),
        XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Scale the derivative block to account for the rate of change
    // of light time.
    //
    for J in 4..=6 {
        for I in 1..=3 {
            XFORM[[J, I]] = ((1.0 - DLT) * XFORM[[J, I]]);
        }
    }

    spicelib::MXVG(
        XFORM.as_slice(),
        ISTATE.as_slice(),
        6,
        6,
        ESTATE.as_slice_mut(),
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"~/",
        ESTATE.as_slice(),
        6,
        VTIGHT,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"~/", ELT, VTIGHT, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Perform an independent test to see if \"apparent\" positions in non-inertial frames are properly computed. Frame center is target. CN+S correction. Transmission case.", ctx)?;

    spicelib::SPKEZ(
        399,
        ET,
        b"IAU_EARTH",
        b"XCN+S",
        301001,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKACS(
        399,
        ET,
        b"J2000",
        b"XCN+S",
        301001,
        ISTATE.as_slice_mut(),
        &mut ELT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(
        b"J2000",
        b"IAU_EARTH",
        (ET + ELT),
        XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Scale the derivative block to account for the rate of change
    // of light time.
    //
    for J in 4..=6 {
        for I in 1..=3 {
            XFORM[[J, I]] = ((1.0 + DLT) * XFORM[[J, I]]);
        }
    }

    spicelib::MXVG(
        XFORM.as_slice(),
        ISTATE.as_slice(),
        6,
        6,
        ESTATE.as_slice_mut(),
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"~/",
        ESTATE.as_slice(),
        6,
        VTIGHT,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"~/", ELT, VTIGHT, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Perform an independent test to see if \"apparent\" positions in non-inertial frames are properly computed. Frame center is observer. CN+S correction. Reception case.", ctx)?;

    spicelib::SPKEZ(
        301001,
        ET,
        b"IAU_EARTH",
        b"CN+S",
        399,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKACS(
        301001,
        ET,
        b"J2000",
        b"CN+S",
        399,
        ISTATE.as_slice_mut(),
        &mut ELT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(b"J2000", b"IAU_EARTH", ET, XFORM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::MXVG(
        XFORM.as_slice(),
        ISTATE.as_slice(),
        6,
        6,
        ESTATE.as_slice_mut(),
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"~/",
        ESTATE.as_slice(),
        6,
        VTIGHT,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"~/", ELT, VTIGHT, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Perform an independent test to see if \"apparent\" positions in non-inertial frames are properly computed. Frame center is observer. CN+S correction. Transmission case.", ctx)?;

    spicelib::SPKEZ(
        301001,
        ET,
        b"IAU_EARTH",
        b"XCN+S",
        399,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKACS(
        301001,
        ET,
        b"J2000",
        b"XCN+S",
        399,
        ISTATE.as_slice_mut(),
        &mut ELT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(b"J2000", b"IAU_EARTH", ET, XFORM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::MXVG(
        XFORM.as_slice(),
        ISTATE.as_slice(),
        6,
        6,
        ESTATE.as_slice_mut(),
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"~/",
        ESTATE.as_slice(),
        6,
        VTIGHT,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"~/", ELT, VTIGHT, OK, ctx)?;

    spicelib::SPKUEF(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::KILFIL(b"spkfun.bsp", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
