//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const BCMARK: &[u8] = b"~NAIF/SPC BEGIN COMMENTS~";
const ECMARK: &[u8] = b"~NAIF/SPC END COMMENTS~";
const LINLEN: i32 = 255;
const ARCHLN: i32 = 3;
const TYPELN: i32 = 4;
const RESREC: i32 = 0;

//$ Procedure CONVTB ( Convert kernel file from text to binary )
pub fn CONVTB(TXTFIL: &[u8], BINFIL: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ARCH = [b' '; ARCHLN as usize];
    let mut LINE = [b' '; LINLEN as usize];
    let mut TYPE = [b' '; TYPELN as usize];
    let mut HANDLE: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut SCRLUN: i32 = 0;
    let mut TXTLUN: i32 = 0;
    let mut HAVCOM: bool = false;
    let mut EOC: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    // Begin and end markers in the file for the comment area.
    //

    //
    // Maximum length of an input text line.
    //
    //
    // Maximum length of a file architecture.
    //
    //
    // Maximum length of a file type.
    //
    //
    // Number of reserved records to use when creating a binar DAF file.
    //
    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"CONVTB", ctx)?;
    }
    //
    // Get the architecture and type of the file to be converted.
    //
    spicelib::GETFAT(TXTFIL, &mut ARCH, &mut TYPE, ctx)?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"CONVTB", ctx)?;
        return Ok(());
    }
    //
    // Verify the architecture and type of the file, and perform any
    // processing necessary..
    //
    if (fstr::eq(&ARCH, b"XFR") || fstr::eq(&ARCH, b"DEC")) {
        //
        // Open the text file that is to be converted to binary.
        //
        spicelib::TXTOPR(TXTFIL, &mut TXTLUN, ctx)?;

        if spicelib::FAILED(ctx) {
            spicelib::CHKOUT(b"CONVTB", ctx)?;
            return Ok(());
        }
        //
        // Read the information line to skip it. We already know the
        // architecture and type of the file.
        //
        {
            use f2rust_std::{
                data::Val,
                io::{self, Reader},
            };

            let mut reader = io::FormattedReader::new(ctx.io_unit(TXTLUN)?, None, b"(A)")?;
            IOSTAT = io::capture_iostat(|| {
                reader.start()?;
                reader.read_str(&mut LINE)?;
                reader.finish()?;
                Ok(())
            })?;
        }

        if (IOSTAT != 0) {
            //
            // If there was an error then we need to close the text
            // file, and then check out and return to the caller.
            //
            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(TXTLUN),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }
            spicelib::SETMSG(b"Error reading the text file: #. IOSTAT =  #.", ctx);
            spicelib::ERRCH(b"#", TXTFIL, ctx);
            spicelib::ERRINT(b"#", IOSTAT, ctx);
            spicelib::SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
            spicelib::CHKOUT(b"CONVTB", ctx)?;
            return Ok(());
        }
    }
    //
    // Process the file based on the derived architecture and type.
    //
    if (fstr::eq(&ARCH, b"XFR") && fstr::eq(&TYPE, b"DAF")) {
        //
        // We got a DAF file.
        //
        // Convert the data portion of the text file to binary. At this
        // point, we know that we have a current DAF text file format.
        //
        // We expect to have comments.
        //
        HAVCOM = true;
        //
        // Convert it.
        //
        spicelib::DAFTB(TXTLUN, BINFIL, ctx)?;

        if spicelib::FAILED(ctx) {
            //
            // If there was an error then we need to close the
            // text file, and then check out and return to the
            // caller.
            //
            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(TXTLUN),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }
            spicelib::CHKOUT(b"CONVTB", ctx)?;
            return Ok(());
        }
    } else if (fstr::eq(&ARCH, b"XFR") && fstr::eq(&TYPE, b"DAS")) {
        //
        // We got a DAS file. So we should begin converting it to binary.
        // DAS files are easier: all we do is call one routine.
        //
        // We do not have comments. Actually, we might but they are
        // included as part of the DAS file conversion process.
        //
        HAVCOM = false;
        //
        // Convert it.
        //
        spicelib::DASTB(TXTLUN, BINFIL, ctx)?;

        if spicelib::FAILED(ctx) {
            //
            // If there was an error then we need to close the
            // text file, and then check out and return to the
            // caller.
            //
            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(TXTLUN),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }
            spicelib::CHKOUT(b"CONVTB", ctx)?;
            return Ok(());
        }
    } else if fstr::eq(&ARCH, b"DAS") {
        //
        // This is an error case, most likely caused by reading a binary
        // DAS file by accident. So signal an appropriate error.
        //
        spicelib::SETMSG(
            b"The file \'#\' appears to be a binary DAS file and not a transfer file.",
            ctx,
        );
        spicelib::ERRCH(b"#", TXTFIL, ctx);
        spicelib::SIGERR(b"SPICE(NOTATRANSFERFILE)", ctx)?;
        spicelib::CHKOUT(b"CONVTB", ctx)?;
        return Ok(());
    } else if (fstr::eq(&ARCH, b"DAS") && fstr::eq(&TYPE, b"PRE")) {
        //
        // This is an error case, most likely caused by reading a binary
        // DAS file by accident. So signal an appropriate error.
        //
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(TXTLUN),
                ..Default::default()
            };
            ctx.close(specs)?;
        }
        spicelib::SETMSG(
            b"The file \'#\' appears to be a pre-release binary DAS file and not a transfer file.",
            ctx,
        );
        spicelib::ERRCH(b"#", TXTFIL, ctx);
        spicelib::SIGERR(b"SPICE(NOTATRANSFERFILE)", ctx)?;
        spicelib::CHKOUT(b"CONVTB", ctx)?;
        return Ok(());
    } else if fstr::eq(&ARCH, b"DAF") {
        //
        // This is an error case, most likely caused by reading a binary
        // DAF file by accident. So signal an appropriate error.
        //
        spicelib::SETMSG(
            b"The file \'#\' appears to be a binary DAF file and not a transfer file.",
            ctx,
        );
        spicelib::ERRCH(b"#", TXTFIL, ctx);
        spicelib::SIGERR(b"SPICE(NOTATRANSFERFILE)", ctx)?;
        spicelib::CHKOUT(b"CONVTB", ctx)?;
        return Ok(());
    } else if (fstr::eq(&ARCH, b"DEC") && fstr::eq(&TYPE, b"DAF")) {
        //
        // This is the case for the old text file format. It has no
        // identifying marks whatsoever, so we simply have to try and
        // convert it.
        //
        // We expect to have comments.
        //
        HAVCOM = true;
        //
        // Back up one record so that we are positioned in the file where
        // we were when this routine was entered.
        //
        {
            use f2rust_std::io;

            let specs = io::PosSpecs {
                unit: Some(TXTLUN),
                ..Default::default()
            };
            ctx.backspace(specs)?;
        }
        //
        // Convert it.
        //
        spicelib::DAFT2B(TXTLUN, BINFIL, RESREC, ctx)?;

        if spicelib::FAILED(ctx) {
            //
            // If there was an error then we need to close the text
            // file, and then check out and return to the caller.
            //
            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(TXTLUN),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }
            spicelib::CHKOUT(b"CONVTB", ctx)?;
            return Ok(());
        }
    } else {
        //
        // This is the catch all error case. At this point, we didn't
        // match any of the files whose architecture and types are
        // recognized. So, we toss our hands in the air and signal an
        // error.
        //
        spicelib::SETMSG(
            b"The architecture and type of the file \'#\'could not be determined.",
            ctx,
        );
        spicelib::ERRCH(b"#", TXTFIL, ctx);
        spicelib::SIGERR(b"SPICE(UNRECOGNIZABLEFILE)", ctx)?;
        spicelib::CHKOUT(b"CONVTB", ctx)?;
        return Ok(());
    }
    //
    // If we have comments to process, then process them.
    //
    if HAVCOM {
        //
        // There are three situations that we need to consider here:
        //
        //    1) We have a SPICE text file with comments. This implies
        //       that we have a bunch of comments to be put into the
        //       comment area that are surrounded by the begin comments
        //       marker, BCMARK, and the end comemnts marker, ECMARK.
        //
        //    2) We are at the end of the file. This means that we have
        //       an old SPICE kernel file, from the good old days before
        //       the comment area was implemented, or we ahve a plain old
        //       ordinary DAF file.
        //
        //    3) We are not at the end of the file, but there are no
        //       comments. This means that a text DAF file may be embedded
        //       in a larger text file or something. PDS does things like
        //       this: SFDUs and such.
        //
        // So, we need to look out for and deal with each of these
        // possibilities.
        //
        {
            use f2rust_std::{
                data::Val,
                io::{self, Reader},
            };

            let mut reader = io::FormattedReader::new(ctx.io_unit(TXTLUN)?, None, b"(A)")?;
            IOSTAT = io::capture_iostat(|| {
                reader.start()?;
                reader.read_str(&mut LINE)?;
                reader.finish()?;
                Ok(())
            })?;
        }

        if (IOSTAT > 0) {
            //
            // If there was an error then we need to close the text
            // file, and then check out and return to the caller.
            //
            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(TXTLUN),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }
            spicelib::SETMSG(b"Error reading the text file: #. IOSTAT = #.", ctx);
            spicelib::ERRCH(b"#", TXTFIL, ctx);
            spicelib::ERRINT(b"#", IOSTAT, ctx);
            spicelib::SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
            spicelib::CHKOUT(b"CONVTB", ctx)?;
            return Ok(());
        }
        //
        // If we encountered the end of the file, just check out and
        // return. This is not an error.
        //
        if (IOSTAT < 0) {
            spicelib::CHKOUT(b"CONVTB", ctx)?;
            return Ok(());
        }
        //
        // We got a line, so left justify it and see if it matches the
        // begin comments marker. If not, then use the Fortran BACKSPACE
        // command to reposition the file pointer to be ready to read the
        // line we just read.
        //
        if fstr::ne(fstr::substr(&LINE, spicelib::LTRIM(&LINE)..), BCMARK) {
            {
                use f2rust_std::io;

                let specs = io::PosSpecs {
                    unit: Some(TXTLUN),
                    ..Default::default()
                };
                ctx.backspace(specs)?;
            }
            spicelib::CHKOUT(b"CONVTB", ctx)?;
            return Ok(());
        }
        //
        // We're not at the end of the file, and the line we read
        // is BCMARK, so we write the comments to a scratch file.
        // We do this because we have to use SPCAC to add the comments
        // to the comment area of the binary file, and SPCAC rewinds
        // the file. It's okay for SPCAC to rewind a scratch file, because
        // it will probably not be very big, but it's not okay to rewind
        // the file connected to TXTLUN -- we don't know the initial
        // location of the file pointer or how big the file is.
        //
        spicelib::GETLUN(&mut SCRLUN, ctx)?;
        {
            use f2rust_std::io;

            let specs = io::OpenSpecs {
                unit: Some(SCRLUN),
                form: Some(b"FORMATTED"),
                access: Some(b"SEQUENTIAL"),
                status: Some(b"SCRATCH"),
                ..Default::default()
            };
            IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
        }

        if (IOSTAT != 0) {
            //
            // If there was an error then we need to close the text
            // file, and then check out and return to the caller.
            //
            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(SCRLUN),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }
            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(TXTLUN),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }
            spicelib::SETMSG(b"Error opening temporary file. IOSTAT = #.", ctx);
            spicelib::ERRINT(b"#", IOSTAT, ctx);
            spicelib::SIGERR(b"SPICE(FILEOPENERROR)", ctx)?;
            spicelib::CHKOUT(b"CONVTB", ctx)?;
            return Ok(());
        }
        //
        // Continue reading lines from the text file and storing them
        // in the scratch file until we get to the end marker. We do not
        // write the begin and end markers to the scratch file. We do not
        // need them.
        //
        EOC = false;

        while !EOC {
            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Reader},
                };

                let mut reader = io::FormattedReader::new(ctx.io_unit(TXTLUN)?, None, b"(A)")?;
                IOSTAT = io::capture_iostat(|| {
                    reader.start()?;
                    reader.read_str(&mut LINE)?;
                    reader.finish()?;
                    Ok(())
                })?;
            }

            if (IOSTAT != 0) {
                //
                // If there was an error then we need to close the
                // scratch file, the text file, and then check out
                // and return to the caller.
                //
                {
                    use f2rust_std::io;

                    let specs = io::CloseSpecs {
                        unit: Some(SCRLUN),
                        ..Default::default()
                    };
                    ctx.close(specs)?;
                }
                {
                    use f2rust_std::io;

                    let specs = io::CloseSpecs {
                        unit: Some(TXTLUN),
                        ..Default::default()
                    };
                    ctx.close(specs)?;
                }
                spicelib::SETMSG(b"Error reading the text file: #. IOSTAT = #.", ctx);
                spicelib::ERRCH(b"#", TXTFIL, ctx);
                spicelib::ERRINT(b"#", IOSTAT, ctx);
                spicelib::SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
                spicelib::CHKOUT(b"CONVTB", ctx)?;
                return Ok(());
            }
            //
            // If we are not at the end of the comments, then write the
            // line ot the scratch file. Otherwise set the end of comments
            // flag to .TRUE..
            //
            if fstr::ne(fstr::substr(&LINE, spicelib::LTRIM(&LINE)..), ECMARK) {
                {
                    use f2rust_std::{
                        data::Val,
                        io::{self, Writer},
                    };

                    let mut writer = io::FormattedWriter::new(ctx.io_unit(SCRLUN)?, None, b"(A)")?;
                    IOSTAT = io::capture_iostat(|| {
                        writer.start()?;
                        writer.write_str(fstr::substr(&LINE, 1..=spicelib::RTRIM(&LINE)))?;
                        writer.finish()?;
                        Ok(())
                    })?;
                }

                if (IOSTAT != 0) {
                    //
                    // If there was an error then we need to close the
                    // scratch file, the text file, and then check out
                    // and return to the caller.
                    //
                    {
                        use f2rust_std::io;

                        let specs = io::CloseSpecs {
                            unit: Some(SCRLUN),
                            ..Default::default()
                        };
                        ctx.close(specs)?;
                    }
                    {
                        use f2rust_std::io;

                        let specs = io::CloseSpecs {
                            unit: Some(TXTLUN),
                            ..Default::default()
                        };
                        ctx.close(specs)?;
                    }
                    spicelib::SETMSG(b"Error writing to temporary file. IOSTAT = #.", ctx);
                    spicelib::ERRINT(b"#", IOSTAT, ctx);
                    spicelib::SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
                    spicelib::CHKOUT(b"CONVTB", ctx)?;
                    return Ok(());
                }
            } else {
                EOC = true;
            }
        }
        //
        // Open the new binary file and add the comments that have been
        // stored temporarily in a scratch file.
        //
        spicelib::DAFOPW(BINFIL, &mut HANDLE, ctx)?;

        if spicelib::FAILED(ctx) {
            //
            // If there was an error then we need to close the scratch
            // file and the text file, and then check out and return to
            // the caller.
            //
            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(SCRLUN),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }
            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(TXTLUN),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }
            spicelib::CHKOUT(b"CONVTB", ctx)?;
            return Ok(());
        }

        spicelib::SPCAC(HANDLE, SCRLUN, b" ", b" ", ctx)?;

        if spicelib::FAILED(ctx) {
            //
            // If there was an error then we need to close the scratch
            // file and the text file, and then check out and return to
            // the caller.
            //
            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(SCRLUN),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }
            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(TXTLUN),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }
            spicelib::DAFCLS(HANDLE, ctx)?;
            spicelib::CHKOUT(b"CONVTB", ctx)?;
            return Ok(());
        }
        //
        // We succeeded, so close the files we opened to deal with the
        // comments. The scratch file is automatically deleted.
        //
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(SCRLUN),
                ..Default::default()
            };
            ctx.close(specs)?;
        }
        spicelib::DAFCLS(HANDLE, ctx)?;
    }
    //
    // Close the transfer file. We know it is open, because we got here.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(TXTLUN),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    spicelib::CHKOUT(b"CONVTB", ctx)?;
    Ok(())
}
