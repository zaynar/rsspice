//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const MSGLEN: i32 = 80;

/// Surface string and body ID code to surface ID code
///
/// Translate a surface string, together with a body ID code, to the
/// corresponding surface ID code. The input surface string may
/// contain a name or an integer ID code.
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
///  SRFSTR     I   Surface name or ID string.
///  BODYID     I   Body ID code.
///  CODE       O   Integer surface ID code.
///  FOUND      O   Flag indicating whether surface ID was found.
/// ```
///
/// # Detailed Input
///
/// ```text
///  SRFSTR   is a string designating a surface. SRFSTR may contain
///           a surface name or a string representation of the
///           surface's integer ID code.
///
///           If, for the body specified by BODYID, multiple surface
///           names are associated with one surface ID code, then any
///           of these names may be used as the value of SRFSTR.
///
///           Case and leading and trailing blanks in a surface name
///           are not significant. Sequences of consecutive embedded
///           blanks are considered equivalent to a single blank.
///           For example, all of the strings below are considered
///           to be equivalent:
///
///              'MGS MOLA 128 PIXEL/DEG'
///              'MGS MOLA 128 pixel/deg'
///              'MGS MOLA 128 PIXEL/DEG   '
///              'MGS MOLA 128    PIXEL/DEG'
///              '   MGS MOLA 128 PIXEL/DEG'
///
///           However,
///
///              'MGSMOLA 128PIXEL/DEG'
///
///           is not equivalent to the names above.
///
///
///  BODYID   is the integer ID code of the body associated with the
///           surface designated by SRFSTR.
/// ```
///
/// # Detailed Output
///
/// ```text
///  CODE     is integer ID code of the surface designated by
///           SRFSTR, for the body designated by BODYID, if for this
///           body an association exists between the input surface
///           string and a surface ID code. CODE is defined if and
///           only if the output flag FOUND is .TRUE.
///
///  FOUND    is a logical flag that is .TRUE. if a surface code
///           corresponding to the input surface string and body ID
///           code was found. FOUND is .FALSE. otherwise.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input surface string does not map to an ID code
///      and does not represent an integer, the output CODE is
///      undefined and the output FOUND is set to .FALSE.
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
///  SRFSCC is one of four related subroutines:
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
///     Translate each surface string and body ID pair to the
///     associated surface ID code. Also perform a translation
///     for a surface name having no matching ID.
///
///     Use the meta-kernel shown below to define the required SPICE
///     kernel variables.
///
///
///        KPL/MK
///
///        File: srfscc_ex1.tm
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
///           PROGRAM SRFSCC_EX1
///           IMPLICIT NONE
///
///           INCLUDE 'srftrn.inc'
///
///           INTEGER               FILSIZ
///           PARAMETER           ( FILSIZ = 255 )
///
///           INTEGER               NCASE
///           PARAMETER           ( NCASE = 7 )
///
///           INTEGER               BDNMLN
///           PARAMETER           ( BDNMLN = 36 )
///
///           CHARACTER*(FILSIZ)    META
///           CHARACTER*(SFNMLN)    SRFSTR ( NCASE )
///
///           INTEGER               BODYID ( NCASE )
///           INTEGER               I
///           INTEGER               SURFID
///
///           LOGICAL               FOUND
///
///
///           DATA  ( SRFSTR(I), BODYID(I), I = 1, NCASE )  /
///          .
///          .        'MGS MOLA  64 pixel/deg',    499,
///          .        'PHOBOS GASKELL Q512',       401,
///          .        'MGS MOLA 128 pixel/deg',    499,
///          .        '1',                         499,
///          .        '1',                         401,
///          .        '2',                         499,
///          .        'ZZZ',                       499      /
///
///
///           META = 'srfscc_ex1.tm'
///
///           CALL FURNSH ( META )
///
///           WRITE (*,*) ' '
///
///           DO I = 1, NCASE
///
///              CALL SRFSCC ( SRFSTR(I), BODYID(I),
///          .                 SURFID,    FOUND     )
///
///              WRITE (*,*) 'surface string   = ', SRFSTR(I)
///              WRITE (*,*) 'body ID          = ', BODYID(I)
///              WRITE (*,*) 'surface ID found = ', FOUND
///
///              IF ( FOUND ) THEN
///                 WRITE (*,*) 'surface ID       = ', SURFID
///              END IF
///
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
///      surface string   = MGS MOLA  64 pixel/deg
///      body ID          =          499
///      surface ID found =  T
///      surface ID       =            1
///
///      surface string   = PHOBOS GASKELL Q512
///      body ID          =          401
///      surface ID found =  T
///      surface ID       =            1
///
///      surface string   = MGS MOLA 128 pixel/deg
///      body ID          =          499
///      surface ID found =  T
///      surface ID       =            2
///
///      surface string   = 1
///      body ID          =          499
///      surface ID found =  T
///      surface ID       =            1
///
///      surface string   = 1
///      body ID          =          401
///      surface ID found =  T
///      surface ID       =            1
///
///      surface string   = 2
///      body ID          =          499
///      surface ID found =  T
///      surface ID       =            2
///
///      surface string   = ZZZ
///      body ID          =          499
///      surface ID found =  F
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
/// -    SPICELIB Version 1.0.1, 22-OCT-2021 (JDR) (NJB)
///
///         Edited the header to comply with NAIF standard.
///
///         Updated description of SRFSTR to indicate that any
///         surface name alias may be used.
///
/// -    SPICELIB Version 1.0.0, 14-JAN-2016 (NJB) (EDW) (BVS)
/// ```
pub fn srfscc(
    ctx: &mut SpiceContext,
    srfstr: &str,
    bodyid: i32,
    code: &mut i32,
    found: &mut bool,
) -> crate::Result<()> {
    SRFSCC(srfstr.as_bytes(), bodyid, code, found, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SRFSCC (Surface string and body ID code to surface ID code)
pub fn SRFSCC(
    SRFSTR: &[u8],
    BODYID: i32,
    CODE: &mut i32,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut ERRMSG = [b' '; MSGLEN as usize];
    let mut PTR: i32 = 0;

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

    CHKIN(b"SRFSCC", ctx)?;

    //
    // No result has been found yet.
    //
    *FOUND = false;

    //
    // Try to translate the input surface name to a known surface ID
    // code.
    //
    ZZSRFN2C(SRFSTR, BODYID, CODE, FOUND, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SRFSCC", ctx)?;
        return Ok(());
    }

    if !*FOUND {
        //
        // The surface string could not be mapped to an ID code.
        //
        // It's possible the name is a string representation of an
        // integer, for example, '999'.  If so, find the equivalent datum
        // of INTEGER type.
        //
        NPARSI(SRFSTR, CODE, &mut ERRMSG, &mut PTR, ctx);
        //
        // If the string parsed as an integer, PTR is zero; otherwise
        // it's non-zero.
        //
        *FOUND = (PTR == 0);
    }
    //
    // CODE and FOUND are set.
    //
    CHKOUT(b"SRFSCC", ctx)?;
    Ok(())
}
