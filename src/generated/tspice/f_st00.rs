//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const TSTSTR: &[u8] = b" Now is the time,   for all good men     to come.";
const ARRZS: i32 = 30;
const LINSZ: i32 = 49;
const NDIM: i32 = 2;
const NWORDS: i32 = 10;
const OUTSZ: i32 = 20;
const WRDSZ: i32 = 5;

struct SaveVars {
    ARRAY: ActualCharArray,
    DATARR: ActualCharArray,
    NEXT: Vec<u8>,
    REST: Vec<u8>,
    STRING: Vec<u8>,
    OUT: Vec<u8>,
    WORD: Vec<u8>,
    XARRAY: ActualCharArray,
    XPWORD: ActualCharArray,
    XPREST: ActualCharArray,
    LOC: i32,
    XPLOC: StackArray<i32, 10>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ARRAY = ActualCharArray::new(ARRZS, 1..=NDIM);
        let mut DATARR = ActualCharArray::new(ARRZS, 1..=NDIM);
        let mut NEXT = vec![b' '; WRDSZ as usize];
        let mut REST = vec![b' '; LINSZ as usize];
        let mut STRING = vec![b' '; LINSZ as usize];
        let mut OUT = vec![b' '; OUTSZ as usize];
        let mut WORD = vec![b' '; WRDSZ as usize];
        let mut XARRAY = ActualCharArray::new(ARRZS, 1..=NDIM);
        let mut XPWORD = ActualCharArray::new(WRDSZ, 1..=NWORDS);
        let mut XPREST = ActualCharArray::new(LINSZ, 1..=(NWORDS - 1));
        let mut LOC: i32 = 0;
        let mut XPLOC = StackArray::<i32, 10>::new(1..=NWORDS);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"0123456789"), Val::C(b"0123456789")].into_iter();
            DATARR
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"                              "),
                Val::C(b"                              "),
            ]
            .into_iter();
            XARRAY
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(2),
                Val::I(6),
                Val::I(9),
                Val::I(13),
                Val::I(21),
                Val::I(25),
                Val::I(29),
                Val::I(34),
                Val::I(42),
                Val::I(45),
            ]
            .into_iter();
            XPLOC
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"Now"),
                Val::C(b"is"),
                Val::C(b"the"),
                Val::C(b"time,"),
                Val::C(b"for"),
                Val::C(b"all"),
                Val::C(b"good"),
                Val::C(b"men"),
                Val::C(b"to"),
                Val::C(b"come."),
            ]
            .into_iter();
            XPWORD
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"is the time,   for all good men     to come."),
                Val::C(b"the time,   for all good men     to come."),
                Val::C(b"time,   for all good men     to come."),
                Val::C(b"for all good men     to come."),
                Val::C(b"all good men     to come."),
                Val::C(b"good men     to come."),
                Val::C(b"men     to come."),
                Val::C(b"to come."),
                Val::C(b"come."),
            ]
            .into_iter();
            XPREST
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            ARRAY,
            DATARR,
            NEXT,
            REST,
            STRING,
            OUT,
            WORD,
            XARRAY,
            XPWORD,
            XPREST,
            LOC,
            XPLOC,
        }
    }
}

//$Procedure F_ST00 ( Tests for string routines, subset 00 )
pub fn F_ST00(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Test Utility Functions
    //

    //
    // SPICELIB Functions
    //

    //
    // Local Parameters
    //

    //
    // Local variables.
    //

    //
    // Test data.
    //

    //
    // Save all variables.
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ST00", ctx)?;

    //
    // *****************************************************************
    //
    //    Normal cases: CLEARC
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"CLEARC: NDIM < 1", ctx)?;

    fstr::assign(save.ARRAY.get_mut(1), save.DATARR.get(1));
    fstr::assign(save.ARRAY.get_mut(2), save.DATARR.get(2));

    spicelib::CLEARC(0, save.ARRAY.as_arg_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAC(
        b"ARRAY",
        save.ARRAY.as_arg(),
        b"=",
        save.DATARR.as_arg(),
        2,
        OK,
        ctx,
    )?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"CLEARC: Clear an array of 2 dimensions", ctx)?;

    spicelib::CLEARC(NDIM, save.ARRAY.as_arg_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAC(
        b"ARRAY",
        save.ARRAY.as_arg(),
        b"=",
        save.XARRAY.as_arg(),
        2,
        OK,
        ctx,
    )?;

    //
    // *****************************************************************
    //
    //    Error cases: INSSUB
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"INSSUB: Insertion before the beginning of the string.",
        ctx,
    )?;

    spicelib::INSSUB(b"Apple a day", b"An ", 0, &mut save.OUT, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDINDEX)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"INSSUB: Insertion after the end of the string.", ctx)?;

    spicelib::INSSUB(b"An Apple a day", b" keeps", 16, &mut save.OUT, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDINDEX)", OK, ctx)?;

    //
    // *****************************************************************
    //
    //    Normal cases: INSSUB
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    //    Lets use WORD, which has a declared size of 5 characters.
    //
    testutil::TCASE(b"INSSUB: Insertion happens after the end of output string, with different input and output strings.", ctx)?;

    spicelib::INSSUB(b"012345", b"6789", 6, &mut save.WORD, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(b"WORD", &save.WORD, b"=", b"01234", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"INSSUB: Insertion happens after the end of output string, with equal input and output strings.", ctx)?;

    fstr::assign(&mut save.WORD, b"01234");

    spicelib::INSSUB(b"012345", b"6789", 6, &mut save.WORD, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(b"WORD", &save.WORD, b"=", b"01234", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"INSSUB: Insertion happens at the beginning of the input string.",
        ctx,
    )?;

    spicelib::INSSUB(b" other woman", b"The", 1, &mut save.OUT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(b"OUT", &save.OUT, b"=", b"The other woman     ", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"INSSUB: Insertion happens in the middle of the input string.",
        ctx,
    )?;

    spicelib::INSSUB(b"The rabbit", b"best ", 5, &mut save.OUT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(b"OUT", &save.OUT, b"=", b"The best rabbit     ", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"INSSUB: Insertion happens at the end of the input string.",
        ctx,
    )?;

    spicelib::INSSUB(b"An Apple a day", b" keeps", 15, &mut save.OUT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(b"OUT", &save.OUT, b"=", b"An Apple a day keeps", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"INSSUB: Insertion causes truncation of original substring.",
        ctx,
    )?;

    spicelib::INSSUB(
        b"An Apple a day",
        b" keeps the doctor",
        15,
        &mut save.OUT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(b"OUT", &save.OUT, b"=", b"An Apple a day keeps", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"INSSUB: Insertion causes truncation of final part of input string.",
        ctx,
    )?;

    spicelib::INSSUB(b"An a day keeps the", b" Apple", 3, &mut save.OUT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(b"OUT", &save.OUT, b"=", b"An Apple a day keeps", OK, ctx)?;

    //
    // *****************************************************************
    //
    //    Normal cases: NEXTWD
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"NEXTWD: Trivial case.", ctx)?;

    spicelib::NEXTWD(b" ", &mut save.NEXT, &mut save.REST);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(b"NEXT", &save.NEXT, b"=", b" ", OK, ctx)?;
    testutil::CHCKSC(b"REST", &save.REST, b"=", b" ", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"NEXTWD: Non trivial case.", ctx)?;

    fstr::assign(&mut save.STRING, TSTSTR);

    for I in 1..=(NWORDS - 1) {
        spicelib::NEXTWD(&save.STRING, &mut save.NEXT, &mut save.REST);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSC(b"NEXT", &save.NEXT, b"=", &save.XPWORD[I], OK, ctx)?;
        testutil::CHCKSC(b"REST", &save.REST, b"=", &save.XPREST[I], OK, ctx)?;

        fstr::assign(&mut save.STRING, &save.REST);
    }

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"NEXTWD: Non trivial case. Empty rest string.", ctx)?;

    spicelib::NEXTWD(&save.XPWORD[NWORDS], &mut save.NEXT, &mut save.REST);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(b"NEXT", &save.NEXT, b"=", &save.XPWORD[NWORDS], OK, ctx)?;
    testutil::CHCKSC(b"REST", &save.REST, b"=", b" ", OK, ctx)?;

    //
    // *****************************************************************
    //
    //    Normal cases: NTHWD
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"NTHWD: Trivial case (blank string)", ctx)?;

    spicelib::NTHWD(b" ", 10, &mut save.WORD, &mut save.LOC);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(b"WORD", &save.WORD, b"=", b" ", OK, ctx)?;
    testutil::CHCKSI(b"LOC", save.LOC, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"NTHWD: NTH nonpositive", ctx)?;

    spicelib::NTHWD(TSTSTR, 0, &mut save.WORD, &mut save.LOC);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(b"WORD", &save.WORD, b"=", b" ", OK, ctx)?;
    testutil::CHCKSI(b"LOC", save.LOC, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"NTHWD: Non trivial case.", ctx)?;

    for I in 1..=NWORDS {
        spicelib::NTHWD(TSTSTR, I, &mut save.WORD, &mut save.LOC);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSC(b"WORD", &save.WORD, b"=", &save.XPWORD[I], OK, ctx)?;
        testutil::CHCKSI(b"LOC", save.LOC, b"=", save.XPLOC[I], 0, OK, ctx)?;
    }

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"NTHWD: NTH too large -- greater than number of words on input string.",
        ctx,
    )?;

    spicelib::NTHWD(TSTSTR, 30, &mut save.WORD, &mut save.LOC);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(b"WORD", &save.WORD, b"=", b" ", OK, ctx)?;
    testutil::CHCKSI(b"LOC", save.LOC, b"=", 0, 0, OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
