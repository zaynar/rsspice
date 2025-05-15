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

/// Encode spacecraft clock
///
/// Encode a character representation of spacecraft clock time into a
/// double precision number.
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
///  SCLKCH     I   Character representation of a spacecraft clock.
///  SCLKDP     O   Encoded representation of the clock count.
///  MXPART     P   Maximum number of spacecraft clock partitions.
/// ```
///
/// # Detailed Input
///
/// ```text
///  SC       is the standard NAIF ID of the spacecraft whose clock's
///           time is being encoded.
///
///  SCLKCH   is the character representation of some spacecraft's
///           clock count.
///
///           SCLKCH will have the following general format:
///
///              'pp/sclk_string', or just
///                 'sclk_string'
///
///           'pp' is an integer greater than or equal to one
///           and is called the partition number.
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
///           file, which needs to be loaded into the kernel pool,
///           using the routine FURNSH.
///
///           The routine SCPART is used to read the partition
///           start and stop times, in encoded units of SCLK (called
///           "ticks" -- see SCLKDP below) from the kernel file.
///
///           If the partition number is included, it must be
///           separated from the rest of the string by a '/'.
///           Any number of spaces may separate the partition number,
///           the '/', and the rest of the clock string.
///
///           If the partition number is omitted, a default partition
///           will be assumed. The default partition is the lowest-
///           numbered partition that contains the given clock time.
///           If the clock time does not fall in any of the
///           partition boundaries then an error is signaled.
///
///           'sclk_string' is a spacecraft specific clock string.
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
///           SCLK components may be separated by any of these
///           five characters: ' '  ':'  ','  '-'  '.'
///           Any number of spaces can separate the components and
///           the delimiters. The presence of the RIM component
///           is required. Successive components may be omitted, and
///           in such cases are assumed to represent zero values.
///
///           Values for the individual components may exceed the
///           maximum expected values. For instance, '0:0:0:9' is
///           an acceptable Galileo clock string, and will convert
///           to the same number of ticks as '0:0:1:1'.
///
///           Consecutive delimiters containing no intervening digits
///           are treated as if they delimit zero components.
///
///           Trailing zeros should always be included to match the
///           length of the counter. For example, a Galileo clock
///           count of '25684.90' should not be represented as
///           '25684.9'.
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
/// # Detailed Output
///
/// ```text
///  SCLKDP   is the double precision encoding of SCLKCH.
///
///           The encoding is such that order and proximity will be
///           preserved. That is, if t1, t2, and t3 are spacecraft
///           clock times, and t1*, t2*, and t3* are their encodings,
///           then if
///
///              t1 < t2 < t3, and
///
///           t2 is closer to t1 than to t3, you will have the result
///           that
///
///              t1* < t2* < t3*, and
///
///           t2* is closer to t1* than to t3*.
///
///           The units of encoded SCLK are "ticks since the start of
///           the mission", where a "tick" is defined to be the
///           shortest time increment expressible by a particular
///           spacecraft's clock.
///
///           Each clock string without partition number represents
///           a certain number of ticks, but you need to include
///           partition information to determine the relative
///           position of that time in relation to the start of the
///           mission.
///
///           Since the end time of one partition is coincident
///           with the begin time of the next, there are two
///           different representations for this instant, and they
///           will both yield the same encoding.
///
///           For example, if partition 1 has an end time of t1, and
///           partition 2 has a begin time of t2, then if we executed
///           the code fragment
///
///              CALL SCENCD ( '1/t1', SC, X )
///              CALL SCENCD ( '2/t2', SC, Y )
///
///           we would obtain X = Y.
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
///  1)  If the number of partitions in the kernel file for spacecraft
///      SC exceeds the parameter MXPART, the error
///      SPICE(TOOMANYPARTS) is signaled.
///
///  2)  If any of the extracted clock components cannot be parsed as
///      integers, or the string has too many components, or the value
///      of one of the components is less than the offset value,
///      an error is signaled by a routine in the call tree of this
///      routine.
///
///  If a partition number is included in the SCLK string, the
///  following exceptions may occur:
///
///  3)  If the partition number cannot be parsed as an integer, the
///      error SPICE(BADPARTNUMBER) is signaled.
///
///  4)  If the partition number is not in the range of the number of
///      partitions found in the kernel pool, the error
///      SPICE(BADPARTNUMBER) is signaled.
///
///  5)  If the clock count does not fall in the boundaries of the
///      specified partition, the error SPICE(NOTINPART) is
///      signaled.
///
///  If a partition number is not included in the SCLK string, the
///  following exception may occur.
///
///  6)  If the clock count does not fall in the boundaries of any
///      partition found in the kernel pool, the error
///      SPICE(NOPARTITION) is signaled.
///
///  7)  If the partition delimiter (slash) is first found in the last
///      position of SCLKCH, the error SPICE(INVALIDSCLKSTRING) is
///      signaled.
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
///  single comparison. This routine provides a method of assigning a
///  single double precision number to a spacecraft's clock count,
///  given one of its character representations.
///
///  The routine SCDECD performs the inverse operation of SCENCD,
///  converting an encoded double precision number to character format.
///
///  To convert the string to ticks since the start of the mission,
///  SCENCD
///
///     1) Converts the non-partition portion of the string to
///        ticks, using the routine SCTIKS.
///
///     2) Determines the partition number for the clock time,
///        either by getting it directly from the input string, or
///        determining the default partition if none was specified.
///
///     3) Includes partition start and stop times, which are also
///        measured in ticks, to compute the number of ticks
///        from the beginning of the mission to the clock time.
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
///           PROGRAM SCENCD_EX1
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
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  J.M. Lynch         (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  R.E. Thurman       (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.3.0, 21-NOV-2021 (NJB) (JDR)
///
///         Now variables PSTART, PSTOP, and PTOTLS are saved. Made minor
///         changes to formatting of code.
///
///         Bug fix: out-of-range character positions of SCLKCH are
///         no longer referenced.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example based on existing example fragments using PDS
///         archived CASSINI data.
///
///         Removed wrong reference to routine CLPOOL from SCLKCH detailed
///         description and $Files section, and reference to nonexistent
///         TIKSnn routines.
///
///         Removed unnecessary $Revisions section.
///
/// -    SPICELIB Version 1.2.0, 28-FEB-2014 (BVS)
///
///         Added FAILED checks to prevent passing uninitialized values to
///         ANINT, which can causing numeric exceptions on some
///         environments.
///
/// -    SPICELIB Version 1.1.0, 05-FEB-2008 (NJB)
///
///         The values of the parameter MXPART is now
///         provided by the INCLUDE file sclk.inc.
///
/// -    SPICELIB Version 1.0.2, 22-AUG-2006 (EDW)
///
///         Replaced references to LDPOOL with references
///         to FURNSH.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 03-SEP-1990 (JML) (RET)
/// ```
pub fn scencd(
    ctx: &mut SpiceContext,
    sc: i32,
    sclkch: &str,
    sclkdp: &mut f64,
) -> crate::Result<()> {
    SCENCD(sc, sclkch.as_bytes(), sclkdp, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SCENCD ( Encode spacecraft clock )
pub fn SCENCD(
    SC: i32,
    SCLKCH: &[u8],
    SCLKDP: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut POS: i32 = 0;
    let mut NPARTS: i32 = 0;
    let mut PNTER: i32 = 0;
    let mut PART: i32 = 0;
    let mut TICKS: f64 = 0.0;
    let mut ERROR = [b' '; 25 as usize];

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

    CHKIN(b"SCENCD", ctx)?;

    //
    // Read the partition start and stop times (in ticks) for this
    // mission. Error if there are too many of them.
    //
    SCPART(
        SC,
        &mut NPARTS,
        save.PSTART.as_slice_mut(),
        save.PSTOP.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"SCENCD", ctx)?;
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
        CHKOUT(b"SCENCD", ctx)?;
        return Ok(());
    }

    //
    // Convert the non-partition portion of the clock string to ticks.
    //
    POS = CPOS(SCLKCH, b"/", 1);

    if (POS == 1) {
        SETMSG(
            b"Unable to parse the partition number from SCLK string #.",
            ctx,
        );
        ERRCH(b"#", SCLKCH, ctx);
        SIGERR(b"SPICE(BADPARTNUMBER)", ctx)?;
        CHKOUT(b"SCENCD", ctx)?;
        return Ok(());
    }

    if (POS == intrinsics::LEN(SCLKCH)) {
        SETMSG(b"No SCLK components follow slash in SCLK string #.", ctx);
        ERRCH(b"#", SCLKCH, ctx);
        SIGERR(b"SPICE(INVALIDSCLKSTRING)", ctx)?;
        CHKOUT(b"SCENCD", ctx)?;
        return Ok(());
    }

    SCTIKS(SC, fstr::substr(SCLKCH, (POS + 1)..), &mut TICKS, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SCENCD", ctx)?;
        return Ok(());
    }

    TICKS = f64::round(TICKS);

    //
    // PSTART and PSTOP represent integers but are read from the
    // kernel pool as double precision numbers. Make them whole
    // numbers so that logical tests may be performed with them.
    //
    for I in 1..=NPARTS {
        save.PSTOP[I] = f64::round(save.PSTOP[I]);
        save.PSTART[I] = f64::round(save.PSTART[I]);
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
    // Determine the partition number for the input clock string:
    //
    //    If it was included in the string make sure it's valid for
    //    this mission.
    //
    //       Error if
    //
    //       1) The partition number can't be parsed.
    //       2) The partition number is not in the range 1 to the number
    //          of partitions.
    //       3) The clock count does not fall in the boundaries of the
    //          specified partition.
    //
    //    If it wasn't included, determine the default partition for
    //    this clock count.
    //
    //       Error if
    //
    //       1) The clock count does not fall in the boundaries of any
    //          of the partitions.
    //
    //
    if (POS > 1) {
        PART = 0;

        NPARSI(
            fstr::substr(SCLKCH, 1..=(POS - 1)),
            &mut PART,
            &mut ERROR,
            &mut PNTER,
            ctx,
        );

        if fstr::ne(&ERROR, b" ") {
            SETMSG(
                b"Unable to parse the partition number from SCLK string #.",
                ctx,
            );
            ERRCH(b"#", SCLKCH, ctx);
            SIGERR(b"SPICE(BADPARTNUMBER)", ctx)?;
            CHKOUT(b"SCENCD", ctx)?;
            return Ok(());
        } else if ((PART <= 0) || (PART > NPARTS)) {
            SETMSG(
                b"Partition number # taken from SCLK string # is not in acceptable range 1 to #.",
                ctx,
            );
            ERRINT(b"#", PART, ctx);
            ERRCH(b"#", SCLKCH, ctx);
            ERRINT(b"#", NPARTS, ctx);
            SIGERR(b"SPICE(BADPARTNUMBER)", ctx)?;
            CHKOUT(b"SCENCD", ctx)?;
            return Ok(());
        } else if ((TICKS < save.PSTART[PART]) || (TICKS > save.PSTOP[PART])) {
            SETMSG(
                b"SCLK count # does not fall in the boundaries of partition number #.",
                ctx,
            );
            ERRCH(b"#", SCLKCH, ctx);
            ERRINT(b"#", PART, ctx);
            SIGERR(b"SPICE(NOTINPART)", ctx)?;
            CHKOUT(b"SCENCD", ctx)?;
            return Ok(());
        }
    } else {
        PART = 1;

        while ((PART <= NPARTS) && ((TICKS < save.PSTART[PART]) || (TICKS > save.PSTOP[PART]))) {
            PART = (PART + 1);
        }

        if (PART > NPARTS) {
            SETMSG(b"SCLK count # does not fall in the boundaries of any of the partitions for spacecraft #.", ctx);
            ERRCH(b"#", SCLKCH, ctx);
            ERRINT(b"#", SC, ctx);
            SIGERR(b"SPICE(NOPARTITION)", ctx)?;
            CHKOUT(b"SCENCD", ctx)?;
            return Ok(());
        }
    }

    //
    // Now we have a valid partition number, and the number of ticks for
    // the clock string. To convert to ticks since the start of the
    // mission, add in the total number of ticks in preceding partitions
    // and subtract off the starting ticks value for this partition.
    //
    if (PART > 1) {
        *SCLKDP = ((TICKS - save.PSTART[PART]) + save.PTOTLS[(PART - 1)]);
    } else {
        *SCLKDP = (TICKS - save.PSTART[PART]);
    }

    CHKOUT(b"SCENCD", ctx)?;
    Ok(())
}
