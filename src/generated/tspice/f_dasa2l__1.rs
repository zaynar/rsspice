//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LBCELL: i32 = -5;
const WIDTH: i32 = 50;
const DEPTH: i32 = 30;

struct SaveVars {
    SVPATH: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SVPATH = ActualCharArray::new(WIDTH, LBCELL..=DEPTH);

        Self { SVPATH }
    }
}

//**********************************************************************
//
//     P A T H   T E S T   U T I L I T I E S
//
//**********************************************************************

//
// Utilities for execution path checking:
//
pub fn T_PATH(STR: &[u8], PATH: CharArray) {}

//
// Initialize path.
//
pub fn T_PTHNEW(ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    spicelib::SSIZEC(DEPTH, save.SVPATH.as_arg_mut(), ctx)?;

    Ok(())
}

//
// Append action to path.
//
pub fn T_PTHAPP(STR: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    spicelib::APPNDC(STR, save.SVPATH.as_arg_mut(), ctx)?;

    Ok(())
}

//
// Get stored path.
//
pub fn T_PTHGET(PATH: CharArrayMut, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut PATH = DummyCharArrayMut::new(PATH, None, LBCELL..);

    spicelib::COPYC(save.SVPATH.as_arg(), PATH.as_arg_mut(), ctx)?;

    Ok(())
}
