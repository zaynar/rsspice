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
const DSK0: &[u8] = b"dskrb2_test.bds";
const TIGHT: f64 = 0.000000000001;
const MAXV: i32 = 10000;
const MAXP: i32 = (2 * MAXV);
const NSHAPE: i32 = 2;
const NAMLEN: i32 = 32;

struct SaveVars {
    FIXREF: Vec<u8>,
    A: f64,
    ALT: f64,
    B: f64,
    C: f64,
    CENTER: StackArray<f64, 3>,
    CORPAR: StackArray<f64, 10>,
    DIST: f64,
    F: f64,
    LAT: f64,
    LON: f64,
    MXCOR3: f64,
    MNCOR3: f64,
    NORMAL: StackArray<f64, 3>,
    ORIGIN: StackArray<f64, 3>,
    PLTCTR: StackArray<f64, 3>,
    PNEAR: StackArray<f64, 3>,
    R: f64,
    RCROSS: f64,
    S: f64,
    TOL: f64,
    VERTS: ActualArray2D<f64>,
    XMAX: f64,
    XMIN: f64,
    BODYID: i32,
    CORSYS: i32,
    DLADSC: StackArray<i32, 8>,
    HANDLE: i32,
    PLATES: ActualArray2D<i32>,
    N: i32,
    NCROSS: i32,
    NLAT: i32,
    NLON: i32,
    NP: i32,
    NPOLYV: i32,
    NV: i32,
    SURFID: i32,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FIXREF = vec![b' '; NAMLEN as usize];
        let mut A: f64 = 0.0;
        let mut ALT: f64 = 0.0;
        let mut B: f64 = 0.0;
        let mut C: f64 = 0.0;
        let mut CENTER = StackArray::<f64, 3>::new(1..=3);
        let mut CORPAR = StackArray::<f64, 10>::new(1..=NSYPAR);
        let mut DIST: f64 = 0.0;
        let mut F: f64 = 0.0;
        let mut LAT: f64 = 0.0;
        let mut LON: f64 = 0.0;
        let mut MXCOR3: f64 = 0.0;
        let mut MNCOR3: f64 = 0.0;
        let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
        let mut ORIGIN = StackArray::<f64, 3>::new(1..=3);
        let mut PLTCTR = StackArray::<f64, 3>::new(1..=3);
        let mut PNEAR = StackArray::<f64, 3>::new(1..=3);
        let mut R: f64 = 0.0;
        let mut RCROSS: f64 = 0.0;
        let mut S: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut VERTS = ActualArray2D::<f64>::new(1..=3, 1..=MAXV);
        let mut XMAX: f64 = 0.0;
        let mut XMIN: f64 = 0.0;
        let mut BODYID: i32 = 0;
        let mut CORSYS: i32 = 0;
        let mut DLADSC = StackArray::<i32, 8>::new(1..=DLADSZ);
        let mut HANDLE: i32 = 0;
        let mut PLATES = ActualArray2D::<i32>::new(1..=3, 1..=MAXP);
        let mut N: i32 = 0;
        let mut NCROSS: i32 = 0;
        let mut NLAT: i32 = 0;
        let mut NLON: i32 = 0;
        let mut NP: i32 = 0;
        let mut NPOLYV: i32 = 0;
        let mut NV: i32 = 0;
        let mut SURFID: i32 = 0;
        let mut FOUND: bool = false;

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
            A,
            ALT,
            B,
            C,
            CENTER,
            CORPAR,
            DIST,
            F,
            LAT,
            LON,
            MXCOR3,
            MNCOR3,
            NORMAL,
            ORIGIN,
            PLTCTR,
            PNEAR,
            R,
            RCROSS,
            S,
            TOL,
            VERTS,
            XMAX,
            XMIN,
            BODYID,
            CORSYS,
            DLADSC,
            HANDLE,
            PLATES,
            N,
            NCROSS,
            NLAT,
            NLON,
            NP,
            NPOLYV,
            NV,
            SURFID,
            FOUND,
        }
    }
}

//$Procedure      F_DSKRB2 ( Test DSKRB2 )
pub fn F_DSKRB2(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_DSKRB2", ctx)?;

    //***********************************************************************
    //
    //     Normal cases
    //
    //***********************************************************************

    //
    // Loop over the set of shapes.
    //
    for SHAPIX in 1..=NSHAPE {
        //
        // --- Case --------------------------------------------------------
        //
        if (SHAPIX == 1) {
            testutil::TCASE(b"Set-up: create plates for a tessellated ellipsoid. NLON = 100; NLAT = 50; NV = 4902; NP = 9800.", ctx)?;

            save.A = 3000.0;
            save.B = save.A;
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
        } else if (SHAPIX == 2) {
            //
            // Create a plate set consisting of a tessellated torus.
            // We'll create a DSK file containing the model, then
            // we'll extract the vertices and plates. We don't need
            // the file; it's just simpler this way.
            //

            testutil::TCASE(b"Set-up: create plates for a tessellated torus.", ctx)?;

            save.BODYID = 401;
            save.SURFID = 1;
            fstr::assign(&mut save.FIXREF, b"IAU_PHOBOS");
            save.NPOLYV = 100;
            save.NCROSS = 100;
            save.R = 3000.0;
            save.RCROSS = 300.0;

            spicelib::VPACK(30000.0, 40000.0, 50000.0, save.CENTER.as_slice_mut());
            spicelib::VPACK(0.0, 0.0, 1.0, save.NORMAL.as_slice_mut());

            if spicelib::EXISTS(DSK0, ctx)? {
                spicelib::DELFIL(DSK0, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
            }

            testutil::T_TORUS(
                save.BODYID,
                save.SURFID,
                &save.FIXREF,
                save.NPOLYV,
                save.NCROSS,
                save.R,
                save.RCROSS,
                save.CENTER.as_slice(),
                save.NORMAL.as_slice_mut(),
                DSK0,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Extract the plate set from the DSK we just created.
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

            testutil::CHCKSL(b"DLABFS found", save.FOUND, true, OK, ctx)?;

            spicelib::DSKZ02(
                save.HANDLE,
                save.DLADSC.as_slice(),
                &mut save.NV,
                &mut save.NP,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::DSKV02(
                save.HANDLE,
                save.DLADSC.as_slice(),
                1,
                save.NV,
                &mut save.N,
                save.VERTS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::DSKP02(
                save.HANDLE,
                save.DLADSC.as_slice(),
                1,
                save.NP,
                &mut save.N,
                save.PLATES.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::DASCLS(save.HANDLE, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        //
        // --- Case --------------------------------------------------------
        //

        testutil::TCASE(b"Check the radius extents of the plate set.", ctx)?;

        save.CORSYS = LATSYS;
        spicelib::CLEARD(NSYPAR, save.CORPAR.as_slice_mut());

        spicelib::DSKRB2(
            save.NV,
            save.VERTS.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.CORSYS,
            save.CORPAR.as_slice(),
            &mut save.MNCOR3,
            &mut save.MXCOR3,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Find the expected radius extents.
        //
        save.XMAX = spicelib::DPMIN();
        save.XMIN = spicelib::DPMAX();

        //
        // The maximum radius occurs at a vertex.
        //
        for I in 1..=save.NV {
            save.XMAX =
                intrinsics::DMAX1(&[save.XMAX, spicelib::VNORM(save.VERTS.subarray([1, I]))]);
        }
        //
        // The minimum radius could occur in the interior of
        // a plate. We must find the minimum distance of each
        // plate from the origin.
        //
        for I in 1..=save.NP {
            spicelib::PLTNP(
                save.ORIGIN.as_slice(),
                save.VERTS.subarray([1, save.PLATES[[1, I]]]),
                save.VERTS.subarray([1, save.PLATES[[2, I]]]),
                save.VERTS.subarray([1, save.PLATES[[3, I]]]),
                save.PNEAR.as_slice_mut(),
                &mut save.DIST,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.XMIN = intrinsics::DMIN1(&[save.XMIN, save.DIST]);
        }

        testutil::CHCKSD(b"MNCOR3", save.MNCOR3, b"~/", save.XMIN, save.TOL, OK, ctx)?;
        testutil::CHCKSD(b"MXCOR3", save.MXCOR3, b"~/", save.XMAX, save.TOL, OK, ctx)?;

        //
        // --- Case --------------------------------------------------------
        //
        testutil::TCASE(b"Check the Z extents of the plate set.", ctx)?;

        //
        // The coordinate system is rectangular.
        //

        save.CORSYS = RECSYS;
        spicelib::CLEARD(NSYPAR, save.CORPAR.as_slice_mut());

        spicelib::DSKRB2(
            save.NV,
            save.VERTS.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.CORSYS,
            save.CORPAR.as_slice(),
            &mut save.MNCOR3,
            &mut save.MXCOR3,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Find the expected Z extents. The extreme values occur at
        // vertices.
        //
        save.XMAX = spicelib::DPMIN();
        save.XMIN = spicelib::DPMAX();

        for I in 1..=save.NV {
            save.XMIN = intrinsics::DMIN1(&[save.XMIN, save.VERTS[[3, I]]]);
            save.XMAX = intrinsics::DMAX1(&[save.XMAX, save.VERTS[[3, I]]]);
        }

        testutil::CHCKSD(b"MNCOR3", save.MNCOR3, b"~/", save.XMIN, save.TOL, OK, ctx)?;
        testutil::CHCKSD(b"MXCOR3", save.MXCOR3, b"~/", save.XMAX, save.TOL, OK, ctx)?;

        //
        // --- Case --------------------------------------------------------
        //
        testutil::TCASE(b"Check the altitude extents of the plate set. Use the spheroid parameters that were used to create the tessellated spheroid.", ctx)?;

        //
        // The coordinate system is rectangular.
        //
        save.CORSYS = PDTSYS;
        spicelib::CLEARD(NSYPAR, save.CORPAR.as_slice_mut());

        save.CORPAR[1] = save.A;
        save.F = ((save.A - save.C) / save.A);
        save.CORPAR[2] = save.F;

        spicelib::DSKRB2(
            save.NV,
            save.VERTS.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.CORSYS,
            save.CORPAR.as_slice(),
            &mut save.MNCOR3,
            &mut save.MXCOR3,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Find the expected altitude extents. The maximum
        // values occur at vertices The minimum values can
        // occur with plates' interiors.
        //
        save.XMAX = spicelib::DPMIN();
        save.XMIN = spicelib::DPMAX();

        for I in 1..=save.NV {
            spicelib::RECGEO(
                save.VERTS.subarray([1, I]),
                save.A,
                save.F,
                &mut save.LON,
                &mut save.LAT,
                &mut save.ALT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.XMAX = intrinsics::DMAX1(&[save.XMAX, save.ALT]);
        }

        //
        // Minimum altitudes are a little more complicated. We don't
        // compute exact minima; we simply compute a reasonable lower
        // bound for the altitude of each plate. This is done by finding
        // the altitude of the plate's center and subtracting the maximum
        // distance from the center of the plate's vertices.
        //
        for I in 1..=save.NP {
            save.S = (1.0 / 3.0);

            spicelib::VLCOM3(
                save.S,
                save.VERTS.subarray([1, save.PLATES[[1, I]]]),
                save.S,
                save.VERTS.subarray([1, save.PLATES[[2, I]]]),
                save.S,
                save.VERTS.subarray([1, save.PLATES[[3, I]]]),
                save.PLTCTR.as_slice_mut(),
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::RECGEO(
                save.PLTCTR.as_slice(),
                save.A,
                save.F,
                &mut save.LON,
                &mut save.LAT,
                &mut save.ALT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.DIST = intrinsics::DMAX1(&[
                spicelib::VDIST(
                    save.PLTCTR.as_slice(),
                    save.VERTS.subarray([1, save.PLATES[[1, I]]]),
                ),
                spicelib::VDIST(
                    save.PLTCTR.as_slice(),
                    save.VERTS.subarray([1, save.PLATES[[2, I]]]),
                ),
                spicelib::VDIST(
                    save.PLTCTR.as_slice(),
                    save.VERTS.subarray([1, save.PLATES[[3, I]]]),
                ),
            ]);

            save.XMIN = intrinsics::DMIN1(&[save.XMIN, (save.ALT - save.DIST)]);
        }

        //
        // Note: use absolute comparisons for the ellipsoid case,
        // since the expected value may be zero.
        //
        save.TOL = TIGHT;

        if (SHAPIX == 1) {
            testutil::CHCKSD(b"MNCOR3", save.MNCOR3, b"~", save.XMIN, save.TOL, OK, ctx)?;
            testutil::CHCKSD(b"MXCOR3", save.MXCOR3, b"~", save.XMAX, save.TOL, OK, ctx)?;
        } else {
            testutil::CHCKSD(b"MNCOR3", save.MNCOR3, b"~/", save.XMIN, save.TOL, OK, ctx)?;
            testutil::CHCKSD(b"MXCOR3", save.MXCOR3, b"~/", save.XMAX, save.TOL, OK, ctx)?;
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
    testutil::TCASE(b"Bad coordinate system.", ctx)?;

    spicelib::DSKRB2(
        save.NV,
        save.VERTS.as_slice(),
        save.NP,
        save.PLATES.as_slice(),
        0,
        save.CORPAR.as_slice(),
        &mut save.MNCOR3,
        &mut save.MXCOR3,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Bad coordinate parameters.", ctx)?;

    save.CORSYS = PDTSYS;
    save.CORPAR[1] = 0.0;

    spicelib::DSKRB2(
        save.NV,
        save.VERTS.as_slice(),
        save.NP,
        save.PLATES.as_slice(),
        save.CORSYS,
        save.CORPAR.as_slice(),
        &mut save.MNCOR3,
        &mut save.MXCOR3,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    save.CORSYS = PDTSYS;
    save.CORPAR[1] = save.A;
    save.CORPAR[2] = 2.0;

    spicelib::DSKRB2(
        save.NV,
        save.VERTS.as_slice(),
        save.NP,
        save.PLATES.as_slice(),
        save.CORSYS,
        save.CORPAR.as_slice(),
        &mut save.MNCOR3,
        &mut save.MXCOR3,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //***********************************************************************
    //
    //     Clean up.
    //
    //***********************************************************************

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Clean up.", ctx)?;

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
