//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXLEN: i32 = 132;
const WDSIZE: i32 = 32;
const LNSIZE: i32 = 2000;

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
pub fn SHOSYM(TEMPLT: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut MYLINE = [b' '; MAXLEN as usize];
    let mut LINE = [b' '; MAXLEN as usize];
    let mut MESSGE = [b' '; MAXLEN as usize];
    let mut REST = [b' '; MAXLEN as usize];
    let mut NAME = [b' '; WDSIZE as usize];
    let mut FRSTWD = [b' '; WDSIZE as usize];
    let mut DEF = [b' '; LNSIZE as usize];
    let mut VALUE = [b' '; LNSIZE as usize];
    let mut PRESRV = StackArray::<bool, 3>::new(1..=3);
    let mut LMARGE: i32 = 0;
    let mut SPACE = StackArray::<i32, 3>::new(1..=3);
    let mut SPCIAL = ActualCharArray::new(1, 1..=3);
    let mut JUSTR = StackArray::<bool, 3>::new(1..=3);
    let mut WIDTH = StackArray::<i32, 3>::new(1..=3);
    let mut SIZE = StackArray::<i32, 3>::new(1..=3);
    let mut ITEM = StackArray::<i32, 3>::new(1..=3);
    let mut NCOL: i32 = 0;
    let mut N: i32 = 0;
    let mut R: i32 = 0;
    let mut PAGEWD: i32 = 0;
    let mut MARGIN = [b' '; WDSIZE as usize];
    let mut TRAN: bool = false;

    R = spicelib::RTRIM(TEMPLT);

    SYMPAT(fstr::substr(TEMPLT, 1..=R), ctx);
    SYMGET(&mut NAME, &mut DEF, ctx)?;

    NSPMRG(&mut MARGIN, ctx);

    if fstr::eq(&NAME, b" ") {
        fstr::assign(
            &mut MESSGE,
            b"There are no symbols that match the template \"#\".",
        );
        spicelib::REPMC(
            &MESSGE.clone(),
            b"#",
            fstr::substr(TEMPLT, 1..=R),
            &mut MESSGE,
        );
        NICEPR_1(&MESSGE, &MARGIN, NSPWLN, ctx)?;
        return Ok(());
    }

    //
    // If still here there are some matching symbols.  Set up the
    // standard defaults.
    //
    fstr::assign(&mut LINE, b"========================================================================================================================================================================");

    PRESRV[1] = true;
    PRESRV[2] = true;
    PRESRV[3] = true;

    LMARGE = 1;

    SPACE[1] = 2;
    SPACE[2] = 2;
    SPACE[3] = 2;

    fstr::assign(SPCIAL.get_mut(1), b" ");
    fstr::assign(SPCIAL.get_mut(2), b" ");
    fstr::assign(SPCIAL.get_mut(3), b" ");

    JUSTR[1] = false;
    JUSTR[2] = false;
    JUSTR[3] = false;
    //
    // Get the width of the page and based upon that determine
    // the basic table style that will be used to display the
    // symbol definition.
    //
    NSPGLR(&mut N, &mut PAGEWD, ctx);

    WIDTH[1] = 14;
    WIDTH[2] = 30;
    WIDTH[3] = 30;

    SIZE[1] = 1;
    SIZE[2] = 1;
    SIZE[3] = 1;

    ITEM[1] = 1;
    ITEM[2] = 2;
    ITEM[3] = 3;

    NCOL = 3;
    //
    // Adjust all of the columns
    //
    for I in 1..=NCOL {
        WIDTH[I] = ((WIDTH[I] * PAGEWD) / 80);
    }

    PAGEWD = 0;

    for I in 1..=NCOL {
        PAGEWD = ((WIDTH[I] + SPACE[I]) + PAGEWD);
    }

    PAGEWD = (PAGEWD - SPACE[NCOL]);

    NSPWLN(b" ", ctx)?;
    NSPWLN(b"Symbols Matching Request: ", ctx)?;
    NSPWLN(b" ", ctx)?;

    PAGRST(ctx);
    PAGSET(b"PAGEWIDTH", PAGEWD, ctx);
    PAGSCN(b"BODY", ctx);

    SETSYM(b"Symbol Name", b"Definition", b"Expanded Value", ctx);
    TABRPT(
        NCOL,
        ITEM.as_slice(),
        SIZE.as_slice(),
        WIDTH.as_slice(),
        JUSTR.as_slice(),
        PRESRV.as_slice(),
        SPCIAL.as_arg(),
        LMARGE,
        *SPACE.first(),
        RETSYM,
        ctx,
    )?;

    fstr::assign(&mut MYLINE, fstr::substr(&LINE, 1..=PAGEWD));

    NSPWLN(&MYLINE, ctx)?;

    while fstr::ne(&NAME, b" ") {
        //
        // Expand this symbol until there's nothing left to do.
        //
        fstr::assign(&mut VALUE, &DEF);
        TRAN = true;

        while TRAN {
            spicelib::NEXTWD(&DEF, &mut FRSTWD, &mut REST);
            spicelib::UCASE(&FRSTWD.clone(), &mut FRSTWD, ctx);

            if (fstr::ne(&FRSTWD, b"DEFINE") && fstr::ne(&FRSTWD, b"UNDEFINE")) {
                STRAN(&VALUE.clone(), &mut VALUE, &mut TRAN, ctx)?;
            } else {
                TRAN = false;
            }
        }

        SETSYM(&NAME, &DEF, &VALUE, ctx);
        TABRPT(
            NCOL,
            ITEM.as_slice(),
            SIZE.as_slice(),
            WIDTH.as_slice(),
            JUSTR.as_slice(),
            PRESRV.as_slice(),
            SPCIAL.as_arg(),
            LMARGE,
            *SPACE.first(),
            RETSYM,
            ctx,
        )?;

        SYMGET(&mut NAME, &mut DEF, ctx)?;
    }

    NSPWLN(b" ", ctx)?;
    Ok(())
}
