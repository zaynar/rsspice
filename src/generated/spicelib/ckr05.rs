//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const C05TP0: i32 = 0;
const C05TP1: i32 = (C05TP0 + 1);
const C05TP2: i32 = (C05TP1 + 1);
const C05TP3: i32 = (C05TP2 + 1);
const C05PS0: i32 = 8;
const C05PS1: i32 = 4;
const C05PS2: i32 = 14;
const C05PS3: i32 = 7;
const QSIZ: i32 = 4;
const QAVSIZ: i32 = 7;
const CK1DTP: i32 = 1;
const CK1RSZ: i32 = 8;
const CK2DTP: i32 = 2;
const CK2RSZ: i32 = 10;
const CK3DTP: i32 = 3;
const CK3RSZ: i32 = 17;
const CK4DTP: i32 = 4;
const CK4PCD: f64 = 128.0;
const CK4MXD: i32 = 18;
const CK4SFT: i32 = 10;
const CK4RSZ: i32 = (((CK4MXD + 1) * QAVSIZ) + CK4SFT);
const CK5DTP: i32 = 5;
const CK5MXD: i32 = 23;
const CK5MET: i32 = 4;
const CK5MXP: i32 = 14;
const CK5RSZ: i32 = (((CK5MXD + 1) * CK5MXP) + CK5MET);
const CK6DTP: i32 = 6;
const CK6MXD: i32 = 23;
const CK6MET: i32 = 4;
const CK6PS3: i32 = 7;
const CK6RSZ: i32 = (((CK6MXD + 1) * (CK6PS3 + 1)) + CK6MET);
const CKMRSZ: i32 = CK5RSZ;
const ND: i32 = 2;
const NI: i32 = 6;
const DIRSIZ: i32 = 100;
const PBUFSZ: i32 = (DIRSIZ + 1);
const CTRLSZ: i32 = 5;
const PARSIZ: i32 = 4;
const SBUFSZ: i32 = (DIRSIZ + 3);
const MAXDEG: i32 = 23;

struct SaveVars {
    PREVN: f64,
    PREVNN: f64,
    PREVS: f64,
    LBEG: i32,
    LEND: i32,
    LHAND: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PREVN: f64 = 0.0;
        let mut PREVNN: f64 = 0.0;
        let mut PREVS: f64 = 0.0;
        let mut LBEG: i32 = 0;
        let mut LEND: i32 = 0;
        let mut LHAND: i32 = 0;

        LBEG = -1;
        LEND = -1;
        LHAND = 0;
        PREVN = -1.0;
        PREVNN = -1.0;
        PREVS = -1.0;

        Self {
            PREVN,
            PREVNN,
            PREVS,
            LBEG,
            LEND,
            LHAND,
        }
    }
}

/// Read CK record from segment, type 05
///
/// Read a single CK data record from a segment of type 05
/// (MEX/Rosetta Attitude file interpolation).
///
/// # Required Reading
///
/// * [CK](crate::required_reading::ck)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   File handle.
///  DESCR      I   Segment descriptor.
///  SCLKDP     I   Pointing request time.
///  TOL        I   Lookup tolerance.
///  NEEDAV     I   Angular velocity flag.
///  RECORD     O   Data record.
///  FOUND      O   Flag indicating whether record was found.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE,
///  DESCR    are the file handle and segment descriptor for
///           a CK segment of type 05.
///
///  SCLKDP   is an encoded spacecraft clock time indicating
///           the epoch for which pointing is desired.
///
///  TOL      is a time tolerance, measured in the same units as
///           encoded spacecraft clock.
///
///           When SCLKDP falls within the bounds of one of the
///           interpolation intervals then the tolerance has no
///           effect because pointing will be returned at the
///           request time.
///
///           However, if the request time is not in one of the
///           intervals, then the tolerance is used to determine
///           if pointing at one of the interval endpoints should
///           be returned.
///
///  NEEDAV   is .TRUE. if angular velocity is requested.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RECORD   is a set of data from the specified segment which,
///           when evaluated at epoch SCLKDP, will give the
///           attitude and angular velocity of some body, relative
///           to the reference frame indicated by DESCR.
///
///           The structure of the record is as follows:
///
///              +----------------------+
///              | evaluation epoch     |
///              +----------------------+
///              | subtype code         |
///              +----------------------+
///              | number of packets (n)|
///              +----------------------+
///              | nominal SCLK rate    |
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
///  This routine follows the pattern established in the lower-numbered
///  CK data type readers of not explicitly performing error
///  diagnoses. Exceptions are listed below nonetheless.
///
///  1)  If the input HANDLE does not designate a loaded CK file, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  2)  If the segment specified by DESCR is not of data type 05,
///      the error SPICE(WRONGCKTYPE) is signaled.
///
///  3)  If the input SCLK value is not within the range specified
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
///  7)  If the tolerance is negative, the error SPICE(VALUEOUTOFRANGE)
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
///  See the CK Required Reading file for a description of the
///  structure of a data type 05 segment.
/// ```
///
/// # Examples
///
/// ```text
///  The data returned by the CKRnn routine is in its rawest form,
///  taken directly from the segment. As such, it will be meaningless
///  to a user unless he/she understands the structure of the data type
///  completely. Given that understanding, however, the CKRxx
///  routines might be used to "dump" and check segment data for a
///  particular epoch.
///
///  The search performed here does not mimic the behavior of the CK
///  reader APIs CKGP and CKGPAV, which continue searching when an
///  applicable segment doesn't satisfy a pointing request. See the CK
///  Required reading for details.
///
///  C
///  C     Get a segment applicable to a specified body and epoch.
///  C
///        CALL CKBSS ( INST,   SCLKDP, TOL,   NEEDAV )
///        CALL CKSNS ( HANDLE, DESCR,  SEGID, SFND   )
///
///        IF ( .NOT. SFND ) THEN
///           [Handle case of pointing not being found]
///        END IF
///
///  C
///  C     Look at parts of the descriptor.
///  C
///        CALL DAFUS ( DESCR, 2, 6, DCD, ICD )
///        CENTER = ICD( 2 )
///        REF    = ICD( 3 )
///        TYPE   = ICD( 4 )
///
///        IF ( TYPE .EQ. 05 ) THEN
///
///           CALL CKR05 ( HANDLE, DESCR, SCLKDP, TOL, NEEDAV,
///       .                RECORD, FOUND                       )
///
///           IF ( .NOT. FOUND ) THEN
///              [Handle case of pointing not being found]
///           END IF
///
///           [Look at the RECORD data]
///               .
///               .
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
/// -    SPICELIB Version 2.0.1, 06-JUL-2021 (NJB) (JDR)
///
///         Corrected code example: removed comment character preceding
///         CKBSS call. Added note regarding difference between this
///         search and those performed by the CK reader APIs CKGP and
///         CKGPAV.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.0.0, 27-JAN-2014 (NJB)
///
///         Increased MAXDEG to 23 for compatibility with CK type 6.
///
/// -    SPICELIB Version 1.1.0, 06-SEP-2002 (NJB)
/// ```
pub fn ckr05(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64; 5],
    sclkdp: f64,
    tol: f64,
    needav: bool,
    record: &mut [f64],
    found: &mut bool,
) -> crate::Result<()> {
    CKR05(
        handle,
        descr,
        sclkdp,
        tol,
        needav,
        record,
        found,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CKR05 ( Read CK record from segment, type 05 )
pub fn CKR05(
    HANDLE: i32,
    DESCR: &[f64],
    SCLKDP: f64,
    TOL: f64,
    NEEDAV: bool,
    RECORD: &mut [f64],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DESCR = DummyArray::new(DESCR, 1..=5);
    let mut RECORD = DummyArrayMut::new(RECORD, 1..);
    let mut CONTRL = StackArray::<f64, 5>::new(1..=CTRLSZ);
    let mut DC = StackArray::<f64, 2>::new(1..=ND);
    let mut HEPOCH: f64 = 0.0;
    let mut LEPOCH: f64 = 0.0;
    let mut NSTART: f64 = 0.0;
    let mut NNSTRT: f64 = 0.0;
    let mut PBUFFR = StackArray::<f64, 101>::new(1..=PBUFSZ);
    let mut RATE: f64 = 0.0;
    let mut SBUFFR = StackArray::<f64, 103>::new(1..=SBUFSZ);
    let mut START: f64 = 0.0;
    let mut T: f64 = 0.0;
    let mut PBEGIX: i32 = 0;
    let mut BEGIN: i32 = 0;
    let mut BUFBAS: i32 = 0;
    let mut DIRBAS: i32 = 0;
    let mut END: i32 = 0;
    let mut PENDIX: i32 = 0;
    let mut FIRST: i32 = 0;
    let mut HIGH: i32 = 0;
    let mut I: i32 = 0;
    let mut IC = StackArray::<i32, 6>::new(1..=NI);
    let mut J: i32 = 0;
    let mut LAST: i32 = 0;
    let mut LSIZE: i32 = 0;
    let mut LOW: i32 = 0;
    let mut MAXWND: i32 = 0;
    let mut N: i32 = 0;
    let mut NIDIR: i32 = 0;
    let mut NPDIR: i32 = 0;
    let mut NINTS: i32 = 0;
    let mut NPREAD: i32 = 0;
    let mut NSRCH: i32 = 0;
    let mut NSREAD: i32 = 0;
    let mut PACKSZ: i32 = 0;
    let mut PGROUP: i32 = 0;
    let mut REMAIN: i32 = 0;
    let mut RSIZE: i32 = 0;
    let mut SBEGIX: i32 = 0;
    let mut SENDIX: i32 = 0;
    let mut SGROUP: i32 = 0;
    let mut SUBTYP: i32 = 0;
    let mut TIMBAS: i32 = 0;
    let mut TYPE: i32 = 0;
    let mut WNDSIZ: i32 = 0;
    let mut WSTART: i32 = 0;

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

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"CKR05", ctx)?;

    //
    // No pointing found so far.
    //
    *FOUND = false;

    //
    // Unpack the segment descriptor, and get the start and end addresses
    // of the segment.
    //
    DAFUS(DESCR.as_slice(), 2, 6, DC.as_slice_mut(), IC.as_slice_mut());

    TYPE = IC[3];
    BEGIN = IC[5];
    END = IC[6];

    //
    // Make sure that this really is a type 05 data segment.
    //
    if (TYPE != 5) {
        SETMSG(
            b"You are attempting to locate type * data in a type 5 data segment.",
            ctx,
        );
        ERRINT(b"*", TYPE, ctx);
        SIGERR(b"SPICE(WRONGCKTYPE)", ctx)?;
        CHKOUT(b"CKR05", ctx)?;
        return Ok(());
    }

    //
    // Check the tolerance value.
    //
    if (TOL < 0.0) {
        SETMSG(b"Tolerance must be non-negative but was actually *.", ctx);
        ERRDP(b"*", TOL, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"CKR05", ctx)?;
        return Ok(());
    }

    //
    // Check the request time and tolerance against the bounds in
    // the segment descriptor.
    //
    if (((SCLKDP + TOL) < DC[1]) || ((SCLKDP - TOL) > DC[2])) {
        //
        // The request time is too far outside the segment's coverage
        // interval for any pointing to satisfy the request.
        //
        CHKOUT(b"CKR05", ctx)?;
        return Ok(());
    }

    //
    // Set the request time to use for searching.
    //
    T = BRCKTD(SCLKDP, DC[1], DC[2]);

    //
    // From this point onward, we assume the segment was constructed
    // correctly.  In particular, we assume:
    //
    //    1)  The segment descriptor's time bounds are in order and are
    //        distinct.
    //
    //    2)  The epochs in the segment are in strictly increasing
    //        order.
    //
    //
    //    3)  The interpolation interval start times in the segment are
    //        in strictly increasing order.
    //
    //
    //    4)  The degree of the interpolating polynomial specified by
    //        the segment is at least 1 and is no larger than MAXDEG.
    //
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
    // any DAF file and the error action is not set to ABORT.  We
    // do this only after the first call to DAFGDA, as in CKR03.
    //
    if FAILED(ctx) {
        CHKOUT(b"CKR05", ctx)?;
        return Ok(());
    }

    RATE = CONTRL[1];
    SUBTYP = intrinsics::IDNINT(CONTRL[2]);
    WNDSIZ = intrinsics::IDNINT(CONTRL[3]);
    NINTS = intrinsics::IDNINT(CONTRL[4]);
    N = intrinsics::IDNINT(CONTRL[5]);

    //
    // Set the packet size, which is a function of the subtype.
    //
    if (SUBTYP == C05TP0) {
        PACKSZ = C05PS0;
    } else if (SUBTYP == C05TP1) {
        PACKSZ = C05PS1;
    } else if (SUBTYP == C05TP2) {
        PACKSZ = C05PS2;
    } else if (SUBTYP == C05TP3) {
        PACKSZ = C05PS3;
    } else {
        SETMSG(
            b"Unexpected CK type 5 subtype # found in type 5 segment.",
            ctx,
        );
        ERRINT(b"#", SUBTYP, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"CKR05", ctx)?;
        return Ok(());
    }

    //
    // Check the window size.
    //
    if (WNDSIZ <= 0) {
        SETMSG(
            b"Window size in type 05 segment was #; must be positive.",
            ctx,
        );
        ERRINT(b"#", WNDSIZ, ctx);
        SIGERR(b"SPICE(INVALIDVALUE)", ctx)?;
        CHKOUT(b"CKR05", ctx)?;
        return Ok(());
    }

    if ((SUBTYP == C05TP0) || (SUBTYP == C05TP2)) {
        //
        // These are the Hermite subtypes.
        //
        MAXWND = ((MAXDEG + 1) / 2);

        if (WNDSIZ > MAXWND) {
            SETMSG(b"Window size in type 05 segment was #; max allowed value is # for subtypes 0 and 2 (Hermite, 8 or 14-element packets).", ctx);
            ERRINT(b"#", WNDSIZ, ctx);
            ERRINT(b"#", MAXWND, ctx);
            SIGERR(b"SPICE(INVALIDVALUE)", ctx)?;
            CHKOUT(b"CKR05", ctx)?;
            return Ok(());
        }

        if ODD(WNDSIZ) {
            SETMSG(b"Window size in type 05 segment was #; must be even for subtypes 0 and 2 (Hermite, 8 or 14-element packets).", ctx);
            ERRINT(b"#", WNDSIZ, ctx);
            SIGERR(b"SPICE(INVALIDVALUE)", ctx)?;
            CHKOUT(b"CKR05", ctx)?;
            return Ok(());
        }
    } else if ((SUBTYP == C05TP1) || (SUBTYP == C05TP3)) {
        //
        // These are the Lagrange subtypes.
        //
        MAXWND = (MAXDEG + 1);

        if (WNDSIZ > MAXWND) {
            SETMSG(b"Window size in type 05 segment was #; max allowed value is # for subtypes 1 and 3 (Lagrange, 4 or 7-element packets).", ctx);
            ERRINT(b"#", WNDSIZ, ctx);
            ERRINT(b"#", MAXWND, ctx);
            SIGERR(b"SPICE(INVALIDVALUE)", ctx)?;
            CHKOUT(b"CKR05", ctx)?;
            return Ok(());
        }

        if ODD(WNDSIZ) {
            SETMSG(b"Window size in type 05 segment was #; must be even for subtypes 1 and 3 (Lagrange, 4 or 7-element packets).", ctx);
            ERRINT(b"#", WNDSIZ, ctx);
            SIGERR(b"SPICE(INVALIDVALUE)", ctx)?;
            CHKOUT(b"CKR05", ctx)?;
            return Ok(());
        }
    } else {
        SETMSG(b"This point should not be reached. Getting here may indicate that the code needs to updated to handle the new subtype #", ctx);
        ERRINT(b"#", SUBTYP, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"CKR05", ctx)?;
        return Ok(());
    }

    //
    // We now need to select the pointing values to interpolate
    // in order to satisfy the pointing request.  The first step
    // is to use the pointing directories (if any) to locate a set of
    // epochs bracketing the request time.  Note that the request
    // time might not be bracketed:  it could precede the first
    // epoch or follow the last epoch.
    //
    // We'll use the variable PGROUP to refer to the set of epochs
    // to search.  The first group consists of the epochs prior to
    // and including the first pointing directory entry.  The last
    // group consists of the epochs following the last pointing
    // directory entry.  Other groups consist of epochs following
    // one pointing directory entry up to and including the next
    // pointing directory entry.
    //

    NPDIR = ((N - 1) / DIRSIZ);
    DIRBAS = (((BEGIN + (N * PACKSZ)) + N) - 1);

    if (NPDIR == 0) {
        //
        // There's no mystery about which group of epochs to search.
        //
        PGROUP = 1;
    } else {
        //
        // There's at least one directory.  Find the first directory
        // whose time is greater than or equal to the request time, if
        // there is such a directory.  We'll search linearly through the
        // directory entries, reading up to DIRSIZ of them at a time.
        // Having found the correct set of directory entries, we'll
        // perform a binary search within that set for the desired entry.
        //
        BUFBAS = DIRBAS;
        NPREAD = intrinsics::MIN0(&[NPDIR, DIRSIZ]);

        DAFGDA(
            HANDLE,
            (BUFBAS + 1),
            (BUFBAS + NPREAD),
            PBUFFR.as_slice_mut(),
            ctx,
        )?;

        REMAIN = (NPDIR - NPREAD);

        while ((PBUFFR[NPREAD] < T) && (REMAIN > 0)) {
            BUFBAS = (BUFBAS + NPREAD);
            NPREAD = intrinsics::MIN0(&[REMAIN, DIRSIZ]);
            //
            // Note:  NPREAD is always > 0 here.
            //
            DAFGDA(
                HANDLE,
                (BUFBAS + 1),
                (BUFBAS + NPREAD),
                PBUFFR.as_slice_mut(),
                ctx,
            )?;

            REMAIN = (REMAIN - NPREAD);
        }

        //
        // At this point, BUFBAS - DIRBAS is the number of directory
        // entries preceding the one contained in PBUFFR(1).
        //
        // PGROUP is one more than the number of directories we've
        // passed by.
        //
        PGROUP = (((BUFBAS - DIRBAS) + LSTLTD(T, NPREAD, PBUFFR.as_slice())) + 1);
    }

    //
    // PGROUP now indicates the set of epochs in which to search for the
    // request epoch.  The following cases can occur:
    //
    //    PGROUP = 1
    //    ==========
    //
    //       NPDIR = 0
    //       --------
    //       The request time may precede the first time tag
    //       of the segment, exceed the last time tag, or lie
    //       in the closed interval bounded by these time tags.
    //
    //       NPDIR >= 1
    //       ---------
    //       The request time may precede the first time tag
    //       of the group but does not exceed the last epoch
    //       of the group.
    //
    //
    //    1 < PGROUP <= NPDIR
    //    ===================
    //
    //       The request time follows the last time of the
    //       previous group and is less than or equal to
    //       the pointing directory entry at index PGROUP.
    //
    //    1 < PGROUP = NPDIR + 1
    //    ======================
    //
    //       The request time follows the last time of the
    //       last pointing directory entry.  The request time
    //       may exceed the last time tag.
    //
    //
    // Now we'll look up the time tags in the group of epochs
    // we've identified.
    //
    // We'll use the variable names PBEGIX and PENDIX to refer to
    // the indices, relative to the set of time tags, of the first
    // and last time tags in the set we're going to look up.
    //
    if (PGROUP == 1) {
        PBEGIX = 1;
        PENDIX = intrinsics::MIN0(&[N, DIRSIZ]);
    } else {
        //
        // If the group index is greater than 1, we'll include the last
        // time tag of the previous group in the set of time tags we look
        // up.  That way, the request time is strictly bracketed on the
        // low side by the time tag set we look up.
        //
        PBEGIX = ((PGROUP - 1) * DIRSIZ);
        PENDIX = intrinsics::MIN0(&[(PBEGIX + DIRSIZ), N]);
    }

    TIMBAS = (DIRBAS - N);

    DAFGDA(
        HANDLE,
        (TIMBAS + PBEGIX),
        (TIMBAS + PENDIX),
        PBUFFR.as_slice_mut(),
        ctx,
    )?;

    NPREAD = ((PENDIX - PBEGIX) + 1);

    //
    // At this point, we'll deal with the cases where T lies outside
    // of the range of epochs we've buffered.
    //
    if (T < PBUFFR[1]) {
        //
        // This can happen only if PGROUP = 1 and T precedes all epochs.
        // If the input request time is too far from PBUFFR(1) on
        // the low side, we're done.
        //
        if ((SCLKDP + TOL) < PBUFFR[1]) {
            CHKOUT(b"CKR05", ctx)?;
            return Ok(());
        }

        //
        // Bracket T to move it within the range of buffered epochs.
        //
        T = PBUFFR[1];
    } else if (T > PBUFFR[NPREAD]) {
        //
        // This can happen only if T follows all epochs.
        //
        if ((SCLKDP - TOL) > PBUFFR[NPREAD]) {
            CHKOUT(b"CKR05", ctx)?;
            return Ok(());
        }

        //
        // Bracket T to move it within the range of buffered epochs.
        //
        T = PBUFFR[NPREAD];
    }

    //
    // At this point,
    //
    //    | T - SCLKDP |  <=  TOL
    //
    // Also, one of the following is true:
    //
    //    T is the first time of the segment
    //
    //    T is the last time of the segment
    //
    //    T equals SCLKDP
    //
    //
    //
    // Find two adjacent time tags bounding the request epoch.  The
    // request time cannot be greater than all of time tags in the
    // group, and it cannot precede the first element of the group.
    //
    I = LSTLTD(T, NPREAD, PBUFFR.as_slice());

    //
    // The variables LOW and HIGH are the indices of a pair of time
    // tags that bracket the request time.  Remember that NPREAD could
    // be equal to 1, in which case we would have LOW = HIGH.
    //
    if (I == 0) {
        //
        // This can happen only if PGROUP = 1 and T = PBUFFR(1).
        //
        LOW = 1;
        LEPOCH = PBUFFR[1];

        if (N == 1) {
            HIGH = 1;
        } else {
            HIGH = 2;
        }

        HEPOCH = PBUFFR[HIGH];
    } else {
        LOW = ((PBEGIX + I) - 1);
        LEPOCH = PBUFFR[I];

        HIGH = (LOW + 1);
        HEPOCH = PBUFFR[(I + 1)];
    }

    //
    // We now need to find the interpolation interval containing
    // T, if any.  We may be able to use the interpolation
    // interval found on the previous call to this routine.  If
    // this is the first call or if the previous interval is not
    // applicable, we'll search for the interval.
    //
    // First check if the request time falls in the same interval as
    // it did last time.  We need to make sure that we are dealing
    // with the same segment as well as the same time range.
    //
    //
    //    PREVS      is the start time of the interval that satisfied
    //               the previous request for pointing.
    //
    //    PREVN      is the start time of the interval that followed
    //               the interval specified above.
    //
    //    PREVNN     is the start time of the interval that followed
    //               the interval starting at PREVN.
    //
    //    LHAND      is the handle of the file that PREVS and PREVN
    //               were found in.
    //
    //    LBEG,      are the beginning and ending addresses of the
    //    LEND       segment in the file LHAND that PREVS and PREVN
    //               were found in.
    //
    if (((((HANDLE == save.LHAND) && (BEGIN == save.LBEG)) && (END == save.LEND))
        && (T >= save.PREVS))
        && (T < save.PREVN))
    {
        START = save.PREVS;
        NSTART = save.PREVN;
        NNSTRT = save.PREVNN;
    } else {
        //
        // Search for the interpolation interval.
        //
        NIDIR = ((NINTS - 1) / DIRSIZ);
        DIRBAS = ((END - CTRLSZ) - NIDIR);

        if (NIDIR == 0) {
            //
            // There's no mystery about which group of epochs to search.
            //
            SGROUP = 1;
        } else {
            //
            // There's at least one directory.  Find the first directory
            // whose time is greater than or equal to the request time, if
            // there is such a directory.  We'll search linearly through
            // the directory entries, reading up to DIRSIZ of them at a
            // time. Having found the correct set of directory entries,
            // we'll perform a binary search within that set for the
            // desired entry.
            //
            BUFBAS = DIRBAS;
            NSREAD = intrinsics::MIN0(&[NIDIR, DIRSIZ]);
            REMAIN = (NIDIR - NSREAD);

            DAFGDA(
                HANDLE,
                (BUFBAS + 1),
                (BUFBAS + NSREAD),
                SBUFFR.as_slice_mut(),
                ctx,
            )?;

            while ((SBUFFR[NSREAD] < T) && (REMAIN > 0)) {
                BUFBAS = (BUFBAS + NSREAD);
                NSREAD = intrinsics::MIN0(&[REMAIN, DIRSIZ]);
                REMAIN = (REMAIN - NSREAD);
                //
                // Note:  NSREAD is always > 0 here.
                //
                DAFGDA(
                    HANDLE,
                    (BUFBAS + 1),
                    (BUFBAS + NSREAD),
                    SBUFFR.as_slice_mut(),
                    ctx,
                )?;
            }

            //
            // At this point, BUFBAS - DIRBAS is the number of directory
            // entries preceding the one contained in SBUFFR(1).
            //
            // SGROUP is one more than the number of directories we've
            // passed by.
            //
            SGROUP = (((BUFBAS - DIRBAS) + LSTLTD(T, NSREAD, SBUFFR.as_slice())) + 1);
        }

        //
        // SGROUP now indicates the set of interval start times in which
        // to search for the request epoch.
        //
        // Now we'll look up the time tags in the group of epochs we've
        // identified.
        //
        // We'll use the variable names SBEGIX and SENDIX to refer to the
        // indices, relative to the set of start times, of the first and
        // last start times in the set we're going to look up.
        //
        if (SGROUP == 1) {
            SBEGIX = 1;
            SENDIX = intrinsics::MIN0(&[NINTS, (DIRSIZ + 2)]);
        } else {
            //
            // Look up the start times for the group of interest. Also
            // buffer last start time from the previous group. Also, it
            // turns out to be useful to pick up two extra start
            // times---the first two start times of the next group---if
            // they exist.
            //
            SBEGIX = ((SGROUP - 1) * DIRSIZ);

            SENDIX = intrinsics::MIN0(&[((SBEGIX + DIRSIZ) + 2), NINTS]);
        }

        TIMBAS = (DIRBAS - NINTS);

        DAFGDA(
            HANDLE,
            (TIMBAS + SBEGIX),
            (TIMBAS + SENDIX),
            SBUFFR.as_slice_mut(),
            ctx,
        )?;

        NSREAD = ((SENDIX - SBEGIX) + 1);

        //
        // Find the last interval start time less than or equal to the
        // request time.  We know T is greater than or equal to the
        // first start time, so I will be > 0.
        //
        NSRCH = intrinsics::MIN0(&[(DIRSIZ + 1), NSREAD]);

        I = LSTLED(T, NSRCH, SBUFFR.as_slice());

        START = SBUFFR[I];

        //
        // Let NSTART ("next start") be the start time that follows
        // START, if START is not the last start time.  If NSTART
        // has a successor, let NNSTRT be that start time.
        //
        if (I < NSREAD) {
            NSTART = SBUFFR[(I + 1)];

            if ((I + 1) < NSREAD) {
                NNSTRT = SBUFFR[(I + 2)];
            } else {
                NNSTRT = DPMAX();
            }
        } else {
            NSTART = DPMAX();
            NNSTRT = DPMAX();
        }
    }

    //
    // If T does not lie within the interpolation interval starting
    // at time START, we'll determine whether T is closer to this
    // interval or the next.  If the distance between T and the
    // closer interval is less than or equal to TOL, we'll map T
    // to the closer endpoint of the closer interval.  Otherwise,
    // we return without finding pointing.
    //
    if (HEPOCH == NSTART) {
        //
        // The first time tag greater than or equal to T is the start
        // time of the next interpolation interval.
        //
        // The request time lies between interpolation intervals.
        // LEPOCH is the last time tag of the first interval; HEPOCH
        // is the first time tag of the next interval.
        //
        if (f64::abs((T - LEPOCH)) <= f64::abs((HEPOCH - T))) {
            //
            // T is closer to the first interval...
            //
            if (f64::abs((T - LEPOCH)) > TOL) {
                //
                // ...But T is too far from the interval.
                //
                CHKOUT(b"CKR05", ctx)?;
                return Ok(());
            }

            //
            // Map T to the right endpoint of the preceding interval.
            //
            T = LEPOCH;
            HIGH = LOW;
            HEPOCH = LEPOCH;
        } else {
            //
            // T is closer to the second interval...
            //
            if (f64::abs((HEPOCH - T)) > TOL) {
                //
                // ...But T is too far from the interval.
                //
                CHKOUT(b"CKR05", ctx)?;
                return Ok(());
            }

            //
            // Map T to the left endpoint of the next interval.
            //
            T = HEPOCH;
            LOW = HIGH;
            LEPOCH = HEPOCH;

            //
            // Since we're going to be picking time tags from the next
            // interval, we'll need to adjust START and NSTART.
            //
            START = NSTART;
            NSTART = NNSTRT;
        }
    }

    //
    // We now have
    //
    //    LEPOCH < T <  HEPOCH
    //            -   -
    //
    // where LEPOCH and HEPOCH are the time tags at indices
    // LOW and HIGH, respectively.
    //
    // Now select the set of packets used for interpolation.  Note
    // that the window size is known to be even.
    //
    // Unlike CK types 8, 9, 12, and 13, for type 05 we adjust
    // the window size to keep the request time within the central
    // interval of the window.
    //
    // The nominal bracketing epochs we've found are the (WNDSIZ/2)nd
    // and (WNDSIZ/2 + 1)st of the interpolating set.  If the request
    // time is too close to one end of the interpolation interval, we
    // reduce the window size, after which one endpoint of the window
    // will coincide with an endpoint of the interpolation interval.
    //
    // We start out by looking up the set of time tags we'd use
    // if there were no gaps in the coverage.  We then trim our
    // time tag set to ensure all tags are in the interpolation
    // interval.  It's possible that the interpolation window will
    // collapse to a single point as a result of this last step.
    //
    // Let LSIZE be the size of the "left half" of the window:  the
    // size of the set of window epochs to the left of the request time.
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
    // Buffer the epochs.
    //
    WSTART = (((BEGIN + (N * PACKSZ)) + FIRST) - 1);

    DAFGDA(
        HANDLE,
        WSTART,
        ((WSTART + WNDSIZ) - 1),
        PBUFFR.as_slice_mut(),
        ctx,
    )?;

    //
    // Discard any epochs less than START or greater than or equal
    // to NSTART.  The set of epochs we want ranges from indices
    // I+1 to J.  This range is non-empty unless START and NSTART
    // are both DPMAX().
    //
    I = LSTLTD(START, WNDSIZ, PBUFFR.as_slice());
    J = LSTLTD(NSTART, WNDSIZ, PBUFFR.as_slice());

    if (I == J) {
        //
        // Fuggedaboudit.
        //
        CHKOUT(b"CKR05", ctx)?;
        return Ok(());
    }

    //
    // Update FIRST, LAST, and WNDSIZ.
    //
    WNDSIZ = (J - I);
    FIRST = (FIRST + I);
    LAST = ((FIRST + WNDSIZ) - 1);

    //
    // Put the subtype into the output record.  The size of the group
    // of packets is derived from the subtype, so we need not include
    // the size.
    //
    RECORD[1] = T;
    RECORD[2] = SUBTYP as f64;
    RECORD[3] = WNDSIZ as f64;
    RECORD[4] = RATE;

    //
    // Read the packets.
    //
    DAFGDA(
        HANDLE,
        (BEGIN + ((FIRST - 1) * PACKSZ)),
        ((BEGIN + (LAST * PACKSZ)) - 1),
        RECORD.subarray_mut((PARSIZ + 1)),
        ctx,
    )?;

    //
    // Finally, add the epochs to the output record.
    //
    MOVED(
        PBUFFR.subarray((I + 1)),
        (J - I),
        RECORD.subarray_mut(((PARSIZ + 1) + (WNDSIZ * PACKSZ))),
    );

    //
    // Save the information about the interval and segment.
    //
    save.LHAND = HANDLE;
    save.LBEG = BEGIN;
    save.LEND = END;
    save.PREVS = START;
    save.PREVN = NSTART;
    save.PREVNN = NNSTRT;

    //
    // Indicate pointing was found.
    //
    *FOUND = true;

    CHKOUT(b"CKR05", ctx)?;
    Ok(())
}
