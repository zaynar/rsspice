//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const IFNLEN: i32 = 60;

/// SPK and CK, delete comments
///
/// Empty the comment area of a binary SPK or CK file.
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
///           will have an empty comment area: all previous comments
///           are deleted. Note, however, that the size of the file
///           does not change.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the file does not contain any comments in its comment area
///      on input, it will be unchanged by this routine.
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
///  1)  Suppose we have a binary SPK file called A.BSP. The following
///      code fragment deletes any comments that may have been stored
///      in the comment area of the file.
///
///              CALL DAFOPW ( 'A.BSP', HANDLE )
///
///              CALL SPCDC  ( HANDLE )
///
///  2)  Suppose B.BSP is a binary SPK file with comments in its
///      comment area. The routine TXTOPN opens a new text file.
///
///        C
///        C     Open the binary SPK file with write access and
///        C     get its handle.
///        C
///              CALL DAFOPW ( 'B.BSP', HANDLE )
///
///        C
///        C     Open a new text file and write the comments
///        C     from the SPK file to it.
///        C
///              CALL TXTOPN ( 'COMMENTS.TXT',   UNIT1 )
///              CALL SPCEC  ( HANDLE,           UNIT1 )
///
///        C
///        C     Delete the comments in the SPK file.
///        C
///              CALL SPCDC  ( HANDLE )
///
///        C
///        C     Open another new text file and try to write
///        C     comments from the SPK file to it.
///        C
///              CALL TXTOPN ( 'NOCOMMENTS.TXT', UNIT2 )
///              CALL SPCEC  ( HANDLE,           UNIT2 )
///
///      After executing this code fragment, COMMENTS.TXT would
///      contain the comments from the SPK file.  NOCOMMENTS.TXT
///      would be empty because of the call to SPCDC.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  J.E. McLean        (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 03-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
///         Moved the contents of the $Files section to the description of
///         HANDLE in $Detailed_Input section, and referred to it from
///         $Files.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 05-APR-1991 (JEM)
/// ```
pub fn spcdc(ctx: &mut SpiceContext, handle: i32) -> crate::Result<()> {
    SPCDC(handle, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPCDC ( SPK and CK, delete comments )
pub fn SPCDC(HANDLE: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut IFNAME = [b' '; IFNLEN as usize];
    let mut BWARD: i32 = 0;
    let mut FREE: i32 = 0;
    let mut FWARD: i32 = 0;
    let mut ND: i32 = 0;
    let mut NI: i32 = 0;
    let mut NRR: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    // IFNLEN      is the length of a DAF internal file name.
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
        CHKIN(b"SPCDC", ctx)?;
    }

    //
    // The comment area IS the reserved records.  To empty the comment
    // area we just remove the reserved records.
    //
    // Read the file record to find out how many reserved records are
    // in the DAF.  The reserved records are stored between the first
    // record (the file record) and the first summary record.  FWARD
    // is the record number of that first summary record, and NRR is
    // the number of reserved records in the file.
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
    // Once we know how many there are, we can remove them.
    //
    DAFRRR(HANDLE, NRR, ctx)?;

    CHKOUT(b"SPCDC", ctx)?;
    Ok(())
}
