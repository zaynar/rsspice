//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const RECLEN: i32 = 32;
const SHRT: i32 = (RECLEN / 2);

//$Procedure  F_ZZASC1 ( Family of tests for ZZASCII )
pub fn F_ZZASC1(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut FAIL = StackArray::<i32, 2>::new(1..=2);
    let mut NATIVE: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut NUMBER: i32 = 0;
    let mut REC: i32 = 0;
    let mut LINE = ActualCharArray::new(RECLEN, 1..=3);
    let mut FILES = ActualCharArray::new(12, 1..=3);
    let mut TERMIN = ActualCharArray::new(2, 1..=3);
    let mut EXPECT = ActualCharArray::new(5, 1..=3);
    let mut OUTTRM = [b' '; 5 as usize];
    let mut WORD = [b' '; 5 as usize];
    let mut WRKLIN = [b' '; 50 as usize];
    let mut SHRTLN = [b' '; SHRT as usize];
    let mut XSHRTL = [b' '; 2 as usize];

    //
    // Write ascii text containing line terminators
    // for each platform, Macintosh classic, Unix,
    // and DOS.
    //

    //
    // SPICELIB functions.
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_ZZASC1", ctx)?;

    //
    // Define the data to write to test files. ID the
    // file as an LSK kernel, with a begin text block - thus
    // we don't risk damaging any kernel pool variable
    // assignments. The third line contains the data to write.
    // Thirty-two characters written forty times.
    //
    fstr::assign(LINE.get_mut(1), b"KPL/LSK                         ");
    fstr::assign(LINE.get_mut(2), b"\\begintext                      ");
    fstr::assign(LINE.get_mut(3), b"12345678901234567890123456789012");

    //
    // Initialize the file names and the corresponding line
    // terminators.
    //
    fstr::assign(FILES.get_mut(1), b"mac.ker");
    fstr::assign(FILES.get_mut(2), b"unix.ker");
    fstr::assign(FILES.get_mut(3), b"dos.ker");

    fstr::assign(
        fstr::substr_mut(TERMIN.get_mut(1), 1..=2),
        &intrinsics::CHAR(13),
    );
    fstr::assign(
        fstr::substr_mut(TERMIN.get_mut(2), 1..=2),
        &intrinsics::CHAR(10),
    );
    fstr::assign(
        fstr::substr_mut(TERMIN.get_mut(3), 1..=2),
        &fstr::concat(&intrinsics::CHAR(13), &intrinsics::CHAR(10)),
    );

    fstr::assign(EXPECT.get_mut(1), b"CR");
    fstr::assign(EXPECT.get_mut(2), b"LF");
    fstr::assign(EXPECT.get_mut(3), b"CR-LF");

    //
    // Initialize the REC marker value. Start from the
    // beginning of the file.
    //
    REC = 1;

    //
    // Loop over the three terminator types. Create
    // a test file with an appropriate name for the terminator
    // character.
    //
    for I in 1..=3 {
        //
        // If the test file exists, delete it.
        //
        if spicelib::EXISTS(&FILES[I], ctx)? {
            testutil::KILFIL(&FILES[I], ctx)?;
        }

        //
        // Find a free logical unit for file access.
        //
        spicelib::GETLUN(&mut NUMBER, ctx)?;

        //
        // Open the file as direct access thus suppressing the print
        // of a native line terminator.
        //
        {
            use f2rust_std::io;

            let specs = io::OpenSpecs {
                unit: Some(NUMBER),
                file: Some(FILES.get(I)),
                access: Some(b"DIRECT"),
                status: Some(b"NEW"),
                recl: Some(RECLEN),
                ..Default::default()
            };
            IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
        }

        //
        // Add the terminator to the first line.
        //
        fstr::assign(fstr::substr_mut(LINE.get_mut(1), 31..=32), TERMIN.get(I));

        //
        // Write the line to record REC then increment REC.
        //
        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::UnformattedWriter::new(ctx.io_unit(NUMBER)?, Some(REC))?;
            writer.start()?;
            writer.write_str(fstr::substr(LINE.get(1), 1..=32))?;
            writer.finish()?;
        }
        REC = (REC + 1);

        //
        // Add the terminator to the second line.
        //

        fstr::assign(fstr::substr_mut(LINE.get_mut(2), 31..=32), TERMIN.get(I));

        //
        // Write the line to record REC then increment REC.
        //
        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::UnformattedWriter::new(ctx.io_unit(NUMBER)?, Some(REC))?;
            writer.start()?;
            writer.write_str(fstr::substr(LINE.get(2), 1..=32))?;
            writer.finish()?;
        }
        REC = (REC + 1);

        //
        // Add the terminator to the third data line. We
        // write this line 40 times to create a kernel file
        // larger than 1032 (the record size read by GETFAT).
        //
        fstr::assign(fstr::substr_mut(LINE.get_mut(3), 31..=32), TERMIN.get(I));

        //
        // Write the line, incrementing the record counter each
        // pass.
        //
        for J in 1..=40 {
            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Writer},
                };

                let mut writer = io::UnformattedWriter::new(ctx.io_unit(NUMBER)?, Some(REC))?;
                writer.start()?;
                writer.write_str(fstr::substr(LINE.get(3), 1..=32))?;
                writer.finish()?;
            }
            REC = (REC + 1);
        }

        //
        // Finished. Close this file, reset the REC marker for the
        // next file.
        //
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(NUMBER),
                ..Default::default()
            };
            ctx.close(specs)?;
        }
        REC = 1;
    }

    //
    // Case 1
    //
    testutil::TCASE(b"Check for correct terminator ID", ctx)?;

    //
    // Start tests. Check the correct response for each file.
    // Note, the length of WRKLIN must exceed the declared length of
    // the LINE variable for the test to succeed.
    //
    for I in 1..=3 {
        spicelib::ZZASCII(&FILES[I], &mut WRKLIN, false, &mut OUTTRM, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSC(b"Terminator", &OUTTRM, b"=", &EXPECT[I], OK, ctx)?;
    }

    //
    // Case 2
    //
    testutil::TCASE(b"Check for failure for short line", ctx)?;

    //
    // Test that a scan fails when using a line length shorter than
    // the line length of the first line in the file of interest.
    // In this case we know the test kernels have line
    // lengths of thirty-two.
    //
    for I in 1..=3 {
        spicelib::ZZASCII(&FILES[I], &mut SHRTLN, false, &mut OUTTRM, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSC(b"Terminator", &OUTTRM, b"=", b"?", OK, ctx)?;
    }

    //
    // Retrieve the proper line terminator for the platform.
    //
    spicelib::ZZPLATFM(b"TEXT_FORMAT", &mut WORD, ctx);

    //
    // Set the index for the native terminator.
    //
    if spicelib::EQSTR(&WORD, &EXPECT[1]) {
        NATIVE = 1;
        FAIL[1] = 2;
        FAIL[2] = 3;
    } else if spicelib::EQSTR(&WORD, &EXPECT[2]) {
        NATIVE = 2;
        FAIL[1] = 1;
        FAIL[2] = 3;
    } else if spicelib::EQSTR(&WORD, &EXPECT[3]) {
        NATIVE = 3;
        FAIL[1] = 1;
        FAIL[2] = 2;
    } else {
        //
        // BAD BUG! This block should never execute.
        //
        testutil::CHCKXC(true, b"UNKNOWNLINETERMINATOR", OK, ctx)?;

        //
        // Clean-up. Delete files.
        //
        for I in 1..=3 {
            testutil::KILFIL(&FILES[I], ctx)?;
        }

        //
        // Close out the test family.
        //
        testutil::T_SUCCESS(OK, ctx);
        return Ok(());
    }

    //
    // Case 3
    //
    testutil::TCASE(b"Check for correct FURNSH response, native.", ctx)?;

    //
    // Check a native file passes through a FURNSH call without
    // an error signal.
    //
    spicelib::FURNSH(&FILES[NATIVE], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Unload the kernel so we don't mess-up another kernel
    // test family.
    //
    spicelib::UNLOAD(&FILES[NATIVE], ctx)?;

    //
    // Case 4
    //
    testutil::TCASE(b"Check for correct FURNSH response, non-native.", ctx)?;

    //
    // Check a non-native file causes an error signal when used in a
    // FURNSH call.
    //
    spicelib::FURNSH(&FILES[FAIL[1]], ctx)?;
    testutil::CHCKXC(true, b"SPICE(INCOMPATIBLEEOL)", OK, ctx)?;

    spicelib::FURNSH(&FILES[FAIL[2]], ctx)?;
    testutil::CHCKXC(true, b"SPICE(INCOMPATIBLEEOL)", OK, ctx)?;

    //
    // Case 5
    //
    testutil::TCASE(b"Check for ZZASCII short work line signal.", ctx)?;

    //
    // Ensure ZZASCII signals an error for work line lengths
    // below the 3 character threshold.
    //
    spicelib::ZZASCII(&FILES[NATIVE], &mut XSHRTL, false, &mut OUTTRM, ctx)?;
    testutil::CHCKXC(true, b"SPICE(STRINGTOOSHORT)", OK, ctx)?;

    //
    // Clean-up. Delete files.
    //
    for I in 1..=3 {
        testutil::KILFIL(&FILES[I], ctx)?;
    }

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
