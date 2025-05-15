//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const FPARSZ: i32 = 1;
const SGTIDX: i32 = 1;

//$Procedure      ZZEKMLOC ( EK, return integer metadata location )
pub fn ZZEKMLOC(
    HANDLE: i32,
    SEGNO: i32,
    PAGE: &mut i32,
    BASE: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut NSEG: i32 = 0;
    let mut TBASE: i32 = 0;
    let mut TREE: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Non-SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //
    //
    // Validate the segment number to start out.
    //
    //
    // Get the segment count; validate SEGNO.
    //
    NSEG = EKNSEG(HANDLE, ctx)?;

    //
    // Check out SEGNO.
    //
    if ((SEGNO < 1) || (SEGNO > NSEG)) {
        CHKIN(b"ZZEKMLOC", ctx)?;
        SETMSG(b"Segment number = #; valid range is 1:#.", ctx);
        ERRINT(b"#", SEGNO, ctx);
        ERRINT(b"#", NSEG, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"ZZEKMLOC", ctx)?;
        return Ok(());
    }

    //
    // Find the segment in the segment tree.
    // Obtain the base address of the first integer page.
    //
    TBASE = ZZEKTRBS(1, ctx)?;

    //
    // Look up the head node of the segment tree.
    //
    DASRDI(
        HANDLE,
        (TBASE + SGTIDX),
        (TBASE + SGTIDX),
        std::slice::from_mut(&mut TREE),
        ctx,
    )?;

    //
    // Get the segment pointer for the segment having index SEGNO.
    // This pointer is actually the page number we're looking for.
    //
    ZZEKTRDP(HANDLE, TREE, SEGNO, PAGE, ctx)?;

    //
    // Return the base address of the metadata page as well.
    //
    *BASE = ZZEKTRBS(*PAGE, ctx)?;

    Ok(())
}
