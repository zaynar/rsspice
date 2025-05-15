//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const RECL: i32 = 1024;
const BODID: i32 = 1;
const CENID: i32 = (BODID + 1);
const SPKFRM: i32 = (CENID + 1);
const SPKTYP: i32 = (SPKFRM + 1);
const START: i32 = (SPKTYP + 1);
const FINISH: i32 = (START + 1);
const CKRATE: i32 = SPKTYP;
const ND: i32 = 2;
const NI: i32 = 6;
const SUMSIZ: i32 = (ND + ((NI + 1) / 2));
const IDLEN: i32 = 12;
const MINPCH: i32 = 32;
const MAXPCH: i32 = 126;

//$Procedure ZZGETFAT ( Get file architecture, type, and unit )
pub fn ZZGETFAT(
    FILE: &[u8],
    ARCH: &mut [u8],
    TYPE: &mut [u8],
    NUMBER: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut IDWORD = [b' '; IDLEN as usize];
    let mut TMPWRD = [b' '; IDLEN as usize];
    let mut IOSTAT: i32 = 0;
    let mut DIROPN: bool = false;
    let mut SEQOPN: bool = false;
    let mut CHECK: bool = false;
    let mut EXIST: bool = false;
    let mut OPENED: bool = false;

    //

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    //
    // The following parameters point to the various slots in the
    // integer portion of the DAF descriptor where the values are
    // located.
    //

    //
    // These parameters give the number of integer and double precision
    // components of the descriptor for SPK and CK files.
    //

    //
    // The size of a summary.
    //

    //
    // Set the length of a SPICE kernel file ID word.
    //
    //
    // Set minimum and maximum values for the range of ASCII printing
    // characters.
    //

    //
    // Local Variables
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"ZZGETFAT", ctx)?;
    }

    //
    // Initialize the temporary storage variables that we use.
    //
    fstr::assign(&mut IDWORD, b" ");
    SEQOPN = false;
    CHECK = true;

    //
    // If the filename we got is blank, signal an error and return.
    //
    if fstr::eq(FILE, b" ") {
        spicelib::SETMSG(b"The file name is blank.", ctx);
        spicelib::SIGERR(b"SPICE(BLANKFILENAME)", ctx)?;
        spicelib::CHKOUT(b"ZZGETFAT", ctx)?;
        return Ok(());
    }

    //
    // We'll do a bit of inquiring before we try opening anything.
    //
    {
        use f2rust_std::io;

        let specs = io::InquireSpecs {
            file: Some(FILE),
            exist: Some(&mut EXIST),
            opened: Some(&mut OPENED),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.inquire(specs))?;
    }

    //
    // Not too likely, but if the INQUIRE statement fails...
    //
    if (IOSTAT != 0) {
        spicelib::SETMSG(b"IOSTAT error in INQUIRE statement. IOSTAT = #.", ctx);
        spicelib::ERRINT(b"#", IOSTAT, ctx);
        spicelib::SIGERR(b"SPICE(INQUIREERROR)", ctx)?;
        spicelib::CHKOUT(b"ZZGETFAT", ctx)?;
        return Ok(());
    }
    //
    // Note: the following two tests MUST be performed in the order in
    //       which they appear, since in some environments files that do
    //       not exist are considered to be open.
    //
    // By calling this routine, the user implies that the file exists.
    //
    if !EXIST {
        spicelib::SETMSG(b"The kernel file \'#\' does not exist.", ctx);
        spicelib::ERRCH(b"#", FILE, ctx);
        spicelib::SIGERR(b"SPICE(NOSUCHFILE)", ctx)?;
        spicelib::CHKOUT(b"ZZGETFAT", ctx)?;
        return Ok(());
    }

    //
    // This routine should not be called if the file is already open.
    //
    if OPENED {
        spicelib::SETMSG(b"The kernel file \'#\' is already open.", ctx);
        spicelib::ERRCH(b"#", FILE, ctx);
        spicelib::SIGERR(b"SPICE(FILECURRENTLYOPEN)", ctx)?;
        spicelib::CHKOUT(b"ZZGETFAT", ctx)?;
        return Ok(());
    }

    //
    // Open the file with a record length of RECL (the length of the
    // DAF and DAS records). We assume, for now, that opening the file as
    // a direct access file will work.
    //

    DIROPN = true;

    spicelib::GETLUN(NUMBER, ctx)?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(*NUMBER),
            file: Some(FILE),
            access: Some(b"DIRECT"),
            recl: Some(RECL),
            status: Some(b"OLD"),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
    }

    //
    // If we had trouble opening the file, try opening it as a sequential
    // file.
    //
    if (IOSTAT != 0) {
        DIROPN = false;

        {
            use f2rust_std::io;

            let specs = io::OpenSpecs {
                unit: Some(*NUMBER),
                file: Some(FILE),
                access: Some(b"SEQUENTIAL"),
                status: Some(b"OLD"),
                ..Default::default()
            };
            IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
        }
        //
        // If we still have problems opening the file, we don't have a
        // clue about the file architecture and type.
        //
        if (IOSTAT != 0) {
            fstr::assign(ARCH, b"?");
            fstr::assign(TYPE, b"?");
            spicelib::SETMSG(b"Attempt to open the file \'#\' failed. IOSTAT = #.", ctx);
            spicelib::ERRCH(b"#", FILE, ctx);
            spicelib::ERRINT(b"#", IOSTAT, ctx);
            spicelib::SIGERR(b"SPICE(FILEOPENFAILED)", ctx)?;
            spicelib::CHKOUT(b"ZZGETFAT", ctx)?;
            return Ok(());
        }
    }

    //
    // We opened the file successfully, so let's try to read from the
    // file. We need to be sure to use the correct form of the read
    // statement, depending on whether the file was opened with direct
    // access or sequential access.
    //
    if DIROPN {
        {
            use f2rust_std::{
                data::Val,
                io::{self, Reader},
            };

            let mut reader = io::UnformattedReader::new(ctx.io_unit(*NUMBER)?, Some(1))?;
            IOSTAT = io::capture_iostat(|| {
                reader.start()?;
                reader.read_str(&mut TMPWRD)?;
                reader.finish()?;
                Ok(())
            })?;
        }
        //
        // If we couldn't read from the file as a direct access file with
        // a fixed record length, then try to open the file as a
        // sequential file and read from it.
        //
        if (IOSTAT == 0) {
            SEQOPN = true;
            DIROPN = false;

            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(*NUMBER),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }

            {
                use f2rust_std::io;

                let specs = io::OpenSpecs {
                    unit: Some(*NUMBER),
                    file: Some(FILE),
                    access: Some(b"SEQUENTIAL"),
                    status: Some(b"OLD"),
                    ..Default::default()
                };
                IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
            }
            //
            // If we could not open the file, we don't have a clue about
            // the file architecture and type.
            //
            if (IOSTAT != 0) {
                fstr::assign(ARCH, b"?");
                fstr::assign(TYPE, b"?");
                spicelib::SETMSG(b"Attempt to open the file \'#\' failed. IOSTAT = #.", ctx);
                spicelib::ERRCH(b"#", FILE, ctx);
                spicelib::ERRINT(b"#", IOSTAT, ctx);
                spicelib::SIGERR(b"SPICE(FILEOPENFAILED)", ctx)?;
                spicelib::CHKOUT(b"ZZGETFAT", ctx)?;
                return Ok(());
            }
            //
            // Try to read from the file.
            //
            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Reader},
                };

                let mut reader = io::FormattedReader::new(ctx.io_unit(*NUMBER)?, None, b"(A)")?;
                IOSTAT = io::capture_iostat(|| {
                    reader.start()?;
                    reader.read_str(&mut TMPWRD)?;
                    reader.finish()?;
                    Ok(())
                })?;
            }
        }
    } else {
        SEQOPN = true;
        {
            use f2rust_std::{
                data::Val,
                io::{self, Reader},
            };

            let mut reader = io::FormattedReader::new(ctx.io_unit(*NUMBER)?, None, b"(A)")?;
            IOSTAT = io::capture_iostat(|| {
                reader.start()?;
                reader.read_str(&mut TMPWRD)?;
                reader.finish()?;
                Ok(())
            })?;
        }
    }

    //
    // If we had an error while reading, we don't recognize this file.
    //
    if (IOSTAT != 0) {
        fstr::assign(ARCH, b"?");
        fstr::assign(TYPE, b"?");
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(*NUMBER),
                ..Default::default()
            };
            ctx.close(specs)?;
        }
        spicelib::SETMSG(b"Attempt to read from file \'#\' failed. IOSTAT = #.", ctx);
        spicelib::ERRCH(b"#", FILE, ctx);
        spicelib::ERRINT(b"#", IOSTAT, ctx);
        spicelib::SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
        spicelib::CHKOUT(b"ZZGETFAT", ctx)?;
        return Ok(());
    }

    //
    // Loop until a known NAIF file ID word is found.
    //
    while CHECK {
        //
        // At this point, we have a candidate for an ID word. To avoid
        // difficulties with Fortran I/O and other things, we will now
        // replace any non printing ASCII characters with blanks.
        //
        for I in 1..=IDLEN {
            if ((intrinsics::ICHAR(fstr::substr(&TMPWRD, I..=I)) < MINPCH)
                || (intrinsics::ICHAR(fstr::substr(&TMPWRD, I..=I)) > MAXPCH))
            {
                fstr::assign(fstr::substr_mut(&mut TMPWRD, I..=I), b" ");
            }
        }

        //
        // Identify the architecture and type, if we can.
        //
        spicelib::LJUST(&TMPWRD.clone(), &mut TMPWRD);
        spicelib::UCASE(&TMPWRD.clone(), &mut TMPWRD, ctx);
        spicelib::NEXTWD(&TMPWRD.clone(), &mut IDWORD, &mut TMPWRD);

        if fstr::eq(&IDWORD, b"DAFETF") {
            //
            // We have a DAF encoded transfer file.
            //
            fstr::assign(ARCH, b"XFR");
            fstr::assign(TYPE, b"DAF");
            CHECK = false;
        } else if fstr::eq(&IDWORD, b"DASETF") {
            //
            // We have a DAS encoded transfer file.
            //
            fstr::assign(ARCH, b"XFR");
            fstr::assign(TYPE, b"DAS");
            CHECK = false;
        } else if fstr::eq(fstr::substr(&IDWORD, 1..=10), b"\'NAIF/DAF\'") {
            //
            // We have an old DAF decimal text file.
            //
            fstr::assign(ARCH, b"DEC");
            fstr::assign(TYPE, b"DAF");
            CHECK = false;
        } else if fstr::eq(fstr::substr(&IDWORD, 1..=8), b"NAIF/DAS") {
            //
            // We have a pre release DAS binary file.
            //
            fstr::assign(ARCH, b"DAS");
            fstr::assign(TYPE, b"PRE");
            CHECK = false;
        } else {
            //
            // Get the architecture and type from the ID word, if we can.
            //
            spicelib::IDW2AT(fstr::substr(&IDWORD, 1..=8), ARCH, TYPE, ctx)?;

            if (fstr::eq(ARCH, b"DAF") && fstr::eq(TYPE, b"?")) {
                CHECK = false;
            } else {
                //
                // No identification on line.  Read another line.
                //
                if SEQOPN {
                    {
                        use f2rust_std::{
                            data::Val,
                            io::{self, Reader},
                        };

                        let mut reader =
                            io::FormattedReader::new(ctx.io_unit(*NUMBER)?, None, b"(A)")?;
                        IOSTAT = io::capture_iostat(|| {
                            reader.start()?;
                            reader.read_str(&mut TMPWRD)?;
                            reader.finish()?;
                            Ok(())
                        })?;
                    }
                } else {
                    {
                        use f2rust_std::{
                            data::Val,
                            io::{self, Reader},
                        };

                        let mut reader =
                            io::UnformattedReader::new(ctx.io_unit(*NUMBER)?, Some(1))?;
                        IOSTAT = io::capture_iostat(|| {
                            reader.start()?;
                            reader.read_str(&mut TMPWRD)?;
                            reader.finish()?;
                            Ok(())
                        })?;
                    }
                }

                //
                // If IOSTAT is a negative value, we probably hit an
                // end-of-file.  Error out.
                //
                if (IOSTAT < 0) {
                    fstr::assign(ARCH, b"?");
                    fstr::assign(TYPE, b"?");
                    {
                        use f2rust_std::io;

                        let specs = io::CloseSpecs {
                            unit: Some(*NUMBER),
                            ..Default::default()
                        };
                        ctx.close(specs)?;
                    }
                    spicelib::SETMSG(
                        b"Encountered end-of-file of # before  finding known SPICE ID word.",
                        ctx,
                    );
                    spicelib::ERRCH(b"#", FILE, ctx);
                    spicelib::SIGERR(b"SPICE(ENDOFFILE)", ctx)?;
                    spicelib::CHKOUT(b"ZZGETFAT", ctx)?;
                    return Ok(());
                }
            }
        }
    }

    spicelib::CHKOUT(b"ZZGETFAT", ctx)?;
    Ok(())
}
