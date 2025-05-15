//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;
const WDSIZE: i32 = 32;

struct SaveVars {
    ALPHA: ActualCharArray,
    BODY: ActualCharArray,
    DAY: ActualCharArray,
    DP: ActualCharArray,
    ENGLSH: ActualCharArray,
    EPOCH: ActualCharArray,
    GWORD: ActualCharArray,
    INT: ActualCharArray,
    MONTH: ActualCharArray,
    NAME: ActualCharArray,
    OTHER: ActualCharArray,
    TIME: ActualCharArray,
    UNITS: ActualCharArray,
    YEAR: ActualCharArray,
    B: i32,
    C: i32,
    E: i32,
    NUMBER: i32,
    BASE: Vec<u8>,
    KEY: bool,
    RTEMP: bool,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ALPHA = ActualCharArray::new(LNSIZE, 1..=2);
        let mut BODY = ActualCharArray::new(LNSIZE, 1..=2);
        let mut DAY = ActualCharArray::new(LNSIZE, 1..=2);
        let mut DP = ActualCharArray::new(LNSIZE, 1..=2);
        let mut ENGLSH = ActualCharArray::new(LNSIZE, 1..=2);
        let mut EPOCH = ActualCharArray::new(LNSIZE, 1..=2);
        let mut GWORD = ActualCharArray::new(LNSIZE, 1..=2);
        let mut INT = ActualCharArray::new(LNSIZE, 1..=2);
        let mut MONTH = ActualCharArray::new(LNSIZE, 1..=2);
        let mut NAME = ActualCharArray::new(LNSIZE, 1..=2);
        let mut OTHER = ActualCharArray::new(LNSIZE, 1..=2);
        let mut TIME = ActualCharArray::new(LNSIZE, 1..=2);
        let mut UNITS = ActualCharArray::new(LNSIZE, 1..=2);
        let mut YEAR = ActualCharArray::new(LNSIZE, 1..=2);
        let mut B: i32 = 0;
        let mut C: i32 = 0;
        let mut E: i32 = 0;
        let mut NUMBER: i32 = 0;
        let mut BASE = vec![b' '; WDSIZE as usize];
        let mut KEY: bool = false;
        let mut RTEMP: bool = false;
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            ALPHA,
            BODY,
            DAY,
            DP,
            ENGLSH,
            EPOCH,
            GWORD,
            INT,
            MONTH,
            NAME,
            OTHER,
            TIME,
            UNITS,
            YEAR,
            B,
            C,
            E,
            NUMBER,
            BASE,
            KEY,
            RTEMP,
            FIRST,
        }
    }
}

//$Procedure      M2CLSS (Meta 2 --- meta 2 word classification )
pub fn M2CLSS(WORD: &[u8], NUM: i32, PHRASE: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    if save.FIRST {
        save.FIRST = false;
        fstr::assign(save.ALPHA.get_mut(1), b"word beginning with a letter");
        fstr::assign(save.ALPHA.get_mut(2), b"words beginning with a letter");

        fstr::assign(save.BODY.get_mut(1), b"body name or id-code");
        fstr::assign(save.BODY.get_mut(2), b"body names or id-codes");

        fstr::assign(save.DAY.get_mut(1), b"day of the year");
        fstr::assign(save.DAY.get_mut(2), b"days of the year");

        fstr::assign(save.ENGLSH.get_mut(1), b"word containing only letters");
        fstr::assign(save.ENGLSH.get_mut(2), b"words containing only letters");

        fstr::assign(save.EPOCH.get_mut(1), b"epoch");
        fstr::assign(save.EPOCH.get_mut(2), b"epochs");

        fstr::assign(save.MONTH.get_mut(1), b"month of the year");
        fstr::assign(save.MONTH.get_mut(2), b"months of the year");

        fstr::assign(
            save.NAME.get_mut(1),
            b"word of letters and digits starting with a letter",
        );
        fstr::assign(
            save.NAME.get_mut(2),
            b"words of letters and digits each starting with a letter ",
        );

        fstr::assign(save.TIME.get_mut(1), b"time of day");
        fstr::assign(save.TIME.get_mut(2), b"times of the day");

        fstr::assign(save.YEAR.get_mut(1), b"calendar year (1000 to 3000) ");
        fstr::assign(save.YEAR.get_mut(2), b"calendar years (1000 to 3000) ");

        fstr::assign(save.GWORD.get_mut(1), b"generic word");
        fstr::assign(save.GWORD.get_mut(2), b"generic words");

        fstr::assign(
            save.OTHER.get_mut(1),
            &fstr::concat(b"word of class ", WORD),
        );
        fstr::assign(
            save.OTHER.get_mut(2),
            &fstr::concat(b"words of class ", WORD),
        );

        fstr::assign(save.INT.get_mut(1), b"integer");
        fstr::assign(save.INT.get_mut(2), b"integers");

        fstr::assign(save.DP.get_mut(1), b"number");
        fstr::assign(save.DP.get_mut(2), b"numbers");

        fstr::assign(save.UNITS.get_mut(1), b"unit specification");
        fstr::assign(save.UNITS.get_mut(2), b"unit specifications");
    }

    if (NUM == 1) {
        save.NUMBER = 1;
    } else {
        save.NUMBER = 2;
    }

    save.B = 1;
    save.E = spicelib::RTRIM(WORD);

    M2TRAN(
        WORD,
        &mut save.B,
        save.E,
        &mut save.BASE,
        &mut save.KEY,
        &mut save.RTEMP,
        ctx,
    );

    if fstr::eq(&save.BASE, b"@int") {
        fstr::assign(PHRASE, save.INT.get(save.NUMBER));

        if save.RTEMP {
            save.C = spicelib::POS(WORD, b":", save.B);

            if (save.C == (save.B + 1)) {
                spicelib::SUFFIX(b"less than or equal to #", 1, PHRASE);
                spicelib::REPMC(
                    &PHRASE.to_vec(),
                    b"#",
                    fstr::substr(WORD, (save.B + 2)..=(save.E - 1)),
                    PHRASE,
                );
            } else if (save.C == (save.E - 1)) {
                spicelib::SUFFIX(b"greater than or equal to #", 1, PHRASE);
                spicelib::REPMC(
                    &PHRASE.to_vec(),
                    b"#",
                    fstr::substr(WORD, (save.B + 1)..=(save.E - 2)),
                    PHRASE,
                );
            } else {
                spicelib::SUFFIX(b"between # and # (inclusive)", 1, PHRASE);
                spicelib::REPMC(
                    &PHRASE.to_vec(),
                    b"#",
                    fstr::substr(WORD, (save.B + 1)..=(save.C - 1)),
                    PHRASE,
                );
                spicelib::REPMC(
                    &PHRASE.to_vec(),
                    b"#",
                    fstr::substr(WORD, (save.C + 1)..=(save.E - 1)),
                    PHRASE,
                );
            }
        }

        return;
    }

    if fstr::eq(&save.BASE, b"@number") {
        fstr::assign(PHRASE, save.DP.get(save.NUMBER));

        if save.RTEMP {
            save.C = spicelib::POS(WORD, b":", (save.B + 1));

            if (save.C == (save.B + 1)) {
                spicelib::SUFFIX(b"less than or equal to #", 1, PHRASE);
                spicelib::REPMC(
                    &PHRASE.to_vec(),
                    b"#",
                    fstr::substr(WORD, (save.B + 2)..=(save.E - 1)),
                    PHRASE,
                );
            } else if (save.C == (save.E - 1)) {
                spicelib::SUFFIX(b"greater than or equal to #", 1, PHRASE);
                spicelib::REPMC(
                    &PHRASE.to_vec(),
                    b"#",
                    fstr::substr(WORD, (save.B + 1)..=(save.E - 2)),
                    PHRASE,
                );
            } else {
                spicelib::SUFFIX(b"between # and # (inclusive)", 1, PHRASE);
                spicelib::REPMC(
                    &PHRASE.to_vec(),
                    b"#",
                    fstr::substr(WORD, (save.B + 1)..=(save.C - 1)),
                    PHRASE,
                );
                spicelib::REPMC(
                    &PHRASE.to_vec(),
                    b"#",
                    fstr::substr(WORD, (save.C + 1)..=(save.E - 1)),
                    PHRASE,
                );
            }
        }

        return;
    }

    if fstr::eq(&save.BASE, b"@unit") {
        fstr::assign(PHRASE, save.UNITS.get(save.NUMBER));

        if save.RTEMP {
            spicelib::SUFFIX(b"with dimensions compatible with #", 1, PHRASE);
            spicelib::REPMC(
                &PHRASE.to_vec(),
                b"#",
                fstr::substr(WORD, (save.B + 1)..=(save.E - 1)),
                PHRASE,
            );
        }

        return;
    }

    if fstr::eq(&save.BASE, b"@alpha") {
        fstr::assign(PHRASE, save.ALPHA.get(save.NUMBER));
    } else if fstr::eq(&save.BASE, b"@body") {
        fstr::assign(PHRASE, save.BODY.get(save.NUMBER));
    } else if fstr::eq(&save.BASE, b"@day") {
        fstr::assign(PHRASE, save.DAY.get(save.NUMBER));
    } else if fstr::eq(&save.BASE, b"@english") {
        fstr::assign(PHRASE, save.ENGLSH.get(save.NUMBER));
    } else if fstr::eq(&save.BASE, b"@epoch") {
        fstr::assign(PHRASE, save.EPOCH.get(save.NUMBER));
    } else if fstr::eq(&save.BASE, b"@month") {
        fstr::assign(PHRASE, save.MONTH.get(save.NUMBER));
    } else if fstr::eq(&save.BASE, b"@name") {
        fstr::assign(PHRASE, save.NAME.get(save.NUMBER));
    } else if fstr::eq(&save.BASE, b"@time") {
        fstr::assign(PHRASE, save.TIME.get(save.NUMBER));
    } else if fstr::eq(&save.BASE, b"@year") {
        fstr::assign(PHRASE, save.YEAR.get(save.NUMBER));
    } else if fstr::eq(&save.BASE, b"@word") {
        fstr::assign(PHRASE, save.GWORD.get(save.NUMBER));
    } else {
        fstr::assign(PHRASE, save.OTHER.get(save.NUMBER));
    }

    if save.RTEMP {
        spicelib::SUFFIX(b"that matches the pattern \'", 1, PHRASE);
        spicelib::SUFFIX(fstr::substr(WORD, (save.B + 1)..=(save.E - 1)), 0, PHRASE);
        spicelib::SUFFIX(b"\'", 0, PHRASE);
    }
}
