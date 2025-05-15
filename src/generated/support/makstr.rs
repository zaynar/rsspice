//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      MAKSTR (Make String )
pub fn MAKSTR(PATTRN: &[u8], THIS: &[u8], NEXT: &[u8]) {

    //
    // Spicelib functions
    //

    //
    // Local Varialbes
    //
}

//$Procedure      FSTSTR ( First string matching a pattern )
pub fn FSTSTR(PATTRN: &[u8], NEXT: &mut [u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut I: i32 = 0;
    let mut J: i32 = 0;
    let mut KEEP: bool = false;

    //
    // There are two things to handle:
    //
    // balanced brackets: <>
    // balanced braces:   {}
    //
    // We do this in one pass.
    //
    fstr::assign(NEXT, b" ");
    KEEP = true;
    J = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = spicelib::RTRIM(PATTRN);
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            if fstr::eq(fstr::substr(PATTRN, I..=I), b">") {
                KEEP = true;
            }

            if fstr::eq(fstr::substr(PATTRN, I..=I), b"{") {
                fstr::assign(
                    fstr::substr_mut(NEXT, J..=J),
                    fstr::substr(PATTRN, (I + 1)..=(I + 1)),
                );
                J = (J + 1);
                KEEP = false;
            }

            if KEEP {
                fstr::assign(fstr::substr_mut(NEXT, J..=J), fstr::substr(PATTRN, I..=I));
                J = (J + 1);
            }

            if fstr::eq(fstr::substr(PATTRN, I..=I), b"<") {
                KEEP = false;
            }

            if fstr::eq(fstr::substr(PATTRN, I..=I), b"}") {
                KEEP = true;
            }

            if (J > intrinsics::LEN(NEXT)) {
                spicelib::CHKIN(b"FSTSTR", ctx)?;
                spicelib::SETMSG(
                    b"The string provided for the first name is too short for the input pattern. ",
                    ctx,
                );
                spicelib::SIGERR(b"SPICE(OUTPUTTOOLONG)", ctx)?;
                spicelib::CHKOUT(b"FSTSTR", ctx)?;
            }

            I += m3__;
        }
    }

    Ok(())
}

//$Procedure      NXTSTR (Next String)
pub fn NXTSTR(PATTRN: &[u8], THIS: &[u8], NEXT: &mut [u8]) {
    let mut I: i32 = 0;
    let mut J: i32 = 0;
    let mut K: i32 = 0;
    let mut MAX: i32 = 0;
    let mut MIN: i32 = 0;

    //
    // First copy THIS into NEXT and find the ends of PATTRN and NEXT.
    //
    fstr::assign(NEXT, THIS);
    J = spicelib::RTRIM(PATTRN);
    I = spicelib::RTRIM(NEXT);
    //
    // We work backwards from the right end of the string.
    //
    while (J > 0) {
        //
        // If the current character is a right brace we are going
        // to assume we are at the end of a restriction token.  Use
        // the range of the restriction and the current character
        // of NEXT to determine the "next" character and whether or
        // not we can quit now.
        //
        if fstr::eq(fstr::substr(PATTRN, J..=J), b"}") {
            MAX = intrinsics::ICHAR(fstr::substr(PATTRN, (J - 1)..=(J - 1)));
            MIN = intrinsics::ICHAR(fstr::substr(PATTRN, (J - 3)..=(J - 3)));
            K = (intrinsics::ICHAR(fstr::substr(NEXT, I..=I)) + 1);

            if (K > MAX) {
                //
                // Roll over the characters, We aren't done we
                // need to keep stepping back through the string
                //
                fstr::assign(fstr::substr_mut(NEXT, I..=I), &intrinsics::CHAR(MIN));
            } else if ((K > intrinsics::ICHAR(b"9")) && (K < intrinsics::ICHAR(b"a"))) {
                //
                // By convention, the first character following '9' is 'a'.
                // Since we don't need to "roll over" this character we
                // are done at this point.
                //
                fstr::assign(fstr::substr_mut(NEXT, I..=I), b"a");
                return;
            } else {
                //
                // We didn't need to roll over the character so we just
                // put in the new one and we can quit now.
                //
                fstr::assign(fstr::substr_mut(NEXT, I..=I), &intrinsics::CHAR(K));
                return;
            }
            //
            // perform the arithmetic needed if we had to roll over the
            // character.
            //
            J = (J - 5);
            I = (I - 1);
        //
        // If the character is '>' we assume we are at the right end
        // of an expansion.
        //
        } else if fstr::eq(fstr::substr(PATTRN, J..=J), b">") {
            //
            // Skip over the invisible portion of the expansion.
            //
            while fstr::ne(fstr::substr(PATTRN, J..=J), b"<") {
                J = (J - 1);
            }

            I = (I - 1);
        } else {
            //
            // Nothing to do, just back up to the character to the
            // left of the current character.
            //
            J = (J - 1);
            I = (I - 1);
        }
    }
}
