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
const SPK: &[u8] = b"zzgfdiu.bsp";
const VTIGHT: f64 = 0.000000000001;
const BDNMLN: i32 = 36;
const LNSIZE: i32 = 80;
const TIMLEN: i32 = 50;
const NCORR: i32 = 9;
const NSAMP: i32 = 50;

struct SaveVars {
    ABCORR: Vec<u8>,
    CORR: ActualCharArray,
    OBSRVR: Vec<u8>,
    TARGET: Vec<u8>,
    TITLE: Vec<u8>,
    TIMSTR: Vec<u8>,
    DIST: f64,
    ET: f64,
    ET0: f64,
    LT: f64,
    REFVAL: f64,
    STATE: StackArray<f64, 6>,
    STEPSZ: f64,
    XDIST: f64,
    HANDLE: i32,
    DECRES: bool,
    LSSTHN: bool,
    XDECRS: bool,
    XLSTHN: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ABCORR = vec![b' '; LNSIZE as usize];
        let mut CORR = ActualCharArray::new(LNSIZE, 1..=NCORR);
        let mut OBSRVR = vec![b' '; BDNMLN as usize];
        let mut TARGET = vec![b' '; BDNMLN as usize];
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut TIMSTR = vec![b' '; TIMLEN as usize];
        let mut DIST: f64 = 0.0;
        let mut ET: f64 = 0.0;
        let mut ET0: f64 = 0.0;
        let mut LT: f64 = 0.0;
        let mut REFVAL: f64 = 0.0;
        let mut STATE = StackArray::<f64, 6>::new(1..=6);
        let mut STEPSZ: f64 = 0.0;
        let mut XDIST: f64 = 0.0;
        let mut HANDLE: i32 = 0;
        let mut DECRES: bool = false;
        let mut LSSTHN: bool = false;
        let mut XDECRS: bool = false;
        let mut XLSTHN: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"NONE"),
                Val::C(b"lt"),
                Val::C(b" lt+s"),
                Val::C(b" cn"),
                Val::C(b" cn + s"),
                Val::C(b"XLT"),
                Val::C(b"XLT + S"),
                Val::C(b"XCN"),
                Val::C(b"XCN+S"),
            ]
            .into_iter();
            CORR.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            ABCORR,
            CORR,
            OBSRVR,
            TARGET,
            TITLE,
            TIMSTR,
            DIST,
            ET,
            ET0,
            LT,
            REFVAL,
            STATE,
            STEPSZ,
            XDIST,
            HANDLE,
            DECRES,
            LSSTHN,
            XDECRS,
            XLSTHN,
        }
    }
}

//$Procedure F_ZZGFDIU ( ZZGFDIU family tests )
pub fn F_ZZGFDIU(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

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
    // Save everything.
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZGFDIU", ctx)?;

    testutil::TCASE(b"Setup: create and load SPK, PCK, LSK files.", ctx)?;

    // Leapseconds:  Note that the LSK is deleted after loading, so we
    // don't have to clean it up later.
    //
    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load an SPK file as well.
    //
    testutil::TSTSPK(SPK, true, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    // CALL FURNSH ( 'de421.bsp' )
    // CALL CHCKXC ( .FALSE., ' ', OK )

    //*********************************************************************
    //*
    //*    Error cases
    //*
    //*********************************************************************

    //
    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Unrecognized target or observer", ctx)?;

    save.REFVAL = 100000.0;
    spicelib::ZZGFREF(save.REFVAL, ctx)?;

    spicelib::ZZGFDIIN(b"MOOON", b"NONE", b"EARTH", ctx)?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    spicelib::ZZGFDIIN(b"MOON", b"NONE", b"EEARTH", ctx)?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Target and observer are identical", ctx)?;

    spicelib::ZZGFDIIN(b"MOON", b"LT+S", b"MOON", ctx)?;

    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Bad aberration correction specifiers", ctx)?;

    spicelib::ZZGFDIIN(b"MOON", b"S", b"EARTH", ctx)?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::ZZGFDIIN(b"MOON", b"XS", b"EARTH", ctx)?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::ZZGFDIIN(b"MOON", b"RLT", b"EARTH", ctx)?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::ZZGFDIIN(b"MOON", b"XRLT", b"EARTH", ctx)?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::ZZGFDIIN(b"MOON", b"z", b"EARTH", ctx)?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"ZZGFDIDC: Ephemeris data unavailable", ctx)?;

    fstr::assign(&mut save.TIMSTR, b"2008 MAY 22");
    spicelib::STR2ET(&save.TIMSTR, &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFDIIN(b"GASPRA", b"LT+S", b"EARTH", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFDIDC(spicelib::UDF, &mut save.ET, &mut save.DECRES, ctx)?;
    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"ZZGFDILT: Ephemeris data unavailable", ctx)?;

    fstr::assign(&mut save.TIMSTR, b"2008 MAY 22");
    spicelib::STR2ET(&save.TIMSTR, &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFDIIN(b"GASPRA", b"LT+S", b"EARTH", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFUDLT(spicelib::ZZGFDIGQ, &mut save.ET, &mut save.LSSTHN, ctx)?;
    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"ZZGFDIGQ: Ephemeris data unavailable", ctx)?;

    fstr::assign(&mut save.TIMSTR, b"2008 MAY 22");
    spicelib::STR2ET(&save.TIMSTR, &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFDIIN(b"GASPRA", b"LT+S", b"EARTH", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFDIGQ(&mut save.ET, &mut save.DIST, ctx)?;
    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Normal cases
    //*
    //*********************************************************************

    //
    // Check values returned by ZZGFDIU entry points for each aberration
    // correction and a variety of times.
    //
    fstr::assign(&mut save.TIMSTR, b"2008 MAY 22");
    spicelib::STR2ET(&save.TIMSTR, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.STEPSZ = ((1000.0 / NSAMP as f64) * spicelib::SPD());

    for I in 1..=NCORR {
        //
        // Perform tests at a series of times.
        //
        fstr::assign(&mut save.ABCORR, save.CORR.get(I));

        for J in 1..=NSAMP {
            save.ET = (save.ET0 + (((J - 1) as f64) * save.STEPSZ));

            //
            // Use different observer-target pairs.
            //
            if spicelib::ODD(J) {
                fstr::assign(&mut save.OBSRVR, b"EARTH");
                fstr::assign(&mut save.TARGET, b"MOON");
            } else {
                fstr::assign(&mut save.OBSRVR, b"SUN");
                fstr::assign(&mut save.TARGET, b"MARS");
            }

            //
            //---- Case -------------------------------------------------------------
            //

            fstr::assign(
                &mut save.TITLE,
                b"ZZGFDIGQ: check #-# distance. ABCORR = #.",
            );
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.OBSRVR, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);

            testutil::TCASE(&save.TITLE, ctx)?;

            save.REFVAL = 400000.0;
            spicelib::ZZGFREF(save.REFVAL, ctx)?;

            spicelib::ZZGFDIIN(&save.TARGET, &save.ABCORR, &save.OBSRVR, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::ZZGFDIGQ(&mut save.ET, &mut save.DIST, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::SPKEZR(
                &save.TARGET,
                save.ET,
                b"J2000",
                &save.ABCORR,
                &save.OBSRVR,
                save.STATE.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.XDIST = spicelib::VNORM(save.STATE.as_slice());

            //
            // We expect a pretty good match.
            //
            testutil::CHCKSD(b"DIST", save.DIST, b"~/", save.XDIST, VTIGHT, OK, ctx)?;

            //
            //---- Case -------------------------------------------------------------
            //
            fstr::assign(
                &mut save.TITLE,
                b"ZZGFDILT: check #-# \"is less than?\" state. ABCORR = #.",
            );
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.OBSRVR, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);

            testutil::TCASE(&save.TITLE, ctx)?;

            save.XLSTHN = (save.XDIST < save.REFVAL);

            spicelib::ZZGFUDLT(spicelib::ZZGFDIGQ, &mut save.ET, &mut save.LSSTHN, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"LSSTHN", save.LSSTHN, save.XLSTHN, OK, ctx)?;

            //
            //---- Case -------------------------------------------------------------
            //
            fstr::assign(
                &mut save.TITLE,
                b"ZZGFREF: check #-# \"is less than?\" state. ABCORR = #.",
            );
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.OBSRVR, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);

            testutil::TCASE(&save.TITLE, ctx)?;

            save.XLSTHN = (save.XDIST < save.REFVAL);
            //
            // Set the reference value so as to invert the
            // expected relationship to the reference value.
            //
            if save.XLSTHN {
                save.REFVAL = spicelib::DPMIN();
            } else {
                save.REFVAL = spicelib::DPMAX();
            }
            spicelib::ZZGFREF(save.REFVAL, ctx)?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.XLSTHN = !save.XLSTHN;

            spicelib::ZZGFUDLT(spicelib::ZZGFDIGQ, &mut save.ET, &mut save.LSSTHN, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"LSSTHN", save.LSSTHN, save.XLSTHN, OK, ctx)?;

            //
            //---- Case -------------------------------------------------------------
            //
            fstr::assign(
                &mut save.TITLE,
                b"ZZGFDIDC: check #-# \"is decreasing?\" state. ABCORR = #.",
            );
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.OBSRVR, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);

            testutil::TCASE(&save.TITLE, ctx)?;

            save.XDECRS = (spicelib::VDOT(save.STATE.as_slice(), save.STATE.subarray(4)) < 0.0);

            spicelib::ZZGFDIDC(spicelib::UDF, &mut save.ET, &mut save.DECRES, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"DECRES", save.DECRES, save.XDECRS, OK, ctx)?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up:  delete kernels.", ctx)?;

    spicelib::SPKUEF(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
