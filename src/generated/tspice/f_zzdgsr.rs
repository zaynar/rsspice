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

//$Procedure F_ZED'S ( ZZDAFGSR Test Family )
pub fn F_ZZDGSR(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut CASENM = [b' '; 80 as usize];
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
    let mut PAARY = StackArray::<f64, 128>::new(1..=128);
    let mut RDREC = StackArray::<f64, 128>::new(1..=128);
    let mut TDC = StackArray::<f64, 125>::new(1..=125);
    let mut BWARD: i32 = 0;
    let mut FREE: i32 = 0;
    let mut FWARD: i32 = 0;
    let mut HANLST = StackArray::<i32, 4>::new(1..=NUMBFF);
    let mut IC = StackArray::<i32, 250>::new(1..=250);
    let mut MXNSUM: i32 = 0;
    let mut NATBFF: i32 = 0;
    let mut NEXT: i32 = 0;
    let mut NSUM: i32 = 0;
    let mut NUMSUP: i32 = 0;
    let mut PREV: i32 = 0;
    let mut SUMSIZ: i32 = 0;
    let mut SUPBFF = StackArray::<i32, 4>::new(1..=NUMBFF);
    let mut TIC = StackArray::<i32, 250>::new(1..=250);
    let mut UNIT: i32 = 0;
    let mut ADDFTP: bool = false;
    let mut FOUND: bool = false;

    //
    // Local Variables
    //

    //
    // Start the test family with an open call.
    //
    testutil::TOPEN(b"F_ZZDGSR", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"F_ZZDGSR Initialization", ctx)?;

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
    // Construct the contents of the DAF to create.  These values
    // are just to satisfy the arguments of T_DAFWFR, as we are only
    // creating a test file to exercise ZZDAFGSR.
    //
    fstr::assign(&mut IDWORD, b"DAF/TEST");
    fstr::assign(&mut IFNAME, b"TSPICE Test DAF");
    FWARD = 2;
    BWARD = 2;
    FREE = ((128 * 125) + 1);
    ADDFTP = true;

    //
    // Construct the DAFs for each supported architecture, load them
    // into the handle manager and perform the necessary tests.
    //
    for I in 1..=NUMSUP {
        spicelib::REPMI(&FNMTMP, b"#", I, &mut FNAME, ctx);

        //
        // Now loop over all possible combinations of ND and NI.
        // J ranges over the possible values of ND, while K over NI.
        //
        //  DO J = 0, 124
        //     DO K = 2, 2*(125-J)
        //
        // In the interest of increasing execution speed on most of
        // the test platforms only perform the following, commonly
        // used summary tests.
        //
        for J in 2..=2 {
            for K in 5..=6 {
                //
                // Declare a new test case, since we are constructing them
                // dynamically.
                //
                fstr::assign(&mut CASENM, b"# reading # data. ND = #, NI = #.");
                spicelib::REPMC(&CASENM.clone(), b"#", &STRBFF[NATBFF], &mut CASENM);
                spicelib::REPMC(&CASENM.clone(), b"#", &STRBFF[SUPBFF[I]], &mut CASENM);
                spicelib::REPMI(&CASENM.clone(), b"#", J, &mut CASENM, ctx);
                spicelib::REPMI(&CASENM.clone(), b"#", K, &mut CASENM, ctx);

                testutil::TCASE(&CASENM, ctx)?;

                //
                // Open the new DAF.
                //
                T_DAFOPN(&FNAME, SUPBFF[I], &mut UNIT, ctx)?;

                //
                // Dump the semi-bogus file record into the new DAF.
                //
                T_DAFWFR(
                    UNIT, SUPBFF[I], &IDWORD, J, K, &IFNAME, FWARD, BWARD, FREE, ADDFTP, ctx,
                )?;

                //
                // Compute the maximum number of summaries each summary
                // record can hold for these values of ND and NI.
                //
                MXNSUM = (125 / (J + ((K + 1) / 2)));
                SUMSIZ = (J + ((K + 1) / 2));

                //
                // Now construct the summaries we are going to store
                // in each summary record.  We will saturate ARRAY
                // with a sequence of packed summaries.
                //
                for L in 1..=MXNSUM {
                    //
                    // Construct the DP components.
                    //
                    for M in 1..=J {
                        DC[M] = ((L as f64) * (M as f64));
                    }

                    //
                    // Construct the integer components
                    //
                    for M in 1..=K {
                        IC[M] = (L * M);
                    }

                    spicelib::DAFPS(
                        J,
                        K,
                        DC.as_slice(),
                        IC.as_slice(),
                        ARRAY.subarray_mut((((L - 1) * SUMSIZ) + 1)),
                    );
                }

                //
                // Now construct each summary record that is to be
                // written into the DAF.  The first L summaries from
                // ARRAY will be copied into the target record along
                // with bogus values for NEXT, PREV, and L itself as
                // NSUM.  The remainder of the record is zeroed out
                // for the safety of the reverse conversion module's
                // sake.
                //
                for L in 0..=MXNSUM {
                    //
                    // We are just going to assign bogus values to NEXT and
                    // PREV, because we are not really going to be reading
                    // this with any of the high level DAF routines.
                    //
                    NEXT = 2;
                    PREV = 2;
                    NSUM = L;

                    //
                    // We have the baseline ARRAY of MXNSUM packed
                    // summaries.  Start populating records, by copying
                    // the appropriate number of packed summaries into
                    // PAARY.
                    //
                    spicelib::CLEARD(128, PAARY.as_slice_mut());
                    spicelib::MOVED(ARRAY.as_slice(), (L * SUMSIZ), PAARY.as_slice_mut());

                    //
                    // Write the summary record to the new DAF.
                    //
                    T_DAFWSR(
                        UNIT,
                        (L + 2),
                        SUPBFF[I],
                        J,
                        K,
                        NEXT,
                        PREV,
                        NSUM,
                        PAARY.as_slice(),
                        ctx,
                    )?;
                }

                //
                // Close the DAF.
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

                //
                // Now check each record in the file we just created.
                //
                for L in 0..=MXNSUM {
                    //
                    // Setup the inputs and outputs.
                    //
                    spicelib::CLEARI(250, TIC.as_slice_mut());
                    spicelib::CLEARD(125, TDC.as_slice_mut());

                    FOUND = false;

                    //
                    // Invoke the module.
                    //
                    spicelib::ZZDAFGSR(
                        HANLST[I],
                        (L + 2),
                        J,
                        K,
                        DATA.as_slice_mut(),
                        &mut FOUND,
                        ctx,
                    )?;

                    //
                    // Check for the absence of an exception.
                    //
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Now check the results. Start by examining the
                    // values stored in the record for NEXT, PREV,
                    // and NSUM.
                    //
                    testutil::CHCKSD(b"NEXT", DATA[1], b"=", (NEXT as f64), 0.0, OK, ctx)?;
                    testutil::CHCKSD(b"PREV", DATA[2], b"=", (PREV as f64), 0.0, OK, ctx)?;
                    testutil::CHCKSD(b"NSUM", DATA[3], b"=", (L as f64), 0.0, OK, ctx)?;

                    //
                    // Now verify the contents of each summary.
                    //
                    for M in 1..=L {
                        //
                        // Unpack the summary for the test array that was
                        // written to the file.
                        //
                        spicelib::DAFUS(
                            ARRAY.subarray((((M - 1) * SUMSIZ) + 1)),
                            J,
                            K,
                            DC.as_slice_mut(),
                            IC.as_slice_mut(),
                        );

                        //
                        // Unpack the next summary from DATA.
                        //
                        spicelib::DAFUS(
                            DATA.subarray((4 + ((M - 1) * SUMSIZ))),
                            J,
                            K,
                            TDC.as_slice_mut(),
                            TIC.as_slice_mut(),
                        );

                        //
                        // Compare the contents of the summaries.
                        //
                        testutil::CHCKAD(
                            b"DC",
                            TDC.as_slice(),
                            b"=",
                            DC.as_slice(),
                            J,
                            0.0,
                            OK,
                            ctx,
                        )?;
                        testutil::CHCKAI(b"IC", TIC.as_slice(), b"=", IC.as_slice(), K, OK, ctx)?;
                    }
                }

                //
                // Close the file, removing it from the handle manager.
                //
                spicelib::ZZDDHCLS(HANLST[I], b"DAF", false, ctx)?;

                //
                // Kill the file, so we can reuse the name safely.
                //
                testutil::KILFIL(&FNAME, ctx)?;
            }
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SPICE(HANDLENOTFOUND) Exception", ctx)?;

    //
    // Setup outputs and expected values.
    //
    FOUND = true;

    spicelib::CLEARD(128, DATA.as_slice_mut());
    spicelib::CLEARD(128, RDREC.as_slice_mut());

    //
    // Since we know we just unloaded HANLST(NUMSUP), attempt to read
    // from that handle.
    //
    spicelib::ZZDAFGSR(
        HANLST[NUMSUP],
        2,
        124,
        2,
        RDREC.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    //
    // Check for the presence of the exception.
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
