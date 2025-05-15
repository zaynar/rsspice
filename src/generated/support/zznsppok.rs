//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure ZZNSPPOK (Private Routine -- NSPIO Port)
pub fn ZZNSPPOK(
    PORT: &[u8],
    NPORTS: i32,
    PORTS: CharArray,
    ctx: &mut Context,
) -> f2rust_std::Result<i32> {
    let PORTS = DummyCharArray::new(PORTS, None, 1..);
    let mut ZZNSPPOK: i32 = 0;
    let mut ID: i32 = 0;

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Find PORT in the PORTS array.
    //
    ID = spicelib::ISRCHC(PORT, NPORTS, PORTS.as_arg());

    //
    // Set ZZNSPPOK to the return value.
    //
    ZZNSPPOK = ID;

    //
    // Check to see if we were able to find the integer ID of PORT.
    // If not, use discovery check in/out and signal an error.
    //
    if (ID == 0) {
        spicelib::CHKIN(b"ZZNSPPOK", ctx)?;
        spicelib::SETMSG(b"$ is an unrecognized port.", ctx);
        spicelib::ERRCH(b"$", PORT, ctx);
        spicelib::SIGERR(b"NSPIO(UNKNOWNPORT)", ctx)?;
        spicelib::CHKOUT(b"ZZNSPPOK", ctx)?;
    }

    Ok(ZZNSPPOK)
}
