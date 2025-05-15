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
const ND: i32 = 2;
const NI: i32 = 6;
const DIRSIZ: i32 = 100;
const BUFSIZ: i32 = (DIRSIZ + 1);
const CTRLSZ: i32 = 3;
const S19MW0: i32 = ((MAXDEG + 1) / 2);
const S19MW1: i32 = (MAXDEG + 1);
const S19MW2: i32 = (MAXDEG + 1);

struct SaveVars {
    SVBTIM: f64,
    SVETIM: f64,
    MXWNSZ: StackArray<i32, 3>,
    PKTSZS: StackArray<i32, 3>,
    SVBEG: i32,
    SVHAN: i32,
    SVMIIX: i32,
    SVMINB: i32,
    SVN: i32,
    SVNPKT: i32,
    SVPKDB: i32,
    SVPKND: i32,
    SVPKSZ: i32,
    SVSTYP: i32,
    SVWNSZ: i32,
    PASS1: bool,
    SVLAST: bool,
    SVOK: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SVBTIM: f64 = 0.0;
        let mut SVETIM: f64 = 0.0;
        let mut MXWNSZ = StackArray::<i32, 3>::new(0..=(S19NST - 1));
        let mut PKTSZS = StackArray::<i32, 3>::new(0..=(S19NST - 1));
        let mut SVBEG: i32 = 0;
        let mut SVHAN: i32 = 0;
        let mut SVMIIX: i32 = 0;
        let mut SVMINB: i32 = 0;
        let mut SVN: i32 = 0;
        let mut SVNPKT: i32 = 0;
        let mut SVPKDB: i32 = 0;
        let mut SVPKND: i32 = 0;
        let mut SVPKSZ: i32 = 0;
        let mut SVSTYP: i32 = 0;
        let mut SVWNSZ: i32 = 0;
        let mut PASS1: bool = false;
        let mut SVLAST: bool = false;
        let mut SVOK: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(S19MW0), Val::I(S19MW1), Val::I(S19MW2)].into_iter();
            MXWNSZ
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        PASS1 = true;
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(S19PS0), Val::I(S19PS1), Val::I(S19PS2)].into_iter();
            PKTSZS
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        SVBEG = -1;
        SVBTIM = 0.0;
        SVETIM = -1.0;
        SVHAN = 0;
        SVLAST = false;
        SVMIIX = -1;
        SVMINB = -1;
        SVN = -1;
        SVNPKT = -1;
        SVOK = false;
        SVPKDB = -1;
        SVPKND = -1;
        SVPKSZ = -1;
        SVSTYP = -1;
        SVWNSZ = -1;

        Self {
            SVBTIM,
            SVETIM,
            MXWNSZ,
            PKTSZS,
            SVBEG,
            SVHAN,
            SVMIIX,
            SVMINB,
            SVN,
            SVNPKT,
            SVPKDB,
            SVPKND,
            SVPKSZ,
            SVSTYP,
            SVWNSZ,
            PASS1,
            SVLAST,
            SVOK,
        }
    }
}

/// SPK, read record from segment, type 19
///
/// Read a single SPK data record from a segment of type 19
/// (ESOC/DDID Piecewise Interpolation).
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
///  DESCR    are the file handle and segment descriptor for an SPK
///           segment of type 19. The SPK file designated by HANDLE
///           must be open for read access.
///
///  ET       is an epoch for which a data record from a specific
///           segment is required. ET is expressed as seconds past
///           J2000 TDB.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RECORD   is an array of data from the specified segment which,
///           when evaluated at epoch ET, will give the state
///           (position and velocity) of the target body identified
///           by the input segment descriptor. The descriptor
///           specifies the center of motion and reference frame of
///           the state.
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
///           The packet size is a function of the type 19 subtype.
///           All packets in a record have the same size.
/// ```
///
/// # Parameters
///
/// ```text
///  See the Fortran INCLUDE file spk19.inc.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input HANDLE does not designate a loaded SPK file, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  2)  If the segment specified by DESCR is not of data type 19,
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
///  SPICE user applications normally will have no need to call this
///  routine directly. For further information, see the headers of the
///  SPICE SPK APIs
///
///     SPKEZR
///     SPKPOS
///
///  the SPK Required Reading file spk.req, and the SPICE SPK
///  tutorial.
///
///  See the SPK Required Reading file for a description of the
///  structure of a data type 19 segment.
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
///        IF ( TYPE .EQ. 19 ) THEN
///           CALL SPKR19 ( HANDLE, DESCR, ET, RECORD )
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
///  B.V. Semenov       (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.0.1, 12-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.0.0, 11-MAY-2015 (NJB)
///
///         Updated to support subtype 2.
///
/// -    SPICELIB Version 1.0.0, 14-MAR-2014 (NJB) (BVS)
/// ```
pub fn spkr19(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64; 5],
    et: f64,
    record: &mut [f64],
) -> crate::Result<()> {
    SPKR19(handle, descr, et, record, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKR19 ( SPK, read record from segment, type 19 )
pub fn SPKR19(
    HANDLE: i32,
    DESCR: &[f64],
    ET: f64,
    RECORD: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DESCR = DummyArray::new(DESCR, 1..=5);
    let mut RECORD = DummyArrayMut::new(RECORD, 1..);
    let mut BUFFER = StackArray::<f64, 101>::new(1..=BUFSIZ);
    let mut CONTRL = StackArray::<f64, 3>::new(1..=CTRLSZ);
    let mut DC = StackArray::<f64, 2>::new(1..=ND);
    let mut MINTIM = StackArray::<f64, 2>::new(1..=ND);
    let mut BADDR: i32 = 0;
    let mut BEGIDX: i32 = 0;
    let mut BUFBAS: i32 = 0;
    let mut DIRBAS: i32 = 0;
    let mut EADDR: i32 = 0;
    let mut ENDIDX: i32 = 0;
    let mut FIRST: i32 = 0;
    let mut GROUP: i32 = 0;
    let mut HIGH: i32 = 0;
    let mut I: i32 = 0;
    let mut IC = StackArray::<i32, 6>::new(1..=NI);
    let mut ISEL: i32 = 0;
    let mut IVBAS: i32 = 0;
    let mut IVBIX: i32 = 0;
    let mut IVEIX: i32 = 0;
    let mut LAST: i32 = 0;
    let mut LOW: i32 = 0;
    let mut LSIZE: i32 = 0;
    let mut MAXWND: i32 = 0;
    let mut MINIB: i32 = 0;
    let mut MINIE: i32 = 0;
    let mut MINIIX: i32 = 0;
    let mut N: i32 = 0;
    let mut NDIR: i32 = 0;
    let mut NPKDIR: i32 = 0;
    let mut NPKT: i32 = 0;
    let mut NRCPKT: i32 = 0;
    let mut NREAD: i32 = 0;
    let mut PKDBAS: i32 = 0;
    let mut PKTSIZ: i32 = 0;
    let mut REMAIN: i32 = 0;
    let mut RSIZE: i32 = 0;
    let mut SUBTYP: i32 = 0;
    let mut TIMBAS: i32 = 0;
    let mut TYPE: i32 = 0;
    let mut WNDSIZ: i32 = 0;
    let mut IVLSEL: bool = false;
    let mut PRVOK: bool = false;
    let mut SAMIVL: bool = false;
    let mut SAMSEG: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Maximum window sizes, based on subtypes:
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

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"SPKR19", ctx)?;

    //
    // Before any error checks are done, copy the status from
    // the previous call. Set the saved status variable to
    // .FALSE. here so it will be .FALSE. on exit unless this
    // call is successful.
    //
    PRVOK = save.SVOK;
    save.SVOK = false;

    //
    // Terminology: below, the phrase "base address of 'X'" refers to
    // the DAF address immediately preceding X. Base addresses simplify
    // mapping DAF array (here "array" means an array stored in
    // consecutive DAF addresses, not "segment") indices to DAF
    // addresses, since the DAF address of the Ith array element is
    // obtained by adding I to the DAF array's base address.
    //
    // Key variables:
    //
    //    Name      Meaning
    //    ----      -------
    //    BADDR     Segment begin DAF address.
    //
    //    DIRBAS    Base address of interpolation interval directory.
    //
    //    EADDR     Segment end DAF address.
    //
    //    FIRST     Index (mini-segment-relative) of first time tag in
    //              sequence transferred to to output record.
    //
    //    HIGH      Index (mini-segment-relative) of time tag following
    //              the tag at index LOW (see description below).
    //
    //    IVBIX     Index in the interpolation interval bounds array of
    //              the start time of the applicable interval.
    //
    //    IVLBAS    Base address of interpolation interval time bounds.
    //
    //    IVLSEL    Interval selection flag: this routine selects the
    //              last applicable interval if true; otherwise it
    //              selects the first applicable interval.
    //
    //    LAST      Index (mini-segment-relative) of last time tag in
    //              sequence transferred to the output record.
    //
    //    LOW       Index (mini-segment-relative) of last time tag less
    //              than the request time, or of the first time tag if
    //              this tag equals the request time.
    //
    //    MINIB,
    //    MINIE     Mini-segment begin and end DAF addresses. These
    //              addresses are absolute, not segment-relative.
    //
    //    MINIIX    Interpolation interval/mini-segment index.
    //
    //    N         Count of interpolation intervals/mini-segments.
    //
    //    NDIR      Number of interpolation interval time bounds
    //              directories.
    //
    //    NPKDIR    Number of packet directory entries for current
    //              mini-segment.
    //
    //    NPKT      Packet count for current mini-segment.
    //
    //    NRCPKT    Output record packet count. Note that this count,
    //              due to reduction of order at mini-segment
    //              boundaries, may be smaller than the window size
    //              stored in the current mini-segment.
    //
    //    PKDBAS    Base address of packet directory for current
    //              mini-segment.
    //
    //    PKTSIZ    Size of packets of current mini-segment.
    //
    //    SUBTYP    Subtype of current mini-segment.
    //
    //    TIMBAS    Base address of time tags of current mini-segment.
    //
    //    WNDSIZ    Interpolation window size of current mini-segment.
    //
    //
    // Re-used variables: the variables shown in the list below
    // are used as short-duration variables, much like loop index
    // variables; they are re-used as needed.
    //
    //    BUFBAS
    //    BUFFER
    //    GROUP
    //    NREAD
    //    REMAIN
    //
    // Start with a parameter compatibility check on the first
    // pass.
    //
    if save.PASS1 {
        if (MAXREC < MAXRSZ) {
            SETMSG(b"SPK type 19 record size may be as large as #, but SPKPVN record size (defined in spkrec.inc) is #.", ctx);
            ERRINT(b"#", MAXRSZ, ctx);
            ERRINT(b"#", MAXREC, ctx);
            SIGERR(b"SPICE(BUG)", ctx)?;
        }

        //
        // Indicate the first pass was completed.
        //
        save.PASS1 = false;
    }

    //
    // Unpack the segment descriptor, and get the start and end
    // addresses of the segment.
    //
    DAFUS(DESCR.as_slice(), 2, 6, DC.as_slice_mut(), IC.as_slice_mut());

    TYPE = IC[4];
    BADDR = IC[5];
    EADDR = IC[6];

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
        CHKOUT(b"SPKR19", ctx)?;
        return Ok(());
    }

    //
    // Decide whether we're looking at the same segment we saw on the
    // previous call, and whether the interpolation interval used on
    // that call is still applicable.
    //
    // Re-use of data from a previous call requires that the saved
    // data were set on a successful call.
    //
    SAMSEG = (((HANDLE == save.SVHAN) && (BADDR == save.SVBEG)) && PRVOK);

    //
    // Give SAMIVL an initial value. If we do have the
    // same interval, update SAMIVL to indicate this.
    //
    SAMIVL = false;

    if SAMSEG {
        //
        // We now assume that all data saved from the last
        // read of this segment are valid.
        //
        if save.SVLAST {
            //
            // We pick the last interval containing ET. For
            // all intervals but the last, ET must be
            // less than the interval end time.
            //
            if (save.SVMIIX < save.SVN) {
                SAMIVL = ((ET >= save.SVBTIM) && (ET < save.SVETIM));
            } else {
                SAMIVL = ((ET >= save.SVBTIM) && (ET <= save.SVETIM));
            }
        } else {
            //
            // We pick the first interval containing ET. For
            // all intervals but the first, ET must be
            // greater than the interval start time.
            //
            if (save.SVMIIX > 1) {
                SAMIVL = ((ET > save.SVBTIM) && (ET <= save.SVETIM));
            } else {
                SAMIVL = ((ET >= save.SVBTIM) && (ET <= save.SVETIM));
            }
        }
    }

    if (SAMSEG && SAMIVL) {
        //
        // We're looking at the same segment as last time, and the
        // interpolation interval we looked up last time is applicable
        // for the input time ET.
        //
        // Simply restore the segment and interval parameters we
        // saved from the previous lookup.
        //
        // We don't need to restore the segment start DAF address
        // BADDR, since we've already extracted it from DESCR.
        //
        // Restore
        //
        //    - The mini-segment's packet directory count
        //    - The mini-segment's packet directory base address
        //
        NPKDIR = save.SVPKND;
        PKDBAS = save.SVPKDB;

        //
        // Restore
        //
        //    - The mini-segment/interval count
        //    - The mini-segment/interval index
        //    - The mini-segment/interval start pointer
        //
        N = save.SVN;
        MINIIX = save.SVMIIX;
        MINIB = save.SVMINB;

        //
        // Restore
        //
        //    - The mini-segment subtype
        //    - The mini-segment packet size
        //    - The mini-segment packet count
        //    - The mini-segment window size
        //
        SUBTYP = save.SVSTYP;
        PKTSIZ = save.SVPKSZ;
        NPKT = save.SVNPKT;
        WNDSIZ = save.SVWNSZ;
    } else {
        //
        // The segment and interval information for the current segment
        // must be looked up.
        //
        // Perform checks on this segment.
        //
        // Make sure that this really is a type 19 data segment.
        //
        if (TYPE != 19) {
            SETMSG(
                b"You are attempting to locate type * data in a type 19 data segment.",
                ctx,
            );
            ERRINT(b"*", TYPE, ctx);
            SIGERR(b"SPICE(WRONGSPKTYPE)", ctx)?;
            CHKOUT(b"SPKR19", ctx)?;
            return Ok(());
        }

        //
        // Locate the interpolation interval that contains the request
        // time.
        //
        // Before getting started, we need to determine which interval to
        // use if the request time lies on a boundary between two
        // intervals. The segment's interval selection flag tells us how
        // to resolve this.
        //
        DAFGDA(HANDLE, (EADDR - 1), EADDR, CONTRL.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"SPKR19", ctx)?;
            return Ok(());
        }

        ISEL = intrinsics::IDNINT(CONTRL[1]);
        N = intrinsics::IDNINT(CONTRL[2]);

        IVLSEL = (ISEL == ITRUE);

        //
        // Determine the number of interval directory entries in the
        // segment. Note that for most SPK types, this computation is
        // performed by computing
        //
        //    ( N - 1 ) / DIRSIZ
        //
        // where N is the segment's epoch count.
        //
        // However the set of items in this case is a sequence
        // of N start times followed by a final stop time, so
        // the epoch count is
        //
        //    N + 1
        //
        // and the numerator in the ratio above is incremented by 1.
        //
        NDIR = (N / DIRSIZ);

        //
        // Note that the directory placement scheme always leaves
        // a non-empty set of epochs following the last directory
        // entry.
        //
        // Let DIRBAS be the base address of the interval directory.
        // We'll compute DIRBAS whether or not the interval directory
        // is non-empty.
        //
        // If the interval directory is non-empty, it spans the address
        // range
        //
        //    DIRBAS+1 : DIRBAS+NDIR
        //
        // We compute DIRBAS by starting at the end of the segment
        // and skipping over the control area, the mini-segment
        // start/stop pointers, and the interval directory itself.
        //
        DIRBAS = (((EADDR - 2) - (N + 1)) - NDIR);

        //
        // The way we search the directory depends on the treatment
        // of request times that lie on interval boundaries.
        //
        if IVLSEL {
            //
            // If there is an interval directory, search it to determine
            // the group of interval times to search next.
            //
            if (NDIR > 0) {
                //
                // Find the last directory entry *less than or equal to*
                // the request time. The directory entry *after* that one,
                // if such exists, is the one to pick.
                //
                NREAD = intrinsics::MIN0(&[NDIR, BUFSIZ]);
                BUFBAS = DIRBAS;

                //
                // Fetch the current batch of directory entries.
                //
                DAFGDA(
                    HANDLE,
                    (BUFBAS + 1),
                    (BUFBAS + NREAD),
                    BUFFER.as_slice_mut(),
                    ctx,
                )?;

                if FAILED(ctx) {
                    CHKOUT(b"SPKR19", ctx)?;
                    return Ok(());
                }

                REMAIN = (NDIR - NREAD);

                //
                // The variable NREAD always contains a positive value at
                // this point, so we can use it as an array index.
                //
                while ((REMAIN > 0) && (BUFFER[NREAD] <= ET)) {
                    BUFBAS = (BUFBAS + NREAD);
                    NREAD = intrinsics::MIN0(&[REMAIN, BUFSIZ]);
                    //
                    // Fetch the current batch of directory entries.
                    //
                    DAFGDA(
                        HANDLE,
                        (BUFBAS + 1),
                        (BUFBAS + NREAD),
                        BUFFER.as_slice_mut(),
                        ctx,
                    )?;

                    if FAILED(ctx) {
                        CHKOUT(b"SPKR19", ctx)?;
                        return Ok(());
                    }

                    REMAIN = (REMAIN - NREAD);
                }

                //
                // Count the directory entries that are less than or equal
                // to ET. The number we skipped over before the final loop
                // iteration is BUFBAS-DIRBAS. The index of the group of
                // epochs containing ET exceeds the skipped directory count
                // by 1.
                //
                GROUP = (((BUFBAS - DIRBAS) + LSTLED(ET, NREAD, BUFFER.as_slice())) + 1);
            } else {
                //
                // There's no question about which group of epochs to
                // search.
                //
                GROUP = 1;
            }

            //
            // Let IVBAS be the base address of the sequence of interval
            // time bounds.
            //
            IVBAS = (DIRBAS - (N + 1));

            //
            // Now find the index of the last interval boundary less than
            // or equal to ET. We'll need to read the current group of
            // epochs first, so compute the base of the range of addresses
            // containing this group.

            BUFBAS = (IVBAS + ((GROUP - 1) * DIRSIZ));

            //
            // Compute the number of epochs to read. Note that all groups
            // of epochs except the last have DIRSIZ elements.
            //
            REMAIN = ((N + 1) - ((GROUP - 1) * DIRSIZ));
            //
            // Note that REMAIN is always non-zero, since there's always
            // at least one epoch that exceeds the last directory entry.
            //
            NREAD = intrinsics::MIN0(&[DIRSIZ, REMAIN]);

            DAFGDA(
                HANDLE,
                (BUFBAS + 1),
                (BUFBAS + NREAD),
                BUFFER.as_slice_mut(),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"SPKR19", ctx)?;
                return Ok(());
            }

            //
            // Find the index of the first epoch greater than ET. The case
            // where ET matches the final epoch must be handled here,
            // since in this case no epoch exceeds ET.
            //
            IVEIX = (((BUFBAS - IVBAS) + LSTLED(ET, NREAD, BUFFER.as_slice())) + 1);

            IVEIX = intrinsics::MIN0(&[IVEIX, (N + 1)]);

            //
            // Backstop test:
            //
            if (IVEIX < 2) {
                SETMSG(b"IVEIX = #.", ctx);
                ERRINT(b"#", IVEIX, ctx);
                SIGERR(b"SPICE(BUG)", ctx)?;
                CHKOUT(b"SPKR19", ctx)?;
                return Ok(());
            }

            //
            // The epoch at index IVEIX is the end time of the
            // interpolation interval we'll use. The index of
            // the interval itself is IVEIX - 1.
            //
            MINIIX = (IVEIX - 1);
        } else {
            //
            // IVLSEL is .FALSE., meaning we must pick the first interval
            // containing the request time.
            //
            // If there is an interval directory, search it to determine
            // the group of interval times to search next.
            //
            if (NDIR > 0) {
                //
                // Find the last directory entry *less than* the request
                // time. The directory entry *after* that one, if such
                // exists, is the one to pick.
                //
                NREAD = intrinsics::MIN0(&[NDIR, BUFSIZ]);
                BUFBAS = DIRBAS;
                REMAIN = (NDIR - NREAD);

                //
                // Fetch the current batch of directory entries.
                //
                DAFGDA(
                    HANDLE,
                    (BUFBAS + 1),
                    (BUFBAS + NREAD),
                    BUFFER.as_slice_mut(),
                    ctx,
                )?;

                if FAILED(ctx) {
                    CHKOUT(b"SPKR19", ctx)?;
                    return Ok(());
                }

                //
                // The variable NREAD always contains a positive value at
                // this point, so we can use it as an array index.
                //
                while ((REMAIN > 0) && (BUFFER[NREAD] < ET)) {
                    BUFBAS = (BUFBAS + NREAD);
                    NREAD = intrinsics::MIN0(&[REMAIN, BUFSIZ]);
                    //
                    // Fetch the current batch of directory entries.
                    //
                    DAFGDA(
                        HANDLE,
                        (BUFBAS + 1),
                        (BUFBAS + NREAD),
                        BUFFER.as_slice_mut(),
                        ctx,
                    )?;

                    if FAILED(ctx) {
                        CHKOUT(b"SPKR19", ctx)?;
                        return Ok(());
                    }

                    REMAIN = (REMAIN - NREAD);
                }

                //
                // Count the directory entries that are less than ET. The
                // number we skipped over before the final loop iteration
                // is BUFBAS-DIRBAS. The index of the group of epochs
                // containing ET exceeds the skipped directory count by 1.
                //
                GROUP = (((BUFBAS - DIRBAS) + LSTLTD(ET, NREAD, BUFFER.as_slice())) + 1);
            } else {
                //
                // There's no question about which group of epochs to
                // search.
                //
                GROUP = 1;
            }

            //
            // Let IVBAS be the base address of the sequence of interval
            // time bounds.
            //
            IVBAS = (DIRBAS - (N + 1));

            //
            // Now find the index of the last interval epoch less than ET.
            // We'll need to read the current group of epochs first, so
            // compute the base of the range of addresses containing this
            // group.

            BUFBAS = (IVBAS + ((GROUP - 1) * DIRSIZ));

            //
            // Compute the number of epochs to read. Note that all groups
            // of epochs except the last have DIRSIZ elements.
            //
            REMAIN = ((N + 1) - ((GROUP - 1) * DIRSIZ));
            //
            // Note that REMAIN is always non-zero, since there's always
            // at least one epoch that exceeds the last directory entry.
            //
            NREAD = intrinsics::MIN0(&[DIRSIZ, REMAIN]);

            DAFGDA(
                HANDLE,
                (BUFBAS + 1),
                (BUFBAS + NREAD),
                BUFFER.as_slice_mut(),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"SPKR19", ctx)?;
                return Ok(());
            }

            //
            // Find the index of the last epoch less than ET. The case
            // where ET matches the first epoch must be handled here,
            // since in this case no epoch precedes ET.
            //
            IVBIX = ((BUFBAS - IVBAS) + LSTLTD(ET, NREAD, BUFFER.as_slice()));

            IVBIX = intrinsics::MAX0(&[IVBIX, 1]);

            //
            // Backstop test:
            //
            if (IVBIX > N) {
                SETMSG(b"IVBIX = #.", ctx);
                ERRINT(b"#", IVBIX, ctx);
                SIGERR(b"SPICE(BUG)", ctx)?;
                CHKOUT(b"SPKR19", ctx)?;
                return Ok(());
            }

            //
            // The epoch at index IVBIX is the begin time of the
            // interpolation interval we'll use. The index of the interval
            // itself is also IVBIX.
            //
            MINIIX = IVBIX;
        }
        //
        // This is the end of the IF block that handles mini-segment
        // selection for the two possible values of IVLSEL.
        //
        // Look up the begin and end pointers of the mini-segment at index
        // MINIIX. For the first N-1 mini-segments, the "end pointer"
        // of one mini-segment is the "begin" pointer of the next.
        //
        BUFBAS = (((EADDR - 2) - (N + 1)) + (MINIIX - 1));

        DAFGDA(
            HANDLE,
            (BUFBAS + 1),
            (BUFBAS + 2),
            BUFFER.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"SPKR19", ctx)?;
            return Ok(());
        }

        MINIB = ((intrinsics::IDNINT(BUFFER[1]) + BADDR) - 1);
        //
        // Note that the end of the current mini-segment
        // precedes the start of the next mini-segment by
        // one address.
        //
        MINIE = ((intrinsics::IDNINT(BUFFER[2]) + BADDR) - 2);

        //
        // Look up the time bounds of the mini-segment at index MINIIX.
        // These bounds are used quite a bit farther on, when we save
        // them for future use.
        //
        DAFGDA(
            HANDLE,
            (IVBAS + MINIIX),
            ((IVBAS + MINIIX) + 1),
            MINTIM.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"SPKR19", ctx)?;
            return Ok(());
        }

        //
        // From this point onward, we'll work with the mini-segment
        // that occupies the address range MINIB : MINIE.
        //
        // Look up the control area of the mini-segment.
        //
        DAFGDA(
            HANDLE,
            ((MINIE - CTRLSZ) + 1),
            MINIE,
            CONTRL.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"SPKR19", ctx)?;
            return Ok(());
        }

        //
        // Fetch the control area parameters for the mini-segment.
        //
        SUBTYP = intrinsics::IDNINT(CONTRL[1]);
        WNDSIZ = intrinsics::IDNINT(CONTRL[2]);
        NPKT = intrinsics::IDNINT(CONTRL[3]);

        if ((SUBTYP < 0) || (SUBTYP >= S19NST)) {
            SETMSG(
                b"Unexpected SPK type 19 subtype # found in type 19 segment within mini-segment #.",
                ctx,
            );
            ERRINT(b"#", SUBTYP, ctx);
            ERRINT(b"#", MINIIX, ctx);
            SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
            CHKOUT(b"SPKR19", ctx)?;
            return Ok(());
        }

        PKTSIZ = save.PKTSZS[SUBTYP];
        MAXWND = save.MXWNSZ[SUBTYP];

        //
        // Check the window size.
        //
        if ((WNDSIZ < 2) || (WNDSIZ > MAXWND)) {
            SETMSG(b"Window size in type 19 segment was #; must be in the range 2:# for subtype #. Mini-segment index is #.", ctx);
            ERRINT(b"#", WNDSIZ, ctx);
            ERRINT(b"#", MAXWND, ctx);
            ERRINT(b"#", SUBTYP, ctx);
            ERRINT(b"#", MINIIX, ctx);
            SIGERR(b"SPICE(INVALIDVALUE)", ctx)?;
            CHKOUT(b"SPKR19", ctx)?;
            return Ok(());
        }

        if ODD(WNDSIZ) {
            SETMSG(b"Window size in type 19 segment was #; must be even for subtype #. Mini-segment index is #.", ctx);
            ERRINT(b"#", WNDSIZ, ctx);
            ERRINT(b"#", SUBTYP, ctx);
            ERRINT(b"#", MINIIX, ctx);
            SIGERR(b"SPICE(INVALIDVALUE)", ctx)?;
            CHKOUT(b"SPKR19", ctx)?;
            return Ok(());
        }

        //
        // Compute the number of packet directory entries for
        // the current mini-segment/interval.
        //
        NPKDIR = ((NPKT - 1) / DIRSIZ);
        //
        // Compute the base address of the sequence of packet
        // directory entries for the current mini-segment/interval.
        //
        PKDBAS = ((MINIB - 1) + (NPKT * (PKTSIZ + 1)));

        //
        // The test below is done for safety. No SPICE error s
        // should ever be detected at this point.
        //
        if FAILED(ctx) {
            CHKOUT(b"SPKR19", ctx)?;
            return Ok(());
        }

        //
        // If we made it this far, we did so without a SPICE error. We
        // have valid segment parameters which can be saved for the next
        // call.
        //
        // Save
        //
        //    - The DAF handle
        //    - The segment begin DAF address
        //    - The segment's "select last/first interval" flag
        //
        save.SVHAN = HANDLE;
        save.SVBEG = BADDR;
        save.SVLAST = IVLSEL;

        //
        // Save the time bounds of the applicable mini-segment/interval.
        //
        save.SVBTIM = MINTIM[1];
        save.SVETIM = MINTIM[2];

        //
        // Save
        //
        //    - The mini-segment/interval directory count
        //    - The mini-segment/interval directory base address
        //
        save.SVPKND = NPKDIR;
        save.SVPKDB = PKDBAS;

        //
        // Save
        //
        //    - The mini-segment/interval count
        //    - The mini-segment/interval index
        //    - The mini-segment/interval start pointer
        //
        save.SVN = N;
        save.SVMIIX = MINIIX;
        save.SVMINB = MINIB;

        //
        // Save
        //
        //    - The mini-segment subtype
        //    - The mini-segment packet size
        //    - The mini-segment packet count
        //    - The mini-segment window size
        //
        save.SVSTYP = SUBTYP;
        save.SVPKSZ = PKTSIZ;
        save.SVNPKT = NPKT;
        save.SVWNSZ = WNDSIZ;
    }

    //
    // We're ready to construct the output record. The first step is to
    // identify the indices of the packets and epochs corresponding to
    // the request.
    //
    // We'll now select the set of packets that define the interpolating
    // polynomials.   We'll start out by finding the first directory
    // entry that is greater than or equal to the request epoch.  We'll
    // use the variable GROUP to indicate the set of epochs to search
    // within, once we've found the right directory entry.
    //
    if (NPKDIR == 0) {
        //
        // There's no mystery about which group of epochs to search.
        //
        GROUP = 1;
    } else {
        //
        // There's at least one directory entry. Find the first directory
        // entry whose time is greater than or equal to the request time,
        // if there is such an entry.  We'll search linearly through the
        // directory entries, reading up to DIRSIZ of them at a time.
        // Having found the correct set of directory entries, we'll
        // perform a binary search within that set for the desired entry.
        //
        BUFBAS = PKDBAS;
        NREAD = intrinsics::MIN0(&[NPKDIR, DIRSIZ]);
        REMAIN = (NPKDIR - NREAD);

        DAFGDA(
            HANDLE,
            (BUFBAS + 1),
            (BUFBAS + NREAD),
            BUFFER.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"SPKR19", ctx)?;
            return Ok(());
        }

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

            if FAILED(ctx) {
                CHKOUT(b"SPKR19", ctx)?;
                return Ok(());
            }
        }

        //
        // At this point, BUFBAS - PKDBAS is the number of directory
        // entries preceding the one contained in BUFFER(1).
        //
        GROUP = (((BUFBAS - PKDBAS) + LSTLTD(ET, NREAD, BUFFER.as_slice())) + 1);
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
        ENDIDX = intrinsics::MIN0(&[NPKT, DIRSIZ]);
    } else {
        //
        // If the group index is greater than 1, we'll include the last
        // time tag of the previous group in the set of time tags we look
        // up.  That way, the request time is bracketed by the time tag
        // set we look up.
        //
        BEGIDX = ((GROUP - 1) * DIRSIZ);
        ENDIDX = intrinsics::MIN0(&[(BEGIDX + DIRSIZ), NPKT]);
    }

    TIMBAS = (PKDBAS - NPKT);

    DAFGDA(
        HANDLE,
        (TIMBAS + BEGIDX),
        (TIMBAS + ENDIDX),
        BUFFER.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"SPKR19", ctx)?;
        return Ok(());
    }

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
    // Unlike SPK types 8, 9, 12, and 13, for type 19 we allow the
    // window size to shrink when the window must be truncated due to
    // proximity to an interval boundary.
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
    RSIZE = intrinsics::MIN0(&[(WNDSIZ / 2), ((NPKT - HIGH) + 1)]);

    //
    // The actual window size is simply the sum of LSIZE and RSIZE.
    //
    NRCPKT = (LSIZE + RSIZE);

    //
    // FIRST and LAST are the endpoints of the range of indices of
    // time tags (and packets) we'll collect in the output record.
    //
    FIRST = ((LOW - LSIZE) + 1);

    LAST = ((FIRST + NRCPKT) - 1);

    //
    // We're ready to construct the output record.
    //
    // Put the subtype and window size into the output record.
    //
    RECORD[1] = SUBTYP as f64;

    RECORD[2] = NRCPKT as f64;

    //
    // Read the packets.
    //
    DAFGDA(
        HANDLE,
        (MINIB + ((FIRST - 1) * PKTSIZ)),
        ((MINIB + (LAST * PKTSIZ)) - 1),
        RECORD.subarray_mut(3),
        ctx,
    )?;

    //
    // Finally, add the epochs to the output record.
    // Read the sequence of time tags.
    //
    BUFBAS = (((MINIB - 1) + (NPKT * PKTSIZ)) + (FIRST - 1));

    DAFGDA(
        HANDLE,
        (BUFBAS + 1),
        (BUFBAS + NRCPKT),
        RECORD.subarray_mut((3 + (NRCPKT * PKTSIZ))),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"SPKR19", ctx)?;
        return Ok(());
    }

    //
    // The call was successful. Record this fact so that saved
    // interval data are available for re-use.
    //
    save.SVOK = true;

    CHKOUT(b"SPKR19", ctx)?;
    Ok(())
}
