//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LBCELL: i32 = -5;
const MSGLEN: i32 = 240;
const NUMCAS: i32 = 10;
const ITMWID: i32 = 30;
const NARWID: i32 = 2;
const MAXN: i32 = 10;
const MAXSET: i32 = 100;
const SMALLN: i32 = 2;
const LNSIZE: i32 = 80;

//$Procedure F_LPARSE ( SPICE list parser tests )
pub fn F_LPARSE(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut CHRITM = ActualCharArray::new(1, 1..=MAXN);
    let mut DELIM = [b' '; 1 as usize];
    let mut DELIMS = [b' '; LNSIZE as usize];
    let mut LIST = [b' '; LNSIZE as usize];
    let mut ITEMS = ActualCharArray::new(ITMWID, 1..=MAXN);
    let mut ITMSET = ActualCharArray::new(ITMWID, LBCELL..=MAXSET);
    let mut NARITM = ActualCharArray::new(NARWID, 1..=MAXN);
    let mut TITLE = [b' '; MSGLEN as usize];
    let mut XITEMS = ActualCharArray::new(ITMWID, 1..=MAXN);
    let mut N: i32 = 0;
    let mut NMAX: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Saved values
    //

    //
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_LPARSE", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"LPARSE test:  header example 1");

    testutil::TCASE(&TITLE, ctx)?;

    fstr::assign(&mut LIST, b"  A number of words   separated   by spaces   ");

    fstr::assign(XITEMS.get_mut(1), b"A");
    fstr::assign(XITEMS.get_mut(2), b"number");
    fstr::assign(XITEMS.get_mut(3), b"of");
    fstr::assign(XITEMS.get_mut(4), b"words");
    fstr::assign(XITEMS.get_mut(5), b"separated");
    fstr::assign(XITEMS.get_mut(6), b"by");
    fstr::assign(XITEMS.get_mut(7), b"spaces");

    spicelib::LPARSE(&LIST, b" ", MAXN, &mut N, ITEMS.as_arg_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", N, b"=", 7, 0, OK, ctx)?;

    testutil::CHCKAC(b"ITEMS", ITEMS.as_arg(), b"=", XITEMS.as_arg(), N, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"LPARSE test:  header example 1, with comma as delimiter.",
    );

    testutil::TCASE(&TITLE, ctx)?;

    fstr::assign(
        XITEMS.get_mut(1),
        b"A number of words   separated   by spaces",
    );

    fstr::assign(&mut DELIM, b",");
    spicelib::LPARSE(&LIST, &DELIM, MAXN, &mut N, ITEMS.as_arg_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    testutil::CHCKAC(b"ITEMS", ITEMS.as_arg(), b"=", XITEMS.as_arg(), N, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"LPARSE test:  header example 2");

    testutil::TCASE(&TITLE, ctx)?;

    fstr::assign(&mut LIST, b"//option1//option2/ //");

    fstr::assign(XITEMS.get_mut(1), b" ");
    fstr::assign(XITEMS.get_mut(2), b" ");
    fstr::assign(XITEMS.get_mut(3), b"option1");
    fstr::assign(XITEMS.get_mut(4), b" ");
    fstr::assign(XITEMS.get_mut(5), b"option2");
    fstr::assign(XITEMS.get_mut(6), b" ");
    fstr::assign(XITEMS.get_mut(7), b" ");
    fstr::assign(XITEMS.get_mut(8), b" ");

    //
    // Use RTRIM to remove trailing blanks from the list.
    //
    spicelib::LPARSE(
        fstr::substr(&LIST, 1..=spicelib::RTRIM(&LIST)),
        b"/",
        MAXN,
        &mut N,
        ITEMS.as_arg_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", N, b"=", 8, 0, OK, ctx)?;

    testutil::CHCKAC(b"ITEMS", ITEMS.as_arg(), b"=", XITEMS.as_arg(), N, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"LPARSE test:  header example 3");

    testutil::TCASE(&TITLE, ctx)?;

    fstr::assign(&mut LIST, b" ,bob,   carol,, ted,  alice");

    fstr::assign(XITEMS.get_mut(1), b" ");
    fstr::assign(XITEMS.get_mut(2), b"bob");
    fstr::assign(XITEMS.get_mut(3), b"carol");
    fstr::assign(XITEMS.get_mut(4), b" ");
    fstr::assign(XITEMS.get_mut(5), b"ted");
    fstr::assign(XITEMS.get_mut(6), b"alice");

    //
    // Use RTRIM to remove trailing blanks from the list.
    //
    spicelib::LPARSE(
        fstr::substr(&LIST, 1..=spicelib::RTRIM(&LIST)),
        b",",
        MAXN,
        &mut N,
        ITEMS.as_arg_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", N, b"=", 6, 0, OK, ctx)?;

    testutil::CHCKAC(b"ITEMS", ITEMS.as_arg(), b"=", XITEMS.as_arg(), N, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"LPARSE test:  header example 3, narrow item array",
    );

    testutil::TCASE(&TITLE, ctx)?;

    fstr::assign(&mut LIST, b" ,bob,   carol,, ted,  alice");

    fstr::assign(XITEMS.get_mut(1), b" ");
    fstr::assign(XITEMS.get_mut(2), b"bo");
    fstr::assign(XITEMS.get_mut(3), b"ca");
    fstr::assign(XITEMS.get_mut(4), b" ");
    fstr::assign(XITEMS.get_mut(5), b"te");
    fstr::assign(XITEMS.get_mut(6), b"al");

    //
    // Use RTRIM to remove trailing blanks from the list.
    //
    spicelib::LPARSE(
        fstr::substr(&LIST, 1..=spicelib::RTRIM(&LIST)),
        b",",
        MAXN,
        &mut N,
        NARITM.as_arg_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", N, b"=", 6, 0, OK, ctx)?;

    testutil::CHCKAC(b"ITEMS", NARITM.as_arg(), b"=", XITEMS.as_arg(), N, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"LPARSE test:  header example 3, 1-char item array",
    );

    testutil::TCASE(&TITLE, ctx)?;

    fstr::assign(&mut LIST, b" ,bob,   carol,, ted,  alice");

    fstr::assign(XITEMS.get_mut(1), b" ");
    fstr::assign(XITEMS.get_mut(2), b"b");
    fstr::assign(XITEMS.get_mut(3), b"c");
    fstr::assign(XITEMS.get_mut(4), b" ");
    fstr::assign(XITEMS.get_mut(5), b"t");
    fstr::assign(XITEMS.get_mut(6), b"a");

    //
    // Use RTRIM to remove trailing blanks from the list.
    //
    fstr::assign(&mut DELIM, b",");

    spicelib::LPARSE(
        fstr::substr(&LIST, 1..=spicelib::RTRIM(&LIST)),
        &DELIM,
        MAXN,
        &mut N,
        CHRITM.as_arg_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", N, b"=", 6, 0, OK, ctx)?;

    testutil::CHCKAC(b"ITEMS", CHRITM.as_arg(), b"=", XITEMS.as_arg(), N, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"LPARSE test:  header example 3, ITEMS dim = 2");

    testutil::TCASE(&TITLE, ctx)?;

    fstr::assign(&mut LIST, b" ,bob,   carol,, ted,  alice");

    fstr::assign(XITEMS.get_mut(1), b" ");
    fstr::assign(XITEMS.get_mut(2), b"bob");

    //
    // Use RTRIM to remove trailing blanks from the list.
    //
    spicelib::LPARSE(
        fstr::substr(&LIST, 1..=spicelib::RTRIM(&LIST)),
        b",",
        2,
        &mut N,
        ITEMS.as_arg_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", N, b"=", 2, 0, OK, ctx)?;

    testutil::CHCKAC(b"ITEMS", ITEMS.as_arg(), b"=", XITEMS.as_arg(), N, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"LPARSE test:  list with single non-blank delimiter",
    );

    testutil::TCASE(&TITLE, ctx)?;

    fstr::assign(&mut LIST, b",");

    fstr::assign(XITEMS.get_mut(1), b" ");
    fstr::assign(XITEMS.get_mut(2), b" ");

    //
    // Use RTRIM to remove trailing blanks from the list.
    //
    spicelib::LPARSE(
        fstr::substr(&LIST, 1..=spicelib::RTRIM(&LIST)),
        b",",
        MAXN,
        &mut N,
        ITEMS.as_arg_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", N, b"=", 2, 0, OK, ctx)?;

    testutil::CHCKAC(b"ITEMS", ITEMS.as_arg(), b"=", XITEMS.as_arg(), N, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"LPARSE test:  blank list, non-blank delimiter");

    testutil::TCASE(&TITLE, ctx)?;

    fstr::assign(&mut LIST, b" ");

    fstr::assign(XITEMS.get_mut(1), b" ");

    //
    // Use RTRIM to remove trailing blanks from the list.
    //
    spicelib::LPARSE(&LIST, b",", MAXN, &mut N, ITEMS.as_arg_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    testutil::CHCKAC(b"ITEMS", ITEMS.as_arg(), b"=", XITEMS.as_arg(), N, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"LPARSE test:  blank list, blank delimiter");

    testutil::TCASE(&TITLE, ctx)?;

    fstr::assign(&mut LIST, b" ");

    fstr::assign(XITEMS.get_mut(1), b" ");

    //
    // Use RTRIM to remove trailing blanks from the list.
    //
    spicelib::LPARSE(&LIST, b" ", MAXN, &mut N, ITEMS.as_arg_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    testutil::CHCKAC(b"ITEMS", ITEMS.as_arg(), b"=", XITEMS.as_arg(), N, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"LPARSE test:  consecutive non-blank delimiters",
    );

    testutil::TCASE(&TITLE, ctx)?;

    fstr::assign(&mut LIST, b",,,,,,,");

    fstr::assign(XITEMS.get_mut(1), b" ");
    fstr::assign(XITEMS.get_mut(2), b" ");
    fstr::assign(XITEMS.get_mut(3), b" ");
    fstr::assign(XITEMS.get_mut(4), b" ");
    fstr::assign(XITEMS.get_mut(5), b" ");
    fstr::assign(XITEMS.get_mut(6), b" ");
    fstr::assign(XITEMS.get_mut(7), b" ");
    fstr::assign(XITEMS.get_mut(8), b" ");

    //
    // Use RTRIM to remove trailing blanks from the list.
    //
    fstr::assign(&mut DELIM, b",");

    spicelib::LPARSE(&LIST, &DELIM, MAXN, &mut N, ITEMS.as_arg_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", N, b"=", 8, 0, OK, ctx)?;

    testutil::CHCKAC(b"ITEMS", ITEMS.as_arg(), b"=", XITEMS.as_arg(), N, OK, ctx)?;

    //*************************************************************
    //
    //
    //     LPARSM tests follow.
    //
    //
    //*************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"LPARSM test:  header example 1");

    testutil::TCASE(&TITLE, ctx)?;

    fstr::assign(&mut LIST, b"  A number of words   separated   by spaces   ");

    fstr::assign(XITEMS.get_mut(1), b"A");
    fstr::assign(XITEMS.get_mut(2), b"number");
    fstr::assign(XITEMS.get_mut(3), b"of");
    fstr::assign(XITEMS.get_mut(4), b"words");
    fstr::assign(XITEMS.get_mut(5), b"separated");
    fstr::assign(XITEMS.get_mut(6), b"by");
    fstr::assign(XITEMS.get_mut(7), b"spaces");

    spicelib::LPARSM(&LIST, b" ", MAXN, &mut N, ITEMS.as_arg_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", N, b"=", 7, 0, OK, ctx)?;

    testutil::CHCKAC(b"ITEMS", ITEMS.as_arg(), b"=", XITEMS.as_arg(), N, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"LPARSM test:  header example 2");

    testutil::TCASE(&TITLE, ctx)?;

    fstr::assign(&mut LIST, b"  1986-187// 13:15:12.184 ");

    fstr::assign(&mut DELIMS, b" ,/-:");

    //
    // Use NMAX rather than the "standard" MAXN.
    //
    NMAX = 20;

    fstr::assign(XITEMS.get_mut(1), b"1986");
    fstr::assign(XITEMS.get_mut(2), b"187");
    fstr::assign(XITEMS.get_mut(3), b" ");
    fstr::assign(XITEMS.get_mut(4), b"13");
    fstr::assign(XITEMS.get_mut(5), b"15");
    fstr::assign(XITEMS.get_mut(6), b"12.184");

    spicelib::LPARSM(&LIST, &DELIMS, NMAX, &mut N, ITEMS.as_arg_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", N, b"=", 6, 0, OK, ctx)?;

    testutil::CHCKAC(b"ITEMS", ITEMS.as_arg(), b"=", XITEMS.as_arg(), N, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"LPARSM test:  LPARSE header example 3");

    testutil::TCASE(&TITLE, ctx)?;

    fstr::assign(&mut LIST, b" ,bob,   carol,, ted,  alice");

    fstr::assign(XITEMS.get_mut(1), b" ");
    fstr::assign(XITEMS.get_mut(2), b"bob");
    fstr::assign(XITEMS.get_mut(3), b"carol");
    fstr::assign(XITEMS.get_mut(4), b" ");
    fstr::assign(XITEMS.get_mut(5), b"ted");
    fstr::assign(XITEMS.get_mut(6), b"alice");

    //
    // Use RTRIM to remove trailing blanks from the list.
    //
    spicelib::LPARSM(
        fstr::substr(&LIST, 1..=spicelib::RTRIM(&LIST)),
        b",",
        MAXN,
        &mut N,
        ITEMS.as_arg_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", N, b"=", 6, 0, OK, ctx)?;

    testutil::CHCKAC(b"ITEMS", ITEMS.as_arg(), b"=", XITEMS.as_arg(), N, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"LPARSM test:  LPARSE header example 3 with narrow item array",
    );

    testutil::TCASE(&TITLE, ctx)?;

    fstr::assign(&mut LIST, b" ,bob,   carol,, ted,  alice");

    fstr::assign(XITEMS.get_mut(1), b" ");
    fstr::assign(XITEMS.get_mut(2), b"bo");
    fstr::assign(XITEMS.get_mut(3), b"ca");
    fstr::assign(XITEMS.get_mut(4), b" ");
    fstr::assign(XITEMS.get_mut(5), b"te");
    fstr::assign(XITEMS.get_mut(6), b"al");

    //
    // Use RTRIM to remove trailing blanks from the list.
    //
    spicelib::LPARSM(
        fstr::substr(&LIST, 1..=spicelib::RTRIM(&LIST)),
        b",",
        MAXN,
        &mut N,
        NARITM.as_arg_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", N, b"=", 6, 0, OK, ctx)?;

    testutil::CHCKAC(b"ITEMS", NARITM.as_arg(), b"=", XITEMS.as_arg(), N, OK, ctx)?;

    //

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"LPARSM test:  LPARSE header example 3, with blank and comma both considered delimiters",
    );

    testutil::TCASE(&TITLE, ctx)?;

    fstr::assign(&mut LIST, b" ,bob,   carol,, ted,  alice");

    fstr::assign(&mut DELIMS, b" ,");

    fstr::assign(XITEMS.get_mut(1), b" ");
    fstr::assign(XITEMS.get_mut(2), b"bob");
    fstr::assign(XITEMS.get_mut(3), b"carol");
    fstr::assign(XITEMS.get_mut(4), b" ");
    fstr::assign(XITEMS.get_mut(5), b"ted");
    fstr::assign(XITEMS.get_mut(6), b"alice");

    //
    // Use RTRIM to remove trailing blanks from the list.
    //
    spicelib::LPARSM(
        fstr::substr(&LIST, 1..=spicelib::RTRIM(&LIST)),
        &DELIMS,
        MAXN,
        &mut N,
        ITEMS.as_arg_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", N, b"=", 6, 0, OK, ctx)?;

    testutil::CHCKAC(b"ITEMS", ITEMS.as_arg(), b"=", XITEMS.as_arg(), N, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"LPARSE test:  list with single non-blank delimiter",
    );

    testutil::TCASE(&TITLE, ctx)?;

    fstr::assign(&mut LIST, b",");

    fstr::assign(XITEMS.get_mut(1), b" ");
    fstr::assign(XITEMS.get_mut(2), b" ");

    //
    // Use RTRIM to remove trailing blanks from the list.
    //
    spicelib::LPARSM(
        fstr::substr(&LIST, 1..=spicelib::RTRIM(&LIST)),
        b",",
        MAXN,
        &mut N,
        ITEMS.as_arg_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", N, b"=", 2, 0, OK, ctx)?;

    testutil::CHCKAC(b"ITEMS", ITEMS.as_arg(), b"=", XITEMS.as_arg(), N, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"LPARSM test:  list starting and ending with single non-blank delimiter",
    );

    testutil::TCASE(&TITLE, ctx)?;

    fstr::assign(&mut DELIMS, b",:");
    fstr::assign(&mut LIST, b", :");

    fstr::assign(XITEMS.get_mut(1), b" ");
    fstr::assign(XITEMS.get_mut(2), b" ");
    fstr::assign(XITEMS.get_mut(3), b" ");

    //
    // Use RTRIM to remove trailing blanks from the list.
    //
    spicelib::LPARSM(
        fstr::substr(&LIST, 1..=spicelib::RTRIM(&LIST)),
        &DELIMS,
        MAXN,
        &mut N,
        ITEMS.as_arg_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", N, b"=", 3, 0, OK, ctx)?;

    testutil::CHCKAC(b"ITEMS", ITEMS.as_arg(), b"=", XITEMS.as_arg(), N, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"LPARSM test:  consecutive non-blank delimiters",
    );

    testutil::TCASE(&TITLE, ctx)?;

    fstr::assign(&mut LIST, b",,,,,,,");

    fstr::assign(XITEMS.get_mut(1), b" ");
    fstr::assign(XITEMS.get_mut(2), b" ");
    fstr::assign(XITEMS.get_mut(3), b" ");
    fstr::assign(XITEMS.get_mut(4), b" ");
    fstr::assign(XITEMS.get_mut(5), b" ");
    fstr::assign(XITEMS.get_mut(6), b" ");
    fstr::assign(XITEMS.get_mut(7), b" ");
    fstr::assign(XITEMS.get_mut(8), b" ");

    //
    // Use RTRIM to remove trailing blanks from the list.
    //
    fstr::assign(&mut DELIMS, b",");

    spicelib::LPARSM(&LIST, &DELIMS, MAXN, &mut N, ITEMS.as_arg_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", N, b"=", 8, 0, OK, ctx)?;

    testutil::CHCKAC(b"ITEMS", ITEMS.as_arg(), b"=", XITEMS.as_arg(), N, OK, ctx)?;

    //*************************************************************
    //
    //
    //     LPARSS tests follow.
    //
    //
    //*************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"LPARSS test:  header example 1");

    testutil::TCASE(&TITLE, ctx)?;

    //
    // Initialize item set.
    //
    spicelib::SSIZEC(MAXN, ITMSET.as_arg_mut(), ctx)?;

    fstr::assign(&mut LIST, b"  A number of words   separated   by spaces.");

    fstr::assign(XITEMS.get_mut(1), b" ");
    fstr::assign(XITEMS.get_mut(2), b"A");
    fstr::assign(XITEMS.get_mut(3), b"by");
    fstr::assign(XITEMS.get_mut(4), b"number");
    fstr::assign(XITEMS.get_mut(5), b"of");
    fstr::assign(XITEMS.get_mut(6), b"separated");
    fstr::assign(XITEMS.get_mut(7), b"spaces");
    fstr::assign(XITEMS.get_mut(8), b"words");

    fstr::assign(&mut DELIMS, b" ,.");

    spicelib::LPARSS(&LIST, &DELIMS, ITMSET.as_arg_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    N = spicelib::CARDC(ITMSET.as_arg(), ctx)?;

    testutil::CHCKSI(b"CARDC(ITMSET)", N, b"=", 8, 0, OK, ctx)?;

    testutil::CHCKAC(
        b"ITEMS",
        ITMSET.subarray(1),
        b"=",
        XITEMS.as_arg(),
        N,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"LPARSS test:  header example 2");

    testutil::TCASE(&TITLE, ctx)?;

    fstr::assign(&mut LIST, b"  1986-187// 13:15:12.184 ");
    fstr::assign(&mut DELIMS, b" ,/-:");

    fstr::assign(XITEMS.get_mut(1), b" ");
    fstr::assign(XITEMS.get_mut(2), b"12.184");
    fstr::assign(XITEMS.get_mut(3), b"13");
    fstr::assign(XITEMS.get_mut(4), b"15");
    fstr::assign(XITEMS.get_mut(5), b"187");
    fstr::assign(XITEMS.get_mut(6), b"1986");

    spicelib::LPARSS(&LIST, &DELIMS, ITMSET.as_arg_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    N = spicelib::CARDC(ITMSET.as_arg(), ctx)?;

    testutil::CHCKSI(b"CARDC(ITMSET)", N, b"=", 6, 0, OK, ctx)?;

    testutil::CHCKAC(
        b"ITEMS",
        ITMSET.subarray(1),
        b"=",
        XITEMS.as_arg(),
        N,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"LPARSS test:  header example 3");

    testutil::TCASE(&TITLE, ctx)?;

    fstr::assign(&mut LIST, b"  ,This,  is, ,an,, example, ");
    fstr::assign(&mut DELIMS, b" ,");

    fstr::assign(XITEMS.get_mut(1), b" ");
    fstr::assign(XITEMS.get_mut(2), b"This");
    fstr::assign(XITEMS.get_mut(3), b"an");
    fstr::assign(XITEMS.get_mut(4), b"example");
    fstr::assign(XITEMS.get_mut(5), b"is");

    spicelib::LPARSS(&LIST, &DELIMS, ITMSET.as_arg_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    N = spicelib::CARDC(ITMSET.as_arg(), ctx)?;

    testutil::CHCKSI(b"CARDC(ITMSET)", N, b"=", 5, 0, OK, ctx)?;

    testutil::CHCKAC(
        b"ITEMS",
        ITMSET.subarray(1),
        b"=",
        XITEMS.as_arg(),
        N,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"LPARSS test:  header example 4");

    testutil::TCASE(&TITLE, ctx)?;

    fstr::assign(
        &mut LIST,
        b"Mary had a little lamb, little lamb whose fleece was white      as snow.",
    );
    fstr::assign(&mut DELIMS, b" ,.");

    spicelib::SSIZEC(6, ITMSET.as_arg_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LPARSS(&LIST, &DELIMS, ITMSET.as_arg_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(SETEXCESS)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"LPARSS test:  header example 5");

    testutil::TCASE(&TITLE, ctx)?;

    fstr::assign(&mut LIST, b"1 2 3 4 5 6 7 8 9 10.");
    fstr::assign(&mut DELIMS, b" .");

    spicelib::SSIZEC(10, ITMSET.as_arg_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LPARSS(&LIST, &DELIMS, ITMSET.as_arg_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(SETEXCESS)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"LPARSS test:  header example 6");

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::SSIZEC(100, ITMSET.as_arg_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut LIST, b"1 2 3 4 5 6 7 8 9 10.");
    fstr::assign(&mut DELIMS, b".");

    fstr::assign(XITEMS.get_mut(1), b" ");
    fstr::assign(XITEMS.get_mut(2), b"1 2 3 4 5 6 7 8 9 10");

    spicelib::LPARSS(
        &LIST,
        fstr::substr(&DELIMS, 1..=spicelib::RTRIM(&DELIMS)),
        ITMSET.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    N = spicelib::CARDC(ITMSET.as_arg(), ctx)?;

    testutil::CHCKSI(b"CARDC(ITMSET)", N, b"=", 2, 0, OK, ctx)?;

    testutil::CHCKAC(
        b"ITEMS",
        ITMSET.subarray(1),
        b"=",
        XITEMS.as_arg(),
        N,
        OK,
        ctx,
    )?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
