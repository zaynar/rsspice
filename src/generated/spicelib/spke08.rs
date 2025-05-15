//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const MAXREC: i32 = 198;
const STATSZ: i32 = 6;
const SIZIDX: i32 = 1;
const BEGIDX: i32 = 2;
const STPIDX: i32 = 3;
const STAIDX: i32 = 4;

struct SaveVars {
    LOCREC: StackArray<f64, 198>,
    WORK: StackArray<f64, 198>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut LOCREC = StackArray::<f64, 198>::new(1..=MAXREC);
        let mut WORK = StackArray::<f64, 198>::new(1..=MAXREC);

        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::D(0.0), MAXREC as usize))
                .chain([]);

            WORK.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { LOCREC, WORK }
    }
}

/// S/P Kernel, evaluate, type 8
///
/// Evaluate a single SPK data record from a segment of type 8
/// (equally spaced discrete states, interpolated by Lagrange
/// polynomials).
///
/// # Required Reading
///
/// * [SPK](crate::required_reading::spk)
/// * [TIME](crate::required_reading::time)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ET         I   Target epoch.
///  RECORD     I   Data record.
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
///           will obtain RECORD by calling SPKR08.
///
///           The structure of the record is as follows:
///
///              +----------------------+
///              | number of states (n) |
///              +----------------------+
///              | start epoch          |
///              +----------------------+
///              | step size            |
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
/// ```
///
/// # Detailed Output
///
/// ```text
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
///  The exact format and structure of type 8 (equally spaced discrete
///  states, interpolated by Lagrange polynomials) segments are
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
///        IF ( TYPE .EQ. 8 ) THEN
///
///           CALL SPKR08 ( HANDLE, DESCR, ET, RECORD )
///               .
///               .  Look at the RECORD data.
///               .
///           CALL SPKE08 ( ET, RECORD, STATE )
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
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.0.1, 12-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.0.0, 10-DEC-2013 (NJB)
///
///         RECORD is now strictly an input; it is not overwritten by this
///         routine. Formerly RECORD was used as a workspace array.
///
/// -    SPICELIB Version 1.1.0, 25-AUG-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in XPOSEG and LGRESP calls.
///
/// -    SPICELIB Version 1.0.0, 14-AUG-1993 (NJB)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.1.0, 25-AUG-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in XPOSEG and LGRESP calls.
/// ```
pub fn spke08(
    ctx: &mut SpiceContext,
    et: f64,
    record: &[f64],
    state: &mut [f64; 6],
) -> crate::Result<()> {
    SPKE08(et, record, state, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKE08 ( S/P Kernel, evaluate, type 8 )
pub fn SPKE08(
    ET: f64,
    RECORD: &[f64],
    STATE: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let RECORD = DummyArray::new(RECORD, 1..);
    let mut STATE = DummyArrayMut::new(STATE, 1..=6);
    let mut N: i32 = 0;
    let mut YSTART: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    //
    // Size of a state vector:
    //

    //
    // Indices of input record elements:
    //
    //    -- size
    //    -- start epoch
    //    -- step size
    //    -- start of state information
    //

    //
    // Local variables
    //

    //
    // Saved values
    //
    // Save arrays to prevent stack overflow problems on some
    // platforms.
    //

    //
    // Initial values
    //
    //
    // Initialize the workspace array to suppress compiler warnings.
    //

    //
    // Use discovery check-in.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // We'll transpose the state information in the input record
    // so that contiguous pieces of it can be shoved directly into the
    // interpolation routine LGRESP.
    //
    N = intrinsics::IDNINT(RECORD[SIZIDX]);

    XPOSEG(
        RECORD.subarray(STAIDX),
        STATSZ,
        N,
        save.LOCREC.as_slice_mut(),
    );

    //
    // We interpolate each state component in turn.
    //
    for I in 1..=STATSZ {
        YSTART = (1 + ((I - 1) * N));

        STATE[I] = LGRESP(
            N,
            RECORD[BEGIDX],
            RECORD[STPIDX],
            save.LOCREC.subarray(YSTART),
            save.WORK.as_slice_mut(),
            ET,
            ctx,
        )?;
    }

    Ok(())
}
