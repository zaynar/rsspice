//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const IMPLE: i32 = 0;
const IMPCLS: i32 = 1;
const EXPLT: i32 = 2;
const EXPLE: i32 = 3;
const EXPCLS: i32 = 4;
const MNIDXT: i32 = 0;
const MXIDXT: i32 = 4;
const CONBAS: i32 = 1;
const NCON: i32 = (CONBAS + 1);
const RDRBAS: i32 = (NCON + 1);
const NRDR: i32 = (RDRBAS + 1);
const RDRTYP: i32 = (NRDR + 1);
const REFBAS: i32 = (RDRTYP + 1);
const NREF: i32 = (REFBAS + 1);
const PDRBAS: i32 = (NREF + 1);
const NPDR: i32 = (PDRBAS + 1);
const PDRTYP: i32 = (NPDR + 1);
const PKTBAS: i32 = (PDRTYP + 1);
const NPKT: i32 = (PKTBAS + 1);
const RSVBAS: i32 = (NPKT + 1);
const NRSV: i32 = (RSVBAS + 1);
const PKTSZ: i32 = (NRSV + 1);
const PKTOFF: i32 = (PKTSZ + 1);
const NMETA: i32 = (PKTOFF + 1);
const MXMETA: i32 = NMETA;
const MNMETA: i32 = 15;

/// Generic Segments: Fetch constants
///
/// Fetch from the constants partition of a generic segment the
/// double precision numbers from FIRST to LAST. The segment is
/// identified by a DAF file handle and segment descriptor.
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of a DAF open for reading.
///  DESCR      I   Descriptor for a generic segment in the DAF.
///  FIRST      I   The index of the first constant value to fetch.
///  LAST       I   The index of the last constant value to fetch.
///  VALUES     O   The constant values that were requested.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a DAF opened for reading that
///           contains the segment described by DESCR.
///
///  DESCR    is the descriptor of the segment with the desired
///           constant values. This must be the descriptor for a
///           generic segment in the DAF associated with HANDLE.
///
///  FIRST    is the index of the first value to fetch from the
///           constants section of the generic segment associated
///           with HANDLE and DESCR.
///
///  LAST     is the index of the last value to fetch from the
///           constants section of the generic segment associated
///           with HANDLE and DESCR.
/// ```
///
/// # Detailed Output
///
/// ```text
///  VALUES   is the array of constant values obtained from the
///           constants section of the generic segment associated
///           with HANDLE and DESCR.
/// ```
///
/// # Parameters
///
/// ```text
///  This subroutine makes use of parameters defined in the file
///  'sgparam.inc'.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If FIRST is less than 1 or LAST is greater than the number of
///      constants, the error SPICE(REQUESTOUTOFBOUNDS) is signaled.
///
///  2)  If LAST is less than FIRST, the error SPICE(REQUESTOUTOFORDER)
///      is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See the description of HANDLE above.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine allows easy access to values from the constants
///  partition of a generic segment in a DAF file. Please see the DAF
///  Required Reading or the include file 'sgparam.inc' for a more
///  detailed description of a generic segment.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that you have located a DAF generic segment. The
///  fragment of code below shows how to fetch all of the
///  constants from that segment.
///
///     Declarations:
///
///     DOUBLE PRECISION      CONSTS(<enough room to hold constants>)
///
///     INTEGER               MYNCON
///
///     Get the number of items in the constants section.
///
///     CALL SGMETA ( HANDLE, DESCR, NCON, MYNCON )
///
///     Fetch the constants from the segment.
///
///     CALL SGFCON ( HANDLE, DESCR, 1, MYNCON, CONSTS )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The segment described by DESCR must be a generic segment,
///      otherwise the results of this routine are not predictable.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.1, 26-OCT-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.1.0, 07-SEP-2001 (EDW)
///
///         Replaced DAFRDA call with DAFGDA.
///
/// -    SPICELIB Version 1.0.0, 11-APR-1995 (KRG) (WLT)
/// ```
pub fn sgfcon(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64],
    first: i32,
    last: i32,
    values: &mut [f64],
) -> crate::Result<()> {
    SGFCON(handle, descr, first, last, values, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SGFCON ( Generic Segments: Fetch constants )
pub fn SGFCON(
    HANDLE: i32,
    DESCR: &[f64],
    FIRST: i32,
    LAST: i32,
    VALUES: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DESCR = DummyArray::new(DESCR, 1..);
    let mut VALUES = DummyArrayMut::new(VALUES, 1..);
    let mut B: i32 = 0;
    let mut BASE: i32 = 0;
    let mut E: i32 = 0;
    let mut MYNCON: i32 = 0;

    //
    // SPICELIB Functions
    //

    //
    // Local Parameters
    //
    // Include the mnemonic values for the generic segment declarations.
    //

    //
    // Local Variables
    //
    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"SGFCON", ctx)?;

    //
    // Get the value for the base of the constants and the number of
    // constants in the generic segment.
    //
    SGMETA(HANDLE, DESCR.as_slice(), CONBAS, &mut BASE, ctx)?;
    SGMETA(HANDLE, DESCR.as_slice(), NCON, &mut MYNCON, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SGFCON", ctx)?;
        return Ok(());
    }

    //
    // Perform checks on the inputs for reasonableness.
    //
    if ((FIRST < 1) || (LAST > MYNCON)) {
        SETMSG(b"The range of constants requested extends beyond the available constant data.  Constants are available for indices 1 to #.  You have requested data from # to #. ", ctx);
        ERRINT(b"#", MYNCON, ctx);
        ERRINT(b"#", FIRST, ctx);
        ERRINT(b"#", LAST, ctx);
        SIGERR(b"SPICE(REQUESTOUTOFBOUNDS)", ctx)?;
        CHKOUT(b"SGFCON", ctx)?;
        return Ok(());
    }

    if (LAST < FIRST) {
        SETMSG(
            b"The last constant item requested, #, is before the first constant item requested, #.",
            ctx,
        );
        ERRINT(b"#", LAST, ctx);
        ERRINT(b"#", FIRST, ctx);
        SIGERR(b"SPICE(REQUESTOUTOFORDER)", ctx)?;
        CHKOUT(b"SGFCON", ctx)?;
        return Ok(());
    }

    //
    // Compute the addresses of the data within the file and then fetch
    // the data.
    //
    B = (BASE + FIRST);
    E = (BASE + LAST);

    DAFGDA(HANDLE, B, E, VALUES.as_slice_mut(), ctx)?;

    CHKOUT(b"SGFCON", ctx)?;
    Ok(())
}
