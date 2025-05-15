//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const FTSIZE: i32 = 5000;
const RSVUNT: i32 = 2;
const SCRUNT: i32 = 1;
const UTSIZE: i32 = ((20 + SCRUNT) + RSVUNT);
const READ: i32 = 1;
const WRITE: i32 = 2;
const SCRTCH: i32 = 3;
const NEW: i32 = 4;
const NUMAMH: i32 = 4;
const BIGI3E: i32 = 1;
const LTLI3E: i32 = 2;
const VAXGFL: i32 = 3;
const VAXDFL: i32 = 4;
const NUMBFF: i32 = 4;
const STRSIZ: i32 = 8;
const STRLEN: i32 = ((STRSIZ + 1) * NUMBFF);
const DAF: i32 = 1;
const DAS: i32 = 2;
const NUMARC: i32 = 2;
const RECL: i32 = 1024;
const FILEN: i32 = 255;
const CBFSIZ: i32 = 1024;

//$Procedure F_DDHRCM ( ZZDDHRCM Test Family )
pub fn F_DDHRCM(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut CKCST = StackArray::<i32, 23>::new(1..=UTSIZE);
    let mut NUT: i32 = 0;
    let mut REQCNT: i32 = 0;
    let mut UTCST = StackArray::<i32, 23>::new(1..=UTSIZE);

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Start the test family with an open call.
    //
    testutil::TOPEN(b"F_DDHRCM", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Empty table with REQCNT less than INTMAX.", ctx)?;

    //
    // Prepare the inputs and output default values.
    //
    UTCST[1] = 500;
    NUT = 0;

    REQCNT = 10000;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHRCM(NUT, UTCST.as_slice_mut(), &mut REQCNT);

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check outputs.
    //
    testutil::CHCKSI(b"REQCNT", REQCNT, b"=", 10001, 0, OK, ctx)?;

    testutil::CHCKSI(b"NUT", NUT, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Empty table with REQCNT at INTMAX.", ctx)?;

    //
    // Prepare the inputs and output default values.
    //
    UTCST[1] = 500;
    NUT = 0;

    REQCNT = spicelib::INTMAX();

    //
    // Invoke the module.
    //
    spicelib::ZZDDHRCM(NUT, UTCST.as_slice_mut(), &mut REQCNT);

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check outputs.
    //
    testutil::CHCKSI(
        b"REQCNT",
        REQCNT,
        b"=",
        ((spicelib::INTMAX() / 2) + 1),
        0,
        OK,
        ctx,
    )?;

    testutil::CHCKSI(b"NUT", NUT, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Non-empty table, REQCNT less than INTMAX.", ctx)?;

    //
    // Prepare the inputs and output default values.
    //
    NUT = 10;

    for I in 1..=NUT {
        UTCST[I] = (10000 * I);
        CKCST[I] = (10000 * I);
    }

    REQCNT = 101000000;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHRCM(NUT, UTCST.as_slice_mut(), &mut REQCNT);

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check outputs.
    //
    testutil::CHCKSI(b"REQCNT", REQCNT, b"=", 101000001, 0, OK, ctx)?;

    testutil::CHCKSI(b"UTCST(1)", UTCST[1], b"=", CKCST[1], 0, OK, ctx)?;
    testutil::CHCKSI(b"UTCST(2)", UTCST[2], b"=", CKCST[2], 0, OK, ctx)?;
    testutil::CHCKSI(b"UTCST(3)", UTCST[3], b"=", CKCST[3], 0, OK, ctx)?;
    testutil::CHCKSI(b"UTCST(4)", UTCST[4], b"=", CKCST[4], 0, OK, ctx)?;
    testutil::CHCKSI(b"UTCST(5)", UTCST[5], b"=", CKCST[5], 0, OK, ctx)?;
    testutil::CHCKSI(b"UTCST(6)", UTCST[6], b"=", CKCST[6], 0, OK, ctx)?;
    testutil::CHCKSI(b"UTCST(7)", UTCST[7], b"=", CKCST[7], 0, OK, ctx)?;
    testutil::CHCKSI(b"UTCST(8)", UTCST[8], b"=", CKCST[8], 0, OK, ctx)?;
    testutil::CHCKSI(b"UTCST(9)", UTCST[9], b"=", CKCST[9], 0, OK, ctx)?;
    testutil::CHCKSI(b"UTCST(10)", UTCST[10], b"=", CKCST[10], 0, OK, ctx)?;

    testutil::CHCKSI(b"NUT", NUT, b"=", 10, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Non-empty table, REQCNT is INTMAX.", ctx)?;

    //
    // Prepare the inputs and output default values.
    //
    NUT = 10;

    for I in 1..=NUT {
        UTCST[I] = (10000 * I);
        CKCST[I] = (5000 * I);
    }

    //
    // Set REQCNT to INTMAX and UTCST(7) to 1 to test the
    // MAX(1,...) logic.
    //
    REQCNT = spicelib::INTMAX();
    UTCST[7] = 1;
    CKCST[7] = 1;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHRCM(NUT, UTCST.as_slice_mut(), &mut REQCNT);

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check outputs.
    //
    testutil::CHCKSI(
        b"REQCNT",
        REQCNT,
        b"=",
        ((spicelib::INTMAX() / 2) + 1),
        0,
        OK,
        ctx,
    )?;

    testutil::CHCKSI(b"UTCST(1)", UTCST[1], b"=", CKCST[1], 0, OK, ctx)?;
    testutil::CHCKSI(b"UTCST(2)", UTCST[2], b"=", CKCST[2], 0, OK, ctx)?;
    testutil::CHCKSI(b"UTCST(3)", UTCST[3], b"=", CKCST[3], 0, OK, ctx)?;
    testutil::CHCKSI(b"UTCST(4)", UTCST[4], b"=", CKCST[4], 0, OK, ctx)?;
    testutil::CHCKSI(b"UTCST(5)", UTCST[5], b"=", CKCST[5], 0, OK, ctx)?;
    testutil::CHCKSI(b"UTCST(6)", UTCST[6], b"=", CKCST[6], 0, OK, ctx)?;
    testutil::CHCKSI(b"UTCST(7)", UTCST[7], b"=", CKCST[7], 0, OK, ctx)?;
    testutil::CHCKSI(b"UTCST(8)", UTCST[8], b"=", CKCST[8], 0, OK, ctx)?;
    testutil::CHCKSI(b"UTCST(9)", UTCST[9], b"=", CKCST[9], 0, OK, ctx)?;
    testutil::CHCKSI(b"UTCST(10)", UTCST[10], b"=", CKCST[10], 0, OK, ctx)?;

    testutil::CHCKSI(b"NUT", NUT, b"=", 10, 0, OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
