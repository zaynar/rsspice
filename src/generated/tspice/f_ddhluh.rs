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

//$Procedure F_DDHLUH ( ZZDDHLUH Test Family )
pub fn F_DDHLUH(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ARCH = [b' '; 20 as usize];
    let mut BINFMT = [b' '; 20 as usize];
    let mut FNAME = [b' '; FILEN as usize];
    let mut FNMPAT = [b' '; FILEN as usize];
    let mut METHOD = [b' '; 20 as usize];
    let mut ALTHAN: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut HANTST = ActualArray::<i32>::new(1..=FTSIZE);
    let mut UNIT: i32 = 0;
    let mut FOUND: bool = false;

    //
    // Local Variables
    //

    //
    // Start the test family with an open call.
    //
    testutil::TOPEN(b"F_DDHLUH", ctx)?;

    //
    // Setup the filename pattern for loop tests.
    //
    fstr::assign(&mut FNMPAT, b"test#.fil");

    //
    // Retrive the native binary file format string to use
    // when creating test files.
    //
    spicelib::ZZPLATFM(b"FILE_FORMAT", &mut BINFMT, ctx);

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Exercise Nominal Logic", ctx)?;

    //
    // Load a file into the handle manager.
    //
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"READ");
    fstr::assign(&mut ARCH, b"DAF");
    HANDLE = 0;
    ALTHAN = 0;

    testutil::KILFIL(&FNAME, ctx)?;

    //
    // Create FNAME.
    //
    T_CPTFIL(
        &FNAME,
        DAF,
        2,
        &BINFMT,
        b"ABCD",
        b"EFGH",
        b"IJKL",
        true,
        false,
        b"DAF/CK  ",
        ctx,
    )?;

    //
    // Open the file.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Fetch the UNIT for HANDLE.
    //
    spicelib::ZZDDHHLU(HANDLE, &ARCH, false, &mut UNIT, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now retrieve the HANDLE for this UNIT.
    //
    spicelib::ZZDDHLUH(UNIT, &mut ALTHAN, &mut FOUND, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now check the values.
    //
    testutil::CHCKSI(b"HANDLE", HANDLE, b"=", ALTHAN, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    //
    // Close the file and clean up.
    //
    spicelib::ZZDDHCLS(HANDLE, &ARCH, false, ctx)?;
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Exercise Exceptional Case Logic", ctx)?;

    //
    // Open a handful of files.
    //
    for I in 1..=(UTSIZE + 5) {
        spicelib::REPMI(&FNMPAT, b"#", I, &mut FNAME, ctx);
        spicelib::ZZDDHOPN(&FNAME, b"NEW", b"DAF", &mut HANTST[I], ctx)?;
    }

    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut ARCH, b"DAF");
    fstr::assign(&mut METHOD, b"READ");
    HANDLE = 0;
    FOUND = true;

    testutil::KILFIL(&FNAME, ctx)?;

    //
    // Create FNAME.
    //
    T_CPTFIL(
        &FNAME,
        DAF,
        2,
        &BINFMT,
        b"ABCD",
        b"EFGH",
        b"IJKL",
        true,
        false,
        b"DAF/CK  ",
        ctx,
    )?;

    //
    // Open the file.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Lock HANDLE to its unit.
    //
    spicelib::ZZDDHHLU(HANDLE, &ARCH, true, &mut UNIT, ctx)?;

    //
    // Now close HANDLE, since we have more than UTSIZE files loaded,
    // UNIT will end up being reserved by ZZDDHCLS.
    //
    spicelib::ZZDDHCLS(HANDLE, &ARCH, false, ctx)?;

    //
    // Now ask for the handle associated with UNIT.
    //
    spicelib::ZZDDHLUH(UNIT, &mut ALTHAN, &mut FOUND, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the outputs.
    //
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSI(b"HANDLE", ALTHAN, b"=", 0, 0, OK, ctx)?;

    //
    // Clean up.
    //
    testutil::KILFIL(&FNAME, ctx)?;

    for I in 1..=(UTSIZE + 5) {
        spicelib::ZZDDHCLS(HANTST[I], &ARCH, false, ctx)?;
        spicelib::REPMI(&FNMPAT, b"#", I, &mut FNAME, ctx);
        testutil::KILFIL(&FNAME, ctx)?;
    }

    //
    // Lastly, check to see if we send it a unit that is not in use by
    // the handle manager it returns the appropriate values.
    //
    spicelib::GETLUN(&mut UNIT, ctx)?;

    FOUND = true;
    HANDLE = 1;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHLUH(UNIT, &mut HANDLE, &mut FOUND, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the values.
    //
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSI(b"HANDLE", HANDLE, b"=", 0, 0, OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
