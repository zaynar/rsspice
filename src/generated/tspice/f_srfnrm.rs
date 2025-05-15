//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const VERIDX: i32 = 1;
const LLBIDX: i32 = (VERIDX + 1);
const LLEIDX: i32 = (LLBIDX + 1);
const NULPTR: i32 = -1;
const BWDIDX: i32 = 1;
const FWDIDX: i32 = (BWDIDX + 1);
const IBSIDX: i32 = (FWDIDX + 1);
const ISZIDX: i32 = (IBSIDX + 1);
const DBSIDX: i32 = (ISZIDX + 1);
const DSZIDX: i32 = (DBSIDX + 1);
const CBSIDX: i32 = (DSZIDX + 1);
const CSZIDX: i32 = (CBSIDX + 1);
const DLADSZ: i32 = CSZIDX;
const FMTVER: i32 = 1000000;
const NCHREC: i32 = 1024;
const MAXSRF: i32 = 100;
const IXNV: i32 = 1;
const IXNP: i32 = (IXNV + 1);
const IXNVXT: i32 = (IXNP + 1);
const IXVGRX: i32 = (IXNVXT + 1);
const IXCGSC: i32 = (IXVGRX + 3);
const IXVXPS: i32 = (IXCGSC + 1);
const IXVXLS: i32 = (IXVXPS + 1);
const IXVTLS: i32 = (IXVXLS + 1);
const IXPLAT: i32 = (IXVTLS + 1);
const IXDSCR: i32 = 1;
const DSCSZ2: i32 = 24;
const IXVTBD: i32 = (IXDSCR + DSCSZ2);
const IXVXOR: i32 = (IXVTBD + 6);
const IXVXSZ: i32 = (IXVXOR + 3);
const IXVERT: i32 = (IXVXSZ + 1);
const KWNV: i32 = 1;
const KWNP: i32 = (KWNV + 1);
const KWNVXT: i32 = (KWNP + 1);
const KWVGRX: i32 = (KWNVXT + 1);
const KWCGSC: i32 = (KWVGRX + 1);
const KWVXPS: i32 = (KWCGSC + 1);
const KWVXLS: i32 = (KWVXPS + 1);
const KWVTLS: i32 = (KWVXLS + 1);
const KWPLAT: i32 = (KWVTLS + 1);
const KWVXPT: i32 = (KWPLAT + 1);
const KWVXPL: i32 = (KWVXPT + 1);
const KWVTPT: i32 = (KWVXPL + 1);
const KWVTPL: i32 = (KWVTPT + 1);
const KWCGPT: i32 = (KWVTPL + 1);
const KWDSC: i32 = (KWCGPT + 1);
const KWVTBD: i32 = (KWDSC + 1);
const KWVXOR: i32 = (KWVTBD + 1);
const KWVXSZ: i32 = (KWVXOR + 1);
const KWVERT: i32 = (KWVXSZ + 1);
const MAXVRT: i32 = 16000002;
const MAXPLT: i32 = (2 * (MAXVRT - 2));
const MAXNPV: i32 = (((3 * MAXPLT) / 2) + 1);
const MAXVOX: i32 = 100000000;
const MAXCGR: i32 = 100000;
const MAXEDG: i32 = 120;
const SIVGRX: i32 = 1;
const SICGSC: i32 = (SIVGRX + 3);
const SIVXNP: i32 = (SICGSC + 1);
const SIVXNL: i32 = (SIVXNP + 1);
const SIVTNL: i32 = (SIVXNL + 1);
const SICGRD: i32 = (SIVTNL + 1);
const IXIFIX: i32 = (MAXCGR + 7);
const SIVTBD: i32 = 1;
const SIVXOR: i32 = (SIVTBD + 6);
const SIVXSZ: i32 = (SIVXOR + 3);
const IXDFIX: i32 = 10;
const MAXVXP: i32 = (MAXPLT / 2);
const MAXCEL: i32 = 60000000;
const MXNVLS: i32 = (MAXCEL + (MAXVXP / 2));
const SPAISZ: i32 = ((((IXIFIX + MAXVXP) + MXNVLS) + MAXVRT) + MAXNPV);
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
const DSK0: &[u8] = b"latsrf_dsk0.bds";
const DSK1: &[u8] = b"latsrf_dsk1.bds";
const DSK2: &[u8] = b"latsrf_dsk2.bds";
const DSK3: &[u8] = b"latsrf_dsk3.bds";
const PCK: &[u8] = b"test_0008.tpc";
const VTIGHT: f64 = 0.00000000000001;
const LBLSIZ: i32 = 50;
const LNSIZE: i32 = 320;
const NAMLEN: i32 = 32;
const NREF: i32 = 2;
const TIMLEN: i32 = 50;
const NTARG: i32 = 2;
const MAXPTS: i32 = 100;
const NMAP: i32 = 5;
const NMETH: i32 = 4;
const NTEXT: i32 = 10;

struct SaveVars {
    FIXREF: Vec<u8>,
    FRMTXT: ActualCharArray,
    KVNAME: Vec<u8>,
    LABEL: Vec<u8>,
    METHOD: Vec<u8>,
    METHDS: ActualCharArray,
    PNTDEF: Vec<u8>,
    REFS: ActualCharArray2D,
    SHAPE: Vec<u8>,
    SRFNMS: ActualCharArray,
    SUBTYP: Vec<u8>,
    TARGET: Vec<u8>,
    TRMTYP: Vec<u8>,
    TRGFRM: Vec<u8>,
    TRGNMS: ActualCharArray,
    TITLE: Vec<u8>,
    UTC: Vec<u8>,
    ALTRAD: StackArray<f64, 3>,
    BADRAD: StackArray<f64, 3>,
    DC: StackArray<f64, 1>,
    DLAT: f64,
    DLON: f64,
    DSKDSC: StackArray<f64, 24>,
    ET: f64,
    ET0: f64,
    LAT: f64,
    LON: f64,
    LONLAT: StackArray2D<f64, 200>,
    MAXRAD: f64,
    NORMAL: StackArray<f64, 3>,
    NORMLS: ActualArray2D<f64>,
    R: f64,
    RADII: StackArray<f64, 3>,
    RAYDIR: StackArray<f64, 3>,
    SPOINT: StackArray<f64, 3>,
    SRFPTS: ActualArray2D<f64>,
    TMPVEC: StackArray<f64, 3>,
    TOL: f64,
    VERTEX: StackArray<f64, 3>,
    XFORM: StackArray2D<f64, 9>,
    XNORML: ActualArray2D<f64>,
    XPT: StackArray<f64, 3>,
    BODYID: i32,
    DLADSC: StackArray<i32, 8>,
    FIXFID: i32,
    HANDLE: i32,
    IC: StackArray<i32, 1>,
    K: i32,
    MAXD: i32,
    MAXI: i32,
    N: i32,
    NLAT: i32,
    NLON: i32,
    NPTS: i32,
    NREFS: StackArray<i32, 2>,
    NSFLAT: i32,
    NSFLON: i32,
    NSURF: i32,
    PTIDX: i32,
    SEGFID: i32,
    SRFBOD: StackArray<i32, 5>,
    SRFIDS: StackArray<i32, 5>,
    SRFLST: StackArray<i32, 100>,
    SURFID: i32,
    TRGCDE: i32,
    FOUND: bool,
    ISDSK: bool,
    PRI: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FIXREF = vec![b' '; NAMLEN as usize];
        let mut FRMTXT = ActualCharArray::new(LNSIZE, 1..=NTEXT);
        let mut KVNAME = vec![b' '; NAMLEN as usize];
        let mut LABEL = vec![b' '; LBLSIZ as usize];
        let mut METHOD = vec![b' '; MTHLEN as usize];
        let mut METHDS = ActualCharArray::new(MTHLEN, 1..=NMETH);
        let mut PNTDEF = vec![b' '; NAMLEN as usize];
        let mut REFS = ActualCharArray2D::new(NAMLEN, 1..=NREF, 1..=NTARG);
        let mut SHAPE = vec![b' '; NAMLEN as usize];
        let mut SRFNMS = ActualCharArray::new(NAMLEN, 1..=NMAP);
        let mut SUBTYP = vec![b' '; NAMLEN as usize];
        let mut TARGET = vec![b' '; NAMLEN as usize];
        let mut TRMTYP = vec![b' '; NAMLEN as usize];
        let mut TRGFRM = vec![b' '; NAMLEN as usize];
        let mut TRGNMS = ActualCharArray::new(NAMLEN, 1..=NTARG);
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut UTC = vec![b' '; TIMLEN as usize];
        let mut ALTRAD = StackArray::<f64, 3>::new(1..=3);
        let mut BADRAD = StackArray::<f64, 3>::new(1..=3);
        let mut DC = StackArray::<f64, 1>::new(1..=1);
        let mut DLAT: f64 = 0.0;
        let mut DLON: f64 = 0.0;
        let mut DSKDSC = StackArray::<f64, 24>::new(1..=DSKDSZ);
        let mut ET: f64 = 0.0;
        let mut ET0: f64 = 0.0;
        let mut LAT: f64 = 0.0;
        let mut LON: f64 = 0.0;
        let mut LONLAT = StackArray2D::<f64, 200>::new(1..=2, 1..=MAXPTS);
        let mut MAXRAD: f64 = 0.0;
        let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
        let mut NORMLS = ActualArray2D::<f64>::new(1..=3, 1..=MAXPTS);
        let mut R: f64 = 0.0;
        let mut RADII = StackArray::<f64, 3>::new(1..=3);
        let mut RAYDIR = StackArray::<f64, 3>::new(1..=3);
        let mut SPOINT = StackArray::<f64, 3>::new(1..=3);
        let mut SRFPTS = ActualArray2D::<f64>::new(1..=3, 1..=MAXPTS);
        let mut TMPVEC = StackArray::<f64, 3>::new(1..=3);
        let mut TOL: f64 = 0.0;
        let mut VERTEX = StackArray::<f64, 3>::new(1..=3);
        let mut XFORM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut XNORML = ActualArray2D::<f64>::new(1..=3, 1..=MAXPTS);
        let mut XPT = StackArray::<f64, 3>::new(1..=3);
        let mut BODYID: i32 = 0;
        let mut DLADSC = StackArray::<i32, 8>::new(1..=DLADSZ);
        let mut FIXFID: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut IC = StackArray::<i32, 1>::new(1..=1);
        let mut K: i32 = 0;
        let mut MAXD: i32 = 0;
        let mut MAXI: i32 = 0;
        let mut N: i32 = 0;
        let mut NLAT: i32 = 0;
        let mut NLON: i32 = 0;
        let mut NPTS: i32 = 0;
        let mut NREFS = StackArray::<i32, 2>::new(1..=NTARG);
        let mut NSFLAT: i32 = 0;
        let mut NSFLON: i32 = 0;
        let mut NSURF: i32 = 0;
        let mut PTIDX: i32 = 0;
        let mut SEGFID: i32 = 0;
        let mut SRFBOD = StackArray::<i32, 5>::new(1..=NMAP);
        let mut SRFIDS = StackArray::<i32, 5>::new(1..=NMAP);
        let mut SRFLST = StackArray::<i32, 100>::new(1..=MAXSRF);
        let mut SURFID: i32 = 0;
        let mut TRGCDE: i32 = 0;
        let mut FOUND: bool = false;
        let mut ISDSK: bool = false;
        let mut PRI: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"IAU_MARS"),
                Val::C(b"MARS_FIXED"),
                Val::C(b"IAU_PHOBOS"),
                Val::C(b" "),
            ]
            .into_iter();
            REFS.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(2), Val::I(1)].into_iter();
            NREFS
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

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
            FIXREF,
            FRMTXT,
            KVNAME,
            LABEL,
            METHOD,
            METHDS,
            PNTDEF,
            REFS,
            SHAPE,
            SRFNMS,
            SUBTYP,
            TARGET,
            TRMTYP,
            TRGFRM,
            TRGNMS,
            TITLE,
            UTC,
            ALTRAD,
            BADRAD,
            DC,
            DLAT,
            DLON,
            DSKDSC,
            ET,
            ET0,
            LAT,
            LON,
            LONLAT,
            MAXRAD,
            NORMAL,
            NORMLS,
            R,
            RADII,
            RAYDIR,
            SPOINT,
            SRFPTS,
            TMPVEC,
            TOL,
            VERTEX,
            XFORM,
            XNORML,
            XPT,
            BODYID,
            DLADSC,
            FIXFID,
            HANDLE,
            IC,
            K,
            MAXD,
            MAXI,
            N,
            NLAT,
            NLON,
            NPTS,
            NREFS,
            NSFLAT,
            NSFLON,
            NSURF,
            PTIDX,
            SEGFID,
            SRFBOD,
            SRFIDS,
            SRFLST,
            SURFID,
            TRGCDE,
            FOUND,
            ISDSK,
            PRI,
        }
    }
}

//$Procedure      F_SRFNRM ( SRFNRM family tests )
pub fn F_SRFNRM(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    testutil::TOPEN(b"F_SRFNRM", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Setup:  create PCK file.", ctx)?;

    //
    // Create the PCK file, and load it. Do not delete it.
    //
    testutil::T_PCK08(PCK, true, true, ctx)?;
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
    // Add an incomplete frame definition to the kernel pool;
    // we'll need this later.
    //
    spicelib::PIPOOL(b"FRAME_BAD_NAME", 1, &[-666], ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Setup: create alternate Mars-fixed frame.", ctx)?;

    fstr::assign(
        save.FRMTXT.get_mut(1),
        b"FRAME_MARS_FIXED         = 1499000",
    );
    fstr::assign(
        save.FRMTXT.get_mut(2),
        b"FRAME_1499000_NAME       = \'MARS_FIXED\' ",
    );
    fstr::assign(save.FRMTXT.get_mut(3), b"FRAME_1499000_CLASS      = 4");
    fstr::assign(
        save.FRMTXT.get_mut(4),
        b"FRAME_1499000_CLASS_ID   = 1499000",
    );
    fstr::assign(save.FRMTXT.get_mut(5), b"FRAME_1499000_CENTER     = 499");
    fstr::assign(
        save.FRMTXT.get_mut(6),
        b"TKFRAME_1499000_RELATIVE = \'IAU_MARS\' ",
    );
    fstr::assign(
        save.FRMTXT.get_mut(7),
        b"TKFRAME_1499000_SPEC     = \'MATRIX\' ",
    );
    fstr::assign(
        save.FRMTXT.get_mut(8),
        b"TKFRAME_1499000_MATRIX   = ( 0, 1, 0,",
    );
    fstr::assign(
        save.FRMTXT.get_mut(9),
        b"                           0, 0, 1,",
    );
    fstr::assign(
        save.FRMTXT.get_mut(10),
        b"                           1, 0, 0, )",
    );

    spicelib::LMPOOL(save.FRMTXT.as_arg(), NTEXT, ctx)?;
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
    save.NLON = 200;
    save.NLAT = 100;

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
    // Surface 2 for Mars is very low-res. We also use a
    // different scale for the Mars radii used to create
    // the tessellated shape model.
    //
    save.BODYID = save.TRGCDE;
    save.SURFID = 2;
    save.NLON = 40;
    save.NLAT = 20;

    if spicelib::EXISTS(DSK1, ctx)? {
        spicelib::DELFIL(DSK1, ctx)?;
    }

    fstr::assign(&mut save.KVNAME, b"BODY499_RADII");
    spicelib::GDPOOL(
        &save.KVNAME,
        1,
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"499 radii FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::VSCL(2.0, save.RADII.as_slice(), save.ALTRAD.as_slice_mut());

    spicelib::PDPOOL(&save.KVNAME, 3, save.ALTRAD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

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
    // Restore normal Mars radii.
    //
    spicelib::PDPOOL(&save.KVNAME, 3, save.RADII.as_slice(), ctx)?;
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
    // Surface 2 for Phobos is lower-res. We also use a
    // different scale for the Mars radii used to create
    // the tessellated shape model.
    //
    save.BODYID = 401;
    save.SURFID = 2;
    fstr::assign(&mut save.FIXREF, b"IAU_PHOBOS");

    save.NLON = 80;
    save.NLAT = 40;

    if spicelib::EXISTS(DSK3, ctx)? {
        spicelib::DELFIL(DSK3, ctx)?;
    }

    fstr::assign(&mut save.KVNAME, b"BODY401_RADII");
    spicelib::GDPOOL(
        &save.KVNAME,
        1,
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"401 radii FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::VSCL(3.0, save.RADII.as_slice(), save.ALTRAD.as_slice_mut());

    spicelib::PDPOOL(&save.KVNAME, 3, save.ALTRAD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

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

    //
    // Restore normal Phobos radii.
    //
    spicelib::PDPOOL(&save.KVNAME, 3, save.RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load the DSK.
    //
    spicelib::FURNSH(DSK3, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Setup: create surface map.", ctx)?;

    //
    // Set up a surface name-ID map.
    //
    save.SRFBOD[1] = 499;
    save.SRFIDS[1] = 1;
    fstr::assign(save.SRFNMS.get_mut(1), b"high-res");

    save.SRFBOD[2] = 499;
    save.SRFIDS[2] = 2;
    fstr::assign(save.SRFNMS.get_mut(2), b"low-res");

    save.SRFBOD[3] = 499;
    save.SRFIDS[3] = 3;
    fstr::assign(save.SRFNMS.get_mut(3), b"c-g");

    save.SRFBOD[4] = 401;
    save.SRFIDS[4] = 1;
    fstr::assign(save.SRFNMS.get_mut(4), b"high-res");

    save.SRFBOD[5] = 401;
    save.SRFIDS[5] = 2;
    fstr::assign(save.SRFNMS.get_mut(5), b"low-res");

    spicelib::PCPOOL(b"NAIF_SURFACE_NAME", NMAP, save.SRFNMS.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(b"NAIF_SURFACE_CODE", NMAP, save.SRFIDS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(b"NAIF_SURFACE_BODY", NMAP, save.SRFBOD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Generate a grid of planetocentric longitude/latitude coordinate
    // pairs. These, combined with surface models, will yield a grid of
    // coordinates at which to compute surface points.
    //
    save.NSFLON = 4;
    save.NSFLAT = 5;
    save.NPTS = (save.NSFLON * save.NSFLAT);

    save.DLON = (((2 as f64) * spicelib::PI(ctx)) / save.NSFLON as f64);
    save.DLAT = (spicelib::PI(ctx) / (save.NSFLAT - 1) as f64);

    save.K = 0;

    for I in 1..=save.NSFLON {
        //
        // We shift the coordinates away from possible plate
        // edges because we can't expect the normal vectors
        // to match our computed values at those locations.
        //
        save.LON = ((((I - 1) as f64) * save.DLON) + 0.001);

        for J in 1..=save.NSFLAT {
            save.LAT = spicelib::BRCKTD(
                (spicelib::HALFPI(ctx) - (((J - 1) as f64) * save.DLAT)),
                -spicelib::HALFPI(ctx),
                spicelib::HALFPI(ctx),
            );

            if (J <= (save.NSFLAT / 2)) {
                save.LAT = (save.LAT - 0.001);
            } else {
                save.LAT = (save.LAT + 0.001);
            }

            save.K = (save.K + 1);

            save.LONLAT[[1, save.K]] = save.LON;
            save.LONLAT[[2, save.K]] = save.LAT;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     The first test loop follows. In this loop, we call SRFNRM
    //     once for each coordinate pair.
    //

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
        // Loop over the surface point sequence.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NPTS;
            let m3__: i32 = 1;
            save.PTIDX = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.LON = save.LONLAT[[1, save.PTIDX]];
                save.LAT = save.LONLAT[[2, save.PTIDX]];

                //
                // Loop over every target body-fixed frame choice.
                //
                for REFIDX in 1..=save.NREFS[TRGIDX] {
                    fstr::assign(&mut save.TRGFRM, save.REFS.get([REFIDX, TRGIDX]));

                    spicelib::NAMFRM(&save.TRGFRM, &mut save.FIXFID, ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Loop over all method choices.
                    //
                    for MIX in 1..=NMETH {
                        fstr::assign(&mut save.METHOD, save.METHDS.get(MIX));

                        save.ISDSK = spicelib::MATCHI(&save.METHOD, b"*DSK*", b"*", b"?", ctx);

                        //
                        //- Case: ------------------------------------------------------
                        //
                        fstr::assign(&mut save.TITLE, b"Target = #; TRGFRM = #; METHOD = #; Longitude (deg) = #; Latitude (deg) = #; ET = #.");
                        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
                        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TRGFRM, &mut save.TITLE);
                        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.METHOD, &mut save.TITLE);
                        spicelib::REPMD(
                            &save.TITLE.to_vec(),
                            b"#",
                            (save.LON * spicelib::DPR(ctx)),
                            14,
                            &mut save.TITLE,
                            ctx,
                        );
                        spicelib::REPMD(
                            &save.TITLE.to_vec(),
                            b"#",
                            (save.LAT * spicelib::DPR(ctx)),
                            14,
                            &mut save.TITLE,
                            ctx,
                        );
                        spicelib::REPMD(
                            &save.TITLE.to_vec(),
                            b"#",
                            save.ET,
                            14,
                            &mut save.TITLE,
                            ctx,
                        );
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        testutil::TCASE(&save.TITLE, ctx)?;

                        //
                        // Extract the surface list from the method string.
                        //
                        spicelib::ZZPRSMET(
                            save.TRGCDE,
                            &save.METHOD,
                            MAXSRF,
                            &mut save.SHAPE,
                            &mut save.SUBTYP,
                            &mut save.PRI,
                            &mut save.NSURF,
                            save.SRFLST.as_slice_mut(),
                            &mut save.PNTDEF,
                            &mut save.TRMTYP,
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        //
                        // Obtain a surface point corresponding to the input
                        // coordinates.
                        //
                        spicelib::LATSRF(
                            &save.METHOD,
                            &save.TARGET,
                            save.ET,
                            &save.TRGFRM,
                            1,
                            save.LONLAT.subarray([1, save.PTIDX]),
                            save.SPOINT.as_slice_mut(),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        //
                        // Call SRFNRM to produce a normal vector for the
                        // input coordinates.
                        //
                        spicelib::SRFNRM(
                            &save.METHOD,
                            &save.TARGET,
                            save.ET,
                            &save.TRGFRM,
                            1,
                            save.SPOINT.as_slice(),
                            save.NORMAL.as_slice_mut(),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        //
                        // Prepare to check the result. If the target shape is
                        // given by DSK data, we'll need to find the plate on
                        // which the surface point lies.
                        //
                        // Get an outer bounding radius for the target.
                        //
                        if save.ISDSK {
                            //
                            // Perform initialization to enable generation
                            // of a bounding radius for the current surface
                            // list.
                            //
                            spicelib::ZZSUDSKI(
                                save.TRGCDE,
                                save.NSURF,
                                save.SRFLST.as_slice(),
                                save.FIXFID,
                                ctx,
                            )?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            spicelib::ZZMAXRAD(&mut save.MAXRAD, ctx);
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            //
                            // Create an inward-pointing ray. The surface
                            // intercept of this ray is our expected result from
                            // SRFNRM.
                            //
                            save.R = (2.0 * save.MAXRAD);

                            spicelib::LATREC(
                                save.R,
                                save.LON,
                                save.LAT,
                                save.VERTEX.as_slice_mut(),
                            );
                            spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

                            // Call DSKXSI to produce a surface point for the
                            // input coordinates, and to identify the plate
                            // on which this point lies.
                            //
                            save.MAXD = 1;
                            save.MAXI = 1;

                            spicelib::DSKXSI(
                                false,
                                &save.TARGET,
                                save.NSURF,
                                save.SRFLST.as_slice(),
                                save.ET,
                                &save.TRGFRM,
                                save.VERTEX.as_slice(),
                                save.RAYDIR.as_slice(),
                                save.MAXD,
                                save.MAXI,
                                save.XPT.as_slice_mut(),
                                &mut save.HANDLE,
                                save.DLADSC.as_slice_mut(),
                                save.DSKDSC.as_slice_mut(),
                                save.DC.as_slice_mut(),
                                save.IC.as_slice_mut(),
                                &mut save.FOUND,
                                ctx,
                            )?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

                            //
                            // Get the normal vector for the indicated plate.
                            //
                            spicelib::DSKN02(
                                save.HANDLE,
                                save.DLADSC.as_slice(),
                                save.IC[1],
                                save.XNORML.as_slice_mut(),
                                ctx,
                            )?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            //
                            // The vector returned by DSKN02 is in the frame
                            // of the designated segment. Transform the vector
                            // to the SRFNRM request frame, if necessary.
                            //
                            save.SEGFID = intrinsics::IDNINT(save.DSKDSC[FRMIDX]);

                            if (save.SEGFID != save.FIXFID) {
                                spicelib::REFCHG(
                                    save.SEGFID,
                                    save.FIXFID,
                                    save.ET,
                                    save.XFORM.as_slice_mut(),
                                    ctx,
                                )?;
                                testutil::CHCKXC(false, b" ", OK, ctx)?;

                                spicelib::MXV(
                                    save.XFORM.as_slice(),
                                    save.XNORML.as_slice(),
                                    save.TMPVEC.as_slice_mut(),
                                );
                                spicelib::VEQU(save.TMPVEC.as_slice(), save.XNORML.as_slice_mut());
                            }
                        } else {
                            //
                            // This is the ellipsoid case. Compute the
                            // surface normal directly. (Recall SURFNM
                            // is the low-level ellipsoid routine.)
                            //
                            spicelib::SURFNM(
                                save.RADII[1],
                                save.RADII[2],
                                save.RADII[3],
                                save.SPOINT.as_slice(),
                                save.XNORML.as_slice_mut(),
                                ctx,
                            )?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                        }

                        //
                        // We expect near-perfect agreement between the
                        // actual and expected normal vector.
                        //
                        save.TOL = VTIGHT;

                        testutil::CHCKAD(
                            b"NORMAL",
                            save.NORMAL.as_slice(),
                            b"~~/",
                            save.XNORML.as_slice(),
                            3,
                            save.TOL,
                            OK,
                            ctx,
                        )?;
                    }
                    //
                    // End of the method loop.
                    //
                }
                //
                // End of the reference frame loop.
                //
                save.PTIDX += m3__;
            }
        }
        //
        // End of the surface point loop.
        //
    }
    //
    // End of the target loop.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     The second test loop follows. In this loop, we call SRFNRM
    //     once for the full set of coordinate pairs.
    //

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
        // Loop over every target body-fixed frame choice.
        //
        for REFIDX in 1..=save.NREFS[TRGIDX] {
            fstr::assign(&mut save.TRGFRM, save.REFS.get([REFIDX, TRGIDX]));

            spicelib::NAMFRM(&save.TRGFRM, &mut save.FIXFID, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Loop over all method choices.
            //
            for MIX in 1..=NMETH {
                fstr::assign(&mut save.METHOD, save.METHDS.get(MIX));

                save.ISDSK = spicelib::MATCHI(&save.METHOD, b"*DSK*", b"*", b"?", ctx);

                //
                //- Case: ------------------------------------------------------
                //
                fstr::assign(
                    &mut save.TITLE,
                    b"Target = #; TRGFRM = #; METHOD = #; ET = #.",
                );
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TRGFRM, &mut save.TITLE);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.METHOD, &mut save.TITLE);
                spicelib::REPMD(
                    &save.TITLE.to_vec(),
                    b"#",
                    save.ET,
                    14,
                    &mut save.TITLE,
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&save.TITLE, ctx)?;

                //
                // Check the output point array. In order to do this, we'll
                // extract the surface list from the method string.

                spicelib::ZZPRSMET(
                    save.TRGCDE,
                    &save.METHOD,
                    MAXSRF,
                    &mut save.SHAPE,
                    &mut save.SUBTYP,
                    &mut save.PRI,
                    &mut save.NSURF,
                    save.SRFLST.as_slice_mut(),
                    &mut save.PNTDEF,
                    &mut save.TRMTYP,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Generate the array of surface points.
                //
                spicelib::LATSRF(
                    &save.METHOD,
                    &save.TARGET,
                    save.ET,
                    &save.TRGFRM,
                    save.NPTS,
                    save.LONLAT.as_slice(),
                    save.SRFPTS.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Call SRFNRM to produce a normal vector for each
                // surface point.
                //
                spicelib::SRFNRM(
                    &save.METHOD,
                    &save.TARGET,
                    save.ET,
                    &save.TRGFRM,
                    save.NPTS,
                    save.SRFPTS.as_slice(),
                    save.NORMLS.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                if save.ISDSK {
                    //
                    // To check the normal vectors, we'll find the plates
                    // with which the vectors are associated. We'll use
                    // DSKXSI for this.
                    //
                    // Get an outer bounding radius for the target.
                    //
                    // Perform initialization to enable generation
                    // of a bounding radius for the current surface
                    // list.
                    //
                    spicelib::ZZSUDSKI(
                        save.TRGCDE,
                        save.NSURF,
                        save.SRFLST.as_slice(),
                        save.FIXFID,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::ZZMAXRAD(&mut save.MAXRAD, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    //
                    // Create an array of inward-pointing rays. The
                    // respective surface intercepts of these rays are our
                    // expected results from SRFNRM.
                    //
                    save.R = (2.0 * save.MAXRAD);

                    {
                        let m1__: i32 = 1;
                        let m2__: i32 = save.NPTS;
                        let m3__: i32 = 1;
                        save.PTIDX = m1__;
                        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                            save.LON = save.LONLAT[[1, save.PTIDX]];
                            save.LAT = save.LONLAT[[2, save.PTIDX]];

                            spicelib::LATREC(
                                save.R,
                                save.LON,
                                save.LAT,
                                save.VERTEX.as_slice_mut(),
                            );
                            spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

                            save.MAXD = 1;
                            save.MAXI = 1;

                            spicelib::DSKXSI(
                                false,
                                &save.TARGET,
                                save.NSURF,
                                save.SRFLST.as_slice(),
                                save.ET,
                                &save.TRGFRM,
                                save.VERTEX.as_slice(),
                                save.RAYDIR.as_slice(),
                                save.MAXD,
                                save.MAXI,
                                save.XPT.as_slice_mut(),
                                &mut save.HANDLE,
                                save.DLADSC.as_slice_mut(),
                                save.DSKDSC.as_slice_mut(),
                                save.DC.as_slice_mut(),
                                save.IC.as_slice_mut(),
                                &mut save.FOUND,
                                ctx,
                            )?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            //
                            // Get the normal vector for the current surface
                            // point.
                            //
                            spicelib::DSKN02(
                                save.HANDLE,
                                save.DLADSC.as_slice(),
                                save.IC[1],
                                save.XNORML.subarray_mut([1, save.PTIDX]),
                                ctx,
                            )?;

                            //
                            // The vector returned by DSKN02 is in the frame
                            // of the designated segment. Transform the vector
                            // to the SRFNRM request frame, if necessary.
                            //
                            save.SEGFID = intrinsics::IDNINT(save.DSKDSC[FRMIDX]);

                            if (save.SEGFID != save.FIXFID) {
                                spicelib::REFCHG(
                                    save.SEGFID,
                                    save.FIXFID,
                                    save.ET,
                                    save.XFORM.as_slice_mut(),
                                    ctx,
                                )?;
                                testutil::CHCKXC(false, b" ", OK, ctx)?;

                                spicelib::MXV(
                                    save.XFORM.as_slice(),
                                    save.XNORML.subarray([1, save.PTIDX]),
                                    save.TMPVEC.as_slice_mut(),
                                );
                                spicelib::VEQU(
                                    save.TMPVEC.as_slice(),
                                    save.XNORML.subarray_mut([1, save.PTIDX]),
                                );
                            }

                            save.PTIDX += m3__;
                        }
                    }
                } else {
                    //
                    // Compute normal vectors on the reference ellipsoid
                    // for the current target.
                    //
                    {
                        let m1__: i32 = 1;
                        let m2__: i32 = save.NPTS;
                        let m3__: i32 = 1;
                        save.PTIDX = m1__;
                        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                            spicelib::SURFNM(
                                save.RADII[1],
                                save.RADII[2],
                                save.RADII[3],
                                save.SRFPTS.subarray([1, save.PTIDX]),
                                save.XNORML.subarray_mut([1, save.PTIDX]),
                                ctx,
                            )?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            save.PTIDX += m3__;
                        }
                    }
                }

                //
                // Check the normal vectors.
                //
                // We should get extremely good agreement.
                //
                {
                    let m1__: i32 = 1;
                    let m2__: i32 = save.NPTS;
                    let m3__: i32 = 1;
                    save.PTIDX = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        fstr::assign(&mut save.LABEL, b"FOUND #");
                        spicelib::REPMI(
                            &save.LABEL.to_vec(),
                            b"#",
                            save.PTIDX,
                            &mut save.LABEL,
                            ctx,
                        );
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                        //
                        // We expect near-perfect agreement between the
                        // actual and expected normal vector.
                        //
                        save.TOL = VTIGHT;

                        testutil::CHCKAD(
                            b"NORMLS",
                            save.NORMLS.as_slice(),
                            b"~~/",
                            save.XNORML.as_slice(),
                            (3 * save.NPTS),
                            save.TOL,
                            OK,
                            ctx,
                        )?;

                        if !*OK {
                            ctx.stop()?;
                        }

                        save.PTIDX += m3__;
                    }
                }
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
    // End of the target loop.
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
    // Input handling tests:  make sure target
    // can be identified using integer "name."
    //
    testutil::TCASE(b"Use integer target name.", ctx)?;

    //
    // Set target and target-fixed frame.
    //
    fstr::assign(&mut save.TARGET, b"MARS");
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    fstr::assign(&mut save.METHOD, b"ellipsoid");

    spicelib::LATSRF(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        1,
        save.LONLAT.subarray([1, save.PTIDX]),
        save.SPOINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SRFNRM(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        1,
        save.SPOINT.as_slice(),
        save.XNORML.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SRFNRM(
        &save.METHOD,
        b"499",
        save.ET,
        &save.FIXREF,
        1,
        save.SPOINT.as_slice(),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"NORMAL",
        save.NORMAL.as_slice(),
        b"=",
        save.XNORML.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;

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

    spicelib::SRFNRM(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        1,
        save.SPOINT.as_slice(),
        save.XNORML.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODDEF(b"JUPITER", 499, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SRFNRM(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        1,
        save.SPOINT.as_slice(),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect exact matches here.
    //
    testutil::CHCKAD(
        b"NORMAL",
        save.NORMAL.as_slice(),
        b"=",
        save.XNORML.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;
    //
    // Restore original mapping.
    //
    spicelib::BODDEF(b"JUPITER", 599, ctx)?;
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

    fstr::assign(&mut save.METHOD, b"dsk/unprioritized/surfaces = 1");

    spicelib::LATSRF(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        1,
        save.LONLAT.subarray([1, save.PTIDX]),
        save.SPOINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SRFNRM(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        1,
        save.SPOINT.as_slice(),
        save.XNORML.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(save.SRFNMS.get_mut(1), b"AAAbbb");

    spicelib::PCPOOL(b"NAIF_SURFACE_NAME", NMAP, save.SRFNMS.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.METHOD, b"dsk/unprioritized/surfaces = AAAbbb");

    spicelib::SRFNRM(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        1,
        save.SPOINT.as_slice(),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect exact matches here.
    //
    testutil::CHCKAD(
        b"NORMAL",
        save.NORMAL.as_slice(),
        b"=",
        save.XNORML.as_slice(),
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

    fstr::assign(&mut save.METHOD, b"dsk/unprioritized/surfaces = low-res");

    spicelib::LATSRF(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        1,
        save.LONLAT.subarray([1, save.PTIDX]),
        save.SPOINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SRFNRM(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        1,
        save.SPOINT.as_slice(),
        save.XNORML.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Unload the high-res DSK; set METHOD to remove
    // surface specification.
    //
    spicelib::UNLOAD(DSK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.METHOD, b"dsk/unprioritized");

    spicelib::SRFNRM(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        1,
        save.SPOINT.as_slice(),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect exact matches here.
    //
    testutil::CHCKAD(
        b"NORMAL",
        save.NORMAL.as_slice(),
        b"=",
        save.XNORML.as_slice(),
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
    testutil::TCASE(b"Unload Mars low-res DSK; reload Mars high-res DSK.", ctx)?;

    //
    // Restore DSK, unload low-res DSK, and repeat computation.
    //
    spicelib::FURNSH(DSK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(DSK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LATSRF(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        1,
        save.LONLAT.subarray([1, save.PTIDX]),
        save.SPOINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SRFNRM(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        1,
        save.SPOINT.as_slice(),
        save.XNORML.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure the result matches that obtained with the
    // high-res DSK specified.
    //
    fstr::assign(
        &mut save.METHOD,
        b"dsk/unprioritized/ SURFACES = \"HIGH-RES\" ",
    );

    spicelib::SRFNRM(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        1,
        save.SPOINT.as_slice(),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect exact matches here.
    //
    testutil::CHCKAD(
        b"NORMAL",
        save.NORMAL.as_slice(),
        b"=",
        save.XNORML.as_slice(),
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

    save.ET = 0.0;
    fstr::assign(&mut save.TARGET, b"MARS");
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");

    fstr::assign(&mut save.METHOD, b"ELLIPSOD");

    spicelib::SRFNRM(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        1,
        save.SRFPTS.subarray([1, (save.NPTS / 2)]),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADMETHODSYNTAX)", OK, ctx)?;

    fstr::assign(&mut save.METHOD, b"ELLIPSOID/UNPRIORITIZED");

    spicelib::SRFNRM(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        1,
        save.SRFPTS.subarray([1, (save.NPTS / 2)]),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADMETHODSYNTAX)", OK, ctx)?;

    fstr::assign(&mut save.METHOD, b"DSK/unprioritized/intercept");

    spicelib::SRFNRM(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        1,
        save.SRFPTS.subarray([1, (save.NPTS / 2)]),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDMETHOD)", OK, ctx)?;

    fstr::assign(&mut save.METHOD, b"DSK/nadir/unprioritized");

    spicelib::SRFNRM(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        1,
        save.SRFPTS.subarray([1, (save.NPTS / 2)]),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDMETHOD)", OK, ctx)?;

    fstr::assign(&mut save.METHOD, b"DSK");

    spicelib::SRFNRM(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        1,
        save.SRFPTS.subarray([1, (save.NPTS / 2)]),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADPRIORITYSPEC)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid target name.", ctx)?;

    fstr::assign(&mut save.METHOD, b"DSK/UNPRIORITIZED");

    spicelib::SRFNRM(
        &save.METHOD,
        b"XXX",
        save.ET,
        &save.FIXREF,
        1,
        save.SRFPTS.subarray([1, (save.NPTS / 2)]),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid reference frame center", ctx)?;

    spicelib::SRFNRM(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        b"IAU_EARTH",
        1,
        save.SRFPTS.subarray([1, (save.NPTS / 2)]),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Unrecognized reference frame", ctx)?;

    spicelib::SRFNRM(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        b"ZZZ",
        1,
        save.SRFPTS.subarray([1, (save.NPTS / 2)]),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No orientation data for target", ctx)?;

    //
    // This error applies only to the DSK case.
    //
    fstr::assign(&mut save.METHOD, b"UNPRIORITIZED  /  DSK");

    spicelib::CLPOOL(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a class 2 (PCK) frame with no orientation data.
    //

    fstr::assign(
        save.FRMTXT.get_mut(1),
        b"FRAME_MARS_FIXED_PCK     = 1499001",
    );
    fstr::assign(
        save.FRMTXT.get_mut(2),
        b"FRAME_1499001_NAME       = \'MARS_FIXED_PCK\' ",
    );
    fstr::assign(save.FRMTXT.get_mut(3), b"FRAME_1499001_CLASS      = 2");
    fstr::assign(
        save.FRMTXT.get_mut(4),
        b"FRAME_1499001_CLASS_ID   = 1499001",
    );
    fstr::assign(save.FRMTXT.get_mut(5), b"FRAME_1499001_CENTER     = 499");

    spicelib::LMPOOL(save.FRMTXT.as_arg(), 5, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LMPOOL(save.FRMTXT.as_arg(), NTEXT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SRFNRM(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        b"MARS_FIXED_PCK",
        1,
        save.SRFPTS.subarray([1, (save.NPTS / 2)]),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(FRAMEDATANOTFOUND)", OK, ctx)?;

    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LDPOOL(PCK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No radius data for target", ctx)?;

    fstr::assign(&mut save.METHOD, b"ELLIPSOID");

    spicelib::DVPOOL(b"BODY499_RADII", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SRFNRM(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        1,
        save.SRFPTS.subarray([1, (save.NPTS / 2)]),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    spicelib::LDPOOL(PCK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad radius data for target", ctx)?;

    fstr::assign(&mut save.METHOD, b"ELLIPSOID");

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

    spicelib::SRFNRM(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        1,
        save.SRFPTS.subarray([1, (save.NPTS / 2)]),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    //
    // Replace original radii.
    //
    spicelib::PDPOOL(b"BODY499_RADII", 3, save.RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"No loaded DSKs.", ctx)?;

    fstr::assign(&mut save.METHOD, b"UNPRIORITIZED  /  DSK");

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

    fstr::assign(&mut save.METHOD, b"dsk/unprioritized");

    spicelib::SRFNRM(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        1,
        save.SRFPTS.subarray([1, (save.NPTS / 2)]),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOLOADEDDSKFILES)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Clean up.
    //
    spicelib::DELFIL(PCK, ctx)?;
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
