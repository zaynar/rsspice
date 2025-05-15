//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const VERIDX: i32 = 1;
const LLBIDX: i32 = (VERIDX + 1);
const LLEIDX: i32 = (LLBIDX + 1);
const NULPTR: i32 = -1;
const BWDIDX: i32 = 1;
const FWDIDX: i32 = (BWDIDX + 1);
const IBSIDX: i32 = (FWDIDX + 1);
const ISZIDX: i32 = (IBSIDX + 1);
const DBSIDX: i32 = (ISZIDX + 1);
const DSZIDX: i32 = (DBSIDX + 1);
const CBSIDX: i32 = (DSZIDX + 1);
const CSZIDX: i32 = (CBSIDX + 1);
const DLADSZ: i32 = CSZIDX;
const FMTVER: i32 = 1000000;
const NCHREC: i32 = 1024;
const METHLN: i32 = 10;

/// DSK, close file
///
/// Close a DSK file.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
/// * [DSK](crate::required_reading::dsk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle assigned to the opened DSK file.
///  OPTMIZ     I   Flag indicating whether to segregate the DSK.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the DAS file handle associated with the file.
///           The file may be open for read or write access.
///
///  OPTMIZ   is a logical flag indicating whether the DSK
///           should be segregated before it is closed. This
///           option applies only to files open for write
///           access. The value of OPTMIZ has no effect for
///           files opened for read access.
///
///           See the DAS Required Reading das.req for a
///           discussion of segregation of DAS files.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None. This routine operates by side effects.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If an error occurs when the file is closed, the error is
///      signaled by a routine in the call tree of this routine.
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
///  This routine provides a DSK-level interface for closing DSK files.
///
///  In cases where DSKs opened for write access are to be closed
///  without segregation, this interface is slightly simpler than that
///  available at the DAS level.
/// ```
///
/// # Examples
///
/// ```text
///  1) Close a new DSK file using DAS segregation. HANDLE
///     is the DAS file handle of the DSK.
///
///     This is the normal choice for DSK creation.
///
///        CALL DSKCLS ( HANDLE, .TRUE. )
///
///  2) Close a new DSK file without using DAS segregation. The
///     close operation will be fast, but reading the file will be
///     less efficient than if the file had been segregated.
///
///        CALL DSKCLS ( HANDLE, .TRUE. )
///
///  3) Close an existing DSK file that had been opened
///     for read access. In this case OPTMIZ is ignored:
///
///        CALL DSKCLS ( HANDLE, .FALSE. )
///
///     or
///
///        CALL DSKCLS ( HANDLE, .TRUE. )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine should not be called by user applications
///      that have loaded a DSK file via FURNSH. Such applications
///      should call the KEEPER entry points UNLOAD or KCLEAR instead.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 12-OCT-2021 (JDR) (NJB)
///
///         Bug fix: now calls FAILED after call to DASHAM.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 08-FEB-2017 (NJB)
///
///
///         09-OCT-2009 (NJB)
///
///            Updated header.
///
///         20-OCT-2006 (NJB)
///
///            Original DSKLIB version.
/// ```
pub fn dskcls(ctx: &mut SpiceContext, handle: i32, optmiz: bool) -> crate::Result<()> {
    DSKCLS(handle, optmiz, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DSKCLS ( DSK, close file )
pub fn DSKCLS(HANDLE: i32, OPTMIZ: bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut METHOD = [b' '; METHLN as usize];

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"DSKCLS", ctx)?;

    if OPTMIZ {
        //
        // Segregate the file to enable fast read access.  This is
        // the "normal" way to close a DSK.  Segregating a large file
        // can be slow, however.
        //
        DASCLS(HANDLE, ctx)?;
    } else {
        //
        // Close the file without first segregating it; this allows
        // the caller to close the file quickly, but results in a
        // file that will be read more slowly.
        //
        // Any buffered data to be written must be explicitly flushed
        // to the file, if the file is open for write access.
        //
        DASHAM(HANDLE, &mut METHOD, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"DSKCLS", ctx)?;
            return Ok(());
        }

        if fstr::eq(&METHOD, b"WRITE ") {
            //
            // Write out any buffered records belonging to the
            // indicated file.
            //
            DASWBR(HANDLE, ctx)?;
        }

        //
        // Close the file without segregating records.
        //
        DASLLC(HANDLE, ctx)?;
    }

    CHKOUT(b"DSKCLS", ctx)?;
    Ok(())
}
