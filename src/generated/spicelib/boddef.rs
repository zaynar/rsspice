//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Body name/ID code definition
///
/// Define a body name/ID code pair for later translation via
/// BODN2C or BODC2N.
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
///  NAME       I   Common name of some body.
///  CODE       I   Integer code for that body.
///  MAXL       P   Maximum length of NAME string.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NAME     is an arbitrary name of a body which could be
///           a planet, satellite, barycenter, spacecraft,
///           asteroid, comet, or other ephemeris object.
///
///           The case and positions of blanks in a name are
///           significant. BODC2N returns the same string
///           (case and space) most recently mapped to a code.
///           When NAME consists of more than one word, the
///           words require separation by at least one blank.
///
///           The kernel sub-system stores NAME as described in
///           the BODDEF call, but creates an equivalence class
///           based on NAME for comparisons in BODN2C. This class
///           ignores leading/trailing whitespace, compresses
///           interior whitespace to a single space, and ignores
///           character case.
///
///           The following strings belong to the same equivalence
///           class:
///
///                   'JUPITER BARYCENTER'
///                   'Jupiter Barycenter'
///                   'JUPITER BARYCENTER   '
///                   'JUPITER    BARYCENTER'
///                   '   JUPITER BARYCENTER'
///
///           However, 'JUPITERBARYCENTER' is distinct from
///           the names above.
///
///           When ignoring trailing blanks, NAME must be short
///           enough to fit into the space defined by parameter
///           MAXL.
///
///  CODE     is the integer ID code for assignment to body NAME.
/// ```
///
/// # Parameters
///
/// ```text
///  MAXL     is the maximum allowed length of a body NAME.
///           Names exceeding this length will be truncated
///           on assignment to a code with BODDEF. The value
///           of this parameter may be found in the include
///           file 'zzbodtrn.inc'.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If improper inputs are supplied, or if there is insufficient
///      room to store the requested addition, an error is signaled by
///      a routine in the call tree of this routine.
///
///  2)  If the length of NAME exceeds the maximum allowed length for a
///      body name, the name stored in the kernel pool will be
///      truncated on the right.
///
///  3)  If a name-code definition inserted into this routine seems to
///      have no effect, it is possible that the contents of the
///      definition are masked by the higher precedence kernel pool
///      assignments. See the $Particulars section of this document
///      for more information.
/// ```
///
/// # Particulars
///
/// ```text
///  BODDEF is one of five related subroutines,
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
///  Refer to NAIF_IDs for the list of name/code associations built
///  into SPICE, and for details concerning adding new name/code
///  associations at run time by loading text kernels.
///
///  Modifying the SPICE name-ID mapping set
///  =======================================
///
///  Each body has a unique integer CODE, but may have several
///  names. Thus you may associate more than one name with
///  a particular integer code.
///
///  CODE may already have a name as defined by a previous
///  call to BODDEF or as part of the set of default
///  definitions. That previous definition will remain,
///  and a translation of that name will still give the
///  same CODE. However, future translations of CODE will
///  give the new NAME instead of the previous one. This
///  feature is useful for assigning a more familiar or
///  abbreviated name to a body. For example, in addition
///  to the default name for body 5, 'JUPITER BARYCENTER',
///  you could define the abbreviation 'JB' to mean 5.
///
///  Note: In the case where BODDEF performs a name-to-ID mapping
///  assignment for an unused body name and unused ID value,
///  any subsequent assignment to NAME destroys the previous
///  mapping.
///
///     BODDEF( 'spud', 22)
///
///  then
///
///     BODDEF( 'spud', 23)
///
///  results in the state 'spud' maps to 23, 23 maps to 'spud',
///  and 22 maps to nothing (FOUND in BODC2N returns .FALSE.).
/// ```
///
/// # Examples
///
/// ```text
///  You may associate a new name for a previously defined code:
///
///         CALL BODDEF ( 'JB', 5 )
///
///  You may also define the name and integer code for a new body:
///
///         CALL BODDEF ( 'Asteroid Frank', 20103456 )
///
///  After these calls to BODDEF, BODN2C would return the following
///  translations:
///
///     Name                         Code    Found?
///     ------------------------   ------    ------
///     'JB'                            5    Yes
///     'Jupiter Barycenter'            5    Yes
///     'ASTEROID FRANK'         20103456    Yes
///     'ASTEROIDFRANK'                 -    No
///     'Frank'                         -    No
///
///  and BODC2N will return these translations:
///
///     Code        Name                     Found?
///     -------     -------------------      ------
///            5    'JB'                     Yes
///     20103456    'Asteroid Frank'         Yes
/// ```
///
/// # Author and Institution
///
/// ```text
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
/// -    SPICELIB Version 1.2.0, 13-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added entry #2
///         to $Exceptions section.
///
/// -    SPICELIB Version 1.1.2, 16-MAY-2009 (EDW)
///
///         Edit to $Particulars section to document the BODC2S routine.
///
/// -    SPICELIB Version 1.1.1, 28-FEB-2008 (BVS)
///
///         Corrected the contents of the $Required_Reading section.
///
/// -    SPICELIB Version 1.1.0, 23-JAN-2004 (EDW)
///
///         Rewrote header for clarity with regards to the
///         current capabilities of the kernel subsystem.
///
/// -    SPICELIB Version 1.0.2, 26-AUG-2002 (FST)
///
///         Updated header to describe the parameter MAXL and
///         its effect on this module. The $Exceptions section
///         was updated to include a more general discussion
///         of errors that routines in the call tree of this
///         routine may signal.
///
/// -    SPICELIB Version 1.0.1, 12-AUG-2001 (EDW)
///
///         Updated header with information on new functionality.
///         The code-to-name retrieval routines now return the exact
///         string as defined in the last code/name mapping (case
///         and space).
///
/// -    SPICELIB Version 1.0.0, 23-JAN-1996 (KRG)
///
///         This was the BODDEF entry point from the original BODTRN
///         subroutine that was in the NAIF toolkit SUPPORT library.
///         When the private subroutine ZZBODTRN was added to SPICELIB,
///         superseding the BODTRN from SUPPORT, the body ID code/name
///         translation interface from the original BODTRN was moved to
///         SPICELIB so that ID codes did not have to be hard coded by
///         users of the toolkit.
///
///         This subroutine simply calls the private subroutine ZZBODDEF
///         to perform its job.
/// ```
pub fn boddef(ctx: &mut SpiceContext, name: &str, code: i32) -> crate::Result<()> {
    BODDEF(name.as_bytes(), code, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure BODDEF ( Body name/ID code definition )
pub fn BODDEF(NAME: &[u8], CODE: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Standard SPICELIB error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"BODDEF", ctx)?;
    }

    ZZBODDEF(NAME, CODE, ctx)?;

    //
    // No need for any error checking, since all we do is check out
    // and return anyway. We leave the error checking to the caller.
    //
    CHKOUT(b"BODDEF", ctx)?;
    Ok(())
}
