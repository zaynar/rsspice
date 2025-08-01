//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
const KWINTV: &[u8] = b"INTERVAL";
const KWSEG: &[u8] = b"SEGMENT";
const LNSIZE: i32 = 80;
const ND: i32 = 2;
const NI: i32 = 6;

/// CK coverage
///
/// Find the coverage window for a specified object in a specified CK
/// file.
///
/// # Required Reading
///
/// * [CELLS](crate::required_reading::cells)
/// * [DAF](crate::required_reading::daf)
/// * [CK](crate::required_reading::ck)
/// * [TIME](crate::required_reading::time)
/// * [WINDOWS](crate::required_reading::windows)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  CKFNM      I   Name of CK file.
///  IDCODE     I   ID code of object.
///  NEEDAV     I   Flag indicating whether angular velocity is needed.
///  LEVEL      I   Coverage level: 'SEGMENT' OR 'INTERVAL'.
///  TOL        I   Tolerance in ticks.
///  TIMSYS     I   Time system used to represent coverage.
///  COVER     I-O  Window giving coverage for IDCODE.
/// ```
///
/// # Detailed Input
///
/// ```text
///  CKFNM    is the name of a C-kernel.
///
///  IDCODE   is the integer ID code of an object, normally a
///           spacecraft structure or instrument, for which pointing
///           data are expected to exist in the specified CK file.
///
///  NEEDAV   is a logical variable indicating whether only segments
///           having angular velocity are to be considered when
///           determining coverage. When NEEDAV is .TRUE., segments
///           without angular velocity don't contribute to the coverage
///           window; when NEEDAV is .FALSE., all segments for IDCODE
///           may contribute to the coverage window.
///
///  LEVEL    is the level (granularity) at which the coverage is
///           examined. Allowed values and corresponding meanings are:
///
///              'SEGMENT'    The output coverage window contains
///                           intervals defined by the start and stop
///                           times of segments for the object
///                           designated by IDCODE.
///
///              'INTERVAL'   The output coverage window contains
///                           interpolation intervals of segments for
///                           the object designated by IDCODE. For type
///                           1 segments, which don't have
///                           interpolation intervals, each epoch
///                           associated with a pointing instance is
///                           treated as a singleton interval; these
///                           intervals are added to the coverage
///                           window.
///
///                           All interpolation intervals are
///                           considered to lie within the segment
///                           bounds for the purpose of this summary:
///                           if an interpolation interval extends
///                           beyond the segment coverage interval,
///                           only its intersection with the segment
///                           coverage interval is considered to
///                           contribute to the total coverage.
///
///  TOL      is a tolerance value expressed in ticks of the spacecraft
///           clock associated with IDCODE. Before each interval is
///           inserted into the coverage window, the interval is
///           intersected with the segment coverage interval, then if
///           the intersection is non-empty, it is expanded by TOL: the
///           left endpoint of the intersection interval is reduced by
///           TOL and the right endpoint is increased by TOL. Adjusted
///           interval endpoints, when expressed as encoded SCLK, never
///           are less than zero ticks. Any intervals that overlap as a
///           result of the expansion are merged.
///
///           The coverage window returned when TOL > 0 indicates the
///           coverage provided by the file to the CK readers CKGPAV
///           and CKGP when that value of TOL is passed to them as an
///           input.
///
///  TIMSYS   is a string indicating the time system used in the output
///           coverage window. TIMSYS may have the values:
///
///               'SCLK'    Elements of COVER are expressed in encoded
///                         SCLK ("ticks"), where the clock is
///                         associated with the object designated by
///                         IDCODE.
///
///               'TDB'     Elements of COVER are expressed as seconds
///                         past J2000 TDB.
///
///
///  COVER    is an initialized SPICE window data structure. COVER
///           optionally may contain coverage data on input; on output,
///           the data already present in COVER will be combined with
///           coverage found for the object designated by IDCODE in the
///           file CKFNM.
///
///           If COVER contains no data on input, its size and
///           cardinality still must be initialized.
/// ```
///
/// # Detailed Output
///
/// ```text
///  COVER    is a SPICE window data structure which represents the
///           merged coverage for IDCODE. When the coverage level is
///           'INTERVAL', this is the set of time intervals for which
///           data for IDCODE are present in the file CKFNM, merged
///           with the set of time intervals present in COVER on input.
///           The merged coverage is represented as the union of one or
///           more disjoint time intervals. The window COVER contains
///           the pairs of endpoints of these intervals.
///
///           When the coverage level is 'SEGMENT', COVER is computed
///           in a manner similar to that described above, but the
///           coverage intervals used in the computation are those of
///           segments rather than interpolation intervals within
///           segments.
///
///           When TOL is > 0, the intervals comprising the coverage
///           window for IDCODE are expanded by TOL and any intervals
///           overlapping as a result are merged. The resulting window
///           is returned in COVER. The expanded window in no case
///           extends beyond the segment bounds in either direction by
///           more than TOL.
///
///           The interval endpoints contained in COVER are encoded
///           spacecraft clock times if TIMSYS is 'SCLK'; otherwise the
///           times are converted from encoded spacecraft clock to
///           seconds past J2000 TDB.
///
///           See the $Examples section below for a complete example
///           program showing how to retrieve the endpoints from COVER.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input file has transfer format, the error
///      SPICE(INVALIDFORMAT) is signaled.
///
///  2)  If the input file is not a transfer file but has architecture
///      other than DAF, the error SPICE(INVALIDARCHTYPE) is signaled.
///
///  3)  If the input file is a binary DAF file of type other than CK,
///      the error SPICE(INVALIDFILETYPE) is signaled.
///
///  4)  If the CK file cannot be opened or read, an error is signaled
///      by a routine in the call tree of this routine. The output
///      window will not be modified.
///
///  5)  If the size of the output window argument COVER is
///      insufficient to contain the actual number of intervals in the
///      coverage window for IDCODE, an error is signaled by a routine
///      in the call tree of this routine.
///
///  6)  If TOL is negative, the error SPICE(VALUEOUTOFRANGE) is
///      signaled.
///
///  7)  If LEVEL is not recognized, the error SPICE(INVALIDOPTION)
///      is signaled.
///
///  8)  If TIMSYS is not recognized, the error SPICE(NOTSUPPORTED)
///      is signaled.
///
///  9)  If a time conversion error occurs, the error is signaled by a
///      routine in the call tree of this routine.
///
///  10) If the output time system is TDB, the CK subsystem must be
///      able to map IDCODE to the ID code of the associated spacecraft
///      clock. If this mapping cannot be performed, an error is
///      signaled by a routine in the call tree of this routine.
///
///  11) If the input CK type is not one of the supported CK types, the
///      error SPICE(NOTSUPPORTED) is signaled. This problem may
///      indicate the version of the SPICE Toolkit being used is
///      outdated and a new version is required.
/// ```
///
/// # Files
///
/// ```text
///  This routine reads a C-kernel.
///
///  If the output time system is 'TDB', then a leapseconds kernel
///  and an SCLK kernel for the spacecraft clock associated with
///  IDCODE must be loaded before this routine is called.
///
///  If the ID code of the clock associated with IDCODE is not
///  equal to
///
///     IDCODE / 1000
///
///  then the kernel variable
///
///     CK_<IDCODE>_SCLK
///
///  must be present in the kernel pool to identify the clock
///  associated with IDCODE. This variable must contain the ID code
///  to be used for conversion between SCLK and TDB. Normally this
///  variable is provided in a text kernel loaded via FURNSH.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine provides an API via which applications can determine
///  the coverage a specified CK file provides for a specified
///  object.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for these examples may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Display the interval-level coverage for each object in a
///     specified CK file. Use tolerance of zero ticks. Do not
///     request angular velocity. Express the results in the TDB time
///     system.
///
///     Find the set of objects in the file. Loop over the contents
///     of the ID code set: find the coverage for each item in the
///     set and display the coverage.
///
///
///     Example code begins here.
///
///
///           PROGRAM CKCOV_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           INTEGER               WNCARD
///           INTEGER               CARDI
///     C
///     C     Local parameters
///     C
///     C
///     C     Declare the coverage window.  Make enough room
///     C     for MAXIV intervals.
///     C
///           INTEGER               FILSIZ
///           PARAMETER           ( FILSIZ = 255 )
///
///           INTEGER               LBCELL
///           PARAMETER           ( LBCELL = -5 )
///
///           INTEGER               MAXIV
///           PARAMETER           ( MAXIV  = 100000 )
///
///           INTEGER               WINSIZ
///           PARAMETER           ( WINSIZ = 2 * MAXIV )
///
///           INTEGER               TIMLEN
///           PARAMETER           ( TIMLEN = 50 )
///
///           INTEGER               MAXOBJ
///           PARAMETER           ( MAXOBJ = 1000 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(FILSIZ)    CKFNM
///           CHARACTER*(FILSIZ)    LSK
///           CHARACTER*(FILSIZ)    SCLK
///           CHARACTER*(TIMLEN)    TIMSTR
///
///           DOUBLE PRECISION      B
///           DOUBLE PRECISION      COVER ( LBCELL : WINSIZ )
///           DOUBLE PRECISION      E
///
///           INTEGER               I
///           INTEGER               IDS   ( LBCELL : MAXOBJ )
///           INTEGER               J
///           INTEGER               NIV
///
///     C
///     C     Load a leapseconds kernel and SCLK kernel for output
///     C     time conversion.  Note that we assume a single spacecraft
///     C     clock is associated with all of the objects in the CK.
///     C
///           CALL PROMPT ( 'Name of leapseconds kernel > ', LSK  )
///           CALL FURNSH ( LSK )
///
///           CALL PROMPT ( 'Name of SCLK kernel        > ', SCLK )
///           CALL FURNSH ( SCLK )
///
///     C
///     C     Get name of CK file.
///     C
///           CALL PROMPT ( 'Name of CK file            > ', CKFNM )
///
///     C
///     C     Initialize the set IDS.
///     C
///           CALL SSIZEI ( MAXOBJ, IDS )
///
///     C
///     C     Initialize the window COVER.
///     C
///           CALL SSIZED ( WINSIZ, COVER )
///
///     C
///     C     Find the set of objects in the CK file.
///     C
///           CALL CKOBJ ( CKFNM, IDS )
///
///     C
///     C     We want to display the coverage for each object.  Loop
///     C     over the contents of the ID code set, find the coverage
///     C     for each item in the set, and display the coverage.
///     C
///           DO I = 1, CARDI( IDS )
///     C
///     C        Find the coverage window for the current
///     C        object. Empty the coverage window each time
///     C        so we don't include data for the previous object.
///     C
///              CALL SCARDD ( 0,   COVER )
///              CALL CKCOV  ( CKFNM,       IDS(I),  .FALSE.,
///          .                 'INTERVAL',  0.D0,    'TDB',    COVER )
///
///     C
///     C        Get the number of intervals in the coverage
///     C        window.
///     C
///              NIV = WNCARD( COVER )
///
///     C
///     C        Display a simple banner.
///     C
///              WRITE (*,*) '========================================'
///              WRITE (*,*) 'Coverage for object ', IDS(I)
///
///     C
///     C        Convert the coverage interval start and stop
///     C        times to TDB calendar strings.
///     C
///              DO J = 1, NIV
///     C
///     C           Get the endpoints of the Jth interval.
///     C
///                 CALL WNFETD ( COVER, J, B, E )
///     C
///     C           Convert the endpoints to TDB calendar
///     C           format time strings and display them.
///     C
///                 CALL TIMOUT ( B,
///          .                    'YYYY MON DD HR:MN:SC.###### ' //
///          .                    '(TDB) ::TDB',
///          .                    TIMSTR                           )
///                 WRITE (*,*) ' '
///                 WRITE (*,*) 'Interval: ', J
///                 WRITE (*,*) 'Start:    ', TIMSTR
///
///                 CALL TIMOUT ( E,
///          .                    'YYYY MON DD HR:MN:SC.###### ' //
///          .                    '(TDB) ::TDB',
///          .                    TIMSTR                          )
///                 WRITE (*,*) 'Stop:     ', TIMSTR
///                 WRITE (*,*) ' '
///
///              END DO
///
///              WRITE (*,*) '========================================'
///
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, using the LSK file named naif0010.tls, the SCLK file
///     named cas00145.tsc and the CK file named 08052_08057ra.bc, the
///     output was:
///
///
///     Name of leapseconds kernel > naif0010.tls
///     Name of SCLK kernel        > cas00145.tsc
///     Name of CK file            > 08052_08057ra.bc
///      ========================================
///      Coverage for object       -82000
///
///      Interval:            1
///      Start:    2008 FEB 21 00:01:07.771186 (TDB)
///      Stop:     2008 FEB 23 22:53:30.001738 (TDB)
///
///
///      Interval:            2
///      Start:    2008 FEB 23 22:58:13.999732 (TDB)
///      Stop:     2008 FEB 24 02:22:25.913175 (TDB)
///
///
///      Interval:            3
///      Start:    2008 FEB 24 02:27:49.910886 (TDB)
///      Stop:     2008 FEB 24 19:46:33.470587 (TDB)
///
///
///      Interval:            4
///      Start:    2008 FEB 24 19:49:33.469315 (TDB)
///      Stop:     2008 FEB 25 04:25:21.250677 (TDB)
///
///
///      Interval:            5
///      Start:    2008 FEB 25 04:29:33.248897 (TDB)
///      Stop:     2008 FEB 25 15:23:44.971594 (TDB)
///
///
///      Interval:            6
///      Start:    2008 FEB 25 15:24:12.971396 (TDB)
///      Stop:     2008 FEB 25 20:25:04.843864 (TDB)
///
///
///      Interval:            7
///      Start:    2008 FEB 25 20:25:48.843553 (TDB)
///      Stop:     2008 FEB 26 00:01:04.752306 (TDB)
///
///      ========================================
///
///
///  2) Find the segment-level coverage for the object designated by
///     IDCODE provided by the set of CK files loaded via a
///     metakernel. (The metakernel must also specify leapseconds and
///     SCLK kernels.) Use tolerance of zero ticks. Do not request
///     angular velocity. Express the results in the TDB time system.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: ckcov_ex2.tm
///
///        This meta-kernel is intended to support operation of SPICE
///        example programs. The kernels shown here should not be
///        assumed to contain adequate or correct versions of data
///        required by SPICE-based user applications.
///
///        In order for an application to use this meta-kernel, the
///        kernels referenced here must be present in the user's
///        current working directory.
///
///        The names and contents of the kernels referenced
///        by this meta-kernel are as follows:
///
///          File name                      Contents
///          ---------                      --------
///          naif0010.tls                   Leapseconds
///          cas00145.tsc                   Cassini SCLK
///          08052_08057ra.bc               Orientation for Cassini
///
///        \begindata
///
///          KERNELS_TO_LOAD = ( 'naif0010.tls'
///                              'cas00145.tsc'
///                              '08052_08057ra.bc')
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM CKCOV_EX2
///           IMPLICIT NONE
///     C
///     C     SPICELIB functions
///     C
///           INTEGER               WNCARD
///
///     C
///     C     Local parameters
///     C
///           INTEGER               LBCELL
///           PARAMETER           ( LBCELL = -5 )
///
///           INTEGER               FILSIZ
///           PARAMETER           ( FILSIZ = 255 )
///
///           INTEGER               LNSIZE
///           PARAMETER           ( LNSIZE = 80 )
///
///           INTEGER               MAXCOV
///           PARAMETER           ( MAXCOV = 100000 )
///
///           INTEGER               TIMLEN
///           PARAMETER           ( TIMLEN = 50 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(FILSIZ)    FILE
///           CHARACTER*(LNSIZE)    IDCH
///           CHARACTER*(FILSIZ)    META
///           CHARACTER*(FILSIZ)    SOURCE
///           CHARACTER*(TIMLEN)    TIMSTR
///           CHARACTER*(LNSIZE)    TYPE
///
///           DOUBLE PRECISION      B
///           DOUBLE PRECISION      COVER  ( LBCELL : 2*MAXCOV )
///           DOUBLE PRECISION      E
///
///           INTEGER               COUNT
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               IDCODE
///           INTEGER               NIV
///
///           LOGICAL               FOUND
///
///     C
///     C     Prompt for the metakernel name; load the metakernel.
///     C     The metakernel lists the CK files whose coverage
///     C     for IDCODE we'd like to determine.  The metakernel
///     C     must also specify a leapseconds kernel and an SCLK
///     C     kernel for the clock associated with IDCODE.
///     C
///           CALL PROMPT ( 'Enter name of metakernel > ', META )
///
///           CALL FURNSH ( META )
///
///     C
///     C     Get the ID code of interest.
///     C
///           CALL PROMPT ( 'Enter ID code            > ', IDCH )
///
///           CALL PRSINT ( IDCH,  IDCODE )
///
///     C
///     C     Initialize the coverage window.
///     C
///           CALL SSIZED ( MAXCOV, COVER )
///
///     C
///     C     Find out how many kernels are loaded.  Loop over the
///     C     kernels:  for each loaded CK file, add its coverage
///     C     for IDCODE, if any, to the coverage window.
///     C
///           CALL KTOTAL ( 'CK', COUNT )
///
///           DO I = 1, COUNT
///
///              CALL KDATA ( I,       'CK',    FILE,  TYPE,
///          .                SOURCE,  HANDLE,  FOUND       )
///
///              CALL CKCOV ( FILE,      IDCODE,  .FALSE.,
///          .                'SEGMENT', 0.D0,    'TDB',    COVER )
///
///           END DO
///
///     C
///     C     Display results.
///     C
///     C     Get the number of intervals in the coverage
///     C     window.
///     C
///           NIV = WNCARD( COVER )
///
///     C
///     C     Display a simple banner.
///     C
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Coverage for object ', IDCODE
///
///     C
///     C     Convert the coverage interval start and stop
///     C     times to TDB calendar strings.
///     C
///           DO I = 1, NIV
///     C
///     C        Get the endpoints of the Ith interval.
///     C
///              CALL WNFETD ( COVER, I, B, E )
///     C
///     C        Convert the endpoints to TDB calendar
///     C        format time strings and display them.
///     C
///              CALL TIMOUT ( B,
///          .                 'YYYY MON DD HR:MN:SC.###### ' //
///          .                 '(TDB) ::TDB',
///          .                 TIMSTR                           )
///              WRITE (*,*) ' '
///              WRITE (*,*) 'Interval: ', I
///              WRITE (*,*) 'Start:    ', TIMSTR
///
///              CALL TIMOUT ( E,
///          .                 'YYYY MON DD HR:MN:SC.###### ' //
///          .                 '(TDB) ::TDB',
///          .                 TIMSTR                           )
///              WRITE (*,*) 'Stop:     ', TIMSTR
///              WRITE (*,*) ' '
///
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, using the meta-kernel file named ckcov_ex2.tm and
///     the NAIF ID "-82000" (Cassini spacecraft bus), the output was:
///
///
///     Enter name of metakernel > ckcov_ex2.tm
///     Enter ID code            > -82000
///
///      Coverage for object       -82000
///
///      Interval:            1
///      Start:    2008 FEB 21 00:01:07.771186 (TDB)
///      Stop:     2008 FEB 26 00:01:04.752306 (TDB)
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  When this routine is used to accumulate coverage for IDCODE
///      provided by multiple CK files, the inputs NEEDAV, LEVEL, TOL,
///      and TIMSYS  must have the same values for all files in order
///      for the result to be meaningful.
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
/// -    SPICELIB Version 2.1.0, 08-OCT-2021 (JDR)
///
///         Bug fix: added call to FAILED after call to GETFAT.
///
///         Changed input argument name "CK" to "CKFNM" for consistency
///         with other routines.
///
///         Edited the header to comply with NAIF standard. Added solutions
///         using CASSINI data. Fixed a bug on Example #2. Added entry #11
///         in $Exceptions section and corrected short error messages in
///         entry #2 and #3.
///
/// -    SPICELIB Version 2.0.0, 05-JAN-2014 (NJB) (BVS)
///
///         Updated index entries.
///
///         Last update was 05-JAN-2014 (NJB) (BVS)
///
///            Updated to support type 6.
///
/// -    SPICELIB Version 1.0.1, 30-NOV-2007 (NJB)
///
///         Corrected bug in first program in header $Examples section:
///         program now empties the coverage window prior to collecting
///         data for the current object. Updated examples to use WNCARD
///         rather than CARDD.
///
/// -    SPICELIB Version 1.0.0, 07-JAN-2005 (NJB)
/// ```
pub fn ckcov(
    ctx: &mut SpiceContext,
    ckfnm: &str,
    idcode: i32,
    needav: bool,
    level: &str,
    tol: f64,
    timsys: &str,
    cover: &mut [f64],
) -> crate::Result<()> {
    CKCOV(
        ckfnm.as_bytes(),
        idcode,
        needav,
        level.as_bytes(),
        tol,
        timsys.as_bytes(),
        cover,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CKCOV ( CK coverage )
pub fn CKCOV(
    CKFNM: &[u8],
    IDCODE: i32,
    NEEDAV: bool,
    LEVEL: &[u8],
    TOL: f64,
    TIMSYS: &[u8],
    COVER: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut COVER = DummyArrayMut::new(COVER, LBCELL..);
    let mut ARCH = [b' '; LNSIZE as usize];
    let mut KERTYP = [b' '; LNSIZE as usize];
    let mut DC = StackArray::<f64, 2>::new(1..=ND);
    let mut DCTOL = StackArray::<f64, 2>::new(1..=ND);
    let mut DESCR = StackArray::<f64, 5>::new(1..=(ND + ((NI + 1) / 2)));
    let mut ET: f64 = 0.0;
    let mut CLKID: i32 = 0;
    let mut DTYPE: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut IC = StackArray::<i32, 6>::new(1..=NI);
    let mut SEGBEG: i32 = 0;
    let mut SEGEND: i32 = 0;
    let mut AVOK: bool = false;
    let mut FOUND: bool = false;
    let mut ISTDB: bool = false;
    let mut SEGLVL: bool = false;

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
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"CKCOV", ctx)?;

    //
    // Check tolerance value.
    //
    if (TOL < 0.0) {
        SETMSG(b"Tolerance must be non-negative; actual value was #.", ctx);
        ERRDP(b"#", TOL, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"CKCOV", ctx)?;
        return Ok(());
    }

    //
    // Use a logical flag to indicate whether this is a segment-level
    // coverage description.
    //
    SEGLVL = EQSTR(LEVEL, KWSEG);

    //
    // Check coverage level keyword.
    //
    if !(SEGLVL || EQSTR(LEVEL, KWINTV)) {
        SETMSG(
            b"Allowed values of LEVEL are # and #; actual value was #.",
            ctx,
        );
        ERRCH(b"#", KWSEG, ctx);
        ERRCH(b"#", KWINTV, ctx);
        ERRCH(b"#", LEVEL, ctx);
        SIGERR(b"SPICE(INVALIDOPTION)", ctx)?;
        CHKOUT(b"CKCOV", ctx)?;
        return Ok(());
    }

    //
    // See whether GETFAT thinks we've got a CK file.
    //
    GETFAT(CKFNM, &mut ARCH, &mut KERTYP, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"CKCOV", ctx)?;
        return Ok(());
    }

    if fstr::eq(&ARCH, b"XFR") {
        SETMSG(b"Input file # has architecture #. The file must be a binary CK file to be readable by this routine.  If the input file is an CK file in transfer format, run TOBIN on the file to convert it to binary format.", ctx);
        ERRCH(b"#", CKFNM, ctx);
        ERRCH(b"#", &ARCH, ctx);
        SIGERR(b"SPICE(INVALIDFORMAT)", ctx)?;
        CHKOUT(b"CKCOV", ctx)?;
        return Ok(());
    } else if fstr::ne(&ARCH, b"DAF") {
        SETMSG(b"Input file # has architecture #. The file must be a binary CK file to be readable by this routine.  Binary CK files have DAF architecture.  If you expected the file to be a binary CK file, the problem may be due to the file being an old non-native file lacking binary file format information. It\'s also possible the file has been corrupted.", ctx);
        ERRCH(b"#", CKFNM, ctx);
        ERRCH(b"#", &ARCH, ctx);
        SIGERR(b"SPICE(INVALIDARCHTYPE)", ctx)?;
        CHKOUT(b"CKCOV", ctx)?;
        return Ok(());
    } else if fstr::ne(&KERTYP, b"CK") {
        SETMSG(b"Input file # has file type #. The file must be a binary CK file to be readable by this routine. If you expected the file to be a binary CK file, the problem may be due to the file being an old non-native file lacking binary file format information. It\'s also possible the file has been corrupted.", ctx);
        ERRCH(b"#", CKFNM, ctx);
        ERRCH(b"#", &KERTYP, ctx);
        SIGERR(b"SPICE(INVALIDFILETYPE)", ctx)?;
        CHKOUT(b"CKCOV", ctx)?;
        return Ok(());
    }

    //
    // Set a logical flag indicating whether the time system is SCLK.
    //
    ISTDB = EQSTR(TIMSYS, b"TDB");

    //
    // Check time system.
    //
    if !ISTDB {
        if !EQSTR(TIMSYS, b"SCLK") {
            SETMSG(
                b"Time system spec TIMSYS was #; allowed values are SCLK and TDB.",
                ctx,
            );
            ERRCH(b"#", TIMSYS, ctx);
            SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
            CHKOUT(b"CKCOV", ctx)?;
            return Ok(());
        }
    }

    //
    // If the output time system is TDB, find the clock ID associated
    // with IDCODE.
    //
    if ISTDB {
        CKMETA(IDCODE, b"SCLK", &mut CLKID, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"CKCOV", ctx)?;
            return Ok(());
        }
    }

    //
    // Open the file for reading.
    //
    DAFOPR(CKFNM, &mut HANDLE, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"CKCOV", ctx)?;
        return Ok(());
    }

    //
    // We will examine each segment descriptor in the file, and
    // we'll update our coverage bounds according to the data found
    // in these descriptors.
    //
    // If TOL > 0, we'll apply TOL after we've found the coverage
    // for the zero-tolerance case.
    //
    // If the time system is TDB, we'll convert the times to TDB
    // at the end of this routine.

    //
    // Start a forward search.
    //
    DAFBFS(HANDLE, ctx)?;

    //
    // Find the next DAF array.
    //
    DAFFNA(&mut FOUND, ctx)?;

    while FOUND {
        //
        // Note:  we check FAILED() at the bottom of this loop; this
        // routine returns if FAILED() returns .TRUE. at that point.
        //
        // Fetch and unpack the segment descriptor.
        //
        DAFGS(DESCR.as_slice_mut(), ctx)?;
        DAFUS(
            DESCR.as_slice(),
            ND,
            NI,
            DC.as_slice_mut(),
            IC.as_slice_mut(),
        );

        //
        // Let AVOK indicate whether the segment satisfies the
        // angular velocity restriction.
        //
        AVOK = ((IC[4] == 1) || !NEEDAV);

        if ((IC[1] == IDCODE) && AVOK) {
            //
            // This segment is for the body of interest.  If angular
            // velocity is needed, this segment has it.
            //
            if SEGLVL {
                //
                // This is a segment-level summary.
                //
                // Insert the coverage bounds into the coverage window.
                // Adjust the interval using the tolerance.
                //
                DCTOL[1] = intrinsics::DMAX1(&[(DC[1] - TOL), 0.0]);
                DCTOL[2] = (DC[2] + TOL);

                //
                // Convert the time to TDB if necessary.
                //
                if ISTDB {
                    //
                    // Convert the time bounds to TDB before inserting
                    // into the window.
                    //
                    for I in 1..=2 {
                        SCT2E(CLKID, DCTOL[I], &mut ET, ctx)?;
                        DCTOL[I] = ET;
                    }
                }

                if (DCTOL[1] <= DCTOL[2]) {
                    WNINSD(DCTOL[1], DCTOL[2], COVER.as_slice_mut(), ctx)?;
                }
            } else {
                //
                // We're looking for an interval-level coverage window.
                // This information must be retrieved in a
                // data-type-dependent fashion.  The coverage routines
                // we'll call will, if necessary, adjust intervals by TOL
                // and convert interval times to TDB.
                //
                DTYPE = IC[3];
                SEGBEG = IC[5];
                SEGEND = IC[6];

                if (DTYPE == 1) {
                    ZZCKCV01(
                        HANDLE,
                        SEGBEG,
                        SEGEND,
                        CLKID,
                        TOL,
                        TIMSYS,
                        COVER.as_slice_mut(),
                        ctx,
                    )?;
                } else if (DTYPE == 2) {
                    ZZCKCV02(
                        HANDLE,
                        SEGBEG,
                        SEGEND,
                        CLKID,
                        TOL,
                        TIMSYS,
                        COVER.as_slice_mut(),
                        ctx,
                    )?;
                } else if (DTYPE == 3) {
                    ZZCKCV03(
                        HANDLE,
                        SEGBEG,
                        SEGEND,
                        CLKID,
                        TOL,
                        TIMSYS,
                        COVER.as_slice_mut(),
                        ctx,
                    )?;
                } else if (DTYPE == 4) {
                    ZZCKCV04(
                        HANDLE,
                        SEGBEG,
                        SEGEND,
                        CLKID,
                        TOL,
                        TIMSYS,
                        COVER.as_slice_mut(),
                        ctx,
                    )?;
                } else if (DTYPE == 5) {
                    ZZCKCV05(
                        HANDLE,
                        SEGBEG,
                        SEGEND,
                        CLKID,
                        DC.as_slice(),
                        TOL,
                        TIMSYS,
                        COVER.as_slice_mut(),
                        ctx,
                    )?;
                } else if (DTYPE == 6) {
                    ZZCKCV06(
                        HANDLE,
                        SEGBEG,
                        SEGEND,
                        CLKID,
                        DC.as_slice(),
                        TOL,
                        TIMSYS,
                        COVER.as_slice_mut(),
                        ctx,
                    )?;
                } else {
                    SETMSG(b"Supported CK data types are 1, 2, 3, 4, 5.  Data type of segment: #. This problem may indicate that you need to update your SPICE Toolkit.", ctx);
                    ERRINT(b"#", DTYPE, ctx);
                    SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
                    CHKOUT(b"CKCOV", ctx)?;
                    return Ok(());
                }
            }
        }

        DAFFNA(&mut FOUND, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"CKCOV", ctx)?;
            return Ok(());
        }
    }

    //
    // COVER now represents the coverage of the entire file at the
    // granularity indicated by LEVEL, combined with the coverage
    // contained in COVER on input.
    //
    // Release the file.
    //
    DAFCLS(HANDLE, ctx)?;

    CHKOUT(b"CKCOV", ctx)?;
    Ok(())
}
