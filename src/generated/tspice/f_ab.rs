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
const MXCELL: i32 = 300;
const MAXA: i32 = 100;
const MAXB: i32 = 200;
const NULPTR: i32 = -1;
const LNSIZE: i32 = 80;
const NAVALS: i32 = 3;

struct SaveVars {
    LABEL: Vec<u8>,
    ALIST: StackArray<i32, 100>,
    AVALS: StackArray<i32, 3>,
    BLIST: StackArray<i32, 200>,
    BVAL: i32,
    CELLS: ActualArray2D<i32>,
    I: i32,
    J: i32,
    N: i32,
    NB: i32,
    NCELL: i32,
    XVAL: i32,
    XVALS: ActualArray<i32>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut LABEL = vec![b' '; LNSIZE as usize];
        let mut ALIST = StackArray::<i32, 100>::new(1..=MAXA);
        let mut AVALS = StackArray::<i32, 3>::new(1..=NAVALS);
        let mut BLIST = StackArray::<i32, 200>::new(1..=MAXB);
        let mut BVAL: i32 = 0;
        let mut CELLS = ActualArray2D::<i32>::new(1..=2, 1..=MXCELL);
        let mut I: i32 = 0;
        let mut J: i32 = 0;
        let mut N: i32 = 0;
        let mut NB: i32 = 0;
        let mut NCELL: i32 = 0;
        let mut XVAL: i32 = 0;
        let mut XVALS = ActualArray::<i32>::new(1..=MXCELL);

        Self {
            LABEL,
            ALIST,
            AVALS,
            BLIST,
            BVAL,
            CELLS,
            I,
            J,
            N,
            NB,
            NCELL,
            XVAL,
            XVALS,
        }
    }
}

//$Procedure      F_AB ( Test DSK AB list routines )
pub fn F_AB(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    testutil::TOPEN(b"F_AB", ctx)?;

    //***********************************************************************
    //
    //     ZZINILNK tests
    //
    //***********************************************************************

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Bad cell count", ctx)?;

    spicelib::ZZINILNK(
        MAXA,
        0,
        &mut save.NCELL,
        save.ALIST.as_slice_mut(),
        save.CELLS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Bad pointer count", ctx)?;

    spicelib::ZZINILNK(
        0,
        MXCELL,
        &mut save.NCELL,
        save.ALIST.as_slice_mut(),
        save.CELLS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Initialize an AB list", ctx)?;

    spicelib::ZZINILNK(
        MAXA,
        MXCELL,
        &mut save.NCELL,
        save.ALIST.as_slice_mut(),
        save.CELLS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The A list should have all entries initialized to "null."
    //
    spicelib::FILLI(NULPTR, MAXA, save.XVALS.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAI(
        b"ALIST",
        save.ALIST.as_slice(),
        b"=",
        save.XVALS.as_slice(),
        MAXA,
        OK,
        ctx,
    )?;

    //
    // The cell array should have all value elements set to 0.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = MXCELL;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.LABEL, b"CELLS(1,@)");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", save.I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSI(&save.LABEL, save.CELLS[[1, save.I]], b"=", 0, 0, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // The cell array should have all pointer elements set to NULPTR.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = MXCELL;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.LABEL, b"CELLS(2,@)");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", save.I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSI(
                &save.LABEL,
                save.CELLS[[2, save.I]],
                b"=",
                NULPTR,
                0,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // The pointer array should have all pointer elements set to NULPTR.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXA;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.LABEL, b"ALIST(@)");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", save.I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSI(&save.LABEL, save.ALIST[save.I], b"=", NULPTR, 0, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // The used cell count should be zero on output.
    //
    testutil::CHCKSI(b"NCELL", save.NCELL, b"=", 0, 0, OK, ctx)?;

    //***********************************************************************
    //
    //     ZZADDLNK tests
    //
    //***********************************************************************

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"AVAL out of range", ctx)?;

    spicelib::ZZINILNK(
        MAXA,
        MXCELL,
        &mut save.NCELL,
        save.ALIST.as_slice_mut(),
        save.CELLS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZADDLNK(
        0,
        1,
        MAXA,
        MXCELL,
        save.ALIST.as_slice_mut(),
        &mut save.NCELL,
        save.CELLS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(AVALOUTOFRANGE)", OK, ctx)?;

    spicelib::ZZADDLNK(
        -1,
        1,
        MAXA,
        MXCELL,
        save.ALIST.as_slice_mut(),
        &mut save.NCELL,
        save.CELLS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(AVALOUTOFRANGE)", OK, ctx)?;

    spicelib::ZZADDLNK(
        (MAXA + 1),
        1,
        MAXA,
        MXCELL,
        save.ALIST.as_slice_mut(),
        &mut save.NCELL,
        save.CELLS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(AVALOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Too many cell entries.", ctx)?;

    spicelib::ZZINILNK(
        MAXA,
        MXCELL,
        &mut save.NCELL,
        save.ALIST.as_slice_mut(),
        save.CELLS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = MXCELL;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::ZZADDLNK(
                1,
                save.I,
                MAXA,
                MXCELL,
                save.ALIST.as_slice_mut(),
                &mut save.NCELL,
                save.CELLS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    spicelib::ZZADDLNK(
        1,
        (save.I + 1),
        MAXA,
        MXCELL,
        save.ALIST.as_slice_mut(),
        &mut save.NCELL,
        save.CELLS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(CELLARRAYTOOSMALL)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Build trivial list for one A-value.", ctx)?;

    spicelib::ZZINILNK(
        MAXA,
        MXCELL,
        &mut save.NCELL,
        save.ALIST.as_slice_mut(),
        save.CELLS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = MXCELL;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::ZZADDLNK(
                1,
                save.I,
                MAXA,
                MXCELL,
                save.ALIST.as_slice_mut(),
                &mut save.NCELL,
                save.CELLS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Check NCELL.
    //
    testutil::CHCKSI(b"NCELL", save.NCELL, b"=", MXCELL, 0, OK, ctx)?;

    //
    // Check the pointer array. The first pointer should point to
    // the last cell; the other pointers should be null.
    //
    testutil::CHCKSI(b"ALIST(1)", save.ALIST[1], b"=", MXCELL, 0, OK, ctx)?;

    {
        let m1__: i32 = 2;
        let m2__: i32 = MAXA;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.LABEL, b"ALIST(@)");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", save.I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSI(&save.LABEL, save.ALIST[save.I], b"=", -1, 0, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Check the cell entries.
    //
    {
        let m1__: i32 = MXCELL;
        let m2__: i32 = 1;
        let m3__: i32 = -1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.LABEL, b"CELLS(1,@)");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", save.I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSI(
                &save.LABEL,
                save.CELLS[[1, save.I]],
                b"=",
                save.I,
                0,
                OK,
                ctx,
            )?;

            fstr::assign(&mut save.LABEL, b"CELLS(2,@)");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", save.I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            if (save.I > 1) {
                save.XVAL = (save.I - 1);
            } else {
                save.XVAL = -1;
            }

            testutil::CHCKSI(
                &save.LABEL,
                save.CELLS[[2, save.I]],
                b"=",
                save.XVAL,
                0,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Build a singleton list for each A-value.", ctx)?;

    spicelib::ZZINILNK(
        MAXA,
        MXCELL,
        &mut save.NCELL,
        save.ALIST.as_slice_mut(),
        save.CELLS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = MAXA;
        let m2__: i32 = 1;
        let m3__: i32 = -1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::ZZADDLNK(
                save.I,
                -save.I,
                MAXA,
                MXCELL,
                save.ALIST.as_slice_mut(),
                &mut save.NCELL,
                save.CELLS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Check NCELL.
    //
    testutil::CHCKSI(b"NCELL", save.NCELL, b"=", MAXA, 0, OK, ctx)?;

    //
    // Check the pointer array. The Ith pointer should point to
    // the cell at index MAXA+1-I.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXA;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.LABEL, b"ALIST(@)");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", save.I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSI(
                &save.LABEL,
                save.ALIST[save.I],
                b"=",
                ((MAXA + 1) - save.I),
                0,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // Check the cell entries.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXA;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.LABEL, b"CELLS(1,@)");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", save.I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.XVAL = -((MAXA + 1) - save.I);

            testutil::CHCKSI(
                &save.LABEL,
                save.CELLS[[1, save.I]],
                b"=",
                save.XVAL,
                0,
                OK,
                ctx,
            )?;

            fstr::assign(&mut save.LABEL, b"CELLS(2,@)");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", save.I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.XVAL = -1;

            testutil::CHCKSI(
                &save.LABEL,
                save.CELLS[[2, save.I]],
                b"=",
                save.XVAL,
                0,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"Build a list for each of three A-values, where the list additions are interleaved.",
        ctx,
    )?;

    spicelib::ZZINILNK(
        MAXA,
        MXCELL,
        &mut save.NCELL,
        save.ALIST.as_slice_mut(),
        save.CELLS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.AVALS[1] = 4;
    save.AVALS[2] = 1;
    save.AVALS[3] = 30;

    save.N = 20;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = NAVALS;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.BVAL = ((save.J * 1000) + save.I);

                    spicelib::ZZADDLNK(
                        save.AVALS[save.J],
                        save.BVAL,
                        MAXA,
                        MXCELL,
                        save.ALIST.as_slice_mut(),
                        &mut save.NCELL,
                        save.CELLS.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.J += m3__;
                }
            }

            save.I += m3__;
        }
    }

    //
    // Check NCELL.
    //
    testutil::CHCKSI(b"NCELL", save.NCELL, b"=", (NAVALS * save.N), 0, OK, ctx)?;

    //
    // Recover the B values for each A value.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NAVALS;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.J = save.ALIST[save.AVALS[save.I]];

            for K in 1..=save.N {
                save.BVAL = save.CELLS[[1, save.J]];

                save.XVAL = ((save.I * 1000) + ((save.N + 1) - K));

                fstr::assign(&mut save.LABEL, b"BLIST(@) for AVAL @");
                spicelib::REPMI(&save.LABEL.to_vec(), b"@", K, &mut save.LABEL, ctx);
                spicelib::REPMI(&save.LABEL.to_vec(), b"@", save.I, &mut save.LABEL, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSI(&save.LABEL, save.BVAL, b"=", save.XVAL, 0, OK, ctx)?;

                save.J = save.CELLS[[2, save.J]];
            }

            save.I += m3__;
        }
    }

    //***********************************************************************
    //
    //     ZZTRVLNK tests
    //
    //***********************************************************************

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"Collect the B-lists for AVALS from the previous ZZADDLNK case.",
        ctx,
    )?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NAVALS;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::ZZTRVLNK(
                save.AVALS[save.I],
                MAXA,
                save.ALIST.as_slice(),
                MXCELL,
                save.CELLS.as_slice(),
                MAXB,
                &mut save.NB,
                save.BLIST.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Check size of returned list.
            //
            testutil::CHCKSI(b"NB", save.NB, b"=", save.N, 0, OK, ctx)?;

            //
            // Check the list itself.
            //
            for K in 1..=save.N {
                save.XVAL = ((save.I * 1000) + ((save.N + 1) - K));

                fstr::assign(&mut save.LABEL, b"BLIST(@) for AVAL @");
                spicelib::REPMI(&save.LABEL.to_vec(), b"@", K, &mut save.LABEL, ctx);
                spicelib::REPMI(&save.LABEL.to_vec(), b"@", save.I, &mut save.LABEL, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSI(&save.LABEL, save.BLIST[K], b"=", save.XVAL, 0, OK, ctx)?;
            }

            save.I += m3__;
        }
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Pointer for AVAL is null.", ctx)?;

    save.I = -1;
    spicelib::ZZTRVLNK(
        1,
        MAXA,
        &[save.I],
        MXCELL,
        save.CELLS.as_slice(),
        MAXB,
        &mut save.NB,
        save.BLIST.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check size of returned list.
    //
    testutil::CHCKSI(b"NB", save.NB, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Pointer for AVAL is out of range.", ctx)?;

    save.I = -2;
    spicelib::ZZTRVLNK(
        1,
        MAXA,
        &[save.I],
        MXCELL,
        save.CELLS.as_slice(),
        MAXB,
        &mut save.NB,
        save.BLIST.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(POINTEROUTOFRANGE)", OK, ctx)?;

    save.I = 0;
    spicelib::ZZTRVLNK(
        1,
        MAXA,
        &[save.I],
        MXCELL,
        save.CELLS.as_slice(),
        MAXB,
        &mut save.NB,
        save.BLIST.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(POINTEROUTOFRANGE)", OK, ctx)?;

    save.I = (MXCELL + 1);
    spicelib::ZZTRVLNK(
        1,
        MAXA,
        &[save.I],
        MXCELL,
        save.CELLS.as_slice(),
        MAXB,
        &mut save.NB,
        save.BLIST.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(POINTEROUTOFRANGE)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"AVAL out of range.", ctx)?;

    spicelib::ZZTRVLNK(
        0,
        MAXA,
        save.ALIST.as_slice(),
        MXCELL,
        save.CELLS.as_slice(),
        MAXB,
        &mut save.NB,
        save.BLIST.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INDEXOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"MAXB out of range.", ctx)?;

    spicelib::ZZTRVLNK(
        1,
        MAXA,
        save.ALIST.as_slice(),
        MXCELL,
        save.CELLS.as_slice(),
        0,
        &mut save.NB,
        save.BLIST.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDSIZE)", OK, ctx)?;

    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"BLIST too large", ctx)?;

    spicelib::ZZTRVLNK(
        save.AVALS[1],
        MAXA,
        save.ALIST.as_slice(),
        MXCELL,
        save.CELLS.as_slice(),
        (save.N - 1),
        &mut save.NB,
        save.BLIST.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BARRAYTOOSMALL)", OK, ctx)?;

    //***********************************************************************
    //
    //     ZZUNTNGL tests
    //
    //***********************************************************************

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"Untangle the interleaved list structure from the last ZZADDLNK case.",
        ctx,
    )?;

    spicelib::ZZUNTNGL(
        MAXA,
        MXCELL,
        save.CELLS.as_slice(),
        MAXB,
        save.ALIST.as_slice_mut(),
        &mut save.NB,
        save.BLIST.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check size of returned array.
    //
    testutil::CHCKSI(b"NB", save.NB, b"=", (NAVALS * (save.N + 1)), 0, OK, ctx)?;

    //
    // Check the array itself.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NAVALS;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Let J be the start index of the Ith list. A count
            // should be present at that index.
            //
            save.J = save.ALIST[save.AVALS[save.I]];

            fstr::assign(&mut save.LABEL, b"Count for AVAL @");
            spicelib::REPMI(&save.LABEL.to_vec(), b"@", save.I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSI(&save.LABEL, save.BLIST[save.J], b"=", save.N, 0, OK, ctx)?;

            //
            // Check the B-values for the Ith list.
            //
            for K in 1..=save.N {
                save.XVAL = ((save.I * 1000) + ((save.N + 1) - K));

                fstr::assign(&mut save.LABEL, b"BLIST(@) for AVAL @");
                spicelib::REPMI(&save.LABEL.to_vec(), b"@", K, &mut save.LABEL, ctx);
                spicelib::REPMI(&save.LABEL.to_vec(), b"@", save.I, &mut save.LABEL, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSI(
                    &save.LABEL,
                    save.BLIST[(save.J + K)],
                    b"=",
                    save.XVAL,
                    0,
                    OK,
                    ctx,
                )?;
            }

            save.I += m3__;
        }
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Output array size too small.", ctx)?;

    spicelib::ZZUNTNGL(
        MAXA,
        MXCELL,
        save.CELLS.as_slice(),
        1,
        save.ALIST.as_slice_mut(),
        &mut save.NB,
        save.BLIST.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BARRAYTOOSMALL)", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
