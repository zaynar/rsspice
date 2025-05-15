//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXTKN: i32 = 64;
const PICLEN: i32 = (MAXTKN * 5);
const WDSIZE: i32 = MAXTKN;
const LNSIZE: i32 = PICLEN;
const ERRSIZ: i32 = (PICLEN * 2);

//$Procedure      F_ZZTIME ( Family of tests for ZZTIME )
pub fn F_ZZTIME(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut PIECES = ActualCharArray::new(WDSIZE, 1..=87);
    let mut W1 = [b' '; WDSIZE as usize];
    let mut W2 = [b' '; WDSIZE as usize];
    let mut M1 = [b' '; 2 as usize];
    let mut M2 = [b' '; 2 as usize];
    let mut CMLINE = [b' '; LNSIZE as usize];
    let mut STRING = [b' '; ERRSIZ as usize];
    let mut MSTYLE = [b' '; LNSIZE as usize];
    let mut GOOD = [b' '; LNSIZE as usize];
    let mut BAD = [b' '; LNSIZE as usize];
    let mut PIC = [b' '; LNSIZE as usize];
    let mut ERROR = [b' '; ERRSIZ as usize];
    let mut EXPERR = [b' '; ERRSIZ as usize];
    let mut TKNERR = [b' '; ERRSIZ as usize];
    let mut PICERR = [b' '; ERRSIZ as usize];
    let mut REP = [b' '; WDSIZE as usize];
    let mut EREP = [b' '; WDSIZE as usize];
    let mut TRANSL = [b' '; WDSIZE as usize];
    let mut ETRANS = [b' '; WDSIZE as usize];
    let mut ONECHR = [b' '; 1 as usize];
    let mut TVEC = StackArray::<f64, 10>::new(1..=10);
    let mut ETVEC = StackArray::<f64, 10>::new(1..=10);
    let mut EXPN: i32 = 0;
    let mut B: i32 = 0;
    let mut E: i32 = 0;
    let mut NTVEC: i32 = 0;
    let mut LAST: i32 = 0;
    let mut SHDIAG: bool = false;
    let mut DID: bool = false;
    let mut GOT: bool = false;
    let mut L2R: bool = false;
    let mut R2L: bool = false;
    let mut YABBRV: bool = false;

    //
    // Test Utility Functions
    //

    //
    // SPICELIB Functions
    //

    //
    // Parameters from ZZTIME: maximum number of tokens that a valid
    // time string can contain and length of the string buffer
    // containing the time string picture.
    //

    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZTIME", ctx)?;

    fstr::assign(&mut MSTYLE, b"LEFT 5 RIGHT 75 FLAG Diagnostic: ");
    L2R = true;
    R2L = !L2R;
    YABBRV = false;

    support::GETCML(&mut CMLINE, ctx);
    spicelib::UCASE(&CMLINE.clone(), &mut CMLINE, ctx);

    SHDIAG = (intrinsics::INDEX(&CMLINE, b"-DIAGS") > 0);

    testutil::TCASE(
        b"Check that that tokenization process behaves as expected. ",
        ctx,
    )?;

    fstr::assign(PIECES.get_mut(1), b"PDT Z");
    fstr::assign(PIECES.get_mut(2), b"pst Z");
    fstr::assign(PIECES.get_mut(3), b"CDT Z");
    fstr::assign(PIECES.get_mut(4), b"CST Z");
    fstr::assign(PIECES.get_mut(5), b"MST Z");
    fstr::assign(PIECES.get_mut(6), b"MDT Z");
    fstr::assign(PIECES.get_mut(7), b"EST Z");
    fstr::assign(PIECES.get_mut(8), b"EDT Z");
    fstr::assign(PIECES.get_mut(9), b"UTC+ O");
    fstr::assign(PIECES.get_mut(10), b"UTC- o");
    fstr::assign(PIECES.get_mut(11), b"TDT s");
    fstr::assign(PIECES.get_mut(12), b"TDB s");
    fstr::assign(PIECES.get_mut(13), b"JD j");
    fstr::assign(PIECES.get_mut(14), b"JD j");
    fstr::assign(PIECES.get_mut(15), b"( [");
    fstr::assign(PIECES.get_mut(16), b") ]");
    fstr::assign(PIECES.get_mut(17), b". .");
    fstr::assign(PIECES.get_mut(18), b": :");
    fstr::assign(PIECES.get_mut(19), b":: d");
    fstr::assign(PIECES.get_mut(20), b"// d");
    fstr::assign(PIECES.get_mut(21), b"/ /");
    fstr::assign(PIECES.get_mut(22), b"January m");
    fstr::assign(PIECES.get_mut(23), b"February m");
    fstr::assign(PIECES.get_mut(24), b"March m");
    fstr::assign(PIECES.get_mut(25), b"April m");
    fstr::assign(PIECES.get_mut(26), b"May m");
    fstr::assign(PIECES.get_mut(27), b"June m");
    fstr::assign(PIECES.get_mut(28), b"July m");
    fstr::assign(PIECES.get_mut(29), b"August m");
    fstr::assign(PIECES.get_mut(30), b"September m");
    fstr::assign(PIECES.get_mut(31), b"October m");
    fstr::assign(PIECES.get_mut(32), b"November m");
    fstr::assign(PIECES.get_mut(33), b"December m");
    fstr::assign(PIECES.get_mut(34), b"jan, m,");
    fstr::assign(PIECES.get_mut(35), b"feb m");
    fstr::assign(PIECES.get_mut(36), b"mar, m,");
    fstr::assign(PIECES.get_mut(37), b"apr m");
    fstr::assign(PIECES.get_mut(38), b"may m");
    fstr::assign(PIECES.get_mut(39), b"jun, m,");
    fstr::assign(PIECES.get_mut(40), b"jul m");
    fstr::assign(PIECES.get_mut(41), b"aug, m,");
    fstr::assign(PIECES.get_mut(42), b"sept, m,");
    fstr::assign(PIECES.get_mut(43), b"oct, m,");
    fstr::assign(PIECES.get_mut(44), b"nov, m,");
    fstr::assign(PIECES.get_mut(45), b"dec, m,");
    fstr::assign(PIECES.get_mut(46), b"mon. w.");
    fstr::assign(PIECES.get_mut(47), b"tues. w.");
    fstr::assign(PIECES.get_mut(48), b"wed. w.");
    fstr::assign(PIECES.get_mut(49), b"thu w");
    fstr::assign(PIECES.get_mut(50), b"fri. w.");
    fstr::assign(PIECES.get_mut(51), b"sat. w.");
    fstr::assign(PIECES.get_mut(52), b"sun. w.");
    fstr::assign(PIECES.get_mut(53), b"a.d. e");
    fstr::assign(PIECES.get_mut(54), b"ad e");
    fstr::assign(PIECES.get_mut(55), b"b.c. e");
    fstr::assign(PIECES.get_mut(56), b"bc e");
    fstr::assign(PIECES.get_mut(57), b"1768 i");
    fstr::assign(PIECES.get_mut(58), b"12 i");
    fstr::assign(PIECES.get_mut(59), b"1 i");
    fstr::assign(PIECES.get_mut(60), b"2 i");
    fstr::assign(PIECES.get_mut(61), b"3 i");
    fstr::assign(PIECES.get_mut(62), b"4 i");
    fstr::assign(PIECES.get_mut(63), b"5 i");
    fstr::assign(PIECES.get_mut(64), b"6 i");
    fstr::assign(PIECES.get_mut(65), b"7 i");
    fstr::assign(PIECES.get_mut(66), b"8 i");
    fstr::assign(PIECES.get_mut(67), b"9 i");
    fstr::assign(PIECES.get_mut(68), b"0 i");
    fstr::assign(PIECES.get_mut(69), b"10 i");
    fstr::assign(PIECES.get_mut(70), b"21 i");
    fstr::assign(PIECES.get_mut(71), b"22 i");
    fstr::assign(PIECES.get_mut(72), b"23 i");
    fstr::assign(PIECES.get_mut(73), b"24 i");
    fstr::assign(PIECES.get_mut(74), b"25 i");
    fstr::assign(PIECES.get_mut(75), b". .");
    fstr::assign(PIECES.get_mut(76), b", ,");
    fstr::assign(PIECES.get_mut(77), b"\' \'");
    fstr::assign(PIECES.get_mut(78), b"- -");
    fstr::assign(PIECES.get_mut(79), b"am N");
    fstr::assign(PIECES.get_mut(80), b"a.m. N");
    fstr::assign(PIECES.get_mut(81), b"pm N");
    fstr::assign(PIECES.get_mut(82), b"p.m. N");
    fstr::assign(PIECES.get_mut(83), b"t t");
    fstr::assign(PIECES.get_mut(84), b"T t");
    fstr::assign(PIECES.get_mut(85), b"TDB s");
    fstr::assign(PIECES.get_mut(86), b"TDT s");
    fstr::assign(PIECES.get_mut(87), b"UTC s");

    for I in 80..=87 {
        spicelib::NEXTWD(&PIECES[I], &mut W1, &mut M1);

        for J in 1..=20 {
            spicelib::NEXTWD(&PIECES[J], &mut W2, &mut M2);

            fstr::assign(&mut EREP, &M1);
            spicelib::SUFFIX(&M2, 0, &mut EREP);

            fstr::assign(&mut STRING, &W1);
            spicelib::SUFFIX(&W2, 0, &mut STRING);

            if fstr::eq(&EREP, b"ii") {
                fstr::assign(&mut EREP, b"i");
            } else if fstr::eq(&STRING, b"::") {
                fstr::assign(&mut EREP, b"d");
            } else if fstr::eq(&STRING, b":::") {
                fstr::assign(&mut EREP, b"d:");
            } else if fstr::eq(&STRING, b"//") {
                fstr::assign(&mut EREP, b"d");
            } else if fstr::eq(&STRING, b"///") {
                fstr::assign(&mut EREP, b"d/");
            } else if fstr::eq(&STRING, b"UTC-") {
                fstr::assign(&mut EREP, b"o");
            }

            DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);
            GOT = spicelib::ZZGREP(&mut REP, ctx);

            testutil::TSTMSG(b"#", b"The value of string was: \"#\"", ctx);
            testutil::TSTMSC(&STRING, ctx);

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSL(b"DID", DID, true, OK, ctx)?;
            testutil::CHCKSL(b"GOT", GOT, true, OK, ctx)?;
            testutil::CHCKSC(b"REP", &REP, b"=", &EREP, OK, ctx)?;

            fstr::assign(&mut EREP, &M1);
            spicelib::SUFFIX(b"b", 0, &mut EREP);
            spicelib::SUFFIX(&M2, 0, &mut EREP);
            fstr::assign(&mut STRING, &W1);

            spicelib::SUFFIX(&W2, 3, &mut STRING);
            DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);
            GOT = spicelib::ZZGREP(&mut REP, ctx);

            testutil::TSTMSG(b"#", b"The value of string was: \"#\"", ctx);
            testutil::TSTMSC(&STRING, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSL(b"DID", DID, true, OK, ctx)?;
            testutil::CHCKSL(b"GOT", GOT, true, OK, ctx)?;
            testutil::CHCKSC(b"REP", &REP, b"=", &EREP, OK, ctx)?;
        }
    }

    testutil::TSTMSG(b" ", b" ", ctx);

    testutil::TCASE(b"Make sure that ZZNOTE can retrieve components from a date properly and that the representation is properly reduced. ", ctx)?;

    fstr::assign(&mut STRING, b"12 JAN 1992 A.D., 11:12:18 P.M.");
    fstr::assign(&mut EREP, b"ibmbibe,bi:i:ibN");

    fstr::assign(&mut ERROR, b"xxx");

    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"DID", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", b" ", OK, ctx)?;

    DID = spicelib::ZZGREP(&mut REP, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"DID", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"REP", &REP, b"=", &EREP, OK, ctx)?;

    GOT = spicelib::ZZNOTE(b"e", &mut B, &mut E, ctx);
    DID = spicelib::ZZGREP(&mut REP, ctx);
    fstr::assign(&mut EREP, b"ibmbib,bi:i:ibN");
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT", GOT, true, OK, ctx)?;
    testutil::CHCKSL(b"DID", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"REP", &REP, b"=", &EREP, OK, ctx)?;
    testutil::CHCKSC(
        b"STRING(B:E)",
        fstr::substr(&STRING, B..=E),
        b"=",
        b"A.D.",
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Make sure that items are properly removed by calls to ZZREMT and that ZZNOTE can still locate tokens after such removals. ", ctx)?;

    fstr::assign(&mut STRING, b" 12 JAN 1992 A.D., 11:12:18 P.M.");
    fstr::assign(&mut EREP, b"bibmbibe,bi:i:ibN");

    fstr::assign(&mut ERROR, b"xxx");

    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"DID1", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", b" ", OK, ctx)?;

    DID = spicelib::ZZGREP(&mut REP, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"DID2", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"REP", &REP, b"=", &EREP, OK, ctx)?;

    GOT = spicelib::ZZREMT(b"b", ctx);
    DID = spicelib::ZZGREP(&mut REP, ctx);
    fstr::assign(&mut EREP, b"imie,i:i:iN");

    testutil::CHCKSL(b"GOT", GOT, true, OK, ctx)?;
    testutil::CHCKSL(b"DID3", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"REP", &REP, b"=", &EREP, OK, ctx)?;

    GOT = spicelib::ZZREMT(b"b", ctx);
    DID = spicelib::ZZGREP(&mut REP, ctx);
    fstr::assign(&mut EREP, b"imie,i:i:iN");

    testutil::CHCKSL(b"GOT", GOT, false, OK, ctx)?;
    testutil::CHCKSL(b"DID4", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"REP", &REP, b"=", &EREP, OK, ctx)?;

    GOT = spicelib::ZZNOTE(b"e", &mut B, &mut E, ctx);
    DID = spicelib::ZZGREP(&mut REP, ctx);
    fstr::assign(&mut EREP, b"imi,i:i:iN");

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT", GOT, true, OK, ctx)?;
    testutil::CHCKSL(b"DID5", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"REP", &REP, b"=", &EREP, OK, ctx)?;
    testutil::CHCKSC(
        b"STRING(B:E)",
        fstr::substr(&STRING, B..=E),
        b"=",
        b"A.D.",
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Make sure that tokens can be combined successfully. ", ctx)?;

    fstr::assign(&mut STRING, b" 12 JAN 1992 A.D., 11:12:18 P.M.");
    fstr::assign(&mut EREP, b"bibmbibe,bi:i:ibN");

    fstr::assign(&mut ERROR, b"xxx");
    //
    // First tokenize the string and make sure it has the
    // expected tokenization.
    //
    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"DID1", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", b" ", OK, ctx)?;

    DID = spicelib::ZZGREP(&mut REP, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"DID2", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"REP1", &REP, b"=", &EREP, OK, ctx)?;
    //
    // Now remove 'b' from the tokenization and check the
    // representation.
    //
    GOT = spicelib::ZZREMT(b"b", ctx);
    DID = spicelib::ZZGREP(&mut REP, ctx);
    fstr::assign(&mut EREP, b"imie,i:i:iN");

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT", GOT, true, OK, ctx)?;
    testutil::CHCKSL(b"DID3", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"REP2", &REP, b"=", &EREP, OK, ctx)?;
    //
    // Combine a few tokens and see if we have the expected
    // representation.
    //
    GOT = spicelib::ZZCMBT(b"i:i", b"K", R2L, ctx);
    DID = spicelib::ZZGREP(&mut REP, ctx);
    fstr::assign(&mut EREP, b"imie,i:KN");

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT", GOT, true, OK, ctx)?;
    testutil::CHCKSL(b"DID4", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"REP3", &REP, b"=", &EREP, OK, ctx)?;
    //
    // See if the 'K' in the current tokenization maps to the
    // expected substring and check the NOTEd representation.
    //
    GOT = spicelib::ZZNOTE(b"K", &mut B, &mut E, ctx);
    DID = spicelib::ZZGREP(&mut REP, ctx);
    fstr::assign(&mut EREP, b"imie,i:N");

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT", GOT, true, OK, ctx)?;
    testutil::CHCKSL(b"DID5", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"REP4", &REP, b"=", &EREP, OK, ctx)?;
    testutil::CHCKSC(
        b"STRING(B:E)",
        fstr::substr(&STRING, B..=E),
        b"=",
        b"12:18",
        OK,
        ctx,
    )?;

    //
    // Combine tokens again and go through the same steps as
    // the last block of code.
    //
    GOT = spicelib::ZZCMBT(b"imie", b"K", L2R, ctx);
    DID = spicelib::ZZGREP(&mut REP, ctx);
    fstr::assign(&mut EREP, b"K,i:N");

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT", GOT, true, OK, ctx)?;
    testutil::CHCKSL(b"DID6", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"REP5", &REP, b"=", &EREP, OK, ctx)?;

    GOT = spicelib::ZZNOTE(b"K", &mut B, &mut E, ctx);
    DID = spicelib::ZZGREP(&mut REP, ctx);
    fstr::assign(&mut EREP, b",i:N");

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT", GOT, true, OK, ctx)?;
    testutil::CHCKSL(b"DID7", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"REP6", &REP, b"=", &EREP, OK, ctx)?;
    testutil::CHCKSC(
        b"STRING(B:E)",
        fstr::substr(&STRING, B..=E),
        b"=",
        b"12 JAN 1992 A.D.",
        OK,
        ctx,
    )?;
    //
    // Now try combining something that's not present.
    //
    GOT = spicelib::ZZCMBT(b"i:i", b"K", L2R, ctx);
    DID = spicelib::ZZGREP(&mut REP, ctx);
    fstr::assign(&mut EREP, b",i:N");

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT", GOT, false, OK, ctx)?;
    testutil::CHCKSL(b"DID6", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"REP5", &REP, b"=", &EREP, OK, ctx)?;

    GOT = spicelib::ZZNOTE(b"K", &mut B, &mut E, ctx);
    DID = spicelib::ZZGREP(&mut REP, ctx);
    fstr::assign(&mut EREP, b",i:N");

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT", GOT, false, OK, ctx)?;
    testutil::CHCKSL(b"DID7", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"REP6", &REP, b"=", &EREP, OK, ctx)?;
    testutil::CHCKSI(b"B", B, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"E", E, b"=", 0, 0, OK, ctx)?;

    testutil::TCASE(
        b"Make sure that the entry point ZZSUBT works as expected. ",
        ctx,
    )?;

    fstr::assign(&mut STRING, b" 12 JAN 1992 A.D., 11:12:18 P.M.");
    //
    // First tokenize the string and remove blanks.
    //
    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);
    GOT = spicelib::ZZREMT(b"b", ctx);
    DID = spicelib::ZZGREP(&mut REP, ctx);
    fstr::assign(&mut EREP, b"imie,i:i:iN");

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT", GOT, true, OK, ctx)?;
    testutil::CHCKSL(b"DID1", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"REP1", &REP, b"=", &EREP, OK, ctx)?;
    //
    // Now perform a left to right substitution.
    //
    GOT = spicelib::ZZSUBT(b"i:i", b"H:M", L2R, ctx);
    DID = spicelib::ZZGREP(&mut REP, ctx);
    fstr::assign(&mut EREP, b"imie,H:M:iN");

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT", GOT, true, OK, ctx)?;
    testutil::CHCKSL(b"DID2", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"REP2", &REP, b"=", &EREP, OK, ctx)?;
    //
    // Try another left to right substitution, this should
    // turn up with no substitution.
    //
    GOT = spicelib::ZZSUBT(b"i:i", b"M:S", L2R, ctx);
    DID = spicelib::ZZGREP(&mut REP, ctx);
    fstr::assign(&mut EREP, b"imie,H:M:iN");

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT", GOT, false, OK, ctx)?;
    testutil::CHCKSL(b"DID3", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"REP3", &REP, b"=", &EREP, OK, ctx)?;
    //
    // Get the value associated with H and make sure
    // it's the correct value.
    //
    GOT = spicelib::ZZNOTE(b"H", &mut B, &mut E, ctx);
    DID = spicelib::ZZGREP(&mut REP, ctx);
    fstr::assign(&mut EREP, b"imie,:M:iN");

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT", GOT, true, OK, ctx)?;
    testutil::CHCKSL(b"DID4", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"REP4", &REP, b"=", &EREP, OK, ctx)?;
    testutil::CHCKSC(
        b"STRING(B:E)",
        fstr::substr(&STRING, B..=E),
        b"=",
        b"11",
        OK,
        ctx,
    )?;
    //
    // Perform a right to left substitution and make sure the
    // representation changes as expected.
    //
    GOT = spicelib::ZZSUBT(b"imi", b"YmD", R2L, ctx);
    DID = spicelib::ZZGREP(&mut REP, ctx);
    fstr::assign(&mut EREP, b"YmDe,:M:iN");

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT", GOT, true, OK, ctx)?;
    testutil::CHCKSL(b"DID5", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"REP5", &REP, b"=", &EREP, OK, ctx)?;

    testutil::TCASE(
        b"Check to make sure that a pair of consecutive delimiters is recognized by ZZISPT. ",
        ctx,
    )?;

    fstr::assign(&mut STRING, b" 12 JAN 1992 A.D.,/ 11:12:18 P.M.");
    //
    // First tokenize the string
    //
    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);
    GOT = spicelib::ZZISPT(b",/:", &mut B, &mut E, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT", GOT, true, OK, ctx)?;
    testutil::CHCKSL(b"DID", DID, true, OK, ctx)?;
    testutil::CHCKSC(
        b"STRING(B:E)",
        fstr::substr(&STRING, B..=E),
        b"=",
        b",/",
        OK,
        ctx,
    )?;

    fstr::assign(&mut STRING, b" 12 JAN 1992 A.D., 11:12:18 P.M.");
    //
    // First tokenize the string
    //
    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);
    GOT = spicelib::ZZISPT(&STRING, &mut B, &mut E, ctx);
    fstr::assign(&mut STRING, b" 12 JAN 1992 A.D., 11:12:18 P.M.");
    //
    // First tokenize the string
    //
    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);
    GOT = spicelib::ZZISPT(b",/:", &mut B, &mut E, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT", GOT, false, OK, ctx)?;
    testutil::CHCKSL(b"DID", DID, true, OK, ctx)?;
    testutil::CHCKSI(b"B", B, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"E", E, b"=", 0, 0, OK, ctx)?;

    testutil::TCASE(
        b"Make sure that the function ZZIST works as expected. ",
        ctx,
    )?;

    fstr::assign(&mut STRING, b" 12 JAN 1992 A.D., 11:12:18 P.M.");
    //
    // First tokenize the string
    //
    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);

    testutil::CHCKSL(b"DID", DID, true, OK, ctx)?;

    GOT = spicelib::ZZIST(b"i", ctx);
    testutil::CHCKSL(b"GOT1", GOT, true, OK, ctx)?;

    GOT = spicelib::ZZIST(b"m", ctx);
    testutil::CHCKSL(b"GOT2", GOT, true, OK, ctx)?;

    GOT = spicelib::ZZIST(b":", ctx);
    testutil::CHCKSL(b"GOT3", GOT, true, OK, ctx)?;

    GOT = spicelib::ZZIST(b"e", ctx);
    testutil::CHCKSL(b"GOT4", GOT, true, OK, ctx)?;

    GOT = spicelib::ZZIST(b"q", ctx);
    testutil::CHCKSL(b"GOT5", GOT, false, OK, ctx)?;

    GOT = spicelib::ZZIST(b"N", ctx);
    testutil::CHCKSL(b"GOT6", GOT, true, OK, ctx)?;

    GOT = spicelib::ZZIST(b"Z", ctx);
    testutil::CHCKSL(b"GOT7", GOT, false, OK, ctx)?;

    testutil::TCASE(b"Verify that ZZVALT replaces tokens when it should and that it leaves them alone when there is nothing to do. ", ctx)?;

    fstr::assign(&mut STRING, b" 12 JAN 2012 A.D., 11:12:18 P.M.");
    //
    // First tokenize the string
    //
    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"DID", DID, true, OK, ctx)?;

    GOT = spicelib::ZZVALT(&STRING, 1000, 1000000, b"Y", ctx);
    DID = spicelib::ZZGREP(&mut REP, ctx);
    fstr::assign(&mut EREP, b"bibmbYbe,bi:i:ibN");

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT1", GOT, true, OK, ctx)?;
    testutil::CHCKSL(b"DID", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"REP", &REP, b"=", &EREP, OK, ctx)?;

    GOT = spicelib::ZZVALT(&STRING, 1000, 1000000, b"K", ctx);
    DID = spicelib::ZZGREP(&mut REP, ctx);
    fstr::assign(&mut EREP, b"bibmbYbe,bi:i:ibN");

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT2", GOT, false, OK, ctx)?;
    testutil::CHCKSL(b"DID", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"REP", &REP, b"=", &EREP, OK, ctx)?;

    testutil::TCASE(
        b"Check to make sure that ZZUNPCK can unpack items from a string. ",
        ctx,
    )?;

    fstr::assign(&mut STRING, b" 12 JAN 2012 A.D., 11:12:18 P.M.");
    //
    // First tokenize the string
    //
    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);

    testutil::CHCKSL(b"DID", DID, true, OK, ctx)?;

    GOT = spicelib::ZZVALT(&STRING, 1000, 1000000, b"Y", ctx);
    DID = spicelib::ZZGREP(&mut REP, ctx);
    fstr::assign(&mut EREP, b"bibmbYbe,bi:i:ibN");

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT1", GOT, true, OK, ctx)?;
    testutil::CHCKSL(b"DID", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"REP", &REP, b"=", &EREP, OK, ctx)?;

    DID = spicelib::ZZREMT(b"b", ctx);
    DID = spicelib::ZZREMT(b":", ctx);
    DID = spicelib::ZZREMT(b",", ctx);
    DID = spicelib::ZZREMT(b"e", ctx);
    DID = spicelib::ZZREMT(b"N", ctx);

    DID = spicelib::ZZSUBT(b"imYiii", b"DmYHMS", L2R, ctx);
    GOT = spicelib::ZZUNPCK(
        &STRING,
        YABBRV,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TRANSL,
        &mut PIC,
        &mut ERROR,
        ctx,
    );

    ETVEC[1] = 2012.0;
    ETVEC[2] = 1.0;
    ETVEC[3] = 12.0;
    ETVEC[4] = 11.0;
    ETVEC[5] = 12.0;
    ETVEC[6] = 18.0;

    fstr::assign(&mut EXPERR, b" ");
    fstr::assign(&mut ETRANS, b"YMD");
    EXPN = 6;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT2", GOT, true, OK, ctx)?;
    testutil::CHCKSL(b"DID2", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", b" ", OK, ctx)?;
    testutil::CHCKSI(b"NTVEC", NTVEC, b"=", 6, 0, OK, ctx)?;
    testutil::CHCKSC(b"TRANSL", &TRANSL, b"=", b"YMD", OK, ctx)?;
    testutil::CHCKAD(
        b"TVEC",
        TVEC.as_slice(),
        b"=",
        ETVEC.as_slice(),
        6,
        0.0,
        OK,
        ctx,
    )?;

    fstr::assign(&mut STRING, b"2012 A.D. 129// 11:12:18 P.M.");
    //
    // First tokenize the string
    //
    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);

    testutil::CHCKSL(b"DID", DID, true, OK, ctx)?;

    GOT = spicelib::ZZVALT(&STRING, 1000, 1000000, b"Y", ctx);
    DID = spicelib::ZZGREP(&mut REP, ctx);
    fstr::assign(&mut EREP, b"Ybebidbi:i:ibN");

    testutil::CHCKSL(b"GOT1", GOT, true, OK, ctx)?;
    testutil::CHCKSL(b"DID", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"REP", &REP, b"=", &EREP, OK, ctx)?;

    DID = spicelib::ZZREMT(b"b", ctx);
    DID = spicelib::ZZREMT(b":", ctx);
    DID = spicelib::ZZREMT(b",", ctx);
    DID = spicelib::ZZREMT(b"e", ctx);
    DID = spicelib::ZZREMT(b"d", ctx);
    DID = spicelib::ZZREMT(b"N", ctx);

    DID = spicelib::ZZSUBT(b"Yiiii", b"YyHMS", L2R, ctx);
    GOT = spicelib::ZZUNPCK(
        &STRING,
        YABBRV,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TRANSL,
        &mut PIC,
        &mut ERROR,
        ctx,
    );

    ETVEC[1] = 2012.0;
    ETVEC[2] = 129.0;
    ETVEC[3] = 11.0;
    ETVEC[4] = 12.0;
    ETVEC[5] = 18.0;

    fstr::assign(&mut EXPERR, b" ");
    fstr::assign(&mut ETRANS, b"YMD");
    EXPN = 6;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT2", GOT, true, OK, ctx)?;
    testutil::CHCKSL(b"DID2", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", b" ", OK, ctx)?;
    testutil::CHCKSI(b"NTVEC", NTVEC, b"=", 5, 0, OK, ctx)?;
    testutil::CHCKSC(b"TRANSL", &TRANSL, b"=", b"YD", OK, ctx)?;
    testutil::CHCKAD(
        b"TVEC",
        TVEC.as_slice(),
        b"=",
        ETVEC.as_slice(),
        6,
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(
        b"Check to make sure that an unresolved string is diagnosed as one by ZZUNPCK. ",
        ctx,
    )?;

    fstr::assign(&mut STRING, b"2012 A.D. 129// 11:12:18 P.M.");
    //
    // First tokenize the string
    //
    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);

    testutil::CHCKSL(b"DID", DID, true, OK, ctx)?;

    GOT = spicelib::ZZVALT(&STRING, 1000, 1000000, b"Y", ctx);
    DID = spicelib::ZZGREP(&mut REP, ctx);
    fstr::assign(&mut EREP, b"Ybebidbi:i:ibN");

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT1", GOT, true, OK, ctx)?;
    testutil::CHCKSL(b"DID", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"REP", &REP, b"=", &EREP, OK, ctx)?;

    DID = spicelib::ZZREMT(b"b", ctx);
    DID = spicelib::ZZREMT(b":", ctx);
    DID = spicelib::ZZREMT(b",", ctx);
    DID = spicelib::ZZREMT(b"e", ctx);
    DID = spicelib::ZZREMT(b"d", ctx);
    DID = spicelib::ZZREMT(b"N", ctx);

    GOT = spicelib::ZZUNPCK(
        &STRING,
        YABBRV,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TRANSL,
        &mut PIC,
        &mut ERROR,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT2", GOT, false, OK, ctx)?;
    testutil::CHCKSC(b"TRANSL", &TRANSL, b"=", b" ", OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    if SHDIAG {
        testutil::TSTSTY(&mut GOOD, &mut BAD, ctx);
        testutil::TSTLGS(&MSTYLE, &MSTYLE, ctx);
        testutil::TSTLOG(b" ", false, ctx)?;
        testutil::TSTLOG(&ERROR, false, ctx)?;
        testutil::TSTLOG(b" ", false, ctx)?;
        testutil::TSTLGS(&GOOD, &BAD, ctx);
    }

    testutil::TCASE(b"Checking Diagnostic Messages YmDHMn", ctx)?;

    fstr::assign(&mut STRING, b" 1995JAN19 12 13 12");
    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);
    DID = spicelib::ZZREMT(b"b", ctx);
    DID = spicelib::ZZSUBT(b"imiiii", b"YmDHMn", L2R, ctx);
    GOT = spicelib::ZZUNPCK(
        &STRING,
        YABBRV,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TRANSL,
        &mut PIC,
        &mut ERROR,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT2", GOT, false, OK, ctx)?;
    testutil::CHCKSC(b"TRANSL", &TRANSL, b"=", b" ", OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    if SHDIAG {
        testutil::TSTSTY(&mut GOOD, &mut BAD, ctx);
        testutil::TSTLGS(&MSTYLE, &MSTYLE, ctx);
        testutil::TSTLOG(b" ", false, ctx)?;
        testutil::TSTLOG(&ERROR, false, ctx)?;
        testutil::TSTLOG(b" ", false, ctx)?;
        testutil::TSTLGS(&GOOD, &BAD, ctx);
    }

    testutil::TCASE(b"Checking Diagnostic Messages YmYHMS", ctx)?;

    fstr::assign(&mut STRING, b" 1995JAN1995 12 13 12");
    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);
    DID = spicelib::ZZREMT(b"b", ctx);
    DID = spicelib::ZZSUBT(b"imiiii", b"YmYHMS", L2R, ctx);
    GOT = spicelib::ZZUNPCK(
        &STRING,
        YABBRV,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TRANSL,
        &mut PIC,
        &mut ERROR,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT2", GOT, false, OK, ctx)?;
    testutil::CHCKSC(b"TRANSL", &TRANSL, b"=", b" ", OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    if SHDIAG {
        testutil::TSTSTY(&mut GOOD, &mut BAD, ctx);
        testutil::TSTLGS(&MSTYLE, &MSTYLE, ctx);
        testutil::TSTLOG(b" ", false, ctx)?;
        testutil::TSTLOG(&ERROR, false, ctx)?;
        testutil::TSTLOG(b" ", false, ctx)?;
        testutil::TSTLGS(&GOOD, &BAD, ctx);
    }

    testutil::TCASE(b"Checking Diagnostic Messages YDHMS", ctx)?;

    fstr::assign(&mut STRING, b" 1995 19 12 13 12");
    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);
    DID = spicelib::ZZREMT(b"b", ctx);
    DID = spicelib::ZZSUBT(b"iiiii", b"YDHMS", L2R, ctx);
    GOT = spicelib::ZZUNPCK(
        &STRING,
        YABBRV,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TRANSL,
        &mut PIC,
        &mut ERROR,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT2", GOT, false, OK, ctx)?;
    testutil::CHCKSC(b"TRANSL", &TRANSL, b"=", b" ", OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    if SHDIAG {
        testutil::TSTSTY(&mut GOOD, &mut BAD, ctx);
        testutil::TSTLGS(&MSTYLE, &MSTYLE, ctx);
        testutil::TSTLOG(b" ", false, ctx)?;
        testutil::TSTLOG(&ERROR, false, ctx)?;
        testutil::TSTLOG(b" ", false, ctx)?;
        testutil::TSTLGS(&GOOD, &BAD, ctx);
    }

    testutil::TCASE(b"Checking Diagnostic Messages YmDyHMS", ctx)?;

    fstr::assign(&mut STRING, b" 1995JAN19 95 12 13 12");
    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);
    DID = spicelib::ZZREMT(b"b", ctx);
    DID = spicelib::ZZSUBT(b"imiiiii", b"YmDyHMS", L2R, ctx);
    GOT = spicelib::ZZUNPCK(
        &STRING,
        YABBRV,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TRANSL,
        &mut PIC,
        &mut ERROR,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT2", GOT, false, OK, ctx)?;
    testutil::CHCKSC(b"TRANSL", &TRANSL, b"=", b" ", OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    if SHDIAG {
        testutil::TSTSTY(&mut GOOD, &mut BAD, ctx);
        testutil::TSTLGS(&MSTYLE, &MSTYLE, ctx);
        testutil::TSTLOG(b" ", false, ctx)?;
        testutil::TSTLOG(&ERROR, false, ctx)?;
        testutil::TSTLOG(b" ", false, ctx)?;
        testutil::TSTLGS(&GOOD, &BAD, ctx);
    }

    testutil::TCASE(b"Checking Diagnostic Messages mDHMS", ctx)?;

    fstr::assign(&mut STRING, b"JAN 19 12 13 12");
    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);
    DID = spicelib::ZZREMT(b"b", ctx);
    DID = spicelib::ZZSUBT(b"miiii", b"mDHMS", L2R, ctx);
    GOT = spicelib::ZZUNPCK(
        &STRING,
        YABBRV,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TRANSL,
        &mut PIC,
        &mut ERROR,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT2", GOT, false, OK, ctx)?;
    testutil::CHCKSC(b"TRANSL", &TRANSL, b"=", b" ", OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    if SHDIAG {
        testutil::TSTSTY(&mut GOOD, &mut BAD, ctx);
        testutil::TSTLGS(&MSTYLE, &MSTYLE, ctx);
        testutil::TSTLOG(b" ", false, ctx)?;
        testutil::TSTLOG(&ERROR, false, ctx)?;
        testutil::TSTLOG(b" ", false, ctx)?;
        testutil::TSTLGS(&GOOD, &BAD, ctx);
    }

    testutil::TCASE(b"Checking Diagnostic Messages YmmDHMS", ctx)?;

    fstr::assign(&mut STRING, b"1995JAN 11 5 12 13 12");
    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);
    DID = spicelib::ZZREMT(b"b", ctx);
    DID = spicelib::ZZSUBT(b"imiiiii", b"YmmDHMS", L2R, ctx);
    GOT = spicelib::ZZUNPCK(
        &STRING,
        YABBRV,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TRANSL,
        &mut PIC,
        &mut ERROR,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT2", GOT, false, OK, ctx)?;
    testutil::CHCKSC(b"TRANSL", &TRANSL, b"=", b" ", OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    if SHDIAG {
        testutil::TSTSTY(&mut GOOD, &mut BAD, ctx);
        testutil::TSTLGS(&MSTYLE, &MSTYLE, ctx);
        testutil::TSTLOG(b" ", false, ctx)?;
        testutil::TSTLOG(&ERROR, false, ctx)?;
        testutil::TSTLOG(b" ", false, ctx)?;
        testutil::TSTLGS(&GOOD, &BAD, ctx);
    }

    testutil::TCASE(b"Checking Diagnostic Messages YyyHMS", ctx)?;

    fstr::assign(&mut STRING, b"1995 12 11 12 13 12");
    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);
    DID = spicelib::ZZREMT(b"b", ctx);
    DID = spicelib::ZZSUBT(b"iiiii", b"YyyHMS", L2R, ctx);
    GOT = spicelib::ZZUNPCK(
        &STRING,
        YABBRV,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TRANSL,
        &mut PIC,
        &mut ERROR,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT2", GOT, false, OK, ctx)?;
    testutil::CHCKSC(b"TRANSL", &TRANSL, b"=", b" ", OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    if SHDIAG {
        testutil::TSTSTY(&mut GOOD, &mut BAD, ctx);
        testutil::TSTLGS(&MSTYLE, &MSTYLE, ctx);
        testutil::TSTLOG(b" ", false, ctx)?;
        testutil::TSTLOG(&ERROR, false, ctx)?;
        testutil::TSTLOG(b" ", false, ctx)?;
        testutil::TSTLGS(&GOOD, &BAD, ctx);
    }

    testutil::TCASE(b"Checking Diagnostic Messages YDmDHMS", ctx)?;

    fstr::assign(&mut STRING, b"1995 12 JAN 11 12 13 12");
    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);
    DID = spicelib::ZZREMT(b"b", ctx);
    DID = spicelib::ZZSUBT(b"iimiiii", b"YDmDHMS", L2R, ctx);
    GOT = spicelib::ZZUNPCK(
        &STRING,
        YABBRV,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TRANSL,
        &mut PIC,
        &mut ERROR,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT2", GOT, false, OK, ctx)?;
    testutil::CHCKSC(b"TRANSL", &TRANSL, b"=", b" ", OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    if SHDIAG {
        testutil::TSTSTY(&mut GOOD, &mut BAD, ctx);
        testutil::TSTLGS(&MSTYLE, &MSTYLE, ctx);
        testutil::TSTLOG(b" ", false, ctx)?;
        testutil::TSTLOG(&ERROR, false, ctx)?;
        testutil::TSTLOG(b" ", false, ctx)?;
        testutil::TSTLGS(&GOOD, &BAD, ctx);
    }

    testutil::TCASE(b"Checking Diagnostic Messages YDmHH", ctx)?;

    fstr::assign(&mut STRING, b"1995 12 JAN 11 12 ");
    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);
    DID = spicelib::ZZREMT(b"b", ctx);
    DID = spicelib::ZZSUBT(b"iimii", b"YDmHH", L2R, ctx);
    GOT = spicelib::ZZUNPCK(
        &STRING,
        YABBRV,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TRANSL,
        &mut PIC,
        &mut ERROR,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT2", GOT, false, OK, ctx)?;
    testutil::CHCKSC(b"TRANSL", &TRANSL, b"=", b" ", OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    if SHDIAG {
        testutil::TSTSTY(&mut GOOD, &mut BAD, ctx);
        testutil::TSTLGS(&MSTYLE, &MSTYLE, ctx);
        testutil::TSTLOG(b" ", false, ctx)?;
        testutil::TSTLOG(&ERROR, false, ctx)?;
        testutil::TSTLOG(b" ", false, ctx)?;
        testutil::TSTLGS(&GOOD, &BAD, ctx);
    }

    testutil::TCASE(b"Checking Diagnostic Messages YDmSS", ctx)?;

    fstr::assign(&mut STRING, b"1995 12 JAN 12 13 12");
    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);
    DID = spicelib::ZZREMT(b"b", ctx);
    DID = spicelib::ZZSUBT(b"iimii", b"YDmSS", L2R, ctx);
    GOT = spicelib::ZZUNPCK(
        &STRING,
        YABBRV,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TRANSL,
        &mut PIC,
        &mut ERROR,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT2", GOT, false, OK, ctx)?;
    testutil::CHCKSC(b"TRANSL", &TRANSL, b"=", b" ", OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    if SHDIAG {
        testutil::TSTSTY(&mut GOOD, &mut BAD, ctx);
        testutil::TSTLGS(&MSTYLE, &MSTYLE, ctx);
        testutil::TSTLOG(b" ", false, ctx)?;
        testutil::TSTLOG(&ERROR, false, ctx)?;
        testutil::TSTLOG(b" ", false, ctx)?;
        testutil::TSTLGS(&GOOD, &BAD, ctx);
    }

    testutil::TCASE(b"Checking Diagnostic Messages YDmMM", ctx)?;

    fstr::assign(&mut STRING, b"1995 12 JAN 11 12 ");
    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);
    DID = spicelib::ZZREMT(b"b", ctx);
    DID = spicelib::ZZSUBT(b"iimii", b"YDmMM", L2R, ctx);
    GOT = spicelib::ZZUNPCK(
        &STRING,
        YABBRV,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TRANSL,
        &mut PIC,
        &mut ERROR,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT2", GOT, false, OK, ctx)?;
    testutil::CHCKSC(b"TRANSL", &TRANSL, b"=", b" ", OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    if SHDIAG {
        testutil::TSTSTY(&mut GOOD, &mut BAD, ctx);
        testutil::TSTLGS(&MSTYLE, &MSTYLE, ctx);
        testutil::TSTLOG(b" ", false, ctx)?;
        testutil::TSTLOG(&ERROR, false, ctx)?;
        testutil::TSTLOG(b" ", false, ctx)?;
        testutil::TSTLGS(&GOOD, &BAD, ctx);
    }

    testutil::TCASE(b"Checking Diagnostic Messages YDmMS", ctx)?;

    fstr::assign(&mut STRING, b"1995 12 JAN 11 12 ");
    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);
    DID = spicelib::ZZREMT(b"b", ctx);
    DID = spicelib::ZZSUBT(b"iimii", b"YDmMS", L2R, ctx);
    GOT = spicelib::ZZUNPCK(
        &STRING,
        YABBRV,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TRANSL,
        &mut PIC,
        &mut ERROR,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT2", GOT, false, OK, ctx)?;
    testutil::CHCKSC(b"TRANSL", &TRANSL, b"=", b" ", OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    if SHDIAG {
        testutil::TSTSTY(&mut GOOD, &mut BAD, ctx);
        testutil::TSTLGS(&MSTYLE, &MSTYLE, ctx);
        testutil::TSTLOG(b" ", false, ctx)?;
        testutil::TSTLOG(&ERROR, false, ctx)?;
        testutil::TSTLOG(b" ", false, ctx)?;
        testutil::TSTLGS(&GOOD, &BAD, ctx);
    }

    testutil::TCASE(b"Checking Diagnostic Messages YDmHS", ctx)?;

    fstr::assign(&mut STRING, b"1995 12 JAN 11 12 ");
    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);
    DID = spicelib::ZZREMT(b"b", ctx);
    DID = spicelib::ZZSUBT(b"iimii", b"YDmHS", L2R, ctx);
    GOT = spicelib::ZZUNPCK(
        &STRING,
        YABBRV,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TRANSL,
        &mut PIC,
        &mut ERROR,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT2", GOT, false, OK, ctx)?;
    testutil::CHCKSC(b"TRANSL", &TRANSL, b"=", b" ", OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    if SHDIAG {
        testutil::TSTSTY(&mut GOOD, &mut BAD, ctx);
        testutil::TSTLGS(&MSTYLE, &MSTYLE, ctx);
        testutil::TSTLOG(b" ", false, ctx)?;
        testutil::TSTLOG(&ERROR, false, ctx)?;
        testutil::TSTLOG(b" ", false, ctx)?;
        testutil::TSTLGS(&GOOD, &BAD, ctx);
    }

    testutil::TCASE(
        b"Make sure that we can perform the substitution and removal of *\'d tokens. ",
        ctx,
    )?;

    fstr::assign(&mut STRING, b"Monday April 22, 9:24:18.19 PST 1997");

    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);
    GOT = spicelib::ZZREMT(b"b", ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT1", GOT, true, OK, ctx)?;
    testutil::CHCKSL(b"DID1", DID, true, OK, ctx)?;

    GOT = spicelib::ZZCMBT(b"i.i", b"n", L2R, ctx);
    DID = spicelib::ZZGREP(&mut REP, ctx);
    fstr::assign(&mut EREP, b"wmi,i:i:nZi");

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT2", GOT, true, OK, ctx)?;
    testutil::CHCKSL(b"DID2", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"REP", &REP, b"=", &EREP, OK, ctx)?;

    GOT = spicelib::ZZREPT(b"i:i:n", b"H*M*S", R2L, ctx);
    DID = spicelib::ZZGREP(&mut REP, ctx);
    fstr::assign(&mut EREP, b"wmi,HMSZi");

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"GOT2", GOT, true, OK, ctx)?;
    testutil::CHCKSL(b"DID2", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"REP", &REP, b"=", &EREP, OK, ctx)?;

    testutil::TCASE(b"Check that blank strings are properly identified.", ctx)?;

    fstr::assign(&mut EXPERR, b"The input time string is blank.");

    fstr::assign(&mut STRING, b" ");
    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"DID1", DID, false, OK, ctx)?;
    testutil::CHCKSC(b"ERR1", &ERROR, b"=", &EXPERR, OK, ctx)?;

    fstr::assign(&mut STRING, b"  ");
    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"DID2", DID, false, OK, ctx)?;
    testutil::CHCKSC(b"ERR2", &ERROR, b"=", &EXPERR, OK, ctx)?;

    fstr::assign(&mut ONECHR, b" ");
    DID = spicelib::ZZTOKNS(&ONECHR, &mut ERROR, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"DID3", DID, false, OK, ctx)?;
    testutil::CHCKSC(b"ERR3", &ERROR, b"=", &EXPERR, OK, ctx)?;

    DID = spicelib::ZZTOKNS(b" ", &mut ERROR, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"DID4", DID, false, OK, ctx)?;
    testutil::CHCKSC(b"ERR4", &ERROR, b"=", &EXPERR, OK, ctx)?;

    DID = spicelib::ZZTOKNS(b"  ", &mut ERROR, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"DID5", DID, false, OK, ctx)?;
    testutil::CHCKSC(b"ERR5", &ERROR, b"=", &EXPERR, OK, ctx)?;

    testutil::TCASE(b"Check error detection for ASCII character 127", ctx)?;

    fstr::assign(&mut STRING, b"1 JAN 2000");
    fstr::assign(fstr::substr_mut(&mut STRING, 2..=2), &intrinsics::CHAR(127));

    fstr::assign(&mut EXPERR, b"There is a non-printing, non-tab character (ASCII 127) at position 2 of the time string: 1<>JAN 2000");

    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"DID", DID, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EXPERR, OK, ctx)?;

    testutil::TCASE(b"Check error detection for ASCII character 128", ctx)?;

    fstr::assign(&mut STRING, b"1 JAN 2000");
    fstr::assign(fstr::substr_mut(&mut STRING, 6..=6), &intrinsics::CHAR(128));

    fstr::assign(&mut EXPERR, b"There is a non-printing, non-tab character (ASCII 128) at position 6 of the time string: 1 JAN<>2000");

    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"DID", DID, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EXPERR, OK, ctx)?;

    //
    // Check ZZTOKNS token count overflow exceptions.
    // Set expected error templates to match the one in ZZTOKNS.
    //
    fstr::assign(&mut TKNERR, b"The input time string \'#\' cannot be processed because it contains more than @ recognizable tokens. The token that could not be processed was \'#\'.");
    spicelib::REPMI(&TKNERR.clone(), b"@", MAXTKN, &mut TKNERR, ctx);

    testutil::TCASE(
        b"Check that ZZTOKNS can process a string with the maximum number of tokens.",
        ctx,
    )?;

    fstr::assign(&mut STRING, b" 2");
    for I in intrinsics::range(4, MAXTKN, 2) {
        spicelib::SUFFIX(b"#", 1, &mut STRING);
        spicelib::REPMI(&STRING.clone(), b"#", I, &mut STRING, ctx);
    }
    LAST = spicelib::RTRIM(&STRING);

    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"DID", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", b" ", OK, ctx)?;

    testutil::TCASE(
        b"Check error detection for token count overflow, unsigned integer branch of ZZTOKNS",
        ctx,
    )?;

    fstr::assign(fstr::substr_mut(&mut STRING, (LAST - 1)..), b"pm0");
    spicelib::REPMC(&TKNERR, b"#", &STRING, &mut EXPERR);
    spicelib::REPMC(&EXPERR.clone(), b"#", b"0", &mut EXPERR);

    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"DID", DID, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EXPERR, OK, ctx)?;

    testutil::TCASE(
        b"Check error detection for token count overflow, blanks branch of ZZTOKNS",
        ctx,
    )?;

    fstr::assign(fstr::substr_mut(&mut STRING, (LAST - 1)..), b"pm 0");
    spicelib::REPMC(&TKNERR, b"#", &STRING, &mut EXPERR);
    spicelib::REPMC(&EXPERR.clone(), b"#", b" ", &mut EXPERR);

    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"DID", DID, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EXPERR, OK, ctx)?;

    testutil::TCASE(
        b"Check error detection for token count overflow, TABs branch of ZZTOKNS",
        ctx,
    )?;

    fstr::assign(
        fstr::substr_mut(&mut STRING, (LAST - 1)..),
        &fstr::concat(b"pm", &intrinsics::CHAR(9)),
    );
    spicelib::REPMC(&TKNERR, b"#", &STRING, &mut EXPERR);
    spicelib::REPMC(&EXPERR.clone(), b"#", b"<TAB>", &mut EXPERR);

    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"DID", DID, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EXPERR, OK, ctx)?;

    testutil::TCASE(
        b"Check error detection for token count overflow, other tokens branch of ZZTOKNS",
        ctx,
    )?;

    fstr::assign(fstr::substr_mut(&mut STRING, (LAST - 1)..), b"0B.C.");
    spicelib::REPMC(&TKNERR, b"#", &STRING, &mut EXPERR);
    spicelib::REPMC(&EXPERR.clone(), b"#", b"B.C.", &mut EXPERR);

    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"DID", DID, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EXPERR, OK, ctx)?;

    //
    // Check ZZTOKNS picture length overflow exceptions.
    // Set expected error template to match the one in ZZTOKNS.
    //
    fstr::assign(&mut PICERR, b"The input time string \'#\' cannot be processed because the internal picture describing it requires more than @ characters. The token that could not be processed was \'#\'.");
    spicelib::REPMI(&PICERR.clone(), b"@", PICLEN, &mut PICERR, ctx);

    testutil::TCASE(
        b"Check that ZZTOKNS can process a string with the maximum possible picture length.",
        ctx,
    )?;

    fstr::assign(&mut STRING, b"2007-08-09");
    fstr::assign(fstr::substr_mut(&mut STRING, (PICLEN - 7)..), b"12:34:56");
    LAST = spicelib::RTRIM(&STRING);

    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"DID", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", b" ", OK, ctx)?;

    testutil::TCASE(
        b"Check error detection for picture overflow, unsigned integer branch of ZZTOKNS",
        ctx,
    )?;

    fstr::assign(fstr::substr_mut(&mut STRING, (LAST + 1)..), b"0");
    spicelib::REPMC(&PICERR, b"#", &STRING, &mut EXPERR);
    spicelib::REPMC(&EXPERR.clone(), b"#", b"560", &mut EXPERR);

    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"DID", DID, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EXPERR, OK, ctx)?;

    testutil::TCASE(
        b"Check error detection for picture overflow, blanks branch of ZZTOKNS",
        ctx,
    )?;

    fstr::assign(fstr::substr_mut(&mut STRING, (LAST + 1)..), b" 0");
    spicelib::REPMC(&PICERR, b"#", &STRING, &mut EXPERR);
    spicelib::REPMC(&EXPERR.clone(), b"#", b" ", &mut EXPERR);

    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"DID", DID, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EXPERR, OK, ctx)?;

    testutil::TCASE(
        b"Check error detection for picture overflow, TABs branch of ZZTOKNS",
        ctx,
    )?;

    fstr::assign(
        fstr::substr_mut(&mut STRING, (LAST + 1)..),
        &intrinsics::CHAR(9),
    );
    spicelib::REPMC(&PICERR, b"#", &STRING, &mut EXPERR);
    spicelib::REPMC(&EXPERR.clone(), b"#", b"<TAB>", &mut EXPERR);

    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"DID", DID, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EXPERR, OK, ctx)?;

    testutil::TCASE(
        b"Check error detection for picture overflow, month branch of ZZTOKNS",
        ctx,
    )?;

    fstr::assign(fstr::substr_mut(&mut STRING, (LAST + 1)..), b"May");
    spicelib::REPMC(&PICERR, b"#", &STRING, &mut EXPERR);
    spicelib::REPMC(&EXPERR.clone(), b"#", b"May", &mut EXPERR);

    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"DID", DID, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EXPERR, OK, ctx)?;

    testutil::TCASE(
        b"Check error detection for picture overflow, month branch of ZZTOKNS",
        ctx,
    )?;

    fstr::assign(fstr::substr_mut(&mut STRING, (LAST + 1)..), b"Sunday");
    spicelib::REPMC(&PICERR, b"#", &STRING, &mut EXPERR);
    spicelib::REPMC(&EXPERR.clone(), b"#", b"Sunday", &mut EXPERR);

    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"DID", DID, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EXPERR, OK, ctx)?;

    testutil::TCASE(
        b"Check error detection for picture overflow, era branch of ZZTOKNS",
        ctx,
    )?;

    fstr::assign(fstr::substr_mut(&mut STRING, (LAST + 1)..), b"B.C.");
    spicelib::REPMC(&PICERR, b"#", &STRING, &mut EXPERR);
    spicelib::REPMC(&EXPERR.clone(), b"#", b"B.C.", &mut EXPERR);

    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"DID", DID, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EXPERR, OK, ctx)?;

    testutil::TCASE(
        b"Check error detection for picture overflow, am/pm branch of ZZTOKNS",
        ctx,
    )?;

    fstr::assign(fstr::substr_mut(&mut STRING, (LAST + 1)..), b"AM");
    spicelib::REPMC(&PICERR, b"#", &STRING, &mut EXPERR);
    spicelib::REPMC(&EXPERR.clone(), b"#", b"AM", &mut EXPERR);

    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"DID", DID, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EXPERR, OK, ctx)?;

    testutil::TCASE(
        b"Check error detection for picture overflow, other tokens branch of ZZTOKNS",
        ctx,
    )?;

    fstr::assign(fstr::substr_mut(&mut STRING, (LAST + 1)..), b"/");
    spicelib::REPMC(&PICERR, b"#", &STRING, &mut EXPERR);
    spicelib::REPMC(&EXPERR.clone(), b"#", b"/", &mut EXPERR);

    DID = spicelib::ZZTOKNS(&STRING, &mut ERROR, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"DID", DID, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", &EXPERR, OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
