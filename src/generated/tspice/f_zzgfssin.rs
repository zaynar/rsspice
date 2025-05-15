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
const PCK: &[u8] = b"zzgfssob.tpc";
const SPK: &[u8] = b"zzgfssob.bsp";
const LOOSE: f64 = 0.001;
const MEDABS: f64 = 0.001;
const TIGHT: f64 = 0.00001;
const TDELTA: f64 = 10.0;
const BDNMLN: i32 = 36;
const LNSIZE: i32 = 200;
const FRNMLN: i32 = 32;
const TIMLEN: i32 = 50;
const NCORR: i32 = 9;
const NREF: i32 = 4;
const NSAMP: i32 = 10;

struct SaveVars {
    ABCORR: Vec<u8>,
    CORR: ActualCharArray,
    DREF: Vec<u8>,
    EFRAME: ActualCharArray,
    FIXREF: Vec<u8>,
    JFRAME: ActualCharArray,
    OBSRVR: Vec<u8>,
    TARGET: Vec<u8>,
    TIMSTR: Vec<u8>,
    TITLE: Vec<u8>,
    METHOD: Vec<u8>,
    DELTA: f64,
    DVEC: StackArray<f64, 3>,
    ET: f64,
    ET0: f64,
    LT: f64,
    NEGVEC: StackArray<f64, 3>,
    RADII: StackArray<f64, 3>,
    SPNT: StackArray2D<f64, 6>,
    SRFVEC: StackArray<f64, 3>,
    STATE: StackArray<f64, 6>,
    T: f64,
    TOL: f64,
    TRGEPC: f64,
    XPNT: StackArray<f64, 3>,
    XVEL: StackArray<f64, 3>,
    DCTR: i32,
    FRCLID: i32,
    FRCLSS: i32,
    FRCODE: i32,
    HANDLE: i32,
    N: i32,
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
        let mut FIXREF = vec![b' '; FRNMLN as usize];
        let mut JFRAME = ActualCharArray::new(FRNMLN, 1..=NREF);
        let mut OBSRVR = vec![b' '; BDNMLN as usize];
        let mut TARGET = vec![b' '; BDNMLN as usize];
        let mut TIMSTR = vec![b' '; TIMLEN as usize];
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut METHOD = vec![b' '; LNSIZE as usize];
        let mut DELTA: f64 = 0.0;
        let mut DVEC = StackArray::<f64, 3>::new(1..=3);
        let mut ET: f64 = 0.0;
        let mut ET0: f64 = 0.0;
        let mut LT: f64 = 0.0;
        let mut NEGVEC = StackArray::<f64, 3>::new(1..=3);
        let mut RADII = StackArray::<f64, 3>::new(1..=3);
        let mut SPNT = StackArray2D::<f64, 6>::new(1..=3, 1..=2);
        let mut SRFVEC = StackArray::<f64, 3>::new(1..=3);
        let mut STATE = StackArray::<f64, 6>::new(1..=6);
        let mut T: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut TRGEPC: f64 = 0.0;
        let mut XPNT = StackArray::<f64, 3>::new(1..=3);
        let mut XVEL = StackArray::<f64, 3>::new(1..=3);
        let mut DCTR: i32 = 0;
        let mut FRCLID: i32 = 0;
        let mut FRCLSS: i32 = 0;
        let mut FRCODE: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut N: i32 = 0;
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

            let mut clist = [
                Val::C(b"J2000"),
                Val::C(b"IAU_JUPITER"),
                Val::C(b"ECLIPJ2000"),
                Val::C(b"IAU_IO"),
            ]
            .into_iter();
            JFRAME
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            ABCORR,
            CORR,
            DREF,
            EFRAME,
            FIXREF,
            JFRAME,
            OBSRVR,
            TARGET,
            TIMSTR,
            TITLE,
            METHOD,
            DELTA,
            DVEC,
            ET,
            ET0,
            LT,
            NEGVEC,
            RADII,
            SPNT,
            SRFVEC,
            STATE,
            T,
            TOL,
            TRGEPC,
            XPNT,
            XVEL,
            DCTR,
            FRCLID,
            FRCLSS,
            FRCODE,
            HANDLE,
            N,
            OBSID,
            TRGID,
            ATTBLK,
            FND,
            FOUND,
        }
    }
}

//$Procedure      F_ZZGFSSIN ( Test surface intercept state computation )
pub fn F_ZZGFSSIN(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    // Time delta used for discrete derivatives. The 10-second
    // value used here was found by trial and error: it yields
    // better approximations than does the "standard" 1-second
    // value.
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
    testutil::TOPEN(b"F_ZZGFSSIN", ctx)?;

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
    testutil::TCASE(b"Bad aberration correction", ctx)?;

    fstr::assign(&mut save.ABCORR, b"None.");
    fstr::assign(&mut save.METHOD, b"ellipsoid");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.FIXREF, b"IAU_EARTH");
    fstr::assign(&mut save.DREF, &save.FIXREF);
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

    spicelib::BODN2C(&save.TARGET, &mut save.TRGID, &mut save.FOUND, ctx)?;
    spicelib::BODN2C(&save.OBSRVR, &mut save.OBSID, &mut save.FOUND, ctx)?;

    spicelib::ZZGFSSIN(
        &save.METHOD,
        save.TRGID,
        save.ET,
        &save.FIXREF,
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
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.FIXREF, b"IAU_EARTH");
    fstr::assign(&mut save.DREF, &save.FIXREF);
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

    spicelib::BODN2C(&save.TARGET, &mut save.TRGID, &mut save.FOUND, ctx)?;
    spicelib::BODN2C(&save.OBSRVR, &mut save.OBSID, &mut save.FOUND, ctx)?;

    spicelib::ZZGFSSIN(
        &save.METHOD,
        save.TRGID,
        save.ET,
        &save.FIXREF,
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

    fstr::assign(&mut save.ABCORR, b"None");
    fstr::assign(&mut save.METHOD, b"ellipsoid");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.FIXREF, b"ITRF93");
    fstr::assign(&mut save.DREF, &save.FIXREF);
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

    spicelib::BODN2C(&save.TARGET, &mut save.TRGID, &mut save.FOUND, ctx)?;
    spicelib::BODN2C(&save.OBSRVR, &mut save.OBSID, &mut save.FOUND, ctx)?;

    spicelib::ZZGFSSIN(
        &save.METHOD,
        save.TRGID,
        save.ET,
        &save.FIXREF,
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

    fstr::assign(&mut save.ABCORR, b"None");
    fstr::assign(&mut save.METHOD, b"ellipsoid");
    fstr::assign(&mut save.TARGET, b"Mars");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.FIXREF, b"IAU_EARTH");
    fstr::assign(&mut save.DREF, &save.FIXREF);
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

    spicelib::BODN2C(&save.TARGET, &mut save.TRGID, &mut save.FOUND, ctx)?;
    spicelib::BODN2C(&save.OBSRVR, &mut save.OBSID, &mut save.FOUND, ctx)?;

    spicelib::ZZGFSSIN(
        &save.METHOD,
        save.TRGID,
        save.ET,
        &save.FIXREF,
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

    fstr::assign(&mut save.ABCORR, b"None");
    fstr::assign(&mut save.METHOD, b"ellipsoid");
    fstr::assign(&mut save.TARGET, b"GASPRA");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.FIXREF, b"IAU_GASPRA");
    fstr::assign(&mut save.DREF, &save.FIXREF);
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

    spicelib::BODN2C(&save.TARGET, &mut save.TRGID, &mut save.FOUND, ctx)?;
    spicelib::BODN2C(&save.OBSRVR, &mut save.OBSID, &mut save.FOUND, ctx)?;

    spicelib::ZZGFSSIN(
        &save.METHOD,
        save.TRGID,
        save.ET,
        &save.FIXREF,
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

    fstr::assign(&mut save.ABCORR, b"None");
    fstr::assign(&mut save.METHOD, b"ellipsoid");
    fstr::assign(&mut save.TARGET, b"MOON");
    fstr::assign(&mut save.OBSRVR, b"GASPRA");
    fstr::assign(&mut save.FIXREF, b"IAU_MOON");
    fstr::assign(&mut save.DREF, &save.FIXREF);
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

    spicelib::BODN2C(&save.TARGET, &mut save.TRGID, &mut save.FOUND, ctx)?;
    spicelib::BODN2C(&save.OBSRVR, &mut save.OBSID, &mut save.FOUND, ctx)?;

    spicelib::ZZGFSSIN(
        &save.METHOD,
        save.TRGID,
        save.ET,
        &save.FIXREF,
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

    fstr::assign(&mut save.ABCORR, b"LT");
    fstr::assign(&mut save.METHOD, b"ellipsoid");
    fstr::assign(&mut save.TARGET, b"MOON");
    fstr::assign(&mut save.OBSRVR, b"Earth");
    fstr::assign(&mut save.FIXREF, b"IAU_MOON");
    fstr::assign(&mut save.DREF, b"IAU_GASPRA");
    save.DCTR = 9511010;
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

    spicelib::BODN2C(&save.TARGET, &mut save.TRGID, &mut save.FOUND, ctx)?;
    spicelib::BODN2C(&save.OBSRVR, &mut save.OBSID, &mut save.FOUND, ctx)?;

    spicelib::ZZGFSSIN(
        &save.METHOD,
        save.TRGID,
        save.ET,
        &save.FIXREF,
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

    fstr::assign(&mut save.ABCORR, b"LT");
    fstr::assign(&mut save.METHOD, b"ellipsoid");
    fstr::assign(&mut save.TARGET, b"MOON");
    fstr::assign(&mut save.OBSRVR, b"Earth");
    fstr::assign(&mut save.FIXREF, b"IAU_MOON");
    fstr::assign(&mut save.DREF, b"ITRF93");
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

    spicelib::BODN2C(&save.TARGET, &mut save.TRGID, &mut save.FOUND, ctx)?;
    spicelib::BODN2C(&save.OBSRVR, &mut save.OBSID, &mut save.FOUND, ctx)?;

    spicelib::ZZGFSSIN(
        &save.METHOD,
        save.TRGID,
        save.ET,
        &save.FIXREF,
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

    //
    // We're going to loop over all aberration corrections and
    // ray direction frames.
    //

    for NC in 1..=NCORR {
        for NF in 1..=NREF {
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
                //
                fstr::assign(&mut save.ABCORR, save.CORR.get(NC));
                fstr::assign(&mut save.METHOD, b"Ellipsoid");
                fstr::assign(&mut save.DREF, save.EFRAME.get(NF));
                fstr::assign(&mut save.TARGET, b"EARTH");
                fstr::assign(&mut save.OBSRVR, b"MOON");
                fstr::assign(&mut save.FIXREF, b"IAU_EARTH");

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

                fstr::assign(&mut save.TITLE, b"#-# intercept. #; #; DREF = #; #");

                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.OBSRVR, &mut save.TITLE);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.METHOD, &mut save.TITLE);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.DREF, &mut save.TITLE);

                save.ET = (save.ET0 + (((NS - 1) as f64) * save.DELTA));
                spicelib::ETCAL(save.ET, &mut save.TIMSTR, ctx);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TIMSTR, &mut save.TITLE);

                testutil::TCASE(&save.TITLE, ctx)?;

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

                spicelib::ZZGFSSIN(
                    &save.METHOD,
                    save.TRGID,
                    save.ET,
                    &save.FIXREF,
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
                // By construction, the state should be found.
                //
                testutil::CHCKSL(b"ZZGFSSIN found", save.FOUND, true, OK, ctx)?;

                // WRITE (*,*) 'STATE = ', STATE

                //
                // Use SRFVEC and FIXREF as our ray direction and frame.
                //
                // Estimate the surface intercept point velocity
                // numerically.
                //
                for I in 1..=2 {
                    save.T = (save.ET + ((((2 * I) - 3) as f64) * TDELTA));

                    //
                    // DVEC is constant in frame DREF, so we don't need
                    // to re-compute it.
                    //
                    spicelib::SINCPT(
                        &save.METHOD,
                        &save.TARGET,
                        save.T,
                        &save.FIXREF,
                        &save.ABCORR,
                        &save.OBSRVR,
                        &save.DREF,
                        save.DVEC.as_slice(),
                        save.SPNT.subarray_mut([1, I]),
                        &mut save.TRGEPC,
                        save.SRFVEC.as_slice_mut(),
                        &mut save.FOUND,
                        ctx,
                    )?;

                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::CHCKSL(b"ZZGFSSIN found (0)", save.FOUND, true, OK, ctx)?;
                }

                spicelib::QDERIV(
                    3,
                    save.SPNT.subarray([1, 1]),
                    save.SPNT.subarray([1, 2]),
                    TDELTA,
                    save.XVEL.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Check the velocity from ZZGFSSIN. We expect an error of
                // less than 1 m/s, except when the frame is IAU_EARTH,
                // because the velocity of the intercept is quite high in
                // that frame.
                //
                if fstr::ne(&save.DREF, b"IAU_EARTH") {
                    save.TOL = MEDABS;
                } else {
                    save.TOL = LOOSE;
                }

                testutil::CHCKAD(
                    b"Intercept vel (0)",
                    save.STATE.subarray(4),
                    b"~~",
                    save.XVEL.as_slice(),
                    3,
                    save.TOL,
                    OK,
                    ctx,
                )?;

                // IF ( .NOT. OK ) THEN
                //    WRITE (*,*) '>>>>>>>>>>>>>Speed = ', VNORM(XVEL)
                // END IF

                //
                // Check the surface intercept.
                //
                spicelib::SINCPT(
                    &save.METHOD,
                    &save.TARGET,
                    save.ET,
                    &save.FIXREF,
                    &save.ABCORR,
                    &save.OBSRVR,
                    &save.DREF,
                    save.DVEC.as_slice(),
                    save.XPNT.as_slice_mut(),
                    &mut save.TRGEPC,
                    save.SRFVEC.as_slice_mut(),
                    &mut save.FOUND,
                    ctx,
                )?;

                //
                // The tolerance we use depends on the aberration
                // correction.
                //
                spicelib::ZZPRSCOR(&save.ABCORR, save.ATTBLK.as_slice_mut(), ctx)?;

                if save.ATTBLK[CNVIDX] {
                    save.TOL = MEDABS;
                } else if save.ATTBLK[LTIDX] {
                    save.TOL = LOOSE;
                } else {
                    save.TOL = TIGHT;
                }

                testutil::CHCKAD(
                    b"Surface intercept point (0)",
                    save.STATE.as_slice(),
                    b"~~",
                    save.XPNT.as_slice(),
                    3,
                    save.TOL,
                    OK,
                    ctx,
                )?;

                //
                // --- Case: ------------------------------------------------------
                //

                //
                // Given the inputs above, make sure we do not find
                // the state if we reverse the ray's direction.'
                //
                spicelib::PREFIX(b"Earth non-intersection case:", 1, &mut save.TITLE);
                testutil::TCASE(&save.TITLE, ctx)?;

                spicelib::VMINUS(save.DVEC.as_slice(), save.NEGVEC.as_slice_mut());
                spicelib::VEQU(save.NEGVEC.as_slice(), save.DVEC.as_slice_mut());

                spicelib::ZZGFSSIN(
                    &save.METHOD,
                    save.TRGID,
                    save.ET,
                    &save.FIXREF,
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

                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut save.ABCORR, save.CORR.get(NC));
                fstr::assign(&mut save.METHOD, b"ellipsoid");
                fstr::assign(&mut save.TARGET, b"JUPITER");
                fstr::assign(&mut save.OBSRVR, b"IO");
                fstr::assign(&mut save.FIXREF, b"IAU_JUPITER");
                fstr::assign(&mut save.DREF, save.JFRAME.get(NF));

                fstr::assign(&mut save.TITLE, b"#-# intercept. #; #; DREF = #; #");

                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.OBSRVR, &mut save.TITLE);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.METHOD, &mut save.TITLE);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.DREF, &mut save.TITLE);

                spicelib::ETCAL(save.ET, &mut save.TIMSTR, ctx);

                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TIMSTR, &mut save.TITLE);

                testutil::TCASE(&save.TITLE, ctx)?;

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

                spicelib::ZZGFSSIN(
                    &save.METHOD,
                    save.TRGID,
                    save.ET,
                    &save.FIXREF,
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
                // By construction, the state should be found.
                //
                testutil::CHCKSL(b"ZZGFSSIN found", save.FOUND, true, OK, ctx)?;

                // WRITE (*,*) 'DREF, STATE = ', DREF, STATE

                //
                // Use SRFVEC and FIXREF as our ray direction and frame.
                //
                // Estimate the surface intercept point velocity
                // numerically.
                //
                for I in 1..=2 {
                    save.T = (save.ET + ((((2 * I) - 3) as f64) * TDELTA));

                    //
                    // DVEC is constant in frame DREF, so we don't need
                    // to re-compute it.
                    //
                    spicelib::SINCPT(
                        &save.METHOD,
                        &save.TARGET,
                        save.T,
                        &save.FIXREF,
                        &save.ABCORR,
                        &save.OBSRVR,
                        &save.DREF,
                        save.DVEC.as_slice(),
                        save.SPNT.subarray_mut([1, I]),
                        &mut save.TRGEPC,
                        save.SRFVEC.as_slice_mut(),
                        &mut save.FOUND,
                        ctx,
                    )?;

                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::CHCKSL(b"ZZGFSSIN found (2)", save.FOUND, true, OK, ctx)?;
                }

                spicelib::QDERIV(
                    3,
                    save.SPNT.subarray([1, 1]),
                    save.SPNT.subarray([1, 2]),
                    TDELTA,
                    save.XVEL.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Check the velocity from ZZGFSSIN. We expect an error of
                // less than 1 m/s.
                //
                //
                // The tolerance we use depends on the aberration
                // correction.
                //
                spicelib::ZZPRSCOR(&save.ABCORR, save.ATTBLK.as_slice_mut(), ctx)?;

                if save.ATTBLK[CNVIDX] {
                    if fstr::eq(&save.FIXREF, b"IAU_JUPITER") {
                        save.TOL = LOOSE;
                    } else {
                        save.TOL = MEDABS;
                    }
                } else if save.ATTBLK[LTIDX] {
                    save.TOL = LOOSE;
                } else {
                    save.TOL = MEDABS;
                }

                testutil::CHCKAD(
                    b"Intercept vel (1)",
                    save.STATE.subarray(4),
                    b"~~",
                    save.XVEL.as_slice(),
                    3,
                    save.TOL,
                    OK,
                    ctx,
                )?;
                // IF ( .NOT. OK ) THEN
                //    WRITE (*,*) '>>>>>>>>>>>>>Speed = ', VNORM(XVEL)
                // END IF

                //
                // Check the surface intercept.
                //
                spicelib::SINCPT(
                    &save.METHOD,
                    &save.TARGET,
                    save.ET,
                    &save.FIXREF,
                    &save.ABCORR,
                    &save.OBSRVR,
                    &save.DREF,
                    save.DVEC.as_slice(),
                    save.XPNT.as_slice_mut(),
                    &mut save.TRGEPC,
                    save.SRFVEC.as_slice_mut(),
                    &mut save.FOUND,
                    ctx,
                )?;

                if save.ATTBLK[CNVIDX] {
                    save.TOL = MEDABS;
                } else if save.ATTBLK[LTIDX] {
                    save.TOL = LOOSE;
                } else {
                    save.TOL = TIGHT;
                }

                testutil::CHCKAD(
                    b"Surface intercept point (1)",
                    save.STATE.as_slice(),
                    b"~~",
                    save.XPNT.as_slice(),
                    3,
                    save.TOL,
                    OK,
                    ctx,
                )?;

                //
                // --- Case: ------------------------------------------------------
                //

                //
                // Given the inputs above, make sure we do not find
                // the state if we reverse the ray's direction.'
                //
                spicelib::PREFIX(b"Non-intersection:", 1, &mut save.TITLE);
                testutil::TCASE(&save.TITLE, ctx)?;

                spicelib::VMINUS(save.DVEC.as_slice(), save.NEGVEC.as_slice_mut());
                spicelib::VEQU(save.NEGVEC.as_slice(), save.DVEC.as_slice_mut());

                spicelib::ZZGFSSIN(
                    &save.METHOD,
                    save.TRGID,
                    save.ET,
                    &save.FIXREF,
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
                testutil::CHCKSL(b"ZZGFSSIN found (3)", save.FOUND, false, OK, ctx)?;
            }
            //
            // End of cases for the current epoch.
            //
        }
        //
        // End of cases for the current method.
        //
    }
    //
    // End of cases for the current aberration correction.
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
