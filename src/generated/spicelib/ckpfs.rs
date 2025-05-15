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

/// C-kernel, get pointing from segment
///
/// Evaluate pointing data from a segment for a given time.
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
///  HANDLE     I   CK file handle.
///  DESCR      I   Segment descriptor.
///  SCLKDP     I   Spacecraft clock time.
///  TOL        I   Time tolerance.
///  NEEDAV     I   .TRUE. when angular velocity data is requested.
///  CMAT       O   C-matrix.
///  AV         O   Angular velocity vector.
///  CLKOUT     O   Output spacecraft clock time.
///  FOUND      O   .TRUE. when requested pointing is available.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of the binary CK file containing the
///           desired segment. The file should have been opened
///           for read access, either by CKLPF or DAFOPR.
///
///  DESCR    is the packed descriptor of the segment.
///
///  SCLKDP   is the encoded spacecraft clock time for which
///           pointing is desired.
///
///  TOL      is a time tolerance, measured in the same units as
///           encoded spacecraft clock. The C-matrix returned by
///           CKPFS is the one whose time is closest to SCLKDP and
///           within TOL units of SCLKDP.
///
///  NEEDAV   is .TRUE. when angular velocity data is requested.
/// ```
///
/// # Detailed Output
///
/// ```text
///  CMAT     is a rotation matrix that transforms the components of
///           of a vector expressed in the reference frame given in
///           the segment to components expressed in the instrument
///           fixed frame at time CLKOUT.
///
///           Thus, if a vector v has components x, y, z in the
///           CK base frame, then v has components x', y', z' in
///           the instrument fixed frame at time CLKOUT:
///
///                [ x' ]     [          ] [ x ]
///                | y' |  =  |   CMAT   | | y |
///                [ z' ]     [          ] [ z ]
///
///           If the x', y', z' components are known, use the
///           transpose of the C-matrix to determine x, y, z as
///           follows.
///
///                [ x ]      [          ]T    [ x' ]
///                | y |  =   |   CMAT   |     | y' |
///                [ z ]      [          ]     [ z' ]
///                        (Transpose of CMAT)
///
///  AV       is the angular velocity vector. This is returned only
///           if it has been requested, as indicated by NEEDAV. In
///           other words, if NEEDAV is .TRUE., then the pointing
///           records in the segment must contain AV data.
///
///           The angular velocity vector is the right-handed axis
///           about which the reference frame tied to the instrument
///           is instantaneously rotating at time CLKOUT. The
///           magnitude of AV is the magnitude of the instantaneous
///           velocity of the rotation, in radians per second.
///
///           The components of AV are given relative to the
///           reference frame specified in the segment descriptor.
///
///  CLKOUT   is the encoded spacecraft clock time associated with
///           the returned C-matrix and, optionally, the returned
///           angular velocity vector.
///
///  FOUND    is .TRUE. if a C-matrix and an angular velocity vector
///           (if requested) were found to satisfy the pointing
///           request. FOUND will be false otherwise.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the data type of the segment is not one of those supported
///      by this routine, the error SPICE(CKUNKNOWNDATATYPE) is
///      signaled.
///
///  2)  If the specified handle does not belong to any file that is
///      currently known to be open, an error is signaled by a routine
///      in the call tree of this routine.
///
///  3)  If DESCR is not a valid, packed descriptor of a segment in
///      the CK file specified by HANDLE, the results of this routine
///      are unpredictable.
///
///  4)  If TOL is negative, FOUND is .FALSE.
///
///  5)  If NEEDAV is .TRUE., but the segment doesn't contain AV data,
///      an error is signaled by a routine in the call tree of this
///      routine.
/// ```
///
/// # Particulars
///
/// ```text
///  The structure of this routine is just a big case statement. Each
///  segment data type is supported by two routines:
///
///     CKRnn   which reads a single logical pointing record from a
///             segment of type nn. (A logical record is defined as
///             a collection of numbers sufficient to determine the
///             C-matrix, and optionally the angular velocity vector,
///             at the input time.)
///
///     CKEnn   which evaluates the pointing record returned by CKRnn
///             to give the C-matrix and optionally the angular
///             velocity vector at the input time.
///
///  The data type is determined from the segment descriptor, and the
///  appropriate routines are called.
/// ```
///
/// # Examples
///
/// ```text
///  CKPFS allows you to be more selective than CKGP or CKGPAV about
///  choosing segments to satisfy CK pointing requests.
///
///  Suppose MOC.BC is a CK file consisting of several segments
///  containing Mars Observer Camera pointing data. Each segment
///  covers the same time period, but produces different pointing
///  values (one segment may contain predict values, another may
///  contain telemetry-based values, and others may contain different
///  corrected versions).
///
///  The following code fragment shows how different the results are
///  for each segment. The program steps through the file segment by
///  segment and requests pointing for the same time from each
///  segment. The results are printed to the screen.
///
///  GETIME is an imaginary routine used to get an encoded SCLK time
///  (SCLKDP) and time tolerance from the user.
///
///        SC     = -94
///        INST   = -94001
///        NEEDAV = .TRUE.
///
///        CALL CKLPF ( 'MOC.BC', HANDLE )
///
///        CALL GETIME ( SCLKDP, TOL, QUIT )
///
///  C
///  C     For each time, begin a forward search through the file, and
///  C     for each segment found, get its descriptor, identifier, and
///  C     evaluate the pointing.
///  C
///        DO WHILE ( .NOT. QUIT )
///
///           CALL DAFBFS ( HANDLE )
///           CALL DAFFNA ( FOUND  )
///
///           DO WHILE ( FOUND )
///
///              CALL DAFGS ( DESCR )
///              CALL DAFGN ( IDENT )
///
///              CALL CKPFS ( HANDLE, DESCR, SCLKDP, TOL,   NEEDAV,
///       .                   CMAT,   AV,    CLKOUT, PFOUND         )
///
///              IF ( PFOUND ) THEN
///                 WRITE (*,*) 'Segment:          ', IDENT
///                 WRITE (*,*) 'C-Matrix:         ', CMAT
///                 WRITE (*,*) 'Angular velocity: ', AV
///
///              ELSE
///                 CALL SCDECD ( SC, SCLKDP, SCLKCH )
///                 WRITE (*,*) 'Data not found at time ', SCLKCH
///
///              END IF
///
///              CALL DAFFNA ( FOUND )
///
///           END DO
///
///           CALL GETIME ( SCLKDP, TOL, QUIT )
///
///        END DO
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  A C-kernel file should have been loaded by either CKLPF
///      or DAFOPR.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  J.M. Lynch         (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  R.E. Thurman       (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 6.1.0, 12-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 6.0.0, 24-MAR-2014 (NJB)
///
///         Bug fix: this routine now sets the output FOUND to
///         .FALSE. if a SPICE error is detected.
///
///         The routine was updated to handle data type 6 segments.
///         Several comment typos were corrected.
///
/// -    SPICELIB Version 5.0.0, 19-AUG-2002 (NJB)
///
///         The routine was updated to handle data type 5 segments.
///
/// -    SPICELIB Version 4.0.0, 02-MAY-1999 (BVS)
///
///         The routine was updated to handle data type 4 segments.
///         The RECSIZ size parameter was eliminated. The dimension
///         of the RECORD buffer is now defined by the CKMRSZ parameter
///         specified in the 'ckparam.inc' include file.
///
/// -    SPICELIB Version 3.0.0, 11-SEP-1992 (JML)
///
///         The routine was updated to handle data type 3 segments.
///
/// -    SPICELIB Version 2.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 2.0.0, 30-AUG-1991 (JML)
///
///          The routine was updated to handle data type 2 segments.
///
///          FOUND is now initialized to false.
///
/// -    SPICELIB Version 1.0.1, 02-NOV-1990 (JML)
///
///          The restriction that a C-kernel file must be loaded
///          was explicitly stated.
///
///
/// -    SPICELIB Version 1.0.0, 07-SEP-1990 (RET) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 6.0.0, 24-MAR-2014 (NJB)
///
///         The routine was updated to handle data type 6 segments.
///
/// -    SPICELIB Version 5.0.0, 19-AUG-2002 (NJB)
///
///         The routine was updated to handle data type 5 segments.
///
/// -    SPICELIB Version 4.0.0, 02-MAY-1999 (BVS)
///
///         The routine was updated to handle data type 4 segments.
///
///            a) 'ckparam.inc' include file was included.
///
///            b) RECSIZ size parameter was eliminated.
///
///            c) Size of the RECORD was reset to CKMRSZ, parameter
///               defined in the 'ckparam.inc' include file.
///
///            d) Calls to CKR04 and CKE04 were added to the case
///               statement.
///
/// -    SPICELIB Version 3.0.0, 11-SEP-1992 (JML)
///
///         The routine was updated to handle data type 3 segments.
///
///            a) RECSIZ was increased to 17.
///
///            b) Calls to CKR03 and CKE03 were added to the case
///               statement.
///
/// -    SPICELIB Version 2.0.0, 30-AUG-1991 (JML)
///
///         1) The routine was updated to handle data type 2 segments.
///
///         2) FOUND is initialized to false to guard against it being
///            left unchanged from its previous value when an error is
///            detected.
///
/// -    SPICELIB Version 1.0.1, 02-NOV-1990 (JML)
///
///         1) The restriction that a C-kernel file must be loaded
///            was explicitly stated.
///
/// -    Beta Version 1.1.0, 30-AUG-1990 (MJS)
///
///         The following changes were made as a result of the
///         NAIF CK Code and Documentation Review:
///
///         1) The variable SCLK was changed to SCLKDP.
///         2) The declarations for the parameters RECSIZ, NDC, and NIC
///            were moved from the "Declarations" section of the header
///            to the "Local parameters" section of the code below the
///            header. These parameters are not meant to modified by
///            users.
///         3) The header was updated.
///         4) The comments in the code were improved.
///
/// -    Beta Version 1.0.0, 07-MAY-1990 (RET) (IMU)
/// ```
pub fn ckpfs(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64],
    sclkdp: f64,
    tol: f64,
    needav: bool,
    cmat: &mut [[f64; 3]; 3],
    av: &mut [f64; 3],
    clkout: &mut f64,
    found: &mut bool,
) -> crate::Result<()> {
    CKPFS(
        handle,
        descr,
        sclkdp,
        tol,
        needav,
        cmat.as_flattened_mut(),
        av,
        clkout,
        found,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CKPFS ( C-kernel, get pointing from segment )
pub fn CKPFS(
    HANDLE: i32,
    DESCR: &[f64],
    SCLKDP: f64,
    TOL: f64,
    NEEDAV: bool,
    CMAT: &mut [f64],
    AV: &mut [f64],
    CLKOUT: &mut f64,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DESCR = DummyArray::new(DESCR, 1..);
    let mut CMAT = DummyArrayMut2D::new(CMAT, 1..=3, 1..=3);
    let mut AV = DummyArrayMut::new(AV, 1..=3);
    let mut ICD = StackArray::<i32, 6>::new(1..=NIC);
    let mut TYPE: i32 = 0;
    let mut DCD = StackArray::<f64, 2>::new(1..=NDC);
    let mut RECORD = ActualArray::<f64>::new(1..=CKMRSZ);

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

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"CKPFS", ctx)?;
    }

    //
    // Start off with FOUND set to false.
    //
    *FOUND = false;

    //
    // Upgrading CKPFS to accommodate new data types involves following
    // these steps:
    //
    // 1)  Write the two new routines CKRnn and CKEnn. (You may need to
    //     add or subtract from the arguments used in the existing CKRnn
    //     and CKEnn calling sequences, but should not have to change
    //     the inputs or outputs to CKPFS.)
    //
    // 2)  Insert a new case into the code of CKPFS.
    //
    // 3)  Depending on the size of RECORD returned from CKRnn, modify
    //     the parameter RECSIZ.  (You will only need to change it if
    //     RECSIZ is not large enough for the new CKRnn's RECORD.)
    //

    //
    // Unpack the descriptor to see what the data type of the segment is,
    // and call the appropriate read-and-evaluate routines.
    //
    DAFUS(
        DESCR.as_slice(),
        NDC,
        NIC,
        DCD.as_slice_mut(),
        ICD.as_slice_mut(),
    );

    TYPE = ICD[3];

    if (TYPE == 1) {
        CKR01(
            HANDLE,
            DESCR.as_slice(),
            SCLKDP,
            TOL,
            NEEDAV,
            RECORD.as_slice_mut(),
            FOUND,
            ctx,
        )?;

        if *FOUND {
            CKE01(
                NEEDAV,
                RECORD.as_slice(),
                CMAT.as_slice_mut(),
                AV.as_slice_mut(),
                CLKOUT,
                ctx,
            )?;
        }
    } else if (TYPE == 2) {
        CKR02(
            HANDLE,
            DESCR.as_slice(),
            SCLKDP,
            TOL,
            RECORD.as_slice_mut(),
            FOUND,
            ctx,
        )?;

        if *FOUND {
            CKE02(
                NEEDAV,
                RECORD.as_slice(),
                CMAT.as_slice_mut(),
                AV.as_slice_mut(),
                CLKOUT,
                ctx,
            )?;
        }
    } else if (TYPE == 3) {
        CKR03(
            HANDLE,
            DESCR.as_slice(),
            SCLKDP,
            TOL,
            NEEDAV,
            RECORD.as_slice_mut(),
            FOUND,
            ctx,
        )?;

        if *FOUND {
            CKE03(
                NEEDAV,
                RECORD.as_slice(),
                CMAT.as_slice_mut(),
                AV.as_slice_mut(),
                CLKOUT,
                ctx,
            )?;
        }
    } else if (TYPE == 4) {
        CKR04(
            HANDLE,
            DESCR.as_slice(),
            SCLKDP,
            TOL,
            NEEDAV,
            RECORD.as_slice_mut(),
            FOUND,
            ctx,
        )?;

        if *FOUND {
            CKE04(
                NEEDAV,
                RECORD.as_slice(),
                CMAT.as_slice_mut(),
                AV.as_slice_mut(),
                CLKOUT,
            );
        }
    } else if (TYPE == 5) {
        CKR05(
            HANDLE,
            DESCR.as_slice(),
            SCLKDP,
            TOL,
            NEEDAV,
            RECORD.as_slice_mut(),
            FOUND,
            ctx,
        )?;

        if *FOUND {
            CKE05(
                NEEDAV,
                RECORD.as_slice_mut(),
                CMAT.as_slice_mut(),
                AV.as_slice_mut(),
                CLKOUT,
                ctx,
            )?;
        }
    } else if (TYPE == 6) {
        CKR06(
            HANDLE,
            DESCR.as_slice(),
            SCLKDP,
            TOL,
            NEEDAV,
            RECORD.as_slice_mut(),
            FOUND,
            ctx,
        )?;

        if *FOUND {
            CKE06(
                NEEDAV,
                RECORD.as_slice_mut(),
                CMAT.as_slice_mut(),
                AV.as_slice_mut(),
                CLKOUT,
                ctx,
            )?;
        }
    } else {
        SETMSG(b"The data type # is not currently supported.", ctx);
        ERRINT(b"#", TYPE, ctx);
        SIGERR(b"SPICE(CKUNKNOWNDATATYPE)", ctx)?;
    }

    //
    // In case an evaluator signaled an error, we check the SPICE
    // error status here. If a SPICE error occurred, indicate no
    // data were found.
    //
    if FAILED(ctx) {
        *FOUND = false;
    }

    CHKOUT(b"CKPFS", ctx)?;
    Ok(())
}
