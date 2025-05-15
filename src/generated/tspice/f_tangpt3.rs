//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const DSKSHP: i32 = 2;
const ELLSHP: i32 = 1;
const MTHLEN: i32 = 500;
const SUBLEN: i32 = 20;
const CVTLEN: i32 = 20;
const TANGNT: i32 = 1;
const GUIDED: i32 = 2;
const TMTLEN: i32 = 20;
const LMBCRV: i32 = 0;
const UMBRAL: i32 = 1;
const PNMBRL: i32 = 2;
const ACLLEN: i32 = 25;
const CTRCOR: i32 = 1;
const ELLCOR: i32 = 2;
const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;
const PCK0: &[u8] = b"tangpt_test0.tpc";
const PCK1: &[u8] = b"tangpt_test0.tpc";
const SCNAME: &[u8] = b"MARS_LOW_ORBITER";
const SPK0: &[u8] = b"tangpt_test0.bsp";
const SPK1: &[u8] = b"tangpt_test1.bsp";
const BDNMLN: i32 = 36;
const FRNMLN: i32 = 32;
const IXPAIR: i32 = 1;
const IXRFRM: i32 = (IXPAIR + 1);
const IXCORR: i32 = (IXRFRM + 1);
const IXLOC: i32 = (IXCORR + 1);
const IXSHAP: i32 = (IXLOC + 1);
const LNSIZE: i32 = 255;
const LOCLEN: i32 = 20;
const NAZ: i32 = 1;
const NCORR: i32 = 9;
const NDIMS: i32 = 6;
const NEL: i32 = 6;
const NELTS: i32 = 8;
const NFRAME: i32 = 4;
const NLOC: i32 = 2;
const NPAIRS: i32 = 3;
const NSHAPE: i32 = 1;
const NTIMES: i32 = 1;
const SCID: i32 = -499001;
const SHPLEN: i32 = 25;
const TIMLEN: i32 = 40;
const TIGHT: f64 = 0.000000000001;
const VTIGHT: f64 = 0.00000000000001;

struct SaveVars {
    ABCORR: Vec<u8>,
    CORARR: ActualCharArray,
    CORLOC: Vec<u8>,
    FIXARR: ActualCharArray,
    FIXREF: Vec<u8>,
    LOCARR: ActualCharArray,
    OBSARR: ActualCharArray,
    OBSRVR: Vec<u8>,
    RAYARR: ActualCharArray,
    RAYFRM: Vec<u8>,
    RFCNAM: Vec<u8>,
    SHAPNM: ActualCharArray,
    SHPNAM: Vec<u8>,
    TARGET: Vec<u8>,
    TITLE: Vec<u8>,
    TRGARR: ActualCharArray,
    UTC: Vec<u8>,
    ALT: f64,
    AZ: f64,
    AZDELT: f64,
    CP: StackArray<f64, 3>,
    EL: f64,
    ELEVS: StackArray<f64, 6>,
    ELTS: StackArray<f64, 8>,
    EPOCHS: StackArray<f64, 1>,
    ET: f64,
    ET0: f64,
    FIXDIR: StackArray<f64, 3>,
    LNORML: StackArray<f64, 3>,
    LT: f64,
    NORMAL: StackArray<f64, 3>,
    PNEAR: StackArray<f64, 3>,
    POINTS: StackArray2D<f64, 3>,
    POS: StackArray<f64, 3>,
    RADII: StackArray<f64, 3>,
    RANGE: f64,
    RAYDIR: StackArray<f64, 3>,
    RAYEPC: f64,
    RAYMAT: StackArray2D<f64, 9>,
    REFVEC: StackArray<f64, 3>,
    RFMAT: StackArray2D<f64, 9>,
    SCHSTP: f64,
    SEP: f64,
    SOLTOL: f64,
    SRFPT: StackArray<f64, 3>,
    SRFSTA: StackArray<f64, 6>,
    SRFTAN: StackArray<f64, 3>,
    SRFVEC: StackArray<f64, 3>,
    STATE0: StackArray<f64, 6>,
    STLOBS: StackArray<f64, 3>,
    SUBALT: f64,
    SUBP: StackArray<f64, 3>,
    SUBZEN: StackArray<f64, 3>,
    TANGTS: StackArray2D<f64, 3>,
    TANPT: StackArray<f64, 3>,
    TANSTA: StackArray<f64, 6>,
    THETA: f64,
    TOL: f64,
    TRGEPC: f64,
    XALT: f64,
    XEPOCH: f64,
    XFORM: StackArray2D<f64, 9>,
    XLT: f64,
    XPT: StackArray<f64, 3>,
    XPTEPC: f64,
    XPTVEC: StackArray<f64, 3>,
    XRANGE: f64,
    XRAYDR: StackArray<f64, 3>,
    XSRFPT: StackArray<f64, 3>,
    XSRFVC: StackArray<f64, 3>,
    XTANPT: StackArray<f64, 3>,
    CIX: i32,
    COORDS: StackArray<i32, 6>,
    DIMS: StackArray<i32, 6>,
    FIX: i32,
    HAN0: i32,
    HAN1: i32,
    LIX: i32,
    MAXN: i32,
    NCASES: i32,
    NPTS: StackArray<i32, 1>,
    NRAD: i32,
    PIX: i32,
    RAYFID: i32,
    RFCENT: i32,
    RFCLAS: i32,
    RFCLID: i32,
    ATTBLK: StackArray<bool, 6>,
    FOUND: bool,
    GEOM: bool,
    USECN: bool,
    USELT: bool,
    USESTL: bool,
    XPTFND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ABCORR = vec![b' '; CORLEN as usize];
        let mut CORARR = ActualCharArray::new(CORLEN, 1..=NCORR);
        let mut CORLOC = vec![b' '; LOCLEN as usize];
        let mut FIXARR = ActualCharArray::new(FRNMLN, 1..=NPAIRS);
        let mut FIXREF = vec![b' '; FRNMLN as usize];
        let mut LOCARR = ActualCharArray::new(LOCLEN, 1..=NLOC);
        let mut OBSARR = ActualCharArray::new(BDNMLN, 1..=NPAIRS);
        let mut OBSRVR = vec![b' '; BDNMLN as usize];
        let mut RAYARR = ActualCharArray::new(FRNMLN, 1..=NFRAME);
        let mut RAYFRM = vec![b' '; FRNMLN as usize];
        let mut RFCNAM = vec![b' '; BDNMLN as usize];
        let mut SHAPNM = ActualCharArray::new(SHPLEN, 1..=NSHAPE);
        let mut SHPNAM = vec![b' '; SHPLEN as usize];
        let mut TARGET = vec![b' '; BDNMLN as usize];
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut TRGARR = ActualCharArray::new(BDNMLN, 1..=NPAIRS);
        let mut UTC = vec![b' '; TIMLEN as usize];
        let mut ALT: f64 = 0.0;
        let mut AZ: f64 = 0.0;
        let mut AZDELT: f64 = 0.0;
        let mut CP = StackArray::<f64, 3>::new(1..=3);
        let mut EL: f64 = 0.0;
        let mut ELEVS = StackArray::<f64, 6>::new(1..=NEL);
        let mut ELTS = StackArray::<f64, 8>::new(1..=NELTS);
        let mut EPOCHS = StackArray::<f64, 1>::new(1..=NAZ);
        let mut ET: f64 = 0.0;
        let mut ET0: f64 = 0.0;
        let mut FIXDIR = StackArray::<f64, 3>::new(1..=3);
        let mut LNORML = StackArray::<f64, 3>::new(1..=3);
        let mut LT: f64 = 0.0;
        let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
        let mut PNEAR = StackArray::<f64, 3>::new(1..=3);
        let mut POINTS = StackArray2D::<f64, 3>::new(1..=3, 1..=NAZ);
        let mut POS = StackArray::<f64, 3>::new(1..=3);
        let mut RADII = StackArray::<f64, 3>::new(1..=3);
        let mut RANGE: f64 = 0.0;
        let mut RAYDIR = StackArray::<f64, 3>::new(1..=3);
        let mut RAYEPC: f64 = 0.0;
        let mut RAYMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut REFVEC = StackArray::<f64, 3>::new(1..=3);
        let mut RFMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut SCHSTP: f64 = 0.0;
        let mut SEP: f64 = 0.0;
        let mut SOLTOL: f64 = 0.0;
        let mut SRFPT = StackArray::<f64, 3>::new(1..=3);
        let mut SRFSTA = StackArray::<f64, 6>::new(1..=6);
        let mut SRFTAN = StackArray::<f64, 3>::new(1..=3);
        let mut SRFVEC = StackArray::<f64, 3>::new(1..=3);
        let mut STATE0 = StackArray::<f64, 6>::new(1..=6);
        let mut STLOBS = StackArray::<f64, 3>::new(1..=3);
        let mut SUBALT: f64 = 0.0;
        let mut SUBP = StackArray::<f64, 3>::new(1..=3);
        let mut SUBZEN = StackArray::<f64, 3>::new(1..=3);
        let mut TANGTS = StackArray2D::<f64, 3>::new(1..=3, 1..=NAZ);
        let mut TANPT = StackArray::<f64, 3>::new(1..=3);
        let mut TANSTA = StackArray::<f64, 6>::new(1..=6);
        let mut THETA: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut TRGEPC: f64 = 0.0;
        let mut XALT: f64 = 0.0;
        let mut XEPOCH: f64 = 0.0;
        let mut XFORM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut XLT: f64 = 0.0;
        let mut XPT = StackArray::<f64, 3>::new(1..=3);
        let mut XPTEPC: f64 = 0.0;
        let mut XPTVEC = StackArray::<f64, 3>::new(1..=3);
        let mut XRANGE: f64 = 0.0;
        let mut XRAYDR = StackArray::<f64, 3>::new(1..=3);
        let mut XSRFPT = StackArray::<f64, 3>::new(1..=3);
        let mut XSRFVC = StackArray::<f64, 3>::new(1..=3);
        let mut XTANPT = StackArray::<f64, 3>::new(1..=3);
        let mut CIX: i32 = 0;
        let mut COORDS = StackArray::<i32, 6>::new(1..=NDIMS);
        let mut DIMS = StackArray::<i32, 6>::new(1..=NDIMS);
        let mut FIX: i32 = 0;
        let mut HAN0: i32 = 0;
        let mut HAN1: i32 = 0;
        let mut LIX: i32 = 0;
        let mut MAXN: i32 = 0;
        let mut NCASES: i32 = 0;
        let mut NPTS = StackArray::<i32, 1>::new(1..=NAZ);
        let mut NRAD: i32 = 0;
        let mut PIX: i32 = 0;
        let mut RAYFID: i32 = 0;
        let mut RFCENT: i32 = 0;
        let mut RFCLAS: i32 = 0;
        let mut RFCLID: i32 = 0;
        let mut ATTBLK = StackArray::<bool, 6>::new(1..=ABATSZ);
        let mut FOUND: bool = false;
        let mut GEOM: bool = false;
        let mut USECN: bool = false;
        let mut USELT: bool = false;
        let mut USESTL: bool = false;
        let mut XPTFND: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"NONE"),
                Val::C(b"CN"),
                Val::C(b"XCN"),
                Val::C(b"CN+S"),
                Val::C(b"XCN+S"),
                Val::C(b"LT"),
                Val::C(b"XLT"),
                Val::C(b"LT+S"),
                Val::C(b"XLT+S"),
            ]
            .into_iter();
            CORARR
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(NPAIRS),
                Val::I(NFRAME),
                Val::I(NCORR),
                Val::I(NLOC),
                Val::I(NSHAPE),
                Val::I(NTIMES),
            ]
            .into_iter();
            DIMS.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(0.000001),
                Val::D(0.01),
                Val::D(0.1),
                Val::D(1.0),
                Val::D(-0.00000005),
                Val::D(1.6),
            ]
            .into_iter();
            ELEVS
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"IAU_MARS"),
                Val::C(b"IAU_MARS"),
                Val::C(b"IAU_EARTH"),
            ]
            .into_iter();
            FIXARR
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"TANGENT POINT"), Val::C(b"SURFACE POINT")].into_iter();
            LOCARR
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(SCNAME), Val::C(b"PHOBOS"), Val::C(b"MARS")].into_iter();
            OBSARR
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"J2000"),
                Val::C(b"IAU_MARS"),
                Val::C(b"ECLIPJ2000"),
                Val::C(b"IAU_PHOBOS"),
            ]
            .into_iter();
            RAYARR
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"Normal")].into_iter();
            SHAPNM
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"MARS"), Val::C(b"MARS"), Val::C(b"EARTH")].into_iter();
            TRGARR
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            ABCORR,
            CORARR,
            CORLOC,
            FIXARR,
            FIXREF,
            LOCARR,
            OBSARR,
            OBSRVR,
            RAYARR,
            RAYFRM,
            RFCNAM,
            SHAPNM,
            SHPNAM,
            TARGET,
            TITLE,
            TRGARR,
            UTC,
            ALT,
            AZ,
            AZDELT,
            CP,
            EL,
            ELEVS,
            ELTS,
            EPOCHS,
            ET,
            ET0,
            FIXDIR,
            LNORML,
            LT,
            NORMAL,
            PNEAR,
            POINTS,
            POS,
            RADII,
            RANGE,
            RAYDIR,
            RAYEPC,
            RAYMAT,
            REFVEC,
            RFMAT,
            SCHSTP,
            SEP,
            SOLTOL,
            SRFPT,
            SRFSTA,
            SRFTAN,
            SRFVEC,
            STATE0,
            STLOBS,
            SUBALT,
            SUBP,
            SUBZEN,
            TANGTS,
            TANPT,
            TANSTA,
            THETA,
            TOL,
            TRGEPC,
            XALT,
            XEPOCH,
            XFORM,
            XLT,
            XPT,
            XPTEPC,
            XPTVEC,
            XRANGE,
            XRAYDR,
            XSRFPT,
            XSRFVC,
            XTANPT,
            CIX,
            COORDS,
            DIMS,
            FIX,
            HAN0,
            HAN1,
            LIX,
            MAXN,
            NCASES,
            NPTS,
            NRAD,
            PIX,
            RAYFID,
            RFCENT,
            RFCLAS,
            RFCLID,
            ATTBLK,
            FOUND,
            GEOM,
            USECN,
            USELT,
            USESTL,
            XPTFND,
        }
    }
}

//$Procedure F_TANGPT3 ( Test tangent point routine TANGPT, part 3 )
pub fn F_TANGPT3(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Elevation units are radians.
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_TANGPT3", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Setup", ctx)?;

    //
    // Create and load LSK, then delete LSK.
    //
    testutil::TSTLSK(ctx)?;

    //
    // Create default test PCK.
    //
    if spicelib::EXISTS(PCK0, ctx)? {
        spicelib::DELFIL(PCK0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    testutil::T_PCK10(PCK0, true, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create test SPK.
    //
    if spicelib::EXISTS(SPK0, ctx)? {
        spicelib::DELFIL(SPK0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    testutil::TSTSPK(SPK0, true, &mut save.HAN0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a Mars orbiter SPK file.
    //
    if spicelib::EXISTS(SPK1, ctx)? {
        spicelib::DELFIL(SPK1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::SPKOPN(SPK1, SPK1, 0, &mut save.HAN1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set initial time.
    //
    fstr::assign(&mut save.UTC, b"2020 JAN 1");
    spicelib::STR2ET(&save.UTC, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set up elements defining a state.  The elements expected
    // by CONICS are:
    //
    //    RP      Perifocal distance.
    //    ECC     Eccentricity.
    //    INC     Inclination.
    //    LNODE   Longitude of the ascending node.
    //    ARGP    Argument of periapse.
    //    M0      Mean anomaly at epoch.
    //    T0      Epoch.
    //    MU      Gravitational parameter.
    //
    save.ELTS[1] = 3500.0;
    save.ELTS[2] = 0.1;
    save.ELTS[3] = (80.0 * spicelib::RPD(ctx));
    save.ELTS[4] = 0.0;
    save.ELTS[5] = (90.0 * spicelib::RPD(ctx));
    save.ELTS[6] = 0.0;
    save.ELTS[7] = save.ET0;
    save.ELTS[8] = 42828.314;

    spicelib::CONICS(
        save.ELTS.as_slice(),
        save.ET0,
        save.STATE0.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKW05(
        save.HAN1,
        SCID,
        499,
        b"MARSIAU",
        -((20 as f64) * spicelib::JYEAR()),
        ((20 as f64) * spicelib::JYEAR()),
        b"Mars orbiter",
        save.ELTS[8],
        1,
        save.STATE0.as_slice(),
        &[save.ET0],
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKCLS(save.HAN1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load the new spacecraft SPK file.
    //
    spicelib::SPKLEF(SPK1, &mut save.HAN1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Add the orbiter's name/ID mapping to the kernel pool.
    //
    spicelib::PCPOOL(b"NAIF_BODY_NAME", 1, CharArray::from_ref(SCNAME), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::PIPOOL(b"NAIF_BODY_CODE", 1, &[SCID], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*****************************************************************
    //
    //     Normal cases
    //
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Setup for Mars-Phobos tests", ctx)?;

    //
    // The following sets of tests cover a range of combinations of
    //
    //    - Observer-target pairs  3
    //    - Ray frames  4
    //    - Aberration corrections 9
    //    - Correction loci  2
    //    - Ray azimuths  1
    //    - Ray elevations  6
    //    - Shapes  1
    //    - Times  1
    //

    //
    // Set reference epoch. We use a single epoch in this test family.
    //
    save.ET0 = ((5 as f64) * spicelib::JYEAR());

    //
    // We use the utility MULTIX to convert a 1-dimensional index to
    // a 6-dimensional index. For efficiency, we handle AZ and EL
    // values separately; this allows us to do one limb computation
    // for all ray directions.
    //
    save.NCASES = (((((NPAIRS * NFRAME) * NCORR) * NLOC) * NSHAPE) * NTIMES);

    for CASE in 1..=save.NCASES {
        //
        // --- Case: ------------------------------------------------------
        //

        //
        // Compute the indices of each input from the case number.
        //
        testutil::MULTIX(
            1,
            NDIMS,
            save.DIMS.as_slice(),
            CASE,
            save.COORDS.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.PIX = save.COORDS[IXPAIR];
        save.FIX = save.COORDS[IXRFRM];
        save.CIX = save.COORDS[IXCORR];
        save.LIX = save.COORDS[IXLOC];

        //
        // Set the inputs to TANGPT based on the test case number.
        //
        fstr::assign(&mut save.TARGET, save.TRGARR.get(save.PIX));
        fstr::assign(&mut save.OBSRVR, save.OBSARR.get(save.PIX));
        fstr::assign(&mut save.FIXREF, save.FIXARR.get(save.PIX));
        fstr::assign(&mut save.RAYFRM, save.RAYARR.get(save.FIX));
        fstr::assign(&mut save.CORLOC, save.LOCARR.get(save.LIX));
        fstr::assign(&mut save.SHPNAM, save.SHAPNM.get(1));

        //
        // Set the aberration correction and get the corresponding
        // attribute block.
        //
        fstr::assign(&mut save.ABCORR, save.CORARR.get(save.CIX));
        spicelib::ZZVALCOR(&save.ABCORR, save.ATTBLK.as_slice_mut(), ctx)?;

        save.USELT = save.ATTBLK[LTIDX];
        save.GEOM = !save.USELT;
        save.USECN = save.ATTBLK[CNVIDX];
        save.USESTL = save.ATTBLK[STLIDX];

        //
        // We announce a case here so we can locate errors in the
        // setup, if any.
        //
        fstr::assign(&mut save.TITLE, b"Cartesian product setup case number #");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", CASE, &mut save.TITLE, ctx);

        testutil::TCASE(&save.TITLE, ctx)?;

        //
        // Get the radii of the target body.
        //
        spicelib::BODVRD(
            &save.TARGET,
            b"RADII",
            3,
            &mut save.NRAD,
            save.RADII.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Set time.
        //
        save.ET = save.ET0;

        //
        // Units of AZ and EL are radians.
        //
        save.AZDELT = (spicelib::TWOPI(ctx) / NAZ as f64);

        //
        // Generate limb points for the current target, observer,
        // aberration correction, set of azimuth values, and time.
        //
        save.SCHSTP = 0.0;
        save.SOLTOL = 0.0;
        save.MAXN = NAZ;

        spicelib::VPACK(0.0, 0.0, 1.0, save.REFVEC.as_slice_mut());

        spicelib::LIMBPT(
            b"TANGENT/ELLIPSOID",
            &save.TARGET,
            save.ET,
            &save.FIXREF,
            &save.ABCORR,
            b"ELLIPSOID LIMB",
            &save.OBSRVR,
            save.REFVEC.as_slice(),
            save.AZDELT,
            NAZ,
            save.SCHSTP,
            save.SOLTOL,
            save.MAXN,
            save.NPTS.as_slice_mut(),
            save.POINTS.as_slice_mut(),
            save.EPOCHS.as_slice_mut(),
            save.TANGTS.as_slice_mut(),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        for AZIX in 1..=NAZ {
            save.AZ = (save.AZDELT * AZIX as f64);
            //
            // Compute the outward normal at the current limb point.
            //
            spicelib::SURFNM(
                save.RADII[1],
                save.RADII[2],
                save.RADII[3],
                save.POINTS.subarray([1, AZIX]),
                save.LNORML.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Create a rotation axis about which to rotate the
            // tangent vector to create a pointing direction.
            //
            spicelib::UCRSS(
                save.TANGTS.subarray([1, AZIX]),
                save.LNORML.as_slice(),
                save.CP.as_slice_mut(),
            );

            //
            // Loop over elevation values.
            //
            for ELIX in 1..=NEL {
                save.EL = save.ELEVS[ELIX];

                fstr::assign(&mut save.TITLE, b"Target: #; Observer: #; Body-fixed frame: #; Correction #; Locus: #; Ray frame: #; Az (deg): #; El (deg): #; Shape type: #; Time (TDB): #");

                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.OBSRVR, &mut save.TITLE);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.FIXREF, &mut save.TITLE);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CORLOC, &mut save.TITLE);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.RAYFRM, &mut save.TITLE);
                spicelib::REPMD(
                    &save.TITLE.to_vec(),
                    b"#",
                    (save.AZ * spicelib::DPR(ctx)),
                    9,
                    &mut save.TITLE,
                    ctx,
                );
                spicelib::REPMD(
                    &save.TITLE.to_vec(),
                    b"#",
                    (save.EL * spicelib::DPR(ctx)),
                    9,
                    &mut save.TITLE,
                    ctx,
                );
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.SHPNAM, &mut save.TITLE);
                spicelib::REPMD(&save.TITLE.to_vec(), b"#", save.ET, 9, &mut save.TITLE, ctx);

                //
                // --- Case: ------------------------------------------------------
                //
                testutil::TCASE(&save.TITLE, ctx)?;

                //
                // Use the limb point corresponding to the current AZ,
                // and the current EL, to generate a ray in the target
                // body-fixed frame, evaluated at the epoch of the
                // limb point for this AZ value.
                //
                // We rotate the vector from the observer to the current
                // limb point upward (away from the target) by the
                // elevation angle.
                //
                spicelib::VROTV(
                    save.TANGTS.subarray([1, AZIX]),
                    save.CP.as_slice(),
                    save.EL,
                    save.FIXDIR.as_slice_mut(),
                );

                //
                // Transform the ray's direction from the target
                // body-fixed frame to the specified ray frame. To
                // obtain a meaningful evaluation epoch for the ray
                // frame, compute the light time to the frame's center if
                // the frame is non-inertial.
                //
                if fstr::eq(&save.RAYFRM, b"J2000") {
                    save.RAYEPC = save.ET;
                } else {
                    spicelib::NAMFRM(&save.RAYFRM, &mut save.RAYFID, ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::FRINFO(
                        save.RAYFID,
                        &mut save.RFCENT,
                        &mut save.RFCLAS,
                        &mut save.RFCLID,
                        &mut save.FOUND,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

                    spicelib::BODC2S(save.RFCENT, &mut save.RFCNAM, ctx)?;
                    spicelib::SPKPOS(
                        &save.RFCNAM,
                        save.ET,
                        b"J2000",
                        &save.ABCORR,
                        &save.OBSRVR,
                        save.POS.as_slice_mut(),
                        &mut save.LT,
                        ctx,
                    )?;

                    spicelib::ZZCOREPC(&save.ABCORR, save.ET, save.LT, &mut save.RAYEPC, ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                //
                // Compute the transformation from the target frame at
                // the limb point epoch EPOCHS(AZIX) to the fray frame at
                // RAYEPC.
                //
                spicelib::PXFRM2(
                    &save.FIXREF,
                    &save.RAYFRM,
                    save.EPOCHS[AZIX],
                    save.RAYEPC,
                    save.RFMAT.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::MXV(
                    save.RFMAT.as_slice(),
                    save.FIXDIR.as_slice(),
                    save.RAYDIR.as_slice_mut(),
                );

                //
                // Get the tangent point and related outputs.
                //
                spicelib::TANGPT(
                    b"ELLIPSOID",
                    &save.TARGET,
                    save.ET,
                    &save.FIXREF,
                    &save.ABCORR,
                    &save.CORLOC,
                    &save.OBSRVR,
                    &save.RAYFRM,
                    save.RAYDIR.as_slice(),
                    save.TANPT.as_slice_mut(),
                    &mut save.ALT,
                    &mut save.RANGE,
                    save.SRFPT.as_slice_mut(),
                    &mut save.TRGEPC,
                    save.SRFVEC.as_slice_mut(),
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;
                //
                // Let STLOBS be the position of the observer relative to
                // the target center in the target body-fixed frame
                // evaluated at TRGEPC. STLOBS reflects stellar aberration
                // as well if those corrections are used.
                //
                // STLOBS will be used in several tests below.
                //
                spicelib::VSUB(
                    save.SRFPT.as_slice(),
                    save.SRFVEC.as_slice(),
                    save.STLOBS.as_slice_mut(),
                );

                //
                // Perform consistency check using NPEDLN for cases where
                // the tangent point doesn't coincide with the observer or
                // surface point.
                //
                if ((save.ALT > 0.0) && (save.RANGE != 0.0)) {
                    //
                    // This is a normal geometric case.
                    //
                    // Check consistency of outputs. Using the target epoch,
                    // surface point, and observer-to-surface vector,
                    // compute the ray direction in the body-fixed frame and
                    // re-compute the surface point and tangent point.
                    //
                    // Transform the input ray to the target body-fixed
                    // frame at trgepc.
                    //
                    spicelib::PXFRM2(
                        &save.RAYFRM,
                        &save.FIXREF,
                        save.RAYEPC,
                        save.TRGEPC,
                        save.XFORM.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::MXV(
                        save.XFORM.as_slice(),
                        save.RAYDIR.as_slice(),
                        save.FIXDIR.as_slice_mut(),
                    );

                    spicelib::NPEDLN(
                        save.RADII[1],
                        save.RADII[2],
                        save.RADII[3],
                        save.STLOBS.as_slice(),
                        save.FIXDIR.as_slice(),
                        save.XSRFPT.as_slice_mut(),
                        &mut save.XALT,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Check SRFPT.
                    //
                    if (save.RANGE < 10000000.0) {
                        if save.GEOM {
                            save.TOL = 0.00000000001;
                        } else if save.USECN {
                            //
                            // We're using converged light time corrections.
                            //
                            if !save.USESTL {
                                save.TOL = TIGHT;
                            } else {
                                save.TOL = 0.00000000001;
                            }
                        } else {
                            //
                            // Light time correction is not converged.
                            //
                            save.TOL = 0.00000000005;
                        }
                    } else {
                        //
                        // Range to the tangent point is over 1e7 km.
                        //
                        if save.GEOM {
                            save.TOL = 0.00000000005;
                        } else if save.USECN {
                            //
                            // We're using converged light time corrections.
                            //
                            if !save.USESTL {
                                save.TOL = 0.00000000005;
                            } else {
                                save.TOL = 0.00000000005;
                            }
                        } else {
                            //
                            // Light time correction is not converged.
                            //
                            save.TOL = 0.00000001;
                        }
                    }

                    testutil::CHCKAD(
                        b"SRFPT (npedln)",
                        save.SRFPT.as_slice(),
                        b"~~/",
                        save.XSRFPT.as_slice(),
                        3,
                        save.TOL,
                        OK,
                        ctx,
                    )?;

                    //
                    // Check TANPT. Compute the expected tangent point.
                    //
                    spicelib::NPLNPT(
                        save.STLOBS.as_slice(),
                        save.FIXDIR.as_slice(),
                        save.SRFPT.as_slice(),
                        save.XTANPT.as_slice_mut(),
                        &mut save.XALT,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    if save.GEOM {
                        if (save.RANGE < 10000000.0) {
                            save.TOL = VTIGHT;
                        } else {
                            save.TOL = 0.00000000005;
                        }
                    } else if save.USECN {
                        if (save.RANGE < 10000000.0) {
                            save.TOL = VTIGHT;
                        } else {
                            save.TOL = 0.0000000005;
                        }
                    } else {
                        //
                        // Light time correction is not converged.
                        //
                        if (save.RANGE < 10000000.0) {
                            save.TOL = 0.0000000001;
                        } else {
                            save.TOL = 0.000000001;
                        }
                    }

                    testutil::CHCKAD(
                        b"TANPT (npedln)",
                        save.TANPT.as_slice(),
                        b"~~/",
                        save.XTANPT.as_slice(),
                        3,
                        save.TOL,
                        OK,
                        ctx,
                    )?;
                    //
                    // Check altitude.
                    //
                    if (save.XALT > 100.0) {
                        //
                        // Check relative error.
                        //
                        if save.GEOM {
                            if (save.RANGE < 10000000.0) {
                                save.TOL = TIGHT;
                            } else {
                                save.TOL = 0.000000001;
                            }
                        } else if save.USECN {
                            //
                            // We're using converged light time corrections.
                            //
                            if (save.RANGE < 10000000.0) {
                                //
                                // Ray direction errors induce altitude errors
                                // that are roughly proportional to range.
                                // At closer range, we can use a tighter
                                // tolerance for our altitude checks.
                                //
                                save.TOL = TIGHT;
                            } else {
                                save.TOL = 0.0000000005;
                            }
                        } else {
                            //
                            // Light time correction is not converged.
                            //
                            save.TOL = 0.00000001;
                        }

                        testutil::CHCKSD(
                            b"ALT (npedln)",
                            save.ALT,
                            b"~/",
                            save.XALT,
                            save.TOL,
                            OK,
                            ctx,
                        )?;
                    } else {
                        //
                        // This is the low-altitude case.
                        //
                        // Check absolute error.
                        //
                        if save.GEOM {
                            save.TOL = 0.0000000001;
                        } else if save.USECN {
                            //
                            // We're using converged light time corrections.
                            //
                            save.TOL = 0.000000001;
                        } else {
                            //
                            // Light time correction is not converged.
                            //
                            save.TOL = 0.000000001;
                        }

                        testutil::CHCKSD(
                            b"ALT (npedln)",
                            save.ALT,
                            b"~",
                            save.XALT,
                            save.TOL,
                            OK,
                            ctx,
                        )?;
                    }

                    //
                    // Check range.
                    //
                    save.XRANGE = spicelib::VDIST(save.STLOBS.as_slice(), save.TANPT.as_slice());

                    if (save.XRANGE > 100.0) {
                        save.TOL = VTIGHT;

                        testutil::CHCKSD(
                            b"RANGE (npedln)",
                            save.RANGE,
                            b"~/",
                            save.XRANGE,
                            save.TOL,
                            OK,
                            ctx,
                        )?;
                    } else {
                        //
                        // The expected range is under 100 km. Use
                        // a 0.1 mm tolerance for absolute range error.

                        save.TOL = 0.0000001;
                        testutil::CHCKSD(
                            b"RANGE (npedln)",
                            save.RANGE,
                            b"~",
                            save.XRANGE,
                            save.TOL,
                            OK,
                            ctx,
                        )?;
                    }
                }

                //
                // This is the end of the NPEDLN checks, which are
                // performed only when range and altitude are both
                // non-zero.
                //
                // At this point, we need to decide whether the ray is
                // "looking away" from the target, in which case the
                // tangent point is supposed to be set equal to the
                // observer's location.
                //
                // At this point, we have the inputs needed to find the
                // sub-observer point on the apparent target. We'll use
                // this point later to decide whether or not we have a
                // "look away" ray direction.
                //
                // Note that, due to the highly non-spherical shapes of
                // targets, we can't rely on the angular separation between
                // the ray and the observer-target direction for this
                // determination. Instead we look at the angular separation
                // between the ray direction and the zenith direction at
                // the sub-observer point.
                //
                // The observer position STLOBS reflects the aberration
                // corrections we're using and target epoch used to
                // evaluate the orientation of the target body-fixed frame.
                //
                spicelib::NEARPT(
                    save.STLOBS.as_slice(),
                    save.RADII[1],
                    save.RADII[2],
                    save.RADII[3],
                    save.SUBP.as_slice_mut(),
                    &mut save.SUBALT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                //
                // Get the zenith direction at the sub-observer point.
                // We'll use this later.
                //
                spicelib::SURFNM(
                    save.RADII[1],
                    save.RADII[2],
                    save.RADII[3],
                    save.SUBP.as_slice(),
                    save.SUBZEN.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Let THETA be the angular offset of the ray direction
                // from the sub-observer point's zenith direction.
                //
                save.THETA = spicelib::VSEP(save.FIXDIR.as_slice(), save.SUBZEN.as_slice(), ctx);

                if (save.THETA < spicelib::HALFPI(ctx)) {
                    //
                    // The ray is pointing upward relative to the local
                    // level plane at the sub-observer point. This is a
                    // "look away" case.
                    //
                    // We expect:
                    //
                    //    - TRGEPC is exactly ET if the locus is the
                    //      tangent point; otherwise TRGEPC is derived from
                    //      the observer altitude.
                    //
                    //    - RANGE is exactly zero
                    //
                    //    - TANPT coincides with the observer's position
                    //      relative to the target center
                    //
                    //    - SRFPT is the near point on the target
                    //
                    //    - ALT is the altitude of the observer above the
                    //      target
                    //
                    //
                    // Find the nearest point on the target to TANPT.
                    //
                    spicelib::NEARPT(
                        save.TANPT.as_slice(),
                        save.RADII[1],
                        save.RADII[2],
                        save.RADII[3],
                        save.PNEAR.as_slice_mut(),
                        &mut save.XALT,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    if (save.LIX == 1) {
                        //
                        // The locus is the tangent point, which in this
                        // case coincides with the observer.
                        //
                        testutil::CHCKSD(b"TRGEPC A", save.TRGEPC, b"=", save.ET, 0.0, OK, ctx)?;
                    } else {
                        //
                        // The locus is the surface point, which in this
                        // case is the nearest point on the target to the
                        // observer.
                        //
                        spicelib::ZZCOREPC(
                            &save.ABCORR,
                            save.ET,
                            (save.XALT / spicelib::CLIGHT()),
                            &mut save.XEPOCH,
                            ctx,
                        )?;

                        save.TOL = TIGHT;

                        testutil::CHCKSD(
                            b"TRGEPC B",
                            save.TRGEPC,
                            b"~/",
                            save.XEPOCH,
                            save.TOL,
                            OK,
                            ctx,
                        )?;
                    }

                    //
                    // Check the range, which should be exactly 0.
                    //
                    testutil::CHCKSD(b"RANGE", save.RANGE, b"=", 0.0, 0.0, OK, ctx)?;

                    //
                    // Check the tangent point, which should be
                    // equal to the observer position.
                    //
                    save.TOL = VTIGHT;

                    testutil::CHCKAD(
                        b"TANPT",
                        save.TANPT.as_slice(),
                        b"~~/",
                        save.STLOBS.as_slice(),
                        3,
                        save.TOL,
                        OK,
                        ctx,
                    )?;

                    //
                    // Check the surface point, which should be
                    // equal to the near point.
                    //
                    save.TOL = VTIGHT;

                    testutil::CHCKAD(
                        b"SRFPT",
                        save.SRFPT.as_slice(),
                        b"~~/",
                        save.PNEAR.as_slice(),
                        3,
                        save.TOL,
                        OK,
                        ctx,
                    )?;

                    //
                    // Check the tangent point altitude, which should be
                    // equal to the altitude found by NEARPT.
                    //
                    save.TOL = VTIGHT;

                    testutil::CHCKSD(b"ALT", save.ALT, b"~/", save.XALT, save.TOL, OK, ctx)?;
                } else if (save.EL <= 0.0) {
                    //
                    // This is an intercept case.
                    //
                    // We expect:
                    //
                    //    - ALT is exactly zero
                    //
                    //    - TANPT is exactly SRFPT
                    //
                    //    - RANGE is exactly ||SRFVEC||
                    //
                    //    - TRGEPC is ET +/- RANGE / c
                    //
                    testutil::CHCKSD(b"ALT", save.ALT, b"=", 0.0, 0.0, OK, ctx)?;

                    testutil::CHCKAD(
                        b"TANPT",
                        save.TANPT.as_slice(),
                        b"=",
                        save.SRFPT.as_slice(),
                        3,
                        save.TOL,
                        OK,
                        ctx,
                    )?;

                    save.XRANGE = spicelib::VNORM(save.SRFVEC.as_slice());

                    save.TOL = 0.0;

                    testutil::CHCKSD(b"RANGE", save.RANGE, b"=", save.XRANGE, save.TOL, OK, ctx)?;

                    spicelib::ZZCOREPC(
                        &save.ABCORR,
                        save.ET,
                        (save.RANGE / spicelib::CLIGHT()),
                        &mut save.XEPOCH,
                        ctx,
                    )?;

                    save.TOL = TIGHT;

                    testutil::CHCKSD(
                        b"TRGEPC",
                        save.TRGEPC,
                        b"~/",
                        save.XEPOCH,
                        save.TOL,
                        OK,
                        ctx,
                    )?;

                    if fstr::eq(&save.CORLOC, b"SURFACE POINT") {
                        //
                        // Compare the surface point against that produced
                        // by SINCPT. We can expect good agreement if
                        // we're not using aberration corrections. The
                        // instability of near-limb ray intercepts requires
                        // large tolerances when converged Newtonian light
                        // time corrections are used. We don't attempt
                        // comparisons when stellar aberration corrections
                        // or non-converged light time corrections are used.
                        //
                        spicelib::SINCPT(
                            b"ELLIPSOID",
                            &save.TARGET,
                            save.ET,
                            &save.FIXREF,
                            &save.ABCORR,
                            &save.OBSRVR,
                            &save.RAYFRM,
                            save.RAYDIR.as_slice(),
                            save.XPT.as_slice_mut(),
                            &mut save.XPTEPC,
                            save.XPTVEC.as_slice_mut(),
                            &mut save.XPTFND,
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        if (!save.USELT || save.USECN) {
                            //
                            // Verify that an intercept was found.
                            //
                            testutil::CHCKSL(b"XPTFND", save.XPTFND, true, OK, ctx)?;

                            if save.XPTFND {
                                //
                                // Check the target epoch. Use 0.1 microsecond
                                // time tolerance.
                                //
                                save.TOL = 0.0000001;

                                testutil::CHCKSD(
                                    b"TRGEPC",
                                    save.TRGEPC,
                                    b"~",
                                    save.XPTEPC,
                                    save.TOL,
                                    OK,
                                    ctx,
                                )?;

                                //
                                // Check the intercept.
                                //
                                if !save.USELT {
                                    //
                                    // Look for agreement at the 1 mm level.
                                    //
                                    save.TOL = 0.000001;
                                } else {
                                    //
                                    // Allow errors of up to 5 m.
                                    //
                                    save.TOL = 5.0;
                                }

                                testutil::CHCKAD(
                                    b"SRFPT",
                                    save.SRFPT.as_slice(),
                                    b"~~",
                                    save.XPT.as_slice(),
                                    3,
                                    save.TOL,
                                    OK,
                                    ctx,
                                )?;
                            }
                        }
                    }
                } else {
                    //
                    // Checks for all normal cases follow.
                    //
                    // Check target epoch.
                    //
                    if fstr::eq(&save.CORLOC, b"TANGENT POINT") {
                        save.XLT = (save.RANGE / spicelib::CLIGHT());
                    } else {
                        save.XLT = (spicelib::VNORM(save.SRFVEC.as_slice()) / spicelib::CLIGHT());
                    }

                    spicelib::ZZCOREPC(&save.ABCORR, save.ET, save.XLT, &mut save.XEPOCH, ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Check the target epoch. Use 0.1 microsecond
                    // time tolerance.
                    //
                    save.TOL = 0.0000001;
                    testutil::CHCKSD(b"TRGEPC", save.TRGEPC, b"~", save.XEPOCH, save.TOL, OK, ctx)?;

                    //
                    // Find the nearest point on the target to TANPT.
                    //
                    spicelib::NEARPT(
                        save.TANPT.as_slice(),
                        save.RADII[1],
                        save.RADII[2],
                        save.RADII[3],
                        save.PNEAR.as_slice_mut(),
                        &mut save.XALT,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Check altitude of the tangent point. The altitude will
                    // be zero for intercept cases, but no special test is
                    // required for those cases.
                    //
                    if (save.XALT > 100.0) {
                        //
                        // Check relative error.
                        //
                        save.TOL = 0.000000001;
                        testutil::CHCKSD(
                            b"ALT (A)", save.ALT, b"~/", save.XALT, save.TOL, OK, ctx,
                        )?;
                    } else {
                        //
                        // Check absolute error.
                        //
                        save.TOL = 0.000000001;
                        testutil::CHCKSD(b"ALT (B)", save.ALT, b"~", save.XALT, save.TOL, OK, ctx)?;
                    }

                    //
                    // Check range to the tangent point.
                    //
                    save.XRANGE = spicelib::VDIST(save.STLOBS.as_slice(), save.TANPT.as_slice());

                    if (save.XRANGE > 100.0) {
                        //
                        // This is the higher altitude case. Check relative
                        // error.
                        //
                        save.TOL = TIGHT;
                        testutil::CHCKSD(
                            b"RANGE",
                            save.RANGE,
                            b"~/",
                            save.XRANGE,
                            save.TOL,
                            OK,
                            ctx,
                        )?;
                    } else {
                        //
                        // This is the lower altitude case. Check absolute
                        // error. We can expect agreement at the 1 micron
                        // level.
                        //
                        save.TOL = 0.000000001;
                        testutil::CHCKSD(
                            b"RANGE",
                            save.RANGE,
                            b"~",
                            save.XRANGE,
                            save.TOL,
                            OK,
                            ctx,
                        )?;
                    }

                    //
                    // Compare SRFPT to the surface point found by NEARPT.
                    //
                    // Check SRFPT absolute error.
                    //
                    if (save.RANGE < 10000000.0) {
                        //
                        // Observer and target are less than 10 M km apart.
                        //
                        save.TOL = 0.0000000001;
                    } else {
                        save.TOL = 0.0000005;
                    }

                    testutil::CHCKAD(
                        b"SRFPT (abs)",
                        save.SRFPT.as_slice(),
                        b"~~",
                        save.PNEAR.as_slice(),
                        3,
                        save.TOL,
                        OK,
                        ctx,
                    )?;

                    //
                    // Check SRFPT relative error.
                    //
                    save.TOL = 0.0000000001;

                    testutil::CHCKAD(
                        b"SRFPT (rel)",
                        save.SRFPT.as_slice(),
                        b"~~/",
                        save.PNEAR.as_slice(),
                        3,
                        save.TOL,
                        OK,
                        ctx,
                    )?;

                    //
                    // Check the position of the tangent point above the
                    // surface point.
                    //
                    spicelib::SURFNM(
                        save.RADII[1],
                        save.RADII[2],
                        save.RADII[3],
                        save.SRFPT.as_slice(),
                        save.NORMAL.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::VSUB(
                        save.TANPT.as_slice(),
                        save.SRFPT.as_slice(),
                        save.SRFTAN.as_slice_mut(),
                    );

                    save.SEP = spicelib::VSEP(save.SRFTAN.as_slice(), save.NORMAL.as_slice(), ctx);

                    if ((save.EL > 0.0) && (save.EL < spicelib::HALFPI(ctx))) {
                        //
                        // Check angular separation of zenith direction at
                        // SRFPT vs the SRFPT-to-TANPT vector.
                        //
                        // Use looser tolerance here since the relative
                        // errors in TANPT and SRFPT are larger compared to
                        // the lengths of those vectors than they are
                        // relative to the observer-target distance.
                        //
                        save.TOL = intrinsics::DMAX1(&[0.0000000001, (0.000001 / save.ALT)]);

                        testutil::CHCKSD(
                            b"SRFPT-TANPT ZENITH SEP",
                            save.SEP,
                            b"~",
                            0.0,
                            save.TOL,
                            OK,
                            ctx,
                        )?;
                        //
                        // Check ALT.
                        //
                        save.XALT = spicelib::VDIST(save.SRFPT.as_slice(), save.TANPT.as_slice());

                        if (save.XALT > 100.0) {
                            save.TOL = (VTIGHT * intrinsics::DMAX1(&[save.RANGE, 1.0]));

                            testutil::CHCKSD(
                                b"ALT (C)", save.ALT, b"~/", save.XALT, save.TOL, OK, ctx,
                            )?;
                        } else {
                            save.TOL = 0.000000001;

                            testutil::CHCKSD(
                                b"ALT (C)", save.ALT, b"~", save.XALT, save.TOL, OK, ctx,
                            )?;
                        }
                    }

                    if (fstr::eq(&save.CORLOC, b"TANGENT POINT") && (save.RANGE != 0 as f64)) {
                        //
                        // We'll treat the tangent point as an ephemeris
                        // object. We'll find the angular separation between
                        // the input ray and the ray from the observer to the
                        // tangent point.
                        //
                        // Find the aberration-corrected position of the
                        // tangent point relative to the observer, expressed
                        // in the J2000 frame. Convert this vector to the ray
                        // frame.
                        //
                        spicelib::SPKCPT(
                            save.TANPT.as_slice(),
                            &save.TARGET,
                            &save.FIXREF,
                            save.ET,
                            b"J2000",
                            b"TARGET",
                            &save.ABCORR,
                            &save.OBSRVR,
                            save.TANSTA.as_slice_mut(),
                            &mut save.LT,
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        spicelib::PXFORM(
                            b"J2000",
                            &save.RAYFRM,
                            save.RAYEPC,
                            save.RAYMAT.as_slice_mut(),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        spicelib::MXV(
                            save.RAYMAT.as_slice(),
                            save.TANSTA.as_slice(),
                            save.XRAYDR.as_slice_mut(),
                        );
                        //
                        // Check angular separation of ray direction and
                        // direction to tangent point.
                        //
                        save.SEP =
                            spicelib::VSEP(save.RAYDIR.as_slice(), save.XRAYDR.as_slice(), ctx);

                        if (save.EL < 0.2) {
                            //
                            // These are the low angular separation cases.
                            //
                            if ((save.RANGE > 0.0) && (save.ALT > 0.0)) {
                                if (save.RANGE < 10000000.0) {
                                    if save.USECN {
                                        if save.USESTL {
                                            save.TOL = 0.0000000005;
                                        } else {
                                            save.TOL = 0.0000000001;
                                        }
                                    } else if save.USELT {
                                        //
                                        // We expect very loose agreement with
                                        // non-converged light time.
                                        //
                                        save.TOL = 0.0000001;
                                    } else {
                                        //
                                        // This is the geometric case.
                                        //
                                        save.TOL = VTIGHT;
                                    }
                                } else {
                                    //
                                    // These are the long-range cases.
                                    //
                                    if save.USECN {
                                        save.TOL = 0.0000000001;
                                    } else if save.USELT {
                                        //
                                        // We expect very loose agreement with
                                        // non-converged light time.
                                        //
                                        save.TOL = 0.0001;
                                    } else {
                                        //
                                        // This is the geometric case.
                                        //
                                        save.TOL = VTIGHT;
                                    }
                                }
                            } else {
                                //
                                // These are special cases: the tangent point
                                // coincides with the observer or the surface
                                // intercept.
                                //
                                if save.USELT {
                                    save.TOL = 0.0000000001;
                                } else {
                                    save.TOL = 0.0000000001;
                                }
                            }
                        } else {
                            //
                            // These are the high angular separation cases.
                            //
                            if (save.RANGE > 10000000.0) {
                                //
                                // Large range implies high altitude, in this
                                // case.
                                //
                                // Note that a high-altitude tangent point,
                                // where Mars is the central body, has very
                                // high velocity.
                                //
                                if save.USECN {
                                    save.TOL = 0.0000001;
                                } else if save.USELT {
                                    save.TOL = 0.01;
                                } else {
                                    save.TOL = 0.0000001;
                                }
                            } else {
                                if save.USECN {
                                    save.TOL = 0.0000005;
                                } else if save.USELT {
                                    save.TOL = 0.001;
                                } else {
                                    //
                                    // Geometric case for high angular
                                    // separation.
                                    //
                                    save.TOL = 0.00000001;
                                }
                            }
                        }

                        testutil::CHCKSD(
                            b"RAY-OBS-TO-TANPT SEP",
                            save.SEP,
                            b"~",
                            0.0,
                            save.TOL,
                            OK,
                            ctx,
                        )?;

                        //
                        // Check range against the magnitude of the
                        // position of the tangent point relative to
                        // the observer.
                        //
                        save.XRANGE = spicelib::VNORM(save.TANSTA.as_slice());

                        testutil::CHCKSD(
                            b"RANGE (vs TANSTA)",
                            save.RANGE,
                            b"~/",
                            save.XRANGE,
                            save.TOL,
                            OK,
                            ctx,
                        )?;

                    //
                    // End of TANGENT POINT locus case.
                    //
                    } else if (fstr::eq(&save.CORLOC, b"SURFACE POINT") && (save.ALT != 0 as f64)) {
                        //
                        // The locus is the surface point.
                        //
                        // We'll treat the surface point as an ephemeris
                        // object. We'll find the angular separation between
                        // the ray found by TANGPT from the observer to the
                        // surface point and the position found by SPKCPT of
                        // the surface point relative to the observer.
                        //
                        // Find the aberration-corrected position of the
                        // surface point relative to the observer, expressed
                        // in the target body-fixed frame.
                        //
                        spicelib::SPKCPT(
                            save.SRFPT.as_slice(),
                            &save.TARGET,
                            &save.FIXREF,
                            save.ET,
                            &save.FIXREF,
                            b"TARGET",
                            &save.ABCORR,
                            &save.OBSRVR,
                            save.SRFSTA.as_slice_mut(),
                            &mut save.LT,
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        spicelib::VEQU(save.SRFSTA.as_slice(), save.XSRFVC.as_slice_mut());

                        save.SEP =
                            spicelib::VSEP(save.SRFVEC.as_slice(), save.XSRFVC.as_slice(), ctx);

                        if (save.EL < 0.2) {
                            //
                            // These are the low angular separation cases.
                            //
                            if !save.USELT {
                                save.TOL = 0.0000000001;
                            } else if save.USECN {
                                if save.USESTL {
                                    save.TOL = 0.0000000005;
                                } else {
                                    save.TOL = 0.0000000005;
                                }
                            } else {
                                save.TOL = 0.0000001;
                            }
                        } else {
                            save.TOL = 0.0000001;
                        }

                        testutil::CHCKSD(b"SRFPT SEP", save.SEP, b"~", 0.0, save.TOL, OK, ctx)?;

                        //
                        // Check the surface vector as well. We can use
                        // the same tolerances.
                        //
                        testutil::CHCKAD(
                            b"SRFVEC",
                            save.SRFVEC.as_slice(),
                            b"~~/",
                            save.XSRFVC.as_slice(),
                            3,
                            save.TOL,
                            OK,
                            ctx,
                        )?;
                    }
                    //
                    // This is the end of the normal case check block.
                    //
                }
            }
            //
            // This is the end of the elevation loop.
            //
        }
        //
        // This is the end of the azimuth loop.
        //
    }
    //
    // This is the end of the case loop.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up.", ctx)?;

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if spicelib::EXISTS(PCK0, ctx)? {
        spicelib::DELFIL(PCK0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    if spicelib::EXISTS(PCK1, ctx)? {
        spicelib::DELFIL(PCK1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    if spicelib::EXISTS(SPK0, ctx)? {
        spicelib::SPKUEF(save.HAN0, ctx)?;
        spicelib::DELFIL(SPK0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    if spicelib::EXISTS(SPK1, ctx)? {
        spicelib::SPKUEF(save.HAN1, ctx)?;
        spicelib::DELFIL(SPK1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
