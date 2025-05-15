//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CTRSIZ: i32 = 2;
const MAXL: i32 = 36;
const LINLEN: i32 = 80;
const BUFLEN: i32 = 6;
const NCASES: i32 = 4;
const NPTS: i32 = 8;
const ROOM: i32 = 3;

struct SaveVars {
    BSIGHT1: StackArray<f64, 3>,
    BSIGHT2: StackArray<f64, 3>,
    BOUNDS1: StackArray<f64, 3>,
    BOUNDS2: StackArray<f64, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BSIGHT1 = StackArray::<f64, 3>::new(1..=3);
        let mut BSIGHT2 = StackArray::<f64, 3>::new(1..=3);
        let mut BOUNDS1 = StackArray::<f64, 3>::new(1..=3);
        let mut BOUNDS2 = StackArray::<f64, 3>::new(1..=3);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(1.0), Val::D(1.0), Val::D(1.0)].into_iter();
            BOUNDS1
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(-1.0), Val::D(-1.0), Val::D(-1.0)].into_iter();
            BOUNDS2
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            BSIGHT1
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(-1.0)].into_iter();
            BSIGHT2
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            BSIGHT1,
            BSIGHT2,
            BOUNDS1,
            BOUNDS2,
        }
    }
}

//$Procedure F_ZZBODS2C ( Family of tests for ZZBODS2C and its callers )
pub fn F_ZZBODS2C(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut USRCTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
    let mut SAVCTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
    let mut TARG = [b' '; MAXL as usize];
    let mut TARGID: i32 = 0;
    let mut FOUND: bool = false;
    let mut SVTARG = [b' '; MAXL as usize];
    let mut SVTGID: i32 = 0;
    let mut SVFND: bool = false;
    let mut BUFFER = ActualCharArray::new(LINLEN, 1..=BUFLEN);
    let mut HANDLE: i32 = 0;
    let mut ET: f64 = 0.0;
    let mut BODY1 = [b' '; MAXL as usize];
    let mut BODY2 = [b' '; MAXL as usize];
    let mut BODY3 = [b' '; MAXL as usize];
    let mut CI: i32 = 0;
    let mut TITLE = [b' '; LINLEN as usize];
    let mut N = StackArray::<i32, 4>::new(1..=NCASES);
    let mut RADII = StackArray2D::<f64, 12>::new(1..=3, 1..=NCASES);
    let mut F: f64 = 0.0;
    let mut JACOB1 = StackArray3D::<f64, 36>::new(1..=3, 1..=3, 1..=NCASES);
    let mut JACOB2 = StackArray3D::<f64, 36>::new(1..=3, 1..=3, 1..=NCASES);
    let mut B1CODE: i32 = 0;
    let mut FRCODE: i32 = 0;
    let mut FIXREF = [b' '; MAXL as usize];
    let mut TRGEP1 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut OBSPO1 = StackArray2D::<f64, 12>::new(1..=3, 1..=NCASES);
    let mut TRMPT1 = StackArray3D::<f64, 96>::new(1..=3, 1..=NPTS, 1..=NCASES);
    let mut SPOINT = StackArray::<f64, 3>::new(1..=3);
    let mut PHASE1 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut SOLAR1 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut EMISS1 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut TRGEP2 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut SRFVE2 = StackArray2D::<f64, 12>::new(1..=3, 1..=NCASES);
    let mut PHASE2 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut INCDN2 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut EMISS2 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut LS = StackArray::<f64, 4>::new(1..=NCASES);
    let mut RECTA1 = StackArray2D::<f64, 12>::new(1..=3, 1..=NCASES);
    let mut PHASE3 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut RECTAN = StackArray::<f64, 3>::new(1..=3);
    let mut LON = StackArray::<f64, 4>::new(1..=NCASES);
    let mut LAT = StackArray::<f64, 4>::new(1..=NCASES);
    let mut ALT = StackArray::<f64, 4>::new(1..=NCASES);
    let mut B3CODE: i32 = 0;
    let mut DVEC = StackArray::<f64, 3>::new(1..=3);
    let mut LT: f64 = 0.0;
    let mut SPOIN3 = StackArray2D::<f64, 12>::new(1..=3, 1..=NCASES);
    let mut TRGEP3 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut SRFVE3 = StackArray2D::<f64, 12>::new(1..=3, 1..=NCASES);
    let mut FOUND3 = StackArray::<bool, 4>::new(1..=NCASES);
    let mut OBSSTA = StackArray::<f64, 6>::new(1..=6);
    let mut STATE1 = StackArray2D::<f64, 24>::new(1..=6, 1..=NCASES);
    let mut LT1 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut STATE2 = StackArray2D::<f64, 24>::new(1..=6, 1..=NCASES);
    let mut LT2 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut STATE3 = StackArray2D::<f64, 24>::new(1..=6, 1..=NCASES);
    let mut LT3 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut POS4 = StackArray2D::<f64, 12>::new(1..=3, 1..=NCASES);
    let mut LT4 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut SPOIN4 = StackArray2D::<f64, 12>::new(1..=3, 1..=NCASES);
    let mut DIST4 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut TRGEP4 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut OBSPO4 = StackArray2D::<f64, 12>::new(1..=3, 1..=NCASES);
    let mut FOUND4 = StackArray::<bool, 4>::new(1..=NCASES);
    let mut SPOIN5 = StackArray2D::<f64, 12>::new(1..=3, 1..=NCASES);
    let mut TRGEP5 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut SRFVE5 = StackArray2D::<f64, 12>::new(1..=3, 1..=NCASES);
    let mut SPOIN6 = StackArray2D::<f64, 12>::new(1..=3, 1..=NCASES);
    let mut ALT6 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut SPOIN7 = StackArray2D::<f64, 12>::new(1..=3, 1..=NCASES);
    let mut TRGEP7 = StackArray::<f64, 4>::new(1..=NCASES);
    let mut SRFVE7 = StackArray2D::<f64, 12>::new(1..=3, 1..=NCASES);
    let mut SPOIN8 = StackArray2D::<f64, 12>::new(1..=3, 1..=NCASES);
    let mut ISTATE = StackArray::<f64, 6>::new(1..=6);
    let mut OSTATE = StackArray2D::<f64, 24>::new(1..=6, 1..=NCASES);
    let mut NBND = StackArray::<i32, 4>::new(1..=NCASES);
    let mut FOVSHP = ActualCharArray::new(MAXL, 1..=NCASES);
    let mut FOVFRM = ActualCharArray::new(MAXL, 1..=NCASES);
    let mut FOVBS = StackArray2D::<f64, 12>::new(1..=3, 1..=NCASES);
    let mut FOVBND = StackArray2D::<f64, 12>::new(1..=3, 1..=NCASES);

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
    testutil::TOPEN(b"F_ZZBODS2C", ctx)?;

    //
    // Check ZZBODS2C update after counter set to user value.
    //
    // Call ZZBODS2C twice, first to make sure that there is an
    // update, second that there is not update.
    //
    testutil::TCASE(b"ZZBODS2C update after initial counter setting.", ctx)?;

    spicelib::ZZCTRUIN(USRCTR.as_slice_mut(), ctx);

    spicelib::MOVEI(USRCTR.as_slice(), CTRSIZ, SAVCTR.as_slice_mut());

    fstr::assign(&mut TARG, b"SUN");
    TARGID = -1;
    FOUND = false;
    fstr::assign(&mut SVTARG, b"SUN");
    SVTGID = -1;
    SVFND = true;

    spicelib::ZZBODS2C(
        USRCTR.as_slice_mut(),
        &mut SVTARG,
        &mut SVTGID,
        &mut SVFND,
        &TARG,
        &mut TARGID,
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"USRCTR", USRCTR[1], b"!=", SAVCTR[1], 0, OK, ctx)?;
    testutil::CHCKSC(b"SVTARG", &SVTARG, b"=", b"SUN", OK, ctx)?;
    testutil::CHCKSI(b"SVTGID", SVTGID, b"=", 10, 0, OK, ctx)?;
    testutil::CHCKSL(b"SVFND", SVFND, true, OK, ctx)?;
    testutil::CHCKSC(b"TARG", &TARG, b"=", b"SUN", OK, ctx)?;
    testutil::CHCKSI(b"TARGID", TARGID, b"=", 10, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    spicelib::MOVEI(USRCTR.as_slice(), CTRSIZ, SAVCTR.as_slice_mut());

    spicelib::ZZBODS2C(
        USRCTR.as_slice_mut(),
        &mut SVTARG,
        &mut SVTGID,
        &mut SVFND,
        &TARG,
        &mut TARGID,
        &mut FOUND,
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
    testutil::CHCKSC(b"SVTARG", &SVTARG, b"=", b"SUN", OK, ctx)?;
    testutil::CHCKSI(b"SVTGID", SVTGID, b"=", 10, 0, OK, ctx)?;
    testutil::CHCKSL(b"SVFND", SVFND, true, OK, ctx)?;
    testutil::CHCKSC(b"TARG", &TARG, b"=", b"SUN", OK, ctx)?;
    testutil::CHCKSI(b"TARGID", TARGID, b"=", 10, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    //
    // Check ZZBODS2C update after BODDEF mapping update.
    //
    // Call ZZBODS2C twice, first to make sure that there is an
    // update, second that there is not update.
    //
    testutil::TCASE(b"ZZBODS2C update after BODDEF.", ctx)?;

    spicelib::BODDEF(b"SUN", 20, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::MOVEI(USRCTR.as_slice(), CTRSIZ, SAVCTR.as_slice_mut());

    fstr::assign(&mut TARG, b"SUN");
    TARGID = -1;
    FOUND = false;
    fstr::assign(&mut SVTARG, b"SUN");
    SVTGID = 10;
    SVFND = true;

    spicelib::ZZBODS2C(
        USRCTR.as_slice_mut(),
        &mut SVTARG,
        &mut SVTGID,
        &mut SVFND,
        &TARG,
        &mut TARGID,
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"USRCTR", USRCTR[1], b"!=", SAVCTR[1], 0, OK, ctx)?;
    testutil::CHCKSC(b"SVTARG", &SVTARG, b"=", b"SUN", OK, ctx)?;
    testutil::CHCKSI(b"SVTGID", SVTGID, b"=", 20, 0, OK, ctx)?;
    testutil::CHCKSL(b"SVFND", SVFND, true, OK, ctx)?;
    testutil::CHCKSC(b"TARG", &TARG, b"=", b"SUN", OK, ctx)?;
    testutil::CHCKSI(b"TARGID", TARGID, b"=", 20, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    spicelib::MOVEI(USRCTR.as_slice(), CTRSIZ, SAVCTR.as_slice_mut());

    spicelib::ZZBODS2C(
        USRCTR.as_slice_mut(),
        &mut SVTARG,
        &mut SVTGID,
        &mut SVFND,
        &TARG,
        &mut TARGID,
        &mut FOUND,
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
    testutil::CHCKSC(b"SVTARG", &SVTARG, b"=", b"SUN", OK, ctx)?;
    testutil::CHCKSI(b"SVTGID", SVTGID, b"=", 20, 0, OK, ctx)?;
    testutil::CHCKSL(b"SVFND", SVFND, true, OK, ctx)?;
    testutil::CHCKSC(b"TARG", &TARG, b"=", b"SUN", OK, ctx)?;
    testutil::CHCKSI(b"TARGID", TARGID, b"=", 20, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    //
    // Check ZZBODS2C update after POOL mapping update.
    //
    // Call ZZBODS2C twice, first to make sure that there is an
    // update, second that there is not update.
    //
    testutil::TCASE(b"ZZBODS2C update after POOL mapping change.", ctx)?;

    fstr::assign(BUFFER.get_mut(1), b"NAIF_BODY_CODE += 30");
    fstr::assign(BUFFER.get_mut(2), b"NAIF_BODY_NAME += \'SUN\'");

    spicelib::LMPOOL(BUFFER.as_arg(), 2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::MOVEI(USRCTR.as_slice(), CTRSIZ, SAVCTR.as_slice_mut());

    fstr::assign(&mut TARG, b"SUN");
    TARGID = -1;
    FOUND = false;
    fstr::assign(&mut SVTARG, b"SUN");
    SVTGID = 10;
    SVFND = true;

    spicelib::ZZBODS2C(
        USRCTR.as_slice_mut(),
        &mut SVTARG,
        &mut SVTGID,
        &mut SVFND,
        &TARG,
        &mut TARGID,
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"USRCTR", USRCTR[1], b"!=", SAVCTR[1], 0, OK, ctx)?;
    testutil::CHCKSC(b"SVTARG", &SVTARG, b"=", b"SUN", OK, ctx)?;
    testutil::CHCKSI(b"SVTGID", SVTGID, b"=", 30, 0, OK, ctx)?;
    testutil::CHCKSL(b"SVFND", SVFND, true, OK, ctx)?;
    testutil::CHCKSC(b"TARG", &TARG, b"=", b"SUN", OK, ctx)?;
    testutil::CHCKSI(b"TARGID", TARGID, b"=", 30, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    spicelib::MOVEI(USRCTR.as_slice(), CTRSIZ, SAVCTR.as_slice_mut());

    spicelib::ZZBODS2C(
        USRCTR.as_slice_mut(),
        &mut SVTARG,
        &mut SVTGID,
        &mut SVFND,
        &TARG,
        &mut TARGID,
        &mut FOUND,
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
    testutil::CHCKSC(b"SVTARG", &SVTARG, b"=", b"SUN", OK, ctx)?;
    testutil::CHCKSI(b"SVTGID", SVTGID, b"=", 30, 0, OK, ctx)?;
    testutil::CHCKSL(b"SVFND", SVFND, true, OK, ctx)?;
    testutil::CHCKSC(b"TARG", &TARG, b"=", b"SUN", OK, ctx)?;
    testutil::CHCKSI(b"TARGID", TARGID, b"=", 30, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    //
    // Check ZZBODS2C update for input saved flag set to .FALSE.
    //
    // Call ZZBODS2C twice, first to make sure that there is an
    // update, second that there is not update.
    //
    testutil::TCASE(b"ZZBODS2C update after .FALSE. saved flag.", ctx)?;

    spicelib::MOVEI(USRCTR.as_slice(), CTRSIZ, SAVCTR.as_slice_mut());

    fstr::assign(&mut TARG, b"SUN");
    TARGID = -1;
    FOUND = false;
    fstr::assign(&mut SVTARG, b"SUN");
    SVTGID = -1;
    SVFND = false;

    spicelib::ZZBODS2C(
        USRCTR.as_slice_mut(),
        &mut SVTARG,
        &mut SVTGID,
        &mut SVFND,
        &TARG,
        &mut TARGID,
        &mut FOUND,
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
    testutil::CHCKSC(b"SVTARG", &SVTARG, b"=", b"SUN", OK, ctx)?;
    testutil::CHCKSI(b"SVTGID", SVTGID, b"=", 30, 0, OK, ctx)?;
    testutil::CHCKSL(b"SVFND", SVFND, true, OK, ctx)?;
    testutil::CHCKSC(b"TARG", &TARG, b"=", b"SUN", OK, ctx)?;
    testutil::CHCKSI(b"TARGID", TARGID, b"=", 30, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    spicelib::MOVEI(USRCTR.as_slice(), CTRSIZ, SAVCTR.as_slice_mut());

    spicelib::ZZBODS2C(
        USRCTR.as_slice_mut(),
        &mut SVTARG,
        &mut SVTGID,
        &mut SVFND,
        &TARG,
        &mut TARGID,
        &mut FOUND,
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
    testutil::CHCKSC(b"SVTARG", &SVTARG, b"=", b"SUN", OK, ctx)?;
    testutil::CHCKSI(b"SVTGID", SVTGID, b"=", 30, 0, OK, ctx)?;
    testutil::CHCKSL(b"SVFND", SVFND, true, OK, ctx)?;
    testutil::CHCKSC(b"TARG", &TARG, b"=", b"SUN", OK, ctx)?;
    testutil::CHCKSI(b"TARGID", TARGID, b"=", 30, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    //
    // Check ZZBODS2C update for different input name.
    //
    // Call ZZBODS2C twice, first to make sure that there is an
    // update, second that there is not update.
    //
    testutil::TCASE(b"ZZBODS2C update for different input name.", ctx)?;

    spicelib::MOVEI(USRCTR.as_slice(), CTRSIZ, SAVCTR.as_slice_mut());

    fstr::assign(&mut TARG, b"sun");
    TARGID = -1;
    FOUND = false;
    fstr::assign(&mut SVTARG, b"SUN");
    SVTGID = -1;
    SVFND = true;

    spicelib::ZZBODS2C(
        USRCTR.as_slice_mut(),
        &mut SVTARG,
        &mut SVTGID,
        &mut SVFND,
        &TARG,
        &mut TARGID,
        &mut FOUND,
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
    testutil::CHCKSC(b"SVTARG", &SVTARG, b"=", b"sun", OK, ctx)?;
    testutil::CHCKSI(b"SVTGID", SVTGID, b"=", 30, 0, OK, ctx)?;
    testutil::CHCKSL(b"SVFND", SVFND, true, OK, ctx)?;
    testutil::CHCKSC(b"TARG", &TARG, b"=", b"sun", OK, ctx)?;
    testutil::CHCKSI(b"TARGID", TARGID, b"=", 30, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    spicelib::MOVEI(USRCTR.as_slice(), CTRSIZ, SAVCTR.as_slice_mut());

    spicelib::ZZBODS2C(
        USRCTR.as_slice_mut(),
        &mut SVTARG,
        &mut SVTGID,
        &mut SVFND,
        &TARG,
        &mut TARGID,
        &mut FOUND,
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
    testutil::CHCKSC(b"SVTARG", &SVTARG, b"=", b"sun", OK, ctx)?;
    testutil::CHCKSI(b"SVTGID", SVTGID, b"=", 30, 0, OK, ctx)?;
    testutil::CHCKSL(b"SVFND", SVFND, true, OK, ctx)?;
    testutil::CHCKSC(b"TARG", &TARG, b"=", b"sun", OK, ctx)?;
    testutil::CHCKSI(b"TARGID", TARGID, b"=", 30, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    //
    // Clear POOL, reset BODTRN, and load some test data.
    //
    spicelib::CLPOOL(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZBODRST(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::KILFIL(b"spkfun.bsp", ctx)?;
    testutil::KILFIL(b"spkfun.ker", ctx)?;

    testutil::TSTSPK(b"spkfun.bsp", true, &mut HANDLE, ctx)?;
    testutil::TSTPCK(b"spkfun.ker", true, false, ctx)?;

    //
    // Test all routines that call ZZBODS2C. Do it by computing two
    // bench mark sets of data (at indexes 1 and 2), each for a unique,
    // non-overlapping set of bodies, then setting two unique sets of
    // body aliases and computing the same sets of data for them (at
    // indexes 3 and 4), and checking each "alias" set against its
    // benchmark. It saved mapping was updated correctly set 1 should
    // match set 3 and set 2 should match set 4.
    //
    // In addition to setting up all inputs we will call T_CTRBEQF to
    // adjust ZZBODS2C and ZZNAMFRM counters to the same value before
    // calling each of the routine that make use of saved values and
    // counters. This will help exposing any cases when the same counter
    // is erroneously used in more than one place.
    //
    for I in 1..=4 {
        if (I == 1) {
            fstr::assign(&mut BODY1, b"MERCURY");
            fstr::assign(&mut BODY2, b"VENUS");
            fstr::assign(&mut BODY3, b"EARTH");

            //
            // Set up FOV keywords required by GETFVN test case
            //
            spicelib::PCPOOL(
                b"INS199_FOV_FRAME",
                1,
                CharArray::from_ref(b"IAU_MERCURY"),
                ctx,
            )?;
            spicelib::PCPOOL(b"INS199_FOV_SHAPE", 1, CharArray::from_ref(b"CIRCLE"), ctx)?;
            spicelib::PDPOOL(b"INS199_BORESIGHT", 3, save.BSIGHT1.as_slice(), ctx)?;
            spicelib::PDPOOL(b"INS199_FOV_BOUNDARY", 3, save.BOUNDS1.as_slice(), ctx)?;

            CI = 1;

            fstr::assign(&mut TITLE, b"Check 1st bench data set against itself");
        } else if (I == 2) {
            fstr::assign(&mut BODY1, b"MARS");
            fstr::assign(&mut BODY2, b"JUPITER");
            fstr::assign(&mut BODY3, b"SATURN");

            //
            // Set up FOV keywords required by GETFVN test case
            //
            spicelib::PCPOOL(
                b"INS499_FOV_FRAME",
                1,
                CharArray::from_ref(b"IAU_MARS"),
                ctx,
            )?;
            spicelib::PCPOOL(b"INS499_FOV_SHAPE", 1, CharArray::from_ref(b"CIRCLE"), ctx)?;
            spicelib::PDPOOL(b"INS499_BORESIGHT", 3, save.BSIGHT2.as_slice(), ctx)?;
            spicelib::PDPOOL(b"INS499_FOV_BOUNDARY", 3, save.BOUNDS2.as_slice(), ctx)?;

            CI = 2;

            fstr::assign(&mut TITLE, b"Check 2nd bench data set against itself");
        } else if (I == 3) {
            fstr::assign(&mut BODY1, b"BODY1");
            fstr::assign(&mut BODY2, b"BODY2");
            fstr::assign(&mut BODY3, b"BODY3");

            CI = 1;

            fstr::assign(BUFFER.get_mut(1), b"NAIF_BODY_CODE += 199");
            fstr::assign(BUFFER.get_mut(2), b"NAIF_BODY_NAME += \'BODY1\'");

            fstr::assign(BUFFER.get_mut(3), b"NAIF_BODY_CODE += 299");
            fstr::assign(BUFFER.get_mut(4), b"NAIF_BODY_NAME += \'BODY2\'");

            fstr::assign(BUFFER.get_mut(5), b"NAIF_BODY_CODE += 399");
            fstr::assign(BUFFER.get_mut(6), b"NAIF_BODY_NAME += \'BODY3\'");

            spicelib::LMPOOL(BUFFER.as_arg(), 6, ctx)?;

            fstr::assign(
                &mut TITLE,
                b"Check 1st mapped data set against 1st benchmark",
            );
        } else if (I == 4) {
            fstr::assign(&mut BODY1, b"BODY1");
            fstr::assign(&mut BODY2, b"BODY2");
            fstr::assign(&mut BODY3, b"BODY3");

            CI = 2;

            fstr::assign(BUFFER.get_mut(1), b"NAIF_BODY_CODE += 499");
            fstr::assign(BUFFER.get_mut(2), b"NAIF_BODY_NAME += \'BODY1\'");

            fstr::assign(BUFFER.get_mut(3), b"NAIF_BODY_CODE += 599");
            fstr::assign(BUFFER.get_mut(4), b"NAIF_BODY_NAME += \'BODY2\'");

            fstr::assign(BUFFER.get_mut(5), b"NAIF_BODY_CODE += 699");
            fstr::assign(BUFFER.get_mut(6), b"NAIF_BODY_NAME += \'BODY3\'");

            spicelib::LMPOOL(BUFFER.as_arg(), 6, ctx)?;

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
        // Call all routines that make use of ZZBODS2C. Set arbitrary
        // input time.
        //
        ET = 10000000.0;

        //
        // -----------------------------------------------------------
        //
        T_CTRBEQF(ctx)?;

        spicelib::BODVRD(
            &BODY1,
            b"RADII",
            3,
            &mut N[I],
            RADII.subarray_mut([1, I]),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(b"BODVRD N", N[I], b"=", N[CI], 0, OK, ctx)?;
        testutil::CHCKAD(
            b"BODVRD RADII",
            RADII.subarray([1, I]),
            b"=",
            RADII.subarray([1, CI]),
            3,
            0.0,
            OK,
            ctx,
        )?;

        //
        // -----------------------------------------------------------
        //
        F = ((RADII[[1, I]] - RADII[[3, I]]) / RADII[[1, I]]);

        T_CTRBEQF(ctx)?;

        spicelib::DPGRDR(
            &BODY1,
            RADII[[1, I]],
            RADII[[2, I]],
            RADII[[3, I]],
            RADII[[1, I]],
            F,
            JACOB1.subarray_mut([1, 1, I]),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKAD(
            b"DPGRDR JACOBI",
            JACOB1.subarray([1, 1, I]),
            b"=",
            JACOB1.subarray([1, 1, CI]),
            9,
            0.0,
            OK,
            ctx,
        )?;

        //
        // -----------------------------------------------------------
        //
        T_CTRBEQF(ctx)?;

        spicelib::DRDPGR(
            &BODY1,
            0.5,
            0.5,
            1.0,
            RADII[[1, I]],
            F,
            JACOB2.subarray_mut([1, 1, I]),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKAD(
            b"DRDPGR JACOBI",
            JACOB2.subarray([1, 1, I]),
            b"=",
            JACOB2.subarray([1, 1, CI]),
            9,
            0.0,
            OK,
            ctx,
        )?;

        //
        // -----------------------------------------------------------
        //
        spicelib::BODN2C(&BODY1, &mut B1CODE, &mut FOUND, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(b"BODN2C FOUND", FOUND, true, OK, ctx)?;

        spicelib::CIDFRM(B1CODE, &mut FRCODE, &mut FIXREF, &mut FOUND, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(b"CIDFRM FOUND", FOUND, true, OK, ctx)?;

        T_CTRBEQF(ctx)?;

        spicelib::EDTERM(
            b"UMBRAL",
            &BODY2,
            &BODY1,
            ET,
            &FIXREF,
            b"NONE",
            &BODY3,
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
            b"EDTERM JACOBI",
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
        spicelib::VPACK(RADII[[1, I]], 0.0, 0.0, SPOINT.as_slice_mut());

        T_CTRBEQF(ctx)?;

        spicelib::ILLUM(
            &BODY1,
            ET,
            b"NONE",
            &BODY3,
            SPOINT.as_slice(),
            &mut PHASE1[I],
            &mut SOLAR1[I],
            &mut EMISS1[I],
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSD(b"ILLUM PHASE", PHASE1[I], b"=", PHASE1[CI], 0.0, OK, ctx)?;
        testutil::CHCKSD(b"ILLUM SOLAR", SOLAR1[I], b"=", SOLAR1[CI], 0.0, OK, ctx)?;
        testutil::CHCKSD(b"ILLUM EMISSN", EMISS1[I], b"=", EMISS1[CI], 0.0, OK, ctx)?;

        //
        // -----------------------------------------------------------
        //
        T_CTRBEQF(ctx)?;

        spicelib::ILLUMG(
            b"Ellipsoid",
            &BODY1,
            &BODY2,
            ET,
            &FIXREF,
            b"NONE",
            &BODY3,
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

        LS[I] = spicelib::LSPCN(&BODY1, ET, b"NONE", ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSD(b"LSPCN", LS[I], b"=", LS[CI], 0.0, OK, ctx)?;

        //
        // -----------------------------------------------------------
        //
        T_CTRBEQF(ctx)?;

        spicelib::PGRREC(
            &BODY1,
            0.5,
            0.5,
            1.0,
            RADII[[1, I]],
            F,
            RECTA1.subarray_mut([1, I]),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKAD(
            b"PGRREC RECTAN",
            RECTA1.subarray([1, I]),
            b"=",
            RECTA1.subarray([1, CI]),
            3,
            0.0,
            OK,
            ctx,
        )?;

        //
        // -----------------------------------------------------------
        //
        T_CTRBEQF(ctx)?;

        PHASE3[I] = spicelib::PHASEQ(ET, &BODY1, &BODY2, &BODY3, b"NONE", ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSD(b"PHASEQ", PHASE3[I], b"=", PHASE3[CI], 0.0, OK, ctx)?;

        //
        // -----------------------------------------------------------
        //
        spicelib::VPACK(
            RADII[[1, I]],
            RADII[[2, I]],
            RADII[[3, I]],
            RECTAN.as_slice_mut(),
        );

        T_CTRBEQF(ctx)?;

        spicelib::RECPGR(
            &BODY1,
            RECTAN.as_slice(),
            RADII[[1, I]],
            F,
            &mut LON[I],
            &mut LAT[I],
            &mut ALT[I],
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSD(b"RECPGR LON", LON[I], b"=", LON[CI], 0.0, OK, ctx)?;
        testutil::CHCKSD(b"RECPGR LAT", LAT[I], b"=", LAT[CI], 0.0, OK, ctx)?;
        testutil::CHCKSD(b"RECPGR ALT", ALT[I], b"=", ALT[CI], 0.0, OK, ctx)?;

        //
        // -----------------------------------------------------------
        //
        spicelib::BODN2C(&BODY3, &mut B3CODE, &mut FOUND, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(b"BODN2C FOUND", FOUND, true, OK, ctx)?;

        spicelib::SPKEZP(
            B1CODE,
            ET,
            b"J2000",
            b"NONE",
            B3CODE,
            DVEC.as_slice_mut(),
            &mut LT,
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        T_CTRBEQF(ctx)?;

        spicelib::SINCPT(
            b"Ellipsoid",
            &BODY1,
            ET,
            &FIXREF,
            b"NONE",
            &BODY3,
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

        T_CTRBEQF(ctx)?;

        spicelib::SPKCVO(
            &BODY1,
            ET,
            b"J2000",
            b"OBSERVER",
            b"NONE",
            OBSSTA.as_slice(),
            ET,
            &BODY3,
            b"J2000",
            STATE1.subarray_mut([1, I]),
            &mut LT1[I],
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKAD(
            b"SPKCVO STATE",
            STATE1.subarray([1, I]),
            b"=",
            STATE1.subarray([1, CI]),
            6,
            0.0,
            OK,
            ctx,
        )?;
        testutil::CHCKSD(b"SPKCVO LT", LT1[I], b"=", LT1[CI], 0.0, OK, ctx)?;

        //
        // -----------------------------------------------------------
        //
        T_CTRBEQF(ctx)?;

        spicelib::SPKCVT(
            OBSSTA.as_slice(),
            ET,
            &BODY1,
            &FIXREF,
            ET,
            b"J2000",
            b"OBSERVER",
            b"NONE",
            &BODY3,
            STATE2.subarray_mut([1, I]),
            &mut LT2[I],
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKAD(
            b"SPKCVT STATE",
            STATE2.subarray([1, I]),
            b"=",
            STATE2.subarray([1, CI]),
            6,
            0.0,
            OK,
            ctx,
        )?;
        testutil::CHCKSD(b"SPKCVT LT", LT2[I], b"=", LT2[CI], 0.0, OK, ctx)?;

        //
        // -----------------------------------------------------------
        //
        T_CTRBEQF(ctx)?;

        spicelib::SPKEZR(
            &BODY1,
            ET,
            b"J2000",
            b"NONE",
            &BODY3,
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

        spicelib::SPKPOS(
            &BODY1,
            ET,
            b"J2000",
            b"NONE",
            &BODY3,
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

        spicelib::SRFXPT(
            b"Ellipsoid",
            &BODY1,
            ET,
            b"NONE",
            &BODY3,
            b"J2000",
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
            &BODY1,
            ET,
            &FIXREF,
            b"NONE",
            &BODY3,
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

        spicelib::SUBPT(
            b"Near point",
            &BODY1,
            ET,
            b"NONE",
            &BODY3,
            SPOIN6.subarray_mut([1, I]),
            &mut ALT6[I],
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKAD(
            b"SUBPT SPOINT",
            SPOIN6.subarray([1, I]),
            b"=",
            SPOIN6.subarray([1, CI]),
            3,
            0.0,
            OK,
            ctx,
        )?;
        testutil::CHCKSD(b"SUBPT ALT", ALT6[I], b"=", ALT6[CI], 0.0, OK, ctx)?;

        //
        // -----------------------------------------------------------
        //
        T_CTRBEQF(ctx)?;

        spicelib::SUBSLR(
            b"Near point: ellipsoid",
            &BODY1,
            ET,
            &FIXREF,
            b"NONE",
            &BODY3,
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

        spicelib::SUBSOL(
            b"Near point",
            &BODY1,
            ET,
            b"NONE",
            &BODY3,
            SPOIN8.subarray_mut([1, I]),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKAD(
            b"SUBSOL SPOINT",
            SPOIN8.subarray([1, I]),
            b"=",
            SPOIN8.subarray([1, CI]),
            3,
            0.0,
            OK,
            ctx,
        )?;

        //
        // -----------------------------------------------------------
        //
        spicelib::VPACK(
            RADII[[1, I]],
            RADII[[2, I]],
            RADII[[3, I]],
            ISTATE.subarray_mut(1),
        );
        spicelib::VPACK(0.0, 0.0, 0.0, ISTATE.subarray_mut(4));

        T_CTRBEQF(ctx)?;

        spicelib::XFMSTA(
            ISTATE.as_slice(),
            b"RECTANGULAR",
            b"PLANETOGRAPHIC",
            &BODY1,
            OSTATE.subarray_mut([1, I]),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKAD(
            b"XFMSTA OSTATE",
            OSTATE.subarray([1, I]),
            b"=",
            OSTATE.subarray([1, CI]),
            6,
            0.0,
            OK,
            ctx,
        )?;

        //
        // -----------------------------------------------------------
        //
        T_CTRBEQF(ctx)?;

        spicelib::GETFVN(
            &BODY1,
            ROOM,
            &mut FOVSHP[I],
            &mut FOVFRM[I],
            FOVBS.subarray_mut([1, I]),
            &mut NBND[I],
            FOVBND.subarray_mut([1, I]),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSC(b"GETFVN SHAPE", &FOVSHP[I], b"=", &FOVSHP[CI], OK, ctx)?;
        testutil::CHCKSC(b"GETFVN FRAME", &FOVFRM[I], b"=", &FOVFRM[CI], OK, ctx)?;
        testutil::CHCKSI(b"GETFVN N", NBND[I], b"=", NBND[CI], 0, OK, ctx)?;
        testutil::CHCKAD(
            b"GETFVN BORESIGHT",
            FOVBS.subarray([1, I]),
            b"=",
            FOVBS.subarray([1, CI]),
            3,
            0.0,
            OK,
            ctx,
        )?;
        testutil::CHCKAD(
            b"GETFVN BOUNDS",
            FOVBND.subarray([1, I]),
            b"=",
            FOVBND.subarray([1, CI]),
            3,
            0.0,
            OK,
            ctx,
        )?;
    }

    //
    // Clean up.
    //
    testutil::KILFIL(b"spkfun.bsp", ctx)?;
    testutil::KILFIL(b"spkfun.ker", ctx)?;

    //
    // This is good enough for now.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
