//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure MATCHE ( Match two words, allowing for common errors )
pub fn MATCHE(
    WORD: &[u8],
    GUESS: &[u8],
    TRANSF: &mut [u8],
    LOC: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut COPY = [b' '; 65 as usize];
    let mut TEMPL = [b' '; 65 as usize];
    let mut MYGUES = [b' '; 65 as usize];
    let mut CLEN: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Save a copy of the first 64 significant characters in a buffer,
    // from which we may construct templates.
    //
    fstr::assign(&mut COPY, b" ");

    spicelib::LJUST(WORD, fstr::substr_mut(&mut COPY, 1..=64));
    spicelib::LJUST(GUESS, &mut MYGUES);

    CLEN = QRTRIM(&COPY);

    //
    // Apply the transformations one at a time, in the order most
    // likely to succeed:
    //
    //    Removal
    //    Transposition
    //    Replacement
    //    Insertion
    //
    // Quit as soon as a possible match is found.
    //
    // Actually, we need to check for identity first. Otherwise,
    // we're likely to find a transposition that yields the same
    // word: for example, transposing the second and third letters
    // of APPLE yields APPLE.
    //
    if spicelib::EQSTR(WORD, &MYGUES) {
        fstr::assign(TRANSF, b"IDENTITY");
        *LOC = 0;
        return Ok(());
    }

    //
    // Removal
    // -------
    //
    // Remove the character at each location, and check against MYGUES.
    //
    for I in 1..=CLEN {
        spicelib::REMSUB(&COPY, I, I, &mut TEMPL, ctx)?;

        if spicelib::EQSTR(&TEMPL, &MYGUES) {
            fstr::assign(TRANSF, b"REMOVE");
            *LOC = I;
            return Ok(());
        }
    }

    //
    // Transposition
    // -------------
    //
    // Transpose each pair of characters, and check against MYGUES.
    //
    for I in 1..=(CLEN - 1) {
        fstr::assign(&mut TEMPL, &COPY);
        fstr::assign(
            fstr::substr_mut(&mut TEMPL, I..=I),
            fstr::substr(&COPY, (I + 1)..=(I + 1)),
        );
        fstr::assign(
            fstr::substr_mut(&mut TEMPL, (I + 1)..=(I + 1)),
            fstr::substr(&COPY, I..=I),
        );

        if spicelib::EQSTR(&TEMPL, &MYGUES) {
            fstr::assign(TRANSF, b"TRANSPOSE");
            *LOC = I;
            return Ok(());
        }
    }

    //
    // Replacement
    // -----------
    //
    // Replace each character with a wild character, and check
    // against MYGUES.
    //
    for I in 1..=CLEN {
        fstr::assign(&mut TEMPL, &COPY);
        fstr::assign(
            fstr::substr_mut(&mut TEMPL, I..=I),
            fstr::substr(&MYGUES, I..=I),
        );

        if spicelib::EQSTR(&TEMPL, &MYGUES) {
            fstr::assign(TRANSF, b"REPLACE");
            *LOC = I;
            return Ok(());
        }
    }

    //
    // Insertion
    // ---------
    //
    // Insert a wild character at each location, and check against
    // MYGUES.
    //
    for I in 1..=(CLEN + 1) {
        if (I == 1) {
            fstr::assign(
                fstr::substr_mut(&mut TEMPL, 1..=1),
                fstr::substr(&MYGUES, 1..=1),
            );
            fstr::assign(fstr::substr_mut(&mut TEMPL, 2..), &COPY);
        } else if (I == (CLEN + 1)) {
            fstr::assign(&mut TEMPL, &COPY);
            fstr::assign(
                fstr::substr_mut(&mut TEMPL, I..=I),
                fstr::substr(&MYGUES, I..=I),
            );
        } else {
            fstr::assign(
                fstr::substr_mut(&mut TEMPL, 1..=(I - 1)),
                fstr::substr(&COPY, 1..=(I - 1)),
            );
            fstr::assign(
                fstr::substr_mut(&mut TEMPL, I..=I),
                fstr::substr(&MYGUES, I..=I),
            );
            fstr::assign(
                fstr::substr_mut(&mut TEMPL, (I + 1)..),
                fstr::substr(&COPY, I..),
            );
        }

        if spicelib::EQSTR(&TEMPL, &MYGUES) {
            fstr::assign(TRANSF, b"INSERT");
            *LOC = I;
            return Ok(());
        }
    }

    //
    // None of these transformations work.
    //
    fstr::assign(TRANSF, b"NONE");
    *LOC = 0;

    Ok(())
}
