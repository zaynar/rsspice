//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

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
const MAXDEG: i32 = 23;
const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const C06TP0: i32 = 0;
const C06TP1: i32 = (C06TP0 + 1);
const C06TP2: i32 = (C06TP1 + 1);
const C06TP3: i32 = (C06TP2 + 1);
const C06NST: i32 = 4;
const C06PS0: i32 = 8;
const C06PS1: i32 = 4;
const C06PS2: i32 = 14;
const C06PS3: i32 = 7;
const C06MXZ: i32 = C06PS2;
const C06MNZ: i32 = C06PS1;
const MAXRSZ: i32 = (4 + ((MAXDEG + 1) * (C06PS3 + 1)));
const ND: i32 = 2;
const NI: i32 = 6;
const DIRSIZ: i32 = 100;
const BUFSIZ: i32 = (DIRSIZ + 1);
const CTRLSZ: i32 = 4;
const NSGPAR: i32 = 2;
const C06MW0: i32 = ((MAXDEG + 1) / 2);
const C06MW1: i32 = (MAXDEG + 1);
const C06MW2: i32 = ((MAXDEG + 1) / 2);
const C06MW3: i32 = (MAXDEG + 1);

struct SaveVars {
    SVBTIM: f64,
    SVETIM: f64,
    SVRATE: f64,
    MXWNSZ: StackArray<i32, 4>,
    PKTSZS: StackArray<i32, 4>,
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
    SVFND: bool,
    SVLAST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SVBTIM: f64 = 0.0;
        let mut SVETIM: f64 = 0.0;
        let mut SVRATE: f64 = 0.0;
        let mut MXWNSZ = StackArray::<i32, 4>::new(0..=(C06NST - 1));
        let mut PKTSZS = StackArray::<i32, 4>::new(0..=(C06NST - 1));
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
        let mut SVFND: bool = false;
        let mut SVLAST: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(C06MW0),
                Val::I(C06MW1),
                Val::I(C06MW2),
                Val::I(C06MW3),
            ]
            .into_iter();
            MXWNSZ
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        PASS1 = true;
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(C06PS0),
                Val::I(C06PS1),
                Val::I(C06PS2),
                Val::I(C06PS3),
            ]
            .into_iter();
            PKTSZS
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        SVBEG = -1;
        SVBTIM = 0.0;
        SVETIM = -1.0;
        SVFND = false;
        SVHAN = 0;
        SVLAST = false;
        SVMIIX = -1;
        SVMINB = -1;
        SVN = -1;
        SVNPKT = -1;
        SVPKDB = -1;
        SVPKND = -1;
        SVPKSZ = -1;
        SVRATE = -1.0;
        SVSTYP = -1;
        SVWNSZ = -1;

        Self {
            SVBTIM,
            SVETIM,
            SVRATE,
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
            SVFND,
            SVLAST,
        }
    }
}

/// C-kernel, read record from segment, type 6
///
/// Read a single CK data record from a segment of type 6
/// (ESOC/DDID Piecewise Interpolation).
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
///  DESCR    are the file handle and segment descriptor for a CK
///           segment of type 6.
///
///  SCLKDP   is an encoded spacecraft clock time indicating the
///           epoch for which pointing is desired.
///
///  TOL      is a time tolerance, measured in the same units as
///           encoded spacecraft clock.
///
///           When SCLKDP falls between the start time of one of
///           the mini-segment intervals and the last time tag of
///           that interval, the tolerance has no effect because
///           pointing will be returned at the request time.
///
///           However, if the request time falls within a coverage
///           gap in one of the intervals, or outside of any
///           interval, then the tolerance is used to determine if
///           pointing should be returned at the closest epoch for
///           which pointing is available. This epoch is either an
///           interval's start time or the smaller of an interval's
///           end time and its last time tag.
///
///
///  NEEDAV   is .TRUE. if angular velocity is requested. If the
///           input segment descriptor indicates angular velocity
///           is absent, the error SPICE(NOAVDATA) is signaled.
///
///           Note: although all subtypes of type 6 records either
///           contain or compute angular velocity, a CK creator may
///           choose to indicate that the provided angular velocity
///           data are not valid; this can be done by setting the
///           segment descriptor angular velocity flag to .FALSE.
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
///
///
///  FOUND    is a logical flag indicating whether data were found.
///           If NEEDAV is .FALSE., data will be found if the
///           request time is within TOL ticks of a time for which
///           the segment provides data. If NEEDAV is .TRUE., the
///           segment's angular velocity flag must also be set in
///           order for data to be found.
///
///           A type 6 segment provides data for times that are
///           between its descriptor time bounds and that are
///           within the coverage region of a mini-segment
///           interval. The coverage region of a mini-segment
///           interval extends from its start time to the lesser of
///           its stop time and its last time tag.
/// ```
///
/// # Exceptions
///
/// ```text
///  This routine roughly follows the pattern established in the
///  lower-numbered CK data type readers of not explicitly performing
///  error diagnoses. In particular, the C-kernel from which data are
///  read is assumed to be valid in most respects. The few exceptions
///  that are handled here are listed below.
///
///  1)  If the input HANDLE does not designate a loaded CK file, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  2)  If the segment specified by DESCR is not of data type 6, the
///      error SPICE(WRONGCKTYPE) is signaled.
///
///  3)  If the input SCLK value is not within TOL ticks of a time
///      for which the segment provides data, FOUND is set to .FALSE.
///      and the output record is undefined.
///
///  4)  If the window size is non-positive or greater than the
///      maximum allowed value, the error SPICE(INVALIDVALUE) is
///      signaled.
///
///  5)  If the window size is not compatible with the segment
///      subtype, the error SPICE(INVALIDVALUE) is signaled.
///
///  6)  If the segment subtype is not recognized, the error
///      SPICE(INVALIDSUBTYPE) is signaled.
///
///  7)  If the tolerance is negative, the error SPICE(NEGATIVETOL) is
///      signaled.
///
///  8)  If an error occurs while trying to read data from a C-kernel,
///      the error is signaled by a routine in the call tree of this
///      routine.
///
///  9)  If the input segment descriptor indicates that angular
///      velocity data are not present, and if the input flag NEEDAV
///      is set to .TRUE., the error SPICE(NOAVDATA) is signaled.
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
///  structure of a data type 6 segment.
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
///        IF ( TYPE .EQ. 6 ) THEN
///
///           CALL CKR06 ( HANDLE, DESCR, SCLKDP, TOL, NEEDAV,
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
///  1)  Correctness of the C-kernel read by this routine is
///      assumed.
///
///  2)  Correctness of inputs must be ensured by the caller of
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
/// -    SPICELIB Version 1.0.1, 06-JUL-2021 (NJB) (JDR)
///
///         Corrected code example: removed comment character preceding
///         CKBSS call. Added note regarding difference between this
///         search and those performed by the CK reader APIs CKGP and
///         CKGPAV.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 14-MAR-2014 (NJB) (BVS)
/// ```
pub fn ckr06(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64; 5],
    sclkdp: f64,
    tol: f64,
    needav: bool,
    record: &mut [f64],
    found: &mut bool,
) -> crate::Result<()> {
    CKR06(
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

//$Procedure CKR06 ( C-kernel, read record from segment, type 6 )
pub fn CKR06(
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
    let mut BUFFER = StackArray::<f64, 101>::new(1..=BUFSIZ);
    let mut CONTRL = StackArray::<f64, 4>::new(1..=CTRLSZ);
    let mut DC = StackArray::<f64, 2>::new(1..=ND);
    let mut GAP: f64 = 0.0;
    let mut LSTEPC: f64 = 0.0;
    let mut MINTIM = StackArray::<f64, 2>::new(1..=2);
    let mut RATE: f64 = 0.0;
    let mut T: f64 = 0.0;
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
    let mut AVFLAG: bool = false;
    let mut IVLSEL: bool = false;
    let mut LVAL: bool = false;
    let mut PRVFND: bool = false;
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

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"CKR06", ctx)?;

    //
    // Start with a parameter compatibility check on the first
    // pass.
    //
    if save.PASS1 {
        if (CKMRSZ < MAXRSZ) {
            SETMSG(b"CK type 6 record size may be as large as #, but CKPFS record size (defined in ckparam.inc) is #.", ctx);
            ERRINT(b"#", MAXRSZ, ctx);
            ERRINT(b"#", CKMRSZ, ctx);
            SIGERR(b"SPICE(BUG)", ctx)?;
        }
        //
        // Indicate the first pass was completed.
        //
        save.PASS1 = false;
    }

    //
    // No pointing found so far.
    //
    *FOUND = false;

    //
    // Let PRVFND indicate the last value of FOUND we returned. PRVFND
    // allows us to reset SVFND to .FALSE. at the start of this routine,
    // so we don't have to do this prior to every return (of which there
    // are more than 35).
    //
    PRVFND = save.SVFND;

    //
    // Set the saved value of FOUND so as to reflect the value
    // of FOUND we'll return next.
    //
    save.SVFND = false;

    //
    // "Touch" the input argument NEEDAV to suppress compiler warnings.
    //
    LVAL = TOUCHL(NEEDAV);
    LVAL = TOUCHL(LVAL);

    //
    // Unpack the segment descriptor, and get the start and end addresses
    // of the segment.
    //
    DAFUS(DESCR.as_slice(), 2, 6, DC.as_slice_mut(), IC.as_slice_mut());

    TYPE = IC[3];
    AVFLAG = (IC[4] == 1);
    BADDR = IC[5];
    EADDR = IC[6];

    //
    // Check whether angular velocity data are requested but not
    // available.
    //
    if (NEEDAV && !AVFLAG) {
        SETMSG(b"Segment descriptor indicates angular velocity data are not available, but such data were requested.", ctx);
        SIGERR(b"SPICE(NOAVDATA)", ctx)?;
        CHKOUT(b"CKR06", ctx)?;
        return Ok(());
    }

    //
    // Check the tolerance value.
    //
    if (TOL < 0.0) {
        SETMSG(b"Tolerance must be non-negative but was actually *.", ctx);
        ERRDP(b"*", TOL, ctx);
        SIGERR(b"SPICE(NEGATIVETOL)", ctx)?;
        CHKOUT(b"CKR06", ctx)?;
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
        CHKOUT(b"CKR06", ctx)?;
        return Ok(());
    }

    //
    // Set the request time to use for searching.
    //
    T = BRCKTD(SCLKDP, DC[1], DC[2]);

    //
    // From this point onward, we assume the segment was constructed
    // correctly.
    //
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
    //    DIRBAS    Base address of mini-segment interval directory.
    //
    //    EADDR     Segment end DAF address.
    //
    //    FIRST     Index (mini-segment-relative) of first time tag in
    //              sequence transferred to the output record.
    //
    //    HIGH      Index (mini-segment-relative) of time tag following
    //              the tag at index LOW (see description below).
    //
    //    IVBIX     Index in the mini-segment interval bounds array of
    //              the start time of the applicable interval.
    //
    //    IVLBAS    Base address of mini-segment interval time bounds.
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
    //    MINIIX    Mini-segment/mini-segment interval index.
    //
    //    N         Count of mini-segments.
    //
    //    NDIR      Number of mini-segment interval time bounds
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
    //
    // Decide whether we're looking at the same segment we saw on the
    // previous call, and whether the mini-segment interval used on
    // that call is still applicable.
    //
    // Re-use of data from a previous call requires that the saved
    // data were set on a successful call. Note that PRVFND can not
    // be true on the first pass.
    //
    SAMSEG = (((HANDLE == save.SVHAN) && (BADDR == save.SVBEG)) && PRVFND);

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
            // We pick the last interval containing T. For all intervals
            // but the last, T must be less than the interval end time.
            // For the last interval, T may equal the end time.
            //
            // Note that we don't bother to test for the special case
            // where the interval is not the last, there's a gap at the
            // end of the interval and T equals the last epoch of the
            // interval. In this rare case, we do not reuse the old
            // interval data, even though it would be possible to
            // add code to do so.
            //
            if (save.SVMIIX < save.SVN) {
                SAMIVL = ((T >= save.SVBTIM) && (T < save.SVETIM));
            } else {
                SAMIVL = ((T >= save.SVBTIM) && (T <= save.SVETIM));
            }
        } else {
            //
            // We pick the first interval containing T. For all intervals
            // but the first, T must be greater than the interval start
            // time. For the first interval, T may equal the start time.
            //
            if (save.SVMIIX > 1) {
                SAMIVL = ((T > save.SVBTIM) && (T <= save.SVETIM));
            } else {
                SAMIVL = ((T >= save.SVBTIM) && (T <= save.SVETIM));
            }
        }
    }

    if (SAMSEG && SAMIVL) {
        //
        // We're looking at the same segment as last time, and the
        // mini-segment interval we looked up last time is applicable
        // for the input time T.
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
        //    - The mini-segment interpolation window size
        //    - The mini-segment clock rate
        //
        SUBTYP = save.SVSTYP;
        PKTSIZ = save.SVPKSZ;
        NPKT = save.SVNPKT;
        WNDSIZ = save.SVWNSZ;
        RATE = save.SVRATE;
    } else {
        //
        // The segment and interval information for the current segment
        // must be looked up.
        //
        // Perform checks on this segment.
        //
        // Make sure that this really is a type 06 data segment.
        //
        if (TYPE != 6) {
            SETMSG(
                b"You are attempting to locate type * data in a type 6 data segment.",
                ctx,
            );
            ERRINT(b"*", TYPE, ctx);
            SIGERR(b"SPICE(WRONGCKTYPE)", ctx)?;
            CHKOUT(b"CKR06", ctx)?;
            return Ok(());
        }

        //
        // Locate the mini-segment interval that contains the request
        // time. If the request time lies a common boundary of two
        // intervals, the choice of interval is determined by the
        // interval selection flag.
        //
        // Before getting started, we need to determine which interval to
        // use if the request time lies on a boundary between two
        // intervals. The segment's interval selection flag tells us how
        // to resolve this.
        //
        DAFGDA(HANDLE, (EADDR - 1), EADDR, CONTRL.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"CKR06", ctx)?;
            return Ok(());
        }

        ISEL = intrinsics::IDNINT(CONTRL[1]);
        N = intrinsics::IDNINT(CONTRL[2]);

        IVLSEL = (ISEL == ITRUE);

        //
        // Determine the number of interval directory entries in the
        // segment. Note that for most CK types, this computation is
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
        // a non-empty group of epochs following the last directory
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
        DIRBAS = (((EADDR - NSGPAR) - (N + 1)) - NDIR);

        //
        // The way we search the directory depends on the treatment
        // of request times that lie on interval boundaries.
        //
        if IVLSEL {
            //
            // We must pick the latest interval containing the request
            // time.
            //
            // The stop time of the interval we seek is the first interval
            // boundary strictly greater than T, unless T is the stop time
            // of the final interval.
            //
            // We want to find the group of interval boundaries containing
            // the stop time of the interval containing T. There are
            // NDIR+1 such groups; all but the last have a directory entry
            // that coincides with the final epoch of the group. We'll use
            // the variable GROUP as the group index.
            //
            // If there is an interval directory, search it to determine
            // the group of interval times to search next.
            //
            if (NDIR == 0) {
                //
                // There's no question about which group of epochs to
                // search.
                //
                GROUP = 1;
            } else {
                //
                // The index of the group we seek is the index of the first
                // directory entry that is greater than T, if such an entry
                // exists. If there's no such entry, the group we seek is
                // the final one.
                //
                // Find the last directory entry less than or equal to
                // the request time. The directory entry after that one,
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
                    CHKOUT(b"CKR06", ctx)?;
                    return Ok(());
                }

                REMAIN = (NDIR - NREAD);
                //
                // The variable NREAD always contains a positive value at
                // this point, so we can use it as an array index.
                //
                while ((REMAIN > 0) && (BUFFER[NREAD] <= T)) {
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
                        CHKOUT(b"CKR06", ctx)?;
                        return Ok(());
                    }

                    REMAIN = (REMAIN - NREAD);
                }
                //
                // Count the directory entries that are less than or equal
                // to T. The number we skipped over before the final loop
                // iteration is BUFBAS-DIRBAS; the number of buffered
                // entries we're skipping is the number of entries that are
                // less than or equal to T. The index of the group of
                // epochs containing T exceeds the skipped directory count
                // by 1.
                //
                GROUP = (((BUFBAS - DIRBAS) + LSTLED(T, NREAD, BUFFER.as_slice())) + 1);
                //
                // GROUP is in the range 1 : NDIR+1.
                //
            }

            //
            // Let IVBAS be the base address of the sequence of interval
            // time bounds.
            //
            IVBAS = (DIRBAS - (N + 1));
            //
            // Now find the index of the last interval boundary less than
            // or equal to T. We'll need to read the current group of
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
                CHKOUT(b"CKR06", ctx)?;
                return Ok(());
            }

            //
            // Find the index of the first epoch greater than T; this is
            // the number of epochs that are less than or equal to T, plus
            // 1. The case where T matches the final epoch must be handled
            // here, since in this case no epoch exceeds T.
            //
            IVEIX = (((BUFBAS - IVBAS) + LSTLED(T, NREAD, BUFFER.as_slice())) + 1);

            IVEIX = intrinsics::MIN0(&[IVEIX, (N + 1)]);

            //
            // Backstop test:
            //
            if (IVEIX < 2) {
                SETMSG(b"IVEIX = #.", ctx);
                ERRINT(b"#", IVEIX, ctx);
                SIGERR(b"SPICE(BUG)", ctx)?;
                CHKOUT(b"CKR06", ctx)?;
                return Ok(());
            }

            //
            // The epoch at index IVEIX is the end time of the
            // mini-segment interval we'll use. The index of
            // the interval itself is IVEIX - 1.
            //
            MINIIX = (IVEIX - 1);
        } else {
            //
            // IVLSEL is .FALSE., meaning we must pick the first interval
            // containing the request time.

            // The start time of the interval we seek is the last interval
            // boundary strictly less than T, unless T is the start time
            // of the first interval. The stop time of this interval is
            // the first boundary greater than or equal to T.
            //
            // We want to find the group of interval boundaries containing
            // the stop time of the interval containing T. There are
            // NDIR+1 such groups; all but the last have a directory entry
            // that coincides with the final epoch of the group. We'll use
            // the variable GROUP as the group index.
            //
            // If there is an interval directory, search it to determine
            // the group of interval times to search next.
            //
            if (NDIR == 0) {
                //
                // There's no question about which group of epochs to
                // search.
                //
                GROUP = 1;
            } else {
                //
                // Find the last directory entry strictly less than the
                // request time. The directory entry after that one, if
                // such exists, is the one to pick.
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
                    CHKOUT(b"CKR06", ctx)?;
                    return Ok(());
                }
                //
                // The variable NREAD always contains a positive value at
                // this point, so we can use it as an array index.
                //
                while ((REMAIN > 0) && (BUFFER[NREAD] < T)) {
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
                        CHKOUT(b"CKR06", ctx)?;
                        return Ok(());
                    }

                    REMAIN = (REMAIN - NREAD);
                }
                //
                // Count the directory entries that are less than T. The
                // number we skipped over before the final loop iteration
                // is BUFBAS-DIRBAS; the number of buffered entries we're
                // skipping is the number of entries that are less than T.
                // The index of the group of epochs containing T exceeds
                // the skipped directory count by 1.
                //
                GROUP = (((BUFBAS - DIRBAS) + LSTLTD(T, NREAD, BUFFER.as_slice())) + 1);
                //
                // GROUP is in the range 1 : NDIR+1.
                //
            }

            //
            // Let IVBAS be the base address of the sequence of interval
            // time bounds.
            //
            IVBAS = (DIRBAS - (N + 1));

            //
            // Now find the index of the last interval boundary epoch less
            // than T. We'll need to read the current group of epochs
            // first, so compute the base of the range of addresses
            // containing this group.
            //
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
                CHKOUT(b"CKR06", ctx)?;
                return Ok(());
            }

            //
            // Find the index of the last interval boundary less than T.
            // The case where T matches the first boundary must be handled
            // here, since in this case no boundary precedes T.
            //
            IVBIX = ((BUFBAS - IVBAS) + LSTLTD(T, NREAD, BUFFER.as_slice()));

            IVBIX = intrinsics::MAX0(&[IVBIX, 1]);

            //
            // Backstop test:
            //
            if (IVBIX > N) {
                SETMSG(b"IVBIX = #.", ctx);
                ERRINT(b"#", IVBIX, ctx);
                SIGERR(b"SPICE(BUG)", ctx)?;
                CHKOUT(b"CKR06", ctx)?;
                return Ok(());
            }

            //
            // The epoch at index IVBIX is the begin time of the
            // mini-segment interval we'll use.
            //
            MINIIX = IVBIX;
        }
        //
        // This is the end of the IF block that handles mini-segment
        // selection for the two possible values of IVLSEL.
        //
        // If the mini-segment we just found has a gap, and if TOL is
        // positive, it's possible that the mini-segment we want actually
        // is the successor of the one at index MINIIX. We'll check this
        // by finding the last epoch of the mini-segment we just
        // identified.
        //
        // Look up the begin and end pointers of the mini-segment at index
        // MINIIX. For the first N-1 mini-segments, the "end pointer"
        // of one mini-segment is the "begin" pointer of the next.
        //
        BUFBAS = (((EADDR - NSGPAR) - (N + 1)) + (MINIIX - 1));

        DAFGDA(
            HANDLE,
            (BUFBAS + 1),
            (BUFBAS + 2),
            BUFFER.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"CKR06", ctx)?;
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
            CHKOUT(b"CKR06", ctx)?;
            return Ok(());
        }

        //
        // Fetch the control area parameters for the mini-segment.
        //
        RATE = CONTRL[1];
        SUBTYP = intrinsics::IDNINT(CONTRL[2]);
        WNDSIZ = intrinsics::IDNINT(CONTRL[3]);
        NPKT = intrinsics::IDNINT(CONTRL[4]);

        //
        // Compute the directory count for the mini-segment.
        //
        NPKDIR = ((NPKT - 1) / DIRSIZ);

        //
        // The last epoch of the mini-segment precedes the epoch
        // directories and the control area. Look up this epoch.
        //
        BUFBAS = (((MINIE - CTRLSZ) - NPKDIR) - 1);

        DAFGDA(
            HANDLE,
            (BUFBAS + 1),
            (BUFBAS + 1),
            std::slice::from_mut(&mut LSTEPC),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"CKR06", ctx)?;
            return Ok(());
        }

        //
        // Determine whether the request time is in a gap.
        //
        if (T > LSTEPC) {
            //
            // Yep, T lies in a gap. But we may still be able to
            // find data for this request, if the lookup tolerance
            // is positive.
            //
            if (TOL == 0.0) {
                //
                // We're out of luck. We can't find pointing for this
                // request. FOUND is already .FALSE., so just return.
                //
                CHKOUT(b"CKR06", ctx)?;
                return Ok(());
            } else {
                //
                // Determine the distance of T from the nearest epochs.
                //
                // Look up the time bounds of the mini-segment at index
                // MINIIX.
                //
                DAFGDA(
                    HANDLE,
                    (IVBAS + MINIIX),
                    ((IVBAS + MINIIX) + 1),
                    MINTIM.as_slice_mut(),
                    ctx,
                )?;

                if FAILED(ctx) {
                    CHKOUT(b"CKR06", ctx)?;
                    return Ok(());
                }

                //
                // See whether T is close enough to a stored epoch for
                // us to find pointing. If not, return now.
                //
                if (((T - LSTEPC) > TOL) && ((MINTIM[2] - T) > TOL)) {
                    //
                    // We can't find pointing for T. FOUND is already
                    // .FALSE., so just return.
                    //
                    CHKOUT(b"CKR06", ctx)?;
                    return Ok(());
                }

                //
                // Continue to look for pointing usable for time T.
                //
                if (MINIIX == N) {
                    //
                    // We're looking at the final mini-segment. If
                    // T is close enough to LSTEPC, we can find
                    // pointing.
                    //
                    if ((T - LSTEPC) <= TOL) {
                        //
                        // We're going to carry on using the current
                        // mini-segment. We'll update T to be the last epoch
                        // of this mini-segment.
                        //
                        T = LSTEPC;
                    } else {
                        //
                        // T is too far from LSTEPC. We're done. FOUND is
                        // already .FALSE., so just return.
                        //
                        CHKOUT(b"CKR06", ctx)?;
                        return Ok(());
                    }
                } else {
                    //
                    // There's a successor to the current interval. Determine
                    // which interval contains an epoch closest to T.
                    //
                    // Compute the size of the gap at the right end of the
                    // interior of the current interval.
                    //
                    GAP = (MINTIM[2] - LSTEPC);

                    if ((T - LSTEPC) <= (GAP / 2 as f64)) {
                        //
                        // T is closer to LSTEPC than the start time of the
                        // next interval. We're going to carry on using the
                        // current mini-segment. We'll update T to be the
                        // last epoch of this mini-segment.
                        //
                        T = LSTEPC;
                    } else {
                        //
                        // T is closer to the start time of the next interval
                        // than to LSTEPC. than the start time of the next
                        // interval. We're going to use the next
                        // mini-segment.
                        //
                        MINIIX = (MINIIX + 1);

                        //
                        // Update the mini-segment parameters we already
                        // found, since these have been superseded.
                        //
                        // The mini-segment pointers:
                        //
                        BUFBAS = (((EADDR - NSGPAR) - (N + 1)) + (MINIIX - 1));

                        DAFGDA(
                            HANDLE,
                            (BUFBAS + 1),
                            (BUFBAS + 2),
                            BUFFER.as_slice_mut(),
                            ctx,
                        )?;

                        if FAILED(ctx) {
                            CHKOUT(b"CKR06", ctx)?;
                            return Ok(());
                        }

                        MINIB = ((intrinsics::IDNINT(BUFFER[1]) + BADDR) - 1);
                        //
                        // Note that the end of the current mini-segment
                        // precedes the start of the next mini-segment by one
                        // address.
                        //
                        MINIE = ((intrinsics::IDNINT(BUFFER[2]) + BADDR) - 2);

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
                            CHKOUT(b"CKR06", ctx)?;
                            return Ok(());
                        }

                        //
                        // Fetch the control area parameters for the
                        // mini-segment.
                        //
                        RATE = intrinsics::IDNINT(CONTRL[1]) as f64;
                        SUBTYP = intrinsics::IDNINT(CONTRL[2]);
                        WNDSIZ = intrinsics::IDNINT(CONTRL[3]);
                        NPKT = intrinsics::IDNINT(CONTRL[4]);
                        //
                        // Since we have new mini-segment parameters, we need
                        // to check them. We'll defer these checks until
                        // later, so we can perform one set of checks,
                        // regardless of which logic path we followed to
                        // select a mini-segment.
                        //
                        // Compute the directory count for the mini-segment.
                        //
                        NPKDIR = ((NPKT - 1) / DIRSIZ);
                        //
                        // We're going to set T to the start time of the
                        // current mini-segment interval, which is the stop
                        // time of the previous one.
                        //
                        T = MINTIM[2];
                        //
                        // We still need to look up the last epoch of the
                        // current mini-segment. We'll use this when we save
                        // the time bounds of the mini-segment.
                        //
                        BUFBAS = (((MINIE - CTRLSZ) - NPKDIR) - 1);

                        DAFGDA(
                            HANDLE,
                            (BUFBAS + 1),
                            (BUFBAS + 1),
                            std::slice::from_mut(&mut LSTEPC),
                            ctx,
                        )?;

                        if FAILED(ctx) {
                            CHKOUT(b"CKR06", ctx)?;
                            return Ok(());
                        }
                    }
                }
                //
                // At this point T is set. If we had to update the
                // mini-segment index and its parameters, we did so.
                //
            }
            //
            // We've handled the case where T lies in a gap and the
            // tolerance is non-zero.
        }
        //
        // This is the end of the block that handles the case where T lies
        // in a gap. At this point, the following items are set:
        //
        //    T
        //    MINIIX
        //    MINIB
        //    MINIE
        //    SUBTYP
        //    WNDSIZ
        //    NPKT
        //    NPKDIR
        //    RATE
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
            CHKOUT(b"CKR06", ctx)?;
            return Ok(());
        }

        //
        // From this point onward, we'll work with the mini-segment
        // that occupies the address range MINIB : MINIE.
        //
        // Set the packet size, which is a function of the subtype.
        // Also set the maximum window size. First check the subtype,
        // which will be used as an array index.
        //
        if ((SUBTYP < 0) || (SUBTYP >= C06NST)) {
            SETMSG(
                b"Unexpected CK type 6 subtype # found in type 06 segment within mini-segment #.",
                ctx,
            );
            ERRINT(b"#", SUBTYP, ctx);
            ERRINT(b"#", MINIIX, ctx);
            SIGERR(b"SPICE(INVALIDSUBTYPE)", ctx)?;
            CHKOUT(b"CKR06", ctx)?;
            return Ok(());
        }

        PKTSIZ = save.PKTSZS[SUBTYP];
        MAXWND = save.MXWNSZ[SUBTYP];

        //
        // Check the window size.
        //
        if ((WNDSIZ < 2) || (WNDSIZ > MAXWND)) {
            SETMSG(b"Window size in type 6 segment was #; must be in the range 2:# for subtype #. Mini-segment index is #.", ctx);
            ERRINT(b"#", WNDSIZ, ctx);
            ERRINT(b"#", MAXWND, ctx);
            ERRINT(b"#", SUBTYP, ctx);
            ERRINT(b"#", MINIIX, ctx);
            SIGERR(b"SPICE(INVALIDVALUE)", ctx)?;
            CHKOUT(b"CKR06", ctx)?;
            return Ok(());
        }

        if ODD(WNDSIZ) {
            SETMSG(b"Window size in type 06 segment was #; must be even for subtype #. Mini-segment index is #.", ctx);
            ERRINT(b"#", WNDSIZ, ctx);
            ERRINT(b"#", SUBTYP, ctx);
            ERRINT(b"#", MINIIX, ctx);
            SIGERR(b"SPICE(INVALIDVALUE)", ctx)?;
            CHKOUT(b"CKR06", ctx)?;
            return Ok(());
        }

        //
        // Compute the base address of the sequence of packet
        // directory entries for the current mini-segment/interval.
        //
        PKDBAS = ((MINIB - 1) + (NPKT * (PKTSIZ + 1)));

        //
        // The test below is done for safety. No SPICE errors
        // should ever be detected at this point.
        //
        if FAILED(ctx) {
            CHKOUT(b"CKR06", ctx)?;
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
        // We don't want to indicate data availability within a gap, since
        // the re-use logic assumes data availability.
        //
        save.SVBTIM = MINTIM[1];
        save.SVETIM = intrinsics::DMIN1(&[MINTIM[2], LSTEPC]);

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
        //    - The mini-segment clock rate
        //
        save.SVSTYP = SUBTYP;
        save.SVPKSZ = PKTSIZ;
        save.SVNPKT = NPKT;
        save.SVWNSZ = WNDSIZ;
        save.SVRATE = RATE;
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
            CHKOUT(b"CKR06", ctx)?;
            return Ok(());
        }

        while ((BUFFER[NREAD] < T) && (REMAIN > 0)) {
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
                CHKOUT(b"CKR06", ctx)?;
                return Ok(());
            }
        }

        //
        // At this point, BUFBAS - PKDBAS is the number of directory
        // entries preceding the one contained in BUFFER(1).
        //
        GROUP = (((BUFBAS - PKDBAS) + LSTLTD(T, NREAD, BUFFER.as_slice())) + 1);
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
        CHKOUT(b"CKR06", ctx)?;
        return Ok(());
    }

    //
    // Find two adjacent epochs bounding the request epoch.  The request
    // time cannot be greater than all of epochs in the group, and it
    // cannot precede the first element of the group.
    //
    I = LSTLTD(T, ((ENDIDX - BEGIDX) + 1), BUFFER.as_slice());

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
    // Now select the set of packets used for interpolation. Note
    // that the window size is known to be even.
    //
    // For CK type 6 we allow the window size to shrink when the window
    // must be truncated due to proximity to an interval boundary.
    //
    // The nominal bracketing epochs we've found are the (WNDSIZ/2)nd
    // and (WNDSIZ/2 + 1)st of the interpolating set.  If the
    // request time is too close to one end of the coverage interval,
    // we reduce the window size, after which one endpoint of the
    // window will coincide with an endpoint of the coverage interval.
    //
    // Let LSIZE be the size of the "left half" of the window: the size
    // set of window epochs to the left of the request time. We want
    // this size to be WNDSIZ/2, but if not enough packets are
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
    // The fourth element is the nominal SCLK rate.
    //
    RECORD[1] = T;
    RECORD[2] = SUBTYP as f64;
    RECORD[3] = NRCPKT as f64;
    RECORD[4] = RATE;

    //
    // Read the packets.
    //
    DAFGDA(
        HANDLE,
        (MINIB + ((FIRST - 1) * PKTSIZ)),
        ((MINIB + (LAST * PKTSIZ)) - 1),
        RECORD.subarray_mut((CTRLSZ + 1)),
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
        RECORD.subarray_mut(((CTRLSZ + (NRCPKT * PKTSIZ)) + 1)),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"CKR06", ctx)?;
        return Ok(());
    }
    //
    // Indicate pointing was found.
    //
    *FOUND = true;
    save.SVFND = true;

    CHKOUT(b"CKR06", ctx)?;
    Ok(())
}
