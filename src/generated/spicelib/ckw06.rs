//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const MAXDEG: i32 = 23;
const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const C06TP0: i32 = 0;
const C06TP1: i32 = (C06TP0 + 1);
const C06TP2: i32 = (C06TP1 + 1);
const C06TP3: i32 = (C06TP2 + 1);
const C06NST: i32 = 4;
const C06PS0: i32 = 8;
const C06PS1: i32 = 4;
const C06PS2: i32 = 14;
const C06PS3: i32 = 7;
const C06MXZ: i32 = C06PS2;
const C06MNZ: i32 = C06PS1;
const MAXRSZ: i32 = (4 + ((MAXDEG + 1) * (C06PS3 + 1)));
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
const SIDLEN: i32 = 40;
const FPRINT: i32 = 32;
const LPRINT: i32 = 126;
const ND: i32 = 2;
const NI: i32 = 6;
const DSCSIZ: i32 = 5;
const DTYPE: i32 = 6;
const DIRSIZ: i32 = 100;
const QIDX: i32 = 1;

struct SaveVars {
    PKTSZS: StackArray<i32, 4>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PKTSZS = StackArray::<i32, 4>::new(0..=(C06NST - 1));

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(C06PS0),
                Val::I(C06PS1),
                Val::I(C06PS2),
                Val::I(C06PS3),
            ]
            .into_iter();
            PKTSZS
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { PKTSZS }
    }
}

/// CK, Write segment, type 6
///
/// Write a type 6 segment to a CK file.
///
/// # Required Reading
///
/// * [CK](crate::required_reading::ck)
/// * [DAF](crate::required_reading::daf)
/// * [NAIF_IDS](crate::required_reading::naif_ids)
/// * [SCLK](crate::required_reading::sclk)
/// * [SPC](crate::required_reading::spc)
/// * [TIME](crate::required_reading::time)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of a CK file open for writing.
///  INST       I   NAIF instrument ID code.
///  REF        I   Reference frame name.
///  AVFLAG     I   Flag indicating if the segment will contain angular
///                 velocity.
///  FIRST      I   Start time of interval covered by segment.
///  LAST       I   End time of interval covered by segment.
///  SEGID      I   Segment identifier.
///  NMINI      I   Number of mini-segments.
///  NPKTS      I   Array of packet counts of mini-segments.
///  SUBTPS     I   Array of segment subtypes of mini-segments.
///  DEGRES     I   Array of polynomial degrees of mini-segments.
///  PACKTS     I   Array of data packets of mini-segments.
///  RATES      I   Nominal SCLK rates in seconds per tick.
///  SCLKDP     I   Array of epochs of mini-segments.
///  IVLBDS     I   Mini-segment interval bounds.
///  SELLST     I   Interval selection flag.
///  MAXDEG     P   Maximum allowed degree of interpolating polynomial.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a CK file that has been opened
///           for writing.
///
///
///  INST     is a NAIF integer code associated with an
///           instrument or spacecraft structure whose
///           orientation is described by the segment to be
///           created. INST is treated by the SPICE frame
///           subsystem as a CK frame class ID (see the
///           Frames Required Reading for details).
///
///
///  AVFLAG   is a logical flag which indicates whether or not
///           the segment will contain angular velocity.
///
///
///  REF      is the NAIF name for a reference frame relative to
///           which the pointing (attitude) information for INST
///           is specified.
///
///  FIRST,
///  LAST     are, respectively, the bounds of the time interval
///           over which the segment defines the attitude of
///           INST. FIRST and LAST are encoded SCLK times.
///
///           FIRST must be greater than or equal to the first
///           mini-segment interval start time; LAST must be
///           less than or equal to the last mini-segment
///           interval stop time. See the description of IVLBDS
///           below.
///
///
///  SEGID    is the segment identifier. A CK segment
///           identifier may contain up to 40 characters.
///
///
///  NMINI    is the number of mini-segments comprised by
///           the input data. Each mini-segment contains data
///           that could be stored in a type 5 segment.
///           The parameters and data of a mini-segment are:
///
///              - a packet count
///              - a type 6 subtype
///              - an interpolating polynomial degree
///              - a nominal SCLK rate in seconds/tick
///              - a sequence of type 6 data packets
///              - a sequence of packet epochs
///
///           These inputs are described below.
///
///
///  NPKTS    is an array of packet counts. The Ith element of
///           NPKTS is the packet count of the Ith mini-segment.
///
///           NPKTS has dimension NMINI.
///
///
///  SUBTPS   is an array of type 6 subtypes. The Ith element of
///           SUBTPS is the subtype of the packets associated
///           with the Ith mini-segment.
///
///           SUBTPS has dimension NMINI.
///
///
///  DEGRES   is an array of interpolating polynomial degrees.
///           The Ith element of DEGRES is the polynomial degree
///           of the packets associated with the Ith
///           mini-segment.
///
///           For subtypes 0 and 2, interpolation degrees must be
///           equivalent to 3 mod 4, that is, they must be in
///           the set
///
///              { 3, 7, 11, ..., MAXDEG }
///
///           For subtypes 1 and 3, interpolation degrees must
///           be odd and must be in the range 1:MAXDEG.
///
///           DEGRES has dimension NMINI.
///
///
///  PACKTS   is an array of data packets representing the
///           orientation of INST relative to the frame REF. The
///           packets for a given mini-segment are stored
///           contiguously in increasing time order. The order
///           of the sets of packets for different mini-segments
///           is the same as the order of their corresponding
///           mini-segment intervals.
///
///           Each packet contains a SPICE-style quaternion and
///           optionally, depending on the segment subtype,
///           attitude derivative data, from which a C-matrix
///           and an angular velocity vector may be derived.
///
///           See the discussion of quaternion styles in
///           $Particulars below.
///
///           The C-matrix CMAT represented by the Ith data
///           packet is a rotation matrix that transforms the
///           components of a vector expressed in the base frame
///           specified by REF to components expressed in the
///           instrument fixed frame at the time SCLKDP(I).
///
///           Thus, if a vector V has components x, y, z in the
///           base frame, then V has components x', y', z'
///           in the instrument fixed frame where:
///
///              [ x' ]     [          ] [ x ]
///              | y' |  =  |   CMAT   | | y |
///              [ z' ]     [          ] [ z ]
///
///           Attitude derivative information either explicitly
///           contained in, or else derived from, PACKTS(I)
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
///                          velocity vector, as well as
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
///           Units of the input data are:
///
///              Quaternions                unitless
///              Quaternion derivatives     1/TDB second
///              Angular velocity           radians/TDB second
///              Angular acceleration       radians/TDB second**2
///
///           For the Hermite subtypes (0 and 2), quaternion
///           representations must be selected so that, for
///           consecutive quaternions Q(I) and Q(I+1) in a
///           mini-segment, the distance between Q and Q(I+1) is
///           less than the distance between Q and -Q(I+1). The
///           Lagrange subtypes do not have this restriction.
///
///
///  RATES    is an array of nominal rates of the spacecraft
///           clock associated with INST. The Ith element of
///           rates is the clock rate for the packets associated
///           with the Ith mini-segment. Units are seconds per
///           tick. Spacecraft clock rates are used to scale
///           angular velocity to radians/second.
///
///
///  SCLKDP   is an array containing epochs for all input
///           mini-segments. The epochs have a one-to-one
///           relationship with the packets in the input
///           packet array. The epochs are encoded SCLK times.
///
///           The epochs for a given mini-segment are stored
///           contiguously in increasing order. The order of the
///           sets of epochs for different mini-segments is the
///           same as the order of their corresponding
///           mini-segment intervals.
///
///           For each mini-segment, "padding" is allowed: the
///           sequence of epochs for that mini-segment may start
///           before the corresponding mini-segment interval
///           start time and end after the corresponding
///           mini-segment interval stop time. Padding is used
///           to control behavior of interpolating polynomials
///           near mini-segment interval boundaries.
///
///           Due to possible use of padding, the elements of
///           SCLKDP, taken as a whole, might not be in
///           increasing order.
///
///
///  IVLBDS   is an array of mini-segment interval boundary
///           times. This array is a strictly increasing list of
///           the mini-segment interval start times, to which
///           the end time for the last interval is appended.
///           The interval bounds are encoded SCLK times.
///
///           The Ith mini-segment interval is the time
///           coverage interval of the Ith mini-segment (see the
///           description of NPKTS above).
///
///           For each mini-segment, the corresponding
///           mini-segment interval's start time is greater
///           than or equal to the mini-segment's first epoch.
///           The interval's stop time may exceed the
///           mini-segment's last epoch, allowing a single
///           coverage gap to exist between a mini-segment's
///           last epoch and its interval stop time.
///
///           The "interpolation interval" of the ith
///           mini-segment is contained in the ith mini-segment
///           interval: the interpolation interval extends from
///           IVLBDS(I) to the minimum of IVLBDS(I+1) and the
///           last epoch of the mini-segment.
///
///           For each mini-segment interval other than the
///           last, the interval's coverage stop time coincides
///           with the coverage start time of the next interval.
///
///           IVLBDS has dimension NMINI+1.
///
///
///  SELLST   is a logical flag indicating to the CK type 6
///           segment reader CKR06 how to select the
///           mini-segment interval when a request time
///           coincides with a time boundary shared by two
///           mini-segment intervals. When SELLST ("select
///           last") is .TRUE., the later interval is selected;
///           otherwise the earlier interval is selected.
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
///           polynomial.
///
///           See the INCLUDE file ck06.inc for the value of
///           MAXDEG.
/// ```
///
/// # Exceptions
///
/// ```text
///  If any of the following exceptions occur, this routine will
///  return without creating a new segment.
///
///
///  1)  If FIRST is greater than LAST, the error
///      SPICE(BADDESCRTIMES) is signaled.
///
///  2)  If REF is not a recognized name, the error
///      SPICE(INVALIDREFFRAME) is signaled.
///
///  3)  If the last non-blank character of SEGID occurs past index
///      40, the error SPICE(SEGIDTOOLONG) is signaled.
///
///  4)  If SEGID contains any nonprintable characters, the error
///      SPICE(NONPRINTABLECHARS) is signaled.
///
///  5)  If NMINI is not at least 1, the error SPICE(INVALIDCOUNT)
///      is signaled.
///
///  6)  If the elements of the array IVLBDS are not in strictly
///      increasing order, the error SPICE(BOUNDSOUTOFORDER) is
///      signaled.
///
///  7)  If the first interval start time IVLBDS(1) is greater than
///      FIRST, or if the last interval end time IVLBDS(NMINI+1) is
///      less than LAST, the error SPICE(COVERAGEGAP) is signaled.
///
///  8)  If any packet count in the array NPKTS is not at least 2, the
///      error SPICE(TOOFEWPACKETS) is signaled.
///
///  9)  If any subtype code in the array SUBTPS is not recognized,
///      the error SPICE(INVALIDSUBTYPE) is signaled.
///
///  10) If any interpolation degree in the array DEGRES is not at
///      least 1 or is greater than MAXDEG, the error
///      SPICE(INVALIDDEGREE) is signaled.
///
///  11) If the window size implied by any element of the array DEGRES
///      is odd, the error SPICE(BADWINDOWSIZE) is signaled.
///
///  12) If the elements of the array SCLKDP corresponding to a given
///      mini-segment are not in strictly increasing order, the error
///      SPICE(TIMESOUTOFORDER) is signaled.
///
///  13) If the first epoch of a mini-segment exceeds the start time
///      of the associated mini-segment interval, or if the last
///      epoch of a mini-segment is less than the interval start
///      time, the error SPICE(BOUNDSDISAGREE) is signaled. However,
///      the last epoch of a mini-segment may be less than the end
///      time of the corresponding mini-segment interval.
///
///  14) If any quaternion has magnitude zero, the error
///      SPICE(ZEROQUATERNION) is signaled.
///
///  15) If an error occurs while writing the output segment, the error
///      is signaled by a routine in the call tree of this routine.
///
///  16) This routine assumes that the rotation between adjacent
///      quaternions that are stored in the same interval has a
///      rotation angle of THETA radians, where
///
///         0  <  THETA  <  pi.
///            _
///
///      The routines that evaluate the data in the segment produced
///      by this routine cannot distinguish between rotations of THETA
///      radians, where THETA is in the interval [0, pi), and
///      rotations of
///
///         THETA   +   2 * k * pi
///
///      radians, where k is any integer. These "large" rotations will
///      yield invalid results when interpolated. The segment creator
///      must ensure that the data stored in the segment will not be
///      subject to this sort of ambiguity.
///
///  17) For the Hermite subtypes (0 and 2), quaternion
///      representations must be selected so that, for consecutive
///      quaternions Q(I) and Q(I+1) in a mini-segment, the distance
///      between Q and Q(I+1) is less than the distance between Q and
///      -Q(I+1).
///
///      If a pair of quaternions violating this condition is found in
///      the input array PACKTS, the error SPICE(BADQUATSIGN) is
///      signaled.
///
///  18) If any element of the input RATES array is non-positive, the
///      error SPICE(INVALIDSCLKRATE) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  A new type 6 CK segment is written to the CK file attached
///  to HANDLE.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine writes a CK type 6 data segment to the open CK
///  file according to the format described in the type 6 section of
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
///  Suppose that you have states and are prepared to produce
///  a segment of type 6 in a CK file.
///
///  The following code fragment could be used to add the new segment
///  to a previously opened CK file attached to HANDLE. The file must
///  have been opened with write access.
///
///     C
///     C     Create a segment identifier.
///     C
///           SEGID = 'MY_SAMPLE_CK_TYPE_6_SEGMENT'
///
///     C
///     C     Write the segment.
///     C
///           CALL CKW06 ( HANDLE,  INST,    REF,     AVFLAG,
///          .             FIRST,   LAST,    SEGID,   NMINI,
///          .             NPKTS,   SUBTPS,  DEGRES,  PACKTS,
///          .             RATES,   SCLKDP,  IVLBDS,  SELLST  )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.0.1, 02-JUN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.0.0, 11-AUG-2015 (NJB)
///
///         Added check for invalid SCLK rates.
///
///         Corrected error in header $Exceptions section: changed
///         subscript N+1 to NMINI+1. Corrected typo in description
///         of subtype 2 data. Added mention of angular acceleration
///         units.
///
/// -    SPICELIB Version 1.0.0, 14-MAR-2014 (NJB) (BVS)
/// ```
pub fn ckw06(
    ctx: &mut SpiceContext,
    handle: i32,
    inst: i32,
    ref_: &str,
    avflag: bool,
    first: f64,
    last: f64,
    segid: &str,
    nmini: i32,
    npkts: &[i32],
    subtps: &[i32],
    degres: &[i32],
    packts: &[f64],
    rates: &[f64],
    sclkdp: &[f64],
    ivlbds: &[f64],
    sellst: bool,
) -> crate::Result<()> {
    CKW06(
        handle,
        inst,
        ref_.as_bytes(),
        avflag,
        first,
        last,
        segid.as_bytes(),
        nmini,
        npkts,
        subtps,
        degres,
        packts,
        rates,
        sclkdp,
        ivlbds,
        sellst,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CKW06 ( CK, Write segment, type 6 )
pub fn CKW06(
    HANDLE: i32,
    INST: i32,
    REF: &[u8],
    AVFLAG: bool,
    FIRST: f64,
    LAST: f64,
    SEGID: &[u8],
    NMINI: i32,
    NPKTS: &[i32],
    SUBTPS: &[i32],
    DEGRES: &[i32],
    PACKTS: &[f64],
    RATES: &[f64],
    SCLKDP: &[f64],
    IVLBDS: &[f64],
    SELLST: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let NPKTS = DummyArray::new(NPKTS, 1..);
    let SUBTPS = DummyArray::new(SUBTPS, 1..);
    let DEGRES = DummyArray::new(DEGRES, 1..);
    let PACKTS = DummyArray::new(PACKTS, 1..);
    let RATES = DummyArray::new(RATES, 1..);
    let SCLKDP = DummyArray::new(SCLKDP, 1..);
    let IVLBDS = DummyArray::new(IVLBDS, 1..);
    let mut DC = StackArray::<f64, 2>::new(1..=ND);
    let mut DESCR = StackArray::<f64, 5>::new(1..=DSCSIZ);
    let mut PREVQ = StackArray::<f64, 4>::new(0..=3);
    let mut Q = StackArray::<f64, 4>::new(0..=3);
    let mut QNEG = StackArray::<f64, 4>::new(0..=3);
    let mut ADDR: i32 = 0;
    let mut BEPIX: i32 = 0;
    let mut CHRCOD: i32 = 0;
    let mut EEPIX: i32 = 0;
    let mut IC = StackArray::<i32, 6>::new(1..=NI);
    let mut ISEL: i32 = 0;
    let mut K: i32 = 0;
    let mut MINISZ: i32 = 0;
    let mut NDIR: i32 = 0;
    let mut PKTBEG: i32 = 0;
    let mut PKTDSZ: i32 = 0;
    let mut PKTEND: i32 = 0;
    let mut PKTSIZ: i32 = 0;
    let mut REFCOD: i32 = 0;
    let mut SEGBEG: i32 = 0;
    let mut SEGEND: i32 = 0;
    let mut SUBTYP: i32 = 0;
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
    // Saved values
    //

    //
    // Initial values
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"CKW06", ctx)?;

    //
    // Start with a parameter compatibility check.
    //
    if (CKMRSZ < MAXRSZ) {
        SETMSG(
            b"CK type 6 record size is #, but CKPFS record size is #.is #.",
            ctx,
        );
        ERRINT(b"#", MAXRSZ, ctx);
        ERRINT(b"#", CKMRSZ, ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"CKW06", ctx)?;
        return Ok(());
    }

    //
    // Make sure the segment descriptor bounds are
    // correctly ordered.
    //
    if (LAST < FIRST) {
        SETMSG(
            b"Segment start time is #; stop time is #; bounds must be in nondecreasing order.",
            ctx,
        );
        ERRDP(b"#", FIRST, ctx);
        ERRDP(b"#", LAST, ctx);
        SIGERR(b"SPICE(BADDESCRTIMES)", ctx)?;
        CHKOUT(b"CKW06", ctx)?;
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
        CHKOUT(b"CKW06", ctx)?;
        return Ok(());
    }

    //
    // Check to see if the segment identifier is too long.
    //
    if (LASTNB(SEGID) > SIDLEN) {
        SETMSG(b"Segment identifier contains more than 40 characters.", ctx);
        SIGERR(b"SPICE(SEGIDTOOLONG)", ctx)?;
        CHKOUT(b"CKW06", ctx)?;
        return Ok(());
    }

    //
    // Now check that all the characters in the segment identifier
    // can be printed.
    //
    for I in 1..=LASTNB(SEGID) {
        CHRCOD = intrinsics::ICHAR(fstr::substr(SEGID, I..=I));

        if ((CHRCOD < FPRINT) || (CHRCOD > LPRINT)) {
            SETMSG(
                b"The segment identifier contains nonprintable characters",
                ctx,
            );
            SIGERR(b"SPICE(NONPRINTABLECHARS)", ctx)?;
            CHKOUT(b"CKW06", ctx)?;
            return Ok(());
        }
    }

    //
    // The mini-segment count must be positive.
    //
    if (NMINI < 1) {
        SETMSG(
            b"Mini-segment count was #; this count must be positive.",
            ctx,
        );
        ERRINT(b"#", NMINI, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"CKW06", ctx)?;
        return Ok(());
    }

    //
    // Make sure the interval bounds form a strictly
    // increasing sequence.
    //
    // Note that there are NMINI+1 bounds.
    //
    for I in 1..=NMINI {
        if (IVLBDS[I] >= IVLBDS[(I + 1)]) {
            SETMSG(b"Mini-segment interval bounds at indices # and # are # and # respectively. The difference is #. The bounds are required to be strictly increasing.", ctx);
            ERRINT(b"#", I, ctx);
            ERRINT(b"#", (I + 1), ctx);
            ERRDP(b"#", IVLBDS[I], ctx);
            ERRDP(b"#", IVLBDS[(I + 1)], ctx);
            ERRDP(b"#", (IVLBDS[(I + 1)] - IVLBDS[I]), ctx);
            SIGERR(b"SPICE(BOUNDSOUTOFORDER)", ctx)?;
            CHKOUT(b"CKW06", ctx)?;
            return Ok(());
        }
    }

    //
    // Make sure the time span of the descriptor doesn't extend
    // beyond the span of the interval bounds.
    //
    if ((FIRST < IVLBDS[1]) || (LAST > IVLBDS[(NMINI + 1)])) {
        SETMSG(b"First mini-segment interval start time is #; segment start time is #; segment stop time is #; last mini-segment interval stop time is #. This sequence of times is required to be non-decreasing: segment coverage must be contained within the union of the mini-segment intervals.", ctx);
        ERRDP(b"#", IVLBDS[1], ctx);
        ERRDP(b"#", FIRST, ctx);
        ERRDP(b"#", LAST, ctx);
        ERRDP(b"#", IVLBDS[(NMINI + 1)], ctx);
        SIGERR(b"SPICE(COVERAGEGAP)", ctx)?;
        CHKOUT(b"CKW06", ctx)?;
        return Ok(());
    }

    //
    // Check the input data before writing to the file.
    //
    // This order of operations entails some redundant
    // calculations, but it allows for rapid error
    // detection.
    //
    // Initialize the mini-segment packet array indices,
    // and those of the mini-segment epoch array as well.
    //
    PKTBEG = 0;
    PKTEND = 0;

    BEPIX = 0;
    EEPIX = 0;

    for I in 1..=NMINI {
        //
        // First, just make sure the packet count for the current
        // mini-segment is at least two. This check reduces our chances
        // of a subscript range violation.
        //
        // Check the number of packets.
        //
        if (NPKTS[I] < 2) {
            SETMSG(b"At least 2 packets are required for CK type 6. Number of packets supplied was # in mini-segment at index #.", ctx);
            ERRINT(b"#", NPKTS[I], ctx);
            ERRINT(b"#", I, ctx);
            SIGERR(b"SPICE(TOOFEWPACKETS)", ctx)?;
            CHKOUT(b"CKW06", ctx)?;
            return Ok(());
        }

        //
        // Set the packet size, which is a function of the subtype. Also
        // set the window size. First check the subtype, which will be
        // used as an array index.
        //
        SUBTYP = SUBTPS[I];

        if ((SUBTYP < 0) || (SUBTYP > (C06NST - 1))) {
            SETMSG(
                b"Unexpected CK type 6 subtype # found in mini-segment #.",
                ctx,
            );
            ERRINT(b"#", SUBTYP, ctx);
            ERRINT(b"#", I, ctx);
            SIGERR(b"SPICE(INVALIDSUBTYPE)", ctx)?;
            CHKOUT(b"CKW06", ctx)?;
            return Ok(());
        }

        PKTSIZ = save.PKTSZS[SUBTYP];

        if ODD(SUBTYP) {
            WINSIZ = (DEGRES[I] + 1);
        } else {
            WINSIZ = ((DEGRES[I] + 1) / 2);
        }

        //
        // Make sure the SCLK rates in this mini-segment are positive.
        //
        if (RATES[I] <= 0.0) {
            SETMSG(b"SCLK rate at index # was #; rate must be positive.", ctx);
            ERRINT(b"#", I, ctx);
            ERRDP(b"#", RATES[I], ctx);
            SIGERR(b"SPICE(INVALIDSCLKRATE)", ctx)?;
            CHKOUT(b"CKW06", ctx)?;
            return Ok(());
        }

        //
        // Update the packet range pointers for this mini-segment.
        //
        PKTBEG = (PKTEND + 1);
        PKTEND = ((PKTBEG - 1) + (NPKTS[I] * PKTSIZ));

        //
        // Make sure that the degree of the interpolating polynomials is
        // in range.
        //
        if ((DEGRES[I] < 1) || (DEGRES[I] > MAXDEG)) {
            SETMSG(b"The interpolating polynomials of mini-segment # have degree #; the valid degree range is [1, #]", ctx);
            ERRINT(b"#", I, ctx);
            ERRINT(b"#", DEGRES[I], ctx);
            ERRINT(b"#", MAXDEG, ctx);
            SIGERR(b"SPICE(INVALIDDEGREE)", ctx)?;
            CHKOUT(b"CKW06", ctx)?;
            return Ok(());
        }

        //
        // Make sure that the window size is even.
        //
        if ODD(WINSIZ) {
            SETMSG(b"The interpolating polynomials of mini-segment # have window size # and degree # for CK type 6. The mini-segment subtype is #. The degree must be equivalent to 3 mod 4 for subtypes 0 or 2 (Hermite interpolation) and odd for subtypes 1 or 3 (Lagrange interpolation).", ctx);
            ERRINT(b"#", I, ctx);
            ERRINT(b"#", WINSIZ, ctx);
            ERRINT(b"#", DEGRES[I], ctx);
            ERRINT(b"#", SUBTPS[I], ctx);
            SIGERR(b"SPICE(BADWINDOWSIZE)", ctx)?;
            CHKOUT(b"CKW06", ctx)?;
            return Ok(());
        }

        //
        // Make sure the epochs of the Ith mini-segment form a
        // strictly increasing sequence.
        //
        // To start out, determine the indices of the epoch sequence
        // of the Ith mini-segment. We'll call the begin and end
        // epoch indices BEPIX and EEPIX respectively.
        //
        BEPIX = (EEPIX + 1);
        EEPIX = ((BEPIX - 1) + NPKTS[I]);

        for J in 1..=(NPKTS[I] - 1) {
            K = ((BEPIX + J) - 1);

            if (SCLKDP[K] >= SCLKDP[(K + 1)]) {
                SETMSG(b"In mini-segment #, epoch # having mini-segment-relative index # and array-relative index # is greater than or equal to its successor #.", ctx);
                ERRINT(b"#", I, ctx);
                ERRDP(b"#", SCLKDP[K], ctx);
                ERRINT(b"#", J, ctx);
                ERRINT(b"#", K, ctx);
                ERRDP(b"#", SCLKDP[(K + 1)], ctx);
                SIGERR(b"SPICE(TIMESOUTOFORDER)", ctx)?;
                CHKOUT(b"CKW06", ctx)?;
                return Ok(());
            }
        }

        //
        // Make sure that the span of the input epochs of the Ith
        // mini-segment includes the start of the Ith mini-segment
        // interval. Note that the stop time need not be covered, since
        // gaps are allowed at the right ends of mini-segment intervals.
        //
        if (SCLKDP[BEPIX] > IVLBDS[I]) {
            SETMSG(
                b"Mini-segment interval # start time # precedes mini-segment\'s first epoch #.",
                ctx,
            );
            ERRINT(b"#", I, ctx);
            ERRDP(b"#", IVLBDS[I], ctx);
            ERRDP(b"#", SCLKDP[BEPIX], ctx);
            SIGERR(b"SPICE(BOUNDSDISAGREE)", ctx)?;
            CHKOUT(b"CKW06", ctx)?;
            return Ok(());
        } else if (SCLKDP[EEPIX] < IVLBDS[I]) {
            SETMSG(
                b"Mini-segment interval # start time # follows mini-segment\'s last epoch #.",
                ctx,
            );
            ERRINT(b"#", I, ctx);
            ERRDP(b"#", IVLBDS[I], ctx);
            ERRDP(b"#", SCLKDP[EEPIX], ctx);
            SIGERR(b"SPICE(BOUNDSDISAGREE)", ctx)?;
            CHKOUT(b"CKW06", ctx)?;
            return Ok(());
        }

        //
        // Make sure that the quaternions are non-zero. This is just a
        // check for uninitialized data.
        //
        // For the Hermite subtypes, make sure quaternions are suitable
        // for interpolation.
        //
        for J in 1..=NPKTS[I] {
            //
            // We have to address the quaternion explicitly, since the
            // shape of the packet array is not known at compile time.
            //
            ADDR = (((PKTBEG + (PKTSIZ * (J - 1))) - 1) + QIDX);

            if VZEROG(PACKTS.subarray(ADDR), 4) {
                SETMSG(
                    b"The quaternion in packet # within mini-segment # has magnitude zero.",
                    ctx,
                );
                ERRINT(b"#", J, ctx);
                ERRINT(b"#", I, ctx);
                SIGERR(b"SPICE(ZEROQUATERNION)", ctx)?;
                CHKOUT(b"CKW06", ctx)?;
                return Ok(());
            }

            //
            // For the Hermite subtypes, each quaternion must be closer
            // than its negative to its predecessor in the quaternion
            // sequence.
            //

            if ((J > 1) && EVEN(SUBTYP)) {
                //
                // Compare the distance between the current quaternion
                // and its predecessor vs the distance between the
                // negative of the current quaternion and its predecessor.
                //
                MOVED(PACKTS.subarray(ADDR), 4, Q.as_slice_mut());
                MOVED(PACKTS.subarray((ADDR - PKTSIZ)), 4, PREVQ.as_slice_mut());
                VMINUG(Q.as_slice(), 4, QNEG.as_slice_mut());

                if (VDISTG(PREVQ.as_slice(), QNEG.as_slice(), 4)
                    < VDISTG(PREVQ.as_slice(), Q.as_slice(), 4))
                {
                    SETMSG(b"The quaternion in packet # within mini-segment # is farther than its negative from its predecessor at index #. This makes the quaternion sequence unsuitable for Hermite interpolation. The quaternions, and if applicable, their derivatives, must be adjusted before they are passed to this routine.", ctx);
                    ERRINT(b"#", J, ctx);
                    ERRINT(b"#", I, ctx);
                    ERRINT(b"#", (J - 1), ctx);
                    SIGERR(b"SPICE(BADQUATSIGN)", ctx)?;
                    CHKOUT(b"CKW06", ctx)?;
                    return Ok(());
                }
            }
        }
    }

    //
    // If we made it this far, we're ready to start writing the segment.
    //
    // The type 6 segment structure is eloquently described by this
    // diagram from the CK Required Reading:
    //
    //    +---------------------------------------+
    //    | Mini-segment 1                        |
    //    +---------------------------------------+
    //          .
    //          .
    //          .
    //    +---------------------------------------+
    //    | Mini-segment N                        |
    //    +---------------------------------------+
    //    | Mini-segment interval 1 start time    |
    //    +---------------------------------------+
    //          .
    //          .
    //          .
    //    +---------------------------------------+
    //    | Mini-segment interval N start time    |
    //    +---------------------------------------+
    //    | Mini-segment interval N stop time     |
    //    +---------------------------------------+
    //    | Mini-seg. interval start time 100     | (First interval
    //    +---------------------------------------+  directory)
    //          .
    //          .
    //          .
    //    +---------------------------------------+
    //    | Mini-seg. ival. start time (N/100)*100| (Last interval
    //    +---------------------------------------+  directory)
    //    | Mini-segment 1 start pointer          |
    //    +---------------------------------------+
    //          .
    //          .
    //          .
    //    +---------------------------------------+
    //    | Mini-segment N start pointer          |
    //    +---------------------------------------+
    //    | Mini-segment N stop pointer + 1       |
    //    +---------------------------------------+
    //    | Boundary choice flag                  |
    //    +---------------------------------------+
    //    | Number of intervals                   |
    //    +---------------------------------------+
    //
    // CK type 6 mini-segments have the following structure:
    //
    //    +-----------------------+
    //    | Packet 1              |
    //    +-----------------------+
    //                .
    //                .
    //                .
    //    +-----------------------+
    //    | Packet M              |
    //    +-----------------------+
    //    | Epoch 1               |
    //    +-----------------------+
    //                .
    //                .
    //                .
    //    +-----------------------+
    //    | Epoch M               |
    //    +-----------------------+
    //    | Epoch 100             | (First time tag directory)
    //    +-----------------------+
    //                .
    //                .
    //                .
    //    +-----------------------+
    //    | Epoch ((M-1)/100)*100 | (Last time tag directory)
    //    +-----------------------+
    //    | Clock rate (sec/tick) |
    //    +-----------------------+
    //    | Subtype code          |
    //    +-----------------------+
    //    | Window size           |
    //    +-----------------------+
    //    | Number of packets     |
    //    +-----------------------+
    //
    // Note that the set of parameters at the end of a mini-segment does
    // not contain an mini-segment interval count. This is because,
    // unlike a CK type 5 segment, a CK type 6 segment can contain at
    // most one gap. If present, the gap is located at the end of
    // mini-segment's mini-segment interval.
    //
    // Create the segment descriptor. We don't use CKPDS because
    // that routine doesn't allow creation of a singleton segment.
    //
    IC[1] = INST;
    IC[2] = REFCOD;
    IC[3] = DTYPE;

    if AVFLAG {
        IC[4] = 1;
    } else {
        IC[4] = 0;
    }

    DC[1] = FIRST;
    DC[2] = LAST;

    DAFPS(ND, NI, DC.as_slice(), IC.as_slice(), DESCR.as_slice_mut());

    //
    // Begin a new segment.
    //
    DAFBNA(HANDLE, DESCR.as_slice(), SEGID, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"CKW06", ctx)?;
        return Ok(());
    }

    //
    // Re-initialize the mini-segment packet array indices,
    // and those of the mini-segment epoch array as well.
    //
    PKTBEG = 0;
    PKTEND = 0;

    BEPIX = 0;
    EEPIX = 0;

    //
    // Write data for each mini-segment to the file.
    //
    for I in 1..=NMINI {
        //
        // Set the packet size, which is a function of the subtype.
        //
        SUBTYP = SUBTPS[I];

        PKTSIZ = save.PKTSZS[SUBTYP];

        if ODD(SUBTYP) {
            WINSIZ = (DEGRES[I] + 1);
        } else {
            WINSIZ = ((DEGRES[I] + 1) / 2);
        }

        //
        // Now that we have the packet size, we can compute
        // mini-segment packet index range. We'll let PKTDSZ
        // be the total count of packet data entries for this
        // mini-segment.
        //
        PKTDSZ = (NPKTS[I] * PKTSIZ);

        PKTBEG = (PKTEND + 1);
        PKTEND = ((PKTBEG - 1) + PKTDSZ);

        //
        // At this point, we're read to start writing the
        // current mini-segment to the file. Start with the
        // packet data.
        //
        DAFADA(PACKTS.subarray(PKTBEG), PKTDSZ, ctx)?;

        //
        // Write the epochs for this mini-segment.
        //
        BEPIX = (EEPIX + 1);
        EEPIX = ((BEPIX - 1) + NPKTS[I]);

        DAFADA(SCLKDP.subarray(BEPIX), NPKTS[I], ctx)?;

        //
        // Compute the number of epoch directories for the
        // current mini-segment.
        //
        NDIR = ((NPKTS[I] - 1) / DIRSIZ);

        //
        // Write the epoch directories to the segment.
        //
        for J in 1..=NDIR {
            K = ((BEPIX - 1) + (J * DIRSIZ));

            DAFADA(SCLKDP.subarray(K), 1, ctx)?;
        }

        //
        // Write the mini-segment's SCLK rate, subtype, window size, and
        // packet count to the segment.
        //
        DAFADA(RATES.subarray(I), 1, ctx)?;
        DAFADA(&[(SUBTPS[I] as f64)], 1, ctx)?;
        DAFADA(&[(WINSIZ as f64)], 1, ctx)?;
        DAFADA(&[(NPKTS[I] as f64)], 1, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"CKW06", ctx)?;
            return Ok(());
        }
    }

    //
    // We've finished writing the mini-segments.
    //
    // Next write the mini-segment interval bounds.
    //
    DAFADA(IVLBDS.as_slice(), (NMINI + 1), ctx)?;

    //
    // Create and write directories for the interval
    // bounds.
    //
    // The directory count is the interval bound count
    // (N+1), minus 1, divided by the directory size.
    //
    NDIR = (NMINI / DIRSIZ);

    for I in 1..=NDIR {
        DAFADA(IVLBDS.subarray((DIRSIZ * I)), 1, ctx)?;
    }

    //
    // Now we compute and write the start/stop pointers
    // for each mini-segment.
    //
    // The pointers are relative to the DAF address
    // preceding the segment. For example, a pointer
    // to the first DAF address in the segment has
    // value 1.
    //
    SEGEND = 0;

    for I in 1..=NMINI {
        //
        // Set the packet size, which is a function of the subtype. Also
        // set the window size. First check the subtype, which will be
        // used as an array index.
        //
        PKTSIZ = save.PKTSZS[SUBTPS[I]];

        //
        // In order to compute the end pointer of the current
        // mini-segment, we must compute the size, in terms
        // of DAF addresses, of this mini-segment. The formula
        // for the size is
        //
        //     size =     n_packets * packet_size
        //             +  n_epochs
        //             +  n_epoch_directories
        //             +  4
        //
        //          =     n_packets * ( packet_size + 1 )
        //             +  ( n_packets - 1 ) / DIRSIZ
        //             +  4
        //
        MINISZ = (((NPKTS[I] * (PKTSIZ + 1)) + ((NPKTS[I] - 1) / DIRSIZ)) + 4);

        SEGBEG = (SEGEND + 1);
        SEGEND = ((SEGBEG + MINISZ) - 1);

        //
        // Write the mini-segment begin pointer.
        //
        // After the loop terminates, the final end pointer, incremented
        // by 1, will be written.
        //
        DAFADA(&[(SEGBEG as f64)], 1, ctx)?;
    }

    //
    // Write the last mini-segment end pointer, incremented by one.
    // SEGEND was computed on the last iteration of the above loop.
    //
    DAFADA(&[((SEGEND + 1) as f64)], 1, ctx)?;

    //
    // Write out the interval selection flag. The input
    // boolean value is represented by a numeric constant.
    //
    if SELLST {
        ISEL = ITRUE;
    } else {
        ISEL = IFALSE;
    }

    DAFADA(&[(ISEL as f64)], 1, ctx)?;

    //
    // Write the mini-segment/mini-segment interval count.
    //
    DAFADA(&[(NMINI as f64)], 1, ctx)?;

    //
    // End the segment.
    //
    DAFENA(ctx)?;

    CHKOUT(b"CKW06", ctx)?;
    Ok(())
}
