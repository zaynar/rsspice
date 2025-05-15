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
const VTIGHT: f64 = 0.00000000000001;
const FILSIZ: i32 = 255;
const FRNMLN: i32 = 32;
const MXNVOX: i32 = 2100;
const MAXV: i32 = 3000;
const MAXP: i32 = 2000;
const LNSIZE: i32 = 320;
const MAXIXI: i32 = (MAXCGR + (10 * MAXP));
const NSYS: i32 = 3;
const WORKSZ: i32 = (MAXP * 100);

struct SaveVars {
    DSK: Vec<u8>,
    DSK1: Vec<u8>,
    FRAME: Vec<u8>,
    LABEL: Vec<u8>,
    TITLE: Vec<u8>,
    CORPAR: StackArray<f64, 10>,
    FINSCL: f64,
    FIRST: f64,
    LAST: f64,
    MNCOR1: f64,
    MNCOR2: f64,
    MNCOR3: f64,
    MXCOR1: f64,
    MXCOR2: f64,
    MXCOR3: f64,
    ORIGIN: StackArray<f64, 3>,
    RAYDIR: StackArray<f64, 3>,
    SPAIXD: StackArray<f64, 10>,
    VERTS: ActualArray2D<f64>,
    VERTEX: StackArray<f64, 3>,
    VOXSIZ: f64,
    VTXBDS: StackArray2D<f64, 6>,
    XOFF: f64,
    XPT: StackArray<f64, 3>,
    XXPT: StackArray<f64, 3>,
    ZOFF: f64,
    CENTER: i32,
    CGRPTR: ActualArray<i32>,
    CGRSCL: i32,
    CORSCL: i32,
    CORSYS: i32,
    SYSTMS: StackArray<i32, 3>,
    DCLASS: i32,
    DLADSC: StackArray<i32, 8>,
    HAN1: i32,
    HANDLE: i32,
    I: i32,
    J: i32,
    N: i32,
    NCVOX: i32,
    NP: i32,
    NV: i32,
    NVOX: i32,
    PLATE: StackArray<i32, 3>,
    PLATES: ActualArray2D<i32>,
    PLID: i32,
    PRVDSC: StackArray<i32, 8>,
    SEGNO: i32,
    SPAIXI: ActualArray<i32>,
    SURFCE: i32,
    TO: i32,
    VGREXT: StackArray<i32, 3>,
    VOXLSZ: i32,
    VOXNPL: i32,
    VOXNPT: i32,
    VOXPLT: ActualArray<i32>,
    VOXPTR: ActualArray<i32>,
    VOXPSZ: i32,
    WORK: ActualArray2D<i32>,
    XPLATE: StackArray<i32, 3>,
    XPLID: i32,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut DSK = vec![b' '; FILSIZ as usize];
        let mut DSK1 = vec![b' '; FILSIZ as usize];
        let mut FRAME = vec![b' '; FRNMLN as usize];
        let mut LABEL = vec![b' '; LNSIZE as usize];
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut CORPAR = StackArray::<f64, 10>::new(1..=NSYPAR);
        let mut FINSCL: f64 = 0.0;
        let mut FIRST: f64 = 0.0;
        let mut LAST: f64 = 0.0;
        let mut MNCOR1: f64 = 0.0;
        let mut MNCOR2: f64 = 0.0;
        let mut MNCOR3: f64 = 0.0;
        let mut MXCOR1: f64 = 0.0;
        let mut MXCOR2: f64 = 0.0;
        let mut MXCOR3: f64 = 0.0;
        let mut ORIGIN = StackArray::<f64, 3>::new(1..=3);
        let mut RAYDIR = StackArray::<f64, 3>::new(1..=3);
        let mut SPAIXD = StackArray::<f64, 10>::new(1..=IXDFIX);
        let mut VERTS = ActualArray2D::<f64>::new(1..=3, 1..=MAXV);
        let mut VERTEX = StackArray::<f64, 3>::new(1..=3);
        let mut VOXSIZ: f64 = 0.0;
        let mut VTXBDS = StackArray2D::<f64, 6>::new(1..=2, 1..=3);
        let mut XOFF: f64 = 0.0;
        let mut XPT = StackArray::<f64, 3>::new(1..=3);
        let mut XXPT = StackArray::<f64, 3>::new(1..=3);
        let mut ZOFF: f64 = 0.0;
        let mut CENTER: i32 = 0;
        let mut CGRPTR = ActualArray::<i32>::new(1..=MAXCGR);
        let mut CGRSCL: i32 = 0;
        let mut CORSCL: i32 = 0;
        let mut CORSYS: i32 = 0;
        let mut SYSTMS = StackArray::<i32, 3>::new(1..=NSYS);
        let mut DCLASS: i32 = 0;
        let mut DLADSC = StackArray::<i32, 8>::new(1..=DLADSZ);
        let mut HAN1: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut I: i32 = 0;
        let mut J: i32 = 0;
        let mut N: i32 = 0;
        let mut NCVOX: i32 = 0;
        let mut NP: i32 = 0;
        let mut NV: i32 = 0;
        let mut NVOX: i32 = 0;
        let mut PLATE = StackArray::<i32, 3>::new(1..=3);
        let mut PLATES = ActualArray2D::<i32>::new(1..=3, 1..=MAXP);
        let mut PLID: i32 = 0;
        let mut PRVDSC = StackArray::<i32, 8>::new(1..=DLADSZ);
        let mut SEGNO: i32 = 0;
        let mut SPAIXI = ActualArray::<i32>::new(1..=MAXIXI);
        let mut SURFCE: i32 = 0;
        let mut TO: i32 = 0;
        let mut VGREXT = StackArray::<i32, 3>::new(1..=3);
        let mut VOXLSZ: i32 = 0;
        let mut VOXNPL: i32 = 0;
        let mut VOXNPT: i32 = 0;
        let mut VOXPLT = ActualArray::<i32>::new(1..=MXNVOX);
        let mut VOXPTR = ActualArray::<i32>::new(1..=MXNVOX);
        let mut VOXPSZ: i32 = 0;
        let mut WORK = ActualArray2D::<i32>::new(1..=2, 1..=WORKSZ);
        let mut XPLATE = StackArray::<i32, 3>::new(1..=3);
        let mut XPLID: i32 = 0;
        let mut FOUND: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(LATSYS), Val::I(RECSYS), Val::I(PDTSYS)].into_iter();
            SYSTMS
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            DSK,
            DSK1,
            FRAME,
            LABEL,
            TITLE,
            CORPAR,
            FINSCL,
            FIRST,
            LAST,
            MNCOR1,
            MNCOR2,
            MNCOR3,
            MXCOR1,
            MXCOR2,
            MXCOR3,
            ORIGIN,
            RAYDIR,
            SPAIXD,
            VERTS,
            VERTEX,
            VOXSIZ,
            VTXBDS,
            XOFF,
            XPT,
            XXPT,
            ZOFF,
            CENTER,
            CGRPTR,
            CGRSCL,
            CORSCL,
            CORSYS,
            SYSTMS,
            DCLASS,
            DLADSC,
            HAN1,
            HANDLE,
            I,
            J,
            N,
            NCVOX,
            NP,
            NV,
            NVOX,
            PLATE,
            PLATES,
            PLID,
            PRVDSC,
            SEGNO,
            SPAIXI,
            SURFCE,
            TO,
            VGREXT,
            VOXLSZ,
            VOXNPL,
            VOXNPT,
            VOXPLT,
            VOXPTR,
            VOXPSZ,
            WORK,
            XPLATE,
            XPLID,
            FOUND,
        }
    }
}

//$Procedure F_DSKX02 ( DSKX02 tests )
pub fn F_DSKX02(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //

    //
    // Number of supported coordinate systems.
    //

    //
    // Workspace and index sizes
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
    testutil::TOPEN(b"F_DSKX02", ctx)?;

    //**********************************************************************
    //
    //     Normal cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create DSK containing type 2 segment", ctx)?;

    //
    // We're going to create a plate model and associated spatial
    // index data structures entirely by hand.
    //
    // The voxel grid has voxel extents
    //
    //    4 x 4 x 4
    //
    // The coarse voxel scale is 2.
    //
    // The edge length of each voxel is 10 km.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.VGREXT[save.I] = 4;
            save.I += m3__;
        }
    }

    save.CGRSCL = 2;
    save.VOXSIZ = 10.0;

    save.NCVOX = 8;

    //
    // The voxel grid is centered at the reference frame center.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.ORIGIN[save.I] = -(((save.VGREXT[save.I] as f64) * save.VOXSIZ) / 2 as f64);
            save.I += m3__;
        }
    }

    //
    // Set up plates and vertices:
    //
    //    Voxel (1,1,1) contains a plate facing in the -X direction:
    //
    save.VERTS[[1, 1]] = -19.0;
    save.VERTS[[2, 1]] = -18.0;
    save.VERTS[[3, 1]] = -19.0;

    save.VERTS[[1, 2]] = -19.0;
    save.VERTS[[2, 2]] = -19.0;
    save.VERTS[[3, 2]] = -19.0;

    save.VERTS[[1, 3]] = -19.0;
    save.VERTS[[2, 3]] = -18.5;
    save.VERTS[[3, 3]] = -18.0;

    save.PLATES[[1, 1]] = 1;
    save.PLATES[[2, 1]] = 2;
    save.PLATES[[3, 1]] = 3;

    //
    // Add a plate that should be occulted by plate 1, as seen from
    // the -X direction.
    //
    // Plate 2 is a copy of plate 1, shifted by 1 in the +X direction.
    //
    // Vertices 4-6 define this plate.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.VERTS[[1, (save.I + 3)]] = (save.VERTS[[1, save.I]] + 1.0);

            {
                let m1__: i32 = 2;
                let m2__: i32 = 3;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.VERTS[[save.J, (save.I + 3)]] = save.VERTS[[save.J, save.I]];
                    save.J += m3__;
                }
            }

            save.I += m3__;
        }
    }

    save.PLATES[[1, 2]] = 4;
    save.PLATES[[2, 2]] = 5;
    save.PLATES[[3, 2]] = 6;

    //
    // Add a plate that normally should occult plate 1, as seen from
    // the -X direction. However, this plate will be "owned" by voxel 2,
    // which is voxel 1's neighbor on the +X side. Thus this plate should
    // be examined as a target only if plates 1 and 2 are missed.
    //
    //
    // Plate 3 is a copy of plate 1, shifted by 1/2 in the -X direction.
    //
    // Vertices 7-9 define this plate.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.VERTS[[1, (save.I + 6)]] = (save.VERTS[[1, save.I]] - 0.5);

            {
                let m1__: i32 = 2;
                let m2__: i32 = 3;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.VERTS[[save.J, (save.I + 6)]] = save.VERTS[[save.J, save.I]];
                    save.J += m3__;
                }
            }

            save.I += m3__;
        }
    }

    save.PLATES[[1, 3]] = 7;
    save.PLATES[[2, 3]] = 8;
    save.PLATES[[3, 3]] = 9;

    //
    // Add a plate that normally should occult plate 1, as seen from
    // the -X direction. However, this plate has its vertex orientation
    // reversed.
    //
    // Vertices 10-12 define this plate.
    //
    save.VERTS[[1, 10]] = (save.VERTS[[1, 1]] - 0.75);
    save.VERTS[[1, 11]] = save.VERTS[[1, 10]];
    save.VERTS[[1, 12]] = save.VERTS[[1, 10]];

    //
    // The Y component of vertex 10 is that of vertex 2;
    // the Y component of vertex 11 is that of vertex 1.
    //
    save.VERTS[[2, 10]] = save.VERTS[[2, 2]];
    save.VERTS[[2, 11]] = save.VERTS[[2, 1]];
    save.VERTS[[2, 12]] = save.VERTS[[2, 3]];

    save.VERTS[[3, 10]] = save.VERTS[[3, 1]];
    save.VERTS[[3, 11]] = save.VERTS[[3, 2]];
    save.VERTS[[3, 12]] = save.VERTS[[3, 3]];

    save.PLATES[[1, 4]] = 10;
    save.PLATES[[2, 4]] = 11;
    save.PLATES[[3, 4]] = 12;

    //
    // Add a plate on the opposite side of the grid from plate 1,
    // but also facing in the same direction as plate one. Offset
    // this plate in the +Z direction by 1 km.
    //
    // This plate is owned by voxel 4, which is contained in
    // coarse voxel 2.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.VERTS[[1, (save.I + 12)]] = (save.VERTS[[1, 1]] + ((3 as f64) * save.VOXSIZ));

            save.VERTS[[2, (save.I + 12)]] = save.VERTS[[2, save.I]];

            save.VERTS[[3, (save.I + 12)]] = (save.VERTS[[3, save.I]] + 1.0);

            save.I += m3__;
        }
    }

    save.PLATES[[1, 5]] = 13;
    save.PLATES[[2, 5]] = 14;
    save.PLATES[[3, 5]] = 15;

    save.NV = 15;
    save.NP = 5;

    //
    // Compute vertex extents.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.VTXBDS[[1, save.I]] = spicelib::DPMAX();
            save.VTXBDS[[2, save.I]] = spicelib::DPMIN();
            save.I += m3__;
        }
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NV;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = 3;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.VTXBDS[[1, save.J]] = intrinsics::DMIN1(&[
                        save.VTXBDS[[1, save.J]],
                        save.VERTS[[save.J, save.I]],
                    ]);
                    save.VTXBDS[[2, save.J]] = intrinsics::DMAX1(&[
                        save.VTXBDS[[2, save.J]],
                        save.VERTS[[save.J, save.I]],
                    ]);

                    save.J += m3__;
                }
            }

            save.I += m3__;
        }
    }

    //
    // Set the voxel-plate mapping.
    //
    spicelib::CLEARI(MXNVOX, save.VOXPLT.as_slice_mut());

    //
    // Initialize all pointers to null.
    //
    spicelib::FILLI(-1, MXNVOX, save.VOXPTR.as_slice_mut());
    //
    // The voxel pointer list must contain an entry for
    // each voxel belonging to a non-empty coarse voxel.
    // Coarse voxels 1 and 2 are non-empty.
    //
    save.VOXNPT = 16;

    //
    // The first voxel contains plates 1 and 2. The pointer
    // associated with this voxel points to the start of the
    // voxel's plate list.
    //
    save.VOXPTR[1] = 1;

    //
    // The second voxel contains plate 3. The pointer
    // associated with this voxel points to the start of the
    // voxel's plate list, which follows the list for voxel 1.
    //
    save.VOXPTR[2] = 5;

    //
    // The fourth voxel contains plate 5. The pointer
    // associated with this voxel points to the start of the
    // voxel's plate list, which follows the list for voxel 2.
    // Note that the voxel pointer list contains 6 null entries
    // following the entry for voxel 2, since there are entries
    // for every voxel in the coarse voxel containing voxel 1.
    //
    save.VOXPTR[10] = 7;

    //
    // The first element of a plate list is the count.
    // The plate numbers follow.
    //
    // Entries for voxel 1:
    //
    save.VOXPLT[1] = 3;
    save.VOXPLT[2] = 1;
    save.VOXPLT[3] = 2;
    save.VOXPLT[4] = 4;
    //
    // Entries for voxel 2:
    //
    save.VOXPLT[5] = 1;
    save.VOXPLT[6] = 3;

    //
    // Entries for voxel 4:
    //
    save.VOXPLT[7] = 1;
    save.VOXPLT[8] = 5;

    //
    // VOXNPL is the number of elements in the plate list, including
    // plate counts.
    //
    save.VOXNPL = 8;

    //
    // The first coarse voxel is non-empty and points
    // to the voxel pointer for the first voxel.
    //
    // The second coarse voxel is non-empty and points
    // to the voxel pointer for the fourth voxel.
    //
    // The rest of the coarse grid contains null values.
    //
    spicelib::FILLI(0, save.NCVOX, save.CGRPTR.as_slice_mut());

    save.CGRPTR[1] = 1;
    save.CGRPTR[2] = 9;

    //
    // We will not use the vertex-plate list.
    //

    //
    // Pack the integer component of the spatial index array.
    //
    spicelib::MOVEI(save.VGREXT.as_slice(), 3, save.SPAIXI.subarray_mut(SIVGRX));

    save.SPAIXI[SICGSC] = save.CGRSCL;
    save.SPAIXI[SIVXNP] = save.VOXNPT;
    save.SPAIXI[SIVXNL] = save.VOXNPL;
    save.SPAIXI[SIVTNL] = 0;

    spicelib::MOVEI(
        save.CGRPTR.as_slice(),
        MAXCGR,
        save.SPAIXI.subarray_mut(SICGRD),
    );

    save.I = (SICGRD + MAXCGR);

    spicelib::MOVEI(
        save.VOXPTR.as_slice(),
        save.VOXNPT,
        save.SPAIXI.subarray_mut(save.I),
    );

    save.I = (save.I + save.VOXNPT);

    spicelib::MOVEI(
        save.VOXPLT.as_slice(),
        save.VOXNPL,
        save.SPAIXI.subarray_mut(save.I),
    );

    //
    // Pack the d.p. component of the spatial index array.
    //
    spicelib::MOVED(save.VTXBDS.as_slice(), 6, save.SPAIXD.subarray_mut(SIVTBD));
    spicelib::VEQU(save.ORIGIN.as_slice(), save.SPAIXD.subarray_mut(SIVXOR));

    save.SPAIXD[SIVXSZ] = save.VOXSIZ;

    //
    // Set up ID and coverage parameters for the segment. These
    // are required in order to create a DSK segment but have no
    // other use for the tests performed here.
    //
    save.SURFCE = 499;
    save.CENTER = 4;
    save.DCLASS = 2;
    fstr::assign(&mut save.FRAME, b"IAU_MARS");
    save.CORSYS = RECSYS;
    save.MNCOR1 = save.VTXBDS[[1, 1]];
    save.MXCOR1 = save.VTXBDS[[2, 1]];
    save.MNCOR2 = save.VTXBDS[[1, 2]];
    save.MXCOR2 = save.VTXBDS[[2, 2]];
    save.MNCOR3 = save.VTXBDS[[1, 3]];
    save.MXCOR3 = save.VTXBDS[[2, 3]];

    save.FIRST = -1.0;
    save.LAST = 1.0;

    //
    // Open a new DSK file.
    //
    fstr::assign(&mut save.DSK, b"dskx02.bds");

    if spicelib::EXISTS(&save.DSK, ctx)? {
        spicelib::DELFIL(&save.DSK, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::DSKOPN(&save.DSK, &save.DSK, 0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Write the segment.
    //
    spicelib::DSKW02(
        save.HANDLE,
        save.CENTER,
        save.SURFCE,
        save.DCLASS,
        &save.FRAME,
        save.CORSYS,
        save.CORPAR.as_slice(),
        save.MNCOR1,
        save.MXCOR1,
        save.MNCOR2,
        save.MXCOR2,
        save.MNCOR3,
        save.MXCOR3,
        save.FIRST,
        save.LAST,
        save.NV,
        save.VERTS.as_slice(),
        save.NP,
        save.PLATES.as_slice(),
        save.SPAIXD.as_slice(),
        save.SPAIXI.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DSKCLS(save.HANDLE, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Find ray intercept on plate 1; ray vertex is on the -X side.",
        ctx,
    )?;

    spicelib::DASOPR(&save.DSK, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DLABFS(
        save.HANDLE,
        save.DLADSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The ray vertex is on the -X side of the grid, and the ray
    // points strictly in the +X direction.
    //
    save.VERTEX[1] = -((3 as f64) * save.VOXSIZ);
    save.VERTEX[2] = ((save.VERTS[[2, 1]] + save.VERTS[[2, 2]]) / 2 as f64);
    save.VERTEX[3] = ((save.VERTS[[3, 1]] + save.VERTS[[3, 3]]) / 2 as f64);

    save.RAYDIR[1] = 1.0;
    save.RAYDIR[2] = 0.0;
    save.RAYDIR[3] = 0.0;

    spicelib::DSKX02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        &mut save.PLID,
        save.XPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    if save.FOUND {
        testutil::CHCKSI(b"PLID", save.PLID, b"=", 1, 0, OK, ctx)?;

        spicelib::VEQU(save.VERTEX.as_slice(), save.XXPT.as_slice_mut());
        save.XXPT[1] = save.VERTS[[1, 1]];

        testutil::CHCKAD(
            b"XPT",
            save.XPT.as_slice(),
            b"~~/",
            save.XXPT.as_slice(),
            3,
            VTIGHT,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Find ray intercept on plate 4; ray vertex is on the +X side.",
        ctx,
    )?;

    //
    // The ray vertex is on the +X side of the grid, and the ray
    // points strictly in the -X direction.
    //
    // Plates other than plate 4 should be invisible due to their
    // vertex orientation.
    //
    //
    save.VERTEX[1] = ((3 as f64) * save.VOXSIZ);
    save.VERTEX[2] = ((save.VERTS[[2, 1]] + save.VERTS[[2, 2]]) / 2 as f64);
    save.VERTEX[3] = ((save.VERTS[[3, 1]] + save.VERTS[[3, 3]]) / 2 as f64);

    save.RAYDIR[1] = -1.0;
    save.RAYDIR[2] = 0.0;
    save.RAYDIR[3] = 0.0;

    spicelib::DSKX02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        &mut save.PLID,
        save.XPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    if save.FOUND {
        testutil::CHCKSI(b"PLID", save.PLID, b"=", 4, 0, OK, ctx)?;

        spicelib::VEQU(save.VERTEX.as_slice(), save.XXPT.as_slice_mut());
        save.XXPT[1] = save.VERTS[[1, 10]];

        testutil::CHCKAD(
            b"XPT",
            save.XPT.as_slice(),
            b"~~/",
            save.XXPT.as_slice(),
            3,
            VTIGHT,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Find ray intercept on plate 5; ray vertex is on the -X side.",
        ctx,
    )?;

    //
    // This case requires examination of the second group of
    // voxels within the DSKX02 loop that works with one voxel
    // group at a time.
    //
    // The ray vertex is on the -X side of the grid, and the ray
    // points strictly in the +X direction.
    //
    save.VERTEX[1] = -((3 as f64) * save.VOXSIZ);
    save.VERTEX[2] = ((save.VERTS[[2, 13]] + save.VERTS[[2, 14]]) / 2 as f64);
    save.VERTEX[3] = ((save.VERTS[[3, 13]] + save.VERTS[[3, 15]]) / 2 as f64);

    save.RAYDIR[1] = 1.0;
    save.RAYDIR[2] = 0.0;
    save.RAYDIR[3] = 0.0;

    spicelib::DSKX02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        &mut save.PLID,
        save.XPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    if save.FOUND {
        testutil::CHCKSI(b"PLID", save.PLID, b"=", 5, 0, OK, ctx)?;

        spicelib::VEQU(save.VERTEX.as_slice(), save.XXPT.as_slice_mut());
        save.XXPT[1] = save.VERTS[[1, 13]];

        testutil::CHCKAD(
            b"XPT",
            save.XPT.as_slice(),
            b"~~/",
            save.XXPT.as_slice(),
            3,
            VTIGHT,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Shoot a ray through empty voxels in coarse voxel 1.", ctx)?;

    //
    // Voxels above the bottom layer are empty. In coarse voxel 1,
    // the voxel IDs of the upper layer are
    //
    //    17, 18, 21, 22
    //
    //
    //
    save.VERTEX[1] = -(2.5 * save.VOXSIZ);
    save.VERTEX[2] = -(0.5 * save.VOXSIZ);
    save.VERTEX[3] = -(0.5 * save.VOXSIZ);

    save.RAYDIR[1] = 1.0;
    save.RAYDIR[2] = -1.0;
    save.RAYDIR[3] = 0.0;

    spicelib::DSKX02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        &mut save.PLID,
        save.XPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Shoot a ray through empty coarse voxels on the top layer of the grid.",
        ctx,
    )?;

    save.VERTEX[1] = -((3 as f64) * save.VOXSIZ);
    save.VERTEX[2] = (0.5 * save.VOXSIZ);
    save.VERTEX[3] = (1.5 * save.VOXSIZ);

    save.RAYDIR[1] = 1.0;
    save.RAYDIR[2] = 0.0;
    save.RAYDIR[3] = 0.0;

    spicelib::DSKX02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        &mut save.PLID,
        save.XPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Add a segment for testing handling of duplicate plates.
    //
    testutil::TCASE(b"Add second segment to DSK.", ctx)?;

    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASOPW(&save.DSK, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The voxel grid has voxel extents
    //
    //    100 x 1 x 1
    //
    // The coarse voxel scale is 1.
    //
    // The edge length of each voxel is 10 km.
    //
    save.VGREXT[1] = 100;
    save.VGREXT[2] = 1;
    save.VGREXT[3] = 1;

    save.CGRSCL = 1;
    save.VOXSIZ = 10.0;

    save.NCVOX = 100;

    //
    // The voxel grid origin coincides with the reference frame center.
    //
    spicelib::CLEARD(3, save.ORIGIN.as_slice_mut());

    //
    // Voxel 1 contains 10 plates.
    //
    // Each other voxel "owns" the plates in the voxel behind it in the
    // -X direction, plus its own plates. The plates physically
    // contained in voxel `n' are shifted in the +Z direction by (n-1)
    // km relative to the plate set in voxel 1.
    //

    //
    // Create the vertices and plates for the first voxel.
    //
    // The vertices are orders to as to make the normal vectors point
    // in the -X direction.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = 10;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.J = (3 * (save.I - 1));

            save.VERTS[[1, (save.J + 1)]] = 5.0;
            save.VERTS[[2, (save.J + 1)]] = (save.I as f64);
            save.VERTS[[3, (save.J + 1)]] = 0.0;

            save.VERTS[[1, (save.J + 2)]] = 5.0;
            save.VERTS[[2, (save.J + 2)]] = ((save.I - 1) as f64);
            save.VERTS[[3, (save.J + 2)]] = 0.0;

            save.VERTS[[1, (save.J + 3)]] = 5.0;
            save.VERTS[[2, (save.J + 3)]] = ((save.I as f64) - 0.5);
            save.VERTS[[3, (save.J + 3)]] = 1.0;

            for K in 1..=3 {
                save.PLATES[[K, save.I]] = (save.J + K);
            }

            save.I += m3__;
        }
    }

    save.NV = 30;
    save.NP = 10;

    //
    // Set up the voxel-plate mapping.
    //
    save.VOXNPT = 1;

    save.VOXPTR[1] = 1;

    save.VOXPLT[1] = 10;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 10;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.VOXPLT[(1 + save.I)] = save.I;
            save.I += m3__;
        }
    }

    save.VOXNPL = 11;

    save.TO = (save.VOXNPL + 1);

    //
    // Set the coarse grid pointers.
    //
    spicelib::FILLI(-1, save.NCVOX, save.CGRPTR.as_slice_mut());

    save.CGRPTR[1] = 1;

    //
    // Create vertices, plates, and spatial index entries for
    // the other voxels.
    //
    save.NVOX = save.VGREXT[1];

    for VI in 2..=save.NVOX {
        //
        // Create vertices and plates for the current voxel.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = 10;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.J = (3 * ((save.NP + save.I) - 1));

                save.XOFF = (((VI - 1) as f64) * save.VOXSIZ);
                save.ZOFF = (((VI - 1) as f64) * ((save.VOXSIZ - 1 as f64) / save.NVOX as f64));

                for K in 1..=3 {
                    save.VERTS[[1, (save.J + K)]] = (save.VERTS[[1, 1]] + save.XOFF);
                    save.VERTS[[2, (save.J + K)]] = save.VERTS[[2, (K + (3 * (save.I - 1)))]];
                    save.VERTS[[3, (save.J + K)]] = (save.VERTS[[3, K]] + save.ZOFF);
                }

                for K in 1..=3 {
                    save.PLATES[[K, (save.NP + save.I)]] = (save.J + K);
                }

                save.I += m3__;
            }
        }

        //
        // Set the coarse grid pointer for the current coarse voxel.
        //
        save.CGRPTR[VI] = VI;

        //
        // Set the voxel pointer for the current voxel.
        //
        save.VOXPTR[VI] = save.TO;

        //
        // Fill in the plate list for the current voxel.
        //
        save.VOXPLT[save.TO] = 20;

        //
        // Add the new plates to the plate list.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = 10;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.VOXPLT[(save.TO + save.I)] = (save.NP + save.I);
                save.I += m3__;
            }
        }

        //
        // Add the plates from the preceding voxel to the plate list.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = 10;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.VOXPLT[((save.TO + 10) + save.I)] = ((save.NP - 10) + save.I);
                save.I += m3__;
            }
        }

        save.NP = (save.NP + 10);
        save.NV = (save.NV + 30);

        save.VOXNPL = (save.VOXNPL + 21);
        save.TO = (save.VOXNPL + 1);
    }

    save.VOXNPT = save.NVOX;

    //
    // We will not use the vertex-plate list.
    //

    //
    // Compute vertex extents.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.VTXBDS[[1, save.I]] = spicelib::DPMAX();
            save.VTXBDS[[2, save.I]] = spicelib::DPMIN();
            save.I += m3__;
        }
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NV;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = 3;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.VTXBDS[[1, save.J]] = intrinsics::DMIN1(&[
                        save.VTXBDS[[1, save.J]],
                        save.VERTS[[save.J, save.I]],
                    ]);
                    save.VTXBDS[[2, save.J]] = intrinsics::DMAX1(&[
                        save.VTXBDS[[2, save.J]],
                        save.VERTS[[save.J, save.I]],
                    ]);

                    save.J += m3__;
                }
            }

            save.I += m3__;
        }
    }

    //
    // Pack the integer component of the spatial index array.
    //
    spicelib::MOVEI(save.VGREXT.as_slice(), 3, save.SPAIXI.subarray_mut(SIVGRX));

    save.SPAIXI[SICGSC] = save.CGRSCL;
    save.SPAIXI[SIVXNP] = save.VOXNPT;
    save.SPAIXI[SIVXNL] = save.VOXNPL;
    save.SPAIXI[SIVTNL] = 0;

    spicelib::MOVEI(
        save.CGRPTR.as_slice(),
        MAXCGR,
        save.SPAIXI.subarray_mut(SICGRD),
    );

    save.I = (SICGRD + MAXCGR);

    spicelib::MOVEI(
        save.VOXPTR.as_slice(),
        save.VOXNPT,
        save.SPAIXI.subarray_mut(save.I),
    );

    save.I = (save.I + save.VOXNPT);

    spicelib::MOVEI(
        save.VOXPLT.as_slice(),
        save.VOXNPL,
        save.SPAIXI.subarray_mut(save.I),
    );

    //
    // Pack the d.p. component of the spatial index array.
    //
    spicelib::MOVED(save.VTXBDS.as_slice(), 6, save.SPAIXD.subarray_mut(SIVTBD));
    spicelib::VEQU(save.ORIGIN.as_slice(), save.SPAIXD.subarray_mut(SIVXOR));

    save.SPAIXD[SIVXSZ] = save.VOXSIZ;

    //

    //
    // Set up ID and coverage parameters for the segment. These
    // are required in order to create a DSK segment but have no
    // other use for the tests performed here.
    //
    save.SURFCE = 599;
    save.CENTER = 5;
    save.DCLASS = 2;
    fstr::assign(&mut save.FRAME, b"IAU_JUPITER");
    save.CORSYS = RECSYS;
    save.MNCOR1 = 0.0;
    save.MXCOR1 = ((save.VGREXT[1] as f64) * save.VOXSIZ);
    save.MNCOR2 = 0.0;
    save.MXCOR2 = ((save.VGREXT[2] as f64) * save.VOXSIZ);
    save.MNCOR3 = 0.0;
    save.MXCOR3 = ((save.VGREXT[3] as f64) * save.VOXSIZ);

    save.FIRST = -1.0;
    save.LAST = 1.0;

    //
    // Write the segment.
    //
    spicelib::DSKW02(
        save.HANDLE,
        save.CENTER,
        save.SURFCE,
        save.DCLASS,
        &save.FRAME,
        save.CORSYS,
        save.CORPAR.as_slice(),
        save.MNCOR1,
        save.MXCOR1,
        save.MNCOR2,
        save.MXCOR2,
        save.MNCOR3,
        save.MXCOR3,
        save.FIRST,
        save.LAST,
        save.NV,
        save.VERTS.as_slice(),
        save.NP,
        save.PLATES.as_slice(),
        save.SPAIXD.as_slice(),
        save.SPAIXI.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DSKCLS(save.HANDLE, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Find ray-plate intercepts in the second segment.", ctx)?;

    spicelib::DASOPR(&save.DSK, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DLABFS(
        save.HANDLE,
        save.DLADSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::MOVEI(save.DLADSC.as_slice(), DLADSZ, save.PRVDSC.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DLAFNS(
        save.HANDLE,
        save.PRVDSC.as_slice(),
        save.DLADSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    for VI in 1..=save.NVOX {
        {
            let m1__: i32 = 1;
            let m2__: i32 = 10;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                fstr::assign(&mut save.TITLE, b"Intercept on plate # of voxel #.");
                spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);
                spicelib::REPMI(&save.TITLE.to_vec(), b"#", VI, &mut save.TITLE, ctx);

                //
                // --- Case: ------------------------------------------------------
                //
                testutil::TCASE(&save.TITLE, ctx)?;

                save.VERTEX[1] = -1.0;
                save.VERTEX[2] = ((save.I as f64) - 0.5);
                save.VERTEX[3] = (0.95 + (((VI - 1) as f64) * 0.09));

                save.RAYDIR[1] = 1.0;
                save.RAYDIR[2] = 0.0;
                save.RAYDIR[2] = 0.0;

                spicelib::DSKX02(
                    save.HANDLE,
                    save.DLADSC.as_slice(),
                    save.VERTEX.as_slice(),
                    save.RAYDIR.as_slice(),
                    &mut save.PLID,
                    save.XPT.as_slice_mut(),
                    &mut save.FOUND,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                if !*OK {
                    ctx.stop()?;
                }

                testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

                if save.FOUND {
                    save.XPLID = (((VI - 1) * 10) + save.I);

                    testutil::CHCKSI(b"PLID", save.PLID, b"=", save.XPLID, 0, OK, ctx)?;

                    spicelib::VEQU(save.VERTEX.as_slice(), save.XXPT.as_slice_mut());
                    save.XXPT[1] = (save.VERTS[[1, 1]] + (((VI - 1) as f64) * save.VOXSIZ));

                    testutil::CHCKAD(
                        b"XPT",
                        save.XPT.as_slice(),
                        b"~~/",
                        save.XXPT.as_slice(),
                        3,
                        VTIGHT,
                        OK,
                        ctx,
                    )?;
                }

                save.I += m3__;
            }
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // This case tests the fix for the bug in which an out-of-segment
    // intercept can be returned, even though the returned plate ID is
    // for a valid, in-segment solution. The original bug fix was
    // written in April 2017. The bug itself was introduced that same
    // month, slightly before the N0066 delivery.
    //
    testutil::TCASE(b"Test out-of-segment intercept bug fix", ctx)?;

    //
    // We'll create a DSK that contains six segments. For each
    // coordinate system, there will be two segments, each containing
    // two plates.
    //
    // For each coordinate system, in one of the segments associated
    // with that coordinate system, the first plate will lie within the
    // segment bounds, and the second plate will be outside the segment
    // bounds. In the second segment, the order of the plates will be
    // reversed. These variations are needed to ensure that at least one
    // segment will satisfy the conditions required to exhibit the bug,
    // regardless of the order in which the DSK type 2 writer adds data
    // to the segment.

    // The plate set used for this test is contrived and completely
    // unrealistic, but it suffices to demonstrate the error in the
    // previous version of DSKX02 and the correction in the N0067
    // version.
    //
    fstr::assign(&mut save.DSK1, b"test_dskx02_fix.bds");

    if spicelib::EXISTS(&save.DSK1, ctx)? {
        spicelib::DELFIL(&save.DSK1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Open the DSK for writing.
    //
    spicelib::DSKOPN(&save.DSK1, &save.DSK1, 0, &mut save.HAN1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create two plates. The plates are parallel to the Y-Z plane.
    // The plate centers are on the positive X axis. Vertices of the
    // first plate are
    //
    //     ( 10, -1, -1 )
    //     ( 10,  1, -1 )
    //     ( 10,  0,  1 )
    //
    // Vertices of the second plate are
    //
    //     ( 20, -1, -1 )
    //     ( 20,  1, -1 )
    //     ( 20,  0,  1 )
    //
    save.VERTS[[1, 1]] = 10.0;
    save.VERTS[[2, 1]] = -1.0;
    save.VERTS[[3, 1]] = -1.0;

    save.VERTS[[1, 2]] = 10.0;
    save.VERTS[[2, 2]] = 2.0;
    save.VERTS[[3, 2]] = -1.0;

    save.VERTS[[1, 3]] = 10.0;
    save.VERTS[[2, 3]] = 0.0;
    save.VERTS[[3, 3]] = 1.0;

    save.PLATES[[1, 1]] = 1;
    save.PLATES[[2, 1]] = 2;
    save.PLATES[[3, 1]] = 3;

    save.VERTS[[1, 4]] = 20.0;
    save.VERTS[[2, 4]] = -1.0;
    save.VERTS[[3, 4]] = -1.0;

    save.VERTS[[1, 5]] = 20.0;
    save.VERTS[[2, 5]] = 2.0;
    save.VERTS[[3, 5]] = -1.0;

    save.VERTS[[1, 6]] = 20.0;
    save.VERTS[[2, 6]] = 0.0;
    save.VERTS[[3, 6]] = 1.0;

    save.PLATES[[1, 2]] = 4;
    save.PLATES[[2, 2]] = 5;
    save.PLATES[[3, 2]] = 6;

    save.NV = 6;
    save.NP = 2;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NSYS;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // For each coordinate system, create segment bounds that contain
            // one segment and exclude the other.
            //
            if (save.SYSTMS[save.I] == LATSYS) {
                //
                // Latitudinal bounds are for longitude, latitude, and radius,
                // in that order.
                //
                save.MNCOR1 = -spicelib::PI(ctx);
                save.MXCOR1 = spicelib::PI(ctx);
                save.MNCOR2 = -(spicelib::PI(ctx) / 2 as f64);
                save.MXCOR2 = (spicelib::PI(ctx) / 2 as f64);
                save.MNCOR3 = 0.0;
                save.MXCOR3 = 15.0;
                //
                // There are no coordinate system parameters.
                //
                spicelib::CLEARD(NSYPAR, save.CORPAR.as_slice_mut());
                testutil::CHCKXC(false, b" ", OK, ctx)?;
            } else if (save.SYSTMS[save.I] == RECSYS) {
                save.MNCOR1 = 0.0;
                save.MXCOR1 = 15.0;
                save.MNCOR2 = -2.0;
                save.MXCOR2 = 2.0;
                save.MNCOR3 = -2.0;
                save.MXCOR3 = 2.0;
                //
                // There are no coordinate system parameters.
                //
                spicelib::CLEARD(NSYPAR, save.CORPAR.as_slice_mut());
                testutil::CHCKXC(false, b" ", OK, ctx)?;
            } else if (save.SYSTMS[save.I] == PDTSYS) {
                //
                // The coordinate system parameters for planetodetic
                // coordinates are RE and F, in that order. Choose values
                // that correspond to an equatorial radius of 10 and a polar
                // radius of 8.
                //
                save.CORPAR[1] = 10.0;
                save.CORPAR[2] = 0.2;

                spicelib::CLEARD((NSYPAR - 2), save.CORPAR.subarray_mut(3));
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Planetodetic bounds are for longitude, latitude, and
                // altitude, in that order.
                //
                save.MNCOR1 = -spicelib::PI(ctx);
                save.MXCOR1 = spicelib::PI(ctx);
                save.MNCOR2 = -(spicelib::PI(ctx) / 2 as f64);
                save.MXCOR2 = (spicelib::PI(ctx) / 2 as f64);
                //
                // The altitude range is from -3 to 5.
                //
                save.MNCOR3 = -3.0;
                save.MXCOR3 = 5.0;
            } else {
                spicelib::SETMSG(b"Unexpected coordinate system code: #.", ctx);
                spicelib::ERRINT(b"#", save.SYSTMS[save.I], ctx);
                spicelib::SIGERR(b"SPICE(TESTBUG)", ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
            }

            {
                let m1__: i32 = 1;
                let m2__: i32 = 2;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    if (save.J == 2) {
                        //
                        // Swap the plate order.
                        //
                        for K in 1..=3 {
                            spicelib::SWAPI_ARRAY(
                                save.PLATES.subscript([K, 1]),
                                save.PLATES.subscript([K, 2]),
                                save.PLATES.as_slice_mut(),
                            );
                        }
                    }

                    //
                    // Set the voxel scales. We need very large voxels, since the
                    // case of interest is one in which both plates are in the
                    // same voxel, even though one plate is inside the segment
                    // bounds and the other is not.
                    //
                    save.FINSCL = 100.0;
                    save.CORSCL = 1;

                    //
                    // Create spatial index for the segment. Choose generous
                    // values for the data structure sizes. We're not calculating
                    // the exact required sizes.
                    //
                    save.VOXPSZ = MXNVOX;
                    save.VOXLSZ = MXNVOX;

                    spicelib::DSKMI2(
                        save.NV,
                        save.VERTS.as_slice(),
                        save.NP,
                        save.PLATES.as_slice(),
                        save.FINSCL,
                        save.CORSCL,
                        WORKSZ,
                        save.VOXPSZ,
                        save.VOXLSZ,
                        true,
                        MAXIXI,
                        save.WORK.as_slice_mut(),
                        save.SPAIXD.as_slice_mut(),
                        save.SPAIXI.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Set up ID parameters for the segment. These are required in
                    // order to create a DSK segment but have no other use for the
                    // tests performed here.
                    //
                    save.SURFCE = 499;
                    save.CENTER = 4;
                    save.DCLASS = 2;
                    fstr::assign(&mut save.FRAME, b"IAU_MARS");
                    save.CORSYS = save.SYSTMS[save.I];
                    //
                    // Write the segment.
                    //
                    spicelib::DSKW02(
                        save.HAN1,
                        save.CENTER,
                        save.SURFCE,
                        save.DCLASS,
                        &save.FRAME,
                        save.CORSYS,
                        save.CORPAR.as_slice(),
                        save.MNCOR1,
                        save.MXCOR1,
                        save.MNCOR2,
                        save.MXCOR2,
                        save.MNCOR3,
                        save.MXCOR3,
                        save.FIRST,
                        save.LAST,
                        save.NV,
                        save.VERTS.as_slice(),
                        save.NP,
                        save.PLATES.as_slice(),
                        save.SPAIXD.as_slice(),
                        save.SPAIXI.as_slice(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    if (save.J == 2) {
                        //
                        // Undo the plate order swap.
                        //
                        for K in 1..=3 {
                            spicelib::SWAPI_ARRAY(
                                save.PLATES.subscript([K, 1]),
                                save.PLATES.subscript([K, 2]),
                                save.PLATES.as_slice_mut(),
                            );
                        }
                    }

                    save.J += m3__;
                }
            }

            save.I += m3__;
        }
    }
    //
    // Close the DSK.
    //
    spicelib::DSKCLS(save.HAN1, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now open the file for read access and traverse its segments.
    // For each segment, try a ray-surface intercept test.
    //
    spicelib::DASOPR(&save.DSK1, &mut save.HAN1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DLABFS(save.HAN1, save.DLADSC.as_slice_mut(), &mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SEGNO = 0;

    while save.FOUND {
        save.SEGNO = (save.SEGNO + 1);
        //
        // Create the vertex and direction vector of a ray to be used
        // in a call to DSKX02.
        //
        spicelib::VPACK(100.0, 0.0, 0.0, save.VERTEX.as_slice_mut());
        spicelib::VPACK(-1.0, 0.0, 0.0, save.RAYDIR.as_slice_mut());

        //
        // Compute the ray-surface intercept. We expect the ray to hit
        // the plate in the X = 10 plane.
        //
        spicelib::DSKX02(
            save.HAN1,
            save.DLADSC.as_slice(),
            save.VERTEX.as_slice(),
            save.RAYDIR.as_slice(),
            &mut save.PLID,
            save.XPT.as_slice_mut(),
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Make sure the intercept was found.
        //
        fstr::assign(&mut save.LABEL, b"FOUND #");
        spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.SEGNO, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(&save.LABEL, save.FOUND, true, OK, ctx)?;
        spicelib::VPACK(10.0, 0.0, 0.0, save.XXPT.as_slice_mut());

        //
        // Check the intercept.
        //
        fstr::assign(&mut save.LABEL, b"XPT #");
        spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.SEGNO, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            &save.LABEL,
            save.XPT.as_slice(),
            b"~~/",
            save.XXPT.as_slice(),
            3,
            VTIGHT,
            OK,
            ctx,
        )?;

        //
        // To check the plate ID, we must see whether that ID belongs
        // to the plate lying in the X = 10 plane. Look up the plate.
        //
        spicelib::DSKP02(
            save.HAN1,
            save.DLADSC.as_slice(),
            save.PLID,
            1,
            &mut save.N,
            save.PLATE.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // The correct plate always has vertices 1, 2, 3.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = 3;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.XPLATE[save.I] = save.I;
                save.I += m3__;
            }
        }

        fstr::assign(&mut save.LABEL, b"PLATE #");
        spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.SEGNO, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            &save.LABEL,
            save.PLATE.as_slice(),
            b"=",
            save.XPLATE.as_slice(),
            3,
            OK,
            ctx,
        )?;

        //
        // Look up the next DLA segment.
        //
        spicelib::MOVEI(save.DLADSC.as_slice(), DLADSZ, save.PRVDSC.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DLAFNS(
            save.HAN1,
            save.PRVDSC.as_slice(),
            save.DLADSC.as_slice_mut(),
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::DASCLS(save.HAN1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up kernels.", ctx)?;

    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(&save.DSK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASCLS(save.HAN1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(&save.DSK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
