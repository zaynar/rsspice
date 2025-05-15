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
const FMTLEN: i32 = 8;
const IDWLEN: i32 = 8;
const IFNLEN: i32 = 60;
const TAILEN: i32 = 932;

//$Procedure F_DASFR ( DASFR Test Family )
pub fn F_DASFR(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut FORMAT = [b' '; FMTLEN as usize];
    let mut SYSFMT = [b' '; FMTLEN as usize];
    let mut FNAME = [b' '; FILEN as usize];
    let mut IDWORD = [b' '; IDWLEN as usize];
    let mut IFNAME = [b' '; IFNLEN as usize];
    let mut TAIL = [b' '; TAILEN as usize];
    let mut HANDLE: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut LUN: i32 = 0;
    let mut NRESVR: i32 = 0;
    let mut NRESVC: i32 = 0;
    let mut NCOMR: i32 = 0;
    let mut NCOMC: i32 = 0;
    let mut FTPERR: bool = false;

    //
    // SPICELIB Functions
    //

    //
    // Local Parameters
    //

    //
    // See DASWFR for a detailed explanation of TAILEN.
    //

    //
    // Local Variables
    //

    //
    // Start the test family with an open call.
    //
    testutil::TOPEN(b"F_DASFR", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZDASNFR Test", ctx)?;

    //
    // Setup the file.
    //
    fstr::assign(&mut FNAME, b"test.das");
    testutil::KILFIL(&FNAME, ctx)?;

    spicelib::GETLUN(&mut LUN, ctx)?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(LUN),
            file: Some(fstr::substr(&FNAME, 1..=spicelib::RTRIM(&FNAME))),
            access: Some(b"DIRECT"),
            recl: Some(RECL),
            status: Some(b"NEW"),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
    }

    spicelib::ZZDASNFR(
        LUN,
        b"DAS/TEST",
        b"TEST DAS",
        -1,
        1,
        -2,
        2,
        b"BFFIDWRD",
        ctx,
    )?;

    //
    // Initialize the values we are about to read.
    //
    fstr::assign(&mut IDWORD, b" ");
    fstr::assign(&mut IFNAME, b" ");
    NRESVR = 0;
    NRESVC = 0;
    NCOMR = 0;
    NCOMC = 0;
    fstr::assign(&mut FORMAT, b" ");
    fstr::assign(&mut TAIL, b" ");

    {
        use f2rust_std::{
            data::Val,
            io::{self, Reader},
        };

        let mut reader = io::UnformattedReader::new(ctx.io_unit(LUN)?, Some(1))?;
        IOSTAT = io::capture_iostat(|| {
            reader.start()?;
            reader.read_str(&mut IDWORD)?;
            reader.read_str(&mut IFNAME)?;
            NRESVR = reader.read_i32()?;
            NRESVC = reader.read_i32()?;
            NCOMR = reader.read_i32()?;
            NCOMC = reader.read_i32()?;
            reader.read_str(&mut FORMAT)?;
            reader.read_str(&mut TAIL)?;
            reader.finish()?;
            Ok(())
        })?;
    }

    //
    // Check the results.
    //
    testutil::CHCKSI(b"IOSTAT", IOSTAT, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSC(b"IDWORD", &IDWORD, b"=", b"DAS/TEST", OK, ctx)?;
    testutil::CHCKSC(b"IFNAME", &IFNAME, b"=", b"TEST DAS", OK, ctx)?;
    testutil::CHCKSI(b"NRESVR", NRESVR, b"=", -1, 0, OK, ctx)?;
    testutil::CHCKSI(b"NRESVC", NRESVC, b"=", 1, 0, OK, ctx)?;
    testutil::CHCKSI(b"NCOMR", NCOMR, b"=", -2, 0, OK, ctx)?;
    testutil::CHCKSI(b"NCOMC", NCOMC, b"=", 2, 0, OK, ctx)?;
    testutil::CHCKSC(b"FORMAT", &FORMAT, b"=", b"BFFIDWRD", OK, ctx)?;

    spicelib::ZZFTPCHK(&TAIL, &mut FTPERR, ctx);

    testutil::CHCKSL(b"FTPERR", FTPERR, false, OK, ctx)?;

    testutil::CHCKSC(
        b"FTPSTR",
        fstr::substr(&TAIL, 608..=613),
        b"=",
        b"FTPSTR",
        OK,
        ctx,
    )?;

    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(LUN),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"DASOPN Test", ctx)?;

    fstr::assign(&mut FNAME, b"test.das");

    testutil::KILFIL(&FNAME, ctx)?;

    spicelib::DASOPN(&FNAME, b"TEST DASOPN", &mut HANDLE, ctx)?;

    spicelib::DASRFR(
        HANDLE,
        &mut IDWORD,
        &mut IFNAME,
        &mut NRESVR,
        &mut NRESVC,
        &mut NCOMR,
        &mut NCOMC,
        ctx,
    )?;

    //
    // Check the results.  We don't need to worry about FORMAT or
    // the FTP string since DASOPN invokes ZZDASNFR which we just
    // checked.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(b"IDWORD", &IDWORD, b"=", b"NAIF/DAS", OK, ctx)?;
    testutil::CHCKSC(b"IFNAME", &IFNAME, b"=", b"TEST DASOPN", OK, ctx)?;
    testutil::CHCKSI(b"NRESVR", NRESVR, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"NRESVC", NRESVC, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"NCOMR", NCOMR, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"NCOMC", NCOMC, b"=", 0, 0, OK, ctx)?;

    spicelib::DASCLS(HANDLE, ctx)?;

    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"DASONW Test", ctx)?;

    fstr::assign(&mut FNAME, b"test.das");

    testutil::KILFIL(&FNAME, ctx)?;

    spicelib::DASONW(&FNAME, b"TEST", b"TEST DASONW", 5, &mut HANDLE, ctx)?;

    spicelib::DASRFR(
        HANDLE,
        &mut IDWORD,
        &mut IFNAME,
        &mut NRESVR,
        &mut NRESVC,
        &mut NCOMR,
        &mut NCOMC,
        ctx,
    )?;

    //
    // Check the results.  We don't need to worry about FORMAT or
    // the FTP string since DASONW invokes ZZDASNFR which we just
    // checked.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(b"IDWORD", &IDWORD, b"=", b"DAS/TEST", OK, ctx)?;
    testutil::CHCKSC(b"IFNAME", &IFNAME, b"=", b"TEST DASONW", OK, ctx)?;
    testutil::CHCKSI(b"NRESVR", NRESVR, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"NRESVC", NRESVC, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"NCOMR", NCOMR, b"=", 5, 0, OK, ctx)?;
    testutil::CHCKSI(b"NCOMC", NCOMC, b"=", 0, 0, OK, ctx)?;

    spicelib::DASCLS(HANDLE, ctx)?;

    testutil::KILFIL(&FNAME, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"DASWFR Test", ctx)?;

    fstr::assign(&mut FNAME, b"test.das");

    testutil::KILFIL(&FNAME, ctx)?;

    spicelib::DASONW(&FNAME, b"TEST", b"TEST DASWFR", 5, &mut HANDLE, ctx)?;

    spicelib::DASWFR(HANDLE, b"DAS/WORD", b"DASWFR UPDATE", 2, 4, 6, 8, ctx)?;

    //
    // Check the results.
    //
    spicelib::DASRFR(
        HANDLE,
        &mut IDWORD,
        &mut IFNAME,
        &mut NRESVR,
        &mut NRESVC,
        &mut NCOMR,
        &mut NCOMC,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(b"IDWORD", &IDWORD, b"=", b"DAS/WORD", OK, ctx)?;
    testutil::CHCKSC(b"IFNAME", &IFNAME, b"=", b"DASWFR UPDATE", OK, ctx)?;
    testutil::CHCKSI(b"NRESVR", NRESVR, b"=", 2, 0, OK, ctx)?;
    testutil::CHCKSI(b"NRESVC", NRESVC, b"=", 4, 0, OK, ctx)?;
    testutil::CHCKSI(b"NCOMR", NCOMR, b"=", 6, 0, OK, ctx)?;
    testutil::CHCKSI(b"NCOMC", NCOMC, b"=", 8, 0, OK, ctx)?;

    //
    // Check that DASWFR perserved the binary file format and FTP
    // strings.
    //
    spicelib::DASHLU(HANDLE, &mut LUN, ctx)?;

    {
        use f2rust_std::{
            data::Val,
            io::{self, Reader},
        };

        let mut reader = io::UnformattedReader::new(ctx.io_unit(LUN)?, Some(1))?;
        IOSTAT = io::capture_iostat(|| {
            reader.start()?;
            reader.read_str(&mut IDWORD)?;
            reader.read_str(&mut IFNAME)?;
            NRESVR = reader.read_i32()?;
            NRESVC = reader.read_i32()?;
            NCOMR = reader.read_i32()?;
            NCOMC = reader.read_i32()?;
            reader.read_str(&mut FORMAT)?;
            reader.read_str(&mut TAIL)?;
            reader.finish()?;
            Ok(())
        })?;
    }

    //
    // Check the results.
    //
    testutil::CHCKSI(b"IOSTAT", IOSTAT, b"=", 0, 0, OK, ctx)?;

    spicelib::ZZPLATFM(b"FILE_FORMAT", &mut SYSFMT, ctx);
    testutil::CHCKSC(b"FORMAT", &FORMAT, b"=", &SYSFMT, OK, ctx)?;

    spicelib::ZZFTPCHK(&TAIL, &mut FTPERR, ctx);
    testutil::CHCKSL(b"FTPERR", FTPERR, false, OK, ctx)?;
    testutil::CHCKSC(
        b"FTPSTR",
        fstr::substr(&TAIL, 608..=613),
        b"=",
        b"FTPSTR",
        OK,
        ctx,
    )?;

    spicelib::DASCLS(HANDLE, ctx)?;

    testutil::KILFIL(&FNAME, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
