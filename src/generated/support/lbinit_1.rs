//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
pub const LBCBUF: i32 = 0;

//$Procedure LBINIT ( Line buffer, initialize )
pub fn LBINIT_1(
    PSIZE: i32,
    VDIM: i32,
    PTRS: &mut [i32],
    BUFFER: CharArrayMut,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut PTRS = DummyArrayMut::new(PTRS, LBCELL..=PSIZE);
    let mut BUFFER = DummyCharArrayMut::new(BUFFER, None, LBCBUF..=VDIM);
    let mut MAXLN: i32 = 0;

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
        spicelib::CHKIN(b"LBINIT_1", ctx)?;
    }

    //
    // Initialize the character buffer first.
    //
    CBINIT_1(VDIM, BUFFER.as_arg_mut(), ctx)?;

    //
    // The size must be 4(n+1), where n is the maximum number of
    // lines that can be stored. (The line buffer must be able to
    // store at least one line!)
    //
    // Every line buffer starts out with zero lines and one complement
    // interval, which covers the entire CB.
    //
    MAXLN = ((PSIZE / 4) - 1);

    if (MAXLN < 1) {
        spicelib::SIGERR(b"SPICE(INSUFPTRSIZE)", ctx)?;
    } else {
        spicelib::SSIZEI((4 * (MAXLN + 1)), PTRS.as_slice_mut(), ctx)?;

        PTRS[1] = 1;
        PTRS[2] = SIZECB_1(BUFFER.as_arg(), ctx)?;

        LBUPD_1(0, 1, PTRS.as_slice_mut(), ctx)?;
    }

    spicelib::CHKOUT(b"LBINIT_1", ctx)?;
    Ok(())
}
