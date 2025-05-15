//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LINLEN: i32 = 1000;
const IFNLEN: i32 = 60;
const BUFSIZ: i32 = 2000;
const MAXPCH: i32 = 126;
const MINPCH: i32 = 32;

struct SaveVars {
    COMBUF: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut COMBUF = ActualCharArray::new(LINLEN, 1..=BUFSIZ);

        Self { COMBUF }
    }
}

//$Procedure DAFACU ( DAF add comments from a logical unit )
pub fn DAFACU(
    COMLUN: i32,
    BEGMRK: &[u8],
    ENDMRK: &[u8],
    INSBLN: bool,
    HANDLE: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut IFNAME = [b' '; IFNLEN as usize];
    let mut LINE = [b' '; LINLEN as usize];
    let mut I: i32 = 0;
    let mut INTCHR: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut LENGTH: i32 = 0;
    let mut NCOMR: i32 = 0;
    let mut NUMCOM: i32 = 0;
    let mut SCRLUN: i32 = 0;
    let mut BWARD: i32 = 0;
    let mut FWARD: i32 = 0;
    let mut FREE: i32 = 0;
    let mut ND: i32 = 0;
    let mut NI: i32 = 0;
    let mut EOF: bool = false;
    let mut MORE: bool = false;
    let mut OPENED: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    // Set the value for the maximum length of a text line.
    //
    //
    // Set the length of a DAF file internal filename.
    //
    //
    // Set the size of the comment buffer.
    //
    //
    // Maximum and minimum decimal values for the printable ASCII
    // characters.
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"DAFACU", ctx)?;
    }
    //
    // Verify that the DAF file attached to HANDLE is opened with write
    // access.
    //
    spicelib::DAFSIH(HANDLE, b"WRITE", ctx)?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"DAFACU", ctx)?;
        return Ok(());
    }
    //
    // Logical units must be positive. If it is not, signal an error.
    //
    if (COMLUN <= 0) {
        spicelib::SETMSG(
            b"# is not a valid logical unit. Logical units must be positive.",
            ctx,
        );
        spicelib::ERRINT(b"#", COMLUN, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDARGUMENT)", ctx)?;
        spicelib::CHKOUT(b"DAFACU", ctx)?;
        return Ok(());
    }
    //
    // Verify that there is an open ASCII text file attached to COMLUN.
    //
    {
        use f2rust_std::io;

        let specs = io::InquireSpecs {
            unit: Some(COMLUN),
            opened: Some(&mut OPENED),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.inquire(specs))?;
    }

    if (IOSTAT != 0) {
        spicelib::SETMSG(
            b"The INQUIRE on logical unit # failed. The value of IOSTAT was #.",
            ctx,
        );
        spicelib::ERRINT(b"#", COMLUN, ctx);
        spicelib::ERRINT(b"#", IOSTAT, ctx);
        spicelib::SIGERR(b"SPICE(INQUIREFAILED)", ctx)?;
        spicelib::CHKOUT(b"DAFACU", ctx)?;
        return Ok(());
    }

    if !OPENED {
        spicelib::SETMSG(
            b"There is no open file attached to logical unit #, so no comments could be read.",
            ctx,
        );
        spicelib::ERRINT(b"#", COMLUN, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDARGUMENT)", ctx)?;
        spicelib::CHKOUT(b"DAFACU", ctx)?;
        return Ok(());
    }

    //
    // Read the file record of the DAF attached to HANDLE. We get back
    // some stuff that we do not use.
    //
    spicelib::DAFRFR(
        HANDLE,
        &mut ND,
        &mut NI,
        &mut IFNAME,
        &mut FWARD,
        &mut BWARD,
        &mut FREE,
        ctx,
    )?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"DAFACU", ctx)?;
        return Ok(());
    }
    //
    // Compute the number of comment records.
    //
    NCOMR = (FWARD - 2);
    //
    // Get an available logical unit for the comment scratch file.
    //
    spicelib::GETLUN(&mut SCRLUN, ctx)?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"DAFACU", ctx)?;
        return Ok(());
    }
    //
    // Attempt to open the comment scratch file.
    //
    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(SCRLUN),
            status: Some(b"SCRATCH"),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
    }

    if (IOSTAT != 0) {
        spicelib::SETMSG(b"Attempt to open a temporary file failed. IOSTAT = #.", ctx);
        spicelib::ERRINT(b"#", IOSTAT, ctx);
        spicelib::SIGERR(b"SPICE(FILEOPENFAILED)", ctx)?;
        spicelib::CHKOUT(b"DAFACU", ctx)?;
        return Ok(());
    }
    //
    // Start looking for the begin comment marker. If the begin marker
    // is a blank line, then the comments begin on the first line of the
    // comment file. Otherwise, the comments begin on the line
    // immediately following the line which contains the begin comments
    // marker.
    //
    fstr::assign(&mut LINE, b" ");
    EOF = false;
    while fstr::ne(&LINE, BEGMRK) {
        spicelib::READLN(COMLUN, &mut LINE, &mut EOF, ctx)?;
        spicelib::LJUST(&LINE.clone(), &mut LINE);

        if spicelib::FAILED(ctx) {
            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(SCRLUN),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }
            spicelib::CHKOUT(b"DAFACU", ctx)?;
            return Ok(());
        }
        //
        // If we have encountered the end of file  here, we have a
        // problem: We did not find the begin comments marker in the
        // text file. So, set an appropriate error message and signal
        // the error. don't forget to close the scratch file.
        //
        if EOF {
            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(SCRLUN),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }
            spicelib::SETMSG(
                b"The begin comments marker \'#\' was not found in the comment file \'#\'.",
                ctx,
            );
            spicelib::ERRCH(b"#", BEGMRK, ctx);
            spicelib::ERRFNM(b"#", COMLUN, ctx)?;
            spicelib::SIGERR(b"SPICE(MARKERNOTFOUND)", ctx)?;
            spicelib::CHKOUT(b"DAFACU", ctx)?;
            return Ok(());
        }
    }
    //
    // Begin reading in the comment lines from the comment file,
    // placing them a buffer at a time into the temporary file.
    // We also scan each line for non printing characters.
    //
    fstr::assign(&mut LINE, b" ");
    if fstr::eq(ENDMRK, b" ") {
        //
        // If the end mark is blank, then we want to go until we hit the
        // end of the comment file.
        //
        while !EOF {
            NUMCOM = 0;
            spicelib::READLA(
                COMLUN,
                BUFSIZ,
                &mut NUMCOM,
                save.COMBUF.as_arg_mut(),
                &mut EOF,
                ctx,
            )?;

            if spicelib::FAILED(ctx) {
                {
                    use f2rust_std::io;

                    let specs = io::CloseSpecs {
                        unit: Some(SCRLUN),
                        ..Default::default()
                    };
                    ctx.close(specs)?;
                }
                spicelib::CHKOUT(b"DAFACU", ctx)?;
                return Ok(());
            }
            //
            // If we got some comments, we need to scan them for non-
            // printing characters.
            //
            if (NUMCOM > 0) {
                {
                    let m1__: i32 = 1;
                    let m2__: i32 = NUMCOM;
                    let m3__: i32 = 1;
                    I = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        LENGTH = spicelib::LASTNB(&save.COMBUF[I]);
                        //
                        // Scan the comment line for non printinig characters.
                        //
                        for J in 1..=LENGTH {
                            //
                            // Check to see that the characters in the buffer
                            // are all printing ASCII characters. The bounds
                            // for printing ASCII characters are given by
                            // MAXPCH and MINPCH, which are defined in the
                            // $ Local Parameters section of the header.
                            //
                            INTCHR = intrinsics::ICHAR(fstr::substr(&save.COMBUF[I], J..=J));
                            if ((INTCHR > MAXPCH) || (INTCHR < MINPCH)) {
                                {
                                    use f2rust_std::io;

                                    let specs = io::CloseSpecs {
                                        unit: Some(SCRLUN),
                                        ..Default::default()
                                    };
                                    ctx.close(specs)?;
                                }
                                spicelib::SETMSG(b"A nonprinting character was encountered in the comments. Value: #", ctx);
                                spicelib::ERRINT(b"#", INTCHR, ctx);
                                spicelib::SIGERR(b"SPICE(ILLEGALCHARACTER)", ctx)?;
                                spicelib::CHKOUT(b"DAFACU", ctx)?;
                                return Ok(());
                            }
                        }

                        I += m3__;
                    }
                }
                //
                // Write the comments to the temporary file.
                //
                spicelib::WRITLA(NUMCOM, save.COMBUF.as_arg(), SCRLUN, ctx)?;
            }

            if spicelib::FAILED(ctx) {
                {
                    use f2rust_std::io;

                    let specs = io::CloseSpecs {
                        unit: Some(SCRLUN),
                        ..Default::default()
                    };
                    ctx.close(specs)?;
                }
                spicelib::CHKOUT(b"DAFACU", ctx)?;
                return Ok(());
            }
        }
    } else {
        //
        // The endmark is non blank, then  we want to go until we find a
        // line in the comment file that matches the end mark that was
        // entered.
        //
        MORE = true;
        while MORE {
            NUMCOM = 0;
            spicelib::READLA(
                COMLUN,
                BUFSIZ,
                &mut NUMCOM,
                save.COMBUF.as_arg_mut(),
                &mut EOF,
                ctx,
            )?;

            if spicelib::FAILED(ctx) {
                {
                    use f2rust_std::io;

                    let specs = io::CloseSpecs {
                        unit: Some(SCRLUN),
                        ..Default::default()
                    };
                    ctx.close(specs)?;
                }
                spicelib::CHKOUT(b"DAFACU", ctx)?;
                return Ok(());
            }
            //
            // Look for ENDMRK in the current buffer, if we got some
            // comments.
            //
            if (NUMCOM > 0) {
                I = 1;
                while (MORE && (I <= NUMCOM)) {
                    fstr::assign(&mut LINE, save.COMBUF.get(I));
                    spicelib::LJUST(&LINE.clone(), &mut LINE);

                    if fstr::eq(&LINE, ENDMRK) {
                        MORE = false;
                        NUMCOM = (I - 1);
                    } else {
                        I = (I + 1);
                    }
                }
            }
            //
            // If we still have some comments, we need to scan them for
            // non printing characters.
            //
            if (NUMCOM > 0) {
                {
                    let m1__: i32 = 1;
                    let m2__: i32 = NUMCOM;
                    let m3__: i32 = 1;
                    I = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        LENGTH = spicelib::LASTNB(&save.COMBUF[I]);
                        //
                        // Scan the comment line for non printinig characters.
                        //
                        for J in 1..=LENGTH {
                            //
                            // Check to see that the characters in the buffer
                            // are all printing ASCII characters. The bounds
                            // for printing ASCII characters are given by
                            // MAXPCH and MINPCH, which are defined in the
                            // $ Local Parameters section of the header.
                            //
                            INTCHR = intrinsics::ICHAR(fstr::substr(&save.COMBUF[I], J..=J));
                            if ((INTCHR > MAXPCH) || (INTCHR < MINPCH)) {
                                {
                                    use f2rust_std::io;

                                    let specs = io::CloseSpecs {
                                        unit: Some(SCRLUN),
                                        ..Default::default()
                                    };
                                    ctx.close(specs)?;
                                }
                                spicelib::SETMSG(b"A nonprinting character was encountered in the comment buffer. Value: #", ctx);
                                spicelib::ERRINT(b"#", INTCHR, ctx);
                                spicelib::SIGERR(b"SPICE(ILLEGALCHARACTER)", ctx)?;
                                spicelib::CHKOUT(b"DAFACU", ctx)?;
                                return Ok(());
                            }
                        }

                        I += m3__;
                    }
                }
                //
                // Write the comments to the temporary file.
                //
                spicelib::WRITLA(NUMCOM, save.COMBUF.as_arg(), SCRLUN, ctx)?;
            }

            if spicelib::FAILED(ctx) {
                {
                    use f2rust_std::io;

                    let specs = io::CloseSpecs {
                        unit: Some(SCRLUN),
                        ..Default::default()
                    };
                    ctx.close(specs)?;
                }
                spicelib::CHKOUT(b"DAFACU", ctx)?;
                return Ok(());
            }
            //
            // If we have encountered the end of file here, we have a
            // problem: We did not find the end comments marker in the
            // text file. So, set an appropriate error message and
            // signal the error.
            //
            if (MORE && EOF) {
                {
                    use f2rust_std::io;

                    let specs = io::CloseSpecs {
                        unit: Some(SCRLUN),
                        ..Default::default()
                    };
                    ctx.close(specs)?;
                }
                spicelib::SETMSG(
                    b"The end comments marker \'#\' was not found in the comment file \'#\'.",
                    ctx,
                );
                spicelib::ERRCH(b"#", ENDMRK, ctx);
                spicelib::ERRFNM(b"#", COMLUN, ctx)?;
                spicelib::SIGERR(b"SPICE(MARKERNOTFOUND)", ctx)?;
                spicelib::CHKOUT(b"DAFACU", ctx)?;
                return Ok(());
            }
        }
    }
    //
    // If we made it to here, we have culled all of the comments out of
    // the text file and they were all OK. So we need to add all of the
    // comments to the DAF comment area now.
    //
    // If we are supposed to insert a blank line to separate the current
    // addition from any previously stored comments, and there are
    // comments already in the comment area, indicated by NCOMR > 0, then
    // we insert the blank line. Otherwise, just add the comments.
    //
    if (INSBLN && (NCOMR > 0)) {
        spicelib::DAFAC(HANDLE, 1, CharArray::from_ref(b" "), ctx)?;

        if spicelib::FAILED(ctx) {
            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(SCRLUN),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }
            spicelib::CHKOUT(b"DAFACU", ctx)?;
            return Ok(());
        }
    }
    //
    // Rewind the scratch file to get ready to put the comments into the
    // comment area.
    //
    {
        use f2rust_std::io;

        let specs = io::PosSpecs {
            unit: Some(SCRLUN),
            ..Default::default()
        };
        ctx.rewind(specs)?;
    }
    //
    // Begin reading through the scratch file, placing the comment lines
    // into the comment area of the DAF file a buffer at a time
    //
    EOF = false;
    while !EOF {
        NUMCOM = 0;
        //
        // Read in a buffer of comment lines.
        //
        spicelib::READLA(
            SCRLUN,
            BUFSIZ,
            &mut NUMCOM,
            save.COMBUF.as_arg_mut(),
            &mut EOF,
            ctx,
        )?;
        //
        // If we got some, add them to the comment area of the DAF file.
        //
        if (NUMCOM > 0) {
            spicelib::DAFAC(HANDLE, NUMCOM, save.COMBUF.as_arg(), ctx)?;
        }

        if spicelib::FAILED(ctx) {
            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(SCRLUN),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }
            spicelib::CHKOUT(b"DAFACU", ctx)?;
            return Ok(());
        }
    }
    //
    // Close the scratch file before exiting, it's the only one we
    // opened.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(SCRLUN),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    spicelib::CHKOUT(b"DAFACU", ctx)?;
    Ok(())
}
