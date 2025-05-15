//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const FMTLEN: i32 = 8;
const IDWLEN: i32 = 8;
const IFNLEN: i32 = 60;
const TAILEN: i32 = 932;

/// DAS write file record
///
/// Update the contents of the file record of a specified DAS file.
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
///  HANDLE     I   DAS file handle.
///  IDWORD     I   ID word.
///  IFNAME     I   DAS internal file name.
///  NRESVR     I   Number of reserved records in file.
///  NRESVC     I   Number of characters in use in reserved rec. area.
///  NCOMR      I   Number of comment records in file.
///  NCOMC      I   Number of characters in use in comment area.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is a file handle for a DAS file open for writing.
///
///  IDWORD   is the `ID word' contained in the first eight
///           characters of the file record.
///
///  IFNAME   is the internal file name of the DAS file. The
///           maximum length of the internal file name is 60
///           characters.
///
///  NRESVR   is the number of reserved records in the DAS file
///           specified by HANDLE.
///
///  NRESVC   is the number of characters in use in the reserved
///           record area of the DAS file specified by HANDLE.
///
///  NCOMR    is the number of comment records in the DAS file
///           specified by HANDLE.
///
///  NCOMC    is the number of characters in use in the comment area
///           of the DAS file specified by HANDLE.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the handle passed to this routine is not the handle of an
///      open DAS file, an error is signaled by a routine in the call
///      tree of this routine.
///
///  2)  If the specified DAS file is not open for write access, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  3)  If the attempt to read the file record fails, the error
///      SPICE(DASREADFAIL) is signaled.
///
///  4)  If the file write attempted by this routine fails, the error
///      SPICE(DASFILEWRITEFAILED) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See the description of HANDLE under $Detailed_Input.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine provides a convenient way of updating the internal
///  file name of a DAS file.
///
///  The `ID word' contained in the file record is a string of eight
///  characters that identifies the file as a DAS file and optionally
///  indicates a specific file format, for example, `EK'.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Update the internal file name of an existing DAS file.
///
///         C
///         C     Open the file for writing.
///         C
///               CALL DASOPW ( FNAME, HANDLE  )
///
///         C
///         C     Retrieve the ID word and current reserved record
///         C     and comment area record and character counts.
///         C
///               CALL DASRFR ( HANDLE,
///              .              IDWORD,
///              .              IFNAME,
///              .              NRESVR,
///              .              NRESVC,
///              .              NCOMR,
///              .              NCOMC  )
///
///         C
///         C     Set the internal file name and update the file
///         C     with it.
///         C
///               IFNAME = 'New internal file name'
///
///               CALL DASWFR ( HANDLE,
///              .              IDWORD,
///              .              IFNAME,
///              .              NRESVR,
///              .              NRESVC,
///              .              NCOMR,
///              .              NCOMC  )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The DAS file must have a binary file format native to the host
///      system.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.1.1, 02-JUN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 3.1.0, 05-FEB-2015 (NJB)
///
///         Updated to support integration with the handle
///         manager subsystem.
///
/// -    SPICELIB Version 3.0.0, 11-DEC-2001 (FST)
///
///         This routine was modified to accommodate the preservation
///         of the FTP validation and binary file format strings that
///         are not part of the DAS file record.
///
/// -    SPICELIB Version 2.0.0, 27-OCT-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if these open routines ever
///         change.
///
///         Added a check of FAILED after the call to DASHLU which will
///         check out and return if DASHLU fails. This is so that when in
///         return mode of the error handling the READ following the call
///         to DASHLU will not be executed.
///
///         Reworded some of the descriptions contained in the
///         $Detailed_Output section of the header so that they were more
///         clear.
///
/// -    SPICELIB Version 1.0.0, 24-NOV-1992 (NJB) (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 3.0.0, 11-DEC-2001 (FST)
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
///         The string arguments passed into this routine are now
///         copied to local buffers of the appropriate length.
///
/// -    SPICELIB Version 2.0.0, 27-OCT-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if these open routines ever
///         change.
///
///         Added a check of FAILED after the call to DASHLU which will
///         check out and return if DASHLU fails. This is so that when in
///         return mode of the error handling the READ following the call
///         to DASHLU will not be executed.
///
///         Reworded some of the descriptions contained in the
///         $Detailed_Output section of the header so that they were more
///         clear.
///
/// -    SPICELIB Version 1.0.0, 24-NOV-1992 (NJB) (WLT)
/// ```
pub fn daswfr(
    ctx: &mut SpiceContext,
    handle: i32,
    idword: &str,
    ifname: &str,
    nresvr: i32,
    nresvc: i32,
    ncomr: i32,
    ncomc: i32,
) -> crate::Result<()> {
    DASWFR(
        handle,
        idword.as_bytes(),
        ifname.as_bytes(),
        nresvr,
        nresvc,
        ncomr,
        ncomc,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASWFR ( DAS write file record )
pub fn DASWFR(
    HANDLE: i32,
    IDWORD: &[u8],
    IFNAME: &[u8],
    NRESVR: i32,
    NRESVC: i32,
    NCOMR: i32,
    NCOMC: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut FORMAT = [b' '; FMTLEN as usize];
    let mut LOCIDW = [b' '; IDWLEN as usize];
    let mut LOCIFN = [b' '; IFNLEN as usize];
    let mut IFN = [b' '; IFNLEN as usize];
    let mut TAIL = [b' '; TAILEN as usize];
    let mut FREE: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut LASTLA = StackArray::<i32, 3>::new(1..=3);
    let mut LASTRC = StackArray::<i32, 3>::new(1..=3);
    let mut LASTWD = StackArray::<i32, 3>::new(1..=3);
    let mut LOCNCC: i32 = 0;
    let mut LOCNCR: i32 = 0;
    let mut LOCNVC: i32 = 0;
    let mut LOCNVR: i32 = 0;
    let mut OLDRRC: i32 = 0;
    let mut OLDRCH: i32 = 0;
    let mut OLDCRC: i32 = 0;
    let mut OLDCCH: i32 = 0;
    let mut UNIT: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //

    //
    // The parameter TAILEN determines the tail length of a DAS file
    // record.  This is the number of bytes (characters) that
    // occupy the portion of the file record that follows the
    // integer holding the first free address.  For environments
    // with a 32 bit word length, 1 byte characters, and DAS
    // record sizes of 1024 bytes, we have:
    //
    //       8 bytes - IDWORD
    //      60 bytes - IFNAME
    //       4 bytes - NRESVR (32 bit integer)
    //       4 bytes - NRESVC (32 bit integer)
    //       4 bytes - NCOMR  (32 bit integer)
    //     + 4 bytes - NCOMC  (32 bit integer)
    //      ---------
    //      84 bytes - (All file records utilize this space.)
    //
    // So the size of the remaining portion (or tail) of the DAS
    // file record for computing environments as described above
    // would be:
    //
    //    1024 bytes - DAS record size
    //  -    8 bytes - DAS Binary File Format Word
    //  -   84 bytes - (from above)
    //   ------------
    //     932 bytes - DAS file record tail length
    //
    // Note: environments that do not have a 32 bit word length,
    // 1 byte characters, and a DAS record size of 1024 bytes, will
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
    }

    CHKIN(b"DASWFR", ctx)?;

    //
    // Check to be sure that HANDLE is attached to a file that is open
    // with write access.  If the call fails, check out and return.
    //
    DASSIH(HANDLE, b"WRITE", ctx)?;

    //
    // Get the logical unit for this DAS file.
    //
    ZZDDHHLU(HANDLE, b"DAS", false, &mut UNIT, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DASWFR", ctx)?;
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
            reader.read_str(&mut LOCIDW)?;
            reader.read_str(&mut LOCIFN)?;
            LOCNVR = reader.read_i32()?;
            LOCNVC = reader.read_i32()?;
            LOCNCR = reader.read_i32()?;
            LOCNCC = reader.read_i32()?;
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
        SIGERR(b"SPICE(DASREADFAIL)", ctx)?;
        CHKOUT(b"DASWFR", ctx)?;
        return Ok(());
    }

    //
    // Set the value of the internal file name and IDWORD before
    // writing.  This is to guarantee that their lengths are ok.
    //
    fstr::assign(&mut IFN, IFNAME);
    fstr::assign(&mut LOCIDW, IDWORD);

    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::UnformattedWriter::new(ctx.io_unit(UNIT)?, Some(1))?;
        IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            writer.write_str(&LOCIDW)?;
            writer.write_str(&IFN)?;
            writer.write_i32(NRESVR)?;
            writer.write_i32(NRESVC)?;
            writer.write_i32(NCOMR)?;
            writer.write_i32(NCOMC)?;
            writer.write_str(&FORMAT)?;
            writer.write_str(&TAIL)?;
            writer.finish()?;
            Ok(())
        })?;
    }

    if (IOSTAT != 0) {
        SETMSG(
            b"Could not write file record.  File was #.  IOSTAT was #.",
            ctx,
        );
        ERRFNM(b"#", UNIT, ctx)?;
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(DASFILEWRITEFAILED)", ctx)?;
        CHKOUT(b"DASWFR", ctx)?;
        return Ok(());
    }

    //
    // Update the file summary, in case the values of the reserved
    // record or comment area counts have changed.
    //
    DASHFS(
        HANDLE,
        &mut OLDRRC,
        &mut OLDRCH,
        &mut OLDCRC,
        &mut OLDCCH,
        &mut FREE,
        LASTLA.as_slice_mut(),
        LASTRC.as_slice_mut(),
        LASTWD.as_slice_mut(),
        ctx,
    )?;

    DASUFS(
        HANDLE,
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        ctx,
    )?;

    CHKOUT(b"DASWFR", ctx)?;
    Ok(())
}
