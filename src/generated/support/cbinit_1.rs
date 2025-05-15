//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCBUF: i32 = 0;

//$Procedure CBINIT ( Character buffer, initialize  )
pub fn CBINIT_1(DIM: i32, BUFFER: CharArrayMut, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut BUFFER = DummyCharArrayMut::new(BUFFER, None, LBCBUF..=DIM);

    //
    // SPICELIB functions
    //

    //
    // Standard error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"CBINIT_1", ctx)?;

        if (intrinsics::LEN(&BUFFER[0]) < 8) {
            spicelib::SETMSG(b"Length is #.", ctx);
            spicelib::ERRINT(b"#", intrinsics::LEN(&BUFFER[0]), ctx);

            spicelib::SIGERR(b"SPICE(NOTLEGALCB)", ctx)?;
            spicelib::CHKOUT(b"CBINIT_1", ctx)?;
            return Ok(());
        } else if (DIM < 1) {
            spicelib::SETMSG(b"Dimension is #.", ctx);
            spicelib::ERRINT(b"#", DIM, ctx);

            spicelib::SIGERR(b"SPICE(NOTLEGALCB)", ctx)?;
            spicelib::CHKOUT(b"CBINIT_1", ctx)?;
            return Ok(());
        }
    }

    //
    // Store only the dimension.
    //
    spicelib::ENCHAR(DIM, fstr::substr_mut(&mut BUFFER[0], 1..=8), ctx)?;

    spicelib::CHKOUT(b"CBINIT_1", ctx)?;
    Ok(())
}
