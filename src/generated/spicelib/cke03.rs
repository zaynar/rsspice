//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// C-kernel, evaluate pointing record, data type 3
///
/// Evaluate a pointing record returned by CKR03 from a CK type 3
/// segment. Return the C-matrix and angular velocity vector
/// associated with the time CLKOUT.
///
/// # Required Reading
///
/// * [CK](crate::required_reading::ck)
/// * [ROTATION](crate::required_reading::rotation)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NEEDAV     I   .TRUE. if angular velocity is requested.
///  RECORD     I   Data type 3 pointing record.
///  CMAT       O   C-matrix.
///  AV         O   Angular velocity vector.
///  CLKOUT     O   SCLK associated with C-matrix.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NEEDAV   is .TRUE. if angular velocity is requested.
///
///  RECORD   is a set of double precision numbers returned by CKR03
///           that contain sufficient information from a type 3 CK
///           segment to evaluate the C-matrix and the angular
///           velocity vector at a particular time. Depending on
///           the contents of RECORD, this routine will either
///           interpolate between two pointing instances that
///           bracket a request time, or it will simply return the
///           pointing given by a single pointing instance.
///
///           When pointing at the request time can be determined
///           by linearly interpolating between the two pointing
///           instances that bracket that time, the bracketing
///           pointing instances are returned in RECORD as follows:
///
///              RECORD( 1  ) = Left bracketing SCLK time.
///
///              RECORD( 2  ) = lq0  \
///              RECORD( 3  ) = lq1   \    Left bracketing
///              RECORD( 4  ) = lq2   /      quaternion.
///              RECORD( 5  ) = lq3  /
///
///              RECORD( 6  ) = lav1 \     Left bracketing
///              RECORD( 7  ) = lav2  |    angular velocity
///              RECORD( 8  ) = lav3 /       ( optional )
///
///              RECORD( 9  ) = Right bracketing SCLK time.
///
///              RECORD( 10 ) = rq0  \
///              RECORD( 11 ) = rq1   \    Right bracketing
///              RECORD( 12 ) = rq2   /       quaternion.
///              RECORD( 13 ) = rq3  /
///
///              RECORD( 14 ) = rav1 \     Right bracketing
///              RECORD( 15 ) = rav2  |    angular velocity
///              RECORD( 16 ) = rav3 /       ( optional )
///
///              RECORD( 17 ) = pointing request time
///
///           The quantities lq0 - lq3 and rq0 - rq3 are the
///           components of the quaternions that represent the
///           C-matrices associated with the times that bracket
///           the requested time.
///
///           The quantities lav1, lav2, lav3 and rav1, rav2, rav3
///           are the components of the angular velocity vectors at
///           the respective bracketing times. The components of the
///           angular velocity vectors are specified relative to the
///           inertial reference frame of the segment.
///
///           When the routine is to simply return the pointing
///           given by a particular pointing instance, then the
///           values of that pointing instance are returned in both
///           parts of RECORD ( i.e. RECORD(1-9) and RECORD(10-16) ).
/// ```
///
/// # Detailed Output
///
/// ```text
///  CMAT     is a rotation matrix that transforms the components
///           of a vector expressed in the inertial frame given in
///           the segment to components expressed in the instrument
///           fixed frame at the returned time.
///
///           Thus, if a vector v has components x, y, z in the
///           inertial frame, then v has components x', y', z' in the
///           instrument fixed frame where:
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
///  AV       is the angular velocity vector of the instrument fixed
///           frame defined by CMAT. The angular velocity is
///           returned only if NEEDAV is .TRUE.
///
///           The direction of the angular velocity vector gives
///           the right-handed axis about which the instrument fixed
///           reference frame is rotating. The magnitude of AV is
///           the magnitude of the instantaneous velocity of the
///           rotation, in radians per second.
///
///           The angular velocity vector is returned in component
///           form
///
///                    AV = [ AV1  , AV2  , AV3  ]
///
///           which is in terms of the inertial coordinate frame
///           specified in the segment descriptor.
///
///  CLKOUT   is the encoded SCLK associated with the returned
///           C-matrix and angular velocity vector.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If inputs are invalid or otherwise in appropriate, such that
///      the computed matrix is not a rotation matrix, an error is
///      signaled by a routine in the call tree of this routine.
/// ```
///
/// # Particulars
///
/// ```text
///  If the array RECORD contains pointing instances that bracket the
///  request time then CKE03 will linearly interpolate between those
///  two values to obtain pointing at the request time. If the
///  pointing instances in RECORD are for the same time, then this
///  routine will simply unpack the record and convert the quaternion
///  to a C-matrix.
///
///  The linear interpolation performed by this routine is defined
///  as follows:
///
///  1)  Let t be the time for which pointing is requested and
///      let CMAT1 and CMAT2 be C-matrices associated with times
///      t1 and t2 where:
///
///             t1 < t2,  and  t1 <= t,  and  t <= t2.
///
///  2)  Assume that the spacecraft frame rotates about a fixed
///      axis at a constant angular rate from time t1 to time t2.
///      The angle and rotation axis can be obtained from the
///      rotation matrix ROT12 where:
///
///                         T                       T
///                    CMAT2   =  ROT12    *   CMAT1
///
///         or
///                                    T
///                    ROT12   =  CMAT2    *   CMAT1
///
///
///                    ROT12   ==> ( ANGLE, AXIS )
///
///
///  3)  To obtain pointing at time t, rotate the spacecraft frame
///      about the vector AXIS from its orientation at time t1 by the
///      angle THETA where:
///
///                                         ( t  - t1 )
///                    THETA  =  ANGLE  *   -----------
///                                         ( t2 - t1 )
///
///  4)  Thus if ROT1t is the matrix that rotates vectors by the
///      angle THETA about the vector AXIS, then the output C-matrix
///      is given by:
///
///                        T                     T
///                    CMAT  =  ROT1t   *   CMAT1
///
///                                              T
///                    CMAT  =  CMAT1   *   ROT1t
///
///
///  5)  The angular velocity is treated independently of the
///      C-matrix. If it is requested, then the AV at time t is
///      the weighted average of the angular velocity vectors at
///      the times t1 and t2:
///
///                       ( t  - t1 )
///                 W  =  -----------
///                       ( t2 - t1 )
///
///
///                 AV  = ( 1 - W ) * AV1   +   W * AV2
/// ```
///
/// # Examples
///
/// ```text
///  The CKRnn routines are usually used in tandem with the CKEnn
///  routines, which evaluate the record returned by CKRnn to give
///  the pointing information and output time.
///
///  The following code fragment searches through all of the segments
///  in a file applicable to the Mars Observer spacecraft bus that
///  are of data type 3, for a particular spacecraft clock time.
///  It then evaluates the pointing for that epoch and prints the
///  result.
///
///        CHARACTER*(20)        SCLKCH
///        CHARACTER*(20)        SCTIME
///        CHARACTER*(40)        IDENT
///
///        INTEGER               I
///        INTEGER               SC
///        INTEGER               INST
///        INTEGER               HANDLE
///        INTEGER               DTYPE
///        INTEGER               ICD      (    6 )
///
///        DOUBLE PRECISION      SCLKDP
///        DOUBLE PRECISION      TOL
///        DOUBLE PRECISION      CLKOUT
///        DOUBLE PRECISION      DESCR    (    5 )
///        DOUBLE PRECISION      DCD      (    2 )
///        DOUBLE PRECISION      RECORD   (   17 )
///        DOUBLE PRECISION      CMAT     ( 3, 3 )
///        DOUBLE PRECISION      AV       (    3 )
///
///        LOGICAL               NEEDAV
///        LOGICAL               FND
///        LOGICAL               SFND
///
///
///        SC     = -94
///        INST   = -94000
///        DTYPE  =  3
///        NEEDAV = .FALSE.
///
///  C
///  C     Load the MO SCLK kernel and the C-kernel.
///  C
///        CALL FURNSH ( 'MO_SCLK.TSC'       )
///        CALL DAFOPR ( 'MO_CK.BC',  HANDLE )
///  C
///  C     Get the spacecraft clock time. Then encode it for use
///  C     in the C-kernel.
///  C
///        WRITE (*,*) 'Enter spacecraft clock time string:'
///        READ (*,FMT='(A)') SCLKCH
///
///        CALL SCENCD ( SC, SCLKCH, SCLKDP )
///  C
///  C     Use a tolerance of 2 seconds ( half of the nominal
///  C     separation between MO pointing instances ).
///  C
///        CALL SCTIKS ( SC, '0000000002:000', TOL )
///
///  C
///  C     Search from the beginning of the CK file through all
///  C     of the segments.
///  C
///        CALL DAFBFS ( HANDLE )
///        CALL DAFFNA ( SFND   )
///
///        FND    = .FALSE.
///
///        DO WHILE ( ( SFND ) .AND. ( .NOT. FND ) )
///
///  C
///  C        Get the segment identifier and descriptor.
///  C
///
///           CALL DAFGN ( IDENT                 )
///           CALL DAFGS ( DESCR                 )
///  C
///  C        Unpack the segment descriptor into its integer and
///  C        double precision components.
///  C
///           CALL DAFUS ( DESCR, 2, 6, DCD, ICD )
///
///  C
///  C        Determine if this segment should be processed.
///  C
///           IF ( ( INST          .EQ. ICD( 1 ) ) .AND.
///       .        ( SCLKDP + TOL  .GE. DCD( 1 ) ) .AND.
///       .        ( SCLKDP - TOL  .LE. DCD( 2 ) ) .AND.
///       .        ( DTYPE         .EQ. ICD( 3 ) )      ) THEN
///
///
///              CALL CKR03 ( HANDLE, DESCR, SCLKDP, TOL, NEEDAV,
///       .                   RECORD, FND )
///
///              IF ( FND ) THEN
///
///                 CALL CKE03 (NEEDAV,RECORD,CMAT,AV,CLKOUT)
///
///                 CALL SCDECD ( SC, CLKOUT, SCTIME )
///
///                 WRITE (*,*)
///                 WRITE (*,*) 'Segment identifier: ', IDENT
///                 WRITE (*,*)
///                 WRITE (*,*) 'Pointing returned for time: ',
///       .                      SCTIME
///                 WRITE (*,*)
///                 WRITE (*,*) 'C-matrix:'
///                 WRITE (*,*)
///                 WRITE (*,*) ( CMAT(1,I), I = 1, 3 )
///                 WRITE (*,*) ( CMAT(2,I), I = 1, 3 )
///                 WRITE (*,*) ( CMAT(3,I), I = 1, 3 )
///                 WRITE (*,*)
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
/// # Restrictions
///
/// ```text
///  1)  No explicit checking is done on the input RECORD.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  J.M. Lynch         (JPL)
///  F.S. Turner        (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 12-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.0.1, 22-AUG-2006 (EDW)
///
///         Replaced references to LDPOOL with references
///         to FURNSH.
///
/// -    SPICELIB Version 2.0.0, 13-JUN-2002 (FST)
///
///         This routine now participates in error handling properly.
///
/// -    SPICELIB Version 1.0.0, 25-NOV-1992 (JML)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 2.0.0, 13-JUN-2002 (FST)
///
///         Calls to CHKIN and CHKOUT in the standard SPICE error
///         handling style were added. Versions prior to 2.0.0
///         were error free, however changes to RAXISA from error
///         free to error signaling forced this update.
///
///         Additionally, FAILED is now checked after the call to
///         RAXISA. This prevents garbage from being placed into
///         the output arguments.
/// ```
pub fn cke03(
    ctx: &mut SpiceContext,
    needav: bool,
    record: &[f64],
    cmat: &mut [[f64; 3]; 3],
    av: &mut [f64; 3],
    clkout: &mut f64,
) -> crate::Result<()> {
    CKE03(
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

//$Procedure CKE03  ( C-kernel, evaluate pointing record, data type 3 )
pub fn CKE03(
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
    let mut T: f64 = 0.0;
    let mut T1: f64 = 0.0;
    let mut T2: f64 = 0.0;
    let mut Q1 = StackArray::<f64, 4>::new(1..=4);
    let mut Q2 = StackArray::<f64, 4>::new(1..=4);
    let mut CMAT1 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut CMAT2 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut AV1 = StackArray::<f64, 3>::new(1..=3);
    let mut AV2 = StackArray::<f64, 3>::new(1..=3);
    let mut AXIS = StackArray::<f64, 3>::new(1..=3);
    let mut ROT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut DELTA = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut FRAC: f64 = 0.0;
    let mut ANGLE: f64 = 0.0;

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
        CHKIN(b"CKE03", ctx)?;
    }

    //
    // Unpack the record, for easier reading.
    //
    T = RECORD[17];
    T1 = RECORD[1];
    T2 = RECORD[9];

    MOVED(RECORD.subarray(2), 4, Q1.as_slice_mut());
    MOVED(RECORD.subarray(6), 3, AV1.as_slice_mut());
    MOVED(RECORD.subarray(10), 4, Q2.as_slice_mut());
    MOVED(RECORD.subarray(14), 3, AV2.as_slice_mut());

    //
    // If T1 and T2 are the same then no interpolation or extrapolation
    // is performed.  Simply convert the quaternion to a C-matrix and
    // return.
    //
    if (T1 == T2) {
        Q2M(Q1.as_slice(), CMAT.as_slice_mut());

        *CLKOUT = T1;

        if NEEDAV {
            VEQU(AV1.as_slice(), AV.as_slice_mut());
        }

        CHKOUT(b"CKE03", ctx)?;

        return Ok(());
    }

    //
    // Interpolate between the two pointing instances to obtain pointing
    // at the request time.
    //

    //
    // Calculate what fraction of the interval the request time
    // represents.
    //
    FRAC = ((T - T1) / (T2 - T1));

    //
    // Convert the left and right quaternions to C-matrices.
    //
    Q2M(Q1.as_slice(), CMAT1.as_slice_mut());
    Q2M(Q2.as_slice(), CMAT2.as_slice_mut());

    //
    // Find the matrix that rotates the spacecraft instrument frame from
    // the orientation specified by CMAT1 to that specified by CMAT2.
    // Then find the axis and angle of that rotation matrix.
    //
    //         T                      T
    //    CMAT2   =    ROT    *  CMAT1
    //
    //                      T
    //    ROT     =    CMAT2  *  CMAT1
    //

    MTXM(CMAT2.as_slice(), CMAT1.as_slice(), ROT.as_slice_mut());

    RAXISA(ROT.as_slice(), AXIS.as_slice_mut(), &mut ANGLE, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"CKE03", ctx)?;
        return Ok(());
    }

    //
    // Calculate the matrix that rotates vectors about the vector AXIS
    // by the angle ANGLE * FRAC.
    //
    AXISAR(AXIS.as_slice(), (ANGLE * FRAC), DELTA.as_slice_mut());

    //
    // The interpolated pointing at the request time is given by CMAT
    // where:
    //
    //          T                    T
    //      CMAT   =  DELTA  *  CMAT1
    //
    // and
    //                               T
    //      CMAT   =  CMAT1  *  DELTA
    //

    MXMT(CMAT1.as_slice(), DELTA.as_slice(), CMAT.as_slice_mut());

    //
    // Set CLKOUT equal to the time that pointing is being returned.
    //
    *CLKOUT = T;

    //
    // If angular velocity is requested then take a weighted average
    // of the angular velocities at the left and right endpoints.
    //

    if NEEDAV {
        VLCOM(
            (1.0 - FRAC),
            AV1.as_slice(),
            FRAC,
            AV2.as_slice(),
            AV.as_slice_mut(),
        );
    }

    CHKOUT(b"CKE03", ctx)?;

    Ok(())
}
