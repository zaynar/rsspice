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

//$Procedure F_DDHPPF ( ZZDDHPPF Test Family )
pub fn F_DDHPPF(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut CFDREC = [b' '; 4 as usize];
    let mut CNI = [b' '; 4 as usize];
    let mut CNSUM = [b' '; 8 as usize];
    let mut NATIVE = [b' '; STRSIZ as usize];
    let mut NULBFF = [b' '; 8 as usize];
    let mut NULL = [b' '; 1 as usize];
    let mut STRBFF = ActualCharArray::new(STRSIZ, 1..=NUMBFF);
    let mut ARC: i32 = 0;
    let mut BFF: i32 = 0;
    let mut FDREC: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut NATBFF: i32 = 0;
    let mut UNIT: i32 = 0;

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Start the test family with an open call.
    //
    testutil::TOPEN(b"F_DDHPPF", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"F_DDHPPF Initialization", ctx)?;

    //
    // Now retrieve the list of supported binary file formats.
    //
    for I in 1..=NUMBFF {
        spicelib::ZZDDHGSD(b"BFF", I, &mut STRBFF[I], ctx);
    }

    //
    // Setup the null byte character.
    //
    fstr::assign(&mut NULL, &intrinsics::CHAR(0));

    //
    // Initialize NULBFF.
    //
    for I in 1..=8 {
        fstr::assign(fstr::substr_mut(&mut NULBFF, I..=I), &NULL);
    }

    //
    // Check for the absence of a rogue exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid architecture code exception.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    spicelib::GETLUN(&mut UNIT, ctx)?;

    ARC = -1;
    BFF = 1;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHPPF(UNIT, ARC, &mut BFF, ctx)?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(UNKNOWNFILARC)", OK, ctx)?;

    //
    // Check BFF.
    //
    testutil::CHCKSI(b"BFF", BFF, b"=", 0, 0, OK, ctx)?;

    //
    // Setup the inputs and outputs for the other bound.
    //
    ARC = (NUMARC + 1);
    BFF = 1;

    //
    // Invoke the module.
    //
    spicelib::ZZDDHPPF(UNIT, ARC, &mut BFF, ctx)?;

    //
    // Check for the exception.
    //
    testutil::CHCKXC(true, b"SPICE(UNKNOWNFILARC)", OK, ctx)?;

    //
    // Check BFF.
    //
    testutil::CHCKSI(b"BFF", BFF, b"=", 0, 0, OK, ctx)?;

    //
    //
    //     We have commented this case out because it leaves a "fort.#"
    //     file around on some systems.
    //
    // --- Case: ------------------------------------------------------
    //
    //     CALL TCASE ( 'File read failure.' )
    //
    //
    //     Setup the inputs and outputs.
    //
    //     CALL GETLUN ( UNIT )
    //
    //     ARC = DAF
    //     BFF = BIGI3E
    //
    //
    //     Invoke the module.
    //
    //     CALL ZZDDHPPF ( UNIT, ARC, BFF )
    //
    //
    //     Check for the exception.
    //
    //     CALL CHCKXC ( .TRUE., 'SPICE(FILEREADFAILED)', OK )
    //
    //
    //     Check BFF.
    //
    //     CALL CHCKSI ( 'BFF', BFF, '=', 0, 0, OK )
    //
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Unknown File Architecture.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    spicelib::GETLUN(&mut UNIT, ctx)?;

    ARC = DAF;
    BFF = BIGI3E;

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    T_CPTFIL(
        b"testdaf.daf",
        ARC,
        2,
        b"BIG-IEEE",
        b"ABCD",
        b"EFGH",
        b"IJKL",
        true,
        false,
        b"TEST/FIL",
        ctx,
    )?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(UNIT),
            file: Some(b"testdaf.daf"),
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
    // Now invoke the module.
    //
    spicelib::ZZDDHPPF(UNIT, ARC, &mut BFF, ctx)?;

    //
    // Check for the presence of the exception.
    //
    testutil::CHCKXC(true, b"SPICE(UNKNOWNFILARC)", OK, ctx)?;

    //
    // Now check BFF.
    //
    testutil::CHCKSI(b"BFF", BFF, b"=", 0, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"DAS File Architecture, should be DAF.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    spicelib::GETLUN(&mut UNIT, ctx)?;

    ARC = DAF;
    BFF = BIGI3E;

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    T_CPTFIL(
        b"testdaf.daf",
        ARC,
        2,
        b"BIG-IEEE",
        b"ABCD",
        b"EFGH",
        b"IJKL",
        true,
        false,
        b"DAS/EK  ",
        ctx,
    )?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(UNIT),
            file: Some(b"testdaf.daf"),
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
    // Now invoke the module.
    //
    spicelib::ZZDDHPPF(UNIT, ARC, &mut BFF, ctx)?;

    //
    // Check for the presence of the exception.
    //
    testutil::CHCKXC(true, b"SPICE(FILARCHMISMATCH)", OK, ctx)?;

    //
    // Now check BFF.
    //
    testutil::CHCKSI(b"BFF", BFF, b"=", 0, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"DAF File Architecture, should be DAS.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    spicelib::GETLUN(&mut UNIT, ctx)?;

    ARC = DAS;
    BFF = BIGI3E;

    testutil::KILFIL(b"testdas.das", ctx)?;

    T_CPTFIL(
        b"testdas.das",
        ARC,
        2,
        b"BIG-IEEE",
        b"ABCD",
        b"EFGH",
        b"IJKL",
        true,
        false,
        b"DAF/SPK ",
        ctx,
    )?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(UNIT),
            file: Some(b"testdas.das"),
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
    // Now invoke the module.
    //
    spicelib::ZZDDHPPF(UNIT, ARC, &mut BFF, ctx)?;

    //
    // Check for the presence of the exception.
    //
    testutil::CHCKXC(true, b"SPICE(FILARCHMISMATCH)", OK, ctx)?;

    //
    // Now check BFF.
    //
    testutil::CHCKSI(b"BFF", BFF, b"=", 0, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    testutil::KILFIL(b"testdas.das", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"FTP String. Error in FTP check, DAF.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    spicelib::GETLUN(&mut UNIT, ctx)?;

    ARC = DAF;
    BFF = BIGI3E;

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    T_CPTFIL(
        b"testdaf.daf",
        ARC,
        2,
        b"BIG-IEEE",
        b"ABCD",
        b"EFGH",
        b"IJKL",
        true,
        true,
        b"DAF/SPK ",
        ctx,
    )?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(UNIT),
            file: Some(b"testdaf.daf"),
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
    // Now invoke the module.
    //
    spicelib::ZZDDHPPF(UNIT, ARC, &mut BFF, ctx)?;

    //
    // Check for the presence of the exception.
    //
    testutil::CHCKXC(true, b"SPICE(FTPXFERERROR)", OK, ctx)?;

    //
    // Now check BFF.
    //
    testutil::CHCKSI(b"BFF", BFF, b"=", 0, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"FTP String. Error in FTP check, DAS.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    spicelib::GETLUN(&mut UNIT, ctx)?;

    ARC = DAS;
    BFF = BIGI3E;

    testutil::KILFIL(b"testdas.das", ctx)?;

    T_CPTFIL(
        b"testdas.das",
        ARC,
        2,
        b"BIG-IEEE",
        b"ABCD",
        b"EFGH",
        b"IJKL",
        true,
        true,
        b"DAS/EK  ",
        ctx,
    )?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(UNIT),
            file: Some(b"testdas.das"),
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
    // Now invoke the module.
    //
    spicelib::ZZDDHPPF(UNIT, ARC, &mut BFF, ctx)?;

    //
    // Check for the presence of the exception.
    //
    testutil::CHCKXC(true, b"SPICE(FTPXFERERROR)", OK, ctx)?;

    //
    // Now check BFF.
    //
    testutil::CHCKSI(b"BFF", BFF, b"=", 0, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    testutil::KILFIL(b"testdas.das", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"FTP String. Unknown Binary File Format.", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    spicelib::GETLUN(&mut UNIT, ctx)?;

    ARC = DAF;
    BFF = BIGI3E;

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    T_CPTFIL(
        b"testdaf.daf",
        ARC,
        2,
        b"UNKNOWN-",
        b"ABCD",
        b"EFGH",
        b"IJKL",
        true,
        false,
        b" ",
        ctx,
    )?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(UNIT),
            file: Some(b"testdaf.daf"),
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
    // Now invoke the module.
    //
    spicelib::ZZDDHPPF(UNIT, ARC, &mut BFF, ctx)?;

    //
    // Check for the presence of the exception.
    //
    testutil::CHCKXC(true, b"SPICE(UNKNOWNBFF)", OK, ctx)?;

    //
    // Now check BFF.
    //
    testutil::CHCKSI(b"BFF", BFF, b"=", 0, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"FTP String. BFFID: BIG-IEEE DAF", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    spicelib::GETLUN(&mut UNIT, ctx)?;

    ARC = DAF;
    BFF = 0;

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    T_CPTFIL(
        b"testdaf.daf",
        ARC,
        2,
        b"BIG-IEEE",
        b"ABCD",
        b"EFGH",
        b"IJKL",
        true,
        false,
        b" ",
        ctx,
    )?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(UNIT),
            file: Some(b"testdaf.daf"),
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
    // Now invoke the module.
    //
    spicelib::ZZDDHPPF(UNIT, ARC, &mut BFF, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check BFF for the appropriate value.
    //
    testutil::CHCKSI(b"BFF", BFF, b"=", BIGI3E, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"FTP String. BFFID: LTL-IEEE DAF", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    spicelib::GETLUN(&mut UNIT, ctx)?;

    ARC = DAF;
    BFF = 0;

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    T_CPTFIL(
        b"testdaf.daf",
        ARC,
        2,
        b"LTL-IEEE",
        b"ABCD",
        b"EFGH",
        b"IJKL",
        true,
        false,
        b" ",
        ctx,
    )?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(UNIT),
            file: Some(b"testdaf.daf"),
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
    // Now invoke the module.
    //
    spicelib::ZZDDHPPF(UNIT, ARC, &mut BFF, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check BFF for the appropriate value.
    //
    testutil::CHCKSI(b"BFF", BFF, b"=", LTLI3E, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"FTP String. BFFID: VAX-GFLT DAF", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    spicelib::GETLUN(&mut UNIT, ctx)?;

    ARC = DAF;
    BFF = 0;

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    T_CPTFIL(
        b"testdaf.daf",
        ARC,
        2,
        b"VAX-GFLT",
        b"ABCD",
        b"EFGH",
        b"IJKL",
        true,
        false,
        b" ",
        ctx,
    )?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(UNIT),
            file: Some(b"testdaf.daf"),
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
    // Now invoke the module.
    //
    spicelib::ZZDDHPPF(UNIT, ARC, &mut BFF, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check BFF for the appropriate value.
    //
    testutil::CHCKSI(b"BFF", BFF, b"=", VAXGFL, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"FTP String. BFFID: VAX-DFLT DAF", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    spicelib::GETLUN(&mut UNIT, ctx)?;

    ARC = DAF;
    BFF = 0;

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    T_CPTFIL(
        b"testdaf.daf",
        ARC,
        2,
        b"VAX-DFLT",
        b"ABCD",
        b"EFGH",
        b"IJKL",
        true,
        false,
        b" ",
        ctx,
    )?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(UNIT),
            file: Some(b"testdaf.daf"),
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
    // Now invoke the module.
    //
    spicelib::ZZDDHPPF(UNIT, ARC, &mut BFF, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check BFF for the appropriate value.
    //
    testutil::CHCKSI(b"BFF", BFF, b"=", VAXDFL, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"FTP String. BFFID: BIG-IEEE DAS", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    spicelib::GETLUN(&mut UNIT, ctx)?;

    ARC = DAS;
    BFF = 0;

    testutil::KILFIL(b"testdas.das", ctx)?;

    T_CPTFIL(
        b"testdas.das",
        ARC,
        2,
        b"BIG-IEEE",
        b"ABCD",
        b"EFGH",
        b"IJKL",
        true,
        false,
        b" ",
        ctx,
    )?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(UNIT),
            file: Some(b"testdas.das"),
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
    // Now invoke the module.
    //
    spicelib::ZZDDHPPF(UNIT, ARC, &mut BFF, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check BFF for the appropriate value.
    //
    testutil::CHCKSI(b"BFF", BFF, b"=", BIGI3E, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    testutil::KILFIL(b"testdas.das", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"FTP String. BFFID: LTL-IEEE DAS", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    spicelib::GETLUN(&mut UNIT, ctx)?;

    ARC = DAS;
    BFF = 0;

    testutil::KILFIL(b"testdas.das", ctx)?;

    T_CPTFIL(
        b"testdas.das",
        ARC,
        2,
        b"LTL-IEEE",
        b"ABCD",
        b"EFGH",
        b"IJKL",
        true,
        false,
        b" ",
        ctx,
    )?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(UNIT),
            file: Some(b"testdas.das"),
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
    // Now invoke the module.
    //
    spicelib::ZZDDHPPF(UNIT, ARC, &mut BFF, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check BFF for the appropriate value.
    //
    testutil::CHCKSI(b"BFF", BFF, b"=", LTLI3E, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    testutil::KILFIL(b"testdas.das", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"FTP String. BFFID: VAX-GFLT DAS", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    spicelib::GETLUN(&mut UNIT, ctx)?;

    ARC = DAS;
    BFF = 0;

    testutil::KILFIL(b"testdas.das", ctx)?;

    T_CPTFIL(
        b"testdas.das",
        ARC,
        2,
        b"VAX-GFLT",
        b"ABCD",
        b"EFGH",
        b"IJKL",
        true,
        false,
        b" ",
        ctx,
    )?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(UNIT),
            file: Some(b"testdas.das"),
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
    // Now invoke the module.
    //
    spicelib::ZZDDHPPF(UNIT, ARC, &mut BFF, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check BFF for the appropriate value.
    //
    testutil::CHCKSI(b"BFF", BFF, b"=", VAXGFL, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    testutil::KILFIL(b"testdas.das", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"FTP String. BFFID: VAX-DFLT DAS", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    spicelib::GETLUN(&mut UNIT, ctx)?;

    ARC = DAS;
    BFF = 0;

    testutil::KILFIL(b"testdas.das", ctx)?;

    T_CPTFIL(
        b"testdas.das",
        ARC,
        2,
        b"VAX-DFLT",
        b"ABCD",
        b"EFGH",
        b"IJKL",
        true,
        false,
        b" ",
        ctx,
    )?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(UNIT),
            file: Some(b"testdas.das"),
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
    // Now invoke the module.
    //
    spicelib::ZZDDHPPF(UNIT, ARC, &mut BFF, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check BFF for the appropriate value.
    //
    testutil::CHCKSI(b"BFF", BFF, b"=", VAXDFL, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    testutil::KILFIL(b"testdas.das", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No FTP String. No BFFID. DAS", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    spicelib::GETLUN(&mut UNIT, ctx)?;

    ARC = DAS;
    BFF = 0;

    testutil::KILFIL(b"testdas.das", ctx)?;

    T_CPTFIL(
        b"testdas.das",
        ARC,
        2,
        b"        ",
        b"ABCD",
        b"EFGH",
        b"IJKL",
        false,
        false,
        b" ",
        ctx,
    )?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(UNIT),
            file: Some(b"testdas.das"),
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
    // Now invoke the module.
    //
    spicelib::ZZDDHPPF(UNIT, ARC, &mut BFF, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Fetch the native environment from ZZPLATFM.
    //
    spicelib::ZZPLATFM(b"FILE_FORMAT", &mut NATIVE, ctx);
    spicelib::UCASE(&NATIVE.clone(), &mut NATIVE, ctx);
    //
    // Convert it to the appropriate integer code.
    //
    NATBFF = spicelib::ISRCHC(&NATIVE, NUMBFF, STRBFF.as_arg());

    //
    // Check BFF for the appropriate value.
    //
    testutil::CHCKSI(b"BFF", BFF, b"=", NATBFF, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    testutil::KILFIL(b"testdas.das", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No FTP String. No BFFID. BIG-IEEE DAF", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    spicelib::GETLUN(&mut UNIT, ctx)?;

    ARC = DAF;
    BFF = 0;

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    fstr::assign(fstr::substr_mut(&mut CNI, 1..=1), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNI, 2..=2), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNI, 3..=3), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNI, 4..=4), &intrinsics::CHAR(128));

    T_CPTFIL(
        b"testdaf.daf",
        ARC,
        2,
        b"       ",
        &CNI,
        b"EFGH",
        b"IJKL",
        false,
        false,
        b" ",
        ctx,
    )?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(UNIT),
            file: Some(b"testdaf.daf"),
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
    // Now invoke the module.
    //
    spicelib::ZZDDHPPF(UNIT, ARC, &mut BFF, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check BFF for the appropriate value.
    //
    testutil::CHCKSI(b"BFF", BFF, b"=", BIGI3E, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"FTP String. Null BFFID. BIG-IEEE DAF", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    spicelib::GETLUN(&mut UNIT, ctx)?;

    ARC = DAF;
    BFF = 0;

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    fstr::assign(fstr::substr_mut(&mut CNI, 1..=1), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNI, 2..=2), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNI, 3..=3), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNI, 4..=4), &intrinsics::CHAR(128));

    T_CPTFIL(
        b"testdaf.daf",
        ARC,
        2,
        &NULBFF,
        &CNI,
        b"EFGH",
        b"IJKL",
        true,
        false,
        b" ",
        ctx,
    )?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(UNIT),
            file: Some(b"testdaf.daf"),
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
    // Now invoke the module.
    //
    spicelib::ZZDDHPPF(UNIT, ARC, &mut BFF, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check BFF for the appropriate value.
    //
    testutil::CHCKSI(b"BFF", BFF, b"=", BIGI3E, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No FTP String. No BFFID. LTL-IEEE DAF", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    spicelib::GETLUN(&mut UNIT, ctx)?;

    ARC = DAF;
    BFF = 0;

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    fstr::assign(fstr::substr_mut(&mut CNI, 1..=1), &intrinsics::CHAR(128));
    fstr::assign(fstr::substr_mut(&mut CNI, 2..=2), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNI, 3..=3), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNI, 4..=4), &NULL);

    //
    // As long as we keep FDREC byte-sized we can do the following.
    //
    FDREC = 12;
    fstr::assign(
        fstr::substr_mut(&mut CFDREC, 1..=1),
        &intrinsics::CHAR(FDREC),
    );
    fstr::assign(fstr::substr_mut(&mut CFDREC, 2..=2), &NULL);
    fstr::assign(fstr::substr_mut(&mut CFDREC, 3..=3), &NULL);
    fstr::assign(fstr::substr_mut(&mut CFDREC, 4..=4), &NULL);

    //
    // Now construct the character NSUM values.  Recall that the first
    // two bytes must be NULL in order to properly represent LTL-IEEE.
    //
    fstr::assign(fstr::substr_mut(&mut CNSUM, 1..=1), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 2..=2), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 3..=3), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 4..=4), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 5..=5), &intrinsics::CHAR(128));
    fstr::assign(fstr::substr_mut(&mut CNSUM, 6..=6), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 7..=7), &intrinsics::CHAR(64));
    fstr::assign(fstr::substr_mut(&mut CNSUM, 8..=8), &intrinsics::CHAR(80));

    T_CPTFIL(
        b"testdaf.daf",
        ARC,
        FDREC,
        b"       ",
        &CNI,
        &CFDREC,
        &CNSUM,
        false,
        false,
        b" ",
        ctx,
    )?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(UNIT),
            file: Some(b"testdaf.daf"),
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
    // Now invoke the module.
    //
    spicelib::ZZDDHPPF(UNIT, ARC, &mut BFF, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check BFF for the appropriate value.
    //
    testutil::CHCKSI(b"BFF", BFF, b"=", LTLI3E, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"FTP String. Null BFFID. LTL-IEEE DAF", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    spicelib::GETLUN(&mut UNIT, ctx)?;

    ARC = DAF;
    BFF = 0;

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    fstr::assign(fstr::substr_mut(&mut CNI, 1..=1), &intrinsics::CHAR(128));
    fstr::assign(fstr::substr_mut(&mut CNI, 2..=2), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNI, 3..=3), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNI, 4..=4), &NULL);

    //
    // As long as we keep FDREC byte-sized we can do the following.
    //
    FDREC = 12;
    fstr::assign(
        fstr::substr_mut(&mut CFDREC, 1..=1),
        &intrinsics::CHAR(FDREC),
    );
    fstr::assign(fstr::substr_mut(&mut CFDREC, 2..=2), &NULL);
    fstr::assign(fstr::substr_mut(&mut CFDREC, 3..=3), &NULL);
    fstr::assign(fstr::substr_mut(&mut CFDREC, 4..=4), &NULL);

    //
    // Now construct the character NSUM values.  Recall that the first
    // two bytes must be NULL in order to properly represent LTL-IEEE.
    //
    fstr::assign(fstr::substr_mut(&mut CNSUM, 1..=1), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 2..=2), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 3..=3), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 4..=4), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 5..=5), &intrinsics::CHAR(128));
    fstr::assign(fstr::substr_mut(&mut CNSUM, 6..=6), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 7..=7), &intrinsics::CHAR(64));
    fstr::assign(fstr::substr_mut(&mut CNSUM, 8..=8), &intrinsics::CHAR(80));

    T_CPTFIL(
        b"testdaf.daf",
        ARC,
        FDREC,
        &NULBFF,
        &CNI,
        &CFDREC,
        &CNSUM,
        true,
        false,
        b" ",
        ctx,
    )?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(UNIT),
            file: Some(b"testdaf.daf"),
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
    // Now invoke the module.
    //
    spicelib::ZZDDHPPF(UNIT, ARC, &mut BFF, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check BFF for the appropriate value.
    //
    testutil::CHCKSI(b"BFF", BFF, b"=", LTLI3E, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No FTP String. No BFFID. VAX-GFLT DAF", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    spicelib::GETLUN(&mut UNIT, ctx)?;

    ARC = DAF;
    BFF = 0;

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    fstr::assign(fstr::substr_mut(&mut CNI, 1..=1), &intrinsics::CHAR(128));
    fstr::assign(fstr::substr_mut(&mut CNI, 2..=2), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNI, 3..=3), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNI, 4..=4), &NULL);

    //
    // As long as we keep FDREC byte-sized we can do the following.
    //
    FDREC = 20;
    fstr::assign(
        fstr::substr_mut(&mut CFDREC, 1..=1),
        &intrinsics::CHAR(FDREC),
    );
    fstr::assign(fstr::substr_mut(&mut CFDREC, 2..=2), &NULL);
    fstr::assign(fstr::substr_mut(&mut CFDREC, 3..=3), &NULL);
    fstr::assign(fstr::substr_mut(&mut CFDREC, 4..=4), &NULL);

    //
    // Now construct the character NSUM values.  Recall that the first
    // two bytes must be NULL in order to properly represent LTL-IEEE.
    //
    fstr::assign(fstr::substr_mut(&mut CNSUM, 1..=1), &intrinsics::CHAR(16));
    fstr::assign(fstr::substr_mut(&mut CNSUM, 2..=2), &intrinsics::CHAR(64));
    fstr::assign(fstr::substr_mut(&mut CNSUM, 3..=3), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 4..=4), &intrinsics::CHAR(64));
    fstr::assign(fstr::substr_mut(&mut CNSUM, 5..=5), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 6..=6), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 7..=7), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 8..=8), &NULL);

    T_CPTFIL(
        b"testdaf.daf",
        ARC,
        FDREC,
        b"       ",
        &CNI,
        &CFDREC,
        &CNSUM,
        false,
        false,
        b" ",
        ctx,
    )?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(UNIT),
            file: Some(b"testdaf.daf"),
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
    // Now invoke the module.
    //
    spicelib::ZZDDHPPF(UNIT, ARC, &mut BFF, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check BFF for the appropriate value.
    //
    testutil::CHCKSI(b"BFF", BFF, b"=", VAXGFL, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"FTP String. Null BFFID. VAX-GFLT DAF", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    spicelib::GETLUN(&mut UNIT, ctx)?;

    ARC = DAF;
    BFF = 0;

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    fstr::assign(fstr::substr_mut(&mut CNI, 1..=1), &intrinsics::CHAR(128));
    fstr::assign(fstr::substr_mut(&mut CNI, 2..=2), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNI, 3..=3), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNI, 4..=4), &NULL);

    //
    // As long as we keep FDREC byte-sized we can do the following.
    //
    FDREC = 20;
    fstr::assign(
        fstr::substr_mut(&mut CFDREC, 1..=1),
        &intrinsics::CHAR(FDREC),
    );
    fstr::assign(fstr::substr_mut(&mut CFDREC, 2..=2), &NULL);
    fstr::assign(fstr::substr_mut(&mut CFDREC, 3..=3), &NULL);
    fstr::assign(fstr::substr_mut(&mut CFDREC, 4..=4), &NULL);

    //
    // Now construct the character NSUM values.  Recall that the first
    // two bytes must be NULL in order to properly represent LTL-IEEE.
    //
    fstr::assign(fstr::substr_mut(&mut CNSUM, 1..=1), &intrinsics::CHAR(16));
    fstr::assign(fstr::substr_mut(&mut CNSUM, 2..=2), &intrinsics::CHAR(64));
    fstr::assign(fstr::substr_mut(&mut CNSUM, 3..=3), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 4..=4), &intrinsics::CHAR(64));
    fstr::assign(fstr::substr_mut(&mut CNSUM, 5..=5), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 6..=6), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 7..=7), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 8..=8), &NULL);

    T_CPTFIL(
        b"testdaf.daf",
        ARC,
        FDREC,
        &NULBFF,
        &CNI,
        &CFDREC,
        &CNSUM,
        true,
        false,
        b" ",
        ctx,
    )?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(UNIT),
            file: Some(b"testdaf.daf"),
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
    // Now invoke the module.
    //
    spicelib::ZZDDHPPF(UNIT, ARC, &mut BFF, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check BFF for the appropriate value.
    //
    testutil::CHCKSI(b"BFF", BFF, b"=", VAXGFL, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No FTP String. No BFFID. VAX-DFLT DAF", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    spicelib::GETLUN(&mut UNIT, ctx)?;

    ARC = DAF;
    BFF = 0;

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    fstr::assign(fstr::substr_mut(&mut CNI, 1..=1), &intrinsics::CHAR(128));
    fstr::assign(fstr::substr_mut(&mut CNI, 2..=2), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNI, 3..=3), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNI, 4..=4), &NULL);

    //
    // As long as we keep FDREC byte-sized we can do the following.
    //
    FDREC = 20;
    fstr::assign(
        fstr::substr_mut(&mut CFDREC, 1..=1),
        &intrinsics::CHAR(FDREC),
    );
    fstr::assign(fstr::substr_mut(&mut CFDREC, 2..=2), &NULL);
    fstr::assign(fstr::substr_mut(&mut CFDREC, 3..=3), &NULL);
    fstr::assign(fstr::substr_mut(&mut CFDREC, 4..=4), &NULL);

    //
    // Now construct the character NSUM values.  Recall that the first
    // two bytes must be NULL in order to properly represent LTL-IEEE.
    //
    fstr::assign(fstr::substr_mut(&mut CNSUM, 1..=1), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 2..=2), &intrinsics::CHAR(65));
    fstr::assign(fstr::substr_mut(&mut CNSUM, 3..=3), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 4..=4), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 5..=5), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 6..=6), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 7..=7), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 8..=8), &NULL);

    T_CPTFIL(
        b"testdaf.daf",
        ARC,
        FDREC,
        b"       ",
        &CNI,
        &CFDREC,
        &CNSUM,
        false,
        false,
        b" ",
        ctx,
    )?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(UNIT),
            file: Some(b"testdaf.daf"),
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
    // Now invoke the module.
    //
    spicelib::ZZDDHPPF(UNIT, ARC, &mut BFF, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check BFF for the appropriate value.
    //
    testutil::CHCKSI(b"BFF", BFF, b"=", VAXDFL, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"FTP String. Null BFFID. VAX-DFLT DAF", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    spicelib::GETLUN(&mut UNIT, ctx)?;

    ARC = DAF;
    BFF = 0;

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    fstr::assign(fstr::substr_mut(&mut CNI, 1..=1), &intrinsics::CHAR(128));
    fstr::assign(fstr::substr_mut(&mut CNI, 2..=2), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNI, 3..=3), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNI, 4..=4), &NULL);

    //
    // As long as we keep FDREC byte-sized we can do the following.
    //
    FDREC = 20;
    fstr::assign(
        fstr::substr_mut(&mut CFDREC, 1..=1),
        &intrinsics::CHAR(FDREC),
    );
    fstr::assign(fstr::substr_mut(&mut CFDREC, 2..=2), &NULL);
    fstr::assign(fstr::substr_mut(&mut CFDREC, 3..=3), &NULL);
    fstr::assign(fstr::substr_mut(&mut CFDREC, 4..=4), &NULL);

    //
    // Now construct the character NSUM values.  Recall that the first
    // two bytes must be NULL in order to properly represent LTL-IEEE.
    //
    fstr::assign(fstr::substr_mut(&mut CNSUM, 1..=1), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 2..=2), &intrinsics::CHAR(65));
    fstr::assign(fstr::substr_mut(&mut CNSUM, 3..=3), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 4..=4), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 5..=5), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 6..=6), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 7..=7), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 8..=8), &NULL);

    T_CPTFIL(
        b"testdaf.daf",
        ARC,
        FDREC,
        &NULBFF,
        &CNI,
        &CFDREC,
        &CNSUM,
        true,
        false,
        b" ",
        ctx,
    )?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(UNIT),
            file: Some(b"testdaf.daf"),
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
    // Now invoke the module.
    //
    spicelib::ZZDDHPPF(UNIT, ARC, &mut BFF, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check BFF for the appropriate value.
    //
    testutil::CHCKSI(b"BFF", BFF, b"=", VAXDFL, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No FTP String. No BFFID. VAX-DFLT (Exception) DAF", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    spicelib::GETLUN(&mut UNIT, ctx)?;

    ARC = DAF;
    BFF = 0;

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    fstr::assign(fstr::substr_mut(&mut CNI, 1..=1), &intrinsics::CHAR(128));
    fstr::assign(fstr::substr_mut(&mut CNI, 2..=2), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNI, 3..=3), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNI, 4..=4), &NULL);

    //
    // As long as we keep FDREC byte-sized we can do the following.
    //
    FDREC = 20;
    fstr::assign(
        fstr::substr_mut(&mut CFDREC, 1..=1),
        &intrinsics::CHAR(FDREC),
    );
    fstr::assign(fstr::substr_mut(&mut CFDREC, 2..=2), &NULL);
    fstr::assign(fstr::substr_mut(&mut CFDREC, 3..=3), &NULL);
    fstr::assign(fstr::substr_mut(&mut CFDREC, 4..=4), &NULL);

    //
    // Now construct the character NSUM values.  Recall that the first
    // two bytes must be NULL in order to properly represent LTL-IEEE.
    //
    fstr::assign(fstr::substr_mut(&mut CNSUM, 1..=1), &intrinsics::CHAR(128));
    fstr::assign(fstr::substr_mut(&mut CNSUM, 2..=2), &intrinsics::CHAR(64));
    fstr::assign(fstr::substr_mut(&mut CNSUM, 3..=3), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 4..=4), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 5..=5), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 6..=6), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 7..=7), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 8..=8), &NULL);

    T_CPTFIL(
        b"testdaf.daf",
        ARC,
        FDREC,
        b"       ",
        &CNI,
        &CFDREC,
        &CNSUM,
        false,
        false,
        b" ",
        ctx,
    )?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(UNIT),
            file: Some(b"testdaf.daf"),
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
    // Now invoke the module.
    //
    spicelib::ZZDDHPPF(UNIT, ARC, &mut BFF, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check BFF for the appropriate value.
    //
    testutil::CHCKSI(b"BFF", BFF, b"=", VAXDFL, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"FTP String. Null BFFID. VAX-DFLT (Exception) DAF", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    spicelib::GETLUN(&mut UNIT, ctx)?;

    ARC = DAF;
    BFF = 0;

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    fstr::assign(fstr::substr_mut(&mut CNI, 1..=1), &intrinsics::CHAR(128));
    fstr::assign(fstr::substr_mut(&mut CNI, 2..=2), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNI, 3..=3), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNI, 4..=4), &NULL);

    //
    // As long as we keep FDREC byte-sized we can do the following.
    //
    FDREC = 20;
    fstr::assign(
        fstr::substr_mut(&mut CFDREC, 1..=1),
        &intrinsics::CHAR(FDREC),
    );
    fstr::assign(fstr::substr_mut(&mut CFDREC, 2..=2), &NULL);
    fstr::assign(fstr::substr_mut(&mut CFDREC, 3..=3), &NULL);
    fstr::assign(fstr::substr_mut(&mut CFDREC, 4..=4), &NULL);

    //
    // Now construct the character NSUM values.  Recall that the first
    // two bytes must be NULL in order to properly represent LTL-IEEE.
    //
    fstr::assign(fstr::substr_mut(&mut CNSUM, 1..=1), &intrinsics::CHAR(128));
    fstr::assign(fstr::substr_mut(&mut CNSUM, 2..=2), &intrinsics::CHAR(64));
    fstr::assign(fstr::substr_mut(&mut CNSUM, 3..=3), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 4..=4), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 5..=5), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 6..=6), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 7..=7), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 8..=8), &NULL);

    T_CPTFIL(
        b"testdaf.daf",
        ARC,
        FDREC,
        &NULBFF,
        &CNI,
        &CFDREC,
        &CNSUM,
        true,
        false,
        b" ",
        ctx,
    )?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(UNIT),
            file: Some(b"testdaf.daf"),
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
    // Now invoke the module.
    //
    spicelib::ZZDDHPPF(UNIT, ARC, &mut BFF, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check BFF for the appropriate value.
    //
    testutil::CHCKSI(b"BFF", BFF, b"=", VAXDFL, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No FTP String. No BFFID. Zero-Sized DAF", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    spicelib::GETLUN(&mut UNIT, ctx)?;

    ARC = DAF;
    BFF = -1;

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    fstr::assign(fstr::substr_mut(&mut CNI, 1..=1), &intrinsics::CHAR(128));
    fstr::assign(fstr::substr_mut(&mut CNI, 2..=2), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNI, 3..=3), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNI, 4..=4), &NULL);

    //
    // As long as we keep FDREC byte-sized we can do the following.
    //
    FDREC = 20;
    fstr::assign(
        fstr::substr_mut(&mut CFDREC, 1..=1),
        &intrinsics::CHAR(FDREC),
    );
    fstr::assign(fstr::substr_mut(&mut CFDREC, 2..=2), &NULL);
    fstr::assign(fstr::substr_mut(&mut CFDREC, 3..=3), &NULL);
    fstr::assign(fstr::substr_mut(&mut CFDREC, 4..=4), &NULL);

    //
    // Now construct the character NSUM values.  Recall that the first
    // two bytes must be NULL in order to properly represent LTL-IEEE.
    //
    fstr::assign(fstr::substr_mut(&mut CNSUM, 1..=1), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 2..=2), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 3..=3), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 4..=4), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 5..=5), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 6..=6), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 7..=7), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 8..=8), &NULL);

    T_CPTFIL(
        b"testdaf.daf",
        ARC,
        FDREC,
        b"       ",
        &CNI,
        &CFDREC,
        &CNSUM,
        false,
        false,
        b" ",
        ctx,
    )?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(UNIT),
            file: Some(b"testdaf.daf"),
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
    // Now invoke the module.
    //
    spicelib::ZZDDHPPF(UNIT, ARC, &mut BFF, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(true, b"SPICE(UNKNOWNBFF)", OK, ctx)?;

    //
    // Check BFF for the appropriate value.
    //
    testutil::CHCKSI(b"BFF", BFF, b"=", 0, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No FTP String. No BFFID. Fall-through ARCH DAF", ctx)?;

    //
    // Setup the inputs and outputs.
    //
    spicelib::GETLUN(&mut UNIT, ctx)?;

    ARC = DAF;
    BFF = -1;

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    fstr::assign(fstr::substr_mut(&mut CNI, 1..=1), &intrinsics::CHAR(128));
    fstr::assign(fstr::substr_mut(&mut CNI, 2..=2), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNI, 3..=3), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNI, 4..=4), &NULL);

    //
    // As long as we keep FDREC byte-sized we can do the following.
    //
    FDREC = 20;
    fstr::assign(
        fstr::substr_mut(&mut CFDREC, 1..=1),
        &intrinsics::CHAR(FDREC),
    );
    fstr::assign(fstr::substr_mut(&mut CFDREC, 2..=2), &NULL);
    fstr::assign(fstr::substr_mut(&mut CFDREC, 3..=3), &NULL);
    fstr::assign(fstr::substr_mut(&mut CFDREC, 4..=4), &NULL);

    //
    // Now construct the character NSUM values.  Recall that the first
    // two bytes must be NULL in order to properly represent LTL-IEEE.
    //
    fstr::assign(fstr::substr_mut(&mut CNSUM, 1..=1), &intrinsics::CHAR(144));
    fstr::assign(fstr::substr_mut(&mut CNSUM, 2..=2), &intrinsics::CHAR(64));
    fstr::assign(fstr::substr_mut(&mut CNSUM, 3..=3), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 4..=4), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 5..=5), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 6..=6), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 7..=7), &NULL);
    fstr::assign(fstr::substr_mut(&mut CNSUM, 8..=8), &NULL);

    T_CPTFIL(
        b"testdaf.daf",
        ARC,
        FDREC,
        b"       ",
        &CNI,
        &CFDREC,
        &CNSUM,
        false,
        false,
        b" ",
        ctx,
    )?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(UNIT),
            file: Some(b"testdaf.daf"),
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
    // Now invoke the module.
    //
    spicelib::ZZDDHPPF(UNIT, ARC, &mut BFF, ctx)?;

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(true, b"SPICE(UNKNOWNBFF)", OK, ctx)?;

    //
    // Check BFF for the appropriate value.
    //
    testutil::CHCKSI(b"BFF", BFF, b"=", 0, 0, OK, ctx)?;

    //
    // Close and remove the file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    testutil::KILFIL(b"testdaf.daf", ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
