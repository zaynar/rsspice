//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 160;
const DELIMS: &[u8] = b"() ,:/-";

struct SaveVars {
    M2WMCH: bool,
    BEG: i32,
    END: i32,
    WB: i32,
    WE: i32,
    BASE: Vec<u8>,
    ERROR: Vec<u8>,
    KEY: bool,
    TEMP: bool,
    XIN: f64,
    XOUT: f64,
    X: f64,
    Y: f64,
    V: f64,
    PNTR: i32,
    NB: i32,
    NE: i32,
    L: i32,
    LBRACE: i32,
    RBRACE: i32,
    I: i32,
    STATUS: i32,
    NAMFND: bool,
    TMPLOG: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut M2WMCH: bool = false;
        let mut BEG: i32 = 0;
        let mut END: i32 = 0;
        let mut WB: i32 = 0;
        let mut WE: i32 = 0;
        let mut BASE = vec![b' '; 32 as usize];
        let mut ERROR = vec![b' '; LNSIZE as usize];
        let mut KEY: bool = false;
        let mut TEMP: bool = false;
        let mut XIN: f64 = 0.0;
        let mut XOUT: f64 = 0.0;
        let mut X: f64 = 0.0;
        let mut Y: f64 = 0.0;
        let mut V: f64 = 0.0;
        let mut PNTR: i32 = 0;
        let mut NB: i32 = 0;
        let mut NE: i32 = 0;
        let mut L: i32 = 0;
        let mut LBRACE: i32 = 0;
        let mut RBRACE: i32 = 0;
        let mut I: i32 = 0;
        let mut STATUS: i32 = 0;
        let mut NAMFND: bool = false;
        let mut TMPLOG: bool = false;

        Self {
            M2WMCH,
            BEG,
            END,
            WB,
            WE,
            BASE,
            ERROR,
            KEY,
            TEMP,
            XIN,
            XOUT,
            X,
            Y,
            V,
            PNTR,
            NB,
            NE,
            L,
            LBRACE,
            RBRACE,
            I,
            STATUS,
            NAMFND,
            TMPLOG,
        }
    }
}

//$Procedure            M2WMCH ( Match a word against a META/2 class )
pub fn M2WMCH(
    STRING: &[u8],
    WORDB: i32,
    WORDE: i32,
    CLASS: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<bool> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // There are some obvious things we can handle right now.
    // Note that if we input a substring that is completely outside
    // the range (1, LEN(STRING)), then WB will be greater than WE.
    // Otherwize we will have trimmed the substring to lie within
    // the bounds of the string.
    //
    save.WB = intrinsics::MAX0(&[WORDB, 1]);
    save.WE = intrinsics::MIN0(&[WORDE, intrinsics::LEN(STRING)]);

    if (save.WB > save.WE) {
        save.M2WMCH = false;
        return Ok(save.M2WMCH);
    }

    //
    // Get the first and last characters of CLASS
    // These are EXPECTED to be the first and last characters of
    // CLASS.
    //
    save.BEG = 1;
    save.L = intrinsics::LEN(CLASS);
    save.LBRACE = intrinsics::ICHAR(b"[");
    save.RBRACE = intrinsics::ICHAR(b"]");

    //
    // Next see if there is a name attached to which we will write the
    // results of successful matches.
    //
    save.NAMFND = false;
    save.END = save.L;
    //
    // If the length is not at least 4 or the last character is not
    // a right brace, there is no name associated with this word.
    //
    if ((intrinsics::ICHAR(fstr::substr(CLASS, save.L..=save.L)) == save.RBRACE) && (save.L >= 4)) {
        //
        // Ok. We have a chance at getting a name.  Look for
        // a left brace and if found set the name and class end.
        //
        save.I = 2;

        while (save.I < (save.L - 1)) {
            if (intrinsics::ICHAR(fstr::substr(CLASS, save.I..=save.I)) == save.LBRACE) {
                save.NB = (save.I + 1);
                save.NE = (save.L - 1);
                save.END = (save.I - 1);
                save.I = save.L;
                save.NAMFND = true;
            }

            save.I = (save.I + 1);
        }
    }

    //
    // See if CLASS represents a specification keyword or a META-KEY.
    //
    M2TRAN(
        CLASS,
        &mut save.BEG,
        save.END,
        &mut save.BASE,
        &mut save.KEY,
        &mut save.TEMP,
        ctx,
    );

    //
    // If we have a specification keyword, the input WORD must match
    // exactly.
    //
    if save.KEY {
        save.M2WMCH = spicelib::EQSTR(
            fstr::substr(CLASS, 1..=save.END),
            fstr::substr(STRING, save.WB..=save.WE),
        );

    //
    // See if we are trying to match a numeric string.
    //
    } else if (fstr::eq(&save.BASE, b"@int") || fstr::eq(&save.BASE, b"@number")) {
        if fstr::eq(&save.BASE, b"@int") {
            save.M2WMCH = M2INT(fstr::substr(STRING, save.WB..=save.WE), ctx);
        } else if fstr::eq(&save.BASE, b"@number") {
            save.M2WMCH = M2NUMB(fstr::substr(STRING, save.WB..=save.WE), ctx);
        }

        if (save.M2WMCH && save.TEMP) {
            //
            // Parse the number and see if it is in bounds.
            //
            M2NTEM(
                CLASS,
                &save.BASE,
                &mut save.BEG,
                save.END,
                &mut save.X,
                &mut save.Y,
                ctx,
            );
            spicelib::NPARSD(
                fstr::substr(STRING, save.WB..=save.WE),
                &mut save.V,
                &mut save.ERROR,
                &mut save.PNTR,
                ctx,
            );

            save.M2WMCH = ((save.V <= save.Y) && (save.V >= save.X));
        }

        if (save.M2WMCH && save.NAMFND) {
            M2SAVE(
                fstr::substr(CLASS, save.NB..=save.NE),
                save.WB,
                save.WE,
                ctx,
            )?;
        }

        return Ok(save.M2WMCH);
    } else if fstr::eq(&save.BASE, b"@unit") {
        save.M2WMCH = M2UNIT(fstr::substr(STRING, save.WB..=save.WE), ctx)?;

        if (save.M2WMCH && save.TEMP) {
            save.XIN = 1.0;
            CONVRT_3(
                save.XIN,
                fstr::substr(STRING, save.WB..=save.WE),
                fstr::substr(CLASS, (save.BEG + 1)..=(save.END - 1)),
                &mut save.XOUT,
                &mut save.STATUS,
                ctx,
            )?;

            save.M2WMCH = (save.STATUS == 0);
        }

        if (save.M2WMCH && save.NAMFND) {
            M2SAVE(
                fstr::substr(CLASS, save.NB..=save.NE),
                save.WB,
                save.WE,
                ctx,
            )?;
        }

        return Ok(save.M2WMCH);
    } else if fstr::eq(&save.BASE, b"@name") {
        save.M2WMCH = M2NAME(fstr::substr(STRING, save.WB..=save.WE), ctx);
    } else if fstr::eq(&save.BASE, b"@body") {
        save.M2WMCH = M2BODY(fstr::substr(STRING, save.WB..=save.WE), ctx)?;
    } else if fstr::eq(&save.BASE, b"@english") {
        save.M2WMCH = M2ENGL(fstr::substr(STRING, save.WB..=save.WE), ctx);
    } else if fstr::eq(&save.BASE, b"@alpha") {
        save.M2WMCH = M2ALPH(fstr::substr(STRING, save.WB..=save.WE), ctx);
    } else if fstr::eq(&save.BASE, b"@time") {
        save.M2WMCH = M2TIME(fstr::substr(STRING, save.WB..=save.WE), ctx);
    } else if fstr::eq(&save.BASE, b"@epoch") {
        save.M2WMCH = M2EPOC(fstr::substr(STRING, save.WB..=save.WE), ctx);
    } else if fstr::eq(&save.BASE, b"@day") {
        save.M2WMCH = M2DAY(fstr::substr(STRING, save.WB..=save.WE), ctx);
    } else if fstr::eq(&save.BASE, b"@year") {
        save.M2WMCH = M2YEAR(fstr::substr(STRING, save.WB..=save.WE), ctx);
    } else if fstr::eq(&save.BASE, b"@month") {
        save.M2WMCH = M2MON(fstr::substr(STRING, save.WB..=save.WE), ctx);
    } else if fstr::eq(&save.BASE, b"@calendar") {
        save.TMPLOG = spicelib::ZZTOKNS(
            fstr::substr(STRING, save.WB..=save.WE),
            &mut save.ERROR,
            ctx,
        );

        save.M2WMCH = fstr::eq(&save.ERROR, b" ");
    } else if fstr::eq(&save.BASE, b"@word") {
        save.M2WMCH = true;
    }

    if (save.M2WMCH && save.TEMP) {
        save.M2WMCH = MATCHM(
            fstr::substr(STRING, save.WB..=save.WE),
            fstr::substr(CLASS, (save.BEG + 1)..=(save.END - 1)),
            b"*",
            b"%",
            b"~",
            b"|",
            ctx,
        )?;
    }

    if (save.M2WMCH && save.NAMFND) {
        M2SAVE(
            fstr::substr(CLASS, save.NB..=save.NE),
            save.WB,
            save.WE,
            ctx,
        )?;
    }

    Ok(save.M2WMCH)
}
