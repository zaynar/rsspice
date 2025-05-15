//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// PCK, evaluate data record from type 2 segment
///
/// Evaluate a single PCK data record from a segment of type 2
/// (Chebyshev Polynomials, position only).
///
/// # Required Reading
///
/// * [PCK](crate::required_reading::pck)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ET         I   Epoch.
///  RECORD     I   Data record.
///  EULANG     O   Euler angles and their derivatives.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ET       is an epoch, at which the Euler angles are to
///           be computed.
///
///  RECORD   is a data record which, when evaluated at epoch ET,
///           will give the Euler angles of some body.
/// ```
///
/// # Detailed Output
///
/// ```text
///  EULANG   is the Euler angles and their derivatives at
///           time ET.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
/// ```
///
/// # Particulars
///
/// ```text
///  The exact format and structure of type 2 (Chebyshev polynomials,
///  position only) segments are described in the PCK Required Reading
///  file.
///
///  A type 2 segment contains three sets of Chebyshev coefficients,
///  one set each for the Euler angles phi, delta and psi.  PCKE02
///  calls the routine SPKE02 for each set to evaluate the polynomial
///  AND its first derivative.
/// ```
///
/// # Examples
///
/// ```text
///  The PCKEnn routines are almost always used in conjunction with
///     the corresponding PCKRnn routines, which read the records from
///     binary PCK files.
///
///     The data returned by the PCKRnn routine is in its rawest form,
///     taken directly from the segment. As such, it will be
///     meaningless to a user unless he/she understands the structure
///     of the data type completely. Given that understanding, however,
///     the PCKRnn routines might be used to examine raw segment data
///     before evaluating it with the PCKEnn routines.
///
///
///  Here we load a binary PCK files and use PCKE02 to get the
///  Euler angles.
///
///  C
///  C  Load binary PCK file.
///  C
///     CALL PCKLOF ('example.pck', HANDLE)
///
///
///  C  Get a segment applicable to a specified body and epoch.
///
///     CALL PCKSFS ( BODY, ET, HANDLE, DESCR, IDENT, FOUND )
///
///     IF ( FOUND ) THEN
///
///
///        Look at parts of the descriptor.
///
///        CALL DAFUS ( DESCR, ND, NI, DCD, ICD )
///        TYPE   = ICD( NT )
///        REF    = ICD( NR )
///
///        IF ( TYPE .EQ. 2 ) THEN
///
///           Read in Chebyshev coefficients from segment.
///
///           CALL PCKR02 ( HANDLE, DESCR, ET, RECORD )
///
///
///           Call evaluation routine to get Euler angles
///           phi, delta, w.
///
///           CALL PCKE02 ( ET, RECORD, EULANG )
///
///
///  The Euler angles and their derivatives are returned
///  in EULANG.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  E.D. Wright        (JPL)
///  K.S. Zukor         (JPL)
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
/// -    SPICELIB Version 1.1.1, 03-JAN-2014 (EDW)
///
///         Minor edits to $Procedure; clean trailing whitespace.
///         Removed unneeded $Revisions section.
///
/// -    SPICELIB Version 1.1.0, 13-MAR-1995 (KSZ)
///
///          Added error handling.
///
/// -    SPICELIB Version 1.0.0, 30-SEP-1994 (KSZ)
/// ```
pub fn pcke02(
    ctx: &mut SpiceContext,
    et: f64,
    record: &[f64],
    eulang: &mut [f64; 6],
) -> crate::Result<()> {
    PCKE02(et, record, eulang, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure PCKE02 ( PCK, evaluate data record from type 2 segment )
pub fn PCKE02(
    ET: f64,
    RECORD: &[f64],
    EULANG: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let RECORD = DummyArray::new(RECORD, 1..);
    let mut EULANG = DummyArrayMut::new(EULANG, 1..=6);

    //
    // SPICELIB functions
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"PCKE02", ctx)?;
    }

    //
    // Call evaluation routine to get Euler angles
    // phi, delta, w.
    //
    SPKE02(ET, RECORD.as_slice(), EULANG.as_slice_mut(), ctx)?;

    //
    // Mod the 3rd element of the state by TWOPI.
    // We do this because we've always done this.
    //
    EULANG[3] = intrinsics::DMOD(EULANG[3], TWOPI(ctx));

    CHKOUT(b"PCKE02", ctx)?;

    Ok(())
}
