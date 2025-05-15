//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CMDLEN: i32 = 255;

//$Procedure     EDTCMD ( Edit a file using a specified text editor )
pub fn EDTCMD(CMD: &[u8], FILE: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut LOCCMD = [b' '; CMDLEN as usize];

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"EDTCMD", ctx)?;
    }

    //
    //
    // SUN:
    //
    //    Computer:         Sun SPARCstation 2
    //    Operating System: Sun OS 4.1.2
    //    Fortran:          Sun FORTRAN 1.3.1
    //
    // HP:
    //
    //    Computer:         HP 715/50
    //    Operating System: HP-UX 9.01
    //    Fortran:          HP-UX.09.00.24
    //                      HP-UX FORTRAN/9000
    //                         Series 700 B2408A.09.00
    //                         Series 800 B2409B.09.00
    //
    // NEXT:
    //
    //    Computer:         NeXT
    //    Operating System: NeXtStep 3.0, 3.2
    //    Fortran:          Absoft Fortran V3.2
    //    NEXT (NeXT 3.0, Absoft Fortran 3.2):
    //
    //    Computer:         Alpha/OSF1
    //    Operating System: OSF1 V3.2
    //    Fortran:        : DEC Fortran Compiler Driver V3.5-053
    //
    //
    // Build the edit command to be passed to the system.
    //
    fstr::assign(&mut LOCCMD, CMD);
    spicelib::SUFFIX(FILE, 1, &mut LOCCMD);

    //
    // For safety, append a null to the command.  If the user has
    // linked to a C version of the "system" function, this may
    // save us much grief.
    //
    spicelib::SUFFIX(&intrinsics::CHAR(0), 0, &mut LOCCMD);

    //
    // Invoke the editor.
    //
    EXESYS(fstr::substr(&LOCCMD, 1..=spicelib::RTRIM(&LOCCMD)), ctx)?;

    spicelib::CHKOUT(b"EDTCMD", ctx)?;
    Ok(())
}
