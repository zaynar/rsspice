//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const MAXDEG: i32 = 27;
const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const S19TP0: i32 = 0;
const S19TP1: i32 = (S19TP0 + 1);
const S19TP2: i32 = (S19TP1 + 1);
const S19PS0: i32 = 12;
const S19PS1: i32 = 6;
const S19PS2: i32 = 6;
const S19NST: i32 = 3;
const S19MXZ: i32 = S19PS0;
const S19MNZ: i32 = S19PS1;
const MAXRSZ: i32 = (2 + ((MAXDEG + 1) * (S19PS1 + 1)));
const MAXREC: i32 = 198;
const SIDLEN: i32 = 40;
const FPRINT: i32 = 32;
const LPRINT: i32 = 126;
const ND: i32 = 2;
const NI: i32 = 6;
const DSCSIZ: i32 = 5;
const DTYPE: i32 = 19;
const DIRSIZ: i32 = 100;

struct SaveVars {
    PKTSZS: StackArray<i32, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PKTSZS = StackArray::<i32, 3>::new(0..=(S19NST - 1));

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(S19PS0), Val::I(S19PS1), Val::I(S19PS2)].into_iter();
            PKTSZS
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { PKTSZS }
    }
}

/// Write SPK segment, type 19
///
/// Write a type 19 segment to an SPK file.
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
/// * [NAIF_IDS](crate::required_reading::naif_ids)
/// * [SPC](crate::required_reading::spc)
/// * [SPK](crate::required_reading::spk)
/// * [TIME](crate::required_reading::time)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of an SPK file open for writing.
///  BODY       I   NAIF ID code for an ephemeris object.
///  CENTER     I   NAIF ID code for center of motion of BODY.
///  FRAME      I   Reference frame name.
///  FIRST      I   Start time of interval covered by segment.
///  LAST       I   End time of interval covered by segment.
///  SEGID      I   Segment identifier.
///  NINTVL     I   Number of mini-segments and interpolation
///                 intervals.
///  NPKTS      I   Array of packet counts of mini-segments.
///  SUBTPS     I   Array of segment subtypes of mini-segments.
///  DEGRES     I   Array of polynomial degrees of mini-segments.
///  PACKTS     I   Array of data packets of mini-segments.
///  EPOCHS     I   Array of epochs of mini-segments.
///  IVLBDS     I   Interpolation interval bounds.
///  SELLST     I   Interval selection flag.
///  MAXDEG     P   Maximum allowed degree of interpolating polynomial.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of an SPK file that has been opened
///           for writing.
///
///  BODY     is the NAIF integer code for an ephemeris object
///           whose state relative to another body is described
///           by the segment to be created.
///
///  CENTER   is the NAIF integer code for the center of motion
///           of the object identified by BODY.
///
///  FRAME    is the NAIF name for a reference frame
///           relative to which the state information for BODY
///           is specified.
///
///  FIRST,
///  LAST     are, respectively, the bounds of the time interval
///           over which the segment defines the state of BODY.
///
///           FIRST must be greater than or equal to the first
///           interpolation interval start time; LAST must be
///           less than or equal to the last interpolation
///           interval stop time. See the description of IVLBDS
///           below.
///
///  SEGID    is the segment identifier. An SPK segment
///           identifier may contain up to 40 characters.
///
///  NINTVL   is the number of interpolation intervals
///           associated with the input data. The interpolation
///           intervals are associated with data sets referred
///           to as "mini-segments."
///
///           The input data comprising each mini-segment are:
///
///              - a packet count
///              - a type 19 subtype
///              - an interpolating polynomial degree
///              - a sequence of type 19 data packets
///              - a sequence of packet epochs
///
///           These inputs are described below.
///
///  NPKTS    is an array of packet counts. The Ith element of
///           NPKTS is the packet count of the Ith interpolation
///           interval/mini-segment.
///
///           NPKTS has dimension NINTVL.
///
///  SUBTPS   is an array of type 19 subtypes. The Ith element
///           of SUBTPS is the subtype of the packets associated
///           with the Ith interpolation interval/mini-segment.
///
///           SUBTPS has dimension NINTVL.
///
///  DEGRES   is an array of interpolating polynomial degrees.
///           The Ith element of DEGRES is the polynomial degree
///           of the packets associated with the Ith
///           interpolation interval/mini-segment.
///
///           For subtype 0, interpolation degrees must be
///           equivalent to 3 mod 4, that is, they must be in
///           the set
///
///              { 3, 7, 11, ..., MAXDEG }
///
///           For subtype 1, interpolation degrees must be odd
///           and must be in the range 1:MAXDEG.
///
///           DEGRES has dimension NINTVL.
///
///  PACKTS   is an array containing data packets for all input
///           mini-segments. The packets for a given
///           mini-segment are stored contiguously in increasing
///           time order. The order of the sets of packets for
///           different mini-segments is the same as the order
///           of their corresponding interpolation intervals.
///
///           Each packet represents geometric states of BODY
///           relative to CENTER, specified relative to FRAME.
///           The packet structure depends on the segment
///           subtype as follows:
///
///              Type 0 (indicated by code S19TP0):
///
///                  x,  y,  z,  dx/dt,  dy/dt,  dz/dt,
///                  vx, vy, vz, dvx/dt, dvy/dt, dvz/dt
///
///              where x, y, z represent Cartesian position
///              components and  vx, vy, vz represent Cartesian
///              velocity components. Note well: vx, vy, and
///              vz *are not necessarily equal* to the time
///              derivatives of x, y, and z. This packet
///              structure mimics that of the Rosetta/MEX orbit
///              file.
///
///              Type 1 (indicated by code S19TP1):
///
///                  x,  y,  z,  dx/dt,  dy/dt,  dz/dt
///
///              where x, y, z represent Cartesian position
///              components and  vx, vy, vz represent Cartesian
///              velocity components.
///
///
///              Type 2 (indicated by code S19TP2):
///
///                  Data are identical to type 1; only the
///                  interpolation algorithm is different.
///
///           Position units are kilometers, velocity units
///           are kilometers per second, and acceleration units
///           are kilometers per second per second.
///
///  EPOCHS   is an array containing epochs for all input
///           mini-segments. Each epoch is expressed as seconds
///           past J2000 TDB. The epochs have a one-to-one
///           relationship with the packets in the input packet
///           array.
///
///           The epochs for a given mini-segment are stored
///           contiguously in increasing order. The order of the
///           sets of epochs for different mini-segments is the
///           same as the order of their corresponding
///           interpolation intervals.
///
///           For each mini-segment, "padding" is allowed: the
///           sequence of epochs for that mini-segment may start
///           before the corresponding interpolation interval
///           start time and end after the corresponding
///           interpolation interval stop time. Padding is used
///           to control behavior of interpolating polynomials
///           near interpolation interval boundaries.
///
///           Due to possible use of padding, the elements of
///           EPOCHS, taken as a whole, may not be in increasing
///           order.
///
///  IVLBDS   is an array of interpolation interval boundary
///           times. This array is an ordered list of the
///           interpolation interval start times, to which the
///           the end time for the last interval is appended.
///
///           The Ith interpolation interval is the time
///           coverage interval of the Ith mini-segment (see the
///           description of NPKTS above).
///
///           For each mini-segment, the corresponding
///           interpolation interval's start time is greater
///           than or equal to the mini-segment's first epoch,
///           and the interval's stop time is less than or equal
///           to the mini-segment's last epoch.
///
///           For each interpolation interval other than the
///           last, the interval's coverage stop time coincides
///           with the coverage start time of the next interval.
///           There are no coverage gaps, and coverage overlap
///           for adjacent intervals consists of a single epoch.
///
///           IVLBDS has dimension NINTVL+1.
///
///  SELLST   is a logical flag indicating to the SPK type 19
///           segment reader SPKR19 how to select the
///           interpolation interval when a request time
///           coincides with a time boundary shared by two
///           interpolation intervals. When SELLST ("select
///           last") is .TRUE., the later interval is selected;
///           otherwise the earlier interval is selected.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None. See $Particulars for a description of the effect of this
///  routine.
/// ```
///
/// # Parameters
///
/// ```text
///  MAXDEG   is the maximum allowed degree of the interpolating
///           polynomial.
///
///           See the INCLUDE file spk19.inc for the value of
///           MAXDEG.
/// ```
///
/// # Exceptions
///
/// ```text
///  If any of the following exceptions occur, this routine will
///  return without creating a new segment.
///
///  1)  If FIRST is greater than LAST, the error
///      SPICE(BADDESCRTIMES) is signaled.
///
///  2)  If FRAME is not a recognized name, the error
///      SPICE(INVALIDREFFRAME) is signaled.
///
///  3)  If the last non-blank character of SEGID occurs past index
///      40, the error SPICE(SEGIDTOOLONG) is signaled.
///
///  4)  If SEGID contains any nonprintable characters, the error
///      SPICE(NONPRINTABLECHARS) is signaled.
///
///  5)  If NINTVL is not at least 1, the error SPICE(INVALIDCOUNT)
///      is signaled.
///
///  6)  If the elements of the array IVLBDS are not in strictly
///      increasing order, the error SPICE(BOUNDSOUTOFORDER) is
///      signaled.
///
///  7)  If the first interval start time IVLBDS(1) is greater than
///      FIRST, or if the last interval end time IVLBDS(N+1) is less
///      than LAST, the error SPICE(COVERAGEGAP) is signaled.
///
///  8)  If any packet count in the array NPKTS is not at least 2, the
///      error SPICE(TOOFEWPACKETS) is signaled.
///
///  9)  If any subtype code in the array SUBTPS is not recognized,
///      the error SPICE(INVALIDSUBTYPE) is signaled.
///
///  10) If any interpolation degree in the array DEGRES
///      is not at least 1 or is greater than MAXDEG, the
///      error SPICE(INVALIDDEGREE) is signaled.
///
///  11) If the window size implied by any element of the array DEGRES
///      is odd, the error SPICE(BADWINDOWSIZE) is signaled.
///
///  12) If the elements of the array EPOCHS corresponding to a given
///      mini-segment are not in strictly increasing order, the error
///      SPICE(TIMESOUTOFORDER) is signaled.
///
///  13) If the first epoch of a mini-segment exceeds the start
///      time of the associated interpolation interval, or if the
///      last epoch of the mini-segment precedes the end time of the
///      interpolation interval, the error SPICE(BOUNDSDISAGREE)
///      is signaled.
///
///  14) If an error occurs while writing the output segment, the error
///      is signaled by a routine in the call tree of this routine.
/// ```
///
/// # Files
///
/// ```text
///  A new type 19 SPK segment is written to the SPK file attached
///  to HANDLE.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine writes an SPK type 19 data segment to the open SPK
///  file according to the format described in the type 19 section of
///  the SPK Required Reading. The SPK file must have been opened with
///  write access.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that you have states and are prepared to produce
///  a segment of type 19 in an SPK file.
///
///  The following code fragment could be used to add the new segment
///  to a previously opened SPK file attached to HANDLE. The file must
///  have been opened with write access.
///
///     C
///     C     Create a segment identifier.
///     C
///               SEGID = 'MY_SAMPLE_SPK_TYPE_19_SEGMENT'
///
///     C
///     C     Write the segment.
///     C
///           CALL SPKW19 ( HANDLE,  BODY,    CENTER,  FRAME,
///          .              FIRST,   LAST,    SEGID,   NINTVL,
///          .              NPKTS,   SUBTPS,  DEGRES,  PACKTS,
///          .              EPOCHS,  IVLBDS,  SELLST           )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.0.1, 03-JUN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.0.0, 21-DEC-2015 (NJB)
///
///         Updated to support subtype 2.
///
/// -    SPICELIB Version 1.0.0, 05-FEB-2014 (NJB) (BVS)
/// ```
pub fn spkw19(
    ctx: &mut SpiceContext,
    handle: i32,
    body: i32,
    center: i32,
    frame: &str,
    first: f64,
    last: f64,
    segid: &str,
    nintvl: i32,
    npkts: &[i32],
    subtps: &[i32],
    degres: &[i32],
    packts: &[f64],
    epochs: &[f64],
    ivlbds: &[f64],
    sellst: bool,
) -> crate::Result<()> {
    SPKW19(
        handle,
        body,
        center,
        frame.as_bytes(),
        first,
        last,
        segid.as_bytes(),
        nintvl,
        npkts,
        subtps,
        degres,
        packts,
        epochs,
        ivlbds,
        sellst,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKW19 ( Write SPK segment, type 19 )
pub fn SPKW19(
    HANDLE: i32,
    BODY: i32,
    CENTER: i32,
    FRAME: &[u8],
    FIRST: f64,
    LAST: f64,
    SEGID: &[u8],
    NINTVL: i32,
    NPKTS: &[i32],
    SUBTPS: &[i32],
    DEGRES: &[i32],
    PACKTS: &[f64],
    EPOCHS: &[f64],
    IVLBDS: &[f64],
    SELLST: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let NPKTS = DummyArray::new(NPKTS, 1..);
    let SUBTPS = DummyArray::new(SUBTPS, 1..);
    let DEGRES = DummyArray::new(DEGRES, 1..);
    let PACKTS = DummyArray::new(PACKTS, 1..);
    let EPOCHS = DummyArray::new(EPOCHS, 1..);
    let IVLBDS = DummyArray::new(IVLBDS, 1..);
    let mut DC = StackArray::<f64, 2>::new(1..=ND);
    let mut DESCR = StackArray::<f64, 5>::new(1..=DSCSIZ);
    let mut BEPIX: i32 = 0;
    let mut CHRCOD: i32 = 0;
    let mut EEPIX: i32 = 0;
    let mut IC = StackArray::<i32, 6>::new(1..=NI);
    let mut ISEL: i32 = 0;
    let mut K: i32 = 0;
    let mut MINISZ: i32 = 0;
    let mut NDIR: i32 = 0;
    let mut PKTBEG: i32 = 0;
    let mut PKTDSZ: i32 = 0;
    let mut PKTEND: i32 = 0;
    let mut PKTSIZ: i32 = 0;
    let mut REFCOD: i32 = 0;
    let mut SEGBEG: i32 = 0;
    let mut SEGEND: i32 = 0;
    let mut SUBTYP: i32 = 0;
    let mut WINSIZ: i32 = 0;

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
    // Saved values
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

    CHKIN(b"SPKW19", ctx)?;

    //
    // Start with a parameter compatibility check.
    //
    if (MAXREC < MAXRSZ) {
        SETMSG(
            b"SPK type 19 record size may be as large as #, but SPKPVN record size is #.",
            ctx,
        );
        ERRINT(b"#", MAXRSZ, ctx);
        ERRINT(b"#", MAXREC, ctx);
        SIGERR(b"SPICE(BUG0)", ctx)?;
        CHKOUT(b"SPKW19", ctx)?;
        return Ok(());
    }

    //
    // Make sure the segment descriptor bounds are
    // correctly ordered.
    //
    if (LAST < FIRST) {
        SETMSG(
            b"Segment start time is #; stop time is #; bounds must be in nondecreasing order.",
            ctx,
        );
        ERRDP(b"#", FIRST, ctx);
        ERRDP(b"#", LAST, ctx);
        SIGERR(b"SPICE(BADDESCRTIMES)", ctx)?;
        CHKOUT(b"SPKW19", ctx)?;
        return Ok(());
    }

    //
    // Get the NAIF integer code for the reference frame.
    //
    NAMFRM(FRAME, &mut REFCOD, ctx)?;

    if (REFCOD == 0) {
        SETMSG(b"The reference frame # is not supported.", ctx);
        ERRCH(b"#", FRAME, ctx);
        SIGERR(b"SPICE(INVALIDREFFRAME)", ctx)?;
        CHKOUT(b"SPKW19", ctx)?;
        return Ok(());
    }

    //
    // Check to see if the segment identifier is too long.
    //
    if (LASTNB(SEGID) > SIDLEN) {
        SETMSG(b"Segment identifier contains more than 40 characters.", ctx);
        SIGERR(b"SPICE(SEGIDTOOLONG)", ctx)?;
        CHKOUT(b"SPKW19", ctx)?;
        return Ok(());
    }

    //
    // Now check that all the characters in the segment identifier
    // can be printed.
    //
    for I in 1..=LASTNB(SEGID) {
        CHRCOD = intrinsics::ICHAR(fstr::substr(SEGID, I..=I));

        if ((CHRCOD < FPRINT) || (CHRCOD > LPRINT)) {
            SETMSG(
                b"The segment identifier contains nonprintable characters",
                ctx,
            );
            SIGERR(b"SPICE(NONPRINTABLECHARS)", ctx)?;
            CHKOUT(b"SPKW19", ctx)?;
            return Ok(());
        }
    }

    //
    // The mini-segment/interval count must be positive.
    //
    if (NINTVL < 1) {
        SETMSG(
            b"Mini-segment/interval count was #; this count must be positive.",
            ctx,
        );
        ERRINT(b"#", NINTVL, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"SPKW19", ctx)?;
        return Ok(());
    }

    //
    // Make sure the interval bounds form a strictly
    // increasing sequence.
    //
    // Note that there are NINTVL+1 bounds.
    //
    for I in 1..=NINTVL {
        if (IVLBDS[I] >= IVLBDS[(I + 1)]) {
            SETMSG(b"Interval bounds at indices # and # are # and # respectively. The difference is #. The bounds are required to be strictly increasing.", ctx);
            ERRINT(b"#", I, ctx);
            ERRINT(b"#", (I + 1), ctx);
            ERRDP(b"#", IVLBDS[I], ctx);
            ERRDP(b"#", IVLBDS[(I + 1)], ctx);
            ERRDP(b"#", (IVLBDS[(I + 1)] - IVLBDS[I]), ctx);
            SIGERR(b"SPICE(BOUNDSOUTOFORDER)", ctx)?;
            CHKOUT(b"SPKW19", ctx)?;
            return Ok(());
        }
    }

    //
    // Make sure the time span of the descriptor doesn't extend
    // beyond the span of the interval bounds.
    //
    if ((FIRST < IVLBDS[1]) || (LAST > IVLBDS[(NINTVL + 1)])) {
        SETMSG(b"First interval start time is #; segment start time is #; segment stop time is #; last interval stop time is #. This sequence of times is required to be non-decreasing: segment coverage must be contained within the union of the interpolation intervals.", ctx);
        ERRDP(b"#", IVLBDS[1], ctx);
        ERRDP(b"#", FIRST, ctx);
        ERRDP(b"#", LAST, ctx);
        ERRDP(b"#", IVLBDS[(NINTVL + 1)], ctx);
        SIGERR(b"SPICE(COVERAGEGAP)", ctx)?;
        CHKOUT(b"SPKW19", ctx)?;
        return Ok(());
    }

    //
    // Check the input data before writing to the file.
    //
    // This order of operations entails some redundant
    // calculations, but it allows for rapid error
    // detection.
    //
    // Initialize the mini-segment packet array indices,
    // and those of the mini-segment epoch array as well.
    //
    PKTBEG = 0;
    PKTEND = 0;

    BEPIX = 0;
    EEPIX = 0;

    for I in 1..=NINTVL {
        //
        // First, just make sure the packet count for the current
        // mini-segment is at least two. This check reduces our chances
        // of a subscript range violation.
        //
        // Check the number of packets.
        //
        if (NPKTS[I] < 2) {
            SETMSG(b"At least 2 packets are required for SPK type 19. Number of packets supplied was # in mini-segment at index #.", ctx);
            ERRINT(b"#", NPKTS[I], ctx);
            ERRINT(b"#", I, ctx);
            SIGERR(b"SPICE(TOOFEWPACKETS)", ctx)?;
            CHKOUT(b"SPKW19", ctx)?;
            return Ok(());
        }

        //
        // Set the packet size, which is a function of the subtype. Also
        // set the window size. First check the subtype, which will be
        // used as an array index.
        //
        SUBTYP = SUBTPS[I];

        if ((SUBTYP < 0) || (SUBTYP > (S19NST - 1))) {
            SETMSG(
                b"Unexpected SPK type 19 subtype # found in mini-segment #.",
                ctx,
            );
            ERRINT(b"#", SUBTYP, ctx);
            ERRINT(b"#", I, ctx);
            SIGERR(b"SPICE(INVALIDSUBTYPE)", ctx)?;
            CHKOUT(b"SPKW19", ctx)?;
            return Ok(());
        }

        PKTSIZ = save.PKTSZS[SUBTYP];

        if (SUBTYP == S19TP0) {
            WINSIZ = ((DEGRES[I] + 1) / 2);
        } else if (SUBTYP == S19TP1) {
            WINSIZ = (DEGRES[I] + 1);
        } else if (SUBTYP == S19TP2) {
            WINSIZ = ((DEGRES[I] + 1) / 2);
        } else {
            SETMSG(b"Subtype = #; not expected.", ctx);
            ERRINT(b"#", SUBTYP, ctx);
            SIGERR(b"SPICE(BUG1)", ctx)?;
        }

        //
        // Make sure that the degree of the interpolating polynomials is
        // in range.
        //
        if ((DEGRES[I] < 1) || (DEGRES[I] > MAXDEG)) {
            SETMSG(b"The interpolating polynomials of mini-segment # have degree #; the valid degree range is [1, #]", ctx);
            ERRINT(b"#", I, ctx);
            ERRINT(b"#", DEGRES[I], ctx);
            ERRINT(b"#", MAXDEG, ctx);
            SIGERR(b"SPICE(INVALIDDEGREE)", ctx)?;
            CHKOUT(b"SPKW19", ctx)?;
            return Ok(());
        }

        //
        // Make sure that the window size is even.
        //
        if ODD(WINSIZ) {
            SETMSG(b"The interpolating polynomials of mini-segment # have window size # and degree # for SPK type 19. The mini-segment subtype is #. The degree must be equivalent to 3 mod 4 for subtype 0 (Hermite interpolation) and be odd for subtype 1 (Lagrange interpolation).", ctx);
            ERRINT(b"#", I, ctx);
            ERRINT(b"#", WINSIZ, ctx);
            ERRINT(b"#", DEGRES[I], ctx);
            ERRINT(b"#", SUBTPS[I], ctx);
            SIGERR(b"SPICE(BADWINDOWSIZE)", ctx)?;
            CHKOUT(b"SPKW19", ctx)?;
            return Ok(());
        }

        //
        // Make sure the epochs of the Ith mini-segment form a
        // strictly increasing sequence.
        //
        // To start out, determine the indices of the epoch sequence
        // of the Ith mini-segment. We'll call the begin and end
        // epoch indices BEPIX and EEPIX respectively.
        //
        BEPIX = (EEPIX + 1);
        EEPIX = ((BEPIX - 1) + NPKTS[I]);

        for J in 1..=(NPKTS[I] - 1) {
            K = ((BEPIX + J) - 1);

            if (EPOCHS[K] >= EPOCHS[(K + 1)]) {
                SETMSG(b"In mini-segment #, epoch # having index # in array EPOCHS and index # in the mini-segment is greater than or equal to its successor #.", ctx);
                ERRINT(b"#", I, ctx);
                ERRDP(b"#", EPOCHS[K], ctx);
                ERRINT(b"#", K, ctx);
                ERRINT(b"#", J, ctx);
                ERRDP(b"#", EPOCHS[(K + 1)], ctx);
                SIGERR(b"SPICE(TIMESOUTOFORDER)", ctx)?;
                CHKOUT(b"SPKW19", ctx)?;
                return Ok(());
            }
        }

        //
        // Make sure that the span of the input epochs of the Ith
        // mini-segment includes the Ith interpolation interval.
        //
        if (EPOCHS[BEPIX] > IVLBDS[I]) {
            SETMSG(
                b"Interpolation interval # start time # precedes mini-segment\'s first epoch #.",
                ctx,
            );
            ERRINT(b"#", I, ctx);
            ERRDP(b"#", IVLBDS[I], ctx);
            ERRDP(b"#", EPOCHS[BEPIX], ctx);
            SIGERR(b"SPICE(BOUNDSDISAGREE)", ctx)?;
            CHKOUT(b"SPKW19", ctx)?;
            return Ok(());
        } else if (EPOCHS[EEPIX] < IVLBDS[(I + 1)]) {
            SETMSG(
                b"Interpolation interval # end time # exceeds mini-segment\'s last epoch #.",
                ctx,
            );
            ERRINT(b"#", I, ctx);
            ERRDP(b"#", IVLBDS[(I + 1)], ctx);
            ERRDP(b"#", EPOCHS[EEPIX], ctx);
            SIGERR(b"SPICE(BOUNDSDISAGREE)", ctx)?;
            CHKOUT(b"SPKW19", ctx)?;
            return Ok(());
        }
    }

    //
    // If we made it this far, we're ready to start writing the segment.
    //
    // The type 19 segment structure is eloquently described by this
    // diagram from the SPK Required Reading:
    //
    //    +--------------------------------+
    //    | Interval 1 mini-segment        |
    //    +--------------------------------+
    //          .
    //          .
    //          .
    //    +--------------------------------+
    //    | Interval N mini-segment        |
    //    +--------------------------------+
    //    | Interval 1 start time          |
    //    +--------------------------------+
    //          .
    //          .
    //          .
    //    +--------------------------------+
    //    | Interval N start time          |
    //    +--------------------------------+
    //    | Interval N stop time           |
    //    +--------------------------------+
    //    | Interval start 100             | (First interval directory)
    //    +--------------------------------+
    //          .
    //          .
    //          .
    //    +--------------------------------+
    //    | Interval start (N/100)*100     | (Last interval directory)
    //    +--------------------------------+
    //    | Interval 1 start pointer       |
    //    +--------------------------------+
    //          .
    //          .
    //          .
    //    +--------------------------------+
    //    | Interval N start pointer       |
    //    +--------------------------------+
    //    | Interval N stop pointer + 1    |
    //    +--------------------------------+
    //    | Boundary choice flag           |
    //    +--------------------------------+
    //    | Number of intervals            |
    //    +--------------------------------+
    //
    //
    // SPK type 19 mini-segments have the following structure:
    //
    //    +-----------------------+
    //    | Packet 1              |
    //    +-----------------------+
    //                .
    //                .
    //                .
    //    +-----------------------+
    //    | Packet M              |
    //    +-----------------------+
    //    | Epoch 1               |
    //    +-----------------------+
    //                .
    //                .
    //                .
    //    +-----------------------+
    //    | Epoch M               |
    //    +-----------------------+
    //    | Epoch 100             | (First time tag directory)
    //    +-----------------------+
    //                .
    //                .
    //                .
    //    +-----------------------+
    //    | Epoch ((M-1)/100)*100 | (Last time tag directory)
    //    +-----------------------+
    //    | Subtype code          |
    //    +-----------------------+
    //    | Window size           |
    //    +-----------------------+
    //    | Number of packets     |
    //    +-----------------------+
    //
    //
    // Create the segment descriptor. We don't use SPKPDS because
    // that routine doesn't allow creation of a singleton segment.
    //
    IC[1] = BODY;
    IC[2] = CENTER;
    IC[3] = REFCOD;
    IC[4] = DTYPE;

    DC[1] = FIRST;
    DC[2] = LAST;

    DAFPS(ND, NI, DC.as_slice(), IC.as_slice(), DESCR.as_slice_mut());

    //
    // Begin a new segment.
    //
    DAFBNA(HANDLE, DESCR.as_slice(), SEGID, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SPKW19", ctx)?;
        return Ok(());
    }

    //
    // Re-initialize the mini-segment packet array indices,
    // and those of the mini-segment epoch array as well.
    //
    PKTBEG = 0;
    PKTEND = 0;

    BEPIX = 0;
    EEPIX = 0;

    //
    // Write data for each mini-segment to the file.
    //
    for I in 1..=NINTVL {
        //
        // Set the packet size, which is a function of the subtype.
        //
        SUBTYP = SUBTPS[I];

        PKTSIZ = save.PKTSZS[SUBTYP];

        if (SUBTYP == S19TP0) {
            WINSIZ = ((DEGRES[I] + 1) / 2);
        } else if (SUBTYP == S19TP1) {
            WINSIZ = (DEGRES[I] + 1);
        } else if (SUBTYP == S19TP2) {
            WINSIZ = ((DEGRES[I] + 1) / 2);
        } else {
            SETMSG(b"Subtype = #; not expected.", ctx);
            ERRINT(b"#", SUBTYP, ctx);
            SIGERR(b"SPICE(BUG2)", ctx)?;
        }

        //
        // Now that we have the packet size, we can compute
        // mini-segment packet index range. We'll let PKTDSZ
        // be the total count of packet data entries for this
        // mini-segment.
        //
        PKTDSZ = (NPKTS[I] * PKTSIZ);

        PKTBEG = (PKTEND + 1);
        PKTEND = ((PKTBEG - 1) + PKTDSZ);

        //
        // At this point, we're read to start writing the
        // current mini-segment to the file. Start with the
        // packet data.
        //
        DAFADA(PACKTS.subarray(PKTBEG), PKTDSZ, ctx)?;

        //
        // Write the epochs for this mini-segment.
        //
        BEPIX = (EEPIX + 1);
        EEPIX = ((BEPIX - 1) + NPKTS[I]);

        DAFADA(EPOCHS.subarray(BEPIX), NPKTS[I], ctx)?;

        //
        // Compute the number of epoch directories for the
        // current mini-segment.
        //
        NDIR = ((NPKTS[I] - 1) / DIRSIZ);

        //
        // Write the epoch directories to the segment.
        //
        for J in 1..=NDIR {
            K = ((BEPIX - 1) + (J * DIRSIZ));

            DAFADA(EPOCHS.subarray(K), 1, ctx)?;
        }

        //
        // Write the mini-segment's subtype, window size, and packet
        // count to the segment.
        //
        DAFADA(&[(SUBTPS[I] as f64)], 1, ctx)?;
        DAFADA(&[(WINSIZ as f64)], 1, ctx)?;
        DAFADA(&[(NPKTS[I] as f64)], 1, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"SPKW19", ctx)?;
            return Ok(());
        }
    }

    //
    // We've finished writing the mini-segments.
    //
    // Next write the interpolation interval bounds.
    //
    DAFADA(IVLBDS.as_slice(), (NINTVL + 1), ctx)?;

    //
    // Create and write directories for the interval
    // bounds.
    //
    // The directory count is the interval bound count
    // (N+1), minus 1, divided by the directory size.
    //
    NDIR = (NINTVL / DIRSIZ);

    for I in 1..=NDIR {
        DAFADA(IVLBDS.subarray((DIRSIZ * I)), 1, ctx)?;
    }

    //
    // Now we compute and write the start/stop pointers
    // for each mini-segment.
    //
    // The pointers are relative to the DAF address
    // preceding the segment. For example, a pointer
    // to the first DAF address in the segment has
    // value 1.
    //
    SEGEND = 0;

    for I in 1..=NINTVL {
        //
        // Set the packet size, which is a function of the subtype.
        //
        PKTSIZ = save.PKTSZS[SUBTPS[I]];

        //
        // In order to compute the end pointer of the current
        // mini-segment, we must compute the size, in terms
        // of DAF addresses, of this mini-segment. The formula
        // for the size is
        //
        //     size =     n_packets * packet_size
        //             +  n_epochs
        //             +  n_epoch_directories
        //             +  3
        //
        //          =     n_packets * ( packet_size + 1 )
        //             +  ( n_packets - 1 ) / DIRSIZ
        //             +  3
        //
        MINISZ = (((NPKTS[I] * (PKTSIZ + 1)) + ((NPKTS[I] - 1) / DIRSIZ)) + 3);

        SEGBEG = (SEGEND + 1);
        SEGEND = ((SEGBEG + MINISZ) - 1);

        //
        // Write the mini-segment begin pointer.
        //
        // After the loop terminates, the final end pointer, incremented
        // by 1, will be written.
        //
        DAFADA(&[(SEGBEG as f64)], 1, ctx)?;
    }

    //
    // Write the last mini-segment end pointer, incremented by one.
    // SEGEND was computed on the last iteration of the above loop.
    //
    DAFADA(&[((SEGEND + 1) as f64)], 1, ctx)?;

    //
    // Write out the interval selection flag. The input
    // boolean value is represented by a numeric constant.
    //
    if SELLST {
        ISEL = ITRUE;
    } else {
        ISEL = IFALSE;
    }

    DAFADA(&[(ISEL as f64)], 1, ctx)?;

    //
    // Write the mini-segment/interpolation interval count.
    //
    DAFADA(&[(NINTVL as f64)], 1, ctx)?;

    //
    // End the segment.
    //
    DAFENA(ctx)?;

    CHKOUT(b"SPKW19", ctx)?;
    Ok(())
}
