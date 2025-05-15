//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NQUICK: i32 = 20;
const NSLOW: i32 = 25;

struct SaveVars {
    M2KEYW: bool,
    END: i32,
    I: i32,
    J: i32,
    K: i32,
    L: i32,
    LBRACE: i32,
    RBRACE: i32,
    BLANK: i32,
    CWORD: Vec<u8>,
    QUICK: ActualCharArray,
    CHECKS: StackArray<i32, 20>,
    PNTRS: StackArray<i32, 20>,
    SLOW: ActualCharArray,
    MATCH: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut M2KEYW: bool = false;
        let mut END: i32 = 0;
        let mut I: i32 = 0;
        let mut J: i32 = 0;
        let mut K: i32 = 0;
        let mut L: i32 = 0;
        let mut LBRACE: i32 = 0;
        let mut RBRACE: i32 = 0;
        let mut BLANK: i32 = 0;
        let mut CWORD = vec![b' '; 4];
        let mut QUICK = ActualCharArray::new(4, 1..=NQUICK);
        let mut CHECKS = StackArray::<i32, 20>::new(1..=NQUICK);
        let mut PNTRS = StackArray::<i32, 20>::new(1..=NQUICK);
        let mut SLOW = ActualCharArray::new(16, 1..=NSLOW);
        let mut MATCH: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b")"),
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
                Val::I(0),
                Val::I(2),
                Val::I(1),
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
                Val::I(0),
                Val::I(1),
                Val::I(3),
                Val::I(4),
                Val::I(5),
                Val::I(6),
                Val::I(7),
                Val::I(9),
                Val::I(10),
                Val::I(12),
                Val::I(14),
                Val::I(16),
                Val::I(18),
                Val::I(20),
                Val::I(21),
                Val::I(23),
                Val::I(25),
                Val::I(25),
                Val::I(25),
                Val::I(25),
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
            SLOW.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            M2KEYW,
            END,
            I,
            J,
            K,
            L,
            LBRACE,
            RBRACE,
            BLANK,
            CWORD,
            QUICK,
            CHECKS,
            PNTRS,
            SLOW,
            MATCH,
        }
    }
}

//$Procedure      M2KEYW ( Determine whether or not a word is a keyword )
pub fn M2KEYW(WORD: &[u8], ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // We are going to look at the first four characters of the input
    // word.  If it doesn't match one of the following, then it isn't
    // a meta-2 specification word, it's a keyword.  The data in
    // this array should always be in increasing order.
    //

    //
    // If after checking against the previous list we have a match,
    // then we need to do further checks to see if we have a
    // legitimate meta-2 specification word.  If we have a bracket or
    // vertical bar, we are done ( zero more checks are required ).
    // In other cases 1 or two more checks may be required.  The
    // data below tells how many further checks may be required.
    //

    //
    // The PNTRS array points to the slot in the SLOW check array
    // where our matching pattern templates reside for checking
    // the current input word.
    //

    fstr::assign(&mut save.CWORD, WORD);
    save.I = spicelib::BSRCHC(&save.CWORD, NQUICK, save.QUICK.as_arg());

    if (save.I == 0) {
        save.M2KEYW = true;
        return save.M2KEYW;
    }

    //
    // We only want to examine the portion of the word that preceeds
    // a parsing qualifier.  First locate the last non-blank character
    // of the word.
    //
    save.LBRACE = intrinsics::ICHAR(b"[");
    save.RBRACE = intrinsics::ICHAR(b"]");
    save.BLANK = intrinsics::ICHAR(b" ");
    save.END = intrinsics::LEN(WORD);

    while ((save.END > 1)
        && (intrinsics::ICHAR(fstr::substr(WORD, save.END..=save.END)) == save.BLANK))
    {
        save.END = (save.END - 1);
    }

    //
    // If the length is not at least 4 or the last character is not
    // a right brace, there is no name associated with this word.
    //
    if ((intrinsics::ICHAR(fstr::substr(WORD, save.END..=save.END)) == save.RBRACE)
        && (save.END >= 4))
    {
        //
        // Ok. We have a chance at getting a name.  Look for
        // a left brace and if found set the name and class end.
        //
        save.L = 2;

        while (save.L < (save.END - 1)) {
            if (intrinsics::ICHAR(fstr::substr(WORD, save.L..=save.L)) == save.LBRACE) {
                //
                // We've found the beginning of the name portion
                // of the word.  Record the end of the meta-2
                // word and then reset L so that we exit this loop.
                //
                save.END = (save.L - 1);
                save.L = save.END;
            }
            save.L = (save.L + 1);
        }
    }

    save.M2KEYW = false;
    save.K = save.PNTRS[save.I];
    save.J = 1;
    save.MATCH = false;

    while ((save.J <= save.CHECKS[save.I]) && !save.MATCH) {
        save.MATCH = spicelib::MATCHW(
            fstr::substr(WORD, 1..=save.END),
            &save.SLOW[save.K],
            b"*",
            b"%",
        );
        save.M2KEYW = !save.MATCH;

        save.K = (save.K + 1);
        save.J = (save.J + 1);
    }

    save.M2KEYW
}
