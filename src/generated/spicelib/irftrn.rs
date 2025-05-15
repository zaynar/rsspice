//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Inertial reference frame transformation
///
/// Return the matrix that transforms vectors from one specified
/// inertial reference frame to another.
///
/// # Required Reading
///
/// * [SPK](crate::required_reading::spk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  REFA       I   Name of reference frame to transform vectors FROM.
///  REFB       I   Name of reference frame to transform vectors TO.
///  ROTAB      O   REFA-to-REFB transformation matrix.
/// ```
///
/// # Detailed Input
///
/// ```text
///  REFA,
///  REFB     are the names of two inertial reference frames. Any names
///           accepted by the routine IRFNUM may be used. See
///           $Particulars for a list of some of the more commonly used
///           inertial reference frame names.
/// ```
///
/// # Detailed Output
///
/// ```text
///  ROTAB    is a rotation matrix that transforms the
///           coordinates of a vector V relative to the
///           reference frame specified by REFA to the
///           coordinates of V relative to the reference frame
///           specified by REFB. The transformation is carried
///           out by the matrix multiplication
///
///              V = ROTAB * V.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If either of the input reference frame names is invalid, an
///      error is signaled by a routine in the call tree of this
///      routine.
/// ```
///
/// # Particulars
///
/// ```text
///  Normally applications should call the more general, higher level
///  routine PXFORM instead of this routine.
///
///  This routine is a macro that replaces the code fragment
///
///     CALL IRFNUM ( REFA,  CODEA        )
///     CALL IRFNUM ( REFB,  CODEB        )
///     CALL IRFROT ( CODEA, CODEB, ROTAB )
///
///
///  Among the reference frame names accepted by IRFNUM are:
///
///     'J2000'
///     'B1950'
///     'FK4'
///     'DE-96'
///     'DE-102'
///     'DE-108'
///     'DE-111'
///     'DE-114'
///     'DE-118'
///     'DE-122'
///     'DE-125'
///     'DE-130'
///     'DE-200'
///     'DE-202'
///     'GALACTIC'
///
///  See the SPICELIB routine GHGIRF for details.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Transform a vector V1950 from the B1950 to the J2000
///      reference frame.
///
///         C
///         C     Ask IRFTRN for the matrix that transforms vectors
///         C     from the B1950 to the J2000 reference frame.
///         C
///               CALL IRFTRN ( 'B1950', 'J2000', TRANS )
///
///         C
///         C     Now transform V1950 to the J2000 reference frame.
///         C
///               CALL MXV ( TRANS, V1950, V2000 )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 20-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.2, 28-SEP-2004 (NJB)
///
///         Corrected comment in code example in header. Made other minor
///         updates to header.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 30-AUG-1991 (NJB)
/// ```
pub fn irftrn(
    ctx: &mut SpiceContext,
    refa: &str,
    refb: &str,
    rotab: &mut [[f64; 3]; 3],
) -> crate::Result<()> {
    IRFTRN(
        refa.as_bytes(),
        refb.as_bytes(),
        rotab.as_flattened_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure IRFTRN ( Inertial reference frame transformation )
pub fn IRFTRN(
    REFA: &[u8],
    REFB: &[u8],
    ROTAB: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut ROTAB = DummyArrayMut2D::new(ROTAB, 1..=3, 1..=3);
    let mut CODEA: i32 = 0;
    let mut CODEB: i32 = 0;

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
        CHKIN(b"IRFTRN", ctx)?;
    }

    //
    // Encode the reference frame names, and find the transformation
    // matrix.
    //
    IRFNUM(REFA, &mut CODEA, ctx);
    IRFNUM(REFB, &mut CODEB, ctx);
    IRFROT(CODEA, CODEB, ROTAB.as_slice_mut(), ctx)?;

    CHKOUT(b"IRFTRN", ctx)?;
    Ok(())
}
