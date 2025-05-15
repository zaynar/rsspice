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
const PCK: &[u8] = b"zzgfcslv.tpc";
const NPCK: &[u8] = b"nat.tpc";
const NSPK: &[u8] = b"nat.bsp";
const SPK: &[u8] = b"zzgfcslv.bsp";
const TIGHT: f64 = 0.00000001;
const MEDABS: f64 = 0.000001;
const TIMTOL: f64 = 0.00001;
const TDELTA: f64 = 120.0;
const BDNMLN: i32 = 36;
const LNSIZE: i32 = 200;
const FRNMLN: i32 = 32;
const TIMLEN: i32 = 50;
const SYSLEN: i32 = 32;
const CRDLEN: i32 = 32;
const VDFLEN: i32 = 32;
const MTHLEN: i32 = 80;
const MAXWIN: i32 = (2 * 1000);
const MW: i32 = MAXWIN;
const NW: i32 = NWMAX;
const LBCELL: i32 = -5;
const OPLEN: i32 = 40;

struct SaveVars {
    ABCORR: Vec<u8>,
    CRDNAM: Vec<u8>,
    CRDSYS: Vec<u8>,
    DREF: Vec<u8>,
    REF: Vec<u8>,
    RELATE: Vec<u8>,
    METHOD: Vec<u8>,
    OBSRVR: Vec<u8>,
    QNAME: Vec<u8>,
    TARGET: Vec<u8>,
    TIMSTR: Vec<u8>,
    VECDEF: Vec<u8>,
    ADJUST: f64,
    CNFINE: ActualArray<f64>,
    DEC: f64,
    DVEC: StackArray<f64, 3>,
    ET: f64,
    ET0: f64,
    ET1: f64,
    ET2: f64,
    FINISH: f64,
    LATS: StackArray<f64, 3>,
    LON: f64,
    LT: f64,
    MAXADJ: f64,
    MAXEPC: f64,
    MINADJ: f64,
    MINEPC: f64,
    QTOL: f64,
    RADII: StackArray<f64, 3>,
    POS: StackArray<f64, 3>,
    RA: f64,
    RA0: f64,
    RA1: f64,
    REFVAL: f64,
    RESULT: ActualArray<f64>,
    RESLT2: ActualArray<f64>,
    RESLT3: ActualArray<f64>,
    SRFVEC: StackArray<f64, 3>,
    START: f64,
    STATE: StackArray<f64, 6>,
    TOL: f64,
    TRGEPC: f64,
    LAT: f64,
    WORK: ActualArray2D<f64>,
    R: f64,
    XTIME: f64,
    HAN2: i32,
    HANDLE: i32,
    I: i32,
    N: i32,
    OBSID: i32,
    TRGID: i32,
    XN: i32,
    BAIL: bool,
    FOUND: bool,
    RPT: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ABCORR = vec![b' '; LNSIZE as usize];
        let mut CRDNAM = vec![b' '; CRDLEN as usize];
        let mut CRDSYS = vec![b' '; SYSLEN as usize];
        let mut DREF = vec![b' '; FRNMLN as usize];
        let mut REF = vec![b' '; FRNMLN as usize];
        let mut RELATE = vec![b' '; OPLEN as usize];
        let mut METHOD = vec![b' '; MTHLEN as usize];
        let mut OBSRVR = vec![b' '; BDNMLN as usize];
        let mut QNAME = vec![b' '; LNSIZE as usize];
        let mut TARGET = vec![b' '; BDNMLN as usize];
        let mut TIMSTR = vec![b' '; TIMLEN as usize];
        let mut VECDEF = vec![b' '; VDFLEN as usize];
        let mut ADJUST: f64 = 0.0;
        let mut CNFINE = ActualArray::<f64>::new(LBCELL..=MW);
        let mut DEC: f64 = 0.0;
        let mut DVEC = StackArray::<f64, 3>::new(1..=3);
        let mut ET: f64 = 0.0;
        let mut ET0: f64 = 0.0;
        let mut ET1: f64 = 0.0;
        let mut ET2: f64 = 0.0;
        let mut FINISH: f64 = 0.0;
        let mut LATS = StackArray::<f64, 3>::new(1..=3);
        let mut LON: f64 = 0.0;
        let mut LT: f64 = 0.0;
        let mut MAXADJ: f64 = 0.0;
        let mut MAXEPC: f64 = 0.0;
        let mut MINADJ: f64 = 0.0;
        let mut MINEPC: f64 = 0.0;
        let mut QTOL: f64 = 0.0;
        let mut RADII = StackArray::<f64, 3>::new(1..=3);
        let mut POS = StackArray::<f64, 3>::new(1..=3);
        let mut RA: f64 = 0.0;
        let mut RA0: f64 = 0.0;
        let mut RA1: f64 = 0.0;
        let mut REFVAL: f64 = 0.0;
        let mut RESULT = ActualArray::<f64>::new(LBCELL..=MW);
        let mut RESLT2 = ActualArray::<f64>::new(LBCELL..=MW);
        let mut RESLT3 = ActualArray::<f64>::new(LBCELL..=MW);
        let mut SRFVEC = StackArray::<f64, 3>::new(1..=3);
        let mut START: f64 = 0.0;
        let mut STATE = StackArray::<f64, 6>::new(1..=6);
        let mut TOL: f64 = 0.0;
        let mut TRGEPC: f64 = 0.0;
        let mut LAT: f64 = 0.0;
        let mut WORK = ActualArray2D::<f64>::new(LBCELL..=MW, 1..=NW);
        let mut R: f64 = 0.0;
        let mut XTIME: f64 = 0.0;
        let mut HAN2: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut I: i32 = 0;
        let mut N: i32 = 0;
        let mut OBSID: i32 = 0;
        let mut TRGID: i32 = 0;
        let mut XN: i32 = 0;
        let mut BAIL: bool = false;
        let mut FOUND: bool = false;
        let mut RPT: bool = false;

        Self {
            ABCORR,
            CRDNAM,
            CRDSYS,
            DREF,
            REF,
            RELATE,
            METHOD,
            OBSRVR,
            QNAME,
            TARGET,
            TIMSTR,
            VECDEF,
            ADJUST,
            CNFINE,
            DEC,
            DVEC,
            ET,
            ET0,
            ET1,
            ET2,
            FINISH,
            LATS,
            LON,
            LT,
            MAXADJ,
            MAXEPC,
            MINADJ,
            MINEPC,
            QTOL,
            RADII,
            POS,
            RA,
            RA0,
            RA1,
            REFVAL,
            RESULT,
            RESLT2,
            RESLT3,
            SRFVEC,
            START,
            STATE,
            TOL,
            TRGEPC,
            LAT,
            WORK,
            R,
            XTIME,
            HAN2,
            HANDLE,
            I,
            N,
            OBSID,
            TRGID,
            XN,
            BAIL,
            FOUND,
            RPT,
        }
    }
}

//$Procedure      F_ZZGFCSLV ( Test GF coordinate solver )
pub fn F_ZZGFCSLV(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // EXTERNAL declarations
    //

    //
    // Local parameters
    //
    //
    // Local parameters
    //

    //
    // Maximum length of a coordinate system name.
    //

    //
    // Maximum length of a coordinate name.
    //

    //
    // Local variables
    //
    // CHARACTER*(LNSIZE)    TITLE

    //
    // Save everything.
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZGFCSLV", ctx)?;

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
    // Create and load Nat's solar system SPK and PCK/FK
    // kernels.
    //
    testutil::NATPCK(NPCK, true, false, ctx)?;
    testutil::NATSPK(NSPK, true, &mut save.HAN2, ctx)?;

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
    // The following error cases involve invalid initialization
    // values or missing data discovered at search time.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad observer or target name.", ctx)?;

    fstr::assign(&mut save.ABCORR, b"CN+S");
    fstr::assign(&mut save.VECDEF, SINDEF);
    fstr::assign(&mut save.METHOD, b"ellipsoid");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.REF, b"IAU_EARTH");
    fstr::assign(&mut save.DREF, &save.REF);
    fstr::assign(&mut save.CRDSYS, b"LATITUDINAL");
    fstr::assign(&mut save.CRDNAM, b"LATITUDE");

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

    fstr::assign(&mut save.TARGET, b"X");

    fstr::assign(&mut save.RELATE, b"=");
    save.REFVAL = 0.0;
    save.TOL = CNVTOL;
    save.ADJUST = 0.0;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;

    save.ET0 = (save.ET - spicelib::SPD());
    save.ET1 = (save.ET + spicelib::SPD());

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.RPT = false;
    save.BAIL = false;

    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"Y");

    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad window count", ctx)?;

    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");

    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        0,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(TOOFEWWINDOWS)", OK, ctx)?;

    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        14,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(TOOFEWWINDOWS)", OK, ctx)?;

    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        -1,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(TOOFEWWINDOWS)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad window size", ctx)?;

    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");

    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        0,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(WINDOWSTOOSMALL)", OK, ctx)?;

    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        1,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(WINDOWSTOOSMALL)", OK, ctx)?;

    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        -1,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(WINDOWSTOOSMALL)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Observer equals target", ctx)?;

    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"EARTH");

    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Negative adjustment value", ctx)?;

    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");

    save.ADJUST = -1.0;

    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Non-positive tolerance value", ctx)?;

    //
    // Try both zero and negative tolerance values.
    //
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");

    save.ADJUST = 0.0;
    save.TOL = 0.0;

    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    save.TOL = -1.0;

    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // Reset TOL to something valid.
    //
    save.TOL = 0.001;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad vector definition", ctx)?;

    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.ABCORR, b"None");
    fstr::assign(&mut save.METHOD, b"Near point");

    fstr::assign(&mut save.VECDEF, b"ACCELERATION");

    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad computation method", ctx)?;

    //
    // This error is detected post-initialization.
    //
    fstr::assign(&mut save.VECDEF, SINDEF);
    fstr::assign(&mut save.ABCORR, b"None");
    fstr::assign(&mut save.METHOD, b"Near point");

    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad aberration correction", ctx)?;

    fstr::assign(&mut save.VECDEF, SINDEF);
    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");

    fstr::assign(&mut save.ABCORR, b"None.");

    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad coordinate system", ctx)?;

    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.ABCORR, b"XCN+S");

    fstr::assign(&mut save.CRDSYS, b"GAUSSIAN");

    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad coordinate name", ctx)?;

    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.ABCORR, b"XCN+S");

    fstr::assign(&mut save.CRDSYS, b"CYLINDRICAL");
    fstr::assign(&mut save.CRDNAM, b"LATITUDE");

    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad reference frame REF", ctx)?;

    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.ABCORR, b"XCN+S");

    fstr::assign(&mut save.CRDSYS, b"CYLINDRICAL");
    fstr::assign(&mut save.CRDNAM, b"Z");

    fstr::assign(&mut save.REF, b"EME2000");

    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SINDEF: Reference frame REF not centered on target when VECDEF requires it.",
        ctx,
    )?;

    fstr::assign(&mut save.VECDEF, SOBDEF);
    fstr::assign(&mut save.METHOD, b"NEAR POINT: ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"Mars");
    fstr::assign(&mut save.REF, b"IAU_EARTH");

    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDFRAME)", OK, ctx)?;

    fstr::assign(&mut save.VECDEF, SINDEF);
    fstr::assign(&mut save.METHOD, b"ELLIPSOID");

    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SINDEF: Non-existent reference frame DREF", ctx)?;

    fstr::assign(&mut save.VECDEF, SINDEF);
    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.ABCORR, b"XCN+S");

    fstr::assign(&mut save.CRDSYS, b"CYLINDRICAL");
    fstr::assign(&mut save.CRDNAM, b"Z");

    fstr::assign(&mut save.REF, b"IAU_EARTH");
    fstr::assign(&mut save.DREF, b"EME2000");

    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad vector DVEC", ctx)?;

    fstr::assign(&mut save.VECDEF, SINDEF);
    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.ABCORR, b"XCN+S");

    fstr::assign(&mut save.CRDSYS, b"RECTANGULAR");
    fstr::assign(&mut save.CRDNAM, b"Z");

    fstr::assign(&mut save.REF, b"IAU_EARTH");
    fstr::assign(&mut save.DREF, b"J2000");

    spicelib::CLEARD(3, save.DVEC.as_slice_mut());

    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(ZEROVECTOR)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad target radii count", ctx)?;

    spicelib::VPACK(0.0, 0.0, 1.0, save.DVEC.as_slice_mut());

    fstr::assign(&mut save.VECDEF, SOBDEF);
    fstr::assign(&mut save.CRDSYS, b"LATITUDINAL");
    fstr::assign(&mut save.CRDNAM, b"LATITUDE");

    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.REF, b"IAU_EARTH");
    fstr::assign(&mut save.ABCORR, b"None");
    fstr::assign(&mut save.DREF, b"J2000");

    spicelib::GDPOOL(
        b"BODY399_RADII",
        1,
        1,
        &mut save.N,
        save.RADII.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(b"BODY399_RADII", 2, save.RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDCOUNT)", OK, ctx)?;

    //
    // Restore all three radii.
    //
    spicelib::PDPOOL(b"BODY399_RADII", 3, save.RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No target orientation data available", ctx)?;

    //
    // This error is detected post-initialization.
    //
    fstr::assign(&mut save.VECDEF, SINDEF);
    fstr::assign(&mut save.METHOD, b"ellipsoid");
    fstr::assign(&mut save.REF, b"ITRF93");

    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(FRAMEDATANOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No target ephemeris data available", ctx)?;
    //
    // This error is detected post-initialization.
    //

    fstr::assign(&mut save.TARGET, b"GASPRA");
    save.TRGID = 9511010;
    fstr::assign(&mut save.REF, b"IAU_GASPRA");

    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No observer ephemeris data available", ctx)?;
    //
    // This error is detected post-initialization.
    //

    fstr::assign(&mut save.TARGET, b"MOON");
    save.TRGID = 301;
    fstr::assign(&mut save.OBSRVR, b"GASPRA");
    save.OBSID = 9511010;
    fstr::assign(&mut save.REF, b"IAU_MOON");

    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No DREF ephemeris data available", ctx)?;
    //
    // This error is detected post-initialization.
    //

    fstr::assign(&mut save.DREF, b"IAU_GASPRA");
    spicelib::VPACK(0.0, 0.0, 1.0, save.DVEC.as_slice_mut());

    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No DREF orientation data available", ctx)?;
    //
    // This error is detected post-initialization.
    //

    fstr::assign(&mut save.VECDEF, SINDEF);
    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"EARTH");
    save.TRGID = 399;
    fstr::assign(&mut save.OBSRVR, b"MOON");
    save.OBSID = 301;
    fstr::assign(&mut save.REF, b"IAU_EARTH");
    fstr::assign(&mut save.DREF, b"ITRF93");
    spicelib::VPACK(0.0, 0.0, 1.0, save.DVEC.as_slice_mut());

    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(FRAMEDATANOTFOUND)", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Normal cases --- Nat's solar system
    //*
    //*********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // We're going to find times when the Alpha-Gamma vector crosses the
    // J2000 X-Y plane. This should occur every 12 hours, starting
    // at the J2000 epoch.
    //
    testutil::TCASE(b"Find times when Gamma crosses the J2000 x-y plane.", ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESLT2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 JAN 1 3:00 TDB", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 JAN 3 3:00 TDB", &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.VECDEF, POSDEF);
    fstr::assign(&mut save.METHOD, b" ");
    fstr::assign(&mut save.TARGET, b"GAMMA");
    fstr::assign(&mut save.OBSRVR, b"ALPHA");
    fstr::assign(&mut save.REF, b"J2000");
    fstr::assign(&mut save.DREF, b" ");
    fstr::assign(&mut save.CRDSYS, b"rectangular");
    fstr::assign(&mut save.CRDNAM, b"z");

    spicelib::CLEARD(3, save.DVEC.as_slice_mut());

    fstr::assign(&mut save.RELATE, b"=");
    save.REFVAL = 0.0;
    save.TOL = CNVTOL;
    save.ADJUST = 0.0;

    save.RPT = false;
    save.BAIL = false;

    //
    // Set the search step size.
    //
    spicelib::GFSSTP(10000.0, ctx)?;

    //
    // Perform the search.
    //
    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure we found 4 roots.
    //
    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKSI(b"N", save.N, b"=", 4, 0, OK, ctx)?;

    //
    // Make sure that the start and finish times are
    // those we expect.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.XTIME = ((12.0 * 3600.0) * (save.I - 1) as f64);

            spicelib::WNFETD(
                save.RESULT.as_slice(),
                save.I,
                &mut save.START,
                &mut save.FINISH,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.QNAME, b"Interval start # (0)");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

            testutil::CHCKSD(&save.QNAME, save.START, b"~", save.XTIME, CNVTOL, OK, ctx)?;
            testutil::CHCKSD(&save.QNAME, save.FINISH, b"~", save.XTIME, CNVTOL, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Check the Z coordinate at the crossing time.
    //
    // In order to decide how accurate the coordinate should be,
    // we'll use the speed of Gamma together with the time
    // tolerance.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::WNFETD(
                save.RESULT.as_slice(),
                save.I,
                &mut save.START,
                &mut save.FINISH,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::SPKEZR(
                &save.TARGET,
                save.START,
                &save.REF,
                &save.ABCORR,
                &save.OBSRVR,
                save.STATE.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            if (save.I == 1) {
                save.QTOL = (spicelib::VNORM(save.STATE.subarray(4)) * CNVTOL);
            }

            fstr::assign(&mut save.QNAME, b"Z # (0)");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

            testutil::CHCKSD(&save.QNAME, save.STATE[3], b"~", 0.0, save.QTOL, OK, ctx)?;

            save.I += m3__;
        }
    }

    //*********************************************************************
    //*
    //*    Normal cases --- "real" solar system
    //*
    //*********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // We're going to find times when the Earth-Sun vector crosses the
    // Earth equator (the x-y plane of the IAU_EARTH frame). We'll then
    // check the planetocentric latitude of the vector at the result
    // times. We'll also perform the search looking for 0 position
    // Z-component and compare the results of that search.
    //

    testutil::TCASE(
        b"Find times when the Sun crosses the IAU_EARTH x-y plane.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESLT2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 JAN 1", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2001 JAN 1", &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.ABCORR, b"LT+S");
    fstr::assign(&mut save.VECDEF, POSDEF);
    fstr::assign(&mut save.METHOD, b" ");
    fstr::assign(&mut save.TARGET, b"10");
    fstr::assign(&mut save.OBSRVR, b"399");
    fstr::assign(&mut save.REF, b"IAU_EARTH");
    fstr::assign(&mut save.DREF, b" ");
    fstr::assign(&mut save.CRDSYS, b"latitudinal");
    fstr::assign(&mut save.CRDNAM, b"latitude");

    spicelib::CLEARD(3, save.DVEC.as_slice_mut());

    fstr::assign(&mut save.RELATE, b"=");
    save.REFVAL = 0.0;
    save.TOL = CNVTOL;
    save.ADJUST = 0.0;

    save.RPT = false;
    save.BAIL = false;

    //
    // Set the search step size.
    //
    spicelib::GFSSTP(((100 as f64) * spicelib::SPD()), ctx)?;

    //
    // Perform the search.
    //
    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure we found at least one root.
    //
    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

    testutil::CHCKSI(b"N (0)", save.N, b">", 0, 0, OK, ctx)?;

    //
    // Make sure that the start and finish times are
    // close together.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::WNFETD(
                save.RESULT.as_slice(),
                save.I,
                &mut save.START,
                &mut save.FINISH,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.QNAME, b"Interval start # (0)");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

            testutil::CHCKSD(&save.QNAME, save.START, b"~", save.FINISH, CNVTOL, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Check the planetocentric latitude at the crossing time.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::WNFETD(
                save.RESULT.as_slice(),
                save.I,
                &mut save.START,
                &mut save.FINISH,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::SPKPOS(
                &save.TARGET,
                save.START,
                &save.REF,
                &save.ABCORR,
                &save.OBSRVR,
                save.POS.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::RECLAT(
                save.POS.as_slice(),
                &mut save.R,
                &mut save.LON,
                &mut save.LAT,
            );

            fstr::assign(&mut save.QNAME, b"Latitude # (0)");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

            testutil::CHCKSD(&save.QNAME, save.LAT, b"~", 0.0, MEDABS, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // We're going to find times when the Earth-Sun vector attains
    // local minima and maxima.
    //
    testutil::TCASE(
        b"Find local min/max of Solar latitude as seen from Earth.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESLT2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 JAN 1", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2005 JAN 1", &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.ABCORR, b"LT+S");
    fstr::assign(&mut save.VECDEF, POSDEF);
    fstr::assign(&mut save.METHOD, b" ");
    fstr::assign(&mut save.TARGET, b"SUN");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.REF, b"IAU_EARTH");
    fstr::assign(&mut save.DREF, b" ");
    fstr::assign(&mut save.CRDSYS, b"latitudinal");
    fstr::assign(&mut save.CRDNAM, b"latitude");

    spicelib::CLEARD(3, save.DVEC.as_slice_mut());

    fstr::assign(&mut save.RELATE, b"LOCMAX");
    save.REFVAL = 0.0;
    save.TOL = CNVTOL;
    save.ADJUST = 0.0;

    save.RPT = false;
    save.BAIL = false;

    //
    // Set the search step size.
    //
    spicelib::GFSSTP(((100 as f64) * spicelib::SPD()), ctx)?;

    //
    // Perform the search.
    //
    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure we found five roots.
    //
    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

    testutil::CHCKSI(b"N (0)", save.N, b"=", 5, 0, OK, ctx)?;

    //
    // Make sure that the start and finish times are
    // close together.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::WNFETD(
                save.RESULT.as_slice(),
                save.I,
                &mut save.START,
                &mut save.FINISH,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.QNAME, b"Interval start # (0)");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

            testutil::CHCKSD(&save.QNAME, save.START, b"~", save.FINISH, CNVTOL, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Check the planetocentric latitude at the crossing time. We
    // should find the latitude is greater than that at +/-
    // TDELTA seconds from each interval start time.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::WNFETD(
                save.RESULT.as_slice(),
                save.I,
                &mut save.START,
                &mut save.FINISH,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            for J in 1..=3 {
                save.ET = (save.START + (((J - 2) as f64) * TDELTA));

                spicelib::SPKPOS(
                    &save.TARGET,
                    save.ET,
                    &save.REF,
                    &save.ABCORR,
                    &save.OBSRVR,
                    save.POS.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::RECLAT(
                    save.POS.as_slice(),
                    &mut save.R,
                    &mut save.LON,
                    &mut save.LATS[J],
                );
            }

            // WRITE (*, '(I10,3E24.16)'  ) I, LATS

            fstr::assign(&mut save.QNAME, b"Max: Left side latitude #");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

            testutil::CHCKSD(
                &save.QNAME,
                save.LATS[1],
                b"<",
                save.LATS[2],
                MEDABS,
                OK,
                ctx,
            )?;

            fstr::assign(&mut save.QNAME, b"Max: Right side latitude #");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

            testutil::CHCKSD(
                &save.QNAME,
                save.LATS[3],
                b"<",
                save.LATS[2],
                MEDABS,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // Now test using local minima.
    //
    fstr::assign(&mut save.RELATE, b"locmin");

    //
    // Perform the search.
    //
    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure we found five roots.
    //
    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

    testutil::CHCKSI(b"N (0)", save.N, b"=", 5, 0, OK, ctx)?;

    //
    // Make sure that the start and finish times are
    // close together.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::WNFETD(
                save.RESULT.as_slice(),
                save.I,
                &mut save.START,
                &mut save.FINISH,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.QNAME, b"Interval start # (0)");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

            testutil::CHCKSD(&save.QNAME, save.START, b"~", save.FINISH, CNVTOL, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Check the planetocentric latitude at the crossing time. We
    // should find the latitude is greater than that at +/-
    // TDELTA seconds from each interval start time.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::WNFETD(
                save.RESULT.as_slice(),
                save.I,
                &mut save.START,
                &mut save.FINISH,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            for J in 1..=3 {
                save.ET = (save.START + (((J - 2) as f64) * TDELTA));

                spicelib::SPKPOS(
                    &save.TARGET,
                    save.ET,
                    &save.REF,
                    &save.ABCORR,
                    &save.OBSRVR,
                    save.POS.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::RECLAT(
                    save.POS.as_slice(),
                    &mut save.R,
                    &mut save.LON,
                    &mut save.LATS[J],
                );
            }

            // WRITE (*, '(I10,3E24.16)'  ) I, LATS

            fstr::assign(&mut save.QNAME, b"Min: Left side latitude #");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

            testutil::CHCKSD(
                &save.QNAME,
                save.LATS[1],
                b">",
                save.LATS[2],
                MEDABS,
                OK,
                ctx,
            )?;

            fstr::assign(&mut save.QNAME, b"Min: Right side latitude #");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

            testutil::CHCKSD(
                &save.QNAME,
                save.LATS[3],
                b">",
                save.LATS[2],
                MEDABS,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(
        b"Find times when the Sun crosses the J2000 prime meridian.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESLT2.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESLT3.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 JAN 1", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2001 JAN 1", &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.VECDEF, POSDEF);
    fstr::assign(&mut save.METHOD, b" ");
    fstr::assign(&mut save.TARGET, b"SUN");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.REF, b"J2000");
    fstr::assign(&mut save.DREF, b" ");
    fstr::assign(&mut save.CRDSYS, b"Latitudinal");
    fstr::assign(&mut save.CRDNAM, b"LONGITUDE");

    spicelib::CLEARD(3, save.DVEC.as_slice_mut());

    fstr::assign(&mut save.RELATE, b"=");
    save.REFVAL = 0.0;
    save.TOL = CNVTOL;
    save.ADJUST = 0.0;

    save.RPT = false;
    save.BAIL = false;

    //
    // Set the search step size.
    //
    spicelib::GFSSTP(((100 as f64) * spicelib::SPD()), ctx)?;
    spicelib::GFSSTP(((10 as f64) * spicelib::SPD()), ctx)?;

    //
    // Perform the search.
    //
    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure we found at least one root.
    //
    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

    testutil::CHCKSI(b"N (0)", save.N, b">", 0, 0, OK, ctx)?;

    //
    // Make sure that the start and finish times are
    // close together.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::WNFETD(
                save.RESULT.as_slice(),
                save.I,
                &mut save.START,
                &mut save.FINISH,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.QNAME, b"Interval start # (0)");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

            testutil::CHCKSD(&save.QNAME, save.START, b"~", save.FINISH, CNVTOL, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Check the longitude at the crossing time.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::WNFETD(
                save.RESULT.as_slice(),
                save.I,
                &mut save.START,
                &mut save.FINISH,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::SPKPOS(
                &save.TARGET,
                save.START,
                &save.REF,
                &save.ABCORR,
                &save.OBSRVR,
                save.POS.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::RECLAT(
                save.POS.as_slice(),
                &mut save.R,
                &mut save.LON,
                &mut save.LAT,
            );

            fstr::assign(&mut save.QNAME, b"Longitude # (0)");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

            testutil::CHCKSD(&save.QNAME, save.LON, b"~", 0.0, MEDABS, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Now find times when the Sun crosses the J2000 x-z plane.
    // Delete times when the X coordinate is negative. The
    // result should match that from the longitude search.
    //
    fstr::assign(&mut save.CRDSYS, b"rectangular");
    fstr::assign(&mut save.CRDNAM, b"Y");
    save.REFVAL = 0.0;

    //
    // Perform the search.
    //
    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESLT2.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Empty the window RESLT3.
    //
    spicelib::SCARDD(0, save.RESLT3.as_slice_mut(), ctx)?;

    //
    // Copy the times corresponding to positive X-values to RESLT3.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = spicelib::WNCARD(save.RESLT2.as_slice(), ctx)?;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::WNFETD(
                save.RESLT2.as_slice(),
                save.I,
                &mut save.START,
                &mut save.FINISH,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::SPKPOS(
                &save.TARGET,
                save.START,
                &save.REF,
                &save.ABCORR,
                &save.OBSRVR,
                save.POS.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;

            if (save.POS[1] > 0.0) {
                spicelib::WNINSD(save.START, save.FINISH, save.RESLT3.as_slice_mut(), ctx)?;
            }

            save.I += m3__;
        }
    }

    //
    // Make sure we found at as many roots as
    // we did on the previous search.
    //
    save.XN = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    save.N = spicelib::WNCARD(save.RESLT3.as_slice(), ctx)?;

    testutil::CHCKSI(b"N (1)", save.N, b"=", save.XN, 0, OK, ctx)?;

    //
    // Check the solution times against those from the
    // Z-component search.
    //
    testutil::CHCKAD(
        b"RESLT3",
        save.RESLT3.as_slice(),
        b"~",
        save.RESULT.as_slice(),
        save.N,
        TIMTOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // We're going to find times when the sub-solar point
    // crosses the Earth equator (the x-y plane of the
    // IAU_EARTH frame). We'll then check the Z component
    // of the vector at the result times. We'll also
    // perform the search looking for 0 geodetic latitude
    // and compare the results of that search.
    //

    testutil::TCASE(b"Find times when the sub-solar point on the Earth (near point definition) crosses the IAU_EARTH x-y plane.", ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESLT2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 JAN 1", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2001 JAN 1", &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.ABCORR, b"LT+S");
    fstr::assign(&mut save.VECDEF, SOBDEF);
    fstr::assign(&mut save.METHOD, b"Near point: Ellipsoid");
    fstr::assign(&mut save.TARGET, b"399");
    fstr::assign(&mut save.OBSRVR, b"10");
    fstr::assign(&mut save.REF, b"IAU_EARTH");
    fstr::assign(&mut save.DREF, b" ");
    fstr::assign(&mut save.CRDSYS, b"Geodetic");
    fstr::assign(&mut save.CRDNAM, b"latitude");

    spicelib::CLEARD(3, save.DVEC.as_slice_mut());

    fstr::assign(&mut save.RELATE, b"=");
    save.REFVAL = 0.0;
    save.TOL = CNVTOL;
    save.ADJUST = 0.0;

    save.RPT = false;
    save.BAIL = false;

    //
    // Set the search step size.
    //
    spicelib::GFSSTP(((100 as f64) * spicelib::SPD()), ctx)?;

    //
    // Perform the search.
    //
    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure we found at least one root.
    //
    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

    testutil::CHCKSI(b"N (0)", save.N, b">", 0, 0, OK, ctx)?;

    //
    // Make sure that the start and finish times are
    // close together.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::WNFETD(
                save.RESULT.as_slice(),
                save.I,
                &mut save.START,
                &mut save.FINISH,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.QNAME, b"Interval start # (0)");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

            testutil::CHCKSD(&save.QNAME, save.START, b"~", save.FINISH, CNVTOL, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Check the latitude at the crossing time.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::WNFETD(
                save.RESULT.as_slice(),
                save.I,
                &mut save.START,
                &mut save.FINISH,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::SPKPOS(
                &save.TARGET,
                save.START,
                &save.REF,
                &save.ABCORR,
                &save.OBSRVR,
                save.POS.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::RECLAT(
                save.POS.as_slice(),
                &mut save.R,
                &mut save.LON,
                &mut save.LAT,
            );

            fstr::assign(&mut save.QNAME, b"Latitude # (0)");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

            testutil::CHCKSD(&save.QNAME, save.LAT, b"~", 0.0, MEDABS, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Repeat the search using Cartesian coordinates.
    //
    fstr::assign(&mut save.CRDSYS, b"Rectangular");
    fstr::assign(&mut save.CRDNAM, b"z");

    //
    // Perform the search.
    //
    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESLT2.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure we found at as many roots as
    // we did on the previous search.
    //
    save.XN = save.N;
    save.N = spicelib::WNCARD(save.RESLT2.as_slice(), ctx)?;

    testutil::CHCKSI(b"N (1)", save.N, b"=", save.XN, 0, OK, ctx)?;

    //
    // Check the solution times against those from the
    // Z-component search.
    //
    testutil::CHCKAD(
        b"RESLT2",
        save.RESLT2.as_slice(),
        b"~",
        save.RESULT.as_slice(),
        save.N,
        TIMTOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Find times when the sub-solar point on the Earth (near point definition) crosses the IAU_EARTH prime meridian.", ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESLT2.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESLT3.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We use a time period of a few days, not a year.
    //
    spicelib::STR2ET(b"2000 JAN 1", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 JAN 5", &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.ABCORR, b"lt+s");
    fstr::assign(&mut save.VECDEF, SOBDEF);
    fstr::assign(&mut save.METHOD, b"near point: ellipsoid");
    fstr::assign(&mut save.TARGET, b"Earth");
    fstr::assign(&mut save.OBSRVR, b"Sun");
    fstr::assign(&mut save.REF, b"Iau_Earth");
    fstr::assign(&mut save.DREF, b" ");
    fstr::assign(&mut save.CRDSYS, b"Latitudinal");
    fstr::assign(&mut save.CRDNAM, b"Longitude");

    spicelib::CLEARD(3, save.DVEC.as_slice_mut());

    fstr::assign(&mut save.RELATE, b"=");
    save.REFVAL = 0.0;
    save.TOL = CNVTOL;
    save.ADJUST = 0.0;

    save.RPT = false;
    save.BAIL = false;

    //
    // Set the search step size. We need a size smaller than
    // one half day.
    //
    spicelib::GFSSTP(((8 as f64) * 3600.0), ctx)?;

    //
    // Perform the search.
    //
    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure we found at least one root.
    //
    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

    testutil::CHCKSI(b"N (0)", save.N, b">", 0, 0, OK, ctx)?;

    //
    // Make sure that the start and finish times are
    // close together.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::WNFETD(
                save.RESULT.as_slice(),
                save.I,
                &mut save.START,
                &mut save.FINISH,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.QNAME, b"Interval start # (0)");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

            testutil::CHCKSD(&save.QNAME, save.START, b"~", save.FINISH, CNVTOL, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Check the longitude at the crossing time.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::WNFETD(
                save.RESULT.as_slice(),
                save.I,
                &mut save.START,
                &mut save.FINISH,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::SUBPNT(
                &save.METHOD,
                &save.TARGET,
                save.START,
                &save.REF,
                &save.ABCORR,
                &save.OBSRVR,
                save.POS.as_slice_mut(),
                &mut save.TRGEPC,
                save.SRFVEC.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::RECLAT(
                save.POS.as_slice(),
                &mut save.R,
                &mut save.LON,
                &mut save.LAT,
            );

            fstr::assign(&mut save.QNAME, b"Longitude # (0)");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

            testutil::CHCKSD(&save.QNAME, save.LON, b"~", 0.0, MEDABS, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Now find times when the sub-solar point crosses the IAU_EARTH x-z
    // plane. Delete times when the X coordinate is negative. The result
    // should match that from the longitude search.
    //
    fstr::assign(&mut save.CRDSYS, b"rectangular");
    fstr::assign(&mut save.CRDNAM, b"y");

    //
    // Perform the search.
    //
    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESLT2.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Empty the window RESLT3.
    //
    spicelib::SCARDD(0, save.RESLT3.as_slice_mut(), ctx)?;

    //
    // Copy the times corresponding to positive X-values to RESLT3.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = spicelib::WNCARD(save.RESLT2.as_slice(), ctx)?;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::WNFETD(
                save.RESLT2.as_slice(),
                save.I,
                &mut save.START,
                &mut save.FINISH,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::SPKPOS(
                &save.TARGET,
                save.START,
                &save.REF,
                &save.ABCORR,
                &save.OBSRVR,
                save.POS.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;

            if (save.POS[1] > 0.0) {
                spicelib::WNINSD(save.START, save.FINISH, save.RESLT3.as_slice_mut(), ctx)?;
            }

            save.I += m3__;
        }
    }

    //
    // Make sure we found at as many roots as
    // we did on the previous search.
    //
    save.XN = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    save.N = spicelib::WNCARD(save.RESLT3.as_slice(), ctx)?;

    testutil::CHCKSI(b"N (1)", save.N, b"=", save.XN, 0, OK, ctx)?;

    //
    // Check the solution times against those from the
    // Z-component search.
    //
    testutil::CHCKAD(
        b"RESLT3",
        save.RESLT3.as_slice(),
        b"~",
        save.RESULT.as_slice(),
        save.N,
        TIMTOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // We're going to find times when a selected Sun-Earth ray crosses
    // the Earth equator (the x-y plane of the IAU_EARTH frame). We'll
    // perform the search looking for 0 geodetic latitude and compare
    // the results of that search. We'll then check the Z component of
    // the vector at the result times.
    //

    testutil::TCASE(
        b"J2000-fixed Sun-Earth ray at 2000 Mar 21 crosses the IAU_EARTH x-y plane.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESLT2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 MAR 20", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 MAR 22", &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.ABCORR, b"LT+S");
    fstr::assign(&mut save.VECDEF, SINDEF);
    fstr::assign(&mut save.METHOD, b"Ellipsoid");
    fstr::assign(&mut save.TARGET, b"399");
    fstr::assign(&mut save.OBSRVR, b"10");
    fstr::assign(&mut save.REF, b"IAU_EARTH");
    fstr::assign(&mut save.DREF, b"J2000");
    fstr::assign(&mut save.CRDSYS, b"Geodetic");
    fstr::assign(&mut save.CRDNAM, b"latitude");

    //
    // Set DVEC.
    //
    spicelib::STR2ET(b"2000 MAR 21", &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

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

    fstr::assign(&mut save.RELATE, b"=");
    save.REFVAL = 0.0;
    save.TOL = CNVTOL;
    save.ADJUST = 0.0;

    save.RPT = false;
    save.BAIL = false;

    //
    // Set the search step size. We need a step small enough
    // to capture the existence window, which we expect to be
    // about 7 minutes long, based on the Earth's orbital speed
    // and the Earth's ellipsoidal radii.
    //
    spicelib::GFSSTP(200.0, ctx)?;

    //
    // Perform the search.
    //
    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure we found at least one root.
    //
    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

    testutil::CHCKSI(b"N (0)", save.N, b">", 0, 0, OK, ctx)?;

    //
    // Make sure that the start and finish times are
    // close together.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::WNFETD(
                save.RESULT.as_slice(),
                save.I,
                &mut save.START,
                &mut save.FINISH,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.QNAME, b"Interval start # (0)");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

            testutil::CHCKSD(&save.QNAME, save.START, b"~", save.FINISH, CNVTOL, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Check the latitude at the crossing time.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::WNFETD(
                save.RESULT.as_slice(),
                save.I,
                &mut save.START,
                &mut save.FINISH,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::SINCPT(
                &save.METHOD,
                &save.TARGET,
                save.START,
                &save.REF,
                &save.ABCORR,
                &save.OBSRVR,
                &save.DREF,
                save.DVEC.as_slice(),
                save.POS.as_slice_mut(),
                &mut save.TRGEPC,
                save.SRFVEC.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSL(b"Intercept FOUND (0)", save.FOUND, true, OK, ctx)?;

            //
            // Although we're working with geodetic coordinates, we
            // can use RECLAT for latitudes close to zero.
            //
            spicelib::RECLAT(
                save.POS.as_slice(),
                &mut save.R,
                &mut save.LON,
                &mut save.LAT,
            );

            spicelib::SINCPT(
                &save.METHOD,
                &save.TARGET,
                save.FINISH,
                &save.REF,
                &save.ABCORR,
                &save.OBSRVR,
                &save.DREF,
                save.DVEC.as_slice(),
                save.POS.as_slice_mut(),
                &mut save.TRGEPC,
                save.SRFVEC.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSL(b"Intercept FOUND (1)", save.FOUND, true, OK, ctx)?;

            spicelib::RECLAT(
                save.POS.as_slice(),
                &mut save.R,
                &mut save.LON,
                &mut save.LAT,
            );

            fstr::assign(&mut save.QNAME, b"Latitude # (0)");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

            testutil::CHCKSD(&save.QNAME, save.LAT, b"~", 0.0, MEDABS, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Repeat the search using Cartesian coordinates.
    //
    fstr::assign(&mut save.CRDSYS, b"Rectangular");
    fstr::assign(&mut save.CRDNAM, b"z");

    //
    // Perform the search.
    //
    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESLT2.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure we found at as many roots as
    // we did on the previous search.
    //
    save.XN = save.N;
    save.N = spicelib::WNCARD(save.RESLT2.as_slice(), ctx)?;

    testutil::CHCKSI(b"N (1)", save.N, b"=", save.XN, 0, OK, ctx)?;

    //
    // Check the solution times against those from the
    // Z-component search.
    //
    testutil::CHCKAD(
        b"RESLT2",
        save.RESLT2.as_slice(),
        b"~",
        save.RESULT.as_slice(),
        save.N,
        TIMTOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Using the same observer, target, ray, and time period from
    // the previous search, we'll find the times when the absolute
    // minimum and maximum intercept of right ascension occur.
    // Since orbital motion is a larger effect than the Earth's
    // spin in this case, the RA of the intercept is actually
    // increasing with time.
    //
    // Note: since the intercept longitude passes through 180 degrees
    // during the time span of this test, the absolute minimum and
    // absolute maximum longitude are both 180 degrees.
    //
    testutil::TCASE(
        b"J2000-fixed Sun-Earth ray at 2000 Mar 21 attains extreme RA values.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESLT2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 MAR 20", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 MAR 22", &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.ABCORR, b"LT+S");
    fstr::assign(&mut save.VECDEF, SINDEF);
    fstr::assign(&mut save.METHOD, b"Ellipsoid");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    fstr::assign(&mut save.REF, b"IAU_EARTH");
    fstr::assign(&mut save.DREF, b"J2000");
    fstr::assign(&mut save.CRDSYS, b"ra/dec");
    fstr::assign(&mut save.CRDNAM, b"right ascension");

    //
    // Set DVEC.
    //
    spicelib::STR2ET(b"2000 MAR 21", &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

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

    fstr::assign(&mut save.RELATE, b"ABSMIN");
    save.REFVAL = 0.0;
    save.TOL = CNVTOL;
    save.ADJUST = 0.0;

    save.RPT = false;
    save.BAIL = false;

    //
    // Set the search step size. We need a step small enough
    // to capture the existence window, which we expect to be
    // about 7 minutes long, based on the Earth's orbital speed
    // and the Earth's ellipsoidal radii.
    //
    spicelib::GFSSTP(200.0, ctx)?;

    //
    // Perform the search.
    //
    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure we found exactly one root.
    //
    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

    testutil::CHCKSI(b"N (0)", save.N, b"=", 1, 0, OK, ctx)?;

    //
    // Make sure that the start and finish times are
    // close together.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::WNFETD(
                save.RESULT.as_slice(),
                save.I,
                &mut save.START,
                &mut save.FINISH,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.QNAME, b"Interval start # (0)");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

            testutil::CHCKSD(&save.QNAME, save.START, b"~", save.FINISH, CNVTOL, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Save the epoch of the minimum.
    //
    save.MINEPC = save.RESULT[1];

    fstr::assign(&mut save.RELATE, b"ABSMAX");

    //
    // Now search for an absolute maximum.
    //
    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure we found exactly one root.
    //
    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

    testutil::CHCKSI(b"N (0)", save.N, b"=", 1, 0, OK, ctx)?;

    //
    // Make sure that the start and finish times are
    // close together.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::WNFETD(
                save.RESULT.as_slice(),
                save.I,
                &mut save.START,
                &mut save.FINISH,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.QNAME, b"Interval start # (0)");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

            testutil::CHCKSD(&save.QNAME, save.START, b"~", save.FINISH, CNVTOL, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Save the epoch of the maximum.
    //
    save.MAXEPC = save.RESULT[1];

    //
    // Compare RA at the epochs of minimum and
    // maximum values.
    //
    spicelib::SINCPT(
        &save.METHOD,
        &save.TARGET,
        save.MINEPC,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.POS.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"Intercept FOUND (0)", save.FOUND, true, OK, ctx)?;

    spicelib::RECRAD(
        save.POS.as_slice(),
        &mut save.R,
        &mut save.RA0,
        &mut save.DEC,
        ctx,
    );

    spicelib::SINCPT(
        &save.METHOD,
        &save.TARGET,
        save.MAXEPC,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.POS.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"Intercept FOUND (0)", save.FOUND, true, OK, ctx)?;

    spicelib::RECRAD(
        save.POS.as_slice(),
        &mut save.R,
        &mut save.RA1,
        &mut save.DEC,
        ctx,
    );

    //
    // Verify that MINEPC < MAXPEC and RA0 < RA1.
    //
    testutil::CHCKSD(b"epoch order", save.MINEPC, b"<", save.MAXEPC, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"RA order", save.RA0, b"<", save.RA1, 0.0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Repeat the previous searches using an adjustment value of
    // 1 degree.
    //
    testutil::TCASE(
        b"J2000-fixed Sun-Earth ray at 2000 Mar 21 attains extreme RA values; adjust = 1 deg.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESLT2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 MAR 20", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 MAR 22", &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.ABCORR, b"LT+S");
    fstr::assign(&mut save.VECDEF, SINDEF);
    fstr::assign(&mut save.METHOD, b"Ellipsoid");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    fstr::assign(&mut save.REF, b"IAU_EARTH");
    fstr::assign(&mut save.DREF, b"J2000");
    fstr::assign(&mut save.CRDSYS, b"ra/dec");
    fstr::assign(&mut save.CRDNAM, b"right ascension");

    //
    // Set DVEC.
    //
    spicelib::STR2ET(b"2000 MAR 21", &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

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

    fstr::assign(&mut save.RELATE, b"ABSMIN");
    save.REFVAL = 0.0;
    save.TOL = CNVTOL;
    save.ADJUST = (1.0 * spicelib::RPD(ctx));

    save.RPT = false;
    save.BAIL = false;

    //
    // Set the search step size. We need a step small enough
    // to capture the existence window, which we expect to be
    // about 7 minutes long, based on the Earth's orbital speed
    // and the Earth's ellipsoidal radii.
    //
    spicelib::GFSSTP(200.0, ctx)?;

    //
    // Perform the search.
    //
    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure we found exactly one root. Note that the
    // expectation that there's one root is based on
    // the geometry of this case.
    //
    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

    testutil::CHCKSI(b"N (0)", save.N, b"=", 1, 0, OK, ctx)?;

    //
    // Save the final epoch of the solution interval.
    //
    save.MINADJ = save.RESULT[2];

    //
    // Now search for an absolute maximum.
    //
    fstr::assign(&mut save.RELATE, b"ABSMAX");

    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure we found exactly one root.
    //
    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

    testutil::CHCKSI(b"N (0)", save.N, b"=", 1, 0, OK, ctx)?;

    //
    // Save the initial epoch of the solution interval.
    //
    save.MAXADJ = save.RESULT[1];

    //
    // Check RA at the epochs of where the adjusted minimum and
    // maximum values occur.
    //
    spicelib::SINCPT(
        &save.METHOD,
        &save.TARGET,
        save.MINADJ,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.POS.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"Intercept FOUND (0)", save.FOUND, true, OK, ctx)?;

    spicelib::RECRAD(
        save.POS.as_slice(),
        &mut save.R,
        &mut save.RA,
        &mut save.DEC,
        ctx,
    );

    testutil::CHCKSD(
        b"Adjusted min RA",
        save.RA,
        b"~",
        (save.RA0 + save.ADJUST),
        TIGHT,
        OK,
        ctx,
    )?;

    spicelib::SINCPT(
        &save.METHOD,
        &save.TARGET,
        save.MAXADJ,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.POS.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"Intercept FOUND (0)", save.FOUND, true, OK, ctx)?;

    spicelib::RECRAD(
        save.POS.as_slice(),
        &mut save.R,
        &mut save.RA,
        &mut save.DEC,
        ctx,
    );

    testutil::CHCKSD(
        b"Adjusted max RA",
        save.RA,
        b"~",
        (save.RA1 - save.ADJUST),
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Using the same observer, target, ray, and time period from
    // the previous search, we'll find the times when the right
    // ascension of the intercept is 180 degrees.
    //
    testutil::TCASE(
        b"J2000-fixed Sun-Earth ray at 2000 Mar 21 attains RA 180 degrees.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESLT2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 MAR 20", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 MAR 22", &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.ABCORR, b"LT+S");
    fstr::assign(&mut save.VECDEF, SINDEF);
    fstr::assign(&mut save.METHOD, b"Ellipsoid");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    fstr::assign(&mut save.REF, b"IAU_EARTH");
    fstr::assign(&mut save.DREF, b"J2000");
    fstr::assign(&mut save.CRDSYS, b"ra/dec");
    fstr::assign(&mut save.CRDNAM, b"right ascension");

    //
    // Set DVEC.
    //
    spicelib::STR2ET(b"2000 MAR 21", &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

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

    fstr::assign(&mut save.RELATE, b"=");
    save.REFVAL = spicelib::PI(ctx);
    save.TOL = CNVTOL;
    save.ADJUST = 0.0;

    save.RPT = false;
    save.BAIL = false;

    //
    // Set the search step size. We need a step small enough
    // to capture the existence window, which we expect to be
    // about 7 minutes long, based on the Earth's orbital speed
    // and the Earth's ellipsoidal radii.
    //
    spicelib::GFSSTP(200.0, ctx)?;

    //
    // Perform the search.
    //
    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure we found exactly one root.
    //
    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

    testutil::CHCKSI(b"N (0)", save.N, b"=", 1, 0, OK, ctx)?;

    //
    // Make sure that the start and finish times are
    // close together.
    //
    spicelib::WNFETD(
        save.RESULT.as_slice(),
        1,
        &mut save.START,
        &mut save.FINISH,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.QNAME, b"Interval start # (0)");
    spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

    testutil::CHCKSD(&save.QNAME, save.START, b"~", save.FINISH, CNVTOL, OK, ctx)?;

    //
    // Save the epoch of this root for re-use.
    //
    save.ET2 = save.START;

    //
    // Check the RA of the intercept at the start time.
    //
    spicelib::SINCPT(
        &save.METHOD,
        &save.TARGET,
        save.ET2,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.POS.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"Intercept FOUND (0)", save.FOUND, true, OK, ctx)?;

    spicelib::RECRAD(
        save.POS.as_slice(),
        &mut save.R,
        &mut save.RA0,
        &mut save.DEC,
        ctx,
    );

    testutil::CHCKSD(b"RA0", save.RA0, b"~", spicelib::PI(ctx), MEDABS, OK, ctx)?;

    //
    // Repeat the search using Cartesian coordinates. We're looking
    // for a time when the Y-coordinate is zero. For this case, we
    // happen not to have any roots for positive X, so we should
    // find the same epoch as we did in the RA search.
    //
    //
    fstr::assign(&mut save.CRDSYS, b"Rectangular");
    fstr::assign(&mut save.CRDNAM, b"Y");
    save.REFVAL = 0.0;

    //
    // Perform the search.
    //
    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESLT2.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure we found at as many roots as
    // we did on the previous search.
    //
    save.XN = save.N;
    save.N = spicelib::WNCARD(save.RESLT2.as_slice(), ctx)?;

    testutil::CHCKSI(b"N (1)", save.N, b"=", save.XN, 0, OK, ctx)?;

    //
    // Check the solution times against those from the
    // Z-component search.
    //
    testutil::CHCKAD(
        b"RESLT2",
        save.RESLT2.as_slice(),
        b"~",
        save.RESULT.as_slice(),
        save.N,
        TIMTOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Using the same observer, target, ray, and time period from
    // the previous search, we'll find the times when the right
    // ascension of the intercept is less than 180 degrees.
    //
    testutil::TCASE(
        b"J2000-fixed Sun-Earth ray at 2000 Mar 21 has RA < 180 degrees.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESLT2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 MAR 20", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 MAR 22", &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.ABCORR, b"LT+S");
    fstr::assign(&mut save.VECDEF, SINDEF);
    fstr::assign(&mut save.METHOD, b"Ellipsoid");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    fstr::assign(&mut save.REF, b"IAU_EARTH");
    fstr::assign(&mut save.DREF, b"J2000");
    fstr::assign(&mut save.CRDSYS, b"ra/dec");
    fstr::assign(&mut save.CRDNAM, b"right ascension");

    //
    // Set DVEC.
    //
    spicelib::STR2ET(b"2000 MAR 21", &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

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

    fstr::assign(&mut save.RELATE, b"<");
    save.REFVAL = spicelib::PI(ctx);
    save.TOL = CNVTOL;
    save.ADJUST = 0.0;

    save.RPT = false;
    save.BAIL = false;

    //
    // Set the search step size. We need a step small enough
    // to capture the existence window, which we expect to be
    // about 7 minutes long, based on the Earth's orbital speed
    // and the Earth's ellipsoidal radii.
    //
    spicelib::GFSSTP(200.0, ctx)?;

    //
    // Perform the search.
    //
    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure we found exactly one root.
    //
    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

    testutil::CHCKSI(b"N (0)", save.N, b"=", 1, 0, OK, ctx)?;

    //
    // We expect the start and finish times to match MINPEC and ET2,
    // within a reasonable tolerance.
    //
    spicelib::WNFETD(
        save.RESULT.as_slice(),
        1,
        &mut save.START,
        &mut save.FINISH,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.QNAME, b"Interval start # (0)");
    spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

    testutil::CHCKSD(&save.QNAME, save.START, b"~", save.MINEPC, TIMTOL, OK, ctx)?;

    fstr::assign(&mut save.QNAME, b"Interval end # (0)");
    spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

    testutil::CHCKSD(&save.QNAME, save.FINISH, b"~", save.ET2, TIMTOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Using the same observer, target, ray, and time period from
    // the previous search, we'll find the times when the right
    // ascension of the intercept is greater than 180 degrees.
    //
    testutil::TCASE(
        b"J2000-fixed Sun-Earth ray at 2000 Mar 21 has RA > 180 degrees.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESLT2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 MAR 20", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 MAR 22", &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.ABCORR, b"LT+S");
    fstr::assign(&mut save.VECDEF, SINDEF);
    fstr::assign(&mut save.METHOD, b"Ellipsoid");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    fstr::assign(&mut save.REF, b"IAU_EARTH");
    fstr::assign(&mut save.DREF, b"J2000");
    fstr::assign(&mut save.CRDSYS, b"ra/dec");
    fstr::assign(&mut save.CRDNAM, b"right ascension");

    //
    // Set DVEC.
    //
    spicelib::STR2ET(b"2000 MAR 21", &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

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

    fstr::assign(&mut save.RELATE, b">");
    save.REFVAL = spicelib::PI(ctx);
    save.TOL = CNVTOL;
    save.ADJUST = 0.0;

    save.RPT = false;
    save.BAIL = false;

    //
    // Set the search step size. We need a step small enough
    // to capture the existence window, which we expect to be
    // about 7 minutes long, based on the Earth's orbital speed
    // and the Earth's ellipsoidal radii.
    //
    spicelib::GFSSTP(200.0, ctx)?;

    //
    // Perform the search.
    //
    spicelib::ZZGFCSLV(
        &save.VECDEF,
        &save.METHOD,
        &save.TARGET,
        &save.REF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        &save.CRDSYS,
        &save.CRDNAM,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        MW,
        NW,
        save.WORK.as_slice_mut(),
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure we found exactly one root.
    //
    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

    testutil::CHCKSI(b"N (0)", save.N, b"=", 1, 0, OK, ctx)?;

    //
    // We expect the start and finish times to match ET2 and MAXEPC,
    // within a reasonable tolerance.
    //
    spicelib::WNFETD(
        save.RESULT.as_slice(),
        1,
        &mut save.START,
        &mut save.FINISH,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.QNAME, b"Interval start # (0)");
    spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

    testutil::CHCKSD(&save.QNAME, save.START, b"~", save.ET2, TIMTOL, OK, ctx)?;

    fstr::assign(&mut save.QNAME, b"Interval end # (0)");
    spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

    testutil::CHCKSD(&save.QNAME, save.FINISH, b"~", save.MAXEPC, TIMTOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up:  delete kernels.", ctx)?;

    spicelib::SPKUEF(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKUEF(save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(NSPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
