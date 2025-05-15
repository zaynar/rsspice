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
const BUFSIZ: i32 = 100;
const CTRLSZ: i32 = 3;
const DIRSIZ: i32 = 100;
const FILSIZ: i32 = 255;

struct SaveVars {
    SPK: Vec<u8>,
    CONTRL: StackArray<f64, 3>,
    DATA: StackArray<f64, 100>,
    IV1BEG: f64,
    IV1END: f64,
    IVFBEG: f64,
    IVFEND: f64,
    IVLBEG: f64,
    IVLEND: f64,
    BEGIDX: i32,
    BEPIDX: i32,
    BUFBAS: i32,
    CURIVL: i32,
    EEPIDX: i32,
    ENDIDX: i32,
    I: i32,
    ISEL: i32,
    IVLBAS: i32,
    L: i32,
    MIN1SZ: i32,
    MINBEP: i32,
    MINFSZ: i32,
    MINIB: i32,
    MINIE: i32,
    MINNDR: i32,
    MINNPK: i32,
    NDIR: i32,
    NINTVL: i32,
    NOIVL: i32,
    NPAD: i32,
    NPKT: i32,
    NREAD: i32,
    NSDIR: i32,
    PKTSIZ: i32,
    PKTSZS: StackArray<i32, 3>,
    PTRBAS: i32,
    REMAIN: i32,
    SHIFT: i32,
    START: i32,
    SUBTYP: i32,
    UB: i32,
    WNDSIZ: i32,
    FINAL: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SPK = vec![b' '; FILSIZ as usize];
        let mut CONTRL = StackArray::<f64, 3>::new(1..=CTRLSZ);
        let mut DATA = StackArray::<f64, 100>::new(1..=BUFSIZ);
        let mut IV1BEG: f64 = 0.0;
        let mut IV1END: f64 = 0.0;
        let mut IVFBEG: f64 = 0.0;
        let mut IVFEND: f64 = 0.0;
        let mut IVLBEG: f64 = 0.0;
        let mut IVLEND: f64 = 0.0;
        let mut BEGIDX: i32 = 0;
        let mut BEPIDX: i32 = 0;
        let mut BUFBAS: i32 = 0;
        let mut CURIVL: i32 = 0;
        let mut EEPIDX: i32 = 0;
        let mut ENDIDX: i32 = 0;
        let mut I: i32 = 0;
        let mut ISEL: i32 = 0;
        let mut IVLBAS: i32 = 0;
        let mut L: i32 = 0;
        let mut MIN1SZ: i32 = 0;
        let mut MINBEP: i32 = 0;
        let mut MINFSZ: i32 = 0;
        let mut MINIB: i32 = 0;
        let mut MINIE: i32 = 0;
        let mut MINNDR: i32 = 0;
        let mut MINNPK: i32 = 0;
        let mut NDIR: i32 = 0;
        let mut NINTVL: i32 = 0;
        let mut NOIVL: i32 = 0;
        let mut NPAD: i32 = 0;
        let mut NPKT: i32 = 0;
        let mut NREAD: i32 = 0;
        let mut NSDIR: i32 = 0;
        let mut PKTSIZ: i32 = 0;
        let mut PKTSZS = StackArray::<i32, 3>::new(0..=(S19NST - 1));
        let mut PTRBAS: i32 = 0;
        let mut REMAIN: i32 = 0;
        let mut SHIFT: i32 = 0;
        let mut START: i32 = 0;
        let mut SUBTYP: i32 = 0;
        let mut UB: i32 = 0;
        let mut WNDSIZ: i32 = 0;
        let mut FINAL: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(S19PS0), Val::I(S19PS1), Val::I(S19PS2)].into_iter();
            PKTSZS
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            SPK,
            CONTRL,
            DATA,
            IV1BEG,
            IV1END,
            IVFBEG,
            IVFEND,
            IVLBEG,
            IVLEND,
            BEGIDX,
            BEPIDX,
            BUFBAS,
            CURIVL,
            EEPIDX,
            ENDIDX,
            I,
            ISEL,
            IVLBAS,
            L,
            MIN1SZ,
            MINBEP,
            MINFSZ,
            MINIB,
            MINIE,
            MINNDR,
            MINNPK,
            NDIR,
            NINTVL,
            NOIVL,
            NPAD,
            NPKT,
            NREAD,
            NSDIR,
            PKTSIZ,
            PKTSZS,
            PTRBAS,
            REMAIN,
            SHIFT,
            START,
            SUBTYP,
            UB,
            WNDSIZ,
            FINAL,
        }
    }
}

/// S/P Kernel, subset, type 19
///
/// Extract a subset of the data in an SPK segment of type 19
/// into a new segment.
///
/// # Required Reading
///
/// * [SPK](crate::required_reading::spk)
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of file containing source segment.
///  BADDR      I   Beginning address in file of source segment.
///  EADDR      I   Ending address in file of source segment.
///  BEGIN      I   Beginning (initial epoch) of subset.
///  END        I   End (final epoch) of subset.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE,
///  BADDR,
///  EADDR    are the file handle assigned to an SPK file, and the
///           beginning and ending addresses of a segment within
///           that file. Together these identify an SPK segment
///           from which a subset is to be extracted.
///
///           The subset is written to a second SPK file which is
///           open for writing, and in which a new segment has been
///           started. See the $Particulars section below for
///           details.
///
///  BEGIN,
///  END      are the initial and final epochs (ephemeris time)
///           of the subset.
///
///           The first epoch for which there will be ephemeris
///           data in the new segment will be the greatest time
///           in the source segment that is less than or equal
///           to BEGIN.
///
///           The last epoch for which there will be ephemeris
///           data in the new segment will be the smallest time
///           in the source segment that is greater than or equal
///           to END.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None. See $Files section.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  This routine relies on the caller to ensure that the
///      interval [BEGIN, END] is contained in the coverage
///      interval of the source segment.
///
///  2)  If BEGIN > END, no data are written to the target file.
///
///  3)  If a unexpected SPK type 19 subtype is found in the input
///      segment, the error SPICE(INVALIDVALUE) is signaled.
///
///  4)  The input segment must have valid structure; this
///      routine may fail in unpredictable ways if not.
/// ```
///
/// # Files
///
/// ```text
///  Data are extracted from the file connected to the input
///  handle, and written to the current DAF open for writing.
///
///  The segment descriptor and summary must already have been written
///  prior to calling this routine. The segment must be ended
///  external to this routine.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is intended solely for use as a utility by the
///  routine SPKSUB.
///
///  It transfers a subset of a type 19 SPK data segment to
///  a properly initialized segment of a second SPK file.
///
///  The exact structure of a segment of data type 19 is described
///  in the section on type 19 in the SPK Required Reading.
/// ```
///
/// # Examples
///
/// ```text
///  This routine is intended only for use as a utility by SPKSUB.
///  To use this routine successfully, you must:
///
///     Open the SPK file from which to extract data.
///     Locate the segment from which data should be extracted.
///
///     Open the SPK file to which this data should be written.
///     Begin a new segment (array).
///     Write the summary information for the array.
///
///     Call this routine to extract the appropriate data from the
///     SPK open for read.
///
///     End the array to which this routine writes data.
///
///  Much of this procedure is carried out by the routine SPKSUB. The
///  examples of that routine illustrate more fully the process
///  described above.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine relies on the input segment being correct;
///      very limited error checking is performed on the input
///      data.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.0.1, 03-JUN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section.
///
/// -    SPICELIB Version 2.0.0, 04-APR-2017 (NJB)
///
///         Typo in comment fixed.
///
///         11-MAY-2015 (NJB)
///
///         Updated to support subtype 2.
///
/// -    SPICELIB Version 1.0.0, 17-OCT-2011 (NJB) (BVS) (WLT) (IMU) (EDW)
/// ```
pub fn spks19(
    ctx: &mut SpiceContext,
    handle: i32,
    baddr: i32,
    eaddr: i32,
    begin: f64,
    end: f64,
) -> crate::Result<()> {
    SPKS19(handle, baddr, eaddr, begin, end, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKS19 ( S/P Kernel, subset, type 19 )
pub fn SPKS19(
    HANDLE: i32,
    BADDR: i32,
    EADDR: i32,
    BEGIN: f64,
    END: f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Mini-segment control area size:
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

    CHKIN(b"SPKS19", ctx)?;

    //
    // Terminology
    // ===========
    //
    //   - A point P is in the "interior" of a set S if P is
    //     an element of S and P is not contained in the boundary
    //     of S. If S is a discrete set of distinct times, then
    //     the interior points of S are greater than the earliest
    //     time in S and earlier than the latest time in S. If
    //     S is the closed interval [A, B], that is, if S is the set of
    //     points P such that
    //
    //        A  <  P  <  B
    //           -     -
    //
    //     then the interior of S is the set of points P such that
    //
    //        A  <  P  <  B
    //
    //   - A subset S2 of a set S1 is in the "interior" of S1 if
    //     every point of S2 is contained in the interior of S1.
    //
    //   - SPK type 19 interpolation intervals are often simply
    //     called "intervals."
    //
    //   - The data set corresponding to a type 19 interpolation
    //     interval is called a "mini-segment."
    //
    //   - "Padding" consists of a sequence of contiguous data packets
    //     and a corresponding sequence of epochs provided to enable
    //     correct interpolation near interval boundaries, where the
    //     epochs lie outside of the interval's coverage time range.
    //     Padding data and epochs are always drawn only from the same
    //     input mini-segment that provides data for the output
    //     mini-segment under construction.
    //
    //   - A "base address" of a structure is the DAF address preceding
    //     the first address occupied by the structure.
    //
    //
    // Algorithm
    // =========
    //
    // The algorithm below transfers to the output segment sufficient
    // data to cover the time range BEGIN : END, such that the output
    // segment yields interpolation behavior identical to that of the
    // selected portion of the input segment.
    //
    // No use is made of the selection order attribute other than to
    // transfer it to the output segment. This simplifies the algorithm,
    // at the expense of making the output segment larger than necessary
    // by at most a small, bounded amount. Specifically, when either
    // BEGIN or END coincides with an interior interval boundary, a
    // small additional output interval is created so as to make that
    // boundary an interior point of the output segment's coverage
    // interval. This guarantees that the correct interval can be
    // selected when a request time coincides with the boundary of
    // interest.
    //
    // The overall approach is:
    //
    //    1)  Obtain attribute information from the input segment.
    //
    //    2)  Create a first output mini-segment. This mini-segment is
    //        created using data from the first input mini-segment
    //        having an end time greater than or equal to BEGIN.
    //
    //        The first output mini-segment contains padding, if needed,
    //        on both the left and right sides. On the left side, given
    //        a nominal interpolation window width W (W must be even),
    //        the nominal pad size NPAD is (W/2) - 1. If I is the index
    //        of the last time tag (in the selected input mini-segment)
    //        less than or equal to BEGIN, the pad starts at I-NPAD or
    //        1, whichever is greater.
    //
    //        On the right side, if END is greater than or equal to the
    //        last epoch of the input mini-segment, all epochs and
    //        packets of the input mini-segment following the first ones
    //        selected are transferred to the output mini-segment.
    //
    //        The first mini-segment requires padding on the right only
    //        if END precedes the end time of the input mini-segment. In
    //        this case the pad size is chosen so that the output
    //        mini-segment contains W/2 epochs greater than or equal to
    //        END, if possible. If I is the index of the first time tag
    //        in the mini-segment greater than or equal to END, then the
    //        pad ends at I + (W/2) - 1 or NPKT, whichever is smaller.
    //
    //        Note that due to the asymmetry of the search techniques
    //        used (there are no SPICELIB right-to-left search routines
    //        analogous to LSTLTD and LSTLED), the implementation of the
    //        pad computation for the right side is not as similar to
    //        that for the left side as one might expect.
    //
    //        The first output mini-segment and all subsequent output
    //        mini-segments have the structure of an SPK type 18
    //        segment. They consist of
    //
    //           a) A sequence of data packets
    //
    //           b) A sequence of epochs
    //
    //           c) An epoch directory, if needed
    //
    //           d) A control area, consisting of
    //
    //                - A subtype code
    //                - An interpolation window size
    //                - A packet count
    //
    //    3)  All input mini-segments whose interpolation intervals
    //        follow that of the first used mini-segment and whose stop
    //        times are less than or equal to END are copied whole
    //        to the output segment. We refer to this sequence of
    //        mini-segments as the "middle group." The middle group may
    //        be empty.
    //
    //    4)  If necessary, a final output mini-segment is written. This
    //        mini-segment will be required unless either
    //
    //           - The interval of the first mini-segment contains in
    //             its interior the interval BEG : END.
    //
    //           - The middle group ends at the end of the input segment.
    //
    //        Note that if the last interval of the middle group ends at
    //        END, but END is less than the final input interval's stop
    //        time, a final mini-segment is still needed to ensure
    //        correct interval selection. If there is no middle group
    //        and the first used interpolation interval ends at END, and
    //        if END is less than the final input interval's stop time,
    //        a final mini-segment is required as well.
    //
    //        The interpolation interval of the final output
    //        mini-segment always starts at an input interval boundary.
    //        This interval has padding on the left only if the
    //        corresponding input interval has padding on the left; any
    //        existing left side padding from the input mini-segment is
    //        simply copied to the output mini-segment. On the right
    //        side, padding is created if it is necessary and possible
    //        to do so. When right side padding is used, the pad size
    //        and placement follow the same rules used for the right
    //        side padding of the first output mini-segment.
    //
    //    5)  After all output mini-segments have been written, the
    //        following segment-level structures are written to the
    //        output segment:
    //
    //           a) The output segment interpolation interval
    //              boundaries. This list includes the start time of
    //              each output interval and the stop time of the final
    //              output interval.
    //
    //           b) The output segment interpolation interval boundary
    //              directory, if needed.
    //
    //           c) The output segment's mini-segment begin and "end"
    //              pointers. This list consists of the segment
    //              base-relative first address of each mini-segment,
    //              plus the relative address following the final output
    //              mini-segment.
    //
    //           d) The output segment control area. This consists of:
    //
    //                - The interval selection order flag. This is copied
    //                  from the input segment.
    //
    //                - The output segment interval count
    //
    //
    //
    //
    // See whether there's any work to do; return immediately if not.
    //
    if (BEGIN > END) {
        CHKOUT(b"SPKS19", ctx)?;
        return Ok(());
    }
    //
    // We don't check BEGIN and END against the time bounds of the
    // descriptor of the input file because, according to the
    // SPK subsetting subsystem design, the calling routine SPKSUB
    // has done this already. Note that the descriptor of the source
    // segment is not even an input to this routine. If we wanted to,
    // we could search the input DAF for a descriptor that mapped to
    // the address range BADDR : EADDR. but we're not going to do
    // that.
    //
    // Initialize the flag indicating the existence of the "final"
    // output mini-segment.
    //
    save.FINAL = false;

    //***********************************************************************
    //
    //     Part 1: Obtain attributes of the input segment
    //
    //***********************************************************************

    //
    // Read the input segment structure control area.
    //
    DAFGDA(HANDLE, (EADDR - 1), EADDR, save.DATA.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SPKS19", ctx)?;
        return Ok(());
    }

    //
    // Fetch the interval selection order flag and the
    // number of interpolation intervals.
    //
    save.ISEL = intrinsics::IDNINT(save.DATA[1]);
    save.NINTVL = intrinsics::IDNINT(save.DATA[2]);

    //
    // Compute the number of interval boundary directories. Recall that
    // the final interval stop time must be accounted for, so the
    // directory count is
    //
    //    ( ( NINTVL + 1) - 1 ) / DIRSIZ
    //

    save.NDIR = (save.NINTVL / DIRSIZ);

    //
    // Find the base address IVLBAS of the interval start times. First
    // set PTRBAS, which is the address preceding the interval pointers.
    //
    // The interval base precedes the interval bounds, the interval
    // directories, the interval pointers, and the control area.
    //
    save.PTRBAS = (EADDR - ((2 + save.NINTVL) + 1));
    save.IVLBAS = (save.PTRBAS - ((save.NDIR + save.NINTVL) + 1));

    //***********************************************************************
    //
    //     Part 2: Create the first output mini-segment
    //
    //***********************************************************************

    //
    // Search for the first interval that will contribute data to the
    // output segment. We first find the last interval boundary that is
    // strictly less than the epoch BEGIN. The final interval stop time
    // need not be considered, since the segment covers the interval
    // [BEGIN : END]. Note however there is a "corner case" in which
    //
    //    BEGIN == END == <final interval stop time>
    //
    // Since we're only examining interval start times, the last one
    // we may need to read is at index NINTVL.
    //
    save.NREAD = intrinsics::MIN0(&[BUFSIZ, save.NINTVL]);

    save.BUFBAS = save.IVLBAS;

    //
    // NREAD is at least 1 here.
    //
    DAFGDA(
        HANDLE,
        (save.BUFBAS + 1),
        (save.BUFBAS + save.NREAD),
        save.DATA.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"SPKS19", ctx)?;
        return Ok(());
    }

    save.REMAIN = (save.NINTVL - save.NREAD);

    //
    // The variable NREAD is the array index of the last
    // item read into the buffer on the previous read
    // operation. On the first pass NREAD is at least 1.
    //
    while ((save.REMAIN > 0) && (save.DATA[save.NREAD] < BEGIN)) {
        save.BUFBAS = (save.BUFBAS + save.NREAD);

        save.NREAD = intrinsics::MIN0(&[BUFSIZ, save.REMAIN]);
        //
        // NREAD is at least 1 here.
        //
        DAFGDA(
            HANDLE,
            (save.BUFBAS + 1),
            (save.BUFBAS + save.NREAD),
            save.DATA.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"SPKS19", ctx)?;
            return Ok(());
        }

        save.REMAIN = (save.REMAIN - save.NREAD);
    }
    //
    // Let I be the index of the last interval boundary time that
    // precedes BEGIN. If there are no such boundary times, I will be
    // zero. This latter case can happen only when the first interval
    // start time is equal to BEGIN.
    //
    // At this point BUFBAS - IVLBAS is the number of boundaries we
    // examined before the final call above to DAFGDA. All of those
    // boundary times were strictly less than BEGIN.
    //
    save.I = ((save.BUFBAS - save.IVLBAS) + LSTLTD(BEGIN, save.NREAD, save.DATA.as_slice()));

    //
    // Let BEGIDX be the index of the last interval start time that
    // precedes BEGIN, unless BEGIN coincides with the first interval
    // start time; in this case, BEGIDX must be 1.
    //
    save.BEGIDX = intrinsics::MAX0(&[1, save.I]);

    //
    // In order to extract data from the mini-segment, we'll need its
    // address range.
    //
    DAFGDA(
        HANDLE,
        (save.PTRBAS + save.BEGIDX),
        ((save.PTRBAS + save.BEGIDX) + 1),
        save.DATA.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"SPKS19", ctx)?;
        return Ok(());
    }

    //
    // Convert the segment-base-relative mini-segment begin and end
    // pointers to absolute DAF addresses.
    //
    save.MINIB = ((BADDR - 1) + intrinsics::IDNINT(save.DATA[1]));
    save.MINIE = (((BADDR - 1) + intrinsics::IDNINT(save.DATA[2])) - 1);

    //
    // Read the control area of the mini-segment.
    //
    save.BUFBAS = (save.MINIE - CTRLSZ);

    DAFGDA(
        HANDLE,
        (save.BUFBAS + 1),
        (save.BUFBAS + CTRLSZ),
        save.CONTRL.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"SPKS19", ctx)?;
        return Ok(());
    }

    //
    // Fetch the control area parameters for the mini-segment.
    //
    save.SUBTYP = intrinsics::IDNINT(save.CONTRL[1]);
    save.WNDSIZ = intrinsics::IDNINT(save.CONTRL[2]);
    save.NPKT = intrinsics::IDNINT(save.CONTRL[3]);

    //
    // Set the packet size, which is a function of the subtype.
    //
    if ((save.SUBTYP < 0) || (save.SUBTYP >= S19NST)) {
        SETMSG(
            b"Unexpected SPK type 19 subtype # found in type 19 segment within mini-segment #.",
            ctx,
        );
        ERRINT(b"#", save.SUBTYP, ctx);
        ERRINT(b"#", save.BEGIDX, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"SPKS19", ctx)?;
        return Ok(());
    }

    save.PKTSIZ = save.PKTSZS[save.SUBTYP];

    //
    // Determine how much of the mini-segment we need to transfer. The
    // first step is to find the last epoch less than or equal to BEGIN
    // in the mini-segment's epoch list. Let MINBEP be the base address
    // of the epoch list (that is, the start address minus 1).
    //
    save.MINBEP = ((save.MINIB - 1) + (save.NPKT * save.PKTSIZ));

    //
    // Read epochs until we find one greater than or equal to BEGIN.
    //
    // It's possible that only the last epoch of the input mini-segment
    // satisfies this criterion, but at least one epoch must satisfy it.
    //
    save.NREAD = intrinsics::MIN0(&[BUFSIZ, save.NPKT]);

    save.BUFBAS = save.MINBEP;

    DAFGDA(
        HANDLE,
        (save.BUFBAS + 1),
        (save.BUFBAS + save.NREAD),
        save.DATA.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"SPKS19", ctx)?;
        return Ok(());
    }

    save.REMAIN = (save.NPKT - save.NREAD);

    //
    // The variable NREAD is the array index of the last
    // item read into the buffer on the previous read
    // operation.
    //
    while ((save.REMAIN > 0) && (save.DATA[save.NREAD] < BEGIN)) {
        //
        // Advance the buffer base to account for the NREAD
        // epochs fetched on the previous DAFGDA call.
        //
        save.BUFBAS = (save.BUFBAS + save.NREAD);

        save.NREAD = intrinsics::MIN0(&[BUFSIZ, save.REMAIN]);
        //
        // Since REMAIN was positive at the beginning of this
        // loop iteration, NREAD is positive here.
        //
        DAFGDA(
            HANDLE,
            (save.BUFBAS + 1),
            (save.BUFBAS + save.NREAD),
            save.DATA.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"SPKS19", ctx)?;
            return Ok(());
        }

        save.REMAIN = (save.REMAIN - save.NREAD);
    }
    //
    // At this point BUFBAS - MINBEP is the number of epochs in the
    // input mini-segment we've examined before the final call above to
    // DAFGDA. All of those epochs were strictly less than BEGIN.
    //
    // Let BEPIDX be the index of the last epoch that precedes or is
    // equal to BEGIN. That epoch is contained in the last buffer we
    // read.
    //
    save.BEPIDX = ((save.BUFBAS - save.MINBEP) + LSTLED(BEGIN, save.NREAD, save.DATA.as_slice()));

    //
    // BEPIDX is at least 1 and may be as large as NPKT.
    //
    // Compute the number of pad epochs we need to maintain proper
    // interpolation behavior in the neighborhood of the epoch at
    // index BEPIDX.
    //
    save.NPAD = ((save.WNDSIZ / 2) - 1);

    //
    // Shift BEPIDX by the pad amount, if possible. The minimum value
    // of BEPIDX is 1.
    //
    save.BEPIDX = intrinsics::MAX0(&[1, (save.BEPIDX - save.NPAD)]);

    //
    // The output mini-segment can never have fewer than two epochs.
    // When the window size is 2 and BEPIDX is equal to NPKT, we
    // must extend the window on the left.
    //
    save.BEPIDX = intrinsics::MIN0(&[save.BEPIDX, (save.NPKT - 1)]);

    //
    // If the input interval end time is less than or equal to END, we
    // need to use the rest of the data from this interval. Otherwise
    // find out how much data from this interval we need to transfer.
    //
    save.BUFBAS = (save.IVLBAS + save.BEGIDX);

    DAFGDA(
        HANDLE,
        (save.BUFBAS + 1),
        (save.BUFBAS + 1),
        std::slice::from_mut(&mut save.IVLEND),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"SPKS19", ctx)?;
        return Ok(());
    }

    //
    // Let EEPIDX be the index of the last epoch we select from
    // the current input mini-segment. We'll set EEPIDX below.
    //
    if (save.IVLEND <= END) {
        //
        // The requested subset coverage is either equal to or extends
        // beyond the right boundary of this interval. We'll use all data
        // from this interval.
        //
        save.EEPIDX = save.NPKT;
    } else {
        //
        // IVLEND is strictly greater than END. This interval covers
        //
        //    [BEGIN, END].
        //
        // Read epochs from this mini-segment until we find one greater
        // than or equal to END. We have an error if we run out of
        // epochs.
        //
        // The input mini-segment contains ( NPKT - BEPIDX + 1 ) epochs
        // following and including the one at BEPIDX.
        //
        save.REMAIN = ((save.NPKT - save.BEPIDX) + 1);
        //
        // REMAIN is at least 2 at this point, since in this case,
        // some epoch exceeds END, and that epoch must have index
        // greater than BEPIDX.
        //
        save.NREAD = intrinsics::MIN0(&[BUFSIZ, save.REMAIN]);
        //
        // NREAD is at least 2.
        //
        if (save.NREAD < 2) {
            //
            // This code should not be reached.
            //
            DAFHFN(HANDLE, &mut save.SPK, ctx)?;

            SETMSG(b"Input file: #. Segment address range: #:#. Structural error found: NREAD is #; end time of interval # is #.", ctx);
            ERRCH(b"#", &save.SPK, ctx);
            ERRINT(b"#", BADDR, ctx);
            ERRINT(b"#", EADDR, ctx);
            ERRINT(b"#", save.NREAD, ctx);
            ERRINT(b"#", save.BEGIDX, ctx);
            ERRDP(b"#", save.IVLEND, ctx);
            SIGERR(b"SPICE(SPKSTRUCTUREERROR)", ctx)?;
            CHKOUT(b"SPKS19", ctx)?;
            return Ok(());
        }
        //
        // Set the buffer base address so that we start reading at
        // address MINBEP + BEPIDX.
        //
        save.BUFBAS = ((save.MINBEP + save.BEPIDX) - 1);

        DAFGDA(
            HANDLE,
            (save.BUFBAS + 1),
            (save.BUFBAS + save.NREAD),
            save.DATA.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"SPKS19", ctx)?;
            return Ok(());
        }

        save.REMAIN = (save.REMAIN - save.NREAD);

        //
        // NREAD is (still) at least 2.
        //
        while ((save.REMAIN > 0) && (save.DATA[save.NREAD] <= END)) {
            save.BUFBAS = (save.BUFBAS + save.NREAD);

            save.NREAD = intrinsics::MIN0(&[save.REMAIN, BUFSIZ]);
            //
            // NREAD is at least 1 since REMAIN was positive
            // at the top of the loop.
            //
            DAFGDA(
                HANDLE,
                (save.BUFBAS + 1),
                (save.BUFBAS + save.NREAD),
                save.DATA.as_slice_mut(),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"SPKS19", ctx)?;
                return Ok(());
            }

            save.REMAIN = (save.REMAIN - save.NREAD);
        }
        //
        // At this point BUFBAS - MINBEP is the number of epochs in the
        // input mini-segment we've examined before the final call above
        // to DAFGDA. If this set of epochs is non-empty, all of these
        // epochs are less than or equal to END. Note that it's possible
        // for END and BEGIN to be equal to the first epoch.
        //
        // Let EEPIDX be the index of the first epoch that is strictly
        // greater than END. As asserted above, in this branch of the
        // code, such an epoch must exist. That epoch is contained in the
        // last buffer we read.
        //
        // EEPIDX exceeds by 1 the index of the last epoch less than or
        // equal to END.
        //
        save.L = LSTLED(END, save.NREAD, save.DATA.as_slice());

        save.EEPIDX = (((save.BUFBAS - save.MINBEP) + save.L) + 1);

        //
        // EEPIDX is at least 2 and is less than or equal to NPKT.
        //
        if ((save.EEPIDX < 2) || (save.EEPIDX > save.NPKT)) {
            //
            // This code should not be reached.
            //
            DAFHFN(HANDLE, &mut save.SPK, ctx)?;

            SETMSG(b"Input file: #. Segment address range: #:#. Structural error found: last epoch is #; end time of interval # is #.", ctx);
            ERRCH(b"#", &save.SPK, ctx);
            ERRINT(b"#", BADDR, ctx);
            ERRINT(b"#", EADDR, ctx);
            ERRDP(b"#", save.DATA[save.NREAD], ctx);
            ERRINT(b"#", save.BEGIDX, ctx);
            ERRDP(b"#", save.IVLEND, ctx);
            SIGERR(b"SPICE(SPKSTRUCTUREERROR)", ctx)?;
            CHKOUT(b"SPKS19", ctx)?;
            return Ok(());
        }

        //
        // Compute the number of pad epochs we need to maintain proper
        // interpolation behavior in the neighborhood of the epoch at
        // index EEPIDX.
        //
        if (save.DATA[save.L] == END) {
            //
            // The epochs at indices EEPIDX-1 and EEPIDX comprise the
            // first two epochs of the right half of an interpolation
            // window of size WNDSIZ. We need two fewer pad epochs to
            // complete the right half of the window.
            //
            save.NPAD = ((save.WNDSIZ / 2) - 2);
        } else {
            //
            // The epoch at EEPIDX is the first of the pad.
            //
            save.NPAD = ((save.WNDSIZ / 2) - 1);
        }

        //
        // The maximum allowed value of EEPIDX is NPKT.
        //
        save.EEPIDX = intrinsics::MIN0(&[save.NPKT, (save.EEPIDX + save.NPAD)]);
    }

    //
    // At this point BEPIDX and EEPIDX are both set.
    //
    // Look up the input interval's start time at index BEGIDX.
    // We'll use this below when we compute the interval start
    // time of the first output mini-segment.
    //
    DAFGDA(
        HANDLE,
        (save.IVLBAS + save.BEGIDX),
        (save.IVLBAS + save.BEGIDX),
        std::slice::from_mut(&mut save.IVLBEG),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"SPKS19", ctx)?;
        return Ok(());
    }

    // We're ready to start transferring data to the output segment. The
    // first mini-segment of the output segment will contain packets
    // BEPIDX through EEPIDX of the input mini-segment at index BEGIDX.
    //
    {
        let m1__: i32 = save.BEPIDX;
        let m2__: i32 = save.EEPIDX;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.BUFBAS = ((save.MINIB - 1) + ((save.I - 1) * save.PKTSIZ));

            DAFGDA(
                HANDLE,
                (save.BUFBAS + 1),
                (save.BUFBAS + save.PKTSIZ),
                save.DATA.as_slice_mut(),
                ctx,
            )?;

            DAFADA(save.DATA.as_slice(), save.PKTSIZ, ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"SPKS19", ctx)?;
                return Ok(());
            }

            save.I += m3__;
        }
    }

    //
    // Now transfer the epochs at indices BEPIDX : EEPIDX.
    //
    // Inside this loop, determine the bounds of the first output
    // interpolation interval. Each bound is either the corresponding
    // bound of the input interval, or a boundary epoch (first or last)
    // of the output epoch list, whichever is most restrictive.
    //
    {
        let m1__: i32 = save.BEPIDX;
        let m2__: i32 = save.EEPIDX;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.BUFBAS = (((save.MINIB - 1) + (save.NPKT * save.PKTSIZ)) + (save.I - 1));

            DAFGDA(
                HANDLE,
                (save.BUFBAS + 1),
                (save.BUFBAS + 1),
                save.DATA.as_slice_mut(),
                ctx,
            )?;

            DAFADA(save.DATA.as_slice(), 1, ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"SPKS19", ctx)?;
                return Ok(());
            }

            //
            // Let IV1BEG be the start time of the first output
            // interpolation interval. Determine IV1BEG on the first loop
            // pass. IVLBEG has already been set to the start time of the
            // input interval at index BEGIDX.
            //
            if (save.I == save.BEPIDX) {
                //
                // The first output interval cannot start earlier than
                // the interval from which its data are taken. It may
                // start later.
                //
                save.IV1BEG = intrinsics::DMAX1(&[save.IVLBEG, save.DATA[1]]);
            }

            //
            // Determine IV1END on the final loop pass.
            //
            if (save.I == save.EEPIDX) {
                //
                // The first output interval cannot end later than
                // the interval from which its data are taken. It may
                // end earlier.
                //
                save.IV1END = intrinsics::DMIN1(&[save.IVLEND, save.DATA[1]]);
            }

            save.I += m3__;
        }
    }

    //
    // Create the epoch directory for the first output mini-segment.
    //
    save.MINNPK = ((save.EEPIDX - save.BEPIDX) + 1);

    save.MINNDR = ((save.MINNPK - 1) / DIRSIZ);

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.MINNDR;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Set BUFBAS to the address that immediately precedes the
            // element we're about to read. We must skip over the data
            // packets and the first (BEPIDX-1) epochs before starting our
            // count.
            //
            save.BUFBAS = (((((save.MINIB - 1) + (save.NPKT * save.PKTSIZ)) + (save.BEPIDX - 1))
                + (save.I * DIRSIZ))
                - 1);

            DAFGDA(
                HANDLE,
                (save.BUFBAS + 1),
                (save.BUFBAS + 1),
                save.DATA.as_slice_mut(),
                ctx,
            )?;

            DAFADA(save.DATA.as_slice(), 1, ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"SPKS19", ctx)?;
                return Ok(());
            }

            save.I += m3__;
        }
    }

    //
    // Finally, write out the control area for the first mini-segment.
    //
    DAFADA(&[(save.SUBTYP as f64)], 1, ctx)?;
    DAFADA(&[(save.WNDSIZ as f64)], 1, ctx)?;
    DAFADA(&[(save.MINNPK as f64)], 1, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SPKS19", ctx)?;
        return Ok(());
    }

    //
    // Compute the size of the first output mini-segment; we'll need
    // this later to compute the second mini-segment start pointer.
    // The size is the sum of the sizes of the packet set, the
    // epochs, the epoch directories, and the control area.
    //
    save.MIN1SZ = (((save.MINNPK * (save.PKTSIZ + 1)) + save.MINNDR) + CTRLSZ);

    //***********************************************************************
    //
    //     Part 3: Transfer the middle group of mini-segments to the
    //             output segment, if this group is non-empty
    //
    //***********************************************************************

    //
    // At this point, we might already be done with copying
    // mini-segments. If the coverage interval of the mini-segment we
    // just processed contains [BEGIN, END] in its interior, we're done.
    // If there are no more input mini-segments, we're also done.
    // Otherwise, we'll continue to transfer data from subsequent
    // input mini-segments.
    //
    // At this point IVLEND is the end time of the first input
    // interval. Note that this time may differ from IV1END, which
    // is the end time of the first output interval.
    //
    if ((save.IVLEND > END) || (save.BEGIDX == save.NINTVL)) {
        //
        // We've transferred all the data we need. We don't need
        // to obtain data from other mini-segments.
        //
        save.ENDIDX = save.BEGIDX;

    //
    // FINAL is already set to .FALSE.
    //
    } else {
        //
        // We need more data, and there are more data to be had.
        //
        // Things get a bit easier here: all mini-segments that follow
        // the one we just wrote, and that have end times less than or
        // equal to END, get copied without modification to the output
        // file. Note that this sequence of mini-segments could be empty.

        save.CURIVL = (save.BEGIDX + 1);

        //
        // Initialize the start time of the final output mini-segment.
        // We'll update this if we produce more output mini-segments.
        //
        save.IVFBEG = save.IVLEND;

        //
        // Get the end time of the interval at index CURIVL.
        //
        save.BUFBAS = (save.IVLBAS + save.CURIVL);

        DAFGDA(
            HANDLE,
            (save.BUFBAS + 1),
            (save.BUFBAS + 1),
            std::slice::from_mut(&mut save.IVLEND),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"SPKS19", ctx)?;
            return Ok(());
        }

        //
        // CURIVL is the index of the interval we're about to process,
        // and if CURIVL is in range, IVLEND is its end time.
        //
        while ((save.IVLEND <= END) && (save.CURIVL <= save.NINTVL)) {
            //
            // Entering this loop means the "middle" component of the
            // output segment is non-empty.
            //
            // Get the begin and end pointers for the current mini-segment.
            //
            save.BUFBAS = ((save.PTRBAS + save.CURIVL) - 1);

            DAFGDA(
                HANDLE,
                (save.BUFBAS + 1),
                (save.BUFBAS + 2),
                save.DATA.as_slice_mut(),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"SPKS19", ctx)?;
                return Ok(());
            }

            save.MINIB = ((BADDR - 1) + intrinsics::IDNINT(save.DATA[1]));
            save.MINIE = (((BADDR - 1) + intrinsics::IDNINT(save.DATA[2])) - 1);

            //
            // Transfer all data from DAF address MINIB through DAF
            // address MINIE to the target SPK segment.
            //
            save.REMAIN = ((save.MINIE - save.MINIB) + 1);

            save.BUFBAS = (save.MINIB - 1);

            save.NREAD = intrinsics::MIN0(&[BUFSIZ, save.REMAIN]);

            while (save.REMAIN > 0) {
                DAFGDA(
                    HANDLE,
                    (save.BUFBAS + 1),
                    (save.BUFBAS + save.NREAD),
                    save.DATA.as_slice_mut(),
                    ctx,
                )?;

                DAFADA(save.DATA.as_slice(), save.NREAD, ctx)?;

                if FAILED(ctx) {
                    CHKOUT(b"SPKS19", ctx)?;
                    return Ok(());
                }

                save.REMAIN = (save.REMAIN - save.NREAD);

                save.BUFBAS = (save.BUFBAS + save.NREAD);

                save.NREAD = intrinsics::MIN0(&[BUFSIZ, save.REMAIN]);
            }
            //
            // We've copied the mini-segment at index CURIVL.
            //
            // Save the end time of this mini-segment in case
            // this one turns out NOT to be the last; in that
            // case this is the final interval's start time.
            //
            save.IVFBEG = save.IVLEND;

            //
            // Get the end time of the next interval, if there
            // is one.
            //
            save.CURIVL = (save.CURIVL + 1);

            if (save.CURIVL <= save.NINTVL) {
                save.BUFBAS = (save.IVLBAS + save.CURIVL);

                DAFGDA(
                    HANDLE,
                    (save.BUFBAS + 1),
                    (save.BUFBAS + 1),
                    std::slice::from_mut(&mut save.IVLEND),
                    ctx,
                )?;

                if FAILED(ctx) {
                    CHKOUT(b"SPKS19", ctx)?;
                    return Ok(());
                }
            }
        }
        //
        // We've transferred the middle group, if it exists, to the
        // output segment.
        //
        // If the last mini-segment we transferred isn't the last of the
        // input segment, we're going to copy at least a portion of the
        // next mini-segment to the output file.
        //
        // At this point CURIVL is the index of the next interval to
        // process, if any. If CURIVL is valid, IVLEND is the interval's
        // end time.
        //

        //***********************************************************************
        //
        //     Part 4: Create the final output mini-segment, if necessary
        //
        //***********************************************************************

        if (save.CURIVL > save.NINTVL) {
            //
            // The coverage of the middle group extends to the end of
            // the coverage of the input segment. There's no more data to
            // transfer.
            //
            // FINAL is already set to .FALSE.
            //
            save.ENDIDX = save.NINTVL;
        } else {
            //
            // We're going to create one last output mini-segment.
            //
            save.FINAL = true;
            //
            // The input segment contains at least one more interpolation
            // interval, and the end time of this interval is greater than
            // END. Note that if this interval's end time were equal to
            // END, the interval would have been processed in the loop
            // above.
            //
            save.ENDIDX = save.CURIVL;

            //
            // In order to extract data from the mini-segment, we'll need
            // its address range.
            //
            DAFGDA(
                HANDLE,
                (save.PTRBAS + save.ENDIDX),
                ((save.PTRBAS + save.ENDIDX) + 1),
                save.DATA.as_slice_mut(),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"SPKS19", ctx)?;
                return Ok(());
            }

            save.MINIB = ((BADDR - 1) + intrinsics::IDNINT(save.DATA[1]));
            save.MINIE = (((BADDR - 1) + intrinsics::IDNINT(save.DATA[2])) - 1);

            //
            // Read the control area of the mini-segment.
            //
            save.BUFBAS = (save.MINIE - CTRLSZ);

            DAFGDA(
                HANDLE,
                (save.BUFBAS + 1),
                (save.BUFBAS + CTRLSZ),
                save.CONTRL.as_slice_mut(),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"SPKS19", ctx)?;
                return Ok(());
            }

            //
            // Fetch the control area parameters for the mini-segment.
            //
            save.SUBTYP = intrinsics::IDNINT(save.CONTRL[1]);
            save.WNDSIZ = intrinsics::IDNINT(save.CONTRL[2]);
            save.NPKT = intrinsics::IDNINT(save.CONTRL[3]);

            //
            // Set the packet size, which is a function of the subtype.
            //
            if ((save.SUBTYP < 0) || (save.SUBTYP >= S19NST)) {
                SETMSG(b"Unexpected SPK type 19 subtype # found in type 19 segment within mini-segment #.", ctx);
                ERRINT(b"#", save.SUBTYP, ctx);
                ERRINT(b"#", save.CURIVL, ctx);
                SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
                CHKOUT(b"SPKS19", ctx)?;
                return Ok(());
            }

            save.PKTSIZ = save.PKTSZS[save.SUBTYP];

            //
            // Determine how much of the mini-segment we need to transfer.
            // The first step is to find the last epoch less than or equal
            // to END in the mini-segment's epoch list. Let MINBEP be the
            // base address of the epoch list (that is, the start address
            // minus 1).
            //
            save.MINBEP = ((save.MINIB - 1) + (save.NPKT * save.PKTSIZ));

            //
            // Read epochs until we find one strictly greater than END.
            // The previous interval was the last one with an end time
            // less than or equal to END, so the epoch we seek should
            // exist. We have an error condition if it doesn't.
            //
            save.NREAD = intrinsics::MIN0(&[BUFSIZ, save.NPKT]);

            save.BUFBAS = save.MINBEP;

            DAFGDA(
                HANDLE,
                (save.BUFBAS + 1),
                (save.BUFBAS + save.NREAD),
                save.DATA.as_slice_mut(),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"SPKS19", ctx)?;
                return Ok(());
            }

            save.REMAIN = (save.NPKT - save.NREAD);

            //
            // The variable NREAD is the array index of the last item read
            // into the buffer on the previous read operation.
            //
            while ((save.REMAIN > 0) && (save.DATA[save.NREAD] <= END)) {
                save.BUFBAS = (save.BUFBAS + save.NREAD);

                save.NREAD = intrinsics::MIN0(&[BUFSIZ, save.REMAIN]);

                DAFGDA(
                    HANDLE,
                    (save.BUFBAS + 1),
                    (save.BUFBAS + save.NREAD),
                    save.DATA.as_slice_mut(),
                    ctx,
                )?;

                if FAILED(ctx) {
                    CHKOUT(b"SPKS19", ctx)?;
                    return Ok(());
                }

                save.REMAIN = (save.REMAIN - save.NREAD);
            }
            //
            // At this point BUFBAS - MINBEP is the number of epochs in
            // the input mini-segment we've examined before the final call
            // above to DAFGDA. If this set of epochs is non-empty, all of
            // these epochs are less than or equal to END. Note that it's
            // possible for END and BEGIN to be equal to the first epoch.
            //
            // Let EEPIDX be the index of the first epoch that is strictly
            // greater than END. As asserted above, in this branch of the
            // code, such an epoch must exist. That epoch is contained in
            // the last buffer we read.
            //
            // EEPIDX exceeds by 1 the index of the last epoch less than
            // or equal to END.
            //
            save.L = LSTLED(END, save.NREAD, save.DATA.as_slice());

            save.EEPIDX = (((save.BUFBAS - save.MINBEP) + save.L) + 1);

            //
            // EEPIDX is at least 2 and is less than or equal to NPKT.
            //
            if (save.EEPIDX < 2) {
                //
                // This code should not be reached, since getting here
                // implies the first epoch of the interval is greater than
                // END.
                //
                DAFHFN(HANDLE, &mut save.SPK, ctx)?;

                SETMSG(b"Input file: #. Segment address range: #:#. Structural error found: no epochs in final input interval exceed END. Interval index is #; END is #.", ctx);
                ERRCH(b"#", &save.SPK, ctx);
                ERRINT(b"#", BADDR, ctx);
                ERRINT(b"#", EADDR, ctx);
                ERRINT(b"#", save.ENDIDX, ctx);
                ERRDP(b"#", END, ctx);
                SIGERR(b"SPICE(SPKSTRUCTUREERROR)", ctx)?;
                CHKOUT(b"SPKS19", ctx)?;
                return Ok(());
            }

            //
            // Compute the number of pad epochs we need to maintain proper
            // interpolation behavior in the neighborhood of the epoch at
            // index EEPIDX.
            //
            if (save.DATA[save.L] == END) {
                //
                // The epochs at indices EEPIDX-1 and EEPIDX comprise
                // the first two epochs of the right half of an
                // interpolation window of size WNDSIZ. We need two
                // fewer pad epochs to complete the right half of the
                // window.
                //
                save.NPAD = ((save.WNDSIZ / 2) - 2);
            } else {
                //
                // The epoch at EEPIDX is the first of the pad.
                //
                save.NPAD = ((save.WNDSIZ / 2) - 1);
            }

            //
            // Update the final epoch index to include the pad. The index
            // cannot exceed the mini-segment's packet count.
            //
            save.EEPIDX = intrinsics::MIN0(&[save.NPKT, (save.EEPIDX + save.NPAD)]);

            //
            // EEPIDX must always exceed BEPIDX; no interpolation
            // interval may have zero length.
            //
            // When BEGIN is equal to END, and both are equal to the
            // first epoch, and the window size is 2, NPAD will be
            // -1, and EEPIDX will be 1. We don't want to allow
            // EEPIDX to be less than 2.
            //
            save.EEPIDX = intrinsics::MAX0(&[save.EEPIDX, 2]);

            //
            // EEPIDX should always be in range at this point.
            //
            if ((save.EEPIDX < 2) || (save.EEPIDX > save.NPKT)) {
                //
                // This code should not be reached, since getting here
                // implies the first epoch of the interval is greater than
                // END.
                //
                DAFHFN(HANDLE, &mut save.SPK, ctx)?;

                SETMSG(b"Input file: #. Segment address range: #:#. BEPIDX = #; EEPIDX = #; NPKT = #.Interval index is #; END is #.", ctx);
                ERRCH(b"#", &save.SPK, ctx);
                ERRINT(b"#", BADDR, ctx);
                ERRINT(b"#", EADDR, ctx);
                ERRINT(b"#", save.BEPIDX, ctx);
                ERRINT(b"#", save.EEPIDX, ctx);
                ERRINT(b"#", save.NPKT, ctx);
                ERRINT(b"#", save.ENDIDX, ctx);
                ERRDP(b"#", END, ctx);
                SIGERR(b"SPICE(SPKSTRUCTUREERROR)", ctx)?;
                CHKOUT(b"SPKS19", ctx)?;
                return Ok(());
            }

            //
            // Write the packets of the last mini-segment.
            //
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.EEPIDX;
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.BUFBAS = ((save.MINIB - 1) + ((save.I - 1) * save.PKTSIZ));

                    DAFGDA(
                        HANDLE,
                        (save.BUFBAS + 1),
                        (save.BUFBAS + save.PKTSIZ),
                        save.DATA.as_slice_mut(),
                        ctx,
                    )?;

                    DAFADA(save.DATA.as_slice(), save.PKTSIZ, ctx)?;

                    if FAILED(ctx) {
                        CHKOUT(b"SPKS19", ctx)?;
                        return Ok(());
                    }

                    save.I += m3__;
                }
            }

            //
            // Write the epochs of the last mini-segment. Save the
            // final epoch; we'll need it later.
            //
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.EEPIDX;
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.BUFBAS = (((save.MINIB - 1) + (save.NPKT * save.PKTSIZ)) + (save.I - 1));

                    DAFGDA(
                        HANDLE,
                        (save.BUFBAS + 1),
                        (save.BUFBAS + 1),
                        save.DATA.as_slice_mut(),
                        ctx,
                    )?;

                    DAFADA(save.DATA.as_slice(), 1, ctx)?;

                    if FAILED(ctx) {
                        CHKOUT(b"SPKS19", ctx)?;
                        return Ok(());
                    }

                    if (save.I == save.EEPIDX) {
                        //
                        // The current interval is the last of the output
                        // segment. The interval end must be greater than or
                        // equal to END. It's safe to simply choose the final
                        // epoch as the interval end.
                        //
                        save.IVFEND = save.DATA[1];
                    }

                    save.I += m3__;
                }
            }

            //
            // Create epoch directories for the last mini-segment.
            //
            save.MINNPK = save.EEPIDX;

            save.MINNDR = ((save.MINNPK - 1) / DIRSIZ);

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.MINNDR;
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.BUFBAS =
                        ((((save.MINIB - 1) + (save.NPKT * save.PKTSIZ)) + (save.I * DIRSIZ)) - 1);

                    DAFGDA(
                        HANDLE,
                        (save.BUFBAS + 1),
                        (save.BUFBAS + 1),
                        save.DATA.as_slice_mut(),
                        ctx,
                    )?;

                    DAFADA(save.DATA.as_slice(), 1, ctx)?;

                    if FAILED(ctx) {
                        CHKOUT(b"SPKS19", ctx)?;
                        return Ok(());
                    }

                    save.I += m3__;
                }
            }

            //
            // Finally, write out the control area for the last
            // mini-segment.
            //
            DAFADA(&[(save.SUBTYP as f64)], 1, ctx)?;
            DAFADA(&[(save.WNDSIZ as f64)], 1, ctx)?;
            DAFADA(&[(save.MINNPK as f64)], 1, ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"SPKS19", ctx)?;
                return Ok(());
            }

            //
            // Compute the size in DAF addresses of the last mini-segment.
            // This is the sum of the sizes of the packet space, the
            // epochs, the directories, and the control area.
            //
            save.MINFSZ = (((save.MINNPK * (save.PKTSIZ + 1)) + save.MINNDR) + CTRLSZ);
        }
        //
        // We're done with the final mini-segment.
        //
    }
    //
    // We've transferred all of the data we need from mini-segments at
    // indices BEGIDX : ENDIDX.

    //***********************************************************************
    //
    //     Part 5: Create segment-level data structures in the output segment
    //
    //***********************************************************************

    //
    // Write out the interval bounds for the new segment.
    //
    // Let NOIVL be the number of intervals in the output subset
    // segment.
    //
    save.NOIVL = ((save.ENDIDX - save.BEGIDX) + 1);

    //
    // The first interval start time is IV1BEG.
    //
    DAFADA(&[save.IV1BEG], 1, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SPKS19", ctx)?;
        return Ok(());
    }

    //
    // Write the remaining interval boundaries.
    //
    if (save.NOIVL == 1) {
        //
        // The final interval boundary is the stop time of
        // the first interval.
        //
        DAFADA(&[save.IV1END], 1, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"SPKS19", ctx)?;
            return Ok(());
        }
    } else {
        //
        // There are multiple output mini-segments. There is either
        // a non-empty middle group, a final mini-segment, or both.
        //
        // Set the upper bound of the interval boundary transfer loop.
        //
        if save.FINAL {
            //
            // We'll transfer all interval start times up to,
            // but not including, the final one.
            //
            save.UB = (save.NOIVL - 1);
        } else {
            //
            // There's no mini-segment following the middle group.
            //
            // Transfer all start times of the middle group, plus
            // the end time of the last interval of the middle
            // group.
            //
            save.UB = (save.NOIVL + 1);
        }

        //
        // Transfer interval boundaries from the middle group.
        //
        {
            let m1__: i32 = 2;
            let m2__: i32 = save.UB;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.BUFBAS = ((save.IVLBAS + (save.BEGIDX - 1)) + (save.I - 1));

                DAFGDA(
                    HANDLE,
                    (save.BUFBAS + 1),
                    (save.BUFBAS + 1),
                    save.DATA.as_slice_mut(),
                    ctx,
                )?;

                DAFADA(save.DATA.subarray(1), 1, ctx)?;

                if FAILED(ctx) {
                    CHKOUT(b"SPKS19", ctx)?;
                    return Ok(());
                }

                save.I += m3__;
            }
        }

        //
        // If the "final" mini-segment exists, we haven't
        // transferred its interval boundaries. Do it now.
        //
        if save.FINAL {
            //
            // The start and end times of the last output interpolation
            // interval are stored in IVFBEG and IVFEND.
            //
            // Note that IVFBEG was initialized after the first output
            // mini-segment was written, and it was updated if necessary
            // in the block of code that transferred the middle group.
            //
            DAFADA(&[save.IVFBEG], 1, ctx)?;
            DAFADA(&[save.IVFEND], 1, ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"SPKS19", ctx)?;
                return Ok(());
            }
        }
    }
    //
    // The interval boundaries have been written.
    //
    // Create an interval boundary directory for the new segment. Every
    // boundary whose index relative to BEGIDX-1 is multiple of DIRSIZ
    // becomes a directory entry, unless that entry has no successors.
    // This implies that the interval bounds to be read belong to the
    // range
    //
    //    BEGIDX + 1  :  ENDIDX - 1
    //
    // This implies that we can read all of the directory entries
    // from the input segment; we won't use as directory entries
    // the initial or final interval bounds of the output segment.
    //
    // Since the number of epoch boundaries is NOIVL + 1, the directory
    // count is
    //
    //    ( ( NOIVL + 1 ) - 1 ) / DIRSIZ
    //
    //
    save.NSDIR = (save.NOIVL / DIRSIZ);

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSDIR;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Look up the interval boundary at offset I*DIRSIZ from
            // the boundary index BEGIDX-1.
            //
            save.BUFBAS = (((save.IVLBAS + (save.BEGIDX - 1)) + (save.I * DIRSIZ)) - 1);

            DAFGDA(
                HANDLE,
                (save.BUFBAS + 1),
                (save.BUFBAS + 1),
                save.DATA.as_slice_mut(),
                ctx,
            )?;

            //
            // Write this directory entry to the output segment.
            //
            DAFADA(save.DATA.as_slice(), 1, ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"SPKS19", ctx)?;
                return Ok(());
            }

            save.I += m3__;
        }
    }

    //
    // Write out mini-segment pointers for the new segment.
    //
    // The first output mini-segment ranges from relative
    // addresses 1 : MIN1SZ.
    //
    DAFADA(&[(1 as f64)], 1, ctx)?;

    if (save.NOIVL == 1) {
        //
        // The next pointer indicates the first address after the
        // mini-segment, whether or not there is another mini-segment.
        //
        // Note that MIN1SZ was initialized after the first output
        // mini-segment was written.
        //
        DAFADA(&[((save.MIN1SZ + 1) as f64)], 1, ctx)?;
    } else {
        //
        // There are multiple output mini-segments. There is either
        // a non-empty middle group, a final mini-segment, or both.
        //
        // We can obtain from the input segment the sizes of the
        // mini-segments that were copied whole.
        //
        save.START = (save.MIN1SZ + 1);

        //
        // Set the upper bound of the mini-segment pointer transfer loop.
        //
        if save.FINAL {
            //
            // We'll transfer all mini-segment start pointers up to and
            // including the start pointer of the final output
            //
            save.UB = save.NOIVL;
        } else {
            //
            // The middle group is non-empty, and there's no mini-segment
            // following the middle group.
            //
            // Write all start pointers of the middle group, plus the end
            // pointer of the last mini-segment of the middle group. The
            // end pointer is the successor of the last DAF address
            // occupied by the mini-segment.
            //
            save.UB = (save.NOIVL + 1);
        }

        //
        // Write mini-segment pointers from the middle group.
        //
        // All of the middle group pointers of the output segment will be
        // shifted relative to the corresponding pointers of the input
        // segment. The shift reflects the sum of the sizes of the input
        // mini-segments preceding the first one from which data were
        // transferred, as well as the amount by which the first output
        // mini-segment "shrank" relative to the mini-segment from which
        // it was created. The shift equals the difference between the
        // final address of the first output mini-segment and the final
        // address of the input mini-segment at index BEGIDX.
        //
        {
            let m1__: i32 = 2;
            let m2__: i32 = save.UB;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                //
                // Look up the Ith start pointer.
                //
                save.BUFBAS = ((save.PTRBAS + (save.BEGIDX - 1)) + (save.I - 1));

                DAFGDA(
                    HANDLE,
                    (save.BUFBAS + 1),
                    (save.BUFBAS + 1),
                    save.DATA.as_slice_mut(),
                    ctx,
                )?;
                //
                // On the first pass, compute the pointer shift amount.
                //
                if (save.I == 2) {
                    save.SHIFT = ((save.MIN1SZ + 1) - intrinsics::IDNINT(save.DATA[1]));
                }

                save.START = (intrinsics::IDNINT(save.DATA[1]) + save.SHIFT);

                DAFADA(&[(save.START as f64)], 1, ctx)?;

                if FAILED(ctx) {
                    CHKOUT(b"SPKS19", ctx)?;
                    return Ok(());
                }

                save.I += m3__;
            }
        }

        //
        // If the "final" mini-segment exists, we haven't
        // transferred its end pointer. Do it now.
        //
        if save.FINAL {
            //
            // MINFSZ is the size of the final output mini-segment.
            //
            // The end pointer of the last output mini-segment is
            // START+MINFSZ. The end pointer is the successor of the last
            // DAF address of the mini-segment.
            //
            // Write the pointer.
            //
            DAFADA(&[((save.START + save.MINFSZ) as f64)], 1, ctx)?;
        }
    }

    //
    // Write the interval count and selection flag to the
    // new segment.
    //
    DAFADA(&[(save.ISEL as f64)], 1, ctx)?;
    DAFADA(&[(save.NOIVL as f64)], 1, ctx)?;

    CHKOUT(b"SPKS19", ctx)?;
    Ok(())
}
