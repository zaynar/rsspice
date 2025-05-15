//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// STAR catalog type 1, load catalog file
///
/// Load SPICE type 1 star catalog and return the catalog's
/// table name.
///
/// # Required Reading
///
/// * [EK](crate::required_reading::ek)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  CATFNM     I   Catalog file name.
///  TABNAM     O   Catalog table name.
///  HANDLE     O   Catalog file handle.
/// ```
///
/// # Detailed Input
///
/// ```text
///  CATFNM   is the name of the catalog file.
/// ```
///
/// # Detailed Output
///
/// ```text
///  TABNAM   is the name of the table loaded from the catalog
///           file. This name must be provided as an input argument
///           to STCF01 catalog search routine. Multiple catalogs
///           containing the table TABNAM may be loaded. Sets of
///           columns, column names and attributes must be
///           identical through all these files.
///
///  HANDLE   is the integer handle of the catalog file.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the indicated file cannot be opened, an error is signaled
///      by a routine in the call tree of this routine.
///
///  2)  If the indicated file has the wrong architecture version, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  3)  If an I/O error occurs while reading the indicated file, the
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  4)  If the catalog file is not a type 1 star catalog file, the
///      error SPICE(BADCATALOGFILE) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  This routine loads a SPICE type 1 star catalog file.
///
///  SPICE type 1 star catalog files MUST contain a single data table.
///  It can occupy a single segment or it can spread across multiple
///  segments. This table MUST include the following columns:
///
///     column name                data type          units
///     ----------------------------------------------------
///     RA                   DOUBLE PRECISION        DEGREES
///     DEC                  DOUBLE PRECISION        DEGREES
///     RA_SIGMA             DOUBLE PRECISION        DEGREES
///     DEC_SIGMA            DOUBLE PRECISION        DEGREES
///     CATALOG_NUMBER       INTEGER
///     SPECTRAL_TYPE        CHARACTER*(4)
///     VISUAL_MAGNITUDE     DOUBLE PRECISION
///
///  Nulls are not allowed in any of the columns.
///  Other columns can also be present in the table but their data
///  will NOT be accessible through STCF01 and STCG01 --
///  the interface used to access data in the catalog. Note
///  that the names and attributes of these additional columns
///  must be identical for all segments containing this table.
/// ```
///
/// # Particulars
///
/// ```text
///  This STCL01 routine is intended to be part of the user
///  interface to the SPICE type 1 star catalog. It loads a
///  SPICE type 1 star catalog file and makes its data available
///  for searches and retrieval.
///
///  Other routines in SPICE type 1 star catalog access family are:
///
///     STCF01  search through the catalog for all stars within
///             a specified RA-DEC rectangle.
///
///     STCG01  retrieve position and characteristics for
///             every single star found.
/// ```
///
/// # Examples
///
/// ```text
///  In the following code fragment, STCL01 is used to load
///  a SPICE type 1 star catalog.
///
///  C
///  C     Load catalog file.
///  C
///        CALL STCL01 ( CATFN, TABNAM, HANDLE )
///  C
///  C     Search through the loaded catalog.
///  C
///        CALL STCF01 ( TABNAM, RAMIN,  RAMAX,
///       .              DECMIN, DECMAX, NSTARS )
///  C
///  C     Retrieve data for every star that matched the
///  C     search criteria.
///  C
///        DO I = 1, NSTARS
///
///           CALL STCG01 ( I, RA, DEC, RASIG, DECSIG,
///       .                 CATNUM, SPTYPE, VMAG )
///
///        END DO
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 20-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.1.0, 18-JUN-1999 (WLT)
///
///         Balanced calls to CHKIN/CHKOUT.
///
/// -    SPICELIB Version 1.0.0, 15-MAY-1996 (BVS)
/// ```
pub fn stcl01(
    ctx: &mut SpiceContext,
    catfnm: &str,
    tabnam: &mut str,
    handle: &mut i32,
) -> crate::Result<()> {
    STCL01(
        catfnm.as_bytes(),
        fstr::StrBytes::new(tabnam).as_mut(),
        handle,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure STCL01 ( STAR catalog type 1, load catalog file )
pub fn STCL01(
    CATFNM: &[u8],
    TABNAM: &mut [u8],
    HANDLE: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut ISTYP1: bool = false;
    let mut ERRMSG = [b' '; 256 as usize];

    //
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
        CHKIN(b"STCL01", ctx)?;
    }

    //
    // Check whether the file is really a type 1 star catalog file.
    // If not then signal an error.
    //
    STCC01(CATFNM, TABNAM, &mut ISTYP1, &mut ERRMSG, ctx)?;

    if !ISTYP1 {
        SETMSG(
            &fstr::concat(b"File # is not type 1 star catalog file.", &ERRMSG),
            ctx,
        );
        ERRCH(b"#", CATFNM, ctx);
        SIGERR(b"SPICE(BADCATALOGFILE)", ctx)?;
        CHKOUT(b"STCL01", ctx)?;
        return Ok(());
    }

    //
    // Load the catalog file with the high level EK loader.
    //
    EKLEF(CATFNM, HANDLE, ctx)?;

    CHKOUT(b"STCL01", ctx)?;
    Ok(())
}
