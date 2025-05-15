//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXSRF: i32 = 100;
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
const DSK0: &[u8] = b"sincpt_dsk0.bds";
const DSK1: &[u8] = b"sincpt_dsk1.bds";
const DSK2: &[u8] = b"sincpt_dsk2.bds";
const DSK3: &[u8] = b"sincpt_dsk3.bds";
const PCK: &[u8] = b"test_0008.tpc";
const SPK1: &[u8] = b"sincpt_spk.bsp";
const SPK2: &[u8] = b"orbiter.bsp";
const TIGHT: f64 = 0.000000000001;
const MTIGHT: f64 = 0.0000000001;
const MEDIUM: f64 = 0.00000001;
const LOOSE: f64 = 0.000005;
const SLOPPY: f64 = 0.001;
const UBEL: i32 = 9;
const LNSIZE: i32 = 160;
const NAMLEN: i32 = 32;
const NREF: i32 = 4;
const NABC: i32 = 9;
const ABCLEN: i32 = 10;
const TIMLEN: i32 = 50;
const NOBS: i32 = 2;
const NTARG: i32 = 2;
const NGEOM: i32 = 4;
const SCID: i32 = -499;
const NMAP: i32 = 4;
const NMETH: i32 = 4;

struct SaveVars {
    ABCS: ActualCharArray,
    ABCORR: Vec<u8>,
    DREF: Vec<u8>,
    FIXREF: Vec<u8>,
    GEOM: Vec<u8>,
    GEOMS: ActualCharArray,
    METHOD: Vec<u8>,
    METHDS: ActualCharArray,
    OBSRVR: Vec<u8>,
    OBSNMS: ActualCharArray,
    REFS: ActualCharArray2D,
    SRFNMS: ActualCharArray,
    TARGET: Vec<u8>,
    TRGFRM: Vec<u8>,
    TRGNMS: ActualCharArray,
    TITLE: Vec<u8>,
    UTC: Vec<u8>,
    AXISTA: StackArray<f64, 6>,
    AXLT: f64,
    DELTA: f64,
    DEPOCH: f64,
    DIST: f64,
    DJ2: StackArray<f64, 3>,
    DJ2M: StackArray2D<f64, 9>,
    DLT: f64,
    DVEC: StackArray<f64, 3>,
    DVECFX: StackArray<f64, 3>,
    DVECJ2: StackArray<f64, 3>,
    ELTS: StackArray<f64, 8>,
    ET: f64,
    ETOL: f64,
    FRAC: f64,
    J2OBS: StackArray<f64, 3>,
    LCENTR: StackArray<f64, 3>,
    LIMB: StackArray<f64, 9>,
    LMBOFF: StackArray<f64, 3>,
    NEGVEC: StackArray<f64, 3>,
    OBSPOS: StackArray<f64, 3>,
    OBSVEC: StackArray<f64, 3>,
    RADII: StackArray<f64, 3>,
    RAYDIR: StackArray<f64, 3>,
    RAYSTA: StackArray<f64, 6>,
    REFPOS: StackArray<f64, 3>,
    RLT: f64,
    SEP: f64,
    SMAJOR: StackArray<f64, 3>,
    SMINOR: StackArray<f64, 3>,
    SPOINT: StackArray<f64, 3>,
    SPNTLT: f64,
    SRFVEC: StackArray<f64, 3>,
    SRFVJ2: StackArray<f64, 3>,
    SSBOBS: StackArray<f64, 6>,
    SSBTRG: StackArray<f64, 6>,
    STATE0: StackArray<f64, 6>,
    TE: f64,
    TGT: StackArray<f64, 3>,
    TIPFX: StackArray<f64, 3>,
    TIPM: StackArray2D<f64, 9>,
    TLT: f64,
    TMPVEC: StackArray<f64, 3>,
    TOL: f64,
    TRGEPC: f64,
    TRGJ2M: StackArray2D<f64, 9>,
    XEPOCH: f64,
    XFORM: StackArray2D<f64, 9>,
    XLT: f64,
    XOBSPS: StackArray<f64, 3>,
    XPT: StackArray<f64, 3>,
    XRAY: StackArray<f64, 3>,
    XSPNT: StackArray<f64, 3>,
    XSRFVC: StackArray<f64, 3>,
    XTE: f64,
    BODYID: i32,
    CLS: i32,
    CLSSID: i32,
    DREFID: i32,
    FRCODE: i32,
    HANDLE: StackArray<i32, 2>,
    N: i32,
    NLAT: i32,
    NLON: i32,
    OBSCDE: i32,
    REFCTR: i32,
    SRFBOD: StackArray<i32, 4>,
    SRFIDS: StackArray<i32, 4>,
    SURFID: i32,
    SRFLST: StackArray<i32, 100>,
    TRGCDE: i32,
    FND: bool,
    FOUND: bool,
    USECN: bool,
    USELT: bool,
    USESTL: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ABCS = ActualCharArray::new(ABCLEN, 1..=NABC);
        let mut ABCORR = vec![b' '; ABCLEN as usize];
        let mut DREF = vec![b' '; NAMLEN as usize];
        let mut FIXREF = vec![b' '; NAMLEN as usize];
        let mut GEOM = vec![b' '; LNSIZE as usize];
        let mut GEOMS = ActualCharArray::new(LNSIZE, 1..=NGEOM);
        let mut METHOD = vec![b' '; MTHLEN as usize];
        let mut METHDS = ActualCharArray::new(MTHLEN, 1..=NMETH);
        let mut OBSRVR = vec![b' '; NAMLEN as usize];
        let mut OBSNMS = ActualCharArray::new(NAMLEN, 1..=NOBS);
        let mut REFS = ActualCharArray2D::new(NAMLEN, 1..=NREF, 1..=NTARG);
        let mut SRFNMS = ActualCharArray::new(NAMLEN, 1..=NMAP);
        let mut TARGET = vec![b' '; NAMLEN as usize];
        let mut TRGFRM = vec![b' '; NAMLEN as usize];
        let mut TRGNMS = ActualCharArray::new(NAMLEN, 1..=NTARG);
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut UTC = vec![b' '; TIMLEN as usize];
        let mut AXISTA = StackArray::<f64, 6>::new(1..=6);
        let mut AXLT: f64 = 0.0;
        let mut DELTA: f64 = 0.0;
        let mut DEPOCH: f64 = 0.0;
        let mut DIST: f64 = 0.0;
        let mut DJ2 = StackArray::<f64, 3>::new(1..=3);
        let mut DJ2M = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut DLT: f64 = 0.0;
        let mut DVEC = StackArray::<f64, 3>::new(1..=3);
        let mut DVECFX = StackArray::<f64, 3>::new(1..=3);
        let mut DVECJ2 = StackArray::<f64, 3>::new(1..=3);
        let mut ELTS = StackArray::<f64, 8>::new(1..=8);
        let mut ET: f64 = 0.0;
        let mut ETOL: f64 = 0.0;
        let mut FRAC: f64 = 0.0;
        let mut J2OBS = StackArray::<f64, 3>::new(1..=3);
        let mut LCENTR = StackArray::<f64, 3>::new(1..=3);
        let mut LIMB = StackArray::<f64, 9>::new(1..=UBEL);
        let mut LMBOFF = StackArray::<f64, 3>::new(1..=3);
        let mut NEGVEC = StackArray::<f64, 3>::new(1..=3);
        let mut OBSPOS = StackArray::<f64, 3>::new(1..=3);
        let mut OBSVEC = StackArray::<f64, 3>::new(1..=3);
        let mut RADII = StackArray::<f64, 3>::new(1..=3);
        let mut RAYDIR = StackArray::<f64, 3>::new(1..=3);
        let mut RAYSTA = StackArray::<f64, 6>::new(1..=6);
        let mut REFPOS = StackArray::<f64, 3>::new(1..=3);
        let mut RLT: f64 = 0.0;
        let mut SEP: f64 = 0.0;
        let mut SMAJOR = StackArray::<f64, 3>::new(1..=3);
        let mut SMINOR = StackArray::<f64, 3>::new(1..=3);
        let mut SPOINT = StackArray::<f64, 3>::new(1..=3);
        let mut SPNTLT: f64 = 0.0;
        let mut SRFVEC = StackArray::<f64, 3>::new(1..=3);
        let mut SRFVJ2 = StackArray::<f64, 3>::new(1..=3);
        let mut SSBOBS = StackArray::<f64, 6>::new(1..=6);
        let mut SSBTRG = StackArray::<f64, 6>::new(1..=6);
        let mut STATE0 = StackArray::<f64, 6>::new(1..=6);
        let mut TE: f64 = 0.0;
        let mut TGT = StackArray::<f64, 3>::new(1..=3);
        let mut TIPFX = StackArray::<f64, 3>::new(1..=3);
        let mut TIPM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut TLT: f64 = 0.0;
        let mut TMPVEC = StackArray::<f64, 3>::new(1..=3);
        let mut TOL: f64 = 0.0;
        let mut TRGEPC: f64 = 0.0;
        let mut TRGJ2M = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut XEPOCH: f64 = 0.0;
        let mut XFORM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut XLT: f64 = 0.0;
        let mut XOBSPS = StackArray::<f64, 3>::new(1..=3);
        let mut XPT = StackArray::<f64, 3>::new(1..=3);
        let mut XRAY = StackArray::<f64, 3>::new(1..=3);
        let mut XSPNT = StackArray::<f64, 3>::new(1..=3);
        let mut XSRFVC = StackArray::<f64, 3>::new(1..=3);
        let mut XTE: f64 = 0.0;
        let mut BODYID: i32 = 0;
        let mut CLS: i32 = 0;
        let mut CLSSID: i32 = 0;
        let mut DREFID: i32 = 0;
        let mut FRCODE: i32 = 0;
        let mut HANDLE = StackArray::<i32, 2>::new(1..=2);
        let mut N: i32 = 0;
        let mut NLAT: i32 = 0;
        let mut NLON: i32 = 0;
        let mut OBSCDE: i32 = 0;
        let mut REFCTR: i32 = 0;
        let mut SRFBOD = StackArray::<i32, 4>::new(1..=NMAP);
        let mut SRFIDS = StackArray::<i32, 4>::new(1..=NMAP);
        let mut SURFID: i32 = 0;
        let mut SRFLST = StackArray::<i32, 100>::new(1..=MAXSRF);
        let mut TRGCDE: i32 = 0;
        let mut FND: bool = false;
        let mut FOUND: bool = false;
        let mut USECN: bool = false;
        let mut USELT: bool = false;
        let mut USESTL: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"None"),
                Val::C(b"Lt"),
                Val::C(b"Lt+s"),
                Val::C(b"Cn"),
                Val::C(b"Cn+s"),
                Val::C(b"Xlt"),
                Val::C(b"Xlt+s"),
                Val::C(b"Xcn"),
                Val::C(b"Xcn+s"),
            ]
            .into_iter();
            ABCS.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"J2000"),
                Val::C(b"J2000"),
                Val::C(b"ECLIPJ2000"),
                Val::C(b"ECLIPJ2000"),
                Val::C(b"IAU_MARS"),
                Val::C(b"IAU_PHOBOS"),
                Val::C(b"IAU_EARTH"),
                Val::C(b"IAU_EARTH"),
            ]
            .into_iter();
            REFS.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"POINT_AT_CENTER"),
                Val::C(b"MISS_BACKWARD"),
                Val::C(b"LIMB_INSIDE_NEAR"),
                Val::C(b"MISS_LIMB_NEAR"),
            ]
            .into_iter();
            GEOMS
                .iter_mut()
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
                Val::C(b"ELLIPSOID"),
                Val::C(b"dsk/unprioritized/surfaces=\"high-res\""),
                Val::C(b"UNPRIORITIZED/ dsk /SURFACES =\"LOW-RES\""),
                Val::C(b"UNPRIORITIZED/ dsk /SURFACES =\"LOW-RES\""),
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
            DREF,
            FIXREF,
            GEOM,
            GEOMS,
            METHOD,
            METHDS,
            OBSRVR,
            OBSNMS,
            REFS,
            SRFNMS,
            TARGET,
            TRGFRM,
            TRGNMS,
            TITLE,
            UTC,
            AXISTA,
            AXLT,
            DELTA,
            DEPOCH,
            DIST,
            DJ2,
            DJ2M,
            DLT,
            DVEC,
            DVECFX,
            DVECJ2,
            ELTS,
            ET,
            ETOL,
            FRAC,
            J2OBS,
            LCENTR,
            LIMB,
            LMBOFF,
            NEGVEC,
            OBSPOS,
            OBSVEC,
            RADII,
            RAYDIR,
            RAYSTA,
            REFPOS,
            RLT,
            SEP,
            SMAJOR,
            SMINOR,
            SPOINT,
            SPNTLT,
            SRFVEC,
            SRFVJ2,
            SSBOBS,
            SSBTRG,
            STATE0,
            TE,
            TGT,
            TIPFX,
            TIPM,
            TLT,
            TMPVEC,
            TOL,
            TRGEPC,
            TRGJ2M,
            XEPOCH,
            XFORM,
            XLT,
            XOBSPS,
            XPT,
            XRAY,
            XSPNT,
            XSRFVC,
            XTE,
            BODYID,
            CLS,
            CLSSID,
            DREFID,
            FRCODE,
            HANDLE,
            N,
            NLAT,
            NLON,
            OBSCDE,
            REFCTR,
            SRFBOD,
            SRFIDS,
            SURFID,
            SRFLST,
            TRGCDE,
            FND,
            FOUND,
            USECN,
            USELT,
            USESTL,
        }
    }
}

//$Procedure      F_SINCPT ( SINCPT family tests )
pub fn F_SINCPT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    // DOUBLE PRECISION      VTIGHT
    // PARAMETER           ( VTIGHT  = 1.D-14 )

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
    // REFS is a two-dimensional array. There's a set of
    // ray reference  frames for each target. Currently
    // there are only two targets: Mars and Phobos.
    //

    //
    // Note that the last two method strings are identical. This
    // is done to test the logic that uses saved values obtained
    // by parsing method string.
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_SINCPT", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Setup:  create SPK, PCK file.", ctx)?;

    testutil::TSTSPK(SPK1, true, &mut save.HANDLE[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create the PCK file, load it, and delete it.
    //
    testutil::T_PCK08(PCK, true, false, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create LSK, load it, and delete it.
    //
    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set time.
    //
    fstr::assign(&mut save.UTC, b"2004 FEB 17");
    spicelib::STR2ET(&save.UTC, &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a Mars orbiter SPK file.
    //
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
    save.NLON = 1200;
    save.NLAT = 600;

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
    // Surface 2 for Mars is very low-res.
    //
    save.BODYID = save.TRGCDE;
    save.SURFID = 2;
    save.NLON = 40;
    save.NLAT = 20;

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
    // Surface 1 for Phobos is low-res.
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
            // Get target body-fixed frame name.
            //
            spicelib::CNMFRM(
                &save.TARGET,
                &mut save.FRCODE,
                &mut save.TRGFRM,
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

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
            // Loop over every viewing geometry case.
            //
            for GEOMIX in 1..=NGEOM {
                fstr::assign(&mut save.GEOM, save.GEOMS.get(GEOMIX));

                //
                // Loop over every aberration correction choice.
                //
                for ABCIDX in 1..=NABC {
                    fstr::assign(&mut save.ABCORR, save.ABCS.get(ABCIDX));

                    //
                    // Set up some logical variables describing the
                    // attributes of the selected correction.
                    //
                    save.USELT = fstr::ne(&save.ABCORR, b"None");

                    save.USECN = (fstr::eq(fstr::substr(&save.ABCORR, 1..=2), b"Cn")
                        || fstr::eq(fstr::substr(&save.ABCORR, 1..=3), b"Xcn"));

                    save.USESTL = (intrinsics::INDEX(&save.ABCORR, b"+s") != 0);

                    //
                    // Loop over every direction vector frame choice.
                    //
                    for REFIDX in 1..=NREF {
                        fstr::assign(&mut save.DREF, save.REFS.get([REFIDX, TRGIDX]));
                        //
                        // Set light time RLT from observer to center of
                        // frame for the direction vector.
                        //
                        spicelib::NAMFRM(&save.DREF, &mut save.DREFID, ctx)?;
                        spicelib::FRINFO(
                            save.DREFID,
                            &mut save.REFCTR,
                            &mut save.CLS,
                            &mut save.CLSSID,
                            &mut save.FND,
                            ctx,
                        )?;

                        spicelib::SPKEZP(
                            save.REFCTR,
                            save.ET,
                            b"J2000",
                            &save.ABCORR,
                            save.OBSCDE,
                            save.REFPOS.as_slice_mut(),
                            &mut save.RLT,
                            ctx,
                        )?;

                        //
                        // We'll need the epoch DEPOCH associated
                        // with the center of DREF.  RLT is the
                        // light time from DREF's center to the observer.
                        //
                        spicelib::ZZCOREPC(&save.ABCORR, save.ET, save.RLT, &mut save.DEPOCH, ctx)?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        //
                        // Look up the transformation from frame DREF to
                        // J2000. We don't need this right away, but we'll
                        // have occasion to use it later.
                        //
                        spicelib::PXFORM(
                            &save.DREF,
                            b"J2000",
                            save.DEPOCH,
                            save.DJ2M.as_slice_mut(),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        //
                        // Loop over all method choices.
                        //
                        for MIX in 1..=NMETH {
                            fstr::assign(&mut save.METHOD, save.METHDS.get(MIX));

                            //
                            // --- Case: ------------------------------------------------------
                            //

                            fstr::assign(&mut save.TITLE, b"Observer = #; Target = #; Geometry = #; ABCORR = #; DREF = #; METHOD = #.");
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
                                &save.GEOM,
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
                                &save.DREF,
                                &mut save.TITLE,
                            );
                            spicelib::REPMC(
                                &save.TITLE.to_vec(),
                                b"#",
                                &save.METHOD,
                                &mut save.TITLE,
                            );

                            testutil::TCASE(&save.TITLE, ctx)?;

                            if fstr::eq(&save.GEOM, b"POINT_AT_CENTER") {
                                //
                                // Look up direction vector using current frame
                                // and aberration correction.  The direction
                                // vector is going to point to the target's
                                // center, so we should hit the target.
                                //
                                spicelib::SPKPOS(
                                    &save.TARGET,
                                    save.ET,
                                    &save.DREF,
                                    &save.ABCORR,
                                    &save.OBSRVR,
                                    save.DVEC.as_slice_mut(),
                                    &mut save.DLT,
                                    ctx,
                                )?;
                                testutil::CHCKXC(false, b" ", OK, ctx)?;
                            } else if fstr::eq(&save.GEOM, b"MISS_BACKWARD") {
                                //
                                // Set the pointing direction to the inverse of
                                // that obtained in the 'POINT_AT_CENTER' case.
                                //
                                spicelib::SPKPOS(
                                    &save.TARGET,
                                    save.ET,
                                    &save.DREF,
                                    &save.ABCORR,
                                    &save.OBSRVR,
                                    save.NEGVEC.as_slice_mut(),
                                    &mut save.DLT,
                                    ctx,
                                )?;
                                testutil::CHCKXC(false, b" ", OK, ctx)?;

                                spicelib::VMINUS(save.NEGVEC.as_slice(), save.DVEC.as_slice_mut());
                            } else if (fstr::eq(&save.GEOM, b"LIMB_INSIDE_NEAR")
                                || fstr::eq(&save.GEOM, b"MISS_LIMB_NEAR"))
                            {
                                //
                                // Find the limb of the target based on the
                                // aberration-corrected target center position.
                                // Select ray to hit limb plane along major
                                // axis, slightly inside or slightly outside
                                // the ellipse, depending on the geometry case.
                                //
                                // Note we're looking up the target state in
                                // the target's body-fixed frame, not in the
                                // DREF frame.
                                //
                                spicelib::SPKPOS(
                                    &save.TARGET,
                                    save.ET,
                                    &save.TRGFRM,
                                    &save.ABCORR,
                                    &save.OBSRVR,
                                    save.NEGVEC.as_slice_mut(),
                                    &mut save.TLT,
                                    ctx,
                                )?;
                                testutil::CHCKXC(false, b" ", OK, ctx)?;

                                spicelib::VMINUS(
                                    save.NEGVEC.as_slice(),
                                    save.OBSVEC.as_slice_mut(),
                                );

                                //
                                // Get the limb's center and semi-axis vectors.
                                //
                                spicelib::EDLIMB(
                                    save.RADII[1],
                                    save.RADII[2],
                                    save.RADII[3],
                                    save.OBSVEC.as_slice(),
                                    save.LIMB.as_slice_mut(),
                                    ctx,
                                )?;

                                spicelib::EL2CGV(
                                    save.LIMB.as_slice(),
                                    save.LCENTR.as_slice_mut(),
                                    save.SMAJOR.as_slice_mut(),
                                    save.SMINOR.as_slice_mut(),
                                );

                                //
                                // Improve the limb if we're using aberration
                                // corrections.
                                //
                                // We'll treat one endpoint of the limb's major
                                // axis as an ephemeris object, and we'll
                                // compute the aberration-corrected position of
                                // this object.
                                //
                                // To get an accurate limb, we'll find the
                                // light time from observer to tip of
                                // semi-major axis and get an improved light
                                // time estimate.

                                if save.USELT {
                                    spicelib::VADD(
                                        save.LCENTR.as_slice(),
                                        save.SMAJOR.as_slice(),
                                        save.TIPFX.as_slice_mut(),
                                    );
                                    //
                                    // Find the aberration-corrected position of
                                    // the tip of the major axis. Compute the
                                    // offset from the original position.
                                    //
                                    spicelib::SPKCPT(
                                        save.TIPFX.as_slice(),
                                        &save.TARGET,
                                        &save.TRGFRM,
                                        save.ET,
                                        &save.TRGFRM,
                                        b"TARGET",
                                        &save.ABCORR,
                                        &save.OBSRVR,
                                        save.AXISTA.as_slice_mut(),
                                        &mut save.AXLT,
                                        ctx,
                                    )?;
                                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                                    spicelib::VSUB(
                                        save.AXISTA.as_slice(),
                                        save.TIPFX.as_slice(),
                                        save.LMBOFF.as_slice_mut(),
                                    );

                                    //
                                    // Shift the limb center by the offset we
                                    // just computed.
                                    //
                                    spicelib::VADD(
                                        save.LCENTR.as_slice(),
                                        save.LMBOFF.as_slice(),
                                        save.TMPVEC.as_slice_mut(),
                                    );
                                    spicelib::VEQU(
                                        save.TMPVEC.as_slice(),
                                        save.LCENTR.as_slice_mut(),
                                    );

                                    //
                                    // We've compute our improved limb estimate.
                                    //
                                }

                                //
                                // Pick our target point near the limb. The
                                // point is 1+/- DELTA of the semi-major axis
                                // length out from the center, along one of the
                                // semi- major axes.
                                //
                                // A challenge that occurs here, since our
                                // target is a DSK-based shape, is that the
                                // actual limb is a polygon inscribed in the
                                // ellipsoid limb. Depending on the orientation
                                // of the polygon, the semi-major axis of the
                                // ellipsoid limb may extend outside of the DSK
                                // shape, and our target point may define a ray
                                // that will miss the ellipsoid even for cases
                                // where an intersection should occur.
                                //
                                // We'll mitigate this problem for the
                                // geometric case by modifying the target
                                // point. We'll try to find a surface point on
                                // the DSK shape "below" the target point, and
                                // if we find it, we'll use it instead of the
                                // original target.
                                //
                                if save.USECN {
                                    save.DELTA = 0.00001;
                                } else if save.USELT {
                                    save.DELTA = 0.001;
                                } else {
                                    //
                                    // Value used for ellipsoidal targets is
                                    //
                                    //    DELTA = 1.D-9
                                    //
                                    // We can still use this value if we're
                                    // using the high-resolution DSK for Mars.
                                    // Otherwise, we need a point deeper inside
                                    // the ellipsoid, which corresponds to a
                                    // larger value of DELTA.
                                    //
                                    if spicelib::MATCHI(&save.METHOD, b"*HIGH*", b"*", b"?", ctx) {
                                        if fstr::eq(&save.TARGET, b"MARS") {
                                            save.DELTA = 0.000000001;
                                        } else {
                                            save.DELTA = 0.001;
                                        }
                                    } else {
                                        save.DELTA = 0.01;
                                    }
                                }

                                if fstr::eq(fstr::substr(&save.GEOM, 1..=4), b"MISS") {
                                    save.FRAC = (1.0 + save.DELTA);
                                } else {
                                    save.FRAC = (1.0 - save.DELTA);
                                }

                                spicelib::VLCOM(
                                    1.0,
                                    save.LCENTR.as_slice(),
                                    save.FRAC,
                                    save.SMAJOR.as_slice(),
                                    save.TGT.as_slice_mut(),
                                );

                                if !save.USELT {
                                    //
                                    // Find the surface point "under" TGT. This
                                    // can be done simply only for the geometric
                                    // case, because the limb has not been
                                    // translated to account for aberration
                                    // corrections.
                                    //
                                    spicelib::VMINUS(
                                        save.TGT.as_slice(),
                                        save.RAYDIR.as_slice_mut(),
                                    );

                                    spicelib::DSKXV(
                                        false,
                                        &save.TARGET,
                                        0,
                                        save.SRFLST.as_slice(),
                                        save.ET,
                                        &save.TRGFRM,
                                        1,
                                        save.TGT.as_slice(),
                                        save.RAYDIR.as_slice(),
                                        save.XPT.as_slice_mut(),
                                        std::slice::from_mut(&mut save.FND),
                                        ctx,
                                    )?;
                                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                                    if save.FND {
                                        //
                                        // If an intercept was found, use it
                                        // instead of TGT. If an intercept was
                                        // not found, TGT is already inside the
                                        // tessellated ellipsoid.
                                        //
                                        spicelib::VEQU(
                                            save.XPT.as_slice(),
                                            save.TGT.as_slice_mut(),
                                        );
                                    }
                                }

                                //
                                // Our ray extends from the observer through
                                // the target point.
                                //
                                spicelib::VSUB(
                                    save.TGT.as_slice(),
                                    save.OBSVEC.as_slice(),
                                    save.DVECFX.as_slice_mut(),
                                );

                                save.SEP = spicelib::VSEP(
                                    save.NEGVEC.as_slice(),
                                    save.DVECFX.as_slice(),
                                    ctx,
                                );

                                //
                                // Convert the ray from the target body fixed
                                // frame to J2000, then from J2000 to the DREF
                                // frame. We need the target frame epoch TE to
                                // find the first transformation matrix TIPM.
                                //
                                spicelib::ZZCOREPC(
                                    &save.ABCORR,
                                    save.ET,
                                    save.TLT,
                                    &mut save.TE,
                                    ctx,
                                )?;

                                spicelib::PXFORM(
                                    b"J2000",
                                    &save.TRGFRM,
                                    save.TE,
                                    save.TIPM.as_slice_mut(),
                                    ctx,
                                )?;

                                spicelib::MTXV(
                                    save.TIPM.as_slice(),
                                    save.DVECFX.as_slice(),
                                    save.DVECJ2.as_slice_mut(),
                                );

                                //
                                // The matrix DJ2M maps from DREF to J2000, so
                                // apply the transpose of this matrix to obtain
                                // DVEC.
                                //
                                spicelib::MTXV(
                                    save.DJ2M.as_slice(),
                                    save.DVECJ2.as_slice(),
                                    save.DVEC.as_slice_mut(),
                                );
                            } else {
                                //
                                // Oops!  Name mismatch.
                                //
                                spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
                                testutil::CHCKXC(false, b" ", OK, ctx)?;
                            }

                            //
                            // Find the surface intercept point.
                            //
                            spicelib::SINCPT(
                                &save.METHOD,
                                &save.TARGET,
                                save.ET,
                                &save.TRGFRM,
                                &save.ABCORR,
                                &save.OBSRVR,
                                &save.DREF,
                                save.DVEC.as_slice(),
                                save.SPOINT.as_slice_mut(),
                                &mut save.TRGEPC,
                                save.SRFVEC.as_slice_mut(),
                                &mut save.FOUND,
                                ctx,
                            )?;

                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            //
                            // Check the results.
                            //
                            if !save.FOUND {
                                if fstr::eq(fstr::substr(&save.GEOM, 1..=4), b"MISS") {
                                    //
                                    // FOUND should be .FALSE.; the other outputs
                                    // are undefined.
                                    //
                                    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;
                                } else {
                                    //
                                    // We're supposed to have an intercept.
                                    // Force an error signal.
                                    //
                                    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
                                }
                            } else {
                                //
                                // FOUND is true.
                                //
                                // Compute the observer position relative to
                                // the target center, consistent with the
                                // aberration corrections applicable to the
                                // apparent intercept point. Also compute the
                                // distance of between the observer and the
                                // apparent intercept point.

                                spicelib::VSUB(
                                    save.SPOINT.as_slice(),
                                    save.SRFVEC.as_slice(),
                                    save.OBSPOS.as_slice_mut(),
                                );

                                //
                                // Let DIST be the length of SRFVEC.
                                //
                                save.DIST = spicelib::VNORM(save.SRFVEC.as_slice());

                                //
                                // The target epoch had better be consistent
                                // with DIST and ABCORR.
                                //
                                save.XLT = (save.DIST / spicelib::CLIGHT());

                                spicelib::ZZCOREPC(
                                    &save.ABCORR,
                                    save.ET,
                                    save.XLT,
                                    &mut save.XEPOCH,
                                    ctx,
                                )?;

                                //
                                // This is a relative error check.
                                //
                                if save.USECN {
                                    save.ETOL = TIGHT;
                                } else {
                                    save.ETOL = MEDIUM;
                                }

                                testutil::CHCKSD(
                                    b"TRGEPC",
                                    save.TRGEPC,
                                    b"~/",
                                    save.XEPOCH,
                                    save.ETOL,
                                    OK,
                                    ctx,
                                )?;

                                //
                                // Get the transformation from the target frame
                                // to J2000.

                                spicelib::PXFORM(
                                    &save.TRGFRM,
                                    b"J2000",
                                    save.TRGEPC,
                                    save.TRGJ2M.as_slice_mut(),
                                    ctx,
                                )?;
                                testutil::CHCKXC(false, b" ", OK, ctx)?;

                                //
                                // Now transform DVEC to the J2000 frame.
                                //
                                spicelib::MXV(
                                    save.DJ2M.as_slice(),
                                    save.DVEC.as_slice(),
                                    save.DJ2.as_slice_mut(),
                                );

                                //
                                // The following check applies only to the case
                                // in which the pointing direction is toward
                                // the target's center.
                                //
                                if fstr::eq(&save.GEOM, b"POINT_AT_CENTER") {
                                    //
                                    // The angular separation of SRFVEC and DVEC
                                    // should be pretty small when these vectors
                                    // are compared in compatible reference
                                    // frames. We don't expect these vectors to
                                    // be identical (even theoretically) because
                                    // they've been computed with different
                                    // target epochs.
                                    //
                                    // First step: get SRFVEC in the J2000
                                    // frame.
                                    //
                                    spicelib::MXV(
                                        save.TRGJ2M.as_slice(),
                                        save.SRFVEC.as_slice(),
                                        save.SRFVJ2.as_slice_mut(),
                                    );

                                    //
                                    // Find the angular separation and make sure
                                    // it's not too large.
                                    //
                                    save.SEP = spicelib::VSEP(
                                        save.SRFVJ2.as_slice(),
                                        save.DJ2.as_slice(),
                                        ctx,
                                    );

                                    testutil::CHCKSD(
                                        b"DJ2 vs NEG2 SEP",
                                        save.SEP,
                                        b"~",
                                        0.0,
                                        SLOPPY,
                                        OK,
                                        ctx,
                                    )?;
                                }

                                //
                                // End of sanity check test for the
                                // POINT_AT_CENTER case.
                                //
                                // Having made it this far, we're ready for
                                // some more rigorous tests. In particular,
                                // we're going treat SPOINT as an ephemeris
                                // object and find its aberration-corrected
                                // position relative to the observer in J2000
                                // coordinates. This computation will allow us
                                // to derive expected values of TRGEPC, OBSPOS,
                                // the transformation from the J2000 frame to
                                // the target body-fixed frame at TRGEPC. We
                                // will verify that the aberration-corrected
                                // location of SPOINT, lies on the ray DVEC:
                                // this is the criterion we used to define
                                // SPOINT.
                                //
                                // These tests are primarily of interest when
                                // aberration corrections are used, but they
                                // still serve as a consistency check for the
                                // geometric case.
                                //
                                // We're expecting to get good agreement
                                // between all of these items and their
                                // counterparts obtained from SINCPT,
                                // especially when we use converged Newtonian
                                // corrections.

                                if spicelib::EQSTR(&save.OBSRVR, b"EARTH") {
                                    if save.USECN {
                                        save.TOL = TIGHT;
                                    } else if save.USELT {
                                        save.TOL = MEDIUM;
                                    } else {
                                        save.TOL = TIGHT;
                                    }
                                } else {
                                    //
                                    // Use looser tolerances for the Mars
                                    // orbiter. For the orbiter, small errors in
                                    // SPOINT lead to larger relative errors in
                                    // DIST and SEP.

                                    if save.USECN {
                                        save.TOL = MTIGHT;
                                    } else if save.USELT {
                                        save.TOL = LOOSE;
                                    } else {
                                        save.TOL = MTIGHT;
                                    }

                                    if fstr::eq(&save.GEOM, b"LIMB_INSIDE_NEAR") {
                                        //
                                        // Loosen the tolerance so the
                                        // MAC/OSX/Native C platform can handle
                                        // these tests.
                                        //
                                        save.TOL = (save.TOL * 100.0);
                                    }
                                }

                                //
                                // Find the aberration-corrected location of
                                // SPOINT.
                                //
                                // Prior to N0065, we had to do this manually.
                                // Now we can call a SPICE routine to do the
                                // job.
                                //
                                spicelib::SPKCPT(
                                    save.SPOINT.as_slice(),
                                    &save.TARGET,
                                    &save.TRGFRM,
                                    save.ET,
                                    b"J2000",
                                    b"TARGET",
                                    &save.ABCORR,
                                    &save.OBSRVR,
                                    save.RAYSTA.as_slice_mut(),
                                    &mut save.SPNTLT,
                                    ctx,
                                )?;
                                testutil::CHCKXC(false, b" ", OK, ctx)?;

                                //
                                // XRAY is our expected result.
                                //
                                spicelib::VEQU(save.RAYSTA.as_slice(), save.XRAY.as_slice_mut());

                                //
                                // Compute the expected target epoch.
                                //
                                spicelib::ZZCOREPC(
                                    &save.ABCORR,
                                    save.ET,
                                    save.SPNTLT,
                                    &mut save.XTE,
                                    ctx,
                                )?;
                                testutil::CHCKXC(false, b" ", OK, ctx)?;

                                //
                                // Moment of truth: XRAY is the J2000 vector
                                // from the observer to the
                                // aberration-corrected position of our
                                // "ephemeris object" located on the target
                                // surface at location SPOINT. If SPOINT were
                                // correct in the first place, then XRAY should
                                // be lined up with our boresight direction
                                // DVEC, when DVEC is rotated to the J2000
                                // frame.
                                //
                                // Actually, we computed DVEC in the J2000
                                // frame long ago: this vector is called DJ2.
                                //
                                testutil::CHCKSD(
                                    b"TRGEPC vs XTE",
                                    save.TRGEPC,
                                    b"~/",
                                    save.XTE,
                                    save.TOL,
                                    OK,
                                    ctx,
                                )?;

                                save.SEP =
                                    spicelib::VSEP(save.DJ2.as_slice(), save.XRAY.as_slice(), ctx);

                                testutil::CHCKSD(
                                    b"XRAY vs DJ2 sep",
                                    save.SEP,
                                    b"~",
                                    0.0,
                                    save.TOL,
                                    OK,
                                    ctx,
                                )?;

                                //
                                // Check DIST against its predicted value.
                                //
                                testutil::CHCKSD(
                                    b"DIST",
                                    save.DIST,
                                    b"~/",
                                    spicelib::VNORM(save.XRAY.as_slice()),
                                    save.TOL,
                                    OK,
                                    ctx,
                                )?;

                                //
                                // Create a predicted value for SRFVEC: convert
                                // XRAY to the target frame using XFORM.
                                //
                                spicelib::PXFORM(
                                    b"J2000",
                                    &save.TRGFRM,
                                    save.XTE,
                                    save.XFORM.as_slice_mut(),
                                    ctx,
                                )?;
                                testutil::CHCKXC(false, b" ", OK, ctx)?;

                                spicelib::MXV(
                                    save.XFORM.as_slice(),
                                    save.XRAY.as_slice(),
                                    save.XSRFVC.as_slice_mut(),
                                );

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

                                //
                                // The following test only makes sense when
                                // stellar aberration is not used. (When
                                // stellar aberration IS used, SPOINT does not
                                // lie on the ray; the image of SPOINT under
                                // the stellar aberration correction lies on
                                // the ray.
                                //
                                if !save.USESTL {
                                    //
                                    // Create a predicted value for SPOINT:
                                    // convert the (optionally light-time
                                    // corrected) target-observer vector to the
                                    // target body-fixed frame at epoch XTE and
                                    // add to SRFVEC to form XSPNT.
                                    //
                                    spicelib::SPKSSB(
                                        save.OBSCDE,
                                        save.ET,
                                        b"J2000",
                                        save.SSBOBS.as_slice_mut(),
                                        ctx,
                                    )?;
                                    spicelib::SPKSSB(
                                        save.TRGCDE,
                                        save.XTE,
                                        b"J2000",
                                        save.SSBTRG.as_slice_mut(),
                                        ctx,
                                    )?;
                                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                                    spicelib::VSUB(
                                        save.SSBOBS.as_slice(),
                                        save.SSBTRG.as_slice(),
                                        save.J2OBS.as_slice_mut(),
                                    );
                                    spicelib::MXV(
                                        save.XFORM.as_slice(),
                                        save.J2OBS.as_slice(),
                                        save.XOBSPS.as_slice_mut(),
                                    );
                                    spicelib::VADD(
                                        save.XOBSPS.as_slice(),
                                        save.SRFVEC.as_slice(),
                                        save.XSPNT.as_slice_mut(),
                                    );
                                    //
                                    // Use absolute tolerances for this check:
                                    // 1cm for converged light time; 1km for
                                    // non-converged.
                                    //
                                    if save.USECN {
                                        save.TOL = 0.00001;
                                    } else {
                                        save.TOL = 1.0;
                                    }

                                    testutil::CHCKAD(
                                        b"SPOINT",
                                        save.SPOINT.as_slice(),
                                        b"~~",
                                        save.XSPNT.as_slice(),
                                        3,
                                        save.TOL,
                                        OK,
                                        ctx,
                                    )?;
                                }
                            }
                            //
                            // We're finished with the consistency checks for
                            // the intercept cases.
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
            // End of the geometry case loop.
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
    testutil::TCASE(b"Use integer observer and target names.", ctx)?;
    //
    // Set up the ray first.
    //
    spicelib::SPKPOS(
        b"499",
        save.ET,
        &save.DREF,
        &save.ABCORR,
        b"399",
        save.DVEC.as_slice_mut(),
        &mut save.DLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"(0) FOUND", save.FOUND, true, OK, ctx)?;

    //
    // Set target and target-fixed frame.
    //
    fstr::assign(&mut save.TARGET, b"MARS");
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");

    spicelib::SINCPT(
        b"Ellipsoid",
        b"MARS",
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        b"Earth",
        &save.DREF,
        save.DVEC.as_slice(),
        save.XSPNT.as_slice_mut(),
        &mut save.XEPOCH,
        save.XSRFVC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::SINCPT(
        b"Ellipsoid",
        b"499",
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        b"399",
        &save.DREF,
        save.DVEC.as_slice(),
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    testutil::CHCKAD(
        b"SPOINT",
        save.SPOINT.as_slice(),
        b"=",
        save.XSPNT.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKAD(
        b"SRFVEC",
        save.SRFVEC.as_slice(),
        b"=",
        save.XSRFVC.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"TRGEPC", save.TRGEPC, b"=", save.XEPOCH, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"DIST", save.DIST, b"=", save.DIST, 0.0, OK, ctx)?;

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
    //    - Ray direction frame definition
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
    fstr::assign(&mut save.DREF, b"J2000");
    //
    // Set up the ray first.
    //
    spicelib::SPKPOS(
        b"499",
        save.ET,
        &save.DREF,
        &save.ABCORR,
        b"399",
        save.DVEC.as_slice_mut(),
        &mut save.DLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SINCPT(
        b"Ellipsoid",
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.XSPNT.as_slice_mut(),
        &mut save.XEPOCH,
        save.XSRFVC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"(0) FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::BODDEF(b"JUPITER", 499, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SINCPT(
        b"Ellipsoid",
        b"JUPITER",
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"(1) FOUND", save.FOUND, true, OK, ctx)?;

    //
    // We expect an exact match here.
    //
    testutil::CHCKAD(
        b"SPOINT",
        save.SPOINT.as_slice(),
        b"=",
        save.XSPNT.as_slice(),
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

    spicelib::SINCPT(
        b"Ellipsoid",
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        b"SUN",
        &save.DREF,
        save.DVEC.as_slice(),
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"(0) FOUND", save.FOUND, true, OK, ctx)?;

    //
    // We expect an exact match here.
    //
    testutil::CHCKAD(
        b"SPOINT",
        save.SPOINT.as_slice(),
        b"=",
        save.XSPNT.as_slice(),
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
    fstr::assign(&mut save.DREF, b"J2000");

    spicelib::SPKPOS(
        b"499",
        save.ET,
        &save.DREF,
        &save.ABCORR,
        b"399",
        save.DVEC.as_slice_mut(),
        &mut save.DLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.METHOD, b"dsk/unprioritized/surfaces = 1");

    spicelib::SINCPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.XSPNT.as_slice_mut(),
        &mut save.XEPOCH,
        save.XSRFVC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"(0) FOUND", save.FOUND, true, OK, ctx)?;

    fstr::assign(save.SRFNMS.get_mut(1), b"AAAbbb");

    spicelib::PCPOOL(b"NAIF_SURFACE_NAME", NMAP, save.SRFNMS.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.METHOD, b"dsk/unprioritized/surfaces = AAAbbb");

    spicelib::SINCPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"(1) FOUND", save.FOUND, true, OK, ctx)?;

    //
    // We expect an exact match here.
    //
    testutil::CHCKAD(
        b"SPOINT",
        save.SPOINT.as_slice(),
        b"=",
        save.XSPNT.as_slice(),
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
    fstr::assign(&mut save.DREF, b"J2000");

    spicelib::SPKPOS(
        b"499",
        save.ET,
        &save.DREF,
        &save.ABCORR,
        b"399",
        save.DVEC.as_slice_mut(),
        &mut save.DLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.METHOD, b"dsk/unprioritized/surfaces = low-res");

    spicelib::SINCPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.XSPNT.as_slice_mut(),
        &mut save.XEPOCH,
        save.XSRFVC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"(0) FOUND", save.FOUND, true, OK, ctx)?;

    //
    // Unload the high-res DSK; set METHOD to remove
    // surface specification.
    //
    spicelib::UNLOAD(DSK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.METHOD, b"dsk/unprioritized");

    spicelib::SINCPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"(1) FOUND", save.FOUND, true, OK, ctx)?;

    //
    // We expect an exact match here.
    //
    testutil::CHCKAD(
        b"SPOINT",
        save.SPOINT.as_slice(),
        b"=",
        save.XSPNT.as_slice(),
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

    fstr::assign(&mut save.METHOD, b"dsk/unprioritized");

    spicelib::SINCPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"(1) FOUND", save.FOUND, true, OK, ctx)?;

    //
    // Make sure the result matches that obtained with the
    // high-res DSK specified.
    //
    fstr::assign(
        &mut save.METHOD,
        b"dsk/unprioritized/ SURFACES = \"HIGH-RES\" ",
    );

    spicelib::SINCPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.XSPNT.as_slice_mut(),
        &mut save.XEPOCH,
        save.XSRFVC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"(0) FOUND", save.FOUND, true, OK, ctx)?;

    //
    // We expect an exact match here.
    //
    testutil::CHCKAD(
        b"SPOINT",
        save.SPOINT.as_slice(),
        b"=",
        save.XSPNT.as_slice(),
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
    testutil::TCASE(b"Target name not translated", ctx)?;
    //
    // Find the surface intercept point.
    //
    spicelib::SINCPT(
        b"Ellipsoid",
        b"xyz",
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Observer name not translated", ctx)?;
    //
    // Find the surface intercept point.
    //
    spicelib::SINCPT(
        b"Ellipsoid",
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        b"xyz",
        &save.DREF,
        save.DVEC.as_slice(),
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Observer coincides with target", ctx)?;
    //
    // Find the surface intercept point.
    //
    spicelib::SINCPT(
        b"Ellipsoid",
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.TARGET,
        &save.DREF,
        save.DVEC.as_slice(),
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Unsupported computation method", ctx)?;
    //
    // Find the surface intercept point.
    //
    spicelib::SINCPT(
        b"xyz",
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADMETHODSYNTAX)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Body-fixed frame is not centered on target. ", ctx)?;
    //
    // Find the surface intercept point.  Use reference frame 'BAD'
    // for direction vector.
    //
    spicelib::SINCPT(
        b"Ellipsoid",
        b"Mars_orbiter",
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        b"EARTH",
        b"BAD",
        save.DVEC.as_slice(),
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Bad aberration correction. ", ctx)?;
    //
    // Find the surface intercept point. Use unrecognized aberration
    // correction.
    //
    spicelib::SINCPT(
        b"Ellipsoid",
        b"Mars_orbiter",
        save.ET,
        &save.FIXREF,
        b"ZZZ",
        b"EARTH",
        b"IAU_EARTH",
        save.DVEC.as_slice(),
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Bad aberration correction: relativistic ", ctx)?;
    //
    // Find the surface intercept point. Use relativistic aberration
    // correction.
    //
    spicelib::SINCPT(
        b"Ellipsoid",
        b"Mars",
        save.ET,
        &save.FIXREF,
        b"RL",
        b"EARTH",
        b"IAU_EARTH",
        save.DVEC.as_slice(),
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    testutil::TCASE(b"Bad aberration correction: stellar aberration only", ctx)?;
    //
    // Find the surface intercept point. Use stellar aberration
    // correction w/o light time correction.
    //
    spicelib::SINCPT(
        b"Ellipsoid",
        b"MARS",
        save.ET,
        &save.FIXREF,
        b"XS",
        b"EARTH",
        b"IAU_EARTH",
        save.DVEC.as_slice(),
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Zero direction vector", ctx)?;

    spicelib::CLEARD(3, save.DVEC.as_slice_mut());
    spicelib::SINCPT(
        b"Ellipsoid",
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(ZEROVECTOR)", OK, ctx)?;

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

    //
    // Get reference result using low-res Mars DSK.
    //
    save.ET = ((8 as f64) * spicelib::JYEAR());

    fstr::assign(&mut save.TARGET, b"MARS");
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.DREF, b"J2000");

    spicelib::SPKPOS(
        b"499",
        save.ET,
        &save.DREF,
        &save.ABCORR,
        b"399",
        save.DVEC.as_slice_mut(),
        &mut save.DLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.METHOD, b"dsk/unprioritized/surfaces = low-res");

    spicelib::SINCPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.XSPNT.as_slice_mut(),
        &mut save.XEPOCH,
        save.XSRFVC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOLOADEDDSKFILES)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Clean up.
    //
    spicelib::SPKUEF(save.HANDLE[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DELFIL(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKUEF(save.HANDLE[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DELFIL(SPK2, ctx)?;
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

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
