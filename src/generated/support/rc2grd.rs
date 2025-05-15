//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    BSIZE: i32,
    J: i32,
    K: i32,
    MAXPXX: i32,
    MAXPXY: i32,
    MINPXX: i32,
    MINPXY: i32,
    NGRID: i32,
    RNGMAX: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BSIZE: i32 = 0;
        let mut J: i32 = 0;
        let mut K: i32 = 0;
        let mut MAXPXX: i32 = 0;
        let mut MAXPXY: i32 = 0;
        let mut MINPXX: i32 = 0;
        let mut MINPXY: i32 = 0;
        let mut NGRID: i32 = 0;
        let mut RNGMAX: i32 = 0;

        Self {
            BSIZE,
            J,
            K,
            MAXPXX,
            MAXPXY,
            MINPXX,
            MINPXY,
            NGRID,
            RNGMAX,
        }
    }
}

//$Procedure RC2GRD ( DSKBRIEF, rectangles to pixel grid )
pub fn RC2GRD(
    NREC: i32,
    BNDS1: &[f64],
    BNDS2: &[f64],
    MAXGRD: i32,
    MAXORD: i32,
    VALUE: bool,
    ORD1: &mut [i32],
    ORD2: &mut [i32],
    CIVOR1: &mut [i32],
    CIVOR2: &mut [i32],
    PXMAP1: &mut [i32],
    PXMAP2: &mut [i32],
    NROWS: &mut i32,
    NCOLS: &mut i32,
    GRID: &mut [bool],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let BNDS1 = DummyArray2D::new(BNDS1, 1..=2, 1..=NREC);
    let BNDS2 = DummyArray2D::new(BNDS2, 1..=2, 1..=NREC);
    let mut ORD1 = DummyArrayMut::new(ORD1, 1..);
    let mut ORD2 = DummyArrayMut::new(ORD2, 1..);
    let mut CIVOR1 = DummyArrayMut::new(CIVOR1, 1..);
    let mut CIVOR2 = DummyArrayMut::new(CIVOR2, 1..);
    let mut PXMAP1 = DummyArrayMut::new(PXMAP1, 1..);
    let mut PXMAP2 = DummyArrayMut::new(PXMAP2, 1..);
    let mut GRID = DummyArrayMut::new(GRID, 1..);

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Saved values
    //

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"RC2GRD", ctx)?;

    //
    // Check input size arguments for obvious initialization errors.
    //
    if (NREC < 1) {
        spicelib::SETMSG(b"NREC is #; must be positive.", ctx);
        spicelib::ERRINT(b"#", NREC, ctx);
        spicelib::SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        spicelib::CHKOUT(b"RC2GRD", ctx)?;
        return Ok(());
    }

    if (MAXGRD < 1) {
        spicelib::SETMSG(b"MAXGRD is #; must be positive.", ctx);
        spicelib::ERRINT(b"#", MAXGRD, ctx);
        spicelib::SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        spicelib::CHKOUT(b"RC2GRD", ctx)?;
        return Ok(());
    }

    if (MAXORD < 1) {
        spicelib::SETMSG(b"MAXORD is #; must be positive.", ctx);
        spicelib::ERRINT(b"#", MAXORD, ctx);
        spicelib::SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        spicelib::CHKOUT(b"RC2GRD", ctx)?;
        return Ok(());
    }

    //
    // All input rectangle heights and widths must be strictly
    // positive.
    //
    for I in 1..=NREC {
        if (BNDS1[[2, I]] <= BNDS1[[1, I]]) {
            spicelib::SETMSG(
                b"BNDS1(2,#) = #; BNDS1(1,#) = #. Rectangle widths (and heights) must be positive.",
                ctx,
            );
            spicelib::ERRINT(b"#", I, ctx);
            spicelib::ERRDP(b"#", BNDS1[[2, I]], ctx);
            spicelib::ERRINT(b"#", I, ctx);
            spicelib::ERRDP(b"#", BNDS1[[1, I]], ctx);
            spicelib::SIGERR(b"SPICE(INVALIDBOUNDS)", ctx)?;
            spicelib::CHKOUT(b"RC2GRD", ctx)?;
            return Ok(());
        }

        if (BNDS2[[2, I]] <= BNDS2[[1, I]]) {
            spicelib::SETMSG(
                b"BNDS2(2,#) = #; BNDS2(1,#) = #. Rectangle heights (and widths) must be positive.",
                ctx,
            );
            spicelib::ERRINT(b"#", I, ctx);
            spicelib::ERRDP(b"#", BNDS2[[2, I]], ctx);
            spicelib::ERRINT(b"#", I, ctx);
            spicelib::ERRDP(b"#", BNDS2[[1, I]], ctx);
            spicelib::SIGERR(b"SPICE(INVALIDBOUNDS)", ctx)?;
            spicelib::CHKOUT(b"RC2GRD", ctx)?;
            return Ok(());
        }
    }

    //
    // Find the order of the array of X bounds. We treat the array as a
    // one-dimensional array of length 2*NREC.
    //
    // Produce the corresponding "compressed" inverse order vector. By
    // "compressed" we mean: suppose the set of input values were sorted
    // and compressed so that it contained no duplicates. For each
    // member of the original value array, map the member's index to the
    // index of the member in the compressed, sorted array. The
    // compressed inverse order vector contains this mapping.
    //
    save.BSIZE = (2 * NREC);

    IOVCMP(
        BNDS1.as_slice(),
        save.BSIZE,
        ORD1.as_slice_mut(),
        CIVOR1.as_slice_mut(),
        &mut save.RNGMAX,
    );

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"RC2GRD", ctx)?;
        return Ok(());
    }

    for I in 1..=save.BSIZE {
        save.J = ORD1[I];

        save.K = (((save.J - 1) / 2) + 1);
    }

    //
    // The width of the pixel grid is one less than the number of
    // distinct X bound values.
    //
    *NCOLS = (save.RNGMAX - 1);

    //
    // Get the order vector and compressed inverse order vector of
    // the Y bounds. (Note we have the same number of X and Y
    // bounds.)
    //
    IOVCMP(
        BNDS2.as_slice(),
        save.BSIZE,
        ORD2.as_slice_mut(),
        CIVOR2.as_slice_mut(),
        &mut save.RNGMAX,
    );

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"RC2GRD", ctx)?;
        return Ok(());
    }

    //
    // The height of the pixel grid is one less than the number of
    // distinct Y bound values.
    //
    *NROWS = (save.RNGMAX - 1);

    //
    // Check the grid size again, now that we know how large it
    // needs to be.
    //
    save.NGRID = (*NROWS * *NCOLS);

    if (MAXGRD < save.NGRID) {
        spicelib::SETMSG(b"MAXGRD is #; must be have size at least # in order to hold pixels for current set of rectangles.", ctx);
        spicelib::ERRINT(b"#", MAXGRD, ctx);
        spicelib::ERRINT(b"#", save.NGRID, ctx);
        spicelib::SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        spicelib::CHKOUT(b"RC2GRD", ctx)?;
        return Ok(());
    }

    //
    // A program using this routine normally will need to map pixel
    // coordinates back to their corresponding d.p. values. Create
    // arrays to represent these mappings.
    //
    // Note that, since the bounds arrays are generally larger than the
    // corresponding pixel grid dimensions, the mappings we're about to
    // perform (which map the integers 1:BIZE into the ranges of the
    // compressed inverse order vectors) are not 1-1. They're still
    // valid; the process is just a bit ungainly because it can involve
    // overwriting elements of the output array. Each time this happens,
    // the affected output array element gets overwritten with the same
    // value it already had.
    //
    // We'll store the mappings in the arrays PXMAP1 and PXMAP2.
    // The pixel coordinates
    //
    //    ( I, J )
    //
    // correspond to the double precision coordinates
    //
    //    BNDS1( PXMAP1(I) )
    //    BNDS2( PXMAP2(J) )
    //
    // where we're treating BNDS1 and BNDS2 as one-dimensional
    // arrays of length BSIZE.
    //

    for I in 1..=save.BSIZE {
        PXMAP1[CIVOR1[I]] = I;
        PXMAP2[CIVOR2[I]] = I;
    }

    //
    // Now map all rectangles to the integer indices of their
    // bounds in pixel space. Note that the pixel grid has
    // dimensions
    //
    //    ( NROWS, NCOLS )
    //
    // and the ranges of the integer coordinates of the
    // rectangle boundaries are
    //
    //    1 : NROWS + 1
    //    1 : NCOLS + 1
    //
    // We'll fill in the pixel grid to indicate which pixels are
    // covered by rectangles, and which ones lie in gaps.
    //
    // Initialize the grid to indicate that it consists of one
    // large gap.
    //
    for I in 1..=save.NGRID {
        GRID[I] = !VALUE;
    }

    //
    // For each input rectangle, mark the corresponding pixels
    // covered by the rectangle. Note that maximum pixel indices
    // are less by one than those of the corresponding rectangle
    // upper bound indices.
    //
    for I in 1..=NREC {
        //
        // Compute the bounds of the current rectangle in pixel
        // space. Recall that the all bounds for a given coordinate
        // (X or Y) are combined in a sequence of size 2*NREC.
        //
        save.J = ((2 * (I - 1)) + 1);

        save.MINPXX = CIVOR1[save.J];
        save.MINPXY = CIVOR2[save.J];

        save.K = (2 * I);

        save.MAXPXX = CIVOR1[save.K];
        save.MAXPXY = CIVOR2[save.K];

        for COL in save.MINPXX..=(save.MAXPXX - 1) {
            for ROW in save.MINPXY..=(save.MAXPXY - 1) {
                //
                // Mark the pixel at indices (ROW, COL) as
                // covered.
                //
                save.J = ((*NROWS * (COL - 1)) + ROW);

                GRID[save.J] = VALUE;
            }
        }
    }

    spicelib::CHKOUT(b"RC2GRD", ctx)?;
    Ok(())
}
