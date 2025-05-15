//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const IFNLEN: i32 = 60;
const MAXCPR: i32 = 1000;

/// SPK and CK, add comments
///
/// Store text from a text file in the comment area of a binary SPK
/// or CK file, appending it to whatever text may already have
/// been stored there.
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
///  UNIT       I   Logical unit connected to comment file.
///  BMARK      I   Beginning marker.
///  EMARK      I   Ending marker.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle assigned to the binary SPK or CK file
///           which has been opened for write access.
///
///           Use the SPICELIB routine DAFOPW to open the file for
///           write access and get HANDLE. Upon exit, this binary file
///           will contain the specified text from the comment file in
///           its comment area, appended to whatever text may already
///           have been stored there. SPCAC will include an extra blank
///           line between the original text and the appended text.
///
///  UNIT     is the logical unit connected to the comment file.
///           This file must contain only text (printable
///           ASCII characters, namely ASCII 32-126).  Open this
///           file with read access and get its UNIT using TXTOPR.
///
///  BMARK,
///  EMARK    are markers that delimit a group of consecutive
///           lines in the text file (UNIT), that get stored in the
///           comment area of the binary file (HANDLE).
///
///           The group of lines begins with the line that
///           immediately follows the first line of the file
///           equivalent to BMARK. It ends with line that
///           precedes the next line of the file equivalent to
///           EMARK, including blank lines. Leading and
///           trailing blanks are ignored when testing for
///           equivalence.
///
///           By convention, if BMARK is blank, the first line of
///           the group is the first line of the file; if EMARK is
///           blank, the last line of the group is the last line
///           of the file.
///
///           If a marker is non-blank and is not found, or if
///           non-blank markers are on successive lines in the text
///           file, nothing gets stored in the comment area of
///           the binary file.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the specified DAF file is not open for write access, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  2)  If there is a problem reading from the comment area of the
///      binary file, the error SPICE(FILEREADFAILED) is signaled.
///
///  3)  If there is a problem writing to the comment area of the
///      binary file, the error SPICE(FILEWRITEFAILED) is signaled.
///
///  4)  If there is a problem reading from the text file, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  5)  If a non-printing ASCII character is encountered in the
///      comments, an error is signaled by a routine in the call tree
///      of this routine.
/// ```
///
/// # Files
///
/// ```text
///  See arguments HANDLE and UNIT.
/// ```
///
/// # Particulars
///
/// ```text
///  The structure of SPK and CK files accommodates comments in
///  addition to data. The following three routines are available
///  for accessing the comment area of a binary SPK or CK file:
///
///        SPCAC           add comments
///
///        SPCEC           extract comments
///
///        SPCDC           delete comments
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
///  Suppose we have a binary SPK file called A.BSP and we have
///  a text file called COMMENTS.TXT that contains comments
///  about the data in the SPK file.
///
///  The following code fragment stores the entire contents of
///  COMMENTS.TXT in the comment area of A.BSP.
///
///         CALL DAFOPW ( 'A.BSP', HANDLE )
///
///         CALL TXTOPR ( 'COMMENTS.TXT', UNIT )
///
///         BMARK = ' '
///         EMARK = ' '
///
///         CALL SPCAC  ( HANDLE, UNIT, BMARK, EMARK )
///
///         CLOSE ( UNIT )
///
///  Now suppose MORE.TXT is a text file that contains additional
///  information about the data in A.BSP, as well as information
///  about several other SPK files.  The contents of MORE.TXT are
///
///            \begin A info
///
///              DATAFILE = A
///              SOURCE   = JPL, 1990 September 12
///              MISSION  = Galileo
///
///            \end A info
///
///            \begin B info
///
///              DATAFILE = B
///              SOURCE   = JPL, 1988 August 1
///              MISSION  = Voyager 2
///
///            \end B info
///
///            \begin C info
///
///              DATAFILE = C
///              SOURCE   = JPL, 1994 January 31
///              MISSION  = Mars Observer
///
///            \end C info
///
///  This code fragment stores only the information that pertains
///  to A.BSP, and appends it to the text from COMMENTS.TXT that
///  has already been stored in the comment area of A.BSP
///
///         CALL TXTOPR ( 'MORE.TXT', UNIT )
///
///         BMARK = '\begin A info'
///         EMARK = '\end A info'
///
///         CALL SPCAC  ( HANDLE, UNIT, BMARK, EMARK )
///
///         CLOSE ( UNIT )
///
///         CALL DAFCLS ( HANDLE )
///
///  Note that, ignoring leading and trailing blanks, BMARK and
///  EMARK are exactly equivalent to lines in the text file.
///  If the assignment had been instead BMARK = '\ begin A info',
///  with an extra space between the slash and the word begin,
///  SPCAC would not have found the marker and no comments from
///  the text file would be written to the binary file.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The lines in the comment file should not exceed 1000
///      characters in length. SPCAC truncates lines longer than
///      this on the right.
///
///  2)  Use TXTOPR to open text files for read access and get
///      the logical unit. System dependencies regarding
///      opening text files have been isolated in the routines
///      TXTOPN and TXTOPR.
///
///  3)  This routine assumes that the comment area of the binary SPK
///      or CK file contains only text stored by SPCAC. Comments
///      written any other way may not be handled properly.
///
///  4)  The comment area of the binary SPK or CK file must contain
///      only one EOT character. This routine seeks back from the
///      last reserved record searching for the first EOT it
///      encounters. Thus the multiple EOT's will cause the appended
///      comments to be invisible to any reader that starts at the
///      first reserved record and reads until the first EOT present.
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
/// -    SPICELIB Version 2.1.0, 13-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
///         Moved the contents of the $Files section to the description of
///         HANDLE and UNIT in $Detailed_Input section, and referred to
///         them from $Files. Removed unnecessary entries from $Revisions
///         section.
///
/// -    SPICELIB Version 2.0.0, 16-NOV-2001 (FST)
///
///         Updated this routine to utilize new handle manager
///         interfaces.
///
/// -    SPICELIB Version 1.3.0, 12-FEB-1999 (FST)
///
///         Modified the EOT search code to seek back through any
///         reserved records, as opposed to just the last one. This
///         provides the flexibility to use DAFOPN to reserve records
///         that may ultimately be used for storing comments. As a direct
///         result of these changes the SPICE(MISSINGEOT) error is no
///         longer signaled, since if no EOT is found in the reserved
///         records, they are considered available for writes.
///
/// -    SPICELIB Version 1.2.0, 12-MAY-1994 (KRG)
///
///         Added an IF statement so that DAFARR is called only if new
///         reserved records need to be added to the comment area.
///
/// -    SPICELIB Version 1.1.0, 09-APR-1993 (KRG)
///
///         Added code to initialize the variable LASTRR to zero. This
///         variable is used in a function call, MAX ( LASTRR-1, 1 ),
///         regardless of whether or not any reserved records are in
///         the file. Thus the need to initialize it.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 05-APR-1991 (JEM)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 2.0.0, 16-NOV-2001 (FST)
///
///         This routine now utilizes DAFSIH to determine if
///         HANDLE is open for WRITE access. The call to DAFHLU
///         has been replaced with a call to ZZDDHHLU, the handle
///         manager interface for retrieving a logical unit.
///         DAFHLU is no longer used, since it locks the unit
///         returned to its HANDLE, tying up resources in the
///         handle manager.
/// ```
pub fn spcac(
    ctx: &mut SpiceContext,
    handle: i32,
    unit: i32,
    bmark: &str,
    emark: &str,
) -> crate::Result<()> {
    SPCAC(
        handle,
        unit,
        bmark.as_bytes(),
        emark.as_bytes(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPCAC ( SPK and CK, add comments )
pub fn SPCAC(
    HANDLE: i32,
    UNIT: i32,
    BMARK: &[u8],
    EMARK: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut DATA = [b' '; (MAXCPR + 2) as usize];
    let mut EOT = [b' '; 1 as usize];
    let mut IFNAME = [b' '; IFNLEN as usize];
    let mut LINE = [b' '; MAXCPR as usize];
    let mut NULL = [b' '; 1 as usize];
    let mut RECORD = [b' '; MAXCPR as usize];
    let mut BLINE: i32 = 0;
    let mut BWARD: i32 = 0;
    let mut CHARS: i32 = 0;
    let mut DAFU: i32 = 0;
    let mut EOL: i32 = 0;
    let mut ELINE: i32 = 0;
    let mut FREE: i32 = 0;
    let mut FWARD: i32 = 0;
    let mut I: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut LASTRR: i32 = 0;
    let mut LINES: i32 = 0;
    let mut ND: i32 = 0;
    let mut NI: i32 = 0;
    let mut NR: i32 = 0;
    let mut NRR: i32 = 0;
    let mut POS: i32 = 0;
    let mut POSEOT: i32 = 0;
    let mut REC: i32 = 0;
    let mut SPACE: i32 = 0;
    let mut START: i32 = 0;
    let mut TOTAL: i32 = 0;
    let mut FOUND: bool = false;

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
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"SPCAC", ctx)?;
    }

    //
    // Before doing anything, determine if the file associated with
    // HANDLE is available for WRITE access.
    //
    DAFSIH(HANDLE, b"WRITE", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SPCAC", ctx)?;
        return Ok(());
    }

    //
    // Rewind the comment file - we'll start the search for BMARK
    // and EMARK at the beginning.  Once we have located the markers,
    // count the number of lines between them and the number of
    // characters in those lines, ignoring trailing blanks.
    //
    // We rewind the file so that we know where the file pointer is.
    // LOCLN will compute BLINE and ELINE taking the current position
    // of the file pointer as line 1.
    //
    {
        use f2rust_std::io;

        let specs = io::PosSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.rewind(specs)?;
    }

    LOCLN(
        UNIT, BMARK, EMARK, &mut LINE, &mut BLINE, &mut ELINE, &mut FOUND, ctx,
    )?;

    //
    // If the markers are not found, or if BMARK and EMARK are on
    // successive lines, there is nothing to put in the comment area.
    //
    if !FOUND {
        CHKOUT(b"SPCAC", ctx)?;
        return Ok(());
    }

    //
    // Adjust BLINE and ELINE so we are pointing to the group of lines
    // BETWEEN the markers.  Check and make sure there is at least one
    // line in the group.
    //
    if fstr::ne(BMARK, b" ") {
        BLINE = (BLINE + 1);
    }

    if fstr::ne(EMARK, b" ") {
        ELINE = (ELINE - 1);
    }

    if (BLINE > ELINE) {
        CHKOUT(b"SPCAC", ctx)?;
        return Ok(());
    }

    //
    // Calculate the number of lines and the total number of characters
    // in those lines.  The characters must all be printable, or
    // else COUNTC will signal an error.
    //
    LINES = ((ELINE - BLINE) + 1);
    CHARS = COUNTC(UNIT, BLINE, ELINE, &mut LINE, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SPCAC", ctx)?;
        return Ok(());
    }

    //
    // Read the file record to find out if the DAF contains any
    // reserved records.  The reserved records in an array file
    // are stored between the first record (the file record) and
    // the first summary record.  FWARD is the record number of
    // that first summary record, and NRR is the number of reserved
    // records in the file.
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
    NRR = (FWARD - 2);

    //
    // Get the logical unit for reading from and writing to the DAF.
    //
    ZZDDHHLU(HANDLE, b"DAF", false, &mut DAFU, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SPCAC", ctx)?;
        return Ok(());
    }

    //
    // Assign the value of NULL and EOT.  NULL gets appended to the
    // end of each line of text.  EOT gets appended to the end of
    // all the comments.  Assign initial values for SPACE, RECORD,
    // and START.
    //
    fstr::assign(&mut NULL, &intrinsics::CHAR(0));
    fstr::assign(&mut EOT, &intrinsics::CHAR(4));
    SPACE = 0;
    fstr::assign(&mut RECORD, b" ");
    START = 0;
    LASTRR = 0;

    if (NRR != 0) {
        //
        // At this point, we know there exist reserved records in the
        // DAF. We need to search from the last record to the first,
        // seeking for the EOT (end of transmission) character, as it
        // marks the end of the comment region.
        //
        LASTRR = (FWARD - 1);
        I = (LASTRR + 1);
        POSEOT = 0;

        while ((I > 1) && (POSEOT == 0)) {
            //
            // Decrement the counter now.  This keeps it in
            // sync with the exit conditions.
            //
            I = (I - 1);
            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Reader},
                };

                let mut reader = io::UnformattedReader::new(ctx.io_unit(DAFU)?, Some(I))?;
                IOSTAT = io::capture_iostat(|| {
                    reader.start()?;
                    reader.read_str(&mut RECORD)?;
                    reader.finish()?;
                    Ok(())
                })?;
            }

            if (IOSTAT != 0) {
                SETMSG(
                    b"Error reading comment area of binary file named FILE.  IOSTAT = *.",
                    ctx,
                );
                ERRINT(b"*", IOSTAT, ctx);
                ERRFNM(b"FILE", DAFU, ctx)?;
                SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
                CHKOUT(b"SPCAC", ctx)?;
                return Ok(());
            }
            //
            // Call INDEX. If POSEOT is 0, then RECORD doesn't contain
            // the EOT character.
            //
            POSEOT = intrinsics::INDEX(&RECORD, &EOT);
        }

        //
        // The amount of free space in the reserved records of the
        // files is determined by the number of empty reserved
        // records ( LASTRR - I ), and the number of characters used
        // in last record with data (MAXCPR - POSEOT).
        //
        SPACE = ((MAXCPR * ((LASTRR - I) + 1)) - POSEOT);
        //
        // Adjust the value of LASTRR to indicate the record where
        // the EOT lies.  From here on out, the purpose of this
        // variable is to indicate where to start dumping comments.
        //
        LASTRR = I;
        //
        // If POSEOT is 0, then there are no comments in the file, but
        // there are reserved records.  Branch on this:
        //
        if (POSEOT == 0) {
            //
            // Leaving this string index at zero may be causing all sorts
            // of warning bells to go off in your head. However, before
            // this index value is used to address the contents of a
            // string it's incremented by 1.
            //
            START = POSEOT;
        //
        // Handle the case when POSEOT is non-zero.
        //
        } else {
            //
            // Replace the end-of-transmission character with a new line
            // character (we use null), so a blank line will come between
            // the old text and new text in the comment area.  START is the
            // position after which the first character of the new text
            // goes.
            //
            fstr::assign(fstr::substr_mut(&mut RECORD, POSEOT..=POSEOT), &NULL);

            START = POSEOT;
        }
    }

    //
    // Compute the number of records (NR) needed to store all of these
    // characters.
    //
    // Each line should end with a null (ASCII 0) character.  The final
    // line should also be followed by an end-of-transmission (ASCII 4)
    // character.  So the total is the number of characters, plus the
    // number of lines, plus one for the EOT.
    //
    // If the TOTAL fits in the SPACE available in the last reserved
    // record, we don't need to reserve any more.  Otherwise compute
    // the number we need.
    //
    TOTAL = ((CHARS + LINES) + 1);

    if ((TOTAL - SPACE) > 0) {
        NR = ((((TOTAL - SPACE) - 1) / MAXCPR) + 1);
    } else {
        NR = 0;
    }

    //
    // Reserve the records to create a comment area large enough
    // to hold it all, if we need to.  If we can't do it, there's no
    // point in going on.
    //
    if (NR > 0) {
        DAFARR(HANDLE, NR, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"SPCAC", ctx)?;
            return Ok(());
        }
    }

    //
    // Load the group of lines in the comment file into the reserved
    // records. Keep adding lines to the current record until it has
    // been filled, then write it to the DAF, and begin a new record.
    //
    REC = intrinsics::MAX0(&[(LASTRR - 1), 1]);
    POS = START;

    //
    // Rewind the text file then skip past the lines that we don't want
    // to position the file pointer at the correct record.
    //
    {
        use f2rust_std::io;

        let specs = io::PosSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.rewind(specs)?;
    }

    for L in 1..=(BLINE - 1) {
        {
            use f2rust_std::{
                data::Val,
                io::{self, Reader},
            };

            let mut reader = io::FormattedReader::new(ctx.io_unit(UNIT)?, None, b"(A)")?;
            IOSTAT = io::capture_iostat(|| {
                reader.start()?;
                reader.read_str(&mut LINE)?;
                reader.finish()?;
                Ok(())
            })?;
        }

        if (IOSTAT != 0) {
            SETMSG(
                b"Error reading line # in text file named FILE.  IOSTAT = *.",
                ctx,
            );
            ERRINT(b"#", L, ctx);
            ERRINT(b"*", IOSTAT, ctx);
            ERRFNM(b"FILE", UNIT, ctx)?;
            SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
            CHKOUT(b"SPCAC", ctx)?;
            return Ok(());
        }
    }

    //
    // Start reading the lines that we do want.  LINE is MAXCPR long
    // so that's the maximum number of characters that are read.
    //
    for L in 1..=LINES {
        {
            use f2rust_std::{
                data::Val,
                io::{self, Reader},
            };

            let mut reader = io::FormattedReader::new(ctx.io_unit(UNIT)?, None, b"(A)")?;
            IOSTAT = io::capture_iostat(|| {
                reader.start()?;
                reader.read_str(&mut LINE)?;
                reader.finish()?;
                Ok(())
            })?;
        }

        if (IOSTAT != 0) {
            SETMSG(
                b"Error reading line # in text file named FILE.  IOSTAT = *.",
                ctx,
            );
            ERRINT(b"#", ((L + BLINE) - 1), ctx);
            ERRINT(b"*", IOSTAT, ctx);
            ERRFNM(b"FILE", UNIT, ctx)?;
            SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
            CHKOUT(b"SPCAC", ctx)?;
            return Ok(());
        }

        //
        // Each line is followed by a null character.
        //
        fstr::assign(&mut DATA, &LINE);
        EOL = (LASTNB(&DATA) + 1);
        fstr::assign(fstr::substr_mut(&mut DATA, EOL..=EOL), &NULL);

        //
        // The final line is followed by an additional
        // end-of-transmission character.
        //
        if (L == LINES) {
            EOL = (EOL + 1);
            fstr::assign(fstr::substr_mut(&mut DATA, EOL..=EOL), &EOT);
        }

        //
        // Moving characters one at a time is slower, but simpler,
        // than trying to move them in blocks.
        //
        for C in 1..=EOL {
            //
            // If the current record is full, write it to the DAF.
            //
            if (POS == MAXCPR) {
                REC = (REC + 1);
                {
                    use f2rust_std::{
                        data::Val,
                        io::{self, Writer},
                    };

                    let mut writer = io::UnformattedWriter::new(ctx.io_unit(DAFU)?, Some(REC))?;
                    IOSTAT = io::capture_iostat(|| {
                        writer.start()?;
                        writer.write_str(&RECORD)?;
                        writer.finish()?;
                        Ok(())
                    })?;
                }

                if (IOSTAT == 0) {
                    fstr::assign(&mut RECORD, b" ");
                    POS = 0;
                } else {
                    SETMSG(
                        b"Error writing to record # of the binary file named FILE. IOSTAT = *.",
                        ctx,
                    );
                    ERRINT(b"#", REC, ctx);
                    ERRINT(b"*", IOSTAT, ctx);
                    ERRFNM(b"FILE", DAFU, ctx)?;
                    SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
                    CHKOUT(b"SPCAC", ctx)?;
                    return Ok(());
                }
            }

            //
            // Add the next character to the current record.
            //
            POS = (POS + 1);
            fstr::assign(
                fstr::substr_mut(&mut RECORD, POS..=POS),
                fstr::substr(&DATA, C..=C),
            );
        }
    }

    //
    // Write the final record to the DAF.
    //
    REC = (REC + 1);
    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::UnformattedWriter::new(ctx.io_unit(DAFU)?, Some(REC))?;
        IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            writer.write_str(&RECORD)?;
            writer.finish()?;
            Ok(())
        })?;
    }

    if (IOSTAT != 0) {
        SETMSG(b"Error writing the final record, record #, of the binary file named FILE.  IOSTAT = *.", ctx);
        ERRINT(b"#", REC, ctx);
        ERRINT(b"*", IOSTAT, ctx);
        ERRFNM(b"FILE", DAFU, ctx)?;
        SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
        CHKOUT(b"SPCAC", ctx)?;
        return Ok(());
    }

    CHKOUT(b"SPCAC", ctx)?;
    Ok(())
}
