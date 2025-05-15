//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure ZZHASHI ( Private---integer hash function )
pub fn ZZHASHI(N: i32, M: i32, ctx: &mut Context) -> f2rust_std::Result<i32> {
    let mut ZZHASHI: i32 = 0;

    //
    // Check divisor.
    //
    if (M <= 0) {
        ZZHASHI = 0;

        CHKIN(b"ZZHASHI", ctx)?;
        SETMSG(
            b"The input hash function divisor was not a positive number. It was #.",
            ctx,
        );
        ERRINT(b"#", M, ctx);
        SIGERR(b"SPICE(INVALIDDIVISOR)", ctx)?;
        CHKOUT(b"ZZHASHI", ctx)?;
        return Ok(ZZHASHI);
    }

    //
    // Use simple division method -- h(k) = k mod m.
    //
    ZZHASHI = (intrinsics::MOD(i32::abs(N), M) + 1);

    Ok(ZZHASHI)
}
