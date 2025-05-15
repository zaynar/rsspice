//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const FILSIZ: i32 = 255;
const LNSIZE: i32 = 80;

struct SaveVars {
    MYVER: Vec<u8>,
    MYENV: Vec<u8>,
    MYTIME: Vec<u8>,
    MYFILE: Vec<u8>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut MYVER = vec![b' '; LNSIZE as usize];
        let mut MYENV = vec![b' '; LNSIZE as usize];
        let mut MYTIME = vec![b' '; LNSIZE as usize];
        let mut MYFILE = vec![b' '; FILSIZ as usize];

        fstr::assign(&mut MYVER, b" ");
        fstr::assign(&mut MYENV, b" ");
        fstr::assign(&mut MYTIME, b" ");
        fstr::assign(&mut MYFILE, b" ");

        Self {
            MYVER,
            MYENV,
            MYTIME,
            MYFILE,
        }
    }
}

//$Procedure      TSTSAV ( Save test info )
//
pub fn TSTSAV(ENV: &[u8], VERSN: &[u8], TIME: &[u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //

    fstr::assign(&mut save.MYENV, ENV);
    fstr::assign(&mut save.MYVER, VERSN);
    fstr::assign(&mut save.MYTIME, TIME);
}

//
// The entry point below fetches the saved values of ENV, VERSN,
// and TIME.
//
pub fn TSTGET(ENV: &mut [u8], VERSN: &mut [u8], TIME: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    fstr::assign(ENV, &save.MYENV);
    fstr::assign(VERSN, &save.MYVER);
    fstr::assign(TIME, &save.MYTIME);
}

//
// This entry point allows you to save the name of the test log file.
//
pub fn TSTSLF(ENV: &[u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    fstr::assign(&mut save.MYFILE, ENV);
}

//
// This entry point allows you to retrieve the name of the test log
// file.
//
pub fn TSTGLF(ENV: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    fstr::assign(ENV, &save.MYFILE);
}
