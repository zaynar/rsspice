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

/// C-kernel, number of records, data type 4
///
/// Return the number of pointing records in a CK type 04 segment.
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
///  DESCR      I   The descriptor of the type 4 segment.
///  NREC       O   The number of pointing records in the segment.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of the binary CK file containing the
///           segment. The file should have been opened for read
///           or write access, either by CKLPF, DAFOPR, or DAFOPW.
///
///  DESCR    is the packed descriptor of a data type 4 segment.
/// ```
///
/// # Detailed Output
///
/// ```text
///  NREC     is the number of pointing records in the type 4
///           segment.
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
///  1)  If the segment indicated by DESCR is not a type 4 segment,
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
///  For a complete description of the internal structure of a type 4
///  segment, see the CK required reading.
///
///  This routine returns the number of pointing records contained
///  in the specified segment. It is normally used in conjunction
///  with CKGR04 which returns the Ith pointing record in the
///  segment.
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
///  C
///  C     $Declarations.
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
pub fn cknr04(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64],
    nrec: &mut i32,
) -> crate::Result<()> {
    CKNR04(handle, descr, nrec, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CKNR04 ( C-kernel, number of records, data type 4 )
pub fn CKNR04(
    HANDLE: i32,
    DESCR: &[f64],
    NREC: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DESCR = DummyArray::new(DESCR, 1..);
    let mut DCD = StackArray::<f64, 2>::new(1..=NDC);
    let mut ICD = StackArray::<i32, 6>::new(1..=NIC);

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
    } else {
        CHKIN(b"CKNR04", ctx)?;
    }

    //
    // Check whether our segment is of the type 4 by unpacking
    // descriptor and checking value of its third integer component.
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
            b"Data type of the segment should be 4: Passed descriptor shows type = #.",
            ctx,
        );
        ERRINT(b"#", ICD[3], ctx);
        SIGERR(b"SPICE(CKWRONGDATATYPE)", ctx)?;
        CHKOUT(b"CKNR04", ctx)?;
        return Ok(());
    }

    //
    // The number of records (packets) can be obtained by a call to
    // SGMETA. This number is a meta item 12 (see sgparam.inc for
    // details.)
    //
    SGMETA(HANDLE, DESCR.as_slice(), 12, NREC, ctx)?;

    //
    // All done.
    //
    CHKOUT(b"CKNR04", ctx)?;

    Ok(())
}
