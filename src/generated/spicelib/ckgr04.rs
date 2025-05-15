//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const QSIZ: i32 = 4;
const QAVSIZ: i32 = 7;
const CK1DTP: i32 = 1;
const CK1RSZ: i32 = 8;
const CK2DTP: i32 = 2;
const CK2RSZ: i32 = 10;
const CK3DTP: i32 = 3;
const CK3RSZ: i32 = 17;
const CK4DTP: i32 = 4;
const CK4PCD: f64 = 128.0;
const CK4MXD: i32 = 18;
const CK4SFT: i32 = 10;
const CK4RSZ: i32 = (((CK4MXD + 1) * QAVSIZ) + CK4SFT);
const CK5DTP: i32 = 5;
const CK5MXD: i32 = 23;
const CK5MET: i32 = 4;
const CK5MXP: i32 = 14;
const CK5RSZ: i32 = (((CK5MXD + 1) * CK5MXP) + CK5MET);
const CK6DTP: i32 = 6;
const CK6MXD: i32 = 23;
const CK6MET: i32 = 4;
const CK6PS3: i32 = 7;
const CK6RSZ: i32 = (((CK6MXD + 1) * (CK6PS3 + 1)) + CK6MET);
const CKMRSZ: i32 = CK5RSZ;
const NDC: i32 = 2;
const NIC: i32 = 6;
const SHFTAD: i32 = (CK4SFT - 1);

/// C-kernel, get record, type 04
///
/// Return a specified pointing record from a CK type 04 segment.
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
///  RECNO      I   The number of the pointing record to be returned.
///  RECORD     O   The pointing record.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of the binary CK file containing the
///           desired segment. The file should have been opened
///           for read or write access, either by CKLPF, DAFOPR,
///           or DAFOPW.
///
///  DESCR    is the packed descriptor of the data type 4 segment.
///
///  RECNO    is the number of the pointing record to be returned
///           from the data type 4 segment.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RECORD   is the pointing record indexed by RECNO in the
///           segment. The contents of the record are as follows:
///
///           ---------------------------------------------------
///           | The midpoint of the approximation interval      |
///           ---------------------------------------------------
///           | The radius of the approximation interval        |
///           ---------------------------------------------------
///           | Number of coefficients for q0                   |
///           ---------------------------------------------------
///           | Number of coefficients for q1                   |
///           ---------------------------------------------------
///           | Number of coefficients for q2                   |
///           ---------------------------------------------------
///           | Number of coefficients for q3                   |
///           ---------------------------------------------------
///           | Number of coefficients for AV1                  |
///           ---------------------------------------------------
///           | Number of coefficients for AV2                  |
///           ---------------------------------------------------
///           | Number of coefficients for AV3                  |
///           ---------------------------------------------------
///           | q0 Cheby coefficients                           |
///           ---------------------------------------------------
///           | q1 Cheby coefficients                           |
///           ---------------------------------------------------
///           | q2 Cheby coefficients                           |
///           ---------------------------------------------------
///           | q3 Cheby coefficients                           |
///           ---------------------------------------------------
///           | AV1 Cheby coefficients (optional)               |
///           ---------------------------------------------------
///           | AV2 Cheby coefficients (optional)               |
///           ---------------------------------------------------
///           | AV3 Cheby coefficients (optional)               |
///           ---------------------------------------------------
/// ```
///
/// # Parameters
///
/// ```text
///  See 'ckparam.inc'.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the segment is not of data type 4, the error
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
///  For a detailed description of the structure of a type 4 segment,
///  see the CK required reading.
///
///  This is a utility routine that may be used to read the individual
///  pointing records that make up a type 4 segment. It is normally
///  used in conjunction with CKNR04, which gives the number of
///  pointing records stored in a segment.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that DATA.BC is a CK file that contains segments of
///  data type 4. Then the following code fragment extracts the
///  data packets contained in the segment.
///
///  C
///  C     CK parameters include file.
///  C
///        INCLUDE               'ckparam.inc'
///
///  C
///  C     Local variables.
///  C
///        DOUBLE PRECISION      DCD    ( 2 )
///        DOUBLE PRECISION      DESCR  ( 5 )
///        DOUBLE PRECISION      PKTDAT ( CK4RSZ )
///
///        INTEGER               AVFLAG
///        INTEGER               HANDLE
///        INTEGER               I
///        INTEGER               ICD    ( 6 )
///        INTEGER               K
///        INTEGER               LASTAD
///        INTEGER               NCOEF  ( QAVSIZ )
///        INTEGER               NREC
///
///        LOGICAL               FOUND
///  C
///  C     First load the file. (The file may also be opened by using
///  C     CKLPF.)
///  C
///        CALL DAFOPR ( 'DATA.BC', HANDLE )
///  C
///  C     Begin forward search. Find the first array.
///  C
///        CALL DAFBFS ( HANDLE )
///        CALL DAFFNA ( FOUND  )
///  C
///  C     Get segment descriptor.
///  C
///        CALL DAFGS ( DESCR )
///  C
///  C     Unpack the segment descriptor into its double precision
///  C     and integer components.
///  C
///        CALL DAFUS ( DESCR, 2, 6, DCD, ICD )
///
///        IF ( ICD( 3 ) .EQ. 4 ) THEN
///  C
///  C        How many records does this segment contain?
///  C
///           CALL CKNR04 ( HANDLE, DESCR, NREC )
///
///           DO I = 1, NREC
///  C
///  C           Get the data records stored in the segment.
///  C
///              CALL CKGR04 ( HANDLE, DESCR, I, PKTDAT )
///  C
///  C           Print data packet contents. Print coverage interval
///  C           midpoint & radii first.
///  C
///              WRITE (2,*) PKTDAT (1)
///              WRITE (2,*) PKTDAT (2)
///  C
///  C           Decode numbers of coefficients.
///  C
///              CALL ZZCK4D2I ( PKTDAT(3), QAVSIZ, CK4PCD, NCOEF )
///  C
///  C           Print number of coefficients for Q0, Q1, Q2 and Q3.
///  C
///              WRITE (2,FMT='(I2,6X,I2)') NCOEF( 1 ), NCOEF( 2 )
///              WRITE (2,FMT='(I2,6X,I2)') NCOEF( 3 ), NCOEF( 4 )
///  C
///  C           Print number coefficients for AV1, AV2 and AV3.
///  C
///              WRITE (2,FMT='(I2,6X,I2)') NCOEF( 5 ), NCOEF( 6 )
///              WRITE (2,FMT='(I2,6X,I2)') NCOEF( 7 )
///  C
///  C           Print Cheby coefficients.
///  C
///              LASTAD = 0
///
///              DO K = 1, QAVSIZ
///                 LASTAD = LASTAD + NCOEF( K )
///              END DO
///
///              DO K = 4, LASTAD + 4
///                 WRITE (2,*) PKTDAT (K)
///              END DO
///
///           END DO
///
///        END IF
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The binary CK file containing the segment whose descriptor
///      was passed to this routine must be opened for read or write
///      access by either CKLPF, DAFOPR, or DAFOPW.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  Y.K. Zaiko         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.2, 26-OCT-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.1, 18-APR-2014 (BVS)
///
///         Minor header edits.
///
/// -    SPICELIB Version 1.0.0, 05-MAY-1999 (YKZ) (BVS)
/// ```
pub fn ckgr04(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64],
    recno: i32,
    record: &mut [f64],
) -> crate::Result<()> {
    CKGR04(handle, descr, recno, record, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CKGR04 ( C-kernel, get record, type 04 )
pub fn CKGR04(
    HANDLE: i32,
    DESCR: &[f64],
    RECNO: i32,
    RECORD: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DESCR = DummyArray::new(DESCR, 1..);
    let mut RECORD = DummyArrayMut::new(RECORD, 1..);
    let mut DCD = StackArray::<f64, 2>::new(1..=NDC);
    let mut ENDS = StackArray::<i32, 1>::new(1..=1);
    let mut ICD = StackArray::<i32, 6>::new(1..=NIC);
    let mut NREC: i32 = 0;
    let mut NUMALL: i32 = 0;
    let mut NUMCFT = StackArray::<i32, 7>::new(1..=QAVSIZ);

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Length (in DPs) of non-coefficient front part of RECORD when
    // it contains decoded numbers of coefficients. It is one less
    // than the length of the same part in a record exchanged between
    // CKR04 and CKE04 because it doesn't contain time at which
    // pointing has to be evaluated.
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
        CHKIN(b"CKGR04", ctx)?;
    }

    //
    // Unpack descriptor and check segment data type. Signal an error
    // if it's not 4.
    //
    DAFUS(
        DESCR.as_slice(),
        NDC,
        NIC,
        DCD.as_slice_mut(),
        ICD.as_slice_mut(),
    );

    if (ICD[3] != CK4DTP) {
        SETMSG(
            b"Data type of the segment should be 4: Passed  descriptor shows type = #.",
            ctx,
        );
        ERRINT(b"#", ICD[3], ctx);
        SIGERR(b"SPICE(CKWRONGDATATYPE)", ctx)?;
        CHKOUT(b"CKGR04", ctx)?;
        return Ok(());
    }

    //
    // If a request was made for a data record which doesn't
    // exist, then signal an error and leave.
    //
    CKNR04(HANDLE, DESCR.as_slice(), &mut NREC, ctx)?;

    if ((RECNO < 1) || (RECNO > NREC)) {
        SETMSG(
            b"Requested record number (#) does not exist. There are # records in the segment.",
            ctx,
        );
        ERRINT(b"#", RECNO, ctx);
        ERRINT(b"#", NREC, ctx);
        SIGERR(b"SPICE(CKNONEXISTREC)", ctx)?;
        CHKOUT(b"CKGR04", ctx)?;
        return Ok(());
    }

    //
    // Get the data record indexed by RECNO.
    //
    SGFPKT(
        HANDLE,
        DESCR.as_slice(),
        RECNO,
        RECNO,
        RECORD.as_slice_mut(),
        ENDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Decode 7 numbers of coefficients from double precision value.
    //
    ZZCK4D2I(&mut RECORD[3], QAVSIZ, CK4PCD, NUMCFT.as_slice_mut());

    //
    // Compute total number of coefficients in the fetched packet.
    //
    NUMALL = 0;

    for K in 1..=QAVSIZ {
        NUMALL = (NUMALL + NUMCFT[K]);
    }

    //
    // Move polynomial coefficients to the right to free space for
    // decoded numbers of coefficients and insert these numbers
    // starting from the third position.
    //
    for K in intrinsics::range(NUMALL, 1, -1) {
        RECORD[(K + SHFTAD)] = RECORD[(K + 3)];
    }

    for K in 1..=7 {
        RECORD[(K + 2)] = NUMCFT[K] as f64;
    }

    //
    // All done.
    //
    CHKOUT(b"CKGR04", ctx)?;

    Ok(())
}
