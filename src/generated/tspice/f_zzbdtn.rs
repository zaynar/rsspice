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
const NAMLEN: i32 = 32;
const LINLEN: i32 = 200;

//$Procedure F_ZZBDTN ( ZZBODTRN Test Family  )
pub fn F_ZZBDTN(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut BUFFER = ActualCharArray::new(LINLEN, 1..=4);
    let mut NAME = [b' '; NAMLEN as usize];
    let mut CODE: i32 = 0;
    let USRCTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
    let mut FOUND: bool = false;
    let mut UPDATE: bool = false;

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_ZZBDTN", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZBODTRN - SPICE(BOGUSENTRY) exception", ctx)?;

    spicelib::ZZBODTRN(&NAME, CODE, FOUND, USRCTR.as_slice(), UPDATE, ctx)?;

    testutil::CHCKXC(true, b"SPICE(BOGUSENTRY)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZBODRST - Coverage Test", ctx)?;

    //
    // Install a new mapping.
    //
    spicelib::ZZBODDEF(b"NAME", 1000, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check that the mapping is installed.
    //
    spicelib::ZZBODN2C(b"NAME", &mut CODE, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", CODE, b"=", 1000, 0, OK, ctx)?;

    //
    // Reset the body name-code system.
    //
    spicelib::CLPOOL(ctx)?;
    spicelib::ZZBODRST(ctx)?;

    //
    // Now verify that NAME<->1000 is not currently assigned.
    //
    spicelib::ZZBODN2C(b"NAME", &mut CODE, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    spicelib::ZZBODC2N(1000, &mut b"NAME".clone(), &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZBODTRN - Check watchers are properly set", ctx)?;

    //
    // Make certain that the initialization block is executed
    // prior to attempting to check the watcher's set status.
    // It is difficult to verify that each initialization block
    // functions properly in a test family, since once INIT is
    // set the code will not execute again.
    //
    spicelib::ZZBODN2C(b"EARTH", &mut CODE, &mut FOUND, ctx)?;

    fstr::assign(BUFFER.get_mut(1), b"NAIF_BODY_CODE = 3000");
    fstr::assign(
        BUFFER.get_mut(2),
        b"NAIF_BODY_NAME = \'ZZBODTRN_TEST_BODY\'",
    );

    spicelib::LMPOOL(BUFFER.as_arg(), 2, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check to see that the watchers are activated.
    //
    spicelib::CVPOOL(b"ZZBODTRN", &mut UPDATE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"UPDATE", UPDATE, true, OK, ctx)?;

    //
    // Clean up.
    //
    spicelib::CLPOOL(ctx)?;
    spicelib::ZZBODRST(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZBODN2C - Coverage Test - UPDATE", ctx)?;

    //
    // Cause CVPOOL to set UPDATE to .TRUE.
    //
    fstr::assign(BUFFER.get_mut(1), b"NAIF_BODY_CODE = 3000");
    fstr::assign(
        BUFFER.get_mut(2),
        b"NAIF_BODY_NAME = \'ZZBODTRN_TEST_BODY_2\'",
    );

    spicelib::LMPOOL(BUFFER.as_arg(), 2, ctx)?;

    CODE = 0;
    FOUND = true;

    //
    // Now invoke the module with an unknown name.
    //
    spicelib::ZZBODN2C(b"<UNDEFINED>", &mut CODE, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSI(b"CODE", CODE, b"=", 0, 0, OK, ctx)?;

    //
    // Verify that ZZBODTRN_TEST_BODY_2 is available now. EXTKER
    // success path.
    //
    spicelib::LMPOOL(BUFFER.as_arg(), 2, ctx)?;

    spicelib::ZZBODN2C(b"ZZBODTRN_TEST_BODY_2", &mut CODE, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", CODE, b"=", 3000, 0, OK, ctx)?;

    //
    // Check a built-in assignment, EXTKER failure path.  Make LJUST
    // UCASE, and COMPRSS perform non-trivial operations.
    //
    spicelib::LMPOOL(BUFFER.as_arg(), 2, ctx)?;

    spicelib::ZZBODN2C(b"    earth    barycenter", &mut CODE, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", CODE, b"=", 3, 0, OK, ctx)?;

    //
    // Set UPDATE and force EXTKER to be .FALSE., look for unknown name.
    //
    spicelib::CLPOOL(ctx)?;

    CODE = 0;
    FOUND = true;

    spicelib::ZZBODN2C(b"<UNDEFINED>", &mut CODE, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSI(b"CODE", CODE, b"=", 0, 0, OK, ctx)?;

    //
    // Check to see that ZZBODTRN_TEST_BODY_2 is no longer available.
    //
    spicelib::CLPOOL(ctx)?;

    CODE = 0;
    FOUND = true;

    spicelib::ZZBODN2C(b"ZZBODTRN_TEST_BODY_2", &mut CODE, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSI(b"CODE", CODE, b"=", 0, 0, OK, ctx)?;

    //
    // Lastly look up a built-in name's code.
    //
    spicelib::CLPOOL(ctx)?;

    spicelib::ZZBODN2C(b"EARTH", &mut CODE, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", CODE, b"=", 399, 0, OK, ctx)?;

    spicelib::ZZBODRST(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZBODN2C - Coverage Test - no UPDATE", ctx)?;

    //
    // Force EXTKER to be set.
    //
    fstr::assign(BUFFER.get_mut(1), b"NAIF_BODY_CODE = 3000");
    fstr::assign(
        BUFFER.get_mut(2),
        b"NAIF_BODY_NAME = \'ZZBODTRN_TEST_BODY_3\'",
    );

    spicelib::LMPOOL(BUFFER.as_arg(), 2, ctx)?;

    //
    // Make sure no watchers set UPDATE.
    //
    CODE = 0;

    spicelib::ZZBODN2C(b"<UNDEFINED>", &mut CODE, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSI(b"CODE", CODE, b"=", 0, 0, OK, ctx)?;

    //
    // Now invoke ZZBODN2C with the name defined in the kernel pool.
    //
    spicelib::ZZBODN2C(b"ZZBODTRN_TEST_BODY_3", &mut CODE, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", CODE, b"=", 3000, 0, OK, ctx)?;

    //
    // Now invoke ZZBODN2C with a built-in name.
    //
    spicelib::ZZBODN2C(b"EARTH", &mut CODE, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", CODE, b"=", 399, 0, OK, ctx)?;

    //
    // Lastly invoke it with an unknown name.
    //
    CODE = 0;

    spicelib::ZZBODN2C(b"<UNDEFINED>", &mut CODE, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSI(b"CODE", CODE, b"=", 0, 0, OK, ctx)?;

    //
    // Clear the pool and remove EXTKER's .TRUE. status.
    //
    spicelib::CLPOOL(ctx)?;

    CODE = 0;

    spicelib::ZZBODN2C(b"<UNDEFINED>", &mut CODE, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSI(b"CODE", CODE, b"=", 0, 0, OK, ctx)?;

    //
    // Now check the previously defined name in the kernel pool.
    //
    CODE = 0;

    spicelib::ZZBODN2C(b"ZZBODTRN_TEST_BODY_3", &mut CODE, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSI(b"CODE", CODE, b"=", 0, 0, OK, ctx)?;

    //
    // Check a built-in code.
    //
    spicelib::ZZBODN2C(b"EARTH", &mut CODE, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", CODE, b"=", 399, 0, OK, ctx)?;

    //
    // Lastly invoke it with an undefined name.
    //
    CODE = 0;

    spicelib::ZZBODN2C(b"<UNDEFINED>", &mut CODE, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSI(b"CODE", CODE, b"=", 0, 0, OK, ctx)?;

    spicelib::ZZBODRST(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZBODC2N - Coverage Test - UPDATE", ctx)?;

    //
    // Cause CVPOOL to set UPDATE to .TRUE.
    //
    fstr::assign(BUFFER.get_mut(1), b"NAIF_BODY_CODE = 3000");
    fstr::assign(
        BUFFER.get_mut(2),
        b"NAIF_BODY_NAME = \'ZZBODTRN_TEST_BODY_4\'",
    );
    fstr::assign(BUFFER.get_mut(3), b"NAIF_BODY_CODE += 3002");
    fstr::assign(BUFFER.get_mut(4), b"NAIF_BODY_NAME += \'EARTH\'");

    spicelib::LMPOOL(BUFFER.as_arg(), 4, ctx)?;

    fstr::assign(&mut NAME, b"<UNCHANGED>");
    FOUND = true;

    //
    // Now invoke the module with an unknown code.
    //
    spicelib::ZZBODC2N(3001, &mut NAME, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &NAME, b"=", b"<UNCHANGED>", OK, ctx)?;

    //
    // Verify that 3001 is available now. EXTKER success path.
    //
    spicelib::LMPOOL(BUFFER.as_arg(), 4, ctx)?;

    spicelib::ZZBODC2N(3000, &mut NAME, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &NAME, b"=", b"ZZBODTRN_TEST_BODY_4", OK, ctx)?;

    //
    // Check a built-in assignment, EXTKER failure path no "kernel
    // pool" masking.
    //
    spicelib::LMPOOL(BUFFER.as_arg(), 4, ctx)?;

    spicelib::ZZBODC2N(3, &mut NAME, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &NAME, b"=", b"EARTH BARYCENTER", OK, ctx)?;

    //
    // Check a built-in assignment, EXTKER failure path with "kernel
    // pool" masking.
    //
    spicelib::LMPOOL(BUFFER.as_arg(), 4, ctx)?;

    fstr::assign(&mut NAME, b"<UNCHANGED>");

    spicelib::ZZBODC2N(399, &mut NAME, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &NAME, b"=", b"<UNCHANGED>", OK, ctx)?;

    //
    // Set UPDATE and force EXTKER to be .FALSE., look for unknown code.
    //
    spicelib::CLPOOL(ctx)?;

    fstr::assign(&mut NAME, b"<UNCHANGED>");
    FOUND = true;

    spicelib::ZZBODC2N(3001, &mut NAME, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &NAME, b"=", b"<UNCHANGED>", OK, ctx)?;

    //
    // Check to see that 3000 is no longer available.
    //
    spicelib::CLPOOL(ctx)?;

    fstr::assign(&mut NAME, b"<UNCHANGED>");
    FOUND = true;

    spicelib::ZZBODC2N(3000, &mut NAME, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &NAME, b"=", b"<UNCHANGED>", OK, ctx)?;

    //
    // Lastly look up a built-in code's name.
    //
    spicelib::CLPOOL(ctx)?;

    spicelib::ZZBODC2N(399, &mut NAME, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &NAME, b"=", b"EARTH", OK, ctx)?;

    spicelib::ZZBODRST(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZBODC2N - Coverage Test - no UPDATE", ctx)?;

    //
    // Force EXTKER to be set.
    //
    fstr::assign(BUFFER.get_mut(1), b"NAIF_BODY_CODE = 3000");
    fstr::assign(
        BUFFER.get_mut(2),
        b"NAIF_BODY_NAME = \'ZZBODTRN_TEST_BODY_5\'",
    );
    fstr::assign(BUFFER.get_mut(3), b"NAIF_BODY_CODE += 3002");
    fstr::assign(BUFFER.get_mut(4), b"NAIF_BODY_NAME += \'EARTH\'");

    spicelib::LMPOOL(BUFFER.as_arg(), 4, ctx)?;

    //
    // Make sure no watchers set UPDATE.
    //
    fstr::assign(&mut NAME, b"<UNCHANGED>");

    spicelib::ZZBODC2N(3001, &mut NAME, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &NAME, b"=", b"<UNCHANGED>", OK, ctx)?;

    //
    // Now invoke ZZBODN2C with a code defined in the kernel pool.
    //
    spicelib::ZZBODC2N(3000, &mut NAME, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &NAME, b"=", b"ZZBODTRN_TEST_BODY_5", OK, ctx)?;

    //
    // Check a built-in code, EXTKER is .TRUE., with no "kernel pool"
    // masking.
    //
    spicelib::ZZBODC2N(3, &mut NAME, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &NAME, b"=", b"EARTH BARYCENTER", OK, ctx)?;

    //
    // Check a built-in code, EXTER is .TRUE., with "kernel pool"
    // masking.
    //
    fstr::assign(&mut NAME, b"<UNCHANGED>");

    spicelib::ZZBODC2N(399, &mut NAME, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &NAME, b"=", b"<UNCHANGED>", OK, ctx)?;

    //
    // Check for an unknown code.
    //
    fstr::assign(&mut NAME, b"<UNCHANGED>");

    spicelib::ZZBODC2N(3001, &mut NAME, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &NAME, b"=", b"<UNCHANGED>", OK, ctx)?;

    //
    // Now clear the pool to remove EXTKER's .TRUE. status.
    //
    spicelib::CLPOOL(ctx)?;

    fstr::assign(&mut NAME, b"<UNCHANGED>");

    //
    // Look up an unknown code, to clear UPDATE's status.
    //
    spicelib::ZZBODC2N(3001, &mut NAME, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &NAME, b"=", b"<UNCHANGED>", OK, ctx)?;

    //
    // Now check the previously defined codes in the kernel pool.
    //
    fstr::assign(&mut NAME, b"<UNCHANGED>");

    spicelib::ZZBODC2N(3000, &mut NAME, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &NAME, b"=", b"<UNCHANGED>", OK, ctx)?;

    fstr::assign(&mut NAME, b"<UNCHANGED>");

    spicelib::ZZBODC2N(3002, &mut NAME, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &NAME, b"=", b"<UNCHANGED>", OK, ctx)?;

    //
    // Now look up the built-in codes.
    //
    spicelib::ZZBODC2N(399, &mut NAME, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &NAME, b"=", b"EARTH", OK, ctx)?;

    spicelib::ZZBODC2N(3, &mut NAME, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &NAME, b"=", b"EARTH BARYCENTER", OK, ctx)?;

    //
    // Lastly invoke it with an unknown code.
    //
    fstr::assign(&mut NAME, b"<UNCHANGED>");

    spicelib::ZZBODC2N(3001, &mut NAME, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &NAME, b"=", b"<UNCHANGED>", OK, ctx)?;

    spicelib::ZZBODRST(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZBODDEF - Exception SPICE(BLANKNAMEASSIGNED)", ctx)?;

    spicelib::ZZBODDEF(b" ", 1000, ctx)?;

    testutil::CHCKXC(true, b"SPICE(BLANKNAMEASSIGNED)", OK, ctx)?;

    //
    // Check that it did nothing to the name-code mapping
    // system.
    //
    spicelib::ZZBODN2C(b" ", &mut CODE, &mut FOUND, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    spicelib::ZZBODC2N(1000, &mut NAME, &mut FOUND, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    //
    // Clean up.
    //
    spicelib::ZZBODRST(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZBODDEF - Coverage Tests - Name Replace Case", ctx)?;

    //
    // Replace the name for EARTH with the same code.  This forces a
    // simple replace, rather than a sort/update.
    //
    spicelib::ZZBODDEF(b"eArTh", 399, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // See if it worked.
    //
    spicelib::ZZBODC2N(399, &mut NAME, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &NAME, b"=", b"eArTh", OK, ctx)?;

    spicelib::ZZBODN2C(b"eArTh", &mut CODE, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", CODE, b"=", 399, 0, OK, ctx)?;

    spicelib::CLPOOL(ctx)?;
    spicelib::ZZBODRST(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZBODDEF - Coverage Tests - Name Replace Sort", ctx)?;

    //
    // Replace the name for EARTH with a new code.  This forces a
    // compression and append operation.
    //
    spicelib::ZZBODDEF(b"EaRtH", 1000, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // See if it worked.
    //
    spicelib::ZZBODC2N(1000, &mut NAME, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &NAME, b"=", b"EaRtH", OK, ctx)?;

    spicelib::ZZBODN2C(b"EaRtH", &mut CODE, OK, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", CODE, b"=", 1000, 0, OK, ctx)?;

    spicelib::CLPOOL(ctx)?;
    spicelib::ZZBODRST(ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZBODDEF - Coverage Tests - Append New", ctx)?;

    //
    // Replace the name for EARTH with a new code.  This forces a
    // compression and append operation.
    //
    spicelib::ZZBODDEF(b"SPUDSpam", 1000, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // See if it worked.
    //
    spicelib::ZZBODC2N(1000, &mut NAME, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &NAME, b"=", b"SPUDSpam", OK, ctx)?;

    spicelib::ZZBODN2C(b"spudspam", &mut CODE, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"CODE", CODE, b"=", 1000, 0, OK, ctx)?;

    spicelib::CLPOOL(ctx)?;
    spicelib::ZZBODRST(ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
