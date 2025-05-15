//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// ET to SCLK string
///
/// Convert an epoch specified as ephemeris seconds past J2000 (ET) to
/// a character string representation of a spacecraft clock value
/// (SCLK).
///
/// # Required Reading
///
/// * [SCLK](crate::required_reading::sclk)
/// * [TIME](crate::required_reading::time)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  SC         I   NAIF spacecraft clock ID code.
///  ET         I   Ephemeris time, specified as seconds past J2000.
///  SCLKCH     O   An SCLK string.
/// ```
///
/// # Detailed Input
///
/// ```text
///  SC       is a NAIF ID code for a spacecraft clock whose
///           reading at the epoch specified by ET is desired.
///
///  ET       is an epoch, specified as ephemeris seconds past
///           J2000.
/// ```
///
/// # Detailed Output
///
/// ```text
///  SCLKCH   is a character string representation of the
///           spacecraft clock value that corresponds to ET, for
///           the spacecraft clock specified by the input
///           argument SC. SCLKCH is an absolute spacecraft
///           clock value, so a partition number is included in
///           the string. The format of SCLKCH is specified in
///           the SCLK kernel for the clock SC. A general
///           discussion of spacecraft clock string formats is
///           available in the SCLK Required Reading.
///
///           In order to choose an appropriate length for
///           SCLKCH, you can examine an SCLK kernel for the
///           clock specified by SC. The format of string
///           representations of the clock's values is specified
///           by kernel variables associated with the clock. See
///           $Examples below for further information.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If an SCLK kernel has not been loaded, does not contain all of
///      the required data, or contains invalid data, an error is
///      signaled by a routine in the call tree of this routine. The
///      output argument SCLKCH will not be modified. This routine
///      assumes that an SCLK kernel appropriate to the spacecraft
///      clock identified by the input argument SC has been loaded.
///
///  2)  If a leapseconds kernel is required for conversion between
///      SCLK and ET but is not loaded, an error is signaled by a
///      routine in the call tree of this routine. The output argument
///      SCLKCH will not be modified. When using an SCLK kernel that
///      maps SCLK to a time system other than ET (also called
///      barycentric dynamical time---`TDB'), it is necessary to have a
///      leapseconds kernel loaded at the time this routine is called.
///
///      The time system to which an SCLK kernel maps SCLK epochs is
///      indicated by the variable SCLK_TIME_SYSTEM_nn in the kernel,
///      where nn is the negative of the NAIF integer code for the
///      spacecraft. The time system used in a kernel is TDB if and
///      only if the variable is assigned the value 1.
///
///  3)  If the input ET value is not representable in the spacecraft
///      clock string format for the spacecraft clock identified by SC,
///      an error is signaled by a routine in the call tree of this
///      routine. The output argument SCLKCH will not be modified.
///
///  4)  If the output argument SCLKCH is too short to contain the
///      output spacecraft clock string produced by this routine, an
///      error is signaled by a routine in the call tree of this
///      routine. The output argument SCLKCH may contain a portion of
///      the truncated string.
/// ```
///
/// # Files
///
/// ```text
///  An SCLK kernel, appropriate to the spacecraft clock identified
///  by SC, must be loaded at the time this routine is called.
///
///  If the SCLK kernel used with this routine does not map SCLK
///  directly to barycentric dynamical time, a leapseconds kernel
///  must be loaded at the time this routine is called.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is provided as a convenience; it is simply shorthand
///  for the code fragment
///
///     CALL SCE2T  ( SC,  ET,      SCLKDP )
///     CALL SCDECD ( SC,  SCLKDP,  SCLKCH )
///
///  See the SCLK Required Reading for a list of the entire set of
///  SCLK conversion routines.
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
///
///  1)  Determine the length of Galileo spacecraft clock strings.
///
///      Examine a Galileo SCLK kernel. There you'll find the
///      kernel variable assignments
///
///         SCLK01_MODULI_77          = ( 16777215 91 10 8 )
///         SCLK01_OFFSETS_77         = (        0  0  0 0 )
///
///      Each field of the clock string contains values ranging
///      from the offset value to M-1, where M is the corresponding
///      modulus. So the Galileo clock fields have maximum values
///
///         16777214 90 9 7
///
///      representing the partition number by the symbol "pp" and
///      the field delimiter character by the symbol "D", we see
///      that the GLL SCLK format is
///
///         pp/xxxxxxxxDxxDxDx
///
///      This string has length 18 characters. Accounting for the
///      terminating null character, the value of `lenout' should
///      be set to at least 19.
///
///      Note:  the delimiter character is determined by the integer
///      code assignment
///
///         SCLK01_OUTPUT_DELIM_77    = (                2 )
///
///      The SCLK Required Reading indicates that 2 is the SCLK kernel
///      code for the colon character.
///
///
///  2)  Find the Galileo SCLK value corresponding to the ET
///
///         -322452420.5593641.
///
///
///         C
///         C     Start out by loading the SCLK kernel. In your own
///         C     program, you must use the name of a real SCLK kernel.
///         C     The name shown here is fictitious.
///         C
///               CALL FURNSH ( 'GLLSCLK.KER' )
///
///         C
///         C     Load a leapseconds kernel in case it is needed for
///         C     SCLK-to-ET conversion. Depending on the SCLK kernel
///         C     used, it may not be necessary to load this file; it's
///         C     just a simple, reliable way of making sure that the
///         C     leapseconds kernel constants are available if we need
///         C     them. Again, a fictitious name is used.
///         C
///               CALL FURNSH ( 'LEAPSECONDS.KER' )
///
///         C
///         C     The spacecraft ID code for Galileo is -77.
///         C
///               SC = -77
///               ET = -322452420.5593641
///
///               CALL SCE2S ( SC, ET, SCLKCH )
///
///
///      The returned value of SCLKCH will be
///
///         1/00010001:44:2:0.
///
///
///  3)  Convert the UTC time
///
///         August 25 1989 4:00:00
///
///      to a Voyager 2 SCLK value.
///
///      To enable you to perform UTC to ET conversion, your
///      initialization code must load the leapseconds and SCLK
///      kernels:
///
///         C
///         C     Load leapseconds and SCLK kernels:
///         C
///               CALL FURNSH ( 'LEAPSECONDS.KER' )
///               CALL FURNSH ( 'VGR2SCLK.KER'    )
///
///
///      To find Voyager 2 SCLK string corresponding to the
///      specified UTC time, you can use the code fragment
///
///               CALL UTC2ET ( 'Aug 25 1989 4:00:00',  ET     )
///               CALL SCE2S  ( -32,        ET,         SCLKCH )
///
///      The result of the conversion is
///
///         SCLKCH  =  '4/11390:22:012'
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.3.0, 05-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary entries in $Revisions section.
///
/// -    SPICELIB Version 1.2.2, 29-JUL-2003 (NJB)
///
///         Various header changes were made to improve clarity and
///         more fully explain the routine's functionality.
///
/// -    SPICELIB Version 1.2.1, 09-MAR-1999 (NJB)
///
///         Explicit list of SCLK conversion routines in $Particulars
///         section has been replaced by a pointer to the SCLK Required
///         Reading.
///
/// -    SPICELIB Version 1.2.0, 10-APR-1992 (NJB) (WLT)
///
///         Truncation of the output string is now treated as an error.
///         Header was updated to reflect possibility of needing to load
///         a leapseconds kernel before calling this routine. Comment
///         section for permuted index source lines was added following the
///         header.
///
/// -    SPICELIB Version 1.0.1, 12-OCT-1990 (NJB)
///
///         Missing example added to the $Examples section.  $Restrictions
///         section no longer states that you must load the leapseconds
///         kernel prior to calling this routine.
///
///         The second example no longer uses a call to CLPOOL.
///
/// -    SPICELIB Version 1.0.0, 03-SEP-1990 (NJB)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.2.0, 10-APR-1992 (NJB) (WLT)
///
///         Truncation of the output string is now treated as an error.
///         The code changes made to implement the error checking were
///         in SCDECD and other lower-level routines.
/// ```
pub fn sce2s(ctx: &mut SpiceContext, sc: i32, et: f64, sclkch: &mut str) -> crate::Result<()> {
    SCE2S(
        sc,
        et,
        fstr::StrBytes::new(sclkch).as_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SCE2S ( ET to SCLK string )
pub fn SCE2S(SC: i32, ET: f64, SCLKCH: &mut [u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut SCLKDP: f64 = 0.0;

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
        CHKIN(b"SCE2S", ctx)?;
    }

    //
    // Convert ET to encoded SCLK, and then to an SCLK string.
    //
    SCE2T(SC, ET, &mut SCLKDP, ctx)?;
    SCDECD(SC, SCLKDP, SCLKCH, ctx)?;

    CHKOUT(b"SCE2S", ctx)?;
    Ok(())
}
