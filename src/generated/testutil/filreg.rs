//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const MAXFIL: i32 = 100;
const FILSIZ: i32 = 255;

struct SaveVars {
    FILES: ActualCharArray,
    NFILE: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FILES = ActualCharArray::new(FILSIZ, 1..=MAXFIL);
        let mut NFILE: i32 = 0;

        NFILE = 0;

        Self { FILES, NFILE }
    }
}

//$Procedure      FILREG ( File Registry )
pub fn FILREG(FILE: &[u8]) {}

//
// The next entry point registers a file.
//
pub fn TFILES(FILE: &[u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Don't register files when we are in DEBUGGING mode.
    //
    if ISON(b"DEBUGGING", ctx) {
        return;
    }

    if (save.NFILE < MAXFIL) {
        save.NFILE = (save.NFILE + 1);
        fstr::assign(save.FILES.get_mut(save.NFILE), FILE);
    }
}

//
// This entry point deletes all registered files.
//
pub fn KFILES(ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut R: i32 = 0;

    for I in 1..=save.NFILE {
        R = spicelib::RTRIM(&save.FILES[I]);
        KILFIL(fstr::substr(&save.FILES[I], 1..=R), ctx)?;
    }

    save.NFILE = 0;

    Ok(())
}
