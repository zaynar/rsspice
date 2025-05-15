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
const DSK0: &[u8] = b"zztanutl_0.bds";
const DSK1: &[u8] = b"zztanutl_1.bds";
const DSK2: &[u8] = b"zztanutl_2.bds";
const DSK3: &[u8] = b"zztanutl_3.bds";
const DSK4: &[u8] = b"zztanutl_4.bds";
const PCK0: &[u8] = b"zztanutl.tpc";
const PCK1: &[u8] = b"nat.tpc";
const SPK1: &[u8] = b"nat.bsp";
const MED: f64 = 0.0000001;
const TIGHT: f64 = 0.0000000001;
const VTIGHT: f64 = 0.00000000001;
const ALPHA: i32 = 1000;
const NAMLEN: i32 = 32;
const NMAP: i32 = 4;
const UBEL: i32 = 9;
const UBPL: i32 = 4;
const MAXWIN: i32 = 2000;
const LBCELL: i32 = -5;
const NCROSS: i32 = 400;
const NPOLYV: i32 = 100;
const LNSIZE: i32 = 240;

struct SaveVars {
    FIXREF: Vec<u8>,
    LABEL: Vec<u8>,
    SRFNMS: ActualCharArray,
    TARGET: Vec<u8>,
    TITLE: Vec<u8>,
    A: f64,
    ANGLE: f64,
    AXIS: StackArray<f64, 3>,
    B: f64,
    C: f64,
    CDIR: f64,
    CENTER: StackArray<f64, 3>,
    CUT: StackArray<f64, 4>,
    D: f64,
    DP: f64,
    DIR: StackArray<f64, 3>,
    DIST: f64,
    DTHETA: f64,
    ET: f64,
    LIMB: StackArray<f64, 9>,
    LPOINT: StackArray<f64, 3>,
    LT: f64,
    NORMAL: StackArray<f64, 3>,
    NPT: StackArray<f64, 3>,
    OFFSET: StackArray<f64, 3>,
    ORIGIN: StackArray<f64, 3>,
    POINT: StackArray<f64, 3>,
    POINTS: ActualArray2D<f64>,
    PLNVEC: StackArray<f64, 3>,
    PNEAR: StackArray<f64, 3>,
    PROJ: StackArray<f64, 3>,
    R: f64,
    RADII: StackArray<f64, 3>,
    RAYDIR: StackArray<f64, 3>,
    REFVEC: StackArray<f64, 3>,
    RCROSS: f64,
    RESULT: ActualArray<f64>,
    RPOINT: StackArray<f64, 3>,
    SCHSTP: f64,
    SDIR: f64,
    SOLTOL: f64,
    SRCRAD: f64,
    SUNLPT: StackArray<f64, 3>,
    SUNRAD: StackArray<f64, 3>,
    TANPNT: StackArray<f64, 3>,
    THETA: f64,
    TOL: f64,
    VERTEX: StackArray<f64, 3>,
    VTEMP: StackArray<f64, 3>,
    XVEC: StackArray<f64, 3>,
    XPT1: StackArray<f64, 3>,
    XPT2: StackArray<f64, 3>,
    YVEC: StackArray<f64, 3>,
    ZVEC: StackArray<f64, 3>,
    BODYID: i32,
    CURVE: i32,
    FIXFID: i32,
    HANDLE: i32,
    N: i32,
    NCUTS: i32,
    NLAT: i32,
    NLON: i32,
    NSURF: i32,
    NXPTS: i32,
    SHAPE: i32,
    SRFBOD: StackArray<i32, 4>,
    SRFIDS: StackArray<i32, 4>,
    SRFLST: StackArray<i32, 100>,
    SURFID: i32,
    TRGCDE: i32,
    FOUND: bool,
    OCULTD: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FIXREF = vec![b' '; NAMLEN as usize];
        let mut LABEL = vec![b' '; NAMLEN as usize];
        let mut SRFNMS = ActualCharArray::new(NAMLEN, 1..=NMAP);
        let mut TARGET = vec![b' '; NAMLEN as usize];
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut A: f64 = 0.0;
        let mut ANGLE: f64 = 0.0;
        let mut AXIS = StackArray::<f64, 3>::new(1..=3);
        let mut B: f64 = 0.0;
        let mut C: f64 = 0.0;
        let mut CDIR: f64 = 0.0;
        let mut CENTER = StackArray::<f64, 3>::new(1..=3);
        let mut CUT = StackArray::<f64, 4>::new(1..=UBPL);
        let mut D: f64 = 0.0;
        let mut DP: f64 = 0.0;
        let mut DIR = StackArray::<f64, 3>::new(1..=3);
        let mut DIST: f64 = 0.0;
        let mut DTHETA: f64 = 0.0;
        let mut ET: f64 = 0.0;
        let mut LIMB = StackArray::<f64, 9>::new(1..=UBEL);
        let mut LPOINT = StackArray::<f64, 3>::new(1..=3);
        let mut LT: f64 = 0.0;
        let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
        let mut NPT = StackArray::<f64, 3>::new(1..=3);
        let mut OFFSET = StackArray::<f64, 3>::new(1..=3);
        let mut ORIGIN = StackArray::<f64, 3>::new(1..=3);
        let mut POINT = StackArray::<f64, 3>::new(1..=3);
        let mut POINTS = ActualArray2D::<f64>::new(1..=3, 1..=MAXWIN);
        let mut PLNVEC = StackArray::<f64, 3>::new(1..=3);
        let mut PNEAR = StackArray::<f64, 3>::new(1..=3);
        let mut PROJ = StackArray::<f64, 3>::new(1..=3);
        let mut R: f64 = 0.0;
        let mut RADII = StackArray::<f64, 3>::new(1..=3);
        let mut RAYDIR = StackArray::<f64, 3>::new(1..=3);
        let mut REFVEC = StackArray::<f64, 3>::new(1..=3);
        let mut RCROSS: f64 = 0.0;
        let mut RESULT = ActualArray::<f64>::new(LBCELL..=MAXWIN);
        let mut RPOINT = StackArray::<f64, 3>::new(1..=3);
        let mut SCHSTP: f64 = 0.0;
        let mut SDIR: f64 = 0.0;
        let mut SOLTOL: f64 = 0.0;
        let mut SRCRAD: f64 = 0.0;
        let mut SUNLPT = StackArray::<f64, 3>::new(1..=3);
        let mut SUNRAD = StackArray::<f64, 3>::new(1..=3);
        let mut TANPNT = StackArray::<f64, 3>::new(1..=3);
        let mut THETA: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut VERTEX = StackArray::<f64, 3>::new(1..=3);
        let mut VTEMP = StackArray::<f64, 3>::new(1..=3);
        let mut XVEC = StackArray::<f64, 3>::new(1..=3);
        let mut XPT1 = StackArray::<f64, 3>::new(1..=3);
        let mut XPT2 = StackArray::<f64, 3>::new(1..=3);
        let mut YVEC = StackArray::<f64, 3>::new(1..=3);
        let mut ZVEC = StackArray::<f64, 3>::new(1..=3);
        let mut BODYID: i32 = 0;
        let mut CURVE: i32 = 0;
        let mut FIXFID: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut N: i32 = 0;
        let mut NCUTS: i32 = 0;
        let mut NLAT: i32 = 0;
        let mut NLON: i32 = 0;
        let mut NSURF: i32 = 0;
        let mut NXPTS: i32 = 0;
        let mut SHAPE: i32 = 0;
        let mut SRFBOD = StackArray::<i32, 4>::new(1..=NMAP);
        let mut SRFIDS = StackArray::<i32, 4>::new(1..=NMAP);
        let mut SRFLST = StackArray::<i32, 100>::new(1..=MAXSRF);
        let mut SURFID: i32 = 0;
        let mut TRGCDE: i32 = 0;
        let mut FOUND: bool = false;
        let mut OCULTD: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::D(0.0), 3 as usize))
                .chain([]);

            ORIGIN
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            FIXREF,
            LABEL,
            SRFNMS,
            TARGET,
            TITLE,
            A,
            ANGLE,
            AXIS,
            B,
            C,
            CDIR,
            CENTER,
            CUT,
            D,
            DP,
            DIR,
            DIST,
            DTHETA,
            ET,
            LIMB,
            LPOINT,
            LT,
            NORMAL,
            NPT,
            OFFSET,
            ORIGIN,
            POINT,
            POINTS,
            PLNVEC,
            PNEAR,
            PROJ,
            R,
            RADII,
            RAYDIR,
            REFVEC,
            RCROSS,
            RESULT,
            RPOINT,
            SCHSTP,
            SDIR,
            SOLTOL,
            SRCRAD,
            SUNLPT,
            SUNRAD,
            TANPNT,
            THETA,
            TOL,
            VERTEX,
            VTEMP,
            XVEC,
            XPT1,
            XPT2,
            YVEC,
            ZVEC,
            BODYID,
            CURVE,
            FIXFID,
            HANDLE,
            N,
            NCUTS,
            NLAT,
            NLON,
            NSURF,
            NXPTS,
            SHAPE,
            SRFBOD,
            SRFIDS,
            SRFLST,
            SURFID,
            TRGCDE,
            FOUND,
            OCULTD,
        }
    }
}

//$Procedure F_ZZTANUTL ( ZZTANUTL/ZZTANGNT tests )
pub fn F_ZZTANUTL(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    // Saved values
    //
    // Save variables in order to avoid stack room problems.
    //

    //
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_ZZTANUTL", ctx)?;

    //**********************************************************************
    //
    //     Setup for ZZTANINI/ZZTANSTA tests
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Set-up: create PCK.", ctx)?;

    if spicelib::EXISTS(PCK0, ctx)? {
        spicelib::DELFIL(PCK0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    testutil::T_PCK08(PCK0, true, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

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

    //**********************************************************************
    //
    //     ZZTANUTL Error cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Call ZZTANUTL directly.", ctx)?;

    spicelib::ZZTANUTL(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        save.ANGLE,
        save.OCULTD,
        save.POINT.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BOGUSENTRY)", OK, ctx)?;

    //**********************************************************************
    //
    //     ZZTANINI Error cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZTANINI: Zero axis vector.", ctx)?;

    save.CURVE = LMBCRV;
    save.SRCRAD = 0.0;
    save.SHAPE = ELLSHP;
    save.TRGCDE = 499;
    save.NSURF = 0;
    save.SRFLST[1] = 0;

    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 10000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::CLEARD(3, save.AXIS.as_slice_mut());

    spicelib::ZZTANINI(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ZEROVECTOR)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZTANINI: Zero half-plane reference vector.", ctx)?;

    save.CURVE = LMBCRV;
    save.SRCRAD = 0.0;
    save.SHAPE = ELLSHP;
    save.TRGCDE = 499;
    save.NSURF = 0;
    save.SRFLST[1] = 0;

    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 10000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::CLEARD(3, save.PLNVEC.as_slice_mut());

    spicelib::ZZTANINI(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ZEROVECTOR)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZTANINI: invalid curve type.", ctx)?;

    save.CURVE = -1;
    save.SRCRAD = 0.0;
    save.SHAPE = ELLSHP;
    save.TRGCDE = 499;
    save.NSURF = 0;
    save.SRFLST[1] = 0;

    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 10000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::ZZTANINI(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADCURVETYPE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZTANINI: invalid source radius", ctx)?;

    save.CURVE = UMBRAL;
    save.SRCRAD = 0.0;
    save.SHAPE = ELLSHP;
    save.TRGCDE = 499;
    save.NSURF = 0;
    save.SRFLST[1] = 0;

    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 10000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::ZZTANINI(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADSOURCERADIUS)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZTANINI: axis and reference vector are linearly dependent.",
        ctx,
    )?;

    save.CURVE = UMBRAL;
    save.SRCRAD = 696000.0;
    save.SHAPE = ELLSHP;
    save.TRGCDE = 499;
    save.NSURF = 0;
    save.SRFLST[1] = 0;

    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 10000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );

    spicelib::VSCL(2.0, save.AXIS.as_slice(), save.PLNVEC.as_slice_mut());

    spicelib::ZZTANINI(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DEGENERATECASE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZTANINI: invalid target shape.", ctx)?;

    save.CURVE = UMBRAL;
    save.SRCRAD = 696000.0;
    save.SHAPE = -1;
    save.TRGCDE = 499;
    save.NSURF = 0;
    save.SRFLST[1] = 0;

    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 10000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::ZZTANINI(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADSHAPE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZTANINI: missing PCK data.", ctx)?;

    save.CURVE = UMBRAL;
    save.SRCRAD = 696000.0;
    save.SHAPE = ELLSHP;
    save.TRGCDE = 499;
    save.NSURF = 0;
    save.SRFLST[1] = 0;

    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 10000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::CLPOOL(ctx)?;

    spicelib::ZZTANINI(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    spicelib::FURNSH(PCK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //**********************************************************************
    //
    //     ZZTANSTA Error cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZTANSTA: no DSKs loaded.", ctx)?;

    save.CURVE = UMBRAL;
    save.SRCRAD = 696000.0;
    save.SHAPE = DSKSHP;
    save.TRGCDE = 499;
    save.NSURF = 0;
    save.SRFLST[1] = 0;

    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 10000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::ZZTANINI(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(DSK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::UNLOAD(DSK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::UNLOAD(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::UNLOAD(DSK3, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZTANSTA(0.0, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(NOLOADEDDSKFILES)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZTANSTA: no DSK data for target", ctx)?;

    save.CURVE = UMBRAL;
    save.SRCRAD = 696000.0;
    save.SHAPE = DSKSHP;
    save.TRGCDE = 499;
    save.NSURF = 0;
    save.SRFLST[1] = 0;

    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 10000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::ZZTANINI(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOLOADEDDSKFILES)", OK, ctx)?;

    spicelib::UNLOAD(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //**********************************************************************
    //
    //     ZZTANGNT Error cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZTANGNT: Zero axis vector.", ctx)?;

    save.CURVE = UMBRAL;
    save.SRCRAD = 696000.0;
    save.SHAPE = ELLSHP;
    save.TRGCDE = 499;
    save.NSURF = 0;
    save.SRFLST[1] = 0;

    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 100000000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SCHSTP = 4.0;
    save.SOLTOL = 0.000000000000001;

    spicelib::CLEARD(3, save.AXIS.as_slice_mut());

    spicelib::ZZTANGNT(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        save.SCHSTP,
        save.SOLTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ZEROVECTOR)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZTANGNT: Zero plane reference vector.", ctx)?;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SCHSTP = 4.0;
    save.SOLTOL = 0.000000000000001;

    spicelib::CLEARD(3, save.PLNVEC.as_slice_mut());

    spicelib::ZZTANGNT(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        save.SCHSTP,
        save.SOLTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ZEROVECTOR)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZTANGNT: Bad curve type.", ctx)?;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SCHSTP = 4.0;
    save.SOLTOL = 0.000000000000001;

    save.CURVE = -1;

    spicelib::ZZTANGNT(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        save.SCHSTP,
        save.SOLTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADCURVETYPE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZTANGNT: Bad source radius.", ctx)?;

    save.CURVE = UMBRAL;
    save.SRCRAD = 0.0;
    save.SHAPE = ELLSHP;
    save.TRGCDE = 499;
    save.NSURF = 0;
    save.SRFLST[1] = 0;

    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 100000000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SCHSTP = 4.0;
    save.SOLTOL = 0.000000000000001;

    spicelib::ZZTANGNT(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        save.SCHSTP,
        save.SOLTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADSOURCERADIUS)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZTANGNT: Axis and plane reference vector are linearly dependent.",
        ctx,
    )?;

    save.CURVE = UMBRAL;
    save.SRCRAD = 696000.0;
    save.SHAPE = ELLSHP;
    save.TRGCDE = 499;
    save.NSURF = 0;
    save.SRFLST[1] = 0;

    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 100000000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );

    spicelib::VSCL(2.0, save.AXIS.as_slice(), save.PLNVEC.as_slice_mut());

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SCHSTP = 4.0;
    save.SOLTOL = 0.000000000000001;

    spicelib::ZZTANGNT(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        save.SCHSTP,
        save.SOLTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DEGENERATECASE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZTANGNT: Bad target shape.", ctx)?;

    save.CURVE = UMBRAL;
    save.SRCRAD = 0.0;
    save.SHAPE = -1;
    save.TRGCDE = 499;
    save.NSURF = 0;
    save.SRFLST[1] = 0;

    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 100000000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SCHSTP = 4.0;
    save.SOLTOL = 0.000000000000001;

    spicelib::ZZTANGNT(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        save.SCHSTP,
        save.SOLTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADSOURCERADIUS)", OK, ctx)?;

    //**********************************************************************
    //
    //     ZZTANINI/ZZTANSTA normal cases
    //
    //**********************************************************************

    //
    // These cases are combined because the two entry points
    // work in a cooperative fashion, and because the results of
    // initialization alone are not observable.
    //

    //**********************************************************************
    //
    //     ELLIPSOID target shape
    //
    //**********************************************************************

    //
    // LIMB cases
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Initialize limb computation using ellipsoidal target shape. (Mars)",
        ctx,
    )?;

    save.CURVE = LMBCRV;
    save.SRCRAD = 0.0;
    save.SHAPE = ELLSHP;
    save.TRGCDE = 499;
    save.NSURF = 0;
    save.SRFLST[1] = 0;

    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 10000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::ZZTANINI(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near limb point in cutting half-plane of previous case. Non-occultation case. (Mars)", ctx)?;

    //
    // Find limb point in cutting half-plane of previous case.
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

    save.A = save.RADII[1];
    save.B = save.RADII[2];
    save.C = save.RADII[3];

    spicelib::EDLIMB(
        save.A,
        save.B,
        save.C,
        save.AXIS.as_slice(),
        save.LIMB.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PSV2PL(
        save.ORIGIN.as_slice(),
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.CUT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::INELPL(
        save.LIMB.as_slice(),
        save.CUT.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Generate basis vectors defined by AXIS (primary)
    // and PLNVEC (secondary).
    //
    spicelib::UCRSS(
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.ZVEC.as_slice_mut(),
    );
    spicelib::VHAT(save.AXIS.as_slice(), save.XVEC.as_slice_mut());
    spicelib::UCRSS(
        save.ZVEC.as_slice(),
        save.XVEC.as_slice(),
        save.YVEC.as_slice_mut(),
    );

    if (spicelib::VDOT(save.XPT1.as_slice(), save.YVEC.as_slice()) > 0 as f64) {
        //
        // XPT1 is in the half-plane defined by AXIS and PLNVEC.
        //
        spicelib::VEQU(save.XPT1.as_slice(), save.LPOINT.as_slice_mut());
    } else {
        spicelib::VEQU(save.XPT2.as_slice(), save.LPOINT.as_slice_mut());
    }

    //
    // Generate off-ellipsoid point RPOINT. RPOINT is 1 mm above the
    // surface.
    //
    spicelib::VLCOM(
        1.0,
        save.LPOINT.as_slice(),
        0.000001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    save.ANGLE = spicelib::VSEP(save.AXIS.as_slice(), save.RAYDIR.as_slice(), ctx);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should miss the ellipsoid.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near limb point in cutting half-plane of previous case. Occultation case. (Mars)", ctx)?;
    //
    // Generate off-ellipsoid point RPOINT. RPOINT is 1 mm below the
    // surface.
    //
    spicelib::VLCOM(
        1.0,
        save.LPOINT.as_slice(),
        -0.000001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    save.ANGLE = spicelib::VSEP(save.AXIS.as_slice(), save.RAYDIR.as_slice(), ctx);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should hit the ellipsoid.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, true, OK, ctx)?;

    //
    // The returned point should be on the ellipsoid. Map the
    // point to a point that is on the ellipsoid and check the
    // distance between the two.
    //
    spicelib::EDPNT(
        save.POINT.as_slice(),
        save.A,
        save.B,
        save.C,
        save.NPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"POINT",
        save.POINT.as_slice(),
        b"~~",
        save.NPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be in the plane containing
    // the cutting half-plane.
    //
    spicelib::VPRJP(
        save.POINT.as_slice(),
        save.CUT.as_slice(),
        save.PROJ.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"PROJ",
        save.PROJ.as_slice(),
        b"~~",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be the correct half-plane.
    //
    save.DP = spicelib::VDOT(save.POINT.as_slice(), save.YVEC.as_slice());

    save.TOL = TIGHT;

    testutil::CHCKSD(b"YVEC dot", save.DP, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // The returned point should be on the line containing the ray.
    //
    spicelib::NPLNPT(
        save.AXIS.as_slice(),
        save.RAYDIR.as_slice(),
        save.POINT.as_slice(),
        save.PNEAR.as_slice_mut(),
        &mut save.DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = TIGHT;
    testutil::CHCKAD(
        b"PNEAR",
        save.PNEAR.as_slice(),
        b"~~/",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be on the correct side of the vertex of
    // the line containing the ray.
    //
    spicelib::VSUB(
        save.POINT.as_slice(),
        save.AXIS.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    save.DP = spicelib::VDOT(save.OFFSET.as_slice(), save.RAYDIR.as_slice());

    testutil::CHCKSD(b"AXIS dot", save.DP, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZTANGNT test: create limb point matching that from the previous case. (Mars)",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SCHSTP = 4.0;
    save.SOLTOL = 0.000000000000001;

    spicelib::ZZTANGNT(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        save.SCHSTP,
        save.SOLTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note that RESULT is a cell, not a window.
    //
    save.N = spicelib::CARDD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    //
    // Set absolute tolerance to 1 m. (Can use 10 cm on PC/Linux).
    //
    spicelib::VEQU(save.POINTS.as_slice(), save.TANPNT.as_slice_mut());

    save.TOL = 0.001;
    testutil::CHCKAD(
        b"TANPNT",
        save.TANPNT.as_slice(),
        b"~~",
        save.LPOINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be on the ellipsoid. Map the
    // point to a point that is on the ellipsoid and check the
    // distance between the two.
    //
    spicelib::EDPNT(
        save.TANPNT.as_slice(),
        save.A,
        save.B,
        save.C,
        save.NPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"TANPNT",
        save.TANPNT.as_slice(),
        b"~~",
        save.NPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be in the plane containing
    // the cutting half-plane.
    //
    spicelib::VPRJP(
        save.POINT.as_slice(),
        save.CUT.as_slice(),
        save.PROJ.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"PROJ",
        save.PROJ.as_slice(),
        b"~~/",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be the correct half-plane.
    //
    save.DP = spicelib::VDOT(save.TANPNT.as_slice(), save.YVEC.as_slice());

    save.TOL = TIGHT;

    testutil::CHCKSD(b"YVEC dot", save.DP, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // The returned point should be on the line containing the ray.
    //
    spicelib::NPLNPT(
        save.AXIS.as_slice(),
        save.RAYDIR.as_slice(),
        save.TANPNT.as_slice(),
        save.PNEAR.as_slice_mut(),
        &mut save.DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = MED;
    testutil::CHCKAD(
        b"PNEAR",
        save.PNEAR.as_slice(),
        b"~~/",
        save.TANPNT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be on the correct side of the vertex of
    // the line containing the ray.
    //
    spicelib::VSUB(
        save.TANPNT.as_slice(),
        save.AXIS.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    save.DP = spicelib::VDOT(save.OFFSET.as_slice(), save.RAYDIR.as_slice());

    testutil::CHCKSD(b"AXIS dot", save.DP, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Repeat the test with a different ellipsoidal target. Initialize the system for Phobos.",
        ctx,
    )?;

    save.CURVE = LMBCRV;
    save.SRCRAD = 0.0;
    save.SHAPE = ELLSHP;
    save.TRGCDE = 401;
    save.NSURF = 0;
    save.SRFLST[1] = 0;

    fstr::assign(&mut save.FIXREF, b"IAU_PHOBOS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 10000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::ZZTANINI(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near limb point in cutting half-plane of previous case. Non-occultation case. (Phobos)", ctx)?;

    //
    // Find limb point in cutting half-plane of previous case.
    //
    spicelib::BODVCD(
        401,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.A = save.RADII[1];
    save.B = save.RADII[2];
    save.C = save.RADII[3];

    spicelib::EDLIMB(
        save.A,
        save.B,
        save.C,
        save.AXIS.as_slice(),
        save.LIMB.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PSV2PL(
        save.ORIGIN.as_slice(),
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.CUT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::INELPL(
        save.LIMB.as_slice(),
        save.CUT.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if (spicelib::VDOT(save.XPT1.as_slice(), save.PLNVEC.as_slice()) > 0 as f64) {
        //
        // XPT1 is in the half-plane defined by AXIS and PLNVEC.
        //
        spicelib::VEQU(save.XPT1.as_slice(), save.LPOINT.as_slice_mut());
    } else {
        spicelib::VEQU(save.XPT2.as_slice(), save.LPOINT.as_slice_mut());
    }

    //
    // Get unit direction vector parallel to PLNVEC. Generate
    // off-ellipsoid point.
    //
    spicelib::VLCOM(
        1.0,
        save.LPOINT.as_slice(),
        0.000001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    save.ANGLE = spicelib::VSEP(save.AXIS.as_slice(), save.RAYDIR.as_slice(), ctx);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should miss the ellipsoid.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near limb point in cutting half-plane of previous case. Occultation case. (Phobos)", ctx)?;

    spicelib::VLCOM(
        1.0,
        save.LPOINT.as_slice(),
        -0.000001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    save.ANGLE = spicelib::VSEP(save.AXIS.as_slice(), save.RAYDIR.as_slice(), ctx);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should hit the ellipsoid.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, true, OK, ctx)?;

    //
    // The returned point should be on the ellipsoid. Map the
    // point to a point that is on the ellipsoid and check the
    // distance between the two.
    //
    spicelib::EDPNT(
        save.POINT.as_slice(),
        save.A,
        save.B,
        save.C,
        save.NPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"POINT",
        save.POINT.as_slice(),
        b"~~",
        save.NPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be in the plane containing
    // the cutting half-plane.
    //
    spicelib::VPRJP(
        save.POINT.as_slice(),
        save.CUT.as_slice(),
        save.PROJ.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"PROJ",
        save.PROJ.as_slice(),
        b"~~/",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be the correct half-plane.
    //
    save.DP = spicelib::VDOT(save.POINT.as_slice(), save.YVEC.as_slice());

    save.TOL = TIGHT;

    testutil::CHCKSD(b"YVEC dot", save.DP, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // The returned point should be on the line containing the ray.
    //
    spicelib::NPLNPT(
        save.AXIS.as_slice(),
        save.RAYDIR.as_slice(),
        save.POINT.as_slice(),
        save.PNEAR.as_slice_mut(),
        &mut save.DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = TIGHT;
    testutil::CHCKAD(
        b"PNEAR",
        save.PNEAR.as_slice(),
        b"~~/",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be on the correct side of the vertex of
    // the line containing the ray.
    //
    spicelib::VSUB(
        save.POINT.as_slice(),
        save.AXIS.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    save.DP = spicelib::VDOT(save.OFFSET.as_slice(), save.RAYDIR.as_slice());

    testutil::CHCKSD(b"AXIS dot", save.DP, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZTANGNT test: create limb point matching that from the previous case. (Phobos)",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SCHSTP = 4.0;
    save.SOLTOL = 0.000000000000001;

    spicelib::ZZTANGNT(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        save.SCHSTP,
        save.SOLTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note that RESULT is a cell, not a window.
    //
    save.N = spicelib::CARDD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    //
    // Set absolute tolerance to 1 m. (Can use 10 cm on PC/Linux).
    //
    spicelib::VEQU(save.POINTS.as_slice(), save.TANPNT.as_slice_mut());

    save.TOL = 0.001;
    testutil::CHCKAD(
        b"TANPNT",
        save.TANPNT.as_slice(),
        b"~~",
        save.LPOINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be on the ellipsoid. Map the
    // point to a point that is on the ellipsoid and check the
    // distance between the two.
    //
    spicelib::EDPNT(
        save.TANPNT.as_slice(),
        save.A,
        save.B,
        save.C,
        save.NPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"TANPNT",
        save.TANPNT.as_slice(),
        b"~~",
        save.NPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be in the plane containing
    // the cutting half-plane.
    //
    spicelib::VPRJP(
        save.POINT.as_slice(),
        save.CUT.as_slice(),
        save.PROJ.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"PROJ",
        save.PROJ.as_slice(),
        b"~~/",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be the correct half-plane.
    //
    save.DP = spicelib::VDOT(save.TANPNT.as_slice(), save.YVEC.as_slice());

    save.TOL = TIGHT;

    testutil::CHCKSD(b"YVEC dot", save.DP, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // The returned point should be on the line containing the ray.
    //
    spicelib::NPLNPT(
        save.AXIS.as_slice(),
        save.RAYDIR.as_slice(),
        save.TANPNT.as_slice(),
        save.PNEAR.as_slice_mut(),
        &mut save.DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = 0.000001;
    testutil::CHCKAD(
        b"PNEAR",
        save.PNEAR.as_slice(),
        b"~~/",
        save.TANPNT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be on the correct side of the vertex of
    // the line containing the ray.
    //
    spicelib::VSUB(
        save.TANPNT.as_slice(),
        save.AXIS.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    save.DP = spicelib::VDOT(save.OFFSET.as_slice(), save.RAYDIR.as_slice());

    testutil::CHCKSD(b"AXIS dot", save.DP, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // UMBRAL TERMINATOR cases
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Initialize umbral terminator computation using ellipsoidal target shape. (Mars)",
        ctx,
    )?;

    //
    // Get solar radius.
    //
    spicelib::BODVCD(
        10,
        b"RADII",
        3,
        &mut save.N,
        save.SUNRAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CURVE = UMBRAL;
    save.SRCRAD = save.SUNRAD[1];
    save.SHAPE = ELLSHP;
    save.TRGCDE = 499;
    save.NSURF = 0;
    save.SRFLST[1] = 0;

    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 100000000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::BODVCD(
        499,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZTANINI(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near umbral terminator point in cutting half-plane of previous case. Non-occultation case. (Mars)", ctx)?;

    //
    // Find umbral point in cutting half-plane of previous case.
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

    save.A = save.RADII[1];
    save.B = save.RADII[2];
    save.C = save.RADII[3];
    //
    // We can use ZZTANGNT to "suggest" a solution
    // point.
    //
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SCHSTP = 4.0;
    save.SOLTOL = 0.000000000000001;

    spicelib::ZZTANGNT(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        save.SCHSTP,
        save.SOLTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note that RESULT is a cell, not a window.
    //
    save.N = spicelib::CARDD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    spicelib::VEQU(save.POINTS.as_slice(), save.LPOINT.as_slice_mut());

    //
    // Get unit direction vector parallel to PLNVEC. Generate
    // off-ellipsoid point RPOINT. RPOINT is 1 cm above the
    // terminator point found by ZZTANGNT.
    //
    spicelib::VLCOM(
        1.0,
        save.LPOINT.as_slice(),
        0.00001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    //
    // To generate a near-tangent ray passing through RPOINT,
    // we need to find the ray's vertex on the source. We can
    // do this by finding the limb on the source as seen from
    // RPOINT.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    spicelib::EDLIMB(
        save.SUNRAD[1],
        save.SUNRAD[2],
        save.SUNRAD[3],
        save.OFFSET.as_slice(),
        save.LIMB.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PSV2PL(
        save.ORIGIN.as_slice(),
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.CUT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Caution: LIMB and CUT are expressed with respect to different
    // origins. Shift LIMB so its center is an offset from the
    // target center.
    //
    spicelib::VADD(
        save.AXIS.as_slice(),
        save.LIMB.subarray(1),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VEQU(save.VTEMP.as_slice(), save.LIMB.subarray_mut(1));

    spicelib::INELPL(
        save.LIMB.as_slice(),
        save.CUT.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Generate basis vectors defined by AXIS (primary)
    // and PLNVEC (secondary).
    //
    spicelib::UCRSS(
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.ZVEC.as_slice_mut(),
    );
    spicelib::VHAT(save.AXIS.as_slice(), save.XVEC.as_slice_mut());
    spicelib::UCRSS(
        save.ZVEC.as_slice(),
        save.XVEC.as_slice(),
        save.YVEC.as_slice_mut(),
    );

    if (spicelib::VDOT(save.XPT1.as_slice(), save.YVEC.as_slice()) > 0 as f64) {
        //
        // XPT1 is in the half-plane defined by AXIS and PLNVEC.
        //
        spicelib::VEQU(save.XPT1.as_slice(), save.SUNLPT.as_slice_mut());
    } else {
        spicelib::VEQU(save.XPT2.as_slice(), save.SUNLPT.as_slice_mut());
    }

    //
    // Now generate the ray's direction vector.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    //
    // We can't just use VSEP to find the rotation angle of the
    // tangent ray with respect to AXIS, because the angle may
    // be larger than pi. Compute the angle from the components
    // of RAYDIR.
    //
    save.CDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.XVEC.as_slice());
    save.SDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.YVEC.as_slice());

    save.ANGLE = f64::atan2(save.SDIR, save.CDIR);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should miss the ellipsoid.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near umbral terminator point in cutting half-plane of previous case. Occultation case. (Mars)", ctx)?;

    //
    // Generate off-ellipsoid point RPOINT. RPOINT is 1 cm below the
    // terminator point found by ZZTANGNT.
    //
    spicelib::VLCOM(
        1.0,
        save.LPOINT.as_slice(),
        -0.00001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    spicelib::EDLIMB(
        save.SUNRAD[1],
        save.SUNRAD[2],
        save.SUNRAD[3],
        save.OFFSET.as_slice(),
        save.LIMB.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PSV2PL(
        save.ORIGIN.as_slice(),
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.CUT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Caution: LIMB and CUT are expressed with respect to different
    // origins. Shift LIMB so its center is an offset from the
    // target center.
    //
    spicelib::VADD(
        save.AXIS.as_slice(),
        save.LIMB.subarray(1),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VEQU(save.VTEMP.as_slice(), save.LIMB.subarray_mut(1));

    spicelib::INELPL(
        save.LIMB.as_slice(),
        save.CUT.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.CUT.as_slice(), save.NORMAL.as_slice_mut());

    //
    // The basis from the previous case still applies.
    //
    if (spicelib::VDOT(save.XPT1.as_slice(), save.YVEC.as_slice()) > 0 as f64) {
        //
        // XPT1 is in the half-plane defined by AXIS and PLNVEC.
        //
        spicelib::VEQU(save.XPT1.as_slice(), save.SUNLPT.as_slice_mut());
    } else {
        spicelib::VEQU(save.XPT2.as_slice(), save.SUNLPT.as_slice_mut());
    }

    //
    // Now generate the ray's direction vector.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    //
    // We can't just use VSEP to find the rotation angle of the
    // tangent ray with respect to AXIS, because the angle may
    // be larger than pi. Compute the angle from the components
    // of RAYDIR.
    //
    save.CDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.XVEC.as_slice());
    save.SDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.YVEC.as_slice());

    save.ANGLE = f64::atan2(save.SDIR, save.CDIR);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should hit the ellipsoid.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, true, OK, ctx)?;

    //
    // The returned point should be on the ellipsoid. Map the
    // point to a point that is on the ellipsoid and check the
    // distance between the two.
    //
    spicelib::EDPNT(
        save.POINT.as_slice(),
        save.A,
        save.B,
        save.C,
        save.NPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"POINT",
        save.POINT.as_slice(),
        b"~~/",
        save.NPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be in the plane containing
    // the cutting half-plane.
    //
    spicelib::VPRJP(
        save.POINT.as_slice(),
        save.CUT.as_slice(),
        save.PROJ.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"PROJ",
        save.PROJ.as_slice(),
        b"~~/",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be the correct half-plane.
    //
    save.DP = spicelib::VDOT(save.POINT.as_slice(), save.YVEC.as_slice());

    save.TOL = TIGHT;

    testutil::CHCKSD(b"YVEC dot", save.DP, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // The returned point should be on the line containing the ray.
    //
    spicelib::NPLNPT(
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice(),
        save.POINT.as_slice(),
        save.PNEAR.as_slice_mut(),
        &mut save.DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = MED;
    testutil::CHCKAD(
        b"PNEAR",
        save.PNEAR.as_slice(),
        b"~~/",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be on the correct side of the vertex of
    // the line containing the ray.
    //
    spicelib::VSUB(
        save.POINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    save.D = spicelib::VDOT(save.OFFSET.as_slice(), save.RAYDIR.as_slice());

    testutil::CHCKSD(b"AXIS dot", save.D, b">", 0.0, save.TOL, OK, ctx)?;

    //
    //
    // Repeat previous tests, this time using Phobos as the target.
    //
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Initialize umbral terminator computation using ellipsoidal target shape. (Phobos)",
        ctx,
    )?;

    //
    // Get solar radius.
    //
    spicelib::BODVCD(
        10,
        b"RADII",
        3,
        &mut save.N,
        save.SUNRAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CURVE = UMBRAL;
    save.SRCRAD = save.SUNRAD[1];
    save.SHAPE = ELLSHP;
    save.TRGCDE = 401;
    save.NSURF = 0;
    save.SRFLST[1] = 0;

    fstr::assign(&mut save.FIXREF, b"IAU_PHOBOS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 100000000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::BODVCD(
        499,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZTANINI(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near umbral terminator point in cutting half-plane of previous case. Non-occultation case. (Phobos)", ctx)?;

    //
    // Find umbral point in cutting half-plane of previous case.
    //
    spicelib::BODVCD(
        401,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.A = save.RADII[1];
    save.B = save.RADII[2];
    save.C = save.RADII[3];
    //
    // We can use ZZTANGNT to "suggest" a solution
    // point.
    //
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SCHSTP = 4.0;
    save.SOLTOL = 0.000000000000001;

    spicelib::ZZTANGNT(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        save.SCHSTP,
        save.SOLTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note that RESULT is a cell, not a window.
    //
    save.N = spicelib::CARDD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    spicelib::VEQU(save.POINTS.as_slice(), save.LPOINT.as_slice_mut());

    //
    // Get unit direction vector parallel to PLNVEC. Generate
    // off-ellipsoid point RPOINT. RPOINT is 1 cm above the
    // terminator point found by ZZTANGNT.
    //
    spicelib::VLCOM(
        1.0,
        save.LPOINT.as_slice(),
        0.00001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    //
    // To generate a near-tangent ray passing through RPOINT,
    // we need to find the ray's vertex on the source. We can
    // do this by finding the limb on the source as seen from
    // RPOINT.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    spicelib::EDLIMB(
        save.SUNRAD[1],
        save.SUNRAD[2],
        save.SUNRAD[3],
        save.OFFSET.as_slice(),
        save.LIMB.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PSV2PL(
        save.ORIGIN.as_slice(),
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.CUT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Caution: LIMB and CUT are expressed with respect to different
    // origins. Shift LIMB so its center is an offset from the
    // target center.
    //
    spicelib::VADD(
        save.AXIS.as_slice(),
        save.LIMB.subarray(1),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VEQU(save.VTEMP.as_slice(), save.LIMB.subarray_mut(1));

    spicelib::INELPL(
        save.LIMB.as_slice(),
        save.CUT.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Generate basis vectors defined by AXIS (primary)
    // and PLNVEC (secondary).
    //
    spicelib::UCRSS(
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.ZVEC.as_slice_mut(),
    );
    spicelib::VHAT(save.AXIS.as_slice(), save.XVEC.as_slice_mut());
    spicelib::UCRSS(
        save.ZVEC.as_slice(),
        save.XVEC.as_slice(),
        save.YVEC.as_slice_mut(),
    );

    if (spicelib::VDOT(save.XPT1.as_slice(), save.YVEC.as_slice()) > 0 as f64) {
        //
        // XPT1 is in the half-plane defined by AXIS and PLNVEC.
        //
        spicelib::VEQU(save.XPT1.as_slice(), save.SUNLPT.as_slice_mut());
    } else {
        spicelib::VEQU(save.XPT2.as_slice(), save.SUNLPT.as_slice_mut());
    }

    //
    // Now generate the ray's direction vector.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    //
    // We can't just use VSEP to find the rotation angle of the
    // tangent ray with respect to AXIS, because the angle may
    // be larger than pi. Compute the angle from the components
    // of RAYDIR.
    //
    save.CDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.XVEC.as_slice());
    save.SDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.YVEC.as_slice());

    save.ANGLE = f64::atan2(save.SDIR, save.CDIR);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should miss the ellipsoid.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near umbral terminator point in cutting half-plane of previous case. Occultation case. (Phobos)", ctx)?;

    //
    // Generate off-ellipsoid point RPOINT. RPOINT is 1 cm below the
    // terminator point found by ZZTANGNT.
    //
    spicelib::VLCOM(
        1.0,
        save.LPOINT.as_slice(),
        -0.00001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    spicelib::EDLIMB(
        save.SUNRAD[1],
        save.SUNRAD[2],
        save.SUNRAD[3],
        save.OFFSET.as_slice(),
        save.LIMB.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PSV2PL(
        save.ORIGIN.as_slice(),
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.CUT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Caution: LIMB and CUT are expressed with respect to different
    // origins. Shift LIMB so its center is an offset from the
    // target center.
    //
    spicelib::VADD(
        save.AXIS.as_slice(),
        save.LIMB.subarray(1),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VEQU(save.VTEMP.as_slice(), save.LIMB.subarray_mut(1));

    spicelib::INELPL(
        save.LIMB.as_slice(),
        save.CUT.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.CUT.as_slice(), save.NORMAL.as_slice_mut());

    //
    // The basis from the previous case still applies.
    //
    if (spicelib::VDOT(save.XPT1.as_slice(), save.YVEC.as_slice()) > 0 as f64) {
        //
        // XPT1 is in the half-plane defined by AXIS and PLNVEC.
        //
        spicelib::VEQU(save.XPT1.as_slice(), save.SUNLPT.as_slice_mut());
    } else {
        spicelib::VEQU(save.XPT2.as_slice(), save.SUNLPT.as_slice_mut());
    }

    //
    // Now generate the ray's direction vector.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    //
    // We can't just use VSEP to find the rotation angle of the
    // tangent ray with respect to AXIS, because the angle may
    // be larger than pi. Compute the angle from the components
    // of RAYDIR.
    //
    save.CDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.XVEC.as_slice());
    save.SDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.YVEC.as_slice());

    save.ANGLE = f64::atan2(save.SDIR, save.CDIR);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should hit the ellipsoid.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, true, OK, ctx)?;

    //
    // The returned point should be on the ellipsoid. Map the
    // point to a point that is on the ellipsoid and check the
    // distance between the two.
    //
    spicelib::EDPNT(
        save.POINT.as_slice(),
        save.A,
        save.B,
        save.C,
        save.NPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"POINT",
        save.POINT.as_slice(),
        b"~~/",
        save.NPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be in the plane containing
    // the cutting half-plane.
    //
    spicelib::VPRJP(
        save.POINT.as_slice(),
        save.CUT.as_slice(),
        save.PROJ.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = MED;

    testutil::CHCKAD(
        b"PROJ",
        save.PROJ.as_slice(),
        b"~~/",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be the correct half-plane.
    //
    save.DP = spicelib::VDOT(save.POINT.as_slice(), save.YVEC.as_slice());

    save.TOL = TIGHT;

    testutil::CHCKSD(b"YVEC dot", save.DP, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // The returned point should be on the line containing the ray.
    //
    spicelib::NPLNPT(
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice(),
        save.POINT.as_slice(),
        save.PNEAR.as_slice_mut(),
        &mut save.DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = 0.00001;
    testutil::CHCKAD(
        b"PNEAR",
        save.PNEAR.as_slice(),
        b"~~/",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be on the correct side of the vertex of
    // the line containing the ray.
    //
    spicelib::VSUB(
        save.POINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    save.D = spicelib::VDOT(save.OFFSET.as_slice(), save.RAYDIR.as_slice());

    testutil::CHCKSD(b"AXIS dot", save.D, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // PENUMBRAL TERMINATOR cases
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Initialize penumbral terminator computation using ellipsoidal target shape. (Mars)",
        ctx,
    )?;

    //
    // Get solar radius.
    //
    spicelib::BODVCD(
        10,
        b"RADII",
        3,
        &mut save.N,
        save.SUNRAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CURVE = PNMBRL;
    save.SRCRAD = save.SUNRAD[1];
    save.SHAPE = ELLSHP;
    save.TRGCDE = 499;
    save.NSURF = 0;
    save.SRFLST[1] = 0;

    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 100000000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::BODVCD(
        499,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZTANINI(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near penumbral terminator point in cutting half-plane of previous case. Non-occultation case. (Mars)", ctx)?;

    //
    // Find umbral point in cutting half-plane of previous case.
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

    save.A = save.RADII[1];
    save.B = save.RADII[2];
    save.C = save.RADII[3];
    //
    // We can use ZZTANGNT to "suggest" a solution point.
    //
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SCHSTP = 4.0;
    save.SOLTOL = 0.000000000000001;

    spicelib::ZZTANGNT(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        save.SCHSTP,
        save.SOLTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note that RESULT is a cell, not a window.
    //
    save.N = spicelib::CARDD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    spicelib::VEQU(save.POINTS.as_slice(), save.LPOINT.as_slice_mut());

    //
    // Get unit direction vector parallel to PLNVEC. Generate
    // off-ellipsoid point RPOINT. RPOINT is 1 cm above the
    // terminator point found by ZZTANGNT.
    //
    spicelib::VLCOM(
        1.0,
        save.LPOINT.as_slice(),
        0.00001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    //
    // To generate a near-tangent ray passing through RPOINT,
    // we need to find the ray's vertex on the source. We can
    // do this by finding the limb on the source as seen from
    // RPOINT.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    spicelib::EDLIMB(
        save.SUNRAD[1],
        save.SUNRAD[2],
        save.SUNRAD[3],
        save.OFFSET.as_slice(),
        save.LIMB.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PSV2PL(
        save.ORIGIN.as_slice(),
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.CUT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Caution: LIMB and CUT are expressed with respect to different
    // origins. Shift LIMB so its center is an offset from the
    // target center.
    //
    spicelib::VADD(
        save.AXIS.as_slice(),
        save.LIMB.subarray(1),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VEQU(save.VTEMP.as_slice(), save.LIMB.subarray_mut(1));

    spicelib::INELPL(
        save.LIMB.as_slice(),
        save.CUT.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Generate basis vectors defined by AXIS (primary)
    // and PLNVEC (secondary).
    //
    spicelib::UCRSS(
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.ZVEC.as_slice_mut(),
    );
    spicelib::VHAT(save.AXIS.as_slice(), save.XVEC.as_slice_mut());
    spicelib::UCRSS(
        save.ZVEC.as_slice(),
        save.XVEC.as_slice(),
        save.YVEC.as_slice_mut(),
    );

    //
    // For the penumbral case, the sun limb point should be
    // on the -Y side of the axis.
    //
    if (spicelib::VDOT(save.XPT1.as_slice(), save.YVEC.as_slice()) < 0 as f64) {
        //
        // XPT1 is on the opposite side of the axis bounding the
        // half-plane defined by AXIS and PLNVEC.
        //
        spicelib::VEQU(save.XPT1.as_slice(), save.SUNLPT.as_slice_mut());
    } else {
        spicelib::VEQU(save.XPT2.as_slice(), save.SUNLPT.as_slice_mut());
    }

    //
    // Now generate the ray's direction vector.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    //
    // We can't just use VSEP to find the rotation angle of the
    // tangent ray with respect to AXIS, because the angle may
    // be larger than pi. Compute the angle from the components
    // of RAYDIR.
    //
    save.CDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.XVEC.as_slice());
    save.SDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.YVEC.as_slice());

    save.ANGLE = f64::atan2(save.SDIR, save.CDIR);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should miss the ellipsoid.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near penumbral terminator  point in cutting half-plane of previous case. Occultation case. (Mars)", ctx)?;

    //
    // Generate off-ellipsoid point RPOINT. RPOINT is 1 cm below the
    // terminator point found by ZZTANGNT.
    //
    spicelib::VLCOM(
        1.0,
        save.LPOINT.as_slice(),
        -0.00001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    spicelib::EDLIMB(
        save.SUNRAD[1],
        save.SUNRAD[2],
        save.SUNRAD[3],
        save.OFFSET.as_slice(),
        save.LIMB.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PSV2PL(
        save.ORIGIN.as_slice(),
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.CUT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Caution: LIMB and CUT are expressed with respect to different
    // origins. Shift LIMB so its center is an offset from the
    // target center.
    //
    spicelib::VADD(
        save.AXIS.as_slice(),
        save.LIMB.subarray(1),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VEQU(save.VTEMP.as_slice(), save.LIMB.subarray_mut(1));

    spicelib::INELPL(
        save.LIMB.as_slice(),
        save.CUT.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.CUT.as_slice(), save.NORMAL.as_slice_mut());

    //
    // The basis from the previous case still applies.
    //
    // For the penumbral case, the sun limb point should be
    // on the -Y side of the axis.
    //
    if (spicelib::VDOT(save.XPT1.as_slice(), save.YVEC.as_slice()) < 0 as f64) {
        //
        // XPT1 is on the opposite side of the axis bounding the
        // half-plane defined by AXIS and PLNVEC.
        //
        spicelib::VEQU(save.XPT1.as_slice(), save.SUNLPT.as_slice_mut());
    } else {
        spicelib::VEQU(save.XPT2.as_slice(), save.SUNLPT.as_slice_mut());
    }

    //
    // Now generate the ray's direction vector.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    //
    // We can't just use VSEP to find the rotation angle of the
    // tangent ray with respect to AXIS, because the angle may
    // be larger than pi. Compute the angle from the components
    // of RAYDIR.
    //
    save.CDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.XVEC.as_slice());
    save.SDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.YVEC.as_slice());

    save.ANGLE = f64::atan2(save.SDIR, save.CDIR);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should hit the ellipsoid.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, true, OK, ctx)?;

    //
    // The returned point should be on the ellipsoid. Map the
    // point to a point that is on the ellipsoid and check the
    // distance between the two.
    //
    spicelib::EDPNT(
        save.POINT.as_slice(),
        save.A,
        save.B,
        save.C,
        save.NPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"POINT",
        save.POINT.as_slice(),
        b"~~/",
        save.NPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be in the plane containing
    // the cutting half-plane.
    //
    spicelib::VPRJP(
        save.POINT.as_slice(),
        save.CUT.as_slice(),
        save.PROJ.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"PROJ",
        save.PROJ.as_slice(),
        b"~~/",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be the correct half-plane.
    //
    save.DP = spicelib::VDOT(save.POINT.as_slice(), save.YVEC.as_slice());

    save.TOL = TIGHT;

    testutil::CHCKSD(b"YVEC dot", save.DP, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // The returned point should be on the line containing the ray.
    //
    spicelib::NPLNPT(
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice(),
        save.POINT.as_slice(),
        save.PNEAR.as_slice_mut(),
        &mut save.DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = MED;
    testutil::CHCKAD(
        b"PNEAR",
        save.PNEAR.as_slice(),
        b"~~/",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be on the correct side of the vertex of
    // the line containing the ray.
    //
    spicelib::VSUB(
        save.POINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    save.D = spicelib::VDOT(save.OFFSET.as_slice(), save.RAYDIR.as_slice());

    testutil::CHCKSD(b"AXIS dot", save.D, b">", 0.0, save.TOL, OK, ctx)?;

    //
    //
    // Repeat previous tests, this time using Phobos as the target.
    //
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Initialize penumbral terminator computation using ellipsoidal target shape. (Phobos)",
        ctx,
    )?;

    //
    // Get solar radius.
    //
    spicelib::BODVCD(
        10,
        b"RADII",
        3,
        &mut save.N,
        save.SUNRAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CURVE = PNMBRL;
    save.SRCRAD = save.SUNRAD[1];
    save.SHAPE = ELLSHP;
    save.TRGCDE = 401;
    save.NSURF = 0;
    save.SRFLST[1] = 0;

    fstr::assign(&mut save.FIXREF, b"IAU_PHOBOS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 100000000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::BODVCD(
        401,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZTANINI(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near penumbral terminator point in cutting half-plane of previous case. Non-occultation case. (Phobos)", ctx)?;

    //
    // Find umbral point in cutting half-plane of previous case.
    //
    spicelib::BODVCD(
        401,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.A = save.RADII[1];
    save.B = save.RADII[2];
    save.C = save.RADII[3];
    //
    // We can use ZZTANGNT to "suggest" a solution point.
    //
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SCHSTP = 4.0;
    save.SOLTOL = 0.000000000000001;

    spicelib::ZZTANGNT(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        save.SCHSTP,
        save.SOLTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note that RESULT is a cell, not a window.
    //
    save.N = spicelib::CARDD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    spicelib::VEQU(save.POINTS.as_slice(), save.LPOINT.as_slice_mut());

    //
    // Get unit direction vector parallel to PLNVEC. Generate
    // off-ellipsoid point RPOINT. RPOINT is 1 cm above the
    // terminator point found by ZZTANGNT.
    //
    spicelib::VLCOM(
        1.0,
        save.LPOINT.as_slice(),
        0.00001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    //
    // To generate a near-tangent ray passing through RPOINT,
    // we need to find the ray's vertex on the source. We can
    // do this by finding the limb on the source as seen from
    // RPOINT.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    spicelib::EDLIMB(
        save.SUNRAD[1],
        save.SUNRAD[2],
        save.SUNRAD[3],
        save.OFFSET.as_slice(),
        save.LIMB.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PSV2PL(
        save.ORIGIN.as_slice(),
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.CUT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Caution: LIMB and CUT are expressed with respect to different
    // origins. Shift LIMB so its center is an offset from the
    // target center.
    //
    spicelib::VADD(
        save.AXIS.as_slice(),
        save.LIMB.subarray(1),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VEQU(save.VTEMP.as_slice(), save.LIMB.subarray_mut(1));

    spicelib::INELPL(
        save.LIMB.as_slice(),
        save.CUT.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Generate basis vectors defined by AXIS (primary)
    // and PLNVEC (secondary).
    //
    spicelib::UCRSS(
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.ZVEC.as_slice_mut(),
    );
    spicelib::VHAT(save.AXIS.as_slice(), save.XVEC.as_slice_mut());
    spicelib::UCRSS(
        save.ZVEC.as_slice(),
        save.XVEC.as_slice(),
        save.YVEC.as_slice_mut(),
    );

    //
    // For the penumbral case, the sun limb point should be
    // on the -Y side of the axis.
    //
    if (spicelib::VDOT(save.XPT1.as_slice(), save.YVEC.as_slice()) < 0 as f64) {
        //
        // XPT1 is on the opposite side of the axis bounding the
        // half-plane defined by AXIS and PLNVEC.
        //
        spicelib::VEQU(save.XPT1.as_slice(), save.SUNLPT.as_slice_mut());
    } else {
        spicelib::VEQU(save.XPT2.as_slice(), save.SUNLPT.as_slice_mut());
    }

    //
    // Now generate the ray's direction vector.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    //
    // We can't just use VSEP to find the rotation angle of the
    // tangent ray with respect to AXIS, because the angle may
    // be larger than pi. Compute the angle from the components
    // of RAYDIR.
    //
    save.CDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.XVEC.as_slice());
    save.SDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.YVEC.as_slice());

    save.ANGLE = f64::atan2(save.SDIR, save.CDIR);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should miss the ellipsoid.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near penumbral terminator  point in cutting half-plane of previous case. Occultation case. (Phobos)", ctx)?;

    //
    // Generate off-ellipsoid point RPOINT. RPOINT is 1 cm below the
    // terminator point found by ZZTANGNT.
    //
    spicelib::VLCOM(
        1.0,
        save.LPOINT.as_slice(),
        -0.00001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    spicelib::EDLIMB(
        save.SUNRAD[1],
        save.SUNRAD[2],
        save.SUNRAD[3],
        save.OFFSET.as_slice(),
        save.LIMB.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PSV2PL(
        save.ORIGIN.as_slice(),
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.CUT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Caution: LIMB and CUT are expressed with respect to different
    // origins. Shift LIMB so its center is an offset from the
    // target center.
    //
    spicelib::VADD(
        save.AXIS.as_slice(),
        save.LIMB.subarray(1),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VEQU(save.VTEMP.as_slice(), save.LIMB.subarray_mut(1));

    spicelib::INELPL(
        save.LIMB.as_slice(),
        save.CUT.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.CUT.as_slice(), save.NORMAL.as_slice_mut());

    //
    // The basis from the previous case still applies.
    //
    // For the penumbral case, the sun limb point should be
    // on the -Y side of the axis.
    //
    if (spicelib::VDOT(save.XPT1.as_slice(), save.YVEC.as_slice()) < 0 as f64) {
        //
        // XPT1 is on the opposite side of the axis bounding the
        // half-plane defined by AXIS and PLNVEC.
        //
        spicelib::VEQU(save.XPT1.as_slice(), save.SUNLPT.as_slice_mut());
    } else {
        spicelib::VEQU(save.XPT2.as_slice(), save.SUNLPT.as_slice_mut());
    }

    //
    // Now generate the ray's direction vector.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    //
    // We can't just use VSEP to find the rotation angle of the
    // tangent ray with respect to AXIS, because the angle may
    // be larger than pi. Compute the angle from the components
    // of RAYDIR.
    //
    save.CDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.XVEC.as_slice());
    save.SDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.YVEC.as_slice());

    save.ANGLE = f64::atan2(save.SDIR, save.CDIR);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should hit the ellipsoid.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, true, OK, ctx)?;

    //
    // The returned point should be on the ellipsoid. Map the
    // point to a point that is on the ellipsoid and check the
    // distance between the two.
    //
    spicelib::EDPNT(
        save.POINT.as_slice(),
        save.A,
        save.B,
        save.C,
        save.NPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"POINT",
        save.POINT.as_slice(),
        b"~~/",
        save.NPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be in the plane containing
    // the cutting half-plane.
    //
    spicelib::VPRJP(
        save.POINT.as_slice(),
        save.CUT.as_slice(),
        save.PROJ.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = 0.00001;

    testutil::CHCKAD(
        b"PROJ",
        save.PROJ.as_slice(),
        b"~~/",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be the correct half-plane.
    //
    save.DP = spicelib::VDOT(save.POINT.as_slice(), save.YVEC.as_slice());

    save.TOL = TIGHT;

    testutil::CHCKSD(b"YVEC dot", save.DP, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // The returned point should be on the line containing the ray.
    //
    spicelib::NPLNPT(
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice(),
        save.POINT.as_slice(),
        save.PNEAR.as_slice_mut(),
        &mut save.DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = 0.00001;
    testutil::CHCKAD(
        b"PNEAR",
        save.PNEAR.as_slice(),
        b"~~/",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be on the correct side of the vertex of
    // the line containing the ray.
    //
    spicelib::VSUB(
        save.POINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    save.D = spicelib::VDOT(save.OFFSET.as_slice(), save.RAYDIR.as_slice());

    testutil::CHCKSD(b"AXIS dot", save.D, b">", 0.0, save.TOL, OK, ctx)?;

    //**********************************************************************
    //
    //     DSK target shape
    //
    //**********************************************************************

    //
    // LIMB cases
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Initialize limb computation using DSK target shape. Empty surface list. (Mars)",
        ctx,
    )?;

    //
    // Load DSKs.
    //
    spicelib::FURNSH(DSK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(DSK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(DSK3, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Use all surfaces (empty surface list).
    //

    save.CURVE = LMBCRV;
    save.SRCRAD = 0.0;
    save.SHAPE = DSKSHP;
    save.TRGCDE = 499;
    save.NSURF = 0;
    save.SRFLST[1] = 0;

    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 10000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::ZZTANINI(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near DSK limb point in cutting half-plane of previous case. Non-occultation case. (Mars)", ctx)?;

    //
    // Find limb point in cutting half-plane of previous case.
    //

    save.SCHSTP = 4.0;
    save.SOLTOL = 0.000000000000001;

    spicelib::ZZTANGNT(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        save.SCHSTP,
        save.SOLTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note that RESULT is a cell, not a window.
    //
    save.N = spicelib::CARDD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    spicelib::VEQU(save.POINTS.as_slice(), save.TANPNT.as_slice_mut());

    //
    // Generate off-DSK point RPOINT. RPOINT is 1 cm above the
    // surface.
    //
    spicelib::VLCOM(
        1.0,
        save.TANPNT.as_slice(),
        0.00001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    save.ANGLE = spicelib::VSEP(save.AXIS.as_slice(), save.RAYDIR.as_slice(), ctx);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should miss the DSK surface.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near DSK limb point in cutting half-plane of previous case. Occultation case. (Mars)", ctx)?;
    //
    // Generate off-DSK point RPOINT. RPOINT is 1 cm below the
    // surface.
    //
    spicelib::VLCOM(
        1.0,
        save.TANPNT.as_slice(),
        -0.00001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    save.ANGLE = spicelib::VSEP(save.AXIS.as_slice(), save.RAYDIR.as_slice(), ctx);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should hit the DSK surface.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, true, OK, ctx)?;

    //
    // The returned point should be very close to the DSK surface. Map
    // the point to a point that is on the surface and check the
    // distance between the two.
    //
    spicelib::VSCL(2.0, save.POINT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VMINUS(save.VERTEX.as_slice(), save.DIR.as_slice_mut());

    spicelib::DSKXV(
        false,
        &save.TARGET,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.ET,
        &save.FIXREF,
        1,
        save.VERTEX.as_slice(),
        save.DIR.as_slice(),
        save.NPT.as_slice_mut(),
        std::slice::from_mut(&mut save.FOUND),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"POINT",
        save.POINT.as_slice(),
        b"~~/",
        save.NPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be in the plane containing
    // the cutting half-plane.
    //
    spicelib::VPRJP(
        save.POINT.as_slice(),
        save.CUT.as_slice(),
        save.PROJ.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"PROJ",
        save.PROJ.as_slice(),
        b"~~",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be the correct half-plane.
    //
    save.DP = spicelib::VDOT(save.POINT.as_slice(), save.YVEC.as_slice());

    save.TOL = TIGHT;

    testutil::CHCKSD(b"YVEC dot", save.DP, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // The returned point should be on the line containing the ray.
    //
    spicelib::NPLNPT(
        save.AXIS.as_slice(),
        save.RAYDIR.as_slice(),
        save.POINT.as_slice(),
        save.PNEAR.as_slice_mut(),
        &mut save.DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = TIGHT;
    testutil::CHCKAD(
        b"PNEAR",
        save.PNEAR.as_slice(),
        b"~~/",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be on the correct side of the vertex of
    // the line containing the ray.
    //
    spicelib::VSUB(
        save.POINT.as_slice(),
        save.AXIS.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    save.DP = spicelib::VDOT(save.OFFSET.as_slice(), save.RAYDIR.as_slice());

    testutil::CHCKSD(b"AXIS dot", save.DP, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Initialize limb computation using DSK target shape. Specify both surfaces in surface list. (Mars)", ctx)?;

    //
    // Use all surfaces (empty surface list).
    //

    save.CURVE = LMBCRV;
    save.SRCRAD = 0.0;
    save.SHAPE = DSKSHP;
    save.TRGCDE = 499;
    save.NSURF = 2;
    save.SRFLST[1] = 1;
    save.SRFLST[2] = 2;

    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 10000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::ZZTANINI(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZTANGNT test: compare DSK limb point to that found in previous case. (Mars)",
        ctx,
    )?;

    //
    // Find limb point in cutting half-plane of previous case.
    //

    save.SCHSTP = 4.0;
    save.SOLTOL = 0.000000000000001;

    spicelib::ZZTANGNT(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        save.SCHSTP,
        save.SOLTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note that RESULT is a cell, not a window.
    //
    save.N = spicelib::CARDD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    spicelib::VEQU(save.POINTS.as_slice(), save.LPOINT.as_slice_mut());
    //
    // We expect an exact match.
    //
    save.TOL = 0.0;

    testutil::CHCKAD(
        b"LPOINT",
        save.LPOINT.as_slice(),
        b"~~/",
        save.TANPNT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Initialize limb computation using DSK target shape. Use surface 2. (Mars)",
        ctx,
    )?;

    //
    // Use all surfaces (empty surface list).
    //

    save.CURVE = LMBCRV;
    save.SRCRAD = 0.0;
    save.SHAPE = DSKSHP;
    save.TRGCDE = 499;
    save.NSURF = 1;
    save.SRFLST[1] = 2;

    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 10000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::ZZTANINI(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near DSK limb point in cutting half-plane of previous case. Non-occultation case. Use surface 2. (Mars)", ctx)?;

    //
    // Find limb point in cutting half-plane of previous case.
    //

    save.SCHSTP = 4.0;
    save.SOLTOL = 0.000000000000001;

    spicelib::ZZTANGNT(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        save.SCHSTP,
        save.SOLTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note that RESULT is a cell, not a window.
    //
    save.N = spicelib::CARDD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    spicelib::VEQU(save.POINTS.as_slice(), save.TANPNT.as_slice_mut());

    //
    // Generate off-DSK point RPOINT. RPOINT is 1 cm above the
    // surface.
    //
    spicelib::VLCOM(
        1.0,
        save.TANPNT.as_slice(),
        0.00001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    save.ANGLE = spicelib::VSEP(save.AXIS.as_slice(), save.RAYDIR.as_slice(), ctx);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should miss the DSK surface.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near DSK limb point in cutting half-plane of previous case. Occultation case.  Use surface 2. (Mars)", ctx)?;
    //
    // Generate off-ellipsoid point RPOINT. RPOINT is 1 cm below the
    // surface.
    //
    spicelib::VLCOM(
        1.0,
        save.TANPNT.as_slice(),
        -0.00001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    save.ANGLE = spicelib::VSEP(save.AXIS.as_slice(), save.RAYDIR.as_slice(), ctx);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should hit the DSK surface.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, true, OK, ctx)?;

    //
    // The returned point should be very close to the DSK surface. Map
    // the point to a point that is on the surface and check the
    // distance between the two.
    //
    spicelib::VSCL(2.0, save.POINT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VMINUS(save.VERTEX.as_slice(), save.DIR.as_slice_mut());

    spicelib::DSKXV(
        false,
        &save.TARGET,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.ET,
        &save.FIXREF,
        1,
        save.VERTEX.as_slice(),
        save.DIR.as_slice(),
        save.NPT.as_slice_mut(),
        std::slice::from_mut(&mut save.FOUND),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"POINT",
        save.POINT.as_slice(),
        b"~~/",
        save.NPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be in the plane containing
    // the cutting half-plane.
    //
    spicelib::VPRJP(
        save.POINT.as_slice(),
        save.CUT.as_slice(),
        save.PROJ.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"PROJ",
        save.PROJ.as_slice(),
        b"~~",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be the correct half-plane.
    //
    save.DP = spicelib::VDOT(save.POINT.as_slice(), save.YVEC.as_slice());

    save.TOL = TIGHT;

    testutil::CHCKSD(b"YVEC dot", save.DP, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // The returned point should be on the line containing the ray.
    //
    spicelib::NPLNPT(
        save.AXIS.as_slice(),
        save.RAYDIR.as_slice(),
        save.POINT.as_slice(),
        save.PNEAR.as_slice_mut(),
        &mut save.DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = TIGHT;
    testutil::CHCKAD(
        b"PNEAR",
        save.PNEAR.as_slice(),
        b"~~/",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be on the correct side of the vertex of
    // the line containing the ray.
    //
    spicelib::VSUB(
        save.POINT.as_slice(),
        save.AXIS.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    save.DP = spicelib::VDOT(save.OFFSET.as_slice(), save.RAYDIR.as_slice());

    testutil::CHCKSD(b"AXIS dot", save.DP, b">", 0.0, save.TOL, OK, ctx)?;

    //
    //
    //     Use Phobos as the target.
    //
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Initialize limb computation using DSK target shape. Empty surface list. (Phobos)",
        ctx,
    )?;

    //
    // Use all surfaces (empty surface list).
    //

    save.CURVE = LMBCRV;
    save.SRCRAD = 0.0;
    save.SHAPE = DSKSHP;
    save.TRGCDE = 401;
    fstr::assign(&mut save.TARGET, b"PHOBOS");
    save.NSURF = 0;
    save.SRFLST[1] = 0;

    fstr::assign(&mut save.FIXREF, b"IAU_PHOBOS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 10000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::ZZTANINI(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near DSK limb point in cutting half-plane of previous case. Non-occultation case. (Phobos)", ctx)?;

    //
    // Find limb point in cutting half-plane of previous case.
    //

    save.SCHSTP = 4.0;
    save.SOLTOL = 0.000000000000001;

    spicelib::ZZTANGNT(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        save.SCHSTP,
        save.SOLTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note that RESULT is a cell, not a window.
    //
    save.N = spicelib::CARDD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    spicelib::VEQU(save.POINTS.as_slice(), save.TANPNT.as_slice_mut());

    //
    // Generate off-DSK point RPOINT. RPOINT is 1 cm above the
    // surface.
    //
    spicelib::VLCOM(
        1.0,
        save.TANPNT.as_slice(),
        0.00001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    save.ANGLE = spicelib::VSEP(save.AXIS.as_slice(), save.RAYDIR.as_slice(), ctx);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should miss the DSK surface.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near DSK limb point in cutting half-plane of previous case. Occultation case. (Phobos)", ctx)?;
    //
    // Generate off-DSK point RPOINT. RPOINT is 1 cm below the
    // surface.
    //
    spicelib::VLCOM(
        1.0,
        save.TANPNT.as_slice(),
        -0.00001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    save.ANGLE = spicelib::VSEP(save.AXIS.as_slice(), save.RAYDIR.as_slice(), ctx);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should hit the DSK surface.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, true, OK, ctx)?;

    //
    // The returned point should be very close to the DSK surface. Map
    // the point to a point that is on the surface and check the
    // distance between the two.
    //
    spicelib::VSCL(2.0, save.POINT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VMINUS(save.VERTEX.as_slice(), save.DIR.as_slice_mut());

    spicelib::DSKXV(
        false,
        &save.TARGET,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.ET,
        &save.FIXREF,
        1,
        save.VERTEX.as_slice(),
        save.DIR.as_slice(),
        save.NPT.as_slice_mut(),
        std::slice::from_mut(&mut save.FOUND),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"POINT",
        save.POINT.as_slice(),
        b"~~/",
        save.NPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be in the plane containing
    // the cutting half-plane.
    //
    spicelib::VPRJP(
        save.POINT.as_slice(),
        save.CUT.as_slice(),
        save.PROJ.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"PROJ",
        save.PROJ.as_slice(),
        b"~~",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be the correct half-plane.
    //
    save.DP = spicelib::VDOT(save.POINT.as_slice(), save.YVEC.as_slice());

    save.TOL = TIGHT;

    testutil::CHCKSD(b"YVEC dot", save.DP, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // The returned point should be on the line containing the ray.
    //
    spicelib::NPLNPT(
        save.AXIS.as_slice(),
        save.RAYDIR.as_slice(),
        save.POINT.as_slice(),
        save.PNEAR.as_slice_mut(),
        &mut save.DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = TIGHT;
    testutil::CHCKAD(
        b"PNEAR",
        save.PNEAR.as_slice(),
        b"~~/",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be on the correct side of the vertex of
    // the line containing the ray.
    //
    spicelib::VSUB(
        save.POINT.as_slice(),
        save.AXIS.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    save.DP = spicelib::VDOT(save.OFFSET.as_slice(), save.RAYDIR.as_slice());

    testutil::CHCKSD(b"AXIS dot", save.DP, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Initialize limb computation using DSK target shape. Specify both surfaces in surface list. (Phobos)", ctx)?;

    //
    // Use all surfaces (empty surface list).
    //

    save.CURVE = LMBCRV;
    save.SRCRAD = 0.0;
    save.SHAPE = DSKSHP;
    save.TRGCDE = 401;
    save.NSURF = 2;
    save.SRFLST[1] = 1;
    save.SRFLST[2] = 2;

    fstr::assign(&mut save.FIXREF, b"IAU_PHOBOS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 10000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::ZZTANINI(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZTANGNT test: compare DSK limb point to that found in previous case. (Phobos)",
        ctx,
    )?;

    //
    // Find limb point in cutting half-plane of previous case.
    //

    save.SCHSTP = 4.0;
    save.SOLTOL = 0.000000000000001;

    spicelib::ZZTANGNT(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        save.SCHSTP,
        save.SOLTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note that RESULT is a cell, not a window.
    //
    save.N = spicelib::CARDD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    spicelib::VEQU(save.POINTS.as_slice(), save.LPOINT.as_slice_mut());
    //
    // We expect an exact match.
    //
    save.TOL = 0.0;

    testutil::CHCKAD(
        b"LPOINT",
        save.LPOINT.as_slice(),
        b"~~/",
        save.TANPNT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Initialize limb computation using DSK target shape. Use surface 2. (Phobos)",
        ctx,
    )?;

    //
    // Use all surfaces (empty surface list).
    //

    save.CURVE = LMBCRV;
    save.SRCRAD = 0.0;
    save.SHAPE = DSKSHP;
    save.TRGCDE = 401;
    fstr::assign(&mut save.TARGET, b"PHOBOS");
    save.NSURF = 1;
    save.SRFLST[1] = 2;

    fstr::assign(&mut save.FIXREF, b"IAU_PHOBOS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 10000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::ZZTANINI(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near DSK limb point in cutting half-plane of previous case. Non-occultation case. Use surface 2. (Phobos)", ctx)?;

    //
    // Find limb point in cutting half-plane of previous case.
    //

    save.SCHSTP = 4.0;
    save.SOLTOL = 0.000000000000001;

    spicelib::ZZTANGNT(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        save.SCHSTP,
        save.SOLTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note that RESULT is a cell, not a window.
    //
    save.N = spicelib::CARDD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    spicelib::VEQU(save.POINTS.as_slice(), save.TANPNT.as_slice_mut());

    //
    // Generate off-DSK point RPOINT. RPOINT is 1 cm above the
    // surface.
    //
    spicelib::VLCOM(
        1.0,
        save.TANPNT.as_slice(),
        0.00001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    save.ANGLE = spicelib::VSEP(save.AXIS.as_slice(), save.RAYDIR.as_slice(), ctx);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should miss the DSK surface.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near DSK limb point in cutting half-plane of previous case. Occultation case.  Use surface 2. (Phobos)", ctx)?;
    //
    // Generate off-ellipsoid point RPOINT. RPOINT is 1 cm below the
    // surface.
    //
    spicelib::VLCOM(
        1.0,
        save.TANPNT.as_slice(),
        -0.00001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    save.ANGLE = spicelib::VSEP(save.AXIS.as_slice(), save.RAYDIR.as_slice(), ctx);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should hit the DSK surface.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, true, OK, ctx)?;

    //
    // The returned point should be very close to the DSK surface. Map
    // the point to a point that is on the surface and check the
    // distance between the two.
    //
    spicelib::VSCL(2.0, save.POINT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VMINUS(save.VERTEX.as_slice(), save.DIR.as_slice_mut());

    spicelib::DSKXV(
        false,
        &save.TARGET,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.ET,
        &save.FIXREF,
        1,
        save.VERTEX.as_slice(),
        save.DIR.as_slice(),
        save.NPT.as_slice_mut(),
        std::slice::from_mut(&mut save.FOUND),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"POINT",
        save.POINT.as_slice(),
        b"~~/",
        save.NPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be in the plane containing
    // the cutting half-plane.
    //
    spicelib::VPRJP(
        save.POINT.as_slice(),
        save.CUT.as_slice(),
        save.PROJ.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"PROJ",
        save.PROJ.as_slice(),
        b"~~",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be the correct half-plane.
    //
    save.DP = spicelib::VDOT(save.POINT.as_slice(), save.YVEC.as_slice());

    save.TOL = TIGHT;

    testutil::CHCKSD(b"YVEC dot", save.DP, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // The returned point should be on the line containing the ray.
    //
    spicelib::NPLNPT(
        save.AXIS.as_slice(),
        save.RAYDIR.as_slice(),
        save.POINT.as_slice(),
        save.PNEAR.as_slice_mut(),
        &mut save.DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = TIGHT;
    testutil::CHCKAD(
        b"PNEAR",
        save.PNEAR.as_slice(),
        b"~~/",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be on the correct side of the vertex of
    // the line containing the ray.
    //
    spicelib::VSUB(
        save.POINT.as_slice(),
        save.AXIS.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    save.DP = spicelib::VDOT(save.OFFSET.as_slice(), save.RAYDIR.as_slice());

    testutil::CHCKSD(b"AXIS dot", save.DP, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // DSK UMBRAL TERMINATOR cases
    //

    //
    //
    // Mars is the target.
    //
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Initialize umbral terminator computation using DSK target shape. Use empty surface list. (Mars)", ctx)?;

    //
    // Get solar radius.
    //
    spicelib::BODVCD(
        10,
        b"RADII",
        3,
        &mut save.N,
        save.SUNRAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CURVE = UMBRAL;
    save.SRCRAD = save.SUNRAD[1];
    save.SHAPE = DSKSHP;
    save.TRGCDE = 499;
    fstr::assign(&mut save.TARGET, b"MARS");
    save.NSURF = 0;
    save.SRFLST[1] = 0;

    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 100000000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::BODVCD(
        499,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZTANINI(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near DSK umbral terminator point in cutting half-plane of previous case. Non-occultation case. Empty surface list. (Mars)", ctx)?;

    //
    // Find umbral point in cutting half-plane of previous case.
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

    save.A = save.RADII[1];
    save.B = save.RADII[2];
    save.C = save.RADII[3];
    //
    // We can use ZZTANGNT to "suggest" a solution
    // point.
    //
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SCHSTP = 4.0;
    save.SOLTOL = 0.000000000000001;

    spicelib::ZZTANGNT(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        save.SCHSTP,
        save.SOLTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note that RESULT is a cell, not a window.
    //
    save.N = spicelib::CARDD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    spicelib::VEQU(save.POINTS.as_slice(), save.TANPNT.as_slice_mut());

    //
    // Get unit direction vector parallel to PLNVEC. Generate
    // off-DSK surface point RPOINT. RPOINT is 1 cm above the
    // terminator point found by ZZTANGNT.
    //
    spicelib::VLCOM(
        1.0,
        save.TANPNT.as_slice(),
        0.00001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    //
    // To generate a near-tangent ray passing through RPOINT,
    // we need to find the ray's vertex on the source. We can
    // do this by finding the limb on the source as seen from
    // RPOINT.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    spicelib::EDLIMB(
        save.SUNRAD[1],
        save.SUNRAD[2],
        save.SUNRAD[3],
        save.OFFSET.as_slice(),
        save.LIMB.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PSV2PL(
        save.ORIGIN.as_slice(),
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.CUT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Caution: LIMB and CUT are expressed with respect to different
    // origins. Shift LIMB so its center is an offset from the
    // target center.
    //
    spicelib::VADD(
        save.AXIS.as_slice(),
        save.LIMB.subarray(1),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VEQU(save.VTEMP.as_slice(), save.LIMB.subarray_mut(1));

    spicelib::INELPL(
        save.LIMB.as_slice(),
        save.CUT.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Generate basis vectors defined by AXIS (primary)
    // and PLNVEC (secondary).
    //
    spicelib::UCRSS(
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.ZVEC.as_slice_mut(),
    );
    spicelib::VHAT(save.AXIS.as_slice(), save.XVEC.as_slice_mut());
    spicelib::UCRSS(
        save.ZVEC.as_slice(),
        save.XVEC.as_slice(),
        save.YVEC.as_slice_mut(),
    );

    if (spicelib::VDOT(save.XPT1.as_slice(), save.YVEC.as_slice()) > 0 as f64) {
        //
        // XPT1 is in the half-plane defined by AXIS and PLNVEC.
        //
        spicelib::VEQU(save.XPT1.as_slice(), save.SUNLPT.as_slice_mut());
    } else {
        spicelib::VEQU(save.XPT2.as_slice(), save.SUNLPT.as_slice_mut());
    }

    //
    // Now generate the ray's direction vector.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    //
    // We can't just use VSEP to find the rotation angle of the
    // tangent ray with respect to AXIS, because the angle may
    // be larger than pi. Compute the angle from the components
    // of RAYDIR.
    //
    save.CDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.XVEC.as_slice());
    save.SDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.YVEC.as_slice());

    save.ANGLE = f64::atan2(save.SDIR, save.CDIR);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should miss the DSK surface.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near DSK umbral terminator point in cutting half-plane of previous case. Occultation case. Empty surface list. (Mars)", ctx)?;

    //
    // Generate off-DSK point RPOINT. RPOINT is 1 cm below the
    // terminator point found by ZZTANGNT.
    //
    spicelib::VLCOM(
        1.0,
        save.TANPNT.as_slice(),
        -0.00001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    //
    // Find limb on the source (sun) as seen from the tangent point.
    //
    spicelib::EDLIMB(
        save.SUNRAD[1],
        save.SUNRAD[2],
        save.SUNRAD[3],
        save.OFFSET.as_slice(),
        save.LIMB.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PSV2PL(
        save.ORIGIN.as_slice(),
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.CUT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Caution: LIMB and CUT are expressed with respect to different
    // origins. Shift LIMB so its center is an offset from the
    // target center.
    //
    spicelib::VADD(
        save.AXIS.as_slice(),
        save.LIMB.subarray(1),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VEQU(save.VTEMP.as_slice(), save.LIMB.subarray_mut(1));

    spicelib::INELPL(
        save.LIMB.as_slice(),
        save.CUT.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.CUT.as_slice(), save.NORMAL.as_slice_mut());

    //
    // The basis from the previous case still applies.
    //
    if (spicelib::VDOT(save.XPT1.as_slice(), save.YVEC.as_slice()) > 0 as f64) {
        //
        // XPT1 is in the half-plane defined by AXIS and PLNVEC.
        //
        spicelib::VEQU(save.XPT1.as_slice(), save.SUNLPT.as_slice_mut());
    } else {
        spicelib::VEQU(save.XPT2.as_slice(), save.SUNLPT.as_slice_mut());
    }

    //
    // Now generate the ray's direction vector.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    //
    // We can't just use VSEP to find the rotation angle of the
    // tangent ray with respect to AXIS, because the angle may
    // be larger than pi. Compute the angle from the components
    // of RAYDIR.
    //
    save.CDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.XVEC.as_slice());
    save.SDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.YVEC.as_slice());

    save.ANGLE = f64::atan2(save.SDIR, save.CDIR);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should hit the DSK surface.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, true, OK, ctx)?;

    //
    // The returned point should be very close to the DSK surface. Map
    // the point to a point that is on the surface and check the
    // distance between the two.
    //
    spicelib::VSCL(2.0, save.POINT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VMINUS(save.VERTEX.as_slice(), save.DIR.as_slice_mut());

    spicelib::DSKXV(
        false,
        &save.TARGET,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.ET,
        &save.FIXREF,
        1,
        save.VERTEX.as_slice(),
        save.DIR.as_slice(),
        save.NPT.as_slice_mut(),
        std::slice::from_mut(&mut save.FOUND),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"POINT",
        save.POINT.as_slice(),
        b"~~/",
        save.NPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be in the plane containing
    // the cutting half-plane.
    //
    spicelib::VPRJP(
        save.POINT.as_slice(),
        save.CUT.as_slice(),
        save.PROJ.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"PROJ",
        save.PROJ.as_slice(),
        b"~~/",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be the correct half-plane.
    //
    save.DP = spicelib::VDOT(save.POINT.as_slice(), save.YVEC.as_slice());

    save.TOL = TIGHT;

    testutil::CHCKSD(b"YVEC dot", save.DP, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // The returned point should be on the line containing the ray.
    //
    spicelib::NPLNPT(
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice(),
        save.POINT.as_slice(),
        save.PNEAR.as_slice_mut(),
        &mut save.DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = MED;
    testutil::CHCKAD(
        b"PNEAR",
        save.PNEAR.as_slice(),
        b"~~/",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be on the correct side of the vertex of
    // the line containing the ray.
    //
    spicelib::VSUB(
        save.POINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    save.D = spicelib::VDOT(save.OFFSET.as_slice(), save.RAYDIR.as_slice());

    testutil::CHCKSD(b"AXIS dot", save.D, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Initialize umbral terminator computation using DSK target shape. Specify both surfaces in surface list. (Mars)", ctx)?;

    //
    // Use all surfaces (empty surface list).
    //

    save.CURVE = UMBRAL;
    save.SRCRAD = save.SUNRAD[1];
    save.SHAPE = DSKSHP;
    fstr::assign(&mut save.TARGET, b"MARS");
    save.TRGCDE = 499;
    save.NSURF = 2;
    save.SRFLST[1] = 1;
    save.SRFLST[2] = 2;

    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 100000000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::ZZTANINI(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZTANGNT test: compare DSK umbral terminator point to that found in previous case. (Mars)", ctx)?;

    //
    // Find limb point in cutting half-plane of previous case.
    //

    save.SCHSTP = 4.0;
    save.SOLTOL = 0.000000000000001;

    spicelib::ZZTANGNT(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        save.SCHSTP,
        save.SOLTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note that RESULT is a cell, not a window.
    //
    save.N = spicelib::CARDD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    spicelib::VEQU(save.POINTS.as_slice(), save.POINT.as_slice_mut());
    //
    // We expect an exact match.
    //
    save.TOL = 0.0;

    testutil::CHCKAD(
        b"POINT",
        save.POINT.as_slice(),
        b"~~/",
        save.TANPNT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Initialize umbral terminator computation using DSK target shape. Use surface 2. (Mars)",
        ctx,
    )?;

    //
    // Use all surfaces (empty surface list).
    //

    save.CURVE = UMBRAL;
    save.SRCRAD = save.SUNRAD[1];
    save.SHAPE = DSKSHP;
    save.TRGCDE = 499;
    save.NSURF = 1;
    save.SRFLST[1] = 2;

    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 100000000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::ZZTANINI(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near DSK umbral terminator point in cutting half-plane of previous case. Non-occultation case. Use surface 2. (Mars)", ctx)?;

    //
    // Find umbral point in cutting half-plane of previous case.
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

    save.A = save.RADII[1];
    save.B = save.RADII[2];
    save.C = save.RADII[3];
    //
    // We can use ZZTANGNT to "suggest" a solution
    // point.
    //
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SCHSTP = 4.0;
    save.SOLTOL = 0.000000000000001;

    spicelib::ZZTANGNT(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        save.SCHSTP,
        save.SOLTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note that RESULT is a cell, not a window.
    //
    save.N = spicelib::CARDD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    spicelib::VEQU(save.POINTS.as_slice(), save.TANPNT.as_slice_mut());

    //
    // Get unit direction vector parallel to PLNVEC. Generate
    // off-DSK surface point RPOINT. RPOINT is 1 cm above the
    // terminator point found by ZZTANGNT.
    //
    spicelib::VLCOM(
        1.0,
        save.TANPNT.as_slice(),
        0.00001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    //
    // To generate a near-tangent ray passing through RPOINT,
    // we need to find the ray's vertex on the source. We can
    // do this by finding the limb on the source as seen from
    // RPOINT.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    spicelib::EDLIMB(
        save.SUNRAD[1],
        save.SUNRAD[2],
        save.SUNRAD[3],
        save.OFFSET.as_slice(),
        save.LIMB.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PSV2PL(
        save.ORIGIN.as_slice(),
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.CUT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Caution: LIMB and CUT are expressed with respect to different
    // origins. Shift LIMB so its center is an offset from the
    // target center.
    //
    spicelib::VADD(
        save.AXIS.as_slice(),
        save.LIMB.subarray(1),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VEQU(save.VTEMP.as_slice(), save.LIMB.subarray_mut(1));

    spicelib::INELPL(
        save.LIMB.as_slice(),
        save.CUT.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Generate basis vectors defined by AXIS (primary)
    // and PLNVEC (secondary).
    //
    spicelib::UCRSS(
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.ZVEC.as_slice_mut(),
    );
    spicelib::VHAT(save.AXIS.as_slice(), save.XVEC.as_slice_mut());
    spicelib::UCRSS(
        save.ZVEC.as_slice(),
        save.XVEC.as_slice(),
        save.YVEC.as_slice_mut(),
    );

    if (spicelib::VDOT(save.XPT1.as_slice(), save.YVEC.as_slice()) > 0 as f64) {
        //
        // XPT1 is in the half-plane defined by AXIS and PLNVEC.
        //
        spicelib::VEQU(save.XPT1.as_slice(), save.SUNLPT.as_slice_mut());
    } else {
        spicelib::VEQU(save.XPT2.as_slice(), save.SUNLPT.as_slice_mut());
    }

    //
    // Now generate the ray's direction vector.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    //
    // We can't just use VSEP to find the rotation angle of the
    // tangent ray with respect to AXIS, because the angle may
    // be larger than pi. Compute the angle from the components
    // of RAYDIR.
    //
    save.CDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.XVEC.as_slice());
    save.SDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.YVEC.as_slice());

    save.ANGLE = f64::atan2(save.SDIR, save.CDIR);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should miss the DSK surface.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near DSK umbral terminator point in cutting half-plane of previous case. Occultation case. Use surface 2. (Mars)", ctx)?;

    //
    // Generate off-DSK point RPOINT. RPOINT is 1 cm below the
    // terminator point found by ZZTANGNT.
    //
    spicelib::VLCOM(
        1.0,
        save.TANPNT.as_slice(),
        -0.00001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    //
    // Find limb on the source (sun) as seen from the tangent point.
    //
    spicelib::EDLIMB(
        save.SUNRAD[1],
        save.SUNRAD[2],
        save.SUNRAD[3],
        save.OFFSET.as_slice(),
        save.LIMB.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PSV2PL(
        save.ORIGIN.as_slice(),
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.CUT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Caution: LIMB and CUT are expressed with respect to different
    // origins. Shift LIMB so its center is an offset from the
    // target center.
    //
    spicelib::VADD(
        save.AXIS.as_slice(),
        save.LIMB.subarray(1),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VEQU(save.VTEMP.as_slice(), save.LIMB.subarray_mut(1));

    spicelib::INELPL(
        save.LIMB.as_slice(),
        save.CUT.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.CUT.as_slice(), save.NORMAL.as_slice_mut());

    //
    // The basis from the previous case still applies.
    //
    if (spicelib::VDOT(save.XPT1.as_slice(), save.YVEC.as_slice()) > 0 as f64) {
        //
        // XPT1 is in the half-plane defined by AXIS and PLNVEC.
        //
        spicelib::VEQU(save.XPT1.as_slice(), save.SUNLPT.as_slice_mut());
    } else {
        spicelib::VEQU(save.XPT2.as_slice(), save.SUNLPT.as_slice_mut());
    }

    //
    // Now generate the ray's direction vector.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    //
    // We can't just use VSEP to find the rotation angle of the
    // tangent ray with respect to AXIS, because the angle may
    // be larger than pi. Compute the angle from the components
    // of RAYDIR.
    //
    save.CDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.XVEC.as_slice());
    save.SDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.YVEC.as_slice());

    save.ANGLE = f64::atan2(save.SDIR, save.CDIR);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should hit the DSK surface.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, true, OK, ctx)?;

    //
    // The returned point should be very close to the DSK surface. Map
    // the point to a point that is on the surface and check the
    // distance between the two.
    //
    spicelib::VSCL(2.0, save.POINT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VMINUS(save.VERTEX.as_slice(), save.DIR.as_slice_mut());

    spicelib::DSKXV(
        false,
        &save.TARGET,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.ET,
        &save.FIXREF,
        1,
        save.VERTEX.as_slice(),
        save.DIR.as_slice(),
        save.NPT.as_slice_mut(),
        std::slice::from_mut(&mut save.FOUND),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"POINT",
        save.POINT.as_slice(),
        b"~~/",
        save.NPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be in the plane containing
    // the cutting half-plane.
    //
    spicelib::VPRJP(
        save.POINT.as_slice(),
        save.CUT.as_slice(),
        save.PROJ.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"PROJ",
        save.PROJ.as_slice(),
        b"~~/",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be the correct half-plane.
    //
    save.DP = spicelib::VDOT(save.POINT.as_slice(), save.YVEC.as_slice());

    save.TOL = TIGHT;

    testutil::CHCKSD(b"YVEC dot", save.DP, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // The returned point should be on the line containing the ray.
    //
    spicelib::NPLNPT(
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice(),
        save.POINT.as_slice(),
        save.PNEAR.as_slice_mut(),
        &mut save.DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = MED;
    testutil::CHCKAD(
        b"PNEAR",
        save.PNEAR.as_slice(),
        b"~~/",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be on the correct side of the vertex of
    // the line containing the ray.
    //
    spicelib::VSUB(
        save.POINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    save.D = spicelib::VDOT(save.OFFSET.as_slice(), save.RAYDIR.as_slice());

    testutil::CHCKSD(b"AXIS dot", save.D, b">", 0.0, save.TOL, OK, ctx)?;

    //
    //
    // Phobos is the target.
    //
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Initialize umbral terminator computation using DSK target shape. Use empty surface list. (Phobos)", ctx)?;

    //
    // Get solar radius.
    //
    spicelib::BODVCD(
        10,
        b"RADII",
        3,
        &mut save.N,
        save.SUNRAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CURVE = UMBRAL;
    save.SRCRAD = save.SUNRAD[1];
    save.SHAPE = DSKSHP;
    save.TRGCDE = 401;
    fstr::assign(&mut save.TARGET, b"PHOBOS");
    save.NSURF = 0;
    save.SRFLST[1] = 0;

    fstr::assign(&mut save.FIXREF, b"IAU_PHOBOS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 100000000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::BODVCD(
        401,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZTANINI(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near DSK umbral terminator point in cutting half-plane of previous case. Non-occultation case. Empty surface list. (Phobos)", ctx)?;

    //
    // Find umbral point in cutting half-plane of previous case.
    //
    spicelib::BODVCD(
        401,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.A = save.RADII[1];
    save.B = save.RADII[2];
    save.C = save.RADII[3];
    //
    // We can use ZZTANGNT to "suggest" a solution
    // point.
    //
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SCHSTP = 4.0;
    save.SOLTOL = 0.000000000000001;

    spicelib::ZZTANGNT(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        save.SCHSTP,
        save.SOLTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note that RESULT is a cell, not a window.
    //
    save.N = spicelib::CARDD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    spicelib::VEQU(save.POINTS.as_slice(), save.TANPNT.as_slice_mut());

    //
    // Get unit direction vector parallel to PLNVEC. Generate
    // off-DSK surface point RPOINT. RPOINT is 1 cm above the
    // terminator point found by ZZTANGNT.
    //
    spicelib::VLCOM(
        1.0,
        save.TANPNT.as_slice(),
        0.00001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    //
    // To generate a near-tangent ray passing through RPOINT,
    // we need to find the ray's vertex on the source. We can
    // do this by finding the limb on the source as seen from
    // RPOINT.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    spicelib::EDLIMB(
        save.SUNRAD[1],
        save.SUNRAD[2],
        save.SUNRAD[3],
        save.OFFSET.as_slice(),
        save.LIMB.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PSV2PL(
        save.ORIGIN.as_slice(),
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.CUT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Caution: LIMB and CUT are expressed with respect to different
    // origins. Shift LIMB so its center is an offset from the
    // target center.
    //
    spicelib::VADD(
        save.AXIS.as_slice(),
        save.LIMB.subarray(1),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VEQU(save.VTEMP.as_slice(), save.LIMB.subarray_mut(1));

    spicelib::INELPL(
        save.LIMB.as_slice(),
        save.CUT.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Generate basis vectors defined by AXIS (primary)
    // and PLNVEC (secondary).
    //
    spicelib::UCRSS(
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.ZVEC.as_slice_mut(),
    );
    spicelib::VHAT(save.AXIS.as_slice(), save.XVEC.as_slice_mut());
    spicelib::UCRSS(
        save.ZVEC.as_slice(),
        save.XVEC.as_slice(),
        save.YVEC.as_slice_mut(),
    );

    if (spicelib::VDOT(save.XPT1.as_slice(), save.YVEC.as_slice()) > 0 as f64) {
        //
        // XPT1 is in the half-plane defined by AXIS and PLNVEC.
        //
        spicelib::VEQU(save.XPT1.as_slice(), save.SUNLPT.as_slice_mut());
    } else {
        spicelib::VEQU(save.XPT2.as_slice(), save.SUNLPT.as_slice_mut());
    }

    //
    // Now generate the ray's direction vector.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    //
    // We can't just use VSEP to find the rotation angle of the
    // tangent ray with respect to AXIS, because the angle may
    // be larger than pi. Compute the angle from the components
    // of RAYDIR.
    //
    save.CDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.XVEC.as_slice());
    save.SDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.YVEC.as_slice());

    save.ANGLE = f64::atan2(save.SDIR, save.CDIR);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should miss the DSK surface.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near DSK umbral terminator point in cutting half-plane of previous case. Occultation case. Empty surface list. (Phobos)", ctx)?;

    //
    // Generate off-DSK point RPOINT. RPOINT is 1 cm below the
    // terminator point found by ZZTANGNT.
    //
    spicelib::VLCOM(
        1.0,
        save.TANPNT.as_slice(),
        -0.00001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    //
    // Find limb on the source (sun) as seen from the tangent point.
    //
    spicelib::EDLIMB(
        save.SUNRAD[1],
        save.SUNRAD[2],
        save.SUNRAD[3],
        save.OFFSET.as_slice(),
        save.LIMB.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PSV2PL(
        save.ORIGIN.as_slice(),
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.CUT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Caution: LIMB and CUT are expressed with respect to different
    // origins. Shift LIMB so its center is an offset from the
    // target center.
    //
    spicelib::VADD(
        save.AXIS.as_slice(),
        save.LIMB.subarray(1),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VEQU(save.VTEMP.as_slice(), save.LIMB.subarray_mut(1));

    spicelib::INELPL(
        save.LIMB.as_slice(),
        save.CUT.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.CUT.as_slice(), save.NORMAL.as_slice_mut());

    //
    // The basis from the previous case still applies.
    //
    if (spicelib::VDOT(save.XPT1.as_slice(), save.YVEC.as_slice()) > 0 as f64) {
        //
        // XPT1 is in the half-plane defined by AXIS and PLNVEC.
        //
        spicelib::VEQU(save.XPT1.as_slice(), save.SUNLPT.as_slice_mut());
    } else {
        spicelib::VEQU(save.XPT2.as_slice(), save.SUNLPT.as_slice_mut());
    }

    //
    // Now generate the ray's direction vector.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    //
    // We can't just use VSEP to find the rotation angle of the
    // tangent ray with respect to AXIS, because the angle may
    // be larger than pi. Compute the angle from the components
    // of RAYDIR.
    //
    save.CDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.XVEC.as_slice());
    save.SDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.YVEC.as_slice());

    save.ANGLE = f64::atan2(save.SDIR, save.CDIR);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should hit the DSK surface.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, true, OK, ctx)?;

    //
    // The returned point should be very close to the DSK surface. Map
    // the point to a point that is on the surface and check the
    // distance between the two.
    //
    spicelib::VSCL(2.0, save.POINT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VMINUS(save.VERTEX.as_slice(), save.DIR.as_slice_mut());

    spicelib::DSKXV(
        false,
        &save.TARGET,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.ET,
        &save.FIXREF,
        1,
        save.VERTEX.as_slice(),
        save.DIR.as_slice(),
        save.NPT.as_slice_mut(),
        std::slice::from_mut(&mut save.FOUND),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"POINT",
        save.POINT.as_slice(),
        b"~~/",
        save.NPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be in the plane containing
    // the cutting half-plane.
    //
    spicelib::VPRJP(
        save.POINT.as_slice(),
        save.CUT.as_slice(),
        save.PROJ.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = MED;

    testutil::CHCKAD(
        b"PROJ",
        save.PROJ.as_slice(),
        b"~~/",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be the correct half-plane.
    //
    save.DP = spicelib::VDOT(save.POINT.as_slice(), save.YVEC.as_slice());

    save.TOL = TIGHT;

    testutil::CHCKSD(b"YVEC dot", save.DP, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // The returned point should be on the line containing the ray.
    //
    spicelib::NPLNPT(
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice(),
        save.POINT.as_slice(),
        save.PNEAR.as_slice_mut(),
        &mut save.DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = 0.00001;
    testutil::CHCKAD(
        b"PNEAR",
        save.PNEAR.as_slice(),
        b"~~/",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be on the correct side of the vertex of
    // the line containing the ray.
    //
    spicelib::VSUB(
        save.POINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    save.D = spicelib::VDOT(save.OFFSET.as_slice(), save.RAYDIR.as_slice());

    testutil::CHCKSD(b"AXIS dot", save.D, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Initialize umbral terminator computation using DSK target shape. Specify both surfaces in surface list. (Phobos)", ctx)?;

    //
    // Use all surfaces (empty surface list).
    //

    save.CURVE = UMBRAL;
    save.SRCRAD = save.SUNRAD[1];
    save.SHAPE = DSKSHP;
    fstr::assign(&mut save.TARGET, b"PHOBOS");
    save.TRGCDE = 401;
    save.NSURF = 2;
    save.SRFLST[1] = 1;
    save.SRFLST[2] = 2;

    fstr::assign(&mut save.FIXREF, b"IAU_PHOBOS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 100000000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::ZZTANINI(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZTANGNT test: compare DSK umbral terminator point to that found in previous case. (Phobos)", ctx)?;

    //
    // Find limb point in cutting half-plane of previous case.
    //

    save.SCHSTP = 4.0;
    save.SOLTOL = 0.000000000000001;

    spicelib::ZZTANGNT(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        save.SCHSTP,
        save.SOLTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note that RESULT is a cell, not a window.
    //
    save.N = spicelib::CARDD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    spicelib::VEQU(save.POINTS.as_slice(), save.POINT.as_slice_mut());
    //
    // We expect an exact match.
    //
    save.TOL = 0.0;

    testutil::CHCKAD(
        b"POINT",
        save.POINT.as_slice(),
        b"~~/",
        save.TANPNT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Initialize umbral terminator computation using DSK target shape. Use surface 2. (Phobos)",
        ctx,
    )?;

    //
    // Use all surfaces (empty surface list).
    //

    save.CURVE = UMBRAL;
    save.SRCRAD = save.SUNRAD[1];
    save.SHAPE = DSKSHP;
    save.TRGCDE = 401;
    save.NSURF = 1;
    save.SRFLST[1] = 2;

    fstr::assign(&mut save.FIXREF, b"IAU_PHOBOS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 100000000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::ZZTANINI(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near DSK umbral terminator point in cutting half-plane of previous case. Non-occultation case. Use surface 2. (Phobos)", ctx)?;

    //
    // Find umbral point in cutting half-plane of previous case.
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

    save.A = save.RADII[1];
    save.B = save.RADII[2];
    save.C = save.RADII[3];
    //
    // We can use ZZTANGNT to "suggest" a solution
    // point.
    //
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SCHSTP = 4.0;
    save.SOLTOL = 0.000000000000001;

    spicelib::ZZTANGNT(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        save.SCHSTP,
        save.SOLTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note that RESULT is a cell, not a window.
    //
    save.N = spicelib::CARDD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    spicelib::VEQU(save.POINTS.as_slice(), save.TANPNT.as_slice_mut());

    //
    // Get unit direction vector parallel to PLNVEC. Generate
    // off-DSK surface point RPOINT. RPOINT is 1 cm above the
    // terminator point found by ZZTANGNT.
    //
    spicelib::VLCOM(
        1.0,
        save.TANPNT.as_slice(),
        0.00001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    //
    // To generate a near-tangent ray passing through RPOINT,
    // we need to find the ray's vertex on the source. We can
    // do this by finding the limb on the source as seen from
    // RPOINT.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    spicelib::EDLIMB(
        save.SUNRAD[1],
        save.SUNRAD[2],
        save.SUNRAD[3],
        save.OFFSET.as_slice(),
        save.LIMB.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PSV2PL(
        save.ORIGIN.as_slice(),
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.CUT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Caution: LIMB and CUT are expressed with respect to different
    // origins. Shift LIMB so its center is an offset from the
    // target center.
    //
    spicelib::VADD(
        save.AXIS.as_slice(),
        save.LIMB.subarray(1),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VEQU(save.VTEMP.as_slice(), save.LIMB.subarray_mut(1));

    spicelib::INELPL(
        save.LIMB.as_slice(),
        save.CUT.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Generate basis vectors defined by AXIS (primary)
    // and PLNVEC (secondary).
    //
    spicelib::UCRSS(
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.ZVEC.as_slice_mut(),
    );
    spicelib::VHAT(save.AXIS.as_slice(), save.XVEC.as_slice_mut());
    spicelib::UCRSS(
        save.ZVEC.as_slice(),
        save.XVEC.as_slice(),
        save.YVEC.as_slice_mut(),
    );

    if (spicelib::VDOT(save.XPT1.as_slice(), save.YVEC.as_slice()) > 0 as f64) {
        //
        // XPT1 is in the half-plane defined by AXIS and PLNVEC.
        //
        spicelib::VEQU(save.XPT1.as_slice(), save.SUNLPT.as_slice_mut());
    } else {
        spicelib::VEQU(save.XPT2.as_slice(), save.SUNLPT.as_slice_mut());
    }

    //
    // Now generate the ray's direction vector.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    //
    // We can't just use VSEP to find the rotation angle of the
    // tangent ray with respect to AXIS, because the angle may
    // be larger than pi. Compute the angle from the components
    // of RAYDIR.
    //
    save.CDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.XVEC.as_slice());
    save.SDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.YVEC.as_slice());

    save.ANGLE = f64::atan2(save.SDIR, save.CDIR);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should miss the DSK surface.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near DSK umbral terminator point in cutting half-plane of previous case. Occultation case. Use surface 2. (Phobos)", ctx)?;

    //
    // Generate off-DSK point RPOINT. RPOINT is 1 cm below the
    // terminator point found by ZZTANGNT.
    //
    spicelib::VLCOM(
        1.0,
        save.TANPNT.as_slice(),
        -0.00001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    //
    // Find limb on the source (sun) as seen from the tangent point.
    //
    spicelib::EDLIMB(
        save.SUNRAD[1],
        save.SUNRAD[2],
        save.SUNRAD[3],
        save.OFFSET.as_slice(),
        save.LIMB.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PSV2PL(
        save.ORIGIN.as_slice(),
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.CUT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Caution: LIMB and CUT are expressed with respect to different
    // origins. Shift LIMB so its center is an offset from the
    // target center.
    //
    spicelib::VADD(
        save.AXIS.as_slice(),
        save.LIMB.subarray(1),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VEQU(save.VTEMP.as_slice(), save.LIMB.subarray_mut(1));

    spicelib::INELPL(
        save.LIMB.as_slice(),
        save.CUT.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.CUT.as_slice(), save.NORMAL.as_slice_mut());

    //
    // The basis from the previous case still applies.
    //
    if (spicelib::VDOT(save.XPT1.as_slice(), save.YVEC.as_slice()) > 0 as f64) {
        //
        // XPT1 is in the half-plane defined by AXIS and PLNVEC.
        //
        spicelib::VEQU(save.XPT1.as_slice(), save.SUNLPT.as_slice_mut());
    } else {
        spicelib::VEQU(save.XPT2.as_slice(), save.SUNLPT.as_slice_mut());
    }

    //
    // Now generate the ray's direction vector.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    //
    // We can't just use VSEP to find the rotation angle of the
    // tangent ray with respect to AXIS, because the angle may
    // be larger than pi. Compute the angle from the components
    // of RAYDIR.
    //
    save.CDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.XVEC.as_slice());
    save.SDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.YVEC.as_slice());

    save.ANGLE = f64::atan2(save.SDIR, save.CDIR);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should hit the DSK surface.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, true, OK, ctx)?;

    //
    // The returned point should be very close to the DSK surface. Map
    // the point to a point that is on the surface and check the
    // distance between the two.
    //
    spicelib::VSCL(2.0, save.POINT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VMINUS(save.VERTEX.as_slice(), save.DIR.as_slice_mut());

    spicelib::DSKXV(
        false,
        &save.TARGET,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.ET,
        &save.FIXREF,
        1,
        save.VERTEX.as_slice(),
        save.DIR.as_slice(),
        save.NPT.as_slice_mut(),
        std::slice::from_mut(&mut save.FOUND),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"POINT",
        save.POINT.as_slice(),
        b"~~/",
        save.NPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be in the plane containing
    // the cutting half-plane.
    //
    spicelib::VPRJP(
        save.POINT.as_slice(),
        save.CUT.as_slice(),
        save.PROJ.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = 0.00001;

    testutil::CHCKAD(
        b"PROJ",
        save.PROJ.as_slice(),
        b"~~/",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be the correct half-plane.
    //
    save.DP = spicelib::VDOT(save.POINT.as_slice(), save.YVEC.as_slice());

    save.TOL = TIGHT;

    testutil::CHCKSD(b"YVEC dot", save.DP, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // The returned point should be on the line containing the ray.
    //
    spicelib::NPLNPT(
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice(),
        save.POINT.as_slice(),
        save.PNEAR.as_slice_mut(),
        &mut save.DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = 0.00001;
    testutil::CHCKAD(
        b"PNEAR",
        save.PNEAR.as_slice(),
        b"~~/",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be on the correct side of the vertex of
    // the line containing the ray.
    //
    spicelib::VSUB(
        save.POINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    save.D = spicelib::VDOT(save.OFFSET.as_slice(), save.RAYDIR.as_slice());

    testutil::CHCKSD(b"AXIS dot", save.D, b">", 0.0, save.TOL, OK, ctx)?;

    //
    //
    // DSK PENUMBRAL TERMINATOR cases
    //

    //
    //
    // Mars is the target.
    //
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Initialize penumbral terminator computation using DSK target shape. Use empty surface list. (Mars)", ctx)?;

    //
    // Get solar radius.
    //
    spicelib::BODVCD(
        10,
        b"RADII",
        3,
        &mut save.N,
        save.SUNRAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CURVE = PNMBRL;
    save.SRCRAD = save.SUNRAD[1];
    save.SHAPE = DSKSHP;
    save.TRGCDE = 499;
    fstr::assign(&mut save.TARGET, b"MARS");
    save.NSURF = 0;
    save.SRFLST[1] = 0;

    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 100000000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::BODVCD(
        499,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZTANINI(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near DSK penumbral terminator point in cutting half-plane of previous case. Non-occultation case. Empty surface list. (Mars)", ctx)?;

    //
    // Find umbral point in cutting half-plane of previous case.
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

    save.A = save.RADII[1];
    save.B = save.RADII[2];
    save.C = save.RADII[3];
    //
    // We can use ZZTANGNT to "suggest" a solution
    // point.
    //
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SCHSTP = 4.0;
    save.SOLTOL = 0.000000000000001;

    spicelib::ZZTANGNT(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        save.SCHSTP,
        save.SOLTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note that RESULT is a cell, not a window.
    //
    save.N = spicelib::CARDD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    spicelib::VEQU(save.POINTS.as_slice(), save.TANPNT.as_slice_mut());

    //
    // Get unit direction vector parallel to PLNVEC. Generate
    // off-DSK surface point RPOINT. RPOINT is 1 cm above the
    // terminator point found by ZZTANGNT.
    //
    spicelib::VLCOM(
        1.0,
        save.TANPNT.as_slice(),
        0.00001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    //
    // To generate a near-tangent ray passing through RPOINT,
    // we need to find the ray's vertex on the source. We can
    // do this by finding the limb on the source as seen from
    // RPOINT.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    spicelib::EDLIMB(
        save.SUNRAD[1],
        save.SUNRAD[2],
        save.SUNRAD[3],
        save.OFFSET.as_slice(),
        save.LIMB.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PSV2PL(
        save.ORIGIN.as_slice(),
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.CUT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Caution: LIMB and CUT are expressed with respect to different
    // origins. Shift LIMB so its center is an offset from the
    // target center.
    //
    spicelib::VADD(
        save.AXIS.as_slice(),
        save.LIMB.subarray(1),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VEQU(save.VTEMP.as_slice(), save.LIMB.subarray_mut(1));

    spicelib::INELPL(
        save.LIMB.as_slice(),
        save.CUT.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Generate basis vectors defined by AXIS (primary)
    // and PLNVEC (secondary).
    //
    spicelib::UCRSS(
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.ZVEC.as_slice_mut(),
    );
    spicelib::VHAT(save.AXIS.as_slice(), save.XVEC.as_slice_mut());
    spicelib::UCRSS(
        save.ZVEC.as_slice(),
        save.XVEC.as_slice(),
        save.YVEC.as_slice_mut(),
    );

    if (spicelib::VDOT(save.XPT1.as_slice(), save.YVEC.as_slice()) < 0 as f64) {
        //
        // XPT1 is on the opposite side of the half-plane defined by AXIS
        // and PLNVEC.
        //
        spicelib::VEQU(save.XPT1.as_slice(), save.SUNLPT.as_slice_mut());
    } else {
        spicelib::VEQU(save.XPT2.as_slice(), save.SUNLPT.as_slice_mut());
    }

    //
    // Now generate the ray's direction vector.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    //
    // We can't just use VSEP to find the rotation angle of the
    // tangent ray with respect to AXIS, because the angle may
    // be larger than pi. Compute the angle from the components
    // of RAYDIR.
    //
    save.CDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.XVEC.as_slice());
    save.SDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.YVEC.as_slice());

    save.ANGLE = f64::atan2(save.SDIR, save.CDIR);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should miss the DSK surface.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near DSK penumbral terminator point in cutting half-plane of previous case. Occultation case. Empty surface list. (Mars)", ctx)?;

    //
    // Generate off-DSK point RPOINT. RPOINT is 1 cm below the
    // terminator point found by ZZTANGNT.
    //
    spicelib::VLCOM(
        1.0,
        save.TANPNT.as_slice(),
        -0.00001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    //
    // Find limb on the source (sun) as seen from the tangent point.
    //
    spicelib::EDLIMB(
        save.SUNRAD[1],
        save.SUNRAD[2],
        save.SUNRAD[3],
        save.OFFSET.as_slice(),
        save.LIMB.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PSV2PL(
        save.ORIGIN.as_slice(),
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.CUT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Caution: LIMB and CUT are expressed with respect to different
    // origins. Shift LIMB so its center is an offset from the
    // target center.
    //
    spicelib::VADD(
        save.AXIS.as_slice(),
        save.LIMB.subarray(1),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VEQU(save.VTEMP.as_slice(), save.LIMB.subarray_mut(1));

    spicelib::INELPL(
        save.LIMB.as_slice(),
        save.CUT.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.CUT.as_slice(), save.NORMAL.as_slice_mut());

    //
    // The basis from the previous case still applies.
    //
    if (spicelib::VDOT(save.XPT1.as_slice(), save.YVEC.as_slice()) < 0 as f64) {
        //
        // XPT1 is on the opposite side of the half-plane defined by AXIS
        // and PLNVEC.
        //
        spicelib::VEQU(save.XPT1.as_slice(), save.SUNLPT.as_slice_mut());
    } else {
        spicelib::VEQU(save.XPT2.as_slice(), save.SUNLPT.as_slice_mut());
    }

    //
    // Now generate the ray's direction vector.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    //
    // We can't just use VSEP to find the rotation angle of the
    // tangent ray with respect to AXIS, because the angle may
    // be larger than pi. Compute the angle from the components
    // of RAYDIR.
    //
    save.CDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.XVEC.as_slice());
    save.SDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.YVEC.as_slice());

    save.ANGLE = f64::atan2(save.SDIR, save.CDIR);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should hit the DSK surface.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, true, OK, ctx)?;

    //
    // The returned point should be very close to the DSK surface. Map
    // the point to a point that is on the surface and check the
    // distance between the two.
    //
    spicelib::VSCL(2.0, save.POINT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VMINUS(save.VERTEX.as_slice(), save.DIR.as_slice_mut());

    spicelib::DSKXV(
        false,
        &save.TARGET,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.ET,
        &save.FIXREF,
        1,
        save.VERTEX.as_slice(),
        save.DIR.as_slice(),
        save.NPT.as_slice_mut(),
        std::slice::from_mut(&mut save.FOUND),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"POINT",
        save.POINT.as_slice(),
        b"~~/",
        save.NPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be in the plane containing
    // the cutting half-plane.
    //
    spicelib::VPRJP(
        save.POINT.as_slice(),
        save.CUT.as_slice(),
        save.PROJ.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = MED;

    testutil::CHCKAD(
        b"PROJ",
        save.PROJ.as_slice(),
        b"~~/",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be the correct half-plane.
    //
    save.DP = spicelib::VDOT(save.POINT.as_slice(), save.YVEC.as_slice());

    save.TOL = TIGHT;

    testutil::CHCKSD(b"YVEC dot", save.DP, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // The returned point should be on the line containing the ray.
    //
    spicelib::NPLNPT(
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice(),
        save.POINT.as_slice(),
        save.PNEAR.as_slice_mut(),
        &mut save.DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = MED;
    testutil::CHCKAD(
        b"PNEAR",
        save.PNEAR.as_slice(),
        b"~~/",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be on the correct side of the vertex of
    // the line containing the ray.
    //
    spicelib::VSUB(
        save.POINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    save.D = spicelib::VDOT(save.OFFSET.as_slice(), save.RAYDIR.as_slice());

    testutil::CHCKSD(b"AXIS dot", save.D, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Initialize penumbral terminator computation using DSK target shape. Specify both surfaces in surface list. (Mars)", ctx)?;

    //
    // Use all surfaces (empty surface list).
    //

    save.CURVE = PNMBRL;
    save.SRCRAD = save.SUNRAD[1];
    save.SHAPE = DSKSHP;
    fstr::assign(&mut save.TARGET, b"MARS");
    save.TRGCDE = 499;
    save.NSURF = 2;
    save.SRFLST[1] = 1;
    save.SRFLST[2] = 2;

    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 100000000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::ZZTANINI(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZTANGNT test: compare DSK penumbral terminator point to that found in previous case. (Mars)", ctx)?;

    //
    // Find limb point in cutting half-plane of previous case.
    //

    save.SCHSTP = 4.0;
    save.SOLTOL = 0.000000000000001;

    spicelib::ZZTANGNT(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        save.SCHSTP,
        save.SOLTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note that RESULT is a cell, not a window.
    //
    save.N = spicelib::CARDD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    spicelib::VEQU(save.POINTS.as_slice(), save.POINT.as_slice_mut());
    //
    // We expect an exact match.
    //
    save.TOL = 0.0;

    testutil::CHCKAD(
        b"POINT",
        save.POINT.as_slice(),
        b"~~/",
        save.TANPNT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Initialize penumbral terminator computation using DSK target shape. Use surface 2. (Mars)", ctx)?;

    //
    // Use all surfaces (empty surface list).
    //

    save.CURVE = PNMBRL;
    save.SRCRAD = save.SUNRAD[1];
    save.SHAPE = DSKSHP;
    save.TRGCDE = 499;
    save.NSURF = 1;
    save.SRFLST[1] = 2;

    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 100000000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::ZZTANINI(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near DSK penumbral terminator point in cutting half-plane of previous case. Non-occultation case. Use surface 2. (Mars)", ctx)?;

    //
    // Find umbral point in cutting half-plane of previous case.
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

    save.A = save.RADII[1];
    save.B = save.RADII[2];
    save.C = save.RADII[3];
    //
    // We can use ZZTANGNT to "suggest" a solution
    // point.
    //
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SCHSTP = 4.0;
    save.SOLTOL = 0.000000000000001;

    spicelib::ZZTANGNT(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        save.SCHSTP,
        save.SOLTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note that RESULT is a cell, not a window.
    //
    save.N = spicelib::CARDD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    spicelib::VEQU(save.POINTS.as_slice(), save.TANPNT.as_slice_mut());

    //
    // Get unit direction vector parallel to PLNVEC. Generate
    // off-DSK surface point RPOINT. RPOINT is 1 cm above the
    // terminator point found by ZZTANGNT.
    //
    spicelib::VLCOM(
        1.0,
        save.TANPNT.as_slice(),
        0.00001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    //
    // To generate a near-tangent ray passing through RPOINT,
    // we need to find the ray's vertex on the source. We can
    // do this by finding the limb on the source as seen from
    // RPOINT.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    spicelib::EDLIMB(
        save.SUNRAD[1],
        save.SUNRAD[2],
        save.SUNRAD[3],
        save.OFFSET.as_slice(),
        save.LIMB.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PSV2PL(
        save.ORIGIN.as_slice(),
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.CUT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Caution: LIMB and CUT are expressed with respect to different
    // origins. Shift LIMB so its center is an offset from the
    // target center.
    //
    spicelib::VADD(
        save.AXIS.as_slice(),
        save.LIMB.subarray(1),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VEQU(save.VTEMP.as_slice(), save.LIMB.subarray_mut(1));

    spicelib::INELPL(
        save.LIMB.as_slice(),
        save.CUT.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Generate basis vectors defined by AXIS (primary)
    // and PLNVEC (secondary).
    //
    spicelib::UCRSS(
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.ZVEC.as_slice_mut(),
    );
    spicelib::VHAT(save.AXIS.as_slice(), save.XVEC.as_slice_mut());
    spicelib::UCRSS(
        save.ZVEC.as_slice(),
        save.XVEC.as_slice(),
        save.YVEC.as_slice_mut(),
    );

    if (spicelib::VDOT(save.XPT1.as_slice(), save.YVEC.as_slice()) < 0 as f64) {
        //
        // XPT1 is on the opposite side of the half-plane defined by AXIS
        // and PLNVEC.
        //
        spicelib::VEQU(save.XPT1.as_slice(), save.SUNLPT.as_slice_mut());
    } else {
        spicelib::VEQU(save.XPT2.as_slice(), save.SUNLPT.as_slice_mut());
    }

    //
    // Now generate the ray's direction vector.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    //
    // We can't just use VSEP to find the rotation angle of the
    // tangent ray with respect to AXIS, because the angle may
    // be larger than pi. Compute the angle from the components
    // of RAYDIR.
    //
    save.CDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.XVEC.as_slice());
    save.SDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.YVEC.as_slice());

    save.ANGLE = f64::atan2(save.SDIR, save.CDIR);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should miss the DSK surface.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near DSK penumbral terminator point in cutting half-plane of previous case. Occultation case. Use surface 2. (Mars)", ctx)?;

    //
    // Generate off-DSK point RPOINT. RPOINT is 1 cm below the
    // terminator point found by ZZTANGNT.
    //
    spicelib::VLCOM(
        1.0,
        save.TANPNT.as_slice(),
        -0.00001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    //
    // Find limb on the source (sun) as seen from the tangent point.
    //
    spicelib::EDLIMB(
        save.SUNRAD[1],
        save.SUNRAD[2],
        save.SUNRAD[3],
        save.OFFSET.as_slice(),
        save.LIMB.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PSV2PL(
        save.ORIGIN.as_slice(),
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.CUT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Caution: LIMB and CUT are expressed with respect to different
    // origins. Shift LIMB so its center is an offset from the
    // target center.
    //
    spicelib::VADD(
        save.AXIS.as_slice(),
        save.LIMB.subarray(1),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VEQU(save.VTEMP.as_slice(), save.LIMB.subarray_mut(1));

    spicelib::INELPL(
        save.LIMB.as_slice(),
        save.CUT.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.CUT.as_slice(), save.NORMAL.as_slice_mut());

    //
    // The basis from the previous case still applies.
    //
    if (spicelib::VDOT(save.XPT1.as_slice(), save.YVEC.as_slice()) < 0 as f64) {
        //
        // XPT1 is on the opposite side of the half-plane defined by AXIS
        // and PLNVEC.
        //
        spicelib::VEQU(save.XPT1.as_slice(), save.SUNLPT.as_slice_mut());
    } else {
        spicelib::VEQU(save.XPT2.as_slice(), save.SUNLPT.as_slice_mut());
    }

    //
    // Now generate the ray's direction vector.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    //
    // We can't just use VSEP to find the rotation angle of the
    // tangent ray with respect to AXIS, because the angle may
    // be larger than pi. Compute the angle from the components
    // of RAYDIR.
    //
    save.CDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.XVEC.as_slice());
    save.SDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.YVEC.as_slice());

    save.ANGLE = f64::atan2(save.SDIR, save.CDIR);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should hit the DSK surface.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, true, OK, ctx)?;

    //
    // The returned point should be very close to the DSK surface. Map
    // the point to a point that is on the surface and check the
    // distance between the two.
    //
    spicelib::VSCL(2.0, save.POINT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VMINUS(save.VERTEX.as_slice(), save.DIR.as_slice_mut());

    spicelib::DSKXV(
        false,
        &save.TARGET,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.ET,
        &save.FIXREF,
        1,
        save.VERTEX.as_slice(),
        save.DIR.as_slice(),
        save.NPT.as_slice_mut(),
        std::slice::from_mut(&mut save.FOUND),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"POINT",
        save.POINT.as_slice(),
        b"~~/",
        save.NPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be in the plane containing
    // the cutting half-plane.
    //
    spicelib::VPRJP(
        save.POINT.as_slice(),
        save.CUT.as_slice(),
        save.PROJ.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = MED;

    testutil::CHCKAD(
        b"PROJ",
        save.PROJ.as_slice(),
        b"~~/",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be the correct half-plane.
    //
    save.DP = spicelib::VDOT(save.POINT.as_slice(), save.YVEC.as_slice());

    save.TOL = TIGHT;

    testutil::CHCKSD(b"YVEC dot", save.DP, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // The returned point should be on the line containing the ray.
    //
    spicelib::NPLNPT(
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice(),
        save.POINT.as_slice(),
        save.PNEAR.as_slice_mut(),
        &mut save.DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = MED;
    testutil::CHCKAD(
        b"PNEAR",
        save.PNEAR.as_slice(),
        b"~~/",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be on the correct side of the vertex of
    // the line containing the ray.
    //
    spicelib::VSUB(
        save.POINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    save.D = spicelib::VDOT(save.OFFSET.as_slice(), save.RAYDIR.as_slice());

    testutil::CHCKSD(b"AXIS dot", save.D, b">", 0.0, save.TOL, OK, ctx)?;

    //
    //
    // Phobos is the target.
    //
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Initialize penumbral terminator computation using DSK target shape. Use empty surface list. (Phobos)", ctx)?;

    //
    // Get solar radius.
    //
    spicelib::BODVCD(
        10,
        b"RADII",
        3,
        &mut save.N,
        save.SUNRAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CURVE = PNMBRL;
    save.SRCRAD = save.SUNRAD[1];
    save.SHAPE = DSKSHP;
    save.TRGCDE = 401;
    fstr::assign(&mut save.TARGET, b"PHOBOS");
    save.NSURF = 0;
    save.SRFLST[1] = 0;

    fstr::assign(&mut save.FIXREF, b"IAU_PHOBOS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 100000000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::BODVCD(
        401,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZTANINI(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near DSK penumbral terminator point in cutting half-plane of previous case. Non-occultation case. Empty surface list. (Phobos)", ctx)?;

    //
    // Find umbral point in cutting half-plane of previous case.
    //
    spicelib::BODVCD(
        401,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.A = save.RADII[1];
    save.B = save.RADII[2];
    save.C = save.RADII[3];
    //
    // We can use ZZTANGNT to "suggest" a solution
    // point.
    //
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SCHSTP = 4.0;
    save.SOLTOL = 0.000000000000001;

    spicelib::ZZTANGNT(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        save.SCHSTP,
        save.SOLTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note that RESULT is a cell, not a window.
    //
    save.N = spicelib::CARDD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    spicelib::VEQU(save.POINTS.as_slice(), save.TANPNT.as_slice_mut());

    //
    // Get unit direction vector parallel to PLNVEC. Generate
    // off-DSK surface point RPOINT. RPOINT is 1 cm above the
    // terminator point found by ZZTANGNT.
    //
    spicelib::VLCOM(
        1.0,
        save.TANPNT.as_slice(),
        0.00001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    //
    // To generate a near-tangent ray passing through RPOINT,
    // we need to find the ray's vertex on the source. We can
    // do this by finding the limb on the source as seen from
    // RPOINT.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    spicelib::EDLIMB(
        save.SUNRAD[1],
        save.SUNRAD[2],
        save.SUNRAD[3],
        save.OFFSET.as_slice(),
        save.LIMB.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PSV2PL(
        save.ORIGIN.as_slice(),
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.CUT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Caution: LIMB and CUT are expressed with respect to different
    // origins. Shift LIMB so its center is an offset from the
    // target center.
    //
    spicelib::VADD(
        save.AXIS.as_slice(),
        save.LIMB.subarray(1),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VEQU(save.VTEMP.as_slice(), save.LIMB.subarray_mut(1));

    spicelib::INELPL(
        save.LIMB.as_slice(),
        save.CUT.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Generate basis vectors defined by AXIS (primary)
    // and PLNVEC (secondary).
    //
    spicelib::UCRSS(
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.ZVEC.as_slice_mut(),
    );
    spicelib::VHAT(save.AXIS.as_slice(), save.XVEC.as_slice_mut());
    spicelib::UCRSS(
        save.ZVEC.as_slice(),
        save.XVEC.as_slice(),
        save.YVEC.as_slice_mut(),
    );

    if (spicelib::VDOT(save.XPT1.as_slice(), save.YVEC.as_slice()) < 0 as f64) {
        //
        // XPT1 is on the opposite side of the half-plane defined by AXIS
        // and PLNVEC.
        //
        spicelib::VEQU(save.XPT1.as_slice(), save.SUNLPT.as_slice_mut());
    } else {
        spicelib::VEQU(save.XPT2.as_slice(), save.SUNLPT.as_slice_mut());
    }

    //
    // Now generate the ray's direction vector.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    //
    // We can't just use VSEP to find the rotation angle of the
    // tangent ray with respect to AXIS, because the angle may
    // be larger than pi. Compute the angle from the components
    // of RAYDIR.
    //
    save.CDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.XVEC.as_slice());
    save.SDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.YVEC.as_slice());

    save.ANGLE = f64::atan2(save.SDIR, save.CDIR);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should miss the DSK surface.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near DSK penumbral terminator point in cutting half-plane of previous case. Occultation case. Empty surface list. (Phobos)", ctx)?;

    //
    // Generate off-DSK point RPOINT. RPOINT is 1 cm below the
    // terminator point found by ZZTANGNT.
    //
    spicelib::VLCOM(
        1.0,
        save.TANPNT.as_slice(),
        -0.00001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    //
    // Find limb on the source (sun) as seen from the tangent point.
    //
    spicelib::EDLIMB(
        save.SUNRAD[1],
        save.SUNRAD[2],
        save.SUNRAD[3],
        save.OFFSET.as_slice(),
        save.LIMB.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PSV2PL(
        save.ORIGIN.as_slice(),
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.CUT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Caution: LIMB and CUT are expressed with respect to different
    // origins. Shift LIMB so its center is an offset from the
    // target center.
    //
    spicelib::VADD(
        save.AXIS.as_slice(),
        save.LIMB.subarray(1),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VEQU(save.VTEMP.as_slice(), save.LIMB.subarray_mut(1));

    spicelib::INELPL(
        save.LIMB.as_slice(),
        save.CUT.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.CUT.as_slice(), save.NORMAL.as_slice_mut());

    //
    // The basis from the previous case still applies.
    //
    if (spicelib::VDOT(save.XPT1.as_slice(), save.YVEC.as_slice()) < 0 as f64) {
        //
        // XPT1 is on the opposite side of the half-plane defined by AXIS
        // and PLNVEC.
        //
        spicelib::VEQU(save.XPT1.as_slice(), save.SUNLPT.as_slice_mut());
    } else {
        spicelib::VEQU(save.XPT2.as_slice(), save.SUNLPT.as_slice_mut());
    }

    //
    // Now generate the ray's direction vector.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    //
    // We can't just use VSEP to find the rotation angle of the
    // tangent ray with respect to AXIS, because the angle may
    // be larger than pi. Compute the angle from the components
    // of RAYDIR.
    //
    save.CDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.XVEC.as_slice());
    save.SDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.YVEC.as_slice());

    save.ANGLE = f64::atan2(save.SDIR, save.CDIR);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should hit the DSK surface.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, true, OK, ctx)?;

    //
    // The returned point should be very close to the DSK surface. Map
    // the point to a point that is on the surface and check the
    // distance between the two.
    //
    spicelib::VSCL(2.0, save.POINT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VMINUS(save.VERTEX.as_slice(), save.DIR.as_slice_mut());

    spicelib::DSKXV(
        false,
        &save.TARGET,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.ET,
        &save.FIXREF,
        1,
        save.VERTEX.as_slice(),
        save.DIR.as_slice(),
        save.NPT.as_slice_mut(),
        std::slice::from_mut(&mut save.FOUND),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"POINT",
        save.POINT.as_slice(),
        b"~~/",
        save.NPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be in the plane containing
    // the cutting half-plane.
    //
    spicelib::VPRJP(
        save.POINT.as_slice(),
        save.CUT.as_slice(),
        save.PROJ.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = 0.00001;

    testutil::CHCKAD(
        b"PROJ",
        save.PROJ.as_slice(),
        b"~~/",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be the correct half-plane.
    //
    save.DP = spicelib::VDOT(save.POINT.as_slice(), save.YVEC.as_slice());

    save.TOL = TIGHT;

    testutil::CHCKSD(b"YVEC dot", save.DP, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // The returned point should be on the line containing the ray.
    //
    spicelib::NPLNPT(
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice(),
        save.POINT.as_slice(),
        save.PNEAR.as_slice_mut(),
        &mut save.DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = 0.00001;
    testutil::CHCKAD(
        b"PNEAR",
        save.PNEAR.as_slice(),
        b"~~/",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be on the correct side of the vertex of
    // the line containing the ray.
    //
    spicelib::VSUB(
        save.POINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    save.D = spicelib::VDOT(save.OFFSET.as_slice(), save.RAYDIR.as_slice());

    testutil::CHCKSD(b"AXIS dot", save.D, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Initialize penumbral terminator computation using DSK target shape. Specify both surfaces in surface list. (Phobos)", ctx)?;

    //
    // Use all surfaces (empty surface list).
    //

    save.CURVE = PNMBRL;
    save.SRCRAD = save.SUNRAD[1];
    save.SHAPE = DSKSHP;
    fstr::assign(&mut save.TARGET, b"PHOBOS");
    save.TRGCDE = 401;
    save.NSURF = 2;
    save.SRFLST[1] = 1;
    save.SRFLST[2] = 2;

    fstr::assign(&mut save.FIXREF, b"IAU_PHOBOS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 100000000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::ZZTANINI(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZTANGNT test: compare DSK penumbral terminator point to that found in previous case. (Phobos)", ctx)?;

    //
    // Find limb point in cutting half-plane of previous case.
    //

    save.SCHSTP = 4.0;
    save.SOLTOL = 0.000000000000001;

    spicelib::ZZTANGNT(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        save.SCHSTP,
        save.SOLTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note that RESULT is a cell, not a window.
    //
    save.N = spicelib::CARDD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    spicelib::VEQU(save.POINTS.as_slice(), save.POINT.as_slice_mut());
    //
    // We expect an exact match.
    //
    save.TOL = 0.0;

    testutil::CHCKAD(
        b"POINT",
        save.POINT.as_slice(),
        b"~~/",
        save.TANPNT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Initialize penumbral terminator computation using DSK target shape. Use surface 2. (Phobos)", ctx)?;

    //
    // Use all surfaces (empty surface list).
    //

    save.CURVE = PNMBRL;
    save.SRCRAD = save.SUNRAD[1];
    save.SHAPE = DSKSHP;
    save.TRGCDE = 401;
    save.NSURF = 1;
    save.SRFLST[1] = 2;

    fstr::assign(&mut save.FIXREF, b"IAU_PHOBOS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    save.D = 100000000.0;

    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.AXIS.as_slice_mut(),
    );
    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::ZZTANINI(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near DSK penumbral terminator point in cutting half-plane of previous case. Non-occultation case. Use surface 2. (Phobos)", ctx)?;

    //
    // Find umbral point in cutting half-plane of previous case.
    //
    spicelib::BODVCD(
        401,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.A = save.RADII[1];
    save.B = save.RADII[2];
    save.C = save.RADII[3];
    //
    // We can use ZZTANGNT to "suggest" a solution
    // point.
    //
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SCHSTP = 4.0;
    save.SOLTOL = 0.000000000000001;

    spicelib::ZZTANGNT(
        save.CURVE,
        save.SRCRAD,
        save.SHAPE,
        save.TRGCDE,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        save.ET,
        save.PLNVEC.as_slice(),
        save.AXIS.as_slice(),
        save.SCHSTP,
        save.SOLTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note that RESULT is a cell, not a window.
    //
    save.N = spicelib::CARDD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

    spicelib::VEQU(save.POINTS.as_slice(), save.TANPNT.as_slice_mut());

    //
    // Get unit direction vector parallel to PLNVEC. Generate
    // off-DSK surface point RPOINT. RPOINT is 1 cm above the
    // terminator point found by ZZTANGNT.
    //
    spicelib::VLCOM(
        1.0,
        save.TANPNT.as_slice(),
        0.00001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    //
    // To generate a near-tangent ray passing through RPOINT,
    // we need to find the ray's vertex on the source. We can
    // do this by finding the limb on the source as seen from
    // RPOINT.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    spicelib::EDLIMB(
        save.SUNRAD[1],
        save.SUNRAD[2],
        save.SUNRAD[3],
        save.OFFSET.as_slice(),
        save.LIMB.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PSV2PL(
        save.ORIGIN.as_slice(),
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.CUT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Caution: LIMB and CUT are expressed with respect to different
    // origins. Shift LIMB so its center is an offset from the
    // target center.
    //
    spicelib::VADD(
        save.AXIS.as_slice(),
        save.LIMB.subarray(1),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VEQU(save.VTEMP.as_slice(), save.LIMB.subarray_mut(1));

    spicelib::INELPL(
        save.LIMB.as_slice(),
        save.CUT.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Generate basis vectors defined by AXIS (primary)
    // and PLNVEC (secondary).
    //
    spicelib::UCRSS(
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.ZVEC.as_slice_mut(),
    );
    spicelib::VHAT(save.AXIS.as_slice(), save.XVEC.as_slice_mut());
    spicelib::UCRSS(
        save.ZVEC.as_slice(),
        save.XVEC.as_slice(),
        save.YVEC.as_slice_mut(),
    );

    if (spicelib::VDOT(save.XPT1.as_slice(), save.YVEC.as_slice()) < 0 as f64) {
        //
        // XPT1 is on the opposite side of the half-plane defined by AXIS
        // and PLNVEC.
        //
        spicelib::VEQU(save.XPT1.as_slice(), save.SUNLPT.as_slice_mut());
    } else {
        spicelib::VEQU(save.XPT2.as_slice(), save.SUNLPT.as_slice_mut());
    }

    //
    // Now generate the ray's direction vector.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    //
    // We can't just use VSEP to find the rotation angle of the
    // tangent ray with respect to AXIS, because the angle may
    // be larger than pi. Compute the angle from the components
    // of RAYDIR.
    //
    save.CDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.XVEC.as_slice());
    save.SDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.YVEC.as_slice());

    save.ANGLE = f64::atan2(save.SDIR, save.CDIR);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should miss the DSK surface.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check state of ray near DSK penumbral terminator point in cutting half-plane of previous case. Occultation case. Use surface 2. (Phobos)", ctx)?;

    //
    // Generate off-DSK point RPOINT. RPOINT is 1 cm below the
    // terminator point found by ZZTANGNT.
    //
    spicelib::VLCOM(
        1.0,
        save.TANPNT.as_slice(),
        -0.00001,
        save.YVEC.as_slice(),
        save.RPOINT.as_slice_mut(),
    );

    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.AXIS.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    //
    // Find limb on the source (sun) as seen from the tangent point.
    //
    spicelib::EDLIMB(
        save.SUNRAD[1],
        save.SUNRAD[2],
        save.SUNRAD[3],
        save.OFFSET.as_slice(),
        save.LIMB.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PSV2PL(
        save.ORIGIN.as_slice(),
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.CUT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Caution: LIMB and CUT are expressed with respect to different
    // origins. Shift LIMB so its center is an offset from the
    // target center.
    //
    spicelib::VADD(
        save.AXIS.as_slice(),
        save.LIMB.subarray(1),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VEQU(save.VTEMP.as_slice(), save.LIMB.subarray_mut(1));

    spicelib::INELPL(
        save.LIMB.as_slice(),
        save.CUT.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.CUT.as_slice(), save.NORMAL.as_slice_mut());

    //
    // The basis from the previous case still applies.
    //
    if (spicelib::VDOT(save.XPT1.as_slice(), save.YVEC.as_slice()) < 0 as f64) {
        //
        // XPT1 is on the opposite side of the half-plane defined by AXIS
        // and PLNVEC.
        //
        spicelib::VEQU(save.XPT1.as_slice(), save.SUNLPT.as_slice_mut());
    } else {
        spicelib::VEQU(save.XPT2.as_slice(), save.SUNLPT.as_slice_mut());
    }

    //
    // Now generate the ray's direction vector.
    //
    spicelib::VSUB(
        save.RPOINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice_mut(),
    );

    //
    // We can't just use VSEP to find the rotation angle of the
    // tangent ray with respect to AXIS, because the angle may
    // be larger than pi. Compute the angle from the components
    // of RAYDIR.
    //
    save.CDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.XVEC.as_slice());
    save.SDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.YVEC.as_slice());

    save.ANGLE = f64::atan2(save.SDIR, save.CDIR);

    //
    // Find intersection state, given the angle between AXIS and RAYDIR.
    //
    spicelib::ZZTANSTA(save.ANGLE, &mut save.OCULTD, save.POINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray should hit the DSK surface.
    //
    testutil::CHCKSL(b"OCULTD", save.OCULTD, true, OK, ctx)?;

    //
    // The returned point should be very close to the DSK surface. Map
    // the point to a point that is on the surface and check the
    // distance between the two.
    //
    spicelib::VSCL(2.0, save.POINT.as_slice(), save.VERTEX.as_slice_mut());
    spicelib::VMINUS(save.VERTEX.as_slice(), save.DIR.as_slice_mut());

    spicelib::DSKXV(
        false,
        &save.TARGET,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.ET,
        &save.FIXREF,
        1,
        save.VERTEX.as_slice(),
        save.DIR.as_slice(),
        save.NPT.as_slice_mut(),
        std::slice::from_mut(&mut save.FOUND),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"POINT",
        save.POINT.as_slice(),
        b"~~/",
        save.NPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be in the plane containing
    // the cutting half-plane.
    //
    spicelib::VPRJP(
        save.POINT.as_slice(),
        save.CUT.as_slice(),
        save.PROJ.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = 0.00001;

    testutil::CHCKAD(
        b"PROJ",
        save.PROJ.as_slice(),
        b"~~/",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be the correct half-plane.
    //
    save.DP = spicelib::VDOT(save.POINT.as_slice(), save.YVEC.as_slice());

    save.TOL = TIGHT;

    testutil::CHCKSD(b"YVEC dot", save.DP, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // The returned point should be on the line containing the ray.
    //
    spicelib::NPLNPT(
        save.SUNLPT.as_slice(),
        save.RAYDIR.as_slice(),
        save.POINT.as_slice(),
        save.PNEAR.as_slice_mut(),
        &mut save.DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = 0.00001;
    testutil::CHCKAD(
        b"PNEAR",
        save.PNEAR.as_slice(),
        b"~~/",
        save.POINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The returned point should be on the correct side of the vertex of
    // the line containing the ray.
    //
    spicelib::VSUB(
        save.POINT.as_slice(),
        save.SUNLPT.as_slice(),
        save.OFFSET.as_slice_mut(),
    );

    save.D = spicelib::VDOT(save.OFFSET.as_slice(), save.RAYDIR.as_slice());

    testutil::CHCKSD(b"AXIS dot", save.D, b">", 0.0, save.TOL, OK, ctx)?;

    //***********************************************************************
    //
    //     TORUS tests
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Torus test setup", ctx)?;

    //
    // Create and load Nat's SPK and PCK.
    //
    if spicelib::EXISTS(SPK1, ctx)? {
        spicelib::DELFIL(SPK1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    testutil::NATSPK(SPK1, true, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if spicelib::EXISTS(PCK1, ctx)? {
        spicelib::DELFIL(PCK1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    testutil::NATPCK(PCK1, true, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create DSK containing three nested tori. The object is
    // centered at body Alpha.
    //
    if spicelib::EXISTS(DSK4, ctx)? {
        spicelib::DELFIL(DSK4, ctx)?;
    }

    fstr::assign(&mut save.FIXREF, b"ALPHA_VIEW_XY");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"FIXFID", save.FIXFID, b">", 0, 0, OK, ctx)?;

    save.R = 110000.0;
    save.RCROSS = 10000.0;

    spicelib::VPACK(0.0, 0.0, 0.0, save.CENTER.as_slice_mut());
    spicelib::VPACK(1.0, 0.0, 0.0, save.NORMAL.as_slice_mut());

    save.SURFID = 1;
    testutil::T_TORUS(
        ALPHA,
        save.SURFID,
        &save.FIXREF,
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

    save.R = 80000.0;

    spicelib::VPACK(0.0, 0.0, 0.0, save.CENTER.as_slice_mut());
    spicelib::VPACK(1.0, 0.0, 0.0, save.NORMAL.as_slice_mut());

    save.SURFID = 2;
    testutil::T_TORUS(
        ALPHA,
        save.SURFID,
        &save.FIXREF,
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

    save.R = 50000.0;

    spicelib::VPACK(0.0, 0.0, 0.0, save.CENTER.as_slice_mut());
    spicelib::VPACK(1.0, 0.0, 0.0, save.NORMAL.as_slice_mut());

    save.SURFID = 3;
    testutil::T_TORUS(
        ALPHA,
        save.SURFID,
        &save.FIXREF,
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

    spicelib::FURNSH(DSK4, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //
    // NESTED TORUS LIMB CASES
    //
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Limb computation using nestedtorus target shape. Use empty surface list. (ALPHA)",
        ctx,
    )?;

    //
    // Get solar radius.
    //
    spicelib::BODVCD(
        10,
        b"RADII",
        3,
        &mut save.N,
        save.SUNRAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CURVE = LMBCRV;
    save.SRCRAD = 0.0;
    save.SHAPE = DSKSHP;
    save.TRGCDE = 1000;
    fstr::assign(&mut save.TARGET, b"ALPHA");
    save.NSURF = 0;
    save.SRFLST[1] = 0;

    fstr::assign(&mut save.FIXREF, b"ALPHA_VIEW_XY");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    spicelib::SPKPOS(
        b"SUN",
        save.ET,
        &save.FIXREF,
        b"NONE",
        &save.TARGET,
        save.AXIS.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODVCD(
        save.TRGCDE,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VHAT(save.AXIS.as_slice(), save.ZVEC.as_slice_mut());
    spicelib::FRAME(
        save.ZVEC.as_slice_mut(),
        save.REFVEC.as_slice_mut(),
        save.VTEMP.as_slice_mut(),
    );

    //
    // The ZZTANINI call requires a plane reference vector,
    // so this call must be made for each cutting half-plane.
    //

    save.NCUTS = 10;
    save.DTHETA = (((2 as f64) * spicelib::PI(ctx)) / save.NCUTS as f64);

    for I in 1..=save.NCUTS {
        save.THETA = (((I - 1) as f64) * save.DTHETA);

        //
        // --- Case: ------------------------------------------------------
        //
        fstr::assign(&mut save.TITLE, b"Nested torus limb test: THETA (deg) = #.");
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            (save.THETA * spicelib::DPR(ctx)),
            14,
            &mut save.TITLE,
            ctx,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::TCASE(&save.TITLE, ctx)?;

        spicelib::VROTV(
            save.REFVEC.as_slice(),
            save.AXIS.as_slice(),
            save.THETA,
            save.PLNVEC.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::ZZTANINI(
            save.CURVE,
            save.SRCRAD,
            save.SHAPE,
            save.TRGCDE,
            save.NSURF,
            save.SRFLST.as_slice(),
            save.FIXFID,
            save.ET,
            save.PLNVEC.as_slice(),
            save.AXIS.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.SCHSTP = 0.0001;
        save.SOLTOL = 0.000000000000001;

        spicelib::ZZTANGNT(
            save.CURVE,
            save.SRCRAD,
            save.SHAPE,
            save.TRGCDE,
            save.NSURF,
            save.SRFLST.as_slice(),
            save.FIXFID,
            save.ET,
            save.PLNVEC.as_slice(),
            save.AXIS.as_slice(),
            save.SCHSTP,
            save.SOLTOL,
            save.RESULT.as_slice_mut(),
            save.POINTS.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Note that RESULT is a cell, not a window.
        //
        save.N = spicelib::CARDD(save.RESULT.as_slice(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // We expect to find six limb points in each cutting half-plane.
        //
        testutil::CHCKSI(b"N", save.N, b"=", 6, 0, OK, ctx)?;

        //
        // Check the points and the angles in the RESULT array.
        //
        for J in 1..=save.N {
            spicelib::VEQU(save.POINTS.subarray([1, J]), save.TANPNT.as_slice_mut());

            //
            // The returned point should be very close to the DSK surface.
            // Map the point to a point that is on the surface and check
            // the distance between the two.
            //
            if spicelib::ODD(J) {
                spicelib::VLCOM(
                    1.0,
                    save.TANPNT.as_slice(),
                    100.0,
                    save.PLNVEC.as_slice(),
                    save.VERTEX.as_slice_mut(),
                );
                spicelib::VMINUS(save.PLNVEC.as_slice(), save.DIR.as_slice_mut());
            } else {
                spicelib::VLCOM(
                    1.0,
                    save.TANPNT.as_slice(),
                    -100.0,
                    save.PLNVEC.as_slice(),
                    save.VERTEX.as_slice_mut(),
                );
                spicelib::VEQU(save.PLNVEC.as_slice(), save.DIR.as_slice_mut());
            }

            spicelib::DSKXV(
                false,
                &save.TARGET,
                save.NSURF,
                save.SRFLST.as_slice(),
                save.ET,
                &save.FIXREF,
                1,
                save.VERTEX.as_slice(),
                save.DIR.as_slice(),
                save.NPT.as_slice_mut(),
                std::slice::from_mut(&mut save.FOUND),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"TANPNT #");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.TANPNT.as_slice(),
                b"~~/",
                save.NPT.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;

            //
            // The returned point should be in the plane containing
            // the cutting half-plane.
            //
            spicelib::PSV2PL(
                save.ORIGIN.as_slice(),
                save.AXIS.as_slice(),
                save.PLNVEC.as_slice(),
                save.CUT.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::VPRJP(
                save.TANPNT.as_slice(),
                save.CUT.as_slice(),
                save.PROJ.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = MED;

            fstr::assign(&mut save.LABEL, b"PROJ #");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.PROJ.as_slice(),
                b"~~",
                save.TANPNT.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;

            //
            // The returned point should be in the correct half-plane.
            //
            save.DP = spicelib::VDOT(save.TANPNT.as_slice(), save.PLNVEC.as_slice());

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"YVEC dot #");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSD(&save.LABEL, save.DP, b">", 0.0, save.TOL, OK, ctx)?;

            //
            // Compute the direction vector of the current tangent ray.
            //
            spicelib::VSUB(
                save.TANPNT.as_slice(),
                save.AXIS.as_slice(),
                save.RAYDIR.as_slice_mut(),
            );

            //
            // The returned point should be on the line containing the
            // ray.
            //
            spicelib::NPLNPT(
                save.AXIS.as_slice(),
                save.RAYDIR.as_slice(),
                save.TANPNT.as_slice(),
                save.PNEAR.as_slice_mut(),
                &mut save.DIST,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"PNEAR #");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.PNEAR.as_slice(),
                b"~~/",
                save.TANPNT.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;

            //
            // The returned point should be on the correct side of the
            // vertex of the line containing the ray.
            //
            spicelib::VSUB(
                save.TANPNT.as_slice(),
                save.AXIS.as_slice(),
                save.OFFSET.as_slice_mut(),
            );

            save.DP = spicelib::VDOT(save.OFFSET.as_slice(), save.RAYDIR.as_slice());

            fstr::assign(&mut save.LABEL, b"AXIS dot #");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSD(&save.LABEL, save.DP, b">", 0.0, save.TOL, OK, ctx)?;

            //
            // The angle between RAYDIR and AXIS should match the
            // Jth element of RESULT.
            //
            save.ANGLE = spicelib::VSEP(save.AXIS.as_slice(), save.RAYDIR.as_slice(), ctx);

            // WRITE (*,*) '-------'
            // WRITE (*,*) 'ANGLE  = ', ANGLE
            // WRITE (*,*) 'RESULT = ', RESULT(J)

            fstr::assign(&mut save.LABEL, b"RESULT(#)");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = TIGHT;

            testutil::CHCKSD(
                &save.LABEL,
                save.RESULT[J],
                b"~",
                save.ANGLE,
                save.TOL,
                OK,
                ctx,
            )?;
        }
    }

    //
    //
    // NESTED TORUS UMBRAL TERMINATOR CASES
    //
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Umbral terminator computation using nestedtorus target shape. Use empty surface list. (ALPHA)", ctx)?;

    //
    // Get solar radius.
    //
    spicelib::BODVCD(
        10,
        b"RADII",
        3,
        &mut save.N,
        save.SUNRAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CURVE = UMBRAL;
    save.SRCRAD = save.SUNRAD[1];
    save.SHAPE = DSKSHP;
    save.TRGCDE = 1000;
    fstr::assign(&mut save.TARGET, b"ALPHA");
    save.NSURF = 0;
    save.SRFLST[1] = 0;

    fstr::assign(&mut save.FIXREF, b"ALPHA_VIEW_XY");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    spicelib::SPKPOS(
        b"SUN",
        save.ET,
        &save.FIXREF,
        b"NONE",
        &save.TARGET,
        save.AXIS.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODVCD(
        save.TRGCDE,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VHAT(save.AXIS.as_slice(), save.ZVEC.as_slice_mut());
    spicelib::FRAME(
        save.ZVEC.as_slice_mut(),
        save.REFVEC.as_slice_mut(),
        save.VTEMP.as_slice_mut(),
    );

    //
    // The ZZTANINI call requires a plane reference vector,
    // so this call must be made for each cutting half-plane.
    //

    save.NCUTS = 10;
    save.DTHETA = (((2 as f64) * spicelib::PI(ctx)) / save.NCUTS as f64);

    for I in 1..=save.NCUTS {
        save.THETA = (((I - 1) as f64) * save.DTHETA);

        //
        // --- Case: ------------------------------------------------------
        //
        fstr::assign(
            &mut save.TITLE,
            b"Nested torus umbral terminator test: THETA (deg) = #.",
        );
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            (save.THETA * spicelib::DPR(ctx)),
            14,
            &mut save.TITLE,
            ctx,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::TCASE(&save.TITLE, ctx)?;

        spicelib::VROTV(
            save.REFVEC.as_slice(),
            save.AXIS.as_slice(),
            save.THETA,
            save.PLNVEC.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::ZZTANINI(
            save.CURVE,
            save.SRCRAD,
            save.SHAPE,
            save.TRGCDE,
            save.NSURF,
            save.SRFLST.as_slice(),
            save.FIXFID,
            save.ET,
            save.PLNVEC.as_slice(),
            save.AXIS.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.SCHSTP = 0.0001;
        save.SOLTOL = 0.000000000000001;

        spicelib::ZZTANGNT(
            save.CURVE,
            save.SRCRAD,
            save.SHAPE,
            save.TRGCDE,
            save.NSURF,
            save.SRFLST.as_slice(),
            save.FIXFID,
            save.ET,
            save.PLNVEC.as_slice(),
            save.AXIS.as_slice(),
            save.SCHSTP,
            save.SOLTOL,
            save.RESULT.as_slice_mut(),
            save.POINTS.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Note that RESULT is a cell, not a window.
        //
        save.N = spicelib::CARDD(save.RESULT.as_slice(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // We expect to find six umbral terminator points in each cutting
        // half-plane.
        //
        testutil::CHCKSI(b"N", save.N, b"=", 6, 0, OK, ctx)?;

        //
        // Check the points and the angles in the RESULT array.
        //
        for J in 1..=save.N {
            spicelib::VEQU(save.POINTS.subarray([1, J]), save.TANPNT.as_slice_mut());

            //
            // The returned point should be very close to the DSK surface.
            // Map the point to a point that is on the surface and check
            // the distance between the two.
            //
            if spicelib::ODD(J) {
                spicelib::VLCOM(
                    1.0,
                    save.TANPNT.as_slice(),
                    100.0,
                    save.PLNVEC.as_slice(),
                    save.VERTEX.as_slice_mut(),
                );
                spicelib::VMINUS(save.PLNVEC.as_slice(), save.DIR.as_slice_mut());
            } else {
                spicelib::VLCOM(
                    1.0,
                    save.TANPNT.as_slice(),
                    -100.0,
                    save.PLNVEC.as_slice(),
                    save.VERTEX.as_slice_mut(),
                );
                spicelib::VEQU(save.PLNVEC.as_slice(), save.DIR.as_slice_mut());
            }

            spicelib::DSKXV(
                false,
                &save.TARGET,
                save.NSURF,
                save.SRFLST.as_slice(),
                save.ET,
                &save.FIXREF,
                1,
                save.VERTEX.as_slice(),
                save.DIR.as_slice(),
                save.NPT.as_slice_mut(),
                std::slice::from_mut(&mut save.FOUND),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"TANPNT #");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.TANPNT.as_slice(),
                b"~~/",
                save.NPT.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;

            //
            // The returned point should be in the plane containing
            // the cutting half-plane.
            //
            spicelib::PSV2PL(
                save.ORIGIN.as_slice(),
                save.AXIS.as_slice(),
                save.PLNVEC.as_slice(),
                save.CUT.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::VPRJP(
                save.TANPNT.as_slice(),
                save.CUT.as_slice(),
                save.PROJ.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = MED;

            fstr::assign(&mut save.LABEL, b"PROJ #");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.PROJ.as_slice(),
                b"~~",
                save.TANPNT.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;

            //
            // The returned point should be in the correct half-plane.
            //
            save.DP = spicelib::VDOT(save.TANPNT.as_slice(), save.PLNVEC.as_slice());

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"YVEC dot #");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSD(&save.LABEL, save.DP, b">", 0.0, save.TOL, OK, ctx)?;

            //
            // Compute the direction vector of the current tangent ray.
            //
            // To generate a near-tangent ray passing through TANPNT, we
            // need to find the ray's vertex on the source. We can do this
            // by finding the limb on the source as seen from TANPNT.
            //
            spicelib::VSUB(
                save.TANPNT.as_slice(),
                save.AXIS.as_slice(),
                save.OFFSET.as_slice_mut(),
            );

            spicelib::EDLIMB(
                save.SUNRAD[1],
                save.SUNRAD[2],
                save.SUNRAD[3],
                save.OFFSET.as_slice(),
                save.LIMB.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // Caution: LIMB and CUT are expressed with respect to
            // different origins. Shift LIMB so its center is an offset
            // from the target center.
            //
            spicelib::VADD(
                save.AXIS.as_slice(),
                save.LIMB.subarray(1),
                save.VTEMP.as_slice_mut(),
            );
            spicelib::VEQU(save.VTEMP.as_slice(), save.LIMB.subarray_mut(1));

            spicelib::INELPL(
                save.LIMB.as_slice(),
                save.CUT.as_slice(),
                &mut save.NXPTS,
                save.XPT1.as_slice_mut(),
                save.XPT2.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // Generate basis vectors defined by AXIS (primary)
            // and PLNVEC (secondary).
            //
            spicelib::UCRSS(
                save.AXIS.as_slice(),
                save.PLNVEC.as_slice(),
                save.ZVEC.as_slice_mut(),
            );
            spicelib::VHAT(save.AXIS.as_slice(), save.XVEC.as_slice_mut());
            spicelib::UCRSS(
                save.ZVEC.as_slice(),
                save.XVEC.as_slice(),
                save.YVEC.as_slice_mut(),
            );

            if (spicelib::VDOT(save.XPT1.as_slice(), save.YVEC.as_slice()) > 0 as f64) {
                //
                // XPT1 is in the half-plane defined by AXIS and PLNVEC.
                //
                spicelib::VEQU(save.XPT1.as_slice(), save.SUNLPT.as_slice_mut());
            } else {
                spicelib::VEQU(save.XPT2.as_slice(), save.SUNLPT.as_slice_mut());
            }

            //
            // Now generate the ray's direction vector.
            //
            spicelib::VSUB(
                save.TANPNT.as_slice(),
                save.SUNLPT.as_slice(),
                save.RAYDIR.as_slice_mut(),
            );

            //
            // The returned point should be on the line containing the
            // ray.
            //
            spicelib::NPLNPT(
                save.SUNLPT.as_slice(),
                save.RAYDIR.as_slice(),
                save.TANPNT.as_slice(),
                save.PNEAR.as_slice_mut(),
                &mut save.DIST,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"PNEAR #");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.PNEAR.as_slice(),
                b"~~/",
                save.TANPNT.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;

            //
            // The returned point should be on the correct side of the
            // vertex of the line containing the ray.
            //
            spicelib::VSUB(
                save.TANPNT.as_slice(),
                save.AXIS.as_slice(),
                save.OFFSET.as_slice_mut(),
            );

            save.DP = spicelib::VDOT(save.OFFSET.as_slice(), save.RAYDIR.as_slice());

            fstr::assign(&mut save.LABEL, b"AXIS dot #");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSD(&save.LABEL, save.DP, b">", 0.0, save.TOL, OK, ctx)?;

            //
            // The angle between RAYDIR and AXIS should match the
            // Jth element of RESULT.
            //
            // We can't just use VSEP to find the rotation angle of the
            // tangent ray with respect to AXIS, because the angle may be
            // larger than pi. Compute the angle from the components of
            // RAYDIR.
            //
            save.CDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.XVEC.as_slice());
            save.SDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.YVEC.as_slice());

            save.ANGLE = f64::atan2(save.SDIR, save.CDIR);
            // WRITE (*,*) '-------'
            // WRITE (*,*) 'ANGLE  = ', ANGLE
            // WRITE (*,*) 'RESULT = ', RESULT(J)

            fstr::assign(&mut save.LABEL, b"RESULT(#)");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = TIGHT;

            testutil::CHCKSD(
                &save.LABEL,
                save.RESULT[J],
                b"~",
                save.ANGLE,
                save.TOL,
                OK,
                ctx,
            )?;
        }
    }

    //
    //
    // NESTED TORUS PENUMBRAL TERMINATOR CASES
    //
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Penumbral terminator computation using nestedtorus target shape. Use empty surface list. (ALPHA)", ctx)?;

    //
    // Get solar radius.
    //
    spicelib::BODVCD(
        10,
        b"RADII",
        3,
        &mut save.N,
        save.SUNRAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CURVE = PNMBRL;
    save.SRCRAD = save.SUNRAD[1];
    save.SHAPE = DSKSHP;
    save.TRGCDE = 1000;
    fstr::assign(&mut save.TARGET, b"ALPHA");
    save.NSURF = 0;
    save.SRFLST[1] = 0;

    fstr::assign(&mut save.FIXREF, b"ALPHA_VIEW_XY");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = 0.0;

    spicelib::SPKPOS(
        b"SUN",
        save.ET,
        &save.FIXREF,
        b"NONE",
        &save.TARGET,
        save.AXIS.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODVCD(
        save.TRGCDE,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VHAT(save.AXIS.as_slice(), save.ZVEC.as_slice_mut());
    spicelib::FRAME(
        save.ZVEC.as_slice_mut(),
        save.REFVEC.as_slice_mut(),
        save.VTEMP.as_slice_mut(),
    );

    //
    // The ZZTANINI call requires a plane reference vector,
    // so this call must be made for each cutting half-plane.
    //

    save.NCUTS = 10;
    save.DTHETA = (((2 as f64) * spicelib::PI(ctx)) / save.NCUTS as f64);

    for I in 1..=save.NCUTS {
        save.THETA = (((I - 1) as f64) * save.DTHETA);

        //
        // --- Case: ------------------------------------------------------
        //
        fstr::assign(
            &mut save.TITLE,
            b"Nested torus penumbral terminator test: THETA (deg) = #.",
        );
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            (save.THETA * spicelib::DPR(ctx)),
            14,
            &mut save.TITLE,
            ctx,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::TCASE(&save.TITLE, ctx)?;

        spicelib::VROTV(
            save.REFVEC.as_slice(),
            save.AXIS.as_slice(),
            save.THETA,
            save.PLNVEC.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::ZZTANINI(
            save.CURVE,
            save.SRCRAD,
            save.SHAPE,
            save.TRGCDE,
            save.NSURF,
            save.SRFLST.as_slice(),
            save.FIXFID,
            save.ET,
            save.PLNVEC.as_slice(),
            save.AXIS.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.SCHSTP = 0.0001;
        save.SOLTOL = 0.000000000000001;

        spicelib::ZZTANGNT(
            save.CURVE,
            save.SRCRAD,
            save.SHAPE,
            save.TRGCDE,
            save.NSURF,
            save.SRFLST.as_slice(),
            save.FIXFID,
            save.ET,
            save.PLNVEC.as_slice(),
            save.AXIS.as_slice(),
            save.SCHSTP,
            save.SOLTOL,
            save.RESULT.as_slice_mut(),
            save.POINTS.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Note that RESULT is a cell, not a window.
        //
        save.N = spicelib::CARDD(save.RESULT.as_slice(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // We expect to find six penumbral terminator points in each
        // cutting half-plane.
        //
        testutil::CHCKSI(b"N", save.N, b"=", 6, 0, OK, ctx)?;

        //
        // Check the points and the angles in the RESULT array.
        //
        for J in 1..=save.N {
            spicelib::VEQU(save.POINTS.subarray([1, J]), save.TANPNT.as_slice_mut());

            //
            // The returned point should be very close to the DSK surface.
            // Map the point to a point that is on the surface and check
            // the distance between the two.
            //
            if spicelib::ODD(J) {
                spicelib::VLCOM(
                    1.0,
                    save.TANPNT.as_slice(),
                    100.0,
                    save.PLNVEC.as_slice(),
                    save.VERTEX.as_slice_mut(),
                );
                spicelib::VMINUS(save.PLNVEC.as_slice(), save.DIR.as_slice_mut());
            } else {
                spicelib::VLCOM(
                    1.0,
                    save.TANPNT.as_slice(),
                    -100.0,
                    save.PLNVEC.as_slice(),
                    save.VERTEX.as_slice_mut(),
                );
                spicelib::VEQU(save.PLNVEC.as_slice(), save.DIR.as_slice_mut());
            }

            spicelib::DSKXV(
                false,
                &save.TARGET,
                save.NSURF,
                save.SRFLST.as_slice(),
                save.ET,
                &save.FIXREF,
                1,
                save.VERTEX.as_slice(),
                save.DIR.as_slice(),
                save.NPT.as_slice_mut(),
                std::slice::from_mut(&mut save.FOUND),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"TANPNT #");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.TANPNT.as_slice(),
                b"~~/",
                save.NPT.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;

            //
            // The returned point should be in the plane containing
            // the cutting half-plane.
            //
            spicelib::PSV2PL(
                save.ORIGIN.as_slice(),
                save.AXIS.as_slice(),
                save.PLNVEC.as_slice(),
                save.CUT.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::VPRJP(
                save.TANPNT.as_slice(),
                save.CUT.as_slice(),
                save.PROJ.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = MED;

            fstr::assign(&mut save.LABEL, b"PROJ #");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.PROJ.as_slice(),
                b"~~",
                save.TANPNT.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;

            //
            // The returned point should be in the correct half-plane.
            //
            save.DP = spicelib::VDOT(save.TANPNT.as_slice(), save.PLNVEC.as_slice());

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"YVEC dot #");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSD(&save.LABEL, save.DP, b">", 0.0, save.TOL, OK, ctx)?;

            //
            // Compute the direction vector of the current tangent ray.
            //
            // To generate a near-tangent ray passing through TANPNT, we
            // need to find the ray's vertex on the source. We can do this
            // by finding the limb on the source as seen from TANPNT.
            //
            spicelib::VSUB(
                save.TANPNT.as_slice(),
                save.AXIS.as_slice(),
                save.OFFSET.as_slice_mut(),
            );

            spicelib::EDLIMB(
                save.SUNRAD[1],
                save.SUNRAD[2],
                save.SUNRAD[3],
                save.OFFSET.as_slice(),
                save.LIMB.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // Caution: LIMB and CUT are expressed with respect to
            // different origins. Shift LIMB so its center is an offset
            // from the target center.
            //
            spicelib::VADD(
                save.AXIS.as_slice(),
                save.LIMB.subarray(1),
                save.VTEMP.as_slice_mut(),
            );
            spicelib::VEQU(save.VTEMP.as_slice(), save.LIMB.subarray_mut(1));

            spicelib::INELPL(
                save.LIMB.as_slice(),
                save.CUT.as_slice(),
                &mut save.NXPTS,
                save.XPT1.as_slice_mut(),
                save.XPT2.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // Generate basis vectors defined by AXIS (primary)
            // and PLNVEC (secondary).
            //
            spicelib::UCRSS(
                save.AXIS.as_slice(),
                save.PLNVEC.as_slice(),
                save.ZVEC.as_slice_mut(),
            );
            spicelib::VHAT(save.AXIS.as_slice(), save.XVEC.as_slice_mut());
            spicelib::UCRSS(
                save.ZVEC.as_slice(),
                save.XVEC.as_slice(),
                save.YVEC.as_slice_mut(),
            );

            if (spicelib::VDOT(save.XPT1.as_slice(), save.YVEC.as_slice()) < 0 as f64) {
                //
                // XPT1 is in the half-plane on the opposite side
                // of AXI from the one defined by AXIS and PLNVEC.
                //
                spicelib::VEQU(save.XPT1.as_slice(), save.SUNLPT.as_slice_mut());
            } else {
                spicelib::VEQU(save.XPT2.as_slice(), save.SUNLPT.as_slice_mut());
            }

            //
            // Now generate the ray's direction vector.
            //
            spicelib::VSUB(
                save.TANPNT.as_slice(),
                save.SUNLPT.as_slice(),
                save.RAYDIR.as_slice_mut(),
            );

            //
            // The returned point should be on the line containing the
            // ray.
            //
            spicelib::NPLNPT(
                save.SUNLPT.as_slice(),
                save.RAYDIR.as_slice(),
                save.TANPNT.as_slice(),
                save.PNEAR.as_slice_mut(),
                &mut save.DIST,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"PNEAR #");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.PNEAR.as_slice(),
                b"~~/",
                save.TANPNT.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;

            //
            // The returned point should be on the correct side of the
            // vertex of the line containing the ray.
            //
            spicelib::VSUB(
                save.TANPNT.as_slice(),
                save.AXIS.as_slice(),
                save.OFFSET.as_slice_mut(),
            );

            save.DP = spicelib::VDOT(save.OFFSET.as_slice(), save.RAYDIR.as_slice());

            fstr::assign(&mut save.LABEL, b"AXIS dot #");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSD(&save.LABEL, save.DP, b">", 0.0, save.TOL, OK, ctx)?;

            //
            // The angle between RAYDIR and AXIS should match the
            // Jth element of RESULT.
            //
            // We can't just use VSEP to find the rotation angle of the
            // tangent ray with respect to AXIS, because the angle may be
            // larger than pi. Compute the angle from the components of
            // RAYDIR.
            //
            save.CDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.XVEC.as_slice());
            save.SDIR = spicelib::VDOT(save.RAYDIR.as_slice(), save.YVEC.as_slice());

            save.ANGLE = f64::atan2(save.SDIR, save.CDIR);

            fstr::assign(&mut save.LABEL, b"RESULT(#)");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = TIGHT;

            testutil::CHCKSD(
                &save.LABEL,
                save.RESULT[J],
                b"~",
                save.ANGLE,
                save.TOL,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up.", ctx)?;

    //
    // Clean up.
    //
    spicelib::DELFIL(PCK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(PCK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKUEF(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DELFIL(SPK1, ctx)?;
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

    spicelib::KCLEAR(ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
