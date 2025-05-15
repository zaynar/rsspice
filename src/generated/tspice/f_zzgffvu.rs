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
const BDNMLN: i32 = 36;
const LNSIZE: i32 = 200;
const FRNMLN: i32 = 32;
const KVARLN: i32 = 32;
const TXTSIZ: i32 = 2;

struct SaveVars {
    ABCORR: Vec<u8>,
    FIXREF: Vec<u8>,
    INST: Vec<u8>,
    KVNAME: Vec<u8>,
    OBSRVR: Vec<u8>,
    TARGET: Vec<u8>,
    TFRAME: Vec<u8>,
    TSHAPE: Vec<u8>,
    TXTBUF: ActualCharArray,
    ANGRAD: f64,
    BADBND: ActualArray2D<f64>,
    BADRAD: StackArray<f64, 3>,
    BIGRAD: f64,
    BOUNDS: StackArray2D<f64, 6>,
    BSITE: StackArray<f64, 3>,
    D: f64,
    EPS: f64,
    ET: f64,
    FOVBND: ActualArray2D<f64>,
    H: f64,
    LMBRAD: f64,
    LT: f64,
    NEGBOR: StackArray<f64, 3>,
    POLE: StackArray<f64, 3>,
    R: f64,
    RADII: StackArray<f64, 3>,
    RAYDIR: StackArray<f64, 3>,
    REFVEC: StackArray<f64, 3>,
    ROTANG: f64,
    ROTBDS: StackArray2D<f64, 6>,
    ROTBOR: StackArray<f64, 3>,
    SMLRAD: f64,
    SVRADI: StackArray<f64, 3>,
    TRGPOS: StackArray<f64, 3>,
    XFORM: StackArray2D<f64, 9>,
    XFORMI: StackArray2D<f64, 9>,
    X: StackArray<f64, 3>,
    Y: StackArray<f64, 3>,
    HAN2: i32,
    HANDLE: i32,
    INSTID: i32,
    N: i32,
    OBSID: i32,
    TFRMID: i32,
    FOUND: bool,
    VISTAT: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ABCORR = vec![b' '; LNSIZE as usize];
        let mut FIXREF = vec![b' '; FRNMLN as usize];
        let mut INST = vec![b' '; BDNMLN as usize];
        let mut KVNAME = vec![b' '; KVARLN as usize];
        let mut OBSRVR = vec![b' '; BDNMLN as usize];
        let mut TARGET = vec![b' '; BDNMLN as usize];
        let mut TFRAME = vec![b' '; FRNMLN as usize];
        let mut TSHAPE = vec![b' '; SHPLEN as usize];
        let mut TXTBUF = ActualCharArray::new(LNSIZE, 1..=TXTSIZ);
        let mut ANGRAD: f64 = 0.0;
        let mut BADBND = ActualArray2D::<f64>::new(1..=3, 1..=MAXVRT);
        let mut BADRAD = StackArray::<f64, 3>::new(1..=3);
        let mut BIGRAD: f64 = 0.0;
        let mut BOUNDS = StackArray2D::<f64, 6>::new(1..=3, 1..=2);
        let mut BSITE = StackArray::<f64, 3>::new(1..=3);
        let mut D: f64 = 0.0;
        let mut EPS: f64 = 0.0;
        let mut ET: f64 = 0.0;
        let mut FOVBND = ActualArray2D::<f64>::new(1..=3, 1..=MAXVRT);
        let mut H: f64 = 0.0;
        let mut LMBRAD: f64 = 0.0;
        let mut LT: f64 = 0.0;
        let mut NEGBOR = StackArray::<f64, 3>::new(1..=3);
        let mut POLE = StackArray::<f64, 3>::new(1..=3);
        let mut R: f64 = 0.0;
        let mut RADII = StackArray::<f64, 3>::new(1..=3);
        let mut RAYDIR = StackArray::<f64, 3>::new(1..=3);
        let mut REFVEC = StackArray::<f64, 3>::new(1..=3);
        let mut ROTANG: f64 = 0.0;
        let mut ROTBDS = StackArray2D::<f64, 6>::new(1..=3, 1..=2);
        let mut ROTBOR = StackArray::<f64, 3>::new(1..=3);
        let mut SMLRAD: f64 = 0.0;
        let mut SVRADI = StackArray::<f64, 3>::new(1..=3);
        let mut TRGPOS = StackArray::<f64, 3>::new(1..=3);
        let mut XFORM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut XFORMI = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut X = StackArray::<f64, 3>::new(1..=3);
        let mut Y = StackArray::<f64, 3>::new(1..=3);
        let mut HAN2: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut INSTID: i32 = 0;
        let mut N: i32 = 0;
        let mut OBSID: i32 = 0;
        let mut TFRMID: i32 = 0;
        let mut FOUND: bool = false;
        let mut VISTAT: bool = false;

        Self {
            ABCORR,
            FIXREF,
            INST,
            KVNAME,
            OBSRVR,
            TARGET,
            TFRAME,
            TSHAPE,
            TXTBUF,
            ANGRAD,
            BADBND,
            BADRAD,
            BIGRAD,
            BOUNDS,
            BSITE,
            D,
            EPS,
            ET,
            FOVBND,
            H,
            LMBRAD,
            LT,
            NEGBOR,
            POLE,
            R,
            RADII,
            RAYDIR,
            REFVEC,
            ROTANG,
            ROTBDS,
            ROTBOR,
            SMLRAD,
            SVRADI,
            TRGPOS,
            XFORM,
            XFORMI,
            X,
            Y,
            HAN2,
            HANDLE,
            INSTID,
            N,
            OBSID,
            TFRMID,
            FOUND,
            VISTAT,
        }
    }
}

//$Procedure F_ZZGFFVU ( Test GF FOV utilities )
pub fn F_ZZGFFVU(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

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
    testutil::TOPEN(b"F_ZZGFFVU", ctx)?;

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
    testutil::TCASE(b"Bad observer or target name.", ctx)?;

    fstr::assign(&mut save.INST, b"ALPHA_ELLIPSE_NONE");
    fstr::assign(&mut save.TSHAPE, b"ELLIPSOID");

    spicelib::VPACK(0.0, 0.0, 1.0, save.RAYDIR.as_slice_mut());

    fstr::assign(&mut save.TARGET, b"BET");
    fstr::assign(&mut save.TFRAME, b"BETAFIXED");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"SUN");

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    fstr::assign(&mut save.TARGET, b"BETA");
    fstr::assign(&mut save.TFRAME, b"BETAFIXED");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"SN");

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Observer equals target", ctx)?;

    fstr::assign(&mut save.TARGET, b"SUN");
    fstr::assign(&mut save.OBSRVR, b"SUN");

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
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

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
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

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
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

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
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

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
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

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
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

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Blank reference frame TFRAME for ray target", ctx)?;

    fstr::assign(&mut save.TSHAPE, b"RAY");
    fstr::assign(&mut save.TARGET, b"BETA");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    fstr::assign(&mut save.ABCORR, b"XS");
    fstr::assign(&mut save.TFRAME, b" ");

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDFRAME)", OK, ctx)?;

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

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
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

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
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

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
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

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
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

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    spicelib::VPACK(1.0, -1.0, 2.0, save.BADRAD.as_slice_mut());

    spicelib::PDPOOL(b"BODY399_RADII", 3, save.BADRAD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    spicelib::VPACK(1.0, 1.0, -2.0, save.BADRAD.as_slice_mut());

    spicelib::PDPOOL(b"BODY399_RADII", 3, save.BADRAD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    //
    // Restore all three radii.
    //
    spicelib::PDPOOL(b"BODY399_RADII", 3, save.SVRADI.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad ray direction vector RAYDIR", ctx)?;

    fstr::assign(&mut save.TSHAPE, b"RAY");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.ABCORR, b"XS");

    fstr::assign(&mut save.TFRAME, b"J2000");

    spicelib::CLEARD(3, save.RAYDIR.as_slice_mut());

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(ZEROVECTOR)", OK, ctx)?;

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

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
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

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
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
    // Keep the second boundary vector.
    //
    spicelib::MOVED(save.BSITE.as_slice(), 3, save.BADBND.subarray_mut([1, 1]));
    spicelib::MOVED(
        save.FOVBND.subarray([1, 2]),
        3,
        save.BADBND.subarray_mut([1, 2]),
    );

    spicelib::PDPOOL(&save.KVNAME, save.N, save.BADBND.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.TSHAPE, b"ELLIPSOID");

    spicelib::VPACK(0.0, 0.0, 1.0, save.RAYDIR.as_slice_mut());

    fstr::assign(&mut save.TARGET, b"BETA");
    fstr::assign(&mut save.TFRAME, b"BETAFIXED");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"SUN");

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
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

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
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

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
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
    fstr::assign(&mut save.INST, b"ALPHA_ELLIPSE_NONE");
    fstr::assign(&mut save.TSHAPE, b"ellipsoid");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.TFRAME, b"ITRF93");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.ABCORR, b"None");

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;
    spicelib::ZZGFFVST(save.ET, &mut save.VISTAT, ctx)?;

    testutil::CHCKXC(true, b"SPICE(FRAMEDATANOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No target ephemeris data available", ctx)?;
    //
    // This error is detected post-initialization.
    //

    fstr::assign(&mut save.TSHAPE, b"ellipsoid");
    fstr::assign(&mut save.TARGET, b"GASPRA");
    fstr::assign(&mut save.TFRAME, b"IAU_GASPRA");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.ABCORR, b"None");

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;
    spicelib::ZZGFFVST(save.ET, &mut save.VISTAT, ctx)?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No observer ephemeris data available", ctx)?;
    //
    // This error is detected post-initialization.
    //
    fstr::assign(&mut save.TSHAPE, b"ellipsoid");
    fstr::assign(&mut save.TARGET, b"SUN");
    fstr::assign(&mut save.TFRAME, b"IAU_SUN");
    fstr::assign(&mut save.OBSRVR, b"GASPRA");
    fstr::assign(&mut save.ABCORR, b"None");

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;
    spicelib::ZZGFFVST(save.ET, &mut save.VISTAT, ctx)?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No TFRAME orientation data available", ctx)?;
    //
    // This error is detected post-initialization.
    //
    fstr::assign(&mut save.TSHAPE, b"ellipsoid");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.TFRAME, b"ITRF93");

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;
    spicelib::ZZGFFVST(save.ET, &mut save.VISTAT, ctx)?;

    testutil::CHCKXC(true, b"SPICE(FRAMEDATANOTFOUND)", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Non-error exceptional cases
    //*
    //*********************************************************************

    //*********************************************************************
    //*
    //*    Non-error cases
    //*
    //*********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create instrument with near-180 degree circular FOV, centered 2 m above the Moon, aimed at the Moon. Verify that ZZGFFVIN doesn\'t create an ellipsoid that intersects the Moon.", ctx)?;

    //
    // We'll use as the observer the "Tranquility base" object from the
    // SPK created by TSTSPK. This object is located at radius 1737.402
    // km from the center of the Moon. The PCK created by TSTPCK sets
    // all three Moon radii to 1737.400 km. This test case relies on the
    // 2m difference.
    //
    fstr::assign(&mut save.OBSRVR, b"301001");
    fstr::assign(&mut save.TARGET, b"Moon");
    fstr::assign(&mut save.TSHAPE, b"ELLIPSOID");
    fstr::assign(&mut save.FIXREF, b"IAU_MOON");
    save.ET = 0.0;
    fstr::assign(&mut save.ABCORR, b"NONE");

    //
    // Get the position of the center of the Moon relative to the
    // observer.
    //
    spicelib::SPKPOS(
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.TRGPOS.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create an instrument frame that's centered at Tranquility base.
    // The axis directions of the frame are:
    //
    //     X:  North
    //     Y:  East
    //     Z:  Inward normal
    //
    spicelib::VPACK(0.0, 0.0, 1.0, save.POLE.as_slice_mut());
    spicelib::TWOVEC(
        save.TRGPOS.as_slice(),
        3,
        save.POLE.as_slice(),
        1,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.OBSID = 301001;
    save.TFRMID = 1301001;
    fstr::assign(&mut save.TFRAME, b"TRANQUILITY_INST");

    //
    // Set the first instrument name and ID. Add the name-ID mapping
    // to the kernel pool.
    //
    save.INSTID = 301001000;
    fstr::assign(&mut save.INST, b"INST_#");
    spicelib::REPMI(&save.INST.to_vec(), b"#", save.INSTID, &mut save.INST, ctx);

    fstr::assign(save.TXTBUF.get_mut(1), b"NAIF_BODY_NAME += \'#\'");
    spicelib::REPMC(
        &save.TXTBUF[1].to_vec(),
        b"#",
        &save.INST,
        &mut save.TXTBUF[1],
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(save.TXTBUF.get_mut(2), b"NAIF_BODY_CODE += #");
    spicelib::REPMI(
        &save.TXTBUF[2].to_vec(),
        b"#",
        save.INSTID,
        &mut save.TXTBUF[2],
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LMPOOL(save.TXTBUF.as_arg(), 2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.KVNAME, b"FRAME_#");
    spicelib::REPMC(&save.KVNAME.to_vec(), b"#", &save.TFRAME, &mut save.KVNAME);
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::PIPOOL(&save.KVNAME, 1, &[save.TFRMID], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.KVNAME, b"FRAME_#_NAME");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.TFRMID,
        &mut save.KVNAME,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::PCPOOL(&save.KVNAME, 1, CharArray::from_ref(&save.TFRAME), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.KVNAME, b"FRAME_#_CLASS");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.TFRMID,
        &mut save.KVNAME,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::PIPOOL(&save.KVNAME, 1, &[4], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.KVNAME, b"FRAME_#_CLASS_ID");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.TFRMID,
        &mut save.KVNAME,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::PIPOOL(&save.KVNAME, 1, &[save.TFRMID], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.KVNAME, b"FRAME_#_CENTER");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.TFRMID,
        &mut save.KVNAME,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::PIPOOL(&save.KVNAME, 1, &[save.OBSID], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.KVNAME, b"TKFRAME_#_RELATIVE");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.TFRMID,
        &mut save.KVNAME,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::PCPOOL(&save.KVNAME, 1, CharArray::from_ref(&save.FIXREF), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.KVNAME, b"TKFRAME_#_SPEC");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.TFRMID,
        &mut save.KVNAME,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::PCPOOL(&save.KVNAME, 1, CharArray::from_ref(b"MATRIX"), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.KVNAME, b"TKFRAME_#_MATRIX");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.TFRMID,
        &mut save.KVNAME,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::XPOSE(save.XFORM.as_slice(), save.XFORMI.as_slice_mut());
    spicelib::PDPOOL(&save.KVNAME, 9, save.XFORMI.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create the specification of an instrument having a circular FOV.
    // The instrument boresight points in the +Z direction of the
    // instrument frame (towards the center of the Moon).
    //
    fstr::assign(&mut save.KVNAME, b"INS#_FOV_SHAPE");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.INSTID,
        &mut save.KVNAME,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::PCPOOL(&save.KVNAME, 1, CharArray::from_ref(b"CIRCLE"), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.KVNAME, b"INS#_FOV_FRAME");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.INSTID,
        &mut save.KVNAME,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::PCPOOL(&save.KVNAME, 1, CharArray::from_ref(&save.TFRAME), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.KVNAME, b"INS#_BORESIGHT");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.INSTID,
        &mut save.KVNAME,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPACK(0.0, 0.0, 1.0, save.BSITE.as_slice_mut());
    spicelib::PDPOOL(&save.KVNAME, 3, save.BSITE.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a circular FOV with angular radius equal the angular
    // radius of the limb as seen from the observer.
    //
    spicelib::BODVRD(
        &save.TARGET,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.R = save.RADII[1];
    save.H = 0.002;
    save.D = (save.R + save.H);

    save.LMBRAD = f64::asin((save.R / save.D));
    save.ANGRAD = save.LMBRAD;
    //
    // Create a reference vector by rotating the boresight by ANGRAD
    // radians about the +Y axis of the instrument frame.
    //
    spicelib::VPACK(0.0, 1.0, 0.0, save.Y.as_slice_mut());

    spicelib::VROTV(
        save.BSITE.as_slice(),
        save.Y.as_slice(),
        save.ANGRAD,
        save.REFVEC.as_slice_mut(),
    );

    fstr::assign(&mut save.KVNAME, b"INS#_FOV_BOUNDARY");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.INSTID,
        &mut save.KVNAME,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(&save.KVNAME, 3, save.REFVEC.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Initialize the GF FOV subsystem with parameters for the
    // instrument.
    //
    spicelib::CLEARD(3, save.RAYDIR.as_slice_mut());

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now test whether the target is in the FOV.
    //
    spicelib::ZZGFFVST(save.ET, &mut save.VISTAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"VISTAT", save.VISTAT, true, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create instrument with near-180 degree circular FOV, centered 2 m above the Moon, aimed away from the Moon. Verify that the target is outside of the FOV.", ctx)?;

    fstr::assign(&mut save.KVNAME, b"INS#_BORESIGHT");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.INSTID,
        &mut save.KVNAME,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VMINUS(save.BSITE.as_slice(), save.NEGBOR.as_slice_mut());
    spicelib::PDPOOL(&save.KVNAME, 3, save.NEGBOR.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a new reference vector.
    //
    spicelib::VROTV(
        save.NEGBOR.as_slice(),
        save.Y.as_slice(),
        save.ANGRAD,
        save.REFVEC.as_slice_mut(),
    );

    fstr::assign(&mut save.KVNAME, b"INS#_FOV_BOUNDARY");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.INSTID,
        &mut save.KVNAME,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(&save.KVNAME, 3, save.REFVEC.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now test whether the target is in the FOV.
    //
    spicelib::ZZGFFVST(save.ET, &mut save.VISTAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"VISTAT", save.VISTAT, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create instrument with near-180 degree circular FOV, centered 2 m above the Moon. Rotate the instrument boresight from the nadir direction by slightly less than twice the angular radius of the FOV and verify the target intersects the FOV cone.", ctx)?;

    fstr::assign(&mut save.KVNAME, b"INS#_BORESIGHT");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.INSTID,
        &mut save.KVNAME,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.EPS = 0.000000000001;
    save.ROTANG = (((2 as f64) * save.ANGRAD) - save.EPS);

    spicelib::VROTV(
        save.BSITE.as_slice(),
        save.Y.as_slice(),
        save.ROTANG,
        save.ROTBOR.as_slice_mut(),
    );

    spicelib::PDPOOL(&save.KVNAME, 3, save.ROTBOR.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a new reference vector.
    //
    spicelib::VROTV(
        save.ROTBOR.as_slice(),
        save.Y.as_slice(),
        save.ANGRAD,
        save.REFVEC.as_slice_mut(),
    );

    fstr::assign(&mut save.KVNAME, b"INS#_FOV_BOUNDARY");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.INSTID,
        &mut save.KVNAME,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(&save.KVNAME, 3, save.REFVEC.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now test whether the target is in the FOV.
    //
    spicelib::ZZGFFVST(save.ET, &mut save.VISTAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"VISTAT", save.VISTAT, true, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create instrument with near-180 degree circular FOV, centered 2 m above the Moon. Rotate the instrument boresight from the nadir direction by slightly more than twice the angular radius of the FOV and verify the target does not intersect the FOV cone.", ctx)?;

    fstr::assign(&mut save.KVNAME, b"INS#_BORESIGHT");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.INSTID,
        &mut save.KVNAME,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.EPS = 0.000000000001;
    save.ROTANG = (((2 as f64) * save.ANGRAD) + save.EPS);

    spicelib::VROTV(
        save.BSITE.as_slice(),
        save.Y.as_slice(),
        save.ROTANG,
        save.ROTBOR.as_slice_mut(),
    );

    spicelib::PDPOOL(&save.KVNAME, 3, save.ROTBOR.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a new reference vector.
    //
    spicelib::VROTV(
        save.ROTBOR.as_slice(),
        save.Y.as_slice(),
        save.ANGRAD,
        save.REFVEC.as_slice_mut(),
    );

    fstr::assign(&mut save.KVNAME, b"INS#_FOV_BOUNDARY");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.INSTID,
        &mut save.KVNAME,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(&save.KVNAME, 3, save.REFVEC.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now test whether the target is in the FOV.
    //
    spicelib::ZZGFFVST(save.ET, &mut save.VISTAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"VISTAT", save.VISTAT, false, OK, ctx)?;

    //
    // The following cases use an elliptical FOV.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create instrument elliptical FOV. Make the angular size corresponding to the semi-major axis nearly 90 degrees and make the angular size corresponding to semi-minor axis half that. Point the instrument at the target.", ctx)?;

    fstr::assign(&mut save.KVNAME, b"INS#_BORESIGHT");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.INSTID,
        &mut save.KVNAME,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPACK(0.0, 0.0, 1.0, save.BSITE.as_slice_mut());
    spicelib::PDPOOL(&save.KVNAME, 3, save.BSITE.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.KVNAME, b"INS#_FOV_SHAPE");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.INSTID,
        &mut save.KVNAME,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::PCPOOL(&save.KVNAME, 1, CharArray::from_ref(b"ELLIPSE"), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BIGRAD = save.ANGRAD;
    save.SMLRAD = (save.ANGRAD / 2 as f64);

    spicelib::VPACK(1.0, 0.0, 0.0, save.X.as_slice_mut());

    spicelib::VROTV(
        save.BSITE.as_slice(),
        save.Y.as_slice(),
        save.BIGRAD,
        save.BOUNDS.subarray_mut([1, 1]),
    );
    spicelib::VROTV(
        save.BSITE.as_slice(),
        save.X.as_slice(),
        save.SMLRAD,
        save.BOUNDS.subarray_mut([1, 2]),
    );

    fstr::assign(&mut save.KVNAME, b"INS#_FOV_BOUNDARY");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.INSTID,
        &mut save.KVNAME,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(&save.KVNAME, 6, save.BOUNDS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now test whether the target is in the FOV.
    //
    spicelib::ZZGFFVST(save.ET, &mut save.VISTAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"VISTAT", save.VISTAT, true, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create instrument elliptical FOV. Make the angular size corresponding to the semi-major axis nearly 90 degrees and make the angular size corresponding to semi-minor axis half that. Rotate the FOV from the nadir direction, about the X axis, in the direction of the semi-minor axis, by slightly more than the sum of the FOV angular radius in that direction and the limb angular radius. Verify the target is not in the FOV cone.", ctx)?;

    fstr::assign(&mut save.KVNAME, b"INS#_BORESIGHT");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.INSTID,
        &mut save.KVNAME,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We use a looser tolerance for this test because the FOV
    // intersection computation relies on an ellipse-ray angular
    // separation computation, which relies on a root-finding process
    // that cannot achieve high precision. The value 1.D-10 radians
    // was found to work on a Mac Intel/gfortran/64-bit platform. The
    // value 1.D-8 radians was selected for portability.
    //
    save.EPS = 0.00000001;
    save.ROTANG = ((save.SMLRAD + save.LMBRAD) + save.EPS);

    spicelib::VROTV(
        save.BSITE.as_slice(),
        save.X.as_slice(),
        save.ROTANG,
        save.ROTBOR.as_slice_mut(),
    );

    spicelib::PDPOOL(&save.KVNAME, 3, save.ROTBOR.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We must rotate the boundary vectors as well.
    //
    for I in 1..=2 {
        spicelib::VROTV(
            save.BOUNDS.subarray([1, I]),
            save.X.as_slice(),
            save.ROTANG,
            save.ROTBDS.subarray_mut([1, I]),
        );
    }

    fstr::assign(&mut save.KVNAME, b"INS#_FOV_BOUNDARY");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.INSTID,
        &mut save.KVNAME,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(&save.KVNAME, 6, save.ROTBDS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now test whether the target is in the FOV.
    //
    spicelib::ZZGFFVST(save.ET, &mut save.VISTAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"VISTAT", save.VISTAT, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create instrument elliptical FOV. Make the angular size corresponding to the semi-major axis nearly 90 degrees and make the angular size corresponding to semi-minor axis half that. Rotate the FOV from the nadir direction, about the X axis, in the direction of the semi-minor axis, by slightly *less* than the sum of the FOV angular radius in that direction and the limb angular radius. Verify the target intersects FOV cone.", ctx)?;

    fstr::assign(&mut save.KVNAME, b"INS#_BORESIGHT");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.INSTID,
        &mut save.KVNAME,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We use a looser tolerance for this test because the FOV
    // intersection computation relies on an ellipse-ray angular
    // separation computation, which relies on a root-finding process
    // that cannot achieve high precision. The value 1.D-10 radians
    // was found to work on a Mac Intel/gfortran/64-bit platform. The
    // value 1.D-8 radians was selected for portability.
    //
    save.EPS = 0.00000001;
    save.ROTANG = ((save.SMLRAD + save.LMBRAD) - save.EPS);

    spicelib::VROTV(
        save.BSITE.as_slice(),
        save.X.as_slice(),
        save.ROTANG,
        save.ROTBOR.as_slice_mut(),
    );

    spicelib::PDPOOL(&save.KVNAME, 3, save.ROTBOR.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We must rotate the boundary vectors as well.
    //
    for I in 1..=2 {
        spicelib::VROTV(
            save.BOUNDS.subarray([1, I]),
            save.X.as_slice(),
            save.ROTANG,
            save.ROTBDS.subarray_mut([1, I]),
        );
    }

    fstr::assign(&mut save.KVNAME, b"INS#_FOV_BOUNDARY");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.INSTID,
        &mut save.KVNAME,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(&save.KVNAME, 6, save.ROTBDS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now test whether the target is in the FOV.
    //
    spicelib::ZZGFFVST(save.ET, &mut save.VISTAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"VISTAT", save.VISTAT, true, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create instrument elliptical FOV. Make the angular size corresponding to the semi-major axis nearly 90 degrees and make the angular size corresponding to semi-minor axis half that. Rotate the FOV from the nadir direction, about the Y axis, in the direction of the semi-major axis, by slightly more than the sum of the FOV angular radius in that direction and the limb angular radius. Verify the target does not intersect the FOV cone.", ctx)?;

    fstr::assign(&mut save.KVNAME, b"INS#_BORESIGHT");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.INSTID,
        &mut save.KVNAME,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We use a looser tolerance for this test because the FOV
    // intersection computation relies on an ellipse-ray angular
    // separation computation, which relies on a root-finding process
    // that cannot achieve high precision. The value 1.D-10 radians
    // was found to work on a Mac Intel/gfortran/64-bit platform. The
    // value 1.D-8 radians was selected for portability.
    //
    save.EPS = 0.00000001;
    save.ROTANG = ((save.BIGRAD + save.LMBRAD) + save.EPS);

    spicelib::VROTV(
        save.BSITE.as_slice(),
        save.Y.as_slice(),
        save.ROTANG,
        save.ROTBOR.as_slice_mut(),
    );

    spicelib::PDPOOL(&save.KVNAME, 3, save.ROTBOR.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We must rotate the boundary vectors as well.
    //
    for I in 1..=2 {
        spicelib::VROTV(
            save.BOUNDS.subarray([1, I]),
            save.Y.as_slice(),
            save.ROTANG,
            save.ROTBDS.subarray_mut([1, I]),
        );
    }

    fstr::assign(&mut save.KVNAME, b"INS#_FOV_BOUNDARY");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.INSTID,
        &mut save.KVNAME,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(&save.KVNAME, 6, save.ROTBDS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now test whether the target is in the FOV.
    //
    spicelib::ZZGFFVST(save.ET, &mut save.VISTAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"VISTAT", save.VISTAT, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create instrument elliptical FOV. Make the angular size corresponding to the semi-major axis nearly 90 degrees and make the angular size corresponding to semi-minor axis half that. Rotate the FOV from the nadir direction, about the Y axis, in the direction of the semi-major axis, by slightly *less* than the sum of the FOV angular radius in that direction and the limb angular radius. Verify the target intersects the FOV cone.", ctx)?;

    fstr::assign(&mut save.KVNAME, b"INS#_BORESIGHT");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.INSTID,
        &mut save.KVNAME,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We use a looser tolerance for this test because the FOV
    // intersection computation relies on an ellipse-ray angular
    // separation computation, which relies on a root-finding process
    // that cannot achieve high precision. The value 1.D-10 radians
    // was found to work on a Mac Intel/gfortran/64-bit platform. The
    // value 1.D-8 radians was selected for portability.
    //
    save.EPS = 0.00000001;
    save.ROTANG = ((save.BIGRAD + save.LMBRAD) - save.EPS);

    spicelib::VROTV(
        save.BSITE.as_slice(),
        save.Y.as_slice(),
        save.ROTANG,
        save.ROTBOR.as_slice_mut(),
    );

    spicelib::PDPOOL(&save.KVNAME, 3, save.ROTBOR.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We must rotate the boundary vectors as well.
    //
    for I in 1..=2 {
        spicelib::VROTV(
            save.BOUNDS.subarray([1, I]),
            save.Y.as_slice(),
            save.ROTANG,
            save.ROTBDS.subarray_mut([1, I]),
        );
    }

    fstr::assign(&mut save.KVNAME, b"INS#_FOV_BOUNDARY");
    spicelib::REPMI(
        &save.KVNAME.to_vec(),
        b"#",
        save.INSTID,
        &mut save.KVNAME,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(&save.KVNAME, 6, save.ROTBDS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFFVIN(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now test whether the target is in the FOV.
    //
    spicelib::ZZGFFVST(save.ET, &mut save.VISTAT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"VISTAT", save.VISTAT, true, OK, ctx)?;

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
