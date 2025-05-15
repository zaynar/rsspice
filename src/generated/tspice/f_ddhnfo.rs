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
const NUMFIL: i32 = (NUMARC * (NUMAMH + NUMBFF));

//$Procedure F_DDHNFO ( ZZDDHNFO Test Family )
pub fn F_DDHNFO(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut TBLFNM = ActualCharArray::new(FILEN, 1..=NUMFIL);
    let mut TBLAMH = StackArray::<i32, 16>::new(1..=NUMFIL);
    let mut TBLARC = StackArray::<i32, 16>::new(1..=NUMFIL);
    let mut TBLBFF = StackArray::<i32, 16>::new(1..=NUMFIL);
    let mut TBLHAN = StackArray::<i32, 16>::new(1..=NUMFIL);
    let mut FNAME = [b' '; FILEN as usize];
    let mut FNMPAT = [b' '; FILEN as usize];
    let mut STRAMH = ActualCharArray::new(STRSIZ, 1..=NUMAMH);
    let mut STRARC = ActualCharArray::new(STRSIZ, 1..=NUMARC);
    let mut STRBFF = ActualCharArray::new(STRSIZ, 1..=NUMBFF);
    let mut TCSTR = [b' '; 80 as usize];
    let mut TSTSTR = [b' '; FILEN as usize];
    let mut FILCNT: i32 = 0;
    let mut I: i32 = 0;
    let mut INTAMH: i32 = 0;
    let mut INTARC: i32 = 0;
    let mut INTBFF: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut NATBFF: i32 = 0;
    let mut NUMSUP: i32 = 0;
    let mut SUPBFF = StackArray::<i32, 2>::new(1..=NUMARC);
    let mut UNIT: i32 = 0;
    let mut FOUND: bool = false;

    //
    // Local Parameters
    //
    // NUMFIL is the maximum number of different possible types of files
    // that this system may open in the ZZDDHMAN interface.  It is
    // computed based on the following:
    //
    //    There are NUMARC architectures.  Each method is valid for
    //    either architecture.
    //
    //    All non-native files may only be opened for 'READ' access.
    //

    //
    // Test Table
    //

    //
    // Local Variables
    //

    //
    // Start the test family with an open call.
    //
    testutil::TOPEN(b"F_DDHNFO", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"F_DDHNFO Initialization", ctx)?;

    //
    // Set the filename pattern we will use.
    //
    fstr::assign(&mut FNMPAT, b"test#.fil");

    //
    // Fetch some initialization data.
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
    // Now create entries in the test table for each of the native
    // binary file format files of each architecture and access method.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = (NUMARC * NUMAMH);
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::REPMI(&FNMPAT, b"#", I, &mut TBLFNM[I], ctx);
            TBLARC[I] = (intrinsics::MOD(I, NUMARC) + 1);
            TBLAMH[I] = (((I - 1) / NUMARC) + 1);
            TBLBFF[I] = NATBFF;

            I += m3__;
        }
    }

    //
    // Create entries in the test table for each of the non-native
    // binary file format files of each architecture with 'READ' access.
    //
    I = (NUMARC * NUMAMH);

    for J in 1..=NUMSUP {
        //
        // We are going to ignore the native binary file format, since
        // we addressed it in the preceding loop.
        //
        if (SUPBFF[J] != NATBFF) {
            //
            // Loop over every possible architecture, preparing the
            // list of files for 'READ' access.
            //
            for K in 1..=NUMARC {
                I = (I + 1);

                spicelib::REPMI(&FNMPAT, b"#", I, &mut TBLFNM[I], ctx);
                TBLARC[I] = (intrinsics::MOD(I, NUMARC) + 1);
                TBLAMH[I] = READ;
                TBLBFF[I] = SUPBFF[J];
            }
        }
    }

    //
    // Store the number of files we placed into the handle list.
    //
    FILCNT = I;

    //
    // Now create the files.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = FILCNT;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Only create files for access methods that are not 'NEW'
            // or 'SCRATCH'.
            //
            if ((TBLAMH[I] != NEW) && (TBLAMH[I] != SCRTCH)) {
                //
                // Create the file.
                //
                T_CPTFIL(
                    &TBLFNM[I],
                    TBLARC[I],
                    2,
                    &STRBFF[TBLBFF[I]],
                    b"ABCD",
                    b"EFGH",
                    b"IJKL",
                    true,
                    false,
                    b" ",
                    ctx,
                )?;
            }

            //
            // Open the file into the handle manager.
            //
            spicelib::ZZDDHOPN(
                &TBLFNM[I],
                &STRAMH[TBLAMH[I]],
                &STRARC[TBLARC[I]],
                &mut TBLHAN[I],
                ctx,
            )?;

            I += m3__;
        }
    }

    //
    // Now that we have finished all of the initialization stuff,
    // Perform the checks on each of the files in the test table.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = FILCNT;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Treat each file as an individual case.  Generate the
            // TCASE string automatically from the data in the table.
            //
            fstr::assign(&mut TCSTR, b"Exercise - # # # Nominal Logic");

            spicelib::REPMC(&TCSTR.clone(), b"#", &STRAMH[TBLAMH[I]], &mut TCSTR);
            spicelib::REPMC(&TCSTR.clone(), b"#", &STRBFF[TBLBFF[I]], &mut TCSTR);
            spicelib::REPMC(&TCSTR.clone(), b"#", &STRARC[TBLARC[I]], &mut TCSTR);

            testutil::TCASE(&TCSTR, ctx)?;

            //
            // Now setup the inputs and outputs.
            //
            fstr::assign(&mut FNAME, b" ");
            INTARC = 0;
            INTBFF = 0;
            INTAMH = 0;
            FOUND = false;

            //
            // Invoke the module.
            //
            spicelib::ZZDDHNFO(
                TBLHAN[I],
                &mut FNAME,
                &mut INTARC,
                &mut INTBFF,
                &mut INTAMH,
                &mut FOUND,
                ctx,
            )?;

            //
            // Check for the absence of an exception.
            //
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Check outputs.
            //
            testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

            //
            // Handle the case of scratch filenames separately.
            // Since their names are constructed automatically.
            //
            if (TBLAMH[I] == SCRTCH) {
                //
                // INQUIRE on the UNIT associated with the scratch
                // handle.
                //
                spicelib::ZZDDHHLU(TBLHAN[I], &STRARC[TBLARC[I]], false, &mut UNIT, ctx)?;

                //
                // Set the default value of the filename, in case the
                // INQUIRE does not change it.
                //
                fstr::assign(&mut TSTSTR, b"# SCRATCH FILE");
                spicelib::REPMC(&TSTSTR.clone(), b"#", &STRARC[TBLARC[I]], &mut TSTSTR);

                {
                    use f2rust_std::io;

                    let specs = io::InquireSpecs {
                        unit: Some(UNIT),
                        name: Some(&mut TSTSTR),
                        ..Default::default()
                    };
                    IOSTAT = io::capture_iostat(|| ctx.inquire(specs))?;
                }

                //
                // Check the IOSTAT from the INQUIRE.
                //
                testutil::CHCKSI(b"IOSTAT", IOSTAT, b"=", 0, 0, OK, ctx)?;
            } else {
                fstr::assign(&mut TSTSTR, TBLFNM.get(I));
            }

            testutil::CHCKSC(b"FNAME", &FNAME, b"=", &TSTSTR, OK, ctx)?;

            testutil::CHCKSI(b"INTARC", INTARC, b"=", TBLARC[I], 0, OK, ctx)?;
            testutil::CHCKSI(b"INTBFF", INTBFF, b"=", TBLBFF[I], 0, OK, ctx)?;
            testutil::CHCKSI(b"INTAMH", INTAMH, b"=", TBLAMH[I], 0, OK, ctx)?;

            I += m3__;
        }
    }

    //
    // Check for the absence of a rogue exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Exercise Handle Not Found Exception Logic", ctx)?;

    //
    // Setup inputs and outputs.
    //
    fstr::assign(&mut FNAME, b"BOGUS FILENAME");
    INTARC = 1;
    INTBFF = 1;
    INTAMH = 1;
    FOUND = true;

    //
    // Invoke the module on the zero-valued handle.
    //
    spicelib::ZZDDHNFO(
        0,
        &mut FNAME,
        &mut INTARC,
        &mut INTBFF,
        &mut INTAMH,
        &mut FOUND,
        ctx,
    )?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check outputs.
    //
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    testutil::CHCKSC(b"FNAME", &FNAME, b"=", b" ", OK, ctx)?;

    testutil::CHCKSI(b"INTARC", INTARC, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"INTBFF", INTBFF, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"INTAMH", INTAMH, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Exercise Handle Opposite Sign Exception Logic", ctx)?;

    //
    // Setup inputs and outputs.
    //
    fstr::assign(&mut FNAME, b"BOGUS FILENAME");
    INTARC = 1;
    INTBFF = 1;
    INTAMH = 1;
    FOUND = true;

    //
    // Invoke the module on the negative of an existing, in-use handle.
    //
    spicelib::ZZDDHNFO(
        -TBLHAN[1],
        &mut FNAME,
        &mut INTARC,
        &mut INTBFF,
        &mut INTAMH,
        &mut FOUND,
        ctx,
    )?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check outputs.
    //
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    testutil::CHCKSC(b"FNAME", &FNAME, b"=", b" ", OK, ctx)?;

    testutil::CHCKSI(b"INTARC", INTARC, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"INTBFF", INTBFF, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"INTAMH", INTAMH, b"=", 0, 0, OK, ctx)?;

    //
    // ----------------------------------------------------------------
    //
    //     Now clean up.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = FILCNT;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::ZZDDHCLS(TBLHAN[I], &STRARC[TBLARC[I]], false, ctx)?;
            testutil::KILFIL(&TBLFNM[I], ctx)?;
            I += m3__;
        }
    }

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
