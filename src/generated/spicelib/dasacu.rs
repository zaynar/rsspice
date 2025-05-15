//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const LINLEN: i32 = 255;
const IDWLEN: i32 = 8;
const IFNLEN: i32 = 60;
const BUFSIZ: i32 = 22;
const MAXPCH: i32 = 126;
const MINPCH: i32 = 32;

/// DAS add comments from a logical unit
///
/// Add comments to a previously opened binary DAS file from a
/// previously opened text file attached to a Fortran logical unit.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  COMLUN    I   Logical unit of the open comment text file.
///  BEGMRK    I   The begin comments marker in the comment text file.
///  ENDMRK    I   The end comments marker in the comment text file.
///  INSBLN    I   A flag indicating whether to insert a blank line.
///  HANDLE    I   Handle of a DAS file opened with write access.
///  LNSIZE    P   Maximum length of comment line.
/// ```
///
/// # Detailed Input
///
/// ```text
///  COMLUN   is the Fortran logical unit of a previously opened text
///           file which contains comments that are to be added to
///           the comment area of a binary E-Kernel file.
///
///  BEGMRK   is a marker which identifies the beginning of the
///           comments in the comment text file. This marker must
///           appear on a line by itself, and leading and trailing
///           blanks are not significant.
///
///           The line immediately following this marker is the first
///           comment line to be placed into the comment area of the
///           binary DAS file.
///
///           If the begin marker is blank, BEGMRK .EQ. ' ', then the
///           comments are assumed to start at the current location
///           in the comment text file.
///
///  ENDMRK   is a marker which identifies the end of the comments in
///           the comment text file. This marker must appear on a line
///           by itself, and leading and trailing blanks are not
///           significant.
///
///           The line immediately preceding this marker is the last
///           comment line to be placed into the comment area of the
///           binary DAS file.
///
///           If the end marker is blank, ENDMRK .EQ. ' ', then the
///           comments are assumed to stop at the end of the comment
///           text file.
///
///  INSBLN   is a logical flag which indicates whether a blank line is
///           to be inserted into the comment area of the binary DAS
///           file attached to HANDLE before any comments are added
///           to the comment area of the DAS file. This is to provide
///           a simple mechanism for separating any comments already
///           contained in the comment area of a DAS file from those
///           comments that are being added.
///
///           If the comment area of a binary DAS file is empty, the
///           value of this flag is not significant, the comments will
///           simply be placed into the comment area.
///
///  HANDLE   is the file handle for a binary DAS file that has been
///           opened with write access.
/// ```
///
/// # Parameters
///
/// ```text
///  LNSIZE   is both the maximum length of a comment line that can
///           be read from the input file, and the maximum length
///           of a comment line that this routine can write to the
///           output DAS file.
///
///           LNSIZE is set to 255 characters.
///
///           The DAS file format itself does not impose a limit
///           on the length of lines in comment area, other than
///           that the character count must be expressible in a
///           32-bit signed integer.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the scratch file for temporarily holding the comments
///      culled from the text file cannot be opened, the
///      error SPICE(FILEOPENFAILED) is signaled.
///
///  2)  If a non printing ASCII character is encountered in the
///      comments, the error SPICE(ILLEGALCHARACTER) is signaled.
///
///  3)  If the begin marker cannot be found in the text file, the
///      error SPICE(MARKERNOTFOUND) is signaled.
///
///  4)  If the end marker cannot be found in the text file, the
///      error SPICE(MARKERNOTFOUND) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See parameters COMLUN and HANDLE in the $Detailed_Inputs
///  section.
///
///  A scratch file is used to temporarily hold the comments culled
///  from the comment text file. This is so we do not have to find the
///  place where we started searching for comments in the original
///  file.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine will place all lines between two specified markers,
///  a "begin comments marker" and an "end comments marker," in a
///  text file into the comment area of a binary DAS file attached to
///  HANDLE. If the "begin comments marker" is blank, then the
///  comments are assumed to start at the current location of the
///  comment text file attached to COMLUN. If the "end comments
///  marker" is blank, then the comments are assumed to stop at the
///  end of the comment text file attached to COMLUN.
/// ```
///
/// # Examples
///
/// ```text
///  We will be using the files 'jabber.txt', 'batty.txt', and
///  'wndrland.DAS' in the example which follows.
///
///  'wndrland.dat' is a binary DAS file with an empty comment area
///                 into which we are going to place the entire file
///                 'jabber.txt' and a selected portion of the file
///                 'batty.txt'.
///
///  'jabber.txt'   is a text file that is to be placed into the
///                 comment area of the binary DAS file 'wndrland.DAS'.
///
///  'batty.txt'    is a text file from which will have a selected
///                 portion of its text placed into the comment area
///                 of the binary DAS file 'wndrland.DAS'.
///
///  Let -BOF- and -EOF- denote the beginning and end of a file,
///  respectively.
///
///  The file `jabber.txt' contains:
///
///     -BOF-
///               The Jabberwock
///
///     'Twas brillig, and the slithy toves
///           Did gyre and gimble in the wabe;
///     All mimsy were the borogoves,
///           And the mome raths outgrabe.
///
///     ``Beware the Jabberwock, my son!
///           The jaws that bite, the claws that catch!''
///
///     And as in uffish thought he stood,
///           The Jabberwock, with eyes of flame,
///     Came whiffling through the tulgey wood,
///           And burbled as it came!
///
///     One, two! One, two! And through and through
///           The vorpal blade went snicker-snack!
///     He left it dead, and with its head
///           He went galumphing back.
///
///     ``And hast thou slain the Jabberwock?
///           Come to my arms, my beamish boy!
///     O frabjous day! Callooh! Callay!''
///           He chortled in his joy.
///
///            Through the Looking-Glass
///            Lewis Carroll
///     -EOF-
///
///  The file `batty.txt' contains:
///
///     -BOF-
///     This file contains a brief poem about bats.
///
///     BEGIN bat poem
///     Twinkle, twinkle, little bat!
///     How I wonder what you're at!
///     Up above the world you fly!
///     Like a teatray in the sky.
///
///            Alice's Adventures in Wonderland
///            Lewis Carroll
///     END bat poem
///
///     And that's that for bats.
///     -EOF-
///
///  Let
///
///        JABLUN   be the logical unit for the file 'jabber.txt'
///        BATLUN   be the logical unit for the file 'batty.txt'
///  and
///        HANDLE   be the DAS handle for the file 'wndrland.DAS'
///
///  The code fragment
///
///  C
///  C      Open the files.
///  C
///         CALL DASOPW ( 'wndrland.DAS', HANDLE )
///         CALL TXTOPN ( 'jabber.txt'  , JABLUN )
///         CALL TXTOPN ( 'batty.txt'   , BATLUN )
///  C
///  C      Initialize the markers for the file 'jabber.txt'. We want
///  C      to include the entire file, so both markers are blank.
///  C
///         BEGMRK = ' '
///         ENDMRK = ' '
///         INSBLN = .TRUE.
///  C
///  C      Add the comments from the file 'jabber.txt'
///  C
///         CALL DASACU ( JABLUN, BEGMRK, ENDMRK, INSBLN, HANDLE )
///  C
///  C      Initialize the markers for the file `batty.txt'. We want
///  C      to include the bat poem only, so we define the begin and
///  C      end marker accordingly.
///  C
///         BEGMRK = 'BEGIN bat poem'
///         ENDMRK = 'END bat poem'
///         INSBLN = .TRUE.
///  C
///  C      Add the comments from the file 'batty.txt'
///  C
///         CALL DASACU ( BATLUN, BEGMRK, ENDMRK, INSBLN, HANDLE )
///  C
///  C      Close the files.
///
///         CLOSE       ( JABLUN )
///         CLOSE       ( BATLUN )
///         CALL DASCLS ( HANDLE )
///
///  will create a comment area in 'wndrland.DAS' which contains:
///
///     -BOC-
///               The Jabberwock
///
///     'Twas brillig, and the slithy toves
///           Did gyre and gimble in the wabe;
///     All mimsy were the borogoves,
///           And the mome raths outgrabe.
///
///     ``Beware the Jabberwock, my son!
///           The jaws that bite, the claws that catch!''
///
///     And as in uffish thought he stood,
///           The Jabberwock, with eyes of flame,
///     Came whiffling through the tulgey wood,
///           And burbled as it came!
///
///     One, two! One, two! And through and through
///           The vorpal blade went snicker-snack!
///     He left it dead, and with its head
///           He went galumphing back.
///
///     ``And hast thou slain the Jabberwock?
///           Come to my arms, my beamish boy!
///     O frabjous day! Callooh! Callay!''
///           He chortled in his joy.
///
///            Through the Looking-Glass
///            Lewis Carroll
///
///     Twinkle, twinkle, little bat!
///     How I wonder what you're at!
///     Up above the world you fly!
///     Like a teatray in the sky.
///
///            Alice's Adventures in Wonderland
///            Lewis Carroll
///     -EOC-
///
///  where -BOC- and -EOC- represent the beginning and end of the
///  comments, respectively.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The begin comments marker, BEGMRK, and the end comments
///      marker, ENDMRK, must each appear alone on a line in the
///      comment text file if they are not blank.
///
///  2)  The maximum length of a text line in the input comment file
///      is specified by the LINLEN parameter defined below. Currently
///      this value is 255 characters.
///
///  3)  The maximum length of a single comment line that can be
///      written by this routine to the output DAS file's comment area
///      is specified by the parameter LINLEN defined below. Currently
///      this value is 255 characters.
///
///  4)  This routine uses constants that are specific to the ASCII
///      character sequence. The results of using this routine with a
///      different character sequence are unpredictable.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.2, 02-JUN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.2.1, 15-MAR-2017 (NJB)
///
///         Added description of parameter LNSIZE. Fixed typos
///         throughout the comments.
///
/// -    SPICELIB Version 1.2.0, 07-JUL-1996 (NJB) (KRG)
///
///         Removed declaration, DATA and SAVE statements for unused
///         variable FIRST.
///
///      Beta Version 1.1.0, 20-SEP-1995 (KRG)
///
///         Added a check of FAILED after the call to GETLUN to trap
///         an error, if one is signaled by GETLUN, before attempting to
///         open the SCRATCH file.
///
///      Beta Version 1.0.0, 04-JAN-1993 (KRG)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.2.0, 07-JUL-1996 (NJB)
///
///         Removed declaration, DATA and SAVE statements for unused
///         variable FIRST.
///
/// -    Beta Version 1.1.0, 20-SEP-1995 (KRG)
///
///         Added a check of FAILED after the call to GETLUN to trap
///         an error, if one is signaled by GETLUN, before attempting to
///         open the SCRATCH file.
/// ```
pub fn dasacu(
    ctx: &mut SpiceContext,
    comlun: i32,
    begmrk: &str,
    endmrk: &str,
    insbln: bool,
    handle: i32,
) -> crate::Result<()> {
    DASACU(
        comlun,
        begmrk.as_bytes(),
        endmrk.as_bytes(),
        insbln,
        handle,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASACU ( DAS add comments from a logical unit )
pub fn DASACU(
    COMLUN: i32,
    BEGMRK: &[u8],
    ENDMRK: &[u8],
    INSBLN: bool,
    HANDLE: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut COMBUF = ActualCharArray::new(LINLEN, 1..=BUFSIZ);
    let mut IDWORD = [b' '; IDWLEN as usize];
    let mut IFNAME = [b' '; IFNLEN as usize];
    let mut LINE = [b' '; LINLEN as usize];
    let mut I: i32 = 0;
    let mut INTCHR: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut LENGTH: i32 = 0;
    let mut NCOMC: i32 = 0;
    let mut NCOMR: i32 = 0;
    let mut NRESVC: i32 = 0;
    let mut NRESVR: i32 = 0;
    let mut NUMCOM: i32 = 0;
    let mut SCRLUN: i32 = 0;
    let mut EOF: bool = false;
    let mut MORE: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    // Set the value for the maximum length of a text line.
    //
    //
    // Set the length of a DAS file ID word.
    //
    //
    // Set the length of a DAS file internal filename.
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
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DASACU", ctx)?;
    }
    //
    // Verify that the DAS file attached to HANDLE is opened with write
    // access.
    //
    DASSIH(HANDLE, b"WRITE", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DASACU", ctx)?;
        return Ok(());
    }
    //
    // Get the number of comment characters, and some other stuff that
    // we will not be using.
    //
    DASRFR(
        HANDLE,
        &mut IDWORD,
        &mut IFNAME,
        &mut NRESVR,
        &mut NRESVC,
        &mut NCOMR,
        &mut NCOMC,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"DASACU", ctx)?;
        return Ok(());
    }
    //
    // Get an available logical unit for the comment scratch file.
    //
    GETLUN(&mut SCRLUN, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DASACU", ctx)?;
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
        SETMSG(b"Attempt to open a temporary file failed. IOSTAT = #.", ctx);
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(FILEOPENFAILED)", ctx)?;
        CHKOUT(b"DASACU", ctx)?;
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
        READLN(COMLUN, &mut LINE, &mut EOF, ctx)?;
        LJUST(&LINE.clone(), &mut LINE);

        if FAILED(ctx) {
            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(SCRLUN),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }
            CHKOUT(b"DASACU", ctx)?;
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
            SETMSG(
                b"The begin comments marker \'#\' was not found in the comment file \'#\'.",
                ctx,
            );
            ERRCH(b"#", BEGMRK, ctx);
            ERRFNM(b"#", COMLUN, ctx)?;
            SIGERR(b"SPICE(MARKERNOTFOUND)", ctx)?;
            CHKOUT(b"DASACU", ctx)?;
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
            READLA(
                COMLUN,
                BUFSIZ,
                &mut NUMCOM,
                COMBUF.as_arg_mut(),
                &mut EOF,
                ctx,
            )?;

            if FAILED(ctx) {
                {
                    use f2rust_std::io;

                    let specs = io::CloseSpecs {
                        unit: Some(SCRLUN),
                        ..Default::default()
                    };
                    ctx.close(specs)?;
                }
                CHKOUT(b"DASACU", ctx)?;
                return Ok(());
            }
            //
            // If we got some comments, we need to scan them for non
            // printing characters.
            //
            if (NUMCOM > 0) {
                {
                    let m1__: i32 = 1;
                    let m2__: i32 = NUMCOM;
                    let m3__: i32 = 1;
                    I = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        LENGTH = LASTNB(&COMBUF[I]);
                        //
                        // Scan the comment line for non printing characters.
                        //
                        for J in 1..=LENGTH {
                            //
                            // Check to see that the characters in the buffer
                            // are all printing ASCII characters. The bounds
                            // for printing ASCII characters are given by
                            // MAXPCH and MINPCH, which are defined in the
                            // $ Local Parameters section of the header.
                            //
                            INTCHR = intrinsics::ICHAR(fstr::substr(&COMBUF[I], J..=J));
                            if ((INTCHR > MAXPCH) || (INTCHR < MINPCH)) {
                                {
                                    use f2rust_std::io;

                                    let specs = io::CloseSpecs {
                                        unit: Some(SCRLUN),
                                        ..Default::default()
                                    };
                                    ctx.close(specs)?;
                                }
                                SETMSG(b"A nonprinting character was encountered in the comments. Value: #", ctx);
                                ERRINT(b"#", INTCHR, ctx);
                                SIGERR(b"SPICE(ILLEGALCHARACTER)", ctx)?;
                                CHKOUT(b"DASACU", ctx)?;
                                return Ok(());
                            }
                        }

                        I += m3__;
                    }
                }
                //
                // Write the comments to the temporary file.
                //
                WRITLA(NUMCOM, COMBUF.as_arg(), SCRLUN, ctx)?;
            }

            if FAILED(ctx) {
                {
                    use f2rust_std::io;

                    let specs = io::CloseSpecs {
                        unit: Some(SCRLUN),
                        ..Default::default()
                    };
                    ctx.close(specs)?;
                }
                CHKOUT(b"DASACU", ctx)?;
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
            READLA(
                COMLUN,
                BUFSIZ,
                &mut NUMCOM,
                COMBUF.as_arg_mut(),
                &mut EOF,
                ctx,
            )?;

            if FAILED(ctx) {
                {
                    use f2rust_std::io;

                    let specs = io::CloseSpecs {
                        unit: Some(SCRLUN),
                        ..Default::default()
                    };
                    ctx.close(specs)?;
                }
                CHKOUT(b"DASACU", ctx)?;
                return Ok(());
            }
            //
            // Look for ENDMRK in the current buffer if we got some
            // comments.
            //
            if (NUMCOM > 0) {
                I = 1;
                while (MORE && (I <= NUMCOM)) {
                    fstr::assign(&mut LINE, COMBUF.get(I));
                    LJUST(&LINE.clone(), &mut LINE);

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
                        LENGTH = LASTNB(&COMBUF[I]);
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
                            INTCHR = intrinsics::ICHAR(fstr::substr(&COMBUF[I], J..=J));
                            if ((INTCHR > MAXPCH) || (INTCHR < MINPCH)) {
                                {
                                    use f2rust_std::io;

                                    let specs = io::CloseSpecs {
                                        unit: Some(SCRLUN),
                                        ..Default::default()
                                    };
                                    ctx.close(specs)?;
                                }
                                SETMSG(b"A nonprinting character was encountered in the comment buffer. Value: #", ctx);
                                ERRINT(b"#", INTCHR, ctx);
                                SIGERR(b"SPICE(ILLEGALCHARACTER)", ctx)?;
                                CHKOUT(b"DASACU", ctx)?;
                                return Ok(());
                            }
                        }

                        I += m3__;
                    }
                }
                //
                // Write the comments to the temporary file.
                //
                WRITLA(NUMCOM, COMBUF.as_arg(), SCRLUN, ctx)?;
            }

            if FAILED(ctx) {
                {
                    use f2rust_std::io;

                    let specs = io::CloseSpecs {
                        unit: Some(SCRLUN),
                        ..Default::default()
                    };
                    ctx.close(specs)?;
                }
                CHKOUT(b"DASACU", ctx)?;
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
                SETMSG(
                    b"The end comments marker \'#\' was not found in the comment file \'#\'.",
                    ctx,
                );
                ERRCH(b"#", ENDMRK, ctx);
                ERRFNM(b"#", COMLUN, ctx)?;
                SIGERR(b"SPICE(MARKERNOTFOUND)", ctx)?;
                CHKOUT(b"DASACU", ctx)?;
                return Ok(());
            }
        }
    }
    //
    // If we made it to here, we have culled all of the comments out of
    // the text file and they were all OK. So we need to add all of the
    // comments to the DAS comment area now.
    //
    // If we are supposed to insert a blank line to separate the current
    // addition from any previously stored comments, and there are
    // comments already in the comment area, indicated by NCOMC > 0, then
    // we insert the blank line. Otherwise, just add the comments.
    //
    if (INSBLN && (NCOMC > 0)) {
        DASAC(HANDLE, 1, CharArray::from_ref(b" "), ctx)?;

        if FAILED(ctx) {
            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(SCRLUN),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }
            CHKOUT(b"DASACU", ctx)?;
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
    // into the comment area of the DAS file a buffer at a time
    //
    EOF = false;
    while !EOF {
        NUMCOM = 0;
        //
        // Read in a buffer of comment lines.
        //
        READLA(
            SCRLUN,
            BUFSIZ,
            &mut NUMCOM,
            COMBUF.as_arg_mut(),
            &mut EOF,
            ctx,
        )?;
        //
        // If we got some, add them to the comment area of the DAS file.
        //
        if (NUMCOM > 0) {
            DASAC(HANDLE, NUMCOM, COMBUF.as_arg(), ctx)?;
        }

        if FAILED(ctx) {
            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(SCRLUN),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }
            CHKOUT(b"DASACU", ctx)?;
            return Ok(());
        }
    }
    //
    // Close the scratch file before exiting; it's the only one we
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

    CHKOUT(b"DASACU", ctx)?;
    Ok(())
}
