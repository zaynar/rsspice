//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const NDELIM: i32 = 5;
const DELIMS: &[u8; NDELIM as usize] = &fstr::extend_const::<{ NDELIM as usize }>(b".:-, ");
const MXPART: i32 = 9999;
const PARTLN: i32 = 5;
const MXCOEF: i32 = 100000;
const MXNFLD: i32 = 10;
const DPLEN: i32 = 30;
const MXTSYS: i32 = 2;

struct SaveVars {
    PSTART: ActualArray<f64>,
    PSTOP: ActualArray<f64>,
    PTOTLS: ActualArray<f64>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PSTART = ActualArray::<f64>::new(1..=MXPART);
        let mut PSTOP = ActualArray::<f64>::new(1..=MXPART);
        let mut PTOTLS = ActualArray::<f64>::new(1..=MXPART);

        Self {
            PSTART,
            PSTOP,
            PTOTLS,
        }
    }
}

/// Decode spacecraft clock
///
/// Convert a double precision encoding of spacecraft clock time into
/// a character representation.
///
/// # Required Reading
///
/// * [SCLK](crate::required_reading::sclk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  SC         I   NAIF spacecraft identification code.
///  SCLKDP     I   Encoded representation of a spacecraft clock count.
///  SCLKCH     O   Character representation of a clock count.
///  MXPART     P   Maximum number of spacecraft clock partitions.
/// ```
///
/// # Detailed Input
///
/// ```text
///  SC       is the NAIF integer code of the spacecraft whose
///           clock's time is being decoded.
///
///  SCLKDP   is the double precision encoding of a clock time in
///           units of ticks since the spacecraft clock start time.
///           This value does reflect partition information.
///
///           An analogy may be drawn between a spacecraft clock
///           and a standard wall clock. The number of ticks
///           corresponding to the wall clock string
///
///              hh:mm:ss
///
///           would be the number of seconds represented by that
///           time.
///
///           For example:
///
///              Clock string      Number of ticks
///              ------------      ---------------
///                00:00:10              10
///                00:01:00              60
///                00:10:00             600
///                01:00:00            3600
///
///           If SCLKDP contains a fractional part the result
///           is the same as if SCLKDP had been rounded to the
///           nearest whole number.
/// ```
///
/// # Detailed Output
///
/// ```text
///  SCLKCH   is the character representation of the clock count.
///           The exact form that SCLKCH takes depends on the
///           spacecraft.
///
///           Nevertheless, SCLKCH will have the following general
///           format:
///
///              'pp/sclk_string'
///
///           'pp' is an integer greater than or equal to one and
///           represents a "partition number".
///
///           Each mission is divided into some number of partitions.
///           A new partition starts when the spacecraft clock
///           resets, either to zero, or to some other
///           value. Thus, the first partition for any mission
///           starts with launch, and ends with the first clock
///           reset. The second partition starts immediately when
///           the first stopped, and so on.
///
///           In order to be completely unambiguous about a
///           particular time, you need to specify a partition number
///           along with the standard clock string.
///
///           Information about when partitions occur for different
///           missions is contained in a spacecraft clock kernel
///           file which needs to be loaded into the kernel pool
///           before calling SCDECD.
///
///           The routine SCPART may be used to read the partition
///           start and stop times, in encoded units of ticks, from
///           the kernel file.
///
///           Since the end time of one partition is coincident with
///           the begin time of the next, two different time strings
///           with different partition numbers can encode into the
///           same value.
///
///           For example, if partition 1 ends at time t1, and
///           partition 2 starts at time t2, then
///
///              '1/t1' and '2/t2'
///
///           will be encoded into the same value, say X. SCDECD
///           always decodes such values into the latter of the
///           two partitions. In this example,
///
///              CALL SCDECD ( X, SC, CLKSTR )
///
///           will result in
///
///              CLKSTR = '2/t2'.
///
///           'sclk_string' is a spacecraft specific clock string,
///           typically consisting of a number of components
///           separated by delimiters.
///
///           Using Galileo as an example, the full format is
///
///              wwwwwwww:xx:y:z
///
///           where z is a mod-8 counter (values 0-7) which
///           increments approximately once every 8 1/3 ms., y is a
///           mod-10 counter (values 0-9) which increments once
///           every time z turns over, i.e., approximately once every
///           66 2/3 ms., xx is a mod-91 (values 0-90) counter
///           which increments once every time y turns over, i.e.,
///           once every 2/3 seconds. wwwwwwww is the Real-Time Image
///           Count (RIM), which increments once every time xx turns
///           over, i.e., once every 60 2/3 seconds. The roll-over
///           expression for the RIM is 16777215, which corresponds
///           to approximately 32 years.
///
///           wwwwwwww, xx, y, and z are referred to interchangeably
///           as the fields or components of the spacecraft clock.
///           SCLK components may be separated by any of these five
///           characters: ' '  ':'  ','  '-'  '.'
///           The delimiter used is determined by a kernel pool
///           variable and can be adjusted by the user.
///
///           Some spacecraft clock components have offset, or
///           starting, values different from zero. For example,
///           with an offset value of 1, a mod 20 counter would
///           cycle from 1 to 20 instead of from 0 to 19.
///
///           See the SCLK required reading for a detailed
///           description of the Voyager and Mars Observer clock
///           formats.
/// ```
///
/// # Parameters
///
/// ```text
///  MXPART   is the maximum number of spacecraft clock partitions
///           expected in the kernel file for any one spacecraft.
///           See the INCLUDE file sclk.inc for this parameter's
///           value.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If kernel variables required by this routine are unavailable,
///      an error is signaled by a routine in the call tree of this
///      routine. SCLKCH will be returned as a blank string in this
///      case.
///
///  2)  If the number of partitions in the kernel file for spacecraft
///      SC exceeds the parameter MXPART, the error
///      SPICE(TOOMANYPARTS) is signaled. SCLKCH will be returned
///      as a blank string in this case.
///
///  3)  If the encoded value does not fall in the boundaries of the
///      mission, the error SPICE(VALUEOUTOFRANGE) is signaled.
///      SCLKCH will be returned as a blank string in this case.
///
///  4)  If the declared length of SCLKCH is not large enough to
///      contain the output clock string, the error
///      SPICE(SCLKTRUNCATED) is signaled by either this routine or a
///      routine in the call tree of this routine. On output SCLKCH
///      will contain a portion of the truncated clock string.
/// ```
///
/// # Files
///
/// ```text
///  A kernel file containing spacecraft clock partition information
///  for the desired spacecraft must be loaded, using the routine
///  FURNSH, before calling this routine.
/// ```
///
/// # Particulars
///
/// ```text
///  In general, it is difficult to compare spacecraft clock counts
///  numerically since there are too many clock components for a
///  single comparison. The routine SCENCD provides a method of
///  assigning a single double precision number to a spacecraft's
///  clock count, given one of its character representations.
///
///  This routine performs the inverse operation to SCENCD, converting
///  an encoded double precision number to character format.
///
///  To convert the number of ticks since the start of the mission to
///  a clock format character string, SCDECD:
///
///     1) Determines the spacecraft clock partition that TICKS falls
///        in.
///
///     2) Subtracts off the number of ticks occurring in previous
///        partitions, to get the number of ticks since the beginning
///        of the current partition.
///
///     3) Converts the resulting ticks to clock format and forms the
///        string
///
///           'partition_number/clock_string'
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as input,
///  the compiler and supporting libraries, and the machine specific
///  arithmetic implementation.
///
///  1) Double precision encodings of spacecraft clock counts are used
///     to tag pointing data in the C-kernel.
///
///     In the following example, pointing for a sequence of images
///     from the CASSINI Imaging Science Subsystem (ISS) is requested
///     from the C-kernel using an array of character spacecraft clock
///     counts as input. The clock counts attached to the output are
///     then decoded to character and compared with the input strings.
///
///     Use the CK kernel below to load the CASSINI image navigated
///     spacecraft pointing and orientation data.
///
///        04153_04182ca_ISS.bc
///
///
///     Use the SCLK kernel below to load the CASSINI spacecraft clock
///     time correlation data required for the conversion between
///     spacecraft clock string representation and double precision
///     encoding of spacecraft clock counts.
///
///        cas00071.tsc
///
///
///     Example code begins here.
///
///
///           PROGRAM SCDECD_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///     C     The instrument we want pointing for is the CASSINI
///     C     spacecraft. The reference frame we want is
///     C     J2000. The spacecraft is CASSINI.
///     C
///           INTEGER               SC
///           PARAMETER           ( SC     = -82 )
///
///           INTEGER               INST
///           PARAMETER           ( INST   = -82000 )
///
///           CHARACTER*(*)         REF
///           PARAMETER           ( REF    = 'J2000' )
///
///           CHARACTER*(*)         CK
///           PARAMETER           ( CK     = '04153_04182ca_ISS.bc' )
///
///           CHARACTER*(*)         SCLK
///           PARAMETER           ( SCLK   = 'cas00071.tsc' )
///
///           INTEGER               NPICS
///           PARAMETER           ( NPICS  = 4 )
///
///           CHARACTER*(*)         CLKTOL
///           PARAMETER           ( CLKTOL = '1.0' )
///
///           INTEGER               MAXLEN
///           PARAMETER           ( MAXLEN = 30 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(25)        SCLKIN (4)
///           CHARACTER*(25)        SCLKOUT
///
///           DOUBLE PRECISION      CMAT   (3,3)
///           DOUBLE PRECISION      TIMEIN
///           DOUBLE PRECISION      TIMEOUT
///           DOUBLE PRECISION      TOL
///
///           INTEGER               I
///           INTEGER               J
///           INTEGER               K
///
///           LOGICAL               FOUND
///
///     C
///     C     Set the input SCLK strings.
///     C
///           DATA                  SCLKIN /  '1/1465644279.0',
///          .                                '1/1465644281.0',
///          .                                '1/1465644351.0',
///          .                                '1/1465644361.0'  /
///
///     C
///     C     Load the appropriate files. We need
///     C
///     C        1. CK file containing pointing data.
///     C        2. Spacecraft clock kernel file.
///     C
///           CALL FURNSH ( CK   )
///           CALL FURNSH ( SCLK )
///
///     C
///     C     Convert the tolerance string to ticks.
///     C
///           CALL SCTIKS ( SC, CLKTOL, TOL )
///
///           DO I= 1, NPICS
///
///              CALL SCENCD ( SC, SCLKIN(I), TIMEIN )
///
///              CALL CKGP ( INST, TIMEIN,  TOL,  REF,
///          .               CMAT, TIMEOUT, FOUND     )
///
///              WRITE(*,*)
///              WRITE(*,'(2A)') 'Input s/c clock count : ', SCLKIN(I)
///
///              IF ( FOUND ) THEN
///
///                 CALL SCDECD ( SC, TIMEOUT, SCLKOUT )
///
///                 WRITE(*,'(2A)') 'Output s/c clock count: ',
///          .                                          SCLKOUT
///                 WRITE(*,'(A)') 'Output C-Matrix:'
///
///                 DO J = 1, 3
///
///                    WRITE(*,'(3F21.15)') ( CMAT(J,K), K = 1, 3 )
///
///                 END DO
///
///              ELSE
///
///                 WRITE(*,'(A)') 'No pointing found.'
///
///              END IF
///
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Input s/c clock count : 1/1465644279.0
///     No pointing found.
///
///     Input s/c clock count : 1/1465644281.0
///     Output s/c clock count: 1/1465644281.171
///     Output C-Matrix:
///        -0.335351455948710    0.864374440205611    0.374694846658341
///        -0.937887426812980   -0.343851965210223   -0.046184419961653
///         0.088918927227039   -0.366909598048763    0.925997176691424
///
///     Input s/c clock count : 1/1465644351.0
///     Output s/c clock count: 1/1465644351.071
///     Output C-Matrix:
///        -0.335380929397586    0.864363638262230    0.374693385378623
///        -0.937874292008090   -0.343889838107825   -0.046169163264003
///         0.088946301703530   -0.366899550417080    0.925998528787713
///
///     Input s/c clock count : 1/1465644361.0
///     No pointing found.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  Assumes that an SCLK kernel file appropriate for the clock
///      designated by SC is loaded in the kernel pool at the time
///      this routine is called.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  J.M. Lynch         (JPL)
///  W.L. Taber         (JPL)
///  R.E. Thurman       (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.2.0, 18-NOV-2021 (NJB) (JDR)
///
///         Now variables PSTART, PSTOP, and PTOTLS are saved. Made minor
///         changes to formatting of code.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example based on existing example fragments using PDS
///         archived CASSINI data.
///
///         Added FAILED() call after SCFMT call.
///
///         Removed unnecessary entries in $Revisions section.
///
/// -    SPICELIB Version 2.1.0, 05-FEB-2008 (NJB)
///
///         Values of parameter MXPART and PARTLN are now
///         provided by the INCLUDE file sclk.inc.
///
/// -    SPICELIB Version 2.0.1, 22-AUG-2006 (EDW)
///
///         Replaced references to LDPOOL with references
///         to FURNSH.
///
/// -    SPICELIB Version 2.0.0, 17-APR-1992 (JML) (WLT)
///
///         The routine was changed to signal an error when SCLKCH is
///         not long enough to contain the output spacecraft clock
///         string.
///
///         FAILED is now checked after calling SCPART.
///
///         References to CLPOOL were deleted.
///
///         Miscellaneous minor updates to the header were performed.
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 06-SEP-1990 (JML) (RET)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 2.0.0, 17-APR-1992 (JML) (WLT)
///
///         The routine was changed to signal an error when SCLKCH is
///         not long enough to contain the output spacecraft clock
///         string. Previously, the SCLK routines simply truncated
///         the clock string on the right. It was determined that
///         since this truncation could easily go undetected by the
///         user ( only the leftmost field of a clock string is
///         required when clock string is used as an input to a
///         SCLK routine ), it would be better to signal an error
///         when this happens.
///
///         FAILED is checked after calling SCPART in case an
///         error has occurred reading the kernel file and the
///         error action is not set to 'abort'.
///
///         References to CLPOOL were deleted.
///
///         Miscellaneous minor updates to the header were performed.
///
///         Comment section for permuted index source lines was added
///         following the header.
/// ```
pub fn scdecd(ctx: &mut SpiceContext, sc: i32, sclkdp: f64, sclkch: &mut str) -> crate::Result<()> {
    SCDECD(
        sc,
        sclkdp,
        fstr::StrBytes::new(sclkch).as_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SCDECD ( Decode spacecraft clock )
pub fn SCDECD(
    SC: i32,
    SCLKDP: f64,
    SCLKCH: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut PRTSTR = [b' '; PARTLN as usize];
    let mut NPARTS: i32 = 0;
    let mut PART: i32 = 0;
    let mut PRELEN: i32 = 0;
    let mut SUFLEN: i32 = 0;
    let mut TICKS: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"SCDECD", ctx)?;

    //
    // Use a working copy of the input.
    //
    TICKS = f64::round(SCLKDP);

    fstr::assign(SCLKCH, b" ");

    //
    // Read the partition start and stop times (in ticks) for this
    // mission. Error if there are too many of them.  Also need to
    // check FAILED in case error handling is not in ABORT or
    // DEFAULT mode.
    //
    SCPART(
        SC,
        &mut NPARTS,
        save.PSTART.as_slice_mut(),
        save.PSTOP.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"SCDECD", ctx)?;
        return Ok(());
    }

    if (NPARTS > MXPART) {
        //
        // This code should be unreachable. It is included for safety.
        //
        SETMSG(b"The number of partitions, #, for spacecraft # exceeds the value for parameter MXPART, #.", ctx);
        ERRINT(b"#", NPARTS, ctx);
        ERRINT(b"#", SC, ctx);
        ERRINT(b"#", MXPART, ctx);
        SIGERR(b"SPICE(TOOMANYPARTS)", ctx)?;
        CHKOUT(b"SCDECD", ctx)?;
        return Ok(());
    }

    //
    // For each partition, compute the total number of ticks in that
    // partition plus all preceding partitions.
    //
    save.PTOTLS[1] = f64::round((save.PSTOP[1] - save.PSTART[1]));

    for I in 2..=NPARTS {
        save.PTOTLS[I] = f64::round(((save.PTOTLS[(I - 1)] + save.PSTOP[I]) - save.PSTART[I]));
    }

    //
    // The partition corresponding to the input ticks is the first one
    // whose tick total is greater than the input value.  The one
    // exception is when the input ticks is equal to the total number
    // of ticks represented by all the partitions.  In this case the
    // partition number is the last one, i.e. NPARTS.
    //
    // Error if TICKS comes before the first partition (that is, if it's
    // negative), or after the last one.
    //
    if (TICKS == save.PTOTLS[NPARTS]) {
        PART = NPARTS;
    } else {
        PART = (LSTLED(TICKS, NPARTS, save.PTOTLS.as_slice()) + 1);
    }

    if ((TICKS < 0.0) || (PART > NPARTS)) {
        SETMSG(
            b"Value for ticks, #, does not fall in any partition for spacecraft #.",
            ctx,
        );
        ERRDP(b"#", TICKS, ctx);
        ERRINT(b"#", SC, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"SCDECD", ctx)?;
        return Ok(());
    }

    //
    // To get the count in this partition, subtract off the total of
    // the preceding partition counts and add the beginning count for
    // this partition.
    //
    if (PART == 1) {
        TICKS = (TICKS + save.PSTART[PART]);
    } else {
        TICKS = ((TICKS + save.PSTART[PART]) - save.PTOTLS[(PART - 1)]);
    }

    //
    // Now create the output SCLK clock string.
    //
    // First convert from ticks to clock string format.
    //
    SCFMT(SC, TICKS, SCLKCH, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SCDECD", ctx)?;
        return Ok(());
    }

    //
    // Now convert the partition number to a character string and prefix
    // it to the output string.
    //
    INTSTR(PART, &mut PRTSTR, ctx);

    SUFFIX(b"/", 0, &mut PRTSTR);

    PRELEN = LASTNB(&PRTSTR);
    SUFLEN = LASTNB(SCLKCH);

    if ((intrinsics::LEN(SCLKCH) - SUFLEN) < PRELEN) {
        SETMSG(b"Output string too short to contain clock string. Input tick value: #, requires string of length #, but declared length is #.", ctx);
        ERRDP(b"#", SCLKDP, ctx);
        ERRINT(b"#", (PRELEN + SUFLEN), ctx);
        ERRINT(b"#", intrinsics::LEN(SCLKCH), ctx);
        SIGERR(b"SPICE(SCLKTRUNCATED)", ctx)?;
        CHKOUT(b"SCDECD", ctx)?;
        return Ok(());
    }

    PREFIX(&PRTSTR, 0, SCLKCH);

    CHKOUT(b"SCDECD", ctx)?;
    Ok(())
}
