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

//$Procedure F_DDHUNL ( ZZDDHUNL Test Family )
pub fn F_DDHUNL(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ALTARC = [b' '; 20 as usize];
    let mut ARCH = [b' '; 20 as usize];
    let mut FNAME = [b' '; FILEN as usize];
    let mut FNMPAT = [b' '; FILEN as usize];
    let mut METHOD = [b' '; 20 as usize];
    let mut TMPFNM = [b' '; FILEN as usize];
    let mut ALTUNT: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut HANLST = StackArray::<i32, 23>::new(1..=UTSIZE);
    let mut UNIT: i32 = 0;

    //
    // Local Variables
    //

    //
    // Start the test family with an open call.
    //
    testutil::TOPEN(b"F_DDHUNL", ctx)?;

    //
    // Set the filename pattern we will use.
    //
    fstr::assign(&mut FNMPAT, b"test#.fil");

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Zero Handle NO-OP Exception", ctx)?;

    //
    // Set the inputs and outputs.
    //
    HANDLE = 0;
    fstr::assign(&mut ARCH, b"DAF");

    //
    // Invoke the module.
    //
    spicelib::ZZDDHUNL(HANDLE, &ARCH, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Missing Handle NO-OP Exception", ctx)?;

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
    // Open FNAME as a new file, close it and delete it.  This will
    // allocate a HANDLE, and once a HANDLE is allocated it will not
    // be used again.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;
    spicelib::ZZDDHCLS(HANDLE, &ARCH, false, ctx)?;
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // Now at this point, HANDLE has been used but is not currently
    // in use.  Invoke the module.
    //
    spicelib::ZZDDHUNL(HANDLE, &ARCH, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Alternate Sign Handle NO-OP Exception", ctx)?;

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
    // Open FNAME as a new file, but this time close and delete it
    // afterwards.  We will go to ZZDDHUNL with -HANDLE.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHUNL(-HANDLE, &ARCH, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close and delete the file.
    //
    spicelib::ZZDDHCLS(HANDLE, &ARCH, false, ctx)?;
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Handle-Unit Not Locked NO-OP Exception", ctx)?;

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
    // Open FNAME as a new file.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHUNL(HANDLE, &ALTARC, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Clean up.
    //
    spicelib::ZZDDHCLS(HANDLE, &ARCH, false, ctx)?;
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Architecture Mismatch Exception", ctx)?;

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
    // Open FNAME as a new file.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // We have to lock the handle to its unit to reach the
    // the architecture exception.  (This may be a bad design.)
    //
    spicelib::ZZDDHHLU(HANDLE, &ARCH, true, &mut UNIT, ctx)?;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHUNL(HANDLE, &ALTARC, ctx)?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(FILARCMISMATCH)", OK, ctx)?;

    //
    // Clean up.
    //
    spicelib::ZZDDHCLS(HANDLE, &ARCH, false, ctx)?;
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad Architecture Code Exception", ctx)?;

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
    // Open FNAME as a new file.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // We have to lock the handle to its unit to reach the
    // the architecture exception.  (This may be a bad design.)
    //
    spicelib::ZZDDHHLU(HANDLE, &ARCH, true, &mut UNIT, ctx)?;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHUNL(HANDLE, &ALTARC, ctx)?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(FILARCMISMATCH)", OK, ctx)?;

    //
    // Clean up.
    //
    spicelib::ZZDDHCLS(HANDLE, &ARCH, false, ctx)?;
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Nominal Execution", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"NEW");
    fstr::assign(&mut ARCH, b"DAF");
    HANDLE = 0;
    UNIT = 0;
    ALTUNT = 0;

    //
    // Kill FNAME.
    //
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // Now to perform this test, we need to verify that the unlock
    // actually occurs.  To do this we will create UTSIZE additional
    // files and load them, then proceed by making logical unit
    // requests for each of them to cycle FNAME out of the unit table.
    //
    for I in 1..=UTSIZE {
        spicelib::REPMI(&FNMPAT, b"#", I, &mut TMPFNM, ctx);
        testutil::KILFIL(&TMPFNM, ctx)?;
    }

    //
    // Open test.fil.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Lock HANDLE to it's UNIT and retrieve the unit.
    //
    spicelib::ZZDDHHLU(HANDLE, &ARCH, true, &mut UNIT, ctx)?;

    //
    // Check to see that UNIT is not zero.
    //
    testutil::CHCKSI(b"UNIT", UNIT, b"!=", 0, 0, OK, ctx)?;

    //
    // Unlock the unit.
    //
    spicelib::ZZDDHUNL(HANDLE, &ARCH, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now open the UTSIZE files and to cycle through the units.
    //
    for I in 1..=UTSIZE {
        spicelib::REPMI(&FNMPAT, b"#", I, &mut TMPFNM, ctx);
        spicelib::ZZDDHOPN(&TMPFNM, &METHOD, &ARCH, &mut HANLST[I], ctx)?;
    }

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now request the UNIT for HANDLE.
    //
    spicelib::ZZDDHHLU(HANDLE, &ARCH, true, &mut ALTUNT, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Verify that ALTUNT is not UNIT.  What follows is a brief
    // discussion as to why this works.  Note it relies heavily
    // on the current implementation of ZZDDHMAN and it's
    // subroutines:
    //
    //    At the start of this routine, we assume no files
    //    are loaded in the handle manager.  This is expected for
    //    a test family.
    //
    //    When FNAME is open, it is assigned a unit and placed
    //    at the head of the unit table.
    //
    //    Once the UTSIZE files are loaded, the cost bumping
    //    system works such that the last file loaded into the
    //    handle manager takes FNAME's place.
    //
    //    Then when we make the second ZZDDHUNL call the file
    //    FNAME gets assigned the logical unit that was assigned
    //    to the first file in the list of UTSIZE files.
    //
    //
    // As convoluted as that is, it works.  Do the consistency check.
    //
    testutil::CHCKSI(b"ALTUNT", ALTUNT, b"!=", UNIT, 0, OK, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Clean up.
    //
    spicelib::ZZDDHCLS(HANDLE, &ARCH, false, ctx)?;
    testutil::KILFIL(&FNAME, ctx)?;

    for I in 1..=UTSIZE {
        spicelib::ZZDDHCLS(HANLST[I], &ARCH, false, ctx)?;
        spicelib::REPMI(&FNMPAT, b"#", I, &mut TMPFNM, ctx);
        testutil::KILFIL(&TMPFNM, ctx)?;
    }

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Nominal Scratch File Execution", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    fstr::assign(&mut METHOD, b"NEW");
    fstr::assign(&mut ARCH, b"DAF");
    HANDLE = 0;
    UNIT = 0;
    ALTUNT = 0;

    //
    // Now to perform this test, we need to verify that the unlock
    // actually occurs.  To do this we will create UTSIZE additional
    // files and load them, then proceed by making logical unit
    // requests for each of them to cycle FNAME out of the unit table.
    //
    for I in 1..=UTSIZE {
        spicelib::REPMI(&FNMPAT, b"#", I, &mut TMPFNM, ctx);
        testutil::KILFIL(&TMPFNM, ctx)?;
    }

    //
    // Open test.fil.
    //
    spicelib::ZZDDHOPN(b" ", b"SCRATCH", &ARCH, &mut HANDLE, ctx)?;

    //
    // Retrieve the logical unit.  Scratch files are locked to their
    // units by the open routine.
    //
    spicelib::ZZDDHHLU(HANDLE, &ARCH, false, &mut UNIT, ctx)?;

    //
    // Check to see that UNIT is not zero.
    //
    testutil::CHCKSI(b"UNIT", UNIT, b"!=", 0, 0, OK, ctx)?;

    //
    // Attempt to unlock the unit.
    //
    spicelib::ZZDDHUNL(HANDLE, &ARCH, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now open the UTSIZE files and to cycle through the units.
    //
    for I in 1..=UTSIZE {
        spicelib::REPMI(&FNMPAT, b"#", I, &mut TMPFNM, ctx);
        spicelib::ZZDDHOPN(&TMPFNM, &METHOD, &ARCH, &mut HANLST[I], ctx)?;
    }

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now request the UNIT for HANDLE.
    //
    spicelib::ZZDDHHLU(HANDLE, &ARCH, true, &mut ALTUNT, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Since SCRATCH files must always be locked to their logical units,
    // the call to ZZDDHUNL is effectively a no-op.  From the nominal
    // execution case, we know that opening UTSIZE files should force
    // the original scratch file to be rotated out of the unit table
    // in ZZDDHMAN and be reassigned a different unit.  Since this can
    // not happen, check to see if ALTUNT is UNIT (verifying the lock).
    //
    testutil::CHCKSI(b"ALTUNT", ALTUNT, b"=", UNIT, 0, OK, ctx)?;

    //
    // Clean up.
    //
    spicelib::ZZDDHCLS(HANDLE, &ARCH, false, ctx)?;
    testutil::KILFIL(&FNAME, ctx)?;

    for I in 1..=UTSIZE {
        spicelib::ZZDDHCLS(HANLST[I], &ARCH, false, ctx)?;
        spicelib::REPMI(&FNMPAT, b"#", I, &mut TMPFNM, ctx);
        testutil::KILFIL(&TMPFNM, ctx)?;
    }

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
