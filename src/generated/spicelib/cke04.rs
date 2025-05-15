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

/// C-kernel, evaluate pointing record, type 4
///
/// Evaluate a pointing record returned by CKR04 from a CK type 4
/// segment. Return the C-matrix and angular velocity vector
/// associated with the time CLKOUT.
///
/// # Required Reading
///
/// * [CK](crate::required_reading::ck)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NEEDAV     I   .TRUE. if angular velocity is requested.
///  RECORD     I   Data type 4 pointing record.
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
///  RECORD   is a set of double precision numbers returned by
///           CKR04. RECORD must have the following structure:
///
///           ---------------------------------------------------
///           |    Encoded onboard time which is the closest    |
///           |  to SCLKDP and belongs to one of approximation  |
///           |                   intervals                     |
///           ---------------------------------------------------
///           |       encoded SCLK time of the midpoint of      |
///           |             interpolation interval              |
///           ---------------------------------------------------
///           |          radii of interpolation interval        |
///           |    expressed as double precision SCLK ticks     |
///           ---------------------------------------------------
///           |         Number of coefficients for q0           |
///           ---------------------------------------------------
///           |         Number of coefficients for q1           |
///           ---------------------------------------------------
///           |         Number of coefficients for q2           |
///           ---------------------------------------------------
///           |         Number of coefficients for q3           |
///           ---------------------------------------------------
///           |         Number of coefficients for AV1          |
///           ---------------------------------------------------
///           |         Number of coefficients for AV2          |
///           ---------------------------------------------------
///           |         Number of coefficients for AV3          |
///           ---------------------------------------------------
///           |               q0 Cheby coefficients             |
///           ---------------------------------------------------
///           |               q1 Cheby coefficients             |
///           ---------------------------------------------------
///           |               q2 Cheby coefficients             |
///           ---------------------------------------------------
///           |               q3 Cheby coefficients             |
///           ---------------------------------------------------
///           |         AV1 Cheby coefficients (optional)       |
///           ---------------------------------------------------
///           |         AV2 Cheby coefficients (optional)       |
///           ---------------------------------------------------
///           |         AV3 Cheby coefficients (optional)       |
///           ---------------------------------------------------
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
///           inertial frame, then v has components x', y', z' in
///           the instrument fixed frame where:
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
/// # Parameters
///
/// ```text
///  See 'ckparam.inc'.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  No checking is done to determine whether RECORD is valid.
/// ```
///
/// # Particulars
///
/// ```text
///  For a detailed description of the structure of a type 4 pointing
///  segment, see the CK Required Reading file.
///
///  The work done by CKE04 is to calculate quaternion and angular
///  velocity components using Chebyshev polynomial approximation
///  parameters. The second step of evaluation is to convert the
///  pointing portion of the record from quaternion form to C-matrix
///  form.
///
///  The angular velocity vector will only be returned if it has been
///  requested. In other words, if NEEDAV is .TRUE., the routine will
///  expect the angular velocity component of the record to be
///  present.
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
///  in a file applicable to the Mars Global Surveyor spacecraft bus
///  that are of data type 4, for a particular spacecraft clock time.
///  It then evaluates the pointing for that epoch and prints the
///  result.
///
///  C
///  C     CK parameters include file.
///  C
///        INCLUDE               'ckparam.inc'
///  C
///  C     Local variables
///  C
///        CHARACTER*(20)        SCLKCH
///        CHARACTER*(20)        SCTIME
///        CHARACTER*(40)        IDENT
///
///        DOUBLE PRECISION      AV     ( 3 )
///        DOUBLE PRECISION      CLKOUT
///        DOUBLE PRECISION      CMAT   ( 3, 3 )
///        DOUBLE PRECISION      DCD    ( 2 )
///        DOUBLE PRECISION      DESCR  ( 5 )
///        DOUBLE PRECISION      RECORD ( CK4RSZ )
///        DOUBLE PRECISION      SCLKDP
///        DOUBLE PRECISION      TOL
///
///        INTEGER               HANDLE
///        INTEGER               I
///        INTEGER               ICD    ( 6 )
///        INTEGER               INST
///        INTEGER               SC
///
///        LOGICAL               FND
///        LOGICAL               NEEDAV
///        LOGICAL               SFND
///  C
///  C     Initial values.
///  C
///        SC     = -94
///        INST   = -94000
///        NEEDAV = .FALSE.
///  C
///  C     Load the MGS SCLK kernel and the C-kernel.
///  C
///        CALL FURNSH( 'MGS_SCLK.TSC' )
///        CALL DAFOPR( 'MGS_CK4.BC', HANDLE )
///  C
///  C     Get the spacecraft clock time. Then encode it for use
///  C     in the C-kernel.
///  C
///        CALL PROMPT( 'Enter SCLK string: ', SCLKCH )
///        CALL SCENCD( SC, SCLKCH, SCLKDP )
///  C
///  C     Use a tolerance of 2 seconds (half of the nominal
///  C     separation between MGS pointing instances ).
///  C
///        CALL SCTIKS ( SC, '0000000002:000', TOL )
///  C
///  C     Search from the beginning of the CK file through all
///  C     of the segments.
///  C
///        CALL DAFBFS( HANDLE )
///        CALL DAFFNA( SFND   )
///
///        FND = .FALSE.
///
///        DO WHILE ( ( SFND ) .AND. ( .NOT. FND ) )
///  C
///  C        Get the segment identifier and descriptor.
///  C
///           CALL DAFGN( IDENT )
///           CALL DAFGS( DESCR )
///  C
///  C        Unpack the segment descriptor into its integer and
///  C        double precision components.
///  C
///           CALL DAFUS( DESCR, 2, 6, DCD, ICD )
///  C
///  C        Determine if this segment should be processed.
///  C
///           IF ( ( INST          .EQ. ICD( 1 ) ) .AND.
///       .        ( SCLKDP + TOL  .GE. DCD( 1 ) ) .AND.
///       .        ( SCLKDP - TOL  .LE. DCD( 2 ) ) .AND.
///       .        ( CK4DTP        .EQ. ICD( 3 ) )      ) THEN
///  C
///  C           Find CK 4 record covering requested time.
///  C
///              CALL CKR04( HANDLE, DESCR, SCLKDP, TOL, NEEDAV,
///       .                  RECORD, FND )
///
///              IF ( FND ) THEN
///  C
///  C              Compute pointing using found CK 4 record.
///  C
///                 CALL CKE04( NEEDAV, RECORD, CMAT, AV, CLKOUT)
///
///                 CALL SCDECD( SC, CLKOUT, SCTIME )
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
///  1)  No checking is done on the input RECORD.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  E.D. Wright        (JPL)
///  Y.K. Zaiko         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.3, 12-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.2, 18-APR-2014 (BVS)
///
///         Minor header edits.
///
/// -    SPICELIB Version 1.0.1, 22-AUG-2006 (EDW)
///
///         Replaced references to LDPOOL with references
///         to FURNSH.
///
/// -    SPICELIB Version 1.0.0, 05-MAY-1999 (YKZ) (BVS)
/// ```
pub fn cke04(
    needav: bool,
    record: &[f64],
    cmat: &mut [[f64; 3]; 3],
    av: &mut [f64; 3],
    clkout: &mut f64,
) {
    CKE04(needav, record, cmat.as_flattened_mut(), av, clkout);
}

//$Procedure CKE04 ( C-kernel, evaluate pointing record, type 4 )
pub fn CKE04(NEEDAV: bool, RECORD: &[f64], CMAT: &mut [f64], AV: &mut [f64], CLKOUT: &mut f64) {
    let RECORD = DummyArray::new(RECORD, 1..);
    let mut CMAT = DummyArrayMut2D::new(CMAT, 1..=3, 1..=3);
    let mut AV = DummyArrayMut::new(AV, 1..=3);
    let mut Q = StackArray::<f64, 4>::new(1..=4);
    let mut QOUT = StackArray::<f64, 4>::new(1..=4);
    let mut BASADD: i32 = 0;
    let mut IDEG = StackArray::<i32, 7>::new(1..=QAVSIZ);

    //
    // Local variables
    //

    //
    // Initial values.
    //
    AV[1] = 0.0;
    AV[2] = 0.0;
    AV[3] = 0.0;

    //
    // Read numbers of polynomial coefficients from input record to
    // local integer array.
    //
    for I in 1..=QAVSIZ {
        IDEG[I] = (RECORD[(3 + I)] as i32);
    }

    //
    // Evaluate polynomial function for quaternion components at time
    // RECORD( 1 ).
    //
    BASADD = (CK4SFT + 1);

    for I in 1..=QSIZ {
        CHBVAL(
            RECORD.subarray(BASADD),
            (IDEG[I] - 1),
            RECORD.subarray(2),
            RECORD[1],
            &mut Q[I],
        );
        BASADD = (BASADD + IDEG[I]);
    }

    //
    // Normalize quaternion.
    //
    VHATG(Q.as_slice(), QSIZ, QOUT.as_slice_mut());

    //
    // Convert the quaternion to a C-matrix.
    //
    Q2M(QOUT.as_slice(), CMAT.as_slice_mut());

    *CLKOUT = RECORD[1];

    //
    // Check if angular velocities have to be evaluated, then
    // evaluate them.
    //
    if NEEDAV {
        for I in (QSIZ + 1)..=QAVSIZ {
            CHBVAL(
                RECORD.subarray(BASADD),
                (IDEG[I] - 1),
                RECORD.subarray(2),
                RECORD[1],
                &mut AV[(I - QSIZ)],
            );
            BASADD = (BASADD + IDEG[I]);
        }
    }

    //
    // All done.
    //
}
