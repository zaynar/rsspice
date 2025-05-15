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
const DSK0: &[u8] = b"termpt_dsk0.bds";
const DSK1: &[u8] = b"termpt_dsk1.bds";
const DSK2: &[u8] = b"termpt_dsk2.bds";
const DSK3: &[u8] = b"termpt_dsk3.bds";
const DSK4: &[u8] = b"termpt_dsk4.bds";
const PCK0: &[u8] = b"test_0008.tpc";
const PCK1: &[u8] = b"nat.tpc";
const SPK1: &[u8] = b"termpt_spk.bsp";
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
const NABC: i32 = 5;
const ABCLEN: i32 = 10;
const TIMLEN: i32 = 50;
const NOBS: i32 = 2;
const NTARG: i32 = 3;
const NTIMES: i32 = 5;
const SCID: i32 = -499;
const NCROSS: i32 = 400;
const NMAP: i32 = 4;
const NMETH: i32 = 12;
const NPOLYV: i32 = 100;
const NTANGT: i32 = 6;
const NLOC: i32 = 2;
const MAXCUT: i32 = 1000;
const MAXPNT: i32 = (MAXCUT * 20);
const UBPL: i32 = 4;

struct SaveVars {
    ABCS: ActualCharArray,
    ABCORR: Vec<u8>,
    CORLCS: ActualCharArray,
    CORLOC: Vec<u8>,
    FIXREF: Vec<u8>,
    ILUSRC: Vec<u8>,
    LABEL: Vec<u8>,
    METHD2: Vec<u8>,
    METHOD: Vec<u8>,
    METHDS: ActualCharArray,
    OBSRVR: Vec<u8>,
    OBSNMS: ActualCharArray,
    REFS: ActualCharArray2D,
    SPKLOC: Vec<u8>,
    ILSRCS: ActualCharArray,
    SRFNMS: ActualCharArray,
    TARGET: Vec<u8>,
    TRGFRM: Vec<u8>,
    TRGNMS: ActualCharArray,
    TITLE: Vec<u8>,
    UTC: Vec<u8>,
    A: f64,
    ALT: f64,
    ASIZE: f64,
    AXIS: StackArray<f64, 3>,
    AXPROJ: StackArray<f64, 3>,
    B: f64,
    BADRAD: StackArray<f64, 3>,
    C: f64,
    CENTER: StackArray<f64, 3>,
    CTPERP: StackArray<f64, 3>,
    CTRDIR: StackArray<f64, 3>,
    CTRVEC: StackArray<f64, 3>,
    CUTNML: StackArray<f64, 3>,
    D: f64,
    DIST: f64,
    DP: f64,
    EDEPS: ActualArray<f64>,
    EDPNTS: ActualArray2D<f64>,
    EDTANS: ActualArray2D<f64>,
    ELTS: StackArray<f64, 8>,
    EPOCHS: ActualArray<f64>,
    EPNTS: ActualArray2D<f64>,
    ET: f64,
    ET0: f64,
    ETNGTS: ActualArray2D<f64>,
    HTOL: f64,
    ILURAD: StackArray<f64, 3>,
    ISTATE: StackArray<f64, 6>,
    ITANVC: StackArray<f64, 3>,
    TPTEPC: f64,
    LT: f64,
    NORMAL: StackArray<f64, 3>,
    PLNVEC: StackArray<f64, 3>,
    PMCOEF: StackArray<f64, 3>,
    PNEAR: StackArray<f64, 3>,
    POINTS: ActualArray2D<f64>,
    PRJOFF: StackArray<f64, 3>,
    R: f64,
    RADII: StackArray<f64, 3>,
    RAYDIR: StackArray<f64, 3>,
    RCROSS: f64,
    REFVEC: StackArray<f64, 3>,
    ROLSTP: f64,
    SCHSTP: f64,
    SOLTOL: f64,
    SRCLT: f64,
    SRCPOS: StackArray<f64, 3>,
    SRCRAD: f64,
    STATE: StackArray<f64, 6>,
    STATE0: StackArray<f64, 6>,
    TANGTS: ActualArray2D<f64>,
    TANPLN: StackArray<f64, 4>,
    TANVEC: StackArray<f64, 3>,
    TDELTA: f64,
    TRMVEC: StackArray<f64, 3>,
    THETA: f64,
    TOL: f64,
    TPOINT: StackArray<f64, 3>,
    TRGLT: f64,
    TRGPOS: StackArray<f64, 3>,
    VERTEX: StackArray<f64, 3>,
    VRTOFF: StackArray<f64, 3>,
    XEPOCH: ActualArray<f64>,
    XFORM: StackArray2D<f64, 9>,
    XISRFV: StackArray<f64, 3>,
    XPOINT: StackArray<f64, 3>,
    XPT: StackArray<f64, 3>,
    XSRFVC: StackArray<f64, 3>,
    XTE: f64,
    BODYID: i32,
    HANDLE: StackArray<i32, 3>,
    ILUCDE: i32,
    K: i32,
    MAXN: i32,
    N: i32,
    NCUTS: i32,
    NEDP: ActualArray<i32>,
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
    ISPNUM: bool,
    ISTAN: bool,
    ISUMBR: bool,
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
        let mut ILUSRC = vec![b' '; NAMLEN as usize];
        let mut LABEL = vec![b' '; NAMLEN as usize];
        let mut METHD2 = vec![b' '; MTHLEN as usize];
        let mut METHOD = vec![b' '; MTHLEN as usize];
        let mut METHDS = ActualCharArray::new(MTHLEN, 1..=NMETH);
        let mut OBSRVR = vec![b' '; NAMLEN as usize];
        let mut OBSNMS = ActualCharArray::new(NAMLEN, 1..=NOBS);
        let mut REFS = ActualCharArray2D::new(NAMLEN, 1..=NREF, 1..=NTARG);
        let mut SPKLOC = vec![b' '; NAMLEN as usize];
        let mut ILSRCS = ActualCharArray::new(NAMLEN, 1..=NTARG);
        let mut SRFNMS = ActualCharArray::new(NAMLEN, 1..=NMAP);
        let mut TARGET = vec![b' '; NAMLEN as usize];
        let mut TRGFRM = vec![b' '; NAMLEN as usize];
        let mut TRGNMS = ActualCharArray::new(NAMLEN, 1..=NTARG);
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut UTC = vec![b' '; TIMLEN as usize];
        let mut A: f64 = 0.0;
        let mut ALT: f64 = 0.0;
        let mut ASIZE: f64 = 0.0;
        let mut AXIS = StackArray::<f64, 3>::new(1..=3);
        let mut AXPROJ = StackArray::<f64, 3>::new(1..=3);
        let mut B: f64 = 0.0;
        let mut BADRAD = StackArray::<f64, 3>::new(1..=3);
        let mut C: f64 = 0.0;
        let mut CENTER = StackArray::<f64, 3>::new(1..=3);
        let mut CTPERP = StackArray::<f64, 3>::new(1..=3);
        let mut CTRDIR = StackArray::<f64, 3>::new(1..=3);
        let mut CTRVEC = StackArray::<f64, 3>::new(1..=3);
        let mut CUTNML = StackArray::<f64, 3>::new(1..=3);
        let mut D: f64 = 0.0;
        let mut DIST: f64 = 0.0;
        let mut DP: f64 = 0.0;
        let mut EDEPS = ActualArray::<f64>::new(1..=MAXCUT);
        let mut EDPNTS = ActualArray2D::<f64>::new(1..=3, 1..=MAXCUT);
        let mut EDTANS = ActualArray2D::<f64>::new(1..=3, 1..=MAXCUT);
        let mut ELTS = StackArray::<f64, 8>::new(1..=8);
        let mut EPOCHS = ActualArray::<f64>::new(1..=MAXCUT);
        let mut EPNTS = ActualArray2D::<f64>::new(1..=3, 1..=MAXPNT);
        let mut ET: f64 = 0.0;
        let mut ET0: f64 = 0.0;
        let mut ETNGTS = ActualArray2D::<f64>::new(1..=3, 1..=MAXPNT);
        let mut HTOL: f64 = 0.0;
        let mut ILURAD = StackArray::<f64, 3>::new(1..=3);
        let mut ISTATE = StackArray::<f64, 6>::new(1..=6);
        let mut ITANVC = StackArray::<f64, 3>::new(1..=3);
        let mut TPTEPC: f64 = 0.0;
        let mut LT: f64 = 0.0;
        let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
        let mut PLNVEC = StackArray::<f64, 3>::new(1..=3);
        let mut PMCOEF = StackArray::<f64, 3>::new(1..=3);
        let mut PNEAR = StackArray::<f64, 3>::new(1..=3);
        let mut POINTS = ActualArray2D::<f64>::new(1..=3, 1..=MAXPNT);
        let mut PRJOFF = StackArray::<f64, 3>::new(1..=3);
        let mut R: f64 = 0.0;
        let mut RADII = StackArray::<f64, 3>::new(1..=3);
        let mut RAYDIR = StackArray::<f64, 3>::new(1..=3);
        let mut RCROSS: f64 = 0.0;
        let mut REFVEC = StackArray::<f64, 3>::new(1..=3);
        let mut ROLSTP: f64 = 0.0;
        let mut SCHSTP: f64 = 0.0;
        let mut SOLTOL: f64 = 0.0;
        let mut SRCLT: f64 = 0.0;
        let mut SRCPOS = StackArray::<f64, 3>::new(1..=3);
        let mut SRCRAD: f64 = 0.0;
        let mut STATE = StackArray::<f64, 6>::new(1..=6);
        let mut STATE0 = StackArray::<f64, 6>::new(1..=6);
        let mut TANGTS = ActualArray2D::<f64>::new(1..=3, 1..=MAXPNT);
        let mut TANPLN = StackArray::<f64, 4>::new(1..=UBPL);
        let mut TANVEC = StackArray::<f64, 3>::new(1..=3);
        let mut TDELTA: f64 = 0.0;
        let mut TRMVEC = StackArray::<f64, 3>::new(1..=3);
        let mut THETA: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut TPOINT = StackArray::<f64, 3>::new(1..=3);
        let mut TRGLT: f64 = 0.0;
        let mut TRGPOS = StackArray::<f64, 3>::new(1..=3);
        let mut VERTEX = StackArray::<f64, 3>::new(1..=3);
        let mut VRTOFF = StackArray::<f64, 3>::new(1..=3);
        let mut XEPOCH = ActualArray::<f64>::new(1..=MAXPNT);
        let mut XFORM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut XISRFV = StackArray::<f64, 3>::new(1..=3);
        let mut XPOINT = StackArray::<f64, 3>::new(1..=3);
        let mut XPT = StackArray::<f64, 3>::new(1..=3);
        let mut XSRFVC = StackArray::<f64, 3>::new(1..=3);
        let mut XTE: f64 = 0.0;
        let mut BODYID: i32 = 0;
        let mut HANDLE = StackArray::<i32, 3>::new(1..=3);
        let mut ILUCDE: i32 = 0;
        let mut K: i32 = 0;
        let mut MAXN: i32 = 0;
        let mut N: i32 = 0;
        let mut NCUTS: i32 = 0;
        let mut NEDP = ActualArray::<i32>::new(1..=MAXCUT);
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
        let mut ISPNUM: bool = false;
        let mut ISTAN: bool = false;
        let mut ISUMBR: bool = false;
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
            ]
            .into_iter();
            ABCS.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"ELLIPSOID TERMINATOR"), Val::C(b"CENTER")].into_iter();
            CORLCS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"IAU_MARS"),
                Val::C(b"IAU_PHOBOS"),
                Val::C(b"IAU_MARS"),
            ]
            .into_iter();
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

            let mut clist = [Val::C(b"Mars"), Val::C(b"PHOBOS"), Val::C(b"mars")].into_iter();
            TRGNMS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"SUN"), Val::C(b"sun"), Val::C(b"Phobos")].into_iter();
            ILSRCS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"umbral/ELLIPSOID / tangent"),
                Val::C(b"penumbral/ELLIPSOID / tangent"),
                Val::C(b"umbral/tangent/dsk/unprioritized/surfaces=\"high-res\""),
                Val::C(b"umbral/tangent/dsk/unprioritized/surfaces=\"LOW-RES\""),
                Val::C(b"penumbral/tangent/dsk/unprioritized/surfaces=\"high-res\""),
                Val::C(b"penumbral/tangent/dsk/unprioritized/surfaces=\"LOW-RES\""),
                Val::C(b"umbral/guided/ellipsoid"),
                Val::C(b"penumbral/guided/ellipsoid"),
                Val::C(b"umbral / dsk/ guided /unprioritized/surfaces=\"high-res\""),
                Val::C(b"penumbral/dsk/ guided /unprioritized/surfaces=\"high-res\""),
                Val::C(b"umbral/dsk/ guided /unprioritized/surfaces=\"LOW-RES\""),
                Val::C(b"penumbral/dsk/ guided /unprioritized/surfaces=\"LOW-RES\""),
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
            ILUSRC,
            LABEL,
            METHD2,
            METHOD,
            METHDS,
            OBSRVR,
            OBSNMS,
            REFS,
            SPKLOC,
            ILSRCS,
            SRFNMS,
            TARGET,
            TRGFRM,
            TRGNMS,
            TITLE,
            UTC,
            A,
            ALT,
            ASIZE,
            AXIS,
            AXPROJ,
            B,
            BADRAD,
            C,
            CENTER,
            CTPERP,
            CTRDIR,
            CTRVEC,
            CUTNML,
            D,
            DIST,
            DP,
            EDEPS,
            EDPNTS,
            EDTANS,
            ELTS,
            EPOCHS,
            EPNTS,
            ET,
            ET0,
            ETNGTS,
            HTOL,
            ILURAD,
            ISTATE,
            ITANVC,
            TPTEPC,
            LT,
            NORMAL,
            PLNVEC,
            PMCOEF,
            PNEAR,
            POINTS,
            PRJOFF,
            R,
            RADII,
            RAYDIR,
            RCROSS,
            REFVEC,
            ROLSTP,
            SCHSTP,
            SOLTOL,
            SRCLT,
            SRCPOS,
            SRCRAD,
            STATE,
            STATE0,
            TANGTS,
            TANPLN,
            TANVEC,
            TDELTA,
            TRMVEC,
            THETA,
            TOL,
            TPOINT,
            TRGLT,
            TRGPOS,
            VERTEX,
            VRTOFF,
            XEPOCH,
            XFORM,
            XISRFV,
            XPOINT,
            XPT,
            XSRFVC,
            XTE,
            BODYID,
            HANDLE,
            ILUCDE,
            K,
            MAXN,
            N,
            NCUTS,
            NEDP,
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
            ISPNUM,
            ISTAN,
            ISUMBR,
            USECN,
            USELT,
        }
    }
}

//$Procedure      F_TERMPT ( TERMPT family tests )
pub fn F_TERMPT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_TERMPT", ctx)?;

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
    // Optionally modify Phobos' radii to make the shape
    // even more non-spherical.
    //
    //  CALL VPACK ( 15.D0, 50.D0, 2.D0, RADII )
    //  CALL PDPOOL ( 'BODY401_RADII', 3, RADII )
    //  CALL CHCKXC ( .FALSE., ' ', OK )

    //
    // Optionally make Phobos round.
    //
    //  CALL VPACK ( 10.D0, 10.D0, 10.D0, RADII )
    //  CALL PDPOOL ( 'BODY401_RADII', 3, RADII )
    // CALL CHCKXC ( .FALSE., ' ', OK )

    //
    // Optionally modify Mars' radii to make the shape
    // even more non-spherical.
    //
    //  CALL VPACK ( 3.D3, 5.D3, 2.D3, RADII )
    //  CALL PDPOOL ( 'BODY499_RADII', 3, RADII )
    //  CALL CHCKXC ( .FALSE., ' ', OK )

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

    // Loop over every choice of observer.
    //
    for OBSIDX in 1..=NOBS {
        fstr::assign(&mut save.OBSRVR, save.OBSNMS.get(OBSIDX));
        //
        // Set the observer ID code.
        //
        spicelib::BODN2C(&save.OBSRVR, &mut save.OBSCDE, &mut save.FOUND, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Loop over every target-illumination source pair.
        //
        for TRGIDX in 1..=NTARG {
            fstr::assign(&mut save.TARGET, save.TRGNMS.get(TRGIDX));
            fstr::assign(&mut save.ILUSRC, save.ILSRCS.get(TRGIDX));

            spicelib::BODS2C(&save.ILUSRC, &mut save.ILUCDE, &mut save.FOUND, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

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

            save.A = save.RADII[1];
            save.B = save.RADII[2];
            save.C = save.RADII[3];

            //
            // Get source radii.
            //
            spicelib::BODVAR(
                save.ILUCDE,
                b"RADII",
                &mut save.N,
                save.ILURAD.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            save.SRCRAD = save.ILURAD[1];

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
                            // The locus is 'ELLIPSOID TERMINATOR'. Only
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

                            save.ISPNUM =
                                spicelib::MATCHI(&save.METHOD, b"*PENUMBRAL*", b"*", b"?", ctx);

                            save.ISUMBR = !save.ISPNUM;

                            //
                            // --- Case: ------------------------------------------------------
                            //

                            fstr::assign(&mut save.TITLE, b"Observer = #; Target = #; Source = #; ABCORR = #; TRGFRM = #; METHOD = #; CORLOC = #.");
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
                                &save.ILUSRC,
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

                            // CALL TOSTDO ( '<<<<<<<<<<<<<<<<<<<<<<>>>>>>>>' )
                            // CALL TOSTDO ( TITLE )

                            //
                            // Start off by computing the set of terminator
                            // points We'll then check the results.
                            //
                            save.NCUTS = 1;

                            save.ROLSTP = (spicelib::TWOPI(ctx) / save.NCUTS as f64);
                            spicelib::VPACK(0.0, 0.0, 1.0, save.REFVEC.as_slice_mut());

                            //
                            // We expect a single terminator point in each
                            // half plane, so we can use a large step.
                            //
                            save.SCHSTP = 4.0;

                            //
                            // Derive the solution tolerance from the height
                            // error tolerance and the target-source distance.
                            // Ideally the input epoch should be corrected for
                            // observer-target light time, but this is not
                            // essential.
                            //
                            spicelib::SPKPOS(
                                &save.ILUSRC,
                                save.ET,
                                &save.TRGFRM,
                                &save.ABCORR,
                                &save.TARGET,
                                save.SRCPOS.as_slice_mut(),
                                &mut save.SRCLT,
                                ctx,
                            )?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            save.DIST = spicelib::VNORM(save.SRCPOS.as_slice());

                            save.HTOL = 0.0001;
                            save.SOLTOL = (save.HTOL / save.DIST);

                            save.MAXN = MAXPNT;

                            //
                            // If the shape is DSK and the locus is ELLIPSOID
                            // terminator, make a preliminary call to obtain
                            // the expected epochs.
                            //
                            if (save.ISDSK
                                && spicelib::EQSTR(&save.CORLOC, b"ELLIPSOID TERMINATOR"))
                            {
                                //
                                // Create a version of the method string with
                                // an ellipsoid shape specification.
                                //
                                if save.ISUMBR {
                                    fstr::assign(&mut save.METHD2, b"ELLIPSOID/TANGENT/UMBRAL");
                                } else {
                                    fstr::assign(&mut save.METHD2, b"ELLIPSOID/TANGENT/PENUMBRAL");
                                }
                                //
                                // Get the expected epochs for the call below
                                // that uses a DSK target shape.
                                //
                                spicelib::TERMPT(
                                    &save.METHD2,
                                    &save.ILUSRC,
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

                            spicelib::TERMPT(
                                &save.METHOD,
                                &save.ILUSRC,
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
                                // there should be just one terminator point per
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
                                    // We'll treat the Jth terminator point as an
                                    // ephemeris object and find its position
                                    // relative to the observer.
                                    //
                                    spicelib::VEQU(
                                        save.POINTS.subarray([1, save.K]),
                                        save.TPOINT.as_slice_mut(),
                                    );
                                    spicelib::VEQU(
                                        save.TANGTS.subarray([1, save.K]),
                                        save.TRMVEC.as_slice_mut(),
                                    );

                                    save.TPTEPC = save.EPOCHS[I];

                                    //
                                    // Get an inertially referenced version
                                    // of the tangent vector.
                                    //
                                    spicelib::PXFORM(
                                        b"J2000",
                                        &save.TRGFRM,
                                        save.TPTEPC,
                                        save.XFORM.as_slice_mut(),
                                        ctx,
                                    )?;
                                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                                    spicelib::MTXV(
                                        save.XFORM.as_slice(),
                                        save.TRMVEC.as_slice(),
                                        save.ITANVC.as_slice_mut(),
                                    );

                                    //
                                    // Set the SPK lookup locus to be compatible
                                    // with the aberration correction locus used
                                    // with the terminator point.
                                    //
                                    if fstr::eq(&save.CORLOC, b"CENTER") {
                                        //
                                        // In this case we're using aberration
                                        // corrections for the target body
                                        // center. We must compute the expected
                                        // state of the terminator point
                                        // "manually."
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
                                            save.TPOINT.as_slice(),
                                            save.XSRFVC.as_slice_mut(),
                                        );
                                        //
                                        // For the 'CENTER' locus, the epochs
                                        // associated with terminator points are
                                        // all set to the epoch associated with
                                        // the target body's center.
                                        //
                                        save.LT = save.TRGLT;
                                        //
                                        // Get an inertially referenced version
                                        // of the expected tangent vector.
                                        //
                                        spicelib::PXFORM(
                                            b"J2000",
                                            &save.TRGFRM,
                                            save.TPTEPC,
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
                                        // This is the 'ellipsoid terminator'
                                        // case.
                                        //
                                        fstr::assign(&mut save.SPKLOC, b"TARGET");

                                        spicelib::SPKCPT(
                                            save.TPOINT.as_slice(),
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
                                            save.TPOINT.as_slice(),
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
                                    // If TPOINT is correct, then the position
                                    // of TPOINT relative to the observer should
                                    // be equal to TRMVEC. The light time
                                    // obtained from SPKCPT or computed manually
                                    // should match that implied by TPTEPC.
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

                                    if (save.ISDSK
                                        && fstr::eq(&save.CORLOC, b"ELLIPSOID TERMINATOR"))
                                    {
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

                                    fstr::assign(&mut save.LABEL, b"TPTEPC PLANE #, PT #");
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
                                        save.TPTEPC,
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
                                        // TERMPT is for the corresponding
                                        // ellipsoid terminator point.
                                        //
                                        save.TOL = (save.TOL * 1000.0);
                                    }

                                    fstr::assign(&mut save.LABEL, b"TRMVEC PLANE #, PT #");
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
                                            save.TRMVEC.as_slice(),
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
                                        fstr::assign(&mut save.LABEL, b"TRMVEC PLANE #, PT #");
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
                                            save.TRMVEC.as_slice(),
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
                                        fstr::assign(&mut save.LABEL, b"TRMVEC PLANE #, PT #");
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
                                            save.TRMVEC.as_slice(),
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
                                        // for centimeter-level agreement.
                                        //
                                        fstr::assign(&mut save.LABEL, b"TRMVEC PLANE #, PT #");
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
                                            save.TRMVEC.as_slice(),
                                            b"~~",
                                            save.XSRFVC.as_slice(),
                                            3,
                                            0.00001,
                                            OK,
                                            ctx,
                                        )?;
                                    }

                                    //
                                    // We've checked the consistency of TPOINT,
                                    // TRMVEC, and TPTEPC, but we haven't done
                                    // anything to show that TPOINT is a
                                    // terminator point. Do this now.
                                    //
                                    // We need to verify that
                                    //
                                    //    1)  The terminator point is on the
                                    //        target body's surface.
                                    //
                                    //    2)  For combinations of "tangent"
                                    //        methods and DSK target shapes, the
                                    //        terminator point is a point of
                                    //        tangency on the target of a ray
                                    //        emanating from the surface of the
                                    //        source.
                                    //
                                    //        For combinations of "tangent"
                                    //        methods and ellipsoid target
                                    //        shapes, the terminator point is a
                                    //        point of tangency on the target of
                                    //        a plane emanating from the surface
                                    //        of the source.
                                    //
                                    //        For the "guided" methods, the
                                    //        terminator point lies on a ray
                                    //        emanating from the center of the
                                    //        reference ellipsoid.
                                    //
                                    //        When the target shape is an
                                    //        ellipsoid, there should be no
                                    //        difference between results
                                    //        obtained using the tangent and
                                    //        guided methods.
                                    //
                                    //    3)  The terminator point lies in the
                                    //        correct half-plane.
                                    //
                                    //
                                    // Check the terminator point's distance
                                    // from the target surface.
                                    //
                                    if save.ISELL {
                                        //
                                        // Find the altitude of the terminator
                                        // point with respect to the reference
                                        // ellipsoid.
                                        //
                                        spicelib::NEARPT(
                                            save.TPOINT.as_slice(),
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

                                        fstr::assign(&mut save.LABEL, b"TPOINT ALT, PLANE #, PT #");
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
                                        // that ideally passes through the
                                        // terminator point, and we'll find the
                                        // ray-surface intercept. This intercept
                                        // should be close to the original
                                        // terminator point.
                                        //
                                        spicelib::VSCL(
                                            10.0,
                                            save.TPOINT.as_slice(),
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
                                        // Our ray should hit the DSK surface
                                        // very close to the terminator point.
                                        // One millimeter is enough margin.
                                        //
                                        save.TOL = 0.000001;

                                        save.DIST = spicelib::VDIST(
                                            save.XPT.as_slice(),
                                            save.TPOINT.as_slice(),
                                        );

                                        fstr::assign(
                                            &mut save.LABEL,
                                            b"TPOINT DIST, PLANE #, PT #",
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
                                    // Verify that the putative terminator point
                                    // is really on the terminator.
                                    //
                                    // Create the central axis and normal to the
                                    // current cutting half-plane. We'll use
                                    // these for the DSK case below, and for the
                                    // next set of tests, in which the terminator
                                    // point is tested for inclusion in the
                                    // cutting half-plane.
                                    //
                                    spicelib::SPKPOS(
                                        &save.ILUSRC,
                                        save.TPTEPC,
                                        &save.TRGFRM,
                                        &save.ABCORR,
                                        &save.TARGET,
                                        save.AXIS.as_slice_mut(),
                                        &mut save.LT,
                                        ctx,
                                    )?;
                                    testutil::CHCKXC(false, b" ", OK, ctx)?;

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
                                    spicelib::UCRSS(
                                        save.CUTNML.as_slice(),
                                        save.AXIS.as_slice(),
                                        save.PLNVEC.as_slice_mut(),
                                    );

                                    //
                                    if save.ISELL {
                                        //
                                        // This is the ellipsoid case.
                                        //
                                        // If TPOINT is all it's claimed to be,
                                        // the tangent plane at TPOINT should be
                                        // tangent to the source as well.
                                        //
                                        spicelib::SURFNM(
                                            save.RADII[1],
                                            save.RADII[2],
                                            save.RADII[3],
                                            save.TPOINT.as_slice(),
                                            save.NORMAL.as_slice_mut(),
                                            ctx,
                                        )?;
                                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                                        spicelib::NVP2PL(
                                            save.NORMAL.as_slice(),
                                            save.TPOINT.as_slice(),
                                            save.TANPLN.as_slice_mut(),
                                            ctx,
                                        )?;
                                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                                        //
                                        // Find the distance between the center
                                        // of the source and the tangent plane.
                                        // The distance should match the source
                                        // radius (the source is considered to be
                                        // a sphere).
                                        //
                                        spicelib::VPRJP(
                                            save.AXIS.as_slice(),
                                            save.TANPLN.as_slice(),
                                            save.AXPROJ.as_slice_mut(),
                                            ctx,
                                        )?;

                                        save.D = spicelib::VDIST(
                                            save.AXIS.as_slice(),
                                            save.AXPROJ.as_slice(),
                                        );

                                        if !save.USELT {
                                            save.TOL = TIGHT;
                                        } else if save.USECN {
                                            save.TOL = 0.0000001;
                                        } else {
                                            //
                                            // This test is quite sensitive to the
                                            // error in the terminator point
                                            // location. We don't expect much with
                                            // an unconverged light time solution.
                                            //
                                            save.TOL = 0.001;
                                        }

                                        fstr::assign(
                                            &mut save.LABEL,
                                            b"||AXIS-AXPROJ||, PLANE #, PT #",
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
                                            b"~/",
                                            save.SRCRAD,
                                            save.TOL,
                                            OK,
                                            ctx,
                                        )?;

                                        //
                                        // We're not done yet. Make sure the
                                        // tangent point on the source is on the
                                        // correct side of the source.
                                        //
                                        if save.ISUMBR {
                                            //
                                            // The terminator type is umbral. The
                                            // tangent point on the source should
                                            // be on the same side as the tangent
                                            // point on the target.
                                            //
                                            spicelib::VSUB(
                                                save.AXPROJ.as_slice(),
                                                save.AXIS.as_slice(),
                                                save.PRJOFF.as_slice_mut(),
                                            );

                                            save.DP = spicelib::VDOT(
                                                save.PRJOFF.as_slice(),
                                                save.NORMAL.as_slice(),
                                            );

                                            testutil::CHCKSD(
                                                &save.LABEL,
                                                save.DP,
                                                b">",
                                                0.0,
                                                0.0,
                                                OK,
                                                ctx,
                                            )?;
                                        } else {
                                            //
                                            // The terminator type is penumbral.
                                            // The tangent point on the source
                                            // should be on the opposite side from
                                            // the tangent point on the target.
                                            //
                                            spicelib::VSUB(
                                                save.AXPROJ.as_slice(),
                                                save.AXIS.as_slice(),
                                                save.PRJOFF.as_slice_mut(),
                                            );

                                            save.DP = spicelib::VDOT(
                                                save.PRJOFF.as_slice(),
                                                save.NORMAL.as_slice(),
                                            );

                                            testutil::CHCKSD(
                                                &save.LABEL,
                                                save.DP,
                                                b"<",
                                                0.0,
                                                0.0,
                                                OK,
                                                ctx,
                                            )?;
                                        }
                                    } else {
                                        //
                                        // This is the DSK case.
                                        //
                                        // We could check the relative
                                        // orientation of the outward normal and
                                        // the associated tangent vector, but we
                                        // can't expect these vectors to be
                                        // orthogonal.
                                        //
                                        // We can perform a more accurate check
                                        // by determining whether a small
                                        // rotation of the ray defined by the
                                        // illumination source and the tangent
                                        // vector, performed within the cutting
                                        // half-plane, will move the ray off of
                                        // the target.

                                        if save.ISTAN {
                                            //
                                            // This is the DSK tangent case.
                                            //
                                            // We need to locate the vertex of the
                                            // tangent ray. This is a surface point
                                            // on the source; the line containing
                                            // the tangent ray is tangent to the
                                            // source.
                                            //
                                            spicelib::VSUB(
                                                save.AXIS.as_slice(),
                                                save.TPOINT.as_slice(),
                                                save.CTRVEC.as_slice_mut(),
                                            );

                                            spicelib::VHAT(
                                                save.CTRVEC.as_slice(),
                                                save.CTRDIR.as_slice_mut(),
                                            );

                                            spicelib::UCRSS(
                                                save.CUTNML.as_slice(),
                                                save.CTRVEC.as_slice(),
                                                save.CTPERP.as_slice_mut(),
                                            );

                                            save.ASIZE = f64::asin(
                                                (save.SRCRAD
                                                    / spicelib::VNORM(save.CTRVEC.as_slice())),
                                            );

                                            if save.ISUMBR {
                                                spicelib::VLCOM(
                                                    (save.SRCRAD * f64::cos(save.ASIZE)),
                                                    save.CTPERP.as_slice(),
                                                    -(save.SRCRAD * f64::sin(save.ASIZE)),
                                                    save.CTRDIR.as_slice(),
                                                    save.VRTOFF.as_slice_mut(),
                                                );
                                            } else {
                                                spicelib::VLCOM(
                                                    -(save.SRCRAD * f64::cos(save.ASIZE)),
                                                    save.CTPERP.as_slice(),
                                                    -(save.SRCRAD * f64::sin(save.ASIZE)),
                                                    save.CTRDIR.as_slice(),
                                                    save.VRTOFF.as_slice_mut(),
                                                );
                                            }

                                            spicelib::VADD(
                                                save.AXIS.as_slice(),
                                                save.VRTOFF.as_slice(),
                                                save.VERTEX.as_slice_mut(),
                                            );

                                            //
                                            // Rotate the tangent ray away from
                                            // the axis by 2x the angular
                                            // search tolerance. Presuming the ray
                                            // was nearly tangential, it should
                                            // now point off the target.
                                            //
                                            save.THETA = -((2 as f64) * save.SOLTOL);

                                            spicelib::VSUB(
                                                save.TPOINT.as_slice(),
                                                save.VERTEX.as_slice(),
                                                save.TANVEC.as_slice_mut(),
                                            );

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
                                                save.VERTEX.as_slice(),
                                                save.RAYDIR.as_slice(),
                                                save.XPT.as_slice_mut(),
                                                std::slice::from_mut(&mut save.FOUND),
                                                ctx,
                                            )?;
                                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                                            fstr::assign(
                                                &mut save.LABEL,
                                                b"(miss) Ray hit PLANE #, PT #",
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

                                            testutil::CHCKSL(
                                                &save.LABEL,
                                                save.FOUND,
                                                false,
                                                OK,
                                                ctx,
                                            )?;

                                            //
                                            // Make sure we didn't miss because we
                                            // were way off the mark. :) The
                                            // opposite rotation should yield a
                                            // hit.
                                            //
                                            save.THETA = -save.THETA;

                                            spicelib::VROTV(
                                                save.TANVEC.as_slice(),
                                                save.CUTNML.as_slice(),
                                                save.THETA,
                                                save.RAYDIR.as_slice_mut(),
                                            );

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

                                            fstr::assign(
                                                &mut save.LABEL,
                                                b"(hit) Ray hit PLANE #, PT #",
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

                                            testutil::CHCKSL(
                                                &save.LABEL,
                                                save.FOUND,
                                                true,
                                                OK,
                                                ctx,
                                            )?;
                                        } else {
                                            //
                                            // This is the DSK "guided" case.
                                            //
                                            // We've already checked that the
                                            // terminator point is on the DSK
                                            // surface; we need to make sure it's
                                            // on a ray emanating from the target
                                            // center and passing through a
                                            // terminator point on the reference
                                            // ellipsoid. The last check we'll do
                                            // will ensure the point is in the
                                            // correct cutting half-plane.
                                            //
                                            // We'll trust that TERMPT works
                                            // correctly for ellipsoidal targets.
                                            //
                                            // Find the ellipsoid terminator point
                                            // on the ray emanating from the origin
                                            // and passing through the current
                                            // terminator point. To do this, find
                                            // the reference ellipsoid intercept
                                            // of a ray emanating from a point
                                            // outside the target, passing through
                                            // the DSK terminator point, and
                                            // hitting the target center.
                                            //
                                            spicelib::VSCL(
                                                10.0,
                                                save.TPOINT.as_slice(),
                                                save.VERTEX.as_slice_mut(),
                                            );
                                            spicelib::VMINUS(
                                                save.VERTEX.as_slice(),
                                                save.RAYDIR.as_slice_mut(),
                                            );

                                            spicelib::SURFPT(
                                                save.VERTEX.as_slice(),
                                                save.RAYDIR.as_slice(),
                                                save.A,
                                                save.B,
                                                save.C,
                                                save.XPT.as_slice_mut(),
                                                &mut save.FOUND,
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
                                            // Find the terminator points on
                                            // the ellipsoid.
                                            //
                                            if save.ISUMBR {
                                                fstr::assign(
                                                    &mut save.METHD2,
                                                    b"ELLIPSOID/TANGENT/UMBRAL",
                                                );
                                            } else {
                                                fstr::assign(
                                                    &mut save.METHD2,
                                                    b"ELLIPSOID/TANGENT/PENUMBRAL",
                                                );
                                            }

                                            spicelib::TERMPT(
                                                &save.METHD2,
                                                &save.ILUSRC,
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
                                                save.NEDP.as_slice_mut(),
                                                save.EDPNTS.as_slice_mut(),
                                                save.EDEPS.as_slice_mut(),
                                                save.EDTANS.as_slice_mut(),
                                                ctx,
                                            )?;
                                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                                            testutil::CHCKSI(
                                                b"NEDP",
                                                save.NEDP[I],
                                                b"=",
                                                1,
                                                0,
                                                OK,
                                                ctx,
                                            )?;

                                            fstr::assign(
                                                &mut save.LABEL,
                                                b"TPOINT ERROR, PLANE #, PT #",
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

                                            save.TOL = TIGHT;

                                            testutil::CHCKAD(
                                                &save.LABEL,
                                                save.XPT.as_slice(),
                                                b"~~/",
                                                save.EDPNTS.subarray([1, I]),
                                                3,
                                                save.TOL,
                                                OK,
                                                ctx,
                                            )?;
                                        }
                                    }

                                    //
                                    // Verify that the terminator point is in the
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
                                        save.TPOINT.as_slice(),
                                    );

                                    //
                                    // The tolerances below are absolute (km).
                                    //
                                    if !save.USELT {
                                        save.TOL = 0.0000001;
                                    } else if save.USECN {
                                        save.TOL = 0.000001;
                                    } else {
                                        save.TOL = 0.01;
                                    }

                                    fstr::assign(
                                        &mut save.LABEL,
                                        b"<TPOINT, CUTNML>, PLANE #, PT #",
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
                                    // The terminator point must be on the
                                    // correct side of the axis, in order to
                                    // be in the correct half-plane.
                                    //
                                    save.D = spicelib::VDOT(
                                        save.TPOINT.as_slice(),
                                        save.PLNVEC.as_slice(),
                                    );

                                    fstr::assign(
                                        &mut save.LABEL,
                                        b"<TPOINT, PLNVEC>, PLANE #, PT #",
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

                                    testutil::CHCKSD(&save.LABEL, save.D, b">", 0.0, 0.0, OK, ctx)?;

                                    //
                                    // Update K to point to the next terminator
                                    // point.
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

    spicelib::BODVCD(
        10,
        b"RADII",
        3,
        &mut save.N,
        save.ILURAD.as_slice_mut(),
        ctx,
    )?;
    save.SRCRAD = save.ILURAD[1];

    //
    // Create DSK containing two nested tori.
    //
    if spicelib::EXISTS(DSK4, ctx)? {
        spicelib::DELFIL(DSK4, ctx)?;
    }

    save.NSURF = 2;
    save.SRFIDS[1] = 3;
    save.SRFIDS[2] = 4;
    fstr::assign(&mut save.TRGFRM, b"ALPHA_VIEW_XY");

    save.R = 70000.0;
    save.RCROSS = 10000.0;

    spicelib::VPACK(0.0, 0.0, 0.0, save.CENTER.as_slice_mut());
    spicelib::VPACK(1.0, 0.0, 0.0, save.NORMAL.as_slice_mut());

    testutil::T_TORUS(
        ALPHA,
        save.SRFIDS[1],
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
        save.SRFIDS[2],
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
    // Prepare TERMPT call.
    //
    fstr::assign(&mut save.OBSRVR, b"SUN");
    fstr::assign(&mut save.ILUSRC, b"SUN");
    fstr::assign(&mut save.TARGET, b"ALPHA");
    fstr::assign(&mut save.CORLOC, b"CENTER");
    fstr::assign(
        &mut save.METHOD,
        b"DSK/UMBRAL/TANGENT/UNPRIORITIZED/SURFACES=3,4",
    );
    fstr::assign(&mut save.METHOD, b"DSK/UMBRAL/TANGENT/UNPRIORITIZED");
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
            // Start off by computing the set of terminator points
            // We'll then check the results.
            //
            save.NCUTS = 10;
            save.NCUTS = 1;
            save.ROLSTP = (spicelib::TWOPI(ctx) / save.NCUTS as f64);
            spicelib::VPACK(0.0, 0.0, 1.0, save.REFVEC.as_slice_mut());

            //
            // We expect a multiple terminator points in each half
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

            spicelib::TERMPT(
                &save.METHOD,
                &save.ILUSRC,
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
                // There should be 4 terminator points in each cutting
                // half-plane.
                //
                fstr::assign(&mut save.LABEL, b"NPTS HALF-PLANE #,");
                spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);

                testutil::CHCKSI(&save.LABEL, save.NPTS[I], b"=", 4, 0, OK, ctx)?;

                for J in 1..=save.NPTS[I] {
                    //
                    // We'll treat the Jth terminator point as an ephemeris
                    // object and find its position relative to the
                    // observer.

                    spicelib::VEQU(
                        save.POINTS.subarray([1, save.K]),
                        save.TPOINT.as_slice_mut(),
                    );
                    spicelib::VEQU(
                        save.TANGTS.subarray([1, save.K]),
                        save.TRMVEC.as_slice_mut(),
                    );

                    save.TPTEPC = save.EPOCHS[I];

                    //
                    // Get an inertially referenced version
                    // of the tangent vector.
                    //
                    spicelib::PXFORM(
                        b"J2000",
                        &save.TRGFRM,
                        save.TPTEPC,
                        save.XFORM.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::MTXV(
                        save.XFORM.as_slice(),
                        save.TRMVEC.as_slice(),
                        save.ITANVC.as_slice_mut(),
                    );
                    //
                    // In this case we're using aberration corrections for
                    // the target body center. We must compute the expected
                    // state of the terminator point "manually."

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
                        save.TPOINT.as_slice(),
                        save.XSRFVC.as_slice_mut(),
                    );
                    //
                    // For the 'CENTER' locus, the epochs associated with
                    // terminator points are all set to the epoch associated
                    // with the target body's center.
                    //
                    // Compute the expected epoch from the SPKPOS call.
                    //
                    spicelib::ZZCOREPC(&save.ABCORR, save.ET, save.TRGLT, &mut save.XTE, ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    fstr::assign(&mut save.LABEL, b"TPTEPC PLANE #, PT #");
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);

                    testutil::CHCKSD(&save.LABEL, save.TPTEPC, b"~/", save.XTE, save.TOL, OK, ctx)?;

                    //
                    // Get an inertially referenced version of the expected
                    // tangent vector.
                    //
                    spicelib::PXFORM(
                        b"J2000",
                        &save.TRGFRM,
                        save.TPTEPC,
                        save.XFORM.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::MTXV(
                        save.XFORM.as_slice(),
                        save.XSRFVC.as_slice(),
                        save.XISRFV.as_slice_mut(),
                    );

                    //
                    // Compare the inertially referenced tangent vectors.
                    //
                    fstr::assign(&mut save.LABEL, b"ITANVC PLANE #, PT #");
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);

                    save.TOL = TIGHT;
                    //
                    // Use a tighter tolerance for the inertial vector test.
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
                    fstr::assign(&mut save.LABEL, b"TRMVEC PLANE #, PT #");
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);

                    testutil::CHCKAD(
                        &save.LABEL,
                        save.TRMVEC.as_slice(),
                        b"~~/",
                        save.XSRFVC.as_slice(),
                        3,
                        save.TOL,
                        OK,
                        ctx,
                    )?;

                    //
                    // We've checked the consistency of TPOINT, TRMVEC, and
                    // TPTEPC, but we haven't done anything to show that
                    // TPOINT is a terminator point. Do this now.
                    //
                    // We need to verify that
                    //
                    // 1)  The terminator point is on the target body's
                    //     surface.
                    //
                    // 2)  For combinations of "tangent" methods and DSK
                    //     target shapes, the terminator point is a point of
                    //     tangency on the target of a ray emanating from
                    //     the surface of the source.
                    //
                    //     For combinations of "tangent" methods and
                    //     ellipsoid target shapes, the terminator point is
                    //     a point of tangency on the target of a plane
                    //     emanating from the surface of the source.
                    //
                    //     For the "guided" methods, the terminator point
                    //     lies on a ray emanating from the center of the
                    //     reference ellipsoid.
                    //
                    //     When the target shape is an ellipsoid, there
                    //     should be no difference between results obtained
                    //     using the tangent and
                    //
                    // 3)  The terminator point lies in the correct
                    //     half-plane.
                    //
                    //     Check the terminator point's distance from the
                    //     target surface.
                    //
                    //     We'll create an inward-pointing ray that ideally
                    //     passes through the terminator point, and we'll
                    //     find the ray-surface intercept. This intercept
                    //     should be close to the original terminator point.
                    //
                    //     Because we're working with a point on the surface
                    //     of a set of nested tori, we can't just scale the
                    //     point to create a ray vertex. We'll use the
                    //     outward normal at the point.
                    //
                    spicelib::SRFNRM(
                        b"DSK/UNPRIORITIZED/SURFACES = 3,4",
                        &save.TARGET,
                        save.ET,
                        &save.TRGFRM,
                        1,
                        save.TPOINT.as_slice(),
                        save.NORMAL.as_slice_mut(),
                        ctx,
                    )?;

                    spicelib::VLCOM(
                        1.0,
                        save.TPOINT.as_slice(),
                        10.0,
                        save.NORMAL.as_slice(),
                        save.VERTEX.as_slice_mut(),
                    );
                    spicelib::VMINUS(save.NORMAL.as_slice(), save.RAYDIR.as_slice_mut());

                    spicelib::DSKXV(
                        false,
                        &save.TARGET,
                        2,
                        save.SRFIDS.as_slice(),
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
                    // Our ray should hit the DSK surface very close to the
                    // terminator point. One millimeter is enough margin.
                    //
                    save.TOL = 0.000001;

                    save.DIST = spicelib::VDIST(save.XPT.as_slice(), save.TPOINT.as_slice());

                    fstr::assign(&mut save.LABEL, b"TPOINT DIST, PLANE #, PT #");
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);

                    testutil::CHCKSD(&save.LABEL, save.DIST, b"~", 0.0, save.TOL, OK, ctx)?;

                    //
                    // Verify that the putative terminator point is really
                    // on the terminator.
                    //
                    // Create the central axis and normal to the current
                    // cutting half-plane. We'll use these for the DSK case
                    // below, and for the next set of tests, in which the
                    // terminator point is tested for inclusion in the
                    // cutting half-plane.
                    //
                    spicelib::SPKPOS(
                        &save.ILUSRC,
                        save.TPTEPC,
                        &save.TRGFRM,
                        &save.ABCORR,
                        &save.TARGET,
                        save.AXIS.as_slice_mut(),
                        &mut save.LT,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

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
                    spicelib::UCRSS(
                        save.CUTNML.as_slice(),
                        save.AXIS.as_slice(),
                        save.PLNVEC.as_slice_mut(),
                    );

                    //
                    // This is the DSK tangent case.
                    //
                    // We need to locate the vertex of the tangent ray.
                    // This is a surface point on the source; the line
                    // containing the tangent ray is tangent to the source.
                    //
                    spicelib::VSUB(
                        save.AXIS.as_slice(),
                        save.TPOINT.as_slice(),
                        save.CTRVEC.as_slice_mut(),
                    );

                    spicelib::VHAT(save.CTRVEC.as_slice(), save.CTRDIR.as_slice_mut());

                    spicelib::UCRSS(
                        save.CUTNML.as_slice(),
                        save.CTRVEC.as_slice(),
                        save.CTPERP.as_slice_mut(),
                    );

                    save.ASIZE = f64::asin((save.SRCRAD / spicelib::VNORM(save.CTRVEC.as_slice())));

                    //
                    // We're working only with umbral shadows.
                    //
                    spicelib::VLCOM(
                        (save.SRCRAD * f64::cos(save.ASIZE)),
                        save.CTPERP.as_slice(),
                        -(save.SRCRAD * f64::sin(save.ASIZE)),
                        save.CTRDIR.as_slice(),
                        save.VRTOFF.as_slice_mut(),
                    );

                    spicelib::VADD(
                        save.AXIS.as_slice(),
                        save.VRTOFF.as_slice(),
                        save.VERTEX.as_slice_mut(),
                    );

                    //
                    // For the odd-indexed terminator points, rotate the
                    // tangent ray away from the axis by 2x the angular
                    // search tolerance. For the even-indexed points, rotate
                    // the ray toward the axis. Presuming the ray was nearly
                    // tangential, it should now point off the target.
                    //
                    if spicelib::ODD(J) {
                        save.THETA = -((2 as f64) * save.SOLTOL);
                    } else {
                        save.THETA = ((2 as f64) * save.SOLTOL);
                    }

                    spicelib::VSUB(
                        save.TPOINT.as_slice(),
                        save.VERTEX.as_slice(),
                        save.TANVEC.as_slice_mut(),
                    );

                    spicelib::VROTV(
                        save.TANVEC.as_slice(),
                        save.CUTNML.as_slice(),
                        save.THETA,
                        save.RAYDIR.as_slice_mut(),
                    );

                    spicelib::DSKXV(
                        false,
                        &save.TARGET,
                        save.NSURF,
                        save.SRFIDS.as_slice(),
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

                    fstr::assign(&mut save.LABEL, b"(miss) Ray hit PLANE #, PT #");
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);

                    testutil::CHCKSL(&save.LABEL, save.FOUND, false, OK, ctx)?;

                    //
                    // Make sure we didn't miss because we were way off
                    // the mark. :) The opposite rotation should yield
                    // a hit.
                    //
                    save.THETA = -save.THETA;

                    spicelib::VROTV(
                        save.TANVEC.as_slice(),
                        save.CUTNML.as_slice(),
                        save.THETA,
                        save.RAYDIR.as_slice_mut(),
                    );

                    spicelib::DSKXV(
                        false,
                        &save.TARGET,
                        save.NSURF,
                        save.SRFIDS.as_slice(),
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

                    fstr::assign(&mut save.LABEL, b"(hit) Ray hit PLANE #, PT #");
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);

                    testutil::CHCKSL(&save.LABEL, save.FOUND, true, OK, ctx)?;

                    //
                    // Check the results for the next point.
                    //
                    save.K = (save.K + 1);
                }
                //
                // End of terminator point loop.
                //
            }
            //
            // End of loop for current half-plane.
            //
        }
        //
        // End of aberration correction loop.
        //
    }
    //
    // End of time loop.
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
    // Input handling tests:  make sure target, observer, and source
    // can be identified using integer "names."
    //

    //***********************************************************************
    //
    //     Normal case: state change detection
    //
    //***********************************************************************

    //
    //     Certain subsystem state changes must be detected and responded to
    //     by SINCPT. The subsystems (or structures) having states that must
    //     be monitored are:
    //
    //        - Target name-ID mapping
    //
    //        - Observer name-ID mapping
    //
    //        - Source name-ID mapping
    //
    //        - Surface name-ID mapping
    //
    //        - Target body-fixed frame definition
    //
    //        - ZZDSKBSR state
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Target name change: JUPITER maps to ID code 499.", ctx)?;

    //
    // First, get expected terminator point.
    //
    save.ET = ((8 as f64) * spicelib::JYEAR());

    fstr::assign(&mut save.TARGET, b"MARS");
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.ILUSRC, b"SATURN");
    fstr::assign(&mut save.ABCORR, b"NONE");
    save.NCUTS = 1;
    fstr::assign(&mut save.METHOD, b"PENUMBRAL/DSK/UNPRIORITIZED/GUIDED");
    fstr::assign(&mut save.CORLOC, b"CENTER");

    save.SCHSTP = 4.0;
    save.SOLTOL = 0.0000000000001;

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
        b"JUPITER",
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

    testutil::TCASE(b"Observer name change: SUN maps to ID code 399.", ctx)?;

    //
    // First, get expected terminator point.
    //
    save.ET = ((8 as f64) * spicelib::JYEAR());

    fstr::assign(&mut save.TARGET, b"MARS");
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.ILUSRC, b"SATURN");
    fstr::assign(&mut save.ABCORR, b"NONE");
    save.NCUTS = 1;
    fstr::assign(&mut save.METHOD, b"PENUMBRAL/DSK/UNPRIORITIZED/GUIDED");
    fstr::assign(&mut save.CORLOC, b"CENTER");

    spicelib::VPACK(0.0, 0.0, 1.0, save.REFVEC.as_slice_mut());

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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

    spicelib::BODDEF(b"SUN", 399, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        b"SUN",
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

    testutil::TCASE(b"Source name change: PLUTO maps to ID code 699.", ctx)?;

    //
    // First, get expected terminator point.
    //
    save.ET = ((8 as f64) * spicelib::JYEAR());

    fstr::assign(&mut save.TARGET, b"MARS");
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.ILUSRC, b"SATURN");
    fstr::assign(&mut save.ABCORR, b"NONE");
    save.NCUTS = 1;
    save.MAXN = MAXPNT;
    fstr::assign(&mut save.METHOD, b"PENUMBRAL/DSK/UNPRIORITIZED/GUIDED");
    fstr::assign(&mut save.CORLOC, b"CENTER");

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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

    spicelib::BODDEF(b"PLUTO", 699, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::TERMPT(
        &save.METHOD,
        b"pluto",
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
    spicelib::BODDEF(b"PLUTO", 999, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

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
    fstr::assign(&mut save.ILUSRC, b"SUN");
    fstr::assign(&mut save.ABCORR, b"NONE");

    fstr::assign(
        &mut save.METHOD,
        b"DSK/penumbral/UNPRIORITIZED/GUIDED/surfaces = 1",
    );

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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
        b"guided/dsk/unprioritized/penumbral/surfaces = AAAbbb",
    );

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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
    fstr::assign(&mut save.ILUSRC, b"SUN");
    fstr::assign(&mut save.ABCORR, b"NONE");

    fstr::assign(
        &mut save.METHOD,
        b"penumbral/guided/dsk/unprioritized/surfaces = low-res",
    );

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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

    fstr::assign(&mut save.METHOD, b"penumbral/guided/dsk/unprioritized");

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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

    save.ET = ((8 as f64) * spicelib::JYEAR());

    fstr::assign(&mut save.TARGET, b"MARS");
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.ILUSRC, b"SATURN");
    fstr::assign(&mut save.ABCORR, b"NONE");
    save.NCUTS = 1;
    save.MAXN = MAXPNT;
    fstr::assign(&mut save.CORLOC, b"CENTER");

    spicelib::VPACK(0.0, 0.0, 1.0, save.REFVEC.as_slice_mut());

    save.SCHSTP = 4.0;
    save.SOLTOL = 0.0000000000001;

    //
    // Restore DSK, unload low-res DSK, and repeat computation.
    //
    spicelib::FURNSH(DSK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(DSK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.METHOD, b"penumbral/guided/dsk/unprioritized");

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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
        b"penumbral/guided/dsk/unprioritized/ SURFACES = \"HIGH-RES\" ",
    );

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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
    fstr::assign(&mut save.ILUSRC, b"SUN");
    fstr::assign(&mut save.ABCORR, b"NONE");
    save.NCUTS = 1;
    save.MAXN = MAXPNT;
    fstr::assign(&mut save.CORLOC, b"CENTER");

    spicelib::VPACK(0.0, 0.0, 1.0, save.REFVEC.as_slice_mut());

    save.SCHSTP = 4.0;
    save.SOLTOL = 0.0000000000001;

    fstr::assign(&mut save.METHOD, b"guided/dsk/unprioritized");

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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
    testutil::CHCKXC(true, b"SPICE(INVALIDSHADOW)", OK, ctx)?;

    fstr::assign(&mut save.METHOD, b"tangent/dsk/unprioritized");

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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
    testutil::CHCKXC(true, b"SPICE(INVALIDSHADOW)", OK, ctx)?;

    fstr::assign(&mut save.METHOD, b"tangent/unprioritized/umbral");

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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

    fstr::assign(&mut save.METHOD, b"tangent/dsk/umbral");

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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

    fstr::assign(&mut save.METHOD, b"unprioritized/dsk/umbral");

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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
    testutil::CHCKXC(true, b"SPICE(INVALIDTERMTYPE)", OK, ctx)?;

    fstr::assign(&mut save.METHOD, b"dsk tangent/unprioritized/umbral");

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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
    testutil::CHCKXC(true, b"SPICE(INVALIDTERMTYPE)", OK, ctx)?;

    fstr::assign(&mut save.METHOD, b"ellipsoid/ tangent/unprioritized/umbral");

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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

    fstr::assign(&mut save.METHOD, b"ellipsoid/ guided / tangent /umbral");

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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

    //
    // Restore a valid method. We'll use the tangent limb
    // type because it requires the search step and
    // tolerance.
    //
    fstr::assign(&mut save.METHOD, b"UMBRAL/TANGENT/DSK/UNPRIORITIZED");

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid target name.", ctx)?;

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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
    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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
    testutil::TCASE(b"Transmission aberration correction", ctx)?;
    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        b"XCN",
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
    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid search step.", ctx)?;

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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
    testutil::CHCKXC(true, b"SPICE(INVALIDCONSTSTEP)", OK, ctx)?;

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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
    testutil::CHCKXC(true, b"SPICE(INVALIDCONSTSTEP)", OK, ctx)?;

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        save.REFVEC.as_slice(),
        save.ROLSTP,
        save.NCUTS,
        0.000000000000000001,
        save.SOLTOL,
        save.MAXN,
        save.NPTS.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.EPOCHS.as_slice_mut(),
        save.TANGTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDCONSTSTEP)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid roll step.", ctx)?;

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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
    // Restore target and illumination radii so we can get to the error
    // condition we're looking for.
    //
    spicelib::PDPOOL(b"BODY499_RADII", 3, save.BADRAD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(b"BODY10_RADII", 3, save.BADRAD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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

    fstr::assign(&mut save.METHOD, b"UMBRAL/DSK / UNPRIORITIZED / GUIDED");

    spicelib::DVPOOL(b"BODY499_RADII", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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
    testutil::TCASE(b"No radius data for illumination source", ctx)?;

    //
    // We need a method that uses an ellipsoid for this one.
    //
    fstr::assign(&mut save.METHOD, b"UMBRAL/DSK / UNPRIORITIZED / GUIDED");

    spicelib::DVPOOL(b"BODY10_RADII", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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
    // occurring in the usual case, in which there is one terminator
    // point per cutting half-plane. The torus case can present
    // problems.
    //
    // Prepare TERMPT call.
    //
    fstr::assign(&mut save.OBSRVR, b"SUN");
    fstr::assign(&mut save.TARGET, b"ALPHA");
    fstr::assign(&mut save.FIXREF, b"ALPHAFIXED");
    fstr::assign(&mut save.CORLOC, b"CENTER");
    fstr::assign(&mut save.METHOD, b"UMBRAL/DSK/TANGENT/UNPRIORITIZED");
    save.ET = 0.0;
    save.NCUTS = 2;
    save.MAXN = 2;

    save.SCHSTP = 0.0001;
    save.SOLTOL = 0.0000000000001;

    spicelib::FURNSH(DSK4, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(PCK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKLEF(SPK3, &mut save.HANDLE[3], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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
        b"GUIDED terminator definition w/ ELLIPSOID TERMINATOR correction locus.",
        ctx,
    )?;

    fstr::assign(&mut save.TARGET, b"MARS");
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.CORLOC, b"ELLIPSOID TERMINATOR");

    fstr::assign(&mut save.METHOD, b"UMBRAL/GUIDED/DSK/UNPRIORITIZED");

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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
    testutil::CHCKXC(true, b"SPICE(BADTERMLOCUSMIX)", OK, ctx)?;

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

    fstr::assign(&mut save.METHOD, b"UMBRAL/guided/dsk/unprioritized");

    spicelib::TERMPT(
        &save.METHOD,
        &save.ILUSRC,
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
