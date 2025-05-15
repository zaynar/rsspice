//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

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
const TOL: f64 = 0.001;

struct SaveVars {
    AVEXT: f64,
    BXMAX: f64,
    BXMIN: f64,
    BYMAX: f64,
    BYMIN: f64,
    BZMAX: f64,
    BZMIN: f64,
    CVXSIZ: f64,
    MDLTOL: f64,
    VMOD: StackArray<f64, 3>,
    XEXTNT: StackArray<f64, 6>,
    XMAX: f64,
    XMIN: f64,
    XP: StackArray<f64, 3>,
    XVMAX: f64,
    XVMIN: f64,
    YMAX: f64,
    YMIN: f64,
    YP: StackArray<f64, 3>,
    YVMAX: f64,
    YVMIN: f64,
    ZMAX: f64,
    ZMIN: f64,
    ZP: StackArray<f64, 3>,
    ZVMAX: f64,
    ZVMIN: f64,
    CGOF1D: i32,
    CGOFF: StackArray<i32, 3>,
    CGRDIM: StackArray<i32, 3>,
    CGXYZ: StackArray<i32, 3>,
    CVID: i32,
    GXMAX: i32,
    GXMIN: i32,
    GYMAX: i32,
    GYMIN: i32,
    GZMAX: i32,
    GZMIN: i32,
    IXPTR: i32,
    NCELL: i32,
    NCGFLG: i32,
    NPCG: i32,
    NX: i32,
    NY: i32,
    NZ: i32,
    Q: i32,
    R: i32,
    TO: i32,
    VIXYZ: StackArray<i32, 3>,
    VCOORD: StackArray<i32, 3>,
    INBOX: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut AVEXT: f64 = 0.0;
        let mut BXMAX: f64 = 0.0;
        let mut BXMIN: f64 = 0.0;
        let mut BYMAX: f64 = 0.0;
        let mut BYMIN: f64 = 0.0;
        let mut BZMAX: f64 = 0.0;
        let mut BZMIN: f64 = 0.0;
        let mut CVXSIZ: f64 = 0.0;
        let mut MDLTOL: f64 = 0.0;
        let mut VMOD = StackArray::<f64, 3>::new(1..=3);
        let mut XEXTNT = StackArray::<f64, 6>::new(1..=6);
        let mut XMAX: f64 = 0.0;
        let mut XMIN: f64 = 0.0;
        let mut XP = StackArray::<f64, 3>::new(1..=3);
        let mut XVMAX: f64 = 0.0;
        let mut XVMIN: f64 = 0.0;
        let mut YMAX: f64 = 0.0;
        let mut YMIN: f64 = 0.0;
        let mut YP = StackArray::<f64, 3>::new(1..=3);
        let mut YVMAX: f64 = 0.0;
        let mut YVMIN: f64 = 0.0;
        let mut ZMAX: f64 = 0.0;
        let mut ZMIN: f64 = 0.0;
        let mut ZP = StackArray::<f64, 3>::new(1..=3);
        let mut ZVMAX: f64 = 0.0;
        let mut ZVMIN: f64 = 0.0;
        let mut CGOF1D: i32 = 0;
        let mut CGOFF = StackArray::<i32, 3>::new(1..=3);
        let mut CGRDIM = StackArray::<i32, 3>::new(1..=3);
        let mut CGXYZ = StackArray::<i32, 3>::new(1..=3);
        let mut CVID: i32 = 0;
        let mut GXMAX: i32 = 0;
        let mut GXMIN: i32 = 0;
        let mut GYMAX: i32 = 0;
        let mut GYMIN: i32 = 0;
        let mut GZMAX: i32 = 0;
        let mut GZMIN: i32 = 0;
        let mut IXPTR: i32 = 0;
        let mut NCELL: i32 = 0;
        let mut NCGFLG: i32 = 0;
        let mut NPCG: i32 = 0;
        let mut NX: i32 = 0;
        let mut NY: i32 = 0;
        let mut NZ: i32 = 0;
        let mut Q: i32 = 0;
        let mut R: i32 = 0;
        let mut TO: i32 = 0;
        let mut VIXYZ = StackArray::<i32, 3>::new(1..=3);
        let mut VCOORD = StackArray::<i32, 3>::new(1..=3);
        let mut INBOX: bool = false;

        Self {
            AVEXT,
            BXMAX,
            BXMIN,
            BYMAX,
            BYMIN,
            BZMAX,
            BZMIN,
            CVXSIZ,
            MDLTOL,
            VMOD,
            XEXTNT,
            XMAX,
            XMIN,
            XP,
            XVMAX,
            XVMIN,
            YMAX,
            YMIN,
            YP,
            YVMAX,
            YVMIN,
            ZMAX,
            ZMIN,
            ZP,
            ZVMAX,
            ZVMIN,
            CGOF1D,
            CGOFF,
            CGRDIM,
            CGXYZ,
            CVID,
            GXMAX,
            GXMIN,
            GYMAX,
            GYMIN,
            GZMAX,
            GZMIN,
            IXPTR,
            NCELL,
            NCGFLG,
            NPCG,
            NX,
            NY,
            NZ,
            Q,
            R,
            TO,
            VIXYZ,
            VCOORD,
            INBOX,
        }
    }
}

//$Procedure ZZMKSPIN ( Make spatial index of plates )
pub fn ZZMKSPIN(
    NP: i32,
    PLATES: &[i32],
    VRTCES: &[f64],
    VOXSCL: f64,
    CGSCAL: i32,
    MAXPTR: i32,
    MXCELL: i32,
    MAXVXL: i32,
    CELLS: &mut [i32],
    NVOX: &mut [i32],
    VOXSIZ: &mut f64,
    VOXORI: &mut [f64],
    NVXTOT: &mut i32,
    NVXPTR: &mut i32,
    VXPTR: &mut [i32],
    NVXLST: &mut i32,
    VXLIST: &mut [i32],
    EXTENT: &mut [f64],
    CGRPTR: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let PLATES = DummyArray2D::new(PLATES, 1..=3, 1..);
    let VRTCES = DummyArray2D::new(VRTCES, 1..=3, 1..);
    let mut CELLS = DummyArrayMut2D::new(CELLS, 1..=2, 1..=MXCELL);
    let mut NVOX = DummyArrayMut::new(NVOX, 1..=3);
    let mut VOXORI = DummyArrayMut::new(VOXORI, 1..=3);
    let mut VXPTR = DummyArrayMut::new(VXPTR, 1..);
    let mut VXLIST = DummyArrayMut::new(VXLIST, 1..);
    let mut EXTENT = DummyArrayMut::new(EXTENT, 1..=6);
    let mut CGRPTR = DummyArrayMut::new(CGRPTR, 1..);

    //
    // SPICELIB functions
    //

    //
    // Other functions
    //

    //
    // Local parameters
    //

    //
    // Fraction of voxel edge length used a tolerance for plate
    // inclusion in voxels:
    //

    //
    // Local Variables
    //

    //
    // Saved variables
    //
    //
    // Required for f2c use on Linux, all local variables
    // to static.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZMKSPIN", ctx)?;

    //
    // Check NP.
    //
    if ((NP < 1) || (NP > MAXPLT)) {
        SETMSG(b"Plate count NP = #; count must be in the range 1:#.", ctx);
        ERRINT(b"#", NP, ctx);
        ERRINT(b"#", MAXPLT, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZMKSPIN", ctx)?;
        return Ok(());
    }

    //
    // Make sure the coarse voxel scale is positive. We'll
    // perform additional checks later on. Those checks
    // require computations that can't be done if the coarse
    // scale is zero.
    //
    if (CGSCAL < 1) {
        SETMSG(b"Coarse voxel scale = #; scale must be positive.", ctx);
        ERRINT(b"#", CGSCAL, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZMKSPIN", ctx)?;
        return Ok(());
    }

    //
    // Get the average extents of all plates and the
    // overall model extent. The extents have model units; in
    // other words km.
    //
    save.AVEXT = 0.0;

    save.XMIN = DPMAX();
    save.XMAX = DPMIN();
    save.YMIN = DPMAX();
    save.YMAX = DPMIN();
    save.ZMIN = DPMAX();
    save.ZMAX = DPMIN();

    for I in 1..=NP {
        save.BXMIN = DPMAX();
        save.BXMAX = DPMIN();
        save.BYMIN = DPMAX();
        save.BYMAX = DPMIN();
        save.BZMIN = DPMAX();
        save.BZMAX = DPMIN();

        save.XP[1] = VRTCES[[1, PLATES[[1, I]]]];
        save.XP[2] = VRTCES[[1, PLATES[[2, I]]]];
        save.XP[3] = VRTCES[[1, PLATES[[3, I]]]];

        save.YP[1] = VRTCES[[2, PLATES[[1, I]]]];
        save.YP[2] = VRTCES[[2, PLATES[[2, I]]]];
        save.YP[3] = VRTCES[[2, PLATES[[3, I]]]];

        save.ZP[1] = VRTCES[[3, PLATES[[1, I]]]];
        save.ZP[2] = VRTCES[[3, PLATES[[2, I]]]];
        save.ZP[3] = VRTCES[[3, PLATES[[3, I]]]];

        for J in 1..=3 {
            //
            // Determine plate extents.
            //
            save.BXMIN = intrinsics::DMIN1(&[save.BXMIN, save.XP[J]]);
            save.BXMAX = intrinsics::DMAX1(&[save.BXMAX, save.XP[J]]);

            save.BYMIN = intrinsics::DMIN1(&[save.BYMIN, save.YP[J]]);
            save.BYMAX = intrinsics::DMAX1(&[save.BYMAX, save.YP[J]]);

            save.BZMIN = intrinsics::DMIN1(&[save.BZMIN, save.ZP[J]]);
            save.BZMAX = intrinsics::DMAX1(&[save.BZMAX, save.ZP[J]]);

            //
            // Determine model extent.
            //
            save.XMIN = intrinsics::DMIN1(&[save.XMIN, save.BXMIN]);
            save.XMAX = intrinsics::DMAX1(&[save.XMAX, save.BXMAX]);

            save.YMIN = intrinsics::DMIN1(&[save.YMIN, save.BYMIN]);
            save.YMAX = intrinsics::DMAX1(&[save.YMAX, save.BYMAX]);

            save.ZMIN = intrinsics::DMIN1(&[save.ZMIN, save.BZMIN]);
            save.ZMAX = intrinsics::DMAX1(&[save.ZMAX, save.BZMAX]);
        }

        EXTENT[1] = save.XMIN;
        EXTENT[2] = save.XMAX;
        EXTENT[3] = save.YMIN;
        EXTENT[4] = save.YMAX;
        EXTENT[5] = save.ZMIN;
        EXTENT[6] = save.ZMAX;

        //
        // Calculate the cumulative extent of all plates for
        // each degree of freedom.
        //
        save.AVEXT = (((save.AVEXT + f64::abs((save.BXMAX - save.BXMIN)))
            + f64::abs((save.BYMAX - save.BYMIN)))
            + f64::abs((save.BZMAX - save.BZMIN)));
    }

    //
    // Calculate the average extent of all plates for
    // and the voxel size, i.e the length of one side
    // of a voxel cube.
    //
    save.AVEXT = (save.AVEXT / ((3 * NP) as f64));
    *VOXSIZ = (VOXSCL * save.AVEXT);
    save.MDLTOL = (*VOXSIZ * TOL);

    //
    // Produce a set of vertex extents, extended by MDLTOL,
    // to be used later.
    //
    for I in intrinsics::range(1, 5, 2) {
        save.XEXTNT[I] = (EXTENT[I] - save.MDLTOL);
        save.XEXTNT[(I + 1)] = (EXTENT[(I + 1)] + save.MDLTOL);
    }

    //
    // Determine the size of the coarse voxels.
    //
    save.CVXSIZ = (*VOXSIZ * CGSCAL as f64);

    //
    // Determine the minima and maxima of the body centered
    // vertex coordinates expressed in coarse voxel units. Scale the
    // vertices coord values by CVXSIZ: this scales the
    // axis in the voxel model space producing cubic voxels
    // with length 1 along each edge in voxel space,
    // CVXSIZ along an edge in model space.
    //
    save.XVMIN = (save.XMIN / save.CVXSIZ);
    save.YVMIN = (save.YMIN / save.CVXSIZ);
    save.ZVMIN = (save.ZMIN / save.CVXSIZ);
    save.XVMAX = (save.XMAX / save.CVXSIZ);
    save.YVMAX = (save.YMAX / save.CVXSIZ);
    save.ZVMAX = (save.ZMAX / save.CVXSIZ);

    //
    // Extend the coarse voxel grid by at least 1/2
    // coarse voxel length along each degree of freedom.
    //
    save.XVMIN = f64::round((save.XVMIN - 1.0));
    save.YVMIN = f64::round((save.YVMIN - 1.0));
    save.ZVMIN = f64::round((save.ZVMIN - 1.0));
    save.XVMAX = f64::round((save.XVMAX + 1.0));
    save.YVMAX = f64::round((save.YVMAX + 1.0));
    save.ZVMAX = f64::round((save.ZVMAX + 1.0));

    //
    // Calculate the coarse voxel grid origin in model units.
    //
    VOXORI[1] = (save.XVMIN * save.CVXSIZ);
    VOXORI[2] = (save.YVMIN * save.CVXSIZ);
    VOXORI[3] = (save.ZVMIN * save.CVXSIZ);

    //
    // Calculate the dimension of the voxel grid in
    // units of (regular) voxels.
    //
    save.NX = (intrinsics::IDNINT((save.XVMAX - save.XVMIN)) * CGSCAL);
    save.NY = (intrinsics::IDNINT((save.YVMAX - save.YVMIN)) * CGSCAL);
    save.NZ = (intrinsics::IDNINT((save.ZVMAX - save.ZVMIN)) * CGSCAL);

    NVOX[1] = save.NX;
    NVOX[2] = save.NY;
    NVOX[3] = save.NZ;

    *NVXTOT = ((save.NX * save.NY) * save.NZ);
    //
    // Make sure the number of voxels NVXTOT is within range.
    //
    if (*NVXTOT > MAXVOX) {
        SETMSG(
            b"Fine voxel count NVXTOT = #; count must be in the range 1:#.",
            ctx,
        );
        ERRINT(b"#", *NVXTOT, ctx);
        ERRINT(b"#", MAXVOX, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZMKSPIN", ctx)?;
        return Ok(());
    }

    //
    // Check the coarse voxel scale. It must be at least 1, and its
    // cube must not exceed the fine voxel count.
    //
    if ((CGSCAL < 1) || ((CGSCAL as f64) > f64::powf(*NVXTOT as f64, (1.0 / 3 as f64)))) {
        SETMSG(b"Coarse voxel scale = #; scale must be in the range 1:NVXTOT**3, where NVXTOT is the total fine voxel count. In this case, NVXTOT = #.", ctx);
        ERRINT(b"#", CGSCAL, ctx);
        ERRINT(b"#", *NVXTOT, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZMKSPIN", ctx)?;
        return Ok(());
    }

    //
    // The cube of the coarse scale must divide the total voxel count
    // evenly. This is a consistency check: the code that derives the
    // voxel grid dimensions should ensure this condition is met.
    //
    save.Q = (*NVXTOT / intrinsics::pow(CGSCAL, 3));
    save.R = (*NVXTOT - (save.Q * intrinsics::pow(CGSCAL, 3)));

    if (save.R != 0) {
        SETMSG(b"Coarse voxel scale = #; the cube of the scale must divide NVXTOT evenly, where NVXTOT is the total  fine voxel count. In this case, NVXTOT = #.", ctx);
        ERRINT(b"#", CGSCAL, ctx);
        ERRINT(b"#", *NVXTOT, ctx);
        SIGERR(b"SPICE(INCOMPATIBLESCALE)", ctx)?;
        CHKOUT(b"ZZMKSPIN", ctx)?;
        return Ok(());
    }

    //
    // Check the number of coarse voxels.
    //
    save.NPCG = intrinsics::pow(CGSCAL, 3);

    save.NCGFLG = (*NVXTOT / save.NPCG);

    if (save.NCGFLG > MAXCGR) {
        SETMSG(b"Number of coarse voxels # exceeds limit #. Increase coarse voxel scale, fine voxel scale, or both.", ctx);
        ERRINT(b"#", save.NCGFLG, ctx);
        ERRINT(b"#", MAXCGR, ctx);
        SIGERR(b"SPICE(COARSEGRIDOVERFLOW)", ctx)?;
        CHKOUT(b"ZZMKSPIN", ctx)?;
        return Ok(());
    }

    //
    // Enumerate all voxels that each plate might intersect.
    //
    ZZINILNK(
        MAXPTR,
        MXCELL,
        &mut save.NCELL,
        VXPTR.as_slice_mut(),
        CELLS.as_slice_mut(),
        ctx,
    )?;

    //
    // Set the dimensions of the coarse grid.
    //
    save.CGRDIM[1] = (save.NX / CGSCAL);
    save.CGRDIM[2] = (save.NY / CGSCAL);
    save.CGRDIM[3] = (save.NZ / CGSCAL);

    CLEARI(save.NCGFLG, CGRPTR.as_slice_mut());

    //
    // TO points to the first free location in the VXPTR array.
    //
    save.TO = 1;

    for I in 1..=NP {
        //
        // Find the extents of the Ith plate, where the extents
        // are expanded by TOL in each direction. We truncate
        // the expanded box at a distance of MDLTOL beyond the
        // extents of the vertex set, if necessary.
        //
        save.XP[1] = VRTCES[[1, PLATES[[1, I]]]];
        save.XP[2] = VRTCES[[1, PLATES[[2, I]]]];
        save.XP[3] = VRTCES[[1, PLATES[[3, I]]]];

        save.YP[1] = VRTCES[[2, PLATES[[1, I]]]];
        save.YP[2] = VRTCES[[2, PLATES[[2, I]]]];
        save.YP[3] = VRTCES[[2, PLATES[[3, I]]]];

        save.ZP[1] = VRTCES[[3, PLATES[[1, I]]]];
        save.ZP[2] = VRTCES[[3, PLATES[[2, I]]]];
        save.ZP[3] = VRTCES[[3, PLATES[[3, I]]]];

        save.BXMIN = BRCKTD(
            (intrinsics::DMIN1(&[save.XP[1], save.XP[2], save.XP[3]]) - save.MDLTOL),
            save.XEXTNT[1],
            save.XEXTNT[2],
        );

        save.BXMAX = BRCKTD(
            (intrinsics::DMAX1(&[save.XP[1], save.XP[2], save.XP[3]]) + save.MDLTOL),
            save.XEXTNT[1],
            save.XEXTNT[2],
        );

        save.BYMIN = BRCKTD(
            (intrinsics::DMIN1(&[save.YP[1], save.YP[2], save.YP[3]]) - save.MDLTOL),
            save.XEXTNT[3],
            save.XEXTNT[4],
        );

        save.BYMAX = BRCKTD(
            (intrinsics::DMAX1(&[save.YP[1], save.YP[2], save.YP[3]]) + save.MDLTOL),
            save.XEXTNT[3],
            save.XEXTNT[4],
        );

        save.BZMIN = BRCKTD(
            (intrinsics::DMIN1(&[save.ZP[1], save.ZP[2], save.ZP[3]]) - save.MDLTOL),
            save.XEXTNT[5],
            save.XEXTNT[6],
        );

        save.BZMAX = BRCKTD(
            (intrinsics::DMAX1(&[save.ZP[1], save.ZP[2], save.ZP[3]]) + save.MDLTOL),
            save.XEXTNT[5],
            save.XEXTNT[6],
        );

        //
        // Find the range of voxel coordinates that contain the bounding
        // box of the plate. All we need look at are the coordinates
        // of the two corners having minimum and maximum coordinates.
        //
        // Start with the corner having minimum coordinates:
        //
        VPACK(save.BXMIN, save.BYMIN, save.BZMIN, save.VMOD.as_slice_mut());

        ZZGETVOX(
            *VOXSIZ,
            VOXORI.as_slice(),
            NVOX.as_slice(),
            save.VMOD.as_slice(),
            &mut save.INBOX,
            save.VCOORD.as_slice_mut(),
            ctx,
        )?;

        if !save.INBOX {
            //
            // A corner of the bounding box lies outside the voxel grid.
            // This should never occur.
            //
            SETMSG(b"BUG: bounding box of plate is outside of voxel grid. Input coordinates were (#, #, #). Plate ID = #.", ctx);
            ERRDP(b"#", save.VMOD[1], ctx);
            ERRDP(b"#", save.VMOD[2], ctx);
            ERRDP(b"#", save.VMOD[3], ctx);
            ERRINT(b"#", I, ctx);
            SIGERR(b"SPICE(BUG)", ctx)?;
            CHKOUT(b"ZZMKSPIN", ctx)?;
            return Ok(());
        }

        //
        // Unpack minimum voxel coordinates from VCOORD.
        //
        save.GXMIN = save.VCOORD[1];
        save.GYMIN = save.VCOORD[2];
        save.GZMIN = save.VCOORD[3];

        //
        // Now handle the corner having maximum coordinates:
        //
        VPACK(save.BXMAX, save.BYMAX, save.BZMAX, save.VMOD.as_slice_mut());

        ZZGETVOX(
            *VOXSIZ,
            VOXORI.as_slice(),
            NVOX.as_slice(),
            save.VMOD.as_slice(),
            &mut save.INBOX,
            save.VCOORD.as_slice_mut(),
            ctx,
        )?;

        if !save.INBOX {
            //
            // A corner of the bounding box lies outside the voxel grid.
            // This should never occur.
            //
            SETMSG(b"BUG: bounding box of plate is outside of voxel grid. Input coordinates were (#, #, #). Plate ID = #.", ctx);
            ERRDP(b"#", save.VMOD[1], ctx);
            ERRDP(b"#", save.VMOD[2], ctx);
            ERRDP(b"#", save.VMOD[3], ctx);
            ERRINT(b"#", I, ctx);
            SIGERR(b"SPICE(BUG)", ctx)?;
            CHKOUT(b"ZZMKSPIN", ctx)?;
            return Ok(());
        }

        //
        // Unpack maximum voxel coordinates from VCOORD.
        //
        save.GXMAX = save.VCOORD[1];
        save.GYMAX = save.VCOORD[2];
        save.GZMAX = save.VCOORD[3];

        //
        // Determine voxels that the bounding box of the plate
        // intersects.
        //
        for IZ in save.GZMIN..=save.GZMAX {
            for IY in save.GYMIN..=save.GYMAX {
                for IX in save.GXMIN..=save.GXMAX {
                    save.VIXYZ[1] = IX;
                    save.VIXYZ[2] = IY;
                    save.VIXYZ[3] = IZ;
                    //
                    // Find the coarse voxel containing this voxel, and
                    // compute the offset of this voxel within the coarse
                    // voxel. The output CGXYZ contains the 3-dimensional
                    // coordinates of the coarse voxel within the coarse
                    // grid. The output CGOF1D is the 1-based,
                    // 1-dimensional offset of the current voxel (having
                    // coordinates VIXYZ) from the start of the coarse
                    // voxel.
                    //
                    ZZVOXCVO(
                        save.VIXYZ.as_slice(),
                        NVOX.as_slice(),
                        CGSCAL,
                        save.CGXYZ.as_slice_mut(),
                        save.CGOFF.as_slice_mut(),
                        &mut save.CGOF1D,
                        ctx,
                    )?;

                    if FAILED(ctx) {
                        CHKOUT(b"ZZMKSPIN", ctx)?;
                        return Ok(());
                    }

                    save.CVID = ZZVOX2ID(save.CGXYZ.as_slice(), save.CGRDIM.as_slice());

                    if (CGRPTR[save.CVID] == 0) {
                        //
                        // The coarse voxel at index CVID is empty so far.
                        // Allocate CGSCAL pointers for it in the VXPTR
                        // array; make the coarse voxel point to the first
                        // element of this sub-array.
                        //
                        CGRPTR[save.CVID] = save.TO;
                        save.TO = (save.TO + save.NPCG);
                    }
                    //
                    // Let IXPTR be the index in the VXPTR array of the
                    // pointer for the current voxel.
                    //
                    save.IXPTR = ((CGRPTR[save.CVID] - 1) + save.CGOF1D);

                    ZZADDLNK(
                        save.IXPTR,
                        I,
                        MAXPTR,
                        MXCELL,
                        VXPTR.as_slice_mut(),
                        &mut save.NCELL,
                        CELLS.as_slice_mut(),
                        ctx,
                    )?;

                    if FAILED(ctx) {
                        CHKOUT(b"ZZMKSPIN", ctx)?;
                        return Ok(());
                    }
                }
            }
        }
    }

    *NVXPTR = (save.TO - 1);

    //
    // Generate two linked lists mapping voxel ID to the plates enclosed
    // within that voxel (if any).
    //
    // VXPTR : An array, indexed by voxel ID. For an array element,
    //         VXPTR(VOX_ID), greater than zero, the value identifies an
    //         index in VXLIST, the value of that VXLIST array element
    //         equaling the number of plates contained in the voxel
    //         specified by the ID. The condition VXPTR(VOX_ID) = -1
    //         indicates the voxel contains no plates.
    //
    // VXLIST: An array, indexed by the positive entries in VXPTR. The
    //         element, N, identified by a VXPTR value describes the
    //         number of plates contained in the corresponding voxel.
    //
    //             N = VXLIST( VXPTR(VOX_ID) )
    //
    //         The N elements following VXLIST( VXPTR(VOX_ID) ),
    //         contain the IDs of those plates within the voxel.
    //
    ZZUNTNGL(
        *NVXPTR,
        MXCELL,
        CELLS.as_slice(),
        MAXVXL,
        VXPTR.as_slice_mut(),
        NVXLST,
        VXLIST.as_slice_mut(),
        ctx,
    )?;

    CHKOUT(b"ZZMKSPIN", ctx)?;
    Ok(())
}
