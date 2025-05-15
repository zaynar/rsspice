//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LBCELL: i32 = -5;
const MAXROW: i32 = 1000;
const MAXCOL: i32 = 1000;
const MAXGRD: i32 = (MAXROW * MAXCOL);
const TITLEN: i32 = 320;
const LNSIZE: i32 = 80;

struct SaveVars {
    LABEL: Vec<u8>,
    TITLE: Vec<u8>,
    COL: i32,
    I: i32,
    J: i32,
    MAXPXX: ActualArray<i32>,
    MAXPXY: ActualArray<i32>,
    MINPXX: ActualArray<i32>,
    MINPXY: ActualArray<i32>,
    MRKSET: ActualArray<i32>,
    N: i32,
    NCOLS: i32,
    NCOMP: i32,
    NGAP: i32,
    NGRID: i32,
    NROWS: i32,
    ROW: i32,
    SEED: i32,
    TMPSET: ActualArray<i32>,
    VSET: ActualArray<i32>,
    W: i32,
    XBDS: ActualArray<i32>,
    GRID: ActualArray<bool>,
    GRID1: ActualArray<bool>,
    GRID2: ActualArray<bool>,
    VALUE: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut LABEL = vec![b' '; LNSIZE as usize];
        let mut TITLE = vec![b' '; TITLEN as usize];
        let mut COL: i32 = 0;
        let mut I: i32 = 0;
        let mut J: i32 = 0;
        let mut MAXPXX = ActualArray::<i32>::new(1..=MAXGRD);
        let mut MAXPXY = ActualArray::<i32>::new(1..=MAXGRD);
        let mut MINPXX = ActualArray::<i32>::new(1..=MAXGRD);
        let mut MINPXY = ActualArray::<i32>::new(1..=MAXGRD);
        let mut MRKSET = ActualArray::<i32>::new(LBCELL..=MAXGRD);
        let mut N: i32 = 0;
        let mut NCOLS: i32 = 0;
        let mut NCOMP: i32 = 0;
        let mut NGAP: i32 = 0;
        let mut NGRID: i32 = 0;
        let mut NROWS: i32 = 0;
        let mut ROW: i32 = 0;
        let mut SEED: i32 = 0;
        let mut TMPSET = ActualArray::<i32>::new(LBCELL..=MAXGRD);
        let mut VSET = ActualArray::<i32>::new(LBCELL..=MAXGRD);
        let mut W: i32 = 0;
        let mut XBDS = ActualArray::<i32>::new(1..=MAXGRD);
        let mut GRID = ActualArray::<bool>::new(1..=MAXGRD);
        let mut GRID1 = ActualArray::<bool>::new(1..=MAXGRD);
        let mut GRID2 = ActualArray::<bool>::new(1..=MAXGRD);
        let mut VALUE: bool = false;

        Self {
            LABEL,
            TITLE,
            COL,
            I,
            J,
            MAXPXX,
            MAXPXY,
            MINPXX,
            MINPXY,
            MRKSET,
            N,
            NCOLS,
            NCOMP,
            NGAP,
            NGRID,
            NROWS,
            ROW,
            SEED,
            TMPSET,
            VSET,
            W,
            XBDS,
            GRID,
            GRID1,
            GRID2,
            VALUE,
        }
    }
}

//$Procedure F_FNDCMP ( FNDCMP tests )
pub fn F_FNDCMP(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    testutil::TOPEN(b"F_FNDCMP", ctx)?;

    //**********************************************************************
    //
    //     FNDCMP error cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"Output arrays are too small to accommodate the output rectangle set.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // Initialize all sets.
    //
    spicelib::SSIZEI(MAXGRD, save.MRKSET.as_slice_mut(), ctx)?;
    spicelib::SSIZEI(MAXGRD, save.TMPSET.as_slice_mut(), ctx)?;
    spicelib::SSIZEI(MAXGRD, save.VSET.as_slice_mut(), ctx)?;

    save.VALUE = false;

    //
    // Note: the row and column count must be odd in order to
    // achieve the checkerboard patter we want.
    //
    save.NROWS = 5;
    save.NCOLS = 5;

    save.NGAP = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOLS;
        let m3__: i32 = 1;
        save.COL = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NROWS;
                let m3__: i32 = 1;
                save.ROW = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.I = (((save.COL - 1) * save.NROWS) + save.ROW);

                    if spicelib::EVEN(save.I) {
                        save.GRID[save.I] = false;
                        save.NGAP = (save.NGAP + 1);
                    } else {
                        save.GRID[save.I] = true;
                    }

                    save.ROW += m3__;
                }
            }

            save.COL += m3__;
        }
    }

    //
    // J is the maximum number of output components.
    //
    save.J = 5;

    support::FNDCMP(
        save.NROWS,
        save.NCOLS,
        save.VALUE,
        save.J,
        save.GRID.as_slice_mut(),
        save.VSET.as_slice_mut(),
        save.MRKSET.as_slice_mut(),
        save.TMPSET.as_slice_mut(),
        &mut save.NCOMP,
        save.MINPXX.as_slice_mut(),
        save.MAXPXX.as_slice_mut(),
        save.MINPXY.as_slice_mut(),
        save.MAXPXY.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ARRAYTOOSMALL)", OK, ctx)?;

    //**********************************************************************
    //
    //     FNDCMP normal cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Trivial case: 3x3 grid, no gaps.");

    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // Initialize all sets.
    //
    // VALUE is the logical value indicating a gap.
    //
    save.VALUE = false;

    save.NROWS = 3;
    save.NCOLS = 3;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOLS;
        let m3__: i32 = 1;
        save.COL = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NROWS;
                let m3__: i32 = 1;
                save.ROW = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.I = (((save.COL - 1) * save.NROWS) + save.ROW);

                    save.GRID[save.I] = true;

                    save.ROW += m3__;
                }
            }

            save.COL += m3__;
        }
    }

    //
    // Find the gaps in the grid.
    //
    support::FNDCMP(
        save.NROWS,
        save.NCOLS,
        save.VALUE,
        MAXGRD,
        save.GRID.as_slice_mut(),
        save.VSET.as_slice_mut(),
        save.MRKSET.as_slice_mut(),
        save.TMPSET.as_slice_mut(),
        &mut save.NCOMP,
        save.MINPXX.as_slice_mut(),
        save.MAXPXX.as_slice_mut(),
        save.MINPXY.as_slice_mut(),
        save.MAXPXY.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NCOMP", save.NCOMP, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Trivial case: 3x3 grid, one big gap.");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::SCARDI(0, save.MRKSET.as_slice_mut(), ctx)?;
    spicelib::SCARDI(0, save.TMPSET.as_slice_mut(), ctx)?;
    spicelib::SCARDI(0, save.VSET.as_slice_mut(), ctx)?;

    //
    // VALUE is the logical value indicating a gap.
    //
    save.VALUE = false;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOLS;
        let m3__: i32 = 1;
        save.COL = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NROWS;
                let m3__: i32 = 1;
                save.ROW = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.I = (((save.COL - 1) * save.NROWS) + save.ROW);

                    save.GRID[save.I] = false;

                    save.ROW += m3__;
                }
            }

            save.COL += m3__;
        }
    }

    //
    // Find the gaps in the grid.
    //
    support::FNDCMP(
        save.NROWS,
        save.NCOLS,
        save.VALUE,
        MAXGRD,
        save.GRID.as_slice_mut(),
        save.VSET.as_slice_mut(),
        save.MRKSET.as_slice_mut(),
        save.TMPSET.as_slice_mut(),
        &mut save.NCOMP,
        save.MINPXX.as_slice_mut(),
        save.MAXPXX.as_slice_mut(),
        save.MINPXY.as_slice_mut(),
        save.MAXPXY.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NCOMP", save.NCOMP, b"=", 1, 0, OK, ctx)?;

    //
    // Check bounds of the single gap.
    //
    testutil::CHCKSI(b"MINPXX", *save.MINPXX.first(), b"=", 1, 0, OK, ctx)?;
    testutil::CHCKSI(b"MAXPXX", *save.MAXPXX.first(), b"=", 3, 0, OK, ctx)?;

    testutil::CHCKSI(b"MINPXY", *save.MINPXY.first(), b"=", 1, 0, OK, ctx)?;
    testutil::CHCKSI(b"MAXPXY", *save.MAXPXY.first(), b"=", 3, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"3x3 grid, gap at element (2,2).");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::SCARDI(0, save.MRKSET.as_slice_mut(), ctx)?;
    spicelib::SCARDI(0, save.TMPSET.as_slice_mut(), ctx)?;
    spicelib::SCARDI(0, save.VSET.as_slice_mut(), ctx)?;

    //
    // VALUE is the logical value indicating a gap.
    //
    save.VALUE = false;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOLS;
        let m3__: i32 = 1;
        save.COL = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NROWS;
                let m3__: i32 = 1;
                save.ROW = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.I = (((save.COL - 1) * save.NROWS) + save.ROW);

                    save.GRID[save.I] = true;

                    save.ROW += m3__;
                }
            }

            save.COL += m3__;
        }
    }

    save.COL = 2;
    save.ROW = 2;
    save.I = (((save.COL - 1) * save.NROWS) + save.ROW);

    save.GRID[save.I] = false;

    //
    // Find the gaps in the grid.
    //
    support::FNDCMP(
        save.NROWS,
        save.NCOLS,
        save.VALUE,
        MAXGRD,
        save.GRID.as_slice_mut(),
        save.VSET.as_slice_mut(),
        save.MRKSET.as_slice_mut(),
        save.TMPSET.as_slice_mut(),
        &mut save.NCOMP,
        save.MINPXX.as_slice_mut(),
        save.MAXPXX.as_slice_mut(),
        save.MINPXY.as_slice_mut(),
        save.MAXPXY.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NCOMP", save.NCOMP, b"=", 1, 0, OK, ctx)?;

    //
    // Check bounds of the single gap.
    //
    testutil::CHCKSI(b"MINPXX", *save.MINPXX.first(), b"=", 2, 0, OK, ctx)?;
    testutil::CHCKSI(b"MAXPXX", *save.MAXPXX.first(), b"=", 2, 0, OK, ctx)?;

    testutil::CHCKSI(b"MINPXY", *save.MINPXY.first(), b"=", 2, 0, OK, ctx)?;
    testutil::CHCKSI(b"MAXPXY", *save.MAXPXY.first(), b"=", 2, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"3x3 grid, middle column is a gap.");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::SCARDI(0, save.MRKSET.as_slice_mut(), ctx)?;
    spicelib::SCARDI(0, save.TMPSET.as_slice_mut(), ctx)?;
    spicelib::SCARDI(0, save.VSET.as_slice_mut(), ctx)?;

    //
    // VALUE is the logical value indicating a gap.
    //
    save.VALUE = false;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOLS;
        let m3__: i32 = 1;
        save.COL = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NROWS;
                let m3__: i32 = 1;
                save.ROW = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.I = (((save.COL - 1) * save.NROWS) + save.ROW);

                    save.GRID[save.I] = (save.COL != 2);

                    save.ROW += m3__;
                }
            }

            save.COL += m3__;
        }
    }

    //
    // Find the gaps in the grid.
    //
    support::FNDCMP(
        save.NROWS,
        save.NCOLS,
        save.VALUE,
        MAXGRD,
        save.GRID.as_slice_mut(),
        save.VSET.as_slice_mut(),
        save.MRKSET.as_slice_mut(),
        save.TMPSET.as_slice_mut(),
        &mut save.NCOMP,
        save.MINPXX.as_slice_mut(),
        save.MAXPXX.as_slice_mut(),
        save.MINPXY.as_slice_mut(),
        save.MAXPXY.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NCOMP", save.NCOMP, b"=", 1, 0, OK, ctx)?;

    //
    // Check bounds of the single gap.
    //
    testutil::CHCKSI(b"MINPXX", *save.MINPXX.first(), b"=", 2, 0, OK, ctx)?;
    testutil::CHCKSI(b"MAXPXX", *save.MAXPXX.first(), b"=", 2, 0, OK, ctx)?;

    testutil::CHCKSI(b"MINPXY", *save.MINPXY.first(), b"=", 1, 0, OK, ctx)?;
    testutil::CHCKSI(b"MAXPXY", *save.MAXPXY.first(), b"=", 3, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"3x3 grid, middle row is a gap.");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::SCARDI(0, save.MRKSET.as_slice_mut(), ctx)?;
    spicelib::SCARDI(0, save.TMPSET.as_slice_mut(), ctx)?;
    spicelib::SCARDI(0, save.VSET.as_slice_mut(), ctx)?;

    //
    // VALUE is the logical value indicating a gap.
    //
    save.VALUE = false;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOLS;
        let m3__: i32 = 1;
        save.COL = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NROWS;
                let m3__: i32 = 1;
                save.ROW = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.I = (((save.COL - 1) * save.NROWS) + save.ROW);

                    save.GRID[save.I] = (save.ROW != 2);

                    save.ROW += m3__;
                }
            }

            save.COL += m3__;
        }
    }

    //
    // Find the gaps in the grid.
    //
    support::FNDCMP(
        save.NROWS,
        save.NCOLS,
        save.VALUE,
        MAXGRD,
        save.GRID.as_slice_mut(),
        save.VSET.as_slice_mut(),
        save.MRKSET.as_slice_mut(),
        save.TMPSET.as_slice_mut(),
        &mut save.NCOMP,
        save.MINPXX.as_slice_mut(),
        save.MAXPXX.as_slice_mut(),
        save.MINPXY.as_slice_mut(),
        save.MAXPXY.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NCOMP", save.NCOMP, b"=", 1, 0, OK, ctx)?;

    //
    // Check bounds of the single gap.
    //
    testutil::CHCKSI(b"MINPXX", *save.MINPXX.first(), b"=", 1, 0, OK, ctx)?;
    testutil::CHCKSI(b"MAXPXX", *save.MAXPXX.first(), b"=", 3, 0, OK, ctx)?;

    testutil::CHCKSI(b"MINPXY", *save.MINPXY.first(), b"=", 2, 0, OK, ctx)?;
    testutil::CHCKSI(b"MAXPXY", *save.MAXPXY.first(), b"=", 2, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"3x3 grid, all corners are gaps");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::SCARDI(0, save.MRKSET.as_slice_mut(), ctx)?;
    spicelib::SCARDI(0, save.TMPSET.as_slice_mut(), ctx)?;
    spicelib::SCARDI(0, save.VSET.as_slice_mut(), ctx)?;

    //
    // VALUE is the logical value indicating a gap.
    //
    save.VALUE = false;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOLS;
        let m3__: i32 = 1;
        save.COL = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NROWS;
                let m3__: i32 = 1;
                save.ROW = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.I = (((save.COL - 1) * save.NROWS) + save.ROW);

                    if (((save.ROW == 1) || (save.ROW == 3))
                        && ((save.COL == 1) || (save.COL == 3)))
                    {
                        save.GRID[save.I] = false;
                    } else {
                        save.GRID[save.I] = true;
                    }

                    save.ROW += m3__;
                }
            }

            save.COL += m3__;
        }
    }

    //
    // Find the gaps in the grid.
    //
    support::FNDCMP(
        save.NROWS,
        save.NCOLS,
        save.VALUE,
        MAXGRD,
        save.GRID.as_slice_mut(),
        save.VSET.as_slice_mut(),
        save.MRKSET.as_slice_mut(),
        save.TMPSET.as_slice_mut(),
        &mut save.NCOMP,
        save.MINPXX.as_slice_mut(),
        save.MAXPXX.as_slice_mut(),
        save.MINPXY.as_slice_mut(),
        save.MAXPXY.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NCOMP", save.NCOMP, b"=", 4, 0, OK, ctx)?;

    //
    // Check bounds of all four gaps.
    //
    save.XBDS[1] = 1;
    save.XBDS[2] = 1;
    save.XBDS[3] = 3;
    save.XBDS[4] = 3;

    testutil::CHCKAI(
        b"MINPXX",
        save.MINPXX.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.NCOMP,
        OK,
        ctx,
    )?;

    save.XBDS[1] = 1;
    save.XBDS[2] = 1;
    save.XBDS[3] = 3;
    save.XBDS[4] = 3;

    testutil::CHCKAI(
        b"MAXPXX",
        save.MAXPXX.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.NCOMP,
        OK,
        ctx,
    )?;

    save.XBDS[1] = 1;
    save.XBDS[2] = 3;
    save.XBDS[3] = 1;
    save.XBDS[4] = 3;

    testutil::CHCKAI(
        b"MINPXY",
        save.MINPXY.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.NCOMP,
        OK,
        ctx,
    )?;

    save.XBDS[1] = 1;
    save.XBDS[2] = 3;
    save.XBDS[3] = 1;
    save.XBDS[4] = 3;

    testutil::CHCKAI(
        b"MAXPXY",
        save.MAXPXY.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.NCOMP,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"3x3 grid, all corners are present; central cross is a gap.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::SCARDI(0, save.MRKSET.as_slice_mut(), ctx)?;
    spicelib::SCARDI(0, save.TMPSET.as_slice_mut(), ctx)?;
    spicelib::SCARDI(0, save.VSET.as_slice_mut(), ctx)?;

    //
    // VALUE is the logical value indicating a gap.
    //
    save.VALUE = false;
    save.NGAP = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOLS;
        let m3__: i32 = 1;
        save.COL = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NROWS;
                let m3__: i32 = 1;
                save.ROW = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.I = (((save.COL - 1) * save.NROWS) + save.ROW);

                    if (((save.ROW == 1) || (save.ROW == 3))
                        && ((save.COL == 1) || (save.COL == 3)))
                    {
                        save.GRID[save.I] = true;
                    } else {
                        save.GRID[save.I] = false;
                        save.NGAP = (save.NGAP + 1);
                    }

                    save.ROW += m3__;
                }
            }

            save.COL += m3__;
        }
    }

    //
    // Get a copy of the input grid, since GRID is an in-out argument.
    //
    save.N = (save.NCOLS * save.NROWS);

    spicelib::MOVEL(save.GRID.as_slice(), save.N, save.GRID1.as_slice_mut());

    //
    // Find the gaps in the grid.
    //
    support::FNDCMP(
        save.NROWS,
        save.NCOLS,
        save.VALUE,
        MAXGRD,
        save.GRID.as_slice_mut(),
        save.VSET.as_slice_mut(),
        save.MRKSET.as_slice_mut(),
        save.TMPSET.as_slice_mut(),
        &mut save.NCOMP,
        save.MINPXX.as_slice_mut(),
        save.MAXPXX.as_slice_mut(),
        save.MINPXY.as_slice_mut(),
        save.MAXPXY.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NCOMP", save.NCOMP, b"=", 3, 0, OK, ctx)?;

    //
    // Check bounds of all three gaps.
    //
    save.XBDS[1] = 1;
    save.XBDS[2] = 2;
    save.XBDS[3] = 2;

    testutil::CHCKAI(
        b"MINPXX",
        save.MINPXX.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.NCOMP,
        OK,
        ctx,
    )?;

    save.XBDS[1] = 3;
    save.XBDS[2] = 2;
    save.XBDS[3] = 2;

    testutil::CHCKAI(
        b"MAXPXX",
        save.MAXPXX.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.NCOMP,
        OK,
        ctx,
    )?;

    save.XBDS[1] = 2;
    save.XBDS[2] = 1;
    save.XBDS[3] = 3;

    testutil::CHCKAI(
        b"MINPXY",
        save.MINPXY.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.NCOMP,
        OK,
        ctx,
    )?;

    save.XBDS[1] = 2;
    save.XBDS[2] = 1;
    save.XBDS[3] = 3;

    testutil::CHCKAI(
        b"MAXPXY",
        save.MAXPXY.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.NCOMP,
        OK,
        ctx,
    )?;

    //
    // Find the gaps in the inverse grid. These should
    // combine to form the coverage of the original grid.
    //
    save.VALUE = !save.VALUE;

    support::FNDCMP(
        save.NROWS,
        save.NCOLS,
        save.VALUE,
        MAXGRD,
        save.GRID1.as_slice_mut(),
        save.VSET.as_slice_mut(),
        save.MRKSET.as_slice_mut(),
        save.TMPSET.as_slice_mut(),
        &mut save.NCOMP,
        save.MINPXX.as_slice_mut(),
        save.MAXPXX.as_slice_mut(),
        save.MINPXY.as_slice_mut(),
        save.MAXPXY.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Count the pixels in the gaps.
    //
    save.J = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOMP;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.J = (save.J
                + (((save.MAXPXX[save.I] - save.MINPXX[save.I]) + 1)
                    * ((save.MAXPXY[save.I] - save.MINPXY[save.I]) + 1)));

            save.I += m3__;
        }
    }

    //
    // Check the coverage count versus that implied by the
    // gap count.
    //
    testutil::CHCKSI(
        b"J (coverage)",
        save.J,
        b"=",
        (save.N - save.NGAP),
        0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"3x3 grid, first column and first row are gaps",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::SCARDI(0, save.MRKSET.as_slice_mut(), ctx)?;
    spicelib::SCARDI(0, save.TMPSET.as_slice_mut(), ctx)?;
    spicelib::SCARDI(0, save.VSET.as_slice_mut(), ctx)?;

    //
    // VALUE is the logical value indicating a gap.
    //
    save.VALUE = false;
    save.NGAP = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOLS;
        let m3__: i32 = 1;
        save.COL = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NROWS;
                let m3__: i32 = 1;
                save.ROW = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.I = (((save.COL - 1) * save.NROWS) + save.ROW);

                    if ((save.ROW == 1) || (save.COL == 1)) {
                        save.GRID[save.I] = false;
                        save.NGAP = (save.NGAP + 1);
                    } else {
                        save.GRID[save.I] = true;
                    }

                    save.ROW += m3__;
                }
            }

            save.COL += m3__;
        }
    }

    //
    // Get a copy of the input grid, since GRID is an in-out argument.
    //
    save.N = (save.NCOLS * save.NROWS);

    spicelib::MOVEL(save.GRID.as_slice(), save.N, save.GRID1.as_slice_mut());

    //
    // Find the gaps in the grid.
    //
    support::FNDCMP(
        save.NROWS,
        save.NCOLS,
        save.VALUE,
        MAXGRD,
        save.GRID.as_slice_mut(),
        save.VSET.as_slice_mut(),
        save.MRKSET.as_slice_mut(),
        save.TMPSET.as_slice_mut(),
        &mut save.NCOMP,
        save.MINPXX.as_slice_mut(),
        save.MAXPXX.as_slice_mut(),
        save.MINPXY.as_slice_mut(),
        save.MAXPXY.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NCOMP", save.NCOMP, b"=", 2, 0, OK, ctx)?;

    //
    // Check bounds of both gaps.
    //
    save.XBDS[1] = 1;
    save.XBDS[2] = 2;

    testutil::CHCKAI(
        b"MINPXX",
        save.MINPXX.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.NCOMP,
        OK,
        ctx,
    )?;

    save.XBDS[1] = 1;
    save.XBDS[2] = 3;

    testutil::CHCKAI(
        b"MAXPXX",
        save.MAXPXX.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.NCOMP,
        OK,
        ctx,
    )?;

    save.XBDS[1] = 1;
    save.XBDS[2] = 1;

    testutil::CHCKAI(
        b"MINPXY",
        save.MINPXY.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.NCOMP,
        OK,
        ctx,
    )?;

    save.XBDS[1] = 3;
    save.XBDS[2] = 1;

    testutil::CHCKAI(
        b"MAXPXY",
        save.MAXPXY.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.NCOMP,
        OK,
        ctx,
    )?;

    //
    // Find the gaps in the inverse grid. These should
    // combine to form the coverage of the original grid.
    //
    save.VALUE = !save.VALUE;

    support::FNDCMP(
        save.NROWS,
        save.NCOLS,
        save.VALUE,
        MAXGRD,
        save.GRID1.as_slice_mut(),
        save.VSET.as_slice_mut(),
        save.MRKSET.as_slice_mut(),
        save.TMPSET.as_slice_mut(),
        &mut save.NCOMP,
        save.MINPXX.as_slice_mut(),
        save.MAXPXX.as_slice_mut(),
        save.MINPXY.as_slice_mut(),
        save.MAXPXY.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Count the pixels in the gaps.
    //
    save.J = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOMP;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.J = (save.J
                + (((save.MAXPXX[save.I] - save.MINPXX[save.I]) + 1)
                    * ((save.MAXPXY[save.I] - save.MINPXY[save.I]) + 1)));

            save.I += m3__;
        }
    }

    //
    // Check the coverage count versus that implied by the
    // gap count.
    //
    testutil::CHCKSI(
        b"J (coverage)",
        save.J,
        b"=",
        (save.N - save.NGAP),
        0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"3x3 grid, last column and first row are gaps",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::SCARDI(0, save.MRKSET.as_slice_mut(), ctx)?;
    spicelib::SCARDI(0, save.TMPSET.as_slice_mut(), ctx)?;
    spicelib::SCARDI(0, save.VSET.as_slice_mut(), ctx)?;

    //
    // VALUE is the logical value indicating a gap.
    //
    save.VALUE = false;

    save.NGAP = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOLS;
        let m3__: i32 = 1;
        save.COL = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NROWS;
                let m3__: i32 = 1;
                save.ROW = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.I = (((save.COL - 1) * save.NROWS) + save.ROW);

                    if ((save.ROW == 1) || (save.COL == 3)) {
                        save.GRID[save.I] = false;
                        save.NGAP = (save.NGAP + 1);
                    } else {
                        save.GRID[save.I] = true;
                    }

                    save.ROW += m3__;
                }
            }

            save.COL += m3__;
        }
    }

    //
    // Get a copy of the input grid, since GRID is an in-out argument.
    //
    save.N = (save.NCOLS * save.NROWS);

    spicelib::MOVEL(save.GRID.as_slice(), save.N, save.GRID1.as_slice_mut());

    //
    // Find the gaps in the grid.
    //
    support::FNDCMP(
        save.NROWS,
        save.NCOLS,
        save.VALUE,
        MAXGRD,
        save.GRID.as_slice_mut(),
        save.VSET.as_slice_mut(),
        save.MRKSET.as_slice_mut(),
        save.TMPSET.as_slice_mut(),
        &mut save.NCOMP,
        save.MINPXX.as_slice_mut(),
        save.MAXPXX.as_slice_mut(),
        save.MINPXY.as_slice_mut(),
        save.MAXPXY.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NCOMP", save.NCOMP, b"=", 2, 0, OK, ctx)?;

    //
    // Check bounds of both gaps.
    //
    save.XBDS[1] = 1;
    save.XBDS[2] = 3;

    testutil::CHCKAI(
        b"MINPXX",
        save.MINPXX.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.NCOMP,
        OK,
        ctx,
    )?;

    save.XBDS[1] = 3;
    save.XBDS[2] = 3;

    testutil::CHCKAI(
        b"MAXPXX",
        save.MAXPXX.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.NCOMP,
        OK,
        ctx,
    )?;

    save.XBDS[1] = 1;
    save.XBDS[2] = 2;

    testutil::CHCKAI(
        b"MINPXY",
        save.MINPXY.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.NCOMP,
        OK,
        ctx,
    )?;

    save.XBDS[1] = 1;
    save.XBDS[2] = 3;

    testutil::CHCKAI(
        b"MAXPXY",
        save.MAXPXY.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.NCOMP,
        OK,
        ctx,
    )?;

    //
    // Find the gaps in the inverse grid. These should
    // combine to form the coverage of the original grid.
    //
    save.VALUE = !save.VALUE;

    support::FNDCMP(
        save.NROWS,
        save.NCOLS,
        save.VALUE,
        MAXGRD,
        save.GRID1.as_slice_mut(),
        save.VSET.as_slice_mut(),
        save.MRKSET.as_slice_mut(),
        save.TMPSET.as_slice_mut(),
        &mut save.NCOMP,
        save.MINPXX.as_slice_mut(),
        save.MAXPXX.as_slice_mut(),
        save.MINPXY.as_slice_mut(),
        save.MAXPXY.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Count the pixels in the gaps.
    //
    save.J = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOMP;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.J = (save.J
                + (((save.MAXPXX[save.I] - save.MINPXX[save.I]) + 1)
                    * ((save.MAXPXY[save.I] - save.MINPXY[save.I]) + 1)));

            save.I += m3__;
        }
    }

    //
    // Check the coverage count versus that implied by the
    // gap count.
    //
    testutil::CHCKSI(
        b"J (coverage)",
        save.J,
        b"=",
        (save.N - save.NGAP),
        0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"3x3 grid, first column and last row are gaps",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::SCARDI(0, save.MRKSET.as_slice_mut(), ctx)?;
    spicelib::SCARDI(0, save.TMPSET.as_slice_mut(), ctx)?;
    spicelib::SCARDI(0, save.VSET.as_slice_mut(), ctx)?;

    //
    // VALUE is the logical value indicating a gap.
    //
    save.VALUE = false;

    //
    // We'll count the gaps this time.
    //
    save.NGAP = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOLS;
        let m3__: i32 = 1;
        save.COL = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NROWS;
                let m3__: i32 = 1;
                save.ROW = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.I = (((save.COL - 1) * save.NROWS) + save.ROW);

                    if ((save.ROW == 3) || (save.COL == 1)) {
                        save.GRID[save.I] = false;
                        save.NGAP = (save.NGAP + 1);
                    } else {
                        save.GRID[save.I] = true;
                    }

                    save.ROW += m3__;
                }
            }

            save.COL += m3__;
        }
    }

    //
    // Get a copy of the input grid, since GRID is an in-out argument.
    //
    save.N = (save.NCOLS * save.NROWS);

    spicelib::MOVEL(save.GRID.as_slice(), save.N, save.GRID1.as_slice_mut());

    //
    // Find the gaps in the grid.
    //
    support::FNDCMP(
        save.NROWS,
        save.NCOLS,
        save.VALUE,
        MAXGRD,
        save.GRID.as_slice_mut(),
        save.VSET.as_slice_mut(),
        save.MRKSET.as_slice_mut(),
        save.TMPSET.as_slice_mut(),
        &mut save.NCOMP,
        save.MINPXX.as_slice_mut(),
        save.MAXPXX.as_slice_mut(),
        save.MINPXY.as_slice_mut(),
        save.MAXPXY.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NCOMP", save.NCOMP, b"=", 2, 0, OK, ctx)?;

    //
    // Check bounds of both gaps.
    //
    save.XBDS[1] = 1;
    save.XBDS[2] = 2;

    testutil::CHCKAI(
        b"MINPXX",
        save.MINPXX.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.NCOMP,
        OK,
        ctx,
    )?;

    save.XBDS[1] = 1;
    save.XBDS[2] = 3;

    testutil::CHCKAI(
        b"MAXPXX",
        save.MAXPXX.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.NCOMP,
        OK,
        ctx,
    )?;

    save.XBDS[1] = 1;
    save.XBDS[2] = 3;

    testutil::CHCKAI(
        b"MINPXY",
        save.MINPXY.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.NCOMP,
        OK,
        ctx,
    )?;

    save.XBDS[1] = 3;
    save.XBDS[2] = 3;

    testutil::CHCKAI(
        b"MAXPXY",
        save.MAXPXY.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.NCOMP,
        OK,
        ctx,
    )?;

    //
    // Find the gaps in the inverse grid. These should
    // combine to form the coverage of the original grid.
    //
    save.VALUE = !save.VALUE;

    support::FNDCMP(
        save.NROWS,
        save.NCOLS,
        save.VALUE,
        MAXGRD,
        save.GRID1.as_slice_mut(),
        save.VSET.as_slice_mut(),
        save.MRKSET.as_slice_mut(),
        save.TMPSET.as_slice_mut(),
        &mut save.NCOMP,
        save.MINPXX.as_slice_mut(),
        save.MAXPXX.as_slice_mut(),
        save.MINPXY.as_slice_mut(),
        save.MAXPXY.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Count the pixels in the gaps.
    //
    save.J = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOMP;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.J = (save.J
                + (((save.MAXPXX[save.I] - save.MINPXX[save.I]) + 1)
                    * ((save.MAXPXY[save.I] - save.MINPXY[save.I]) + 1)));

            save.I += m3__;
        }
    }

    //
    // Check the coverage count versus that implied by the
    // gap count.
    //
    testutil::CHCKSI(
        b"J (coverage)",
        save.J,
        b"=",
        (save.N - save.NGAP),
        0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"3x3 grid, last column and last row are gaps",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::SCARDI(0, save.MRKSET.as_slice_mut(), ctx)?;
    spicelib::SCARDI(0, save.TMPSET.as_slice_mut(), ctx)?;
    spicelib::SCARDI(0, save.VSET.as_slice_mut(), ctx)?;

    //
    // VALUE is the logical value indicating a gap.
    //
    save.VALUE = false;

    //
    // We'll count the gaps this time.
    //
    save.NGAP = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOLS;
        let m3__: i32 = 1;
        save.COL = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NROWS;
                let m3__: i32 = 1;
                save.ROW = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.I = (((save.COL - 1) * save.NROWS) + save.ROW);

                    if ((save.ROW == 3) || (save.COL == 3)) {
                        save.GRID[save.I] = false;
                        save.NGAP = (save.NGAP + 1);
                    } else {
                        save.GRID[save.I] = true;
                    }

                    save.ROW += m3__;
                }
            }

            save.COL += m3__;
        }
    }

    //
    // Get a copy of the input grid, since GRID is an in-out argument.
    //
    save.N = (save.NCOLS * save.NROWS);

    spicelib::MOVEL(save.GRID.as_slice(), save.N, save.GRID1.as_slice_mut());

    //
    // Find the gaps in the grid.
    //
    support::FNDCMP(
        save.NROWS,
        save.NCOLS,
        save.VALUE,
        MAXGRD,
        save.GRID.as_slice_mut(),
        save.VSET.as_slice_mut(),
        save.MRKSET.as_slice_mut(),
        save.TMPSET.as_slice_mut(),
        &mut save.NCOMP,
        save.MINPXX.as_slice_mut(),
        save.MAXPXX.as_slice_mut(),
        save.MINPXY.as_slice_mut(),
        save.MAXPXY.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NCOMP", save.NCOMP, b"=", 2, 0, OK, ctx)?;

    //
    // Check bounds of both gaps.
    //
    save.XBDS[1] = 1;
    save.XBDS[2] = 3;

    testutil::CHCKAI(
        b"MINPXX",
        save.MINPXX.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.NCOMP,
        OK,
        ctx,
    )?;

    save.XBDS[1] = 3;
    save.XBDS[2] = 3;

    testutil::CHCKAI(
        b"MAXPXX",
        save.MAXPXX.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.NCOMP,
        OK,
        ctx,
    )?;

    save.XBDS[1] = 3;
    save.XBDS[2] = 1;

    testutil::CHCKAI(
        b"MINPXY",
        save.MINPXY.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.NCOMP,
        OK,
        ctx,
    )?;

    save.XBDS[1] = 3;
    save.XBDS[2] = 2;

    testutil::CHCKAI(
        b"MAXPXY",
        save.MAXPXY.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.NCOMP,
        OK,
        ctx,
    )?;

    //
    // Find the gaps in the inverse grid. These should
    // combine to form the coverage of the original grid.
    //
    save.VALUE = !save.VALUE;

    support::FNDCMP(
        save.NROWS,
        save.NCOLS,
        save.VALUE,
        MAXGRD,
        save.GRID1.as_slice_mut(),
        save.VSET.as_slice_mut(),
        save.MRKSET.as_slice_mut(),
        save.TMPSET.as_slice_mut(),
        &mut save.NCOMP,
        save.MINPXX.as_slice_mut(),
        save.MAXPXX.as_slice_mut(),
        save.MINPXY.as_slice_mut(),
        save.MAXPXY.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Count the pixels in the gaps.
    //
    save.J = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOMP;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.J = (save.J
                + (((save.MAXPXX[save.I] - save.MINPXX[save.I]) + 1)
                    * ((save.MAXPXY[save.I] - save.MINPXY[save.I]) + 1)));

            save.I += m3__;
        }
    }

    //
    // Check the coverage count versus that implied by the
    // gap count.
    //
    testutil::CHCKSI(
        b"J (coverage)",
        save.J,
        b"=",
        (save.N - save.NGAP),
        0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"3x3 grid, last column and last row are gaps",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::SCARDI(0, save.MRKSET.as_slice_mut(), ctx)?;
    spicelib::SCARDI(0, save.TMPSET.as_slice_mut(), ctx)?;
    spicelib::SCARDI(0, save.VSET.as_slice_mut(), ctx)?;

    //
    // VALUE is the logical value indicating a gap.
    //
    save.VALUE = false;

    //
    // We'll count the gaps this time.
    //
    save.NGAP = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOLS;
        let m3__: i32 = 1;
        save.COL = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NROWS;
                let m3__: i32 = 1;
                save.ROW = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.I = (((save.COL - 1) * save.NROWS) + save.ROW);

                    if ((save.ROW == 3) || (save.COL == 3)) {
                        save.GRID[save.I] = false;
                        save.NGAP = (save.NGAP + 1);
                    } else {
                        save.GRID[save.I] = true;
                    }

                    save.ROW += m3__;
                }
            }

            save.COL += m3__;
        }
    }

    //
    // Get a copy of the input grid, since GRID is an in-out argument.
    //
    save.N = (save.NCOLS * save.NROWS);

    spicelib::MOVEL(save.GRID.as_slice(), save.N, save.GRID1.as_slice_mut());

    //
    // Find the gaps in the grid.
    //
    support::FNDCMP(
        save.NROWS,
        save.NCOLS,
        save.VALUE,
        MAXGRD,
        save.GRID.as_slice_mut(),
        save.VSET.as_slice_mut(),
        save.MRKSET.as_slice_mut(),
        save.TMPSET.as_slice_mut(),
        &mut save.NCOMP,
        save.MINPXX.as_slice_mut(),
        save.MAXPXX.as_slice_mut(),
        save.MINPXY.as_slice_mut(),
        save.MAXPXY.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NCOMP", save.NCOMP, b"=", 2, 0, OK, ctx)?;

    //
    // Check bounds of both gaps.
    //
    save.XBDS[1] = 1;
    save.XBDS[2] = 3;

    testutil::CHCKAI(
        b"MINPXX",
        save.MINPXX.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.NCOMP,
        OK,
        ctx,
    )?;

    save.XBDS[1] = 3;
    save.XBDS[2] = 3;

    testutil::CHCKAI(
        b"MAXPXX",
        save.MAXPXX.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.NCOMP,
        OK,
        ctx,
    )?;

    save.XBDS[1] = 3;
    save.XBDS[2] = 1;

    testutil::CHCKAI(
        b"MINPXY",
        save.MINPXY.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.NCOMP,
        OK,
        ctx,
    )?;

    save.XBDS[1] = 3;
    save.XBDS[2] = 2;

    testutil::CHCKAI(
        b"MAXPXY",
        save.MAXPXY.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.NCOMP,
        OK,
        ctx,
    )?;

    //
    // Find the gaps in the inverse grid. These should
    // combine to form the coverage of the original grid.
    //
    save.VALUE = !save.VALUE;

    support::FNDCMP(
        save.NROWS,
        save.NCOLS,
        save.VALUE,
        MAXGRD,
        save.GRID1.as_slice_mut(),
        save.VSET.as_slice_mut(),
        save.MRKSET.as_slice_mut(),
        save.TMPSET.as_slice_mut(),
        &mut save.NCOMP,
        save.MINPXX.as_slice_mut(),
        save.MAXPXX.as_slice_mut(),
        save.MINPXY.as_slice_mut(),
        save.MAXPXY.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Count the pixels in the gaps.
    //
    save.J = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOMP;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.J = (save.J
                + (((save.MAXPXX[save.I] - save.MINPXX[save.I]) + 1)
                    * ((save.MAXPXY[save.I] - save.MINPXY[save.I]) + 1)));

            save.I += m3__;
        }
    }

    //
    // Check the coverage count versus that implied by the
    // gap count.
    //
    testutil::CHCKSI(
        b"J (coverage)",
        save.J,
        b"=",
        (save.N - save.NGAP),
        0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"3x3 grid, last column and last row are gaps",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::SCARDI(0, save.MRKSET.as_slice_mut(), ctx)?;
    spicelib::SCARDI(0, save.TMPSET.as_slice_mut(), ctx)?;
    spicelib::SCARDI(0, save.VSET.as_slice_mut(), ctx)?;

    //
    // VALUE is the logical value indicating a gap.
    //
    save.VALUE = false;

    //
    // We'll count the gaps this time.
    //
    save.NGAP = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOLS;
        let m3__: i32 = 1;
        save.COL = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NROWS;
                let m3__: i32 = 1;
                save.ROW = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.I = (((save.COL - 1) * save.NROWS) + save.ROW);

                    if ((save.ROW == 3) || (save.COL == 3)) {
                        save.GRID[save.I] = false;
                        save.NGAP = (save.NGAP + 1);
                    } else {
                        save.GRID[save.I] = true;
                    }

                    save.ROW += m3__;
                }
            }

            save.COL += m3__;
        }
    }

    //
    // Get a copy of the input grid, since GRID is an in-out argument.
    //
    save.N = (save.NCOLS * save.NROWS);

    spicelib::MOVEL(save.GRID.as_slice(), save.N, save.GRID1.as_slice_mut());

    //
    // Find the gaps in the grid.
    //
    support::FNDCMP(
        save.NROWS,
        save.NCOLS,
        save.VALUE,
        MAXGRD,
        save.GRID.as_slice_mut(),
        save.VSET.as_slice_mut(),
        save.MRKSET.as_slice_mut(),
        save.TMPSET.as_slice_mut(),
        &mut save.NCOMP,
        save.MINPXX.as_slice_mut(),
        save.MAXPXX.as_slice_mut(),
        save.MINPXY.as_slice_mut(),
        save.MAXPXY.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NCOMP", save.NCOMP, b"=", 2, 0, OK, ctx)?;

    //
    // Check bounds of both gaps.
    //
    save.XBDS[1] = 1;
    save.XBDS[2] = 3;

    testutil::CHCKAI(
        b"MINPXX",
        save.MINPXX.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.NCOMP,
        OK,
        ctx,
    )?;

    save.XBDS[1] = 3;
    save.XBDS[2] = 3;

    testutil::CHCKAI(
        b"MAXPXX",
        save.MAXPXX.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.NCOMP,
        OK,
        ctx,
    )?;

    save.XBDS[1] = 3;
    save.XBDS[2] = 1;

    testutil::CHCKAI(
        b"MINPXY",
        save.MINPXY.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.NCOMP,
        OK,
        ctx,
    )?;

    save.XBDS[1] = 3;
    save.XBDS[2] = 2;

    testutil::CHCKAI(
        b"MAXPXY",
        save.MAXPXY.as_slice(),
        b"=",
        save.XBDS.as_slice(),
        save.NCOMP,
        OK,
        ctx,
    )?;

    //
    // Find the gaps in the inverse grid. These should
    // combine to form the coverage of the original grid.
    //
    save.VALUE = !save.VALUE;

    support::FNDCMP(
        save.NROWS,
        save.NCOLS,
        save.VALUE,
        MAXGRD,
        save.GRID1.as_slice_mut(),
        save.VSET.as_slice_mut(),
        save.MRKSET.as_slice_mut(),
        save.TMPSET.as_slice_mut(),
        &mut save.NCOMP,
        save.MINPXX.as_slice_mut(),
        save.MAXPXX.as_slice_mut(),
        save.MINPXY.as_slice_mut(),
        save.MAXPXY.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Count the pixels in the gaps.
    //
    save.J = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOMP;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.J = (save.J
                + (((save.MAXPXX[save.I] - save.MINPXX[save.I]) + 1)
                    * ((save.MAXPXY[save.I] - save.MINPXY[save.I]) + 1)));

            save.I += m3__;
        }
    }

    //
    // Check the coverage count versus that implied by the
    // gap count.
    //
    testutil::CHCKSI(
        b"J (coverage)",
        save.J,
        b"=",
        (save.N - save.NGAP),
        0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"101x101 grid. Every other pixel is a gap.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // We must have an odd row count to create the checkerboard
    // pattern we desire.
    //
    save.NROWS = 101;
    save.NCOLS = 101;

    spicelib::SCARDI(0, save.MRKSET.as_slice_mut(), ctx)?;
    spicelib::SCARDI(0, save.TMPSET.as_slice_mut(), ctx)?;
    spicelib::SCARDI(0, save.VSET.as_slice_mut(), ctx)?;

    //
    // VALUE is the logical value indicating a gap.
    //
    save.VALUE = false;

    save.NGAP = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOLS;
        let m3__: i32 = 1;
        save.COL = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NROWS;
                let m3__: i32 = 1;
                save.ROW = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.I = (((save.COL - 1) * save.NROWS) + save.ROW);

                    if spicelib::EVEN(save.I) {
                        save.GRID[save.I] = false;
                        save.NGAP = (save.NGAP + 1);
                    } else {
                        save.GRID[save.I] = true;
                    }

                    save.ROW += m3__;
                }
            }

            save.COL += m3__;
        }
    }

    //
    // Get a copy of the input grid, since GRID is an in-out argument.
    //
    save.N = (save.NCOLS * save.NROWS);

    spicelib::MOVEL(save.GRID.as_slice(), save.N, save.GRID1.as_slice_mut());

    //
    // Find the gaps in the grid.
    //
    support::FNDCMP(
        save.NROWS,
        save.NCOLS,
        save.VALUE,
        MAXGRD,
        save.GRID.as_slice_mut(),
        save.VSET.as_slice_mut(),
        save.MRKSET.as_slice_mut(),
        save.TMPSET.as_slice_mut(),
        &mut save.NCOMP,
        save.MINPXX.as_slice_mut(),
        save.MAXPXX.as_slice_mut(),
        save.MINPXY.as_slice_mut(),
        save.MAXPXY.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Test the results: create a new grid with no gaps; fill in
    // each output component with gap values. The result should
    // match the copy of the input grid.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.GRID2[save.I] = true;
            save.I += m3__;
        }
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOMP;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = save.MINPXX[save.I];
                let m2__: i32 = save.MAXPXX[save.I];
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    for K in save.MINPXY[save.I]..=save.MAXPXY[save.I] {
                        save.W = (((save.J - 1) * save.NROWS) + K);

                        save.GRID2[save.W] = false;
                    }

                    save.J += m3__;
                }
            }

            save.I += m3__;
        }
    }

    //
    // Did we recover the grid?
    //
    // Use CHCKSL and a loop.
    //
    *OK = true;
    save.I = 1;

    while ((save.I <= save.N) && *OK) {
        if (save.GRID2[save.I] != save.GRID1[save.I]) {
            fstr::assign(&mut save.LABEL, b"GRID2 element *");
            spicelib::REPMI(&save.LABEL.to_vec(), b"*", save.I, &mut save.LABEL, ctx);

            testutil::CHCKSL(&save.LABEL, save.GRID2[save.I], save.GRID1[save.I], OK, ctx)?;
        }

        save.I = (save.I + 1);
    }

    //
    // Find the gaps in the inverse grid. These should
    // combine to form the coverage of the original grid.
    //
    save.VALUE = !save.VALUE;

    support::FNDCMP(
        save.NROWS,
        save.NCOLS,
        save.VALUE,
        MAXGRD,
        save.GRID1.as_slice_mut(),
        save.VSET.as_slice_mut(),
        save.MRKSET.as_slice_mut(),
        save.TMPSET.as_slice_mut(),
        &mut save.NCOMP,
        save.MINPXX.as_slice_mut(),
        save.MAXPXX.as_slice_mut(),
        save.MINPXY.as_slice_mut(),
        save.MAXPXY.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Count the pixels in the gaps.
    //
    save.J = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOMP;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.J = (save.J
                + (((save.MAXPXX[save.I] - save.MINPXX[save.I]) + 1)
                    * ((save.MAXPXY[save.I] - save.MINPXY[save.I]) + 1)));

            save.I += m3__;
        }
    }

    //
    // Check the coverage count versus that implied by the
    // gap count.
    //
    testutil::CHCKSI(
        b"J (coverage)",
        save.J,
        b"=",
        (save.N - save.NGAP),
        0,
        OK,
        ctx,
    )?;

    //***********************************************************************
    //
    //
    //
    //     Random cases
    //
    //
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"300x300 grid. Random pixels are selected as gap members.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    save.NROWS = 300;
    save.NCOLS = 300;

    save.N = (save.NCOLS * save.NROWS);

    spicelib::SCARDI(0, save.MRKSET.as_slice_mut(), ctx)?;
    spicelib::SCARDI(0, save.TMPSET.as_slice_mut(), ctx)?;
    spicelib::SCARDI(0, save.VSET.as_slice_mut(), ctx)?;

    //
    // VALUE is the logical value indicating a gap.
    //
    save.VALUE = false;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.GRID[save.I] = true;
            save.I += m3__;
        }
    }

    //
    // Set J to the nominal number of gap pixels.
    //
    save.J = 10000;
    save.NGAP = 0;
    save.SEED = -1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.J;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.J = intrinsics::IDNINT(testutil::T_RANDD(
                1.0,
                (save.N as f64),
                &mut save.SEED,
                ctx,
            )?);

            //
            // We might have duplicate values of J. Count the current pixel
            // only if it's not already a gap pixel.
            //
            if save.GRID[save.J] {
                save.NGAP = (save.NGAP + 1);
            }

            save.GRID[save.J] = false;

            save.I += m3__;
        }
    }

    //
    // Get a copy of the input grid, since GRID is an in-out argument.
    //
    spicelib::MOVEL(save.GRID.as_slice(), save.N, save.GRID1.as_slice_mut());

    //
    // Find the gaps in the grid.
    //
    support::FNDCMP(
        save.NROWS,
        save.NCOLS,
        save.VALUE,
        MAXGRD,
        save.GRID.as_slice_mut(),
        save.VSET.as_slice_mut(),
        save.MRKSET.as_slice_mut(),
        save.TMPSET.as_slice_mut(),
        &mut save.NCOMP,
        save.MINPXX.as_slice_mut(),
        save.MAXPXX.as_slice_mut(),
        save.MINPXY.as_slice_mut(),
        save.MAXPXY.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Test the results: create a new grid with no gaps; fill in
    // each output component with gap values. The result should
    // match the copy of the input grid.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.GRID2[save.I] = true;
            save.I += m3__;
        }
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOMP;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = save.MINPXX[save.I];
                let m2__: i32 = save.MAXPXX[save.I];
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    for K in save.MINPXY[save.I]..=save.MAXPXY[save.I] {
                        save.W = (((save.J - 1) * save.NROWS) + K);

                        save.GRID2[save.W] = false;
                    }

                    save.J += m3__;
                }
            }

            save.I += m3__;
        }
    }

    //
    // Did we recover the grid?
    //
    // Use CHCKSL and a loop.
    //
    *OK = true;
    save.I = 1;

    while ((save.I <= save.N) && *OK) {
        if (save.GRID2[save.I] != save.GRID1[save.I]) {
            fstr::assign(&mut save.LABEL, b"GRID2 element *");
            spicelib::REPMI(&save.LABEL.to_vec(), b"*", save.I, &mut save.LABEL, ctx);

            testutil::CHCKSL(&save.LABEL, save.GRID2[save.I], save.GRID1[save.I], OK, ctx)?;
        }

        save.I = (save.I + 1);
    }

    //
    // Find the gaps in the inverse grid. These should
    // combine to form the coverage of the original grid.
    //
    save.VALUE = !save.VALUE;

    support::FNDCMP(
        save.NROWS,
        save.NCOLS,
        save.VALUE,
        MAXGRD,
        save.GRID1.as_slice_mut(),
        save.VSET.as_slice_mut(),
        save.MRKSET.as_slice_mut(),
        save.TMPSET.as_slice_mut(),
        &mut save.NCOMP,
        save.MINPXX.as_slice_mut(),
        save.MAXPXX.as_slice_mut(),
        save.MINPXY.as_slice_mut(),
        save.MAXPXY.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Count the pixels in the gaps.
    //
    save.J = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCOMP;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.J = (save.J
                + (((save.MAXPXX[save.I] - save.MINPXX[save.I]) + 1)
                    * ((save.MAXPXY[save.I] - save.MINPXY[save.I]) + 1)));

            save.I += m3__;
        }
    }

    //
    // Check the coverage count versus that implied by the
    // gap count.
    //
    testutil::CHCKSI(
        b"J (coverage)",
        save.J,
        b"=",
        (save.N - save.NGAP),
        0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     Following is a sequence of random cases using small grids.
    //     we'll use gap sizes of 1 : NGRID-1, where the locations
    //     of the gap pixels are selected randomly.
    //
    //
    save.NROWS = 18;
    save.NCOLS = 23;
    save.NGRID = (save.NROWS * save.NCOLS);

    //
    // The nominal gap size is the number of random indices we
    // select. Since some indices may overlap, the actual gap
    // size may be smaller.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = (save.NGRID - 1);
        let m3__: i32 = 1;
        save.N = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // --- Case: ------------------------------------------------------
            //
            fstr::assign(
                &mut save.TITLE,
                b"Random case: grid size = #; nominal gap count = #.",
            );
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.NGRID, &mut save.TITLE, ctx);
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.N, &mut save.TITLE, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::TCASE(&save.TITLE, ctx)?;
            //
            // The nominal gap size is N.
            //
            // Initialize the grid, then set the gap pixels.
            //
            // VALUE is the logical value indicating a gap.
            //
            save.VALUE = false;

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NGRID;
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.GRID[save.I] = true;
                    save.I += m3__;
                }
            }

            save.NGAP = 0;
            save.SEED = -1;

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.N;
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.J = intrinsics::IDNINT(testutil::T_RANDD(
                        2.0,
                        ((save.NGRID - 1) as f64),
                        &mut save.SEED,
                        ctx,
                    )?);

                    //
                    // We might have duplicate values of J. Count the current pixel
                    // only if it's not already a gap pixel.
                    //
                    if save.GRID[save.J] {
                        save.NGAP = (save.NGAP + 1);
                    }

                    save.GRID[save.J] = false;

                    save.I += m3__;
                }
            }

            // WRITE (*,*) 'N, NGAP = ', N, NGAP

            //
            // Get a copy of the input grid, since GRID is an in-out argument.
            //
            spicelib::MOVEL(save.GRID.as_slice(), save.NGRID, save.GRID1.as_slice_mut());

            //
            // Find the gaps in the grid.
            //
            support::FNDCMP(
                save.NROWS,
                save.NCOLS,
                save.VALUE,
                MAXGRD,
                save.GRID.as_slice_mut(),
                save.VSET.as_slice_mut(),
                save.MRKSET.as_slice_mut(),
                save.TMPSET.as_slice_mut(),
                &mut save.NCOMP,
                save.MINPXX.as_slice_mut(),
                save.MAXPXX.as_slice_mut(),
                save.MINPXY.as_slice_mut(),
                save.MAXPXY.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Test the results: create a new grid with no gaps; fill in
            // each output component with gap values. The result should
            // match the copy of the input grid.
            //
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NGRID;
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.GRID2[save.I] = true;
                    save.I += m3__;
                }
            }

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NCOMP;
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    {
                        let m1__: i32 = save.MINPXX[save.I];
                        let m2__: i32 = save.MAXPXX[save.I];
                        let m3__: i32 = 1;
                        save.J = m1__;
                        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                            for K in save.MINPXY[save.I]..=save.MAXPXY[save.I] {
                                save.W = (((save.J - 1) * save.NROWS) + K);

                                save.GRID2[save.W] = false;
                            }

                            save.J += m3__;
                        }
                    }

                    save.I += m3__;
                }
            }

            //
            // Did we recover the grid?
            //
            // Use CHCKSL and a loop.
            //
            *OK = true;
            save.I = 1;

            while ((save.I <= save.NGRID) && *OK) {
                if (save.GRID2[save.I] != save.GRID1[save.I]) {
                    fstr::assign(&mut save.LABEL, b"GRID2 element *");
                    spicelib::REPMI(&save.LABEL.to_vec(), b"*", save.I, &mut save.LABEL, ctx);

                    testutil::CHCKSL(&save.LABEL, save.GRID2[save.I], save.GRID1[save.I], OK, ctx)?;
                }

                save.I = (save.I + 1);
            }

            //
            // Find the gaps in the inverse grid. These should
            // combine to form the coverage of the original grid.
            //
            save.VALUE = !save.VALUE;

            support::FNDCMP(
                save.NROWS,
                save.NCOLS,
                save.VALUE,
                MAXGRD,
                save.GRID1.as_slice_mut(),
                save.VSET.as_slice_mut(),
                save.MRKSET.as_slice_mut(),
                save.TMPSET.as_slice_mut(),
                &mut save.NCOMP,
                save.MINPXX.as_slice_mut(),
                save.MAXPXX.as_slice_mut(),
                save.MINPXY.as_slice_mut(),
                save.MAXPXY.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Count the pixels in the gaps.
            //
            save.J = 0;

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NCOMP;
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.J = (save.J
                        + (((save.MAXPXX[save.I] - save.MINPXX[save.I]) + 1)
                            * ((save.MAXPXY[save.I] - save.MINPXY[save.I]) + 1)));

                    save.I += m3__;
                }
            }
            //
            // Check the coverage count versus that implied by the
            // gap count.
            //
            testutil::CHCKSI(
                b"J (coverage)",
                save.J,
                b"=",
                (save.NGRID - save.NGAP),
                0,
                OK,
                ctx,
            )?;

            save.N += m3__;
        }
    }

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
