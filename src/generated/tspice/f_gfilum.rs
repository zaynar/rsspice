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
const PCK: &[u8] = b"nat.tpc";
const PCK2: &[u8] = b"generic.tpc";
const SPK1: &[u8] = b"nat.bsp";
const SPK2: &[u8] = b"generic.bsp";
const OCTSPK: &[u8] = b"octl_test.bsp";
const OCTL: &[u8] = b"OCTL";
const ANGTOL: f64 = 0.000000001;
const TIMTOL: f64 = 0.000001;
const LOOSE: f64 = 0.00001;
const LNSIZE: i32 = 80;
const TIMLEN: i32 = 50;
const LBCELL: i32 = -5;
const MAXIVL: i32 = 100;
const MAXWIN: i32 = (2 * MAXIVL);
const BDNMLN: i32 = 36;
const FRNMLN: i32 = 32;
const OPLEN: i32 = 6;
const NREL: i32 = 9;
const NCORR: i32 = 5;
const TYPLEN: i32 = 25;
const TXTSIZ: i32 = 25;
const MSGLEN: i32 = 800;
const SYSLEN: i32 = 25;
const CRDLEN: i32 = 25;

struct SaveVars {
    ABCORR: Vec<u8>,
    ANGTYP: Vec<u8>,
    COORD: Vec<u8>,
    CORRS: ActualCharArray,
    CORSYS: Vec<u8>,
    FIXREF: Vec<u8>,
    ILLUM: Vec<u8>,
    METHOD: Vec<u8>,
    OBSRVR: Vec<u8>,
    RELATE: Vec<u8>,
    RELOPS: ActualCharArray,
    SRFREF: Vec<u8>,
    TARGET: Vec<u8>,
    TITLE: Vec<u8>,
    TIME0: Vec<u8>,
    TXT: ActualCharArray,
    ADJ: StackArray<f64, 9>,
    ADJUST: f64,
    ALT: f64,
    CNFINE: StackArray<f64, 206>,
    BEG: f64,
    END: f64,
    EMISSN: f64,
    ET: f64,
    ET0: f64,
    ET1: f64,
    FINISH: f64,
    FIRST: f64,
    LAST: f64,
    LAT: f64,
    LEFT: f64,
    LON: f64,
    LT: f64,
    MODCFN: StackArray<f64, 206>,
    MODRES: StackArray<f64, 206>,
    NP: StackArray<f64, 3>,
    PHASE: f64,
    POS: StackArray<f64, 3>,
    RADII: StackArray<f64, 3>,
    REFVAL: f64,
    RESULT: StackArray<f64, 206>,
    RIGHT: f64,
    SOLAR: f64,
    SPOINT: StackArray<f64, 3>,
    SRFVEC: StackArray<f64, 3>,
    START: f64,
    STATES: StackArray2D<f64, 12>,
    STEP: f64,
    TRGEPC: f64,
    VALS: StackArray<f64, 9>,
    XRSULT: StackArray<f64, 206>,
    XTIME: f64,
    WORK: ActualArray2D<f64>,
    COUNT: i32,
    HANDLE: i32,
    HAN1: i32,
    HAN2: i32,
    N: i32,
    OCTID: i32,
    XN: i32,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ABCORR = vec![b' '; CORLEN as usize];
        let mut ANGTYP = vec![b' '; TYPLEN as usize];
        let mut COORD = vec![b' '; CRDLEN as usize];
        let mut CORRS = ActualCharArray::new(CORLEN, 1..=NCORR);
        let mut CORSYS = vec![b' '; SYSLEN as usize];
        let mut FIXREF = vec![b' '; FRNMLN as usize];
        let mut ILLUM = vec![b' '; BDNMLN as usize];
        let mut METHOD = vec![b' '; LNSIZE as usize];
        let mut OBSRVR = vec![b' '; BDNMLN as usize];
        let mut RELATE = vec![b' '; OPLEN as usize];
        let mut RELOPS = ActualCharArray::new(OPLEN, 1..=NREL);
        let mut SRFREF = vec![b' '; FRNMLN as usize];
        let mut TARGET = vec![b' '; BDNMLN as usize];
        let mut TITLE = vec![b' '; MSGLEN as usize];
        let mut TIME0 = vec![b' '; TIMLEN as usize];
        let mut TXT = ActualCharArray::new(LNSIZE, 1..=TXTSIZ);
        let mut ADJ = StackArray::<f64, 9>::new(1..=NREL);
        let mut ADJUST: f64 = 0.0;
        let mut ALT: f64 = 0.0;
        let mut CNFINE = StackArray::<f64, 206>::new(LBCELL..=MAXWIN);
        let mut BEG: f64 = 0.0;
        let mut END: f64 = 0.0;
        let mut EMISSN: f64 = 0.0;
        let mut ET: f64 = 0.0;
        let mut ET0: f64 = 0.0;
        let mut ET1: f64 = 0.0;
        let mut FINISH: f64 = 0.0;
        let mut FIRST: f64 = 0.0;
        let mut LAST: f64 = 0.0;
        let mut LAT: f64 = 0.0;
        let mut LEFT: f64 = 0.0;
        let mut LON: f64 = 0.0;
        let mut LT: f64 = 0.0;
        let mut MODCFN = StackArray::<f64, 206>::new(LBCELL..=MAXWIN);
        let mut MODRES = StackArray::<f64, 206>::new(LBCELL..=MAXWIN);
        let mut NP = StackArray::<f64, 3>::new(1..=3);
        let mut PHASE: f64 = 0.0;
        let mut POS = StackArray::<f64, 3>::new(1..=3);
        let mut RADII = StackArray::<f64, 3>::new(1..=3);
        let mut REFVAL: f64 = 0.0;
        let mut RESULT = StackArray::<f64, 206>::new(LBCELL..=MAXWIN);
        let mut RIGHT: f64 = 0.0;
        let mut SOLAR: f64 = 0.0;
        let mut SPOINT = StackArray::<f64, 3>::new(1..=3);
        let mut SRFVEC = StackArray::<f64, 3>::new(1..=3);
        let mut START: f64 = 0.0;
        let mut STATES = StackArray2D::<f64, 12>::new(1..=6, 1..=2);
        let mut STEP: f64 = 0.0;
        let mut TRGEPC: f64 = 0.0;
        let mut VALS = StackArray::<f64, 9>::new(1..=NREL);
        let mut XRSULT = StackArray::<f64, 206>::new(LBCELL..=MAXWIN);
        let mut XTIME: f64 = 0.0;
        let mut WORK = ActualArray2D::<f64>::new(LBCELL..=MAXWIN, 1..=NWMAX);
        let mut COUNT: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut HAN1: i32 = 0;
        let mut HAN2: i32 = 0;
        let mut N: i32 = 0;
        let mut OCTID: i32 = 0;
        let mut XN: i32 = 0;
        let mut FOUND: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"NONE"),
                Val::C(b"LT"),
                Val::C(b"CN"),
                Val::C(b"LT+S"),
                Val::C(b"CN+S"),
            ]
            .into_iter();
            CORRS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"="),
                Val::C(b"<"),
                Val::C(b">"),
                Val::C(b"LOCMIN"),
                Val::C(b"LOCMAX"),
                Val::C(b"ABSMIN"),
                Val::C(b"ABSMIN"),
                Val::C(b"ABSMAX"),
                Val::C(b"ABSMAX"),
            ]
            .into_iter();
            RELOPS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(65.0), Val::D(70.0), Val::D(60.0)]
                .into_iter()
                .chain(std::iter::repeat_n(Val::D(0.0), 6 as usize))
                .chain([]);

            VALS.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::D(0.0), 5 as usize))
                .chain([Val::D(0.0), Val::D(0.05), Val::D(0.0), Val::D(0.05)]);

            ADJ.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            ABCORR,
            ANGTYP,
            COORD,
            CORRS,
            CORSYS,
            FIXREF,
            ILLUM,
            METHOD,
            OBSRVR,
            RELATE,
            RELOPS,
            SRFREF,
            TARGET,
            TITLE,
            TIME0,
            TXT,
            ADJ,
            ADJUST,
            ALT,
            CNFINE,
            BEG,
            END,
            EMISSN,
            ET,
            ET0,
            ET1,
            FINISH,
            FIRST,
            LAST,
            LAT,
            LEFT,
            LON,
            LT,
            MODCFN,
            MODRES,
            NP,
            PHASE,
            POS,
            RADII,
            REFVAL,
            RESULT,
            RIGHT,
            SOLAR,
            SPOINT,
            SRFVEC,
            START,
            STATES,
            STEP,
            TRGEPC,
            VALS,
            XRSULT,
            XTIME,
            WORK,
            COUNT,
            HANDLE,
            HAN1,
            HAN2,
            N,
            OCTID,
            XN,
            FOUND,
        }
    }
}

//$Procedure      F_GFILUM ( GFILUM family tests )
pub fn F_GFILUM(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    // INTEGER               NAMLEN
    // PARAMETER           ( NAMLEN = 32 )

    // INTEGER               NREF
    // PARAMETER           ( NREF = 3 )

    //
    // Local variables
    //

    //
    // The workspace array has to handle the largest workspace
    // we'll use, which is that required by GFPOSC.
    //

    //
    // Saved everything.
    //

    //
    // Initial values
    //

    //
    // Note: the absolute extremum operators are repeated because
    // they're used with both zero and non-zero adjustment values.
    //

    //
    // Reference values to be used with binary operators:
    // These values are in degrees; they must be converted to
    // radians before use.
    //

    //
    // Ajustment values to be used with absolute extremum operators:
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_GFILUM", ctx)?;

    testutil::TCASE(b"Setup: create and load SPK, PCK, LSK files.", ctx)?;

    testutil::NATSPK(SPK1, true, &mut save.HAN1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTSPK(SPK2, true, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::NATPCK(PCK, true, false, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTPCK(PCK2, true, false, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set up a confinement window. Initialize this and
    // the result window.
    //
    fstr::assign(&mut save.TIME0, b"2000 JAN 1  00:00:00 TDB");

    spicelib::STR2ET(&save.TIME0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;

    //
    // Create a confinement window with 4 intervals.
    //
    for I in 1..=4 {
        save.LEFT = ((save.ET0 + (((I - 1) as f64) * spicelib::SPD())) + 3600.0);
        save.RIGHT = (save.LEFT + (3600.0 * 22 as f64));

        spicelib::WNINSD(save.LEFT, save.RIGHT, save.CNFINE.as_slice_mut(), ctx)?;
    }

    //*********************************************************************
    //*
    //*    Error cases
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Non-positive step size", ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.METHOD, b"Ellipsoid");
    fstr::assign(&mut save.ANGTYP, b"INCIDENCE");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.ILLUM, b"SUN");
    fstr::assign(&mut save.FIXREF, b"IAU_EARTH");
    fstr::assign(&mut save.ABCORR, b"CN+S");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.RELATE, b"LOCMAX");
    save.REFVAL = 0.0;
    save.ADJUST = 0.0;
    save.STEP = 1000.0;

    save.LON = (100.0 * spicelib::RPD(ctx));
    save.LAT = (30.0 * spicelib::RPD(ctx));

    spicelib::SRFREC(399, save.LON, save.LAT, save.SPOINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.STEP = 0.0;

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        &save.ILLUM,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDSTEP)", OK, ctx)?;

    save.STEP = -1.0;

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        &save.ILLUM,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDSTEP)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Workspace window size too small.", ctx)?;

    spicelib::SSIZED(1, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.METHOD, b"Ellipsoid");
    fstr::assign(&mut save.ANGTYP, b"INCIDENCE");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.ILLUM, b"SUN");
    fstr::assign(&mut save.FIXREF, b"IAU_EARTH");
    fstr::assign(&mut save.ABCORR, b"CN+S");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.RELATE, b"LOCMAX");
    save.REFVAL = 0.0;
    save.ADJUST = 0.0;
    save.STEP = 1000.0;

    save.LON = (100.0 * spicelib::RPD(ctx));
    save.LAT = (30.0 * spicelib::RPD(ctx));

    spicelib::SRFREC(399, save.LON, save.LAT, save.SPOINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        &save.ILLUM,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        1,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDIMENSION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Result window too small (detected before search)", ctx)?;

    spicelib::SSIZED(1, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.METHOD, b"Ellipsoid");
    fstr::assign(&mut save.ANGTYP, b"INCIDENCE");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.ILLUM, b"SUN");
    fstr::assign(&mut save.FIXREF, b"IAU_EARTH");
    fstr::assign(&mut save.ABCORR, b"CN+S");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.RELATE, b"LOCMAX");
    save.REFVAL = 0.0;
    save.ADJUST = 0.0;
    save.STEP = 1000.0;

    save.LON = (100.0 * spicelib::RPD(ctx));
    save.LAT = (30.0 * spicelib::RPD(ctx));

    spicelib::SRFREC(399, save.LON, save.LAT, save.SPOINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        &save.ILLUM,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDIMENSION)", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Result window too small (detected during search)", ctx)?;

    fstr::assign(&mut save.METHOD, b"Ellipsoid");
    fstr::assign(&mut save.ANGTYP, b"INCIDENCE");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.ILLUM, b"SUN");
    fstr::assign(&mut save.FIXREF, b"IAU_EARTH");
    fstr::assign(&mut save.ABCORR, b"CN+S");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.RELATE, b"LOCMAX");
    save.REFVAL = 0.0;
    save.ADJUST = 0.0;
    save.STEP = 1000.0;

    save.LON = (100.0 * spicelib::RPD(ctx));
    save.LAT = (30.0 * spicelib::RPD(ctx));

    spicelib::SRFREC(399, save.LON, save.LAT, save.SPOINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 JAN 1 TDB", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 JAN 5 TDB", &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.STEP = 300.0;

    spicelib::SSIZED(2, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        &save.ILLUM,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(OUTOFROOM)", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Unrecognized value of FIXREF.", ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        &save.ILLUM,
        b"XYZ",
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(UNKNOWNFRAME)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"FIXREF is not centered on the target body.", ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        &save.ILLUM,
        b"IAU_MARS",
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDFRAME)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Bad computation method.", ctx)?;

    save.STEP = 1.0;

    fstr::assign(&mut save.ANGTYP, b"EMISSION");

    fstr::assign(&mut save.METHOD, b"DSK");

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        &save.ILLUM,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDMETHOD)", OK, ctx)?;

    fstr::assign(&mut save.METHOD, b"ELLIPSOID");

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Bad illumination angle type", ctx)?;

    save.STEP = 1.0;

    fstr::assign(&mut save.ANGTYP, b"XYZ");

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        &save.ILLUM,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Bad relational operator", ctx)?;

    save.STEP = 1.0;

    fstr::assign(&mut save.ANGTYP, b"INCIDENCE");

    fstr::assign(&mut save.RELATE, b">=");

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        &save.ILLUM,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTRECOGNIZED)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Bad aberration correction values", ctx)?;

    save.STEP = 1.0;

    fstr::assign(&mut save.RELATE, b"LOCMAX");

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        &save.ILLUM,
        &save.FIXREF,
        b"S",
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        &save.ILLUM,
        &save.FIXREF,
        b"XS",
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        &save.ILLUM,
        &save.FIXREF,
        b"RLT",
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        &save.ILLUM,
        &save.FIXREF,
        b"XRLT",
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        &save.ILLUM,
        &save.FIXREF,
        b"Z",
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Transmission aberration correction values.", ctx)?;

    save.STEP = 1.0;

    fstr::assign(&mut save.RELATE, b"LOCMAX");

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        &save.ILLUM,
        &save.FIXREF,
        b"XLT",
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        &save.ILLUM,
        &save.FIXREF,
        b"XCN",
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        &save.ILLUM,
        &save.FIXREF,
        b"XLT+S",
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        &save.ILLUM,
        &save.FIXREF,
        b"XCN+S",
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Bad value of ADJUST.", ctx)?;

    save.ADJUST = -1.0;

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        &save.ILLUM,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Unrecognized target, observer, or illumination source.",
        ctx,
    )?;

    save.ADJUST = 0.0;

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        b"X",
        &save.ILLUM,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        b"X",
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        &save.ILLUM,
        &save.FIXREF,
        &save.ABCORR,
        b"X",
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Target and observer are identical.", ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        &save.ILLUM,
        &save.FIXREF,
        &save.ABCORR,
        &save.TARGET,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Target and illumination source are identical.", ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        &save.TARGET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"No SPK data for observer", ctx)?;

    save.STEP = 1.0;

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        &save.ILLUM,
        &save.FIXREF,
        &save.ABCORR,
        b"GASPRA",
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"No SPK data for target", ctx)?;

    save.STEP = 1.0;

    fstr::assign(&mut save.FIXREF, b"IAU_GASPRA");

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        b"GASPRA",
        &save.ILLUM,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"No SPK data for illumination source", ctx)?;

    save.STEP = 1.0;

    fstr::assign(&mut save.FIXREF, b"IAU_EARTH");

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        b"GASPRA",
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"No PCK orientation data for target.", ctx)?;

    save.STEP = 1000.0;

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        &save.ILLUM,
        b"ITRF93",
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(FRAMEDATANOTFOUND)", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Normal cases
    //*
    //*********************************************************************

    //*********************************************************************
    //*
    //*    Simple cases using Nat's solar system
    //*
    //*********************************************************************

    //
    // The cases below all involve finding times when local extrema are
    // attained. In each case we know the correct answers. All of these
    // tests are done with aberration corrections set to 'NONE'.
    //
    // Following these tests, a series of comparisons is performed using
    // results produced by alternate methods. The second set of tests is
    // comprehensive: those tests use all combinations of operators and
    // aberration corrections.
    //

    //
    //---- Case -------------------------------------------------------------
    //

    //
    // All expected event times are based on Nat's solar system.
    //
    testutil::TCASE(
        b"Local minimum of emission angle at north pole of ALPHA; observer is SUN; abcorr = NONE",
        ctx,
    )?;

    fstr::assign(&mut save.ANGTYP, b"EMISSION");
    fstr::assign(&mut save.RELATE, b"locmin");
    save.REFVAL = 0.0;
    save.ADJUST = 0.0;

    fstr::assign(&mut save.ABCORR, b"NONE");

    fstr::assign(&mut save.TARGET, b" ALPHA");
    fstr::assign(&mut save.FIXREF, b" ALPHAFIXED");
    fstr::assign(&mut save.OBSRVR, b" SUN");
    fstr::assign(&mut save.ILLUM, b" SUN");

    spicelib::BODVRD(
        b"ALPHA",
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPACK(0.0, 0.0, save.RADII[3], save.SPOINT.as_slice_mut());

    fstr::assign(&mut save.TIME0, b"2000 JAN 1 12:00:00 TDB");

    spicelib::STR2ET(&save.TIME0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Check sun position for debugging:
    // CALL SPKPOS ( OBSRVR, ET0, FIXREF, 'NONE', TARGET, POS, LT )
    // WRITE (*,*) 'Position of sun wrt alpha at J2000:'
    // WRITE (*,*) POS
    //

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;

    //
    // The period of ALPHA's orbit is 7 days. This can be derived
    // from the relative angular velocities of ALPHA and BETA
    // and the period of the occultation starts.
    //
    // A minimum of the emission angle should occur whenever ALPHA's
    // north pole points toward the sun. This should happen at the
    // J2000 epoch and every 7 days before or after.
    //
    // Create a confinement window with 4 intervals.
    //
    for I in 1..=4 {
        save.LEFT = ((save.ET0 + ((((I - 1) as f64) * spicelib::SPD()) * 7 as f64)) - 3600.0);
        save.RIGHT = (save.LEFT + ((2 as f64) * 3600.0));

        spicelib::WNINSD(save.LEFT, save.RIGHT, save.CNFINE.as_slice_mut(), ctx)?;
    }

    //
    // We can use a large step. The extrema should be 12 hours
    // apart.
    //
    save.STEP = ((10 as f64) * 3600.0);

    fstr::assign(&mut save.METHOD, b"Ellipsoid");

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        &save.ILLUM,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(
        b"RESULT cardinality",
        spicelib::WNCARD(save.RESULT.as_slice(), ctx)?,
        b"=",
        4,
        0,
        OK,
        ctx,
    )?;

    for I in 1..=spicelib::WNCARD(save.RESULT.as_slice(), ctx)? {
        spicelib::WNFETD(
            save.RESULT.as_slice(),
            I,
            &mut save.LEFT,
            &mut save.RIGHT,
            ctx,
        )?;
        //
        // Check event start time.
        //
        fstr::assign(&mut save.TITLE, b"Event # beg time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        save.XTIME = (save.ET0 + ((((I - 1) as f64) * spicelib::SPD()) * 7 as f64));

        testutil::CHCKSD(&save.TITLE, save.LEFT, b"~", save.XTIME, TIMTOL, OK, ctx)?;

        //
        // Check event end time; this should equal the begin time
        // since we're searching for an extremum.
        //
        fstr::assign(&mut save.TITLE, b"Event # end time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        testutil::CHCKSD(&save.TITLE, save.RIGHT, b"~", save.XTIME, TIMTOL, OK, ctx)?;

        //
        //    Dump event times for debugging:
        //
        //     CALL TIMOUT ( LEFT,
        // .                 'YYYY MON DD HR:MN:SC.###### ::TDB (TDB)',
        // .                 TIMSTR )
        //     WRITE (*,*) I, ' ',  TIMSTR
        //
        //     CALL TIMOUT ( RIGHT,
        // .                 'YYYY MON DD HR:MN:SC.###### ::TDB (TDB)',
        // .                 TIMSTR )
        //     WRITE (*,*) '  ', TIMSTR
    }

    //
    //---- Case -------------------------------------------------------------
    //

    //
    // All expected event times are based on Nat's solar system.
    //
    testutil::TCASE(b"Local minimum of solar incidence angle at north pole of ALPHA; observer is SUN; abcorr = NONE", ctx)?;

    //
    // This is a repeat of the previous case, with only the angle
    // type changed.
    //
    fstr::assign(&mut save.ANGTYP, b" Incidence");

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        &save.ILLUM,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(
        b"RESULT cardinality",
        spicelib::WNCARD(save.RESULT.as_slice(), ctx)?,
        b"=",
        4,
        0,
        OK,
        ctx,
    )?;

    for I in 1..=spicelib::WNCARD(save.RESULT.as_slice(), ctx)? {
        spicelib::WNFETD(
            save.RESULT.as_slice(),
            I,
            &mut save.LEFT,
            &mut save.RIGHT,
            ctx,
        )?;
        //
        // Check event start time.
        //
        fstr::assign(&mut save.TITLE, b"Event # beg time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        save.XTIME = (save.ET0 + ((((I - 1) as f64) * spicelib::SPD()) * 7 as f64));

        testutil::CHCKSD(&save.TITLE, save.LEFT, b"~", save.XTIME, TIMTOL, OK, ctx)?;
        //
        // Check event end time; this should equal the begin time
        // since we're searching for an extremum.
        //
        fstr::assign(&mut save.TITLE, b"Event # end time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        testutil::CHCKSD(&save.TITLE, save.RIGHT, b"~", save.XTIME, TIMTOL, OK, ctx)?;
    }

    //
    //---- Case -------------------------------------------------------------
    //

    //
    // All expected event times are based on Nat's solar system.
    //
    testutil::TCASE(
        b"Local maximum of emission angle at north pole of ALPHA; observer is SUN; abcorr = NONE",
        ctx,
    )?;

    fstr::assign(&mut save.RELATE, b"LOCMAX");

    //
    // ALPHA orbits with constant angular velocity on a circular path,
    // so local maxima occur 1/2 period apart from local minima.
    //

    //
    // Create a confinement window with 4 intervals.
    //
    for I in 1..=4 {
        save.LEFT = (((save.ET0 + (3.5 * spicelib::SPD()))
            + ((((I - 1) as f64) * spicelib::SPD()) * 7 as f64))
            - 3600.0);
        save.RIGHT = (save.LEFT + ((2 as f64) * 3600.0));

        spicelib::WNINSD(save.LEFT, save.RIGHT, save.CNFINE.as_slice_mut(), ctx)?;
    }

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        &save.ILLUM,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(
        b"RESULT cardinality",
        spicelib::WNCARD(save.RESULT.as_slice(), ctx)?,
        b"=",
        4,
        0,
        OK,
        ctx,
    )?;

    for I in 1..=spicelib::WNCARD(save.RESULT.as_slice(), ctx)? {
        spicelib::WNFETD(
            save.RESULT.as_slice(),
            I,
            &mut save.LEFT,
            &mut save.RIGHT,
            ctx,
        )?;
        //
        // Check event start time.
        //
        fstr::assign(&mut save.TITLE, b"Event # beg time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        save.XTIME = ((save.ET0 + (3.5 * spicelib::SPD()))
            + ((((I - 1) as f64) * spicelib::SPD()) * 7 as f64));

        testutil::CHCKSD(&save.TITLE, save.LEFT, b"~", save.XTIME, TIMTOL, OK, ctx)?;

        //
        // Check event end time; this should equal the begin time
        // since we're searching for an extremum.
        //
        fstr::assign(&mut save.TITLE, b"Event # end time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        testutil::CHCKSD(&save.TITLE, save.RIGHT, b"~", save.XTIME, TIMTOL, OK, ctx)?;

        //
        //    Dump event times for debugging:
        //
        //     CALL TIMOUT ( LEFT,
        // .                 'YYYY MON DD HR:MN:SC.###### ::TDB (TDB)',
        // .                 TIMSTR )
        //     WRITE (*,*) I, ' ',  TIMSTR
        //
        //     CALL TIMOUT ( RIGHT,
        // .                 'YYYY MON DD HR:MN:SC.###### ::TDB (TDB)',
        // .                 TIMSTR )
        //     WRITE (*,*) '  ', TIMSTR
    }

    //
    //---- Case -------------------------------------------------------------
    //

    testutil::TCASE(b"Local maximum of solar incidence angle at north pole of ALPHA; observer is SUN; abcorr = NONE", ctx)?;

    //
    // This is a repeat of the previous case, with only the angle
    // type changed.
    //
    fstr::assign(&mut save.ANGTYP, b"  Incidence");

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        &save.ILLUM,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(
        b"RESULT cardinality",
        spicelib::WNCARD(save.RESULT.as_slice(), ctx)?,
        b"=",
        4,
        0,
        OK,
        ctx,
    )?;

    for I in 1..=spicelib::WNCARD(save.RESULT.as_slice(), ctx)? {
        spicelib::WNFETD(
            save.RESULT.as_slice(),
            I,
            &mut save.LEFT,
            &mut save.RIGHT,
            ctx,
        )?;
        //
        // Check event start time.
        //
        fstr::assign(&mut save.TITLE, b"Event # beg time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        save.XTIME = ((save.ET0 + (3.5 * spicelib::SPD()))
            + ((((I - 1) as f64) * spicelib::SPD()) * 7 as f64));

        testutil::CHCKSD(&save.TITLE, save.LEFT, b"~", save.XTIME, TIMTOL, OK, ctx)?;
        //
        // Check event end time; this should equal the begin time
        // since we're searching for an extremum.
        //
        fstr::assign(&mut save.TITLE, b"Event # end time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        testutil::CHCKSD(&save.TITLE, save.RIGHT, b"~", save.XTIME, TIMTOL, OK, ctx)?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Local minimum of phase angle at -X axis of ALPHA; observer is BETA; abcorr = NONE",
        ctx,
    )?;

    //
    // This is a phase angle search.
    //
    fstr::assign(&mut save.ANGTYP, b"PHASE");

    //
    // This time we want to use Beta as the observer. Note that
    // the surface point on Alpha's north pole won't do for this
    // computation.
    //
    // We're going to work around this problem by changing the
    // orientation of Alpha. We'll use the ALPHA_VIEW_XY frame as
    // Alpha's body-fixed frame. This is a two-vector dynamic frame
    // in which the +X axis points from the sun to Alpha at all times.
    // We'll create a sun-facing surface point on Alpha at the tip
    // of Alpha's -X axis.
    //
    spicelib::BODVRD(
        b"ALPHA",
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPACK(-save.RADII[1], 0.0, 0.0, save.SPOINT.as_slice_mut());

    fstr::assign(&mut save.FIXREF, b"ALPHA_VIEW_XY");

    //
    // Set up the participants.
    //
    fstr::assign(&mut save.OBSRVR, b"BETA");
    fstr::assign(&mut save.TARGET, b"ALPHA");
    fstr::assign(&mut save.ILLUM, b"SUN");

    fstr::assign(&mut save.RELATE, b"LOCMIN");
    save.STEP = 3600.0;

    //
    // Set the confinement window to cover a single 4-day time interval.
    //
    save.ET0 = 0.0;

    save.ET1 = (save.ET0 + ((4 as f64) * spicelib::SPD()));

    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        &save.ILLUM,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(
        b"RESULT cardinality",
        spicelib::WNCARD(save.RESULT.as_slice(), ctx)?,
        b"=",
        8,
        0,
        OK,
        ctx,
    )?;

    for I in 1..=spicelib::WNCARD(save.RESULT.as_slice(), ctx)? {
        spicelib::WNFETD(
            save.RESULT.as_slice(),
            I,
            &mut save.LEFT,
            &mut save.RIGHT,
            ctx,
        )?;
        //
        // Check event start time.
        //
        fstr::assign(&mut save.TITLE, b"Event # beg time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        //
        // The phase angle is minimized at the central time of
        // each occultation and midway between these times.
        //
        save.XTIME = (300.0 + ((((I - 1) as f64) * spicelib::SPD()) / 2 as f64));

        testutil::CHCKSD(&save.TITLE, save.LEFT, b"~", save.XTIME, TIMTOL, OK, ctx)?;
        //
        // Check event end time; this should equal the begin time
        // since we're searching for an extremum.
        //
        fstr::assign(&mut save.TITLE, b"Event # end time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        testutil::CHCKSD(&save.TITLE, save.RIGHT, b"~", save.XTIME, TIMTOL, OK, ctx)?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Repeat the previous test with the illumination source set to BETA and the observer set to SUN.", ctx)?;

    //
    // We need at least one test in which the sun is not the illumination
    // source. The event times should be unchanged.
    //

    //
    // This is a phase angle search.
    //
    fstr::assign(&mut save.ANGTYP, b"PHASE");

    //
    // This time we want to use Beta as the observer. Note that
    // the surface point on Alpha's north pole won't do for this
    // computation.
    //
    // We're going to work around this problem by changing the
    // orientation of Alpha. We'll use the ALPHA_VIEW_XY frame as
    // Alpha's body-fixed frame. This is a two-vector dynamic frame
    // in which the +X axis points from the sun to Alpha at all times.
    // We'll create a sun-facing surface point on Alpha at the tip
    // of Alpha's -X axis.
    //
    spicelib::BODVRD(
        b"ALPHA",
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPACK(-save.RADII[1], 0.0, 0.0, save.SPOINT.as_slice_mut());

    fstr::assign(&mut save.FIXREF, b"ALPHA_VIEW_XY");

    //
    // Set up the participants.
    //
    fstr::assign(&mut save.OBSRVR, b"SUN");
    fstr::assign(&mut save.TARGET, b"ALPHA");
    fstr::assign(&mut save.ILLUM, b"BETA");

    fstr::assign(&mut save.RELATE, b"LOCMIN");
    save.STEP = 3600.0;

    //
    // Set the confinement window to cover a single 4-day time interval.
    //
    save.ET0 = 0.0;

    save.ET1 = (save.ET0 + ((4 as f64) * spicelib::SPD()));

    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        &save.ILLUM,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(
        b"RESULT cardinality",
        spicelib::WNCARD(save.RESULT.as_slice(), ctx)?,
        b"=",
        8,
        0,
        OK,
        ctx,
    )?;

    for I in 1..=spicelib::WNCARD(save.RESULT.as_slice(), ctx)? {
        spicelib::WNFETD(
            save.RESULT.as_slice(),
            I,
            &mut save.LEFT,
            &mut save.RIGHT,
            ctx,
        )?;
        //
        // Check event start time.
        //
        fstr::assign(&mut save.TITLE, b"Event # beg time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        //
        // The phase angle is minimized at the central time of
        // each occultation and midway between these times.
        //
        save.XTIME = (300.0 + ((((I - 1) as f64) * spicelib::SPD()) / 2 as f64));

        testutil::CHCKSD(&save.TITLE, save.LEFT, b"~", save.XTIME, TIMTOL, OK, ctx)?;
        //
        // Check event end time; this should equal the begin time
        // since we're searching for an extremum.
        //
        fstr::assign(&mut save.TITLE, b"Event # end time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        testutil::CHCKSD(&save.TITLE, save.RIGHT, b"~", save.XTIME, TIMTOL, OK, ctx)?;
    }

    //*********************************************************************
    //*
    //*    Comprehensive cases using comparisons against alternate
    //*    computations
    //*
    //*********************************************************************

    //*********************************************************************
    //*
    //*    PHASE angle tests
    //*
    //*********************************************************************

    //
    // We'use use the earth, moon, and sun as the three participating
    // bodies. The surface point will be the OCTL telescope; we'll
    // create an SPK file for this point. We also need an FK for a
    // topocentric frame centered at the point.
    //
    // Though it's not strictly necessary, we'll use real data for
    // these kernels, with one exception: we'll use the terrestrial
    // reference frame IAU_EARTH rather than ITRF93.
    //
    // The original reference frame specifications follow:
    //
    //
    //    Topocentric frame OCTL_TOPO
    //
    //       The Z axis of this frame points toward the zenith.
    //       The X axis of this frame points North.
    //
    //       Topocentric frame OCTL_TOPO is centered at the site OCTL
    //       which has Cartesian coordinates
    //
    //          X (km):                 -0.2448937761729E+04
    //          Y (km):                 -0.4667935793438E+04
    //          Z (km):                  0.3582748499430E+04
    //
    //       and planetodetic coordinates
    //
    //          Longitude (deg):      -117.6828380000000
    //          Latitude  (deg):        34.3817491000000
    //          Altitude   (km):         0.2259489999999E+01
    //
    //       These planetodetic coordinates are expressed relative to
    //       a reference spheroid having the dimensions
    //
    //          Equatorial radius (km):  6.3781400000000E+03
    //          Polar radius      (km):  6.3567523100000E+03
    //
    //       All of the above coordinates are relative to the frame
    //       ITRF93.
    //

    //
    //---- Case -------------------------------------------------------------
    //

    //
    // This isn't a test, but we'll call it that so we'll have
    // a meaningful label in any error messages that arise.
    //
    testutil::TCASE(b"Create OCTL kernels.", ctx)?;

    //
    // As mentioned, we go with a frame that's more convenient than
    // ITRF93:
    //
    fstr::assign(&mut save.FIXREF, b"IAU_EARTH");

    //
    // Prepare a frame kernel in a string array.
    //
    fstr::assign(
        save.TXT.get_mut(1),
        b"FRAME_OCTL_TOPO            =  1398962",
    );
    fstr::assign(
        save.TXT.get_mut(2),
        b"FRAME_1398962_NAME         =  \'OCTL_TOPO\' ",
    );
    fstr::assign(save.TXT.get_mut(3), b"FRAME_1398962_CLASS        =  4");
    fstr::assign(
        save.TXT.get_mut(4),
        b"FRAME_1398962_CLASS_ID     =  1398962",
    );
    fstr::assign(save.TXT.get_mut(5), b"FRAME_1398962_CENTER       =  398962");

    fstr::assign(
        save.TXT.get_mut(6),
        b"OBJECT_398962_FRAME        =  \'OCTL_TOPO\' ",
    );

    fstr::assign(
        save.TXT.get_mut(7),
        b"TKFRAME_1398962_RELATIVE   =  \'IAU_EARTH\' ",
    );
    fstr::assign(
        save.TXT.get_mut(8),
        b"TKFRAME_1398962_SPEC       =  \'ANGLES\' ",
    );
    fstr::assign(
        save.TXT.get_mut(9),
        b"TKFRAME_1398962_UNITS      =  \'DEGREES\' ",
    );
    fstr::assign(
        save.TXT.get_mut(10),
        b"TKFRAME_1398962_AXES       =  ( 3, 2, 3 )",
    );
    fstr::assign(
        save.TXT.get_mut(11),
        b"TKFRAME_1398962_ANGLES     =  ( -242.3171620000000,",
    );
    fstr::assign(
        save.TXT.get_mut(12),
        b"                                 -55.6182509000000,",
    );
    fstr::assign(
        save.TXT.get_mut(13),
        b"                                 180.0000000000000  )",
    );
    fstr::assign(
        save.TXT.get_mut(14),
        b"NAIF_BODY_NAME            +=  \'OCTL\' ",
    );
    fstr::assign(
        save.TXT.get_mut(15),
        b"NAIF_BODY_CODE            +=  398962",
    );

    //
    // It will be convenient to have a version of this frame that
    // has the +Z axis pointed down instead of up.
    //
    fstr::assign(
        save.TXT.get_mut(16),
        b"FRAME_OCTL_FLIP            =  2398962",
    );
    fstr::assign(
        save.TXT.get_mut(17),
        b"FRAME_2398962_NAME         =  \'OCTL_FLIP\' ",
    );
    fstr::assign(save.TXT.get_mut(18), b"FRAME_2398962_CLASS        =  4");
    fstr::assign(
        save.TXT.get_mut(19),
        b"FRAME_2398962_CLASS_ID     =  2398962",
    );
    fstr::assign(
        save.TXT.get_mut(20),
        b"FRAME_2398962_CENTER       =  398962",
    );

    fstr::assign(
        save.TXT.get_mut(21),
        b"TKFRAME_2398962_RELATIVE   =  \'OCTL_TOPO\' ",
    );
    fstr::assign(
        save.TXT.get_mut(22),
        b"TKFRAME_2398962_SPEC       =  \'ANGLES\' ",
    );
    fstr::assign(
        save.TXT.get_mut(23),
        b"TKFRAME_2398962_UNITS      =  \'DEGREES\' ",
    );
    fstr::assign(
        save.TXT.get_mut(24),
        b"TKFRAME_2398962_AXES       =  ( 3, 2, 3 )",
    );
    fstr::assign(
        save.TXT.get_mut(25),
        b"TKFRAME_2398962_ANGLES     =  ( 0, 180.0, 0 ) ",
    );

    spicelib::LMPOOL(save.TXT.as_arg(), TXTSIZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now create an SPK file containing a type 8 segment for OCTL.
    //
    spicelib::SPKOPN(OCTSPK, OCTSPK, 0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Initialize both states to zero.
    //
    spicelib::CLEARD(12, save.STATES.as_slice_mut());

    //
    // The first position component:
    //
    save.SPOINT[1] = -2448.937761729;
    save.SPOINT[2] = -4667.935793438;
    save.SPOINT[3] = 3582.74849943;

    //
    // We're going to use a version of the OCTL position
    // that has zero altitude relative to the earth's
    // reference ellipsoid. This is done to enable
    // consistency checks to be done using GFPOSC.
    //
    // For compatibility with the topocentric frame
    // specification above, we'll use the following
    // earth radii:
    //
    save.RADII[1] = 6378.14;
    save.RADII[2] = 6378.14;
    save.RADII[3] = 6356.75231;

    spicelib::PDPOOL(b"BODY399_RADII", 3, save.RADII.as_slice(), ctx)?;

    spicelib::NEARPT(
        save.SPOINT.as_slice(),
        save.RADII[1],
        save.RADII[2],
        save.RADII[3],
        save.NP.as_slice_mut(),
        &mut save.ALT,
        ctx,
    )?;

    spicelib::VEQU(save.NP.as_slice(), save.SPOINT.as_slice_mut());

    spicelib::VEQU(save.SPOINT.as_slice(), save.STATES.subarray_mut([1, 1]));

    //
    // The second position matches the first: we don't model
    // plate motion.
    //
    spicelib::VEQU(save.SPOINT.as_slice(), save.STATES.subarray_mut([1, 2]));

    //
    // Time bounds for the segment:
    //

    save.FIRST = (((-50 as f64) * spicelib::SPD()) * 365.25);
    save.STEP = (((100 as f64) * spicelib::SPD()) * 365.25);

    save.LAST = ((save.FIRST + save.STEP) - 0.000001);

    //
    // Get the OCTL ID code from the kernel we just
    // loaded.
    //
    spicelib::BODN2C(OCTL, &mut save.OCTID, &mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // Write the segment.
    //
    spicelib::SPKW08(
        save.HANDLE,
        save.OCTID,
        399,
        &save.FIXREF,
        save.FIRST,
        save.LAST,
        b"octl",
        1,
        2,
        save.STATES.as_slice(),
        save.FIRST,
        save.STEP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now load the OCTL SPK file.
    //
    spicelib::FURNSH(OCTSPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Phase angle tests: we'll compare results from GFILUM to those
    // obtained using GFPA. Note that the surface point must be
    // an ephemeris object in order to carry out these tests.
    //
    fstr::assign(&mut save.ANGTYP, b" Phase");

    fstr::assign(&mut save.ILLUM, b"Sun");
    fstr::assign(&mut save.TARGET, b" 399");
    fstr::assign(&mut save.OBSRVR, b" moon");

    //
    // Note that FIXREF has already been set.
    //

    //
    // Set up the confinement window.
    //
    fstr::assign(&mut save.TIME0, b"2011 JAN 1");

    spicelib::STR2ET(&save.TIME0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Search over approximately two months.
    //
    save.ET1 = (save.ET0 + ((60 as f64) * spicelib::SPD()));

    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;

    //
    // Use a 10 day step. We expect the extrema to
    // be about 14 days apart.
    //
    save.STEP = ((10 as f64) * spicelib::SPD());

    //
    // Loop over all operators and aberration corrections.
    //

    for OPIDX in 1..=NREL {
        //
        // Convert the reference value to radians.
        //
        fstr::assign(&mut save.RELATE, save.RELOPS.get(OPIDX));
        save.REFVAL = (save.VALS[OPIDX] * spicelib::RPD(ctx));
        save.ADJUST = save.ADJ[OPIDX];

        for CORIDX in 1..=NCORR {
            fstr::assign(&mut save.ABCORR, save.CORRS.get(CORIDX));
            //
            //---- Case -------------------------------------------------------------
            //
            fstr::assign(&mut save.TITLE, b"Phase angle search: RELATE = #; REFVAL (deg) = #; ADJUST = #; ABCORR = #; observer = #; target = #; illum source = #; FIXREF = #; SPOINT = ( # # # ).");

            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.RELATE, &mut save.TITLE);
            spicelib::REPMD(
                &save.TITLE.to_vec(),
                b"#",
                save.VALS[OPIDX],
                14,
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMD(
                &save.TITLE.to_vec(),
                b"#",
                save.ADJUST,
                14,
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.OBSRVR, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ILLUM, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.FIXREF, &mut save.TITLE);
            spicelib::REPMD(
                &save.TITLE.to_vec(),
                b"#",
                save.SPOINT[1],
                14,
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMD(
                &save.TITLE.to_vec(),
                b"#",
                save.SPOINT[2],
                14,
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMD(
                &save.TITLE.to_vec(),
                b"#",
                save.SPOINT[3],
                14,
                &mut save.TITLE,
                ctx,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::TCASE(&save.TITLE, ctx)?;

            //
            // Initialize the result windows.
            //
            spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
            spicelib::SSIZED(MAXWIN, save.XRSULT.as_slice_mut(), ctx)?;

            //
            // Find the expected result window. Note that the
            // target is OCTL in this case.
            //
            spicelib::GFPA(
                OCTL,
                &save.ILLUM,
                &save.ABCORR,
                &save.OBSRVR,
                &save.RELATE,
                save.REFVAL,
                save.ADJUST,
                save.STEP,
                save.CNFINE.as_slice(),
                MAXWIN,
                NWPA,
                save.WORK.as_slice_mut(),
                save.XRSULT.as_slice_mut(),
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            if *OK {
                //
                // Search for the condition of interest using GFILUM.
                //
                spicelib::GFILUM(
                    &save.METHOD,
                    &save.ANGTYP,
                    &save.TARGET,
                    &save.ILLUM,
                    &save.FIXREF,
                    &save.ABCORR,
                    &save.OBSRVR,
                    save.SPOINT.as_slice(),
                    &save.RELATE,
                    save.REFVAL,
                    save.ADJUST,
                    save.STEP,
                    save.CNFINE.as_slice(),
                    MAXWIN,
                    NWILUM,
                    save.WORK.as_slice_mut(),
                    save.RESULT.as_slice_mut(),
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                if *OK {
                    //
                    // Compare result window cardinalities first.
                    //
                    save.XN = spicelib::WNCARD(save.XRSULT.as_slice(), ctx)?;
                    //
                    // Test family validity check: we're not supposed to
                    // have any cases where the results are empty.
                    //
                    testutil::CHCKSI(b"XN", save.XN, b">", 0, 0, OK, ctx)?;

                    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

                    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

                    if *OK {
                        //
                        // Compare result window interval bounds.
                        //
                        testutil::CHCKAD(
                            b"RESULT",
                            save.RESULT.subarray(1),
                            b"~",
                            save.XRSULT.subarray(1),
                            (2 * save.N),
                            LOOSE,
                            OK,
                            ctx,
                        )?;

                        if *OK {
                            //
                            // The result window found by GFILUM agrees
                            // with that found by GFPA. Check the actual
                            // angular values at the window endpoints
                            // for the cases where the operator is
                            // binary.
                            //
                            if fstr::eq(&save.RELATE, b"=") {
                                //
                                // Check the phase angle at each interval
                                // endpoint.
                                //
                                for I in 1..=save.N {
                                    spicelib::WNFETD(
                                        save.RESULT.as_slice(),
                                        I,
                                        &mut save.START,
                                        &mut save.FINISH,
                                        ctx,
                                    )?;
                                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                                    fstr::assign(&mut save.TITLE, b"Angle at start of interval #");
                                    spicelib::REPMI(
                                        &save.TITLE.to_vec(),
                                        b"#",
                                        I,
                                        &mut save.TITLE,
                                        ctx,
                                    );

                                    spicelib::ILUMIN(
                                        &save.METHOD,
                                        b"EARTH",
                                        save.START,
                                        &save.FIXREF,
                                        &save.ABCORR,
                                        &save.OBSRVR,
                                        save.SPOINT.as_slice(),
                                        &mut save.TRGEPC,
                                        save.SRFVEC.as_slice_mut(),
                                        &mut save.PHASE,
                                        &mut save.SOLAR,
                                        &mut save.EMISSN,
                                        ctx,
                                    )?;

                                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                                    testutil::CHCKSD(
                                        &save.TITLE,
                                        save.PHASE,
                                        b"~",
                                        save.REFVAL,
                                        ANGTOL,
                                        OK,
                                        ctx,
                                    )?;

                                    fstr::assign(&mut save.TITLE, b"Angle at end of interval #");
                                    spicelib::REPMI(
                                        &save.TITLE.to_vec(),
                                        b"#",
                                        I,
                                        &mut save.TITLE,
                                        ctx,
                                    );

                                    spicelib::ILUMIN(
                                        &save.METHOD,
                                        b"EARTH",
                                        save.FINISH,
                                        &save.FIXREF,
                                        &save.ABCORR,
                                        &save.OBSRVR,
                                        save.SPOINT.as_slice(),
                                        &mut save.TRGEPC,
                                        save.SRFVEC.as_slice_mut(),
                                        &mut save.PHASE,
                                        &mut save.SOLAR,
                                        &mut save.EMISSN,
                                        ctx,
                                    )?;

                                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                                    testutil::CHCKSD(
                                        &save.TITLE,
                                        save.PHASE,
                                        b"~",
                                        save.REFVAL,
                                        ANGTOL,
                                        OK,
                                        ctx,
                                    )?;
                                }
                            } else if (fstr::eq(&save.RELATE, b"<") || fstr::eq(&save.RELATE, b">"))
                            {
                                //
                                // Perform checks only at interval endpoints
                                // contained in the interior of the confinement
                                // window. At the confinement window boundaries,
                                // the inequality may be satisfied without the
                                // value being equal to the reference value.
                                //
                                for I in 1..=save.N {
                                    spicelib::WNFETD(
                                        save.RESULT.as_slice(),
                                        I,
                                        &mut save.START,
                                        &mut save.FINISH,
                                        ctx,
                                    )?;
                                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                                    fstr::assign(&mut save.TITLE, b"Angle at start of interval #");
                                    spicelib::REPMI(
                                        &save.TITLE.to_vec(),
                                        b"#",
                                        I,
                                        &mut save.TITLE,
                                        ctx,
                                    );

                                    spicelib::ILUMIN(
                                        &save.METHOD,
                                        b"EARTH",
                                        save.START,
                                        &save.FIXREF,
                                        &save.ABCORR,
                                        &save.OBSRVR,
                                        save.SPOINT.as_slice(),
                                        &mut save.TRGEPC,
                                        save.SRFVEC.as_slice_mut(),
                                        &mut save.PHASE,
                                        &mut save.SOLAR,
                                        &mut save.EMISSN,
                                        ctx,
                                    )?;

                                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                                    if (save.START > save.ET0) {
                                        testutil::CHCKSD(
                                            &save.TITLE,
                                            save.PHASE,
                                            b"~",
                                            save.REFVAL,
                                            ANGTOL,
                                            OK,
                                            ctx,
                                        )?;
                                    }

                                    fstr::assign(&mut save.TITLE, b"Angle at end of interval #");
                                    spicelib::REPMI(
                                        &save.TITLE.to_vec(),
                                        b"#",
                                        I,
                                        &mut save.TITLE,
                                        ctx,
                                    );

                                    spicelib::ILUMIN(
                                        &save.METHOD,
                                        b"EARTH",
                                        save.FINISH,
                                        &save.FIXREF,
                                        &save.ABCORR,
                                        &save.OBSRVR,
                                        save.SPOINT.as_slice(),
                                        &mut save.TRGEPC,
                                        save.SRFVEC.as_slice_mut(),
                                        &mut save.PHASE,
                                        &mut save.SOLAR,
                                        &mut save.EMISSN,
                                        ctx,
                                    )?;

                                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                                    if (save.FINISH < save.ET1) {
                                        testutil::CHCKSD(
                                            &save.TITLE,
                                            save.PHASE,
                                            b"~",
                                            save.REFVAL,
                                            ANGTOL,
                                            OK,
                                            ctx,
                                        )?;
                                    }
                                }
                                //
                                // End of phase angle value check loop.
                                //
                            }
                            //
                            // End of phase angle checks.
                            //
                        }
                        //
                        // End of IF block for successful RESULT check.
                        //
                    }
                    //
                    // End of interval endpoint checks.
                    //
                }
                //
                // End of IF block for successful GFILUM call.
                //
            }
            //
            // End of IF block for successful GFPA call.
            //
        }
        //
        // End of aberration correction loop.
        //
    }
    //
    // End of operator loop.
    //

    //*********************************************************************
    //*
    //*    EMISSION angle tests
    //*
    //*********************************************************************

    //
    // Emission angle tests: we'll compare results from GFILUM to those
    // obtained using GFPOSC. Note that the surface point must be
    // an ephemeris object having an associated topocentric frame
    // in order to carry out these tests.
    //

    //
    // The emission angle is the supplement of the colatitude,
    // measured in the surface point topocentric frame, of
    // the observer-surface point vector. Equivalently, the emission
    // angle is the colatitude of the observer-surface point vector
    // in the "flip" frame, which has its +Z axis pointing downward.
    //
    // We can use GFPOSC to find times when this colatitude, measured
    // in the flip frame, meets conditions on the emission angle.
    //
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.ILLUM, b"SUN");
    fstr::assign(&mut save.SRFREF, b"OCTL_FLIP");
    fstr::assign(&mut save.CORSYS, b"SPHERICAL");
    fstr::assign(&mut save.COORD, b"COLATITUDE");

    fstr::assign(&mut save.ANGTYP, b"EMISSION");

    //
    // For the emission angle test, we don't need to use
    // a month-long confinement window. A few days is enough.
    //
    save.ET1 = (save.ET0 + ((3 as f64) * spicelib::SPD()));

    //
    // Use a 6 hour step. We expect the extrema to
    // be 12+delta hours apart, where delta may be
    // a few hours.
    //
    save.STEP = (6.0 * 3600.0);

    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Loop over all operators and aberration corrections.
    //
    for OPIDX in 1..=NREL {
        //
        // Convert the reference value to radians.
        //
        fstr::assign(&mut save.RELATE, save.RELOPS.get(OPIDX));
        save.REFVAL = (save.VALS[OPIDX] * spicelib::RPD(ctx));
        save.ADJUST = save.ADJ[OPIDX];

        for CORIDX in 1..=NCORR {
            fstr::assign(&mut save.ABCORR, save.CORRS.get(CORIDX));
            //
            //---- Case -------------------------------------------------------------
            //
            fstr::assign(&mut save.TITLE, b"Emission angle search: RELATE = #; REFVAL (deg) = #; ADJUST = #; ABCORR = #; observer = #; target = #; illum source = #; FIXREF = #; SPOINT = ( # # # ); SRFREF = #.");

            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.RELATE, &mut save.TITLE);
            spicelib::REPMD(
                &save.TITLE.to_vec(),
                b"#",
                save.VALS[OPIDX],
                14,
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMD(
                &save.TITLE.to_vec(),
                b"#",
                save.ADJUST,
                14,
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.OBSRVR, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ILLUM, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.FIXREF, &mut save.TITLE);
            spicelib::REPMD(
                &save.TITLE.to_vec(),
                b"#",
                save.SPOINT[1],
                14,
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMD(
                &save.TITLE.to_vec(),
                b"#",
                save.SPOINT[2],
                14,
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMD(
                &save.TITLE.to_vec(),
                b"#",
                save.SPOINT[3],
                14,
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.SRFREF, &mut save.TITLE);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::TCASE(&save.TITLE, ctx)?;

            //
            // Initialize the result windows.
            //
            spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
            spicelib::SSIZED(MAXWIN, save.XRSULT.as_slice_mut(), ctx)?;

            //
            // Find the expected result window. Note that the
            // target is OCTL in this case.
            //
            spicelib::GFPOSC(
                OCTL,
                &save.SRFREF,
                &save.ABCORR,
                &save.OBSRVR,
                &save.CORSYS,
                &save.COORD,
                &save.RELATE,
                save.REFVAL,
                save.ADJUST,
                save.STEP,
                save.CNFINE.as_slice(),
                MAXWIN,
                NWMAX,
                save.WORK.as_slice_mut(),
                save.XRSULT.as_slice_mut(),
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            if *OK {
                //
                // Search for the condition of interest using GFILUM.
                //
                spicelib::GFILUM(
                    &save.METHOD,
                    &save.ANGTYP,
                    &save.TARGET,
                    &save.ILLUM,
                    &save.FIXREF,
                    &save.ABCORR,
                    &save.OBSRVR,
                    save.SPOINT.as_slice(),
                    &save.RELATE,
                    save.REFVAL,
                    save.ADJUST,
                    save.STEP,
                    save.CNFINE.as_slice(),
                    MAXWIN,
                    NWILUM,
                    save.WORK.as_slice_mut(),
                    save.RESULT.as_slice_mut(),
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                if *OK {
                    //
                    // Compare result window cardinalities first.
                    //
                    save.XN = spicelib::WNCARD(save.XRSULT.as_slice(), ctx)?;
                    //
                    // Test family validity check: we're not supposed to
                    // have any cases where the results are empty.
                    //
                    testutil::CHCKSI(b"XN", save.XN, b">", 0, 0, OK, ctx)?;

                    // WRITE (*,*) RELATE, '  ', 'XN = ', XN

                    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

                    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

                    if *OK {
                        //
                        // Compare result window interval bounds.
                        //
                        testutil::CHCKAD(
                            b"RESULT",
                            save.RESULT.subarray(1),
                            b"~",
                            save.XRSULT.subarray(1),
                            (2 * save.N),
                            LOOSE,
                            OK,
                            ctx,
                        )?;

                        if *OK {
                            //
                            // The result window found by GFILUM agrees
                            // with that found by GFPOSC. Check the actual
                            // angular values at the window endpoints
                            // for the cases where the operator is
                            // binary.
                            //
                            if fstr::eq(&save.RELATE, b"=") {
                                //
                                // Check the emission angle at each interval
                                // endpoint.
                                //
                                for I in 1..=save.N {
                                    spicelib::WNFETD(
                                        save.RESULT.as_slice(),
                                        I,
                                        &mut save.START,
                                        &mut save.FINISH,
                                        ctx,
                                    )?;
                                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                                    fstr::assign(&mut save.TITLE, b"Angle at start of interval #");
                                    spicelib::REPMI(
                                        &save.TITLE.to_vec(),
                                        b"#",
                                        I,
                                        &mut save.TITLE,
                                        ctx,
                                    );

                                    spicelib::ILUMIN(
                                        &save.METHOD,
                                        b"EARTH",
                                        save.START,
                                        &save.FIXREF,
                                        &save.ABCORR,
                                        &save.OBSRVR,
                                        save.SPOINT.as_slice(),
                                        &mut save.TRGEPC,
                                        save.SRFVEC.as_slice_mut(),
                                        &mut save.PHASE,
                                        &mut save.SOLAR,
                                        &mut save.EMISSN,
                                        ctx,
                                    )?;

                                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                                    testutil::CHCKSD(
                                        &save.TITLE,
                                        save.EMISSN,
                                        b"~",
                                        save.REFVAL,
                                        ANGTOL,
                                        OK,
                                        ctx,
                                    )?;

                                    fstr::assign(&mut save.TITLE, b"Angle at end of interval #");
                                    spicelib::REPMI(
                                        &save.TITLE.to_vec(),
                                        b"#",
                                        I,
                                        &mut save.TITLE,
                                        ctx,
                                    );

                                    spicelib::ILUMIN(
                                        &save.METHOD,
                                        b"EARTH",
                                        save.FINISH,
                                        &save.FIXREF,
                                        &save.ABCORR,
                                        &save.OBSRVR,
                                        save.SPOINT.as_slice(),
                                        &mut save.TRGEPC,
                                        save.SRFVEC.as_slice_mut(),
                                        &mut save.PHASE,
                                        &mut save.SOLAR,
                                        &mut save.EMISSN,
                                        ctx,
                                    )?;

                                    testutil::CHCKSD(
                                        &save.TITLE,
                                        save.EMISSN,
                                        b"~",
                                        save.REFVAL,
                                        ANGTOL,
                                        OK,
                                        ctx,
                                    )?;
                                }
                            } else if (fstr::eq(&save.RELATE, b"<") || fstr::eq(&save.RELATE, b">"))
                            {
                                //
                                // Perform checks only at interval endpoints
                                // contained in the interior of the confinement
                                // window. At the confinement window boundaries,
                                // the inequality may be satisfied without the
                                // value being equal to the reference value.
                                //
                                for I in 1..=save.N {
                                    spicelib::WNFETD(
                                        save.RESULT.as_slice(),
                                        I,
                                        &mut save.START,
                                        &mut save.FINISH,
                                        ctx,
                                    )?;
                                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                                    fstr::assign(&mut save.TITLE, b"Angle at start of interval #");
                                    spicelib::REPMI(
                                        &save.TITLE.to_vec(),
                                        b"#",
                                        I,
                                        &mut save.TITLE,
                                        ctx,
                                    );

                                    spicelib::ILUMIN(
                                        &save.METHOD,
                                        b"EARTH",
                                        save.START,
                                        &save.FIXREF,
                                        &save.ABCORR,
                                        &save.OBSRVR,
                                        save.SPOINT.as_slice(),
                                        &mut save.TRGEPC,
                                        save.SRFVEC.as_slice_mut(),
                                        &mut save.PHASE,
                                        &mut save.SOLAR,
                                        &mut save.EMISSN,
                                        ctx,
                                    )?;

                                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                                    if (save.START > save.ET0) {
                                        testutil::CHCKSD(
                                            &save.TITLE,
                                            save.EMISSN,
                                            b"~",
                                            save.REFVAL,
                                            ANGTOL,
                                            OK,
                                            ctx,
                                        )?;
                                    }

                                    fstr::assign(&mut save.TITLE, b"Angle at end of interval #");
                                    spicelib::REPMI(
                                        &save.TITLE.to_vec(),
                                        b"#",
                                        I,
                                        &mut save.TITLE,
                                        ctx,
                                    );

                                    spicelib::ILUMIN(
                                        &save.METHOD,
                                        b"EARTH",
                                        save.FINISH,
                                        &save.FIXREF,
                                        &save.ABCORR,
                                        &save.OBSRVR,
                                        save.SPOINT.as_slice(),
                                        &mut save.TRGEPC,
                                        save.SRFVEC.as_slice_mut(),
                                        &mut save.PHASE,
                                        &mut save.SOLAR,
                                        &mut save.EMISSN,
                                        ctx,
                                    )?;

                                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                                    if (save.FINISH < save.ET1) {
                                        testutil::CHCKSD(
                                            &save.TITLE,
                                            save.EMISSN,
                                            b"~",
                                            save.REFVAL,
                                            ANGTOL,
                                            OK,
                                            ctx,
                                        )?;
                                    }
                                }
                                //
                                // End of emission angle value check loop.
                                //
                            }
                            //
                            // End of emission angle checks.
                            //
                        }
                        //
                        // End of IF block for successful RESULT check.
                        //
                    }
                    //
                    // End of interval endpoint checks.
                    //
                }
                //
                // End of IF block for successful GFILUM call.
                //
            }
            //
            // End of IF block for successful GFPOSC call.
            //
        }
        //
        // End of aberration correction loop.
        //
    }
    //
    // End of operator loop.
    //

    //*********************************************************************
    //*
    //*    INCIDENCE angle tests
    //*
    //*********************************************************************

    //
    // Incidence angle tests: we'll compare results from GFILUM to
    // those obtained using GFPOSC, where in the latter case, the
    // surface point is treated as the observer. Results obtained using
    // GFILUM must have one-way observer-surface point light time
    // subtracted in order to be comparable to those from GFPOSC.
    //
    // Note that the surface point must be an ephemeris object having an
    // associate topocentric frame in order to carry out these tests.
    //
    // In the geometric case, the solar incidence angle is the
    // colatitude, measured in the surface point topocentric frame, of
    // the surface point-sun vector. When aberration corrections are
    // used, the surface point-sun vector must be computed at an epoch
    // corrected for observer-surface point light time.
    //
    // We can use GFPOSC to find times when this colatitude, measured in
    // the OCTL topocentric frame, meets conditions on the solar
    // incidence angle.
    //
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.ILLUM, b"SUN");

    fstr::assign(&mut save.SRFREF, b"OCTL_TOPO");
    fstr::assign(&mut save.CORSYS, b"SPHERICAL");
    fstr::assign(&mut save.COORD, b"COLATITUDE");

    fstr::assign(&mut save.ANGTYP, b"INCIDENCE");

    //
    // For the solar incidence angle test, we don't need to use
    // a month-long confinement window. A few days is enough.
    //

    //
    // Set up the confinement window.
    //
    fstr::assign(&mut save.TIME0, b"2011 JAN 1");

    spicelib::STR2ET(&save.TIME0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET1 = (save.ET0 + ((3 as f64) * spicelib::SPD()));

    //
    // Use a 6 hour step. We expect the extrema to
    // be 12+delta hours apart, where delta may be
    // a few hours.
    //
    save.STEP = (6.0 * 3600.0);

    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Loop over all operators and aberration corrections.
    //
    for OPIDX in 1..=NREL {
        //
        // Convert the reference value to radians.
        //
        fstr::assign(&mut save.RELATE, save.RELOPS.get(OPIDX));
        save.REFVAL = (save.VALS[OPIDX] * spicelib::RPD(ctx));
        save.ADJUST = save.ADJ[OPIDX];

        for CORIDX in 1..=NCORR {
            fstr::assign(&mut save.ABCORR, save.CORRS.get(CORIDX));
            //
            //---- Case -------------------------------------------------------------
            //
            fstr::assign(&mut save.TITLE, b"Solar incidence angle search: RELATE = #; REFVAL (deg) = #; ADJUST = #; ABCORR = #; observer = #; target = #; illum source = #; FIXREF = #; SPOINT = ( # # # ); SRFREF = #.");

            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.RELATE, &mut save.TITLE);
            spicelib::REPMD(
                &save.TITLE.to_vec(),
                b"#",
                save.VALS[OPIDX],
                14,
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMD(
                &save.TITLE.to_vec(),
                b"#",
                save.ADJUST,
                14,
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.OBSRVR, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ILLUM, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.FIXREF, &mut save.TITLE);
            spicelib::REPMD(
                &save.TITLE.to_vec(),
                b"#",
                save.SPOINT[1],
                14,
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMD(
                &save.TITLE.to_vec(),
                b"#",
                save.SPOINT[2],
                14,
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMD(
                &save.TITLE.to_vec(),
                b"#",
                save.SPOINT[3],
                14,
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.SRFREF, &mut save.TITLE);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::TCASE(&save.TITLE, ctx)?;

            //
            // Initialize the result windows.
            //
            spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
            spicelib::SSIZED(MAXWIN, save.XRSULT.as_slice_mut(), ctx)?;
            spicelib::SSIZED(MAXWIN, save.MODCFN.as_slice_mut(), ctx)?;
            spicelib::SSIZED(MAXWIN, save.MODRES.as_slice_mut(), ctx)?;

            if spicelib::EQSTR(&save.ABCORR, b"NONE") {
                spicelib::COPYD(save.CNFINE.as_slice(), save.MODCFN.as_slice_mut(), ctx)?;
            } else {
                //
                // Create a modified confinement window: this is the
                // original confinement window, adjusted for one-way light
                // time between the observer and the surface point.
                //
                for I in 1..=spicelib::CARDD(save.CNFINE.as_slice(), ctx)? {
                    save.ET = save.CNFINE[I];

                    spicelib::SPKPOS(
                        OCTL,
                        save.ET,
                        b"J2000",
                        &save.ABCORR,
                        &save.OBSRVR,
                        save.POS.as_slice_mut(),
                        &mut save.LT,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.MODCFN[I] = (save.ET - save.LT);
                }

                spicelib::SCARDD(
                    spicelib::CARDD(save.CNFINE.as_slice(), ctx)?,
                    save.MODCFN.as_slice_mut(),
                    ctx,
                )?;
            }

            //
            // Find the expected result window. Note that the
            // target is the sun and the observer is OCTL in this case.
            //
            spicelib::GFPOSC(
                b"SUN",
                &save.SRFREF,
                &save.ABCORR,
                OCTL,
                &save.CORSYS,
                &save.COORD,
                &save.RELATE,
                save.REFVAL,
                save.ADJUST,
                save.STEP,
                save.MODCFN.as_slice(),
                MAXWIN,
                NWMAX,
                save.WORK.as_slice_mut(),
                save.XRSULT.as_slice_mut(),
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            if *OK {
                //
                // Search for the condition of interest using GFILUM.
                //
                spicelib::GFILUM(
                    &save.METHOD,
                    &save.ANGTYP,
                    &save.TARGET,
                    &save.ILLUM,
                    &save.FIXREF,
                    &save.ABCORR,
                    &save.OBSRVR,
                    save.SPOINT.as_slice(),
                    &save.RELATE,
                    save.REFVAL,
                    save.ADJUST,
                    save.STEP,
                    save.CNFINE.as_slice(),
                    MAXWIN,
                    NWILUM,
                    save.WORK.as_slice_mut(),
                    save.RESULT.as_slice_mut(),
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                if *OK {
                    //
                    // Compare result window cardinalities first.
                    //
                    save.XN = spicelib::WNCARD(save.XRSULT.as_slice(), ctx)?;
                    //
                    // Test family validity check: we're not supposed to
                    // have any cases where the results are empty.
                    //
                    testutil::CHCKSI(b"XN", save.XN, b">", 0, 0, OK, ctx)?;

                    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

                    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

                    if *OK {
                        //
                        // Compare result window interval bounds.
                        //
                        // Here's where things get a bit tricky. We can't
                        // just compare RESULT against XRSULT, because
                        // these windows contain event times for different
                        // locations. They're directly comparable only
                        // when aberration corrections are turned off.
                        //
                        if spicelib::EQSTR(&save.ABCORR, b"NONE") {
                            //
                            // No problem: this is the geometric case.
                            //
                            testutil::CHCKAD(
                                b"RESULT",
                                save.RESULT.subarray(1),
                                b"~",
                                save.XRSULT.subarray(1),
                                (2 * save.N),
                                LOOSE,
                                OK,
                                ctx,
                            )?;
                        } else {
                            //
                            // Modify the window of event times by subtracting
                            // the applicable one-way light time from each
                            // element.
                            //
                            spicelib::SSIZED(MAXWIN, save.MODRES.as_slice_mut(), ctx)?;

                            for I in 1..=spicelib::CARDD(save.RESULT.as_slice(), ctx)? {
                                save.ET = save.RESULT[I];
                                //
                                // Find the light time from the surface point
                                // to the observer, where the reception time
                                // is ET.
                                //
                                spicelib::SPKPOS(
                                    OCTL,
                                    save.ET,
                                    b"J2000",
                                    &save.ABCORR,
                                    &save.OBSRVR,
                                    save.POS.as_slice_mut(),
                                    &mut save.LT,
                                    ctx,
                                )?;
                                testutil::CHCKXC(false, b" ", OK, ctx)?;

                                save.MODRES[I] = (save.RESULT[I] - save.LT);
                            }

                            if *OK {
                                //
                                // Compare the transformed result window to
                                // the expected window.
                                //
                                testutil::CHCKAD(
                                    b"RESULT (modified)",
                                    save.MODRES.subarray(1),
                                    b"~",
                                    save.XRSULT.subarray(1),
                                    (2 * save.N),
                                    LOOSE,
                                    OK,
                                    ctx,
                                )?;
                            }
                        }

                        if *OK {
                            //
                            // The result window found by GFILUM agrees
                            // with that found by GFPOSC. Check the actual
                            // angular values at the window endpoints
                            // for the cases where the operator is
                            // binary.
                            //
                            if fstr::eq(&save.RELATE, b"=") {
                                //
                                // Check the solar incidence angle at each
                                // interval endpoint.
                                //
                                for I in 1..=save.N {
                                    spicelib::WNFETD(
                                        save.RESULT.as_slice(),
                                        I,
                                        &mut save.START,
                                        &mut save.FINISH,
                                        ctx,
                                    )?;
                                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                                    fstr::assign(&mut save.TITLE, b"Angle at start of interval #");
                                    spicelib::REPMI(
                                        &save.TITLE.to_vec(),
                                        b"#",
                                        I,
                                        &mut save.TITLE,
                                        ctx,
                                    );

                                    spicelib::ILUMIN(
                                        &save.METHOD,
                                        b"EARTH",
                                        save.START,
                                        &save.FIXREF,
                                        &save.ABCORR,
                                        &save.OBSRVR,
                                        save.SPOINT.as_slice(),
                                        &mut save.TRGEPC,
                                        save.SRFVEC.as_slice_mut(),
                                        &mut save.PHASE,
                                        &mut save.SOLAR,
                                        &mut save.EMISSN,
                                        ctx,
                                    )?;

                                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                                    testutil::CHCKSD(
                                        &save.TITLE,
                                        save.SOLAR,
                                        b"~",
                                        save.REFVAL,
                                        ANGTOL,
                                        OK,
                                        ctx,
                                    )?;

                                    fstr::assign(&mut save.TITLE, b"Angle at end of interval #");
                                    spicelib::REPMI(
                                        &save.TITLE.to_vec(),
                                        b"#",
                                        I,
                                        &mut save.TITLE,
                                        ctx,
                                    );

                                    spicelib::ILUMIN(
                                        &save.METHOD,
                                        b"EARTH",
                                        save.FINISH,
                                        &save.FIXREF,
                                        &save.ABCORR,
                                        &save.OBSRVR,
                                        save.SPOINT.as_slice(),
                                        &mut save.TRGEPC,
                                        save.SRFVEC.as_slice_mut(),
                                        &mut save.PHASE,
                                        &mut save.SOLAR,
                                        &mut save.EMISSN,
                                        ctx,
                                    )?;

                                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                                    testutil::CHCKSD(
                                        &save.TITLE,
                                        save.SOLAR,
                                        b"~",
                                        save.REFVAL,
                                        ANGTOL,
                                        OK,
                                        ctx,
                                    )?;
                                }
                            } else if (fstr::eq(&save.RELATE, b"<") || fstr::eq(&save.RELATE, b">"))
                            {
                                //
                                // Perform checks only at interval endpoints
                                // contained in the interior of the confinement
                                // window. At the confinement window boundaries,
                                // the inequality may be satisfied without the
                                // value being equal to the reference value.
                                //
                                for I in 1..=save.N {
                                    spicelib::WNFETD(
                                        save.RESULT.as_slice(),
                                        I,
                                        &mut save.START,
                                        &mut save.FINISH,
                                        ctx,
                                    )?;
                                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                                    fstr::assign(&mut save.TITLE, b"Angle at start of interval #");
                                    spicelib::REPMI(
                                        &save.TITLE.to_vec(),
                                        b"#",
                                        I,
                                        &mut save.TITLE,
                                        ctx,
                                    );

                                    spicelib::ILUMIN(
                                        &save.METHOD,
                                        b"EARTH",
                                        save.START,
                                        &save.FIXREF,
                                        &save.ABCORR,
                                        &save.OBSRVR,
                                        save.SPOINT.as_slice(),
                                        &mut save.TRGEPC,
                                        save.SRFVEC.as_slice_mut(),
                                        &mut save.PHASE,
                                        &mut save.SOLAR,
                                        &mut save.EMISSN,
                                        ctx,
                                    )?;

                                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                                    if (save.START > save.ET0) {
                                        testutil::CHCKSD(
                                            &save.TITLE,
                                            save.SOLAR,
                                            b"~",
                                            save.REFVAL,
                                            ANGTOL,
                                            OK,
                                            ctx,
                                        )?;
                                    }

                                    fstr::assign(&mut save.TITLE, b"Angle at end of interval #");
                                    spicelib::REPMI(
                                        &save.TITLE.to_vec(),
                                        b"#",
                                        I,
                                        &mut save.TITLE,
                                        ctx,
                                    );

                                    spicelib::ILUMIN(
                                        &save.METHOD,
                                        b"EARTH",
                                        save.FINISH,
                                        &save.FIXREF,
                                        &save.ABCORR,
                                        &save.OBSRVR,
                                        save.SPOINT.as_slice(),
                                        &mut save.TRGEPC,
                                        save.SRFVEC.as_slice_mut(),
                                        &mut save.PHASE,
                                        &mut save.SOLAR,
                                        &mut save.EMISSN,
                                        ctx,
                                    )?;

                                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                                    if (save.FINISH < save.ET1) {
                                        testutil::CHCKSD(
                                            &save.TITLE,
                                            save.SOLAR,
                                            b"~",
                                            save.REFVAL,
                                            ANGTOL,
                                            OK,
                                            ctx,
                                        )?;
                                    }
                                }
                                //
                                // End of solar incidence angle value check
                                // loop.
                                //
                            }
                            //
                            // End of solar incidence angle checks.
                            //
                        }
                        //
                        // End of IF block for successful RESULT check.
                        //
                    }
                    //
                    // End of interval endpoint checks.
                    //
                }
                //
                // End of IF block for successful GFILUM call.
                //
            }
            //
            // End of IF block for successful GFPOSC call.
            //
        }
        //
        // End of aberration correction loop.
        //
    }
    //
    // End of operator loop.
    //

    //
    // Case
    //
    fstr::assign(&mut save.TITLE, b"Check the GF call uses the GFSTOL value");
    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // Re-run a valid search after using GFSTOL to set the convergence
    // tolerance to a value that should cause a numerical error signal.
    //

    fstr::assign(&mut save.ANGTYP, b"EMISSION");
    fstr::assign(&mut save.RELATE, b"locmin");
    save.REFVAL = 0.0;
    save.ADJUST = 0.0;
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.TARGET, b" ALPHA");
    fstr::assign(&mut save.FIXREF, b" ALPHAFIXED");
    fstr::assign(&mut save.OBSRVR, b" SUN");
    fstr::assign(&mut save.ILLUM, b" SUN");
    fstr::assign(&mut save.METHOD, b"Ellipsoid");
    save.STEP = 3600.0;

    save.LEFT = 0.0;
    save.RIGHT = (10.0 * spicelib::SPD());

    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
    spicelib::WNINSD(save.LEFT, save.RIGHT, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        &save.ILLUM,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.COUNT = 0;
    save.COUNT = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKSI(b"COUNT", save.COUNT, b"!=", 0, 0, OK, ctx)?;

    spicelib::WNFETD(save.RESULT.as_slice(), 1, &mut save.BEG, &mut save.END, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Reset tol.
    //

    spicelib::GFSTOL(0.0001, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFILUM(
        &save.METHOD,
        &save.ANGTYP,
        &save.TARGET,
        &save.ILLUM,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWILUM,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.COUNT = 0;
    save.COUNT = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKSI(b"COUNT", save.COUNT, b"!=", 0, 0, OK, ctx)?;

    spicelib::WNFETD(
        save.RESULT.as_slice(),
        1,
        &mut save.LEFT,
        &mut save.RIGHT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The values in the time window should not match
    // as the search used different tolerances. Check
    // the first value in the first interval.
    //
    testutil::CHCKSD(&save.TITLE, save.BEG, b"!=", save.LEFT, 0.0, OK, ctx)?;

    //
    // Reset the convergence tolerance.
    //
    spicelib::GFSTOL(CNVTOL, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Clean up. Unload and delete kernels.", ctx)?;

    //
    // Clean up SPK files.
    //
    spicelib::SPKUEF(save.HAN1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKUEF(save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(OCTSPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(OCTSPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
