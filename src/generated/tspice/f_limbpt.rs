//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXSRF: i32 = 100;
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
const DSK0: &[u8] = b"limbpt_dsk0.bds";
const DSK1: &[u8] = b"limbpt_dsk1.bds";
const DSK2: &[u8] = b"limbpt_dsk2.bds";
const DSK3: &[u8] = b"limbpt_dsk3.bds";
const DSK4: &[u8] = b"limbpt_dsk4.bds";
const PCK0: &[u8] = b"test_0008.tpc";
const PCK1: &[u8] = b"nat.tpc";
const SPK1: &[u8] = b"limbpt_spk.bsp";
const SPK2: &[u8] = b"orbiter.bsp";
const SPK3: &[u8] = b"nat.bsp";
const VTIGHT: f64 = 0.00000000000001;
const TIGHT: f64 = 0.000000000001;
const MTIGHT: f64 = 0.0000000001;
const LOOSE: f64 = 0.000005;
const ALPHA: i32 = 1000;
const LNSIZE: i32 = 320;
const NAMLEN: i32 = 32;
const NREF: i32 = 1;
const NABC: i32 = 9;
const ABCLEN: i32 = 10;
const TIMLEN: i32 = 50;
const NOBS: i32 = 2;
const NTARG: i32 = 2;
const NTIMES: i32 = 5;
const SCID: i32 = -499;
const NCROSS: i32 = 400;
const NMAP: i32 = 4;
const NMETH: i32 = 7;
const NPOLYV: i32 = 100;
const NTANGT: i32 = 4;
const NLOC: i32 = 2;
const MAXCUT: i32 = 1000;
const MAXPNT: i32 = (MAXCUT * 20);
const UBEL: i32 = 9;
const UBPL: i32 = 4;

struct SaveVars {
    ABCS: ActualCharArray,
    ABCORR: Vec<u8>,
    CORLCS: ActualCharArray,
    CORLOC: Vec<u8>,
    FIXREF: Vec<u8>,
    LABEL: Vec<u8>,
    METHD2: Vec<u8>,
    METHOD: Vec<u8>,
    METHDS: ActualCharArray,
    OBSRVR: Vec<u8>,
    OBSNMS: ActualCharArray,
    REFS: ActualCharArray2D,
    SPKLOC: Vec<u8>,
    SRFNMS: ActualCharArray,
    TARGET: Vec<u8>,
    TRGFRM: Vec<u8>,
    TRGNMS: ActualCharArray,
    TITLE: Vec<u8>,
    UTC: Vec<u8>,
    ALT: f64,
    AXIS: StackArray<f64, 3>,
    BADRAD: StackArray<f64, 3>,
    CENTER: StackArray<f64, 3>,
    CUTNML: StackArray<f64, 3>,
    D: f64,
    DIST: f64,
    ELTS: StackArray<f64, 8>,
    EPOCHS: ActualArray<f64>,
    EPNTS: ActualArray2D<f64>,
    ET: f64,
    ET0: f64,
    ETNGTS: ActualArray2D<f64>,
    HTOL: f64,
    ISTATE: StackArray<f64, 6>,
    ITANVC: StackArray<f64, 3>,
    LIMB: StackArray<f64, 9>,
    LPLANE: StackArray<f64, 4>,
    LPOINT: StackArray<f64, 3>,
    LPROJ: StackArray<f64, 3>,
    LPTEPC: f64,
    LT: f64,
    NORMAL: StackArray<f64, 3>,
    PLNVEC: StackArray<f64, 3>,
    PMCOEF: StackArray<f64, 3>,
    PNEAR: StackArray<f64, 3>,
    POINTS: ActualArray2D<f64>,
    R: f64,
    RADII: StackArray<f64, 3>,
    RAYDIR: StackArray<f64, 3>,
    RCROSS: f64,
    REFVEC: StackArray<f64, 3>,
    ROLSTP: f64,
    SCHSTP: f64,
    SMAJOR: StackArray<f64, 3>,
    SMINOR: StackArray<f64, 3>,
    SOLTOL: f64,
    STATE: StackArray<f64, 6>,
    STATE0: StackArray<f64, 6>,
    TANGTS: ActualArray2D<f64>,
    TANVEC: StackArray<f64, 3>,
    TDELTA: f64,
    THETA: f64,
    TOL: f64,
    TRGLT: f64,
    TRGPOS: StackArray<f64, 3>,
    UTAN: StackArray<f64, 3>,
    VERTEX: StackArray<f64, 3>,
    XEPOCH: ActualArray<f64>,
    XFORM: StackArray2D<f64, 9>,
    XISRFV: StackArray<f64, 3>,
    XPOINT: ActualArray2D<f64>,
    XPT: StackArray<f64, 3>,
    XSRFVC: StackArray<f64, 3>,
    XTE: f64,
    BODYID: i32,
    HANDLE: StackArray<i32, 3>,
    K: i32,
    MAXN: i32,
    N: i32,
    NCUTS: i32,
    NLAT: i32,
    NLON: i32,
    NPTS: ActualArray<i32>,
    NSURF: i32,
    OBSCDE: i32,
    SRFBOD: StackArray<i32, 4>,
    SRFIDS: StackArray<i32, 4>,
    SURFID: i32,
    TRGCDE: i32,
    UB: i32,
    ATTBLK: StackArray<bool, 6>,
    FOUND: bool,
    ISDSK: bool,
    ISELL: bool,
    ISHI: bool,
    ISTAN: bool,
    USECN: bool,
    USELT: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ABCS = ActualCharArray::new(ABCLEN, 1..=NABC);
        let mut ABCORR = vec![b' '; ABCLEN as usize];
        let mut CORLCS = ActualCharArray::new(NAMLEN, 1..=NLOC);
        let mut CORLOC = vec![b' '; NAMLEN as usize];
        let mut FIXREF = vec![b' '; NAMLEN as usize];
        let mut LABEL = vec![b' '; NAMLEN as usize];
        let mut METHD2 = vec![b' '; MTHLEN as usize];
        let mut METHOD = vec![b' '; MTHLEN as usize];
        let mut METHDS = ActualCharArray::new(MTHLEN, 1..=NMETH);
        let mut OBSRVR = vec![b' '; NAMLEN as usize];
        let mut OBSNMS = ActualCharArray::new(NAMLEN, 1..=NOBS);
        let mut REFS = ActualCharArray2D::new(NAMLEN, 1..=NREF, 1..=NTARG);
        let mut SPKLOC = vec![b' '; NAMLEN as usize];
        let mut SRFNMS = ActualCharArray::new(NAMLEN, 1..=NMAP);
        let mut TARGET = vec![b' '; NAMLEN as usize];
        let mut TRGFRM = vec![b' '; NAMLEN as usize];
        let mut TRGNMS = ActualCharArray::new(NAMLEN, 1..=NTARG);
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut UTC = vec![b' '; TIMLEN as usize];
        let mut ALT: f64 = 0.0;
        let mut AXIS = StackArray::<f64, 3>::new(1..=3);
        let mut BADRAD = StackArray::<f64, 3>::new(1..=3);
        let mut CENTER = StackArray::<f64, 3>::new(1..=3);
        let mut CUTNML = StackArray::<f64, 3>::new(1..=3);
        let mut D: f64 = 0.0;
        let mut DIST: f64 = 0.0;
        let mut ELTS = StackArray::<f64, 8>::new(1..=8);
        let mut EPOCHS = ActualArray::<f64>::new(1..=MAXCUT);
        let mut EPNTS = ActualArray2D::<f64>::new(1..=3, 1..=MAXPNT);
        let mut ET: f64 = 0.0;
        let mut ET0: f64 = 0.0;
        let mut ETNGTS = ActualArray2D::<f64>::new(1..=3, 1..=MAXPNT);
        let mut HTOL: f64 = 0.0;
        let mut ISTATE = StackArray::<f64, 6>::new(1..=6);
        let mut ITANVC = StackArray::<f64, 3>::new(1..=3);
        let mut LIMB = StackArray::<f64, 9>::new(1..=UBEL);
        let mut LPLANE = StackArray::<f64, 4>::new(1..=UBPL);
        let mut LPOINT = StackArray::<f64, 3>::new(1..=3);
        let mut LPROJ = StackArray::<f64, 3>::new(1..=3);
        let mut LPTEPC: f64 = 0.0;
        let mut LT: f64 = 0.0;
        let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
        let mut PLNVEC = StackArray::<f64, 3>::new(1..=3);
        let mut PMCOEF = StackArray::<f64, 3>::new(1..=3);
        let mut PNEAR = StackArray::<f64, 3>::new(1..=3);
        let mut POINTS = ActualArray2D::<f64>::new(1..=3, 1..=MAXPNT);
        let mut R: f64 = 0.0;
        let mut RADII = StackArray::<f64, 3>::new(1..=3);
        let mut RAYDIR = StackArray::<f64, 3>::new(1..=3);
        let mut RCROSS: f64 = 0.0;
        let mut REFVEC = StackArray::<f64, 3>::new(1..=3);
        let mut ROLSTP: f64 = 0.0;
        let mut SCHSTP: f64 = 0.0;
        let mut SMAJOR = StackArray::<f64, 3>::new(1..=3);
        let mut SMINOR = StackArray::<f64, 3>::new(1..=3);
        let mut SOLTOL: f64 = 0.0;
        let mut STATE = StackArray::<f64, 6>::new(1..=6);
        let mut STATE0 = StackArray::<f64, 6>::new(1..=6);
        let mut TANGTS = ActualArray2D::<f64>::new(1..=3, 1..=MAXPNT);
        let mut TANVEC = StackArray::<f64, 3>::new(1..=3);
        let mut TDELTA: f64 = 0.0;
        let mut THETA: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut TRGLT: f64 = 0.0;
        let mut TRGPOS = StackArray::<f64, 3>::new(1..=3);
        let mut UTAN = StackArray::<f64, 3>::new(1..=3);
        let mut VERTEX = StackArray::<f64, 3>::new(1..=3);
        let mut XEPOCH = ActualArray::<f64>::new(1..=MAXPNT);
        let mut XFORM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut XISRFV = StackArray::<f64, 3>::new(1..=3);
        let mut XPOINT = ActualArray2D::<f64>::new(1..=3, 1..=MAXPNT);
        let mut XPT = StackArray::<f64, 3>::new(1..=3);
        let mut XSRFVC = StackArray::<f64, 3>::new(1..=3);
        let mut XTE: f64 = 0.0;
        let mut BODYID: i32 = 0;
        let mut HANDLE = StackArray::<i32, 3>::new(1..=3);
        let mut K: i32 = 0;
        let mut MAXN: i32 = 0;
        let mut N: i32 = 0;
        let mut NCUTS: i32 = 0;
        let mut NLAT: i32 = 0;
        let mut NLON: i32 = 0;
        let mut NPTS = ActualArray::<i32>::new(1..=MAXCUT);
        let mut NSURF: i32 = 0;
        let mut OBSCDE: i32 = 0;
        let mut SRFBOD = StackArray::<i32, 4>::new(1..=NMAP);
        let mut SRFIDS = StackArray::<i32, 4>::new(1..=NMAP);
        let mut SURFID: i32 = 0;
        let mut TRGCDE: i32 = 0;
        let mut UB: i32 = 0;
        let mut ATTBLK = StackArray::<bool, 6>::new(1..=ABATSZ);
        let mut FOUND: bool = false;
        let mut ISDSK: bool = false;
        let mut ISELL: bool = false;
        let mut ISHI: bool = false;
        let mut ISTAN: bool = false;
        let mut USECN: bool = false;
        let mut USELT: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"None"),
                Val::C(b"Cn"),
                Val::C(b"Cn+s"),
                Val::C(b"Lt"),
                Val::C(b"Lt+s"),
                Val::C(b"XCN"),
                Val::C(b"xcn+S"),
                Val::C(b"lT"),
                Val::C(b"LT+S"),
            ]
            .into_iter();
            ABCS.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"ELLIPSOID LIMB"), Val::C(b"CENTER")].into_iter();
            CORLCS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"IAU_MARS"), Val::C(b"IAU_PHOBOS")].into_iter();
            REFS.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"Earth"), Val::C(b"MARS_ORBITER")].into_iter();
            OBSNMS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"Mars"), Val::C(b"PHOBOS")].into_iter();
            TRGNMS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"ELLIPSOID / tangent"),
                Val::C(b"tangent/ellipsoid"),
                Val::C(b"tangent/dsk/unprioritized/surfaces=\"high-res\""),
                Val::C(b"tangent/dsk/unprioritized/surfaces=\"LOW-RES\""),
                Val::C(b"guided/ellipsoid"),
                Val::C(b"dsk/ guided /unprioritized/surfaces=\"high-res\""),
                Val::C(b"dsk/ guided /unprioritized/surfaces=\"LOW-RES\""),
            ]
            .into_iter();
            METHDS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            ABCS,
            ABCORR,
            CORLCS,
            CORLOC,
            FIXREF,
            LABEL,
            METHD2,
            METHOD,
            METHDS,
            OBSRVR,
            OBSNMS,
            REFS,
            SPKLOC,
            SRFNMS,
            TARGET,
            TRGFRM,
            TRGNMS,
            TITLE,
            UTC,
            ALT,
            AXIS,
            BADRAD,
            CENTER,
            CUTNML,
            D,
            DIST,
            ELTS,
            EPOCHS,
            EPNTS,
            ET,
            ET0,
            ETNGTS,
            HTOL,
            ISTATE,
            ITANVC,
            LIMB,
            LPLANE,
            LPOINT,
            LPROJ,
            LPTEPC,
            LT,
            NORMAL,
            PLNVEC,
            PMCOEF,
            PNEAR,
            POINTS,
            R,
            RADII,
            RAYDIR,
            RCROSS,
            REFVEC,
            ROLSTP,
            SCHSTP,
            SMAJOR,
            SMINOR,
            SOLTOL,
            STATE,
            STATE0,
            TANGTS,
            TANVEC,
            TDELTA,
            THETA,
            TOL,
            TRGLT,
            TRGPOS,
            UTAN,
            VERTEX,
            XEPOCH,
            XFORM,
            XISRFV,
            XPOINT,
            XPT,
            XSRFVC,
            XTE,
            BODYID,
            HANDLE,
            K,
            MAXN,
            N,
            NCUTS,
            NLAT,
            NLON,
            NPTS,
            NSURF,
            OBSCDE,
            SRFBOD,
            SRFIDS,
            SURFID,
            TRGCDE,
            UB,
            ATTBLK,
            FOUND,
            ISDSK,
            ISELL,
            ISHI,
            ISTAN,
            USECN,
            USELT,
        }
    }
}

//$Procedure      F_LIMBPT ( LIMBPT family tests )
pub fn F_LIMBPT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // This array is two-dimensional. The second dimension
    // is indexed by the input METHOD. This makes it simpler
    // to control the combinations of methods and locus
    // values.
    //

    //
    // Note that the last two method strings are identical. This
    // is done to test the logic that uses saved values obtained
    // by parsing method string.
    //

    //
    // Begin every test family with an open call.q
    //
    testutil::TOPEN(b"F_LIMBPT", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Setup:  create SPK, PCK file.", ctx)?;

    if spicelib::EXISTS(SPK1, ctx)? {
        spicelib::DELFIL(SPK1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    testutil::TSTSPK(SPK1, true, &mut save.HANDLE[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create the PCK file, and load it. Do not delete it.
    //
    if spicelib::EXISTS(PCK0, ctx)? {
        spicelib::DELFIL(PCK0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    testutil::T_PCK08(PCK0, true, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Optionally scale Mars rotation.
    //
    spicelib::BODVRD(
        b"MARS",
        b"PM",
        3,
        &mut save.N,
        save.PMCOEF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Leave as is for delivery.
    //
    save.PMCOEF[2] = (save.PMCOEF[2] * 1 as f64);

    spicelib::PDPOOL(b"BODY499_PM", 3, save.PMCOEF.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create LSK, load it, and delete it.
    //
    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set initial time.
    //
    fstr::assign(&mut save.UTC, b"2004 FEB 17");
    spicelib::STR2ET(&save.UTC, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = save.ET0;

    //
    // Create a Mars orbiter SPK file.
    //
    if spicelib::EXISTS(SPK2, ctx)? {
        spicelib::DELFIL(SPK2, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::SPKOPN(SPK2, SPK2, 0, &mut save.HANDLE[2], ctx)?;
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
    save.ELTS[1] = 3800.0;
    save.ELTS[2] = 0.1;
    save.ELTS[3] = (80.0 * spicelib::RPD(ctx));
    save.ELTS[4] = 0.0;
    save.ELTS[5] = (90.0 * spicelib::RPD(ctx));
    save.ELTS[6] = 0.0;
    save.ELTS[7] = save.ET;
    save.ELTS[8] = 42828.314;

    spicelib::CONICS(
        save.ELTS.as_slice(),
        save.ET,
        save.STATE0.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKW05(
        save.HANDLE[2],
        SCID,
        499,
        b"MARSIAU",
        -((10 as f64) * spicelib::JYEAR()),
        ((10 as f64) * spicelib::JYEAR()),
        b"Mars orbiter",
        save.ELTS[8],
        1,
        save.STATE0.as_slice(),
        &[save.ET],
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKCLS(save.HANDLE[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load the new SPK file.
    //
    spicelib::SPKLEF(SPK2, &mut save.HANDLE[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Add the orbiter's name/ID mapping to the kernel pool.
    //
    spicelib::PCPOOL(b"NAIF_BODY_NAME", 1, save.OBSNMS.subarray(2), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::PIPOOL(b"NAIF_BODY_CODE", 1, &[SCID], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Add an incomplete frame definition to the kernel pool;
    // we'll need this later.
    //
    spicelib::PIPOOL(b"FRAME_BAD_NAME", 1, &[-666], ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Setup: create DSK files.", ctx)?;

    //
    // For Mars, surface 1 is the "main" surface.
    //
    fstr::assign(&mut save.TARGET, b"MARS");
    save.TRGCDE = 499;
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");

    save.BODYID = save.TRGCDE;
    save.SURFID = 1;
    save.NLON = 220;
    save.NLAT = 110;

    if spicelib::EXISTS(DSK0, ctx)? {
        spicelib::DELFIL(DSK0, ctx)?;
    }

    testutil::T_ELDS2Z(
        save.BODYID,
        save.SURFID,
        &save.FIXREF,
        save.NLON,
        save.NLAT,
        DSK0,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load main Mars DSK.
    //
    spicelib::FURNSH(DSK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Surface 2 for Mars has slightly lower resolution.
    //
    save.BODYID = save.TRGCDE;
    save.SURFID = 2;
    save.NLON = 190;
    save.NLAT = 95;

    if spicelib::EXISTS(DSK1, ctx)? {
        spicelib::DELFIL(DSK1, ctx)?;
    }

    //
    // Create and load the second DSK.
    //
    testutil::T_ELDS2Z(
        save.BODYID,
        save.SURFID,
        &save.FIXREF,
        save.NLON,
        save.NLAT,
        DSK1,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(DSK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Surface 1 for Phobos is high-res.
    //
    save.BODYID = 401;
    save.SURFID = 1;
    fstr::assign(&mut save.FIXREF, b"IAU_PHOBOS");

    save.NLON = 200;
    save.NLAT = 100;

    if spicelib::EXISTS(DSK2, ctx)? {
        spicelib::DELFIL(DSK2, ctx)?;
    }

    //
    // Create and load the first Phobos DSK.
    //
    testutil::T_ELDS2Z(
        save.BODYID,
        save.SURFID,
        &save.FIXREF,
        save.NLON,
        save.NLAT,
        DSK2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Surface 2 for Phobos is lower-res.
    //
    save.BODYID = 401;
    save.SURFID = 2;
    fstr::assign(&mut save.FIXREF, b"IAU_PHOBOS");

    save.NLON = 80;
    save.NLAT = 40;

    if spicelib::EXISTS(DSK3, ctx)? {
        spicelib::DELFIL(DSK3, ctx)?;
    }

    //
    // Create and load the second Phobos DSK.
    //
    testutil::T_ELDS2Z(
        save.BODYID,
        save.SURFID,
        &save.FIXREF,
        save.NLON,
        save.NLAT,
        DSK3,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(DSK3, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set up a surface name-ID map.
    //
    save.SRFBOD[1] = 499;
    save.SRFIDS[1] = 1;
    fstr::assign(save.SRFNMS.get_mut(1), b"high-res");

    save.SRFBOD[2] = 499;
    save.SRFIDS[2] = 2;
    fstr::assign(save.SRFNMS.get_mut(2), b"low-res");

    save.SRFBOD[3] = 401;
    save.SRFIDS[3] = 1;
    fstr::assign(save.SRFNMS.get_mut(3), b"high-res");

    save.SRFBOD[4] = 401;
    save.SRFIDS[4] = 2;
    fstr::assign(save.SRFNMS.get_mut(4), b"low-res");

    spicelib::PCPOOL(b"NAIF_SURFACE_NAME", NMAP, save.SRFNMS.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(b"NAIF_SURFACE_CODE", NMAP, save.SRFIDS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(b"NAIF_SURFACE_BODY", NMAP, save.SRFBOD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     Main test loop follows.
    //
    //
    //     This loop uses a single epoch. The set of tests using
    //     nested tori, located further down in this file, does
    //     vary the input times.
    //
    //     Loop over every choice of observer.
    //
    for OBSIDX in 1..=NOBS {
        fstr::assign(&mut save.OBSRVR, save.OBSNMS.get(OBSIDX));
        //
        // Set the observer ID code.
        //
        spicelib::BODN2C(&save.OBSRVR, &mut save.OBSCDE, &mut save.FOUND, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Loop over every choice of target.
        //
        for TRGIDX in 1..=NTARG {
            fstr::assign(&mut save.TARGET, save.TRGNMS.get(TRGIDX));
            //
            // Set the target ID code.
            //
            spicelib::BODN2C(&save.TARGET, &mut save.TRGCDE, &mut save.FOUND, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Get target radii.
            //
            spicelib::BODVAR(
                save.TRGCDE,
                b"RADII",
                &mut save.N,
                save.RADII.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Loop over every aberration correction locus.
            //
            for LOCIDX in 1..=NLOC {
                fstr::assign(&mut save.CORLOC, save.CORLCS.get(LOCIDX));
                //
                // Loop over every aberration correction choice.
                //
                for ABCIDX in 1..=NABC {
                    fstr::assign(&mut save.ABCORR, save.ABCS.get(ABCIDX));

                    spicelib::ZZPRSCOR(&save.ABCORR, save.ATTBLK.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Set up some logical variables describing the
                    // attributes of the selected correction.
                    //
                    save.USELT = save.ATTBLK[LTIDX];
                    save.USECN = save.ATTBLK[CNVIDX];

                    //
                    // Loop over every target body-fixed frame choice.
                    //
                    for REFIDX in 1..=NREF {
                        fstr::assign(&mut save.TRGFRM, save.REFS.get([REFIDX, TRGIDX]));
                        //
                        // Loop over all method choices that are
                        // compatible with the choice of CORLOC.
                        //
                        // The "center" locus is required  in order
                        // to use the "guided" methods.
                        //
                        if spicelib::EQSTR(&save.CORLOC, b"CENTER") {
                            save.UB = NMETH;
                        } else {
                            //
                            // The locus is 'ELLIPSOID LIMB'. Only
                            // the "tangent" methods can be used.
                            //
                            save.UB = NTANGT;
                        }

                        for MIX in 1..=save.UB {
                            fstr::assign(&mut save.METHOD, save.METHDS.get(MIX));

                            save.ISELL =
                                spicelib::MATCHI(&save.METHOD, b"*ELLIPSOID*", b"*", b"?", ctx);

                            save.ISDSK = !save.ISELL;

                            save.ISTAN =
                                spicelib::MATCHI(&save.METHOD, b"*TANGENT*", b"*", b"?", ctx);

                            save.ISHI = spicelib::MATCHI(&save.METHOD, b"*HIGH*", b"*", b"?", ctx);

                            //
                            // --- Case: ------------------------------------------------------
                            //

                            fstr::assign(&mut save.TITLE, b"Observer = #; Target = #; ABCORR = #; TRGFRM = #; METHOD = #; CORLOC = #.");
                            spicelib::REPMC(
                                &save.TITLE.to_vec(),
                                b"#",
                                &save.OBSRVR,
                                &mut save.TITLE,
                            );
                            spicelib::REPMC(
                                &save.TITLE.to_vec(),
                                b"#",
                                &save.TARGET,
                                &mut save.TITLE,
                            );
                            spicelib::REPMC(
                                &save.TITLE.to_vec(),
                                b"#",
                                &save.ABCORR,
                                &mut save.TITLE,
                            );
                            spicelib::REPMC(
                                &save.TITLE.to_vec(),
                                b"#",
                                &save.TRGFRM,
                                &mut save.TITLE,
                            );
                            spicelib::REPMC(
                                &save.TITLE.to_vec(),
                                b"#",
                                &save.METHOD,
                                &mut save.TITLE,
                            );
                            spicelib::REPMC(
                                &save.TITLE.to_vec(),
                                b"#",
                                &save.CORLOC,
                                &mut save.TITLE,
                            );

                            testutil::TCASE(&save.TITLE, ctx)?;

                            //
                            // Start off by computing the set of limb points
                            // We'll then check the results.
                            //
                            save.NCUTS = 4;

                            save.ROLSTP = (spicelib::TWOPI(ctx) / save.NCUTS as f64);
                            spicelib::VPACK(0.0, 0.0, 1.0, save.REFVEC.as_slice_mut());

                            //
                            // We expect a single limb point in each half
                            // plane, so we can use a large step.
                            //
                            save.SCHSTP = 4.0;

                            //
                            // Derive the solution tolerance from the height
                            // error tolerance and the observer-target center
                            // distance.
                            //
                            spicelib::SPKPOS(
                                &save.TARGET,
                                save.ET,
                                &save.TRGFRM,
                                &save.ABCORR,
                                &save.OBSRVR,
                                save.TRGPOS.as_slice_mut(),
                                &mut save.TRGLT,
                                ctx,
                            )?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            save.DIST = spicelib::VNORM(save.TRGPOS.as_slice());

                            save.HTOL = 0.000001;
                            save.SOLTOL = (save.HTOL / save.DIST);

                            save.MAXN = MAXPNT;

                            //
                            // If the shape is DSK and the locus is ELLIPSOID
                            // limb, make a preliminary call to obtain the
                            // expected epochs.
                            //
                            if (save.ISDSK && spicelib::EQSTR(&save.CORLOC, b"ELLIPSOID LIMB")) {
                                //
                                // Create a version of the method string with
                                // an ellipsoid shape specification.
                                //
                                fstr::assign(&mut save.METHD2, b"ELLIPSOID/TANGENT");
                                //
                                // Get the expected epochs for the call below
                                // that uses a DSK target shape.
                                //
                                spicelib::LIMBPT(
                                    &save.METHD2,
                                    &save.TARGET,
                                    save.ET,
                                    &save.TRGFRM,
                                    &save.ABCORR,
                                    &save.CORLOC,
                                    &save.OBSRVR,
                                    save.REFVEC.as_slice(),
                                    save.ROLSTP,
                                    save.NCUTS,
                                    save.SCHSTP,
                                    save.SOLTOL,
                                    save.MAXN,
                                    save.NPTS.as_slice_mut(),
                                    save.EPNTS.as_slice_mut(),
                                    save.XEPOCH.as_slice_mut(),
                                    save.ETNGTS.as_slice_mut(),
                                    ctx,
                                )?;
                            }

                            spicelib::LIMBPT(
                                &save.METHOD,
                                &save.TARGET,
                                save.ET,
                                &save.TRGFRM,
                                &save.ABCORR,
                                &save.CORLOC,
                                &save.OBSRVR,
                                save.REFVEC.as_slice(),
                                save.ROLSTP,
                                save.NCUTS,
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

                            //
                            // Check the outputs.
                            //
                            save.K = 1;

                            for I in 1..=save.NCUTS {
                                //
                                // For convex shapes that include the origin,
                                // there should be just one limb point per
                                // cutting half-plane.
                                //
                                fstr::assign(&mut save.LABEL, b"NPTS(#)");
                                spicelib::REPMI(
                                    &save.LABEL.to_vec(),
                                    b"#",
                                    I,
                                    &mut save.LABEL,
                                    ctx,
                                );
                                testutil::CHCKXC(false, b" ", OK, ctx)?;

                                testutil::CHCKSI(&save.LABEL, save.NPTS[I], b"=", 1, 0, OK, ctx)?;

                                for J in 1..=save.NPTS[I] {
                                    //
                                    // We'll treat the Jth limb point as an
                                    // ephemeris object and find its position
                                    // relative to the observer.
                                    //
                                    spicelib::VEQU(
                                        save.POINTS.subarray([1, save.K]),
                                        save.LPOINT.as_slice_mut(),
                                    );
                                    spicelib::VEQU(
                                        save.TANGTS.subarray([1, save.K]),
                                        save.TANVEC.as_slice_mut(),
                                    );
                                    save.LPTEPC = save.EPOCHS[I];

                                    //
                                    // Get an inertially referenced version
                                    // of the tangent vector.
                                    //
                                    spicelib::PXFORM(
                                        b"J2000",
                                        &save.TRGFRM,
                                        save.LPTEPC,
                                        save.XFORM.as_slice_mut(),
                                        ctx,
                                    )?;
                                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                                    spicelib::MTXV(
                                        save.XFORM.as_slice(),
                                        save.TANVEC.as_slice(),
                                        save.ITANVC.as_slice_mut(),
                                    );

                                    //
                                    // Set the SPK lookup locus to be compatible
                                    // with the aberration correction locus used
                                    // with the limb point.
                                    //
                                    if fstr::eq(&save.CORLOC, b"CENTER") {
                                        //
                                        // In this case we're using aberration
                                        // corrections for the target body
                                        // center. We must compute the expected
                                        // state of the limb point "manually."
                                        //
                                        spicelib::SPKPOS(
                                            &save.TARGET,
                                            save.ET,
                                            &save.TRGFRM,
                                            &save.ABCORR,
                                            &save.OBSRVR,
                                            save.TRGPOS.as_slice_mut(),
                                            &mut save.TRGLT,
                                            ctx,
                                        )?;
                                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                                        spicelib::VADD(
                                            save.TRGPOS.as_slice(),
                                            save.LPOINT.as_slice(),
                                            save.XSRFVC.as_slice_mut(),
                                        );
                                        //
                                        // For the 'CENTER' locus, epochs
                                        // associated with limb points be the
                                        // epoch associated with the target
                                        // body's center.
                                        //
                                        save.LT = save.TRGLT;
                                        //
                                        // Get an inertially referenced version
                                        // of the expected tangent vector.
                                        //
                                        spicelib::PXFORM(
                                            b"J2000",
                                            &save.TRGFRM,
                                            save.LPTEPC,
                                            save.XFORM.as_slice_mut(),
                                            ctx,
                                        )?;
                                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                                        spicelib::MTXV(
                                            save.XFORM.as_slice(),
                                            save.XSRFVC.as_slice(),
                                            save.XISRFV.as_slice_mut(),
                                        );
                                    } else {
                                        //
                                        // This is the 'ellipsoid limb' case.
                                        //
                                        fstr::assign(&mut save.SPKLOC, b"TARGET");

                                        spicelib::SPKCPT(
                                            save.LPOINT.as_slice(),
                                            &save.TARGET,
                                            &save.TRGFRM,
                                            save.ET,
                                            &save.TRGFRM,
                                            &save.SPKLOC,
                                            &save.ABCORR,
                                            &save.OBSRVR,
                                            save.STATE.as_slice_mut(),
                                            &mut save.LT,
                                            ctx,
                                        )?;
                                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                                        spicelib::VEQU(
                                            save.STATE.as_slice(),
                                            save.XSRFVC.as_slice_mut(),
                                        );

                                        spicelib::SPKCPT(
                                            save.LPOINT.as_slice(),
                                            &save.TARGET,
                                            &save.TRGFRM,
                                            save.ET,
                                            b"J2000",
                                            &save.SPKLOC,
                                            &save.ABCORR,
                                            &save.OBSRVR,
                                            save.ISTATE.as_slice_mut(),
                                            &mut save.LT,
                                            ctx,
                                        )?;
                                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                                        spicelib::VEQU(
                                            save.ISTATE.as_slice(),
                                            save.XISRFV.as_slice_mut(),
                                        );
                                    }

                                    //
                                    // If LPOINT is correct, then the position
                                    // of LPOINT relative to the observer should
                                    // be equal to TANVEC. The light time
                                    // obtained from SPKCPT or computed manually
                                    // should match that implied by LPTEPC.
                                    //
                                    save.TOL = VTIGHT;

                                    //
                                    // If the light time correction is
                                    // non-converged, we can't expect tight
                                    // agreement.
                                    //
                                    if (save.USELT && !save.USECN) {
                                        save.TOL = (save.TOL * 100.0);
                                    }

                                    if (save.ISDSK && fstr::eq(&save.CORLOC, b"ELLIPSOID LIMB")) {
                                        //
                                        // Set the expected epoch using that
                                        // obtained using the ellipsoid target
                                        // shape.
                                        //
                                        save.XTE = save.XEPOCH[I];
                                    } else {
                                        //
                                        // Compute the expected epoch from the
                                        // SPKCPT or SPKPOS call.
                                        //
                                        spicelib::ZZCOREPC(
                                            &save.ABCORR,
                                            save.ET,
                                            save.LT,
                                            &mut save.XTE,
                                            ctx,
                                        )?;
                                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                                    }

                                    fstr::assign(&mut save.LABEL, b"LPTEPC PLANE #, PT #");
                                    spicelib::REPMI(
                                        &save.LABEL.to_vec(),
                                        b"#",
                                        I,
                                        &mut save.LABEL,
                                        ctx,
                                    );
                                    spicelib::REPMI(
                                        &save.LABEL.to_vec(),
                                        b"#",
                                        J,
                                        &mut save.LABEL,
                                        ctx,
                                    );

                                    testutil::CHCKSD(
                                        &save.LABEL,
                                        save.LPTEPC,
                                        b"~/",
                                        save.XTE,
                                        save.TOL,
                                        OK,
                                        ctx,
                                    )?;

                                    if save.USELT {
                                        if save.USECN {
                                            save.TOL = MTIGHT;
                                        } else {
                                            save.TOL = LOOSE;
                                        }
                                    } else {
                                        save.TOL = VTIGHT;
                                    }

                                    if save.ISDSK {
                                        //
                                        // Use looser tolerances, since the
                                        // aberration correction performed by
                                        // LIMBPT is for the corresponding
                                        // ellipsoid limb point.
                                        //
                                        save.TOL = (save.TOL * 1000.0);
                                    }

                                    fstr::assign(&mut save.LABEL, b"TANVEC PLANE #, PT #");
                                    spicelib::REPMI(
                                        &save.LABEL.to_vec(),
                                        b"#",
                                        I,
                                        &mut save.LABEL,
                                        ctx,
                                    );
                                    spicelib::REPMI(
                                        &save.LABEL.to_vec(),
                                        b"#",
                                        J,
                                        &mut save.LABEL,
                                        ctx,
                                    );

                                    if spicelib::EQSTR(&save.OBSRVR, b"MARS_ORBITER") {
                                        //
                                        // Check the absolute distance error for
                                        // this case. Use a 5m tolerance.
                                        //
                                        testutil::CHCKAD(
                                            &save.LABEL,
                                            save.TANVEC.as_slice(),
                                            b"~~",
                                            save.XSRFVC.as_slice(),
                                            3,
                                            0.005,
                                            OK,
                                            ctx,
                                        )?;
                                    } else {
                                        //
                                        // Compare the inertially referenced
                                        // tangent vectors.
                                        //
                                        fstr::assign(&mut save.LABEL, b"ITANVC PLANE #, PT #");
                                        spicelib::REPMI(
                                            &save.LABEL.to_vec(),
                                            b"#",
                                            I,
                                            &mut save.LABEL,
                                            ctx,
                                        );
                                        spicelib::REPMI(
                                            &save.LABEL.to_vec(),
                                            b"#",
                                            J,
                                            &mut save.LABEL,
                                            ctx,
                                        );

                                        //
                                        // Use a tighter tolerance for the
                                        // inertial vector test.
                                        //
                                        testutil::CHCKAD(
                                            &save.LABEL,
                                            save.ITANVC.as_slice(),
                                            b"~~/",
                                            save.XISRFV.as_slice(),
                                            3,
                                            (save.TOL / 10 as f64),
                                            OK,
                                            ctx,
                                        )?;

                                        //
                                        // Check the relative error for the
                                        // body-fixed vectors.
                                        //
                                        testutil::CHCKAD(
                                            &save.LABEL,
                                            save.TANVEC.as_slice(),
                                            b"~~/",
                                            save.XSRFVC.as_slice(),
                                            3,
                                            save.TOL,
                                            OK,
                                            ctx,
                                        )?;
                                    }

                                    if !*OK {
                                        //
                                        // Perform the following test just to
                                        // display the absolute magnitude of
                                        // the error.
                                        //
                                        fstr::assign(&mut save.LABEL, b"TANVEC PLANE #, PT #");
                                        spicelib::REPMI(
                                            &save.LABEL.to_vec(),
                                            b"#",
                                            I,
                                            &mut save.LABEL,
                                            ctx,
                                        );
                                        spicelib::REPMI(
                                            &save.LABEL.to_vec(),
                                            b"#",
                                            J,
                                            &mut save.LABEL,
                                            ctx,
                                        );

                                        testutil::CHCKAD(
                                            &save.LABEL,
                                            save.TANVEC.as_slice(),
                                            b"~~",
                                            save.XSRFVC.as_slice(),
                                            3,
                                            0.000001,
                                            OK,
                                            ctx,
                                        )?;

                                        fstr::assign(&mut save.LABEL, b"ITANVC PLANE #, PT #");
                                        spicelib::REPMI(
                                            &save.LABEL.to_vec(),
                                            b"#",
                                            I,
                                            &mut save.LABEL,
                                            ctx,
                                        );
                                        spicelib::REPMI(
                                            &save.LABEL.to_vec(),
                                            b"#",
                                            J,
                                            &mut save.LABEL,
                                            ctx,
                                        );

                                        testutil::CHCKAD(
                                            &save.LABEL,
                                            save.ITANVC.as_slice(),
                                            b"~~",
                                            save.XISRFV.as_slice(),
                                            3,
                                            0.000001,
                                            OK,
                                            ctx,
                                        )?;
                                    }

                                    if ((ABCIDX <= 3) && save.ISELL) {
                                        //
                                        // The correction is 'NONE', 'CN', or
                                        // 'CN+S'. The shape is 'ELLIPSOID'. Look
                                        // for millimeter-level agreement.

                                        testutil::CHCKAD(
                                            &save.LABEL,
                                            save.TANVEC.as_slice(),
                                            b"~~",
                                            save.XSRFVC.as_slice(),
                                            3,
                                            0.000001,
                                            OK,
                                            ctx,
                                        )?;
                                    }

                                    //
                                    // We've checked the consistency of LPOINT,
                                    // TANVEC, and LPTEPC, but we haven't done
                                    // anything to show that LPOINT is a limb
                                    // point. Do this now.
                                    //
                                    // We need to verify that
                                    //
                                    //    1) The limb point is on the target
                                    //       body's surface.
                                    //
                                    //    2) For the "tangent" methods, the
                                    //       limb point is a point of tangency
                                    //       on the target of a ray emanating
                                    //       from the observer.
                                    //
                                    //       For the "guided" methods, the limb
                                    //       point lies on a ray emanating from
                                    //       the center of the limb of the
                                    //       reference ellipsoid.
                                    //
                                    //       When the target shape is an
                                    //       ellipsoid, there should be no
                                    //       difference between results obtained
                                    //       using the tangent and guided
                                    //       methods.
                                    //
                                    //    3) The limb point lies in the
                                    //       correct half-plane.
                                    //
                                    //
                                    // Check the limb point's distance from the
                                    // target surface.
                                    //
                                    if save.ISELL {
                                        //
                                        // Find the altitude of the limb point
                                        // with respect to the reference
                                        // ellipsoid.
                                        //
                                        spicelib::NEARPT(
                                            save.LPOINT.as_slice(),
                                            save.RADII[1],
                                            save.RADII[2],
                                            save.RADII[3],
                                            save.PNEAR.as_slice_mut(),
                                            &mut save.ALT,
                                            ctx,
                                        )?;
                                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                                        //
                                        // For ellipsoidal targets, one micron
                                        // is a generous tolerance.
                                        //
                                        save.TOL = 0.000000001;

                                        fstr::assign(&mut save.LABEL, b"LIMBPT ALT, PLANE #, PT #");
                                        spicelib::REPMI(
                                            &save.LABEL.to_vec(),
                                            b"#",
                                            I,
                                            &mut save.LABEL,
                                            ctx,
                                        );
                                        spicelib::REPMI(
                                            &save.LABEL.to_vec(),
                                            b"#",
                                            J,
                                            &mut save.LABEL,
                                            ctx,
                                        );

                                        testutil::CHCKSD(
                                            &save.LABEL,
                                            save.ALT,
                                            b"~",
                                            0.0,
                                            save.TOL,
                                            OK,
                                            ctx,
                                        )?;
                                    } else {
                                        //
                                        // This is the DSK case.
                                        //
                                        // We'll create an inward-pointing ray
                                        // that ideally passes through the limb
                                        // point, and we'll find the ray-surface
                                        // intercept. This intercept should be
                                        // close to the original limb point.

                                        spicelib::VSCL(
                                            10.0,
                                            save.LPOINT.as_slice(),
                                            save.VERTEX.as_slice_mut(),
                                        );
                                        spicelib::VMINUS(
                                            save.VERTEX.as_slice(),
                                            save.RAYDIR.as_slice_mut(),
                                        );

                                        save.NSURF = 1;

                                        if save.ISHI {
                                            save.SURFID = 1;
                                        } else {
                                            save.SURFID = 2;
                                        }

                                        spicelib::DSKXV(
                                            false,
                                            &save.TARGET,
                                            save.NSURF,
                                            &[save.SURFID],
                                            save.ET,
                                            &save.TRGFRM,
                                            1,
                                            save.VERTEX.as_slice(),
                                            save.RAYDIR.as_slice(),
                                            save.XPT.as_slice_mut(),
                                            std::slice::from_mut(&mut save.FOUND),
                                            ctx,
                                        )?;
                                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                                        testutil::CHCKSL(
                                            b"DSKXV FOUND",
                                            save.FOUND,
                                            true,
                                            OK,
                                            ctx,
                                        )?;

                                        //
                                        // Our ray should hit the DSK surface very
                                        // close to the limb point. One micron is
                                        // enough margin.
                                        //
                                        save.TOL = 0.000000001;

                                        save.DIST = spicelib::VDIST(
                                            save.XPT.as_slice(),
                                            save.LPOINT.as_slice(),
                                        );

                                        fstr::assign(
                                            &mut save.LABEL,
                                            b"LIMBPT DIST, PLANE #, PT #",
                                        );
                                        spicelib::REPMI(
                                            &save.LABEL.to_vec(),
                                            b"#",
                                            I,
                                            &mut save.LABEL,
                                            ctx,
                                        );
                                        spicelib::REPMI(
                                            &save.LABEL.to_vec(),
                                            b"#",
                                            J,
                                            &mut save.LABEL,
                                            ctx,
                                        );

                                        testutil::CHCKSD(
                                            &save.LABEL,
                                            save.DIST,
                                            b"~",
                                            0.0,
                                            save.TOL,
                                            OK,
                                            ctx,
                                        )?;
                                    }

                                    //
                                    // Verify that the putative limb point is
                                    // really on the limb.
                                    //
                                    // Create the central axis and normal to the
                                    // current cutting half-plane. We'll use
                                    // these for the DSK case below, and for the
                                    // next set of tests, in which the limb
                                    // point is tested for inclusion in the
                                    // cutting half- plane.

                                    spicelib::VSUB(
                                        save.LPOINT.as_slice(),
                                        save.TANVEC.as_slice(),
                                        save.AXIS.as_slice_mut(),
                                    );

                                    spicelib::VROTV(
                                        save.REFVEC.as_slice(),
                                        save.AXIS.as_slice(),
                                        (((I - 1) as f64) * save.ROLSTP),
                                        save.PLNVEC.as_slice_mut(),
                                    );

                                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                                    spicelib::UCRSS(
                                        save.AXIS.as_slice(),
                                        save.PLNVEC.as_slice(),
                                        save.CUTNML.as_slice_mut(),
                                    );

                                    //
                                    if save.ISELL {
                                        //
                                        // This is the ellipsoid case.
                                        //
                                        // If LPOINT is all it's claimed to be,
                                        // the outward normal at LPOINT should be
                                        // orthogonal to the observer-limb point
                                        // vector.
                                        //
                                        spicelib::SURFNM(
                                            save.RADII[1],
                                            save.RADII[2],
                                            save.RADII[3],
                                            save.LPOINT.as_slice(),
                                            save.NORMAL.as_slice_mut(),
                                            ctx,
                                        )?;
                                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                                        spicelib::VHAT(
                                            save.TANVEC.as_slice(),
                                            save.UTAN.as_slice_mut(),
                                        );

                                        save.D = spicelib::VDOT(
                                            save.NORMAL.as_slice(),
                                            save.UTAN.as_slice(),
                                        );

                                        save.TOL = VTIGHT;

                                        fstr::assign(
                                            &mut save.LABEL,
                                            b"<LIMBPT, TANGNT>, PLANE #, PT #",
                                        );
                                        spicelib::REPMI(
                                            &save.LABEL.to_vec(),
                                            b"#",
                                            I,
                                            &mut save.LABEL,
                                            ctx,
                                        );
                                        spicelib::REPMI(
                                            &save.LABEL.to_vec(),
                                            b"#",
                                            J,
                                            &mut save.LABEL,
                                            ctx,
                                        );

                                        testutil::CHCKSD(
                                            &save.LABEL,
                                            save.D,
                                            b"~",
                                            0.0,
                                            save.TOL,
                                            OK,
                                            ctx,
                                        )?;
                                    } else {
                                        //
                                        // This is the DSK case.
                                        //
                                        // We could check the relative orientation
                                        // of the outward normal and the
                                        // associated tangent vector, but we
                                        // can't expect these vectors to be
                                        // orthogonal.
                                        //
                                        // We can perform a more accurate check
                                        // by determining whether a small
                                        // rotation of the ray defined by the
                                        // observer and the tangent vector,
                                        // performed within the cutting
                                        // half-plane, will move the ray off of
                                        // the target.

                                        if save.ISTAN {
                                            //
                                            // This is the DSK tangent case.
                                            //
                                            // Rotate the tangent ray away from
                                            // the axis by twice the angular
                                            // search tolerance. Presuming the ray
                                            // was nearly tangential, it should
                                            // now point off the target.
                                            //
                                            save.THETA = -((2 as f64) * save.SOLTOL);

                                            spicelib::VROTV(
                                                save.TANVEC.as_slice(),
                                                save.CUTNML.as_slice(),
                                                save.THETA,
                                                save.RAYDIR.as_slice_mut(),
                                            );

                                            save.NSURF = 1;

                                            if save.ISHI {
                                                save.SURFID = 1;
                                            } else {
                                                save.SURFID = 2;
                                            }

                                            spicelib::DSKXV(
                                                false,
                                                &save.TARGET,
                                                save.NSURF,
                                                &[save.SURFID],
                                                save.ET,
                                                &save.TRGFRM,
                                                1,
                                                save.AXIS.as_slice(),
                                                save.RAYDIR.as_slice(),
                                                save.XPT.as_slice_mut(),
                                                std::slice::from_mut(&mut save.FOUND),
                                                ctx,
                                            )?;
                                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                                            fstr::assign(&mut save.LABEL, b"Ray hit PLANE #, PT #");
                                            spicelib::REPMI(
                                                &save.LABEL.to_vec(),
                                                b"#",
                                                I,
                                                &mut save.LABEL,
                                                ctx,
                                            );
                                            spicelib::REPMI(
                                                &save.LABEL.to_vec(),
                                                b"#",
                                                J,
                                                &mut save.LABEL,
                                                ctx,
                                            );

                                            testutil::CHCKSL(
                                                &save.LABEL,
                                                save.FOUND,
                                                false,
                                                OK,
                                                ctx,
                                            )?;
                                        } else {
                                            //
                                            // This is the DSK "guided" case.
                                            //
                                            // We've already checked that the limb
                                            // point is on the DSK surface; we
                                            // need to make sure it's in the
                                            // reference ellipsoid's limb plane.
                                            // The last check we'll do will ensure
                                            // the point is in the correct cutting
                                            // half-plane.
                                            //
                                            // Create the ellipsoid limb and
                                            // the limb plane.
                                            //
                                            spicelib::EDLIMB(
                                                save.RADII[1],
                                                save.RADII[2],
                                                save.RADII[3],
                                                save.AXIS.as_slice(),
                                                save.LIMB.as_slice_mut(),
                                                ctx,
                                            )?;
                                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                                            spicelib::EL2CGV(
                                                save.LIMB.as_slice(),
                                                save.CENTER.as_slice_mut(),
                                                save.SMAJOR.as_slice_mut(),
                                                save.SMINOR.as_slice_mut(),
                                            );
                                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                                            spicelib::PSV2PL(
                                                save.CENTER.as_slice(),
                                                save.SMAJOR.as_slice(),
                                                save.SMINOR.as_slice(),
                                                save.LPLANE.as_slice_mut(),
                                                ctx,
                                            )?;
                                            //
                                            // Project the guided DSK limb point
                                            // onto the limb plane.
                                            //
                                            spicelib::VPRJP(
                                                save.LPOINT.as_slice(),
                                                save.LPLANE.as_slice(),
                                                save.LPROJ.as_slice_mut(),
                                                ctx,
                                            )?;

                                            save.DIST = spicelib::VDIST(
                                                save.LPOINT.as_slice(),
                                                save.LPROJ.as_slice(),
                                            );
                                            //
                                            // One micron is a generous tolerance
                                            // for the distance of the limb point
                                            // from the ellipsoid limb plane.
                                            //
                                            save.TOL = 0.000000001;

                                            fstr::assign(
                                                &mut save.LABEL,
                                                b"LIMBPT ERROR, PLANE #, PT #",
                                            );
                                            spicelib::REPMI(
                                                &save.LABEL.to_vec(),
                                                b"#",
                                                I,
                                                &mut save.LABEL,
                                                ctx,
                                            );
                                            spicelib::REPMI(
                                                &save.LABEL.to_vec(),
                                                b"#",
                                                J,
                                                &mut save.LABEL,
                                                ctx,
                                            );

                                            testutil::CHCKSD(
                                                &save.LABEL,
                                                save.ALT,
                                                b"~",
                                                0.0,
                                                save.TOL,
                                                OK,
                                                ctx,
                                            )?;
                                        }
                                    }

                                    //
                                    // Verify that the limb point is in the
                                    // cutting half-plane. This test applies to
                                    // both ellipsoids and DSKs.
                                    //
                                    // The plane containing the cutting
                                    // half-plane contains the origin, so the
                                    // distance from the plane of a point is
                                    // given by the dot product of the point
                                    // with the plane's unit normal vector.
                                    //
                                    save.D = spicelib::VDOT(
                                        save.CUTNML.as_slice(),
                                        save.LPOINT.as_slice(),
                                    );

                                    save.TOL = 0.000001;

                                    fstr::assign(
                                        &mut save.LABEL,
                                        b"<LIMBPT, CUTNML>, PLANE #, PT #",
                                    );
                                    spicelib::REPMI(
                                        &save.LABEL.to_vec(),
                                        b"#",
                                        I,
                                        &mut save.LABEL,
                                        ctx,
                                    );
                                    spicelib::REPMI(
                                        &save.LABEL.to_vec(),
                                        b"#",
                                        J,
                                        &mut save.LABEL,
                                        ctx,
                                    );

                                    testutil::CHCKSD(
                                        &save.LABEL,
                                        save.D,
                                        b"~",
                                        0.0,
                                        save.TOL,
                                        OK,
                                        ctx,
                                    )?;

                                    //
                                    // Update K to point to the next limb point.
                                    //
                                    save.K = (save.K + 1);
                                }
                            }
                            //
                            // We're finished with the consistency checks.
                            //
                        }
                        //
                        // End of the method loop.
                        //
                    }
                    //
                    // End of the reference frame loop.
                    //
                }
                //
                // End of the aberration correction loop.
                //
            }
            //
            // End of the correction locus loop.
            //
        }
        //
        // End of the target loop.
        //
    }
    //
    // End of the observer loop.
    //

    //***********************************************************************
    //
    //     Normal case: nested tori as target shape
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Torus test setup", ctx)?;

    //
    // Create and load Nat's SPK and PCK.
    //
    testutil::NATSPK(SPK3, true, &mut save.HANDLE[3], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::NATPCK(PCK1, true, true, ctx)?;

    //
    // Create DSK containing two nested tori.
    //
    if spicelib::EXISTS(DSK4, ctx)? {
        spicelib::DELFIL(DSK4, ctx)?;
    }

    save.SURFID = 3;
    fstr::assign(&mut save.TRGFRM, b"ALPHA_VIEW_XY");

    save.R = 70000.0;
    save.RCROSS = 10000.0;

    spicelib::VPACK(0.0, 0.0, 0.0, save.CENTER.as_slice_mut());
    spicelib::VPACK(1.0, 0.0, 0.0, save.NORMAL.as_slice_mut());

    testutil::T_TORUS(
        ALPHA,
        save.SURFID,
        &save.TRGFRM,
        NPOLYV,
        NCROSS,
        save.R,
        save.RCROSS,
        save.CENTER.as_slice(),
        save.NORMAL.as_slice_mut(),
        DSK4,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.R = 40000.0;

    spicelib::VPACK(0.0, 0.0, 0.0, save.CENTER.as_slice_mut());
    spicelib::VPACK(1.0, 0.0, 0.0, save.NORMAL.as_slice_mut());

    testutil::T_TORUS(
        ALPHA,
        save.SURFID,
        &save.TRGFRM,
        NPOLYV,
        NCROSS,
        save.R,
        save.RCROSS,
        save.CENTER.as_slice(),
        save.NORMAL.as_slice_mut(),
        DSK4,
        ctx,
    )?;

    spicelib::FURNSH(DSK4, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Prepare LIMBPT call.
    //
    fstr::assign(&mut save.OBSRVR, b"SUN");
    fstr::assign(&mut save.TARGET, b"ALPHA");
    fstr::assign(&mut save.CORLOC, b"CENTER");
    fstr::assign(&mut save.METHOD, b"DSK/TANGENT/UNPRIORITIZED");
    save.ET0 = 0 as f64;
    save.TDELTA = 100000.0;
    //
    // Loop over epochs.
    //
    for TIMIDX in 1..=NTIMES {
        save.ET = (save.ET0 + (((TIMIDX - 1) as f64) * save.TDELTA));

        //
        // Loop over the aberration correction choices. For this
        // test, we use only 'NONE' and 'CN'.
        //
        for ABCIDX in 1..=2 {
            //
            //--- Case: ------------------------------------------------------
            //
            fstr::assign(&mut save.ABCORR, save.ABCS.get(ABCIDX));

            fstr::assign(&mut save.TITLE, b"Observer = #; Target = #; ABCORR = #; TRGFRM = #; METHOD = #; CORLOC = #; ET = #.");
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.OBSRVR, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TRGFRM, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.METHOD, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CORLOC, &mut save.TITLE);
            spicelib::REPMD(
                &save.TITLE.to_vec(),
                b"#",
                save.ET,
                14,
                &mut save.TITLE,
                ctx,
            );
            testutil::TCASE(&save.TITLE, ctx)?;

            //
            // Start off by computing the set of limb points
            // We'll then check the results.
            //
            save.NCUTS = 1;
            save.ROLSTP = (spicelib::TWOPI(ctx) / save.NCUTS as f64);
            spicelib::VPACK(0.0, 0.0, 1.0, save.REFVEC.as_slice_mut());

            //
            // We expect a multiple limb points in each half
            // plane, so we must use a small step.
            //
            save.SCHSTP = 0.0001;

            //
            // Derive the solution tolerance from the height
            // error tolerance and the observer-target center
            // distance.
            //
            spicelib::SPKPOS(
                &save.TARGET,
                save.ET,
                &save.TRGFRM,
                &save.ABCORR,
                &save.OBSRVR,
                save.TRGPOS.as_slice_mut(),
                &mut save.TRGLT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.DIST = spicelib::VNORM(save.TRGPOS.as_slice());

            save.HTOL = 0.000001;
            save.SOLTOL = (save.HTOL / save.DIST);

            save.MAXN = MAXPNT;

            spicelib::LIMBPT(
                &save.METHOD,
                &save.TARGET,
                save.ET,
                &save.TRGFRM,
                &save.ABCORR,
                &save.CORLOC,
                &save.OBSRVR,
                save.REFVEC.as_slice(),
                save.ROLSTP,
                save.NCUTS,
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

            //
            // Check the outputs.
            //
            save.K = 1;

            for I in 1..=save.NCUTS {
                //
                // There should be 4 limb points in each cutting
                // half-plane.
                //
                fstr::assign(&mut save.LABEL, b"NPTS HALF-PLANE #,");
                spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);

                testutil::CHCKSI(&save.LABEL, save.NPTS[I], b"=", 4, 0, OK, ctx)?;

                for J in 1..=save.NPTS[I] {
                    //
                    // We'll treat the Jth limb point as an
                    // ephemeris object and find its position
                    // relative to the observer.
                    //
                    spicelib::VEQU(
                        save.POINTS.subarray([1, save.K]),
                        save.LPOINT.as_slice_mut(),
                    );
                    spicelib::VEQU(
                        save.TANGTS.subarray([1, save.K]),
                        save.TANVEC.as_slice_mut(),
                    );
                    save.LPTEPC = save.EPOCHS[I];

                    fstr::assign(&mut save.SPKLOC, b"TARGET");

                    spicelib::SPKCPT(
                        save.LPOINT.as_slice(),
                        &save.TARGET,
                        &save.TRGFRM,
                        save.ET,
                        &save.TRGFRM,
                        &save.SPKLOC,
                        &save.ABCORR,
                        &save.OBSRVR,
                        save.STATE.as_slice_mut(),
                        &mut save.LT,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::VEQU(save.STATE.as_slice(), save.XSRFVC.as_slice_mut());

                    //
                    // Since we're using the 'CENTER' locus, the light time
                    // obtained from SPKPOS should match that implied by
                    // LPTEPC.
                    //
                    save.TOL = VTIGHT;

                    spicelib::ZZCOREPC(&save.ABCORR, save.ET, save.TRGLT, &mut save.XTE, ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    fstr::assign(&mut save.LABEL, b"LPTEPC PLANE #, PT #");
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);

                    testutil::CHCKSD(&save.LABEL, save.LPTEPC, b"~/", save.XTE, save.TOL, OK, ctx)?;

                    //
                    // If LPOINT is correct, then the position
                    // of LPOINT relative to the observer should
                    // be equal to TANVEC.
                    //
                    if fstr::eq(&save.ABCORR, b"NONE") {
                        save.TOL = VTIGHT;
                    } else {
                        //
                        // Use looser tolerances, since the aberration
                        // correction performed by LIMBPT is for the
                        // target center.

                        save.TOL = TIGHT;
                    }

                    fstr::assign(&mut save.LABEL, b"TANVEC PLANE #, PT #");
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);

                    //
                    // Check the absolute distance error for
                    // this case. Use a 5mm tolerance.
                    //
                    testutil::CHCKAD(
                        &save.LABEL,
                        save.TANVEC.as_slice(),
                        b"~~",
                        save.XSRFVC.as_slice(),
                        3,
                        0.000005,
                        OK,
                        ctx,
                    )?;

                    //
                    // Check the relative error for this case.
                    //
                    testutil::CHCKAD(
                        &save.LABEL,
                        save.TANVEC.as_slice(),
                        b"~~/",
                        save.XSRFVC.as_slice(),
                        3,
                        save.TOL,
                        OK,
                        ctx,
                    )?;

                    if !*OK {
                        //
                        // Perform the following test just to
                        // display the absolute magnitude of
                        // the error.
                        //
                        testutil::CHCKAD(
                            &save.LABEL,
                            save.TANVEC.as_slice(),
                            b"~~",
                            save.XSRFVC.as_slice(),
                            3,
                            0.000001,
                            OK,
                            ctx,
                        )?;
                    }

                    //
                    // We've checked the consistency of LPOINT, TANVEC, and
                    // LPTEPC, but we haven't done anything to show that
                    // LPOINT is a limb point. Do this now.
                    //
                    // Check the limb point's distance from the target
                    // surface.
                    //
                    // We'll create an inward-pointing ray that ideally
                    // passes through the limb point, and we'll find the
                    // ray-surface intercept. This intercept should be close
                    // to the original limb point.
                    //
                    // Create the ray's vertex using the outward normal at
                    // the surface point. The ray's direction will be the
                    // negative of the normal.

                    spicelib::SRFNRM(
                        b"DSK/UNPRIORITIZED",
                        &save.TARGET,
                        save.ET,
                        &save.TRGFRM,
                        1,
                        save.LPOINT.as_slice(),
                        save.NORMAL.as_slice_mut(),
                        ctx,
                    )?;

                    spicelib::VLCOM(
                        1.0,
                        save.LPOINT.as_slice(),
                        0.1,
                        save.NORMAL.as_slice(),
                        save.VERTEX.as_slice_mut(),
                    );
                    spicelib::VMINUS(save.NORMAL.as_slice(), save.RAYDIR.as_slice_mut());

                    save.NSURF = 1;

                    spicelib::DSKXV(
                        false,
                        &save.TARGET,
                        save.NSURF,
                        &[save.SURFID],
                        save.ET,
                        &save.TRGFRM,
                        1,
                        save.VERTEX.as_slice(),
                        save.RAYDIR.as_slice(),
                        save.XPT.as_slice_mut(),
                        std::slice::from_mut(&mut save.FOUND),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::CHCKSL(b"DSKXV FOUND", save.FOUND, true, OK, ctx)?;

                    //
                    // Our ray should hit the DSK surface very
                    // close to the limb point. One micron is
                    // enough margin.
                    //
                    save.TOL = 0.000000001;

                    save.DIST = spicelib::VDIST(save.XPT.as_slice(), save.LPOINT.as_slice());

                    fstr::assign(&mut save.LABEL, b"LIMBPT DIST, PLANE #, PT #");
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);

                    testutil::CHCKSD(&save.LABEL, save.DIST, b"~", 0.0, save.TOL, OK, ctx)?;

                    //
                    // Verify that the putative limb point is
                    // really on the limb.
                    //
                    // Create the central axis and normal to the
                    // current cutting half-plane. We'll use these
                    // for the DSK case below, and for the next
                    // set of tests, in which the limb point is
                    // tested for inclusion in the cutting half-
                    // plane.
                    //
                    spicelib::VSUB(
                        save.LPOINT.as_slice(),
                        save.TANVEC.as_slice(),
                        save.AXIS.as_slice_mut(),
                    );

                    spicelib::VROTV(
                        save.REFVEC.as_slice(),
                        save.AXIS.as_slice(),
                        (((I - 1) as f64) * save.ROLSTP),
                        save.PLNVEC.as_slice_mut(),
                    );

                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::UCRSS(
                        save.AXIS.as_slice(),
                        save.PLNVEC.as_slice(),
                        save.CUTNML.as_slice_mut(),
                    );

                    //
                    // This is the DSK tangent case.
                    //
                    // Rotate the tangent ray to move it off of the target
                    // surface. The rotation is within the cutting
                    // half-plane. The direction of rotation depends on the
                    // placement of the surface point: for points having
                    // outward normals pointing away from the axis, the
                    // rotation is away from the axis; for the other points,
                    // the direction is inward. For the nested tori, the
                    // points having and odd index are the ones for which
                    // the rotation is outward.
                    //
                    // Outward rotation about CUTNML corresponds to a
                    // negative rotation angle.
                    //
                    // The rotation angle is set to twice the angular search
                    // tolerance. Presuming the ray was nearly tangential,
                    // it should now point off the target.
                    //
                    if spicelib::ODD(J) {
                        save.THETA = -((2 as f64) * save.SOLTOL);
                    } else {
                        save.THETA = ((2 as f64) * save.SOLTOL);
                    }

                    spicelib::VROTV(
                        save.TANVEC.as_slice(),
                        save.CUTNML.as_slice(),
                        save.THETA,
                        save.RAYDIR.as_slice_mut(),
                    );

                    save.NSURF = 1;

                    spicelib::DSKXV(
                        false,
                        &save.TARGET,
                        save.NSURF,
                        &[save.SURFID],
                        save.ET,
                        &save.TRGFRM,
                        1,
                        save.AXIS.as_slice(),
                        save.RAYDIR.as_slice(),
                        save.XPT.as_slice_mut(),
                        std::slice::from_mut(&mut save.FOUND),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    fstr::assign(&mut save.LABEL, b"Ray hit PLANE #, PT #");
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);

                    testutil::CHCKSL(&save.LABEL, save.FOUND, false, OK, ctx)?;
                    //
                    // Verify that the limb point is in the cutting
                    // half-plane. This test applies to both ellipsoids and
                    // DSKs.
                    //
                    // The plane containing the cutting half-plane contains
                    // the origin, so the distance from the plane of a point
                    // is given by the dot product of the point with the
                    // plane's unit normal vector.
                    //
                    save.D = spicelib::VDOT(save.CUTNML.as_slice(), save.LPOINT.as_slice());

                    save.TOL = 0.000001;

                    fstr::assign(&mut save.LABEL, b"<LIMBPT, CUTNML>, PLANE #, PT #");
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);

                    testutil::CHCKSD(&save.LABEL, save.D, b"~", 0.0, save.TOL, OK, ctx)?;

                    //
                    // Update K to point to the next limb point.
                    //
                    save.K = (save.K + 1);
                }
            }

            //
            // We're finished with the consistency checks.
            //
        }
        //
        // End of the aberration correction loop.
        //
    }
    //
    // End of the time loop.
    //

    //
    // Unload the torus DSK. Leaving it loaded would interfere
    // with some of the following tests.
    //
    spicelib::UNLOAD(DSK4, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //***********************************************************************
    //
    //     Normal case: input handling
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Input handling tests:  make sure target and observer
    // can be identified using integer "names."
    //

    //***********************************************************************
    //
    //     Normal case: state change detection
    //
    //***********************************************************************

    //
    // Certain subsystem state changes must be detected and responded to
    // by SINCPT. The subsystems (or structures) having states that must
    // be monitored are:
    //
    //    - Target name-ID mapping
    //
    //    - Observer name-ID mapping
    //
    //    - Surface name-ID mapping
    //
    //    - Target body-fixed frame definition
    //
    //    - ZZDSKBSR state
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Target name changed to JUPITER for ID code 499.", ctx)?;

    //
    // First, get expected intercept.
    //
    save.ET = ((8 as f64) * spicelib::JYEAR());

    fstr::assign(&mut save.TARGET, b"MARS");
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.ABCORR, b"NONE");
    save.NCUTS = 1;
    fstr::assign(&mut save.METHOD, b"DSK/UNPRIORITIZED/GUIDED");
    fstr::assign(&mut save.CORLOC, b"CENTER");

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.XPOINT.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODDEF(b"JUPITER", 499, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
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

    //
    // We expect an exact match here.
    //
    testutil::CHCKAD(
        b"POINTS",
        save.POINTS.as_slice(),
        b"=",
        save.XPOINT.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;

    // Restore original mapping.
    //
    spicelib::BODDEF(b"JUPITER", 599, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Observer name changed to SUN for ID code 399.", ctx)?;

    spicelib::BODDEF(b"SUN", 399, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
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
    //
    // We expect an exact match here.
    //
    testutil::CHCKAD(
        b"POINTS",
        save.POINTS.as_slice(),
        b"=",
        save.XPOINT.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Restore original mapping.
    //
    spicelib::BODDEF(b"SUN", 10, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Mars high-res surface name changed to AAAbbb.", ctx)?;

    //
    // Get expected results first.
    //
    save.ET = ((8 as f64) * spicelib::JYEAR());

    fstr::assign(&mut save.TARGET, b"MARS");
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.ABCORR, b"NONE");

    fstr::assign(&mut save.METHOD, b"DSK/UNPRIORITIZED/GUIDED/surfaces = 1");

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.XPOINT.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(save.SRFNMS.get_mut(1), b"AAAbbb");

    spicelib::PCPOOL(b"NAIF_SURFACE_NAME", NMAP, save.SRFNMS.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(
        &mut save.METHOD,
        b"guided/dsk/unprioritized/surfaces = AAAbbb",
    );

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
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

    //
    // We expect an exact match here.
    //
    testutil::CHCKAD(
        b"POINTS",
        save.POINTS.as_slice(),
        b"=",
        save.XPOINT.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Restore original mapping.
    //
    fstr::assign(save.SRFNMS.get_mut(1), b"high-res");

    spicelib::PCPOOL(b"NAIF_SURFACE_NAME", NMAP, save.SRFNMS.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Unload Mars high-res DSK.", ctx)?;

    //
    // Get reference result using low-res Mars DSK.
    //
    save.ET = ((8 as f64) * spicelib::JYEAR());

    fstr::assign(&mut save.TARGET, b"MARS");
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.ABCORR, b"NONE");

    fstr::assign(
        &mut save.METHOD,
        b"guided/dsk/unprioritized/surfaces = low-res",
    );

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.XPOINT.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Unload the high-res DSK; set METHOD to remove
    // surface specification.
    //
    spicelib::UNLOAD(DSK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.METHOD, b"guided/dsk/unprioritized");

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
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
    //
    // We expect an exact match here.
    //
    testutil::CHCKAD(
        b"POINTS",
        save.POINTS.as_slice(),
        b"=",
        save.XPOINT.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Unload Mars low-res DSK; reload Mars high-res DSK.", ctx)?;

    //
    // Restore DSK, unload low-res DSK, and repeat computation.
    //
    spicelib::FURNSH(DSK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(DSK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.METHOD, b"guided/dsk/unprioritized");

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
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

    //
    // Make sure the result matches that obtained with the
    // high-res DSK specified.
    //
    fstr::assign(
        &mut save.METHOD,
        b"guided/dsk/unprioritized/ SURFACES = \"HIGH-RES\" ",
    );

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.XPOINT.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an exact match here.
    //
    testutil::CHCKAD(
        b"POINTS",
        save.POINTS.as_slice(),
        b"=",
        save.XPOINT.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;

    //***********************************************************************
    //
    //     Error handling tests follow.
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid method.", ctx)?;

    save.ET = ((8 as f64) * spicelib::JYEAR());

    fstr::assign(&mut save.TARGET, b"MARS");
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.ABCORR, b"NONE");

    fstr::assign(&mut save.METHOD, b"ELLIPSOID TANGENT");

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADMETHODSYNTAX)", OK, ctx)?;

    fstr::assign(&mut save.METHOD, b"DSK/GUIDED");

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADPRIORITYSPEC)", OK, ctx)?;

    fstr::assign(&mut save.METHOD, b"DSK/UNPRIORITIZED");

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDLIMBTYPE)", OK, ctx)?;

    fstr::assign(&mut save.METHOD, b"UNPRIORITIZED /DSK/guided/INTERCEPT");

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDMETHOD)", OK, ctx)?;

    fstr::assign(&mut save.METHOD, b"UNPRIORITIZED/umbral /DSK/guided/");

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDMETHOD)", OK, ctx)?;

    //
    // Restore a valid method. We'll use the tangent limb
    // type because it requires the search step and
    // tolerance.
    //
    fstr::assign(&mut save.METHOD, b"TANGENT/DSK/UNPRIORITIZED");

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid target name.", ctx)?;

    spicelib::LIMBPT(
        &save.METHOD,
        b"marr",
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid observer name.", ctx)?;

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        b"sn",
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Observer is target.", ctx)?;

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.TARGET,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid reference frame center", ctx)?;

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        b"IAU_MOON",
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Reference frame not found", ctx)?;

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        b"IAU_M",
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid aberration correction", ctx)?;

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        b"L",
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // Test SAVE logic by repeating the call.
    //
    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        b"L",
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Relativistic aberration correction", ctx)?;
    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        b"RL",
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Stellar aberration correction w/o light time", ctx)?;

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        b"S",
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid aberration correction locus", ctx)?;

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        b"dsk",
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDLOCUS)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid reference vector.", ctx)?;

    spicelib::CLEARD(6, save.STATE.as_slice_mut());

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.STATE.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ZEROVECTOR)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid cut count.", ctx)?;

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        0,
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDCOUNT)", OK, ctx)?;

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        -1,
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDCOUNT)", OK, ctx)?;

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        (save.MAXN + 1),
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDCOUNT)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid maximum point count.", ctx)?;

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        save.SCHSTP,
        save.SOLTOL,
        0,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDSIZE)", OK, ctx)?;

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        save.SCHSTP,
        save.SOLTOL,
        -1,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDSIZE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid search tolerance.", ctx)?;

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        save.SCHSTP,
        -1.0,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDTOLERANCE)", OK, ctx)?;

    spicelib::LIMBPT(
        b"ellipsoid/tangent",
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        -1.0,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid search step.", ctx)?;

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        -1.0,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDSEARCHSTEP)", OK, ctx)?;

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        0.0,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDSEARCHSTEP)", OK, ctx)?;

    spicelib::LIMBPT(
        b"ellipsoid/tangent",
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        -1.0,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid roll step.", ctx)?;

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        0.0,
        2,
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDROLLSTEP)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No loaded SPK files", ctx)?;

    spicelib::SPKUEF(save.HANDLE[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKUEF(save.HANDLE[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKUEF(save.HANDLE[3], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOLOADEDFILES)", OK, ctx)?;

    spicelib::SPKLEF(SPK1, &mut save.HANDLE[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKLEF(SPK2, &mut save.HANDLE[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKLEF(SPK3, &mut save.HANDLE[3], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No ephemeris data for observer", ctx)?;

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        b"GASPRA",
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No ephemeris data for target", ctx)?;

    spicelib::FURNSH(DSK4, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKUEF(save.HANDLE[3], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LIMBPT(
        &save.METHOD,
        b"ALPHA",
        save.ET,
        b"ALPHAFIXED",
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    spicelib::UNLOAD(DSK4, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No orientation data for target", ctx)?;

    //
    // Fetch target radii.
    //
    spicelib::BODVCD(
        499,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Clear the kernel pool.
    //
    spicelib::CLPOOL(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Restore target radii so we can get to the error
    // condition we're looking for.
    //
    spicelib::PDPOOL(b"BODY499_RADII", 3, save.BADRAD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(FRAMEDATANOTFOUND)", OK, ctx)?;

    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(PCK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No radius data for target", ctx)?;

    //
    // We need a method that uses an ellipsoid for this one.
    //
    fstr::assign(&mut save.METHOD, b"DSK / UNPRIORITIZED / GUIDED");

    spicelib::DVPOOL(b"BODY499_RADII", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    spicelib::FURNSH(PCK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad radius data for target", ctx)?;

    //
    // Fetch original radii.
    //
    spicelib::BODVCD(
        499,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Overwrite good radii with bad in the kernel pool.
    //
    spicelib::VPACK(-1.0, 0.0, 3.0, save.BADRAD.as_slice_mut());

    spicelib::PDPOOL(b"BODY499_RADII", 3, save.BADRAD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDAXISLENGTH)", OK, ctx)?;

    //
    // Replace original radii.
    //
    spicelib::PDPOOL(b"BODY499_RADII", 3, save.RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad radius count for target", ctx)?;

    //
    // Fetch original radii.
    //
    spicelib::BODVCD(
        499,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Overwrite good radii with short array
    // in the kernel pool.
    //
    spicelib::PDPOOL(b"BODY499_RADII", 2, save.RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADRADIUSCOUNT)", OK, ctx)?;

    //
    // Replace original radii.
    //
    spicelib::PDPOOL(b"BODY499_RADII", 3, save.RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Not enough room in output arrays.", ctx)?;

    //
    // The checks on NCUTS and MAXN prevent this situation from
    // occurring in the usual case, in which there is one limb
    // point per cutting half-plane. The torus case can present
    // problems.
    //
    // Prepare LIMBPT call.
    //
    fstr::assign(&mut save.OBSRVR, b"SUN");
    fstr::assign(&mut save.TARGET, b"ALPHA");
    fstr::assign(&mut save.FIXREF, b"ALPHAFIXED");
    fstr::assign(&mut save.CORLOC, b"CENTER");
    fstr::assign(&mut save.METHOD, b"DSK/TANGENT/UNPRIORITIZED");
    save.ET = 0.0;
    save.NCUTS = 2;
    save.MAXN = 2;

    spicelib::FURNSH(DSK4, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(PCK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKLEF(SPK3, &mut save.HANDLE[3], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(OUTOFROOM)", OK, ctx)?;

    spicelib::UNLOAD(DSK4, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKUEF(save.HANDLE[3], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"GUIDED limb definition w/ ELLIPSOID LIMB correction locus.",
        ctx,
    )?;

    fstr::assign(&mut save.TARGET, b"MARS");
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.CORLOC, b"ELLIPSOID LIMB");

    fstr::assign(&mut save.METHOD, b"GUIDED/DSK/UNPRIORITIZED");

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADLIMBLOCUSMIX)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"No loaded DSKs.", ctx)?;

    spicelib::UNLOAD(DSK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::UNLOAD(DSK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::UNLOAD(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::UNLOAD(DSK3, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = ((8 as f64) * spicelib::JYEAR());

    fstr::assign(&mut save.TARGET, b"MARS");
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.ABCORR, b"NONE");

    fstr::assign(&mut save.METHOD, b"guided/dsk/unprioritized");

    spicelib::LIMBPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        save.SCHSTP,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOLOADEDDSKFILES)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Clean up.
    //
    spicelib::KCLEAR(ctx)?;

    spicelib::DELFIL(PCK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(PCK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKUEF(save.HANDLE[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DELFIL(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKUEF(save.HANDLE[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DELFIL(SPK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKUEF(save.HANDLE[3], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DELFIL(SPK3, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(DSK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DELFIL(DSK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(DSK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DELFIL(DSK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DELFIL(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(DSK3, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DELFIL(DSK3, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(DSK4, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DELFIL(DSK4, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
//***********************************************************************
//
//     S U P P O R T   U T I L I T I E S
//
//***********************************************************************
