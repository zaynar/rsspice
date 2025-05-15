//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const MXCREC: i32 = 1024;
const MAXPCH: i32 = 126;
const MINPCH: i32 = 32;
const INTEOL: i32 = 0;
const IDWLEN: i32 = 8;
const IFNLEN: i32 = 60;

struct SaveVars {
    EOLMRK: Vec<u8>,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut EOLMRK = vec![b' '; 1 as usize];
        let mut FIRST: bool = false;

        FIRST = true;

        Self { EOLMRK, FIRST }
    }
}

/// DAS add comments
///
/// Add comments from a buffer of character strings to the comment
/// area of a binary DAS file, appending them to any comments which
/// are already present in the file's comment area.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE    I    DAS handle of a file opened with write access.
///  N         I    Number of comments to put into the comment area.
///  BUFFER    I    Buffer of lines to be put into the comment area.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the file handle of a binary DAS file which has been
///           opened with write access.
///
///  N        is the number of comments in BUFFER that are to be
///           added to the comment area of the binary DAS file
///           attached to HANDLE.
///
///  BUFFER   is a buffer containing comments which are to be added
///           to the comment area of the binary DAS file attached
///           to HANDLE.
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
///  3)  If the binary DAS file attached to HANDLE is not open with
///      write access, an error is signaled by a routine in the call
///      tree of this routine.
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
///  Binary DAS files contain a data area which is reserved for storing
///  annotations or descriptive textual information about the data
///  contained in a file. This area is referred to as the ``comment
///  area'' of the file. The comment area of a DAS file is a line
///  oriented medium for storing textual information. The comment
///  area preserves any leading or embedded white space in the line(s)
///  of text which are stored so that the appearance of the
///  information will be unchanged when it is retrieved (extracted) at
///  some other time. Trailing blanks, however, are NOT preserved,
///  due to the way that character strings are represented in
///  standard Fortran 77.
///
///  This routine will take a buffer of text lines and add (append)
///  them to the comment area of a binary DAS file. If there are no
///  comments in the comment area of the file, then space will be
///  allocated and the text lines in BUFFER will then placed into the
///  comment area. The text lines may contain only printable ASCII
///  characters (decimal values 32 - 126).
///
///  There is NO maximum length imposed on the significant portion
///  of a text line that may be placed into the comment area of a
///  DAS file. The maximum length of a line stored in the comment
///  area should be reasonable, however, so that they may be easily
///  extracted. A good value for this would be 255 characters, as
///  this can easily accommodate ``screen width'' lines as well as
///  long lines which may contain some other form of information.
/// ```
///
/// # Examples
///
/// ```text
///  Let
///
///        HANDLE   be the handle for a DAS file which has been opened
///                 with write access.
///
///        N        be the number of lines of text to be added to the
///                 comment area of the binary DAS file attached to
///                 HANDLE.
///
///        BUFFER   is a list of text lines to be added to the comment
///                 area of the binary DAS file attached to HANDLE.
///
///  The call
///
///        CALL DASAC ( HANDLE, N, BUFFER )
///
///  will append the first N line(s) in BUFFER to the comment area
///  of the binary DAS file attached to HANDLE.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine uses constants that are specific to the ASCII
///      character sequence. The results of using this routine with
///      a different character sequence are unpredictable.
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
/// -    SPICELIB Version 1.2.0, 13-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE standard.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section and fixed the first two entry
///         version lines (Beta -> SPICELIB).
///
/// -    SPICELIB Version 1.1.0, 05-FEB-2015 (NJB) (KRG)
///
///         Updated to use ZZDDHHLU.
///
/// -    SPICELIB Version 1.0.1, 12-MAY-1994 (KRG)
///
///         Fixed a typo in the $Particulars section.
///
/// -    SPICELIB Version 1.0.0, 23-NOV-1992 (KRG)
/// ```
pub fn dasac(ctx: &mut SpiceContext, handle: i32, n: i32, buffer: CharArray) -> crate::Result<()> {
    DASAC(handle, n, buffer, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASAC ( DAS add comments )
pub fn DASAC(HANDLE: i32, N: i32, BUFFER: CharArray, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let BUFFER = DummyCharArray::new(BUFFER, None, 1..);
    let mut CRECRD = [b' '; MXCREC as usize];
    let mut IDWORD = [b' '; IDWLEN as usize];
    let mut IFNAME = [b' '; IFNLEN as usize];
    let mut CURPOS: i32 = 0;
    let mut DASLUN: i32 = 0;
    let mut LENGTH: i32 = 0;
    let mut NCHARS: i32 = 0;
    let mut NCOMC: i32 = 0;
    let mut NCOMR: i32 = 0;
    let mut NEWREC: i32 = 0;
    let mut NRESVC: i32 = 0;
    let mut NRESVR: i32 = 0;
    let mut RECNO: i32 = 0;
    let mut RINUSE: i32 = 0;
    let mut SPACE: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    // Length of a DAS character record, in characters.
    //
    //
    // Maximum and minimum decimal values for the printable ASCII
    // characters.
    //

    //
    // Decimal value for the DAS comment area end-of-line (EOL) marker.
    //
    //
    // Length of a DAS file ID word.
    //
    //
    // Length of a DAS file internal filename.
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
        CHKIN(b"DASAC", ctx)?;
    }
    //
    // The lines of text in BUFFER will be ``packed'' into DAS comment
    // records: the significant portion of each comment line from BUFFER
    // will be terminated by the special character EOLMRK to indicate the
    // end of the line. When a comment record is full or all of the
    // comments have been added to the file, the comment record will be
    // written to the comment area of the binary DAS file.
    //
    // If this is the first time that this routine has been called,
    // we need to initialize the character value for the end-of-line
    // marker.
    //
    if save.FIRST {
        save.FIRST = false;
        fstr::assign(&mut save.EOLMRK, &intrinsics::CHAR(INTEOL));
    }
    //
    // Verify that the DAS file attached to HANDLE is opened with write
    // access.
    //
    DASSIH(HANDLE, b"WRITE", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DASAC", ctx)?;
        return Ok(());
    }
    //
    // Convert the DAS file handle to its corresponding Fortran logical
    // unit number for reading and writing comment records.
    //
    ZZDDHHLU(HANDLE, b"DAS", false, &mut DASLUN, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DASAC", ctx)?;
        return Ok(());
    }
    //
    // Check for a nonpositive number of lines in the buffer.
    //
    if (N <= 0) {
        SETMSG(b"The number of comment lines to be added to the binary DAS file # was not positive: #.", ctx);
        ERRFNM(b"#", DASLUN, ctx)?;
        ERRINT(b"#", N, ctx);
        SIGERR(b"SPICE(INVALIDARGUMENT)", ctx)?;
        CHKOUT(b"DASAC", ctx)?;
        return Ok(());
    }
    //
    // Count the number of characters in the buffer ignoring trailing
    // blanks on nonblank lines and blank lines. The count will be
    // modified to include the contribution of blank lines later. This
    // count is used to determine the number of character records to be
    // added to the binary DAS file attached to HANDLE.
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
            // characters are given by MAXPCH and MINPCH, which are
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
                CHKOUT(b"DASAC", ctx)?;
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
    // count.
    //
    NCHARS = (NCHARS + N);
    //
    // Get the current number of comment records and comment characters
    // from the DAS file attached to HANDLE. We will also get back some
    // extra stuff that we do not use.
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
        CHKOUT(b"DASAC", ctx)?;
        return Ok(());
    }
    //
    // Determine the amount of free space in the comment area. If
    // there are some comment records allocated, the space available
    // is the number of comment records allocated times the length of
    // a comment record, minus the number of comment characters already
    // used. Otherwise, the space available is zero.
    //
    if (NCOMR > 0) {
        SPACE = ((NCOMR * MXCREC) - NCOMC);
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
        DASACR(HANDLE, NEWREC, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"DASAC", ctx)?;
            return Ok(());
        }

        //
        // Update the value for the number of comment records to include
        // those that were just added. We need this value when we write
        // the file record at the end of the routine to update the number
        // comment characters, NCOMC.
        //
        NCOMR = (NCOMR + NEWREC);
    }
    //
    // At this point, we know that we have enough space to write all of
    // the comments in BUFFER to the comment area. Either there was
    // enough space already there, or we figured out how many new comment
    // records were needed, and we added them to the file. So, now we
    // begin ``packing'' the comments into DAS character records and
    // writing them to the file.
    //
    // We begin by reading the last comment record if there is one.
    // Otherwise we just initialize the appropriate variables.
    //
    if (NCOMC == 0) {
        //
        // If there are no comments in the comment area, then we need to
        // skip the file record and the reserved records, if any. The
        // first available comment record is the record immediately
        // after the last reserved record, so we set RECNO accordingly.
        // We also initialize the current position in the comment record,
        // and the comment record itself.
        //
        RECNO = ((1 + NRESVR) + 1);
        CURPOS = 1;
        fstr::assign(&mut CRECRD, b" ");
    } else {
        //
        // If there are comments in the comment area, then we need to skip
        // the file record, the reserved records, if any, and any comment
        // records which have been filled. The first comment record
        // with space available is the record immediately following the
        // last completely filled comment record. So calculate the number
        // of comment records in use, and set RECNO appropriately. Then
        // calculate the initial position and read in the comment record.
        //
        RINUSE = (1 + (NCOMC / MXCREC));
        RECNO = ((1 + NRESVR) + RINUSE);
        CURPOS = ((NCOMC - (MXCREC * (RINUSE - 1))) + 1);

        DASIOC(b"READ", DASLUN, RECNO, &mut CRECRD, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"DASAC", ctx)?;
            return Ok(());
        }
    }
    //
    // Begin ``packing'' the comments from the input buffer into the
    // comment records, writing the comment records to the DAS file
    // as they become filled.
    //
    for I in 1..=N {
        //
        // Get the length of the significant portion of a comment line.
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
                DASIOC(b"WRITE", DASLUN, RECNO, &mut CRECRD, ctx)?;

                if FAILED(ctx) {
                    CHKOUT(b"DASAC", ctx)?;
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
            DASIOC(b"WRITE", DASLUN, RECNO, &mut CRECRD, ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"DASAC", ctx)?;
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
    DASIOC(b"WRITE", DASLUN, RECNO, &mut CRECRD, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DASAC", ctx)?;
        return Ok(());
    }

    //
    // And finally, we need to update the number of comment characters
    // in the file record by adding NCHARS, and writing the file record.
    //
    NCOMC = (NCOMC + NCHARS);

    DASWFR(HANDLE, &IDWORD, &IFNAME, NRESVR, NRESVC, NCOMR, NCOMC, ctx)?;
    //
    // Check out and leave DASAC. A test of FAILED should be done by
    // the calling routine to catch an error that may occur during
    // the call to DASWFR.
    //
    CHKOUT(b"DASAC", ctx)?;
    Ok(())
}
