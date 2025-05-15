//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NQUICK: i32 = 19;
const NFULL: i32 = 26;

struct SaveVars {
    I: i32,
    J: i32,
    K: i32,
    CWORD: Vec<u8>,
    QUICK: ActualCharArray,
    CHECKS: StackArray<i32, 19>,
    PNTRS: StackArray<i32, 19>,
    FULL: ActualCharArray,
    MATCH: bool,
    TEMPS: StackArray<i32, 19>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut I: i32 = 0;
        let mut J: i32 = 0;
        let mut K: i32 = 0;
        let mut CWORD = vec![b' '; 4];
        let mut QUICK = ActualCharArray::new(4, 1..=NQUICK);
        let mut CHECKS = StackArray::<i32, 19>::new(1..=NQUICK);
        let mut PNTRS = StackArray::<i32, 19>::new(1..=NQUICK);
        let mut FULL = ActualCharArray::new(16, 1..=NFULL);
        let mut MATCH: bool = false;
        let mut TEMPS = StackArray::<i32, 19>::new(1..=NQUICK);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"@alp"),
                Val::C(b"@bod"),
                Val::C(b"@cal"),
                Val::C(b"@day"),
                Val::C(b"@end"),
                Val::C(b"@eng"),
                Val::C(b"@epo"),
                Val::C(b"@int"),
                Val::C(b"@mon"),
                Val::C(b"@nam"),
                Val::C(b"@num"),
                Val::C(b"@the"),
                Val::C(b"@tim"),
                Val::C(b"@uni"),
                Val::C(b"@wor"),
                Val::C(b"@yea"),
                Val::C(b"{"),
                Val::C(b"|"),
                Val::C(b"}"),
            ]
            .into_iter();
            QUICK
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(6),
                Val::I(5),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(8),
                Val::I(0),
                Val::I(4),
                Val::I(6),
                Val::I(5),
                Val::I(7),
                Val::I(5),
                Val::I(0),
                Val::I(5),
                Val::I(5),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
            ]
            .into_iter();
            TEMPS
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(2),
                Val::I(2),
                Val::I(1),
                Val::I(1),
                Val::I(1),
                Val::I(2),
                Val::I(1),
                Val::I(2),
                Val::I(2),
                Val::I(2),
                Val::I(2),
                Val::I(2),
                Val::I(1),
                Val::I(2),
                Val::I(2),
                Val::I(1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
            ]
            .into_iter();
            CHECKS
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(1),
                Val::I(3),
                Val::I(5),
                Val::I(6),
                Val::I(7),
                Val::I(8),
                Val::I(10),
                Val::I(11),
                Val::I(13),
                Val::I(15),
                Val::I(17),
                Val::I(19),
                Val::I(21),
                Val::I(22),
                Val::I(24),
                Val::I(26),
                Val::I(26),
                Val::I(26),
                Val::I(26),
            ]
            .into_iter();
            PNTRS
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"@alpha"),
                Val::C(b"@alpha(%*)"),
                Val::C(b"@body"),
                Val::C(b"@body(%*)"),
                Val::C(b"@calendar"),
                Val::C(b"@day"),
                Val::C(b"@end"),
                Val::C(b"@english"),
                Val::C(b"@english(%*)"),
                Val::C(b"@epoch"),
                Val::C(b"@int"),
                Val::C(b"@int(*:*)"),
                Val::C(b"@month"),
                Val::C(b"@month(%*)"),
                Val::C(b"@name"),
                Val::C(b"@name(%*)"),
                Val::C(b"@number"),
                Val::C(b"@number(*:*)"),
                Val::C(b"@then"),
                Val::C(b"@then(%*)"),
                Val::C(b"@time"),
                Val::C(b"@unit"),
                Val::C(b"@unit(%*)"),
                Val::C(b"@word"),
                Val::C(b"@word(%*)"),
                Val::C(b"@year"),
            ]
            .into_iter();
            FULL.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            I,
            J,
            K,
            CWORD,
            QUICK,
            CHECKS,
            PNTRS,
            FULL,
            MATCH,
            TEMPS,
        }
    }
}

//$Procedure      M2TRAN ( See if a word has a restriction template )
pub fn M2TRAN(
    STRING: &[u8],
    BEG: &mut i32,
    END: i32,
    BASE: &mut [u8],
    KEY: &mut bool,
    TEMP: &mut bool,
    ctx: &mut Context,
) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // The array QUICK contains abbreviations of all of the know META-KEY
    // words in alphabetical order.
    //

    //
    // The array TEMPS gives the character position within a word where
    // a template will be attached to a META-KEY word.
    // If the first portion of a word equals QUICK(I), TEMP(I) will be
    // the character immediately before the template (if one is present).
    //
    // If a template is not allowed for a META-KEY word, TEMP will be 0.
    //

    //
    // The array CHECKS tells how many different ways a META-KEY word
    // can be represented.  For example @alpha or @alpha(template).
    // If a word matches up in the beginning with QUICK(I) then there
    // are at most CHECKS(I) checks that we must perform to see if it
    // is in fact a legitimate META-KEY word.
    //

    //
    // PNTRS(I) points to the first position in the array FULL where
    // one should look to find the actual patterns that should be
    // checked to see if a word that matches the initial portion
    // in QUICK(I) is in fact a META-KEY
    //

    //
    // First do a binary search on the abreviations of the META-KEYS
    // to see if this is a key word.
    //
    fstr::assign(&mut save.CWORD, fstr::substr(STRING, *BEG..=END));
    save.I = spicelib::BSRCHC(&save.CWORD, NQUICK, save.QUICK.as_arg());

    if (save.I == 0) {
        //
        // We didn't even match up with one of the abbreviations,  this
        // can't be a META-KEY and so must be a language specification
        // keyword.
        //
        *KEY = true;
        *TEMP = false;
        fstr::assign(BASE, fstr::substr(STRING, *BEG..=END));
        *BEG = (END + 1);
        return;
    } else {
        //
        // We at least match an abbreviation.  See if we match the
        // full expansion of the abbreviation.
        //
        *KEY = false;
        save.K = save.PNTRS[save.I];
        save.J = 1;
        save.MATCH = false;

        while ((save.J <= save.CHECKS[save.I]) && !save.MATCH) {
            save.MATCH = spicelib::MATCHW(
                fstr::substr(STRING, *BEG..=END),
                &save.FULL[save.K],
                b"*",
                b"%",
            );
            *KEY = !save.MATCH;

            save.K = (save.K + 1);
            save.J = (save.J + 1);
        }

        if *KEY {
            *TEMP = false;
            fstr::assign(BASE, fstr::substr(STRING, *BEG..=END));
            *BEG = (END + 1);
            return;
        }

        //
        // If we get this far we must have a META-KEY.  See if there
        // is a restriction template.
        //

        if fstr::eq(
            fstr::substr(STRING, *BEG..=END),
            save.FULL.get(save.PNTRS[save.I]),
        ) {
            //
            // There is no restriction template.
            //
            fstr::assign(BASE, fstr::substr(STRING, *BEG..=END));
            *BEG = (END + 1);
            *TEMP = false;
        } else {
            //
            // We have a restriction template.
            //

            fstr::assign(BASE, save.FULL.get(save.PNTRS[save.I]));
            *BEG = (*BEG + save.TEMPS[save.I]);
            *TEMP = true;
        }
    }
}
