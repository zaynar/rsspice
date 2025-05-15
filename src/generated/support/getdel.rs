//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    DELIM: Vec<u8>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut DELIM = vec![b' '; 1 as usize];

        fstr::assign(&mut DELIM, b";");

        Self { DELIM }
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
pub fn GETDEL(LETTER: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let LETTER = &mut LETTER[..1 as usize];

    //
    // This is simply a utility routine that sets the special
    // character that is used by the a command loop program to
    // terminate command inputs.  It has a get and set entry
    // points.  The functions should be obvious.  If not you
    // should consider another career.
    //

    fstr::assign(LETTER, &save.DELIM);
}

pub fn SETDEL(LETTER: &[u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let LETTER = &LETTER[..1 as usize];

    fstr::assign(&mut save.DELIM, LETTER);
}
