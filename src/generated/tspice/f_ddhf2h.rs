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
const NUMFIL: i32 = (UTSIZE + 5);

//$Procedure F_DDHF2H ( ZZDDHF2H Test Family )
pub fn F_DDHF2H(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut NFT: i32 = 0;
    let mut FTABS = ActualArray::<i32>::new(1..=FTSIZE);
    let mut FTAMH = ActualArray::<i32>::new(1..=FTSIZE);
    let mut FTARC = ActualArray::<i32>::new(1..=FTSIZE);
    let mut FTBFF = ActualArray::<i32>::new(1..=FTSIZE);
    let mut FTHAN = ActualArray::<i32>::new(1..=FTSIZE);
    let mut FTNAM = ActualCharArray::new(FILEN, 1..=FTSIZE);
    let mut FTRTM = ActualArray::<i32>::new(1..=FTSIZE);
    let mut FTMNM = ActualArray::<f64>::new(1..=FTSIZE);
    let mut NUT: i32 = 0;
    let mut UTCST = StackArray::<i32, 23>::new(1..=UTSIZE);
    let mut UTHAN = StackArray::<i32, 23>::new(1..=UTSIZE);
    let mut UTLCK = StackArray::<bool, 23>::new(1..=UTSIZE);
    let mut UTLUN = StackArray::<i32, 23>::new(1..=UTSIZE);
    let mut FNAME = [b' '; FILEN as usize];
    let mut FNMTMP = [b' '; FILEN as usize];
    let mut HANDLE: i32 = 0;
    let mut INDEX: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut LUN: i32 = 0;
    let mut MNM: f64 = 0.0;
    let mut FILMNM = StackArray::<f64, 28>::new(1..=NUMFIL);
    let mut EXISTS: bool = false;
    let mut FOUND: bool = false;
    let mut OPENED: bool = false;

    //
    // SPICELIB Functions
    //

    //
    // Local Parameters
    //
    //
    // This parameter defines the number of test files we are going
    // to create.  It should be something more than UTSIZE, to
    // properly execute logic in ZZDDHF2H.
    //

    //
    // Local Variables
    //
    // File Table
    //

    //
    // Unit Table
    //

    //
    // Other Variables
    //

    //
    // Start the test family with an open call.
    //
    testutil::TOPEN(b"F_DDHF2H", ctx)?;

    fstr::assign(&mut FNMTMP, b"spk#.bsp");

    //
    // Start by creating NUMFIL files for testing purposes.
    //
    for I in 1..=NUMFIL {
        spicelib::REPMI(&FNMTMP, b"#", I, &mut FNAME, ctx);
        testutil::TSTSPK(&FNAME, false, &mut HANDLE, ctx)?;

        //
        // Save the magic number for each test file. Open file first.
        //
        spicelib::GETLUN(&mut LUN, ctx)?;

        {
            use f2rust_std::io;

            let specs = io::OpenSpecs {
                unit: Some(LUN),
                file: Some(&FNAME),
                access: Some(b"DIRECT"),
                recl: Some(RECL),
                status: Some(b"OLD"),
                ..Default::default()
            };
            IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
        }

        //
        // Check the IOSTAT.
        //
        if (IOSTAT != 0) {
            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(LUN),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }
            testutil::TSTMSG(b"#", b"This IOSTAT error should never occur.", ctx);
            testutil::CHCKSI(b"IOSTAT", IOSTAT, b"=", 0, 0, OK, ctx)?;
        }

        //
        // Compute the magic number.
        //
        FILMNM[I] = spicelib::ZZDDHMNM(LUN, ctx)?;

        //
        // Close the file.
        //
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(LUN),
                ..Default::default()
            };
            ctx.close(specs)?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"File does not exist exceptional case.", ctx)?;

    //
    // First, kill the file name we are about to check against, since
    // it should not exist.
    //
    testutil::KILFIL(b"spk0.bsp", ctx)?;

    //
    // Prepare the inputs and output default values.
    //
    NUT = 0;
    NFT = 0;

    EXISTS = true;
    OPENED = true;
    HANDLE = -1;
    FOUND = true;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHF2H(
        b"spk0.bsp",
        FTABS.as_slice(),
        FTAMH.as_slice(),
        FTARC.as_slice(),
        FTBFF.as_slice(),
        FTHAN.as_slice(),
        FTNAM.as_arg(),
        FTRTM.as_slice(),
        FTMNM.as_slice(),
        NFT,
        UTCST.as_slice_mut(),
        UTHAN.as_slice_mut(),
        UTLCK.as_slice_mut(),
        UTLUN.as_slice_mut(),
        &mut NUT,
        &mut EXISTS,
        &mut OPENED,
        &mut HANDLE,
        &mut FOUND,
        &mut MNM,
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
    testutil::CHCKSL(b"OPENED", OPENED, false, OK, ctx)?;
    testutil::CHCKSL(b"EXISTS", EXISTS, false, OK, ctx)?;
    testutil::CHCKSI(b"HANDLE", HANDLE, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"File exists and is opened externally exceptional case.",
        ctx,
    )?;

    //
    // Open the first test file we created.
    //
    spicelib::GETLUN(&mut LUN, ctx)?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(LUN),
            file: Some(b"spk1.bsp"),
            access: Some(b"DIRECT"),
            recl: Some(RECL),
            status: Some(b"OLD"),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
    }

    //
    // Check the IOSTAT.
    //
    if (IOSTAT != 0) {
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(LUN),
                ..Default::default()
            };
            ctx.close(specs)?;
        }
        testutil::TSTMSG(b"#", b"This IOSTAT error should never occur.", ctx);
        testutil::CHCKSI(b"IOSTAT", IOSTAT, b"=", 0, 0, OK, ctx)?;
    }

    //
    // Setup the inputs and output default values.
    //
    NUT = 0;
    NFT = 0;

    EXISTS = false;
    OPENED = false;
    HANDLE = -1;
    FOUND = true;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHF2H(
        b"spk1.bsp",
        FTABS.as_slice(),
        FTAMH.as_slice(),
        FTARC.as_slice(),
        FTBFF.as_slice(),
        FTHAN.as_slice(),
        FTNAM.as_arg(),
        FTRTM.as_slice(),
        FTMNM.as_slice(),
        NFT,
        UTCST.as_slice_mut(),
        UTHAN.as_slice_mut(),
        UTLCK.as_slice_mut(),
        UTLUN.as_slice_mut(),
        &mut NUT,
        &mut EXISTS,
        &mut OPENED,
        &mut HANDLE,
        &mut FOUND,
        &mut MNM,
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
    testutil::CHCKSL(b"OPENED", OPENED, true, OK, ctx)?;
    testutil::CHCKSL(b"EXISTS", EXISTS, true, OK, ctx)?;
    testutil::CHCKSI(b"HANDLE", HANDLE, b"=", 0, 0, OK, ctx)?;

    //
    // Close the test file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(LUN),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"File exists and is opened internally and is in the unit table case.",
        ctx,
    )?;

    //
    // Open the first test file we created.
    //
    spicelib::GETLUN(&mut LUN, ctx)?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(LUN),
            file: Some(b"spk1.bsp"),
            access: Some(b"DIRECT"),
            recl: Some(RECL),
            status: Some(b"OLD"),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
    }

    //
    // Check the IOSTAT.
    //
    if (IOSTAT != 0) {
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(LUN),
                ..Default::default()
            };
            ctx.close(specs)?;
        }
        testutil::TSTMSG(b"#", b"This IOSTAT error should never occur.", ctx);
        testutil::CHCKSI(b"IOSTAT", IOSTAT, b"=", 0, 0, OK, ctx)?;
    }

    //
    // Setup the inputs and output default values. Create a row for
    // spk1.bsp in the file table and unit tables.
    //
    NFT = 1;
    FTABS[NFT] = i32::abs(NFT);
    FTAMH[NFT] = READ;
    FTARC[NFT] = DAF;
    FTBFF[NFT] = BIGI3E;
    FTHAN[NFT] = NFT;
    fstr::assign(FTNAM.get_mut(NFT), b"spk1.bsp");
    FTRTM[NFT] = spicelib::RTRIM(&FTNAM[NFT]);
    FTMNM[NFT] = FILMNM[1];

    NUT = 1;
    UTCST[NUT] = 2;
    UTHAN[NUT] = FTHAN[NFT];
    UTLCK[NUT] = false;
    UTLUN[NUT] = LUN;

    EXISTS = false;
    OPENED = false;
    HANDLE = -1;
    FOUND = false;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHF2H(
        b"spk1.bsp",
        FTABS.as_slice(),
        FTAMH.as_slice(),
        FTARC.as_slice(),
        FTBFF.as_slice(),
        FTHAN.as_slice(),
        FTNAM.as_arg(),
        FTRTM.as_slice(),
        FTMNM.as_slice(),
        NFT,
        UTCST.as_slice_mut(),
        UTHAN.as_slice_mut(),
        UTLCK.as_slice_mut(),
        UTLUN.as_slice_mut(),
        &mut NUT,
        &mut EXISTS,
        &mut OPENED,
        &mut HANDLE,
        &mut FOUND,
        &mut MNM,
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
    testutil::CHCKSL(b"OPENED", OPENED, true, OK, ctx)?;
    testutil::CHCKSL(b"EXISTS", EXISTS, true, OK, ctx)?;
    testutil::CHCKSI(b"HANDLE", HANDLE, b"=", 1, 0, OK, ctx)?;

    //
    // Close the test file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(LUN),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"File exists and is opened internally and is not in the unit table case.",
        ctx,
    )?;

    //
    // Open the files we want to put in the unit table.
    //
    NUT = 0;
    NFT = 0;

    for I in 1..=10 {
        //
        // Get a unit.
        //
        spicelib::GETLUN(&mut LUN, ctx)?;

        //
        // Compute the filename.
        //
        spicelib::REPMI(&FNMTMP, b"#", I, &mut FNAME, ctx);

        {
            use f2rust_std::io;

            let specs = io::OpenSpecs {
                unit: Some(LUN),
                file: Some(&FNAME),
                access: Some(b"DIRECT"),
                recl: Some(RECL),
                status: Some(b"OLD"),
                ..Default::default()
            };
            IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
        }

        //
        // Check the IOSTAT.
        //
        if (IOSTAT != 0) {
            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(LUN),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }
            testutil::TSTMSG(b"#", b"This IOSTAT error should never occur.", ctx);
            testutil::CHCKSI(b"IOSTAT", IOSTAT, b"=", 0, 0, OK, ctx)?;
        }

        //
        // Setup the inputs and output default values. Create a row for
        // FNAME in the file table and unit tables.
        //
        NFT = (NFT + 1);
        FTABS[NFT] = i32::abs(NFT);
        FTAMH[NFT] = READ;
        FTARC[NFT] = DAF;
        FTBFF[NFT] = BIGI3E;
        FTHAN[NFT] = NFT;
        fstr::assign(FTNAM.get_mut(NFT), &FNAME);
        FTRTM[NFT] = spicelib::RTRIM(&FTNAM[NFT]);
        FTMNM[NFT] = FILMNM[I];

        NUT = (NUT + 1);
        UTCST[NUT] = (I * 2);
        UTHAN[NUT] = FTHAN[NFT];
        UTLCK[NUT] = false;
        UTLUN[NUT] = LUN;
    }

    //
    // Add a few more files to the file table.
    //
    for I in 1..=5 {
        spicelib::REPMI(&FNMTMP, b"#", (I + 10), &mut FNAME, ctx);

        NFT = (NFT + 1);
        FTABS[NFT] = i32::abs(NFT);
        FTAMH[NFT] = READ;
        FTARC[NFT] = DAF;
        FTBFF[NFT] = BIGI3E;
        FTHAN[NFT] = NFT;
        fstr::assign(FTNAM.get_mut(NFT), &FNAME);
        FTRTM[NFT] = spicelib::RTRIM(&FTNAM[NFT]);
        FTMNM[NFT] = FILMNM[(I + 10)];
    }

    EXISTS = false;
    OPENED = true;
    HANDLE = -1;
    FOUND = false;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHF2H(
        b"spk13.bsp",
        FTABS.as_slice(),
        FTAMH.as_slice(),
        FTARC.as_slice(),
        FTBFF.as_slice(),
        FTHAN.as_slice(),
        FTNAM.as_arg(),
        FTRTM.as_slice(),
        FTMNM.as_slice(),
        NFT,
        UTCST.as_slice_mut(),
        UTHAN.as_slice_mut(),
        UTLCK.as_slice_mut(),
        UTLUN.as_slice_mut(),
        &mut NUT,
        &mut EXISTS,
        &mut OPENED,
        &mut HANDLE,
        &mut FOUND,
        &mut MNM,
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
    testutil::CHCKSL(b"OPENED", OPENED, false, OK, ctx)?;
    testutil::CHCKSL(b"EXISTS", EXISTS, true, OK, ctx)?;
    testutil::CHCKSI(b"HANDLE", HANDLE, b"=", 13, 0, OK, ctx)?;

    //
    // Close the test files.
    //
    for I in 1..=NUT {
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(UTLUN[I]),
                ..Default::default()
            };
            ctx.close(specs)?;
        }
    }

    //
    // We need to free the logical unit reserved implicitly by ZZDDHF2H's
    // use of ZZDDHRMU.
    //
    INDEX = spicelib::ISRCHI(0, NUT, UTHAN.as_slice());

    if (INDEX > 0) {
        spicelib::FRELUN(UTLUN[INDEX], ctx);
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Unit table full, file not found, but in file table case.",
        ctx,
    )?;

    //
    // Open the files we want to put in the unit table.
    //
    NUT = 0;
    NFT = 0;

    for I in 1..=UTSIZE {
        //
        // Get a unit.
        //
        spicelib::GETLUN(&mut LUN, ctx)?;

        //
        // Compute the filename.
        //
        spicelib::REPMI(&FNMTMP, b"#", I, &mut FNAME, ctx);

        {
            use f2rust_std::io;

            let specs = io::OpenSpecs {
                unit: Some(LUN),
                file: Some(&FNAME),
                access: Some(b"DIRECT"),
                recl: Some(RECL),
                status: Some(b"OLD"),
                ..Default::default()
            };
            IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
        }

        //
        // Check the IOSTAT.
        //
        if (IOSTAT != 0) {
            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(LUN),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }
            testutil::TSTMSG(b"#", b"This IOSTAT error should never occur.", ctx);
            testutil::CHCKSI(b"IOSTAT", IOSTAT, b"=", 0, 0, OK, ctx)?;
        }

        //
        // Setup the inputs and output default values. Create a row for
        // FNAME in the file table and unit tables.
        //
        NFT = (NFT + 1);
        FTABS[NFT] = i32::abs(NFT);
        FTAMH[NFT] = READ;
        FTARC[NFT] = DAF;
        FTBFF[NFT] = BIGI3E;
        FTHAN[NFT] = NFT;
        fstr::assign(FTNAM.get_mut(NFT), &FNAME);
        FTRTM[NFT] = spicelib::RTRIM(&FTNAM[NFT]);
        FTMNM[NFT] = FILMNM[I];

        NUT = (NUT + 1);
        UTCST[NUT] = (I * 2);
        UTHAN[NUT] = FTHAN[NFT];
        UTLCK[NUT] = false;
        UTLUN[NUT] = LUN;
    }

    //
    // Add a few more files to the file table.
    //
    for I in 1..=5 {
        spicelib::REPMI(&FNMTMP, b"#", (I + UTSIZE), &mut FNAME, ctx);

        NFT = (NFT + 1);
        FTABS[NFT] = i32::abs(NFT);
        FTAMH[NFT] = READ;
        FTARC[NFT] = DAF;
        FTBFF[NFT] = BIGI3E;
        FTHAN[NFT] = NFT;
        fstr::assign(FTNAM.get_mut(NFT), &FNAME);
        FTRTM[NFT] = spicelib::RTRIM(&FTNAM[NFT]);
        FTMNM[NFT] = FILMNM[(I + UTSIZE)];
    }

    EXISTS = false;
    OPENED = true;
    HANDLE = -1;
    FOUND = false;

    spicelib::REPMI(&FNMTMP, b"#", NUMFIL, &mut FNAME, ctx);

    //
    // Invoke the module.
    //
    spicelib::ZZDDHF2H(
        &FNAME,
        FTABS.as_slice(),
        FTAMH.as_slice(),
        FTARC.as_slice(),
        FTBFF.as_slice(),
        FTHAN.as_slice(),
        FTNAM.as_arg(),
        FTRTM.as_slice(),
        FTMNM.as_slice(),
        NFT,
        UTCST.as_slice_mut(),
        UTHAN.as_slice_mut(),
        UTLCK.as_slice_mut(),
        UTLUN.as_slice_mut(),
        &mut NUT,
        &mut EXISTS,
        &mut OPENED,
        &mut HANDLE,
        &mut FOUND,
        &mut MNM,
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
    testutil::CHCKSL(b"OPENED", OPENED, false, OK, ctx)?;
    testutil::CHCKSL(b"EXISTS", EXISTS, true, OK, ctx)?;
    testutil::CHCKSI(b"HANDLE", HANDLE, b"=", NUMFIL, 0, OK, ctx)?;

    //
    // Close the test files.
    //
    for I in 1..=UTSIZE {
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(UTLUN[I]),
                ..Default::default()
            };
            ctx.close(specs)?;
        }
    }

    //
    // We need to free the logical unit reserved implicitly by ZZDDHF2H's
    // use of ZZDDHRMU.
    //
    INDEX = spicelib::ISRCHI(0, NUT, UTHAN.as_slice());

    if (INDEX > 0) {
        spicelib::FRELUN(UTLUN[INDEX], ctx);
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Unit table full, file not found case.", ctx)?;

    //
    // Open the files we want to put in the unit table.
    //
    NUT = 0;
    NFT = 0;

    for I in 1..=UTSIZE {
        //
        // Get a unit.
        //
        spicelib::GETLUN(&mut LUN, ctx)?;

        //
        // Compute the filename.
        //
        spicelib::REPMI(&FNMTMP, b"#", I, &mut FNAME, ctx);

        {
            use f2rust_std::io;

            let specs = io::OpenSpecs {
                unit: Some(LUN),
                file: Some(&FNAME),
                access: Some(b"DIRECT"),
                recl: Some(RECL),
                status: Some(b"OLD"),
                ..Default::default()
            };
            IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
        }

        //
        // Check the IOSTAT.
        //
        if (IOSTAT != 0) {
            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(LUN),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }
            testutil::TSTMSG(b"#", b"This IOSTAT error should never occur.", ctx);
            testutil::CHCKSI(b"IOSTAT", IOSTAT, b"=", 0, 0, OK, ctx)?;
        }

        //
        // Setup the inputs and output default values. Create a row for
        // FNAME in the file table and unit tables.
        //
        NFT = (NFT + 1);
        FTABS[NFT] = i32::abs(NFT);
        FTAMH[NFT] = READ;
        FTARC[NFT] = DAF;
        FTBFF[NFT] = BIGI3E;
        FTHAN[NFT] = NFT;
        fstr::assign(FTNAM.get_mut(NFT), &FNAME);
        FTRTM[NFT] = spicelib::RTRIM(&FTNAM[NFT]);
        FTMNM[NFT] = FILMNM[I];

        NUT = (NUT + 1);
        UTCST[NUT] = (I * 2);
        UTHAN[NUT] = FTHAN[NFT];
        UTLCK[NUT] = false;
        UTLUN[NUT] = LUN;
    }

    //
    // Add a few more files to the file table.
    //
    for I in 1..=4 {
        spicelib::REPMI(&FNMTMP, b"#", (I + UTSIZE), &mut FNAME, ctx);

        NFT = (NFT + 1);
        FTABS[NFT] = i32::abs(NFT);
        FTAMH[NFT] = READ;
        FTARC[NFT] = DAF;
        FTBFF[NFT] = BIGI3E;
        FTHAN[NFT] = NFT;
        fstr::assign(FTNAM.get_mut(NFT), &FNAME);
        FTRTM[NFT] = spicelib::RTRIM(&FTNAM[NFT]);
        FTMNM[NFT] = FILMNM[(I + UTSIZE)];
    }

    EXISTS = false;
    OPENED = true;
    HANDLE = -1;
    FOUND = false;

    spicelib::REPMI(&FNMTMP, b"#", NUMFIL, &mut FNAME, ctx);

    //
    // Invoke the module.
    //
    spicelib::ZZDDHF2H(
        &FNAME,
        FTABS.as_slice(),
        FTAMH.as_slice(),
        FTARC.as_slice(),
        FTBFF.as_slice(),
        FTHAN.as_slice(),
        FTNAM.as_arg(),
        FTRTM.as_slice(),
        FTMNM.as_slice(),
        NFT,
        UTCST.as_slice_mut(),
        UTHAN.as_slice_mut(),
        UTLCK.as_slice_mut(),
        UTLUN.as_slice_mut(),
        &mut NUT,
        &mut EXISTS,
        &mut OPENED,
        &mut HANDLE,
        &mut FOUND,
        &mut MNM,
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
    testutil::CHCKSL(b"OPENED", OPENED, false, OK, ctx)?;
    testutil::CHCKSL(b"EXISTS", EXISTS, true, OK, ctx)?;
    testutil::CHCKSI(b"HANDLE", HANDLE, b"=", 0, 0, OK, ctx)?;

    //
    // Close the test files.
    //
    for I in 1..=UTSIZE {
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(UTLUN[I]),
                ..Default::default()
            };
            ctx.close(specs)?;
        }
    }

    //
    // We need to free the logical unit reserved implicitly by ZZDDHF2H's
    // use of ZZDDHRMU.
    //
    INDEX = spicelib::ISRCHI(0, NUT, UTHAN.as_slice());

    if (INDEX > 0) {
        spicelib::FRELUN(UTLUN[INDEX], ctx);
    }

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
