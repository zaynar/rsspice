//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure ZZTRVLNK ( Traverse AB cell linked-list )
pub fn ZZTRVLNK(
    AVAL: i32,
    MAXA: i32,
    PNTRS: &[i32],
    CELLSZ: i32,
    CELLS: &[i32],
    MAXB: i32,
    NB: &mut i32,
    BLIST: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let PNTRS = DummyArray::new(PNTRS, 1..);
    let CELLS = DummyArray2D::new(CELLS, 1..=2, 1..);
    let mut BLIST = DummyArrayMut::new(BLIST, 1..);
    let mut I: i32 = 0;
    let mut PTR: i32 = 0;

    //
    // Standard SPICE error handling.
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZTRVLNK", ctx)?;

    if ((AVAL < 1) || (AVAL > MAXA)) {
        SETMSG(
            b"Index AVAL is out of range. Index = #1. Valid range = 1:#2.",
            ctx,
        );
        ERRINT(b"#1", AVAL, ctx);
        ERRINT(b"#2", MAXA, ctx);
        SIGERR(b"SPICE(INDEXOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZTRVLNK", ctx)?;
        return Ok(());
    }

    if (MAXB < 1) {
        SETMSG(b"Maximum output list size MAXB is invalid. MAXB = #1.", ctx);
        ERRINT(b"#1", MAXB, ctx);
        SIGERR(b"SPICE(INVALIDSIZE)", ctx)?;
        CHKOUT(b"ZZTRVLNK", ctx)?;
        return Ok(());
    }

    *NB = 0;
    BLIST[1] = 0;

    PTR = PNTRS[AVAL];

    while (PTR != -1) {
        if (((PTR < -1) || (PTR == 0)) || (PTR > CELLSZ)) {
            SETMSG(b"Value in PNTRS array is not a valid index in the cell array.Value = #1. Array size = #2.", ctx);
            ERRINT(b"#1", PTR, ctx);
            ERRINT(b"#2", CELLSZ, ctx);
            SIGERR(b"SPICE(POINTEROUTOFRANGE)", ctx)?;
            CHKOUT(b"ZZTRVLNK", ctx)?;
            return Ok(());
        }

        *NB = (*NB + 1);

        if (*NB > MAXB) {
            SETMSG(b"Output value count is larger than B-list array room. Count = #1. Output array room = #2. Input pointer index was #3. Input pointer list size was #4. Last pointer was #5. Cell size was #6.", ctx);
            ERRINT(b"#1", *NB, ctx);
            ERRINT(b"#2", MAXB, ctx);
            ERRINT(b"#3", AVAL, ctx);
            ERRINT(b"#4", MAXA, ctx);
            ERRINT(b"#5", PTR, ctx);
            ERRINT(b"#6", CELLSZ, ctx);
            SIGERR(b"SPICE(BARRAYTOOSMALL)", ctx)?;
            CHKOUT(b"ZZTRVLNK", ctx)?;
            return Ok(());
        }

        BLIST[*NB] = CELLS[[1, PTR]];
        I = CELLS[[2, PTR]];
        PTR = I;
    }

    //
    // Standard SPICE error handling.
    //
    CHKOUT(b"ZZTRVLNK", ctx)?;
    Ok(())
}
