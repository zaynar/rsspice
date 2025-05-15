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
const PCK: &[u8] = b"zzgflong.tpc";
const NPCK: &[u8] = b"nat.tpc";
const NSPK: &[u8] = b"nat.bsp";
const SPK: &[u8] = b"zzgflong.bsp";
const BDNMLN: i32 = 36;
const LNSIZE: i32 = 200;
const FRNMLN: i32 = 32;
const TIMLEN: i32 = 35;
const SYSLEN: i32 = 32;
const CRDLEN: i32 = 32;
const VDFLEN: i32 = 32;
const MTHLEN: i32 = 80;
const MAXWIN: i32 = (2 * 20000);
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
    TARGET: Vec<u8>,
    TIMSTR: Vec<u8>,
    VECDEF: Vec<u8>,
    ADJUST: f64,
    CNFINE: ActualArray<f64>,
    DVEC: StackArray<f64, 3>,
    ET: f64,
    ET0: f64,
    ET1: f64,
    RADII: StackArray<f64, 3>,
    REFVAL: f64,
    RESULT: ActualArray<f64>,
    TOL: f64,
    WORK: ActualArray2D<f64>,
    HAN2: i32,
    HANDLE: i32,
    N: i32,
    OBSID: i32,
    TRGID: i32,
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
        let mut TARGET = vec![b' '; BDNMLN as usize];
        let mut TIMSTR = vec![b' '; TIMLEN as usize];
        let mut VECDEF = vec![b' '; VDFLEN as usize];
        let mut ADJUST: f64 = 0.0;
        let mut CNFINE = ActualArray::<f64>::new(LBCELL..=MW);
        let mut DVEC = StackArray::<f64, 3>::new(1..=3);
        let mut ET: f64 = 0.0;
        let mut ET0: f64 = 0.0;
        let mut ET1: f64 = 0.0;
        let mut RADII = StackArray::<f64, 3>::new(1..=3);
        let mut REFVAL: f64 = 0.0;
        let mut RESULT = ActualArray::<f64>::new(LBCELL..=MW);
        let mut TOL: f64 = 0.0;
        let mut WORK = ActualArray2D::<f64>::new(LBCELL..=MW, 1..=NW);
        let mut HAN2: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut N: i32 = 0;
        let mut OBSID: i32 = 0;
        let mut TRGID: i32 = 0;
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
            TARGET,
            TIMSTR,
            VECDEF,
            ADJUST,
            CNFINE,
            DVEC,
            ET,
            ET0,
            ET1,
            RADII,
            REFVAL,
            RESULT,
            TOL,
            WORK,
            HAN2,
            HANDLE,
            N,
            OBSID,
            TRGID,
            BAIL,
            FOUND,
            RPT,
        }
    }
}

//$Procedure      F_ZZGFLNG1 ( Test GF longitude solver, part 1 )
pub fn F_ZZGFLNG1(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    // Maximum length of a coordinate system name.
    //

    //
    // Maximum length of a coordinate name.
    //

    //
    // Local variables
    //

    //
    // Saved all.
    //

    //
    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZGFLNG1", ctx)?;

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
    testutil::TCASE(b"Bad window count", ctx)?;

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

    spicelib::ZZGFLONG(
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

    spicelib::ZZGFLONG(
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

    spicelib::ZZGFLONG(
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

    spicelib::ZZGFLONG(
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
    testutil::TCASE(b"Bad operator", ctx)?;

    fstr::assign(&mut save.RELATE, b"==");
    save.REFVAL = 0.0;
    save.TOL = CNVTOL;
    save.ADJUST = 0.0;

    spicelib::ZZGFLONG(
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

    testutil::CHCKXC(true, b"SPICE(NOTRECOGNIZED)", OK, ctx)?;

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

    spicelib::ZZGFLONG(
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

    spicelib::ZZGFLONG(
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
    testutil::TCASE(b"Observer equals target", ctx)?;

    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"EARTH");

    spicelib::ZZGFLONG(
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

    spicelib::ZZGFLONG(
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

    spicelib::ZZGFLONG(
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

    spicelib::ZZGFLONG(
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

    spicelib::ZZGFLONG(
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

    spicelib::ZZGFLONG(
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

    spicelib::ZZGFLONG(
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

    spicelib::ZZGFLONG(
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
    testutil::TCASE(b"Non-longitude/RA system", ctx)?;

    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.ABCORR, b"XCN+S");

    fstr::assign(&mut save.CRDSYS, b"RECTANGULAR");

    spicelib::ZZGFLONG(
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

    spicelib::ZZGFLONG(
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

    spicelib::ZZGFLONG(
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

    spicelib::ZZGFLONG(
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

    spicelib::ZZGFLONG(
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

    spicelib::ZZGFLONG(
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

    spicelib::ZZGFLONG(
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

    spicelib::ZZGFLONG(
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

    spicelib::ZZGFLONG(
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

    spicelib::ZZGFLONG(
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

    spicelib::ZZGFLONG(
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

    spicelib::ZZGFLONG(
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

    spicelib::ZZGFLONG(
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
    //*    Non-error exceptional cases
    //*
    //*********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Empty confinement window", ctx)?;

    //
    // Start with an empty confinement window and non-empty
    // result window; make sure the result window is empty
    // on output.
    //
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

    fstr::assign(&mut save.RELATE, b"=");
    save.REFVAL = 0.0;
    save.TOL = CNVTOL;
    save.ADJUST = 0.0;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;

    save.ET0 = save.ET;
    save.ET1 = (save.ET + 1.0);

    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.RPT = false;
    save.BAIL = false;

    //
    // Endow RESULT with a single interval.
    //
    spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
    spicelib::WNINSD(save.ET0, save.ET1, save.RESULT.as_slice_mut(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFLONG(
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

    testutil::CHCKSI(
        b"WNCARD(RESULT)",
        spicelib::WNCARD(save.RESULT.as_slice(), ctx)?,
        b"=",
        0,
        0,
        OK,
        ctx,
    )?;

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
