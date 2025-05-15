//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const MAXDEG: i32 = 50;
const TOLSCL: f64 = 0.0000000000001;
const DTYPE: i32 = 20;
const ND: i32 = 2;
const NI: i32 = 6;
const NS: i32 = 5;
const SIDLEN: i32 = 40;

/// SPK, write segment, type 20
///
/// Write a type 20 segment to an SPK file.
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
/// * [NAIF_IDS](crate::required_reading::naif_ids)
/// * [TIME](crate::required_reading::time)
/// * [SPK](crate::required_reading::spk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of SPK file open for writing.
///  BODY       I   NAIF code for ephemeris object.
///  CENTER     I   NAIF code for the center of motion of the body.
///  FRAME      I   Reference frame name.
///  FIRST      I   Start time of interval covered by segment.
///  LAST       I   End time of interval covered by segment.
///  SEGID      I   Segment identifier.
///  INTLEN     I   Length of time covered by logical record (days).
///  N          I   Number of logical records in segment.
///  POLYDG     I   Chebyshev polynomial degree.
///  CDATA      I   Array of Chebyshev coefficients and positions.
///  DSCALE     I   Distance scale of data.
///  TSCALE     I   Time scale of data.
///  INITJD     I   Integer part of begin time (TDB Julian date) of
///                 first record.
///  INITFR     I   Fractional part of begin time of first record.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the DAF handle of an SPK file to which a type 20
///           segment is to be added. The SPK file must be open
///           for writing.
///
///  BODY     is the NAIF integer code for an ephemeris object
///           whose state relative to another body is described
///           by the segment to be created.
///
///  CENTER   is the NAIF integer code for the center of motion
///           of the object identified by BODY.
///
///  FRAME    is the NAIF name for a reference frame relative to
///           which the state information for BODY is specified.
///
///  FIRST,
///  LAST     are the start and stop times of the time interval
///           over which the segment defines the state of the
///           object identified by BODY.
///
///  SEGID    is a segment identifier. An SPK segment identifier
///           may contain up to 40 characters.
///
///  INTLEN   is the length of time, in TDB Julian days, covered
///           by each set of Chebyshev polynomial coefficients
///           (each logical record).
///
///  N        is the number of logical records to be stored in
///           the segment. There is one logical record for each
///           time period. Each logical record contains three
///           sets of Chebyshev coefficients---one for each
///           coordinate---and three position vector components.
///
///  POLYDG   is the degree of each set of Chebyshev
///           polynomials, i.e. the number of Chebyshev
///           coefficients per coordinate minus one. POLYDG must
///           be less than or equal to the parameter MAXDEG.
///
///  CDATA    is an array containing all the sets of Chebyshev
///           polynomial coefficients and position components to
///           be placed in the new segment of the SPK file.
///           There are three sets of coefficients and position
///           components for each time interval covered by the
///           segment.
///
///           The coefficients and position components are
///           stored in CDATA in order as follows:
///
///              the (POLYDG + 1) coefficients for the first
///              coordinate of the first logical record,
///              followed by the X component of position at the
///              first interval midpoint. The first coefficient
///              is that of the constant term of the expansion.
///
///              the coefficients for the second coordinate,
///              followed by the Y component of position at the
///              first interval midpoint.
///
///              the coefficients for the third coordinate,
///              followed by the Z component of position at the
///              first interval midpoint.
///
///              the coefficients for the first coordinate for
///              the second logical record, followed by the X
///              component of position at the second interval
///              midpoint.
///
///              and so on.
///
///           The logical data records are stored contiguously:
///
///              +----------+
///              | Record 1 |
///              +----------+
///              | Record 2 |
///              +----------+
///                  ...
///              +----------+
///              | Record N |
///              +----------+
///
///           The contents of an individual record are:
///
///              +--------------------------------------+
///              | Coeff set for X velocity component   |
///              +--------------------------------------+
///              | X position component                 |
///              +--------------------------------------+
///              | Coeff set for Y velocity component   |
///              +--------------------------------------+
///              | Y position component                 |
///              +--------------------------------------+
///              | Coeff set for Z velocity component   |
///              +--------------------------------------+
///              | Z position component                 |
///              +--------------------------------------+
///
///                Each coefficient set has the structure:
///
///              +--------------------------------------+
///              | Coefficient of T_0                   |
///              +--------------------------------------+
///              | Coefficient of T_1                   |
///              +--------------------------------------+
///                                ...
///              +--------------------------------------+
///              | Coefficient of T_POLYDG              |
///              +--------------------------------------+
///
///           Where T_n represents the Chebyshev polynomial
///           of the first kind of degree n.
///
///  DSCALE,
///  TSCALE   are, respectively, the distance scale of the input
///           position and velocity data in km, and the time
///           scale of the input velocity data in TDB seconds.
///
///           For example, if the input distance data have units
///           of astronomical units (AU), DSCALE should be set
///           to the number of km in one AU. If the input
///           velocity data have time units of Julian days, then
///           TSCALE should be set to the number of seconds per
///           Julian day (86400).
///
///  INITJD   is the integer part of the Julian ephemeris date
///           of initial epoch of the first record. INITJD may
///           be less than, equal to, or greater than the
///           initial epoch.
///
///  INITFR   is the fractional part of the Julian ephemeris date
///           of initial epoch of the first record. INITFR has
///           units of Julian days. INITFR has magnitude
///           strictly less than 1 day. The sum
///
///              INITJD + INITFR
///
///           equals the Julian ephemeris date of the initial
///           epoch of the first record.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None. This routine writes data to an SPK file.
/// ```
///
/// # Parameters
///
/// ```text
///  The parameters described in this section are declared in the
///  Fortran INCLUDE file spk20.inc
///
///  MAXDEG   is the maximum allowed degree of the input
///           Chebyshev expansions.
///
///  TOLSCL   is a tolerance scale factor (also called a
///           "relative tolerance") used for time coverage
///           bound checking. TOLSCL is unitless. TOLSCL
///           produces a tolerance value via the formula
///
///              TOL = TOLSCL * MAX( ABS(FIRST), ABS(LAST) )
///
///           where FIRST and LAST are the coverage time bounds
///           of a type 20 segment, expressed as seconds past
///           J2000 TDB.
///
///           The resulting parameter TOL is used as a tolerance
///           for comparing the input segment descriptor time
///           bounds to the first and last epoch covered by the
///           sequence of time intervals defined by the inputs
///
///              INITJD
///              INITFR
///              INTLEN
///              N
///
///           See the $Exceptions section below for a description
///           of the error check using this tolerance.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the number of sets of coefficients is not positive,
///      the error SPICE(INVALIDCOUNT) is signaled.
///
///  2)  If the interval length is not positive, the error
///      SPICE(INTLENNOTPOS) is signaled.
///
///  3)  If the name of the reference frame is not recognized,
///      the error SPICE(INVALIDREFFRAME) is signaled.
///
///  4)  If segment stop time is not greater than or equal to
///      the begin time, the error SPICE(BADDESCRTIMES) is signaled.
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
///
///  8)  If either the distance or time scale is non-positive, the
///      error SPICE(NONPOSITIVESCALE) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  A new type 20 SPK segment is written to the SPK file attached
///  to HANDLE.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine writes an SPK type 20 data segment to the designated
///  SPK file, according to the format described in the SPK Required
///  Reading.
///
///  Each segment can contain data for only one target, central body,
///  and reference frame. The Chebyshev polynomial degree and length
///  of time covered by each logical record are also fixed. However,
///  an arbitrary number of logical records of Chebyshev polynomial
///  coefficients can be written in each segment. Minimizing the
///  number of segments in an SPK file will help optimize how the
///  SPICE system accesses the file.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that you have in an array CDATA sets of Chebyshev
///  polynomial coefficients and position vectors representing the
///  state of the moon (NAIF ID = 301), relative to the Earth-moon
///  barycenter (NAIF ID = 3), in the J2000 reference frame, and you
///  want to put these into a type 20 segment in an existing SPK file.
///  The following code could be used to add one new type 20 segment.
///  To add multiple segments, put the call to SPKW20 in a loop.
///
///  C
///  C      First open the SPK file and get a handle for it.
///  C
///         CALL DAFOPW ( SPKNAM, HANDLE )
///
///  C
///  C      Create a segment identifier.
///  C
///         SEGID = 'MY_SAMPLE_SPK_TYPE_20_SEGMENT'
///
///  C
///  C      Note that the interval length INTLEN has units
///  C      of Julian days. The start time of the first record
///  C      is expressed using two inputs: integer and fractional
///  C      portions of the Julian ephemeris date of the start
///  C      time.
///  C
///  C      Write the segment.
///  C
///         CALL SPKW20 ( HANDLE, 301,    3,      'J2000',
///       .               FIRST,  LAST,   SEGID,  INTLEN,
///       .               N,      POLYDG, CDATA,  DSCALE,
///       .               TSCALE, INITJD, INITFR           )
///
///  C
///  C      Close the file.
///  C
///         CALL DAFCLS ( HANDLE )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.S. Zukor         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 05-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
///         Added missing description of INITFR to $Brief_I/O section.
///
/// -    SPICELIB Version 1.0.0, 17-JAN-2017 (NJB) (KSZ)
/// ```
pub fn spkw20(
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
    dscale: f64,
    tscale: f64,
    initjd: f64,
    initfr: f64,
) -> crate::Result<()> {
    SPKW20(
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
        dscale,
        tscale,
        initjd,
        initfr,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKW20 ( SPK, write segment, type 20 )
pub fn SPKW20(
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
    DSCALE: f64,
    TSCALE: f64,
    INITJD: f64,
    INITFR: f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let CDATA = DummyArray::new(CDATA, 1..);
    let mut ETSTR = [b' '; SIDLEN as usize];
    let mut NETSTR = [b' '; SIDLEN as usize];
    let mut BTIME: f64 = 0.0;
    let mut DCD = StackArray::<f64, 2>::new(1..=ND);
    let mut DESCR = StackArray::<f64, 5>::new(1..=NS);
    let mut LTIME: f64 = 0.0;
    let mut NUMREC: f64 = 0.0;
    let mut TOL: f64 = 0.0;
    let mut ICD = StackArray::<i32, 6>::new(1..=NI);
    let mut NINREC: i32 = 0;
    let mut REFCOD: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //

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
    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"SPKW20", ctx)?;

    //
    // The number of sets of coefficients must be positive.
    //
    if (N <= 0) {
        SETMSG(
            b"The number of sets of coordinate coefficients is not positive. N = # ",
            ctx,
        );
        ERRINT(b"#", N, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"SPKW20", ctx)?;
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
        CHKOUT(b"SPKW20", ctx)?;
        return Ok(());
    }

    //
    // The interval length must be positive.
    //
    if (INTLEN <= 0 as f64) {
        SETMSG(b"The interval length is not positive.N = #", ctx);
        ERRDP(b"#", INTLEN, ctx);
        SIGERR(b"SPICE(INTLENNOTPOS)", ctx)?;
        CHKOUT(b"SPKW20", ctx)?;
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
        CHKOUT(b"SPKW20", ctx)?;
        return Ok(());
    }

    //
    // The segment stop time must be greater than or equal to the begin
    // time.
    //
    if (FIRST > LAST) {
        SETMSG(
            b"The segment start time: # (# TDB) is greater than the segment end time: (# TDB).",
            ctx,
        );
        ETCAL(FIRST, &mut ETSTR, ctx);
        ERRCH(b"#", &ETSTR, ctx);
        ERRDP(b"#", FIRST, ctx);
        ETCAL(LAST, &mut NETSTR, ctx);
        ERRCH(b"#", &NETSTR, ctx);
        ERRDP(b"#", LAST, ctx);
        SIGERR(b"SPICE(BADDESCRTIMES)", ctx)?;
        CHKOUT(b"SPKW20", ctx)?;
        return Ok(());
    }

    //
    // The distance and time scales must be positive.
    //
    if (DSCALE <= 0.0) {
        SETMSG(b"The distance scale is not positive.DSCALE = #", ctx);
        ERRDP(b"#", DSCALE, ctx);
        SIGERR(b"SPICE(NONPOSITIVESCALE)", ctx)?;
        CHKOUT(b"SPKW20", ctx)?;
        return Ok(());
    }

    if (TSCALE <= 0.0) {
        SETMSG(b"The time scale is not positive.TSCALE = #", ctx);
        ERRDP(b"#", TSCALE, ctx);
        SIGERR(b"SPICE(NONPOSITIVESCALE)", ctx)?;
        CHKOUT(b"SPKW20", ctx)?;
        return Ok(());
    }

    //
    // The begin time of the first record must be less than or equal
    // to the begin time of the segment. Convert the two-part input
    // epoch to seconds past J2000 for the purpose of this check.
    //
    BTIME = (SPD() * ((INITJD - J2000()) + INITFR));

    LTIME = (BTIME + (((N as f64) * INTLEN) * SPD()));

    //
    // Compute the tolerance to use for descriptor time bound checks.
    //
    TOL = (TOLSCL * intrinsics::DMAX1(&[f64::abs(BTIME), f64::abs(LTIME)]));

    if (FIRST < (BTIME - TOL)) {
        SETMSG(b"The segment descriptor start time # is too much less than the beginning time of the segment data # (in seconds past J2000: #). The difference is # seconds; the tolerance is # seconds.", ctx);
        ETCAL(FIRST, &mut ETSTR, ctx);
        ERRCH(b"#", &ETSTR, ctx);
        ETCAL(BTIME, &mut ETSTR, ctx);
        ERRCH(b"#", &ETSTR, ctx);
        ERRDP(b"#", FIRST, ctx);
        ERRDP(b"#", (BTIME - FIRST), ctx);
        ERRDP(b"#", TOL, ctx);
        SIGERR(b"SPICE(COVERAGEGAP)", ctx)?;
        CHKOUT(b"SPKW20", ctx)?;
        return Ok(());
    }

    //
    // The end time of the final record must be greater than or
    // equal to the end time of the segment.
    //
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
        CHKOUT(b"SPKW20", ctx)?;
        return Ok(());
    }

    //
    // Now check the validity of the segment identifier.
    //
    CHCKID(b"SPK segment identifier", SIDLEN, SEGID, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SPKW20", ctx)?;
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
    // Begin a new segment of SPK type 20 form:
    //
    //    Record 1
    //    Record 2
    //    ...
    //    Record N
    //    DSCALE     ( distance scale in km )
    //    TSCALE     ( time scale in seconds )
    //    INITJD     ( integer part of initial epoch of first record,
    //                 expressed as a TDB Julian date )
    //    INITFR     ( fractional part of initial epoch, in units of
    //                 TDB Julian days )
    //    INTLEN     ( length of interval covered by each record, in
    //                 units of TDB Julian days )
    //    RSIZE      ( number of data elements in each record )
    //    N          ( number of records in segment )
    //
    // Each record will have the form:
    //
    //    X coefficients
    //    X position component at interval midpoint
    //    Y coefficients
    //    Y position component at interval midpoint
    //    Z coefficients
    //    Z position component at interval midpoint
    //
    //
    DAFBNA(HANDLE, DESCR.as_slice(), SEGID, ctx)?;

    //
    // Calculate the number of entries in a record.
    //
    NINREC = ((POLYDG + 2) * 3);

    //
    // Fill segment with N records of data.
    //
    DAFADA(CDATA.as_slice(), (N * NINREC), ctx)?;

    //
    // Store the distance and time scales.
    //
    DAFADA(&[DSCALE], 1, ctx)?;
    DAFADA(&[TSCALE], 1, ctx)?;

    //
    // Store the integer and fractional parts of the initial epoch of
    // the first record.
    //
    DAFADA(&[INITJD], 1, ctx)?;
    DAFADA(&[INITFR], 1, ctx)?;

    //
    // Store the length of interval covered by each record.
    //
    DAFADA(&[INTLEN], 1, ctx)?;

    //
    // Store the size of each record (total number of array elements).
    // Note that this size is smaller by 2 than the size of a type 2
    // record of the same degree, since the record coverage midpoint
    // and radius are not stored.
    //
    DAFADA(&[(NINREC as f64)], 1, ctx)?;

    //
    // Store the number of records contained in the segment.
    //
    NUMREC = N as f64;
    DAFADA(&[NUMREC], 1, ctx)?;

    //
    // End this segment.
    //
    DAFENA(ctx)?;

    CHKOUT(b"SPKW20", ctx)?;
    Ok(())
}
