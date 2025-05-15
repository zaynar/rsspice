//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const DTYPE: i32 = 2;
const NS: i32 = 5;
const ND: i32 = 2;
const NI: i32 = 5;
const SIDLEN: i32 = 40;

/// PCK, write type 2 segment
///
/// Write a type 2 segment to a PCK binary file given the file
/// handle, body-fixed frame class ID, frame, time range covered by
/// the segment, and the Chebyshev polynomial coefficients.
///
/// # Required Reading
///
/// * [NAIF_IDS](crate::required_reading::naif_ids)
/// * [SPC](crate::required_reading::spc)
/// * [PCK](crate::required_reading::pck)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of binary PCK file open for writing.
///  CLSSID     I   Frame class ID of body-fixed frame.
///  FRAME      I   Reference frame name.
///  FIRST      I   Start time of interval covered by segment.
///  LAST       I   End time of interval covered by segment.
///  SEGID      I   Segment identifier.
///  INTLEN     I   Length of time covered by logical record.
///  N          I   Number of logical records in segment.
///  POLYDG     I   Chebyshev polynomial degree.
///  CDATA      I   Array of Chebyshev coefficients.
///  BTIME      I   Begin time of first logical record.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the DAF handle of an PCK file to which a type 2
///           segment is to be added. The PCK file must be open
///           for writing.
///
///  CLSSID   is the frame class ID of a body-fixed reference
///           frame whose orientation is described by the
///           segment to be created.
///
///  FRAME    is the NAIF name for a reference frame relative to
///           which the orientation information for the frame
///           designated by CLSSID is specified. The frame
///           designated by FRAME is called the "base frame."
///
///  FIRST,
///  LAST     are, respectively, the start and stop times of
///           the time interval over which the segment defines
///           the orientation of body.
///
///  SEGID    is the segment identifier. A PCK segment
///           identifier may contain up to 40 characters.
///
///  INTLEN   is the length of time, in seconds, covered by each set of
///           Chebyshev polynomial coefficients (each logical record).
///           Each set of Chebyshev coefficients must cover this fixed
///           time interval, INTLEN.
///
///  N        is the number of sets of Chebyshev polynomial
///           coefficients (number of logical records)
///           to be stored in the segment. There is one set
///           of Chebyshev coefficients for each time period.
///
///  POLYDG   is the degree of each set of Chebyshev
///           polynomials.
///
///  CDATA    is an array containing all the sets of Chebyshev
///           polynomial coefficients to be contained in the
///           segment of the PCK file. The coefficients are
///           stored in CDATA in order as follows:
///
///              the (degree + 1) coefficients for the first
///              Euler angle of the first logical record,
///
///              the coefficients for the second Euler angle,
///
///              the coefficients for the third Euler angle,
///
///              the coefficients for the first Euler angle for
///              the second logical record, ...
///
///              and so on.
///
///  BTIME    is the begin time (seconds past J2000 TDB) of
///           first set of Chebyshev polynomial coefficients
///           (first logical record).
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the number of sets of coefficients is not positive,
///      the error SPICE(NUMCOEFFSNOTPOS) is signaled.
///
///  2)  If the interval length is not positive, the error
///      SPICE(INTLENNOTPOS) is signaled.
///
///  3)  If the integer code for the reference frame is not recognized,
///      the error SPICE(INVALIDREFFRAME) is signaled.
///
///  4)  If segment stop time is not greater then the begin time,
///      the error SPICE(BADDESCRTIMES) is signaled.
///
///  5)  If the time of the first record is not greater than or equal
///      to the descriptor begin time, the error SPICE(BADDESCRTIMES)
///      is signaled.
///
///  6)  If the end time of the last record is not greater than or
///      equal to the descriptor end time, the error
///      SPICE(BADDESCRTIMES) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  A new type 2 PCK segment is written to the PCK file attached
///  to HANDLE.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine writes an PCK type 2 data segment to the designated
///  PCK file, according to the format described in the PCK Required
///  Reading.
///
///  Each segment can contain data for only one body-fixed frame and
///  reference frame. The Chebyshev polynomial degree and length of
///  time covered by each logical record are also fixed. However, an
///  arbitrary number of logical records of Chebyshev polynomial
///  coefficients can be written in each segment. Minimizing the
///  number of segments in a PCK file will help optimize how the SPICE
///  system accesses the file.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Suppose that you have sets of Chebyshev polynomial
///     coefficients in an array pertaining to the orientation of
///     the Moon body-fixed frame with the frame class ID 301
///     relative to the J2000 reference frame, and want
///     to put these into a type 2 segment PCK file. The following
///     example could be used to add one new type 2 segment. To add
///     multiple segments, put the call to PCKW02 in a loop.
///
///
///     Example code begins here.
///
///
///           PROGRAM PCKW02_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         FNAME
///           PARAMETER           ( FNAME = 'pckw02_ex1.bpc' )
///
///           INTEGER               BODY
///           PARAMETER           ( BODY   = 301  )
///
///           INTEGER               IIDLEN
///           PARAMETER           ( IIDLEN = 40   )
///
///           INTEGER               POLYDG
///           PARAMETER           ( POLYDG = 9    )
///
///           INTEGER               SZCDAT
///           PARAMETER           ( SZCDAT = 60   )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(IIDLEN)    IFNAME
///           CHARACTER*(IIDLEN)    SEGID
///
///           DOUBLE PRECISION      BTIME
///           DOUBLE PRECISION      CDATA  ( SZCDAT )
///           DOUBLE PRECISION      FIRST
///           DOUBLE PRECISION      INTLEN
///           DOUBLE PRECISION      LAST
///
///           INTEGER               HANDLE
///           INTEGER               N
///           INTEGER               NRESVC
///
///     C
///     C     Set the input data: RA/DEC/W coefficients,
///     C     begin time for the first record, start/end times
///     C     for the segment, length of the time covered by
///     C     each record, and number of logical records.
///     C
///     C     CDATA contains the RA/DEC/W coefficients: first the
///     C     the POLYDEG + 1 for the RA first record, then the
///     C     POLYDEG + 1 for the DEC first record, then the
///     C     POLYDEG +1 for W first record, then the POLYDEG + 1
///     C     for the RA second record, and so on.
///     C
///           DATA                  CDATA /
///          .   -5.4242086033301107D-002, -5.2241405162792561D-005,
///          .    8.9751456289930307D-005, -1.5288696963234620D-005,
///          .    1.3218870864581395D-006,  5.9822156790328180D-007,
///          .   -6.5967702052551211D-008, -9.9084309118396298D-009,
///          .    4.9276055963541578D-010,  1.1612267413829385D-010,
///          .    0.42498898565916610D0,    1.3999219324235620D-004,
///          .   -1.8855140511098865D-005, -2.1964684808526649D-006,
///          .    1.4229817868138752D-006, -1.6991716166847001D-007,
///          .   -3.4824688140649506D-008,  2.9208428745895990D-009,
///          .    4.4217757657060300D-010, -3.9211207055305402D-012,
///          .    2565.0633504619473D0,     0.92003769451305328D0,
///          .   -8.0503797901914501D-005,  1.1960860244433900D-005,
///          .   -1.2237900518372542D-006, -5.3651349407824562D-007,
///          .    6.0843372260403005D-008,  9.0211287487688797D-009,
///          .   -4.6460429330339309D-010, -1.0446918704281774D-010,
///          .   -5.3839796353225056D-002,  4.3378021974424991D-004,
///          .    4.8130091384819459D-005, -1.2283066272873327D-005,
///          .   -5.4099296265403208D-006, -4.4237368347319652D-007,
///          .    1.3004982445546169D-007,  1.9017128275284284D-008,
///          .   -7.0368223839477803D-011, -1.7119414526133175D-010,
///          .    0.42507987850614548D0,   -7.1844899448557937D-005,
///          .   -5.1052122872412865D-005, -8.9810401387721321D-006,
///          .   -1.4611718567948972D-007,  4.0883847771062547D-007,
///          .    4.6812854485029333D-008, -4.5698075960784951D-009,
///          .   -9.8679875320349531D-010, -7.9392503778178240D-011,
///          .    2566.9029069934054D0,     0.91952244801740568D0,
///          .   -6.0426151041179828D-005,  1.0850559330577959D-005,
///          .    5.1756033678137497D-006,  4.2127585555214782D-007,
///          .   -1.1774737441872970D-007, -1.7397191490163833D-008,
///          .    5.8908810244396165D-012,  1.4594279337955166D-010 /
///
///           FIRST  =   -43200.D0
///           LAST   =  1339200.D0
///           BTIME  =  FIRST
///           INTLEN =   691200.D0
///           N      =  2
///
///     C
///     C     Open a new PCK file.  For simplicity, we will not
///     C     reserve any space for the comment area, so the
///     C     number of reserved comment characters is zero.
///     C     The variable IFNAME is the internal file name.
///     C
///           NRESVC  =  0
///           IFNAME  =  'Test PCK/Created 04-SEP-2019'
///
///           CALL PCKOPN ( FNAME, IFNAME, NRESVC, HANDLE    )
///
///     C
///     C     Create a segment identifier.
///     C
///           SEGID = 'MY_SAMPLE_PCK_TYPE_2_SEGMENT'
///
///     C
///     C     Write the segment.
///     C
///           CALL PCKW02 (  HANDLE, BODY,  'J2000',
///          .               FIRST,  LAST,   SEGID,   INTLEN,
///          .               N,      POLYDG, CDATA,   BTIME)
///
///     C
///     C     Close the file.
///     C
///           CALL PCKCLS ( HANDLE )
///
///           END
///
///
///     When this program is executed, no output is presented on
///     screen. After run completion, a new PCK type 2 exists in
///     the output directory.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  E.D. Wright        (JPL)
///  K.S. Zukor         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 17-AUG-2021 (JDR)
///
///         Changed the input argument name BODY to CLSSID for consistency
///         with other routines, and updated its description.
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example based on existing fragment.
///
/// -    SPICELIB Version 2.0.1, 03-JAN-2014 (EDW)
///
///         Minor edits to $Procedure; clean trailing whitespace.
///         Removed unneeded $Revisions section.
///
/// -    SPICELIB Version 2.0.0, 01-AUG-1995 (KSZ)
///
///         The calling sequence was corrected so that REF is
///         a character string and BTIME contains only the start
///         time of the first record. Comments updated, and new
///         routine CHCKID is called to check segment identifier.
///
/// -    SPICELIB Version 1.0.0, 11-MAR-1994 (KSZ)
/// ```
pub fn pckw02(
    ctx: &mut SpiceContext,
    handle: i32,
    clssid: i32,
    frame: &str,
    first: f64,
    last: f64,
    segid: &str,
    intlen: f64,
    n: i32,
    polydg: i32,
    cdata: &[f64],
    btime: f64,
) -> crate::Result<()> {
    PCKW02(
        handle,
        clssid,
        frame.as_bytes(),
        first,
        last,
        segid.as_bytes(),
        intlen,
        n,
        polydg,
        cdata,
        btime,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure PCKW02 ( PCK, write type 2 segment )
pub fn PCKW02(
    HANDLE: i32,
    CLSSID: i32,
    FRAME: &[u8],
    FIRST: f64,
    LAST: f64,
    SEGID: &[u8],
    INTLEN: f64,
    N: i32,
    POLYDG: i32,
    CDATA: &[f64],
    BTIME: f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let CDATA = DummyArray::new(CDATA, 1..);
    let mut ETSTR = [b' '; SIDLEN as usize];
    let mut NETSTR = [b' '; SIDLEN as usize];
    let mut DCD = StackArray::<f64, 2>::new(1..=ND);
    let mut DESCR = StackArray::<f64, 5>::new(1..=NS);
    let mut LTIME: f64 = 0.0;
    let mut MID: f64 = 0.0;
    let mut NUMREC: f64 = 0.0;
    let mut RADIUS: f64 = 0.0;
    let mut RSIZE: f64 = 0.0;
    let mut ICD = StackArray::<i32, 5>::new(1..=NI);
    let mut K: i32 = 0;
    let mut NINREC: i32 = 0;
    let mut REFCOD: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //
    // DTYPE is the PCK data type.
    //
    //
    // NS is the size of a packed PCK segment descriptor.
    //
    //
    // ND is the number of double precision components in an PCK
    // segment descriptor. PCK uses ND = 2.
    //
    //
    // NI is the number of integer components in an PCK segment
    // descriptor. PCK uses NI = 5.
    //
    //
    // SIDLEN is the maximum number of characters allowed in an
    // PCK segment identifier.
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"PCKW02", ctx)?;
    }

    //
    // The number of sets of coefficients must be positive.
    //
    if (N <= 0) {
        SETMSG(
            b"The number of sets of Euler anglecoefficients is not positive. N = #",
            ctx,
        );
        ERRINT(b"#", N, ctx);
        SIGERR(b"SPICE(NUMCOEFFSNOTPOS)", ctx)?;
        CHKOUT(b"PCKW02", ctx)?;
        return Ok(());
    }
    //
    // The interval length must be positive.
    //
    if (INTLEN <= 0 as f64) {
        SETMSG(b"The interval length is not positive.N = #", ctx);
        ERRDP(b"#", INTLEN, ctx);
        SIGERR(b"SPICE(INTLENNOTPOS)", ctx)?;
        CHKOUT(b"PCKW02", ctx)?;
        return Ok(());
    }
    //
    // Get the NAIF integer code for the reference frame.
    //
    IRFNUM(FRAME, &mut REFCOD, ctx);

    if (REFCOD == 0) {
        SETMSG(b"The reference frame # is not supported.", ctx);
        ERRCH(b"#", FRAME, ctx);
        SIGERR(b"SPICE(INVALIDREFFRAME)", ctx)?;
        CHKOUT(b"PCKW02", ctx)?;
        return Ok(());
    }

    //
    // The segment stop time must be greater than the begin time.
    //
    if (FIRST > LAST) {
        SETMSG(
            b"The segment start time: # is greater than the segment end time: #",
            ctx,
        );
        ETCAL(FIRST, &mut ETSTR, ctx);
        ERRCH(b"#", &ETSTR, ctx);
        ETCAL(LAST, &mut NETSTR, ctx);
        ERRCH(b"#", &NETSTR, ctx);
        SIGERR(b"SPICE(BADDESCRTIMES)", ctx)?;
        CHKOUT(b"PCKW02", ctx)?;
        return Ok(());
    }

    //
    // The begin time of the first record must be less than or equal
    // to the begin time of the segment.
    //
    if (FIRST < BTIME) {
        SETMSG(b"The segment descriptor start time: # is less than the beginning time of the segment data: #", ctx);
        ETCAL(FIRST, &mut ETSTR, ctx);
        ERRCH(b"#", &ETSTR, ctx);
        ETCAL(BTIME, &mut ETSTR, ctx);
        ERRCH(b"#", &ETSTR, ctx);
        SIGERR(b"SPICE(BADDESCRTIMES)", ctx)?;
        CHKOUT(b"PCKW02", ctx)?;
        return Ok(());
    }
    //
    // The end time of the final record must be greater than or
    // equal to the end time of the segment.
    //
    LTIME = (BTIME + ((N as f64) * INTLEN));
    if (LAST > LTIME) {
        SETMSG(b"The segment descriptor end time: # is greater than the end time of the segment data: #", ctx);
        ETCAL(LAST, &mut ETSTR, ctx);
        ERRCH(b"#", &ETSTR, ctx);
        ETCAL(LTIME, &mut ETSTR, ctx);
        ERRCH(b"#", &ETSTR, ctx);
        SIGERR(b"SPICE(BADDESCRTIMES)", ctx)?;
        CHKOUT(b"PCKW02", ctx)?;
        return Ok(());
    }

    //
    // Now check the validity of the segment identifier.
    //
    CHCKID(b"PCK segment identifier", SIDLEN, SEGID, ctx)?;
    if FAILED(ctx) {
        CHKOUT(b"PCKW02", ctx)?;
        return Ok(());
    }

    //
    // Store the start and end times to be associated
    // with this segment.
    //
    DCD[1] = FIRST;
    DCD[2] = LAST;

    //
    // Create the integer portion of the descriptor.
    //
    ICD[1] = CLSSID;
    ICD[2] = REFCOD;
    ICD[3] = DTYPE;

    //
    // Pack the segment descriptor.
    //
    DAFPS(ND, NI, DCD.as_slice(), ICD.as_slice(), DESCR.as_slice_mut());

    //
    // Begin a new segment of PCK type 2 form:
    //
    //    Record 1
    //    Record 2
    //    ...
    //    Record N
    //    INIT       ( initial epoch of first record )
    //    INTLEN     ( length of interval covered by each record )
    //    RSIZE      ( number of data elements in each record )
    //    N          ( number of records in segment )
    //
    // Each record will have the form:
    //
    //    MID        ( midpoint of time interval )
    //    RADIUS     ( radius of time interval )
    //    X coefficients, Y coefficients, Z coefficients
    //
    DAFBNA(HANDLE, DESCR.as_slice(), SEGID, ctx)?;

    //
    // Calculate the number of entries in a record.
    //
    NINREC = ((POLYDG + 1) * 3);

    //
    // Fill segment with N records of data.
    //
    for I in 1..=N {
        //
        // Calculate the midpoint and radius of the time of each
        // record, and put that at the beginning of each record.
        //
        RADIUS = (INTLEN / 2 as f64);
        MID = ((BTIME + RADIUS) + (((I - 1) as f64) * INTLEN));

        DAFADA(&[MID], 1, ctx)?;
        DAFADA(&[RADIUS], 1, ctx)?;

        //
        // Put one set of coefficients into the segment.
        //
        K = (1 + ((I - 1) * NINREC));

        DAFADA(CDATA.subarray(K), NINREC, ctx)?;
    }
    //
    // Store the initial epoch of the first record.
    //
    DAFADA(&[BTIME], 1, ctx)?;

    //
    // Store the length of interval covered by each record.
    //
    DAFADA(&[INTLEN], 1, ctx)?;

    //
    // Store the size of each record (total number of array elements).
    //
    RSIZE = (2 + NINREC) as f64;
    DAFADA(&[RSIZE], 1, ctx)?;

    //
    // Store the number of records contained in the segment.
    //
    NUMREC = N as f64;
    DAFADA(&[NUMREC], 1, ctx)?;

    //
    // End this segment.
    //
    DAFENA(ctx)?;

    CHKOUT(b"PCKW02", ctx)?;
    Ok(())
}
