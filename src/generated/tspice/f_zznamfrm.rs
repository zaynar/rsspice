//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CTRSIZ: i32 = 2;
const VTOL: f64 = 0.00000000000001;
const TTOL: f64 = 0.000000000001;
const FRNMLN: i32 = 32;
const LINLEN: i32 = 80;
const BUFLEN: i32 = 10;
const NCASES: i32 = 4;
const NPTS: i32 = 8;

//$Procedure F_ZZNAMFRM ( Family of tests for ZZNAMFRM and its callers )
pub fn F_ZZNAMFRM(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut USRCTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
    let mut SAVCTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
    let mut FRNAME = [b' '; FRNMLN as usize];
    let mut FRCODE: i32 = 0;
    let mut FOUND: bool = false;
    let mut SAVNAM = [b' '; FRNMLN as usize];
    let mut SAVCDE: i32 = 0;
    let mut BUFFER = ActualCharArray::new(LINLEN, 1..=BUFLEN);
    let mut HANDLE: i32 = 0;
    let mut HANDL1: i32 = 0;
    let mut ET: f64 = 0.0;
    let mut FRAME1 = [b' '; FRNMLN as usize];
    let mut FRAME2 = [b' '; FRNMLN as usize];
    let mut BODY2 = [b' '; FRNMLN as usize];
    let mut CI: i32 = 0;
    let mut TITLE = [b' '; LINLEN as usize];
    let mut N = StackArray::<i32, 4>::new(1..=NCASES);
    let mut RADII = StackArray2D::<f64, 12>::new(1..=3, 1..=NCASES);
    let mut SCLKDP: f64 = 0.0;
    let mut CMAT1 = StackArray3D::<f64, 36>::new(1..=3, 1..=3, 1..=NCASES);
    let mut CLKOU1 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut FOUND1 = StackArray::<bool, 4>::new(1..=NCASES);
    let mut CMAT2 = StackArray3D::<f64, 36>::new(1..=3, 1..=3, 1..=NCASES);
    let mut AV2 = StackArray2D::<f64, 12>::new(1..=3, 1..=NCASES);
    let mut CLKOU2 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut FOUND2 = StackArray::<bool, 4>::new(1..=NCASES);
    let mut TRGEP1 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut OBSPO1 = StackArray2D::<f64, 12>::new(1..=3, 1..=NCASES);
    let mut TRMPT1 = StackArray3D::<f64, 96>::new(1..=3, 1..=NPTS, 1..=NCASES);
    let mut TRGEP2 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut SRFVE2 = StackArray2D::<f64, 12>::new(1..=3, 1..=NCASES);
    let mut PHASE2 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut INCDN2 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut EMISS2 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut MAT1 = StackArray3D::<f64, 36>::new(1..=3, 1..=3, 1..=NCASES);
    let mut MAT2 = StackArray3D::<f64, 36>::new(1..=3, 1..=3, 1..=NCASES);
    let mut SPOIN3 = StackArray2D::<f64, 12>::new(1..=3, 1..=NCASES);
    let mut TRGEP3 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut SRFVE3 = StackArray2D::<f64, 12>::new(1..=3, 1..=NCASES);
    let mut FOUND3 = StackArray::<bool, 4>::new(1..=NCASES);
    let mut STATE1 = StackArray2D::<f64, 24>::new(1..=6, 1..=NCASES);
    let mut LT1 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut STATE2 = StackArray2D::<f64, 24>::new(1..=6, 1..=NCASES);
    let mut LT2 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut STATE3 = StackArray2D::<f64, 24>::new(1..=6, 1..=NCASES);
    let mut LT3 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut POS4 = StackArray2D::<f64, 12>::new(1..=3, 1..=NCASES);
    let mut LT4 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut STATE5 = StackArray2D::<f64, 24>::new(1..=6, 1..=NCASES);
    let mut LT5 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut POS6 = StackArray2D::<f64, 12>::new(1..=3, 1..=NCASES);
    let mut LT6 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut DESCR = StackArray::<f64, 5>::new(1..=5);
    let mut STATE7 = StackArray2D::<f64, 24>::new(1..=6, 1..=NCASES);
    let mut LT7 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut POS8 = StackArray2D::<f64, 12>::new(1..=3, 1..=NCASES);
    let mut LT8 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut STATE9 = StackArray2D::<f64, 24>::new(1..=6, 1..=NCASES);
    let mut CENTE9 = StackArray::<i32, 4>::new(1..=NCASES);
    let mut SPOIN4 = StackArray2D::<f64, 12>::new(1..=3, 1..=NCASES);
    let mut DIST4 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut TRGEP4 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut OBSPO4 = StackArray2D::<f64, 12>::new(1..=3, 1..=NCASES);
    let mut FOUND4 = StackArray::<bool, 4>::new(1..=NCASES);
    let mut SPOIN5 = StackArray2D::<f64, 12>::new(1..=3, 1..=NCASES);
    let mut TRGEP5 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut SRFVE5 = StackArray2D::<f64, 12>::new(1..=3, 1..=NCASES);
    let mut SPOIN7 = StackArray2D::<f64, 12>::new(1..=3, 1..=NCASES);
    let mut TRGEP7 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut SRFVE7 = StackArray2D::<f64, 12>::new(1..=3, 1..=NCASES);
    let mut MAT3 = StackArray3D::<f64, 144>::new(1..=6, 1..=6, 1..=NCASES);
    let mut SPOINT = StackArray::<f64, 3>::new(1..=3);
    let mut DVEC = StackArray::<f64, 3>::new(1..=3);
    let mut LT: f64 = 0.0;
    let mut OBSSTA = StackArray::<f64, 6>::new(1..=6);

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
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZNAMFRM", ctx)?;

    //
    // Check ZZNAMFRM update after counter set to user value.
    //
    // Call ZZNAMFRM twice, first to make sure that there is an
    // update, second that there is no update.
    //
    testutil::TCASE(b"ZZNAMFRM update after initial counter setting.", ctx)?;

    spicelib::ZZCTRUIN(USRCTR.as_slice_mut(), ctx);

    spicelib::MOVEI(USRCTR.as_slice(), CTRSIZ, SAVCTR.as_slice_mut());

    fstr::assign(&mut FRNAME, b"J2000");
    FRCODE = -1;
    fstr::assign(&mut SAVNAM, b"ECLIPJ2000");
    SAVCDE = 17;

    spicelib::ZZNAMFRM(
        USRCTR.as_slice_mut(),
        &mut SAVNAM,
        &mut SAVCDE,
        &FRNAME,
        &mut FRCODE,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"USRCTR", USRCTR[1], b"!=", SAVCTR[1], 0, OK, ctx)?;
    testutil::CHCKSC(b"SAVNAM", &SAVNAM, b"=", b"J2000", OK, ctx)?;
    testutil::CHCKSI(b"SAVCDE", SAVCDE, b"=", 1, 0, OK, ctx)?;
    testutil::CHCKSC(b"FRNAME", &FRNAME, b"=", b"J2000", OK, ctx)?;
    testutil::CHCKSI(b"FRCODE", FRCODE, b"=", 1, 0, OK, ctx)?;

    spicelib::MOVEI(USRCTR.as_slice(), CTRSIZ, SAVCTR.as_slice_mut());

    spicelib::ZZNAMFRM(
        USRCTR.as_slice_mut(),
        &mut SAVNAM,
        &mut SAVCDE,
        &FRNAME,
        &mut FRCODE,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAI(
        b"USRCTR",
        USRCTR.as_slice(),
        b"=",
        SAVCTR.as_slice(),
        CTRSIZ,
        OK,
        ctx,
    )?;
    testutil::CHCKSC(b"SAVNAM", &SAVNAM, b"=", b"J2000", OK, ctx)?;
    testutil::CHCKSI(b"SAVCDE", SAVCDE, b"=", 1, 0, OK, ctx)?;
    testutil::CHCKSC(b"FRNAME", &FRNAME, b"=", b"J2000", OK, ctx)?;
    testutil::CHCKSI(b"FRCODE", FRCODE, b"=", 1, 0, OK, ctx)?;

    //
    // Check ZZNAMFRM update after POOL frame addition.
    //
    // Call ZZNAMFRM twice, first to make sure that there is an
    // update, second that there is no update.
    //
    testutil::TCASE(b"ZZNAMFRM update after POOL frame insertion.", ctx)?;

    fstr::assign(BUFFER.get_mut(1), b"FRAME_MYFRAME        = -99");
    fstr::assign(BUFFER.get_mut(2), b"FRAME_-99_NAME       = \'MYFRAME\'");
    fstr::assign(BUFFER.get_mut(3), b"FRAME_-99_CLASS      = 4");
    fstr::assign(BUFFER.get_mut(4), b"FRAME_-99_CLASS_ID   = -99");
    fstr::assign(BUFFER.get_mut(5), b"FRAME_-99_CENTER     = 0");

    spicelib::LMPOOL(BUFFER.as_arg(), 5, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::MOVEI(USRCTR.as_slice(), CTRSIZ, SAVCTR.as_slice_mut());

    fstr::assign(&mut FRNAME, b"MYFRAME");
    FRCODE = -1;
    fstr::assign(&mut SAVNAM, b"ECLIPJ2000");
    SAVCDE = 17;

    spicelib::ZZNAMFRM(
        USRCTR.as_slice_mut(),
        &mut SAVNAM,
        &mut SAVCDE,
        &FRNAME,
        &mut FRCODE,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"USRCTR", USRCTR[1], b"!=", SAVCTR[1], 0, OK, ctx)?;
    testutil::CHCKSC(b"SAVNAM", &SAVNAM, b"=", b"MYFRAME", OK, ctx)?;
    testutil::CHCKSI(b"SAVCDE", SAVCDE, b"=", -99, 0, OK, ctx)?;
    testutil::CHCKSC(b"FRNAME", &FRNAME, b"=", b"MYFRAME", OK, ctx)?;
    testutil::CHCKSI(b"FRCODE", FRCODE, b"=", -99, 0, OK, ctx)?;

    spicelib::MOVEI(USRCTR.as_slice(), CTRSIZ, SAVCTR.as_slice_mut());

    spicelib::ZZNAMFRM(
        USRCTR.as_slice_mut(),
        &mut SAVNAM,
        &mut SAVCDE,
        &FRNAME,
        &mut FRCODE,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAI(
        b"USRCTR",
        USRCTR.as_slice(),
        b"=",
        SAVCTR.as_slice(),
        CTRSIZ,
        OK,
        ctx,
    )?;
    testutil::CHCKSC(b"SAVNAM", &SAVNAM, b"=", b"MYFRAME", OK, ctx)?;
    testutil::CHCKSI(b"SAVCDE", SAVCDE, b"=", -99, 0, OK, ctx)?;
    testutil::CHCKSC(b"FRNAME", &FRNAME, b"=", b"MYFRAME", OK, ctx)?;
    testutil::CHCKSI(b"FRCODE", FRCODE, b"=", -99, 0, OK, ctx)?;

    //
    // Check ZZNAMFRM update after POOL frame update.
    //
    // Call ZZNAMFRM twice, first to make sure that there is an
    // update, second that there is no update.
    //
    testutil::TCASE(b"ZZNAMFRM update after POOL frame update.", ctx)?;

    fstr::assign(BUFFER.get_mut(1), b"FRAME_MYFRAME        = -88");
    fstr::assign(BUFFER.get_mut(2), b"FRAME_-88_NAME       = \'MYFRAME\'");
    fstr::assign(BUFFER.get_mut(3), b"FRAME_-88_CLASS      = 4");
    fstr::assign(BUFFER.get_mut(4), b"FRAME_-88_CLASS_ID   = -88");
    fstr::assign(BUFFER.get_mut(5), b"FRAME_-88_CENTER     = 0");

    spicelib::LMPOOL(BUFFER.as_arg(), 5, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::MOVEI(USRCTR.as_slice(), CTRSIZ, SAVCTR.as_slice_mut());

    fstr::assign(&mut FRNAME, b"MYFRAME");
    FRCODE = -1;
    fstr::assign(&mut SAVNAM, b"ECLIPJ2000");
    SAVCDE = 17;

    spicelib::ZZNAMFRM(
        USRCTR.as_slice_mut(),
        &mut SAVNAM,
        &mut SAVCDE,
        &FRNAME,
        &mut FRCODE,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"USRCTR", USRCTR[1], b"!=", SAVCTR[1], 0, OK, ctx)?;
    testutil::CHCKSC(b"SAVNAM", &SAVNAM, b"=", b"MYFRAME", OK, ctx)?;
    testutil::CHCKSI(b"SAVCDE", SAVCDE, b"=", -88, 0, OK, ctx)?;
    testutil::CHCKSC(b"FRNAME", &FRNAME, b"=", b"MYFRAME", OK, ctx)?;
    testutil::CHCKSI(b"FRCODE", FRCODE, b"=", -88, 0, OK, ctx)?;

    spicelib::MOVEI(USRCTR.as_slice(), CTRSIZ, SAVCTR.as_slice_mut());

    spicelib::ZZNAMFRM(
        USRCTR.as_slice_mut(),
        &mut SAVNAM,
        &mut SAVCDE,
        &FRNAME,
        &mut FRCODE,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAI(
        b"USRCTR",
        USRCTR.as_slice(),
        b"=",
        SAVCTR.as_slice(),
        CTRSIZ,
        OK,
        ctx,
    )?;
    testutil::CHCKSC(b"SAVNAM", &SAVNAM, b"=", b"MYFRAME", OK, ctx)?;
    testutil::CHCKSI(b"SAVCDE", SAVCDE, b"=", -88, 0, OK, ctx)?;
    testutil::CHCKSC(b"FRNAME", &FRNAME, b"=", b"MYFRAME", OK, ctx)?;
    testutil::CHCKSI(b"FRCODE", FRCODE, b"=", -88, 0, OK, ctx)?;

    //
    // Check ZZNAMFRM update for input saved code set to 0.
    //
    // Call ZZNAMFRM twice, first to make sure that there is an
    // update, second that there is no update.
    //
    testutil::TCASE(b"ZZNAMFRM update after 0 saved frame ID.", ctx)?;

    spicelib::MOVEI(USRCTR.as_slice(), CTRSIZ, SAVCTR.as_slice_mut());

    fstr::assign(&mut FRNAME, b"MYFRAME");
    FRCODE = -1;
    fstr::assign(&mut SAVNAM, b"ECLIPJ2000");
    SAVCDE = 0;

    spicelib::ZZNAMFRM(
        USRCTR.as_slice_mut(),
        &mut SAVNAM,
        &mut SAVCDE,
        &FRNAME,
        &mut FRCODE,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAI(
        b"USRCTR",
        USRCTR.as_slice(),
        b"=",
        SAVCTR.as_slice(),
        CTRSIZ,
        OK,
        ctx,
    )?;
    testutil::CHCKSC(b"SAVNAM", &SAVNAM, b"=", b"MYFRAME", OK, ctx)?;
    testutil::CHCKSI(b"SAVCDE", SAVCDE, b"=", -88, 0, OK, ctx)?;
    testutil::CHCKSC(b"FRNAME", &FRNAME, b"=", b"MYFRAME", OK, ctx)?;
    testutil::CHCKSI(b"FRCODE", FRCODE, b"=", -88, 0, OK, ctx)?;

    spicelib::MOVEI(USRCTR.as_slice(), CTRSIZ, SAVCTR.as_slice_mut());

    spicelib::ZZNAMFRM(
        USRCTR.as_slice_mut(),
        &mut SAVNAM,
        &mut SAVCDE,
        &FRNAME,
        &mut FRCODE,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAI(
        b"USRCTR",
        USRCTR.as_slice(),
        b"=",
        SAVCTR.as_slice(),
        CTRSIZ,
        OK,
        ctx,
    )?;
    testutil::CHCKSC(b"SAVNAM", &SAVNAM, b"=", b"MYFRAME", OK, ctx)?;
    testutil::CHCKSI(b"SAVCDE", SAVCDE, b"=", -88, 0, OK, ctx)?;
    testutil::CHCKSC(b"FRNAME", &FRNAME, b"=", b"MYFRAME", OK, ctx)?;
    testutil::CHCKSI(b"FRCODE", FRCODE, b"=", -88, 0, OK, ctx)?;

    //
    // Check ZZNAMFRM update for different input name.
    //
    // Call ZZNAMFRM twice, first to make sure that there is an
    // update, second that there is no update.
    //
    testutil::TCASE(b"ZZNAMFRM update for different input name.", ctx)?;

    spicelib::MOVEI(USRCTR.as_slice(), CTRSIZ, SAVCTR.as_slice_mut());

    fstr::assign(&mut FRNAME, b"MyFrame");
    FRCODE = -1;
    fstr::assign(&mut SAVNAM, b"ECLIPJ2000");
    SAVCDE = 17;

    spicelib::ZZNAMFRM(
        USRCTR.as_slice_mut(),
        &mut SAVNAM,
        &mut SAVCDE,
        &FRNAME,
        &mut FRCODE,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAI(
        b"USRCTR",
        USRCTR.as_slice(),
        b"=",
        SAVCTR.as_slice(),
        CTRSIZ,
        OK,
        ctx,
    )?;
    testutil::CHCKSC(b"SAVNAM", &SAVNAM, b"=", b"MyFrame", OK, ctx)?;
    testutil::CHCKSI(b"SAVCDE", SAVCDE, b"=", -88, 0, OK, ctx)?;
    testutil::CHCKSC(b"FRNAME", &FRNAME, b"=", b"MyFrame", OK, ctx)?;
    testutil::CHCKSI(b"FRCODE", FRCODE, b"=", -88, 0, OK, ctx)?;

    spicelib::MOVEI(USRCTR.as_slice(), CTRSIZ, SAVCTR.as_slice_mut());

    spicelib::ZZNAMFRM(
        USRCTR.as_slice_mut(),
        &mut SAVNAM,
        &mut SAVCDE,
        &FRNAME,
        &mut FRCODE,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAI(
        b"USRCTR",
        USRCTR.as_slice(),
        b"=",
        SAVCTR.as_slice(),
        CTRSIZ,
        OK,
        ctx,
    )?;
    testutil::CHCKSC(b"SAVNAM", &SAVNAM, b"=", b"MyFrame", OK, ctx)?;
    testutil::CHCKSI(b"SAVCDE", SAVCDE, b"=", -88, 0, OK, ctx)?;
    testutil::CHCKSC(b"FRNAME", &FRNAME, b"=", b"MyFrame", OK, ctx)?;
    testutil::CHCKSI(b"FRCODE", FRCODE, b"=", -88, 0, OK, ctx)?;

    //
    // Clear POOL and make and load the some test data.
    //
    spicelib::CLPOOL(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::KILFIL(b"fun.bsp", ctx)?;
    testutil::KILFIL(b"fun.ker", ctx)?;
    testutil::KILFIL(b"fun.bc", ctx)?;
    testutil::KILFIL(b"fun.tsc", ctx)?;

    testutil::TSTSPK(b"fun.bsp", true, &mut HANDLE, ctx)?;
    testutil::TSTPCK(b"fun.ker", true, true, ctx)?;
    testutil::TSTCK3(b"fun.bc", b"fun.tsc", true, true, true, &mut HANDL1, ctx)?;

    //
    // Test all routines that call ZZNAMFRM. Do it by computing two
    // bench mark sets of data (at indexes 1 and 2), each for a unique,
    // non-overlapping set of frames, then setting two unique sets of
    // alias frames and computing the same sets of data for them (at
    // indexes 3 and 4), and checking each "alias" set against its
    // benchmark. If saved mapping was updated correctly set 1 should
    // match set 3 and set 2 should match set 4.
    //
    // In addition to setting up all inputs we will call T_CTRBEQF to
    // adjust ZZBODS2C and ZZNAMFRM counters to the same value before
    // calling each of the routine that make use of saved values and
    // counters. This will help exposing any cases when the same counter
    // is erroneously used in more than one place.
    //
    for I in 1..=4 {
        //
        // Clear POOL and reload text kernels.
        //
        spicelib::CLPOOL(ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::LDPOOL(b"fun.ker", ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::LDPOOL(b"fun.tsc", ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Set up frames for each iteration.
        //
        if (I == 1) {
            fstr::assign(&mut FRAME1, b"J2000");
            fstr::assign(&mut FRAME2, b"IAU_EARTH");
            fstr::assign(&mut BODY2, b"EARTH");

            spicelib::IRFDEF(3, ctx)?;

            CI = 1;

            fstr::assign(&mut TITLE, b"Check 1st bench data set against itself");
        } else if (I == 2) {
            fstr::assign(&mut FRAME1, b"ECLIPJ2000");
            fstr::assign(&mut FRAME2, b"IAU_MARS");
            fstr::assign(&mut BODY2, b"MARS");

            spicelib::IRFDEF(13, ctx)?;

            CI = 2;

            fstr::assign(&mut TITLE, b"Check 2nd bench data set against itself");
        } else if (I == 3) {
            fstr::assign(&mut FRAME1, b"FRAME1");
            fstr::assign(&mut FRAME2, b"FRAME2");
            fstr::assign(&mut BODY2, b"EARTH");

            spicelib::IRFDEF(3, ctx)?;

            CI = 1;

            fstr::assign(BUFFER.get_mut(1), b"FRAME_FRAME1         = -99");
            fstr::assign(BUFFER.get_mut(2), b"FRAME_-99_NAME       = \'FRAME1\'");
            fstr::assign(BUFFER.get_mut(3), b"FRAME_-99_CLASS      = 4");
            fstr::assign(BUFFER.get_mut(4), b"FRAME_-99_CLASS_ID   = -99");
            fstr::assign(BUFFER.get_mut(5), b"FRAME_-99_CENTER     = 0");
            fstr::assign(BUFFER.get_mut(6), b"TKFRAME_-99_SPEC     = \'MATRIX\'");
            fstr::assign(BUFFER.get_mut(7), b"TKFRAME_-99_RELATIVE = \'J2000\'");
            fstr::assign(
                BUFFER.get_mut(8),
                b"TKFRAME_-99_MATRIX = ( 1,0,0,0,1,0,0,0,1 )",
            );
            fstr::assign(BUFFER.get_mut(9), b" ");
            fstr::assign(BUFFER.get_mut(10), b" ");

            spicelib::LMPOOL(BUFFER.as_arg(), 10, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(BUFFER.get_mut(1), b"FRAME_FRAME2         = -88");
            fstr::assign(BUFFER.get_mut(2), b"FRAME_-88_NAME       = \'FRAME2\'");
            fstr::assign(BUFFER.get_mut(3), b"FRAME_-88_CLASS      = 4");
            fstr::assign(BUFFER.get_mut(4), b"FRAME_-88_CLASS_ID   = -88");
            fstr::assign(BUFFER.get_mut(5), b"FRAME_-88_CENTER     = 399");
            fstr::assign(BUFFER.get_mut(6), b"TKFRAME_-88_SPEC     = \'MATRIX\'");
            fstr::assign(BUFFER.get_mut(7), b"TKFRAME_-88_RELATIVE = \'IAU_EARTH\'");
            fstr::assign(
                BUFFER.get_mut(8),
                b"TKFRAME_-88_MATRIX = ( 1,0,0,0,1,0,0,0,1 )",
            );
            fstr::assign(BUFFER.get_mut(9), b" ");
            fstr::assign(BUFFER.get_mut(10), b" ");

            spicelib::LMPOOL(BUFFER.as_arg(), 10, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(
                &mut TITLE,
                b"Check 1st mapped data set against 1st benchmark",
            );
        } else if (I == 4) {
            fstr::assign(&mut FRAME1, b"FRAME1");
            fstr::assign(&mut FRAME2, b"FRAME2");
            fstr::assign(&mut BODY2, b"MARS");

            spicelib::IRFDEF(13, ctx)?;

            CI = 2;

            fstr::assign(BUFFER.get_mut(1), b"FRAME_FRAME1         = -77");
            fstr::assign(BUFFER.get_mut(2), b"FRAME_-77_NAME       = \'FRAME1\'");
            fstr::assign(BUFFER.get_mut(3), b"FRAME_-77_CLASS      = 4");
            fstr::assign(BUFFER.get_mut(4), b"FRAME_-77_CLASS_ID   = -77");
            fstr::assign(BUFFER.get_mut(5), b"FRAME_-77_CENTER     = 0");
            fstr::assign(BUFFER.get_mut(6), b"TKFRAME_-77_SPEC     = \'MATRIX\'");
            fstr::assign(BUFFER.get_mut(7), b"TKFRAME_-77_RELATIVE = \'ECLIPJ2000\'");
            fstr::assign(
                BUFFER.get_mut(8),
                b"TKFRAME_-77_MATRIX = ( 1,0,0,0,1,0,0,0,1 )",
            );
            fstr::assign(BUFFER.get_mut(9), b" ");
            fstr::assign(BUFFER.get_mut(10), b" ");

            spicelib::LMPOOL(BUFFER.as_arg(), 10, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(BUFFER.get_mut(1), b"FRAME_FRAME2         = -66");
            fstr::assign(BUFFER.get_mut(2), b"FRAME_-66_NAME       = \'FRAME2\'");
            fstr::assign(BUFFER.get_mut(3), b"FRAME_-66_CLASS      = 4");
            fstr::assign(BUFFER.get_mut(4), b"FRAME_-66_CLASS_ID   = -66");
            fstr::assign(BUFFER.get_mut(5), b"FRAME_-66_CENTER     = 499");
            fstr::assign(BUFFER.get_mut(6), b"TKFRAME_-66_SPEC     = \'MATRIX\'");
            fstr::assign(BUFFER.get_mut(7), b"TKFRAME_-66_RELATIVE = \'IAU_MARS\'");
            fstr::assign(
                BUFFER.get_mut(8),
                b"TKFRAME_-66_MATRIX = ( 1,0,0,0,1,0,0,0,1 )",
            );
            fstr::assign(BUFFER.get_mut(9), b" ");
            fstr::assign(BUFFER.get_mut(10), b" ");

            spicelib::LMPOOL(BUFFER.as_arg(), 10, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(
                &mut TITLE,
                b"Check 2nd mapped data set against 2nd benchmark",
            );
        }

        //
        // Declare test case.
        //
        testutil::TCASE(&TITLE, ctx)?;

        //
        // Call all routines that make use of ZZNAMFRM. Set arbitrary
        // input time.
        //
        ET = 10000000.0;

        //
        // -----------------------------------------------------------
        //
        spicelib::SCE2T(-9, ET, &mut SCLKDP, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        T_CTRBEQF(ctx)?;

        spicelib::CKGP(
            -9999,
            SCLKDP,
            0.0,
            &FRAME1,
            CMAT1.subarray_mut([1, 1, I]),
            &mut CLKOU1[I],
            &mut FOUND1[I],
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKAD(
            b"CKGP CMAT",
            CMAT1.subarray([1, 1, I]),
            b"=",
            CMAT1.subarray([1, 1, CI]),
            9,
            0.0,
            OK,
            ctx,
        )?;
        testutil::CHCKSD(b"CKGP CKLOUT", CLKOU1[I], b"=", CLKOU1[CI], 0.0, OK, ctx)?;
        testutil::CHCKSL(b"CKGP FOUND", FOUND1[I], FOUND1[CI], OK, ctx)?;

        //
        // -----------------------------------------------------------
        //
        spicelib::SCE2T(-9, ET, &mut SCLKDP, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        T_CTRBEQF(ctx)?;

        spicelib::CKGPAV(
            -9999,
            SCLKDP,
            0.0,
            &FRAME1,
            CMAT2.subarray_mut([1, 1, I]),
            AV2.subarray_mut([1, I]),
            &mut CLKOU2[I],
            &mut FOUND2[I],
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKAD(
            b"CKGPAV CMAT",
            CMAT2.subarray([1, 1, I]),
            b"=",
            CMAT2.subarray([1, 1, CI]),
            9,
            0.0,
            OK,
            ctx,
        )?;
        testutil::CHCKAD(
            b"CKGPAV AV",
            AV2.subarray([1, I]),
            b"=",
            AV2.subarray([1, CI]),
            3,
            0.0,
            OK,
            ctx,
        )?;
        testutil::CHCKSD(b"CKGP CKLOUT", CLKOU2[I], b"=", CLKOU2[CI], 0.0, OK, ctx)?;
        testutil::CHCKSL(b"CKGPAV FOUND", FOUND2[I], FOUND1[CI], OK, ctx)?;

        //
        // -----------------------------------------------------------
        //
        T_CTRBEQF(ctx)?;

        spicelib::EDTERM(
            b"UMBRAL",
            b"SUN",
            &BODY2,
            ET,
            &FRAME2,
            b"NONE",
            b"MERCURY",
            NPTS,
            &mut TRGEP1[I],
            OBSPO1.subarray_mut([1, I]),
            TRMPT1.subarray_mut([1, 1, I]),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSD(b"EDTERM TRGEPC", TRGEP1[I], b"=", TRGEP1[CI], 0.0, OK, ctx)?;
        testutil::CHCKAD(
            b"EDTERM OBSPOS",
            OBSPO1.subarray([1, I]),
            b"=",
            OBSPO1.subarray([1, CI]),
            3,
            0.0,
            OK,
            ctx,
        )?;
        testutil::CHCKAD(
            b"EDTERM TRMPTS",
            TRMPT1.subarray([1, 1, I]),
            b"=",
            TRMPT1.subarray([1, 1, CI]),
            (3 * NPTS),
            0.0,
            OK,
            ctx,
        )?;

        //
        // -----------------------------------------------------------
        //
        spicelib::BODVRD(
            &BODY2,
            b"RADII",
            3,
            &mut N[I],
            RADII.subarray_mut([1, I]),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::VPACK(RADII[[1, I]], 0.0, 0.0, SPOINT.as_slice_mut());

        T_CTRBEQF(ctx)?;

        spicelib::ILLUMG(
            b"Ellipsoid",
            &BODY2,
            b"SUN",
            ET,
            &FRAME2,
            b"NONE",
            b"MERCURY",
            SPOINT.as_slice(),
            &mut TRGEP2[I],
            SRFVE2.subarray_mut([1, I]),
            &mut PHASE2[I],
            &mut INCDN2[I],
            &mut EMISS2[I],
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSD(b"ILLUMG TRGEPC", TRGEP2[I], b"=", TRGEP2[CI], 0.0, OK, ctx)?;
        testutil::CHCKAD(
            b"ILLUMG SRFVEC",
            SRFVE2.subarray([1, I]),
            b"=",
            SRFVE2.subarray([1, CI]),
            3,
            0.0,
            OK,
            ctx,
        )?;
        testutil::CHCKSD(b"ILLUMG PHASE", PHASE2[I], b"=", PHASE2[CI], 0.0, OK, ctx)?;
        testutil::CHCKSD(b"ILLUMG INCDNC", INCDN2[I], b"=", INCDN2[CI], 0.0, OK, ctx)?;
        testutil::CHCKSD(b"ILLUMG EMISSN", EMISS2[I], b"=", EMISS2[CI], 0.0, OK, ctx)?;

        //
        // -----------------------------------------------------------
        //
        T_CTRBEQF(ctx)?;

        spicelib::PXFORM(&FRAME1, &FRAME2, ET, MAT1.subarray_mut([1, 1, I]), ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKAD(
            b"PXFORM ROTATE",
            MAT1.subarray([1, 1, I]),
            b"=",
            MAT1.subarray([1, 1, CI]),
            9,
            0.0,
            OK,
            ctx,
        )?;

        //
        // -----------------------------------------------------------
        //
        T_CTRBEQF(ctx)?;

        spicelib::PXFRM2(
            &FRAME1,
            &FRAME2,
            ET,
            (ET + 1000000.0),
            MAT2.subarray_mut([1, 1, I]),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKAD(
            b"PXFRM2 ROTATE",
            MAT2.subarray([1, 1, I]),
            b"=",
            MAT2.subarray([1, 1, CI]),
            9,
            0.0,
            OK,
            ctx,
        )?;

        //
        // -----------------------------------------------------------
        //
        spicelib::SPKPOS(
            &BODY2,
            ET,
            b"J2000",
            b"NONE",
            b"SUN",
            DVEC.as_slice_mut(),
            &mut LT,
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        T_CTRBEQF(ctx)?;

        spicelib::SINCPT(
            b"Ellipsoid",
            &BODY2,
            ET,
            &FRAME2,
            b"NONE",
            b"SUN",
            b"J2000",
            DVEC.as_slice(),
            SPOIN3.subarray_mut([1, I]),
            &mut TRGEP3[I],
            SRFVE3.subarray_mut([1, I]),
            &mut FOUND3[I],
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKAD(
            b"SINCPT SPOINT",
            SPOIN3.subarray([1, I]),
            b"=",
            SPOIN3.subarray([1, CI]),
            3,
            0.0,
            OK,
            ctx,
        )?;
        testutil::CHCKSD(b"SINCPT TRGEPC", TRGEP3[I], b"=", TRGEP3[CI], 0.0, OK, ctx)?;
        testutil::CHCKAD(
            b"SINCPT SRFVEC",
            SRFVE3.subarray([1, I]),
            b"=",
            SRFVE3.subarray([1, CI]),
            3,
            0.0,
            OK,
            ctx,
        )?;
        testutil::CHCKSL(b"SINCPT FOUND", FOUND3[I], FOUND3[CI], OK, ctx)?;

        //
        // -----------------------------------------------------------
        //
        spicelib::CLEARD(6, OBSSTA.as_slice_mut());
        OBSSTA[1] = 1000.0;

        T_CTRBEQF(ctx)?;

        spicelib::SPKCVO(
            b"SUN",
            ET,
            &FRAME1,
            b"OBSERVER",
            b"LT+S",
            OBSSTA.as_slice(),
            ET,
            &BODY2,
            &FRAME2,
            STATE1.subarray_mut([1, I]),
            &mut LT1[I],
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKAD(
            b"SPKCVO STATE",
            STATE1.subarray([1, I]),
            b"~/",
            STATE1.subarray([1, CI]),
            6,
            VTOL,
            OK,
            ctx,
        )?;
        testutil::CHCKSD(b"SPKCVO LT", LT1[I], b"~/", LT1[CI], TTOL, OK, ctx)?;

        //
        // -----------------------------------------------------------
        //
        T_CTRBEQF(ctx)?;

        spicelib::SPKCVT(
            OBSSTA.as_slice(),
            ET,
            &BODY2,
            &FRAME2,
            ET,
            &FRAME1,
            b"OBSERVER",
            b"LT+S",
            b"SUN",
            STATE2.subarray_mut([1, I]),
            &mut LT2[I],
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKAD(
            b"SPKCVT STATE",
            STATE2.subarray([1, I]),
            b"~/",
            STATE2.subarray([1, CI]),
            6,
            VTOL,
            OK,
            ctx,
        )?;
        testutil::CHCKSD(b"SPKCVT LT", LT2[I], b"~/", LT2[CI], TTOL, OK, ctx)?;

        //
        // -----------------------------------------------------------
        //
        T_CTRBEQF(ctx)?;

        spicelib::SPKEZ(
            1,
            ET,
            &FRAME2,
            b"NONE",
            399,
            STATE3.subarray_mut([1, I]),
            &mut LT3[I],
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKAD(
            b"SPKEZR STATE",
            STATE3.subarray([1, I]),
            b"=",
            STATE3.subarray([1, CI]),
            6,
            0.0,
            OK,
            ctx,
        )?;
        testutil::CHCKSD(b"SPKEZR LT", LT3[I], b"=", LT3[CI], 0.0, OK, ctx)?;

        //
        // -----------------------------------------------------------
        //
        T_CTRBEQF(ctx)?;

        spicelib::SPKEZP(
            1,
            ET,
            &FRAME2,
            b"NONE",
            399,
            POS4.subarray_mut([1, I]),
            &mut LT4[I],
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKAD(
            b"SPKPOS POS",
            POS4.subarray([1, I]),
            b"=",
            POS4.subarray([1, CI]),
            3,
            0.0,
            OK,
            ctx,
        )?;
        testutil::CHCKSD(b"SPKPOS LT", LT4[I], b"=", LT4[CI], 0.0, OK, ctx)?;

        //
        // -----------------------------------------------------------
        //
        T_CTRBEQF(ctx)?;

        spicelib::SPKGEO(
            499,
            ET,
            &FRAME1,
            10,
            STATE5.subarray_mut([1, I]),
            &mut LT5[I],
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKAD(
            b"SPKGEO STATE",
            STATE5.subarray([1, I]),
            b"~/",
            STATE5.subarray([1, CI]),
            6,
            VTOL,
            OK,
            ctx,
        )?;
        testutil::CHCKSD(b"SPKGEO LT", LT5[I], b"~/", LT5[CI], TTOL, OK, ctx)?;

        T_CTRBEQF(ctx)?;

        spicelib::SPKGEO(
            1,
            ET,
            b"DEFAULT",
            399,
            STATE7.subarray_mut([1, I]),
            &mut LT7[I],
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKAD(
            b"SPKGEO STATE",
            STATE7.subarray([1, I]),
            b"=",
            STATE7.subarray([1, CI]),
            6,
            0.0,
            OK,
            ctx,
        )?;
        testutil::CHCKSD(b"SPKGEO LT", LT7[I], b"=", LT7[CI], 0.0, OK, ctx)?;

        //
        // -----------------------------------------------------------
        //
        T_CTRBEQF(ctx)?;

        spicelib::SPKGPS(
            499,
            ET,
            &FRAME1,
            10,
            POS6.subarray_mut([1, I]),
            &mut LT6[I],
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKAD(
            b"SPKGPS POS",
            POS6.subarray([1, I]),
            b"=",
            POS6.subarray([1, CI]),
            3,
            0.0,
            OK,
            ctx,
        )?;
        testutil::CHCKSD(b"SPKGPS LT", LT6[I], b"=", LT6[CI], 0.0, OK, ctx)?;

        T_CTRBEQF(ctx)?;

        spicelib::SPKGPS(
            1,
            ET,
            b"DEFAULT",
            399,
            POS8.subarray_mut([1, I]),
            &mut LT8[I],
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKAD(
            b"SPKGPS POS",
            POS8.subarray([1, I]),
            b"=",
            POS8.subarray([1, CI]),
            3,
            0.0,
            OK,
            ctx,
        )?;
        testutil::CHCKSD(b"SPKGPS LT", LT8[I], b"=", LT8[CI], 0.0, OK, ctx)?;

        //
        // -----------------------------------------------------------
        //
        spicelib::DAFBFS(HANDLE, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DAFFNA(&mut FOUND, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(b"DAFFNA FOUND", FOUND, true, OK, ctx)?;

        spicelib::DAFGS(DESCR.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        T_CTRBEQF(ctx)?;

        spicelib::SPKPV(
            HANDLE,
            DESCR.as_slice(),
            ET,
            &FRAME2,
            STATE9.subarray_mut([1, I]),
            &mut CENTE9[I],
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKAD(
            b"SPKPV STATE",
            STATE9.subarray([1, I]),
            b"=",
            STATE9.subarray([1, CI]),
            6,
            0.0,
            OK,
            ctx,
        )?;
        testutil::CHCKSI(b"SPKPV CENTER", CENTE9[I], b"=", CENTE9[CI], 0, OK, ctx)?;

        //
        // -----------------------------------------------------------
        //
        spicelib::SPKPOS(
            &BODY2,
            ET,
            &FRAME1,
            b"NONE",
            b"SUN",
            DVEC.as_slice_mut(),
            &mut LT,
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        T_CTRBEQF(ctx)?;

        spicelib::SRFXPT(
            b"Ellipsoid",
            &BODY2,
            ET,
            b"NONE",
            b"SUN",
            &FRAME1,
            DVEC.as_slice(),
            SPOIN4.subarray_mut([1, I]),
            &mut DIST4[I],
            &mut TRGEP4[I],
            OBSPO4.subarray_mut([1, I]),
            &mut FOUND4[I],
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKAD(
            b"SRFXPT SPOINT",
            SPOIN4.subarray([1, I]),
            b"=",
            SPOIN4.subarray([1, CI]),
            3,
            0.0,
            OK,
            ctx,
        )?;
        testutil::CHCKSD(b"SRFXPT DIST", DIST4[I], b"=", DIST4[CI], 0.0, OK, ctx)?;
        testutil::CHCKSD(b"SRFXPT TRGEPC", TRGEP4[I], b"=", TRGEP4[CI], 0.0, OK, ctx)?;
        testutil::CHCKAD(
            b"SRFXPT OBSPOS",
            OBSPO4.subarray([1, I]),
            b"=",
            OBSPO4.subarray([1, CI]),
            3,
            0.0,
            OK,
            ctx,
        )?;
        testutil::CHCKSL(b"SRFXPT FOUND", FOUND4[I], FOUND4[CI], OK, ctx)?;

        //
        // -----------------------------------------------------------
        //
        T_CTRBEQF(ctx)?;

        spicelib::SUBPNT(
            b"Near point: ellipsoid",
            &BODY2,
            ET,
            &FRAME2,
            b"NONE",
            b"SUN",
            SPOIN5.subarray_mut([1, I]),
            &mut TRGEP5[I],
            SRFVE5.subarray_mut([1, I]),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKAD(
            b"SUBPNT SPOINT",
            SPOIN5.subarray([1, I]),
            b"=",
            SPOIN5.subarray([1, CI]),
            3,
            0.0,
            OK,
            ctx,
        )?;
        testutil::CHCKSD(b"SUBPNT TRGEPC", TRGEP5[I], b"=", TRGEP5[CI], 0.0, OK, ctx)?;
        testutil::CHCKAD(
            b"SUBPNT SRFVEC",
            SRFVE5.subarray([1, I]),
            b"=",
            SRFVE5.subarray([1, CI]),
            3,
            0.0,
            OK,
            ctx,
        )?;

        //
        // -----------------------------------------------------------
        //
        T_CTRBEQF(ctx)?;

        spicelib::SUBSLR(
            b"Near point: ellipsoid",
            &BODY2,
            ET,
            &FRAME2,
            b"NONE",
            b"MERCURY",
            SPOIN7.subarray_mut([1, I]),
            &mut TRGEP7[I],
            SRFVE7.subarray_mut([1, I]),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKAD(
            b"SUBSLR SPOINT",
            SPOIN7.subarray([1, I]),
            b"=",
            SPOIN7.subarray([1, CI]),
            3,
            0.0,
            OK,
            ctx,
        )?;
        testutil::CHCKSD(b"SUBSLR TRGEPC", TRGEP7[I], b"=", TRGEP7[CI], 0.0, OK, ctx)?;
        testutil::CHCKAD(
            b"SUBSLR SRFVEC",
            SRFVE7.subarray([1, I]),
            b"=",
            SRFVE7.subarray([1, CI]),
            3,
            0.0,
            OK,
            ctx,
        )?;

        //
        // -----------------------------------------------------------
        //
        T_CTRBEQF(ctx)?;

        spicelib::SXFORM(&FRAME1, &FRAME2, ET, MAT3.subarray_mut([1, 1, I]), ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKAD(
            b"SXFORM XFORM",
            MAT3.subarray([1, 1, I]),
            b"=",
            MAT3.subarray([1, 1, CI]),
            36,
            0.0,
            OK,
            ctx,
        )?;
    }

    //
    // Clean up.
    //
    testutil::KILFIL(b"fun.bsp", ctx)?;
    testutil::KILFIL(b"fun.ker", ctx)?;
    testutil::KILFIL(b"fun.bc", ctx)?;
    testutil::KILFIL(b"fun.tsc", ctx)?;

    //
    // This is good enough for now.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
