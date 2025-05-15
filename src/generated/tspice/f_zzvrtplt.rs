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
const LBLSIZ: i32 = 40;
const MAXV: i32 = 10000;
const MAXP: i32 = (2 * MAXV);
const MXCEL: i32 = (3 * MAXP);
const MXLIST: i32 = ((3 * MAXP) + MAXV);

struct SaveVars {
    LABEL: Vec<u8>,
    A: f64,
    B: f64,
    C: f64,
    VERTS: ActualArray2D<f64>,
    CELLS: ActualArray2D<i32>,
    CELSIZ: i32,
    I: i32,
    K: i32,
    PLATES: ActualArray2D<i32>,
    MAXLST: i32,
    N: i32,
    NLAT: i32,
    NLIST: i32,
    NLON: i32,
    NP: i32,
    NV: i32,
    P: i32,
    PLT: i32,
    PLTLST: ActualArray<i32>,
    VRTPTR: ActualArray<i32>,
    VTX: i32,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut LABEL = vec![b' '; LBLSIZ as usize];
        let mut A: f64 = 0.0;
        let mut B: f64 = 0.0;
        let mut C: f64 = 0.0;
        let mut VERTS = ActualArray2D::<f64>::new(1..=3, 1..=MAXV);
        let mut CELLS = ActualArray2D::<i32>::new(1..=2, 1..=MXCEL);
        let mut CELSIZ: i32 = 0;
        let mut I: i32 = 0;
        let mut K: i32 = 0;
        let mut PLATES = ActualArray2D::<i32>::new(1..=3, 1..=MAXP);
        let mut MAXLST: i32 = 0;
        let mut N: i32 = 0;
        let mut NLAT: i32 = 0;
        let mut NLIST: i32 = 0;
        let mut NLON: i32 = 0;
        let mut NP: i32 = 0;
        let mut NV: i32 = 0;
        let mut P: i32 = 0;
        let mut PLT: i32 = 0;
        let mut PLTLST = ActualArray::<i32>::new(1..=MXCEL);
        let mut VRTPTR = ActualArray::<i32>::new(1..=MAXV);
        let mut VTX: i32 = 0;
        let mut FOUND: bool = false;

        Self {
            LABEL,
            A,
            B,
            C,
            VERTS,
            CELLS,
            CELSIZ,
            I,
            K,
            PLATES,
            MAXLST,
            N,
            NLAT,
            NLIST,
            NLON,
            NP,
            NV,
            P,
            PLT,
            PLTLST,
            VRTPTR,
            VTX,
            FOUND,
        }
    }
}

//$Procedure      F_ZZVRTPLT ( Test ZZVRTPLT )
pub fn F_ZZVRTPLT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    testutil::TOPEN(b"F_ZZVRTPLT", ctx)?;

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
    testutil::TCASE(
        b"Create vertex-plate mapping for tessellated ellipsoid.",
        ctx,
    )?;

    save.MAXLST = MXLIST;

    spicelib::ZZVRTPLT(
        save.NV,
        save.NP,
        save.PLATES.as_slice(),
        MXCEL,
        save.MAXLST,
        save.CELLS.as_slice_mut(),
        save.VRTPTR.as_slice_mut(),
        &mut save.NLIST,
        save.PLTLST.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The output list size should be exactly
    //
    //    NV  +  3*NP
    //
    // since each plate is on exactly three vertex lists, and
    // since there is one plate count per vertex.
    //
    testutil::CHCKSI(
        b"NLIST",
        save.NLIST,
        b"=",
        (save.NV + (3 * save.NP)),
        0,
        OK,
        ctx,
    )?;

    //
    // Each vertex pointer should be in the range 1:NLIST-1.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NV;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.LABEL, b"VRTPTR(#)");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSI(&save.LABEL, *save.VRTPTR.first(), b">=", 1, 0, OK, ctx)?;
            testutil::CHCKSI(
                &save.LABEL,
                *save.VRTPTR.first(),
                b"<=",
                (save.NLIST - 1),
                0,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // Given the tiling pattern, except for the polar vertices,
    // each vertex's plate count should be in the range 1:6.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = (save.NV - 2);
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.N = save.PLTLST[save.VRTPTR[save.I]];
            fstr::assign(&mut save.LABEL, b"Plate count(#)");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSI(&save.LABEL, save.N, b">=", 1, 0, OK, ctx)?;
            testutil::CHCKSI(&save.LABEL, save.N, b"<=", 6, 0, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // The polar vertices are shared by NLON plates each.
    //
    save.I = (save.NV - 1);

    save.N = save.PLTLST[save.VRTPTR[save.I]];
    fstr::assign(&mut save.LABEL, b"Plate count(#)");
    spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(&save.LABEL, save.N, b"=", save.NLON, 0, OK, ctx)?;

    save.I = save.NV;

    save.N = save.PLTLST[save.VRTPTR[save.I]];
    fstr::assign(&mut save.LABEL, b"Plate count(#)");
    spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(&save.LABEL, save.N, b"=", save.NLON, 0, OK, ctx)?;

    //
    // Make sure each plate is on the plate lists of
    // its three vertices.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NP;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            for J in 1..=3 {
                //
                // P is the pointer from vertex J of plate I
                // into the plate list. N is the plate count
                // for that vertex.
                //
                save.VTX = save.PLATES[[J, save.I]];

                save.P = save.VRTPTR[save.VTX];

                save.N = save.PLTLST[save.P];

                save.FOUND = false;
                save.K = 0;

                while ((save.K < save.N) && !save.FOUND) {
                    //
                    // Examine the next plate in the list of the
                    // current vertex.
                    //
                    save.K = (save.K + 1);

                    save.PLT = save.PLTLST[(save.P + save.K)];

                    save.FOUND = (save.PLT == save.I);
                }

                fstr::assign(&mut save.LABEL, b"Entry for plate(#), vertex(#)");

                spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
                spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.VTX, &mut save.LABEL, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSL(&save.LABEL, save.FOUND, true, OK, ctx)?;
            }

            save.I += m3__;
        }
    }

    //
    // If the last test passed, each plate is on at least the three
    // plate lists of its own vertices. The plate is not on any other
    // vertex's list, since total entry count is correct. The plate list
    // is valid.
    //

    //***********************************************************************
    //
    //     Error cases
    //
    //***********************************************************************

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Bad plate count.", ctx)?;

    save.MAXLST = MXCEL;

    spicelib::ZZVRTPLT(
        save.NV,
        0,
        save.PLATES.as_slice(),
        MXCEL,
        save.MAXLST,
        save.CELLS.as_slice_mut(),
        save.VRTPTR.as_slice_mut(),
        &mut save.NLIST,
        save.PLTLST.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADPLATECOUNT)", OK, ctx)?;

    spicelib::ZZVRTPLT(
        save.NV,
        -1,
        save.PLATES.as_slice(),
        MXCEL,
        save.MAXLST,
        save.CELLS.as_slice_mut(),
        save.VRTPTR.as_slice_mut(),
        &mut save.NLIST,
        save.PLTLST.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADPLATECOUNT)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Bad vertex count.", ctx)?;

    save.MAXLST = MXCEL;

    spicelib::ZZVRTPLT(
        0,
        save.NP,
        save.PLATES.as_slice(),
        MXCEL,
        save.MAXLST,
        save.CELLS.as_slice_mut(),
        save.VRTPTR.as_slice_mut(),
        &mut save.NLIST,
        save.PLTLST.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADVERTEXCOUNT)", OK, ctx)?;

    spicelib::ZZVRTPLT(
        -1,
        save.NP,
        save.PLATES.as_slice(),
        MXCEL,
        save.MAXLST,
        save.CELLS.as_slice_mut(),
        save.VRTPTR.as_slice_mut(),
        &mut save.NLIST,
        save.PLTLST.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADVERTEXCOUNT)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Cell array too small.", ctx)?;

    save.MAXLST = MXCEL;

    save.CELSIZ = ((3 * save.NP) - 1);

    spicelib::ZZVRTPLT(
        save.NV,
        save.NP,
        save.PLATES.as_slice(),
        save.CELSIZ,
        save.MAXLST,
        save.CELLS.as_slice_mut(),
        save.VRTPTR.as_slice_mut(),
        &mut save.NLIST,
        save.PLTLST.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(CELLARRAYTOOSMALL)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Plate list too small.", ctx)?;

    save.MAXLST = (((3 * save.NP) + save.NV) - 1);

    spicelib::ZZVRTPLT(
        save.NV,
        save.NP,
        save.PLATES.as_slice(),
        MXCEL,
        save.MAXLST,
        save.CELLS.as_slice_mut(),
        save.VRTPTR.as_slice_mut(),
        &mut save.NLIST,
        save.PLTLST.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(PLATELISTTOOSMALL)", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
