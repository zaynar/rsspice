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
const NUMCHR: i32 = 1024;
const NUMDP: i32 = 128;
const NUMINT: i32 = 256;

//$Procedure F_DAFNN ( DAF Non-Native Test Family )
pub fn F_DAFNN(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut FNAME = [b' '; FILEN as usize];
    let mut FNMTMP = [b' '; FILEN as usize];
    let mut NATIVE = [b' '; FILEN as usize];
    let mut CHARS = [b' '; NUMCHR as usize];
    let mut STRAMH = ActualCharArray::new(STRSIZ, 1..=NUMAMH);
    let mut STRARC = ActualCharArray::new(STRSIZ, 1..=NUMARC);
    let mut STRBFF = ActualCharArray::new(STRSIZ, 1..=NUMBFF);
    let mut DPS = StackArray::<f64, 128>::new(1..=NUMDP);
    let mut HANDLE: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut INTS = StackArray::<i32, 256>::new(1..=NUMINT);
    let mut NATBFF: i32 = 0;
    let mut NATI: i32 = 0;
    let mut NNBFF = StackArray::<i32, 4>::new(1..=NUMBFF);
    let mut NUMNN: i32 = 0;
    let mut NUMSUP: i32 = 0;
    let mut SUPBFF = StackArray::<i32, 4>::new(1..=NUMBFF);
    let mut NUMHAN: i32 = 0;
    let mut HANLST = StackArray::<i32, 4>::new(1..=NUMBFF);
    let mut UNIT: i32 = 0;
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
    testutil::TOPEN(b"F_DAFNN", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"F_DAFNN Initialization", ctx)?;

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
    // Check to see if this system supports multiple binary file formats.
    // If it does not, then just end the test family here.
    //
    if (NUMSUP == 1) {
        testutil::T_SUCCESS(OK, ctx);
        return Ok(());
    }

    //
    // Now locate the native file format in SUPBFF and construct the
    // list of non-native format codes.
    //
    NATI = 0;
    NUMNN = 0;

    for I in 1..=NUMSUP {
        if (SUPBFF[I] == NATBFF) {
            NATI = I;
        } else {
            NUMNN = (NUMNN + 1);
            NNBFF[NUMNN] = SUPBFF[I];
        }
    }

    //
    // Check the value of NATI, it should be non-zero.
    //
    testutil::CHCKSI(b"NATIVE_INDEX", NATI, b"!=", 0, 0, OK, ctx)?;

    //
    // Check the value of NUMNN, it should be at least 1.
    //
    testutil::CHCKSI(b"NUMBER_NN_BFFS", NUMNN, b">=", 1, 0, OK, ctx)?;

    //
    // Construct a native file first.
    //
    fstr::assign(&mut NATIVE, b"daf.daf");
    testutil::TSTSPK(&NATIVE, false, &mut HANDLE, ctx)?;

    //
    // Setup the non-native filename template.
    //
    fstr::assign(&mut FNMTMP, b"nndaf#.daf");

    //
    // Construct the DAFs for each non-native file format.
    //
    for I in 1..=NUMNN {
        spicelib::REPMI(&FNMTMP, b"#", I, &mut FNAME, ctx);

        //
        // Convert the native source DAF into the appropriate
        // format.
        //
        T_BINGO(&NATIVE, &FNAME, NNBFF[I], ctx)?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"DAFOPW on Native and Non-Native Formats", ctx)?;

    //
    // Attempt to open the native format for write, this should work.
    //
    HANDLE = 0;
    spicelib::DAFOPW(&NATIVE, &mut HANDLE, ctx)?;

    //
    // Check for the absence of errors.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"HANDLE", HANDLE, b"!=", 0, 0, OK, ctx)?;

    //
    // Unload the file.
    //
    spicelib::DAFCLS(HANDLE, ctx)?;

    //
    // Now attempt the same for each of the non-native files.
    //
    for I in 1..=NUMNN {
        spicelib::REPMI(&FNMTMP, b"#", I, &mut FNAME, ctx);
        spicelib::DAFOPW(&FNAME, &mut HANDLE, ctx)?;

        //
        // Check for the exception.
        //
        testutil::CHCKXC(true, b"SPICE(UNSUPPORTEDBFF)", OK, ctx)?;
        testutil::CHCKSI(b"HANDLE", HANDLE, b"=", 0, 0, OK, ctx)?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check DAF routines requiring write access", ctx)?;

    //
    // Clear the handle list.
    //
    spicelib::CLEARI(NUMBFF, HANLST.as_slice_mut());

    //
    // Open all the non-native files for read access.
    // This should work.
    //
    for I in 1..=NUMNN {
        spicelib::REPMI(&FNMTMP, b"#", I, &mut FNAME, ctx);
        spicelib::DAFOPR(&FNAME, &mut HANLST[I], ctx)?;

        //
        // Check for the absence of an exception.
        //
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(b"HANDLE", HANLST[I], b"!=", 0, 0, OK, ctx)?;
    }

    //
    // Now append the native file to the handle list.
    //
    NUMHAN = (NUMNN + 1);

    spicelib::DAFOPR(&NATIVE, &mut HANLST[NUMHAN], ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"HANDLE", HANLST[NUMHAN], b"!=", 0, 0, OK, ctx)?;

    //
    // Now loop over every file in the handle list, checking each
    // of the routines and entry points that require write access
    // for DAFs.
    //
    fstr::assign(&mut CHARS, b" ");
    spicelib::CLEARD(NUMDP, DPS.as_slice_mut());

    for I in 1..=NUMHAN {
        HANDLE = HANLST[I];

        spicelib::DAFBFS(HANDLE, ctx)?;
        spicelib::DAFFNA(&mut FOUND, ctx)?;

        testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

        spicelib::DAFRS(DPS.as_slice(), ctx)?;
        testutil::CHCKXC(true, b"SPICE(DAFINVALIDACCESS)", OK, ctx)?;

        spicelib::DAFRN(b"NONE", ctx)?;
        testutil::CHCKXC(true, b"SPICE(DAFINVALIDACCESS)", OK, ctx)?;

        spicelib::DAFWS(DPS.as_slice(), ctx)?;
        testutil::CHCKXC(true, b"SPICE(DAFILLEGWRITE)", OK, ctx)?;

        spicelib::DAFBNA(HANDLE, DPS.as_slice(), &CHARS, ctx)?;
        testutil::CHCKXC(true, b"SPICE(DAFINVALIDACCESS)", OK, ctx)?;

        spicelib::DAFCAD(HANDLE, ctx)?;
        testutil::CHCKXC(true, b"SPICE(DAFINVALIDACCESS)", OK, ctx)?;

        spicelib::DAFARR(HANDLE, 5, ctx)?;
        testutil::CHCKXC(true, b"SPICE(DAFINVALIDACCESS)", OK, ctx)?;

        INTS[1] = 1;
        INTS[2] = 3;
        INTS[3] = 2;

        spicelib::DAFRA(HANDLE, INTS.as_slice_mut(), 3, ctx)?;
        testutil::CHCKXC(true, b"SPICE(DAFILLEGWRITE)", OK, ctx)?;

        spicelib::DAFRRR(HANDLE, 5, ctx)?;
        testutil::CHCKXC(true, b"SPICE(DAFINVALIDACCESS)", OK, ctx)?;

        spicelib::DAFWDR(HANDLE, 2, DPS.as_slice(), ctx)?;
        testutil::CHCKXC(true, b"SPICE(DAFILLEGWRITE)", OK, ctx)?;

        spicelib::DAFWCR(HANDLE, 2, &CHARS, ctx)?;
        testutil::CHCKXC(true, b"SPICE(DAFINVALIDACCESS)", OK, ctx)?;

        //
        // The following routine signals different errors depending
        // on whether or not the HANDLE is associated with a native
        // or non-native file.
        //
        spicelib::DAFWDA(HANDLE, 128, 250, DPS.as_slice(), ctx)?;

        if (I != NUMHAN) {
            testutil::CHCKXC(true, b"SPICE(UNSUPPORTEDBFF)", OK, ctx)?;
        } else {
            testutil::CHCKXC(true, b"SPICE(DAFILLEGWRITE)", OK, ctx)?;
        }

        spicelib::DAFWFR(HANDLE, 2, 6, b"NONE", 2, 2, 1205, ctx)?;
        testutil::CHCKXC(true, b"SPICE(DAFINVALIDACCESS)", OK, ctx)?;
    }

    //
    // Close the files.
    //
    for I in 1..=NUMHAN {
        spicelib::DAFCLS(HANLST[I], ctx)?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check obsolete DAF routines requiring native", ctx)?;

    //
    // Clear the handle list.
    //
    spicelib::CLEARI(NUMBFF, HANLST.as_slice_mut());

    //
    // Open all the non-native files for read access.
    // This should work.
    //
    for I in 1..=NUMNN {
        spicelib::REPMI(&FNMTMP, b"#", I, &mut FNAME, ctx);
        spicelib::DAFOPR(&FNAME, &mut HANLST[I], ctx)?;

        //
        // Check for the absence of an exception.
        //
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(b"HANDLE", HANLST[I], b"!=", 0, 0, OK, ctx)?;
    }

    //
    // Now append the native file to the handle list.
    //
    NUMHAN = (NUMNN + 1);

    spicelib::DAFOPR(&NATIVE, &mut HANLST[NUMHAN], ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"HANDLE", HANLST[NUMHAN], b"!=", 0, 0, OK, ctx)?;

    //
    // Loop over all non-native files, checking for the appropriate
    // exceptions.
    //
    for I in 1..=NUMNN {
        HANDLE = HANLST[I];

        spicelib::DAFRDA(HANDLE, 128, 250, DPS.as_slice_mut(), ctx)?;
        testutil::CHCKXC(true, b"SPICE(UNSUPPORTEDBFF)", OK, ctx)?;

        spicelib::DAFRDR(HANDLE, 1, 1, 128, DPS.as_slice_mut(), &mut FOUND, ctx)?;
        testutil::CHCKXC(true, b"SPICE(UNSUPPORTEDBFF)", OK, ctx)?;
        testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
    }

    //
    // Verify that we can still use these routines on native files.
    //
    HANDLE = HANLST[NUMHAN];

    spicelib::DAFRDA(HANDLE, 385, 390, DPS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFRDR(HANDLE, 4, 2, 10, DPS.as_slice_mut(), &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    //
    // Close the files.
    //
    for I in 1..=NUMHAN {
        spicelib::DAFCLS(HANLST[I], ctx)?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check native NULL BFFID File Records Load", ctx)?;

    //
    // Note: This test modifies the file record of the file
    // referenced by the name NATIVE.  If you need to add tests,
    // add them before this one.
    //

    spicelib::GETLUN(&mut UNIT, ctx)?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(UNIT),
            file: Some(fstr::substr(&NATIVE, 1..=spicelib::RTRIM(&NATIVE))),
            access: Some(b"DIRECT"),
            recl: Some(RECL),
            status: Some(b"OLD"),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
    }

    //
    // Check IOSTAT.
    //
    testutil::CHCKSI(b"OPEN IOSTAT", IOSTAT, b"=", 0, 0, OK, ctx)?;

    //
    // Read the file record into the CHARS buffer.
    //
    {
        use f2rust_std::{
            data::Val,
            io::{self, Reader},
        };

        let mut reader = io::UnformattedReader::new(ctx.io_unit(UNIT)?, Some(1))?;
        IOSTAT = io::capture_iostat(|| {
            reader.start()?;
            reader.read_str(&mut CHARS)?;
            reader.finish()?;
            Ok(())
        })?;
    }

    //
    // Check IOSTAT.
    //
    testutil::CHCKSI(b"READ IOSTAT", IOSTAT, b"=", 0, 0, OK, ctx)?;

    //
    // Replace the BFF ID with nulls.
    //
    for I in 89..=96 {
        fstr::assign(fstr::substr_mut(&mut CHARS, I..=I), &intrinsics::CHAR(0));
    }

    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::UnformattedWriter::new(ctx.io_unit(UNIT)?, Some(1))?;
        IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            writer.write_str(&CHARS)?;
            writer.finish()?;
            Ok(())
        })?;
    }

    //
    // Check IOSTAT.
    //
    testutil::CHCKSI(b"WRITE IOSTAT", IOSTAT, b"=", 0, 0, OK, ctx)?;

    //
    // Close the file.
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
    // Check for any exceptions; there should be none.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Attempt to open the file with DAFOPR.
    //
    spicelib::DAFOPR(&NATIVE, &mut HANDLE, ctx)?;

    //
    // Check for an exception; again we expect none.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the file and try DAFOPW.
    //
    spicelib::DAFCLS(HANDLE, ctx)?;

    //
    // Now try DAFOPW.
    //
    spicelib::DAFOPW(&NATIVE, &mut HANDLE, ctx)?;

    //
    // Check for any exceptions; again we expect none.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the file.
    //
    spicelib::DAFCLS(HANDLE, ctx)?;

    //
    // Clean up by removing the test files.
    //
    for I in 1..=NUMNN {
        spicelib::REPMI(&FNMTMP, b"#", I, &mut FNAME, ctx);
        testutil::KILFIL(&FNAME, ctx)?;
    }

    testutil::KILFIL(&NATIVE, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
