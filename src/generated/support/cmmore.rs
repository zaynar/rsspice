//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const WDSIZE: i32 = 32;

struct SaveVars {
    CMMORE: bool,
    EXIT: Vec<u8>,
    LC: i32,
    R: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CMMORE: bool = false;
        let mut EXIT = vec![b' '; WDSIZE as usize];
        let mut LC: i32 = 0;
        let mut R: i32 = 0;
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            CMMORE,
            EXIT,
            LC,
            R,
            FIRST,
        }
    }
}

//$Procedure      CMMORE ( Command Loop---More Commands)
pub fn CMMORE(COMMND: &[u8], ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    // SPICELIB Functions
    //

    //
    // Local Variables.
    //

    //
    // On the first pass we fetch the "exit" command and
    // spruce it up a bit for use when comparing with
    // the input command.
    //
    if save.FIRST {
        save.FIRST = false;

        TRNLAT(b"EXIT", &mut save.EXIT, ctx);
        spicelib::CMPRSS(b" ", 1, &save.EXIT.to_vec(), &mut save.EXIT);
        spicelib::LJUST(&save.EXIT.to_vec(), &mut save.EXIT);
        save.R = spicelib::RTRIM(&save.EXIT);
    }
    //
    // If the input command is shorter than the non-blank
    // length of EXIT, then this cannot be the exit command.
    // There is more to do.
    //
    // Note we assign a value to CMMORE so that the compiler
    // won't have a fit about having a function unassigned.
    // The if conditions below ensure that we assign a value
    // but most compilers aren't smart enough to figure that
    // out.
    //
    save.CMMORE = true;
    save.LC = intrinsics::LEN(COMMND);

    if (save.LC < save.R) {
        save.CMMORE = true;
        return save.CMMORE;
    }

    //
    // Check to see if the input command matches the exit command.
    // We do this a character at a time.  We search from the
    // left to right, because most commands are not EXIT and this
    // allows us to quit early in the process.
    //
    for I in 1..=save.R {
        if spicelib::NECHR(
            fstr::substr(COMMND, I..=I),
            fstr::substr(&save.EXIT, I..=I),
            ctx,
        ) {
            save.CMMORE = true;
            return save.CMMORE;
        }
    }

    //
    // It's looking like this might be it.  See if the rest of
    // the input command is blank.
    //
    if (save.LC == save.R) {
        //
        // We've got an exact match.  There are no more commands
        // to look at.
        //
        save.CMMORE = false;
    } else if (save.LC > save.R) {
        //
        // There will be more commands only if the rest of the input
        // command is non-blank.
        //
        save.CMMORE = fstr::ne(fstr::substr(COMMND, (save.R + 1)..), b" ");
    }

    save.CMMORE
}
