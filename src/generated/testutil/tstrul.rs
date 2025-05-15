//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;

//$Procedure      TSTRUL (Test Rules)
//
pub fn TSTRUL(ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut MESSGE = [b' '; LNSIZE as usize];

    //

    //

    if VERBOS(ctx) {
        fstr::assign(
            &mut MESSGE,
            b"----------------------------------------------------------------------- ",
        );

        TSTLOG(&MESSGE, false, ctx)?;
    }

    Ok(())
}
