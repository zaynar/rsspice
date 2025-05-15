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
const NRCORR: i32 = 5;
const NXCORR: i32 = 4;
const BDNMLN: i32 = 36;
const LNSIZE: i32 = 80;

struct SaveVars {
    RCORR: ActualCharArray,
    XCORR: ActualCharArray,
    TARGET: ActualCharArray,
    ILLUMN: ActualCharArray,
    OBSRVR: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut RCORR = ActualCharArray::new(CORLEN, 1..=NRCORR);
        let mut XCORR = ActualCharArray::new(CORLEN, 1..=NXCORR);
        let mut TARGET = ActualCharArray::new(BDNMLN, 1..=6);
        let mut ILLUMN = ActualCharArray::new(BDNMLN, 1..=6);
        let mut OBSRVR = ActualCharArray::new(BDNMLN, 1..=6);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"nOne"),
                Val::C(b"lT"),
                Val::C(b"Cn"),
                Val::C(b"Lt+s"),
                Val::C(b"cN+S"),
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
                Val::C(b"xlT"),
                Val::C(b"xCn"),
                Val::C(b"XLt+s"),
                Val::C(b"XcN+S"),
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

//$Procedure F_ZZGFPAU ( ZZGFPAU family tests )
pub fn F_ZZGFPAU(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ABCORR = [b' '; CORLEN as usize];
    let mut XABCOR = [b' '; CORLEN as usize];
    let mut TARG = [b' '; BDNMLN as usize];
    let mut ILLUM = [b' '; BDNMLN as usize];
    let mut OBS = [b' '; BDNMLN as usize];
    let mut TXT = [b' '; (2 * LNSIZE) as usize];
    let mut ET: f64 = 0.0;
    let mut RVL: f64 = 0.0;
    let mut HANDLE: i32 = 0;
    let mut XTARG: i32 = 0;
    let mut XILLUM: i32 = 0;
    let mut XOBS: i32 = 0;
    let mut YTARG: i32 = 0;
    let mut YILLUM: i32 = 0;
    let mut YOBS: i32 = 0;
    let mut XABLK = StackArray::<bool, 15>::new(1..=NABCOR);
    let mut ABLK = StackArray::<bool, 15>::new(1..=NABCOR);
    let mut DECRES: bool = false;
    let mut FOUND: bool = false;

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
    // Indices 1:3 for Invalid body name test, 4:6 for not distinct
    // body names test.
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZGFPAU", ctx)?;

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

    testutil::NATSPK(SPK, true, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(SPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Case 2: Invalid body names.
    //
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

        spicelib::ZZGFPAIN(&TARG, &ILLUM, &ABCORR, &OBS, ctx)?;
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

        spicelib::ZZGFPAIN(&TARG, &ILLUM, &ABCORR, &OBS, ctx)?;
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

        spicelib::ZZGFPAIN(&TARG, &ILLUM, &ABCORR, &OBS, ctx)?;
        testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;
    }

    //
    // Case 5: Confirm initialized values are correctly saved.
    //

    fstr::assign(&mut TARG, save.TARGET.get(1));
    fstr::assign(&mut ILLUM, save.ILLUMN.get(1));
    fstr::assign(&mut OBS, save.OBSRVR.get(2));
    fstr::assign(&mut ABCORR, save.RCORR.get(1));

    spicelib::BODS2C(&TARG, &mut YTARG, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODS2C(&ILLUM, &mut YILLUM, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODS2C(&OBS, &mut YOBS, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Aberration correction NONE.
    //

    for J in 1..=ABATSZ {
        ABLK[J] = false;
        XABLK[J] = false;
    }

    fstr::assign(&mut TXT, b"Initialize then check saved values in ZZGFPAIN. TARG = #, ILLUM = #, OBS = #, ABCORR = #.");
    spicelib::REPMC(&TXT.clone(), b"#", &TARG, &mut TXT);
    spicelib::REPMC(&TXT.clone(), b"#", &ILLUM, &mut TXT);
    spicelib::REPMC(&TXT.clone(), b"#", &OBS, &mut TXT);
    spicelib::REPMC(&TXT.clone(), b"#", &ABCORR, &mut TXT);

    testutil::TCASE(&TXT, ctx)?;

    spicelib::ZZGFPAIN(&TARG, &ILLUM, &ABCORR, &OBS, ctx)?;

    spicelib::ZZGFPAX(
        &mut XTARG,
        &mut XILLUM,
        &mut XABCOR,
        &mut XOBS,
        XABLK.as_slice_mut(),
        ctx,
    );

    testutil::CHCKSI(b"TARG X vs Y", XTARG, b"=", YTARG, 0, OK, ctx)?;
    testutil::CHCKSI(b"ILLUM X vs Y", XILLUM, b"=", YILLUM, 0, OK, ctx)?;
    testutil::CHCKSI(b"OBS X vs Y", XOBS, b"=", YOBS, 0, OK, ctx)?;

    spicelib::UCASE(&ABCORR.clone(), &mut ABCORR, ctx);
    testutil::CHCKSC(b"ABCORR vs XABCORR", &XABCOR, b"=", &ABCORR, OK, ctx)?;

    spicelib::ZZVALCOR(&ABCORR, ABLK.as_slice_mut(), ctx)?;

    for J in 1..=ABATSZ {
        testutil::CHCKSL(b"ABLK", XABLK[J], ABLK[J], OK, ctx)?;
    }

    //
    // Aberration correction not NONE.
    //

    for I in 2..=NRCORR {
        for J in 1..=ABATSZ {
            ABLK[J] = false;
            XABLK[J] = false;
        }

        fstr::assign(&mut TXT, b"Initialize then check saved values in ZZGFPAIN. TARG = #, ILLUM = #, OBS = #, ABCORR = #.");
        spicelib::REPMC(&TXT.clone(), b"#", &TARG, &mut TXT);
        spicelib::REPMC(&TXT.clone(), b"#", &ILLUM, &mut TXT);
        spicelib::REPMC(&TXT.clone(), b"#", &OBS, &mut TXT);

        fstr::assign(&mut ABCORR, save.RCORR.get(I));

        spicelib::REPMC(&TXT.clone(), b"#", &ABCORR, &mut TXT);

        spicelib::ZZVALCOR(&ABCORR, ABLK.as_slice_mut(), ctx)?;

        testutil::TCASE(&TXT, ctx)?;

        spicelib::ZZGFPAIN(&TARG, &ILLUM, &ABCORR, &OBS, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::ZZGFPAX(
            &mut XTARG,
            &mut XILLUM,
            &mut XABCOR,
            &mut XOBS,
            XABLK.as_slice_mut(),
            ctx,
        );

        spicelib::UCASE(&ABCORR.clone(), &mut ABCORR, ctx);
        testutil::CHCKSC(b"ABCORR vs Y", &XABCOR, b"=", &ABCORR, OK, ctx)?;

        for J in 1..=ABATSZ {
            testutil::CHCKSL(&TXT, XABLK[J], ABLK[J], OK, ctx)?;
        }
    }

    //
    // Case 6: Check calculation of phase angle derivative.
    //

    //
    // Occultation of ALPHA by BETA as observed from Sol starts
    // at 0 TDB secs from J2000 epoch (geometric). The occultation
    // lasts ten minutes.
    //
    // Confirm the phase angle decreases prior to occultation
    // and increases post occultation.
    //
    fstr::assign(&mut TARG, b"ALPHA");
    fstr::assign(&mut ILLUM, b"SUN");
    fstr::assign(&mut OBS, b"BETA");

    //
    // Loop over all aberration corrections.
    //
    for I in 1..=NRCORR {
        fstr::assign(&mut ABCORR, save.RCORR.get(I));

        fstr::assign(
            &mut TXT,
            b"Phase angle decreasing prior and increasing post occultaion. ABCORR = #.",
        );
        spicelib::REPMC(&TXT.clone(), b"#", &ABCORR, &mut TXT);

        testutil::TCASE(&TXT, ctx)?;

        spicelib::ZZGFPAIN(&TARG, &ILLUM, &ABCORR, &OBS, ctx)?;

        DECRES = false;

        //
        // Ten minutes prior to start of occultation.
        //
        ET = -(10.0 * 60.0);
        spicelib::ZZGFPAGQ(&mut ET, &mut RVL, ctx)?;
        spicelib::ZZGFPADC(spicelib::UDF, &mut ET, &mut DECRES, ctx)?;

        testutil::CHCKSL(&TXT, DECRES, true, OK, ctx)?;

        //
        // Ten minutes post to start of occultation.
        //
        ET = (10.0 * 60.0);
        spicelib::ZZGFPAGQ(&mut ET, &mut RVL, ctx)?;
        spicelib::ZZGFPADC(spicelib::UDF, &mut ET, &mut DECRES, ctx)?;

        testutil::CHCKSL(&TXT, DECRES, false, OK, ctx)?;
    }

    //
    // Case N:
    //
    testutil::TCASE(b"Clean up:  delete kernels.", ctx)?;

    spicelib::KCLEAR(ctx)?;

    spicelib::DELFIL(PCK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
