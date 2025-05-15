//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const IFNLEN: i32 = 60;
const MAXCPR: i32 = 1000;

struct SaveVars {
    EOT: Vec<u8>,
    IFNAME: Vec<u8>,
    NULL: Vec<u8>,
    RECORD: Vec<u8>,
    TEMP: Vec<u8>,
    BOL: i32,
    BWARD: i32,
    DAFU: i32,
    EOL: i32,
    FREE: i32,
    FWARD: i32,
    HANBUF: i32,
    IOSTAT: i32,
    ND: i32,
    NI: i32,
    NRR: i32,
    POSNUL: i32,
    REC: i32,
    TMPLEN: i32,
    CALLED: bool,
    EOCSAV: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut EOT = vec![b' '; 1 as usize];
        let mut IFNAME = vec![b' '; IFNLEN as usize];
        let mut NULL = vec![b' '; 1 as usize];
        let mut RECORD = vec![b' '; MAXCPR as usize];
        let mut TEMP = vec![b' '; MAXCPR as usize];
        let mut BOL: i32 = 0;
        let mut BWARD: i32 = 0;
        let mut DAFU: i32 = 0;
        let mut EOL: i32 = 0;
        let mut FREE: i32 = 0;
        let mut FWARD: i32 = 0;
        let mut HANBUF: i32 = 0;
        let mut IOSTAT: i32 = 0;
        let mut ND: i32 = 0;
        let mut NI: i32 = 0;
        let mut NRR: i32 = 0;
        let mut POSNUL: i32 = 0;
        let mut REC: i32 = 0;
        let mut TMPLEN: i32 = 0;
        let mut CALLED: bool = false;
        let mut EOCSAV: bool = false;

        Self {
            EOT,
            IFNAME,
            NULL,
            RECORD,
            TEMP,
            BOL,
            BWARD,
            DAFU,
            EOL,
            FREE,
            FWARD,
            HANBUF,
            IOSTAT,
            ND,
            NI,
            NRR,
            POSNUL,
            REC,
            TMPLEN,
            CALLED,
            EOCSAV,
        }
    }
}

/// SPK and CK, read first line of comments
///
/// Read the first line of text from the comment area
/// of a binary SPK or CK file.
///
/// # Required Reading
///
/// * [SPC](crate::required_reading::spc)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle assigned to binary SPK or CK file.
///  LINE       O   First line of text from the comment area.
///  EOC        O   End of comments?
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle assigned to the binary SPK or CK file
///           which has been opened for read access.
///
///           Use the SPICELIB routine DAFOPR to open the file for read
///           access and get HANDLE, unless SPKLEF or CKLPF has already
///           been called and returned the handle. This file is
///           unchanged by calling SPCRFL.
/// ```
///
/// # Detailed Output
///
/// ```text
///  LINE     is the first line of text from the comment area of
///           the SPK or CK file specified by HANDLE. LINE may
///           be blank.
///
///  EOC      is .TRUE. if the comment area is empty. If there
///           are comments in the comment area, then EOC is .FALSE.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the comment area of the SPK or CK file is empty, LINE
///      will be blank.
///
///  2)  If the first line of comments in the comment area is longer
///      than the declared length of LINE, it will be truncated to
///      fit into the variable.
///
///  3)  If there is a problem reading from the comment area, the error
///      SPICE(FILEREADFAILED) is signaled.
///
///  4)  If the comments are not in the correct format, the error
///      SPICE(FORMATERROR) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See argument HANDLE.
/// ```
///
/// # Particulars
///
/// ```text
///  The structure of SPK and CK files accommodates comments in
///  addition to data. The following routines are available
///  for accessing the comment area of a binary SPK or CK file:
///
///        SPCAC           add comments
///
///        SPCEC           extract comments
///
///        SPCDC           delete comments
///
///        SPCRFL          read first line of comments
///
///        SPCRNL          read next line of comments
///
///  Note that comments must consist of only text, that is, printable
///  ASCII characters, specifically ASCII 32-126. This excludes
///  tabs (ASCII 9) and control characters.
///
///  The SPC conversion routines---SPCB2A, SPCA2B, SPCB2T, and
///  SPCT2B---include these comments when converting SPK and CK
///  files between binary and text formats.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose we have a binary SPK file called A.BSP. The following
///  code fragment searches the comment area for a lines containing
///  the character string `SOURCE' and writes the lines to standard
///  output.
///
///   C
///   C     Open the binary SPK file and get its handle.
///   C
///         CALL DAFOPR ( 'A.BSP', HANDLE )
///
///   C
///   C     Read the first line of comments.
///   C
///         CALL SPCRFL ( HANDLE, LINE, EOC )
///
///   C
///   C     Search for the string 'SOURCE' in the line. If
///   C     it is found, write the line. Then get the next
///   C     line of comments and repeat as long as we're not
///   C     at the end.
///   C
///         DO WHILE ( .NOT. EOC )
///
///            IF (  POS ( LINE, 'SOURCE', 1 ) .NE. 0  ) THEN
///               WRITE (*,*) LINE
///            END IF
///
///            CALL SPCRNL ( LINE, EOC )
///
///         END DO
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine assumes that the comment area of the binary SPK
///      or CK file contains only text stored by SPCAC. Comments
///      written any other way may not be handled properly.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  J.E. McLean        (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 17-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
///         Moved the contents of the $Files section to the description of
///         HANDLE in $Detailed_Input section, and referred to it from
///         $Files.
///
/// -    SPICELIB Version 2.0.0, 16-NOV-2001 (FST)
///
///         Updated this routine to utilize the new handle manager
///         interfaces.
///
/// -    SPICELIB Version 1.1.0, 27-JUL-1992 (KRG)
///
///         Removed a call to the SPICELIB subroutine SUFFIX() which
///         was used to join two parts of a comment line that may be
///         broken across two comment records. The problem was, SUFFIX
///         cannot know about leading/embedded blanks when it appends, so
///         blanks were inadvertently removed when they happened to be
///         stored at the end of comment record.
///
///         Added the variable TMPLEN to record the length of the first
///         part of a comment line that may be broken across comment
///         records.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 15-APR-1991 (JEM)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 2.0.0, 16-NOV-2001 (FST)
///
///         Calls to DAFHLU now lock handles to their logical units.
///         While at first glance it may seem this is the appropriate
///         course of action due to the buffering of the logical unit
///         by this routine for its entry point, adding a call to
///         ZZDDHUNL in the entry point removes the need to lock DAFU
///         to its handle. The value of HANDLE is now buffered in
///         HANBUF, to allow the entry point to retrieve a logical
///         unit.
/// ```
pub fn spcrfl(
    ctx: &mut SpiceContext,
    handle: i32,
    line: &mut str,
    eoc: &mut bool,
) -> crate::Result<()> {
    SPCRFL(
        handle,
        fstr::StrBytes::new(line).as_mut(),
        eoc,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPCRFL ( SPK and CK, read first line of comments )
pub fn SPCRFL(
    HANDLE: i32,
    LINE: &mut [u8],
    EOC: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    // IFNLEN      is the length of a DAF internal file name.
    //
    // MAXCPR      is the maximum number of characters per DAF record and
    //             hence the maximum comment line length.
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
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"SPCRFL", ctx)?;
    }

    //
    // SPCRFL has been called for this file.
    //
    save.CALLED = true;

    //
    // Read the file record to find out if the DAF contains any
    // reserved records.  The reserved records in an array file
    // are stored between the first record and the first summary
    // record.  FWARD is the record number of that first summary
    // record, and NRR is the number of reserved records in the file.
    //
    // If there are no reserved records, there are no comments.
    //
    DAFRFR(
        HANDLE,
        &mut save.ND,
        &mut save.NI,
        &mut save.IFNAME,
        &mut save.FWARD,
        &mut save.BWARD,
        &mut save.FREE,
        ctx,
    )?;
    save.NRR = (save.FWARD - 2);

    if (save.NRR == 0) {
        fstr::assign(LINE, b" ");
        *EOC = true;
        save.EOCSAV = *EOC;
        CHKOUT(b"SPCRFL", ctx)?;
        return Ok(());
    }

    //
    // We need to read directly from the SPK/CK file, using a logical
    // unit instead of a handle.
    //
    ZZDDHHLU(HANDLE, b"DAF", false, &mut save.DAFU, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SPCRFL", ctx)?;
        return Ok(());
    }

    //
    // Buffer the value of HANDLE.
    //
    save.HANBUF = HANDLE;

    //
    // In the comment area, NULL means end-of-line, and EOT means
    // end-of-transmission, or in other words, end-of-comments.
    //
    fstr::assign(&mut save.NULL, &intrinsics::CHAR(0));
    fstr::assign(&mut save.EOT, &intrinsics::CHAR(4));

    //
    // Read the first reserved record.
    //
    save.REC = 2;

    {
        use f2rust_std::{
            data::Val,
            io::{self, Reader},
        };

        let mut reader = io::UnformattedReader::new(ctx.io_unit(save.DAFU)?, Some(save.REC))?;
        save.IOSTAT = io::capture_iostat(|| {
            reader.start()?;
            reader.read_str(&mut save.RECORD)?;
            reader.finish()?;
            Ok(())
        })?;
    }

    if (save.IOSTAT != 0) {
        SETMSG(b"Error reading comment area of the binary file named FNM at record #.  Value of IOSTAT is #.", ctx);
        ERRINT(b"#", save.REC, ctx);
        ERRINT(b"#", save.IOSTAT, ctx);
        ERRFNM(b"FNM", save.DAFU, ctx)?;
        SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
        CHKOUT(b"SPCRFL", ctx)?;
        return Ok(());
    }

    //
    // The first line of comments begins with the first character
    // of the record.  A NULL character specifies the end.
    //
    save.POSNUL = POS(&save.RECORD, &save.NULL, 1);

    if (save.POSNUL == 0) {
        //
        // No NULL is in the record, so LINE is just the whole
        // record.  (The maximum length of a line written to
        // the comment area by SPCAC is MAXCPR characters).
        //
        save.EOL = MAXCPR;
    } else {
        //
        // The end of the line precedes the NULL character.
        //
        save.EOL = (save.POSNUL - 1);
    }

    //
    // Now we have the position of the end of the first line.
    // Assign it to LINE.  We're not yet at the end of comments,
    // since we have a line to return.  If the first character
    // was a NULL, the line is blank.
    //
    if (save.EOL == 0) {
        fstr::assign(LINE, b" ");
    } else {
        fstr::assign(LINE, fstr::substr(&save.RECORD, 1..=save.EOL));
    }

    *EOC = false;
    save.EOCSAV = *EOC;

    CHKOUT(b"SPCRFL", ctx)?;
    Ok(())
}

/// SPK and CK, read next line of comments
///
/// Continue reading lines from the comment area of a binary
/// SPK or CK file specified by the most recent call to
/// the routine SPCRFL.
///
/// # Required Reading
///
/// * [SPC](crate::required_reading::spc)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  LINE       O   Next line of text from the comment area.
///  EOC        O   End of comments?
/// ```
///
/// # Detailed Output
///
/// ```text
///  LINE     is the next line of text from the comment area of
///           the SPK or CK file. LINE may be blank.
///           SPCRFL reads the first line of comments from
///           a specified binary SPK or CK file. Once SPCRFL
///           has been called, SPCRNL may be called repetitively
///           to read the next lines of the comment area until
///           the end.
///
///  EOC      is .TRUE. if there are no more comments to read.
///           Otherwise, EOC is .FALSE.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If SPCRFL is not called prior to calling SPCRNL, the error
///      SPICE(SPCRFLNOTCALLED) is signaled.
///
///  2)  If the most recent call to SPCRFL returned EOC with the value
///      true, then SPCRNL will return EOC with the same value.
///
///  3)  If EOC is .TRUE., LINE will be blank.
///
///  4)  If the first line of comments in the comment area is longer
///      than the declared length of LINE, it will be truncated to
///      fit into the variable.
///
///  5)  If there is a problem reading from the comment area, the error
///      SPICE(FILEREADFAILED) is signaled.
///
///  6)  If the comments are not in the correct format, the error
///      SPICE(FORMATERROR) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  The handle of the binary SPK or CK is specified with the routine
///  SPCRFL.
/// ```
///
/// # Particulars
///
/// ```text
///  The structure of SPK and CK files accommodates comments in
///  addition to data. The following five routines are available
///  for accessing the comment area of a binary SPK or CK file:
///
///        SPCAC           add comments
///
///        SPCEC           extract comments
///
///        SPCDC           delete comments
///
///        SPCRFL          read first line of comments
///
///        SPCRNL          read next line of comments
///
///  Note that comments must consist of only text, that is, printable
///  ASCII characters, specifically ASCII 32-126. This excludes
///  tabs (ASCII 9) and control characters.
///
///  The SPC conversion routines---SPCB2A, SPCA2B, SPCB2T, and
///  SPCT2B---include these comments when converting SPK and CK
///  files between binary and text formats.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose we have a binary SPK file called A.BSP. The following
///  code fragment searches the comment area for a lines containing
///  the character string `SOURCE' and writes the lines to standard
///  output.
///
///   C
///   C     Open the binary SPK file and get its handle.
///   C
///         CALL DAFOPR ( 'A.BSP', HANDLE )
///
///   C
///   C     Read the first line of comments.
///   C
///         CALL SPCRFL ( HANDLE, LINE, EOC )
///
///   C
///   C     Search for the string 'SOURCE' in the line. If
///   C     it is found, write the line. Then get the next
///   C     line of comments and repeat as long as we're not
///   C     at the end.
///   C
///         DO WHILE ( .NOT. EOC )
///
///            IF (  POS ( LINE, 'SOURCE', 1 ) .NE. 0  ) THEN
///               WRITE (*,*) LINE
///            END IF
///
///            CALL SPCRNL ( LINE, EOC )
///
///         END DO
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine assumes that the comment area of the binary SPK
///      or CK file contains only text stored by SPCAC. Comments
///      written any other way may not be handled properly.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  J.E. McLean        (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 17-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.0.0, 16-NOV-2001 (FST)
///
///         Updated this entry point to utilize the handle manager
///         interfaces. See the $Revisions section of the subroutine
///         header above for a detailed discussion of the changes.
///
/// -    SPICELIB Version 1.1.0, 27-JUL-1992 (KRG)
///
///         Removed a call to the SPICELIB subroutine SUFFIX() which
///         was used to join two parts of a comment line that may be
///         broken across two comment records. The problem was, SUFFIX
///         cannot know about leading/embedded blanks when it appends, so
///         blanks were inadvertently removed when they happened to be
///         stored at the end of comment record.
///
///         Added the variable TMPLEN to record the length of the first
///         part of a comment line that may be broken across comment
///         records.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 15-APR-1991 (JEM)
/// ```
pub fn spcrnl(ctx: &mut SpiceContext, line: &mut str, eoc: &mut bool) -> crate::Result<()> {
    SPCRNL(fstr::StrBytes::new(line).as_mut(), eoc, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPCRNL ( SPK and CK, read next line of comments )
pub fn SPCRNL(LINE: &mut [u8], EOC: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"SPCRNL", ctx)?;
    }

    //
    // If SPCRFL hasn't been called, then we don't know which
    // file to read from.
    //
    if !save.CALLED {
        SETMSG(b"You must call SPCRFL to read the first line of comments before calling SPCRNL to read the next line.", ctx);
        SIGERR(b"SPICE(SPCRFLNOTCALLED)", ctx)?;
        CHKOUT(b"SPCRNL", ctx)?;
        return Ok(());
    }

    //
    // If we were at the end of comments before, then we're still
    // at the end.
    //
    if save.EOCSAV {
        fstr::assign(LINE, b" ");
        *EOC = true;
        CHKOUT(b"SPCRNL", ctx)?;
        return Ok(());
    }

    //
    // Retrieve a logical unit for HANBUF.
    //
    ZZDDHHLU(save.HANBUF, b"DAF", false, &mut save.DAFU, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SPCRNL", ctx)?;
        return Ok(());
    }

    //
    // RECORD contains the last line and EOL is the position of
    // the end of that line.  Now we need to determine the
    // position of the beginning of the next line (BOL).  There
    // is a NULL between EOL and BOL, so BOL is two more than
    // EOL.  If that puts BOL off the end of the current RECORD,
    // then we have to go to the next record.
    //
    save.BOL = (save.EOL + 2);

    if (save.BOL > MAXCPR) {
        save.BOL = (save.BOL - MAXCPR);

        save.REC = (save.REC + 1);

        //
        // Check to make sure that we're not reading past the
        // reserved records.  FWARD is the "forward list pointer".
        // It is the number of the first record after the reserved
        // records.
        //
        if (save.REC >= save.FWARD) {
            SETMSG(b"The comment area of the binary file named FNM is formatted incorrectly. The end of the comments is not marked as it should be in record #. Calling SPCDC or DAFRRR will remove the comment area and eliminate this format error. Comments should be written ONLY by SPCAC.", ctx);
            ERRINT(b"#", (save.REC - 1), ctx);
            ERRFNM(b"FNM", save.DAFU, ctx)?;
            SIGERR(b"SPICE(FORMATERROR)", ctx)?;
            CHKOUT(b"SPCRNL", ctx)?;
            return Ok(());
        }

        //
        // All clear to read the record.
        //
        {
            use f2rust_std::{
                data::Val,
                io::{self, Reader},
            };

            let mut reader = io::UnformattedReader::new(ctx.io_unit(save.DAFU)?, Some(save.REC))?;
            save.IOSTAT = io::capture_iostat(|| {
                reader.start()?;
                reader.read_str(&mut save.RECORD)?;
                reader.finish()?;
                Ok(())
            })?;
        }

        if (save.IOSTAT != 0) {
            SETMSG(b"Error reading comment area of the binary file named FNM at record #.  Value of IOSTAT is #.", ctx);
            ERRINT(b"#", save.REC, ctx);
            ERRINT(b"#", save.IOSTAT, ctx);
            ERRFNM(b"FNM", save.DAFU, ctx)?;
            SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
            CHKOUT(b"SPCRNL", ctx)?;
            return Ok(());
        }
    }

    //
    // RECORD is now the record of the file that contains the
    // beginning of the next line (BOL).  The line may not
    // exist or may be blank or may be a character string.
    //

    if fstr::eq(fstr::substr(&save.RECORD, save.BOL..=save.BOL), &save.EOT) {
        //
        // There isn't a next line to get.  We're at the end of
        // the comments.
        //
        fstr::assign(LINE, b" ");
        *EOC = true;
        save.EOCSAV = *EOC;
        CHKOUT(b"SPCRNL", ctx)?;
        return Ok(());
    }

    if fstr::eq(fstr::substr(&save.RECORD, save.BOL..=save.BOL), &save.NULL) {
        //
        // Just a NULL means a blank line.
        //
        save.EOL = (save.BOL - 1);
        fstr::assign(LINE, b" ");
        *EOC = false;
        save.EOCSAV = *EOC;
        CHKOUT(b"SPCRNL", ctx)?;
        return Ok(());
    }

    //
    // The beginning of the next line is a character.  Now we have
    // to find the end.  It precedes the next NULL.
    //
    save.POSNUL = POS(&save.RECORD, &save.NULL, save.BOL);

    if (save.POSNUL != 0) {
        save.EOL = (save.POSNUL - 1);
        fstr::assign(LINE, fstr::substr(&save.RECORD, save.BOL..=save.EOL));
        *EOC = false;
        save.EOCSAV = *EOC;
    } else {
        //
        // There is no NULL in the rest of the record, so we have to
        // read the next record to find it.  Save the first part
        // of the line in TEMP.
        //
        fstr::assign(
            &mut save.TEMP,
            fstr::substr(&save.RECORD, save.BOL..=MAXCPR),
        );
        save.TMPLEN = ((MAXCPR - save.BOL) + 1);

        save.REC = (save.REC + 1);

        //
        // Check to make sure that we're not reading past the
        // reserved records.  FWARD is the "forward list pointer".
        // It is the number of the first record after the reserved
        // records.
        //
        if (save.REC >= save.FWARD) {
            SETMSG(b"The comment area of the binary file named FNM is formatted incorrectly. The end of the comments is not marked as it should be in record #. Calling SPCDC or DAFRRR will remove the comment area and eliminate this format error. Comments should be written ONLY by SPCAC.", ctx);
            ERRINT(b"#", (save.REC - 1), ctx);
            ERRFNM(b"FNM", save.DAFU, ctx)?;
            SIGERR(b"SPICE(FORMATERROR)", ctx)?;
            CHKOUT(b"SPCRNL", ctx)?;
            return Ok(());
        }

        //
        // All clear to read the record.
        //
        {
            use f2rust_std::{
                data::Val,
                io::{self, Reader},
            };

            let mut reader = io::UnformattedReader::new(ctx.io_unit(save.DAFU)?, Some(save.REC))?;
            save.IOSTAT = io::capture_iostat(|| {
                reader.start()?;
                reader.read_str(&mut save.RECORD)?;
                reader.finish()?;
                Ok(())
            })?;
        }

        if (save.IOSTAT != 0) {
            SETMSG(b"Error reading comment area of the binary file named FNM at record #.  Value of IOSTAT is #.", ctx);
            ERRINT(b"#", save.REC, ctx);
            ERRINT(b"#", save.IOSTAT, ctx);
            ERRFNM(b"FNM", save.DAFU, ctx)?;
            SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
            CHKOUT(b"SPCRNL", ctx)?;
            return Ok(());
        }

        //
        // There should be a null in this new record.  If there isn't
        // then something is wrong.  The maximum length of a line is
        // MAXCPR characters according to SPCAC.  So BOL and the NULL
        // should be in the same record or in adjacent records.
        //
        save.POSNUL = POS(&save.RECORD, &save.NULL, 1);

        if (save.POSNUL == 0) {
            SETMSG(b"Cannot find the end of the line.  There is something wrong with the format of thecomments.", ctx);
            SIGERR(b"SPICE(FORMATERROR)", ctx)?;
            CHKOUT(b"SPCRNL", ctx)?;
            return Ok(());
        }

        save.EOL = (save.POSNUL - 1);

        //
        // EOL is zero if the NULL was the first character of the
        // new record.  Otherwise, concatenate the two parts of
        // the line from the two adjacent records.  Then assign the
        // values of LINE and EOC.
        //
        if (save.EOL != 0) {
            fstr::assign(
                fstr::substr_mut(&mut save.TEMP, (save.TMPLEN + 1)..),
                fstr::substr(&save.RECORD, 1..=save.EOL),
            );
        }

        fstr::assign(LINE, &save.TEMP);
        *EOC = false;
        save.EOCSAV = *EOC;
    }

    CHKOUT(b"SPCRNL", ctx)?;
    Ok(())
}
