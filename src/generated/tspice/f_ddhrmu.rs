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

//$Procedure F_DDHRMU ( ZZDDHRMU Test Family )
pub fn F_DDHRMU(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut LUN: i32 = 0;
    let mut NFT: i32 = 0;
    let mut NUT: i32 = 0;
    let mut UINDEX: i32 = 0;
    let mut RESRVD: bool = false;
    let mut UTCST = StackArray::<i32, 23>::new(1..=UTSIZE);
    let mut UTHAN = StackArray::<i32, 23>::new(1..=UTSIZE);
    let mut UTLUN = StackArray::<i32, 23>::new(1..=UTSIZE);
    let mut UTLCK = StackArray::<bool, 23>::new(1..=UTSIZE);

    //
    // Local Variables
    //

    //
    // The unit table columns
    //

    //
    // Start the test family with an open call.
    //
    testutil::TOPEN(b"F_DDHRMU", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Empty unit table exceptional case.", ctx)?;

    //
    // Prepare the inputs and output default values.
    //
    UINDEX = 1;
    NUT = 0;
    NFT = 1;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHRMU(
        UINDEX,
        NFT,
        UTCST.as_slice_mut(),
        UTHAN.as_slice_mut(),
        UTLCK.as_slice_mut(),
        UTLUN.as_slice_mut(),
        &mut NUT,
        ctx,
    )?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"UINDEX out of range exception.", ctx)?;

    //
    // Prepare the inputs and output default values.
    //
    NUT = 10;

    for I in 1..=NUT {
        UTCST[I] = (10 * I);
        UTHAN[I] = I;
        UTLCK[I] = false;
        UTLUN[I] = I;
    }

    UINDEX = 5000;
    NFT = 12;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHRMU(
        UINDEX,
        NFT,
        UTCST.as_slice_mut(),
        UTHAN.as_slice_mut(),
        UTLCK.as_slice_mut(),
        UTLUN.as_slice_mut(),
        &mut NUT,
        ctx,
    )?;

    //
    // Check for the presence of the exception.
    //
    testutil::CHCKXC(true, b"SPICE(INDEXOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test table compression case.", ctx)?;

    //
    // Prepare the inputs and output default values.
    //
    NUT = 4;

    for I in 1..=NUT {
        UTCST[I] = (10 * I);
        UTHAN[I] = I;
        UTLCK[I] = false;
        UTLUN[I] = I;
    }

    UINDEX = 2;
    NFT = 3;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHRMU(
        UINDEX,
        NFT,
        UTCST.as_slice_mut(),
        UTHAN.as_slice_mut(),
        UTLCK.as_slice_mut(),
        UTLUN.as_slice_mut(),
        &mut NUT,
        ctx,
    )?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check outputs.
    //
    testutil::CHCKSI(b"UTCST(1)", UTCST[1], b"=", 10, 0, OK, ctx)?;
    testutil::CHCKSI(b"UTHAN(1)", UTHAN[1], b"=", 1, 0, OK, ctx)?;
    testutil::CHCKSL(b"UTLCK(1)", UTLCK[1], false, OK, ctx)?;
    testutil::CHCKSI(b"UTLUN(1)", UTLUN[1], b"=", 1, 0, OK, ctx)?;

    testutil::CHCKSI(b"UTCST(2)", UTCST[2], b"=", 30, 0, OK, ctx)?;
    testutil::CHCKSI(b"UTHAN(2)", UTHAN[2], b"=", 3, 0, OK, ctx)?;
    testutil::CHCKSL(b"UTLCK(2)", UTLCK[2], false, OK, ctx)?;
    testutil::CHCKSI(b"UTLUN(2)", UTLUN[2], b"=", 3, 0, OK, ctx)?;

    testutil::CHCKSI(b"UTCST(3)", UTCST[3], b"=", 40, 0, OK, ctx)?;
    testutil::CHCKSI(b"UTHAN(3)", UTHAN[3], b"=", 4, 0, OK, ctx)?;
    testutil::CHCKSL(b"UTLCK(3)", UTLCK[3], false, OK, ctx)?;
    testutil::CHCKSI(b"UTLUN(3)", UTLUN[3], b"=", 4, 0, OK, ctx)?;

    testutil::CHCKSI(b"NUT", NUT, b"=", 3, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Zero row test case.", ctx)?;

    //
    // Prepare the inputs and output default values.
    //
    NUT = 4;

    for I in 1..=NUT {
        UTCST[I] = (10 * I);
        UTHAN[I] = I;
        UTLCK[I] = false;
        UTLUN[I] = I;
    }

    UINDEX = 2;
    NFT = 4;

    //
    // Setup row 2 with an actual logical unit.
    //
    spicelib::GETLUN(&mut LUN, ctx)?;
    UTLUN[2] = LUN;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHRMU(
        UINDEX,
        NFT,
        UTCST.as_slice_mut(),
        UTHAN.as_slice_mut(),
        UTLCK.as_slice_mut(),
        UTLUN.as_slice_mut(),
        &mut NUT,
        ctx,
    )?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check outputs
    //
    testutil::CHCKSI(b"UTCST(1)", UTCST[1], b"=", 10, 0, OK, ctx)?;
    testutil::CHCKSI(b"UTHAN(1)", UTHAN[1], b"=", 1, 0, OK, ctx)?;
    testutil::CHCKSL(b"UTLCK(1)", UTLCK[1], false, OK, ctx)?;
    testutil::CHCKSI(b"UTLUN(1)", UTLUN[1], b"=", 1, 0, OK, ctx)?;

    testutil::CHCKSI(b"UTCST(2)", UTCST[2], b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"UTHAN(2)", UTHAN[2], b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSL(b"UTLCK(2)", UTLCK[2], false, OK, ctx)?;
    testutil::CHCKSI(b"UTLUN(2)", UTLUN[2], b"=", LUN, 0, OK, ctx)?;

    testutil::CHCKSI(b"UTCST(3)", UTCST[3], b"=", 30, 0, OK, ctx)?;
    testutil::CHCKSI(b"UTHAN(3)", UTHAN[3], b"=", 3, 0, OK, ctx)?;
    testutil::CHCKSL(b"UTLCK(3)", UTLCK[3], false, OK, ctx)?;
    testutil::CHCKSI(b"UTLUN(3)", UTLUN[3], b"=", 3, 0, OK, ctx)?;

    testutil::CHCKSI(b"UTCST(4)", UTCST[4], b"=", 40, 0, OK, ctx)?;
    testutil::CHCKSI(b"UTHAN(4)", UTHAN[4], b"=", 4, 0, OK, ctx)?;
    testutil::CHCKSL(b"UTLCK(4)", UTLCK[4], false, OK, ctx)?;
    testutil::CHCKSI(b"UTLUN(4)", UTLUN[4], b"=", 4, 0, OK, ctx)?;

    testutil::CHCKSI(b"NUT", NUT, b"=", 4, 0, OK, ctx)?;

    //
    // Check to see if LUN is reserved.
    //
    T_TSTRLN(LUN, &mut RESRVD, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the value of RESRVD.
    //
    testutil::CHCKSL(b"RESRVD", RESRVD, true, OK, ctx)?;

    //
    // Free up the logical unit.
    //
    spicelib::FRELUN(LUN, ctx);

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
