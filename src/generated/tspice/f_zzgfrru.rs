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
const BDNMLN: i32 = 36;
const LNSIZE: i32 = 80;

struct SaveVars {
    TARGET: ActualCharArray,
    OBSRVR: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TARGET = ActualCharArray::new(BDNMLN, 1..=3);
        let mut OBSRVR = ActualCharArray::new(BDNMLN, 1..=3);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"EARTH"), Val::C(b"X"), Val::C(b"EARTH")].into_iter();
            TARGET
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"X"), Val::C(b"MARS"), Val::C(b"EARTH")].into_iter();
            OBSRVR
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { TARGET, OBSRVR }
    }
}

//$Procedure F_ZZGFRRU ( ZZGFRRU family tests )
pub fn F_ZZGFRRU(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ABCORR = [b' '; CORLEN as usize];
    let mut XABCOR = [b' '; CORLEN as usize];
    let mut TARG = [b' '; BDNMLN as usize];
    let mut OBS = [b' '; BDNMLN as usize];
    let mut TXT = [b' '; (2 * LNSIZE) as usize];
    let mut I: i32 = 0;
    let mut XTARG: i32 = 0;
    let mut XOBS: i32 = 0;
    let mut YTARG: i32 = 0;
    let mut YOBS: i32 = 0;
    let mut FOUND: bool = false;
    let mut DT: f64 = 0.0;
    let mut XDT: f64 = 0.0;

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
    // Indices 1:2 for Invalid body name test, 3 for not distinct
    // body names test.
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZGFRRU", ctx)?;

    //
    // Case 1: Invalid body names.
    //
    fstr::assign(&mut ABCORR, b"NONE");
    DT = 1.0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 2;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut TARG, save.TARGET.get(I));
            fstr::assign(&mut OBS, save.OBSRVR.get(I));

            fstr::assign(&mut TXT, b"Invalid body name test. TARG = #, OBS = #");
            spicelib::REPMC(&TXT.clone(), b"#", &TARG, &mut TXT);
            spicelib::REPMC(&TXT.clone(), b"#", &OBS, &mut TXT);

            testutil::TCASE(&TXT, ctx)?;

            spicelib::ZZGFRRIN(&TARG, &ABCORR, &OBS, DT, ctx)?;
            testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

            I += m3__;
        }
    }

    //
    // Case 2: Not distinct body names.
    //

    fstr::assign(&mut ABCORR, b"NONE");
    I = 3;
    DT = 1.0;
    fstr::assign(&mut TARG, save.TARGET.get(I));
    fstr::assign(&mut OBS, save.OBSRVR.get(I));

    fstr::assign(&mut TXT, b"Not distinct body name test. TARG = #, OBS = #");
    spicelib::REPMC(&TXT.clone(), b"#", &TARG, &mut TXT);
    spicelib::REPMC(&TXT.clone(), b"#", &OBS, &mut TXT);

    testutil::TCASE(&TXT, ctx)?;

    spicelib::ZZGFRRIN(&TARG, &ABCORR, &OBS, DT, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    //
    // Case 3: DT = 0.
    //

    fstr::assign(&mut ABCORR, b"NONE");
    DT = 0.0;
    fstr::assign(&mut TARG, b"EARTH");
    fstr::assign(&mut OBS, b"MOON");

    fstr::assign(&mut TXT, b"Delta value zero");

    testutil::TCASE(&TXT, ctx)?;

    spicelib::ZZGFRRIN(&TARG, &ABCORR, &OBS, DT, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDVALUE)", OK, ctx)?;

    //
    // Case 4: Confirm initialized values are correctly saved.
    //

    fstr::assign(&mut ABCORR, b"NONE");
    DT = 1.0;
    fstr::assign(&mut TARG, b"MERCURY");
    fstr::assign(&mut OBS, b"MARS");

    spicelib::BODS2C(&TARG, &mut YTARG, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODS2C(&OBS, &mut YOBS, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(
        &mut TXT,
        b"Initialize then check saved values in ZZGFRRIN. TARG = #, OBS = #, ABCORR = #, DT = #",
    );
    spicelib::REPMC(&TXT.clone(), b"#", &TARG, &mut TXT);
    spicelib::REPMC(&TXT.clone(), b"#", &OBS, &mut TXT);
    spicelib::REPMC(&TXT.clone(), b"#", &ABCORR, &mut TXT);
    spicelib::REPMD(&TXT.clone(), b"#", DT, 2, &mut TXT, ctx);

    testutil::TCASE(&TXT, ctx)?;

    spicelib::ZZGFRRIN(&TARG, &ABCORR, &OBS, DT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFRRX(&mut XTARG, &mut XABCOR, &mut XOBS, &mut XDT, ctx);

    testutil::CHCKSI(b"TARG X vs Y", XTARG, b"=", YTARG, 0, OK, ctx)?;
    testutil::CHCKSI(b"OBS X vs Y", XOBS, b"=", YOBS, 0, OK, ctx)?;

    spicelib::UCASE(&ABCORR.clone(), &mut ABCORR, ctx);
    testutil::CHCKSC(b"ABCORR vs XABCORR", &XABCOR, b"=", &ABCORR, OK, ctx)?;

    testutil::CHCKSD(b"DT vs XDT", DT, b"=", XDT, 0.0, OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
