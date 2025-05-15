//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      ZZINILNK ( Initialize an AB cell linked-list )
pub fn ZZINILNK(
    MAXP: i32,
    MAXC: i32,
    NCELL: &mut i32,
    PNTRS: &mut [i32],
    CELLS: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut PNTRS = DummyArrayMut::new(PNTRS, 1..);
    let mut CELLS = DummyArrayMut2D::new(CELLS, 1..=2, 1..);

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZINILNK", ctx)?;

    if (MAXP < 1) {
        SETMSG(b"Pointer array size MAXP = #; size must be positive.", ctx);
        ERRINT(b"#", MAXP, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZINILNK", ctx)?;
        return Ok(());
    }

    if (MAXC < MAXP) {
        SETMSG(
            b"Cell array size MAXC = #; size must be at least as large as pointer array size #.",
            ctx,
        );
        ERRINT(b"#", MAXC, ctx);
        ERRINT(b"#", MAXP, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZINILNK", ctx)?;
        return Ok(());
    }

    //
    // Initialize pointer array and cells.
    //
    for I in 1..=MAXP {
        PNTRS[I] = -1;
    }

    for I in 1..=MAXC {
        CELLS[[1, I]] = 0;
        CELLS[[2, I]] = -1;
    }

    //
    // Set count of cells in use to 0, for the convenience
    // of the calling routine.
    //
    *NCELL = 0;

    CHKOUT(b"ZZINILNK", ctx)?;
    Ok(())
}
