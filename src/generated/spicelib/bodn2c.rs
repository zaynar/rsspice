//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Body name to ID code translation
///
/// Translate the name of a body or object to the corresponding SPICE
/// integer ID code.
///
/// # Required Reading
///
/// * [NAIF_IDS](crate::required_reading::naif_ids)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NAME       I   Body name to be translated into a SPICE ID code.
///  CODE       O   SPICE integer ID code for the named body.
///  FOUND      O   .TRUE. if translated, otherwise false.
///  MAXL       P   Maximum length of NAME string.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NAME     is the name of a body or object, such as a planet,
///           satellite, comet, asteroid, barycenter, DSN station,
///           spacecraft, or instrument, that is "known" to the
///           SPICE system, whether through hard-coded
///           registration or run-time registration in the SPICE
///           kernel pool.
///
///           Case and leading and trailing blanks in a name
///           are not significant. However when a name is made
///           up of more than one word, they must be separated by
///           at least one blank. That is, all of the following
///           strings are equivalent names:
///
///               'JUPITER BARYCENTER'
///               'Jupiter Barycenter'
///               'JUPITER BARYCENTER   '
///               'JUPITER    BARYCENTER'
///               '   JUPITER BARYCENTER'
///
///           However, 'JUPITERBARYCENTER' is not equivalent to
///           the names above.
/// ```
///
/// # Detailed Output
///
/// ```text
///  CODE     is the SPICE or user-defined integer ID code for the
///           named body.
///
///  FOUND    is .TRUE. if NAME has a translation. Otherwise, FOUND
///           is .FALSE.
/// ```
///
/// # Parameters
///
/// ```text
///  MAXL     is the maximum allowable length of a body name. The
///           current value of this parameter is 36. See the SPICELIB
///           include file zzbodtrn.inc for details.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If there is any problem with the body name-ID mapping kernel
///      variables present in the kernel pool, an error is signaled by
///      a routine in the call tree of this routine.
///
///  2)  Body name strings are upper-cased, their leading and trailing
///      blanks removed, and embedded blanks are compressed out, after
///      which they get truncated to the maximum body name length MAXL.
///      Therefore, two body names that differ only after that maximum
///      length are considered equal.
/// ```
///
/// # Files
///
/// ```text
///  Body-name mappings may be defined at run time by loading text
///  kernels containing kernel variable assignments of the form
///
///     NAIF_BODY_NAME += ( <name 1>, ... )
///     NAIF_BODY_CODE += ( <code 1>, ... )
///
///  See naif_ids.req for details.
/// ```
///
/// # Particulars
///
/// ```text
///  BODN2C is one of five related subroutines,
///
///     BODS2C      Body string to code
///     BODC2S      Body code to string
///     BODN2C      Body name to code
///     BODC2N      Body code to name
///     BODDEF      Body name/code definition
///
///  BODS2C, BODC2S, BODN2C, and BODC2N perform translations between
///  body names and their corresponding integer ID codes which are
///  used in SPICE files and routines.
///
///  BODS2C is a slightly more general version of BODN2C: support
///  for strings containing ID codes in string format enables a caller
///  to identify a body using a string, even when no name is
///  associated with that body.
///
///  BODC2S is a general version of BODC2N; the routine returns either
///  the name assigned in the body ID to name mapping or a string
///  representation of the CODE value if no mapping exists.
///
///  BODDEF assigns a body name to ID mapping. The mapping has
///  priority in name-to-ID and ID-to-name translations.
///
///  Programmers writing user interface code should consider using the
///  SPICELIB routine BODS2C. BODS2C provides more flexibility in
///  handling input strings, since it accepts both body names and
///  strings representing integer ID codes, for example '399'.
///
///  Refer to naif_ids.req for the list of name/code associations built
///  into SPICE, and for details concerning adding new name/code
///  associations at run time by loading text kernels.
/// ```
///
/// # Examples
///
/// ```text
///  1)  In the following code fragment, BODVCD returns the radii
///      of Jupiter. BODVCD requires the SPICE integer ID code for
///      Jupiter, so we use BODN2C to convert the name to
///      its corresponding integer ID code.
///
///         CALL BODN2C ( 'JUPITER', JUPID,  FOUND )
///
///         CALL BODVCD ( JUPID, 'RADII', 3, N, RADII )
///
///
///  2)  In this example, we assume that only the set of default
///      name/code pairs has been defined.
///
///      Given these names, BODN2C will return the following codes:
///
///         Name                         Code    Found?
///         ------------------------   ------    ------
///         'EARTH'                       399    Yes
///         '  Earth '                    399    Yes
///         'EMB'                           3    Yes
///         'Solar System Barycenter'       0    Yes
///         'SolarSystemBarycenter'         -    No
///         'SSB'                           0    Yes
///         'Voyager 2'                   -32    Yes
///         'U.S.S. Enterprise'             -    No
///         ' '                             -    No
///         'Halley's Comet'                -    No
///
///
///      Given these codes, BODC2N will return the following names:
///
///         Code        Name                        Found?
///         -------     -------------------         ------
///         399         'EARTH'                     Yes
///           0         'SOLAR SYSTEM BARYCENTER'   Yes
///           3         'EARTH BARYCENTER'          Yes
///         -77         'GALILEO ORBITER'           Yes
///          11          -                          No
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  See exception <2>.
/// ```
///
/// # Author and Institution
///
/// ```text
///  C.H. Acton         (JPL)
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  B.V. Semenov       (JPL)
///  F.S. Turner        (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 20-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Updated
///         description of MAXL parameter and added $Exceptions and
///         $Restrictions.
///
/// -    SPICELIB Version 1.0.8, 16-MAY-2009 (EDW)
///
///         Edit to $Particulars section to document the BODC2S routine.
///
/// -    SPICELIB Version 1.0.7, 28-FEB-2008 (BVS)
///
///         Corrected the contents of the $Required_Reading section.
///
/// -    SPICELIB Version 1.0.6, 31-JAN-2008 (NJB)
///
///         References to the routine BODS2C were added to the header.
///
/// -    SPICELIB Version 1.0.5, 24-OCT-2005 (NJB)
///
///         Header update: changed references to BODVAR to references
///         to BODVCD.
///
/// -    SPICELIB Version 1.0.4, 20-JUL-2004 (EDW)
///
///         Removed unneeded assignment of FOUND = .FALSE.
///
/// -    SPICELIB Version 1.0.3, 29-JUL-2003 (NJB) (CHA)
///
///         Various header changes were made to improve clarity. Some
///         minor header corrections were made.
///
/// -    SPICELIB Version 1.0.2, 26-AUG-2002 (FST)
///
///         Added discussion of MAXL to the $Parameters section.
///
/// -    SPICELIB Version 1.0.1, 22-AUG-2001 (EDW)
///
///         Corrected ENDIF to END IF.
///
/// -    SPICELIB Version 1.0.0, 23-JAN-1996 (KRG)
///
///         This was the BODN2C entry point from the original BODTRN
///         subroutine that was in the NAIF toolkit SUPPORT library.
///         When the private subroutine ZZBODTRN was added to SPICELIB,
///         superseding the BODTRN from SUPPORT, the body ID code/name
///         translation interface from the original BODTRN was moved to
///         SPICELIB so that ID codes did not have to be hard coded by
///         users of the Toolkit.
///
///         This subroutine simply calls the private subroutine ZZBODN2C
///         to perform its job.
/// ```
pub fn bodn2c(
    ctx: &mut SpiceContext,
    name: &str,
    code: &mut i32,
    found: &mut bool,
) -> crate::Result<()> {
    BODN2C(name.as_bytes(), code, found, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure BODN2C ( Body name to ID code translation )
pub fn BODN2C(
    NAME: &[u8],
    CODE: &mut i32,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //
    //
    // Standard SPICELIB error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"BODN2C", ctx)?;
    }

    ZZBODN2C(NAME, CODE, FOUND, ctx)?;
    //
    // No need for any error checking, since all we do is check out
    // and return anyway. We leave the error checking to the caller.
    //
    CHKOUT(b"BODN2C", ctx)?;
    Ok(())
}
