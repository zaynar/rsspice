//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    M2MON: bool,
    I: i32,
    LENGTH: i32,
    START: i32,
    END: i32,
    SHORT: ActualCharArray,
    MONTHS: ActualCharArray,
    COPY: Vec<u8>,
    MONTH: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut M2MON: bool = false;
        let mut I: i32 = 0;
        let mut LENGTH: i32 = 0;
        let mut START: i32 = 0;
        let mut END: i32 = 0;
        let mut SHORT = ActualCharArray::new(3, 1..=12);
        let mut MONTHS = ActualCharArray::new(9, 1..=12);
        let mut COPY = vec![b' '; 9];
        let mut MONTH: i32 = 0;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"APR"),
                Val::C(b"AUG"),
                Val::C(b"DEC"),
                Val::C(b"FEB"),
                Val::C(b"JAN"),
                Val::C(b"JUL"),
                Val::C(b"JUN"),
                Val::C(b"MAR"),
                Val::C(b"MAY"),
                Val::C(b"NOV"),
                Val::C(b"OCT"),
                Val::C(b"SEP"),
            ]
            .into_iter();
            SHORT
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"APRIL"),
                Val::C(b"AUGUST"),
                Val::C(b"DECEMBER"),
                Val::C(b"FEBRUARY"),
                Val::C(b"JANUARY"),
                Val::C(b"JULY"),
                Val::C(b"JUNE"),
                Val::C(b"MARCH"),
                Val::C(b"MAY"),
                Val::C(b"NOVEMBER"),
                Val::C(b"OCTOBER"),
                Val::C(b"SEPTEMBER"),
            ]
            .into_iter();
            MONTHS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            M2MON,
            I,
            LENGTH,
            START,
            END,
            SHORT,
            MONTHS,
            COPY,
            MONTH,
        }
    }
}

//$Procedure      M2MON ( Determine whether or not a word is a month )
pub fn M2MON(WORD: &[u8], ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Make sure the string has the right length.
    //
    save.START = spicelib::LTRIM(WORD);
    save.END = QRTRIM(WORD);
    save.LENGTH = ((save.END - save.START) + 1);

    if (save.LENGTH < 3) {
        save.M2MON = false;
        return save.M2MON;
    }

    if (save.LENGTH > 9) {
        save.M2MON = false;
        return save.M2MON;
    }

    spicelib::UCASE(WORD, &mut save.COPY, ctx);

    //
    // See if the first three letters match anything we've got so far.
    //
    save.MONTH = spicelib::BSRCHC(
        fstr::substr(&save.COPY, save.START..=(save.START + 2)),
        12,
        save.SHORT.as_arg(),
    );

    if (save.MONTH == 0) {
        save.M2MON = false;
        return save.M2MON;
    }

    //
    // Now make sure that any remaining letters match up exactly.
    //
    save.I = (save.START + 3);
    save.M2MON = true;

    while ((save.I <= save.END) && save.M2MON) {
        save.M2MON = fstr::eq(
            fstr::substr(&save.COPY, save.I..=save.I),
            fstr::substr(save.MONTHS.get(save.MONTH), save.I..=save.I),
        );
        save.I = (save.I + 1);
    }

    save.M2MON
}
