//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Surface and body ID codes to surface string
///
/// Translate a surface ID code, together with a body ID code, to the
/// corresponding surface name. If no such name exists, return a
/// string representation of the surface ID code.
///
/// # Required Reading
///
/// * [DSK](crate::required_reading::dsk)
/// * [NAIF_IDS](crate::required_reading::naif_ids)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  CODE       I   Integer surface ID code to translate to a string.
///  BODYID     I   ID code of body associated with surface.
///  SRFSTR     O   String corresponding to surface ID code.
///  ISNAME     O   Logical flag indicating output is a surface name.
///  SFNMLN     P   Maximum length of surface name.
/// ```
///
/// # Detailed Input
///
/// ```text
///  CODE     is an integer code for a surface associated with a
///           body.
///
///  BODYID   is an integer code for the body associated with the
///           surface designated by CODE. The combination of CODE
///           and BODYID is to be mapped to a surface name.
/// ```
///
/// # Detailed Output
///
/// ```text
///  SRFSTR   is the name of the surface identified by CODE, for the
///           body designated by BODYID, if an association exists
///           between this pair of ID codes and a surface name.
///
///           If CODE has more than one translation, then the most
///           recently defined surface name corresponding to CODE is
///           returned. SRFSTR will have the exact format (case and
///           embedded blanks) used in the definition of the
///           name/code association.
///
///           If the input pair of codes does not map to a surface
///           name, SRFSTR is set to the string representation of
///           CODE.
///
///           SRFSTR should be declared with length SFNMLN (see the
///           $Parameters section below).
///
///
///  ISNAME   is a logical flag that is .TRUE. if a surface name
///           corresponding to the input ID codes was found and
///           .FALSE. otherwise. When ISNAME is .FALSE., the output
///           string SRFSTR contains a string representing the
///           integer CODE.
/// ```
///
/// # Parameters
///
/// ```text
///  SFNMLN   is the maximum length of a surface name. This
///           parameter is declared in the SPICELIB include file
///
///              srftrn.inc
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input surface ID code cannot be mapped to a name, the
///      output SRFSTR is set to a string representation of the code.
///      The input body ID is ignored. The output ISNAME is set to
///      .FALSE.
///
///      This case is not treated as an error.
/// ```
///
/// # Files
///
/// ```text
///  Surface name-to-ID mappings may be defined at run time by loading
///  text kernels containing kernel variable assignments of the form
///
///     NAIF_SURFACE_NAME += ( <surface name 1>, ... )
///     NAIF_SURFACE_CODE += ( <surface code 1>, ... )
///     NAIF_SURFACE_BODY += ( <body code 1>,    ... )
///
///  Above, the Ith elements of the lists on the assignments' right
///  hand sides together define the Ith surface name/ID mapping.
///
///  The same effect can be achieved using assignments formatted as
///  follows:
///
///     NAIF_SURFACE_NAME += <surface name 1>
///     NAIF_SURFACE_CODE += <surface code 1>
///     NAIF_SURFACE_BODY += <body code 1>
///
///     NAIF_SURFACE_NAME += <surface name 2>
///     NAIF_SURFACE_CODE += <surface code 2>
///     NAIF_SURFACE_BODY += <body code 2>
///
///        ...
///
///  Note the use of the
///
///     +=
///
///  operator; this operator appends to rather than overwrites the
///  kernel variable named on the left hand side of the assignment.
/// ```
///
/// # Particulars
///
/// ```text
///  Surfaces are always associated with bodies (which usually are
///  ephemeris objects). For any given body, a mapping between surface
///  names and surface ID codes can be established.
///
///  Bodies serve to disambiguate surface names and ID codes: the set
///  of surface names and surface ID codes for a given body can be
///  thought of as belonging to a name space. A given surface ID code
///  or surface name may be used for surfaces of multiple bodies,
///  without conflict.
///
///  Associations between surface names and ID codes are always made
///  via kernel pool assignments; there are no built-in associations.
///
///  SRFC2S is one of four related subroutines:
///
///     SRFS2C      Surface string and body string to surface ID code
///     SRFSCC      Surface string and body ID code to surface ID code
///     SRFC2S      Surface ID code and body ID code to surface string
///     SRFCSS      Surface ID code and body string to surface string
///
///  SRFS2C, SRFC2S, SRFSCC, and SRFCSS perform translations between
///  surface strings and their corresponding integer ID codes.
///
///  Refer to naif_ids.req for details concerning adding new surface
///  name/code associations at run time by loading text kernels.
/// ```
///
/// # Examples
///
/// ```text
///  The formatting of the results shown for this example may differ
///  across platforms.
///
///  1) Supposed a text kernel has been loaded that contains
///     the following assignments:
///
///        NAIF_SURFACE_NAME += ( 'MGS MOLA  64 pixel/deg',
///                               'MGS MOLA 128 pixel/deg',
///                               'PHOBOS GASKELL Q512'     )
///        NAIF_SURFACE_CODE += (   1,   2,    1 )
///        NAIF_SURFACE_BODY += ( 499, 499,  401 )
///
///     Translate each surface and body ID code pair to the
///     associated surface name. Also perform a translation
///     for a surface ID having no matching name.
///
///     Use the meta-kernel shown below to define the required SPICE
///     kernel variables.
///
///
///        KPL/MK
///
///        File: srfc2s_ex1.tm
///
///        This meta-kernel is intended to support operation of SPICE
///        example programs. The file contents shown here should not be
///        assumed to contain adequate or correct versions of data
///        required by SPICE-based user applications.
///
///
///        \begindata
///
///        NAIF_SURFACE_NAME += ( 'MGS MOLA  64 pixel/deg',
///                               'MGS MOLA 128 pixel/deg',
///                               'PHOBOS GASKELL Q512'     )
///        NAIF_SURFACE_CODE += (   1,   2,    1 )
///        NAIF_SURFACE_BODY += ( 499, 499,  401 )
///
///        \begintext
///
///
///     Example code begins here.
///
///
///           PROGRAM SRFC2S_EX1
///           IMPLICIT NONE
///
///           INCLUDE 'srftrn.inc'
///
///           INTEGER               FILSIZ
///           PARAMETER           ( FILSIZ = 255 )
///
///           INTEGER               NCASE
///           PARAMETER           ( NCASE  = 5 )
///
///           CHARACTER*(FILSIZ)    META
///           CHARACTER*(SFNMLN)    SRFNAM
///
///           INTEGER               BODYID ( NCASE )
///           INTEGER               I
///           INTEGER               SURFID ( NCASE )
///
///           LOGICAL               ISNAME
///
///           DATA  ( SURFID(I), BODYID(I), I = 1, NCASE ) /
///          .
///          .        1,         499,
///          .        1,         401,
///          .        2,         499,
///          .        3,         499,
///          .        1,          -1                      /
///
///
///           META = 'srfc2s_ex1.tm'
///
///           CALL FURNSH ( META )
///
///           WRITE (*,*) ' '
///
///           DO I = 1, NCASE
///
///              CALL SRFC2S ( SURFID(I), BODYID(I),
///          .                 SRFNAM,    ISNAME    )
///
///              WRITE (*,*) 'surface ID     = ', SURFID(I)
///              WRITE (*,*) 'body ID        = ', BODYID(I)
///              WRITE (*,*) 'name found     = ', ISNAME
///              WRITE (*,*) 'surface string = ', SRFNAM
///              WRITE (*,*) ' '
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
///      surface ID     =            1
///      body ID        =          499
///      name found     =  T
///      surface string = MGS MOLA  64 pixel/deg
///
///      surface ID     =            1
///      body ID        =          401
///      name found     =  T
///      surface string = PHOBOS GASKELL Q512
///
///      surface ID     =            2
///      body ID        =          499
///      name found     =  T
///      surface string = MGS MOLA 128 pixel/deg
///
///      surface ID     =            3
///      body ID        =          499
///      name found     =  F
///      surface string = 3
///
///      surface ID     =            1
///      body ID        =           -1
///      name found     =  F
///      surface string = 1
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 12-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 14-JAN-2016 (NJB) (EDW) (BVS)
/// ```
pub fn srfc2s(
    ctx: &mut SpiceContext,
    code: i32,
    bodyid: i32,
    srfstr: &mut str,
    isname: &mut bool,
) -> crate::Result<()> {
    SRFC2S(
        code,
        bodyid,
        fstr::StrBytes::new(srfstr).as_mut(),
        isname,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SRFC2S ( Surface and body ID codes to surface string )
pub fn SRFC2S(
    CODE: i32,
    BODYID: i32,
    SRFSTR: &mut [u8],
    ISNAME: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"SRFC2S", ctx)?;

    //
    // Try to translate the input codes to a known surface name.
    //
    ZZSRFC2N(CODE, BODYID, SRFSTR, ISNAME, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SRFC2S", ctx)?;
        return Ok(());
    }

    //
    // If there is no matching name, convert the surface ID code to a
    // string representation.
    //
    if !*ISNAME {
        INTSTR(CODE, SRFSTR, ctx);
    }

    CHKOUT(b"SRFC2S", ctx)?;
    Ok(())
}
