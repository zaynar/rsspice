//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const SIDLEN: i32 = 40;
const NDC: i32 = 5;
const ND: i32 = 2;
const NI: i32 = 6;
const DTYPE: i32 = 2;
const FPRINT: i32 = 32;
const LPRINT: i32 = 126;

/// C-Kernel, write segment to C-kernel, data type 2
///
/// Write a type 2 segment to a C-kernel.
///
/// # Required Reading
///
/// * [CK](crate::required_reading::ck)
/// * [DAF](crate::required_reading::daf)
/// * [SCLK](crate::required_reading::sclk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of an open CK file.
///  BEGTIM     I   The beginning encoded SCLK of the segment.
///  ENDTIM     I   The ending encoded SCLK of the segment.
///  INST       I   The NAIF instrument ID code.
///  REF        I   The reference frame of the segment.
///  SEGID      I   Segment identifier.
///  NREC       I   Number of pointing records.
///  START      I   Encoded SCLK interval start times.
///  STOP       I   Encoded SCLK interval stop times.
///  QUATS      I   SPICE quaternions representing instrument pointing.
///  AVVS       I   Angular velocity vectors.
///  RATES      I   Number of seconds per tick for each interval.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of the CK file to which the segment will
///           be written. The file must have been opened with write
///           access.
///
///  BEGTIM   is the beginning encoded SCLK time of the segment. This
///           value should be less than or equal to the first START
///           time in the segment.
///
///  ENDTIM   is the encoded SCLK time at which the segment ends.
///           This value should be greater than or equal to the last
///           STOP time in the segment.
///
///  INST     is the NAIF integer ID code for the instrument.
///
///  REF      is a character string that specifies the
///           reference frame of the segment. This should be one of
///           the frames supported by the SPICELIB routine NAMFRM
///           which is an entry point to FRAMEX.
///
///  SEGID    is the segment identifier. A CK segment identifier may
///           contain up to 40 characters.
///
///  NREC     is the number of pointing intervals that will be
///           written to the segment.
///
///  START    are the start times of each interval in encoded
///           spacecraft clock. These times must be strictly
///           increasing.
///
///  STOP     are the stop times of each interval in encoded
///           spacecraft clock. These times must be greater than
///           the START times that they correspond to but less
///           than or equal to the START time of the next interval.
///
///  QUATS    is an array of SPICE-style quaternions representing
///           the C-matrices associated with the start times of each
///           interval. See the discussion of quaternion styles in
///           $Particulars below.
///
///  AVVS     are the angular velocity vectors for each interval.
///
///  RATES    are the number of seconds per encoded spacecraft clock
///           tick for each interval.
///
///           In most applications this value will be the same for
///           each interval within a segment. For example, when
///           constructing a predict C-kernel for Mars Observer, the
///           rate would be 1/256 for each interval since this is
///           the smallest time unit expressible by the MO clock. The
///           nominal seconds per tick rates for Galileo and Voyager
///           are 1/120 and 0.06 respectively.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None. See $Files section.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If HANDLE is not the handle of a C-kernel opened for writing,
///      an error is signaled by a routine in the call tree of this
///      routine.
///
///  2)  If SEGID is more than 40 characters long, the error
///      SPICE(SEGIDTOOLONG) is signaled.
///
///  3)  If SEGID contains any nonprintable characters, the error
///      SPICE(NONPRINTABLECHARS) is signaled.
///
///  4)  If the first START time is negative, the error
///      SPICE(INVALIDSCLKTIME) is signaled.
///
///  5)  If the second or any subsequent START times are negative, the
///      error SPICE(TIMESOUTOFORDER) is signaled.
///
///  6)  If any of the STOP times are negative, the error
///      SPICE(DEGENERATEINTERVAL) is signaled.
///
///  7)  If the STOP time of any of the intervals is less than or equal
///      to the START time, the error SPICE(DEGENERATEINTERVAL) is
///      signaled.
///
///  8)  If the START times are not strictly increasing, the
///      error SPICE(TIMESOUTOFORDER) is signaled.
///
///  9)  If the STOP time of one interval is greater than the START
///      time of the next interval, the error SPICE(BADSTOPTIME)
///      is signaled.
///
///  10) If BEGTIM is greater than START(1) or ENDTIM is less than
///      STOP(NREC), the error SPICE(INVALIDDESCRTIME) is
///      signaled.
///
///  11) If the name of the reference frame is not one of those
///      supported by the routine NAMFRM, the error
///      SPICE(INVALIDREFFRAME) is signaled.
///
///  12) If NREC, the number of pointing records, is less than or
///      equal to 0, the error SPICE(INVALIDNUMRECS) is signaled.
///
///  13) If any quaternion has magnitude zero, the error
///      SPICE(ZEROQUATERNION) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  This routine adds a type 2 segment to a C-kernel. The C-kernel
///  may be either a new one or an existing one opened for writing.
/// ```
///
/// # Particulars
///
/// ```text
///  For a detailed description of a type 2 CK segment please see the
///  CK Required Reading.
///
///  This routine relieves the user from performing the repetitive
///  calls to the DAF routines necessary to construct a CK segment.
///
///
///  Quaternion Styles
///  -----------------
///
///  There are different "styles" of quaternions used in
///  science and engineering applications. Quaternion styles
///  are characterized by
///
///  -  The order of quaternion elements
///
///  -  The quaternion multiplication formula
///
///  -  The convention for associating quaternions
///     with rotation matrices
///
///  Two of the commonly used styles are
///
///     - "SPICE"
///
///        > Invented by Sir William Rowan Hamilton
///        > Frequently used in mathematics and physics textbooks
///
///     - "Engineering"
///
///        > Widely used in aerospace engineering applications
///
///
///  SPICELIB subroutine interfaces ALWAYS use SPICE quaternions.
///  Quaternions of any other style must be converted to SPICE
///  quaternions before they are passed to SPICELIB routines.
///
///
///  Relationship between SPICE and Engineering Quaternions
///  ------------------------------------------------------
///
///  Let M be a rotation matrix such that for any vector V,
///
///     M*V
///
///  is the result of rotating V by theta radians in the
///  counterclockwise direction about unit rotation axis vector A.
///  Then the SPICE quaternions representing M are
///
///     (+/-) (  cos(theta/2),
///              sin(theta/2) A(1),
///              sin(theta/2) A(2),
///              sin(theta/2) A(3)  )
///
///  while the engineering quaternions representing M are
///
///     (+/-) ( -sin(theta/2) A(1),
///             -sin(theta/2) A(2),
///             -sin(theta/2) A(3),
///              cos(theta/2)       )
///
///  For both styles of quaternions, if a quaternion q represents
///  a rotation matrix M, then -q represents M as well.
///
///  Given an engineering quaternion
///
///     QENG   = ( q0,  q1,  q2,  q3 )
///
///  the equivalent SPICE quaternion is
///
///     QSPICE = ( q3, -q0, -q1, -q2 )
///
///
///  Associating SPICE Quaternions with Rotation Matrices
///  ----------------------------------------------------
///
///  Let FROM and TO be two right-handed reference frames, for
///  example, an inertial frame and a spacecraft-fixed frame. Let the
///  symbols
///
///     V    ,   V
///      FROM     TO
///
///  denote, respectively, an arbitrary vector expressed relative to
///  the FROM and TO frames. Let M denote the transformation matrix
///  that transforms vectors from frame FROM to frame TO; then
///
///     V   =  M * V
///      TO         FROM
///
///  where the expression on the right hand side represents left
///  multiplication of the vector by the matrix.
///
///  Then if the unit-length SPICE quaternion q represents M, where
///
///     q = (q0, q1, q2, q3)
///
///  the elements of M are derived from the elements of q as follows:
///
///       .-                                                         -.
///       |           2    2                                          |
///       | 1 - 2*( q2 + q3 )   2*(q1*q2 - q0*q3)   2*(q1*q3 + q0*q2) |
///       |                                                           |
///       |                                                           |
///       |                               2    2                      |
///   M = | 2*(q1*q2 + q0*q3)   1 - 2*( q1 + q3 )   2*(q2*q3 - q0*q1) |
///       |                                                           |
///       |                                                           |
///       |                                                   2    2  |
///       | 2*(q1*q3 - q0*q2)   2*(q2*q3 + q0*q1)   1 - 2*( q1 + q2 ) |
///       |                                                           |
///       `-                                                         -'
///
///  Note that substituting the elements of -q for those of q in the
///  right hand side leaves each element of M unchanged; this shows
///  that if a quaternion q represents a matrix M, then so does the
///  quaternion -q.
///
///  To map the rotation matrix M to a unit quaternion, we start by
///  decomposing the rotation matrix as a sum of symmetric
///  and skew-symmetric parts:
///
///                                     2
///     M = [ I  +  (1-cos(theta)) OMEGA  ] + [ sin(theta) OMEGA ]
///
///                  symmetric                   skew-symmetric
///
///
///  OMEGA is a skew-symmetric matrix of the form
///
///                .-             -.
///                |  0   -n3   n2 |
///                |               |
///      OMEGA  =  |  n3   0   -n1 |
///                |               |
///                | -n2   n1   0  |
///                `-             -'
///
///  The vector N of matrix entries (n1, n2, n3) is the rotation axis
///  of M and theta is M's rotation angle. Note that N and theta
///  are not unique.
///
///  Let
///
///     C = cos(theta/2)
///     S = sin(theta/2)
///
///  Then the unit quaternions Q corresponding to M are
///
///     Q = +/- ( C, S*n1, S*n2, S*n3 )
///
///  The mappings between quaternions and the corresponding rotations
///  are carried out by the SPICELIB routines
///
///     Q2M {quaternion to matrix}
///     M2Q {matrix to quaternion}
///
///  M2Q always returns a quaternion with scalar part greater than
///  or equal to zero.
///
///
///  SPICE Quaternion Multiplication Formula
///  ---------------------------------------
///
///  Given a SPICE quaternion
///
///     Q = ( q0, q1, q2, q3 )
///
///  corresponding to rotation axis A and angle theta as above, we can
///  represent Q using "scalar + vector" notation as follows:
///
///     s =   q0           = cos(theta/2)
///
///     v = ( q1, q2, q3 ) = sin(theta/2) * A
///
///     Q = s + v
///
///  Let Q1 and Q2 be SPICE quaternions with respective scalar
///  and vector parts s1, s2 and v1, v2:
///
///     Q1 = s1 + v1
///     Q2 = s2 + v2
///
///  We represent the dot product of v1 and v2 by
///
///     <v1, v2>
///
///  and the cross product of v1 and v2 by
///
///     v1 x v2
///
///  Then the SPICE quaternion product is
///
///     Q1*Q2 = s1*s2 - <v1,v2>  + s1*v2 + s2*v1 + (v1 x v2)
///
///  If Q1 and Q2 represent the rotation matrices M1 and M2
///  respectively, then the quaternion product
///
///     Q1*Q2
///
///  represents the matrix product
///
///     M1*M2
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) The following example creates a CK file with a type-2 segment,
///     with data for a simple time dependent rotation and angular
///     velocity.
///
///     Example code begins here.
///
///
///           PROGRAM CKW02_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           CHARACTER*(*)         CK2
///           PARAMETER           ( CK2 = 'ckw02_ex1.bc' )
///
///           DOUBLE PRECISION      SPTICK
///           PARAMETER           ( SPTICK = 0.001D0 )
///
///           INTEGER               INST
///           PARAMETER           ( INST = -77702 )
///
///           INTEGER               MAXREC
///           PARAMETER           ( MAXREC = 21 )
///
///           INTEGER               NAMLEN
///           PARAMETER           ( NAMLEN = 42 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(NAMLEN)    REF
///           CHARACTER*(NAMLEN)    IFNAME
///           CHARACTER*(NAMLEN)    SEGID
///
///           DOUBLE PRECISION      AVVS   (   3,MAXREC )
///           DOUBLE PRECISION      BEGTIM
///           DOUBLE PRECISION      ENDTIM
///           DOUBLE PRECISION      QUATS  ( 0:3,MAXREC )
///           DOUBLE PRECISION      RATE
///           DOUBLE PRECISION      RATES  (     MAXREC )
///           DOUBLE PRECISION      RWMAT  ( 3, 3 )
///           DOUBLE PRECISION      SPACES
///           DOUBLE PRECISION      STARTS (     MAXREC )
///           DOUBLE PRECISION      STOPS  (     MAXREC )
///           DOUBLE PRECISION      STICKS
///           DOUBLE PRECISION      THETA
///           DOUBLE PRECISION      WMAT   ( 3, 3 )
///           DOUBLE PRECISION      WQUAT  ( 0:3 )
///
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               NCOMCH
///
///     C
///     C     NCOMCH is the number of characters to reserve for the
///     C     kernel's comment area. This example doesn't write
///     C     comments, so set to zero.
///     C
///           NCOMCH = 0
///
///     C
///     C     The base reference from for the rotation data.
///     C
///           REF = 'J2000'
///
///     C
///     C     Time spacing in encoded ticks and in seconds
///     C
///           STICKS = 10.D0
///           SPACES = STICKS * SPTICK
///
///     C
///     C     Declare an angular rate in radians per sec.
///     C
///           RATE = 1.D-2
///
///     C
///     C     Internal file name and segment ID.
///     C
///           SEGID  = 'Test type 2 CK segment'
///           IFNAME = 'Test CK type 2 segment created by CKW02'
///
///
///     C
///     C     Open a new kernel.
///     C
///           CALL CKOPN ( CK2, IFNAME, NCOMCH, HANDLE )
///
///     C
///     C     Create a 3x3 double precision identity matrix.
///     C
///           CALL IDENT ( WMAT )
///
///     C
///     C     Convert the matrix to quaternion.
///     C
///           CALL M2Q ( WMAT, WQUAT )
///
///     C
///     C     Copy the work quaternion to the first row of
///     C     QUATS.
///     C
///           CALL MOVED ( WQUAT, 4, QUATS(0,1) )
///
///     C
///     C     Create an angular velocity vector. This vector is in the
///     C     REF reference frame and indicates a constant rotation
///     C     about the Z axis.
///     C
///           CALL VPACK ( 0.D0, 0.D0, RATE, AVVS(1,1) )
///
///     C
///     C     Set the initial value of the encoded ticks. The interval
///     C     associated with each quaternion will start at the epoch
///     C     of the quaternion and will extend 0.8 * STICKS forward in
///     C     time, leaving small gaps between the intervals.
///     C
///     C     The clock rates array will have a constant SPTICK value.
///     C
///           STARTS(1) = 1000.D0
///           STOPS(1)  = STARTS(1) + ( 0.8D0 * STICKS )
///           RATES(1)  = SPTICK
///
///     C
///     C     Fill the rest of the AVVS and QUATS matrices
///     C     with simple data.
///     C
///           DO I = 2, MAXREC
///
///     C
///     C        Create the corresponding encoded tick value in
///     C        increments of STICKS with an initial value of
///     C        1000.0 ticks.
///     C
///              STARTS(I) = 1000.D0 + (I-1) * STICKS
///              STOPS(I)  = STARTS(I) + ( 0.8D0 * STICKS )
///              RATES(I)  = SPTICK
///
///     C
///     C        Create the transformation matrix for a rotation of
///     C        THETA about the Z axis. Calculate THETA from the
///     C        constant angular rate RATE at increments of SPACES.
///     C
///              THETA = (I-1) * RATE * SPACES
///              CALL ROTMAT ( WMAT, THETA, 3, RWMAT )
///
///     C
///     C        Convert the RWMAT matrix to SPICE type quaternion.
///     C
///              CALL M2Q ( RWMAT, WQUAT )
///
///     C
///     C        Store the quaternion in the QUATS matrix.
///     C        Store angular velocity in AVVS.
///     C
///              CALL MOVED ( WQUAT, 4, QUATS(0,I) )
///              CALL VPACK ( 0.D0, 0.D0, RATE, AVVS(1,I) )
///
///           END DO
///
///     C
///     C     Set the segment boundaries equal to the first and last
///     C     time for the data arrays.
///     C
///           BEGTIM = STARTS(1)
///           ENDTIM = STOPS(MAXREC)
///
///     C
///     C     All information ready to write. Write to a CK type 2
///     C     segment to the file indicated by HANDLE.
///     C
///           CALL CKW02 ( HANDLE, BEGTIM, ENDTIM, INST,  REF,
///          .             SEGID,  MAXREC, STARTS, STOPS, QUATS,
///          .             AVVS,   RATES                       )
///
///     C
///     C     SAFELY close the file.
///     C
///           CALL CKCLS ( HANDLE )
///
///           END
///
///
///     When this program is executed, no output is presented on
///     screen. After run completion, a new CK file exists in the
///     output directory.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  J.M. Lynch         (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.0.1, 26-MAY-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Created
///         complete code example from existing fragment.
///
///         Updated Exception #12 to describe the actual check and error
///         produced by this routine.
///
/// -    SPICELIB Version 3.0.0, 01-JUN-2010 (NJB)
///
///         The check for non-unit quaternions has been replaced
///         with a check for zero-length quaternions.
///
/// -    SPICELIB Version 2.2.0, 26-FEB-2008 (NJB)
///
///         Updated header; added information about SPICE
///         quaternion conventions.
///
///         Minor typo in a long error message was corrected.
///
/// -    SPICELIB Version 2.1.0, 22-FEB-1999 (WLT)
///
///         Added check to make sure that all quaternions are unit
///         length to single precision.
///
/// -    SPICELIB Version 2.0.0, 28-DEC-1993 (WLT)
///
///         The routine was upgraded to support non-inertial reference
///         frames.
///
/// -    SPICELIB Version 1.1.1, 05-SEP-1993 (KRG)
///
///         Removed all references to a specific method of opening the CK
///         file in the $Brief_I/O, $Detailed_Input, $Exceptions,
///         $Files, and $Examples sections of the header. It is assumed
///         that a person using this routine has some knowledge of the DAF
///         system and the methods for obtaining file handles.
///
/// -    SPICELIB Version 1.1.0, 25-NOV-1992 (JML)
///
///         1) If the number of pointing records is not positive an error
///            is now signaled.
///
///         2) FAILED is checked after the call to DAFBNA.
///
///         3) The variables HLDBEG and HLDEND were removed from the loop
///            where the interval start and stop times are tested.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 30-AUG-1991 (JML)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 2.1.0, 22-FEB-1999 (WLT)
///
///         Added check to make sure that all quaternions are unit
///         length to single precision.
///
/// -    SPICELIB Version 1.1.1, 05-SEP-1993 (KRG)
///
///         Removed all references to a specific method of opening the CK
///         file in the $Brief_I/O, $Detailed_Input, $Exceptions,
///         $Files, and $Examples sections of the header. It is assumed
///         that a person using this routine has some knowledge of the DAF
///         system and the methods for obtaining file handles.
///
/// -    SPICELIB Version 1.1.0, 25-NOV-1992 (JML)
///
///         1) If the number of pointing records is not positive an error
///            is now signaled.
///
///         2) FAILED is checked after the call to DAFBNA.
///
///         3) The variables HLDBEG and HLDEND were removed from the loop
///            where the interval start and stop times are tested.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 30-AUG-1991 (JML)
/// ```
pub fn ckw02(
    ctx: &mut SpiceContext,
    handle: i32,
    begtim: f64,
    endtim: f64,
    inst: i32,
    ref_: &str,
    segid: &str,
    nrec: i32,
    start: &[f64],
    stop: &[f64],
    quats: &[[f64; 4]],
    avvs: &[[f64; 3]],
    rates: &[f64],
) -> crate::Result<()> {
    CKW02(
        handle,
        begtim,
        endtim,
        inst,
        ref_.as_bytes(),
        segid.as_bytes(),
        nrec,
        start,
        stop,
        quats.as_flattened(),
        avvs.as_flattened(),
        rates,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CKW02 ( C-Kernel, write segment to C-kernel, data type 2 )
pub fn CKW02(
    HANDLE: i32,
    BEGTIM: f64,
    ENDTIM: f64,
    INST: i32,
    REF: &[u8],
    SEGID: &[u8],
    NREC: i32,
    START: &[f64],
    STOP: &[f64],
    QUATS: &[f64],
    AVVS: &[f64],
    RATES: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let START = DummyArray::new(START, 1..);
    let STOP = DummyArray::new(STOP, 1..);
    let QUATS = DummyArray2D::new(QUATS, 0..=3, 1..);
    let AVVS = DummyArray2D::new(AVVS, 1..=3, 1..);
    let RATES = DummyArray::new(RATES, 1..);
    let mut DCD = StackArray::<f64, 2>::new(1..=ND);
    let mut DIRENT: f64 = 0.0;
    let mut DESCR = StackArray::<f64, 5>::new(1..=NDC);
    let mut ICD = StackArray::<i32, 6>::new(1..=NI);
    let mut INDEX: i32 = 0;
    let mut NDIR: i32 = 0;
    let mut REFCOD: i32 = 0;
    let mut VALUE: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    // SIDLEN   is the maximum number of characters allowed in a CK
    //          segment identifier.
    //
    // NDC      is the size of a packed CK segment descriptor.
    //
    // ND       is the number of double precision components in a CK
    //          segment descriptor.
    //
    // NI       is the number of integer components in a CK segment
    //          descriptor.
    //
    // DTYPE    is the data type of the segment that this routine
    //          operates on.
    //
    // FPRINT   is the integer value of the first printable ASCII
    //          character.
    //
    // LPRINT   is the integer value of the last printable ASCII
    //          character.
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
    } else {
        CHKIN(b"CKW02", ctx)?;
    }

    //
    // The first thing that we will do is create the segment descriptor.
    //
    // The structure of the segment descriptor is as follows.
    //
    //       DCD( 1 ) and DCD( 2 ) -- SCLK limits of the segment.
    //       ICD( 1 )              -- Instrument code.
    //       ICD( 2 )              -- Reference frame ID.
    //       ICD( 3 )              -- Data type of the segment.
    //       ICD( 4 )              -- Angular rates flag.
    //       ICD( 5 )              -- Beginning address of the segment.
    //       ICD( 6 )              -- Ending address of the segment.
    //

    //
    // Make sure that there is a positive number of pointing records.
    //
    if (NREC <= 0) {
        SETMSG(
            b"# is an invalid number of pointing instances for type 2.",
            ctx,
        );
        ERRINT(b"#", NREC, ctx);
        SIGERR(b"SPICE(INVALIDNUMREC)", ctx)?;
        CHKOUT(b"CKW02", ctx)?;
        return Ok(());
    }

    //
    // Check that the SCLK bounds on the segment are reasonable.
    //
    if (BEGTIM > START[1]) {
        SETMSG(
            b"The first d.p. component of the descriptor is invalid.  DCD(1) = # and START(1) = # ",
            ctx,
        );
        ERRDP(b"#", BEGTIM, ctx);
        ERRDP(b"#", START[1], ctx);
        SIGERR(b"SPICE(INVALIDDESCRTIME)", ctx)?;
        CHKOUT(b"CKW02", ctx)?;
        return Ok(());
    }

    if (ENDTIM < STOP[NREC]) {
        SETMSG(b"The second d.p. component of the descriptor is invalid.  DCD(2) = # and STOP(NREC) = # ", ctx);
        ERRDP(b"#", ENDTIM, ctx);
        ERRDP(b"#", STOP[NREC], ctx);
        SIGERR(b"SPICE(INVALIDDESCRTIME)", ctx)?;
        CHKOUT(b"CKW02", ctx)?;
        return Ok(());
    }

    DCD[1] = BEGTIM;
    DCD[2] = ENDTIM;

    //
    // Get the NAIF integer code for the reference frame.
    //
    NAMFRM(REF, &mut REFCOD, ctx)?;

    if (REFCOD == 0) {
        SETMSG(b"The reference frame # is not supported.", ctx);
        ERRCH(b"#", REF, ctx);
        SIGERR(b"SPICE(INVALIDREFFRAME)", ctx)?;
        CHKOUT(b"CKW02", ctx)?;
        return Ok(());
    }

    //
    // Assign values to the integer components of the segment descriptor.
    // By definition data type two must have angular velocity.
    //
    ICD[1] = INST;
    ICD[2] = REFCOD;
    ICD[3] = DTYPE;
    ICD[4] = 1;

    //
    // Now pack the segment descriptor.
    //
    DAFPS(ND, NI, DCD.as_slice(), ICD.as_slice(), DESCR.as_slice_mut());

    //
    // Now check that all the characters in the segid can be printed.
    //
    for I in 1..=LASTNB(SEGID) {
        VALUE = intrinsics::ICHAR(fstr::substr(SEGID, I..=I));

        if ((VALUE < FPRINT) || (VALUE > LPRINT)) {
            SETMSG(
                b"The segment identifier contains nonprintable characters",
                ctx,
            );
            SIGERR(b"SPICE(NONPRINTABLECHARS)", ctx)?;
            CHKOUT(b"CKW02", ctx)?;
            return Ok(());
        }
    }

    //
    // Also check to see if the segment identifier is too long.
    //
    if (LASTNB(SEGID) > SIDLEN) {
        SETMSG(b"Segment identifier contains more than 40 characters.", ctx);
        SIGERR(b"SPICE(SEGIDTOOLONG)", ctx)?;
        CHKOUT(b"CKW02", ctx)?;
        return Ok(());
    }

    //
    // Now check that the START and STOP times on the intervals
    // make sense. Three checks will be performed on each interval:
    //
    //    1)  Check that the STOP time is greater than the START time.
    //
    //    2)  Check that the START times are strictly increasing.
    //
    //    3)  Check that the START time is greater than or equal to the
    //        STOP time from the previous interval.
    //
    // For the first interval also make sure that the START time is
    // nonnegative.
    //
    if (START[1] < 0.0) {
        SETMSG(b"The first START time: # is negative.", ctx);
        ERRDP(b"#", START[1], ctx);
        SIGERR(b"SPICE(INVALIDSCLKTIME)", ctx)?;
        CHKOUT(b"CKW02", ctx)?;
        return Ok(());
    }

    if (STOP[1] <= START[1]) {
        SETMSG(b"The STOP time is less than or equal to the START time for interval number 1. START time is # and STOP time is #.", ctx);
        ERRDP(b"#", START[1], ctx);
        ERRDP(b"#", STOP[1], ctx);
        SIGERR(b"SPICE(DEGENERATEINTERVAL)", ctx)?;
        CHKOUT(b"CKW02", ctx)?;
        return Ok(());
    }

    for I in 2..=NREC {
        if (STOP[I] <= START[I]) {
            SETMSG(b"The STOP time is less than or equal to the START time for interval number #. START time is # and STOP time is #.", ctx);
            ERRINT(b"#", I, ctx);
            ERRDP(b"#", START[I], ctx);
            ERRDP(b"#", STOP[I], ctx);
            SIGERR(b"SPICE(DEGENERATEINTERVAL)", ctx)?;
            CHKOUT(b"CKW02", ctx)?;
            return Ok(());
        }

        if (START[I] <= START[(I - 1)]) {
            SETMSG(
                b"The START times are not strictly increasing.  START(#) = # and START(#) = #.",
                ctx,
            );
            ERRINT(b"#", I, ctx);
            ERRDP(b"#", START[I], ctx);
            ERRINT(b"#", (I - 1), ctx);
            ERRDP(b"#", START[(I - 1)], ctx);
            SIGERR(b"SPICE(TIMESOUTOFORDER)", ctx)?;
            CHKOUT(b"CKW02", ctx)?;
            return Ok(());
        }

        if (STOP[(I - 1)] > START[I]) {
            SETMSG(b"The STOP time for interval # is greater than the following START time. STOP(#) = # and START(#) = #.", ctx);
            ERRINT(b"#", (I - 1), ctx);
            ERRINT(b"#", (I - 1), ctx);
            ERRDP(b"#", STOP[(I - 1)], ctx);
            ERRINT(b"#", I, ctx);
            ERRDP(b"#", START[I], ctx);
            SIGERR(b"SPICE(BADSTOPTIME)", ctx)?;
            CHKOUT(b"CKW02", ctx)?;
            return Ok(());
        }
    }

    //
    // Make sure that the quaternions are non-zero. This is just
    // a check for uninitialized data.
    //
    for I in 1..=NREC {
        if VZEROG(QUATS.subarray([0, I]), 4) {
            SETMSG(b"The quaternion at index # has magnitude zero.", ctx);
            ERRINT(b"#", I, ctx);
            SIGERR(b"SPICE(ZEROQUATERNION)", ctx)?;
            CHKOUT(b"CKW02", ctx)?;
            return Ok(());
        }
    }

    //
    // No more checks, begin writing the segment.
    //

    DAFBNA(HANDLE, DESCR.as_slice(), SEGID, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"CKW02", ctx)?;
        return Ok(());
    }

    //
    // Now add the quaternions, angular velocity vectors, and time
    // conversion factors for each interval.
    //
    for I in 1..=NREC {
        DAFADA(QUATS.subarray([0, I]), 4, ctx)?;
        DAFADA(AVVS.subarray([1, I]), 3, ctx)?;
        DAFADA(RATES.subarray(I), 1, ctx)?;
    }

    //
    // The SCLK start times.
    //
    DAFADA(START.as_slice(), NREC, ctx)?;
    //
    // The SCLK stop times.
    //
    DAFADA(STOP.as_slice(), NREC, ctx)?;

    //
    // The time tag directory.  The Ith element is defined to be the
    // average of the (I*100)th STOP time and the (I*100+1)th START time.
    //
    NDIR = ((NREC - 1) / 100);

    INDEX = 100;

    for I in 1..=NDIR {
        DIRENT = ((STOP[INDEX] + START[(INDEX + 1)]) / 2.0);
        DAFADA(&[DIRENT], 1, ctx)?;
        INDEX = (INDEX + 100);
    }

    //
    // End the segment.
    //
    DAFENA(ctx)?;

    CHKOUT(b"CKW02", ctx)?;
    Ok(())
}
