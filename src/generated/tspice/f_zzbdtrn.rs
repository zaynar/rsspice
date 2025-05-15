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
const LNSIZE: i32 = 80;

//$Procedure F_ZZBDTRN ( Family of tests for ZZBODTRN )
pub fn F_ZZBDTRN(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut TEXT = ActualCharArray::new(LNSIZE, 1..=6);
    let mut GET = [b' '; MAXL as usize];
    let mut CODE: i32 = 0;
    let mut FOUND: bool = false;

    //
    // Test Utility Functions
    //

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //

    testutil::TOPEN(b"F_ZZBDTRN", ctx)?;

    //
    // CASE 1
    //
    testutil::TCASE(
        b"Checking to make sure FOUND comes back FALSE for a bogus name. ",
        ctx,
    )?;

    spicelib::BODN2C(b"SPUD", &mut CODE, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    //
    // CASE 2
    //
    testutil::TCASE(
        b"Checking to make sure that FOUND comes back FALSE for a bogus ID code.",
        ctx,
    )?;

    spicelib::BODC2N(22, &mut GET, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    //
    // CASE 3
    //
    testutil::TCASE(
        b"Checking to make sure we can load a kernel and add new ID/NAME pairs. ",
        ctx,
    )?;

    testutil::BEGDAT(&mut TEXT[1]);
    fstr::assign(TEXT.get_mut(2), b" NAIF_BODY_CODE  = ( 22, 23, 24, 25 )");
    fstr::assign(
        TEXT.get_mut(3),
        b" NAIF_BODY_NAME  = ( \'SPUD\', \'MOON\', \'CURLEY\', \'SHEMP\' )",
    );

    testutil::TSTTXT(b"sample.nam", TEXT.as_arg(), 3, true, false, ctx)?;

    spicelib::BODN2C(b"SPUD", &mut CODE, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", CODE, b"=", 22, 0, OK, ctx)?;

    spicelib::BODC2N(24, &mut GET, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"GET", &GET, b"=", b"CURLEY", OK, ctx)?;

    //
    // CASE 4
    //
    // Test to check we can override a name-ID assignment from the
    // default list with an external kernel assignment.
    //
    testutil::TCASE(
        b"Checking to make sure we can override an existing NAME/ID pair. ",
        ctx,
    )?;

    spicelib::BODN2C(b"MOON", &mut CODE, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", CODE, b"=", 23, 0, OK, ctx)?;

    //
    // CASE 5
    //
    // Test on external name-ID assignments.
    //
    testutil::TCASE(b"Check to ensure case insensitivity in N2C.", ctx)?;

    testutil::BEGDAT(&mut TEXT[1]);
    fstr::assign(TEXT.get_mut(2), b"NAIF_BODY_CODE =  ( -170101, -170102 )");
    fstr::assign(
        TEXT.get_mut(3),
        b"NAIF_BODY_NAME =  ( \'1701-A\', \'   1701-b\' )",
    );
    fstr::assign(TEXT.get_mut(4), b" ");

    testutil::KILFIL(b"testdata.ker", ctx)?;
    testutil::TSTTXT(b"testdata.ker", TEXT.as_arg(), 4, false, true, ctx)?;
    spicelib::LDPOOL(b"testdata.ker", ctx)?;

    spicelib::BODN2C(b"1701-a", &mut CODE, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", CODE, b"=", -170101, 0, OK, ctx)?;

    spicelib::BODN2C(b" 1701-B ", &mut CODE, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", CODE, b"=", -170102, 0, OK, ctx)?;

    spicelib::BODC2N(-170102, &mut GET, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"GET", &GET, b"=", b"   1701-b", OK, ctx)?;

    //
    // CASE 6
    //
    testutil::TCASE(
        b"Checking that we cannot override an existing kernel based NAME/ID pair.",
        ctx,
    )?;

    testutil::BEGDAT(&mut TEXT[1]);
    fstr::assign(TEXT.get_mut(2), b"NAIF_BODY_CODE =  ( -170101, -170102 )");
    fstr::assign(
        TEXT.get_mut(3),
        b"NAIF_BODY_NAME =  ( \'1701-A\', \'1701-b\' )",
    );
    fstr::assign(TEXT.get_mut(4), b"NAIF_BODY_CODE += ( -170105 )");
    fstr::assign(TEXT.get_mut(5), b"NAIF_BODY_NAME += ( \'1701-a\' )");
    fstr::assign(TEXT.get_mut(6), b" ");

    testutil::KILFIL(b"testdata.ker", ctx)?;
    testutil::TSTTXT(b"testdata.ker", TEXT.as_arg(), 6, false, true, ctx)?;

    //
    // CASE 7
    //
    // Test of BODDEF.
    //
    testutil::TCASE(b"Check BODDEF functionality.", ctx)?;

    spicelib::BODDEF(b"Woof", -10009, ctx)?;

    spicelib::BODC2N(-10009, &mut GET, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"GET", &GET, b"=", b"Woof", OK, ctx)?;

    //
    // Assign a member of the equivalence class to the ID code.
    // As last on the stack, a C2N lookup should return the
    // same member string.
    //
    spicelib::BODDEF(b"woof", -10009, ctx)?;
    spicelib::BODC2N(-10009, &mut GET, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"GET", &GET, b"=", b"woof", OK, ctx)?;

    //
    // Case and space insensitiviy
    //
    spicelib::BODN2C(b" WOOF ", &mut CODE, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", CODE, b"=", -10009, 0, OK, ctx)?;

    //
    // Functionality unaffected by the previous test's error state?
    // Try a BODDEF.
    //
    spicelib::BODDEF(b" WOof ", -10009, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now perform a post error case and space insensitiviy test.
    //
    spicelib::BODN2C(b"   wOOf ", &mut CODE, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", CODE, b"=", -10009, 0, OK, ctx)?;

    //
    // CASE 8
    //
    // Test the mapping precedence behavior.
    //
    // Assign a mapping via BODDEF. Note: 'Tis very important to call
    // CLPOOL an clear the kernel pool before this set of tests. The
    // previous tests left the pool in an error signalling state.
    //
    spicelib::CLPOOL(ctx)?;

    testutil::TCASE(b"Check precedence functionality. BODDEF 1", ctx)?;

    spicelib::BODDEF(b"TestBODEF1", -30001, ctx)?;

    fstr::assign(&mut GET, b" ");
    spicelib::BODC2N(-30001, &mut GET, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"GET", &GET, b"=", b"TestBODEF1", OK, ctx)?;

    spicelib::BODN2C(b"TestBODEF1", &mut CODE, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", CODE, b"=", -30001, 0, OK, ctx)?;

    //
    // CASE 9
    //
    // Load a new mapping to the kernel pool using the previous ID value.
    // This mapping should override BODDEF.
    //
    testutil::TCASE(b"Check precedence functionality. TestKer 1", ctx)?;

    fstr::assign(TEXT.get_mut(1), b"      NAIF_BODY_NAME += \'TestKer1\'");
    fstr::assign(TEXT.get_mut(2), b"      NAIF_BODY_CODE += -30001");

    spicelib::LMPOOL(TEXT.as_arg(), 2, ctx)?;

    fstr::assign(&mut GET, b" ");
    spicelib::BODC2N(-30001, &mut GET, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"GET", &GET, b"=", b"TestKer1", OK, ctx)?;

    spicelib::BODN2C(b"TestKer1", &mut CODE, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", CODE, b"=", -30001, 0, OK, ctx)?;

    //
    // CASE 10
    //
    // Assign another name to the previous ID code. This mapping
    // should override all others.
    //
    testutil::TCASE(b"Check precedence functionality. TestKer 2", ctx)?;

    fstr::assign(TEXT.get_mut(1), b"      NAIF_BODY_NAME += \'TestKer2\'");
    fstr::assign(TEXT.get_mut(2), b"      NAIF_BODY_CODE += -30001");

    spicelib::LMPOOL(TEXT.as_arg(), 2, ctx)?;

    fstr::assign(&mut GET, b" ");
    spicelib::BODC2N(-30001, &mut GET, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"GET", &GET, b"=", b"TestKer2", OK, ctx)?;

    spicelib::BODN2C(b"TestKer2", &mut CODE, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", CODE, b"=", -30001, 0, OK, ctx)?;

    //
    // CASE 11
    //
    // Clear the kernel pool, test for BODDEF's first mapping for the
    // test ID.
    //
    testutil::TCASE(b"Check precedence functionality. BODDEF 1 retest.", ctx)?;

    spicelib::CLPOOL(ctx)?;

    fstr::assign(&mut GET, b" ");
    spicelib::BODC2N(-30001, &mut GET, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"GET", &GET, b"=", b"TestBODEF1", OK, ctx)?;

    spicelib::BODN2C(b"TestBODEF1", &mut CODE, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", CODE, b"=", -30001, 0, OK, ctx)?;

    //
    // CASE 12
    //
    // Assign another mapping via BODDEF to override the first
    // BODDEF mapping.
    //
    testutil::TCASE(b"Check precedence functionality. BODDEF 2.", ctx)?;

    spicelib::BODDEF(b"TestBODEF2", -30001, ctx)?;

    fstr::assign(&mut GET, b" ");
    spicelib::BODC2N(-30001, &mut GET, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"GET", &GET, b"=", b"TestBODEF2", OK, ctx)?;

    spicelib::BODN2C(b"TestBODEF2", &mut CODE, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", CODE, b"=", -30001, 0, OK, ctx)?;

    //
    // CASE 13
    //
    testutil::TCASE(b"Check precedence functionality. Masking.", ctx)?;

    spicelib::CLPOOL(ctx)?;

    //
    // Assign a mapping via BODDEF, then remap the name to a new
    // number.
    //
    spicelib::BODDEF(b"TestName", 20000, ctx)?;
    fstr::assign(TEXT.get_mut(1), b"      NAIF_BODY_NAME = \'TestName\'");
    fstr::assign(TEXT.get_mut(2), b"      NAIF_BODY_CODE = -20000");

    spicelib::LMPOOL(TEXT.as_arg(), 2, ctx)?;

    spicelib::BODN2C(b"TestName", &mut CODE, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", CODE, b"=", -20000, 0, OK, ctx)?;

    fstr::assign(&mut GET, b" ");
    spicelib::BODC2N(-20000, &mut GET, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"GET", &GET, b"=", b"TestName", OK, ctx)?;

    //
    // CASE 14
    //
    //
    // The kernel pool assignment should mask the BODDEF assignment
    // so a request for the name of body 20000 ought to fail.
    //
    testutil::TCASE(b"Check precedence functionality. De-masking.", ctx)?;

    fstr::assign(&mut GET, b" ");
    spicelib::BODC2N(20000, &mut GET, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    //
    // Destroy the kernel pool entries. The BODDEF assignment should now
    // respond; a request for body 20000 should succeed.
    //
    spicelib::CLPOOL(ctx)?;
    fstr::assign(&mut GET, b" ");
    spicelib::BODC2N(20000, &mut GET, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"GET", &GET, b"=", b"TestName", OK, ctx)?;

    //
    // CASE 15
    //
    // Set the kernel assignment using a different body ID than
    // previous tests. This should remask the BODDEF assignment.
    //

    testutil::TCASE(b"Check precedence functionality. Re-masking.", ctx)?;

    fstr::assign(TEXT.get_mut(1), b"      NAIF_BODY_NAME = \'TestName\'");
    fstr::assign(TEXT.get_mut(2), b"      NAIF_BODY_CODE = -30000");
    spicelib::LMPOOL(TEXT.as_arg(), 2, ctx)?;

    spicelib::BODN2C(b"TestName", &mut CODE, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", CODE, b"=", -30000, 0, OK, ctx)?;

    fstr::assign(&mut GET, b" ");
    spicelib::BODC2N(-30000, &mut GET, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"GET", &GET, b"=", b"TestName", OK, ctx)?;

    //
    // CASE 16
    //
    // Test watcher logic in ZZBODN2C: if ZZBODKER fails due
    // to a body-name assignment error, ZZBODN2C should exit.
    // Make the call twice to ensure correct handling of the
    // error.
    //
    testutil::TCASE(b"ZZBODN2C: test CVPOOL handling for bad mapping ", ctx)?;

    spicelib::CLPOOL(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::BEGDAT(&mut TEXT[1]);
    fstr::assign(TEXT.get_mut(2), b" NAIF_BODY_CODE  = ( 22, 23, 24, 25 )");
    fstr::assign(
        TEXT.get_mut(3),
        b" NAIF_BODY_NAME  = ( \'SPUD\', \'MOON\',  \'CURLEY\' )",
    );

    testutil::TSTTXT(b"sample.nam", TEXT.as_arg(), 3, true, false, ctx)?;

    spicelib::ZZBODN2C(b"MOON", &mut CODE, &mut FOUND, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADDIMENSIONS)", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    spicelib::ZZBODN2C(b"MOON", &mut CODE, &mut FOUND, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADDIMENSIONS)", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    //
    // CASE 17
    //
    // Test watcher logic in ZZBODC2N: if ZZBODKER fails due
    // to a body-name assignment error, ZZBODC2N should exit.
    // Make the call twice to ensure correct handling of the
    // error.
    //
    testutil::TCASE(b"ZZBODC2N: test CVPOOL handling for bad mapping ", ctx)?;

    spicelib::CLPOOL(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::BEGDAT(&mut TEXT[1]);
    fstr::assign(TEXT.get_mut(2), b" NAIF_BODY_CODE  = ( 22, 23, 24, 25 )");
    fstr::assign(
        TEXT.get_mut(3),
        b" NAIF_BODY_NAME  = ( \'SPUD\', \'MOON\', \'CURLEY\' )",
    );

    testutil::TSTTXT(b"sample.nam", TEXT.as_arg(), 3, true, false, ctx)?;

    spicelib::ZZBODC2N(23, &mut GET, &mut FOUND, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADDIMENSIONS)", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    spicelib::ZZBODC2N(23, &mut GET, &mut FOUND, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADDIMENSIONS)", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    //
    // CASE 17
    //
    // Test watcher logic in ZZBODKIK: if ZZBODKER fails due
    // to a body-name assignment error, ZZBODKIK should exit.
    // Since ZZBODKIK doesn't make direct use of the kernel
    // pool assignments, call ZZBODC2N to ensure that it calls
    // ZZBODKER if NODATA is .TRUE. upon entry.
    //
    // Run the sequence of calls twice to ensure correct handling of the
    // error.
    //
    testutil::TCASE(b"ZZBODKIK: test CVPOOL handling for bad mapping ", ctx)?;

    spicelib::CLPOOL(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::BEGDAT(&mut TEXT[1]);
    fstr::assign(TEXT.get_mut(2), b" NAIF_BODY_CODE  = ( 22, 23, 24, 25 )");
    fstr::assign(
        TEXT.get_mut(3),
        b" NAIF_BODY_NAME  = ( \'SPUD\', \'MOON\', \'CURLEY\' )",
    );

    testutil::TSTTXT(b"sample.nam", TEXT.as_arg(), 3, true, false, ctx)?;

    spicelib::ZZBODKIK(ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADDIMENSIONS)", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    spicelib::ZZBODC2N(23, &mut GET, &mut FOUND, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADDIMENSIONS)", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    spicelib::ZZBODKIK(ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADDIMENSIONS)", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    spicelib::ZZBODC2N(23, &mut GET, &mut FOUND, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADDIMENSIONS)", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
