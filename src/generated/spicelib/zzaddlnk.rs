//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      ZZADDLNK ( Add a new AB cell to an AB structure )
pub fn ZZADDLNK(
    AVAL: i32,
    BVAL: i32,
    MAXA: i32,
    CELLSZ: i32,
    PNTRS: &mut [i32],
    NCELL: &mut i32,
    CELLS: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut PNTRS = DummyArrayMut::new(PNTRS, 1..);
    let mut CELLS = DummyArrayMut2D::new(CELLS, 1..=2, 1..);

    //
    // SPICE functions
    //

    //
    // Standard RETURN test.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZADDLNK", ctx)?;

    //
    // Test the pointer array index AVAL.
    //
    if ((AVAL < 1) || (AVAL > MAXA)) {
        SETMSG(
            b"Index AVAL is out of range. AVAL = #1; valid range is 1:#2.",
            ctx,
        );
        ERRINT(b"#1", AVAL, ctx);
        ERRINT(b"#2", MAXA, ctx);
        SIGERR(b"SPICE(AVALOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZADDLNK", ctx)?;
        return Ok(());
    }

    //
    // Increment the cell counter.
    //
    *NCELL = (*NCELL + 1);

    if (*NCELL > CELLSZ) {
        SETMSG(
            b"NCELL larger than cell array. Cell index = #1. Array size = #2.",
            ctx,
        );
        ERRINT(b"#1", *NCELL, ctx);
        ERRINT(b"#2", CELLSZ, ctx);
        SIGERR(b"SPICE(CELLARRAYTOOSMALL)", ctx)?;
        CHKOUT(b"ZZADDLNK", ctx)?;
        return Ok(());
    }

    //
    // Update the cell address of the last occurrence of the A-value,
    // if any. If none, PNTRS(AVAL) has value -1.
    //
    CELLS[[1, *NCELL]] = BVAL;
    CELLS[[2, *NCELL]] = PNTRS[AVAL];

    PNTRS[AVAL] = *NCELL;

    CHKOUT(b"ZZADDLNK", ctx)?;
    Ok(())
}
