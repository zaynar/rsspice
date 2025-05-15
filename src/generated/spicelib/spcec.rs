//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const IFNLEN: i32 = 60;
const MAXCPR: i32 = 1000;

/// SPK and CK, extract comments
///
/// Extract the text from the comment area of a binary SPK or CK file
/// and write it to a text file.
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
///  UNIT       I   Logical unit connected to text file.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle assigned to the binary SPK or CK file
///           which has been opened for read access.
///
///           Use the SPICELIB routine DAFOPR to open the file for read
///           access and get HANDLE unless SPKLEF or CKLPF has already
///           been called and returned the handle. This file is
///           unchanged by calling SPCEC.
///
///  UNIT     is the logical unit connected to the text file to
///           which the contents of the comment area of the SPK
///           or CK file will be written, beginning at the current
///           position of the file pointer.
///
///           Use the SPICELIB routine TXTOPN to open the file and get
///           UNIT. Upon exit, this file will contain the text from the
///           comment area of the binary SPK or CK file, beginning at
///           the line that was the position of the file pointer when
///           SPCEC was called. In other words, SPCEC does not rewind
///           or backspace this file before writing the text to it.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the comment area of the SPK or CK file is empty, nothing
///      will be written to the text file.
///
///  2)  If there is a problem reading from the comment area, the error
///      SPICE(FILEREADFAILED) is signaled.
///
///  3)  If there is a problem writing to the text file, the error
///      SPICE(FILEWRITEFAILED) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See argument HANDLE and UNIT.
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
///  Suppose we have a binary SPK file called A.BSP. The following
///  code fragment stores the contents of the comment area of A.BSP
///  in a text file called COMMENTS.TXT and surrounds the comments
///  with markers.
///
///         CALL DAFOPR ( 'A.BSP', HANDLE )
///
///         CALL TXTOPN ( 'COMMENTS.TXT', UNIT )
///
///         WRITE (UNIT,*) '\begincomments'
///
///         CALL SPCEC  ( HANDLE, UNIT )
///
///         WRITE (UNIT,*) '\endcomments'
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  Use TXTOPN to open new text files and get their logical unit.
///      There are system dependencies regarding opening text files,
///      and these have been isolated in the routines TXTOPN and
///      TXTOPR.
///
///  2)  This routine assumes that the comment area of the binary SPK
///      or CK file contains only text stored by SPCAC. Comments
///      written any other way may not be handled properly.
///
///  3)  This routine is only used to read records on environments
///      whose characters are a single byte in size. Updates
///      to this routine and routines in its call tree may be
///      required to properly handle other cases.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  J.E. McLean        (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 03-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
///         Moved the contents of the $Files section to the description of
///         HANDLE and UNIT in $Detailed_Input section, and referred to
///         them from $Files.
///
/// -    SPICELIB Version 2.0.0, 16-NOV-2001 (FST)
///
///         Updated this routine to utilize new handle manager
///         interfaces.
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
///         The call to DAFHLU has been replaced with a call to
///         ZZDDHHLU, the handle manager interface for retrieving
///         a logical unit. DAFHLU is no longer used, since it
///         locks the unit returned to its HANDLE, tying up resources
///         in the handle manager. A call to DAFSIH was inserted to
///         make certain that HANDLE is present in DAFAH's file table.
/// ```
pub fn spcec(ctx: &mut SpiceContext, handle: i32, unit: i32) -> crate::Result<()> {
    SPCEC(handle, unit, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPCEC ( SPK and CK, extract comments )
pub fn SPCEC(HANDLE: i32, UNIT: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut IFNAME = [b' '; IFNLEN as usize];
    let mut LINE = [b' '; MAXCPR as usize];
    let mut RECORD = [b' '; MAXCPR as usize];
    let mut EOT = [b' '; 1 as usize];
    let mut NULL = [b' '; 1 as usize];
    let mut BWARD: i32 = 0;
    let mut DAFU: i32 = 0;
    let mut FREE: i32 = 0;
    let mut FWARD: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut ND: i32 = 0;
    let mut NI: i32 = 0;
    let mut NRR: i32 = 0;
    let mut POS: i32 = 0;

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
        CHKIN(b"SPCEC", ctx)?;
    }

    //
    // First, check to see if HANDLE is a legitimate DAF handle.
    //
    DAFSIH(HANDLE, b"READ", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SPCEC", ctx)?;
        return Ok(());
    }

    //
    // Read the file record to find out if the DAF contains any
    // reserved records.  The reserved records in an array file
    // are stored between the first record and the first summary
    // record.  FWARD is the record number of that first summary
    // record, and NRR is the number of reserved records in the file.
    // If there are no reserved records, there's nothing to be done.
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

    if (NRR == 0) {
        CHKOUT(b"SPCEC", ctx)?;
        return Ok(());
    }

    //
    // We need to read directly from the SPK or CK file, using a logical
    // unit instead of a handle.
    //
    ZZDDHHLU(HANDLE, b"DAF", false, &mut DAFU, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SPCEC", ctx)?;
        return Ok(());
    }

    //
    // Load the contents of the reserved records into individual lines,
    // for printing.  Keep adding characters to the current line until
    // it has been filled, then write it to the text file, and
    // begin a new line.
    //
    // In the comment area, NULL means end-of-line, and EOT means
    // end-of-transmission, or in other words, end-of-comments.
    //
    fstr::assign(&mut NULL, &intrinsics::CHAR(0));
    fstr::assign(&mut EOT, &intrinsics::CHAR(4));

    fstr::assign(&mut LINE, b" ");
    fstr::assign(&mut RECORD, b" ");
    POS = 0;

    for REC in 1..=NRR {
        {
            use f2rust_std::{
                data::Val,
                io::{self, Reader},
            };

            let mut reader = io::UnformattedReader::new(ctx.io_unit(DAFU)?, Some((REC + 1)))?;
            IOSTAT = io::capture_iostat(|| {
                reader.start()?;
                reader.read_str(&mut RECORD)?;
                reader.finish()?;
                Ok(())
            })?;
        }

        if (IOSTAT != 0) {
            SETMSG(
                b"Error reading comment area of the binary file named FNM.  Value of IOSTAT is #.",
                ctx,
            );
            ERRINT(b"#", IOSTAT, ctx);
            ERRFNM(b"FNM", DAFU, ctx)?;
            SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
            CHKOUT(b"SPCEC", ctx)?;
            return Ok(());
        }

        for C in 1..=MAXCPR {
            //
            // End-of-transmission means we're done.
            //
            if fstr::eq(fstr::substr(&RECORD, C..=C), &EOT) {
                CHKOUT(b"SPCEC", ctx)?;
                return Ok(());

            //
            // NULL means that the current line is ready to be written to
            // the text file.  The end-of-line character itself does not
            // get written.  After this, the current line should be empty
            // again.
            //
            } else if fstr::eq(fstr::substr(&RECORD, C..=C), &NULL) {
                if (POS == 0) {
                    {
                        use f2rust_std::{
                            data::Val,
                            io::{self, Writer},
                        };

                        let mut writer = io::ListDirectedWriter::new(ctx.io_unit(UNIT)?, None)?;
                        IOSTAT = io::capture_iostat(|| {
                            writer.start()?;
                            writer.finish()?;
                            Ok(())
                        })?;
                    }
                } else {
                    {
                        use f2rust_std::{
                            data::Val,
                            io::{self, Writer},
                        };

                        let mut writer =
                            io::FormattedWriter::new(ctx.io_unit(UNIT)?, None, b"(A)")?;
                        IOSTAT = io::capture_iostat(|| {
                            writer.start()?;
                            writer.write_str(fstr::substr(&LINE, 1..=POS))?;
                            writer.finish()?;
                            Ok(())
                        })?;
                    }
                }

                if (IOSTAT != 0) {
                    SETMSG(
                        b"Error writing to the text file named FNM.  Value of IOSTAT is #.",
                        ctx,
                    );
                    ERRINT(b"#", IOSTAT, ctx);
                    SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
                    CHKOUT(b"SPCEC", ctx)?;
                    return Ok(());
                }

                fstr::assign(&mut LINE, b" ");
                POS = 0;

            //
            // If this a normal character, add it to the current line.
            //
            } else {
                POS = (POS + 1);
                fstr::assign(
                    fstr::substr_mut(&mut LINE, POS..=POS),
                    fstr::substr(&RECORD, C..=C),
                );
            }
        }
    }

    CHKOUT(b"SPCEC", ctx)?;
    Ok(())
}
