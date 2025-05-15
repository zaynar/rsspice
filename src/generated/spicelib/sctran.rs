//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const MAXLEN: i32 = 32;

/// SCLK name/ID code translation
///
/// Convert between SCLK name strings and ID codes.
///
/// # Required Reading
///
/// * [SCLK](crate::required_reading::sclk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  ENTRY POINTS
///  --------  ---  --------------------------------------------------
///  CLKNAM    I-O  SCID2N, SCN2ID
///  CLKID     I-O  SCID2N, SCN2ID
///  FOUND      O   SCID2N, SCN2ID
///  MAXLEN     P   All
/// ```
///
/// # Detailed Input
///
/// ```text
///  See the entry points for a discussion of their arguments.
/// ```
///
/// # Detailed Output
///
/// ```text
///  See the entry points for a discussion of their arguments.
/// ```
///
/// # Parameters
///
/// ```text
///  MAXLEN   is the maximum allowed length, in characters, of a
///           string containing the name of a spacecraft clock.
/// ```
///
/// # Exceptions
///
/// ```text
///  See the entry points for a discussion of exceptions specific to
///  those routines.
///
///  1)  This is an umbrella subroutine that contains declarations
///      for its entry points. This routine should never be called
///      directly. If it is, the error SPICE(BOGUSENTRY) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This set of subroutines centralizes the mapping between
///  spacecraft clock names and their corresponding NAIF integer
///  codes. Translation between these names and codes is frequently
///  required by user interface functions.
///
///  The set of supported clocks is identical to the set of spacecraft
///  supported by BODTRN. The mapping may be extended by calling
///  BODDEF.
/// ```
///
/// # Examples
///
/// ```text
///  See the entry points for examples of their usage.
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
/// -    SPICELIB Version 1.2.1, 12-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.2.0, 29-OCT-2001 (NJB)
///
///         Bug fix: modified algorithm to handle case where string
///         "SCLK" appears in SCLK name.
///
/// -    SPICELIB Version 1.1.0, 25-FEB-2000 (NJB)
///
///         Updated to use BODTRN for SCLK name/code mapping.
///
/// -    SPICELIB Version 1.0.0, 17-NOV-1995 (NJB)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.2.0, 29-OCT-2001 (NJB)
///
///         Bug fix: modified algorithm to handle case where string
///         "SCLK" appears in SCLK name.  SCN2ID now uses POSR to locate
///         the substring "SCLK" in the input string.
/// ```
pub fn sctran(ctx: &mut SpiceContext, clknam: &str, clkid: i32, found: bool) -> crate::Result<()> {
    SCTRAN(clknam.as_bytes(), clkid, found, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SCTRAN  ( SCLK name/ID code translation )
pub fn SCTRAN(CLKNAM: &[u8], CLKID: i32, FOUND: bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
        CHKIN(b"SCTRAN", ctx)?;
    }

    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;

    CHKOUT(b"SCTRAN", ctx)?;
    Ok(())
}

/// SCLK name to ID code
///
/// Convert an SCLK name string to a NAIF integer code.
///
/// # Required Reading
///
/// * [SCLK](crate::required_reading::sclk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  CLKNAM     I   String giving spacecraft clock name.
///  CLKID      O   NAIF integer code of spacecraft clock.
///  FOUND      O   Flag indicating whether item was found.
/// ```
///
/// # Detailed Input
///
/// ```text
///  CLKNAM   is a short string identifying the spacecraft
///           clock of interest. The form of the string
///           is:
///
///              <spacecraft name or acronym> SCLK
///
///           for example
///
///              VGR1 SCLK
///              VOYAGER 1 SCLK
///              GLL SCLK
///              GALILEO ORBITER SCLK
///
///           Case and white space (including embedded white
///           space) are not significant.
/// ```
///
/// # Detailed Output
///
/// ```text
///  CLKID    is the NAIF integer code associated with the
///           input clock. CLKID is defined only if the
///           output flag FOUND is returned .TRUE.
///
///  FOUND    is a logical flag indicating whether the input
///           string specified a clock known to this routine.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If the input name is not recognized, FOUND is set to .FALSE.
///      CLKID is not modified.
///
///  2)  If the input name is recognized but does not refer to a
///      spacecraft, no error is signaled. For example, the string
///      'JUPITER BARYCENTER SCLK' maps to the code 5.
/// ```
///
/// # Particulars
///
/// ```text
///  SCN2ID provides a means of mapping human-readable clock names
///  to integer codes used by the SPICELIB SCLK routines to
///  identify spacecraft clocks.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Look up the spacecraft clock code for the Galileo orbiter.
///
///         CALL SCN2ID ( 'GLL SCLK', CLKID, FOUND )
///
///      The outputs will be
///
///         CLKID  =  -77
///         FOUND  =  .TRUE.
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
/// -    SPICELIB Version 1.2.1, 12-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.2.0, 12-AUG-2001 (NJB)
///
///         Bug fix: modified algorithm to handle case where string
///         "SCLK" appears in SCLK name.
///
/// -    SPICELIB Version 1.1.0, 25-FEB-2000 (NJB)
///
///         Updated to use BODTRN for SCLK name/code mapping.
///
/// -    SPICELIB Version 1.0.0, 17-NOV-1995 (NJB)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.2.0, 12-AUG-2001 (NJB)
///
///         Bug fix: modified algorithm to handle case where string
///         "SCLK" appears in SCLK name.  SCN2ID now uses POSR to locate
///         the substring "SCLK" in the input string.
/// ```
pub fn scn2id(
    ctx: &mut SpiceContext,
    clknam: &str,
    clkid: &mut i32,
    found: &mut bool,
) -> crate::Result<()> {
    SCN2ID(clknam.as_bytes(), clkid, found, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SCN2ID  ( SCLK name to ID code )
pub fn SCN2ID(
    CLKNAM: &[u8],
    CLKID: &mut i32,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut TMPNAM = [b' '; MAXLEN as usize];
    let mut LOC: i32 = 0;

    //
    // Convert name to upper case.
    //
    UCASE(CLKNAM, &mut TMPNAM, ctx);

    //
    // Remove the final occurrence of the  string 'SCLK' from
    // the input name.
    //
    LOC = POSR(&TMPNAM, b"SCLK", RTRIM(&TMPNAM));

    if (LOC > 0) {
        fstr::assign(fstr::substr_mut(&mut TMPNAM, LOC..=(LOC + 3)), b" ");
    }

    BODN2C(&TMPNAM, CLKID, FOUND, ctx)?;

    Ok(())
}

/// SCLK ID code to name
///
/// Convert a NAIF integer code for a spacecraft clock to an SCLK name
/// string.
///
/// # Required Reading
///
/// * [SCLK](crate::required_reading::sclk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  CLKID      I   NAIF integer code of spacecraft clock.
///  CLKNAM     O   String giving spacecraft clock name.
///  FOUND      O   Flag indicating whether item was found.
/// ```
///
/// # Detailed Input
///
/// ```text
///  CLKID    is the NAIF integer code of a spacecraft clock of
///           interest.
/// ```
///
/// # Detailed Output
///
/// ```text
///  CLKNAM   is a short, human-readable string identifying
///           the specified spacecraft clock. The returned
///           string has the form
///
///              <spacecraft name or acronym> SCLK
///
///           where the spacecraft name is the same string
///           returned by BODC2N when CLKID is supplied as the
///           input code.
///
///           CLKNAM is defined only if the output flag FOUND is
///           returned .TRUE.
///
///  FOUND    is a logical flag indicating whether the input
///           code specified a clock known to this routine.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If the input code is not recognized, FOUND is set to .FALSE.
///      CLKNAM is not modified.
///
///  2)  If the input code is recognized but does not refer to a
///      spacecraft, no error is signaled. For example, the code
///      5 maps to the string 'JUPITER BARYCENTER SCLK'.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine converts a NAIF spacecraft clock code to a human-
///  readable string. This function is useful for constructing
///  messages.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Look up the spacecraft clock name for code -77.
///
///         CALL SCID2N ( -77, CLKNAM, FOUND )
///
///      The outputs will be
///
///         CLKNAM  =  'GALILEO ORBITER SCLK'
///         FOUND   =  .TRUE.
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
/// -    SPICELIB Version 1.0.1, 12-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 25-FEB-2000 (NJB)
///
///         Updated to use BODTRN for SCLK name/code mapping.
///
/// -    SPICELIB Version 1.0.0, 17-NOV-1995 (NJB)
/// ```
pub fn scid2n(
    ctx: &mut SpiceContext,
    clkid: i32,
    clknam: &mut str,
    found: &mut bool,
) -> crate::Result<()> {
    SCID2N(
        clkid,
        fstr::StrBytes::new(clknam).as_mut(),
        found,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SCID2N  ( SCLK ID code to name )
pub fn SCID2N(
    CLKID: i32,
    CLKNAM: &mut [u8],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    BODC2N(CLKID, CLKNAM, FOUND, ctx)?;

    if !*FOUND {
        return Ok(());
    }

    SUFFIX(b"SCLK", 1, CLKNAM);

    Ok(())
}
