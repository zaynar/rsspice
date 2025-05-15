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
const EPCIDX: i32 = 1;
const SBTIDX: i32 = 2;
const CNTIDX: i32 = 3;
const PKTIDX: i32 = 5;
const PKTBAS: i32 = (PKTIDX - 1);

/// C-Kernel, evaluate, type 5
///
/// Evaluate a single data record from a type 5 CK segment.
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
///  RECORD    I-O  Data type 5 record.
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
///  RECORD   is a record from a type 5 CK segment which, when
///           evaluated at the epoch contained in its first
///           element, will give the attitude and angular velocity
///           of a spacecraft structure or instrument relative to a
///           base reference frame.
///
///           The structure of the record is as follows:
///
///              +----------------------+
///              | evaluation epoch     |
///              +----------------------+
///              | subtype code         |
///              +----------------------+
///              | number of packets (n)|
///              +----------------------+
///              | nominal SCLK rate    |
///              +----------------------+
///              | packet 1             |
///              +----------------------+
///              | packet 2             |
///              +----------------------+
///                       .
///                       .
///                       .
///              +----------------------+
///              | packet n             |
///              +----------------------+
///              | epochs 1--n          |
///              +----------------------+
///
///             See the CK Required Reading or the include file
///             ck05.inc for details on CK type 5 packet contents.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RECORD   has been modified due to its use as a workspace array.
///           The contents are undefined.
///
///
///  CMAT     is a rotation matrix that transforms the components
///           of a vector expressed in the base frame given in
///           the segment to components expressed in the instrument
///           fixed frame at the returned time.
///
///           Thus, if a vector v has components x, y, z in the
///           base frame, then v has components x', y', z' in the
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
///           which is in terms of the base coordinate frame
///           specified in the segment descriptor.
///
///  CLKOUT   is the encoded SCLK associated with the returned
///           C-matrix and angular velocity vector.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input record contains an unrecognized subtype code,
///      the error SPICE(NOTSUPPORTED) is signaled.
///
///  2)  If the record subtype is one for which quaternion derivatives
///      are stored (subtypes 0 and 2), and if the Ith quaternion in
///      the input record is farther than its negative from the (I-1)st
///      quaternion in the record, the error SPICE(BADQUATSIGN) is
///      signaled.
///
///      For subtypes 1 and 3, this condition is not considered an
///      error: the closer to the preceding quaternion of the two
///      quaternion representations is used for interpolation.
/// ```
///
/// # Particulars
///
/// ```text
///  The exact format and structure of CK type 5 (MEX/Rosetta Attitude
///  file interpolation) CK segments is described in the CK Required
///  Reading.
/// ```
///
/// # Examples
///
/// ```text
///  The CKEnn routines are almost always used in conjunction with
///  the corresponding CKRnn routines, which read the records from
///  CK files.
///
///  The following code fragment searches through all of the segments
///  in a file applicable to the Mars Express spacecraft bus that
///  are of data type 5, for a particular spacecraft clock time.
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
///        SC     = -41
///        INST   = -41000
///        DTYPE  =  5
///        NEEDAV = .FALSE.
///
///  C
///  C     Load the MEX SCLK kernel and the C-kernel.
///  C
///        CALL FURNSH ( 'MEX_SCLK.TSC'       )
///        CALL DAFOPR ( 'MEX_CK.BC',  HANDLE )
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
///  C     separation between MEX pointing instances ).
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
///           CALL DAFGN ( IDENT )
///           CALL DAFGS ( DESCR )
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
///              CALL CKR05 ( HANDLE, DESCR, SCLKDP, TOL, NEEDAV,
///       .                   RECORD, FND )
///
///              IF ( FND ) THEN
///
///                 CALL CKE05 (NEEDAV,RECORD,CMAT,AV,CLKOUT)
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
///  1)  This routine assumes that the input record is valid. Any
///      checking of the input data is assumed to have been performed
///      when the source CK file was created.
///
///  2)  This routine assumes that the input data are suitable for the
///      interpolation method indicated by the subtype code in the
///      input record. Since the mapping of rotations to quaternions
///      is multiple-valued, this routine assumes that whichever sign
///      minimizes the Euclidean distance between one quaternion and
///      the next is the correct sign.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.1.1, 12-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 3.1.0, 11-AUG-2015 (NJB)
///
///         Bug fix: PRVPTR is now updated at the end of the quaternion
///         sequence check for Hermite subtypes.
///
/// -    SPICELIB Version 3.0.0, 06-FEB-2014 (NJB)
///
///         Bug fix and functional change: quaternion sign adjustment
///         is no longer performed for the Hermite subtypes (0 and 2).
///         If a sign adjustment is needed for quaternions belonging to
///         a record of Hermite subtype, an error is signaled. Sign
///         adjustment is still performed for the Lagrange subtypes.
///
///         Corrected in-line comments concerning change of AV units.
///
/// -    SPICELIB Version 2.0.0, 20-NOV-2006 (NJB)
///
///         Bug fix: this routine now assumes that angular velocity
///         and quaternion derivative values stored in the input
///         record have units of radians/second.
///
///         Bug fix: this routine no longer attempts to determine
///         the correct sign of quaternion derivatives. The caller
///         must supply quaternion derivatives that are suitable
///         for interpolation.
///
/// -    SPICELIB Version 1.3.0, 23-OCT-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments in
///         XPOSEG and VSCL calls. Replaced header reference to LDPOOL
///         with reference to FURNSH.
///
/// -    SPICELIB Version 1.2.0, 14-FEB-2003 (NJB)
///
///         Bug fix: angular velocity computation was modified to
///         match that used in the corresponding algorithm employed
///         by the MEX/Rosetta attitude file reader. The quaternion
///         derivative used to derive angular velocity now is the
///         derivative of the *unit* quaternion.
///
/// -    SPICELIB Version 1.1.0, 06-SEP-2002 (NJB)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.3.0, 23-OCT-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments in
///         XPOSEG and VSCL calls. Replaced header reference to LDPOOL
///         with reference to FURNSH.
///
/// -    SPICELIB Version 1.2.0, 14-FEB-2003 (NJB)
///
///         Bug fix: angular velocity computation was modified to
///         match that used in the corresponding algorithm employed
///         by the MEX/Rosetta attitude file reader. The quaternion
///         derivative used to derive angular velocity now is the
///         derivative of the *unit* quaternion.
///
///         Letting Q(t) be the quaternion derived by polynomial
///         interpolation, and letting UQ(t) be Q(t)/||Q(t)||,
///         the quaternion derivative d(UQ)/dt is now used.
/// ```
pub fn cke05(
    ctx: &mut SpiceContext,
    needav: bool,
    record: &mut [f64],
    cmat: &mut [[f64; 3]; 3],
    av: &mut [f64; 3],
    clkout: &mut f64,
) -> crate::Result<()> {
    CKE05(
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

//$Procedure CKE05 ( C-Kernel, evaluate, type 5 )
pub fn CKE05(
    NEEDAV: bool,
    RECORD: &mut [f64],
    CMAT: &mut [f64],
    AV: &mut [f64],
    CLKOUT: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut RECORD = DummyArrayMut::new(RECORD, 1..);
    let mut CMAT = DummyArrayMut2D::new(CMAT, 1..=3, 1..=3);
    let mut AV = DummyArrayMut::new(AV, 1..=3);
    let mut DQ = StackArray::<f64, 4>::new(0..=3);
    let mut DS = StackArray::<f64, 4>::new(0..=3);
    let mut LOCREC = ActualArray::<f64>::new(1..=CKMRSZ);
    let mut MAGS: f64 = 0.0;
    let mut Q = StackArray::<f64, 4>::new(0..=3);
    let mut QNEG = StackArray::<f64, 4>::new(0..=3);
    let mut RADTRM = StackArray::<f64, 4>::new(0..=3);
    let mut RATE: f64 = 0.0;
    let mut SCLDDQ = StackArray::<f64, 4>::new(0..=3);
    let mut SCLKDP: f64 = 0.0;
    let mut STATE = StackArray::<f64, 8>::new(1..=8);
    let mut VBUFF = StackArray::<f64, 6>::new(1..=6);
    let mut WORK = ActualArray2D::<f64>::new(1..=(CKMRSZ * 2), 1..=2);
    let mut FROM: i32 = 0;
    let mut N: i32 = 0;
    let mut NEWPTR: i32 = 0;
    let mut PACKSZ: i32 = 0;
    let mut PRVPTR: i32 = 0;
    let mut SUBTYP: i32 = 0;
    let mut TO: i32 = 0;
    let mut UB: i32 = 0;
    let mut XSTART: i32 = 0;
    let mut YSTART: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Index of evaluation epoch in record:
    //

    //
    // Index of subtype code in record:
    //

    //
    // Index of packet count in record:
    //

    //
    // Index at which packets start; packet base:
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

    CHKIN(b"CKE05", ctx)?;

    //
    // Capture the subtype from the record and set the packet size
    // accordingly.
    //
    SUBTYP = intrinsics::IDNINT(RECORD[SBTIDX]);

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
        CHKOUT(b"CKE05", ctx)?;
        return Ok(());
    }

    //
    // Get the packet count and epoch.
    //
    N = intrinsics::IDNINT(RECORD[CNTIDX]);
    SCLKDP = RECORD[EPCIDX];

    //
    // Get the nominal clock rate.
    //
    RATE = RECORD[4];

    //
    // Adjust quaternion "signs" as necessary to minimize distance
    // between successive quaternions. This adjustment is performed
    // only for subtypes that don't store quaternion derivatives
    // (these are the Lagrange subtypes).
    //
    if ((SUBTYP == C05TP1) || (SUBTYP == C05TP3)) {
        //
        // For these subtypes, only the quaternions themselves need be
        // adjusted.
        //
        // PRVPTR is the index of the "previous" quaternion---the one to
        // which the successor and its negative will be compared.
        //
        PRVPTR = PKTIDX;

        for I in 2..=N {
            //
            // NEWPTR points to the quaternion ahead of the one
            // pointed to by PRVPTR.
            //
            NEWPTR = (PKTIDX + (PACKSZ * (I - 1)));

            VMINUG(RECORD.subarray(NEWPTR), 4, QNEG.as_slice_mut());

            //
            // Replace the Ith quaternion with QNEG if QNEG is closer
            // than the current quaternion to the previous quaternion.
            //
            if (VDISTG(RECORD.subarray(PRVPTR), QNEG.as_slice(), 4)
                < VDISTG(RECORD.subarray(PRVPTR), RECORD.subarray(NEWPTR), 4))
            {
                MOVED(QNEG.as_slice(), 4, RECORD.subarray_mut(NEWPTR));
            }

            PRVPTR = NEWPTR;
        }
    } else {
        //
        // For the Hermite types, if the quaternions need to be adjusted,
        // we have an error condition.
        //
        // PRVPTR is the index of the "previous" quaternion---the one to
        // which the successor and its negative will be compared.
        //
        PRVPTR = PKTIDX;

        for I in 2..=N {
            //
            // NEWPTR points to the quaternion ahead of the one
            // pointed to by PRVPTR.
            //
            NEWPTR = (PKTIDX + (PACKSZ * (I - 1)));

            VMINUG(RECORD.subarray(NEWPTR), 4, QNEG.as_slice_mut());
            //
            // Replace the Ith quaternion with QNEG if QNEG is closer
            // than the current quaternion to the previous quaternion.
            //
            if (VDISTG(RECORD.subarray(PRVPTR), QNEG.as_slice(), 4)
                < VDISTG(RECORD.subarray(PRVPTR), RECORD.subarray(NEWPTR), 4))
            {
                SETMSG(b"Quaternion sign error: quaternion at index # in the input record is farther than its negative from the preceding quaternion in the record. Quaternion is (#, #, #, #); predecessor is (#, #, #, #). This makes the quaternion sequence unsuitable for Hermite interpolation. The quaternions, and if applicable, their derivatives, must be adjusted before they are passed to this routine.", ctx);
                ERRINT(b"#", I, ctx);
                ERRDP(b"#", RECORD[NEWPTR], ctx);
                ERRDP(b"#", RECORD[(NEWPTR + 1)], ctx);
                ERRDP(b"#", RECORD[(NEWPTR + 2)], ctx);
                ERRDP(b"#", RECORD[(NEWPTR + 3)], ctx);
                ERRDP(b"#", RECORD[PRVPTR], ctx);
                ERRDP(b"#", RECORD[(PRVPTR + 1)], ctx);
                ERRDP(b"#", RECORD[(PRVPTR + 2)], ctx);
                ERRDP(b"#", RECORD[(PRVPTR + 3)], ctx);
                SIGERR(b"SPICE(BADQUATSIGN)", ctx)?;
                CHKOUT(b"CKE05", ctx)?;
                return Ok(());
            }

            PRVPTR = NEWPTR;
        }
    }

    if (SUBTYP == C05TP1) {
        //
        // We perform Lagrange interpolation on each quaternion
        // component, and obtain quaternion derivatives from the
        // interpolating polynomials.  The quaternion and derivative
        // gives us angular velocity.
        //
        // We'll transpose the pointing information in the input record so
        // that contiguous pieces of it can be shoved directly into the
        // interpolation routine LGRINT.  We allow LGRINT to overwrite
        // the state values in the input record, since this saves local
        // storage and does no harm.  (See the header of LGRINT for a
        // description of its work space usage.)
        //
        N = intrinsics::IDNINT(RECORD[CNTIDX]);

        XPSGIP(PACKSZ, N, RECORD.subarray_mut(PKTIDX));

        //
        // We interpolate each state component in turn.
        //
        XSTART = (PKTIDX + (N * PACKSZ));

        for I in 1..=PACKSZ {
            YSTART = (PKTIDX + (N * (I - 1)));

            let [arg5, arg6] = STATE
                .get_disjoint_mut([I, (I + 4)])
                .expect("mutable array elements passed to function must have disjoint indexes");
            LGRIND(
                N,
                RECORD.subarray(XSTART),
                RECORD.subarray(YSTART),
                WORK.as_slice_mut(),
                SCLKDP,
                arg5,
                arg6,
                ctx,
            )?;
        }

        //
        // The output quaternion is a unitized version of the
        // interpolated state.
        //
        MAGS = VNORMG(STATE.as_slice(), 4);

        if (MAGS == 0.0) {
            SETMSG(b"Quaternion magnitude at SCLK # was zero.", ctx);
            ERRDP(b"#", SCLKDP, ctx);
            SIGERR(b"SPICE(DIVIDEBYZERO)", ctx)?;
            CHKOUT(b"CKE05", ctx)?;
            return Ok(());
        }

        VSCLG((1.0 / MAGS), STATE.as_slice(), 4, Q.as_slice_mut());

        if NEEDAV {
            //
            // Find the time derivative of the unit quaternion:
            // Letting S represent the quaternion portion of STATE, we
            // have
            //
            //    Q = S/||S||
            //
            //
            // Then letting < , > denote the 4-dimensional inner product
            // operator, we have
            //
            //
            //               d(S)/dt      < Q, d(S)/dt >
            //    d(Q)/dt =  -------  -   -------------- * Q
            //                ||S||            ||S||
            //
            //
            MOVED(STATE.subarray(5), 4, DS.as_slice_mut());

            VSCLG((1.0 / MAGS), DS.as_slice(), 4, SCLDDQ.as_slice_mut());
            VSCLG(
                (VDOTG(Q.as_slice(), DS.as_slice(), 4) / MAGS),
                Q.as_slice(),
                4,
                RADTRM.as_slice_mut(),
            );

            VSUBG(SCLDDQ.as_slice(), RADTRM.as_slice(), 4, DQ.as_slice_mut());
            //
            // Derive angular velocity from Q and dQ/dt:
            //
            QDQ2AV(Q.as_slice(), DQ.as_slice(), AV.as_slice_mut());

            //
            // Scale the AV from radians/tick to radians/second.
            //
            VSCLIP((1.0 / RATE), AV.as_slice_mut());
        }

    //
    // Q and if required AV have been assigned.
    //
    } else if (SUBTYP == C05TP3) {
        //
        // This is the easiest case:  we perform Lagrange interpolation
        // on each quaternion or angular velocity component.
        //
        // We'll transpose the pointing information in the input record so
        // that contiguous pieces of it can be shoved directly into the
        // interpolation routine LGRINT.  We allow LGRINT to overwrite
        // the state values in the input record, since this saves local
        // storage and does no harm.  (See the header of LGRINT for a
        // description of its work space usage.)
        //
        N = intrinsics::IDNINT(RECORD[CNTIDX]);

        XPSGIP(PACKSZ, N, RECORD.subarray_mut(PKTIDX));

        //
        // We interpolate each state component in turn.
        //
        XSTART = (PKTIDX + (N * PACKSZ));

        if NEEDAV {
            UB = PACKSZ;
        } else {
            UB = 4;
        }

        for I in 1..=UB {
            YSTART = (PKTIDX + (N * (I - 1)));

            STATE[I] = LGRINT(
                N,
                RECORD.subarray(XSTART),
                RECORD.subarray(YSTART),
                LOCREC.as_slice_mut(),
                SCLKDP,
                ctx,
            )?;
        }

        //
        // The output quaternion is a unitized version of the
        // interpolated state.
        //
        VHATG(STATE.as_slice(), 4, Q.as_slice_mut());

        if NEEDAV {
            //
            // The angular velocity already is in units of radians/second.
            //
            VEQU(STATE.subarray(5), AV.as_slice_mut());
        }

    //
    // Q and if required AV have been assigned.
    //
    } else {
        //
        // We have a Hermite-style subtype.  Whether it's subtype 0
        // or 2, we perform Hermite interpolation on the quaternions.
        //
        // We interpolate each quaternion component in turn.  Attitude and
        // angular velocity are interpolated separately.
        //
        XSTART = (PKTIDX + (PACKSZ * N));

        for I in 1..=4 {
            for J in 1..=N {
                //
                // For the Jth input packet, copy the Ith position and
                // velocity components into the local record buffer RECORD.
                //
                // In order to perform Hermite interpolation, the
                // quaternions and quaternion derivatives must have a
                // common time scale. So prior to interpolation, we scale
                // the units of the quaternion derivatives from radians/sec
                // to radians/tick.
                //
                FROM = ((PKTBAS + (PACKSZ * (J - 1))) + I);
                TO = ((2 * J) - 1);

                LOCREC[TO] = RECORD[FROM];
                LOCREC[(TO + 1)] = (RECORD[(FROM + 4)] * RATE);
            }

            //
            // Interpolate the Ith quaternion and quaternion derivative
            // components.
            //
            let [arg5, arg6] = STATE
                .get_disjoint_mut([I, (I + 4)])
                .expect("mutable array elements passed to function must have disjoint indexes");
            HRMINT(
                N,
                RECORD.subarray(XSTART),
                LOCREC.as_slice(),
                SCLKDP,
                WORK.as_slice_mut(),
                arg5,
                arg6,
                ctx,
            )?;
        }

        //
        // The output quaternion is a unitized version of the
        // interpolated state.
        //
        MAGS = VNORMG(STATE.as_slice(), 4);

        if (MAGS == 0.0) {
            SETMSG(b"Quaternion magnitude at SCLK # was zero.", ctx);
            ERRDP(b"#", SCLKDP, ctx);
            SIGERR(b"SPICE(DIVIDEBYZERO)", ctx)?;
            CHKOUT(b"CKE05", ctx)?;
            return Ok(());
        }

        VSCLG((1.0 / MAGS), STATE.as_slice(), 4, Q.as_slice_mut());

        if NEEDAV {
            if (SUBTYP == C05TP0) {
                //
                // Find the time derivative of the unit quaternion:
                // Letting S represent the quaternion portion of STATE, we
                // have
                //
                //    Q = S/||S||
                //
                //
                // Then letting < , > denote the 4-dimensional inner product
                // operator, we have
                //
                //
                //               d(S)/dt      < Q, d(S)/dt >
                //    d(Q)/dt =  -------  -   -------------- * Q
                //                ||S||            ||S||
                //
                //
                MOVED(STATE.subarray(5), 4, DS.as_slice_mut());

                VSCLG((1.0 / MAGS), DS.as_slice(), 4, SCLDDQ.as_slice_mut());
                VSCLG(
                    (VDOTG(Q.as_slice(), DS.as_slice(), 4) / MAGS),
                    Q.as_slice(),
                    4,
                    RADTRM.as_slice_mut(),
                );

                VSUBG(SCLDDQ.as_slice(), RADTRM.as_slice(), 4, DQ.as_slice_mut());
                //
                // Derive angular velocity from Q and dQ/dt:
                //
                QDQ2AV(Q.as_slice(), DQ.as_slice(), AV.as_slice_mut());

                //
                // Scale the AV from radians/tick to radians/second.
                //
                VSCLIP((1.0 / RATE), AV.as_slice_mut());
            } else {
                //
                // This is subtype 2; we perform Hermite interpolation on
                // the angular velocity and its derivative.
                //
                // Now interpolate angular velocity, using separate angular
                // velocity data and angular acceleration.
                //
                for I in 1..=3 {
                    for J in 1..=N {
                        //
                        // For the Jth input packet, copy the Ith position
                        // and velocity components into the local record
                        // buffer LOCREC.  Note that, as with quaternion
                        // derivatives, we must scale angular acceleration
                        // from radians/sec**2 to radians/(sec*tick) before
                        // interpolating.
                        //
                        FROM = (((PKTBAS + (PACKSZ * (J - 1))) + 8) + I);
                        TO = ((2 * J) - 1);

                        LOCREC[TO] = RECORD[FROM];
                        LOCREC[(TO + 1)] = (RECORD[(FROM + 3)] * RATE);
                    }

                    //
                    // Interpolate the Ith angular velocity and angular
                    // acceleration components of the attitude. We'll
                    // capture the result in a temporary buffer, then
                    // transfer the velocity to the output argument AV.
                    //
                    let [arg5, arg6] = VBUFF.get_disjoint_mut([I, (I + 3)]).expect(
                        "mutable array elements passed to function must have disjoint indexes",
                    );
                    HRMINT(
                        N,
                        RECORD.subarray(XSTART),
                        LOCREC.as_slice(),
                        SCLKDP,
                        WORK.as_slice_mut(),
                        arg5,
                        arg6,
                        ctx,
                    )?;
                }

                //
                // Fill in the angular velocity in the output angular
                // velocity vector using the results of interpolating
                // velocity and acceleration.
                //
                // The angular velocity is already in units of
                // radians/second.
                //
                VEQU(VBUFF.as_slice(), AV.as_slice_mut());
            }
            //
            // We've handled the type 0 and type 2 cases.
            //
        }
        //
        // We've computed the angular velocity AV for the Hermite
        // subtypes, if a.v. was requested.
        //
    }
    //
    // We've handled all four subtypes.
    //

    //
    // Produce a C-matrix from the interpolated quaternion. Set CLKOUT.
    //
    Q2M(Q.as_slice(), CMAT.as_slice_mut());

    *CLKOUT = RECORD[EPCIDX];

    CHKOUT(b"CKE05", ctx)?;
    Ok(())
}
