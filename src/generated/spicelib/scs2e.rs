//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// SCLK string to ET
///
/// Convert a spacecraft clock string to ephemeris seconds past
/// J2000 (ET).
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
///  SC         I   NAIF integer code for a spacecraft.
///  SCLKCH     I   An SCLK string.
///  ET         O   Ephemeris time, seconds past J2000.
/// ```
///
/// # Detailed Input
///
/// ```text
///  SC       is a NAIF ID code for a spacecraft, one of whose
///           clock values is represented by SCLKCH. The set of
///           supported spacecraft clocks is listed in the SCLK
///           Required Reading.
///
///  SCLKCH   is a character string representation of the
///           spacecraft clock value that corresponds to ET, for
///           the spacecraft specified by the input argument SC.
///           SCLKCH is an absolute spacecraft clock time, so
///           partition information should be included in this
///           string. The precise format of SCLKCH is specified
///           in the SCLK Required Reading.
/// ```
///
/// # Detailed Output
///
/// ```text
///  ET       is the epoch, specified as ephemeris seconds past
///           J2000, that corresponds to SCLKCH.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If an SCLK kernel has not been loaded, does not contain all of
///      the required data, or contains invalid data, an error is
///      signaled by a routine in the call tree of this routine. The
///      output argument ET will not be modified. This routine assumes
///      that that an SCLK kernel appropriate to the spacecraft clock
///      identified by the input argument SC has been loaded.
///
///  2)  If a leapseconds kernel is required for conversion between
///      SCLK and ET but is not loaded, an error is signaled by a
///      routine in the call tree of this routine. The output argument
///      ET will not be modified. When using SCLK kernels that map SCLK
///      to a time system other than ET (also called barycentric
///      dynamical time---`TDB'), it is necessary to have a leapseconds
///      kernel loaded at the time this routine is called.
///
///      The time system that an SCLK kernel maps SCLK to is indicated
///      by the variable SCLK_TIME_SYSTEM_nn in the kernel, where nn
///      is the negative of the NAIF integer code for the spacecraft.
///      The time system used in a kernel is TDB if and only if the
///      variable is assigned the value 1.
///
///  3)  If the value of SCLKCH is invalid, an error is signaled by a
///      routine in the call tree of this routine. The output argument
///      ET will not be modified.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is provided as a convenience; it is simply shorthand
///  for the code fragment
///
///     CALL SCENCD ( SC, SCLKCH, SCLKDP )
///     CALL SCT2E  ( SC, SCLKDP, ET     )
///
///  See the SCLK Required Reading for a list of the entire set of
///  SCLK conversion routines.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Find the state (position and velocity) of Jupiter, as seen
///      from the Galileo spacecraft, at the epoch corresponding to
///      the SCLK value
///
///         2 / 3110578:89:09
///
///      The digit `2', followed by the forward slash, indicates that
///      the time value is in the second mission partition.
///
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
///            C
///            C     Load an SPK file (again, a fictitious file)
///            C     containing an ephemeris for Jupiter and the
///            C     GLL orbiter's trajectory.
///            C
///                  CALL SPKLEF ( 'GLLSPK.KER', HANDLE )
///
///         The Galileo spacecraft ID is -77.  Convert our SCLK
///         string to ephemeris seconds past J2000, which is the
///         time representation expected by SPKEZ.
///
///                  CALL SCS2E ( -77, '2 / 3110578:89:09', ET )
///
///
///         Find the state of Jupiter (body 599) as seen from Galileo
///         at time ET.  To use SPKEZ, you must first load an SPK
///         kernel, using the routine SPKLEF.
///
///                  CALL SPKEZ ( 599,
///                 .             ET,
///                 .             REFSYS,
///                 .             CORR,
///                 .             -77,
///                 .             STATE,
///                 .             LT      )
///
///
///
///  2)  Convert a Voyager 2 SCLK value to UTC, using calendar format,
///      with 3 digits of precision in the seconds component.
///
///         Again, your initialization code must load the leapseconds
///         and SCLK kernels:
///
///            C
///            C     Load leapseconds and SCLK kernels:
///            C
///                  CALL FURNSH ( 'LEAPSECONDS.KER' )
///                  CALL FURNSH ( 'VGR2SCLK.KER'    )
///
///
///         To find the UTC value corresponding to Voyager 2 SCLK
///         string
///
///                  11389.20.768
///
///         you can use the code fragment
///
///                  CALL SCS2E  ( -32,  '11389.29.768',  ET  )
///                  CALL ET2UTC (  ET,  'C',      3,     UTC )
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
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section.
///
///         Moved the required readings present in $Literature_References
///         section to $Required_Reading and added CK to the list.
///
/// -    SPICELIB Version 1.0.4, 22-AUG-2006 (EDW)
///
///         Replaced references to LDPOOL with references
///         to FURNSH.
///
/// -    SPICELIB Version 1.0.3, 09-MAR-1999 (NJB)
///
///         Explicit list of SCLK conversion routines in $Particulars
///         section has been replaced by a pointer to the SCLK Required
///         Reading.
///
/// -    SPICELIB Version 1.0.2, 10-APR-1992 (NJB) (WLT)
///
///         The $Brief_I/O section now lists ET correctly as an output
///         from this routine. Header was updated to reflect possibility
///         of needing to load a leapseconds kernel before calling this
///         routine. Comment section for permuted index source lines was
///         added following the header.
///
/// -    SPICELIB Version 1.0.1, 12-OCT-1990 (NJB)
///
///         $Restrictions section no longer states that you must load the
///         leapseconds kernel prior to calling this routine.
///
///         The examples have been slightly re-written. In particular,
///         they no longer use calls to CLPOOL.
///
/// -    SPICELIB Version 1.0.0, 03-SEP-1990 (NJB)
/// ```
pub fn scs2e(ctx: &mut SpiceContext, sc: i32, sclkch: &str, et: &mut f64) -> crate::Result<()> {
    SCS2E(sc, sclkch.as_bytes(), et, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SCS2E ( SCLK string to ET )
pub fn SCS2E(SC: i32, SCLKCH: &[u8], ET: &mut f64, ctx: &mut Context) -> f2rust_std::Result<()> {
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
        CHKIN(b"SCS2E", ctx)?;
    }

    //
    // Encode SCLKCH, and convert the result to ET.
    //
    SCENCD(SC, SCLKCH, &mut SCLKDP, ctx)?;
    SCT2E(SC, SCLKDP, ET, ctx)?;

    CHKOUT(b"SCS2E", ctx)?;
    Ok(())
}
