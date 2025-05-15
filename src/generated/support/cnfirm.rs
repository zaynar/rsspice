//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LINLEN: i32 = 256;

//$Procedure CNFIRM ( Return status of a yes/no query )
pub fn CNFIRM(PRMPT: &[u8], TORF: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut RESPNS = [b' '; LINLEN as usize];
    let mut YESNO: bool = false;

    //
    // SPICELIB functions
    //
    // None.
    //

    //
    // Local Parameters
    //
    //
    // Local Variables
    //

    //
    // Do while we have not gotten a yes/no response
    //
    YESNO = false;
    while !YESNO {
        //
        // Prompt for a response
        //
        spicelib::PROMPT(PRMPT, &mut RESPNS, ctx)?;
        //
        // Left justify the response string, RESPNS, and convert it to
        // uppercase.
        //
        spicelib::LJUST(&RESPNS.clone(), &mut RESPNS);
        spicelib::UCASE(&RESPNS.clone(), &mut RESPNS, ctx);

        if (fstr::eq(&RESPNS, b"Y") || fstr::eq(&RESPNS, b"YES")) {
            *TORF = true;
            YESNO = true;
        } else if (fstr::eq(&RESPNS, b"N") || fstr::eq(&RESPNS, b"NO")) {
            *TORF = false;
            YESNO = true;
        }
    }

    Ok(())
}
