//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const MXPGWD: i32 = 131;
const WDSIZE: i32 = 16;

struct SaveVars {
    STRLFT: Vec<u8>,
    STRRHT: Vec<u8>,
    MYLEFT: i32,
    MYRGHT: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut STRLFT = vec![b' '; WDSIZE as usize];
        let mut STRRHT = vec![b' '; WDSIZE as usize];
        let mut MYLEFT: i32 = 0;
        let mut MYRGHT: i32 = 0;

        MYLEFT = 1;
        MYRGHT = 80;

        Self {
            STRLFT,
            STRRHT,
            MYLEFT,
            MYRGHT,
        }
    }
}

//$Procedure      NSPPWD ( NSP --- Page width)
pub fn NSPPWD(MARGIN: &[u8], LEFT: i32, RIGHT: i32) {}

pub fn NSPMRG(MARGIN: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Return the current margins to be used by the NICEIO and NICEPR
    // routines.
    //

    spicelib::INTSTR(save.MYLEFT, &mut save.STRLFT, ctx);
    spicelib::INTSTR(save.MYRGHT, &mut save.STRRHT, ctx);

    fstr::assign(MARGIN, b"LEFT");

    spicelib::SUFFIX(&save.STRLFT, 1, MARGIN);
    spicelib::SUFFIX(b"RIGHT", 1, MARGIN);
    spicelib::SUFFIX(&save.STRRHT, 1, MARGIN);
}

pub fn NSPSLR(LEFT: i32, RIGHT: i32, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Set the left and right margins to be used when creating
    // margin style strings in the entry point above.  Note
    // we force these to be reasonable.  No error checking is
    // done.
    //
    save.MYLEFT = intrinsics::MAX0(&[1, intrinsics::MIN0(&[LEFT, RIGHT, (MXPGWD - 2)])]);
    save.MYRGHT = intrinsics::MIN0(&[MXPGWD, intrinsics::MAX0(&[LEFT, RIGHT, (save.MYLEFT + 2)])]);
}

pub fn NSPGLR(LEFT: &mut i32, RIGHT: &mut i32, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Get the left and right margins that are currently
    // being used.
    //

    *LEFT = save.MYLEFT;
    *RIGHT = save.MYRGHT;
}
