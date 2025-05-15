//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const PCK: &[u8] = b"f_ls.tpc";
const SPK: &[u8] = b"f_ls.bsp";
const LOOSE: f64 = 0.005;
const LNSIZE: i32 = 80;
const NCASE: i32 = 100;
const NCORR: i32 = 3;
const CORLEN: i32 = 15;
const TIMLEN: i32 = 40;

struct SaveVars {
    ABCORR: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ABCORR = ActualCharArray::new(CORLEN, 1..=NCORR);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"NONE"), Val::C(b"LT"), Val::C(b"LT+S")].into_iter();
            ABCORR
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { ABCORR }
    }
}

//$Procedure      F_LS ( Test solar longitude routines )
pub fn F_LS(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TIMSTR = [b' '; TIMLEN as usize];
    let mut TITLE = [b' '; LNSIZE as usize];
    let mut DEC: f64 = 0.0;
    let mut ET: f64 = 0.0;
    let mut LS: f64 = 0.0;
    let mut LT: f64 = 0.0;
    let mut RA: f64 = 0.0;
    let mut RANGE: f64 = 0.0;
    let mut SSTATE = StackArray::<f64, 6>::new(1..=6);
    let mut HANDLE: i32 = 0;

    //
    // SPICELIB functions
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
    testutil::TOPEN(b"F_LS", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Setup:  create and load kernels.", ctx)?;
    //
    // Create, load, and delete a PCK, an SPK and an LSK.
    //
    testutil::TSTPCK(PCK, true, false, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTSPK(SPK, true, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Test for all aberration corrections.
    //
    for I in 1..=NCORR {
        //
        // For a variety of times of year, find the RA of
        // the earth-sun vector in the ECLIPJ2000 frame.
        //
        for J in 1..=NCASE {
            //
            // --- Case: ------------------------------------------------------
            //
            ET = ((((J - 1) as f64) * spicelib::JYEAR()) / NCASE as f64);

            fstr::assign(&mut TITLE, b"ET = #; ABCORR = #");

            spicelib::ETCAL(ET, &mut TIMSTR, ctx);
            spicelib::REPMC(&TITLE.clone(), b"#", &TIMSTR, &mut TITLE);
            spicelib::REPMC(&TITLE.clone(), b"#", &save.ABCORR[I], &mut TITLE);
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::TCASE(&TITLE, ctx)?;

            spicelib::SPKEZR(
                b"SUN",
                ET,
                b"ECLIPJ2000",
                &save.ABCORR[I],
                b"EARTH",
                SSTATE.as_slice_mut(),
                &mut LT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::RECRAD(SSTATE.as_slice(), &mut RANGE, &mut RA, &mut DEC, ctx);

            //
            // Find Ls.
            //
            LS = spicelib::LSPCN(b"EARTH", ET, &save.ABCORR[I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSD(b"LS", LS, b"~", RA, LOOSE, OK, ctx)?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up: delete SPK file.", ctx)?;
    //
    // Get rid of the SPK file.  First unload using the routine
    // corresponding to the loader called by TSTSPK.
    //
    spicelib::SPKUEF(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
