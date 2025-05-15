//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
pub const LBCBUF: i32 = 0;

//$Procedure LBGET ( Line buffer, get )
pub fn LBGET_1(
    POS: i32,
    PTRS: &[i32],
    BUFFER: CharArray,
    LINE: &mut [u8],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let PTRS = DummyArray::new(PTRS, LBCELL..);
    let BUFFER = DummyCharArray::new(BUFFER, None, LBCBUF..);
    let mut MAXLN: i32 = 0;
    let mut NLINE: i32 = 0;
    let mut NCOM: i32 = 0;
    let mut PCARD: i32 = 0;
    let mut POSPTR: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Standard error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"LBGET_1", ctx)?;
    }

    //
    // Recover all the essential control information.
    //
    LBDES_1(
        PTRS.as_slice(),
        &mut MAXLN,
        &mut NLINE,
        &mut NCOM,
        &mut PCARD,
        ctx,
    )?;

    //
    // What are the endpoints of the stored line? Once we have
    // them, we can return the line directly.
    //
    *FOUND = ((POS >= 1) && (POS <= NLINE));

    if *FOUND {
        POSPTR = ((2 * POS) - 1);

        CBGET_1(PTRS[POSPTR], PTRS[(POSPTR + 1)], BUFFER.as_arg(), LINE, ctx)?;
    }

    spicelib::CHKOUT(b"LBGET_1", ctx)?;
    Ok(())
}
