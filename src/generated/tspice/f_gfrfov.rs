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
const NCORS: i32 = 3;

struct SaveVars {
    ABCORR: Vec<u8>,
    ABCORS: ActualCharArray,
    INST: Vec<u8>,
    INSTRS: ActualCharArray,
    KVNAME: Vec<u8>,
    OBSRVR: Vec<u8>,
    QNAME: Vec<u8>,
    RFRAME: Vec<u8>,
    TITLE: Vec<u8>,
    BADBND: ActualArray2D<f64>,
    BEG: f64,
    BSITE: StackArray<f64, 3>,
    CNFINE: ActualArray<f64>,
    END: f64,
    ENDPT: StackArray<f64, 2>,
    FOVBND: ActualArray2D<f64>,
    LEFT: f64,
    RAYDIR: StackArray<f64, 3>,
    RESULT: ActualArray<f64>,
    RIGHT: f64,
    STEP: f64,
    XTIME: f64,
    COUNT: i32,
    HAN2: i32,
    HANDLE: i32,
    INS: i32,
    INSTID: i32,
    N: i32,
    NC: i32,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ABCORR = vec![b' '; LNSIZE as usize];
        let mut ABCORS = ActualCharArray::new(CORLEN, 1..=NCORS);
        let mut INST = vec![b' '; BDNMLN as usize];
        let mut INSTRS = ActualCharArray::new(BDNMLN, 1..=NINST);
        let mut KVNAME = vec![b' '; KVARLN as usize];
        let mut OBSRVR = vec![b' '; BDNMLN as usize];
        let mut QNAME = vec![b' '; LBLSIZ as usize];
        let mut RFRAME = vec![b' '; FRNMLN as usize];
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut BADBND = ActualArray2D::<f64>::new(1..=3, 1..=MAXVRT);
        let mut BEG: f64 = 0.0;
        let mut BSITE = StackArray::<f64, 3>::new(1..=3);
        let mut CNFINE = ActualArray::<f64>::new(LBCELL..=MAXWIN);
        let mut END: f64 = 0.0;
        let mut ENDPT = StackArray::<f64, 2>::new(1..=2);
        let mut FOVBND = ActualArray2D::<f64>::new(1..=3, 1..=MAXVRT);
        let mut LEFT: f64 = 0.0;
        let mut RAYDIR = StackArray::<f64, 3>::new(1..=3);
        let mut RESULT = ActualArray::<f64>::new(LBCELL..=MAXWIN);
        let mut RIGHT: f64 = 0.0;
        let mut STEP: f64 = 0.0;
        let mut XTIME: f64 = 0.0;
        let mut COUNT: i32 = 0;
        let mut HAN2: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut INS: i32 = 0;
        let mut INSTID: i32 = 0;
        let mut N: i32 = 0;
        let mut NC: i32 = 0;
        let mut FOUND: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"NONE"), Val::C(b"S"), Val::C(b"XS")].into_iter();
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
            OBSRVR,
            QNAME,
            RFRAME,
            TITLE,
            BADBND,
            BEG,
            BSITE,
            CNFINE,
            END,
            ENDPT,
            FOVBND,
            LEFT,
            RAYDIR,
            RESULT,
            RIGHT,
            STEP,
            XTIME,
            COUNT,
            HAN2,
            HANDLE,
            INS,
            INSTID,
            N,
            NC,
            FOUND,
        }
    }
}

//$Procedure      F_GFRFOV ( Test GFRFOV )
pub fn F_GFRFOV(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    testutil::TOPEN(b"F_GFRFOV", ctx)?;

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

    testutil::NATIK(NIK, NSPK, NPCK, true, true, ctx)?;
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

    spicelib::VPACK(0.0, 0.0, 1.0, save.RAYDIR.as_slice_mut());

    fstr::assign(&mut save.RFRAME, b"BETAFIXED");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    save.STEP = 0.0;

    spicelib::GFRFOV(
        &save.INST,
        save.RAYDIR.as_slice(),
        &save.RFRAME,
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

    spicelib::VPACK(0.0, 0.0, 1.0, save.RAYDIR.as_slice_mut());

    fstr::assign(&mut save.RFRAME, b"BETAFIXED");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    save.STEP = 300.0;

    spicelib::GFRFOV(
        &save.INST,
        save.RAYDIR.as_slice(),
        &save.RFRAME,
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
    testutil::TCASE(b"Bad observer name.", ctx)?;

    fstr::assign(&mut save.INST, b"ALPHA_ELLIPSE_NONE");

    spicelib::VPACK(0.0, 0.0, 1.0, save.RAYDIR.as_slice_mut());

    fstr::assign(&mut save.RFRAME, b"BETAFIXED");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"BET");

    spicelib::GFRFOV(
        &save.INST,
        save.RAYDIR.as_slice(),
        &save.RFRAME,
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
    testutil::TCASE(b"Bad aberration correction for ray target.", ctx)?;

    fstr::assign(&mut save.ABCORR, b"LT+S");

    fstr::assign(&mut save.OBSRVR, b"SUN");

    spicelib::GFRFOV(
        &save.INST,
        save.RAYDIR.as_slice(),
        &save.RFRAME,
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
    testutil::TCASE(b"Bad reference frame RFRAME", ctx)?;

    fstr::assign(&mut save.OBSRVR, b"SUN");
    fstr::assign(&mut save.ABCORR, b"XCN+S");
    fstr::assign(&mut save.RFRAME, b"EME2000");

    spicelib::GFRFOV(
        &save.INST,
        save.RAYDIR.as_slice(),
        &save.RFRAME,
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

    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.ABCORR, b"XCN+S");
    fstr::assign(&mut save.RFRAME, b"EME2000");

    spicelib::GFRFOV(
        &save.INST,
        save.RAYDIR.as_slice(),
        &save.RFRAME,
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
    testutil::TCASE(b"No ID code for instrument.", ctx)?;

    fstr::assign(&mut save.INST, b"ALPHA_ELLIPSE");

    spicelib::VPACK(0.0, 0.0, 1.0, save.RAYDIR.as_slice_mut());

    fstr::assign(&mut save.RFRAME, b"BETAFIXED");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"SUN");

    spicelib::GFRFOV(
        &save.INST,
        save.RAYDIR.as_slice(),
        &save.RFRAME,
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

    spicelib::VPACK(0.0, 0.0, 1.0, save.RAYDIR.as_slice_mut());

    fstr::assign(&mut save.RFRAME, b"BETAFIXED");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"SUN");

    spicelib::GFRFOV(
        &save.INST,
        save.RAYDIR.as_slice(),
        &save.RFRAME,
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

    spicelib::VPACK(0.0, 0.0, 1.0, save.RAYDIR.as_slice_mut());

    fstr::assign(&mut save.RFRAME, b"BETAFIXED");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"SUN");

    spicelib::GFRFOV(
        &save.INST,
        save.RAYDIR.as_slice(),
        &save.RFRAME,
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

    spicelib::VPACK(0.0, 0.0, 1.0, save.RAYDIR.as_slice_mut());

    fstr::assign(&mut save.RFRAME, b"BETAFIXED");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"SUN");

    spicelib::GFRFOV(
        &save.INST,
        save.RAYDIR.as_slice(),
        &save.RFRAME,
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

    spicelib::VPACK(0.0, 0.0, 1.0, save.RAYDIR.as_slice_mut());

    fstr::assign(&mut save.RFRAME, b"BETAFIXED");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"SUN");

    spicelib::GFRFOV(
        &save.INST,
        save.RAYDIR.as_slice(),
        &save.RFRAME,
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
    testutil::TCASE(b"No observer ephemeris data available", ctx)?;
    //
    // This error is detected post-initialization.
    //
    spicelib::WNINSD(0.0, spicelib::SPD(), save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.RFRAME, b"IAU_SUN");
    fstr::assign(&mut save.OBSRVR, b"GASPRA");
    fstr::assign(&mut save.ABCORR, b"S");

    spicelib::GFRFOV(
        &save.INST,
        save.RAYDIR.as_slice(),
        &save.RFRAME,
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
    testutil::TCASE(b"No RFRAME orientation data available", ctx)?;
    //
    // This error is detected post-initialization.
    //
    spicelib::WNINSD(0.0, spicelib::SPD(), save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.RFRAME, b"ITRF93");

    spicelib::GFRFOV(
        &save.INST,
        save.RAYDIR.as_slice(),
        &save.RFRAME,
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
    // We'll use the four instruments defined in nat.ti:
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

    //
    // For this set of cases, we treat the target as a ray: the target
    // is represented by the X-axis of the BETA_VIEW_XY frame.
    //
    // This should cause each FOV entry to start 1 minute late and end
    // 1 minute early.
    //
    fstr::assign(&mut save.RFRAME, b"BETA_VIEW_XY");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    save.STEP = 300.0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NINST;
        let m3__: i32 = 1;
        save.INS = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.INST, save.INSTRS.get(save.INS));

            {
                let m1__: i32 = 1;
                let m2__: i32 = NCORS;
                let m3__: i32 = 1;
                save.NC = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    //
                    // --- Case: ------------------------------------------------------
                    //
                    //
                    fstr::assign(&mut save.ABCORR, save.ABCORS.get(save.NC));

                    //
                    // Set up the TCASE call.
                    //
                    fstr::assign(&mut save.TITLE, b"Inst. #; #; inst frame #");

                    spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.INST, &mut save.TITLE);
                    spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);
                    spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.RFRAME, &mut save.TITLE);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::TCASE(&save.TITLE, ctx)?;

                    //
                    // Set the ray's direction vector: BETA_VIEW_XY +X.
                    //
                    spicelib::VPACK(1.0, 0.0, 0.0, save.RAYDIR.as_slice_mut());

                    //
                    // Perform the search.
                    //
                    spicelib::GFRFOV(
                        &save.INST,
                        save.RAYDIR.as_slice(),
                        &save.RFRAME,
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
                        for I in 1..=save.N {
                            let [arg2, arg3] = save.ENDPT.get_disjoint_mut([1, 2]).expect("mutable array elements passed to function must have disjoint indexes");
                            spicelib::WNFETD(save.RESULT.as_slice(), I, arg2, arg3, ctx)?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                            //
                            // Stellar aberration should have *no* effect on
                            // event times, since the observer is the Sun
                            // and the Sun is placed at the solar system
                            // barycenter in Nat's solar system. We still
                            // exercise the target code's aberration correction
                            // logic paths.
                            //
                            // The target is a ray, so the event times will be
                            // offset from those of the ellipsoidal target BETA.
                            // Events will start 1 minute late and end 1 minute
                            // early.
                            //
                            fstr::assign(&mut save.QNAME, b"Start time @");
                            spicelib::REPMI(&save.QNAME.to_vec(), b"@", I, &mut save.QNAME, ctx);

                            save.XTIME = ((((I - 1) as f64) * spicelib::SPD()) + 60.0);

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
                            spicelib::REPMI(&save.QNAME.to_vec(), b"@", I, &mut save.QNAME, ctx);

                            save.XTIME = (((((I - 1) as f64) * spicelib::SPD()) + 600.0) - 60.0);

                            testutil::CHCKSD(
                                &save.QNAME,
                                save.ENDPT[2],
                                b"~",
                                save.XTIME,
                                TIMTOL,
                                OK,
                                ctx,
                            )?;
                        }
                        //
                        // End of result interval check loop.
                        //
                    }
                    //
                    // End of non-empty result window block.
                    //

                    save.NC += m3__;
                }
            }
            //
            // End of aberration correction loop.
            //
            save.INS += m3__;
        }
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

    save.INS = 1;
    save.NC = 1;
    fstr::assign(&mut save.INST, save.INSTRS.get(save.INS));
    fstr::assign(&mut save.ABCORR, save.ABCORS.get(save.NC));
    fstr::assign(&mut save.RFRAME, b"BETA_VIEW_XY");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    save.STEP = 300.0;

    spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

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
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set the ray's direction vector: BETA_VIEW_XY +X.
    //
    spicelib::VPACK(1.0, 0.0, 0.0, save.RAYDIR.as_slice_mut());

    //
    // Perform the search.
    //
    spicelib::GFRFOV(
        &save.INST,
        save.RAYDIR.as_slice(),
        &save.RFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
        save.CNFINE.as_slice(),
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

    spicelib::GFRFOV(
        &save.INST,
        save.RAYDIR.as_slice(),
        &save.RFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
        save.CNFINE.as_slice(),
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

    spicelib::CLPOOL(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
