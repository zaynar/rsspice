//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const C05TP0: i32 = 0;
const C05TP1: i32 = (C05TP0 + 1);
const C05TP2: i32 = (C05TP1 + 1);
const C05TP3: i32 = (C05TP2 + 1);
const C05PS0: i32 = 8;
const C05PS1: i32 = 4;
const C05PS2: i32 = 14;
const C05PS3: i32 = 7;
const NDC: i32 = 2;
const NIC: i32 = 6;
const DTYPE: i32 = 5;

/// C-kernel, get record, type 05
///
/// Return a specified pointing instance from a CK type 05 segment.
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
///  DESCR      I   The segment descriptor.
///  RECNO      I   The number of the pointing instance to be returned.
///  RECORD     O   The pointing record.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of the binary CK file containing the
///           desired segment.
///
///  DESCR    is the packed descriptor of the data type 5 segment.
///
///  RECNO    is the number of the discrete pointing instance to be
///           returned from the data type 5 segment.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RECORD   is the pointing instance indexed by RECNO in the
///           segment. The contents are as follows:
///
///              RECORD( 1 ) = CLKOUT
///
///           CLKOUT is the encoded spacecraft clock time associated
///           with the returned pointing values.
///
///              RECORD( 2 ) = SUBTYP
///
///           SUBTYP is the CK type 5 subtype code. This code
///           identifies the structure and meaning of the rest
///           of the record. However, all subtypes have a
///           quaternion stored in elements 3-6.
///
///              RECORD( 3 ) = q0
///              RECORD( 4 ) = q1
///              RECORD( 5 ) = q2
///              RECORD( 6 ) = q3
///
///           Subtype 1 ends here; there are no angular velocity
///           data. Angular velocity is derived by differentiating
///           Lagrange interpolating polynomials.
///
///              RECORD(  7 ) =  ]
///              RECORD(  8 ) =  ] --- For subtypes 0 and 2, these
///              RECORD(  9 ) =  ]     elements contain a quaternion
///              RECORD( 10 ) =  ]     derivative. For subtype 3,
///                                    elements 7-9 contain an
///                                    angular velocity vector;
///                                    element 10 is unassigned.
///
///                                    All subtypes except subtype
///                                    2 stop here.
///
///              RECORD( 11 ) =  ]
///              RECORD( 12 ) =  ] --- For subtype 2, these
///              RECORD( 13 ) =  ]     elements contain an angular
///                                    velocity vector.
///
///
///              RECORD( 14 ) =  ]
///              RECORD( 15 ) =  ] --- For subtype 2, these
///              RECORD( 16 ) =  ]     elements contain the
///                                    derivative of an angular
///                                    velocity vector.
///
///           The quantities q0 - q3 are the components of the
///           quaternion that represents the C-matrix that transforms
///           vectors from the inertial reference frame of the
///           segment to the instrument frame at time CLKOUT.
///
///           Quaternion derivatives, angular velocity, or the
///           derivative of angular velocity are returned only
///           these are supported by the segment subtype and
///           if the segment descriptor indicates that angular
///           velocity is present.
///
///           The components of the angular velocity vector are
///           specified relative to the inertial reference frame of
///           the segment.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the segment is not of data type 5, the error
///      SPICE(CKWRONGDATATYPE) is signaled.
///
///  2)  If RECNO is less than one or greater than the number of
///      records in the specified segment, the error
///      SPICE(CKNONEXISTREC) is signaled.
///
///  3)  If the specified handle does not belong to any DAF file that
///      is currently known to be open, an error is signaled by a
///      routine in the call tree of this routine.
///
///  4)  If DESCR is not a valid descriptor of a segment in the CK
///      file specified by HANDLE, the results of this routine are
///      unpredictable.
///
///  5)  If the segment subtype is not recognized, the error
///      SPICE(NOTSUPPORTED) is signaled.
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
///  For a detailed description of the structure of a type 5 segment,
///  see the CK required reading.
///
///  This is a utility routine that may be used to read the individual
///  pointing instances that make up a type 5 segment. It is normally
///  used in conjunction with CKNR05, which gives the number of
///  pointing instances stored in a segment.
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
/// -    SPICELIB Version 1.1.0, 26-OCT-2021 (JDR) (NJB)
///
///         Edited the header to comply with NAIF standard.
///
///         Bug fix: added FAILED tests after all DAFGDA calls but the
///         last.
///
///         Added explicit conversion to INTEGER for SUBTYP.
///
/// -    SPICELIB Version 1.0.0, 27-AUG-2002 (NJB) (JML)
/// ```
pub fn ckgr05(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64],
    recno: i32,
    record: &mut [f64],
) -> crate::Result<()> {
    CKGR05(handle, descr, recno, record, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CKGR05 ( C-kernel, get record, type 05 )
pub fn CKGR05(
    HANDLE: i32,
    DESCR: &[f64],
    RECNO: i32,
    RECORD: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DESCR = DummyArray::new(DESCR, 1..);
    let mut RECORD = DummyArrayMut::new(RECORD, 1..);
    let mut ICD = StackArray::<i32, 6>::new(1..=NIC);
    let mut NREC: i32 = 0;
    let mut BEG: i32 = 0;
    let mut END: i32 = 0;
    let mut ADDR: i32 = 0;
    let mut PACKSZ: i32 = 0;
    let mut SUBTYP: i32 = 0;
    let mut DCD = StackArray::<f64, 2>::new(1..=NDC);
    let mut NPOINT: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    //    NDC        is the number of double precision components in an
    //               unpacked C-kernel segment descriptor.
    //
    //    NIC        is the number of integer components in an unpacked
    //               C-kernel segment descriptor.
    //
    //    DTYPE      is the data type of the segment that this routine
    //               operates on.
    //
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

    CHKIN(b"CKGR05", ctx)?;

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
    // From the descriptor, determine
    //
    //   1 - Is this really a type 5 segment?
    //   2 - The beginning address of the segment.
    //   3 - The number of pointing instances in the segment (it's the
    //       last word in the segment).
    //   4 - The existence of angular velocity data, which determines how
    //       big the pointing portion of the returned record will be.
    //
    DAFUS(
        DESCR.as_slice(),
        NDC,
        NIC,
        DCD.as_slice_mut(),
        ICD.as_slice_mut(),
    );

    if (ICD[3] != DTYPE) {
        SETMSG(
            b"Data type of the segment should be 5: Passed descriptor shows type = #.",
            ctx,
        );
        ERRINT(b"#", ICD[3], ctx);
        SIGERR(b"SPICE(CKWRONGDATATYPE)", ctx)?;
        CHKOUT(b"CKGR05", ctx)?;
        return Ok(());
    }

    //
    // Capture the segment's address range.
    //
    BEG = ICD[5];
    END = ICD[6];

    //
    // Read the subtype from the segment.
    //
    DAFGDA(HANDLE, (END - 3), (END - 3), RECORD.subarray_mut(2), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"CKGR05", ctx)?;
        return Ok(());
    }

    SUBTYP = intrinsics::IDNINT(RECORD[2]);

    if (SUBTYP == C05TP0) {
        PACKSZ = C05PS0;
    } else if (SUBTYP == C05TP1) {
        PACKSZ = C05PS1;
    } else if (SUBTYP == C05TP2) {
        PACKSZ = C05PS2;
    } else if (SUBTYP == C05TP3) {
        PACKSZ = C05PS3;
    } else {
        SETMSG(
            b"Unexpected CK type 5 subtype # found in type 5 segment.",
            ctx,
        );
        ERRINT(b"#", SUBTYP, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"CKGR05", ctx)?;
        return Ok(());
    }

    DAFGDA(HANDLE, END, END, std::slice::from_mut(&mut NPOINT), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"CKGR05", ctx)?;
        return Ok(());
    }

    NREC = intrinsics::IDNINT(NPOINT);

    //
    // If a request was made for a record which doesn't exist, then
    // signal an error and leave.
    //
    if ((RECNO < 1) || (RECNO > NREC)) {
        SETMSG(
            b"Requested record number (#) does not exist. There are # records in the segment.",
            ctx,
        );
        ERRINT(b"#", RECNO, ctx);
        ERRINT(b"#", NREC, ctx);
        SIGERR(b"SPICE(CKNONEXISTREC)", ctx)?;
        CHKOUT(b"CKGR05", ctx)?;
        return Ok(());
    }

    //
    // Get the pointing record indexed by RECNO.
    //
    ADDR = (BEG + (PACKSZ * (RECNO - 1)));

    DAFGDA(
        HANDLE,
        ADDR,
        ((ADDR + PACKSZ) - 1),
        RECORD.subarray_mut(3),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"CKGR05", ctx)?;
        return Ok(());
    }

    //
    // Next get the SCLK time.  Need to go past all of the NREC pointing
    // records (PACKSZ * NREC numbers), and then to the RECNOth SCLK
    // time.
    //
    ADDR = (((BEG + (PACKSZ * NREC)) + RECNO) - 1);

    DAFGDA(HANDLE, ADDR, ADDR, RECORD.subarray_mut(1), ctx)?;

    CHKOUT(b"CKGR05", ctx)?;
    Ok(())
}
