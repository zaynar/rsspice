//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// S/P Kernel, evaluate, type 14
///
/// Evaluate a single data record from a type 14 SPK segment.
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
///  ET         I   Epoch for which a state is desired.
///  RECORD     I   Record from a type 14 SPK segment valid for ET.
///  STATE      O   State (position and velocity) at epoch ET.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ET       is the epoch for which a state vector is desired.
///
///  RECORD   is a record from a type 14 SPK segment which, when
///           evaluated at epoch ET, will give the state (position
///           and velocity) of some body, relative to some center, in
///           some inertial reference frame.
/// ```
///
/// # Detailed Output
///
/// ```text
///  STATE    is the state vector at epoch ET. Its contents are, in
///           order, X, Y, Z, X', Y', and Z'. Units are km and km/sec.
/// ```
///
/// # Particulars
///
/// ```text
///  The exact format and structure of a type 14 SPK segment is
///  described in the SPK Required Reading.
///
///  A type 14 record contains six sets of Chebyshev coefficients,
///  one set each for the position coordinates X, Y, and Z, and one
///  set each for the velocity coordinates X', Y', and Z' of a state
///  vector.  SPKE14 calls the routine CHBVAL to evaluate each
///  Chebyshev polynomial, and arrive at the complete state.
/// ```
///
/// # Examples
///
/// ```text
///  The SPKEnn routines are almost always used in conjunction with
///  the corresponding SPKRnn routines, which read the records from
///  SPK files.
///
///  The data returned by the SPKRnn routine is in a raw form, taken
///  directly from the segment. As such, it will be not be directly
///  useful to a user unless they have a complete understanding of the
///  structure of the data type. Given that understanding, however,
///  the SPKRnn routines could be used to "dump" and check segment data
///  for a particular epoch before evaluating the record to obtain a
///  state vector, as in the example which follows.
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
///
///        CENTER = ICD( 2 )
///        REF    = ICD( 3 )
///        TYPE   = ICD( 4 )
///
///        IF ( TYPE .EQ. 14 ) THEN
///
///           CALL SPKR14 ( HANDLE, DESCR, ET, RECORD )
///               .
///               .  Look at the RECORD data.
///               .
///           CALL SPKE14 ( ET, RECORD, STATE )
///               .
///               .  Check out the evaluated state.
///               .
///        END IF
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 17-JUN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Moved SPK
///         required reading from $Literature_References to
///         $Required_Reading section.
///
/// -    SPICELIB Version 1.0.0, 10-MAR-1995 (KRG)
/// ```
pub fn spke14(
    ctx: &mut SpiceContext,
    et: f64,
    record: &[f64],
    state: &mut [f64; 6],
) -> crate::Result<()> {
    SPKE14(et, record, state, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKE14 ( S/P Kernel, evaluate, type 14 )
pub fn SPKE14(
    ET: f64,
    RECORD: &[f64],
    STATE: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let RECORD = DummyArray::new(RECORD, 1..);
    let mut STATE = DummyArrayMut::new(STATE, 1..=6);
    let mut COFLOC: i32 = 0;
    let mut DEGREE: i32 = 0;
    let mut NCOEFF: i32 = 0;

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
        CHKIN(b"SPKE14", ctx)?;
    }

    //
    // The first number in the record is the number of Chebyshev
    // Polynomial coefficients used to represent each component of the
    // state vector. Following it are two numbers that will be used
    // when evaluating the sets of coefficients, and finally the six sets
    // of coefficients.
    //
    NCOEFF = (RECORD[1] as i32);

    //
    // The degree of each polynomial is one less than the number of
    // coefficients.
    //
    DEGREE = (NCOEFF - 1);

    //
    // Call CHBVAL once for each quantity to evaluate the position
    // and velocity values.
    //
    for I in 1..=6 {
        //
        // The coefficients for each variable are located contiguously,
        // following the first three words in the record.
        //
        COFLOC = ((NCOEFF * (I - 1)) + 4);
        //
        // CHBVAL needs as input the coefficients, the degree of the
        // polynomial, also two variable transformation parameters, which
        // are located in the second and third slots of the record, and
        // the epoch. We get back the appropriate element of a state
        // vector.
        //
        CHBVAL(
            RECORD.subarray(COFLOC),
            DEGREE,
            RECORD.subarray(2),
            ET,
            &mut STATE[I],
        );
    }

    CHKOUT(b"SPKE14", ctx)?;
    Ok(())
}
