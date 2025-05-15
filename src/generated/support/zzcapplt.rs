//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure ZZCAPPLT ( Make polar cap plates )
pub fn ZZCAPPLT(
    NCOLS: i32,
    NORTH: bool,
    WRAP: bool,
    BASIDX: i32,
    POLIDX: i32,
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

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"ZZCAPPLT", ctx)?;
    //
    // Check column dimensions.
    //
    if (NCOLS < 2) {
        spicelib::SETMSG(b"Grid must have at least two columns but NCOLSS is #.", ctx);
        spicelib::ERRINT(b"#", NCOLS, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDDIMENSION)", ctx)?;
        spicelib::CHKOUT(b"ZZCAPPLT", ctx)?;
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

    for I in 1..=UB {
        if NORTH {
            //
            // Create plates for a north polar cap.
            //
            // Set the vertex index of the north pole.
            //
            TL = POLIDX;
            //
            // Longitude increases with increasing column index.
            //
            if (WRAP && (I == UB)) {
                //
                // Form a plate by connecting the right edge
                // of the surface to the left edge.
                //
                BL = (BASIDX + UB);
                BR = (BASIDX + 1);
            } else {
                BL = (BASIDX + I);
                BR = (BL + 1);
            }

            //
            // Create the current plate.
            //
            *NP = (*NP + 1);

            PLATES[[1, *NP]] = TL;
            PLATES[[2, *NP]] = BL;
            PLATES[[3, *NP]] = BR;
        } else {
            //
            // Create plates for a south polar cap.
            //
            // Set the vertex index of the south pole.
            //
            BL = POLIDX;

            //
            // Longitude increases with increasing column index.
            //
            if (WRAP && (I == UB)) {
                //
                // Form a plate by connecting the right edge
                // of the surface to the left edge.
                //
                TL = (BASIDX + UB);
                TR = (BASIDX + 1);
            } else {
                TL = (BASIDX + I);
                TR = (TL + 1);
            }

            //
            // Create the current plate.
            //
            *NP = (*NP + 1);

            PLATES[[1, *NP]] = BL;
            PLATES[[2, *NP]] = TR;
            PLATES[[3, *NP]] = TL;
        }
    }

    //
    // The plate and vertex counts and arrays have been
    // assigned.
    //
    spicelib::CHKOUT(b"ZZCAPPLT", ctx)?;
    Ok(())
}
