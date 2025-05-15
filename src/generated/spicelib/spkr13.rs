//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const MAXREC: i32 = 129;

/// Read SPK record from segment, type 13
///
/// Read a single data record from a type 13 SPK segment.
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
///  RECORD     O   The desired type 13 SPK record.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of the open SPK file which contains
///           the segment of interest.
///
///  DESCR    is the descriptor for a type 13 SPK segment that
///           contains the record of interest.
///
///  ET       is the target epoch used to determine the
///           particular record to be obtained from the SPK
///           segment.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RECORD   is the record from the specified segment which,
///           when evaluated at epoch ET, will give the state
///           (position and velocity) of some body, relative
///           to some center, in some inertial reference frame.
///
///           The structure of the record is as follows:
///
///              +----------------------+
///              | number of states (n) |
///              +----------------------+
///              | state 1 (6 elts.)    |
///              +----------------------+
///              | state 2 (6 elts.)    |
///              +----------------------+
///                          .
///                          .
///                          .
///              +----------------------+
///              | state n (6 elts.)    |
///              +----------------------+
///              | epochs 1--n          |
///              +----------------------+
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  It is assumed that the descriptor and handle supplied are
///      for a properly constructed type 13 segment. No checks are
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
///  This subroutine will read a single record from a type 13 SPK
///  segment. The record read will provide the data necessary to
///  compute the state for the body designated by DESCR at epoch
///  ET.
///
///  The exact format and structure of a type 13 SPK segment is
///  described in the SPK Required Reading.
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
///        IF ( TYPE .EQ. 13 ) THEN
///           CALL SPKR13 ( HANDLE, DESCR, ET, RECORD )
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
///      user. It is intended for use by the subroutine SPKPVN, and
///      certain tests for error conditions are not performed here, as
///      SPKPVN will have already performed them.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 14-APR-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Updated entry
///         #3 in $Exceptions section. Moved SPK required reading from
///         $Literature_References to $Required_Reading section.
///
/// -    SPICELIB Version 1.0.0, 25-FEB-2000 (NJB)
/// ```
pub fn spkr13(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64],
    et: f64,
    record: &mut [f64],
) -> crate::Result<()> {
    SPKR13(handle, descr, et, record, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKR13 ( Read SPK record from segment, type 13 )
pub fn SPKR13(
    HANDLE: i32,
    DESCR: &[f64],
    ET: f64,
    RECORD: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DESCR = DummyArray::new(DESCR, 1..);
    let mut RECORD = DummyArrayMut::new(RECORD, 1..);

    //
    // SPICELIB functions
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"SPKR13", ctx)?;

    //
    // The type 9 reader knows how to obtain a type 13 record.
    //
    SPKR09(HANDLE, DESCR.as_slice(), ET, RECORD.as_slice_mut(), ctx)?;

    CHKOUT(b"SPKR13", ctx)?;
    Ok(())
}
