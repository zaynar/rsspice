//! #  Rotation
//!
//!  Last revised on 2017 MAR 09 by N. J. Bachman.
//!
//!  
//!
//!
//!  
//! ##  Abstract
//!
//!  The SPICE rotation routines manipulate and convert between different
//!    representations of rotation transformations: matrices, quaternions,
//!    Euler angles, and axis-angle pairs.
//!
//!  
//!
//!
//!  
//! ##  Introduction
//!
//!  This document covers the SPICELIB routines that deal with rotations and
//!    the mathematical ideas behind the routines.
//!
//!  There are three chapters:
//!
//!  
//!
//! * SPICELIB routines
//!
//!  *  The "SPICELIB routines" chapter tells you what routines are available,
//! what they do, and how to call them.
//!
//!  * Tutorial introduction to rotations
//!
//!  *  If rotations are new to you, you'll profit most by reading the "tutorial"
//! chapter first. This chapter lists most of the facts about rotations used in
//! the SPICELIB code. The emphasis is on building intuition about rotations;
//! proofs of any noticeable difficulty or length are deferred to the
//! "Mathematical Road Map" chapter.
//!
//!  * Mathematical road map
//!
//!  *  This chapter contains detailed explanations of a number of the mathematical
//! ideas used in the SPICELIB rotation routines. And as we've said, proofs of
//! some of assertions made in the tutorial chapter are stowed here.
//!
//!     
//! ###  Using this document
//!
//!  For many readers, the first chapter will be the only one of interest.
//!    The mechanics of using the rotation routines in your code are covered
//!    here.
//!
//!  The rest of the document covers the ideas behind the code. This material
//!    is meant to be used as a reference rather than to be read from start to
//!    finish; the topics are ordered loosely according to logical dependence,
//!    but there is no narrative progression from section to section.
//!
//!  The purpose of the tutorial and "road map" chapters is to make it
//!    easier to be certain that you're using the code correctly. In our
//!    experience, thinking about this category of code only in terms of
//!    "inputs" and "outputs" is a tricky and error-prone approach; really
//!    understanding the mathematics helps you to verify that you're putting
//!    the pieces together in ways that make sense.
//!
//!  Because some of the ideas required to understand the code seem to exist
//!    as lore and are rarely written down, we've collected them here for your
//!    convenience.
//!
//!  
//!
//!
//!  
//! ###  References
//!
//!  
//!
//! *  1. For SPICELIB routines dealing with inertial reference frames: SPK
//! "required reading." ([spk.req](crate::required_reading::spk))
//!
//!  *  2. For SPICELIB routines dealing with body-fixed reference frames: KERNEL
//! "required reading." ([kernel.req](crate::required_reading::kernel))
//!
//!  *  3. For linear algebra: "Calculus, Vol. II." Tom M. Apostol. Wiley and Sons,
//! 1969.
//!
//!  *  Also, "Elementary Linear Algebra." Howard Anton. Wiley and Sons, 1977.
//!
//!  *  4. NAIF document number 179.0, "Rotations and Their Habits," by Dr. William
//! Taber.
//!
//!  *  5. NAIF document number 192.0, "Memo to: Quaternion Seekers," by Allen
//! Klumpp.
//!
//!     
//! ###  Notation
//!
//!  Here are the symbols used in this document, and their definitions:
//!
//!  
//!
//! ```text
//!     Symbol                      Meaning
//!    ---------          ----------------------------------
//!  
//!       E3              Three-dimensional Euclidean space.
//!  
//!       E2              Two-dimensional Euclidean space.
//!  
//!    < u, v >           Inner product of vectors u and v.
//!  
//!         T
//!       M               Transpose of the matrix M.
//!  
//!        -1
//!       M               Inverse of the matrix M.
//!  
//!        -1
//!       f               Inverse of the function f.
//!  
//!     u x v             Cross product of vectors u and v.
//!  
//!      M v              Product of matrix M and vector v.
//!  
//!      M N              Product of matrix M and matrix N.
//!  
//!    Trace(M)           Sum of elements on the main diagonal of M.
//!  
//!      [w]              Matrix that rotates a coordinate system by
//!         i             w radians about the ith coordinate axis
//!                       (and rotates vectors by -w radians about the
//!                       same axis).  We also use this notation to refer
//!                       to the linear transformation corresponding to
//!                       this matrix.
//! ```
//!
//!  There are only three types of rotation matrices representing rotations
//!    about coordinate axes:
//!
//!  
//!
//! ```text
//!             +-                        -+
//!             |     1       0        0   |
//!    [w]   =  |     0    cos(w)   sin(w) |
//!       1     |     0   -sin(w)   cos(w) |
//!             +-                        -+
//!  
//!             +-                        -+
//!             |  cos(w)     0    -sin(w) |
//!    [w]   =  |     0       1        0   |
//!       2     |  sin(w)     0     cos(w) |
//!             +-                        -+
//!  
//!             +-                        -+
//!             |  cos(w)  sin(w)      0   |
//!    [w]   =  | -sin(w)  cos(w)      0   |
//!       3     |     0       0        1   |
//!             +-                        -+
//! ```
//!
//!  The composition of rotations
//!
//!  
//!
//! ```text
//!    [w1]   [w2]   [w3]
//!        a      b      c
//! ```
//!
//!  is sometimes referred to as a "a-b-c" rotation. For example, we may
//!    talk about a "3-1-3" rotation.
//!
//!  About angles: all angles in this document are measured in radians.
//!
//!  About directions: the "right hand rule" is in effect at all times in
//!    this document, so counterclockwise rotations about an axis have positive
//!    measure.
//!
//!  
//!
//!
//!  
//! #  SPICELIB Routines
//!
//!  This chapter describes the SPICELIB routines that deal with rotations.
//!    The chapter lists the SPICELIB rotation routines and discusses each
//!    routine.
//!
//!  The SPICELIB routines that deal with general rotations are:
//!
//!  
//!
//! *  [AXISAR](crate::raw::axisar)
//!
//!
//!  ( Axis and angle to rotation )
//!
//!  *  [DROTAT](crate::raw::drotat)
//!
//!
//!  ( Derivative of a rotation matrix )
//!
//!  *  [EUL2M](crate::raw::eul2m)
//!
//!
//!  ( Euler angles to matrix )
//!
//!  *  [EUL2XF](crate::raw::eul2xf)
//!
//!
//!  ( Euler angles and derivative to transformation )
//!
//!  *  [INVSTM](crate::raw::invstm)
//!
//!
//!  ( Inverse of state transformation matrix )
//!
//!  *  [ISROT](crate::raw::isrot)
//!
//!
//!  ( Is it a rotation matrix? )
//!
//!  *  [M2EUL](crate::raw::m2eul)
//!
//!
//!  ( Matrix to Euler angles )
//!
//!  *  [M2Q](crate::raw::m2q)
//!
//!
//!  ( Matrix to quaternion )
//!
//!  *  [PXFORM](crate::raw::pxform)
//!
//!
//!  ( Position transformation matrix )
//!
//!  *  [Q2M](crate::raw::q2m)
//!
//!
//!  ( Quaternion to matrix )
//!
//!  *  [QDQ2AV](crate::raw::qdq2av)
//!
//!
//!  ( Quaternion and derivative to angular velocity )
//!
//!  *  [QXQ](crate::raw::qxq)
//!
//!
//!  ( Quaternion times quaternion )
//!
//!  *  [RAXISA](crate::raw::raxisa)
//!
//!
//!  ( Rotation axis and angle )
//!
//!  *  [RAV2XF](crate::raw::rav2xf)
//!
//!
//!  ( Rotation and angular velocity to transform )
//!
//!  *  ROTATE
//!
//!
//!  ( Generate a rotation matrix )
//!
//!  *  [ROTMAT](crate::raw::rotmat)
//!
//!
//!  ( Rotate a matrix about a coordinate axis )
//!
//!  *  [ROTVEC](crate::raw::rotvec)
//!
//!
//!  ( Rotate a vector about a coordinate axis )
//!
//!  *  [SXFORM](crate::raw::sxform)
//!
//!
//!  ( State transformation matrix )
//!
//!  *  [TIPBOD](crate::raw::tipbod)
//!
//!
//!  ( Transformation, inertial position to bodyfixed )
//!
//!  *  [TISBOD](crate::raw::tisbod)
//!
//!
//!  ( Transformation, inertial state to bodyfixed )
//!
//!  *  [VROTV](crate::raw::vrotv)
//!
//!
//!  ( Rotate a vector about an arbitrary axis )
//!
//!  *  [XF2EUL](crate::raw::xf2eul)
//!
//!
//!  ( State transformation to Euler angles )
//!
//!  *  [XF2RAV](crate::raw::xf2rav)
//!
//!
//!  ( Transform to rotation and angular velocity)
//!
//!  Additional SPICELIB routines that deal with rotations between specific
//!    coordinate systems are documented in the FRAMES required reading and in
//!    the header of the SPICELIB routine [SXFORM](crate::raw::sxform).
//!
//!  
//!
//!
//!  
//! ###  Categories of routines
//!
//!  The discussions of the routines are categorized according to the type of
//!    problem that the routines solve. This chapter contains one section for
//!    each category. The categories are:
//!
//!  
//!
//! * Euler angles
//!
//!  * Quaternions
//!
//!  * Rotating vectors and matrices
//!
//!  * Rotation axis and angle
//!
//!  * Rotation derivatives
//!
//!  * Validating rotation matrices
//!
//!  The rotation routines constitute a "family" of routines insofar as
//!    they deal with related problems, but they do not constitute a
//!    "system." So, there is no logical interdependence between the
//!    discussions of routines in different categories.
//!
//!  
//!
//!
//!  
//! ##  Euler angle routines
//!
//!  The SPICELIB Euler angle routines are:
//!
//!  
//!
//! *  [EUL2M](crate::raw::eul2m)
//!
//!
//!  ( Euler angles to matrix )
//!
//!  *  [M2EUL](crate::raw::m2eul)
//!
//!
//!  ( Matrix to Euler angles )
//!
//!  [EUL2M](crate::raw::eul2m) and [M2EUL](crate::raw::m2eul) provide a convenient way to solve problems such as
//!    converting between C-matrices and RA, Dec, and Twist. These routines are
//!    inverses of each other, roughly speaking.
//!
//!  
//!
//!
//!  
//! ###  Constructing a matrix from Euler angles
//!
//!  The routine [EUL2M](crate::raw::eul2m) constructs the rotation matrix defined by three Euler
//!    angles ANG(3), ANG(2), ANG(1), and three coordinate axes indexed by the
//!    integers I(3), I(2), I(1).
//!
//!  The call
//!
//!  
//!
//! ```text
//!     CALL EUL2M ( ANG(3),   ANG(2),   ANG(1),
//!    .             I(3),     I(2),     I(1),     M )
//! ```
//!
//!  returns the matrix M, where
//!
//!  
//!
//! ```text
//!    M   =   [ANG(3)]       [ANG(2)]        [ANG(1)]    .
//!                    I(3)           I(2)            I(1)
//! ```
//!
//!  The indices I(3), I(2), I(1) must belong to the set
//!
//!  
//!
//! ```text
//!    {1, 2, 3}.
//! ```
//!
//!     
//! ###  Finding Euler angles that represent a matrix
//!
//!  Given a rotation matrix M and three coordinate axes indexed by the
//!    integers I(3), I(2), I(1), the routine [M2EUL](crate::raw::m2eul) finds angles ANG(3),
//!    ANG(2), ANG(1) such that
//!
//!  
//!
//! ```text
//!    M  =   [ANG(3)]       [ANG(2)]        [ANG(1)]    .
//!                   I(3)           I(2)            I(1)
//! ```
//!
//!  The call
//!
//!  
//!
//! ```text
//!     CALL M2EUL ( M,  I(3),    I(2),     I(1),
//!    .                 ANG(3),  ANG(2),   ANG(1) )
//! ```
//!
//!  returns the desired angles. [M2EUL](crate::raw::m2eul) restricts the ranges of the output
//!    angles so as to guarantee that the Euler angle representation is unique.
//!    The output angles ANG(3) and ANG(1) are always in the range (-pi,pi].
//!    The range of ANG(2) is determined by the set of rotation axes. When I(3)
//!    equals I(1), ANG(2) is in the range \[0, pi]. Otherwise, ANG(2) is in the
//!    range \[-pi/2, pi/2]. These ranges make unique determinations of Euler
//!    angles possible, except in degenerate cases.
//!
//!  In cases where the Euler angles are not uniquely determined, [EUL2M](crate::raw::eul2m) sets
//!    the first angle (called ANG(3) above) to zero. The other two angles are
//!    then uniquely determined.
//!
//!  Again, the indices I(3), I(2), I(1) are members of
//!
//!  
//!
//! ```text
//!    {1, 2, 3}.
//! ```
//!
//!  There is a restriction on the allowed set of coordinate axes: I(2) must
//!    not equal I(3) or I(1). If this constraint is not met, the desired
//!    representation of M by Euler angles may not exist; [M2EUL](crate::raw::m2eul) signals an
//!    error in this case.
//!
//!  
//!
//!
//!  
//! ###  Programming hazards
//!
//!  There are several pitfalls associated with converting matrices to Euler
//!    angles. First, any mapping from matrices to Euler angles has
//!    singularities. These come in two flavors: some matrices don't map to a
//!    unique set of Euler angles, and some matrices have the property that a
//!    small change in the matrix can result in a large change in the
//!    corresponding Euler angles.
//!
//!  The first category of singularity occurs with matrices that represent
//!    rotations about the first or third axis in the sequence of rotation axes
//!    (for example, axis 3 for a 2-1-3 rotation). In practical terms, if [EUL2M](crate::raw::eul2m)
//!    encounters one of these special matrices, [EUL2M](crate::raw::eul2m) must choose the Euler
//!    angles. Immediately the possibility arises that [EUL2M](crate::raw::eul2m) will disagree with
//!    any other code performing the same task.
//!
//!  The second kind of singularity occurs when any of the Euler angles
//!    corresponding to a matrix is at one of the endpoints of its range, for
//!    example, when the first angle has the value pi. If the matrix is
//!    perturbed slightly, the first angle may jump from pi to a value close to
//!    -pi. Again, two different pieces of code may give different results in
//!    such a case, merely because of round-off error. Euler angles near the
//!    limits of their ranges should be regarded with suspicion.
//!
//!  The existence of singularities in the matrix-to-Euler angle mapping
//!    prevents [EUL2M](crate::raw::eul2m) and [M2EUL](crate::raw::m2eul) from being exact inverses: most of the time,
//!    the code fragment
//!
//!  
//!
//! ```text
//!     CALL EUL2M (     ANG(3),   ANG(2),   ANG(1),
//!    .                 AXIS(1),  AXIS(2),  AXIS(3),  M  )
//!  
//!     CALL M2EUL ( M,  AXIS(1),  AXIS(2),  AXIS(3),
//!    .                 ANG(3),   ANG(2),   ANG(1)       )
//! ```
//!
//!  leaves the angles ANG(3), ANG(2), ANG(1) unchanged, except for round-off
//!    error, but in some cases, the angles may change drastically.
//!
//!  If we reverse the order of the routine calls in the last code fragment,
//!    the matrix M should be preserved, except for errors due to loss of
//!    precision. The loss of precision can be considerable, though, for
//!    matrices whose entries are nearly those of any degenerate case matrix.
//!
//!  For more details on this topic, consult the "Mathematical road map"
//!    section.
//!
//!  
//!
//!
//!  
//! ###  Working with RA, Dec and Twist
//!
//!  The Euler angle routines could be used for conversion between "RA, Dec,
//!    and Twist" and a "C-matrix." Most projects, including Voyager and
//!    Cassini define the relationship as:
//!
//!  
//!
//! ```text
//!    C  =  [ Twist ]   [ pi/2 - Dec ]   [ pi/2 + RA ]
//!                   3                1               3
//! ```
//!
//!  so the code fragments
//!
//!  
//!
//! ```text
//!     CALL EUL2M ( TWIST,    HALFPI() - DEC,   HALFPI() + RA
//!    .             3,        1,                3,              C  )
//! ```
//!
//!  and
//!
//!  
//!
//! ```text
//!     CALL M2EUL ( C,    3,        1,        3,
//!    .                   ANG(3),   ANG(2),   ANG(1)  )
//!  
//!     TWIST = ANG(3)
//!     DEC   = HALFPI() - ANG(2)
//!     RA    = ANG(1)   - HALFPI()
//! ```
//!
//!  carry out the conversion from RA, Dec, and Twist to a C matrix, and
//!    back.
//!
//!  Note that definitions of "RA, Dec, and Twist" vary: on the Galileo
//!    project, the C matrix is related to the angles "RA, Dec, and Twist" by
//!    the equation
//!
//!  
//!
//! ```text
//!    C  =  [ Twist ]   [ pi/2 - Dec ]    [ RA ] .
//!                   3                2         3
//! ```
//!
//!     
//! ##  Quaternions
//!
//!  The SPICELIB quaternion routines are:
//!
//!  
//!
//! *  [M2Q](crate::raw::m2q)
//!
//!
//!  ( Matrix to quaternion )
//!
//!  *  [Q2M](crate::raw::q2m)
//!
//!
//!  ( Quaternion to matrix )
//!
//!  *  [QDQ2AV](crate::raw::qdq2av)
//!
//!
//!  ( Quaternion and derivative to angular velocity )
//!
//!  *  [QXQ](crate::raw::qxq)
//!
//!
//!  ( Quaternion times quaternion )
//!
//!  Quaternions are used in SPICELIB as a compact representation for
//!    rotations. They may be used to compose rotations more efficiently than
//!    can be done with matrix multiplication.
//!
//!  
//!
//!
//!  
//! ###  Finding a quaternion that represents a matrix
//!
//!  The routine [M2Q](crate::raw::m2q) produces a quaternion that represents a specified
//!    rotation matrix.
//!
//!  The code fragment
//!
//!  
//!
//! ```text
//!    CALL M2Q ( M, Q )
//! ```
//!
//!  returns a unit quaternion Q that represents the rotation matrix M.
//!
//!  If you really want to know about it, the elements of the quaternion are
//!    defined as follows:
//!
//!  Let the unit vector A be a choice of rotation axis for M, and let ANGLE
//!    be the rotation angle, where ANGLE is in the interval
//!
//!  
//!
//! ```text
//!    [0, pi]
//! ```
//!
//!  Then the elements of the unit quaternion Q returned by [M2Q](crate::raw::m2q) are
//!
//!  
//!
//! ```text
//!    Q(0) = cos( ANGLE/2 )
//!  
//!    Q(1) = sin( ANGLE/2 ) * A(1)
//!    Q(2) = sin( ANGLE/2 ) * A(2)
//!    Q(3) = sin( ANGLE/2 ) * A(3)
//! ```
//!
//!  [M2Q](crate::raw::m2q) considers the rotation angle of M to lie in \[0, pi]. Therefore, half
//!    the rotation angle lies in \[0, pi/2], so Q(0) is always in \[0, 1]. For a
//!    given rotation matrix M, the corresponding quaternion is uniquely
//!    determined except if the rotation angle is pi.
//!
//!  
//!
//!
//!  
//! ###  Finding the matrix represented by a quaternion
//!
//!  The routine [Q2M](crate::raw::q2m) inverts the transformation performed by [M2Q](crate::raw::m2q). The
//!    following call finds the rotation matrix M represented by the unit
//!    quaternion Q:
//!
//!  
//!
//! ```text
//!    CALL Q2M ( Q, M )
//! ```
//!
//!     
//! ###  [Q2M](crate::raw::q2m) and [M2Q](crate::raw::m2q) are approximate inverses of each other
//!
//!  The calls
//!
//!  
//!
//! ```text
//!    CALL M2Q ( M, Q )
//!    CALL Q2M ( Q, M )
//! ```
//!
//!  always preserve M, except for round-off error.
//!
//!  However, since there are two quaternions that represent each rotation,
//!    the sequence of calls
//!
//!  
//!
//! ```text
//!    CALL Q2M ( Q, M )
//!    CALL M2Q ( M, Q )
//! ```
//!
//!  do not necessarily preserve Q.
//!
//!  
//!
//!
//!  
//! ###  Multiplying quaternions
//!
//!  Two quaternions Q1, Q2 may be multiplied by calling [QXQ](crate::raw::qxq):
//!
//!  
//!
//! ```text
//!    CALL QXQ ( Q1, Q2, QOUT )
//! ```
//!
//!  The resulting product QOUT is computed using the multiplication formula
//!    given in the section "Quaternion Arithmetic" below. QOUT represents
//!    the rotation formed by composing the rotations represented by Q1 and Q2.
//!
//!  
//!
//!
//!  
//! ###  Obtaining angular velocity from quaternions
//!
//!  Given a quaternion Q and its derivative with respect to time DQ, the
//!    angular velocity of the reference frame represented by Q may be found by
//!    calling [QDQ2AV](crate::raw::qdq2av):
//!
//!  
//!
//! ```text
//!    CALL QDQ2AV ( Q, DQ, AV )
//! ```
//!
//!  The resulting angular velocity vector AV has units of radians/T, where
//!    1/T is the time unit associated with DQ.
//!
//!  
//!
//!
//!  
//! ##  Rotating vectors and matrices
//!
//!  The SPICELIB routines that "rotate" vectors and matrices are:
//!
//!  
//!
//! *  [ROTVEC](crate::raw::rotvec)
//!
//!
//!  ( Rotate vector )
//!
//!  *  [ROTMAT](crate::raw::rotmat)
//!
//!
//!  ( Rotate matrix )
//!
//!  *  [VROTV](crate::raw::vrotv)
//!
//!
//!  ( Rotate a vector about an axis )
//!
//!  This set of routines are frequently used as utilities from which more
//!    complicated routines may be constructed. For example, the routine [EUL2M](crate::raw::eul2m) constructs a rotation matrix from a sequence of three rotations about
//!    specified coordinate axis, so there is no need to call [ROTMAT](crate::raw::rotmat) directly
//!    to accomplish this.
//!
//!  
//!
//!
//!  
//! ###  A word of warning
//!
//!  Some care is needed when dealing with signs of rotation angles: a
//!    rotation of a vector by an angle THETA can be viewed as rotating the
//!    coordinate system by -THETA. We try to avoid confusion here by referring
//!    to routines as "coordinate system rotations" or "vector rotations,"
//!    depending on whether a positive rotation angle corresponds to rotating
//!    the coordinate system by a positive angle, or to rotating a vector by a
//!    positive angle. The same criterion applies to matrix rotations.
//!    According to this classification, [ROTVEC](crate::raw::rotvec) and [ROTMAT](crate::raw::rotmat) perform coordinate
//!    rotations, and [VROTV](crate::raw::vrotv) performs a vector rotation.
//!
//!  
//!
//!
//!  
//! ###  Rotating a vector about a coordinate axis
//!
//!  To apply the rotation
//!
//!  
//!
//! ```text
//!    [angle]
//!           i
//! ```
//!
//!  to a vector, use [ROTVEC](crate::raw::rotvec). The following code fragment applies
//!
//!  
//!
//! ```text
//!    [ ANGLE ]
//!             3
//! ```
//!
//!  to the vector V, yielding VOUT.
//!
//!  
//!
//! ```text
//!    CALL ROTVEC ( V, ANGLE, 3, VOUT )
//! ```
//!
//!  The components of VOUT are the coordinates of the vector V in a system
//!    rotated by ANGLE radians about the third coordinate axis. We can also
//!    regard VOUT as V, rotated by
//!
//!  
//!
//! ```text
//!    -ANGLE
//! ```
//!
//!  radians about the third coordinate axis.
//!
//!  
//!
//!
//!  
//! ###  Rotating a matrix about a coordinate axis
//!
//!  We can apply the same rotation
//!
//!  
//!
//! ```text
//!    [angle]
//!           i
//! ```
//!
//!  to a matrix using [ROTMAT](crate::raw::rotmat), as follows:
//!
//!  
//!
//! ```text
//!    CALL ROTMAT ( M, ANGLE, 3, MOUT )
//! ```
//!
//!  After this routine call, MOUT is equal to
//!
//!  
//!
//! ```text
//!    [ANGLE]  * M
//!           3
//! ```
//!
//!     
//! ###  Rotating a vector about an arbitrary axis
//!
//!  We can rotate a vector about an arbitrary axis using [VROTV](crate::raw::vrotv). The code
//!    fragment
//!
//!  
//!
//! ```text
//!    CALL VROTV ( V, AXIS, ANGLE, VOUT )
//! ```
//!
//!  rotates the vector V about the vector AXIS by ANGLE radians, yielding
//!    VOUT.
//!
//!  
//!
//!
//!  
//! ##  Rotation axis and angle
//!
//!  There are three SPICELIB routines that deal with rotation axes and
//!    angles:
//!
//!  
//!
//! *  [AXISAR](crate::raw::axisar)
//!
//!
//!  ( Axis and angle to rotation )
//!
//!  *  [RAXISA](crate::raw::raxisa)
//!
//!
//!  ( Rotation axis and angle )
//!
//!  *  ROTATE
//!
//!
//!  ( Generate a rotation matrix )
//!
//!  Like the routines that rotate vectors and matrices, these routines are
//!    frequently used as building blocks for more sophisticated routines.
//!
//!  
//!
//!
//!  
//! ###  Constructing a matrix from a rotation axis and angle
//!
//!  To generate a matrix that rotates vectors by a specified angle about an
//!    arbitrary axis, use [AXISAR](crate::raw::axisar). If AXIS is the axis vector and ANGLE is the
//!    rotation angle, the code fragment
//!
//!  
//!
//! ```text
//!    CALL AXISAR ( AXIS, ANGLE, M )
//! ```
//!
//!  produces M, the desired rotation matrix.
//!
//!  What if we want generate a coordinate system rotation about an arbitrary
//!    axis, as opposed to a coordinate axis? We can use [AXISAR](crate::raw::axisar) for this. Let
//!    AXIS be the coordinate system rotation axis and ANGLE be the rotation
//!    angle; then the code fragment
//!
//!  
//!
//! ```text
//!    CALL AXISAR ( AXIS, -ANGLE, M )
//! ```
//!
//!  produces the desired coordinate system rotation matrix. Note that the
//!    input angle is the NEGATIVE of that associated with a vector rotation. [AXISAR](crate::raw::axisar) is designed this way for compatibility with [RAXISA](crate::raw::raxisa), which is an
//!    inverse routine for [AXISAR](crate::raw::axisar).
//!
//!  
//!
//!
//!  
//! ###  Finding the axis and angle of a rotation matrix
//!
//!  To find the rotation axis and angle of a rotation matrix M, use [RAXISA](crate::raw::raxisa):
//!
//!  
//!
//! ```text
//!    CALL RAXISA ( M,  AXIS,  ANGLE )
//! ```
//!
//!  AXIS and ANGLE have the property that for any vector V,
//!
//!  
//!
//! ```text
//!    M V
//! ```
//!
//!  yields V, rotated by ANGLE radians about the vector AXIS. If M is viewed
//!    as a coordinate transformation, we can say that M rotates the initial
//!    coordinate system by
//!
//!  
//!
//! ```text
//!    - ANGLE
//! ```
//!
//!  radians about AXIS.
//!
//!  
//!
//!
//!  
//! ###  [AXISAR](crate::raw::axisar) and [RAXISA](crate::raw::raxisa) are approximate inverses
//!
//!  The code fragment
//!
//!  
//!
//! ```text
//!    CALL RAXISA ( M,    AXIS,  ANGLE )
//!    CALL AXISAR ( AXIS, ANGLE, M     )
//! ```
//!
//!  leaves M unchanged, except for round-off error.
//!
//!  The code fragment
//!
//!  
//!
//! ```text
//!    CALL AXISAR ( AXIS, ANGLE, M     )
//!    CALL RAXISA ( M,    AXIS,  ANGLE )
//! ```
//!
//!  usually leaves AXIS and ANGLE unchanged, except for round-off error,
//!    provided that two conditions are met:
//!
//!  
//!
//! * ANGLE is in the range (0,pi)
//!
//!  * AXIS is a unit vector.
//!
//!  If ANGLE is near zero or pi, loss of precision may preclude recovery of
//!    AXIS.
//!
//!  
//!
//!
//!  
//! ###  Using [RAXISA](crate::raw::raxisa) and [AXISAR](crate::raw::axisar)
//!
//!  [RAXISA](crate::raw::raxisa) and [AXISAR](crate::raw::axisar) can be used to perform linear interpolation between
//!    two rotation matrices. Here's a code fragment illustrating the
//!    procedure:
//!
//!  
//!
//! ```text
//!    C
//!    C     Let R(t) be a time-varying rotation matrix; R could be
//!    C     a C-matrix describing the orientation of a spacecraft
//!    C     structure.  Given two points in time t1 and t2 at which
//!    C     R(t) is known, and given a third time t3, where
//!    C
//!    C        t1  <  t3  <  t2,
//!    C
//!    C     we can estimate R(t3) by linear interpolation.  In other
//!    C     words, we approximate the motion of R by pretending that
//!    C     R rotates about a fixed axis at a uniform angular rate
//!    C     during the time interval [t1, t2].  More specifically, we
//!    C     assume that each column vector of R rotates in this
//!    C     fashion.  This procedure will not work if R rotates through
//!    C     an angle of pi radians or more during the time interval
//!    C     [t1, t2]; an aliasing effect would occur in that case.
//!    C
//!    C     If we let
//!    C
//!    C        R1 = R(t1)
//!    C        R2 = R(t2), and
//!    C
//!    C                    -1
//!    C        Q  = R2 * R1  ,
//!    C
//!    C     then the rotation axis and angle of Q define the rotation
//!    C     that each column of R(t) undergoes from time t1 to time
//!    C     t2.  Since R(t) is orthogonal, we can find Q using the
//!    C     transpose of R1.  We find the rotation axis and angle via
//!    C     RAXISA.
//!    C
//!          CALL MXMT   ( R2,   R1,    Q      )
//!          CALL RAXISA ( Q,    AXIS,  ANGLE  )
//!  
//!    C
//!    C     Find the fraction of the total rotation angle that R
//!    C     rotates through in the time interval [t1, t3].
//!    C
//!          FRAC = ( T3 - T1 )  /  ( T2 - T1 )
//!  
//!    C
//!    C     Finally, find the rotation DELTA that R(t) undergoes
//!    C     during the time interval [t1, t3], and apply that rotation
//!    C     to R1, yielding R(t3), which we'll call R3.
//!    C
//!             CALL AXISAR ( AXIS,   FRAC * ANGLE,  DELTA  )
//!             CALL MXM    ( DELTA,  R1,            R3     )
//! ```
//!
//!     
//! ###  Constructing a coordinate axis rotation matrix
//!
//!  The routine ROTATE generates the rotation matrix
//!
//!  
//!
//! ```text
//!    [ANGLE] ,
//!           I
//! ```
//!
//!  which corresponds to a rotation about a coordinate axis. This is a
//!    special case of the problem solved by [AXISAR](crate::raw::axisar). Note however that the
//!    matrix produced by ROTATE is the inverse of that produced by [AXISAR](crate::raw::axisar), if
//!    both routines are provided with the same input angle, and [AXISAR](crate::raw::axisar) is
//!    given the Ith coordinate basis vector as the rotation axis.
//!
//!  The call
//!
//!  
//!
//! ```text
//!    CALL ROTATE ( ANGLE, I, M )
//! ```
//!
//!  produces M, the desired matrix.
//!
//!  
//!
//!
//!  
//! ##  Rotation derivatives
//!
//!  The SPICELIB routines that deal with derivatives of rotations are:
//!
//!  
//!
//! *  [DROTAT](crate::raw::drotat)
//!
//!
//!  ( Derivative of a rotation matrix )
//!
//!  *  [INVSTM](crate::raw::invstm)
//!
//!
//!  ( Inverse of state transformation matrix )
//!
//!  *  [TISBOD](crate::raw::tisbod)
//!
//!
//!  ( Transformation, inertial state to bodyfixed )
//!
//!  The rotation derivative routines are utilities that simplify finding
//!    derivatives of time-varying coordinate transformations. In particular,
//!    these routines are used to transform state vectors between non-inertial
//!    reference frames.
//!
//!  
//!
//!
//!  
//! ###  Differentiating rotations
//!
//!  The routine [DROTAT](crate::raw::drotat) gives the derivative of a coordinate axis rotation
//!    with respect to the rotation angle. For example, the transformation
//!
//!  
//!
//! ```text
//!    [ANGLE]
//!           1
//! ```
//!
//!  which has the matrix representation
//!
//!  
//!
//! ```text
//!    +-                             -+
//!    |  1         0           0      |
//!    |  0    cos(ANGLE)  sin(ANGLE)  |
//!    |  0   -sin(ANGLE)  cos(ANGLE)  |
//!    +-                             -+
//! ```
//!
//!  yields, when differentiated with respect to ANGLE, the matrix
//!
//!  
//!
//! ```text
//!    +-                             -+
//!    |  0         0           0      |
//!    |  0   -sin(ANGLE)  cos(ANGLE)  |
//!    |  0   -cos(ANGLE) -sin(ANGLE)  |
//!    +-                             -+
//! ```
//!
//!  The routine [DROTAT](crate::raw::drotat) is useful for differentiating rotations that are
//!    defined by time-varying Euler angles. For example, if the rotation R is
//!    defined by
//!
//!  
//!
//! ```text
//!    R = [ Twist ]    [ pi/2 - Dec ]    [ pi/2 + RA ]
//!                 3                 1                3
//! ```
//!
//!  where RA, Dec, and Twist are time-dependent, then if we make the
//!    abbreviations
//!
//!  
//!
//! ```text
//!    A(Twist) = [ Twist ]
//!                        3
//!  
//!    B(Dec)   = [ pi/2 - Dec ]
//!                             1
//!  
//!    C(RA)    = [ pi/2 + RA ]
//!                            3
//! ```
//!
//!  we can write
//!
//!  
//!
//! ```text
//!    d(R)            d(A)     d(Twist)
//!    ----  =       -------- * --------       *     B     *      C
//!     dt           d(Twist)      dt
//!  
//!                                      d(B)    - d(Dec)
//!              +       A    *         ------ * --------  *      C
//!                                     d(Dec)      dt
//!  
//!                                                  d(C)     d(RA)
//!              +       A    *        B       *     ----- * -------
//!                                                  d(RA)      dt
//! ```
//!
//!  The derivatives of A, B, and C can be found using [DROTAT](crate::raw::drotat).
//!
//!  
//!
//!
//!  
//! ###  State transformations
//!
//!  Transforming state vectors between inertial and non-inertial coordinates
//!    requires the derivative of the rotation that relates the two frames: Let
//!
//!  
//!
//! ```text
//!    P (t)
//!     I
//! ```
//!
//!  be a position vector referenced to an inertial frame "I," and let
//!
//!  
//!
//! ```text
//!    P (t)
//!     N
//! ```
//!
//!  be the equivalent position vector referenced to a non-inertial frame
//!    "N." If R(t) is the transformation from frame I to N at time t, then
//!    the two vectors are related as follows:
//!
//!  
//!
//! ```text
//!    P (t)  =  R(t) P (t)
//!     N              I
//! ```
//!
//!  Therefore, the derivatives of the position vectors satisfy
//!
//!  
//!
//! ```text
//!    d [ P (t) ]               d [ P (t) ]
//!         N                         I              d [ R(t) ]
//!    -----------   =   R(t) *  -----------    +    ----------  *  P (t)
//!        dt                        dt                  dt          I
//! ```
//!
//!  It's well to note that although R(t) may vary slowly, the second term in
//!    the above equation is not necessarily insignificant. For example, if
//!    R(t) describes a transformation between an inertial frame and a
//!    body-centered frame that uses a body-center-to-Sun vector to define one
//!    of its coordinate axes, then for any point that is fixed on this axis,
//!    the two addends above have equal and opposite magnitude. In particular,
//!    if the fixed point is the location of the Sun, the magnitude of the
//!    second addend is (ignoring the velocity of the Sun with respect to the
//!    inertial frame) that of the inertially referenced velocity of the body
//!    used to define the body-centered frame.
//!
//!  SPICELIB provides routines to transform states between inertial frames
//!    and body-fixed planetocentric frames. The routine [TISBOD](crate::raw::tisbod) returns the 6x6
//!    transformation matrix required to transform inertially referenced state
//!    vectors to body-fixed planetocentric coordinates. If REF is the name of
//!    the inertial frame of interest, BODY is the NAIF integer code of a body
//!    defining a body-fixed planetocentric frame, and ET is ephemeris time
//!    used to define the body-fixed frame, then the call
//!
//!  
//!
//! ```text
//!    CALL TISBOD ( REF, BODY, ET, TSIPM )
//! ```
//!
//!  returns TSIPM, the desired 6x6 state transformation matrix. A state
//!    vector S can be transformed to the body-fixed state vector SBFIXD by the
//!    routine call
//!
//!  
//!
//! ```text
//!    CALL MXVG ( TSIPM, S, 6, 6, SBFIXD )
//! ```
//!
//!  Since the inverse of a state transformation matrix is not simply its
//!    transpose, SPICELIB provides the utility routine [INVSTM](crate::raw::invstm) to perform the
//!    inversion. If M is a state transformation matrix, the inverse matrix
//!    MINV can be obtained via the routine call
//!
//!  
//!
//! ```text
//!    CALL INVSTM ( M, MINV )
//! ```
//!
//!     
//! ##  Validating a rotation matrix
//!
//!  [ISROT](crate::raw::isrot) is a logical function that indicates whether a matrix is a valid
//!    rotation matrix. The criteria for validity are:
//!
//!  
//!
//! * The columns of the matrix are unit vectors, within a specified tolerance.
//!
//!  * The determinant of the matrix formed by unitizing the columns of the input
//! matrix is 1, within a specified tolerance. This criterion ensures that the
//! columns of the matrix are nearly orthogonal, and that they form a
//! right-handed basis.
//!
//!  We might use [ISROT](crate::raw::isrot) as follows:
//!
//!  
//!
//! ```text
//!    C
//!    C     Set values for the column norm and determinant tolerances
//!    C     NTOL and DTOL:
//!    C
//!          NTOL = 1.D-7
//!          DTOL = 1.D-7
//!  
//!          IF (  .NOT.  ISROT ( M, NTOL, DTOL )   ) THEN
//!  
//!             [perform error handling]
//!  
//!          ELSE
//!  
//!             CALL M2Q ( M, Q )
//!                  .
//!                  .
//!                  .
//!          END IF
//! ```
//!
//!     
//! #  Tutorial introduction to rotations
//!
//!  This tutorial is intended to bridge the gap between knowing linear
//!    algebra and understanding rotations. If you haven't reached the gap yet,
//!    consult reference \[2], or any reasonable textbook on the subject.
//!
//!  In this section, we make some assertions that we don't prove. Our goal
//!    is to supply you with the most important information first, and fill in
//!    the details later. Proofs are supplied only when they're instructive and
//!    not too distracting. The longer or more difficult proofs are deferred to
//!    the "Mathematical road map" chapter.
//!
//!  
//!
//!
//!  
//! ##  A comment of the heuristic variety
//!
//!  If you're going to read the rest of this tutorial, you're going to see a
//!    lot of definitions, symbol manipulation, and proofs. This information
//!    may be more accessible if you have some kind of reasonable mental model
//!    of rotations; then you can test our claims against your model.
//!
//!  We offer the following model: Take a soccer ball, put two fingers on
//!    diametrically opposed points on the ball, and rotate the ball through
//!    some angle, keeping your fingers in place. What you use to rotate the
//!    ball is up to you.
//!
//!  Well, that's it. That's the effect of a rotation on a soccer ball. Now
//!    you're equipped to answer some questions about rotations. Do rotations
//!    preserve inner products of vectors? That is, is it true that for vectors
//!    u and v, and a rotation R,
//!
//!  
//!
//! ```text
//!    < R u,  R v >  =  < u, v > ?
//! ```
//!
//!  Well, presume that your soccer ball is centered at the origin, and mark
//!    the ball where u and v, or extensions of them, intercept the surface
//!    (perhaps you could hold a marker pen between your teeth). Does rotating
//!    the ball change the angular separation of the marks? No. So rotations
//!    preserve angular separation. Does rotating the ball change the norm of u
//!    or of v? No. So rotations preserve both angular separation and norms,
//!    and hence inner products.
//!
//!  Do rotations preserve cross products? For vectors u and v, is it true
//!    that
//!
//!  
//!
//! ```text
//!    ( R u )  x  ( R v )  =  R ( u x v )?
//! ```
//!
//!  Mark the intercepts of u, v, and u x v on the soccer ball. After you
//!    rotate the ball, does the intercept mark of u x v still lie at the right
//!    place, relative to the u and v intercept marks? Yes. Since we already
//!    know that rotations preserve norms, we can conclude that they preserve
//!    cross products as well.
//!
//!  The soccer ball model shows that rotations preserve geometrical
//!    relationships between vectors.
//!
//!  
//!
//!
//!  
//! ##  Definition of "rotation"
//!
//!  Actually, we present three definitions of the term "rotation." What
//!    for? Having more than one way of knowing that a mapping is a rotation
//!    makes it easier to check whether any particular mapping is a rotation or
//!    not. Some properties of rotations are easier to derive from one
//!    definition than from another.
//!
//!  
//!
//!
//!  
//! ###  Definition 1
//!
//!  A "rotation" R is a linear transformation defined on E3 that has the
//!    following properties:
//!
//!  
//!
//! * R preserves norms: if v is a vector, then
//!
//!  ```text
//!    || R(v) ||  =  ||v||
//! ```
//!  * R preserves cross products: if a and b are vectors, then
//!
//!  ```text
//!    R ( a x b )  =  ( R a )  x  ( R b ).
//! ```
//!     
//! ###  Definition 2
//!
//!  A "rotation' R is a mapping defined on E3 that satisfies definition
//!    (1), and also has the property:
//!
//!  
//!
//! * R keeps some vector (called the rotation axis) fixed. That is, if n is the
//! rotation axis, then
//!
//!  ```text
//!    R(n) = n.
//! ```
//!  *  Since R is linear, the entire line containing n is fixed.
//!
//!     
//! ###  Definition 3
//!
//!  A rotation R is a linear mapping defined on E3 that has the following
//!    properties:
//!
//!  
//!
//! * R preserves distances on the unit sphere.
//!
//!  * R keeps exactly two points on the unit sphere fixed, or else R is the
//! identity.
//!
//!  *  This property rules out the possibility that R is a "reflection."
//!
//!     
//! ###  Uses of the definitions
//!
//!  Definition (1) is useful for checking that a mapping is a rotation,
//!    because there's not much to check.
//!
//!  Definition (2) obviously implies definition (1). Less obviously,
//!    definition (1) implies definition (2). This had better be true if
//!    definition (1) is valid, since we expect rotations to have rotation
//!    axes. In the "Mathematical road map" chapter, we prove that the two
//!    definitions are equivalent.
//!
//!  Definition (3) is a mathematical paraphrase of our soccer ball model of
//!    rotations.
//!
//!  
//!
//!
//!  
//! ##  Definition of "rotation" and "orthogonal" matrix
//!
//!  A "rotation matrix" M is a 3 by 3 matrix whose columns form an
//!    orthonormal set, and whose third column is the cross product of the
//!    first two. Given a rotation R and an orthonormal basis B, the matrix
//!    representation of R relative to B is a rotation matrix.
//!
//!  Any matrix whose columns form an orthonormal set is called an
//!    "orthogonal" matrix.
//!
//!  
//!
//!
//!  
//! ##  Rotations preserve inner products
//!
//!  This section is for those who do not feel comfortable with soccer ball
//!    arguments.
//!
//!  Our definition of "rotation" says that rotations preserve norms of
//!    vectors. That is, if R is a rotation and v is a vector, then
//!
//!  
//!
//! ```text
//!    || R(v) ||  =  || v ||.
//! ```
//!
//!  Preserving norms also implies the seemingly stronger property of
//!    preserving inner products: if R preserves norms and u, v are vectors,
//!    then
//!
//!  
//!
//! ```text
//!                       2               2
//!    ||  R ( u - v )  ||  =  || u - v ||
//!  
//!  
//!                         =  < u - v,  u - v >
//!  
//!                                   2                        2
//!                         =  || u ||  - 2 < u, v >  + || v ||,
//! ```
//!
//!  and also
//!
//!  
//!
//! ```text
//!                       2
//!    ||  R ( u - v )  ||  =  < R ( u - v ),  R ( u - v ) >
//!  
//!  
//!                         =  < R(u) - R(v),  R(u) - R(v) >
//!  
//!                                      2               2
//!                         =  || R(u) ||   +  || R(v) ||
//!  
//!                                        -  2 < R u, R v >
//!  
//!                                   2             2
//!                         =  || u ||   +   || v ||
//!  
//!                                        -  2 < R u, R v >
//! ```
//!
//!  so
//!
//!  
//!
//! ```text
//!    < R(u), R(v) >       =   < u, v >.
//! ```
//!
//!  So rotations really do preserve inner products. In particular, for any
//!    orthonormal basis, the images of the basis vectors under a rotation are
//!    also an orthonormal set. Then rotation matrices, expressed relative to
//!    an orthonormal basis, are in fact orthogonal, as claimed.
//!
//!  
//!
//!
//!  
//! ##  Inverses of rotation matrices
//!
//!  We've seen that the columns of a rotation matrix R form an orthonormal
//!    set. Since the (i,j) entry of
//!
//!  
//!
//! ```text
//!     T
//!    R  R
//! ```
//!
//!  is the inner product of the ith column of R and the jth column of R, all
//!    entries of the product are zero except for those on the main diagonal,
//!    and the entries on the main diagonal are all 1. So
//!
//!  
//!
//! ```text
//!     T
//!    R  R  =  I.
//! ```
//!
//!  If A and B are square matrices with real or complex entries, it's a fact
//!    that if
//!
//!  
//!
//! ```text
//!    A B = I
//! ```
//!
//!  then
//!
//!  
//!
//! ```text
//!    B A = I.
//! ```
//!
//!  We won't prove this (the "Nullity and Rank" theorem is useful, if you
//!    wish to do so). But this result implies that
//!
//!  
//!
//! ```text
//!       T
//!    R R  = I,
//! ```
//!
//!  given the previous result.
//!
//!  This shows that the rows of R form an orthonormal set as well, and that
//!    the transpose of R is also a rotation matrix.
//!
//!  
//!
//!
//!  
//! ##  Composition of rotations
//!
//!  If we multiply a sequence of rotation matrices, is the result a rotation
//!    matrix? Equivalently, if we compose a sequence of rotations, is the
//!    resulting mapping a rotation?
//!
//!  What does our soccer ball model say? We can rotate the ball as many
//!    times as we like, changing the rotation axis each time, without changing
//!    the distance of any surface point from the center, so norms are
//!    preserved. Similarly, after marking the intercepts of u, v, and u x v on
//!    the surface, we can perform any sequence of rotations without changing
//!    the position of the u x v intercept mark relative to those of u and v.
//!    So cross products are preserved. That's all we need to verify that the
//!    composition of a sequence of rotations is a rotation. It follows that
//!    the product of a sequence of rotation matrices is a rotation matrix.
//!
//!  If you don't agree that that's all we need, we can present the same
//!    argument using the usual symbols:
//!
//!  Suppose R1 and R2 are rotation mappings, and for any vector v,
//!
//!  
//!
//! ```text
//!    R3(v) = R2 ( R1(v) ).
//! ```
//!
//!  Then for any vector v, we have
//!
//!  
//!
//! ```text
//!    || R3(v) ||  =  || R2 ( R1 v ) ||
//!  
//!                 =  || R1( v ) ||
//!  
//!                 =  || v ||.
//! ```
//!
//!  So R3 preserves norms.
//!
//!  If u and v are vectors, then
//!
//!  
//!
//! ```text
//!    R3 ( u x v ) =  R2 ( R1 ( u x v )  )
//!  
//!                 =  R2 ( R1(u)  x  R1(v) )
//!  
//!                 =  (  R2 ( R1(u) )   x   R2 ( R1(v) )  )
//!  
//!                 =  R3(u)  x  R3(v),
//! ```
//!
//!  so R3 preserves cross products. We've used only the definition of R3 and
//!    the fact that R2 and R1 preserve cross products in this proof.
//!
//!  We conclude that R3 is a rotation. We can extend the result to the
//!    product of a finite number of rotations by mathematical induction; the
//!    argument we've made is almost identical to the induction step.
//!
//!  
//!
//!
//!  
//! ##  Coordinate transformations
//!
//!  Change-of-basis transformations between right-handed, orthonormal bases
//!    are rotations. You can verify this using our first definition of
//!    rotations. In particular, the inertial coordinate system transformations
//!    available in SPICELIB are all rotations.
//!
//!  How do we transform a vector v from one coordinate system to another?
//!    This result really belongs to linear algebra, but we'll state it here
//!    because it seems to come up a lot.
//!
//!  Given two vector space bases,
//!
//!  
//!
//! ```text
//!    B1 = { e1, e2, e3 },   B2 = { u1, u2, u3 },
//! ```
//!
//!  and a vector v that has components ( v1, v2, v3 ) relative to B1, we
//!    wish to express v relative to B2. We can say that
//!
//!  
//!
//! ```text
//!    v  =  x1 u1 +  x2 u2  +  x3 u3,
//! ```
//!
//!  where the x's are unknowns. Let M be the matrix whose columns are u1,
//!    u2, and u3, represented relative to basis B1. M represents the linear
//!    transformation T defined by
//!
//!  
//!
//! ```text
//!    T  (e1) = u1, T(e2) = u2, T(e3) = u3.
//! ```
//!
//!  Then since
//!
//!  
//!
//! ```text
//!     -1         -1
//!    T  (v)  =  T  ( x1 u1 +  x2 u2  +  x3 u3 )
//!  
//!            =     ( x1 e1 +  x2 e2  +  x3 e3 ),
//! ```
//!
//!  we see that
//!
//!  
//!
//! ```text
//!     -1
//!    M  v  = ( x1, x2, x3 ).
//! ```
//!
//!  So we've found the components of v, relative to basis B2.
//!
//!  In the case where B1 and B2 are orthonormal bases, the matrix M is
//!    orthogonal. So we have
//!
//!  
//!
//! ```text
//!     T
//!    M  v  = ( x1, x2, x3 ).
//! ```
//!
//!  Conversely, if M is the matrix that transforms vectors from orthonormal
//!    basis B1 to orthonormal basis B2, then the rows of M are the basis
//!    vectors of B2.
//!
//!  For example, if M is the matrix that transforms vectors from J2000
//!    coordinates to body equator and prime meridian coordinates, then the
//!    first row is the vector, expressed in J2000 coordinates, that points
//!    from the body center to the intersection of the prime meridian and body
//!    equator. The third row is the vector, expressed in J2000 coordinates,
//!    that points from the body center toward the body's north pole.
//!
//!  
//!
//!
//!  
//! ##  Rotation of vectors in the plane
//!
//!  How do we rotate a two-dimensional vector v by theta radians? Trivial as
//!    this problem may seem, it's probably worth your while to get a firm grip
//!    on its solution. When you've understood it, you've almost understood
//!    three-dimensional rotations.
//!
//!  We can assume that v is a unit vector; since rotations are linear, it's
//!    easy to extend the result to vectors of any length.
//!
//!  Now, if v is (1,0), the result of the rotation will be
//!
//!  
//!
//! ```text
//!    ( cos(theta),  sin(theta) ).
//! ```
//!
//!  How does this help us if v is an arbitrary unit vector? Given a unit
//!    vector v, let v' be the vector perpendicular to v, obtained by rotating
//!    v by pi/2. Now v and v' form an orthonormal basis, and relative to this
//!    basis, v has coordinates (1,0). But we've already found out what we get
//!    by rotating v by theta radians: relative to our new basis, the result
//!    must be
//!
//!  
//!
//! ```text
//!    ( cos(theta), sin(theta) ).
//! ```
//!
//!  Relative to our original basis, this vector is
//!
//!  
//!
//! ```text
//!    cos(theta) v  +  sin(theta) v'
//! ```
//!
//!  This is the result we're looking for: "If you rotate a vector v by
//!    theta radians, you end up with cos(theta) v plus sin(theta) v'," where
//!    v' is v, rotated by pi/2.
//!
//!  Scaling v does not affect this result.
//!
//!  A consequence of this result is that the mapping R that rotates vectors
//!    by theta radians is represented by the matrix
//!
//!  
//!
//! ```text
//!    +-                        -+
//!    |  cos(theta)  -sin(theta) |
//!    |                          |.
//!    |  sin(theta)   cos(theta) |
//!    +-                        -+
//! ```
//!
//!  It is useful to note that R has this exact representation relative to
//!    any orthonormal basis where the second vector is obtained from the first
//!    by a rotation of pi/2.
//!
//!  
//!
//!
//!  
//! ##  A canonical representation for rotations
//!
//!  Suppose we have a rotation R, defined in three-dimensional space, that
//!    rotates vectors by angle theta about unit vector n. For an arbitrary
//!    vector r, we can break r up into two orthogonal components: one parallel
//!    to n and one perpendicular to n. We can call these components rParallel
//!    and rPerp.
//!
//!  The two-dimensional diagram below shows this decomposition. All of the
//!    vectors lie in the plane containing r and n.
//!
//!  
//!
//! ```text
//!                 \           . rParallel
//!                  \          .
//!                   \         .
//!                 r  \        .
//!                     \       .
//!                      \      .
//!                       \     .
//!                        \    ^
//!                         \   |
//!                          \  |  n
//!                           \ |
//!                  ..........\|
//!                 rPerp
//! ```
//!
//!  Now, what does R do to vectors that are perpendicular to n? Since R
//!    rotates each vector about n, if a vector v is perpendicular to n, then
//!    R(v) is perpendicular to n as well (remember that rotations preserve
//!    inner products, and orthogonality in particular). So rPerp just gets
//!    rotated in the plane perpendicular to n. We know from the last section
//!    how to find R(rPerp): if we let rPerp' be the vector obtained by
//!    rotating rPerp by pi/2 about n, then
//!
//!  
//!
//! ```text
//!    R(rPerp) = cos(theta) rPerp  +  sin(theta) rPerp'
//! ```
//!
//!  We will also want to know what R(rPerp') is. Since rotating rPerp' by
//!    pi/2 about n yields -rPerp, applying our familiar formula to rPerp'
//!    gives us
//!
//!  
//!
//! ```text
//!    R(rPerp') = cos(theta) rPerp'  -  sin(theta) rPerp.
//! ```
//!
//!  Now, since n, rPerp, and rPerp' are mutually orthogonal, these vectors
//!    form a basis. Since we can scale r so that rPerp has norm 1, and since
//!    rPerp' has the same norm as rPerp, we may assume that the basis is
//!    actually orthonormal.
//!
//!  The matrix of R relative to this basis is
//!
//!  
//!
//! ```text
//!    +-                              -+
//!    |  1         0            0      |
//!    |                                |
//!    |  0     cos(theta)  -sin(theta) |.
//!    |                                |
//!    |  0     sin(theta)   cos(theta) |
//!    +-                              -+
//! ```
//!
//!  Since the rotation we're representing is arbitrary, we've shown that
//!    every rotation can be represented by a matrix of the above form.
//!    Equivalently, every rotation matrix is similar to one of the above form.
//!    This fact justifies the use of the term "canonical form."
//!
//!  The canonical form we've found shows why three-dimensional rotations are
//!    very much like two-dimensional rotations: The effect of a
//!    three-dimensional rotation on any vector is to rotate the component of
//!    that vector that is normal to the rotation axis, and leave the component
//!    parallel to the rotation axis fixed.
//!
//!  This rotation matrix is a useful "model" to keep in mind when dealing
//!    with rotations because of its particularly simple form. It's easy to
//!    read off some types of information directly from this matrix.
//!
//!  Some examples:
//!
//!  
//!
//! * The trace of the above matrix is 1 + 2 cos(theta). Since the trace of a
//! matrix is invariant under similarity transformations, every rotation matrix
//! has trace equal to 1 + 2 cos(theta). So we can easily find the rotation
//! angle of any rotation matrix.
//!
//!  * Every rotation has 1 as one of its eigenvalues. We already knew that, but
//! there it is, sitting alone in its one-dimensional diagonal block. The other
//! eigenvalues are complex unless the rotation angle is a multiple of pi.
//!
//!  * Every rotation is an orthogonal mapping, that is, orthogonal vectors map to
//! orthogonal vectors. This has to be true because the canonical form is an
//! orthogonal matrix, represented relative to an orthonormal basis.
//!
//!     
//! ##  Rotation axis and angle
//!
//!  Our soccer ball model shows that a rotation has a fixed vector, called
//!    the "axis." Now if the vector n is fixed by R, then -n is fixed as
//!    well, so the direction of the rotation axis is not unique.
//!
//!  Given a rotation R and a vector v, normal to the rotation axis n of R,
//!    the angle between v and R(v), measured counterclockwise around n, is the
//!    rotation angle of R. We see that the rotation angle depends on the
//!    direction of the axis: if we pick -n as the axis, we change the sign of
//!    the angle.
//!
//!  Note that while the rotation axis and angle of a rotation are not
//!    uniquely defined, a choice of axis and angle do determine a unique
//!    rotation.
//!
//!  How do we find the rotation matrix R that rotates vectors by angle theta
//!    about the unit vector n? If n is
//!
//!  
//!
//! ```text
//!    n = (n1, n2, n3),
//! ```
//!
//!  then
//!
//!  
//!
//! ```text
//!                                       2
//!    R  =   I   +   ( 1 - cos(theta) ) N   +   sin(theta) N,
//! ```
//!
//!  where
//!
//!  
//!
//! ```text
//!        +-             -+
//!        |  0   -n3   n2 |
//!        |               |
//!    N = |  n3   0   -n1 |.
//!        |               |
//!        | -n2   n1   0  |
//!        +-             -+
//! ```
//!
//!  How do we recover the rotation angle and axis of a rotation R from a
//!    corresponding rotation matrix, M?
//!
//!  We've already seen in the "canonical form" section that the rotation
//!    angle is
//!
//!  
//!
//! ```text
//!    ACOS (  ( Trace(M) - 1 ) / 2  ).
//! ```
//!
//!  If the rotation angle is not zero or pi, then the relation
//!
//!  
//!
//! ```text
//!         T
//!    M - M  = 2 sin(theta) N
//! ```
//!
//!  allows us to recover the rotation axis n from M, while if the rotation
//!    angle is pi, we have
//!
//!  
//!
//! ```text
//!               2
//!    M = I + 2 N,
//! ```
//!
//!  again determining n.
//!
//!  In the "Mathematical road map" chapter, we'll verify these assertions.
//!
//!  
//!
//!
//!  
//! ##  Time-dependent coordinate transformations
//!
//!  Suppose we have two bases, B1 and B2, where the elements of B2,
//!    expressed relative to B1, are time dependent:
//!
//!  
//!
//! ```text
//!    B2 = { v1(t), v2(t), v3(t) }.
//! ```
//!
//!  An example of a time-dependent coordinate transformation is the
//!    transformation from J2000 to body equator and prime meridian
//!    coordinates.
//!
//!  If R(t) transforms vectors from basis B1 to basis B2, the basis vectors
//!    of B2 are the rows of the matrix R(t).
//!
//!  Let p(t) and p'(t) be position and velocity vectors expressed relative
//!    to B1. What is the corresponding velocity, expressed relative to B2? We
//!    know that p(t) has coordinates
//!
//!  
//!
//! ```text
//!    R(t) p(t)
//! ```
//!
//!  relative to B2, so the time derivative of R(p(t)) is
//!
//!  
//!
//! ```text
//!    R(t) p'(t)  +  R'(t) p(t),
//! ```
//!
//!  relative to B2.
//!
//!  If R(t) is expressed as a product of the form
//!
//!  
//!
//! ```text
//!    R(t) = [ w1(t) ]   [ w2(t) ]   [ w3(t) ] ,
//!                    i           j           k
//! ```
//!
//!  then
//!
//!  
//!
//! ```text
//!    R'(t)  =     [ w1(t) ]'  [ w2(t) ]   [ w3(t) ]
//!                          i           j           k
//!  
//!              +  [ w1(t) ]   [ w2(t) ]'  [ w3(t) ]
//!                          i           j           k
//!  
//!              +  [ w1(t) ]   [ w2(t) ]   [ w3(t) ]'
//!                          i           j           k
//! ```
//!
//!  Since we know the explicit form of the factors (given in the
//!    "Notation" section), we can compute R'(t).
//!
//!  We must take care when converting velocity vectors between systems whose
//!    bases are related in a time-dependent way. If R(t) varies extremely
//!    slowly, it is tempting to ignore the R' term, and in fact this is a
//!    valid approximation in some cases. However, since the magnitude of this
//!    term is proportional to the magnitude of p, the term can be large when R
//!    is quite slowly varying. An example:
//!
//!  Let B1 be the basis vectors of the J2000 system, and let
//!
//!  
//!
//! ```text
//!    B2  =  { v1(t), v2(t), v3(t) }
//! ```
//!
//!  be defined as follows: v1(t) is the geometric Jupiter-Sun vector at
//!    ephemeris time t, v3(t) is orthogonal to v1(t) and lies in the plane
//!    containing v1(t) and Jupiter's pole at time t, and v2(t) is the cross
//!    product of v3(t) and v1(t).
//!
//!  Let R(t) be the transformation matrix from basis B1 to B2. Then the
//!    period of R(t) is 1 Jovian year (we're ignoring movement of Jupiter's
//!    pole). Now if p(t) is the Jupiter-Sun vector in J2000 coordinates, then
//!    p'(t) is the negative of Jupiter's velocity in J2000 coordinates. But in
//!    B2 coordinates, R(t) ( p(t) ) always lies along the x-axis, and if we
//!    approximate Jupiter's motion as a circle, then R(p(t))' is the zero
//!    vector. So we have the equation
//!
//!  
//!
//! ```text
//!    R(t) p'(t)  +  R'(t) p(t)  =  [ R(t)( p(t) ) ]'  =  0,
//! ```
//!
//!  which implies
//!
//!  
//!
//! ```text
//!    R'(t)p(t)   =   - R(t) p'(t).
//! ```
//!
//!  So in this case, the term involving R' has the same magnitude as the
//!    term involving R, even though R is slowly varying.
//!
//!  
//!
//!
//!  
//! ##  Euler angles
//!
//!  Given a rotation matrix M (usually representing a coordinate
//!    transformation), we occasionally have the need to express it as the
//!    product
//!
//!  
//!
//! ```text
//!    M  =  [w1]    [w2]    [w3]  .
//!              i1      i2      i3
//! ```
//!
//!  The angles w1, w2, and w3 are called "Euler angles."
//!
//!  It is not necessarily obvious that this "factorization" is possible.
//!    It turns out that as long as i2 does not equal i1 or i3, it is possible,
//!    for any rotation matrix M. In the "Mathematical road map" chapter, we
//!    exhibit the formulas for calculating w1, w2, and w3, given M and i1, i2,
//!    and i3.
//!
//!  
//!
//!
//!  
//! ##  Quaternions
//!
//!  Quaternions are four dimensional vectors, on which a particular kind of
//!    arithmetic is defined. The quaternions that have norm equal to 1 are
//!    called "unit quaternions."
//!
//!  Unit quaternions may be associated with rotations in the following way:
//!    if a rotation R has unit vector n = (n1, n2, n3) as an axis and w as a
//!    rotation angle, then we represent R by
//!
//!  
//!
//! ```text
//!    Q = ( cos(w/2),  sin(w/2) n1,  sin(w/2) n2,  sin(w/2) n3 ).
//! ```
//!
//!  As you might suspect, this association is not unique: substituting (w +
//!    2*pi) for w, we see that -Q is also a representation for R.
//!
//!  If we choose the rotation axis and angle of R so that the angle lies in
//!    \[0, pi], then there is a unique quaternion representing R, except in the
//!    case where R is a rotation by pi radians.
//!
//!  
//!
//!
//!  
//! ##  Quaternion arithmetic
//!
//!  SPICELIB does not currently contain any routines that make use of
//!    quaternion arithmetic. Eventually, SPICELIB may use quaternion
//!    multiplication to compose rotations, since quaternion arithmetic is more
//!    efficient than matrix multiplication. At present, this section is merely
//!    for the curious.
//!
//!  
//!
//!
//!  
//! ###  Definitions
//!
//!  There are two binary operations defined on quaternions: addition and
//!    multiplication.
//!
//!  The main interest of quaternion multiplication is that we can actually
//!    carry out composition of rotations using the multiplication defined on
//!    the quaternions. If quaternions Q1 and Q2 represent rotations R1 and R2,
//!    then Q2*Q1 represents R2(R1). So the mapping from unit quaternions to
//!    rotations is a group homomorphism, where the "multiplication"
//!    operation on the rotations is functional composition.
//!
//!  Quaternion addition is simple vector addition. Multiplication is a
//!    little more complicated. Before defining it, we're going to introduce a
//!    new notation for quaternions that makes it easier to deal with products.
//!
//!  The quaternion
//!
//!  
//!
//! ```text
//!    Q = ( Q0, Q1, Q2, Q3 )
//! ```
//!
//!  can be represented as
//!
//!  
//!
//! ```text
//!    Q0 + ( Q1, Q2, Q3 ),
//! ```
//!
//!  or
//!
//!  
//!
//! ```text
//!    s + v,
//! ```
//!
//!  where s represents the "scalar" Q0 and v represents the "vector"
//!
//!  
//!
//! ```text
//!    ( Q1, Q2, Q3 ).
//! ```
//!
//!  We define the "conjugate" of the quaternion
//!
//!  
//!
//! ```text
//!    q = s + v
//! ```
//!
//!  as
//!
//!  
//!
//! ```text
//!     *
//!    q  = s - v.
//! ```
//!
//!  Given two quaternions,
//!
//!  
//!
//! ```text
//!    q1 = s1 + v1,  q2 = s2 + v2,
//! ```
//!
//!  we define the product q1 * q2 as
//!
//!  
//!
//! ```text
//!    ( s1 * s2 - < v1, v2 > )  +  ( s1 * v2  +  s2 * v1  +  v1 x v2 ).
//! ```
//!
//!  We've grouped the "scalar" and "vector" portions of the product.
//!
//!  
//!
//!
//!  
//! ###  Basic properties of multiplication
//!
//!  Let's check out a few of the properties of the multiplication formula.
//!
//!  Is multiplication commutative? No; if s1 and s2 above are zero, then the
//!    product is
//!
//!  
//!
//! ```text
//!    - < v1, v2 >   +   v1 x v2,
//! ```
//!
//!  which is not commutative. However, multiplication is associative: given
//!    three quaternions q1, q2, and q3, we have
//!
//!  
//!
//! ```text
//!    q3 * ( q2 * q1 ) = ( q3 * q2 ) * q1.
//! ```
//!
//!  We'll forgo checking this; it's messy but straightforward. If you do
//!    check it, the vector identities
//!
//!  
//!
//! ```text
//!     A x ( B x C ) =  < A, C > B  -  < A, B > C
//!  
//!    ( A x B ) x C  =  < C, A > B  -  < C, B > A
//! ```
//!
//!  will be useful.
//!
//!  What's the product of q and its conjugate? It comes out to
//!
//!  
//!
//! ```text
//!           2
//!    || q ||.
//! ```
//!
//!  What's the conjugate of the product
//!
//!  
//!
//! ```text
//!    q1 * q2,
//! ```
//!
//!  where q1 and q2 are as defined above? The product formula allows us to
//!    verify that the answer is
//!
//!  
//!
//! ```text
//!      *     *
//!    q2  * q1.
//! ```
//!
//!     
//! ###  Deducing the multiplication formula
//!
//!  One really interesting fact about the product formula is that it is a
//!    sum of binary operations, each of which is linear (where the
//!    coefficients are scalars, not quaternions) in both operands. This
//!    implies that the product formula itself is linear in q1 and q2.
//!
//!  You can check this: if we scale q1 by x, the product gets scaled by x.
//!    The same thing happens if we scale q2. If we replace q1 by the sum of
//!    two quaternions, say
//!
//!  
//!
//! ```text
//!    q1 = q + q' =  ( s + s' )  +  ( v + v' ),
//! ```
//!
//!  the product is
//!
//!  
//!
//! ```text
//!    q * q2   +   q' * q2.
//! ```
//!
//!  The analogous result occurs when we replace q2 by a sum of two
//!    quaternions.
//!
//!  Because of this linearity property, we can define multiplication on a
//!    small set of quaternions, and then define multiplication on the whole
//!    set of quaternions by insisting that the multiplication operator is
//!    linear in both operands. This gives us an equivalent definition of
//!    multiplication.
//!
//!  To carry out this definition, we first define multiplication on the four
//!    quaternions
//!
//!  
//!
//! ```text
//!    1 + ( 0, 0, 0 ), which we call ``1,''
//!    0 + ( 1, 0, 0 ), which we call ``i,''
//!    0 + ( 0, 1, 0 ), which we call ``j,''
//!    0 + ( 0, 0, 1 ), which we call ``k.''
//! ```
//!
//!  We treat "1" as a scalar and i, j, and k as vectors, and define the
//!    products
//!
//!  
//!
//! ```text
//!    1 * v   = v,  for v = i, j, k;
//!  
//!    v * v   = -1,  for v = i, j, k;
//!  
//!    v1 * v2 = - v2 * v1, for v1, v2 = i, j, k;
//!  
//!    i * j   = k;
//!    j * k   = i;
//!    k * i   = j.
//! ```
//!
//!  Multiplication of i, j, and k works just like taking cross products.
//!
//!  If we now proclaim that multiplication is linear in both operands, then
//!    since all quaternions can be expressed as linear combinations of "1,"
//!    i, j, and k, we've defined multiplication on the entire set of
//!    quaternions. You can check that this definition of multiplication is
//!    consistent with our formula above.
//!
//!  
//!
//!
//!  
//! ###  Composing rotations using quaternions
//!
//!  There is one last assertion to check: we've said that you can carry out
//!    composition of rotations using quaternion multiplication. Let's examine
//!    what that means:
//!
//!  We've defined a mapping from quaternions to rotations, since the
//!    relation
//!
//!  
//!
//! ```text
//!    Q = ( cos(w/2),  sin(w/2) n1,  sin(w/2) n2,  sin(w/2) n3 )
//! ```
//!
//!  allows us to recover w and the axis ( n1, n2, n3 ), hence the
//!    corresponding rotation. Now suppose we have two quaternions Q1 and Q2
//!    that represent rotations R1 and R2, respectively. We're claiming that
//!    the product Q2 * Q1 represents R2(R1). So, we should be able to recover
//!    the rotation axis and angle of R2(R1) from the quaternion Q2 * Q1. In
//!    the "Mathematical road map' chapter, we will verify this claim.
//!
//!  
//!
//!
//!  
//! #  Mathematical road map
//!
//!  The purpose of this chapter is to familiarize you with the mathematical
//!    ideas essential to dealing with rotations. If you understand the
//!    relevant mathematics, you are in a position to judge the merits of
//!    alternative software designs based on SPICELIB routines. If you don't
//!    understand the mathematics, you can still build programs that work by
//!    paying careful attention to routine interface specifications, but the
//!    design process is more error-prone, and you're unlikely to hit upon
//!    efficient and elegant solutions.
//!
//!  The difference between the two perspectives is a bit like the difference
//!    between having a set of directions to get from point A to point B, and
//!    having a road map of the entire area.
//!
//!  This chapter is not organized sequentially, since there is little
//!    logical dependence of one section on another. It is simply a collection
//!    of discussions.
//!
//!  
//!
//!
//!  
//! ##  Rotation of a vector about an axis
//!
//!  Suppose we have a unit vector n, and we wish to rotate a vector r about
//!    n by an angle of theta radians. What's the resulting vector?
//!
//!  As in the tutorial discussion of the canonical form for rotations, we
//!    can express r as the sum of two orthogonal components:
//!
//!  
//!
//! ```text
//!    r  =  rParallel + rPerp.
//! ```
//!
//!  Let's give the name rPerp' to the vector obtained by rotating rPerp by
//!    pi/2 radians about n.
//!
//!  We know, from the results of the "canonical form" section, that
//!    applying our rotation to r will yield
//!
//!  
//!
//! ```text
//!    rParallel  +  cos(theta) rPerp  +  sin(theta) rPerp'
//! ```
//!
//!  So all we have to do is find rPerp and rPerp' in terms of r, n, and
//!    theta.
//!
//!  It turns out that rPerp' is precisely n x r, since n x r is parallel to
//!    rPerp' and has the same magnitude as rPerp, namely
//!
//!  
//!
//! ```text
//!    |r| sin(phi),
//! ```
//!
//!  where phi is the angle between r and n. Rotating rPerp' by another pi/2
//!    radians yields -rPerp, so
//!
//!  
//!
//! ```text
//!    rPerp = -n x ( n x r ).
//! ```
//!
//!  In the picture below,
//!
//!  
//!
//! * The diagonal, dashed line segment represents r.
//!
//!  * The short, vertical, dashed line segment represents n.
//!
//!  * The dotted, vertical extension of n represents the projection of r onto n.
//!
//!  * The dotted, horizontal segments represent
//!
//!  ```text
//!    n x ( n x r)
//! ```
//!  *  on the right, and
//!
//!  ```text
//!    - n x ( n x r)
//! ```
//!  *  on the left.
//!
//!  * The dotted, diagonal segment represents n x r.
//!
//!  * The little boxes at the intersection of the segments are supposed to
//! indicate orthogonality.
//!
//!  * The ugly, little segments with the label "phi" between them are supposed
//! to indicate the angle phi.
//!
//!  ```text
//!                       \           .  rParallel
//!                        \    phi --.
//!                         \ /       .
//!                       r  \        .
//!                           \       .
//!                            \      .
//!                             \     .
//!                              \    ^
//!                               \   |
//!                                \  |    n
//!                                 \ |_
//!      - n x ( n x r )   ...........|_|.........   n  x  ( n x r )
//!                                  /_/
//!          = rPerp                .                     =  - rPerp
//!                                .
//!                               .
//!                              .  n x r
//!  
//!                                      = rPerp'
//! ```
//!  Now we're ready to compute the image of r under the rotation. It is:
//!
//!  
//!
//! ```text
//!          rParallel     +   cos(theta) rPerp
//!                        +   sin(theta) rPerp'
//!  
//!    =   ( r - rPerp )   +   cos(theta) rPerp
//!                        +   sin(theta) rPerp'
//!  
//!    =        r          +   ( cos(theta) - 1 )   rPerp
//!                        +     sin(theta)         rPerp'
//!  
//!    =        r          +   ( 1 - cos(theta) )  ( n x ( n x r ) )
//!                        +      sin(theta)       (     n x r     ).
//! ```
//!
//!  This is what we were after: an expression for the image of r under the
//!    rotation, given in terms of r, theta, and n.
//!
//!  
//!
//!
//!  
//! ##  Formation of a rotation matrix from axis and angle
//!
//!  In this section, we derive an expression for a rotation matrix that
//!    explicitly relates the matrix to the rotation axis and angle. This
//!    expression is valuable for understanding how to find the rotation axis
//!    and angle of a rotation matrix, as well as how to build a rotation
//!    matrix having a given rotation axis and angle. The problem of finding a
//!    quaternion corresponding to a specified rotation matrix is also solved
//!    by the expression derived here.
//!
//!  What's the rotation matrix R that rotates vectors by theta radians about
//!    the vector n? If n is a unit vector, then the result of the last section
//!    implies that
//!
//!  
//!
//! ```text
//!    R * r  =  r     +   ( 1 - cos(theta) )  ( n x ( n x r ) )
//!                    +      sin(theta)       (     n x r     ),
//! ```
//!
//!  for any vector r. Now, let
//!
//!  
//!
//! ```text
//!    n = (n1, n2, n3),
//! ```
//!
//!  and define the matrix N by
//!
//!  
//!
//! ```text
//!        +-             -+
//!        |  0   -n3   n2 |
//!        |               |
//!    N = |  n3   0   -n1 |;
//!        |               |
//!        | -n2   n1   0  |
//!        +-             -+
//! ```
//!
//!  this definition implies that
//!
//!  
//!
//! ```text
//!    N * r = n x r
//! ```
//!
//!  for all r. So we can rewrite the above expression as
//!
//!  
//!
//! ```text
//!    R * r  =  r     +   ( 1 - cos(theta) )  ( N * ( N * r ) )
//!                    +      sin(theta)       (     N * r     ),
//! ```
//!
//!  or
//!
//!  
//!
//! ```text
//!                                           2
//!    R * r  =  [  I  +  ( 1 - cos(theta) ) N  +  sin(theta) N  ]  r.
//! ```
//!
//!  Since r is arbitrary, we must have
//!
//!  
//!
//! ```text
//!                                       2
//!    R  =   I   +   ( 1 - cos(theta) ) N   +   sin(theta) N.
//! ```
//!
//!  R is the desired matrix.
//!
//!  
//!
//!
//!  
//! ##  Finding the axis and angle of a rotation matrix
//!
//!  The topic is covered in reference \[1], so we'll just make a few notes.
//!
//!  There are many ways to recover the rotation axis. The most elegant
//!    method we know of is presented in \[1]. The idea is based on the
//!    observation that any rotation matrix R can be expressed by
//!
//!  
//!
//! ```text
//!                                       2
//!    R  =   I   +   ( 1 - cos(theta) ) N   +   sin(theta) N,
//! ```
//!
//!  where N is derived from the rotation axis, as in the last section. Now N
//!    is skew-symmetric and N squared is symmetric, so
//!
//!  
//!
//! ```text
//!         T
//!    R - R   =  2 sin(theta) N.
//! ```
//!
//!  As long as sin(theta) is non-zero, we've found N and hence the axis
//!    itself. If theta is pi, we have
//!
//!  
//!
//! ```text
//!                    2
//!    R  =   I  +  2 N,
//! ```
//!
//!  which still allows us to recover the axis.
//!
//!  In the tutorial section, we showed that the rotation angle can be
//!    recovered from the trace of a rotation matrix:
//!
//!  
//!
//! ```text
//!    angle = ACOS (  ( trace - 1 ) / 2  ).
//! ```
//!
//!  If the angle is very small, we will determine it more accurately from
//!    the relation
//!
//!  
//!
//! ```text
//!         T
//!    R - R   =  2 sin(theta) N.
//! ```
//!
//!     
//! ##  Formation of a rotation matrix from a quaternion
//!
//!  Since the quaternion gives us a rotation's axis and angle, an earlier
//!    discussion in this chapter gives us one way of recovering the rotation
//!    matrix: twice the arccosine of the first component of the quaternion
//!    gives us the rotation angle, and the rest of the quaternion is the
//!    rotation axis, so [AXISAR](crate::raw::axisar) can be used to form the matrix. In this
//!    approach, we may want to treat small rotation angles as a special case,
//!    since the arccosine function is very inaccurate when the argument is
//!    close to 1. We would use the norm of the "vector" portion of the
//!    quaternion to give us the sine of half the rotation angle instead, and
//!    recover the rotation angle from this.
//!
//!  There is a fast, accurate solution available. It depends on the formula
//!    relating a rotation matrix to its axis and angle, which we derived
//!    earlier in the chapter. In this approach, we compute the matrix
//!    corresponding to a quaternion, component by component.
//!
//!  Define
//!
//!  
//!
//! ```text
//!    c = cos(theta/2),
//!  
//!    s = sin(theta/2),
//! ```
//!
//!  and let the quaternion
//!
//!  
//!
//! ```text
//!    q  =   c   +  s n
//!       =   q0  +  s ( q1, q2, q3 )
//! ```
//!
//!  represent a rotation R having unit axis vector n and rotation angle
//!    theta.
//!
//!  If n = ( n1, n2, n3 ), and we define the matrix N by
//!
//!  
//!
//! ```text
//!        +-             -+
//!        |  0   -n3   n2 |
//!        |               |
//!    N = |  n3   0   -n1 |,
//!        |               |
//!        | -n2   n1   0  |
//!        +-             -+
//! ```
//!
//!  then the matrix M representing R is
//!
//!  
//!
//! ```text
//!                                       2
//!    M  =   I   +   ( 1 - cos(theta) ) N   +   sin(theta) N.
//! ```
//!
//!  Now we can make the substitutions
//!
//!  
//!
//! ```text
//!    sin(theta) = 2 c s,
//!  
//!                            2
//!    ( 1 - cos(theta) ) = 2 s
//! ```
//!
//!  to obtain
//!
//!  
//!
//! ```text
//!                            2
//!    M  =   I   +   2 ( s N )   +   2 c ( s N ).
//! ```
//!
//!  Substituting the elements of our quaternion into s N, we find
//!
//!  
//!
//! ```text
//!                             +-                                     -+
//!                             |     2    2                            |
//!            +-       -+      | -(q2 + q3 )     q1 q2        q1 q3    |
//!            | 1  0  0 |      |                                       |
//!            |         |      |                  2    2               |
//!    M  =    | 0  1  0 | + 2  |    q1 q2     -(q1 + q3 )     q2 q3    |
//!            |         |      |                                       |
//!            | 0  0  1 |      |                               2    2  |
//!            +-       -+      |    q1 q3        q2 q3     -(q1 + q2 ) |
//!                             +-                                     -+
//!  
//!                             +-                                     -+
//!                             |                                       |
//!                             |      0         -q0 q3       q0 q2     |
//!                             |                                       |
//!                             |                                       |
//!                        + 2  |     q0 q3         0        -q0 q1     |,
//!                             |                                       |
//!                             |                                       |
//!                             |    -q0 q2       q0 q1         0       |
//!                             +-                                     -+
//! ```
//!
//!  so
//!
//!  
//!
//! ```text
//!        +-                                                          -+
//!        |           2    2                                           |
//!        | 1 - 2 ( q2 + q3 )    2 (q1 q2 - q0 q3)   2 (q1 q3 + q0 q2) |
//!        |                                                            |
//!        |                                                            |
//!        |                               2    2                       |
//!    M = | 2 (q1 q2 + q0 q3)    1 - 2 ( q1 + q3 )   2 (q2 q3 - q0 q1) |.
//!        |                                                            |
//!        |                                                            |
//!        |                                                    2    2  |
//!        | 2 (q1 q3 - q0 q2)    2 (q2 q3 + q0 q1)   1 - 2 ( q1 + q2 ) |
//!        |                                                            |
//!        +-                                                          -+
//! ```
//!
//!     
//! ##  Equivalence of rotation definitions
//!
//!  The idea discussed here is used implicitly throughout the SPICELIB
//!    rotation routines.
//!
//!  We wish to prove that definitions (1) and (2) from the "Definition of
//!    rotations" section of the tutorial are equivalent. To do this, we need
//!    to show that a mapping R that satisfies definition (1) also satisfies
//!    definition (2). This amounts to showing that R has a fixed subspace of
//!    dimension 1, or equivalently, that R has 1 as one of its eigenvalues.
//!
//!  
//!
//!
//!  
//! ###  An algebraic approach
//!
//!  We observe that the characteristic polynomial of a rotation is of degree
//!    three, and so has either zero or two complex roots, hence at least one
//!    real root. Because rotations preserve norms, the magnitudes of all of
//!    the roots (eigenvalues), real or complex, are equal to one. So the real
//!    roots are 1 or -1. The determinant of any rotation matrix is 1, since
//!    the determinant of any 3 by 3 matrix is the dot product of the third
//!    column with the cross product of the first and second columns, and for
//!    rotations, this cross product is the third column. But the determinant
//!    is also the product of the eigenvalues. In the case where all three
//!    roots are 1 or -1, we cannot get a product of 1 unless at least one
//!    eigenvalue is equal to 1. If there are complex roots, they are complex
//!    conjugates, so their product is 1, which implies that the real root must
//!    be 1 as well, if the product of all three roots is 1.
//!
//!  
//!
//!
//!  
//! ###  A geometric approach
//!
//!  Again, we assume that our rotation R satisfies definition (1), and we
//!    prove that R has a fixed axis.
//!
//!  We're going to look at the effect of R on the unit sphere, and
//!    demonstrate that two points on the sphere are fixed. We'll assume that
//!    the rotation is not the identity and does not map any vector v to -v.
//!    This last case corresponds to a rotation of pi radians.
//!
//!  Our first observation is that R maps great circles to great circles.
//!    This follows from the fact that a great circle is a set of unit vectors,
//!    all orthogonal to some particular vector v. Since R preserves inner
//!    products, the image of the great circle is a set of unit vectors, all
//!    orthogonal to R(v).
//!
//!  Now, consider the distances that vectors on the unit sphere move when
//!    the rotation R is applied; there is some vector v, not necessarily
//!    unique, that moves the maximum distance. Let C1 be a great circle
//!    passing through v and R(v), and let C2 be a great circle that passes
//!    through v and intersects C1 at right angles. Now R(C2) passes through
//!    R(v), and if we can show that it passes through at right angles to C1,
//!    then C2 and R(C2) intersect at vectors p and -p, both of which are
//!    normal to v and R(v). So R(p) is either p or -p. But we've assumed that
//!    R does not map any vector to its inverse, so R(p) = p, and we have a
//!    fixed vector.
//!
//!  So, we must show that R(C2) passes through R(v) at right angles to C1.
//!    If it did not, there would be some point w on C2, close to v, such that
//!
//!  
//!
//! ```text
//!    || R(w) - w ||  >  || R(v) - v ||,
//! ```
//!
//!  contradicting our hypothesis that no vector moves farther that v. We
//!    will leave the rigorous proof of this last assertion to the energetic
//!    reader.
//!
//!  
//!
//!
//!  
//! ##  Quaternion multiplication
//!
//!  In this section, we verify some claims made in the tutorial on
//!    rotations.
//!
//!  There are two assertions that we need to prove:
//!
//!  
//!
//! *  1. If the unit quaternion q represents the rotation R, then for any vector v,
//!
//!  ```text
//!                      *
//!    R(v)  =  q * v * q,
//! ```
//!  *  where v is treated as a quaternion.
//!
//!  *  2. If the unit quaternions q1 and q2 represent the rotations R1 and R2, then
//! the rotation R2(R1) is represented by the quaternion
//!
//!  ```text
//!    q2 * q1.
//! ```
//!     
//! ###  Assertion 1
//!
//!  To prove the first assertion, we express R(v) in the form
//!
//!  
//!
//! ```text
//!    R(v) =  v + sin(theta) n x v  + ( 1 - cos(theta) ) n x ( n x v ),
//! ```
//!
//!  where n is a unit axis vector and theta is the corresponding rotation
//!    angle. We also define the constants C and S by
//!
//!  
//!
//! ```text
//!    C = cos(theta/2),
//!    S = sin(theta/2).
//! ```
//!
//!  The quaternion
//!
//!  
//!
//! ```text
//!    q = C  +  S n
//! ```
//!
//!  represents R. To check the assertion, we compute
//!
//!  
//!
//! ```text
//!         ( C  +  S n ) * v * ( C  -  S n )
//!  
//!  
//!    =    ( C  +  S n ) * [ ( S  <v,n> )  +  ( C v  -  S v x n ) ]
//!  
//!                                            2
//!    =    [ C S <v, n>  -  S C < n, v >  +  S  < n,  v x n > ]
//!  
//!            2                    2
//!      +  [ C v  - C S v x n  +  S <v, n> n  +  S C n x v
//!  
//!          2
//!      -  S n x ( v x n ) ].
//! ```
//!
//!  Since n is normal to v x n, the scalar part of the last line is zero,
//!    which leaves us with
//!
//!  
//!
//! ```text
//!  
//!     2       2                              2
//!    C v  +  S <v, n> n  +  2 S C n x v  -  S n x ( v x n ).
//! ```
//!
//!  We can re-write this again as
//!
//!  
//!
//! ```text
//!     2       2                               2
//!    C v  +  S <v, n> n  +  2 S C n x v  + 2 S n x ( n x v )
//!  
//!                                             2
//!                                        -   S n x ( n x v ),
//! ```
//!
//!  and using the vector identity
//!
//!  
//!
//! ```text
//!    A x ( B x C ) =  < A, C > B  -  < A, B > C,
//! ```
//!
//!  we can modify the final term to arrive at
//!
//!  
//!
//! ```text
//!     2       2                               2
//!    C v  +  S <v, n> n  +  2 S C n x v  + 2 S n x ( n x v )
//!  
//!                                             2
//!                                        -   S ( <n, v> n - <n, n> v ).
//! ```
//!
//!  Since n is a unit vector, the entire expression reduces to
//!
//!  
//!
//! ```text
//!                                  2
//!         v  +  2 S C n x v  +  2 S  n x ( n x v )
//!  
//!    =    v  +  2 sin(theta/2) cos(theta/2) n x v
//!  
//!                    2
//!            +  2 sin (theta/2) n x ( n x v)
//!  
//!    =    v  +  sin(theta) n x v  +  ( 1 - cos(theta) ) n x ( n x v )
//!  
//!    =    R(v).
//! ```
//!
//!     
//! ###  Assertion 2
//!
//!  The second assertion follows rather more quickly from the first. Given
//!    that R1(v) is
//!
//!  
//!
//! ```text
//!               *
//!    q1 * v * q1 ,
//! ```
//!
//!  we can express R2(R1(v)) by
//!
//!  
//!
//! ```text
//!                         *      *
//!       q2 * ( q1 * v * q1 ) * q2
//!                                    *
//!    =  ( q2 * q1 ) * v * ( q2 * q1 ).
//! ```
//!
//!  Now let q be a quaternion that represents R2(R1); then
//!
//!  
//!
//! ```text
//!             *
//!    q * v * q   =  R2(R1(v))
//! ```
//!
//!  for all v. We'll be done if we can show that, in general, for unit
//!    quaternions x and y, if
//!
//!  
//!
//! ```text
//!             *             *
//!    x * v * x  =  y * v * y
//! ```
//!
//!  for all vectors v, then x equals y or -y. But this equation implies that
//!
//!  
//!
//! ```text
//!     *                 *
//!    y * x * v  =  v * y * x,
//! ```
//!
//!  for all v, which in turn implies that
//!
//!  
//!
//! ```text
//!     *
//!    y * x
//! ```
//!
//!  is a scalar, since only scalar quaternions commute with every vector
//!    quaternion (due to the cross product term in the product formula). Since
//!    y and x are unit quaternions, either
//!
//!  
//!
//! ```text
//!     *
//!    y * x = 1
//! ```
//!
//!  or
//!
//!  
//!
//! ```text
//!     *
//!    y * x = -1,
//! ```
//!
//!  so x = y or -y.
//!
//!  We conclude that q2 * q1 = q or -q, so q2 * q1 does represent R2(R1).
//!
//!  
//!
//!
//!  
//! ##  Recovery of Euler angles from a rotation matrix
//!
//!  Here's the problem: Given a rotation matrix M, and a set of coordinate
//!    axes indexed by i1, i2, i3, find angles w1, w2, w3 such that
//!
//!  
//!
//! ```text
//!    M = [w1]     [w2]      [w3]  .                       (1)
//!            i1       i2        i3
//! ```
//!
//!  There are a couple of reasons why we might want to solve this problem:
//!    first, the representation of a rotation by three Euler angles is a
//!    common one, so it is convenient to be able to convert the matrix
//!    representation to this form. Also, the three angles on the right hand
//!    side of equation (1) often allow you to visualize a rotation more
//!    readily than does the matrix representation M.
//!
//!  This "factorization" is possible if i2 does not equal i1 or i3. For
//!    each valid sequence (i1-i2-i3) of axes, there is a set of functions that
//!    give us w1, w2, and w3 as a function of M:
//!
//!  
//!
//! ```text
//!    w1 = f1         ( M ),
//!           i1-i2-i3
//!  
//!    w2 = f2         ( M ),
//!           i1-i2-i3
//!  
//!    w3 = f3         ( M ).
//!           i1-i2-i3
//! ```
//!
//!  How can we derive the functions
//!
//!  
//!
//! ```text
//!    f1        ,  f2        ,  f3        ?
//!      i1-i2-i3     i1-i2-i3     i1-i2-i3
//! ```
//!
//!  One approach is to multiply the matrices on the right hand side of
//!    equation (1); this yields a matrix whose entries are sums of products of
//!    sines and cosines of w1, w2, and w3. We can then equate the entries of
//!    this matrix to those of M, and find formulas for w1, w2, and w3 that
//!    arise from the component-wise correspondence. In subsequent sections, we
//!    actually carry out this procedure for 3-1-3 and 1-2-3 factorizations.
//!
//!  There are twelve sets of axes to consider, so there are potentially
//!    twelve sets of functions to compute. However, the procedure we've just
//!    described is not enjoyable enough to justify doing it twelve times. We'd
//!    like to find a slicker way of solving the problem. One approach is to
//!    find a way of "recycling" the formulas we derived for one particular
//!    axis sequence. Here's an example of how we might do this:
//!
//!  Suppose that we already have functions
//!
//!  
//!
//! ```text
//!    f1     ,  f2     ,  f3
//!      3-1-3     3-1-3     3-1-3
//! ```
//!
//!  that allow us to factor rotation matrix M as a 3-1-3 product:
//!
//!  If
//!
//!  
//!
//! ```text
//!    M = [w1]   [w2]    [w3] .                            (2)
//!            3      1       3
//! ```
//!
//!  then
//!
//!  
//!
//! ```text
//!    w1 = f1      ( M ),
//!           3-1-3
//!  
//!    w2 = f2      ( M ),
//!           3-1-3
//!  
//!    w3 = f3      ( M ).
//!           3-1-3
//! ```
//!
//!  We'd like to somehow use the functions we've already got to factor M as
//!    a 2-3-2 product: we want to find functions
//!
//!  
//!
//! ```text
//!    f1     ,   f2     ,   f3
//!      2-3-2      2-3-2      2-3-2
//! ```
//!
//!  such that
//!
//!  
//!
//! ```text
//!    M = [y1]   [y2]   [y3] ,                             (3)
//!            2      3      2
//! ```
//!
//!  and
//!
//!  
//!
//! ```text
//!    y1 = f1      ( M ),
//!           2-3-2
//!  
//!    y2 = f2      ( M ),
//!           2-3-2
//!  
//!    y3 = f3      ( M )
//!           2-3-2
//! ```
//!
//!  without having to derive
//!
//!  
//!
//! ```text
//!    f1     ,   f2     ,   f3
//!      2-3-2      2-3-2      2-3-2
//! ```
//!
//!  from scratch.
//!
//!  We'll start out by using a new basis, relative to which the right hand
//!    side of (3) is not a 2-3-2, but rather a 3-1-3 rotation. It is important
//!    to note here that bases are ordered sets of vectors; changing the order
//!    changes the basis.
//!
//!  Let the basis B1 be the ordered set of vectors
//!
//!  
//!
//! ```text
//!    {e(1), e(2), e(3)},
//! ```
//!
//!  and let the basis B2 be the ordered set of vectors
//!
//!  
//!
//! ```text
//!    {e(3), e(1), e(2)}.
//! ```
//!
//!  Now the rotation matrix
//!
//!  
//!
//! ```text
//!    [y]
//!       2
//! ```
//!
//!  expressed relative to B1 represents the same rotation as the matrix
//!
//!  
//!
//! ```text
//!    [y]
//!       3
//! ```
//!
//!  expressed relative to B2. Both matrices represent a rotation of y
//!    radians about the vector e(2). Similarly, the matrix
//!
//!  
//!
//! ```text
//!    M  =  [y1]  [y2]  [y3]
//!              2     3     2
//! ```
//!
//!  expressed relative to B1 represents the same rotation as the matrix
//!
//!  
//!
//! ```text
//!    M' =  [y1]  [y2]  [y3]
//!              3     1     3
//! ```
//!
//!  expressed relative to B2. So if C is the matrix whose columns are the
//!    elements of B2, expressed relative to B1, namely
//!
//!  
//!
//! ```text
//!         +-       -+
//!         | 0  1  0 |
//!    C =  | 0  0  1 |,
//!         | 1  0  0 |
//!         +-       -+
//! ```
//!
//!  then
//!
//!  
//!
//! ```text
//!     -1
//!    C   M  C   =  M'                                     (4)
//! ```
//!
//!  We can use the functions
//!
//!  
//!
//! ```text
//!    f1     ,  f2     ,  f3
//!      3-1-3     3-1-3     3-1-3
//! ```
//!
//!  to factor M' as a 3-1-3 product: applying (4), we have
//!
//!  
//!
//! ```text
//!               -1
//!    y1 = f1 ( C   M  C ),                                (5)
//!  
//!               -1
//!    y2 = f2 ( C   M  C ),                                (6)
//!  
//!               -1
//!    y3 = f3 ( C   M  C ),                                (7)
//! ```
//!
//!  so we've found functions that yield the angles y1, y2 and y3 that we
//!    sought. "No muss, no fuss."
//!
//!  How much mileage can we get out of our 3-1-3 factorization functions?
//!    Looking at our example, we see that the main "trick" is to find a
//!    basis so that the factorization we want is a 3-1-3 factorization with
//!    respect to that basis. It is important the new basis be right-handed;
//!    otherwise the form of the matrices
//!
//!  
//!
//! ```text
//!    [w]
//!       i
//! ```
//!
//!  is not preserved. It turns out that for any axis sequence of the form
//!    a-b-a, we can find a right-handed basis such that the factorization we
//!    want is a 3-1-3 factorization with respect to that basis. There are two
//!    cases: if we define a successor function s on the integers 1, 2, 3 such
//!    that
//!
//!  
//!
//! ```text
//!    s(1) = 2,
//!    s(2) = 3,
//!    s(3) = 1,
//! ```
//!
//!  we either have b = s(a) or a = s(b).
//!
//!  In the first case, b = s(a), and if our original ordered basis is
//!
//!  
//!
//! ```text
//!    B1 = { e(1), e(2), e(3) },
//! ```
//!
//!  then
//!
//!  
//!
//! ```text
//!    B2 = { e(b), e( s(b) ), e(a) }
//! ```
//!
//!  is the right-handed basis we're looking for. You can check that
//!
//!  
//!
//! ```text
//!    e(a) = e(b) x e( s(b) ).
//! ```
//!
//!  We recall that the transformation matrix C we require has the elements
//!    of B2 as columns.
//!
//!  For example, if a is 2 and b is 3, then B2 is
//!
//!  
//!
//! ```text
//!    { e(3), e(1), e(2) },
//! ```
//!
//!  and the matrix C is
//!
//!  
//!
//! ```text
//!         +-       -+
//!         | 0  1  0 |
//!    C =  | 0  0  1 |,
//!         | 1  0  0 |
//!         +-       -+
//! ```
//!
//!  as we have seen previously.
//!
//!  The axis sequences that can be handled by the above procedure are 1-2-1,
//!    2-3-2, and 3-1-3.
//!
//!  In the second case, a = s(b), and if our original ordered basis is
//!
//!  
//!
//! ```text
//!    B1 = { e(1), e(2), e(3) },
//! ```
//!
//!  then
//!
//!  
//!
//! ```text
//!    B2 = { e(b), -e( s(a) ), e(a) }
//! ```
//!
//!  is the right-handed basis we're looking for. Again, you can verify this
//!    by taking cross products. The transformation matrix C we require has the
//!    elements of B2 as columns.
//!
//!  For example, if a = 2 and b = 1, then B2 is
//!
//!  
//!
//! ```text
//!    { e(1), -e(3), e(2)) }
//! ```
//!
//!  and the matrix C is
//!
//!  
//!
//! ```text
//!         +-       -+
//!         | 1  0  0 |
//!    C =  | 0  0  1 |.
//!         | 0 -1  0 |
//!         +-       -+
//! ```
//!
//!  The axis sequences that can be handled by the above procedure are 1-3-1,
//!    2-1-2, and 3-2-3. So we can use our 3-1-3 formula to handle all of the
//!    a-b-a factorizations, just by computing the correct transformation
//!    matrix C.
//!
//!  What about a-b-c factorizations? As you might guess, the procedure we've
//!    described also applies to these, with very little modification.
//!
//!  Suppose we have the formulas we need to carry out a 1-2-3 factorization.
//!    We'd like to find a basis that allows us to represent the a-b-c product
//!
//!  
//!
//! ```text
//!    [w1]   [w2]   [w3]
//!        a      b      c
//! ```
//!
//!  as the product
//!
//!  
//!
//! ```text
//!    [w1]   [w2]   [w3] .
//!        1      2      3
//! ```
//!
//!  Again, there are two cases, depending on whether b is the successor of a
//!    or a is the successor of b, according to our cyclic ordering.
//!
//!  In the case where b is the successor of a, the right-handed basis we
//!    want is
//!
//!  
//!
//! ```text
//!    B2 = { e(a), e(b), e(c) }.
//! ```
//!
//!  With respect to the basis B2, our a-b-c factorization is a 1-2-3
//!    factorization. Again, we can form the transformation matrix C by letting
//!    its columns be the elements of B2.
//!
//!  In the second case, a is the successor of b. Our new basis is
//!
//!  
//!
//! ```text
//!    B2 = { e(a), e(b), -e(c)}.
//! ```
//!
//!  In this case, there is a slight twist: the change of basis we use
//!    negates the third rotation angle. This is not a serious problem; the
//!    change of basis converts the product
//!
//!  
//!
//! ```text
//!    [w1]   [w2]   [w3]
//!        a      b      c
//! ```
//!
//!  to
//!
//!  
//!
//! ```text
//!    [w1]   [w2]   [-w3] ,
//!        1      2       3
//! ```
//!
//!  so we can still recover the angles w1, w2, and w3 easily. So our 1-2-3
//!    factorization formula allows us to handle all the a-b-c factorizations.
//!
//!  Having shown that we can perform all of the a-b-a and a-b-c
//!    factorizations using just one formula for each type of factorization, we
//!    now proceed to derive those formulas. This is not a particularly
//!    instructive procedure, but the derivations ought to be written down
//!    somewhere, and this is as good a place as any.
//!
//!  
//!
//!
//!  
//! ###  Euler angle recovery: a-b-a case
//!
//!  We'll derive the formulas for a 3-1-3 factorization.
//!
//!  In this case, the right hand side of (1) is
//!
//!  
//!
//! ```text
//!    +-                     -+     +-                      -+
//!    |  cos(w1)  sin(w1)  0  |     | 1       0        0     |
//!    | -sin(w1)  cos(w1)  0  |  *  | 0    cos(w2)  sin(w2)  |  *
//!    |      0       0     1  |     | 0   -sin(w2)  cos(w2)  |
//!    +-                     -+     +-                      -+
//!  
//!    +-                     -+
//!    |  cos(w3)  sin(w3)  0  |
//!    | -sin(w3)  cos(w3)  0  |
//!    |     0        0     1  |
//!    +-                     -+
//! ```
//!
//!  which equals
//!
//!  
//!
//! ```text
//!    +-                     -+
//!    |  cos(w1)  sin(w1)  0  |
//!    | -sin(w1)  cos(w1)  0  |   *
//!    |      0       0     1  |
//!    +-                     -+
//!    +-                                            -+
//!    |          cos(w3)       sin(w3)         0     |
//!    |  -cos(w2)sin(w3)   cos(w2)cos(w3)   sin(w2)  |
//!    |   sin(w2)sin(w3)  -sin(w2)cos(w3)   cos(w2)  |
//!    +-                                            -+
//! ```
//!
//!  which comes out to
//!
//!  
//!
//! ```text
//!    +-                                                              -+
//!    |         cos(w1)cos(w3)          cos(w1)sin(w3)  sin(w1)sin(w2) |
//!    | -sin(w1)cos(w2)sin(w3)  +sin(w1)cos(w2)cos(w3)                 |
//!    |                                                                |
//!    |        -sin(w1)cos(w3)         -sin(w1)sin(w3)  cos(w1)sin(w2) |.
//!    | -cos(w1)cos(w2)sin(w3)  +cos(w1)cos(w2)cos(w3)                 |
//!    |                                                                |
//!    |         sin(w2)sin(w3)         -sin(w2)cos(w3)         cos(w2) |
//!    +-                                                              -+
//! ```
//!
//!  At this point, we can recover w1, w2, and w3 from the elements of M. The
//!    inverse trigonometric functions used below are borrowed from Fortran. We
//!    find w2 from the relation
//!
//!  
//!
//! ```text
//!    w2 = ACOS( M(3,3) ).
//! ```
//!
//!  (So w2 is in \[0, pi].)
//!
//!  If w2 is not equal to 0 or pi, then we can recover w1 as follows:
//!
//!  
//!
//! ```text
//!    M(1,3)       sin(w1)sin(w2)
//!    ------   =   --------------  =  tan(w1),
//!    M(2,3)       cos(w1)sin(w2)
//!  
//!    w1 = ATAN2 ( M(1,3), M(2,3) ).
//! ```
//!
//!  We find w3 in an analogous fashion, again assuming w2 is not equal to 0
//!    or pi. We find
//!
//!  
//!
//! ```text
//!     M(3,1)       sin(w2)sin(w3)
//!    -------   =   --------------  =  tan(w3),
//!    -M(3,2)       sin(w2)cos(w3)
//!  
//!    w3 = ATAN2 ( M(3,1), -M(3,2) ).
//! ```
//!
//!  Note the minus sign used in the second ATAN2 argument. For ATAN2 to
//!    determine the correct value, it is necessary that the first and second
//!    arguments have the same signs as sin(w3) and cos(w3), respectively.
//!
//!  Now if w2 is equal to zero or pi, we have a degenerate case: M is the
//!    product of two rotations about the third coordinate axis. The angles of
//!    the rotations are not determined uniquely, only the sum of the angles
//!    is. One way of finding a factorization is to set w3 to zero, and solve
//!    for w1. The matrix M then is equal to
//!
//!  
//!
//! ```text
//!    +-                                 -+
//!    |  cos(w1)  cos(w2)sin(w1)     0    |
//!    | -sin(w1)  cos(w2)cos(w1)     0    |,
//!    |    0            0         cos(w2) |
//!    +-                                 -+
//! ```
//!
//!  so we can recover w1 by computing
//!
//!  
//!
//! ```text
//!    w1 = ATAN2( -M(2,1), M(1,1) ).
//! ```
//!
//!     
//! ###  Euler angle recovery: a-b-c case
//!
//!  We'll derive the formulas for a 1-2-3 factorization.
//!
//!  In this case, the right hand side of (1) is
//!
//!  
//!
//! ```text
//!    +-                      -+     +-                      -+
//!    | 1       0        0     |     | cos(w2)   0  -sin(w2)  |
//!    | 0    cos(w1)  sin(w1)  |  *  |    0      1      0     |  *
//!    | 0   -sin(w1)  cos(w1)  |     | sin(w2)   0   cos(w2)  |
//!    +-                      -+     +-                      -+
//!  
//!    +-                      -+
//!    |  cos(w3)  sin(w3)  0   |
//!    | -sin(w3)  cos(w3)  0   |
//!    |     0        0     1   |
//!    +-                      -+
//! ```
//!
//!  which equals
//!
//!  
//!
//! ```text
//!    +-                      -+
//!    | 1       0        0     |
//!    | 0    cos(w1)  sin(w1)  |  *
//!    | 0   -sin(w1)  cos(w1)  |
//!    +-                      -+
//!  
//!    +-                                              -+
//!    | cos(w2)cos(w3)   cos(w2)sin(w3)      -sin(w2)  |
//!    |                                                |
//!    |       -sin(w3)          cos(w3)          0     |
//!    |                                                |
//!    | sin(w2)cos(w3)   sin(w2)sin(w3)       cos(w2)  |
//!    +-                                              -+
//! ```
//!
//!  which comes out to
//!
//!  
//!
//! ```text
//!    +-                                                              -+
//!    |         cos(w2)cos(w3)          cos(w2)sin(w3)        -sin(w2) |
//!    |                                                                |
//!    |        -cos(w1)sin(w3)          cos(w1)cos(w3)  sin(w1)cos(w2) |
//!    | +sin(w1)sin(w2)cos(w3)  +sin(w1)sin(w2)sin(w3)                 |.
//!    |                                                                |
//!    |         sin(w1)sin(w3)         -sin(w1)cos(w3)  cos(w1)cos(w2) |
//!    | +cos(w1)sin(w2)cos(w3)  +cos(w1)sin(w2)sin(w3)                 |
//!    +-                                                              -+
//! ```
//!
//!  We recover w2 by
//!
//!  
//!
//! ```text
//!    w2 = ASIN ( -M(1,3) ),
//! ```
//!
//!  so w2 is in the interval \[-pi/2, pi/2].
//!
//!  As long as w2 does not equal pi/2 or -pi/2, we can find w1 by the
//!    formula
//!
//!  
//!
//! ```text
//!    w1 = ATAN2 ( M(2,3), M(3,3) ),
//! ```
//!
//!  and w3 from the formula
//!
//!  
//!
//! ```text
//!    w3 = ATAN2 ( M(1,2), M(1,1) ).
//! ```
//!
//!  If w2 is -pi/2 or pi/2, we have a degenerate case. The sum of w1 and w3
//!    is determined, but w1 and w3 are not determined individually. We can set
//!    w3 to zero, which reduces our right hand side to
//!
//!  
//!
//! ```text
//!    +-                                       -+
//!    |         0            0        -sin(w2)  |
//!    |  sin(w1)sin(w2)    cos(w1)       0      |,
//!    |  cos(w1)sin(w2)   -sin(w1)       0      |
//!    +-                                       -+
//! ```
//!
//!  so we can recover w1 from the formula
//!
//!  
//!
//! ```text
//!    w1 = ATAN2 ( -M(3,2), M(2,2) ).
//! ```
//!
//!     
//! #  Appendix A: Document Revision History
//!
//!  
//!
//!
//!  
//! ###  March 9, 2017
//!
//!  Minor edits to eliminate typos.
//!
//!  
//!
//!
//!  
//! ###  May 27, 2010
//!
//!  Minor edit to eliminate typo.
//!
//!  
//!
//!
//!  
//! ###  November 17, 2005
//!
//!  Documentation of the routines [QXQ](crate::raw::qxq), [QDQ2AV](crate::raw::qdq2av) was added.
//!
//!  
//!
//!
//!  
//! ###  January 10, 2005
//!
//!  A few corrections were made to environment tags in the .ftm source for
//!    this document.
//!
//!  
//!
//!
//!  
//! ###  February 2, 2004
//!
//!  Performed a spell-check on text.
//!
//!  
//!
//!
//!  
//! ###  December 2, 2002
//!
//!  
//!
//! *  The relationship between Euler angles and the C-matrix is defined
//! differently by different projects. Modifications have been made to clarify
//! the definitions used by Cassini, Voyager and Galileo.
//!
//!     
//! ###  April 26, 1999
//!
//!  
//!
//! *  An equation involving quaternion multiplication was corrected: the
//! expression v*v = 0 was replaced by v*v = -1.
//!
//!  *  Quotation style was changed from British to American.
//!
//!  *  Some variable names were changed to remove underscores.
//!
//!  *  Some minor re-wording was done to simplify creation of the CSPICE version
//! of this document.
//!
//!      
