//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// S/P Kernel, evaluate, type 3
///
/// Evaluate a single SPK data record from a segment of type 3
/// (Chebyshev Polynomials, position and velocity).
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
///  ET         I   Evaluation epoch.
///  RECORD     I   Data record.
///  STATE      O   State (position and velocity).
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
///           structure for SPK type 3 data is:
///
///              +--------------------------------------+
///              | record size (excluding this element) |
///              +--------------------------------------+
///              | Coverage interval midpoint           |
///              +--------------------------------------+
///              | Coverage interval radius             |
///              +--------------------------------------+
///              | Coeffs for X position component      |
///              +--------------------------------------+
///              | Coeffs for Y position component      |
///              +--------------------------------------+
///              | Coeffs for Z position component      |
///              +--------------------------------------+
///              | Coeffs for X velocity component      |
///              +--------------------------------------+
///              | Coeffs for Y velocity component      |
///              +--------------------------------------+
///              | Coeffs for Z velocity component      |
///              +--------------------------------------+
///
///           In the above record
///
///              - Times are expressed as seconds past J2000 TDB.
///              - Position components have units of km.
///              - Velocity components have units of km/s.
///
///           RECORD must be declared by the caller with size large
///           enough to accommodate the largest record that can be
///           returned by this routine. See the INCLUDE file
///           spkrec.inc for the correct record length.
/// ```
///
/// # Detailed Output
///
/// ```text
///  STATE    is the state. In order, X, Y, Z, X', Y', and Z'.
///           Units are km and km/sec.
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
///  The exact format and structure of type 3 (Chebyshev polynomials,
///  position and velocity) segments are described in the SPK
///  Required Reading file.
///
///  A type 3 segment contains six sets of Chebyshev coefficients,
///  one set each for the position coordinates X, Y, and Z, and one
///  set each for the velocity coordinates X', Y', and Z'.  SPKE03
///  calls the routine CHBVAL to evaluate each polynomial, and arrive
///  at the complete state.
/// ```
///
/// # Examples
///
/// ```text
///  The SPKEnn routines are almost always used in conjunction with
///  the corresponding SPKRnn routines, which read the records from
///  SPK files.
///
///  The data returned by the SPKRnn routine is in its rawest form,
///  taken directly from the segment. As such, it will be meaningless
///  to a user unless he/she understands the structure of the data type
///  completely. Given that understanding, however, the SPKRnn
///  routines might be used to examine raw segment data before
///  evaluating it with the SPKEnn routines.
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
///        IF ( TYPE .EQ. 3 ) THEN
///
///           CALL SPKR03 ( HANDLE, DESCR, ET, RECORD )
///               .
///               .  Look at the RECORD data.
///               .
///           CALL SPKE03 ( ET, RECORD, STATE )
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
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  R.E. Thurman       (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 14-APR-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Moved SPK
///         required reading from $Literature_References to
///         $Required_Reading section.
///
/// -    SPICELIB Version 2.0.0, 31-DEC-2013 (NJB)
///
///         Added error checks for invalid coefficient counts
///         and invalid interval radius. Changed error handling
///         style to "discovery." Enhanced header documentation.
///
/// -    SPICELIB Version 1.0.3, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.2, 23-AUG-1991 (HAN)
///
///         SPK03 was removed from the $Required_Reading section of the
///         header. The information in the SPK03 Required Reading file
///         is now part of the SPK Required Reading file.
///
/// -    SPICELIB Version 1.0.1, 22-MAR-1990 (HAN)
///
///         Literature references added to the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (RET)
/// ```
pub fn spke03(
    ctx: &mut SpiceContext,
    et: f64,
    record: &[f64],
    state: &mut [f64; 6],
) -> crate::Result<()> {
    SPKE03(et, record, state, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKE03 ( S/P Kernel, evaluate, type 3 )
pub fn SPKE03(
    ET: f64,
    RECORD: &[f64],
    STATE: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let RECORD = DummyArray::new(RECORD, 1..);
    let mut STATE = DummyArrayMut::new(STATE, 1..=6);
    let mut NCOF: i32 = 0;
    let mut DEGP: i32 = 0;
    let mut COFLOC: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // The first number in the record is the record size.  Following it
    // are two numbers that will be used later, then the six sets of
    // coefficients.  The number of coefficients for each quantity can
    // be determined from the record size, since there are the same
    // number of coefficients for each quantity.
    //
    NCOF = (((RECORD[1] as i32) - 2) / 6);

    if (NCOF < 1) {
        CHKIN(b"SPKE03", ctx)?;
        SETMSG(
            b"The input record\'s coefficient count NCOF should be positive but was #.",
            ctx,
        );
        ERRINT(b"#", NCOF, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"SPKE03", ctx)?;
        return Ok(());
    }

    //
    // Check the radius of the domain interval.
    //
    if (RECORD[3] <= 0.0) {
        CHKIN(b"SPKE03", ctx)?;
        SETMSG(b"Interval radius must be positive but was #.", ctx);
        ERRDP(b"#", RECORD[3], ctx);
        SIGERR(b"SPICE(INVALIDRADIUS)", ctx)?;
        CHKOUT(b"SPKE03", ctx)?;
        return Ok(());
    }

    //
    // The degree of each polynomial is one less than the number of
    // coefficients.
    //
    DEGP = (NCOF - 1);

    //
    // Call CHBVAL once for each quantity to evaluate the position
    // and velocity values.
    //
    for I in 1..=6 {
        //
        // The coefficients for each variable are located contiguously,
        // following the first three words in the record.
        //
        COFLOC = ((NCOF * (I - 1)) + 4);

        //
        // CHBVAL needs as input the coefficients, the degree of the
        // polynomial, the epoch, and also two variable transformation
        // parameters, which are located, in our case, in the second and
        // third slots of the record.
        //
        CHBVAL(
            RECORD.subarray(COFLOC),
            DEGP,
            RECORD.subarray(2),
            ET,
            &mut STATE[I],
        );
    }

    Ok(())
}
