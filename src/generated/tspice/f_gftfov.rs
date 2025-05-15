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
const PCK: &[u8] = b"zzgffvu.tpc";
const NPCK: &[u8] = b"nat.tpc";
const NSPK: &[u8] = b"nat.bsp";
const NIK: &[u8] = b"nat.ti";
const SPK: &[u8] = b"zzgffvu.bsp";
const TIMTOL: f64 = ((2 as f64) * CNVTOL);
const BDNMLN: i32 = 36;
const LNSIZE: i32 = 200;
const FRNMLN: i32 = 32;
const KVARLN: i32 = 32;
const MAXWIN: i32 = 20000;
const LBCELL: i32 = -5;
const NINST: i32 = 4;
const LBLSIZ: i32 = 40;
const NCORS: i32 = 5;

struct SaveVars {
    ABCORR: Vec<u8>,
    ABCORS: ActualCharArray,
    INST: Vec<u8>,
    INSTRS: ActualCharArray,
    KVNAME: Vec<u8>,
    NEWFRM: Vec<u8>,
    OLDFRM: Vec<u8>,
    OBSRVR: Vec<u8>,
    QNAME: Vec<u8>,
    TARGET: Vec<u8>,
    TFRAME: Vec<u8>,
    TITLE: Vec<u8>,
    TSHAPE: Vec<u8>,
    BADBND: ActualArray2D<f64>,
    BADRAD: StackArray<f64, 3>,
    BEG: f64,
    BSITE: StackArray<f64, 3>,
    CNFINE: ActualArray<f64>,
    DELTA: f64,
    END: f64,
    ENDPT: StackArray<f64, 2>,
    FOVBND: ActualArray2D<f64>,
    LEFT: f64,
    RADII: StackArray<f64, 3>,
    RAYDIR: StackArray<f64, 3>,
    RESULT: ActualArray<f64>,
    RIGHT: f64,
    STEP: f64,
    SVRADI: StackArray<f64, 3>,
    XTIME: f64,
    HAN2: i32,
    HANDLE: i32,
    I: i32,
    INSTID: i32,
    N: i32,
    ATTBLK: StackArray<bool, 6>,
    FOUND: bool,
    USELT: bool,
    XMIT: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ABCORR = vec![b' '; LNSIZE as usize];
        let mut ABCORS = ActualCharArray::new(CORLEN, 1..=NCORS);
        let mut INST = vec![b' '; BDNMLN as usize];
        let mut INSTRS = ActualCharArray::new(BDNMLN, 1..=NINST);
        let mut KVNAME = vec![b' '; KVARLN as usize];
        let mut NEWFRM = vec![b' '; FRNMLN as usize];
        let mut OLDFRM = vec![b' '; FRNMLN as usize];
        let mut OBSRVR = vec![b' '; BDNMLN as usize];
        let mut QNAME = vec![b' '; LBLSIZ as usize];
        let mut TARGET = vec![b' '; BDNMLN as usize];
        let mut TFRAME = vec![b' '; FRNMLN as usize];
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut TSHAPE = vec![b' '; SHPLEN as usize];
        let mut BADBND = ActualArray2D::<f64>::new(1..=3, 1..=MAXVRT);
        let mut BADRAD = StackArray::<f64, 3>::new(1..=3);
        let mut BEG: f64 = 0.0;
        let mut BSITE = StackArray::<f64, 3>::new(1..=3);
        let mut CNFINE = ActualArray::<f64>::new(LBCELL..=MAXWIN);
        let mut DELTA: f64 = 0.0;
        let mut END: f64 = 0.0;
        let mut ENDPT = StackArray::<f64, 2>::new(1..=2);
        let mut FOVBND = ActualArray2D::<f64>::new(1..=3, 1..=MAXVRT);
        let mut LEFT: f64 = 0.0;
        let mut RADII = StackArray::<f64, 3>::new(1..=3);
        let mut RAYDIR = StackArray::<f64, 3>::new(1..=3);
        let mut RESULT = ActualArray::<f64>::new(LBCELL..=MAXWIN);
        let mut RIGHT: f64 = 0.0;
        let mut STEP: f64 = 0.0;
        let mut SVRADI = StackArray::<f64, 3>::new(1..=3);
        let mut XTIME: f64 = 0.0;
        let mut HAN2: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut I: i32 = 0;
        let mut INSTID: i32 = 0;
        let mut N: i32 = 0;
        let mut ATTBLK = StackArray::<bool, 6>::new(1..=ABATSZ);
        let mut FOUND: bool = false;
        let mut USELT: bool = false;
        let mut XMIT: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"NONE"),
                Val::C(b"CN"),
                Val::C(b"XCN"),
                Val::C(b"CN+S"),
                Val::C(b"XCN+S"),
            ]
            .into_iter();
            ABCORS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"ALPHA_CIRCLE_NONE"),
                Val::C(b"ALPHA_ELLIPSE_NONE"),
                Val::C(b"ALPHA_RECTANGLE_NONE"),
                Val::C(b"ALPHA_DIAMOND_NONE"),
            ]
            .into_iter();
            INSTRS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            ABCORR,
            ABCORS,
            INST,
            INSTRS,
            KVNAME,
            NEWFRM,
            OLDFRM,
            OBSRVR,
            QNAME,
            TARGET,
            TFRAME,
            TITLE,
            TSHAPE,
            BADBND,
            BADRAD,
            BEG,
            BSITE,
            CNFINE,
            DELTA,
            END,
            ENDPT,
            FOVBND,
            LEFT,
            RADII,
            RAYDIR,
            RESULT,
            RIGHT,
            STEP,
            SVRADI,
            XTIME,
            HAN2,
            HANDLE,
            I,
            INSTID,
            N,
            ATTBLK,
            FOUND,
            USELT,
            XMIT,
        }
    }
}

//$Procedure      F_GFTFOV ( Test GFTFOV )
pub fn F_GFTFOV(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    // Local variables
    //

    //
    // Saved Variables
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_GFTFOV", ctx)?;

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
    // Create and load Nat's solar system SPK, PCK/FK, and IK
    // files.
    //
    testutil::NATPCK(NPCK, true, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::NATSPK(NSPK, true, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::NATIK(NIK, NSPK, NPCK, true, false, ctx)?;
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
    // The following error cases involve invalid initialization
    // values or missing data discovered at search time.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad step size", ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;

    fstr::assign(&mut save.INST, b"ALPHA_ELLIPSE_NONE");
    fstr::assign(&mut save.TSHAPE, b"ELLIPSOID");

    spicelib::VPACK(0.0, 0.0, 1.0, save.RAYDIR.as_slice_mut());

    fstr::assign(&mut save.TARGET, b"BET");
    fstr::assign(&mut save.TFRAME, b"BETAFIXED");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    save.STEP = 0.0;

    spicelib::GFTFOV(
        &save.INST,
        &save.TARGET,
        &save.TSHAPE,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDSTEP)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Result window too small", ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(0, save.RESULT.as_slice_mut(), ctx)?;

    fstr::assign(&mut save.INST, b"ALPHA_ELLIPSE_NONE");
    fstr::assign(&mut save.TSHAPE, b"ELLIPSOID");

    spicelib::VPACK(0.0, 0.0, 1.0, save.RAYDIR.as_slice_mut());

    fstr::assign(&mut save.TARGET, b"BETA");
    fstr::assign(&mut save.TFRAME, b"BETAFIXED");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    save.STEP = 300.0;

    spicelib::GFTFOV(
        &save.INST,
        &save.TARGET,
        &save.TSHAPE,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(WINDOWTOOSMALL)", OK, ctx)?;

    //
    // Restore window sizes.
    //
    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad observer or target name.", ctx)?;

    fstr::assign(&mut save.INST, b"ALPHA_ELLIPSE_NONE");
    fstr::assign(&mut save.TSHAPE, b"ELLIPSOID");

    spicelib::VPACK(0.0, 0.0, 1.0, save.RAYDIR.as_slice_mut());

    fstr::assign(&mut save.TARGET, b"BET");
    fstr::assign(&mut save.TFRAME, b"BETAFIXED");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"SUN");

    spicelib::GFTFOV(
        &save.INST,
        &save.TARGET,
        &save.TSHAPE,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    fstr::assign(&mut save.TARGET, b"BETA");
    fstr::assign(&mut save.TFRAME, b"BETAFIXED");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"SN");

    spicelib::GFTFOV(
        &save.INST,
        &save.TARGET,
        &save.TSHAPE,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Observer equals target", ctx)?;

    fstr::assign(&mut save.TARGET, b"SUN");
    fstr::assign(&mut save.OBSRVR, b"SUN");

    spicelib::GFTFOV(
        &save.INST,
        &save.TARGET,
        &save.TSHAPE,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad aberration correction for ellipsoid target.", ctx)?;

    fstr::assign(&mut save.ABCORR, b"XS");

    fstr::assign(&mut save.TSHAPE, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"BETA");
    fstr::assign(&mut save.OBSRVR, b"SUN");

    spicelib::GFTFOV(
        &save.INST,
        &save.TARGET,
        &save.TSHAPE,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad aberration correction for point target.", ctx)?;

    fstr::assign(&mut save.ABCORR, b"S");

    fstr::assign(&mut save.TSHAPE, b"POINT");
    fstr::assign(&mut save.TARGET, b"BETA");
    fstr::assign(&mut save.OBSRVR, b"SUN");

    spicelib::GFTFOV(
        &save.INST,
        &save.TARGET,
        &save.TSHAPE,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad aberration correction for ray target.", ctx)?;

    fstr::assign(&mut save.ABCORR, b"LT+S");

    fstr::assign(&mut save.TSHAPE, b"Ray");
    fstr::assign(&mut save.TARGET, b"BETA");
    fstr::assign(&mut save.OBSRVR, b"SUN");

    spicelib::GFTFOV(
        &save.INST,
        &save.TARGET,
        &save.TSHAPE,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad shape specification.", ctx)?;

    fstr::assign(&mut save.ABCORR, b"LT+S");

    fstr::assign(&mut save.TSHAPE, b"Line");
    fstr::assign(&mut save.TARGET, b"BETA");
    fstr::assign(&mut save.OBSRVR, b"SUN");

    spicelib::GFTFOV(
        &save.INST,
        &save.TARGET,
        &save.TSHAPE,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDSHAPE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad reference frame TFRAME", ctx)?;

    fstr::assign(&mut save.TSHAPE, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"BETA");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    fstr::assign(&mut save.ABCORR, b"XCN+S");
    fstr::assign(&mut save.TFRAME, b"EME2000");

    spicelib::GFTFOV(
        &save.INST,
        &save.TARGET,
        &save.TSHAPE,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Blank reference frame TFRAME for ellipsoidal target", ctx)?;

    fstr::assign(&mut save.TSHAPE, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"BETA");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    fstr::assign(&mut save.ABCORR, b"XCN+S");
    fstr::assign(&mut save.TFRAME, b" ");

    spicelib::GFTFOV(
        &save.INST,
        &save.TARGET,
        &save.TSHAPE,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad shape: ray.", ctx)?;

    fstr::assign(&mut save.TSHAPE, b"RAY");
    fstr::assign(&mut save.TARGET, b"BETA");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    fstr::assign(&mut save.ABCORR, b"XS");
    fstr::assign(&mut save.TFRAME, b" ");

    spicelib::GFTFOV(
        &save.INST,
        &save.TARGET,
        &save.TSHAPE,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Reference frame TFRAME not centered on target when TSHAPE requires it.",
        ctx,
    )?;

    fstr::assign(&mut save.TSHAPE, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"Mars");
    fstr::assign(&mut save.TFRAME, b"IAU_EARTH");
    fstr::assign(&mut save.OBSRVR, b"SUN");

    spicelib::GFTFOV(
        &save.INST,
        &save.TARGET,
        &save.TSHAPE,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Non-existent reference frame.", ctx)?;

    fstr::assign(&mut save.TARGET, b"beta");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.ABCORR, b"XCN+S");
    fstr::assign(&mut save.TFRAME, b"EME2000");

    spicelib::GFTFOV(
        &save.INST,
        &save.TARGET,
        &save.TSHAPE,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No target radii in kernel pool", ctx)?;

    fstr::assign(&mut save.TSHAPE, b"ellipsoid");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.TFRAME, b"IAU_EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.ABCORR, b"None");

    spicelib::GDPOOL(
        b"BODY399_RADII",
        1,
        3,
        &mut save.N,
        save.SVRADI.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DVPOOL(b"BODY399_RADII", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFTFOV(
        &save.INST,
        &save.TARGET,
        &save.TSHAPE,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // Restore all three radii.
    //
    spicelib::PDPOOL(b"BODY399_RADII", 3, save.SVRADI.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad target radii count", ctx)?;

    fstr::assign(&mut save.TSHAPE, b"ellipsoid");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.TFRAME, b"IAU_EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.ABCORR, b"None");

    spicelib::GDPOOL(
        b"BODY399_RADII",
        1,
        3,
        &mut save.N,
        save.SVRADI.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(b"BODY399_RADII", 2, save.RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFTFOV(
        &save.INST,
        &save.TARGET,
        &save.TSHAPE,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDCOUNT)", OK, ctx)?;

    //
    // Restore all three radii.
    //
    spicelib::PDPOOL(b"BODY399_RADII", 3, save.SVRADI.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad target radii values", ctx)?;

    fstr::assign(&mut save.TSHAPE, b"ellipsoid");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.TFRAME, b"IAU_EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.ABCORR, b"None");

    spicelib::GDPOOL(
        b"BODY399_RADII",
        1,
        3,
        &mut save.N,
        save.SVRADI.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPACK(0.0, 1.0, 2.0, save.BADRAD.as_slice_mut());

    spicelib::PDPOOL(b"BODY399_RADII", 3, save.BADRAD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFTFOV(
        &save.INST,
        &save.TARGET,
        &save.TSHAPE,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    spicelib::VPACK(1.0, -1.0, 2.0, save.BADRAD.as_slice_mut());

    spicelib::PDPOOL(b"BODY399_RADII", 3, save.BADRAD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFTFOV(
        &save.INST,
        &save.TARGET,
        &save.TSHAPE,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    spicelib::VPACK(1.0, 1.0, -2.0, save.BADRAD.as_slice_mut());

    spicelib::PDPOOL(b"BODY399_RADII", 3, save.BADRAD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GDPOOL(
        b"BODY399_RADII",
        1,
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFTFOV(
        &save.INST,
        &save.TARGET,
        &save.TSHAPE,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    //
    // Restore all three radii.
    //
    spicelib::PDPOOL(b"BODY399_RADII", 3, save.SVRADI.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GDPOOL(
        b"BODY399_RADII",
        1,
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No ID code for instrument.", ctx)?;

    fstr::assign(&mut save.INST, b"ALPHA_ELLIPSE");
    fstr::assign(&mut save.TSHAPE, b"ELLIPSOID");

    spicelib::VPACK(0.0, 0.0, 1.0, save.RAYDIR.as_slice_mut());

    fstr::assign(&mut save.TARGET, b"BETA");
    fstr::assign(&mut save.TFRAME, b"BETAFIXED");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"SUN");

    spicelib::GFTFOV(
        &save.INST,
        &save.TARGET,
        &save.TSHAPE,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Instrument parameters missing from kernel pool.", ctx)?;

    fstr::assign(&mut save.INST, b"ALPHA_ELLIPSE_NONE");

    spicelib::BODS2C(&save.INST, &mut save.INSTID, &mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    fstr::assign(&mut save.KVNAME, b"INS#_FOV_SHAPE");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.INSTID,
        &mut save.KVNAME,
        ctx,
    );
    //
    // Delete the instrument shape from the kernel pool.
    //
    spicelib::DVPOOL(&save.KVNAME, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.TSHAPE, b"ELLIPSOID");

    spicelib::VPACK(0.0, 0.0, 1.0, save.RAYDIR.as_slice_mut());

    fstr::assign(&mut save.TARGET, b"BETA");
    fstr::assign(&mut save.TFRAME, b"BETAFIXED");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"SUN");

    spicelib::GFTFOV(
        &save.INST,
        &save.TARGET,
        &save.TSHAPE,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SHAPEMISSING)", OK, ctx)?;

    //
    // Restore the instrument shape value.
    //
    spicelib::PCPOOL(&save.KVNAME, 1, CharArray::from_ref(b"ELLIPSE"), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Degenerate FOV ellipse.", ctx)?;

    fstr::assign(&mut save.INST, b"ALPHA_ELLIPSE_NONE");

    spicelib::BODS2C(&save.INST, &mut save.INSTID, &mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // Fetch the instrument boresight vector from the kernel pool.
    //
    fstr::assign(&mut save.KVNAME, b"INS#_BORESIGHT");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.INSTID,
        &mut save.KVNAME,
        ctx,
    );

    spicelib::GDPOOL(
        &save.KVNAME,
        1,
        3,
        &mut save.N,
        save.BSITE.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Fetch the instrument boundary vectors from the kernel pool.
    //
    fstr::assign(&mut save.KVNAME, b"INS#_FOV_BOUNDARY");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.INSTID,
        &mut save.KVNAME,
        ctx,
    );

    spicelib::GDPOOL(
        &save.KVNAME,
        1,
        (3 * MAXVRT),
        &mut save.N,
        save.FOVBND.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set the first boundary vector equal to the boresight vector.
    //
    spicelib::MOVED(save.FOVBND.as_slice(), save.N, save.BADBND.as_slice_mut());
    spicelib::MOVED(save.BSITE.as_slice(), 3, save.BADBND.subarray_mut([1, 1]));

    spicelib::PDPOOL(&save.KVNAME, save.N, save.BADBND.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.TSHAPE, b"ELLIPSOID");

    spicelib::VPACK(0.0, 0.0, 1.0, save.RAYDIR.as_slice_mut());

    fstr::assign(&mut save.TARGET, b"BETA");
    fstr::assign(&mut save.TFRAME, b"BETAFIXED");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"SUN");

    spicelib::GFTFOV(
        &save.INST,
        &save.TARGET,
        &save.TSHAPE,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(DEGENERATECASE)", OK, ctx)?;

    //
    // Restore the first boundary vector and set the second
    // equal to the boresight vector.
    //
    spicelib::MOVED(
        save.FOVBND.subarray([1, 1]),
        3,
        save.BADBND.subarray_mut([1, 1]),
    );
    spicelib::MOVED(save.BSITE.as_slice(), 3, save.BADBND.subarray_mut([1, 2]));

    spicelib::PDPOOL(&save.KVNAME, save.N, save.BADBND.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.TSHAPE, b"ELLIPSOID");

    spicelib::VPACK(0.0, 0.0, 1.0, save.RAYDIR.as_slice_mut());

    fstr::assign(&mut save.TARGET, b"BETA");
    fstr::assign(&mut save.TFRAME, b"BETAFIXED");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"SUN");

    spicelib::GFTFOV(
        &save.INST,
        &save.TARGET,
        &save.TSHAPE,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(DEGENERATECASE)", OK, ctx)?;

    //
    // Restore the instrument FOV boundary vectors.
    //
    spicelib::PDPOOL(&save.KVNAME, save.N, save.FOVBND.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"FOV boundary vector has excessive angular separation from boresight.",
        ctx,
    )?;

    fstr::assign(&mut save.INST, b"ALPHA_RECTANGLE_NONE");

    spicelib::BODS2C(&save.INST, &mut save.INSTID, &mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // Fetch the instrument boresight vector from the kernel pool.
    //
    fstr::assign(&mut save.KVNAME, b"INS#_BORESIGHT");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.INSTID,
        &mut save.KVNAME,
        ctx,
    );

    spicelib::GDPOOL(
        &save.KVNAME,
        1,
        3,
        &mut save.N,
        save.BSITE.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Fetch the instrument boundary vectors from the kernel pool.
    //
    fstr::assign(&mut save.KVNAME, b"INS#_FOV_BOUNDARY");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.INSTID,
        &mut save.KVNAME,
        ctx,
    );

    spicelib::GDPOOL(
        &save.KVNAME,
        1,
        (3 * MAXVRT),
        &mut save.N,
        save.FOVBND.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set the third boundary vector equal to the *negative*
    // of the boresight vector.
    //
    spicelib::MOVED(save.FOVBND.as_slice(), save.N, save.BADBND.as_slice_mut());
    spicelib::VMINUS(save.BSITE.as_slice(), save.BADBND.subarray_mut([1, 3]));

    spicelib::PDPOOL(&save.KVNAME, save.N, save.BADBND.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.TSHAPE, b"ELLIPSOID");

    spicelib::VPACK(0.0, 0.0, 1.0, save.RAYDIR.as_slice_mut());

    fstr::assign(&mut save.TARGET, b"BETA");
    fstr::assign(&mut save.TFRAME, b"BETAFIXED");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"SUN");

    spicelib::GFTFOV(
        &save.INST,
        &save.TARGET,
        &save.TSHAPE,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(FACENOTFOUND)", OK, ctx)?;

    //
    // Restore the instrument FOV boundary vectors.
    //
    spicelib::PDPOOL(&save.KVNAME, save.N, save.FOVBND.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No target orientation data available", ctx)?;

    //
    // This error is detected post-initialization.
    //
    // At this point, we need an actual, non-empty confinement
    // window.
    //
    spicelib::WNINSD(0.0, spicelib::SPD(), save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.INST, b"ALPHA_ELLIPSE_NONE");
    fstr::assign(&mut save.TSHAPE, b"ellipsoid");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.TFRAME, b"ITRF93");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.ABCORR, b"None");

    spicelib::GFTFOV(
        &save.INST,
        &save.TARGET,
        &save.TSHAPE,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
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
    spicelib::WNINSD(0.0, spicelib::SPD(), save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.TSHAPE, b"ellipsoid");
    fstr::assign(&mut save.TARGET, b"GASPRA");
    fstr::assign(&mut save.TFRAME, b"IAU_GASPRA");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.ABCORR, b"None");

    spicelib::GFTFOV(
        &save.INST,
        &save.TARGET,
        &save.TSHAPE,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
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
    spicelib::WNINSD(0.0, spicelib::SPD(), save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.TSHAPE, b"ellipsoid");
    fstr::assign(&mut save.TARGET, b"SUN");
    fstr::assign(&mut save.TFRAME, b"IAU_SUN");
    fstr::assign(&mut save.OBSRVR, b"GASPRA");
    fstr::assign(&mut save.ABCORR, b"None");

    spicelib::GFTFOV(
        &save.INST,
        &save.TARGET,
        &save.TSHAPE,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No TFRAME orientation data available", ctx)?;
    //
    // This error is detected post-initialization.
    //
    spicelib::WNINSD(0.0, spicelib::SPD(), save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.TSHAPE, b"ellipsoid");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.TFRAME, b"ITRF93");

    spicelib::GFTFOV(
        &save.INST,
        &save.TARGET,
        &save.TSHAPE,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
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

    //*********************************************************************
    //*
    //*    Normal cases
    //*
    //*********************************************************************

    //
    // We'll start out with cases using an ellipsoidal target and
    // geometric states. We'll use the four instruments defined
    // in nat.ti:
    //
    //    ALPHA_CIRCLE_NONE
    //    ALPHA_ELLIPSE_NONE
    //    ALPHA_RECTANGLE_NONE
    //    ALPHA_DIAMOND_NONE
    //
    // These have the FOV shapes
    //
    //    ELLIPSE
    //    CIRCLE
    //    RECTANGLE
    //    POLYGON
    //
    // and track body Alpha so body Beta's FOV entry and
    // exit times match the start and stop times of Beta's
    // transit across Alpha.
    //
    // Initialize inputs for the search.
    //
    spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;

    //
    // Use a 3-day span that covers 3 events.
    //
    spicelib::WNINSD(
        -(0.5 * spicelib::SPD()),
        (2.5 * spicelib::SPD()),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;

    fstr::assign(&mut save.TARGET, b"BETA");
    fstr::assign(&mut save.TSHAPE, b"ELLIPSOID");
    fstr::assign(&mut save.TFRAME, b"BETAFIXED");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    save.STEP = 300.0;

    for INS in 1..=NINST {
        fstr::assign(&mut save.INST, save.INSTRS.get(INS));

        for NC in 1..=NCORS {
            //
            // --- Case: ------------------------------------------------------
            //
            //
            fstr::assign(&mut save.ABCORR, save.ABCORS.get(NC));

            spicelib::ZZPRSCOR(&save.ABCORR, save.ATTBLK.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.USELT = save.ATTBLK[LTIDX];
            save.XMIT = save.ATTBLK[XMTIDX];

            if !save.ATTBLK[GEOIDX] {
                //
                // We have some type of aberration correction.
                //
                // We're going to change the instrument frame dynamically
                // so as to keep the frame definition compatible with the
                // aberration correction we're using. We'll fetch the
                // instrument ID and use the aberration correction to
                // construct the name of the appropriate
                // aberration-corrected frame.

                spicelib::BODN2C(&save.INST, &mut save.INSTID, &mut save.FOUND, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSL(b"INSTID found", save.FOUND, true, OK, ctx)?;

                fstr::assign(&mut save.KVNAME, b"INS#_FOV_FRAME");

                spicelib::REPMI(
                    &save.KVNAME.to_vec(),
                    b"#",
                    save.INSTID,
                    &mut save.KVNAME,
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::GCPOOL(
                    &save.KVNAME,
                    1,
                    1,
                    &mut save.N,
                    CharArrayMut::from_mut(&mut save.OLDFRM),
                    &mut save.FOUND,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSL(b"frame found", save.FOUND, true, OK, ctx)?;

                fstr::assign(&mut save.NEWFRM, &save.OLDFRM);

                spicelib::SUFFIX(b"_#", 0, &mut save.NEWFRM);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Substitute the aberration correction flag into
                // the frame name.
                //
                spicelib::REPMC(&save.NEWFRM.to_vec(), b"#", &save.ABCORR, &mut save.NEWFRM);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // At this point we have a frame name such as
                //
                //    ALPHA_VIEW_XY_CN+S
                //
                // We must remove the '+' symbol from the name.
                //
                spicelib::CMPRSS(b"+", 0, &save.NEWFRM.to_vec(), &mut save.NEWFRM);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Insert the new instrument frame association into
                // the kernel pool.
                //
                spicelib::PCPOOL(&save.KVNAME, 1, CharArray::from_ref(&save.NEWFRM), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
            } else {
                //
                // Set NEWFRM, since we reference it in the test case
                // title.
                //
                fstr::assign(&mut save.NEWFRM, &save.OLDFRM);
            }

            //
            // Set up the TCASE call.
            //
            fstr::assign(
                &mut save.TITLE,
                b"Target #; target shape #; Inst. #; #; inst frame #",
            );

            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TSHAPE, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.INST, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.NEWFRM, &mut save.TITLE);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::TCASE(&save.TITLE, ctx)?;

            //
            // Perform the search.
            //
            spicelib::GFTFOV(
                &save.INST,
                &save.TARGET,
                &save.TSHAPE,
                &save.TFRAME,
                &save.ABCORR,
                &save.OBSRVR,
                save.STEP,
                save.CNFINE.as_slice(),
                save.RESULT.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

            //
            // We're expecting 3 events.
            //
            testutil::CHCKSI(b"N", save.N, b"=", 3, 0, OK, ctx)?;

            if (save.N > 0) {
                {
                    let m1__: i32 = 1;
                    let m2__: i32 = save.N;
                    let m3__: i32 = 1;
                    save.I = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        let [arg2, arg3] = save.ENDPT.get_disjoint_mut([1, 2]).expect(
                            "mutable array elements passed to function must have disjoint indexes",
                        );
                        spicelib::WNFETD(save.RESULT.as_slice(), save.I, arg2, arg3, ctx)?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        //
                        // Check the interval endpoints. The event should
                        // start at one noon+delta and stop 10 minutes later,
                        // where delta depends on the aberration correction:
                        //
                        //    CN             delta = +1
                        //    CN+S           delta = +1
                        //    XCN            delta = -1
                        //    XCN+S          delta = -1
                        //
                        if save.USELT {
                            if save.XMIT {
                                save.DELTA = -1.0;
                            } else {
                                save.DELTA = 1.0;
                            }
                        } else {
                            save.DELTA = 0.0;
                        }

                        fstr::assign(&mut save.QNAME, b"Start time @");
                        spicelib::REPMI(&save.QNAME.to_vec(), b"@", save.I, &mut save.QNAME, ctx);

                        save.XTIME = ((((save.I - 1) as f64) * spicelib::SPD()) + save.DELTA);

                        testutil::CHCKSD(
                            &save.QNAME,
                            save.ENDPT[1],
                            b"~",
                            save.XTIME,
                            TIMTOL,
                            OK,
                            ctx,
                        )?;

                        fstr::assign(&mut save.QNAME, b"Stop time @");
                        spicelib::REPMI(&save.QNAME.to_vec(), b"@", save.I, &mut save.QNAME, ctx);

                        save.XTIME =
                            (((((save.I - 1) as f64) * spicelib::SPD()) + 600.0) + save.DELTA);

                        testutil::CHCKSD(
                            &save.QNAME,
                            save.ENDPT[2],
                            b"~",
                            save.XTIME,
                            TIMTOL,
                            OK,
                            ctx,
                        )?;

                        save.I += m3__;
                    }
                }
                //
                // End of result interval check loop.
                //
            }
            //
            // End of non-empty result window block.
            //

            if !save.ATTBLK[GEOIDX] {
                //
                // Restore the old instrument frame association in
                // the kernel pool.
                //
                spicelib::PCPOOL(&save.KVNAME, 1, CharArray::from_ref(&save.OLDFRM), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
            }
        }
        //
        // End of aberration correction loop.
        //
    }
    //
    // End of instrument loop.
    //

    //
    // For the next set of cases, we keep the inputs as before, except
    // for the target shape: we treat the target as a single point.
    // This should cause each FOV entry to start 1 minute late and end
    // 1 minute early.
    //
    fstr::assign(&mut save.TARGET, b"BETA");
    fstr::assign(&mut save.TSHAPE, b"POINT");
    fstr::assign(&mut save.TFRAME, b"BETAFIXED");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    save.STEP = 300.0;

    for INS in 1..=NINST {
        fstr::assign(&mut save.INST, save.INSTRS.get(INS));

        for NC in 1..=NCORS {
            //
            // --- Case: ------------------------------------------------------
            //
            //
            fstr::assign(&mut save.ABCORR, save.ABCORS.get(NC));

            spicelib::ZZPRSCOR(&save.ABCORR, save.ATTBLK.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.USELT = save.ATTBLK[LTIDX];
            save.XMIT = save.ATTBLK[XMTIDX];

            if !save.ATTBLK[GEOIDX] {
                //
                // We have some type of aberration correction.
                //
                // We're going to change the instrument frame dynamically
                // so as to keep the frame definition compatible with the
                // aberration correction we're using. We'll fetch the
                // instrument ID and use the aberration correction to
                // construct the name of the appropriate
                // aberration-corrected frame.

                spicelib::BODN2C(&save.INST, &mut save.INSTID, &mut save.FOUND, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSL(b"INSTID found", save.FOUND, true, OK, ctx)?;

                fstr::assign(&mut save.KVNAME, b"INS#_FOV_FRAME");

                spicelib::REPMI(
                    &save.KVNAME.to_vec(),
                    b"#",
                    save.INSTID,
                    &mut save.KVNAME,
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::GCPOOL(
                    &save.KVNAME,
                    1,
                    1,
                    &mut save.N,
                    CharArrayMut::from_mut(&mut save.OLDFRM),
                    &mut save.FOUND,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSL(b"frame found", save.FOUND, true, OK, ctx)?;

                fstr::assign(&mut save.NEWFRM, &save.OLDFRM);

                spicelib::SUFFIX(b"_#", 0, &mut save.NEWFRM);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Substitute the aberration correction flag into
                // the frame name.
                //
                spicelib::REPMC(&save.NEWFRM.to_vec(), b"#", &save.ABCORR, &mut save.NEWFRM);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // At this point we have a frame name such as
                //
                //    ALPHA_VIEW_XY_CN+S
                //
                // We must remove the '+' symbol from the name.
                //
                spicelib::CMPRSS(b"+", 0, &save.NEWFRM.to_vec(), &mut save.NEWFRM);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Insert the new instrument frame association into
                // the kernel pool.
                //
                spicelib::PCPOOL(&save.KVNAME, 1, CharArray::from_ref(&save.NEWFRM), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
            } else {
                //
                // Set NEWFRM, since we reference it in the test case
                // title.
                //
                fstr::assign(&mut save.NEWFRM, &save.OLDFRM);
            }

            //
            // Set up the TCASE call.
            //
            fstr::assign(
                &mut save.TITLE,
                b"Target #; target shape #; Inst. #; #; inst frame #",
            );

            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TSHAPE, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.INST, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.NEWFRM, &mut save.TITLE);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::TCASE(&save.TITLE, ctx)?;

            //
            // Perform the search.
            //
            spicelib::GFTFOV(
                &save.INST,
                &save.TARGET,
                &save.TSHAPE,
                &save.TFRAME,
                &save.ABCORR,
                &save.OBSRVR,
                save.STEP,
                save.CNFINE.as_slice(),
                save.RESULT.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

            //
            // We're expecting 3 events.
            //
            testutil::CHCKSI(b"N", save.N, b"=", 3, 0, OK, ctx)?;

            if (save.N > 0) {
                {
                    let m1__: i32 = 1;
                    let m2__: i32 = save.N;
                    let m3__: i32 = 1;
                    save.I = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        let [arg2, arg3] = save.ENDPT.get_disjoint_mut([1, 2]).expect(
                            "mutable array elements passed to function must have disjoint indexes",
                        );
                        spicelib::WNFETD(save.RESULT.as_slice(), save.I, arg2, arg3, ctx)?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        //
                        // Check the interval endpoints. The event should
                        // start at one noon+delta and stop 10 minutes later,
                        // where delta depends on the aberration correction:
                        //
                        //    CN             delta = +1
                        //    CN+S           delta = +1
                        //    XCN            delta = -1
                        //    XCN+S          delta = -1
                        //
                        if save.USELT {
                            if save.XMIT {
                                save.DELTA = -1.0;
                            } else {
                                save.DELTA = 1.0;
                            }
                        } else {
                            save.DELTA = 0.0;
                        }

                        fstr::assign(&mut save.QNAME, b"Start time @");
                        spicelib::REPMI(&save.QNAME.to_vec(), b"@", save.I, &mut save.QNAME, ctx);

                        save.XTIME =
                            (((((save.I - 1) as f64) * spicelib::SPD()) + save.DELTA) + 60.0);

                        testutil::CHCKSD(
                            &save.QNAME,
                            save.ENDPT[1],
                            b"~",
                            save.XTIME,
                            TIMTOL,
                            OK,
                            ctx,
                        )?;

                        fstr::assign(&mut save.QNAME, b"Stop time @");
                        spicelib::REPMI(&save.QNAME.to_vec(), b"@", save.I, &mut save.QNAME, ctx);

                        save.XTIME = ((((((save.I - 1) as f64) * spicelib::SPD()) + 600.0)
                            + save.DELTA)
                            - 60.0);

                        testutil::CHCKSD(
                            &save.QNAME,
                            save.ENDPT[2],
                            b"~",
                            save.XTIME,
                            TIMTOL,
                            OK,
                            ctx,
                        )?;

                        save.I += m3__;
                    }
                }
                //
                // End of result interval check loop.
                //
            }
            //
            // End of non-empty result window block.
            //

            if !save.ATTBLK[GEOIDX] {
                //
                // Restore the old instrument frame association in
                // the kernel pool.
                //
                spicelib::PCPOOL(&save.KVNAME, 1, CharArray::from_ref(&save.OLDFRM), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
            }
        }
        //
        // End of aberration correction loop.
        //
    }
    //
    // End of instrument loop.
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

    save.I = 1;
    fstr::assign(&mut save.TARGET, b"BETA");
    fstr::assign(&mut save.TSHAPE, b"POINT");
    fstr::assign(&mut save.TFRAME, b"BETAFIXED");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    save.STEP = 1.0;
    fstr::assign(&mut save.INST, save.INSTRS.get(save.I));
    fstr::assign(&mut save.ABCORR, save.ABCORS.get(save.I));

    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Use a 3-day span that covers 3 events.
    //
    spicelib::WNINSD(
        -(0.5 * spicelib::SPD()),
        (2.5 * spicelib::SPD()),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;

    spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFTFOV(
        &save.INST,
        &save.TARGET,
        &save.TSHAPE,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = 0;
    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKSI(b"COUNT", save.N, b"!=", 0, 0, OK, ctx)?;

    spicelib::WNFETD(save.RESULT.as_slice(), 1, &mut save.BEG, &mut save.END, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Reset tol.
    //

    spicelib::GFSTOL(0.0001, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFTFOV(
        &save.INST,
        &save.TARGET,
        &save.TSHAPE,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = 0;
    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKSI(b"COUNT", save.N, b"!=", 0, 0, OK, ctx)?;

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
