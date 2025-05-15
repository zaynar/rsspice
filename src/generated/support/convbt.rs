//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const BCMARK: &[u8] = b"~NAIF/SPC BEGIN COMMENTS~";
const ECMARK: &[u8] = b"~NAIF/SPC END COMMENTS~";
const CKTYP: &[u8] = b"CK";
const PCKTYP: &[u8] = b"PCK";
const SPKTYP: &[u8] = b"SPK";
const ARCLEN: i32 = 3;
const TYPLEN: i32 = 4;

//$ Procedure CONVBT ( Convert Kernel file from binary to text )
pub fn CONVBT(BINFIL: &[u8], TXTFIL: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut FARCH = [b' '; ARCLEN as usize];
    let mut FTYPE = [b' '; TYPLEN as usize];
    let mut HANDLE: i32 = 0;
    let mut TXTLUN: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut COMNTS: bool = false;

    //
    // SPICELIB functions
    //
    //
    // Local parameters
    //
    // Begin and end markers in the file for the comment area.
    //

    //
    // File types that are recognized.
    //

    //
    // Length of a file architecture.
    //
    //
    // Maximum length for a file type.
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
        spicelib::CHKIN(b"CONVBT", ctx)?;
    }
    //
    // Initialize the file architecture and the file type.
    //
    fstr::assign(&mut FARCH, b" ");
    fstr::assign(&mut FTYPE, b" ");
    //
    // Get the architecture and type of the binary file.
    //
    spicelib::GETFAT(BINFIL, &mut FARCH, &mut FTYPE, ctx)?;

    if spicelib::FAILED(ctx) {
        //
        // If there was an error getting the file architecture, just
        // return. An appropriate error message should have been set.
        // So, all we need to do here is return to the caller.
        //
        spicelib::CHKOUT(b"CONVBT", ctx)?;
        return Ok(());
    }
    //
    // Check to see that we got back a valid architecture and type.
    //
    //
    // Open the text file for output, obtaining a Fortran logical
    // unit.
    //
    spicelib::TXTOPN(TXTFIL, &mut TXTLUN, ctx)?;

    if spicelib::FAILED(ctx) {
        //
        // If there was an error opening the text file, just return.
        // An appropriate error message should have been set by TXTOPN.
        // So, all we need to do here is return to the caller.
        //
        spicelib::CHKOUT(b"CONVBT", ctx)?;
        return Ok(());
    }
    //
    // Process the files based on their binary architectures
    //
    if fstr::eq(&FARCH, b"DAF") {
        //
        // If the file is a NAIF SPK, CK, or PCK binary file, it may have
        // a comment area. So set the COMNTS flag appropriately.
        //
        COMNTS = fstr::eq(&FTYPE, SPKTYP);
        COMNTS = (COMNTS || fstr::eq(&FTYPE, CKTYP));
        COMNTS = (COMNTS || fstr::eq(&FTYPE, PCKTYP));
        //
        // First, convert the data portion of the binary file to text.
        // We only support the latest and greatest text file format for
        // conversion of the binary files to text.
        //
        spicelib::DAFBT(BINFIL, TXTLUN, ctx)?;

        if spicelib::FAILED(ctx) {
            //
            // If an error occurred while attempting to convert the
            // data portion of the DAF file to text, we need to close
            // the text file and return to the caller. We will delete
            // the text file when we close it.
            //
            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(TXTLUN),
                    status: Some(b"DELETE"),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }
            spicelib::CHKOUT(b"CONVBT", ctx)?;
            return Ok(());
        }
        //
        // The DAF file may or may not have a comment area. If it is a
        // NAIF SPICE kernel file, then it does and we need to deal with
        // it. Otherwise we do nothing.
        //
        if COMNTS {
            //
            // We need to open the binary DAF file so that we can extract
            // the comments from its comment area and place them in the
            // text file.
            //
            spicelib::DAFOPR(BINFIL, &mut HANDLE, ctx)?;

            if spicelib::FAILED(ctx) {
                //
                // If an error occurred, we need to close the text file and
                // return to the caller. We will delete the text file when
                // we close it.
                //
                {
                    use f2rust_std::io;

                    let specs = io::CloseSpecs {
                        unit: Some(TXTLUN),
                        status: Some(b"DELETE"),
                        ..Default::default()
                    };
                    ctx.close(specs)?;
                }
                spicelib::CHKOUT(b"CONVBT", ctx)?;
                return Ok(());
            }
            //
            // Write the begin comments marker to the text file.
            //
            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Writer},
                };

                let mut writer = io::ListDirectedWriter::new(ctx.io_unit(TXTLUN)?, None)?;
                IOSTAT = io::capture_iostat(|| {
                    writer.start()?;
                    writer.write_str(BCMARK)?;
                    writer.finish()?;
                    Ok(())
                })?;
            }

            if (IOSTAT != 0) {
                //
                // An error occurred, so close both the text and binary
                // files, set an appropriate error message, and return to
                // the caller. The text file is deleted when it is closed.
                //
                {
                    use f2rust_std::io;

                    let specs = io::CloseSpecs {
                        unit: Some(TXTLUN),
                        status: Some(b"DELETE"),
                        ..Default::default()
                    };
                    ctx.close(specs)?;
                }
                spicelib::DAFCLS(HANDLE, ctx)?;
                spicelib::SETMSG(
                    b"Error writing the begin comments marker to the text file: #. IOSTAT = #.",
                    ctx,
                );
                spicelib::ERRCH(b"#", TXTFIL, ctx);
                spicelib::ERRINT(b"#", IOSTAT, ctx);
                spicelib::SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
                spicelib::CHKOUT(b"CONVBT", ctx)?;
                return Ok(());
            }
            //
            // Extract the comment area of the binary file to the text
            // file.
            //
            spicelib::SPCEC(HANDLE, TXTLUN, ctx)?;

            if spicelib::FAILED(ctx) {
                //
                // If the comment extraction failed, then an appropriate
                // error message should have already been set, so close
                // the text and binary files and return to the caller. The
                // text file is deleted when it is closed.
                //
                {
                    use f2rust_std::io;

                    let specs = io::CloseSpecs {
                        unit: Some(TXTLUN),
                        status: Some(b"DELETE"),
                        ..Default::default()
                    };
                    ctx.close(specs)?;
                }
                spicelib::CHKOUT(b"CONVBT", ctx)?;
                return Ok(());
            }
            //
            // Write the end comments marker.
            //
            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Writer},
                };

                let mut writer = io::ListDirectedWriter::new(ctx.io_unit(TXTLUN)?, None)?;
                IOSTAT = io::capture_iostat(|| {
                    writer.start()?;
                    writer.write_str(ECMARK)?;
                    writer.finish()?;
                    Ok(())
                })?;
            }

            if (IOSTAT != 0) {
                //
                // An error occurred, so close both the text and binary
                // files, set an appropriate error message, and return to
                // the caller. The text file is deleted when it is closed.
                //
                {
                    use f2rust_std::io;

                    let specs = io::CloseSpecs {
                        unit: Some(TXTLUN),
                        status: Some(b"DELETE"),
                        ..Default::default()
                    };
                    ctx.close(specs)?;
                }
                spicelib::DAFCLS(HANDLE, ctx)?;
                spicelib::SETMSG(
                    b"Error writing the end comments marker to the text file: #. IOSTAT = #.",
                    ctx,
                );
                spicelib::ERRCH(b"#", TXTFIL, ctx);
                spicelib::ERRINT(b"#", IOSTAT, ctx);
                spicelib::SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
                spicelib::CHKOUT(b"CONVBT", ctx)?;
                return Ok(());
            }
            //
            // Close the binary DAF file that we opened to extract the
            // comments.
            //
            spicelib::DAFCLS(HANDLE, ctx)?;
        }
    } else if fstr::eq(&FARCH, b"DAS") {
        //
        // DAS files are easy. Everything is integrated into the files
        // so we do not need to worry about comments or reserved records
        // or anything. We just convert it.
        //
        // Convert the data portion of the binary file to text. We
        // only support the latest and greatest text file format for
        // conversion of the binary files to text.
        //
        spicelib::DASBT(BINFIL, TXTLUN, ctx)?;

        if spicelib::FAILED(ctx) {
            //
            // If an error occurred while attempting to convert the
            // DAS file to text, we need to close the text file and
            // return to the caller. We will delete the text file
            // when we close it.
            //
            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(TXTLUN),
                    status: Some(b"DELETE"),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }
            spicelib::CHKOUT(b"CONVBT", ctx)?;
            return Ok(());
        }
    } else if fstr::eq(&FARCH, b"XFR") {
        //
        // This is an error case, most likely caused by reading a transfer
        // file by accident. So signal an appropriate error.
        //
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(TXTLUN),
                status: Some(b"DELETE"),
                ..Default::default()
            };
            ctx.close(specs)?;
        }
        spicelib::SETMSG(
            b"The file \'#\' appears to be a transfer file and not a binary kernel file.",
            ctx,
        );
        spicelib::ERRCH(b"#", BINFIL, ctx);
        spicelib::SIGERR(b"SPICE(NOTABINARYKERNEL)", ctx)?;
        spicelib::CHKOUT(b"CONVBT", ctx)?;
        return Ok(());
    } else if fstr::eq(&FARCH, b"DEC") {
        //
        // This is an error case, most likely caused by reading a transfer
        // file by accident. So signal an appropriate error.
        //
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(TXTLUN),
                status: Some(b"DELETE"),
                ..Default::default()
            };
            ctx.close(specs)?;
        }
        spicelib::SETMSG(
            b"The file \'#\' appears to be a decimal transfer file and not a binary kernel file.",
            ctx,
        );
        spicelib::ERRCH(b"#", BINFIL, ctx);
        spicelib::SIGERR(b"SPICE(NOTABINARYKERNEL)", ctx)?;
        spicelib::CHKOUT(b"CONVBT", ctx)?;
        return Ok(());
    } else {
        //
        // This is the catch all error case. At this point, we didn't
        // match any of the files whose architecture and types are
        // recognized. So, we toss our hands in the air and signal an
        // error.
        //
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(TXTLUN),
                status: Some(b"DELETE"),
                ..Default::default()
            };
            ctx.close(specs)?;
        }
        spicelib::SETMSG(
            b"The architecture and type of the file \'#\' were not recognized.",
            ctx,
        );
        spicelib::ERRCH(b"#", BINFIL, ctx);
        spicelib::SIGERR(b"SPICE(BADFILEFORMAT)", ctx)?;
        spicelib::CHKOUT(b"CONVBT", ctx)?;
        return Ok(());
    }
    //
    // Close the text file that was created.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(TXTLUN),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    spicelib::CHKOUT(b"CONVBT", ctx)?;
    Ok(())
}
