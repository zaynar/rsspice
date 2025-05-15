//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const WDSIZE: i32 = 32;
const LNSIZE: i32 = 1000;

struct SaveVars {
    SYMNAM: Vec<u8>,
    SYMDEF: Vec<u8>,
    SYMVAL: Vec<u8>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SYMNAM = vec![b' '; WDSIZE as usize];
        let mut SYMDEF = vec![b' '; LNSIZE as usize];
        let mut SYMVAL = vec![b' '; LNSIZE as usize];

        Self {
            SYMNAM,
            SYMDEF,
            SYMVAL,
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
pub fn RPTSYM(ID: i32, COMP: i32, STRING: &[u8], WDTH: i32, NAME: &[u8], DEF: &[u8], VALUE: &[u8]) {

    //
    // This routine is a utility for setting and retrieving symbol
    // names, definitions and expanded values.  It is intended that
    // this be used by a higher level routine that fetches symbol
    // definitions one at a time, puts the definition here and
    // passes the routine RETSYM to a formatting routine.
    //
    // The ENTRY point SETSYM allows you to set the symbol and its
    // values.
    //
    // The ENTRY point RETSYM returns the last set values.  To
    // request a portion of a symbol you supply the following
    // values for ID and COMP
    //
    //    1,1 for the symbol name
    //    2,1 for the symbol definition
    //    2,2 or 3,1 for the symbol expanded value.
    //
    // If used with the routine TABRPT you can then easily display
    // symbols as:
    //
    //    name    definition    fully_expanded_value
    //
    // or
    //
    //    name    definition
    //            fully_expanded_value.
    //
}

pub fn SETSYM(NAME: &[u8], DEF: &[u8], VALUE: &[u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    fstr::assign(&mut save.SYMNAM, NAME);
    fstr::assign(&mut save.SYMDEF, DEF);
    fstr::assign(&mut save.SYMVAL, VALUE);
}

pub fn RETSYM(
    ID: i32,
    COMP: i32,
    STRING: &mut [u8],
    WDTH: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    if (ID == 1) {
        if (COMP != 1) {
            fstr::assign(STRING, b" ");
        } else {
            fstr::assign(STRING, &save.SYMNAM);
        }
    } else if (ID == 2) {
        if (COMP == 1) {
            fstr::assign(STRING, &save.SYMDEF);
        } else if (COMP == 2) {
            fstr::assign(STRING, &save.SYMVAL);
        } else {
            fstr::assign(STRING, b" ");
        }
    } else if (ID == 3) {
        if (COMP == 1) {
            fstr::assign(STRING, &save.SYMVAL);
        } else {
            fstr::assign(STRING, b" ");
        }
    }

    *WDTH = spicelib::RTRIM(STRING);

    Ok(())
}
