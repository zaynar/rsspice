//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Convert spacecraft clock string to ticks.
///
/// Convert a spacecraft clock format string to number of "ticks".
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
///  CLKSTR     I   Character representation of a spacecraft clock.
///  TICKS      O   Number of ticks represented by the clock string.
/// ```
///
/// # Detailed Input
///
/// ```text
///  SC       is the NAIF ID number for the spacecraft whose clock
///           string is being converted.
///
///  CLKSTR   is a character string representing a spacecraft clock
///           time, WITHOUT PARTITION NUMBER.
///
///           Using Galileo as an example, the full format is
///
///                          wwwwwwww:xx:y:z
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
///           SCLK components may be separated by any of the
///           following characters: ' '  '.'  ':'  ','  '-'
///           Any number of spaces may separate the components and
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
///           length of the counter.  For example, a Galileo clock
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
///  TICKS    is the number of ticks represented by the spacecraft
///           clock string. A tick is defined to be the smallest
///           time increment expressible by the spacecraft clock.
///
///           An analogy may be drawn between a spacecraft clock
///           and a standard wall clock, measuring hours, minutes
///           and seconds. The number of ticks represented by the
///           wall clock string
///                                hh:mm:ss
///
///           would be the number of seconds represented by that
///           time.
///
///           For example:
///
///                    00:00:10  would convert to 10
///                    00:01:00  would convert to 60
///                    00:10:00  would convert to 600
///                    01:00:00  would convert to 3600
///                    01:01:00  would convert to 3660
///
///           See the $Examples section below for examples for
///           actual spacecraft clocks.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the spacecraft clock type is not supported, the
///      error SPICE(NOTSUPPORTED) is signaled.
///
///  2)  If any of the extracted clock components cannot be parsed as
///      integers, or the string has too many components, or the value
///      of one of the components is less than the offset value, then,
///      an error is signaled by a routine in the call tree of this
///      routine.
///
///  3)  Invalid spacecraft ID's are not diagnosed.
/// ```
///
/// # Particulars
///
/// ```text
///  Each spacecraft is assigned a clock type code in the kernel file.
///  SCTIKS calls the function SCTYPE to determine this value. If the
///  clock type is supported by SPICE, then the SPICELIB routine TIKSnn
///  is called to handle the actual conversion from clock format to
///  number of ticks. The nn in TIKSnn refers to the spacecraft clock
///  type code. Different spacecraft have distinct clock formats but
///  can still be of the same clock type.
///
///  The TIKSnn routines are entry points to the routines SCLKnn, which
///  also contain the ticks-to-clock format conversion routines FMTnn.
///  FMTnn is called by the subroutine SCFMT, which performs the
///  inverse operation to SCTIKS.
///
///  Note the important difference between SCENCD and SCTIKS. SCENCD
///  converts a clock string to the number of ticks it represents
///  since the beginning of the mission, and so uses partition
///  information. SCTIKS just converts to absolute ticks.
/// ```
///
/// # Examples
///
/// ```text
///  SCTIKS is used as part of the process of encoding spacecraft clock
///  by SCENCD, though SCTIKS does not process any partition informa-
///  tion.
///
///  Another use of SCTIKS, however, is to convert a clock measurement
///  to ticks for use as a tolerance for the CK reader CKGP.
///
///
///  C
///  C      Get the pointing from a CK file of the VGR 1 narrow angle
///  C      image corresponding to a particular SCLK count.
///  C
///  C      Load the CK file and the kernel file containing SCLK
///  C      partition information for SCENCD.
///  C
///         CALL CKLPF  ( 'VGR1NA.CK', HANDLE )
///         CALL FURNSH ( 'SCLK.KER' )
///
///  C
///  C      Get the right ID numbers.
///  C
///         SC    = -31
///         INSTR = -31001
///
///  C
///  C      The SCLK string includes a partition number. Pictures are
///  C      never shuttered at intervals smaller than 1 MOD60 count
///  C      from each other. So use 1 MOD60 count as the time
///  C      tolerance.
///  C
///         CLKSTR = '1/20556:14:768'
///         TOLSTR = '      0:01:000'
///
///  C
///  C      Encode the clock string and the tolerance.
///  C
///         CALL SCENCD ( SC, CLKSTR, SCLK )
///         CALL SCTIKS ( SC, TOLSTR, TOL  )
///
///  C
///  C      Get the pointing from the C-kernel.
///  C
///         CALL CKGP ( INSTR, SCLK, TOL, REF, CMAT, CLKOUT, FOUND )
///
///
///
///   Below are some examples illustrating various clock string inputs
///   and the resulting outputs for the Galileo spacecraft. See the
///   SCLK required reading for a detailed description of the Galileo
///   clock format.
///
///      CLKSTR                TICKS
///      ----------------      --------------------
///      '0:0:0:1'             1
///      '0:0:1'               8
///      '0:1'                 80
///      '1'                   7280
///      '1 0 0 0'             7280
///      '1,0,0,0'             7280
///      '1:90'                14480
///      '1:9'                 8000
///      '1:09'                8000
///      '0-0-10'              80   |--  Third component is supposed
///      '0-1-0'               80   |    to be a mod-10 count.
///      '0/1/0'               Error: '/' is not an accepted delimiter.
///      '1: 00 : 0 : 1'       7281
///      '1:::1'               7281
///      '1.1.1.1.1'           Error: Too many components
///      '1.1.1.1.'            Error: The last delimiter signals that
///                                   a fifth component will follow.
///
///
///      The following examples are for the Voyager 2 spacecraft. Note
///      that the last component of the Voyager clock has an offset
///      value of 1.
///
///      CLKSTR                TICKS
///      ----------------      --------------------
///       '0.0.001'              0
///       '0:0:002'              1
///       '0:01'                 800
///       '1'                    48000
///       '1.0'                  48000
///       '1.0.0'                Error: The 3rd component is never 0.
///       '0.0:100'              99
///       '0-60-1'               48000
///       '1-1-1'                48800
///       '1-1-2'                48801
/// ```
///
/// # Author and Institution
///
/// ```text
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
/// -    SPICELIB Version 1.1.0, 01-NOV-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
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
/// -    SPICELIB Version 1.0.0, 06-SEP-1990 (JML) (RET)
/// ```
pub fn sctiks(ctx: &mut SpiceContext, sc: i32, clkstr: &str, ticks: &mut f64) -> crate::Result<()> {
    SCTIKS(sc, clkstr.as_bytes(), ticks, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SCTIKS ( Convert spacecraft clock string to ticks. )
pub fn SCTIKS(
    SC: i32,
    CLKSTR: &[u8],
    TICKS: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut TYPE: i32 = 0;

    //
    // SPICELIB functions
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
        CHKIN(b"SCTIKS", ctx)?;
    }

    //
    // If the spacecraft clock type is supported by NAIF then
    // call TIKSnn to perform the conversion.
    //

    TYPE = SCTYPE(SC, ctx)?;

    if (TYPE == 1) {
        SCTK01(SC, CLKSTR, TICKS, ctx)?;
    } else {
        SETMSG(b"Clock type # is not supported.", ctx);
        ERRINT(b"#", TYPE, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"SCTIKS", ctx)?;
        return Ok(());
    }

    CHKOUT(b"SCTIKS", ctx)?;

    Ok(())
}
