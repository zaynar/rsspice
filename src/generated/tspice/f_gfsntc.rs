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
const SPK: &[u8] = b"gfsntc.bsp";
const PCK: &[u8] = b"gfsntc.pck";
const SPK1: &[u8] = b"nat.bsp";
const PCK1: &[u8] = b"nat.pck";
const BDNMLN: i32 = 36;
const LNSIZE: i32 = 80;
const LBCELL: i32 = -5;
const MAXWIN: i32 = 20000;
const NCORR: i32 = 9;
const NDESC: i32 = 22;

struct SaveVars {
    ABCORR: Vec<u8>,
    CORR: ActualCharArray,
    TARGET: Vec<u8>,
    OBSRVR: Vec<u8>,
    CRDSYS: Vec<u8>,
    COORD: Vec<u8>,
    RELATE: Vec<u8>,
    METHOD: Vec<u8>,
    FIXREF: Vec<u8>,
    DREF: Vec<u8>,
    TITLE: Vec<u8>,
    TIME0: Vec<u8>,
    TIME1: Vec<u8>,
    STEP: f64,
    BEG: f64,
    END: f64,
    LEFT: f64,
    RIGHT: f64,
    REFVAL: f64,
    ET0: f64,
    ET1: f64,
    ADJUST: f64,
    CNFINE: ActualArray<f64>,
    RESULT: ActualArray<f64>,
    WORK: ActualArray2D<f64>,
    X: StackArray<f64, 3>,
    Y: StackArray<f64, 3>,
    Z: StackArray<f64, 3>,
    RAD: StackArray<f64, 3>,
    TIMBEG: StackArray<f64, 2>,
    TIMEND: StackArray<f64, 2>,
    COUNT: i32,
    HANDLE: i32,
    HAN1: i32,
    I: i32,
    DIM: i32,
    MDESC: ActualCharArray,
    MREFS: StackArray<f64, 22>,
    ITEMS: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ABCORR = vec![b' '; LNSIZE as usize];
        let mut CORR = ActualCharArray::new(LNSIZE, 1..=NCORR);
        let mut TARGET = vec![b' '; BDNMLN as usize];
        let mut OBSRVR = vec![b' '; BDNMLN as usize];
        let mut CRDSYS = vec![b' '; BDNMLN as usize];
        let mut COORD = vec![b' '; BDNMLN as usize];
        let mut RELATE = vec![b' '; BDNMLN as usize];
        let mut METHOD = vec![b' '; BDNMLN as usize];
        let mut FIXREF = vec![b' '; BDNMLN as usize];
        let mut DREF = vec![b' '; BDNMLN as usize];
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut TIME0 = vec![b' '; LNSIZE as usize];
        let mut TIME1 = vec![b' '; LNSIZE as usize];
        let mut STEP: f64 = 0.0;
        let mut BEG: f64 = 0.0;
        let mut END: f64 = 0.0;
        let mut LEFT: f64 = 0.0;
        let mut RIGHT: f64 = 0.0;
        let mut REFVAL: f64 = 0.0;
        let mut ET0: f64 = 0.0;
        let mut ET1: f64 = 0.0;
        let mut ADJUST: f64 = 0.0;
        let mut CNFINE = ActualArray::<f64>::new(LBCELL..=MAXWIN);
        let mut RESULT = ActualArray::<f64>::new(LBCELL..=MAXWIN);
        let mut WORK = ActualArray2D::<f64>::new(LBCELL..=MAXWIN, 1..=NWMAX);
        let mut X = StackArray::<f64, 3>::new(1..=3);
        let mut Y = StackArray::<f64, 3>::new(1..=3);
        let mut Z = StackArray::<f64, 3>::new(1..=3);
        let mut RAD = StackArray::<f64, 3>::new(1..=3);
        let mut TIMBEG = StackArray::<f64, 2>::new(1..=2);
        let mut TIMEND = StackArray::<f64, 2>::new(1..=2);
        let mut COUNT: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut HAN1: i32 = 0;
        let mut I: i32 = 0;
        let mut DIM: i32 = 0;
        let mut MDESC = ActualCharArray::new(LNSIZE, 1..=NDESC);
        let mut MREFS = StackArray::<f64, 22>::new(1..=NDESC);
        let mut ITEMS = ActualCharArray::new(BDNMLN, 1..=3);

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

            let mut clist = [Val::D(1.0), Val::D(0.0), Val::D(0.0)].into_iter();
            X.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(1.0), Val::D(0.0)].into_iter();
            Y.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            Z.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            ABCORR,
            CORR,
            TARGET,
            OBSRVR,
            CRDSYS,
            COORD,
            RELATE,
            METHOD,
            FIXREF,
            DREF,
            TITLE,
            TIME0,
            TIME1,
            STEP,
            BEG,
            END,
            LEFT,
            RIGHT,
            REFVAL,
            ET0,
            ET1,
            ADJUST,
            CNFINE,
            RESULT,
            WORK,
            X,
            Y,
            Z,
            RAD,
            TIMBEG,
            TIMEND,
            COUNT,
            HANDLE,
            HAN1,
            I,
            DIM,
            MDESC,
            MREFS,
            ITEMS,
        }
    }
}

//$Procedure F_GFSNTC ( GFSNTC family tests )
pub fn F_GFSNTC(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Saved all.
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_GFSNTC", ctx)?;

    testutil::TCASE(b"Setup: create and load SPK, PCK, LSK files.", ctx)?;

    // Leapseconds:  Note that the LSK is deleted after loading, so we
    // don't have to clean it up later.
    //
    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a PCK, load using FURNSH.
    //
    testutil::T_PCK08(PCK, false, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(PCK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::NATPCK(PCK1, false, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(PCK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create an SPK, load using FURNSH.
    //
    testutil::TSTSPK(SPK, true, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(SPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::NATSPK(SPK1, true, &mut save.HAN1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a confinement window from ET0 and ET1.
    //
    fstr::assign(&mut save.TIME0, b"2000 JAN 1  00:00:00 TDB");
    fstr::assign(&mut save.TIME1, b"2000 JAN 2  00:00:00 TDB");

    spicelib::STR2ET(&save.TIME0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&save.TIME1, &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Error cases
    //

    //
    // Case 1
    //
    testutil::TCASE(b"Non-positive step size", ctx)?;

    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.FIXREF, b"IAU_EARTH");
    fstr::assign(&mut save.METHOD, b"Ellipsoid");
    fstr::assign(&mut save.ABCORR, save.CORR.get(1));
    fstr::assign(&mut save.DREF, b"IAU_EARTH");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    fstr::assign(&mut save.CRDSYS, b"LATITUDINAL");
    fstr::assign(&mut save.COORD, b"LATITUDE");
    fstr::assign(&mut save.RELATE, b"=");
    save.REFVAL = 0.0;
    save.ADJUST = 0.0;
    save.STEP = 0.0;

    spicelib::GFSNTC(
        &save.TARGET,
        &save.FIXREF,
        &save.METHOD,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.X.as_slice(),
        &save.CRDSYS,
        &save.COORD,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWMAX,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDSTEP)", OK, ctx)?;

    //
    // Case 2
    //
    testutil::TCASE(b"Non unique body IDs.", ctx)?;

    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    save.STEP = 1.0;

    spicelib::GFSNTC(
        &save.TARGET,
        &save.FIXREF,
        &save.METHOD,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.X.as_slice(),
        &save.CRDSYS,
        &save.COORD,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWMAX,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    //
    // Case 3
    //
    testutil::TCASE(b"Invalid aberration correction specifier", ctx)?;

    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.ABCORR, b"X");
    fstr::assign(&mut save.OBSRVR, b"SUN");

    spicelib::GFSNTC(
        &save.TARGET,
        &save.FIXREF,
        &save.METHOD,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.X.as_slice(),
        &save.CRDSYS,
        &save.COORD,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWMAX,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // Case 4
    //
    testutil::TCASE(b"Invalid relations operator", ctx)?;

    fstr::assign(&mut save.ABCORR, save.CORR.get(1));
    fstr::assign(&mut save.RELATE, b"==");

    spicelib::GFSNTC(
        &save.TARGET,
        &save.FIXREF,
        &save.METHOD,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.X.as_slice(),
        &save.CRDSYS,
        &save.COORD,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWMAX,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOTRECOGNIZED)", OK, ctx)?;

    //
    // Case 5
    //
    testutil::TCASE(b"Invalid body names", ctx)?;

    fstr::assign(&mut save.RELATE, b"=");
    fstr::assign(&mut save.TARGET, b"X");
    fstr::assign(&mut save.OBSRVR, b"SUN");

    spicelib::GFSNTC(
        &save.TARGET,
        &save.FIXREF,
        &save.METHOD,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.X.as_slice(),
        &save.CRDSYS,
        &save.COORD,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWMAX,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"X");

    spicelib::GFSNTC(
        &save.TARGET,
        &save.FIXREF,
        &save.METHOD,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.X.as_slice(),
        &save.CRDSYS,
        &save.COORD,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWMAX,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // Case 6
    //
    testutil::TCASE(b"Negative adjustment value", ctx)?;

    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    save.ADJUST = -1.0;

    spicelib::GFSNTC(
        &save.TARGET,
        &save.FIXREF,
        &save.METHOD,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.X.as_slice(),
        &save.CRDSYS,
        &save.COORD,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWMAX,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // Case 7
    //
    testutil::TCASE(b"Ephemeris data unavailable", ctx)?;

    save.ADJUST = 0.0;
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"DAWN");

    spicelib::GFSNTC(
        &save.TARGET,
        &save.FIXREF,
        &save.METHOD,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.X.as_slice(),
        &save.CRDSYS,
        &save.COORD,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWMAX,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // Case 8
    //
    testutil::TCASE(b"Unknown coordinate system", ctx)?;

    save.ADJUST = 0.0;
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    fstr::assign(&mut save.CRDSYS, b"X");

    spicelib::GFSNTC(
        &save.TARGET,
        &save.FIXREF,
        &save.METHOD,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.X.as_slice(),
        &save.CRDSYS,
        &save.COORD,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWMAX,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // Case 9
    //
    testutil::TCASE(b"Unknown coordinate.", ctx)?;

    save.ADJUST = 0.0;
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.CRDSYS, b"LATITUDINAL");
    fstr::assign(&mut save.COORD, b"X");

    spicelib::GFSNTC(
        &save.TARGET,
        &save.FIXREF,
        &save.METHOD,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.X.as_slice(),
        &save.CRDSYS,
        &save.COORD,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWMAX,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // Case 10
    //

    {
        let m1__: i32 = 1;
        let m2__: i32 = 2;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            if (save.I == 1) {
                fstr::assign(&mut save.CRDSYS, b"PLANETOGRAPHIC");
            } else {
                fstr::assign(&mut save.CRDSYS, b"GEODETIC");
            }

            fstr::assign(&mut save.TITLE, b"Unknown FIXREF frame for #");
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CRDSYS, &mut save.TITLE);
            testutil::TCASE(&save.TITLE, ctx)?;

            fstr::assign(&mut save.COORD, b"ALTITUDE");
            fstr::assign(&mut save.FIXREF, b"X");

            spicelib::GFSNTC(
                &save.TARGET,
                &save.FIXREF,
                &save.METHOD,
                &save.ABCORR,
                &save.OBSRVR,
                &save.DREF,
                save.X.as_slice(),
                &save.CRDSYS,
                &save.COORD,
                &save.RELATE,
                save.REFVAL,
                save.ADJUST,
                save.STEP,
                save.CNFINE.as_slice(),
                MAXWIN,
                NWMAX,
                save.WORK.as_slice_mut(),
                save.RESULT.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(true, b"SPICE(NOFRAME)", OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Case 11
    //
    testutil::TCASE(b"Invalid target frame", ctx)?;

    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    fstr::assign(&mut save.CRDSYS, b"LATITUDINAL");
    fstr::assign(&mut save.COORD, b"LATITUDE");
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");

    spicelib::GFSNTC(
        &save.TARGET,
        &save.FIXREF,
        &save.METHOD,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.X.as_slice(),
        &save.CRDSYS,
        &save.COORD,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWMAX,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDFRAME)", OK, ctx)?;

    //
    // Define the coordinate test conditions.
    //
    spicelib::BODVRD(
        b"ALPHA",
        b"RADII",
        3,
        &mut save.DIM,
        save.RAD.as_slice_mut(),
        ctx,
    )?;

    //
    // Conditions: all conditions should occur once per
    // GAMMA orbit (delta_t = 24 hours).
    //
    fstr::assign(save.MDESC.get_mut(1), b"RECTANGULAR   : X           : <");
    fstr::assign(save.MDESC.get_mut(2), b"RECTANGULAR   : Y           : <");
    fstr::assign(save.MDESC.get_mut(3), b"RECTANGULAR   : Z           : <");
    fstr::assign(save.MDESC.get_mut(4), b"RECTANGULAR   : X           : >");
    fstr::assign(save.MDESC.get_mut(5), b"RECTANGULAR   : Y           : >");
    fstr::assign(save.MDESC.get_mut(6), b"RECTANGULAR   : Z           : >");
    fstr::assign(save.MDESC.get_mut(7), b"LATITUDINAL   : RADIUS      : >");
    fstr::assign(save.MDESC.get_mut(8), b"LATITUDINAL   : LATITUDE    : >");
    fstr::assign(save.MDESC.get_mut(9), b"LATITUDINAL   : LATITUDE    : <");
    fstr::assign(save.MDESC.get_mut(10), b"RA/DEC        : RANGE       : >");
    fstr::assign(save.MDESC.get_mut(11), b"RA/DEC        : DECLINATION : >");
    fstr::assign(save.MDESC.get_mut(12), b"RA/DEC        : DECLINATION : <");
    fstr::assign(save.MDESC.get_mut(13), b"SPHERICAL     : RADIUS      : >");
    fstr::assign(save.MDESC.get_mut(14), b"SPHERICAL     : COLATITUDE  : >");
    fstr::assign(save.MDESC.get_mut(15), b"SPHERICAL     : COLATITUDE  : <");
    fstr::assign(save.MDESC.get_mut(16), b"CYLINDRICAL   : RADIUS      : >");
    fstr::assign(save.MDESC.get_mut(17), b"CYLINDRICAL   : Z           : <");
    fstr::assign(save.MDESC.get_mut(18), b"CYLINDRICAL   : Z           : >");
    fstr::assign(save.MDESC.get_mut(19), b"CYLINDRICAL   : LONGITUDE   : >");
    fstr::assign(save.MDESC.get_mut(20), b"SPHERICAL     : LONGITUDE   : <");
    fstr::assign(save.MDESC.get_mut(21), b"LATITUDINAL   : LONGITUDE   : <");
    fstr::assign(
        save.MDESC.get_mut(22),
        b"RA/DEC        : RIGHT ASCENSION : >",
    );

    //
    // Test conditions reference values:
    //
    save.MREFS[1] = save.RAD[1];
    save.MREFS[2] = save.RAD[2];
    save.MREFS[3] = save.RAD[3];
    save.MREFS[4] = -save.RAD[1];
    save.MREFS[5] = -save.RAD[2];
    save.MREFS[6] = -save.RAD[3];
    save.MREFS[7] = 0.0;
    save.MREFS[8] = -(90.0 * spicelib::RPD(ctx));
    save.MREFS[9] = (90.0 * spicelib::RPD(ctx));
    save.MREFS[10] = 0.0;
    save.MREFS[11] = -(90.0 * spicelib::RPD(ctx));
    save.MREFS[12] = (90.0 * spicelib::RPD(ctx));
    save.MREFS[13] = 0.0;
    save.MREFS[14] = 0.0;
    save.MREFS[15] = (180.0 * spicelib::RPD(ctx));
    save.MREFS[16] = 0.0;
    save.MREFS[17] = save.RAD[3];
    save.MREFS[18] = -save.RAD[3];
    save.MREFS[19] = 0.0;
    save.MREFS[20] = 0.0;
    save.MREFS[21] = 0.0;
    save.MREFS[22] = 0.0;

    //
    // Time interval thirty days.
    //
    fstr::assign(&mut save.TIME0, b"2000 JAN 1   18:00:00 TDB");
    fstr::assign(&mut save.TIME1, b"2000 JAN 31  18:00:00 TDB");

    spicelib::STR2ET(&save.TIME0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&save.TIME1, &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.0;
    fstr::assign(&mut save.TARGET, b"ALPHA");
    fstr::assign(&mut save.FIXREF, b"ALPHAFIXED");
    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.OBSRVR, b"GAMMA");
    fstr::assign(&mut save.DREF, b"J2000");
    fstr::assign(&mut save.ABCORR, b"NONE");

    //
    // Case 12, Sweeps of the GAMMA centered J2000 Z axis on ALPHA.
    //

    save.STEP = (spicelib::SPD() * (1.25 / 24.0));

    {
        let m1__: i32 = 1;
        let m2__: i32 = NDESC;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;

            spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(
                &mut save.TITLE,
                &fstr::concat(
                    b"MDESC(#) ALPHA sweeps from GAMMA, Z, ",
                    save.MDESC.get(save.I),
                ),
            );
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);

            testutil::TCASE(&save.TITLE, ctx)?;

            //
            // Parse from the MDESC(I) string the coordinate system,
            // coordinate, and relation operator.
            //
            spicelib::LPARSE(
                &save.MDESC[save.I],
                b":",
                3,
                &mut save.DIM,
                save.ITEMS.as_arg_mut(),
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.CRDSYS, save.ITEMS.get(1));
            fstr::assign(&mut save.COORD, save.ITEMS.get(2));
            fstr::assign(&mut save.RELATE, save.ITEMS.get(3));
            save.REFVAL = save.MREFS[save.I];

            spicelib::GFSNTC(
                &save.TARGET,
                &save.FIXREF,
                &save.METHOD,
                &save.ABCORR,
                &save.OBSRVR,
                &save.DREF,
                save.Z.as_slice(),
                &save.CRDSYS,
                &save.COORD,
                &save.RELATE,
                save.REFVAL,
                save.ADJUST,
                save.STEP,
                save.CNFINE.as_slice(),
                MAXWIN,
                NWMAX,
                save.WORK.as_slice_mut(),
                save.RESULT.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSI(
                b"WNCARD(RESULT)",
                spicelib::WNCARD(save.RESULT.as_slice(), ctx)?,
                b"=",
                30,
                0,
                OK,
                ctx,
            )?;

            save.TIMBEG[1] = 0.0;
            save.TIMEND[1] = 0.0;
            save.TIMBEG[2] = 0.0;
            save.TIMEND[2] = 0.0;

            if *OK {
                spicelib::WNFETD(
                    save.RESULT.as_slice(),
                    1,
                    &mut save.TIMBEG[1],
                    &mut save.TIMEND[1],
                    ctx,
                )?;

                for J in 2..=spicelib::WNCARD(save.RESULT.as_slice(), ctx)? {
                    spicelib::WNFETD(
                        save.RESULT.as_slice(),
                        J,
                        &mut save.TIMBEG[2],
                        &mut save.TIMEND[2],
                        ctx,
                    )?;

                    testutil::CHCKSD(
                        b"Z SWEEP BEG",
                        (save.TIMBEG[2] - save.TIMBEG[1]),
                        b"~",
                        spicelib::SPD(),
                        CNVTOL,
                        OK,
                        ctx,
                    )?;

                    testutil::CHCKSD(
                        b"Z SWEEP END",
                        (save.TIMEND[2] - save.TIMEND[1]),
                        b"~",
                        spicelib::SPD(),
                        CNVTOL,
                        OK,
                        ctx,
                    )?;

                    save.TIMBEG[1] = save.TIMBEG[2];
                    save.TIMEND[1] = save.TIMEND[2];
                }
            }

            save.I += m3__;
        }
    }

    //
    // Case 13,  Sweeps of the GAMMA centered J2000 Y axis on ALPHA.
    //
    save.STEP = (spicelib::SPD() * (3.0 / 24.0));

    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NDESC;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(
                &mut save.TITLE,
                &fstr::concat(
                    b"MDESC(#) ALPHA sweeps from GAMMA, Y, ",
                    save.MDESC.get(save.I),
                ),
            );
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);

            testutil::TCASE(&save.TITLE, ctx)?;

            //
            // Parse from the MDESC(I) string the coordinate system,
            // coordinate, and relation operator.
            //
            spicelib::LPARSE(
                &save.MDESC[save.I],
                b":",
                3,
                &mut save.DIM,
                save.ITEMS.as_arg_mut(),
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.CRDSYS, save.ITEMS.get(1));
            fstr::assign(&mut save.COORD, save.ITEMS.get(2));
            fstr::assign(&mut save.RELATE, save.ITEMS.get(3));
            save.REFVAL = save.MREFS[save.I];

            spicelib::GFSNTC(
                &save.TARGET,
                &save.FIXREF,
                &save.METHOD,
                &save.ABCORR,
                &save.OBSRVR,
                &save.DREF,
                save.Y.as_slice(),
                &save.CRDSYS,
                &save.COORD,
                &save.RELATE,
                save.REFVAL,
                save.ADJUST,
                save.STEP,
                save.CNFINE.as_slice(),
                MAXWIN,
                NWMAX,
                save.WORK.as_slice_mut(),
                save.RESULT.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSI(
                b"WNCARD(RESULT)",
                spicelib::WNCARD(save.RESULT.as_slice(), ctx)?,
                b"=",
                30,
                0,
                OK,
                ctx,
            )?;

            save.TIMBEG[1] = 0.0;
            save.TIMEND[1] = 0.0;
            save.TIMBEG[2] = 0.0;
            save.TIMEND[2] = 0.0;

            if *OK {
                spicelib::WNFETD(
                    save.RESULT.as_slice(),
                    1,
                    &mut save.TIMBEG[1],
                    &mut save.TIMEND[1],
                    ctx,
                )?;

                for J in 2..=spicelib::WNCARD(save.RESULT.as_slice(), ctx)? {
                    spicelib::WNFETD(
                        save.RESULT.as_slice(),
                        J,
                        &mut save.TIMBEG[2],
                        &mut save.TIMEND[2],
                        ctx,
                    )?;

                    testutil::CHCKSD(
                        b"Y SWEEP BEG",
                        (save.TIMBEG[2] - save.TIMBEG[1]),
                        b"~",
                        spicelib::SPD(),
                        CNVTOL,
                        OK,
                        ctx,
                    )?;

                    testutil::CHCKSD(
                        b"Y SWEEP END",
                        (save.TIMEND[2] - save.TIMEND[1]),
                        b"~",
                        spicelib::SPD(),
                        CNVTOL,
                        OK,
                        ctx,
                    )?;

                    save.TIMBEG[1] = save.TIMBEG[2];
                    save.TIMEND[1] = save.TIMEND[2];
                }
            }

            save.I += m3__;
        }
    }

    //
    // Case 14, Sweeps of the GAMMA centered J2000 X axis on ALPHA.
    //
    save.STEP = (spicelib::SPD() * (1.25 / 24.0));

    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NDESC;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(
                &mut save.TITLE,
                &fstr::concat(
                    b"MDESC(#) ALPHA sweeps from GAMMA, X, ",
                    save.MDESC.get(save.I),
                ),
            );
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);

            testutil::TCASE(&save.TITLE, ctx)?;

            //
            // Parse from the MDESC(I) string the coordinate system,
            // coordinate, and relation operator.
            //
            spicelib::LPARSE(
                &save.MDESC[save.I],
                b":",
                3,
                &mut save.DIM,
                save.ITEMS.as_arg_mut(),
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.CRDSYS, save.ITEMS.get(1));
            fstr::assign(&mut save.COORD, save.ITEMS.get(2));
            fstr::assign(&mut save.RELATE, save.ITEMS.get(3));
            save.REFVAL = save.MREFS[save.I];

            spicelib::GFSNTC(
                &save.TARGET,
                &save.FIXREF,
                &save.METHOD,
                &save.ABCORR,
                &save.OBSRVR,
                &save.DREF,
                save.X.as_slice(),
                &save.CRDSYS,
                &save.COORD,
                &save.RELATE,
                save.REFVAL,
                save.ADJUST,
                save.STEP,
                save.CNFINE.as_slice(),
                MAXWIN,
                NWMAX,
                save.WORK.as_slice_mut(),
                save.RESULT.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // The J2000 X unit with origin at GAMMA's center should never
            // intersect ALPHA.
            //
            testutil::CHCKSI(
                b"WNCARD(RESULT)",
                spicelib::WNCARD(save.RESULT.as_slice(), ctx)?,
                b"=",
                0,
                0,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // Case 15
    //
    fstr::assign(&mut save.TITLE, b"Check the GF call uses the GFSTOL value");
    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // Re-run a valid search after using GFSTOL.
    //
    save.I = 1;

    //
    // Parse from the MDESC(I) string the coordinate system,
    // coordinate, and relation operator.
    //

    fstr::assign(&mut save.TIME0, b"2000 JAN 1   18:00:00 TDB");
    fstr::assign(&mut save.TIME1, b"2000 JAN 31  18:00:00 TDB");

    spicelib::STR2ET(&save.TIME0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&save.TIME1, &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = 0.0;
    fstr::assign(&mut save.TARGET, b"ALPHA");
    fstr::assign(&mut save.FIXREF, b"ALPHAFIXED");
    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.OBSRVR, b"GAMMA");
    fstr::assign(&mut save.DREF, b"J2000");
    fstr::assign(&mut save.ABCORR, b"NONE");
    save.STEP = (spicelib::SPD() * (1.25 / 24.0));

    save.I = 1;

    spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Parse from the MDESC(I) string the coordinate system,
    // coordinate, and relation operator.
    //
    spicelib::LPARSE(
        &save.MDESC[save.I],
        b":",
        3,
        &mut save.DIM,
        save.ITEMS.as_arg_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.CRDSYS, save.ITEMS.get(1));
    fstr::assign(&mut save.COORD, save.ITEMS.get(2));
    fstr::assign(&mut save.RELATE, save.ITEMS.get(3));
    save.REFVAL = save.MREFS[save.I];

    spicelib::GFSNTC(
        &save.TARGET,
        &save.FIXREF,
        &save.METHOD,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.Z.as_slice(),
        &save.CRDSYS,
        &save.COORD,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWMAX,
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

    spicelib::GFSNTC(
        &save.TARGET,
        &save.FIXREF,
        &save.METHOD,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.Z.as_slice(),
        &save.CRDSYS,
        &save.COORD,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWMAX,
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
    // Case n
    //
    testutil::TCASE(b"Clean up:  delete kernels.", ctx)?;

    spicelib::KCLEAR(ctx)?;

    spicelib::DELFIL(SPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(PCK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(PCK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
