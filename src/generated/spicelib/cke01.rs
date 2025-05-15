//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// CK evaluate pointing record, data type 1
///
/// Evaluate a pointing record returned by CKR01 from a CK data type 1
/// segment. Return the C-matrix and optionally the angular velocity
/// vector associated with the time CLKOUT.
///
/// # Required Reading
///
/// * [CK](crate::required_reading::ck)
/// * [DAF](crate::required_reading::daf)
/// * [ROTATION](crate::required_reading::rotation)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NEEDAV     I   .TRUE. if angular velocity vector is required.
///  RECORD     I   Data type 1 pointing record.
///  CMAT       O   C-matrix.
///  AV         O   Angular velocity vector.
///  CLKOUT     O   Output spacecraft clock time.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NEEDAV   is .TRUE. when angular velocity data is requested.
///
///  RECORD   is a set of double precision numbers returned by CKR01
///           that contain sufficient information from a data type
///           1 pointing segment to evaluate the C-matrix and
///           possibly the angular velocity vector (if NEEDAV is
///           .TRUE.) for a particular instance.
///
///           The contents of RECORD are as follows:
///
///              RECORD( 1 ) = CLKOUT
///
///              RECORD( 2 ) = q0
///              RECORD( 3 ) = q1
///              RECORD( 4 ) = q2
///              RECORD( 5 ) = q3
///
///              RECORD( 6 ) = Av1  ]
///              RECORD( 7 ) = Av2  |-- Optional
///              RECORD( 8 ) = Av3  ]
///
///
///           The quantities q0 - q3 represent a quaternion.
///           The quantities Av1, Av2, and Av3 represent the angular
///           velocity vector.
///
///           CLKOUT is the encoded spacecraft clock time
///           associated with the quaternion and, optionally, the
///           angular velocity vector.
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
///           reference frame, then v has components x', y', z' in
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
///                         (Transpose of CMAT)
///
///  AV       is the angular velocity vector. This is returned only
///           if it has been requested, as indicated by NEEDAV. In
///           other words, if NEEDAV is .TRUE., the angular velocity
///           portion of RECORD must be present.
///
///           The angular velocity vector is the vector whose
///           direction gives the right-handed axis about which
///           the reference frame tied to the instrument is
///           instantaneously rotating at time CLKOUT.
///
///           The angular velocity vector is returned in component
///           form
///
///                    AV = [ AV1  , AV2  , AV3  ]
///
///           which is in terms of the reference coordinate frame
///           specified in the segment descriptor.
///
///           The magnitude of AV is the magnitude of the instantane-
///           ous velocity of the rotation, in radians per second.
///
///  CLKOUT   is the encoded spacecraft clock time associated with the
///           returned C-matrix and, optionally, the returned angular
///           velocity vector.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  No checking is done to determine whether RECORD is a valid
///      record.
///
///  2)  If NEEDAV is .TRUE., then RECORD is assumed to contain angular
///      velocity data. No checking is performed to verify this
///      assumption.
/// ```
///
/// # Particulars
///
/// ```text
///  For a detailed description of the structure of a type 1 pointing
///  segment, see the CK Required Reading file.
///
///  The only real work done by CKE01 is to convert the pointing
///  portion of the record from quaternion form to C-matrix form.
///
///  The angular velocity vector will only be returned if it has been
///  requested. In other words, if NEEDAV is .TRUE., the routine will
///  expect the angular velocity component of the record to be present.
/// ```
///
/// # Examples
///
/// ```text
///  A call to a CKEnn routine is almost always preceded by a call to
///  the corresponding CKRnn routine, which gets the logical record
///  that CKEnn evaluates.
///
///  The following code fragment searches through a file represented
///  by HANDLE for all segments applicable to the Voyager 2 wide angle
///  camera, for a particular spacecraft clock time, which have data
///  type 1. It then evaluates the pointing for that epoch and prints
///  the result.
///
///  C
///  C     - Get the spacecraft clock time. Must encode it for use
///  C       in the C-kernel.
///  C
///  C     - Set the time tolerance high to catch anything close to
///  C       the input time.
///  C
///  C     - We don't need angular velocity data.
///  C
///
///        SC     = -32
///        INST   = -32002
///        TOL    =  1000.D0
///        NEEDAV = .FALSE.
///        DTYPE  =  1
///  C
///  C     Load the Voyager 2 spacecraft clock kernel and the C-kernel.
///  C
///        CALL FURNSH ( 'VGR_SCLK.TSC'        )
///        CALL DAFOPR ( 'VGR2_CK.BC',  HANDLE )
///  C
///  C     Convert the input request time to ticks.
///  C
///        WRITE (*,*) 'Enter spacecraft clock time string:'
///        READ (*,FMT='(A)') SCLKCH
///        CALL SCENCD ( SC, SCLKCH, SCLKDP )
///
///  C
///  C     Search from the beginning through all segments.
///  C
///        CALL DAFBFS ( HANDLE )
///        CALL DAFFNA ( SFND   )
///
///        DO WHILE ( SFND )
///
///           CALL DAFGN ( IDENT                 )
///           CALL DAFGS ( DESCR                 )
///           CALL DAFUS ( DESCR, 2, 6, DCD, ICD )
///
///           IF (        INST          .EQ. ICD( 1 )
///       .               DTYPE         .EQ. ICD( 3 )
///       .        .AND.  SCLKDP + TOL  .GE. DCD( 1 )
///       .        .AND.  SCLKDP - TOL  .LE. DCD( 2 )  ) THEN
///
///              CALL CKR01 ( HANDLE, DESCR, SCLKDP, TOL, NEEDAV,
///       .                   RECORD, FOUND )
///
///              IF ( FOUND ) THEN
///
///                 CALL CKE01 ( NEEDAV, RECORD, CMAT, AV, CLKOUT )
///
///                 WRITE (*,*) 'Segment descriptor and identifier:'
///                 WRITE (*,*) DCD, ICD
///                 WRITE (*,*) IDENT
///
///                 WRITE (*,*) 'C-matrix:'
///                 WRITE (*,*) CMAT
///
///              END IF
///
///           END IF
///
///           CALL DAFFNA ( SFND )
///
///        END DO
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  J.M. Lynch         (JPL)
///  M.J. Spencer       (JPL)
///  W.L. Taber         (JPL)
///  R.E. Thurman       (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.3.0, 12-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.2.1, 22-AUG-2006 (EDW)
///
///         Replaced header references to LDPOOL with references
///         to FURNSH.
///
/// -    SPICELIB Version 1.2.0, 14-NOV-1995 (WLT)
///
///         Changed "inertial frame" to simply reference frame to
///         reflect new capabilities of the SPICE system.
///
/// -    SPICELIB Version 1.1.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.1.0, 30-AUG-1991 (MJS) (JML)
///
///         1) Previously, in the standard SPICE error handling, the
///            logical function RETURN was not written as a function;
///            it is now written as a function.
///
///         2) The example program was changed so that the tolerance
///            and data type are used in selecting which segments to read.
///
///         3) It was specified that the angular velocity vector
///            gives the right-handed axis about which the instrument
///            frame rotates.
///
/// -    SPICELIB Version 1.0.1, 02-NOV-1990 (JML)
///
///         The example program was corrected so that the input
///         instrument code was tested against ICD(1) instead of
///         ICD(3).
///
/// -    SPICELIB Version 1.0.0, 07-SEP-1990 (RET) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.2.0, 14-NOV-1995 (WLT)
///
///         Changed "inertial frame" to simply reference frame to
///         reflect new capabilities of the SPICE system.
///
///         This change affects only documentation not code.
///
/// -    SPICELIB Version 1.1.0, 30-AUG-1991 (MJS) (JML)
///
///         1) In the standard SPICE error handling, the line:
///
///               IF ( RETURN ) THEN
///
///            was changed to
///
///               IF ( RETURN() ) THEN
///
///         2) The example program was changed so that the tolerance
///            and data type are used in selecting  which segments to read.
///
///         3) It was specified that the angular velocity vector
///            gives the right-handed axis about which the instrument
///            frame rotates.
///
/// -    SPICELIB Version 1.0.1, 02-NOV-1990 (JML)
///
///         1) The example program was corrected so that the input
///            instrument code was tested against ICD(1) instead of
///            ICD(3).
///         2) SCLK was removed from the Required Reading section.
///
/// -    Beta Version 1.1.0, 29-AUG-1990 (MJS) (JEM)
///
///         The following changes were made as a result of the
///         NAIF CK Code and Documentation Review:
///
///         1) The argument SCLK was removed from the calling sequence.
///         2) Header was updated.
///         3) The call to the routine QUAT2M_3 was replaced by a call to
///            the routine Q2M.
///
/// -    Beta Version 1.0.0, 18-MAY-1990 (RET) (IMU)
/// ```
pub fn cke01(
    ctx: &mut SpiceContext,
    needav: bool,
    record: &[f64],
    cmat: &mut [[f64; 3]; 3],
    av: &mut [f64; 3],
    clkout: &mut f64,
) -> crate::Result<()> {
    CKE01(
        needav,
        record,
        cmat.as_flattened_mut(),
        av,
        clkout,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CKE01 ( CK evaluate pointing record, data type 1 )
pub fn CKE01(
    NEEDAV: bool,
    RECORD: &[f64],
    CMAT: &mut [f64],
    AV: &mut [f64],
    CLKOUT: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let RECORD = DummyArray::new(RECORD, 1..);
    let mut CMAT = DummyArrayMut2D::new(CMAT, 1..=3, 1..=3);
    let mut AV = DummyArrayMut::new(AV, 1..=3);

    //
    // SPICELIB functions
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"CKE01", ctx)?;
    }

    //
    // Dissect the record.
    //
    *CLKOUT = RECORD[1];

    Q2M(RECORD.subarray(2), CMAT.as_slice_mut());

    if NEEDAV {
        AV[1] = RECORD[6];
        AV[2] = RECORD[7];
        AV[3] = RECORD[8];
    }

    CHKOUT(b"CKE01", ctx)?;
    Ok(())
}
