//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Read SPK record from segment, type 1
///
/// Read a single SPK data record from a segment of type 1
/// (Difference Lines).
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
///  ET         I   Target epoch.
///  RECORD     O   Data record.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE,
///  DESCR    are the file handle and segment descriptor for
///           a SPK segment of type 1.
///
///  ET       is a target epoch, for which a data record from
///           a specific segment is required.
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
///  structure of a data type 1 segment.
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
///        IF ( TYPE .EQ. 1 ) THEN
///           CALL SPKR01 ( HANDLE, DESCR, ET, RECORD )
///               .
///               .  Look at the RECORD data.
///               .
///        END IF
/// ```
///
/// # Author and Institution
///
/// ```text
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
/// -    SPICELIB Version 1.1.1, 14-APR-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Added entry #1
///         to $Exceptions section. Moved SPK required reading from
///         $Literature_References to $Required_Reading section.
///
/// -    SPICELIB Version 1.1.0, 07-SEP-2001 (EDW)
///
///         Replaced DAFRDA call with DAFGDA.
///
/// -    SPICELIB Version 1.0.3, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.2, 23-AUG-1991 (HAN)
///
///         SPK01 was removed from the $Required_Reading section of the
///         header. The information in the SPK01 Required Reading file
///         is now part of the SPK Required Reading file.
///
/// -    SPICELIB Version 1.0.1, 22-MAR-1990 (HAN)
///
///         Literature references added to the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn spkr01(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64; 5],
    et: f64,
    record: &mut [f64],
) -> crate::Result<()> {
    SPKR01(handle, descr, et, record, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKR01 ( Read SPK record from segment, type 1 )
pub fn SPKR01(
    HANDLE: i32,
    DESCR: &[f64],
    ET: f64,
    RECORD: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DESCR = DummyArray::new(DESCR, 1..=5);
    let mut RECORD = DummyArrayMut::new(RECORD, 1..);
    let mut DC = StackArray::<f64, 2>::new(1..=2);
    let mut IC = StackArray::<i32, 6>::new(1..=6);
    let mut BEGIN: i32 = 0;
    let mut END: i32 = 0;
    let mut DATA = StackArray::<f64, 100>::new(1..=100);
    let mut NREC: i32 = 0;
    let mut NDIR: i32 = 0;
    let mut OFFD: i32 = 0;
    let mut OFFE: i32 = 0;
    let mut OFF: i32 = 0;
    let mut RECNO: i32 = 0;
    let mut OFFR: i32 = 0;
    let mut I: i32 = 0;

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
        CHKIN(b"SPKR01", ctx)?;
    }

    //
    // Unpack the segment descriptor.
    //
    DAFUS(DESCR.as_slice(), 2, 6, DC.as_slice_mut(), IC.as_slice_mut());

    BEGIN = IC[5];
    END = IC[6];

    //
    // Get the number of records in the segment. From that, we can
    // compute
    //
    //    NDIR      The number of directory epochs.
    //
    //    OFFD      The offset of the first directory epoch.
    //
    //    OFFE      The offset of the first epoch.
    //
    //
    // the number of directory epochs.
    //
    DAFGDA(HANDLE, END, END, DATA.as_slice_mut(), ctx)?;
    NREC = (DATA[1] as i32);

    NDIR = (NREC / 100);
    OFFD = ((END - NDIR) - 1);
    OFFE = (((END - NDIR) - NREC) - 1);

    //
    // What we want is the record number: once we have that, we can
    // compute the offset of the record from the beginning of the
    // segment, grab it, and go. But how to find it?
    //
    // Ultimately, we want the first record whose epoch is greater
    // than or equal to ET. If there are 100 or fewer records, all
    // the record epochs can be examined in a single group.
    //
    if (NREC <= 100) {
        DAFGDA(HANDLE, (OFFE + 1), (OFFE + NREC), DATA.as_slice_mut(), ctx)?;
        RECNO = (LSTLTD(ET, NREC, DATA.as_slice()) + 1);

        OFFR = ((BEGIN - 1) + ((RECNO - 1) * 71));
        DAFGDA(HANDLE, (OFFR + 1), (OFFR + 71), RECORD.as_slice_mut(), ctx)?;

        CHKOUT(b"SPKR01", ctx)?;
        return Ok(());
    }

    //
    // Searching directories is a little more difficult.
    //
    // The directory contains epochs 100, 200, and so on. Once we
    // find the first directory epoch greater than or equal to ET,
    // we can grab the corresponding set of 100 record epochs, and
    // search them.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NDIR;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            DAFGDA(HANDLE, (OFFD + I), (OFFD + I), DATA.as_slice_mut(), ctx)?;

            if (DATA[1] >= ET) {
                OFF = (OFFE + ((I - 1) * 100));
                DAFGDA(HANDLE, (OFF + 1), (OFF + 100), DATA.as_slice_mut(), ctx)?;

                RECNO = ((((I - 1) * 100) + LSTLTD(ET, 100, DATA.as_slice())) + 1);

                OFFR = ((BEGIN - 1) + ((RECNO - 1) * 71));
                DAFGDA(HANDLE, (OFFR + 1), (OFFR + 71), RECORD.as_slice_mut(), ctx)?;

                CHKOUT(b"SPKR01", ctx)?;
                return Ok(());
            }
            I += m3__;
        }
    }

    //
    // If ET is greater than the final directory epoch, we want one
    // of the final records.
    //
    I = intrinsics::MOD(NREC, 100);

    DAFGDA(
        HANDLE,
        ((END - NDIR) - I),
        ((END - NDIR) - 1),
        DATA.as_slice_mut(),
        ctx,
    )?;
    RECNO = (((NDIR * 100) + LSTLTD(ET, I, DATA.as_slice())) + 1);

    OFFR = ((BEGIN - 1) + ((RECNO - 1) * 71));
    DAFGDA(HANDLE, (OFFR + 1), (OFFR + 71), RECORD.as_slice_mut(), ctx)?;

    CHKOUT(b"SPKR01", ctx)?;
    Ok(())
}
