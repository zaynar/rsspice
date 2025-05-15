//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const NCONST: i32 = 1;
const BEGEL1: i32 = (NCONST + 1);

/// PCK, read record from type 3 segment
///
/// Read a single PCK data record from a segment of type 03.
///
/// # Required Reading
///
/// * [PCK](crate::required_reading::pck)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   File handle for a PCK file.
///  DESCR      I   Descriptor for a type 03 PCK segment.
///  ET         I   Target epoch for orientation information.
///  RECORD     O   Data record associated with epoch ET.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the file handle for a type 03 PCK segment.
///
///  DESCR    is the segment descriptor for a type 03 PCK segment.
///
///  ET       is a target epoch, for which a data record from
///           the specified segment is required.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RECORD   is the record from the specified segment which,
///           when evaluated at epoch ET, will give the RA, DEC,
///           W and body fixed angular rates for the body associated
///           with the segment.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  It is assumed that the descriptor and handle supplied are
///      for a properly constructed type 03 segment. No checks are
///      performed to ensure this.
///
///  2)  If the input ET value is not within the range specified
///      in the segment descriptor, the error SPICE(TIMEOUTOFBOUNDS)
///      is signaled.
///
///  3)  If any issue is detected while reading the PCK data, an error
///      is signaled by a routine in the call tree of this routine.
/// ```
///
/// # Files
///
/// ```text
///  See argument HANDLE.
/// ```
///
/// # Particulars
///
/// ```text
///  This subroutine reads a type 03 PCK record from the segment
///  specified by HANDLE and DESCR. The record read will contain
///  sufficient information to to compute RA, DEC, W and body fixed
///  angular rates for the body associated with the segment for epoch
///  ET.
///
///  See the PCK Required Reading file for a description of the
///  structure of a type 03 PCK segment.
/// ```
///
/// # Examples
///
/// ```text
///  The data returned by the PCKRnn routine is in its rawest form,
///  taken directly from the segment. As such, it will be meaningless
///  to a user unless he/she understands the structure of the data type
///  completely. Given that understanding, however, the PCKRnn
///  routines might be used to "dump" and check segment data for a
///  particular epoch.
///
///
///  C
///  C     Get a segment applicable to a specified body and epoch.
///  C
///        CALL PCKSFS ( BODY, ET, HANDLE, DESCR, IDENT, FOUND )
///
///  C
///  C     Look at parts of the descriptor.
///  C
///        CALL DAFUS ( DESCR, 2, 6, DCD, ICD )
///        CENTER = ICD( 2 )
///        REF    = ICD( 3 )
///        TYPE   = ICD( 4 )
///
///        IF ( TYPE .EQ. 03 ) THEN
///           CALL PCKR03 ( HANDLE, DESCR, ET, RECORD )
///               .
///               .  Look at the RECORD data.
///               .
///        END IF
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  It is assumed that the descriptor and handle supplied are
///      for a properly constructed type 03 segment. No checks are
///      performed to ensure this.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.2, 12-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.1, 03-JAN-2014 (EDW)
///
///         Minor edits to $Procedure; clean trailing whitespace.
///
/// -    SPICELIB Version 1.0.0, 20-SEP-1995 (KRG)
/// ```
pub fn pckr03(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64; 5],
    et: f64,
    record: &mut [f64],
) -> crate::Result<()> {
    PCKR03(handle, descr, et, record, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure PCKR03 ( PCK, read record from type 3 segment )
pub fn PCKR03(
    HANDLE: i32,
    DESCR: &[f64],
    ET: f64,
    RECORD: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DESCR = DummyArray::new(DESCR, 1..=5);
    let mut RECORD = DummyArrayMut::new(RECORD, 1..);
    let mut VALUE: f64 = 0.0;
    let mut ENDS: i32 = 0;
    let mut INDX: i32 = 0;
    let mut FOUND: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //
    // The number of constant values stored with a type 03 segment
    // segment.
    //
    //
    // The beginning location in the output record for the non-constant
    // segment data.
    //

    //
    // Local Variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"PCKR03", ctx)?;
    }
    //
    // Check the request time against the time bounds in the segment
    // descriptor. In order to get the right data back from the generic
    // segment calls below, we need to be sure that the desired epoch
    // falls within the bounds of the segment, as specified by the
    // descriptor. The first two elements of the descriptor are the start
    // time for the segment and the stop time for the segment,
    // respectively.
    //
    if ((ET < DESCR[1]) || (ET > DESCR[2])) {
        SETMSG(
            b"Request time # is outside of descriptor bounds # : #.",
            ctx,
        );
        ERRDP(b"#", ET, ctx);
        ERRDP(b"#", DESCR[1], ctx);
        ERRDP(b"#", DESCR[2], ctx);
        SIGERR(b"SPICE(TIMEOUTOFBOUNDS)", ctx)?;
        CHKOUT(b"PCKR03", ctx)?;
        return Ok(());
    }
    //
    // Fetch the constants and store them in the first part of
    // the output RECORD.
    //
    SGFCON(
        HANDLE,
        DESCR.as_slice(),
        1,
        NCONST,
        RECORD.subarray_mut(1),
        ctx,
    )?;
    //
    // Locate the time in the file less than or equal to the input ET.
    //
    SGFRVI(
        HANDLE,
        DESCR.as_slice(),
        ET,
        &mut VALUE,
        &mut INDX,
        &mut FOUND,
        ctx,
    )?;
    //
    // Fetch the data record.
    //
    SGFPKT(
        HANDLE,
        DESCR.as_slice(),
        INDX,
        INDX,
        RECORD.subarray_mut(BEGEL1),
        std::slice::from_mut(&mut ENDS),
        ctx,
    )?;

    CHKOUT(b"PCKR03", ctx)?;
    Ok(())
}
