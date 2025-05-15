//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const NDC: i32 = 2;
const NIC: i32 = 6;
const DTYPE: i32 = 5;

/// C-kernel, number of records, type 05
///
/// Return the number of pointing instances in a CK type 05 segment.
/// The segment is identified by a CK file handle and segment
/// descriptor.
///
/// # Required Reading
///
/// * [CK](crate::required_reading::ck)
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   The handle of the file containing the segment.
///  DESCR      I   The descriptor of the type 5 segment.
///  NREC       O   The number of pointing instances in the segment.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of the binary CK file containing the
///           segment.
///
///  DESCR    is the packed descriptor of a data type 5 segment.
/// ```
///
/// # Detailed Output
///
/// ```text
///  NREC     is the number of pointing instances in the type 5
///           segment.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the segment indicated by DESCR is not a type 5 segment,
///      the error SPICE(CKWRONGDATATYPE) is signaled.
///
///  2)  If the specified handle does not belong to any DAF file that
///      is currently known to be open, an error is signaled by a
///      routine in the call tree of this routine.
///
///  3)  If DESCR is not a valid descriptor of a segment in the CK
///      file specified by HANDLE, the results of this routine are
///      unpredictable.
/// ```
///
/// # Files
///
/// ```text
///  The file specified by HANDLE should be open for read or
///  write access.
/// ```
///
/// # Particulars
///
/// ```text
///  For a complete description of the internal structure of a type 5
///  segment, see the CK required reading.
///
///  This routine returns the number of discrete pointing instances
///  contained in the specified segment. It is normally used in
///  conjunction with CKGR05 which returns the Ith pointing instance
///  in the segment.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that MOC.BC is a CK file that contains segments of
///  data type 5. Then the following code fragment extracts the
///  SCLK time and boresight vector for each pointing instance
///  in the first segment in the file.
///
///
///        INTEGER               ICD     ( 6 )
///        INTEGER               HANDLE
///        INTEGER               NREC
///        INTEGER               I
///
///        DOUBLE PRECISION      DCD     ( 2 )
///        DOUBLE PRECISION      DESCR   ( 5 )
///        DOUBLE PRECISION      RECORD  ( 16 )
///        DOUBLE PRECISION      QUAT    ( 4 )
///        DOUBLE PRECISION      BORE    ( 3 )
///        DOUBLE PRECISION      CMAT    ( 3, 3 )
///        DOUBLE PRECISION      SCLKDP
///
///        LOGICAL               FOUND
///
///  C
///  C     First load the file. (The file may also be opened by using
///  C     CKLPF.)
///  C
///        CALL DAFOPR ( 'MOC.BC', HANDLE )
///
///  C
///  C     Begin forward search. Find the first array.
///  C
///        CALL DAFBFS ( HANDLE )
///        CALL DAFFNA ( FOUND  )
///
///  C
///  C     Get segment descriptor.
///  C
///        CALL DAFGS ( DESCR )
///
///  C
///  C     Unpack the segment descriptor into its double precision
///  C     and integer components.
///  C
///        CALL DAFUS ( DESCR, 2, 6, DCD, ICD )
///
///  C
///  C     The data type for a segment is located in the third integer
///  C     component of the descriptor.
///  C
///        IF ( ICD( 3 ) .EQ. 5 ) THEN
///  C
///  C        How many records does this segment contain?
///  C
///           CALL CKNR05 ( HANDLE, DESCR, NREC )
///
///           DO I = 1, NREC
///  C
///  C           Get the Ith pointing instance in the segment.
///  C
///              CALL CKGR05 ( HANDLE, DESCR, I, RECORD )
///
///  C
///  C           Unpack from RECORD the time tag and quaternion.
///  C           The locations of these items in the record are
///  C           independent of the subtype.
///  C
///              SCLKDP = RECORD ( 1 )
///
///              CALL MOVED ( RECORD(3), 4, QUAT )
///
///  C
///  C           The boresight vector is the third row of the C-matrix.
///  C
///              CALL Q2M ( QUAT, CMAT )
///
///              BORE(1) = CMAT(3,1)
///              BORE(2) = CMAT(3,2)
///              BORE(3) = CMAT(3,3)
///  C
///  C           Write out the results.
///  C
///              WRITE (*,*) 'Record: ', I
///              WRITE (*,*)
///              WRITE (*,*) 'SCLK time = ', SCLKDP
///              WRITE (*,*)
///              WRITE (*,*) 'boresight: ', BORE
///
///           END DO
///
///        END IF
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  J.M. Lynch         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 26-OCT-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 18-AUG-2002 (NJB) (JML)
/// ```
pub fn cknr05(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64],
    nrec: &mut i32,
) -> crate::Result<()> {
    CKNR05(handle, descr, nrec, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CKNR05 ( C-kernel, number of records, type 05 )
pub fn CKNR05(
    HANDLE: i32,
    DESCR: &[f64],
    NREC: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DESCR = DummyArray::new(DESCR, 1..);
    let mut ICD = StackArray::<i32, 6>::new(1..=NIC);
    let mut DCD = StackArray::<f64, 2>::new(1..=NDC);
    let mut NPOINT: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    //    NDC        is the number of double precision components in an
    //               unpacked C-kernel descriptor.
    //
    //    NIC        is the number of integer components in an unpacked
    //               C-kernel descriptor.
    //
    //    DTYPE      is the data type of the segment that this routine
    //               operates on.
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
        CHKIN(b"CKNR05", ctx)?;
    }

    //
    // The number of discrete pointing instances contained in a data
    // type 5 segment is stored in the last double precision word of
    // the segment.  Since the address of the last word is stored in
    // the sixth integer component of the segment descriptor, it is
    // a trivial matter to extract the count.
    //
    // The unpacked descriptor contains the following information
    // about the segment:
    //
    //    DCD(1)  Initial encoded SCLK
    //    DCD(2)  Final encoded SCLK
    //    ICD(1)  Instrument
    //    ICD(2)  Inertial reference frame
    //    ICD(3)  Data type
    //    ICD(4)  Angular velocity flag
    //    ICD(5)  Initial address of segment data
    //    ICD(6)  Final address of segment data
    //
    //
    DAFUS(
        DESCR.as_slice(),
        NDC,
        NIC,
        DCD.as_slice_mut(),
        ICD.as_slice_mut(),
    );

    //
    // If this segment is not of data type 5, then signal an error.
    //

    if (ICD[3] != DTYPE) {
        SETMSG(
            b"Data type of the segment should be 5: Passed descriptor shows type = #.",
            ctx,
        );
        ERRINT(b"#", ICD[3], ctx);
        SIGERR(b"SPICE(CKWRONGDATATYPE)", ctx)?;
        CHKOUT(b"CKNR05", ctx)?;
        return Ok(());
    }

    //
    // The number of records is the final word in the segment.
    //
    DAFGDA(
        HANDLE,
        ICD[6],
        ICD[6],
        std::slice::from_mut(&mut NPOINT),
        ctx,
    )?;

    *NREC = intrinsics::IDNINT(NPOINT);

    CHKOUT(b"CKNR05", ctx)?;
    Ok(())
}
