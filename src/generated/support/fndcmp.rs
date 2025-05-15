//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

//$Procedure FNDCMP ( DSKBRIEF, find rectangular components )
pub fn FNDCMP(
    NROWS: i32,
    NCOLS: i32,
    VALUE: bool,
    MAXN: i32,
    GRID: &mut [bool],
    VSET: &mut [i32],
    MRKSET: &mut [i32],
    TMPSET: &mut [i32],
    NCOMP: &mut i32,
    MINPXX: &mut [i32],
    MAXPXX: &mut [i32],
    MINPXY: &mut [i32],
    MAXPXY: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut GRID = DummyArrayMut2D::new(GRID, 1..=NROWS, 1..=NCOLS);
    let mut VSET = DummyArrayMut::new(VSET, LBCELL..);
    let mut MRKSET = DummyArrayMut::new(MRKSET, LBCELL..);
    let mut TMPSET = DummyArrayMut::new(TMPSET, LBCELL..);
    let mut MINPXX = DummyArrayMut::new(MINPXX, 1..);
    let mut MAXPXX = DummyArrayMut::new(MAXPXX, 1..);
    let mut MINPXY = DummyArrayMut::new(MINPXY, 1..);
    let mut MAXPXY = DummyArrayMut::new(MAXPXY, 1..);
    let mut COL: i32 = 0;
    let mut COLSIZ: i32 = 0;
    let mut ID: i32 = 0;
    let mut J: i32 = 0;
    let mut MAXCOL: i32 = 0;
    let mut MAXROW: i32 = 0;
    let mut MINCOL: i32 = 0;
    let mut MINROW: i32 = 0;
    let mut REMAIN: i32 = 0;
    let mut ROW: i32 = 0;
    let mut FOUND: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"FNDCMP", ctx)?;

    spicelib::SCARDI(0, VSET.as_slice_mut(), ctx)?;
    spicelib::SCARDI(0, MRKSET.as_slice_mut(), ctx)?;
    spicelib::SCARDI(0, TMPSET.as_slice_mut(), ctx)?;

    //
    // First step: make a pass through the grid, and store the ID
    // of each pixel matching VALUE.
    //
    // Proceed in column-major order.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NCOLS;
        let m3__: i32 = 1;
        COL = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = NROWS;
                let m3__: i32 = 1;
                ROW = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    if (GRID[[ROW, COL]] == VALUE) {
                        //
                        // It's a match.
                        //
                        // ID is the one-dimensional index of the current pixel.
                        //
                        ID = (((COL - 1) * NROWS) + ROW);

                        //
                        // Since we're traversing the grid in increasing ID
                        // order, the elements of VSET will automatically be
                        // in increasing order. We don't need to sort them.
                        //
                        spicelib::APPNDI(ID, VSET.as_slice_mut(), ctx)?;

                        if spicelib::FAILED(ctx) {
                            spicelib::CHKOUT(b"FNDCMP", ctx)?;
                            return Ok(());
                        }
                    }

                    ROW += m3__;
                }
            }

            COL += m3__;
        }
    }

    //
    // Now find rectangular sets of pixels equal to VALUE.
    //
    spicelib::SCARDI(0, MRKSET.as_slice_mut(), ctx)?;

    REMAIN = spicelib::CARDI(VSET.as_slice(), ctx)?;

    *NCOMP = 0;

    while (REMAIN > 0) {
        //
        // Get the row and column coordinates of the first pixel in VSET.
        //
        ID = VSET[1];

        COL = (((ID - 1) / NROWS) + 1);

        ROW = (ID - ((COL - 1) * NROWS));

        MINROW = ROW;
        MINCOL = COL;
        //
        // We'll extend the component in the direction of higher row
        // indices as far as possible, then in the direction of higher
        // column indices  as far as possible. The reason for this is
        // that we want to accumulate pixels in increasing order of ID.
        //
        MAXROW = NROWS;
        MAXCOL = COL;
        FOUND = true;

        while ((COL <= NCOLS) && FOUND) {
            //
            // COL is a valid column number at the top of the loop.
            // We increment COL at the bottom of the loop.
            //
            // Initialize ROW for a pass through the current column.
            //
            ROW = (MINROW - 1);

            //
            // Caution: the value of MAXROW in the loop termination
            // condition below changes during loop execution! The
            // value is NROWS on the first pass; then it changes
            // to the maximum row number of the first column of the
            // component.
            //
            while ((ROW < MAXROW) && FOUND) {
                //
                // Note the .LT. operator in the loop termination
                // condition. We increment ROW at the top of the
                // loop, so the value of ROW is correct after
                // loop termination.
                //
                ROW = (ROW + 1);

                FOUND = (GRID[[ROW, COL]] == VALUE);
            }

            if (COL == MINCOL) {
                //
                // The index of the last row that matched becomes the
                // maximum row index of this component.
                //
                if FOUND {
                    //
                    // The row index reached NROWS.
                    //
                    MAXROW = NROWS;
                } else {
                    //
                    // The last matching row was the one preceding ROW.
                    //
                    MAXROW = (ROW - 1);
                    //
                    // Set FOUND to .TRUE. so we'll go on to look at the
                    // next column.
                    //
                    FOUND = true;
                }
                //
                // Now we know the size of the columns of the component.
                //
                COLSIZ = ((MAXROW - MINROW) + 1);
                //
                // Always go on to look at the next column. FOUND is
                // .TRUE. at this point.
                //
                MAXCOL = COL;
            } else {
                //
                // After we process the first column of the component,
                // we don't adjust MAXROW again. It's set to the highest
                // row number of the first column of the component.
                //
                if !FOUND {
                    //
                    // The current column fails to match in some row of the
                    // current column. This column can't be included in the
                    // component.
                    //
                    MAXCOL = (COL - 1);
                } else {
                    //
                    // The current column matches from row indices MINROW
                    // to MAXROW. This column is part of the component.
                    //
                    MAXCOL = COL;
                    //
                    // Set FOUND to .TRUE. so we'll go on to look at the
                    // next column.
                    //
                    FOUND = true;
                }
            }

            if FOUND {
                //
                // We've found a column of matching pixels in the
                // current component.
                //
                // Add the pixels of the column to the marked set.
                //
                // Let ID be the ID of the first pixel of the column.
                //
                ID = (((MAXCOL - 1) * NROWS) + MINROW);

                for I in 1..=COLSIZ {
                    J = ((ID - 1) + I);

                    spicelib::APPNDI(J, MRKSET.as_slice_mut(), ctx)?;

                    if spicelib::FAILED(ctx) {
                        spicelib::CHKOUT(b"FNDCMP", ctx)?;
                        return Ok(());
                    }

                    //
                    // Fill in the matching pixels so they won't
                    // match again.
                    //
                    GRID[[((MINROW - 1) + I), MAXCOL]] = !VALUE;
                }
                //
                // Note that we've added the IDs to MRKSET in increasing
                // order, so MRKSET remains a set. We don't need to sort
                // its contents.
                //
                // Prepare to examine the next column.
                //
                REMAIN = (REMAIN - COLSIZ);
                COL = (COL + 1);
            }
        }
        //
        // We've finished building a component.
        //
        *NCOMP = (*NCOMP + 1);

        //
        // Update VSET: subtract the pixels of the new component.
        //
        // Note that subtracting one set from another should be an
        // efficient process, if done correctly. We trust DIFFI to manage
        // this.
        //
        spicelib::DIFFI(
            VSET.as_slice(),
            MRKSET.as_slice(),
            TMPSET.as_slice_mut(),
            ctx,
        )?;
        spicelib::COPYI(TMPSET.as_slice(), VSET.as_slice_mut(), ctx)?;

        spicelib::SCARDI(0, MRKSET.as_slice_mut(), ctx)?;
        spicelib::SCARDI(0, TMPSET.as_slice_mut(), ctx)?;

        if spicelib::FAILED(ctx) {
            spicelib::CHKOUT(b"FNDCMP", ctx)?;
            return Ok(());
        }

        // The bounds of the component we just found are given by
        //
        //    MINROW, MAXROW, MINCOL, MAXCOL
        //
        if (*NCOMP <= MAXN) {
            MINPXX[*NCOMP] = MINCOL;
            MAXPXX[*NCOMP] = MAXCOL;
            MINPXY[*NCOMP] = MINROW;
            MAXPXY[*NCOMP] = MAXROW;
        } else {
            //
            // We're out of room.
            //
            spicelib::SETMSG(b"There are more output rectangles than can be accommodated in the output rectangle boundary arrays. So far, # components have been found; the maximum supported number is #.", ctx);
            spicelib::ERRINT(b"#", *NCOMP, ctx);
            spicelib::ERRINT(b"#", MAXN, ctx);
            spicelib::SIGERR(b"SPICE(ARRAYTOOSMALL)", ctx)?;
            spicelib::CHKOUT(b"FNDCMP", ctx)?;
            return Ok(());
        }
    }

    spicelib::CHKOUT(b"FNDCMP", ctx)?;
    Ok(())
}
