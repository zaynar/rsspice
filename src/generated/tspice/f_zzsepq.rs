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
const MAXL: i32 = 36;
const MAXP: i32 = 150;
const NPERM: i32 = 692;
const MAXE: i32 = 853;
const NROOM: i32 = 14983;
const LSK: &[u8] = b"gfsep.tls";
const SPK1: &[u8] = b"nat.bsp";
const PCK1: &[u8] = b"nat.pck";
const TIGHT: f64 = 0.000000000001;
const LNSIZE: i32 = 80;
const TIMLEN: i32 = 50;
const NCORR: i32 = 9;
const ATOL: f64 = 0.000000000001;

struct SaveVars {
    FRAME: ActualCharArray,
    SHAPE: ActualCharArray,
    ABCORR: Vec<u8>,
    CORR: ActualCharArray,
    REF: Vec<u8>,
    TITLE: Vec<u8>,
    OBSRVR: Vec<u8>,
    TARG: ActualCharArray,
    TIME0: Vec<u8>,
    ANG1: f64,
    ANG2: f64,
    ET: f64,
    ET0: f64,
    LT: f64,
    MXRAD: StackArray<f64, 2>,
    PV1: StackArray<f64, 3>,
    PV2: StackArray<f64, 3>,
    R1: f64,
    R2: f64,
    RADA: StackArray<f64, 3>,
    RADB: StackArray<f64, 3>,
    RANGE1: f64,
    RANGE2: f64,
    THETA: f64,
    VALUE: f64,
    XR: StackArray2D<f64, 8>,
    BODS: StackArray<i32, 2>,
    DIM: i32,
    FRAMES: StackArray<i32, 2>,
    XFRAME: StackArray<i32, 2>,
    HAN1: i32,
    I: i32,
    M: i32,
    OBS: i32,
    TARG1: i32,
    TARG2: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FRAME = ActualCharArray::new(16, 1..=3);
        let mut SHAPE = ActualCharArray::new(16, 1..=3);
        let mut ABCORR = vec![b' '; LNSIZE as usize];
        let mut CORR = ActualCharArray::new(LNSIZE, 1..=(NCORR + 1));
        let mut REF = vec![b' '; LNSIZE as usize];
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut OBSRVR = vec![b' '; MAXL as usize];
        let mut TARG = ActualCharArray::new(MAXL, 1..=3);
        let mut TIME0 = vec![b' '; TIMLEN as usize];
        let mut ANG1: f64 = 0.0;
        let mut ANG2: f64 = 0.0;
        let mut ET: f64 = 0.0;
        let mut ET0: f64 = 0.0;
        let mut LT: f64 = 0.0;
        let mut MXRAD = StackArray::<f64, 2>::new(1..=2);
        let mut PV1 = StackArray::<f64, 3>::new(1..=3);
        let mut PV2 = StackArray::<f64, 3>::new(1..=3);
        let mut R1: f64 = 0.0;
        let mut R2: f64 = 0.0;
        let mut RADA = StackArray::<f64, 3>::new(1..=3);
        let mut RADB = StackArray::<f64, 3>::new(1..=3);
        let mut RANGE1: f64 = 0.0;
        let mut RANGE2: f64 = 0.0;
        let mut THETA: f64 = 0.0;
        let mut VALUE: f64 = 0.0;
        let mut XR = StackArray2D::<f64, 8>::new(1..=2, 1..=4);
        let mut BODS = StackArray::<i32, 2>::new(1..=2);
        let mut DIM: i32 = 0;
        let mut FRAMES = StackArray::<i32, 2>::new(1..=2);
        let mut XFRAME = StackArray::<i32, 2>::new(1..=2);
        let mut HAN1: i32 = 0;
        let mut I: i32 = 0;
        let mut M: i32 = 0;
        let mut OBS: i32 = 0;
        let mut TARG1: i32 = 0;
        let mut TARG2: i32 = 0;

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
                Val::C(b"BAD"),
            ]
            .into_iter();
            CORR.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"ALPHA_FIXED"),
                Val::C(b"BETA_FIXED"),
                Val::C(b"BAD"),
            ]
            .into_iter();
            FRAME
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"ALPHA"), Val::C(b"BETA"), Val::C(b"CALUFRAX")].into_iter();
            TARG.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"POINT"), Val::C(b"SPHERE"), Val::C(b"BAD")].into_iter();
            SHAPE
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            FRAME,
            SHAPE,
            ABCORR,
            CORR,
            REF,
            TITLE,
            OBSRVR,
            TARG,
            TIME0,
            ANG1,
            ANG2,
            ET,
            ET0,
            LT,
            MXRAD,
            PV1,
            PV2,
            R1,
            R2,
            RADA,
            RADB,
            RANGE1,
            RANGE2,
            THETA,
            VALUE,
            XR,
            BODS,
            DIM,
            FRAMES,
            XFRAME,
            HAN1,
            I,
            M,
            OBS,
            TARG1,
            TARG2,
        }
    }
}

//$Procedure F_ZZSEPQ ( ZZSEPQ family tests )
pub fn F_ZZSEPQ(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // ATOL is a tolerance value for computing arc sine.
    //

    //
    // Local variables
    //

    //
    // Save all.
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZSEPQ", ctx)?;

    testutil::TCASE(b"Setup: create and load SPK, PCK, LSK files.", ctx)?;

    //
    // Create an LSK, load using FURNSH.
    //
    testutil::ZZTSTLSK(LSK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(LSK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create the PCK for Nat's Solar System.
    //
    testutil::NATPCK(PCK1, false, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(PCK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create the SPK for Nat's Solar System.
    //
    testutil::NATSPK(SPK1, true, &mut save.HAN1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.TIME0, b"2000 JAN 1  12:00:00 TDB");

    spicelib::STR2ET(&save.TIME0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Case 1: ZZSPIN. Initial check to confirm functionality.
    //

    fstr::assign(&mut save.OBSRVR, b"SUN");
    spicelib::NAMFRM(&save.FRAME[1], &mut save.XFRAME[1], ctx)?;
    spicelib::NAMFRM(&save.FRAME[2], &mut save.XFRAME[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::ListDirectedWriter::new(ctx.default_write_unit()?, None)?;
        writer.start()?;
        writer.finish()?;
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = NCORR;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            testutil::TCASE(
                &fstr::concat(b"Confirm basic functionality ", save.CORR.get(save.I)),
                ctx,
            )?;

            //
            // SHAPE(2), "SPHERE", to check MXRAD.
            //
            spicelib::ZZSPIN(
                &save.TARG[1],
                &save.SHAPE[2],
                &save.FRAME[1],
                &save.TARG[2],
                &save.SHAPE[2],
                &save.FRAME[2],
                &save.OBSRVR,
                &save.CORR[save.I],
                save.BODS.as_slice_mut(),
                save.FRAMES.as_slice_mut(),
                save.MXRAD.as_slice_mut(),
                &mut save.OBS,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSI(b"TARG(1) ID", save.BODS[1], b"=", 1000, 0, OK, ctx)?;
            testutil::CHCKSI(b"TARG(2) ID", save.BODS[2], b"=", 2000, 0, OK, ctx)?;
            testutil::CHCKSI(b"OBSRVR ID", save.OBS, b"=", 10, 0, OK, ctx)?;

            testutil::CHCKSD(
                b"MAXRAD(1)",
                save.MXRAD[1],
                b"~",
                73249.39753387542,
                TIGHT,
                OK,
                ctx,
            )?;

            testutil::CHCKSD(
                b"MAXRAD(1)",
                save.MXRAD[2],
                b"~",
                2289.1526271046937,
                TIGHT,
                OK,
                ctx,
            )?;

            //
            //     The current version of ZZSPIN ignores the FRAME argument.
            //
            //     CALL CHCKSI ( 'FRAMES(1) ID', FRAMES(1), '=',
            // .                                 XFRAME(1), 0, OK )
            //     CALL CHCKSI ( 'FRAMES(2) ID', FRAMES(2), '=',
            // .                                 XFRAME(2), 0, OK )
            //

            save.I += m3__;
        }
    }

    testutil::TCASE(b"BAD TARG1", ctx)?;

    spicelib::ZZSPIN(
        &save.TARG[3],
        &save.SHAPE[1],
        &save.FRAME[1],
        &save.TARG[2],
        &save.SHAPE[1],
        &save.FRAME[2],
        &save.OBSRVR,
        &save.CORR[1],
        save.BODS.as_slice_mut(),
        save.FRAMES.as_slice_mut(),
        save.MXRAD.as_slice_mut(),
        &mut save.OBS,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    testutil::TCASE(b"BAD TARG2", ctx)?;

    spicelib::ZZSPIN(
        &save.TARG[1],
        &save.SHAPE[1],
        &save.FRAME[1],
        &save.TARG[3],
        &save.SHAPE[1],
        &save.FRAME[2],
        &save.OBSRVR,
        &save.CORR[save.I],
        save.BODS.as_slice_mut(),
        save.FRAMES.as_slice_mut(),
        save.MXRAD.as_slice_mut(),
        &mut save.OBS,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    testutil::TCASE(b"BAD OBSRVR", ctx)?;

    spicelib::ZZSPIN(
        &save.TARG[1],
        &save.SHAPE[1],
        &save.FRAME[1],
        &save.TARG[2],
        &save.SHAPE[1],
        &save.FRAME[2],
        &save.TARG[3],
        &save.CORR[1],
        save.BODS.as_slice_mut(),
        save.FRAMES.as_slice_mut(),
        save.MXRAD.as_slice_mut(),
        &mut save.OBS,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    testutil::TCASE(b"Bodies not distinct", ctx)?;

    spicelib::ZZSPIN(
        &save.TARG[1],
        &save.SHAPE[1],
        &save.FRAME[1],
        &save.TARG[1],
        &save.SHAPE[1],
        &save.FRAME[2],
        &save.OBSRVR,
        &save.CORR[1],
        save.BODS.as_slice_mut(),
        save.FRAMES.as_slice_mut(),
        save.MXRAD.as_slice_mut(),
        &mut save.OBS,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    spicelib::ZZSPIN(
        &save.TARG[1],
        &save.SHAPE[1],
        &save.FRAME[1],
        &save.TARG[2],
        &save.SHAPE[1],
        &save.FRAME[2],
        &save.TARG[2],
        &save.CORR[1],
        save.BODS.as_slice_mut(),
        save.FRAMES.as_slice_mut(),
        save.MXRAD.as_slice_mut(),
        &mut save.OBS,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    spicelib::ZZSPIN(
        &save.TARG[1],
        &save.SHAPE[1],
        &save.FRAME[1],
        &save.TARG[2],
        &save.SHAPE[1],
        &save.FRAME[2],
        &save.TARG[1],
        &save.CORR[1],
        save.BODS.as_slice_mut(),
        save.FRAMES.as_slice_mut(),
        save.MXRAD.as_slice_mut(),
        &mut save.OBS,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    testutil::TCASE(b"BAD SHAPE1", ctx)?;

    spicelib::ZZSPIN(
        &save.TARG[1],
        &save.SHAPE[3],
        &save.FRAME[1],
        &save.TARG[2],
        &save.SHAPE[1],
        &save.FRAME[2],
        &save.OBSRVR,
        &save.CORR[1],
        save.BODS.as_slice_mut(),
        save.FRAMES.as_slice_mut(),
        save.MXRAD.as_slice_mut(),
        &mut save.OBS,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOTRECOGNIZED)", OK, ctx)?;

    testutil::TCASE(b"BAD SHAPE2)", ctx)?;

    spicelib::ZZSPIN(
        &save.TARG[1],
        &save.SHAPE[2],
        &save.FRAME[1],
        &save.TARG[2],
        &save.SHAPE[3],
        &save.FRAME[2],
        &save.OBSRVR,
        &save.CORR[1],
        save.BODS.as_slice_mut(),
        save.FRAMES.as_slice_mut(),
        save.MXRAD.as_slice_mut(),
        &mut save.OBS,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOTRECOGNIZED)", OK, ctx)?;

    testutil::TCASE(b"BAD ABCORR)", ctx)?;

    spicelib::ZZSPIN(
        &save.TARG[1],
        &save.SHAPE[2],
        &save.FRAME[1],
        &save.TARG[2],
        &save.SHAPE[2],
        &save.FRAME[2],
        &save.OBSRVR,
        &save.CORR[(NCORR + 1)],
        save.BODS.as_slice_mut(),
        save.FRAMES.as_slice_mut(),
        save.MXRAD.as_slice_mut(),
        &mut save.OBS,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // Case 2: ZZSEPQ
    //
    testutil::TCASE(b"Ephemeris data unavailable", ctx)?;

    save.TARG1 = -203;
    save.TARG2 = 499;
    save.OBS = 10;
    save.R1 = 0.0;
    save.R2 = 0.0;
    fstr::assign(&mut save.REF, b"J2000");
    fstr::assign(&mut save.ABCORR, b"NONE");

    save.VALUE = spicelib::ZZSEPQ(
        save.ET0,
        save.TARG1,
        save.TARG2,
        save.R1,
        save.R2,
        save.OBS,
        &save.ABCORR,
        &save.REF,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    spicelib::BODVRD(
        b"ALPHA",
        b"RADII",
        3,
        &mut save.DIM,
        save.RADA.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKSI(b"BODVRD ALPHA", save.DIM, b"=", 3, 0, OK, ctx)?;

    spicelib::BODVRD(
        b"BETA",
        b"RADII",
        3,
        &mut save.DIM,
        save.RADB.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKSI(b"BODVRD ALPHA", save.DIM, b"=", 3, 0, OK, ctx)?;

    //
    // Case 3
    //
    testutil::TCASE(b"Negative body radius R1 and R2", ctx)?;

    spicelib::BODVRD(
        b"ALPHA",
        b"RADII",
        3,
        &mut save.DIM,
        save.RADA.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKSI(b"BODVRD ALPHA", save.DIM, b"=", 3, 0, OK, ctx)?;

    spicelib::BODVRD(
        b"BETA",
        b"RADII",
        3,
        &mut save.DIM,
        save.RADB.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKSI(b"BODVRD ALPHA", save.DIM, b"=", 3, 0, OK, ctx)?;

    save.TARG1 = 1000;
    save.TARG2 = 2000;
    save.OBS = 10;
    save.R1 = -save.RADA[1];
    save.R2 = 0.0;
    fstr::assign(&mut save.REF, b"J2000");
    fstr::assign(&mut save.ABCORR, b"NONE");

    save.VALUE = spicelib::ZZSEPQ(
        save.ET0,
        save.TARG1,
        save.TARG2,
        save.R1,
        save.R2,
        save.OBS,
        &save.ABCORR,
        &save.REF,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADRADIUS)", OK, ctx)?;

    save.R1 = 0.0;
    save.R2 = -save.RADB[1];

    save.VALUE = spicelib::ZZSEPQ(
        save.ET0,
        save.TARG1,
        save.TARG2,
        save.R1,
        save.R2,
        save.OBS,
        &save.ABCORR,
        &save.REF,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADRADIUS)", OK, ctx)?;

    //
    // Case 4
    //

    //
    // Check a simple calculation for all CORR.
    //
    save.TARG1 = 1000;
    save.TARG2 = 2000;
    save.OBS = 10;
    save.R1 = 0.0;
    save.R2 = 0.0;
    fstr::assign(&mut save.REF, b"J2000");
    fstr::assign(&mut save.ABCORR, b"NONE");

    {
        let m1__: i32 = 1;
        let m2__: i32 = NCORR;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            testutil::TCASE(&save.CORR[save.I], ctx)?;

            save.VALUE = spicelib::ZZSEPQ(
                save.ET0,
                save.TARG1,
                save.TARG2,
                save.R1,
                save.R2,
                save.OBS,
                &save.CORR[save.I],
                &save.REF,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Check a simple calculation for invalid CORR.
    //
    fstr::assign(&mut save.ABCORR, save.CORR.get((NCORR + 1)));

    testutil::TCASE(&save.ABCORR, ctx)?;

    save.VALUE = spicelib::ZZSEPQ(
        save.ET0,
        save.TARG1,
        save.TARG2,
        save.R1,
        save.R2,
        save.OBS,
        &save.ABCORR,
        &save.REF,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SPKINVALIDOPTION)", OK, ctx)?;

    //
    // Case 5
    //

    //
    // Run a loop over one orbit of ALPHA at 50 TDB second
    // intervals with each of the aberration corrections.
    // Any occultation event exists for longer than 50 seconds.
    // Compare the value returned from ZZSEPQ with a direct
    // calculation.
    //
    save.TARG1 = 1000;
    save.TARG2 = 2000;
    save.OBS = 10;
    fstr::assign(&mut save.REF, b"J2000");

    //
    // 1 - point, point
    // 2 - sphere, point
    // 3 - point, sphere,
    // 4 - sphere, sphere.
    //
    save.XR[[1, 1]] = 0.0;
    save.XR[[1, 2]] = save.RADA[1];
    save.XR[[1, 3]] = 0.0;
    save.XR[[1, 4]] = save.RADA[1];
    save.XR[[2, 1]] = 0.0;
    save.XR[[2, 2]] = 0.0;
    save.XR[[2, 3]] = save.RADB[1];
    save.XR[[2, 4]] = save.RADB[1];

    {
        let m1__: i32 = 0;
        let m2__: i32 = 86400;
        let m3__: i32 = 50;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            for J in 1..=NCORR {
                fstr::assign(&mut save.ABCORR, save.CORR.get(J));

                for K in 1..=4 {
                    save.R1 = save.XR[[1, K]];
                    save.R2 = save.XR[[2, K]];

                    fstr::assign(&mut save.TITLE, b"ET = #1, R1 = #2, R2 = #3, CORR = #4");
                    spicelib::REPMD(
                        &save.TITLE.to_vec(),
                        b"#1",
                        save.ET,
                        6,
                        &mut save.TITLE,
                        ctx,
                    );
                    spicelib::REPMD(
                        &save.TITLE.to_vec(),
                        b"#2",
                        save.R1,
                        6,
                        &mut save.TITLE,
                        ctx,
                    );
                    spicelib::REPMD(
                        &save.TITLE.to_vec(),
                        b"#3",
                        save.R2,
                        6,
                        &mut save.TITLE,
                        ctx,
                    );
                    spicelib::REPMI(&save.TITLE.to_vec(), b"#4", J, &mut save.TITLE, ctx);
                    testutil::TCASE(&save.TITLE, ctx)?;

                    save.ET = (save.ET0 + (save.I as f64));

                    save.VALUE = spicelib::ZZSEPQ(
                        save.ET,
                        save.TARG1,
                        save.TARG2,
                        save.R1,
                        save.R2,
                        save.OBS,
                        &save.ABCORR,
                        &save.REF,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::SPKEZP(
                        save.TARG1,
                        save.ET,
                        &save.REF,
                        &save.ABCORR,
                        save.OBS,
                        save.PV1.as_slice_mut(),
                        &mut save.LT,
                        ctx,
                    )?;
                    spicelib::SPKEZP(
                        save.TARG2,
                        save.ET,
                        &save.REF,
                        &save.ABCORR,
                        save.OBS,
                        save.PV2.as_slice_mut(),
                        &mut save.LT,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.RANGE1 = spicelib::VNORM(save.PV1.as_slice());
                    save.RANGE2 = spicelib::VNORM(save.PV2.as_slice());
                    save.ANG1 = spicelib::DASINE((save.R1 / save.RANGE1), ATOL, ctx)?;
                    save.ANG2 = spicelib::DASINE((save.R2 / save.RANGE2), ATOL, ctx)?;

                    save.THETA = ((spicelib::VSEP(save.PV1.as_slice(), save.PV2.as_slice(), ctx)
                        - save.ANG1)
                        - save.ANG2);
                    //
                    // The results should match to roundoff.
                    //
                    testutil::CHCKSD(&save.TITLE, save.VALUE, b"=", save.THETA, TIGHT, OK, ctx)?;
                }
            }

            save.I += m3__;
        }
    }

    //
    // Same loop using TRGSEP.
    //

    fstr::assign(&mut save.REF, b"J2000");

    //
    // 1 - point, point
    // 2 - sphere, point
    // 3 - point, sphere,
    // 4 - sphere, sphere.
    //
    save.XR[[1, 1]] = 0.0;
    save.XR[[1, 2]] = save.RADA[1];
    save.XR[[1, 3]] = 0.0;
    save.XR[[1, 4]] = save.RADA[1];
    save.XR[[2, 1]] = 0.0;
    save.XR[[2, 2]] = 0.0;
    save.XR[[2, 3]] = save.RADB[1];
    save.XR[[2, 4]] = save.RADB[1];

    {
        let m1__: i32 = 0;
        let m2__: i32 = 86400;
        let m3__: i32 = 50;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            for J in 1..=NCORR {
                fstr::assign(&mut save.ABCORR, save.CORR.get(J));
                save.M = 1;

                for K in 1..=2 {
                    for L in 1..=2 {
                        save.R1 = save.XR[[1, save.M]];
                        save.R2 = save.XR[[2, save.M]];
                        save.M = (save.M + 1);

                        fstr::assign(&mut save.TITLE, b"SEP ET = #1, R1 = #2, R2 = #3, CORR = #4");
                        spicelib::REPMD(
                            &save.TITLE.to_vec(),
                            b"#1",
                            save.ET,
                            6,
                            &mut save.TITLE,
                            ctx,
                        );
                        spicelib::REPMD(
                            &save.TITLE.to_vec(),
                            b"#2",
                            save.R1,
                            6,
                            &mut save.TITLE,
                            ctx,
                        );
                        spicelib::REPMD(
                            &save.TITLE.to_vec(),
                            b"#3",
                            save.R2,
                            6,
                            &mut save.TITLE,
                            ctx,
                        );
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#4", J, &mut save.TITLE, ctx);
                        testutil::TCASE(&save.TITLE, ctx)?;

                        save.ET = (save.ET0 + (save.I as f64));

                        save.VALUE = spicelib::TRGSEP(
                            save.ET,
                            &save.TARG[1],
                            &save.SHAPE[L],
                            &save.FRAME[1],
                            &save.TARG[2],
                            &save.SHAPE[K],
                            &save.FRAME[2],
                            &save.OBSRVR,
                            &save.ABCORR,
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        spicelib::SPKEZP(
                            save.TARG1,
                            save.ET,
                            &save.REF,
                            &save.ABCORR,
                            save.OBS,
                            save.PV1.as_slice_mut(),
                            &mut save.LT,
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        spicelib::SPKEZP(
                            save.TARG2,
                            save.ET,
                            &save.REF,
                            &save.ABCORR,
                            save.OBS,
                            save.PV2.as_slice_mut(),
                            &mut save.LT,
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        save.RANGE1 = spicelib::VNORM(save.PV1.as_slice());
                        save.RANGE2 = spicelib::VNORM(save.PV2.as_slice());

                        save.ANG1 = spicelib::DASINE((save.R1 / save.RANGE1), ATOL, ctx)?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        save.ANG2 = spicelib::DASINE((save.R2 / save.RANGE2), ATOL, ctx)?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        save.THETA =
                            ((spicelib::VSEP(save.PV1.as_slice(), save.PV2.as_slice(), ctx)
                                - save.ANG1)
                                - save.ANG2);

                        //
                        // The results should match to roundoff, or
                        // something badly wrong.
                        //
                        testutil::CHCKSD(
                            &save.TITLE,
                            save.VALUE,
                            b"=",
                            save.THETA,
                            TIGHT,
                            OK,
                            ctx,
                        )?;
                    }
                }
            }

            save.I += m3__;
        }
    }

    //
    // Case n
    //
    testutil::TCASE(b"Clean up:  delete kernels.", ctx)?;

    spicelib::KCLEAR(ctx)?;

    spicelib::DELFIL(LSK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(PCK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
