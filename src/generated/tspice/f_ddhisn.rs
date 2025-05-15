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

//$Procedure F_DDHISN ( ZZDDHISN Test Family )
pub fn F_DDHISN(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ARCH = [b' '; 20 as usize];
    let mut FNAME = [b' '; FILEN as usize];
    let mut FNMPAT = [b' '; FILEN as usize];
    let mut METHOD = [b' '; 20 as usize];
    let mut STRNAT = [b' '; STRSIZ as usize];
    let mut STRNOW = [b' '; STRSIZ as usize];
    let mut RDSBFF = [b' '; STRLEN as usize];
    let mut HANDLE = StackArray::<i32, 4>::new(1..=NUMBFF);
    let mut I: i32 = 0;
    let mut NATFID: i32 = 0;
    let mut NUMFIL: i32 = 0;
    let mut FOUND: bool = false;
    let mut NATIVE: bool = false;

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Start the test family with an open call.
    //
    testutil::TOPEN(b"F_DDHISN", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"F_DDHISN Initialization", ctx)?;

    //
    // Get the native binary file format for this platform.
    //
    spicelib::ZZPLATFM(b"FILE_FORMAT", &mut STRNAT, ctx);
    spicelib::UCASE(&STRNAT.clone(), &mut STRNAT, ctx);

    //
    // Get the list of supported binary file formats for this platform.
    //
    spicelib::ZZPLATFM(b"READS_BFF", &mut RDSBFF, ctx);
    spicelib::UCASE(&RDSBFF.clone(), &mut RDSBFF, ctx);

    //
    // Set the filename pattern we will use.
    //
    fstr::assign(&mut FNMPAT, b"test#.fil");

    //
    // Now create a file of each supported type.  We are just going to
    // trick the system into thinking it's an actual file of this type.
    //
    I = 1;
    spicelib::NEXTWD(&RDSBFF.clone(), &mut STRNOW, &mut RDSBFF);

    while fstr::ne(&STRNOW, b" ") {
        //
        // Build the name of the file we intend to use.
        //
        spicelib::REPMI(&FNMPAT, b"#", I, &mut FNAME, ctx);

        //
        // Check to see if this is the native format.
        //
        if spicelib::EQSTR(&STRNOW, &STRNAT) {
            NATFID = I;
        }

        //
        // Create the file.
        //
        T_CPTFIL(
            &FNAME,
            DAF,
            2,
            &STRNOW,
            b"ABCD",
            b"EFGH",
            b"IJKL",
            true,
            false,
            b"DAF/SPK ",
            ctx,
        )?;

        //
        // Open the file into the handle manager.
        //
        fstr::assign(&mut METHOD, b"READ");
        fstr::assign(&mut ARCH, b"DAF");

        spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE[I], ctx)?;

        //
        // Strip off the next word from RDSBFF and increment I.
        //
        spicelib::NEXTWD(&RDSBFF.clone(), &mut STRNOW, &mut RDSBFF);
        I = (I + 1);
    }

    //
    // Store the number of files we placed into the handle list.
    //
    NUMFIL = (I - 1);

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now that we have finished all of the initialization stuff,
    // break the tests into two cases.  NATIVE files and NON-NATIVE
    // files.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Exercise Native File Logic", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    NATIVE = false;
    FOUND = false;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHISN(HANDLE[NATFID], &mut NATIVE, &mut FOUND, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check outputs.
    //
    testutil::CHCKSL(b"NATIVE", NATIVE, true, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Exercise Non-Native File Logic", ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NUMFIL;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // First check to see that this file is not of the native
            // format.
            //
            if (I != NATFID) {
                //
                // Setup the inputs and outputs.
                //
                NATIVE = true;
                FOUND = false;

                //
                // Invoke the module.
                //
                spicelib::ZZDDHISN(HANDLE[I], &mut NATIVE, &mut FOUND, ctx)?;

                //
                // Check for the absence of an exception.
                //
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Check outputs.
                //
                testutil::CHCKSL(b"NATIVE", NATIVE, false, OK, ctx)?;
                testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
            }

            I += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Exercise Unknown Handle Exception Logic", ctx)?;

    //
    // Setup inputs and outputs for the zero-valued handle test.
    //
    NATIVE = false;
    FOUND = true;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHISN(0, &mut NATIVE, &mut FOUND, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the outputs.  Since FOUND is FALSE, NATIVE should remain
    // unchanged.
    //
    testutil::CHCKSL(b"NATIVE", NATIVE, false, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    //
    // Setup the inputs and outputs for the opposite-sign handle test.
    //
    NATIVE = true;
    FOUND = false;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHISN(-HANDLE[1], &mut NATIVE, &mut FOUND, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check outputs.
    //
    testutil::CHCKSL(b"NATIVE", NATIVE, true, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    //
    // ----------------------------------------------------------------
    //
    //     Now clean up.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NUMFIL;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::ZZDDHCLS(HANDLE[I], &ARCH, false, ctx)?;
            spicelib::REPMI(&FNMPAT, b"#", I, &mut FNAME, ctx);
            testutil::KILFIL(&FNAME, ctx)?;
            I += m3__;
        }
    }

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
