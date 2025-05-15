//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure MATCHM ( Match string against multiple wildcard templates )
pub fn MATCHM(
    STRING: &[u8],
    TEMPL: &[u8],
    WSTR: &[u8],
    WCHR: &[u8],
    NOTCHR: &[u8],
    ORCHR: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<bool> {
    let WSTR = &WSTR[..1];
    let WCHR = &WCHR[..1];
    let NOTCHR = &NOTCHR[..1];
    let ORCHR = &ORCHR[..1];
    let mut MATCHM: bool = false;
    let mut BEG: i32 = 0;
    let mut END: i32 = 0;
    let mut B: i32 = 0;
    let mut E: i32 = 0;
    let mut MATCH: bool = false;
    let mut NEGATE: bool = false;
    let mut LOOP: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Give the function an initial value.
    //
    MATCHM = false;

    //
    // Standard SPICE error handling
    //
    if spicelib::RETURN(ctx) {
        return Ok(MATCHM);
    } else {
        spicelib::CHKIN(b"MATCHM", ctx)?;
    }

    //
    // Reject bad inputs.
    //
    if (((fstr::eq(WSTR, b" ") || fstr::eq(WCHR, b" ")) || fstr::eq(NOTCHR, b" "))
        || fstr::eq(ORCHR, b" "))
    {
        spicelib::SIGERR(b"SPICE(ILLEGTEMPL)", ctx)?;
        spicelib::CHKOUT(b"MATCHM", ctx)?;
        return Ok(MATCHM);
    } else if (((((fstr::eq(WSTR, WCHR) || fstr::eq(WSTR, NOTCHR)) || fstr::eq(WSTR, ORCHR))
        || fstr::eq(WCHR, NOTCHR))
        || fstr::eq(WCHR, ORCHR))
        || fstr::eq(NOTCHR, ORCHR))
    {
        spicelib::SIGERR(b"SPICE(AMBIGTEMPL)", ctx)?;
        spicelib::CHKOUT(b"MATCHM", ctx)?;
        return Ok(MATCHM);
    }

    //
    // Ignore leading and trailing spaces in the collection.
    //
    BEG = spicelib::FRSTNB(TEMPL);
    END = QLSTNB(TEMPL);

    //
    // A blank collection matches ONLY a blank string.
    //
    if (BEG == 0) {
        MATCHM = fstr::eq(STRING, b" ");
        spicelib::CHKOUT(b"MATCHM", ctx)?;
        return Ok(MATCHM);
    }

    //
    // If the first template is the NOT character, the entire collection
    // is negated, and we can begin with the next template. Otherwise,
    // just start at the beginning again.
    //
    B = BEG;
    E = UPTO(fstr::substr(TEMPL, 1..=END), ORCHR, B);

    if (E >= intrinsics::LEN(TEMPL)) {
        NEGATE = false;
        BEG = B;
    } else if (fstr::eq(fstr::substr(TEMPL, B..=E), NOTCHR)
        && fstr::eq(fstr::substr(TEMPL, (E + 1)..=(E + 1)), ORCHR))
    {
        NEGATE = true;
        BEG = (E + 2);
    } else {
        NEGATE = false;
        BEG = B;
    }

    //
    // Grab one template at a time, comparing them against the string
    // until a match has occured or until no templates remain.
    //
    MATCH = false;

    while ((BEG <= END) && !MATCH) {
        B = BEG;
        E = UPTO(fstr::substr(TEMPL, 1..=END), ORCHR, B);

        //
        // If we started on an OR character, then either we are
        // at the beginning of a string that starts with one,
        // or we just passed one and found another either next to
        // it, or separated by nothing but spaces. By convention,
        // either case is interpreted as a blank template.
        //
        if fstr::eq(fstr::substr(TEMPL, B..=B), ORCHR) {
            MATCH = fstr::eq(STRING, b" ");
            BEG = (BEG + 1);

        //
        // If this is a negated template, negate the results.
        // Remember that a NOT character by itself does not
        // matches anything.
        //
        } else if fstr::eq(fstr::substr(TEMPL, B..=B), NOTCHR) {
            if fstr::eq(fstr::substr(TEMPL, B..=E), NOTCHR) {
                MATCH = false;
            } else {
                MATCH =
                    !spicelib::MATCHI(STRING, fstr::substr(TEMPL, (B + 1)..=E), WSTR, WCHR, ctx);
            }

            BEG = (E + 2);

        //
        // Or a normal one?
        //
        } else {
            MATCH = spicelib::MATCHI(STRING, fstr::substr(TEMPL, B..=E), WSTR, WCHR, ctx);
            BEG = (E + 2);
        }

        //
        // Skip any blanks before the next template.
        // The logic ensures no evaluation of TEMPL(BEG:BEG)
        // if BEG > LEN(TEMPL).
        //
        LOOP = (BEG < END);
        if LOOP {
            LOOP = (LOOP && fstr::eq(fstr::substr(TEMPL, BEG..=BEG), b" "));
        }

        while LOOP {
            BEG = (BEG + 1);

            if (BEG >= END) {
                LOOP = false;
            } else if fstr::ne(fstr::substr(TEMPL, BEG..=BEG), b" ") {
                LOOP = false;
            } else {
                LOOP = true;
            }
        }
    }

    //
    // It doesn't happen often, but occasionally a template ends with
    // the OR character. This implies a blank template at the end of
    // the collection.
    //
    if fstr::eq(fstr::substr(TEMPL, END..=END), ORCHR) {
        if !MATCH {
            MATCH = fstr::eq(STRING, b" ");
        }
    }

    //
    // Negate the results, if appropriate.
    //
    if NEGATE {
        MATCHM = !MATCH;
    } else {
        MATCHM = MATCH;
    }

    spicelib::CHKOUT(b"MATCHM", ctx)?;
    Ok(MATCHM)
}
