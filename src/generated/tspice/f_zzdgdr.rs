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

//$Procedure F_ZZDGDR ( ZZDAFGDR Test Family )
pub fn F_ZZDGDR(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut CASENM = [b' '; 80 as usize];
    let mut NAME = [b' '; 1024 as usize];
    let mut FNAME = [b' '; FILEN as usize];
    let mut FNMTMP = [b' '; FILEN as usize];
    let mut IDWORD = [b' '; 8 as usize];
    let mut IFNAME = [b' '; 60 as usize];
    let mut STRAMH = ActualCharArray::new(STRSIZ, 1..=NUMAMH);
    let mut STRARC = ActualCharArray::new(STRSIZ, 1..=NUMARC);
    let mut STRBFF = ActualCharArray::new(STRSIZ, 1..=NUMBFF);
    let mut ARRAY = StackArray::<f64, 128>::new(1..=128);
    let mut DATA = StackArray::<f64, 128>::new(1..=128);
    let mut DC = StackArray::<f64, 125>::new(1..=125);
    let mut RDREC = StackArray::<f64, 128>::new(1..=128);
    let mut BWARD: i32 = 0;
    let mut FREE: i32 = 0;
    let mut FWARD: i32 = 0;
    let mut HANLST = StackArray::<i32, 4>::new(1..=NUMBFF);
    let mut IC = StackArray::<i32, 250>::new(1..=250);
    let mut IOSTAT: i32 = 0;
    let mut NATBFF: i32 = 0;
    let mut ND: i32 = 0;
    let mut NEXT: i32 = 0;
    let mut NI: i32 = 0;
    let mut NSUM: i32 = 0;
    let mut NUMSUP: i32 = 0;
    let mut PREV: i32 = 0;
    let mut SUPBFF = StackArray::<i32, 4>::new(1..=NUMBFF);
    let mut UNIT: i32 = 0;
    let mut ADDFTP: bool = false;
    let mut FOUND: bool = false;

    //
    // SPICELIB Functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Start the test family with an open call.
    //
    testutil::TOPEN(b"F_ZZDGDR", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"F_ZZDGDR Initialization", ctx)?;

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
    ND = 2;
    NI = 6;
    fstr::assign(&mut IFNAME, b"TSPICE Test DAF");
    FWARD = 2;
    BWARD = 2;
    FREE = ((4 * 128) + 1);
    ADDFTP = false;
    NEXT = 0;
    PREV = 0;
    NSUM = 2;

    //
    // Create and pack the summaries.
    //
    DC[1] = 1.0;
    DC[2] = -1.0;

    IC[1] = 1;
    IC[2] = 2;
    IC[3] = 3;
    IC[4] = 4;
    IC[5] = ((3 * 128) + 1);
    IC[6] = (IC[5] + 64);

    spicelib::DAFPS(ND, NI, DC.as_slice(), IC.as_slice(), ARRAY.as_slice_mut());

    DC[1] = 2.0;
    DC[2] = -2.0;

    IC[1] = 2;
    IC[2] = 4;
    IC[3] = 6;
    IC[4] = 8;
    IC[5] = (IC[6] + 1);
    IC[6] = (4 * 128);

    spicelib::DAFPS(
        ND,
        NI,
        DC.as_slice(),
        IC.as_slice(),
        ARRAY.subarray_mut(((ND + ((NI + 1) / 2)) + 1)),
    );

    //
    // Setup the name record.
    //
    fstr::assign(&mut NAME, b" ");

    //
    // Populate the data record with a simple pattern.  Our
    // aim is not to validate the translation algorithm, but
    // the reader module.
    //
    for I in 1..=128 {
        DATA[I] = f64::powi(10.0, I);
    }

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
        // Now write the only summary record to the file.
        //
        T_DAFWSR(
            UNIT,
            2,
            SUPBFF[I],
            ND,
            NI,
            NEXT,
            PREV,
            NSUM,
            ARRAY.as_slice(),
            ctx,
        )?;

        //
        // Insert a blank name record for completeness.
        //
        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::UnformattedWriter::new(ctx.io_unit(UNIT)?, Some(3))?;
            IOSTAT = io::capture_iostat(|| {
                writer.start()?;
                writer.write_str(&NAME)?;
                writer.finish()?;
                Ok(())
            })?;
        }
        testutil::CHCKSI(b"IOSTAT", IOSTAT, b"=", 0, 0, OK, ctx)?;

        //
        // Lastly, dump the data record and close the file.
        //
        T_DAFWDR(UNIT, 4, SUPBFF[I], 128, DATA.as_slice(), ctx)?;
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
        fstr::assign(&mut CASENM, b"# reading # data.");
        spicelib::REPMC(&CASENM.clone(), b"#", &STRBFF[NATBFF], &mut CASENM);
        spicelib::REPMC(&CASENM.clone(), b"#", &STRBFF[SUPBFF[I]], &mut CASENM);

        testutil::TCASE(&CASENM, ctx)?;

        //
        // Setup the inputs and outputs.
        //
        for J in 1..=128 {
            RDREC[J] = 0.0;
        }

        FOUND = false;

        //
        // We have all the pieces, invoke the module.
        //
        spicelib::ZZDAFGDR(HANLST[I], 4, RDREC.as_slice_mut(), &mut FOUND, ctx)?;

        //
        // Check for the absence of an exception.
        //
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check outputs.
        //
        testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

        testutil::CHCKAD(
            b"RDREC",
            RDREC.as_slice(),
            b"=",
            DATA.as_slice(),
            128,
            0.0,
            OK,
            ctx,
        )?;
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

    for I in 1..=128 {
        RDREC[I] = DATA[I];
    }

    //
    // Since we know we just unloaded HANLST(1), attempt to read
    // from that handle.
    //
    spicelib::ZZDAFGDR(HANLST[1], 4, RDREC.as_slice_mut(), &mut FOUND, ctx)?;

    //
    // Check for the presence of an exception.
    //
    testutil::CHCKXC(true, b"SPICE(HANDLENOTFOUND)", OK, ctx)?;

    //
    // Check outputs. FOUND should be FALSE, and RDREC should be
    // untouched.
    //
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    testutil::CHCKAD(
        b"RDREC",
        RDREC.as_slice(),
        b"=",
        DATA.as_slice(),
        128,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
