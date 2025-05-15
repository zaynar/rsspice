//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const NCONST: i32 = 1;
const BEGEL1: i32 = (NCONST + 1);

/// Read SPK record from segment, type 14
///
/// Read a single data record from a type 14 SPK segment.
///
/// # Required Reading
///
/// * [SPK](crate::required_reading::spk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of the open SPK file.
///  DESCR      I   Descriptor of the segment with the desired record.
///  ET         I   Epoch used to identify the desired record.
///  RECORD     O   The desired type 14 SPK record.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of the open SPK file which contains the
///           segment of interest.
///
///  DESCR    is the descriptor for a type 14 SPK segment that contains
///           the record of interest.
///
///  ET       is the target epoch used to determine the particular
///           record to be obtained from the SPK segment.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RECORD   is the record from the specified segment which,
///           when evaluated at epoch ET, will give the state
///           (position and velocity) of some body, relative
///           to some center, in some inertial reference frame.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  It is assumed that the descriptor and handle supplied are
///      for a properly constructed type 14 segment. No checks are
///      performed to ensure this.
///
///  2)  If the input ET value is not within the range specified
///      in the segment descriptor, the error SPICE(TIMEOUTOFBOUNDS)
///      is signaled.
///
///  3)  If any other error occurs while looking up SPK data, the error
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
///  This subroutine will read a single record from a type 14 SPK
///  segment. The record read will provide the data necessary to
///  compute the state for a some body in some inertial frame at epoch
///  ET.
///
///  See the SPK Required Reading file for a description of the
///  structure of a type 14 SPK segment.
/// ```
///
/// # Examples
///
/// ```text
///  The data returned by the SPKRnn routine is in a raw form, taken
///  directly from the segment. As such, it will be not be directly
///  useful to a user unless they have a complete understanding of the
///  structure of the data type. Given that understanding, however,
///  the SPKRnn routines could be used to "dump" and check segment data
///  for a particular epoch, as in the example which follows.
///
///
///  C
///  C     Get a segment applicable to a specified body and epoch.
///  C
///        CALL SPKSFS ( BODY, ET, HANDLE, DESCR, IDENT, FOUND )
///
///  C
///  C     Look at parts of the descriptor.
///  C
///        CALL DAFUS ( DESCR, 2, 6, DCD, ICD )
///        CENTER = ICD( 2 )
///        REF    = ICD( 3 )
///        TYPE   = ICD( 4 )
///
///        IF ( TYPE .EQ. 14 ) THEN
///           CALL SPKR14 ( HANDLE, DESCR, ET, RECORD )
///               .
///               .  Look at the RECORD data.
///               .
///        END IF
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This subroutine should not be called directly by a casual
///      user. It is intended for use by the subroutine SPKPV, and
///      certain tests for error conditions are not performed here, as
///      SPKPV will have already performed them.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 14-APR-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Updated entry
///         #3 in $Exceptions section. Moved SPK required reading from
///         $Literature_References to $Required_Reading section.
///
/// -    SPICELIB Version 1.0.0, 10-MAR-1995 (KRG)
/// ```
pub fn spkr14(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64],
    et: f64,
    record: &mut [f64],
) -> crate::Result<()> {
    SPKR14(handle, descr, et, record, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKR14 ( Read SPK record from segment, type 14 )
pub fn SPKR14(
    HANDLE: i32,
    DESCR: &[f64],
    ET: f64,
    RECORD: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DESCR = DummyArray::new(DESCR, 1..);
    let mut RECORD = DummyArrayMut::new(RECORD, 1..);
    let mut VALUE: f64 = 0.0;
    let mut ENDS: i32 = 0;
    let mut INDX: i32 = 0;
    let mut FOUND: bool = false;

    //
    // SPICELIB functions
    //
    //
    // Local variables
    //
    // The number of constant values stored with a type 14 segment
    // segment.
    //
    //
    // The beginning location in the output record for the non constant
    // segment data.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"SPKR14", ctx)?;
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
        CHKOUT(b"SPKR14", ctx)?;
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
    // Locate the time in the file less than or equal to the input ET,
    // obtaining its index. This will allow us to retrieve the proper
    // record.
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
    // Fetch the appropriate record from the segment.
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

    CHKOUT(b"SPKR14", ctx)?;
    Ok(())
}
