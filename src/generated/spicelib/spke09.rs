//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const MAXREC: i32 = 198;
const STATSZ: i32 = 6;
const SIZIDX: i32 = 1;
const STAIDX: i32 = 2;

/// S/P Kernel, evaluate, type 9
///
/// Evaluate a single SPK data record from a segment of type 9
/// (discrete states, evaluated by Lagrange interpolation).
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
///  ET         I   Target epoch.
///  RECORD    I-O  Data record.
///  STATE      O   State (position and velocity).
/// ```
///
/// # Detailed Input
///
/// ```text
///  ET       is a target epoch, at which a state vector is to
///           be computed.
///
///  RECORD   is a data record which, when evaluated at epoch ET,
///           will give the state (position and velocity) of some
///           body, relative to some center, in some inertial
///           reference frame. Normally, the caller of this routine
///           will obtain RECORD by calling SPKR09.
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
///  RECORD   is the input record, modified by use as a work area.
///           On output, RECORD no longer contains useful
///           information.
///
///  STATE    is the state. In order, the elements are
///
///              X, Y, Z, X', Y', and Z'
///
///           Units are km and km/sec.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  The caller of this routine must ensure that the input record
///      is appropriate for the supplied ET value. Otherwise,
///      arithmetic overflow may result.
/// ```
///
/// # Particulars
///
/// ```text
///  The exact format and structure of type 9 (unequally spaced
///  discrete states, evaluated by Lagrange interpolation) segments are
///  described in the SPK Required Reading file.
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
///        IF ( TYPE .EQ. 9 ) THEN
///
///           CALL SPKR09 ( HANDLE, DESCR, ET, RECORD )
///               .
///               .  Look at the RECORD data.
///               .
///           CALL SPKE09 ( ET, RECORD, STATE )
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
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 12-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.1.0, 31-AUG-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in XPOSEG and LGRINT calls.
///
/// -    SPICELIB Version 1.0.0, 14-AUG-1993 (NJB) (RET)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.1.0, 31-AUG-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in XPOSEG and LGRINT calls.
/// ```
pub fn spke09(
    ctx: &mut SpiceContext,
    et: f64,
    record: &mut [f64],
    state: &mut [f64; 6],
) -> crate::Result<()> {
    SPKE09(et, record, state, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKE09 ( S/P Kernel, evaluate, type 9 )
pub fn SPKE09(
    ET: f64,
    RECORD: &mut [f64],
    STATE: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut RECORD = DummyArrayMut::new(RECORD, 1..);
    let mut STATE = DummyArrayMut::new(STATE, 1..=6);
    let mut LOCREC = StackArray::<f64, 198>::new(1..=MAXREC);
    let mut N: i32 = 0;
    let mut XSTART: i32 = 0;
    let mut YSTART: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Indices of input record elements:
    //
    //    -- size
    //    -- start of state information
    //

    //
    // Local variables
    //

    //
    // Discovery check-in.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // We'll transpose the state information in the input record
    // so that contiguous pieces of it can be shoved directly into the
    // interpolation routine LGRINT.  We allow LGRINT to overwrite the
    // state values in the input record, since this saves local storage
    // and does no harm.  (See the header of LGRINT for a description of
    // its work space usage.)
    //
    N = intrinsics::IDNINT(RECORD[SIZIDX]);

    XPOSEG(RECORD.subarray(STAIDX), STATSZ, N, LOCREC.as_slice_mut());
    MOVED(LOCREC.as_slice(), (STATSZ * N), RECORD.subarray_mut(STAIDX));

    //
    // We interpolate each state component in turn.
    //
    XSTART = (2 + (N * STATSZ));

    for I in 1..=STATSZ {
        YSTART = (2 + (N * (I - 1)));

        STATE[I] = LGRINT(
            N,
            RECORD.subarray(XSTART),
            RECORD.subarray(YSTART),
            LOCREC.as_slice_mut(),
            ET,
            ctx,
        )?;
    }

    Ok(())
}
