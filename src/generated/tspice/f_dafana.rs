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
const NUMDAF: i32 = 21;
const SUMLEN: i32 = 125;

//$Procedure F_DAFANA ( DAFANA Test Family )
pub fn F_DAFANA(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut FNAME = [b' '; FILEN as usize];
    let mut FNMTMP = [b' '; FILEN as usize];
    let mut SUM = StackArray::<f64, 125>::new(1..=SUMLEN);
    let mut ONES = StackArray::<f64, 125>::new(1..=SUMLEN);
    let mut VALS = StackArray::<f64, 125>::new(1..=SUMLEN);
    let mut HANLST = StackArray::<i32, 21>::new(1..=NUMDAF);

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Start the test family with an open call.
    //
    testutil::TOPEN(b"F_DAFANA", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"F_DAFANA Initialization", ctx)?;

    //
    // Set the file name template.
    //
    fstr::assign(&mut FNMTMP, b"test#.daf");

    //
    // Initialize the summary record for testing purposes.
    //
    for I in 1..=SUMLEN {
        SUM[I] = 0.0;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"DAFBNA SPICE(STFULL) Exception", ctx)?;

    //
    // Create and load some test DAFs to use.
    //
    for I in 1..=NUMDAF {
        spicelib::REPMI(&FNMTMP, b"#", I, &mut FNAME, ctx);
        testutil::KILFIL(&FNAME, ctx)?;
        spicelib::DAFONW(&FNAME, b"SPK ", 2, 6, b"TEST DAF", 0, &mut HANLST[I], ctx)?;
    }

    //
    // Since NUMDAF is 1 more than the number of DAFs DAFANA's
    // state table can track, attempt to initiate a search on
    // all NUMDAF DAFs and check for the error.
    //
    for I in 1..=NUMDAF {
        spicelib::DAFBNA(HANLST[I], SUM.as_slice(), b"TEST SEGMENT", ctx)?;
    }

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(STFULL)", OK, ctx)?;

    //
    // Clean up.
    //
    for I in 1..=NUMDAF {
        spicelib::DAFCLS(HANLST[I], ctx)?;
        spicelib::REPMI(&FNMTMP, b"#", I, &mut FNAME, ctx);
        testutil::KILFIL(&FNAME, ctx)?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"DAFBNA Remove Inactive Entries in State Table", ctx)?;

    //
    // Create and load some test DAFs to use.
    //
    for I in 1..=NUMDAF {
        spicelib::REPMI(&FNMTMP, b"#", I, &mut FNAME, ctx);
        testutil::KILFIL(&FNAME, ctx)?;
        spicelib::DAFONW(&FNAME, b"SPK ", 2, 6, b"TEST DAF", 0, &mut HANLST[I], ctx)?;
    }

    //
    // Now start appends on the first NUMDAF-1 DAFs.
    //
    for I in 1..=(NUMDAF - 1) {
        spicelib::DAFBNA(HANLST[I], SUM.as_slice(), b"TEST SEGMENT", ctx)?;
        spicelib::DAFADA(SUM.as_slice(), SUMLEN, ctx)?;
    }

    //
    // Conclude writing to the first DAF.
    //
    spicelib::DAFCAD(HANLST[1], ctx)?;
    spicelib::DAFENA(ctx)?;

    //
    // Now attempt to write to the NUMDAF DAF.
    //
    spicelib::DAFBNA(HANLST[NUMDAF], SUM.as_slice(), b"TEST SEGMENT", ctx)?;

    for I in 1..=SUMLEN {
        ONES[I] = 1.0;
    }

    spicelib::DAFADA(ONES.as_slice(), SUMLEN, ctx)?;
    spicelib::DAFENA(ctx)?;

    //
    // Check to see that no exception was signaled.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // And that the data was added appropriately.
    //
    spicelib::CLEARD(SUMLEN, VALS.as_slice_mut());
    spicelib::DAFRDA(HANLST[NUMDAF], 385, 509, VALS.as_slice_mut(), ctx)?;

    testutil::CHCKAD(
        b"VALS",
        VALS.as_slice(),
        b"=",
        ONES.as_slice(),
        SUMLEN,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Lastly, begin appending a new array to the file we originally
    // bumped.
    //
    spicelib::DAFBNA(HANLST[1], SUM.as_slice(), b"TEST SEGMENT 2", ctx)?;
    spicelib::DAFADA(ONES.as_slice(), SUMLEN, ctx)?;
    spicelib::DAFENA(ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the values of the data we just added.
    //
    spicelib::CLEARD(SUMLEN, VALS.as_slice_mut());
    spicelib::DAFRDA(HANLST[1], 510, 634, VALS.as_slice_mut(), ctx)?;

    testutil::CHCKAD(
        b"VALS",
        VALS.as_slice(),
        b"=",
        ONES.as_slice(),
        SUMLEN,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Clean up.
    //
    for I in 1..=NUMDAF {
        spicelib::DAFCLS(HANLST[I], ctx)?;
        spicelib::REPMI(&FNMTMP, b"#", I, &mut FNAME, ctx);
        testutil::KILFIL(&FNAME, ctx)?;
    }

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
