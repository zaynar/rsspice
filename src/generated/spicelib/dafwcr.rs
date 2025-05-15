//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// DAF, write character record
///
/// Write or rewrite the contents of a character record to
/// a DAF.
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
///  HANDLE     I   Handle of DAF.
///  RECNO      I   Record number of character record.
///  CREC       I   Character record.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle associated with a DAF.
///
///  RECNO    is the record number of a character record within
///           the file. If the record does not already exist, it
///           is created. Otherwise its contents are overwritten.
///
///  CREC     contains the first 1000 characters of the specified
///           record.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the specified file is not open for write access, an error
///      is signaled by a routine in the call tree of this routine.
///
///  2)  If the declared length of CREC is not 1000 characters,
///      the error SPICE(DAFBADRECLEN) is signaled.
///
///  3)  If the specified record cannot (for some reason) be written,
///      the error SPICE(DAFWRITEFAIL) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  Unlike double precision records, character records are
///  not buffered.
/// ```
///
/// # Examples
///
/// ```text
///  In the following example, matching summary and name records are
///  written to a DAF:
///
///     CALL DAFWDR ( HANDLE, NEXT,   DREC )
///     CALL DAFWCR ( HANDLE, NEXT+1, CREC )
///
///  Note that a character record always immediately follows a summary
///  record.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 14-APR-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Moved DAF
///         required reading from $Literature_References to
///         $Required_Reading section.
///
/// -    SPICELIB Version 2.0.0, 27-NOV-2001 (FST)
///
///         Updated this routine to utilize new handle manager
///         interfaces. Replaced the check of the input handle's
///         sign with the appropriate call to DAFSIH.
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
/// -    SPICELIB Version 2.0.0, 27-NOV-2001 (FST)
///
///         The call to DAFHLU has been replaced with a call to
///         ZZDDHHLU, the handle manager interface for retrieving
///         a logical unit. DAFHLU is no longer used, since it
///         locks the unit returned to its HANDLE, tying up resources
///         in the handle manager. A call to DAFSIH was inserted to
///         make certain that HANDLE is present in DAFAH's file table,
///         rather than simply checking the sign of HANDLE.
/// ```
pub fn dafwcr(ctx: &mut SpiceContext, handle: i32, recno: i32, crec: &str) -> crate::Result<()> {
    DAFWCR(handle, recno, crec.as_bytes(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFWCR ( DAF, write character record )
pub fn DAFWCR(HANDLE: i32, RECNO: i32, CREC: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut UNIT: i32 = 0;
    let mut IOSTAT: i32 = 0;

    //
    // SPICELIB functions
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
        CHKIN(b"DAFWCR", ctx)?;
    }

    ZZDDHHLU(HANDLE, b"DAF", false, &mut UNIT, ctx)?;

    //
    // Look out for
    //
    //   -- Writing to a file that is open for read-only.
    //
    //   -- Trying to write a record that doesn't have length 1000.
    //
    //   -- Failed write.
    //
    DAFSIH(HANDLE, b"WRITE", ctx)?;

    if (intrinsics::LEN(CREC) != 1000) {
        SETMSG(
            b"Expected length of character record is 1000. Length of passed record is #",
            ctx,
        );
        ERRINT(b"#", intrinsics::LEN(CREC), ctx);
        SIGERR(b"SPICE(DAFBADCRECLEN)", ctx)?;
    } else {
        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::UnformattedWriter::new(ctx.io_unit(UNIT)?, Some(RECNO))?;
            IOSTAT = io::capture_iostat(|| {
                writer.start()?;
                writer.write_str(CREC)?;
                writer.finish()?;
                Ok(())
            })?;
        }

        if (IOSTAT != 0) {
            SETMSG(b"Character record write failed. Value of IOSTAT was #", ctx);
            ERRINT(b"#", IOSTAT, ctx);
            SIGERR(b"SPICE(DAFWRITEFAIL)", ctx)?;
        }
    }

    CHKOUT(b"DAFWCR", ctx)?;
    Ok(())
}
