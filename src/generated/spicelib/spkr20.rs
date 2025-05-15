//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// SPK, read record from segment, type 20
///
/// Read a single SPK data record from a segment of type 20
/// (Chebyshev, velocity coefficients only).
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
///  DESCR    are the file handle and segment descriptor for an SPK
///           segment of type 20.
///
///  ET       is an epoch for which a data record from the specified
///           segment is required. ET is expressed as seconds past
///           J2000 TDB.
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
///              | Coeffs for X velocity component      |
///              +--------------------------------------+
///              | Coeffs for Y velocity component      |
///              +--------------------------------------+
///              | Coeffs for Z velocity component      |
///              +--------------------------------------+
///              | X position component                 |
///              +--------------------------------------+
///              | Y position component                 |
///              +--------------------------------------+
///              | Z position component                 |
///              +--------------------------------------+
///
///           In the above record
///
///              - Times are expressed as seconds past J2000 TDB.
///              - Position components have units of km.
///              - Velocity coefficients have units of km/s.
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
///  structure of a data type 20 (Chebyshev polynomials, velocity
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
///        IF ( TYPE .EQ. 20 ) THEN
///           CALL SPKR20 ( HANDLE, DESCR, ET, RECORD )
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
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 14-APR-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Moved SPK
///         required reading from $Literature_References to
///         $Required_Reading section.
///
/// -    SPICELIB Version 1.0.0, 17-JAN-2014 (NJB) (IMU)
/// ```
pub fn spkr20(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64; 5],
    et: f64,
    record: &mut [f64],
) -> crate::Result<()> {
    SPKR20(handle, descr, et, record, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKR20 ( SPK, read record from segment, type 20 )
pub fn SPKR20(
    HANDLE: i32,
    DESCR: &[f64],
    ET: f64,
    RECORD: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DESCR = DummyArray::new(DESCR, 1..=5);
    let mut RECORD = DummyArrayMut::new(RECORD, 1..);
    let mut DC = StackArray::<f64, 2>::new(1..=2);
    let mut DSCALE: f64 = 0.0;
    let mut INIT: f64 = 0.0;
    let mut INITFR: f64 = 0.0;
    let mut INITJD: f64 = 0.0;
    let mut INTLEN: f64 = 0.0;
    let mut INTRVL: f64 = 0.0;
    let mut MID: f64 = 0.0;
    let mut POS = StackArray::<f64, 3>::new(1..=3);
    let mut RADIUS: f64 = 0.0;
    let mut RECBEG: f64 = 0.0;
    let mut TSCALE: f64 = 0.0;
    let mut BEGIN: i32 = 0;
    let mut END: i32 = 0;
    let mut IC = StackArray::<i32, 6>::new(1..=6);
    let mut LOC: i32 = 0;
    let mut NREC: i32 = 0;
    let mut NTERMS: i32 = 0;
    let mut RECADR: i32 = 0;
    let mut RECNO: i32 = 0;
    let mut RECSIZ: i32 = 0;
    let mut SIZE: i32 = 0;

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
    }

    CHKIN(b"SPKR20", ctx)?;
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
    // the integer and fractional parts of the initial time of the first
    // record's coverage interval, and the length of the interval
    // covered by each record. These constants are located at the end of
    // the segment, along with the size of each logical record and the
    // total number of records.
    //
    // For convenience, we'll fetch the segment's distance and time
    // scales in the same call.
    //
    DAFGDA(HANDLE, (END - 6), END, RECORD.as_slice_mut(), ctx)?;

    DSCALE = RECORD[1];
    TSCALE = RECORD[2];
    INITJD = RECORD[3];
    INITFR = RECORD[4];
    INTLEN = RECORD[5];
    RECSIZ = (RECORD[6] as i32);
    NREC = (RECORD[7] as i32);

    //
    // NTERMS is the number of velocity coefficients per component,
    // plus 1 (for the position component).
    //
    NTERMS = (RECSIZ / 3);

    //
    // Convert the initial epoch and interval length to
    // seconds past J2000 TDB.
    //
    INIT = (((INITJD - J2000()) + INITFR) * SPD());
    INTRVL = (INTLEN * SPD());

    //
    // Locate the record containing the coefficients to use.
    //
    RECNO = ((((ET - INIT) / INTRVL) as i32) + 1);
    RECNO = intrinsics::MAX0(&[1, intrinsics::MIN0(&[RECNO, NREC])]);

    //
    // Compute the midpoint and radius of the record at
    // index RECNO. We want to compute the midpoint in such
    // a way that we take advantage of interval lengths that
    // are exactly representable, when we have them.
    //
    // RECBEG is the record start time, minus the fractional
    // part of the segment start time, expressed as seconds
    // past J2000. We'll account for the fractional part of the
    // start time below when we compute MID.
    //
    RECBEG = (((INITJD - J2000()) + (((RECNO - 1) as f64) * INTLEN)) * SPD());

    RADIUS = (INTRVL / 2.0);

    MID = ((RECBEG + (INITFR * SPD())) + RADIUS);

    //
    // Compute the address of the desired record.
    //
    RECADR = (((RECNO - 1) * RECSIZ) + BEGIN);

    //
    // Along with the record, return the size, midpoint, and
    // radius of the record.
    //
    RECORD[1] = ((RECSIZ + 2) as f64);
    RECORD[2] = MID;
    RECORD[3] = RADIUS;
    DAFGDA(
        HANDLE,
        RECADR,
        ((RECADR + RECSIZ) - 1),
        RECORD.subarray_mut(4),
        ctx,
    )?;

    //
    // We're going to re-arrange the record: the position components
    // will be transferred to the end of the record, and the record
    // contents will be left-shifted to fill in the free elements.
    //
    for I in 1..=3 {
        POS[I] = RECORD[(3 + (I * NTERMS))];
    }

    SIZE = (RECSIZ + 3);
    //
    // Remove the position elements from the record.
    //
    for I in 1..=3 {
        //
        // LOC is the index of the element to delete. After the first
        // removal, we must account for the resulting left shift when
        // calculating the indices of subsequent elements to be removed.
        //
        LOC = ((3 + (I * NTERMS)) - (I - 1));

        REMLAD(1, LOC, RECORD.as_slice_mut(), &mut SIZE, ctx)?;
        //
        // Note that SIZE is an in-out argument; on output it indicates
        // the size of the array after removal of the indicated
        // element(s).
        //
    }

    //
    // Convert the position vector to km.
    //
    VSCLIP(DSCALE, POS.as_slice_mut());

    //
    // Append the position to the record. Since we inserted three
    // elements at the start of the record and deleted three position
    // elements, the target index is the same as if we had copied the
    // record directly to the output array.
    //
    MOVED(POS.as_slice(), 3, RECORD.subarray_mut((RECSIZ + 1)));

    //
    // Convert the velocity Chebyshev coefficients to units of km/s.
    //
    for I in 4..=RECSIZ {
        RECORD[I] = (RECORD[I] * (DSCALE / TSCALE));
    }

    CHKOUT(b"SPKR20", ctx)?;
    Ok(())
}
