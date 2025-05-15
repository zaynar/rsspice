//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// SPK, read record from segment, type 2
///
/// Read a single SPK data record from a segment of type 2
/// (Chebyshev, position only).
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
///  HANDLE     I   File handle.
///  DESCR      I   Segment descriptor.
///  ET         I   Evaluation epoch.
///  RECORD     O   Data record.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE,
///  DESCR    are the file handle and segment descriptor for
///           an SPK segment of type 2.
///
///  ET       is an epoch for which a data record from the
///           specified segment is required. ET is expressed as
///           seconds past J2000 TDB.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RECORD   is an array of data from the specified segment which,
///           when evaluated at epoch ET, will give the state
///           (position and velocity) of the target body identified
///           by the input segment descriptor. The descriptor
///           specifies the center of motion and reference frame of
///           the state.
///
///           The structure of the record is as follows:
///
///              +--------------------------------------+
///              | record size (excluding this element) |
///              +--------------------------------------+
///              | Coverage interval midpoint           |
///              +--------------------------------------+
///              | Coverage interval radius             |
///              +--------------------------------------+
///              | Coeffs for X position component      |
///              +--------------------------------------+
///              | Coeffs for Y position component      |
///              +--------------------------------------+
///              | Coeffs for Z position component      |
///              +--------------------------------------+
///
///           In the above record
///
///              - Times are expressed as seconds past J2000 TDB.
///              - Position components have units of km.
///
///           RECORD must be declared by the caller with size large
///           enough to accommodate the largest record that can be
///           returned by this routine. See the INCLUDE file
///           spkrec.inc for the correct record length.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If an error occurs while looking up SPK data, the error is
///      signaled by a routine in the call tree of this routine.
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
///  See the SPK Required Reading file for a description of the
///  structure of a data type 2 (Chebyshev polynomials, position
///  only) segment.
/// ```
///
/// # Examples
///
/// ```text
///  The data returned by the SPKRnn routine is in its rawest form,
///  taken directly from the segment. As such, it will be meaningless
///  to a user unless he/she understands the structure of the data type
///  completely. Given that understanding, however, the SPKRxx
///  routines might be used to "dump" and check segment data for a
///  particular epoch.
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
///        IF ( TYPE .EQ. 2 ) THEN
///           CALL SPKR02 ( HANDLE, DESCR, ET, RECORD )
///               .
///               .  Look at the RECORD data.
///               .
///        END IF
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.2, 14-APR-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Moved SPK
///         required reading from $Literature_References to
///         $Required_Reading section.
///
/// -    SPICELIB Version 1.1.1, 18-JAN-2014 (NJB)
///
///         Enhanced header and in-line documentation.
///
/// -    SPICELIB Version 1.1.0, 07-SEP-2001 (EDW)
///
///         Replaced DAFRDA call with DAFGDA.
///         Added IMPLICIT NONE.
///
/// -    SPICELIB Version 1.0.3, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.2, 23-AUG-1991 (HAN)
///
///         SPK02 was removed from the $Required_Reading section of the
///         header. The information in the SPK02 Required Reading file
///         is now part of the SPK Required Reading file.
///
/// -    SPICELIB Version 1.0.1, 22-MAR-1990 (HAN)
///
///         Literature references added to the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn spkr02(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64; 5],
    et: f64,
    record: &mut [f64],
) -> crate::Result<()> {
    SPKR02(handle, descr, et, record, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKR02 ( SPK, read record from segment, type 2 )
pub fn SPKR02(
    HANDLE: i32,
    DESCR: &[f64],
    ET: f64,
    RECORD: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DESCR = DummyArray::new(DESCR, 1..=5);
    let mut RECORD = DummyArrayMut::new(RECORD, 1..);
    let mut DC = StackArray::<f64, 2>::new(1..=2);
    let mut INIT: f64 = 0.0;
    let mut INTLEN: f64 = 0.0;
    let mut IC = StackArray::<i32, 6>::new(1..=6);
    let mut BEGIN: i32 = 0;
    let mut END: i32 = 0;
    let mut RECSIZ: i32 = 0;
    let mut NREC: i32 = 0;
    let mut RECNO: i32 = 0;
    let mut RECADR: i32 = 0;

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
        CHKIN(b"SPKR02", ctx)?;
    }

    //
    // Unpack the segment descriptor.
    //
    DAFUS(DESCR.as_slice(), 2, 6, DC.as_slice_mut(), IC.as_slice_mut());

    BEGIN = IC[5];
    END = IC[6];

    //
    // The segment is made up of a number of logical records, each
    // having the same size, and covering the same length of time.
    //
    // We can determine which record to return using the input epoch,
    // the initial time of the first record's coverage interval, and the
    // length of the interval covered by each record. These constants
    // are located at the end of the segment, along with the size of
    // each logical record and the total number of records.
    //
    DAFGDA(HANDLE, (END - 3), END, RECORD.as_slice_mut(), ctx)?;

    INIT = RECORD[1];
    INTLEN = RECORD[2];
    RECSIZ = (RECORD[3] as i32);
    NREC = (RECORD[4] as i32);

    RECNO = ((((ET - INIT) / INTLEN) as i32) + 1);
    RECNO = intrinsics::MIN0(&[RECNO, NREC]);

    //
    // Compute the address of the desired record.
    //
    RECADR = (((RECNO - 1) * RECSIZ) + BEGIN);

    //
    // Along with the record, return the size of the record.
    //
    RECORD[1] = RECORD[3];
    DAFGDA(
        HANDLE,
        RECADR,
        ((RECADR + RECSIZ) - 1),
        RECORD.subarray_mut(2),
        ctx,
    )?;

    CHKOUT(b"SPKR02", ctx)?;
    Ok(())
}
