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
const TIGHT: f64 = 0.000000000001;
const LBLLEN: i32 = 40;
const MAXV: i32 = 10000;
const MAXP: i32 = (2 * MAXV);
const DEFPSZ: i32 = (3 * MAXP);
const DEFLSZ: i32 = (5 * MAXP);

struct SaveVars {
    LABEL: Vec<u8>,
    A: f64,
    B: f64,
    C: f64,
    EXTENT: StackArray2D<f64, 6>,
    FINSCL: f64,
    HIGHPT: StackArray<f64, 3>,
    LOWPT: StackArray<f64, 3>,
    MDLTOL: f64,
    PLTEXT: StackArray2D<f64, 6>,
    TOL: f64,
    UB: f64,
    VERTS: ActualArray2D<f64>,
    VOXORI: StackArray<f64, 3>,
    VOXSIZ: f64,
    VTXBDS: StackArray2D<f64, 6>,
    CELLS: ActualArray2D<i32>,
    CG3: i32,
    CGOF1D: i32,
    CGOFF: StackArray<i32, 3>,
    CGP: i32,
    CGRPTR: ActualArray<i32>,
    CGXYZ: StackArray<i32, 3>,
    CORSCL: i32,
    CVOXID: i32,
    HIVOX: StackArray<i32, 3>,
    J: i32,
    K: i32,
    LOC: i32,
    LOWVOX: StackArray<i32, 3>,
    NCGR: i32,
    NCGVOX: StackArray<i32, 3>,
    NLAT: i32,
    NLON: i32,
    NP: i32,
    NV: i32,
    NVOX: StackArray<i32, 3>,
    NVP: i32,
    NVXLST: i32,
    NVXPTR: i32,
    NVXTOT: i32,
    NX: i32,
    NXY: i32,
    PID: i32,
    PLATES: ActualArray2D<i32>,
    R: i32,
    VIX: i32,
    VOXCOR: StackArray<i32, 3>,
    VOXID: i32,
    VOXPTR: ActualArray<i32>,
    VPTR: i32,
    VXLIST: ActualArray<i32>,
    INBOX: bool,
    INCLUD: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut LABEL = vec![b' '; LBLLEN as usize];
        let mut A: f64 = 0.0;
        let mut B: f64 = 0.0;
        let mut C: f64 = 0.0;
        let mut EXTENT = StackArray2D::<f64, 6>::new(1..=2, 1..=3);
        let mut FINSCL: f64 = 0.0;
        let mut HIGHPT = StackArray::<f64, 3>::new(1..=3);
        let mut LOWPT = StackArray::<f64, 3>::new(1..=3);
        let mut MDLTOL: f64 = 0.0;
        let mut PLTEXT = StackArray2D::<f64, 6>::new(1..=2, 1..=3);
        let mut TOL: f64 = 0.0;
        let mut UB: f64 = 0.0;
        let mut VERTS = ActualArray2D::<f64>::new(1..=3, 1..=MAXV);
        let mut VOXORI = StackArray::<f64, 3>::new(1..=3);
        let mut VOXSIZ: f64 = 0.0;
        let mut VTXBDS = StackArray2D::<f64, 6>::new(1..=2, 1..=3);
        let mut CELLS = ActualArray2D::<i32>::new(1..=2, 1..=DEFLSZ);
        let mut CG3: i32 = 0;
        let mut CGOF1D: i32 = 0;
        let mut CGOFF = StackArray::<i32, 3>::new(1..=3);
        let mut CGP: i32 = 0;
        let mut CGRPTR = ActualArray::<i32>::new(1..=MAXCGR);
        let mut CGXYZ = StackArray::<i32, 3>::new(1..=3);
        let mut CORSCL: i32 = 0;
        let mut CVOXID: i32 = 0;
        let mut HIVOX = StackArray::<i32, 3>::new(1..=3);
        let mut J: i32 = 0;
        let mut K: i32 = 0;
        let mut LOC: i32 = 0;
        let mut LOWVOX = StackArray::<i32, 3>::new(1..=3);
        let mut NCGR: i32 = 0;
        let mut NCGVOX = StackArray::<i32, 3>::new(1..=3);
        let mut NLAT: i32 = 0;
        let mut NLON: i32 = 0;
        let mut NP: i32 = 0;
        let mut NV: i32 = 0;
        let mut NVOX = StackArray::<i32, 3>::new(1..=3);
        let mut NVP: i32 = 0;
        let mut NVXLST: i32 = 0;
        let mut NVXPTR: i32 = 0;
        let mut NVXTOT: i32 = 0;
        let mut NX: i32 = 0;
        let mut NXY: i32 = 0;
        let mut PID: i32 = 0;
        let mut PLATES = ActualArray2D::<i32>::new(1..=3, 1..=MAXP);
        let mut R: i32 = 0;
        let mut VIX: i32 = 0;
        let mut VOXCOR = StackArray::<i32, 3>::new(1..=3);
        let mut VOXID: i32 = 0;
        let mut VOXPTR = ActualArray::<i32>::new(1..=DEFPSZ);
        let mut VPTR: i32 = 0;
        let mut VXLIST = ActualArray::<i32>::new(1..=DEFLSZ);
        let mut INBOX: bool = false;
        let mut INCLUD: bool = false;

        Self {
            LABEL,
            A,
            B,
            C,
            EXTENT,
            FINSCL,
            HIGHPT,
            LOWPT,
            MDLTOL,
            PLTEXT,
            TOL,
            UB,
            VERTS,
            VOXORI,
            VOXSIZ,
            VTXBDS,
            CELLS,
            CG3,
            CGOF1D,
            CGOFF,
            CGP,
            CGRPTR,
            CGXYZ,
            CORSCL,
            CVOXID,
            HIVOX,
            J,
            K,
            LOC,
            LOWVOX,
            NCGR,
            NCGVOX,
            NLAT,
            NLON,
            NP,
            NV,
            NVOX,
            NVP,
            NVXLST,
            NVXPTR,
            NVXTOT,
            NX,
            NXY,
            PID,
            PLATES,
            R,
            VIX,
            VOXCOR,
            VOXID,
            VOXPTR,
            VPTR,
            VXLIST,
            INBOX,
            INCLUD,
        }
    }
}

//$Procedure      F_ZZMKSPIN ( Test ZZMKSPIN )
pub fn F_ZZMKSPIN(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    testutil::TOPEN(b"F_ZZMKSPIN", ctx)?;

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
    //
    // Check the outputs from ZZMKSPIN.
    //
    //

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check voxel grid dimensions.", ctx)?;

    for I in 1..=3 {
        fstr::assign(&mut save.LABEL, b"voxel grid dimension @");
        spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(&save.LABEL, save.NVOX[I], b">", 0, 0, OK, ctx)?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check total voxel count.", ctx)?;

    save.J = ((save.NVOX[1] * save.NVOX[2]) * save.NVOX[3]);

    testutil::CHCKSI(b"NVXTOT", save.NVXTOT, b"=", save.J, 0, OK, ctx)?;

    testutil::CHCKSI(b"NVXTOT", save.NVXTOT, b"<", (MAXVOX + 1), 0, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check voxel pointer array count.", ctx)?;

    testutil::CHCKSI(b"NVXPTR", save.NVXPTR, b">", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"NVXPTR", save.NVXPTR, b"<", (DEFPSZ + 1), 0, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check voxel list count.", ctx)?;

    testutil::CHCKSI(b"NVXLST", save.NVXLST, b">", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"NVXLST", save.NVXLST, b"<", (DEFLSZ + 1), 0, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check the model extents.", ctx)?;
    //
    // The extents are the minimum and maximum values of each vertex
    // coordinate, taken over all vertices. Compute the expected
    // extents.
    //
    for I in 1..=3 {
        save.VTXBDS[[1, I]] = spicelib::DPMAX();
        save.VTXBDS[[2, I]] = spicelib::DPMIN();
    }

    for I in 1..=save.NV {
        {
            let m1__: i32 = 1;
            let m2__: i32 = 3;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.VTXBDS[[1, save.J]] =
                    intrinsics::DMIN1(&[save.VTXBDS[[1, save.J]], save.VERTS[[save.J, I]]]);
                save.VTXBDS[[2, save.J]] =
                    intrinsics::DMAX1(&[save.VTXBDS[[2, save.J]], save.VERTS[[save.J, I]]]);

                save.J += m3__;
            }
        }
    }

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"EXTENT",
        save.EXTENT.as_slice(),
        b"~~/",
        save.VTXBDS.as_slice(),
        6,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"Check the voxel grid origin: compare to vertex extents.",
        ctx,
    )?;

    //
    // Each element of the origin should have a lower value than the
    // minimum vertex extent in the same coordinate.
    //
    for I in 1..=3 {
        fstr::assign(&mut save.LABEL, b"voxel origin @");
        spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.TOL = TIGHT;

        testutil::CHCKSD(
            &save.LABEL,
            save.VOXORI[I],
            b"<",
            save.VTXBDS[[1, I]],
            save.TOL,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"Check the voxel grid boundaries: compare to vertex extents.",
        ctx,
    )?;

    //
    // The voxel grid should enclose the smallest box that includes all
    // vertices. We need check only the upper bounds of the coordinates
    // of the grid's corners.
    //
    //
    for I in 1..=3 {
        fstr::assign(&mut save.LABEL, b"voxel grid upper bound @");
        spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.TOL = TIGHT;

        save.UB = (save.VOXORI[I] + (save.VOXSIZ * save.NVOX[I] as f64));

        testutil::CHCKSD(
            &save.LABEL,
            save.UB,
            b">",
            save.VTXBDS[[2, I]],
            save.TOL,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"Test coarse grid pointer array: make sure all pointers are in the range 0:NVXPTR.",
        ctx,
    )?;

    //
    // Compute the size of the coarse grid.
    //
    save.CG3 = intrinsics::pow(save.CORSCL, 3);
    save.NCGR = (save.NVXTOT / save.CG3);

    save.J = 0;

    for I in 1..=save.NCGR {
        fstr::assign(&mut save.LABEL, b"coarse grid pointer @");
        spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(&save.LABEL, save.CGRPTR[I], b">", -1, 0, OK, ctx)?;
        testutil::CHCKSI(
            &save.LABEL,
            save.CGRPTR[I],
            b"<",
            (save.NVXPTR + 1),
            0,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Test voxel-plate pointer array: make sure all pointers are either -1 or are in the range 1:NVXLST.", ctx)?;

    for I in 1..=save.NVXPTR {
        fstr::assign(&mut save.LABEL, b"voxel list pointer @");
        spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (save.VOXPTR[I] != -1) {
            testutil::CHCKSI(&save.LABEL, save.VOXPTR[I], b">", 0, 0, OK, ctx)?;
            testutil::CHCKSI(
                &save.LABEL,
                save.VOXPTR[I],
                b"<",
                (save.NVXLST + 1),
                0,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"Test plate counts in voxel-plate list: make sure all counts are in the range 1:NP.",
        ctx,
    )?;

    //
    // Access the plate counts using the voxel pointer array.
    //
    for I in 1..=save.NVXPTR {
        save.J = save.VOXPTR[I];

        if (save.J > 0) {
            save.K = save.VXLIST[save.J];

            fstr::assign(&mut save.LABEL, b"plate count for voxel pointer @");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSI(&save.LABEL, save.K, b">", 0, 0, OK, ctx)?;
            testutil::CHCKSI(&save.LABEL, save.K, b"<", (save.NP + 1), 0, OK, ctx)?;
        }
    }

    //
    // Below we test the indexing mechanism as a unit. We test the
    // voxel-plate associations defined by the coarse grid pointers,
    // the voxel pointer array, and the voxel-plate list.
    //

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Test the voxel-plate map: look up the plate list for each voxel. Check each plate to make sure its minimal bounding box intersects the voxel.", ctx)?;

    //
    // Define some variables needed for voxel ID-to-coordinate
    // mapping.
    //
    save.NX = save.NVOX[1];
    save.NXY = (save.NVOX[2] * save.NX);

    //
    // NCGVOX contains the coarse grid dimensions.
    //
    for I in 1..=3 {
        save.NCGVOX[I] = (save.NVOX[I] / save.CORSCL);
    }

    //
    // MDLTOL is a tolerance used to expand the extent of a plate
    // for the purpose of computing the set of voxels it intersects.
    //
    save.MDLTOL = (0.001 * save.VOXSIZ);

    for I in 1..=save.NVXTOT {
        //
        // Map the Ith fine voxel to its plate list, if the list is
        // non-empty.
        //
        //
        // Get the 3-d coordinates of the Ith voxel.
        //
        save.VOXCOR[3] = (((I - 1) / save.NXY) + 1);

        save.R = (I - ((save.VOXCOR[3] - 1) * save.NXY));

        save.VOXCOR[2] = (((save.R - 1) / save.NX) + 1);

        save.VOXCOR[1] = (save.R - ((save.VOXCOR[2] - 1) * save.NX));

        //
        // Get the coordinates of the coarse voxel containing the Ith
        // fine voxel. Also get the offsets of the fine voxel from the
        // base of the enclosing coarse voxel, expressed in both 3D and
        // 1D. Note that the offsets are 1-based.
        //
        spicelib::ZZVOXCVO(
            save.VOXCOR.as_slice(),
            save.NVOX.as_slice(),
            save.CORSCL,
            save.CGXYZ.as_slice_mut(),
            save.CGOFF.as_slice_mut(),
            &mut save.CGOF1D,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Get the 1-D coarse grid index corresponding to CGXYZ.
        //
        save.J = spicelib::ZZVOX2ID(save.CGXYZ.as_slice(), save.NCGVOX.as_slice());

        //
        // Get the coarse grid pointer for the current voxel.
        //
        save.CGP = save.CGRPTR[save.J];

        if (save.CGP > 0) {
            //
            // The pointer is non-null, so this coarse voxel is non-empty.
            // (That is, some plates intersect it.)
            //
            // CGP is the index of the first entry in the voxel pointer
            // array for the current coarse voxel. Get the index of the
            // entry for the current fine voxel, and look up the pointer
            // at that index.
            //
            save.K = ((save.CGP + save.CGOF1D) - 1);

            //
            // Make sure K is in range.
            //
            testutil::CHCKSI(b"K", save.K, b">", 0, 0, OK, ctx)?;
            testutil::CHCKSI(b"K", save.K, b"<", (save.NVXPTR + 1), 0, OK, ctx)?;

            save.VPTR = save.VOXPTR[save.K];

            if (save.VPTR > 0) {
                //
                // This voxel is non-empty.
                //
                // Let NVP be the plate count for this voxel.
                //
                save.NVP = save.VXLIST[save.VPTR];
                //
                // Check each plate.
                //
                for PIX in 1..=save.NVP {
                    //
                    // Compute the extents of the vertices of the current
                    // plate.
                    //
                    save.PID = save.VXLIST[(save.VPTR + PIX)];

                    for PLTCO in 1..=3 {
                        save.PLTEXT[[1, PLTCO]] = spicelib::DPMAX();
                        save.PLTEXT[[2, PLTCO]] = spicelib::DPMIN();
                    }

                    for PLTCO in 1..=3 {
                        save.VIX = save.PLATES[[PLTCO, save.PID]];

                        for VRTCO in 1..=3 {
                            save.PLTEXT[[1, VRTCO]] = intrinsics::DMIN1(&[
                                save.VERTS[[VRTCO, save.VIX]],
                                save.PLTEXT[[1, VRTCO]],
                            ]);

                            save.PLTEXT[[2, VRTCO]] = intrinsics::DMAX1(&[
                                save.VERTS[[VRTCO, save.VIX]],
                                save.PLTEXT[[2, VRTCO]],
                            ]);
                        }
                    }

                    //
                    // Let LOWPT and HIGHPT be the coordinates of the corners
                    // of the box defined by the plate extents, where LOWPT
                    // contains the minimum coordinate values and HIGHPT
                    // contains the maximum values.
                    //
                    // Take tolerance into account.
                    //
                    for VRTCO in 1..=3 {
                        save.LOWPT[VRTCO] = (save.PLTEXT[[1, VRTCO]] - save.MDLTOL);
                        save.HIGHPT[VRTCO] = (save.PLTEXT[[2, VRTCO]] + save.MDLTOL);
                    }

                    //
                    // Map the corner's model coordinates to voxel
                    // coordinates.
                    //
                    spicelib::ZZGETVOX(
                        save.VOXSIZ,
                        save.VOXORI.as_slice(),
                        save.NVOX.as_slice(),
                        save.LOWPT.as_slice(),
                        &mut save.INBOX,
                        save.LOWVOX.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::ZZGETVOX(
                        save.VOXSIZ,
                        save.VOXORI.as_slice(),
                        save.NVOX.as_slice(),
                        save.HIGHPT.as_slice(),
                        &mut save.INBOX,
                        save.HIVOX.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // The Ith voxel should be in the set bounded by LOWVOX
                    // and HIVOX.
                    //
                    save.INCLUD = true;

                    for VRTCO in 1..=3 {
                        save.INCLUD = ((save.INCLUD && (save.VOXCOR[VRTCO] >= save.LOWVOX[VRTCO]))
                            && (save.VOXCOR[VRTCO] <= save.HIVOX[VRTCO]));
                    }

                    //
                    // We'd like to think that each plate on the voxel's
                    // plate list does intersect that voxel. But maybe it
                    // doesn't...
                    //
                    if !save.INCLUD {
                        fstr::assign(&mut save.LABEL, b"Plate @ intersects voxel @");

                        spicelib::REPMI(&save.LABEL.to_vec(), b"@", save.PID, &mut save.LABEL, ctx);
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                        spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        testutil::CHCKSL(&save.LABEL, save.INCLUD, true, OK, ctx)?;
                    }
                }
            }
        }
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Test the voxel-plate map: find the set of  voxels touched by each plate. Check each voxel associated with a given plate to verify that the plate is on that voxel\'s plate list.", ctx)?;

    //
    // We repeat some of the code above...someday some more private
    // routines should be written to handle these computations.
    //

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NP;
        let m3__: i32 = 1;
        save.PID = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Find the extents of the Ith plate.
            //
            for PLTCO in 1..=3 {
                save.PLTEXT[[1, PLTCO]] = spicelib::DPMAX();
                save.PLTEXT[[2, PLTCO]] = spicelib::DPMIN();
            }

            for PLTCO in 1..=3 {
                save.VIX = save.PLATES[[PLTCO, save.PID]];

                for VRTCO in 1..=3 {
                    save.PLTEXT[[1, VRTCO]] = intrinsics::DMIN1(&[
                        save.VERTS[[VRTCO, save.VIX]],
                        save.PLTEXT[[1, VRTCO]],
                    ]);

                    save.PLTEXT[[2, VRTCO]] = intrinsics::DMAX1(&[
                        save.VERTS[[VRTCO, save.VIX]],
                        save.PLTEXT[[2, VRTCO]],
                    ]);
                }
            }

            //
            // Let LOWPT and HIGHPT be the coordinates of the corners
            // of the box defined by the plate extents, where LOWPT
            // contains the minimum coordinate values and HIGHPT
            // contains the maximum values.
            //
            // Take tolerance into account.
            //
            for VRTCO in 1..=3 {
                save.LOWPT[VRTCO] = (save.PLTEXT[[1, VRTCO]] - save.MDLTOL);
                save.HIGHPT[VRTCO] = (save.PLTEXT[[2, VRTCO]] + save.MDLTOL);
            }

            //
            // Map the corner's model coordinates to voxel
            // coordinates.
            //
            spicelib::ZZGETVOX(
                save.VOXSIZ,
                save.VOXORI.as_slice(),
                save.NVOX.as_slice(),
                save.LOWPT.as_slice(),
                &mut save.INBOX,
                save.LOWVOX.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::ZZGETVOX(
                save.VOXSIZ,
                save.VOXORI.as_slice(),
                save.NVOX.as_slice(),
                save.HIGHPT.as_slice(),
                &mut save.INBOX,
                save.HIVOX.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // For each voxel in the box bounded by LOWVOX and HIVOX, check
            // the Ith plate against that voxel's plate list.
            //
            for I in save.LOWVOX[1]..=save.HIVOX[1] {
                save.VOXCOR[1] = I;

                {
                    let m1__: i32 = save.LOWVOX[2];
                    let m2__: i32 = save.HIVOX[2];
                    let m3__: i32 = 1;
                    save.J = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        save.VOXCOR[2] = save.J;

                        {
                            let m1__: i32 = save.LOWVOX[3];
                            let m2__: i32 = save.HIVOX[3];
                            let m3__: i32 = 1;
                            save.K = m1__;
                            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                                save.VOXCOR[3] = save.K;

                                //
                                // Start out assuming we have no intersection.
                                // Update INCLUD if we find out otherwise.
                                //
                                save.INCLUD = false;

                                spicelib::ZZVOXCVO(
                                    save.VOXCOR.as_slice(),
                                    save.NVOX.as_slice(),
                                    save.CORSCL,
                                    save.CGXYZ.as_slice_mut(),
                                    save.CGOFF.as_slice_mut(),
                                    &mut save.CGOF1D,
                                    ctx,
                                )?;
                                testutil::CHCKXC(false, b" ", OK, ctx)?;

                                //
                                // Get the 1-D coarse grid index corresponding to CGXYZ.
                                //
                                save.CVOXID = spicelib::ZZVOX2ID(
                                    save.CGXYZ.as_slice(),
                                    save.NCGVOX.as_slice(),
                                );

                                //
                                // Get the coarse grid pointer for the current voxel.
                                //
                                save.CGP = save.CGRPTR[save.CVOXID];

                                if (save.CGP > 0) {
                                    //
                                    // The pointer is non-null, so this coarse voxel is
                                    // non-empty. (That is, some plates intersect it.)
                                    //
                                    // CGP is the index of the first entry in the voxel
                                    // pointer array for the current coarse voxel. Get
                                    // the index of the entry for the current fine voxel,
                                    // and look up the pointer at that index.
                                    //
                                    save.VPTR = save.VOXPTR[((save.CGP + save.CGOF1D) - 1)];

                                    if (save.VPTR > 0) {
                                        //
                                        // This voxel is non-empty.
                                        //
                                        // Let NVP be the plate count for this voxel.
                                        //
                                        save.NVP = save.VXLIST[save.VPTR];

                                        save.LOC = spicelib::ISRCHI(
                                            save.PID,
                                            save.NVP,
                                            save.VXLIST.subarray((save.VPTR + 1)),
                                        );

                                        save.INCLUD = (save.LOC > 0);
                                    }
                                }

                                //
                                // Make sure the plate was found on the list for
                                // the current voxel.
                                //
                                if !save.INCLUD {
                                    save.VOXID = spicelib::ZZVOX2ID(
                                        save.VOXCOR.as_slice(),
                                        save.NVOX.as_slice(),
                                    );

                                    fstr::assign(&mut save.LABEL, b"Plate @ intersects voxel @");

                                    spicelib::REPMI(
                                        &save.LABEL.to_vec(),
                                        b"@",
                                        save.PID,
                                        &mut save.LABEL,
                                        ctx,
                                    );
                                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                                    spicelib::REPMI(
                                        &save.LABEL.to_vec(),
                                        b"@",
                                        save.VOXID,
                                        &mut save.LABEL,
                                        ctx,
                                    );
                                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                                    testutil::CHCKSL(&save.LABEL, save.INCLUD, true, OK, ctx)?;
                                }

                                save.K += m3__;
                            }
                        }

                        save.J += m3__;
                    }
                }
            }

            save.PID += m3__;
        }
    }

    //***********************************************************************
    //
    //     Error cases
    //
    //***********************************************************************

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Error: bad plate count.", ctx)?;

    spicelib::ZZMKSPIN(
        0,
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

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    spicelib::ZZMKSPIN(
        -1,
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

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Error: coarse voxel scale out of range.", ctx)?;

    save.CORSCL = 0;

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

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Error: too many fine voxels.", ctx)?;

    save.FINSCL = 0.1;
    save.CORSCL = 100;

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

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Error: too many coarse voxels.", ctx)?;

    //
    // Set voxel scales.
    //
    save.FINSCL = 0.5;
    save.CORSCL = 1;

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

    testutil::CHCKXC(true, b"SPICE(COARSEGRIDOVERFLOW)", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
