//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    LSTYLE: Vec<u8>,
    SSTYLE: Vec<u8>,
    MARGIN: Vec<u8>,
    SCRSTT: StackArray<bool, 3>,
    SAVSTT: StackArray<bool, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut LSTYLE = vec![b' '; 128 as usize];
        let mut SSTYLE = vec![b' '; 128 as usize];
        let mut MARGIN = vec![b' '; 128 as usize];
        let mut SCRSTT = StackArray::<bool, 3>::new(1..=3);
        let mut SAVSTT = StackArray::<bool, 3>::new(1..=3);

        fstr::assign(&mut LSTYLE, b"LEFT 1 RIGHT 78");
        fstr::assign(&mut SSTYLE, b"LEFT 1 RIGHT 78");

        Self {
            LSTYLE,
            SSTYLE,
            MARGIN,
            SCRSTT,
            SAVSTT,
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
pub fn NSPXCP(STRING: &[u8], ERROR: CharArray, SCREEN: &[u8], LOGFIL: &[u8]) {

    //
}

pub fn NSPERR(STRING: &[u8], ERROR: CharArrayMut, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ERROR = DummyCharArrayMut::new(ERROR, None, 1..=2);

    if fstr::ne(ERROR.get(1), b" ") {
        PRCLR(ctx)?;

        NSPIOH(b"LOG", ctx)?;

        NSPMRG(&mut save.MARGIN, ctx);
        spicelib::SUFFIX(&save.SSTYLE, 1, &mut save.MARGIN);
        NICEPR_1(&ERROR[1], &save.MARGIN, NSPWLN, ctx)?;

        NSPGST(b"SCREEN", save.SCRSTT.as_slice_mut(), ctx)?;
        NSPGST(b"SAVE", save.SAVSTT.as_slice_mut(), ctx)?;

        NSPIOH(b"SCREEN", ctx)?;
        NSPIOH(b"SAVE", ctx)?;

        NSPIOA(b"LOG", ctx)?;

        for I in 1..=2 {
            NICEPR_1(&ERROR[I], &save.LSTYLE, NSPWLN, ctx)?;
        }

        NSPPST(b"SCREEN", save.SCRSTT.as_slice(), ctx)?;
        NSPPST(b"SAVE", save.SAVSTT.as_slice(), ctx)?;

        fstr::assign(ERROR.get_mut(1), b" ");
        return Ok(());
    }

    if fstr::eq(STRING, b"?") {
        if fstr::eq(ERROR.get(2), b" ") {
            TRNLAT(b"NOMOREDIAGNOSTICS", &mut ERROR[2], ctx);
        }
        NSPIOH(b"LOG", ctx)?;

        NSPMRG(&mut save.MARGIN, ctx);
        spicelib::SUFFIX(&save.SSTYLE, 1, &mut save.MARGIN);
        NICEPR_1(&ERROR[2], &save.MARGIN, NSPWLN, ctx)?;
        NSPIOA(b"LOG", ctx)?;

        fstr::assign(ERROR.get_mut(2), b" ");
    }

    Ok(())
}

pub fn NSPSTY(SCREEN: &[u8], LOGFIL: &[u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    fstr::assign(&mut save.SSTYLE, SCREEN);
    fstr::assign(&mut save.LSTYLE, LOGFIL);
}
