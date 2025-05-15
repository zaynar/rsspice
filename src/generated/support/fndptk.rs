//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      FNDPTK ( Find the previous token in a string )
pub fn FNDPTK(
    STRING: &[u8],
    DELIMS: &[u8],
    START: i32,
    BEG: &mut i32,
    END: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut LAST: i32 = 0;
    let mut EOL: i32 = 0;
    let mut B: i32 = 0;
    let mut ATDELM: bool = false;
    let mut ONSPCE: bool = false;

    //
    // SPICE funtions.
    //
    //
    // Local variables
    //

    //%&END_DECLARATIONS

    //
    // Standard SPICE error handling
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"FNDPTK", ctx)?;
    }

    //
    // First we gather some data regarding the input string and
    // delimiters
    //
    LAST = intrinsics::LEN(STRING);
    EOL = (LAST + 1);
    B = intrinsics::MIN0(&[(EOL + 1), START]);

    //
    // We don't have to do anything if we are starting past the end of
    // the string.
    //
    if (B < 1) {
        *BEG = 0;
        *END = 0;
        spicelib::CHKOUT(b"FNDPTK", ctx)?;
        return Ok(());
    }

    if (B < EOL) {
        ONSPCE = fstr::eq(fstr::substr(STRING, B..=B), b" ");
    } else {
        ONSPCE = false;
    }

    //
    // Are we currently pointing at a delimiter?
    //
    if (B > EOL) {
        ATDELM = false;
    } else if (B == EOL) {
        ATDELM = true;
    } else if (intrinsics::INDEX(DELIMS, fstr::substr(STRING, B..=B)) != 0) {
        ATDELM = true;
    } else {
        ATDELM = false;
    }

    if ATDELM {
        //
        // Yes.  Move left to a non-blank character
        //
        B = spicelib::NCPOSR(STRING, b" ", (B - 1));

        //
        // If we didn't find a non-blank, then there is not a previous
        // token.
        //
        if (B == 0) {
            *BEG = 0;
            *END = 0;
            spicelib::CHKOUT(b"FNDPTK", ctx)?;
            return Ok(());
        }
        //
        // Still here? Are we currently pointing at a delimiter?
        //
        if (intrinsics::INDEX(DELIMS, fstr::substr(STRING, B..=B)) != 0) {
            //
            // Yes. Move left to a non-blank.
            //
            B = spicelib::NCPOSR(STRING, b" ", (B - 1));
        }

        //
        // Move left to a delimiter, then Move right 1
        //
        B = (spicelib::CPOSR(STRING, DELIMS, B) + 1);

    //
    // Are we on a space?
    //
    } else if ONSPCE {
        //
        // Yes.  (note: space is not a delimiter ) Find the next
        // non-blank to the right.
        //
        B = spicelib::NCPOS(STRING, b" ", B);

        //
        // Is this a delimiter?
        //
        if (B == 0) {
            //
            // it was all blanks to the end of the string.  Make the
            // B point to the end + 1, that is a delimiter
            //
            B = EOL;
            B = spicelib::CPOSR(STRING, DELIMS, B);
        } else if (intrinsics::INDEX(DELIMS, fstr::substr(STRING, B..=B)) == 0) {
            //
            // No.   Move left to the first delimiter.
            //
            B = spicelib::CPOSR(STRING, DELIMS, B);

            //
            // If we ran off the front of the string without hitting a
            // delimiter, there isn't a previous token.  Checkout and
            // head for home.
            //
            if (B == 0) {
                *BEG = 0;
                *END = 0;
                spicelib::CHKOUT(b"FNDPTK", ctx)?;
                return Ok(());
            }
        }

        //
        // Move left to the first delimiter.
        // Move right 1
        //
        B = (spicelib::CPOSR(STRING, DELIMS, (B - 1)) + 1);
    } else {
        //
        // Otherwise
        //
        //    Move left to the first delimiter.
        //

        if (B > EOL) {
            B = EOL;
        } else {
            B = spicelib::CPOSR(STRING, DELIMS, B);

            //
            // B is now pointing at a delimiter.
            //
        }

        //----------
        if (B == 0) {
            *BEG = 0;
            *END = 0;
            spicelib::CHKOUT(b"FNDPTK", ctx)?;
            return Ok(());
        }
        //
        // Move left to the first non-blank  (here or to the left)
        //
        if (B < EOL) {
            B = spicelib::NCPOSR(STRING, b" ", B);

            //
            // B is now pointing to the first non-blank character to the
            // left of the token we started in.
            //
            if (((intrinsics::INDEX(DELIMS, fstr::substr(STRING, B..=B)) != 0)
                && (intrinsics::INDEX(DELIMS, b" ") != 0))
                && fstr::eq(fstr::substr(STRING, (B - 1)..=(B - 1)), b" "))
            {
                //
                // Move backwards to the true delimiter for the token
                // that ends here.
                //
                B = (spicelib::NCPOSR(STRING, b" ", (B - 1)) + 1);
            }
        } else {
            //
            // If we were at or beyond the EOL position, we need to
            // know if backing up to a non-blank puts us on a delimiter
            // or not.  If it does reset B to EOL.
            //
            B = spicelib::NCPOSR(STRING, b" ", B);

            if (intrinsics::INDEX(DELIMS, fstr::substr(STRING, B..=B)) != 0) {
                B = EOL;
            }
        }

        //
        // Move left to the first deliter, and then move right 1.
        //
        B = (spicelib::CPOSR(STRING, DELIMS, (B - 1)) + 1);
    }

    FNDNTK(STRING, DELIMS, B, BEG, END);

    spicelib::CHKOUT(b"FNDPTK", ctx)?;
    Ok(())
}
