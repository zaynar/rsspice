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

//$Procedure F_DDHFNH ( ZZDDHFNH Test Family )
pub fn F_DDHFNH(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ALTFNM = [b' '; FILEN as usize];
    let mut BINFMT = [b' '; 20 as usize];
    let mut ARCH = [b' '; 20 as usize];
    let mut FNAME = [b' '; FILEN as usize];
    let mut METHOD = [b' '; 20 as usize];
    let mut ALTHAN: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut FOUND: bool = false;

    //
    // Local Variables
    //

    //
    // Start the test family with an open call.
    //
    testutil::TOPEN(b"F_DDHFNH", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"F_DDHFNH Initialization", ctx)?;

    //
    // Setup the test cases.
    //
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut ARCH, b"DAF");
    fstr::assign(&mut METHOD, b"READ");
    HANDLE = 0;
    FOUND = true;

    //
    // Get the native binary file format string to utilize in
    // creating the test file.
    //
    spicelib::ZZPLATFM(b"FILE_FORMAT", &mut BINFMT, ctx);

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
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Exercise Nominal ZZDDHFNH Logic", ctx)?;

    //
    // Now exercise ZZDDHFNH.
    //
    spicelib::ZZDDHFNH(&FNAME, &mut ALTHAN, &mut FOUND, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check outputs.
    //
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"ALTHAN", ALTHAN, b"=", HANDLE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Exercise LJUST of FNAME Logic", ctx)?;

    //
    // Now exercise the insignificance of leading white space.
    //
    fstr::assign(&mut ALTFNM, &fstr::concat(b"       ", &FNAME));

    spicelib::ZZDDHFNH(&ALTFNM, &mut ALTHAN, &mut FOUND, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check outputs.
    //
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"ALTHAN", ALTHAN, b"=", HANDLE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Exercise Blank Filename Exception", ctx)?;

    //
    // Lastly exercise 2 "NOT FOUND" cases.
    //
    spicelib::ZZDDHFNH(b"     ", &mut ALTHAN, &mut FOUND, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check outputs.
    //
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSI(b"ALTHAN", ALTHAN, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Exercise Unknown Filename Exception", ctx)?;

    spicelib::ZZDDHFNH(b"BOGUS_FILENAME_GOES_HERE", &mut ALTHAN, &mut FOUND, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check outputs.
    //
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKSI(b"ALTHAN", ALTHAN, b"=", 0, 0, OK, ctx)?;

    //
    // Clean up.
    //
    spicelib::ZZDDHCLS(HANDLE, &ARCH, false, ctx)?;
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
