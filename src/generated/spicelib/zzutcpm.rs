//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;

//$Procedure      ZZUTCPM ( UTC Plus or Minus Parse )
pub fn ZZUTCPM(
    STRING: &[u8],
    START: i32,
    HOFF: &mut f64,
    MOFF: &mut f64,
    LAST: &mut i32,
    SUCCES: &mut bool,
    ctx: &mut Context,
) {
    let mut ERROR = [b' '; LNSIZE as usize];
    let mut NEED: i32 = 0;
    let mut LENGTH: i32 = 0;
    let mut SIGNAT: i32 = 0;
    let mut UNSAT: i32 = 0;
    let mut UNSTO: i32 = 0;
    let mut NCHAR: i32 = 0;
    let mut PTR: i32 = 0;
    let mut SIGN: f64 = 0.0;
    let mut X: f64 = 0.0;

    //
    // Spicelib functions
    //
    //
    // Local Variables
    //

    //
    // This is a special purpose routine.  The input string must have
    // exactly the right format to be a time zone substring.  If anything
    // goes wrong, we just bail out and leave HOFF and MOFF right at
    // zero.
    //
    *HOFF = 0.0;
    *MOFF = 0.0;
    *LAST = (START - 1);
    *SUCCES = false;

    //
    // Note that NEED   = START + LEN('::UTC+x') - 1
    //           SIGNAT = START + LEN('::UTC+' ) - 1
    //
    LENGTH = intrinsics::LEN(STRING);
    NEED = (START + 6);
    SIGNAT = (START + 5);
    UNSAT = NEED;

    if (LENGTH < NEED) {
        return;
    }

    if (intrinsics::ICHAR(fstr::substr(STRING, SIGNAT..=SIGNAT)) == intrinsics::ICHAR(b"+")) {
        SIGN = 1.0;
    } else if (intrinsics::ICHAR(fstr::substr(STRING, SIGNAT..=SIGNAT)) == intrinsics::ICHAR(b"-"))
    {
        SIGN = -1.0;
    } else {
        return;
    }
    //
    // So far everything looks fine, "lex" the string starting at
    // SIGNAT + 1 for an unsigned integer.
    //
    LX4UNS(STRING, UNSAT, &mut UNSTO, &mut NCHAR, ctx);

    if ((NCHAR > 0) && (NCHAR < 3)) {
        NPARSD(
            fstr::substr(STRING, UNSAT..=UNSTO),
            &mut X,
            &mut ERROR,
            &mut PTR,
            ctx,
        );

        if (X >= 13.0) {
            return;
        }

        *LAST = UNSTO;
        *HOFF = (SIGN * X);
    } else {
        return;
    }
    //
    // If we're still in the game at this point, we have at least
    // an hour offset, see if there is a minutes portion to the
    // time zone.
    //
    *SUCCES = true;

    if SAMCH(STRING, (UNSTO + 1), b":", 1) {
        UNSAT = (UNSTO + 2);
    } else {
        return;
    }

    LX4UNS(STRING, UNSAT, &mut UNSTO, &mut NCHAR, ctx);

    if ((NCHAR > 0) && (NCHAR < 3)) {
        NPARSD(
            fstr::substr(STRING, UNSAT..=UNSTO),
            &mut X,
            &mut ERROR,
            &mut PTR,
            ctx,
        );

        if (X > 59.0) {
            return;
        }

        *LAST = UNSTO;
        *MOFF = (SIGN * X);
    }
}
