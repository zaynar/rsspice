//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

//$Procedure LBDES ( Line buffer, describe )
pub fn LBDES_1(
    PTRS: &[i32],
    MAXLN: &mut i32,
    NLINE: &mut i32,
    NCOM: &mut i32,
    PCARD: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let PTRS = DummyArray::new(PTRS, LBCELL..);
    let mut PSIZE: i32 = 0;

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
        spicelib::CHKIN(b"LBDES_1", ctx)?;
    }

    //
    // Recover some information directly.
    //
    PSIZE = spicelib::SIZEI(PTRS.as_slice(), ctx)?;
    *PCARD = spicelib::CARDI(PTRS.as_slice(), ctx)?;
    *NLINE = PTRS[-2];

    //
    // Infer the rest.
    //
    *MAXLN = ((PSIZE / 4) - 1);
    *NCOM = ((*PCARD / 2) - *NLINE);

    spicelib::CHKOUT(b"LBDES_1", ctx)?;
    Ok(())
}
