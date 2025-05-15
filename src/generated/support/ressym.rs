//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 255;

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
pub fn RESSYM(INPUT: &[u8], OUTPUT: &mut [u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut SUBSTR = [b' '; LNSIZE as usize];
    let mut TAB = [b' '; 1 as usize];
    let mut SPACE = [b' '; 1 as usize];
    let mut EQUOTE = [b' '; 1 as usize];
    let mut I: i32 = 0;
    let mut LOC: i32 = 0;
    let mut R: i32 = 0;
    let mut E: i32 = 0;
    let mut CHANGE: bool = false;
    let mut TRAN1: bool = false;
    let mut TRAN2: bool = false;

    spicelib::CHKIN(b"RESSYM", ctx)?;

    fstr::assign(&mut TAB, &intrinsics::CHAR(9));
    fstr::assign(&mut SPACE, b" ");

    GETEQ(&mut EQUOTE, ctx);

    spicelib::REPLCH(INPUT, &TAB, &SPACE, OUTPUT);

    PRTRAP(OUTPUT, &mut CHANGE, ctx)?;

    //
    // Now we just loop until all translations have
    // been performed.  We do:
    //
    //    1) symbol resolution
    //    2) query resolution
    //    3) tab removal
    //
    while CHANGE {
        CHANGE = false;
        TRAN1 = true;
        TRAN2 = true;
        //
        // First we resolve all symbols.  After each pass we check
        // that we have not created a command that must be trapped.
        //
        while (TRAN1 && TRAN2) {
            STRAN(&OUTPUT.to_vec(), OUTPUT, &mut TRAN1, ctx)?;
            PRTRAP(OUTPUT, &mut TRAN2, ctx)?;
            //
            // Determine whether or not more changes are possible
            // at this point.
            //
            CHANGE = (((CHANGE || TRAN1) && TRAN2) && !spicelib::FAILED(ctx));
        }
        //
        // If we don't have any errors we take a stab at replacing
        // all queries.  Note that queries can not result in changing
        // anything that isn't a query so we don't have to trap
        // inside the loop.  Note that this means you can't have
        // a command like DEFINE? SYMBOL? VALUE? and just replace
        // the first two queries.  You've got to do them all.  If
        // you want a symbol to have a query you must do it this
        // way:  DEFINE SYMBOL  QUERY?  That way the queries won't
        // get resolve too soon.
        //
        // Note:  This can easily be changed so that if a query
        // introduces a symbol, we immediately loop back to the
        // symbol resolution branch.  Simply change the DO WHILE
        // loop below to an IF.  The "loop" will then terminate
        // after one execution leaving any remaining queries
        // untouched until the next pass through the loop.
        //
        if spicelib::FAILED(ctx) {
            spicelib::CHKOUT(b"RESSYM", ctx)?;
            return Ok(());
        }

        TRAN1 = !spicelib::FAILED(ctx);

        while TRAN1 {
            QTRAN(&OUTPUT.to_vec(), OUTPUT, &mut TRAN1, ctx)?;
            spicelib::REPLCH(&OUTPUT.to_vec(), &TAB, &SPACE, OUTPUT);

            CHANGE = (CHANGE || TRAN1);
        }

        PRTRAP(OUTPUT, &mut TRAN2, ctx)?;
        CHANGE = (CHANGE && TRAN2);

        if spicelib::FAILED(ctx) {
            spicelib::CHKOUT(b"RESSYM", ctx)?;
            return Ok(());
        }
    }

    if TRAN2 {
        //
        // We remove the special markers that may have been present to
        // protect symbol or query resolution.
        //
        I = 1;
        NTHUQW(OUTPUT, I, b" ", &mut SUBSTR, &mut LOC);

        while (LOC > 0) {
            R = (spicelib::LASTNB(&SUBSTR) - 1);
            E = (LOC + R);

            spicelib::REPLCH(
                &fstr::substr(OUTPUT, LOC..=E).to_vec(),
                &EQUOTE,
                &SPACE,
                fstr::substr_mut(OUTPUT, LOC..=E),
            );

            I = (I + 1);
            NTHUQW(OUTPUT, I, b" ", &mut SUBSTR, &mut LOC);
        }
    }
    //
    // Finally, left justify the commmand.
    //
    spicelib::LJUST(&OUTPUT.to_vec(), OUTPUT);

    spicelib::CHKOUT(b"RESSYM", ctx)?;
    Ok(())
}
