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

//$Procedure F_ZZDGFR ( ZZDAFGFR Test Family )
pub fn F_ZZDGFR(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut CASENM = [b' '; 80 as usize];
    let mut FNAME = [b' '; FILEN as usize];
    let mut FNMTMP = [b' '; FILEN as usize];
    let mut IDWORD = [b' '; 8 as usize];
    let mut IFNAME = [b' '; 60 as usize];
    let mut RDIFN = [b' '; 60 as usize];
    let mut RDIDW = [b' '; 8 as usize];
    let mut STRAMH = ActualCharArray::new(STRSIZ, 1..=NUMAMH);
    let mut STRARC = ActualCharArray::new(STRSIZ, 1..=NUMARC);
    let mut STRBFF = ActualCharArray::new(STRSIZ, 1..=NUMBFF);
    let mut BWARD: i32 = 0;
    let mut FREE: i32 = 0;
    let mut FWARD: i32 = 0;
    let mut HANLST = StackArray::<i32, 4>::new(1..=NUMBFF);
    let mut NATBFF: i32 = 0;
    let mut ND: i32 = 0;
    let mut NI: i32 = 0;
    let mut NUMSUP: i32 = 0;
    let mut RDBWD: i32 = 0;
    let mut RDFRE: i32 = 0;
    let mut RDFWD: i32 = 0;
    let mut RDND: i32 = 0;
    let mut RDNI: i32 = 0;
    let mut SUPBFF = StackArray::<i32, 4>::new(1..=NUMBFF);
    let mut UNIT: i32 = 0;
    let mut ADDFTP: bool = false;
    let mut FOUND: bool = false;

    //
    // Local Variables
    //

    //
    // Start the test family with an open call.
    //
    testutil::TOPEN(b"F_ZZDGFR", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"F_ZZDGFR Initialization", ctx)?;

    //
    // Retrieve the native format and other related information.
    //
    spicelib::ZZDDHINI(
        &mut NATBFF,
        SUPBFF.as_slice_mut(),
        &mut NUMSUP,
        STRAMH.as_arg_mut(),
        STRARC.as_arg_mut(),
        STRBFF.as_arg_mut(),
        ctx,
    )?;

    //
    // Setup the filename template.
    //
    fstr::assign(&mut FNMTMP, b"daf#.daf");

    //
    // Construct the contents of the DAF to create.
    //
    fstr::assign(&mut IDWORD, b"DAF/TEST");
    ND = 12;
    NI = 16;
    fstr::assign(
        &mut IFNAME,
        b"TSPICE Test DAF Striving for 60 Characters Going Once Twice.",
    );
    FWARD = -4;
    BWARD = 12;
    FREE = ((4 * 128) + 1);
    ADDFTP = true;

    //
    // Construct the DAFs for each environment and load them
    // into the handle manager.
    //
    for I in 1..=NUMSUP {
        spicelib::REPMI(&FNMTMP, b"#", I, &mut FNAME, ctx);

        //
        // Start a new DAF.
        //
        T_DAFOPN(&FNAME, SUPBFF[I], &mut UNIT, ctx)?;

        //
        // Dump the file record to it.
        //
        T_DAFWFR(
            UNIT, SUPBFF[I], &IDWORD, ND, NI, &IFNAME, FWARD, BWARD, FREE, ADDFTP, ctx,
        )?;

        //
        // Close the file.  Since we added the FTP string, ZZDDHOPN
        // only requires a file record to open the file.
        //
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(UNIT),
                ..Default::default()
            };
            ctx.close(specs)?;
        }

        //
        // Open the DAF in the handle manager.
        //
        spicelib::ZZDDHOPN(&FNAME, b"READ", b"DAF", &mut HANLST[I], ctx)?;

        //
        // Check for the absence of an exception.
        //
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Dynamically construct the test cases based on the contents
    // of SUPBFF.
    //
    for I in 1..=NUMSUP {
        fstr::assign(&mut CASENM, b"# reading # file record data.");
        spicelib::REPMC(&CASENM.clone(), b"#", &STRBFF[NATBFF], &mut CASENM);
        spicelib::REPMC(&CASENM.clone(), b"#", &STRBFF[SUPBFF[I]], &mut CASENM);

        testutil::TCASE(&CASENM, ctx)?;

        //
        // Setup the inputs and outputs.
        //
        RDND = 0;
        RDNI = 0;
        fstr::assign(&mut RDIDW, b" ");
        fstr::assign(&mut RDIFN, b" ");
        RDFWD = 0;
        RDBWD = 0;
        RDFRE = 0;

        FOUND = false;

        //
        // We have all the pieces, invoke the module.
        //
        spicelib::ZZDAFGFR(
            HANLST[I], &mut RDIDW, &mut RDND, &mut RDNI, &mut RDIFN, &mut RDFWD, &mut RDBWD,
            &mut RDFRE, &mut FOUND, ctx,
        )?;

        //
        // Check for the absence of an exception.
        //
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check outputs.
        //
        testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

        testutil::CHCKSC(b"IDWORD", &RDIDW, b"=", &IDWORD, OK, ctx)?;

        testutil::CHCKSI(b"ND", RDND, b"=", ND, 0, OK, ctx)?;
        testutil::CHCKSI(b"NI", RDNI, b"=", NI, 0, OK, ctx)?;

        testutil::CHCKSC(b"IFNAME", &RDIFN, b"=", &IFNAME, OK, ctx)?;

        testutil::CHCKSI(b"FWARD", RDFWD, b"=", FWARD, 0, OK, ctx)?;
        testutil::CHCKSI(b"BWARD", RDBWD, b"=", BWARD, 0, OK, ctx)?;
        testutil::CHCKSI(b"FREE", RDFRE, b"=", FREE, 0, OK, ctx)?;
    }

    //
    // Clean up by unloading and removing the test files.
    //
    for I in 1..=NUMSUP {
        spicelib::ZZDDHCLS(HANLST[I], b"DAF", false, ctx)?;
        spicelib::REPMI(&FNMTMP, b"#", I, &mut FNAME, ctx);
        testutil::KILFIL(&FNAME, ctx)?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPICE(HANDLENOTFOUND) Exception", ctx)?;

    //
    // Setup outputs.
    //
    FOUND = true;

    fstr::assign(&mut RDIDW, b" ");
    RDND = 0;
    RDNI = 0;
    fstr::assign(&mut RDIFN, b" ");
    RDFWD = 0;
    RDBWD = 0;
    RDFRE = 0;

    //
    // Since we know we just unloaded HANLST(1), attempt to read
    // from that handle.
    //
    spicelib::ZZDAFGFR(
        HANLST[1], &mut RDIDW, &mut RDND, &mut RDNI, &mut RDIFN, &mut RDFWD, &mut RDBWD,
        &mut RDFRE, &mut FOUND, ctx,
    )?;

    //
    // Check for the presence of an exception.
    //
    testutil::CHCKXC(true, b"SPICE(HANDLENOTFOUND)", OK, ctx)?;

    //
    // Check outputs. FOUND should be FALSE, and RDREC should be
    // untouched.
    //
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    testutil::CHCKSC(b"IDWORD", &RDIDW, b"=", b" ", OK, ctx)?;

    testutil::CHCKSI(b"ND", RDND, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"NI", RDNI, b"=", 0, 0, OK, ctx)?;

    testutil::CHCKSC(b"IFNAME", &RDIFN, b"=", b" ", OK, ctx)?;

    testutil::CHCKSI(b"FWARD", RDFWD, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"BWARD", RDBWD, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"FREE", RDFRE, b"=", 0, 0, OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
