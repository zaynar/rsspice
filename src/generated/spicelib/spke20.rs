//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// SPK, evaluate Chebyshev polynomials, type 20
///
/// Evaluate a single data record from an SPK or PCK segment of type
/// 20 (Chebyshev polynomials, velocity only).
///
/// # Required Reading
///
/// * [SPK](crate::required_reading::spk)
/// * [PCK](crate::required_reading::pck)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ET         I   Evaluation epoch.
///  RECORD     I   Data record.
///  XYZDOT     O   Three function components and their derivatives.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ET       is the epoch at which a state vector or Euler angle
///           state is to be computed. The epoch is represented as
///           seconds past J2000 TDB.
///
///  RECORD   is a data record which, when evaluated at epoch ET,
///           will yield three function components and their
///           derivatives with respect to time. The record
///           structure for SPK type 20 data is:
///
///              +--------------------------------------+
///              | record size (excluding this element) |
///              +--------------------------------------+
///              | Coverage interval midpoint           |
///              +--------------------------------------+
///              | Coverage interval radius             |
///              +--------------------------------------+
///              | Coeffs for X velocity component      |
///              +--------------------------------------+
///              | Coeffs for Y velocity component      |
///              +--------------------------------------+
///              | Coeffs for Z velocity component      |
///              +--------------------------------------+
///              | X position component                 |
///              +--------------------------------------+
///              | Y position component                 |
///              +--------------------------------------+
///              | Z position component                 |
///              +--------------------------------------+
///
///           In the above record
///
///              - Times are expressed as seconds past J2000 TDB.
///              - Position components have units of km.
///              - Velocity coefficients have units of km/s.
///
///           See PCKE20 for a description of PCK type 20 records.
///
///           PCK type 20 records contain coefficients for Euler
///           angle rates and Euler angles corresponding to the
///           interval midpoint. See PCKE20 for a more detailed
///           description of the contents of PCK type 20 records.
/// ```
///
/// # Detailed Output
///
/// ```text
///  XYZDOT   is a 6-vector. In order, the components of XYZDOT are
///           X, Y, Z, X', Y', and Z'. Units for state evaluations
///           will be km and km/sec. Units for angles will be
///           radians and radians/sec.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input record contains an invalid coefficient count,
///      the error SPICE(INVALIDCOUNT) is signaled.
///
///  2)  If the input record contains invalid domain transformation
///      parameters, an error is signaled by a routine in the
///      call tree of this routine.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine evaluates both type 20 SPK and PCK records.
///
///  The exact format and structure of type 20 (Chebyshev polynomials,
///  position only) segments are described in the SPK and PCK Required
///  Reading files.
///
///  A type 20 record contains three sets of Chebyshev coefficients---
///  one set each for velocity components dX/dt, dY/dt, and dZ/dt. It
///  also contains a position vector (or for a PCK record, Euler
///  angles) associated with the midpoint of the record's coverage
///  interval. The position (or orientation) is obtained from the
///  indefinite integral of the velocity and the given vector.
/// ```
///
/// # Examples
///
/// ```text
///  The data returned by the routine is in its rawest form,
///  taken directly from the segment. As such, it will be meaningless
///  to a user unless he/she understands the structure of the data type
///  completely.
///
///  The code fragment below demonstrates reading and evaluating
///  a type 20 SPK record.
///
///
///  C
///  C     Get a segment applicable to a specified body and epoch.
///  C
///        CALL SPKSFS ( BODY, ET, HANDLE, DESCR, IDENT, FOUND )
///
///  C
///  C     Look at parts of the descriptor.
///  C
///        CALL DAFUS ( DESCR, 2, 6, DCD, ICD )
///        CENTER = ICD( 2 )
///        REF    = ICD( 3 )
///        TYPE   = ICD( 4 )
///
///        IF ( TYPE .EQ. 20 ) THEN
///
///           CALL SPKR20 ( HANDLE, DESCR, ET, RECORD )
///               .
///               .  Look at the RECORD data.
///               .
///           CALL SPKE20 ( ET, RECORD, XYZDOT )
///               .
///               .  Check out the evaluated state.
///               .
///        END IF
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  R.E. Thurman       (JPL)
///  K.S. Zukor         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 14-APR-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Moved SPK
///         required reading from $Literature_References to
///         $Required_Reading section.
///
/// -    SPICELIB Version 1.0.0, 17-JAN-2014 (NJB) (RET) (KSZ)
/// ```
pub fn spke20(
    ctx: &mut SpiceContext,
    et: f64,
    record: &[f64],
    xyzdot: &mut [f64; 6],
) -> crate::Result<()> {
    SPKE20(et, record, xyzdot, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKE20 ( SPK, evaluate Chebyshev polynomials, type 20 )
pub fn SPKE20(
    ET: f64,
    RECORD: &[f64],
    XYZDOT: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let RECORD = DummyArray::new(RECORD, 1..);
    let mut XYZDOT = DummyArrayMut::new(XYZDOT, 1..=6);
    let mut INTGRL = StackArray::<f64, 3>::new(1..=3);
    let mut DEGP: i32 = 0;
    let mut J: i32 = 0;
    let mut NCOF: i32 = 0;
    let mut POSLOC: i32 = 0;

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
    }

    CHKIN(b"SPKE20", ctx)?;

    //
    // The first number in the record is the record size. This is the
    // number of elements in the record, excluding the size itself.
    // Following it are the record's midpoint and radius, then the three
    // sets of coefficients. The record ends with three values which
    // represent either a position or orientation at the record coverage
    // interval's midpoint. The number of coefficients for each variable
    // can be determined from the record size, since there are the same
    // number of coefficients for each variable.
    //
    // The number of items counted by RECORD(1), other than the
    // Chebyshev coefficients, is 5.
    //
    NCOF = (((RECORD[1] as i32) - 5) / 3);

    if (NCOF < 1) {
        SETMSG(
            b"The input record\'s coefficient count NCOF should be positive but was #.",
            ctx,
        );
        ERRINT(b"#", NCOF, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"SPKE20", ctx)?;
        return Ok(());
    }

    //
    // The degree of each polynomial is one less than the number of
    // coefficients.
    //
    DEGP = (NCOF - 1);

    //
    // Pass the Chebyshev coefficient portion of the record into CHBIGR,
    // which will evaluate the expansion and its integral for each
    // component. The constants of integration are selected so that the
    // integrals are zero when the input time is the interval midpoint.
    //
    for I in 1..=3 {
        J = (4 + ((I - 1) * NCOF));

        CHBIGR(
            DEGP,
            RECORD.subarray(J),
            RECORD.subarray(2),
            ET,
            &mut XYZDOT[(3 + I)],
            &mut INTGRL[I],
            ctx,
        )?;
    }

    //
    // Add the position vector or Euler angles at the interval midpoint
    // to the integral.
    //
    POSLOC = (4 + (3 * NCOF));

    VADD(
        RECORD.subarray(POSLOC),
        INTGRL.as_slice(),
        XYZDOT.as_slice_mut(),
    );

    CHKOUT(b"SPKE20", ctx)?;
    Ok(())
}
