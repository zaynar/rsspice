//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const MAXDEG: i32 = 27;
const TOLSCL: f64 = 0.0000000000001;
const DTYPE: i32 = 3;
const ND: i32 = 2;
const NI: i32 = 6;
const NS: i32 = 5;
const SIDLEN: i32 = 40;

/// SPK, write segment, type 3
///
/// Write a type 3 segment to an SPK file.
///
/// # Required Reading
///
/// * [NAIF_IDS](crate::required_reading::naif_ids)
/// * [SPC](crate::required_reading::spc)
/// * [SPK](crate::required_reading::spk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  MAXDEG     P   Maximum degree of Chebyshev expansions.
///  TOLSCL     P   Scale factor used to compute time bound tolerance.
///  HANDLE     I   Handle of SPK file open for writing.
///  BODY       I   NAIF code for ephemeris object.
///  CENTER     I   NAIF code for the center of motion of the body.
///  FRAME      I   Reference frame name.
///  FIRST      I   Start time of interval covered by segment.
///  LAST       I   End time of interval covered by segment.
///  SEGID      I   Segment identifier.
///  INTLEN     I   Length of time covered by record.
///  N          I   Number of records in segment.
///  POLYDG     I   Chebyshev polynomial degree.
///  CDATA      I   Array of Chebyshev coefficients.
///  BTIME      I   Begin time of first record.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the DAF handle of an SPK file to which a type 3
///           segment is to be added. The SPK file must be open for
///           writing.
///
///  BODY     is the NAIF integer code for an ephemeris object whose
///           state relative to another body is described by the
///           segment to be created.
///
///  CENTER   is the NAIF integer code for the center of motion of the
///           object identified by BODY.
///
///  FRAME    is the NAIF name for a reference frame relative to which
///           the state information for BODY is specified.
///
///  FIRST,
///  LAST     are the start and stop times of the time interval over
///           which the segment defines the state of body.
///
///  SEGID    is the segment identifier. An SPK segment identifier may
///           contain up to 40 characters.
///
///  INTLEN   is the length of time, in seconds, covered by each set of
///           Chebyshev polynomial coefficients (each logical record).
///           Each set of Chebyshev coefficients must cover this fixed
///           time interval, INTLEN.
///
///  N        is the number of sets of Chebyshev polynomial
///           coefficients for coordinates and their derivatives
///           (number of logical records) to be stored in the segment.
///           There is one set of Chebyshev coefficients for each time
///           period.
///
///  POLYDG   is the degree of each set of Chebyshev polynomials used
///           to represent the ephemeris information. POLYDG must not
///           exceed MAXDEG (see $Parameters below).
///
///  CDATA    is a time-ordered array of N sets of Chebyshev polynomial
///           coefficients to be placed in the segment of the SPK file.
///           Each set has size SETSZ = 6*(POLYDG+1). The coefficients
///           are stored in CDATA in order as follows:
///
///              the (degree + 1) coefficients for the first
///              coordinate of the first logical record,
///
///              the coefficients for the second coordinate,
///
///              the coefficients for the third coordinate,
///
///              the coefficients for the derivative with respect
///              to time of the first coordinate,
///
///              the coefficients for the derivative with respect
///              to time of the second coordinate,
///
///              the coefficients for the derivative with respect
///              to time of the third coordinate,
///
///              the coefficients for the first coordinate for
///              the second logical record, ...
///
///              and so on.
///
///  BTIME    is the begin time (seconds past J2000 TDB) of first set
///           of Chebyshev polynomial coefficients (first logical
///           record).
/// ```
///
/// # Detailed Output
///
/// ```text
///  None.
///
///  The routine writes to the SPK file referred to by HANDLE a type 03
///  SPK segment containing the data in CDATA.
///
///  See the $Particulars section for details about the structure of a
///  type 03 SPK segment.
/// ```
///
/// # Parameters
///
/// ```text
///  See the include file spk03.inc for declarations of the
///  parameters described below.
///
///  TOLSCL   is a tolerance scale for coverage gap at endpoints
///           of the segment coverage interval.
///
///  MAXDEG   is the maximum allowed degree of the input
///           Chebyshev expansions.
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
///  3)  If the name of the reference frame is not recognized,
///      the error SPICE(INVALIDREFFRAME) is signaled.
///
///  4)  If segment stop time is not greater then the begin time,
///      the error SPICE(BADDESCRTIMES) is signaled.
///
///  5)  If the start time of the first record exceeds the descriptor
///      begin time by more than a computed tolerance, or if the end
///      time of the last record precedes the descriptor end time by
///      more than a computed tolerance, the error SPICE(COVERAGEGAP)
///      is signaled. See the $Parameters section above for a
///      description of the tolerance.
///
///  6)  If the input degree POLYDG is less than 0 or greater than
///      MAXDEG, the error SPICE(INVALIDDEGREE) is signaled.
///
///  7)  If the last non-blank character of SEGID occurs past index 40,
///      or if SEGID contains any nonprintable characters, an error is
///      signaled by a routine in the call tree of this routine.
/// ```
///
/// # Files
///
/// ```text
///  A new type 3 SPK segment is written to the SPK file attached
///  to HANDLE.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine writes an SPK type 3 data segment to the designated
///  SPK file, according to the format described in the SPK Required
///  Reading.
///
///  Each segment can contain data for only one target, central body,
///  and reference frame. The Chebyshev polynomial degree and length
///  of time covered by each logical record are also fixed. However,
///  an arbitrary number of logical records of Chebyshev polynomial
///  coefficients can be written in each segment. Minimizing the
///  number of segments in an SPK file will help optimize how the SPICE
///  system accesses the file.
///
///
///  The ephemeris data supplied to the type 3 SPK writer is packed
///  into an array as a sequence of records. The logical data records
///  are stored contiguously:
///
///     +----------+
///     | Record 1 |
///     +----------+
///     | Record 2 |
///     +----------+
///         ...
///     +----------+
///     | Record N |
///     +----------+
///
///  The contents of an individual record are:
///
///     +--------------------------------------+
///     | Coeff set for X position component   |
///     +--------------------------------------+
///     | Coeff set for Y position component   |
///     +--------------------------------------+
///     | Coeff set for Z position component   |
///     +--------------------------------------+
///     | Coeff set for X velocity component   |
///     +--------------------------------------+
///     | Coeff set for Y velocity component   |
///     +--------------------------------------+
///     | Coeff set for Z velocity component   |
///     +--------------------------------------+
///
///  Each coefficient set has the structure:
///
///     +--------------------------------------+
///     | Coefficient of T_0                   |
///     +--------------------------------------+
///     | Coefficient of T_1                   |
///     +--------------------------------------+
///                       ...
///     +--------------------------------------+
///     | Coefficient of T_POLYDG              |
///     +--------------------------------------+
///
///  Where T_n represents the Chebyshev polynomial of the first kind of
///  degree n.
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
///  1) This example demonstrates how to create an SPK type 3 kernel
///     containing only one segment, given a set of Chebychev
///     coefficients and their associated epochs.
///
///
///     Example code begins here.
///
///
///           PROGRAM SPKW03_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               NAMLEN
///           PARAMETER           ( NAMLEN = 40 )
///
///     C
///     C     Define the segment identifier parameters.
///     C
///           CHARACTER*(*)         SPK3
///           PARAMETER           ( SPK3  = 'spkw03_ex1.bsp' )
///
///           CHARACTER*(*)         REF
///           PARAMETER           ( REF    = 'J2000'          )
///
///           INTEGER               BODY
///           PARAMETER           ( BODY   = 3  )
///
///           INTEGER               CENTER
///           PARAMETER           ( CENTER = 10 )
///
///           INTEGER               CHBDEG
///           PARAMETER           ( CHBDEG = 2  )
///
///           INTEGER               NRECS
///           PARAMETER           ( NRECS  = 4  )
///
///           INTEGER               RECSIZ
///           PARAMETER           ( RECSIZ = 6*(CHBDEG+1) )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(NAMLEN)    IFNAME
///           CHARACTER*(NAMLEN)    SEGID
///
///           DOUBLE PRECISION      FIRST
///           DOUBLE PRECISION      INTLEN
///           DOUBLE PRECISION      LAST
///           DOUBLE PRECISION      RECRDS ( RECSIZ, NRECS )
///
///           INTEGER               HANDLE
///           INTEGER               NCOMCH
///
///     C
///     C     Define the coefficients.
///     C
///           DATA                  RECRDS /
///          .                      1.0101D0, 1.0102D0, 1.0103D0,
///          .                      1.0201D0, 1.0202D0, 1.0203D0,
///          .                      1.0301D0, 1.0302D0, 1.0303D0,
///          .                      1.0401D0, 1.0402D0, 1.0403D0,
///          .                      1.0501D0, 1.0502D0, 1.0503D0,
///          .                      1.0601D0, 1.0602D0, 1.0603D0,
///          .                      2.0101D0, 2.0102D0, 2.0103D0,
///          .                      2.0201D0, 2.0202D0, 2.0203D0,
///          .                      2.0301D0, 2.0302D0, 2.0303D0,
///          .                      2.0401D0, 2.0402D0, 2.0403D0,
///          .                      2.0501D0, 2.0502D0, 2.0503D0,
///          .                      2.0601D0, 2.0602D0, 2.0603D0,
///          .                      3.0101D0, 3.0102D0, 3.0103D0,
///          .                      3.0201D0, 3.0202D0, 3.0203D0,
///          .                      3.0301D0, 3.0302D0, 3.0303D0,
///          .                      3.0401D0, 3.0402D0, 3.0403D0,
///          .                      3.0501D0, 3.0502D0, 3.0503D0,
///          .                      3.0601D0, 3.0602D0, 3.0603D0,
///          .                      4.0101D0, 4.0102D0, 4.0103D0,
///          .                      4.0201D0, 4.0202D0, 4.0203D0,
///          .                      4.0301D0, 4.0302D0, 4.0303D0,
///          .                      4.0401D0, 4.0402D0, 4.0403D0,
///          .                      4.0501D0, 4.0502D0, 4.0503D0,
///          .                      4.0601D0, 4.0602D0, 4.0603D0 /
///
///
///     C
///     C     Set the start and end times of interval covered by
///     C     segment, and the length of time covered by logical
///     C     record.
///     C
///           FIRST  = 100.D0
///           LAST   = 500.D0
///           INTLEN = 100.D0
///
///     C
///     C     NCOMCH is the number of characters to reserve for the
///     C     kernel's comment area. This example doesn't write
///     C     comments, so set to zero.
///     C
///           NCOMCH = 0
///
///     C
///     C     Internal file name and segment ID.
///     C
///           IFNAME = 'Type 3 SPK internal file name.'
///           SEGID  = 'SPK type 3 test segment'
///
///     C
///     C     Open a new SPK file.
///     C
///           CALL SPKOPN( SPK3, IFNAME, NCOMCH, HANDLE )
///
///     C
///     C     Write the segment.
///     C
///           CALL SPKW03 ( HANDLE, BODY,   CENTER, REF,
///          .              FIRST,  LAST,   SEGID,  INTLEN,
///          .              NRECS,  CHBDEG, RECRDS, FIRST  )
///
///     C
///     C     Close the SPK file.
///     C
///           CALL SPKCLS ( HANDLE )
///
///           END
///
///
///     When this program is executed, no output is presented on
///     screen. After run completion, a new SPK type 3 exists in
///     the output directory.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  E.D. Wright        (JPL)
///  K.S. Zukor         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.0.1, 17-JUN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Added complete
///         example code from existing fragment.
///
///         Extended POLYDG and CDATA arguments description to provide the
///         size of the Chebyshev polynomials sets. Moved the details of
///         the SPK structure from CDATA argument description to
///         $Particulars section.
///
/// -    SPICELIB Version 2.0.0, 18-JAN-2014 (NJB)
///
///         Relaxed test on relationship between the time bounds of the
///         input record set (determined by BTIME, INTLEN, and N) and the
///         descriptor bounds FIRST and LAST. Now the descriptor bounds
///         may extend beyond the time bounds of the record set by a ratio
///         computed using the parameter TOLSCL (see $Parameters above for
///         details). Added checks on input polynomial degree.
///
/// -    SPICELIB Version 1.1.0, 30-OCT-2006 (BVS)
///
///         Removed restriction that the input reference frame should be
///         inertial by changing the routine that determines the frame ID
///         from the name from IRFNUM to NAMFRM.
///
/// -    SPICELIB Version 1.0.1, 19-SEP-2006 (EDW)
///
///         Corrected typo in the section name ("Example" to "Examples").
///
/// -    SPICELIB Version 1.0.0, 01-AUG-1995 (KSZ)
/// ```
pub fn spkw03(
    ctx: &mut SpiceContext,
    handle: i32,
    body: i32,
    center: i32,
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
    SPKW03(
        handle,
        body,
        center,
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

//$Procedure SPKW03 ( SPK, write segment, type 3 )
pub fn SPKW03(
    HANDLE: i32,
    BODY: i32,
    CENTER: i32,
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
    let mut TOL: f64 = 0.0;
    let mut ICD = StackArray::<i32, 6>::new(1..=NI);
    let mut K: i32 = 0;
    let mut NINREC: i32 = 0;
    let mut REFCOD: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //
    // DTYPE is the SPK data type.
    //
    //
    // ND is the number of double precision components in an SPK
    // segment descriptor. SPK uses ND = 2.
    //
    //
    // NI is the number of integer components in an SPK segment
    // descriptor. SPK uses NI = 6.
    //
    //
    // NS is the size of a packed SPK segment descriptor.
    //
    //
    // SIDLEN is the maximum number of characters allowed in an
    // SPK segment identifier.
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"SPKW03", ctx)?;

    //
    // The number of sets of coefficients must be positive.
    //
    if (N <= 0) {
        SETMSG(
            b"The number of sets of coordinatecoefficients is not positive. N = #.",
            ctx,
        );
        ERRINT(b"#", N, ctx);
        SIGERR(b"SPICE(NUMCOEFFSNOTPOS)", ctx)?;
        CHKOUT(b"SPKW03", ctx)?;
        return Ok(());
    }

    //
    // Make sure that the degree of the interpolating polynomials is
    // in range.
    //
    if ((POLYDG < 0) || (POLYDG > MAXDEG)) {
        SETMSG(
            b"The interpolating polynomials have degree #; the valid degree range is [0, #].",
            ctx,
        );
        ERRINT(b"#", POLYDG, ctx);
        ERRINT(b"#", MAXDEG, ctx);
        SIGERR(b"SPICE(INVALIDDEGREE)", ctx)?;
        CHKOUT(b"SPKW03", ctx)?;
        return Ok(());
    }

    //
    // The interval length must be positive.
    //
    if (INTLEN <= 0 as f64) {
        SETMSG(b"The interval length is not positive.N = #", ctx);
        ERRDP(b"#", INTLEN, ctx);
        SIGERR(b"SPICE(INTLENNOTPOS)", ctx)?;
        CHKOUT(b"SPKW03", ctx)?;
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
        CHKOUT(b"SPKW03", ctx)?;
        return Ok(());
    }

    //
    // The segment stop time must be greater than the begin time.
    //
    if (FIRST > LAST) {
        SETMSG(
            b"The segment descriptor start time: # is greater than the segment end time: #",
            ctx,
        );
        ETCAL(FIRST, &mut ETSTR, ctx);
        ERRCH(b"#", &ETSTR, ctx);
        ETCAL(LAST, &mut NETSTR, ctx);
        ERRCH(b"#", &NETSTR, ctx);
        SIGERR(b"SPICE(BADDESCRTIMES)", ctx)?;
        CHKOUT(b"SPKW03", ctx)?;
        return Ok(());
    }

    //
    // Compute the tolerance to use for descriptor time bound checks.
    //
    TOL = (TOLSCL * intrinsics::DMAX1(&[f64::abs(FIRST), f64::abs(LAST)]));

    if (FIRST < (BTIME - TOL)) {
        SETMSG(b"The segment descriptor start time # is too much less than the beginning time of the  segment data # (in seconds past J2000: #). The difference is # seconds; the  tolerance is # seconds.", ctx);
        ETCAL(FIRST, &mut ETSTR, ctx);
        ERRCH(b"#", &ETSTR, ctx);
        ETCAL(BTIME, &mut ETSTR, ctx);
        ERRCH(b"#", &ETSTR, ctx);
        ERRDP(b"#", FIRST, ctx);
        ERRDP(b"#", (BTIME - FIRST), ctx);
        ERRDP(b"#", TOL, ctx);
        SIGERR(b"SPICE(COVERAGEGAP)", ctx)?;
        CHKOUT(b"SPKW03", ctx)?;
        return Ok(());
    }

    //
    // The end time of the final record must be greater than or
    // equal to the end time of the segment.
    //
    LTIME = (BTIME + ((N as f64) * INTLEN));

    if (LAST > (LTIME + TOL)) {
        SETMSG(b"The segment descriptor end time # is too much greater than the end time of the segment data # (in seconds past J2000: #). The difference is # seconds; the tolerance is # seconds.", ctx);
        ETCAL(LAST, &mut ETSTR, ctx);
        ERRCH(b"#", &ETSTR, ctx);
        ETCAL(LTIME, &mut ETSTR, ctx);
        ERRCH(b"#", &ETSTR, ctx);
        ERRDP(b"#", LAST, ctx);
        ERRDP(b"#", (LAST - LTIME), ctx);
        ERRDP(b"#", TOL, ctx);
        SIGERR(b"SPICE(COVERAGEGAP)", ctx)?;
        CHKOUT(b"SPKW03", ctx)?;
        return Ok(());
    }

    //
    // Now check the validity of the segment identifier.
    //
    CHCKID(b"SPK segment identifier", SIDLEN, SEGID, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SPKW03", ctx)?;
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
    ICD[1] = BODY;
    ICD[2] = CENTER;
    ICD[3] = REFCOD;
    ICD[4] = DTYPE;

    //
    // Pack the segment descriptor.
    //
    DAFPS(ND, NI, DCD.as_slice(), ICD.as_slice(), DESCR.as_slice_mut());

    //
    // Begin a new segment of SPK type 3 form:
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
    //    X' coefficients, Y' coefficients, Z' coefficients
    //
    DAFBNA(HANDLE, DESCR.as_slice(), SEGID, ctx)?;

    //
    // Calculate the number of Chebyshev coefficients in a record.
    //
    NINREC = ((POLYDG + 1) * 6);

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

    CHKOUT(b"SPKW03", ctx)?;
    Ok(())
}
