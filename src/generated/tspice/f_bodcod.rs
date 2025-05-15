//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXL: i32 = 36;
const MAXP: i32 = 150;
const NPERM: i32 = 692;
const MAXE: i32 = 853;
const NROOM: i32 = 14983;
const CTRSIZ: i32 = 2;
const NAMLEN: i32 = 200;
const LINLEN: i32 = 80;
const BUFLEN: i32 = 10;
const TKBASE: i32 = 10000;
const BDBASE: i32 = 20000;

struct SaveVars {
    BASNAM: ActualCharArray,
    BASNOR: ActualCharArray,
    BUFFER: ActualCharArray,
    HOLD: Vec<u8>,
    NAME: Vec<u8>,
    TKNAME: Vec<u8>,
    BASCOD: ActualArray<i32>,
    CODE: i32,
    J: i32,
    NVALS: i32,
    RESTOR: i32,
    USRCTR: StackArray<i32, 2>,
    FOUND: bool,
    UPDATE: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BASNAM = ActualCharArray::new(MAXL, 1..=MAXE);
        let mut BASNOR = ActualCharArray::new(MAXL, 1..=MAXE);
        let mut BUFFER = ActualCharArray::new(LINLEN, 1..=BUFLEN);
        let mut HOLD = vec![b' '; NAMLEN as usize];
        let mut NAME = vec![b' '; NAMLEN as usize];
        let mut TKNAME = vec![b' '; NAMLEN as usize];
        let mut BASCOD = ActualArray::<i32>::new(1..=MAXE);
        let mut CODE: i32 = 0;
        let mut J: i32 = 0;
        let mut NVALS: i32 = 0;
        let mut RESTOR: i32 = 0;
        let mut USRCTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut FOUND: bool = false;
        let mut UPDATE: bool = false;

        Self {
            BASNAM,
            BASNOR,
            BUFFER,
            HOLD,
            NAME,
            TKNAME,
            BASCOD,
            CODE,
            J,
            NVALS,
            RESTOR,
            USRCTR,
            FOUND,
            UPDATE,
        }
    }
}

//$Procedure F_BODCOD ( Body Code/Name Mapping Test Family )
pub fn F_BODCOD(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // TESTUTIL Functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Save everything.
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_BODCOD", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check cleared state of test module ID codes.", ctx)?;

    //
    // Verify that NROOM is even.  It must be otherwise a few of the
    // test cases will fail.
    //
    // This test was commented out because the later test code was
    // changed to work with the even number preceding NROOM.
    //
    //  CALL CHCKSI ( 'NROOM-EVEN', MOD(NROOM,2), '=', 0, 0, OK )

    //
    // Retrieve the built-in code/name arrays.
    //
    spicelib::ZZBODGET(
        MAXE,
        save.BASNAM.as_arg_mut(),
        save.BASNOR.as_arg_mut(),
        save.BASCOD.as_slice_mut(),
        &mut save.NVALS,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Multiple NAMES to a single code, BODDEF", ctx)?;

    spicelib::BODDEF(b"F_BODCOD_TESTA", 1001, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check that this assignment worked as expected.
    //
    spicelib::BODC2N(1001, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"F_BODCOD_TESTA", OK, ctx)?;

    //
    // Now assign a new name to the same code using BODDEF.
    //
    spicelib::BODDEF(b"F_BODCOD_TEST", 1001, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Verify that the newer name takes precedence.
    //
    spicelib::BODC2N(1001, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"F_BODCOD_TEST", OK, ctx)?;

    //
    // Depending on the sorting algorithm this could go a number
    // of ways.  Check a name that would occur "after" F_BODCOD_TESTA
    // in the list as well.
    //
    spicelib::BODDEF(b"F_BODCOD_TESTB", 1001, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Verify that the newer name takes precedence.
    //
    spicelib::BODC2N(1001, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"F_BODCOD_TESTB", OK, ctx)?;

    //
    // Clean up
    //
    spicelib::CLPOOL(ctx)?;
    spicelib::ZZBODRST(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Multiple NAMES to a single code, TEXT KERNEL", ctx)?;

    fstr::assign(save.BUFFER.get_mut(1), b"NAIF_BODY_CODE = 1002");
    fstr::assign(
        save.BUFFER.get_mut(2),
        b"NAIF_BODY_NAME = \'F_BODCOD_TESTC\'",
    );

    spicelib::LMPOOL(save.BUFFER.as_arg(), 2, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Verify that the assignment worked.
    //
    spicelib::BODC2N(1002, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"F_BODCOD_TESTC", OK, ctx)?;

    //
    // Now append a new name with the same code.
    //
    fstr::assign(save.BUFFER.get_mut(1), b"NAIF_BODY_CODE += 1002");
    fstr::assign(
        save.BUFFER.get_mut(2),
        b"NAIF_BODY_NAME += \'F_BODCOD_ATEST\'",
    );

    spicelib::LMPOOL(save.BUFFER.as_arg(), 2, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Verify that the newer name takes precedence.
    //
    spicelib::BODC2N(1002, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"F_BODCOD_ATEST", OK, ctx)?;

    //
    // Depending on the sorting algorithm this could go a number
    // of ways.  Check a name that would occur "after" F_BODCOD_TESTC
    // in the list as well.
    //
    fstr::assign(save.BUFFER.get_mut(1), b"NAIF_BODY_CODE += 1002");
    fstr::assign(
        save.BUFFER.get_mut(2),
        b"NAIF_BODY_NAME += \'F_BODCOD_TESTD\'",
    );

    spicelib::LMPOOL(save.BUFFER.as_arg(), 2, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Verify that the newer name takes precedence.
    //
    spicelib::BODC2N(1002, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"F_BODCOD_TESTD", OK, ctx)?;

    //
    // Clean up
    //
    spicelib::CLPOOL(ctx)?;
    spicelib::ZZBODRST(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Multiple NAMES to a single code, BODDEF then TEXT", ctx)?;

    spicelib::BODDEF(b"F_BODCOD_TESTE", 1003, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check that this assignment worked as expected.
    //
    spicelib::BODC2N(1003, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"F_BODCOD_TESTE", OK, ctx)?;

    //
    // Now add a new name with the text kernel system.
    //
    fstr::assign(save.BUFFER.get_mut(1), b"NAIF_BODY_CODE = 1003");
    fstr::assign(
        save.BUFFER.get_mut(2),
        b"NAIF_BODY_NAME = \'F_BODCOD_TESTF\'",
    );

    spicelib::LMPOOL(save.BUFFER.as_arg(), 2, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Verify that the text-kernel name takes precedence.
    //
    spicelib::BODC2N(1003, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"F_BODCOD_TESTF", OK, ctx)?;

    //
    // Clean up
    //
    spicelib::CLPOOL(ctx)?;
    spicelib::ZZBODRST(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Multiple NAMES to a single code, TEXT then BODDEF", ctx)?;

    fstr::assign(save.BUFFER.get_mut(1), b"NAIF_BODY_CODE = 1004");
    fstr::assign(
        save.BUFFER.get_mut(2),
        b"NAIF_BODY_NAME = \'F_BODCOD_TESTG\'",
    );

    spicelib::LMPOOL(save.BUFFER.as_arg(), 2, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check that this assignment worked as expected.
    //
    spicelib::BODC2N(1004, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"F_BODCOD_TESTG", OK, ctx)?;

    //
    // Now add a new name with the text kernel system.
    //
    spicelib::BODDEF(b"F_BODCOD_TESTH", 1004, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Verify that the text-kernel name takes precedence.
    //
    spicelib::BODC2N(1004, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"F_BODCOD_TESTG", OK, ctx)?;

    //
    // Clean up
    //
    spicelib::CLPOOL(ctx)?;
    spicelib::ZZBODRST(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check whitespace/case-sensitivity on name, BODDEF", ctx)?;

    spicelib::BODDEF(b"F_BODcOd_TestJ  space", 1007, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check that the assignment works properly.
    //
    spicelib::BODC2N(1007, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"F_BODcOd_TestJ  space", OK, ctx)?;

    //
    // Add the same name with different case/spacing.
    //
    spicelib::BODDEF(b"F_BoDCoD_TesTj    space", 1007, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check that the assignment worked properly.
    //
    spicelib::BODC2N(1007, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(
        b"NAME",
        &save.NAME,
        b"=",
        b"F_BoDCoD_TesTj    space",
        OK,
        ctx,
    )?;

    //
    // Clean up
    //
    spicelib::CLPOOL(ctx)?;
    spicelib::ZZBODRST(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check whitespace/case-sensitivity on name, TEXT", ctx)?;

    fstr::assign(save.BUFFER.get_mut(1), b"NAIF_BODY_CODE = 1008");
    fstr::assign(
        save.BUFFER.get_mut(2),
        b"NAIF_BODY_NAME = \'f_bodCoD_TesTK    blank\'",
    );

    spicelib::LMPOOL(save.BUFFER.as_arg(), 2, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check that the assignment works properly.
    //
    spicelib::BODC2N(1008, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(
        b"NAME",
        &save.NAME,
        b"=",
        b"f_bodCoD_TesTK    blank",
        OK,
        ctx,
    )?;

    //
    // Attempt to override this with a BODDEF assignment.
    //
    spicelib::BODDEF(b"F_bodCOD_TESTK   space", 1008, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check to see that this assignment worked.
    //
    spicelib::BODN2C(
        b"F_BODCOD_TESTK SPACE",
        &mut save.CODE,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", save.CODE, b"=", 1008, 0, OK, ctx)?;

    //
    // Now see that the latest text-kernel entry takes precedence.
    //
    spicelib::BODC2N(1008, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(
        b"NAME",
        &save.NAME,
        b"=",
        b"f_bodCoD_TesTK    blank",
        OK,
        ctx,
    )?;

    //
    // Clean up
    //
    spicelib::CLPOOL(ctx)?;
    spicelib::ZZBODRST(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test diagnostics on text-kernel load.", ctx)?;

    //
    // Clear the pool.
    //
    spicelib::CLPOOL(ctx)?;

    //
    // Construct a text-kernel with name-code assignments that
    // will cause FURNSH to signal an error upon loading.
    //
    fstr::assign(&mut save.TKNAME, b"testtk.tk");

    testutil::BEGDAT(&mut save.BUFFER[1]);
    fstr::assign(save.BUFFER.get_mut(2), b"NAIF_BODY_CODE = ( 1009, 1010 )");
    fstr::assign(save.BUFFER.get_mut(3), b"NAIF_BODY_NAME = ( \'SPUD\' )");

    testutil::TSTTXT(&save.TKNAME, save.BUFFER.as_arg(), 3, false, true, ctx)?;

    //
    // Load the kernel and verify that an error has been signaled.
    //
    spicelib::FURNSH(&save.TKNAME, ctx)?;

    testutil::CHCKXC(true, b"SPICE(BADDIMENSIONS)", OK, ctx)?;

    spicelib::CLPOOL(ctx)?;
    testutil::KILFIL(&save.TKNAME, ctx)?;

    //
    // Try a simple code to name conversion to see if the error
    // is still floating around.
    //
    spicelib::BODC2N(399, &mut save.NAME, &mut save.FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Clean up
    //
    spicelib::CLPOOL(ctx)?;
    spicelib::ZZBODRST(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     This test case is commented out, because no changes were made
    //     to BODDEF in N0053.
    //
    testutil::TCASE(b"BODDEF multiple codes to single name masking.", ctx)?;

    spicelib::BODDEF(b"F_BODCOD_TESTI", 1005, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check that this assignment worked as expected.
    //
    spicelib::BODC2N(1005, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"F_BODCOD_TESTI", OK, ctx)?;

    //
    // Now attempt to assign F_BODCOD_TESTI a new ID code, this
    // should be allowed due to masking updates.
    //
    spicelib::BODDEF(b"F_BODCOD_TESTI", 1006, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check that the results of the assignment work as expected.
    //
    spicelib::BODC2N(1006, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"F_BODCOD_TESTI", OK, ctx)?;

    fstr::assign(&mut save.NAME, b"<UNCHANGED>");
    spicelib::BODC2N(1005, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"<UNCHANGED>", OK, ctx)?;

    //
    // Clean up
    //
    spicelib::CLPOOL(ctx)?;
    spicelib::ZZBODRST(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"TEXT multiple codes to single name masking.", ctx)?;

    fstr::assign(save.BUFFER.get_mut(1), b"NAIF_BODY_CODE = 1011");
    fstr::assign(
        save.BUFFER.get_mut(2),
        b"NAIF_BODY_NAME = \'F_BODCOD_TESTL\'",
    );

    spicelib::LMPOOL(save.BUFFER.as_arg(), 2, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check that this assignment worked as expected.
    //
    spicelib::BODC2N(1011, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"F_BODCOD_TESTL", OK, ctx)?;

    //
    // Now attempt to assign F_BODCOD_TEST_L a new ID code.  This
    // should be allowed due to masking updates.
    //
    fstr::assign(save.BUFFER.get_mut(1), b"NAIF_BODY_CODE += 1012");
    fstr::assign(
        save.BUFFER.get_mut(2),
        b"NAIF_BODY_NAME += \'F_BODCOD_TESTL\'",
    );

    spicelib::LMPOOL(save.BUFFER.as_arg(), 2, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check that the results of the assignment work as expected.
    //
    spicelib::BODC2N(1012, &mut save.NAME, &mut save.FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"F_BODCOD_TESTL", OK, ctx)?;

    fstr::assign(&mut save.NAME, b"<UNCHANGED>");
    spicelib::BODC2N(1011, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"<UNCHANGED>", OK, ctx)?;

    //
    // Clean up
    //
    spicelib::CLPOOL(ctx)?;
    spicelib::ZZBODRST(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Simple BODDEF masked by TEXT case.", ctx)?;

    spicelib::BODDEF(b"F_BODCOD_TESTM", 1013, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check that this assignment worked as expected.
    //
    spicelib::BODC2N(1013, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"F_BODCOD_TESTM", OK, ctx)?;

    //
    // Now load a definition that masks this name.
    //
    fstr::assign(save.BUFFER.get_mut(1), b"NAIF_BODY_CODE = 1014");
    fstr::assign(
        save.BUFFER.get_mut(2),
        b"NAIF_BODY_NAME = \'F_BODCOD_TESTM\'",
    );

    spicelib::LMPOOL(save.BUFFER.as_arg(), 2, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Perform all of the necessary state checks at this point.
    //
    spicelib::BODN2C(b"F_BODCOD_TESTM", &mut save.CODE, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", save.CODE, b"=", 1014, 0, OK, ctx)?;

    spicelib::BODC2N(1014, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"F_BODCOD_TESTM", OK, ctx)?;

    //
    // The original BODDEF code should now be effectively masked.
    //
    fstr::assign(&mut save.NAME, b"<UNCHANGED>");
    spicelib::BODC2N(1013, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"<UNCHANGED>", OK, ctx)?;

    //
    // Now clear the pool to unload the text kernel assignment.
    //
    spicelib::CLPOOL(ctx)?;

    spicelib::BODN2C(b"F_BODCOD_TESTM", &mut save.CODE, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", save.CODE, b"=", 1013, 0, OK, ctx)?;

    spicelib::BODC2N(1013, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"F_BODCOD_TESTM", OK, ctx)?;

    fstr::assign(&mut save.NAME, b"<UNCHANGED>");
    spicelib::BODC2N(1014, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"<UNCHANGED>", OK, ctx)?;

    //
    // Clean up
    //
    spicelib::CLPOOL(ctx)?;
    spicelib::ZZBODRST(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"TEXT first, invisible BODDEF", ctx)?;

    //
    // Load a definition via the text-kernel interface.
    //
    fstr::assign(save.BUFFER.get_mut(1), b"NAIF_BODY_CODE = 1015");
    fstr::assign(
        save.BUFFER.get_mut(2),
        b"NAIF_BODY_NAME = \'F_BODCOD_TESTN\'",
    );

    spicelib::LMPOOL(save.BUFFER.as_arg(), 2, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check that this assignment worked as expected.
    //
    spicelib::BODC2N(1015, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"F_BODCOD_TESTN", OK, ctx)?;

    //
    // Now make a BODDEF assignment that would appear to be
    // invisible as a result of masking.
    //
    spicelib::BODDEF(b"F_BODCOD_TESTN", 1016, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Perform all of the necessary state checks at this point.
    //
    spicelib::BODN2C(b"F_BODCOD_TESTN", &mut save.CODE, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", save.CODE, b"=", 1015, 0, OK, ctx)?;

    spicelib::BODC2N(1015, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"F_BODCOD_TESTN", OK, ctx)?;

    //
    // The BODDEF code should now be effectively masked.
    //
    fstr::assign(&mut save.NAME, b"<UNCHANGED>");
    spicelib::BODC2N(1016, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"<UNCHANGED>", OK, ctx)?;

    //
    // Now clear the pool to unload the text kernel assignment.
    //
    spicelib::CLPOOL(ctx)?;

    spicelib::BODN2C(b"F_BODCOD_TESTN", &mut save.CODE, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", save.CODE, b"=", 1016, 0, OK, ctx)?;

    spicelib::BODC2N(1016, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"F_BODCOD_TESTN", OK, ctx)?;

    fstr::assign(&mut save.NAME, b"<UNCHANGED>");
    spicelib::BODC2N(1015, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"<UNCHANGED>", OK, ctx)?;

    //
    // Clean up
    //
    spicelib::CLPOOL(ctx)?;
    spicelib::ZZBODRST(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"BODDEF overriding built-in codes", ctx)?;

    //
    // Obtain the current code for body to override.
    //
    spicelib::BODN2C(b"EARTH", &mut save.RESTOR, &mut save.FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // Now override the code for EARTH with a new one.
    //
    spicelib::BODDEF(b"EARTH", 1017, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check that the assignment worked.
    //
    spicelib::BODN2C(b"EARTH", &mut save.CODE, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", save.CODE, b"=", 1017, 0, OK, ctx)?;

    spicelib::BODC2N(1017, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"EARTH", OK, ctx)?;

    fstr::assign(&mut save.NAME, b"<UNCHANGED>");
    spicelib::BODC2N(save.RESTOR, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"<UNCHANGED>", OK, ctx)?;

    //
    // Restore the original code for EARTH.
    //
    spicelib::BODDEF(b"EARTH", save.RESTOR, ctx)?;

    //
    // Check that it is restored.
    //
    spicelib::BODN2C(b"EARTH", &mut save.CODE, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", save.CODE, b"=", save.RESTOR, 0, OK, ctx)?;

    spicelib::BODC2N(save.RESTOR, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"EARTH", OK, ctx)?;

    //
    // Clean up
    //
    spicelib::CLPOOL(ctx)?;
    spicelib::ZZBODRST(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"TEXT overriding built-in codes", ctx)?;

    spicelib::BODN2C(b"MARS", &mut save.RESTOR, &mut save.FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // Load a definition via the text-kernel interface that blasts
    // a built in code.
    //
    fstr::assign(save.BUFFER.get_mut(1), b"NAIF_BODY_CODE = 1018");
    fstr::assign(save.BUFFER.get_mut(2), b"NAIF_BODY_NAME = \'MARS\'");

    spicelib::LMPOOL(save.BUFFER.as_arg(), 2, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check that the assignment worked.
    //
    spicelib::BODN2C(b"MARS", &mut save.CODE, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", save.CODE, b"=", 1018, 0, OK, ctx)?;

    spicelib::BODC2N(1018, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"MARS", OK, ctx)?;

    fstr::assign(&mut save.NAME, b"<UNCHANGED>");
    spicelib::BODC2N(save.RESTOR, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"<UNCHANGED>", OK, ctx)?;

    //
    // Restore the original assignment.
    //
    spicelib::CLPOOL(ctx)?;

    //
    // Check that it is restored.
    //
    spicelib::BODN2C(b"MARS", &mut save.CODE, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", save.CODE, b"=", save.RESTOR, 0, OK, ctx)?;

    spicelib::BODC2N(save.RESTOR, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"MARS", OK, ctx)?;

    //
    // Clean up
    //
    spicelib::CLPOOL(ctx)?;
    spicelib::ZZBODRST(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Fill TEXT buffer with unique entries", ctx)?;

    //
    // Load up many more definitions than ZZBODTRN can handle.
    //
    for I in 1..=NROOM {
        fstr::assign(save.BUFFER.get_mut(1), b"NAIF_BODY_CODE += #");
        fstr::assign(
            save.BUFFER.get_mut(2),
            b"NAIF_BODY_NAME += \'F_BODCOD_TEST_#\'",
        );

        spicelib::REPMI(
            &save.BUFFER[1].to_vec(),
            b"#",
            (TKBASE + I),
            &mut save.BUFFER[1],
            ctx,
        );
        spicelib::REPMI(
            &save.BUFFER[2].to_vec(),
            b"#",
            (TKBASE + I),
            &mut save.BUFFER[2],
            ctx,
        );

        spicelib::LMPOOL(save.BUFFER.as_arg(), 2, ctx)?;
    }

    //
    // See what happens...
    //
    for I in 1..=NROOM {
        fstr::assign(&mut save.HOLD, b"F_BODCOD_TEST_#");
        fstr::assign(&mut save.NAME, b"<UNCHANGED>");
        spicelib::REPMI(&save.HOLD.to_vec(), b"#", (TKBASE + I), &mut save.HOLD, ctx);

        spicelib::BODC2N((TKBASE + I), &mut save.NAME, &mut save.FOUND, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
        testutil::CHCKSC(b"NAME", &save.NAME, b"=", &save.HOLD, OK, ctx)?;

        spicelib::BODN2C(&save.HOLD, &mut save.CODE, &mut save.FOUND, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
        testutil::CHCKSI(b"CODE", save.CODE, b"=", (TKBASE + I), 0, OK, ctx)?;
    }

    //
    // Clear the pool, resetting kernel pool variable assignments.
    //
    spicelib::CLPOOL(ctx)?;
    spicelib::ZZBODRST(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Overflow TEXT buffer with unique entries.", ctx)?;

    //
    // Load up many more definitions than ZZBODTRN can handle.
    //
    for I in 1..=(NROOM + 1) {
        fstr::assign(save.BUFFER.get_mut(1), b"NAIF_BODY_CODE += #");
        fstr::assign(
            save.BUFFER.get_mut(2),
            b"NAIF_BODY_NAME += \'F_BODCOD_TEST_#\'",
        );

        spicelib::REPMI(
            &save.BUFFER[1].to_vec(),
            b"#",
            (TKBASE + I),
            &mut save.BUFFER[1],
            ctx,
        );
        spicelib::REPMI(
            &save.BUFFER[2].to_vec(),
            b"#",
            (TKBASE + I),
            &mut save.BUFFER[2],
            ctx,
        );

        spicelib::LMPOOL(save.BUFFER.as_arg(), 2, ctx)?;
    }

    //
    // At this point we're set to observe an error signalled by the first
    // routine that causes the processing of the NAME/CODE kernel pool
    // variable pair.
    //
    fstr::assign(&mut save.NAME, b"<UNCHANGED>");
    spicelib::BODC2N((TKBASE + 1), &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKXC(true, b"SPICE(KERVARTOOBIG)", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"<UNCHANGED>", OK, ctx)?;

    //
    // Clear the pool, resetting kernel pool variable assignments.
    //
    spicelib::CLPOOL(ctx)?;
    spicelib::ZZBODRST(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Fill TEXT buffer with doubled up entries.", ctx)?;

    //
    // Fill TEXT buffer with doubled up entries.
    //
    for I in 1..=((NROOM / 2) * 2) {
        fstr::assign(save.BUFFER.get_mut(1), b"NAIF_BODY_CODE += #");
        fstr::assign(
            save.BUFFER.get_mut(2),
            b"NAIF_BODY_NAME += \'F_BODCOD_TEST_#\'",
        );

        spicelib::REPMI(
            &save.BUFFER[1].to_vec(),
            b"#",
            (TKBASE + I),
            &mut save.BUFFER[1],
            ctx,
        );
        spicelib::REPMI(
            &save.BUFFER[2].to_vec(),
            b"#",
            (TKBASE + ((I + 1) / 2)),
            &mut save.BUFFER[2],
            ctx,
        );

        spicelib::LMPOOL(save.BUFFER.as_arg(), 2, ctx)?;
    }

    //
    // Check results.
    //
    for I in 1..=((NROOM / 2) * 2) {
        fstr::assign(&mut save.HOLD, b"F_BODCOD_TEST_#");
        fstr::assign(&mut save.NAME, b"<UNCHANGED>");
        spicelib::REPMI(
            &save.HOLD.to_vec(),
            b"#",
            (TKBASE + ((I + 1) / 2)),
            &mut save.HOLD,
            ctx,
        );

        spicelib::BODC2N((TKBASE + I), &mut save.NAME, &mut save.FOUND, ctx)?;

        if (intrinsics::MOD(I, 2) == 0) {
            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
            testutil::CHCKSC(b"NAME", &save.NAME, b"=", &save.HOLD, OK, ctx)?;
        } else {
            testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;
            testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"<UNCHANGED>", OK, ctx)?;
        }

        spicelib::BODN2C(&save.HOLD, &mut save.CODE, &mut save.FOUND, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
        testutil::CHCKSI(
            b"CODE",
            save.CODE,
            b"=",
            ((TKBASE + I) + intrinsics::MOD(I, 2)),
            0,
            OK,
            ctx,
        )?;
    }

    //
    // Clear the pool, resetting kernel pool variable assignments.
    //
    spicelib::CLPOOL(ctx)?;
    spicelib::ZZBODRST(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Fill TEXT buffer with the same masked entries.", ctx)?;

    //
    // Load up one name with NROOM masks.
    //
    for I in 1..=NROOM {
        fstr::assign(save.BUFFER.get_mut(1), b"NAIF_BODY_CODE += #");
        fstr::assign(
            save.BUFFER.get_mut(2),
            b"NAIF_BODY_NAME += \'F_BODCOD_TEST_DUPES\'",
        );

        spicelib::REPMI(
            &save.BUFFER[1].to_vec(),
            b"#",
            (TKBASE + I),
            &mut save.BUFFER[1],
            ctx,
        );

        spicelib::LMPOOL(save.BUFFER.as_arg(), 2, ctx)?;
    }

    //
    // Check the state of the system.
    //
    for I in 1..=(NROOM - 1) {
        fstr::assign(&mut save.NAME, b"<UNCHANGED>");

        spicelib::BODC2N((TKBASE + I), &mut save.NAME, &mut save.FOUND, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;
        testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"<UNCHANGED>", OK, ctx)?;
    }

    spicelib::BODC2N((TKBASE + NROOM), &mut save.NAME, &mut save.FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"F_BODCOD_TEST_DUPES", OK, ctx)?;

    spicelib::BODN2C(b"F_BODCOD_TEST_DUPES", &mut save.CODE, &mut save.FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", save.CODE, b"=", (TKBASE + NROOM), 0, OK, ctx)?;

    //
    // Clear the pool to reset the variables.
    //
    spicelib::CLPOOL(ctx)?;
    spicelib::ZZBODRST(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    //     This test case must be commented out in general, because
    //     BODDEF assignments (as of yet) can not be removed once made.
    //
    testutil::TCASE(b"Overflow BODDEF buffer.", ctx)?;

    //
    // We expect this to result in an error being signaled, although
    // when this error is signaled is not exactly known.  This is
    // because it is not clear how many unique BODDEF assignments
    // prior to this test family's execution have occurred.
    //
    save.J = (MAXE - NPERM);

    for I in 1..=save.J {
        fstr::assign(&mut save.NAME, b"F_BODCOD_TEST_#");
        spicelib::REPMI(&save.NAME.to_vec(), b"#", (BDBASE + I), &mut save.NAME, ctx);

        spicelib::BODDEF(&save.NAME, (BDBASE + I), ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    fstr::assign(&mut save.NAME, b"F_BODCOD_TEST_#");
    spicelib::REPMI(
        &save.NAME.to_vec(),
        b"#",
        ((BDBASE + save.J) + 1),
        &mut save.NAME,
        ctx,
    );

    spicelib::BODDEF(&save.NAME, ((BDBASE + save.J) + 1), ctx)?;

    testutil::CHCKXC(true, b"SPICE(TOOMANYPAIRS)", OK, ctx)?;

    //
    // Clean up
    //
    spicelib::CLPOOL(ctx)?;
    spicelib::ZZBODRST(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check proper handling of spacing by TEXT", ctx)?;

    fstr::assign(save.BUFFER.get_mut(1), b"NAIF_BODY_CODE = 1019");
    fstr::assign(save.BUFFER.get_mut(2), b"NAIF_BODY_NAME = \'TEST ING\'");
    fstr::assign(save.BUFFER.get_mut(3), b"NAIF_BODY_CODE += 1020");
    fstr::assign(save.BUFFER.get_mut(4), b"NAIF_BODY_NAME += \'TEST    ING\'");
    fstr::assign(save.BUFFER.get_mut(5), b"NAIF_BODY_CODE += 1021");
    fstr::assign(save.BUFFER.get_mut(6), b"NAIF_BODY_NAME += \'TESTING\'");

    spicelib::LMPOOL(save.BUFFER.as_arg(), 6, ctx)?;

    //
    // Check 'TESTING' -> 1021 mapping.
    //
    spicelib::BODN2C(b"TESTING", &mut save.CODE, &mut save.FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", save.CODE, b"=", 1021, 0, OK, ctx)?;

    //
    // Check 'TEST ING' -> 1020 (masked) mapping.
    //
    spicelib::BODN2C(b"TEST ING", &mut save.CODE, &mut save.FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", save.CODE, b"=", 1020, 0, OK, ctx)?;

    //
    // Check 'TEST    ING' -> 1020 mapping.
    //
    spicelib::BODN2C(b"TEST    ING", &mut save.CODE, &mut save.FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", save.CODE, b"=", 1020, 0, OK, ctx)?;

    //
    // Lookup up the masked code, 1019.
    //
    fstr::assign(&mut save.NAME, b"<UNCHANGED>");

    spicelib::BODC2N(1019, &mut save.NAME, &mut save.FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"<UNCHANGED>", OK, ctx)?;

    //
    // Lookup the 1020 -> 'TEST    ING' mapping.
    //
    spicelib::BODC2N(1020, &mut save.NAME, &mut save.FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"TEST    ING", OK, ctx)?;

    //
    // Lookup the 1021 -> 'TESTING' mapping.
    //
    spicelib::BODC2N(1021, &mut save.NAME, &mut save.FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"TESTING", OK, ctx)?;

    //
    // Clean up.
    //
    spicelib::CLPOOL(ctx)?;
    spicelib::ZZBODRST(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check proper handling of spacing by BODDEF", ctx)?;

    spicelib::BODDEF(b"TEST ING2", 1019, ctx)?;
    spicelib::BODDEF(b"TEST    ING2", 1020, ctx)?;
    spicelib::BODDEF(b"TESTING2", 1021, ctx)?;

    //
    // Check 'TESTING2' -> 1021 mapping.
    //
    spicelib::BODN2C(b"TESTING2", &mut save.CODE, &mut save.FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", save.CODE, b"=", 1021, 0, OK, ctx)?;

    //
    // Check 'TEST ING2' -> 1020 (masked) mapping.
    //
    spicelib::BODN2C(b"TEST ING2", &mut save.CODE, &mut save.FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", save.CODE, b"=", 1020, 0, OK, ctx)?;

    //
    // Check 'TEST    ING2' -> 1020 mapping.
    //
    spicelib::BODN2C(b"TEST    ING2", &mut save.CODE, &mut save.FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", save.CODE, b"=", 1020, 0, OK, ctx)?;

    //
    // Lookup up the masked code, 1019.
    //
    fstr::assign(&mut save.NAME, b"<UNCHANGED>");

    spicelib::BODC2N(1019, &mut save.NAME, &mut save.FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"<UNCHANGED>", OK, ctx)?;

    //
    // Lookup the 1020 -> 'TEST    ING2' mapping.
    //
    spicelib::BODC2N(1020, &mut save.NAME, &mut save.FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"TEST    ING2", OK, ctx)?;

    //
    // Lookup the 1021 -> 'TESTING2' mapping.
    //
    spicelib::BODC2N(1021, &mut save.NAME, &mut save.FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"TESTING2", OK, ctx)?;

    //
    // Clean up
    //
    spicelib::CLPOOL(ctx)?;
    spicelib::ZZBODRST(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Replace all built-in assignments with uniques", ctx)?;

    //
    // Set all built-in codes to point to their position in the
    // original sequence.
    //
    for I in 1..=NPERM {
        spicelib::BODDEF(&save.BASNAM[I], I, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Verify that the changes worked.
    //
    for I in 1..=NPERM {
        spicelib::BODC2N(I, &mut save.NAME, &mut save.FOUND, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
        testutil::CHCKSC(b"NAME", &save.NAME, b"=", &save.BASNAM[I], OK, ctx)?;

        spicelib::BODN2C(&save.BASNAM[I], &mut save.CODE, &mut save.FOUND, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
        testutil::CHCKSI(b"CODE", save.CODE, b"=", I, 0, OK, ctx)?;
    }

    //
    // Restore built-in codes.
    //
    for I in 1..=NPERM {
        spicelib::BODDEF(&save.BASNAM[I], save.BASCOD[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Clean up
    //
    spicelib::CLPOOL(ctx)?;
    spicelib::ZZBODRST(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Point all built-ins at a single code.", ctx)?;

    //
    // Set all built-in names to point to the code 1.
    //
    for I in 1..=NPERM {
        spicelib::BODDEF(&save.BASNAM[I], 1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Verify that the changes worked properly.
    //
    for I in 1..=NPERM {
        spicelib::BODN2C(&save.BASNAM[I], &mut save.CODE, &mut save.FOUND, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
        testutil::CHCKSI(b"CODE", save.CODE, b"=", 1, 0, OK, ctx)?;
    }

    spicelib::BODC2N(1, &mut save.NAME, &mut save.FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", &save.BASNAM[NPERM], OK, ctx)?;

    //
    // Restore built-in codes.
    //
    for I in 1..=NPERM {
        spicelib::BODDEF(&save.BASNAM[I], save.BASCOD[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Clean up
    //
    spicelib::CLPOOL(ctx)?;
    spicelib::ZZBODRST(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Point all built-ins at a single name.", ctx)?;

    //
    // Set all built-in codes to point at 'NAME'.
    //
    for I in 1..=NPERM {
        spicelib::BODDEF(b"NAME", save.BASCOD[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Verify that the changes worked properly.
    //
    for I in 1..=(NPERM - 1) {
        spicelib::BODC2N(save.BASCOD[I], &mut save.NAME, &mut save.FOUND, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

        save.FOUND = false;
        save.J = NPERM;
        while (!save.FOUND && (save.J > I)) {
            if (save.BASCOD[save.J] == save.BASCOD[I]) {
                save.FOUND = true;
            } else {
                save.J = (save.J - 1);
            }
        }

        testutil::CHCKSC(b"NAME", &save.NAME, b"=", &save.BASNAM[save.J], OK, ctx)?;
    }

    spicelib::BODC2N(save.BASCOD[NPERM], &mut save.NAME, &mut save.FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"NAME", OK, ctx)?;

    spicelib::BODN2C(b"NAME", &mut save.CODE, &mut save.FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", save.CODE, b"=", save.BASCOD[NPERM], 0, OK, ctx)?;

    //
    // Restore built-in codes.
    //
    for I in 1..=NPERM {
        spicelib::BODDEF(&save.BASNAM[I], save.BASCOD[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Clean up
    //
    spicelib::CLPOOL(ctx)?;
    spicelib::ZZBODRST(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check built-in assignment integrity.", ctx)?;

    //
    // Verify all of the codes and name pairs that exist in the
    // built-in NAME/CODE arrays.
    //
    for I in 1..=NPERM {
        save.CODE = 0;
        fstr::assign(&mut save.NAME, b"<UNCHANGED>");

        spicelib::BODN2C(&save.BASNAM[I], &mut save.CODE, &mut save.FOUND, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
        testutil::CHCKSI(b"CODE", save.CODE, b"=", save.BASCOD[I], 0, OK, ctx)?;

        spicelib::BODC2N(save.BASCOD[I], &mut save.NAME, &mut save.FOUND, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

        //
        // Locate the name that should be found by searching in front
        // of the index I for a match.
        //
        save.FOUND = false;
        save.J = NPERM;
        while (!save.FOUND && (save.J > I)) {
            if (save.BASCOD[save.J] == save.BASCOD[I]) {
                save.FOUND = true;
            } else {
                save.J = (save.J - 1);
            }
        }

        //
        // At this point J is either I, or J is pointing at the
        // index with the highest precedent assignment for BASCOD(I).
        //
        testutil::CHCKSC(b"NAME", &save.NAME, b"=", &save.BASNAM[save.J], OK, ctx)?;
    }

    //
    // Clean up
    //
    spicelib::CLPOOL(ctx)?;
    spicelib::ZZBODRST(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check name to ID translation by BODS2C", ctx)?;

    spicelib::BODS2C(b"JUPITER", &mut save.CODE, &mut save.FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", save.CODE, b"=", 599, 0, OK, ctx)?;

    save.CODE = 777;
    spicelib::BODS2C(b"JUP", &mut save.CODE, &mut save.FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;
    testutil::CHCKSI(b"CODE", save.CODE, b"=", 777, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check integer to ID translation by BODS2C", ctx)?;

    spicelib::BODS2C(b"599", &mut save.CODE, &mut save.FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", save.CODE, b"=", 599, 0, OK, ctx)?;

    spicelib::BODS2C(b"1000000000", &mut save.CODE, &mut save.FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", save.CODE, b"=", 1000000000, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check ZZBODTRN counter update", ctx)?;

    //
    // Clean up
    //
    spicelib::CLPOOL(ctx)?;
    spicelib::ZZBODRST(ctx)?;

    //
    // Initialize the local POOL state counter.
    //
    spicelib::ZZCTRUIN(save.USRCTR.as_slice_mut(), ctx);

    //
    // Test initial counter update and no update on the second
    // immediate call.
    //
    save.UPDATE = false;
    spicelib::ZZBCTRCK(save.USRCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;
    testutil::CHCKSL(b"initial counter UPDATE", save.UPDATE, true, OK, ctx)?;

    save.UPDATE = true;
    spicelib::ZZBCTRCK(save.USRCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;
    testutil::CHCKSL(b"no initial counter UPDATE", save.UPDATE, false, OK, ctx)?;

    //
    // Add a mapping to the POOL. Check counter after BODN2C.
    //
    fstr::assign(save.BUFFER.get_mut(1), b"NAIF_BODY_CODE = 1002");
    fstr::assign(
        save.BUFFER.get_mut(2),
        b"NAIF_BODY_NAME = \'F_BODCOD_1002\'",
    );
    spicelib::LMPOOL(save.BUFFER.as_arg(), 2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODN2C(b"F_BODCOD_1002", &mut save.CODE, &mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", save.CODE, b"=", 1002, 0, OK, ctx)?;

    save.UPDATE = false;
    spicelib::ZZBCTRCK(save.USRCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;
    testutil::CHCKSL(b"Post-BODN2C counter UPDATE", save.UPDATE, true, OK, ctx)?;

    //
    // Add a mapping to the POOL. Check counter after BODC2N.
    //
    fstr::assign(save.BUFFER.get_mut(1), b"NAIF_BODY_CODE = 1003");
    fstr::assign(
        save.BUFFER.get_mut(2),
        b"NAIF_BODY_NAME = \'F_BODCOD_1003\'",
    );
    spicelib::LMPOOL(save.BUFFER.as_arg(), 2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODC2N(1003, &mut save.NAME, &mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &save.NAME, b"=", b"F_BODCOD_1003", OK, ctx)?;

    save.UPDATE = false;
    spicelib::ZZBCTRCK(save.USRCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"Post-BODC2N counter UPDATE", save.UPDATE, true, OK, ctx)?;

    //
    // Add a mapping using BODDEF. Check counter.
    //
    spicelib::BODDEF(b"F_BODCOD_1004", 1004, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.UPDATE = false;
    spicelib::ZZBCTRCK(save.USRCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"Post-BODDEF counter UPDATE", save.UPDATE, true, OK, ctx)?;

    //
    // Add a mapping to the POOL. Check counter after ZZBODKIK.
    //
    fstr::assign(save.BUFFER.get_mut(1), b"NAIF_BODY_CODE = 1005");
    fstr::assign(
        save.BUFFER.get_mut(2),
        b"NAIF_BODY_NAME = \'F_BODCOD_1005\'",
    );
    spicelib::LMPOOL(save.BUFFER.as_arg(), 2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZBODKIK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.UPDATE = false;
    spicelib::ZZBCTRCK(save.USRCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"Post-ZZBODKIK counter UPDATE", save.UPDATE, true, OK, ctx)?;

    //
    // We already added a BODDEF mapping. Check counter after ZZBODRST.
    //
    spicelib::ZZBODRST(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.UPDATE = false;
    spicelib::ZZBCTRCK(save.USRCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"Post-ZZBODRST counter UPDATE", save.UPDATE, true, OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
