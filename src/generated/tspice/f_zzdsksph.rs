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
const MAXSRF: i32 = 100;
const DSK0: &[u8] = b"zzdsksph_test0.bds";
const DSK1: &[u8] = b"zzdsksph_test1.bds";
const PCK0: &[u8] = b"zzdsksph_test0.tpc";
const TOPFK: &[u8] = b"zzdsksph_topo.tf";
const TOPSPK: &[u8] = b"zzdsksph_topo.bsp";
const TIGHT: f64 = 0.00000000001;
const VTIGHT: f64 = 0.0000000000001;
const LNSIZE: i32 = 255;
const NAMLEN: i32 = 32;
const NSITES: i32 = 3;
const MAXN: i32 = 6;
const MAXV: i32 = 5000;
const MAXP: i32 = (2 * MAXV);

struct SaveVars {
    FIXREF: Vec<u8>,
    FRAMES: ActualCharArray,
    SITFNM: ActualCharArray,
    SITNMS: ActualCharArray,
    TARGET: Vec<u8>,
    TOPREF: Vec<u8>,
    A: f64,
    ANGLES: StackArray2D<f64, 9>,
    B: f64,
    BOUNDS: StackArray2D<f64, 6>,
    C: f64,
    CORPAR: StackArray<f64, 10>,
    DIST: f64,
    DSKDSC: StackArray<f64, 24>,
    F: f64,
    FIRST: f64,
    LAST: f64,
    LAT: f64,
    LON: f64,
    LT: f64,
    MAXLAT: f64,
    MAXLON: f64,
    MAXRAD: f64,
    MINLAT: f64,
    MINLON: f64,
    MINRAD: f64,
    OFFMAG: f64,
    OFFSET: StackArray<f64, 3>,
    ORIGIN: StackArray<f64, 3>,
    PNEAR: StackArray<f64, 3>,
    RADII: StackArray<f64, 3>,
    RE: f64,
    RP: f64,
    SGMAXR: f64,
    SGMINR: f64,
    SITPOS: StackArray2D<f64, 9>,
    TOL: f64,
    VERTS: StackArray2D<f64, 9>,
    VRTCES: ActualArray2D<f64>,
    VTEMP: StackArray<f64, 3>,
    XFORM: StackArray2D<f64, 9>,
    XMAXR: f64,
    XMINR: f64,
    AXES: StackArray2D<i32, 9>,
    BODYID: i32,
    CORSYS: i32,
    DLADSC: StackArray<i32, 8>,
    HANDLE: i32,
    K: i32,
    N: i32,
    NLAT: i32,
    NLON: i32,
    NP: i32,
    NSURF: i32,
    NV: i32,
    NXTDSC: StackArray<i32, 8>,
    PLATE: StackArray<i32, 3>,
    PLATES: ActualArray2D<i32>,
    SITFID: StackArray<i32, 3>,
    SITIDS: StackArray<i32, 3>,
    SURFID: i32,
    SRFLST: StackArray<i32, 100>,
    FOUND: bool,
    MAKVTL: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FIXREF = vec![b' '; NAMLEN as usize];
        let mut FRAMES = ActualCharArray::new(NAMLEN, 1..=MAXN);
        let mut SITFNM = ActualCharArray::new(NAMLEN, 1..=NSITES);
        let mut SITNMS = ActualCharArray::new(NAMLEN, 1..=NSITES);
        let mut TARGET = vec![b' '; NAMLEN as usize];
        let mut TOPREF = vec![b' '; NAMLEN as usize];
        let mut A: f64 = 0.0;
        let mut ANGLES = StackArray2D::<f64, 9>::new(1..=3, 1..=NSITES);
        let mut B: f64 = 0.0;
        let mut BOUNDS = StackArray2D::<f64, 6>::new(1..=2, 1..=3);
        let mut C: f64 = 0.0;
        let mut CORPAR = StackArray::<f64, 10>::new(1..=NSYPAR);
        let mut DIST: f64 = 0.0;
        let mut DSKDSC = StackArray::<f64, 24>::new(1..=DSKDSZ);
        let mut F: f64 = 0.0;
        let mut FIRST: f64 = 0.0;
        let mut LAST: f64 = 0.0;
        let mut LAT: f64 = 0.0;
        let mut LON: f64 = 0.0;
        let mut LT: f64 = 0.0;
        let mut MAXLAT: f64 = 0.0;
        let mut MAXLON: f64 = 0.0;
        let mut MAXRAD: f64 = 0.0;
        let mut MINLAT: f64 = 0.0;
        let mut MINLON: f64 = 0.0;
        let mut MINRAD: f64 = 0.0;
        let mut OFFMAG: f64 = 0.0;
        let mut OFFSET = StackArray::<f64, 3>::new(1..=3);
        let mut ORIGIN = StackArray::<f64, 3>::new(1..=3);
        let mut PNEAR = StackArray::<f64, 3>::new(1..=3);
        let mut RADII = StackArray::<f64, 3>::new(1..=3);
        let mut RE: f64 = 0.0;
        let mut RP: f64 = 0.0;
        let mut SGMAXR: f64 = 0.0;
        let mut SGMINR: f64 = 0.0;
        let mut SITPOS = StackArray2D::<f64, 9>::new(1..=3, 1..=NSITES);
        let mut TOL: f64 = 0.0;
        let mut VERTS = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut VRTCES = ActualArray2D::<f64>::new(1..=3, 1..=MAXV);
        let mut VTEMP = StackArray::<f64, 3>::new(1..=3);
        let mut XFORM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut XMAXR: f64 = 0.0;
        let mut XMINR: f64 = 0.0;
        let mut AXES = StackArray2D::<i32, 9>::new(1..=3, 1..=NSITES);
        let mut BODYID: i32 = 0;
        let mut CORSYS: i32 = 0;
        let mut DLADSC = StackArray::<i32, 8>::new(1..=DLADSZ);
        let mut HANDLE: i32 = 0;
        let mut K: i32 = 0;
        let mut N: i32 = 0;
        let mut NLAT: i32 = 0;
        let mut NLON: i32 = 0;
        let mut NP: i32 = 0;
        let mut NSURF: i32 = 0;
        let mut NV: i32 = 0;
        let mut NXTDSC = StackArray::<i32, 8>::new(1..=DLADSZ);
        let mut PLATE = StackArray::<i32, 3>::new(1..=3);
        let mut PLATES = ActualArray2D::<i32>::new(1..=3, 1..=MAXP);
        let mut SITFID = StackArray::<i32, 3>::new(1..=NSITES);
        let mut SITIDS = StackArray::<i32, 3>::new(1..=NSITES);
        let mut SURFID: i32 = 0;
        let mut SRFLST = StackArray::<i32, 100>::new(1..=MAXSRF);
        let mut FOUND: bool = false;
        let mut MAKVTL: bool = false;

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
            FRAMES,
            SITFNM,
            SITNMS,
            TARGET,
            TOPREF,
            A,
            ANGLES,
            B,
            BOUNDS,
            C,
            CORPAR,
            DIST,
            DSKDSC,
            F,
            FIRST,
            LAST,
            LAT,
            LON,
            LT,
            MAXLAT,
            MAXLON,
            MAXRAD,
            MINLAT,
            MINLON,
            MINRAD,
            OFFMAG,
            OFFSET,
            ORIGIN,
            PNEAR,
            RADII,
            RE,
            RP,
            SGMAXR,
            SGMINR,
            SITPOS,
            TOL,
            VERTS,
            VRTCES,
            VTEMP,
            XFORM,
            XMAXR,
            XMINR,
            AXES,
            BODYID,
            CORSYS,
            DLADSC,
            HANDLE,
            K,
            N,
            NLAT,
            NLON,
            NP,
            NSURF,
            NV,
            NXTDSC,
            PLATE,
            PLATES,
            SITFID,
            SITIDS,
            SURFID,
            SRFLST,
            FOUND,
            MAKVTL,
        }
    }
}

//$Procedure F_ZZDSKSPH ( ZZDSKSPH tests )
pub fn F_ZZDSKSPH(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Other functions
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
    testutil::TOPEN(b"F_ZZDSKSPH", ctx)?;

    //**********************************************************************
    //
    //     Set-up
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Setup: create topocentric frame kernels and supporting kernels.",
        ctx,
    )?;

    //
    // Create and load the PCK; keep the file.
    //
    if spicelib::EXISTS(PCK0, ctx)? {
        spicelib::DELFIL(PCK0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // We'll use FURNSH to perform the load; this'll save grief later.
    //
    testutil::TSTPCK(PCK0, false, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(PCK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set the target.
    //
    fstr::assign(&mut save.TARGET, b"MARS");
    save.BODYID = 499;

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

    // Create the FKs and SPKs needed to support topocentric frames.
    //
    for I in 1..=NSITES {
        save.SITIDS[I] = (499000 + I);

        fstr::assign(save.SITNMS.get_mut(I), b"MARS_SURFACE_SITE_#");
        spicelib::REPMI(&save.SITNMS[I].to_vec(), b"#", I, &mut save.SITNMS[I], ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(save.SITFNM.get_mut(I), b"MARS_TOPO_#");
        spicelib::REPMI(&save.SITFNM[I].to_vec(), b"#", I, &mut save.SITFNM[I], ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.SITFID[I] = save.SITIDS[I];

        fstr::assign(save.FRAMES.get_mut(I), b"IAU_MARS");

        save.LON = ((30.0 + (3 * (I - 1)) as f64) * spicelib::RPD(ctx));
        save.LAT = ((45.0 + (2 * (I - 1)) as f64) * spicelib::RPD(ctx));

        if (I == 1) {
            //
            // Create a site on Mars' surface.
            //
            spicelib::SRFREC(
                save.BODYID,
                save.LON,
                save.LAT,
                save.SITPOS.subarray_mut([1, I]),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        } else if (I == 2) {
            //
            // Create a site near Mars' center.
            //
            spicelib::VPACK(100.0, 200.0, 300.0, save.SITPOS.subarray_mut([1, I]));
        } else if (I == 3) {
            //
            // Create a site near Mars' surface, at a radius
            // between C and A.
            //
            spicelib::VPACK(
                ((save.A + save.C) / 2 as f64),
                0.0,
                0.0,
                save.SITPOS.subarray_mut([1, I]),
            );
        } else {
            //
            // This is the backstop case.
            //
            spicelib::SIGERR(b"TEST BUG", ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        save.AXES[[1, I]] = 3;
        save.AXES[[2, I]] = 2;
        save.AXES[[3, I]] = 3;

        save.ANGLES[[1, I]] = -save.LON;
        save.ANGLES[[2, I]] = (save.LAT - (spicelib::PI(ctx) / 2 as f64));
        save.ANGLES[[3, I]] = spicelib::PI(ctx);
    }

    save.FIRST = -((100 as f64) * spicelib::JYEAR());
    save.LAST = ((100 as f64) * spicelib::JYEAR());

    //
    // Create topocentric kernels. We'll need to create
    // separate kernels for the different target bodies.
    //

    if spicelib::EXISTS(TOPFK, ctx)? {
        spicelib::DELFIL(TOPFK, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    if spicelib::EXISTS(TOPSPK, ctx)? {
        spicelib::DELFIL(TOPSPK, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    testutil::T_TOPKER(
        TOPFK,
        TOPSPK,
        &save.TARGET,
        save.FRAMES.first(),
        NSITES,
        save.SITIDS.as_slice(),
        save.SITNMS.as_arg(),
        save.SITPOS.as_slice(),
        save.SITFNM.as_arg(),
        save.SITFID.as_slice(),
        save.FIRST,
        save.LAST,
        save.AXES.as_slice(),
        save.ANGLES.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load the kernels.
    //
    spicelib::FURNSH(TOPFK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(TOPSPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Create Mars DSK containing segments that use each coordinate system.",
        ctx,
    )?;

    if spicelib::EXISTS(DSK0, ctx)? {
        spicelib::DELFIL(DSK0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Start out with a tessellated ellipsoid DSK for Mars. Use
    // latitudinal coordinates.
    //
    save.BODYID = 499;
    save.SURFID = 1;

    save.CORSYS = LATSYS;
    spicelib::CLEARD(3, save.CORPAR.as_slice_mut());

    fstr::assign(&mut save.FIXREF, b"IAU_MARS");

    save.FIRST = -((100 as f64) * spicelib::JYEAR());
    save.LAST = ((100 as f64) * spicelib::JYEAR());

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

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);
    save.BOUNDS[[1, 2]] = -spicelib::HALFPI(ctx);
    save.BOUNDS[[2, 2]] = spicelib::HALFPI(ctx);

    save.NLON = 20;
    save.NLAT = 10;

    save.MAKVTL = false;

    testutil::T_ELDSK2(
        save.BODYID,
        save.SURFID,
        &save.FIXREF,
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
        DSK0,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Second segment: use planetodetic coordinates:
    //
    save.CORSYS = PDTSYS;

    save.RE = save.A;
    save.RP = save.C;
    save.F = ((save.RE - save.RP) / save.RE);

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.SURFID = 2;

    testutil::T_ELDSK2(
        save.BODYID,
        save.SURFID,
        &save.FIXREF,
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
        DSK0,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Third segment: use rectangular coordinates:
    //
    save.CORSYS = RECSYS;

    save.BOUNDS[[1, 1]] = -save.A;
    save.BOUNDS[[2, 1]] = save.A;
    save.BOUNDS[[1, 2]] = -save.A;
    save.BOUNDS[[2, 2]] = save.A;

    save.SURFID = 3;

    testutil::T_ELDSK2(
        save.BODYID,
        save.SURFID,
        &save.FIXREF,
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
        DSK0,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(
        b"Add to Mars DSK three segments having frames with offset centers.",
        ctx,
    )?;

    //
    // The fourth segment of the file, which contains three segments
    // already, will be centered at a surface point. The segment
    // will have rather small extents, so the central body will be
    // well outside of the segment's outer bounding surface.
    //
    save.CORSYS = LATSYS;
    spicelib::CLEARD(3, save.CORPAR.as_slice_mut());

    save.BODYID = 499;
    save.SURFID = 4;

    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    fstr::assign(&mut save.TOPREF, save.SITFNM.get(1));

    save.FIRST = -((100 as f64) * spicelib::JYEAR());
    save.LAST = ((100 as f64) * spicelib::JYEAR());

    spicelib::BODVCD(
        save.BODYID,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;

    save.A = save.RADII[1];
    save.B = save.RADII[2];
    save.C = save.RADII[3];

    save.MINLON = ((25 as f64) * spicelib::RPD(ctx));
    save.MAXLON = ((35 as f64) * spicelib::RPD(ctx));
    save.MINLAT = ((40 as f64) * spicelib::RPD(ctx));
    save.MAXLAT = ((50 as f64) * spicelib::RPD(ctx));

    save.NLON = 20;
    save.NLAT = 10;
    //
    // Create plate set.
    //
    support::ZZELLSEC(
        save.A,
        save.B,
        save.C,
        save.MINLON,
        save.MAXLON,
        save.MINLAT,
        save.MAXLAT,
        save.NLON,
        save.NLAT,
        MAXV,
        MAXP,
        &mut save.NV,
        save.VRTCES.as_slice_mut(),
        &mut save.NP,
        save.PLATES.as_slice_mut(),
        ctx,
    )?;

    //
    // Translate the vertices to make them relative to the origin
    // of the topocentric frame. Rotate the translated vertices
    // to that frame.
    //
    spicelib::SPKEZP(
        save.SITIDS[1],
        0.0,
        &save.FIXREF,
        b"NONE",
        save.BODYID,
        save.OFFSET.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;

    spicelib::PXFORM(
        &save.FIXREF,
        &save.TOPREF,
        0.0,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;

    for I in 1..=save.NV {
        spicelib::VSUB(
            save.VRTCES.subarray([1, I]),
            save.OFFSET.as_slice(),
            save.VTEMP.as_slice_mut(),
        );
        spicelib::MXV(
            save.XFORM.as_slice(),
            save.VTEMP.as_slice(),
            save.VRTCES.subarray_mut([1, I]),
        );
    }

    //
    // Add this segment to the existing Mars DSK.
    //
    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);
    save.BOUNDS[[1, 2]] = -spicelib::HALFPI(ctx);
    save.BOUNDS[[2, 2]] = spicelib::HALFPI(ctx);

    save.MAKVTL = false;

    testutil::T_WRTPLT(
        save.BODYID,
        save.SURFID,
        &save.TOPREF,
        save.FIRST,
        save.LAST,
        save.CORSYS,
        save.CORPAR.as_slice(),
        save.BOUNDS.as_slice(),
        save.NV,
        save.NP,
        save.VRTCES.as_slice(),
        save.PLATES.as_slice(),
        save.MAKVTL,
        DSK0,
        ctx,
    )?;

    //
    // The fifth segment of the file will be centered at a point
    // close to Mars' center. The segment will surround the
    // frame center and be bounded away from it.
    //
    save.CORSYS = LATSYS;
    spicelib::CLEARD(3, save.CORPAR.as_slice_mut());

    save.BODYID = 499;
    save.SURFID = 5;

    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    fstr::assign(&mut save.TOPREF, save.SITFNM.get(2));

    save.FIRST = -((100 as f64) * spicelib::JYEAR());
    save.LAST = ((100 as f64) * spicelib::JYEAR());

    spicelib::BODVCD(
        save.BODYID,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;

    save.A = save.RADII[1];
    save.B = save.RADII[2];
    save.C = save.RADII[3];

    save.NLON = 20;
    save.NLAT = 10;
    //
    // Create plate set.
    //
    support::ZZELLPLT(
        save.A,
        save.B,
        save.C,
        save.NLON,
        save.NLAT,
        MAXV,
        MAXP,
        &mut save.NV,
        save.VRTCES.as_slice_mut(),
        &mut save.NP,
        save.PLATES.as_slice_mut(),
        ctx,
    )?;

    //
    // Translate the vertices to make them relative to the origin
    // of the topocentric frame. Rotate the translated vertices
    // to that frame.
    //
    spicelib::SPKEZP(
        save.SITIDS[2],
        0.0,
        &save.FIXREF,
        b"NONE",
        save.BODYID,
        save.OFFSET.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;

    spicelib::PXFORM(
        &save.FIXREF,
        &save.TOPREF,
        0.0,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;

    for I in 1..=save.NV {
        spicelib::VSUB(
            save.VRTCES.subarray([1, I]),
            save.OFFSET.as_slice(),
            save.VTEMP.as_slice_mut(),
        );
        spicelib::MXV(
            save.XFORM.as_slice(),
            save.VTEMP.as_slice(),
            save.VRTCES.subarray_mut([1, I]),
        );
    }

    //
    // Add this segment to the existing Mars DSK.
    //
    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);
    save.BOUNDS[[1, 2]] = -spicelib::HALFPI(ctx);
    save.BOUNDS[[2, 2]] = spicelib::HALFPI(ctx);

    save.MAKVTL = false;

    testutil::T_WRTPLT(
        save.BODYID,
        save.SURFID,
        &save.TOPREF,
        save.FIRST,
        save.LAST,
        save.CORSYS,
        save.CORPAR.as_slice(),
        save.BOUNDS.as_slice(),
        save.NV,
        save.NP,
        save.VRTCES.as_slice(),
        save.PLATES.as_slice(),
        save.MAKVTL,
        DSK0,
        ctx,
    )?;

    //
    // The sixth segment of the file will be centered at a point
    // close to Mars' surface, but below the sphere of maximum radius.
    //
    save.CORSYS = LATSYS;
    spicelib::CLEARD(3, save.CORPAR.as_slice_mut());

    save.BODYID = 499;
    save.SURFID = 6;

    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    fstr::assign(&mut save.TOPREF, save.SITFNM.get(3));

    save.FIRST = -((100 as f64) * spicelib::JYEAR());
    save.LAST = ((100 as f64) * spicelib::JYEAR());

    spicelib::BODVCD(
        save.BODYID,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;

    save.A = save.RADII[1];
    save.B = save.RADII[2];
    save.C = save.RADII[3];

    save.NLON = 20;
    save.NLAT = 10;
    //
    // Create plate set.
    //
    support::ZZELLPLT(
        save.A,
        save.B,
        save.C,
        save.NLON,
        save.NLAT,
        MAXV,
        MAXP,
        &mut save.NV,
        save.VRTCES.as_slice_mut(),
        &mut save.NP,
        save.PLATES.as_slice_mut(),
        ctx,
    )?;

    //
    // Translate the vertices to make them relative to the origin
    // of the topocentric frame. Rotate the translated vertices
    // to that frame.
    //
    spicelib::SPKEZP(
        save.SITIDS[3],
        0.0,
        &save.FIXREF,
        b"NONE",
        save.BODYID,
        save.OFFSET.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;

    spicelib::PXFORM(
        &save.FIXREF,
        &save.TOPREF,
        0.0,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;

    for I in 1..=save.NV {
        spicelib::VSUB(
            save.VRTCES.subarray([1, I]),
            save.OFFSET.as_slice(),
            save.VTEMP.as_slice_mut(),
        );
        spicelib::MXV(
            save.XFORM.as_slice(),
            save.VTEMP.as_slice(),
            save.VRTCES.subarray_mut([1, I]),
        );
    }

    //
    // Add this segment to the existing Mars DSK.
    //
    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);
    save.BOUNDS[[1, 2]] = -spicelib::HALFPI(ctx);
    save.BOUNDS[[2, 2]] = spicelib::HALFPI(ctx);

    save.MAKVTL = false;

    testutil::T_WRTPLT(
        save.BODYID,
        save.SURFID,
        &save.TOPREF,
        save.FIRST,
        save.LAST,
        save.CORSYS,
        save.CORPAR.as_slice(),
        save.BOUNDS.as_slice(),
        save.NV,
        save.NP,
        save.VRTCES.as_slice(),
        save.PLATES.as_slice(),
        save.MAKVTL,
        DSK0,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Create Saturn DSK containing segments that use each coordinate system.",
        ctx,
    )?;

    if spicelib::EXISTS(DSK1, ctx)? {
        spicelib::DELFIL(DSK1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Start out with a tessellated ellipsoid DSK for Mars. Use
    // latitudinal coordinates.
    //
    save.BODYID = 699;
    save.SURFID = 1;

    save.CORSYS = LATSYS;
    spicelib::CLEARD(3, save.CORPAR.as_slice_mut());

    fstr::assign(&mut save.FIXREF, b"IAU_SATURN");

    save.FIRST = -((100 as f64) * spicelib::JYEAR());
    save.LAST = ((100 as f64) * spicelib::JYEAR());

    spicelib::BODVCD(
        save.BODYID,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;

    save.A = save.RADII[1];
    save.B = save.RADII[2];
    save.C = save.RADII[3];

    save.BOUNDS[[1, 1]] = 0.0;
    save.BOUNDS[[2, 1]] = spicelib::TWOPI(ctx);
    save.BOUNDS[[1, 2]] = -spicelib::HALFPI(ctx);
    save.BOUNDS[[2, 2]] = spicelib::HALFPI(ctx);

    save.NLON = 20;
    save.NLAT = 10;

    save.MAKVTL = false;

    testutil::T_ELDSK2(
        save.BODYID,
        save.SURFID,
        &save.FIXREF,
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
        DSK1,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Second segment: use planetodetic coordinates. Make
    // Saturn into a *prolate* shape to excercise logic for
    // such shapes.
    //
    save.CORSYS = PDTSYS;

    save.RE = save.C;
    save.RP = save.A;
    save.F = ((save.RE - save.RP) / save.RE);

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.SURFID = 2;

    testutil::T_ELDSK2(
        save.BODYID,
        save.SURFID,
        &save.FIXREF,
        save.FIRST,
        save.LAST,
        save.CORSYS,
        save.CORPAR.as_slice(),
        save.BOUNDS.as_slice(),
        save.C,
        save.C,
        save.A,
        save.NLON,
        save.NLAT,
        save.MAKVTL,
        DSK1,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Third segment: use rectangular coordinates:
    //
    save.CORSYS = RECSYS;

    save.BOUNDS[[1, 1]] = -save.A;
    save.BOUNDS[[2, 1]] = save.A;
    save.BOUNDS[[1, 2]] = -save.A;
    save.BOUNDS[[2, 2]] = save.A;

    save.SURFID = 3;

    testutil::T_ELDSK2(
        save.BODYID,
        save.SURFID,
        &save.FIXREF,
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
        DSK1,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //**********************************************************************
    //
    //     Normal cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Find bounding radii for Mars latitudinal segment.", ctx)?;

    spicelib::FURNSH(DSK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CLEARI(MAXSRF, save.SRFLST.as_slice_mut());

    save.BODYID = 499;
    save.NSURF = 1;
    save.SRFLST[1] = 1;

    spicelib::ZZDSKSPH(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        &mut save.MINRAD,
        &mut save.MAXRAD,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Since the number of latitude bands is even, the maximum
    // radius should be attained by vertices on the equator.
    //
    spicelib::BODVCD(
        save.BODYID,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;

    save.A = save.RADII[1];
    save.B = save.RADII[2];
    save.C = save.RADII[3];

    save.XMAXR = save.A;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"MAXRAD", save.MAXRAD, b"~/", save.XMAXR, save.TOL, OK, ctx)?;

    //
    // The minimum radius should match the bound in the dscriptor.
    //

    spicelib::DASOPR(DSK0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DLABFS(
        save.HANDLE,
        save.DLADSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"DLA FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::DSKGD(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XMINR = save.DSKDSC[MN3IDX];

    save.TOL = 0 as f64;

    testutil::CHCKSD(
        b"(1) MINRAD",
        save.MINRAD,
        b"=",
        save.XMINR,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Find the actual minimum radius of the surface and use this
    // as a secondary check.
    //
    // The minimum radius is not so simple to find. We'll brute-force
    // it by finding the distance of each plate from the origin.
    //
    spicelib::DSKZ02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        &mut save.NV,
        &mut save.NP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    save.XMINR = save.A;

    for I in 1..=save.NP {
        spicelib::DSKP02(
            save.HANDLE,
            save.DLADSC.as_slice(),
            I,
            1,
            &mut save.N,
            save.PLATE.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Fetch vertices of the Ith plate.
        //
        for J in 1..=3 {
            save.K = save.PLATE[J];

            spicelib::DSKV02(
                save.HANDLE,
                save.DLADSC.as_slice(),
                save.K,
                1,
                &mut save.N,
                save.VERTS.subarray_mut([1, J]),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        //
        // Compute the distance of the Ith plate from the origin.
        //
        spicelib::PLTNP(
            save.ORIGIN.as_slice(),
            save.VERTS.subarray([1, 1]),
            save.VERTS.subarray([1, 2]),
            save.VERTS.subarray([1, 3]),
            save.PNEAR.as_slice_mut(),
            &mut save.DIST,
            ctx,
        )?;

        save.XMINR = intrinsics::DMIN1(&[save.XMINR, save.DIST]);
    }

    save.TOL = VTIGHT;

    testutil::CHCKSD(
        b"(2) MINRAD",
        save.MINRAD,
        b"~/",
        save.XMINR,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Repeat Mars latitudinal case; re-use segment list.", ctx)?;

    spicelib::ZZDSKSPH(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        &mut save.MINRAD,
        &mut save.MAXRAD,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"MAXRAD", save.MAXRAD, b"~/", save.XMAXR, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"MINRAD", save.MINRAD, b"~/", save.XMINR, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Find bounding radii for Mars planetodetic segment.", ctx)?;

    //
    // Get the DLA descriptor for this segment.
    //
    spicelib::DLAFNS(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.NXTDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"DLA FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::MOVEI(save.NXTDSC.as_slice(), DLADSZ, save.DLADSC.as_slice_mut());

    save.BODYID = 499;
    save.NSURF = 1;
    save.SRFLST[1] = 2;

    spicelib::ZZDSKSPH(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        &mut save.MINRAD,
        &mut save.MAXRAD,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The minimum radius will be computed from the reference
    // ellipsoid's radii and the segment's altitude bounds.
    //
    spicelib::DSKGD(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XMINR = (save.C + save.DSKDSC[MN3IDX]);

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"MINRAD", save.MINRAD, b"~/", save.XMINR, save.TOL, OK, ctx)?;

    //
    // Since the number of latitude bands is even, the maximum
    // radius should be attained by vertices on the equator.
    //
    save.XMAXR = save.A;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"MAXRAD", save.MAXRAD, b"~/", save.XMAXR, save.TOL, OK, ctx)?;

    //
    // Restore the expected minimum radius from the DSK descriptor.
    //
    save.XMINR = (save.DSKDSC[MN3IDX] + save.C);

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Repeat Mars planetodetic case; re-use segment list.", ctx)?;

    spicelib::ZZDSKSPH(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        &mut save.MINRAD,
        &mut save.MAXRAD,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"MAXRAD", save.MAXRAD, b"~/", save.XMAXR, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"MINRAD", save.MINRAD, b"~/", save.XMINR, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Find bounding radii for Mars rectangular segment.", ctx)?;

    //
    // Get the DLA descriptor for this segment.
    //
    spicelib::DLAFNS(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.NXTDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"DLA FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::MOVEI(save.NXTDSC.as_slice(), DLADSZ, save.DLADSC.as_slice_mut());

    save.BODYID = 499;
    save.NSURF = 1;
    save.SRFLST[1] = 3;

    spicelib::ZZDSKSPH(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        &mut save.MINRAD,
        &mut save.MAXRAD,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // In this case, the outer bounding radius is that of a box
    // that circumscribes the ellipsoid.
    //
    save.XMAXR = f64::sqrt((((2 as f64) * f64::powi(save.A, 2)) + f64::powi(save.C, 2)));

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"MAXRAD", save.MAXRAD, b"~/", save.XMAXR, save.TOL, OK, ctx)?;

    //
    // The segment's central body is inside the segment's bounding box;
    // the expected minimum radius is zero.
    //
    save.XMINR = 0.0;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"MINRAD", save.MINRAD, b"~/", save.XMINR, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Repeat Mars rectangular case; re-use segment list.", ctx)?;

    spicelib::ZZDSKSPH(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        &mut save.MINRAD,
        &mut save.MAXRAD,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"MAXRAD", save.MAXRAD, b"~/", save.XMAXR, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"MINRAD", save.MINRAD, b"~/", save.XMINR, save.TOL, OK, ctx)?;

    //
    // Close the DSK so we can unload it later.
    //
    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //
    // Now load Saturn kernels; repeat tests for Saturn.
    //
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Find bounding radii for Saturn latitudinal segment.", ctx)?;

    //
    // Load Saturn DSK; leave Mars DSK loaded.
    //
    spicelib::FURNSH(DSK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CLEARI(MAXSRF, save.SRFLST.as_slice_mut());

    save.BODYID = 699;
    save.NSURF = 1;
    save.SRFLST[1] = 1;

    spicelib::ZZDSKSPH(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        &mut save.MINRAD,
        &mut save.MAXRAD,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Since the number of latitude bands is even, the maximum
    // radius should be attained by vertices on the equator.
    //
    spicelib::BODVCD(
        save.BODYID,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;

    save.A = save.RADII[1];
    save.B = save.RADII[2];
    save.C = save.RADII[3];

    save.XMAXR = save.A;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"MAXRAD", save.MAXRAD, b"~/", save.XMAXR, save.TOL, OK, ctx)?;

    //
    // The minimum radius should match the bound in the dscriptor.
    //

    spicelib::DASOPR(DSK1, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DLABFS(
        save.HANDLE,
        save.DLADSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"DLA FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::DSKGD(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XMINR = save.DSKDSC[MN3IDX];

    save.TOL = 0 as f64;

    testutil::CHCKSD(
        b"(1) MINRAD",
        save.MINRAD,
        b"=",
        save.XMINR,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Find the actual minimum radius of the surface and use this
    // as a secondary check.
    //
    // The minimum radius is not so simple to find. We'll brute-force
    // it by finding the distance of each plate from the origin.
    //
    spicelib::DSKZ02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        &mut save.NV,
        &mut save.NP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    save.XMINR = save.A;

    for I in 1..=save.NP {
        spicelib::DSKP02(
            save.HANDLE,
            save.DLADSC.as_slice(),
            I,
            1,
            &mut save.N,
            save.PLATE.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Fetch vertices of the Ith plate.
        //
        for J in 1..=3 {
            save.K = save.PLATE[J];

            spicelib::DSKV02(
                save.HANDLE,
                save.DLADSC.as_slice(),
                save.K,
                1,
                &mut save.N,
                save.VERTS.subarray_mut([1, J]),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        //
        // Compute the distance of the Ith plate from the origin.
        //
        spicelib::PLTNP(
            save.ORIGIN.as_slice(),
            save.VERTS.subarray([1, 1]),
            save.VERTS.subarray([1, 2]),
            save.VERTS.subarray([1, 3]),
            save.PNEAR.as_slice_mut(),
            &mut save.DIST,
            ctx,
        )?;

        save.XMINR = intrinsics::DMIN1(&[save.XMINR, save.DIST]);
    }

    save.TOL = VTIGHT;

    testutil::CHCKSD(
        b"(2) MINRAD",
        save.MINRAD,
        b"~/",
        save.XMINR,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Repeat Saturn latitudinal case; re-use segment list.", ctx)?;

    spicelib::ZZDSKSPH(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        &mut save.MINRAD,
        &mut save.MAXRAD,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"MAXRAD", save.MAXRAD, b"~/", save.XMAXR, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"MINRAD", save.MINRAD, b"~/", save.XMINR, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Find bounding radii for Saturn planetodetic segment.", ctx)?;

    //
    // Get the DLA descriptor for this segment.
    //
    spicelib::DLAFNS(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.NXTDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"DLA FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::MOVEI(save.NXTDSC.as_slice(), DLADSZ, save.DLADSC.as_slice_mut());

    //
    // We'll need the DSK descriptor for this segment, since
    // the segment bounds are unique to this segment.
    //
    spicelib::DSKGD(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BODYID = 699;
    save.NSURF = 1;
    save.SRFLST[1] = 2;

    spicelib::ZZDSKSPH(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        &mut save.MINRAD,
        &mut save.MAXRAD,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Since the number of latitude bands is even, the maximum
    // radius should be attained by vertices on the pole (recall
    // we've made Saturn prolate in this segment).
    //
    save.XMAXR = save.A;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"MAXRAD", save.MAXRAD, b"~/", save.XMAXR, save.TOL, OK, ctx)?;

    //
    // The minimum radius will be computed from the reference
    // ellipsoid's radii and the segment's altitude bounds.
    //
    spicelib::DSKGD(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Recall this model is prolate. However, A, B, C are from
    // the kernel pool.
    //
    save.XMINR = (save.C + save.DSKDSC[MN3IDX]);

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"MINRAD", save.MINRAD, b"~/", save.XMINR, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Repeat Saturn planetodetic case; re-use segment list.",
        ctx,
    )?;

    spicelib::ZZDSKSPH(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        &mut save.MINRAD,
        &mut save.MAXRAD,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"MAXRAD", save.MAXRAD, b"~/", save.XMAXR, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"MINRAD", save.MINRAD, b"~/", save.XMINR, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Find bounding radii for Saturn rectangular segment.", ctx)?;

    //
    // Get the DLA descriptor for this segment.
    //
    spicelib::DLAFNS(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.NXTDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"DLA FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::MOVEI(save.NXTDSC.as_slice(), DLADSZ, save.DLADSC.as_slice_mut());

    save.BODYID = 699;
    save.NSURF = 1;
    save.SRFLST[1] = 3;

    spicelib::ZZDSKSPH(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        &mut save.MINRAD,
        &mut save.MAXRAD,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // In this case, the outer bounding radius is that of a box
    // that circumscribes the ellipsoid.
    //
    save.XMAXR = f64::sqrt((((2 as f64) * f64::powi(save.A, 2)) + f64::powi(save.C, 2)));

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"MAXRAD", save.MAXRAD, b"~/", save.XMAXR, save.TOL, OK, ctx)?;

    //
    // The segment's central body is inside the segment's bounding box;
    // the expected minimum radius is zero.
    //
    save.XMINR = 0.0;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"MINRAD", save.MINRAD, b"~/", save.XMINR, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Repeat Saturn rectangular case; re-use segment list.", ctx)?;

    spicelib::ZZDSKSPH(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        &mut save.MINRAD,
        &mut save.MAXRAD,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"MAXRAD", save.MAXRAD, b"~/", save.XMAXR, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"MINRAD", save.MINRAD, b"~/", save.XMINR, save.TOL, OK, ctx)?;

    //
    // Check handling of BSR state change.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Unload Mars DSK and repeat Saturn rectangular case again; re-use segment list.",
        ctx,
    )?;

    spicelib::UNLOAD(DSK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZDSKSPH(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        &mut save.MINRAD,
        &mut save.MAXRAD,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"MAXRAD", save.MAXRAD, b"~/", save.XMAXR, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"MINRAD", save.MINRAD, b"~/", save.XMINR, save.TOL, OK, ctx)?;

    //
    // Close the Saturn DSK so we can unload it later.
    //
    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The next set of tests uses frames having centers offset
    // from the central body.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Find bounding radii for Mars segment having its frame center near Mars\' surface. For this case, the minimum radius is the maximum segment radius minus the frame offset.", ctx)?;

    spicelib::FURNSH(DSK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CLEARI(MAXSRF, save.SRFLST.as_slice_mut());

    save.BODYID = 499;
    save.NSURF = 1;
    save.SRFLST[1] = 4;

    spicelib::ZZDSKSPH(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        &mut save.MINRAD,
        &mut save.MAXRAD,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // For this case, the upper bound is the sum of the
    // DSK descriptor's upper radius bound and the magnitude of
    // the offset between the frame's center and Mars.
    //
    spicelib::SPKEZP(
        save.SITIDS[1],
        0.0,
        &save.SITFNM[1],
        b"NONE",
        save.BODYID,
        save.OFFSET.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.OFFMAG = spicelib::VNORM(save.OFFSET.as_slice());

    spicelib::DASOPR(DSK0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DLABFS(
        save.HANDLE,
        save.NXTDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"DLABFS found", save.FOUND, true, OK, ctx)?;

    for I in 1..=3 {
        spicelib::DLAFNS(
            save.HANDLE,
            save.NXTDSC.as_slice(),
            save.DLADSC.as_slice_mut(),
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(b"DLAFNS found", save.FOUND, true, OK, ctx)?;

        spicelib::MOVEI(save.DLADSC.as_slice(), DLADSZ, save.NXTDSC.as_slice_mut());
    }

    spicelib::DSKGD(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XMAXR = (save.DSKDSC[MX3IDX] + save.OFFMAG);

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"MAXRAD", save.MAXRAD, b"~/", save.XMAXR, save.TOL, OK, ctx)?;

    //
    // The minimum radius should be the lower bound in the descriptor,
    // minus the offset magnitude.
    //
    // The minimum radius should be the offset magnitude, minus
    // the upper bound in the descriptor.
    //
    save.XMINR = (save.OFFMAG - save.DSKDSC[MX3IDX]);

    save.TOL = 0 as f64;

    testutil::CHCKSD(b"MINRAD", save.MINRAD, b"=", save.XMINR, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Find bounding radii for Mars segment having its frame center near Mars\' center. For this case, the minimum radius is the minimum segment radius minus the frame offset.", ctx)?;

    spicelib::CLEARI(MAXSRF, save.SRFLST.as_slice_mut());

    save.BODYID = 499;
    save.NSURF = 1;
    save.SRFLST[1] = 5;

    spicelib::ZZDSKSPH(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        &mut save.MINRAD,
        &mut save.MAXRAD,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // For this case, the upper bound is the sum of the
    // DSK descriptor's upper radius bound and the magnitude of
    // the offset between the frame's center and Mars.
    //
    spicelib::SPKEZP(
        save.SITIDS[2],
        0.0,
        &save.SITFNM[3],
        b"NONE",
        save.BODYID,
        save.OFFSET.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.OFFMAG = spicelib::VNORM(save.OFFSET.as_slice());

    spicelib::DLAFNS(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.NXTDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"DLAFNS found", save.FOUND, true, OK, ctx)?;

    spicelib::MOVEI(save.NXTDSC.as_slice(), DLADSZ, save.DLADSC.as_slice_mut());

    spicelib::DSKGD(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XMAXR = (save.DSKDSC[MX3IDX] + save.OFFMAG);

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"MAXRAD", save.MAXRAD, b"~/", save.XMAXR, save.TOL, OK, ctx)?;

    //
    // The minimum radius should be the lower bound in the descriptor,
    // minus the offset magnitude.
    //
    // The minimum radius should be the offset magnitude, minus
    // the upper bound in the descriptor.
    //
    save.XMINR = (save.DSKDSC[MN3IDX] - save.OFFMAG);

    save.TOL = 0 as f64;

    testutil::CHCKSD(b"MINRAD", save.MINRAD, b"=", save.XMINR, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Find bounding radii for Mars segment having its frame center slightly below Mars\' surface. For this case, the minimum radius is zero.", ctx)?;

    spicelib::CLEARI(MAXSRF, save.SRFLST.as_slice_mut());

    save.BODYID = 499;
    save.NSURF = 1;
    save.SRFLST[1] = 6;

    spicelib::ZZDSKSPH(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        &mut save.MINRAD,
        &mut save.MAXRAD,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // For this case, the upper bound is the sum of the
    // DSK descriptor's upper radius bound and the magnitude of
    // the offset between the frame's center and Mars.
    //
    spicelib::SPKEZP(
        save.SITIDS[3],
        0.0,
        &save.SITFNM[3],
        b"NONE",
        save.BODYID,
        save.OFFSET.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.OFFMAG = spicelib::VNORM(save.OFFSET.as_slice());

    spicelib::DLAFNS(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.NXTDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"DLAFNS found", save.FOUND, true, OK, ctx)?;

    spicelib::MOVEI(save.NXTDSC.as_slice(), DLADSZ, save.DLADSC.as_slice_mut());

    spicelib::DSKGD(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XMAXR = (save.DSKDSC[MX3IDX] + save.OFFMAG);

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"MAXRAD", save.MAXRAD, b"~/", save.XMAXR, save.TOL, OK, ctx)?;

    //
    // The frame center offset is between the segment's minimum
    // and maximum radius, so we can't guarantee that the frame
    // center is a positive distance from all plates.
    //
    save.XMINR = 0.0;

    save.TOL = 0 as f64;

    testutil::CHCKSD(b"MINRAD", save.MINRAD, b"=", save.XMINR, save.TOL, OK, ctx)?;

    //
    // Close the Mars DSK so we can unload it later.
    //
    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Find Mars radius bounds using empty surface list.", ctx)?;

    save.BODYID = 499;
    save.NSURF = 0;

    spicelib::CLEARI(MAXSRF, save.SRFLST.as_slice_mut());

    spicelib::ZZDSKSPH(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        &mut save.MINRAD,
        &mut save.MAXRAD,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Generate expected bounds by combining bounds for each surface.
    //
    save.XMINR = spicelib::DPMAX();
    save.XMAXR = 0.0;

    for I in 1..=6 {
        spicelib::ZZDSKSPH(
            save.BODYID,
            save.NSURF,
            save.SRFLST.subarray(I),
            &mut save.SGMINR,
            &mut save.SGMAXR,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.XMINR = intrinsics::DMIN1(&[save.XMINR, save.SGMINR]);
        save.XMAXR = intrinsics::DMAX1(&[save.XMAXR, save.SGMAXR]);
    }

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"MINRAD", save.MINRAD, b"=", save.XMINR, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"MAXRAD", save.MAXRAD, b"=", save.XMAXR, save.TOL, OK, ctx)?;

    //**********************************************************************
    //
    //     Error cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Error: try to find bounds for body having no loaded DSK segments.",
        ctx,
    )?;

    save.BODYID = 299;

    save.NSURF = 0;
    spicelib::CLEARI(MAXSRF, save.SRFLST.as_slice_mut());

    spicelib::ZZDSKSPH(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        &mut save.MINRAD,
        &mut save.MAXRAD,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DSKDATANOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Error: find bounds for Mars. Unload Mars DSK. Try again to find bounds for Mars.",
        ctx,
    )?;

    save.BODYID = 499;
    save.NSURF = 0;

    spicelib::ZZDSKSPH(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        &mut save.MINRAD,
        &mut save.MAXRAD,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(DSK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZDSKSPH(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        &mut save.MINRAD,
        &mut save.MAXRAD,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DSKDATANOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Reload Mars DSK. Unload Mars SPK. Try again to find bounds for Mars.",
        ctx,
    )?;

    spicelib::FURNSH(DSK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(TOPSPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZDSKSPH(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        &mut save.MINRAD,
        &mut save.MAXRAD,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOLOADEDFILES)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Reload Mars SPK. Unload Mars FK. Try again to find bounds for Mars.",
        ctx,
    )?;

    spicelib::FURNSH(TOPSPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(TOPFK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZDSKSPH(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        &mut save.MINRAD,
        &mut save.MAXRAD,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOFRAMEDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Reload FK.  Try again to find bounds for Mars. This is a non-error case.",
        ctx,
    )?;

    spicelib::FURNSH(TOPFK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZDSKSPH(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        &mut save.MINRAD,
        &mut save.MAXRAD,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Generate expected bounds by combining bounds for each surface.
    //
    save.XMINR = spicelib::DPMAX();
    save.XMAXR = 0.0;

    for I in 1..=6 {
        spicelib::ZZDSKSPH(
            save.BODYID,
            save.NSURF,
            save.SRFLST.subarray(I),
            &mut save.SGMINR,
            &mut save.SGMAXR,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.XMINR = intrinsics::DMIN1(&[save.XMINR, save.SGMINR]);
        save.XMAXR = intrinsics::DMAX1(&[save.XMAXR, save.SGMAXR]);
    }

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"MINRAD", save.MINRAD, b"=", save.XMINR, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"MAXRAD", save.MAXRAD, b"=", save.XMAXR, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Repeat previous case, minus the kernel loads.", ctx)?;

    spicelib::ZZDSKSPH(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        &mut save.MINRAD,
        &mut save.MAXRAD,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Generate expected bounds by combining bounds for each surface.
    //
    save.XMINR = spicelib::DPMAX();
    save.XMAXR = 0.0;

    for I in 1..=6 {
        spicelib::ZZDSKSPH(
            save.BODYID,
            save.NSURF,
            save.SRFLST.subarray(I),
            &mut save.SGMINR,
            &mut save.SGMAXR,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.XMINR = intrinsics::DMIN1(&[save.XMINR, save.SGMINR]);
        save.XMAXR = intrinsics::DMAX1(&[save.XMAXR, save.SGMAXR]);
    }

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"MINRAD", save.MINRAD, b"=", save.XMINR, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"MAXRAD", save.MAXRAD, b"=", save.XMAXR, save.TOL, OK, ctx)?;

    //**********************************************************************
    //
    //     Clean up
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up.", ctx)?;

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(PCK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(TOPFK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(TOPSPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
