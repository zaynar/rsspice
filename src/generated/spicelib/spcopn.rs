//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const ND: i32 = 2;
const NI: i32 = 6;
const RESV: i32 = 0;

/// SPK or CK, open new file
///
/// Open a new SPK or CK file for subsequent write requests.
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
/// * [SPC](crate::required_reading::spc)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FNAME      I   Name of SPK or CK file to be created.
///  IFNAME     I   Internal file name.
///  HANDLE     O   Handle of new SPK or CK file.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FNAME    is the name of a new SPK or CK file to be created.
///
///  IFNAME   is the internal file name of the file to be created.
///           IFNAME may contain up to 60 characters.
/// ```
///
/// # Detailed Output
///
/// ```text
///  HANDLE   is the file handle assigned to the new file. This
///           should be used to refer to the file in all subsequent
///           calls to DAF and SPC routines.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the specified file cannot be opened without exceeding the
///      maximum number of files, an error is signaled by a routine in
///      the call tree of this routine.
///
///  2)  If the specified file cannot be opened without exceeding the
///      maximum number of DAF files, an error is signaled by a routine
///      in the call tree of this routine.
///
///  3)  If an I/O error occurs in the process of opening the file,
///      the error is signaled by a routine in the call tree of this
///      routine.
///
///  4)  If (for some reason) the initial records in the file cannot be
///      written, an error is signaled by a routine in the call tree of
///      this routine.
///
///  5)  If no logical units are available, an error is signaled by a
///      routine in the call tree of this routine.
///
///  6)  If the file name is blank or otherwise inappropriate, an error
///      is signaled by a routine in the call tree of this routine.
/// ```
///
/// # Files
///
/// ```text
///  See argument FNAME above.
/// ```
///
/// # Particulars
///
/// ```text
///  SPCOPN opens a new SPK or CK file. It is identical to DAFOPN
///  except SPCOPN defines several of the inputs that DAFOPN requires
///  and which specify that the DAF to be opened is an SPK or CK file.
///  Use DAFCLS to close any DAF including SPK and CK files.
///
///  SPCOPN, is not to be confused with the routines that load and
///  unload files to and from a buffer for use by the readers such as
///  SPKLEF (SPK, load ephemeris file) and CKLPF (CK, load pointing
///  file). The loading and unloading routines open and close the files
///  internally, so there is no need to call SPCOPN when loading or
///  unloading SPK or CK files.
/// ```
///
/// # Examples
///
/// ```text
///  In the following code fragment, SPCOPN opens a new file,
///  to which an array is then added. GETDAT is a ficticious
///  non-SPICELIB routine whose function is to get the array data.
///  DAFBNA begins a new array, DAFADA adds data to an array,
///  and DAFENA ends a new array.
///
///            CALL SPCOPN  ( FNAME,  IFNAME, HANDLE )
///            CALL DAFBNA  ( HANDLE, SUM,    NAME   )
///
///            CALL GETDAT  ( N, DATA, FOUND )
///
///            DO WHILE ( FOUND )
///
///               CALL DAFADA ( N, DATA )
///               CALL GETDAT ( N, DATA, FOUND )
///
///            END DO
///
///            CALL DAFENA
///
///            CALL DAFCLS ( HANDLE )
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
/// -    SPICELIB Version 1.1.0, 19-APR-2021 (JDR)
///
///         Added IMPLICIT NONE statement. Changed input argument name
///         "SPC" to "FNAME" for consistency with other routines.
///
///         Edited the header to comply with NAIF standard. Updated the
///         $Exceptions section to cover all errors detected by this
///         routine and remove unnecessary introduction referencing DAF
///         required reading.
///
///         Added DAF required reading to $Required_Reading.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 05-APR-1991 (JEM)
/// ```
pub fn spcopn(
    ctx: &mut SpiceContext,
    fname: &str,
    ifname: &str,
    handle: &mut i32,
) -> crate::Result<()> {
    SPCOPN(
        fname.as_bytes(),
        ifname.as_bytes(),
        handle,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPCOPN ( SPK or CK, open new file )
pub fn SPCOPN(
    FNAME: &[u8],
    IFNAME: &[u8],
    HANDLE: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    // ND, NI      are the Number of Double precision and the Number of
    //             Integer components in an SPK or CK segment descriptor.
    //
    // RESV        is the number of records to reserve when opening the
    //             file.
    //
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"SPCOPN", ctx)?;
    }

    //
    // DAFOPN does all the work.  We just handle the values of
    // ND and NI which are specific to SPK and CK.  We'll not
    // reserve any records.
    //
    DAFOPN(FNAME, ND, NI, IFNAME, RESV, HANDLE, ctx)?;

    CHKOUT(b"SPCOPN", ctx)?;
    Ok(())
}
