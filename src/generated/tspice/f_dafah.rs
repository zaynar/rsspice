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
const NUMDAF: i32 = (UTSIZE + 5);

//$Procedure F_DAFAH ( DAFAH Test Family )
pub fn F_DAFAH(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut FNAME = [b' '; FILEN as usize];
    let mut FNMTMP = [b' '; FILEN as usize];
    let mut OUTNAM = [b' '; FILEN as usize];
    let mut DASNAM = [b' '; FILEN as usize];
    let mut I: i32 = 0;
    let mut HANLST = StackArray::<i32, 28>::new(1..=NUMDAF);
    let mut HANDLE: i32 = 0;
    let mut OUTHAN: i32 = 0;
    let mut UNIT: i32 = 0;

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Start the test family with an open call.
    //
    testutil::TOPEN(b"F_DAFAH", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"F_DAFAH Initialization", ctx)?;

    //
    // Create some test DAFs to use.
    //
    fstr::assign(&mut FNMTMP, b"test#.daf");

    {
        let m1__: i32 = 1;
        let m2__: i32 = NUMDAF;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::REPMI(&FNMTMP, b"#", I, &mut FNAME, ctx);
            testutil::TSTSPK(&FNAME, false, &mut HANDLE, ctx)?;
            I += m3__;
        }
    }

    //
    // Create something that looks like a DAS.
    //
    fstr::assign(&mut DASNAM, b"test.das");

    testutil::KILFIL(&DASNAM, ctx)?;

    T_CPTFIL(
        &DASNAM,
        DAS,
        2,
        b"        ",
        b"ABCD",
        b"EFGH",
        b"IJKL",
        false,
        false,
        b" ",
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"DAFHLU Lock Check", ctx)?;

    //
    // Open UTSIZE - RSVUNT - SCRUNT and lock them to their
    // units.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = ((UTSIZE - RSVUNT) - SCRUNT);
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::REPMI(&FNMTMP, b"#", I, &mut FNAME, ctx);
            spicelib::DAFOPR(&FNAME, &mut HANLST[I], ctx)?;
            spicelib::DAFHLU(HANLST[I], &mut UNIT, ctx)?;

            I += m3__;
        }
    }

    //
    // Now attempt to open and lock one additional file.
    // This should generate an error from the handle manager.
    //
    I = (((UTSIZE - RSVUNT) - SCRUNT) + 1);

    spicelib::REPMI(&FNMTMP, b"#", I, &mut FNAME, ctx);
    spicelib::DAFOPR(&FNAME, &mut HANDLE, ctx)?;

    //
    // Check to make sure we have no exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now call DAFHLU and generate the exception.
    //
    spicelib::DAFHLU(HANDLE, &mut UNIT, ctx)?;

    //
    // Verify the exception.
    //
    testutil::CHCKXC(true, b"SPICE(HLULOCKFAILED)", OK, ctx)?;

    //
    // Check the value of UNIT.
    //
    testutil::CHCKSI(b"UNIT", UNIT, b"=", 0, 0, OK, ctx)?;

    //
    // Clean up.
    //
    spicelib::DAFCLS(HANDLE, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = ((UTSIZE - RSVUNT) - SCRUNT);
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::DAFCLS(HANLST[I], ctx)?;
            I += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"DAFLUH Exceptions", ctx)?;

    //
    // First, load up a DAS file and see what DAFLUH does
    // with it.
    //
    spicelib::ZZDDHOPN(&DASNAM, b"READ", b"DAS", &mut HANDLE, ctx)?;

    //
    // Fetch the logical unit assigned to DASNAM.
    //
    spicelib::ZZDDHHLU(HANDLE, b"DAS", false, &mut UNIT, ctx)?;

    //
    // Toss UNIT into DAFLUH and see what happens.
    //
    spicelib::DAFLUH(UNIT, &mut OUTHAN, ctx)?;

    //
    // We expect an error, check for it.
    //
    testutil::CHCKXC(true, b"SPICE(DAFNOSUCHUNIT)", OK, ctx)?;

    //
    // Check the value of OUTHAN, it should be 0.
    //
    testutil::CHCKSI(b"HANDLE", OUTHAN, b"=", 0, 0, OK, ctx)?;

    //
    // Clean up from this exception.
    //
    spicelib::ZZDDHCLS(HANDLE, b"DAS", false, ctx)?;

    //
    // Now we know HANDLE is no longer known to the handle
    // manager.  See what DAFLUH does with UNIT again.
    //
    spicelib::DAFLUH(UNIT, &mut OUTHAN, ctx)?;

    //
    // This is a little redundant, check for the same error
    // again.
    //
    testutil::CHCKXC(true, b"SPICE(DAFNOSUCHUNIT)", OK, ctx)?;

    //
    // Check the value of OUTHAN, it should be 0.
    //
    testutil::CHCKSI(b"HANDLE", OUTHAN, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"DAFHFN Nominal Execution", ctx)?;

    //
    // Setup inputs and outputs.
    //
    fstr::assign(&mut OUTNAM, b" ");

    //
    // Open a DAF with DAFOPR.
    //
    spicelib::REPMI(&FNMTMP, b"#", 1, &mut FNAME, ctx);

    spicelib::DAFOPR(&FNAME, &mut HANLST[1], ctx)?;

    //
    // See what file name is associated with HANDLE.
    //
    spicelib::DAFHFN(HANLST[1], &mut OUTNAM, ctx)?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the output argument.
    //
    testutil::CHCKSC(b"FNAME", &OUTNAM, b"=", &FNAME, OK, ctx)?;

    //
    // Clean up from this test case.
    //
    spicelib::DAFCLS(HANLST[1], ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"DAFHFN Exceptions", ctx)?;

    //
    // First, load up a DAS file and see what DAFHFN does
    // with it.
    //
    spicelib::ZZDDHOPN(&DASNAM, b"READ", b"DAS", &mut HANDLE, ctx)?;

    //
    // Set OUTNAM.
    //
    fstr::assign(&mut OUTNAM, b"NOT A FILENAME");

    //
    // Invoke the module.
    //
    spicelib::DAFHFN(HANDLE, &mut OUTNAM, ctx)?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(DAFNOSUCHHANDLE)", OK, ctx)?;

    //
    // Check the value of OUTNAM.
    //
    testutil::CHCKSC(b"FNAME", &OUTNAM, b"=", b"NOT A FILENAME", OK, ctx)?;

    //
    // Clean up.
    //
    spicelib::ZZDDHCLS(HANDLE, b"DAS", false, ctx)?;

    //
    // Now try passing HANDLE into DAFHFN again.
    //
    spicelib::DAFHFN(HANDLE, &mut OUTNAM, ctx)?;

    //
    // Check for the exception again.
    //
    testutil::CHCKXC(true, b"SPICE(DAFNOSUCHHANDLE)", OK, ctx)?;

    //
    // Check the value of OUTNAM.
    //
    testutil::CHCKSC(b"FNAME", &OUTNAM, b"=", b"NOT A FILENAME", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"DAFFNH Exceptions", ctx)?;

    //
    // Load the test DAS, as we are going to use it to exercise
    // exceptions in DAFFNH.
    //
    spicelib::ZZDDHOPN(&DASNAM, b"READ", b"DAS", &mut HANDLE, ctx)?;

    //
    // Set OUTHAN to something ridiculous.
    //
    OUTHAN = -1;

    //
    // Now pass DASNAM into DAFFNH and see what happens.
    //
    spicelib::DAFFNH(&DASNAM, &mut OUTHAN, ctx)?;

    //
    // We expect an exception, check for it.
    //
    testutil::CHCKXC(true, b"SPICE(DAFNOSUCHFILE)", OK, ctx)?;

    //
    // Check the value of OUTHAN.
    //
    testutil::CHCKSI(b"HANDLE", OUTHAN, b"=", 0, 0, OK, ctx)?;

    //
    // Now as with half the other test cases, unload the DAS and
    // try again.
    //
    spicelib::ZZDDHCLS(HANDLE, b"DAS", false, ctx)?;

    OUTHAN = -1;

    //
    // Invoke the module with again.
    //
    spicelib::DAFFNH(&DASNAM, &mut OUTHAN, ctx)?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(DAFNOSUCHFILE)", OK, ctx)?;

    //
    // Check the value of OUTHAN.
    //
    testutil::CHCKSI(b"HANDLE", OUTHAN, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"DAFSIH SPICE(DAFINVALIDACCESS) Exception", ctx)?;

    //
    // Open a DAF with DAFOPR.
    //
    spicelib::REPMI(&FNMTMP, b"#", 1, &mut FNAME, ctx);

    spicelib::DAFOPR(&FNAME, &mut HANLST[1], ctx)?;

    //
    // See what file name is associated with HANDLE.
    //
    spicelib::DAFSIH(HANLST[1], b"WRITE", ctx)?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(DAFINVALIDACCESS)", OK, ctx)?;

    //
    // Clean up from this test case.
    //
    spicelib::DAFCLS(HANLST[1], ctx)?;

    //
    // Clean up the files we used for this test family.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NUMDAF;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::REPMI(&FNMTMP, b"#", I, &mut FNAME, ctx);
            testutil::KILFIL(&FNAME, ctx)?;
            I += m3__;
        }
    }

    testutil::KILFIL(&DASNAM, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
