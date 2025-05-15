//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// DAF, read character record
///
/// Read the contents of a character record from a DAF.
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
///  CREC       O   Character record.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle associated with a DAF.
///
///  RECNO    is the record number of a character record within
///           the file.
/// ```
///
/// # Detailed Output
///
/// ```text
///  CREC     contains the first 1000 characters of the specified
///           record from the specified file.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the declared length of CREC is not 1000 characters,
///      the error SPICE(DAFBADRECLEN) is signaled.
///
///  2)  If the specified record cannot (for some reason) be read,
///      the error SPICE(DAFCRNOTFOUND) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  Unlike double precision records, character records are
///  not buffered. Also, while failing to find a specific double
///  precision record is indicated through the calling sequence,
///  failing to find a character record results in an error.
/// ```
///
/// # Examples
///
/// ```text
///  In the following example, matching summary and name records are
///  read from a DAF:
///
///     CALL DAFGDR ( HANDLE, NEXT,   DREC, FOUND )
///     CALL DAFRCR ( HANDLE, NEXT+1, CREC        )
///
///  Note that a character record always immediately follows a summary
///  record.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine is only used to read records on environments
///      whose characters are a single byte in size. Updates
///      to this routine and routines in its call tree may be
///      required to properly handle other cases.
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
/// -    SPICELIB Version 2.1.0, 17-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Moved DAF
///         required reading from $Literature_References to
///         $Required_Reading section.
///
/// -    SPICELIB Version 2.0.0, 16-NOV-2001 (FST)
///
///         Updated this routine to make proper use of the new
///         handle manager functionality installed underneath
///         DAF.
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
/// -    SPICELIB Version 2.0.0, 16-NOV-2001 (FST)
///
///         This routine now makes use of the handle manager
///         code. A call to DAFSIH was inserted just after
///         the standard SPICE error handling code at the
///         head of the module. This was done to insure that
///         the caller is referring to a legitimately loaded
///         DAF. The penalty for performing this check is
///         a binary search on the number of loaded files,
///         which should be small compared to the actual READ
///         performed below.
///
///         The call to DAFHLU has been replaced with ZZDDHHLU,
///         since calls to DAFHLU locks handles to their logical
///         units.
/// ```
pub fn dafrcr(
    ctx: &mut SpiceContext,
    handle: i32,
    recno: i32,
    crec: &mut str,
) -> crate::Result<()> {
    DAFRCR(
        handle,
        recno,
        fstr::StrBytes::new(crec).as_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFRCR ( DAF, read character record )
pub fn DAFRCR(
    HANDLE: i32,
    RECNO: i32,
    CREC: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
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
        CHKIN(b"DAFRCR", ctx)?;
    }

    //
    // Check to be sure that HANDLE is attached to a file that is open
    // with read access.  If the call fails, check out and return.
    //
    DAFSIH(HANDLE, b"READ", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DAFRCR", ctx)?;
        return Ok(());
    }

    //
    // Now make certain that the string to receive the contents of
    // the character record is the appropriate length.
    //
    if (intrinsics::LEN(CREC) != 1000) {
        SETMSG(
            b"Expected length of character record is 1000. Passed string has length #",
            ctx,
        );
        ERRINT(b"#", intrinsics::LEN(CREC), ctx);
        SIGERR(b"SPICE(DAFBADCRECLEN)", ctx)?;
    } else {
        //
        // Retrieve a logical unit for this handle.  This has the
        // side-effect of locking this UNIT to HANDLE.
        //
        ZZDDHHLU(HANDLE, b"DAF", false, &mut UNIT, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"DAFRCR", ctx)?;
            return Ok(());
        }

        {
            use f2rust_std::{
                data::Val,
                io::{self, Reader},
            };

            let mut reader = io::UnformattedReader::new(ctx.io_unit(UNIT)?, Some(RECNO))?;
            IOSTAT = io::capture_iostat(|| {
                reader.start()?;
                reader.read_str(CREC)?;
                reader.finish()?;
                Ok(())
            })?;
        }

        if (IOSTAT != 0) {
            SETMSG(b"Could not read record #. IOSTAT was #.", ctx);
            ERRINT(b"#", RECNO, ctx);
            ERRINT(b"#", IOSTAT, ctx);
            SIGERR(b"SPICE(DAFCRNOTFOUND)", ctx)?;
        }
    }

    CHKOUT(b"DAFRCR", ctx)?;
    Ok(())
}
