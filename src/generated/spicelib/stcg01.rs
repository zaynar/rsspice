//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// STAR catalog type 1, get star data
///
/// Get data for a single star from a SPICE type 1 star catalog.
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
///  INDEX      I   Star index.
///  RA         O   Right ascension in radians.
///  DEC        O   Declination in radians.
///  RASIG      O   Right ascension uncertainty in radians.
///  DECSIG     O   Declination uncertainty in radians.
///  CATNUM     O   Catalog number.
///  SPTYPE     O   Spectral type.
///  VMAG       O   Visual magnitude.
/// ```
///
/// # Detailed Input
///
/// ```text
///  INDEX    is the index of the star in the list of stars
///           that satisfy the selection criteria specified in
///           the last call to STCF01.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RA       is right ascension of the star at the catalog epoch
///           in radians relative to the J2000 inertial frame.
///
///  DEC      is declination of the star at the catalog epoch in
///           radians relative to the J2000 inertial frame.
///
///  RASIG    is the uncertainty in right ascension of the star at
///           the catalog epoch in radians.
///
///  DECSIG   is the uncertainty in declination of the star at
///           the catalog epoch in radians.
///
///  CATNUM   is the star number in the catalog.
///
///  SPTYPE   is the star's spectral type. See catalog description
///           for more information regarding encoding of spectral
///           type values.
///
///  VMAG     is the visual magnitude of the star.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If fetching of any of output values fails, the error
///      SPICE(BADSTARINDEX) is signaled.
///
///  2)  If no star catalog has been loaded, an error is signaled by a
///      routine in the call tree of this routine.
///
///  3)  If a star catalog type 1 was not queried by calling STCF01
///      before calling this routine, an error is signaled by a routine
///      in the call tree of this routine.
/// ```
///
/// # Files
///
/// ```text
///  This routine reads the data from SPICE type 1 star catalog file
///  loaded into the program by a call to STCL01.
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
///  This routine is intended to be a part of the user interface to
///  the SPICE type 1 star catalog. It allows the caller to retrieve
///  data for a single star found by STCF01 using the star's
///  index within the search result array. This subroutine MUST
///  NOT be called before a search by STCF01 was done.
///
///  Other routines in the SPICE type 1 star catalog access
///  family are:
///
///     STCL01  load the catalog file and make its data
///             available for search and retrieval.
///
///     STCF01  search through the catalog for all stars within
///             a specified RA-DEC rectangle.
/// ```
///
/// # Examples
///
/// ```text
///  In the following code fragment, STCG01 is used to retrieve
///  position and characteristics for every star within an RA - DEC
///  rectangle from a particular SPICE type 1 star catalog.
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
///  C     Retrieve data for every star found.
///  C
///        DO I = 1, NSTARS
///
///           CALL STCG01 ( I, RA, DEC, RASIG, DECSIG,
///       .                 CATNUM, SPTYPE, VMAG )
///
///        END DO
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The catalog file STCG01 reads data from MUST be loaded
///      by STCL01 and a search through the catalog MUST be done by
///      STCF01 before STCG01 is called.
///
///  2)  No other EK queries can be made between the call to STCF01
///      and the call to STCG01.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 16-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Updated header to comply with NAIF standard. Corrected argument
///         names "RASIG" and "DECSIG" in $Brief_I/O.
///
///         Updated entry #3 in $Exceptions section.
///
/// -    SPICELIB Version 1.0.0, 15-MAY-1996 (BVS)
/// ```
pub fn stcg01(
    ctx: &mut SpiceContext,
    index: i32,
    ra: &mut f64,
    dec: &mut f64,
    rasig: &mut f64,
    decsig: &mut f64,
    catnum: &mut i32,
    sptype: &mut str,
    vmag: &mut f64,
) -> crate::Result<()> {
    STCG01(
        index,
        ra,
        dec,
        rasig,
        decsig,
        catnum,
        fstr::StrBytes::new(sptype).as_mut(),
        vmag,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure STCG01 ( STAR catalog type 1, get star data )
pub fn STCG01(
    INDEX: i32,
    RA: &mut f64,
    DEC: &mut f64,
    RASIG: &mut f64,
    DECSIG: &mut f64,
    CATNUM: &mut i32,
    SPTYPE: &mut [u8],
    VMAG: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut FOUND: bool = false;
    let mut NULL: bool = false;

    //
    //
    // SPICELIB functions
    //

    //
    // Local variables.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"STCG01", ctx)?;
    }

    //
    // Fetch data from the catalog in the following order
    // as defined QUERY string template in STCF01 routine
    //
    //       RA, DEC, RASIG, DECSIG, CATNUM, SPTYPE, VMAG
    //
    // Check FOUNDs and report error if any of the parameters
    // is not found.
    //
    // Since NULLs are not allowed in any of the star catalog
    // columns, no check for NULLs is performed.
    //
    EKGD(1, INDEX, 1, RA, &mut NULL, &mut FOUND, ctx)?;
    if !FOUND {
        SETMSG(b"RA value for star # not found. ", ctx);
        ERRINT(b"#", INDEX, ctx);
        SIGERR(b"SPICE(BADSTARINDEX)", ctx)?;
        CHKOUT(b"STCG01", ctx)?;
        return Ok(());
    }

    EKGD(2, INDEX, 1, DEC, &mut NULL, &mut FOUND, ctx)?;
    if !FOUND {
        SETMSG(b"DEC value for star # not found. ", ctx);
        ERRINT(b"#", INDEX, ctx);
        SIGERR(b"SPICE(BADSTARINDEX)", ctx)?;
        CHKOUT(b"STCG01", ctx)?;
        return Ok(());
    }

    EKGD(3, INDEX, 1, RASIG, &mut NULL, &mut FOUND, ctx)?;
    if !FOUND {
        SETMSG(b"RASIG value for star # not found. ", ctx);
        ERRINT(b"#", INDEX, ctx);
        SIGERR(b"SPICE(BADSTARINDEX)", ctx)?;
        CHKOUT(b"STCG01", ctx)?;
        return Ok(());
    }

    EKGD(4, INDEX, 1, DECSIG, &mut NULL, &mut FOUND, ctx)?;
    if !FOUND {
        SETMSG(b"DECSIG value for star # not found.", ctx);
        ERRINT(b"#", INDEX, ctx);
        SIGERR(b"SPICE(BADSTARINDEX)", ctx)?;
        CHKOUT(b"STCG01", ctx)?;
        return Ok(());
    }

    EKGI(5, INDEX, 1, CATNUM, &mut NULL, &mut FOUND, ctx)?;
    if !FOUND {
        SETMSG(b"CATNUM value for star # not found.", ctx);
        ERRINT(b"#", INDEX, ctx);
        SIGERR(b"SPICE(BADSTARINDEX)", ctx)?;
        CHKOUT(b"STCG01", ctx)?;
        return Ok(());
    }

    EKGC(6, INDEX, 1, SPTYPE, &mut NULL, &mut FOUND, ctx)?;
    if !FOUND {
        SETMSG(b"SPTYPE value for star # not found.", ctx);
        ERRINT(b"#", INDEX, ctx);
        SIGERR(b"SPICE(BADSTARINDEX)", ctx)?;
        CHKOUT(b"STCG01", ctx)?;
        return Ok(());
    }

    EKGD(7, INDEX, 1, VMAG, &mut NULL, &mut FOUND, ctx)?;
    if !FOUND {
        SETMSG(b"VMAG value for star # not found. ", ctx);
        ERRINT(b"#", INDEX, ctx);
        SIGERR(b"SPICE(BADSTARINDEX)", ctx)?;
        CHKOUT(b"STCG01", ctx)?;
        return Ok(());
    }

    //
    // Convert angles to radians before return.
    //
    *RA = (*RA * RPD(ctx));
    *DEC = (*DEC * RPD(ctx));
    *RASIG = (*RASIG * RPD(ctx));
    *DECSIG = (*DECSIG * RPD(ctx));

    CHKOUT(b"STCG01", ctx)?;
    Ok(())
}
