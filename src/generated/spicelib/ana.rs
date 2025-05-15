//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const ANWRDS: i32 = 22;
const AWRDS: i32 = 33;
const WDSIZE: i32 = 32;

struct SaveVars {
    ANWORD: ActualCharArray,
    AWORD: ActualCharArray,
    MYWORD: Vec<u8>,
    MYCASE: Vec<u8>,
    BEGIN: Vec<u8>,
    CAPS: i32,
    START: ActualCharArray,
    A: ActualCharArray,
    AN: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ANWORD = ActualCharArray::new(8, 1..=ANWRDS);
        let mut AWORD = ActualCharArray::new(8, 1..=AWRDS);
        let mut MYWORD = vec![b' '; WDSIZE as usize];
        let mut MYCASE = vec![b' '; 1 as usize];
        let mut BEGIN = vec![b' '; 1 as usize];
        let mut CAPS: i32 = 0;
        let mut START = ActualCharArray::new(WDSIZE, 1..=7);
        let mut A = ActualCharArray::new(2, 1..=3);
        let mut AN = ActualCharArray::new(2, 1..=3);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"A"), Val::C(b"A"), Val::C(b"a")].into_iter();
            A.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"AN"), Val::C(b"An"), Val::C(b"an")].into_iter();
            AN.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"HEIR"),
                Val::C(b"HONEST"),
                Val::C(b"HONOR"),
                Val::C(b"H"),
                Val::C(b"HOUR"),
                Val::C(b"HORS "),
                Val::C(b"HOMBRE"),
                Val::C(b"F"),
                Val::C(b"L"),
                Val::C(b"M"),
                Val::C(b"N"),
                Val::C(b"R"),
                Val::C(b"S"),
                Val::C(b"X"),
                Val::C(b"UNIN"),
                Val::C(b"UNIM"),
                Val::C(b"ONEI"),
                Val::C(b"ONER"),
                Val::C(b"SPK"),
                Val::C(b"EK"),
                Val::C(b"IK"),
                Val::C(b"SCLK"),
            ]
            .into_iter();
            ANWORD
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"HORSE"),
                Val::C(b"ONE"),
                Val::C(b"ONE-"),
                Val::C(b"ONCE"),
                Val::C(b"ONENESS"),
                Val::C(b"UIG"),
                Val::C(b"UIN"),
                Val::C(b"UKA"),
                Val::C(b"UKE"),
                Val::C(b"UKO"),
                Val::C(b"UKI"),
                Val::C(b"UKU"),
                Val::C(b"ULOT"),
                Val::C(b"UNANI"),
                Val::C(b"UNI"),
                Val::C(b"UNINU"),
                Val::C(b"UPA"),
                Val::C(b"URA"),
                Val::C(b"URE"),
                Val::C(b"URO"),
                Val::C(b"USA"),
                Val::C(b"USE"),
                Val::C(b"USU"),
                Val::C(b"UTE"),
                Val::C(b"UTI"),
                Val::C(b"UTO"),
                Val::C(b"UVA"),
                Val::C(b"UVE"),
                Val::C(b"UVU"),
                Val::C(b"EU"),
                Val::C(b"EWE"),
                Val::C(b"UTRI"),
                Val::C(b"U"),
            ]
            .into_iter();
            AWORD
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            ANWORD,
            AWORD,
            MYWORD,
            MYCASE,
            BEGIN,
            CAPS,
            START,
            A,
            AN,
        }
    }
}

/// AN or A ?
///
/// Return the correct article "a" or "an" used to modify a word
/// and return it capitalized, lower case, or upper case.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  WORD       I   is a word that should be modified by "a" or "an".
///  CASE       I   'U', 'L', or 'C' to specify capitalization of ANA.
///
///  The function returns the correct article, 'A' or 'AN', needed to
///  modify a word WORD, appropriately capitalized.
/// ```
///
/// # Detailed Input
///
/// ```text
///  WORD     is any English word for which you want to write the
///           correct phrase "a(an) response(answer)".  The case of the
///           letters of word do not matter.
///
///           Leading white space in word is ignored. The characters
///           " and ' are ignored.  Thus ''' apple '' ' and
///           '"apple"' and ' apple' and 'apple' are all treated as
///           the same word.
///
///  CASE     is a character that describes how the value returned in
///           ANA should be capitalized. The rules are:
///
///              'U'  ---  ANA is returned in all caps ( A, AN )
///              'C'  ---  ANA is returned capitalized ( A, An )
///              'L'  ---  ANA is returned lower case  ( a, an )
///
///           The case of CASE does not matter. Any value other than
///           those specified result in ANA being returned in all lower
///           case.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the correct indefinite article needed to
///  modify the word contained in WORD.
///
///  ANA should be declared to be
///
///     CHARACTER*(2)
///
///  (or CHARACTER*(N) where N > 1) in the calling program.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If the uppercase value of CASE is not 'U', 'C' or 'L', it
///      shall be treated as 'L'.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine allows you to construct grammatically correct phrases
///  when you need to modify a word by an indefinite article. Using
///  the pronunciations contained in the Webster's Ninth Collegiate
///  Dictionary, the phrase
///
///     ANA(WORD, CASE) // ' ' // WORD
///
///  will be grammatically correct.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose you wished to construct one of the messages
///
///     'a new file'
///     'an existing file'
///
///  and that the NEW/EXISTING word was in the variable WORD. Then
///  you could write
///
///     MESSAGE = ANA( WORD, 'L' ) // ' ' // WORD // ' file '
///     CALL CMPRSS ( ' ', 1, MESSAGE, MESSAGE )
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  Merriam-Webster (Ed.), "Webster's Ninth New Collegiate
///       Dictionary," 10th edition, 1990.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.3, 24-NOV-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.1.2, 28-FEB-2008 (BVS)
///
///         Corrected the contents of the $Required_Reading section.
///
/// -    SPICELIB Version 1.1.1, 22-SEP-2004 (EDW)
///
///         Added Copyright section.
///
/// -    SPICELIB Version 1.1.0, 18-JAN-2001 (WLT)
///
///         Made SCLK an "an" word.
///
/// -    SPICELIB Version 1.0.0, 29-NOV-1995 (WLT)
/// ```
pub fn ana(ctx: &mut SpiceContext, word: &str, case: &str, ana: &mut str) {
    ANA(
        word.as_bytes(),
        case.as_bytes(),
        fstr::StrBytes::new(ana).as_mut(),
        ctx.raw_context(),
    );
}

//$Procedure ANA ( AN or A ? )
pub fn ANA(WORD: &[u8], CASE: &[u8], ANA: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    UCASE(WORD, &mut save.MYWORD, ctx);
    REPLCH(&save.MYWORD.to_vec(), b"\'", b" ", &mut save.MYWORD);
    REPLCH(&save.MYWORD.to_vec(), b"\"", b" ", &mut save.MYWORD);
    LJUST(&save.MYWORD.to_vec(), &mut save.MYWORD);
    UCASE(CASE, &mut save.MYCASE, ctx);

    fstr::assign(ANA, b" ");

    if fstr::eq(&save.MYCASE, b"U") {
        save.CAPS = 1;
    } else if fstr::eq(&save.MYCASE, b"C") {
        save.CAPS = 2;
    } else {
        save.CAPS = 3;
    }

    //
    // Handle the obvious things first.
    //
    fstr::assign(&mut save.BEGIN, fstr::substr(&save.MYWORD, 1..=1));

    if (intrinsics::INDEX(b"AI", &save.BEGIN) > 0) {
        fstr::assign(ANA, save.AN.get(save.CAPS));
        return;
    } else if (intrinsics::INDEX(b"BCDGJKPQTVWYZ", &save.BEGIN) > 0) {
        fstr::assign(ANA, save.A.get(save.CAPS));
        return;
    }

    //
    // If we are still here, we need to be a bit more careful
    // in our determination of ANA.
    //
    // Get the beginnings of the input word.
    //
    for I in 1..=7 {
        fstr::assign(save.START.get_mut(I), fstr::substr(&save.MYWORD, 1..=I));
    }

    //
    // Now see if the start of the input word belongs to
    // one of the special collections.
    //
    for I in intrinsics::range(7, 2, -1) {
        if (ISRCHC(&save.START[I], AWRDS, save.AWORD.as_arg()) != 0) {
            fstr::assign(ANA, save.A.get(save.CAPS));
            return;
        }

        if (ISRCHC(&save.START[I], ANWRDS, save.ANWORD.as_arg()) != 0) {
            fstr::assign(ANA, save.AN.get(save.CAPS));
            return;
        }
    }

    //
    // If we got this far we can determine the ANA by
    // just looking at the beginning of the string.
    //
    if (intrinsics::INDEX(b"AEIOU", fstr::substr(&save.MYWORD, 1..=1)) > 0) {
        fstr::assign(ANA, save.AN.get(save.CAPS));
    } else {
        fstr::assign(ANA, save.A.get(save.CAPS));
    }
}
