//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const BSLASH: i32 = 92;

struct SaveVars {
    ZZPHSH: i32,
    ZZSHSH: i32,
    ZZHASH: i32,
    ZZHASH2: i32,
    DIVISR: i32,
    VAL: StackArray<i32, 129>,
    F: i32,
    BASE: i32,
    BLANK: i32,
    LENGTH: i32,
    MAXDIV: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ZZPHSH: i32 = 0;
        let mut ZZSHSH: i32 = 0;
        let mut ZZHASH: i32 = 0;
        let mut ZZHASH2: i32 = 0;
        let mut DIVISR: i32 = 0;
        let mut VAL = StackArray::<i32, 129>::new(0..=128);
        let mut F: i32 = 0;
        let mut BASE: i32 = 0;
        let mut BLANK: i32 = 0;
        let mut LENGTH: i32 = 0;
        let mut MAXDIV: i32 = 0;
        let mut FIRST: bool = false;

        FIRST = true;
        DIVISR = -1;

        Self {
            ZZPHSH,
            ZZSHSH,
            ZZHASH,
            ZZHASH2,
            DIVISR,
            VAL,
            F,
            BASE,
            BLANK,
            LENGTH,
            MAXDIV,
            FIRST,
        }
    }
}

//$Procedure ZZPHSH ( Private---kernel POOL hash function umbrella )
pub fn ZZPHSH(WORD: &[u8], M: i32, M2: i32, ctx: &mut Context) -> i32 {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Entry Points
    //

    //
    // SPICELIB functions
    //

    //
    // Local Variables.
    //

    //
    // We do not diagnose a bogus call since this is a private routine.
    //
    save.ZZPHSH = 0;

    save.ZZPHSH
}

//$Procedure ZZSHSH ( Private---Set up POOL hash function )
pub fn ZZSHSH(M: i32, ctx: &mut Context) -> f2rust_std::Result<i32> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Return zero.
    //
    save.ZZSHSH = 0;

    //
    // The initialization block below is identical to the initialization
    // block in the entry ZZHASH2. If this block is changed in any way,
    // the block in ZZHASH2 must be changed in the same way.
    //
    if save.FIRST {
        save.FIRST = false;

        save.BASE = 68;
        save.BLANK = intrinsics::ICHAR(b" ");

        save.MAXDIV = ((INTMAX() / save.BASE) - 1);

        for I in 0..=128 {
            save.VAL[I] = 0;
        }

        save.VAL[intrinsics::ICHAR(b"0")] = 1;
        save.VAL[intrinsics::ICHAR(b"1")] = 2;
        save.VAL[intrinsics::ICHAR(b"2")] = 3;
        save.VAL[intrinsics::ICHAR(b"3")] = 4;
        save.VAL[intrinsics::ICHAR(b"4")] = 5;
        save.VAL[intrinsics::ICHAR(b"5")] = 6;
        save.VAL[intrinsics::ICHAR(b"6")] = 7;
        save.VAL[intrinsics::ICHAR(b"7")] = 8;
        save.VAL[intrinsics::ICHAR(b"8")] = 9;
        save.VAL[intrinsics::ICHAR(b"9")] = 10;
        save.VAL[intrinsics::ICHAR(b"A")] = 11;
        save.VAL[intrinsics::ICHAR(b"B")] = 12;
        save.VAL[intrinsics::ICHAR(b"C")] = 13;
        save.VAL[intrinsics::ICHAR(b"D")] = 14;
        save.VAL[intrinsics::ICHAR(b"E")] = 15;
        save.VAL[intrinsics::ICHAR(b"F")] = 16;
        save.VAL[intrinsics::ICHAR(b"G")] = 17;
        save.VAL[intrinsics::ICHAR(b"H")] = 18;
        save.VAL[intrinsics::ICHAR(b"I")] = 19;
        save.VAL[intrinsics::ICHAR(b"J")] = 20;
        save.VAL[intrinsics::ICHAR(b"K")] = 21;
        save.VAL[intrinsics::ICHAR(b"L")] = 22;
        save.VAL[intrinsics::ICHAR(b"M")] = 23;
        save.VAL[intrinsics::ICHAR(b"N")] = 24;
        save.VAL[intrinsics::ICHAR(b"O")] = 25;
        save.VAL[intrinsics::ICHAR(b"P")] = 26;
        save.VAL[intrinsics::ICHAR(b"Q")] = 27;
        save.VAL[intrinsics::ICHAR(b"R")] = 28;
        save.VAL[intrinsics::ICHAR(b"S")] = 29;
        save.VAL[intrinsics::ICHAR(b"T")] = 30;
        save.VAL[intrinsics::ICHAR(b"U")] = 31;
        save.VAL[intrinsics::ICHAR(b"V")] = 32;
        save.VAL[intrinsics::ICHAR(b"W")] = 33;
        save.VAL[intrinsics::ICHAR(b"X")] = 34;
        save.VAL[intrinsics::ICHAR(b"Y")] = 35;
        save.VAL[intrinsics::ICHAR(b"Z")] = 36;
        save.VAL[intrinsics::ICHAR(b"-")] = 37;
        save.VAL[intrinsics::ICHAR(b"_")] = 38;
        save.VAL[intrinsics::ICHAR(b".")] = 39;
        save.VAL[intrinsics::ICHAR(b"/")] = 40;
        save.VAL[intrinsics::ICHAR(b"!")] = 41;
        save.VAL[intrinsics::ICHAR(b"@")] = 42;
        save.VAL[intrinsics::ICHAR(b"#")] = 43;
        save.VAL[intrinsics::ICHAR(b"$")] = 44;
        save.VAL[intrinsics::ICHAR(b"%")] = 45;
        save.VAL[intrinsics::ICHAR(b"^")] = 46;
        save.VAL[intrinsics::ICHAR(b"&")] = 47;
        save.VAL[intrinsics::ICHAR(b"*")] = 48;
        save.VAL[intrinsics::ICHAR(b"(")] = 49;
        save.VAL[intrinsics::ICHAR(b")")] = 50;
        save.VAL[intrinsics::ICHAR(b"+")] = 51;
        save.VAL[intrinsics::ICHAR(b"=")] = 52;
        save.VAL[intrinsics::ICHAR(b"[")] = 53;
        save.VAL[intrinsics::ICHAR(b"{")] = 54;
        save.VAL[intrinsics::ICHAR(b"]")] = 55;
        save.VAL[intrinsics::ICHAR(b"}")] = 56;
        save.VAL[intrinsics::ICHAR(b"|")] = 57;
        save.VAL[BSLASH] = 58;
        save.VAL[intrinsics::ICHAR(b":")] = 59;
        save.VAL[intrinsics::ICHAR(b";")] = 60;
        save.VAL[intrinsics::ICHAR(b"<")] = 61;
        save.VAL[intrinsics::ICHAR(b",")] = 62;
        save.VAL[intrinsics::ICHAR(b">")] = 63;
        save.VAL[intrinsics::ICHAR(b"?")] = 64;

        //
        // Note, ICHAR('''') returns the ASCII value for the single
        // quote -> '.
        //
        save.VAL[intrinsics::ICHAR(b"\'")] = 65;
        save.VAL[intrinsics::ICHAR(b"\"")] = 66;
        save.VAL[intrinsics::ICHAR(b"`")] = 67;
        save.VAL[intrinsics::ICHAR(b"~")] = 68;

        save.VAL[intrinsics::ICHAR(b"a")] = save.VAL[intrinsics::ICHAR(b"A")];
        save.VAL[intrinsics::ICHAR(b"b")] = save.VAL[intrinsics::ICHAR(b"B")];
        save.VAL[intrinsics::ICHAR(b"c")] = save.VAL[intrinsics::ICHAR(b"C")];
        save.VAL[intrinsics::ICHAR(b"d")] = save.VAL[intrinsics::ICHAR(b"D")];
        save.VAL[intrinsics::ICHAR(b"e")] = save.VAL[intrinsics::ICHAR(b"E")];
        save.VAL[intrinsics::ICHAR(b"f")] = save.VAL[intrinsics::ICHAR(b"F")];
        save.VAL[intrinsics::ICHAR(b"g")] = save.VAL[intrinsics::ICHAR(b"G")];
        save.VAL[intrinsics::ICHAR(b"h")] = save.VAL[intrinsics::ICHAR(b"H")];
        save.VAL[intrinsics::ICHAR(b"i")] = save.VAL[intrinsics::ICHAR(b"I")];
        save.VAL[intrinsics::ICHAR(b"j")] = save.VAL[intrinsics::ICHAR(b"J")];
        save.VAL[intrinsics::ICHAR(b"k")] = save.VAL[intrinsics::ICHAR(b"K")];
        save.VAL[intrinsics::ICHAR(b"l")] = save.VAL[intrinsics::ICHAR(b"L")];
        save.VAL[intrinsics::ICHAR(b"m")] = save.VAL[intrinsics::ICHAR(b"M")];
        save.VAL[intrinsics::ICHAR(b"n")] = save.VAL[intrinsics::ICHAR(b"N")];
        save.VAL[intrinsics::ICHAR(b"o")] = save.VAL[intrinsics::ICHAR(b"O")];
        save.VAL[intrinsics::ICHAR(b"p")] = save.VAL[intrinsics::ICHAR(b"P")];
        save.VAL[intrinsics::ICHAR(b"q")] = save.VAL[intrinsics::ICHAR(b"Q")];
        save.VAL[intrinsics::ICHAR(b"r")] = save.VAL[intrinsics::ICHAR(b"R")];
        save.VAL[intrinsics::ICHAR(b"s")] = save.VAL[intrinsics::ICHAR(b"S")];
        save.VAL[intrinsics::ICHAR(b"t")] = save.VAL[intrinsics::ICHAR(b"T")];
        save.VAL[intrinsics::ICHAR(b"u")] = save.VAL[intrinsics::ICHAR(b"U")];
        save.VAL[intrinsics::ICHAR(b"v")] = save.VAL[intrinsics::ICHAR(b"V")];
        save.VAL[intrinsics::ICHAR(b"w")] = save.VAL[intrinsics::ICHAR(b"W")];
        save.VAL[intrinsics::ICHAR(b"x")] = save.VAL[intrinsics::ICHAR(b"X")];
        save.VAL[intrinsics::ICHAR(b"y")] = save.VAL[intrinsics::ICHAR(b"Y")];
        save.VAL[intrinsics::ICHAR(b"z")] = save.VAL[intrinsics::ICHAR(b"Z")];
    }

    //
    // Check and save divisor.
    //
    if ((M <= 0) || (M > save.MAXDIV)) {
        CHKIN(b"ZZSHSH", ctx)?;
        SETMSG(
            b"The input hash function divisor was not in the allowed range from 1 to #. It was #.",
            ctx,
        );
        ERRINT(b"#", save.MAXDIV, ctx);
        ERRINT(b"#", M, ctx);
        SIGERR(b"SPICE(INVALIDDIVISOR)", ctx)?;
        CHKOUT(b"ZZSHSH", ctx)?;
        return Ok(save.ZZSHSH);
    }

    save.DIVISR = M;

    Ok(save.ZZSHSH)
}

//$Procedure ZZHASH ( Private---POOL Hash function )
pub fn ZZHASH(WORD: &[u8], ctx: &mut Context) -> f2rust_std::Result<i32> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Check if divisor was initialized by a prior call to ZZSHSH.
    //
    if (save.DIVISR == -1) {
        save.ZZHASH = 0;

        CHKIN(b"ZZHASH", ctx)?;
        SETMSG(b"The ZZHASH function was called before the POOL hash parameters were initialized by a call to ZZSHSH.", ctx);
        SIGERR(b"SPICE(CALLEDOUTOFORDER)", ctx)?;
        CHKOUT(b"ZZHASH", ctx)?;
        return Ok(save.ZZHASH);
    }

    //
    // Compute hash value for the input string.
    //
    save.F = 0;
    save.LENGTH = intrinsics::LEN(WORD);

    for I in 1..=save.LENGTH {
        if (intrinsics::ICHAR(fstr::substr(WORD, I..=I)) == save.BLANK) {
            save.ZZHASH = (intrinsics::MOD((save.F * save.BASE), save.DIVISR) + 1);

            //
            // A negative value for ZZHASH indicates a serious problem.
            //
            if (save.ZZHASH < 0) {
                CHKIN(b"ZZHASH", ctx)?;
                SETMSG(
                    b"The ZZHASH function calculated a negative value for string $1. Contact NAIF.",
                    ctx,
                );
                ERRCH(b"$1", WORD, ctx);
                SIGERR(b"SPICE(NEGATIVEHASHVALUE1)", ctx)?;
                CHKOUT(b"ZZHASH", ctx)?;
            }

            return Ok(save.ZZHASH);
        }

        save.F = (save.VAL[intrinsics::MIN0(&[128, intrinsics::ICHAR(fstr::substr(WORD, I..=I))])]
            + (save.F * save.BASE));
        save.F = intrinsics::MOD(save.F, save.DIVISR);
    }

    save.ZZHASH = (intrinsics::MOD((save.F * save.BASE), save.DIVISR) + 1);

    //
    // A negative value for ZZHASH indicates a serious problem.
    //
    if (save.ZZHASH < 0) {
        CHKIN(b"ZZHASH", ctx)?;
        SETMSG(
            b"The ZZHASH function calculated a negative value for string $1. Contact NAIF.",
            ctx,
        );
        ERRCH(b"$1", WORD, ctx);
        SIGERR(b"SPICE(NEGATIVEHASHVALUE2)", ctx)?;
        CHKOUT(b"ZZHASH", ctx)?;
    }

    Ok(save.ZZHASH)
}

//$Procedure ZZHASH2 ( Private---Arbitrary divisor hash function )
pub fn ZZHASH2(WORD: &[u8], M2: i32, ctx: &mut Context) -> f2rust_std::Result<i32> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // The initialization block below is identical to the initialization
    // block in the entry ZZSHSH. If this block is changed in any way,
    // the block in ZZSHSH must be changed in the same way.
    //
    if save.FIRST {
        save.FIRST = false;

        save.BASE = 68;
        save.BLANK = intrinsics::ICHAR(b" ");

        save.MAXDIV = ((INTMAX() / save.BASE) - 1);

        for I in 0..=128 {
            save.VAL[I] = 0;
        }

        save.VAL[intrinsics::ICHAR(b"0")] = 1;
        save.VAL[intrinsics::ICHAR(b"1")] = 2;
        save.VAL[intrinsics::ICHAR(b"2")] = 3;
        save.VAL[intrinsics::ICHAR(b"3")] = 4;
        save.VAL[intrinsics::ICHAR(b"4")] = 5;
        save.VAL[intrinsics::ICHAR(b"5")] = 6;
        save.VAL[intrinsics::ICHAR(b"6")] = 7;
        save.VAL[intrinsics::ICHAR(b"7")] = 8;
        save.VAL[intrinsics::ICHAR(b"8")] = 9;
        save.VAL[intrinsics::ICHAR(b"9")] = 10;
        save.VAL[intrinsics::ICHAR(b"A")] = 11;
        save.VAL[intrinsics::ICHAR(b"B")] = 12;
        save.VAL[intrinsics::ICHAR(b"C")] = 13;
        save.VAL[intrinsics::ICHAR(b"D")] = 14;
        save.VAL[intrinsics::ICHAR(b"E")] = 15;
        save.VAL[intrinsics::ICHAR(b"F")] = 16;
        save.VAL[intrinsics::ICHAR(b"G")] = 17;
        save.VAL[intrinsics::ICHAR(b"H")] = 18;
        save.VAL[intrinsics::ICHAR(b"I")] = 19;
        save.VAL[intrinsics::ICHAR(b"J")] = 20;
        save.VAL[intrinsics::ICHAR(b"K")] = 21;
        save.VAL[intrinsics::ICHAR(b"L")] = 22;
        save.VAL[intrinsics::ICHAR(b"M")] = 23;
        save.VAL[intrinsics::ICHAR(b"N")] = 24;
        save.VAL[intrinsics::ICHAR(b"O")] = 25;
        save.VAL[intrinsics::ICHAR(b"P")] = 26;
        save.VAL[intrinsics::ICHAR(b"Q")] = 27;
        save.VAL[intrinsics::ICHAR(b"R")] = 28;
        save.VAL[intrinsics::ICHAR(b"S")] = 29;
        save.VAL[intrinsics::ICHAR(b"T")] = 30;
        save.VAL[intrinsics::ICHAR(b"U")] = 31;
        save.VAL[intrinsics::ICHAR(b"V")] = 32;
        save.VAL[intrinsics::ICHAR(b"W")] = 33;
        save.VAL[intrinsics::ICHAR(b"X")] = 34;
        save.VAL[intrinsics::ICHAR(b"Y")] = 35;
        save.VAL[intrinsics::ICHAR(b"Z")] = 36;
        save.VAL[intrinsics::ICHAR(b"-")] = 37;
        save.VAL[intrinsics::ICHAR(b"_")] = 38;
        save.VAL[intrinsics::ICHAR(b".")] = 39;
        save.VAL[intrinsics::ICHAR(b"/")] = 40;
        save.VAL[intrinsics::ICHAR(b"!")] = 41;
        save.VAL[intrinsics::ICHAR(b"@")] = 42;
        save.VAL[intrinsics::ICHAR(b"#")] = 43;
        save.VAL[intrinsics::ICHAR(b"$")] = 44;
        save.VAL[intrinsics::ICHAR(b"%")] = 45;
        save.VAL[intrinsics::ICHAR(b"^")] = 46;
        save.VAL[intrinsics::ICHAR(b"&")] = 47;
        save.VAL[intrinsics::ICHAR(b"*")] = 48;
        save.VAL[intrinsics::ICHAR(b"(")] = 49;
        save.VAL[intrinsics::ICHAR(b")")] = 50;
        save.VAL[intrinsics::ICHAR(b"+")] = 51;
        save.VAL[intrinsics::ICHAR(b"=")] = 52;
        save.VAL[intrinsics::ICHAR(b"[")] = 53;
        save.VAL[intrinsics::ICHAR(b"{")] = 54;
        save.VAL[intrinsics::ICHAR(b"]")] = 55;
        save.VAL[intrinsics::ICHAR(b"}")] = 56;
        save.VAL[intrinsics::ICHAR(b"|")] = 57;
        save.VAL[BSLASH] = 58;
        save.VAL[intrinsics::ICHAR(b":")] = 59;
        save.VAL[intrinsics::ICHAR(b";")] = 60;
        save.VAL[intrinsics::ICHAR(b"<")] = 61;
        save.VAL[intrinsics::ICHAR(b",")] = 62;
        save.VAL[intrinsics::ICHAR(b">")] = 63;
        save.VAL[intrinsics::ICHAR(b"?")] = 64;

        //
        // Note, ICHAR('''') returns the ASCII value for the single
        // quote -> '.
        //
        save.VAL[intrinsics::ICHAR(b"\'")] = 65;
        save.VAL[intrinsics::ICHAR(b"\"")] = 66;
        save.VAL[intrinsics::ICHAR(b"`")] = 67;
        save.VAL[intrinsics::ICHAR(b"~")] = 68;

        save.VAL[intrinsics::ICHAR(b"a")] = save.VAL[intrinsics::ICHAR(b"A")];
        save.VAL[intrinsics::ICHAR(b"b")] = save.VAL[intrinsics::ICHAR(b"B")];
        save.VAL[intrinsics::ICHAR(b"c")] = save.VAL[intrinsics::ICHAR(b"C")];
        save.VAL[intrinsics::ICHAR(b"d")] = save.VAL[intrinsics::ICHAR(b"D")];
        save.VAL[intrinsics::ICHAR(b"e")] = save.VAL[intrinsics::ICHAR(b"E")];
        save.VAL[intrinsics::ICHAR(b"f")] = save.VAL[intrinsics::ICHAR(b"F")];
        save.VAL[intrinsics::ICHAR(b"g")] = save.VAL[intrinsics::ICHAR(b"G")];
        save.VAL[intrinsics::ICHAR(b"h")] = save.VAL[intrinsics::ICHAR(b"H")];
        save.VAL[intrinsics::ICHAR(b"i")] = save.VAL[intrinsics::ICHAR(b"I")];
        save.VAL[intrinsics::ICHAR(b"j")] = save.VAL[intrinsics::ICHAR(b"J")];
        save.VAL[intrinsics::ICHAR(b"k")] = save.VAL[intrinsics::ICHAR(b"K")];
        save.VAL[intrinsics::ICHAR(b"l")] = save.VAL[intrinsics::ICHAR(b"L")];
        save.VAL[intrinsics::ICHAR(b"m")] = save.VAL[intrinsics::ICHAR(b"M")];
        save.VAL[intrinsics::ICHAR(b"n")] = save.VAL[intrinsics::ICHAR(b"N")];
        save.VAL[intrinsics::ICHAR(b"o")] = save.VAL[intrinsics::ICHAR(b"O")];
        save.VAL[intrinsics::ICHAR(b"p")] = save.VAL[intrinsics::ICHAR(b"P")];
        save.VAL[intrinsics::ICHAR(b"q")] = save.VAL[intrinsics::ICHAR(b"Q")];
        save.VAL[intrinsics::ICHAR(b"r")] = save.VAL[intrinsics::ICHAR(b"R")];
        save.VAL[intrinsics::ICHAR(b"s")] = save.VAL[intrinsics::ICHAR(b"S")];
        save.VAL[intrinsics::ICHAR(b"t")] = save.VAL[intrinsics::ICHAR(b"T")];
        save.VAL[intrinsics::ICHAR(b"u")] = save.VAL[intrinsics::ICHAR(b"U")];
        save.VAL[intrinsics::ICHAR(b"v")] = save.VAL[intrinsics::ICHAR(b"V")];
        save.VAL[intrinsics::ICHAR(b"w")] = save.VAL[intrinsics::ICHAR(b"W")];
        save.VAL[intrinsics::ICHAR(b"x")] = save.VAL[intrinsics::ICHAR(b"X")];
        save.VAL[intrinsics::ICHAR(b"y")] = save.VAL[intrinsics::ICHAR(b"Y")];
        save.VAL[intrinsics::ICHAR(b"z")] = save.VAL[intrinsics::ICHAR(b"Z")];
    }

    //
    // Check divisor.
    //
    if ((M2 <= 0) || (M2 > save.MAXDIV)) {
        save.ZZHASH2 = 0;

        CHKIN(b"ZZHASH2", ctx)?;
        SETMSG(
            b"The input hash function divisor was not in the allowed range from 1 to #. It was #.",
            ctx,
        );
        ERRINT(b"#", save.MAXDIV, ctx);
        ERRINT(b"#", M2, ctx);
        SIGERR(b"SPICE(INVALIDDIVISOR)", ctx)?;
        CHKOUT(b"ZZHASH2", ctx)?;
        return Ok(save.ZZHASH2);
    }

    //
    // Compute hash value for the input string.
    //
    save.F = 0;
    save.LENGTH = intrinsics::LEN(WORD);

    for I in 1..=save.LENGTH {
        if (intrinsics::ICHAR(fstr::substr(WORD, I..=I)) == save.BLANK) {
            save.ZZHASH2 = (intrinsics::MOD((save.F * save.BASE), M2) + 1);

            //
            // A negative value for ZZHASH2 indicates a serious problem.
            //
            if (save.ZZHASH2 < 0) {
                CHKIN(b"ZZHASH2", ctx)?;
                SETMSG(b"The ZZHASH2 function calculated a negative value for string $1. Contact NAIF.", ctx);
                ERRCH(b"$1", WORD, ctx);
                SIGERR(b"SPICE(NEGATIVEHASHVALUE1)", ctx)?;
                CHKOUT(b"ZZHASH2", ctx)?;
            }

            return Ok(save.ZZHASH2);
        }

        save.F = (save.VAL[intrinsics::MIN0(&[128, intrinsics::ICHAR(fstr::substr(WORD, I..=I))])]
            + (save.F * save.BASE));
        save.F = intrinsics::MOD(save.F, M2);
    }

    save.ZZHASH2 = (intrinsics::MOD((save.F * save.BASE), M2) + 1);

    //
    // A negative value for ZZHASH2 indicates a serious problem.
    //
    if (save.ZZHASH2 < 0) {
        CHKIN(b"ZZHASH2", ctx)?;
        SETMSG(
            b"The ZZHASH2 function calculated a negative value for string $1. Contact NAIF.",
            ctx,
        );
        ERRCH(b"$1", WORD, ctx);
        SIGERR(b"SPICE(NEGATIVEHASHVALUE2)", ctx)?;
        CHKOUT(b"ZZHASH2", ctx)?;
    }

    Ok(save.ZZHASH2)
}
