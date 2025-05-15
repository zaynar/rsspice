//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure ZZGRDPLT ( Create grid of plates )
pub fn ZZGRDPLT(
    NROWS: i32,
    NCOLS: i32,
    WRAP: bool,
    NP: &mut i32,
    PLATES: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut PLATES = DummyArrayMut2D::new(PLATES, 1..=3, 1..);
    let mut BL: i32 = 0;
    let mut BR: i32 = 0;
    let mut TL: i32 = 0;
    let mut TR: i32 = 0;
    let mut UB: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"ZZGRDPLT", ctx)?;

    //
    // Check row and column dimensions.
    //
    if (NROWS < 2) {
        spicelib::SETMSG(b"Grid must have at least two rows but NROWS is #.", ctx);
        spicelib::ERRINT(b"#", NROWS, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDDIMENSION)", ctx)?;
        spicelib::CHKOUT(b"ZZGRDPLT", ctx)?;
        return Ok(());
    }

    if (NCOLS < 2) {
        spicelib::SETMSG(b"Grid must have at least two columns but NCOLSS is #.", ctx);
        spicelib::ERRINT(b"#", NCOLS, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDDIMENSION)", ctx)?;
        spicelib::CHKOUT(b"ZZGRDPLT", ctx)?;
        return Ok(());
    }

    //
    // Set the upper bound on the column loop. If longitude
    // wrapping is turned on, the final column connects to
    // the first column.
    //
    if WRAP {
        UB = NCOLS;
    } else {
        UB = (NCOLS - 1);
    }

    //
    // Connect the vertices to generate plates.
    //
    *NP = 0;

    for I in 1..=(NROWS - 1) {
        for J in 1..=UB {
            //
            // Since the vertices are stored in a 3xN
            // array, it will be convenient to use aliases for
            // their indices---this cuts down on use of complicated
            // index expressions. For each square defined by
            // four neighboring vertices, we'll call the vertices
            //
            //    TL   "top left"
            //    TR   "top right"
            //    BL   "bottom left"
            //    BR   "bottom right"
            //
            // Recall that the input pixel grid has dimensions
            //
            //    NROWS x NCOLS
            //
            // The top row is at the highest latitude.
            //
            // The leftmost column corresponds to the west
            // boundary of the region.
            //
            // The top left vertex corresponds to pixel (I,J).
            //
            if (WRAP && (J == UB)) {
                //
                // Connect the right edge of the grid to the left edge.
                //
                TL = (I * NCOLS);
                TR = ((TL - NCOLS) + 1);
                BL = (TL + NCOLS);
                BR = (TR + NCOLS);
            } else {
                //
                // This is the normal case: the column at index
                // J is connected to the column at index J+1.
                //
                TL = (((I - 1) * NCOLS) + J);
                TR = (TL + 1);
                BL = (TL + NCOLS);
                BR = (BL + 1);
            }

            //
            // For each square defined by neighboring pixel centers,
            // we must represent the corresponding surface by a pair
            // of plates. We have two choices for the diagonal
            // common edge connecting these plates: descending or
            // ascending to the right.
            //
            // We choose the descending diagonal.
            //
            // The vertex assignment must be positively
            // oriented about the outward normal direction.
            //
            *NP = (*NP + 1);

            PLATES[[1, *NP]] = BL;
            PLATES[[2, *NP]] = BR;
            PLATES[[3, *NP]] = TL;

            *NP = (*NP + 1);

            PLATES[[1, *NP]] = TL;
            PLATES[[2, *NP]] = BR;
            PLATES[[3, *NP]] = TR;
        }
    }

    //
    // The plate and vertex counts and arrays have been
    // assigned.
    //
    spicelib::CHKOUT(b"ZZGRDPLT", ctx)?;
    Ok(())
}
