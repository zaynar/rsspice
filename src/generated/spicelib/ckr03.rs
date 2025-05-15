//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const DIRSIZ: i32 = 100;
const BUFSIZ: i32 = 100;
const ND: i32 = 2;
const NI: i32 = 6;
const QSIZ: i32 = 4;
const QAVSIZ: i32 = 7;
const DTYPE: i32 = 3;

struct SaveVars {
    LBEG: i32,
    LEND: i32,
    LHAND: i32,
    PREVS: f64,
    PREVN: f64,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut LBEG: i32 = 0;
        let mut LEND: i32 = 0;
        let mut LHAND: i32 = 0;
        let mut PREVS: f64 = 0.0;
        let mut PREVN: f64 = 0.0;

        PREVS = -1.0;
        PREVN = -1.0;
        LHAND = 0;
        LBEG = -1;
        LEND = -1;

        Self {
            LBEG,
            LEND,
            LHAND,
            PREVS,
            PREVN,
        }
    }
}

/// C-kernel, read pointing record, data type 3
///
/// Read a pointing record from a CK segment, data type 3.
///
/// # Required Reading
///
/// * [CK](crate::required_reading::ck)
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   File handle.
///  DESCR      I   Segment descriptor.
///  SCLKDP     I   Pointing request time.
///  TOL        I   Time tolerance.
///  NEEDAV     I   Angular velocity request flag.
///  RECORD     O   Pointing data record.
///  FOUND      O   .TRUE. when data is found.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the integer handle of the CK file containing the
///           segment.
///
///  DESCR    is the descriptor of the segment.
///
///  SCLKDP   is the encoded spacecraft clock time for which
///           pointing is being requested.
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
///  RECORD   is the record that CKE03 will evaluate to determine
///           the pointing.
///
///           When the request time falls within an interval for
///           which linear interpolation is valid, the values of
///           the two pointing instances that bracket the request
///           time are returned in RECORD as follows:
///
///              RECORD( 1  ) = Left bracketing SCLK time.
///
///              RECORD( 2  ) = lq0  \
///              RECORD( 3  ) = lq1   \    Left bracketing
///              RECORD( 4  ) = lq2   /      quaternion.
///              RECORD( 5  ) = lq3  /
///
///              RECORD( 6  ) = lav1 \     Left bracketing
///              RECORD( 7  ) = lav2       angular velocity
///              RECORD( 8  ) = lav3 /       ( optional )
///
///              RECORD( 9  ) = Right bracketing SCLK time.
///
///              RECORD( 10 ) = rq0  \
///              RECORD( 11 ) = rq1   \    Right bracketing
///              RECORD( 12 ) = rq2   /       quaternion.
///              RECORD( 13 ) = rq3  /
///
///              RECORD( 14 ) = rav1 \     Right bracketing
///              RECORD( 15 ) = rav2       angular velocity
///              RECORD( 16 ) = rav3 /       ( optional )
///
///              RECORD( 17 ) = pointing request time, SCLKDP.
///
///           The quantities lq0 - lq3 and rq0 - rq3 are the
///           components of the quaternions that represent the
///           C-matrices associated with the times that bracket
///           the requested time.
///
///           The quantities lav1, lav2, lav3 and rav1, rav2, rav3
///           are the components of the angular velocity vectors at
///           the respective bracketing times. The components of the
///           angular velocity vectors are specified relative to
///           the inertial reference frame of the segment.
///
///           If the request time does not fall within an
///           interpolation interval, but is within TOL of an
///           interval endpoint, the values of that pointing
///           instance are returned in both parts of RECORD
///           ( i.e. RECORD(1-9) and RECORD(10-16) ).
///
///  FOUND    is .TRUE. if a record was found to satisfy the pointing
///           request. This occurs when the time for which pointing
///           is requested falls inside one of the interpolation
///           intervals, or when the request time is within the
///           tolerance of an interval endpoint.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the specified handle does not belong to an open DAF file,
///      an error is signaled by a routine in the call tree of this
///      routine.
///
///  2)  If DESCR is not a valid descriptor of a segment in the CK
///      file specified by HANDLE, the results of this routine are
///      unpredictable.
///
///  3)  If the segment is not of data type 3, as specified in the
///      third integer component of the segment descriptor,
///      the error SPICE(WRONGDATATYPE) is signaled.
///
///  4)  If angular velocity data was requested but the segment
///      contains no such data, the error SPICE(NOAVDATA) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  The file containing the segment is specified by its handle and
///  should be opened for read or write access, either by CKLPF,
///  DAFOPR, or DAFOPW.
/// ```
///
/// # Particulars
///
/// ```text
///  See the CK Required Reading file for a detailed description of
///  the structure of a type 3 pointing segment.
///
///  When the time for which pointing was requested falls within an
///  interpolation interval, then FOUND will be true and RECORD will
///  contain the pointing instances in the segment that bracket the
///  request time.  CKE03 will evaluate RECORD to give pointing at
///  the request time.
///
///  However, when the request time is not within any of the
///  interpolation intervals, then FOUND will be true only if the
///  interval endpoint closest to the request time is within the
///  tolerance specified by the user. In this case both parts of
///  RECORD will contain this closest pointing instance, and CKE03
///  will evaluate RECORD to give pointing at the time associated
///  with the returned pointing instance.
/// ```
///
/// # Examples
///
/// ```text
///  The CKRnn routines are usually used in tandem with the CKEnn
///  routines, which evaluate the record returned by CKRnn to give
///  the pointing information and output time.
///
///  The following code fragment searches backwards through all of the
///  segments in a file applicable to the Mars Observer spacecraft bus
///  that are of data type 3, for a particular spacecraft clock time.
///  It then evaluates the pointing for that epoch and prints the
///  result.
///
///  The search performed here does not mimic the behavior of the CK
///  reader APIs CKGP and CKGPAV, which consider data from multiple CK
///  files, when available. See the CK Required reading for details.
///
///        CHARACTER*(20)        SCLKCH
///        CHARACTER*(20)        SCTIME
///        CHARACTER*(40)        IDENT
///
///        INTEGER               I
///        INTEGER               SC
///        INTEGER               INST
///        INTEGER               HANDLE
///        INTEGER               DTYPE
///        INTEGER               ICD      (    6 )
///
///        DOUBLE PRECISION      SCLKDP
///        DOUBLE PRECISION      TOL
///        DOUBLE PRECISION      CLKOUT
///        DOUBLE PRECISION      DESCR    (    5 )
///        DOUBLE PRECISION      DCD      (    2 )
///        DOUBLE PRECISION      RECORD   (   17 )
///        DOUBLE PRECISION      CMAT     ( 3, 3 )
///        DOUBLE PRECISION      AV       (    3 )
///
///        LOGICAL               NEEDAV
///        LOGICAL               FND
///        LOGICAL               SFND
///
///
///        SC     = -94
///        INST   = -94000
///        DTYPE  =  3
///        NEEDAV = .FALSE.
///
///  C
///  C     Load the MO SCLK kernel and the C-kernel.
///  C
///        CALL FURNSH ( 'MO_SCLK.TSC'       )
///        CALL DAFOPR ( 'MO_CK.BC',  HANDLE )
///  C
///  C     Get the spacecraft clock time. Then encode it for use
///  C     in the C-kernel.
///  C
///        WRITE (*,*) 'Enter spacecraft clock time string:'
///        READ (*,FMT='(A)') SCLKCH
///
///        CALL SCENCD ( SC, SCLKCH, SCLKDP )
///  C
///  C     Use a tolerance of 2 seconds ( half of the nominal
///  C     separation between MO pointing instances ).
///  C
///        CALL SCTIKS ( SC, '0000000002:000', TOL )
///
///  C
///  C     Search backwards from the end of the CK file through all
///  C     of the segments.
///  C
///        CALL DAFBBS ( HANDLE )
///        CALL DAFFPA ( SFND   )
///
///        FND    = .FALSE.
///
///        DO WHILE ( ( SFND ) .AND. ( .NOT. FND ) )
///
///  C
///  C        Get the segment identifier and descriptor.
///  C
///
///           CALL DAFGN ( IDENT                 )
///           CALL DAFGS ( DESCR                 )
///  C
///  C        Unpack the segment descriptor into its integer and
///  C        double precision components.
///  C
///           CALL DAFUS ( DESCR, 2, 6, DCD, ICD )
///
///  C
///  C        Determine if this segment should be processed.
///  C
///           IF ( ( INST          .EQ. ICD( 1 ) ) .AND.
///       .        ( SCLKDP + TOL  .GE. DCD( 1 ) ) .AND.
///       .        ( SCLKDP - TOL  .LE. DCD( 2 ) ) .AND.
///       .        ( DTYPE         .EQ. ICD( 3 ) )      ) THEN
///
///
///              CALL CKR03 ( HANDLE, DESCR, SCLKDP, TOL, NEEDAV,
///       .                   RECORD, FND )
///
///              IF ( FND ) THEN
///
///                 CALL CKE03 (NEEDAV,RECORD,CMAT,AV,CLKOUT)
///
///                 CALL SCDECD ( SC, CLKOUT, SCTIME )
///
///                 WRITE (*,*)
///                 WRITE (*,*) 'Segment identifier: ', IDENT
///                 WRITE (*,*)
///                 WRITE (*,*) 'Pointing returned for time: ',
///       .                      SCTIME
///                 WRITE (*,*)
///                 WRITE (*,*) 'C-matrix:'
///                 WRITE (*,*)
///                 WRITE (*,*) ( CMAT(1,I), I = 1, 3 )
///                 WRITE (*,*) ( CMAT(2,I), I = 1, 3 )
///                 WRITE (*,*) ( CMAT(3,I), I = 1, 3 )
///                 WRITE (*,*)
///
///              END IF
///
///           END IF
///
///           CALL DAFFPA ( SFND )
///
///        END DO
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The file containing the segment should be opened for read
///      or write access either by CKLPF, DAFOPR, or DAFOPW.
///
///  2)  The record returned by this routine is intended to be
///      evaluated by CKE03.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  J.M. Lynch         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.2, 12-AUG-2021 (NJB) (JDR)
///
///         Updated code example to use backwards search. Added
///         note regarding difference between this search and those
///         performed by the CK reader APIs CKGP and CKGPAV.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.1.1, 22-AUG-2006 (EDW)
///
///         Replaced references to LDPOOL with references
///         to FURNSH.
///
/// -    SPICELIB Version 1.1.0, 07-SEP-2001 (EDW)
///
///         Replaced DAFRDA call with DAFGDA.
///         Added IMPLICIT NONE.
///
/// -    SPICELIB Version 1.0.0, 25-NOV-1992 (JML)
/// ```
pub fn ckr03(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64],
    sclkdp: f64,
    tol: f64,
    needav: bool,
    record: &mut [f64],
    found: &mut bool,
) -> crate::Result<()> {
    CKR03(
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

//$Procedure CKR03 ( C-kernel, read pointing record, data type 3 )
pub fn CKR03(
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

    let DESCR = DummyArray::new(DESCR, 1..);
    let mut RECORD = DummyArrayMut::new(RECORD, 1..);
    let mut ICD = StackArray::<i32, 6>::new(1..=NI);
    let mut BEG: i32 = 0;
    let mut END: i32 = 0;
    let mut NUMREC: i32 = 0;
    let mut NUMINT: i32 = 0;
    let mut NRDIR: i32 = 0;
    let mut NIDIR: i32 = 0;
    let mut GROUP: i32 = 0;
    let mut DIRLOC: i32 = 0;
    let mut REMAIN: i32 = 0;
    let mut SKIP: i32 = 0;
    let mut GRPADD: i32 = 0;
    let mut ADDR: i32 = 0;
    let mut LADDR: i32 = 0;
    let mut RADDR: i32 = 0;
    let mut PSIZ: i32 = 0;
    let mut I: i32 = 0;
    let mut N: i32 = 0;
    let mut DCD = StackArray::<f64, 2>::new(1..=ND);
    let mut BUFFER = StackArray::<f64, 100>::new(1..=BUFSIZ);
    let mut LSCLK: f64 = 0.0;
    let mut RSCLK: f64 = 0.0;
    let mut RDIFF: f64 = 0.0;
    let mut LDIFF: f64 = 0.0;
    let mut START: f64 = 0.0;
    let mut NSTART: f64 = 0.0;
    let mut FND: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    //    DIRSIZ     is the directory size.
    //
    //    BUFSIZ     is the maximum number of double precision numbers
    //               that we will read from the DAF file at one time.
    //               BUFSIZ is normally set equal to DIRSIZ.
    //
    //    ND         is the number of double precision components in an
    //               unpacked C-kernel segment descriptor.
    //
    //    NI         is the number of integer components in an unpacked
    //               C-kernel segment descriptor.
    //
    //    QSIZ       is the number of double precision numbers making up
    //               the quaternion portion of a pointing record.
    //
    //    QAVSIZ     is the number of double precision numbers making up
    //               the quaternion and angular velocity portion of a
    //               pointing record.
    //
    //    DTYPE      is the data type of the segment that this routine
    //               operates on.
    //
    //

    //
    // Local variables
    //

    //
    // Saved variables.
    //

    //
    // Initial values.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"CKR03", ctx)?;
    }

    //
    // Start off with FOUND equal to false just in case a SPICELIB error
    // is signaled and the return mode is not set to ABORT.
    //
    *FOUND = false;

    //
    // We need to look at a few of the descriptor components.
    //
    // The unpacked descriptor contains the following information
    // about the segment:
    //
    //    DCD(1)  Initial encoded SCLK
    //    DCD(2)  Final encoded SCLK
    //    ICD(1)  Instrument
    //    ICD(2)  Inertial reference frame
    //    ICD(3)  Data type
    //    ICD(4)  Angular velocity flag
    //    ICD(5)  Initial address of segment data
    //    ICD(6)  Final address of segment data
    //

    DAFUS(
        DESCR.as_slice(),
        ND,
        NI,
        DCD.as_slice_mut(),
        ICD.as_slice_mut(),
    );

    //
    // Check to make sure that the segment is type 3.
    //
    if (ICD[3] != DTYPE) {
        SETMSG(b"The segment is not a type 3 segment.  Type is #", ctx);
        ERRINT(b"#", ICD[3], ctx);
        SIGERR(b"SPICE(WRONGDATATYPE)", ctx)?;
        CHKOUT(b"CKR03", ctx)?;
        return Ok(());
    }

    //
    // Does this segment contain angular velocity?
    //
    if (ICD[4] == 1) {
        PSIZ = QAVSIZ;
    } else {
        PSIZ = QSIZ;

        if NEEDAV {
            SETMSG(b"Segment does not contain angular velocity data.", ctx);
            SIGERR(b"SPICE(NOAVDATA)", ctx)?;
            CHKOUT(b"CKR03", ctx)?;
            return Ok(());
        }
    }

    //
    // The beginning and ending addresses of the segment are in the
    // descriptor.
    //
    BEG = ICD[5];
    END = ICD[6];

    //
    // The procedure used in finding a record to satisfy the request
    // for pointing is as follows:
    //
    //    1) Find the two pointing instances in the segment that bracket
    //       the request time.
    //
    //       The pointing instance that brackets the request time on the
    //       left is defined to be the one associated with the largest
    //       time in the segment that is less than or equal to SCLKDP.
    //
    //       The pointing instance that brackets the request time on the
    //       right is defined to be the one associated with the first
    //       time in the segment greater than SCLKDP.
    //
    //       Since the times in the segment are strictly increasing the
    //       left and right bracketing pointing instances are always
    //       adjacent.
    //
    //    2) Determine if the bracketing times are in the same
    //       interpolation interval.
    //
    //    3) If they are, then pointing at the request time may be
    //       linearly interpolated from the bracketing times.
    //
    //    4) If the times that bracket the request time are not in the
    //       same interval then, since they are adjacent in the segment
    //       and since intervals begin and end at actual times, they must
    //       both be interval endpoints.  Return the pointing instance
    //       associated with the endpoint closest to the request time,
    //       provided that it is within the tolerance.
    //

    //
    // Get the number of intervals and pointing instances ( records )
    // in this segment, and from that determine the number of respective
    // directory epochs.
    //
    DAFGDA(HANDLE, (END - 1), END, BUFFER.as_slice_mut(), ctx)?;

    NUMINT = intrinsics::IDNINT(BUFFER[1]);
    NUMREC = intrinsics::IDNINT(BUFFER[2]);

    NIDIR = ((NUMINT - 1) / DIRSIZ);
    NRDIR = ((NUMREC - 1) / DIRSIZ);

    //
    // Check the FAILED flag just in case HANDLE is not attached to
    // any DAF file and the error action is not set to ABORT. You need
    // need to do this only after the first call to DAFGDA.
    //
    if FAILED(ctx) {
        CHKOUT(b"CKR03", ctx)?;
        return Ok(());
    }

    //
    // To find the times that bracket the request time we will first
    // find the greatest directory time less than the request time.
    // This will narrow down the search to a group of DIRSIZ or fewer
    // times where the Jth group is defined to contain SCLK times
    // ((J-1)*DIRSIZ + 1) through (J*DIRSIZ).
    //
    // For example if DIRSIZ = 100 then:
    //
    //                     group   first time #     last time #
    //                     -----  ---------------   ------------
    //                       1            1             100
    //                       2          101             200
    //                       .            .               .
    //                       .            .               .
    //                      10          901            1000
    //                       .            .               .
    //                       .            .               .
    //                 NRDIR+1     (NRDIR)*100+1     NUMREC
    //
    //
    // Thus if the Ith directory time is the largest one less than
    // our request time SCLKDP, then we know that:
    //
    //   SCLKS ( DIRSIZ * I ) <  SCLKDP  <= SCLKS ( DIRSIZ * (I+1) )
    //
    // where SCLKS is taken to be the array of NUMREC times associated
    // with the pointing instances.
    //
    // Therefore, at least one of the bracketing times will come from
    // the (I+1)th group.
    //

    //
    // There is only one group if there are no directory epochs.
    //
    if (NRDIR == 0) {
        GROUP = 1;
    } else {
        //
        // Compute the location of the first directory epoch.  From the
        // beginning of the segment, we need to go through all of the
        // pointing numbers (PSIZ*NUMREC of them) and then through all of
        // the NUMREC SCLK times.
        //
        DIRLOC = (BEG + ((PSIZ + 1) * NUMREC));
        //
        // Search through the directory times.  Read in as many as BUFSIZ
        // directory epochs at a time for comparison.
        //
        FND = false;
        REMAIN = NRDIR;
        GROUP = 0;

        while !FND {
            //
            // The number of records to read into the buffer.
            //
            N = intrinsics::MIN0(&[REMAIN, BUFSIZ]);

            DAFGDA(
                HANDLE,
                DIRLOC,
                ((DIRLOC + N) - 1),
                BUFFER.as_slice_mut(),
                ctx,
            )?;

            REMAIN = (REMAIN - N);
            //
            // Determine the last directory element in BUFFER that's less
            // than SCLKDP.
            //
            I = LSTLTD(SCLKDP, N, BUFFER.as_slice());

            if (I < N) {
                GROUP = ((GROUP + I) + 1);
                FND = true;
            } else if (REMAIN == 0) {
                //
                // The request time is greater than the last directory time
                // so we want the last group in the segment.
                //
                GROUP = (NRDIR + 1);
                FND = true;
            } else {
                //
                // Need to read another block of directory times.
                //
                DIRLOC = (DIRLOC + N);
                GROUP = (GROUP + N);
            }
        }
    }

    //
    // Now we know which group of DIRSIZ (or less) times to look at.
    // Out of the NUMREC SCLK times, the number that we should skip over
    // to get to the proper group is DIRSIZ * ( GROUP - 1 ).
    //
    SKIP = (DIRSIZ * (GROUP - 1));

    //
    // From this we can compute the address in the segment of the group
    // of times we want.  From the beginning, we need to pass through
    // PSIZ * NUMREC pointing numbers to get to the first SCLK time.
    // Then we skip over the number just computed above.
    //
    GRPADD = ((BEG + (NUMREC * PSIZ)) + SKIP);

    //
    // The number of times that we have to look at may be less than
    // DIRSIZ.  However many there are, go ahead and read them into the
    // buffer.
    //
    N = intrinsics::MIN0(&[DIRSIZ, (NUMREC - SKIP)]);

    DAFGDA(
        HANDLE,
        GRPADD,
        ((GRPADD + N) - 1),
        BUFFER.as_slice_mut(),
        ctx,
    )?;

    //
    // Find the largest time in the group less than or equal to the input
    // time.
    //
    I = LSTLED(SCLKDP, N, BUFFER.as_slice());

    //
    // Find the pointing instances in the segment that bracket the
    // request time and calculate the addresses for the pointing data
    // associated with these times. For cases in which the request time
    // is equal to one of the times in the segment, that time will be
    // the left bracketing time of the returned pair.
    //
    // Need to handle the cases when the request time is greater than
    // the last or less than the first time in the segment separately.
    //

    if (I == 0) {
        if (GROUP == 1) {
            //
            // The time occurs before the first time in the segment. Since
            // this time cannot possibly be in any of the intervals, the
            // first time can satisfy the request for pointing only if it
            // is within the tolerance of the request time.
            //
            if ((BUFFER[1] - SCLKDP) <= TOL) {
                RECORD[1] = BUFFER[1];
                RECORD[9] = BUFFER[1];
                //
                // Calculate the address of the quaternion and angular
                // velocity data.  Then read it from the file.
                //
                DAFGDA(HANDLE, BEG, ((BEG + PSIZ) - 1), BUFFER.as_slice_mut(), ctx)?;

                MOVED(BUFFER.as_slice(), PSIZ, RECORD.subarray_mut(2));
                MOVED(BUFFER.as_slice(), PSIZ, RECORD.subarray_mut(10));

                RECORD[17] = SCLKDP;

                *FOUND = true;
            }

            CHKOUT(b"CKR03", ctx)?;
            return Ok(());
        } else {
            //
            // The first time in the current group brackets the request
            // time on the right and the last time from the preceding
            // group brackets on the left.
            //
            RSCLK = BUFFER[1];

            RADDR = (BEG + (SKIP * PSIZ));

            DAFGDA(
                HANDLE,
                (GRPADD - 1),
                (GRPADD - 1),
                std::slice::from_mut(&mut LSCLK),
                ctx,
            )?;

            LADDR = (RADDR - PSIZ);
        }
    } else if (I == N) {
        //
        // There are two possible cases, but the same action can handle
        // both.
        //
        // 1) If this is the last group ( NRDIR + 1 ) then the request
        //    time occurs on or after the last time in the segment.
        //    In either case this last time can satisfy the request for
        //    pointing only if it is within the tolerance of the request
        //    time.
        //
        // 2) The request time is greater than or equal to the last time
        //    in this group. Since this time is the same as the (I+1)th
        //    directory time, and since the search on the directory times
        //    used a strictly less than test, we know that the request
        //    time must be equal to this time.  Just return the pointing
        //    instance associated with the request time.  ( Note that
        //    SCLKDP - BUFFER(N) will be zero in this case. )
        //

        if ((SCLKDP - BUFFER[N]) <= TOL) {
            RECORD[1] = BUFFER[N];
            RECORD[9] = BUFFER[N];
            //
            // Calculate the address of the quaternion and angular
            // velocity data.  Then read it from the file.
            //
            ADDR = (BEG + (PSIZ * ((SKIP + N) - 1)));

            DAFGDA(
                HANDLE,
                ADDR,
                ((ADDR + PSIZ) - 1),
                BUFFER.as_slice_mut(),
                ctx,
            )?;

            MOVED(BUFFER.as_slice(), PSIZ, RECORD.subarray_mut(2));
            MOVED(BUFFER.as_slice(), PSIZ, RECORD.subarray_mut(10));

            RECORD[17] = SCLKDP;

            *FOUND = true;
        }

        CHKOUT(b"CKR03", ctx)?;
        return Ok(());
    } else {
        //
        // The bracketing times are contained in this group.
        //
        LSCLK = BUFFER[I];
        RSCLK = BUFFER[(I + 1)];

        LADDR = (BEG + (((SKIP + I) - 1) * PSIZ));

        RADDR = (LADDR + PSIZ);
    }

    //
    // At this point we have the two times in the segment that bracket
    // the request time.  We also have the addresses of the pointing
    // data associated with those times. The task now is to determine
    // if the bracketing times fall in the same interval.  If so then
    // we can interpolate between them.  If they don't then return
    // pointing for whichever of the two times is closest to the
    // request time, provided that it is within the tolerance.
    //

    //
    // Find the interpolation interval that the request time is in and
    // determine if the bracketing SCLK's are both in it.
    //
    // First check if the request time falls in the same interval as
    // it did last time.  We need to make sure that we are dealing
    // with the same segment as well as the same time range.
    //
    //
    // PREVS      is the start time of the interval that satisfied
    //            the previous request for pointing.
    //
    // PREVN      is the start time of the interval that followed
    //            the interval specified above.
    //
    // LHAND      is the handle of the file that PREVS and PREVN
    //            were found in.
    //
    // LBEG,      are the beginning and ending addresses of the
    // LEND       segment in the file LHAND that PREVS and PREVN
    //            were found in.
    //
    if (((((HANDLE == save.LHAND) && (BEG == save.LBEG)) && (END == save.LEND))
        && (SCLKDP >= save.PREVS))
        && (SCLKDP < save.PREVN))
    {
        START = save.PREVS;
        NSTART = save.PREVN;
    } else {
        //
        // The START times of all of the intervals are stored in the
        // segment and a directory of every hundredth START is also
        // stored. The procedure to find the bracketing interval start
        // times is identical to the one used above for finding the
        // bracketing times.
        //
        // The directory epochs narrow down the search for the times that
        // bracket the request time to a group of DIRSIZ or fewer records.
        //

        //
        // There is only one group if there are no directory epochs.
        //
        if (NIDIR == 0) {
            GROUP = 1;
        } else {
            //
            // Compute the location of the first directory epoch.  From the
            // beginning of the segment, we need to go through all of the
            // pointing numbers (PSIZ*NUMREC of them), then through all of
            // the NUMREC SCLK times and NRDIR directory times, and then
            // finally through the NUMINT interval start times.
            //
            DIRLOC = (((BEG + ((PSIZ + 1) * NUMREC)) + NRDIR) + NUMINT);

            //
            // Locate the largest directory time less than the
            // request time SCLKDP.
            //
            // Read in as many as BUFSIZ directory epochs at a time for
            // comparison.
            //
            FND = false;
            REMAIN = NIDIR;
            GROUP = 0;

            while !FND {
                //
                // The number of records to read into the buffer.
                //
                N = intrinsics::MIN0(&[REMAIN, BUFSIZ]);

                DAFGDA(
                    HANDLE,
                    DIRLOC,
                    ((DIRLOC + N) - 1),
                    BUFFER.as_slice_mut(),
                    ctx,
                )?;

                REMAIN = (REMAIN - N);
                //
                // Determine the last directory element in BUFFER that's
                // less than SCLKDP.
                //
                I = LSTLTD(SCLKDP, N, BUFFER.as_slice());

                if (I < N) {
                    GROUP = ((GROUP + I) + 1);
                    FND = true;
                } else if (REMAIN == 0) {
                    //
                    // The request time is greater than the last directory
                    // time so we want the last group in the segment.
                    //
                    GROUP = (NIDIR + 1);
                    FND = true;
                } else {
                    //
                    // Need to read another block of directory times.
                    //
                    DIRLOC = (DIRLOC + N);
                    GROUP = (GROUP + N);
                }
            }
        }

        //
        // Now we know which group of DIRSIZ (or less) times to look at.
        // Out of the NUMINT SCLK START times, the number that we should
        // skip over to get to the proper group is DIRSIZ * ( GROUP - 1 ).
        //
        SKIP = (DIRSIZ * (GROUP - 1));

        //
        // From this we can compute the address in the segment of the
        // group of times we want.  To get to the first interval start
        // time we must pass over PSIZ * NUMREC pointing numbers, NUMREC
        // SCLK times, and NRDIR SCLK directory times.  Then we skip
        // over the number just computed above.
        //

        GRPADD = (((BEG + ((PSIZ + 1) * NUMREC)) + NRDIR) + SKIP);

        //
        // The number of times that we have to look at may be less than
        // DIRSIZ.  However many there are, go ahead and read them into
        // the buffer.
        //
        N = intrinsics::MIN0(&[DIRSIZ, (NUMINT - SKIP)]);

        DAFGDA(
            HANDLE,
            GRPADD,
            ((GRPADD + N) - 1),
            BUFFER.as_slice_mut(),
            ctx,
        )?;
        //
        // Find the index of the largest time in the group that is less
        // than or equal to the input time.
        //
        I = LSTLED(SCLKDP, N, BUFFER.as_slice());

        if (I == 0) {
            //
            // The first start time in the buffer is the start of the
            // interval following the one containing the request time.
            //
            // We don't need to check if GROUP = 1 because the case of
            // the request time occurring before the first time in the
            // segment has already been handled.
            //
            NSTART = BUFFER[1];

            ADDR = (GRPADD - 1);

            DAFGDA(HANDLE, ADDR, ADDR, std::slice::from_mut(&mut START), ctx)?;
        } else if (I == N) {
            if (GROUP == (NIDIR + 1)) {
                //
                // This is the last interval in the segment.
                //
                START = BUFFER[N];

                NSTART = DPMAX();
            } else {
                //
                // The last START time in this group is equal to the
                // request time.
                //
                START = BUFFER[N];

                ADDR = (GRPADD + N);

                DAFGDA(HANDLE, ADDR, ADDR, std::slice::from_mut(&mut NSTART), ctx)?;
            }
        } else {
            //
            // The bracketing START times are contained in this group.
            //
            START = BUFFER[I];
            NSTART = BUFFER[(I + 1)];
        }

        //
        // Save the information about the interval and segment.
        //
        save.LHAND = HANDLE;
        save.LBEG = BEG;
        save.LEND = END;
        save.PREVS = START;
        save.PREVN = NSTART;
    }

    //
    // Check and see if the bracketing pointing instances belong
    // to the same interval.  If they do then we can interpolate
    // between them, if not then check to see if the closer of
    // the two to the request time lies within the tolerance.
    //
    // The left bracketing time will always belong to the same
    // interval as the request time, therefore we need to check
    // only that the right bracketing time is less than the start
    // time of the next interval.
    //

    if (RSCLK < NSTART) {
        RECORD[1] = LSCLK;

        DAFGDA(
            HANDLE,
            LADDR,
            ((LADDR + PSIZ) - 1),
            RECORD.subarray_mut(2),
            ctx,
        )?;

        RECORD[9] = RSCLK;

        DAFGDA(
            HANDLE,
            RADDR,
            ((RADDR + PSIZ) - 1),
            RECORD.subarray_mut(10),
            ctx,
        )?;

        RECORD[17] = SCLKDP;

        *FOUND = true;
    } else {
        LDIFF = (SCLKDP - LSCLK);
        RDIFF = (RSCLK - SCLKDP);

        if ((LDIFF <= TOL) || (RDIFF <= TOL)) {
            //
            // Return the pointing instance closest to the request time.
            //
            // If the request time is midway between LSCLK and RSCLK then
            // grab the pointing instance associated with the greater time.
            //
            if (LDIFF < RDIFF) {
                RECORD[1] = LSCLK;
                RECORD[9] = LSCLK;

                DAFGDA(
                    HANDLE,
                    LADDR,
                    ((LADDR + PSIZ) - 1),
                    BUFFER.as_slice_mut(),
                    ctx,
                )?;

                MOVED(BUFFER.as_slice(), PSIZ, RECORD.subarray_mut(2));
                MOVED(BUFFER.as_slice(), PSIZ, RECORD.subarray_mut(10));
            } else {
                RECORD[1] = RSCLK;
                RECORD[9] = RSCLK;

                DAFGDA(
                    HANDLE,
                    RADDR,
                    ((RADDR + PSIZ) - 1),
                    BUFFER.as_slice_mut(),
                    ctx,
                )?;

                MOVED(BUFFER.as_slice(), PSIZ, RECORD.subarray_mut(2));
                MOVED(BUFFER.as_slice(), PSIZ, RECORD.subarray_mut(10));
            }

            RECORD[17] = SCLKDP;

            *FOUND = true;
        }
    }

    CHKOUT(b"CKR03", ctx)?;

    Ok(())
}
