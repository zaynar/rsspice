//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const MAXREC: i32 = 129;
const SIZIDX: i32 = 1;

/// S/P Kernel, evaluate, type 13
///
/// Evaluate a single data record from a type 13 SPK segment.
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
///  MAXREC     P   Maximum size of SPK record. See SPKPVN.
///  ET         I   Epoch for which a state is desired.
///  RECORD     I   Record from a type 13 SPK segment valid for ET.
///  STATE      O   State (position and velocity) at epoch ET.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ET       is the epoch for which a state vector is desired.
///
///  RECORD   is a record from a type 13 SPK segment which, when
///           evaluated at epoch ET, will give the state
///           (position and velocity) of some body, relative to
///           some center, in some inertial reference frame.
///
///           The structure of the record is as follows:
///
///              +----------------------+
///              | number of states (n) |
///              +----------------------+
///              | state 1 (6 elts.)    |
///              +----------------------+
///              | state 2 (6 elts.)    |
///              +----------------------+
///                          .
///                          .
///                          .
///              +----------------------+
///              | state n (6 elts.)    |
///              +----------------------+
///              | epochs 1--n          |
///              +----------------------+
/// ```
///
/// # Detailed Output
///
/// ```text
///  STATE    is the state vector at epoch ET. Its contents are, in
///           order, X, Y, Z, X', Y', and Z'. Units are km and km/sec.
/// ```
///
/// # Parameters
///
/// ```text
///  MAXREC   is the maximum size of SPK record. See the SPICELIB
///           routine SPKPVN for details.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If an error occurs while interpolating the SPK data, the
///      error is signaled by a routine in the call tree of this
///      routine.
/// ```
///
/// # Particulars
///
/// ```text
///  The exact format and structure of type 13 (unequally spaced
///  discrete states, evaluated by Hermite interpolation) SPK segments
///  is described in the SPK Required Reading.
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
///        IF ( TYPE .EQ. 13 ) THEN
///
///           CALL SPKR13 ( HANDLE, DESCR, ET, RECORD )
///               .
///               .  Look at the RECORD data.
///               .
///           CALL SPKE13 ( ET, RECORD, STATE )
///               .
///               .  Check out the evaluated state.
///               .
///        END IF
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine assumes that the input record is valid. Any
///      checking of the input data is assumed to have been performed
///      when the source SPK file was created.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 14-APR-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Updated
///         $Exceptions section to describe possible issues detected by
///         this routine. Moved SPK required reading from
///         $Literature_References to $Required_Reading section.
///
/// -    SPICELIB Version 1.0.0, 25-FEB-2000 (NJB)
/// ```
pub fn spke13(
    ctx: &mut SpiceContext,
    et: f64,
    record: &[f64],
    state: &mut [f64; 6],
) -> crate::Result<()> {
    SPKE13(et, record, state, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKE13 ( S/P Kernel, evaluate, type 13 )
pub fn SPKE13(
    ET: f64,
    RECORD: &[f64],
    STATE: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let RECORD = DummyArray::new(RECORD, 1..);
    let mut STATE = DummyArrayMut::new(STATE, 1..=6);
    let mut LOCREC = StackArray::<f64, 129>::new(1..=MAXREC);
    let mut WORK = ActualArray2D::<f64>::new(1..=(MAXREC * 2), 1..=2);
    let mut FROM: i32 = 0;
    let mut N: i32 = 0;
    let mut TO: i32 = 0;
    let mut XSTART: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
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

    CHKIN(b"SPKE13", ctx)?;

    N = intrinsics::IDNINT(RECORD[SIZIDX]);

    //
    // We interpolate each state component in turn.
    //
    XSTART = (2 + (6 * N));

    for I in 1..=3 {
        for J in 1..=N {
            //
            // For the Jth input state vector, copy the Ith position and
            // velocity components into the local record buffer LOCREC.
            //
            FROM = ((1 + (6 * (J - 1))) + I);
            TO = ((2 * J) - 1);

            LOCREC[TO] = RECORD[FROM];
            LOCREC[(TO + 1)] = RECORD[(FROM + 3)];
        }

        //
        // Interpolate the Ith position and velocity components of the
        // state.
        //
        let [arg5, arg6] = STATE
            .get_disjoint_mut([I, (I + 3)])
            .expect("mutable array elements passed to function must have disjoint indexes");
        HRMINT(
            N,
            RECORD.subarray(XSTART),
            LOCREC.as_slice(),
            ET,
            WORK.as_slice_mut(),
            arg5,
            arg6,
            ctx,
        )?;
    }

    CHKOUT(b"SPKE13", ctx)?;
    Ok(())
}
