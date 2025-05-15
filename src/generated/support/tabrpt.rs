//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const MXWDTH: i32 = 132;
pub const MAXCOL: i32 = 60;
const RNAME: &[u8] = b"TABRPT";
const STLSIZ: i32 = 80;
const WDSIZE: i32 = 32;
const LNGSIZ: i32 = 1024;
const LBCELL: i32 = -5;
const HLFHLD: i32 = 130;
const THSHLD: i32 = (2 * HLFHLD);
const PAGESZ: i32 = (2 * THSHLD);

struct SaveVars {
    STYLE: ActualCharArray,
    COUNT: StackArray<i32, 60>,
    ROW: StackArray<i32, 60>,
    DONE: StackArray<bool, 60>,
    LONG: Vec<u8>,
    GETSTR: Vec<u8>,
    BUFFER: ActualCharArray,
    PAGE: ActualCharArray,
    VALUE: Vec<u8>,
    KEY: Vec<u8>,
    HRD: Vec<u8>,
    DID: i32,
    L: i32,
    LAST: i32,
    LEFT: i32,
    MAXROW: i32,
    NROWS: i32,
    PUTAT: i32,
    R: i32,
    RIGHT: i32,
    ROOM: i32,
    TOSHIP: i32,
    WDTH: i32,
    DOHRD: bool,
    FILLED: bool,
    FINISH: bool,
    FULL: bool,
    NOROOM: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut STYLE = ActualCharArray::new(STLSIZ, 1..=MAXCOL);
        let mut COUNT = StackArray::<i32, 60>::new(1..=MAXCOL);
        let mut ROW = StackArray::<i32, 60>::new(1..=MAXCOL);
        let mut DONE = StackArray::<bool, 60>::new(1..=MAXCOL);
        let mut LONG = vec![b' '; LNGSIZ as usize];
        let mut GETSTR = vec![b' '; LNGSIZ as usize];
        let mut BUFFER = ActualCharArray::new(MXWDTH, LBCELL..=THSHLD);
        let mut PAGE = ActualCharArray::new(MXWDTH, 1..=PAGESZ);
        let mut VALUE = vec![b' '; WDSIZE as usize];
        let mut KEY = vec![b' '; WDSIZE as usize];
        let mut HRD = vec![b' '; MAXCOL as usize];
        let mut DID: i32 = 0;
        let mut L: i32 = 0;
        let mut LAST: i32 = 0;
        let mut LEFT: i32 = 0;
        let mut MAXROW: i32 = 0;
        let mut NROWS: i32 = 0;
        let mut PUTAT: i32 = 0;
        let mut R: i32 = 0;
        let mut RIGHT: i32 = 0;
        let mut ROOM: i32 = 0;
        let mut TOSHIP: i32 = 0;
        let mut WDTH: i32 = 0;
        let mut DOHRD: bool = false;
        let mut FILLED: bool = false;
        let mut FINISH: bool = false;
        let mut FULL: bool = false;
        let mut NOROOM: bool = false;

        fstr::assign(&mut KEY, b"abort");
        fstr::assign(&mut HRD, b" ");
        DOHRD = false;

        Self {
            STYLE,
            COUNT,
            ROW,
            DONE,
            LONG,
            GETSTR,
            BUFFER,
            PAGE,
            VALUE,
            KEY,
            HRD,
            DID,
            L,
            LAST,
            LEFT,
            MAXROW,
            NROWS,
            PUTAT,
            R,
            RIGHT,
            ROOM,
            TOSHIP,
            WDTH,
            DOHRD,
            FILLED,
            FINISH,
            FULL,
            NOROOM,
        }
    }
}

//$Procedure      TABRPT ( Table Format Report )
pub fn TABRPT(
    NITEMS: i32,
    ITEM: &[i32],
    SIZE: &[i32],
    WIDTH: &[i32],
    JUSTR: &[bool],
    PRESRV: &[bool],
    SPCIAL: CharArray,
    LMARGE: i32,
    SPACE: i32,
    FETCH: fn(i32, i32, &mut [u8], &mut i32, &mut Context) -> f2rust_std::Result<()>,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let ITEM = DummyArray::new(ITEM, 1..);
    let SIZE = DummyArray::new(SIZE, 1..);
    let WIDTH = DummyArray::new(WIDTH, 1..);
    let JUSTR = DummyArray::new(JUSTR, 1..);
    let PRESRV = DummyArray::new(PRESRV, 1..);
    let SPCIAL = DummyCharArray::new(SPCIAL, None, 1..);

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
    // The arrays below are used to store attributes on a column
    // by column basis.
    //
    // STYLE  is the style to be used when formating text for an
    //        individual column
    //
    // COUNT  is a counter that is used to indicate how many components
    //        have been processed for an individual column
    //
    // ROW    keeps track of the last row in the local page where
    //        formatted text was placed.
    //
    // DONE   is a logical that indicates whether we have formatted
    //        all of the data for a column.
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"TABRPT", ctx)?;
    //
    // Initialize the cell that is used by NICEBT and make sure
    // the page is completely blank
    //
    for I in 1..=PAGESZ {
        fstr::assign(save.PAGE.get_mut(I), b" ");
    }
    //
    // Initialize the local page and set the column parameters.
    //

    for I in 1..=NITEMS {
        save.DONE[I] = false;
        save.COUNT[I] = 0;
        save.ROW[I] = 0;
        spicelib::REPMI(b"LEFT 1 RIGHT #", b"#", WIDTH[I], &mut save.STYLE[I], ctx);
        spicelib::SUFFIX(&SPCIAL[I], 1, &mut save.STYLE[I]);
    }
    //
    // The logical FINISH is used to keep track of whether or not
    // we have finished processing all items.  Certainly we haven't
    // done so yet.  It will be the value of the expression given
    // by DONE(1) .AND. DONE(2) .AND. ... .AND. DONE(NITEMS)
    //
    save.FINISH = false;

    while !save.FINISH {
        //
        // We need to reset the left margin of the page.
        //
        save.LEFT = LMARGE;

        for ID in 1..=NITEMS {
            //
            // We are going to format items for output one at a time.
            // We will either fetch all of the components, or we
            // will fill up the room allotted for this item in the
            // buffer that will hold the data.
            //
            // Thus at the end of this loop, we will have filled
            // up as much room as there is for this part of the
            // report and be ready to send that stuff to the
            // printer.
            //
            // Set the right margin and determine whether or not
            // the  COLUMN that holds the text to be formatted is
            // already filled up.
            //
            save.FILLED = ((save.ROW[ID] >= THSHLD) || save.DONE[ID]);

            save.RIGHT = ((save.LEFT + WIDTH[ID]) - 1);

            while !save.FILLED {
                //
                // Put data into the long string for output until
                // it becomes full or it is appropriate to stop doing
                // so (there's no more data, or the PRESRV flag tells
                // us to stop).
                save.PUTAT = 1;
                save.FULL = false;
                save.ROOM = intrinsics::MIN0(&[LNGSIZ, (WIDTH[ID] * HLFHLD)]);
                fstr::assign(&mut save.LONG, b" ");

                while (!save.DONE[ID] && !save.FULL) {
                    //
                    // Increment COUNT so that we can fetch the next
                    // component of this item.
                    //
                    save.COUNT[ID] = (save.COUNT[ID] + 1);

                    FETCH(
                        ITEM[ID],
                        save.COUNT[ID],
                        &mut save.GETSTR,
                        &mut save.WDTH,
                        ctx,
                    )?;

                    if spicelib::FAILED(ctx) {
                        spicelib::CHKOUT(b"TABRPT", ctx)?;
                        return Ok(());
                    }

                    //
                    // Determine the next place to add on to this string
                    // and see if adding on at that point would fill up
                    // the available space in our string.
                    //
                    save.L = QLSTNB(&save.GETSTR);
                    save.LAST = intrinsics::MAX0(&[save.L, 1]);

                    if ((save.PUTAT + save.L) < save.ROOM) {
                        fstr::assign(
                            fstr::substr_mut(&mut save.LONG, save.PUTAT..),
                            fstr::substr(&save.GETSTR, 1..=save.LAST),
                        );
                        save.PUTAT = intrinsics::MIN0(&[((save.PUTAT + save.L) + 2), LNGSIZ]);

                        //
                        // If the input was a blank, we step back to
                        // the beginning of the string.
                        //
                        if (save.PUTAT == 2) {
                            save.PUTAT = 1;
                        }

                        save.NOROOM = ((save.PUTAT + WIDTH[ID]) >= save.ROOM);
                    } else if (save.PUTAT == 1) {
                        //
                        // This case is very funky.  We are at the very
                        // beginning of the output buffer, but there still
                        // isn't room.  This means the user requested
                        // a width such that HLFHLD * WIDTH(ID)  is smaller
                        // than the size of the data in the column.
                        // In other words, the width must be less than
                        // the value DATA_LENGTH/HLFHLD.  Since the
                        // maximum data length is 1024 and HLFHLD is
                        // at last look 130, this means they have asked
                        // to fit data that is very long into a very
                        // column that is less than 8 characters wide.
                        // Sorry but there doesn't seem to be a morally
                        // compelling reason to handle this case
                        // robustly.  We just put some dots at the end
                        // of the output to indicate there's more stuff
                        // that can't be printed.
                        //
                        fstr::assign(&mut save.LONG, &save.GETSTR);
                        save.NOROOM = true;
                        fstr::assign(
                            fstr::substr_mut(&mut save.LONG, (save.ROOM - 7)..=save.ROOM),
                            b"........",
                        );
                        save.PUTAT = save.ROOM;
                    } else {
                        //
                        // There isn't room to append GETSTR to the end
                        // of LONG.  Adjust the counter back by 1 and
                        // set NOROOM to .TRUE.
                        //
                        save.COUNT[ID] = (save.COUNT[ID] - 1);
                        save.NOROOM = true;
                    }

                    save.DONE[ID] = (save.COUNT[ID] >= SIZE[ID]);
                    save.FULL = (PRESRV[ID] || save.NOROOM);
                }

                //
                // Format the string into the holding buffer.
                //
                spicelib::SSIZEC(THSHLD, save.BUFFER.as_arg_mut(), ctx)?;
                NICEBT_1(
                    fstr::substr(&save.LONG, 1..=save.PUTAT),
                    &save.STYLE[ID],
                    save.BUFFER.as_arg_mut(),
                    ctx,
                )?;
                if spicelib::FAILED(ctx) {
                    spicelib::CHKOUT(b"TABRPT", ctx)?;
                    return Ok(());
                }

                save.NROWS = spicelib::CARDC(save.BUFFER.as_arg(), ctx)?;
                //
                // Transfer the data from the holding buffer
                // to the page layout buffer.
                //
                for J in 1..=save.NROWS {
                    save.ROW[ID] = (save.ROW[ID] + 1);
                    save.R = save.ROW[ID];

                    fstr::assign(
                        fstr::substr_mut(save.PAGE.get_mut(save.R), save.LEFT..=save.RIGHT),
                        save.BUFFER.get(J),
                    );

                    if JUSTR[ID] {
                        spicelib::RJUST(
                            &fstr::substr(&save.PAGE[save.R], save.LEFT..=save.RIGHT).to_vec(),
                            fstr::substr_mut(&mut save.PAGE[save.R], save.LEFT..=save.RIGHT),
                        );
                    }
                    //
                    // Replace any "hardspaces" by blanks.
                    //
                    if save.DOHRD {
                        if fstr::ne(fstr::substr(&save.HRD, ID..=ID), b" ") {
                            spicelib::REPLCH(
                                &fstr::substr(&save.PAGE[save.R], save.LEFT..=save.RIGHT).to_vec(),
                                fstr::substr(&save.HRD, ID..=ID),
                                b" ",
                                fstr::substr_mut(&mut save.PAGE[save.R], save.LEFT..=save.RIGHT),
                            );
                        }
                    }
                }

                //
                // Determine whether this column has been sufficiently
                // filled up.
                //
                save.DONE[ID] = (save.COUNT[ID] >= SIZE[ID]);
                save.FILLED = (save.DONE[ID] || (save.ROW[ID] >= THSHLD));
            }

            //
            // Once you get to this point, the current column has
            // been filled as much as is possible.   We need to
            // Set the left margin for the next item to process
            //
            save.LEFT = ((save.RIGHT + SPACE) + 1);
        }

        //
        // By the time you get to this point, every column has either
        // filled up or there's nothing left to print.
        //
        // In either case we need to ship out the rows from
        // 1 to MIN ( MAX{ROW(1) ... ROW(NITEMS)}, THRSHOLD )
        // and shift the rest of the stuff up in the buffer.
        //
        save.MAXROW = 0;

        for I in 1..=NITEMS {
            save.MAXROW = intrinsics::MAX0(&[save.MAXROW, save.ROW[I]]);
        }

        save.TOSHIP = intrinsics::MIN0(&[save.MAXROW, THSHLD]);
        //
        // Ship out the rows that are ready to go.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.TOSHIP;
            let m3__: i32 = 1;
            save.R = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                PAGPUT(&save.PAGE[save.R], ctx)?;

                PAGPMT(&mut save.DID, &mut save.VALUE, ctx);

                if (save.DID != 0) {
                    if spicelib::EQSTR(&save.VALUE, &save.KEY) {
                        spicelib::CHKOUT(b"TABRPT", ctx)?;
                        return Ok(());
                    }
                }

                save.R += m3__;
            }
        }

        //
        // Shift the remaining rows up to the top of the page
        //
        {
            let m1__: i32 = (save.TOSHIP + 1);
            let m2__: i32 = PAGESZ;
            let m3__: i32 = 1;
            save.R = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                let val = save.PAGE.get(save.R).to_vec();
                fstr::assign(save.PAGE.get_mut((save.R - save.TOSHIP)), &val);
                save.R += m3__;
            }
        }

        //
        // Blank out the last TOSHIP rows.
        //
        {
            let m1__: i32 = ((PAGESZ - save.TOSHIP) + 1);
            let m2__: i32 = PAGESZ;
            let m3__: i32 = 1;
            save.R = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                fstr::assign(save.PAGE.get_mut(save.R), b" ");
                save.R += m3__;
            }
        }
        //
        // Finally adjust the positions where each column should begin
        // filling in more data.
        //
        for J in 1..=NITEMS {
            save.ROW[J] = intrinsics::MAX0(&[(save.ROW[J] - save.TOSHIP), 0]);
        }

        //
        // Now examine each of the ID's to see if we are done
        // processing all items.
        //
        save.FINISH = true;

        for ID in 1..=NITEMS {
            save.FINISH = (save.FINISH && save.DONE[ID]);
        }
    }
    //
    // Send any remaining rows out to the page manager.
    //
    save.MAXROW = 0;

    for I in 1..=NITEMS {
        save.MAXROW = intrinsics::MAX0(&[save.MAXROW, save.ROW[I]]);
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.MAXROW;
        let m3__: i32 = 1;
        save.R = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            PAGPUT(&save.PAGE[save.R], ctx)?;
            fstr::assign(save.PAGE.get_mut(save.R), b" ");

            PAGPMT(&mut save.DID, &mut save.VALUE, ctx);

            if (save.DID != 0) {
                if spicelib::EQSTR(&save.VALUE, &save.KEY) {
                    spicelib::CHKOUT(b"TABRPT", ctx)?;
                    return Ok(());
                }
            }

            save.R += m3__;
        }
    }

    spicelib::CHKOUT(b"TABRPT", ctx)?;
    Ok(())
}

//$Procedure      TABABT ( Tabular Report Abort Key )
pub fn TABABT(SPCIAL: CharArray, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let SPCIAL = DummyCharArray::new(SPCIAL, None, 1..);

    fstr::assign(&mut save.KEY, SPCIAL.get(1));
}

//$Procedure      TABHRD ( Tabular Report Hard Space )
pub fn TABHRD(NITEMS: i32, SPCIAL: CharArray, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let SPCIAL = DummyCharArray::new(SPCIAL, None, 1..);

    fstr::assign(&mut save.HRD, b" ");
    save.DOHRD = false;

    for I in 1..=NITEMS {
        fstr::assign(fstr::substr_mut(&mut save.HRD, I..=I), SPCIAL.get(I));
        save.DOHRD = (save.DOHRD || fstr::ne(fstr::substr(&save.HRD, I..=I), b" "));
    }
}
