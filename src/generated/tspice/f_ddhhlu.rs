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

//$Procedure F_DDHHLU ( ZZDDHHLU Test Family )
pub fn F_DDHHLU(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ALTARC = [b' '; 20 as usize];
    let mut ARCH = [b' '; 20 as usize];
    let mut FNAME = [b' '; FILEN as usize];
    let mut FNMPAT = [b' '; FILEN as usize];
    let mut METHOD = [b' '; 20 as usize];
    let mut TMPFNM = [b' '; FILEN as usize];
    let mut ALTUNT: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut HANLST = StackArray::<i32, 23>::new(1..=UTSIZE);
    let mut IOSTAT: i32 = 0;
    let mut UNIT: i32 = 0;
    let mut LOCK: bool = false;

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Start the test family with an open call.
    //
    testutil::TOPEN(b"F_DDHHLU", ctx)?;

    //
    // Set the filename pattern.
    //
    fstr::assign(&mut FNMPAT, b"test#.fil");

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Missing Handle Exception.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    HANDLE = 0;
    fstr::assign(&mut ARCH, b"DAF");
    LOCK = false;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHHLU(HANDLE, &ARCH, LOCK, &mut UNIT, ctx)?;

    //
    // Now check for the presence of the exception.
    //
    testutil::CHCKXC(true, b"SPICE(NOSUCHHANDLE)", OK, ctx)?;

    //
    // Check the value of UNIT, it should be 0.
    //
    testutil::CHCKSI(b"UNIT", UNIT, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Alternate Signed Handle Exception.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    HANDLE = 0;
    fstr::assign(&mut ARCH, b"DAF");
    LOCK = false;
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"NEW");

    //
    // Kill FNAME.
    //
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // Open the new test file.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHHLU(-HANDLE, &ARCH, LOCK, &mut UNIT, ctx)?;

    //
    // Now check for the presence of the exception.
    //
    testutil::CHCKXC(true, b"SPICE(NOSUCHHANDLE)", OK, ctx)?;

    //
    // Check the value of UNIT, it should be 0.
    //
    testutil::CHCKSI(b"UNIT", UNIT, b"=", 0, 0, OK, ctx)?;

    //
    // Clean up.
    //
    spicelib::ZZDDHCLS(HANDLE, &ARCH, false, ctx)?;
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Architecture Mismatch Exception.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    HANDLE = 0;
    fstr::assign(&mut ARCH, b"DAF");
    LOCK = false;
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"NEW");
    fstr::assign(&mut ALTARC, b"DAS");

    //
    // Kill FNAME.
    //
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // Open the new test file.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHHLU(HANDLE, &ALTARC, LOCK, &mut UNIT, ctx)?;

    //
    // Now check for the presence of the exception.
    //
    testutil::CHCKXC(true, b"SPICE(FILARCMISMATCH)", OK, ctx)?;

    //
    // Check the value of UNIT, it should be 0.
    //
    testutil::CHCKSI(b"UNIT", UNIT, b"=", 0, 0, OK, ctx)?;

    //
    // Clean up.
    //
    spicelib::ZZDDHCLS(HANDLE, &ARCH, false, ctx)?;
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid Architecture Mismatch Exception.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    HANDLE = 0;
    fstr::assign(&mut ARCH, b"DAF");
    LOCK = false;
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"NEW");
    fstr::assign(&mut ALTARC, b"UNK");

    //
    // Kill FNAME.
    //
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // Open the new test file.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHHLU(HANDLE, &ALTARC, LOCK, &mut UNIT, ctx)?;

    //
    // Now check for the presence of the exception.
    //
    testutil::CHCKXC(true, b"SPICE(FILARCMISMATCH)", OK, ctx)?;

    //
    // Check the value of UNIT, it should be 0.
    //
    testutil::CHCKSI(b"UNIT", UNIT, b"=", 0, 0, OK, ctx)?;

    //
    // Clean up.
    //
    spicelib::ZZDDHCLS(HANDLE, &ARCH, false, ctx)?;
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Handle Lock Failure Exception.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    HANDLE = 0;
    fstr::assign(&mut ARCH, b"DAF");
    LOCK = true;
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"NEW");

    //
    // Open (UTSIZE - RSVUNT - SCRUNT - 1) files, and lock them to
    // their units.
    //
    for I in 1..=((UTSIZE - RSVUNT) - SCRUNT) {
        spicelib::REPMI(&FNMPAT, b"#", I, &mut TMPFNM, ctx);
        testutil::KILFIL(&TMPFNM, ctx)?;
        spicelib::ZZDDHOPN(&TMPFNM, &METHOD, &ARCH, &mut HANLST[I], ctx)?;
        spicelib::ZZDDHHLU(HANLST[I], &ARCH, LOCK, &mut UNIT, ctx)?;
    }

    //
    // Check for any rogue exceptions.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now open FNAME.
    //
    testutil::KILFIL(&FNAME, ctx)?;
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHHLU(HANDLE, &ARCH, LOCK, &mut UNIT, ctx)?;

    //
    // Check for the presence of the exception.
    //
    testutil::CHCKXC(true, b"SPICE(HLULOCKFAILED)", OK, ctx)?;

    //
    // Check that UNIT is 0.
    //
    testutil::CHCKSI(b"UNIT", UNIT, b"=", 0, 0, OK, ctx)?;

    //
    // Clean up.
    //
    for I in 1..=((UTSIZE - RSVUNT) - SCRUNT) {
        spicelib::ZZDDHCLS(HANLST[I], &ARCH, false, ctx)?;
        spicelib::REPMI(&FNMPAT, b"#", I, &mut TMPFNM, ctx);
        testutil::KILFIL(&TMPFNM, ctx)?;
    }

    spicelib::ZZDDHCLS(HANDLE, &ARCH, false, ctx)?;
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Nominal Non-lock No LUN Rotate HLU Operation.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    HANDLE = 0;
    fstr::assign(&mut ARCH, b"DAF");
    LOCK = false;
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"NEW");

    //
    // Now open FNAME.
    //
    testutil::KILFIL(&FNAME, ctx)?;
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // At this point we know FNAME is attached to a UNIT.  Do an
    // INQUIRE to determine which one.
    //
    {
        use f2rust_std::io;

        let specs = io::InquireSpecs {
            file: Some(&FNAME),
            number: Some(&mut ALTUNT),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.inquire(specs))?;
    }

    //
    // Check IOSTAT.
    //
    testutil::CHCKSI(b"IOSTAT", IOSTAT, b"=", 0, 0, OK, ctx)?;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHHLU(HANDLE, &ARCH, LOCK, &mut UNIT, ctx)?;

    //
    // Check for the absence of the exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check that UNIT is ALTUNT.  We know this must be true,
    // since no other operations have caused HLU to cycle units.
    //
    testutil::CHCKSI(b"UNIT", UNIT, b"=", ALTUNT, 0, OK, ctx)?;

    //
    // Clean up.
    //
    spicelib::ZZDDHCLS(HANDLE, &ARCH, false, ctx)?;
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Nominal Non-lock LUN Rotate HLU Operation.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    HANDLE = 0;
    fstr::assign(&mut ARCH, b"DAF");
    LOCK = false;
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"NEW");

    //
    // Open UTSIZE files.
    //
    for I in 1..=UTSIZE {
        spicelib::REPMI(&FNMPAT, b"#", I, &mut TMPFNM, ctx);
        testutil::KILFIL(&TMPFNM, ctx)?;
        spicelib::ZZDDHOPN(&TMPFNM, &METHOD, &ARCH, &mut HANLST[I], ctx)?;
    }

    //
    // Check for any rogue exceptions.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now perform an INQUIRE on the first file opened to retrieve
    // it's logical unit.  The cost function system guarentees that
    // we will retrieve (as long as this test is run before INTMAX
    // requests for logical units have been made...) the unit for
    // this file when we open the next one.
    //
    spicelib::REPMI(&FNMPAT, b"#", 1, &mut TMPFNM, ctx);

    {
        use f2rust_std::io;

        let specs = io::InquireSpecs {
            file: Some(&TMPFNM),
            number: Some(&mut ALTUNT),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.inquire(specs))?;
    }

    //
    // Check IOSTAT.
    //
    testutil::CHCKSI(b"IOSTAT", IOSTAT, b"=", 0, 0, OK, ctx)?;

    //
    // Now open FNAME.
    //
    testutil::KILFIL(&FNAME, ctx)?;
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHHLU(HANDLE, &ARCH, LOCK, &mut UNIT, ctx)?;

    //
    // Check for the absence of the exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check that UNIT is ALTUNT.
    //
    testutil::CHCKSI(b"UNIT", UNIT, b"=", ALTUNT, 0, OK, ctx)?;

    //
    // Clean up.
    //
    for I in 1..=UTSIZE {
        spicelib::ZZDDHCLS(HANLST[I], &ARCH, false, ctx)?;
        spicelib::REPMI(&FNMPAT, b"#", I, &mut TMPFNM, ctx);
        testutil::KILFIL(&TMPFNM, ctx)?;
    }

    spicelib::ZZDDHCLS(HANDLE, &ARCH, false, ctx)?;
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Rotate in no-lock HLU Operation.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    HANDLE = 0;
    fstr::assign(&mut ARCH, b"DAF");
    LOCK = false;
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"NEW");

    //
    // Now open FNAME.
    //
    testutil::KILFIL(&FNAME, ctx)?;
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Open UTSIZE files.
    //
    for I in 1..=UTSIZE {
        spicelib::REPMI(&FNMPAT, b"#", I, &mut TMPFNM, ctx);
        testutil::KILFIL(&TMPFNM, ctx)?;
        spicelib::ZZDDHOPN(&TMPFNM, &METHOD, &ARCH, &mut HANLST[I], ctx)?;
    }

    //
    // Check for any rogue exceptions.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now perform an INQUIRE on the first file opened to retrieve
    // it's logical unit.  The cost function system guarentees that
    // we will retrieve (as long as this test is run before INTMAX
    // requests for logical units have been made...) the unit for
    // this file when we request a unit for FNAME.
    //
    spicelib::REPMI(&FNMPAT, b"#", 1, &mut TMPFNM, ctx);

    {
        use f2rust_std::io;

        let specs = io::InquireSpecs {
            file: Some(&TMPFNM),
            number: Some(&mut ALTUNT),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.inquire(specs))?;
    }

    //
    // Check IOSTAT.
    //
    testutil::CHCKSI(b"IOSTAT", IOSTAT, b"=", 0, 0, OK, ctx)?;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHHLU(HANDLE, &ARCH, LOCK, &mut UNIT, ctx)?;

    //
    // Check for the absence of the exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check that UNIT is ALTUNT.
    //
    testutil::CHCKSI(b"UNIT", UNIT, b"=", ALTUNT, 0, OK, ctx)?;

    //
    // Clean up.
    //
    for I in 1..=UTSIZE {
        spicelib::ZZDDHCLS(HANLST[I], &ARCH, false, ctx)?;
        spicelib::REPMI(&FNMPAT, b"#", I, &mut TMPFNM, ctx);
        testutil::KILFIL(&TMPFNM, ctx)?;
    }

    spicelib::ZZDDHCLS(HANDLE, &ARCH, false, ctx)?;
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Lock before cycle HLU Operation.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    HANDLE = 0;
    fstr::assign(&mut ARCH, b"DAF");
    LOCK = false;
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"NEW");

    //
    // Now open FNAME.
    //
    testutil::KILFIL(&FNAME, ctx)?;
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Lock the UNIT.
    //
    spicelib::ZZDDHHLU(HANDLE, &ARCH, true, &mut UNIT, ctx)?;

    //
    // Open UTSIZE files.
    //
    for I in 1..=UTSIZE {
        spicelib::REPMI(&FNMPAT, b"#", I, &mut TMPFNM, ctx);
        testutil::KILFIL(&TMPFNM, ctx)?;
        spicelib::ZZDDHOPN(&TMPFNM, &METHOD, &ARCH, &mut HANLST[I], ctx)?;
    }

    //
    // Check for any rogue exceptions.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Since we opened UTSIZE files, if the lock is not functioning,
    // then we would have cycled off HANDLE from the unit table.
    // Check.
    //
    spicelib::ZZDDHHLU(HANDLE, &ARCH, false, &mut ALTUNT, ctx)?;

    //
    // Check for the absence of any exceptions.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the value of ALTUNT matches UNIT.
    //
    testutil::CHCKSI(b"UNIT", UNIT, b"=", ALTUNT, 0, OK, ctx)?;

    //
    // Lastly cycle the UTSIZE files again to verify that the
    // addition of the .FALSE. in the LOCK argument of the last call
    // to ZZDDHHLU does not inadvertantly unlock the file.
    //
    for I in 1..=UTSIZE {
        spicelib::ZZDDHHLU(HANLST[I], &ARCH, false, &mut ALTUNT, ctx)?;
    }

    //
    // Check for any rogue exceptions.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the unit attached to HANDLE.
    //
    spicelib::ZZDDHHLU(HANDLE, &ARCH, true, &mut ALTUNT, ctx)?;

    //
    // Check for the absence of any exceptions.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the value of ALTUNT matches UNIT.
    //
    testutil::CHCKSI(b"UNIT", UNIT, b"=", ALTUNT, 0, OK, ctx)?;

    //
    // Clean up.
    //
    for I in 1..=UTSIZE {
        spicelib::ZZDDHCLS(HANLST[I], &ARCH, false, ctx)?;
        spicelib::REPMI(&FNMPAT, b"#", I, &mut TMPFNM, ctx);
        testutil::KILFIL(&TMPFNM, ctx)?;
    }

    spicelib::ZZDDHCLS(HANDLE, &ARCH, false, ctx)?;
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Lock after cycle HLU Operation.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    HANDLE = 0;
    fstr::assign(&mut ARCH, b"DAF");
    LOCK = false;
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"NEW");

    //
    // Now open FNAME.
    //
    testutil::KILFIL(&FNAME, ctx)?;
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Open UTSIZE files.
    //
    for I in 1..=UTSIZE {
        spicelib::REPMI(&FNMPAT, b"#", I, &mut TMPFNM, ctx);
        testutil::KILFIL(&TMPFNM, ctx)?;
        spicelib::ZZDDHOPN(&TMPFNM, &METHOD, &ARCH, &mut HANLST[I], ctx)?;
    }

    //
    // Check for any rogue exceptions.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Since we opened UTSIZE files, we have cycled HANDLE out
    // of the unit table.  Restore and lock it in place.
    //
    spicelib::ZZDDHHLU(HANDLE, &ARCH, true, &mut UNIT, ctx)?;

    //
    // Check for the absence of any exceptions.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now cycle the UTSIZE files to verify that the lock on
    // the UNIT for HANDLE is not broken.
    //
    for I in 1..=UTSIZE {
        spicelib::ZZDDHHLU(HANLST[I], &ARCH, false, &mut ALTUNT, ctx)?;
    }

    //
    // Check for any rogue exceptions.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the unit attached to HANDLE.
    //
    spicelib::ZZDDHHLU(HANDLE, &ARCH, true, &mut ALTUNT, ctx)?;

    //
    // Check for the absence of any exceptions.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the value of ALTUNT matches UNIT.
    //
    testutil::CHCKSI(b"UNIT", UNIT, b"=", ALTUNT, 0, OK, ctx)?;

    //
    // Clean up.
    //
    for I in 1..=UTSIZE {
        spicelib::ZZDDHCLS(HANLST[I], &ARCH, false, ctx)?;
        spicelib::REPMI(&FNMPAT, b"#", I, &mut TMPFNM, ctx);
        testutil::KILFIL(&TMPFNM, ctx)?;
    }

    spicelib::ZZDDHCLS(HANDLE, &ARCH, false, ctx)?;
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
