//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Body ID code to name translation
///
/// Translate the SPICE integer code of a body into a common name
/// for that body.
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
///  CODE       I   Integer ID code to be translated into a name.
///  NAME       O   A common name for the body identified by CODE.
///  FOUND      O   .TRUE. if translated, otherwise false.
///  MAXL       P   Maximum length of NAME string.
/// ```
///
/// # Detailed Input
///
/// ```text
///  CODE     is an integer code for a body ---
///           a planet, satellite, barycenter, spacecraft,
///           asteroid, comet, or other ephemeris object.
/// ```
///
/// # Detailed Output
///
/// ```text
///  NAME     is a common name of the body identified by CODE.
///           If CODE has more than one translation, then the
///           most recently defined NAME corresponding to CODE
///           is returned. NAME will have the exact format (case
///           and blanks) as when the name/code pair was defined.
///           If the input value of CODE is not recognized, NAME
///           will remain unchanged from its input value.
///
///  FOUND    is .TRUE. if CODE has a translation. Otherwise, FOUND
///           is .FALSE.
/// ```
///
/// # Parameters
///
/// ```text
///  MAXL     is the maximum allowable length of a body name.
///           This amount of storage space should be declared
///           to receive NAME, otherwise truncation may occur.
///           The current value of this parameter is 36. See
///           the SPICELIB include file zzbodtrn.inc for details.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If there is any problem with the body name-ID mapping kernel
///      variables present in the kernel pool, an error is signaled by
///      a routine in the call tree of this routine.
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
///  BODS2N is one of five related subroutines,
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
/// ```
///
/// # Examples
///
/// ```text
///  1. Suppose you ran the utility program SPACIT to summarize
///      an SPK ephemeris file and the following data was output
///      to the terminal screen.
///
///         ----------------------------------------------------------
///         Segment identifier: JPL archive 21354
///         Body        : -77                         Center     : 399
///         From        : 1990 DEC 08 18:00:00.000
///         To          : 1990 DEC 10 21:10:00.000
///         Reference   : DE-200                      SPK Type    :1
///         ----------------------------------------------------------
///
///     You could write a program to translate the body codes
///     shown in the SPACIT output:
///
///        CALL BODC2N ( -77, BODY,   FOUND )
///        CALL BODC2N ( 399, CENTER, FOUND )
///
///        IF ( FOUND ) THEN
///
///           WRITE ( *,* ) 'BODY:    -77 = ', BODY
///           WRITE ( *,* ) 'CENTER:  399 = ', CENTER
///
///        END IF
///
///     You could also read the body and center codes directly from
///     the SPK files, using the appropriate DAF routines, and then
///     translate them, as above.
///
///
///  2. In this example, we assume that BODDEF has not been called,
///      so only the set of default name/code pairs has
///      been defined.
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
///          -1          -                          No
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
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
///         description of MAXL parameter and added $Exceptions,
///         $Restrictions and $Files.
///
/// -    SPICELIB Version 1.0.4, 16-MAY-2009 (EDW)
///
///         Edit to $Particulars section to document the BODC2S routine.
///
/// -    SPICELIB Version 1.0.3, 28-FEB-2008 (BVS)
///
///         Corrected the contents of the $Required_Reading section.
///
/// -    SPICELIB Version 1.0.2, 26-AUG-2002 (FST)
///
///         Added documentation discussing the parameter MAXL.
///
/// -    SPICELIB Version 1.0.1, 01-DEC-1998 (WLT)
///
///         Added documentation that describes the output NAME if CODE
///         is not a recognized body ID.
///
/// -    SPICELIB Version 1.0.0, 23-JAN-1996 (KRG)
///
///         This was the BODC2N entry point from the original BODTRN
///         subroutine that was in the NAIF toolkit SUPPORT library.
///         When the private subroutine ZZBODTRN was added to SPICELIB,
///         superseding the BODTRN from SUPPORT, the body ID code/name
///         translation interface from the original BODTRN was moved to
///         SPICELIB so that ID codes did not have to be hard coded by
///         users of the toolkit.
///
///         This subroutine simply calls the private subroutine ZZBODC2N
///         to perform its job.
/// ```
pub fn bodc2n(
    ctx: &mut SpiceContext,
    code: i32,
    name: &mut str,
    found: &mut bool,
) -> crate::Result<()> {
    BODC2N(
        code,
        fstr::StrBytes::new(name).as_mut(),
        found,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure BODC2N ( Body ID code to name translation )
pub fn BODC2N(
    CODE: i32,
    NAME: &mut [u8],
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
        CHKIN(b"BODC2N", ctx)?;
    }

    ZZBODC2N(CODE, NAME, FOUND, ctx)?;
    //
    // No need for any error checking, since all we do is check out
    // and return anyway. We leave the error checking to the caller.
    //
    CHKOUT(b"BODC2N", ctx)?;
    Ok(())
}
