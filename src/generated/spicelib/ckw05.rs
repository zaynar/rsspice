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
const MAXDEG: i32 = 23;
const SIDLEN: i32 = 40;
const FPRINT: i32 = 32;
const LPRINT: i32 = 126;
const ND: i32 = 2;
const NI: i32 = 6;
const DSCSIZ: i32 = 5;
const TYPIDX: i32 = 4;
const DTYPE: i32 = 5;
const DIRSIZ: i32 = 100;
const STATSZ: i32 = 6;
const QIDX: i32 = 1;

/// Write CK segment, type 5
///
/// Write a type 5 segment to a CK file.
///
/// # Required Reading
///
/// * [CK](crate::required_reading::ck)
/// * [NAIF_IDS](crate::required_reading::naif_ids)
/// * [ROTATION](crate::required_reading::rotation)
/// * [TIME](crate::required_reading::time)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of an CK file open for writing.
///  SUBTYP     I   CK type 5 subtype code.
///  DEGREE     I   Degree of interpolating polynomials.
///  BEGTIM     I   Start time of interval covered by segment.
///  ENDTIM     I   End time of interval covered by segment.
///  INST       I   NAIF code for a s/c instrument or structure.
///  REF        I   Reference frame name.
///  AVFLAG     I   Flag indicating if the segment will contain angular
///                 velocity.
///  SEGID      I   Segment identifier.
///  N          I   Number of packets.
///  SCLKDP     I   Encoded SCLK times.
///  PACKTS     I   Array of packets.
///  RATE       I   Nominal SCLK rate in seconds per tick.
///  NINTS      I   Number of intervals.
///  STARTS     I   Encoded SCLK interval start times.
///  MAXDEG     P   Maximum allowed degree of interpolating polynomial.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the file handle of a CK file that has been
///           opened for writing.
///
///  SUBTYP   is an integer code indicating the subtype of the
///           the segment to be created.
///
///  DEGREE   is the degree of the polynomials used to
///           interpolate the quaternions contained in the input
///           packets. All components of the quaternions are
///           interpolated by polynomials of fixed degree.
///
///  BEGTIM,
///  ENDTIM   are the beginning and ending encoded SCLK times
///           for which the segment provides pointing
///           information. BEGTIM must be less than or equal to
///           ENDTIM, and at least one data packet must have a
///           time tag T such that
///
///              BEGTIM  <  T  <  ENDTIM
///                      -     -
///
///  INST     is the NAIF integer code for the instrument or
///           structure for which a segment is to be created.
///
///  REF      is the NAIF name for a reference frame relative to
///           which the pointing information for INST is
///           specified.
///
///  AVFLAG   is a logical flag which indicates whether or not
///           the segment will contain angular velocity.
///
///  SEGID    is the segment identifier.  A CK segment
///           identifier may contain up to 40 characters.
///
///  N        is the number of packets in the input packet
///           array.
///
///  SCLKDP   are the encoded spacecraft clock times associated
///           with each pointing instance. These times must be
///           strictly increasing.
///
///  PACKTS   contains a time-ordered array of data packets
///           representing the orientation of INST relative to
///           the frame REF. Each packet contains a SPICE-style
///           quaternion and optionally, depending on the
///           segment subtype, attitude derivative data, from
///           which a C-matrix and an angular velocity vector
///           may be derived.
///
///           See the discussion of quaternion styles in
///           $Particulars below.
///
///           The C-matrix represented by the Ith data packet is
///           a rotation matrix that transforms the components
///           of a vector expressed in the base frame specified
///           by REF to components expressed in the instrument
///           fixed frame at the time SCLKDP(I).
///
///           Thus, if a vector V has components x, y, z in the
///           base frame, then V has components x', y', z'
///           in the instrument fixed frame where:
///
///              [ x' ]     [          ] [ x ]
///              | y' |  =  |   CMAT   | | y |
///              [ z' ]     [          ] [ z ]
///
///
///           The attitude derivative information in PACKTS(I)
///           gives the angular velocity of the instrument fixed
///           frame at time SCLKDP(I) with respect to the
///           reference frame specified by REF.
///
///           The direction of an angular velocity vector gives
///           the right-handed axis about which the instrument
///           fixed reference frame is rotating. The magnitude
///           of the vector is the magnitude of the
///           instantaneous velocity of the rotation, in radians
///           per second.
///
///           Packet contents and the corresponding
///           interpolation methods depend on the segment
///           subtype, and are as follows:
///
///              Subtype 0:  Hermite interpolation, 8-element
///                          packets. Quaternion and quaternion
///                          derivatives only, no angular
///                          velocity vector provided.
///                          Quaternion elements are listed
///                          first, followed by derivatives.
///                          Angular velocity is derived from
///                          the quaternions and quaternion
///                          derivatives.
///
///              Subtype 1:  Lagrange interpolation, 4-element
///                          packets. Quaternion only. Angular
///                          velocity is derived by
///                          differentiating the interpolating
///                          polynomials.
///
///              Subtype 2:  Hermite interpolation, 14-element
///                          packets. Quaternion and angular
///                          angular velocity vector, as well as
///                          derivatives of each, are provided.
///                          The quaternion comes first, then
///                          quaternion derivatives, then
///                          angular velocity and its
///                          derivatives.
///
///              Subtype 3:  Lagrange interpolation, 7-element
///                          packets. Quaternion and angular
///                          velocity vector provided. The
///                          quaternion comes first.
///
///           Angular velocity is always specified relative to
///           the base frame.
///
///  RATE     is the nominal rate of the spacecraft clock
///           associated with INST. Units are seconds per
///           tick. RATE is used to scale angular velocity
///           to radians/second.
///
///  NINTS    is the number of intervals that the pointing
///           instances are partitioned into.
///
///  STARTS   are the start times of each of the interpolation
///           intervals. These times must be strictly increasing
///           and must coincide with times for which the segment
///           contains pointing.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None. See $Particulars for a description of the effect of this
///  routine.
/// ```
///
/// # Parameters
///
/// ```text
///  MAXDEG   is the maximum allowed degree of the interpolating
///           polynomial. If the value of MAXDEG is increased,
///           the SPICELIB routine CKPFS must be changed
///           accordingly. In particular, the size of the
///           record passed to CKRnn and CKEnn must be
///           increased, and comments describing the record size
///           must be changed.
/// ```
///
/// # Exceptions
///
/// ```text
///  If any of the following exceptions occur, this routine will
///  return without creating a new segment.
///
///  1)  If HANDLE is not the handle of a C-kernel opened for writing,
///      an error is signaled by a routine in the call tree of this
///      routine.
///
///  2)  If the last non-blank character of SEGID occurs past index
///      40, the error SPICE(SEGIDTOOLONG) is signaled.
///
///  3)  If SEGID contains any nonprintable characters, the error
///      SPICE(NONPRINTABLECHARS) is signaled.
///
///  4)  If the first encoded SCLK time is negative, the error
///      SPICE(INVALIDSCLKTIME) is signaled. If any subsequent times
///      are negative the error will be detected in exception (5).
///
///  5)  If the encoded SCLK times are not strictly increasing,
///      the error SPICE(TIMESOUTOFORDER) is signaled.
///
///  6)  If the name of the reference frame is not one of those
///      supported by the routine FRAMEX, the error
///      SPICE(INVALIDREFFRAME) is signaled.
///
///  7)  If the number of packets N is not at least 1, the error
///      SPICE(TOOFEWPACKETS) is signaled.
///
///  8)  If NINTS, the number of interpolation intervals, is less than
///      or equal to 0, the error SPICE(INVALIDNUMINTS) is signaled.
///
///  9)  If the encoded SCLK interval start times are not strictly
///      increasing, the error SPICE(TIMESOUTOFORDER) is signaled.
///
///  10) If an interval start time does not coincide with a time for
///      which there is an actual pointing instance in the segment, the
///      error SPICE(INVALIDSTARTTIME) is signaled.
///
///  11) This routine assumes that the rotation between adjacent
///      quaternions that are stored in the same interval has a
///      rotation angle of THETA radians, where
///
///         0  <=  THETA  <  pi.
///
///      The routines that evaluate the data in the segment produced
///      by this routine cannot distinguish between rotations of THETA
///      radians, where THETA is in the interval [0, pi), and
///      rotations of
///
///         THETA   +   2 * k * pi
///
///      radians, where k is any integer. These "large" rotations will
///      yield invalid results when interpolated. You must ensure that
///      the data stored in the segment will not be subject to this
///      sort of ambiguity.
///
///  12) If any quaternion has magnitude zero, the error
///      SPICE(ZEROQUATERNION) is signaled.
///
///  13) If the interpolation window size implied by DEGREE is not
///      even, the error SPICE(INVALIDDEGREE) is signaled. The window
///      size is DEGREE+1 for Lagrange subtypes and is (DEGREE+1)/2
///      for Hermite subtypes.
///
///  14) If an unrecognized subtype code is supplied, the error
///      SPICE(NOTSUPPORTED) is signaled.
///
///  15) If DEGREE is not at least 1 or is greater than MAXDEG, the
///      error SPICE(INVALIDDEGREE) is signaled.
///
///  16) If the segment descriptor bounds are out of order, the
///      error SPICE(BADDESCRTIMES) is signaled.
///
///  17) If there is no element of SCLKDP that lies between BEGTIM and
///      ENDTIM inclusive, the error SPICE(EMPTYSEGMENT) is signaled.
///
///  18) If RATE is zero, the error SPICE(INVALIDVALUE) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  A new type 5 CK segment is written to the CK file attached
///  to HANDLE.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine writes a CK type 5 data segment to the open CK
///  file according to the format described in the type 5 section of
///  the CK Required Reading. The CK file must have been opened with
///  write access.
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
///       +-                                                         -+
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
///       +-                                                         -+
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
///                +-             -+
///                |  0   -n3   n2 |
///                |               |
///      OMEGA  =  |  n3   0   -n1 |
///                |               |
///                | -n2   n1   0  |
///                +-             -+
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
///  Suppose that you have data packets and are prepared to produce
///  a segment of type 5 in a CK file.
///
///  The following code fragment could be used to add the new segment
///  to a previously opened CK file attached to HANDLE. The file must
///  have been opened with write access.
///
///     C
///     C     Create a segment identifier.
///     C
///           SEGID = 'MY_SAMPLE_CK_TYPE_5_SEGMENT'
///
///     C
///     C     Write the segment.
///     C
///           CALL CKW05 ( HANDLE, SUBTYP, DEGREE, BEGTIM, ENDTIM,
///          .             INST,   REF,    AVFLAG, SEGID,  N,
///          .             SCLKDP, PACKTS, RATE,   NINTS,  STARTS )
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
/// -    SPICELIB Version 3.0.1, 08-JUN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 3.0.0, 27-JAN-2014 (NJB)
///
///         Increased MAXDEG to 23 for compatibility with CK type 6.
///
/// -    SPICELIB Version 2.0.0, 08-FEB-2010 (NJB)
///
///         The check for non-unit quaternions has been replaced
///         with a check for zero-length quaternions.
///
/// -    SPICELIB Version 1.1.0, 26-FEB-2008 (NJB)
///
///         Updated header; added information about SPICE
///         quaternion conventions.
///
///         Minor typo in a long error message was corrected.
///
/// -    SPICELIB Version 1.0.1, 07-JAN-2005 (NJB)
///
///         Description in $Detailed_Input header section of
///         constraints on BEGTIM and ENDTIM was corrected.
///
/// -    SPICELIB Version 1.0.0, 30-AUG-2002 (NJB) (KRG) (JML) (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 2.0.0, 08-FEB-2010 (NJB)
///
///         The check for non-unit quaternions has been replaced
///         with a check for zero-length quaternions.
///
///         This change was made to accommodate CK generation,
///         via the non-SPICE utility MEX2KER, for European missions.
/// ```
pub fn ckw05(
    ctx: &mut SpiceContext,
    handle: i32,
    subtyp: i32,
    degree: i32,
    begtim: f64,
    endtim: f64,
    inst: i32,
    ref_: &str,
    avflag: bool,
    segid: &str,
    n: i32,
    sclkdp: &[f64],
    packts: &[f64],
    rate: f64,
    nints: i32,
    starts: &[f64],
) -> crate::Result<()> {
    CKW05(
        handle,
        subtyp,
        degree,
        begtim,
        endtim,
        inst,
        ref_.as_bytes(),
        avflag,
        segid.as_bytes(),
        n,
        sclkdp,
        packts,
        rate,
        nints,
        starts,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CKW05 ( Write CK segment, type 5 )
pub fn CKW05(
    HANDLE: i32,
    SUBTYP: i32,
    DEGREE: i32,
    BEGTIM: f64,
    ENDTIM: f64,
    INST: i32,
    REF: &[u8],
    AVFLAG: bool,
    SEGID: &[u8],
    N: i32,
    SCLKDP: &[f64],
    PACKTS: &[f64],
    RATE: f64,
    NINTS: i32,
    STARTS: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let SCLKDP = DummyArray::new(SCLKDP, 1..);
    let PACKTS = DummyArray::new(PACKTS, 1..);
    let STARTS = DummyArray::new(STARTS, 1..);
    let mut DC = StackArray::<f64, 2>::new(1..=ND);
    let mut DESCR = StackArray::<f64, 5>::new(1..=DSCSIZ);
    let mut ADDR: i32 = 0;
    let mut CHRCOD: i32 = 0;
    let mut I: i32 = 0;
    let mut IC = StackArray::<i32, 6>::new(1..=NI);
    let mut PACKSZ: i32 = 0;
    let mut REFCOD: i32 = 0;
    let mut WINSIZ: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Packet structure parameters
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
        CHKIN(b"CKW05", ctx)?;
    }

    //
    // Make sure that the number of packets is positive.
    //
    if (N < 1) {
        SETMSG(
            b"At least 1 packet is required for CK type 5. Number of packets supplied:  #",
            ctx,
        );
        ERRINT(b"#", N, ctx);
        SIGERR(b"SPICE(TOOFEWPACKETS)", ctx)?;
        CHKOUT(b"CKW05", ctx)?;
        return Ok(());
    }

    //
    // Make sure that there is a positive number of interpolation
    // intervals.
    //
    if (NINTS <= 0) {
        SETMSG(
            b"# is an invalid number of interpolation intervals for type 5.",
            ctx,
        );
        ERRINT(b"#", NINTS, ctx);
        SIGERR(b"SPICE(INVALIDNUMINTS)", ctx)?;
        CHKOUT(b"CKW05", ctx)?;
        return Ok(());
    }

    //
    // Get the NAIF integer code for the reference frame.
    //
    NAMFRM(REF, &mut REFCOD, ctx)?;

    if (REFCOD == 0) {
        SETMSG(b"The reference frame # is not supported.", ctx);
        ERRCH(b"#", REF, ctx);
        SIGERR(b"SPICE(INVALIDREFFRAME)", ctx)?;
        CHKOUT(b"CKW05", ctx)?;
        return Ok(());
    }

    //
    // Check to see if the segment identifier is too long.
    //
    if (LASTNB(SEGID) > SIDLEN) {
        SETMSG(b"Segment identifier contains more than 40 characters.", ctx);
        SIGERR(b"SPICE(SEGIDTOOLONG)", ctx)?;
        CHKOUT(b"CKW05", ctx)?;
        return Ok(());
    }

    //
    // Now check that all the characters in the segment identifier
    // can be printed.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = LASTNB(SEGID);
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            CHRCOD = intrinsics::ICHAR(fstr::substr(SEGID, I..=I));

            if ((CHRCOD < FPRINT) || (CHRCOD > LPRINT)) {
                SETMSG(
                    b"The segment identifier contains nonprintable characters",
                    ctx,
                );
                SIGERR(b"SPICE(NONPRINTABLECHARS)", ctx)?;
                CHKOUT(b"CKW05", ctx)?;
                return Ok(());
            }

            I += m3__;
        }
    }

    //
    // Now check that the encoded SCLK times are positive and strictly
    // increasing.
    //
    // Check that the first time is nonnegative.
    //
    if (SCLKDP[1] < 0.0) {
        SETMSG(b"The first SCLKDP time: # is negative.", ctx);
        ERRDP(b"#", SCLKDP[1], ctx);
        SIGERR(b"SPICE(INVALIDSCLKTIME)", ctx)?;
        CHKOUT(b"CKW05", ctx)?;
        return Ok(());
    }

    //
    // Now check that the times are ordered properly.
    //
    {
        let m1__: i32 = 2;
        let m2__: i32 = N;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            if (SCLKDP[I] <= SCLKDP[(I - 1)]) {
                SETMSG(b"The SCLKDP times are not strictly increasing. SCLKDP(#) = # and SCLKDP(#) = #.", ctx);
                ERRINT(b"#", I, ctx);
                ERRDP(b"#", SCLKDP[I], ctx);
                ERRINT(b"#", (I - 1), ctx);
                ERRDP(b"#", SCLKDP[(I - 1)], ctx);
                SIGERR(b"SPICE(TIMESOUTOFORDER)", ctx)?;
                CHKOUT(b"CKW05", ctx)?;
                return Ok(());
            }

            I += m3__;
        }
    }

    //
    // Now check that the interval start times are ordered properly.
    //
    {
        let m1__: i32 = 2;
        let m2__: i32 = NINTS;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            if (STARTS[I] <= STARTS[(I - 1)]) {
                SETMSG(b"The interval start times are not strictly increasing. STARTS(#) = # and STARTS(#) = #.", ctx);
                ERRINT(b"#", I, ctx);
                ERRDP(b"#", STARTS[I], ctx);
                ERRINT(b"#", (I - 1), ctx);
                ERRDP(b"#", STARTS[(I - 1)], ctx);
                SIGERR(b"SPICE(TIMESOUTOFORDER)", ctx)?;
                CHKOUT(b"CKW05", ctx)?;
                return Ok(());
            }

            I += m3__;
        }
    }

    //
    // Now make sure that all of the interval start times coincide with
    // one of the times associated with the actual pointing.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NINTS;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // We know the SCLKDP array is ordered, so a binary search is
            // ok.
            //
            if (BSRCHD(STARTS[I], N, SCLKDP.as_slice()) == 0) {
                SETMSG(
                    b"Interval start time number # is invalid. STARTS(#) = *",
                    ctx,
                );
                ERRINT(b"#", I, ctx);
                ERRINT(b"#", I, ctx);
                ERRDP(b"*", STARTS[I], ctx);
                SIGERR(b"SPICE(INVALIDSTARTTIME)", ctx)?;
                CHKOUT(b"CKW05", ctx)?;
                return Ok(());
            }

            I += m3__;
        }
    }

    //
    // Set the window, packet size and angular velocity flag, all of
    // which are functions of the subtype.
    //
    if (SUBTYP == C05TP0) {
        WINSIZ = ((DEGREE + 1) / 2);
        PACKSZ = C05PS0;
    } else if (SUBTYP == C05TP1) {
        WINSIZ = (DEGREE + 1);
        PACKSZ = C05PS1;
    } else if (SUBTYP == C05TP2) {
        WINSIZ = ((DEGREE + 1) / 2);
        PACKSZ = C05PS2;
    } else if (SUBTYP == C05TP3) {
        WINSIZ = (DEGREE + 1);
        PACKSZ = C05PS3;
    } else {
        SETMSG(b"CK type 5 subtype <#> is not supported.", ctx);
        ERRINT(b"#", SUBTYP, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"CKW05", ctx)?;
        return Ok(());
    }

    //
    // Make sure that the quaternions are non-zero. This is just
    // a check for uninitialized data.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = N;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // We have to address the quaternion explicitly, since the shape
            // of the packet array is not known at compile time.
            //
            ADDR = ((PACKSZ * (I - 1)) + QIDX);

            if VZEROG(PACKTS.subarray(ADDR), 4) {
                SETMSG(b"The quaternion at index # has magnitude zero.", ctx);
                ERRINT(b"#", I, ctx);
                SIGERR(b"SPICE(ZEROQUATERNION)", ctx)?;
                CHKOUT(b"CKW05", ctx)?;
                return Ok(());
            }

            I += m3__;
        }
    }

    //
    // Make sure that the degree of the interpolating polynomials is
    // in range.
    //
    if ((DEGREE < 1) || (DEGREE > MAXDEG)) {
        SETMSG(
            b"The interpolating polynomials have degree #; the valid degree range is [1, #]",
            ctx,
        );
        ERRINT(b"#", DEGREE, ctx);
        ERRINT(b"#", MAXDEG, ctx);
        SIGERR(b"SPICE(INVALIDDEGREE)", ctx)?;
        CHKOUT(b"CKW05", ctx)?;
        return Ok(());
    }

    //
    // Make sure that the window size is even.  If not, the input
    // DEGREE is incompatible with the subtype.
    //
    if ODD(WINSIZ) {
        SETMSG(b"The interpolating polynomials have degree #; for CK type 5, the degree must be equivalent to 3 mod 4 for Hermite interpolation and odd for for Lagrange interpolation.", ctx);
        ERRINT(b"#", DEGREE, ctx);
        SIGERR(b"SPICE(INVALIDDEGREE)", ctx)?;
        CHKOUT(b"CKW05", ctx)?;
        return Ok(());
    }

    //
    // If we made it this far, we're ready to start writing the segment.
    //
    // Create the segment descriptor.
    //
    // Assign values to the integer components of the segment descriptor.
    //
    IC[1] = INST;
    IC[2] = REFCOD;
    IC[3] = DTYPE;

    if AVFLAG {
        IC[4] = 1;
    } else {
        IC[4] = 0;
    }

    DC[1] = BEGTIM;
    DC[2] = ENDTIM;

    //
    // Make sure the descriptor times are in increasing order.
    //
    if (ENDTIM < BEGTIM) {
        SETMSG(b"Descriptor bounds are non-increasing: #:#", ctx);
        ERRDP(b"#", BEGTIM, ctx);
        ERRDP(b"#", ENDTIM, ctx);
        SIGERR(b"SPICE(BADDESCRTIMES)", ctx)?;
        CHKOUT(b"CKW05", ctx)?;
        return Ok(());
    }

    //
    // Make sure that at least one time tag lies between BEGTIM and
    // ENDTIM.  The first time tag not less than BEGTIM must exist
    // and must be less than or equal to ENDTIM.
    //
    I = LSTLTD(BEGTIM, N, SCLKDP.as_slice());

    if (I == N) {
        SETMSG(b"All time tags are less than segment start time #.", ctx);
        ERRDP(b"#", BEGTIM, ctx);
        SIGERR(b"SPICE(EMPTYSEGMENT)", ctx)?;
        CHKOUT(b"CKW05", ctx)?;
        return Ok(());
    } else if (SCLKDP[(I + 1)] > ENDTIM) {
        SETMSG(
            b"No time tags lie between the segment start time # and segment end time #",
            ctx,
        );
        ERRDP(b"#", BEGTIM, ctx);
        ERRDP(b"#", ENDTIM, ctx);
        SIGERR(b"SPICE(EMPTYSEGMENT)", ctx)?;
        CHKOUT(b"CKW05", ctx)?;
        return Ok(());
    }

    //
    // The clock rate must be non-zero.
    //
    if (RATE == 0.0) {
        SETMSG(b"The SCLK rate RATE was zero.", ctx);
        SIGERR(b"SPICE(INVALIDVALUE)", ctx)?;
        CHKOUT(b"CKW05", ctx)?;
        return Ok(());
    }

    //
    // Now pack the segment descriptor.
    //
    DAFPS(ND, NI, DC.as_slice(), IC.as_slice(), DESCR.as_slice_mut());

    //
    // Begin a new segment.
    //
    DAFBNA(HANDLE, DESCR.as_slice(), SEGID, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"CKW05", ctx)?;
        return Ok(());
    }

    //
    // The type 5 segment structure is eloquently described by this
    // diagram from the CK Required Reading:
    //
    //    +-----------------------+
    //    | Packet 1              |
    //    +-----------------------+
    //    | Packet 2              |
    //    +-----------------------+
    //                .
    //                .
    //                .
    //    +-----------------------+
    //    | Packet N              |
    //    +-----------------------+
    //    | Epoch 1               |
    //    +-----------------------+
    //    | Epoch 2               |
    //    +-----------------------+
    //                .
    //                .
    //                .
    //    +----------------------------+
    //    | Epoch N                    |
    //    +----------------------------+
    //    | Epoch 100                  | (First directory)
    //    +----------------------------+
    //                .
    //                .
    //                .
    //    +----------------------------+
    //    | Epoch ((N-1)/100)*100      | (Last directory)
    //    +----------------------------+
    //    | Start time 1               |
    //    +----------------------------+
    //    | Start time 2               |
    //    +----------------------------+
    //                .
    //                .
    //                .
    //    +----------------------------+
    //    | Start time M               |
    //    +----------------------------+
    //    | Start time 100             | (First interval start
    //    +----------------------------+  time directory)
    //                .
    //                .
    //                .
    //    +----------------------------+
    //    | Start time ((M-1)/100)*100 | (Last interval start
    //    +----------------------------+  time directory)
    //    | Seconds per tick           |
    //    +----------------------------+
    //    | Subtype code               |
    //    +----------------------------+
    //    | Window size                |
    //    +----------------------------+
    //    | Number of interp intervals |
    //    +----------------------------+
    //    | Number of packets          |
    //    +----------------------------+
    //
    //
    DAFADA(PACKTS.as_slice(), (N * PACKSZ), ctx)?;
    DAFADA(SCLKDP.as_slice(), N, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = ((N - 1) / DIRSIZ);
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            DAFADA(SCLKDP.subarray((DIRSIZ * I)), 1, ctx)?;

            I += m3__;
        }
    }

    //
    // Now add the interval start times.
    //
    DAFADA(STARTS.as_slice(), NINTS, ctx)?;

    //
    // And the directory of interval start times.  The directory of
    // start times will simply be every (DIRSIZ)th start time.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = ((NINTS - 1) / DIRSIZ);
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            DAFADA(STARTS.subarray((DIRSIZ * I)), 1, ctx)?;

            I += m3__;
        }
    }

    //
    // Add the SCLK rate, segment subtype, window size, interval
    // count, and packet count.
    //
    DAFADA(&[RATE], 1, ctx)?;
    DAFADA(&[(SUBTYP as f64)], 1, ctx)?;
    DAFADA(&[(WINSIZ as f64)], 1, ctx)?;
    DAFADA(&[(NINTS as f64)], 1, ctx)?;
    DAFADA(&[(N as f64)], 1, ctx)?;

    //
    // As long as nothing went wrong, end the segment.
    //
    if !FAILED(ctx) {
        DAFENA(ctx)?;
    }

    CHKOUT(b"CKW05", ctx)?;
    Ok(())
}
