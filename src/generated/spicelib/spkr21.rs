//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const MAXTRM: i32 = 25;
const BUFSIZ: i32 = 100;

/// Read SPK record from segment, type 21
///
/// Read a single SPK data record from a segment of type 21
/// (Extended Difference Lines).
///
/// # Required Reading
///
/// * [SPK](crate::required_reading::spk)
/// * [TIME](crate::required_reading::time)
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
///           a SPK segment of type 21.
///
///  ET       is an epoch for which a data record from a specific
///           segment is required. The epoch is represented as
///           seconds past J2000 TDB.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RECORD   is a data record which, when evaluated at epoch ET,
///           will give the state (position and velocity) of an
///           ephemeris object, relative to its center of motion,
///           in an inertial reference frame.
///
///           The contents of RECORD are as follows:
///
///              RECORD(1):         The difference table size per
///                                 Cartesian component. Call this
///                                 size MAXDIM; then the difference
///                                 line (MDA) size DLSIZE is
///
///                                   ( 4 * MAXDIM ) + 11
///
///              RECORD(2)
///                 ...
///              RECORD(1+DLSIZE):  An extended difference line.
///                                 The contents are:
///
///                 Dimension  Description
///                 ---------  ----------------------------------
///                 1          Reference epoch of difference line
///                 MAXDIM     Stepsize function vector
///                 1          Reference position vector,  x
///                 1          Reference velocity vector,  x
///                 1          Reference position vector,  y
///                 1          Reference velocity vector,  y
///                 1          Reference position vector,  z
///                 1          Reference velocity vector,  z
///                 MAXDIM,3   Modified divided difference
///                            arrays (MDAs)
///                 1          Maximum integration order plus 1
///                 3          Integration order array
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the maximum table size of the input record exceeds
///      MAXTRM, the error SPICE(DIFFLINETOOLARGE) is signaled.
///
///  2)  If an error occurs while looking up SPK data, the error is
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
///  structure of a data type 21 segment.
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
///           CALL SPKR21 ( HANDLE, DESCR, ET, RECORD )
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
///  F.T. Krogh         (JPL)
///  W.L. Taber         (JPL)
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
/// -    SPICELIB Version 1.0.0, 16-JAN-2014 (NJB) (FTK) (WLT) (IMU)
/// ```
pub fn spkr21(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64; 5],
    et: f64,
    record: &mut [f64],
) -> crate::Result<()> {
    SPKR21(handle, descr, et, record, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKR21 ( Read SPK record from segment, type 21 )
pub fn SPKR21(
    HANDLE: i32,
    DESCR: &[f64],
    ET: f64,
    RECORD: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DESCR = DummyArray::new(DESCR, 1..=5);
    let mut RECORD = DummyArrayMut::new(RECORD, 1..);
    let mut DATA = StackArray::<f64, 100>::new(1..=BUFSIZ);
    let mut DC = StackArray::<f64, 2>::new(1..=2);
    let mut BEGIN: i32 = 0;
    let mut DFLSIZ: i32 = 0;
    let mut END: i32 = 0;
    let mut I: i32 = 0;
    let mut IC = StackArray::<i32, 6>::new(1..=6);
    let mut MAXDIM: i32 = 0;
    let mut NDIR: i32 = 0;
    let mut NREC: i32 = 0;
    let mut OFF: i32 = 0;
    let mut OFFD: i32 = 0;
    let mut OFFE: i32 = 0;
    let mut OFFR: i32 = 0;
    let mut RECNO: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
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

    CHKIN(b"SPKR21", ctx)?;

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
    // We'll fetch the difference table dimension as well.
    //
    DAFGDA(HANDLE, (END - 1), END, DATA.as_slice_mut(), ctx)?;

    NREC = intrinsics::IDNINT(DATA[2]);
    NDIR = (NREC / BUFSIZ);
    OFFD = ((END - NDIR) - 2);
    OFFE = (OFFD - NREC);

    MAXDIM = intrinsics::IDNINT(DATA[1]);

    if (MAXDIM > MAXTRM) {
        SETMSG(b"The input record has a maximum table dimension of #, while the maximum supported by this routine is #. It is possible that this problem is due to your SPICE Toolkit being out of date.", ctx);
        ERRINT(b"#", MAXDIM, ctx);
        ERRINT(b"#", MAXTRM, ctx);
        SIGERR(b"SPICE(DIFFLINETOOLARGE)", ctx)?;
        CHKOUT(b"SPKR21", ctx)?;
        return Ok(());
    }

    //
    // The difference line dimension per component is the
    // first element of the output record.
    //
    RECORD[1] = MAXDIM as f64;

    //
    // Set the difference line size.
    //
    DFLSIZ = ((4 * MAXDIM) + 11);

    //
    // What we want is the record number: once we have that, we can
    // compute the offset of the record from the beginning of the
    // segment, grab it, and go. But how to find it?
    //
    // Ultimately, we want the first record whose epoch is greater
    // than or equal to ET. If there are BUFSIZ or fewer records, all
    // the record epochs can be examined in a single group.
    //
    if (NREC <= BUFSIZ) {
        DAFGDA(HANDLE, (OFFE + 1), (OFFE + NREC), DATA.as_slice_mut(), ctx)?;
        RECNO = (LSTLTD(ET, NREC, DATA.as_slice()) + 1);

        OFFR = ((BEGIN - 1) + ((RECNO - 1) * DFLSIZ));
        DAFGDA(
            HANDLE,
            (OFFR + 1),
            (OFFR + DFLSIZ),
            RECORD.subarray_mut(2),
            ctx,
        )?;

        CHKOUT(b"SPKR21", ctx)?;
        return Ok(());
    }

    //
    // Searching directories is a little more difficult.
    //
    // The directory contains epochs BUFSIZ, 2*BUFSIZ, and so on. Once
    // we find the first directory epoch greater than or equal to ET, we
    // can grab the corresponding set of BUFSIZ record epochs, and
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
                OFF = (OFFE + ((I - 1) * BUFSIZ));
                DAFGDA(HANDLE, (OFF + 1), (OFF + BUFSIZ), DATA.as_slice_mut(), ctx)?;

                RECNO = ((((I - 1) * BUFSIZ) + LSTLTD(ET, BUFSIZ, DATA.as_slice())) + 1);

                OFFR = ((BEGIN - 1) + ((RECNO - 1) * DFLSIZ));
                DAFGDA(
                    HANDLE,
                    (OFFR + 1),
                    (OFFR + DFLSIZ),
                    RECORD.subarray_mut(2),
                    ctx,
                )?;

                CHKOUT(b"SPKR21", ctx)?;
                return Ok(());
            }

            I += m3__;
        }
    }

    //
    // If ET is greater than the final directory epoch, we want one
    // of the final records.
    //
    I = intrinsics::MOD(NREC, BUFSIZ);

    DAFGDA(
        HANDLE,
        (((END - NDIR) - I) - 1),
        ((END - NDIR) - 2),
        DATA.as_slice_mut(),
        ctx,
    )?;

    RECNO = (((NDIR * BUFSIZ) + LSTLTD(ET, I, DATA.as_slice())) + 1);

    OFFR = ((BEGIN - 1) + ((RECNO - 1) * DFLSIZ));

    DAFGDA(
        HANDLE,
        (OFFR + 1),
        (OFFR + DFLSIZ),
        RECORD.subarray_mut(2),
        ctx,
    )?;

    CHKOUT(b"SPKR21", ctx)?;
    Ok(())
}
