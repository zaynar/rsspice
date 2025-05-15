//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    LENGTH: i32,
    I: i32,
    J: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut LENGTH: i32 = 0;
        let mut I: i32 = 0;
        let mut J: i32 = 0;

        Self { LENGTH, I, J }
    }
}

//$Procedure      M2THNQ ( Find a META/2 qualified @then directive )
pub fn M2THNQ(STRING: &[u8], POSITN: &mut i32, LABEL: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Get the lengtH of the string.
    //
    save.LENGTH = intrinsics::LEN(STRING);

    //
    // See if there is a qualified @then.
    //
    *POSITN = UPTO(STRING, b"@then(", 1);

    if (*POSITN == save.LENGTH) {
        fstr::assign(LABEL, b" ");
    } else {
        spicelib::FNDNWD(STRING, *POSITN, &mut save.I, &mut save.J);

        if (save.J <= (save.I + 6)) {
            *POSITN = save.LENGTH;
            fstr::assign(LABEL, b" ");
        } else {
            fstr::assign(LABEL, fstr::substr(STRING, (save.I + 6)..=(save.J - 1)));
        }
    }
}
