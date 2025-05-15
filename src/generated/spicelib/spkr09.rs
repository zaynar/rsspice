//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const ND: i32 = 2;
const NI: i32 = 6;
const STATSZ: i32 = 6;
const DIRSIZ: i32 = 100;
const BUFSIZ: i32 = (DIRSIZ + 1);

/// Read SPK record from segment, type 9
///
/// Read a single SPK data record from a segment of type 9
/// (Unequally spaced discrete states, interpolated by Lagrange
/// polynomials).
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
///           a SPK segment of type 9.
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
///  2)  If the segment specified by DESCR is not of data types 9 or
///      13, the error SPICE(WRONGSPKTYPE) is signaled.
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
///  structure of a data type 9 segment.
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
///        IF ( TYPE .EQ. 9 ) THEN
///           CALL SPKR09 ( HANDLE, DESCR, ET, RECORD )
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
///         Data type check was relaxed to enable reading type 13
///         segments.
///
/// -    SPICELIB Version 1.0.1, 24-OCT-1994 (NJB)
///
///         In-line comment concerning transpose of state data was
///         removed.
///
/// -    SPICELIB Version 1.0.0, 14-AUG-1993 (NJB)
/// ```
pub fn spkr09(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64; 5],
    et: f64,
    record: &mut [f64],
) -> crate::Result<()> {
    SPKR09(handle, descr, et, record, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKR09 ( Read SPK record from segment, type 9 )
pub fn SPKR09(
    HANDLE: i32,
    DESCR: &[f64],
    ET: f64,
    RECORD: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DESCR = DummyArray::new(DESCR, 1..=5);
    let mut RECORD = DummyArrayMut::new(RECORD, 1..);
    let mut BUFFER = StackArray::<f64, 101>::new(1..=BUFSIZ);
    let mut CONTRL = StackArray::<f64, 2>::new(1..=2);
    let mut DC = StackArray::<f64, 2>::new(1..=ND);
    let mut BEGIDX: i32 = 0;
    let mut BEGIN: i32 = 0;
    let mut BUFBAS: i32 = 0;
    let mut DEGREE: i32 = 0;
    let mut DIRBAS: i32 = 0;
    let mut END: i32 = 0;
    let mut ENDIDX: i32 = 0;
    let mut FIRST: i32 = 0;
    let mut GROUP: i32 = 0;
    let mut HIGH: i32 = 0;
    let mut I: i32 = 0;
    let mut IC = StackArray::<i32, 6>::new(1..=NI);
    let mut LAST: i32 = 0;
    let mut LOW: i32 = 0;
    let mut N: i32 = 0;
    let mut NDIR: i32 = 0;
    let mut NEAR: i32 = 0;
    let mut NREAD: i32 = 0;
    let mut REMAIN: i32 = 0;
    let mut START: i32 = 0;
    let mut TIMBAS: i32 = 0;
    let mut TYPE: i32 = 0;
    let mut WNDSIZ: i32 = 0;

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
    // Make sure that this really is a type 9 or type 13 data segment.
    //
    if ((TYPE != 9) && (TYPE != 13)) {
        CHKIN(b"SPKR09", ctx)?;
        SETMSG(
            b"You are attempting to locate type 9 or type 13 data in a type # data segment.",
            ctx,
        );
        ERRINT(b"#", TYPE, ctx);
        SIGERR(b"SPICE(WRONGSPKTYPE)", ctx)?;
        CHKOUT(b"SPKR09", ctx)?;
        return Ok(());
    }

    //
    // Check the request time against the bounds in the segment
    // descriptor.
    //
    if ((ET < DC[1]) || (ET > DC[2])) {
        CHKIN(b"SPKR09", ctx)?;
        SETMSG(
            b"Request time # is outside of descriptor bounds # : #.",
            ctx,
        );
        ERRDP(b"#", ET, ctx);
        ERRDP(b"#", DC[1], ctx);
        ERRDP(b"#", DC[2], ctx);
        SIGERR(b"SPICE(TIMEOUTOFBOUNDS)", ctx)?;
        CHKOUT(b"SPKR09", ctx)?;
        return Ok(());
    }
    //
    // From this point onward, we assume the segment was constructed
    // correctly.  In particular, we assume:
    //
    //    1)  The first and last epochs in the segment define a time
    //        interval that contains the interval defined by the segment
    //        descriptor's time bounds.
    //
    //    2)  The segment descriptor's time bounds are in order and are
    //        distinct.
    //
    //    3)  The epochs in the segment are in strictly increasing
    //        order.
    //
    //    4)  The degree of the interpolating polynomial specified by
    //        the segment is at least 1 and is no larger than
    //
    //           ( L - 1 ) / 7         [integer division]
    //
    //        where L is the declared length of the argument RECORD.
    //
    //    5)  There are at least as many epochs in the segment as the
    //        the number of points required to define an interpolating
    //        polynomial of the specified degree.
    //
    //
    // We'll need the last two items before we can determine which
    // states make up our output record.
    //
    //
    DAFGDA(HANDLE, (END - 1), END, CONTRL.as_slice_mut(), ctx)?;

    DEGREE = intrinsics::IDNINT(CONTRL[1]);
    N = intrinsics::IDNINT(CONTRL[2]);

    WNDSIZ = (DEGREE + 1);

    //
    // We'll now select the set of states that define the interpolating
    // polynomials.   We'll start out by finding the first directory
    // entry that is greater than or equal to the request epoch.  We'll
    // use the variable GROUP to indicate the set of epochs to search
    // within, once we've found the right directory entry.
    //
    NDIR = ((N - 1) / DIRSIZ);
    DIRBAS = ((END - NDIR) - 2);

    if (NDIR == 0) {
        //
        // There's no mystery about which group of epochs to search.
        //
        GROUP = 1;
    } else {
        //
        // There's at least one directory.  Find the first directory
        // whose time is greater than or equal to the request time, if
        // there is such a directory.  We'll search linearly through the
        // directory entries, reading up to BUFSIZ of them at a time.
        // Having found the correct set of directory entries, we'll
        // perform a binary search within that set for the desired entry.
        //
        BUFBAS = DIRBAS;
        NREAD = intrinsics::MIN0(&[NDIR, BUFSIZ]);
        REMAIN = (NDIR - NREAD);

        DAFGDA(
            HANDLE,
            (BUFBAS + 1),
            (BUFBAS + NREAD),
            BUFFER.as_slice_mut(),
            ctx,
        )?;

        while ((BUFFER[NREAD] < ET) && (REMAIN > 0)) {
            BUFBAS = (BUFBAS + NREAD);
            NREAD = intrinsics::MIN0(&[REMAIN, BUFSIZ]);
            REMAIN = (REMAIN - NREAD);
            //
            // Note:  NREAD is always > 0 here.
            //
            DAFGDA(
                HANDLE,
                (BUFBAS + 1),
                (BUFBAS + NREAD),
                BUFFER.as_slice_mut(),
                ctx,
            )?;
        }

        //
        // At this point, BUFBAS - DIRBAS is the number of directory
        // entries preceding the one contained in BUFFER(1).
        //
        GROUP = (((BUFBAS - DIRBAS) + LSTLTD(ET, NREAD, BUFFER.as_slice())) + 1);
    }

    //
    // GROUP now indicates the set of epochs in which to search for the
    // request epoch.  If GROUP is 1, the request time lies within the
    // inclusive time interval bounded by the first and last epochs of
    // the first group.  Otherwise, the request time lies in the time
    // interval bounded by the last element of the preceding group and
    // the last element of the current group.
    //
    // We'll use the variable names BEGIDX and ENDIDX to refer to
    // the indices, relative to the set of time tags, of the first
    // and last time tags in the set we're going to look up.
    //
    if (GROUP == 1) {
        BEGIDX = 1;
        ENDIDX = intrinsics::MIN0(&[N, DIRSIZ]);
    } else {
        //
        // If the group index is greater than 1, we'll include the last
        // time tag of the previous group in the set of time tags we look
        // up.  That way, the request time is bracketed by the time tag
        // set we look up.
        //
        BEGIDX = ((GROUP - 1) * DIRSIZ);
        ENDIDX = intrinsics::MIN0(&[(BEGIDX + DIRSIZ), N]);
    }

    TIMBAS = (DIRBAS - N);

    DAFGDA(
        HANDLE,
        (TIMBAS + BEGIDX),
        (TIMBAS + ENDIDX),
        BUFFER.as_slice_mut(),
        ctx,
    )?;

    //
    // Find two adjacent epochs bounding the request epoch.  The request
    // time cannot be greater than all of epochs in the group, and it
    // cannot precede the first element of the group.
    //
    I = LSTLTD(ET, ((ENDIDX - BEGIDX) + 1), BUFFER.as_slice());

    //
    // The variables LOW and high are the indices of a pair of time
    // tags that bracket the request time.
    //
    if (I == 0) {
        LOW = 1;
    } else {
        LOW = ((BEGIDX + I) - 1);
    }

    HIGH = (LOW + 1);

    //
    // Now select the set of states used for interpolation.
    //
    if ODD(WNDSIZ) {
        //
        // Find the index of the state whose epoch is closest to the
        // input epoch.  The index I is in the range [0, DIRSIZ],
        // since ENDIDX - BEGIDX never exceeds DIRSIZ, and ET is
        // never larger than the (ENDIDX-BEGIDX+1)th element of the
        // buffer.
        //
        if (I == 0) {
            //
            // This can happen only if the request time matches the
            // first time tag of the segment.
            //
            NEAR = LOW;
        } else if (f64::abs((ET - BUFFER[I])) < f64::abs((ET - BUFFER[(I + 1)]))) {
            NEAR = LOW;
        } else {
            NEAR = HIGH;
        }

        //
        // The epochs whose index is NEAR is the (WNDSIZ/2 + 1)th
        // of the interpolating set, unless the request time is too close
        // to the end of the coverage interval, in which case one endpoint
        // of the window will coincide with an endpoint of the coverage
        // interval.
        //
        FIRST = intrinsics::MIN0(&[intrinsics::MAX0(&[(NEAR - (DEGREE / 2)), 1]), (N - DEGREE)]);

        LAST = (FIRST + DEGREE);
    } else {
        //
        // The group size is even.
        //
        // The bracketing epochs we've found are the (WNDSIZ/2)th
        // and (WNDSIZ/2 + 1)th of the interpolating set, unless the
        // request time is too close to the end of the coverage interval,
        // in which case one endpoint of the window will coincide with
        // an endpoint of the coverage interval.
        //
        FIRST = intrinsics::MIN0(&[intrinsics::MAX0(&[(LOW - (DEGREE / 2)), 1]), (N - DEGREE)]);

        LAST = (FIRST + DEGREE);
    }

    //
    // Put the size of the group of states into the output record.
    //
    RECORD[1] = WNDSIZ as f64;

    //
    // Read the states.
    //
    DAFGDA(
        HANDLE,
        (BEGIN + ((FIRST - 1) * STATSZ)),
        ((BEGIN + (LAST * STATSZ)) - 1),
        RECORD.subarray_mut(2),
        ctx,
    )?;

    //
    // Finally, add the epochs to the output record.
    //
    START = (((BEGIN + (N * STATSZ)) + FIRST) - 2);

    DAFGDA(
        HANDLE,
        (START + 1),
        (START + WNDSIZ),
        RECORD.subarray_mut((2 + (WNDSIZ * STATSZ))),
        ctx,
    )?;

    Ok(())
}
