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

//$Procedure F_DDHCLS ( ZZDDHCLS Test Family )
pub fn F_DDHCLS(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ALTARC = [b' '; 20 as usize];
    let mut ALTFNM = [b' '; FILEN as usize];
    let mut ARCH = [b' '; 20 as usize];
    let mut FNAME = [b' '; FILEN as usize];
    let mut FNMPAT = [b' '; FILEN as usize];
    let mut METHOD = [b' '; 20 as usize];
    let mut TMPFNM = [b' '; FILEN as usize];
    let mut ALTUNT: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut HANLST = StackArray::<i32, 23>::new(1..=UTSIZE);
    let mut INTAMH: i32 = 0;
    let mut INTARC: i32 = 0;
    let mut INTBFF: i32 = 0;
    let mut UNIT: i32 = 0;
    let mut FOUND: bool = false;

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Start the test family with an open call.
    //
    testutil::TOPEN(b"F_DDHCLS", ctx)?;

    //
    // Set the filename pattern.
    //
    fstr::assign(&mut FNMPAT, b"test#.fil");

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Handle not found NO-OP exception.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    HANDLE = 0;
    fstr::assign(&mut ARCH, b"DAF");

    //
    // Invoke the module.
    //
    spicelib::ZZDDHCLS(HANDLE, &ARCH, false, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Alternate-signed handle NO-OP exception.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"NEW");
    fstr::assign(&mut ARCH, b"DAF");
    HANDLE = 0;

    //
    // Kill FNAME.
    //
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // Open a new file to close.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHCLS(HANDLE, &ARCH, false, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // See if the file is closed.
    //
    spicelib::ZZDDHNFO(
        HANDLE,
        &mut ALTFNM,
        &mut INTARC,
        &mut INTBFF,
        &mut INTAMH,
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    //
    // Clean up.
    //
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Architecture mismatch Exception.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"NEW");
    fstr::assign(&mut ARCH, b"DAF");
    HANDLE = 0;

    fstr::assign(&mut ALTARC, b"DAS");

    //
    // Kill FNAME.
    //
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // Open a new file to close.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHCLS(HANDLE, &ALTARC, false, ctx)?;

    //
    // Check for the presence of an exception.
    //
    testutil::CHCKXC(true, b"SPICE(FILARCMISMATCH)", OK, ctx)?;

    //
    // See if the file is closed, it should not be.
    //
    spicelib::ZZDDHNFO(
        HANDLE,
        &mut ALTFNM,
        &mut INTARC,
        &mut INTBFF,
        &mut INTAMH,
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    testutil::CHCKSI(b"INTARC", INTARC, b"=", DAF, 0, OK, ctx)?;
    testutil::CHCKSI(b"INTAMH", INTAMH, b"=", NEW, 0, OK, ctx)?;

    //
    // Close the file.
    //
    spicelib::ZZDDHCLS(HANDLE, &ARCH, false, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // See if the file is closed.
    //
    spicelib::ZZDDHNFO(
        HANDLE,
        &mut ALTFNM,
        &mut INTARC,
        &mut INTBFF,
        &mut INTAMH,
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    //
    // Clean up.
    //
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid Architecture Mismatch Exception.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"NEW");
    fstr::assign(&mut ARCH, b"DAF");
    HANDLE = 0;

    fstr::assign(&mut ALTARC, b"UNK");

    //
    // Kill FNAME.
    //
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // Open a new file to close.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHCLS(HANDLE, &ALTARC, false, ctx)?;

    //
    // Check for the presence of an exception.
    //
    testutil::CHCKXC(true, b"SPICE(FILARCMISMATCH)", OK, ctx)?;

    //
    // See if the file is closed, it should not be.
    //
    spicelib::ZZDDHNFO(
        HANDLE,
        &mut ALTFNM,
        &mut INTARC,
        &mut INTBFF,
        &mut INTAMH,
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    testutil::CHCKSI(b"INTARC", INTARC, b"=", DAF, 0, OK, ctx)?;
    testutil::CHCKSI(b"INTAMH", INTAMH, b"=", NEW, 0, OK, ctx)?;

    //
    // Close the file.
    //
    spicelib::ZZDDHCLS(HANDLE, &ARCH, false, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // See if the file is closed.
    //
    spicelib::ZZDDHNFO(
        HANDLE,
        &mut ALTFNM,
        &mut INTARC,
        &mut INTBFF,
        &mut INTAMH,
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    //
    // Clean up.
    //
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"KILL set to TRUE, no UNIT attached exception.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    fstr::assign(&mut METHOD, b"NEW");
    fstr::assign(&mut ARCH, b"DAF");

    //
    // Open UTSIZE files.
    //
    for I in 1..=UTSIZE {
        spicelib::REPMI(&FNMPAT, b"#", I, &mut TMPFNM, ctx);
        testutil::KILFIL(&TMPFNM, ctx)?;
        spicelib::ZZDDHOPN(&TMPFNM, &METHOD, &ARCH, &mut HANLST[I], ctx)?;
    }

    //
    // Open an additional file to bump out the first loaded
    // one.
    //
    fstr::assign(&mut FNAME, b"test.fil");
    testutil::KILFIL(&FNAME, ctx)?;
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Close the first file with KILL set.
    //
    spicelib::ZZDDHCLS(HANLST[1], &ARCH, true, ctx)?;

    //
    // Check for the expected exception.
    //
    testutil::CHCKXC(true, b"SPICE(FILENOTCONNECTED)", OK, ctx)?;

    //
    // Check to see that the file is actually closed.
    //
    spicelib::ZZDDHNFO(
        HANLST[1],
        &mut ALTFNM,
        &mut INTARC,
        &mut INTBFF,
        &mut INTAMH,
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    //
    // Clean up.
    //
    spicelib::REPMI(&FNMPAT, b"#", 1, &mut TMPFNM, ctx);
    testutil::KILFIL(&TMPFNM, ctx)?;

    for I in 2..=UTSIZE {
        spicelib::ZZDDHCLS(HANLST[I], &ARCH, false, ctx)?;
        spicelib::REPMI(&FNMPAT, b"#", I, &mut TMPFNM, ctx);
        testutil::KILFIL(&TMPFNM, ctx)?;
    }

    spicelib::ZZDDHCLS(HANDLE, &ARCH, false, ctx)?;
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Nominal Execution.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"NEW");
    fstr::assign(&mut ARCH, b"DAF");
    HANDLE = 0;

    //
    // Kill FNAME.
    //
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // Open a new file to close.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHCLS(HANDLE, &ARCH, false, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // See if the file is closed.
    //
    spicelib::ZZDDHNFO(
        HANDLE,
        &mut ALTFNM,
        &mut INTARC,
        &mut INTBFF,
        &mut INTAMH,
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    //
    // Clean up.
    //
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Nominal Execution - KILL set to TRUE.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"NEW");
    fstr::assign(&mut ARCH, b"DAF");
    HANDLE = 0;

    //
    // Kill FNAME.
    //
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // Open a new file to close.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHCLS(HANDLE, &ARCH, true, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // See if the file is closed.
    //
    spicelib::ZZDDHNFO(
        HANDLE,
        &mut ALTFNM,
        &mut INTARC,
        &mut INTBFF,
        &mut INTAMH,
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    //
    // See if the file was properly deleted.
    //
    testutil::CHCKSL(b"EXISTS", spicelib::EXISTS(&FNAME, ctx)?, false, OK, ctx)?;

    //
    // Clean up.
    //
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Unit Table Clean Up Execution.", ctx)?;

    //
    // This test case is a bit unusual... so I am going to list
    // the steps we will take out here before getting started.
    //
    // (1) Open UTSIZE files.
    // (2) Use ZZDDHHLU to fetch the unit assigned to the first
    //     file loaded.
    // (3) Open one additional file to bump out the first loaded
    //     file from the unit table.
    // (4) Close this additional file.  Now at this point we have
    //     UTSIZE files loaded in the file table, and UTSIZE-1
    //     "active" rows in the unit table and one inactive one.
    // (5) Close the first file loaded.  This will force ZZDDHCLS
    //     to clean up the unit table and remove the row created
    //     when the first file was opened.
    // (6) Using RESLUN reserve the unit originally assigned to
    //     the first file.
    // (7) Open a new file.
    // (8) Fetch its unit using ZZDDHHLU.
    // (9) This unit must disagree with the unit we reserved,
    //     and this disagreement indicates the zero handle row
    //     was properly compressed out of the unit table.
    // (-) Clean up.
    //
    //
    // Setup the inputs and outputs.
    //
    fstr::assign(&mut METHOD, b"NEW");
    fstr::assign(&mut ARCH, b"DAF");

    //
    // Open UTSIZE files.
    //
    for I in 1..=UTSIZE {
        spicelib::REPMI(&FNMPAT, b"#", I, &mut TMPFNM, ctx);
        testutil::KILFIL(&TMPFNM, ctx)?;
        spicelib::ZZDDHOPN(&TMPFNM, &METHOD, &ARCH, &mut HANLST[I], ctx)?;
    }

    //
    // Retrieve the unit assigned to the first file.
    //
    spicelib::ZZDDHHLU(HANLST[1], &ARCH, false, &mut UNIT, ctx)?;

    //
    // Open an additional file to bump out the first loaded
    // one.
    //
    fstr::assign(&mut FNAME, b"test.fil");
    testutil::KILFIL(&FNAME, ctx)?;
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Close this file.
    //
    spicelib::ZZDDHCLS(HANDLE, &ARCH, false, ctx)?;

    //
    // Close the first loaded file.
    //
    spicelib::ZZDDHCLS(HANLST[1], &ARCH, false, ctx)?;

    //
    // Reserve the original unit.
    //
    spicelib::RESLUN(UNIT, ctx);

    //
    // Open another new file.
    //
    fstr::assign(&mut ALTFNM, b"alt.fil");
    testutil::KILFIL(&ALTFNM, ctx)?;
    spicelib::ZZDDHOPN(&ALTFNM, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Retrieve the unit to which it was assigned.
    //
    spicelib::ZZDDHHLU(HANDLE, &ARCH, false, &mut ALTUNT, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Verify that UNIT and ALTUNT do not agree.
    //
    testutil::CHCKSI(b"UNIT", UNIT, b"!=", ALTUNT, 0, OK, ctx)?;

    //
    // Clean up.
    //
    for I in 2..=UTSIZE {
        spicelib::ZZDDHCLS(HANLST[I], &ARCH, false, ctx)?;
        spicelib::REPMI(&FNMPAT, b"#", I, &mut TMPFNM, ctx);
        testutil::KILFIL(&TMPFNM, ctx)?;
    }

    spicelib::ZZDDHCLS(HANDLE, &ARCH, false, ctx)?;
    testutil::KILFIL(&ALTFNM, ctx)?;

    testutil::KILFIL(&FNAME, ctx)?;

    spicelib::REPMI(&FNMPAT, b"#", 1, &mut TMPFNM, ctx);
    testutil::KILFIL(&TMPFNM, ctx)?;

    spicelib::FRELUN(UNIT, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
