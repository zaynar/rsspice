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
const SRFIDX: i32 = 1;
const CTRIDX: i32 = (SRFIDX + 1);
const CLSIDX: i32 = (CTRIDX + 1);
const TYPIDX: i32 = (CLSIDX + 1);
const FRMIDX: i32 = (TYPIDX + 1);
const SYSIDX: i32 = (FRMIDX + 1);
const PARIDX: i32 = (SYSIDX + 1);
const NSYPAR: i32 = 10;
const MN1IDX: i32 = (PARIDX + NSYPAR);
const MX1IDX: i32 = (MN1IDX + 1);
const MN2IDX: i32 = (MX1IDX + 1);
const MX2IDX: i32 = (MN2IDX + 1);
const MN3IDX: i32 = (MX2IDX + 1);
const MX3IDX: i32 = (MN3IDX + 1);
const BTMIDX: i32 = (MX3IDX + 1);
const ETMIDX: i32 = (BTMIDX + 1);
const DSKDSZ: i32 = ETMIDX;
const SVFCLS: i32 = 1;
const GENCLS: i32 = 2;
const LATSYS: i32 = 1;
const CYLSYS: i32 = 2;
const RECSYS: i32 = 3;
const PDTSYS: i32 = 4;
const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;
const DSK: &[u8] = b"gfoclt_nat.bds";
const PCK: &[u8] = b"nat.tpc";
const PCK2: &[u8] = b"generic.tpc";
const SPK1: &[u8] = b"nat.bsp";
const SPK2: &[u8] = b"generic.bsp";
const TIMTOL: f64 = 0.000003;
const TTOL2: f64 = 0.000035;
const TTOL3: f64 = 0.000175;
const LNSIZE: i32 = 80;
const NAMLEN: i32 = 32;
const TIMLEN: i32 = 50;
const LBCELL: i32 = -5;
const MAXWIN: i32 = 100;
const FRNMLN: i32 = 32;

struct SaveVars {
    ABCORR: Vec<u8>,
    AFRAME: Vec<u8>,
    BACK: Vec<u8>,
    BFRAME: Vec<u8>,
    BSHAPE: Vec<u8>,
    FFRAME: Vec<u8>,
    FRONT: Vec<u8>,
    FSHAPE: Vec<u8>,
    OBSRVR: Vec<u8>,
    OCCTYP: Vec<u8>,
    TITLE: Vec<u8>,
    TIMSTR: Vec<u8>,
    UTC0: Vec<u8>,
    UTC1: Vec<u8>,
    A: f64,
    ARAD: StackArray<f64, 3>,
    ASIZE: f64,
    BEG: f64,
    BOUNDS: StackArray2D<f64, 4>,
    BRAD: StackArray<f64, 3>,
    C: f64,
    CNFINE: StackArray<f64, 206>,
    CORPAR: StackArray<f64, 10>,
    END: f64,
    ET0: f64,
    ET1: f64,
    FINISH: f64,
    FIRST: f64,
    LAST: f64,
    LEFT: f64,
    LT: f64,
    NEWRAD: StackArray<f64, 3>,
    PALPHA: StackArray<f64, 3>,
    PMCOEF: StackArray<f64, 3>,
    RANGE: f64,
    RESULT: StackArray<f64, 206>,
    RIGHT: f64,
    RSCALE: f64,
    START: f64,
    STEP: f64,
    TOL: f64,
    XFINSH: f64,
    XSTART: f64,
    XTIME: f64,
    BODYID: i32,
    CORSYS: i32,
    HAN1: i32,
    HAN2: i32,
    N: i32,
    NLAT: i32,
    NLON: i32,
    SURFID: i32,
    FOUND: bool,
    MAKVTL: bool,
    USEPAD: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ABCORR = vec![b' '; CORLEN as usize];
        let mut AFRAME = vec![b' '; FRNMLN as usize];
        let mut BACK = vec![b' '; NAMLEN as usize];
        let mut BFRAME = vec![b' '; FRNMLN as usize];
        let mut BSHAPE = vec![b' '; NAMLEN as usize];
        let mut FFRAME = vec![b' '; FRNMLN as usize];
        let mut FRONT = vec![b' '; NAMLEN as usize];
        let mut FSHAPE = vec![b' '; NAMLEN as usize];
        let mut OBSRVR = vec![b' '; NAMLEN as usize];
        let mut OCCTYP = vec![b' '; NAMLEN as usize];
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut TIMSTR = vec![b' '; TIMLEN as usize];
        let mut UTC0 = vec![b' '; TIMLEN as usize];
        let mut UTC1 = vec![b' '; TIMLEN as usize];
        let mut A: f64 = 0.0;
        let mut ARAD = StackArray::<f64, 3>::new(1..=3);
        let mut ASIZE: f64 = 0.0;
        let mut BEG: f64 = 0.0;
        let mut BOUNDS = StackArray2D::<f64, 4>::new(1..=2, 1..=2);
        let mut BRAD = StackArray::<f64, 3>::new(1..=3);
        let mut C: f64 = 0.0;
        let mut CNFINE = StackArray::<f64, 206>::new(LBCELL..=(2 * MAXWIN));
        let mut CORPAR = StackArray::<f64, 10>::new(1..=NSYPAR);
        let mut END: f64 = 0.0;
        let mut ET0: f64 = 0.0;
        let mut ET1: f64 = 0.0;
        let mut FINISH: f64 = 0.0;
        let mut FIRST: f64 = 0.0;
        let mut LAST: f64 = 0.0;
        let mut LEFT: f64 = 0.0;
        let mut LT: f64 = 0.0;
        let mut NEWRAD = StackArray::<f64, 3>::new(1..=3);
        let mut PALPHA = StackArray::<f64, 3>::new(1..=3);
        let mut PMCOEF = StackArray::<f64, 3>::new(1..=3);
        let mut RANGE: f64 = 0.0;
        let mut RESULT = StackArray::<f64, 206>::new(LBCELL..=(2 * MAXWIN));
        let mut RIGHT: f64 = 0.0;
        let mut RSCALE: f64 = 0.0;
        let mut START: f64 = 0.0;
        let mut STEP: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut XFINSH: f64 = 0.0;
        let mut XSTART: f64 = 0.0;
        let mut XTIME: f64 = 0.0;
        let mut BODYID: i32 = 0;
        let mut CORSYS: i32 = 0;
        let mut HAN1: i32 = 0;
        let mut HAN2: i32 = 0;
        let mut N: i32 = 0;
        let mut NLAT: i32 = 0;
        let mut NLON: i32 = 0;
        let mut SURFID: i32 = 0;
        let mut FOUND: bool = false;
        let mut MAKVTL: bool = false;
        let mut USEPAD: bool = false;

        Self {
            ABCORR,
            AFRAME,
            BACK,
            BFRAME,
            BSHAPE,
            FFRAME,
            FRONT,
            FSHAPE,
            OBSRVR,
            OCCTYP,
            TITLE,
            TIMSTR,
            UTC0,
            UTC1,
            A,
            ARAD,
            ASIZE,
            BEG,
            BOUNDS,
            BRAD,
            C,
            CNFINE,
            CORPAR,
            END,
            ET0,
            ET1,
            FINISH,
            FIRST,
            LAST,
            LEFT,
            LT,
            NEWRAD,
            PALPHA,
            PMCOEF,
            RANGE,
            RESULT,
            RIGHT,
            RSCALE,
            START,
            STEP,
            TOL,
            XFINSH,
            XSTART,
            XTIME,
            BODYID,
            CORSYS,
            HAN1,
            HAN2,
            N,
            NLAT,
            NLON,
            SURFID,
            FOUND,
            MAKVTL,
            USEPAD,
        }
    }
}

//$Procedure      F_GFOCLT ( GFOCLT family tests )
pub fn F_GFOCLT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Looser time tolerance for point/ellipsoid
    // occultations where the ellipsoid is
    // in front:
    //

    //
    // Still looser time tolerance for point/ellipsoid
    // occultations where the ellipsoid is
    // in back:
    //

    // INTEGER               NREF
    // PARAMETER           ( NREF = 3 )

    //
    // Local variables
    //

    //
    // Saved everything.
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_GFOCLT", ctx)?;

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
    fstr::assign(&mut save.UTC0, b"2000 JAN 1  00:00:00 TDB");

    spicelib::STR2ET(&save.UTC0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED((2 * MAXWIN), save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED((2 * MAXWIN), save.RESULT.as_slice_mut(), ctx)?;

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
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Result window too small (detected before search)", ctx)?;

    spicelib::SSIZED(1, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFOCLT(
        b"Any",
        b"ALPHA",
        b"point",
        b"ALPHAFIXED",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"LT",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(WINDOWTOOSMALL)", OK, ctx)?;

    spicelib::SSIZED((2 * MAXWIN), save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Result window too small (detected during search)", ctx)?;

    spicelib::STR2ET(b"2000 JAN 1 TDB", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 JAN 5 TDB", &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.STEP = 300.0;

    spicelib::SSIZED(2, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFOCLT(
        b"Any",
        b"beta",
        b"ELLIPSOID",
        b"betaFIXED",
        b"alpha",
        b"ellipsoid",
        b"ALPHAFIXED",
        b"NONE",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(WINDOWEXCESS)", OK, ctx)?;

    spicelib::SSIZED((2 * MAXWIN), save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Unrecognized shape for front target", ctx)?;

    spicelib::GFOCLT(
        b"Any",
        b"ALPHA",
        b"pnt",
        b"ALPHAFIXED",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"LT",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADMETHODSYNTAX)", OK, ctx)?;

    //
    // Missing "unprioritized" keyword:
    //
    spicelib::GFOCLT(
        b"Any",
        b"ALPHA",
        b"DSK",
        b"ALPHAFIXED",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"LT",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADPRIORITYSPEC)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Unrecognized shape for back target", ctx)?;

    spicelib::GFOCLT(
        b"Any",
        b"ALPHA",
        b"point",
        b"ALPHAFIXED",
        b"BETA",
        b"ellipsod",
        b"BETAFIXED",
        b"LT",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADMETHODSYNTAX)", OK, ctx)?;

    //
    // Missing "unprioritized" keyword:
    //
    spicelib::GFOCLT(
        b"Any",
        b"ALPHA",
        b"point",
        b"ALPHAFIXED",
        b"BETA",
        b"DSK",
        b"BETAFIXED",
        b"LT",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADPRIORITYSPEC)", OK, ctx)?;

    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Two point targets", ctx)?;

    spicelib::GFOCLT(
        b"Any",
        b"ALPHA",
        b"point",
        b"ALPHAFIXED",
        b"BETA",
        b"point",
        b"BETAFIXED",
        b"LT",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDSHAPECOMBO)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Targets are identical", ctx)?;

    spicelib::GFOCLT(
        b"Any",
        b"ALPHA",
        b"point",
        b"ALPHAFIXED",
        b"ALPHA",
        b"ellipsoid",
        b"BETAFIXED",
        b"LT",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Front target and observer are identical", ctx)?;

    spicelib::GFOCLT(
        b"Any",
        b"ALPHA",
        b"point",
        b"ALPHAFIXED",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"LT",
        b"alpha",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Back target and observer are identical", ctx)?;

    spicelib::GFOCLT(
        b"Any",
        b"ALPHA",
        b"point",
        b"ALPHAFIXED",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"LT",
        b"beta",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Non-positive step size", ctx)?;

    save.STEP = 0.0;
    spicelib::GFOCLT(
        b"Any",
        b"ALPHA",
        b"point",
        b"ALPHAFIXED",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"LT",
        b"sun",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDSTEP)", OK, ctx)?;

    save.STEP = -1.0;
    spicelib::GFOCLT(
        b"Any",
        b"ALPHA",
        b"point",
        b"ALPHAFIXED",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"LT",
        b"sun",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDSTEP)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Bad occultation type", ctx)?;

    save.STEP = 1.0;
    spicelib::GFOCLT(
        b"Ay",
        b"ALPHA",
        b"point",
        b"ALPHAFIXED",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"LT",
        b"sun",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOCCTYPE)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Occultation type unsuitable for point target", ctx)?;

    save.STEP = 1.0;
    spicelib::GFOCLT(
        b"Annular",
        b"ALPHA",
        b"point",
        b"ALPHAFIXED",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"LT",
        b"sun",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADTYPESHAPECOMBO)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Unrecognized targets or observer", ctx)?;

    save.STEP = 1.0;
    spicelib::GFOCLT(
        b"Any",
        b"ALPH",
        b"point",
        b"ALPHAFIXED",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"LT",
        b"sun",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    spicelib::GFOCLT(
        b"Any",
        b"ALPHA",
        b"point",
        b"ALPHAFIXED",
        b"BETAA",
        b"ellipsoid",
        b"BETAFIXED",
        b"LT",
        b"sun",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    spicelib::GFOCLT(
        b"Any",
        b"ALPHA",
        b"point",
        b"ALPHAFIXED",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"LT",
        b"sunN",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Bad aberration correction values", ctx)?;

    save.STEP = 1.0;

    spicelib::GFOCLT(
        b"Any",
        b"ALPHA",
        b"point",
        b"ALPHAFIXED",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"S",
        b"sun",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::GFOCLT(
        b"Any",
        b"ALPHA",
        b"point",
        b"ALPHAFIXED",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"XS",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::GFOCLT(
        b"Any",
        b"ALPHA",
        b"point",
        b"ALPHAFIXED",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"RLT",
        b"sun",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::GFOCLT(
        b"Any",
        b"ALPHA",
        b"point",
        b"ALPHAFIXED",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"XRLT",
        b"sun",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::GFOCLT(
        b"Any",
        b"ALPHA",
        b"point",
        b"ALPHAFIXED",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"z",
        b"sun",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"No SPK data for observer", ctx)?;

    save.STEP = 1.0;

    spicelib::GFOCLT(
        b"Any",
        b"ALPHA",
        b"point",
        b"ALPHAFIXED",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"LT",
        b"GASPRA",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"No SPK data for front target", ctx)?;

    save.STEP = 1.0;

    spicelib::GFOCLT(
        b"Any",
        b"GASPRA",
        b"point",
        b"ALPHAFIXED",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"LT",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"No SPK data for back target", ctx)?;

    save.STEP = 1.0;

    spicelib::GFOCLT(
        b"Any",
        b"ALPHA",
        b"ellipsoid",
        b"ALPHAFIXED",
        b"gaspra",
        b"point",
        b"BETAFIXED",
        b"LT",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"No PCK orientation data for front target; both targets are ellipsoids.",
        ctx,
    )?;

    save.STEP = 1000.0;

    spicelib::GFOCLT(
        b"Any",
        b"EARTH",
        b"ELLIPSOID",
        b"ITRF93",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"LT",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(FRAMEDATANOTFOUND)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"No PCK orientation data for back target; both targets are ellipsoids.",
        ctx,
    )?;

    save.STEP = 1000.0;

    spicelib::GFOCLT(
        b"Any",
        b"ALPHA",
        b"ELLIPSOID",
        b"ALPHAFIXED",
        b"EARTH",
        b"ellipsoid",
        b"ITRF93",
        b"LT",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(FRAMEDATANOTFOUND)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"No PCK orientation data for front target; front target is ellipsoid; back is point.",
        ctx,
    )?;

    //
    // Capture Beta's prime meridian data; delete it from
    // the kernel pool.
    //
    spicelib::GDPOOL(
        b"BODY2000_PM",
        1,
        3,
        &mut save.N,
        save.PMCOEF.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::DVPOOL(b"BODY2000_PM", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // In order to exercise the SINCPT call, we need to use a
    // start time such that the bounding sphere logic is skipped.
    // The specific times used here are dependent on the geometry
    // of Nat's solar system.
    //
    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD((240.0 - 0.001), 260.0, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.STEP = 2.0;

    spicelib::GFOCLT(
        b"Any",
        b"beta",
        b"ellipsoid",
        b"betafixed",
        b"alpha",
        b"point",
        b"ALPHAFIXED",
        b"none",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(FRAMEDATANOTFOUND)", OK, ctx)?;

    //
    // Restore the PM data.
    //
    spicelib::PDPOOL(b"BODY2000_PM", 3, save.PMCOEF.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"No PCK orientation data for back target; front target is point; back is ellipsoid.",
        ctx,
    )?;

    //
    // Because the frame ALPHAFIXED is a TK frame defined
    // relative to the PCK frame BETAAFIXED, we can make
    // the SINCPT call, which accesses the ALPHAFIXED frame,
    // fail by damaging the data for the for BETAAFIXED frame.
    //
    // This scheme depends on the Nat's solar system PCK
    // implementation.
    //
    // Capture Beta's prime meridian data; delete it from
    // the kernel pool.
    //
    spicelib::GDPOOL(
        b"BODY2000_PM",
        1,
        3,
        &mut save.N,
        save.PMCOEF.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::DVPOOL(b"BODY2000_PM", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // In order to exercise the SINCPT call, we need to use a
    // start time such that the bounding sphere logic is skipped.
    // The specific times used here are dependent on the geometry
    // of Nat's solar system.
    //
    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD((60.0 - 0.001), 80.0, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.STEP = 2.0;

    spicelib::GFOCLT(
        b"Any",
        b"beta",
        b"point",
        b"betafixed",
        b"alpha",
        b"ellipsoid",
        b"ALPHAFIXED",
        b"none",
        b"Sun",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(FRAMEDATANOTFOUND)", OK, ctx)?;

    //
    // Restore the PM data.
    //
    spicelib::PDPOOL(b"BODY2000_PM", 3, save.PMCOEF.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Normal cases
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //

    //
    // Unload the generic test SPK, so we don't have interference from
    // the sun ephemeris.
    //
    spicelib::SPKUEF(save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // All expected event times are based on Nat's solar system.
    //
    testutil::TCASE(b"Any occultation of ALPHA by BETA, abcorr = NONE", ctx)?;

    fstr::assign(&mut save.UTC0, b"2000 JAN 1  00:00:00 TDB");

    spicelib::STR2ET(&save.UTC0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED((2 * MAXWIN), save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED((2 * MAXWIN), save.RESULT.as_slice_mut(), ctx)?;

    //
    // Create a confinement window with 4 intervals.
    //
    for I in 1..=4 {
        save.LEFT = ((save.ET0 + (((I - 1) as f64) * spicelib::SPD())) + 3600.0);
        save.RIGHT = (save.LEFT + (3600.0 * 22 as f64));

        spicelib::WNINSD(save.LEFT, save.RIGHT, save.CNFINE.as_slice_mut(), ctx)?;
    }

    save.STEP = 60.0;

    spicelib::GFOCLT(
        b"Any",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"ALPHA",
        b"ellipsoid",
        b"ALPHAFIXED",
        b"NONE",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
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
        // Check occultation start time.
        //
        fstr::assign(&mut save.TITLE, b"Occ # beg time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        save.XTIME = (((I - 1) as f64) * spicelib::SPD());
        testutil::CHCKSD(&save.TITLE, save.LEFT, b"~", save.XTIME, TIMTOL, OK, ctx)?;

        //
        // Check occultation end time.
        //
        fstr::assign(&mut save.TITLE, b"Occ # end time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        save.XTIME = ((((I - 1) as f64) * spicelib::SPD()) + ((10 as f64) * 60.0));
        testutil::CHCKSD(&save.TITLE, save.RIGHT, b"~", save.XTIME, TIMTOL, OK, ctx)?;

        spicelib::TIMOUT(
            save.LEFT,
            b"YYYY MON DD HR:MN:SC.###### ::TDB (TDB)",
            &mut save.TIMSTR,
            ctx,
        )?;

        // WRITE (*,*) I, ' ',  TIMSTR
        spicelib::TIMOUT(
            save.RIGHT,
            b"YYYY MON DD HR:MN:SC.###### ::TDB (TDB)",
            &mut save.TIMSTR,
            ctx,
        )?;
        // WRITE (*,*) '  ', TIMSTR
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Any occultation of ALPHA by BETA, abcorr = LT+S", ctx)?;

    //
    // Note: the stellar aberration correction spec should be ignored.
    //
    fstr::assign(&mut save.UTC0, b"2000 JAN 1  00:00:00 TDB");
    fstr::assign(&mut save.UTC1, b"2000 JAN 05 00:00:00 TDB");

    spicelib::STR2ET(&save.UTC0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&save.UTC1, &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED((2 * MAXWIN), save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED((2 * MAXWIN), save.RESULT.as_slice_mut(), ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;

    save.STEP = 60.0;

    spicelib::GFOCLT(
        b"Any",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"ALPHA",
        b"ellipsoid",
        b"ALPHAFIXED",
        b"LT+S",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
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

        fstr::assign(&mut save.TITLE, b"Occ # beg time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
        //
        // Check occultation start time.
        //
        save.XTIME = ((((I - 1) as f64) * spicelib::SPD()) + 1.0);
        testutil::CHCKSD(&save.TITLE, save.LEFT, b"~", save.XTIME, TIMTOL, OK, ctx)?;

        //
        // Check occultation end time.
        //
        fstr::assign(&mut save.TITLE, b"Occ # end time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        save.XTIME = (((((I - 1) as f64) * spicelib::SPD()) + ((10 as f64) * 60.0)) + 1.0);
        testutil::CHCKSD(&save.TITLE, save.RIGHT, b"~", save.XTIME, TIMTOL, OK, ctx)?;

        spicelib::TIMOUT(
            save.LEFT,
            b"YYYY MON DD HR:MN:SC.###### ::TDB (TDB)",
            &mut save.TIMSTR,
            ctx,
        )?;

        // WRITE (*,*) I, ' ',  TIMSTR
        spicelib::TIMOUT(
            save.RIGHT,
            b"YYYY MON DD HR:MN:SC.###### ::TDB (TDB)",
            &mut save.TIMSTR,
            ctx,
        )?;
        // WRITE (*,*) '  ', TIMSTR
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Annular transit of ALPHA by BETA, abcorr = NONE", ctx)?;

    fstr::assign(&mut save.UTC0, b"2000 JAN 1  00:00:00 TDB");
    fstr::assign(&mut save.UTC1, b"2000 JAN 05 00:00:00 TDB");

    spicelib::STR2ET(&save.UTC0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&save.UTC1, &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED((2 * MAXWIN), save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED((2 * MAXWIN), save.RESULT.as_slice_mut(), ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;

    save.STEP = 60.0;

    spicelib::GFOCLT(
        b"ANNULAR",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"ALPHA",
        b"ellipsoid",
        b"ALPHAFIXED",
        b"NONE",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
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
        // Check occultation start time.
        //
        fstr::assign(&mut save.TITLE, b"Occ # beg time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        save.XTIME = ((((I - 1) as f64) * spicelib::SPD()) + ((2 as f64) * 60.0));
        testutil::CHCKSD(&save.TITLE, save.LEFT, b"~", save.XTIME, TIMTOL, OK, ctx)?;

        //
        // Check occultation end time.
        //
        fstr::assign(&mut save.TITLE, b"Occ # end time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        save.XTIME = ((((I - 1) as f64) * spicelib::SPD()) + ((8 as f64) * 60.0));
        testutil::CHCKSD(&save.TITLE, save.RIGHT, b"~", save.XTIME, TIMTOL, OK, ctx)?;

        spicelib::TIMOUT(
            save.LEFT,
            b"YYYY MON DD HR:MN:SC.###### ::TDB (TDB)",
            &mut save.TIMSTR,
            ctx,
        )?;

        // WRITE (*,*) I, ' ',  TIMSTR
        spicelib::TIMOUT(
            save.RIGHT,
            b"YYYY MON DD HR:MN:SC.###### ::TDB (TDB)",
            &mut save.TIMSTR,
            ctx,
        )?;
        // WRITE (*,*) '  ', TIMSTR
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Annular transit of ALPHA by BETA, abcorr = CN", ctx)?;

    fstr::assign(&mut save.UTC0, b"2000 JAN 1  00:00:00 TDB");
    fstr::assign(&mut save.UTC1, b"2000 JAN 05 00:00:00 TDB");

    spicelib::STR2ET(&save.UTC0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&save.UTC1, &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED((2 * MAXWIN), save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED((2 * MAXWIN), save.RESULT.as_slice_mut(), ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;

    save.STEP = 60.0;

    spicelib::GFOCLT(
        b"ANNULAR",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"ALPHA",
        b"ellipsoid",
        b"ALPHAFIXED",
        b"CN",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
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
        // Check occultation start time.
        //
        fstr::assign(&mut save.TITLE, b"Occ # beg time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        save.XTIME = (((((I - 1) as f64) * spicelib::SPD()) + ((2 as f64) * 60.0)) + 1.0);
        testutil::CHCKSD(&save.TITLE, save.LEFT, b"~", save.XTIME, TIMTOL, OK, ctx)?;

        //
        // Check occultation end time.
        //
        fstr::assign(&mut save.TITLE, b"Occ # end time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        save.XTIME = (((((I - 1) as f64) * spicelib::SPD()) + ((8 as f64) * 60.0)) + 1.0);
        testutil::CHCKSD(&save.TITLE, save.RIGHT, b"~", save.XTIME, TIMTOL, OK, ctx)?;

        spicelib::TIMOUT(
            save.LEFT,
            b"YYYY MON DD HR:MN:SC.###### ::TDB (TDB)",
            &mut save.TIMSTR,
            ctx,
        )?;

        // WRITE (*,*) I, ' ',  TIMSTR
        spicelib::TIMOUT(
            save.RIGHT,
            b"YYYY MON DD HR:MN:SC.###### ::TDB (TDB)",
            &mut save.TIMSTR,
            ctx,
        )?;
        // WRITE (*,*) '  ', TIMSTR
    }

    //
    //---- Case -------------------------------------------------------------
    //

    //
    // All expected events time are based on Nat's solar system.
    //
    testutil::TCASE(b"Partial occultation of ALPHA by BETA, abcorr = NONE", ctx)?;

    fstr::assign(&mut save.UTC0, b"2000 JAN 1  00:00:00 TDB");
    fstr::assign(&mut save.UTC1, b"2000 JAN 05 00:00:00 TDB");

    spicelib::STR2ET(&save.UTC0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&save.UTC1, &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED((2 * MAXWIN), save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED((2 * MAXWIN), save.RESULT.as_slice_mut(), ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;

    save.STEP = 60.0;

    spicelib::GFOCLT(
        b"Partial",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"ALPHA",
        b"ellipsoid",
        b"ALPHAFIXED",
        b"NONE",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
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

        fstr::assign(&mut save.TITLE, b"Occ # beg time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
        //
        // Check occultation start time.
        //
        if spicelib::ODD(I) {
            save.XTIME = (((((I + 1) / 2) - 1) as f64) * spicelib::SPD());
        } else {
            save.XTIME = (((((I / 2) - 1) as f64) * spicelib::SPD()) + ((8 as f64) * 60.0));
        }

        testutil::CHCKSD(&save.TITLE, save.LEFT, b"~", save.XTIME, TIMTOL, OK, ctx)?;

        //
        // Check occultation end time.
        //
        fstr::assign(&mut save.TITLE, b"Occ # end time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        if spicelib::ODD(I) {
            save.XTIME = ((((((I + 1) / 2) - 1) as f64) * spicelib::SPD()) + ((2 as f64) * 60.0));
        } else {
            save.XTIME = (((((I / 2) - 1) as f64) * spicelib::SPD()) + ((10 as f64) * 60.0));
        }

        testutil::CHCKSD(&save.TITLE, save.RIGHT, b"~", save.XTIME, TIMTOL, OK, ctx)?;

        spicelib::TIMOUT(
            save.LEFT,
            b"YYYY MON DD HR:MN:SC.###### ::TDB (TDB)",
            &mut save.TIMSTR,
            ctx,
        )?;

        // WRITE (*,*) I, ' ',  TIMSTR
        spicelib::TIMOUT(
            save.RIGHT,
            b"YYYY MON DD HR:MN:SC.###### ::TDB (TDB)",
            &mut save.TIMSTR,
            ctx,
        )?;
        // WRITE (*,*) '  ', TIMSTR
    }

    //
    //---- Case -------------------------------------------------------------
    //

    //
    // All expected events time are based on Nat's solar system.
    //
    testutil::TCASE(b"Partial occultation of ALPHA by BETA, abcorr = XLT", ctx)?;

    fstr::assign(&mut save.UTC0, b"2000 JAN 1  00:00:00 TDB");
    fstr::assign(&mut save.UTC1, b"2000 JAN 05 00:00:00 TDB");

    spicelib::STR2ET(&save.UTC0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&save.UTC1, &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED((2 * MAXWIN), save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED((2 * MAXWIN), save.RESULT.as_slice_mut(), ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;

    save.STEP = 60.0;

    spicelib::GFOCLT(
        b"Partial",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"ALPHA",
        b"ellipsoid",
        b"ALPHAFIXED",
        b"XLT",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
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

        fstr::assign(&mut save.TITLE, b"Occ # beg time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
        //
        // Check occultation start time.
        //
        if spicelib::ODD(I) {
            save.XTIME = ((((((I + 1) / 2) - 1) as f64) * spicelib::SPD()) - 1.0);
        } else {
            save.XTIME = ((((((I / 2) - 1) as f64) * spicelib::SPD()) + ((8 as f64) * 60.0)) - 1.0);
        }

        testutil::CHCKSD(&save.TITLE, save.LEFT, b"~", save.XTIME, TIMTOL, OK, ctx)?;

        //
        // Check occultation end time.
        //
        fstr::assign(&mut save.TITLE, b"Occ # end time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        if spicelib::ODD(I) {
            save.XTIME =
                (((((((I + 1) / 2) - 1) as f64) * spicelib::SPD()) + ((2 as f64) * 60.0)) - 1.0);
        } else {
            save.XTIME =
                ((((((I / 2) - 1) as f64) * spicelib::SPD()) + ((10 as f64) * 60.0)) - 1.0);
        }

        testutil::CHCKSD(&save.TITLE, save.RIGHT, b"~", save.XTIME, TIMTOL, OK, ctx)?;

        spicelib::TIMOUT(
            save.LEFT,
            b"YYYY MON DD HR:MN:SC.###### ::TDB (TDB)",
            &mut save.TIMSTR,
            ctx,
        )?;

        // WRITE (*,*) I, ' ',  TIMSTR
        spicelib::TIMOUT(
            save.RIGHT,
            b"YYYY MON DD HR:MN:SC.###### ::TDB (TDB)",
            &mut save.TIMSTR,
            ctx,
        )?;
        // WRITE (*,*) '  ', TIMSTR
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Full occultation of ALPHA by BETA, abcorr = NONE. No events expected.",
        ctx,
    )?;

    fstr::assign(&mut save.UTC0, b"2000 JAN 1  00:00:00 TDB");
    fstr::assign(&mut save.UTC1, b"2000 JAN 05 00:00:00 TDB");

    spicelib::STR2ET(&save.UTC0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&save.UTC1, &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED((2 * MAXWIN), save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED((2 * MAXWIN), save.RESULT.as_slice_mut(), ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;

    save.STEP = 60.0;

    spicelib::GFOCLT(
        b"Full",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"ALPHA",
        b"ellipsoid",
        b"ALPHAFIXED",
        b"NONE",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // There should be no full occultations.
    //
    testutil::CHCKSI(
        b"RESULT cardinality",
        spicelib::WNCARD(save.RESULT.as_slice(), ctx)?,
        b"=",
        0,
        0,
        OK,
        ctx,
    )?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Full occultation, abcorr = NONE. ALPHA re-sized to 1/2 angular size of BETA.",
        ctx,
    )?;

    fstr::assign(&mut save.UTC0, b"2000 JAN 1  00:00:00 TDB");
    fstr::assign(&mut save.UTC1, b"2000 JAN 05 00:00:00 TDB");

    spicelib::STR2ET(&save.UTC0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&save.UTC1, &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up the radii of alpha.
    //
    spicelib::BODVRD(
        b"ALPHA",
        b"RADII",
        3,
        &mut save.N,
        save.ARAD.as_slice_mut(),
        ctx,
    )?;

    //
    // Compute the scale factor needed to reduce the
    // angular radius in the x-y plane of ALPHA to 1/8 its original
    // size. Recall that alpha's x-axis is parallel to the J2000 z-axis,
    // the +y-axis points toward the Sun at noon, and Alpha's +z axis
    // points in the J2000 -y direction at noon. So Alpha's
    // angular size in the x-y plane is controlled by the lengths
    // of its y and z axes, which are equal.
    //
    spicelib::SPKPOS(
        b"ALPHA",
        save.ET0,
        b"J2000",
        b"NONE",
        b"SUN",
        save.PALPHA.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;

    save.RANGE = spicelib::VNORM(save.PALPHA.as_slice());

    save.ASIZE = f64::asin((save.ARAD[2] / save.RANGE));

    save.NEWRAD[2] = (f64::sin((save.ASIZE / 8.0)) * save.RANGE);

    save.RSCALE = (save.NEWRAD[2] / save.ARAD[2]);

    //
    // Produce the new radii of ALPHA and store these
    // in the kernel pool.
    //
    spicelib::VSCL(
        save.RSCALE,
        save.ARAD.as_slice(),
        save.NEWRAD.as_slice_mut(),
    );

    //
    // Because of alpha's large vertical dimension, we must shrink
    // this dimension even farther to make total occultation possible.
    // We shrink Alpha's x-axis dimension (which is aligned with the
    // J2000 z-axis) by a factor of 4, making the total shrinkage
    // factor in this dimension equal to 32.
    //
    save.NEWRAD[1] = (save.NEWRAD[1] / 4 as f64);

    spicelib::PDPOOL(b"BODY1000_RADII", 3, save.NEWRAD.as_slice(), ctx)?;

    //
    // Search for full occultations using the modified
    // version of ALPHA.
    //
    spicelib::SSIZED((2 * MAXWIN), save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED((2 * MAXWIN), save.RESULT.as_slice_mut(), ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;

    save.STEP = 60.0;

    spicelib::GFOCLT(
        b"Full",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"ALPHA",
        b"ellipsoid",
        b"ALPHAFIXED",
        b"NONE",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
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
        // Check occultation start time.
        //
        fstr::assign(&mut save.TITLE, b"Occ # beg time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        save.XTIME = ((((I - 1) as f64) * spicelib::SPD()) + (4.5 * 60.0));
        testutil::CHCKSD(&save.TITLE, save.LEFT, b"~", save.XTIME, TIMTOL, OK, ctx)?;

        //
        // Check occultation end time.
        //
        fstr::assign(&mut save.TITLE, b"Occ # end time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        save.XTIME = ((((I - 1) as f64) * spicelib::SPD()) + (5.5 * 60.0));
        testutil::CHCKSD(&save.TITLE, save.RIGHT, b"~", save.XTIME, TIMTOL, OK, ctx)?;

        spicelib::TIMOUT(
            save.LEFT,
            b"YYYY MON DD HR:MN:SC.###### ::TDB (TDB)",
            &mut save.TIMSTR,
            ctx,
        )?;

        // WRITE (*,*) I, ' ',  TIMSTR
        spicelib::TIMOUT(
            save.RIGHT,
            b"YYYY MON DD HR:MN:SC.###### ::TDB (TDB)",
            &mut save.TIMSTR,
            ctx,
        )?;
        // WRITE (*,*) '  ', TIMSTR
    }

    //
    // Restore original radii of ALPHA.
    //
    spicelib::PDPOOL(b"BODY1000_RADII", 3, save.ARAD.as_slice(), ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Full occultation, abcorr = LT. ALPHA re-sized to 1/2 angular size of BETA.",
        ctx,
    )?;

    fstr::assign(&mut save.UTC0, b"2000 JAN 1  00:00:00 TDB");
    fstr::assign(&mut save.UTC1, b"2000 JAN 05 00:00:00 TDB");

    spicelib::STR2ET(&save.UTC0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&save.UTC1, &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up the radii of alpha.
    //
    spicelib::BODVRD(
        b"ALPHA",
        b"RADII",
        3,
        &mut save.N,
        save.ARAD.as_slice_mut(),
        ctx,
    )?;

    //
    // Compute the scale factor needed to reduce the
    // angular radius of ALPHA to 1/8 its original size.
    //
    spicelib::SPKPOS(
        b"ALPHA",
        save.ET0,
        b"J2000",
        b"NONE",
        b"SUN",
        save.PALPHA.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;

    save.RANGE = spicelib::VNORM(save.PALPHA.as_slice());

    save.ASIZE = f64::asin((save.ARAD[2] / save.RANGE));

    save.NEWRAD[2] = (f64::sin((save.ASIZE / 8.0)) * save.RANGE);

    save.RSCALE = (save.NEWRAD[2] / save.ARAD[2]);

    //
    // Produce the new radii of ALPHA and store these
    // in the kernel pool.
    //
    spicelib::VSCL(
        save.RSCALE,
        save.ARAD.as_slice(),
        save.NEWRAD.as_slice_mut(),
    );

    //
    // Because of alpha's large vertical dimension, we must shrink
    // this dimension even farther to make total occultation possible.
    // We shrink Alpha's x-axis dimension (which is aligned with the
    // J2000 z-axis) by a factor of 4, making the total shrinkage
    // factor in this dimension equal to 32.
    //
    save.NEWRAD[1] = (save.NEWRAD[1] / 4 as f64);

    spicelib::PDPOOL(b"BODY1000_RADII", 3, save.NEWRAD.as_slice(), ctx)?;

    //
    // Search for full occultations using the modified
    // version of ALPHA.
    //
    spicelib::SSIZED((2 * MAXWIN), save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED((2 * MAXWIN), save.RESULT.as_slice_mut(), ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;

    save.STEP = 60.0;

    spicelib::GFOCLT(
        b"Full",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"ALPHA",
        b"ellipsoid",
        b"ALPHAFIXED",
        b"LT",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
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
        // Check occultation start time.
        //
        fstr::assign(&mut save.TITLE, b"Occ # beg time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        save.XTIME = (((((I - 1) as f64) * spicelib::SPD()) + (4.5 * 60.0)) + 1.0);
        testutil::CHCKSD(&save.TITLE, save.LEFT, b"~", save.XTIME, TIMTOL, OK, ctx)?;

        //
        // Check occultation end time.
        //
        fstr::assign(&mut save.TITLE, b"Occ # end time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        save.XTIME = (((((I - 1) as f64) * spicelib::SPD()) + (5.5 * 60.0)) + 1.0);
        testutil::CHCKSD(&save.TITLE, save.RIGHT, b"~", save.XTIME, TIMTOL, OK, ctx)?;

        spicelib::TIMOUT(
            save.LEFT,
            b"YYYY MON DD HR:MN:SC.###### ::TDB (TDB)",
            &mut save.TIMSTR,
            ctx,
        )?;

        // WRITE (*,*) I, ' ',  TIMSTR
        spicelib::TIMOUT(
            save.RIGHT,
            b"YYYY MON DD HR:MN:SC.###### ::TDB (TDB)",
            &mut save.TIMSTR,
            ctx,
        )?;
        // WRITE (*,*) '  ', TIMSTR
    }

    //
    // Restore original radii of ALPHA.
    //
    spicelib::PDPOOL(b"BODY1000_RADII", 3, save.ARAD.as_slice(), ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Occultation of ALPHA center by BETA, abcorr = NONE.", ctx)?;

    fstr::assign(&mut save.UTC0, b"2000 JAN 1  00:00:00 TDB");
    fstr::assign(&mut save.UTC1, b"2000 JAN 05 00:00:00 TDB");

    spicelib::STR2ET(&save.UTC0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&save.UTC1, &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED((2 * MAXWIN), save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED((2 * MAXWIN), save.RESULT.as_slice_mut(), ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;

    save.STEP = 60.0;

    spicelib::GFOCLT(
        b"Any",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"ALPHA",
        b"point",
        b" ",
        b"NONE",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
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
        // Check occultation start time.
        //
        fstr::assign(&mut save.TITLE, b"Occ # beg time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        save.XTIME = ((((I - 1) as f64) * spicelib::SPD()) + ((4 as f64) * 60.0));
        testutil::CHCKSD(&save.TITLE, save.LEFT, b"~", save.XTIME, TIMTOL, OK, ctx)?;

        //
        // Check occultation end time.
        //
        fstr::assign(&mut save.TITLE, b"Occ # end time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        save.XTIME = ((((I - 1) as f64) * spicelib::SPD()) + ((6 as f64) * 60.0));
        testutil::CHCKSD(&save.TITLE, save.RIGHT, b"~", save.XTIME, TIMTOL, OK, ctx)?;

        spicelib::TIMOUT(
            save.LEFT,
            b"YYYY MON DD HR:MN:SC.###### ::TDB (TDB)",
            &mut save.TIMSTR,
            ctx,
        )?;

        // WRITE (*,*) I, ' ',  TIMSTR
        spicelib::TIMOUT(
            save.RIGHT,
            b"YYYY MON DD HR:MN:SC.###### ::TDB (TDB)",
            &mut save.TIMSTR,
            ctx,
        )?;
        // WRITE (*,*) '  ', TIMSTR
    }

    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Occultation of ALPHA center by BETA, abcorr = NONE, radii of ALPHA set to 0",
        ctx,
    )?;

    spicelib::BODVRD(
        b"ALPHA",
        b"RADII",
        3,
        &mut save.N,
        save.ARAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CLEARD(3, save.NEWRAD.as_slice_mut());

    spicelib::PDPOOL(b"BODY1000_RADII", 3, save.NEWRAD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.UTC0, b"2000 JAN 1  00:00:00 TDB");
    fstr::assign(&mut save.UTC1, b"2000 JAN 05 00:00:00 TDB");

    spicelib::STR2ET(&save.UTC0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&save.UTC1, &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED((2 * MAXWIN), save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED((2 * MAXWIN), save.RESULT.as_slice_mut(), ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;

    save.STEP = 60.0;

    spicelib::GFOCLT(
        b"Any",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"ALPHA",
        b"point",
        b" ",
        b"NONE",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
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
        // Check occultation start time.
        //
        fstr::assign(&mut save.TITLE, b"Occ # beg time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        save.XTIME = ((((I - 1) as f64) * spicelib::SPD()) + ((4 as f64) * 60.0));
        testutil::CHCKSD(&save.TITLE, save.LEFT, b"~", save.XTIME, TIMTOL, OK, ctx)?;

        //
        // Check occultation end time.
        //
        fstr::assign(&mut save.TITLE, b"Occ # end time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        save.XTIME = ((((I - 1) as f64) * spicelib::SPD()) + ((6 as f64) * 60.0));
        testutil::CHCKSD(&save.TITLE, save.RIGHT, b"~", save.XTIME, TIMTOL, OK, ctx)?;

        spicelib::TIMOUT(
            save.LEFT,
            b"YYYY MON DD HR:MN:SC.###### ::TDB (TDB)",
            &mut save.TIMSTR,
            ctx,
        )?;

        // WRITE (*,*) I, ' ',  TIMSTR
        spicelib::TIMOUT(
            save.RIGHT,
            b"YYYY MON DD HR:MN:SC.###### ::TDB (TDB)",
            &mut save.TIMSTR,
            ctx,
        )?;
        // WRITE (*,*) '  ', TIMSTR
    }

    //
    // Restore the original radii.
    //
    spicelib::PDPOOL(b"BODY1000_RADII", 3, save.ARAD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Occultation of ALPHA center by BETA, abcorr = LT.", ctx)?;

    //
    // Note that we use a looser time tolerance for these events,
    // since the light time correction for the ellipsoid is not
    // based on distance from the observer to the ellipsoid's center,
    // but rather to the intercept point, which is closer to
    // the observer.
    //
    fstr::assign(&mut save.UTC0, b"2000 JAN 1  00:00:00 TDB");
    fstr::assign(&mut save.UTC1, b"2000 JAN 05 00:00:00 TDB");

    spicelib::STR2ET(&save.UTC0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&save.UTC1, &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED((2 * MAXWIN), save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED((2 * MAXWIN), save.RESULT.as_slice_mut(), ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;

    save.STEP = 60.0;

    spicelib::GFOCLT(
        b"Any",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"ALPHA",
        b"point",
        b" ",
        b"LT",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
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
        // Check occultation start time.
        //
        fstr::assign(&mut save.TITLE, b"Occ # beg time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        save.XTIME = (((((I - 1) as f64) * spicelib::SPD()) + ((4 as f64) * 60.0)) + 1.0);
        testutil::CHCKSD(&save.TITLE, save.LEFT, b"~", save.XTIME, TTOL2, OK, ctx)?;

        //
        // Check occultation end time.
        //
        fstr::assign(&mut save.TITLE, b"Occ # end time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        save.XTIME = (((((I - 1) as f64) * spicelib::SPD()) + ((6 as f64) * 60.0)) + 1.0);
        testutil::CHCKSD(&save.TITLE, save.RIGHT, b"~", save.XTIME, TTOL2, OK, ctx)?;

        spicelib::TIMOUT(
            save.LEFT,
            b"YYYY MON DD HR:MN:SC.###### ::TDB (TDB)",
            &mut save.TIMSTR,
            ctx,
        )?;

        // WRITE (*,*) I, ' ',  TIMSTR
        spicelib::TIMOUT(
            save.RIGHT,
            b"YYYY MON DD HR:MN:SC.###### ::TDB (TDB)",
            &mut save.TIMSTR,
            ctx,
        )?;
        // WRITE (*,*) '  ', TIMSTR
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Transit of BETA center across ALPHA, abcorr = NONE.", ctx)?;

    fstr::assign(&mut save.UTC0, b"2000 JAN 1  00:00:00 TDB");
    fstr::assign(&mut save.UTC1, b"2000 JAN 05 00:00:00 TDB");

    spicelib::STR2ET(&save.UTC0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&save.UTC1, &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED((2 * MAXWIN), save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED((2 * MAXWIN), save.RESULT.as_slice_mut(), ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;

    save.STEP = 60.0;

    spicelib::GFOCLT(
        b"Any",
        b"BETA",
        b"POINT",
        b" ",
        b"ALPHA",
        b"ELLIPSOID",
        b"ALPHAFIXED",
        b"NONE",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
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
        // Check occultation start time.
        //
        fstr::assign(&mut save.TITLE, b"Occ # beg time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        save.XTIME = ((((I - 1) as f64) * spicelib::SPD()) + ((1 as f64) * 60.0));
        testutil::CHCKSD(&save.TITLE, save.LEFT, b"~", save.XTIME, TIMTOL, OK, ctx)?;

        //
        // Check occultation end time.
        //
        fstr::assign(&mut save.TITLE, b"Occ # end time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        save.XTIME = ((((I - 1) as f64) * spicelib::SPD()) + ((9 as f64) * 60.0));
        testutil::CHCKSD(&save.TITLE, save.RIGHT, b"~", save.XTIME, TIMTOL, OK, ctx)?;

        spicelib::TIMOUT(
            save.LEFT,
            b"YYYY MON DD HR:MN:SC.###### ::TDB (TDB)",
            &mut save.TIMSTR,
            ctx,
        )?;

        // WRITE (*,*) I, ' ',  TIMSTR
        spicelib::TIMOUT(
            save.RIGHT,
            b"YYYY MON DD HR:MN:SC.###### ::TDB (TDB)",
            &mut save.TIMSTR,
            ctx,
        )?;
        // WRITE (*,*) '  ', TIMSTR
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Transit of BETA center across ALPHA, abcorr = NONE, BETA raii set to 0",
        ctx,
    )?;

    spicelib::BODVRD(
        b"BETA",
        b"RADII",
        3,
        &mut save.N,
        save.BRAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CLEARD(3, save.NEWRAD.as_slice_mut());

    spicelib::PDPOOL(b"BODY2000_RADII", 3, save.NEWRAD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.UTC0, b"2000 JAN 1  00:00:00 TDB");
    fstr::assign(&mut save.UTC1, b"2000 JAN 05 00:00:00 TDB");

    spicelib::STR2ET(&save.UTC0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&save.UTC1, &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED((2 * MAXWIN), save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED((2 * MAXWIN), save.RESULT.as_slice_mut(), ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;

    save.STEP = 60.0;

    spicelib::GFOCLT(
        b"Any",
        b"BETA",
        b"POINT",
        b" ",
        b"ALPHA",
        b"ELLIPSOID",
        b"ALPHAFIXED",
        b"NONE",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
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
        // Check occultation start time.
        //
        fstr::assign(&mut save.TITLE, b"Occ # beg time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        save.XTIME = ((((I - 1) as f64) * spicelib::SPD()) + ((1 as f64) * 60.0));
        testutil::CHCKSD(&save.TITLE, save.LEFT, b"~", save.XTIME, TIMTOL, OK, ctx)?;

        //
        // Check occultation end time.
        //
        fstr::assign(&mut save.TITLE, b"Occ # end time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        save.XTIME = ((((I - 1) as f64) * spicelib::SPD()) + ((9 as f64) * 60.0));
        testutil::CHCKSD(&save.TITLE, save.RIGHT, b"~", save.XTIME, TIMTOL, OK, ctx)?;

        spicelib::TIMOUT(
            save.LEFT,
            b"YYYY MON DD HR:MN:SC.###### ::TDB (TDB)",
            &mut save.TIMSTR,
            ctx,
        )?;

        // WRITE (*,*) I, ' ',  TIMSTR
        spicelib::TIMOUT(
            save.RIGHT,
            b"YYYY MON DD HR:MN:SC.###### ::TDB (TDB)",
            &mut save.TIMSTR,
            ctx,
        )?;
        // WRITE (*,*) '  ', TIMSTR
    }

    spicelib::PDPOOL(b"BODY2000_RADII", 3, save.BRAD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Transit of BETA center across ALPHA, abcorr = LT.", ctx)?;

    fstr::assign(&mut save.UTC0, b"2000 JAN 1  00:00:00 TDB");
    fstr::assign(&mut save.UTC1, b"2000 JAN 05 00:00:00 TDB");

    spicelib::STR2ET(&save.UTC0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&save.UTC1, &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED((2 * MAXWIN), save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED((2 * MAXWIN), save.RESULT.as_slice_mut(), ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;

    save.STEP = 60.0;

    spicelib::GFOCLT(
        b"Any",
        b"BETA",
        b"POINT",
        b"BETAFIXED",
        b"ALPHA",
        b"ELLIPSOID",
        b"ALPHAFIXED",
        b"LT",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
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
        // Check occultation start time.
        //
        fstr::assign(&mut save.TITLE, b"Occ # beg time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        save.XTIME = (((((I - 1) as f64) * spicelib::SPD()) + ((1 as f64) * 60.0)) + 1.0);
        testutil::CHCKSD(&save.TITLE, save.LEFT, b"~", save.XTIME, TTOL3, OK, ctx)?;

        //
        // Check occultation end time.
        //
        fstr::assign(&mut save.TITLE, b"Occ # end time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        save.XTIME = (((((I - 1) as f64) * spicelib::SPD()) + ((9 as f64) * 60.0)) + 1.0);
        testutil::CHCKSD(&save.TITLE, save.RIGHT, b"~", save.XTIME, TTOL3, OK, ctx)?;

        spicelib::TIMOUT(
            save.LEFT,
            b"YYYY MON DD HR:MN:SC.###### ::TDB (TDB)",
            &mut save.TIMSTR,
            ctx,
        )?;

        // WRITE (*,*) I, ' ',  TIMSTR
        spicelib::TIMOUT(
            save.RIGHT,
            b"YYYY MON DD HR:MN:SC.###### ::TDB (TDB)",
            &mut save.TIMSTR,
            ctx,
        )?;
        // WRITE (*,*) '  ', TIMSTR
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Any occultation of BETA by ALPHA, abcorr = LT+S", ctx)?;
    //
    // Since Alpha can't occult Beta as seen from the Sun, we
    // should produce an empty result window.
    //

    //
    // Note: the stellar aberration correction spec should be ignored.
    //
    fstr::assign(&mut save.UTC0, b"2000 JAN 1  00:00:00 TDB");
    fstr::assign(&mut save.UTC1, b"2000 JAN 05 00:00:00 TDB");

    spicelib::STR2ET(&save.UTC0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&save.UTC1, &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED((2 * MAXWIN), save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED((2 * MAXWIN), save.RESULT.as_slice_mut(), ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;

    save.STEP = 60.0;

    spicelib::GFOCLT(
        b"Any",
        b"ALPHA",
        b"ellipsoid",
        b"ALPHAFIXED",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"LT+S",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(
        b"RESULT cardinality",
        spicelib::WNCARD(save.RESULT.as_slice(), ctx)?,
        b"=",
        0,
        0,
        OK,
        ctx,
    )?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Occultation of BETA center by ALPHA, abcorr = LT", ctx)?;
    //
    // Since Alpha can't occult Beta as seen from the Sun, we
    // should produce an empty result window.
    //

    //
    // Note: the stellar aberration correction spec should be ignored.
    //
    fstr::assign(&mut save.UTC0, b"2000 JAN 1  00:00:00 TDB");
    fstr::assign(&mut save.UTC1, b"2000 JAN 05 00:00:00 TDB");

    spicelib::STR2ET(&save.UTC0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&save.UTC1, &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED((2 * MAXWIN), save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED((2 * MAXWIN), save.RESULT.as_slice_mut(), ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;

    save.STEP = 60.0;

    spicelib::GFOCLT(
        b"Any",
        b"ALPHA",
        b"ellipsoid",
        b"ALPHAFIXED",
        b"BETA",
        b"point",
        b"BETAFIXED",
        b"LT",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(
        b"RESULT cardinality",
        spicelib::WNCARD(save.RESULT.as_slice(), ctx)?,
        b"=",
        0,
        0,
        OK,
        ctx,
    )?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Occultation of BETA by ALPHA center, abcorr = LT", ctx)?;
    //
    // Since Alpha can't occult Beta as seen from the Sun, we
    // should produce an empty result window.
    //

    //
    // Note: the stellar aberration correction spec should be ignored.
    //
    fstr::assign(&mut save.UTC0, b"2000 JAN 1  00:00:00 TDB");
    fstr::assign(&mut save.UTC1, b"2000 JAN 05 00:00:00 TDB");

    spicelib::STR2ET(&save.UTC0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&save.UTC1, &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED((2 * MAXWIN), save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED((2 * MAXWIN), save.RESULT.as_slice_mut(), ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;

    save.STEP = 60.0;

    spicelib::GFOCLT(
        b"Any",
        b"ALPHA",
        b"point",
        b"ALPHAFIXED",
        b"BETA",
        b"ellipsoid",
        b"BETAFIXED",
        b"LT",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(
        b"RESULT cardinality",
        spicelib::WNCARD(save.RESULT.as_slice(), ctx)?,
        b"=",
        0,
        0,
        OK,
        ctx,
    )?;

    //
    // Case
    //
    fstr::assign(&mut save.TITLE, b"Check the GF call uses the GFSTOL value");
    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // Re-run a valid search after using GFSTOL to set the convergence
    // tolerance to a value that should cause a numerical error signal.
    //

    fstr::assign(&mut save.UTC0, b"2000 JAN 1  00:00:00 TDB");
    fstr::assign(&mut save.UTC1, b"2000 JAN 05 00:00:00 TDB");

    spicelib::STR2ET(&save.UTC0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&save.UTC1, &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.STEP = 60.0;

    spicelib::GFOCLT(
        b"Any",
        b"BETA",
        b"POINT",
        b"BETAFIXED",
        b"ALPHA",
        b"ELLIPSOID",
        b"ALPHAFIXED",
        b"LT",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = 0;
    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKSI(b"N", save.N, b"!=", 0, 0, OK, ctx)?;

    spicelib::WNFETD(save.RESULT.as_slice(), 1, &mut save.BEG, &mut save.END, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Reset tol.
    //

    spicelib::GFSTOL(0.0001, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFOCLT(
        b"Any",
        b"BETA",
        b"POINT",
        b"BETAFIXED",
        b"ALPHA",
        b"ELLIPSOID",
        b"ALPHAFIXED",
        b"LT",
        b"SUN",
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = 0;
    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKSI(b"N", save.N, b"!=", 0, 0, OK, ctx)?;

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

    //***********************************************************************
    //
    //     DSK tests
    //
    //***********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Create a DSK containing shape models for bodies Alpha and Beta.",
        ctx,
    )?;

    //
    // Make sure the generic SPK is unloaded! We need the sun ephemeris
    // from Nat's SPK.
    //
    spicelib::SPKUEF(save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //
    // This block is suitable for use in F_GFOCLT. We don't actually
    // have to have the high-resolution DSK patches to test OCCULT.
    //
    // We'll enhance the generic DSK models for Alpha and Beta by
    // appending segments containing small patches of high-resolution
    // data for the surface regions that participate in computation of
    // accurate occultation ingress and egress times. We use small
    // patches because the size and run time needed to create full
    // models at this resolution would be prohibitive.
    //
    // Start by creating the basic DSK.
    //
    if spicelib::EXISTS(DSK, ctx)? {
        spicelib::UNLOAD(DSK, ctx)?;
        spicelib::DELFIL(DSK, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Use low resolution tessellations for the main models.
    //
    save.NLON = 20;
    save.NLAT = 10;

    fstr::assign(&mut save.AFRAME, b"ALPHAFIXED");
    fstr::assign(&mut save.BFRAME, b"BETAFIXED");

    testutil::NATDSK(
        DSK,
        &save.AFRAME,
        save.NLON,
        save.NLAT,
        &save.BFRAME,
        save.NLON,
        save.NLAT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create high-resolution patches for Alpha. At the J2000 epoch,
    // the ALPHAFIXED +X axis is aligned with the J2000 +Z axis. The
    // ALPHAFIXED +Y axis is aligned with the J2000 +Y axis.
    //
    //
    // Create the -Y patch for body Alpha. The patch covers
    // the lon/lat rectangle
    //
    //    -92 deg. <= lon <= -88 deg.
    //      0 deg. <= lat <=   4 deg.
    //
    // Note that Alpha' body-fixed Z axis lies in Alpha's orbital
    // plane.
    //
    save.BODYID = 1000;
    save.SURFID = 2;
    save.FIRST = -((100 as f64) * spicelib::JYEAR());
    save.LAST = ((100 as f64) * spicelib::JYEAR());

    spicelib::BODVCD(
        save.BODYID,
        b"RADII",
        3,
        &mut save.N,
        save.ARAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make the patch spherical, using the ellipsoid's Z
    // semi-axis length.
    //
    save.C = save.ARAD[3];

    save.CORSYS = LATSYS;
    spicelib::CLEARD(NSYPAR, save.CORPAR.as_slice_mut());

    save.MAKVTL = false;

    save.BOUNDS[[1, 1]] = (spicelib::RPD(ctx) * -92.0);
    save.BOUNDS[[2, 1]] = (spicelib::RPD(ctx) * -88.0);
    save.BOUNDS[[1, 2]] = (spicelib::RPD(ctx) * 0.0);
    save.BOUNDS[[2, 2]] = (spicelib::RPD(ctx) * 4.0);

    save.NLAT = 200;
    save.NLON = 200;

    save.USEPAD = true;

    //
    // Append the patch segment to the existing DSK.
    //
    testutil::T_SECDS2(
        save.BODYID,
        save.SURFID,
        &save.AFRAME,
        save.FIRST,
        save.LAST,
        save.CORSYS,
        save.CORPAR.as_slice(),
        save.BOUNDS.as_slice(),
        save.C,
        save.C,
        save.C,
        save.NLON,
        save.NLAT,
        save.MAKVTL,
        save.USEPAD,
        DSK,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create the +Y patch for body Alpha. The patch covers
    // the lon/lat rectangle
    //
    //     88 deg. <= lon <=  92 deg.
    //      0 deg. <= lat <=   4 deg.
    //

    save.SURFID = 3;

    save.BOUNDS[[1, 1]] = (spicelib::RPD(ctx) * 88.0);
    save.BOUNDS[[2, 1]] = (spicelib::RPD(ctx) * 92.0);
    save.BOUNDS[[1, 2]] = (spicelib::RPD(ctx) * 0.0);
    save.BOUNDS[[2, 2]] = (spicelib::RPD(ctx) * 4.0);

    save.NLAT = 200;
    save.NLON = 200;

    save.USEPAD = true;

    //
    // Append the patch segment to the existing DSK.
    //
    testutil::T_SECDS2(
        save.BODYID,
        save.SURFID,
        &save.AFRAME,
        save.FIRST,
        save.LAST,
        save.CORSYS,
        save.CORPAR.as_slice(),
        save.BOUNDS.as_slice(),
        save.C,
        save.C,
        save.C,
        save.NLON,
        save.NLAT,
        save.MAKVTL,
        save.USEPAD,
        DSK,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create high-resolution patches for Beta. At the J2000 epoch,
    // the BETAFIXED +X axis is aligned with the J2000 +Y axis. The
    // BETAFIXED +Y axis is aligned with the J2000 -X axis.
    //
    save.BODYID = 2000;
    save.SURFID = 2;
    save.FIRST = -((100 as f64) * spicelib::JYEAR());
    save.LAST = ((100 as f64) * spicelib::JYEAR());

    spicelib::BODVCD(
        save.BODYID,
        b"RADII",
        3,
        &mut save.N,
        save.BRAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make the patch spherical, using the ellipsoid's X
    // semi-axis length.
    //
    save.A = save.BRAD[1];

    save.CORSYS = LATSYS;
    spicelib::CLEARD(NSYPAR, save.CORPAR.as_slice_mut());

    save.MAKVTL = false;

    save.BOUNDS[[1, 1]] = (spicelib::RPD(ctx) * 0.0);
    save.BOUNDS[[2, 1]] = (spicelib::RPD(ctx) * 4.0);
    save.BOUNDS[[1, 2]] = (spicelib::RPD(ctx) * -2.0);
    save.BOUNDS[[2, 2]] = (spicelib::RPD(ctx) * 2.0);

    save.NLAT = 200;
    save.NLON = 200;

    save.USEPAD = true;

    testutil::T_SECDS2(
        save.BODYID,
        save.SURFID,
        &save.BFRAME,
        save.FIRST,
        save.LAST,
        save.CORSYS,
        save.CORPAR.as_slice(),
        save.BOUNDS.as_slice(),
        save.A,
        save.A,
        save.A,
        save.NLON,
        save.NLAT,
        save.MAKVTL,
        save.USEPAD,
        DSK,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BOUNDS[[1, 1]] = (spicelib::RPD(ctx) * 178.0);
    save.BOUNDS[[2, 1]] = (spicelib::RPD(ctx) * -178.0);
    save.BOUNDS[[1, 2]] = (spicelib::RPD(ctx) * -2.0);
    save.BOUNDS[[2, 2]] = (spicelib::RPD(ctx) * 2.0);

    testutil::T_SECDS2(
        save.BODYID,
        save.SURFID,
        &save.BFRAME,
        save.FIRST,
        save.LAST,
        save.CORSYS,
        save.CORPAR.as_slice(),
        save.BOUNDS.as_slice(),
        save.A,
        save.A,
        save.A,
        save.NLON,
        save.NLAT,
        save.MAKVTL,
        save.USEPAD,
        DSK,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load the DSK for subsequent use.
    //
    spicelib::FURNSH(DSK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Find transit of point BETA across DSK ALPHA near the epoch 2000 JAN 1 12:05:00 TDB",
        ctx,
    )?;

    spicelib::STR2ET(b"2000 JAN 1 11:00:00 TDB", &mut save.ET0, ctx)?;
    spicelib::STR2ET(b"2000 JAN 1 13:00:00 TDB", &mut save.ET1, ctx)?;

    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.OCCTYP, b"ANY");

    fstr::assign(&mut save.OBSRVR, b"SUN");
    fstr::assign(&mut save.ABCORR, b"NONE");

    fstr::assign(&mut save.FRONT, b"BETA");
    fstr::assign(&mut save.FSHAPE, b"POINT");
    fstr::assign(&mut save.FFRAME, b" ");
    fstr::assign(&mut save.BACK, b"ALPHA");
    fstr::assign(&mut save.BFRAME, b"ALPHAFIXED");
    fstr::assign(&mut save.BSHAPE, b"DSK/UNPRIORITIZED");

    spicelib::GFOCLT(
        &save.OCCTYP,
        &save.FRONT,
        &save.FSHAPE,
        &save.FFRAME,
        &save.BACK,
        &save.BSHAPE,
        &save.BFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    save.START = save.RESULT[1];
    save.FINISH = save.RESULT[2];

    save.XSTART = 60.0;
    save.XFINSH = ((9 as f64) * 60.0);

    //
    // On cayenne (PC/Linux/gfortran 64-bit), this tolerance may be
    // reduced to 1.D-6.
    //
    save.TOL = 0.000003;

    testutil::CHCKSD(b"START", save.START, b"~", save.XSTART, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"FINISH", save.FINISH, b"~", save.XFINSH, save.TOL, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Find occultation by DSK BETA of point ALPHA near the epoch 2000 JAN 1 12:05:00 TDB",
        ctx,
    )?;

    spicelib::STR2ET(b"2000 JAN 1 11:00:00 TDB", &mut save.ET0, ctx)?;
    spicelib::STR2ET(b"2000 JAN 1 13:00:00 TDB", &mut save.ET1, ctx)?;

    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.OCCTYP, b"ANY");

    fstr::assign(&mut save.OBSRVR, b"SUN");
    fstr::assign(&mut save.ABCORR, b"NONE");

    fstr::assign(&mut save.FRONT, b"BETA");
    fstr::assign(&mut save.FSHAPE, b"DSK/UNPRIORITIZED");
    fstr::assign(&mut save.FFRAME, b"BETAFIXED");
    fstr::assign(&mut save.BACK, b"ALPHA");
    fstr::assign(&mut save.BFRAME, b" ");
    fstr::assign(&mut save.BSHAPE, b"POINT");

    spicelib::GFOCLT(
        &save.OCCTYP,
        &save.FRONT,
        &save.FSHAPE,
        &save.FFRAME,
        &save.BACK,
        &save.BSHAPE,
        &save.BFRAME,
        &save.ABCORR,
        &save.OBSRVR,
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    save.START = save.RESULT[1];
    save.FINISH = save.RESULT[2];

    save.XSTART = ((4 as f64) * 60.0);
    save.XFINSH = ((6 as f64) * 60.0);

    //
    // On cayenne (PC/Linux/gfortran 64-bit), this tolerance may be
    // reduced to 1.D-6.
    //
    save.TOL = 0.000003;

    testutil::CHCKSD(b"START", save.START, b"~", save.XSTART, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"FINISH", save.FINISH, b"~", save.XFINSH, save.TOL, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //

    testutil::TCASE(b"Clean up. Unload and delete kernels.", ctx)?;

    //
    // Clean up SPK and DSK files.
    //
    spicelib::SPKUEF(save.HAN1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKUEF(save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(DSK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
