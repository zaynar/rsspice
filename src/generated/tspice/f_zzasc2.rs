//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNGLEN: i32 = 128;
const MIDLEN: i32 = (LNGLEN / 2);
const RECLEN: i32 = (LNGLEN / 2);
const NFILES: i32 = 12;
const EOLTSZ: i32 = 5;

//$Procedure  F_ZZASC2 ( Family of tests for ZZASCII )
pub fn F_ZZASC2(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut IOSTAT: i32 = 0;
    let mut NUMBER: i32 = 0;
    let mut LINE = ActualCharArray::new(MIDLEN, 1..=3);
    let mut TERMIN = ActualCharArray::new(2, 1..=3);
    let mut HLINE = [b' '; MIDLEN as usize];
    let mut FILES = ActualCharArray::new(12, 1..=NFILES);
    let mut MYFILE = [b' '; 12 as usize];
    let mut OUTTRM = [b' '; EOLTSZ as usize];
    let mut WRKLIN = [b' '; LNGLEN as usize];
    let mut XSHRTL = [b' '; 2 as usize];
    let mut MYZZEF = StackArray::<bool, 12>::new(1..=NFILES);
    let mut MYZZEM = ActualCharArray::new(32, 1..=NFILES);
    let mut MYTERM = ActualCharArray::new(EOLTSZ, 1..=NFILES);
    let mut MYFNEF = StackArray::<bool, 12>::new(1..=NFILES);
    let mut MYFNEM = ActualCharArray::new(32, 1..=NFILES);

    //
    // Local parameters
    //

    //
    // Local Variables.
    //

    //
    // SPICELIB functions.
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_ZZASC2", ctx)?;

    //
    // Define the data to write to test files. ID the file as an LSK
    // kernel, with a begin text block - thus we don't risk damaging any
    // kernel pool variable assignments. The third line contains the
    // data to write.
    //
    fstr::assign(
        LINE.get_mut(1),
        b"KPL/LSK                                                         ",
    );
    fstr::assign(
        LINE.get_mut(2),
        b"\\begintext                                                      ",
    );
    fstr::assign(
        LINE.get_mut(3),
        b"1234567890123456789012345678901234567890123456789012345678901234",
    );

    //
    // Initialize the file names. The first three files will be smaller
    // than the buffer that will be used to read them. The second three
    // will have exactly same size as the buffer. The third three will
    // be larger than the buffer. The last three will be larger than the
    // buffer but will contain mixed terminators.
    //
    fstr::assign(FILES.get_mut(1), b"macs.ker");
    fstr::assign(FILES.get_mut(2), b"unxs.ker");
    fstr::assign(FILES.get_mut(3), b"doss.ker");
    fstr::assign(FILES.get_mut(4), b"macm.ker");
    fstr::assign(FILES.get_mut(5), b"unxm.ker");
    fstr::assign(FILES.get_mut(6), b"dosm.ker");
    fstr::assign(FILES.get_mut(7), b"macl.ker");
    fstr::assign(FILES.get_mut(8), b"unxl.ker");
    fstr::assign(FILES.get_mut(9), b"dosl.ker");
    fstr::assign(FILES.get_mut(10), b"mxd1.ker");
    fstr::assign(FILES.get_mut(11), b"mxd2.ker");
    fstr::assign(FILES.get_mut(12), b"mxd3.ker");

    //
    // Initialize terminators and tags.
    //
    fstr::assign(
        TERMIN.get_mut(1),
        &fstr::concat(&intrinsics::CHAR(13), b" "),
    );
    fstr::assign(
        TERMIN.get_mut(2),
        &fstr::concat(&intrinsics::CHAR(10), b" "),
    );
    fstr::assign(
        TERMIN.get_mut(3),
        &fstr::concat(&intrinsics::CHAR(13), &intrinsics::CHAR(10)),
    );

    //
    // Generate test files. The first loop ("I" loop) controls how many
    // records will be written to the test files (up to three, the 4th
    // is for the mixed files.)
    //
    for I in 1..=4 {
        //
        // The second loop ("J" loop) goes over triplets of files and
        // corresponding terminators.
        //
        for J in 1..=3 {
            //
            // If the test file exists, delete it.
            //
            fstr::assign(&mut MYFILE, FILES.get((((I - 1) * 3) + J)));

            if spicelib::EXISTS(&MYFILE, ctx)? {
                testutil::KILFIL(&MYFILE, ctx)?;
            }

            //
            // Find a free logical unit and open the file as direct access.
            //
            spicelib::GETLUN(&mut NUMBER, ctx)?;

            {
                use f2rust_std::io;

                let specs = io::OpenSpecs {
                    unit: Some(NUMBER),
                    file: Some(&MYFILE),
                    access: Some(b"DIRECT"),
                    status: Some(b"NEW"),
                    recl: Some(RECLEN),
                    ..Default::default()
                };
                IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
            }

            //
            // The third loop ("REC" loop) pads "I" lines with the right
            // terminators (for the first three file triplets) or with
            // mixed terminators (for the last triplet) and writes them to
            // the file.
            //
            for REC in 1..=I {
                //
                // Copy line to temporary buffer, add the terminator and
                // write it to the file. If REC is 4 or greater, use
                // incorrect terminator to get all flavors of
                // "contaminated" files.
                //
                if (REC <= 3) {
                    fstr::assign(&mut HLINE, LINE.get(REC));
                    fstr::assign(
                        fstr::substr_mut(&mut HLINE, (MIDLEN - 1)..=MIDLEN),
                        TERMIN.get(J),
                    );
                    {
                        use f2rust_std::{
                            data::Val,
                            io::{self, Writer},
                        };

                        let mut writer =
                            io::UnformattedWriter::new(ctx.io_unit(NUMBER)?, Some(REC))?;
                        writer.start()?;
                        writer.write_str(fstr::substr(&HLINE, 1..=MIDLEN))?;
                        writer.finish()?;
                    }
                } else {
                    fstr::assign(&mut HLINE, LINE.get(1));
                    if (J == 1) {
                        fstr::assign(
                            fstr::substr_mut(&mut HLINE, (MIDLEN - 1)..=MIDLEN),
                            TERMIN.get(2),
                        );
                    } else if (J == 2) {
                        fstr::assign(
                            fstr::substr_mut(&mut HLINE, (MIDLEN - 1)..=MIDLEN),
                            TERMIN.get(3),
                        );
                    } else if (J == 3) {
                        fstr::assign(
                            fstr::substr_mut(&mut HLINE, (MIDLEN - 1)..=MIDLEN),
                            TERMIN.get(1),
                        );
                    }
                    {
                        use f2rust_std::{
                            data::Val,
                            io::{self, Writer},
                        };

                        let mut writer = io::UnformattedWriter::new(ctx.io_unit(NUMBER)?, Some(1))?;
                        writer.start()?;
                        writer.write_str(fstr::substr(&HLINE, 1..=MIDLEN))?;
                        writer.finish()?;
                    }
                }
            }

            //
            // Close the file.
            //
            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(NUMBER),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }
        }
    }

    //
    // Ensure ZZASCII signals an error for work line lengths below the 3
    // character threshold.
    //

    testutil::TCASE(b"Check for error when using tiny buffer.", ctx)?;
    spicelib::ZZASCII(&FILES[1], &mut XSHRTL, false, &mut OUTTRM, ctx)?;
    testutil::CHCKXC(true, b"SPICE(STRINGTOOSHORT)", OK, ctx)?;

    //
    // Ensure ZZASCII signals an error for a file that cannot be opened.
    //

    testutil::TCASE(b"Check for error when cannot open the file.", ctx)?;
    spicelib::ZZASCII(b"hakuna-matata", &mut WRKLIN, false, &mut OUTTRM, ctx)?;
    testutil::CHCKXC(true, b"SPICE(FILEOPENFAIL)", OK, ctx)?;

    //
    // Set ZZASCII exception flag and exception message for each file.
    // Since all files exist and buffer is big enough there should not
    // be any exceptions on any of the platforms.
    //
    for I in 1..=NFILES {
        MYZZEF[I] = false;
        fstr::assign(MYZZEM.get_mut(I), b" ");
    }

    //
    // Set ZZASCII output terminator for each file. These are a
    // different story. We assign them as they should be detected
    // everywhere, but reassign some/all for the platforms that don't
    // "behave."
    //
    fstr::assign(MYTERM.get_mut(1), b"?");
    fstr::assign(MYTERM.get_mut(2), b"?");
    fstr::assign(MYTERM.get_mut(3), b"?");
    fstr::assign(MYTERM.get_mut(4), b"CR");
    fstr::assign(MYTERM.get_mut(5), b"LF");
    fstr::assign(MYTERM.get_mut(6), b"CR-LF");
    fstr::assign(MYTERM.get_mut(7), b"CR");
    fstr::assign(MYTERM.get_mut(8), b"LF");
    fstr::assign(MYTERM.get_mut(9), b"CR-LF");
    fstr::assign(MYTERM.get_mut(10), b"?");
    fstr::assign(MYTERM.get_mut(11), b"?");
    fstr::assign(MYTERM.get_mut(12), b"?");

    //
    // The IFORT, GFORTRAN, DIGITAL, and LAHEY seem to
    // generate IOSTAT=0 when read past the end of a file and,
    // therefore, proceed and detect terminators in short files.
    //

    fstr::assign(MYTERM.get_mut(1), b"CR");
    fstr::assign(MYTERM.get_mut(2), b"LF");
    fstr::assign(MYTERM.get_mut(3), b"CR-LF");

    //
    // Set FURNSH exception flag, exception message and loaded flag for
    // each file as if FURNSH wouldn't fail and then reset some of them
    // depending on the platform.
    //
    for I in 1..=NFILES {
        MYFNEF[I] = false;
        fstr::assign(MYFNEM.get_mut(I), b" ");
    }

    //
    // IFORT on Macs and Linux and GFORTRAN everywhere can detect
    // terminators in small and medium files and, therefore, complain
    // about them same way as for the large files.
    //
    MYFNEF[1] = true;
    MYFNEF[3] = true;
    MYFNEF[4] = true;
    MYFNEF[6] = true;
    MYFNEF[7] = true;
    MYFNEF[9] = true;
    fstr::assign(MYFNEM.get_mut(1), b"SPICE(INCOMPATIBLEEOL)");
    fstr::assign(MYFNEM.get_mut(3), b"SPICE(INCOMPATIBLEEOL)");
    fstr::assign(MYFNEM.get_mut(4), b"SPICE(INCOMPATIBLEEOL)");
    fstr::assign(MYFNEM.get_mut(6), b"SPICE(INCOMPATIBLEEOL)");
    fstr::assign(MYFNEM.get_mut(7), b"SPICE(INCOMPATIBLEEOL)");
    fstr::assign(MYFNEM.get_mut(9), b"SPICE(INCOMPATIBLEEOL)");

    //
    // Now go over the files and call ZZASCII and FURNSH on each of them.
    //
    for I in 1..=NFILES {
        //
        // Direct check by calling ZZASCII.
        //
        testutil::TCASE(
            &fstr::concat(b"Check for EOL terminator in ", FILES.get(I)),
            ctx,
        )?;
        spicelib::ZZASCII(&FILES[I], &mut WRKLIN, false, &mut OUTTRM, ctx)?;
        testutil::CHCKXC(MYZZEF[I], &MYZZEM[I], OK, ctx)?;
        testutil::CHCKSC(b"Terminator", &OUTTRM, b"=", &MYTERM[I], OK, ctx)?;

        //
        // Indirect check by calling FURNSH.
        //
        testutil::TCASE(
            &fstr::concat(b"Check what FURNSH does with ", FILES.get(I)),
            ctx,
        )?;

        spicelib::FURNSH(&FILES[I], ctx)?;
        testutil::CHCKXC(MYFNEF[I], &MYFNEM[I], OK, ctx)?;

        if !MYFNEF[I] {
            spicelib::UNLOAD(&FILES[I], ctx)?;
        }
    }

    //
    // Clean-up: delete test files.
    //
    for I in 1..=12 {
        testutil::KILFIL(&FILES[I], ctx)?;
    }

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
