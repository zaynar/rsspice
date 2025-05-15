//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// ET to continuous SCLK ticks
///
/// Convert ephemeris seconds past J2000 (ET) to continuous encoded
/// spacecraft clock (`ticks').  Non-integral tick values may be
/// returned.
///
/// # Required Reading
///
/// * [CK](crate::required_reading::ck)
/// * [SCLK](crate::required_reading::sclk)
/// * [SPK](crate::required_reading::spk)
/// * [TIME](crate::required_reading::time)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  SC         I   NAIF spacecraft ID code.
///  ET         I   Ephemeris time, seconds past J2000.
///  SCLKDP     O   SCLK, encoded as ticks since spacecraft clock
///                 start. SCLKDP need not be integral.
/// ```
///
/// # Detailed Input
///
/// ```text
///  SC       is a NAIF integer code for a spacecraft whose
///           encoded SCLK value at the epoch specified by ET is
///           desired.
///
///  ET       is an epoch, specified as ephemeris seconds past
///           J2000.
/// ```
///
/// # Detailed Output
///
/// ```text
///  SCLKDP   is an encoded spacecraft clock value. SCLKDP is
///           an encoded representation of the total number
///           of spacecraft clock ticks measured from the time
///           the spacecraft clock started to the epoch ET:
///           partition information IS reflected in the encoded
///           value.
///
///           SCLKDP may be non-integral:  SCLKDP is NOT
///           rounded to the nearest whole tick.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If an SCLK kernel has not been loaded, does not contain all of
///      the required data, or contains invalid data, an error is
///      signaled by a routine in the call tree of this routine. The
///      output argument SCLKDP will not be modified. This routine
///      assumes that that an SCLK kernel appropriate to the spacecraft
///      clock identified by the input argument SC has been loaded.
///
///  2)  If a leapseconds kernel is required for conversion between
///      SCLK and ET but is not loaded, an error is signaled by a
///      routine in the call tree of this routine. The output argument
///      SCLKDP will not be modified. When using SCLK kernels that map
///      SCLK to a time system other than ET (also called barycentric
///      dynamical time---`TDB'), it is necessary to have a leapseconds
///      kernel loaded at the time this routine is called.
///
///      The time system that an SCLK kernel maps SCLK to is indicated
///      by the variable SCLK_TIME_SYSTEM_nn in the kernel, where nn
///      is the negative of the NAIF integer code for the spacecraft.
///      The time system used in a kernel is TDB if and only if the
///      variable is assigned the value 1.
///
///  3)  If the clock type for the spacecraft clock identified by
///      SC is not supported by this routine, the error
///      SPICE(NOTSUPPORTED) is signaled. The output argument SCLKDP
///      will not be modified.
///
///  4)  If the input ET value is not representable as an encoded
///      spacecraft clock value for the spacecraft clock identified by
///      SC, an error is signaled by a routine in the call tree of this
///      routine. The output argument SCLKDP will not be modified.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine outputs continuous encoded SCLK values; unlike the
///  routine SCE2T, the values output by this routine need not be
///  integral.
///
///  This routine supports use of non-integral encoded clock values in
///  C-kernels:  non-integral clock values may be stored as pointing
///  time tags when a C-kernel is created, and they may be supplied
///  as request times to the C-kernel readers.
///
///  The advantage of encoded SCLK, as opposed to character string
///  representations of SCLK, is that encoded SCLK values are easy to
///  perform arithmetic operations on. Also, working with encoded SCLK
///  reduces the overhead of repeated conversion of  character strings
///  to integers or double precision numbers.
///
///  To convert ET to a string representation of an SCLK value, use
///  the SPICELIB routine SCE2S.
///
///  See the SCLK Required Reading for a list of the entire set of
///  SCLK conversion routines.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Convert ET directly to an encoded SCLK value; use both of
///      these time values to look up both C-kernel (pointing) and
///      SPK (position and velocity) data for an epoch specified by an
///      ephemeris time.
///
///         During program initialization, load the leapseconds and
///         SCLK kernels. We will pretend that these files are named
///         "LEAPSECONDS.KER" and "GLLSCLK.KER".  To use this code
///         fragment, you must substitute the actual names of these
///         kernel files for the names used here.
///
///            C
///            C     Load leapseconds and SCLK kernels:
///            C
///                  CALL FURNSH ( 'LEAPSECONDS.KER' )
///                  CALL FURNSH ( 'GLLSCLK.KER'     )
///
///         The mission is Galileo, which has spacecraft ID -77.
///         Let ET be the epoch, specified in ephemeris seconds
///         past J2000, at which both position and pointing data
///         are desired.
///
///         Find the continuous encoded SCLK value corresponding to ET.
///
///                  CALL SCE2C  ( -77,  ET,  SCLKDP )
///
///         Now you're ready to call both CKGP, which expects the input
///         epoch to be specified by an encoded SCLK string, and
///         SPKEZ, which expects the epoch to be specified as an
///         ephemeris time.
///
///            C
///            C     Find scan platform pointing CMAT and s/c--target
///            C     vector (first 3 components of STATE) at epoch.
///            C     We assume that CK and SPK kernels have been loaded
///            C     already, via CKLPF and SPKLEF respectively.
///            C
///                  CALL CKGP  ( SCANPL,
///                 .             SCLKDP,
///                 .             TOL,
///                 .             REFSYS,
///                 .             CMAT,
///                 .             CLKOUT,
///                 .             FOUND   )
///
///                  CALL SPKEZ ( TARGET,
///                 .             ET,
///                 .             REFSYS,
///                 .             CORR,
///                 .             -77,
///                 .             STATE,
///                 .             LT      )
///
///
///  2)  Convert UTC to a continuous encoded Voyager 2 SCLK value.
///
///         Again, your initialization code must load the leapseconds
///         and SCLK kernels.
///
///            C
///            C     Load leapseconds and SCLK kernels:
///            C
///                  CALL FURNSH ( 'LEAPSECONDS.KER' )
///                  CALL FURNSH ( 'VGR2SCLK.KER'    )
///
///
///         To find the encoded Voyager 2 SCLK value SCLKDP
///         corresponding to a UTC time, you can use the code fragment
///
///                  CALL UTC2ET ( UTC,  ET          )
///                  CALL SCE2C  ( -32,  ET,  SCLKDP )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  An SCLK kernel appropriate to the spacecraft clock identified
///      by SC must be loaded at the time this routine is called.
///
///  2)  If the SCLK kernel used with this routine does not map SCLK
///      directly to barycentric dynamical time, a leapseconds kernel
///      must be loaded at the time this routine is called.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 13-APR-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Moved the
///         required readings present in $Literature_References section to
///         $Required_Reading.
///
/// -    SPICELIB Version 1.0.1, 22-AUG-2006 (EDW)
///
///         Replaced references to LDPOOL with references
///         to FURNSH.
///
/// -    SPICELIB Version 1.0.0, 09-MAR-1999 (NJB)
/// ```
pub fn sce2c(ctx: &mut SpiceContext, sc: i32, et: f64, sclkdp: &mut f64) -> crate::Result<()> {
    SCE2C(sc, et, sclkdp, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SCE2C ( ET to continuous SCLK ticks )
pub fn SCE2C(SC: i32, ET: f64, SCLKDP: &mut f64, ctx: &mut Context) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"SCE2C", ctx)?;
    }

    //
    // Just hand off the conversion to the appropriate routine.
    //
    if (SCTYPE(SC, ctx)? == 1) {
        SCEC01(SC, ET, SCLKDP, ctx)?;
    } else {
        SETMSG(b"Clock type # is not supported.", ctx);
        ERRINT(b"#", SCTYPE(SC, ctx)?, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"SCE2C", ctx)?;
        return Ok(());
    }

    CHKOUT(b"SCE2C", ctx)?;
    Ok(())
}
