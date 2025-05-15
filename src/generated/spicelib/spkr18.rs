//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const S18TP0: i32 = 0;
const S18TP1: i32 = (S18TP0 + 1);
const S18PS0: i32 = 12;
const S18PS1: i32 = 6;
const ND: i32 = 2;
const NI: i32 = 6;
const DIRSIZ: i32 = 100;
const BUFSIZ: i32 = (DIRSIZ + 1);
const CTRLSZ: i32 = 3;
const MAXDEG: i32 = 15;

/// Read SPK record from segment, type 18
///
/// Read a single SPK data record from a segment of type 18
/// (MEX/Rosetta Orbit file interpolation).
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
///           a SPK segment of type 18.
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
///           to some center, in some reference frame.
///
///           The structure of the record is as follows:
///
///              +----------------------+
///              | subtype code         |
///              +----------------------+
///              | number of packets (n)|
///              +----------------------+
///              | packet 1             |
///              +----------------------+
///              | packet 2             |
///              +----------------------+
///                          .
///                          .
///                          .
///              +----------------------+
///              | packet n             |
///              +----------------------+
///              | epochs 1--n          |
///              +----------------------+
///
///           The packet size is a function of the subtype code.
///           All packets in a record have the same size.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input HANDLE does not designate a loaded SPK file, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  2)  If the segment specified by DESCR is not of data type 18,
///      the error SPICE(WRONGSPKTYPE) is signaled.
///
///  3)  If the input ET value is not within the range specified
///      in the segment descriptor, the error SPICE(TIMEOUTOFBOUNDS)
///      is signaled.
///
///  4)  If the window size is non-positive or greater than the
///      maximum allowed value, the error SPICE(INVALIDVALUE) is
///      signaled.
///
///  5)  If the window size is not compatible with the segment
///      subtype, the error SPICE(INVALIDVALUE) is signaled.
///
///  6)  If the segment subtype is not recognized, the error
///      SPICE(NOTSUPPORTED) is signaled.
///
///  7)  If the input segment contains fewer than 2 packets, the
///      error SPICE(TOOFEWSTATES) is signaled.
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
///  structure of a data type 18 segment.
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
///        IF ( TYPE .EQ. 18 ) THEN
///           CALL SPKR18 ( HANDLE, DESCR, ET, RECORD )
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
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.0.1, 12-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.0.0, 21-DEC-2012 (NJB)
///
///         An error check was added for segment packet counts
///         less than 2.
///
///         An in-line comment regarding deducibility of record size from
///         segment subtype was removed. The comment now says the actual
///         count of packets in the output record is inserted into the
///         record.
///
/// -    SPICELIB Version 1.0.0, 04-SEP-2002 (NJB)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 2.0.0, 21-DEC-2012 (NJB)
///
///         An error check was added for segment packet counts
///         less than 2.
/// ```
pub fn spkr18(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64; 5],
    et: f64,
    record: &mut [f64],
) -> crate::Result<()> {
    SPKR18(handle, descr, et, record, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKR18 ( Read SPK record from segment, type 18 )
pub fn SPKR18(
    HANDLE: i32,
    DESCR: &[f64],
    ET: f64,
    RECORD: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DESCR = DummyArray::new(DESCR, 1..=5);
    let mut RECORD = DummyArrayMut::new(RECORD, 1..);
    let mut BUFFER = StackArray::<f64, 101>::new(1..=BUFSIZ);
    let mut CONTRL = StackArray::<f64, 3>::new(1..=CTRLSZ);
    let mut DC = StackArray::<f64, 2>::new(1..=ND);
    let mut BEGIDX: i32 = 0;
    let mut BEGIN: i32 = 0;
    let mut BUFBAS: i32 = 0;
    let mut DIRBAS: i32 = 0;
    let mut END: i32 = 0;
    let mut ENDIDX: i32 = 0;
    let mut FIRST: i32 = 0;
    let mut GROUP: i32 = 0;
    let mut HIGH: i32 = 0;
    let mut I: i32 = 0;
    let mut IC = StackArray::<i32, 6>::new(1..=NI);
    let mut LAST: i32 = 0;
    let mut LSIZE: i32 = 0;
    let mut LOW: i32 = 0;
    let mut MAXWND: i32 = 0;
    let mut N: i32 = 0;
    let mut NDIR: i32 = 0;
    let mut NREAD: i32 = 0;
    let mut PACKSZ: i32 = 0;
    let mut REMAIN: i32 = 0;
    let mut RSIZE: i32 = 0;
    let mut START: i32 = 0;
    let mut SUBTYP: i32 = 0;
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
    // Maximum polynomial degree:
    //

    //
    // Local variables
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"SPKR18", ctx)?;

    //
    // Unpack the segment descriptor, and get the start and end addresses
    // of the segment.
    //
    DAFUS(DESCR.as_slice(), 2, 6, DC.as_slice_mut(), IC.as_slice_mut());

    TYPE = IC[4];
    BEGIN = IC[5];
    END = IC[6];

    //
    // Make sure that this really is a type 18 data segment.
    //
    if (TYPE != 18) {
        SETMSG(
            b"You are attempting to locate type * data in a type 18 data segment.",
            ctx,
        );
        ERRINT(b"*", TYPE, ctx);
        SIGERR(b"SPICE(WRONGSPKTYPE)", ctx)?;
        CHKOUT(b"SPKR18", ctx)?;
        return Ok(());
    }

    //
    // Check the request time against the bounds in the segment
    // descriptor.
    //
    if ((ET < DC[1]) || (ET > DC[2])) {
        SETMSG(
            b"Request time # is outside of descriptor bounds # : #.",
            ctx,
        );
        ERRDP(b"#", ET, ctx);
        ERRDP(b"#", DC[1], ctx);
        ERRDP(b"#", DC[2], ctx);
        SIGERR(b"SPICE(TIMEOUTOFBOUNDS)", ctx)?;
        CHKOUT(b"SPKR18", ctx)?;
        return Ok(());
    }

    // We'll need the last two items before we can determine which
    // packets make up our output record.
    //
    DAFGDA(
        HANDLE,
        ((END - CTRLSZ) + 1),
        END,
        CONTRL.as_slice_mut(),
        ctx,
    )?;

    //
    // Check the FAILED flag just in case HANDLE is not attached to
    // any DAF file and the error action is not set to ABORT. You need
    // need to do this only after the first call to DAFGDA.
    //
    if FAILED(ctx) {
        CHKOUT(b"SPKR18", ctx)?;
        return Ok(());
    }

    SUBTYP = intrinsics::IDNINT(CONTRL[1]);
    WNDSIZ = intrinsics::IDNINT(CONTRL[2]);
    N = intrinsics::IDNINT(CONTRL[3]);

    if (N < 2) {
        SETMSG(
            b"Packet count # is less than the minimum valid value, which is 2.",
            ctx,
        );
        ERRINT(b"#", N, ctx);
        SIGERR(b"SPICE(TOOFEWSTATES)", ctx)?;
        CHKOUT(b"SPKR18", ctx)?;
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
    //           MAXDEG
    //

    //
    // Set the packet size, which is a function of the subtype.
    //
    if (SUBTYP == S18TP0) {
        PACKSZ = S18PS0;
    } else if (SUBTYP == S18TP1) {
        PACKSZ = S18PS1;
    } else {
        SETMSG(
            b"Unexpected SPK type 18 subtype # found in type 18 segment.",
            ctx,
        );
        ERRINT(b"#", SUBTYP, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"SPKR18", ctx)?;
        return Ok(());
    }

    //
    // Check the window size.
    //
    if (WNDSIZ <= 0) {
        SETMSG(
            b"Window size in type 18 segment was #; must be positive.",
            ctx,
        );
        ERRINT(b"#", SUBTYP, ctx);
        SIGERR(b"SPICE(INVALIDVALUE)", ctx)?;
        CHKOUT(b"SPKR18", ctx)?;
        return Ok(());
    }

    if (SUBTYP == S18TP0) {
        MAXWND = ((MAXDEG + 1) / 2);

        if (WNDSIZ > MAXWND) {
            SETMSG(b"Window size in type 18 segment was #; max allowed value is # for subtype 0 (Hermite, 12-element packets).", ctx);
            ERRINT(b"#", WNDSIZ, ctx);
            ERRINT(b"#", MAXWND, ctx);
            SIGERR(b"SPICE(INVALIDVALUE)", ctx)?;
            CHKOUT(b"SPKR18", ctx)?;
            return Ok(());
        }

        if ODD(WNDSIZ) {
            SETMSG(b"Window size in type 18 segment was #; must be even for subtype 0 (Hermite, 12-element packets).", ctx);
            ERRINT(b"#", WNDSIZ, ctx);
            SIGERR(b"SPICE(INVALIDVALUE)", ctx)?;
            CHKOUT(b"SPKR18", ctx)?;
            return Ok(());
        }
    } else if (SUBTYP == S18TP1) {
        MAXWND = (MAXDEG + 1);

        if (WNDSIZ > MAXWND) {
            SETMSG(b"Window size in type 18 segment was #; max allowed value is # for subtype 1 (Lagrange, 6-element packets).", ctx);
            ERRINT(b"#", WNDSIZ, ctx);
            ERRINT(b"#", MAXWND, ctx);
            SIGERR(b"SPICE(INVALIDVALUE)", ctx)?;
            CHKOUT(b"SPKR18", ctx)?;
            return Ok(());
        }

        if ODD(WNDSIZ) {
            SETMSG(b"Window size in type 18 segment was #; must be even for subtype 1 (Lagrange, 6-element packets).", ctx);
            ERRINT(b"#", WNDSIZ, ctx);
            SIGERR(b"SPICE(INVALIDVALUE)", ctx)?;
            CHKOUT(b"SPKR18", ctx)?;
            return Ok(());
        }
    } else {
        SETMSG(b"This point should not be reached. Getting here may indicate that the code needs to updated to handle new subtypes.", ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"SPKR18", ctx)?;
        return Ok(());
    }

    //
    // We'll now select the set of packets that define the interpolating
    // polynomials.   We'll start out by finding the first directory
    // entry that is greater than or equal to the request epoch.  We'll
    // use the variable GROUP to indicate the set of epochs to search
    // within, once we've found the right directory entry.
    //
    NDIR = ((N - 1) / DIRSIZ);
    DIRBAS = ((END - NDIR) - CTRLSZ);

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
        NREAD = intrinsics::MIN0(&[NDIR, DIRSIZ]);
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
            NREAD = intrinsics::MIN0(&[REMAIN, DIRSIZ]);
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
    // Now select the set of packets used for interpolation.  Note
    // that the window size is known to be even.
    //
    // Unlike SPK types 8, 9, 12, and 13, for type 18 we adjust
    // the window size to keep the request time within the central
    // interval of the window.
    //
    // The nominal bracketing epochs we've found are the (WNDSIZ/2)nd
    // and (WNDSIZ/2 + 1)st of the interpolating set.  If the
    // request time is too close to one end of the coverage interval,
    // we reduce the window size, after which one endpoint of the
    // window will coincide with an endpoint of the coverage interval.
    //
    // Let LSIZE be the size of the "left half" of the window:  the
    // size set of window epochs to the left of the request time.
    // We want this size to be WNDSIZ/2, but if not enough states are
    // available, the set ranges from index 1 to index LOW.
    //

    LSIZE = intrinsics::MIN0(&[(WNDSIZ / 2), LOW]);

    //
    // RSIZE is defined analogously for the right half of the window.
    //
    RSIZE = intrinsics::MIN0(&[(WNDSIZ / 2), ((N - HIGH) + 1)]);

    //
    // The window size is simply the sum of LSIZE and RSIZE.
    //
    WNDSIZ = (LSIZE + RSIZE);

    //
    // FIRST and LAST are the endpoints of the range of indices of
    // time tags (and packets) we'll collect in the output record.
    //
    FIRST = ((LOW - LSIZE) + 1);

    LAST = ((FIRST + WNDSIZ) - 1);

    //
    // Put the subtype and actual window size, which is the number of
    // packets in the record, into the output record.
    //
    RECORD[1] = SUBTYP as f64;

    RECORD[2] = WNDSIZ as f64;

    //
    // Read the packets.
    //
    DAFGDA(
        HANDLE,
        (BEGIN + ((FIRST - 1) * PACKSZ)),
        ((BEGIN + (LAST * PACKSZ)) - 1),
        RECORD.subarray_mut(3),
        ctx,
    )?;

    //
    // Finally, add the epochs to the output record.
    //
    START = (((BEGIN + (N * PACKSZ)) + FIRST) - 2);

    DAFGDA(
        HANDLE,
        (START + 1),
        (START + WNDSIZ),
        RECORD.subarray_mut((3 + (WNDSIZ * PACKSZ))),
        ctx,
    )?;

    CHKOUT(b"SPKR18", ctx)?;
    Ok(())
}
