//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure     QTRAN
pub fn QTRAN(
    INPUT: &[u8],
    OUTPUT: &mut [u8],
    TRAN: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut EQUOTE = [b' '; 1 as usize];
    let mut DELIM = [b' '; 1 as usize];
    let mut QUERY = [b' '; 33];
    let mut LOC: i32 = 0;
    let mut QLEN: i32 = 0;
    let mut REPLY = [b' '; 128];
    let mut RLEN: i32 = 0;
    let mut PROMPT = [b' '; 55];
    let mut I: i32 = 0;

    //
    // OPTLIB functions
    //

    //
    // Local variables
    //

    //
    // Look up the special marker used for suppressing query
    // evaluation.
    //
    GETEQ(&mut EQUOTE, ctx);
    GETDEL(&mut DELIM, ctx);

    //
    // Look at each word. If a word ends with '?', it's a query.
    // (QUERY is a character longer than a valid query. So any
    // valid query will have at least one blank at the end.)
    //
    *TRAN = false;
    I = 1;

    NTHUQW(INPUT, I, &EQUOTE, &mut QUERY, &mut LOC);

    while (!*TRAN && fstr::ne(&QUERY, b" ")) {
        //
        // First we have to look for the translation supression
        // character.
        //

        *TRAN = ((intrinsics::INDEX(&QUERY, b"? ") > 0) && fstr::ne(&QUERY, b"?"));

        if !*TRAN {
            I = (I + 1);
            NTHUQW(INPUT, I, &EQUOTE, &mut QUERY, &mut LOC);
        }
    }

    fstr::assign(OUTPUT, INPUT);

    //
    // If we found a query, get the user's response, and insert it
    // in place of the query.
    //
    if *TRAN {
        QLEN = spicelib::LASTNB(&QUERY);
        fstr::assign(
            &mut PROMPT,
            &fstr::concat(
                &fstr::concat(b"Enter value for ", fstr::substr(&QUERY, 1..=(QLEN - 1))),
                b" > ",
            ),
        );

        RDSTMN(&PROMPT, &DELIM, &mut REPLY, ctx)?;
        RLEN = intrinsics::MAX0(&[1, spicelib::LASTNB(&REPLY)]);

        spicelib::REPSUB(
            &OUTPUT.to_vec(),
            LOC,
            ((LOC + QLEN) - 1),
            fstr::substr(&REPLY, 1..=RLEN),
            OUTPUT,
            ctx,
        )?;
    }

    Ok(())
}
