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
const RECSYS: &[u8] = b"RECTANGULAR";
const LATSYS: &[u8] = b"LATITUDINAL";
const SPHSYS: &[u8] = b"SPHERICAL";
const RADSYS: &[u8] = b"RA/DEC";
const CYLSYS: &[u8] = b"CYLINDRICAL";
const GEOSYS: &[u8] = b"GEODETIC";
const PGRSYS: &[u8] = b"PLANETOGRAPHIC";
const XCRD: &[u8] = b"X";
const YCRD: &[u8] = b"Y";
const ZCRD: &[u8] = b"Z";
const RADCRD: &[u8] = b"RADIUS";
const LONCRD: &[u8] = b"LONGITUDE";
const LATCRD: &[u8] = b"LATITUDE";
const RACRD: &[u8] = b"RIGHT ASCENSION";
const DECCRD: &[u8] = b"DECLINATION";
const RNGCRD: &[u8] = b"RANGE";
const CLTCRD: &[u8] = b"COLATITUDE";
const ALTCRD: &[u8] = b"ALTITUDE";
const POSDEF: &[u8] = b"POSITION";
const SOBDEF: &[u8] = b"SUB-OBSERVER POINT";
const SINDEF: &[u8] = b"SURFACE INTERCEPT POINT";
const NWREL: i32 = 5;
const NWLONG: i32 = 7;
const EXWIDX: i32 = ((NWREL + NWLONG) + 1);
const MXBEGM: i32 = 55;
const MXENDM: i32 = 13;
const MXMSG: i32 = ((MXBEGM + MXENDM) + 10);
const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;
const PCK: &[u8] = b"zzgfssob.tpc";
const SPK: &[u8] = b"zzgfssob.bsp";
const TIGHT: f64 = 0.00000000000001;
const BDNMLN: i32 = 36;
const LNSIZE: i32 = 200;
const FRNMLN: i32 = 32;
const TIMLEN: i32 = 50;
const NCORR: i32 = 9;
const NREF: i32 = 4;
const NSAMP: i32 = 10;
const VDFLEN: i32 = 32;
const NVDEF: i32 = 3;
const MTHLEN: i32 = 80;

struct SaveVars {
    ABCORR: Vec<u8>,
    CORR: ActualCharArray,
    DREF: Vec<u8>,
    EFRAME: ActualCharArray,
    METHOD: Vec<u8>,
    OBSRVR: Vec<u8>,
    REF: Vec<u8>,
    TARGET: Vec<u8>,
    TIMSTR: Vec<u8>,
    TITLE: Vec<u8>,
    VCDEFS: ActualCharArray,
    VECDEF: Vec<u8>,
    DELTA: f64,
    DVEC: StackArray<f64, 3>,
    ET: f64,
    ET0: f64,
    LT: f64,
    NEGVEC: StackArray<f64, 3>,
    RADII: StackArray<f64, 3>,
    STATE: StackArray<f64, 6>,
    XSTATE: StackArray<f64, 6>,
    DCTR: i32,
    FRCLID: i32,
    FRCLSS: i32,
    FRCODE: i32,
    HANDLE: i32,
    N: i32,
    NF: i32,
    OBSID: i32,
    TRGID: i32,
    ATTBLK: StackArray<bool, 6>,
    FND: bool,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ABCORR = vec![b' '; LNSIZE as usize];
        let mut CORR = ActualCharArray::new(LNSIZE, 1..=NCORR);
        let mut DREF = vec![b' '; FRNMLN as usize];
        let mut EFRAME = ActualCharArray::new(FRNMLN, 1..=NREF);
        let mut METHOD = vec![b' '; MTHLEN as usize];
        let mut OBSRVR = vec![b' '; BDNMLN as usize];
        let mut REF = vec![b' '; FRNMLN as usize];
        let mut TARGET = vec![b' '; BDNMLN as usize];
        let mut TIMSTR = vec![b' '; TIMLEN as usize];
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut VCDEFS = ActualCharArray::new(VDFLEN, 1..=NVDEF);
        let mut VECDEF = vec![b' '; VDFLEN as usize];
        let mut DELTA: f64 = 0.0;
        let mut DVEC = StackArray::<f64, 3>::new(1..=3);
        let mut ET: f64 = 0.0;
        let mut ET0: f64 = 0.0;
        let mut LT: f64 = 0.0;
        let mut NEGVEC = StackArray::<f64, 3>::new(1..=3);
        let mut RADII = StackArray::<f64, 3>::new(1..=3);
        let mut STATE = StackArray::<f64, 6>::new(1..=6);
        let mut XSTATE = StackArray::<f64, 6>::new(1..=6);
        let mut DCTR: i32 = 0;
        let mut FRCLID: i32 = 0;
        let mut FRCLSS: i32 = 0;
        let mut FRCODE: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut N: i32 = 0;
        let mut NF: i32 = 0;
        let mut OBSID: i32 = 0;
        let mut TRGID: i32 = 0;
        let mut ATTBLK = StackArray::<bool, 6>::new(1..=ABATSZ);
        let mut FND: bool = false;
        let mut FOUND: bool = false;

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
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"J2000"),
                Val::C(b"IAU_EARTH"),
                Val::C(b"ECLIPJ2000"),
                Val::C(b"IAU_MOON"),
            ]
            .into_iter();
            EFRAME
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(POSDEF), Val::C(SOBDEF), Val::C(SINDEF)].into_iter();
            VCDEFS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            ABCORR,
            CORR,
            DREF,
            EFRAME,
            METHOD,
            OBSRVR,
            REF,
            TARGET,
            TIMSTR,
            TITLE,
            VCDEFS,
            VECDEF,
            DELTA,
            DVEC,
            ET,
            ET0,
            LT,
            NEGVEC,
            RADII,
            STATE,
            XSTATE,
            DCTR,
            FRCLID,
            FRCLSS,
            FRCODE,
            HANDLE,
            N,
            NF,
            OBSID,
            TRGID,
            ATTBLK,
            FND,
            FOUND,
        }
    }
}

//$Procedure      F_ZZGFCOST ( Test GF coordinate state computation )
pub fn F_ZZGFCOST(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
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
    // Saved Variables
    //

    //
    // Initial values
    //

    //
    // Vector definition methods:
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZGFCOST", ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Setup: create and load SPK, PCK, LSK files.", ctx)?;

    // Leapseconds:  Note that the LSK is deleted after loading, so we
    // don't have to clean it up later.
    //
    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load a PCK.
    //
    testutil::TSTPCK(PCK, true, false, ctx)?;

    //
    // Load an SPK file as well.
    //
    testutil::TSTSPK(SPK, true, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Actual DE-based ephemerides yield better comparisons,
    // since these ephemerides have less noise than do those
    // produced by TSTSPK.
    //
    //  CALL FURNSH ( 'de421.bsp' )
    //  CALL FURNSH ( 'jup230.bsp' )
    //

    //*********************************************************************
    //*
    //*    Error cases
    //*
    //*********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad value of VECDEF", ctx)?;

    fstr::assign(&mut save.ABCORR, b"CN+S");
    fstr::assign(&mut save.METHOD, b"ellipsoid");
    fstr::assign(&mut save.TARGET, b"EARTH");
    save.TRGID = 399;
    fstr::assign(&mut save.OBSRVR, b"MOON");
    save.OBSID = 301;
    fstr::assign(&mut save.REF, b"IAU_EARTH");
    fstr::assign(&mut save.DREF, &save.REF);
    save.DCTR = 399;

    spicelib::VPACK(0.0, 0.0, 1.0, save.DVEC.as_slice_mut());

    fstr::assign(&mut save.TIMSTR, b"2008 MAY 1");

    spicelib::STR2ET(&save.TIMSTR, &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODVRD(
        &save.TARGET,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.VECDEF, b"Surface xcept");

    spicelib::ZZGFCOST(
        &save.VECDEF,
        &save.METHOD,
        save.TRGID,
        save.ET,
        &save.REF,
        &save.ABCORR,
        save.OBSID,
        &save.DREF,
        save.DCTR,
        save.DVEC.as_slice(),
        save.RADII.as_slice(),
        save.STATE.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    fstr::assign(&mut save.VECDEF, SINDEF);

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad aberration correction", ctx)?;

    save.OBSID = 301;
    save.DCTR = 399;
    fstr::assign(&mut save.ABCORR, b"None.");

    spicelib::ZZGFCOST(
        &save.VECDEF,
        &save.METHOD,
        save.TRGID,
        save.ET,
        &save.REF,
        &save.ABCORR,
        save.OBSID,
        &save.DREF,
        save.DCTR,
        save.DVEC.as_slice(),
        save.RADII.as_slice(),
        save.STATE.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad computation method", ctx)?;

    fstr::assign(&mut save.ABCORR, b"None");
    fstr::assign(&mut save.METHOD, b"Near point");

    spicelib::ZZGFCOST(
        &save.VECDEF,
        &save.METHOD,
        save.TRGID,
        save.ET,
        &save.REF,
        &save.ABCORR,
        save.OBSID,
        &save.DREF,
        save.DCTR,
        save.DVEC.as_slice(),
        save.RADII.as_slice(),
        save.STATE.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No target orientation data available", ctx)?;

    fstr::assign(&mut save.METHOD, b"ellipsoid");
    fstr::assign(&mut save.REF, b"ITRF93");

    spicelib::ZZGFCOST(
        &save.VECDEF,
        &save.METHOD,
        save.TRGID,
        save.ET,
        &save.REF,
        &save.ABCORR,
        save.OBSID,
        &save.DREF,
        save.DCTR,
        save.DVEC.as_slice(),
        save.RADII.as_slice(),
        save.STATE.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(FRAMEDATANOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Reference frame not centered on target", ctx)?;

    fstr::assign(&mut save.VECDEF, SOBDEF);
    fstr::assign(&mut save.METHOD, b"NEAR POINT: ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"Mars");
    save.TRGID = 499;
    fstr::assign(&mut save.REF, b"IAU_EARTH");

    spicelib::ZZGFCOST(
        &save.VECDEF,
        &save.METHOD,
        save.TRGID,
        save.ET,
        &save.REF,
        &save.ABCORR,
        save.OBSID,
        &save.DREF,
        save.DCTR,
        save.DVEC.as_slice(),
        save.RADII.as_slice(),
        save.STATE.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No target ephemeris data available", ctx)?;

    fstr::assign(&mut save.TARGET, b"GASPRA");
    save.TRGID = 9511010;
    fstr::assign(&mut save.REF, b"IAU_GASPRA");

    spicelib::ZZGFCOST(
        &save.VECDEF,
        &save.METHOD,
        save.TRGID,
        save.ET,
        &save.REF,
        &save.ABCORR,
        save.OBSID,
        &save.DREF,
        save.DCTR,
        save.DVEC.as_slice(),
        save.RADII.as_slice(),
        save.STATE.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No observer ephemeris data available", ctx)?;

    fstr::assign(&mut save.TARGET, b"MOON");
    save.TRGID = 301;
    fstr::assign(&mut save.OBSRVR, b"GASPRA");
    save.OBSID = 9511010;
    fstr::assign(&mut save.REF, b"IAU_MOON");

    spicelib::ZZGFCOST(
        &save.VECDEF,
        &save.METHOD,
        save.TRGID,
        save.ET,
        &save.REF,
        &save.ABCORR,
        save.OBSID,
        &save.DREF,
        save.DCTR,
        save.DVEC.as_slice(),
        save.RADII.as_slice(),
        save.STATE.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No DREF ephemeris data available", ctx)?;

    fstr::assign(&mut save.DREF, b"IAU_GASPRA");
    save.DCTR = 9511010;
    spicelib::VPACK(0.0, 0.0, 1.0, save.DVEC.as_slice_mut());

    spicelib::ZZGFCOST(
        &save.VECDEF,
        &save.METHOD,
        save.TRGID,
        save.ET,
        &save.REF,
        &save.ABCORR,
        save.OBSID,
        &save.DREF,
        save.DCTR,
        save.DVEC.as_slice(),
        save.RADII.as_slice(),
        save.STATE.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No DREF orientation data available", ctx)?;

    fstr::assign(&mut save.VECDEF, SINDEF);
    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"EARTH");
    save.TRGID = 399;
    fstr::assign(&mut save.OBSRVR, b"MOON");
    save.OBSID = 301;
    fstr::assign(&mut save.REF, b"IAU_EARTH");
    save.DCTR = 399;
    fstr::assign(&mut save.DREF, b"ITRF93");
    save.DCTR = 399;
    spicelib::VPACK(0.0, 0.0, 1.0, save.DVEC.as_slice_mut());

    spicelib::ZZGFCOST(
        &save.VECDEF,
        &save.METHOD,
        save.TRGID,
        save.ET,
        &save.REF,
        &save.ABCORR,
        save.OBSID,
        &save.DREF,
        save.DCTR,
        save.DVEC.as_slice(),
        save.RADII.as_slice(),
        save.STATE.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(FRAMEDATANOTFOUND)", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Normal cases
    //*
    //*********************************************************************

    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.REF, b"IAU_EARTH");
    save.DCTR = 399;

    spicelib::BODVRD(
        &save.TARGET,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODN2C(&save.TARGET, &mut save.TRGID, &mut save.FOUND, ctx)?;
    spicelib::BODN2C(&save.OBSRVR, &mut save.OBSID, &mut save.FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We're going to loop over all vector definitions,
    // aberration corrections, coordinate systems, and
    // coordinates.
    //
    //
    for ND in 1..=NVDEF {
        fstr::assign(&mut save.VECDEF, save.VCDEFS.get(ND));

        if fstr::eq(&save.VECDEF, SINDEF) {
            fstr::assign(&mut save.METHOD, b"Ellipsoid");
        } else if fstr::eq(&save.VECDEF, SOBDEF) {
            fstr::assign(&mut save.METHOD, b"Near point: ellipsoid");
        } else {
            fstr::assign(&mut save.METHOD, b" ");
        }

        for NC in 1..=NCORR {
            fstr::assign(&mut save.ABCORR, save.CORR.get(NC));

            spicelib::ZZPRSCOR(&save.ABCORR, save.ATTBLK.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Loop over the set of sample times.
            //
            fstr::assign(&mut save.TIMSTR, b"2008 MAY 1");

            spicelib::STR2ET(&save.TIMSTR, &mut save.ET0, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.DELTA = ((100 as f64) * spicelib::SPD());

            for NS in 1..=NSAMP {
                //
                // --- Case: ------------------------------------------------------
                //
                if fstr::eq(&save.VECDEF, POSDEF) {
                    fstr::assign(&mut save.TITLE, b"#-# position; #; #");

                    spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.OBSRVR, &mut save.TITLE);
                    spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
                    spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);
                } else if fstr::eq(&save.VECDEF, SOBDEF) {
                    fstr::assign(&mut save.TITLE, b"# sub # point on #; #; #");

                    spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.METHOD, &mut save.TITLE);
                    spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.OBSRVR, &mut save.TITLE);
                    spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
                    spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);
                } else if fstr::eq(&save.VECDEF, SINDEF) {
                    save.NF = (intrinsics::MOD(NS, 4) + 1);
                    fstr::assign(&mut save.DREF, save.EFRAME.get(save.NF));

                    fstr::assign(&mut save.TITLE, b"# #-# intercept. #; DREF = #; #");

                    spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.METHOD, &mut save.TITLE);
                    spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.OBSRVR, &mut save.TITLE);
                    spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
                    spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);
                    spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.DREF, &mut save.TITLE);
                }

                save.ET = (save.ET0 + (((NS - 1) as f64) * save.DELTA));
                spicelib::ETCAL(save.ET, &mut save.TIMSTR, ctx);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TIMSTR, &mut save.TITLE);

                testutil::TCASE(&save.TITLE, ctx)?;

                if fstr::eq(&save.VECDEF, SINDEF) {
                    //
                    // Look up the direction vector.
                    //
                    spicelib::SPKPOS(
                        &save.TARGET,
                        save.ET,
                        &save.DREF,
                        &save.ABCORR,
                        &save.OBSRVR,
                        save.DVEC.as_slice_mut(),
                        &mut save.LT,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::NAMFRM(&save.DREF, &mut save.FRCODE, ctx)?;
                    testutil::CHCKSI(b"FRCODE ", save.FRCODE, b"!=", 0, 0, OK, ctx)?;

                    spicelib::FRINFO(
                        save.FRCODE,
                        &mut save.DCTR,
                        &mut save.FRCLSS,
                        &mut save.FRCLID,
                        &mut save.FND,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    testutil::CHCKSL(b"FRINFO FND", save.FND, true, OK, ctx)?;
                }

                //
                // Look up the expected state vector.
                //
                if fstr::eq(&save.VECDEF, POSDEF) {
                    spicelib::SPKEZR(
                        &save.TARGET,
                        save.ET,
                        &save.REF,
                        &save.ABCORR,
                        &save.OBSRVR,
                        save.XSTATE.as_slice_mut(),
                        &mut save.LT,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                } else if fstr::eq(&save.VECDEF, SOBDEF) {
                    spicelib::ZZGFSSOB(
                        &save.METHOD,
                        save.TRGID,
                        save.ET,
                        &save.REF,
                        &save.ABCORR,
                        save.OBSID,
                        save.RADII.as_slice(),
                        save.XSTATE.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                } else {
                    spicelib::ZZGFSSIN(
                        &save.METHOD,
                        save.TRGID,
                        save.ET,
                        &save.REF,
                        &save.ABCORR,
                        save.OBSID,
                        &save.DREF,
                        save.DCTR,
                        save.DVEC.as_slice(),
                        save.RADII.as_slice(),
                        save.XSTATE.as_slice_mut(),
                        &mut save.FOUND,
                        ctx,
                    )?;

                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    //
                    // By construction, the intercept should be found.
                    //
                    testutil::CHCKSL(b"intercept found", save.FOUND, true, OK, ctx)?;
                }

                //
                // Get the result from ZZGFCOST.
                //
                spicelib::ZZGFCOST(
                    &save.VECDEF,
                    &save.METHOD,
                    save.TRGID,
                    save.ET,
                    &save.REF,
                    &save.ABCORR,
                    save.OBSID,
                    &save.DREF,
                    save.DCTR,
                    save.DVEC.as_slice(),
                    save.RADII.as_slice(),
                    save.STATE.as_slice_mut(),
                    &mut save.FOUND,
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

                //
                // Check position.
                //
                testutil::CHCKAD(
                    b"Position",
                    save.STATE.as_slice(),
                    b"~~/",
                    save.XSTATE.as_slice(),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                //
                // Check velocity.
                //
                testutil::CHCKAD(
                    b"Velocity",
                    save.STATE.subarray(4),
                    b"~~/",
                    save.XSTATE.subarray(4),
                    3,
                    TIGHT,
                    OK,
                    ctx,
                )?;

                if fstr::eq(&save.VECDEF, SINDEF) {
                    //
                    // --- Case: ------------------------------------------------------
                    //

                    // Given the inputs above, make sure we do not find
                    //    the state if we reverse the ray's direction.'
                    //
                    spicelib::PREFIX(b"Earth non-intersection case:", 1, &mut save.TITLE);
                    testutil::TCASE(&save.TITLE, ctx)?;

                    spicelib::VMINUS(save.DVEC.as_slice(), save.NEGVEC.as_slice_mut());
                    spicelib::VEQU(save.NEGVEC.as_slice(), save.DVEC.as_slice_mut());

                    spicelib::ZZGFCOST(
                        &save.VECDEF,
                        &save.METHOD,
                        save.TRGID,
                        save.ET,
                        &save.REF,
                        &save.ABCORR,
                        save.OBSID,
                        &save.DREF,
                        save.DCTR,
                        save.DVEC.as_slice(),
                        save.RADII.as_slice(),
                        save.STATE.as_slice_mut(),
                        &mut save.FOUND,
                        ctx,
                    )?;

                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // By construction, the state should NOT be found.
                    //
                    testutil::CHCKSL(b"ZZGFSSIN found (1)", save.FOUND, false, OK, ctx)?;
                }
            }
            //
            // End of cases for the current epoch.
            //
        }
        //
        // End of cases for the current aberration correction.
        //
    }
    //
    // End of cases for the current vector definition.
    //

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
