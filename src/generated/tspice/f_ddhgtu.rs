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

//$Procedure F_DDHGTU ( ZZDDHGTU Test Family )
pub fn F_DDHGTU(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut IOSTAT: i32 = 0;
    let mut LUN: i32 = 0;
    let mut UINDEX: i32 = 0;
    let mut OPENED: bool = false;
    let mut RESRVD: bool = false;
    let mut NUT: i32 = 0;
    let mut UTCST = StackArray::<i32, 23>::new(1..=UTSIZE);
    let mut UTHAN = StackArray::<i32, 23>::new(1..=UTSIZE);
    let mut UTLUN = StackArray::<i32, 23>::new(1..=UTSIZE);
    let mut UTLCK = StackArray::<bool, 23>::new(1..=UTSIZE);

    //
    // Local Variables
    //

    //
    // Unit Table
    //

    //
    // Start the test family with an open call.
    //
    testutil::TOPEN(b"F_DDHGTU", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Empty unit table insertion.", ctx)?;

    //
    // Prepare the inputs and output default values.
    //
    UTCST[1] = 1;
    UTHAN[1] = 1;
    UTLCK[1] = true;
    UTLUN[1] = 1;

    NUT = 0;

    UINDEX = -1;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHGTU(
        UTCST.as_slice_mut(),
        UTHAN.as_slice_mut(),
        UTLCK.as_slice_mut(),
        UTLUN.as_slice_mut(),
        &mut NUT,
        &mut UINDEX,
        ctx,
    )?;

    //
    // Check for an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now check the values.
    //
    testutil::CHCKSI(b"UTCST(1)", UTCST[1], b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"UTHAN(1)", UTHAN[1], b"=", 0, 0, OK, ctx)?;

    testutil::CHCKSL(b"UTLCK(1)", UTLCK[1], false, OK, ctx)?;

    testutil::CHCKSI(b"UTLUN(1)", UTLUN[1], b">=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"NUT", NUT, b"=", 1, 0, OK, ctx)?;

    testutil::CHCKSI(b"UINDEX", UINDEX, b"=", 1, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Zero cost row exists in the unit table", ctx)?;

    //
    // Prepare the inputs and output default values.
    //
    NUT = 10;

    for I in 1..=10 {
        UTCST[I] = (10 * I);
        UTHAN[I] = I;
        UTLCK[I] = false;
        UTLUN[I] = I;
    }

    //
    // Create the zero cost row.
    //
    UTCST[5] = 0;

    //
    // Actually get a logical unit from GETLUN and lock it down.
    //
    spicelib::GETLUN(&mut LUN, ctx)?;
    spicelib::RESLUN(LUN, ctx);
    UTLUN[5] = LUN;

    UINDEX = -1;

    //
    // Exercise the module. (Note: It will 'FRELUN' UTLUN(5) )
    //
    spicelib::ZZDDHGTU(
        UTCST.as_slice_mut(),
        UTHAN.as_slice_mut(),
        UTLCK.as_slice_mut(),
        UTLUN.as_slice_mut(),
        &mut NUT,
        &mut UINDEX,
        ctx,
    )?;

    //
    // Check for an exception, one should not have been signaled.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now check the relevant values.
    //
    testutil::CHCKSI(b"UINDEX", UINDEX, b"=", 5, 0, OK, ctx)?;

    testutil::CHCKSI(b"UTCST(UINDEX)", UTCST[UINDEX], b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"UTHAN(UINDEX)", UTHAN[UINDEX], b"=", 5, 0, OK, ctx)?;

    testutil::CHCKSL(b"UTLCK(UINDEX)", UTLCK[UINDEX], false, OK, ctx)?;

    testutil::CHCKSI(b"NUT", NUT, b"=", 10, 0, OK, ctx)?;

    //
    // Check to see if LUN is still reserved with RESLUN.
    //
    T_TSTRLN(LUN, &mut RESRVD, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the value of RESRVD.
    //
    testutil::CHCKSL(b"RESRVD", RESRVD, false, OK, ctx)?;

    //
    // Just to be safe, free LUN.
    //
    spicelib::FRELUN(LUN, ctx);

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No zero cost rows, expand table case.", ctx)?;

    //
    // Prepare the inputs and output default values.
    //
    NUT = 10;

    for I in 1..=10 {
        UTCST[I] = (10 * I);
        UTHAN[I] = I;
        UTLCK[I] = false;
        UTLUN[I] = I;
    }

    UINDEX = -1;

    //
    // Prepare the initial values for the row that will be returned.
    //
    UTCST[11] = -1;
    UTHAN[11] = -1;
    UTLCK[11] = true;
    UTLUN[11] = -1;

    //
    // Exercise the module.
    //
    spicelib::ZZDDHGTU(
        UTCST.as_slice_mut(),
        UTHAN.as_slice_mut(),
        UTLCK.as_slice_mut(),
        UTLUN.as_slice_mut(),
        &mut NUT,
        &mut UINDEX,
        ctx,
    )?;

    //
    // Check for an exception, one should not have been signaled.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now check the relevant values.
    //
    testutil::CHCKSI(b"UINDEX", UINDEX, b"=", 11, 0, OK, ctx)?;

    testutil::CHCKSI(b"UTCST(UINDEX)", UTCST[UINDEX], b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"UTHAN(UINDEX)", UTHAN[UINDEX], b"=", 0, 0, OK, ctx)?;

    testutil::CHCKSL(b"UTLCK(UINDEX)", UTLCK[UINDEX], false, OK, ctx)?;

    testutil::CHCKSI(b"UTLUN(UINDEX)", UTLUN[UINDEX], b">=", 0, 0, OK, ctx)?;

    testutil::CHCKSI(b"NUT", NUT, b"=", 11, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Full table, no zero-cost rows.", ctx)?;

    //
    // Prepare the inputs and output default values.
    //
    NUT = UTSIZE;

    for I in 1..=UTSIZE {
        UTCST[I] = (10 * I);
        UTHAN[I] = I;
        UTLCK[I] = false;
        UTLUN[I] = I;
    }

    UINDEX = -1;

    //
    // Lock a few low cost rows to exercise that logic.
    //
    UTLCK[1] = true;
    UTLCK[2] = true;
    UTLCK[3] = true;

    //
    // Since the fourth row will be selected by ZZDDHGTU, retrieve a
    // logical unit for it.
    //
    spicelib::GETLUN(&mut LUN, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Store this unit in the LUN column of the unit table.
    //
    UTLUN[4] = LUN;

    //
    // Now open a scratch file to connect to the unit.
    //
    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(LUN),
            access: Some(b"DIRECT"),
            recl: Some(RECL),
            status: Some(b"SCRATCH"),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
    }

    testutil::CHCKSI(b"IOSTAT", IOSTAT, b"=", 0, 0, OK, ctx)?;

    //
    // Exercise the module.
    //
    spicelib::ZZDDHGTU(
        UTCST.as_slice_mut(),
        UTHAN.as_slice_mut(),
        UTLCK.as_slice_mut(),
        UTLUN.as_slice_mut(),
        &mut NUT,
        &mut UINDEX,
        ctx,
    )?;

    //
    // Check for an exception, one should not have been signaled.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now check the relevant values.
    //
    testutil::CHCKSI(b"UINDEX", UINDEX, b"=", 4, 0, OK, ctx)?;

    //
    // Determine if ZZDDHGTU properly closed the unit.
    //
    {
        use f2rust_std::io;

        let specs = io::InquireSpecs {
            unit: Some(LUN),
            opened: Some(&mut OPENED),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.inquire(specs))?;
    }

    testutil::CHCKSI(b"IOSTAT", IOSTAT, b"=", 0, 0, OK, ctx)?;

    testutil::CHCKSL(b"OPENED", OPENED, false, OK, ctx)?;

    testutil::CHCKSI(b"UTCST(UINDEX)", UTCST[UINDEX], b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"UTHAN(UINDEX)", UTHAN[UINDEX], b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"UTLUN(UINDEX)", UTLUN[UINDEX], b"=", LUN, 0, OK, ctx)?;

    testutil::CHCKSL(b"UTLCK(UINDEX)", UTLCK[UINDEX], false, OK, ctx)?;

    testutil::CHCKSI(b"NUT", NUT, b"=", UTSIZE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Full table, monotonic decreasing cost, no zero.", ctx)?;

    //
    // Prepare the inputs and output default values.
    //
    NUT = UTSIZE;

    for I in 1..=UTSIZE {
        UTCST[I] = (((UTSIZE + 1) - I) * 10);
        UTHAN[I] = I;
        UTLCK[I] = false;
        UTLUN[I] = I;
    }

    UINDEX = -1;

    //
    // Lock a few low cost rows to exercise that logic.
    //
    UTLCK[(UTSIZE - 2)] = true;
    UTLCK[(UTSIZE - 1)] = true;
    UTLCK[UTSIZE] = true;

    //
    // Since the fourth row will be selected by ZZDDHGTU, retrieve a
    // logical unit for it and open a scratch file.
    //
    spicelib::GETLUN(&mut LUN, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    UTLUN[(UTSIZE - 3)] = LUN;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(LUN),
            access: Some(b"DIRECT"),
            recl: Some(RECL),
            status: Some(b"SCRATCH"),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
    }

    testutil::CHCKSI(b"IOSTAT", IOSTAT, b"=", 0, 0, OK, ctx)?;

    //
    // Exercise the module.
    //
    spicelib::ZZDDHGTU(
        UTCST.as_slice_mut(),
        UTHAN.as_slice_mut(),
        UTLCK.as_slice_mut(),
        UTLUN.as_slice_mut(),
        &mut NUT,
        &mut UINDEX,
        ctx,
    )?;

    //
    // Check for an exception, one should not have been signaled.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now check the relevant values.
    //
    testutil::CHCKSI(b"UINDEX", UINDEX, b"=", (UTSIZE - 3), 0, OK, ctx)?;

    //
    // Determine if ZZDDHGTU properly closed the unit.
    //
    {
        use f2rust_std::io;

        let specs = io::InquireSpecs {
            unit: Some(LUN),
            opened: Some(&mut OPENED),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.inquire(specs))?;
    }

    testutil::CHCKSI(b"IOSTAT", IOSTAT, b"=", 0, 0, OK, ctx)?;

    testutil::CHCKSL(b"OPENED", OPENED, false, OK, ctx)?;

    testutil::CHCKSI(b"UTCST(UINDEX)", UTCST[UINDEX], b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"UTHAN(UINDEX)", UTHAN[UINDEX], b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"UTLUN(UINDEX)", UTLUN[UINDEX], b"=", LUN, 0, OK, ctx)?;

    testutil::CHCKSL(b"UTLCK(UINDEX)", UTLCK[UINDEX], false, OK, ctx)?;

    testutil::CHCKSI(b"NUT", NUT, b"=", UTSIZE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Full table, all rows locked exception.", ctx)?;

    //
    // Prepare the inputs and output default values.
    //
    NUT = UTSIZE;

    for I in 1..=UTSIZE {
        UTCST[I] = (10 * I);
        UTHAN[I] = I;
        UTLCK[I] = true;
        UTLUN[I] = I;
    }

    UINDEX = -1;

    //
    // Exercise the module.
    //
    spicelib::ZZDDHGTU(
        UTCST.as_slice_mut(),
        UTHAN.as_slice_mut(),
        UTLCK.as_slice_mut(),
        UTLUN.as_slice_mut(),
        &mut NUT,
        &mut UINDEX,
        ctx,
    )?;

    //
    // Check for an exception, SPICE(BUG) should have been signaled.
    //
    testutil::CHCKXC(true, b"SPICE(BUG)", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
