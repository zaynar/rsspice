//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Body ID code to string translation
///
/// Translate a body ID code to either the corresponding name
/// or if no name to ID code mapping exists, the string
/// representation of the body ID value.
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
///  CODE       I   Integer ID code to translate to a string.
///  NAME       O   String corresponding to CODE.
/// ```
///
/// # Detailed Input
///
/// ```text
///  CODE     is the integer code for a body: planet, satellite,
///           barycenter, spacecraft, asteroid, comet, or
///           other ephemeris object.
/// ```
///
/// # Detailed Output
///
/// ```text
///  NAME     is the string name of the body identified by CODE
///           if a mapping between CODE and a body name exists
///           within SPICE.
///
///           If CODE has more than one translation, then the
///           most recently defined NAME corresponding to CODE
///           is returned. NAME will have the exact format (case
///           and blanks) as when the name/code pair was defined.
///
///           If the input value of CODE does not map to a body
///           name, NAME returns the string representation
///           of CODE.
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
///  BODC2S is one of five related subroutines,
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
///  Refer to naif_ids.req for the list of name/code associations built
///  into SPICE, and for details concerning adding new name/code
///  associations at run time by loading text kernels.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Apply the BODC2S call to several IDs representing codes
///     included in the default SPICE ID-name lists and codes not
///     included in the list.
///
///     Example code begins here.
///
///
///           PROGRAM BODC2S_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(32)         NAME
///           INTEGER                CODE (7)
///           INTEGER                I
///
///     C
///     C     Assign an array of body IDs. Not all the listed IDs
///     C     map to a body name.
///     C
///           CODE(1) = 399
///           CODE(2) = 0
///           CODE(3) = 3
///           CODE(4) = -77
///           CODE(5) = 11
///           CODE(6) = -1
///           CODE(7) = 6000001
///
///     C
///     C     Loop over the CODE array, call BODC2S for each
///     C     element of CODE.
///     C
///           WRITE(*,*) 'Code      Name'
///           WRITE(*,*) '-------   -----------------------'
///
///           DO I= 1, 7
///
///              CALL BODC2S( CODE(I), NAME )
///
///              WRITE(*, '(I8,3x,A)' ) CODE(I),  NAME
///
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Code      Name
///      -------   -----------------------
///          399   EARTH
///            0   SOLAR SYSTEM BARYCENTER
///            3   EARTH BARYCENTER
///          -77   GALILEO ORBITER
///           11   11
///           -1   GEOTAIL
///      6000001   6000001
///
///
///     Note that the codes 11 and 6000001 did not map to a name so the
///     call returns as NAME the string expression of the codes.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 07-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Fixed minor
///         typos in header. Added description of MAXL parameter. Added
///         $Exceptions and $Restrictions.
///
/// -    SPICELIB Version 1.0.1, 18-APR-2014 (BVS)
///
///         Minor header edits.
///
/// -    SPICELIB Version 1.0.0, 10-APR-2010 (EDW)
/// ```
pub fn bodc2s(ctx: &mut SpiceContext, code: i32, name: &mut str) -> crate::Result<()> {
    BODC2S(code, fstr::StrBytes::new(name).as_mut(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure BODC2S ( Body ID code to string translation )
pub fn BODC2S(CODE: i32, NAME: &mut [u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut FOUND: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Standard SPICELIB error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"BODC2S", ctx)?;

    //
    // Fortran. No type check available for CODE. Bother.
    //

    //
    // Attempt to translate the input CODE to a name. Use
    // the private routine ZZBODC2N.
    //
    ZZBODC2N(CODE, NAME, &mut FOUND, ctx)?;

    if FOUND {
        //
        // Success. CODE maps to NAME. Return.
        //
        CHKOUT(b"BODC2S", ctx)?;
        return Ok(());
    }

    //
    // If execution reaches this level, the SPICE body ID
    // to name mapping lacks an assignment for CODE. Convert
    // CODE to a string representation of the integer value.
    //
    INTSTR(CODE, NAME, ctx);

    CHKOUT(b"BODC2S", ctx)?;
    Ok(())
}
