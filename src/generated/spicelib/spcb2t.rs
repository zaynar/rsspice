//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const BMARK: &[u8] = b"~NAIF/SPC BEGIN COMMENTS~";
const EMARK: &[u8] = b"~NAIF/SPC END COMMENTS~";

/// SPK and CK, binary to text
///
/// Convert the contents of a binary SPK or CK file to text,
/// including comments if present, and write them to a text file
/// opened by the calling program.
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
///  BINARY     I   Name of an existing binary SPK or CK file.
///  UNIT       I   Logical unit connected to a text file.
/// ```
///
/// # Detailed Input
///
/// ```text
///  BINARY   is the name of an existing binary SPK or CK file
///           that may contain comments in its comment area.
///
///  UNIT     is the logical unit connected to a text file that
///           has been opened for write access. Use the routine
///           TXTOPN to open this file. Upon exit, this file will
///           contain the same data and comments as the binary
///           file, but in text format which is more suitable for
///           transfer between heterogeneous computing environments.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If there is a problem opening or reading from the binary file,
///      an error is signaled by a routine in the call tree of this
///      routine.
///
///  2)  If there is a problem writing to the text file,
///      the error SPICE(FILEWRITEFAILED) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See arguments BINARY and UNIT above.
/// ```
///
/// # Particulars
///
/// ```text
///  The SPICELIB SPK and CK reader subroutines read binary files.
///  However, because different computing environments have different
///  binary representations of numbers, you must convert SPK and CK
///  files to text format when porting from one system to another.
///  After converting the file to text, you can transfer it using
///  a transfer protocol program like Kermit or FTP. Then, convert
///  the text file back to binary format.
///
///  The following is a list of the SPICELIB routines that convert
///  SPK and CK files between binary and text format:
///
///     SPCA2B    converts text to binary. It opens the text file,
///               creates a new binary file, and closes both files.
///
///     SPCB2A    converts binary to text. It opens the binary file,
///               creates a new text file, and closes both files.
///
///     SPCT2B    converts text to binary. It creates a new binary
///               file and closes it. The text file is open on
///               entrance and exit.
///
///     SPCB2T    converts binary to text. It opens the binary
///               file and closes it. The text file is open on
///               entrance and exit
///
///  See the SPC required reading for more information
///  about SPC routines and the SPK and CK file formats.
/// ```
///
/// # Examples
///
/// ```text
///  The following code fragment creates a text file containing
///  text format SPK data and comments preceded and followed
///  by a standard label.
///
///  The SPICELIB routine TXTOPN opens a new text file and TXTOPR
///  opens an existing text file for read access. TEXT and
///  BINARY are character strings that contain the names of the
///  text and binary files.
///
///         CALL TXTOPN ( TEXT, UNIT )
///
///         (Write header label to UNIT)
///
///         CALL SPCB2T ( BINARY, UNIT )
///
///         (Write trailing label to UNIT)
///
///         CLOSE ( UNIT )
///
///
///  The following code fragment reconverts the text format
///  SPK data and comments back into binary format.
///
///         CALL TXTOPR ( TEXT, UNIT )
///
///         (Read, or just read past, header label from UNIT)
///
///         CALL SPCT2B ( UNIT, BINARY )
///
///         (Read trailing label from UNIT, if desired )
///
///         CLOSE ( UNIT )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine assumes that the comment area of the binary SPK
///      or CK file contains only text stored by SPCAC. Comments
///      written any other way may not be handled properly.
///
///  2)  UNIT must be obtained via TXTOPN. Use TXTOPN to open new
///      text files for write access and get the logical unit.
///      System dependencies regarding opening text files have
///      been isolated in the routines TXTOPN and TXTOPR.
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
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 05-APR-1991 (JEM)
/// ```
pub fn spcb2t(ctx: &mut SpiceContext, binary: &str, unit: i32) -> crate::Result<()> {
    SPCB2T(binary.as_bytes(), unit, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPCB2T ( SPK and CK, binary to text )
pub fn SPCB2T(BINARY: &[u8], UNIT: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut HANDLE: i32 = 0;
    let mut IOSTAT: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    // IFNLEN is the length of a DAF internal file name.
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
        CHKIN(b"SPCB2T", ctx)?;
    }

    //
    // First, convert the binary data to text and write it to
    // the text file.
    //
    DAFB2T(BINARY, UNIT, ctx)?;

    //
    // Next, write the begin comments marker.
    //
    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::ListDirectedWriter::new(ctx.io_unit(UNIT)?, None)?;
        IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            writer.write_str(BMARK)?;
            writer.finish()?;
            Ok(())
        })?;
    }

    if (IOSTAT != 0) {
        SETMSG(
            b"Error writing the begin comments marker to the text file named FNM.  IOSTAT = #.",
            ctx,
        );
        ERRFNM(b"FNM", UNIT, ctx)?;
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
        CHKOUT(b"SPCB2T", ctx)?;
        return Ok(());
    }

    //
    // Open the DAF for read access, extract the comments from
    // it and write them to the text file, then close the DAF.
    // If the comment area of the binary file is empty, SPCEC
    // writes nothing to the text file, but even so, we still
    // want the markers.
    //
    DAFOPR(BINARY, &mut HANDLE, ctx)?;
    SPCEC(HANDLE, UNIT, ctx)?;
    DAFCLS(HANDLE, ctx)?;

    //
    // Finally, write the end comments marker.
    //
    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::ListDirectedWriter::new(ctx.io_unit(UNIT)?, None)?;
        IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            writer.write_str(EMARK)?;
            writer.finish()?;
            Ok(())
        })?;
    }

    if (IOSTAT != 0) {
        SETMSG(
            b"Error writing the end comments marker to the text file named FNM.  IOSTAT = #.",
            ctx,
        );
        ERRFNM(b"FNM", UNIT, ctx)?;
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
        CHKOUT(b"SPCB2T", ctx)?;
        return Ok(());
    }

    CHKOUT(b"SPCB2T", ctx)?;
    Ok(())
}
