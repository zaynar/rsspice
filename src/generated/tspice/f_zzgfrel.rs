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
const LBCELL: i32 = -5;
const LNSIZE: i32 = 80;
const MAXWIN: i32 = 100;

struct SaveVars {
    QNAME: Vec<u8>,
    RELATE: Vec<u8>,
    SRCPRE: ActualCharArray,
    SRCSUF: ActualCharArray,
    ADJUST: f64,
    CNFINE: StackArray<f64, 106>,
    CRITPT: StackArray<f64, 100>,
    DELTA: f64,
    E: f64,
    FINISH: f64,
    FTOL: f64,
    REFVAL: f64,
    RESULT: StackArray<f64, 106>,
    SHORT: StackArray<f64, 7>,
    START: f64,
    STEPSZ: f64,
    TOL: f64,
    VALUE: f64,
    WORK: ActualArray2D<f64>,
    XBEG: StackArray<f64, 50>,
    XBEGVL: StackArray<f64, 50>,
    XEND: StackArray<f64, 50>,
    XENDVL: StackArray<f64, 50>,
    XTIME: StackArray<f64, 50>,
    XVAL: StackArray<f64, 50>,
    I: i32,
    J: i32,
    MW: i32,
    N: i32,
    NCRIT: i32,
    NW: i32,
    BAIL: bool,
    RPT: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut QNAME = vec![b' '; LNSIZE as usize];
        let mut RELATE = vec![b' '; LNSIZE as usize];
        let mut SRCPRE = ActualCharArray::new(LNSIZE, 1..=2);
        let mut SRCSUF = ActualCharArray::new(LNSIZE, 1..=2);
        let mut ADJUST: f64 = 0.0;
        let mut CNFINE = StackArray::<f64, 106>::new(LBCELL..=MAXWIN);
        let mut CRITPT = StackArray::<f64, 100>::new(1..=MAXWIN);
        let mut DELTA: f64 = 0.0;
        let mut E: f64 = 0.0;
        let mut FINISH: f64 = 0.0;
        let mut FTOL: f64 = 0.0;
        let mut REFVAL: f64 = 0.0;
        let mut RESULT = StackArray::<f64, 106>::new(LBCELL..=MAXWIN);
        let mut SHORT = StackArray::<f64, 7>::new(LBCELL..=1);
        let mut START: f64 = 0.0;
        let mut STEPSZ: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut VALUE: f64 = 0.0;
        let mut WORK = ActualArray2D::<f64>::new(LBCELL..=MAXWIN, 1..=NWMAX);
        let mut XBEG = StackArray::<f64, 50>::new(1..=(MAXWIN / 2));
        let mut XBEGVL = StackArray::<f64, 50>::new(1..=(MAXWIN / 2));
        let mut XEND = StackArray::<f64, 50>::new(1..=(MAXWIN / 2));
        let mut XENDVL = StackArray::<f64, 50>::new(1..=(MAXWIN / 2));
        let mut XTIME = StackArray::<f64, 50>::new(1..=(MAXWIN / 2));
        let mut XVAL = StackArray::<f64, 50>::new(1..=(MAXWIN / 2));
        let mut I: i32 = 0;
        let mut J: i32 = 0;
        let mut MW: i32 = 0;
        let mut N: i32 = 0;
        let mut NCRIT: i32 = 0;
        let mut NW: i32 = 0;
        let mut BAIL: bool = false;
        let mut RPT: bool = false;

        Self {
            QNAME,
            RELATE,
            SRCPRE,
            SRCSUF,
            ADJUST,
            CNFINE,
            CRITPT,
            DELTA,
            E,
            FINISH,
            FTOL,
            REFVAL,
            RESULT,
            SHORT,
            START,
            STEPSZ,
            TOL,
            VALUE,
            WORK,
            XBEG,
            XBEGVL,
            XEND,
            XENDVL,
            XTIME,
            XVAL,
            I,
            J,
            MW,
            N,
            NCRIT,
            NW,
            BAIL,
            RPT,
        }
    }
}

//$Procedure      F_ZZGFREL ( Test ZZGFREL )
pub fn F_ZZGFREL(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // External routines
    //

    //
    // Local parameters
    //

    //
    // Local Variables
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
    testutil::TOPEN(b"F_ZZGFREL", ctx)?;

    //*********************************************************************
    //*
    //*    Error cases
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Too few workspace windows.", ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(
        -spicelib::PI(ctx),
        spicelib::PI(ctx),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.0;
    save.REFVAL = 0.5;
    fstr::assign(&mut save.RELATE, b"=");
    save.TOL = 0.000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    T_ZZGFREL_SET(b"SIN(X)", ctx);
    T_ZZGFREL_UP(save.REFVAL, ctx);

    save.MW = MAXWIN;
    save.NW = 4;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(TOOFEWWINDOWS)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Workspace windows too short.", ctx)?;

    spicelib::SSIZED(1, save.SHORT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SHORT[1] = spicelib::PI(ctx);

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.0;
    save.REFVAL = 0.5;
    fstr::assign(&mut save.RELATE, b"=");
    save.TOL = 0.000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    T_ZZGFREL_SET(b"SIN(X)", ctx);
    T_ZZGFREL_UP(save.REFVAL, ctx);

    save.MW = 1;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.SHORT.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDIMENSION)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Result window too short; found before search", ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(
        -spicelib::PI(ctx),
        spicelib::PI(ctx),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(1, save.SHORT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.0;
    save.REFVAL = 0.5;
    fstr::assign(&mut save.RELATE, b"=");
    save.TOL = 0.000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    T_ZZGFREL_SET(b"SIN(X)", ctx);
    T_ZZGFREL_UP(save.REFVAL, ctx);

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.SHORT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDIMENSION)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Result window too short; found during search", ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(
        -((10 as f64) * spicelib::PI(ctx)),
        ((10 as f64) * spicelib::PI(ctx)),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(12, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // This case requires an actual search, so set the step size.
    //
    save.STEPSZ = 0.25;
    spicelib::GFSSTP(save.STEPSZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.0;
    save.REFVAL = 0.5;
    fstr::assign(&mut save.RELATE, b"LOCMAX");
    save.TOL = 0.000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    T_ZZGFREL_SET(b"SIN(X)", ctx);
    T_ZZGFREL_UP(save.REFVAL, ctx);

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(OUTOFROOM)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Bad adjustment value", ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(
        -spicelib::PI(ctx),
        spicelib::PI(ctx),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = -1.0;
    save.REFVAL = 0.5;
    fstr::assign(&mut save.RELATE, b"ABSMAX");
    save.TOL = 0.000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    T_ZZGFREL_SET(b"SIN(X)", ctx);
    T_ZZGFREL_UP(save.REFVAL, ctx);

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Non-positive tolerance value", ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(
        -spicelib::PI(ctx),
        spicelib::PI(ctx),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.0;
    save.REFVAL = 0.5;
    fstr::assign(&mut save.RELATE, b"ABSMAX");
    save.TOL = -0.000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    T_ZZGFREL_SET(b"SIN(X)", ctx);
    T_ZZGFREL_UP(save.REFVAL, ctx);

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDTOLERANCE)", OK, ctx)?;

    save.TOL = 0.0;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDTOLERANCE)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Bad relational operator", ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(
        -spicelib::PI(ctx),
        spicelib::PI(ctx),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.0;
    save.REFVAL = 0.5;
    fstr::assign(&mut save.RELATE, b"==");

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    T_ZZGFREL_SET(b"SIN(X)", ctx);
    T_ZZGFREL_UP(save.REFVAL, ctx);

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTRECOGNIZED)", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Normal cases
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Solve SIN(X) == 0.5 on [-pi,pi]", ctx)?;

    //
    // This case requires an actual search, so set the step size.
    //
    save.STEPSZ = 0.25;
    spicelib::GFSSTP(save.STEPSZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(
        -spicelib::PI(ctx),
        spicelib::PI(ctx),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.0;
    save.REFVAL = 0.5;
    fstr::assign(&mut save.RELATE, b"=");

    //
    // We should be able to work with a very tight tolerance, since
    // our independent variable is close to zero.
    //
    save.TOL = 0.0000000000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    T_ZZGFREL_SET(b"SIN(X)", ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_ZZGFREL_UP(save.REFVAL, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect two roots.
    //
    testutil::CHCKSI(b"N", save.N, b"=", 2, 0, OK, ctx)?;

    //
    // Check the times of and function values at the roots.
    //
    save.XTIME[1] = (spicelib::PI(ctx) / 6 as f64);
    save.XTIME[2] = (((5 as f64) * spicelib::PI(ctx)) / 6 as f64);

    save.XVAL[1] = 0.5;
    save.XVAL[2] = 0.5;

    if *OK {
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

                //
                // Check start time.
                //
                fstr::assign(&mut save.QNAME, b"Start time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                testutil::CHCKSD(
                    &save.QNAME,
                    save.START,
                    b"~",
                    save.XTIME[save.I],
                    save.TOL,
                    OK,
                    ctx,
                )?;

                //
                // Make sure stop time exactly matches start time.
                //
                fstr::assign(&mut save.QNAME, b"Stop time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                testutil::CHCKSD(&save.QNAME, save.FINISH, b"=", save.START, 0.0, OK, ctx)?;

                //
                // Check function value at start time. The function
                // has derivative magnitude not exceeding 1, so the
                // function tolerance can be set equal to the time
                // tolerance.
                //
                fstr::assign(&mut save.QNAME, b"Start value #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                T_ZZGFREL_GET(save.START, &mut save.VALUE, ctx)?;

                save.FTOL = save.TOL;

                testutil::CHCKSD(
                    &save.QNAME,
                    save.VALUE,
                    b"~",
                    save.XVAL[save.I],
                    save.FTOL,
                    OK,
                    ctx,
                )?;

                save.I += m3__;
            }
        }
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Solve SIN(X) < 0.5 on [-pi,pi]", ctx)?;

    save.STEPSZ = 0.25;
    spicelib::GFSSTP(save.STEPSZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(
        -spicelib::PI(ctx),
        spicelib::PI(ctx),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.0;
    save.REFVAL = 0.5;
    fstr::assign(&mut save.RELATE, b"<");
    //
    // We should be able to work with a very tight tolerance, since
    // our independent variable is close to zero.
    //
    save.TOL = 0.0000000000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    T_ZZGFREL_SET(b"SIN(X)", ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_ZZGFREL_UP(save.REFVAL, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect two solution intervals.
    //
    testutil::CHCKSI(b"N", save.N, b"=", 2, 0, OK, ctx)?;

    //
    // Check the times of and function values at the roots.
    //
    save.XBEG[1] = -spicelib::PI(ctx);
    save.XEND[1] = (spicelib::PI(ctx) / 6 as f64);
    save.XBEG[2] = (((5 as f64) * spicelib::PI(ctx)) / 6 as f64);
    save.XEND[2] = spicelib::PI(ctx);

    save.XBEGVL[1] = 0.0;
    save.XENDVL[1] = 0.5;
    save.XBEGVL[2] = 0.5;
    save.XENDVL[2] = 0.0;

    if *OK {
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
                //
                // Check start time.
                //
                fstr::assign(&mut save.QNAME, b"Start time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                testutil::CHCKSD(
                    &save.QNAME,
                    save.START,
                    b"~",
                    save.XBEG[save.I],
                    save.TOL,
                    OK,
                    ctx,
                )?;

                //
                // Check function value at start time. The function
                // has derivative magnitude not exceeding 1, so the
                // function tolerance can be set equal to the time
                // tolerance.
                //
                fstr::assign(&mut save.QNAME, b"Start value #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                T_ZZGFREL_GET(save.START, &mut save.VALUE, ctx)?;

                save.FTOL = save.TOL;

                testutil::CHCKSD(
                    &save.QNAME,
                    save.VALUE,
                    b"~",
                    save.XBEGVL[save.I],
                    save.FTOL,
                    OK,
                    ctx,
                )?;

                //
                // Check stop time and value.
                //
                fstr::assign(&mut save.QNAME, b"Stop time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                testutil::CHCKSD(
                    &save.QNAME,
                    save.FINISH,
                    b"~",
                    save.XEND[save.I],
                    save.TOL,
                    OK,
                    ctx,
                )?;

                fstr::assign(&mut save.QNAME, b"Stop value #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                T_ZZGFREL_GET(save.FINISH, &mut save.VALUE, ctx)?;

                testutil::CHCKSD(
                    &save.QNAME,
                    save.VALUE,
                    b"~",
                    save.XENDVL[save.I],
                    save.FTOL,
                    OK,
                    ctx,
                )?;

                save.I += m3__;
            }
        }
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Solve SIN(X) > 0.5 on [-pi,pi]", ctx)?;

    save.STEPSZ = 0.25;
    spicelib::GFSSTP(save.STEPSZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(
        -spicelib::PI(ctx),
        spicelib::PI(ctx),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.0;
    save.REFVAL = 0.5;
    fstr::assign(&mut save.RELATE, b">");
    //
    // We should be able to work with a very tight tolerance, since
    // our independent variable is close to zero.
    //
    save.TOL = 0.0000000000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    T_ZZGFREL_SET(b"SIN(X)", ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_ZZGFREL_UP(save.REFVAL, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect one solution interval.
    //
    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    //
    // Check the times of and function values at the roots.
    //
    save.XBEG[1] = (spicelib::PI(ctx) / 6 as f64);
    save.XEND[1] = (((5 as f64) * spicelib::PI(ctx)) / 6 as f64);

    save.XBEGVL[1] = 0.5;
    save.XENDVL[1] = 0.5;

    if *OK {
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
                //
                // Check start time.
                //
                fstr::assign(&mut save.QNAME, b"Start time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                testutil::CHCKSD(
                    &save.QNAME,
                    save.START,
                    b"~",
                    save.XBEG[save.I],
                    save.TOL,
                    OK,
                    ctx,
                )?;

                //
                // Check function value at start time. The function
                // has derivative magnitude not exceeding 1, so the
                // function tolerance can be set equal to the time
                // tolerance.
                //
                fstr::assign(&mut save.QNAME, b"Start value #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                T_ZZGFREL_GET(save.START, &mut save.VALUE, ctx)?;

                save.FTOL = save.TOL;

                testutil::CHCKSD(
                    &save.QNAME,
                    save.VALUE,
                    b"~",
                    save.XBEGVL[save.I],
                    save.FTOL,
                    OK,
                    ctx,
                )?;

                //
                // Check stop time and value.
                //
                fstr::assign(&mut save.QNAME, b"Stop time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                testutil::CHCKSD(
                    &save.QNAME,
                    save.FINISH,
                    b"~",
                    save.XEND[save.I],
                    save.TOL,
                    OK,
                    ctx,
                )?;

                fstr::assign(&mut save.QNAME, b"Stop value #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                T_ZZGFREL_GET(save.FINISH, &mut save.VALUE, ctx)?;

                testutil::CHCKSD(
                    &save.QNAME,
                    save.VALUE,
                    b"~",
                    save.XENDVL[save.I],
                    save.FTOL,
                    OK,
                    ctx,
                )?;

                save.I += m3__;
            }
        }
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Find local maxima of SIN(X) on [0,10*pi]", ctx)?;

    save.STEPSZ = 0.25;
    spicelib::GFSSTP(save.STEPSZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(
        0.0,
        ((10 as f64) * spicelib::PI(ctx)),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.0;
    save.REFVAL = 0.0;
    fstr::assign(&mut save.RELATE, b"LOCMAX");
    //
    // We should be able to work with a very tight tolerance, since
    // our independent variable is close to zero.
    //
    save.TOL = 0.0000000000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    T_ZZGFREL_SET(b"SIN(X)", ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_ZZGFREL_UP(save.REFVAL, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect 5 solutions.
    //
    testutil::CHCKSI(b"N", save.N, b"=", 5, 0, OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 5;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.XBEG[save.I] = (((((save.I - 1) * 2) as f64) * spicelib::PI(ctx))
                + (spicelib::PI(ctx) / 2 as f64));
            save.XEND[save.I] = save.XBEG[save.I];

            save.XBEGVL[save.I] = 1.0;

            save.I += m3__;
        }
    }

    //
    // Check the times of and function values at the roots.
    //
    if *OK {
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
                //
                // Check start time.
                //
                fstr::assign(&mut save.QNAME, b"Start time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                testutil::CHCKSD(
                    &save.QNAME,
                    save.START,
                    b"~",
                    save.XBEG[save.I],
                    save.TOL,
                    OK,
                    ctx,
                )?;

                //
                // Check function value at start time. The function
                // has derivative magnitude not exceeding 1, so the
                // function tolerance can be set equal to the time
                // tolerance.
                //
                fstr::assign(&mut save.QNAME, b"Start value #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                T_ZZGFREL_GET(save.START, &mut save.VALUE, ctx)?;

                save.FTOL = save.TOL;

                testutil::CHCKSD(
                    &save.QNAME,
                    save.VALUE,
                    b"~",
                    save.XBEGVL[save.I],
                    save.FTOL,
                    OK,
                    ctx,
                )?;

                //
                // Check that stop time equals start time.
                //
                fstr::assign(&mut save.QNAME, b"Stop time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                testutil::CHCKSD(&save.QNAME, save.FINISH, b"=", save.START, 0.0, OK, ctx)?;

                save.I += m3__;
            }
        }
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Find local minima of SIN(X) on [0,10*pi]", ctx)?;

    save.STEPSZ = 0.25;
    spicelib::GFSSTP(save.STEPSZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(
        0.0,
        ((10 as f64) * spicelib::PI(ctx)),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.0;
    save.REFVAL = 0.0;
    fstr::assign(&mut save.RELATE, b"LOCMIN");
    //
    // We should be able to work with a very tight tolerance, since
    // our independent variable is close to zero.
    //
    save.TOL = 0.0000000000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    T_ZZGFREL_SET(b"SIN(X)", ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_ZZGFREL_UP(save.REFVAL, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect 5 solutions.
    //
    testutil::CHCKSI(b"N", save.N, b"=", 5, 0, OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 5;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.XBEG[save.I] = (((((save.I - 1) * 2) as f64) * spicelib::PI(ctx))
                + (((3 as f64) * spicelib::PI(ctx)) / 2 as f64));
            save.XEND[save.I] = save.XBEG[save.I];

            save.XBEGVL[save.I] = -1.0;

            save.I += m3__;
        }
    }

    //
    // Check the times of and function values at the roots.
    //
    if *OK {
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
                //
                // Check start time.
                //
                fstr::assign(&mut save.QNAME, b"Start time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                testutil::CHCKSD(
                    &save.QNAME,
                    save.START,
                    b"~",
                    save.XBEG[save.I],
                    save.TOL,
                    OK,
                    ctx,
                )?;

                //
                // Check function value at start time. The function
                // has derivative magnitude not exceeding 1, so the
                // function tolerance can be set equal to the time
                // tolerance.
                //
                fstr::assign(&mut save.QNAME, b"Start value #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                T_ZZGFREL_GET(save.START, &mut save.VALUE, ctx)?;

                save.FTOL = save.TOL;

                testutil::CHCKSD(
                    &save.QNAME,
                    save.VALUE,
                    b"~",
                    save.XBEGVL[save.I],
                    save.FTOL,
                    OK,
                    ctx,
                )?;

                //
                // Check that stop time equals start time.
                //
                fstr::assign(&mut save.QNAME, b"Stop time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                testutil::CHCKSD(&save.QNAME, save.FINISH, b"=", save.START, 0.0, OK, ctx)?;

                save.I += m3__;
            }
        }
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Find adjusted absolute maxima of SIN(X) on [0,10*pi]", ctx)?;

    save.STEPSZ = 0.25;
    spicelib::GFSSTP(save.STEPSZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(
        0.0,
        ((10 as f64) * spicelib::PI(ctx)),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.5;
    save.REFVAL = 0.0;
    fstr::assign(&mut save.RELATE, b"ABSMAX");
    //
    // We should be able to work with a very tight tolerance, since
    // our independent variable is close to zero.
    //
    save.TOL = 0.0000000000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    T_ZZGFREL_SET(b"SIN(X)", ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_ZZGFREL_UP(save.REFVAL, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect 5 solutions.
    //
    testutil::CHCKSI(b"N", save.N, b"=", 5, 0, OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 5;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.XBEG[save.I] = (((((save.I - 1) * 2) as f64) * spicelib::PI(ctx))
                + (spicelib::PI(ctx) / 6 as f64));
            save.XEND[save.I] = (((((save.I - 1) * 2) as f64) * spicelib::PI(ctx))
                + (((5 as f64) * spicelib::PI(ctx)) / 6 as f64));

            save.XBEGVL[save.I] = 0.5;
            save.XENDVL[save.I] = 0.5;

            save.I += m3__;
        }
    }

    //
    // Check the times of and function values at the roots.
    //
    if *OK {
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
                //
                // Check start time.
                //
                fstr::assign(&mut save.QNAME, b"Start time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                testutil::CHCKSD(
                    &save.QNAME,
                    save.START,
                    b"~",
                    save.XBEG[save.I],
                    save.TOL,
                    OK,
                    ctx,
                )?;

                //
                // Check function value at start time. The function
                // has derivative magnitude not exceeding 1, so the
                // function tolerance can be set equal to the time
                // tolerance.
                //
                fstr::assign(&mut save.QNAME, b"Start value #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                T_ZZGFREL_GET(save.START, &mut save.VALUE, ctx)?;

                save.FTOL = save.TOL;

                testutil::CHCKSD(
                    &save.QNAME,
                    save.VALUE,
                    b"~",
                    save.XBEGVL[save.I],
                    save.FTOL,
                    OK,
                    ctx,
                )?;

                //
                // Check stop time.
                //
                fstr::assign(&mut save.QNAME, b"Stop time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                testutil::CHCKSD(
                    &save.QNAME,
                    save.FINISH,
                    b"~",
                    save.XEND[save.I],
                    save.TOL,
                    OK,
                    ctx,
                )?;

                //
                // Check function value at stop time. The function
                // has derivative magnitude not exceeding 1, so the
                // function tolerance can be set equal to the time
                // tolerance.
                //
                fstr::assign(&mut save.QNAME, b"Stop value #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                T_ZZGFREL_GET(save.FINISH, &mut save.VALUE, ctx)?;

                save.FTOL = save.TOL;

                testutil::CHCKSD(
                    &save.QNAME,
                    save.VALUE,
                    b"~",
                    save.XENDVL[save.I],
                    save.FTOL,
                    OK,
                    ctx,
                )?;

                save.I += m3__;
            }
        }
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Find adjusted absolute minima of SIN(X) on [0,10*pi]", ctx)?;

    save.STEPSZ = 0.25;
    spicelib::GFSSTP(save.STEPSZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(
        0.0,
        ((10 as f64) * spicelib::PI(ctx)),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.5;
    save.REFVAL = 0.0;
    fstr::assign(&mut save.RELATE, b"ABSMIN");
    //
    // We should be able to work with a very tight tolerance, since
    // our independent variable is close to zero.
    //
    save.TOL = 0.0000000000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    T_ZZGFREL_SET(b"SIN(X)", ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_ZZGFREL_UP(save.REFVAL, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect 5 solutions.
    //
    testutil::CHCKSI(b"N", save.N, b"=", 5, 0, OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 5;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.XBEG[save.I] = (((((save.I - 1) * 2) as f64) * spicelib::PI(ctx))
                + (((7 as f64) * spicelib::PI(ctx)) / 6 as f64));
            save.XEND[save.I] = (((((save.I - 1) * 2) as f64) * spicelib::PI(ctx))
                + (((11 as f64) * spicelib::PI(ctx)) / 6 as f64));

            save.XBEGVL[save.I] = -0.5;
            save.XENDVL[save.I] = -0.5;

            save.I += m3__;
        }
    }

    //
    // Check the times of and function values at the roots.
    //
    if *OK {
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
                //
                // Check start time.
                //
                fstr::assign(&mut save.QNAME, b"Start time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                testutil::CHCKSD(
                    &save.QNAME,
                    save.START,
                    b"~",
                    save.XBEG[save.I],
                    save.TOL,
                    OK,
                    ctx,
                )?;

                //
                // Check function value at start time. The function
                // has derivative magnitude not exceeding 1, so the
                // function tolerance can be set equal to the time
                // tolerance.
                //
                fstr::assign(&mut save.QNAME, b"Start value #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                T_ZZGFREL_GET(save.START, &mut save.VALUE, ctx)?;

                save.FTOL = save.TOL;

                testutil::CHCKSD(
                    &save.QNAME,
                    save.VALUE,
                    b"~",
                    save.XBEGVL[save.I],
                    save.FTOL,
                    OK,
                    ctx,
                )?;

                //
                // Check stop time.
                //
                fstr::assign(&mut save.QNAME, b"Stop time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                testutil::CHCKSD(
                    &save.QNAME,
                    save.FINISH,
                    b"~",
                    save.XEND[save.I],
                    save.TOL,
                    OK,
                    ctx,
                )?;

                //
                // Check function value at stop time. The function
                // has derivative magnitude not exceeding 1, so the
                // function tolerance can be set equal to the time
                // tolerance.
                //
                fstr::assign(&mut save.QNAME, b"Stop value #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                T_ZZGFREL_GET(save.FINISH, &mut save.VALUE, ctx)?;

                save.FTOL = save.TOL;

                testutil::CHCKSD(
                    &save.QNAME,
                    save.VALUE,
                    b"~",
                    save.XENDVL[save.I],
                    save.FTOL,
                    OK,
                    ctx,
                )?;

                save.I += m3__;
            }
        }
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Find absolute maximum of X on [-10*pi,10*pi]", ctx)?;

    save.STEPSZ = 0.25;
    spicelib::GFSSTP(save.STEPSZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(
        -((10 as f64) * spicelib::PI(ctx)),
        ((10 as f64) * spicelib::PI(ctx)),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.0;
    save.REFVAL = 0.0;
    fstr::assign(&mut save.RELATE, b"ABSMAX");
    //
    // We should be able to work with a very tight tolerance, since
    // our independent variable is close to zero.
    //
    save.TOL = 0.0000000000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    T_ZZGFREL_SET(b"X", ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_ZZGFREL_UP(save.REFVAL, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect 1 solution.
    //
    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    save.XBEG[1] = ((10 as f64) * spicelib::PI(ctx));
    save.XEND[1] = save.XBEG[1];

    save.XBEGVL[1] = save.XBEG[1];

    //
    // Check the times of and function values at the roots.
    //
    if *OK {
        spicelib::WNFETD(
            save.RESULT.as_slice(),
            1,
            &mut save.START,
            &mut save.FINISH,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check start time.
        //
        fstr::assign(&mut save.QNAME, b"Start time #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

        testutil::CHCKSD(
            &save.QNAME,
            save.START,
            b"~",
            save.XBEG[1],
            save.TOL,
            OK,
            ctx,
        )?;

        //
        // Check function value at start time. The function
        // has derivative magnitude not exceeding 1, so the
        // function tolerance can be set equal to the time
        // tolerance.
        //
        fstr::assign(&mut save.QNAME, b"Start value #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

        T_ZZGFREL_GET(save.START, &mut save.VALUE, ctx)?;

        save.FTOL = save.TOL;

        testutil::CHCKSD(
            &save.QNAME,
            save.VALUE,
            b"~",
            save.XBEGVL[1],
            save.FTOL,
            OK,
            ctx,
        )?;

        //
        // Check stop time.
        //
        fstr::assign(&mut save.QNAME, b"Stop time #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

        testutil::CHCKSD(&save.QNAME, save.FINISH, b"=", save.START, 0.0, OK, ctx)?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Find absolute minimum of X on [-10*pi,10*pi]", ctx)?;

    save.STEPSZ = 0.25;
    spicelib::GFSSTP(save.STEPSZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(
        -((10 as f64) * spicelib::PI(ctx)),
        ((10 as f64) * spicelib::PI(ctx)),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.0;
    save.REFVAL = 0.0;
    fstr::assign(&mut save.RELATE, b"ABSMIN");
    //
    // We should be able to work with a very tight tolerance, since
    // our independent variable is close to zero.
    //
    save.TOL = 0.0000000000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    T_ZZGFREL_SET(b"X", ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_ZZGFREL_UP(save.REFVAL, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect 1 solution.
    //
    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    save.XBEG[1] = -((10 as f64) * spicelib::PI(ctx));
    save.XEND[1] = save.XBEG[1];

    save.XBEGVL[1] = save.XBEG[1];

    //
    // Check the times of and function values at the roots.
    //
    if *OK {
        spicelib::WNFETD(
            save.RESULT.as_slice(),
            1,
            &mut save.START,
            &mut save.FINISH,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check start time.
        //
        fstr::assign(&mut save.QNAME, b"Start time #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

        testutil::CHCKSD(
            &save.QNAME,
            save.START,
            b"~",
            save.XBEG[1],
            save.TOL,
            OK,
            ctx,
        )?;

        //
        // Check function value at start time. The function
        // has derivative magnitude not exceeding 1, so the
        // function tolerance can be set equal to the time
        // tolerance.
        //
        fstr::assign(&mut save.QNAME, b"Start value #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

        T_ZZGFREL_GET(save.START, &mut save.VALUE, ctx)?;

        save.FTOL = save.TOL;

        testutil::CHCKSD(
            &save.QNAME,
            save.VALUE,
            b"~",
            save.XBEGVL[1],
            save.FTOL,
            OK,
            ctx,
        )?;

        //
        // Check stop time.
        //
        fstr::assign(&mut save.QNAME, b"Stop time #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

        testutil::CHCKSD(&save.QNAME, save.FINISH, b"=", save.START, 0.0, OK, ctx)?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Find critical points of (X**2)SIN(X) on [-10*pi,10*pi]",
        ctx,
    )?;

    save.STEPSZ = 0.25;
    spicelib::GFSSTP(save.STEPSZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(
        -((10 as f64) * spicelib::PI(ctx)),
        ((10 as f64) * spicelib::PI(ctx)),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.0;
    save.REFVAL = 0.0;
    fstr::assign(&mut save.RELATE, b"=");
    //
    // We should be able to work with a very tight tolerance, since
    // our independent variable is close to zero.
    //
    save.TOL = 0.0000000000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    //
    // Set the function to the derivative of (X**2)*SIN(X).
    //
    T_ZZGFREL_SET(b"D((X**2)*SIN(X))", ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_ZZGFREL_UP(save.REFVAL, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // If we had infinite precision, we would expect 21 solution
    // points: there is a critical point at 0, since the function
    //
    //     2
    //    X  * SIN(X)
    //
    // is asymptotic to X**3 as X -> 0. However, the root at 0
    // is a local minimum of the derivative: the derivative is
    // tangent to the X-axis at 0. So the GF system is unlikely
    // to be able to to detect this root. We require only that
    // at least 20 roots are found.
    //
    //
    testutil::CHCKSI(b"N", save.N, b">", 19, 0, OK, ctx)?;

    save.NCRIT = save.N;

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
            save.CRITPT[save.I] = save.START;

            save.I += m3__;
        }
    }

    //
    // If we did find a critical point at zero, delete it from
    // the set of critical points.
    //
    if (save.NCRIT == 21) {
        {
            let m1__: i32 = 10;
            let m2__: i32 = 20;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.CRITPT[save.I] = save.CRITPT[(save.I + 1)];
                save.I += m3__;
            }
        }

        save.NCRIT = 20;
    }

    //
    // Check function values, but not the times, at the roots.
    //
    if *OK {
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

                //
                // Check function value at start time. The function
                // has derivative amplitude on the order of 100, so the
                // function tolerance must be at least 2 orders of magnitude
                // looser than the time tolerance. Use 3 orders of magnitude.
                //
                fstr::assign(&mut save.QNAME, b"Start value #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                T_ZZGFREL_GET(save.START, &mut save.VALUE, ctx)?;

                save.FTOL = (save.TOL * 1000.0);

                testutil::CHCKSD(&save.QNAME, save.VALUE, b"~", 0.0, save.FTOL, OK, ctx)?;

                save.I += m3__;
            }
        }
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Find local maxima of (X**2)SIN(X) on [-10*pi,10*pi]", ctx)?;

    save.STEPSZ = 0.25;
    spicelib::GFSSTP(save.STEPSZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(
        -((10 as f64) * spicelib::PI(ctx)),
        ((10 as f64) * spicelib::PI(ctx)),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.0;
    save.REFVAL = 0.0;
    fstr::assign(&mut save.RELATE, b"LOCMAX");
    //
    // We should be able to work with a very tight tolerance, since
    // our independent variable is close to zero.
    //
    save.TOL = 0.0000000000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    //
    // Set the function to (X**2)*SIN(X).
    //
    T_ZZGFREL_SET(b"(X**2)*SIN(X)", ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_ZZGFREL_UP(save.REFVAL, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We should find 10 local maxima.
    //
    testutil::CHCKSI(b"N", save.N, b"=", 10, 0, OK, ctx)?;

    //
    // Set the expected start times. Local maxima occur between
    // 0 and pi and in intervals offset from [0,pi] by multiples
    // of 2*pi.
    //
    save.J = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 20;
        let m3__: i32 = 2;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.XBEG[save.J] = save.CRITPT[save.I];
            save.J = (save.J + 1);

            save.I += m3__;
        }
    }

    //
    // Check the start times for the local maxima.
    //
    if *OK {
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

                //
                // Check start time.
                //
                fstr::assign(&mut save.QNAME, b"Start time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                testutil::CHCKSD(
                    &save.QNAME,
                    save.START,
                    b"~",
                    save.XBEG[save.I],
                    save.TOL,
                    OK,
                    ctx,
                )?;

                save.I += m3__;
            }
        }
    }

    //
    // Check the 1st and 2nd derivative at each solution epoch.
    //
    save.FTOL = 0.0000000001;

    if *OK {
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

                //
                // Check the function at the start time; the function
                // should be positive.
                //
                fstr::assign(&mut save.QNAME, b"Function at time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                T_ZZGFREL_SET(b"(X**2)*SIN(X)", ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                T_ZZGFREL_GET(save.START, &mut save.VALUE, ctx)?;

                testutil::CHCKSD(&save.QNAME, save.VALUE, b">", 0.0, save.FTOL, OK, ctx)?;

                //
                // Check the derivative at the start time.
                //
                fstr::assign(&mut save.QNAME, b"Derivative at time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                T_ZZGFREL_SET(b"D((X**2)*SIN(X))", ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                T_ZZGFREL_GET(save.START, &mut save.VALUE, ctx)?;

                testutil::CHCKSD(&save.QNAME, save.VALUE, b"~", 0.0, save.FTOL, OK, ctx)?;

                //
                // Check the 2nd derivative at the start time.
                //
                fstr::assign(&mut save.QNAME, b"2nd derivative at time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                T_ZZGFREL_SET(b"D2((X**2)*SIN(X))", ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                T_ZZGFREL_GET(save.START, &mut save.VALUE, ctx)?;

                testutil::CHCKSD(&save.QNAME, save.VALUE, b"<", 0.0, save.FTOL, OK, ctx)?;

                save.I += m3__;
            }
        }
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Find absolute maximum of (X**2)SIN(X) on [-10*pi,10*pi]",
        ctx,
    )?;

    save.STEPSZ = 0.25;
    spicelib::GFSSTP(save.STEPSZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(
        -((10 as f64) * spicelib::PI(ctx)),
        ((10 as f64) * spicelib::PI(ctx)),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.0;
    save.REFVAL = 0.0;
    fstr::assign(&mut save.RELATE, b"ABSMAX");
    //
    // We should be able to work with a very tight tolerance, since
    // our independent variable is close to zero.
    //
    save.TOL = 0.0000000000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    //
    // Set the function to (X**2)*SIN(X).
    //
    T_ZZGFREL_SET(b"(X**2)*SIN(X)", ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_ZZGFREL_UP(save.REFVAL, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We should find 1 absolute maximum.
    //
    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    //
    // Set the expected start time. Local maxima occur between
    // 0 and pi and in intervals offset from [0,pi] by multiples
    // of 2*pi. Given the confinement interval we've picked,
    // the first local maximum should be the absolute
    // maximum.
    //
    save.XBEG[1] = save.CRITPT[1];

    //
    // Check the start time for the absolute maximum.
    //
    if *OK {
        spicelib::WNFETD(
            save.RESULT.as_slice(),
            1,
            &mut save.START,
            &mut save.FINISH,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check start time.
        //
        fstr::assign(&mut save.QNAME, b"Start time #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

        testutil::CHCKSD(
            &save.QNAME,
            save.START,
            b"~",
            save.XBEG[1],
            save.TOL,
            OK,
            ctx,
        )?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Find absolute maximum of (X**2)SIN(X) on [-9*pi,10*pi]",
        ctx,
    )?;

    save.STEPSZ = 0.25;
    spicelib::GFSSTP(save.STEPSZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(
        -((9 as f64) * spicelib::PI(ctx)),
        ((10 as f64) * spicelib::PI(ctx)),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.0;
    save.REFVAL = 0.0;
    fstr::assign(&mut save.RELATE, b"ABSMAX");
    //
    // We should be able to work with a very tight tolerance, since
    // our independent variable is close to zero.
    //
    save.TOL = 0.0000000000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    //
    // Set the function to (X**2)*SIN(X).
    //
    T_ZZGFREL_SET(b"(X**2)*SIN(X)", ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_ZZGFREL_UP(save.REFVAL, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We should find 1 absolute maximum.
    //
    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    //
    // Set the expected start time. Local maxima occur between
    // 0 and pi and in intervals offset from [0,pi] by multiples
    // of 2*pi. Given the confinement interval we've picked,
    // the last local maximum should be the absolute
    // maximum.
    //
    save.XBEG[1] = save.CRITPT[19];

    //
    // Check the start time for the absolute maximum.
    //
    if *OK {
        spicelib::WNFETD(
            save.RESULT.as_slice(),
            1,
            &mut save.START,
            &mut save.FINISH,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check start time.
        //
        fstr::assign(&mut save.QNAME, b"Start time #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

        testutil::CHCKSD(
            &save.QNAME,
            save.START,
            b"~",
            save.XBEG[1],
            save.TOL,
            OK,
            ctx,
        )?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Find local minima of (X**2)SIN(X) on [-10*pi,10*pi]", ctx)?;

    save.STEPSZ = 0.25;
    spicelib::GFSSTP(save.STEPSZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(
        -((10 as f64) * spicelib::PI(ctx)),
        ((10 as f64) * spicelib::PI(ctx)),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.0;
    save.REFVAL = 0.0;
    fstr::assign(&mut save.RELATE, b"LOCMIN");
    //
    // We should be able to work with a very tight tolerance, since
    // our independent variable is close to zero.
    //
    save.TOL = 0.0000000000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    //
    // Set the function to (X**2)*SIN(X).
    //
    T_ZZGFREL_SET(b"(X**2)*SIN(X)", ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_ZZGFREL_UP(save.REFVAL, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We should find 10 local minima.
    //
    testutil::CHCKSI(b"N", save.N, b"=", 10, 0, OK, ctx)?;

    //
    // Set the expected start times. Local minima occur between
    // -pi and 0 and in intervals offset from [-pi,0] by multiples
    // of 2*pi.
    //
    save.J = 1;

    {
        let m1__: i32 = 2;
        let m2__: i32 = 20;
        let m3__: i32 = 2;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.XBEG[save.J] = save.CRITPT[save.I];
            save.J = (save.J + 1);

            save.I += m3__;
        }
    }

    //
    // Check the start times for the local minima.
    //
    if *OK {
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

                //
                // Check start time.
                //
                fstr::assign(&mut save.QNAME, b"Start time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                testutil::CHCKSD(
                    &save.QNAME,
                    save.START,
                    b"~",
                    save.XBEG[save.I],
                    save.TOL,
                    OK,
                    ctx,
                )?;

                save.I += m3__;
            }
        }
    }

    //
    // Check the 1st and 2nd derivative at each solution epoch.
    //
    save.FTOL = 0.0000000001;

    if *OK {
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

                //
                // Check the function at the start time; the function
                // should be negative.
                //
                fstr::assign(&mut save.QNAME, b"Function at time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                T_ZZGFREL_SET(b"(X**2)*SIN(X)", ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                T_ZZGFREL_GET(save.START, &mut save.VALUE, ctx)?;

                testutil::CHCKSD(&save.QNAME, save.VALUE, b"<", 0.0, save.FTOL, OK, ctx)?;

                //
                // Check the derivative at the start time.
                //
                fstr::assign(&mut save.QNAME, b"Derivative at time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                T_ZZGFREL_SET(b"D((X**2)*SIN(X))", ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                T_ZZGFREL_GET(save.START, &mut save.VALUE, ctx)?;

                testutil::CHCKSD(&save.QNAME, save.VALUE, b"~", 0.0, save.FTOL, OK, ctx)?;

                //
                // Check the 2nd derivative at the start time.
                //
                fstr::assign(&mut save.QNAME, b"2nd derivative at time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                T_ZZGFREL_SET(b"D2((X**2)*SIN(X))", ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                T_ZZGFREL_GET(save.START, &mut save.VALUE, ctx)?;

                testutil::CHCKSD(&save.QNAME, save.VALUE, b">", 0.0, save.FTOL, OK, ctx)?;

                save.I += m3__;
            }
        }
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Find absolute minimum of (X**2)SIN(X) on [-10*pi,10*pi]",
        ctx,
    )?;

    save.STEPSZ = 0.25;
    spicelib::GFSSTP(save.STEPSZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(
        -((10 as f64) * spicelib::PI(ctx)),
        ((10 as f64) * spicelib::PI(ctx)),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.0;
    save.REFVAL = 0.0;
    fstr::assign(&mut save.RELATE, b"ABSMIN");
    //
    // We should be able to work with a very tight tolerance, since
    // our independent variable is close to zero.
    //
    save.TOL = 0.0000000000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    //
    // Set the function to (X**2)*SIN(X).
    //
    T_ZZGFREL_SET(b"(X**2)*SIN(X)", ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_ZZGFREL_UP(save.REFVAL, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We should find 1 absolute minimum.
    //
    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    //
    // Set the expected start time. Local minima occur between
    // -pi and 0 and in intervals offset from [-pi,0] by multiples
    // of 2*pi. Given the confinement interval we've picked,
    // the last local minimum should be the absolute
    // minimum.
    //
    save.XBEG[1] = save.CRITPT[20];

    //
    // Check the start time for the absolute maximum.
    //
    if *OK {
        spicelib::WNFETD(
            save.RESULT.as_slice(),
            1,
            &mut save.START,
            &mut save.FINISH,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check start time.
        //
        fstr::assign(&mut save.QNAME, b"Start time #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

        testutil::CHCKSD(
            &save.QNAME,
            save.START,
            b"~",
            save.XBEG[1],
            save.TOL,
            OK,
            ctx,
        )?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Find absolute minimum of (X**2)SIN(X) on [-10*pi,9*pi]",
        ctx,
    )?;

    save.STEPSZ = 0.25;
    spicelib::GFSSTP(save.STEPSZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(
        -((10 as f64) * spicelib::PI(ctx)),
        ((9 as f64) * spicelib::PI(ctx)),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.0;
    save.REFVAL = 0.0;
    fstr::assign(&mut save.RELATE, b"ABSMIN");
    //
    // We should be able to work with a very tight tolerance, since
    // our independent variable is close to zero.
    //
    save.TOL = 0.0000000000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    //
    // Set the function to (X**2)*SIN(X).
    //
    T_ZZGFREL_SET(b"(X**2)*SIN(X)", ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_ZZGFREL_UP(save.REFVAL, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We should find 1 absolute minimum.
    //
    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    //
    // Set the expected start time. Local minima occur between
    // -pi and 0 and in intervals offset from [-pi,0] by multiples
    // of 2*pi. Given the confinement interval we've picked,
    // the first local minimum should be the absolute
    // minimum.
    //
    save.XBEG[1] = save.CRITPT[2];

    //
    // Check the start time for the absolute minimum.
    //
    if *OK {
        spicelib::WNFETD(
            save.RESULT.as_slice(),
            1,
            &mut save.START,
            &mut save.FINISH,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check start time.
        //
        fstr::assign(&mut save.QNAME, b"Start time #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

        testutil::CHCKSD(
            &save.QNAME,
            save.START,
            b"~",
            save.XBEG[1],
            save.TOL,
            OK,
            ctx,
        )?;
    }

    //*********************************************************************
    //*
    //*    Normal cases with multiple-interval confinement windows
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Solve SIN(X) == 0.5 on { [0,pi/6], [pi/3,pi/2], [2pi/3,5pi/6] }",
        ctx,
    )?;

    //
    // This case requires an actual search, so set the step size.
    //
    save.STEPSZ = 0.25;
    spicelib::GFSSTP(save.STEPSZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(
        0.0,
        (spicelib::PI(ctx) / 6 as f64),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(
        (spicelib::PI(ctx) / 3 as f64),
        (spicelib::PI(ctx) / 2 as f64),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(
        (((2 as f64) * spicelib::PI(ctx)) / 3 as f64),
        (((5 as f64) * spicelib::PI(ctx)) / 6 as f64),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Expand the confinement window slightly to compensate for
    // round-off errors.
    //
    save.E = 0.000000000001;

    spicelib::WNEXPD(save.E, save.E, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.0;
    save.REFVAL = 0.5;
    fstr::assign(&mut save.RELATE, b"=");

    //
    // We should be able to work with a very tight tolerance, since
    // our independent variable is close to zero.
    //
    save.TOL = 0.0000000000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    T_ZZGFREL_SET(b"SIN(X)", ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_ZZGFREL_UP(save.REFVAL, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect two roots.
    //
    testutil::CHCKSI(b"N", save.N, b"=", 2, 0, OK, ctx)?;

    //
    // Check the times of and function values at the roots.
    //
    save.XTIME[1] = (spicelib::PI(ctx) / 6 as f64);
    save.XTIME[2] = (((5 as f64) * spicelib::PI(ctx)) / 6 as f64);

    save.XVAL[1] = 0.5;
    save.XVAL[2] = 0.5;

    if *OK {
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

                //
                // Check start time.
                //
                fstr::assign(&mut save.QNAME, b"Start time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                testutil::CHCKSD(
                    &save.QNAME,
                    save.START,
                    b"~",
                    save.XTIME[save.I],
                    save.TOL,
                    OK,
                    ctx,
                )?;

                //
                // Make sure stop time exactly matches start time.
                //
                fstr::assign(&mut save.QNAME, b"Stop time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                testutil::CHCKSD(&save.QNAME, save.FINISH, b"=", save.START, 0.0, OK, ctx)?;

                //
                // Check function value at start time. The function
                // has derivative magnitude not exceeding 1, so the
                // function tolerance can be set equal to the time
                // tolerance.
                //
                fstr::assign(&mut save.QNAME, b"Start value #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                T_ZZGFREL_GET(save.START, &mut save.VALUE, ctx)?;

                save.FTOL = save.TOL;

                testutil::CHCKSD(
                    &save.QNAME,
                    save.VALUE,
                    b"~",
                    save.XVAL[save.I],
                    save.FTOL,
                    OK,
                    ctx,
                )?;

                save.I += m3__;
            }
        }
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Solve SIN(X) < 0.5 on { [-pi/2,-pi/4], [0,pi/6], [pi/3,2*pi/3], [5pi/6, pi] }",
        ctx,
    )?;

    //
    // This case requires an actual search, so set the step size.
    //
    save.STEPSZ = 0.25;
    spicelib::GFSSTP(save.STEPSZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(
        -(spicelib::PI(ctx) / 2 as f64),
        -(spicelib::PI(ctx) / 4 as f64),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(
        0.0,
        (spicelib::PI(ctx) / 6 as f64),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(
        (spicelib::PI(ctx) / 3 as f64),
        (((3 as f64) * spicelib::PI(ctx)) / 3 as f64),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(
        (((5 as f64) * spicelib::PI(ctx)) / 6 as f64),
        spicelib::PI(ctx),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.0;
    save.REFVAL = 0.5;
    fstr::assign(&mut save.RELATE, b"<");
    //
    // We should be able to work with a very tight tolerance, since
    // our independent variable is close to zero.
    //
    save.TOL = 0.0000000000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    T_ZZGFREL_SET(b"SIN(X)", ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_ZZGFREL_UP(save.REFVAL, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect three solution intervals.
    //
    testutil::CHCKSI(b"N", save.N, b"=", 3, 0, OK, ctx)?;

    //
    // Check the times of and function values at the roots.
    //
    save.XBEG[1] = -(spicelib::PI(ctx) / 2 as f64);
    save.XEND[1] = -(spicelib::PI(ctx) / 4 as f64);
    save.XBEG[2] = 0.0;
    save.XEND[2] = (spicelib::PI(ctx) / 6 as f64);
    save.XBEG[3] = (((5 as f64) * spicelib::PI(ctx)) / 6 as f64);
    save.XEND[3] = spicelib::PI(ctx);

    save.XBEGVL[1] = -1.0;
    save.XENDVL[1] = -(f64::sqrt(2.0) / 2 as f64);
    save.XBEGVL[2] = 0.0;
    save.XENDVL[2] = 0.5;
    save.XBEGVL[3] = 0.5;
    save.XENDVL[3] = 0.0;

    if *OK {
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
                //
                // Check start time.
                //
                fstr::assign(&mut save.QNAME, b"Start time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                testutil::CHCKSD(
                    &save.QNAME,
                    save.START,
                    b"~",
                    save.XBEG[save.I],
                    save.TOL,
                    OK,
                    ctx,
                )?;

                //
                // Check function value at start time. The function
                // has derivative magnitude not exceeding 1, so the
                // function tolerance can be set equal to the time
                // tolerance.
                //
                fstr::assign(&mut save.QNAME, b"Start value #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                T_ZZGFREL_GET(save.START, &mut save.VALUE, ctx)?;

                save.FTOL = save.TOL;

                testutil::CHCKSD(
                    &save.QNAME,
                    save.VALUE,
                    b"~",
                    save.XBEGVL[save.I],
                    save.FTOL,
                    OK,
                    ctx,
                )?;

                //
                // Check stop time and value.
                //
                fstr::assign(&mut save.QNAME, b"Stop time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                testutil::CHCKSD(
                    &save.QNAME,
                    save.FINISH,
                    b"~",
                    save.XEND[save.I],
                    save.TOL,
                    OK,
                    ctx,
                )?;

                fstr::assign(&mut save.QNAME, b"Stop value #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                T_ZZGFREL_GET(save.FINISH, &mut save.VALUE, ctx)?;

                testutil::CHCKSD(
                    &save.QNAME,
                    save.VALUE,
                    b"~",
                    save.XENDVL[save.I],
                    save.FTOL,
                    OK,
                    ctx,
                )?;

                save.I += m3__;
            }
        }
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Solve SIN(X) > 0.5 on { [-pi/2,0], [pi/6, pi/3], [2*pi/3, 5*pi/6], [pi, 7*pi/6 }",
        ctx,
    )?;

    //
    // This case requires an actual search, so set the step size.
    //
    save.STEPSZ = 0.25;
    spicelib::GFSSTP(save.STEPSZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(
        -(spicelib::PI(ctx) / 2 as f64),
        0.0,
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(
        (spicelib::PI(ctx) / 6 as f64),
        (spicelib::PI(ctx) / 3 as f64),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(
        (((2 as f64) * spicelib::PI(ctx)) / 3 as f64),
        (((5 as f64) * spicelib::PI(ctx)) / 6 as f64),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(
        spicelib::PI(ctx),
        (((7 as f64) * spicelib::PI(ctx)) / 6 as f64),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.0;
    save.REFVAL = 0.5;
    fstr::assign(&mut save.RELATE, b">");
    //
    // We should be able to work with a very tight tolerance, since
    // our independent variable is close to zero.
    //
    save.TOL = 0.0000000000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    T_ZZGFREL_SET(b"SIN(X)", ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_ZZGFREL_UP(save.REFVAL, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect two solution intervals.
    //
    testutil::CHCKSI(b"N", save.N, b"=", 2, 0, OK, ctx)?;

    //
    // Check the times of and function values at the roots.
    //
    save.XBEG[1] = (spicelib::PI(ctx) / 6 as f64);
    save.XEND[1] = (spicelib::PI(ctx) / 3 as f64);
    save.XBEG[2] = (((2 as f64) * spicelib::PI(ctx)) / 3 as f64);
    save.XEND[2] = (((5 as f64) * spicelib::PI(ctx)) / 6 as f64);

    save.XBEGVL[1] = 0.5;
    save.XENDVL[1] = (f64::sqrt(3.0) / 2 as f64);
    save.XBEGVL[2] = (f64::sqrt(3.0) / 2 as f64);
    save.XENDVL[2] = 0.5;

    if *OK {
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
                //
                // Check start time.
                //
                fstr::assign(&mut save.QNAME, b"Start time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                testutil::CHCKSD(
                    &save.QNAME,
                    save.START,
                    b"~",
                    save.XBEG[save.I],
                    save.TOL,
                    OK,
                    ctx,
                )?;

                //
                // Check function value at start time. The function
                // has derivative magnitude not exceeding 1, so the
                // function tolerance can be set equal to the time
                // tolerance.
                //
                fstr::assign(&mut save.QNAME, b"Start value #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                T_ZZGFREL_GET(save.START, &mut save.VALUE, ctx)?;

                save.FTOL = save.TOL;

                testutil::CHCKSD(
                    &save.QNAME,
                    save.VALUE,
                    b"~",
                    save.XBEGVL[save.I],
                    save.FTOL,
                    OK,
                    ctx,
                )?;

                //
                // Check stop time and value.
                //
                fstr::assign(&mut save.QNAME, b"Stop time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                testutil::CHCKSD(
                    &save.QNAME,
                    save.FINISH,
                    b"~",
                    save.XEND[save.I],
                    save.TOL,
                    OK,
                    ctx,
                )?;

                fstr::assign(&mut save.QNAME, b"Stop value #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                T_ZZGFREL_GET(save.FINISH, &mut save.VALUE, ctx)?;

                testutil::CHCKSD(
                    &save.QNAME,
                    save.VALUE,
                    b"~",
                    save.XENDVL[save.I],
                    save.FTOL,
                    OK,
                    ctx,
                )?;

                save.I += m3__;
            }
        }
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Find local maxima of (X**2)SIN(X) on set of intervals bracketing critical points",
        ctx,
    )?;

    save.STEPSZ = 0.25;
    spicelib::GFSSTP(save.STEPSZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.DELTA = 0.25;

    //
    // This confinement window contains 20 intervals. Make a check
    // here to guard against "code upgrades."
    //
    testutil::CHCKSI(b"NCRIT", save.NCRIT, b"=", 20, 0, OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCRIT;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::WNINSD(
                (save.CRITPT[save.I] - save.DELTA),
                (save.CRITPT[save.I] + save.DELTA),
                save.CNFINE.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.0;
    save.REFVAL = 0.0;
    fstr::assign(&mut save.RELATE, b"LOCMAX");
    //
    // We should be able to work with a very tight tolerance, since
    // our independent variable is close to zero.
    //
    save.TOL = 0.0000000000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    //
    // Set the function to (X**2)*SIN(X).
    //
    T_ZZGFREL_SET(b"(X**2)*SIN(X)", ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_ZZGFREL_UP(save.REFVAL, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We should find 10 local maxima.
    //
    testutil::CHCKSI(b"N", save.N, b"=", 10, 0, OK, ctx)?;

    //
    // Set the expected start times. Local maxima occur between
    // 0 and pi and in intervals offset from [0,pi] by multiples
    // of 2*pi.
    //
    save.J = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 20;
        let m3__: i32 = 2;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.XBEG[save.J] = save.CRITPT[save.I];
            save.J = (save.J + 1);

            save.I += m3__;
        }
    }

    //
    // Check the start times for the local maxima.
    //
    if *OK {
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

                //
                // Check start time.
                //
                fstr::assign(&mut save.QNAME, b"Start time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                testutil::CHCKSD(
                    &save.QNAME,
                    save.START,
                    b"~",
                    save.XBEG[save.I],
                    save.TOL,
                    OK,
                    ctx,
                )?;

                save.I += m3__;
            }
        }
    }

    //
    // Check the 1st and 2nd derivative at each solution epoch.
    //
    save.FTOL = 0.0000000001;

    if *OK {
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

                //
                // Check the function at the start time; the function
                // should be positive.
                //
                fstr::assign(&mut save.QNAME, b"Function at time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                T_ZZGFREL_SET(b"(X**2)*SIN(X)", ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                T_ZZGFREL_GET(save.START, &mut save.VALUE, ctx)?;

                testutil::CHCKSD(&save.QNAME, save.VALUE, b">", 0.0, save.FTOL, OK, ctx)?;

                //
                // Check the derivative at the start time.
                //
                fstr::assign(&mut save.QNAME, b"Derivative at time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                T_ZZGFREL_SET(b"D((X**2)*SIN(X))", ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                T_ZZGFREL_GET(save.START, &mut save.VALUE, ctx)?;

                testutil::CHCKSD(&save.QNAME, save.VALUE, b"~", 0.0, save.FTOL, OK, ctx)?;

                //
                // Check the 2nd derivative at the start time.
                //
                fstr::assign(&mut save.QNAME, b"2nd derivative at time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                T_ZZGFREL_SET(b"D2((X**2)*SIN(X))", ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                T_ZZGFREL_GET(save.START, &mut save.VALUE, ctx)?;

                testutil::CHCKSD(&save.QNAME, save.VALUE, b"<", 0.0, save.FTOL, OK, ctx)?;

                save.I += m3__;
            }
        }
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Find local minima of (X**2)SIN(X) on set of intervals bracketing critical points",
        ctx,
    )?;

    save.STEPSZ = 0.25;
    spicelib::GFSSTP(save.STEPSZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.DELTA = 0.25;

    //
    // This confinement window contains 20 intervals. Make a check
    // here to guard against "code upgrades."
    //
    testutil::CHCKSI(b"NCRIT", save.NCRIT, b"=", 20, 0, OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCRIT;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::WNINSD(
                (save.CRITPT[save.I] - save.DELTA),
                (save.CRITPT[save.I] + save.DELTA),
                save.CNFINE.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.0;
    save.REFVAL = 0.0;
    fstr::assign(&mut save.RELATE, b"LOCMIN");
    //
    // We should be able to work with a very tight tolerance, since
    // our independent variable is close to zero.
    //
    save.TOL = 0.0000000000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    //
    // Set the function to (X**2)*SIN(X).
    //
    T_ZZGFREL_SET(b"(X**2)*SIN(X)", ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_ZZGFREL_UP(save.REFVAL, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We should find 10 local minima.
    //
    testutil::CHCKSI(b"N", save.N, b"=", 10, 0, OK, ctx)?;

    //
    // Set the expected start times. Local minima occur between
    // -pi and 0 and in intervals offset from [-pi,0] by multiples
    // of 2*pi.
    //
    save.J = 1;

    {
        let m1__: i32 = 2;
        let m2__: i32 = 20;
        let m3__: i32 = 2;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.XBEG[save.J] = save.CRITPT[save.I];
            save.J = (save.J + 1);

            save.I += m3__;
        }
    }

    //
    // Check the start times for the local minima.
    //
    if *OK {
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

                //
                // Check start time.
                //
                fstr::assign(&mut save.QNAME, b"Start time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                testutil::CHCKSD(
                    &save.QNAME,
                    save.START,
                    b"~",
                    save.XBEG[save.I],
                    save.TOL,
                    OK,
                    ctx,
                )?;

                save.I += m3__;
            }
        }
    }

    //
    // Check the 1st and 2nd derivative at each solution epoch.
    //
    save.FTOL = 0.0000000001;

    if *OK {
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

                //
                // Check the function at the start time; the function
                // should be negative.
                //
                fstr::assign(&mut save.QNAME, b"Function at time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                T_ZZGFREL_SET(b"(X**2)*SIN(X)", ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                T_ZZGFREL_GET(save.START, &mut save.VALUE, ctx)?;

                testutil::CHCKSD(&save.QNAME, save.VALUE, b"<", 0.0, save.FTOL, OK, ctx)?;

                //
                // Check the derivative at the start time.
                //
                fstr::assign(&mut save.QNAME, b"Derivative at time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                T_ZZGFREL_SET(b"D((X**2)*SIN(X))", ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                T_ZZGFREL_GET(save.START, &mut save.VALUE, ctx)?;

                testutil::CHCKSD(&save.QNAME, save.VALUE, b"~", 0.0, save.FTOL, OK, ctx)?;

                //
                // Check the 2nd derivative at the start time.
                //
                fstr::assign(&mut save.QNAME, b"2nd derivative at time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                T_ZZGFREL_SET(b"D2((X**2)*SIN(X))", ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                T_ZZGFREL_GET(save.START, &mut save.VALUE, ctx)?;

                testutil::CHCKSD(&save.QNAME, save.VALUE, b">", 0.0, save.FTOL, OK, ctx)?;

                save.I += m3__;
            }
        }
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Find local maxima of (X**2)SIN(X) on set of intervals excluding critical points",
        ctx,
    )?;

    save.STEPSZ = 0.25;
    spicelib::GFSSTP(save.STEPSZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.DELTA = 0.25;

    //
    // This confinement window contains 20 intervals. Make a check
    // here to guard against "code upgrades."
    //
    testutil::CHCKSI(b"NCRIT", save.NCRIT, b"=", 20, 0, OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCRIT;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::WNINSD(
                (save.CRITPT[save.I] + save.DELTA),
                (save.CRITPT[save.I] + ((2 as f64) * save.DELTA)),
                save.CNFINE.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.0;
    save.REFVAL = 0.0;
    fstr::assign(&mut save.RELATE, b"LOCMAX");
    //
    // We should be able to work with a very tight tolerance, since
    // our independent variable is close to zero.
    //
    save.TOL = 0.0000000000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    //
    // Set the function to (X**2)*SIN(X).
    //
    T_ZZGFREL_SET(b"(X**2)*SIN(X)", ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_ZZGFREL_UP(save.REFVAL, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We should find NO local maxima.
    //
    testutil::CHCKSI(b"N", save.N, b"=", 0, 0, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Find local minima of (X**2)SIN(X) on set of intervals excluding critical points",
        ctx,
    )?;

    save.STEPSZ = 0.25;
    spicelib::GFSSTP(save.STEPSZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.DELTA = 0.25;

    //
    // This confinement window contains 20 intervals. Make a check
    // here to guard against "code upgrades."
    //
    testutil::CHCKSI(b"NCRIT", save.NCRIT, b"=", 20, 0, OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCRIT;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::WNINSD(
                (save.CRITPT[save.I] + save.DELTA),
                (save.CRITPT[save.I] + ((2 as f64) * save.DELTA)),
                save.CNFINE.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.0;
    save.REFVAL = 0.0;
    fstr::assign(&mut save.RELATE, b"LOCMIN");

    //
    // We should be able to work with a very tight tolerance, since
    // our independent variable is close to zero.
    //
    save.TOL = 0.0000000000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    //
    // Set the function to (X**2)*SIN(X).
    //
    T_ZZGFREL_SET(b"(X**2)*SIN(X)", ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_ZZGFREL_UP(save.REFVAL, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We should find NO local minima.
    //
    testutil::CHCKSI(b"N", save.N, b"=", 0, 0, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Find absolute maximum of (X**2)SIN(X) on { [-10*pi,-9*pi], [-8pi,-7pi],...,[8pi,9pi] }",
        ctx,
    )?;

    save.STEPSZ = 0.25;
    spicelib::GFSSTP(save.STEPSZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = -10;
        let m2__: i32 = 8;
        let m3__: i32 = 2;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::WNINSD(
                ((save.I as f64) * spicelib::PI(ctx)),
                (((save.I + 1) as f64) * spicelib::PI(ctx)),
                save.CNFINE.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            save.I += m3__;
        }
    }

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.0;
    save.REFVAL = 0.0;
    fstr::assign(&mut save.RELATE, b"ABSMAX");
    //
    // We should be able to work with a very tight tolerance, since
    // our independent variable is close to zero.
    //
    save.TOL = 0.0000000000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    //
    // Set the function to (X**2)*SIN(X).
    //
    T_ZZGFREL_SET(b"(X**2)*SIN(X)", ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_ZZGFREL_UP(save.REFVAL, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We should find 1 absolute maximum.
    //
    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    //
    // Set the expected start time. Local maxima occur between
    // 0 and pi and in intervals offset from [0,pi] by multiples
    // of 2*pi. Given the confinement interval we've picked,
    // the first local maximum should be the absolute
    // maximum.
    //
    save.XBEG[1] = save.CRITPT[1];

    //
    // Check the start time for the absolute maximum.
    //
    if *OK {
        spicelib::WNFETD(
            save.RESULT.as_slice(),
            1,
            &mut save.START,
            &mut save.FINISH,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check start time.
        //
        fstr::assign(&mut save.QNAME, b"Start time #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

        testutil::CHCKSD(
            &save.QNAME,
            save.START,
            b"~",
            save.XBEG[1],
            save.TOL,
            OK,
            ctx,
        )?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Find absolute maximum of (X**2)SIN(X) on { [-8*pi,-7*pi], [-6pi,-5pi],...,[8pi,9pi] }",
        ctx,
    )?;

    save.STEPSZ = 0.25;
    spicelib::GFSSTP(save.STEPSZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = -8;
        let m2__: i32 = 8;
        let m3__: i32 = 2;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::WNINSD(
                ((save.I as f64) * spicelib::PI(ctx)),
                (((save.I + 1) as f64) * spicelib::PI(ctx)),
                save.CNFINE.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            save.I += m3__;
        }
    }

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.0;
    save.REFVAL = 0.0;
    fstr::assign(&mut save.RELATE, b"ABSMAX");
    //
    // We should be able to work with a very tight tolerance, since
    // our independent variable is close to zero.
    //
    save.TOL = 0.0000000000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    //
    // Set the function to (X**2)*SIN(X).
    //
    T_ZZGFREL_SET(b"(X**2)*SIN(X)", ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_ZZGFREL_UP(save.REFVAL, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We should find 1 absolute maximum.
    //
    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    //
    // Set the expected start time. Local maxima occur between
    // 0 and pi and in intervals offset from [0,pi] by multiples
    // of 2*pi. Given the confinement interval we've picked,
    // the last local maximum should be the absolute
    // maximum.
    //
    save.XBEG[1] = save.CRITPT[19];

    //
    // Check the start time for the absolute maximum.
    //
    if *OK {
        spicelib::WNFETD(
            save.RESULT.as_slice(),
            1,
            &mut save.START,
            &mut save.FINISH,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check start time.
        //
        fstr::assign(&mut save.QNAME, b"Start time #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

        testutil::CHCKSD(
            &save.QNAME,
            save.START,
            b"~",
            save.XBEG[1],
            save.TOL,
            OK,
            ctx,
        )?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Find absolute minimum of (X**2)SIN(X) on { [-9*pi,-8*pi], [-8pi,-7pi],...,[9pi,10pi] }",
        ctx,
    )?;

    save.STEPSZ = 0.25;
    spicelib::GFSSTP(save.STEPSZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = -9;
        let m2__: i32 = 9;
        let m3__: i32 = 2;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::WNINSD(
                ((save.I as f64) * spicelib::PI(ctx)),
                (((save.I + 1) as f64) * spicelib::PI(ctx)),
                save.CNFINE.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            save.I += m3__;
        }
    }

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.0;
    save.REFVAL = 0.0;
    fstr::assign(&mut save.RELATE, b"ABSMIN");
    //
    // We should be able to work with a very tight tolerance, since
    // our independent variable is close to zero.
    //
    save.TOL = 0.0000000000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    //
    // Set the function to (X**2)*SIN(X).
    //
    T_ZZGFREL_SET(b"(X**2)*SIN(X)", ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_ZZGFREL_UP(save.REFVAL, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We should find 1 absolute minimum.
    //
    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    //
    // Set the expected start time. Local minima occur between
    // -pi and 0 and in intervals offset from [-pi,0] by multiples
    // of 2*pi. Given the confinement interval we've picked,
    // the last local minimum should be the absolute
    // minimum.
    //
    save.XBEG[1] = save.CRITPT[20];

    //
    // Check the start time for the absolute maximum.
    //
    if *OK {
        spicelib::WNFETD(
            save.RESULT.as_slice(),
            1,
            &mut save.START,
            &mut save.FINISH,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check start time.
        //
        fstr::assign(&mut save.QNAME, b"Start time #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

        testutil::CHCKSD(
            &save.QNAME,
            save.START,
            b"~",
            save.XBEG[1],
            save.TOL,
            OK,
            ctx,
        )?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Find absolute minimum of (X**2)SIN(X) on { [-9*pi,-8*pi], [-8pi,-7pi],...,[7pi,8pi] }",
        ctx,
    )?;

    save.STEPSZ = 0.25;
    spicelib::GFSSTP(save.STEPSZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = -9;
        let m2__: i32 = 7;
        let m3__: i32 = 2;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::WNINSD(
                ((save.I as f64) * spicelib::PI(ctx)),
                (((save.I + 1) as f64) * spicelib::PI(ctx)),
                save.CNFINE.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            save.I += m3__;
        }
    }

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.0;
    save.REFVAL = 0.0;
    fstr::assign(&mut save.RELATE, b"ABSMIN");
    //
    // We should be able to work with a very tight tolerance, since
    // our independent variable is close to zero.
    //
    save.TOL = 0.0000000000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    //
    // Set the function to (X**2)*SIN(X).
    //
    T_ZZGFREL_SET(b"(X**2)*SIN(X)", ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_ZZGFREL_UP(save.REFVAL, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We should find 1 absolute minimum.
    //
    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    //
    // Set the expected start time. Local minima occur between
    // -pi and 0 and in intervals offset from [-pi,0] by multiples
    // of 2*pi. Given the confinement interval we've picked,
    // the first local minimum should be the absolute
    // minimum.
    //
    save.XBEG[1] = save.CRITPT[2];

    //
    // Check the start time for the absolute minimum.
    //
    if *OK {
        spicelib::WNFETD(
            save.RESULT.as_slice(),
            1,
            &mut save.START,
            &mut save.FINISH,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check start time.
        //
        fstr::assign(&mut save.QNAME, b"Start time #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

        testutil::CHCKSD(
            &save.QNAME,
            save.START,
            b"~",
            save.XBEG[1],
            save.TOL,
            OK,
            ctx,
        )?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Find adjusted absolute maxima of SIN(X) on { [-9*pi,-8*pi], [-7pi,-6pi],...,[7pi,8pi] }",
        ctx,
    )?;

    save.STEPSZ = 0.25;
    spicelib::GFSSTP(save.STEPSZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = -9;
        let m2__: i32 = 7;
        let m3__: i32 = 2;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::WNINSD(
                ((save.I as f64) * spicelib::PI(ctx)),
                (((save.I + 1) as f64) * spicelib::PI(ctx)),
                save.CNFINE.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            save.I += m3__;
        }
    }

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.5;
    save.REFVAL = 0.0;
    fstr::assign(&mut save.RELATE, b"ABSMAX");
    //
    // We should be able to work with a very tight tolerance, since
    // our independent variable is close to zero.
    //
    save.TOL = 0.0000000000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    T_ZZGFREL_SET(b"SIN(X)", ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_ZZGFREL_UP(save.REFVAL, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect 18 solutions.
    //
    testutil::CHCKSI(b"N", save.N, b"=", 18, 0, OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 2;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.XBEG[save.I] =
                ((((save.I - 1) as f64) * spicelib::PI(ctx)) - ((9 as f64) * spicelib::PI(ctx)));
            save.XEND[save.I] = (((((save.I - 1) as f64) * spicelib::PI(ctx))
                - ((9 as f64) * spicelib::PI(ctx)))
                + (spicelib::PI(ctx) / 6 as f64));

            save.XBEGVL[save.I] = 0.0;
            save.XENDVL[save.I] = -0.5;

            save.XBEG[(save.I + 1)] = (((((save.I - 1) as f64) * spicelib::PI(ctx))
                - ((9 as f64) * spicelib::PI(ctx)))
                + (((5 as f64) * spicelib::PI(ctx)) / 6 as f64));
            save.XEND[(save.I + 1)] =
                ((((save.I - 1) as f64) * spicelib::PI(ctx)) - ((8 as f64) * spicelib::PI(ctx)));

            save.XBEGVL[(save.I + 1)] = -0.5;
            save.XENDVL[(save.I + 1)] = 0.0;

            save.I += m3__;
        }
    }

    //
    // Check the times of and function values at the roots.
    //
    if *OK {
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
                //
                // Check start time.
                //
                fstr::assign(&mut save.QNAME, b"Start time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                testutil::CHCKSD(
                    &save.QNAME,
                    save.START,
                    b"~",
                    save.XBEG[save.I],
                    save.TOL,
                    OK,
                    ctx,
                )?;

                //
                // Check function value at start time. The function
                // has derivative magnitude not exceeding 1, so the
                // function tolerance can be set equal to the time
                // tolerance.
                //
                fstr::assign(&mut save.QNAME, b"Start value #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                T_ZZGFREL_GET(save.START, &mut save.VALUE, ctx)?;

                save.FTOL = save.TOL;

                testutil::CHCKSD(
                    &save.QNAME,
                    save.VALUE,
                    b"~",
                    save.XBEGVL[save.I],
                    save.FTOL,
                    OK,
                    ctx,
                )?;

                //
                // Check stop time.
                //
                fstr::assign(&mut save.QNAME, b"Stop time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                testutil::CHCKSD(
                    &save.QNAME,
                    save.FINISH,
                    b"~",
                    save.XEND[save.I],
                    save.TOL,
                    OK,
                    ctx,
                )?;

                //
                // Check function value at stop time. The function
                // has derivative magnitude not exceeding 1, so the
                // function tolerance can be set equal to the time
                // tolerance.
                //
                fstr::assign(&mut save.QNAME, b"Stop value #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                T_ZZGFREL_GET(save.FINISH, &mut save.VALUE, ctx)?;

                save.FTOL = save.TOL;

                testutil::CHCKSD(
                    &save.QNAME,
                    save.VALUE,
                    b"~",
                    save.XENDVL[save.I],
                    save.FTOL,
                    OK,
                    ctx,
                )?;

                save.I += m3__;
            }
        }
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Find adjusted absolute minima of SIN(X) on { [-9*pi,-8*pi], [-7pi,-6pi],...,[7pi,8pi] }",
        ctx,
    )?;

    save.STEPSZ = 0.25;
    spicelib::GFSSTP(save.STEPSZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = -9;
        let m2__: i32 = 7;
        let m3__: i32 = 2;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::WNINSD(
                ((save.I as f64) * spicelib::PI(ctx)),
                (((save.I + 1) as f64) * spicelib::PI(ctx)),
                save.CNFINE.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            save.I += m3__;
        }
    }

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.5;
    save.REFVAL = 0.0;
    fstr::assign(&mut save.RELATE, b"ABSMIN");
    //
    // We should be able to work with a very tight tolerance, since
    // our independent variable is close to zero.
    //
    save.TOL = 0.0000000000001;

    save.RPT = false;
    save.BAIL = false;

    fstr::assign(save.SRCPRE.get_mut(1), b"Search prefix 1");
    fstr::assign(save.SRCPRE.get_mut(2), b"Search prefix 2");

    fstr::assign(save.SRCSUF.get_mut(1), b"Search suffix 1");
    fstr::assign(save.SRCSUF.get_mut(2), b"Search suffix 2");

    T_ZZGFREL_SET(b"SIN(X)", ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_ZZGFREL_UP(save.REFVAL, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;
    save.NW = NWMAX;

    spicelib::ZZGFREL(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        T_ZZGFREL_DEC,
        T_ZZGFREL_LT,
        T_ZZGFREL_GET,
        T_ZZGFREL_UP,
        &save.RELATE,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.SRCPRE.as_arg(),
        save.SRCSUF.as_arg(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect 9 solutions.
    //
    testutil::CHCKSI(b"N", save.N, b"=", 9, 0, OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.XBEG[save.I] = ((((((save.I - 1) * 2) as f64) * spicelib::PI(ctx))
                - ((9 as f64) * spicelib::PI(ctx)))
                + (spicelib::PI(ctx) / 6 as f64));
            save.XEND[save.I] = ((((((save.I - 1) * 2) as f64) * spicelib::PI(ctx))
                - ((9 as f64) * spicelib::PI(ctx)))
                + (((5 as f64) * spicelib::PI(ctx)) / 6 as f64));

            save.XBEGVL[save.I] = -0.5;
            save.XENDVL[save.I] = -0.5;

            save.I += m3__;
        }
    }

    //
    // Check the times of and function values at the roots.
    //
    if *OK {
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
                //
                // Check start time.
                //
                fstr::assign(&mut save.QNAME, b"Start time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                testutil::CHCKSD(
                    &save.QNAME,
                    save.START,
                    b"~",
                    save.XBEG[save.I],
                    save.TOL,
                    OK,
                    ctx,
                )?;

                //
                // Check function value at start time. The function
                // has derivative magnitude not exceeding 1, so the
                // function tolerance can be set equal to the time
                // tolerance.
                //
                fstr::assign(&mut save.QNAME, b"Start value #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                T_ZZGFREL_GET(save.START, &mut save.VALUE, ctx)?;

                save.FTOL = save.TOL;

                testutil::CHCKSD(
                    &save.QNAME,
                    save.VALUE,
                    b"~",
                    save.XBEGVL[save.I],
                    save.FTOL,
                    OK,
                    ctx,
                )?;

                //
                // Check stop time.
                //
                fstr::assign(&mut save.QNAME, b"Stop time #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                testutil::CHCKSD(
                    &save.QNAME,
                    save.FINISH,
                    b"~",
                    save.XEND[save.I],
                    save.TOL,
                    OK,
                    ctx,
                )?;

                //
                // Check function value at stop time. The function
                // has derivative magnitude not exceeding 1, so the
                // function tolerance can be set equal to the time
                // tolerance.
                //
                fstr::assign(&mut save.QNAME, b"Stop value #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.I, &mut save.QNAME, ctx);

                T_ZZGFREL_GET(save.FINISH, &mut save.VALUE, ctx)?;

                save.FTOL = save.TOL;

                testutil::CHCKSD(
                    &save.QNAME,
                    save.VALUE,
                    b"~",
                    save.XENDVL[save.I],
                    save.FTOL,
                    OK,
                    ctx,
                )?;

                save.I += m3__;
            }
        }
    }

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
