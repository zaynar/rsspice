//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LBCELL: i32 = -5;
const LNSIZE: i32 = 80;
const MAXBND: i32 = 10001;
const MAXROW: i32 = 1000;
const MAXCOL: i32 = 1000;
const MAXORD: i32 = (2 * MAXCOL);
const MAXGRD: i32 = (MAXROW * MAXCOL);
const TITLEN: i32 = 320;

struct SaveVars {
    TITLE: Vec<u8>,
    LABEL: Vec<u8>,
    BNDS1: ActualArray2D<f64>,
    BNDS2: ActualArray2D<f64>,
    COSET1: ActualArray<f64>,
    COSET2: ActualArray<f64>,
    H: f64,
    MAPBD1: f64,
    MAPBD2: f64,
    MID1: f64,
    MID2: f64,
    PXMAX1: f64,
    PXMAX2: f64,
    PXMIN1: f64,
    PXMIN2: f64,
    W: f64,
    C1: i32,
    C2: i32,
    CIVOR1: ActualArray<i32>,
    CIVOR2: ActualArray<i32>,
    COL: i32,
    I: i32,
    J: i32,
    K: i32,
    L: i32,
    M: i32,
    MAPCO1: i32,
    MAPCO2: i32,
    MRKSET: ActualArray<i32>,
    N1: i32,
    N2: i32,
    NCOLS: i32,
    NCOVER: i32,
    NREC: i32,
    NROWS: i32,
    ORD1: ActualArray<i32>,
    ORD2: ActualArray<i32>,
    PXMAP1: ActualArray<i32>,
    PXMAP2: ActualArray<i32>,
    SEED: i32,
    TMPSET: ActualArray<i32>,
    VSET: ActualArray<i32>,
    XCIVO1: ActualArray<i32>,
    XCIVO2: ActualArray<i32>,
    XMAP1: ActualArray<i32>,
    XMAP2: ActualArray<i32>,
    XNCOLS: i32,
    XNROWS: i32,
    GRID: ActualArray<bool>,
    INCLUD: bool,
    VALUE: bool,
    XVALUE: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TITLE = vec![b' '; TITLEN as usize];
        let mut LABEL = vec![b' '; LNSIZE as usize];
        let mut BNDS1 = ActualArray2D::<f64>::new(1..=2, 1..=MAXBND);
        let mut BNDS2 = ActualArray2D::<f64>::new(1..=2, 1..=MAXBND);
        let mut COSET1 = ActualArray::<f64>::new(LBCELL..=MAXORD);
        let mut COSET2 = ActualArray::<f64>::new(LBCELL..=MAXORD);
        let mut H: f64 = 0.0;
        let mut MAPBD1: f64 = 0.0;
        let mut MAPBD2: f64 = 0.0;
        let mut MID1: f64 = 0.0;
        let mut MID2: f64 = 0.0;
        let mut PXMAX1: f64 = 0.0;
        let mut PXMAX2: f64 = 0.0;
        let mut PXMIN1: f64 = 0.0;
        let mut PXMIN2: f64 = 0.0;
        let mut W: f64 = 0.0;
        let mut C1: i32 = 0;
        let mut C2: i32 = 0;
        let mut CIVOR1 = ActualArray::<i32>::new(1..=MAXORD);
        let mut CIVOR2 = ActualArray::<i32>::new(1..=MAXORD);
        let mut COL: i32 = 0;
        let mut I: i32 = 0;
        let mut J: i32 = 0;
        let mut K: i32 = 0;
        let mut L: i32 = 0;
        let mut M: i32 = 0;
        let mut MAPCO1: i32 = 0;
        let mut MAPCO2: i32 = 0;
        let mut MRKSET = ActualArray::<i32>::new(LBCELL..=MAXGRD);
        let mut N1: i32 = 0;
        let mut N2: i32 = 0;
        let mut NCOLS: i32 = 0;
        let mut NCOVER: i32 = 0;
        let mut NREC: i32 = 0;
        let mut NROWS: i32 = 0;
        let mut ORD1 = ActualArray::<i32>::new(1..=MAXORD);
        let mut ORD2 = ActualArray::<i32>::new(1..=MAXORD);
        let mut PXMAP1 = ActualArray::<i32>::new(1..=MAXORD);
        let mut PXMAP2 = ActualArray::<i32>::new(1..=MAXORD);
        let mut SEED: i32 = 0;
        let mut TMPSET = ActualArray::<i32>::new(LBCELL..=MAXGRD);
        let mut VSET = ActualArray::<i32>::new(LBCELL..=MAXGRD);
        let mut XCIVO1 = ActualArray::<i32>::new(1..=MAXORD);
        let mut XCIVO2 = ActualArray::<i32>::new(1..=MAXORD);
        let mut XMAP1 = ActualArray::<i32>::new(1..=MAXORD);
        let mut XMAP2 = ActualArray::<i32>::new(1..=MAXORD);
        let mut XNCOLS: i32 = 0;
        let mut XNROWS: i32 = 0;
        let mut GRID = ActualArray::<bool>::new(1..=MAXGRD);
        let mut INCLUD: bool = false;
        let mut VALUE: bool = false;
        let mut XVALUE: bool = false;

        Self {
            TITLE,
            LABEL,
            BNDS1,
            BNDS2,
            COSET1,
            COSET2,
            H,
            MAPBD1,
            MAPBD2,
            MID1,
            MID2,
            PXMAX1,
            PXMAX2,
            PXMIN1,
            PXMIN2,
            W,
            C1,
            C2,
            CIVOR1,
            CIVOR2,
            COL,
            I,
            J,
            K,
            L,
            M,
            MAPCO1,
            MAPCO2,
            MRKSET,
            N1,
            N2,
            NCOLS,
            NCOVER,
            NREC,
            NROWS,
            ORD1,
            ORD2,
            PXMAP1,
            PXMAP2,
            SEED,
            TMPSET,
            VSET,
            XCIVO1,
            XCIVO2,
            XMAP1,
            XMAP2,
            XNCOLS,
            XNROWS,
            GRID,
            INCLUD,
            VALUE,
            XVALUE,
        }
    }
}

//$Procedure F_RC2GRD ( RC2GRD tests )
pub fn F_RC2GRD(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // TESTUTIL functions
    //

    //
    // SPICELIB functions
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
    testutil::TOPEN(b"F_RC2GRD", ctx)?;

    //**********************************************************************
    //
    //     RC2GRD normal cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"Simple case: two disjoint rectangles, common Y bounds",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // Initialize all sets.
    //
    spicelib::SSIZEI(MAXGRD, save.MRKSET.as_slice_mut(), ctx)?;
    spicelib::SSIZEI(MAXGRD, save.TMPSET.as_slice_mut(), ctx)?;
    spicelib::SSIZEI(MAXGRD, save.VSET.as_slice_mut(), ctx)?;

    //
    // Note that the output grid should have a column corresponding to
    // the horizontal gap between the rectangles.
    //
    save.XNROWS = 1;
    save.XNCOLS = 3;

    save.BNDS1[[1, 1]] = 1.0;
    save.BNDS1[[2, 1]] = 3.0;

    save.BNDS1[[1, 2]] = 4.0;
    save.BNDS1[[2, 2]] = 5.0;

    save.BNDS2[[1, 1]] = -1.0;
    save.BNDS2[[2, 1]] = 1.0;

    save.BNDS2[[1, 2]] = -1.0;
    save.BNDS2[[2, 2]] = 1.0;

    save.NREC = 2;

    //
    // Find the coverage of the grid.
    //
    save.INCLUD = true;

    support::RC2GRD(
        save.NREC,
        save.BNDS1.as_slice(),
        save.BNDS2.as_slice(),
        MAXGRD,
        MAXORD,
        save.INCLUD,
        save.ORD1.as_slice_mut(),
        save.ORD2.as_slice_mut(),
        save.CIVOR1.as_slice_mut(),
        save.CIVOR2.as_slice_mut(),
        save.PXMAP1.as_slice_mut(),
        save.PXMAP2.as_slice_mut(),
        &mut save.NROWS,
        &mut save.NCOLS,
        save.GRID.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the output grid dimensions. We're looking for a 1x3 grid.
    //
    testutil::CHCKSI(b"NROWS", save.NROWS, b"=", save.XNROWS, 0, OK, ctx)?;
    testutil::CHCKSI(b"NCOLS", save.NCOLS, b"=", save.XNCOLS, 0, OK, ctx)?;

    //
    // Check the inverse order vectors. First, create the
    // expected vectors.
    //
    save.I = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = (save.XNCOLS + 1);
        let m3__: i32 = 1;
        save.COL = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.K = (((save.COL - 1) / 2) + 1);
            save.J = (save.COL - (2 * (save.K - 1)));

            for ROW in 1..=save.XNROWS {
                save.XCIVO1[save.I] = save.COL;
                save.I = (save.I + 1);
            }

            save.COL += m3__;
        }
    }

    testutil::CHCKAI(
        b"CIVOR1",
        save.CIVOR1.as_slice(),
        b"=",
        save.XCIVO1.as_slice(),
        (save.XNCOLS + 1),
        OK,
        ctx,
    )?;

    save.I = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = (save.XNCOLS + 1);
        let m3__: i32 = 1;
        save.COL = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.K = (((save.COL - 1) / 2) + 1);
            save.J = (save.COL - (2 * (save.K - 1)));

            for ROW in 1..=(save.XNROWS + 1) {
                save.XCIVO2[save.I] = ROW;
                save.I = (save.I + 1);
            }

            save.COL += m3__;
        }
    }

    testutil::CHCKAI(
        b"CIVOR2",
        save.CIVOR2.as_slice(),
        b"=",
        save.XCIVO2.as_slice(),
        (save.XNROWS + 1),
        OK,
        ctx,
    )?;

    //
    // Check the pixel maps.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = (save.XNCOLS + 1);
        let m3__: i32 = 1;
        save.COL = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.LABEL, b"Grid(1,*) coord 1");
            spicelib::REPMI(&save.LABEL.to_vec(), b"*", save.COL, &mut save.LABEL, ctx);

            save.MAPCO1 = save.PXMAP1[save.COL];

            save.J = (((save.MAPCO1 - 1) / 2) + 1);

            save.I = (save.MAPCO1 - (2 * (save.J - 1)));

            save.MAPBD1 = save.BNDS1[[save.I, save.J]];

            save.J = (((save.COL - 1) / 2) + 1);

            save.I = (save.COL - (2 * (save.J - 1)));

            testutil::CHCKSD(
                &save.LABEL,
                save.MAPBD1,
                b"=",
                save.BNDS1[[save.I, save.J]],
                0.0,
                OK,
                ctx,
            )?;

            save.COL += m3__;
        }
    }

    for ROW in 1..=(save.XNROWS + 1) {
        fstr::assign(&mut save.LABEL, b"Grid(*,1) coord 2");
        spicelib::REPMI(&save.LABEL.to_vec(), b"*", save.COL, &mut save.LABEL, ctx);

        save.MAPCO2 = save.PXMAP2[ROW];

        save.J = (((save.MAPCO2 - 1) / 2) + 1);

        save.I = (save.MAPCO2 - (2 * (save.J - 1)));

        save.MAPBD2 = save.BNDS2[[save.I, save.J]];

        save.J = (((ROW - 1) / 2) + 1);

        save.I = (ROW - (2 * (save.J - 1)));

        testutil::CHCKSD(
            &save.LABEL,
            save.MAPBD2,
            b"=",
            save.BNDS2[[save.I, save.J]],
            0.0,
            OK,
            ctx,
        )?;
    }

    //
    // Check the grid. Marked pixels are included in the
    // rectangle set.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.XNCOLS;
        let m3__: i32 = 1;
        save.COL = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            for ROW in 1..=save.XNROWS {
                fstr::assign(&mut save.LABEL, b"Grid(*,*)");
                spicelib::REPMI(&save.LABEL.to_vec(), b"*", ROW, &mut save.LABEL, ctx);
                spicelib::REPMI(&save.LABEL.to_vec(), b"*", save.COL, &mut save.LABEL, ctx);

                save.I = (ROW + ((save.COL - 1) * save.NROWS));

                save.XVALUE = (save.COL != 2);

                testutil::CHCKSL(&save.LABEL, save.GRID[save.I], save.XVALUE, OK, ctx)?;
            }

            save.COL += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"Simple case: two overlapping rectangles, both X and Y bounds overlap.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // Initialize all sets.
    //
    spicelib::SSIZEI(MAXGRD, save.MRKSET.as_slice_mut(), ctx)?;
    spicelib::SSIZEI(MAXGRD, save.TMPSET.as_slice_mut(), ctx)?;
    spicelib::SSIZEI(MAXGRD, save.VSET.as_slice_mut(), ctx)?;

    //
    // Note that the output grid should have a column corresponding to
    // the horizontal gap between the rectangles.
    //
    save.XNROWS = 3;
    save.XNCOLS = 3;

    save.BNDS1[[1, 1]] = 1.0;
    save.BNDS1[[2, 1]] = 3.0;

    save.BNDS1[[1, 2]] = 2.0;
    save.BNDS1[[2, 2]] = 4.0;

    save.BNDS2[[1, 1]] = -1.0;
    save.BNDS2[[2, 1]] = 1.0;

    save.BNDS2[[1, 2]] = 0.0;
    save.BNDS2[[2, 2]] = 2.0;

    save.NREC = 2;

    //
    // Find the coverage of the grid.
    //
    save.INCLUD = true;

    support::RC2GRD(
        save.NREC,
        save.BNDS1.as_slice(),
        save.BNDS2.as_slice(),
        MAXGRD,
        MAXORD,
        save.INCLUD,
        save.ORD1.as_slice_mut(),
        save.ORD2.as_slice_mut(),
        save.CIVOR1.as_slice_mut(),
        save.CIVOR2.as_slice_mut(),
        save.PXMAP1.as_slice_mut(),
        save.PXMAP2.as_slice_mut(),
        &mut save.NROWS,
        &mut save.NCOLS,
        save.GRID.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the output grid dimensions. We're looking for a 3x3 grid.
    //
    testutil::CHCKSI(b"NROWS", save.NROWS, b"=", save.XNROWS, 0, OK, ctx)?;
    testutil::CHCKSI(b"NCOLS", save.NCOLS, b"=", save.XNCOLS, 0, OK, ctx)?;

    //
    // Check the inverse order vectors. First, create the
    // expected vectors.
    //

    save.XCIVO1[1] = 1;
    save.XCIVO1[2] = 3;
    save.XCIVO1[3] = 2;
    save.XCIVO1[4] = 4;

    testutil::CHCKAI(
        b"CIVOR1",
        save.CIVOR1.as_slice(),
        b"=",
        save.XCIVO1.as_slice(),
        (save.XNCOLS + 1),
        OK,
        ctx,
    )?;

    save.XCIVO2[1] = 1;
    save.XCIVO2[2] = 3;
    save.XCIVO2[3] = 2;
    save.XCIVO2[4] = 4;

    testutil::CHCKAI(
        b"CIVOR2",
        save.CIVOR2.as_slice(),
        b"=",
        save.XCIVO2.as_slice(),
        (save.XNROWS + 1),
        OK,
        ctx,
    )?;

    //
    // Check the pixel maps.
    //
    save.XMAP1[1] = 1;
    save.XMAP1[2] = 3;
    save.XMAP1[3] = 2;
    save.XMAP1[4] = 4;

    testutil::CHCKAI(
        b"PXMAP1",
        save.PXMAP1.as_slice(),
        b"=",
        save.XMAP1.as_slice(),
        (save.XNCOLS + 1),
        OK,
        ctx,
    )?;

    save.XMAP2[1] = 1;
    save.XMAP2[2] = 3;
    save.XMAP2[3] = 2;
    save.XMAP2[4] = 4;

    testutil::CHCKAI(
        b"PXMAP2",
        save.PXMAP2.as_slice(),
        b"=",
        save.XMAP2.as_slice(),
        (save.XNROWS + 1),
        OK,
        ctx,
    )?;

    //
    // Check the grid. Marked pixels are included in the
    // rectangle set.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.XNCOLS;
        let m3__: i32 = 1;
        save.COL = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            for ROW in 1..=save.XNROWS {
                fstr::assign(&mut save.LABEL, b"Grid(*,*)");
                spicelib::REPMI(&save.LABEL.to_vec(), b"*", ROW, &mut save.LABEL, ctx);
                spicelib::REPMI(&save.LABEL.to_vec(), b"*", save.COL, &mut save.LABEL, ctx);

                save.XVALUE = true;

                if ((ROW == 3) && (save.COL == 1)) {
                    save.XVALUE = false;
                } else if ((ROW == 1) && (save.COL == 3)) {
                    save.XVALUE = false;
                }

                save.I = (ROW + ((save.COL - 1) * save.NROWS));

                testutil::CHCKSL(&save.LABEL, save.GRID[save.I], save.XVALUE, OK, ctx)?;
            }

            save.COL += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Simple case: 3x5 grid, no gaps.");

    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // Initialize all sets.
    //
    spicelib::SSIZEI(MAXGRD, save.MRKSET.as_slice_mut(), ctx)?;
    spicelib::SSIZEI(MAXGRD, save.TMPSET.as_slice_mut(), ctx)?;
    spicelib::SSIZEI(MAXGRD, save.VSET.as_slice_mut(), ctx)?;

    //
    // VALUE is the logical value indicating a gap.
    //
    save.VALUE = false;
    save.INCLUD = !save.VALUE;

    save.XNROWS = 3;
    save.XNCOLS = 5;

    save.I = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.XNCOLS;
        let m3__: i32 = 1;
        save.COL = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            for ROW in 1..=save.XNROWS {
                save.BNDS1[[1, save.I]] = save.COL as f64;
                save.BNDS1[[2, save.I]] = (save.COL + 1) as f64;
                save.BNDS2[[1, save.I]] = ROW as f64;
                save.BNDS2[[2, save.I]] = (ROW + 1) as f64;

                save.I = (save.I + 1);
            }

            save.COL += m3__;
        }
    }

    save.NREC = (save.XNROWS * save.XNCOLS);

    //
    // Find the coverage of the grid.
    //
    support::RC2GRD(
        save.NREC,
        save.BNDS1.as_slice(),
        save.BNDS2.as_slice(),
        MAXGRD,
        MAXORD,
        save.INCLUD,
        save.ORD1.as_slice_mut(),
        save.ORD2.as_slice_mut(),
        save.CIVOR1.as_slice_mut(),
        save.CIVOR2.as_slice_mut(),
        save.PXMAP1.as_slice_mut(),
        save.PXMAP2.as_slice_mut(),
        &mut save.NROWS,
        &mut save.NCOLS,
        save.GRID.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the output grid dimensions. We're looking for a 3x5 grid.
    //
    testutil::CHCKSI(b"NROWS", save.NROWS, b"=", save.XNROWS, 0, OK, ctx)?;
    testutil::CHCKSI(b"NCOLS", save.NCOLS, b"=", save.XNCOLS, 0, OK, ctx)?;

    //
    // Check the inverse order vectors. First, create the
    // expected vectors.
    //
    save.I = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.XNCOLS;
        let m3__: i32 = 1;
        save.COL = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            for ROW in 1..=save.XNROWS {
                save.XCIVO1[save.I] = save.COL;
                save.XCIVO1[(save.I + 1)] = (save.COL + 1);

                save.I = (save.I + 2);
            }

            save.COL += m3__;
        }
    }

    testutil::CHCKAI(
        b"CIVOR1",
        save.CIVOR1.as_slice(),
        b"=",
        save.XCIVO1.as_slice(),
        (2 * save.XNCOLS),
        OK,
        ctx,
    )?;

    save.I = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.XNCOLS;
        let m3__: i32 = 1;
        save.COL = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            for ROW in 1..=save.XNROWS {
                save.XCIVO2[save.I] = ROW;
                save.XCIVO2[(save.I + 1)] = (ROW + 1);

                save.I = (save.I + 2);
            }

            save.COL += m3__;
        }
    }

    testutil::CHCKAI(
        b"CIVOR2",
        save.CIVOR2.as_slice(),
        b"=",
        save.XCIVO2.as_slice(),
        (2 * save.XNROWS),
        OK,
        ctx,
    )?;

    //
    // Check the pixel maps.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = (save.XNCOLS + 1);
        let m3__: i32 = 1;
        save.COL = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            for ROW in 1..=(save.XNROWS + 1) {
                fstr::assign(&mut save.LABEL, b"Grid(*,*) coord 1");
                spicelib::REPMI(&save.LABEL.to_vec(), b"*", ROW, &mut save.LABEL, ctx);
                spicelib::REPMI(&save.LABEL.to_vec(), b"*", save.COL, &mut save.LABEL, ctx);

                save.MAPCO1 = save.PXMAP1[save.COL];
                save.MAPCO2 = save.PXMAP2[ROW];

                save.J = (((save.MAPCO1 - 1) / 2) + 1);

                save.I = (save.MAPCO1 - (2 * (save.J - 1)));

                save.MAPBD1 = save.BNDS1[[save.I, save.J]];

                if (save.COL <= save.XNCOLS) {
                    //
                    // Pick the left X-bound of the rectangle in the first row
                    // of column COL.
                    //
                    save.K = (1 + (save.NROWS * (save.COL - 1)));

                    testutil::CHCKSD(
                        &save.LABEL,
                        save.MAPBD1,
                        b"=",
                        save.BNDS1[[1, save.K]],
                        0.0,
                        OK,
                        ctx,
                    )?;
                } else {
                    //
                    // Pick the right X-bound of the rectangle in the first row
                    // of column XNCOLS.
                    //
                    save.K = (1 + (save.NROWS * (save.COL - 2)));

                    testutil::CHCKSD(
                        &save.LABEL,
                        save.MAPBD1,
                        b"=",
                        save.BNDS1[[2, save.K]],
                        0.0,
                        OK,
                        ctx,
                    )?;
                }

                fstr::assign(&mut save.LABEL, b"Grid(*,*) coord 2");
                spicelib::REPMI(&save.LABEL.to_vec(), b"*", ROW, &mut save.LABEL, ctx);
                spicelib::REPMI(&save.LABEL.to_vec(), b"*", save.COL, &mut save.LABEL, ctx);

                save.J = (((save.MAPCO2 - 1) / 2) + 1);

                save.I = (save.MAPCO2 - (2 * (save.J - 1)));

                save.MAPBD2 = save.BNDS2[[save.I, save.J]];

                if (ROW <= save.XNROWS) {
                    //
                    // Pick the lower Y-bound of the rectangle in the first
                    // column of row ROW.
                    //
                    save.K = ROW;

                    testutil::CHCKSD(
                        &save.LABEL,
                        save.MAPBD2,
                        b"=",
                        save.BNDS2[[1, save.K]],
                        0.0,
                        OK,
                        ctx,
                    )?;
                } else {
                    //
                    // Pick the right X-bound of the rectangle in the first row
                    // of column XNCOLS.
                    //
                    save.K = save.NROWS;

                    testutil::CHCKSD(
                        &save.LABEL,
                        save.MAPBD2,
                        b"=",
                        save.BNDS2[[2, save.K]],
                        0.0,
                        OK,
                        ctx,
                    )?;
                }
            }

            save.COL += m3__;
        }
    }

    //
    // Check the grid. Marked pixels are included in the
    // rectangle set.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.XNCOLS;
        let m3__: i32 = 1;
        save.COL = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            for ROW in 1..=save.XNROWS {
                fstr::assign(&mut save.LABEL, b"Grid(*,*)");
                spicelib::REPMI(&save.LABEL.to_vec(), b"*", ROW, &mut save.LABEL, ctx);
                spicelib::REPMI(&save.LABEL.to_vec(), b"*", save.COL, &mut save.LABEL, ctx);

                save.I = (ROW + ((save.COL - 1) * save.NROWS));

                testutil::CHCKSL(&save.LABEL, save.GRID[save.I], save.INCLUD, OK, ctx)?;
            }

            save.COL += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"Simple case: 3x5 grid, no gaps. Reverse the order of the rectangle inputs.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // Initialize all sets.
    //
    spicelib::SSIZEI(MAXGRD, save.MRKSET.as_slice_mut(), ctx)?;
    spicelib::SSIZEI(MAXGRD, save.TMPSET.as_slice_mut(), ctx)?;
    spicelib::SSIZEI(MAXGRD, save.VSET.as_slice_mut(), ctx)?;

    //
    // VALUE is the logical value indicating a gap.
    //
    save.VALUE = false;
    save.INCLUD = !save.VALUE;

    save.XNROWS = 3;
    save.XNCOLS = 5;

    save.I = 1;

    {
        let m1__: i32 = save.XNCOLS;
        let m2__: i32 = 1;
        let m3__: i32 = -1;
        save.COL = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            for ROW in intrinsics::range(save.XNROWS, 1, -1) {
                save.BNDS1[[1, save.I]] = save.COL as f64;
                save.BNDS1[[2, save.I]] = (save.COL + 1) as f64;
                save.BNDS2[[1, save.I]] = ROW as f64;
                save.BNDS2[[2, save.I]] = (ROW + 1) as f64;

                save.I = (save.I + 1);
            }

            save.COL += m3__;
        }
    }

    save.NREC = (save.XNROWS * save.XNCOLS);

    //
    // Find the coverage of the grid.
    //
    support::RC2GRD(
        save.NREC,
        save.BNDS1.as_slice(),
        save.BNDS2.as_slice(),
        MAXGRD,
        MAXORD,
        save.INCLUD,
        save.ORD1.as_slice_mut(),
        save.ORD2.as_slice_mut(),
        save.CIVOR1.as_slice_mut(),
        save.CIVOR2.as_slice_mut(),
        save.PXMAP1.as_slice_mut(),
        save.PXMAP2.as_slice_mut(),
        &mut save.NROWS,
        &mut save.NCOLS,
        save.GRID.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the output grid dimensions. We're looking for a 3x5 grid.
    //
    testutil::CHCKSI(b"NROWS", save.NROWS, b"=", save.XNROWS, 0, OK, ctx)?;
    testutil::CHCKSI(b"NCOLS", save.NCOLS, b"=", save.XNCOLS, 0, OK, ctx)?;

    //
    // Check the inverse order vectors. First, create the
    // expected vectors.
    //
    save.I = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.XNCOLS;
        let m3__: i32 = 1;
        save.COL = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            for ROW in 1..=save.XNROWS {
                save.XCIVO1[save.I] = ((save.XNCOLS + 1) - save.COL);
                save.XCIVO1[(save.I + 1)] = ((save.XNCOLS + 2) - save.COL);

                save.I = (save.I + 2);
            }

            save.COL += m3__;
        }
    }

    testutil::CHCKAI(
        b"CIVOR1",
        save.CIVOR1.as_slice(),
        b"=",
        save.XCIVO1.as_slice(),
        (2 * save.XNCOLS),
        OK,
        ctx,
    )?;

    save.I = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.XNCOLS;
        let m3__: i32 = 1;
        save.COL = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            for ROW in 1..=save.XNROWS {
                save.XCIVO2[save.I] = ((save.XNROWS + 1) - ROW);
                save.XCIVO2[(save.I + 1)] = ((save.XNROWS + 2) - ROW);

                save.I = (save.I + 2);
            }

            save.COL += m3__;
        }
    }

    testutil::CHCKAI(
        b"CIVOR2",
        save.CIVOR2.as_slice(),
        b"=",
        save.XCIVO2.as_slice(),
        (2 * save.XNROWS),
        OK,
        ctx,
    )?;

    //
    // Check the pixel maps.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = (save.XNCOLS + 1);
        let m3__: i32 = 1;
        save.COL = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            for ROW in 1..=(save.XNROWS + 1) {
                fstr::assign(&mut save.LABEL, b"Grid(*,*) coord 1");
                spicelib::REPMI(&save.LABEL.to_vec(), b"*", ROW, &mut save.LABEL, ctx);
                spicelib::REPMI(&save.LABEL.to_vec(), b"*", save.COL, &mut save.LABEL, ctx);

                save.MAPCO1 = save.PXMAP1[save.COL];
                save.MAPCO2 = save.PXMAP2[ROW];

                save.J = (((save.MAPCO1 - 1) / 2) + 1);

                save.I = (save.MAPCO1 - (2 * (save.J - 1)));

                save.MAPBD1 = save.BNDS1[[save.I, save.J]];

                if (save.COL <= save.XNCOLS) {
                    //
                    // Pick the left X-bound of the rectangle in the first row
                    // of column COL.
                    //
                    save.K = (1 + (save.NROWS * (save.COL - 1)));

                // CALL CHCKSD ( LABEL, MAPBD1, '=', BNDS1(1,K), 0.D0, OK )
                } else {
                    //
                    // Pick the right X-bound of the rectangle in the first row
                    // of column XNCOLS.
                    //
                    save.K = (1 + (save.NROWS * (save.COL - 2)));

                    // CALL CHCKSD ( LABEL, MAPBD1, '=', BNDS1(2,K), 0.D0, OK )
                }

                fstr::assign(&mut save.LABEL, b"Grid(*,*) coord 2");
                spicelib::REPMI(&save.LABEL.to_vec(), b"*", ROW, &mut save.LABEL, ctx);
                spicelib::REPMI(&save.LABEL.to_vec(), b"*", save.COL, &mut save.LABEL, ctx);

                save.J = (((save.MAPCO2 - 1) / 2) + 1);

                save.I = (save.MAPCO2 - (2 * (save.J - 1)));

                save.MAPBD2 = save.BNDS2[[save.I, save.J]];

                if (ROW <= save.XNROWS) {
                    //
                    // Pick the lower Y-bound of the rectangle in the first
                    // column of row ROW.
                    //
                    save.K = ((save.XNROWS + 1) - ROW);

                    testutil::CHCKSD(
                        &save.LABEL,
                        save.MAPBD2,
                        b"=",
                        save.BNDS2[[1, save.K]],
                        0.0,
                        OK,
                        ctx,
                    )?;
                } else {
                    //
                    // Pick the right X-bound of the rectangle in the first row
                    // of column XNCOLS.
                    //
                    save.K = 1;

                    testutil::CHCKSD(
                        &save.LABEL,
                        save.MAPBD2,
                        b"=",
                        save.BNDS2[[2, save.K]],
                        0.0,
                        OK,
                        ctx,
                    )?;
                }
            }

            save.COL += m3__;
        }
    }

    //
    // Check the grid. Marked pixels are included in the
    // rectangle set.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.XNCOLS;
        let m3__: i32 = 1;
        save.COL = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            for ROW in 1..=save.XNROWS {
                fstr::assign(&mut save.LABEL, b"Grid(*,*)");
                spicelib::REPMI(&save.LABEL.to_vec(), b"*", ROW, &mut save.LABEL, ctx);
                spicelib::REPMI(&save.LABEL.to_vec(), b"*", save.COL, &mut save.LABEL, ctx);

                save.I = (ROW + ((save.COL - 1) * save.NROWS));

                testutil::CHCKSL(&save.LABEL, save.GRID[save.I], save.INCLUD, OK, ctx)?;
            }

            save.COL += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Test a random set of rectangles.");

    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // Initialize all sets.
    //
    spicelib::SSIZEI(MAXGRD, save.MRKSET.as_slice_mut(), ctx)?;
    spicelib::SSIZEI(MAXGRD, save.TMPSET.as_slice_mut(), ctx)?;
    spicelib::SSIZEI(MAXGRD, save.VSET.as_slice_mut(), ctx)?;

    save.NREC = 100;

    save.H = 20.0;
    save.W = 30.0;

    save.SEED = -1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NREC;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Set the bounds of the Ith rectangle.
            //
            save.BNDS1[[1, save.I]] = testutil::T_RANDD(
                -(save.W / 2 as f64),
                (save.W / 2 as f64),
                &mut save.SEED,
                ctx,
            )?;
            save.BNDS1[[2, save.I]] = testutil::T_RANDD(
                save.BNDS1[[1, save.I]],
                (save.W / 2 as f64),
                &mut save.SEED,
                ctx,
            )?;

            save.BNDS2[[1, save.I]] = testutil::T_RANDD(
                -(save.H / 2 as f64),
                (save.H / 2 as f64),
                &mut save.SEED,
                ctx,
            )?;
            save.BNDS2[[2, save.I]] = testutil::T_RANDD(
                save.BNDS2[[1, save.I]],
                (save.H / 2 as f64),
                &mut save.SEED,
                ctx,
            )?;

            //     WRITE (*,*) 'corners: ', BNDS1(1,I),BNDS2(1,I),
            // .                            BNDS1(2,I),BNDS2(2,I)

            save.I += m3__;
        }
    }

    //
    // Make sets of the X bounds and Y bounds. The cardinalities of
    // these sets will enable us to compute the expected grid
    // dimensions.
    //
    save.N1 = (2 * save.NREC);
    spicelib::MOVED(save.BNDS1.as_slice(), save.N1, save.COSET1.subarray_mut(1));
    spicelib::VALIDD(MAXORD, save.N1, save.COSET1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.C1 = spicelib::CARDD(save.COSET1.as_slice(), ctx)?;
    save.XNCOLS = (save.C1 - 1);

    save.N2 = (2 * save.NREC);
    spicelib::MOVED(save.BNDS1.as_slice(), save.N1, save.COSET2.subarray_mut(1));
    spicelib::VALIDD(MAXORD, save.N2, save.COSET2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.C2 = spicelib::CARDD(save.COSET2.as_slice(), ctx)?;
    save.XNROWS = (save.C2 - 1);

    //
    // Find the coverage of the grid.
    //
    save.INCLUD = true;

    support::RC2GRD(
        save.NREC,
        save.BNDS1.as_slice(),
        save.BNDS2.as_slice(),
        MAXGRD,
        MAXORD,
        save.INCLUD,
        save.ORD1.as_slice_mut(),
        save.ORD2.as_slice_mut(),
        save.CIVOR1.as_slice_mut(),
        save.CIVOR2.as_slice_mut(),
        save.PXMAP1.as_slice_mut(),
        save.PXMAP2.as_slice_mut(),
        &mut save.NROWS,
        &mut save.NCOLS,
        save.GRID.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the grid dimensions.
    //
    testutil::CHCKSI(b"NROWS", save.NROWS, b"=", save.XNROWS, 0, OK, ctx)?;
    testutil::CHCKSI(b"NCOLS", save.NCOLS, b"=", save.XNCOLS, 0, OK, ctx)?;

    //
    // Check the pixels. Count the covered pixels as we go.
    //
    save.NCOVER = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOLS;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Find the first coordinates of the boundaries corresponding
            // to the pixel at coordinates (J,I) in the pixel grid.
            //
            save.M = (((save.PXMAP1[save.I] - 1) / 2) + 1);

            save.L = (save.PXMAP1[save.I] - (2 * (save.M - 1)));

            save.PXMIN1 = save.BNDS1[[save.L, save.M]];

            save.M = (((save.PXMAP1[(save.I + 1)] - 1) / 2) + 1);

            save.L = (save.PXMAP1[(save.I + 1)] - (2 * (save.M - 1)));

            save.PXMAX1 = save.BNDS1[[save.L, save.M]];

            save.MID1 = ((save.PXMIN1 + save.PXMAX1) / 2 as f64);

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NROWS;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    //
                    // Find the second coordinates of the boundaries corresponding
                    // to the pixel at coordinates (J,I) in the pixel grid.
                    //
                    save.M = (((save.PXMAP2[save.J] - 1) / 2) + 1);

                    save.L = (save.PXMAP2[save.J] - (2 * (save.M - 1)));

                    save.PXMIN2 = save.BNDS2[[save.L, save.M]];

                    save.M = (((save.PXMAP2[(save.J + 1)] - 1) / 2) + 1);

                    save.L = (save.PXMAP2[(save.J + 1)] - (2 * (save.M - 1)));

                    save.PXMAX2 = save.BNDS2[[save.L, save.M]];

                    //
                    // Check whether this pixel is inside any rectangle.
                    // It suffices to check the center of the pixel.
                    //
                    save.MID2 = ((save.PXMIN2 + save.PXMAX2) / 2 as f64);

                    save.XVALUE = false;

                    {
                        let m1__: i32 = 1;
                        let m2__: i32 = save.NREC;
                        let m3__: i32 = 1;
                        save.K = m1__;
                        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                            if ((((save.BNDS1[[1, save.K]] < save.MID1)
                                && (save.BNDS1[[2, save.K]] > save.MID1))
                                && (save.BNDS2[[1, save.K]] < save.MID2))
                                && (save.BNDS2[[2, save.K]] > save.MID2))
                            {
                                save.XVALUE = true;
                            }

                            save.K += m3__;
                        }
                    }

                    if save.XVALUE {
                        save.NCOVER = (save.NCOVER + 1);
                    }

                    //
                    // If the current pixel is in one of the original rectangles,
                    // the XVALUE is .TRUE. Otherwise XVALUE is .FALSE.
                    //

                    fstr::assign(&mut save.LABEL, b"Pixel(*,*)");
                    spicelib::REPMI(&save.LABEL.to_vec(), b"*", save.J, &mut save.LABEL, ctx);
                    spicelib::REPMI(&save.LABEL.to_vec(), b"*", save.I, &mut save.LABEL, ctx);

                    save.K = (save.J + ((save.I - 1) * save.NROWS));

                    testutil::CHCKSL(&save.LABEL, save.GRID[save.K], save.XVALUE, OK, ctx)?;

                    save.J += m3__;
                }
            }

            save.I += m3__;
        }
    }

    //**********************************************************************
    //
    //     RC2GRD error cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Invalid value of NREC.");

    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // Initialize all sets.
    //
    spicelib::SSIZEI(MAXGRD, save.MRKSET.as_slice_mut(), ctx)?;
    spicelib::SSIZEI(MAXGRD, save.TMPSET.as_slice_mut(), ctx)?;
    spicelib::SSIZEI(MAXGRD, save.VSET.as_slice_mut(), ctx)?;

    //
    // Note that the output grid should have a column corresponding to
    // the horizontal gap between the rectangles.
    //
    save.XNROWS = 1;
    save.XNCOLS = 3;

    save.BNDS1[[1, 1]] = 1.0;
    save.BNDS1[[2, 1]] = 3.0;

    save.BNDS1[[1, 2]] = 4.0;
    save.BNDS1[[2, 2]] = 5.0;

    save.BNDS2[[1, 1]] = -1.0;
    save.BNDS2[[2, 1]] = 1.0;

    save.BNDS2[[1, 2]] = -1.0;
    save.BNDS2[[2, 2]] = 1.0;

    //
    // Find the coverage of the grid.
    //
    save.INCLUD = true;

    save.NREC = 0;

    support::RC2GRD(
        save.NREC,
        save.BNDS1.as_slice(),
        save.BNDS2.as_slice(),
        MAXGRD,
        MAXORD,
        save.INCLUD,
        save.ORD1.as_slice_mut(),
        save.ORD2.as_slice_mut(),
        save.CIVOR1.as_slice_mut(),
        save.CIVOR2.as_slice_mut(),
        save.PXMAP1.as_slice_mut(),
        save.PXMAP2.as_slice_mut(),
        &mut save.NROWS,
        &mut save.NCOLS,
        save.GRID.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    save.NREC = -1;

    support::RC2GRD(
        save.NREC,
        save.BNDS1.as_slice(),
        save.BNDS2.as_slice(),
        MAXGRD,
        MAXORD,
        save.INCLUD,
        save.ORD1.as_slice_mut(),
        save.ORD2.as_slice_mut(),
        save.CIVOR1.as_slice_mut(),
        save.CIVOR2.as_slice_mut(),
        save.PXMAP1.as_slice_mut(),
        save.PXMAP2.as_slice_mut(),
        &mut save.NROWS,
        &mut save.NCOLS,
        save.GRID.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"MAXGRD is too small.");

    testutil::TCASE(&save.TITLE, ctx)?;

    save.NREC = 2;

    //
    // Find the coverage of the grid.
    //
    save.INCLUD = true;

    support::RC2GRD(
        save.NREC,
        save.BNDS1.as_slice(),
        save.BNDS2.as_slice(),
        2,
        MAXORD,
        save.INCLUD,
        save.ORD1.as_slice_mut(),
        save.ORD2.as_slice_mut(),
        save.CIVOR1.as_slice_mut(),
        save.CIVOR2.as_slice_mut(),
        save.PXMAP1.as_slice_mut(),
        save.PXMAP2.as_slice_mut(),
        &mut save.NROWS,
        &mut save.NCOLS,
        save.GRID.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    support::RC2GRD(
        save.NREC,
        save.BNDS1.as_slice(),
        save.BNDS2.as_slice(),
        -1,
        MAXORD,
        save.INCLUD,
        save.ORD1.as_slice_mut(),
        save.ORD2.as_slice_mut(),
        save.CIVOR1.as_slice_mut(),
        save.CIVOR2.as_slice_mut(),
        save.PXMAP1.as_slice_mut(),
        save.PXMAP2.as_slice_mut(),
        &mut save.NROWS,
        &mut save.NCOLS,
        save.GRID.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"MAXORD is too small.");

    testutil::TCASE(&save.TITLE, ctx)?;

    save.NREC = 2;

    //
    // Find the coverage of the grid.
    //
    save.INCLUD = true;

    support::RC2GRD(
        save.NREC,
        save.BNDS1.as_slice(),
        save.BNDS2.as_slice(),
        MAXGRD,
        0,
        save.INCLUD,
        save.ORD1.as_slice_mut(),
        save.ORD2.as_slice_mut(),
        save.CIVOR1.as_slice_mut(),
        save.CIVOR2.as_slice_mut(),
        save.PXMAP1.as_slice_mut(),
        save.PXMAP2.as_slice_mut(),
        &mut save.NROWS,
        &mut save.NCOLS,
        save.GRID.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    support::RC2GRD(
        save.NREC,
        save.BNDS1.as_slice(),
        save.BNDS2.as_slice(),
        MAXGRD,
        -1,
        save.INCLUD,
        save.ORD1.as_slice_mut(),
        save.ORD2.as_slice_mut(),
        save.CIVOR1.as_slice_mut(),
        save.CIVOR2.as_slice_mut(),
        save.PXMAP1.as_slice_mut(),
        save.PXMAP2.as_slice_mut(),
        &mut save.NROWS,
        &mut save.NCOLS,
        save.GRID.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Invalid edge length.");

    testutil::TCASE(&save.TITLE, ctx)?;

    save.NREC = 2;

    save.XNROWS = 1;
    save.XNCOLS = 3;

    save.BNDS1[[1, 1]] = 1.0;
    save.BNDS1[[2, 1]] = 1.0;

    save.BNDS1[[1, 2]] = 4.0;
    save.BNDS1[[2, 2]] = 5.0;

    save.BNDS2[[1, 1]] = -1.0;
    save.BNDS2[[2, 1]] = 1.0;

    save.BNDS2[[1, 2]] = -1.0;
    save.BNDS2[[2, 2]] = 1.0;
    //
    // Find the coverage of the grid.
    //
    save.INCLUD = true;

    support::RC2GRD(
        save.NREC,
        save.BNDS1.as_slice(),
        save.BNDS2.as_slice(),
        MAXGRD,
        MAXORD,
        save.INCLUD,
        save.ORD1.as_slice_mut(),
        save.ORD2.as_slice_mut(),
        save.CIVOR1.as_slice_mut(),
        save.CIVOR2.as_slice_mut(),
        save.PXMAP1.as_slice_mut(),
        save.PXMAP2.as_slice_mut(),
        &mut save.NROWS,
        &mut save.NCOLS,
        save.GRID.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDBOUNDS)", OK, ctx)?;

    save.BNDS1[[1, 1]] = 1.0;
    save.BNDS1[[2, 1]] = 2.0;

    save.BNDS1[[1, 2]] = 4.0;
    save.BNDS1[[2, 2]] = 5.0;

    save.BNDS2[[1, 1]] = -1.0;
    save.BNDS2[[2, 1]] = 1.0;

    save.BNDS2[[1, 2]] = -1.0;
    save.BNDS2[[2, 2]] = -2.0;
    //
    // Find the coverage of the grid.
    //
    save.INCLUD = true;

    support::RC2GRD(
        save.NREC,
        save.BNDS1.as_slice(),
        save.BNDS2.as_slice(),
        MAXGRD,
        MAXORD,
        save.INCLUD,
        save.ORD1.as_slice_mut(),
        save.ORD2.as_slice_mut(),
        save.CIVOR1.as_slice_mut(),
        save.CIVOR2.as_slice_mut(),
        save.PXMAP1.as_slice_mut(),
        save.PXMAP2.as_slice_mut(),
        &mut save.NROWS,
        &mut save.NCOLS,
        save.GRID.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDBOUNDS)", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
