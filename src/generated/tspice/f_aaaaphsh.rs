//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure F_AAAAPHSH ( Family of tests for ZZPHSH )
pub fn F_AAAAPHSH(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut M2: i32 = 0;
    let mut MAXVAR: i32 = 0;
    let mut TMPINT: i32 = 0;
    let mut FOUND: bool = false;

    //
    // Test Utility Functions
    //

    //
    // SPICELIB Functions
    //

    //
    // Local parameters
    //

    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_AAAAPHSH", ctx)?;

    //
    // NO-OP: calling ZZPHSH.
    //
    testutil::TCASE(b"Call ZZPHSH umbrella directly.", ctx)?;

    TMPINT = spicelib::ZZPHSH(b" ", 1, 1, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"ZZPHSH", TMPINT, b"=", 0, 0, OK, ctx)?;

    TMPINT = spicelib::ZZPHSH(b"BOO", 11, 11, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"ZZPHSH", TMPINT, b"=", 0, 0, OK, ctx)?;

    //
    // The test below cannot be done because CSTART called by TOPEN
    // already initialized the POOL via CLPOOL that called ZZSHSH.
    //
    // ERROR: calling ZZHASH prior to ZZSHSH, try 1.
    //
    //  CALL TCASE ( 'Call ZZHASH prior to ZZSHSH, try 1.' )
    //
    //  TMPINT = ZZHASH ( ' ' )
    //  CALL CHCKXC ( .TRUE., 'SPICE(CALLEDOUTOFORDER)', OK )
    //  CALL CHCKSI ( 'ZZHASH', TMPINT, '=', 0, 0, OK )

    //
    // ERROR: calling ZZSHSH with bad divisors.
    //
    testutil::TCASE(b"Call ZZSHSH with invalid divisors", ctx)?;

    TMPINT = spicelib::ZZSHSH(-1, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDDIVISOR)", OK, ctx)?;
    testutil::CHCKSI(b"ZZSHSH", TMPINT, b"=", 0, 0, OK, ctx)?;

    TMPINT = spicelib::ZZSHSH(0, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDDIVISOR)", OK, ctx)?;
    testutil::CHCKSI(b"ZZSHSH", TMPINT, b"=", 0, 0, OK, ctx)?;

    TMPINT = spicelib::ZZSHSH(spicelib::INTMAX(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDDIVISOR)", OK, ctx)?;
    testutil::CHCKSI(b"ZZSHSH", TMPINT, b"=", 0, 0, OK, ctx)?;

    TMPINT = spicelib::ZZSHSH((spicelib::INTMAX() / 68), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDDIVISOR)", OK, ctx)?;
    testutil::CHCKSI(b"ZZSHSH", TMPINT, b"=", 0, 0, OK, ctx)?;

    //
    // The test below cannot be done because CSTART called by TOPEN
    // already initialized the POOL via CLPOOL that called ZZSHSH.
    //
    // ERROR: calling ZZHASH prior to ZZSHSH, try 2. Although
    // ZZSHSH was already called, it never got to save the
    // POOL hash divisor.
    //
    //  CALL TCASE ( 'Call ZZHASH prior to ZZSHSH, try 2.' )
    //
    //  TMPINT = ZZHASH ( ' ' )
    //  CALL CHCKXC ( .TRUE., 'SPICE(CALLEDOUTOFORDER)', OK )
    //  CALL CHCKSI ( 'ZZHASH', TMPINT, '=', 0, 0, OK )

    //
    // ERROR: calling ZZHASH2 with bad divisors.
    //
    testutil::TCASE(b"Call ZZHASH2 with invalid divisors", ctx)?;

    TMPINT = spicelib::ZZHASH2(b" ", -1, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDDIVISOR)", OK, ctx)?;
    testutil::CHCKSI(b"ZZHASH2", TMPINT, b"=", 0, 0, OK, ctx)?;

    TMPINT = spicelib::ZZHASH2(b" ", 0, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDDIVISOR)", OK, ctx)?;
    testutil::CHCKSI(b"ZZHASH2", TMPINT, b"=", 0, 0, OK, ctx)?;

    TMPINT = spicelib::ZZHASH2(b" ", spicelib::INTMAX(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDDIVISOR)", OK, ctx)?;
    testutil::CHCKSI(b"ZZHASH2", TMPINT, b"=", 0, 0, OK, ctx)?;

    TMPINT = spicelib::ZZHASH2(b" ", (spicelib::INTMAX() / 68), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDDIVISOR)", OK, ctx)?;
    testutil::CHCKSI(b"ZZHASH2", TMPINT, b"=", 0, 0, OK, ctx)?;

    //
    // The test below cannot be done because CSTART called by TOPEN
    // already initialized the POOL via CLPOOL that called ZZSHSH.
    //
    // ERROR: calling ZZHASH prior to ZZSHSH, try 3.
    //
    //  CALL TCASE ( 'Call ZZHASH prior to ZZSHSH, try 3.' )
    //
    //  TMPINT = ZZHASH ( ' ' )
    //  CALL CHCKXC ( .TRUE., 'SPICE(CALLEDOUTOFORDER)', OK )
    //  CALL CHCKSI ( 'ZZHASH', TMPINT, '=', 0, 0, OK )

    //
    // GOOD: check a few ZZHASH2 values.
    //
    testutil::TCASE(b"Call ZZHASH2 with a few good values.", ctx)?;

    M2 = 1;

    TMPINT = spicelib::ZZHASH2(b" ", M2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"ZZHASH2", TMPINT, b"=", 1, 0, OK, ctx)?;

    TMPINT = spicelib::ZZHASH2(b"Hakuna-Matata!", M2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"ZZHASH2", TMPINT, b"=", 1, 0, OK, ctx)?;

    M2 = 5003;

    TMPINT = spicelib::ZZHASH2(b"A", M2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"ZZHASH2", TMPINT, b"=", 749, 0, OK, ctx)?;

    TMPINT = spicelib::ZZHASH2(b"A A", M2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"ZZHASH2", TMPINT, b"=", 749, 0, OK, ctx)?;

    TMPINT = spicelib::ZZHASH2(b"a A", M2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"ZZHASH2", TMPINT, b"=", 749, 0, OK, ctx)?;

    TMPINT = spicelib::ZZHASH2(b"AA", M2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"ZZHASH2", TMPINT, b"=", 1583, 0, OK, ctx)?;

    TMPINT = spicelib::ZZHASH2(b"AA AA", M2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"ZZHASH2", TMPINT, b"=", 1583, 0, OK, ctx)?;

    TMPINT = spicelib::ZZHASH2(b"aa AA", M2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"ZZHASH2", TMPINT, b"=", 1583, 0, OK, ctx)?;

    //
    // The test below cannot be done because CSTART called by TOPEN
    // already initialized the POOL via CLPOOL that called ZZSHSH.
    //
    // ERROR: calling ZZHASH prior to ZZSHSH, try 4. Although
    // ZZHASH2 initialized character-code table, the POOL
    // hash divisor is still uninitialized.
    //
    //  CALL TCASE ( 'Call ZZHASH prior to ZZSHSH, try 4.' )
    //
    //  TMPINT = ZZHASH ( ' ' )
    //  CALL CHCKXC ( .TRUE., 'SPICE(CALLEDOUTOFORDER)', OK )
    //  CALL CHCKSI ( 'ZZHASH', TMPINT, '=', 0, 0, OK )

    //
    // GOOD: finally call ZZSHSH. Use MAXVAR of the POOL for
    // consistency.
    //
    testutil::TCASE(b"Call ZZSHSH with a good divisor.", ctx)?;

    spicelib::SZPOOL(b"MAXVAR", &mut MAXVAR, &mut FOUND, ctx)?;
    testutil::CHCKSL(b"POOL MAXVAR FOUND", FOUND, true, OK, ctx)?;

    TMPINT = spicelib::ZZSHSH(MAXVAR, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"ZZSHSH", TMPINT, b"=", 0, 0, OK, ctx)?;

    //
    // GOOD: check a few ZZHASH values.
    //
    testutil::TCASE(b"Call ZZHASH with a few good values.", ctx)?;

    TMPINT = spicelib::ZZHASH(b"A", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"ZZHASH", TMPINT, b"=", 749, 0, OK, ctx)?;

    TMPINT = spicelib::ZZHASH(b"A A", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"ZZHASH", TMPINT, b"=", 749, 0, OK, ctx)?;

    TMPINT = spicelib::ZZHASH(b"a A", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"ZZHASH", TMPINT, b"=", 749, 0, OK, ctx)?;

    //
    // This is good enough for now.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
