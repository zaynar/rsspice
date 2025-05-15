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

//$Procedure F_DDHOPN ( ZZDDHOPN Test Family )
pub fn F_DDHOPN(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ALTARC = [b' '; 20 as usize];
    let mut ALTFNM = [b' '; FILEN as usize];
    let mut ALTMTH = [b' '; 20 as usize];
    let mut ARCH = [b' '; 20 as usize];
    let mut BINFMT = [b' '; 20 as usize];
    let mut IDWARY = ActualCharArray::new(8, 1..=NUMARC);
    let mut FNAME = [b' '; FILEN as usize];
    let mut FNMPAT = [b' '; FILEN as usize];
    let mut METHOD = [b' '; 20 as usize];
    let mut STRAMH = ActualCharArray::new(STRSIZ, 1..=NUMAMH);
    let mut STRARC = ActualCharArray::new(STRSIZ, 1..=NUMARC);
    let mut STRBFF = ActualCharArray::new(STRSIZ, 1..=NUMBFF);
    let mut ALTHAN: i32 = 0;
    let mut ALTUNT: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut HANLCK = StackArray::<i32, 21>::new(1..=(UTSIZE - RSVUNT));
    let mut HANTST = ActualArray::<i32>::new(1..=FTSIZE);
    let mut IOSTAT: i32 = 0;
    let mut NATBFF: i32 = 0;
    let mut NUMSUP: i32 = 0;
    let mut SUPBFF = StackArray::<i32, 4>::new(1..=NUMBFF);
    let mut UNIT: i32 = 0;
    let mut EXISTS: bool = false;
    let mut OPENED: bool = false;

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Start the test family with an open call.
    //
    testutil::TOPEN(b"F_DDHOPN", ctx)?;

    //
    // Setup the filename pattern for loop tests.
    //
    fstr::assign(&mut FNMPAT, b"test#.fil");

    //
    // Retrieve the native binary file format string to use
    // when creating test files.
    //
    spicelib::ZZPLATFM(b"FILE_FORMAT", &mut BINFMT, ctx);

    //
    // Retrieve other initialization data.
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
    // Setup IDWARY, an array that contains an acceptable file ID word
    // for each supported architecture.
    //
    for I in 1..=NUMARC {
        spicelib::REPMC(b"#/TEST", b"#", &STRARC[I], &mut IDWARY[I]);
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Unsupported Access Method Exception.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"FAILURE-TEST");
    fstr::assign(&mut ARCH, b"DAF");
    HANDLE = 1;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(UNSUPPORTEDMETHOD)", OK, ctx)?;

    //
    // Check HANDLE.
    //
    testutil::CHCKSI(b"HANDLE", HANDLE, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid File Architecture Exception.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"NEW");
    fstr::assign(&mut ARCH, b"FAILURE-TEST");
    HANDLE = 1;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(UNSUPPORTEDARCH)", OK, ctx)?;

    //
    // Check HANDLE.
    //
    testutil::CHCKSI(b"HANDLE", HANDLE, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No free lockable units on SCRATCH Open Exception.", ctx)?;

    //
    // Setup the inputs and outputs.  The easiest way to do this is
    // open (UTSIZE-RSVUNT) scratch files in a loop.
    //
    for I in 1..=(UTSIZE - RSVUNT) {
        spicelib::ZZDDHOPN(b" ", b"SCRATCH", b"DAS", &mut HANLCK[I], ctx)?;
    }

    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"SCRATCH");
    fstr::assign(&mut ARCH, b"DAS");
    HANDLE = 1;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(UTFULL)", OK, ctx)?;

    //
    // Check HANDLE.
    //
    testutil::CHCKSI(b"HANDLE", HANDLE, b"=", 0, 0, OK, ctx)?;

    //
    // Clean up the locked files.
    //
    for I in 1..=(UTSIZE - RSVUNT) {
        spicelib::ZZDDHCLS(HANLCK[I], b"DAS", false, ctx)?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"READ Blank Filename Exception.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    fstr::assign(&mut FNAME, b" ");
    fstr::assign(&mut METHOD, b"READ");
    fstr::assign(&mut ARCH, b"DAF");
    HANDLE = 1;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(BLANKFILENAME)", OK, ctx)?;

    //
    // Check HANDLE.
    //
    testutil::CHCKSI(b"HANDLE", HANDLE, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"NEW Blank Filename Exception.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    fstr::assign(&mut FNAME, b" ");
    fstr::assign(&mut METHOD, b"NEW");
    fstr::assign(&mut ARCH, b"DAF");
    HANDLE = 1;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(BLANKFILENAME)", OK, ctx)?;

    //
    // Check HANDLE.
    //
    testutil::CHCKSI(b"HANDLE", HANDLE, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"WRITE Blank Filename Exception.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    fstr::assign(&mut FNAME, b" ");
    fstr::assign(&mut METHOD, b"WRITE");
    fstr::assign(&mut ARCH, b"DAF");
    HANDLE = 1;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(BLANKFILENAME)", OK, ctx)?;

    //
    // Check HANDLE.
    //
    testutil::CHCKSI(b"HANDLE", HANDLE, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"READ Non-Existent File Exception.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"READ");
    fstr::assign(&mut ARCH, b"DAF");
    HANDLE = 1;

    testutil::KILFIL(&FNAME, ctx)?;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(FILENOTFOUND)", OK, ctx)?;

    //
    // Check HANDLE.
    //
    testutil::CHCKSI(b"HANDLE", HANDLE, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"WRITE Non-Existent File Exception.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"WRITE");
    fstr::assign(&mut ARCH, b"DAF");
    HANDLE = 1;

    testutil::KILFIL(&FNAME, ctx)?;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(FILENOTFOUND)", OK, ctx)?;

    //
    // Check HANDLE.
    //
    testutil::CHCKSI(b"HANDLE", HANDLE, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"READ File Already Connected Exception.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"READ");
    fstr::assign(&mut ARCH, b"DAF");
    HANDLE = 1;

    testutil::KILFIL(&FNAME, ctx)?;

    //
    // Create FNAME and attach it to a logical unit.
    //
    T_CPTFIL(
        &FNAME,
        DAF,
        2,
        &BINFMT,
        b"ABCD",
        b"EFGH",
        b"IJKL",
        true,
        false,
        b"DAF/SPK ",
        ctx,
    )?;

    spicelib::GETLUN(&mut UNIT, ctx)?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(UNIT),
            file: Some(&FNAME),
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
    testutil::CHCKSI(b"IOSTAT", IOSTAT, b"=", 0, 0, OK, ctx)?;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(IMPROPEROPEN)", OK, ctx)?;

    //
    // Check HANDLE.
    //
    testutil::CHCKSI(b"HANDLE", HANDLE, b"=", 0, 0, OK, ctx)?;

    //
    // Remove the file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"WRITE File Already Connected Exception.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"WRITE");
    fstr::assign(&mut ARCH, b"DAF");
    HANDLE = 1;

    testutil::KILFIL(&FNAME, ctx)?;

    //
    // Create FNAME and attach it to a logical unit.
    //
    T_CPTFIL(
        &FNAME,
        DAF,
        2,
        &BINFMT,
        b"ABCD",
        b"EFGH",
        b"IJKL",
        true,
        false,
        b"DAF/SPK ",
        ctx,
    )?;

    spicelib::GETLUN(&mut UNIT, ctx)?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(UNIT),
            file: Some(&FNAME),
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
    testutil::CHCKSI(b"IOSTAT", IOSTAT, b"=", 0, 0, OK, ctx)?;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(IMPROPEROPEN)", OK, ctx)?;

    //
    // Check HANDLE.
    //
    testutil::CHCKSI(b"HANDLE", HANDLE, b"=", 0, 0, OK, ctx)?;

    //
    // Remove the file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"READ DAF Architecture Mismatch Exception.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"READ");
    fstr::assign(&mut ARCH, b"DAF");
    HANDLE = 1;

    testutil::KILFIL(&FNAME, ctx)?;

    fstr::assign(&mut ALTFNM, &FNAME);
    fstr::assign(&mut ALTMTH, &METHOD);
    fstr::assign(&mut ALTARC, b"DAS");
    ALTHAN = 0;

    //
    // Create FNAME.
    //
    T_CPTFIL(
        &ALTFNM,
        DAS,
        2,
        &BINFMT,
        b"ABCD",
        b"EFGH",
        b"IJKL",
        true,
        false,
        b"DAS/EK  ",
        ctx,
    )?;

    //
    // Open the file as a DAS to prepare for the conflict.
    //
    spicelib::ZZDDHOPN(&ALTFNM, &ALTMTH, &ALTARC, &mut ALTHAN, ctx)?;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(FILARCMISMATCH)", OK, ctx)?;

    //
    // Check HANDLE.
    //
    testutil::CHCKSI(b"HANDLE", HANDLE, b"=", 0, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    spicelib::ZZDDHCLS(ALTHAN, &ALTARC, false, ctx)?;

    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"WRITE DAF Architecture Mismatch Exception.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"WRITE");
    fstr::assign(&mut ARCH, b"DAF");
    HANDLE = 1;

    testutil::KILFIL(&FNAME, ctx)?;

    fstr::assign(&mut ALTFNM, &FNAME);
    fstr::assign(&mut ALTMTH, &METHOD);
    fstr::assign(&mut ALTARC, b"DAS");
    ALTHAN = 0;

    //
    // Create FNAME.
    //
    T_CPTFIL(
        &FNAME,
        DAS,
        2,
        &BINFMT,
        b"ABCD",
        b"EFGH",
        b"IJKL",
        true,
        false,
        b"DAS/EK  ",
        ctx,
    )?;

    //
    // Open the file as a DAS to prepare for the conflict.
    //
    spicelib::ZZDDHOPN(&ALTFNM, &ALTMTH, &ALTARC, &mut ALTHAN, ctx)?;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(FILARCMISMATCH)", OK, ctx)?;

    //
    // Check HANDLE.
    //
    testutil::CHCKSI(b"HANDLE", HANDLE, b"=", 0, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    spicelib::ZZDDHCLS(ALTHAN, &ALTARC, false, ctx)?;

    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"READ DAS Architecture Mismatch Exception.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"READ");
    fstr::assign(&mut ARCH, b"DAS");
    HANDLE = 1;

    testutil::KILFIL(&FNAME, ctx)?;

    fstr::assign(&mut ALTFNM, &FNAME);
    fstr::assign(&mut ALTMTH, &METHOD);
    fstr::assign(&mut ALTARC, b"DAF");
    ALTHAN = 0;

    //
    // Create FNAME.
    //
    T_CPTFIL(
        &FNAME,
        DAF,
        2,
        &BINFMT,
        b"ABCD",
        b"EFGH",
        b"IJKL",
        true,
        false,
        b"DAF/SPK ",
        ctx,
    )?;

    //
    // Open the file as a DAF to prepare for the conflict.
    //
    spicelib::ZZDDHOPN(&ALTFNM, &ALTMTH, &ALTARC, &mut ALTHAN, ctx)?;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(FILARCMISMATCH)", OK, ctx)?;

    //
    // Check HANDLE.
    //
    testutil::CHCKSI(b"HANDLE", HANDLE, b"=", 0, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    spicelib::ZZDDHCLS(ALTHAN, &ALTARC, false, ctx)?;

    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"WRITE DAS Architecture Mismatch Exception.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"WRITE");
    fstr::assign(&mut ARCH, b"DAS");
    HANDLE = 1;

    testutil::KILFIL(&FNAME, ctx)?;

    fstr::assign(&mut ALTFNM, &FNAME);
    fstr::assign(&mut ALTMTH, &METHOD);
    fstr::assign(&mut ALTARC, b"DAF");
    ALTHAN = 0;

    //
    // Create FNAME.
    //
    T_CPTFIL(
        &FNAME,
        DAF,
        2,
        &BINFMT,
        b"ABCD",
        b"EFGH",
        b"IJKL",
        true,
        false,
        b"DAF/SPK ",
        ctx,
    )?;

    //
    // Open the file as a DAF to prepare for the conflict.
    //
    spicelib::ZZDDHOPN(&ALTFNM, &ALTMTH, &ALTARC, &mut ALTHAN, ctx)?;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(FILARCMISMATCH)", OK, ctx)?;

    //
    // Check HANDLE.
    //
    testutil::CHCKSI(b"HANDLE", HANDLE, b"=", 0, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    spicelib::ZZDDHCLS(ALTHAN, &ALTARC, false, ctx)?;

    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"DAF WRITE Open Conflict Exception.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"WRITE");
    fstr::assign(&mut ARCH, b"DAF");
    HANDLE = 1;

    testutil::KILFIL(&FNAME, ctx)?;

    fstr::assign(&mut ALTFNM, &FNAME);
    fstr::assign(&mut ALTMTH, b"READ");
    fstr::assign(&mut ALTARC, &ARCH);
    ALTHAN = 0;

    //
    // Create FNAME.
    //
    T_CPTFIL(
        &FNAME,
        DAF,
        2,
        &BINFMT,
        b"ABCD",
        b"EFGH",
        b"IJKL",
        true,
        false,
        b"DAF/SPK ",
        ctx,
    )?;

    //
    // Open the file for READ access to prepare for the conflict.
    //
    spicelib::ZZDDHOPN(&ALTFNM, &ALTMTH, &ALTARC, &mut ALTHAN, ctx)?;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(FILEOPENCONFLICT)", OK, ctx)?;

    //
    // Check HANDLE.
    //
    testutil::CHCKSI(b"HANDLE", HANDLE, b"=", 0, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    spicelib::ZZDDHCLS(ALTHAN, &ALTARC, false, ctx)?;

    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"DAS WRITE Open Conflict Exception.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"WRITE");
    fstr::assign(&mut ARCH, b"DAS");
    HANDLE = 1;

    testutil::KILFIL(&FNAME, ctx)?;

    fstr::assign(&mut ALTFNM, &FNAME);
    fstr::assign(&mut ALTMTH, b"READ");
    fstr::assign(&mut ALTARC, &ARCH);
    ALTHAN = 0;

    //
    // Create FNAME.
    //
    T_CPTFIL(
        &FNAME,
        DAS,
        2,
        &BINFMT,
        b"ABCD",
        b"EFGH",
        b"IJKL",
        true,
        false,
        b"DAS/EK  ",
        ctx,
    )?;

    //
    // Open the file for READ access to prepare for the conflict.
    //
    spicelib::ZZDDHOPN(&ALTFNM, &ALTMTH, &ALTARC, &mut ALTHAN, ctx)?;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(FILEOPENCONFLICT)", OK, ctx)?;

    //
    // Check HANDLE.
    //
    testutil::CHCKSI(b"HANDLE", HANDLE, b"=", 0, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    spicelib::ZZDDHCLS(ALTHAN, &ALTARC, false, ctx)?;

    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"DAF READ/WRITE Conflict Exception.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"READ");
    fstr::assign(&mut ARCH, b"DAF");
    HANDLE = 1;

    testutil::KILFIL(&FNAME, ctx)?;

    fstr::assign(&mut ALTFNM, &FNAME);
    fstr::assign(&mut ALTMTH, b"WRITE");
    fstr::assign(&mut ALTARC, &ARCH);
    ALTHAN = 0;

    //
    // Create FNAME.
    //
    T_CPTFIL(
        &FNAME,
        DAF,
        2,
        &BINFMT,
        b"ABCD",
        b"EFGH",
        b"IJKL",
        true,
        false,
        b"DAF/SPK ",
        ctx,
    )?;

    //
    // Open the file for READ access to prepare for the conflict.
    //
    spicelib::ZZDDHOPN(&ALTFNM, &ALTMTH, &ALTARC, &mut ALTHAN, ctx)?;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(RWCONFLICT)", OK, ctx)?;

    //
    // Check HANDLE.
    //
    testutil::CHCKSI(b"HANDLE", HANDLE, b"=", 0, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    spicelib::ZZDDHCLS(ALTHAN, &ALTARC, false, ctx)?;

    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"DAS READ/WRITE Conflict Exception.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"READ");
    fstr::assign(&mut ARCH, b"DAS");
    HANDLE = 1;

    testutil::KILFIL(&FNAME, ctx)?;

    fstr::assign(&mut ALTFNM, &FNAME);
    fstr::assign(&mut ALTMTH, b"WRITE");
    fstr::assign(&mut ALTARC, &ARCH);
    ALTHAN = 0;

    //
    // Create FNAME.
    //
    T_CPTFIL(
        &FNAME,
        DAS,
        2,
        &BINFMT,
        b"ABCD",
        b"EFGH",
        b"IJKL",
        true,
        false,
        b"DAS/EK  ",
        ctx,
    )?;

    //
    // Open the file for READ access to prepare for the conflict.
    //
    spicelib::ZZDDHOPN(&ALTFNM, &ALTMTH, &ALTARC, &mut ALTHAN, ctx)?;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(RWCONFLICT)", OK, ctx)?;

    //
    // Check HANDLE.
    //
    testutil::CHCKSI(b"HANDLE", HANDLE, b"=", 0, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    spicelib::ZZDDHCLS(ALTHAN, &ALTARC, false, ctx)?;

    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"File Table Full Exception.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    fstr::assign(&mut ARCH, b"DAF");

    for I in 1..=FTSIZE {
        spicelib::REPMI(&FNMPAT, b"#", I, &mut FNAME, ctx);
        spicelib::ZZDDHOPN(&FNAME, b"NEW", &ARCH, &mut HANTST[I], ctx)?;
    }

    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"NEW");
    HANDLE = 0;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(FTFULL)", OK, ctx)?;

    //
    // Check HANDLE.
    //
    testutil::CHCKSI(b"HANDLE", HANDLE, b"=", 0, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    spicelib::ZZDDHCLS(HANDLE, &ARCH, false, ctx)?;

    testutil::KILFIL(&FNAME, ctx)?;

    //
    // Close and remove all the files used to put the system
    // into the FTFULL state.
    //
    for I in 1..=FTSIZE {
        spicelib::ZZDDHCLS(HANTST[I], &ARCH, false, ctx)?;
        spicelib::REPMI(&FNMPAT, b"#", I, &mut FNAME, ctx);
        testutil::KILFIL(&FNAME, ctx)?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"NEW File Open IOSTAT Exception.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"NEW");
    fstr::assign(&mut ARCH, b"DAS");
    HANDLE = 1;

    testutil::KILFIL(&FNAME, ctx)?;

    //
    // Create FNAME.
    //
    T_CPTFIL(
        &FNAME,
        DAS,
        2,
        &BINFMT,
        b"ABCD",
        b"EFGH",
        b"IJKL",
        true,
        false,
        b"DAS/EK  ",
        ctx,
    )?;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(FILEOPENFAIL)", OK, ctx)?;

    //
    // Check HANDLE.
    //
    testutil::CHCKSI(b"HANDLE", HANDLE, b"=", 0, 0, OK, ctx)?;

    //
    // Close the file.
    //
    spicelib::ZZDDHCLS(HANDLE, &ARCH, false, ctx)?;

    //
    // Remove the file.
    //
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"FTP Error Exception.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"READ");
    fstr::assign(&mut ARCH, b"DAS");
    HANDLE = 1;

    testutil::KILFIL(&FNAME, ctx)?;

    //
    // Create FNAME.
    //
    T_CPTFIL(
        &FNAME,
        DAS,
        2,
        &BINFMT,
        b"ABCD",
        b"EFGH",
        b"IJKL",
        true,
        true,
        b"DAS/EK  ",
        ctx,
    )?;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(FTPXFERERROR)", OK, ctx)?;

    //
    // Check HANDLE.
    //
    testutil::CHCKSI(b"HANDLE", HANDLE, b"=", 0, 0, OK, ctx)?;

    //
    // Close the file.
    //
    spicelib::ZZDDHCLS(HANDLE, &ARCH, false, ctx)?;

    //
    // Remove the file.
    //
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ID Word, Architecture Input Mismatch Exception.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"READ");
    fstr::assign(&mut ARCH, b"DAF");
    HANDLE = 1;

    testutil::KILFIL(&FNAME, ctx)?;

    //
    // Create FNAME.
    //
    T_CPTFIL(
        &FNAME,
        DAS,
        2,
        &BINFMT,
        b"ABCD",
        b"EFGH",
        b"IJKL",
        true,
        false,
        b"DAS/EK  ",
        ctx,
    )?;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(FILARCHMISMATCH)", OK, ctx)?;

    //
    // Check HANDLE.
    //
    testutil::CHCKSI(b"HANDLE", HANDLE, b"=", 0, 0, OK, ctx)?;

    //
    // Close the file.
    //
    spicelib::ZZDDHCLS(HANDLE, &ARCH, false, ctx)?;

    //
    // Remove the file.
    //
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // At this point we have exercised most of the readily available
    // exceptions that signal errors.  Now address exceptions that
    // do not signal errors.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Multiple OPEN for READ access.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"READ");
    fstr::assign(&mut ARCH, b"DAF");
    HANDLE = 0;
    ALTHAN = 0;

    testutil::KILFIL(&FNAME, ctx)?;

    //
    // Create FNAME.
    //
    T_CPTFIL(
        &FNAME,
        DAF,
        2,
        &BINFMT,
        b"ABCD",
        b"EFGH",
        b"IJKL",
        true,
        false,
        b"DAF/CK  ",
        ctx,
    )?;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Try to open the file for read again.  This time store the
    // handle returned in ALTHAN.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut ALTHAN, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check HANDLE.
    //
    testutil::CHCKSI(b"ALTHAN", ALTHAN, b"=", HANDLE, 0, OK, ctx)?;

    //
    // Close the file.
    //
    spicelib::ZZDDHCLS(HANDLE, &ARCH, false, ctx)?;

    //
    // Remove the file.
    //
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check UNIT connected to newly opened file.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    fstr::assign(&mut FNAME, b"test.fil");
    fstr::assign(&mut METHOD, b"READ");
    fstr::assign(&mut ARCH, b"DAF");
    HANDLE = 0;

    testutil::KILFIL(&FNAME, ctx)?;

    //
    // Create FNAME.
    //
    T_CPTFIL(
        &FNAME,
        DAF,
        2,
        &BINFMT,
        b"ABCD",
        b"EFGH",
        b"IJKL",
        true,
        false,
        b"DAF/CK  ",
        ctx,
    )?;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // INQUIRE on FNAME to see if it is attached to UNIT.
    //
    {
        use f2rust_std::io;

        let specs = io::InquireSpecs {
            file: Some(&FNAME),
            number: Some(&mut ALTUNT),
            exist: Some(&mut EXISTS),
            opened: Some(&mut OPENED),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.inquire(specs))?;
    }

    //
    // Check open and exists to make certain that they are appropriate
    // values.
    //
    testutil::CHCKSL(b"OPENED", OPENED, true, OK, ctx)?;
    testutil::CHCKSL(b"EXISTS", EXISTS, true, OK, ctx)?;

    //
    // Get the UNIT associated with HANDLE.
    //
    spicelib::ZZDDHHLU(HANDLE, &ARCH, false, &mut UNIT, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check IOSTAT for failure.
    //
    testutil::CHCKSI(b"IOSTAT", IOSTAT, b"=", 0, 0, OK, ctx)?;

    //
    // Compare ALTUNT to UNIT.
    //
    testutil::CHCKSI(b"UNIT", UNIT, b"=", ALTUNT, 0, OK, ctx)?;

    //
    // Close the file.
    //
    spicelib::ZZDDHCLS(HANDLE, &ARCH, false, ctx)?;

    //
    // Remove the file.
    //
    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Exercise Unsupported BFF Exception.", ctx)?;

    //
    // Loop over all possible non-native configurations guaranteed
    // to fail.
    //
    for I in 1..=NUMBFF {
        //
        // Start by handling WRITE access.  All non-native formats
        // are not supported for writing.
        //
        if (I != NATBFF) {
            for J in 1..=NUMARC {
                //
                // Create a test file for each architecture.
                //
                fstr::assign(&mut FNAME, b"test.fil");

                testutil::KILFIL(&FNAME, ctx)?;

                //
                // Create FNAME.
                //
                T_CPTFIL(
                    &FNAME, J, 2, &STRBFF[I], b"ABCD", b"EFGH", b"IJKL", true, false, &IDWARY[J],
                    ctx,
                )?;

                //
                // Begin the test by attempting to open the
                // non-native file for WRITE access.
                //
                fstr::assign(&mut ARCH, STRARC.get(J));
                fstr::assign(&mut METHOD, b"WRITE");
                HANDLE = 1;

                //
                // Invoke the module.
                //
                spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

                //
                // Check for the exception.
                //
                testutil::CHCKXC(true, b"SPICE(UNSUPPORTEDBFF)", OK, ctx)?;

                //
                // Check the value of HANDLE.
                //
                testutil::CHCKSI(b"HANDLE", HANDLE, b"=", 0, 0, OK, ctx)?;

                //
                // Now if I is not in SUPBFF, attempt to open it for
                // READ access.
                //
                if (spicelib::ISRCHI(I, NUMSUP, SUPBFF.as_slice()) == 0) {
                    fstr::assign(&mut ARCH, STRARC.get(J));
                    fstr::assign(&mut METHOD, b"READ");
                    HANDLE = 1;

                    //
                    // Invoke the module.
                    //
                    spicelib::ZZDDHOPN(&FNAME, &METHOD, &ARCH, &mut HANDLE, ctx)?;

                    //
                    // Check for the exception.
                    //
                    testutil::CHCKXC(true, b"SPICE(UNSUPPORTEDBFF)", OK, ctx)?;

                    //
                    // Check the value of HANDLE.
                    //
                    testutil::CHCKSI(b"HANDLE", HANDLE, b"=", 0, 0, OK, ctx)?;
                }

                //
                // All tests related to FNAME are complete.  Delete it.
                //
                testutil::KILFIL(&FNAME, ctx)?;
            }
        }
    }

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
