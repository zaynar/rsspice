//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const FMTLEN: i32 = 8;
const IDWLEN: i32 = 8;
const IFNLEN: i32 = 60;
const TAILEN: i32 = 928;

/// DAF write file record
///
/// Write or rewrite the contents of the file record of a DAF.
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
///  HANDLE     I   Handle of an open DAF file.
///  ND         I   Number of double precision components in summaries.
///  NI         I   Number of integer components in summaries.
///  IFNAME     I   Internal filename.
///  FWARD      I   Forward list pointer.
///  BWARD      I   Backward list pointer.
///  FREE       I   Free address pointer.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle associated with a DAF file opened for
///           writing.
///
///  ND,
///  NI       are the numbers of double precision and integer
///           components, respectively, in each array summary
///           in the specified file.
///
///  IFNAME   is the internal file name to be stored in the first
///           (or file) record of the specified file.
///
///  FWARD    is the forward list pointer. This points to the
///           first summary record in the file.
///
///  BWARD    is the backward list pointer. This points to the
///           final summary record in the file.
///
///  FREE     is the free address pointer. This contains the
///           first free address in the file.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the handle passed to this routine is not the handle of an
///      open DAF file, an error is signaled by a routine in the call
///      tree of this routine.
///
///  2)  If the specified DAF file is not open for write access, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  3)  If the file record cannot (for some reason) be written,
///      the error SPICE(DAFWRITEFAIL) is signaled.
///
///  4)  If the attempt to read the file record fails, the error
///      SPICE(DAFREADFAIL) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  The file record of a DAF is the only record that contains
///  any global information about the file. This record is created
///  when the file is created, and is updated only when new arrays
///  are added.
///
///     DO NOT CHANGE THE CONTENTS OF THE FILE RECORD UNLESS
///     YOU ARE ABSOLUTELY SURE YOU KNOW WHAT YOU ARE DOING.
///
///  Like character records, file records are not buffered.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 4.1.0, 06-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
///         Moved DAF required reading from $Literature_References to
///         $Required_Reading section. Fixed typo in $Brief_I/O: ND
///         argument was listed twice. Second should be NI. Removed
///         unnecessary entries in $Revisions section.
///
/// -    SPICELIB Version 4.0.0, 27-NOV-2001 (FST)
///
///         Updated this routine to utilize new handle manager
///         interfaces. Comments were expanded and clarified.
///
/// -    SPICELIB Version 3.0.0, 21-MAR-1999 (FST)
///
///         This routine was modified to accommodate the preservation
///         of the FTP validation and binary file format strings that
///         are now part of the DAF file record.
///
/// -    SPICELIB Version 2.0.0, 05-OCT-1993 (KRG)
///
///         The error SPICE(DAFNOIDWORD) is no longer signaled by this
///         routine. The reason for this is that if DAFSIH returns OK then
///         the handle passed to this routine is indeed a valid DAF file
///         handle, otherwise the error is diagnosed by DAFSIH.
///
///         Added two new exceptions to the $Exceptions section: 1 and 4.
///         The remaining exceptions (2 and 3) were already present. The
///         exceptions that were added are not new, but are being
///         documented for the first time.
///
/// -    SPICELIB Version 1.0.2, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.1, 22-MAR-1990 (HAN)
///
///         Literature references added to the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 4.0.0, 27-NOV-2001 (FST)
///
///         The call to DAFHLU has been replaced with a call to
///         ZZDDHHLU, the handle manager interface for retrieving
///         a logical unit. DAFHLU is no longer used, since it
///         locks the unit returned to its HANDLE, tying up resources
///         in the handle manager.
///
/// -    SPICELIB Version 3.0.0, 21-MAR-1999 (FST)
///
///         In order to preserve the additional information that
///         now resides in the file record, this routine reads
///         the entire record into local buffers, including the
///         TAILEN characters that follow the actual data content.
///         The contents of the local buffers that correspond to
///         information brought in from the call sequence of the
///         routine are ignored when the record is rewritten.
///         However, the ID word, the file format string, and the
///         trailing TAILEN characters that contain the FTP validation
///         string are rewritten along with the input values.
///
///         This routine does not simply replace the FTP validation
///         string with the components from ZZFTPSTR, since that
///         would possibly validate a corrupt file created using a newer
///         Toolkit.
///
/// -    SPICELIB Version 2.0.0, 05-OCT-1993 (KRG)
///
///         The error SPICE(DAFNOIDWORD) is no longer signaled by this
///         routine. The reason for this is that if DAFSIH returns OK then
///         the handle passed to this routine is indeed a valid DAF file
///         handle, otherwise the error is diagnosed by DAFSIH.
///
///         Added a call to DAFSIH to signal an invalid handle and a test
///         of FAILED () after it. This is to make sure that the DAF file
///         is open for writing. If this call succeeds, we know that we
///         have a valid DAF handle, so there is no need to check FAILED
///         after the call to DAFHLU.
///
///         Added code to read the file ID word so that it could be
///         preserved when the file record is written. This supports the ID
///         word format that contains type information.
///
///         Added variable IDWORD to the routine, as well as the parameters
///         IDWLEN and IFNLEN.
///
///         Added two new exceptions to the $Exceptions section: 1 and 4.
///         The remaining exceptions (2 and 3) were already present. The
///         exceptions that were added are not new, but are being
///         documented for the first time.
///
///         Removed code that tested the sign of HANDLE to see if the file
///         was open for write access, HANDLE < 0. This test was no longer
///         necessary, as the call to DASSIH performs this test as well. No
///         sense doing it twice.
/// ```
pub fn dafwfr(
    ctx: &mut SpiceContext,
    handle: i32,
    nd: i32,
    ni: i32,
    ifname: &str,
    fward: i32,
    bward: i32,
    free: i32,
) -> crate::Result<()> {
    DAFWFR(
        handle,
        nd,
        ni,
        ifname.as_bytes(),
        fward,
        bward,
        free,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFWFR ( DAF write file record )
pub fn DAFWFR(
    HANDLE: i32,
    ND: i32,
    NI: i32,
    IFNAME: &[u8],
    FWARD: i32,
    BWARD: i32,
    FREE: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut FORMAT = [b' '; FMTLEN as usize];
    let mut IDWORD = [b' '; IDWLEN as usize];
    let mut IFN = [b' '; IFNLEN as usize];
    let mut LOCIFN = [b' '; IFNLEN as usize];
    let mut TAIL = [b' '; TAILEN as usize];
    let mut IOSTAT: i32 = 0;
    let mut LOCFDR: i32 = 0;
    let mut LOCFFA: i32 = 0;
    let mut LOCLDR: i32 = 0;
    let mut LOCND: i32 = 0;
    let mut LOCNI: i32 = 0;
    let mut UNIT: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //

    //
    // The parameter TAILEN determines the tail length of a DAF file
    // record.  This is the number of bytes (characters) that
    // occupy the portion of the file record that follows the
    // integer holding the first free address.  For environments
    // with a 32 bit word length, 1 byte characters, and DAF
    // record sizes of 1024 bytes, we have:
    //
    //       8 bytes - IDWORD
    //       4 bytes - ND     (32 bit integer)
    //       4 bytes - NI     (32 bit integer)
    //      60 bytes - IFNAME
    //       4 bytes - FWARD  (32 bit integer)
    //       4 bytes - BWARD  (32 bit integer)
    //     + 4 bytes - FREE   (32 bit integer)
    //      ---------
    //      88 bytes - (All file records utilize this space.)
    //
    // So the size of the remaining portion (or tail) of the DAF
    // file record for computing environments as described above
    // would be:
    //
    //    1024 bytes - DAF record size
    //  -    8 bytes - DAF Binary File Format Word
    //  -   88 bytes - (from above)
    //   ------------
    //     928 bytes - DAF file record tail length
    //
    // Note: environments that do not have a 32 bit word length,
    // 1 byte characters, and a DAF record size of 1024 bytes, will
    // require the adjustment of this parameter.
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
        CHKIN(b"DAFWFR", ctx)?;
    }

    //
    // Do some initializations
    //
    fstr::assign(&mut IDWORD, b" ");

    //
    // Check to be sure that HANDLE is attached to a file that is open
    // with write access. If the call fails, check out and return.
    //
    DAFSIH(HANDLE, b"WRITE", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DAFWFR", ctx)?;
        return Ok(());
    }

    //
    // Get the logical unit for the file, as we know we have a valid DAF
    // handle with the correct access method.
    //
    ZZDDHHLU(HANDLE, b"DAF", false, &mut UNIT, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DAFWFR", ctx)?;
        return Ok(());
    }

    //
    // In order to maintain the integrity of the file ID word, the
    // file FORMAT, and the FTP string if present, we need to
    // read the entire file record into the appropriate sized local
    // buffers. The values of the LOCxxx variables are simply
    // ignored, since the caller passes new values in for updates.
    //
    {
        use f2rust_std::{
            data::Val,
            io::{self, Reader},
        };

        let mut reader = io::UnformattedReader::new(ctx.io_unit(UNIT)?, Some(1))?;
        IOSTAT = io::capture_iostat(|| {
            reader.start()?;
            reader.read_str(&mut IDWORD)?;
            LOCND = reader.read_i32()?;
            LOCNI = reader.read_i32()?;
            reader.read_str(&mut LOCIFN)?;
            LOCFDR = reader.read_i32()?;
            LOCLDR = reader.read_i32()?;
            LOCFFA = reader.read_i32()?;
            reader.read_str(&mut FORMAT)?;
            reader.read_str(&mut TAIL)?;
            reader.finish()?;
            Ok(())
        })?;
    }

    if (IOSTAT != 0) {
        SETMSG(
            b"Attempt to read the file record failed for file \'#\'. IOSTAT = #",
            ctx,
        );
        ERRFNM(b"#", UNIT, ctx)?;
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(DAFREADFAIL)", ctx)?;
        CHKOUT(b"DAFWFR", ctx)?;
        return Ok(());
    }

    //
    // Set the value of the internal filename before writing. This is to
    // guarantee that its length is ok.
    //
    fstr::assign(&mut IFN, IFNAME);

    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::UnformattedWriter::new(ctx.io_unit(UNIT)?, Some(1))?;
        IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            writer.write_str(&IDWORD)?;
            writer.write_i32(ND)?;
            writer.write_i32(NI)?;
            writer.write_str(&IFN)?;
            writer.write_i32(FWARD)?;
            writer.write_i32(BWARD)?;
            writer.write_i32(FREE)?;
            writer.write_str(&FORMAT)?;
            writer.write_str(&TAIL)?;
            writer.finish()?;
            Ok(())
        })?;
    }

    if (IOSTAT != 0) {
        SETMSG(b"File record write failed. Value of IOSTAT was #", ctx);
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(DAFWRITEFAIL)", ctx)?;
        CHKOUT(b"DAFWFR", ctx)?;
        return Ok(());
    }

    CHKOUT(b"DAFWFR", ctx)?;
    Ok(())
}
