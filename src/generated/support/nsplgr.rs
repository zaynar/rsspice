//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXCOM: i32 = 20;
const FILEN: i32 = 128;
const COMSIZ: i32 = 1024;
const ERRSIZ: i32 = 1760;
const STYSIZ: i32 = 120;
const LNGSIZ: i32 = (COMSIZ + 1);

struct SaveVars {
    MYSTR: Vec<u8>,
    SEEN: Vec<u8>,
    HIDE: Vec<u8>,
    DELIM: Vec<u8>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut MYSTR = vec![b' '; LNGSIZ as usize];
        let mut SEEN = vec![b' '; STYSIZ as usize];
        let mut HIDE = vec![b' '; STYSIZ as usize];
        let mut DELIM = vec![b' '; 1 as usize];

        fstr::assign(&mut MYSTR, b" ");
        fstr::assign(&mut SEEN, b"LEFT 1 RIGHT 78");
        fstr::assign(&mut HIDE, b"LEADER ;^ LEFT 1 RIGHT 78 HARDSPACE ^");
        fstr::assign(&mut DELIM, b";");

        Self {
            MYSTR,
            SEEN,
            HIDE,
            DELIM,
        }
    }
}

//$ Disclaimer
//
//     THIS SOFTWARE AND ANY RELATED MATERIALS WERE CREATED BY THE
//     CALIFORNIA INSTITUTE OF TECHNOLOGY (CALTECH) UNDER A U.S.
//     GOVERNMENT CONTRACT WITH THE NATIONAL AERONAUTICS AND SPACE
//     ADMINISTRATION (NASA). THE SOFTWARE IS TECHNOLOGY AND SOFTWARE
//     PUBLICLY AVAILABLE UNDER U.S. EXPORT LAWS AND IS PROVIDED "AS-IS"
//     TO THE RECIPIENT WITHOUT WARRANTY OF ANY KIND, INCLUDING ANY
//     WARRANTIES OF PERFORMANCE OR MERCHANTABILITY OR FITNESS FOR A
//     PARTICULAR USE OR PURPOSE (AS SET FORTH IN UNITED STATES UCC
//     SECTIONS 2312-2313) OR FOR ANY PURPOSE WHATSOEVER, FOR THE
//     SOFTWARE AND RELATED MATERIALS, HOWEVER USED.
//
//     IN NO EVENT SHALL CALTECH, ITS JET PROPULSION LABORATORY, OR NASA
//     BE LIABLE FOR ANY DAMAGES AND/OR COSTS, INCLUDING, BUT NOT
//     LIMITED TO, INCIDENTAL OR CONSEQUENTIAL DAMAGES OF ANY KIND,
//     INCLUDING ECONOMIC DAMAGE OR INJURY TO PROPERTY AND LOST PROFITS,
//     REGARDLESS OF WHETHER CALTECH, JPL, OR NASA BE ADVISED, HAVE
//     REASON TO KNOW, OR, IN FACT, SHALL KNOW OF THE POSSIBILITY.
//
//     RECIPIENT BEARS ALL RISK RELATING TO QUALITY AND PERFORMANCE OF
//     THE SOFTWARE AND ANY RELATED MATERIALS, AND AGREES TO INDEMNIFY
//     CALTECH AND NASA FOR ALL THIRD-PARTY CLAIMS RESULTING FROM THE
//     ACTIONS OF RECIPIENT IN THE USE OF THE SOFTWARE.
//
pub fn NSPLG(COMMND: &[u8], HIDDEN: bool, VSTYLE: &[u8], HSTYLE: &[u8], CDELIM: &[u8]) {

    //
}

pub fn NSPLOG(COMMND: &[u8], HIDDEN: bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    fstr::assign(&mut save.MYSTR, COMMND);
    NSPIOH(b"SCREEN", ctx)?;
    NSPIOA(b"LOG", ctx)?;

    if HIDDEN {
        NICEPR_1(COMMND, &save.HIDE, NSPWLN, ctx)?;
    } else {
        fstr::assign(&mut save.MYSTR, COMMND);
        spicelib::SUFFIX(&save.DELIM, 0, &mut save.MYSTR);
        NICEPR_1(&save.MYSTR, &save.SEEN, NSPWLN, ctx)?;
    }
    NSPIOA(b"SCREEN", ctx)?;
    NSPIOH(b"LOG", ctx)?;

    Ok(())
}

pub fn NSPLGS(VSTYLE: &[u8], HSTYLE: &[u8], CDELIM: &[u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    fstr::assign(&mut save.SEEN, VSTYLE);
    fstr::assign(&mut save.HIDE, HSTYLE);
    fstr::assign(&mut save.DELIM, CDELIM);
}

pub fn NSPGLS(VSTYLE: &mut [u8], HSTYLE: &mut [u8], CDELIM: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    fstr::assign(VSTYLE, &save.SEEN);
    fstr::assign(HSTYLE, &save.HIDE);
    fstr::assign(CDELIM, &save.DELIM);
}
