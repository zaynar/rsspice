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
const PCK0: &[u8] = b"zzdskbux_test0.tpc";
const TOPFK0: &[u8] = b"zzdskbux_mars_topo.tf";
const TOPFK1: &[u8] = b"zzdskbux_saturn_topo.tf";
const TOPSP0: &[u8] = b"zzdskbux_mars_topo.bsp";
const TOPSP1: &[u8] = b"zzdskbux_saturn_topo.bsp";
const TIGHT: f64 = 0.000000000001;
const NAMLEN: i32 = 32;
const FILSIZ: i32 = 255;
const LABSIZ: i32 = 40;
const MAXFH: i32 = 2;
const MAXFW: i32 = 4;
const MAXH: i32 = 8;
const MAXW: i32 = 4;
const NTILEH: i32 = 3;
const NTILEW: i32 = 6;
const NLAYER: i32 = 3;
const NTARG: i32 = 2;
const MAXP: i32 = 20000;
const MAXV: i32 = (MAXP / 2);

struct SaveVars {
    FIXREF: Vec<u8>,
    FIXRFS: ActualCharArray,
    FRAME: Vec<u8>,
    LABEL: Vec<u8>,
    SITFNM: ActualCharArray3D,
    SITNMS: ActualCharArray3D,
    TOPFK: Vec<u8>,
    TOPSPK: Vec<u8>,
    TARGET: Vec<u8>,
    TARGS: ActualCharArray,
    TILDSK: ActualCharArray4D,
    A: f64,
    ANGLES: StackArray3D<f64, 24>,
    B: f64,
    BOUNDS: StackArray2D<f64, 4>,
    C: f64,
    CORPAR: StackArray<f64, 10>,
    CTRLAT: f64,
    CTRLON: f64,
    DC: StackArray<f64, 1>,
    DFRLAT: f64,
    DFRLON: f64,
    DLAT: f64,
    DLON: f64,
    DPLAT: f64,
    DPLON: f64,
    DSKDSC: StackArray<f64, 24>,
    ET: f64,
    FIRST: f64,
    LAT: f64,
    LAST: f64,
    LON: f64,
    LT: f64,
    MAXLAT: f64,
    MAXLON: f64,
    MINLAT: f64,
    MINLON: f64,
    NORMAL: StackArray<f64, 3>,
    OFFSET: StackArray<f64, 3>,
    R: f64,
    RADII: StackArray<f64, 3>,
    RAYDIR: StackArray<f64, 3>,
    SCALE: f64,
    SEGDIR: StackArray<f64, 3>,
    SEGVTX: StackArray<f64, 3>,
    SITPOS: StackArray4D<f64, 48>,
    TILEH: f64,
    TILEW: f64,
    TOL: f64,
    VERTEX: StackArray<f64, 3>,
    VERTS: ActualArray2D<f64>,
    VTEMP: StackArray<f64, 3>,
    XDSKDS: StackArray<f64, 24>,
    XFORM: StackArray2D<f64, 9>,
    XNORML: StackArray<f64, 3>,
    XPT: StackArray<f64, 3>,
    XXPT: StackArray<f64, 3>,
    AXES: StackArray3D<i32, 24>,
    BIDS: StackArray<i32, 2>,
    BIX: i32,
    BODYID: i32,
    CLASS: i32,
    CLSSID: i32,
    CORSYS: i32,
    DLADSC: StackArray<i32, 8>,
    FIXFID: i32,
    FIXH: i32,
    FIXW: i32,
    FRMCTR: i32,
    HANDLE: i32,
    IC: StackArray<i32, 1>,
    LAYSRF: ActualArray2D<i32>,
    N: i32,
    NLAT: i32,
    NLON: i32,
    NP: i32,
    NPLAT: i32,
    NPLON: i32,
    NV: i32,
    NSITES: i32,
    NSURF: i32,
    PLATES: ActualArray<i32>,
    PLID: i32,
    SEGFID: i32,
    SITFID: StackArray3D<i32, 16>,
    SITIDS: StackArray3D<i32, 16>,
    SURFID: i32,
    FOUND: bool,
    MAKVTL: bool,
    USEPAD: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FIXREF = vec![b' '; NAMLEN as usize];
        let mut FIXRFS = ActualCharArray::new(NAMLEN, 1..=NTARG);
        let mut FRAME = vec![b' '; NAMLEN as usize];
        let mut LABEL = vec![b' '; LABSIZ as usize];
        let mut SITFNM = ActualCharArray3D::new(NAMLEN, 1..=MAXFH, 1..=MAXFW, 1..=NTARG);
        let mut SITNMS = ActualCharArray3D::new(NAMLEN, 1..=MAXFH, 1..=MAXFW, 1..=NTARG);
        let mut TOPFK = vec![b' '; FILSIZ as usize];
        let mut TOPSPK = vec![b' '; FILSIZ as usize];
        let mut TARGET = vec![b' '; NAMLEN as usize];
        let mut TARGS = ActualCharArray::new(NAMLEN, 1..=NTARG);
        let mut TILDSK =
            ActualCharArray4D::new(FILSIZ, 1..=NTILEH, 1..=NTILEW, 1..=NLAYER, 1..=NTARG);
        let mut A: f64 = 0.0;
        let mut ANGLES = StackArray3D::<f64, 24>::new(1..=3, 1..=MAXFH, 1..=MAXFW);
        let mut B: f64 = 0.0;
        let mut BOUNDS = StackArray2D::<f64, 4>::new(1..=2, 1..=2);
        let mut C: f64 = 0.0;
        let mut CORPAR = StackArray::<f64, 10>::new(1..=NSYPAR);
        let mut CTRLAT: f64 = 0.0;
        let mut CTRLON: f64 = 0.0;
        let mut DC = StackArray::<f64, 1>::new(1..=1);
        let mut DFRLAT: f64 = 0.0;
        let mut DFRLON: f64 = 0.0;
        let mut DLAT: f64 = 0.0;
        let mut DLON: f64 = 0.0;
        let mut DPLAT: f64 = 0.0;
        let mut DPLON: f64 = 0.0;
        let mut DSKDSC = StackArray::<f64, 24>::new(1..=DSKDSZ);
        let mut ET: f64 = 0.0;
        let mut FIRST: f64 = 0.0;
        let mut LAT: f64 = 0.0;
        let mut LAST: f64 = 0.0;
        let mut LON: f64 = 0.0;
        let mut LT: f64 = 0.0;
        let mut MAXLAT: f64 = 0.0;
        let mut MAXLON: f64 = 0.0;
        let mut MINLAT: f64 = 0.0;
        let mut MINLON: f64 = 0.0;
        let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
        let mut OFFSET = StackArray::<f64, 3>::new(1..=3);
        let mut R: f64 = 0.0;
        let mut RADII = StackArray::<f64, 3>::new(1..=3);
        let mut RAYDIR = StackArray::<f64, 3>::new(1..=3);
        let mut SCALE: f64 = 0.0;
        let mut SEGDIR = StackArray::<f64, 3>::new(1..=3);
        let mut SEGVTX = StackArray::<f64, 3>::new(1..=3);
        let mut SITPOS = StackArray4D::<f64, 48>::new(1..=3, 1..=MAXFH, 1..=MAXFW, 1..=NTARG);
        let mut TILEH: f64 = 0.0;
        let mut TILEW: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut VERTEX = StackArray::<f64, 3>::new(1..=3);
        let mut VERTS = ActualArray2D::<f64>::new(1..=3, 1..=MAXV);
        let mut VTEMP = StackArray::<f64, 3>::new(1..=3);
        let mut XDSKDS = StackArray::<f64, 24>::new(1..=DSKDSZ);
        let mut XFORM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut XNORML = StackArray::<f64, 3>::new(1..=3);
        let mut XPT = StackArray::<f64, 3>::new(1..=3);
        let mut XXPT = StackArray::<f64, 3>::new(1..=3);
        let mut AXES = StackArray3D::<i32, 24>::new(1..=3, 1..=MAXFH, 1..=MAXFW);
        let mut BIDS = StackArray::<i32, 2>::new(1..=NTARG);
        let mut BIX: i32 = 0;
        let mut BODYID: i32 = 0;
        let mut CLASS: i32 = 0;
        let mut CLSSID: i32 = 0;
        let mut CORSYS: i32 = 0;
        let mut DLADSC = StackArray::<i32, 8>::new(1..=DLADSZ);
        let mut FIXFID: i32 = 0;
        let mut FIXH: i32 = 0;
        let mut FIXW: i32 = 0;
        let mut FRMCTR: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut IC = StackArray::<i32, 1>::new(1..=1);
        let mut LAYSRF = ActualArray2D::<i32>::new(1..=MAXSRF, 1..=NLAYER);
        let mut N: i32 = 0;
        let mut NLAT: i32 = 0;
        let mut NLON: i32 = 0;
        let mut NP: i32 = 0;
        let mut NPLAT: i32 = 0;
        let mut NPLON: i32 = 0;
        let mut NV: i32 = 0;
        let mut NSITES: i32 = 0;
        let mut NSURF: i32 = 0;
        let mut PLATES = ActualArray::<i32>::new(1..=MAXP);
        let mut PLID: i32 = 0;
        let mut SEGFID: i32 = 0;
        let mut SITFID = StackArray3D::<i32, 16>::new(1..=MAXFH, 1..=MAXFW, 1..=NTARG);
        let mut SITIDS = StackArray3D::<i32, 16>::new(1..=MAXFH, 1..=MAXFW, 1..=NTARG);
        let mut SURFID: i32 = 0;
        let mut FOUND: bool = false;
        let mut MAKVTL: bool = false;
        let mut USEPAD: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(499), Val::I(699)].into_iter();
            BIDS.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"MARS"), Val::C(b"SATURN")].into_iter();
            TARGS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"IAU_MARS"), Val::C(b"IAU_SATURN")].into_iter();
            FIXRFS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            FIXREF,
            FIXRFS,
            FRAME,
            LABEL,
            SITFNM,
            SITNMS,
            TOPFK,
            TOPSPK,
            TARGET,
            TARGS,
            TILDSK,
            A,
            ANGLES,
            B,
            BOUNDS,
            C,
            CORPAR,
            CTRLAT,
            CTRLON,
            DC,
            DFRLAT,
            DFRLON,
            DLAT,
            DLON,
            DPLAT,
            DPLON,
            DSKDSC,
            ET,
            FIRST,
            LAT,
            LAST,
            LON,
            LT,
            MAXLAT,
            MAXLON,
            MINLAT,
            MINLON,
            NORMAL,
            OFFSET,
            R,
            RADII,
            RAYDIR,
            SCALE,
            SEGDIR,
            SEGVTX,
            SITPOS,
            TILEH,
            TILEW,
            TOL,
            VERTEX,
            VERTS,
            VTEMP,
            XDSKDS,
            XFORM,
            XNORML,
            XPT,
            XXPT,
            AXES,
            BIDS,
            BIX,
            BODYID,
            CLASS,
            CLSSID,
            CORSYS,
            DLADSC,
            FIXFID,
            FIXH,
            FIXW,
            FRMCTR,
            HANDLE,
            IC,
            LAYSRF,
            N,
            NLAT,
            NLON,
            NP,
            NPLAT,
            NPLON,
            NV,
            NSITES,
            NSURF,
            PLATES,
            PLID,
            SEGFID,
            SITFID,
            SITIDS,
            SURFID,
            FOUND,
            MAKVTL,
            USEPAD,
        }
    }
}

//$Procedure F_ZZDSKBUX ( ZZDSKBUX tests )
pub fn F_ZZDSKBUX(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //

    // DOUBLE PRECISION      VTIGHT
    // PARAMETER           ( VTIGHT = 1.D-14 )

    //
    // Local Variables
    //

    // INTEGER               K
    // INTEGER               L
    // INTEGER               NFLAT
    // INTEGER               NFLON
    // INTEGER               SRFLST ( MAXSRF )

    //
    // Saved values
    //

    //
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_ZZDSKBUX", ctx)?;

    //***********************************************************************
    //
    //     Set-up
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create topocentric kernels.", ctx)?;

    //
    // Our setup will be rather elaborate, since we're creating a
    // set of kernels that can be used to exercise all logic branches
    // of a moderately complex algorithm.
    //
    // We'll need a generic text PCK. Keep the file after loading it.
    //
    if spicelib::EXISTS(PCK0, ctx)? {
        spicelib::DELFIL(PCK0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    testutil::TSTPCK(PCK0, true, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a set of topocentric frames for Mars and Saturn.
    //
    save.FIRST = -((100 as f64) * spicelib::JYEAR());
    save.LAST = ((100 as f64) * spicelib::JYEAR());

    save.NSITES = (MAXFW * MAXFH);

    save.DFRLON = (((2 as f64) * spicelib::PI(ctx)) / MAXFW as f64);
    save.DFRLAT = (spicelib::PI(ctx) / MAXFH as f64);

    for I in 1..=NTARG {
        if (I == 1) {
            fstr::assign(&mut save.TOPFK, TOPFK0);
            fstr::assign(&mut save.TOPSPK, TOPSP0);
        } else {
            fstr::assign(&mut save.TOPFK, TOPFK1);
            fstr::assign(&mut save.TOPSPK, TOPSP1);
        }

        fstr::assign(&mut save.TARGET, save.TARGS.get(I));
        save.BODYID = save.BIDS[I];
        fstr::assign(&mut save.FIXREF, save.FIXRFS.get(I));

        for W in 1..=MAXFW {
            save.LON = (((W as f64) - 0.5) * save.DFRLON);

            for H in 1..=MAXFH {
                save.LAT = (spicelib::HALFPI(ctx) - (((H as f64) - 0.5) * save.DFRLAT));

                fstr::assign(save.SITNMS.get_mut([H, W, I]), b"#_H#_W#");
                spicelib::REPMC(
                    &save.SITNMS[[H, W, I]].to_vec(),
                    b"#",
                    &save.TARGET,
                    &mut save.SITNMS[[H, W, I]],
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(
                    &save.SITNMS[[H, W, I]].to_vec(),
                    b"#",
                    H,
                    &mut save.SITNMS[[H, W, I]],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(
                    &save.SITNMS[[H, W, I]].to_vec(),
                    b"#",
                    W,
                    &mut save.SITNMS[[H, W, I]],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.SITIDS[[H, W, I]] = (((1000 * save.BODYID) + H) + ((W - 1) * MAXFH));
                fstr::assign(save.SITFNM.get_mut([H, W, I]), save.SITNMS.get([H, W, I]));
                spicelib::SUFFIX(b"_TOPO", 0, &mut save.SITFNM[[H, W, I]]);

                save.SITFID[[H, W, I]] = save.SITIDS[[H, W, I]];

                save.AXES[[1, H, W]] = 3;
                save.AXES[[2, H, W]] = 2;
                save.AXES[[3, H, W]] = 3;

                save.ANGLES[[1, H, W]] = -save.LON;
                save.ANGLES[[2, H, W]] = (save.LAT - spicelib::HALFPI(ctx));
                save.ANGLES[[3, H, W]] = spicelib::PI(ctx);

                spicelib::SRFREC(
                    save.BODYID,
                    save.LON,
                    save.LAT,
                    save.SITPOS.subarray_mut([1, H, W, I]),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
            }
        }

        if spicelib::EXISTS(&save.TOPFK, ctx)? {
            spicelib::DELFIL(&save.TOPFK, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        if spicelib::EXISTS(&save.TOPSPK, ctx)? {
            spicelib::DELFIL(&save.TOPSPK, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        testutil::T_TOPKER(
            &save.TOPFK,
            &save.TOPSPK,
            &save.TARGET,
            &save.FIXREF,
            save.NSITES,
            save.SITIDS.subarray([1, 1, I]),
            save.SITNMS.subarray([1, 1, I]),
            save.SITPOS.subarray([1, 1, 1, I]),
            save.SITFNM.subarray([1, 1, I]),
            save.SITFID.subarray([1, 1, I]),
            save.FIRST,
            save.LAST,
            save.AXES.as_slice(),
            save.ANGLES.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Load the topocentric kernels.
    //
    spicelib::FURNSH(TOPFK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(TOPSP0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(TOPFK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(TOPSP1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create DSKs.", ctx)?;

    //
    // Start out by creating a three sets of tiled ellipsoid DSKs for
    // Mars and Saturn respectively. The DSKs will represent different
    // "layers": they'll be scaled versions of each other.
    //
    // The first layer will be a tessellation using the default
    // target body radii.
    //
    //
    // Set the coordinate system.
    //
    save.CORSYS = LATSYS;

    spicelib::CLEARD(3, save.CORPAR.as_slice_mut());

    //
    // Use padding.
    //
    save.USEPAD = true;

    //
    // Don't bother with the vertex-plate mapping.
    //
    save.MAKVTL = false;

    //
    // NLON and NLAT indicate the number of longitude and latitude
    // bands of plates per tile, respectively.
    //
    save.NLON = MAXW;
    save.NLAT = MAXH;

    //
    // Determine the spatial coverage of each tile.
    //
    save.TILEW = (((2 as f64) * spicelib::PI(ctx)) / NTILEW as f64);
    save.TILEH = (spicelib::PI(ctx) / NTILEH as f64);

    for TARGIX in 1..=NTARG {
        fstr::assign(&mut save.TARGET, save.TARGS.get(TARGIX));
        save.BODYID = save.BIDS[TARGIX];
        fstr::assign(&mut save.FIXREF, save.FIXRFS.get(TARGIX));

        //
        // Get the radii used for the tessellation.
        //
        spicelib::BODVCD(
            save.BODYID,
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

        for LAYRIX in 1..=NLAYER {
            for W in 1..=NTILEW {
                for H in 1..=NTILEH {
                    fstr::assign(
                        save.TILDSK.get_mut([H, W, LAYRIX, TARGIX]),
                        b"zzdskbux_#_h#_w#_layer#.bds",
                    );

                    spicelib::REPMC(
                        &save.TILDSK[[H, W, LAYRIX, TARGIX]].to_vec(),
                        b"#",
                        &save.TARGS[TARGIX],
                        &mut save.TILDSK[[H, W, LAYRIX, TARGIX]],
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::LCASE(
                        &save.TILDSK[[H, W, LAYRIX, TARGIX]].to_vec(),
                        &mut save.TILDSK[[H, W, LAYRIX, TARGIX]],
                        ctx,
                    );

                    spicelib::REPMI(
                        &save.TILDSK[[H, W, LAYRIX, TARGIX]].to_vec(),
                        b"#",
                        H,
                        &mut save.TILDSK[[H, W, LAYRIX, TARGIX]],
                        ctx,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::REPMI(
                        &save.TILDSK[[H, W, LAYRIX, TARGIX]].to_vec(),
                        b"#",
                        W,
                        &mut save.TILDSK[[H, W, LAYRIX, TARGIX]],
                        ctx,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::REPMI(
                        &save.TILDSK[[H, W, LAYRIX, TARGIX]].to_vec(),
                        b"#",
                        LAYRIX,
                        &mut save.TILDSK[[H, W, LAYRIX, TARGIX]],
                        ctx,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    // CALL TOSTDO ( '0 '//TILDSK(H,W,LAYRIX,TARGIX) )

                    if spicelib::EXISTS(&save.TILDSK[[H, W, LAYRIX, TARGIX]], ctx)? {
                        spicelib::DELFIL(&save.TILDSK[[H, W, LAYRIX, TARGIX]], ctx)?;
                    }

                    //
                    // Each tile gets its own surface ID.
                    //
                    save.SURFID = ((H + ((W - 1) * NTILEH)) + (((LAYRIX - 1) * NTILEH) * NTILEW));

                    //
                    // Set coverage for the current tile.
                    //
                    save.LON = (((W - 1) as f64) * save.TILEW);

                    save.BOUNDS[[1, 1]] = save.LON;
                    save.BOUNDS[[2, 1]] = (save.LON + save.TILEW);

                    save.LAT = (spicelib::HALFPI(ctx) - (((H - 1) as f64) * save.TILEH));

                    save.BOUNDS[[2, 2]] = save.LAT;
                    save.BOUNDS[[1, 2]] = (save.LAT - save.TILEH);

                    if (LAYRIX == 1) {
                        //
                        // Use one of the topocentric frames.
                        //
                        // Decide which frame to use. The frame will be that
                        // associated with the lat/lon rectangle centered at
                        // the closest topocentric frame center.
                        //
                        save.CTRLAT = (save.LAT - (save.TILEH / 2 as f64));
                        save.CTRLON = (save.LON + (save.TILEW / 2 as f64));

                        save.FIXH = intrinsics::MIN0(&[
                            ((((spicelib::HALFPI(ctx) - save.CTRLAT) / save.DFRLAT) as i32) + 1),
                            MAXFH,
                        ]);

                        save.FIXW =
                            intrinsics::MIN0(&[(((save.CTRLON / save.DFRLON) as i32) + 1), MAXFW]);

                        fstr::assign(
                            &mut save.FRAME,
                            save.SITFNM.get([save.FIXH, save.FIXW, TARGIX]),
                        );
                    } else {
                        fstr::assign(&mut save.FRAME, save.FIXRFS.get(TARGIX));

                        if (LAYRIX == 3) {
                            //
                            // Use greatly contracted radii.
                            //
                            save.SCALE = 0.1;

                            save.A = (save.RADII[1] * save.SCALE);
                            save.B = (save.RADII[2] * save.SCALE);
                            save.C = (save.RADII[3] * save.SCALE);
                        }
                    }

                    //
                    //  TILDSK(H,W,LAYRIX,TARGIX) = 'bux0.bds'
                    //
                    //
                    // Note the time bounds.
                    //
                    save.FIRST = (((10 * (LAYRIX - 4)) as f64) * spicelib::JYEAR());
                    save.LAST = (((10 * LAYRIX) as f64) * spicelib::JYEAR());

                    if (LAYRIX > 1) {
                        testutil::T_SECDS2(
                            save.BODYID,
                            save.SURFID,
                            &save.FRAME,
                            save.FIRST,
                            save.LAST,
                            save.CORSYS,
                            save.CORPAR.as_slice(),
                            save.BOUNDS.as_slice(),
                            save.A,
                            save.B,
                            save.C,
                            save.NLON,
                            save.NLAT,
                            save.MAKVTL,
                            save.USEPAD,
                            &save.TILDSK[[H, W, LAYRIX, TARGIX]],
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    } else {
                        //
                        // Create the same section we would have done for the
                        // body-centered frame, but use a topocentric frame.
                        // We can't call T_SECDS2 to do this for us; we'll
                        // need to get our hands dirty.
                        //
                        // We can, however, create the plate set easily
                        // enough.
                        //
                        save.MINLON = save.BOUNDS[[1, 1]];
                        save.MAXLON = save.BOUNDS[[2, 1]];
                        save.MINLAT = save.BOUNDS[[1, 2]];
                        save.MAXLAT = save.BOUNDS[[2, 2]];

                        //
                        // Add longitude and latitude padding.
                        //
                        save.DPLON =
                            ((save.BOUNDS[[2, 1]] - save.BOUNDS[[1, 1]]) / save.NLON as f64);
                        save.DPLAT =
                            ((save.BOUNDS[[2, 2]] - save.BOUNDS[[1, 2]]) / save.NLAT as f64);

                        save.NPLON = save.NLON;
                        save.NPLAT = save.NLAT;

                        if (save.MINLAT > 0.0) {
                            save.MINLAT = (save.MINLAT - save.DPLAT);
                            save.NPLAT = (save.NPLAT + 1);
                        }

                        if (save.MAXLAT < 0.0) {
                            save.MAXLAT = (save.MAXLAT + save.DPLAT);
                            save.NPLAT = (save.NPLAT + 1);
                        }

                        save.MINLON = (save.MINLON - save.DPLON);
                        save.MAXLON = (save.MAXLON + save.DPLON);
                        save.NPLON = (save.NPLON + 2);

                        support::ZZELLSEC(
                            save.A,
                            save.B,
                            save.C,
                            save.MINLON,
                            save.MAXLON,
                            save.MINLAT,
                            save.MAXLAT,
                            save.NPLON,
                            save.NPLAT,
                            MAXV,
                            MAXP,
                            &mut save.NV,
                            save.VERTS.as_slice_mut(),
                            &mut save.NP,
                            save.PLATES.as_slice_mut(),
                            ctx,
                        )?;
                        //
                        // We're not done with this DSK. We're going to shift
                        // the vertices to make them relative to the
                        // topocentric frame center, and we'll transform the
                        // vertices to the reference frame FRAME.
                        //
                        // Get the frame center offset.
                        //
                        spicelib::SPKEZP(
                            save.SITIDS[[save.FIXH, save.FIXW, TARGIX]],
                            0.0,
                            &save.FIXREF,
                            b"NONE",
                            save.BODYID,
                            save.OFFSET.as_slice_mut(),
                            &mut save.LT,
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        //
                        // Get the transformation from the body-centered,
                        // body-fixed frame to the topocentric one.
                        //
                        spicelib::PXFORM(
                            &save.FIXREF,
                            &save.SITFNM[[save.FIXH, save.FIXW, TARGIX]],
                            0.0,
                            save.XFORM.as_slice_mut(),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        for VIX in 1..=save.NV {
                            spicelib::VSUB(
                                save.VERTS.subarray([1, VIX]),
                                save.OFFSET.as_slice(),
                                save.VTEMP.as_slice_mut(),
                            );

                            spicelib::MXV(
                                save.XFORM.as_slice(),
                                save.VTEMP.as_slice(),
                                save.VERTS.subarray_mut([1, VIX]),
                            );
                        }

                        //
                        // Update the segment bounds to reflect that we're
                        // using a topocentric frame. The longitude extent is
                        // 360 degrees; the latitude extent is 180 degrees.
                        //
                        save.BOUNDS[[1, 1]] = 0.0;
                        save.BOUNDS[[2, 1]] = ((2 as f64) * spicelib::PI(ctx));
                        save.BOUNDS[[1, 2]] = -spicelib::HALFPI(ctx);
                        save.BOUNDS[[2, 2]] = spicelib::HALFPI(ctx);

                        //
                        // Create a DSK from our plate set.
                        //
                        testutil::T_WRTPLT(
                            save.BODYID,
                            save.SURFID,
                            &save.FRAME,
                            save.FIRST,
                            save.LAST,
                            save.CORSYS,
                            save.CORPAR.as_slice(),
                            save.BOUNDS.as_slice(),
                            save.NV,
                            save.NP,
                            save.VERTS.as_slice(),
                            save.PLATES.as_slice(),
                            save.MAKVTL,
                            &save.TILDSK[[H, W, LAYRIX, TARGIX]],
                            ctx,
                        )?;

                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    }
                }
            }
        }
    }

    //***********************************************************************
    //
    //     ZZDSKBUX, ZZSBFXR, ZZSBFXRI Normal cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Simple Mars intercept case. Use layers 1 and 2.", ctx)?;

    //
    // Load all DSKs.
    //

    for TARGIX in 1..=NTARG {
        for LAYRIX in 1..=NLAYER {
            for W in 1..=NTILEW {
                for H in 1..=NTILEH {
                    spicelib::FURNSH(&save.TILDSK[[H, W, LAYRIX, TARGIX]], ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }
            }
        }
    }

    //
    // Specify the surfaces from the first Mars layer. This layer
    // uses topocentric frames.
    //
    save.NSURF = (NTILEH * NTILEW);

    for I in 1..=save.NSURF {
        save.LAYSRF[[I, 1]] = I;
        save.LAYSRF[[I, 2]] = (I + save.NSURF);
        save.LAYSRF[[I, 3]] = (I + (save.NSURF * 2));
    }

    //
    // Create ray. R is a scale factor used to
    // control the magnitude of the vertex.
    //
    save.R = 1000000.0;

    //
    // Pick a vertex that doesn't lie on a meridian
    // containing any plate vertices.
    //
    spicelib::VPACK(
        (1.1 * save.R),
        -save.R,
        ((2 as f64) * save.R),
        save.VERTEX.as_slice_mut(),
    );

    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    save.BODYID = 499;

    save.ET = 0.0;

    //
    // Compute our intercept in the IAU_MARS frame.
    //
    save.BIX = 1;
    fstr::assign(&mut save.FIXREF, save.FIXRFS.get(save.BIX));
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We'll call ZZSBFXR rather than ZZDSKBUX since the former
    // calls the latter, and the former has all of the flexibility
    // we need, but has a less complex calling sequence.
    //
    spicelib::ZZSBFXR(
        save.BODYID,
        save.NSURF,
        save.LAYSRF.subarray([1, 1]),
        save.ET,
        save.FIXFID,
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.XPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND (layer 1)", save.FOUND, true, OK, ctx)?;

    if save.FOUND {
        //
        // Get the intercept on the second layer. This layer uses
        // the body-centered frame.
        //
        spicelib::ZZSBFXR(
            save.BODYID,
            save.NSURF,
            save.LAYSRF.subarray([1, 2]),
            save.ET,
            save.FIXFID,
            save.VERTEX.as_slice(),
            save.RAYDIR.as_slice(),
            save.XXPT.as_slice_mut(),
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND (layer 2)", save.FOUND, true, OK, ctx)?;

        save.TOL = TIGHT;

        //
        // Compare to the intercept on layer 1 found by ZZDSKXR.
        //
        testutil::CHCKAD(
            b"XPT",
            save.XPT.as_slice(),
            b"~~/",
            save.XXPT.as_slice(),
            3,
            save.TOL,
            OK,
            ctx,
        )?;

        //
        // Now use ZZSBFXRI to do the computation. Use the second layer.
        //
        spicelib::ZZSBFXRI(
            save.BODYID,
            save.NSURF,
            save.LAYSRF.subarray([1, 2]),
            save.ET,
            save.FIXFID,
            save.VERTEX.as_slice(),
            save.RAYDIR.as_slice(),
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

        testutil::CHCKSL(b"ZZSBFXRI FOUND (layer 2)", save.FOUND, true, OK, ctx)?;

        testutil::CHCKAD(
            b"ZZSBFXRI XPT",
            save.XPT.as_slice(),
            b"~~/",
            save.XXPT.as_slice(),
            3,
            save.TOL,
            OK,
            ctx,
        )?;

        //
        // Use ZZSBFXRI to find the intercept on the first layer.
        //
        spicelib::ZZSBFXRI(
            save.BODYID,
            save.NSURF,
            save.LAYSRF.subarray([1, 1]),
            save.ET,
            save.FIXFID,
            save.VERTEX.as_slice(),
            save.RAYDIR.as_slice(),
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

        testutil::CHCKSL(b"ZZSBFXRI FOUND (layer 2)", save.FOUND, true, OK, ctx)?;

        testutil::CHCKAD(
            b"ZZSBFXRI XPT",
            save.XPT.as_slice(),
            b"~~/",
            save.XXPT.as_slice(),
            3,
            save.TOL,
            OK,
            ctx,
        )?;

        //
        // Check the handle, DSK and DLA descriptors, and plate ID
        // from ZZSBFXRI.
        //
        spicelib::DSKGD(
            save.HANDLE,
            save.DLADSC.as_slice(),
            save.XDSKDS.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"DSKDSC",
            save.DSKDSC.as_slice(),
            b"=",
            save.XDSKDS.as_slice(),
            DSKDSZ,
            0.0,
            OK,
            ctx,
        )?;

        //
        // Confirm that we have the correct DLA descriptor by comparing
        // the ZZDSKXRI intercept to the one we find using DSKX02.
        //
        // In order to use DSKX02, we need to have a vertex and ray
        // direction expressed in the segment frame. The vertex must
        // be an offset relative to the frame center.
        //
        save.SEGFID = intrinsics::IDNINT(save.DSKDSC[FRMIDX]);

        if (save.SEGFID == save.FIXFID) {
            spicelib::VEQU(save.VERTEX.as_slice(), save.SEGVTX.as_slice_mut());
            spicelib::VEQU(save.RAYDIR.as_slice(), save.SEGDIR.as_slice_mut());
        } else {
            //
            // Get the frame center offset in the body-centered frame.
            //
            spicelib::FRINFO(
                save.SEGFID,
                &mut save.FRMCTR,
                &mut save.CLASS,
                &mut save.CLSSID,
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::SPKEZP(
                save.FRMCTR,
                save.ET,
                &save.FIXREF,
                b"NONE",
                save.BODYID,
                save.OFFSET.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;
            //
            // Translate the ray's vertex to make it relative to the
            // segment frame's center.
            //
            spicelib::VSUB(
                save.VERTEX.as_slice(),
                save.OFFSET.as_slice(),
                save.VTEMP.as_slice_mut(),
            );

            //
            // Transform the offset vertex and the direction vector to
            // the segment frame.
            //
            spicelib::REFCHG(
                save.FIXFID,
                save.SEGFID,
                save.ET,
                save.XFORM.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::MXV(
                save.XFORM.as_slice(),
                save.RAYDIR.as_slice(),
                save.SEGDIR.as_slice_mut(),
            );
            spicelib::MXV(
                save.XFORM.as_slice(),
                save.VTEMP.as_slice(),
                save.SEGVTX.as_slice_mut(),
            );
        }

        spicelib::DSKX02(
            save.HANDLE,
            save.DLADSC.as_slice(),
            save.SEGVTX.as_slice(),
            save.SEGDIR.as_slice(),
            &mut save.PLID,
            save.XXPT.as_slice_mut(),
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (save.SEGFID != save.FIXFID) {
            //
            // We must map the intercept back to the body-centered
            // frame.
            //
            spicelib::MTXV(
                save.XFORM.as_slice(),
                save.XXPT.as_slice(),
                save.VTEMP.as_slice_mut(),
            );
            spicelib::VADD(
                save.VTEMP.as_slice(),
                save.OFFSET.as_slice(),
                save.XXPT.as_slice_mut(),
            );
        }

        testutil::CHCKSL(b"DSKX02 FOUND", save.FOUND, true, OK, ctx)?;

        save.TOL = TIGHT;

        testutil::CHCKAD(
            b"ZZDSKXRI XPT (2)",
            save.XPT.as_slice(),
            b"~~/",
            save.XXPT.as_slice(),
            3,
            save.TOL,
            OK,
            ctx,
        )?;

        //
        // Check the plate ID from the integer parameter array returned
        // by ZZDSKXRI.
        //
        testutil::CHCKSI(b"IC(1)", save.IC[1], b"=", save.PLID, 0, OK, ctx)?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Simple Saturn intercept case. Use layers 1 and 2.", ctx)?;

    //
    // Create ray.
    //
    save.R = 1000000.0;

    spicelib::VPACK(
        save.R,
        -save.R,
        ((2 as f64) * save.R),
        save.VERTEX.as_slice_mut(),
    );

    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    //
    // Specify the surfaces from the first Saturn layer. This layer
    // uses topocentric frames.
    //
    save.NSURF = (NTILEH * NTILEW);

    for I in 1..=save.NSURF {
        save.LAYSRF[[I, 1]] = I;
        save.LAYSRF[[I, 2]] = (I + save.NSURF);
        save.LAYSRF[[I, 3]] = (I + (save.NSURF * 2));
    }

    save.BODYID = 699;

    save.ET = 0.0;

    //
    // Compute our intercept in the IAU_SATURN frame.
    //
    save.BIX = 2;
    fstr::assign(&mut save.FIXREF, save.FIXRFS.get(save.BIX));
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We'll call ZZSBFXR rather than ZZDSKBUX since the former
    // calls the latter, and the former has all of the flexibility
    // we need, but has a less complex calling sequence.
    //
    spicelib::ZZSBFXR(
        save.BODYID,
        save.NSURF,
        save.LAYSRF.subarray([1, 1]),
        save.ET,
        save.FIXFID,
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.XPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND (layer 1)", save.FOUND, true, OK, ctx)?;

    if save.FOUND {
        //
        // Get the intercept on the second layer. This layer uses
        // the body-centered frame.
        //
        spicelib::ZZSBFXR(
            save.BODYID,
            save.NSURF,
            save.LAYSRF.subarray([1, 2]),
            save.ET,
            save.FIXFID,
            save.VERTEX.as_slice(),
            save.RAYDIR.as_slice(),
            save.XXPT.as_slice_mut(),
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND (layer 2)", save.FOUND, true, OK, ctx)?;

        save.TOL = TIGHT;

        testutil::CHCKAD(
            b"XPT",
            save.XPT.as_slice(),
            b"~~/",
            save.XXPT.as_slice(),
            3,
            save.TOL,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Simple Mars intercept case. Ray\'s vertex is inside layers 1 and 2 but outside layer 3.",
        ctx,
    )?;

    save.BODYID = 499;

    //
    // Compute the expected intercept.
    //
    spicelib::ZZSBFXR(
        save.BODYID,
        save.NSURF,
        save.LAYSRF.subarray([1, 2]),
        save.ET,
        save.FIXFID,
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.XXPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND (layer 2)", save.FOUND, true, OK, ctx)?;

    //
    // Scale down the point on layer 2 to the scale of layer 3.
    //
    spicelib::VSCLIP(save.SCALE, save.XXPT.as_slice_mut());

    //
    // Create the scaled vertex.
    //
    save.R = 1000.0;

    spicelib::VPACK(
        save.R,
        -save.R,
        ((2 as f64) * save.R),
        save.VERTEX.as_slice_mut(),
    );

    spicelib::ZZSBFXR(
        save.BODYID,
        0,
        save.LAYSRF.as_slice(),
        save.ET,
        save.FIXFID,
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.XPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND (small vertex)", save.FOUND, true, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"XPT",
        save.XPT.as_slice(),
        b"~~/",
        save.XXPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Restore the default vertex.
    //
    save.R = 1000000.0;

    spicelib::VPACK(
        save.R,
        -save.R,
        ((2 as f64) * save.R),
        save.VERTEX.as_slice_mut(),
    );

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Simple Mars non-intercept case. Ray points away from the target. Use all layers.",
        ctx,
    )?;

    save.BODYID = 499;

    //
    // Point the ray away from the target.
    //
    spicelib::ZZSBFXR(
        save.BODYID,
        0,
        save.LAYSRF.as_slice(),
        save.ET,
        save.FIXFID,
        save.VERTEX.as_slice(),
        save.VERTEX.as_slice(),
        save.XPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND (all)", save.FOUND, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Simple Mars non-intercept case. Ray emanates from the target center. Use all layers.",
        ctx,
    )?;

    save.BODYID = 499;

    spicelib::CLEARD(3, save.VERTEX.as_slice_mut());

    //
    // Point the ray away from the target.
    //
    spicelib::ZZSBFXR(
        save.BODYID,
        0,
        save.LAYSRF.as_slice(),
        save.ET,
        save.FIXFID,
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.XPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND (all)", save.FOUND, false, OK, ctx)?;

    //
    // Restore the default vertex and ray direction.
    //
    save.R = 1000000.0;

    spicelib::VPACK(
        save.R,
        -save.R,
        ((2 as f64) * save.R),
        save.VERTEX.as_slice_mut(),
    );

    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Simple Mars non-intercept case. Time is too early for all segments. Use all layers.",
        ctx,
    )?;

    save.BODYID = 499;

    //
    // The first segment starts at 30 Julian years before J2000. The
    // other segments start later.
    //
    save.ET = -((35 as f64) * spicelib::JYEAR());

    spicelib::ZZSBFXR(
        save.BODYID,
        0,
        save.LAYSRF.as_slice(),
        save.ET,
        save.FIXFID,
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.XPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND (all)", save.FOUND, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Simple Mars non-intercept case. Time is too late for all segments. Use all layers.",
        ctx,
    )?;

    save.BODYID = 499;

    //
    // The last segment ends at 30 Julian years after J2000. The
    // other segments end later.
    //
    save.ET = ((35 as f64) * spicelib::JYEAR());

    spicelib::ZZSBFXR(
        save.BODYID,
        0,
        save.LAYSRF.as_slice(),
        save.ET,
        save.FIXFID,
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.XPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND (all)", save.FOUND, false, OK, ctx)?;

    //
    // Restore ET.
    //
    save.ET = 0.0;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Simple Saturn non-intercept case. The selected surfaces don\'t cover the potential intercept coordinates.", ctx)?;

    save.BODYID = 699;

    spicelib::VPACK(0.0, 0.0, -save.R, save.VERTEX.as_slice_mut());
    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZSBFXR(
        save.BODYID,
        1,
        &[1],
        save.ET,
        save.FIXFID,
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.XPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND (surface ID 1)", save.FOUND, false, OK, ctx)?;

    //
    // Restore the default vertex and ray direction.
    //
    save.R = 1000000.0;

    spicelib::VPACK(
        save.R,
        -save.R,
        ((2 as f64) * save.R),
        save.VERTEX.as_slice_mut(),
    );

    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Mars spear case. Compare results using layers 1 and 2.",
        ctx,
    )?;

    save.BODYID = 499;

    save.BIX = 1;
    fstr::assign(&mut save.FIXREF, save.FIXRFS.get(save.BIX));
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NLON = 80;
    save.NLAT = 40;

    save.DLON = (((2 as f64) * spicelib::PI(ctx)) / save.NLON as f64);
    save.DLAT = (spicelib::PI(ctx) / save.NLAT as f64);

    for I in 1..=save.NLON {
        save.LON = (((I - 1) as f64) * save.DLON);

        for J in 0..=save.NLAT {
            save.LAT = (spicelib::HALFPI(ctx) - ((J as f64) * save.DLAT));

            spicelib::LATREC(save.R, save.LON, save.LAT, save.VERTEX.as_slice_mut());
            spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

            spicelib::ZZSBFXR(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 1]),
                save.ET,
                save.FIXFID,
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
                save.XPT.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"FOUND (layer 1)", save.FOUND, true, OK, ctx)?;

            spicelib::ZZSBFXR(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 2]),
                save.ET,
                save.FIXFID,
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
                save.XXPT.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"FOUND (layer 2)", save.FOUND, true, OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"XPT I=@ J=@");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.XPT.as_slice(),
                b"~~/",
                save.XXPT.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Saturn spear case. Compare results using layers 1 and 2.",
        ctx,
    )?;

    save.BODYID = 699;

    save.BIX = 2;
    fstr::assign(&mut save.FIXREF, save.FIXRFS.get(save.BIX));
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NLON = 80;
    save.NLAT = 40;

    save.DLON = (((2 as f64) * spicelib::PI(ctx)) / save.NLON as f64);
    save.DLAT = (spicelib::PI(ctx) / save.NLAT as f64);

    for I in 1..=save.NLON {
        save.LON = (((I - 1) as f64) * save.DLON);

        for J in 0..=save.NLAT {
            save.LAT = (spicelib::HALFPI(ctx) - ((J as f64) * save.DLAT));

            spicelib::LATREC(save.R, save.LON, save.LAT, save.VERTEX.as_slice_mut());
            spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

            spicelib::ZZSBFXR(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 1]),
                save.ET,
                save.FIXFID,
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
                save.XPT.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"FOUND (layer 1)", save.FOUND, true, OK, ctx)?;

            spicelib::ZZSBFXR(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 2]),
                save.ET,
                save.FIXFID,
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
                save.XXPT.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"FOUND (layer 2)", save.FOUND, true, OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"XPT I=@ J=@");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.XPT.as_slice(),
                b"~~/",
                save.XXPT.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Mars spear case. Compare results using layers 1 and 2. This time, load DSK kernels for the different layers in reverse order.", ctx)?;

    save.BODYID = 499;
    save.BIX = 1;
    fstr::assign(&mut save.FIXREF, save.FIXRFS.get(save.BIX));
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for LAYRIX in intrinsics::range(3, 1, -1) {
        for W in 1..=NTILEW {
            for H in 1..=NTILEH {
                spicelib::FURNSH(&save.TILDSK[[H, W, LAYRIX, save.BIX]], ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
            }
        }
    }

    save.NLON = 80;
    save.NLAT = 40;

    save.DLON = (((2 as f64) * spicelib::PI(ctx)) / save.NLON as f64);
    save.DLAT = (spicelib::PI(ctx) / save.NLAT as f64);

    for I in 1..=save.NLON {
        save.LON = (((I - 1) as f64) * save.DLON);

        for J in 0..=save.NLAT {
            save.LAT = (spicelib::HALFPI(ctx) - ((J as f64) * save.DLAT));

            spicelib::LATREC(save.R, save.LON, save.LAT, save.VERTEX.as_slice_mut());
            spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

            spicelib::ZZSBFXR(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 1]),
                save.ET,
                save.FIXFID,
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
                save.XPT.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"FOUND (layer 1)", save.FOUND, true, OK, ctx)?;

            spicelib::ZZSBFXR(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 2]),
                save.ET,
                save.FIXFID,
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
                save.XXPT.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"FOUND (layer 2)", save.FOUND, true, OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"XPT I=@ J=@");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.XPT.as_slice(),
                b"~~/",
                save.XXPT.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Saturn spear case. Compare results using layers 1 and 2. This time, load DSK kernels for the different layers in reverse order.", ctx)?;

    save.BODYID = 699;
    save.BIX = 2;
    fstr::assign(&mut save.FIXREF, save.FIXRFS.get(save.BIX));
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for LAYRIX in intrinsics::range(3, 1, -1) {
        for W in 1..=NTILEW {
            for H in 1..=NTILEH {
                spicelib::FURNSH(&save.TILDSK[[H, W, LAYRIX, save.BIX]], ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
            }
        }
    }

    save.NLON = 80;
    save.NLAT = 40;

    save.DLON = (((2 as f64) * spicelib::PI(ctx)) / save.NLON as f64);
    save.DLAT = (spicelib::PI(ctx) / save.NLAT as f64);

    for I in 1..=save.NLON {
        save.LON = (((I - 1) as f64) * save.DLON);

        for J in 0..=save.NLAT {
            save.LAT = (spicelib::HALFPI(ctx) - ((J as f64) * save.DLAT));

            spicelib::LATREC(save.R, save.LON, save.LAT, save.VERTEX.as_slice_mut());
            spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

            spicelib::ZZSBFXR(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 1]),
                save.ET,
                save.FIXFID,
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
                save.XPT.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"FOUND (layer 1)", save.FOUND, true, OK, ctx)?;

            spicelib::ZZSBFXR(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 2]),
                save.ET,
                save.FIXFID,
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
                save.XXPT.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"FOUND (layer 2)", save.FOUND, true, OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"XPT I=@ J=@");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.XPT.as_slice(),
                b"~~/",
                save.XXPT.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Mars spear case. Compare results using layer 1 against results obtained using all layers.", ctx)?;

    save.BODYID = 499;
    save.BIX = 1;
    fstr::assign(&mut save.FIXREF, save.FIXRFS.get(save.BIX));
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NSURF = (NTILEW * NTILEH);

    //
    // Reduce the number of intercepts for this one.
    //
    save.NLON = 40;
    save.NLAT = 20;

    save.DLON = (((2 as f64) * spicelib::PI(ctx)) / save.NLON as f64);
    save.DLAT = (spicelib::PI(ctx) / save.NLAT as f64);

    for I in 1..=save.NLON {
        save.LON = (((I - 1) as f64) * save.DLON);

        for J in 0..=save.NLAT {
            save.LAT = (spicelib::HALFPI(ctx) - ((J as f64) * save.DLAT));

            spicelib::LATREC(save.R, save.LON, save.LAT, save.VERTEX.as_slice_mut());
            spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

            spicelib::ZZSBFXR(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 1]),
                save.ET,
                save.FIXFID,
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
                save.XPT.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"FOUND (layer 1)", save.FOUND, true, OK, ctx)?;

            spicelib::ZZSBFXR(
                save.BODYID,
                0,
                save.LAYSRF.as_slice(),
                save.ET,
                save.FIXFID,
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
                save.XXPT.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"FOUND (all layers)", save.FOUND, true, OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"XPT I=@ J=@");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.XPT.as_slice(),
                b"~~/",
                save.XXPT.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Saturn spear case. Compare results using layer 1 against results obtained using all layers.", ctx)?;

    save.BODYID = 699;
    save.BIX = 1;
    fstr::assign(&mut save.FIXREF, save.FIXRFS.get(save.BIX));
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NSURF = (NTILEW * NTILEH);

    //
    // Reduce the number of intercepts for this one.
    //
    save.NLON = 40;
    save.NLAT = 20;

    save.DLON = (((2 as f64) * spicelib::PI(ctx)) / save.NLON as f64);
    save.DLAT = (spicelib::PI(ctx) / save.NLAT as f64);

    for I in 1..=save.NLON {
        save.LON = (((I - 1) as f64) * save.DLON);

        for J in 0..=save.NLAT {
            save.LAT = (spicelib::HALFPI(ctx) - ((J as f64) * save.DLAT));

            spicelib::LATREC(save.R, save.LON, save.LAT, save.VERTEX.as_slice_mut());
            spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

            spicelib::ZZSBFXR(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 1]),
                save.ET,
                save.FIXFID,
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
                save.XPT.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"FOUND (layer 1)", save.FOUND, true, OK, ctx)?;

            spicelib::ZZSBFXR(
                save.BODYID,
                0,
                save.LAYSRF.as_slice(),
                save.ET,
                save.FIXFID,
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
                save.XXPT.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"FOUND (all layers)", save.FOUND, true, OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"XPT I=@ J=@");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.XPT.as_slice(),
                b"~~/",
                save.XXPT.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Mars greedy spear case. Deliberately hit segment boundaries on meridians and parallels.",
        ctx,
    )?;

    save.BODYID = 499;

    save.BIX = 1;
    fstr::assign(&mut save.FIXREF, save.FIXRFS.get(save.BIX));
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NLON = (NTILEW * 2);
    save.NLAT = (NTILEH * 2);

    save.DLON = (((2 as f64) * spicelib::PI(ctx)) / save.NLON as f64);
    save.DLAT = (spicelib::PI(ctx) / save.NLAT as f64);

    for I in 1..=save.NLON {
        save.LON = (((I - 1) as f64) * save.DLON);

        for J in 0..=save.NLAT {
            save.LAT = (spicelib::HALFPI(ctx) - ((J as f64) * save.DLAT));

            spicelib::LATREC(save.R, save.LON, save.LAT, save.VERTEX.as_slice_mut());
            spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

            spicelib::ZZSBFXR(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 1]),
                save.ET,
                save.FIXFID,
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
                save.XPT.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"FOUND (layer 1)", save.FOUND, true, OK, ctx)?;

            spicelib::ZZSBFXR(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 2]),
                save.ET,
                save.FIXFID,
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
                save.XXPT.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"FOUND (layer 2)", save.FOUND, true, OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"XPT I=@ J=@");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.XPT.as_slice(),
                b"~~/",
                save.XXPT.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Saturn greedy spear case. Deliberately hit segment boundaries on meridians and parallels.", ctx)?;

    save.BODYID = 699;

    save.BIX = 2;
    fstr::assign(&mut save.FIXREF, save.FIXRFS.get(save.BIX));
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NLON = (NTILEW * 2);
    save.NLAT = (NTILEH * 2);

    save.DLON = (((2 as f64) * spicelib::PI(ctx)) / save.NLON as f64);
    save.DLAT = (spicelib::PI(ctx) / save.NLAT as f64);

    for I in 1..=save.NLON {
        save.LON = (((I - 1) as f64) * save.DLON);

        for J in 0..=save.NLAT {
            save.LAT = (spicelib::HALFPI(ctx) - ((J as f64) * save.DLAT));

            spicelib::LATREC(save.R, save.LON, save.LAT, save.VERTEX.as_slice_mut());
            spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

            spicelib::ZZSBFXR(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 1]),
                save.ET,
                save.FIXFID,
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
                save.XPT.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"FOUND (layer 1)", save.FOUND, true, OK, ctx)?;

            spicelib::ZZSBFXR(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 2]),
                save.ET,
                save.FIXFID,
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
                save.XXPT.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"FOUND (layer 2)", save.FOUND, true, OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"XPT I=@ J=@");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.XPT.as_slice(),
                b"~~/",
                save.XXPT.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;
        }
    }

    //***********************************************************************
    //
    //     ZZDSKBUX Error cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Simple Saturn non-intercept case. No DSK data are available for the specified surfaces. Use all layers for Mars and Saturn. This is an error case.", ctx)?;

    save.BODYID = 699;

    spicelib::ZZSBFXR(
        save.BODYID,
        1,
        &[-1],
        save.ET,
        save.FIXFID,
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.XPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DSKDATANOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Simple Venus non-intercept case. No DSK data are available for Venus. Use all layers for Mars and Saturn. This is an error case.", ctx)?;

    save.BODYID = 299;

    spicelib::ZZSBFXR(
        save.BODYID,
        0,
        save.LAYSRF.as_slice(),
        save.ET,
        save.FIXFID,
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.XPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NODSKSEGMENTS)", OK, ctx)?;

    //***********************************************************************
    //
    //
    //
    //     ZZDSKBUN, ZZSBFNRM tests
    //
    //
    //
    //***********************************************************************

    //***********************************************************************
    //
    //     ZZDSKBUN, ZZSBFNRM Normal cases
    //
    //***********************************************************************

    //
    // We'll call ZZSBFNRM rather than ZZDSKBUN since the former
    // calls the latter, and the former has all of the flexibility
    // we need, but has a less complex calling sequence.

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Mars case: find outward normals on every plate of layer 1. Compare to normals on layer 2. Choose points so as to avoid plate edges.", ctx)?;

    //
    // Re-load DSKS in their original order.
    //

    for TARGIX in 1..=NTARG {
        for LAYRIX in 1..=NLAYER {
            for W in 1..=NTILEW {
                for H in 1..=NTILEH {
                    spicelib::FURNSH(&save.TILDSK[[H, W, LAYRIX, TARGIX]], ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }
            }
        }
    }

    save.BODYID = 499;

    save.BIX = 1;
    fstr::assign(&mut save.FIXREF, save.FIXRFS.get(save.BIX));
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NLON = (NTILEW * 2);
    save.NLAT = (NTILEH * 2);

    save.DLON = (((2 as f64) * spicelib::PI(ctx)) / save.NLON as f64);
    save.DLAT = (spicelib::PI(ctx) / save.NLAT as f64);

    save.R = 1000000.0;

    save.ET = 0.0;

    for I in 1..=save.NLON {
        save.LON = ((((I - 1) as f64) + (1.0 / 3 as f64)) * save.DLON);

        for J in 1..=save.NLAT {
            save.LAT = (spicelib::HALFPI(ctx) - (((J as f64) - 0.5) * save.DLAT));

            spicelib::LATREC(save.R, save.LON, save.LAT, save.VERTEX.as_slice_mut());
            spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());
            //
            // Generate a surface point on the second layer. This
            // point should be very close to the first layer as well.
            //
            spicelib::ZZSBFXRI(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 2]),
                save.ET,
                save.FIXFID,
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
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

            testutil::CHCKSL(b"FOUND (layer 2)", save.FOUND, true, OK, ctx)?;

            //
            // Find the outward normal at XPT, using the first layer.
            //
            spicelib::ZZSBFNRM(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 1]),
                save.ET,
                save.FIXFID,
                save.XPT.as_slice(),
                save.NORMAL.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Find the outward normal at XPT using the second layer.
            //
            spicelib::ZZSBFNRM(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 2]),
                save.ET,
                save.FIXFID,
                save.XPT.as_slice(),
                save.XNORML.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"layer 1 NORMAL I=@ J=@");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.NORMAL.as_slice(),
                b"~~/",
                save.XNORML.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;

            //
            // Fetch the plate normal directly from the segment found
            // by ZZDSKXRI.
            //
            spicelib::DSKN02(
                save.HANDLE,
                save.DLADSC.as_slice(),
                save.IC[1],
                save.NORMAL.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"DSKN02 NORMAL I=@ J=@");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.NORMAL.as_slice(),
                b"~~/",
                save.XNORML.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Saturn case: find outward normals on every plate of layer 1. Compare to normals on layer 2. Choose points so as to avoid plate edges.", ctx)?;

    save.BODYID = 699;

    save.BIX = 2;
    fstr::assign(&mut save.FIXREF, save.FIXRFS.get(save.BIX));
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NLON = (NTILEW * 2);
    save.NLAT = (NTILEH * 2);

    save.DLON = (((2 as f64) * spicelib::PI(ctx)) / save.NLON as f64);
    save.DLAT = (spicelib::PI(ctx) / save.NLAT as f64);

    save.R = 1000000.0;

    save.ET = 0.0;

    for I in 1..=save.NLON {
        save.LON = ((((I - 1) as f64) + (1.0 / 3 as f64)) * save.DLON);

        for J in 1..=save.NLAT {
            save.LAT = (spicelib::HALFPI(ctx) - (((J as f64) - 0.5) * save.DLAT));

            spicelib::LATREC(save.R, save.LON, save.LAT, save.VERTEX.as_slice_mut());
            spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());
            //
            // Generate a surface point on the second layer. This
            // point should be very close to the first layer as well.
            //
            spicelib::ZZSBFXRI(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 2]),
                save.ET,
                save.FIXFID,
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
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

            testutil::CHCKSL(b"FOUND (layer 2)", save.FOUND, true, OK, ctx)?;

            //
            // Find the outward normal at XPT, using the first layer.
            //
            spicelib::ZZSBFNRM(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 1]),
                save.ET,
                save.FIXFID,
                save.XPT.as_slice(),
                save.NORMAL.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Find the outward normal at XPT using the second layer.
            //
            spicelib::ZZSBFNRM(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 2]),
                save.ET,
                save.FIXFID,
                save.XPT.as_slice(),
                save.XNORML.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"layer 1 NORMAL I=@ J=@");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.NORMAL.as_slice(),
                b"~~/",
                save.XNORML.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;

            //
            // Fetch the plate normal directly from the segment found
            // by ZZDSKXRI.
            //
            spicelib::DSKN02(
                save.HANDLE,
                save.DLADSC.as_slice(),
                save.IC[1],
                save.NORMAL.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"DSKN02 NORMAL I=@ J=@");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.NORMAL.as_slice(),
                b"~~/",
                save.XNORML.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Mars case: find outward normals on every plate, using all layers. Compare to normals on layer 2. Choose points so as to avoid plate edges.", ctx)?;

    save.BODYID = 499;

    save.BIX = 1;
    fstr::assign(&mut save.FIXREF, save.FIXRFS.get(save.BIX));
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NLON = (NTILEW * 2);
    save.NLAT = (NTILEH * 2);

    save.DLON = (((2 as f64) * spicelib::PI(ctx)) / save.NLON as f64);
    save.DLAT = (spicelib::PI(ctx) / save.NLAT as f64);

    save.R = 1000000.0;

    save.ET = 0.0;

    for I in 1..=save.NLON {
        save.LON = ((((I - 1) as f64) + (1.0 / 3 as f64)) * save.DLON);

        for J in 1..=save.NLAT {
            save.LAT = (spicelib::HALFPI(ctx) - (((J as f64) - 0.5) * save.DLAT));

            spicelib::LATREC(save.R, save.LON, save.LAT, save.VERTEX.as_slice_mut());
            spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());
            //
            // Generate a surface point on the second layer. This
            // point should be very close to the first layer as well.
            //
            spicelib::ZZSBFXRI(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 2]),
                save.ET,
                save.FIXFID,
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
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

            testutil::CHCKSL(b"FOUND (layer 2)", save.FOUND, true, OK, ctx)?;

            //
            // Find the outward normal at XPT, using all layers.
            //
            spicelib::ZZSBFNRM(
                save.BODYID,
                0,
                save.LAYSRF.as_slice(),
                save.ET,
                save.FIXFID,
                save.XPT.as_slice(),
                save.NORMAL.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Find the outward normal at XPT using the second layer.
            //
            spicelib::ZZSBFNRM(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 2]),
                save.ET,
                save.FIXFID,
                save.XPT.as_slice(),
                save.XNORML.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"layer 1 NORMAL I=@ J=@");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.NORMAL.as_slice(),
                b"~~/",
                save.XNORML.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;

            //
            // Fetch the plate normal directly from the segment found
            // by ZZDSKXRI.
            //
            spicelib::DSKN02(
                save.HANDLE,
                save.DLADSC.as_slice(),
                save.IC[1],
                save.NORMAL.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"DSKN02 NORMAL I=@ J=@");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.NORMAL.as_slice(),
                b"~~/",
                save.XNORML.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Saturn case: find outward normals on every plate, using all layers. Compare to normals on layer 2. Choose points so as to avoid plate edges.", ctx)?;

    save.BODYID = 699;

    save.BIX = 2;
    fstr::assign(&mut save.FIXREF, save.FIXRFS.get(save.BIX));
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NLON = (NTILEW * 2);
    save.NLAT = (NTILEH * 2);

    save.DLON = (((2 as f64) * spicelib::PI(ctx)) / save.NLON as f64);
    save.DLAT = (spicelib::PI(ctx) / save.NLAT as f64);

    save.R = 1000000.0;

    save.ET = 0.0;

    for I in 1..=save.NLON {
        save.LON = ((((I - 1) as f64) + (1.0 / 3 as f64)) * save.DLON);

        for J in 1..=save.NLAT {
            save.LAT = (spicelib::HALFPI(ctx) - (((J as f64) - 0.5) * save.DLAT));

            spicelib::LATREC(save.R, save.LON, save.LAT, save.VERTEX.as_slice_mut());
            spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());
            //
            // Generate a surface point on the second layer. This
            // point should be very close to the first layer as well.
            //
            spicelib::ZZSBFXRI(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 2]),
                save.ET,
                save.FIXFID,
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
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

            testutil::CHCKSL(b"FOUND (layer 2)", save.FOUND, true, OK, ctx)?;

            //
            // Find the outward normal at XPT, using all layers.
            //
            spicelib::ZZSBFNRM(
                save.BODYID,
                0,
                save.LAYSRF.as_slice(),
                save.ET,
                save.FIXFID,
                save.XPT.as_slice(),
                save.NORMAL.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Find the outward normal at XPT using the second layer.
            //
            spicelib::ZZSBFNRM(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 2]),
                save.ET,
                save.FIXFID,
                save.XPT.as_slice(),
                save.XNORML.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"layer 1 NORMAL I=@ J=@");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.NORMAL.as_slice(),
                b"~~/",
                save.XNORML.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;

            //
            // Fetch the plate normal directly from the segment found
            // by ZZDSKXRI.
            //
            spicelib::DSKN02(
                save.HANDLE,
                save.DLADSC.as_slice(),
                save.IC[1],
                save.NORMAL.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"DSKN02 NORMAL I=@ J=@");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.NORMAL.as_slice(),
                b"~~/",
                save.XNORML.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Mars case: find outward normals on every plate, using all layers. Compare to normals on layer 2. Choose points so as to avoid plate edges. Load DSKs so as to reverse the order of the layers.", ctx)?;

    //
    // Re-load DSKs, reversing the order of the layers.
    //
    for TARGIX in 1..=NTARG {
        for LAYRIX in intrinsics::range(NLAYER, 1, -1) {
            for W in 1..=NTILEW {
                for H in 1..=NTILEH {
                    spicelib::FURNSH(&save.TILDSK[[H, W, LAYRIX, TARGIX]], ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }
            }
        }
    }

    save.BODYID = 499;

    save.BIX = 1;
    fstr::assign(&mut save.FIXREF, save.FIXRFS.get(save.BIX));
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NLON = (NTILEW * 2);
    save.NLAT = (NTILEH * 2);

    save.DLON = (((2 as f64) * spicelib::PI(ctx)) / save.NLON as f64);
    save.DLAT = (spicelib::PI(ctx) / save.NLAT as f64);

    save.R = 1000000.0;

    save.ET = 0.0;

    for I in 1..=save.NLON {
        save.LON = ((((I - 1) as f64) + (1.0 / 3 as f64)) * save.DLON);

        for J in 1..=save.NLAT {
            save.LAT = (spicelib::HALFPI(ctx) - (((J as f64) - 0.5) * save.DLAT));

            spicelib::LATREC(save.R, save.LON, save.LAT, save.VERTEX.as_slice_mut());
            spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());
            //
            // Generate a surface point on the second layer. This
            // point should be very close to the first layer as well.
            //
            spicelib::ZZSBFXRI(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 2]),
                save.ET,
                save.FIXFID,
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
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

            testutil::CHCKSL(b"FOUND (layer 2)", save.FOUND, true, OK, ctx)?;

            //
            // Reload the last DSK loaded. This looks strange, but we
            // need to do this here to exercise the BSR update logic
            // in ZZDSKNRM. Otherwise, the buffer updates will be
            // handled in ZZDSKXRI before ZZDSKSNRM gets a chance to
            // respond.
            //
            spicelib::FURNSH(&save.TILDSK[[NTILEH, NTILEW, 1, NTARG]], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Find the outward normal at XPT, using all layers.
            //
            spicelib::ZZSBFNRM(
                save.BODYID,
                0,
                save.LAYSRF.as_slice(),
                save.ET,
                save.FIXFID,
                save.XPT.as_slice(),
                save.NORMAL.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Find the outward normal at XPT using the second layer.
            //
            spicelib::ZZSBFNRM(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 2]),
                save.ET,
                save.FIXFID,
                save.XPT.as_slice(),
                save.XNORML.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"layer 1 NORMAL I=@ J=@");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.NORMAL.as_slice(),
                b"~~/",
                save.XNORML.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;

            //
            // Fetch the plate normal directly from the segment found
            // by ZZDSKXRI.
            //
            spicelib::DSKN02(
                save.HANDLE,
                save.DLADSC.as_slice(),
                save.IC[1],
                save.NORMAL.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"DSKN02 NORMAL I=@ J=@");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.NORMAL.as_slice(),
                b"~~/",
                save.XNORML.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Saturn case: find outward normals on every plate, using all layers. Compare to normals on layer 2. Choose points so as to avoid plate edges. DSKs have been reloaded so as to reverse the order of the layers.", ctx)?;

    //
    // The reload was performed in the previous case.
    //
    save.BODYID = 699;

    save.BIX = 2;
    fstr::assign(&mut save.FIXREF, save.FIXRFS.get(save.BIX));
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NLON = (NTILEW * 2);
    save.NLAT = (NTILEH * 2);

    save.DLON = (((2 as f64) * spicelib::PI(ctx)) / save.NLON as f64);
    save.DLAT = (spicelib::PI(ctx) / save.NLAT as f64);

    save.R = 1000000.0;

    save.ET = 0.0;

    for I in 1..=save.NLON {
        save.LON = ((((I - 1) as f64) + (1.0 / 3 as f64)) * save.DLON);

        for J in 1..=save.NLAT {
            save.LAT = (spicelib::HALFPI(ctx) - (((J as f64) - 0.5) * save.DLAT));

            spicelib::LATREC(save.R, save.LON, save.LAT, save.VERTEX.as_slice_mut());
            spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());
            //
            // Generate a surface point on the second layer. This
            // point should be very close to the first layer as well.
            //
            spicelib::ZZSBFXRI(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 2]),
                save.ET,
                save.FIXFID,
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
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

            testutil::CHCKSL(b"FOUND (layer 2)", save.FOUND, true, OK, ctx)?;

            //
            // Reload the last DSK loaded. This looks strange, but we
            // need to do this here to exercise the BSR update logic
            // in ZZDSKNRM. Otherwise, the buffer updates will be
            // handled in ZZDSKXRI before ZZDSKSNRM gets a chance to
            // respond.
            //
            spicelib::FURNSH(&save.TILDSK[[NTILEH, NTILEW, 1, NTARG]], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Find the outward normal at XPT, using all layers.
            //
            spicelib::ZZSBFNRM(
                save.BODYID,
                0,
                save.LAYSRF.as_slice(),
                save.ET,
                save.FIXFID,
                save.XPT.as_slice(),
                save.NORMAL.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Find the outward normal at XPT using the second layer.
            //
            spicelib::ZZSBFNRM(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 2]),
                save.ET,
                save.FIXFID,
                save.XPT.as_slice(),
                save.XNORML.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"layer 1 NORMAL I=@ J=@");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.NORMAL.as_slice(),
                b"~~/",
                save.XNORML.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;

            //
            // Fetch the plate normal directly from the segment found
            // by ZZDSKXRI.
            //
            spicelib::DSKN02(
                save.HANDLE,
                save.DLADSC.as_slice(),
                save.IC[1],
                save.NORMAL.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"DSKN02 NORMAL I=@ J=@");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.NORMAL.as_slice(),
                b"~~/",
                save.XNORML.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Mars case: find outward normals on every plate of layer 1. Compare to normals on layer 3. Choose points so as to avoid plate edges.", ctx)?;

    save.BODYID = 499;

    save.BIX = 1;
    fstr::assign(&mut save.FIXREF, save.FIXRFS.get(save.BIX));
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NLON = (NTILEW * 2);
    save.NLAT = (NTILEH * 2);

    save.DLON = (((2 as f64) * spicelib::PI(ctx)) / save.NLON as f64);
    save.DLAT = (spicelib::PI(ctx) / save.NLAT as f64);

    save.R = 1000000.0;

    save.ET = 0.0;

    for I in 1..=save.NLON {
        save.LON = ((((I - 1) as f64) + (1.0 / 3 as f64)) * save.DLON);

        for J in 1..=save.NLAT {
            save.LAT = (spicelib::HALFPI(ctx) - (((J as f64) - 0.5) * save.DLAT));

            spicelib::LATREC(save.R, save.LON, save.LAT, save.VERTEX.as_slice_mut());
            spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());
            //
            // Generate a surface point on the second layer. This
            // point should be very close to the first layer as well.
            //
            spicelib::ZZSBFXRI(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 2]),
                save.ET,
                save.FIXFID,
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
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

            testutil::CHCKSL(b"FOUND (layer 2)", save.FOUND, true, OK, ctx)?;

            //
            // Find the outward normal at XPT, using the first layer.
            //
            spicelib::ZZSBFNRM(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 1]),
                save.ET,
                save.FIXFID,
                save.XPT.as_slice(),
                save.NORMAL.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Find the outward normal at the lon/lat of XPT
            // using the third layer.
            //
            // We need to use a scaled version of XPT for this layer.
            //
            spicelib::VSCL(save.SCALE, save.XPT.as_slice(), save.VTEMP.as_slice_mut());

            spicelib::ZZSBFNRM(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 3]),
                save.ET,
                save.FIXFID,
                save.VTEMP.as_slice(),
                save.XNORML.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"layer 3 NORMAL I=@ J=@");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.NORMAL.as_slice(),
                b"~~/",
                save.XNORML.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;

            //
            // Fetch the plate normal directly from the segment found
            // by ZZDSKXRI.
            //
            spicelib::DSKN02(
                save.HANDLE,
                save.DLADSC.as_slice(),
                save.IC[1],
                save.NORMAL.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"DSKN02 NORMAL I=@ J=@");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.NORMAL.as_slice(),
                b"~~/",
                save.XNORML.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Saturn case: find outward normals on every plate of layer 1. Compare to normals on layer 3. Choose points so as to avoid plate edges.", ctx)?;

    save.BODYID = 699;

    save.BIX = 2;
    fstr::assign(&mut save.FIXREF, save.FIXRFS.get(save.BIX));
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NLON = (NTILEW * 2);
    save.NLAT = (NTILEH * 2);

    save.DLON = (((2 as f64) * spicelib::PI(ctx)) / save.NLON as f64);
    save.DLAT = (spicelib::PI(ctx) / save.NLAT as f64);

    save.R = 1000000.0;

    save.ET = 0.0;

    for I in 1..=save.NLON {
        save.LON = ((((I - 1) as f64) + (1.0 / 3 as f64)) * save.DLON);

        for J in 1..=save.NLAT {
            save.LAT = (spicelib::HALFPI(ctx) - (((J as f64) - 0.5) * save.DLAT));

            spicelib::LATREC(save.R, save.LON, save.LAT, save.VERTEX.as_slice_mut());
            spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());
            //
            // Generate a surface point on the second layer. This
            // point should be very close to the first layer as well.
            //
            spicelib::ZZSBFXRI(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 2]),
                save.ET,
                save.FIXFID,
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
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

            testutil::CHCKSL(b"FOUND (layer 2)", save.FOUND, true, OK, ctx)?;

            //
            // Find the outward normal at XPT, using the first layer.
            //
            spicelib::ZZSBFNRM(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 1]),
                save.ET,
                save.FIXFID,
                save.XPT.as_slice(),
                save.NORMAL.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Find the outward normal at the lon/lat of XPT
            // using the third layer.
            //
            // We need to use a scaled version of XPT for this layer.
            //
            spicelib::VSCL(save.SCALE, save.XPT.as_slice(), save.VTEMP.as_slice_mut());

            spicelib::ZZSBFNRM(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 3]),
                save.ET,
                save.FIXFID,
                save.VTEMP.as_slice(),
                save.XNORML.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"layer 3 NORMAL I=@ J=@");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.NORMAL.as_slice(),
                b"~~/",
                save.XNORML.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;

            //
            // Fetch the plate normal directly from the segment found
            // by ZZDSKXRI.
            //
            spicelib::DSKN02(
                save.HANDLE,
                save.DLADSC.as_slice(),
                save.IC[1],
                save.NORMAL.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"DSKN02 NORMAL I=@ J=@");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.NORMAL.as_slice(),
                b"~~/",
                save.XNORML.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Mars case: find outward normals on every plate of layer 1. Compare to normals on layer 3. Choose points so as to avoid plate edges. Select layers by time rather than surface ID.", ctx)?;

    save.BODYID = 499;

    save.BIX = 1;
    fstr::assign(&mut save.FIXREF, save.FIXRFS.get(save.BIX));
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NLON = (NTILEW * 2);
    save.NLAT = (NTILEH * 2);

    save.DLON = (((2 as f64) * spicelib::PI(ctx)) / save.NLON as f64);
    save.DLAT = (spicelib::PI(ctx) / save.NLAT as f64);

    save.R = 1000000.0;

    save.ET = 0.0;

    for I in 1..=save.NLON {
        save.LON = ((((I - 1) as f64) + (1.0 / 3 as f64)) * save.DLON);

        for J in 1..=save.NLAT {
            save.LAT = (spicelib::HALFPI(ctx) - (((J as f64) - 0.5) * save.DLAT));

            spicelib::LATREC(save.R, save.LON, save.LAT, save.VERTEX.as_slice_mut());
            spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());
            //
            // Generate a surface point on the second layer. This
            // point should be very close to the first layer as well.
            //
            // Set ET for this layer.
            //
            save.ET = 0.0;

            spicelib::ZZSBFXRI(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 2]),
                save.ET,
                save.FIXFID,
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
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

            testutil::CHCKSL(b"FOUND (layer 2)", save.FOUND, true, OK, ctx)?;

            //
            // Find the outward normal at XPT, using the first layer. Set
            // ET to a value covered only by the first layer.
            //
            save.ET = -((25 as f64) * spicelib::JYEAR());

            spicelib::ZZSBFNRM(
                save.BODYID,
                0,
                save.LAYSRF.as_slice(),
                save.ET,
                save.FIXFID,
                save.XPT.as_slice(),
                save.NORMAL.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Find the outward normal at the lon/lat of XPT
            // using the third layer.
            //
            // We need to use a scaled version of XPT for this layer.
            //
            spicelib::VSCL(save.SCALE, save.XPT.as_slice(), save.VTEMP.as_slice_mut());

            save.ET = ((25 as f64) * spicelib::JYEAR());

            spicelib::ZZSBFNRM(
                save.BODYID,
                0,
                save.LAYSRF.as_slice(),
                save.ET,
                save.FIXFID,
                save.VTEMP.as_slice(),
                save.XNORML.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"layer 3 NORMAL I=@ J=@");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.NORMAL.as_slice(),
                b"~~/",
                save.XNORML.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;

            //
            // Fetch the plate normal directly from the segment found
            // by ZZDSKXRI.
            //
            spicelib::DSKN02(
                save.HANDLE,
                save.DLADSC.as_slice(),
                save.IC[1],
                save.NORMAL.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"DSKN02 NORMAL I=@ J=@");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.NORMAL.as_slice(),
                b"~~/",
                save.XNORML.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Saturn case: find outward normals on every plate of layer 1. Compare to normals on layer 3. Choose points so as to avoid plate edges. Select layers by time rather than surface ID.", ctx)?;

    save.BODYID = 699;

    save.BIX = 2;
    fstr::assign(&mut save.FIXREF, save.FIXRFS.get(save.BIX));
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NLON = (NTILEW * 2);
    save.NLAT = (NTILEH * 2);

    save.DLON = (((2 as f64) * spicelib::PI(ctx)) / save.NLON as f64);
    save.DLAT = (spicelib::PI(ctx) / save.NLAT as f64);

    save.R = 1000000.0;

    save.ET = 0.0;

    for I in 1..=save.NLON {
        save.LON = ((((I - 1) as f64) + (1.0 / 3 as f64)) * save.DLON);

        for J in 1..=save.NLAT {
            save.LAT = (spicelib::HALFPI(ctx) - (((J as f64) - 0.5) * save.DLAT));

            spicelib::LATREC(save.R, save.LON, save.LAT, save.VERTEX.as_slice_mut());
            spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());
            //
            // Generate a surface point on the second layer. This
            // point should be very close to the first layer as well.
            //
            // Set ET for this layer.
            //
            save.ET = 0.0;

            spicelib::ZZSBFXRI(
                save.BODYID,
                save.NSURF,
                save.LAYSRF.subarray([1, 2]),
                save.ET,
                save.FIXFID,
                save.VERTEX.as_slice(),
                save.RAYDIR.as_slice(),
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

            testutil::CHCKSL(b"FOUND (layer 2)", save.FOUND, true, OK, ctx)?;

            //
            // Find the outward normal at XPT, using the first layer. Set
            // ET to a value covered only by the first layer.
            //
            save.ET = -((25 as f64) * spicelib::JYEAR());

            spicelib::ZZSBFNRM(
                save.BODYID,
                0,
                save.LAYSRF.as_slice(),
                save.ET,
                save.FIXFID,
                save.XPT.as_slice(),
                save.NORMAL.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Find the outward normal at the lon/lat of XPT
            // using the third layer.
            //
            // We need to use a scaled version of XPT for this layer.
            //
            spicelib::VSCL(save.SCALE, save.XPT.as_slice(), save.VTEMP.as_slice_mut());

            save.ET = ((25 as f64) * spicelib::JYEAR());

            spicelib::ZZSBFNRM(
                save.BODYID,
                0,
                save.LAYSRF.as_slice(),
                save.ET,
                save.FIXFID,
                save.VTEMP.as_slice(),
                save.XNORML.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"layer 3 NORMAL I=@ J=@");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.NORMAL.as_slice(),
                b"~~/",
                save.XNORML.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;

            //
            // Fetch the plate normal directly from the segment found
            // by ZZDSKXRI.
            //
            spicelib::DSKN02(
                save.HANDLE,
                save.DLADSC.as_slice(),
                save.IC[1],
                save.NORMAL.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.TOL = TIGHT;

            fstr::assign(&mut save.LABEL, b"DSKN02 NORMAL I=@ J=@");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", J, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.LABEL,
                save.NORMAL.as_slice(),
                b"~~/",
                save.XNORML.as_slice(),
                3,
                save.TOL,
                OK,
                ctx,
            )?;
        }
    }

    //***********************************************************************
    //
    //     ZZDSKBUN Error cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Mars error case: non-existent surface. Use all layers for Mars and Saturn.",
        ctx,
    )?;

    save.ET = 0.0;

    //
    // Generate a legitimate surface point.
    //
    save.BODYID = 499;

    save.BIX = 1;
    fstr::assign(&mut save.FIXREF, save.FIXRFS.get(save.BIX));
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.R = 1000000.0;
    spicelib::VPACK(
        save.R,
        -save.R,
        ((2 as f64) * save.R),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZSBFXRI(
        save.BODYID,
        0,
        save.LAYSRF.as_slice(),
        save.ET,
        save.FIXFID,
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
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

    spicelib::ZZSBFNRM(
        save.BODYID,
        1,
        &[-1],
        save.ET,
        save.FIXFID,
        save.XPT.as_slice(),
        save.XNORML.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(POINTNOTINSEGMENT)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Mars error case: point outside all segments.", ctx)?;

    save.R = 1000000.0;

    spicelib::VPACK(save.R, save.R, save.R, save.VTEMP.as_slice_mut());

    spicelib::ZZSBFNRM(
        save.BODYID,
        0,
        save.LAYSRF.as_slice(),
        save.ET,
        save.FIXFID,
        save.VTEMP.as_slice(),
        save.XNORML.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(POINTNOTINSEGMENT)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Mars error case: point inside segment, but too far from any plate.",
        ctx,
    )?;

    //
    // Start by generating a legitimate point.
    //
    save.BODYID = 499;
    save.BIX = 1;
    fstr::assign(&mut save.FIXREF, save.FIXRFS.get(save.BIX));
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.R = 1000000.0;

    save.ET = 0.0;

    spicelib::VPACK(
        save.R,
        -save.R,
        ((2 as f64) * save.R),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZSBFXRI(
        save.BODYID,
        0,
        save.LAYSRF.as_slice(),
        save.ET,
        save.FIXFID,
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
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
    // Get the normal at this point.
    //
    spicelib::ZZSBFNRM(
        save.BODYID,
        0,
        save.LAYSRF.as_slice(),
        save.ET,
        save.FIXFID,
        save.XPT.as_slice(),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Add to the point a small multiple of the normal.
    //
    spicelib::VLCOM(
        1.0,
        save.XPT.as_slice(),
        0.001,
        save.NORMAL.as_slice(),
        save.VTEMP.as_slice_mut(),
    );

    //
    // Now try to find the normal at the offset point.
    //
    spicelib::ZZSBFNRM(
        save.BODYID,
        0,
        save.LAYSRF.as_slice(),
        save.ET,
        save.FIXFID,
        save.VTEMP.as_slice(),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(POINTOFFSURFACE)", OK, ctx)?;

    //***********************************************************************
    //
    //     Clean up.
    //
    //***********************************************************************

    //
    // ---------------------------------------------------------
    //
    testutil::TCASE(b"Clean up", ctx)?;

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(PCK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(TOPFK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(TOPFK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(TOPSP0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(TOPSP1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NTARG;
        let m3__: i32 = 1;
        save.BIX = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            for LAYRIX in 1..=NLAYER {
                for W in 1..=NTILEW {
                    for H in 1..=NTILEH {
                        spicelib::DELFIL(&save.TILDSK[[H, W, LAYRIX, save.BIX]], ctx)?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    }
                }
            }

            save.BIX += m3__;
        }
    }

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
