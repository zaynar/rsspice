//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure     PRTRAP
pub fn PRTRAP(COMMAND: &[u8], TRAN: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut WORD = ActualCharArray::new(33, 1..=3);
    let mut LOC: i32 = 0;

    //
    // Spicelib Functions
    //
    //
    // Local variables
    //

    //
    // Get the first three words of COMMAND.
    //
    for I in 1..=3 {
        spicelib::NTHWD(COMMAND, I, &mut WORD[I], &mut LOC);
        spicelib::UCASE(&WORD[I].to_vec(), &mut WORD[I], ctx);
    }

    //
    // Is this a SHOW SYMBOL command?
    //
    if (fstr::eq(WORD.get(1), b"SHOW") && fstr::eq(WORD.get(2), b"SYMBOL")) {
        //
        // The third word must not be blank, and must not end with '?'.
        // (WORD is longer than any allowable symbol or query, so there
        // should always be a blank at the end.)
        //
        if fstr::ne(WORD.get(3), b" ") {
            LOC = spicelib::RTRIM(&WORD[3]);

            if fstr::ne(fstr::substr(WORD.get(3), LOC..=LOC), b"?") {
                *TRAN = false;
                return Ok(());
            }
        }

    //
    // Is this an INQUIRE command?
    //
    } else if fstr::eq(WORD.get(1), b"INQUIRE") {
        //
        // The second word must not be blank, and must not end with '?'.
        //
        if fstr::ne(WORD.get(2), b" ") {
            LOC = spicelib::RTRIM(&WORD[2]);

            if fstr::eq(fstr::substr(WORD.get(2), LOC..=LOC), b"?") {
                *TRAN = false;

                spicelib::CHKIN(b"PRTRAP", ctx)?;
                spicelib::SETMSG(b"INQUIRE commands must be of the form INQUIRE <symbol_name>,  You have INQUIRE # which is inquiring for the value of a query. This kind of INQUIRE is not supported. ", ctx);
                spicelib::ERRCH(b"#", &WORD[2], ctx);
                spicelib::SIGERR(b"INVALID_INQUIRE", ctx)?;
                spicelib::CHKOUT(b"PRTRAP", ctx)?;
                return Ok(());
            }
        }
    }

    //
    // No reason to trap this.
    //
    *TRAN = true;

    Ok(())
}
