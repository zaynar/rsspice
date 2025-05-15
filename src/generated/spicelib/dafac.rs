//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const IFNLEN: i32 = 60;
const INTEOC: i32 = 4;
const INTEOL: i32 = 0;
const MXCREC: i32 = 1000;
const MAXPCH: i32 = 126;
const MINPCH: i32 = 32;

struct SaveVars {
    EOCMRK: Vec<u8>,
    EOLMRK: Vec<u8>,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut EOCMRK = vec![b' '; 1 as usize];
        let mut EOLMRK = vec![b' '; 1 as usize];
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            EOCMRK,
            EOLMRK,
            FIRST,
        }
    }
}

/// DAF add comments
///
/// Add comments from a buffer of character strings to the comment
/// area of a binary DAF file, appending them to any comments which
/// are already present in the file's comment area.
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of a DAF opened with write access.
///  N          I   Number of comments to put into the comment area.
///  BUFFER     I   Buffer of comments to put into the comment area.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the file handle of a binary DAF which has been opened
///           with write access.
///
///  N        is the number of comments in BUFFER that are to be added
///           to the comment area of the binary DAF attached to HANDLE.
///
///  BUFFER   is a buffer containing comments which are to be added
///           to the comment area of the binary DAF attached to HANDLE.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the number of comments to be added is not positive, the
///      error SPICE(INVALIDARGUMENT) is signaled.
///
///  2)  If a non printing ASCII character is encountered in the
///      comments, the error SPICE(ILLEGALCHARACTER) is signaled.
///
///  3)  If the binary DAF file attached to HANDLE is not open with
///      write access, an error is signaled by a routine in the call
///      tree of this routine.
///
///  4)  If the end of the comments cannot be found, i.e., the end of
///      comments marker is missing on the last comment record, the
///      error SPICE(BADCOMMENTAREA) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See argument HANDLE in $Detailed_Input.
/// ```
///
/// # Particulars
///
/// ```text
///  A binary DAF contains a data area which is reserved for storing
///  annotations or descriptive textual information about the data
///  contained in a file. This area is referred to as the ``comment
///  area'' of the file. The comment area of a DAF is a line oriented
///  medium for storing textual information. The comment area
///  preserves leading or embedded white space in the line(s) of text
///  which are stored so that the appearance of the information will
///  be unchanged when it is retrieved (extracted) at some other time.
///  Trailing blanks, however, are NOT preserved, due to the way that
///  character strings are represented in standard Fortran 77.
///
///  This routine will take a buffer of text lines and add (append)
///  them to the comment area of a binary DAF. If there are no
///  comments in the comment area of the file, then space will be
///  allocated and the text lines in BUFFER will be placed into the
///  comment area. The text lines may contain only printable ASCII
///  characters (decimal values 32 - 126).
///
///  There is NO maximum length imposed on the significant portion
///  of a text line that may be placed into the comment area of a
///  DAF. The maximum length of a line stored in the comment area
///  should be reasonable, however, so that they may be easily
///  extracted. A good maximum value for this would be 255 characters,
///  as this can easily accommodate ``screen width'' lines as well as
///  long lines which may contain some other form of information.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) This example demonstrates how to append new comments to the
///     comment area of a DAF file.
///
///     Use the SPK kernel below as input DAF file for the program.
///
///        earthstns_itrf93_201023.bsp
///
///
///     Example code begins here.
///
///
///           PROGRAM DAFAC_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           INTEGER               RTRIM
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         KERNEL
///           PARAMETER           ( KERNEL =
///          .                         'earthstns_itrf93_201023.bsp' )
///
///           INTEGER               BUFSIZ
///           PARAMETER           ( BUFSIZ = 25 )
///
///           INTEGER               CMTSIZ
///           PARAMETER           ( CMTSIZ = 7  )
///
///           INTEGER               LINLEN
///           PARAMETER           ( LINLEN = 1000 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(LINLEN)    BUFFER ( BUFSIZ )
///           CHARACTER*(LINLEN)    NEWCMT ( CMTSIZ )
///
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               N
///
///           LOGICAL               DONE
///
///     C
///     C     Set the new comments to be added to the DAF file.
///     C
///           DATA                  NEWCMT /
///          .  '================== NEW COMMENTS ==================',
///          .  '',
///          .  '   New comments can be appended to the end of the',
///          .  '   comment area of a DAF file, with a single',
///          .  '   operation.',
///          .  '',
///          .  '================ END NEW COMMENTS ================' /
///
///
///     C
///     C     Open a DAF for write. Return a HANDLE referring to the
///     C     file.
///     C
///           CALL DAFOPW ( KERNEL, HANDLE )
///
///     C
///     C     Print the end of comment area from the DAF file.
///     C     (Maximum 15 lines.)
///     C
///           DONE = .FALSE.
///
///           DO WHILE ( .NOT. DONE )
///
///              CALL DAFEC  ( HANDLE, 15, N, BUFFER, DONE )
///
///              IF ( DONE ) THEN
///
///                 WRITE(*,'(A)') 'End of comment area of input '
///          .                  // 'DAF file (max. 15 lines): '
///                 WRITE(*,'(A)') '-------------------------------'
///          .                  // '-------------------------------'
///
///                 DO I = 1, N
///
///                    WRITE (*,*) BUFFER(I)(:RTRIM(BUFFER(I)))
///
///                 END DO
///
///                 WRITE(*,'(A)') '-------------------------------'
///          .                  // '-------------------------------'
///
///              END IF
///
///           END DO
///
///     C
///     C     Append the new comments to the DAF file.
///     C
///           CALL DAFAC ( HANDLE, CMTSIZ, NEWCMT )
///
///     C
///     C     Safely close the DAF.
///     C
///           CALL DAFCLS ( HANDLE )
///
///     C
///     C     Check if the comments have indeed appended.
///     C
///     C     Open a DAF for read.
///     C
///           CALL DAFOPR ( KERNEL, HANDLE )
///           DONE = .FALSE.
///
///           DO WHILE ( .NOT. DONE )
///
///              CALL DAFEC  ( HANDLE, BUFSIZ, N, BUFFER, DONE )
///
///              IF ( DONE ) THEN
///
///                 WRITE(*,'(A)') 'End of comment area of input '
///          .                  // 'DAF file (max. 25 lines): '
///                 WRITE(*,'(A)') '-------------------------------'
///          .                  // '-------------------------------'
///
///                 DO I = 1, N
///
///                    WRITE (*,*) BUFFER(I)(:RTRIM(BUFFER(I)))
///
///                 END DO
///
///                 WRITE(*,'(A)') '-------------------------------'
///          .                  // '-------------------------------'
///
///              END IF
///
///           END DO
///
///     C
///     C     Safely close the DAF.
///     C
///           CALL DAFCLS ( HANDLE )
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     End of comment area of input DAF file (max. 15 lines):
///     --------------------------------------------------------------
///         DSS-65_DXYZ       =    (    -0.0100          0.0242     ***
///         DSS-65_TOPO_EPOCH =       @2020-OCT-23/00:00
///         DSS-65_UP         =       'Z'
///         DSS-65_NORTH      =       'X'
///
///      \begintext
///     --------------------------------------------------------------
///     End of comment area of input DAF file (max. 25 lines):
///     --------------------------------------------------------------
///         DSS-65_DXYZ       =    (    -0.0100          0.0242     ***
///         DSS-65_TOPO_EPOCH =       @2020-OCT-23/00:00
///         DSS-65_UP         =       'Z'
///         DSS-65_NORTH      =       'X'
///
///      \begintext
///      ================== NEW COMMENTS ==================
///
///         New comments can be appended to the end of the
///         comment area of a DAF file, with a single
///         operation.
///
///      ================ END NEW COMMENTS ================
///     --------------------------------------------------------------
///
///
///     Warning: incomplete output. 2 lines extended past the right
///     margin of the header and have been truncated. These lines are
///     marked by "***" at the end of each line.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine uses constants that are specific to the ASCII
///      character sequence. The results of using this routine with
///      a different character sequence are unpredictable.
///
///  2)  This routine is only used to extract records on environments
///      whose characters are a single byte in size. Updates to this
///      routine and routines in its call tree may be required to
///      properly handle other cases.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  F.S. Turner        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 25-NOV-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///         Added complete code examples from existing code fragments.
///
/// -    SPICELIB Version 2.0.0, 16-NOV-2001 (FST)
///
///         Updated this routine to utilize the new handle manager
///         interfaces.
///
/// -    SPICELIB Version 1.0.0, 26-JUL-1994 (KRG)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 2.0.0, 16-NOV-2001 (FST)
///
///         The call to DAFHLU has been replaced with a call to ZZDDHHLU,
///         the handle manager interface for retrieving a logical unit.
///         DAFHLU is no longer used, since it locks the unit returned to
///         its HANDLE, tying up resources in the handle manager.
/// ```
pub fn dafac(ctx: &mut SpiceContext, handle: i32, n: i32, buffer: CharArray) -> crate::Result<()> {
    DAFAC(handle, n, buffer, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFAC ( DAF add comments )
pub fn DAFAC(HANDLE: i32, N: i32, BUFFER: CharArray, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let BUFFER = DummyCharArray::new(BUFFER, None, 1..);
    let mut CRECRD = [b' '; MXCREC as usize];
    let mut IFNAME = [b' '; IFNLEN as usize];
    let mut CURPOS: i32 = 0;
    let mut DAFLUN: i32 = 0;
    let mut EOCPOS: i32 = 0;
    let mut LENGTH: i32 = 0;
    let mut NCHARS: i32 = 0;
    let mut NCOMC: i32 = 0;
    let mut NCOMR: i32 = 0;
    let mut NELPOS: i32 = 0;
    let mut NEWREC: i32 = 0;
    let mut NOTUSD: i32 = 0;
    let mut RECNO: i32 = 0;
    let mut RINUSE: i32 = 0;
    let mut SPACE: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut BWARD: i32 = 0;
    let mut FWARD: i32 = 0;
    let mut FREE: i32 = 0;
    let mut ND: i32 = 0;
    let mut NI: i32 = 0;
    let mut EMPTY: bool = false;
    let mut FOUND: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    // Length of a DAF file internal filename.
    //
    //
    // Decimal value for the DAF comment area end-of-comment (EOC)
    // marker.
    //
    //
    // Decimal value for the DAF comment area end-of-line (EOL) marker.
    //
    //
    // Length of a DAF character record, in characters.
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
    // Initial values
    //
    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DAFAC", ctx)?;
    }
    //
    // The lines of text in BUFFER will be ``packed'' into DAF comment
    // records: the significant portion of each comment line from BUFFER
    // will be terminated using the special character EOLMRK to indicate
    // the end of the line. When a comment record is full or all of the
    // comments have been added, the comment record will be written to
    // the comment area of the binary DAF file.
    //
    // If this is the first time that this routine has been called,
    // we need to initialize the character value for the end-of-line
    // marker and the character value for the end of comments marker.
    //
    if save.FIRST {
        save.FIRST = false;
        fstr::assign(&mut save.EOCMRK, &intrinsics::CHAR(INTEOC));
        fstr::assign(&mut save.EOLMRK, &intrinsics::CHAR(INTEOL));
    }
    //
    // Verify that the DAF file attached to HANDLE is opened with write
    // access.
    //
    DAFSIH(HANDLE, b"WRITE", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DAFAC", ctx)?;
        return Ok(());
    }
    //
    // Convert the DAF file handle to its corresponding Fortran logical
    // unit number for reading and writing comment records.
    //
    ZZDDHHLU(HANDLE, b"DAF", false, &mut DAFLUN, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DAFAC", ctx)?;
        return Ok(());
    }
    //
    // Check for a nonpositive number of lines in the buffer.
    //
    if (N <= 0) {
        SETMSG(b"The number of comment lines to be added to the binary DAF file \'#\' was not positive: #.", ctx);
        ERRFNM(b"#", DAFLUN, ctx)?;
        ERRINT(b"#", N, ctx);
        SIGERR(b"SPICE(INVALIDARGUMENT)", ctx)?;
        CHKOUT(b"DAFAC", ctx)?;
        return Ok(());
    }
    //
    // Count the number of characters in the buffer ignoring trailing
    // blanks on nonblank lines and blank lines. The count will be
    // modified to include the contribution of blank lines later. This
    // count is used to determine the number of character records to be
    // added to the binary DAF file attached to HANDLE.
    //
    NCHARS = 0;

    for I in 1..=N {
        //
        // Get the length of the significant portion of a comment line.
        //
        LENGTH = LASTNB(&BUFFER[I]);
        //
        // Scan the comment line for non printing characters.
        //
        for J in 1..=LENGTH {
            //
            // Check to see that the characters in the buffer are all
            // printing ASCII characters. The bounds for printing ASCII
            // characters are given by MINPCH and MAXPCH, which are
            // defined in the $ Local Parameters section of the header.
            //
            if ((intrinsics::ICHAR(fstr::substr(&BUFFER[I], J..=J)) > MAXPCH)
                || (intrinsics::ICHAR(fstr::substr(&BUFFER[I], J..=J)) < MINPCH))
            {
                SETMSG(
                    b"A nonprinting character was encountered in the comment buffer. Value: #",
                    ctx,
                );
                ERRINT(
                    b"#",
                    intrinsics::ICHAR(fstr::substr(&BUFFER[I], J..=J)),
                    ctx,
                );
                SIGERR(b"SPICE(ILLEGALCHARACTER)", ctx)?;
                CHKOUT(b"DAFAC", ctx)?;
                return Ok(());
            }
        }
        //
        // Increment the number of characters by the length of the
        // significant portion of the current line in the buffer.
        //
        NCHARS = (NCHARS + LENGTH);
    }
    //
    // We need to include the number of end of line markers in the
    // number of characters, so add the number of comment lines to
    // be added, N, to the number of characters, NCHARS. This is where
    // the contribution of any blank lines gets added to the character
    // count. We also need to have space for the end of comments marker.
    //
    NCHARS = ((NCHARS + N) + 1);
    //
    // Get the current number of comment records and comment characters
    // from the DAF file attached to HANDLE. We will also get back some
    // extra stuff that we do not use.
    //
    DAFRFR(
        HANDLE,
        &mut ND,
        &mut NI,
        &mut IFNAME,
        &mut FWARD,
        &mut BWARD,
        &mut FREE,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"DAFAC", ctx)?;
        return Ok(());
    }
    //
    // Compute the number of comment records and the number of comment
    // characters. In order to perform these calculations, we assume
    // that we have a valid comment area in the DAF file attached to
    // HANDLE.
    //
    NCOMR = (FWARD - 2);

    if (NCOMR > 0) {
        //
        // The starting record number is the number of comment records + 1
        // where the 1 skips the file record.
        //
        EMPTY = true;
        FOUND = false;
        NOTUSD = 0;

        while (((NCOMR > 0) && !FOUND) && EMPTY) {
            RECNO = (NCOMR + 1);

            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Reader},
                };

                let mut reader = io::UnformattedReader::new(ctx.io_unit(DAFLUN)?, Some(RECNO))?;
                IOSTAT = io::capture_iostat(|| {
                    reader.start()?;
                    reader.read_str(&mut CRECRD)?;
                    reader.finish()?;
                    Ok(())
                })?;
            }

            if (IOSTAT != 0) {
                SETMSG(
                    b"Error reading comment area of binary file named \'#\'.  IOSTAT = #.",
                    ctx,
                );
                ERRFNM(b"#", DAFLUN, ctx)?;
                ERRINT(b"#", IOSTAT, ctx);
                SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
                CHKOUT(b"DAFAC", ctx)?;
                return Ok(());
            }
            //
            // Scan the comment record looking for the end of comments
            // marker.
            //
            EOCPOS = CPOS(fstr::substr(&CRECRD, 1..=MXCREC), &save.EOCMRK, 1);

            if (EOCPOS > 0) {
                FOUND = true;
            } else {
                NELPOS = NCPOS(fstr::substr(&CRECRD, 1..=MXCREC), &save.EOLMRK, 1);

                if (NELPOS != 0) {
                    EMPTY = false;
                } else {
                    NCOMR = (NCOMR - 1);
                    NOTUSD = (NOTUSD + 1);
                }
            }
        }
        //
        // If we do not find the end of comments marker and the comment
        // area is not empty, then it is an error.
        //
        if (!FOUND && !EMPTY) {
            SETMSG(b"The comment area in the DAF file \'#\' may be damaged. The end of the comments could not be found.", ctx);
            ERRFNM(b"#", DAFLUN, ctx)?;
            SIGERR(b"SPICE(BADCOMMENTAREA)", ctx)?;
            CHKOUT(b"DAFAC", ctx)?;
            return Ok(());
        } else if FOUND {
            NCOMC = (((MXCREC * (NCOMR - 1)) + EOCPOS) - 1);
        } else if EMPTY {
            NCOMC = 0;
        }
    } else {
        NCOMC = 0;
        NOTUSD = 0;
    }
    //
    // Determine the amount of free space in the comment area. If
    // there are some comment records allocated, the space available
    // is the number of comment records allocated times the length of
    // a comment record, minus the number of comment characters already
    // used. Otherwise, the space available is zero.
    //
    if ((NCOMR + NOTUSD) > 0) {
        SPACE = (((NOTUSD * MXCREC) + (NCOMR * MXCREC)) - NCOMC);
    } else {
        SPACE = 0;
    }
    //
    // Determine the number of new comment records which are necessary
    // to store all of the comments from the buffer.
    //
    if (NCHARS > SPACE) {
        //
        // If there are more characters to store than available space
        // we need at least one new record.
        //
        NEWREC = (1 + (((NCHARS - SPACE) - 1) / MXCREC));
    } else {
        //
        // Otherwise, we do not need any new records.
        //
        NEWREC = 0;
    }
    //
    // Now add the necessary number of comment records to the file,
    // if we need to add any.
    //
    if (NEWREC > 0) {
        DAFARR(HANDLE, NEWREC, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"DAFAC", ctx)?;
            return Ok(());
        }
    }
    //
    // At this point, we know that we have enough space to write all of
    // the comments in BUFFER to the comment area. Either there was
    // enough space already there, or we calculated how many new comment
    // records were needed, and we added them to the file. So, now we
    // begin ``packing'' the comments into DAF comment records and
    // writing them to the file.
    //
    // We begin initializing the appropriate variables.
    //
    if (NCOMC == 0) {
        //
        // If there are no comments in the comment area, then we need
        // to skip the file record. The first available comment record
        // is the record immediately after the file record, so we set
        // RECNO accordingly. We also initialize the current position in
        // the comment record, and the comment record itself.
        //
        RECNO = 2;
        CURPOS = 1;
        fstr::assign(&mut CRECRD, b" ");
    } else {
        //
        // If there are comments in the comment area, then we need to
        // skip the file record and any comment records which have been
        // filled. The first comment record with space available is the
        // record immediately following the last completely filled
        // comment record. So calculate the number of comment records
        // in use, and set RECNO appropriately. Finally calculate the
        // initial position.
        //
        RINUSE = (1 + (NCOMC / MXCREC));
        RECNO = (1 + RINUSE);
        CURPOS = ((NCOMC - (MXCREC * (RINUSE - 1))) + 1);
    }
    //
    // Begin ``packing'' the comments from the input buffer into the
    // comment records, writing the comment records to the file as they
    // become filled.
    //
    for I in 1..=N {
        //
        // Get the length of the significant portion of comment line I.
        //
        LENGTH = LASTNB(&BUFFER[I]);
        //
        // Process the comment line.
        //
        for J in 1..=LENGTH {
            //
            // If we have filled the comment record while processing
            // comment line BUFFER(I), write out the comment record,
            // increment the record number, RECNO, and reset the values
            // of the current position and the comment record.
            //
            if (CURPOS > MXCREC) {
                {
                    use f2rust_std::{
                        data::Val,
                        io::{self, Writer},
                    };

                    let mut writer = io::UnformattedWriter::new(ctx.io_unit(DAFLUN)?, Some(RECNO))?;
                    IOSTAT = io::capture_iostat(|| {
                        writer.start()?;
                        writer.write_str(&CRECRD)?;
                        writer.finish()?;
                        Ok(())
                    })?;
                }

                if (IOSTAT != 0) {
                    SETMSG(
                        b"Error writing to record # of the binary file named \'#\'. IOSTAT = #.",
                        ctx,
                    );
                    ERRINT(b"#", RECNO, ctx);
                    ERRFNM(b"#", DAFLUN, ctx)?;
                    ERRINT(b"#", IOSTAT, ctx);
                    SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
                    CHKOUT(b"DAFAC", ctx)?;
                    return Ok(());
                }

                RECNO = (RECNO + 1);
                CURPOS = 1;
                fstr::assign(&mut CRECRD, b" ");
            }

            fstr::assign(
                fstr::substr_mut(&mut CRECRD, CURPOS..=CURPOS),
                fstr::substr(BUFFER.get(I), J..=J),
            );
            CURPOS = (CURPOS + 1);
        }
        //
        // Check to see if we happened to exactly fill the comment record
        // when we finished processing comment line BUFFER(I). If we
        // did, CURPOS will be 1 greater than MXCREC, and we will need
        // to write the comment record to the file, increment the record
        // number, RECNO, and reset the values of the current position
        // and the comment record.
        //
        if (CURPOS > MXCREC) {
            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Writer},
                };

                let mut writer = io::UnformattedWriter::new(ctx.io_unit(DAFLUN)?, Some(RECNO))?;
                IOSTAT = io::capture_iostat(|| {
                    writer.start()?;
                    writer.write_str(&CRECRD)?;
                    writer.finish()?;
                    Ok(())
                })?;
            }

            if (IOSTAT != 0) {
                SETMSG(
                    b"Error writing to record # of the binary file named \'#\'. IOSTAT = #.",
                    ctx,
                );
                ERRINT(b"#", RECNO, ctx);
                ERRFNM(b"#", DAFLUN, ctx)?;
                ERRINT(b"#", IOSTAT, ctx);
                SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
                CHKOUT(b"DAFAC", ctx)?;
                return Ok(());
            }

            RECNO = (RECNO + 1);
            CURPOS = 1;
            fstr::assign(&mut CRECRD, b" ");
        }
        //
        // Append the end-of-line marker to the comment line that we just
        // placed into the comment record.
        //
        fstr::assign(fstr::substr_mut(&mut CRECRD, CURPOS..=CURPOS), &save.EOLMRK);
        CURPOS = (CURPOS + 1);
    }
    //
    // We have now finished processing all of the comment lines in
    // BUFFER, so we need write the current record to the file. This
    // record will always contain something, so we always need to write
    // it.
    //
    if (CURPOS > MXCREC) {
        //
        // If we have completely filled the comment record, the last
        // character of the last line n the buffer coincides with the
        // last character in the comment record, then we need to write
        // the record and get set up to add the end of comments mark on
        // the next record.
        //
        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::UnformattedWriter::new(ctx.io_unit(DAFLUN)?, Some(RECNO))?;
            IOSTAT = io::capture_iostat(|| {
                writer.start()?;
                writer.write_str(&CRECRD)?;
                writer.finish()?;
                Ok(())
            })?;
        }

        if (IOSTAT != 0) {
            SETMSG(
                b"Error writing to record # of the binary file named \'#\'. IOSTAT = #.",
                ctx,
            );
            ERRINT(b"#", RECNO, ctx);
            ERRFNM(b"#", DAFLUN, ctx)?;
            ERRINT(b"#", IOSTAT, ctx);
            SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
            CHKOUT(b"DAFAC", ctx)?;
            return Ok(());
        }

        RECNO = (RECNO + 1);
        CURPOS = 1;
        fstr::assign(&mut CRECRD, b" ");
    }
    //
    // Add the end of comments mark to the final comment record and
    // write it to the file.
    //
    fstr::assign(fstr::substr_mut(&mut CRECRD, CURPOS..=CURPOS), &save.EOCMRK);

    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::UnformattedWriter::new(ctx.io_unit(DAFLUN)?, Some(RECNO))?;
        IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            writer.write_str(&CRECRD)?;
            writer.finish()?;
            Ok(())
        })?;
    }

    if (IOSTAT != 0) {
        SETMSG(
            b"Error writing to record # of the binary file named \'#\'. IOSTAT = #.",
            ctx,
        );
        ERRINT(b"#", RECNO, ctx);
        ERRFNM(b"#", DAFLUN, ctx)?;
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
        CHKOUT(b"DAFAC", ctx)?;
        return Ok(());
    }
    //
    // Check out and leave DAFAC.
    //
    CHKOUT(b"DAFAC", ctx)?;
    Ok(())
}
