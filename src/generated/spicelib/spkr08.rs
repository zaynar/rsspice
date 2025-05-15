//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const ND: i32 = 2;
const NI: i32 = 6;
const STATSZ: i32 = 6;
const CTRLSZ: i32 = 4;
const STAIDX: i32 = 4;

/// Read SPK record from segment, type 8
///
/// Read a single SPK data record from a segment of type 8
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
///  HANDLE     I   File handle.
///  DESCR      I   Segment descriptor.
///  ET         I   Target epoch.
///  RECORD     O   Data record.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE,
///  DESCR    are the file handle and segment descriptor for
///           a SPK segment of type 8.
///
///  ET       is a target epoch, for which a data record from
///           a specific segment is required.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RECORD   is a set of data from the specified segment which,
///           when evaluated at epoch ET, will give the state
///           (position and velocity) of some body, relative
///           to some center, in some inertial reference frame.
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
/// # Exceptions
///
/// ```text
///  This routine follows the pattern established in the lower-numbered
///  SPK data type readers of not explicitly performing error
///  diagnoses. Exceptions are listed below nonetheless.
///
///  1)  If the input HANDLE does not designate a loaded SPK file, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  2)  If the segment specified by DESCR is not of data type 08,
///      the error SPICE(WRONGSPKTYPE) is signaled.
///
///  3)  If the input ET value is not within the range specified
///      in the segment descriptor, the error SPICE(TIMEOUTOFBOUNDS)
///      is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See argument HANDLE.
/// ```
///
/// # Particulars
///
/// ```text
///  See the SPK Required Reading file for a description of the
///  structure of a data type 8 segment.
/// ```
///
/// # Examples
///
/// ```text
///  The data returned by the SPKRnn routine is in its rawest form,
///  taken directly from the segment. As such, it will be meaningless
///  to a user unless he/she understands the structure of the data type
///  completely. Given that understanding, however, the SPKRxx
///  routines might be used to "dump" and check segment data for a
///  particular epoch.
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
///           CALL SPKR08 ( HANDLE, DESCR, ET, RECORD )
///               .
///               .  Look at the RECORD data.
///               .
///        END IF
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  Correctness of inputs must be ensured by the caller of
///      this routine.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.1, 12-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.1.0, 07-SEP-2001 (EDW)
///
///         Replaced DAFRDA call with DAFGDA.
///         Added IMPLICIT NONE.
///
/// -    SPICELIB Version 2.0.0, 06-NOV-1999 (NJB)
///
///         Data type check was relaxed to enable reading type 12
///         segments.
///
/// -    SPICELIB Version 1.0.1, 24-OCT-1994 (NJB)
///
///         In-line comment concerning transpose of state data was
///         removed.
///
/// -    SPICELIB Version 1.0.0, 14-AUG-1993 (NJB)
/// ```
pub fn spkr08(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64; 5],
    et: f64,
    record: &mut [f64],
) -> crate::Result<()> {
    SPKR08(handle, descr, et, record, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKR08 ( Read SPK record from segment, type 8 )
pub fn SPKR08(
    HANDLE: i32,
    DESCR: &[f64],
    ET: f64,
    RECORD: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DESCR = DummyArray::new(DESCR, 1..=5);
    let mut RECORD = DummyArrayMut::new(RECORD, 1..);
    let mut CONTRL = StackArray::<f64, 4>::new(1..=CTRLSZ);
    let mut DC = StackArray::<f64, 2>::new(1..=ND);
    let mut START: f64 = 0.0;
    let mut STEP: f64 = 0.0;
    let mut BEGIN: i32 = 0;
    let mut DEGREE: i32 = 0;
    let mut END: i32 = 0;
    let mut FIRST: i32 = 0;
    let mut GRPSIZ: i32 = 0;
    let mut IC = StackArray::<i32, 6>::new(1..=NI);
    let mut LAST: i32 = 0;
    let mut LOW: i32 = 0;
    let mut N: i32 = 0;
    let mut NEAR: i32 = 0;
    let mut TYPE: i32 = 0;

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
    // Use discovery check-in.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // Unpack the segment descriptor, and get the start and end addresses
    // of the segment.
    //
    DAFUS(DESCR.as_slice(), 2, 6, DC.as_slice_mut(), IC.as_slice_mut());

    TYPE = IC[4];
    BEGIN = IC[5];
    END = IC[6];

    //
    // Make sure that this really is a type 8 or type 12 data segment.
    //
    if ((TYPE != 8) && (TYPE != 12)) {
        CHKIN(b"SPKR08", ctx)?;
        SETMSG(
            b"You are attempting to locate type 8 or type 12 data in a type # data segment.",
            ctx,
        );
        ERRINT(b"#", TYPE, ctx);
        SIGERR(b"SPICE(WRONGSPKTYPE)", ctx)?;
        CHKOUT(b"SPKR08", ctx)?;
        return Ok(());
    }

    //
    // Check the request time against the bounds in the segment
    // descriptor.
    //
    if ((ET < DC[1]) || (ET > DC[2])) {
        CHKIN(b"SPKR08", ctx)?;
        SETMSG(
            b"Request time # is outside of descriptor bounds # : #.",
            ctx,
        );
        ERRDP(b"#", ET, ctx);
        ERRDP(b"#", DC[1], ctx);
        ERRDP(b"#", DC[2], ctx);
        SIGERR(b"SPICE(TIMEOUTOFBOUNDS)", ctx)?;
        CHKOUT(b"SPKR08", ctx)?;
        return Ok(());
    }

    //
    // The type 8 segment structure is described by this diagram from
    // the SPK Required Reading:
    //
    //    +-----------------------+
    //    | State 1               |
    //    +-----------------------+
    //    | State 2               |
    //    +-----------------------+
    //                .
    //                .
    //                .
    //    +-----------------------+
    //    | State N               |
    //    +-----------------------+
    //    | Epoch of state 1 (ET) |
    //    +-----------------------+
    //    | Step size             |
    //    +-----------------------+
    //    | Polynomial degree     |
    //    +-----------------------+
    //    | Number of states      |
    //    +-----------------------+
    //
    //
    // We'll need the last four items before we can determine which
    // states make up our output record.
    //
    //
    DAFGDA(HANDLE, (END - 3), END, CONTRL.as_slice_mut(), ctx)?;

    START = CONTRL[1];
    STEP = CONTRL[2];
    DEGREE = intrinsics::IDNINT(CONTRL[3]);
    N = intrinsics::IDNINT(CONTRL[4]);

    GRPSIZ = (DEGREE + 1);

    //
    // We'll now select the set of states that define the interpolating
    // polynomials.  The cases of odd and even GRPSIZ are handled
    // separately.
    //
    if ODD(GRPSIZ) {
        //
        // Find the index of the state whose epoch is closest to the
        // input epoch.  Find the first and last indices in the record
        // of the (GRPSIZ-1)/2 states on either side of this central
        // state.
        //
        NEAR = (intrinsics::IDNINT(((ET - START) / STEP)) + 1);

        FIRST = intrinsics::MIN0(&[intrinsics::MAX0(&[1, (NEAR - (DEGREE / 2))]), (N - DEGREE)]);
        LAST = (FIRST + DEGREE);
    } else {
        //
        // Find the index of the last state whose epoch is less than or
        // equal to that of the input epoch.  Find the first and last
        // indices in the record of the set of GRPSIZ consecutive states
        // having this state as the (GRPSIZ/2)th one.
        //
        LOW = ((((ET - START) / STEP) as i32) + 1);

        FIRST = intrinsics::MIN0(&[intrinsics::MAX0(&[1, (LOW - (DEGREE / 2))]), (N - DEGREE)]);
        LAST = (FIRST + DEGREE);
    }

    //
    // Put the size of the group of states, the epoch of the first
    // state in the record, and the step size into the output record.
    //
    RECORD[1] = GRPSIZ as f64;
    RECORD[2] = (START + (((FIRST - 1) as f64) * STEP));
    RECORD[3] = STEP;

    //
    // Read the states.
    //
    DAFGDA(
        HANDLE,
        (BEGIN + ((FIRST - 1) * STATSZ)),
        ((BEGIN + (LAST * STATSZ)) - 1),
        RECORD.subarray_mut(STAIDX),
        ctx,
    )?;

    Ok(())
}
