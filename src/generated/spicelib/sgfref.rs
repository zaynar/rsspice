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

/// Generic Segments: Fetch references
///
/// Fetch from the references partition of a generic segment
/// the double precision numbers from FIRST to LAST. The
/// segment is identified by a DAF file handle and segment
/// descriptor.
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
///  FIRST      I   The index of the first reference value to fetch.
///  LAST       I   The index of the last reference value to fetch.
///  VALUES     O   The reference values that were requested.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a DAF file opened for reading
///           that contains the segment described by DESCR.
///
///  DESCR    is the descriptor of the segment with the desired
///           constant values. This must be the descriptor for a
///           segment in the DAF associated with HANDLE.
///
///  FIRST    is the index of the first value to fetch from the
///           reference section of the DAF generic segment associated
///           with HANDLE and DESCR.
///
///  LAST     is the index of the last value to fetch from the
///           constants section of the DAF generic segment associated
///           with HANDLE and DESCR.
/// ```
///
/// # Detailed Output
///
/// ```text
///  VALUES   is the array of reference values obtained from the
///           reference section of the DAF generic segment
///           associated with HANDLE and DESCR.
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
///      reference values, the error SPICE(REQUESTOUTOFBOUNDS) is
///      signaled.
///
///  2)  If LAST is less than FIRST, the error SPICE(REQUESTOUTOFORDER)
///      is signaled.
///
///  3)  If the reference directory structure is unrecognized, the
///      error SPICE(UNKNOWNREFDIR) is signaled. The most likely cause
///      of this error is that an upgrade to your version of the SPICE
///      toolkit is needed.
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
///  This routine allows you to easily fetch values from the reference
///  section of a generic segment.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that you have located a DAF generic segment. The code
///  fragment below shows how to fetch the I'th reference value from
///  that segment.
///
///     Declarations:
///
///     DOUBLE PRECISION      REFVAL
///
///     Fetch the Ith reference value from the segment.
///
///     CALL SGFREF ( HANDLE, DESCR, I, I, REFVAL )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The segment described by DESCR MUST be a generic segment,
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
/// -    SPICELIB Version 1.2.1, 26-OCT-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.2.0, 07-SEP-2001 (EDW)
///
///         Replaced DAFRDA call with DAFGDA.
///
/// -    SPICELIB Version 1.0.0, 12-APR-1995 (KRG) (WLT)
/// ```
pub fn sgfref(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64],
    first: i32,
    last: i32,
    values: &mut [f64],
) -> crate::Result<()> {
    SGFREF(handle, descr, first, last, values, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SGFREF ( Generic Segments: Fetch references )
pub fn SGFREF(
    HANDLE: i32,
    DESCR: &[f64],
    FIRST: i32,
    LAST: i32,
    VALUES: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DESCR = DummyArray::new(DESCR, 1..);
    let mut VALUES = DummyArrayMut::new(VALUES, 1..);
    let mut BUFFER = StackArray::<f64, 2>::new(1..=2);
    let mut B: i32 = 0;
    let mut BASE: i32 = 0;
    let mut E: i32 = 0;
    let mut MYNREF: i32 = 0;
    let mut MYREFT: i32 = 0;

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

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"SGFREF", ctx)?;
    //
    // Perform the needed initialization
    //
    SGMETA(HANDLE, DESCR.as_slice(), REFBAS, &mut BASE, ctx)?;
    SGMETA(HANDLE, DESCR.as_slice(), RDRTYP, &mut MYREFT, ctx)?;
    SGMETA(HANDLE, DESCR.as_slice(), NREF, &mut MYNREF, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SGFREF", ctx)?;
        return Ok(());
    }

    //
    // Perform checks on the inputs for reasonableness.
    //
    if ((FIRST < 1) || (LAST > MYNREF)) {
        SETMSG(b"The range of reference items requested extends beyond the available range of reference items.  The reference data is available for indexes 1 to #.  You\'ve requested data from # to #.", ctx);
        ERRINT(b"#", MYNREF, ctx);
        ERRINT(b"#", FIRST, ctx);
        ERRINT(b"#", LAST, ctx);
        SIGERR(b"SPICE(REQUESTOUTOFBOUNDS)", ctx)?;
        CHKOUT(b"SGFREF", ctx)?;
        return Ok(());
    }

    if (LAST < FIRST) {
        SETMSG(b"The last reference item requested, #, is before the first reference item requested, #.", ctx);
        ERRINT(b"#", LAST, ctx);
        ERRINT(b"#", FIRST, ctx);
        SIGERR(b"SPICE(REQUESTOUTOFORDER)", ctx)?;
        CHKOUT(b"SGFREF", ctx)?;
        return Ok(());
    }
    //
    // Ok.  We are ready to go. If the reference type is recognized
    // fetch the requested data.
    //
    if (MYREFT == IMPLE) {
        //
        // The reference values are implied in this case. Read the
        // reference base value and step. If we fail, check out and
        // return; we don't want to try and comput anything with bogus
        // data.
        //
        B = (BASE + 1);
        E = (BASE + 2);

        DAFGDA(HANDLE, B, E, BUFFER.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"SGFREF", ctx)?;
            return Ok(());
        }
        //
        // Now simply compute the reference values using the implicit
        // model for them.
        //
        for I in FIRST..=LAST {
            VALUES[I] = (BUFFER[1] + f64::trunc((((I - 1) as f64) * BUFFER[2])));
        }
    } else if (MYREFT == IMPCLS) {
        //
        // The reference values are implied in this case. Read the
        // reference base value and step. If we fail, check out and
        // return; we don't want to try and comput anything with bogus
        // data.
        //
        B = (BASE + 1);
        E = (BASE + 2);

        DAFGDA(HANDLE, B, E, BUFFER.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"SGFREF", ctx)?;
            return Ok(());
        }
        //
        // Now simply compute the reference values using the implicit
        // model for them.
        //
        for I in FIRST..=LAST {
            VALUES[I] = (BUFFER[1] + f64::trunc((((I - 1) as f64) * BUFFER[2])));
        }
    } else if (((MYREFT == EXPLE) || (MYREFT == EXPLT)) || (MYREFT == EXPCLS)) {
        //
        // In this case the reference values are actually stored
        // in the file.  This is even easier than the last case.
        // We simply fetch them with a call to DAF. We do not check for a
        // failure here, since all we do after the attempt to read is
        // checkout and return anyway.
        //
        B = (BASE + FIRST);
        E = (BASE + LAST);

        DAFGDA(HANDLE, B, E, VALUES.as_slice_mut(), ctx)?;
    } else {
        SETMSG(b"The generic DAF segment you attempted to read has an unsupported reference directory structure. The integer code given for this structure is #. The likely cause of this anomaly is that your version of SPICELIB needs to be updated. Contact your system administrator or NAIF for a toolkit update. ", ctx);
        ERRINT(b"#", MYREFT, ctx);
        SIGERR(b"SPICE(UNKNOWNREFDIR)", ctx)?;
        CHKOUT(b"SGFREF", ctx)?;
        return Ok(());
    }

    CHKOUT(b"SGFREF", ctx)?;
    Ok(())
}
