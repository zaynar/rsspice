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
const MAXV: i32 = 10000;
const MAXP: i32 = (2 * MAXV);
const DEFPSZ: i32 = (3 * MAXP);
const DEFLSZ: i32 = (5 * MAXP);
const DEFWSZ: i32 = (5 * MAXP);
const DEFMSZ: i32 = (MAXV + (3 * MAXP));
const VTXLSZ: i32 = ((3 * MAXP) + MAXV);
const DEFISZ: i32 = (((MAXCGR + DEFPSZ) + DEFLSZ) + DEFMSZ);

struct SaveVars {
    A: f64,
    B: f64,
    C: f64,
    EXTENT: StackArray2D<f64, 6>,
    FINSCL: f64,
    SPAIXD: StackArray<f64, 10>,
    VERTS: ActualArray2D<f64>,
    VOXORI: StackArray<f64, 3>,
    VOXSIZ: f64,
    CELLS: ActualArray2D<i32>,
    CGRPTR: ActualArray<i32>,
    CORSCL: i32,
    I: i32,
    PLATES: ActualArray2D<i32>,
    NLAT: i32,
    NLON: i32,
    NP: i32,
    NV: i32,
    NVLIST: i32,
    NVOX: StackArray<i32, 3>,
    NVXLST: i32,
    NVXPTR: i32,
    NVXTOT: i32,
    REQSIZ: i32,
    SIZE: i32,
    SPAIXI: ActualArray<i32>,
    SPXISZ: i32,
    VOXLSZ: i32,
    VOXNPL: i32,
    VOXNPT: i32,
    VOXPSZ: i32,
    VPLIST: ActualArray<i32>,
    VRTPTR: ActualArray<i32>,
    VXLIST: ActualArray<i32>,
    VOXPTR: ActualArray<i32>,
    WORKSZ: i32,
    WORK: ActualArray2D<i32>,
    MAKVTL: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut A: f64 = 0.0;
        let mut B: f64 = 0.0;
        let mut C: f64 = 0.0;
        let mut EXTENT = StackArray2D::<f64, 6>::new(1..=2, 1..=3);
        let mut FINSCL: f64 = 0.0;
        let mut SPAIXD = StackArray::<f64, 10>::new(1..=IXDFIX);
        let mut VERTS = ActualArray2D::<f64>::new(1..=3, 1..=MAXV);
        let mut VOXORI = StackArray::<f64, 3>::new(1..=3);
        let mut VOXSIZ: f64 = 0.0;
        let mut CELLS = ActualArray2D::<i32>::new(1..=2, 1..=DEFLSZ);
        let mut CGRPTR = ActualArray::<i32>::new(1..=MAXCGR);
        let mut CORSCL: i32 = 0;
        let mut I: i32 = 0;
        let mut PLATES = ActualArray2D::<i32>::new(1..=3, 1..=MAXP);
        let mut NLAT: i32 = 0;
        let mut NLON: i32 = 0;
        let mut NP: i32 = 0;
        let mut NV: i32 = 0;
        let mut NVLIST: i32 = 0;
        let mut NVOX = StackArray::<i32, 3>::new(1..=3);
        let mut NVXLST: i32 = 0;
        let mut NVXPTR: i32 = 0;
        let mut NVXTOT: i32 = 0;
        let mut REQSIZ: i32 = 0;
        let mut SIZE: i32 = 0;
        let mut SPAIXI = ActualArray::<i32>::new(1..=DEFISZ);
        let mut SPXISZ: i32 = 0;
        let mut VOXLSZ: i32 = 0;
        let mut VOXNPL: i32 = 0;
        let mut VOXNPT: i32 = 0;
        let mut VOXPSZ: i32 = 0;
        let mut VPLIST = ActualArray::<i32>::new(1..=DEFPSZ);
        let mut VRTPTR = ActualArray::<i32>::new(1..=MAXV);
        let mut VXLIST = ActualArray::<i32>::new(1..=DEFLSZ);
        let mut VOXPTR = ActualArray::<i32>::new(1..=DEFPSZ);
        let mut WORKSZ: i32 = 0;
        let mut WORK = ActualArray2D::<i32>::new(1..=2, 1..=DEFWSZ);
        let mut MAKVTL: bool = false;

        Self {
            A,
            B,
            C,
            EXTENT,
            FINSCL,
            SPAIXD,
            VERTS,
            VOXORI,
            VOXSIZ,
            CELLS,
            CGRPTR,
            CORSCL,
            I,
            PLATES,
            NLAT,
            NLON,
            NP,
            NV,
            NVLIST,
            NVOX,
            NVXLST,
            NVXPTR,
            NVXTOT,
            REQSIZ,
            SIZE,
            SPAIXI,
            SPXISZ,
            VOXLSZ,
            VOXNPL,
            VOXNPT,
            VOXPSZ,
            VPLIST,
            VRTPTR,
            VXLIST,
            VOXPTR,
            WORKSZ,
            WORK,
            MAKVTL,
        }
    }
}

//$Procedure      F_DSKMI2 ( Test DSKMI2 )
pub fn F_DSKMI2(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Test Utility Functions
    //

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local Variables
    //

    //
    // Saved variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_DSKMI2", ctx)?;

    //***********************************************************************
    //
    //     Normal cases
    //
    //***********************************************************************

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Set-up: create plates for a tessellated ellipsoid. NLON = 100; NLAT = 50; NV = 4902; NP = 9800.", ctx)?;

    save.A = 3000.0;
    save.B = 2000.0;
    save.C = 1000.0;

    save.NLON = 100;
    save.NLAT = 50;

    support::ZZELLPLT(
        save.A,
        save.B,
        save.C,
        save.NLON,
        save.NLAT,
        MAXV,
        MAXP,
        &mut save.NV,
        save.VERTS.as_slice_mut(),
        &mut save.NP,
        save.PLATES.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Setup: create spatial index using the plate set from the previous case. Call ZZMKSPIN directly.", ctx)?;

    //
    // Set voxel scales.
    //
    save.FINSCL = 2.0;
    save.CORSCL = 4;

    //
    // Set sizes of the workspace and output arrays.
    //
    spicelib::ZZMKSPIN(
        save.NP,
        save.PLATES.as_slice(),
        save.VERTS.as_slice(),
        save.FINSCL,
        save.CORSCL,
        DEFPSZ,
        DEFLSZ,
        DEFLSZ,
        save.CELLS.as_slice_mut(),
        save.NVOX.as_slice_mut(),
        &mut save.VOXSIZ,
        save.VOXORI.as_slice_mut(),
        &mut save.NVXTOT,
        &mut save.NVXPTR,
        save.VOXPTR.as_slice_mut(),
        &mut save.NVXLST,
        save.VXLIST.as_slice_mut(),
        save.EXTENT.as_slice_mut(),
        save.CGRPTR.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Setup: vertex-plate map using the plate set from the previous case. Call ZZVRTPLT directly.", ctx)?;

    spicelib::ZZVRTPLT(
        save.NV,
        save.NP,
        save.PLATES.as_slice(),
        DEFLSZ,
        DEFLSZ,
        save.CELLS.as_slice_mut(),
        save.VRTPTR.as_slice_mut(),
        &mut save.NVLIST,
        save.VPLIST.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Create spatial index using the plate set from the previous case. Include a vertex-plate map.", ctx)?;

    //
    // Set voxel scales.
    //
    save.FINSCL = 2.0;
    save.CORSCL = 4;

    //
    // Make a plate-vertex map.
    //
    save.MAKVTL = true;

    //
    // Set sizes of the workspace and output arrays.
    //
    save.WORKSZ = DEFWSZ;
    save.VOXPSZ = DEFPSZ;
    save.VOXLSZ = DEFLSZ;
    save.SPXISZ = DEFISZ;

    spicelib::DSKMI2(
        save.NV,
        save.VERTS.as_slice(),
        save.NP,
        save.PLATES.as_slice(),
        save.FINSCL,
        save.CORSCL,
        save.WORKSZ,
        save.VOXPSZ,
        save.VOXLSZ,
        save.MAKVTL,
        save.SPXISZ,
        save.WORK.as_slice_mut(),
        save.SPAIXD.as_slice_mut(),
        save.SPAIXI.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The following cases test the integer component of
    // the spatial index. This component is nothing but
    // a concatenation of the integer outputs of ZZMKSPIN.
    //

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check voxel grid extents from DSKMI2.", ctx)?;

    testutil::CHCKAI(
        b"Voxel grid extents",
        save.SPAIXI.subarray(SIVGRX),
        b"=",
        save.NVOX.as_slice(),
        3,
        OK,
        ctx,
    )?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check coarse voxel scale from DSKMI2.", ctx)?;

    testutil::CHCKSI(
        b"Coarse voxel scale",
        save.SPAIXI[SICGSC],
        b"=",
        save.CORSCL,
        0,
        OK,
        ctx,
    )?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check coarse voxel pointers from DSKMI2.", ctx)?;

    testutil::CHCKAI(
        b"Coarse voxel pointers",
        save.SPAIXI.subarray(SICGRD),
        b"=",
        save.CGRPTR.as_slice(),
        MAXCGR,
        OK,
        ctx,
    )?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check vertex-plate map from DSKMI2.", ctx)?;

    //
    // DSKMI2 should have simply packaged the map created by
    // ZZVRTPLT.
    //
    // Check the map size first.
    //
    save.SIZE = save.SPAIXI[SIVTNL];

    testutil::CHCKAI(
        b"vertex-plate map size",
        &[save.SIZE],
        b"=",
        &[save.NVLIST],
        0,
        OK,
        ctx,
    )?;

    //
    // Check the vertex-plate pointers. Compute the start index of the
    // pointer array.
    //
    save.VOXNPT = save.SPAIXI[SIVXNP];
    save.VOXNPL = save.SPAIXI[SIVXNL];

    save.I = (((SICGRD + MAXCGR) + save.VOXNPT) + save.VOXNPL);

    testutil::CHCKAI(
        b"vertex-plate pointers",
        save.SPAIXI.subarray(save.I),
        b"=",
        save.VRTPTR.as_slice(),
        save.NV,
        OK,
        ctx,
    )?;

    //
    // Compute the start index of the map itself.
    //

    save.I = (save.I + save.NV);

    testutil::CHCKAI(
        b"vertex-plate map",
        save.SPAIXI.subarray(save.I),
        b"=",
        save.VPLIST.as_slice(),
        save.NVLIST,
        OK,
        ctx,
    )?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check voxel-plate map from DSKMI2.", ctx)?;

    //
    // Check the voxel pointer count.
    //
    testutil::CHCKSI(
        b"voxel pointer count",
        save.VOXNPT,
        b"=",
        save.NVXPTR,
        0,
        OK,
        ctx,
    )?;
    //
    // Check the voxel-plate pointers. Compute the start index of the
    // pointer array.
    //
    save.I = (SICGRD + MAXCGR);

    testutil::CHCKAI(
        b"voxel-plate pointers",
        save.SPAIXI.subarray(save.I),
        b"=",
        save.VOXPTR.as_slice(),
        save.NVXPTR,
        OK,
        ctx,
    )?;

    //
    // Check the voxel-plate list size.
    //
    testutil::CHCKSI(
        b"voxel-plate list size",
        save.VOXNPL,
        b"=",
        save.NVXLST,
        0,
        OK,
        ctx,
    )?;

    //
    // Check the voxel-plate list. Compute the start index of the
    // list.
    //
    save.I = (save.I + save.VOXNPT);

    testutil::CHCKAI(
        b"voxel-plate list",
        save.SPAIXI.subarray(save.I),
        b"=",
        save.VXLIST.as_slice(),
        save.VOXNPL,
        OK,
        ctx,
    )?;

    //
    // The following cases test the double precision component of
    // the spatial index.
    //

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check vertex bounds from DSKMI2.", ctx)?;

    testutil::CHCKAD(
        b"vertex bounds",
        save.SPAIXD.subarray(SIVTBD),
        b"=",
        save.EXTENT.as_slice(),
        6,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check voxel grid origin from DSKMI2.", ctx)?;

    testutil::CHCKAD(
        b"voxel grid origin",
        save.SPAIXD.subarray(SIVXOR),
        b"=",
        save.VOXORI.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check fine voxel size from DSKMI2.", ctx)?;

    testutil::CHCKSD(
        b"fine voxel size",
        save.SPAIXD[SIVXSZ],
        b"=",
        save.VOXSIZ,
        0.0,
        OK,
        ctx,
    )?;

    //***********************************************************************
    //
    //     Error cases
    //
    //***********************************************************************

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Bad fine voxel scale.", ctx)?;

    //
    // Set voxel scales.
    //
    save.FINSCL = -2.0;
    save.CORSCL = 4;

    spicelib::DSKMI2(
        save.NV,
        save.VERTS.as_slice(),
        save.NP,
        save.PLATES.as_slice(),
        save.FINSCL,
        save.CORSCL,
        save.WORKSZ,
        save.VOXPSZ,
        save.VOXLSZ,
        save.MAKVTL,
        save.SPXISZ,
        save.WORK.as_slice_mut(),
        save.SPAIXD.as_slice_mut(),
        save.SPAIXI.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADFINEVOXELSCALE)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Bad coarse voxel scale.", ctx)?;

    //
    // Set voxel scales.
    //
    save.FINSCL = 2.0;
    save.CORSCL = 0;

    spicelib::DSKMI2(
        save.NV,
        save.VERTS.as_slice(),
        save.NP,
        save.PLATES.as_slice(),
        save.FINSCL,
        save.CORSCL,
        save.WORKSZ,
        save.VOXPSZ,
        save.VOXLSZ,
        save.MAKVTL,
        save.SPXISZ,
        save.WORK.as_slice_mut(),
        save.SPAIXD.as_slice_mut(),
        save.SPAIXI.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADCOARSEVOXSCALE)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Bad vertex count.", ctx)?;

    save.FINSCL = 2.0;
    save.CORSCL = 4;

    save.I = 2;

    spicelib::DSKMI2(
        save.I,
        save.VERTS.as_slice(),
        save.NP,
        save.PLATES.as_slice(),
        save.FINSCL,
        save.CORSCL,
        save.WORKSZ,
        save.VOXPSZ,
        save.VOXLSZ,
        save.MAKVTL,
        save.SPXISZ,
        save.WORK.as_slice_mut(),
        save.SPAIXD.as_slice_mut(),
        save.SPAIXI.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADVERTEXCOUNT)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Bad plate count.", ctx)?;

    save.I = 0;

    spicelib::DSKMI2(
        save.NV,
        save.VERTS.as_slice(),
        save.I,
        save.PLATES.as_slice(),
        save.FINSCL,
        save.CORSCL,
        save.WORKSZ,
        save.VOXPSZ,
        save.VOXLSZ,
        save.MAKVTL,
        save.SPXISZ,
        save.WORK.as_slice_mut(),
        save.SPAIXD.as_slice_mut(),
        save.SPAIXI.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADPLATECOUNT)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Workspace too small.", ctx)?;

    save.I = save.NP;

    spicelib::DSKMI2(
        save.NV,
        save.VERTS.as_slice(),
        save.NP,
        save.PLATES.as_slice(),
        save.FINSCL,
        save.CORSCL,
        save.I,
        save.VOXPSZ,
        save.VOXLSZ,
        save.MAKVTL,
        save.SPXISZ,
        save.WORK.as_slice_mut(),
        save.SPAIXD.as_slice_mut(),
        save.SPAIXI.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(WORKSPACETOOSMALL)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Voxel-plate pointer array too small.", ctx)?;

    save.I = 0;

    spicelib::DSKMI2(
        save.NV,
        save.VERTS.as_slice(),
        save.NP,
        save.PLATES.as_slice(),
        save.FINSCL,
        save.CORSCL,
        save.WORKSZ,
        save.I,
        save.VOXLSZ,
        save.MAKVTL,
        save.SPXISZ,
        save.WORK.as_slice_mut(),
        save.SPAIXD.as_slice_mut(),
        save.SPAIXI.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(PTRARRAYTOOSMALL)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Voxel-plate list too small.", ctx)?;

    save.I = save.NP;

    spicelib::DSKMI2(
        save.NV,
        save.VERTS.as_slice(),
        save.NP,
        save.PLATES.as_slice(),
        save.FINSCL,
        save.CORSCL,
        save.WORKSZ,
        save.VOXPSZ,
        save.I,
        save.MAKVTL,
        save.SPXISZ,
        save.WORK.as_slice_mut(),
        save.SPAIXD.as_slice_mut(),
        save.SPAIXI.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(PLATELISTTOOSMALL)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Integer spatial index component too small.", ctx)?;

    save.REQSIZ = ((((IXIFIX + save.VOXPSZ) + save.VOXLSZ) + (2 * save.NV)) + (3 * save.NP));

    save.I = (save.REQSIZ - 1);

    spicelib::DSKMI2(
        save.NV,
        save.VERTS.as_slice(),
        save.NP,
        save.PLATES.as_slice(),
        save.FINSCL,
        save.CORSCL,
        save.WORKSZ,
        save.VOXPSZ,
        save.I,
        save.MAKVTL,
        save.SPXISZ,
        save.WORK.as_slice_mut(),
        save.SPAIXD.as_slice_mut(),
        save.SPAIXI.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INTINDEXTOOSMALL)", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
